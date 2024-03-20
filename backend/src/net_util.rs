// Libs
use crate::config_util::UFCRConfig;
use crate::log_success;
use crate::rt_util::QuitUnwrap;
use crate::ws_uti::create_ws_layer;
use axum::{http::StatusCode, routing::get_service, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

/// Creates a new server that serves the UFC Ripper GUI and the `WebSocket` server.
///
/// # Panics
///
/// Will panic if the port is already in use or fails to serve the Vue "dist" directory.
pub async fn init_server(config: &UFCRConfig) {
    let port = config.port;

    // Axum router
    let app = Router::new()
        .nest_service(
            "/",
            get_service(ServeDir::new("./dist")).handle_error(|_| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Serve error occurred while trying to serve UFC Ripper GUI files",
                )
            }),
        )
        .layer(create_ws_layer(config.clone()));

    // TCP listener
    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port)))
        .await
        .unwrap_or_quit(format!("Failed to start a listener on port \"{port}\"").as_str());

    log_success!("UFC Ripper GUI is live at http://localhost:{port}\n");

    // Axum server
    axum::serve(listener, app)
        .await
        .unwrap_or_quit("Failed to initiate the backend server");
}
