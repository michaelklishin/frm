// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashSet;

use frm::version::{Prerelease, Version};

#[test]
fn parse_valid_version() {
    let v = "4.2.3".parse::<Version>().unwrap();
    assert_eq!(v.major, 4);
    assert_eq!(v.minor, 2);
    assert_eq!(v.patch, 3);
}

#[test]
fn parse_version_with_v_prefix() {
    let v = "v4.2.3".parse::<Version>().unwrap();
    assert_eq!(v.major, 4);
    assert_eq!(v.minor, 2);
    assert_eq!(v.patch, 3);
}

#[test]
fn parse_version_with_whitespace() {
    let v = "  4.2.3  ".parse::<Version>().unwrap();
    assert_eq!(v.major, 4);
    assert_eq!(v.minor, 2);
    assert_eq!(v.patch, 3);
}

#[test]
fn parse_invalid_version_missing_patch() {
    let result = "4.2".parse::<Version>();
    assert!(result.is_err());
}

#[test]
fn parse_invalid_version_empty() {
    let result = "".parse::<Version>();
    assert!(result.is_err());
}

#[test]
fn parse_invalid_version_non_numeric() {
    let result = "4.2.x".parse::<Version>();
    assert!(result.is_err());
}

#[test]
fn parse_invalid_version_too_many_parts() {
    let result = "4.2.3.4".parse::<Version>();
    assert!(result.is_err());
}

#[test]
fn parse_invalid_version_negative() {
    let result = "-4.2.3".parse::<Version>();
    assert!(result.is_err());
}

#[test]
fn version_display() {
    let v = Version::new(4, 2, 3);
    assert_eq!(v.to_string(), "4.2.3");
}

#[test]
fn version_dir_name() {
    let v = Version::new(4, 2, 3);
    assert_eq!(v.dir_name(), "4.2.3");
}

#[test]
fn version_download_url() {
    let v = Version::new(4, 2, 3);
    let url = v.download_url();
    assert!(url.contains("4.2.3"));
    assert!(url.contains("rabbitmq-server-generic-unix"));
    assert!(url.ends_with(".tar.xz"));
}

#[test]
fn version_archive_name() {
    let v = Version::new(4, 2, 3);
    assert_eq!(
        v.archive_name(),
        "rabbitmq-server-generic-unix-4.2.3.tar.xz"
    );
}

#[test]
fn version_extracted_dir_name() {
    let v = Version::new(4, 2, 3);
    assert_eq!(v.extracted_dir_name(), "rabbitmq_server-4.2.3");
}

#[test]
fn version_ordering() {
    let v1 = Version::new(3, 13, 0);
    let v2 = Version::new(4, 0, 0);
    let v3 = Version::new(4, 2, 3);
    let v4 = Version::new(4, 2, 4);

    assert!(v1 < v2);
    assert!(v2 < v3);
    assert!(v3 < v4);
    assert!(v4 > v1);
}

#[test]
fn version_equality() {
    let v1 = Version::new(4, 2, 3);
    let v2 = Version::new(4, 2, 3);
    let v3 = Version::new(4, 2, 4);

    assert_eq!(v1, v2);
    assert_ne!(v1, v3);
}

#[test]
fn version_sorting() {
    let mut versions = vec![
        Version::new(4, 2, 3),
        Version::new(3, 13, 0),
        Version::new(4, 0, 0),
        Version::new(4, 1, 8),
    ];

    versions.sort();

    assert_eq!(versions[0], Version::new(3, 13, 0));
    assert_eq!(versions[1], Version::new(4, 0, 0));
    assert_eq!(versions[2], Version::new(4, 1, 8));
    assert_eq!(versions[3], Version::new(4, 2, 3));
}

#[test]
fn parse_prerelease_alpha() {
    let v = "4.2.4-alpha.2".parse::<Version>().unwrap();
    assert_eq!(v.major, 4);
    assert_eq!(v.minor, 2);
    assert_eq!(v.patch, 4);
    assert_eq!(v.prerelease, Some(Prerelease::Alpha("2".into())));
}

#[test]
fn parse_prerelease_beta() {
    let v = "4.2.4-beta.1".parse::<Version>().unwrap();
    assert_eq!(v.major, 4);
    assert_eq!(v.minor, 2);
    assert_eq!(v.patch, 4);
    assert_eq!(v.prerelease, Some(Prerelease::Beta("1".into())));
}

