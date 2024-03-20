// Libs
use crate::config_util::UFCRConfig;
use socketioxide::{
    extract::{AckSender, SocketRef},
    layer::SocketIoLayer,
    SocketIoBuilder,
};
use std::time::Duration;

/// Handles each UFC Ripper GUI `WebSocket` client
fn handle_ws_client(socket: &SocketRef, config: UFCRConfig) {
    log_info!("GUI connected (ID - {})\n", socket.id);

    socket.on("get-config", |ack: AckSender| {
        ack.send(config).ok();
    });
}

/// Creates a new Tower layer with a `socket.io` server instance on the default namespace.
pub fn create_ws_layer(config: UFCRConfig) -> SocketIoLayer {
    let (layer, io) = SocketIoBuilder::new()
        .ping_timeout(Duration::from_secs(90))
        .build_layer();

    io.ns("/", |socket: SocketRef| handle_ws_client(&socket, config));

    layer
}
