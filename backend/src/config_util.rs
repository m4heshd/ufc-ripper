#![allow(clippy::struct_excessive_bools)]

// Libs
use std::{path::PathBuf, sync::Arc};

use arc_swap::{ArcSwap, Guard};
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};

use crate::{
    app_util::get_app_root_dir,
    fs_util::{read_config_file_to_string, write_config_to_file},
    net_util::LoginSession,
    rt_util::QuitUnwrap,
};

// Structs
#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UFCRConfig {
    pub open_in_browser: bool,
    pub port: u16,
    pub verbose_logging: bool,
    pub api_key: String,
    #[serde(rename = "searchAPIKey")]
    pub search_api_key: String,
    pub region: String,
    pub user: String,
    pub refresh_token: String,
    pub auth_token: String,
    pub search_title_only: bool,
    pub show_thumb: bool,
    pub show_duration: bool,
    pub show_desc: bool,
    pub resolution: String,
    pub merge_ext: String,
    pub vid_quality: String,
    pub aud_quality: String,
    pub dl_path: String,
    pub number_files: bool,
    pub cur_number: u64,
    pub multi_frag: bool,
    pub concur_frags: u64,
    pub throttle: bool,
    pub dl_rate: String,
    pub cus_format: bool,
    #[serde(rename = "formatID")]
    pub format_id: String,
    pub metadata: bool,
    pub use_proxy: bool,
    pub proxy_config: ProxyConfig,
    pub dl_args: Vec<String>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyConfig {
    pub protocol: String,
    pub host: String,
    pub port: u16,
    pub auth: ProxyAuth,
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyAuth {
    pub username: String,
    pub password: String,
}

// Enums
/// Specifies which fields in the configuration are being updated.
pub enum ConfigUpdate {
    Config(Box<UFCRConfig>),
    Region(String),
    Auth(String),
    Tokens(LoginSession),
    FileNum(u64),
}

// Statics
static CONFIG_PATH: Lazy<PathBuf> =
    Lazy::new(|| get_app_root_dir().join("config").join("config.json"));
static CONFIG: Lazy<ArcSwap<UFCRConfig>> =
    Lazy::new(|| ArcSwap::from_pointee(UFCRConfig::default()));
static DEBUG_OVERRIDE: OnceCell<bool> = OnceCell::new();

/// Loads the configuration into global CONFIG.
pub async fn load_config() {
    update_config(ConfigUpdate::Config(Box::new(get_config_from_file().await))).await;
}

/// Gets the config.json file content and turn it into a valid `UFCRConfig`.
pub async fn get_config_from_file() -> UFCRConfig {
    let conf_file = read_config_file_to_string(&CONFIG_PATH).await;

    serde_json::from_str(&conf_file).unwrap_or_quit(
        r#"Invalid configuration format. Please reset your "config.json" file or check the configuration"#,
    )
}

/// Returns the current configuration.
pub fn get_config() -> Guard<Arc<UFCRConfig>> {
    CONFIG.load()
}

/// Returns the debug status.
pub fn is_debug() -> bool {
    match DEBUG_OVERRIDE.get() {
        Some(debug) => *debug || get_config().verbose_logging,
        None => get_config().verbose_logging,
    }
}

/// Updates the configuration with new data and writes to config.json.
pub async fn update_config(update: ConfigUpdate) {
    let mut new_config = get_config().as_ref().clone();

    match update {
        ConfigUpdate::Config(data) => new_config = *data,
        ConfigUpdate::Region(data) => new_config.region = data,
        ConfigUpdate::Auth(data) => new_config.auth_token = data,
        ConfigUpdate::Tokens(data) => {
            new_config.user = data.user;
            new_config.refresh_token = data.refresh;
            new_config.auth_token = data.auth;
        }
        ConfigUpdate::FileNum(data) => new_config.cur_number = data,
    }

    CONFIG.store(Arc::new(new_config));
    write_config_to_file(&CONFIG_PATH)
        .await
        .unwrap_or_quit(r#"An error occurred while trying to update the "config.json" file"#);
}

/// Increases the current file number by one
pub async fn inc_file_number() {
    let config = get_config();

    if config.number_files {
        update_config(ConfigUpdate::FileNum(config.cur_number + 1)).await;
    }
}
