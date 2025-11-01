#![allow(clippy::struct_excessive_bools)]

// Libs
use std::{path::PathBuf, sync::Arc};

use anyhow::Context;
use arc_swap::{ArcSwap, Guard};
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};

use crate::{
    app_util::get_app_root_dir,
    fs_util::{
        build_downloads_dir_path, create_config_backup, read_config_file_to_string,
        write_config_to_file,
    },
    net_util::{LoginSession, JSON},
    rt_util::QuitUnwrap,
};

use ufcr_libs::{log_err, log_success, log_warn};

// Structs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub use_temp_path: bool,
    pub temp_path: String,
    pub number_files: bool,
    pub cur_number: u64,
    pub multi_frag: bool,
    pub concur_frags: u64,
    pub throttle: bool,
    pub dl_rate: String,
    pub cus_format: bool,
    #[serde(rename = "formatID")]
    pub format_id: String,
    pub use_proxy: bool,
    pub proxy_config: ProxyConfig,
    pub dl_args: Vec<String>,
}

impl Default for UFCRConfig {
    fn default() -> Self {
        UFCRConfig {
            open_in_browser: true,
            port: 8383,
            verbose_logging: false,
            api_key: "857a1e5d-e35e-4fdf-805b-a87b6f8364bf".into(),
            search_api_key: "e55ccb3db0399eabe2bfc37a0314c346".into(),
            region: "dce.ufc".into(),
            user: String::new(),
            refresh_token: String::new(),
            auth_token: String::new(),
            search_title_only: false,
            show_thumb: true,
            show_duration: true,
            show_desc: true,
            resolution: "720".into(),
            merge_ext: "mp4".into(),
            vid_quality: "bestvideo".into(),
            aud_quality: "bestaudio".into(),
            dl_path: String::new(),
            use_temp_path: false,
            temp_path: String::new(),
            number_files: true,
            cur_number: 1,
            multi_frag: true,
            concur_frags: 64,
            throttle: false,
            dl_rate: "100K".into(),
            cus_format: false,
            format_id: String::new(),
            use_proxy: false,
            proxy_config: ProxyConfig::default(),
            dl_args: vec![
                "--no-warnings".into(),
                "--no-mtime".into(),
                "--output-na-placeholder".into(),
                "\"\"".into(),
                "--no-cache-dir".into(),
                "--ignore-config".into(),
                "--no-check-certificate".into(),
            ],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyConfig {
    pub protocol: String,
    pub host: String,
    pub port: u16,
    pub auth: ProxyAuth,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        ProxyConfig {
            protocol: "http".into(),
            host: "0.0.0.0".into(),
            port: 1111,
            auth: ProxyAuth::default(),
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyAuth {
    pub username: String,
    pub password: String,
}

// Enums
/// Specifies which fields in the configuration are being updated.
pub enum ConfigUpdate {
    Default,
    Config(Box<UFCRConfig>),
    Region(String),
    Auth(String),
    Tokens(LoginSession),
    FileNum(u64),
}

// Statics
pub static CONFIG_PATH: Lazy<PathBuf> =
    Lazy::new(|| get_app_root_dir().join("config").join("config.json"));
static CONFIG: Lazy<ArcSwap<UFCRConfig>> =
    Lazy::new(|| ArcSwap::from_pointee(UFCRConfig::default()));
static DEBUG_OVERRIDE: OnceCell<bool> = OnceCell::new();

/// Loads the configuration into global CONFIG.
pub async fn load_config() {
    update_config(ConfigUpdate::Config(Box::new(get_config_from_file().await))).await;
}

/// Merges old values of the configuration recursively into new values
fn merge_config_data(new_data: &mut JSON, old_data: JSON) {
    match (new_data, old_data) {
        (JSON::Object(ref mut new_obj), JSON::Object(old_obj)) => {
            for (key, val) in old_obj {
                if let Some(sub_value) = new_obj.get_mut(&key) {
                    match (&sub_value, &val) {
                        (JSON::Object(_), JSON::Object(_)) => merge_config_data(sub_value, val),
                        (JSON::Array(_), JSON::Array(_))
                        | (JSON::String(_), JSON::String(_))
                        | (JSON::Number(_), JSON::Number(_))
                        | (JSON::Bool(_), JSON::Bool(_))
                        | (JSON::Null, JSON::Null) => {
                            *sub_value = val;
                        }
                        _ => {}
                    }
                }
            }
        }
        (new_val, old_val) => {
            *new_val = old_val;
        }
    }
}

/// Migrates an outdated configuration into the current one
pub fn migrate_config(old_config_str: &str) -> anyhow::Result<UFCRConfig> {
    let old_config =
        serde_json::from_str(old_config_str).context("Provided config file is not valid JSON")?;
    let mut new_config = serde_json::to_value(UFCRConfig::default())
        .context("Failed to convert default configuration to JSON")?;

    merge_config_data(&mut new_config, old_config);

    let migrated_config: UFCRConfig = serde_json::from_value(new_config)
        .context("Failed to generate a configuration from migrated data")?;

    Ok(migrated_config)
}

/// Gets the config.json file content and turn it into a valid `UFCRConfig`.
pub async fn get_config_from_file() -> UFCRConfig {
    let conf_file = read_config_file_to_string(&CONFIG_PATH).await;

    let mut config: UFCRConfig = if let Ok(conf) = serde_json::from_str(&conf_file) {
        conf
    } else {
        log_warn!("Invalid configuration format detected. Attempting to migrate..\n");

        match migrate_config(&conf_file) {
            Ok(migrated_conf) => {
                log_success!("Migration process successful. Backing up the old config file..\n");

                create_config_backup(&CONFIG_PATH, "config_backup.json").await
                    .unwrap_or_quit("Failed to create a backup of the old configuration. Please try manually updating the config file");

                log_success!("Backup file successfully created. You can find the backup file in config directory named \"config_backup.json\"\n");

                migrated_conf
            }
            Err(error) => {
                log_err!(
                    "{}. Reverting to default configuration..\n",
                    error.to_string()
                );
                UFCRConfig::default()
            }
        }
    };

    config.dl_path = build_downloads_dir_path(config.dl_path)
        .unwrap_or_quit("Failed to build the path for user's downloads directory");

    config
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
        ConfigUpdate::Default => new_config = UFCRConfig::default(),
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

/*************
 *   Tests   *
 *************/

#[cfg(test)]
mod tests {
    use crate::{fs_util::build_downloads_dir_path, rt_util::set_custom_panic};

    use super::{get_config, is_debug, load_config, UFCRConfig};

    #[tokio::test]
    async fn unit_load_config() {
        set_custom_panic(true);
        load_config().await;
    }

    #[tokio::test]
    async fn unit_get_config() {
        load_config().await;

        let config = get_config();
        let default_config = UFCRConfig {
            dl_path: build_downloads_dir_path(config.dl_path.clone()).unwrap(),
            ..UFCRConfig::default()
        };

        assert_eq!(config.as_ref(), &default_config);
    }

    #[test]
    fn unit_is_debug() {
        assert!(!is_debug());
    }
}
