// Libs
use serde::{Deserialize, Serialize};
use std::env;

// Structs
/// Holds all metadata related to the UFC Ripper application.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppMeta {
    is_container: bool,
    version: String,
}

/// Populates and returns application's metadata as a new instance of `AppMeta`.
pub fn get_app_metadata() -> AppMeta {
    AppMeta {
        is_container: is_container(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

/// Determines if the application is running inside a container using the `RUN_ENV` environment variable.
pub fn is_container() -> bool {
    match env::var("RUN_ENV") {
        Ok(run_env) => run_env == "container",
        Err(_) => false,
    }
}
