// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;

use crate::Result;
use crate::errors::Error;
use crate::paths::Paths;
use crate::version::Version;

pub const CONFIG_FILES: &[&str] = &[
    "rabbitmq.conf",
    "rabbitmq-env.conf",
    "advanced.config",
    "enabled_plugins",
];

pub fn run(paths: &Paths, version: &Version, file: &str) -> Result<()> {
    if !paths.version_installed(version) {
        return Err(Error::VersionNotInstalled(version.clone()));
    }

    if !CONFIG_FILES.contains(&file) {
        return Err(Error::UnknownConfigFile(format!(
            "'{}'. Valid files: {}",
            file,
            CONFIG_FILES.join(", ")
        )));
    }

    let file_path = paths.version_etc_dir(version).join(file);
    if !file_path.exists() {
        return Err(Error::FileNotFound(file_path.display().to_string()));
    }

    let content = fs::read_to_string(&file_path)?;
    print!("{}", content);

    Ok(())
}
