// Libs
use crate::{
    app_util::{check_app_update, get_app_metadata},
    config_util::get_config,
    state_util::get_dlq,
};
use serde_json::json;
use socketioxide::{
    extract::{AckSender, SocketRef},
    layer::SocketIoLayer,
    SocketIoBuilder,
};
use std::time::Duration;

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
        match check_app_update().await {
            Ok(data) => {
                ack.send(data).ok();
            }
            Err(error) => {
                ack.send(json!({"error": error.to_string()})).ok();
            }
        }
    });
}

/// Creates a new Tower layer with a `socket.io` server instance on the default namespace.
pub fn create_ws_layer() -> SocketIoLayer {
    let (layer, io) = SocketIoBuilder::new()
        .ping_timeout(Duration::from_secs(90))
        .build_layer();

    io.ns("/", |socket: SocketRef| handle_ws_client(&socket));

    layer
}
