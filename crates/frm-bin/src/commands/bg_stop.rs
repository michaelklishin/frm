// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::process::Command;

use bel7_cli::print_success;

use crate::Result;
use crate::common::cli_tools::RABBITMQCTL;
use crate::common::env_vars::RABBITMQ_HOME;
use crate::errors::Error;
use crate::paths::Paths;
use crate::version::Version;

pub fn run(paths: &Paths, version: &Version) -> Result<()> {
    if !paths.version_installed(version) {
        return Err(Error::VersionNotInstalled(version.clone()));
    }

    let ctl_path = paths.version_sbin_dir(version).join(RABBITMQCTL);
    if !ctl_path.exists() {
        return Err(Error::FileNotFound(ctl_path.display().to_string()));
    }

    let status = Command::new(&ctl_path)
        .arg("shutdown")
        .env(RABBITMQ_HOME, paths.version_dir(version))
        .status()
        .map_err(|e| {
            Error::CommandFailed(format!("failed to execute {}: {}", ctl_path.display(), e))
        })?;

    if !status.success() {
        return Err(Error::CommandFailed(format!(
            "rabbitmqctl shutdown exited with code {}",
            status.code().unwrap_or(-1)
        )));
    }

    print_success(format!("RabbitMQ {} stopped", version));

    Ok(())
}
