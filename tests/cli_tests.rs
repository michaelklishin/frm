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
        .stdout(predicate::str::contains(
            "Frakking RabbitMQ version Manager",
        ));
}

#[test]
fn cli_version_flag() {
    frm_cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("frm"));
}

#[test]
fn cli_list_empty() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("No RabbitMQ versions installed"));
}

#[test]
fn cli_list_alias() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .arg("ls")
        .assert()
        .success()
        .stdout(predicate::str::contains("No RabbitMQ versions installed"));
}

#[test]
fn cli_list_with_versions() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.0.0")).unwrap();
    fs::create_dir_all(versions_dir.join("4.2.3")).unwrap();

    frm_cmd_with_dir(&temp)
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("4.0.0"))
        .stdout(predicate::str::contains("4.2.3"));
}

#[test]
fn cli_install_already_exists() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["install", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already installed"));
}

#[test]
fn cli_use_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["use", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_use_installed() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(version_dir.join("sbin")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["use", "4.2.3", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("export PATH="))
        .stdout(predicate::str::contains("4.2.3"));
}

#[test]
fn cli_use_with_shell_flag() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(version_dir.join("sbin")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["use", "4.2.3", "--shell", "nu"])
        .assert()
        .success()
        .stdout(predicate::str::contains("$env.PATH"));
}

#[test]
fn cli_use_with_frm_shell_env() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(version_dir.join("sbin")).unwrap();

    frm_cmd_with_dir(&temp)
        .env("FRM_SHELL", "nu")
        .args(["use", "4.2.3"])
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
fn cli_list_marks_default() {
    let temp = TempDir::new().unwrap();
    let versions_dir = temp.path().join("versions");
    fs::create_dir_all(versions_dir.join("4.0.0")).unwrap();
    fs::create_dir_all(versions_dir.join("4.2.3")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["default", "4.2.3"])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("[ ] 4.0.0"))
        .stdout(predicate::str::contains("[*] 4.2.3"));
}

#[test]
fn cli_uninstall_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["uninstall", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_uninstall_installed() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["uninstall", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("RabbitMQ 4.2.3 uninstalled"));

    assert!(!version_dir.exists());
}

#[test]
fn cli_uninstall_alias() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["rm", "4.2.3"])
        .assert()
        .success();

    assert!(!version_dir.exists());
}

