// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;

use serde::{Deserialize, Serialize};

use crate::Result;
use crate::paths::Paths;
use crate::version::Version;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<Version>,
}

impl Config {
    pub fn load(paths: &Paths) -> Result<Self> {
        let config_file = paths.config_file();
        if !config_file.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(config_file)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self, paths: &Paths) -> Result<()> {
        let config_file = paths.config_file();
        let content = toml::to_string_pretty(self)?;
        fs::write(config_file, content)?;
        Ok(())
    }

    pub fn set_default(&mut self, version: Version) {
        self.default_version = Some(version);
    }

    pub fn clear_default(&mut self) {
        self.default_version = None;
    }
}
