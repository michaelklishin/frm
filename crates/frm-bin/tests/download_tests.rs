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

    let conf_path = paths.version_etc_dir(&version).join("rabbitmq.conf");
    assert!(conf_path.exists());
    let content = fs::read_to_string(&conf_path).unwrap();
    assert!(content.contains("listeners.tcp.default"));

    let plugins_path = paths.version_etc_dir(&version).join("enabled_plugins");
    assert!(plugins_path.exists());
    let plugins_content = fs::read_to_string(&plugins_path).unwrap();
    assert!(plugins_content.contains("rabbitmq_management"));

    let logging_conf = paths.version_confd_dir(&version).join("90-logging.conf");
    assert!(logging_conf.exists());
    let logging_content = fs::read_to_string(&logging_conf).unwrap();
    assert!(logging_content.contains("log.file.level"));
    assert!(logging_content.contains("log.console"));
}

#[test]
fn copy_default_config_with_files() {
    let (_temp, paths) = setup_temp_paths();
    paths.ensure_dirs().unwrap();

    let version = Version::new(4, 2, 3);
    fs::create_dir_all(paths.version_dir(&version)).unwrap();

    fs::write(
        paths.etc_dir().join("rabbitmq.conf"),
        "log.console.level = warning\n",
    )
    .unwrap();
    fs::write(paths.etc_dir().join("enabled_plugins"), "[].").unwrap();

    copy_default_config(&paths, &version).unwrap();

    let dest_conf = paths.version_etc_dir(&version).join("rabbitmq.conf");
    let dest_plugins = paths.version_etc_dir(&version).join("enabled_plugins");

    assert!(dest_conf.exists());
    assert!(dest_plugins.exists());
    assert_eq!(
        fs::read_to_string(dest_conf).unwrap(),
        "log.console.level = warning\n"
    );
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

#[test]
fn copy_default_config_user_overrides_template() {
    let (_temp, paths) = setup_temp_paths();
    paths.ensure_dirs().unwrap();

    let version = Version::new(4, 2, 3);
    fs::create_dir_all(paths.version_dir(&version)).unwrap();

    fs::write(
        paths.etc_dir().join("rabbitmq.conf"),
        "cluster_name = rabbit@custom\n",
    )
    .unwrap();

    copy_default_config(&paths, &version).unwrap();

    let conf_path = paths.version_etc_dir(&version).join("rabbitmq.conf");
    assert!(conf_path.exists());
    assert_eq!(
        fs::read_to_string(&conf_path).unwrap(),
        "cluster_name = rabbit@custom\n"
    );
}

#[test]
fn copy_default_config_user_confd_overrides_logging() {
    let (_temp, paths) = setup_temp_paths();
    paths.ensure_dirs().unwrap();

    let version = Version::new(4, 2, 3);
    fs::create_dir_all(paths.version_dir(&version)).unwrap();

    let user_confd = paths.etc_dir().join("conf.d");
    fs::create_dir_all(&user_confd).unwrap();
    fs::write(
        user_confd.join("90-logging.conf"),
        "log.file.level = warning\nlog.console = false\n",
    )
    .unwrap();

    copy_default_config(&paths, &version).unwrap();

    let logging_conf = paths.version_confd_dir(&version).join("90-logging.conf");
    let content = fs::read_to_string(logging_conf).unwrap();
    assert!(content.contains("log.file.level = warning"));
    assert!(content.contains("log.console = false"));
}

#[test]
fn copy_default_config_user_confd_adds_extra_files() {
    let (_temp, paths) = setup_temp_paths();
    paths.ensure_dirs().unwrap();

    let version = Version::new(4, 2, 3);
    fs::create_dir_all(paths.version_dir(&version)).unwrap();

    let user_confd = paths.etc_dir().join("conf.d");
    fs::create_dir_all(&user_confd).unwrap();
    fs::write(
        user_confd.join("50-custom.conf"),
        "cluster_name = my-cluster\n",
    )
    .unwrap();

    copy_default_config(&paths, &version).unwrap();

    let confd_dir = paths.version_confd_dir(&version);
    assert!(confd_dir.join("90-logging.conf").exists());
    assert!(confd_dir.join("50-custom.conf").exists());
    assert_eq!(
        fs::read_to_string(confd_dir.join("50-custom.conf")).unwrap(),
        "cluster_name = my-cluster\n"
    );
}

#[test]
fn copy_default_config_user_overrides_enabled_plugins() {
    let (_temp, paths) = setup_temp_paths();
    paths.ensure_dirs().unwrap();

    let version = Version::new(4, 2, 3);
    fs::create_dir_all(paths.version_dir(&version)).unwrap();

    fs::write(
        paths.etc_dir().join("enabled_plugins"),
        "[rabbitmq_shovel,rabbitmq_federation].",
    )
    .unwrap();

    copy_default_config(&paths, &version).unwrap();

    let plugins_path = paths.version_etc_dir(&version).join("enabled_plugins");
    assert!(plugins_path.exists());
    assert_eq!(
        fs::read_to_string(&plugins_path).unwrap(),
        "[rabbitmq_shovel,rabbitmq_federation]."
    );
}
