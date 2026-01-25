// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;
use std::path::PathBuf;

use flate2::Compression;
use flate2::write::GzEncoder;
use tempfile::TempDir;

use frm::paths::Paths;
use frm::tanzu::{
    CompressionFormat, extract_tarball, extract_version_from_tarball_name, verify_extracted_version,
};
use frm::version::{Prerelease, Version};

fn create_test_tarball_gz(temp_dir: &TempDir, name: &str, inner_dir: &str) -> PathBuf {
    let tarball_path = temp_dir.path().join(name);
    let file = fs::File::create(&tarball_path).unwrap();
    let encoder = GzEncoder::new(file, Compression::default());
    let mut archive = tar::Builder::new(encoder);

    let inner_path = temp_dir.path().join(inner_dir);
    let sbin_path = inner_path.join("sbin");
    fs::create_dir_all(&sbin_path).unwrap();
    fs::write(sbin_path.join("rabbitmqctl"), "#!/bin/bash\necho test\n").unwrap();

    archive.append_dir_all(inner_dir, &inner_path).unwrap();
    archive.finish().unwrap();

    tarball_path
}

#[test]
fn compression_format_from_tar_xz() {
    let path = PathBuf::from("tanzu-rabbitmq-aarch64-4.2.3.tar.xz");
    assert_eq!(
        CompressionFormat::from_path(&path),
        Some(CompressionFormat::Xz)
    );
}

#[test]
fn compression_format_from_tar_gz() {
    let path = PathBuf::from("tanzu-rabbitmq-x86_64-4.2.3.tar.gz");
    assert_eq!(
        CompressionFormat::from_path(&path),
        Some(CompressionFormat::Gzip)
    );
}

#[test]
fn compression_format_from_tgz() {
    let path = PathBuf::from("rabbitmq-server-4.2.3.tgz");
    assert_eq!(
        CompressionFormat::from_path(&path),
        Some(CompressionFormat::Gzip)
    );
}

#[test]
fn compression_format_unsupported() {
    let path = PathBuf::from("archive.zip");
    assert_eq!(CompressionFormat::from_path(&path), None);
}

#[test]
fn compression_format_no_extension() {
    let path = PathBuf::from("archive");
    assert_eq!(CompressionFormat::from_path(&path), None);
}

#[test]
fn extract_version_tanzu_aarch64_ga() {
    let path = PathBuf::from("tanzu-rabbitmq-aarch64-4.2.3.tar.gz");
    let version = extract_version_from_tarball_name(&path).unwrap();
    assert_eq!(version, Version::new(4, 2, 3));
}

#[test]
fn extract_version_tanzu_x86_64_ga() {
    let path = PathBuf::from("tanzu-rabbitmq-x86_64-4.2.3.tar.gz");
    let version = extract_version_from_tarball_name(&path).unwrap();
    assert_eq!(version, Version::new(4, 2, 3));
}

#[test]
fn extract_version_tanzu_aarch64_rc() {
    let path = PathBuf::from("tanzu-rabbitmq-aarch64-4.2.3-rc.1.tar.gz");
    let version = extract_version_from_tarball_name(&path).unwrap();
    assert_eq!(
        version,
        Version::with_prerelease(4, 2, 3, Prerelease::Rc("1".into()))
    );
}

#[test]
fn extract_version_tanzu_x86_64_rc() {
    let path = PathBuf::from("tanzu-rabbitmq-x86_64-4.2.3-rc.1.tar.gz");
    let version = extract_version_from_tarball_name(&path).unwrap();
    assert_eq!(
        version,
        Version::with_prerelease(4, 2, 3, Prerelease::Rc("1".into()))
    );
}

#[test]
fn extract_version_tanzu_beta() {
    let path = PathBuf::from("tanzu-rabbitmq-aarch64-4.2.3-beta.2.tar.gz");
    let version = extract_version_from_tarball_name(&path).unwrap();
    assert_eq!(
        version,
        Version::with_prerelease(4, 2, 3, Prerelease::Beta("2".into()))
    );
}

#[test]
fn extract_version_tanzu_alpha() {
    let path = PathBuf::from("tanzu-rabbitmq-aarch64-4.3.0-alpha.abc123.tar.xz");
    let version = extract_version_from_tarball_name(&path).unwrap();
    assert_eq!(
        version,
        Version::with_prerelease(4, 3, 0, Prerelease::Alpha("abc123".into()))
    );
}

#[test]
fn extract_version_oss_format() {
    let path = PathBuf::from("rabbitmq-server-generic-unix-4.2.3.tar.xz");
    let version = extract_version_from_tarball_name(&path).unwrap();
    assert_eq!(version, Version::new(4, 2, 3));
}

#[test]
fn extract_version_simple_format() {
    let path = PathBuf::from("rabbitmq-4.2.3.tar.gz");
    let version = extract_version_from_tarball_name(&path).unwrap();
    assert_eq!(version, Version::new(4, 2, 3));
}

#[test]
fn extract_version_with_tgz() {
    let path = PathBuf::from("tanzu-rabbitmq-x86_64-4.0.0.tgz");
    let version = extract_version_from_tarball_name(&path).unwrap();
    assert_eq!(version, Version::new(4, 0, 0));
}

#[test]
fn extract_version_no_version() {
    let path = PathBuf::from("random-archive.tar.gz");
    assert!(extract_version_from_tarball_name(&path).is_none());
}

