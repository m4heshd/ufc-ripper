// Libs
use ufcr_libs::{
    config_util::{is_debug, load_config},
    net_util::init_server,
    rt_util::{ExitHandler, set_custom_panic},
};

#[tokio::main]
async fn main() {
    set_custom_panic(true);

    let _exit_handler = ExitHandler; // This needs to be here, so it would be the last thing that will be dropped

    #[cfg(target_os = "windows")]
    ufcr_libs::log_util::enable_win32_conhost_support();
    start_ufcr().await;
}

/// Initializes the configuration and starts the application process.
async fn start_ufcr() {
    let config = load_config();

    set_custom_panic(is_debug());
    init_server(&config).await;
}
