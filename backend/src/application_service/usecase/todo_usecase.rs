use std::sync::Arc;

use crate::{
    application_service::usecase::errors::UsecaseError,
    domain::{models::todo::Todo, repositories::todo_repository::TodoRepository},
};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TodoUsecase {
    async fn get_all_todos(&self) -> Result<Vec<Todo>, UsecaseError>;
    async fn get_todo_by_id(&self, id: Uuid) -> Result<Todo, UsecaseError>;
    async fn create_todo(
        &self,
        title: String,
        description: Option<String>,
    ) -> Result<Todo, UsecaseError>;
    async fn update_todo(
        &self,
        id: Uuid,
        title: String,
        description: Option<String>,
    ) -> Result<Todo, UsecaseError>;
    async fn delete_todo(&self, id: Uuid) -> Result<(), UsecaseError>;
    async fn mark_todo_completed(&self, id: Uuid) -> Result<Todo, UsecaseError>;
    async fn unmark_todo_completed(&self, id: Uuid) -> Result<Todo, UsecaseError>;
}

pub struct TodoUsecaseImpl<T: TodoRepository> {
    repository: Arc<T>,
}

impl<T: TodoRepository> TodoUsecaseImpl<T> {
    pub fn new(repository: T) -> Self {
        Self {
            repository: Arc::new(repository),
        }
    }
}

#[async_trait]
impl<T: TodoRepository + Send + Sync> TodoUsecase for TodoUsecaseImpl<T> {
    async fn get_all_todos(&self) -> Result<Vec<Todo>, UsecaseError> {
        let todos = self.repository.find_all().await?;
        Ok(todos)
    }

    async fn get_todo_by_id(&self, id: Uuid) -> Result<Todo, UsecaseError> {
        let todo = self.repository.find_by_id(id).await?;
        Ok(todo)
    }

    async fn create_todo(
        &self,
        title: String,
        description: Option<String>,
    ) -> Result<Todo, UsecaseError> {
        let todo = Todo::new(title, description);
        let new_todo = self.repository.create(todo).await?;
        Ok(new_todo)
    }

    async fn update_todo(
        &self,
        id: Uuid,
        title: String,
        description: Option<String>,
    ) -> Result<Todo, UsecaseError> {
        let mut todo = self.repository.find_by_id(id).await?;
        todo.update(title, description);
        let new_todo = self.repository.update(todo).await?;
        Ok(new_todo)
    }

    async fn delete_todo(&self, id: Uuid) -> Result<(), UsecaseError> {
        self.repository.delete(id).await?;
        Ok(())
    }

    async fn mark_todo_completed(&self, id: Uuid) -> Result<Todo, UsecaseError> {
        let mut todo = self.repository.find_by_id(id).await?;
        todo.mark_completed()?;
        let new_todo = self.repository.update(todo).await?;
        Ok(new_todo)
    }

