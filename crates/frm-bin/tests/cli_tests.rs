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
fn cli_no_args_shows_help() {
    frm_cmd()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn cli_help_flag() {
    frm_cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("frm "))
        .stdout(predicate::str::contains(
            "Frakking RabbitMQ version Manager",
        ));
}

#[test]
fn cli_releases_list_empty() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["releases", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "No stable RabbitMQ releases installed",
        ));
}

#[test]
fn cli_releases_list_alias() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["releases", "ls"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "No stable RabbitMQ releases installed",
        ));
}

#[test]
fn cli_releases_list_with_versions() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.0.0")).unwrap();
    fs::create_dir_all(versions_dir.join("4.2.3")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.0.0"))
        .stdout(predicate::str::contains("4.2.3"));
}

#[test]
fn cli_releases_list_excludes_alphas() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.2.3")).unwrap();
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.132057c7")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.2.3"))
        .stdout(predicate::str::contains("4.3.0-alpha").not());
}

#[test]
fn cli_releases_completions_empty() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["releases", "completions"])
        .assert()
        .success()
        .stdout(predicate::str::contains("latest"));
}

#[test]
fn cli_releases_completions_with_versions() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.0.0")).unwrap();
    fs::create_dir_all(versions_dir.join("4.2.3")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "completions"])
        .assert()
        .success()
        .stdout(predicate::str::contains("latest"))
        .stdout(predicate::str::contains("4.0.0"))
        .stdout(predicate::str::contains("4.2.3"));
}

#[test]
fn cli_releases_completions_excludes_alphas() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.2.3")).unwrap();
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.132057c7")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "completions"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.2.3"))
        .stdout(predicate::str::contains("4.3.0-alpha").not());
}

#[test]
fn cli_releases_completions_with_shell_bash() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.2.3")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "completions", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("latest"))
        .stdout(predicate::str::contains("4.2.3"));
}

#[test]
fn cli_releases_completions_with_shell_zsh() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.2.3")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "completions", "--shell", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("latest"))
        .stdout(predicate::str::contains("4.2.3"));
}

#[test]
fn cli_releases_completions_with_shell_nu() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.2.3")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "completions", "--shell", "nu"])
        .assert()
        .success()
        .stdout(predicate::str::contains("latest"))
        .stdout(predicate::str::contains("4.2.3"));
}

#[test]
fn cli_releases_install_already_exists() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "install", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already installed"));
}

#[test]
fn cli_releases_use_not_installed_shows_install_hint() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["releases", "use", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_releases_use_installed() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(version_dir.join("sbin")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "use", "4.2.3", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("export PATH="))
        .stdout(predicate::str::contains("4.2.3"));
}

#[test]
fn cli_releases_use_with_shell_flag() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(version_dir.join("sbin")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "use", "4.2.3", "--shell", "nu"])
        .assert()
        .success()
        .stdout(predicate::str::contains("$env.PATH"));
}

#[test]
fn cli_releases_use_with_frm_shell_env() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(version_dir.join("sbin")).unwrap();

    frm_cmd_with_dir(&temp)
        .env("FRM_SHELL", "nu")
        .args(["releases", "use", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("$env.PATH"));
}

#[test]
fn cli_default_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["default", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_default_installed() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["default", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Default version set to 4.2.3"));
}

#[test]
fn cli_default_updates_config() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["default", "4.2.3"])
        .assert()
        .success();

    let config_content = fs::read_to_string(temp.path().join("config.toml")).unwrap();
    assert!(config_content.contains("major = 4"));
    assert!(config_content.contains("minor = 2"));
    assert!(config_content.contains("patch = 3"));

    let default_content = fs::read_to_string(temp.path().join("default")).unwrap();
    assert_eq!(default_content.trim(), "4.2.3");
}

#[test]
fn cli_releases_list_marks_default() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.0.0")).unwrap();
    fs::create_dir_all(versions_dir.join("4.2.3")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["default", "4.2.3"])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["releases", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[ ] 4.0.0"))
        .stdout(predicate::str::contains("[*] 4.2.3"));
}

#[test]
fn cli_releases_uninstall_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["releases", "uninstall", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_releases_uninstall_installed() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "uninstall", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("RabbitMQ 4.2.3 uninstalled"));

    assert!(!version_dir.exists());
}

#[test]
fn cli_releases_uninstall_alias() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "rm", "4.2.3"])
        .assert()
        .success();

    assert!(!version_dir.exists());
}

#[test]
fn cli_releases_uninstall_rejects_alpha() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.3.0-alpha.abc123");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "uninstall", "4.3.0-alpha.abc123"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("expected a non-alpha version"));
}

#[test]
fn cli_releases_uninstall_clears_default() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["default", "4.2.3"])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["releases", "uninstall", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Cleared default version"));

    assert!(!temp.path().join("default").exists());
}

