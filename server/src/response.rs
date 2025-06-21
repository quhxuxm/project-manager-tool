use crate::error::Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub id: u64,
    pub username: String,
    pub create_date: DateTime<Utc>,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRuleResponse {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub create_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStoryResponse {}
