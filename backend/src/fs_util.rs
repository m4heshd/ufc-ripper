// Libs
use anyhow::{Context, Result};

use crate::config_util::get_config;

/// Opens the downloads directory in the default file explorer.
pub fn open_downloads_dir() -> Result<()> {
    open::that_detached(get_config().dl_path)
        .context("An error occurred while trying to open the downloads directory")?;

    Ok(())
}
