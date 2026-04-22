use std::fs::{ self, File };
use std::io::BufWriter;
use std::path::Path;

use crate::error::PdfForgeError;

pub fn ensure_parent_dir_exists(output_path: &str) -> Result<(), PdfForgeError> {
    let path = Path::new(output_path);

    let parent = path.parent().ok_or(PdfForgeError::InvalidOutputPath)?;
    fs::create_dir_all(parent).map_err(|e| PdfForgeError::CreateDirectory(e.to_string()))?;

    Ok(())
}

pub fn create_output_writer(output_path: &str) -> Result<BufWriter<File>, PdfForgeError> {
    let file = File::create(output_path).map_err(|e| PdfForgeError::WriteFile(e.to_string()))?;
    Ok(BufWriter::new(file))
}

pub fn file_size(output_path: &str) -> Result<u64, PdfForgeError> {
    let metadata = std::fs
        ::metadata(output_path)
        .map_err(|e| PdfForgeError::Metadata(e.to_string()))?;
    Ok(metadata.len())
}
