// Libs
use anyhow::{anyhow, Context, Result};
use reqwest::Url;
use uuid::Uuid;

/// Creates a UUID and returns it as a `String`
pub fn create_uuid() -> String {
    Uuid::new_v4().to_string()
}

/// Validates a Fight Pass VOD URL and returns the VOD ID.
pub fn get_vod_id_from_url(url: &str) -> Result<String> {
    let err_msg = "Provided URL is invalid";

    match Url::parse(url)
        .context(err_msg)?
        .path()
        .split("/video/")
        .nth(1)
    {
        Some(split_id) => match split_id.split('/').next() {
            Some(vod_id) => Ok(vod_id.to_string()),
            None => Err(anyhow!(err_msg)),
        },
        None => Err(anyhow!(err_msg)),
    }
}
