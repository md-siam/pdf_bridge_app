use std::path::Path;

use image::ImageReader;
use lopdf::content::{ Content, Operation };
use lopdf::{ dictionary, Document, Object, Stream };

use crate::domain::models::{ CreatePdfRequest, CreatePdfResponse };
use crate::error::PdfForgeError;
use crate::pdf::layout::{
    BODY_FONT_SIZE,
    BOTTOM_MARGIN_PT,
    LINE_HEIGHT_PT,
    PAGE_HEIGHT_PT,
    PAGE_WIDTH_PT,
    TITLE_FONT_SIZE,
};
use crate::pdf::writer::{ ensure_parent_dir_exists, file_size };

pub fn generate_simple_pdf(request: &CreatePdfRequest) -> Result<CreatePdfResponse, PdfForgeError> {
    validate_request(request)?;
    ensure_parent_dir_exists(&request.output_path)?;

    let title = request.title.trim();
    let body = request.body.trim();
    let author = request.author.as_deref().unwrap_or("Unknown").trim();

    let mut doc = Document::with_version("1.5");

    let pages_id = doc.new_object_id();

    let font_id = doc.add_object(
        dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    }
    );

    let font_bold_id = doc.add_object(
        dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica-Bold",
    }
    );

    let image_info = if let Some(bytes) = &request.image_bytes {
        Some(add_image_xobject(&mut doc, bytes)?)
    } else {
        None
    };

    let resources_id = if let Some((image_id, _, _)) = image_info {
        doc.add_object(
            dictionary! {
            "Font" => dictionary! {
                "F1" => font_id,
                "F2" => font_bold_id,
            },
            "XObject" => dictionary! {
                "Im1" => image_id,
            }
        }
        )
    } else {
        doc.add_object(
            dictionary! {
            "Font" => dictionary! {
                "F1" => font_id,
                "F2" => font_bold_id,
            }
        }
        )
    };

    let content = build_page_content(title, body, author, image_info)?;
    let content_id = doc.add_object(
        Stream::new(
            dictionary! {},
            content
                .encode()
                .map_err(|e| {
                    PdfForgeError::GeneratePdf(format!("Failed to encode content stream: {e}"))
                })?
        )
    );

    let page_id = doc.add_object(
        dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "MediaBox" => vec![
            Object::Integer(0),
            Object::Integer(0),
            Object::Real(PAGE_WIDTH_PT as f32),
            Object::Real(PAGE_HEIGHT_PT as f32),
        ],
        "Contents" => content_id,
        "Resources" => resources_id,
    }
    );

    let pages =
        dictionary! {
        "Type" => "Pages",
        "Kids" => vec![Object::Reference(page_id)],
        "Count" => 1,
        "MediaBox" => vec![
            Object::Integer(0),
            Object::Integer(0),
            Object::Real(PAGE_WIDTH_PT as f32),
            Object::Real(PAGE_HEIGHT_PT as f32),
        ],
    };

    doc.objects.insert(pages_id, Object::Dictionary(pages));

    let catalog_id = doc.add_object(
        dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    }
    );

    doc.trailer.set("Root", catalog_id);
    doc.compress();

    doc
        .save(&request.output_path)
        .map_err(|e| PdfForgeError::WriteFile(format!("Failed to save PDF: {e}")))?;

    let size = file_size(&request.output_path)?;

    let file_name = Path::new(&request.output_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("generated.pdf")
        .to_string();

    Ok(CreatePdfResponse {
        file_path: request.output_path.clone(),
        file_name,
        file_size_bytes: size,
        page_count: 1,
    })
}

fn add_image_xobject(
    doc: &mut Document,
    image_bytes: &[u8]
) -> Result<(lopdf::ObjectId, u32, u32), PdfForgeError> {
    let image = ImageReader::new(std::io::Cursor::new(image_bytes))
        .with_guessed_format()
        .map_err(|e| PdfForgeError::GeneratePdf(format!("Failed to read image format: {e}")))?
        .decode()
        .map_err(|e| PdfForgeError::GeneratePdf(format!("Failed to decode image: {e}")))?;

    let rgb_image = image.to_rgb8();
    let width = rgb_image.width();
    let height = rgb_image.height();
    let raw_rgb = rgb_image.into_raw();

    let image_stream = Stream::new(
        dictionary! {
            "Type" => "XObject",
            "Subtype" => "Image",
            "Width" => width as i64,
            "Height" => height as i64,
            "ColorSpace" => "DeviceRGB",
            "BitsPerComponent" => 8,
        },
        raw_rgb
    );

    let image_id = doc.add_object(image_stream);

    Ok((image_id, width, height))
}

