use axum::{Router, extract::Query, routing::get};
use serde::Deserialize;

#[derive(Deserialize)]
struct WaitParams {
    delay: Option<u64>,
}
pub fn create_wait_router() -> Router {
    Router::new().route("/", get(get_wait))
}

async fn get_wait(Query(params): Query<WaitParams>) -> String {
    let delay = params.delay.unwrap_or(10);
    tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
    format!("Waited {} seconds", delay)
}
