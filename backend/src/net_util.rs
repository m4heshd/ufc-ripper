#![allow(clippy::missing_errors_doc)]

// Libs
use std::net::SocketAddr;

use anyhow::{anyhow, Context, Result};
use axum::{
    http::{Method, StatusCode},
    Router,
    routing::get_service,
};
use once_cell::sync::Lazy;
use reqwest::{Client, header::HeaderMap};
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

use crate::{
    app_util::{get_app_metadata, is_container},
    config_util::get_config,
    log_success,
    rt_util::QuitUnwrap,
    ws_util::create_ws_layer,
};

// Structs
/// A pair of authentication and refresh tokens from a successful login.
pub struct LoginSession {
    pub user: String,
    pub refresh: String,
    pub auth: String,
}

// Types
pub type JSON = Value;

// Statics
static HTTP_CLIENT: Lazy<Client> = Lazy::new(Client::new);
static VOD_SEARCH_PARAMS: Lazy<String> = Lazy::new(|| {
    form_urlencoded::Serializer::new(String::new())
        .append_pair("facetFilters", r#"["type:VOD_VIDEO"]"#)
        .append_pair("hitsPerPage", "12")
        .append_pair("advancedSyntax", "true")
        .append_pair(
            "attributesToRetrieve",
            r#"["id","description","thumbnailUrl","duration"]"#,
        )
        .finish()
});

/// Creates a new server that serves the UFC Ripper GUI and the `WebSocket` server.
///
/// # Panics
///
/// Will panic if the port is already in use or fails to serve the Vue "dist" directory.
pub async fn init_server() {
    let port = get_config().port;

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
    let resp = HTTP_CLIENT
        .get(req_url)
        .send()
        .await
        .context("An error occurred while trying to retrieve app update information")?;

    if !resp.status().is_success() {
        return Err(anyhow!(
            "Server responded with an error for the app update check"
        ));
    };

    let json_body: JSON = resp
        .json()
        .await
        .context("App update information contains an invalid response")?;

    Ok(json_body)
}

/// Generates and returns a set of request headers required by the UFC Fight Pass.
fn get_fight_pass_api_headers() -> Result<HeaderMap> {
    let err_msg = r#"Invalid request-header configuration. Please check your "config.json" file"#;
    let mut headers = HeaderMap::new();

    headers.insert("app", "dice".parse().context(err_msg)?);
    headers.insert("Realm", get_config().region.parse().context(err_msg)?);
    headers.insert("x-app-var", "6.0.1.f8add0e".parse().context(err_msg)?);
    headers.insert("x-api-key", get_config().api_key.parse().context(err_msg)?);

    Ok(headers)
}

/// Logs into the UFC Fight Pass and returns the set of auth keys included in the response.
pub async fn login_to_fight_pass(email: &str, pass: &str) -> Result<LoginSession> {
    let resp = HTTP_CLIENT
        .post("https://dce-frontoffice.imggaming.com/api/v2/login")
        .headers(get_fight_pass_api_headers()?)
        .json(&json!({
            "id": email,
            "secret": pass
        }))
        .send()
        .await
        .context("An error occurred while trying to log into the Fight Pass")?;

    if !resp.status().is_success() {
        let err_msg = "Login failed. Check your credentials and try again";
        let login_error_messages = serde_json::from_value::<Vec<String>>(
            resp.json::<JSON>().await.context(err_msg)?["messages"].take(),
        )
        .context(err_msg)?;

        if login_error_messages.contains(&"badLocation".to_string()) {
            return Err(anyhow!(
                "Login was blocked because of the IP address your UFC Ripper backend is bound to. \
                Try disabling any active VPN connections, or use a proxy service (check configuration)"
            ));
        }

        return Err(anyhow!(err_msg));
    };

    let err_msg = "Login information contains an invalid response";
    let json_body: JSON = resp.json().await.context(err_msg)?;

    if let (Some(auth), Some(refresh)) = (
        json_body["authorisationToken"].as_str(),
        json_body["refreshToken"].as_str(),
    ) {
        Ok(LoginSession {
            user: email.to_string(),
            auth: auth.to_string(),
            refresh: refresh.to_string(),
        })
    } else {
        Err(anyhow!(err_msg))
    }
}

/// Searches the UFC Fight Pass library for VODs.
pub async fn search_vods(query: &str, page: u64) -> Result<JSON> {
    let search_params = format!(
        "{}&{}",
        VOD_SEARCH_PARAMS.as_str(),
        form_urlencoded::Serializer::new(String::new())
            .append_pair("query", query)
            .append_pair("page", &page.to_string())
            .finish()
    );
    let resp = HTTP_CLIENT
        .post("https://h99xldr8mj-dsn.algolia.net/1/indexes/*/queries")
        .header("x-algolia-application-id", "H99XLDR8MJ")
        .header("x-algolia-api-key", &get_config().search_api_key)
        .json(&json!({
            "requests": [
                {
                    "indexName": "prod-dce.ufc-livestreaming-events",
                    "params": search_params
                }
            ]
        }))
        .send()
        .await
        .context("An error occurred while trying to search the Fight Pass library")?;

    if !resp.status().is_success() {
        return Err(anyhow!(
            "Server responded with an error for the search request"
        ));
    };

    let json_body: JSON = resp
        .json()
        .await
        .context("Search result contains an invalid response")?;
    let result = &json_body["results"][0];

    if result == &JSON::Null {
        Err(anyhow!("Response does not contain any search results"))
    } else {
        Ok(result.clone())
    }
}
