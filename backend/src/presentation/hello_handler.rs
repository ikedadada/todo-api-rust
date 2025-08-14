use axum::{Router, routing::get};

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::presentation::validator::ValidatedJson;

#[derive(Validate, Deserialize, Serialize)]
struct Hello {
    #[validate(length(min = 1, message = "Message cannot be empty"))]
    message: String,
}

pub fn create_hello_router() -> Router {
    Router::new().route("/", get(get_hello).post(post_hello))
}

async fn get_hello() -> String {
    "Hello from /hello!".to_string()
}

async fn post_hello(ValidatedJson(input): ValidatedJson<Hello>) -> String {
    format!("Hello from /hello! You sent: {}", input.message)
}
