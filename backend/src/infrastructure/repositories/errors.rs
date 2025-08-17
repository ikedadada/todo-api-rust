use crate::domain::repositories::errors::RepositoryError;
use sea_orm::error::DbErr;

impl From<DbErr> for RepositoryError {
    fn from(error: DbErr) -> Self {
        match error {
            DbErr::RecordNotFound(msg) => RepositoryError::NotFound(msg),
            _ => RepositoryError::Unexpected(error.to_string()),
        }
    }
}
