// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::Result;
use crate::paths::Paths;
use crate::version::Version;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Timestamps {
    #[serde(flatten)]
    entries: HashMap<String, u64>,
}

impl Timestamps {
    pub fn load(paths: &Paths) -> Result<Self> {
        let path = paths.timestamps_file();
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path)?;
        let timestamps: Timestamps = serde_json::from_str(&content)?;
        Ok(timestamps)
    }

    pub fn save(&self, paths: &Paths) -> Result<()> {
        let path = paths.timestamps_file();
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn record(&mut self, version: &Version) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        self.entries.insert(version.to_string(), timestamp);
    }

    pub fn remove(&mut self, version: &Version) {
        self.entries.remove(&version.to_string());
    }

    pub fn get(&self, version: &Version) -> Option<u64> {
        self.entries.get(&version.to_string()).copied()
    }
}
