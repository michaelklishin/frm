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
fn frm_cmd() -> Command {
    Command::cargo_bin("frm").unwrap()
}

#[allow(deprecated)]
fn frm_cmd_with_dir(dir: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("frm").unwrap();
    cmd.env("FRM_DIR", dir.path());
    cmd
}

#[test]
fn cli_releases_cp_etc_file_help() {
    frm_cmd()
        .args(["releases", "cp-etc-file", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--local-file-path"))
        .stdout(predicate::str::contains("--etc-file"))
        .stdout(predicate::str::contains("rabbitmq.conf"))
        .stdout(predicate::str::contains("advanced.config"))
        .stdout(predicate::str::contains("enabled_plugins"));
}

#[test]
fn cli_alphas_cp_etc_file_help() {
    frm_cmd()
        .args(["alphas", "cp-etc-file", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--local-file-path"))
        .stdout(predicate::str::contains("--etc-file"))
        .stdout(predicate::str::contains("rabbitmq.conf"));
}

#[test]
fn cli_releases_cp_etc_file_missing_local_path() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--etc-file",
            "rabbitmq.conf",
            "-V",
            "4.2.3",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--local-file-path"));
}

#[test]
fn cli_releases_cp_etc_file_missing_etc_file() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--local-file-path",
            "/tmp/test.conf",
            "-V",
            "4.2.3",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--etc-file"));
}

#[test]
fn cli_releases_cp_etc_file_missing_version() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("source.conf");
    fs::write(&src_file, "test = value\n").unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "rabbitmq.conf",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no version specified"));
}

#[test]
fn cli_releases_cp_etc_file_version_not_installed() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("source.conf");
    fs::write(&src_file, "test = value\n").unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "rabbitmq.conf",
            "-V",
            "4.2.3",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_releases_cp_etc_file_source_not_found() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--local-file-path",
            "/nonexistent/path/file.conf",
            "--etc-file",
            "rabbitmq.conf",
            "-V",
            "4.2.3",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("file not found"));
}

#[test]
fn cli_releases_cp_etc_file_invalid_etc_file() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("source.conf");
    fs::write(&src_file, "test = value\n").unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "invalid.file",
            "-V",
            "4.2.3",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn cli_releases_cp_etc_file_success_rabbitmq_conf() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("source.conf");
    fs::write(&src_file, "listeners.tcp.default = 5672\n").unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "rabbitmq.conf",
            "-V",
            "4.2.3",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Copied"));

    let dest_file = version_dir
        .join("etc")
        .join("rabbitmq")
        .join("rabbitmq.conf");
    assert!(dest_file.exists());
    let content = fs::read_to_string(&dest_file).unwrap();
    assert!(content.contains("listeners.tcp.default = 5672"));
}

#[test]
fn cli_releases_cp_etc_file_success_advanced_config() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("advanced.config");
    let advanced_content = r#"[
  {kernel, [
    {inet_dist_listen_min, 25672}
  ]}
]."#;
    fs::write(&src_file, advanced_content).unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "advanced.config",
            "-V",
            "4.2.3",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Copied"));

    let dest_file = version_dir
        .join("etc")
        .join("rabbitmq")
        .join("advanced.config");
    assert!(dest_file.exists());
    let content = fs::read_to_string(&dest_file).unwrap();
    assert!(content.contains("inet_dist_listen_min"));
}

#[test]
fn cli_releases_cp_etc_file_success_rabbitmq_config() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("rabbitmq.config");
    let config_content = r#"[
  {rabbit, [
    {tcp_listeners, [5672]}
  ]}
]."#;
    fs::write(&src_file, config_content).unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "rabbitmq.config",
            "-V",
            "4.2.3",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Copied"));

    let dest_file = version_dir
        .join("etc")
        .join("rabbitmq")
        .join("rabbitmq.config");
    assert!(dest_file.exists());
}

#[test]
fn cli_releases_cp_etc_file_success_enabled_plugins() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("enabled_plugins");
    fs::write(
        &src_file,
        "[rabbitmq_management,rabbitmq_prometheus,rabbitmq_stream].\n",
    )
    .unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "enabled_plugins",
            "-V",
            "4.2.3",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Copied"));

    let dest_file = version_dir
        .join("etc")
        .join("rabbitmq")
        .join("enabled_plugins");
    assert!(dest_file.exists());
    let content = fs::read_to_string(&dest_file).unwrap();
    assert!(content.contains("rabbitmq_management"));
}

