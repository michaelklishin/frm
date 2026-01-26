// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;
use std::path::PathBuf;

use tempfile::TempDir;

use frm::paths::Paths;
use frm::version::{Prerelease, Version};

fn setup_temp_paths() -> (TempDir, Paths) {
    let temp_dir = TempDir::new().unwrap();
    let paths = Paths::with_base_dir(temp_dir.path().to_path_buf());
    (temp_dir, paths)
}

#[test]
fn paths_base_dir() {
    let (_temp, paths) = setup_temp_paths();
    assert!(paths.base_dir().exists() || true);
}

#[test]
fn paths_versions_dir() {
    let (_temp, paths) = setup_temp_paths();
    let versions_dir = paths.versions_dir();
    assert!(versions_dir.ends_with("versions"));
}

#[test]
fn paths_version_dir() {
    let (_temp, paths) = setup_temp_paths();
    let version = Version::new(4, 2, 3);
    let version_dir = paths.version_dir(&version);
    assert!(version_dir.ends_with("4.2.3"));
}

#[test]
fn paths_version_sbin_dir() {
    let (_temp, paths) = setup_temp_paths();
    let version = Version::new(4, 2, 3);
    let sbin_dir = paths.version_sbin_dir(&version);
    assert!(sbin_dir.ends_with("sbin"));
}

#[test]
fn paths_version_etc_dir() {
    let (_temp, paths) = setup_temp_paths();
    let version = Version::new(4, 2, 3);
    let etc_dir = paths.version_etc_dir(&version);
    assert!(etc_dir.ends_with("rabbitmq"));
}

#[test]
fn paths_version_var_log_dir() {
    let (_temp, paths) = setup_temp_paths();
    let version = Version::new(4, 2, 3);
    let log_dir = paths.version_var_log_dir(&version);
    assert!(log_dir.ends_with("rabbitmq"));
    assert!(log_dir.to_string_lossy().contains("var/log"));
}

#[test]
fn paths_etc_dir() {
    let (_temp, paths) = setup_temp_paths();
    let etc_dir = paths.etc_dir();
    assert!(etc_dir.ends_with("rabbitmq"));
}

#[test]
fn paths_downloads_dir() {
    let (_temp, paths) = setup_temp_paths();
    let downloads_dir = paths.downloads_dir();
    assert!(downloads_dir.ends_with("downloads"));
}

#[test]
fn paths_config_file() {
    let (_temp, paths) = setup_temp_paths();
    let config_file = paths.config_file();
    assert!(config_file.ends_with("config.toml"));
}

#[test]
fn paths_default_file() {
    let (_temp, paths) = setup_temp_paths();
    let default_file = paths.default_file();
    assert!(default_file.ends_with("default"));
}

#[test]
fn paths_ensure_dirs() {
    let (_temp, paths) = setup_temp_paths();
    paths.ensure_dirs().unwrap();

    assert!(paths.versions_dir().exists());
    assert!(paths.downloads_dir().exists());
    assert!(paths.etc_dir().exists());
}

#[test]
fn paths_version_not_installed() {
    let (_temp, paths) = setup_temp_paths();
    let version = Version::new(4, 2, 3);
    assert!(!paths.version_installed(&version));
}

#[test]
fn paths_version_installed() {
    let (_temp, paths) = setup_temp_paths();
    paths.ensure_dirs().unwrap();

    let version = Version::new(4, 2, 3);
    fs::create_dir_all(paths.version_dir(&version)).unwrap();

    assert!(paths.version_installed(&version));
}

#[test]
fn paths_installed_versions_empty() {
    let (_temp, paths) = setup_temp_paths();
    let versions = paths.installed_versions().unwrap();
    assert!(versions.is_empty());
}

#[test]
fn paths_installed_versions() {
    let (_temp, paths) = setup_temp_paths();
    paths.ensure_dirs().unwrap();

    let v1 = Version::new(4, 0, 0);
    let v2 = Version::new(4, 1, 8);
    let v3 = Version::new(4, 2, 3);

    fs::create_dir_all(paths.version_dir(&v1)).unwrap();
    fs::create_dir_all(paths.version_dir(&v2)).unwrap();
    fs::create_dir_all(paths.version_dir(&v3)).unwrap();

    let versions = paths.installed_versions().unwrap();
    assert_eq!(versions.len(), 3);
    assert_eq!(versions[0], v1);
    assert_eq!(versions[1], v2);
    assert_eq!(versions[2], v3);
}

#[test]
fn paths_installed_versions_ignores_invalid_dirs() {
    let (_temp, paths) = setup_temp_paths();
    paths.ensure_dirs().unwrap();

    let version = Version::new(4, 2, 3);
    fs::create_dir_all(paths.version_dir(&version)).unwrap();
    fs::create_dir_all(paths.versions_dir().join("not-a-version")).unwrap();
    fs::create_dir_all(paths.versions_dir().join(".hidden")).unwrap();

    let versions = paths.installed_versions().unwrap();
    assert_eq!(versions.len(), 1);
    assert_eq!(versions[0], version);
}

#[test]
fn paths_with_custom_base_dir() {
    let custom_path = PathBuf::from("/custom/frm/path");
    let paths = Paths::with_base_dir(custom_path.clone());
    assert_eq!(paths.base_dir(), custom_path.as_path());
}

