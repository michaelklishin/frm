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
use crate::config::Config;
use crate::errors::Error;
use crate::paths::Paths;
use crate::timestamps::Timestamps;
use crate::version::Version;

pub fn run_release(paths: &Paths, version: &Version) -> Result<()> {
    if version.is_server_packages_release() {
        return Err(Error::ExpectedNonAlphaVersion(version.clone()));
    }
    run(paths, version)
}

pub fn run_alpha(paths: &Paths, version: &Version) -> Result<()> {
    if !version.is_server_packages_release() {
        return Err(Error::ExpectedAlphaVersion(version.clone()));
    }
    run(paths, version)
}

fn run(paths: &Paths, version: &Version) -> Result<()> {
    if !paths.version_installed(version) {
        return Err(Error::VersionNotInstalled(version.clone()));
    }

    let version_dir = paths.version_dir(version);
    fs::remove_dir_all(&version_dir)?;

    let mut config = Config::load(paths)?;
    if config.default_version.as_ref() == Some(version) {
        config.clear_default();
        config.save(paths)?;

        let default_file = paths.default_file();
        if default_file.exists() {
            fs::remove_file(default_file)?;
        }

        print_info("Cleared default version (uninstalled version was the default)");
    }

    let archive = paths.downloads_dir().join(version.archive_name());
    if archive.exists() {
        fs::remove_file(archive)?;
    }

    let mut timestamps = Timestamps::load(paths)?;
    timestamps.remove(version);
    timestamps.save(paths)?;

    print_success(format!("RabbitMQ {} uninstalled", version));

    Ok(())
}
