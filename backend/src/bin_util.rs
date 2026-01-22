// Libs
use std::{
    collections::HashMap,
    ffi::OsStr,
    path::PathBuf,
    process::Stdio,
    sync::{Arc, Mutex, MutexGuard},
};

use anyhow::{anyhow, Context};
use colored::Colorize;
use once_cell::sync::Lazy;
use serde_json::json;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, Command},
    task::JoinHandle,
    time::Instant,
};

use ufcr_libs::{log_err, log_warn};

use crate::{
    app_util::get_app_root_dir,
    config_util::{get_config, inc_file_number, is_debug, UFCRConfig},
    net_util::JSON,
    rt_util::QuitUnwrap,
    state_util::{add_vod_to_queue, update_dlq_vod_status, Vod},
    txt_util::{process_yt_dlp_stderr, process_yt_dlp_stdout},
};

// Types
type TaskMap = HashMap<String, JoinHandle<()>>;

// Structs
/// Holds all metadata for each helper media tool.
pub struct MediaTools {
    ffmpeg: MediaToolMeta,
    ffprobe: MediaToolMeta,
    yt_dlp: MediaToolMeta,
}

impl MediaTools {
    /// Fetches a media-tool by the given JSON-like name.
    pub fn get_by_name(&self, name: &str) -> Option<&MediaToolMeta> {
        match name {
            "ffmpeg" => Some(&self.ffmpeg),
            "ffprobe" => Some(&self.ffprobe),
            "ytDlp" => Some(&self.yt_dlp),
            _ => None,
        }
    }
}

pub struct MediaToolMeta {
    filename: &'static str,
}

impl MediaToolMeta {
    /// Returns the absolute path to the media tool.
    pub fn get_path(&self) -> PathBuf {
        get_app_root_dir().join("bin").join(self.filename)
    }
}

// Statics
pub static BINS: MediaTools = MediaTools {
    ffmpeg: MediaToolMeta {
        filename: if cfg!(windows) {
            "ffmpeg.exe"
        } else {
            "ffmpeg"
        },
    },
    ffprobe: MediaToolMeta {
        filename: if cfg!(windows) {
            "ffprobe.exe"
        } else {
            "ffprobe"
        },
    },
    yt_dlp: MediaToolMeta {
        filename: if cfg!(windows) {
            "yt-dlp.exe"
        } else {
            "yt-dlp"
        },
    },
};
/// Holds the download progress template for yt-dlp.
static DL_PROGRESS_TEMPLATE: Lazy<String> = Lazy::new(generate_yt_dlp_progress_template);
/// Holds the global yt-dlp download task handles.
static DL_TASKS: Lazy<Arc<Mutex<TaskMap>>> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Validates if the media tools exist and returns the validation for each binary as JSON.
pub fn validate_bins() -> JSON {
    json!({
        "ffmpeg": BINS.ffmpeg.get_path().exists(),
        "ffprobe": BINS.ffprobe.get_path().exists(),
        "ytDlp": BINS.yt_dlp.get_path().exists()
    })
}

