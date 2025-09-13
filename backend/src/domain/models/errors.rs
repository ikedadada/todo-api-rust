use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("DomainError: Conflict({0})")]
    Conflict(String),
    #[error("DomainError: Unexpected({0})")]
    Unexpected(String),
}
