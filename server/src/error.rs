use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Db(#[from] sqlx::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error response: {}", self),
        )
            .into_response()
    }
}
