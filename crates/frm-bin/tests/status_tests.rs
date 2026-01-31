// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;

use tempfile::TempDir;

use frm::commands::Status;
use frm::config::Config;
use frm::paths::Paths;
use frm::version::{Prerelease, Version};

fn setup_temp_paths() -> (TempDir, Paths) {
    let temp_dir = TempDir::new().unwrap();
    let paths = Paths::with_base_dir(temp_dir.path().to_path_buf());
    paths.ensure_dirs().unwrap();
    (temp_dir, paths)
}

fn install_version(paths: &Paths, version: &Version) {
    fs::create_dir_all(paths.version_dir(version)).unwrap();
}

fn set_default(paths: &Paths, version: &Version) {
    let mut config = Config::load(paths).unwrap();
    config.set_default(version.clone());
    config.save(paths).unwrap();
}

#[test]
fn status_no_versions_installed() {
    let (_temp, paths) = setup_temp_paths();

    let status = Status::collect(&paths).unwrap();

    assert!(status.active.is_none());
    assert!(status.default.is_none());
    assert!(status.releases.is_empty());
    assert!(status.alphas.is_empty());
}

#[test]
fn status_with_single_release() {
    let (_temp, paths) = setup_temp_paths();
    let v = Version::new(4, 2, 3);
    install_version(&paths, &v);

    let status = Status::collect(&paths).unwrap();

    assert!(status.active.is_none());
    assert!(status.default.is_none());
    assert_eq!(status.releases, vec![v]);
    assert!(status.alphas.is_empty());
}

#[test]
fn status_with_multiple_releases() {
    let (_temp, paths) = setup_temp_paths();
    let v1 = Version::new(4, 0, 0);
    let v2 = Version::new(4, 1, 8);
    let v3 = Version::new(4, 2, 3);

    install_version(&paths, &v1);
    install_version(&paths, &v2);
    install_version(&paths, &v3);

    let status = Status::collect(&paths).unwrap();

    assert_eq!(status.releases, vec![v1, v2, v3]);
}

#[test]
fn status_with_single_alpha() {
    let (_temp, paths) = setup_temp_paths();
    let alpha = Version::with_prerelease(4, 3, 0, Prerelease::Alpha("abc123".into()));
    install_version(&paths, &alpha);

    let status = Status::collect(&paths).unwrap();

    assert!(status.releases.is_empty());
    assert_eq!(status.alphas, vec![alpha]);
}

#[test]
fn status_with_mixed_versions() {
    let (_temp, paths) = setup_temp_paths();
    let ga = Version::new(4, 2, 3);
    let alpha = Version::with_prerelease(4, 3, 0, Prerelease::Alpha("abc123".into()));

    install_version(&paths, &ga);
    install_version(&paths, &alpha);

    let status = Status::collect(&paths).unwrap();

    assert_eq!(status.releases, vec![ga]);
    assert_eq!(status.alphas, vec![alpha]);
}

#[test]
fn status_with_default_set() {
    let (_temp, paths) = setup_temp_paths();
    let v = Version::new(4, 2, 3);
    install_version(&paths, &v);
    set_default(&paths, &v);

    let status = Status::collect(&paths).unwrap();

    assert!(status.active.is_none());
    assert_eq!(status.default, Some(v));
}

#[test]
fn status_separates_releases_from_alphas() {
    let (_temp, paths) = setup_temp_paths();
    let ga = Version::new(4, 2, 3);
    let beta = Version::with_prerelease(4, 3, 0, Prerelease::Beta("1".into()));
    let rc = Version::with_prerelease(4, 3, 0, Prerelease::Rc("1".into()));
    let alpha = Version::with_prerelease(4, 3, 0, Prerelease::Alpha("abc".into()));

    install_version(&paths, &ga);
    install_version(&paths, &beta);
    install_version(&paths, &rc);
    install_version(&paths, &alpha);

    let status = Status::collect(&paths).unwrap();

    assert!(status.releases.contains(&ga));
    assert!(status.releases.contains(&beta));
    assert!(status.releases.contains(&rc));
    assert!(!status.releases.contains(&alpha));

    assert!(status.alphas.contains(&alpha));
    assert!(!status.alphas.contains(&ga));
}

#[test]
fn format_empty_status() {
    let status = Status {
        active: None,
        default: None,
        releases: vec![],
        alphas: vec![],
    };

    let output = status.format();
    assert_eq!(output, "No RabbitMQ versions installed\n");
}

