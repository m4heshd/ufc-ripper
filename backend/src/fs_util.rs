// Libs
use std::path::PathBuf;

use anyhow::Context;
use bytes::Bytes;
use futures_util::{Stream, StreamExt};
use rust_embed::RustEmbed;
use tokio::{fs, io::AsyncWriteExt};

use crate::{config_util::get_config, rt_util::QuitUnwrap};

// Structs
/// Holds all the static files for UFC Ripper GUI that will be served using axum.
#[derive(RustEmbed, Clone)]
#[folder = "$CARGO_MANIFEST_DIR/dist/"]
pub struct WebAssets;

/// Reads the config.json file from the disk and returns the content as `String`.
pub async fn read_config_file_to_string(path: &PathBuf) -> String {
    fs::read_to_string(path).await.unwrap_or_quit(
        r#"Unable to read "config.json" file. Check if the file exists in "config" directory"#,
    )
}

/// Writes the current configuration to config.json file.
pub async fn write_config_to_file(path: &PathBuf) -> anyhow::Result<()> {
    let mut conf_file = fs::File::create(path).await?;

    conf_file
        .write_all(serde_json::to_string_pretty(get_config().as_ref())?.as_bytes())
        .await?;

    Ok(())
}

/// Creates a file on the disk using the given byte-stream.
pub async fn write_file_to_disk<S>(
    path: PathBuf,
    size: u64,
    #[allow(unused_variables)] is_executable: bool,
    mut stream: S,
    on_progress: impl Fn(f64),
) -> anyhow::Result<()>
where
    S: Stream<Item = Result<Bytes, reqwest::Error>> + Unpin,
{
    let dir_tree = path.parent().context("Invalid file path")?;

    fs::create_dir_all(dir_tree).await?;

    let mut file = fs::File::create(&path).await?;
    let mut dl_total = 0u64;

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;

        file.write_all(&chunk).await?;

        dl_total += chunk.len() as u64;

        on_progress(((dl_total as f64 / size as f64) * 100.0).round());
    }

    file.flush().await?;

    #[cfg(not(target_os = "windows"))]
    if is_executable {
        use std::os::unix::fs::PermissionsExt;
        file.set_permissions(std::fs::Permissions::from_mode(0o775))
            .await?;
    }

    Ok(())
}

/// Opens the downloads directory in the default file explorer.
pub fn open_downloads_dir() -> anyhow::Result<()> {
    open::that_detached(&get_config().dl_path)
        .context("An error occurred while trying to open the downloads directory")?;

    Ok(())
}
