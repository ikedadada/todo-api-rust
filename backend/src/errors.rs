use axum::{http::StatusCode, response::IntoResponse};

use crate::application_service::usecase::errors::UsecaseError;
#[derive(Debug)]
pub enum AppError {
    Timeout,
    NotFound(String),
    Conflict(String),
    Internal(String),
}

impl From<UsecaseError> for AppError {
    fn from(err: UsecaseError) -> Self {
        match err {
            UsecaseError::NotFound => AppError::NotFound("Resource not found".to_string()),
            UsecaseError::Conflict(msg) => AppError::Internal(format!("Conflict: {}", msg)),
            UsecaseError::Unexpected(msg) => {
                AppError::Internal(format!("Unexpected error: {}", msg))
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Timeout => {
                (StatusCode::REQUEST_TIMEOUT, "Request took too long").into_response()
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg).into_response(),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg).into_response(),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
        }
    }
}
