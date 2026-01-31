// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;

use assert_cmd::Command;
use flate2::Compression;
use flate2::write::GzEncoder;
use predicates::prelude::*;
use tempfile::TempDir;

#[allow(deprecated)]
fn frm_cmd() -> Command {
    Command::cargo_bin("frm").unwrap()
}

#[allow(deprecated)]
fn frm_cmd_with_dir(dir: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("frm").unwrap();
    cmd.env("FRM_DIR", dir.path());
    cmd
}

fn create_test_tarball(temp_dir: &TempDir, name: &str, inner_dir: &str) -> std::path::PathBuf {
    let tarball_path = temp_dir.path().join(name);
    let file = fs::File::create(&tarball_path).unwrap();
    let encoder = GzEncoder::new(file, Compression::default());
    let mut archive = tar::Builder::new(encoder);

    let inner_path = temp_dir.path().join("tarball_content").join(inner_dir);
    let sbin_path = inner_path.join("sbin");
    fs::create_dir_all(&sbin_path).unwrap();
    fs::write(sbin_path.join("rabbitmqctl"), "#!/bin/bash\necho test\n").unwrap();
    fs::write(
        sbin_path.join("rabbitmq-server"),
        "#!/bin/bash\necho server\n",
    )
    .unwrap();

    archive.append_dir_all(inner_dir, &inner_path).unwrap();
    archive.finish().unwrap();

    tarball_path
}

#[test]
fn cli_tanzu_help() {
    frm_cmd()
        .args(["tanzu", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Install Tanzu RabbitMQ"));
}

#[test]
fn cli_tanzu_no_subcommand() {
    frm_cmd()
        .arg("tanzu")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn cli_tanzu_install_help() {
    frm_cmd()
        .args(["tanzu", "install", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "--local-tanzu-rabbitmq-tarball-path",
        ))
        .stdout(predicate::str::contains("--version"))
        .stdout(predicate::str::contains(".tar.xz"))
        .stdout(predicate::str::contains(".tar.gz"))
        .stdout(predicate::str::contains(".tgz"));
}

#[test]
fn cli_tanzu_install_missing_tarball_path() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["tanzu", "install", "--version", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "--local-tanzu-rabbitmq-tarball-path",
        ));
}

#[test]
fn cli_tanzu_install_missing_version() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args([
            "tanzu",
            "install",
            "--local-tanzu-rabbitmq-tarball-path",
            "/tmp/dummy.tar.gz",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--version"));
}

#[test]
fn cli_tanzu_install_file_not_found() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args([
            "tanzu",
            "install",
            "--local-tanzu-rabbitmq-tarball-path",
            "/nonexistent/path/tanzu-rabbitmq-4.2.3.tar.gz",
            "--version",
            "4.2.3",
        ])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("file not found").or(predicate::str::contains("not found")),
        );
}

