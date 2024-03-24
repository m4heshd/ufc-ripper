#![allow(clippy::struct_excessive_bools)]

// Libs
use std::{
    path::PathBuf,
    sync::{Arc, Mutex, MutexGuard},
};

use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};

use crate::{
    app_util::get_app_root_dir,
    fs_util::{read_config_file_to_string, write_config_to_file},
    net_util::LoginSession,
    rt_util::QuitUnwrap,
};

// Structs
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub cur_number: i64,
    pub multi_frag: bool,
    pub concur_frags: i64,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyConfig {
    pub protocol: String,
    pub host: String,
    pub port: String,
    pub auth: ProxyAuth,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyAuth {
    pub username: String,
    pub password: String,
}

// Enums
/// Specifies which fields in the configuration are being updated.
pub enum ConfigUpdate {
    Config(Box<UFCRConfig>),
    Tokens(LoginSession),
}

// Statics
static CONFIG_PATH: Lazy<PathBuf> =
    Lazy::new(|| get_app_root_dir().join("config").join("config.json"));
static CONFIG: Lazy<Arc<Mutex<UFCRConfig>>> =
    Lazy::new(|| Arc::new(Mutex::new(UFCRConfig::default())));
static DEBUG_OVERRIDE: OnceCell<bool> = OnceCell::new();

/// Loads the configuration into global CONFIG and returns a copy.
pub async fn load_config() -> UFCRConfig {
    let config = get_config_from_file().await;

    *get_mut_config() = config.clone();

    config
}

/// Gets the config.json file content and turn it into a valid `UFCRConfig`.
pub async fn get_config_from_file() -> UFCRConfig {
    let conf_file = read_config_file_to_string(&CONFIG_PATH).await;

    serde_json::from_str(&conf_file).unwrap_or_quit(
        r#"Invalid configuration format. Please reset your "config.json" file or check the configuration"#,
    )
}

/// Returns the current configuration.
pub fn get_config() -> UFCRConfig {
    CONFIG
        .lock()
        .unwrap_or_quit("Failed to exclusively access the configuration")
        .clone()
}

/// Returns the current configuration with mutability.
pub fn get_mut_config() -> MutexGuard<'static, UFCRConfig> {
    CONFIG
        .lock()
        .unwrap_or_quit("Failed to exclusively access the configuration")
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
    {
        let mut config = get_mut_config();

        match update {
            ConfigUpdate::Config(val) => *config = *val,
            ConfigUpdate::Tokens(val) => {
                config.user = val.user;
                config.refresh_token = val.refresh;
                config.auth_token = val.auth;
            }
        }
    }

    write_config_to_file(&CONFIG_PATH)
        .await
        .unwrap_or_quit(r#"An error occurred while trying to update the "config.json" file"#);
}
