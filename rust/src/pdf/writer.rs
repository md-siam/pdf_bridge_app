use std::fs;
use std::path::Path;

use crate::error::PdfForgeError;

pub fn ensure_parent_dir_exists(output_path: &str) -> Result<(), PdfForgeError> {
    let path = Path::new(output_path);
    let parent = path.parent().ok_or(PdfForgeError::InvalidOutputPath)?;

    fs::create_dir_all(parent).map_err(|e| PdfForgeError::CreateDirectory(e.to_string()))?;

    Ok(())
}

pub fn file_size(output_path: &str) -> Result<u64, PdfForgeError> {
    let metadata = fs::metadata(output_path).map_err(|e| PdfForgeError::Metadata(e.to_string()))?;
    Ok(metadata.len())
}
