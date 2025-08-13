use axum::{http::StatusCode, response::IntoResponse};
#[derive(Debug)]
pub enum AppError {
    Timeout,
    NotFound(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Timeout => {
                (StatusCode::REQUEST_TIMEOUT, "Request took too long").into_response()
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg).into_response(),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
        }
    }
}
