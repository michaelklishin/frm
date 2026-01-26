// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::errors::Error;
use crate::prerelease::Prerelease;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerelease: Option<Prerelease>,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            prerelease: None,
        }
    }

    pub fn with_prerelease(major: u32, minor: u32, patch: u32, prerelease: Prerelease) -> Self {
        Self {
            major,
            minor,
            patch,
            prerelease: Some(prerelease),
        }
    }

    pub fn dir_name(&self) -> String {
        self.to_string()
    }

    pub fn is_ga(&self) -> bool {
        self.prerelease.is_none()
    }

    pub fn is_prerelease(&self) -> bool {
        self.prerelease.is_some()
    }

    pub fn is_alpha(&self) -> bool {
        self.prerelease.as_ref().is_some_and(|p| p.is_alpha())
    }

    pub fn is_beta(&self) -> bool {
        self.prerelease.as_ref().is_some_and(|p| p.is_beta())
    }

    pub fn is_rc(&self) -> bool {
        self.prerelease.as_ref().is_some_and(|p| p.is_rc())
    }

    pub fn is_distributed_via_server_packages_repository(&self) -> bool {
        self.is_alpha()
    }

    pub fn download_url(&self) -> String {
        format!(
            "https://github.com/rabbitmq/rabbitmq-server/releases/download/v{v}/rabbitmq-server-generic-unix-{v}.tar.xz",
            v = self
        )
    }

    pub fn download_url_with_tag(&self, tag: &str) -> String {
        format!(
            "https://github.com/rabbitmq/server-packages/releases/download/{tag}/rabbitmq-server-generic-unix-{v}.tar.xz",
            tag = tag,
            v = self
        )
    }

    pub fn archive_name(&self) -> String {
        format!("rabbitmq-server-generic-unix-{}.tar.xz", self)
    }

    pub fn extracted_dir_name(&self) -> String {
        format!("rabbitmq_server-{}", self)
    }

    pub fn base_version(&self) -> Version {
        Version::new(self.major, self.minor, self.patch)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(ref pre) = self.prerelease {
            write!(f, "-{}", pre)?;
        }
        Ok(())
    }
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().trim_start_matches('v');

        let (version_part, prerelease) = if let Some(idx) = s.find('-') {
            let (ver, pre) = s.split_at(idx);
            let pre = &pre[1..];
            (ver, Some(Prerelease::parse(pre, s)?))
        } else {
            (s, None)
        };

        let parts: Vec<&str> = version_part.split('.').collect();

        if parts.len() != 3 {
            return Err(Error::InvalidVersion(s.to_string()));
        }

        let major = parts[0]
            .parse()
            .map_err(|_| Error::InvalidVersion(s.to_string()))?;
        let minor = parts[1]
            .parse()
            .map_err(|_| Error::InvalidVersion(s.to_string()))?;
        let patch = parts[2]
            .parse()
            .map_err(|_| Error::InvalidVersion(s.to_string()))?;

        Ok(Version {
            major,
            minor,
            patch,
            prerelease,
        })
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        let base =
            (self.major, self.minor, self.patch).cmp(&(other.major, other.minor, other.patch));
        if base != Ordering::Equal {
            return base;
        }

        match (&self.prerelease, &other.prerelease) {
            (None, None) => Ordering::Equal,
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (Some(a), Some(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