#[test]
fn parse_prerelease_rc() {
    let v = "4.2.4-rc.1".parse::<Version>().unwrap();
    assert_eq!(v.major, 4);
    assert_eq!(v.minor, 2);
    assert_eq!(v.patch, 4);
    assert_eq!(v.prerelease, Some(Prerelease::Rc("1".into())));
}

#[test]
fn parse_prerelease_with_v_prefix() {
    let v = "v4.2.4-alpha.1".parse::<Version>().unwrap();
    assert_eq!(v.prerelease, Some(Prerelease::Alpha("1".into())));
}

#[test]
fn prerelease_display() {
    let v = Version::with_prerelease(4, 2, 4, Prerelease::Alpha("2".into()));
    assert_eq!(v.to_string(), "4.2.4-alpha.2");

    let v = Version::with_prerelease(4, 2, 4, Prerelease::Beta("1".into()));
    assert_eq!(v.to_string(), "4.2.4-beta.1");

    let v = Version::with_prerelease(4, 2, 4, Prerelease::Rc("1".into()));
    assert_eq!(v.to_string(), "4.2.4-rc.1");
}

#[test]
fn prerelease_ordering() {
    let alpha1 = Version::with_prerelease(4, 2, 4, Prerelease::Alpha("1".into()));
    let alpha2 = Version::with_prerelease(4, 2, 4, Prerelease::Alpha("2".into()));
    let beta1 = Version::with_prerelease(4, 2, 4, Prerelease::Beta("1".into()));
    let rc1 = Version::with_prerelease(4, 2, 4, Prerelease::Rc("1".into()));
    let release = Version::new(4, 2, 4);

    assert!(alpha1 < alpha2);
    assert!(alpha2 < beta1);
    assert!(beta1 < rc1);
    assert!(rc1 < release);
}

#[test]
fn prerelease_sorting() {
    let mut versions = vec![
        Version::new(4, 2, 4),
        Version::with_prerelease(4, 2, 4, Prerelease::Beta("1".into())),
        Version::with_prerelease(4, 2, 4, Prerelease::Alpha("1".into())),
        Version::with_prerelease(4, 2, 4, Prerelease::Rc("1".into())),
        Version::new(4, 2, 3),
    ];

    versions.sort();

    assert_eq!(versions[0], Version::new(4, 2, 3));
    assert_eq!(versions[1].prerelease, Some(Prerelease::Alpha("1".into())));
    assert_eq!(versions[2].prerelease, Some(Prerelease::Beta("1".into())));
    assert_eq!(versions[3].prerelease, Some(Prerelease::Rc("1".into())));
    assert_eq!(versions[4], Version::new(4, 2, 4));
}

#[test]
fn prerelease_download_url() {
    let v = Version::with_prerelease(4, 2, 4, Prerelease::Alpha("2".into()));
    let url = v.download_url();
    assert!(url.contains("4.2.4-alpha.2"));
}

#[test]
fn parse_invalid_prerelease_type() {
    let result = "4.2.4-gamma.1".parse::<Version>();
    assert!(result.is_err());
}

#[test]
fn parse_invalid_prerelease_format() {
    let result = "4.2.4-alpha".parse::<Version>();
    assert!(result.is_err());
}

#[test]
fn prerelease_alpha_display() {
    assert_eq!(Prerelease::Alpha("1".into()).to_string(), "alpha.1");
    assert_eq!(Prerelease::Alpha("10".into()).to_string(), "alpha.10");
}

#[test]
fn prerelease_beta_display() {
    assert_eq!(Prerelease::Beta("1".into()).to_string(), "beta.1");
    assert_eq!(Prerelease::Beta("5".into()).to_string(), "beta.5");
}

#[test]
fn prerelease_rc_display() {
    assert_eq!(Prerelease::Rc("1".into()).to_string(), "rc.1");
    assert_eq!(Prerelease::Rc("3".into()).to_string(), "rc.3");
}

