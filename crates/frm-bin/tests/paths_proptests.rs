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

use frm::paths::Paths;
use frm::version::{Prerelease, Version};

fn setup_temp_paths() -> (TempDir, Paths) {
    let temp_dir = TempDir::new().unwrap();
    let paths = Paths::with_base_dir(temp_dir.path().to_path_buf());
    (temp_dir, paths)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    #[test]
    fn latest_ga_returns_highest_ga_version(
        versions in prop::collection::vec((0u32..10, 0u32..10, 0u32..10), 1..5)
    ) {
        let (_temp, paths) = setup_temp_paths();
        paths.ensure_dirs().unwrap();

        let mut ga_versions = Vec::new();
        for (major, minor, patch) in versions {
            let v = Version::new(major, minor, patch);
            fs::create_dir_all(paths.version_dir(&v)).unwrap();
            ga_versions.push(v);
        }

        ga_versions.sort();
        let expected = ga_versions.last().cloned();

        let latest = paths.latest_ga_version().unwrap();
        prop_assert_eq!(latest, expected);
    }

    #[test]
    fn latest_ga_ignores_all_prereleases(
        ga_major in 1u32..5,
        ga_minor in 0u32..10,
        ga_patch in 0u32..10,
        pre_major in 5u32..10,
        pre_minor in 0u32..10,
        pre_patch in 0u32..10,
        pre_num in 1u32..10
    ) {
        let (_temp, paths) = setup_temp_paths();
        paths.ensure_dirs().unwrap();

        let ga = Version::new(ga_major, ga_minor, ga_patch);
        let alpha = Version::with_prerelease(pre_major, pre_minor, pre_patch, Prerelease::Alpha(pre_num.to_string()));
        let beta = Version::with_prerelease(pre_major, pre_minor, pre_patch, Prerelease::Beta(pre_num.to_string()));
        let rc = Version::with_prerelease(pre_major, pre_minor, pre_patch, Prerelease::Rc(pre_num.to_string()));

        fs::create_dir_all(paths.version_dir(&ga)).unwrap();
        fs::create_dir_all(paths.version_dir(&alpha)).unwrap();
        fs::create_dir_all(paths.version_dir(&beta)).unwrap();
        fs::create_dir_all(paths.version_dir(&rc)).unwrap();

        let latest = paths.latest_ga_version().unwrap();
        prop_assert_eq!(latest, Some(ga));
    }

    #[test]
    fn latest_alpha_returns_only_alphas(
        ga_major in 1u32..5,
        ga_minor in 0u32..10,
        ga_patch in 0u32..10,
        alpha_major in 5u32..10,
        alpha_minor in 0u32..10,
        alpha_patch in 0u32..10,
        alpha_id in 1u32..100
    ) {
        let (_temp, paths) = setup_temp_paths();
        paths.ensure_dirs().unwrap();

        let ga = Version::new(ga_major, ga_minor, ga_patch);
        let alpha = Version::with_prerelease(alpha_major, alpha_minor, alpha_patch, Prerelease::Alpha(alpha_id.to_string()));
        let beta = Version::with_prerelease(alpha_major, alpha_minor, alpha_patch, Prerelease::Beta("1".to_string()));
        let rc = Version::with_prerelease(alpha_major, alpha_minor, alpha_patch, Prerelease::Rc("1".to_string()));

        fs::create_dir_all(paths.version_dir(&ga)).unwrap();
        fs::create_dir_all(paths.version_dir(&alpha)).unwrap();
        fs::create_dir_all(paths.version_dir(&beta)).unwrap();
        fs::create_dir_all(paths.version_dir(&rc)).unwrap();

        let latest = paths.latest_alpha_version().unwrap();
        prop_assert_eq!(latest, Some(alpha));
    }
}
