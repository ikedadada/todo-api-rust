use thiserror::Error;

use crate::{
    application_service::service::transaction_service::TransactionError,
    domain::{models::errors::DomainError, repositories::errors::RepositoryError},
};

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
        match err {
            DomainError::Conflict(msg) => UsecaseError::Conflict(msg),
            DomainError::Unexpected(msg) => UsecaseError::Unexpected(msg),
        }
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

impl From<TransactionError> for UsecaseError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::Conflict(msg) => UsecaseError::Conflict(msg),
            TransactionError::Unexpected(msg) => UsecaseError::Unexpected(msg),
            TransactionError::NotFound(msg) => UsecaseError::NotFound(msg),
        }
    }
}
