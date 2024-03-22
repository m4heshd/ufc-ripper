// Libs
use crate::{
    net_util::{get_latest_app_meta, JSON},
    rt_util::QuitUnwrap,
};
use anyhow::{Context, Result};
use path_absolutize::Absolutize;
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, path::PathBuf};

// Structs
/// Holds all metadata related to the UFC Ripper application.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppMeta {
    pub is_container: bool,
    pub version: String,
    pub repo: String,
}

/// Populates and returns application's metadata as a new instance of `AppMeta`.
pub fn get_app_metadata() -> AppMeta {
    AppMeta {
        is_container: is_container(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        repo: env!("CARGO_PKG_REPOSITORY").to_string(),
    }
}

/// Determines if the application is running inside a container using the `RUN_ENV`
/// environment variable.
pub fn is_container() -> bool {
    match env::var("RUN_ENV") {
        Ok(run_env) => run_env == "container",
        Err(_) => false,
    }
}

/// Returns the application's root directory, depending on the compiled mode.
pub fn get_app_root_dir() -> PathBuf {
    let err_msg = "Failed to determine the application's root directory";

    if cfg!(debug_assertions) {
        PathBuf::from(".")
            .absolutize()
            .unwrap_or_quit(err_msg)
            .to_path_buf()
    } else {
        env::current_exe()
            .unwrap_or_quit(err_msg)
            .parent()
            .unwrap_or_quit(format!("{err_msg}. Invalid executable path").as_str())
            .to_path_buf()
    }
}

/// Checks if the application has a newer release than the current version
pub async fn check_app_update() -> Result<JSON> {
    let err_msg = "Invalid version information in the app update-check response";
    let remote_meta = get_latest_app_meta().await?;
    let version =
        Version::parse(remote_meta["version"].as_str().context(err_msg)?).context(err_msg)?;

    if version > Version::parse(get_app_metadata().version.as_str())? {
        Ok(json!({
            "updatable": true,
            "version": version.to_string(),
            "url": format!("{}/releases/latest", get_app_metadata().repo)
        }))
    } else {
        Ok(json!({
            "updatable": false
        }))
    }
}
