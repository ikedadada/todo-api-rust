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

pub struct AppState<C> {
    todo_usecase: Arc<dyn TodoUsecase<C> + Send + Sync + 'static>,
    db: Arc<C>,
}

impl<C> Clone for AppState<C> {
    fn clone(&self) -> Self {
        Self {
            todo_usecase: Arc::clone(&self.todo_usecase),
            db: Arc::clone(&self.db),
        }
    }
}

pub fn create_todo_router<C>(
    todo_usecase: Arc<dyn TodoUsecase<C> + Send + Sync + 'static>,
    db: Arc<C>,
) -> Router
where
    C: Send + Sync + 'static,
{
    let app_state = AppState { todo_usecase, db };

    Router::new()
        .route("/", get(get_all_todos::<C>).post(post_todo::<C>))
        .route(
            "/{id}",
            get(get_todo_by_id::<C>)
                .put(update_todo::<C>)
                .delete(delete_todo::<C>),
        )
        .route("/{id}/complete", put(mark_todo_completed::<C>))
        .route("/{id}/uncomplete", put(unmark_todo_completed::<C>))
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

async fn get_all_todos<C>(
    State(app_state): State<AppState<C>>,
) -> Result<impl IntoResponse, AppError>
where
    C: Send + Sync + 'static,
{
    let conn = app_state.db.as_ref();
    let todos = app_state.todo_usecase.get_all_todos(conn).await?;
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

async fn get_todo_by_id<C>(
    State(app_state): State<AppState<C>>,
    WithRejection(Path(id), _): WithRejection<Path<Uuid>, AppError>,
) -> Result<Json<TodoResponse>, AppError>
where
    C: Send + Sync + 'static,
{
    let conn = app_state.db.as_ref();
    let todo = app_state.todo_usecase.get_todo_by_id(conn, id).await?;
    Ok(Json(TodoResponse::from(todo)))
}

async fn post_todo<C>(
    State(app_state): State<AppState<C>>,
    ValidatedJson(input): ValidatedJson<CreateTodoRequest>,
) -> Result<impl IntoResponse, AppError>
where
    C: Send + Sync + 'static,
{
    let conn = app_state.db.as_ref();
    let todo = app_state
        .todo_usecase
        .create_todo(conn, input.title, input.description)
        .await?;
    Ok((StatusCode::CREATED, Json(TodoResponse::from(todo))))
}

async fn update_todo<C>(
    State(app_state): State<AppState<C>>,
    WithRejection(Path(id), _): WithRejection<Path<Uuid>, AppError>,
    ValidatedJson(input): ValidatedJson<UpdateTodoRequest>,
) -> Result<impl IntoResponse, AppError>
where
    C: Send + Sync + 'static,
{
    let conn = app_state.db.as_ref();
    let todo = app_state
        .todo_usecase
        .update_todo(conn, id, input.title, input.description)
        .await?;
    Ok((StatusCode::OK, Json(TodoResponse::from(todo))))
}

async fn delete_todo<C>(
    State(app_state): State<AppState<C>>,
    WithRejection(Path(id), _): WithRejection<Path<Uuid>, AppError>,
) -> Result<impl IntoResponse, AppError>
where
    C: Send + Sync + 'static,
{
    let conn = app_state.db.as_ref();
    app_state.todo_usecase.delete_todo(conn, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn mark_todo_completed<C>(
    State(app_state): State<AppState<C>>,
    WithRejection(Path(id), _): WithRejection<Path<Uuid>, AppError>,
) -> Result<impl IntoResponse, AppError>
where
    C: Send + Sync + 'static,
{
    let conn = app_state.db.as_ref();
    let todo = app_state.todo_usecase.mark_todo_completed(conn, id).await?;
    Ok((StatusCode::OK, Json(TodoResponse::from(todo))))
}

async fn unmark_todo_completed<C>(
    State(app_state): State<AppState<C>>,
    WithRejection(Path(id), _): WithRejection<Path<Uuid>, AppError>,
) -> Result<impl IntoResponse, AppError>
where
    C: Send + Sync + 'static,
{
    let conn = app_state.db.as_ref();
    let todo = app_state
        .todo_usecase
        .unmark_todo_completed(conn, id)
        .await?;
    Ok((StatusCode::OK, Json(TodoResponse::from(todo))))
}
