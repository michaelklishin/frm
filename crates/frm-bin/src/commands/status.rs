// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;
use std::io::{self, Write};

use crate::Result;
use crate::common::env_vars::RABBITMQ_HOME;
use crate::config::Config;
use crate::paths::Paths;
use crate::version::Version;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    pub active: Option<Version>,
    pub default: Option<Version>,
    pub releases: Vec<Version>,
    pub alphas: Vec<Version>,
}

impl Status {
    pub fn collect(paths: &Paths) -> Result<Self> {
        let active = detect_active_version(paths);
        let config = Config::load(paths)?;
        let default = config.default_version;

        let all_versions = paths.installed_versions()?;
        let (alphas, releases): (Vec<_>, Vec<_>) = all_versions
            .into_iter()
            .partition(|v| v.is_distributed_via_server_packages_repository());

        Ok(Self {
            active,
            default,
            releases,
            alphas,
        })
    }

    pub fn format(&self) -> String {
        let mut out = String::new();

        match (&self.active, &self.default) {
            (Some(active), Some(default)) if active == default => {
                out.push_str(&format!("Active:  {} (default)\n", active));
            }
            (Some(active), Some(default)) => {
                out.push_str(&format!("Active:  {}\n", active));
                out.push_str(&format!("Default: {}\n", default));
            }
            (Some(active), None) => {
                out.push_str(&format!("Active:  {}\n", active));
            }
            (None, Some(default)) => {
                out.push_str(&format!("Default: {}\n", default));
            }
            (None, None) => {}
        }

        if self.releases.is_empty() && self.alphas.is_empty() {
            if out.is_empty() {
                out.push_str("No RabbitMQ versions installed\n");
            }
            return out;
        }

        if !out.is_empty() {
            out.push('\n');
        }

        out.push_str("Installed:\n\n");

        for version in self.releases.iter().rev() {
            let marker = self.version_marker(version);
            out.push_str(&format!("  {} {}\n", marker, version));
        }

        for version in self.alphas.iter().rev() {
            let marker = self.version_marker(version);
            out.push_str(&format!("  {} {}\n", marker, version));
        }

        out
    }

    fn version_marker(&self, version: &Version) -> &'static str {
        let is_active = self.active.as_ref() == Some(version);
        let is_default = self.default.as_ref() == Some(version);

        match (is_active, is_default) {
            (true, _) => "🟢",
            (false, true) => "⚪",
            (false, false) => "  ",
        }
    }
}

fn detect_active_version(paths: &Paths) -> Option<Version> {
    let rabbitmq_home = env::var(RABBITMQ_HOME).ok()?;
    let versions_dir = paths.versions_dir();
    let versions_prefix = versions_dir.to_string_lossy();

    if !rabbitmq_home.starts_with(versions_prefix.as_ref()) {
        return None;
    }

    let version_str = rabbitmq_home
        .strip_prefix(versions_prefix.as_ref())?
        .trim_start_matches('/');

    version_str.parse().ok()
}

pub fn run(paths: &Paths) -> Result<()> {
    let status = Status::collect(paths)?;
    let output = status.format();
    io::stdout().write_all(output.as_bytes())?;
    Ok(())
}
