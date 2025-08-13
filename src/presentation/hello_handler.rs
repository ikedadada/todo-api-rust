use axum::{Router, extract::Json, routing::get};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Hello {
    message: String,
}
pub fn create_hello_router() -> Router {
    Router::new().route("/", get(get_hello).post(post_hello))
}

async fn get_hello() -> String {
    "Hello from /hello!".to_string()
}

async fn post_hello(Json(input): Json<Hello>) -> String {
    format!("Hello from /hello! You sent: {}", input.message)
}
