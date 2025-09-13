use crate::domain::repositories::todo_repository::TodoRepository;
use crate::domain::{models::todo::Todo, repositories::conn::Conn};
use crate::infrastructure::repositories::data_models::prelude::Todos as TodoTable;
use crate::infrastructure::repositories::data_models::todos;
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
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
pub struct TodoRepositoryImpl {}

impl TodoRepositoryImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for TodoRepositoryImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn find_all<C>(
        &self,
        conn: &C,
    ) -> Result<Vec<Todo>, crate::domain::repositories::errors::RepositoryError>
    where
        C: Conn,
    {
        let todos = TodoTable::find().all(conn).await?;
        Ok(todos.into_iter().map(Todo::from).collect())
    }

    async fn find_by_id<C>(
        &self,
        conn: &C,
        id: Uuid,
    ) -> Result<Todo, crate::domain::repositories::errors::RepositoryError>
    where
        C: Conn,
    {
        let todo = TodoTable::find_by_id(id).one(conn).await?;
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

    async fn create<C>(
        &self,
        conn: &C,
        todo: Todo,
    ) -> Result<Todo, crate::domain::repositories::errors::RepositoryError>
    where
        C: Conn,
    {
        let todo: todos::ActiveModel = todo.into();
        let todo: todos::Model = todo.insert(conn).await?;
        Ok(todo.into())
    }

    async fn update<C>(
        &self,
        conn: &C,
        todo: Todo,
    ) -> Result<Todo, crate::domain::repositories::errors::RepositoryError>
    where
        C: Conn,
    {
        let todo: todos::ActiveModel = todo.into();
        let todo: todos::Model = todo.update(conn).await?;
        Ok(Todo::from(todo))
    }

    async fn delete<C>(
        &self,
        conn: &C,
        todo: Todo,
    ) -> Result<(), crate::domain::repositories::errors::RepositoryError>
    where
        C: Conn,
    {
        let todo: todos::ActiveModel = todo.into();
        todo.delete(conn).await?;
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

        let repo = TodoRepositoryImpl::new();

        // Test create
        let todo = Todo::new("Test Todo".into(), None);
        let created_todo = repo.create(&conn, todo).await.unwrap();
        assert_eq!(created_todo.title, "Test Todo");

        // Test find_all
        let todos = repo.find_all(&conn).await.unwrap();
        assert!(!todos.is_empty());

        // Test find_by_id
        let found_todo = repo.find_by_id(&conn, created_todo.id).await.unwrap();
        assert_eq!(found_todo.title, "Test Todo");

        // Test update
        let mut updated_todo = created_todo;
        updated_todo.title = "Updated Todo".into();
        let updated_result = repo.update(&conn, updated_todo).await.unwrap();
        assert_eq!(updated_result.title, "Updated Todo");

        // Test delete
        let target_id = updated_result.id;
        repo.delete(&conn, updated_result).await.unwrap();
        let deleted_todo = repo.find_by_id(&conn, target_id).await;
        assert!(deleted_todo.is_err());
    }
}
