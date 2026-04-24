//! Error types and HTTP response mapping.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use crate::models::ErrorResponse;

#[derive(Debug)]
pub enum RegistryError {
    NotFound(String),
    Unauthorized(String),
    RateLimited,
    BadRequest(String),
    Conflict(String),
    InternalError(String),
}

impl IntoResponse for RegistryError {
    fn into_response(self) -> Response {
        let (status, error, detail) = match self {
            RegistryError::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", Some(msg)),
            RegistryError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "unauthorized", Some(msg)),
            RegistryError::RateLimited => (StatusCode::TOO_MANY_REQUESTS, "rate_limited", None),
            RegistryError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", Some(msg)),
            RegistryError::Conflict(msg) => (StatusCode::CONFLICT, "conflict", Some(msg)),
            RegistryError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", Some(msg)),
        };

        let response = ErrorResponse {
            error: error.to_string(),
            detail,
        };

        (status, Json(response)).into_response()
    }
}

impl From<anyhow::Error> for RegistryError {
    fn from(err: anyhow::Error) -> Self {
        RegistryError::InternalError(err.to_string())
    }
}

impl From<rusqlite::Error> for RegistryError {
    fn from(err: rusqlite::Error) -> Self {
        RegistryError::InternalError(err.to_string())
    }
}
