// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;

use bel7_cli::{print_info, print_warning};
use rabbitmq_conf::{RabbitMQConf, keys};

use crate::Result;
use crate::errors::Error;
use crate::paths::Paths;
use crate::version::Version;

/// Get a configuration key value from rabbitmq.conf
pub fn get_key(paths: &Paths, version: &Version, key: &str) -> Result<()> {
    if !paths.version_installed(version) {
        return Err(Error::VersionNotInstalled(version.clone()));
    }

    let conf_path = paths.version_etc_dir(version).join("rabbitmq.conf");
    if !conf_path.exists() {
        return Err(Error::FileNotFound(conf_path.display().to_string()));
    }

    let conf = RabbitMQConf::load(&conf_path).map_err(|e| Error::Config(e.to_string()))?;

    if RabbitMQConf::is_pattern(key) {
        let matches = conf.get_matching(key);
        if matches.is_empty() {
            return Err(Error::Config(format!("no keys matching pattern: {}", key)));
        }
        for (k, v) in matches {
            println!("{} = {}", k, v);
        }
        Ok(())
    } else {
        match conf.get(key) {
            Some(value) => {
                println!("{}", value);
                Ok(())
            }
            None => Err(Error::Config(format!("key not found: {}", key))),
        }
    }
}

/// Set a configuration key value in rabbitmq.conf
pub fn set_key(
    paths: &Paths,
    version: &Version,
    key: &str,
    value: &str,
    force: bool,
) -> Result<()> {
    if !paths.version_installed(version) {
        return Err(Error::VersionNotInstalled(version.clone()));
    }

    // Validate key format
    if !keys::is_valid_key_format(key) {
        return Err(Error::Config(format!("invalid key format: {}", key)));
    }

    // Check if key is known
    if !keys::is_known_key(key) {
        if force {
            print_warning(format!("unknown key: {}", key));
        } else {
            let suggestions = keys::suggest_similar_keys(key);
            let msg = if suggestions.is_empty() {
                format!("unknown configuration key: {}", key)
            } else {
                format!(
                    "unknown configuration key: {}. Similar keys: {}",
                    key,
                    suggestions.join(", ")
                )
            };
            return Err(Error::Config(msg));
        }
    }

    let etc_dir = paths.version_etc_dir(version);
    let conf_path = etc_dir.join("rabbitmq.conf");

    // Ensure the etc directory exists
    if !etc_dir.exists() {
        fs::create_dir_all(&etc_dir)?;
    }

    // Load existing config or create new
    let mut conf = if conf_path.exists() {
        RabbitMQConf::load(&conf_path).map_err(|e| Error::Config(e.to_string()))?
    } else {
        RabbitMQConf::new()
    };

    let was_updated = conf.contains_key(key);
    conf.set(key, value);

    conf.save(&conf_path)
        .map_err(|e| Error::Config(e.to_string()))?;

    if was_updated {
        print_info(format!("updated {} = {}", key, value));
    } else {
        print_info(format!("set {} = {}", key, value));
    }

    Ok(())
}
