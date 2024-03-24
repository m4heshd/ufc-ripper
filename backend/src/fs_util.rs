// Libs
use std::path::PathBuf;

use anyhow::{Context, Result};
use tokio::{fs, io::AsyncWriteExt};

use crate::{config_util::get_config, rt_util::QuitUnwrap};

/// Reads the config.json file from the disk and returns the content as `String`.
pub async fn read_config_file_to_string(path: &PathBuf) -> String {
    fs::read_to_string(path).await.unwrap_or_quit(
        r#"Unable to read "config.json" file. Check if the file exists in "config" directory"#,
    )
}

/// Writes the current configuration to config.json file.
pub async fn write_config_to_file(path: &PathBuf) -> Result<()> {
    let mut conf_file = fs::File::create(path).await?;

    conf_file
        .write_all(serde_json::to_string_pretty(&get_config())?.as_bytes())
        .await?;

    Ok(())
}

/// Opens the downloads directory in the default file explorer.
pub fn open_downloads_dir() -> Result<()> {
    open::that_detached(get_config().dl_path)
        .context("An error occurred while trying to open the downloads directory")?;

    Ok(())
}
