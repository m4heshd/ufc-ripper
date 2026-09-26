// Libs
use ufcr_util::{
    app_util::is_container,
    config_util::{is_debug, load_config},
    net_util::init_server,
    rt_util::{set_custom_panic, ExitHandler},
};

#[tokio::main]
async fn main() {
    set_custom_panic(true);

    // This needs to be here, so it would be the last thing that will be dropped
    let _exit_handler = if is_container() {
        None
    } else {
        Some(ExitHandler)
    };

    #[cfg(target_os = "windows")]
    ufcr_libs::log_util::enable_win32_conhost_support();
    start_ufcr().await;
}

/// Initializes the configuration and starts the application process.
async fn start_ufcr() {
    load_config().await;
    set_custom_panic(is_debug());
    ufcr_util::auto_follow::start();
    init_server().await;
}
