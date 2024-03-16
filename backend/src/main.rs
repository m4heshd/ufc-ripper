// Modules
mod net_util;

// Libs
use crate::net_util::init_server;

#[tokio::main]
async fn main() {
    init_server().await;
}
