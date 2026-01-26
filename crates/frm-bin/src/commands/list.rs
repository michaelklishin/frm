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
use crate::shell::Shell;
use crate::version::Version;

pub fn run_releases(paths: &Paths) -> Result<()> {
    let versions = paths.installed_versions()?;
    let releases: Vec<_> = versions
        .into_iter()
        .filter(|v| !v.is_distributed_via_server_packages_repository())
        .collect();

    if releases.is_empty() {
        print_warning("No stable RabbitMQ releases installed");
        print_info("Install a release with: frm releases install <version>");
        return Ok(());
    }

    print_versions(paths, &releases)
}

pub fn run_alphas(paths: &Paths) -> Result<()> {
    let versions = paths.installed_versions()?;
    let alphas: Vec<_> = versions
        .into_iter()
        .filter(|v| v.is_distributed_via_server_packages_repository())
        .collect();

    if alphas.is_empty() {
        print_warning("No alpha RabbitMQ releases installed");
        print_info("Install an alpha with: frm alphas install --latest");
        return Ok(());
    }

    print_versions(paths, &alphas)
}

fn print_versions(paths: &Paths, versions: &[Version]) -> Result<()> {
    let config = Config::load(paths)?;
    let default_version = config.default_version.as_ref();

    for version in versions {
        let marker = if Some(version) == default_version {
            "[*]"
        } else {
            "[ ]"
        };
        println!("{} {}", marker, version);
    }

    Ok(())
}

pub fn completions_releases(paths: &Paths, _shell: Option<Shell>) -> Result<()> {
    let versions = paths.installed_versions()?;
    let releases: Vec<_> = versions
        .into_iter()
        .filter(|v| !v.is_distributed_via_server_packages_repository())
        .collect();

    println!("latest");
    for version in releases {
        println!("{}", version);
    }

    Ok(())
}

pub fn completions_alphas(paths: &Paths, _shell: Option<Shell>) -> Result<()> {
    let versions = paths.installed_versions()?;
    let alphas: Vec<_> = versions
        .into_iter()
        .filter(|v| v.is_distributed_via_server_packages_repository())
        .collect();

    println!("latest");
    for version in alphas {
        println!("{}", version);
    }

    Ok(())
}
