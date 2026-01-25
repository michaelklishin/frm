// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::unix::process::CommandExt;
use std::process::Command;

use crate::Result;
use crate::errors::Error;
use crate::paths::Paths;
use crate::version::Version;

const RABBITMQ_SERVER: &str = "rabbitmq-server";

pub fn run(paths: &Paths, version: &Version) -> Result<()> {
    if !paths.version_installed(version) {
        return Err(Error::VersionNotInstalled(version.clone()));
    }

    let server_path = paths.version_sbin_dir(version).join(RABBITMQ_SERVER);
    if !server_path.exists() {
        return Err(Error::FileNotFound(server_path.display().to_string()));
    }

    let err = Command::new(&server_path).exec();

    Err(Error::CommandFailed(format!(
        "failed to execute {}: {}",
        server_path.display(),
        err
    )))
}