#[test]
fn extract_version_invalid_version_format() {
    let path = PathBuf::from("archive-1.2.tar.gz");
    assert!(extract_version_from_tarball_name(&path).is_none());
}

#[test]
fn extract_version_longer_prefix() {
    let path = PathBuf::from("vmware-tanzu-rabbitmq-enterprise-aarch64-4.2.3-rc.1.tar.gz");
    let version = extract_version_from_tarball_name(&path).unwrap();
    assert_eq!(
        version,
        Version::with_prerelease(4, 2, 3, Prerelease::Rc("1".into()))
    );
}

#[test]
fn extract_tarball_creates_version_dir() {
    let temp = TempDir::new().unwrap();
    let paths = Paths::with_base_dir(temp.path().to_path_buf());
    paths.ensure_dirs().unwrap();

    let tarball = create_test_tarball_gz(&temp, "test.tar.gz", "rabbitmq_server-4.2.3");
    let version = Version::new(4, 2, 3);

    extract_tarball(&tarball, &version, &paths).unwrap();

    assert!(paths.version_dir(&version).exists());
    assert!(paths.version_sbin_dir(&version).exists());
}

#[test]
fn extract_tarball_removes_temp_dir() {
    let temp = TempDir::new().unwrap();
    let paths = Paths::with_base_dir(temp.path().to_path_buf());
    paths.ensure_dirs().unwrap();

    let tarball = create_test_tarball_gz(&temp, "test.tar.gz", "rabbitmq_server-4.2.3");
    let version = Version::new(4, 2, 3);

    extract_tarball(&tarball, &version, &paths).unwrap();

    let temp_extract_dir = paths.versions_dir().join(".4.2.3-extracting");
    assert!(!temp_extract_dir.exists());
}

#[test]
fn extract_tarball_replaces_existing() {
    let temp = TempDir::new().unwrap();
    let paths = Paths::with_base_dir(temp.path().to_path_buf());
    paths.ensure_dirs().unwrap();

    let version = Version::new(4, 2, 3);
    let version_dir = paths.version_dir(&version);
    fs::create_dir_all(&version_dir).unwrap();
    fs::write(version_dir.join("old_marker"), "old").unwrap();

    let tarball = create_test_tarball_gz(&temp, "test.tar.gz", "rabbitmq_server-4.2.3");

    extract_tarball(&tarball, &version, &paths).unwrap();

    assert!(!version_dir.join("old_marker").exists());
    assert!(paths.version_sbin_dir(&version).exists());
}

#[test]
fn extract_tarball_unsupported_format() {
    let temp = TempDir::new().unwrap();
    let paths = Paths::with_base_dir(temp.path().to_path_buf());
    paths.ensure_dirs().unwrap();

    let tarball = temp.path().join("test.zip");
    fs::write(&tarball, "dummy content").unwrap();

    let version = Version::new(4, 2, 3);
    let result = extract_tarball(&tarball, &version, &paths);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("unsupported"));
}

#[test]
fn extract_tarball_with_non_rabbitmq_dir_name() {
    let temp = TempDir::new().unwrap();
    let paths = Paths::with_base_dir(temp.path().to_path_buf());
    paths.ensure_dirs().unwrap();

    let tarball = create_test_tarball_gz(&temp, "test.tar.gz", "tanzu-server-4.2.3");
    let version = Version::new(4, 2, 3);

    extract_tarball(&tarball, &version, &paths).unwrap();

    assert!(paths.version_dir(&version).exists());
    assert!(paths.version_sbin_dir(&version).exists());
}

#[test]
fn extract_version_version_at_start() {
    let path = PathBuf::from("4.2.3.tar.gz");
    let version = extract_version_from_tarball_name(&path).unwrap();
    assert_eq!(version, Version::new(4, 2, 3));
}

#[test]
fn compression_format_case_sensitive() {
    let path = PathBuf::from("archive.TAR.XZ");
    assert_eq!(CompressionFormat::from_path(&path), None);
}

#[test]
fn extract_version_with_numeric_prefix() {
    let path = PathBuf::from("pkg123-4.2.3.tar.gz");
    let version = extract_version_from_tarball_name(&path).unwrap();
    assert_eq!(version, Version::new(4, 2, 3));
}

#[test]
fn verify_extracted_version_success() {
    let temp = TempDir::new().unwrap();
    let paths = Paths::with_base_dir(temp.path().to_path_buf());
    paths.ensure_dirs().unwrap();

    let version = Version::new(4, 2, 3);
    let version_dir = paths.version_dir(&version);
    let sbin_dir = version_dir.join("sbin");
    fs::create_dir_all(&sbin_dir).unwrap();

    let result = verify_extracted_version(&paths, &version);
    assert!(result.is_ok());
}

#[test]
fn verify_extracted_version_missing_dir() {
    let temp = TempDir::new().unwrap();
    let paths = Paths::with_base_dir(temp.path().to_path_buf());
    paths.ensure_dirs().unwrap();

    let version = Version::new(4, 2, 3);
    let result = verify_extracted_version(&paths, &version);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("not found"));
}

#[test]
fn verify_extracted_version_missing_sbin() {
    let temp = TempDir::new().unwrap();
    let paths = Paths::with_base_dir(temp.path().to_path_buf());
    paths.ensure_dirs().unwrap();

    let version = Version::new(4, 2, 3);
    let version_dir = paths.version_dir(&version);
    fs::create_dir_all(&version_dir).unwrap();

    let result = verify_extracted_version(&paths, &version);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("sbin"));
}