#[test]
fn cli_releases_cp_etc_file_creates_etc_dir() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("source.conf");
    fs::write(&src_file, "test = value\n").unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    let etc_dir = version_dir.join("etc").join("rabbitmq");
    assert!(!etc_dir.exists());

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "rabbitmq.conf",
            "-V",
            "4.2.3",
        ])
        .assert()
        .success();

    assert!(etc_dir.exists());
}

#[test]
fn cli_releases_cp_etc_file_overwrites_existing() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("source.conf");
    fs::write(&src_file, "new_content = true\n").unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let etc_dir = version_dir.join("etc").join("rabbitmq");
    fs::create_dir_all(&etc_dir).unwrap();
    fs::write(etc_dir.join("rabbitmq.conf"), "old_content = false\n").unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "rabbitmq.conf",
            "-V",
            "4.2.3",
        ])
        .assert()
        .success();

    let content = fs::read_to_string(etc_dir.join("rabbitmq.conf")).unwrap();
    assert!(content.contains("new_content"));
    assert!(!content.contains("old_content"));
}

#[test]
fn cli_releases_cp_etc_file_rejects_alpha() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("source.conf");
    fs::write(&src_file, "test = value\n").unwrap();
    let version_dir = temp.path().join("versions").join("4.3.0-alpha.abc123");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "rabbitmq.conf",
            "-V",
            "4.3.0-alpha.abc123",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("expected a non-alpha version"));
}

#[test]
fn cli_releases_cp_etc_file_with_latest() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("source.conf");
    fs::write(&src_file, "latest_config = true\n").unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.0.0")).unwrap();
    fs::create_dir_all(versions_dir.join("4.2.3")).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "rabbitmq.conf",
            "-V",
            "latest",
        ])
        .assert()
        .success();

    let dest_file = versions_dir
        .join("4.2.3")
        .join("etc")
        .join("rabbitmq")
        .join("rabbitmq.conf");
    assert!(dest_file.exists());
    let content = fs::read_to_string(&dest_file).unwrap();
    assert!(content.contains("latest_config"));

    let other_file = versions_dir
        .join("4.0.0")
        .join("etc")
        .join("rabbitmq")
        .join("rabbitmq.conf");
    assert!(!other_file.exists());
}

#[test]
fn cli_alphas_cp_etc_file_version_not_installed() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("source.conf");
    fs::write(&src_file, "test = value\n").unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "alphas",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "rabbitmq.conf",
            "-V",
            "4.3.0-alpha.abc123",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_alphas_cp_etc_file_success() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("source.conf");
    fs::write(&src_file, "alpha_config = true\n").unwrap();
    let version_dir = temp.path().join("versions").join("4.3.0-alpha.abc123");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "alphas",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "rabbitmq.conf",
            "-V",
            "4.3.0-alpha.abc123",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Copied"));

    let dest_file = version_dir
        .join("etc")
        .join("rabbitmq")
        .join("rabbitmq.conf");
    assert!(dest_file.exists());
    let content = fs::read_to_string(&dest_file).unwrap();
    assert!(content.contains("alpha_config"));
}

#[test]
fn cli_alphas_cp_etc_file_rejects_release() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("source.conf");
    fs::write(&src_file, "test = value\n").unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "alphas",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "rabbitmq.conf",
            "-V",
            "4.2.3",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("expected an alpha version"));
}

#[test]
fn cli_alphas_cp_etc_file_with_latest() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("source.conf");
    fs::write(&src_file, "alpha_latest = true\n").unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.abc123")).unwrap();
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.def456")).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "alphas",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "rabbitmq.conf",
            "-V",
            "latest",
        ])
        .assert()
        .success();

    let dest_file = versions_dir
        .join("4.3.0-alpha.def456")
        .join("etc")
        .join("rabbitmq")
        .join("rabbitmq.conf");
    assert!(dest_file.exists());
}

#[test]
fn cli_alphas_cp_etc_file_enabled_plugins() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("enabled_plugins");
    fs::write(&src_file, "[rabbitmq_stream].\n").unwrap();
    let version_dir = temp.path().join("versions").join("4.3.0-alpha.abc123");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "alphas",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "enabled_plugins",
            "-V",
            "4.3.0-alpha.abc123",
        ])
        .assert()
        .success();

    let dest_file = version_dir
        .join("etc")
        .join("rabbitmq")
        .join("enabled_plugins");
    assert!(dest_file.exists());
}

