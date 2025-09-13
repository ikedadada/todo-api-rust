use crate::domain::models::todo::Todo;
use crate::domain::repositories::conn::Conn;
use crate::domain::repositories::errors::RepositoryError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn find_all<C>(&self, conn: &C) -> Result<Vec<Todo>, RepositoryError>
    where
        C: Conn;
    async fn find_by_id<C>(&self, conn: &C, id: Uuid) -> Result<Todo, RepositoryError>
    where
        C: Conn;
    async fn create<C>(&self, conn: &C, todo: Todo) -> Result<Todo, RepositoryError>
    where
        C: Conn;
    async fn update<C>(&self, conn: &C, todo: Todo) -> Result<Todo, RepositoryError>
    where
        C: Conn;
    async fn delete<C>(&self, conn: &C, todo: Todo) -> Result<(), RepositoryError>
    where
        C: Conn;
}
