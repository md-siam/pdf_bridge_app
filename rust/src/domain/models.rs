use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePdfRequest {
    pub output_path: String,
    pub title: String,
    pub body: String,
    pub author: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePdfResponse {
    pub file_path: String,
    pub file_name: String,
    pub file_size_bytes: u64,
    pub page_count: u32,
}
