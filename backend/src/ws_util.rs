// Libs
use std::{
    fmt::{Debug, Display},
    time::Duration,
};

use once_cell::sync::OnceCell;
use serde::Serialize;
use serde_json::json;
use socketioxide::{
    extract::{AckSender, Data, SocketRef},
    layer::SocketIoLayer,
    SocketIo, SocketIoBuilder,
};

use ufcr_libs::{log_err, log_info, log_warn};

use crate::{
    app_util::{check_app_update, get_app_metadata},
    bin_util::{cancel_download, get_vod_formats, start_download, validate_bins},
    config_util::{get_config, is_debug, migrate_config, update_config, ConfigUpdate, UFCRConfig},
    fs_util::open_downloads_dir,
    net_util::{
        download_media_tools, get_vod_meta, get_vod_stream_url, login_to_fight_pass, search_vods,
        update_proxied_client, JsonTryGet, JSON,
    },
    rt_util::QuitUnwrap,
    state_util::{clear_inactive_dlq_vods, get_dlq, Vod},
    txt_util::create_uuid,
};

// Statics
/// Holds the global `WebSocket` instance.
static IO: OnceCell<SocketIo> = OnceCell::new();

/// Creates a new Tower layer with a `socket.io` server instance on the default namespace.
pub fn create_ws_layer() -> SocketIoLayer {
    let (layer, io) = SocketIoBuilder::new()
        .ping_timeout(Duration::from_secs(90))
        .build_layer();

    io.ns("/", |socket: SocketRef| handle_ws_client(&socket));
    IO.set(io)
        .expect("Failed to initiate the WebSocket instance on the server");

    layer
}

/// Handles each UFC Ripper GUI `WebSocket` client.
fn handle_ws_client(socket: &SocketRef) {
    log_info!("GUI connected (ID - {})\n", socket.id);

    socket.on("get-app-meta", |ack: AckSender| {
        ack.send(get_app_metadata()).ok();
    });

    socket.on("get-config", |ack: AckSender| {
        ack.send(get_config().as_ref()).ok();
    });

    socket.on("save-config", handle_save_config_event);

    socket.on("reset-config", handle_reset_config_event);

    socket.on("get-dlq", |ack: AckSender| {
        ack.send(get_dlq().clone()).ok();
    });

    socket.on("check-app-update", |ack: AckSender| async move {
        send_result(ack, check_app_update().await);
    });

    socket.on("validate-media-tools", |ack: AckSender| {
        ack.send(validate_bins()).ok();
    });

    socket.on("get-media-tools", handle_get_media_tools_event);

    socket.on("login", handle_login_event);

    socket.on("search-vods", handle_search_vods_event);

    socket.on("verify-url", handle_verify_url_event);

    socket.on("get-playable", handle_get_playable_event);

    socket.on("download", handle_download_event);

    socket.on("cancel-download", handle_cancel_download_event);

    socket.on("get-formats", handle_get_formats_event);

    socket.on("clear-dlq", |ack: AckSender| {
        clear_inactive_dlq_vods();
        ack.send(get_dlq().clone()).ok();
    });

    socket.on("open-dl-dir", |ack: AckSender| {
        send_result(ack, open_downloads_dir());
    });
}

/// Emits an event with data to all connected clients.
fn emit_to_all<T>(event: &str, data: T)
where
    T: Serialize,
{
    IO.get()
        .unwrap_or_quit("Unable to access the global WebSocket instance")
        .emit(event.to_string(), data)
        .ok();
}

/// Constructs and sends an error event to all connected clients with the provided error
fn emit_error<E>(error: E)
where
    E: Display + Debug,
{
    let error_dbg = format!("{error:#?}");
    let error_msg = error.to_string();

    if is_debug() {
        log_err!("{error_dbg}\n");
    } else {
        log_err!("{error_msg}\n");
    }

    emit_to_all(
        "server-error",
        json!({
                "name": "UFCRError",
                "message": error_dbg,
                "userMsg": error_msg
        }),
    );
}

