use sea_orm::{ConnectionTrait, TransactionTrait};

pub trait Conn: ConnectionTrait + TransactionTrait + Send + Sync {}
impl<T> Conn for T where T: ConnectionTrait + TransactionTrait + Send + Sync {}

#[cfg(test)]
pub mod tests {
    use async_trait::async_trait;

    pub struct MockConn;
    #[async_trait]
    impl sea_orm::ConnectionTrait for MockConn {
        fn get_database_backend(&self) -> sea_orm::DbBackend {
            unimplemented!()
        }
        async fn execute(
            &self,
            _stmt: sea_orm::Statement,
        ) -> Result<sea_orm::ExecResult, sea_orm::DbErr> {
            unimplemented!()
        }
        async fn execute_unprepared(
            &self,
            _sql: &str,
        ) -> Result<sea_orm::ExecResult, sea_orm::DbErr> {
            unimplemented!()
        }
        async fn query_one(
            &self,
            _stmt: sea_orm::Statement,
        ) -> Result<Option<sea_orm::QueryResult>, sea_orm::DbErr> {
            unimplemented!()
        }
        async fn query_all(
            &self,
            _stmt: sea_orm::Statement,
        ) -> Result<Vec<sea_orm::QueryResult>, sea_orm::DbErr> {
            unimplemented!()
        }
        fn support_returning(&self) -> bool {
            unimplemented!()
        }
        fn is_mock_connection(&self) -> bool {
            true
        }
    }

    #[async_trait]
    impl sea_orm::TransactionTrait for MockConn {
        async fn begin(&self) -> Result<sea_orm::DatabaseTransaction, sea_orm::DbErr> {
            unimplemented!()
        }
        async fn begin_with_config(
            &self,
            _: Option<sea_orm::IsolationLevel>,
            _: Option<sea_orm::AccessMode>,
        ) -> Result<sea_orm::DatabaseTransaction, sea_orm::DbErr> {
            unimplemented!()
        }
        async fn transaction<F, T, E>(&self, _: F) -> Result<T, sea_orm::TransactionError<E>>
        where
            F: for<'c> FnOnce(
                    &'c sea_orm::DatabaseTransaction,
                )
                    -> core::pin::Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'c>>
                + Send,
            T: Send,
            E: std::fmt::Display + std::fmt::Debug + Send,
        {
            unimplemented!()
        }
        async fn transaction_with_config<F, T, E>(
            &self,
            _: F,
            _: Option<sea_orm::IsolationLevel>,
            _: Option<sea_orm::AccessMode>,
        ) -> Result<T, sea_orm::TransactionError<E>>
        where
            F: for<'c> FnOnce(
                    &'c sea_orm::DatabaseTransaction,
                )
                    -> core::pin::Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'c>>
                + Send,
            T: Send,
            E: std::fmt::Display + std::fmt::Debug + Send,
        {
            unimplemented!()
        }
    }
}