fn build_page_content(
    title: &str,
    body: &str,
    author: &str,
    image_info: Option<(lopdf::ObjectId, u32, u32)>
) -> Result<Content, PdfForgeError> {
    let mut operations = Vec::<Operation>::new();

    // Layout constants
    const LEFT_MARGIN_PT: f64 = 50.0;
    const TOP_MARGIN_PT: f64 = 60.0;
    const IMAGE_TEXT_GAP_PT: f64 = 25.0;

    const MAX_IMAGE_WIDTH_PT: f64 = 130.0;
    const MAX_IMAGE_HEIGHT_PT: f64 = 170.0;

    // Approximate visible top of Helvetica-Bold text above baseline.
    // You can tweak 0.72 slightly (0.70 - 0.75) if needed.
    const TITLE_ASCENDER_RATIO: f64 = 0.72;

    const AUTHOR_OFFSET_PT: f64 = 45.0;
    const BODY_OFFSET_PT: f64 = 34.0;

    let title_baseline_y = PAGE_HEIGHT_PT - TOP_MARGIN_PT;
    let title_visual_top_y = title_baseline_y + TITLE_FONT_SIZE * TITLE_ASCENDER_RATIO;

    let author_baseline_y = title_baseline_y - AUTHOR_OFFSET_PT;
    let body_start_y = author_baseline_y - BODY_OFFSET_PT;

    let image_x = LEFT_MARGIN_PT;
    let mut text_x = LEFT_MARGIN_PT;

    let has_image = image_info.is_some();

    if let Some((_image_id, image_width, image_height)) = image_info {
        let width_ratio = MAX_IMAGE_WIDTH_PT / (image_width as f64);
        let height_ratio = MAX_IMAGE_HEIGHT_PT / (image_height as f64);
        let scale = width_ratio.min(height_ratio);

        let display_width = (image_width as f64) * scale;
        let display_height = (image_height as f64) * scale;

        // Align image top with the visible top of the title
        let image_y = title_visual_top_y - display_height;

        operations.push(Operation::new("q", vec![]));
        operations.push(
            Operation::new(
                "cm",
                vec![
                    Object::Real(display_width as f32),
                    Object::Real(0.0),
                    Object::Real(0.0),
                    Object::Real(display_height as f32),
                    Object::Real(image_x as f32),
                    Object::Real(image_y as f32)
                ]
            )
        );
        operations.push(Operation::new("Do", vec![Object::Name(b"Im1".to_vec())]));
        operations.push(Operation::new("Q", vec![]));

        text_x = image_x + display_width + IMAGE_TEXT_GAP_PT;
    }

    // Title
    operations.push(Operation::new("BT", vec![]));
    operations.push(
        Operation::new(
            "Tf",
            vec![Object::Name(b"F2".to_vec()), Object::Real(TITLE_FONT_SIZE as f32)]
        )
    );
    operations.push(
        Operation::new(
            "Td",
            vec![Object::Real(text_x as f32), Object::Real(title_baseline_y as f32)]
        )
    );
    operations.push(Operation::new("Tj", vec![Object::string_literal(title)]));
    operations.push(Operation::new("ET", vec![]));

    // Author
    operations.push(Operation::new("BT", vec![]));
    operations.push(
        Operation::new(
            "Tf",
            vec![Object::Name(b"F1".to_vec()), Object::Real(BODY_FONT_SIZE as f32)]
        )
    );
    operations.push(
        Operation::new(
            "Td",
            vec![Object::Real(text_x as f32), Object::Real(author_baseline_y as f32)]
        )
    );
    operations.push(
        Operation::new("Tj", vec![Object::string_literal(format!("Author: {author}"))])
    );
    operations.push(Operation::new("ET", vec![]));

    // Body
    let available_chars = if has_image { 55 } else { 80 };
    let lines = split_text_into_lines(body, available_chars);
    let mut current_y = body_start_y;

    for line in lines {
        if current_y < BOTTOM_MARGIN_PT {
            break;
        }

        operations.push(Operation::new("BT", vec![]));
        operations.push(
            Operation::new(
                "Tf",
                vec![Object::Name(b"F1".to_vec()), Object::Real(BODY_FONT_SIZE as f32)]
            )
        );
        operations.push(
            Operation::new("Td", vec![Object::Real(text_x as f32), Object::Real(current_y as f32)])
        );
        operations.push(Operation::new("Tj", vec![Object::string_literal(line)]));
        operations.push(Operation::new("ET", vec![]));

        current_y -= LINE_HEIGHT_PT;
    }

    Ok(Content { operations })
}

fn validate_request(request: &CreatePdfRequest) -> Result<(), PdfForgeError> {
    if request.output_path.trim().is_empty() {
        return Err(PdfForgeError::InvalidOutputPath);
    }

    if request.title.trim().is_empty() {
        return Err(PdfForgeError::GeneratePdf("Title cannot be empty".to_string()));
    }

    if request.body.trim().is_empty() {
        return Err(PdfForgeError::GeneratePdf("Body cannot be empty".to_string()));
    }

    Ok(())
}

fn split_text_into_lines(text: &str, max_chars_per_line: usize) -> Vec<String> {
    let mut lines = Vec::new();

    for paragraph in text.lines() {
        let trimmed = paragraph.trim();

        if trimmed.is_empty() {
            lines.push(String::new());
            continue;
        }

        let mut current_line = String::new();

        for word in trimmed.split_whitespace() {
            let candidate = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{current_line} {word}")
            };

            if candidate.chars().count() <= max_chars_per_line {
                current_line = candidate;
            } else {
                if !current_line.is_empty() {
                    lines.push(current_line);
                }
                current_line = word.to_string();
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }
    }

    lines
}
