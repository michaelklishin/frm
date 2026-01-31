// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;

use proptest::prelude::*;
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

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    #[test]
    fn status_collects_all_ga_versions(
        versions in prop::collection::vec((0u32..10, 0u32..10, 0u32..10), 1..5)
    ) {
        let (_temp, paths) = setup_temp_paths();

        let mut expected = Vec::new();
        for (major, minor, patch) in versions {
            let v = Version::new(major, minor, patch);
            if !expected.contains(&v) {
                install_version(&paths, &v);
                expected.push(v);
            }
        }
        expected.sort();

        let status = Status::collect(&paths).unwrap();

        prop_assert_eq!(status.releases, expected);
        prop_assert!(status.alphas.is_empty());
    }

    #[test]
    fn status_separates_alphas_from_releases(
        ga_versions in prop::collection::vec((0u32..5, 0u32..10, 0u32..10), 1..3),
        alpha_ids in prop::collection::vec("[a-z0-9]{6}", 1..3)
    ) {
        let (_temp, paths) = setup_temp_paths();

        let mut expected_releases = Vec::new();
        for (major, minor, patch) in ga_versions {
            let v = Version::new(major, minor, patch);
            if !expected_releases.contains(&v) {
                install_version(&paths, &v);
                expected_releases.push(v);
            }
        }
        expected_releases.sort();

        let mut expected_alphas = Vec::new();
        for alpha_id in alpha_ids {
            let v = Version::with_prerelease(4, 3, 0, Prerelease::Alpha(alpha_id));
            if !expected_alphas.contains(&v) {
                install_version(&paths, &v);
                expected_alphas.push(v);
            }
        }
        expected_alphas.sort();

        let status = Status::collect(&paths).unwrap();

        prop_assert_eq!(status.releases, expected_releases);
        prop_assert_eq!(status.alphas, expected_alphas);
    }

    #[test]
    fn status_default_matches_config(
        major in 0u32..10,
        minor in 0u32..10,
        patch in 0u32..10
    ) {
        let (_temp, paths) = setup_temp_paths();
        let v = Version::new(major, minor, patch);
        install_version(&paths, &v);
        set_default(&paths, &v);

        let status = Status::collect(&paths).unwrap();

        prop_assert_eq!(status.default, Some(v));
    }

    #[test]
    fn format_contains_all_versions(
        versions in prop::collection::vec((0u32..10, 0u32..10, 0u32..10), 1..5)
    ) {
        let mut releases = Vec::new();
        for (major, minor, patch) in versions {
            let v = Version::new(major, minor, patch);
            if !releases.contains(&v) {
                releases.push(v);
            }
        }

        let status = Status {
            active: None,
            default: None,
            releases: releases.clone(),
            alphas: vec![],
        };

        let output = status.format();

        for v in &releases {
            prop_assert!(output.contains(&v.to_string()), "Missing version {}", v);
        }
    }

    #[test]
    fn format_active_marker_only_on_active_version(
        major in 0u32..10,
        minor in 0u32..10,
        patch in 0u32..10,
        other_major in 0u32..10,
        other_minor in 0u32..10,
        other_patch in 0u32..10
    ) {
        let active = Version::new(major, minor, patch);
        let other = Version::new(other_major, other_minor, other_patch);

        let mut releases = vec![active.clone()];
        if other != active {
            releases.push(other.clone());
        }
        releases.sort();

        let status = Status {
            active: Some(active.clone()),
            default: None,
            releases: releases.clone(),
            alphas: vec![],
        };

        let output = status.format();
        let active_str = active.to_string();

        for line in output.lines() {
            if line.contains("Installed") || line.contains("Active") {
                continue;
            }
            if line.contains(&active_str) {
                prop_assert!(line.contains("🟢"), "Active version line should have 🟢 marker: {}", line);
            }
        }
    }

    #[test]
    fn format_default_marker_only_on_default_version(
        major in 0u32..10,
        minor in 0u32..10,
        patch in 0u32..10,
        other_major in 0u32..10,
        other_minor in 0u32..10,
        other_patch in 0u32..10
    ) {
        let default = Version::new(major, minor, patch);
        let other = Version::new(other_major, other_minor, other_patch);

        let mut releases = vec![default.clone()];
        if other != default {
            releases.push(other.clone());
        }
        releases.sort();

        let status = Status {
            active: None,
            default: Some(default.clone()),
            releases: releases.clone(),
            alphas: vec![],
        };

        let output = status.format();
        let default_str = default.to_string();

        for line in output.lines() {
            if line.contains("Installed") || line.contains("Default") {
                continue;
            }
            if line.contains(&default_str) {
                prop_assert!(line.contains("⚪"), "Default version line should have ⚪ marker: {}", line);
            }
        }
    }
}
