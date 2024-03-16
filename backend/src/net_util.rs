// Libs
use crate::log_success;
use axum::{http::StatusCode, routing::get_service, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

/// Creates a new server that serves the UFC Ripper GUI
pub async fn init_server() {
    let port = 8383;
    let app = Router::new().nest_service(
        "/",
        get_service(ServeDir::new("./dist")).handle_error(|_| async move {
            (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
        }),
    );
    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port)))
        .await
        .unwrap_or_else(|_| panic!("Failed to start a listener on the port \"{port}\"\n"));

    log_success!("UFC Ripper GUI is live at http://localhost:{port}\n");

    axum::serve(listener, app)
        .await
        .expect("Failed to initiate the backend server\n");
}
