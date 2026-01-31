// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use bel7_cli::print_info;

use crate::Result;
use crate::errors::Error;
use crate::paths::Paths;
use crate::version::Version;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EtcFile {
    RabbitmqConf,
    AdvancedConfig,
    RabbitmqConfig,
    EnabledPlugins,
}

impl EtcFile {
    pub const ALL: &[EtcFile] = &[
        EtcFile::RabbitmqConf,
        EtcFile::AdvancedConfig,
        EtcFile::RabbitmqConfig,
        EtcFile::EnabledPlugins,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            EtcFile::RabbitmqConf => "rabbitmq.conf",
            EtcFile::AdvancedConfig => "advanced.config",
            EtcFile::RabbitmqConfig => "rabbitmq.config",
            EtcFile::EnabledPlugins => "enabled_plugins",
        }
    }

    pub fn all_names() -> Vec<&'static str> {
        Self::ALL.iter().map(|f| f.as_str()).collect()
    }
}

impl fmt::Display for EtcFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for EtcFile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "rabbitmq.conf" => Ok(EtcFile::RabbitmqConf),
            "advanced.config" => Ok(EtcFile::AdvancedConfig),
            "rabbitmq.config" => Ok(EtcFile::RabbitmqConfig),
            "enabled_plugins" => Ok(EtcFile::EnabledPlugins),
            _ => Err(Error::UnknownConfigFile(format!(
                "'{}'. Valid files: {}",
                s,
                EtcFile::all_names().join(", ")
            ))),
        }
    }
}

pub fn run_release(
    paths: &Paths,
    version: &Version,
    local_path: &Path,
    etc_file: EtcFile,
) -> Result<()> {
    if version.is_distributed_via_server_packages_repository() {
        return Err(Error::ExpectedNonAlphaVersion(version.clone()));
    }
    run(paths, version, local_path, etc_file)
}

pub fn run_alpha(
    paths: &Paths,
    version: &Version,
    local_path: &Path,
    etc_file: EtcFile,
) -> Result<()> {
    if !version.is_distributed_via_server_packages_repository() {
        return Err(Error::ExpectedAlphaVersion(version.clone()));
    }
    run(paths, version, local_path, etc_file)
}

fn run(paths: &Paths, version: &Version, local_path: &Path, etc_file: EtcFile) -> Result<()> {
    if !paths.version_installed(version) {
        return Err(Error::VersionNotInstalled(version.clone()));
    }

    if !local_path.exists() {
        return Err(Error::FileNotFound(local_path.display().to_string()));
    }

    let etc_dir = paths.version_etc_dir(version);
    if !etc_dir.exists() {
        fs::create_dir_all(&etc_dir)?;
    }

    let dest_path = etc_dir.join(etc_file.as_str());
    fs::copy(local_path, &dest_path)?;

    print_info(format!(
        "Copied {} to {}",
        local_path.display(),
        dest_path.display()
    ));

    Ok(())
}