/// Constructs and sends a warning event to all connected clients with the provided message
fn emit_warn(msg: &str) {
    log_warn!("{msg}\n");

    emit_to_all(
        "server-warning",
        json!({
                "name": "UFCRWarning",
                "message": msg
        }),
    );
}

/// Emits any updated configuration to all connected clients
pub fn emit_config_update() {
    emit_to_all("config-update", get_config().as_ref());
}

/// Sends a response to the client-event with data or an error, according to the `Result`.
fn send_result<T, E>(ack: AckSender, result: Result<T, E>)
where
    T: Serialize,
    E: Display + Debug,
{
    match result {
        Ok(data) => {
            ack.send(data).ok();
        }
        Err(error) => {
            send_error(ack, error);
        }
    }
}

/// Constructs and sends an error response to the client with the provided error
fn send_error<E>(ack: AckSender, error: E)
where
    E: Display + Debug,
{
    let error_dbg = format!("{error:#?}");
    let error_msg = error.to_string();

    if is_debug() {
        log_err!("{error_dbg}\n");
    } else {
        log_err!("{error_msg}\n");
    }

    ack.send(json!({
        "error": {
            "name": "UFCRError",
            "message": error_dbg,
            "userMsg": error_msg
        },
    }))
    .ok();
}

/// Handles the `save-config` WS event.
async fn handle_save_config_event(ack: AckSender, Data(data): Data<JSON>) {
    let finish_config_update = |ack: AckSender, new_config: UFCRConfig| async {
        update_config(ConfigUpdate::Config(Box::new(new_config))).await;
        ack.send(get_config().as_ref()).ok();

        if let Err(error) = update_proxied_client() {
            emit_error(error);
        }
    };

    if let Ok(new_config) = serde_json::from_value::<UFCRConfig>(data.clone()) {
        finish_config_update(ack, new_config).await;
    } else if let Ok(new_config_str) = serde_json::to_string(&data) {
        match migrate_config(&new_config_str) {
            Ok(migrated_config) => {
                finish_config_update(ack, migrated_config).await;
                emit_warn(
                    "Imported config file was outdated. A migration was performed before importing",
                );
            }
            Err(error) => send_error(ack, format!("Invalid configuration format. {error}")),
        }
    } else {
        send_error(
            ack,
            "Invalid configuration format. Configuration data is not valid JSON",
        );
    }
}

/// Handles the `reset-config` WS event.
async fn handle_reset_config_event(ack: AckSender) {
    update_config(ConfigUpdate::Default).await;
    ack.send(get_config().as_ref()).ok();

    if let Err(error) = update_proxied_client() {
        emit_error(error);
    }
}

/// Handles the `get-media-tools` WS event.
async fn handle_get_media_tools_event(socket: SocketRef, ack: AckSender, Data(data): Data<JSON>) {
    if let Ok(missing_tools) = serde_json::from_value::<Vec<String>>(data) {
        send_result(
            ack,
            download_media_tools(missing_tools, |tool, progress| {
                send_media_tool_download_progress(&socket, tool, progress);
            })
            .await,
        );
    } else {
        send_error(ack, "Invalid media-tools list");
    }
}

// Sends download progress of the given media tool the client that requested the download.
fn send_media_tool_download_progress(socket: &SocketRef, tool: &str, progress: f64) {
    socket
        .emit(
            "media-tool-dl-progress",
            (
                tool,
                json!({
                    "progress": progress
                }),
            ),
        )
        .ok();
}

/// Handles the `login` WS event.
async fn handle_login_event(ack: AckSender, Data(data): Data<JSON>) {
    if let (Some(region), Some(email), Some(pass)) = (
        data.try_get(0).as_str(),
        data.try_get(1).as_str(),
        data.try_get(2).as_str(),
    ) {
        match login_to_fight_pass(region, email, pass).await {
            Ok(tokens) => {
                update_config(ConfigUpdate::Region(region.to_string())).await;
                update_config(ConfigUpdate::Tokens(tokens)).await;
                ack.send(get_config().as_ref()).ok();
            }
            Err(error) => {
                send_error(ack, error);
            }
        }
    } else {
        send_error(ack, "Invalid login information");
    }
}