#[test]
fn format_with_default_only() {
    let v = Version::new(4, 2, 3);
    let status = Status {
        active: None,
        default: Some(v.clone()),
        releases: vec![v.clone()],
        alphas: vec![],
    };

    let output = status.format();
    assert!(output.contains("Default: 4.2.3"));
    assert!(output.contains("⚪ 4.2.3"));
}

#[test]
fn format_with_active_only() {
    let v = Version::new(4, 2, 3);
    let status = Status {
        active: Some(v.clone()),
        default: None,
        releases: vec![v.clone()],
        alphas: vec![],
    };

    let output = status.format();
    assert!(output.contains("Active:  4.2.3"));
    assert!(output.contains("🟢 4.2.3"));
}

#[test]
fn format_active_equals_default() {
    let v = Version::new(4, 2, 3);
    let status = Status {
        active: Some(v.clone()),
        default: Some(v.clone()),
        releases: vec![v.clone()],
        alphas: vec![],
    };

    let output = status.format();
    assert!(output.contains("Active:  4.2.3 (default)"));
    assert!(!output.contains("Default:"));
    assert!(output.contains("🟢 4.2.3"));
}

#[test]
fn format_active_differs_from_default() {
    let active = Version::new(4, 2, 3);
    let default = Version::new(4, 1, 0);
    let status = Status {
        active: Some(active.clone()),
        default: Some(default.clone()),
        releases: vec![default.clone(), active.clone()],
        alphas: vec![],
    };

    let output = status.format();
    assert!(output.contains("Active:  4.2.3"));
    assert!(output.contains("Default: 4.1.0"));
    assert!(output.contains("🟢 4.2.3"));
    assert!(output.contains("⚪ 4.1.0"));
}

#[test]
fn format_versions_listed_newest_first() {
    let v1 = Version::new(4, 0, 0);
    let v2 = Version::new(4, 1, 8);
    let v3 = Version::new(4, 2, 3);
    let status = Status {
        active: None,
        default: None,
        releases: vec![v1.clone(), v2.clone(), v3.clone()],
        alphas: vec![],
    };

    let output = status.format();
    let lines: Vec<_> = output.lines().collect();

    let pos_423 = lines.iter().position(|l| l.contains("4.2.3"));
    let pos_418 = lines.iter().position(|l| l.contains("4.1.8"));
    let pos_400 = lines.iter().position(|l| l.contains("4.0.0"));

    assert!(pos_423.unwrap() < pos_418.unwrap());
    assert!(pos_418.unwrap() < pos_400.unwrap());
}

#[test]
fn format_releases_before_alphas() {
    let ga = Version::new(4, 2, 3);
    let alpha = Version::with_prerelease(4, 3, 0, Prerelease::Alpha("abc".into()));
    let status = Status {
        active: None,
        default: None,
        releases: vec![ga.clone()],
        alphas: vec![alpha.clone()],
    };

    let output = status.format();
    let lines: Vec<_> = output.lines().collect();

    let pos_ga = lines.iter().position(|l| l.contains("4.2.3"));
    let pos_alpha = lines.iter().position(|l| l.contains("4.3.0-alpha.abc"));

    assert!(pos_ga.unwrap() < pos_alpha.unwrap());
}

#[test]
fn format_no_header_lines_when_no_active_or_default() {
    let v = Version::new(4, 2, 3);
    let status = Status {
        active: None,
        default: None,
        releases: vec![v],
        alphas: vec![],
    };

    let output = status.format();
    assert!(!output.contains("Active:"));
    assert!(!output.contains("Default:"));
    assert!(output.starts_with("Installed:"));
}

#[test]
fn format_default_not_in_list() {
    let default = Version::new(4, 2, 3);
    let installed = Version::new(4, 1, 0);
    let status = Status {
        active: None,
        default: Some(default),
        releases: vec![installed],
        alphas: vec![],
    };

    let output = status.format();
    assert!(output.contains("Default: 4.2.3"));
    assert!(output.contains("4.1.0"));
    assert!(!output.contains("⚪ 4.2.3"));
}

#[test]
fn format_active_not_in_list() {
    let active = Version::new(4, 2, 3);
    let installed = Version::new(4, 1, 0);
    let status = Status {
        active: Some(active),
        default: None,
        releases: vec![installed],
        alphas: vec![],
    };

    let output = status.format();
    assert!(output.contains("Active:  4.2.3"));
    assert!(output.contains("4.1.0"));
    assert!(!output.contains("🟢 4.2.3"));
}
