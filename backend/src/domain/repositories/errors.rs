use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("RepositoryError: NotFound")]
    NotFound,
    #[error("RepositoryError: Conflict({0})")]
    Conflict(String),
    #[error("RepositoryError: Unexpected({0})")]
    Unexpected(String),
}