#[allow(clippy::too_many_lines)]
/// Starts a download process using `yt-dlp` and updates the downloads-queue with progress.
pub async fn start_download<P, C, F>(
    vod: &Vod,
    is_restart: bool,
    on_progress: P,
    on_completion: C,
    on_fail: F,
) -> anyhow::Result<Vod>
where
    P: Fn(&str, JSON) + Send + 'static,
    C: FnOnce(&str) + Send + 'static,
    F: Fn(&str, anyhow::Error) + Send + 'static,
{
    let config = get_config();
    let (final_title, dl_config) = generate_vod_download_config(config.as_ref(), vod, is_restart)?;

    println!(
        "\n{}",
        format!(
            "{} {final_title}",
            if is_restart {
                "Restarting"
            } else {
                "Downloading"
            }
        )
        .bright_yellow()
        .bold()
        .underline()
        .on_black()
    );
    println!("{}\n", vod.vod_url.clone().dimmed());

    if is_debug() {
        println!("[yt-dlp-args] {}", dl_config.join(" "));
    }

    if !is_restart {
        inc_file_number().await;
    };

    let download_process = {
        // Need these clones because it's not possible to clone values into a closure
        // Ref: https://github.com/rust-lang/rfcs/issues/2407
        let q_id = vod.q_id.clone();

        async move {
            let mut yt_dlp = start_yt_dlp_process(dl_config).context(
                "Download failed: \
                An error occurred while trying to launch the download process. \
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
                    if let Some(error) = process_yt_dlp_stderr(&line) {
                        return Err(anyhow!(error.to_string()));
                    }

                    return Err(anyhow!(line).context(
                        "Download process failed with an error. \
                        Check the browser console for more information",
                    ));
                }

                Ok::<(), anyhow::Error>(())
            };

            let stdout_task = async move {
                let mut last = Instant::now();

                while let Some(line) = yt_dlp_stdout
                    .next_line()
                    .await
                    .unwrap_or(Some(String::new()))
                {
                    if last.elapsed().as_millis() > 500 {
                        last = Instant::now();
                        on_progress(&q_id, process_yt_dlp_stdout(&line));
                    }
                }

                Ok::<(), anyhow::Error>(())
            };

            tokio::try_join!(stderr_task, stdout_task)
        }
    };

    let dl_process = tokio::spawn({
        let q_id = vod.q_id.clone();
        let final_title = final_title.clone();

        async move {
            let err_msg = "Unable to update the status of the VOD";

            if let Err(error) = download_process.await {
                if let Err(inner_error) = update_dlq_vod_status(&q_id, "failed") {
                    log_err!("{err_msg}:\n{inner_error}\n");
                }

                println!(
                    "\n{}\n",
                    format!("Download failed - {final_title}")
                        .bright_red()
                        .bold()
                        .on_black()
                );

                on_fail(&q_id, error);
            } else {
                // TODO: Might need to check the exit code of yt-dlp here
                if let Err(error) = update_dlq_vod_status(&q_id, "completed") {
                    log_err!("{err_msg}:\n{error}\n");
                }

                println!(
                    "\n{}\n",
                    format!("Completed download - {final_title}")
                        .bright_green()
                        .bold()
                        .on_black()
                );

                on_completion(&q_id);
            }

            if let Err(error) = remove_dl_task(&q_id) {
                log_err!("Failed to remove the download task:\n{error}\n");
            }
        }
    });

    let queued_vod = add_vod_to_queue(Vod {
        title: final_title,
        task: "prepare".to_string(),
        status: "downloading".to_string(),
        progress: 0.0,
        size: "N/A".to_string(),
        speed: "N/A".to_string(),
        eta: "N/A".to_string(),
        ..vod.clone()
    });

    add_dl_task(&vod.q_id, dl_process);

    Ok(queued_vod)
}

/// Starts a format query process using `yt-dlp` and returns the available formats as JSON.
pub async fn get_vod_formats(hls: &str) -> anyhow::Result<JSON> {
    let yt_dlp_args = vec![
        "--no-update",
        "--print",
        "%(formats.:.{format_id,resolution,fps,tbr,vcodec,acodec})j",
        hls,
    ];

    let mut yt_dlp = start_yt_dlp_process(yt_dlp_args).context(
        "Formats query failed: \
        An error occurred while trying to launch the formats query process. \
        Make sure that all of the media-tools are available in the \"bin\" directory",
    )?;

    let mut yt_dlp_stderr = BufReader::new(
        yt_dlp
            .stderr
            .take()
            .context("Failed to capture the error output from formats query process")?,
    )
    .lines();

    let mut yt_dlp_stdout = BufReader::new(
        yt_dlp
            .stdout
            .take()
            .context("Failed to capture the output from formats query process")?,
    )
    .lines();

    let stderr_task = async {
        if let Some(line) = yt_dlp_stderr.next_line().await? {
            if let Some(error) = process_yt_dlp_stderr(&line) {
                return Err(anyhow!(error.to_string()));
            }

            return Err(anyhow!(line).context(
                "Formats query request failed with an error. \
                Check the browser console for more information",
            ));
        }

        Ok::<(), anyhow::Error>(())
    };

    let stdout_task = async move {
        let mut formats = String::new();

        while let Some(line) = yt_dlp_stdout.next_line().await? {
            formats = line;
        }

        Ok::<String, anyhow::Error>(formats)
    };

    match tokio::try_join!(stderr_task, stdout_task) {
        Ok(((), formats)) => {
            let err_msg = "Format output is invalid. Please try again or \
                check the video on Fight Pass to verify that it actually streams";

            if let Ok(formats_json) = serde_json::from_str::<JSON>(&formats) {
                if formats_json.is_array() {
                    Ok(formats_json)
                } else {
                    Err(anyhow!(err_msg))
                }
            } else {
                Err(anyhow!(err_msg))
            }
        }
        Err(error) => Err(error),
    }
}

