use crate::domain::models::todo::Todo;
use crate::domain::repositories::todo_repository::TodoRepository;
use crate::infrastructure::repositories::db::DbPool;
use async_trait::async_trait;
use sqlx::prelude::FromRow;
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(FromRow)]
pub struct TodoRecord {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<TodoRecord> for Todo {
    fn from(record: TodoRecord) -> Self {
        Todo {
            id: record.id,
            title: record.title,
            description: record.description,
            completed: record.completed,
        }
    }
}

#[derive(Clone)]
pub struct TodoRepositoryImpl {
    pool: DbPool,
}

impl TodoRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn find_all(
        &self,
    ) -> Result<Vec<Todo>, crate::domain::repositories::errors::RepositoryError> {
        let todos: Vec<TodoRecord> = sqlx::query_as("SELECT * FROM todos")
            .fetch_all(&self.pool)
            .await?;
        Ok(todos.into_iter().map(Todo::from).collect())
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Todo, crate::domain::repositories::errors::RepositoryError> {
        let todo: TodoRecord = sqlx::query_as("SELECT * FROM todos WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(Todo::from(todo))
    }

    async fn create(
        &self,
        todo: Todo,
    ) -> Result<Todo, crate::domain::repositories::errors::RepositoryError> {
        let created_todo: TodoRecord = sqlx::query_as(
            "INSERT INTO todos (id, title, description, completed, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        )
        .bind(todo.id)
        .bind(todo.title)
        .bind(todo.description)
        .bind(todo.completed)
        .bind(Utc::now())
        .bind(Utc::now())
        .fetch_one(&self.pool)
        .await?;
        Ok(Todo::from(created_todo))
    }

    async fn update(
        &self,
        todo: Todo,
    ) -> Result<Todo, crate::domain::repositories::errors::RepositoryError> {
        let updated_todo: TodoRecord = sqlx::query_as(
            "UPDATE todos SET title = $1, description = $2, completed = $3, updated_at = $4 WHERE id = $5 RETURNING *",
        )
        .bind(todo.title)
        .bind(todo.description)
        .bind(todo.completed)
        .bind(Utc::now())
        .bind(todo.id)
        .fetch_one(&self.pool)
        .await?;
        Ok(Todo::from(updated_todo))
    }

    async fn delete(
        &self,
        id: Uuid,
    ) -> Result<(), crate::domain::repositories::errors::RepositoryError> {
        sqlx::query("DELETE FROM todos WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[cfg(all(test, feature = "db-tests"))]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use testcontainers::{ImageExt, runners::AsyncRunner};
    use testcontainers_modules::postgres::Postgres;

    #[tokio::test]
    async fn test_todo_repository() {
        let image = Postgres::default().with_tag("latest");
        let pg = image.start().await.expect("start postgres");
        let port = pg.get_host_port_ipv4(5432).await.expect("get port");
        let url = &format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres",);

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await
            .expect("connect db");

        sqlx::migrate!().run(&pool).await.expect("run migrations");

        let repo = TodoRepositoryImpl::new(pool);

        // Test create
        let todo = Todo::new("Test Todo".into(), None);
        let created_todo = repo.create(todo).await.unwrap();
        assert_eq!(created_todo.title, "Test Todo");

        // Test find_all
        let todos = repo.find_all().await.unwrap();
        assert!(!todos.is_empty());

        // Test find_by_id
        let found_todo = repo.find_by_id(created_todo.id).await.unwrap();
        assert_eq!(found_todo.title, "Test Todo");

        // Test update
        let mut updated_todo = created_todo;
        updated_todo.title = "Updated Todo".into();
        let updated_result = repo.update(updated_todo).await.unwrap();
        assert_eq!(updated_result.title, "Updated Todo");

        // Test delete
        repo.delete(updated_result.id).await.unwrap();
        let deleted_todo = repo.find_by_id(updated_result.id).await;
        assert!(deleted_todo.is_err());
    }
}
