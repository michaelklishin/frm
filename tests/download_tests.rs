// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;

use tempfile::TempDir;

use frm::download::copy_default_config;
use frm::paths::Paths;
use frm::version::Version;

fn setup_temp_paths() -> (TempDir, Paths) {
    let temp_dir = TempDir::new().unwrap();
    let paths = Paths::with_base_dir(temp_dir.path().to_path_buf());
    (temp_dir, paths)
}

#[test]
fn copy_default_config_no_source() {
    let (_temp, paths) = setup_temp_paths();
    paths.ensure_dirs().unwrap();

    let version = Version::new(4, 2, 3);
    fs::create_dir_all(paths.version_dir(&version)).unwrap();

    let result = copy_default_config(&paths, &version);
    assert!(result.is_ok());
}

#[test]
fn copy_default_config_with_files() {
    let (_temp, paths) = setup_temp_paths();
    paths.ensure_dirs().unwrap();

    let version = Version::new(4, 2, 3);
    fs::create_dir_all(paths.version_dir(&version)).unwrap();

    fs::write(paths.etc_dir().join("rabbitmq.conf"), "test config").unwrap();
    fs::write(paths.etc_dir().join("enabled_plugins"), "[].").unwrap();

    copy_default_config(&paths, &version).unwrap();

    let dest_conf = paths.version_etc_dir(&version).join("rabbitmq.conf");
    let dest_plugins = paths.version_etc_dir(&version).join("enabled_plugins");

    assert!(dest_conf.exists());
    assert!(dest_plugins.exists());
    assert_eq!(fs::read_to_string(dest_conf).unwrap(), "test config");
    assert_eq!(fs::read_to_string(dest_plugins).unwrap(), "[].");
}

#[test]
fn copy_default_config_with_nested_dirs() {
    let (_temp, paths) = setup_temp_paths();
    paths.ensure_dirs().unwrap();

    let version = Version::new(4, 2, 3);
    fs::create_dir_all(paths.version_dir(&version)).unwrap();

    let nested_dir = paths.etc_dir().join("certs");
    fs::create_dir_all(&nested_dir).unwrap();
    fs::write(nested_dir.join("ca.pem"), "CA CERT").unwrap();
    fs::write(nested_dir.join("server.pem"), "SERVER CERT").unwrap();

    copy_default_config(&paths, &version).unwrap();

    let dest_certs = paths.version_etc_dir(&version).join("certs");
    assert!(dest_certs.exists());
    assert!(dest_certs.is_dir());
    assert_eq!(
        fs::read_to_string(dest_certs.join("ca.pem")).unwrap(),
        "CA CERT"
    );
    assert_eq!(
        fs::read_to_string(dest_certs.join("server.pem")).unwrap(),
        "SERVER CERT"
    );
}

#[test]
fn copy_default_config_deeply_nested() {
    let (_temp, paths) = setup_temp_paths();
    paths.ensure_dirs().unwrap();

    let version = Version::new(4, 2, 3);
    fs::create_dir_all(paths.version_dir(&version)).unwrap();

    let deep_dir = paths.etc_dir().join("ssl").join("certs").join("private");
    fs::create_dir_all(&deep_dir).unwrap();
    fs::write(deep_dir.join("key.pem"), "PRIVATE KEY").unwrap();

    copy_default_config(&paths, &version).unwrap();

    let dest_key = paths
        .version_etc_dir(&version)
        .join("ssl")
        .join("certs")
        .join("private")
        .join("key.pem");
    assert!(dest_key.exists());
    assert_eq!(fs::read_to_string(dest_key).unwrap(), "PRIVATE KEY");
}

#[test]
fn copy_default_config_mixed_files_and_dirs() {
    let (_temp, paths) = setup_temp_paths();
    paths.ensure_dirs().unwrap();

    let version = Version::new(4, 2, 3);
    fs::create_dir_all(paths.version_dir(&version)).unwrap();

    fs::write(paths.etc_dir().join("rabbitmq.conf"), "config").unwrap();
    let certs_dir = paths.etc_dir().join("certs");
    fs::create_dir_all(&certs_dir).unwrap();
    fs::write(certs_dir.join("ca.pem"), "CA").unwrap();

    copy_default_config(&paths, &version).unwrap();

    let dest_etc = paths.version_etc_dir(&version);
    assert!(dest_etc.join("rabbitmq.conf").exists());
    assert!(dest_etc.join("certs").is_dir());
    assert!(dest_etc.join("certs").join("ca.pem").exists());
}
