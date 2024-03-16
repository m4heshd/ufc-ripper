// Modules
mod log_util;
mod net_util;

// Libs
use crate::log_util::enable_win32_conhost_support;
use crate::net_util::init_server;

#[tokio::main]
async fn main() {
    #[cfg(target_os = "windows")]
    enable_win32_conhost_support();
    init_server().await;
}
