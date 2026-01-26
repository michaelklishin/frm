// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! End-to-end integration tests that download real packages.
//! These tests require network access and may take longer to run.

use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

const TEST_GA_VERSION: &str = "4.0.5";
const TEST_GA_VERSION_2: &str = "4.0.4";

#[allow(deprecated)]
fn frm_cmd_with_dir(dir: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("frm").unwrap();
    cmd.env("FRM_DIR", dir.path());
    cmd.timeout(std::time::Duration::from_secs(300));
    cmd
}

// ============================================================================
// releases install
// ============================================================================

#[test]
fn e2e_releases_install_and_list() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "RabbitMQ {} installed",
            TEST_GA_VERSION
        )));

    let version_dir = temp.path().join("versions").join(TEST_GA_VERSION);
    assert!(version_dir.exists());
    assert!(version_dir.join("sbin").join("rabbitmq-server").exists());
    assert!(version_dir.join("sbin").join("rabbitmqctl").exists());

    frm_cmd_with_dir(&temp)
        .args(["releases", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains(TEST_GA_VERSION));
}

#[test]
fn e2e_releases_install_already_installed() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already installed"));
}

#[test]
fn e2e_releases_install_force_reinstall() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION, "--force"])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "RabbitMQ {} installed",
            TEST_GA_VERSION
        )));
}

// ============================================================================
// releases path
// ============================================================================

#[test]
fn e2e_releases_path() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["releases", "path", "-V", TEST_GA_VERSION])
        .assert()
        .success()
        .stdout(predicate::str::contains(TEST_GA_VERSION));
}

// ============================================================================
// releases uninstall
// ============================================================================

#[test]
fn e2e_releases_uninstall() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION])
        .assert()
        .success();

    let version_dir = temp.path().join("versions").join(TEST_GA_VERSION);
    assert!(version_dir.exists());

    frm_cmd_with_dir(&temp)
        .args(["releases", "uninstall", TEST_GA_VERSION])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "RabbitMQ {} uninstalled",
            TEST_GA_VERSION
        )));

    assert!(!version_dir.exists());
}

// ============================================================================
// releases reinstall
// ============================================================================

#[test]
fn e2e_releases_reinstall() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["releases", "reinstall", TEST_GA_VERSION])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "RabbitMQ {} reinstalled",
            TEST_GA_VERSION
        )));

    let version_dir = temp.path().join("versions").join(TEST_GA_VERSION);
    assert!(version_dir.exists());
    assert!(version_dir.join("sbin").join("rabbitmq-server").exists());
}

// ============================================================================
// use command
// ============================================================================

#[test]
fn e2e_use_installed_version() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["use", TEST_GA_VERSION, "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("export PATH="))
        .stdout(predicate::str::contains(TEST_GA_VERSION));
}

#[test]
fn e2e_use_latest() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION_2])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["use", "latest", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains(TEST_GA_VERSION));
}

// ============================================================================
// default command
// ============================================================================

#[test]
fn e2e_default_and_list() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["default", TEST_GA_VERSION])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Default version set to {}",
            TEST_GA_VERSION
        )));

    frm_cmd_with_dir(&temp)
        .args(["releases", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("[*] {}", TEST_GA_VERSION)));
}

#[test]
fn e2e_default_latest() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION_2])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["default", "latest"])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Default version set to {}",
            TEST_GA_VERSION
        )));
}

// ============================================================================
// conf commands
// ============================================================================

#[test]
fn e2e_conf_set_and_get_key() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args([
            "conf",
            "set-key",
            "listeners.tcp.default",
            "5673",
            "-V",
            TEST_GA_VERSION,
        ])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args([
            "conf",
            "get-key",
            "listeners.tcp.default",
            "-V",
            TEST_GA_VERSION,
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("5673"));
}

// ============================================================================
// inspect command
// ============================================================================

#[test]
fn e2e_inspect_rabbitmq_conf() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["inspect", "rabbitmq.conf", "-V", TEST_GA_VERSION])
        .assert()
        .success()
        .stdout(predicate::str::contains("listeners.tcp.default"));
}

