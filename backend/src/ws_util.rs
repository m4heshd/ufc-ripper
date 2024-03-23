// Libs
use crate::{
    app_util::{check_app_update, get_app_metadata},
    bin_util::validate_bins,
    config_util::{get_config, is_debug},
    net_util::{search_vods, JSON},
    state_util::get_dlq,
};
use serde::Serialize;
use serde_json::json;
use socketioxide::{
    extract::{AckSender, Data, SocketRef},
    layer::SocketIoLayer,
    SocketIoBuilder,
};
use std::{
    fmt::{Debug, Display},
    time::Duration,
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
    socket.on("get-dlq", |ack: AckSender| {
        ack.send(get_dlq()).ok();
    });
    socket.on("check-app-update", |ack: AckSender| async move {
        send_result(ack, check_app_update().await);
    });
    socket.on("validate-media-tools", |ack: AckSender| {
        ack.send(validate_bins()).ok();
    });
    socket.on(
        "search-vods",
        |ack: AckSender, Data(data): Data<JSON>| async move {
            let query = data[0].as_str();
            let page = data[1].as_u64();

            if query.is_none() || page.is_none() {
                send_error(ack, "Invalid search request");
            } else {
                send_result(ack, search_vods(query.unwrap(), page.unwrap()).await);
            }
        },
    );
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
        log_err!("{error_dbg}");
    } else {
        log_err!("{error_msg}");
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