/// Handles the `search-vods` WS event.
async fn handle_search_vods_event(ack: AckSender, Data(data): Data<JSON>) {
    if let (Some(query), Some(page)) = (data.try_get(0).as_str(), data.try_get(1).as_u64()) {
        send_result(ack, search_vods(query, page).await);
    } else {
        send_error(ack, "Invalid search request");
    }
}

/// Handles the `verify-url` WS event.
async fn handle_verify_url_event(ack: AckSender, Data(data): Data<JSON>) {
    if let Ok(url) = serde_json::from_value::<String>(data) {
        match get_vod_meta(url.as_str()).await {
            Ok(mut meta) => {
                meta.q_id = create_uuid();

                ack.send(meta).ok();
            }
            Err(error) => send_error(ack, error),
        }
    } else {
        send_error(ack, "Invalid verify request");
    }
}

/// Handles the `get-playable` WS event.
async fn handle_get_playable_event(ack: AckSender, Data(data): Data<JSON>) {
    if let Ok(url) = serde_json::from_value::<String>(data) {
        match get_vod_meta(url.as_str()).await {
            Ok(mut vod) => {
                if !vod.access {
                    return send_error(
                        ack,
                        "You need a Fight Pass subscription to watch this video",
                    );
                }

                match get_vod_stream_url(vod.id).await {
                    Ok(hls) => {
                        vod.hls = hls;
                        vod.q_id = create_uuid();

                        ack.send(vod).ok();
                    }
                    Err(error) => send_error(ack, error),
                }
            }
            Err(error) => send_error(ack, error),
        }
    } else {
        send_error(ack, "Invalid video play request");
    }
}

/// Handles the `download` WS event.
async fn handle_download_event(ack: AckSender, Data(mut data): Data<JSON>) {
    if let (Ok(mut vod), Some(is_restart)) = (
        serde_json::from_value::<Vod>(data.try_get_mut(0, &mut JSON::Null).take()),
        data.try_get(1).as_bool(),
    ) {
        match get_vod_stream_url(vod.id).await {
            Ok(hls) => vod.hls = hls,
            Err(error) => return send_error(ack, error),
        }

        let dl = start_download(
            &vod,
            is_restart,
            move |q_id, updates| {
                emit_vod_download_progress(q_id, updates);
            },
            move |q_id| {
                emit_vod_download_progress(
                    q_id,
                    json!({
                        "status": "completed"
                    }),
                );
            },
            move |q_id, error| {
                emit_error(error);
                emit_vod_download_progress(
                    q_id,
                    json!({
                        "status": "failed"
                    }),
                );
            },
        )
        .await;

        if dl.is_ok() {
            emit_config_update();
        }

        send_result(ack, dl);
    } else {
        send_error(ack, "Invalid download request");
    }
}

/// Emits VOD download progress.
fn emit_vod_download_progress(q_id: &str, updates: JSON) {
    emit_to_all("dl-progress", (q_id, updates));
}

/// Handles the `cancel-download` WS event.
fn handle_cancel_download_event(ack: AckSender, Data(data): Data<JSON>) {
    if let Ok(vod) = serde_json::from_value::<Vod>(data) {
        send_result(ack, cancel_download(&vod));
    } else {
        send_error(ack, "Invalid download cancellation request");
    }
}

/// Handles the `get-formats` WS event.
async fn handle_get_formats_event(ack: AckSender, Data(data): Data<JSON>) {
    if let Some(url) = data.as_str() {
        let formats_result = async {
            let mut vod = get_vod_meta(url).await?;
            let hls = get_vod_stream_url(vod.id).await?;
            let formats = get_vod_formats(&hls).await?;

            vod.q_id = create_uuid();

            let response = json!({
                "VOD": vod,
                "formats": formats
            });

            Ok::<JSON, anyhow::Error>(response)
        }
        .await;

        send_result(ack, formats_result);
    } else {
        send_error(ack, "Invalid url in the formats request");
    }
}
