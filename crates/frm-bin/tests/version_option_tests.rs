// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[allow(deprecated)]
fn frm_cmd_with_dir(dir: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("frm").unwrap();
    cmd.env("FRM_DIR", dir.path());
    cmd
}

fn setup_version(temp: &TempDir, version: &str) {
    let version_dir = temp.path().join("versions").join(version);
    fs::create_dir_all(version_dir.join("sbin")).unwrap();
}

// releases use

#[test]
fn releases_use_with_version_option() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.2.3");

    frm_cmd_with_dir(&temp)
        .args(["releases", "use", "--version", "4.2.3", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("export PATH="))
        .stdout(predicate::str::contains("4.2.3"));
}

#[test]
fn releases_use_with_version_option_short() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.2.3");

    frm_cmd_with_dir(&temp)
        .args(["releases", "use", "-V", "4.2.3", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.2.3"));
}

#[test]
fn releases_use_version_option_takes_precedence() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.2.3");
    setup_version(&temp, "4.2.4");

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "use",
            "4.2.3",
            "--version",
            "4.2.4",
            "--shell",
            "bash",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.2.4"));
}

#[test]
fn releases_use_version_option_latest() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.2.2");
    setup_version(&temp, "4.2.3");

    frm_cmd_with_dir(&temp)
        .args(["releases", "use", "--version", "latest", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.2.3"));
}

#[test]
fn releases_use_help_shows_version_option() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "use", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("-V, --version"))
        .stdout(predicate::str::contains(
            "takes precedence over the positional argument",
        ));
}

// releases install

#[test]
fn releases_install_version_option_already_exists() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.2.3");

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", "--version", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already installed"));
}

#[test]
fn releases_install_version_option_short() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.2.3");

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already installed"));
}

#[test]
fn releases_install_version_option_takes_precedence() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.2.4");

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", "4.2.3", "--version", "4.2.4"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("4.2.4"))
        .stderr(predicate::str::contains("already installed"));
}

#[test]
fn releases_install_help_shows_version_option() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("-V, --version"))
        .stdout(predicate::str::contains(
            "takes precedence over the positional argument",
        ));
}

// releases reinstall

#[test]
fn releases_reinstall_version_option_not_installed() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "reinstall", "--version", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn releases_reinstall_version_option_short() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "reinstall", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn releases_reinstall_help_shows_version_option() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "reinstall", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("-V, --version"))
        .stdout(predicate::str::contains(
            "takes precedence over the positional argument",
        ));
}

// releases uninstall

#[test]
fn releases_uninstall_version_option_not_installed() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "uninstall", "--version", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn releases_uninstall_version_option_installed() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.2.3");

    frm_cmd_with_dir(&temp)
        .args(["releases", "uninstall", "--version", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.2.3 uninstalled"));
}

#[test]
fn releases_uninstall_version_option_latest() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.2.2");
    setup_version(&temp, "4.2.3");

    frm_cmd_with_dir(&temp)
        .args(["releases", "uninstall", "--version", "latest"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.2.3 uninstalled"));
}

#[test]
fn releases_uninstall_help_shows_version_option() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "uninstall", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("-V, --version"))
        .stdout(predicate::str::contains(
            "takes precedence over the positional argument",
        ));
}

// alphas use

#[test]
fn alphas_use_with_version_option() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.3.0-alpha.132057c7");

    frm_cmd_with_dir(&temp)
        .args([
            "alphas",
            "use",
            "--version",
            "4.3.0-alpha.132057c7",
            "--shell",
            "bash",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.3.0-alpha.132057c7"));
}

#[test]
fn alphas_use_with_version_option_short() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.3.0-alpha.132057c7");

    frm_cmd_with_dir(&temp)
        .args([
            "alphas",
            "use",
            "-V",
            "4.3.0-alpha.132057c7",
            "--shell",
            "bash",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.3.0-alpha.132057c7"));
}

#[test]
fn alphas_use_version_option_takes_precedence() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.3.0-alpha.132057c7");
    setup_version(&temp, "4.3.0-alpha.232057c8");

    frm_cmd_with_dir(&temp)
        .args([
            "alphas",
            "use",
            "4.3.0-alpha.132057c7",
            "--version",
            "4.3.0-alpha.232057c8",
            "--shell",
            "bash",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.3.0-alpha.232057c8"));
}

#[test]
fn alphas_use_version_option_latest() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.3.0-alpha.132057c7");
    setup_version(&temp, "4.3.0-alpha.232057c8");

    frm_cmd_with_dir(&temp)
        .args(["alphas", "use", "--version", "latest", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.3.0-alpha.232057c8"));
}

#[test]
fn alphas_use_help_shows_version_option() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "use", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("-V, --version"))
        .stdout(predicate::str::contains(
            "takes precedence over the positional argument",
        ));
}

// alphas install

#[test]
fn alphas_install_help_shows_version_option() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "install", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("-V, --version"))
        .stdout(predicate::str::contains(
            "takes precedence over the positional argument",
        ));
}

