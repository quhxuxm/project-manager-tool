mod role;
mod user;
use crate::error::Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
pub use role::*;
use serde::{Deserialize, Serialize};
pub use user::*;
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub reason: String,
    pub code: u32,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                reason: format!("Error response: {}", self),
                code: self.into(),
            }),
        )
            .into_response()
    }
}
