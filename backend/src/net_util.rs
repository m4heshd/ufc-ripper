#![allow(clippy::missing_errors_doc)]

// Libs
use crate::{
    app_util::get_app_metadata, app_util::is_container, config_util::UFCRConfig, log_success,
    rt_util::QuitUnwrap, ws_util::create_ws_layer,
};
use anyhow::{Context, Result};
use axum::{
    http::{Method, StatusCode},
    routing::get_service,
    Router,
};
use once_cell::sync::Lazy;
use reqwest::Client;
use serde_json::Value;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

// Types
pub type JSON = Value;

// Statics
static HTTP_CLIENT: Lazy<Client> = Lazy::new(Client::new);

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
        .layer(create_ws_layer())
        .layer(create_cors_layer());

    // TCP listener
    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port)))
        .await
        .unwrap_or_quit(format!("Failed to start the server on port \"{port}\"").as_str());

    log_success!(
        "UFC Ripper (v{}) GUI is live at http://localhost:{port} {}\n",
        get_app_metadata().version,
        if is_container() { "(container)" } else { "" }
    );

    // Axum server
    axum::serve(listener, app)
        .await
        .unwrap_or_quit("Failed to initiate the backend server");
}

/// Creates a new Tower layer with CORS rules.
fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any)
}

/// Fetches UFC Ripper's update information from the GitHub repo.
pub async fn get_latest_app_meta() -> Result<JSON> {
    let req_url = format!("{}/raw/master/package.json", get_app_metadata().repo);
    let resp: JSON = HTTP_CLIENT
        .get(req_url)
        .send()
        .await
        .context("An error occurred while trying to retrieve app update information")?
        .json()
        .await
        .context("App update information contains an invalid response")?;

    Ok(resp)
}
