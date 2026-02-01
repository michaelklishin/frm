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
use crate::common::cli_tools::RABBITMQ_CLI_TOOLS;
use crate::errors::Error;
use crate::paths::Paths;
use crate::version::Version;

#[cfg(unix)]
pub fn run(paths: &Paths, version: &Version, tool: &str, args: &[String]) -> Result<()> {
    if !paths.version_installed(version) {
        return Err(Error::VersionNotInstalled(version.clone()));
    }

    if !RABBITMQ_CLI_TOOLS.contains(&tool) {
        return Err(Error::UnknownTool(format!(
            "'{}'. Valid tools: {}",
            tool,
            RABBITMQ_CLI_TOOLS.join(", ")
        )));
    }

    let tool_path = paths.version_sbin_dir(version).join(tool);
    if !tool_path.exists() {
        return Err(Error::FileNotFound(tool_path.display().to_string()));
    }

    let err = Command::new(&tool_path).args(args).exec();

    Err(Error::CommandFailed(format!(
        "failed to execute {}: {}",
        tool_path.display(),
        err
    )))
}

#[cfg(windows)]
pub fn run(paths: &Paths, version: &Version, tool: &str, args: &[String]) -> Result<()> {
    if !paths.version_installed(version) {
        return Err(Error::VersionNotInstalled(version.clone()));
    }

    if !RABBITMQ_CLI_TOOLS.contains(&tool) {
        return Err(Error::UnknownTool(format!(
            "'{}'. Valid tools: {}",
            tool,
            RABBITMQ_CLI_TOOLS.join(", ")
        )));
    }

    let tool_path = paths.version_sbin_dir(version).join(tool);
    if !tool_path.exists() {
        return Err(Error::FileNotFound(tool_path.display().to_string()));
    }

    let status = Command::new(&tool_path).args(args).status().map_err(|e| {
        Error::CommandFailed(format!("failed to execute {}: {}", tool_path.display(), e))
    })?;

    process::exit(status.code().unwrap_or(1));
}
