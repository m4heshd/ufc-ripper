// Libs
use anyhow::{anyhow, Context, Result};
use regex_lite::Regex;
use reqwest::Url;
use serde_json::json;
use uuid::Uuid;

use crate::net_util::JSON;

/// Creates a UUID and returns it as a `String`.
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

/// Processes stdout lines from a `yt-dlp` process and returns the progress status as JSON.
pub fn process_yt_dlp_stdout(line: &str) -> JSON {
    if let Ok(dl_stat_json) = serde_json::from_str::<JSON>(line) {
        let size = dl_stat_json["size"].as_str().unwrap_or("").trim();
        let speed = dl_stat_json["speed"].as_str().unwrap_or("").trim();
        let eta = dl_stat_json["eta"].as_str().unwrap_or("").trim();
        let task = if let Some(vcodec) = dl_stat_json["vcodec"].as_str() {
            if vcodec == "none" {
                "audio"
            } else {
                "video"
            }
        } else {
            "audio"
        };

        let progress: f64 = if let (Some(total_size), Some(dl_size)) = (
            dl_stat_json["total_size"].as_f64(),
            dl_stat_json["dl_size"].as_f64(),
        ) {
            ((dl_size / total_size) * 100.0).round()
        } else {
            0.0
        };

        json!({
            "size": size,
            "speed": speed,
            "eta": eta,
            "task": task,
            "progress": progress,
        })
    } else {
        let event = if let Some(matched_event) = Regex::new(r"(\[)(.*)(])").unwrap().find(line) {
            matched_event.as_str()
        } else if line.contains("Deleting") {
            "cleanup"
        } else if line.contains("HTTP Error 404") {
            "fragErr"
        } else if line.contains("Skipping fragment") {
            "fragSkip"
        } else {
            ""
        };

        let task = match event {
            "fragErr" => "fragErr",
            "fragSkip" => "fragSkip",
            "cleanup" => "cleanup",
            "[Merger]" => "merge",
            "[Metadata]" => "meta",
            "[FixupM3u8]" => "fix",
            _ => "prepare",
        };

        json!({
            "task": task
        })
    }
}

/// Processes an stderr line from a `yt-dlp` process and returns a user-friendly message.
pub fn process_yt_dlp_stderr(line: &str) -> Option<&str> {
    if line.contains("Requested format is not available") {
        Some(
            "Requested format is not available. \
            Please try checking available formats before downloading",
        )
    } else if line.contains("ended before the end-of-stream") {
        Some("An error occurred in the download stream. Retry the download")
    } else if line.contains("unable to create directory") {
        Some(
            "Invalid downloads location. \
            Please change it to a valid location from the configuration",
        )
    } else {
        None
    }
}
