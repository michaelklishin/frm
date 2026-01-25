// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use tempfile::TempDir;

use frm::paths::Paths;
use frm::shell::Shell;
use frm::version::Version;

fn setup_temp_paths() -> (TempDir, Paths) {
    let temp_dir = TempDir::new().unwrap();
    let paths = Paths::with_base_dir(temp_dir.path().to_path_buf());
    (temp_dir, paths)
}

#[test]
fn shell_parse_bash() {
    let shell: Shell = "bash".parse().unwrap();
    assert_eq!(shell, Shell::Bash);
}

#[test]
fn shell_parse_zsh() {
    let shell: Shell = "zsh".parse().unwrap();
    assert_eq!(shell, Shell::Zsh);
}

#[test]
fn shell_parse_nu() {
    let shell: Shell = "nu".parse().unwrap();
    assert_eq!(shell, Shell::Nu);
}

#[test]
fn shell_parse_nushell() {
    let shell: Shell = "nushell".parse().unwrap();
    assert_eq!(shell, Shell::Nu);
}

#[test]
fn shell_parse_case_insensitive() {
    let bash: Shell = "BASH".parse().unwrap();
    assert_eq!(bash, Shell::Bash);

    let zsh: Shell = "ZSH".parse().unwrap();
    assert_eq!(zsh, Shell::Zsh);

    let nu: Shell = "NUSHELL".parse().unwrap();
    assert_eq!(nu, Shell::Nu);
}

#[test]
fn shell_parse_invalid() {
    let result: Result<Shell, _> = "fish".parse();
    assert!(result.is_err());
}

#[test]
fn shell_display() {
    assert_eq!(Shell::Bash.to_string(), "bash");
    assert_eq!(Shell::Zsh.to_string(), "zsh");
    assert_eq!(Shell::Nu.to_string(), "nu");
}

#[test]
fn shell_env_script_bash() {
    let (_temp, paths) = setup_temp_paths();
    let version = Version::new(4, 2, 3);
    let script = Shell::Bash.env_script(&paths, &version);

    assert!(script.contains("export PATH="));
    assert!(script.contains("export RABBITMQ_HOME="));
    assert!(script.contains("4.2.3"));
}

#[test]
fn shell_env_script_zsh() {
    let (_temp, paths) = setup_temp_paths();
    let version = Version::new(4, 2, 3);
    let script = Shell::Zsh.env_script(&paths, &version);

    assert!(script.contains("export PATH="));
    assert!(script.contains("export RABBITMQ_HOME="));
    assert!(script.contains("4.2.3"));
}

#[test]
fn shell_env_script_nu() {
    let (_temp, paths) = setup_temp_paths();
    let version = Version::new(4, 2, 3);
    let script = Shell::Nu.env_script(&paths, &version);

    assert!(script.contains("$env.PATH"));
    assert!(script.contains("$env.RABBITMQ_HOME"));
    assert!(script.contains("4.2.3"));
}

#[test]
fn shell_init_script_bash() {
    let (_temp, paths) = setup_temp_paths();
    let script = Shell::Bash.init_script(&paths);

    assert!(script.contains("frm initialization for bash"));
    assert!(script.contains("__frm_use"));
}

#[test]
fn shell_init_script_zsh() {
    let (_temp, paths) = setup_temp_paths();
    let script = Shell::Zsh.init_script(&paths);

    assert!(script.contains("frm initialization for zsh"));
    assert!(script.contains("__frm_use"));
}

#[test]
fn shell_init_script_nu() {
    let (_temp, paths) = setup_temp_paths();
    let script = Shell::Nu.init_script(&paths);

    assert!(script.contains("frm initialization for nushell"));
    assert!(script.contains("def --env frm-use"));
}

#[test]
fn shell_env_script_removes_old_paths_bash() {
    let (_temp, paths) = setup_temp_paths();
    let version = Version::new(4, 2, 3);
    let script = Shell::Bash.env_script(&paths, &version);

    assert!(script.contains("${PATH//*"));
    assert!(script.contains(r"\/versions\/*/}"));
}

#[test]
fn shell_env_script_removes_old_paths_nu() {
    let (_temp, paths) = setup_temp_paths();
    let version = Version::new(4, 2, 3);
    let script = Shell::Nu.env_script(&paths, &version);

    assert!(script.contains("where { |p| not ($p | str contains"));
    assert!(script.contains("/versions\")"));
}