#[test]
fn cli_releases_reinstall_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["releases", "reinstall", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_releases_reinstall_no_version() {
    let temp = TempDir::new().unwrap();
    let work_dir = temp.path().join("empty");
    fs::create_dir_all(&work_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .current_dir(&work_dir)
        .args(["releases", "reinstall"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no version specified"));
}

#[test]
fn cli_releases_reinstall_help() {
    frm_cmd()
        .args(["releases", "reinstall", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Reinstall a stable RabbitMQ release",
        ))
        .stdout(predicate::str::contains("fresh copy"));
}

#[test]
fn cli_shell_env_bash() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["shell", "env", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("frm initialization for bash"));
}

#[test]
fn cli_shell_env_zsh() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["shell", "env", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("frm initialization for zsh"));
}

#[test]
fn cli_shell_env_nu() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["shell", "env", "nu"])
        .assert()
        .success()
        .stdout(predicate::str::contains("frm initialization for nushell"));
}

#[test]
fn cli_shell_completions_bash() {
    frm_cmd()
        .args(["shell", "completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("_frm"));
}

#[test]
fn cli_shell_completions_zsh() {
    frm_cmd()
        .args(["shell", "completions", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("#compdef"));
}

#[test]
fn cli_shell_completions_fish() {
    frm_cmd()
        .args(["shell", "completions", "fish"])
        .assert()
        .success()
        .stdout(predicate::str::contains("complete -c frm"));
}

#[test]
fn cli_shell_completions_powershell() {
    frm_cmd()
        .args(["shell", "completions", "powershell"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Register-ArgumentCompleter"));
}

#[test]
fn cli_shell_completions_elvish() {
    frm_cmd()
        .args(["shell", "completions", "elvish"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "set edit:completion:arg-completer[frm]",
        ));
}

#[test]
fn cli_shell_completions_nushell() {
    frm_cmd()
        .args(["shell", "completions", "nushell"])
        .assert()
        .success()
        .stdout(predicate::str::contains("module completions"));
}

#[test]
fn cli_shell_completions_nushell_alias() {
    frm_cmd()
        .args(["shell", "completions", "nu"])
        .assert()
        .success()
        .stdout(predicate::str::contains("module completions"));
}

#[test]
fn cli_shell_requires_subcommand() {
    frm_cmd()
        .args(["shell"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage: frm shell"));
}

#[test]
fn cli_help_shows_shell_command() {
    frm_cmd()
        .args(["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("shell"))
        .stdout(predicate::str::contains("Shell-related operations"));
}

#[test]
fn cli_shell_help_shows_subcommands() {
    frm_cmd()
        .args(["shell", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("completions"))
        .stdout(predicate::str::contains("env"));
}

#[test]
fn cli_shell_completions_invalid_shell() {
    frm_cmd()
        .args(["shell", "completions", "invalid"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value 'invalid'"));
}

#[test]
fn cli_shell_env_invalid_shell() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["shell", "env", "invalid"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value 'invalid'"));
}

#[test]
fn cli_shell_completions_auto_detects_shell() {
    frm_cmd().args(["shell", "completions"]).assert().success();
}

#[test]
fn cli_shell_env_requires_shell() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["shell", "env"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("<shell>"));
}

#[test]
fn cli_releases_use_invalid_version_format() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["releases", "use", "invalid"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid version format"));
}

#[test]
fn cli_inspect_no_version() {
    let temp = TempDir::new().unwrap();
    let work_dir = temp.path().join("empty");
    fs::create_dir_all(&work_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .current_dir(&work_dir)
        .args(["inspect", "rabbitmq.conf"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no version specified"));
}

#[test]
fn cli_cli_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["cli", "rabbitmqctl", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_cli_unknown_tool() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(version_dir.join("sbin")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["cli", "unknown-tool", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unknown tool"));
}

#[test]
fn cli_cli_tool_not_found() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(version_dir.join("sbin")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["cli", "rabbitmqctl", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("file not found"));
}

#[test]
fn cli_inspect_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["inspect", "rabbitmq.conf", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_inspect_unknown_file() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["inspect", "unknown.conf", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unknown config file"));
}

#[test]
fn cli_inspect_file_not_found() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(version_dir.join("etc").join("rabbitmq")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["inspect", "rabbitmq.conf", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("file not found"));
}

#[test]
fn cli_inspect_file_exists() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let etc_dir = version_dir.join("etc").join("rabbitmq");
    fs::create_dir_all(&etc_dir).unwrap();
    let config = r#"log.file.level = debug
vm_memory_high_watermark.absolute = 5GB
management.tcp.port = 15672
"#;
    fs::write(etc_dir.join("rabbitmq.conf"), config).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["inspect", "rabbitmq.conf", "-V", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("vm_memory_high_watermark"));
}

#[test]
fn cli_inspect_enabled_plugins() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let etc_dir = version_dir.join("etc").join("rabbitmq");
    fs::create_dir_all(&etc_dir).unwrap();
    fs::write(
        etc_dir.join("enabled_plugins"),
        "[rabbitmq_management,rabbitmq_prometheus,rabbitmq_stream].\n",
    )
    .unwrap();

    frm_cmd_with_dir(&temp)
        .args(["inspect", "enabled_plugins", "-V", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rabbitmq_management"))
        .stdout(predicate::str::contains("rabbitmq_prometheus"));
}

#[test]
fn cli_inspect_advanced_config() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let etc_dir = version_dir.join("etc").join("rabbitmq");
    fs::create_dir_all(&etc_dir).unwrap();
    let advanced_config = r#"[
  {kernel, [
    {inet_dist_listen_min, 25672},
    {inet_dist_listen_max, 25672}
  ]}
]."#;
    fs::write(etc_dir.join("advanced.config"), advanced_config).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["inspect", "advanced.config", "-V", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("inet_dist_listen_min"));
}

#[test]
fn cli_releases_logs_path_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["releases", "logs", "path", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_releases_logs_path_no_log_dir() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "logs", "path", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("file not found"));
}

#[test]
fn cli_releases_logs_path_no_log_file() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let log_dir = version_dir.join("var").join("log").join("rabbitmq");
    fs::create_dir_all(&log_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "logs", "path", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no log file found"));
}

#[test]
fn cli_releases_logs_path_found() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let log_dir = version_dir.join("var").join("log").join("rabbitmq");
    fs::create_dir_all(&log_dir).unwrap();
    fs::write(log_dir.join("rabbit@localhost.log"), "test log\n").unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "logs", "path", "-V", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rabbit@localhost.log"));
}

#[test]
fn cli_releases_logs_path_rejects_alpha() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.3.0-alpha.abc123");
    let log_dir = version_dir.join("var").join("log").join("rabbitmq");
    fs::create_dir_all(&log_dir).unwrap();
    fs::write(log_dir.join("rabbit@localhost.log"), "test log\n").unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "logs", "path", "-V", "4.3.0-alpha.abc123"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("expected a non-alpha version"));
}

#[test]
fn cli_releases_logs_tail_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["releases", "logs", "tail", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_releases_logs_tail_rejects_alpha() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.3.0-alpha.abc123");
    let log_dir = version_dir.join("var").join("log").join("rabbitmq");
    fs::create_dir_all(&log_dir).unwrap();
    fs::write(log_dir.join("rabbit@localhost.log"), "test log\n").unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "logs", "tail", "-V", "4.3.0-alpha.abc123"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("expected a non-alpha version"));
}

#[test]
fn cli_releases_logs_tail_default_lines() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let log_dir = version_dir.join("var").join("log").join("rabbitmq");
    fs::create_dir_all(&log_dir).unwrap();
    let log_content = r#"2026-01-16 19:29:14.752351-08:00 [info] <0.443.0> Message store is stopped
2026-01-16 19:29:14.753473-08:00 [info] <0.397.0> Message store is stopped
2026-01-16 19:29:14.754143-08:00 [info] <0.489.0> Message store is stopped
2026-01-16 19:29:14.754618-08:00 [info] <0.466.0> Message store is stopped
2026-01-16 19:29:14.756452-08:00 [debug] <0.208.0> Set stop reason to: normal
2026-01-16 19:29:14.756499-08:00 [debug] <0.208.0> Change boot state to `stopped`
2026-01-16 19:29:14.756590-08:00 [debug] <0.145.0> Boot state/systemd: sending status
2026-01-16 19:29:14.758366-08:00 [debug] <0.199.0> Stopping member in store
2026-01-16 19:29:14.758479-08:00 [debug] <0.248.0> RabbitMQ metadata store: terminating
2026-01-16 19:29:14.758513-08:00 [debug] <0.248.0> Terminating with reason 'shutdown'
2026-01-16 19:29:14.758699-08:00 [debug] <0.199.0> Wait for Ra server to exit
2026-01-16 19:29:14.758727-08:00 [debug] <0.199.0> Ra server already exited
2026-01-16 19:29:14.761437-08:00 [debug] <0.244.0> wal: terminating with shutdown
2026-01-16 19:29:14.761971-08:00 [debug] <0.239.0> ra: meta data store is terminating
2026-01-16 19:29:14.762092-08:00 [debug] <0.234.0> ra_log_ets: terminating with shutdown"#;
    fs::write(log_dir.join("rabbit@localhost.log"), log_content).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "logs", "tail", "-V", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Change boot state"))
        .stdout(predicate::str::contains("ra_log_ets: terminating"))
        .stdout(predicate::str::contains("Message store is stopped").not());
}

#[test]
fn cli_releases_logs_tail_custom_lines() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let log_dir = version_dir.join("var").join("log").join("rabbitmq");
    fs::create_dir_all(&log_dir).unwrap();
    let log_content = r#"2026-01-16 19:29:14.752351-08:00 [info] <0.443.0> accepting AMQP connection
2026-01-16 19:29:14.753473-08:00 [info] <0.397.0> connection established
2026-01-16 19:29:14.754143-08:00 [info] <0.489.0> channel opened
2026-01-16 19:29:14.754618-08:00 [info] <0.466.0> queue declared
2026-01-16 19:29:14.756452-08:00 [debug] <0.208.0> consumer registered
2026-01-16 19:29:14.756499-08:00 [debug] <0.208.0> basic.consume handled
2026-01-16 19:29:14.756590-08:00 [debug] <0.145.0> message published
2026-01-16 19:29:14.758366-08:00 [debug] <0.199.0> message delivered
2026-01-16 19:29:14.758479-08:00 [debug] <0.248.0> basic.ack received
2026-01-16 19:29:14.758513-08:00 [debug] <0.248.0> connection closed"#;
    fs::write(log_dir.join("rabbit@localhost.log"), log_content).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "logs", "tail", "-V", "4.2.3", "-n", "3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("message delivered"))
        .stdout(predicate::str::contains("basic.ack received"))
        .stdout(predicate::str::contains("connection closed"))
        .stdout(predicate::str::contains("consumer registered").not());
}

#[test]
fn cli_releases_logs_tail_long_flag() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let log_dir = version_dir.join("var").join("log").join("rabbitmq");
    fs::create_dir_all(&log_dir).unwrap();
    fs::write(
        log_dir.join("rabbit@localhost.log"),
        "2026-01-16 19:29:14.752351-08:00 [info] <0.443.0> RabbitMQ is starting\n",
    )
    .unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "logs", "tail", "-V", "4.2.3", "--lines", "5"])
        .assert()
        .success()
        .stdout(predicate::str::contains("RabbitMQ is starting"));
}

#[test]
fn cli_releases_logs_tail_more_lines_than_file() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let log_dir = version_dir.join("var").join("log").join("rabbitmq");
    fs::create_dir_all(&log_dir).unwrap();
    let log_content = r#"2026-01-16 19:29:14.752351-08:00 [info] <0.208.0> Starting RabbitMQ
2026-01-16 19:29:14.753473-08:00 [info] <0.208.0> node           : rabbit@localhost"#;
    fs::write(log_dir.join("rabbit@localhost.log"), log_content).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "logs", "tail", "-V", "4.2.3", "-n", "100"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Starting RabbitMQ"))
        .stdout(predicate::str::contains("rabbit@localhost"));
}

#[test]
fn cli_releases_logs_tail_empty_file() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let log_dir = version_dir.join("var").join("log").join("rabbitmq");
    fs::create_dir_all(&log_dir).unwrap();
    fs::write(log_dir.join("rabbit@localhost.log"), "").unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "logs", "tail", "-V", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn cli_releases_logs_no_subcommand() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["releases", "logs"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn cli_fg_no_subcommand() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .arg("fg")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn cli_fg_node_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["fg", "node", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_fg_node_server_not_found() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(version_dir.join("sbin")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["fg", "node", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("file not found"));
}

#[test]
fn cli_releases_help() {
    frm_cmd()
        .args(["releases", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Install or manage RabbitMQ releases",
        ));
}

#[test]
fn cli_alphas_help() {
    frm_cmd()
        .args(["alphas", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Install, manage, rotate alpha RabbitMQ releases",
        ));
}

#[test]
fn cli_alphas_install_help() {
    frm_cmd()
        .args(["alphas", "install", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Install an alpha RabbitMQ release",
        ))
        .stdout(predicate::str::contains("'latest'"));
}

#[test]
fn cli_alphas_install_requires_version() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["alphas", "install"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no version specified"));
}

#[test]
fn cli_alphas_install_rejects_non_alpha() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["alphas", "install", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("expected an alpha version"));
}

#[test]
fn cli_releases_install_rejects_alpha() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["releases", "install", "4.3.0-alpha.132057c7"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("expected a non-alpha version"));
}

#[test]
fn cli_alphas_uninstall_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["alphas", "uninstall", "4.3.0-alpha.132057c7"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_alphas_uninstall_installed() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.3.0-alpha.132057c7");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "uninstall", "4.3.0-alpha.132057c7"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "RabbitMQ 4.3.0-alpha.132057c7 uninstalled",
        ));

    assert!(!version_dir.exists());
}

#[test]
fn cli_alphas_uninstall_rejects_non_alpha() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "uninstall", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("expected an alpha version"));
}

