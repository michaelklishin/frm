// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;

use bel7_cli::{print_info, print_success};

use crate::Result;
use crate::download::{Downloader, copy_default_config};
use crate::errors::Error;
use crate::paths::Paths;
use crate::timestamps::Timestamps;
use crate::version::Version;

pub async fn run_release(paths: &Paths, version: &Version) -> Result<()> {
    if version.is_server_packages_release() {
        return Err(Error::ExpectedNonAlphaVersion(version.clone()));
    }
    run(paths, version).await
}

pub async fn run_alpha(paths: &Paths, version: &Version) -> Result<()> {
    if !version.is_server_packages_release() {
        return Err(Error::ExpectedAlphaVersion(version.clone()));
    }
    run(paths, version).await
}

async fn run(paths: &Paths, version: &Version) -> Result<()> {
    if !paths.version_installed(version) {
        return Err(Error::VersionNotInstalled(version.clone()));
    }

    print_info(format!("Removing RabbitMQ {}", version));
    fs::remove_dir_all(paths.version_dir(version))?;

    let archive = paths.downloads_dir().join(version.archive_name());
    if archive.exists() {
        fs::remove_file(&archive)?;
    }

    paths.ensure_dirs()?;

    print_info(format!("Downloading RabbitMQ {}", version));
    let downloader = Downloader::new();
    downloader.download(version, paths).await?;

    print_info("Copying default configuration");
    copy_default_config(paths, version)?;

    print_info("Cleaning up downloaded archive");
    downloader.cleanup_archive(version, paths)?;

    let mut timestamps = Timestamps::load(paths)?;
    timestamps.record(version);
    timestamps.save(paths)?;

    print_success(format!("RabbitMQ {} reinstalled successfully", version));

    Ok(())
}
