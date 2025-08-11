use axum::{Json, Router, routing::get, serve};

use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Deserialize, Serialize)]
struct Hello {
    message: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/hello",
            get(|| async { "Hello from /hello!" }).post(|Json(input): Json<Hello>| async move {
                format!("Hello from /hello! You sent: {}", input.message)
            }),
        );

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());

    serve(listener, app).await.unwrap();
}
