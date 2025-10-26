// Libs
use std::{env, env::consts::ARCH, env::consts::OS, path::PathBuf};
use anyhow::Context;
use path_absolutize::Absolutize;
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    net_util::{get_latest_app_meta, JSON, JsonTryGet},
    rt_util::QuitUnwrap,
};

// Structs
/// Holds all metadata related to the UFC Ripper application.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppMeta {
    pub is_container: bool,
    pub version: &'static str,
    pub repo: &'static str,
}

/// Populates and returns application's metadata as a new instance of `AppMeta`.
pub fn get_app_metadata() -> AppMeta {
    AppMeta {
        is_container: is_container(),
        version: env!("CARGO_PKG_VERSION"),
        repo: env!("CARGO_PKG_REPOSITORY"),
    }
}

/// Returns a Node.js-like platform name, which the application is currently running on.
pub fn get_os_id() -> String {
    match OS {
        "windows" => "win32".to_string(),
        "linux" => "linux".to_string(),
        _ => "unsupported".to_string(),
    }
}

/// Returns a friendly platform architecture name, which the application is currently running on.
pub fn get_os_arch() -> String {
    match ARCH {
        "x86_64" => "x64".to_string(),
        _ => "unsupported".to_string(),
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
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap_or_quit(err_msg)
            .absolutize()
            .unwrap_or_quit(err_msg)
            .to_path_buf()
    } else {
        env::current_exe()
            .unwrap_or_quit(err_msg)
            .parent()
            .unwrap_or_quit(format!("{err_msg}. Invalid executable path"))
            .to_path_buf()
    }
}

/// Checks if the application has a newer release than the current version
pub async fn check_app_update() -> anyhow::Result<JSON> {
    let err_msg = "Invalid version information in the app update-check response";
    let remote_meta = get_latest_app_meta().await?;
    let version = Version::parse(remote_meta.try_get("version").as_str().context(err_msg)?)
        .context(err_msg)?;

    if version > Version::parse(get_app_metadata().version)? {
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

/*************
 *   Tests   *
 *************/

#[cfg(test)]
mod tests {
    use super::is_container;

    #[test]
    fn unit_is_container() {
        assert!(!is_container());
    }
}
