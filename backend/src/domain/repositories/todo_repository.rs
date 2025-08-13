use crate::domain::models::todo::Todo;
use crate::domain::repositories::errors::RepositoryError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TodoRepository {
    async fn find_all(&self) -> Result<Vec<Todo>, RepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Todo, RepositoryError>;
    async fn create(&self, todo: Todo) -> Result<Todo, RepositoryError>;
    async fn update(&self, todo: Todo) -> Result<Todo, RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
}
