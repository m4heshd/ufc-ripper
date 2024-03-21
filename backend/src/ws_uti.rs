// Libs
use crate::{app_util::get_app_metadata, config_util::get_config, state_util::get_dlq};
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
}

/// Creates a new Tower layer with a `socket.io` server instance on the default namespace.
pub fn create_ws_layer() -> SocketIoLayer {
    let (layer, io) = SocketIoBuilder::new()
        .ping_timeout(Duration::from_secs(90))
        .build_layer();

    io.ns("/", |socket: SocketRef| handle_ws_client(&socket));

    layer
}
