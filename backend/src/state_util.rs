// Libs
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::rt_util::QuitUnwrap;

// Structs
/// Holds all information of a VOD.
#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vod {
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
static DOWNLOADS_QUEUE: Lazy<Arc<Mutex<HashMap<String, Vod>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Returns a copy of the downloads-queue.
pub fn get_dlq() -> MutexGuard<'static, HashMap<String, Vod>> {
    DOWNLOADS_QUEUE
        .lock()
        .unwrap_or_quit("Failed to exclusively access the downloads-queue")
}

/// Adds a new VOD to the downloads-queue.
pub fn add_vod_to_queue(mut vod: Vod) -> Vod {
    let mut q = get_dlq();

    // FIXME: This will be an issue when clearing the DLQ because the VODs added in the future
    //        could have the same `idx` since the DL queue's length could be reduced.
    if q.contains_key(&vod.q_id) {
        q.get_mut(&vod.q_id).unwrap().status = "downloading".to_string();
    } else {
        vod.idx = (q.len() + 1) as u64;
        q.insert(vod.q_id.clone(), vod.clone());
    }

    vod
}
