use std::time::Duration;

use axum::{BoxError, Router, error_handling::HandleErrorLayer, serve};

use todo_api_rust::errors::AppError;
use todo_api_rust::presentation;
use tokio::net::TcpListener;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    let app = app()
        .fallback(|uri: axum::http::Uri| async move {
            AppError::NotFound(format!("No route found for {}", uri))
        })
        .layer(
            ServiceBuilder::new()
                // `timeout` will produce an error if the handler takes
                // too long so we must handle those
                .layer(HandleErrorLayer::new(async |err: BoxError| -> AppError {
                    if err.is::<tower::timeout::error::Elapsed>() {
                        AppError::Timeout
                    } else {
                        AppError::Internal(format!("Unhandled internal error: {err}"))
                    }
                }))
                .timeout(Duration::from_secs(10)),
        );

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());

    serve(listener, app)
        // with_graceful_shutdown() does not have time limit for waiting.
        // When the server is shut down, it will complete all ongoing requests.
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

fn app() -> Router {
    Router::new()
        .nest(
            "/health",
            presentation::health_handler::create_health_router(),
        )
        .nest("/hello", presentation::hello_handler::create_hello_router())
        .nest("/wait", presentation::wait_handler::create_wait_router())
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
