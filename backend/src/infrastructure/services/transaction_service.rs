use std::pin::Pin;

use async_trait::async_trait;

use crate::{
    application_service::service::transaction_service::{TransactionError, TransactionService},
    domain::repositories::conn::Conn,
};

pub struct TransactionServiceImpl;

impl TransactionServiceImpl {
    pub fn new() -> Self {
        TransactionServiceImpl {}
    }
}

impl Default for TransactionServiceImpl {
    fn default() -> Self {
        TransactionServiceImpl::new()
    }
}

impl From<sea_orm::TransactionError<TransactionError>> for TransactionError {
    fn from(err: sea_orm::TransactionError<TransactionError>) -> Self {
        match err {
            sea_orm::TransactionError::Transaction(err) => err,
            sea_orm::TransactionError::Connection(err) => {
                TransactionError::Unexpected(err.to_string())
            }
        }
    }
}

#[async_trait]
impl TransactionService for TransactionServiceImpl {
    type Tx<'a>
        = sea_orm::DatabaseTransaction
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
        T: Send,
    {
        let result = conn.transaction(f).await?;
        Ok(result)
    }
}
