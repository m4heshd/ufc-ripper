// Libs
use ufcr_libs::log_util::enable_win32_conhost_support;
use ufcr_libs::net_util::init_server;
use ufcr_libs::rt_util::ExitHandler;

#[tokio::main]
async fn main() {
    let _exit_handler = ExitHandler; // This needs to be here, so it would be the last thing that will be dropped

    #[cfg(target_os = "windows")]
    enable_win32_conhost_support();
    init_server().await;
}
