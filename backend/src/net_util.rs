#![allow(clippy::missing_errors_doc)]

// Libs
use std::{net::SocketAddr, sync::Arc};

use anyhow::{anyhow, Context};
use arc_swap::ArcSwap;
use axum::{http::Method, Router};
use axum_embed::{FallbackBehavior::Redirect, ServeEmbed};
use once_cell::sync::Lazy;
use reqwest::{header::HeaderMap, Client, Proxy, Response};
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    app_util::{get_app_metadata, get_os_id, is_container},
    bin_util::BINS,
    config_util::{get_config, is_debug, update_config, ConfigUpdate, UFCRConfig},
    fs_util::{write_file_to_disk, WebAssets},
    rt_util::QuitUnwrap,
    state_util::Vod,
    txt_util::get_vod_id_from_url,
    ws_util::{create_ws_layer, emit_config_update},
};

// Structs
/// The user ID, plus a pair of authentication and refresh tokens from a successful login.
pub struct LoginSession {
    pub user: String,
    pub refresh: String,
    pub auth: String,
}

// Types
pub type JSON = Value;

// Statics
static HTTP_CLIENT: Lazy<Client> = Lazy::new(Client::new);
static HTTP_PROXIED_CLIENT: Lazy<ArcSwap<Client>> = Lazy::new(|| {
    ArcSwap::from_pointee(create_proxied_client().expect("Failed to create a proxied HTTP client"))
});
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
    let config = get_config();
    let UFCRConfig {
        port,
        open_in_browser,
        ..
    } = config.as_ref();

    let index_file = Some("index.html".to_string());
    let web_assets =
        ServeEmbed::<WebAssets>::with_parameters(index_file.clone(), Redirect, index_file);

    // Axum router
    let app = Router::new()
        .nest_service("/", web_assets)
        .layer(create_ws_layer())
        .layer(create_cors_layer());

    // TCP listener
    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], *port)))
        .await
        .unwrap_or_quit(format!("Failed to start the server on port \"{port}\"").as_str());

    log_success!(
        "UFC Ripper (v{}) GUI is live at http://localhost:{port} {}\n",
        get_app_metadata().version,
        if is_container() { "(container)" } else { "" }
    );

    if !is_container() && *open_in_browser {
        if let Err(error) = open::that(format!("http://localhost:{port}")) {
            log_err!("Failed to open the UFC Ripper GUI in default browser:\n{error}");
        }
    }

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

/// Set up the proxy according to the configuration and returns a client with the proxy enabled.
fn create_proxied_client() -> anyhow::Result<Client> {
    let proxy_conf = &get_config().proxy_config;
    let mut proxy = Proxy::all(format!(
        "{}://{}:{}",
        proxy_conf.protocol, proxy_conf.host, proxy_conf.port
    ))
    .context("Invalid proxy configuration. Please update your proxy setup with correct values")?;

    if !proxy_conf.auth.username.is_empty() {
        proxy = proxy.basic_auth(
            proxy_conf.auth.username.clone().as_str(),
            proxy_conf.auth.password.clone().as_str(),
        );
    }

    let client = Client::builder().proxy(proxy).build().context(
        "Unable to build the HTTP client with the proxy configuration. \
        Try updating the proxy setup or disabling proxied API requests",
    )?;

    Ok(client)
}

/// Updates the current proxied client with new proxy configuration.
pub fn update_proxied_client() -> anyhow::Result<()> {
    HTTP_PROXIED_CLIENT.store(Arc::new(create_proxied_client()?));

    Ok(())
}

/// Fetches UFC Ripper's update information from the GitHub repo.
pub async fn get_latest_app_meta() -> anyhow::Result<JSON> {
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
        .context("App update response contains invalid information")?;

    Ok(json_body)
}

/// Fetches all the metadata for helper media-tools.
pub async fn get_media_tools_meta() -> anyhow::Result<JSON> {
    let resp = HTTP_CLIENT
        .get("https://raw.githubusercontent.com/m4heshd/media-tools/master/versions.json")
        .send()
        .await
        .context("An error occurred while trying to retrieve media-tools information")?;

    if !resp.status().is_success() {
        return Err(anyhow!(
            "Server responded with an error for the media-tools metadata request"
        ));
    };

    let json_body: JSON = resp
        .json()
        .await
        .context("Media-tools metadata response contains invalid information")?;

    Ok(json_body)
}

