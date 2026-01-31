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
use proptest::prelude::*;
use tempfile::TempDir;

fn arb_version() -> impl Strategy<Value = (u16, u16, u16)> {
    (1u16..20, 0u16..100, 0u16..100)
}

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

proptest! {
    #[test]
    fn releases_use_version_opt_matches_positional((major, minor, patch) in arb_version()) {
        let version = format!("{}.{}.{}", major, minor, patch);
        let temp = TempDir::new().unwrap();
        setup_version(&temp, &version);

        let output_positional = frm_cmd_with_dir(&temp)
            .args(["releases", "use", &version, "--shell", "bash"])
            .output()
            .unwrap();

        let output_opt = frm_cmd_with_dir(&temp)
            .args(["releases", "use", "--version", &version, "--shell", "bash"])
            .output()
            .unwrap();

        prop_assert_eq!(output_positional.status.success(), output_opt.status.success());
        prop_assert_eq!(output_positional.stdout, output_opt.stdout);
    }

    #[test]
    fn default_version_opt_matches_positional((major, minor, patch) in arb_version()) {
        let version = format!("{}.{}.{}", major, minor, patch);
        let temp = TempDir::new().unwrap();
        setup_version(&temp, &version);

        let output_positional = frm_cmd_with_dir(&temp)
            .args(["default", &version])
            .output()
            .unwrap();

        let temp2 = TempDir::new().unwrap();
        setup_version(&temp2, &version);
        let output_opt = frm_cmd_with_dir(&temp2)
            .args(["default", "--version", &version])
            .output()
            .unwrap();

        prop_assert_eq!(output_positional.status.success(), output_opt.status.success());
        prop_assert_eq!(output_positional.stdout, output_opt.stdout);
    }

    #[test]
    fn releases_uninstall_version_opt_matches_positional((major, minor, patch) in arb_version()) {
        let version = format!("{}.{}.{}", major, minor, patch);
        let temp = TempDir::new().unwrap();
        setup_version(&temp, &version);

        let output_positional = frm_cmd_with_dir(&temp)
            .args(["releases", "uninstall", &version])
            .output()
            .unwrap();

        let temp2 = TempDir::new().unwrap();
        setup_version(&temp2, &version);
        let output_opt = frm_cmd_with_dir(&temp2)
            .args(["releases", "uninstall", "--version", &version])
            .output()
            .unwrap();

        prop_assert_eq!(output_positional.status.success(), output_opt.status.success());
        prop_assert_eq!(output_positional.stdout, output_opt.stdout);
    }

    #[test]
    fn positional_always_takes_precedence_over_option((maj1, min1, pat1) in arb_version(), (maj2, min2, pat2) in arb_version()) {
        prop_assume!(maj1 != maj2 || min1 != min2 || pat1 != pat2);

        let version1 = format!("{}.{}.{}", maj1, min1, pat1);
        let version2 = format!("{}.{}.{}", maj2, min2, pat2);

        let temp = TempDir::new().unwrap();
        setup_version(&temp, &version1);
        setup_version(&temp, &version2);

        let output = frm_cmd_with_dir(&temp)
            .args(["releases", "use", &version1, "--version", &version2, "--shell", "bash"])
            .output()
            .unwrap();

        prop_assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        prop_assert!(stdout.contains(&version1));
    }
}

#[test]
fn releases_use_no_version_fails() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "use", "--shell", "bash"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no version specified"));
}

#[test]
fn alphas_use_no_version_fails() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "use", "--shell", "bash"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no version specified"));
}

#[test]
fn tanzu_use_no_version_fails() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["tanzu", "use", "--shell", "bash"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no version specified"));
}

#[test]
fn releases_install_no_version_fails() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no version specified"));
}

#[test]
fn releases_reinstall_no_version_fails() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "reinstall"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no version specified"));
}

#[test]
fn releases_uninstall_no_version_fails() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "uninstall"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no version specified"));
}

#[test]
fn alphas_reinstall_no_version_fails() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "reinstall"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no version specified"));
}

#[test]
fn alphas_uninstall_no_version_fails() {
    let temp = TempDir::new().unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "uninstall"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no version specified"));
}
