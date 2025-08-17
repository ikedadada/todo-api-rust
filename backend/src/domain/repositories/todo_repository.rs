use crate::domain::models::todo::Todo;
use crate::domain::repositories::errors::RepositoryError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TodoRepository<C>: Send + Sync {
    async fn find_all(&self, conn: &C) -> Result<Vec<Todo>, RepositoryError>;
    async fn find_by_id(&self, conn: &C, id: Uuid) -> Result<Todo, RepositoryError>;
    async fn create(&self, conn: &C, todo: Todo) -> Result<Todo, RepositoryError>;
    async fn update(&self, conn: &C, todo: Todo) -> Result<Todo, RepositoryError>;
    async fn delete(&self, conn: &C, todo: Todo) -> Result<(), RepositoryError>;
}
