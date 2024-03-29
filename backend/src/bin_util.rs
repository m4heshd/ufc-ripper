// Libs
use std::{path::PathBuf, process::Stdio};

use anyhow::{anyhow, Context, Result};
use serde_json::json;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    time::Instant,
};

use crate::{
    app_util::get_app_root_dir,
    config_util::{get_config, inc_file_number, UFCRConfig},
    net_util::JSON,
    state_util::Vod,
    txt_util::process_yt_dlp_stdout,
};

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

/// Starts a download process using `yt-dlp` and updates the downloads-queue with progress.
pub async fn start_download<P, C, F>(
    vod: &Vod,
    is_restart: bool,
    on_progress: P,
    on_completion: C,
    on_fail: F,
) -> Result<Vod>
where
    P: Fn(&str, JSON) + Send + 'static,
    C: FnOnce(&str) + Send + 'static,
    F: Fn(&str, anyhow::Error) + Send + 'static,
{
    let config = get_config();
    let (final_title, dl_config) = generate_vod_download_config(config.as_ref(), vod, is_restart)?;

    if !is_restart {
        inc_file_number().await;
    };

    let download_process = {
        // Need these clones because it's not possible to clone values into a closure
        // Ref: https://github.com/rust-lang/rfcs/issues/2407
        let q_id = vod.q_id.clone();

        async move {
            let mut yt_dlp = Command::new(BINS.yt_dlp.get_path())
                .args(dl_config)
                .stderr(Stdio::piped())
                .stdout(Stdio::piped())
                .stdin(Stdio::null())
                .spawn()
                .context(
                    "Download failed: An error occurred while trying to launch the download process. \
            Make sure that all of the media-tools are available in the \"bin\" directory",
                )?;

            let mut yt_dlp_stderr = BufReader::new(
                yt_dlp
                    .stderr
                    .take()
                    .context("Failed to capture the error output from download process")?,
            )
            .lines();

            let mut yt_dlp_stdout = BufReader::new(
                yt_dlp
                    .stdout
                    .take()
                    .context("Failed to capture the output from download process")?,
            )
            .lines();

            let stderr_task = async {
                if let Some(line) = yt_dlp_stderr.next_line().await? {
                    return Err(anyhow!(line));
                }

                Ok::<(), anyhow::Error>(())
            };

            let stdout_task = async move {
                let mut last = Instant::now();

                while let Some(line) = yt_dlp_stdout.next_line().await? {
                    if last.elapsed().as_millis() > 500 {
                        last = Instant::now();
                        on_progress(&q_id, process_yt_dlp_stdout(&line));
                    }
                }

                Ok::<(), anyhow::Error>(())
            };

            tokio::try_join!(stderr_task, stdout_task).context(
                "Download process failed with an error. Check the browser console for more information",
            )
        }
    };

    tokio::spawn({
        let q_id = vod.q_id.clone();

        async move {
            if let Err(error) = download_process.await {
                on_fail(&q_id, error);
            } else {
                // TODO: Might need to check the exit code of yt-dlp here
                on_completion(&q_id);
            }

            println!("Download process completed");
        }
    });

    // TODO: Update the backend's downloads queue with the VOD's idx here

    Ok(Vod {
        title: final_title,
        task: "prepare".to_string(),
        status: "downloading".to_string(),
        progress: 0.0,
        size: "N/A".to_string(),
        speed: "N/A".to_string(),
        eta: "N/A".to_string(),
        ..vod.clone()
    })
}

/// Generates all CLI arguments for a `yt-dlp` download according to the configuration and VOD
/// settings, and returns them as a `Vec<String>`.
pub fn generate_vod_download_config(
    config: &UFCRConfig,
    vod: &Vod,
    is_restart: bool,
) -> Result<(String, Vec<String>)> {
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

    let final_title = if is_restart || !*number_files {
        title.to_string()
    } else {
        format!("{cur_number}. {title}")
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
            "Failed to build the given downloads path. Please change the path and try again. \
            Try changing the downloads directory",
        )?,
        "--progress-template",
        &progress_template,
        "--ffmpeg-location",
        bin_path_buf.to_str().context(
            "Failed to build the path to media-tools. \
            Try moving UFC Ripper to a different location",
        )?,
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

    Ok((final_title, arg_setup_final))
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
    .replace(['\n', '\r'], "");

    progress_template.push('\n');

    progress_template
}
