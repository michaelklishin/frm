// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(unix)]
use std::os::unix::process::CommandExt;
#[cfg(windows)]
use std::process;
use std::process::Command;

use crate::Result;
use crate::common::cli_tools::RABBITMQ_SERVER;
use crate::common::env_vars::RABBITMQ_CONFIG_FILES;
use crate::errors::Error;
use crate::paths::Paths;
use crate::version::Version;

#[cfg(unix)]
pub fn run(paths: &Paths, version: &Version) -> Result<()> {
    if !paths.version_installed(version) {
        return Err(Error::VersionNotInstalled(version.clone()));
    }

    let server_path = paths.version_sbin_dir(version).join(RABBITMQ_SERVER);
    if !server_path.exists() {
        return Err(Error::FileNotFound(server_path.display().to_string()));
    }

    let err = Command::new(&server_path)
        .env(RABBITMQ_CONFIG_FILES, paths.version_confd_dir(version))
        .exec();

    Err(Error::CommandFailed(format!(
        "failed to execute {}: {}",
        server_path.display(),
        err
    )))
}

#[cfg(windows)]
pub fn run(paths: &Paths, version: &Version) -> Result<()> {
    if !paths.version_installed(version) {
        return Err(Error::VersionNotInstalled(version.clone()));
    }

    let server_path = paths.version_sbin_dir(version).join(RABBITMQ_SERVER);
    if !server_path.exists() {
        return Err(Error::FileNotFound(server_path.display().to_string()));
    }

    let status = Command::new(&server_path)
        .env(RABBITMQ_CONFIG_FILES, paths.version_confd_dir(version))
        .status()
        .map_err(|e| {
            Error::CommandFailed(format!(
                "failed to execute {}: {}",
                server_path.display(),
                e
            ))
        })?;

    process::exit(status.code().unwrap_or(1));
}
