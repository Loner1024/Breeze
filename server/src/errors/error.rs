use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown error")]
    Unknown
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::Unknown => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "unknown error"})),
            )
        }
            .into_response()
    }
}


pub type Result<T> = std::result::Result<T, Error>;