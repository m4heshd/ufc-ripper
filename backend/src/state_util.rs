// Libs
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

use anyhow::Context;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::rt_util::QuitUnwrap;

// Types
type VodMap = HashMap<String, Vod>;

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
static DOWNLOADS_QUEUE: Lazy<Arc<Mutex<VodMap>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Locks and returns a `MutexGuard` for the downloads-queue.
pub fn get_dlq() -> MutexGuard<'static, VodMap> {
    DOWNLOADS_QUEUE
        .lock()
        .unwrap_or_quit("Failed to exclusively access the downloads-queue")
}

/// Adds a new VOD to the downloads-queue.
pub fn add_vod_to_queue(mut vod: Vod) -> Vod {
    let mut q = get_dlq();

    if q.contains_key(&vod.q_id) {
        q.get_mut(&vod.q_id).unwrap().status = "downloading".to_string();
    } else {
        // FIXME: This will be an issue when clearing the DLQ because the VODs added in the future
        //        could have the same `idx` since the DL queue's length could be reduced.
        vod.idx = (q.len() + 1) as u64;
        q.insert(vod.q_id.clone(), vod.clone());
    }

    vod
}

/// Updates the status of a VOD in the downloads-queue.
pub fn update_dlq_vod_status(q_id: &str, status: &str) -> anyhow::Result<()> {
    let mut q = get_dlq();
    let vod = q
        .get_mut(q_id)
        .context("VOD does not exist in the downloads-queue")?;

    vod.status = status.to_string();

    Ok(())
}

/// Removes finished or failed downloads from the downloads-queue.
pub fn clear_inactive_dlq_vods() {
    get_dlq().retain(|_, vod| vod.status == "downloading");
}
