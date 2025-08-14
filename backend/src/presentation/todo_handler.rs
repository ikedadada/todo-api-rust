use std::sync::Arc;

use axum::{
    Router,
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    application_service::usecase::todo_usecase::TodoUsecase, domain::models::todo::Todo,
    errors::AppError,
};

#[derive(Clone)]
pub struct AppState<T: TodoUsecase> {
    todo_usecase: Arc<T>,
}

pub fn create_todo_router<T: TodoUsecase + Send + Sync + 'static + Clone>(
    todo_usecase: T,
) -> Router {
    let app_state = AppState {
        todo_usecase: Arc::new(todo_usecase),
    };

    Router::new()
        .route("/", get(get_all_todos::<T>).post(post_todo::<T>))
        .route(
            "/{id}",
            get(get_todo_by_id::<T>)
                .put(update_todo::<T>)
                .delete(delete_todo::<T>),
        )
        .route("/{id}/complete", put(mark_todo_completed::<T>))
        .route("/{id}/uncomplete", put(unmark_todo_completed::<T>))
        .with_state(app_state)
}

#[derive(Serialize)]
struct TodoResponse {
    id: Uuid,
    title: String,
    description: Option<String>,
    completed: bool,
}

impl From<Todo> for TodoResponse {
    fn from(todo: Todo) -> Self {
        Self {
            id: todo.id,
            title: todo.title,
            description: todo.description,
            completed: todo.completed,
        }
    }
}

#[derive(Deserialize)]
struct CreateTodoRequest {
    title: String,
    description: String,
}

#[derive(Deserialize)]
struct UpdateTodoRequest {
    title: String,
    description: String,
}

async fn get_all_todos<T: TodoUsecase>(
    State(app_state): State<AppState<T>>,
) -> Result<impl IntoResponse, AppError> {
    let todos = app_state.todo_usecase.get_all_todos().await?;
    Ok((
        StatusCode::OK,
        Json(
            todos
                .into_iter()
                .map(TodoResponse::from)
                .collect::<Vec<_>>(),
        ),
    ))
}

async fn get_todo_by_id<T: TodoUsecase>(
    State(app_state): State<AppState<T>>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<Json<TodoResponse>, AppError> {
    let todo = app_state.todo_usecase.get_todo_by_id(id).await?;
    Ok(Json(TodoResponse::from(todo)))
}

async fn post_todo<T: TodoUsecase>(
    State(app_state): State<AppState<T>>,
    Json(input): Json<CreateTodoRequest>,
) -> Result<impl IntoResponse, AppError> {
    let todo = app_state
        .todo_usecase
        .create_todo(input.title, Some(input.description))
        .await?;
    Ok((StatusCode::CREATED, Json(TodoResponse::from(todo))))
}

async fn update_todo<T: TodoUsecase>(
    State(app_state): State<AppState<T>>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
    Json(input): Json<UpdateTodoRequest>,
) -> Result<impl IntoResponse, AppError> {
    let todo = app_state
        .todo_usecase
        .update_todo(id, input.title, Some(input.description))
        .await?;
    Ok((StatusCode::OK, Json(TodoResponse::from(todo))))
}

async fn delete_todo<T: TodoUsecase>(
    State(app_state): State<AppState<T>>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    app_state.todo_usecase.delete_todo(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn mark_todo_completed<T: TodoUsecase>(
    State(app_state): State<AppState<T>>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let todo = app_state.todo_usecase.mark_todo_completed(id).await?;
    Ok((StatusCode::OK, Json(TodoResponse::from(todo))))
}

async fn unmark_todo_completed<T: TodoUsecase>(
    State(app_state): State<AppState<T>>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let todo = app_state.todo_usecase.unmark_todo_completed(id).await?;
    Ok((StatusCode::OK, Json(TodoResponse::from(todo))))
}
