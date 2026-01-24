// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proptest::prelude::*;

use frm::version::{Prerelease, Version};

proptest! {
    #![proptest_config(ProptestConfig::with_cases(200))]

    #[test]
    fn version_roundtrip(major in 0u32..100, minor in 0u32..100, patch in 0u32..100) {
        let version = Version::new(major, minor, patch);
        let s = version.to_string();
        let parsed: Version = s.parse().unwrap();
        prop_assert_eq!(version, parsed);
    }

    #[test]
    fn version_parse_with_v_prefix(major in 0u32..100, minor in 0u32..100, patch in 0u32..100) {
        let s = format!("v{}.{}.{}", major, minor, patch);
        let parsed: Version = s.parse().unwrap();
        prop_assert_eq!(parsed.major, major);
        prop_assert_eq!(parsed.minor, minor);
        prop_assert_eq!(parsed.patch, patch);
    }

    #[test]
    fn version_ordering_transitive(
        a_major in 0u32..10, a_minor in 0u32..10, a_patch in 0u32..10,
        b_major in 0u32..10, b_minor in 0u32..10, b_patch in 0u32..10,
        c_major in 0u32..10, c_minor in 0u32..10, c_patch in 0u32..10
    ) {
        let a = Version::new(a_major, a_minor, a_patch);
        let b = Version::new(b_major, b_minor, b_patch);
        let c = Version::new(c_major, c_minor, c_patch);

        if a < b && b < c {
            prop_assert!(a < c);
        }
    }

    #[test]
    fn version_ordering_antisymmetric(
        a_major in 0u32..100, a_minor in 0u32..100, a_patch in 0u32..100,
        b_major in 0u32..100, b_minor in 0u32..100, b_patch in 0u32..100
    ) {
        let a = Version::new(a_major, a_minor, a_patch);
        let b = Version::new(b_major, b_minor, b_patch);

        if a < b {
            prop_assert!(!(b < a));
        }
    }

    #[test]
    fn version_equality_reflexive(major in 0u32..100, minor in 0u32..100, patch in 0u32..100) {
        let v = Version::new(major, minor, patch);
        prop_assert_eq!(v.clone(), v);
    }

    #[test]
    fn version_dir_name_matches_display(major in 0u32..100, minor in 0u32..100, patch in 0u32..100) {
        let v = Version::new(major, minor, patch);
        prop_assert_eq!(v.dir_name(), v.to_string());
    }

    #[test]
    fn version_download_url_contains_version(major in 1u32..10, minor in 0u32..20, patch in 0u32..20) {
        let v = Version::new(major, minor, patch);
        let url = v.download_url();
        prop_assert!(url.contains(&v.to_string()));
    }

    #[test]
    fn version_archive_name_contains_version(major in 1u32..10, minor in 0u32..20, patch in 0u32..20) {
        let v = Version::new(major, minor, patch);
        let name = v.archive_name();
        prop_assert!(name.contains(&v.to_string()));
        prop_assert!(name.ends_with(".tar.xz"));
    }

    #[test]
    fn version_extracted_dir_contains_version(major in 1u32..10, minor in 0u32..20, patch in 0u32..20) {
        let v = Version::new(major, minor, patch);
        let name = v.extracted_dir_name();
        prop_assert!(name.contains(&v.to_string()));
        prop_assert!(name.starts_with("rabbitmq_server-"));
    }

    #[test]
    fn version_major_determines_ordering(major1 in 0u32..100, major2 in 0u32..100, minor in 0u32..100, patch in 0u32..100) {
        let v1 = Version::new(major1, minor, patch);
        let v2 = Version::new(major2, minor, patch);

        if major1 < major2 {
            prop_assert!(v1 < v2);
        } else if major1 > major2 {
            prop_assert!(v1 > v2);
        } else {
            prop_assert_eq!(v1, v2);
        }
    }

    #[test]
    fn prerelease_roundtrip_alpha(major in 1u32..10, minor in 0u32..20, patch in 0u32..20, pre_num in 1u32..10) {
        let version = Version::with_prerelease(major, minor, patch, Prerelease::Alpha(pre_num));
        let s = version.to_string();
        let parsed: Version = s.parse().unwrap();
        prop_assert_eq!(version, parsed);
    }

    #[test]
    fn prerelease_roundtrip_beta(major in 1u32..10, minor in 0u32..20, patch in 0u32..20, pre_num in 1u32..10) {
        let version = Version::with_prerelease(major, minor, patch, Prerelease::Beta(pre_num));
        let s = version.to_string();
        let parsed: Version = s.parse().unwrap();
        prop_assert_eq!(version, parsed);
    }

    #[test]
    fn prerelease_roundtrip_rc(major in 1u32..10, minor in 0u32..20, patch in 0u32..20, pre_num in 1u32..10) {
        let version = Version::with_prerelease(major, minor, patch, Prerelease::Rc(pre_num));
        let s = version.to_string();
        let parsed: Version = s.parse().unwrap();
        prop_assert_eq!(version, parsed);
    }

    #[test]
    fn prerelease_less_than_release(major in 1u32..10, minor in 0u32..20, patch in 0u32..20, pre_num in 1u32..10) {
        let prerelease = Version::with_prerelease(major, minor, patch, Prerelease::Rc(pre_num));
        let release = Version::new(major, minor, patch);
        prop_assert!(prerelease < release);
    }
}
