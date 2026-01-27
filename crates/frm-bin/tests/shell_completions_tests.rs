// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::OsStr;
use std::process::Command;

use assert_cmd::assert::Assert;
#[allow(deprecated)]
use assert_cmd::cargo::cargo_bin;
use bel7_cli::CommandShellExt;
use predicates::prelude::*;

fn run_with_shell_env<I, S>(args: I, shell_path: &str) -> Assert
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd = Command::new(cargo_bin!("frm"));
    cmd.clear_shell_detection_env();
    cmd.env("SHELL", shell_path);
    cmd.args(args);
    Assert::new(cmd.output().unwrap())
}

fn run_with_nu_version<I, S>(args: I) -> Assert
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd = Command::new(cargo_bin!("frm"));
    cmd.clear_shell_detection_env();
    cmd.env("NU_VERSION", "0.100.0");
    cmd.args(args);
    Assert::new(cmd.output().unwrap())
}

fn run_without_shell_env<I, S>(args: I) -> Assert
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd = Command::new(cargo_bin!("frm"));
    cmd.clear_shell_detection_env();
    cmd.args(args);
    Assert::new(cmd.output().unwrap())
}

#[test]
fn shell_completions_auto_detects_bash() {
    run_with_shell_env(["shell", "completions"], "/bin/bash")
        .success()
        .stdout(predicate::str::contains("_frm"));
}

#[test]
fn shell_completions_auto_detects_zsh() {
    run_with_shell_env(["shell", "completions"], "/usr/bin/zsh")
        .success()
        .stdout(predicate::str::contains("#compdef"));
}

#[test]
fn shell_completions_auto_detects_fish() {
    run_with_shell_env(["shell", "completions"], "/usr/local/bin/fish")
        .success()
        .stdout(predicate::str::contains("complete -c frm"));
}

#[test]
fn shell_completions_auto_detects_elvish() {
    run_with_shell_env(["shell", "completions"], "/usr/bin/elvish")
        .success()
        .stdout(predicate::str::contains(
            "set edit:completion:arg-completer[frm]",
        ));
}

#[test]
fn shell_completions_auto_detects_nushell_via_nu_version() {
    run_with_nu_version(["shell", "completions"])
        .success()
        .stdout(predicate::str::contains("module completions"));
}

#[test]
fn shell_completions_auto_detects_nushell_via_shell_path() {
    run_with_shell_env(["shell", "completions"], "/usr/bin/nu")
        .success()
        .stdout(predicate::str::contains("module completions"));
}

#[test]
fn shell_completions_auto_detects_powershell() {
    run_with_shell_env(["shell", "completions"], "/usr/local/bin/pwsh")
        .success()
        .stdout(predicate::str::contains("Register-ArgumentCompleter"));
}

#[test]
fn shell_completions_defaults_to_bash_when_unknown() {
    run_with_shell_env(["shell", "completions"], "/usr/bin/unknown-shell")
        .success()
        .stdout(predicate::str::contains("_frm"));
}

#[test]
fn shell_completions_defaults_to_bash_when_no_shell_env() {
    run_without_shell_env(["shell", "completions"])
        .success()
        .stdout(predicate::str::contains("_frm"));
}

#[test]
fn shell_completions_explicit_shell_overrides_detection() {
    run_with_shell_env(["shell", "completions", "zsh"], "/bin/bash")
        .success()
        .stdout(predicate::str::contains("#compdef"));
}

#[test]
fn shell_completions_nu_version_takes_priority_over_shell() {
    let mut cmd = Command::new(cargo_bin!("frm"));
    cmd.clear_shell_detection_env();
    cmd.env("NU_VERSION", "0.100.0");
    cmd.env("SHELL", "/bin/bash");
    cmd.args(["shell", "completions"]);
    Assert::new(cmd.output().unwrap())
        .success()
        .stdout(predicate::str::contains("module completions"));
}