#[test]
fn cli_alphas_uninstall_alias() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.3.0-alpha.abc123");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "rm", "4.3.0-alpha.abc123"])
        .assert()
        .success();

    assert!(!version_dir.exists());
}

#[test]
fn cli_alphas_reinstall_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["alphas", "reinstall", "4.3.0-alpha.132057c7"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_alphas_reinstall_rejects_non_alpha() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "reinstall", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("expected an alpha version"));
}

#[test]
fn cli_alphas_prune_no_alphas() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["alphas", "prune"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No alpha versions installed"));
}

#[test]
fn cli_alphas_prune_removes_alphas() {
    let temp = TempDir::new().unwrap();
    let alpha1 = temp.path().join("versions").join("4.3.0-alpha.132057c7");
    let alpha2 = temp.path().join("versions").join("4.3.0-alpha.abcd1234");
    let release = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&alpha1).unwrap();
    fs::create_dir_all(&alpha2).unwrap();
    fs::create_dir_all(&release).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "prune"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed 2 alpha version(s)"));

    assert!(!alpha1.exists());
    assert!(!alpha2.exists());
    assert!(release.exists());
}

#[test]
fn cli_alphas_prune_clears_default_if_alpha() {
    let temp = TempDir::new().unwrap();
    let alpha = temp.path().join("versions").join("4.3.0-alpha.132057c7");
    fs::create_dir_all(&alpha).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["default", "4.3.0-alpha.132057c7"])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "prune"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Cleared default version"));

    assert!(!temp.path().join("default").exists());
}

