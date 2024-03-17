// Libs
use crate::log_success;
use crate::rt_util::quit;
use axum::{http::StatusCode, routing::get_service, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

/// Creates a new server that serves the UFC Ripper GUI
///
/// # Panics
///
/// Will panic if the port is already in use or fails to serve the Vue "dist" directory
pub async fn init_server() {
    let port = 8383;
    let app = Router::new().nest_service(
        "/",
        get_service(ServeDir::new("./dista")).handle_error(|_| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to serve UFC Ripper GUI files",
            )
        }),
    );
    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port)))
        .await
        .unwrap_or_else(|_| {
            quit(Some(
                format!("Failed to start a listener on port \"{port}\"").as_str(),
            ))
        });

    log_success!("UFC Ripper GUI is live at http://localhost:{port}\n");

    axum::serve(listener, app)
        .await
        .unwrap_or_else(|_| quit(Some("Failed to initiate the backend server")));
}
