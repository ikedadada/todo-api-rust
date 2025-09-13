use std::sync::Arc;

use crate::{
    application_service::{
        service::transaction_service::{TransactionError, TransactionService},
        usecase::errors::UsecaseError,
    },
    domain::{
        models::todo::Todo,
        repositories::{conn::Conn, todo_repository::TodoRepository},
    },
};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TodoUsecase: Send + Sync + 'static {
    async fn get_all_todos<C>(&self, conn: &C) -> Result<Vec<Todo>, UsecaseError>
    where
        C: Conn;
    async fn get_todo_by_id<C>(&self, conn: &C, id: Uuid) -> Result<Todo, UsecaseError>
    where
        C: Conn;
    async fn create_todo<C>(
        &self,
        conn: &C,
        title: String,
        description: Option<String>,
    ) -> Result<Todo, UsecaseError>
    where
        C: Conn;
    async fn update_todo<C>(
        &self,
        conn: &C,
        id: Uuid,
        title: String,
        description: Option<String>,
    ) -> Result<Todo, UsecaseError>
    where
        C: Conn;
    async fn delete_todo<C>(&self, conn: &C, id: Uuid) -> Result<(), UsecaseError>
    where
        C: Conn;
    async fn mark_todo_completed<C>(&self, conn: &C, id: Uuid) -> Result<Todo, UsecaseError>
    where
        C: Conn;
    async fn unmark_todo_completed<C>(&self, conn: &C, id: Uuid) -> Result<Todo, UsecaseError>
    where
        C: Conn;
}

#[derive(Clone)]
pub struct TodoUsecaseImpl<R, T> {
    repository: Arc<R>,
    transaction_service: Arc<T>,
}

impl<R, T> TodoUsecaseImpl<R, T> {
    pub fn new(repository: R, transaction_service: T) -> Self {
        Self {
            repository: Arc::new(repository),
            transaction_service: Arc::new(transaction_service),
        }
    }
}

#[async_trait]
impl<R, T> TodoUsecase for TodoUsecaseImpl<R, T>
where
    R: TodoRepository + Send + Sync + 'static,
    T: TransactionService + Send + Sync + 'static,
{
    async fn get_all_todos<C>(&self, conn: &C) -> Result<Vec<Todo>, UsecaseError>
    where
        C: Conn,
    {
        let todos = self.repository.find_all(conn).await?;
        Ok(todos)
    }

    async fn get_todo_by_id<C>(&self, conn: &C, id: Uuid) -> Result<Todo, UsecaseError>
    where
        C: Conn,
    {
        let todo = self.repository.find_by_id(conn, id).await?;
        Ok(todo)
    }

    async fn create_todo<C>(
        &self,
        conn: &C,
        title: String,
        description: Option<String>,
    ) -> Result<Todo, UsecaseError>
    where
        C: Conn,
    {
        let todo = Todo::new(title, description);
        let todo = self.repository.create(conn, todo).await?;
        Ok(todo)
    }

    async fn update_todo<C>(
        &self,
        conn: &C,
        id: Uuid,
        title: String,
        description: Option<String>,
    ) -> Result<Todo, UsecaseError>
    where
        C: Conn,
    {
        let repository = self.repository.clone();
        let todo = self
            .transaction_service
            .run(conn, move |tx| {
                Box::pin(async move {
                    let mut todo = repository.find_by_id(tx, id).await?;
                    todo.update(title, description);
                    let updated_todo = repository.update(tx, todo).await?;
                    Ok::<Todo, TransactionError>(updated_todo)
                })
            })
            .await?;
        Ok(todo)
    }

    async fn delete_todo<C>(&self, conn: &C, id: Uuid) -> Result<(), UsecaseError>
    where
        C: Conn,
    {
        let repository = self.repository.clone();
        self.transaction_service
            .run(conn, move |tx| {
                Box::pin(async move {
                    let todo = repository.find_by_id(tx, id).await?;
                    repository.delete(tx, todo).await?;
                    Ok::<(), TransactionError>(())
                })
            })
            .await?;
        Ok(())
    }

    async fn mark_todo_completed<C>(&self, conn: &C, id: Uuid) -> Result<Todo, UsecaseError>
    where
        C: Conn,
    {
        let repository = self.repository.clone();
        let todo = self
            .transaction_service
            .run(conn, move |tx| {
                Box::pin(async move {
                    let mut todo = repository.find_by_id(tx, id).await?;
                    todo.mark_completed()?;
                    let new_todo = repository.update(tx, todo).await?;
                    Ok::<Todo, TransactionError>(new_todo)
                })
            })
            .await?;
        Ok(todo)
    }

    async fn unmark_todo_completed<C>(&self, conn: &C, id: Uuid) -> Result<Todo, UsecaseError>
    where
        C: Conn,
    {
        let repository = self.repository.clone();
        let todo = self
            .transaction_service
            .run(conn, move |tx| {
                Box::pin(async move {
                    let mut todo = repository.find_by_id(tx, id).await?;
                    todo.unmark_completed()?;
                    let new_todo = repository.update(tx, todo).await?;
                    Ok::<Todo, TransactionError>(new_todo)
                })
            })
            .await?;
        Ok(todo)
    }
}

#[cfg(test)]
mod tests {

