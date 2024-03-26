// Libs
use std::path::PathBuf;

use serde_json::json;

use crate::{app_util::get_app_root_dir, net_util::JSON};

// Structs
/// Holds all metadata for each helper media tool.
pub struct MediaTools {
    atomic_parsley: MediaToolsMeta,
    ffmpeg: MediaToolsMeta,
    ffprobe: MediaToolsMeta,
    yt_dlp: MediaToolsMeta,
}

impl MediaTools {
    /// Fetches a media-tool by the given JSON-like name.
    pub fn get_by_name(&self, name: &str) -> Option<&MediaToolsMeta> {
        match name {
            "atomicParsley" => Some(&self.atomic_parsley),
            "ffmpeg" => Some(&self.ffmpeg),
            "ffprobe" => Some(&self.ffprobe),
            "ytDlp" => Some(&self.yt_dlp),
            _ => None,
        }
    }
}

pub struct MediaToolsMeta {
    filename: &'static str,
}

impl MediaToolsMeta {
    /// Returns the absolute path to the media tool.
    pub fn get_path(&self) -> PathBuf {
        get_app_root_dir().join("bin").join(self.filename)
    }
}

// Statics
pub static BINS: MediaTools = MediaTools {
    atomic_parsley: MediaToolsMeta {
        filename: if cfg!(windows) {
            "AtomicParsley.exe"
        } else {
            "AtomicParsley"
        },
    },
    ffmpeg: MediaToolsMeta {
        filename: if cfg!(windows) {
            "ffmpeg.exe"
        } else {
            "ffmpeg"
        },
    },
    ffprobe: MediaToolsMeta {
        filename: if cfg!(windows) {
            "ffprobe.exe"
        } else {
            "ffprobe"
        },
    },
    yt_dlp: MediaToolsMeta {
        filename: if cfg!(windows) {
            "yt-dlp.exe"
        } else {
            "yt-dlp"
        },
    },
};

/// Validates if the media tools exist and returns the validation for each binary as JSON.
pub fn validate_bins() -> JSON {
    json!({
        "atomicParsley": BINS.atomic_parsley.get_path().exists(),
        "ffmpeg": BINS.ffmpeg.get_path().exists(),
        "ffprobe": BINS.ffprobe.get_path().exists(),
        "ytDlp": BINS.yt_dlp.get_path().exists()
    })
}
