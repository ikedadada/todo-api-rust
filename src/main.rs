use std::time::Duration;

use axum::{
    BoxError, Json, Router, error_handling::HandleErrorLayer, http::StatusCode, routing::get, serve,
};

use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower::ServiceBuilder;

#[derive(Deserialize, Serialize)]
struct Hello {
    message: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .fallback(fallback)
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/hello",
            get(|| async { "Hello from /hello!" }).post(|Json(input): Json<Hello>| async move {
                format!("Hello from /hello! You sent: {}", input.message)
            }),
        )
        .route(
            "/wait",
            get(|| async {
                tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
                "Hello from /wait!"
            }),
        )
        .layer(
            ServiceBuilder::new()
                // `timeout` will produce an error if the handler takes
                // too long so we must handle those
                .layer(HandleErrorLayer::new(
                    async |err: BoxError| -> (StatusCode, String) {
                        if err.is::<tower::timeout::error::Elapsed>() {
                            (
                                StatusCode::REQUEST_TIMEOUT,
                                "Request took too long".to_string(),
                            )
                        } else {
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!("Unhandled internal error: {err}"),
                            )
                        }
                    },
                ))
                .timeout(Duration::from_secs(10)),
        );

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());

    serve(listener, app)
        // with_graceful_shutdown() does not have time limit for waiting.
        // When the server is shut down, it will complete all ongoing requests.
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route found for {}", uri))
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install terminate handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            println!("Received Ctrl+C, shutting down...");
        },
        _ = terminate => {
            println!("Received SIGTERM, shutting down...");
        },
    }
}
