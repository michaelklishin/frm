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
fn cli_releases_check_signature_help() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["releases", "check-signature", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Verify the GPG signature"))
        .stdout(predicate::str::contains("--version"));
}

#[test]
fn cli_releases_check_signature_version_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["releases", "check-signature", "--version", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_releases_check_signature_no_version_provided() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["releases", "check-signature"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no version specified"));
}

#[test]
fn cli_releases_check_signature_invalid_version() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["releases", "check-signature", "--version", "not-a-version"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid version"));
}

#[test]
fn cli_releases_check_signature_rejects_alpha() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.3.0-alpha.132057c7");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "check-signature",
            "--version",
            "4.3.0-alpha.132057c7",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("only supports release versions"));
}

#[test]
fn cli_releases_check_signature_latest_no_versions() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["releases", "check-signature", "--version", "latest"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no GA versions installed"));
}

#[test]
fn cli_releases_check_signature_accepts_beta() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.0-beta.1");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "check-signature", "--version", "4.2.0-beta.1"])
        .assert()
        .stdout(predicate::str::contains("OK").or(predicate::str::is_empty()))
        .stderr(
            predicate::str::contains("only supports release versions")
                .not()
                .or(predicate::str::is_empty()),
        );
}

#[test]
fn cli_releases_check_signature_accepts_rc() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.0-rc.1");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "check-signature", "--version", "4.2.0-rc.1"])
        .assert()
        .stdout(predicate::str::contains("OK").or(predicate::str::is_empty()))
        .stderr(
            predicate::str::contains("only supports release versions")
                .not()
                .or(predicate::str::is_empty()),
        );
}
