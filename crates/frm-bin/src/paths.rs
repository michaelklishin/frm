// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::Result;
use crate::errors::Error;
use crate::version::Version;

#[derive(Debug, Clone)]
pub struct Paths {
    base_dir: PathBuf,
}

impl Paths {
    pub fn new() -> Result<Self> {
        let base_dir = Self::detect_base_dir()?;
        Ok(Self { base_dir })
    }

    pub fn with_base_dir(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    fn detect_base_dir() -> Result<PathBuf> {
        if let Ok(dir) = env::var("FRM_DIR") {
            return Ok(PathBuf::from(dir));
        }

        let home =
            dirs::home_dir().ok_or_else(|| Error::Config("cannot find home directory".into()))?;

        Ok(home.join(".local").join("frm"))
    }

    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    pub fn versions_dir(&self) -> PathBuf {
        self.base_dir.join("versions")
    }

    pub fn version_dir(&self, version: &Version) -> PathBuf {
        self.versions_dir().join(version.dir_name())
    }

    pub fn version_sbin_dir(&self, version: &Version) -> PathBuf {
        self.version_dir(version).join("sbin")
    }

    pub fn version_etc_dir(&self, version: &Version) -> PathBuf {
        self.version_dir(version).join("etc").join("rabbitmq")
    }

    pub fn version_var_log_dir(&self, version: &Version) -> PathBuf {
        self.version_dir(version)
            .join("var")
            .join("log")
            .join("rabbitmq")
    }

    pub fn etc_dir(&self) -> PathBuf {
        self.base_dir.join("etc").join("rabbitmq")
    }

    pub fn downloads_dir(&self) -> PathBuf {
        self.base_dir.join("downloads")
    }

    pub fn config_file(&self) -> PathBuf {
        self.base_dir.join("config.toml")
    }

    pub fn default_file(&self) -> PathBuf {
        self.base_dir.join("default")
    }

    pub fn timestamps_file(&self) -> PathBuf {
        self.base_dir.join("version_timestamps.json")
    }

    pub fn ensure_dirs(&self) -> Result<()> {
        fs::create_dir_all(self.versions_dir())?;
        fs::create_dir_all(self.downloads_dir())?;
        fs::create_dir_all(self.etc_dir())?;
        Ok(())
    }

    pub fn version_installed(&self, version: &Version) -> bool {
        self.version_dir(version).exists()
    }

    pub fn installed_versions(&self) -> Result<Vec<Version>> {
        let versions_dir = self.versions_dir();
        if !versions_dir.exists() {
            return Ok(Vec::new());
        }

        let mut versions = Vec::new();
        for entry in fs::read_dir(versions_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir()
                && let Some(name) = entry.file_name().to_str()
                && let Ok(version) = name.parse::<Version>()
            {
                versions.push(version);
            }
        }

        versions.sort();
        Ok(versions)
    }

    pub fn installed_alpha_versions(&self) -> Result<Vec<Version>> {
        let versions = self.installed_versions()?;
        Ok(versions
            .into_iter()
            .filter(|v| v.is_distributed_via_server_packages_repository())
            .collect())
    }

    pub fn latest_ga_version(&self) -> Result<Option<Version>> {
        let versions = self.installed_versions()?;
        Ok(versions.into_iter().rev().find(|v| v.is_ga()))
    }

    pub fn latest_alpha_version(&self) -> Result<Option<Version>> {
        let versions = self.installed_versions()?;
        Ok(versions
            .into_iter()
            .rev()
            .find(|v| v.is_distributed_via_server_packages_repository()))
    }
}

impl Default for Paths {
    fn default() -> Self {
        Self::new().expect("failed to initialize paths")
    }
}
