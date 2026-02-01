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

#[test]
fn cli_bg_help() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["bg", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Start and stop RabbitMQ nodes"))
        .stdout(predicate::str::contains("start"))
        .stdout(predicate::str::contains("stop"));
}

#[test]
fn cli_bg_no_subcommand() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["bg"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn cli_bg_start_help() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["bg", "start", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Start RabbitMQ server in background",
        ))
        .stdout(predicate::str::contains("--version"));
}

#[test]
fn cli_bg_stop_help() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["bg", "stop", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Stop a running RabbitMQ node"))
        .stdout(predicate::str::contains("--version"));
}

#[test]
fn cli_bg_start_version_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["bg", "start", "--version", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_bg_stop_version_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["bg", "stop", "--version", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_bg_start_no_version_provided() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["bg", "start"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no version specified"));
}

#[test]
fn cli_bg_stop_no_version_provided() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["bg", "stop"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no version specified"));
}

#[test]
fn cli_bg_start_server_not_found() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["bg", "start", "--version", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("file not found"));
}

#[test]
fn cli_bg_stop_rabbitmqctl_not_found() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["bg", "stop", "--version", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("file not found"));
}

#[test]
fn cli_bg_start_invalid_version_format() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["bg", "start", "--version", "not-a-version"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid version"));
}

#[test]
fn cli_bg_stop_invalid_version_format() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["bg", "stop", "--version", "invalid"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid version"));
}

#[test]
fn cli_bg_start_latest_no_versions() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["bg", "start", "--version", "latest"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no GA versions installed"));
}

#[test]
fn cli_bg_stop_latest_no_versions() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["bg", "stop", "--version", "latest"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no GA versions installed"));
}

#[test]
fn cli_bg_start_with_positional_version_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["bg", "start", "4.2.3"])
        .assert()
        .failure();
}

#[test]
fn cli_bg_stop_with_positional_version_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["bg", "stop", "4.2.3"])
        .assert()
        .failure();
}

#[test]
fn cli_bg_start_sbin_dir_empty() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let sbin_dir = version_dir.join("sbin");
    fs::create_dir_all(&sbin_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["bg", "start", "--version", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("file not found"));
}

#[test]
fn cli_bg_stop_sbin_dir_empty() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let sbin_dir = version_dir.join("sbin");
    fs::create_dir_all(&sbin_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["bg", "stop", "--version", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("file not found"));
}