#[test]
fn cli_alphas_list_with_alphas() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.2.3")).unwrap();
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.132057c7")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.3.0-alpha.132057c7"))
        .stdout(predicate::str::contains("4.2.3").not());
}

#[test]
fn cli_alphas_list_empty() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["alphas", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "No alpha RabbitMQ releases installed",
        ));
}

#[test]
fn cli_alphas_list_alias() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["alphas", "ls"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "No alpha RabbitMQ releases installed",
        ));
}

#[test]
fn cli_alphas_list_marks_default() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.abc123")).unwrap();
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.def456")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["default", "4.3.0-alpha.def456"])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[ ] 4.3.0-alpha.abc123"))
        .stdout(predicate::str::contains("[*] 4.3.0-alpha.def456"));
}

#[test]
fn cli_alphas_completions_empty() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["alphas", "completions"])
        .assert()
        .success()
        .stdout(predicate::str::contains("latest"));
}

#[test]
fn cli_alphas_completions_with_alphas() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.abc123")).unwrap();
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.def456")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "completions"])
        .assert()
        .success()
        .stdout(predicate::str::contains("latest"))
        .stdout(predicate::str::contains("4.3.0-alpha.abc123"))
        .stdout(predicate::str::contains("4.3.0-alpha.def456"));
}