    async fn unmark_todo_completed(&self, id: Uuid) -> Result<Todo, UsecaseError> {
        let mut todo = self.repository.find_by_id(id).await?;
        todo.unmark_completed()?;
        let new_todo = self.repository.update(todo).await?;
        Ok(new_todo)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::repositories::errors::RepositoryError;

    use super::*;
    use std::sync::{Arc, Mutex};

    #[derive(Clone)]
    struct MockTodoRepository {
        todos: Arc<Mutex<Vec<Todo>>>,
    }

    impl MockTodoRepository {
        pub fn new() -> Self {
            Self {
                todos: Arc::new(Mutex::new(vec![
                    Todo {
                        id: Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8").unwrap(),
                        title: "Test Todo 1".into(),
                        description: None,
                        completed: false,
                    },
                    Todo {
                        id: Uuid::parse_str("b1b2b3b4c1c2d1d2e1e2e3e4e5e6e7e8").unwrap(),
                        title: "Test Todo 2".into(),
                        description: Some("Description for Test Todo 2".into()),
                        completed: true,
                    },
                ])),
            }
        }
    }

    #[async_trait]
    impl TodoRepository for MockTodoRepository {
        async fn find_all(&self) -> Result<Vec<Todo>, RepositoryError> {
            let todos = self.todos.lock().unwrap();
            Ok(todos.clone())
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Todo, RepositoryError> {
            let todos = self.todos.lock().unwrap();
            todos
                .iter()
                .find(|todo| todo.id == id)
                .cloned()
                .ok_or(RepositoryError::NotFound)
        }

        async fn create(&self, todo: Todo) -> Result<Todo, RepositoryError> {
            self.todos.lock().unwrap().push(todo.clone());
            Ok(todo)
        }

        async fn update(&self, todo: Todo) -> Result<Todo, RepositoryError> {
            let mut todos = self.todos.lock().unwrap();
            todos
                .iter_mut()
                .find(|t| t.id == todo.id)
                .map(|t| {
                    *t = todo.clone();
                    todo
                })
                .ok_or(RepositoryError::NotFound)
        }

        async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
            let mut todos = self.todos.lock().unwrap();
            if let Some(index) = todos.iter().position(|t| t.id == id) {
                todos.remove(index);
                Ok(())
            } else {
                Err(RepositoryError::NotFound)
            }
        }
    }

    #[tokio::test]
    async fn test_todo_usecase_impl_get_all_todos() {
        let repository = MockTodoRepository::new();
        let usecase = TodoUsecaseImpl::new(repository);

        let result = usecase.get_all_todos().await;

        assert!(result.is_ok());
        let todos = result.unwrap();
        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0].title, "Test Todo 1");
        assert_eq!(todos[1].title, "Test Todo 2");
    }

    #[tokio::test]
    async fn test_todo_usecase_impl_get_todo_by_id() {
        let repository = MockTodoRepository::new();
        let usecase = TodoUsecaseImpl::new(repository);

        let result = usecase
            .get_todo_by_id(Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8").unwrap())
            .await;

        assert!(result.is_ok());
        let todo = result.unwrap();
        assert_eq!(todo.title, "Test Todo 1");
        assert_eq!(todo.description, None);
        assert!(!todo.completed);
    }

    #[tokio::test]
    async fn test_todo_usecase_impl_create_todo() {
        let repository = MockTodoRepository::new();
        let usecase = TodoUsecaseImpl::new(repository.clone());

        let result = usecase
            .create_todo("New Todo".into(), Some("Description".into()))
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().title, "New Todo");
        let len = { repository.todos.lock().unwrap().len() };
        assert_eq!(len, 3);
    }

    #[tokio::test]
    async fn test_todo_usecase_impl_update_todo() {
        let repository = MockTodoRepository::new();
        let usecase = TodoUsecaseImpl::new(repository.clone());

        let result = usecase
            .update_todo(
                Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8").unwrap(),
                "Updated Todo".into(),
                Some("Updated Description".into()),
            )
            .await;

        assert!(result.is_ok());
        let todo = result.unwrap();
        assert_eq!(todo.title, "Updated Todo");
        assert_eq!(todo.description, Some("Updated Description".into()));
        assert!(!todo.completed);

        let todos = {
            let todos = repository.todos.lock().unwrap();
            todos.clone()
        };
        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0].title, "Updated Todo");
        assert_eq!(todos[0].description, Some("Updated Description".into()));
        assert!(!todos[0].completed);
    }

    #[tokio::test]
    async fn test_todo_usecase_impl_delete_todo() {
        let repository = MockTodoRepository::new();
        let usecase = TodoUsecaseImpl::new(repository.clone());

        let result = usecase
            .delete_todo(Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8").unwrap())
            .await;

        assert!(result.is_ok());
        let len = { repository.todos.lock().unwrap().len() };
        assert_eq!(len, 1);
    }

    #[tokio::test]
    async fn test_todo_usecase_impl_mark_todo_completed() {
        let repository = MockTodoRepository::new();
        let usecase = TodoUsecaseImpl::new(repository.clone());

        let result = usecase
            .mark_todo_completed(Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8").unwrap())
            .await;

        assert!(result.is_ok());
        let completed = { repository.todos.lock().unwrap()[0].completed };
        assert!(completed);
    }

    #[tokio::test]
    async fn test_todo_usecase_impl_mark_todo_incomplete() {
        let repository = MockTodoRepository::new();
        let usecase = TodoUsecaseImpl::new(repository.clone());

        let result = usecase
            .unmark_todo_completed(Uuid::parse_str("b1b2b3b4c1c2d1d2e1e2e3e4e5e6e7e8").unwrap())
            .await;

        assert!(result.is_ok());
        let completed = { repository.todos.lock().unwrap()[1].completed };
        assert!(!completed);
    }
}
