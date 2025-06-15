use crate::request::CreateEngineerRequest;
use crate::response::CreateEngineerResponse;
use axum::http::StatusCode;
use axum::Json;
pub async fn create_engineer(
    Json(create_engineer_request): Json<CreateEngineerRequest>,
) -> Result<Json<CreateEngineerResponse>, StatusCode> {
    todo!()
}
