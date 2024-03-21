// Libs
use crate::rt_util::QuitUnwrap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

// Structs
/// Holds all information of a VOD.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VOD {
    pub id: u64,
    pub title: String,
    pub desc: String,
    pub thumb: String,
    pub access: bool,
    #[serde(rename = "vodURL")]
    pub vod_url: String,
    #[serde(rename = "qID")]
    pub q_id: String,
    pub custom_format: String,
    pub hls: String,
    pub task: String,
    pub status: String,
    pub progress: f32,
    pub size: String,
    pub speed: String,
    pub eta: String,
    pub idx: u64,
}

// Statics
/// Holds the global downloads-queue.
static DOWNLOADS_QUEUE: Lazy<Arc<Mutex<HashMap<String, VOD>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Returns a copy of the downloads-queue
pub fn get_dlq() -> HashMap<String, VOD> {
    DOWNLOADS_QUEUE
        .lock()
        .unwrap_or_quit("Failed to exclusively access the downloads queue")
        .clone()
}
