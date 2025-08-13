use crate::domain::repositories::errors::RepositoryError;
use sqlx::Error;

impl From<Error> for RepositoryError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            Error::RowNotFound => RepositoryError::NotFound,
            _ => RepositoryError::Unexpected(error.to_string()),
        }
    }
}