#[test]
fn cli_releases_cp_etc_file_preserves_binary_content() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("source.conf");
    let binary_content: Vec<u8> = vec![0x00, 0x01, 0x02, 0xFF, 0xFE, 0xFD];
    fs::write(&src_file, &binary_content).unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "rabbitmq.conf",
            "-V",
            "4.2.3",
        ])
        .assert()
        .success();

    let dest_file = version_dir
        .join("etc")
        .join("rabbitmq")
        .join("rabbitmq.conf");
    let copied_content = fs::read(&dest_file).unwrap();
    assert_eq!(copied_content, binary_content);
}

#[test]
fn cli_releases_cp_etc_file_empty_file() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("empty.conf");
    fs::write(&src_file, "").unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "rabbitmq.conf",
            "-V",
            "4.2.3",
        ])
        .assert()
        .success();

    let dest_file = version_dir
        .join("etc")
        .join("rabbitmq")
        .join("rabbitmq.conf");
    assert!(dest_file.exists());
    let content = fs::read_to_string(&dest_file).unwrap();
    assert!(content.is_empty());
}

#[test]
fn cli_releases_cp_etc_file_with_rc_version() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("source.conf");
    fs::write(&src_file, "rc_config = true\n").unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3-rc.1");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "rabbitmq.conf",
            "-V",
            "4.2.3-rc.1",
        ])
        .assert()
        .success();

    let dest_file = version_dir
        .join("etc")
        .join("rabbitmq")
        .join("rabbitmq.conf");
    assert!(dest_file.exists());
}

#[test]
fn cli_releases_cp_etc_file_latest_with_whitespace() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("source.conf");
    fs::write(&src_file, "whitespace_test = true\n").unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "rabbitmq.conf",
            "-V",
            "  latest  ",
        ])
        .assert()
        .success();

    let dest_file = version_dir
        .join("etc")
        .join("rabbitmq")
        .join("rabbitmq.conf");
    assert!(dest_file.exists());
}

#[test]
fn cli_releases_cp_etc_file_latest_case_insensitive() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("source.conf");
    fs::write(&src_file, "case_test = true\n").unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "rabbitmq.conf",
            "-V",
            "LATEST",
        ])
        .assert()
        .success();

    let etc_dir = version_dir.join("etc").join("rabbitmq");
    fs::remove_dir_all(&etc_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "rabbitmq.conf",
            "-V",
            "Latest",
        ])
        .assert()
        .success();

    let dest_file = etc_dir.join("rabbitmq.conf");
    assert!(dest_file.exists());
}

#[test]
fn cli_releases_cp_etc_file_latest_no_ga_versions() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("source.conf");
    fs::write(&src_file, "test = true\n").unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "rabbitmq.conf",
            "-V",
            "latest",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no GA versions installed"));
}

#[test]
fn cli_alphas_cp_etc_file_latest_no_alpha_versions() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("source.conf");
    fs::write(&src_file, "test = true\n").unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "alphas",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "rabbitmq.conf",
            "-V",
            "latest",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no alpha versions installed"));
}

#[test]
fn cli_releases_cp_etc_file_unicode_content() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("source.conf");
    let unicode_content = "# Configuration with Unicode: 日本語, émojis: 🐰🥕\nkey = 值\n";
    fs::write(&src_file, unicode_content).unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "rabbitmq.conf",
            "-V",
            "4.2.3",
        ])
        .assert()
        .success();

    let dest_file = version_dir
        .join("etc")
        .join("rabbitmq")
        .join("rabbitmq.conf");
    let content = fs::read_to_string(&dest_file).unwrap();
    assert!(content.contains("日本語"));
    assert!(content.contains("🐰"));
    assert!(content.contains("值"));
}

#[test]
fn cli_releases_cp_etc_file_with_beta_version() {
    let temp = TempDir::new().unwrap();
    let src_file = temp.path().join("source.conf");
    fs::write(&src_file, "beta_config = true\n").unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3-beta.1");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "releases",
            "cp-etc-file",
            "--local-file-path",
            src_file.to_str().unwrap(),
            "--etc-file",
            "rabbitmq.conf",
            "-V",
            "4.2.3-beta.1",
        ])
        .assert()
        .success();

    let dest_file = version_dir
        .join("etc")
        .join("rabbitmq")
        .join("rabbitmq.conf");
    assert!(dest_file.exists());
}