#[test]
fn cli_alphas_completions_excludes_releases() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.2.3")).unwrap();
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.132057c7")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "completions"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.3.0-alpha.132057c7"))
        .stdout(predicate::str::contains("4.2.3").not());
}

#[test]
fn cli_alphas_completions_with_shell_bash() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.abc123")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "completions", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("latest"))
        .stdout(predicate::str::contains("4.3.0-alpha.abc123"));
}

#[test]
fn cli_alphas_completions_with_shell_zsh() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.abc123")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "completions", "--shell", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("latest"))
        .stdout(predicate::str::contains("4.3.0-alpha.abc123"));
}

#[test]
fn cli_alphas_completions_with_shell_nu() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.abc123")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "completions", "--shell", "nu"])
        .assert()
        .success()
        .stdout(predicate::str::contains("latest"))
        .stdout(predicate::str::contains("4.3.0-alpha.abc123"));
}

#[test]
fn cli_alphas_clean_no_alphas() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["alphas", "clean", "--older-than", "2 weeks ago"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No alpha versions installed"));
}

#[test]
fn cli_alphas_clean_none_old_enough() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.abc123")).unwrap();

    let timestamps_file = temp.path().join("version_timestamps.json");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    fs::write(
        &timestamps_file,
        format!(r#"{{"4.3.0-alpha.abc123":{}}}"#, now),
    )
    .unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "clean", "--older-than", "2 weeks ago"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No alpha versions older than"));
}

#[test]
fn cli_alphas_clean_removes_old() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.abc123")).unwrap();
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.def456")).unwrap();

    let timestamps_file = temp.path().join("version_timestamps.json");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let old_ts = now - 60 * 60 * 24 * 30;
    fs::write(
        &timestamps_file,
        format!(
            r#"{{"4.3.0-alpha.abc123":{},"4.3.0-alpha.def456":{}}}"#,
            old_ts, now
        ),
    )
    .unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "clean", "--older-than", "2 weeks ago"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed 1 alpha version(s)"));

    assert!(!versions_dir.join("4.3.0-alpha.abc123").exists());
    assert!(versions_dir.join("4.3.0-alpha.def456").exists());
}

#[test]
fn cli_alphas_clean_clears_default_if_removed() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.abc123")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["default", "4.3.0-alpha.abc123"])
        .assert()
        .success();

    let timestamps_file = temp.path().join("version_timestamps.json");
    let old_ts = 0;
    fs::write(
        &timestamps_file,
        format!(r#"{{"4.3.0-alpha.abc123":{}}}"#, old_ts),
    )
    .unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "clean", "--older-than", "yesterday"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Cleared default version"));

    assert!(!temp.path().join("default").exists());
}

#[test]
fn cli_alphas_clean_requires_older_than() {
    frm_cmd()
        .args(["alphas", "clean"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--older-than"));
}

#[test]
fn cli_alphas_clean_invalid_time() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.abc123")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "clean", "--older-than", "not a valid time"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid date/time"));
}

#[test]
fn cli_alphas_logs_path_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["alphas", "logs", "path", "-V", "4.3.0-alpha.abc123"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_alphas_logs_path_found() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.3.0-alpha.abc123");
    let log_dir = version_dir.join("var").join("log").join("rabbitmq");
    fs::create_dir_all(&log_dir).unwrap();
    fs::write(log_dir.join("rabbit@localhost.log"), "test log\n").unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "logs", "path", "-V", "4.3.0-alpha.abc123"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rabbit@localhost.log"));
}

