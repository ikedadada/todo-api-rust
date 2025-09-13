use uuid::Uuid;

use crate::domain::models::errors::DomainError;

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
}

impl Todo {
    pub fn new(title: String, description: Option<String>) -> Self {
        Self {
            id: Uuid::now_v7(),
            title,
            description,
            completed: false,
        }
    }

    pub fn update(&mut self, title: String, description: Option<String>) {
        self.title = title;
        self.description = description;
    }

    pub fn mark_completed(&mut self) -> Result<(), DomainError> {
        if self.completed {
            return Err(DomainError::Conflict("Todo is already completed".into()));
        }
        self.completed = true;
        Ok(())
    }

    pub fn unmark_completed(&mut self) -> Result<(), DomainError> {
        if !self.completed {
            return Err(DomainError::Conflict("Todo is not completed".into()));
        }
        self.completed = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_todo_creation() {
        let todo = Todo::new("Test Todo".into(), None);
        assert!(todo.id != Uuid::nil());
        assert_eq!(todo.title, "Test Todo");
        assert_eq!(todo.description, None);
        assert!(!todo.completed);
    }

    #[test]
    fn test_todo_update() {
        let mut todo = Todo::new("Test Todo".into(), None);
        todo.update("Updated Todo".into(), Some("Updated Description".into()));
        assert_eq!(todo.title, "Updated Todo");
        assert_eq!(todo.description, Some("Updated Description".into()));
    }

    #[test]
    fn test_mark_completed() {
        let mut todo = Todo {
            id: Uuid::now_v7(),
            title: "Test Todo".into(),
            description: None,
            completed: false,
        };
        assert!(!todo.completed);
        todo.mark_completed().unwrap();
        assert!(todo.completed);
    }

    #[test]
    fn test_mark_completed_already_completed() {
        let mut todo = Todo {
            id: Uuid::now_v7(),
            title: "Test Todo".into(),
            description: None,
            completed: true,
        };
        let result = todo.mark_completed();
        assert!(result.is_err());
    }

    #[test]
    fn test_unmark_completed() {
        let mut todo = Todo {
            id: Uuid::now_v7(),
            title: "Test Todo".into(),
            description: None,
            completed: true,
        };
        todo.unmark_completed().unwrap();
        assert!(!todo.completed);
    }

    #[test]
    fn test_unmark_completed_already_uncompleted() {
        let mut todo = Todo {
            id: Uuid::now_v7(),
            title: "Test Todo".into(),
            description: None,
            completed: false,
        };
        let result = todo.unmark_completed();
        assert!(result.is_err());
    }
}
