use crate::response::CreateProjectResponse;
use axum::http::StatusCode;
use axum::Json;
pub async fn create_project() -> Result<Json<CreateProjectResponse>, StatusCode> {
    todo!()
}