#[test]
fn cli_alphas_logs_path_rejects_release() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let log_dir = version_dir.join("var").join("log").join("rabbitmq");
    fs::create_dir_all(&log_dir).unwrap();
    fs::write(log_dir.join("rabbit@localhost.log"), "test log\n").unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "logs", "path", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("expected an alpha version"));
}

#[test]
fn cli_alphas_logs_tail_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["alphas", "logs", "tail", "-V", "4.3.0-alpha.abc123"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_alphas_logs_tail_found() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.3.0-alpha.abc123");
    let log_dir = version_dir.join("var").join("log").join("rabbitmq");
    fs::create_dir_all(&log_dir).unwrap();
    fs::write(
        log_dir.join("rabbit@localhost.log"),
        "2026-01-16 19:29:14.752351-08:00 [info] Alpha test log\n",
    )
    .unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "logs", "tail", "-V", "4.3.0-alpha.abc123"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alpha test log"));
}

#[test]
fn cli_alphas_logs_tail_rejects_release() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let log_dir = version_dir.join("var").join("log").join("rabbitmq");
    fs::create_dir_all(&log_dir).unwrap();
    fs::write(log_dir.join("rabbit@localhost.log"), "test log\n").unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "logs", "tail", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("expected an alpha version"));
}

#[test]
fn cli_alphas_logs_no_subcommand() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["alphas", "logs"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn cli_releases_path_no_version() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["releases", "path"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no version specified"));
}

#[test]
fn cli_releases_path_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["releases", "path", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_releases_path_installed() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.2.3")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "path", "-V", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.2.3"));
}

#[test]
fn cli_releases_path_rejects_alpha() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.abc123")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "path", "-V", "4.3.0-alpha.abc123"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("expected a non-alpha version"));
}

#[test]
fn cli_alphas_path_no_version() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["alphas", "path"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no version specified"));
}

#[test]
fn cli_alphas_path_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["alphas", "path", "-V", "4.3.0-alpha.abc123"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_alphas_path_installed() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.abc123")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "path", "-V", "4.3.0-alpha.abc123"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.3.0-alpha.abc123"));
}

#[test]
fn cli_alphas_path_rejects_release() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.2.3")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "path", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("expected an alpha version"));
}

#[test]
fn cli_releases_no_subcommand() {
    frm_cmd()
        .arg("releases")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn cli_alphas_no_subcommand() {
    frm_cmd()
        .arg("alphas")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn cli_releases_install_alias() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "i", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already installed"));
}

#[test]
fn cli_alphas_install_alias() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.3.0-alpha.abc123");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "i", "4.3.0-alpha.abc123"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already installed"));
}

#[test]
fn cli_releases_reinstall_rejects_alpha() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.3.0-alpha.abc123");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "reinstall", "4.3.0-alpha.abc123"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("expected a non-alpha version"));
}

#[test]
fn cli_releases_use_shows_install_hint_for_ga() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["releases", "use", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("frm releases install 4.2.3"));
}

#[test]
fn cli_releases_use_rejects_alpha_version() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["releases", "use", "4.3.0-alpha.abc123"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "this command only supports release versions",
        ));
}

#[test]
fn cli_conf_no_subcommand() {
    frm_cmd()
        .arg("conf")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn cli_conf_get_key_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["conf", "get-key", "listeners.tcp.default", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_conf_get_key_file_not_found() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(version_dir.join("etc").join("rabbitmq")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["conf", "get-key", "listeners.tcp.default", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("file not found"));
}

#[test]
fn cli_conf_get_key_found() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let etc_dir = version_dir.join("etc").join("rabbitmq");
    fs::create_dir_all(&etc_dir).unwrap();
    fs::write(
        etc_dir.join("rabbitmq.conf"),
        "listeners.tcp.default = 5672\n",
    )
    .unwrap();

    frm_cmd_with_dir(&temp)
        .args(["conf", "get-key", "listeners.tcp.default", "-V", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("5672"));
}

#[test]
fn cli_conf_get_key_not_found() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let etc_dir = version_dir.join("etc").join("rabbitmq");
    fs::create_dir_all(&etc_dir).unwrap();
    fs::write(
        etc_dir.join("rabbitmq.conf"),
        "listeners.tcp.default = 5672\n",
    )
    .unwrap();

    frm_cmd_with_dir(&temp)
        .args(["conf", "get-key", "nonexistent.key", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("key not found"));
}

#[test]
fn cli_conf_set_key_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args([
            "conf",
            "set-key",
            "listeners.tcp.default",
            "5673",
            "-V",
            "4.2.3",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_conf_set_key_new_key() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let etc_dir = version_dir.join("etc").join("rabbitmq");
    fs::create_dir_all(&etc_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "conf",
            "set-key",
            "listeners.tcp.default",
            "5672",
            "-V",
            "4.2.3",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("set"));

    let conf_content = fs::read_to_string(etc_dir.join("rabbitmq.conf")).unwrap();
    assert!(conf_content.contains("listeners.tcp.default = 5672"));
}

#[test]
fn cli_conf_set_key_update_existing() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let etc_dir = version_dir.join("etc").join("rabbitmq");
    fs::create_dir_all(&etc_dir).unwrap();
    fs::write(
        etc_dir.join("rabbitmq.conf"),
        "listeners.tcp.default = 5672\n",
    )
    .unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "conf",
            "set-key",
            "listeners.tcp.default",
            "5673",
            "-V",
            "4.2.3",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("updated"));

    let conf_content = fs::read_to_string(etc_dir.join("rabbitmq.conf")).unwrap();
    assert!(conf_content.contains("listeners.tcp.default = 5673"));
}

#[test]
fn cli_conf_set_key_unknown_key_rejected() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "conf",
            "set-key",
            "unknown.random.key",
            "value",
            "-V",
            "4.2.3",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unknown configuration key"));
}

