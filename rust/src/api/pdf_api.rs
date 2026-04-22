use crate::domain::models::{ CreatePdfRequest, CreatePdfResponse };
use crate::pdf::generator::generate_simple_pdf;

pub async fn create_simple_pdf(request: CreatePdfRequest) -> Result<CreatePdfResponse, String> {
    generate_simple_pdf(&request).map_err(Into::into)
}
