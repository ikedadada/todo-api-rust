use std::sync::Arc;

use axum::{
    Router,
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    application_service::usecase::todo_usecase::TodoUsecase,
    domain::models::todo::Todo,
    presentation::{errors::AppError, validator::ValidatedJson},
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

#[derive(Deserialize, Validate)]
struct CreateTodoRequest {
    #[validate(length(min = 2, max = 100))]
    title: String,
    #[validate(length(max = 255))]
    description: Option<String>,
}

#[derive(Deserialize, Validate)]
struct UpdateTodoRequest {
    #[validate(length(min = 2, max = 100))]
    title: String,
    #[validate(length(max = 255))]
    description: Option<String>,
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
    WithRejection(Path(id), _): WithRejection<Path<Uuid>, AppError>,
) -> Result<Json<TodoResponse>, AppError> {
    let todo = app_state.todo_usecase.get_todo_by_id(id).await?;
    Ok(Json(TodoResponse::from(todo)))
}

async fn post_todo<T: TodoUsecase>(
    State(app_state): State<AppState<T>>,
    ValidatedJson(input): ValidatedJson<CreateTodoRequest>,
) -> Result<impl IntoResponse, AppError> {
    let todo = app_state
        .todo_usecase
        .create_todo(input.title, input.description)
        .await?;
    Ok((StatusCode::CREATED, Json(TodoResponse::from(todo))))
}

async fn update_todo<T: TodoUsecase>(
    State(app_state): State<AppState<T>>,
    WithRejection(Path(id), _): WithRejection<Path<Uuid>, AppError>,
    ValidatedJson(input): ValidatedJson<UpdateTodoRequest>,
) -> Result<impl IntoResponse, AppError> {
    let todo = app_state
        .todo_usecase
        .update_todo(id, input.title, input.description)
        .await?;
    Ok((StatusCode::OK, Json(TodoResponse::from(todo))))
}

async fn delete_todo<T: TodoUsecase>(
    State(app_state): State<AppState<T>>,
    WithRejection(Path(id), _): WithRejection<Path<Uuid>, AppError>,
) -> Result<impl IntoResponse, AppError> {
    app_state.todo_usecase.delete_todo(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn mark_todo_completed<T: TodoUsecase>(
    State(app_state): State<AppState<T>>,
    WithRejection(Path(id), _): WithRejection<Path<Uuid>, AppError>,
) -> Result<impl IntoResponse, AppError> {
    let todo = app_state.todo_usecase.mark_todo_completed(id).await?;
    Ok((StatusCode::OK, Json(TodoResponse::from(todo))))
}

async fn unmark_todo_completed<T: TodoUsecase>(
    State(app_state): State<AppState<T>>,
    WithRejection(Path(id), _): WithRejection<Path<Uuid>, AppError>,
) -> Result<impl IntoResponse, AppError> {
    let todo = app_state.todo_usecase.unmark_todo_completed(id).await?;
    Ok((StatusCode::OK, Json(TodoResponse::from(todo))))
}
