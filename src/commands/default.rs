// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;

use bel7_cli::print_success;

use crate::Result;
use crate::config::Config;
use crate::errors::Error;
use crate::paths::Paths;
use crate::version::Version;

pub fn run(paths: &Paths, version: &Version) -> Result<()> {
    if !paths.version_installed(version) {
        return Err(Error::VersionNotInstalled(version.clone()));
    }

    let mut config = Config::load(paths)?;
    config.set_default(version.clone());
    config.save(paths)?;

    fs::write(paths.default_file(), version.to_string())?;

    print_success(format!("Default version set to {}", version));

    Ok(())
}
