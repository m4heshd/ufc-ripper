// Libs
use crate::{
    app_util::{check_app_update, get_app_metadata},
    bin_util::validate_bins,
    config_util::get_config,
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
use std::{fmt::Display, future::Future, time::Duration};

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
        send_async_result(ack, check_app_update()).await;
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
                send_async_result(ack, search_vods(query.unwrap(), page.unwrap())).await;
            }
        },
    );
}

/// Sends a response to the client-event with data or an error, according to the awaited `Result`.
async fn send_async_result<T, E>(ack: AckSender, future: impl Future<Output = Result<T, E>>)
where
    T: Serialize,
    E: Display,
{
    send_result(ack, future.await);
}

/// Sends a response to the client-event with data or an error, according to the `Result`.
fn send_result<T, E>(ack: AckSender, result: Result<T, E>)
where
    T: Serialize,
    E: Display,
{
    match result {
        Ok(data) => {
            ack.send(data).ok();
        }
        Err(error) => {
            send_error(ack, &error.to_string());
        }
    }
}

/// Sends an error response to the client with the provided message
fn send_error(ack: AckSender, error_msg: &str) {
    ack.send(json!({
        "error": error_msg
    }))
    .ok();
}
