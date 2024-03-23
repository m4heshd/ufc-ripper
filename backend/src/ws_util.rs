// Libs
use std::{
    fmt::{Debug, Display},
    time::Duration,
};

use serde::Serialize;
use serde_json::json;
use socketioxide::{
    extract::{AckSender, Data, SocketRef},
    layer::SocketIoLayer,
    SocketIoBuilder,
};

use crate::{
    app_util::{check_app_update, get_app_metadata},
    bin_util::validate_bins,
    config_util::{ConfigUpdate, get_config, is_debug, UFCRConfig, update_config},
    fs_util::open_downloads_dir,
    net_util::{JSON, login_to_fight_pass, search_vods},
    state_util::get_dlq,
};

/// Creates a new Tower layer with a `socket.io` server instance on the default namespace.
pub fn create_ws_layer() -> SocketIoLayer {
    let (layer, io) = SocketIoBuilder::new()
        .ping_timeout(Duration::from_secs(90))
        .build_layer();

    io.ns("/", |socket: SocketRef| handle_ws_client(&socket));

    layer
}

/// Handles each UFC Ripper GUI `WebSocket` client.
fn handle_ws_client(socket: &SocketRef) {
    log_info!("GUI connected (ID - {})\n", socket.id);

    socket.on("get-app-meta", |ack: AckSender| {
        ack.send(get_app_metadata()).ok();
    });

    socket.on("get-config", |ack: AckSender| {
        ack.send(get_config()).ok();
    });

    socket.on("save-config", handle_save_config_event);

    socket.on("get-dlq", |ack: AckSender| {
        ack.send(get_dlq()).ok();
    });

    socket.on("check-app-update", |ack: AckSender| async move {
        send_result(ack, check_app_update().await);
    });

    socket.on("validate-media-tools", |ack: AckSender| {
        ack.send(validate_bins()).ok();
    });

    socket.on("login", handle_login_event);

    socket.on("search-vods", handle_search_vods_event);

    socket.on("open-dl-dir", |ack: AckSender| {
        send_result(ack, open_downloads_dir());
    });
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
fn handle_save_config_event(ack: AckSender, Data(data): Data<JSON>) {
    if let Ok(new_config) = serde_json::from_value::<UFCRConfig>(data) {
        update_config(ConfigUpdate::Config(Box::new(new_config)));
        ack.send(get_config()).ok();
    } else {
        send_error(ack, "Invalid configuration format");
    }
}

/// Handles the `login` WS event.
async fn handle_login_event(ack: AckSender, Data(data): Data<JSON>) {
    if let (Some(email), Some(pass)) = (data[0].as_str(), data[1].as_str()) {
        match login_to_fight_pass(email, pass).await {
            Ok(tokens) => {
                update_config(ConfigUpdate::Tokens(tokens));
                ack.send(get_config()).ok();
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
    if let (Some(query), Some(page)) = (data[0].as_str(), data[1].as_u64()) {
        send_result(ack, search_vods(query, page).await);
    } else {
        send_error(ack, "Invalid search request");
    }
}
