// Libs
use std::{fs::File, path::PathBuf};

use clap::Parser;
use fs_extra::{
    copy_items,
    dir::{CopyOptions, create_all},
    error::Error,
};
use zip::{CompressionMethod, write::FileOptions, ZipWriter};
use zip_extensions::write::ZipWriterExtensions;

use ufcr_libs::{log_err, log_info, log_success};

// Structs
#[derive(Parser)]
#[command(long_about = None)]
struct CLIArgs {
    /// Target platform.
    #[arg(short, long, value_name = "PLATFORM (win32, linux)")]
    platform: String,
    /// Optional version tag for the release artifacts.
    #[arg(short, long, value_name = "RELEASE TAG")]
    tag: Option<String>,
}

fn main() -> Result<(), Error> {
    log_info!("\nPackaging UFC Ripper for release..\n");

    let cli_args = CLIArgs::parse();
    let platform = cli_args.platform.as_str();
    let bin_path = if platform == "win32" {
        "target/release/ufc-ripper.exe"
    } else {
        "target/release/ufc-ripper"
    };
    let tag = match cli_args.tag {
        None => String::new(),
        Some(cli_tag) => format!("-{cli_tag}"),
    };

    let target_dir = format!("package/{platform}");
    let artifacts_dir = "package/artifacts";
    let archive = format!("{artifacts_dir}/ufc-ripper{tag}-{platform}-x64.zip");
    let sources = vec![bin_path, "config"];

    log_info!("Creating directory structure..\n");
    create_all(&target_dir, true)?;
    create_all(artifacts_dir, true)?;
    create_all(format!("{target_dir}/bin"), true)?;

    log_info!("Copying files..\n");
    copy_items(&sources, &target_dir, &CopyOptions::new())?;

    log_info!("Creating archive..\n");
    let file = File::create(archive)?;
    let mut zip = ZipWriter::new(file);

    zip.create_from_directory_with_options(
        &PathBuf::from(&target_dir),
        FileOptions::default().compression_method(CompressionMethod::Deflated),
    )
    .unwrap_or_else(|e| {
        log_err!("Archiving process failed.\nError: {}\n", e.to_string());
        panic!();
    });

    log_success!("Packaging process completed!\n");

    Ok(())
}
