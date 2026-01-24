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

pub async fn run_release(paths: &Paths, version: &Version, force: bool) -> Result<()> {
    if version.is_server_packages_release() {
        return Err(Error::ExpectedNonAlphaVersion(version.clone()));
    }
    run(paths, version, force).await
}

pub async fn run_alpha(paths: &Paths, version: &Version, force: bool) -> Result<()> {
    if !version.is_server_packages_release() {
        return Err(Error::ExpectedAlphaVersion(version.clone()));
    }
    run(paths, version, force).await
}

async fn run(paths: &Paths, version: &Version, force: bool) -> Result<()> {
    if paths.version_installed(version) {
        if force {
            print_info(format!("Removing existing installation of {}", version));
            fs::remove_dir_all(paths.version_dir(version))?;
        } else {
            return Err(Error::VersionAlreadyInstalled(version.clone()));
        }
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

    print_success(format!("RabbitMQ {} installed successfully", version));
    print_info(format!("Activate with: eval \"$(frm use {})\"", version));

    Ok(())
}