#[test]
fn cli_conf_set_key_unknown_key_with_force() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let etc_dir = version_dir.join("etc").join("rabbitmq");
    fs::create_dir_all(&etc_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args([
            "conf",
            "set-key",
            "unknown.random.key",
            "value",
            "-V",
            "4.2.3",
            "--force",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("set"));

    let conf_content = fs::read_to_string(etc_dir.join("rabbitmq.conf")).unwrap();
    assert!(conf_content.contains("unknown.random.key = value"));
}

#[test]
fn cli_conf_set_key_invalid_format() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["conf", "set-key", "invalid..key", "value", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid key format"));
}

#[test]
fn cli_conf_get_key_pattern_match() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let etc_dir = version_dir.join("etc").join("rabbitmq");
    fs::create_dir_all(&etc_dir).unwrap();
    fs::write(
        etc_dir.join("rabbitmq.conf"),
        "listeners.tcp.default = 5672\nlisteners.tcp.amqp = 5673\nlisteners.ssl.default = 5671\n",
    )
    .unwrap();

    frm_cmd_with_dir(&temp)
        .args(["conf", "get-key", "listeners.tcp.*", "-V", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("listeners.tcp.default = 5672"))
        .stdout(predicate::str::contains("listeners.tcp.amqp = 5673"))
        .stdout(predicate::str::contains("listeners.ssl").not());
}

#[test]
fn cli_conf_get_key_pattern_no_match() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let etc_dir = version_dir.join("etc").join("rabbitmq");
    fs::create_dir_all(&etc_dir).unwrap();
    fs::write(
        etc_dir.join("rabbitmq.conf"),
        "listeners.tcp.default = 5672\n",
    )
    .unwrap();

    frm_cmd_with_dir(&temp)
        .args(["conf", "get-key", "listeners.ssl.*", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no keys matching pattern"));
}

#[test]
fn cli_releases_use_latest_no_versions() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["releases", "use", "latest"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no GA versions installed"));
}

#[test]
fn cli_releases_use_latest_only_alphas() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.abc123")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "use", "latest"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no GA versions installed"));
}

#[test]
fn cli_releases_use_latest_selects_highest_ga() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.0.0").join("sbin")).unwrap();
    fs::create_dir_all(versions_dir.join("4.2.3").join("sbin")).unwrap();
    fs::create_dir_all(versions_dir.join("4.1.0").join("sbin")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "use", "latest", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.2.3"));
}

#[test]
fn cli_releases_use_latest_ignores_prereleases() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.2.3").join("sbin")).unwrap();
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.1").join("sbin")).unwrap();
    fs::create_dir_all(versions_dir.join("4.3.0-beta.1").join("sbin")).unwrap();
    fs::create_dir_all(versions_dir.join("4.3.0-rc.1").join("sbin")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "use", "latest", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.2.3"));
}

#[test]
fn cli_releases_use_latest_case_insensitive() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.2.3").join("sbin")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "use", "LATEST", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.2.3"));

    frm_cmd_with_dir(&temp)
        .args(["releases", "use", "Latest", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.2.3"));
}

#[test]
fn cli_default_latest_no_versions() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["default", "latest"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no GA versions installed"));
}

#[test]
fn cli_default_latest_only_alphas() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.abc123")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["default", "latest"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no GA versions installed"));
}

#[test]
fn cli_default_latest_selects_highest_ga() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.0.0")).unwrap();
    fs::create_dir_all(versions_dir.join("4.2.3")).unwrap();
    fs::create_dir_all(versions_dir.join("4.1.0")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["default", "latest"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Default version set to 4.2.3"));
}

#[test]
fn cli_default_latest_ignores_prereleases() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.2.3")).unwrap();
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.1")).unwrap();
    fs::create_dir_all(versions_dir.join("4.3.0-beta.1")).unwrap();
    fs::create_dir_all(versions_dir.join("4.3.0-rc.1")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["default", "latest"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Default version set to 4.2.3"));
}

#[test]
fn cli_default_latest_case_insensitive() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.2.3")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["default", "LATEST"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Default version set to 4.2.3"));

    frm_cmd_with_dir(&temp)
        .args(["default", "Latest"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Default version set to 4.2.3"));
}

#[test]
fn cli_default_latest_updates_config_and_file() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.2.3")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["default", "latest"])
        .assert()
        .success();

    let config_content = fs::read_to_string(temp.path().join("config.toml")).unwrap();
    assert!(config_content.contains("major = 4"));
    assert!(config_content.contains("minor = 2"));
    assert!(config_content.contains("patch = 3"));

    let default_content = fs::read_to_string(temp.path().join("default")).unwrap();
    assert_eq!(default_content.trim(), "4.2.3");
}

