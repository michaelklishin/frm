// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use bel7_cli::{print_info, print_warning};

use crate::Result;
use crate::config::Config;
use crate::paths::Paths;

pub fn run(paths: &Paths) -> Result<()> {
    let versions = paths.installed_versions()?;

    if versions.is_empty() {
        print_warning("No RabbitMQ versions installed");
        print_info("Install a version with: frm install <version>");
        return Ok(());
    }

    let config = Config::load(paths)?;
    let default_version = config.default_version.as_ref();

    for version in &versions {
        let marker = if Some(version) == default_version {
            "[*]"
        } else {
            "[ ]"
        };
        println!("{} {}", marker, version);
    }

    Ok(())
}
