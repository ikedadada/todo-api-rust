use async_trait::async_trait;
use std::pin::Pin;
use thiserror::Error;

use crate::domain::{
    models::errors::DomainError,
    repositories::{conn::Conn, errors::RepositoryError},
};

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("TransactionError: Unexpected({0})")]
    Unexpected(String),
    #[error("TransactionError: Conflict({0})")]
    Conflict(String),
    #[error("TransactionError: NotFound({0})")]
    NotFound(String),
}

impl From<RepositoryError> for TransactionError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::NotFound(msg) => TransactionError::NotFound(msg),
            RepositoryError::Conflict(msg) => TransactionError::Conflict(msg),
            RepositoryError::Unexpected(msg) => TransactionError::Unexpected(msg),
        }
    }
}

impl From<DomainError> for TransactionError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::Conflict(msg) => TransactionError::Conflict(msg),
            DomainError::Unexpected(msg) => TransactionError::Unexpected(msg),
        }
    }
}

#[async_trait]
pub trait TransactionService {
    type Tx<'a>: Conn + 'a
    where
        Self: 'a;
    async fn run<C, F, T>(&self, conn: &C, f: F) -> Result<T, TransactionError>
    where
        C: Conn,
        F: for<'tx> FnOnce(
                &'tx Self::Tx<'tx>,
            ) -> Pin<
                Box<dyn Future<Output = Result<T, TransactionError>> + Send + 'tx>,
            > + Send,
        T: Send;
}