#[test]
fn cli_releases_use_help_mentions_latest() {
    frm_cmd()
        .args(["releases", "use", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("latest"));
}

#[test]
fn cli_default_help_mentions_latest() {
    frm_cmd()
        .args(["default", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("latest"));
}

#[test]
fn cli_releases_uninstall_help_mentions_latest() {
    frm_cmd()
        .args(["releases", "uninstall", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("latest"));
}

#[test]
fn cli_alphas_uninstall_help_mentions_latest() {
    frm_cmd()
        .args(["alphas", "uninstall", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("latest"));
}

#[test]
fn cli_alphas_reinstall_help_mentions_latest() {
    frm_cmd()
        .args(["alphas", "reinstall", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("latest"));
}

#[test]
fn cli_releases_uninstall_latest_no_versions() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["releases", "uninstall", "latest"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no GA versions installed"));
}

#[test]
fn cli_releases_uninstall_latest_selects_highest() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.0.0")).unwrap();
    fs::create_dir_all(versions_dir.join("4.2.3")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "uninstall", "latest"])
        .assert()
        .success()
        .stdout(predicate::str::contains("RabbitMQ 4.2.3 uninstalled"));

    assert!(!versions_dir.join("4.2.3").exists());
    assert!(versions_dir.join("4.0.0").exists());
}

#[test]
fn cli_alphas_uninstall_latest_no_alphas() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["alphas", "uninstall", "latest"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no alpha versions installed"));
}

#[test]
fn cli_alphas_uninstall_latest_selects_highest() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.1")).unwrap();
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.2")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "uninstall", "latest"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "RabbitMQ 4.3.0-alpha.2 uninstalled",
        ));

    assert!(!versions_dir.join("4.3.0-alpha.2").exists());
    assert!(versions_dir.join("4.3.0-alpha.1").exists());
}

#[test]
fn cli_releases_use_latest_with_whitespace() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.2.3").join("sbin")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["releases", "use", "  latest  ", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.2.3"));
}

#[test]
fn cli_default_latest_with_whitespace() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.2.3")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["default", "  latest  "])
        .assert()
        .success()
        .stdout(predicate::str::contains("Default version set to 4.2.3"));
}

#[test]
fn cli_alphas_use_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["alphas", "use", "4.3.0-alpha.abc123"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_alphas_use_installed() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.3.0-alpha.abc123");
    fs::create_dir_all(version_dir.join("sbin")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "use", "4.3.0-alpha.abc123", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("export PATH="))
        .stdout(predicate::str::contains("4.3.0-alpha.abc123"));
}

#[test]
fn cli_alphas_use_with_shell_flag() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.3.0-alpha.abc123");
    fs::create_dir_all(version_dir.join("sbin")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "use", "4.3.0-alpha.abc123", "--shell", "nu"])
        .assert()
        .success()
        .stdout(predicate::str::contains("$env.PATH"));
}

#[test]
fn cli_alphas_use_with_frm_shell_env() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.3.0-alpha.abc123");
    fs::create_dir_all(version_dir.join("sbin")).unwrap();

    frm_cmd_with_dir(&temp)
        .env("FRM_SHELL", "nu")
        .args(["alphas", "use", "4.3.0-alpha.abc123"])
        .assert()
        .success()
        .stdout(predicate::str::contains("$env.PATH"));
}

#[test]
fn cli_alphas_use_rejects_release_version() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["alphas", "use", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "this command only supports alpha versions",
        ));
}

#[test]
fn cli_alphas_use_shows_install_hint() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["alphas", "use", "4.3.0-alpha.abc123"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("frm alphas install"));
}

#[test]
fn cli_alphas_use_latest_no_alphas() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["alphas", "use", "latest"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no alpha versions installed"));
}

#[test]
fn cli_alphas_use_latest_only_releases() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.2.3")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "use", "latest"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no alpha versions installed"));
}

#[test]
fn cli_alphas_use_latest_selects_highest_alpha() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.1").join("sbin")).unwrap();
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.2").join("sbin")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "use", "latest", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.3.0-alpha.2"));
}

#[test]
fn cli_alphas_use_latest_case_insensitive() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.abc123").join("sbin")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "use", "LATEST", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.3.0-alpha.abc123"));

    frm_cmd_with_dir(&temp)
        .args(["alphas", "use", "Latest", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.3.0-alpha.abc123"));
}

#[test]
fn cli_alphas_use_help_mentions_latest() {
    frm_cmd()
        .args(["alphas", "use", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("latest"));
}

#[test]
fn cli_alphas_use_invalid_version_format() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["alphas", "use", "invalid"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid version format"));
}

#[test]
fn cli_alphas_use_latest_with_whitespace() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.3.0-alpha.abc123").join("sbin")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["alphas", "use", "  latest  ", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4.3.0-alpha.abc123"));
}
