// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cmp::Ordering;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::errors::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Prerelease {
    Alpha(String),
    Beta(String),
    Rc(String),
}

impl Prerelease {
    pub fn alpha(identifier: impl Into<String>) -> Self {
        Prerelease::Alpha(identifier.into())
    }

    pub fn beta(identifier: impl Into<String>) -> Self {
        Prerelease::Beta(identifier.into())
    }

    pub fn rc(identifier: impl Into<String>) -> Self {
        Prerelease::Rc(identifier.into())
    }

    pub fn is_alpha(&self) -> bool {
        matches!(self, Prerelease::Alpha(_))
    }

    pub fn is_beta(&self) -> bool {
        matches!(self, Prerelease::Beta(_))
    }

    pub fn is_rc(&self) -> bool {
        matches!(self, Prerelease::Rc(_))
    }

    pub fn identifier(&self) -> &str {
        match self {
            Prerelease::Alpha(s) | Prerelease::Beta(s) | Prerelease::Rc(s) => s,
        }
    }

    pub(crate) fn parse(s: &str, full_version: &str) -> Result<Self, Error> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 2 {
            return Err(Error::InvalidVersion(full_version.to_string()));
        }

        let identifier = parts[1];
        if identifier.is_empty() {
            return Err(Error::InvalidVersion(full_version.to_string()));
        }

        match parts[0].to_lowercase().as_str() {
            "alpha" => Ok(Prerelease::Alpha(identifier.to_string())),
            "beta" => Ok(Prerelease::Beta(identifier.to_string())),
            "rc" => Ok(Prerelease::Rc(identifier.to_string())),
            _ => Err(Error::InvalidVersion(full_version.to_string())),
        }
    }
}

impl fmt::Display for Prerelease {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prerelease::Alpha(s) => write!(f, "alpha.{}", s),
            Prerelease::Beta(s) => write!(f, "beta.{}", s),
            Prerelease::Rc(s) => write!(f, "rc.{}", s),
        }
    }
}

fn compare_prerelease_identifiers(a: &str, b: &str) -> Ordering {
    match (a.parse::<u32>(), b.parse::<u32>()) {
        (Ok(na), Ok(nb)) => na.cmp(&nb),
        _ => a.cmp(b),
    }
}

impl Ord for Prerelease {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Prerelease::Alpha(a), Prerelease::Alpha(b)) => compare_prerelease_identifiers(a, b),
            (Prerelease::Alpha(_), _) => Ordering::Less,
            (_, Prerelease::Alpha(_)) => Ordering::Greater,
            (Prerelease::Beta(a), Prerelease::Beta(b)) => compare_prerelease_identifiers(a, b),
            (Prerelease::Beta(_), Prerelease::Rc(_)) => Ordering::Less,
            (Prerelease::Rc(_), Prerelease::Beta(_)) => Ordering::Greater,
            (Prerelease::Rc(a), Prerelease::Rc(b)) => compare_prerelease_identifiers(a, b),
        }
    }
}

impl PartialOrd for Prerelease {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
