use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

use crate::application_service::usecase::errors::UsecaseError;

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub code: &'static str,
    pub message: String,
}

#[derive(Debug)]
pub enum AppError {
    Timeout,
    BadRequest(ErrorBody),
    NotFound(ErrorBody),
    Conflict(ErrorBody),
    Internal(ErrorBody),
}

impl From<UsecaseError> for AppError {
    fn from(err: UsecaseError) -> Self {
        match err {
            UsecaseError::NotFound(msg) => AppError::NotFound(ErrorBody {
                code: "404",
                message: format!("Not Found: {}", msg),
            }),
            UsecaseError::Conflict(msg) => AppError::Conflict(ErrorBody {
                code: "409",
                message: format!("Conflict: {}", msg),
            }),
            UsecaseError::Unexpected(msg) => AppError::Internal(ErrorBody {
                code: "500",
                message: format!("Unexpected error: {}", msg),
            }),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Timeout => (
                StatusCode::REQUEST_TIMEOUT,
                Json(ErrorBody {
                    code: "408",
                    message: "Request took too long".to_string(),
                }),
            )
                .into_response(),
            AppError::BadRequest(body) => (StatusCode::BAD_REQUEST, Json(body)).into_response(),
            AppError::NotFound(body) => (StatusCode::NOT_FOUND, Json(body)).into_response(),
            AppError::Conflict(body) => (StatusCode::CONFLICT, Json(body)).into_response(),
            AppError::Internal(body) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
            }
        }
    }
}
