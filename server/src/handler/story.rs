use crate::response::CreateStoryResponse;
use axum::http::StatusCode;
use axum::Json;
pub async fn create_story() -> Result<Json<CreateStoryResponse>, StatusCode> {
    todo!()
}
