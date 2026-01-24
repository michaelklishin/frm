// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;

use tempfile::TempDir;

use frm::config::Config;
use frm::paths::Paths;
use frm::version::{Prerelease, Version};

fn setup_temp_paths() -> (TempDir, Paths) {
    let temp_dir = TempDir::new().unwrap();
    let paths = Paths::with_base_dir(temp_dir.path().to_path_buf());
    (temp_dir, paths)
}

#[test]
fn config_default() {
    let config = Config::default();
    assert!(config.default_version.is_none());
}

#[test]
fn config_load_nonexistent() {
    let (_temp, paths) = setup_temp_paths();
    let config = Config::load(&paths).unwrap();
    assert!(config.default_version.is_none());
}

#[test]
fn config_save_and_load() {
    let (temp, paths) = setup_temp_paths();
    fs::create_dir_all(temp.path()).unwrap();

    let version = Version::new(4, 2, 3);
    let mut config = Config::default();
    config.set_default(version.clone());
    config.save(&paths).unwrap();

    let loaded = Config::load(&paths).unwrap();
    assert_eq!(loaded.default_version, Some(version));
}

#[test]
fn config_set_default() {
    let mut config = Config::default();
    let version = Version::new(4, 2, 3);

    config.set_default(version.clone());
    assert_eq!(config.default_version, Some(version));
}

#[test]
fn config_clear_default() {
    let mut config = Config::default();
    let version = Version::new(4, 2, 3);

    config.set_default(version);
    config.clear_default();
    assert!(config.default_version.is_none());
}

#[test]
fn config_serialization_format() {
    let (temp, paths) = setup_temp_paths();
    fs::create_dir_all(temp.path()).unwrap();

    let version = Version::new(4, 2, 3);
    let mut config = Config::default();
    config.set_default(version);
    config.save(&paths).unwrap();

    let content = fs::read_to_string(paths.config_file()).unwrap();
    assert!(content.contains("default_version"));
    assert!(content.contains("major = 4"));
    assert!(content.contains("minor = 2"));
    assert!(content.contains("patch = 3"));
}

#[test]
fn config_prerelease_save_and_load() {
    let (temp, paths) = setup_temp_paths();
    fs::create_dir_all(temp.path()).unwrap();

    let version = Version::with_prerelease(4, 2, 4, Prerelease::Alpha(2));
    let mut config = Config::default();
    config.set_default(version.clone());
    config.save(&paths).unwrap();

    let loaded = Config::load(&paths).unwrap();
    assert_eq!(loaded.default_version, Some(version));

    let content = fs::read_to_string(paths.config_file()).unwrap();
    assert!(content.contains("Alpha"));
}

#[test]
fn config_load_corrupt_file() {
    let (temp, paths) = setup_temp_paths();
    fs::create_dir_all(temp.path()).unwrap();
    fs::write(paths.config_file(), "this is not valid toml {{{{").unwrap();

    let result = Config::load(&paths);
    assert!(result.is_err());
}

#[test]
fn config_load_empty_file() {
    let (temp, paths) = setup_temp_paths();
    fs::create_dir_all(temp.path()).unwrap();
    fs::write(paths.config_file(), "").unwrap();

    let config = Config::load(&paths).unwrap();
    assert!(config.default_version.is_none());
}
