// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::path::PathBuf;

use proptest::prelude::*;

use frm::tanzu::{CompressionFormat, extract_version_from_tarball_name};
use frm::version::{Prerelease, Version};

fn arb_version() -> impl Strategy<Value = Version> {
    (1u32..100, 0u32..100, 0u32..100)
        .prop_map(|(major, minor, patch)| Version::new(major, minor, patch))
}

fn arb_version_with_prerelease() -> impl Strategy<Value = Version> {
    (1u32..100, 0u32..100, 0u32..100, 0usize..4, 1u32..20).prop_map(
        |(major, minor, patch, pre_type, pre_num)| {
            let prerelease = match pre_type {
                0 => None,
                1 => Some(Prerelease::Alpha(pre_num.to_string())),
                2 => Some(Prerelease::Beta(pre_num.to_string())),
                _ => Some(Prerelease::Rc(pre_num.to_string())),
            };
            match prerelease {
                Some(p) => Version::with_prerelease(major, minor, patch, p),
                None => Version::new(major, minor, patch),
            }
        },
    )
}

fn arb_architecture() -> impl Strategy<Value = &'static str> {
    prop_oneof![
        Just("aarch64"),
        Just("x86_64"),
        Just("arm64"),
        Just("amd64"),
    ]
}

fn arb_extension() -> impl Strategy<Value = &'static str> {
    prop_oneof![Just(".tar.xz"), Just(".tar.gz"), Just(".tgz"),]
}

proptest! {
    #[test]
    fn compression_format_consistent_tar_xz(name in "[a-z]+-[a-z]+-[0-9]+\\.[0-9]+\\.[0-9]+\\.tar\\.xz") {
        let path = PathBuf::from(&name);
        let format = CompressionFormat::from_path(&path);
        prop_assert_eq!(format, Some(CompressionFormat::Xz));
    }

    #[test]
    fn compression_format_consistent_tar_gz(name in "[a-z]+-[a-z]+-[0-9]+\\.[0-9]+\\.[0-9]+\\.tar\\.gz") {
        let path = PathBuf::from(&name);
        let format = CompressionFormat::from_path(&path);
        prop_assert_eq!(format, Some(CompressionFormat::Gzip));
    }

    #[test]
    fn compression_format_consistent_tgz(name in "[a-z]+-[a-z]+-[0-9]+\\.[0-9]+\\.[0-9]+\\.tgz") {
        let path = PathBuf::from(&name);
        let format = CompressionFormat::from_path(&path);
        prop_assert_eq!(format, Some(CompressionFormat::Gzip));
    }

    #[test]
    fn compression_format_none_for_other(ext in "[a-z]{3,5}") {
        if ext != "tgz" {
            let name = format!("archive.{}", ext);
            let path = PathBuf::from(&name);
            let format = CompressionFormat::from_path(&path);
            prop_assert_eq!(format, None);
        }
    }

    #[test]
    fn extract_version_tanzu_format_ga(version in arb_version(), arch in arb_architecture(), ext in arb_extension()) {
        let filename = format!("tanzu-rabbitmq-{}-{}{}", arch, version, ext);
        let path = PathBuf::from(&filename);
        let extracted = extract_version_from_tarball_name(&path);
        prop_assert_eq!(extracted, Some(version));
    }

    #[test]
    fn extract_version_tanzu_format_with_prerelease(version in arb_version_with_prerelease(), arch in arb_architecture(), ext in arb_extension()) {
        let filename = format!("tanzu-rabbitmq-{}-{}{}", arch, version, ext);
        let path = PathBuf::from(&filename);
        let extracted = extract_version_from_tarball_name(&path);
        prop_assert_eq!(extracted, Some(version));
    }

    #[test]
    fn extract_version_oss_format(version in arb_version()) {
        let filename = format!("rabbitmq-server-generic-unix-{}.tar.xz", version);
        let path = PathBuf::from(&filename);
        let extracted = extract_version_from_tarball_name(&path);
        prop_assert_eq!(extracted, Some(version));
    }

    #[test]
    fn extract_version_longer_prefix(version in arb_version_with_prerelease(), arch in arb_architecture(), ext in arb_extension()) {
        let filename = format!("vmware-tanzu-enterprise-rabbitmq-{}-{}{}", arch, version, ext);
        let path = PathBuf::from(&filename);
        let extracted = extract_version_from_tarball_name(&path);
        prop_assert_eq!(extracted, Some(version));
    }

    #[test]
    fn extract_version_simple_format(version in arb_version()) {
        let filename = format!("rabbitmq-{}.tar.gz", version);
        let path = PathBuf::from(&filename);
        let extracted = extract_version_from_tarball_name(&path);
        prop_assert_eq!(extracted, Some(version));
    }
}