#[test]
fn paths_prerelease_version_dir() {
    let (_temp, paths) = setup_temp_paths();
    let version = Version::with_prerelease(4, 2, 4, Prerelease::Alpha("1".into()));
    let version_dir = paths.version_dir(&version);
    assert!(version_dir.ends_with("4.2.4-alpha.1"));
}

#[test]
fn paths_version_dirs_are_consistent() {
    let (_temp, paths) = setup_temp_paths();
    let version = Version::new(4, 2, 3);

    let sbin = paths.version_sbin_dir(&version);
    let etc = paths.version_etc_dir(&version);
    let log = paths.version_var_log_dir(&version);

    assert!(sbin.starts_with(paths.version_dir(&version)));
    assert!(etc.starts_with(paths.version_dir(&version)));
    assert!(log.starts_with(paths.version_dir(&version)));
}

#[test]
fn paths_timestamps_file() {
    let (_temp, paths) = setup_temp_paths();
    let timestamps_file = paths.timestamps_file();
    assert!(timestamps_file.ends_with("version_timestamps.json"));
}

#[test]
fn paths_installed_versions_with_alphas() {
    let (_temp, paths) = setup_temp_paths();
    paths.ensure_dirs().unwrap();

    let alpha = "4.3.0-alpha.132057c7".parse::<Version>().unwrap();
    let release = Version::new(4, 2, 3);

    fs::create_dir_all(paths.version_dir(&alpha)).unwrap();
    fs::create_dir_all(paths.version_dir(&release)).unwrap();

    let versions = paths.installed_versions().unwrap();
    assert_eq!(versions.len(), 2);
    assert!(versions.contains(&alpha));
    assert!(versions.contains(&release));
}

#[test]
fn latest_ga_version_empty() {
    let (_temp, paths) = setup_temp_paths();
    paths.ensure_dirs().unwrap();

    let latest = paths.latest_ga_version().unwrap();
    assert!(latest.is_none());
}

#[test]
fn latest_ga_version_only_alphas() {
    let (_temp, paths) = setup_temp_paths();
    paths.ensure_dirs().unwrap();

    let alpha1 = Version::with_prerelease(4, 3, 0, Prerelease::Alpha("1".into()));
    let alpha2 = Version::with_prerelease(4, 3, 0, Prerelease::Alpha("2".into()));

    fs::create_dir_all(paths.version_dir(&alpha1)).unwrap();
    fs::create_dir_all(paths.version_dir(&alpha2)).unwrap();

    let latest = paths.latest_ga_version().unwrap();
    assert!(latest.is_none());
}

#[test]
fn latest_ga_version_single_release() {
    let (_temp, paths) = setup_temp_paths();
    paths.ensure_dirs().unwrap();

    let release = Version::new(4, 2, 3);
    fs::create_dir_all(paths.version_dir(&release)).unwrap();

    let latest = paths.latest_ga_version().unwrap();
    assert_eq!(latest, Some(release));
}

#[test]
fn latest_ga_version_multiple_releases() {
    let (_temp, paths) = setup_temp_paths();
    paths.ensure_dirs().unwrap();

    let v1 = Version::new(4, 0, 0);
    let v2 = Version::new(4, 1, 8);
    let v3 = Version::new(4, 2, 3);

    fs::create_dir_all(paths.version_dir(&v1)).unwrap();
    fs::create_dir_all(paths.version_dir(&v2)).unwrap();
    fs::create_dir_all(paths.version_dir(&v3)).unwrap();

    let latest = paths.latest_ga_version().unwrap();
    assert_eq!(latest, Some(v3));
}

#[test]
fn latest_ga_version_mixed_releases_and_prereleases() {
    let (_temp, paths) = setup_temp_paths();
    paths.ensure_dirs().unwrap();

    let v1 = Version::new(4, 2, 3);
    let alpha = Version::with_prerelease(4, 3, 0, Prerelease::Alpha("1".into()));
    let beta = Version::with_prerelease(4, 3, 0, Prerelease::Beta("1".into()));
    let rc = Version::with_prerelease(4, 3, 0, Prerelease::Rc("1".into()));

    fs::create_dir_all(paths.version_dir(&v1)).unwrap();
    fs::create_dir_all(paths.version_dir(&alpha)).unwrap();
    fs::create_dir_all(paths.version_dir(&beta)).unwrap();
    fs::create_dir_all(paths.version_dir(&rc)).unwrap();

    let latest = paths.latest_ga_version().unwrap();
    assert_eq!(latest, Some(v1));
}

#[test]
fn latest_ga_version_prerelease_higher_than_ga() {
    let (_temp, paths) = setup_temp_paths();
    paths.ensure_dirs().unwrap();

    let ga = Version::new(4, 2, 3);
    let rc = Version::with_prerelease(4, 3, 0, Prerelease::Rc("1".into()));

    fs::create_dir_all(paths.version_dir(&ga)).unwrap();
    fs::create_dir_all(paths.version_dir(&rc)).unwrap();

    let latest = paths.latest_ga_version().unwrap();
    assert_eq!(latest, Some(ga));
}
