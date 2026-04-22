use std::path::Path;

use lopdf::content::{ Content, Operation };
use lopdf::{ dictionary, Document, Object, Stream };

use crate::domain::models::{ CreatePdfRequest, CreatePdfResponse };
use crate::error::PdfForgeError;
use crate::pdf::layout::{
    AUTHOR_Y_PT,
    BODY_FONT_SIZE,
    BODY_START_Y_PT,
    BOTTOM_MARGIN_PT,
    LEFT_MARGIN_PT,
    LINE_HEIGHT_PT,
    PAGE_HEIGHT_PT,
    PAGE_WIDTH_PT,
    TITLE_FONT_SIZE,
    TOP_Y_PT,
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

    let resources_id = doc.add_object(
        dictionary! {
        "Font" => dictionary! {
            "F1" => font_id,
            "F2" => font_bold_id,
        }
    }
    );

    let content = build_page_content(title, body, author)?;
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

fn build_page_content(title: &str, body: &str, author: &str) -> Result<Content, PdfForgeError> {
    let mut operations = Vec::<Operation>::new();

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
            vec![Object::Real(LEFT_MARGIN_PT as f32), Object::Real(TOP_Y_PT as f32)]
        )
    );
    operations.push(Operation::new("Tj", vec![Object::string_literal(escape_pdf_text(title))]));
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
            vec![Object::Real(LEFT_MARGIN_PT as f32), Object::Real(AUTHOR_Y_PT as f32)]
        )
    );
    operations.push(
        Operation::new(
            "Tj",
            vec![Object::string_literal(escape_pdf_text(&format!("Author: {author}")))]
        )
    );
    operations.push(Operation::new("ET", vec![]));

    // Body lines
    let lines = split_text_into_lines(body, 80);
    let mut current_y = BODY_START_Y_PT;

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
            Operation::new(
                "Td",
                vec![Object::Real(LEFT_MARGIN_PT as f32), Object::Real(current_y as f32)]
            )
        );
        operations.push(Operation::new("Tj", vec![Object::string_literal(escape_pdf_text(&line))]));
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

fn escape_pdf_text(input: &str) -> String {
    input.replace('\\', "\\\\").replace('(', "\\(").replace(')', "\\)")
}