// alphas reinstall

#[test]
fn alphas_reinstall_version_option_not_installed() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "reinstall", "--version", "4.3.0-alpha.132057c7"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn alphas_reinstall_version_option_short() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "reinstall", "-V", "4.3.0-alpha.132057c7"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn alphas_reinstall_help_shows_version_option() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "reinstall", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("-V, --version"))
        .stdout(predicate::str::contains(
            "takes precedence over the positional argument",
        ));
}

// alphas uninstall

#[test]
fn alphas_uninstall_version_option_not_installed() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "uninstall", "--version", "4.3.0-alpha.132057c7"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn alphas_uninstall_version_option_installed() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.3.0-alpha.132057c7");

    frm_cmd_with_dir(&temp)
        .args(["alphas", "uninstall", "--version", "4.3.0-alpha.132057c7"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.3.0-alpha.132057c7 uninstalled"));
}

#[test]
fn alphas_uninstall_version_option_latest() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.3.0-alpha.132057c7");
    setup_version(&temp, "4.3.0-alpha.232057c8");

    frm_cmd_with_dir(&temp)
        .args(["alphas", "uninstall", "--version", "latest"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.3.0-alpha.232057c8 uninstalled"));
}

#[test]
fn alphas_uninstall_help_shows_version_option() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "uninstall", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("-V, --version"))
        .stdout(predicate::str::contains(
            "takes precedence over the positional argument",
        ));
}

// tanzu use

#[test]
fn tanzu_use_with_version_option() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.2.3");

    frm_cmd_with_dir(&temp)
        .args(["tanzu", "use", "--version", "4.2.3", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.2.3"));
}

#[test]
fn tanzu_use_with_version_option_short() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.2.3");

    frm_cmd_with_dir(&temp)
        .args(["tanzu", "use", "-V", "4.2.3", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.2.3"));
}

#[test]
fn tanzu_use_version_option_takes_precedence() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.2.3");
    setup_version(&temp, "4.2.4");

    frm_cmd_with_dir(&temp)
        .args([
            "tanzu",
            "use",
            "4.2.3",
            "--version",
            "4.2.4",
            "--shell",
            "bash",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.2.4"));
}

#[test]
fn tanzu_use_help_shows_version_option() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["tanzu", "use", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("-V, --version"))
        .stdout(predicate::str::contains(
            "takes precedence over the positional argument",
        ));
}

// default

#[test]
fn default_with_version_option() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.2.3");

    frm_cmd_with_dir(&temp)
        .args(["default", "--version", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Default version set to 4.2.3"));
}

#[test]
fn default_with_version_option_short() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.2.3");

    frm_cmd_with_dir(&temp)
        .args(["default", "-V", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Default version set to 4.2.3"));
}

#[test]
fn default_version_option_takes_precedence() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.2.3");
    setup_version(&temp, "4.2.4");

    frm_cmd_with_dir(&temp)
        .args(["default", "4.2.3", "--version", "4.2.4"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Default version set to 4.2.4"));
}

#[test]
fn default_version_option_latest() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.2.2");
    setup_version(&temp, "4.2.3");

    frm_cmd_with_dir(&temp)
        .args(["default", "--version", "latest"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Default version set to 4.2.3"));
}

#[test]
fn default_version_option_not_installed() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["default", "--version", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn default_help_shows_version_option() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["default", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("-V, --version"))
        .stdout(predicate::str::contains(
            "takes precedence over the positional argument",
        ));
}

#[test]
fn default_requires_version_either_positional_or_option() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["default"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no version specified"));
}

// Additional precedence tests

#[test]
fn releases_reinstall_version_option_takes_precedence() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "reinstall", "4.2.3", "--version", "4.2.4"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("4.2.4"))
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn releases_uninstall_version_option_takes_precedence() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.2.3");
    setup_version(&temp, "4.2.4");

    frm_cmd_with_dir(&temp)
        .args(["releases", "uninstall", "4.2.3", "--version", "4.2.4"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.2.4 uninstalled"));
}

#[test]
fn alphas_reinstall_version_option_takes_precedence() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "alphas",
            "reinstall",
            "4.3.0-alpha.132057c7",
            "--version",
            "4.3.0-alpha.232057c8",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("4.3.0-alpha.232057c8"))
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn alphas_uninstall_version_option_takes_precedence() {
    let temp = TempDir::new().unwrap();
    setup_version(&temp, "4.3.0-alpha.132057c7");
    setup_version(&temp, "4.3.0-alpha.232057c8");

    frm_cmd_with_dir(&temp)
        .args([
            "alphas",
            "uninstall",
            "4.3.0-alpha.132057c7",
            "--version",
            "4.3.0-alpha.232057c8",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.3.0-alpha.232057c8 uninstalled"));
}
