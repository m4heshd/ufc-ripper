#![allow(clippy::struct_excessive_bools)]

// Libs
use crate::rt_util::QuitUnwrap;
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{
    env::current_exe,
    fs,
    io::{BufWriter, Write},
    path::PathBuf,
    sync::{Mutex, MutexGuard},
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
    pub search_apikey: String,
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

// Statics
static CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    if cfg!(debug_assertions) {
        [".", "config", "config.json"].iter().collect::<PathBuf>()
    } else {
        current_exe()
            .unwrap_or_quit("Failed to determine the application's executable path")
            .parent()
            .unwrap_or_quit("Invalid executable path")
            .join("config")
            .join("config.json")
    }
});
static CONFIG: Lazy<Arc<Mutex<UFCRConfig>>> =
    Lazy::new(|| Arc::new(Mutex::new(UFCRConfig::default())));
static DEBUG_OVERRIDE: OnceCell<bool> = OnceCell::new();

/// Loads the configuration into global CONFIG and returns a copy.
#[must_use]
pub fn load_config() -> UFCRConfig {
    let config = read_config();

    update_config(config.clone());

    config
}

/// Reads the config.json file from the disk.
#[must_use]
pub fn read_config() -> UFCRConfig {
    let conf_file = fs::read_to_string(&*CONFIG_PATH).unwrap_or_quit(
        r#"Unable to read config.json file. Check if the file exists in "config" directory"#,
    );

    serde_json::from_str(&conf_file).unwrap_or_quit(
        r#"Invalid configuration format. Please reset your "config.json" file or check the configuration"#,
    )
}

/// Writes the current configuration to config.json file.
pub fn write_config() {
    (|| -> std::io::Result<()> {
        let conf_file = fs::File::create(&*CONFIG_PATH)?;
        let mut writer = BufWriter::new(conf_file);

        serde_json::to_writer_pretty(&mut writer, &get_config())?;
        writer.flush()?;

        Ok(())
    })()
    .unwrap_or_quit("An error occurred while trying to update the config.json file");
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
pub fn update_config(update: UFCRConfig) {
    *get_mut_config() = update;
    write_config();
}
