use crate::domain::models::todo::Todo;
use crate::domain::repositories::todo_repository::TodoRepository;
use crate::infrastructure::repositories::data_models::prelude::Todos as TodoTable;
use crate::infrastructure::repositories::data_models::todos;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbConn, EntityTrait};
use sqlx::types::chrono::Utc;
use uuid::Uuid;

impl From<todos::Model> for Todo {
    fn from(model: todos::Model) -> Self {
        Todo {
            id: model.id,
            title: model.title,
            description: model.description,
            completed: model.completed,
        }
    }
}

impl From<Todo> for todos::ActiveModel {
    fn from(todo: Todo) -> Self {
        todos::ActiveModel {
            id: Set(todo.id),
            title: Set(todo.title),
            description: Set(todo.description),
            completed: Set(todo.completed),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
        }
    }
}

#[derive(Clone)]
pub struct TodoRepositoryImpl {
    conn: DbConn,
}

impl TodoRepositoryImpl {
    pub fn new(conn: DbConn) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn find_all(
        &self,
    ) -> Result<Vec<Todo>, crate::domain::repositories::errors::RepositoryError> {
        let todos = TodoTable::find().all(&self.conn).await?;
        Ok(todos.into_iter().map(Todo::from).collect())
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Todo, crate::domain::repositories::errors::RepositoryError> {
        let todo = TodoTable::find_by_id(id).one(&self.conn).await?;
        match todo {
            Some(todo) => Ok(Todo::from(todo)),
            None => Err(
                crate::domain::repositories::errors::RepositoryError::NotFound(format!(
                    "Todo with id {} not found",
                    id
                )),
            ),
        }
    }

    async fn create(
        &self,
        todo: Todo,
    ) -> Result<Todo, crate::domain::repositories::errors::RepositoryError> {
        let todo: todos::ActiveModel = todo.into();
        let todo: todos::Model = todo.insert(&self.conn).await?;
        Ok(todo.into())
    }

    async fn update(
        &self,
        todo: Todo,
    ) -> Result<Todo, crate::domain::repositories::errors::RepositoryError> {
        let todo: todos::ActiveModel = todo.into();
        let todo: todos::Model = todo.update(&self.conn).await?;
        Ok(Todo::from(todo))
    }

    async fn delete(
        &self,
        todo: Todo,
    ) -> Result<(), crate::domain::repositories::errors::RepositoryError> {
        let todo: todos::ActiveModel = todo.into();
        todo.delete(&self.conn).await?;
        Ok(())
    }
}

#[cfg(all(test, feature = "db-tests"))]
mod tests {
    use super::*;
    use migration::MigratorTrait;
    use sea_orm::{ConnectOptions, Database};
    use testcontainers::{ImageExt, runners::AsyncRunner};
    use testcontainers_modules::postgres::Postgres;

    #[tokio::test]
    async fn test_todo_repository() {
        let image = Postgres::default().with_tag("latest");
        let pg = image.start().await.expect("start postgres");
        let port = pg.get_host_port_ipv4(5432).await.expect("get port");
        let url = &format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres",);

        let opt = ConnectOptions::new(url.to_string());
        let conn = Database::connect(opt).await.expect("connect db");

        migration::Migrator::up(&conn, None)
            .await
            .expect("create migrator");

        let repo = TodoRepositoryImpl::new(conn);

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
        let target_id = updated_result.id;
        repo.delete(updated_result).await.unwrap();
        let deleted_todo = repo.find_by_id(target_id).await;
        assert!(deleted_todo.is_err());
    }
}
