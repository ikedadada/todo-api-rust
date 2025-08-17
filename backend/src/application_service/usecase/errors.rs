use thiserror::Error;

use crate::domain::{models::errors::DomainError, repositories::errors::RepositoryError};

#[derive(Error, Debug)]
pub enum UsecaseError {
    #[error("UsecaseError: NotFound({0})")]
    NotFound(String),
    #[error("UsecaseError: Conflict({0})")]
    Conflict(String),
    #[error("UsecaseError: Unexpected({0})")]
    Unexpected(String),
}

impl From<DomainError> for UsecaseError {
    fn from(err: DomainError) -> Self {
        UsecaseError::Unexpected(err.to_string())
    }
}

impl From<RepositoryError> for UsecaseError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::NotFound(msg) => UsecaseError::NotFound(msg),
            RepositoryError::Conflict(msg) => UsecaseError::Conflict(msg),
            RepositoryError::Unexpected(msg) => UsecaseError::Unexpected(msg),
        }
    }
}
