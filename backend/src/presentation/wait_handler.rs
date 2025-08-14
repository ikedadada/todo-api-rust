use axum::{Router, routing::get};
use serde::Deserialize;
use validator::Validate;

use crate::presentation::validator::ValidatedQuery;

#[derive(Deserialize, Validate)]
struct WaitParams {
    #[validate(range(min = 1, max = 60, message = "Delay must be between 1 and 60 seconds"))]
    sec: Option<u64>,
}
pub fn create_wait_router() -> Router {
    Router::new().route("/", get(get_wait))
}

async fn get_wait(ValidatedQuery(params): ValidatedQuery<WaitParams>) -> String {
    let sec = params.sec.unwrap_or(10);
    tokio::time::sleep(tokio::time::Duration::from_secs(sec)).await;
    format!("Waited {} seconds", sec)
}
