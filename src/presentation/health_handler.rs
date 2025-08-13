use axum::{Router, routing::get};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
enum HealthStatus {
    Healthy,
    Unhealthy,
}

#[derive(Deserialize, Serialize)]
struct Health {
    status: HealthStatus,
}
pub fn create_health_router() -> Router {
    Router::new().route("/", get(get_health))
}

async fn get_health() -> String {
    let res = Health {
        status: HealthStatus::Healthy,
    };
    serde_json::to_string(&res).unwrap()
}
