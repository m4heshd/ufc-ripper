// Libs
use std::env;

/// Determines if the application is running inside a container using the `RUN_ENV` environment variable.
pub fn is_container() -> bool {
    match env::var("RUN_ENV") {
        Ok(run_env) => run_env == "container",
        Err(_) => false,
    }
}