#[test]
fn e2e_inspect_enabled_plugins() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["inspect", "enabled_plugins", "-V", TEST_GA_VERSION])
        .assert()
        .success()
        .stdout(predicate::str::contains("rabbitmq_management"));
}

// ============================================================================
// alphas commands
// ============================================================================

#[test]
fn e2e_alphas_install_latest_and_list() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "install", "--latest"])
        .assert()
        .success()
        .stdout(predicate::str::contains("installed"));

    frm_cmd_with_dir(&temp)
        .args(["alphas", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alpha"));
}

#[test]
fn e2e_alphas_install_and_uninstall() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "install", "--latest"])
        .assert()
        .success();

    let output = frm_cmd_with_dir(&temp)
        .args(["alphas", "list"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let alpha_version = stdout
        .lines()
        .find(|line| line.contains("alpha"))
        .and_then(|line| line.split_whitespace().find(|s| s.contains("alpha")))
        .unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "uninstall", alpha_version])
        .assert()
        .success()
        .stdout(predicate::str::contains("uninstalled"));

    frm_cmd_with_dir(&temp)
        .args(["alphas", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No alpha"));
}

#[test]
fn e2e_alphas_prune() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "install", "--latest"])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "prune"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed"));

    frm_cmd_with_dir(&temp)
        .args(["alphas", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No alpha"));
}

#[test]
fn e2e_alphas_path() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "install", "--latest"])
        .assert()
        .success();

    let output = frm_cmd_with_dir(&temp)
        .args(["alphas", "list"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let alpha_version = stdout
        .lines()
        .find(|line| line.contains("alpha"))
        .and_then(|line| line.split_whitespace().find(|s| s.contains("alpha")))
        .unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "path", "-V", alpha_version])
        .assert()
        .success()
        .stdout(predicate::str::contains(alpha_version));
}

// ============================================================================
// .tool-versions support
// ============================================================================

#[test]
fn e2e_tool_versions_file() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION])
        .assert()
        .success();

    let work_dir = temp.path().join("project");
    fs::create_dir_all(&work_dir).unwrap();
    fs::write(
        work_dir.join(".tool-versions"),
        format!("rabbitmq {}\n", TEST_GA_VERSION),
    )
    .unwrap();

    frm_cmd_with_dir(&temp)
        .current_dir(&work_dir)
        .args(["use", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains(TEST_GA_VERSION));

    frm_cmd_with_dir(&temp)
        .current_dir(&work_dir)
        .args(["releases", "path"])
        .assert()
        .success()
        .stdout(predicate::str::contains(TEST_GA_VERSION));
}

// ============================================================================
// Full workflow test
// ============================================================================

#[test]
fn e2e_full_workflow() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION_2])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["releases", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains(TEST_GA_VERSION_2))
        .stdout(predicate::str::contains(TEST_GA_VERSION));

    frm_cmd_with_dir(&temp)
        .args(["default", TEST_GA_VERSION])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["releases", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("[*] {}", TEST_GA_VERSION)))
        .stdout(predicate::str::contains(format!(
            "[ ] {}",
            TEST_GA_VERSION_2
        )));

    frm_cmd_with_dir(&temp)
        .args(["use", "latest", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains(TEST_GA_VERSION));

    frm_cmd_with_dir(&temp)
        .args([
            "conf",
            "set-key",
            "vm_memory_high_watermark.relative",
            "0.7",
            "-V",
            TEST_GA_VERSION,
        ])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["inspect", "rabbitmq.conf", "-V", TEST_GA_VERSION])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "vm_memory_high_watermark.relative = 0.7",
        ));

    frm_cmd_with_dir(&temp)
        .args(["releases", "uninstall", TEST_GA_VERSION_2])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["releases", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains(TEST_GA_VERSION))
        .stdout(predicate::str::contains(TEST_GA_VERSION_2).not());
}