#[test]
fn prerelease_direct_ordering() {
    assert!(Prerelease::Alpha("1".into()) < Prerelease::Alpha("2".into()));
    assert!(Prerelease::Alpha("2".into()) < Prerelease::Beta("1".into()));
    assert!(Prerelease::Beta("1".into()) < Prerelease::Beta("2".into()));
    assert!(Prerelease::Beta("2".into()) < Prerelease::Rc("1".into()));
    assert!(Prerelease::Rc("1".into()) < Prerelease::Rc("2".into()));
}

#[test]
fn prerelease_equality() {
    assert_eq!(Prerelease::Alpha("1".into()), Prerelease::Alpha("1".into()));
    assert_ne!(Prerelease::Alpha("1".into()), Prerelease::Alpha("2".into()));
    assert_ne!(Prerelease::Alpha("1".into()), Prerelease::Beta("1".into()));
}

#[test]
fn version_hash_consistency() {
    let v1 = Version::new(4, 2, 3);
    let v2 = Version::new(4, 2, 3);
    let v3 = Version::new(4, 2, 4);

    let mut set = HashSet::new();
    set.insert(v1.clone());

    assert!(set.contains(&v2));
    assert!(!set.contains(&v3));
}

#[test]
fn version_clone() {
    let v1 = Version::new(4, 2, 3);
    let v2 = v1.clone();
    assert_eq!(v1, v2);
}

#[test]
fn prerelease_clone() {
    let p1 = Prerelease::Alpha("1".into());
    let p2 = p1.clone();
    assert_eq!(p1, p2);
}

#[test]
fn version_zero() {
    let v = "0.0.0".parse::<Version>().unwrap();
    assert_eq!(v.major, 0);
    assert_eq!(v.minor, 0);
    assert_eq!(v.patch, 0);
}

#[test]
fn version_large_numbers() {
    let v = "999.888.777".parse::<Version>().unwrap();
    assert_eq!(v.major, 999);
    assert_eq!(v.minor, 888);
    assert_eq!(v.patch, 777);
}

#[test]
fn prerelease_zero_number() {
    let v = "4.2.4-alpha.0".parse::<Version>().unwrap();
    assert_eq!(v.prerelease, Some(Prerelease::Alpha("0".into())));
}

#[test]
fn parse_invalid_prerelease_empty_number() {
    let result = "4.2.4-alpha.".parse::<Version>();
    assert!(result.is_err());
}

#[test]
fn parse_prerelease_commit_hash() {
    let v = "4.3.0-alpha.132057c7".parse::<Version>().unwrap();
    assert_eq!(v.major, 4);
    assert_eq!(v.minor, 3);
    assert_eq!(v.patch, 0);
    assert_eq!(v.prerelease, Some(Prerelease::Alpha("132057c7".into())));
}

#[test]
fn alpha_is_server_packages_release() {
    let v = "4.3.0-alpha.1".parse::<Version>().unwrap();
    assert!(v.is_server_packages_release());

    let v = "4.3.0-alpha.132057c7".parse::<Version>().unwrap();
    assert!(v.is_server_packages_release());
}

#[test]
fn beta_is_not_server_packages_release() {
    let v = "4.3.0-beta.1".parse::<Version>().unwrap();
    assert!(!v.is_server_packages_release());
}

#[test]
fn rc_is_not_server_packages_release() {
    let v = "4.3.0-rc.1".parse::<Version>().unwrap();
    assert!(!v.is_server_packages_release());
}

#[test]
fn release_is_not_server_packages_release() {
    let v = "4.3.0".parse::<Version>().unwrap();
    assert!(!v.is_server_packages_release());
}

#[test]
fn prerelease_is_alpha() {
    let alpha = Prerelease::Alpha("1".into());
    let beta = Prerelease::Beta("1".into());
    let rc = Prerelease::Rc("1".into());

    assert!(alpha.is_alpha());
    assert!(!beta.is_alpha());
    assert!(!rc.is_alpha());
}

#[test]
fn parse_invalid_double_dash() {
    let result = "4.2.4--alpha.1".parse::<Version>();
    assert!(result.is_err());
}

#[test]
fn parse_prerelease_case_insensitive() {
    let v1 = "4.2.4-ALPHA.1".parse::<Version>().unwrap();
    let v2 = "4.2.4-Alpha.1".parse::<Version>().unwrap();
    let v3 = "4.2.4-alpha.1".parse::<Version>().unwrap();

    assert_eq!(v1, v2);
    assert_eq!(v2, v3);
}
