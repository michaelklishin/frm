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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Prerelease {
    Alpha(u32),
    Beta(u32),
    Rc(u32),
}

impl Prerelease {
    fn order(&self) -> (u8, u32) {
        match self {
            Prerelease::Alpha(n) => (0, *n),
            Prerelease::Beta(n) => (1, *n),
            Prerelease::Rc(n) => (2, *n),
        }
    }
}

impl fmt::Display for Prerelease {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prerelease::Alpha(n) => write!(f, "alpha.{}", n),
            Prerelease::Beta(n) => write!(f, "beta.{}", n),
            Prerelease::Rc(n) => write!(f, "rc.{}", n),
        }
    }
}

impl Ord for Prerelease {
    fn cmp(&self, other: &Self) -> Ordering {
        self.order().cmp(&other.order())
    }
}

impl PartialOrd for Prerelease {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

    pub fn download_url(&self) -> String {
        format!(
            "https://github.com/rabbitmq/rabbitmq-server/releases/download/v{v}/rabbitmq-server-generic-unix-{v}.tar.xz",
            v = self
        )
    }

    pub fn archive_name(&self) -> String {
        format!("rabbitmq-server-generic-unix-{}.tar.xz", self)
    }

    pub fn extracted_dir_name(&self) -> String {
        format!("rabbitmq_server-{}", self)
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
            (ver, Some(parse_prerelease(pre, s)?))
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

fn parse_prerelease(s: &str, full: &str) -> Result<Prerelease, Error> {
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 2 {
        return Err(Error::InvalidVersion(full.to_string()));
    }

    let num: u32 = parts[1]
        .parse()
        .map_err(|_| Error::InvalidVersion(full.to_string()))?;

    match parts[0].to_lowercase().as_str() {
        "alpha" => Ok(Prerelease::Alpha(num)),
        "beta" => Ok(Prerelease::Beta(num)),
        "rc" => Ok(Prerelease::Rc(num)),
        _ => Err(Error::InvalidVersion(full.to_string())),
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
