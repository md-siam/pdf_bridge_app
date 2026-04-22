use std::path::Path;

use printpdf::{
    BuiltinFont,
    Mm,
    Op,
    PdfDocument,
    PdfFontHandle,
    PdfPage,
    PdfSaveOptions,
    Point,
    Pt,
    TextItem,
};

use crate::domain::models::{ CreatePdfRequest, CreatePdfResponse };
use crate::error::PdfForgeError;
use crate::pdf::layout::{
    BODY_FONT_SIZE,
    BODY_START_Y_MM,
    LINE_HEIGHT_MM,
    MARGIN_LEFT_MM,
    MARGIN_TOP_MM,
    PAGE_HEIGHT_MM,
    PAGE_WIDTH_MM,
    TITLE_FONT_SIZE,
};
use crate::pdf::writer::{ ensure_parent_dir_exists, file_size };

pub fn generate_simple_pdf(request: &CreatePdfRequest) -> Result<CreatePdfResponse, PdfForgeError> {
    validate_request(request)?;

    ensure_parent_dir_exists(&request.output_path)?;

    let title = request.title.trim();
    let body = request.body.trim();
    let author = request.author.as_deref().unwrap_or("Unknown").trim();

    let mut doc = PdfDocument::new(title);
    let mut ops: Vec<Op> = Vec::new();

    ops.push(Op::StartTextSection);

    // Title
    ops.push(Op::SetFont {
        font: PdfFontHandle::Builtin(BuiltinFont::HelveticaBold),
        size: Pt(TITLE_FONT_SIZE as f32),
    });
    ops.push(Op::SetTextCursor {
        pos: Point {
            x: Mm(MARGIN_LEFT_MM as f32).into(),
            y: Mm(MARGIN_TOP_MM as f32).into(),
        },
    });
    ops.push(Op::ShowText {
        items: vec![TextItem::Text(title.to_string())],
    });

    // Author
    ops.push(Op::SetFont {
        font: PdfFontHandle::Builtin(BuiltinFont::Helvetica),
        size: Pt(BODY_FONT_SIZE as f32),
    });
    ops.push(Op::SetTextCursor {
        pos: Point {
            x: Mm(MARGIN_LEFT_MM as f32).into(),
            y: Mm((BODY_START_Y_MM + 12.0) as f32).into(),
        },
    });
    ops.push(Op::ShowText {
        items: vec![TextItem::Text(format!("Author: {author}"))],
    });

    // Body
    ops.push(Op::SetLineHeight {
        lh: Pt(mm_to_pt(LINE_HEIGHT_MM)),
    });

    let mut current_y_mm = BODY_START_Y_MM;

    for line in split_text_into_lines(body) {
        if current_y_mm <= 20.0 {
            break;
        }

        ops.push(Op::SetTextCursor {
            pos: Point {
                x: Mm(MARGIN_LEFT_MM as f32).into(),
                y: Mm(current_y_mm as f32).into(),
            },
        });

        ops.push(Op::ShowText {
            items: vec![TextItem::Text(line)],
        });

        current_y_mm -= LINE_HEIGHT_MM;
    }

    ops.push(Op::EndTextSection);

    let page = PdfPage::new(Mm(PAGE_WIDTH_MM as f32), Mm(PAGE_HEIGHT_MM as f32), ops);

    doc.with_pages(vec![page]);

    let mut warnings = Vec::new();
    let pdf_bytes = doc.save(&PdfSaveOptions::default(), &mut warnings);

    std::fs
        ::write(&request.output_path, pdf_bytes)
        .map_err(|e| PdfForgeError::WriteFile(format!("Failed to write PDF file: {e}")))?;

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

fn split_text_into_lines(text: &str) -> Vec<String> {
    const MAX_CHARS_PER_LINE: usize = 80;

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

            if candidate.chars().count() <= MAX_CHARS_PER_LINE {
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

fn mm_to_pt(mm: f64) -> f32 {
    ((mm * 72.0) / 25.4) as f32
}