    use crate::domain::repositories::conn::tests::MockConn;
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
        async fn find_all<C>(&self, _conn: &C) -> Result<Vec<Todo>, RepositoryError> {
            let todos = self.todos.lock().unwrap();
            Ok(todos.clone())
        }

        async fn find_by_id<C>(&self, _conn: &C, id: Uuid) -> Result<Todo, RepositoryError> {
            let todos = self.todos.lock().unwrap();
            todos
                .iter()
                .find(|todo| todo.id == id)
                .cloned()
                .ok_or(RepositoryError::NotFound(format!(
                    "Todo with id {} not found",
                    id
                )))
        }

        async fn create<C>(&self, _conn: &C, todo: Todo) -> Result<Todo, RepositoryError> {
            self.todos.lock().unwrap().push(todo.clone());
            Ok(todo)
        }

        async fn update<C>(&self, _conn: &C, todo: Todo) -> Result<Todo, RepositoryError> {
            let target_id = todo.id;
            let mut todos = self.todos.lock().unwrap();
            todos
                .iter_mut()
                .find(|t| t.id == todo.id)
                .map(|t| {
                    *t = todo.clone();
                    todo
                })
                .ok_or(RepositoryError::NotFound(format!(
                    "Todo with id {} not found",
                    target_id
                )))
        }

        async fn delete<C>(&self, _conn: &C, todo: Todo) -> Result<(), RepositoryError> {
            let mut todos = self.todos.lock().unwrap();
            if let Some(index) = todos.iter().position(|t| t.id == todo.id) {
                todos.remove(index);
                Ok(())
            } else {
                Err(RepositoryError::NotFound(format!(
                    "Todo with id {} not found",
                    todo.id
                )))
            }
        }
    }

    struct MockTransactionService;

    impl MockTransactionService {
        pub fn new() -> Self {
            Self
        }
    }

    #[async_trait]
    impl TransactionService for MockTransactionService {
        type Tx<'a>
            = MockConn
        where
            Self: 'a;
        async fn run<C, F, T>(&self, _: &C, f: F) -> Result<T, TransactionError>
        where
            C: Conn,
            F: for<'tx> FnOnce(
                    &'tx Self::Tx<'tx>,
                ) -> std::pin::Pin<
                    Box<dyn std::future::Future<Output = Result<T, TransactionError>> + Send + 'tx>,
                > + Send,
            T: Send,
        {
            f(&MockConn).await
        }
    }

    #[tokio::test]
    async fn test_todo_usecase_impl_get_all_todos() {
        let repository = MockTodoRepository::new();
        let transaction_service = MockTransactionService::new();
        let usecase = TodoUsecaseImpl::new(repository, transaction_service);

        let result = usecase.get_all_todos(&MockConn).await;

        assert!(result.is_ok());
        let todos = result.unwrap();
        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0].title, "Test Todo 1");
        assert_eq!(todos[1].title, "Test Todo 2");
    }

    #[tokio::test]
    async fn test_todo_usecase_impl_get_todo_by_id() {
        let repository = MockTodoRepository::new();
        let transaction_service = MockTransactionService::new();
        let usecase = TodoUsecaseImpl::new(repository, transaction_service);

        let result = usecase
            .get_todo_by_id(
                &MockConn,
                Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8").unwrap(),
            )
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
        let transaction_service = MockTransactionService::new();
        let usecase = TodoUsecaseImpl::new(repository.clone(), transaction_service);

        let result = usecase
            .create_todo(&MockConn, "New Todo".into(), Some("Description".into()))
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().title, "New Todo");
        let len = { repository.todos.lock().unwrap().len() };
        assert_eq!(len, 3);
    }

    #[tokio::test]
    async fn test_todo_usecase_impl_update_todo() {
        let repository = MockTodoRepository::new();
        let transaction_service = MockTransactionService::new();
        let usecase = TodoUsecaseImpl::new(repository.clone(), transaction_service);

        let result = usecase
            .update_todo(
                &MockConn,
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
        let transaction_service = MockTransactionService::new();
        let usecase = TodoUsecaseImpl::new(repository.clone(), transaction_service);

        let result = usecase
            .delete_todo(
                &MockConn,
                Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8").unwrap(),
            )
            .await;

        assert!(result.is_ok());
        let len = { repository.todos.lock().unwrap().len() };
        assert_eq!(len, 1);
    }

    #[tokio::test]
    async fn test_todo_usecase_impl_mark_todo_completed() {
        let repository = MockTodoRepository::new();
        let transaction_service = MockTransactionService::new();
        let usecase = TodoUsecaseImpl::new(repository.clone(), transaction_service);

        let result = usecase
            .mark_todo_completed(
                &MockConn,
                Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8").unwrap(),
            )
            .await;

        assert!(result.is_ok());
        let completed = { repository.todos.lock().unwrap()[0].completed };
        assert!(completed);
    }

    #[tokio::test]
    async fn test_todo_usecase_impl_mark_todo_incomplete() {
        let repository = MockTodoRepository::new();
        let transaction_service = MockTransactionService::new();
        let usecase = TodoUsecaseImpl::new(repository.clone(), transaction_service);

        let result = usecase
            .unmark_todo_completed(
                &MockConn,
                Uuid::parse_str("b1b2b3b4c1c2d1d2e1e2e3e4e5e6e7e8").unwrap(),
            )
            .await;

        assert!(result.is_ok());
        let completed = { repository.todos.lock().unwrap()[1].completed };
        assert!(!completed);
    }
}