/// Starts a new `yt-dlp` process and returns the `Child` handle to it.
fn start_yt_dlp_process<I, S>(args: I) -> std::io::Result<Child>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new(BINS.yt_dlp.get_path())
        .args(args)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .stdin(Stdio::null())
        .spawn()
}

/// Cancels an active download.
pub fn cancel_download(vod: &Vod) -> anyhow::Result<()> {
    remove_dl_task(&vod.q_id)?.abort();
    update_dlq_vod_status(&vod.q_id, "cancelled")?;

    println!(
        "\n{}\n",
        format!("Download cancelled by user - {}", vod.title)
            .bright_red()
            .bold()
            .on_black()
    );

    Ok(())
}

/// Generates all CLI arguments for a `yt-dlp` download according to the configuration and VOD
/// settings, and returns them as a `Vec<String>`.
pub fn generate_vod_download_config(
    config: &UFCRConfig,
    vod: &Vod,
    is_restart: bool,
) -> anyhow::Result<(String, Vec<String>)> {
    let UFCRConfig {
        vid_quality,
        aud_quality,
        resolution,
        merge_ext,
        dl_path,
        use_temp_path,
        temp_path,
        number_files,
        cur_number,
        throttle,
        dl_rate,
        multi_frag,
        concur_frags,
        cus_format,
        format_id,
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
    let dl_path_buf = PathBuf::from(dl_path);
    let home_path = format!(
        "home:{}",
        dl_path_buf.to_str().context(
            "Failed to build the given downloads path. Try changing the downloads directory",
        )?
    );
    let temp_path_buf = PathBuf::from(temp_path);
    let temp_path = format!(
        "temp:{}",
        temp_path_buf.to_str().context(
            "Failed to build the given temporary path. Try changing the temporary directory",
        )?
    );
    let output_template = format!("{final_title}.%(ext)s");
    let bin_path_buf = get_app_root_dir().join("bin");
    let concur_frags_string = concur_frags.to_string();

    let mut arg_setup = vec![
        "--no-update",
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
        "--paths",
        &home_path,
        "--output",
        &output_template,
        "--progress-template",
        &DL_PROGRESS_TEMPLATE,
        "--ffmpeg-location",
        bin_path_buf.to_str().context(
            "Failed to build the path to media-tools. \
            Try moving UFC Ripper to a different location",
        )?,
    ];

    if *use_temp_path {
        if temp_path.eq("temp:") {
            log_warn!(r#"You have "Use temporary path" enabled for downloads but the path is empty. Consider setting a proper temporary path"#);
        } else {
            arg_setup.extend(["--paths", &temp_path]);
        }
    }
    if *throttle {
        arg_setup.extend(["--limit-rate", dl_rate]);
    }
    if *multi_frag {
        arg_setup.extend(["--concurrent-fragments", &concur_frags_string]);
    }

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
    .to_string();

    progress_template.retain(|c| !c.is_whitespace());
    // This newline at the end is essential because else, the `next_line()` won't be triggered since
    // the stdout buffer won't flush till it reaches a newline.
    progress_template.push('\n');

    progress_template
}

/// Locks and returns a `MutexGuard` for the download tasks.
fn get_dl_tasks() -> MutexGuard<'static, TaskMap> {
    DL_TASKS
        .lock()
        .unwrap_or_quit("Failed to exclusively access the download tasks")
}

/// Adds a new task handle to the download tasks.
fn add_dl_task(q_id: &str, task: JoinHandle<()>) {
    get_dl_tasks().insert(q_id.to_string(), task);
}

/// Removes and returns a task handle from the download tasks.
fn remove_dl_task(q_id: &str) -> anyhow::Result<JoinHandle<()>> {
    get_dl_tasks()
        .remove(q_id)
        .context("The download task is not actively available anymore")
}