#[test]
fn cli_tanzu_install_invalid_version() {
    let temp = TempDir::new().unwrap();
    let tarball = create_test_tarball(&temp, "test.tar.gz", "rabbitmq_server-4.2.3");

    frm_cmd_with_dir(&temp)
        .args([
            "tanzu",
            "install",
            "--local-tanzu-rabbitmq-tarball-path",
            tarball.to_str().unwrap(),
            "--version",
            "invalid",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid version"));
}

#[test]
fn cli_tanzu_install_version_mismatch() {
    let temp = TempDir::new().unwrap();
    let tarball = create_test_tarball(
        &temp,
        "tanzu-rabbitmq-aarch64-4.2.3.tar.gz",
        "rabbitmq_server-4.2.3",
    );

    frm_cmd_with_dir(&temp)
        .args([
            "tanzu",
            "install",
            "--local-tanzu-rabbitmq-tarball-path",
            tarball.to_str().unwrap(),
            "--version",
            "4.2.4",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("mismatch"));
}

#[test]
fn cli_tanzu_install_success() {
    let temp = TempDir::new().unwrap();
    let tarball = create_test_tarball(
        &temp,
        "tanzu-rabbitmq-aarch64-4.2.3.tar.gz",
        "rabbitmq_server-4.2.3",
    );

    frm_cmd_with_dir(&temp)
        .args([
            "tanzu",
            "install",
            "--local-tanzu-rabbitmq-tarball-path",
            tarball.to_str().unwrap(),
            "--version",
            "4.2.3",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("installed successfully"));

    let version_dir = temp.path().join("versions").join("4.2.3");
    assert!(version_dir.exists());
    assert!(version_dir.join("sbin").exists());
}

#[test]
fn cli_tanzu_install_success_with_rc() {
    let temp = TempDir::new().unwrap();
    let tarball = create_test_tarball(
        &temp,
        "tanzu-rabbitmq-x86_64-4.2.3-rc.1.tar.gz",
        "rabbitmq_server-4.2.3-rc.1",
    );

    frm_cmd_with_dir(&temp)
        .args([
            "tanzu",
            "install",
            "--local-tanzu-rabbitmq-tarball-path",
            tarball.to_str().unwrap(),
            "--version",
            "4.2.3-rc.1",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.2.3-rc.1"))
        .stdout(predicate::str::contains("installed successfully"));

    let version_dir = temp.path().join("versions").join("4.2.3-rc.1");
    assert!(version_dir.exists());
}

#[test]
fn cli_tanzu_install_already_installed() {
    let temp = TempDir::new().unwrap();
    let tarball = create_test_tarball(
        &temp,
        "tanzu-rabbitmq-aarch64-4.2.3.tar.gz",
        "rabbitmq_server-4.2.3",
    );

    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "tanzu",
            "install",
            "--local-tanzu-rabbitmq-tarball-path",
            tarball.to_str().unwrap(),
            "--version",
            "4.2.3",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already installed"));
}

#[test]
fn cli_tanzu_install_force_reinstall() {
    let temp = TempDir::new().unwrap();
    let tarball = create_test_tarball(
        &temp,
        "tanzu-rabbitmq-aarch64-4.2.3.tar.gz",
        "rabbitmq_server-4.2.3",
    );

    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();
    fs::write(version_dir.join("old_marker"), "old").unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "tanzu",
            "install",
            "--local-tanzu-rabbitmq-tarball-path",
            tarball.to_str().unwrap(),
            "--version",
            "4.2.3",
            "--force",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removing existing"))
        .stdout(predicate::str::contains("installed successfully"));

    assert!(!version_dir.join("old_marker").exists());
    assert!(version_dir.join("sbin").exists());
}

#[test]
fn cli_tanzu_install_alias() {
    let temp = TempDir::new().unwrap();
    let tarball = create_test_tarball(
        &temp,
        "tanzu-rabbitmq-aarch64-4.2.3.tar.gz",
        "rabbitmq_server-4.2.3",
    );

    frm_cmd_with_dir(&temp)
        .args([
            "tanzu",
            "i",
            "--local-tanzu-rabbitmq-tarball-path",
            tarball.to_str().unwrap(),
            "--version",
            "4.2.3",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("installed successfully"));
}

#[test]
fn cli_tanzu_install_copies_config() {
    let temp = TempDir::new().unwrap();
    let tarball = create_test_tarball(
        &temp,
        "tanzu-rabbitmq-aarch64-4.2.3.tar.gz",
        "rabbitmq_server-4.2.3",
    );

    frm_cmd_with_dir(&temp)
        .args([
            "tanzu",
            "install",
            "--local-tanzu-rabbitmq-tarball-path",
            tarball.to_str().unwrap(),
            "--version",
            "4.2.3",
        ])
        .assert()
        .success();

    let config_path = temp
        .path()
        .join("versions")
        .join("4.2.3")
        .join("etc")
        .join("rabbitmq")
        .join("rabbitmq.conf");
    assert!(config_path.exists());
}

#[test]
fn cli_tanzu_install_records_timestamp() {
    let temp = TempDir::new().unwrap();
    let tarball = create_test_tarball(
        &temp,
        "tanzu-rabbitmq-aarch64-4.2.3.tar.gz",
        "rabbitmq_server-4.2.3",
    );

    frm_cmd_with_dir(&temp)
        .args([
            "tanzu",
            "install",
            "--local-tanzu-rabbitmq-tarball-path",
            tarball.to_str().unwrap(),
            "--version",
            "4.2.3",
        ])
        .assert()
        .success();

    let timestamps_path = temp.path().join("version_timestamps.json");
    assert!(timestamps_path.exists());
    let content = fs::read_to_string(&timestamps_path).unwrap();
    assert!(content.contains("4.2.3"));
}

#[test]
fn cli_tanzu_use_installed() {
    let temp = TempDir::new().unwrap();
    let tarball = create_test_tarball(
        &temp,
        "tanzu-rabbitmq-aarch64-4.2.3.tar.gz",
        "rabbitmq_server-4.2.3",
    );

    frm_cmd_with_dir(&temp)
        .args([
            "tanzu",
            "install",
            "--local-tanzu-rabbitmq-tarball-path",
            tarball.to_str().unwrap(),
            "--version",
            "4.2.3",
        ])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["tanzu", "use", "4.2.3", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("export PATH="))
        .stdout(predicate::str::contains("4.2.3"));
}

#[test]
fn cli_tanzu_use_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["tanzu", "use", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_tanzu_use_latest() {
    let temp = TempDir::new().unwrap();
    let tarball = create_test_tarball(
        &temp,
        "tanzu-rabbitmq-aarch64-4.2.3.tar.gz",
        "rabbitmq_server-4.2.3",
    );

    frm_cmd_with_dir(&temp)
        .args([
            "tanzu",
            "install",
            "--local-tanzu-rabbitmq-tarball-path",
            tarball.to_str().unwrap(),
            "--version",
            "4.2.3",
        ])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["tanzu", "use", "latest", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.2.3"));
}

#[test]
fn cli_tanzu_use_help() {
    frm_cmd()
        .args(["tanzu", "use", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("latest"));
}

#[test]
fn cli_tanzu_install_can_set_as_default() {
    let temp = TempDir::new().unwrap();
    let tarball = create_test_tarball(
        &temp,
        "tanzu-rabbitmq-aarch64-4.2.3.tar.gz",
        "rabbitmq_server-4.2.3",
    );

    frm_cmd_with_dir(&temp)
        .args([
            "tanzu",
            "install",
            "--local-tanzu-rabbitmq-tarball-path",
            tarball.to_str().unwrap(),
            "--version",
            "4.2.3",
        ])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["default", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Default version set to 4.2.3"));
}

#[test]
fn cli_tanzu_install_shows_in_releases_list() {
    let temp = TempDir::new().unwrap();
    let tarball = create_test_tarball(
        &temp,
        "tanzu-rabbitmq-aarch64-4.2.3.tar.gz",
        "rabbitmq_server-4.2.3",
    );

    frm_cmd_with_dir(&temp)
        .args([
            "tanzu",
            "install",
            "--local-tanzu-rabbitmq-tarball-path",
            tarball.to_str().unwrap(),
            "--version",
            "4.2.3",
        ])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["releases", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.2.3"));
}

#[test]
fn cli_tanzu_install_short_version_flag() {
    let temp = TempDir::new().unwrap();
    let tarball = create_test_tarball(
        &temp,
        "tanzu-rabbitmq-aarch64-4.2.3.tar.gz",
        "rabbitmq_server-4.2.3",
    );

    frm_cmd_with_dir(&temp)
        .args([
            "tanzu",
            "install",
            "--local-tanzu-rabbitmq-tarball-path",
            tarball.to_str().unwrap(),
            "-V",
            "4.2.3",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("installed successfully"));
}

#[test]
fn cli_tanzu_install_short_force_flag() {
    let temp = TempDir::new().unwrap();
    let tarball = create_test_tarball(
        &temp,
        "tanzu-rabbitmq-aarch64-4.2.3.tar.gz",
        "rabbitmq_server-4.2.3",
    );

    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "tanzu",
            "install",
            "--local-tanzu-rabbitmq-tarball-path",
            tarball.to_str().unwrap(),
            "-V",
            "4.2.3",
            "-f",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("installed successfully"));
}