/// Downloads given helper media-tools to the disk.
pub async fn download_media_tools(
    tools: Vec<String>,
    on_progress: impl Fn(&str, f64),
) -> anyhow::Result<()> {
    let media_tools_meta = get_media_tools_meta().await?[&get_os_id()].take();

    for tool in tools {
        if is_debug() {
            println!("Downloading media tool - {tool}..\n");
        }

        let url = media_tools_meta[&tool]["download"]
            .as_str()
            .context("Invalid media-tool download URL")?;

        let resp = HTTP_CLIENT.get(url).send().await.context(format!(
            "An error occurred while trying to send the media-tool({tool}) download request"
        ))?;

        let dl_size = resp
            .content_length()
            .context(format!("Invalid media-tool download data ({tool})"))?;

        write_file_to_disk(
            BINS.get_by_name(&tool)
                .context(format!("Invalid media-tool name ({tool})"))?
                .get_path(),
            dl_size,
            true,
            resp.bytes_stream(),
            |progress| on_progress(&tool, progress.round()),
        )
        .await
        .context(format!(r#"Failed to save media-tool "{tool}" to the disk"#))?;
    }

    Ok(())
}

/// Logs into the UFC Fight Pass and returns the set of auth keys included in the response.
pub async fn login_to_fight_pass(email: &str, pass: &str) -> anyhow::Result<LoginSession> {
    let proxied_client = &*HTTP_PROXIED_CLIENT.load();
    let client = if get_config().use_proxy {
        proxied_client
    } else {
        &*HTTP_CLIENT
    };

    let resp = client
        .post("https://dce-frontoffice.imggaming.com/api/v2/login")
        .headers(generate_fight_pass_api_headers()?)
        .json(&json!({
            "id": email,
            "secret": pass
        }))
        .send()
        .await
        .context("An error occurred while trying to log into the Fight Pass")?;

    if !resp.status().is_success() {
        let err_msg = "Login failed. Check your credentials and try again";
        let resp_error_messages = get_messages_from_response(resp).await.context(err_msg)?;

        if resp_error_messages.contains(&"badLocation".to_string()) {
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

/// Refreshes an expired access token and returns a new one.
pub async fn refresh_access_token() -> anyhow::Result<()> {
    if is_debug() {
        println!("Refreshing access token..\n");
    }

    let proxied_client = &*HTTP_PROXIED_CLIENT.load();
    let client = if get_config().use_proxy {
        proxied_client
    } else {
        &*HTTP_CLIENT
    };

    let resp = client
        .post("https://dce-frontoffice.imggaming.com/api/v2/token/refresh")
        .headers(generate_fight_pass_api_headers()?)
        .bearer_auth(&get_config().auth_token)
        .json(&json!({
            "refreshToken": &get_config().refresh_token
        }))
        .send()
        .await
        .context("An error occurred while trying fetch VOD metadata")?;

    if !resp.status().is_success() {
        let err_msg = "Failed to refresh your login session. Please login with your UFC Fight Pass account again";
        let resp_error_messages = get_messages_from_response(resp).await.context(err_msg)?;

        if resp_error_messages.contains(&"badLocation".to_string()) {
            return Err(anyhow!(
                "Session refresh request was blocked because of the IP address your UFC Ripper backend is bound to. \
                Try disabling any active VPN connections, or use a proxy service (check configuration)"
            ));
        } else if resp_error_messages.contains(&"errorRefreshingToken".to_string()) {
            return Err(anyhow!(
                "Invalid refresh token. Please log in with your UFC Fight Pass account again"
            ));
        }

        return Err(anyhow!(err_msg));
    };

    let json_body: JSON = resp
        .json()
        .await
        .context("Search result contains invalid response data")?;
    let auth_token = json_body["authorisationToken"].as_str();

    match auth_token {
        Some(new_auth_token) => {
            update_config(ConfigUpdate::Auth(new_auth_token.to_string())).await;
            emit_config_update();
            Ok(())
        }
        None => Err(anyhow!(
            "Server responded with an invalid response to the session refresh request"
        )),
    }
}

/// Searches the UFC Fight Pass library for VODs.
pub async fn search_vods(query: &str, page: u64) -> anyhow::Result<JSON> {
    let proxied_client = &*HTTP_PROXIED_CLIENT.load();
    let client = if get_config().use_proxy {
        proxied_client
    } else {
        &*HTTP_CLIENT
    };

    let search_params = format!(
        "{}&{}",
        VOD_SEARCH_PARAMS.as_str(),
        form_urlencoded::Serializer::new(String::new())
            .append_pair("query", query)
            .append_pair("page", &page.to_string())
            .append_pair(
                "restrictSearchableAttributes",
                if get_config().search_title_only {
                    r#"["name"]"#
                } else {
                    "[]"
                }
            )
            .finish()
    );
    let resp = client
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

/// Retrieves metadata for the given Fight Pass VOD.
pub async fn get_vod_meta(url: &str) -> anyhow::Result<Vod> {
    enum ReqStatus {
        Success(JSON),
        NeedsRefresh,
    }

    let vod_id = get_vod_id_from_url(url)?;

    // Runs the metadata request and returns the status of that request.
    // Having this as a closure allows this process to be run multiple times.
    let run_request = || async {
        let proxied_client = &*HTTP_PROXIED_CLIENT.load();
        let client = if get_config().use_proxy {
            proxied_client
        } else {
            &*HTTP_CLIENT
        };

        let resp = client
            .get(format!(
                "https://dce-frontoffice.imggaming.com/api/v2/vod/{vod_id}"
            ))
            .headers(generate_fight_pass_api_headers()?)
            .bearer_auth(&get_config().auth_token)
            .send()
            .await
            .context("An error occurred while trying fetch VOD metadata")?;

        let status = resp.status();

        if !status.is_success() {
            let err_msg = "An unknown error occurred while trying fetch VOD metadata";

            return match status.as_u16() {
                401 => {
                    let resp_error_messages =
                        get_messages_from_response(resp).await.context(err_msg)?;

                    if resp_error_messages.contains(&"Bearer token is not valid".to_string()) {
                        Ok(ReqStatus::NeedsRefresh)
                    } else {
                        Err(anyhow!(
                            r#"The server responded to the request as "Unauthorized". Please try logging in with your UFC Fight Pass account again"#
                        ))
                    }
                }
                404 => Err(anyhow!(
                    "The video you requested does not exist. Please check the URL and try again"
                )),
                _ => Err(anyhow!(err_msg)),
            };
        };

        let json_body: JSON = resp
            .json()
            .await
            .context("VOD metadata response contains invalid data")?;

        Ok::<ReqStatus, anyhow::Error>(ReqStatus::Success(json_body))
    };

    // Creates and returns a `Vod` instance from JSON
    let create_vod_from_json_meta = |meta: &JSON| {
        let err_msg = "VOD metadata response does not match the expected format";
        let vod = Vod {
            id: meta["id"].as_u64().context(err_msg)?,
            title: meta["title"]
                .as_str()
                .context(err_msg)?
                .to_string()
                .replace(':', " -"),
            desc: meta["description"].as_str().context(err_msg)?.to_string(),
            thumb: meta["thumbnailUrl"].as_str().context(err_msg)?.to_string(),
            access: meta["accessLevel"].as_str().context(err_msg)? != "DENIED",
            vod_url: url.to_string(),
            ..Vod::default()
        };

        Ok::<Vod, anyhow::Error>(vod)
    };

    match run_request().await? {
        ReqStatus::Success(vod_meta) => Ok(create_vod_from_json_meta(&vod_meta)?),
        ReqStatus::NeedsRefresh => {
            refresh_access_token().await?;

            match run_request().await? {
                ReqStatus::Success(vod_meta) => Ok(create_vod_from_json_meta(&vod_meta)?),
                ReqStatus::NeedsRefresh => Err(anyhow!(
                    r#"The server responded to the request as "Unauthorized". Please try logging in with your UFC Fight Pass account again"#
                )),
            }
        }
    }
}

/// Fetches the HLS stream URL for a given Fight Pass video.
pub async fn get_vod_stream_url(vod_id: u64) -> anyhow::Result<String> {
    let proxied_client = &*HTTP_PROXIED_CLIENT.load();
    let client = if get_config().use_proxy {
        proxied_client
    } else {
        &*HTTP_CLIENT
    };

    let resp = client
        .get(format!(
            "https://dce-frontoffice.imggaming.com/api/v3/stream/vod/{vod_id}"
        ))
        .headers(generate_fight_pass_api_headers()?)
        .bearer_auth(&get_config().auth_token)
        .send()
        .await
        .context("An error occurred while trying request the callback URL for VOD stream")?;

    if !resp.status().is_success() {
        return Err(anyhow!(
            "Server responded with an error to the callback URL request"
        ));
    }

    let json_body: JSON = resp
        .json()
        .await
        .context("Callback response contains invalid information")?;

    if let Some(url) = json_body["playerUrlCallback"].as_str() {
        let resp = client
            .get(url)
            .send()
            .await
            .context("An error occurred while trying request VOD stream URL")?;

        if !resp.status().is_success() {
            return Err(anyhow!(
                "Server responded with an error to the VOD stream request"
            ));
        }

        let json_body: JSON = resp
            .json()
            .await
            .context("Stream response contains invalid information")?;

        if let Some(url) = json_body["hls"][0]["url"].as_str() {
            Ok(url.to_string())
        } else {
            Err(anyhow!("No stream URL present in the response"))
        }
    } else {
        Err(anyhow!("No callback request URL present in the response"))
    }
}

/// Generates and returns a set of request headers required by the UFC Fight Pass.
fn generate_fight_pass_api_headers() -> anyhow::Result<HeaderMap> {
    let err_msg = r#"Invalid request-header configuration. Please check your "config.json" file"#;
    let mut headers = HeaderMap::new();

    headers.insert("app", "dice".parse().context(err_msg)?);
    headers.insert("Realm", get_config().region.parse().context(err_msg)?);
    headers.insert("x-app-var", "6.0.1.f8add0e".parse().context(err_msg)?);
    headers.insert("x-api-key", get_config().api_key.parse().context(err_msg)?);

    Ok(headers)
}

/// Deserializes and returns the `messages` array from a response.
async fn get_messages_from_response(resp: Response) -> anyhow::Result<Vec<String>> {
    let resp_messages =
        serde_json::from_value::<Vec<String>>(resp.json::<JSON>().await?["messages"].take())?;

    Ok(resp_messages)
}
