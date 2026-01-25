// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::Result;
use crate::errors::Error;
use crate::paths::Paths;
use crate::version::Version;

pub fn run_release(paths: &Paths, version: &Version) -> Result<()> {
    if version.is_distributed_via_server_packages_repository() {
        return Err(Error::ExpectedNonAlphaVersion(version.clone()));
    }
    run(paths, version)
}

pub fn run_alpha(paths: &Paths, version: &Version) -> Result<()> {
    if !version.is_distributed_via_server_packages_repository() {
        return Err(Error::ExpectedAlphaVersion(version.clone()));
    }
    run(paths, version)
}

fn run(paths: &Paths, version: &Version) -> Result<()> {
    if !paths.version_installed(version) {
        return Err(Error::VersionNotInstalled(version.clone()));
    }

    println!("{}", paths.version_dir(version).display());
    Ok(())
}
