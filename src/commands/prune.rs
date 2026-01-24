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
use crate::paths::Paths;
use crate::timestamps::Timestamps;

pub fn run(paths: &Paths) -> Result<()> {
    let versions = paths.installed_versions()?;
    let alphas: Vec<_> = versions
        .into_iter()
        .filter(|v| v.is_server_packages_release())
        .collect();

    if alphas.is_empty() {
        print_info("No alpha versions installed");
        return Ok(());
    }

    let mut config = Config::load(paths)?;
    let mut timestamps = Timestamps::load(paths)?;
    let mut cleared_default = false;

    for version in &alphas {
        print_info(format!("Removing RabbitMQ {}", version));

        let version_dir = paths.version_dir(version);
        fs::remove_dir_all(&version_dir)?;

        if config.default_version.as_ref() == Some(version) {
            config.clear_default();
            cleared_default = true;
        }

        let archive = paths.downloads_dir().join(version.archive_name());
        if archive.exists() {
            fs::remove_file(archive)?;
        }

        timestamps.remove(version);
    }

    if cleared_default {
        config.save(paths)?;
        let default_file = paths.default_file();
        if default_file.exists() {
            fs::remove_file(default_file)?;
        }
        print_info("Cleared default version (an alpha was the default)");
    }

    timestamps.save(paths)?;

    print_success(format!("Removed {} alpha version(s)", alphas.len()));

    Ok(())
}
