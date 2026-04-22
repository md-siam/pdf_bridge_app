use thiserror::Error;

#[derive(Debug, Error)]
pub enum PdfForgeError {
    #[error("Invalid output path")]
    InvalidOutputPath,

    #[error("Failed to create output directory: {0}")] CreateDirectory(String),

    #[error("Failed to generate PDF: {0}")] GeneratePdf(String),

    #[error("Failed to write file: {0}")] WriteFile(String),

    #[error("Failed to get file metadata: {0}")] Metadata(String),
}

impl From<PdfForgeError> for String {
    fn from(value: PdfForgeError) -> Self {
        value.to_string()
    }
}
