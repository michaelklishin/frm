// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! End-to-end integration tests that download real packages.
//! These tests require network access and may take longer to run.

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
fn end_to_end_releases_install_and_list() {
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
fn end_to_end_releases_install_already_installed() {
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
fn end_to_end_releases_install_force_reinstall() {
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
fn end_to_end_releases_path() {
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
fn end_to_end_releases_uninstall() {
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
fn end_to_end_releases_reinstall() {
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
fn end_to_end_use_installed_version() {
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
fn end_to_end_use_latest() {
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
fn end_to_end_default_and_list() {
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
fn end_to_end_default_latest() {
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
fn end_to_end_conf_set_and_get_key() {
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
fn end_to_end_inspect_rabbitmq_conf() {
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
fn end_to_end_inspect_enabled_plugins() {
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
fn end_to_end_alphas_install_latest_and_list() {
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
fn end_to_end_alphas_install_and_uninstall() {
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
fn end_to_end_alphas_prune() {
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
fn end_to_end_alphas_path() {
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
// Full workflow test
// ============================================================================

#[test]
fn end_to_end_full_workflow() {
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

// ============================================================================
// releases uninstall with latest
// ============================================================================

#[test]
fn end_to_end_releases_uninstall_latest() {
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
        .args(["releases", "uninstall", "latest"])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "RabbitMQ {} uninstalled",
            TEST_GA_VERSION
        )));

    let version_dir = temp.path().join("versions").join(TEST_GA_VERSION);
    assert!(!version_dir.exists());

    let older_version_dir = temp.path().join("versions").join(TEST_GA_VERSION_2);
    assert!(older_version_dir.exists());
}

// ============================================================================
// alphas uninstall with latest
// ============================================================================

#[test]
fn end_to_end_alphas_uninstall_latest() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "install", "--latest"])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alpha"));

    frm_cmd_with_dir(&temp)
        .args(["alphas", "uninstall", "latest"])
        .assert()
        .success()
        .stdout(predicate::str::contains("uninstalled"));

    frm_cmd_with_dir(&temp)
        .args(["alphas", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No alpha"));
}

// ============================================================================
// cli command with rabbitmqctl
// ============================================================================

#[test]
fn end_to_end_cli_rabbitmqctl_version() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["cli", "rabbitmqctl", "-V", TEST_GA_VERSION, "--", "version"])
        .assert()
        .success()
        .stdout(predicate::str::contains(TEST_GA_VERSION));
}

#[test]
fn end_to_end_cli_rabbitmqctl_version_with_latest() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["cli", "rabbitmqctl", "-V", "latest", "--", "version"])
        .assert()
        .success()
        .stdout(predicate::str::contains(TEST_GA_VERSION));
}

// ============================================================================
// Multi-step complex scenarios
// ============================================================================

#[test]
fn end_to_end_scenario_version_switching() {
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
        .args(["use", TEST_GA_VERSION_2, "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains(TEST_GA_VERSION_2));

    frm_cmd_with_dir(&temp)
        .args(["use", "latest", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains(TEST_GA_VERSION));

    frm_cmd_with_dir(&temp)
        .args(["default", TEST_GA_VERSION_2])
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

#[test]
fn end_to_end_scenario_config_across_versions() {
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
        .args([
            "conf",
            "set-key",
            "listeners.tcp.default",
            "5672",
            "-V",
            TEST_GA_VERSION_2,
        ])
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
            TEST_GA_VERSION_2,
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("5672"));

    frm_cmd_with_dir(&temp)
        .args(["conf", "get-key", "listeners.tcp.default", "-V", "latest"])
        .assert()
        .success()
        .stdout(predicate::str::contains("5673"));
}

#[test]
fn end_to_end_scenario_install_configure_run_cli() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["default", "latest"])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args([
            "conf",
            "set-key",
            "log.console.level",
            "warning",
            "-V",
            "latest",
        ])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["inspect", "rabbitmq.conf", "-V", "latest"])
        .assert()
        .success()
        .stdout(predicate::str::contains("log.console.level = warning"));

    frm_cmd_with_dir(&temp)
        .args(["cli", "rabbitmqctl", "-V", "latest", "--", "version"])
        .assert()
        .success()
        .stdout(predicate::str::contains(TEST_GA_VERSION));

    frm_cmd_with_dir(&temp)
        .args(["releases", "path", "-V", "latest"])
        .assert()
        .success()
        .stdout(predicate::str::contains(TEST_GA_VERSION));
}

#[test]
fn end_to_end_scenario_mixed_releases_and_alphas() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "install", "--latest"])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["releases", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains(TEST_GA_VERSION))
        .stdout(predicate::str::contains("alpha").not());

    frm_cmd_with_dir(&temp)
        .args(["alphas", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alpha"));

    frm_cmd_with_dir(&temp)
        .args(["use", "latest", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains(TEST_GA_VERSION));

    frm_cmd_with_dir(&temp)
        .args(["default", "latest"])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Default version set to {}",
            TEST_GA_VERSION
        )));

    frm_cmd_with_dir(&temp)
        .args(["alphas", "uninstall", "latest"])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["releases", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains(TEST_GA_VERSION));
}

#[test]
fn end_to_end_scenario_upgrade_workflow() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION_2])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["default", TEST_GA_VERSION_2])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args([
            "conf",
            "set-key",
            "vm_memory_high_watermark.relative",
            "0.6",
            "-V",
            TEST_GA_VERSION_2,
        ])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args([
            "conf",
            "set-key",
            "vm_memory_high_watermark.relative",
            "0.6",
            "-V",
            "latest",
        ])
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

#[test]
fn end_to_end_scenario_inspect_all_config_files() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", TEST_GA_VERSION])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["inspect", "rabbitmq.conf", "-V", "latest"])
        .assert()
        .success()
        .stdout(predicate::str::contains("listeners.tcp.default"));

    frm_cmd_with_dir(&temp)
        .args(["inspect", "enabled_plugins", "-V", "latest"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rabbitmq_management"));
}
