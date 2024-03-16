// Libs
use ufcr_libs::log_util::enable_win32_conhost_support;
use ufcr_libs::net_util::init_server;

#[tokio::main]
async fn main() {
    #[cfg(target_os = "windows")]
    enable_win32_conhost_support();
    init_server().await;
}