#[test]
fn cli_uninstall_clears_default() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["default", "4.2.3"])
        .assert()
        .success();

    frm_cmd_with_dir(&temp)
        .args(["uninstall", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Cleared default version"));

    assert!(!temp.path().join("default").exists());
}

#[test]
fn cli_reinstall_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["reinstall", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_reinstall_no_version_no_tool_versions() {
    let temp = TempDir::new().unwrap();
    let work_dir = temp.path().join("empty");
    fs::create_dir_all(&work_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .current_dir(&work_dir)
        .arg("reinstall")
        .assert()
        .failure()
        .stderr(predicate::str::contains("no version specified"));
}

#[test]
fn cli_reinstall_with_tool_versions_not_installed() {
    let temp = TempDir::new().unwrap();
    let work_dir = temp.path().join("project");
    fs::create_dir_all(&work_dir).unwrap();
    fs::write(work_dir.join(".tool-versions"), "rabbitmq 4.1.0\n").unwrap();

    frm_cmd_with_dir(&temp)
        .current_dir(&work_dir)
        .arg("reinstall")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_reinstall_help() {
    frm_cmd()
        .args(["reinstall", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Reinstall a RabbitMQ version"))
        .stdout(predicate::str::contains("fresh copy"));
}

#[test]
fn cli_env_bash() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["env", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("frm initialization for bash"));
}

#[test]
fn cli_env_zsh() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["env", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("frm initialization for zsh"));
}

#[test]
fn cli_env_nu() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["env", "nu"])
        .assert()
        .success()
        .stdout(predicate::str::contains("frm initialization for nushell"));
}

#[test]
fn cli_completions_bash() {
    frm_cmd()
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("_frm"));
}

#[test]
fn cli_completions_zsh() {
    frm_cmd()
        .args(["completions", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("#compdef"));
}

#[test]
fn cli_completions_fish() {
    frm_cmd()
        .args(["completions", "fish"])
        .assert()
        .success()
        .stdout(predicate::str::contains("complete -c frm"));
}

#[test]
fn cli_completions_powershell() {
    frm_cmd()
        .args(["completions", "powershell"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Register-ArgumentCompleter"));
}

#[test]
fn cli_completions_elvish() {
    frm_cmd()
        .args(["completions", "elvish"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "set edit:completion:arg-completer[frm]",
        ));
}

#[test]
fn cli_invalid_version_format() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["use", "invalid"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid version format"));
}

#[test]
fn cli_no_version_no_tool_versions() {
    let temp = TempDir::new().unwrap();
    let work_dir = temp.path().join("empty");
    fs::create_dir_all(&work_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .current_dir(&work_dir)
        .args(["show", "rabbitmq.conf"])
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
fn cli_show_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["show", "rabbitmq.conf", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_show_unknown_file() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["show", "unknown.conf", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unknown config file"));
}

#[test]
fn cli_show_file_not_found() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(version_dir.join("etc").join("rabbitmq")).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["show", "rabbitmq.conf", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("file not found"));
}

#[test]
fn cli_show_file_exists() {
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
        .args(["show", "rabbitmq.conf", "-V", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("vm_memory_high_watermark"));
}

#[test]
fn cli_show_enabled_plugins() {
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
        .args(["show", "enabled_plugins", "-V", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rabbitmq_management"))
        .stdout(predicate::str::contains("rabbitmq_prometheus"));
}

#[test]
fn cli_show_advanced_config() {
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
        .args(["show", "advanced.config", "-V", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("inet_dist_listen_min"));
}

#[test]
fn cli_logs_path_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["logs", "path", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_logs_path_no_log_dir() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    fs::create_dir_all(&version_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["logs", "path", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("file not found"));
}

#[test]
fn cli_logs_path_no_log_file() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let log_dir = version_dir.join("var").join("log").join("rabbitmq");
    fs::create_dir_all(&log_dir).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["logs", "path", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no log file found"));
}

#[test]
fn cli_logs_path_found() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let log_dir = version_dir.join("var").join("log").join("rabbitmq");
    fs::create_dir_all(&log_dir).unwrap();
    fs::write(log_dir.join("rabbit@localhost.log"), "test log\n").unwrap();

    frm_cmd_with_dir(&temp)
        .args(["logs", "path", "-V", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rabbit@localhost.log"));
}

#[test]
fn cli_logs_tail_not_installed() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .args(["logs", "tail", "-V", "4.2.3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn cli_logs_tail_default_lines() {
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
        .args(["logs", "tail", "-V", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Change boot state"))
        .stdout(predicate::str::contains("ra_log_ets: terminating"))
        .stdout(predicate::str::contains("Message store is stopped").not());
}

#[test]
fn cli_logs_tail_custom_lines() {
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
        .args(["logs", "tail", "-V", "4.2.3", "-n", "3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("message delivered"))
        .stdout(predicate::str::contains("basic.ack received"))
        .stdout(predicate::str::contains("connection closed"))
        .stdout(predicate::str::contains("consumer registered").not());
}

#[test]
fn cli_logs_tail_long_flag() {
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
        .args(["logs", "tail", "-V", "4.2.3", "--lines", "5"])
        .assert()
        .success()
        .stdout(predicate::str::contains("RabbitMQ is starting"));
}

#[test]
fn cli_logs_tail_more_lines_than_file() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let log_dir = version_dir.join("var").join("log").join("rabbitmq");
    fs::create_dir_all(&log_dir).unwrap();
    let log_content = r#"2026-01-16 19:29:14.752351-08:00 [info] <0.208.0> Starting RabbitMQ
2026-01-16 19:29:14.753473-08:00 [info] <0.208.0> node           : rabbit@localhost"#;
    fs::write(log_dir.join("rabbit@localhost.log"), log_content).unwrap();

    frm_cmd_with_dir(&temp)
        .args(["logs", "tail", "-V", "4.2.3", "-n", "100"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Starting RabbitMQ"))
        .stdout(predicate::str::contains("rabbit@localhost"));
}

#[test]
fn cli_logs_tail_empty_file() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.2.3");
    let log_dir = version_dir.join("var").join("log").join("rabbitmq");
    fs::create_dir_all(&log_dir).unwrap();
    fs::write(log_dir.join("rabbit@localhost.log"), "").unwrap();

    frm_cmd_with_dir(&temp)
        .args(["logs", "tail", "-V", "4.2.3"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn cli_logs_no_subcommand() {
    let temp = TempDir::new().unwrap();
    frm_cmd_with_dir(&temp)
        .arg("logs")
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
fn cli_show_with_tool_versions() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.1.0");
    let etc_dir = version_dir.join("etc").join("rabbitmq");
    fs::create_dir_all(&etc_dir).unwrap();
    let config = r#"log.file.level = info
vm_memory_high_watermark.relative = 0.6
cluster_formation.peer_discovery_backend = rabbit_peer_discovery_classic_config
"#;
    fs::write(etc_dir.join("rabbitmq.conf"), config).unwrap();

    let work_dir = temp.path().join("project");
    fs::create_dir_all(&work_dir).unwrap();
    fs::write(work_dir.join(".tool-versions"), "rabbitmq 4.1.0\n").unwrap();

    frm_cmd_with_dir(&temp)
        .current_dir(&work_dir)
        .args(["show", "rabbitmq.conf"])
        .assert()
        .success()
        .stdout(predicate::str::contains("vm_memory_high_watermark"));
}

#[test]
fn cli_logs_path_with_tool_versions() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.1.0");
    let log_dir = version_dir.join("var").join("log").join("rabbitmq");
    fs::create_dir_all(&log_dir).unwrap();
    fs::write(log_dir.join("rabbit@myhost.log"), "").unwrap();

    let work_dir = temp.path().join("project");
    fs::create_dir_all(&work_dir).unwrap();
    fs::write(work_dir.join(".tool-versions"), "rabbitmq 4.1.0\n").unwrap();

    frm_cmd_with_dir(&temp)
        .current_dir(&work_dir)
        .args(["logs", "path"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rabbit@myhost.log"));
}

#[test]
fn cli_logs_tail_with_tool_versions() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.1.0");
    let log_dir = version_dir.join("var").join("log").join("rabbitmq");
    fs::create_dir_all(&log_dir).unwrap();
    fs::write(
        log_dir.join("rabbit@myhost.log"),
        "2026-01-16 19:29:14.752351-08:00 [info] <0.208.0> Server startup complete\n",
    )
    .unwrap();

    let work_dir = temp.path().join("project");
    fs::create_dir_all(&work_dir).unwrap();
    fs::write(work_dir.join(".tool-versions"), "rabbitmq 4.1.0\n").unwrap();

    frm_cmd_with_dir(&temp)
        .current_dir(&work_dir)
        .args(["logs", "tail"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Server startup complete"));
}

#[test]
fn cli_fg_node_with_tool_versions() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.1.0");
    fs::create_dir_all(version_dir.join("sbin")).unwrap();

    let work_dir = temp.path().join("project");
    fs::create_dir_all(&work_dir).unwrap();
    fs::write(work_dir.join(".tool-versions"), "rabbitmq 4.1.0\n").unwrap();

    frm_cmd_with_dir(&temp)
        .current_dir(&work_dir)
        .args(["fg", "node"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("file not found"));
}

#[test]
fn cli_cli_with_tool_versions() {
    let temp = TempDir::new().unwrap();
    let version_dir = temp.path().join("versions").join("4.1.0");
    fs::create_dir_all(version_dir.join("sbin")).unwrap();

    let work_dir = temp.path().join("project");
    fs::create_dir_all(&work_dir).unwrap();
    fs::write(work_dir.join(".tool-versions"), "rabbitmq 4.1.0\n").unwrap();

    frm_cmd_with_dir(&temp)
        .current_dir(&work_dir)
        .args(["cli", "rabbitmqctl"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("file not found"));
}
