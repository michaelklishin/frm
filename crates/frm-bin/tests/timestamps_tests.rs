// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;

use tempfile::TempDir;

use frm::paths::Paths;
use frm::timestamps::Timestamps;
use frm::version::Version;

fn setup_temp_paths() -> (TempDir, Paths) {
    let temp_dir = TempDir::new().unwrap();
    let paths = Paths::with_base_dir(temp_dir.path().to_path_buf());
    (temp_dir, paths)
}

#[test]
fn timestamps_default_empty() {
    let timestamps = Timestamps::default();
    let version = Version::new(4, 2, 3);
    assert!(timestamps.get(&version).is_none());
}

#[test]
fn timestamps_record_and_get() {
    let mut timestamps = Timestamps::default();
    let version = Version::new(4, 2, 3);

    timestamps.record(&version);

    let ts = timestamps.get(&version);
    assert!(ts.is_some());
    assert!(ts.unwrap() > 0);
}

#[test]
fn timestamps_remove() {
    let mut timestamps = Timestamps::default();
    let version = Version::new(4, 2, 3);

    timestamps.record(&version);
    assert!(timestamps.get(&version).is_some());

    timestamps.remove(&version);
    assert!(timestamps.get(&version).is_none());
}

#[test]
fn timestamps_load_nonexistent() {
    let (_temp, paths) = setup_temp_paths();
    let timestamps = Timestamps::load(&paths).unwrap();
    assert!(timestamps.get(&Version::new(4, 2, 3)).is_none());
}

#[test]
fn timestamps_save_and_load() {
    let (temp, paths) = setup_temp_paths();
    fs::create_dir_all(temp.path()).unwrap();

    let version1 = Version::new(4, 2, 3);
    let version2 = Version::new(4, 0, 0);

    let mut timestamps = Timestamps::default();
    timestamps.record(&version1);
    timestamps.record(&version2);
    timestamps.save(&paths).unwrap();

    let loaded = Timestamps::load(&paths).unwrap();
    assert!(loaded.get(&version1).is_some());
    assert!(loaded.get(&version2).is_some());
}

#[test]
fn timestamps_save_creates_file() {
    let (temp, paths) = setup_temp_paths();
    fs::create_dir_all(temp.path()).unwrap();

    let mut timestamps = Timestamps::default();
    timestamps.record(&Version::new(4, 2, 3));
    timestamps.save(&paths).unwrap();

    assert!(paths.timestamps_file().exists());
}

#[test]
fn timestamps_file_is_json() {
    let (temp, paths) = setup_temp_paths();
    fs::create_dir_all(temp.path()).unwrap();

    let mut timestamps = Timestamps::default();
    timestamps.record(&Version::new(4, 2, 3));
    timestamps.save(&paths).unwrap();

    let content = fs::read_to_string(paths.timestamps_file()).unwrap();
    assert!(content.contains("4.2.3"));
    let _: serde_json::Value = serde_json::from_str(&content).unwrap();
}

#[test]
fn timestamps_prerelease_versions() {
    let mut timestamps = Timestamps::default();
    let alpha = "4.3.0-alpha.132057c7".parse::<Version>().unwrap();
    let beta = "4.2.0-beta.1".parse::<Version>().unwrap();
    let rc = "4.2.0-rc.1".parse::<Version>().unwrap();

    timestamps.record(&alpha);
    timestamps.record(&beta);
    timestamps.record(&rc);

    assert!(timestamps.get(&alpha).is_some());
    assert!(timestamps.get(&beta).is_some());
    assert!(timestamps.get(&rc).is_some());
}

#[test]
fn timestamps_multiple_records_same_version() {
    let mut timestamps = Timestamps::default();
    let version = Version::new(4, 2, 3);

    timestamps.record(&version);
    let first_ts = timestamps.get(&version).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(10));

    timestamps.record(&version);
    let second_ts = timestamps.get(&version).unwrap();

    assert!(second_ts >= first_ts);
}

#[test]
fn timestamps_remove_nonexistent() {
    let mut timestamps = Timestamps::default();
    let version = Version::new(4, 2, 3);

    timestamps.remove(&version);
    assert!(timestamps.get(&version).is_none());
}

#[test]
fn timestamps_load_corrupt_file() {
    let (temp, paths) = setup_temp_paths();
    fs::create_dir_all(temp.path()).unwrap();
    fs::write(paths.timestamps_file(), "not valid json {{{{").unwrap();

    let result = Timestamps::load(&paths);
    assert!(result.is_err());
}

#[test]
fn timestamps_load_empty_file() {
    let (temp, paths) = setup_temp_paths();
    fs::create_dir_all(temp.path()).unwrap();
    fs::write(paths.timestamps_file(), "{}").unwrap();

    let timestamps = Timestamps::load(&paths).unwrap();
    assert!(timestamps.get(&Version::new(4, 2, 3)).is_none());
}
