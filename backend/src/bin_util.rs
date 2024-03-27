// Libs
use std::path::PathBuf;

use anyhow::{Context, Result};
use serde_json::json;

use crate::{app_util::get_app_root_dir, config_util::UFCRConfig, net_util::JSON, state_util::Vod};

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

/// Generates all CLI arguments for a `yt-dlp` download according to the configuration and VOD
/// settings, and returns them as a `Vec<String>`.
pub fn generate_vod_download_config<'a>(
    config: &'a UFCRConfig,
    vod: &'a Vod,
    is_restart: bool,
) -> Result<Vec<String>> {
    let UFCRConfig {
        vid_quality,
        aud_quality,
        resolution,
        merge_ext,
        dl_path,
        number_files,
        cur_number,
        throttle,
        dl_rate,
        multi_frag,
        concur_frags,
        cus_format,
        format_id,
        metadata,
        dl_args,
        ..
    } = config;

    let Vod {
        custom_format,
        title,
        hls,
        ..
    } = vod;

    let numbered_title = format!("{cur_number}. {title}");

    let final_title = if is_restart {
        title
    } else if *number_files {
        numbered_title.as_str()
    } else {
        title
    };

    let default_format = format!(
        "{vid_quality}[height={resolution}]+{aud_quality}/{vid_quality}*[height={resolution}]"
    );
    let dl_path_buf = PathBuf::from(dl_path).join(format!("{final_title}.%(ext)s"));
    let progress_template = generate_yt_dlp_progress_template();
    let bin_path_buf = get_app_root_dir().join("bin");
    let concur_frags_string = concur_frags.to_string();

    let mut arg_setup = vec![
        "--format",
        if custom_format.is_empty() {
            if *cus_format {
                format_id
            } else {
                &default_format
            }
        } else {
            custom_format
        },
        "--merge-output-format",
        merge_ext,
        "--output",
        dl_path_buf.to_str().context(
            "Failed to build the given downloads path. Please change the path and try again",
        )?,
        "--progress-template",
        &progress_template,
        "--ffmpeg-location",
        bin_path_buf
            .to_str()
            .context("Failed to build the path to media-tools")?,
    ];

    if *throttle {
        arg_setup.extend(["--limit-rate", dl_rate]);
    };
    if *multi_frag {
        arg_setup.extend(["--concurrent-fragments", &concur_frags_string]);
    };
    if *metadata {
        arg_setup.push("--add-metadata");
    };

    let mut arg_setup_final = arg_setup
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>();

    arg_setup_final.extend(dl_args.clone());
    arg_setup_final.push(hls.to_string());

    Ok(arg_setup_final)
}

/// Generates the JSON download progress template for `yt-dlp` and returns it as a `String`
fn generate_yt_dlp_progress_template() -> String {
    let mut progress_template = r#"
    {
        "status": "%(progress.status)s",
        "total_size": %(progress.total_bytes_estimate)d,
        "dl_size": %(progress.downloaded_bytes)d,
        "size": "%(progress._total_bytes_estimate_str)s",
        "speed": "%(progress._speed_str)s",
        "eta": "%(progress._eta_str)s",
        "vcodec": "%(info.vcodec)s"
    }
    "#
    .to_string()
    .replace(['\n', '\r'], "");

    progress_template.push('\n');

    progress_template
}
