// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;
use std::path::Path;

use bel7_cli::{print_info, print_success};

use crate::Result;
use crate::download::copy_default_config;
use crate::errors::Error;
use crate::paths::Paths;
use crate::tanzu::{extract_tarball, extract_version_from_tarball_name, verify_extracted_version};
use crate::timestamps::Timestamps;
use crate::version::Version;

pub fn run(
    paths: &Paths,
    tarball_path: &Path,
    expected_version: &Version,
    force: bool,
) -> Result<()> {
    if !tarball_path.exists() {
        return Err(Error::FileNotFound(tarball_path.display().to_string()));
    }

    if let Some(detected_version) = extract_version_from_tarball_name(tarball_path)
        && &detected_version != expected_version
    {
        return Err(Error::TanzuVersionMismatch {
            expected: expected_version.clone(),
            detected: detected_version,
        });
    }

    if paths.version_installed(expected_version) {
        if force {
            print_info(format!(
                "Removing existing installation of {}",
                expected_version
            ));
            fs::remove_dir_all(paths.version_dir(expected_version))?;
        } else {
            return Err(Error::VersionAlreadyInstalled(expected_version.clone()));
        }
    }

    paths.ensure_dirs()?;

    print_info(format!(
        "Extracting Tanzu RabbitMQ {} from {}",
        expected_version,
        tarball_path.display()
    ));
    extract_tarball(tarball_path, expected_version, paths)?;

    print_info("Verifying extracted content");
    verify_extracted_version(paths, expected_version)?;

    print_info("Copying default configuration");
    copy_default_config(paths, expected_version)?;

    let mut timestamps = Timestamps::load(paths)?;
    timestamps.record(expected_version);
    timestamps.save(paths)?;

    print_success(format!(
        "Tanzu RabbitMQ {} installed successfully",
        expected_version
    ));
    print_info(format!(
        "Activate with: eval \"$(frm use {})\"",
        expected_version
    ));

    Ok(())
}
