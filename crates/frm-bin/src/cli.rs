// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use clap::{Arg, ArgAction, Command};

pub use bel7_cli::CompletionShell;

use crate::commands::{CONFIG_FILES, EtcFile};
use crate::common::cli_tools::RABBITMQ_CLI_TOOLS;
use crate::shell::Shell;

pub fn build_cli() -> Command {
    Command::new("frm")
        .version(env!("CARGO_PKG_VERSION"))
        .disable_version_flag(true)
        .author("Michael S. Klishin")
        .about("Frakking RabbitMQ version Manager")
        .help_template("{name} {version}\n{about}\n\n{usage-heading} {usage}\n\n{all-args}")
        .arg_required_else_help(true)
        .subcommand(status_command())
        .subcommand(releases_command())
        .subcommand(alphas_command())
        .subcommand(tanzu_command())
        .subcommand(conf_command())
        .subcommand(default_command())
        .subcommand(cli_command())
        .subcommand(fg_command())
        .subcommand(bg_command())
        .subcommand(inspect_command())
        .subcommand(shell_command())
}

fn status_command() -> Command {
    Command::new("status")
        .about("Show frm status: active version, default, installed versions")
        .long_about(
            "Show frm status: active version, default, installed versions.\n\n\
            🟢 active in current shell (via 'frm releases use', 'frm alphas use', or 'frm tanzu use')\n\
            ⚪ default version",
        )
}

fn releases_command() -> Command {
    Command::new("releases")
        .about("Install or manage RabbitMQ releases (GA, RCs, betas); for alphas, see the 'alphas' command group")
        .arg_required_else_help(true)
        .subcommand(releases_list_command())
        .subcommand(releases_path_command())
        .subcommand(releases_logs_command())
        .subcommand(releases_install_command())
        .subcommand(releases_reinstall_command())
        .subcommand(releases_uninstall_command())
        .subcommand(releases_use_command())
        .subcommand(releases_cp_etc_file_command())
        .subcommand(releases_check_signature_command())
        .subcommand(releases_completions_command())
}

fn releases_use_command() -> Command {
    const HELP: &str = "Version to use (e.g., 4.2.3 or 'latest')";
    Command::new("use")
        .about("Output shell commands to use a specific release version")
        .long_about(
            "Output shell commands to use a specific release version.\n\n\
            Use 'latest' to select the most recent installed GA version.\n\n\
            bash/zsh: eval \"$(frm releases use [version])\"\n\
            nushell:  Use 'frm shell env nu' init script, then call 'frm-use [version]'",
        )
        .arg(positional_version_arg(HELP))
        .arg(version_opt_arg(HELP))
        .arg(
            Arg::new("shell")
                .long("shell")
                .short('s')
                .help("Shell type (bash, zsh, nu)")
                .value_parser(clap::value_parser!(Shell)),
        )
}

fn releases_completions_command() -> Command {
    Command::new("completions")
        .about("Output installed release versions for shell completion")
        .hide(true)
        .arg(
            Arg::new("shell")
                .long("shell")
                .short('s')
                .help("Shell type (bash, zsh, nu)")
                .value_parser(clap::value_parser!(Shell)),
        )
}

fn releases_list_command() -> Command {
    Command::new("list")
        .visible_alias("ls")
        .about("List installed stable RabbitMQ releases")
}

fn releases_path_command() -> Command {
    Command::new("path")
        .about("Show the local path of an installed release")
        .arg(version_arg())
}

fn releases_logs_command() -> Command {
    Command::new("logs")
        .about("Show RabbitMQ log file information for a release")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("path")
                .about("Show the path to the log file")
                .arg(version_arg()),
        )
        .subcommand(
            Command::new("tail")
                .about("Show the last lines of the log file")
                .arg(version_arg())
                .arg(
                    Arg::new("lines")
                        .long("lines")
                        .short('n')
                        .help("Number of lines to show")
                        .default_value("10")
                        .value_parser(clap::value_parser!(usize)),
                ),
        )
}

fn releases_install_command() -> Command {
    const HELP: &str = "Version to install (e.g., 4.2.3 or 4.2.0-rc.1)";
    Command::new("install")
        .visible_alias("i")
        .about("Install a stable RabbitMQ release")
        .long_about(
            "Install a stable RabbitMQ release (beta, rc, or GA).\n\n\
            Alpha versions are not allowed; use 'frm alphas install' instead.",
        )
        .arg(positional_version_arg(HELP))
        .arg(version_opt_arg(HELP))
        .arg(
            Arg::new("force")
                .long("force")
                .short('f')
                .help("Force reinstallation if version exists")
                .action(ArgAction::SetTrue),
        )
}

fn releases_reinstall_command() -> Command {
    const HELP: &str = "Version to reinstall (e.g., 4.2.3)";
    Command::new("reinstall")
        .about("Reinstall a stable RabbitMQ release")
        .long_about(
            "Reinstall a stable RabbitMQ release.\n\n\
            Removes the existing installation and downloads a fresh copy.",
        )
        .arg(positional_version_arg(HELP))
        .arg(version_opt_arg(HELP))
}

fn releases_uninstall_command() -> Command {
    const HELP: &str = "Version to uninstall (e.g., 4.2.3 or 'latest')";
    Command::new("uninstall")
        .visible_alias("rm")
        .about("Uninstall a stable RabbitMQ release")
        .long_about(
            "Uninstall a stable RabbitMQ release.\n\n\
            Use 'latest' to uninstall the most recent installed GA version.",
        )
        .arg(positional_version_arg(HELP))
        .arg(version_opt_arg(HELP))
}

fn releases_cp_etc_file_command() -> Command {
    cp_etc_file_command("Copy a configuration file to a stable release's etc/rabbitmq directory")
}

fn releases_check_signature_command() -> Command {
    Command::new("check-signature")
        .about("Verify the GPG signature of an installed release")
        .long_about(
            "Verify the GPG signature of an installed release.\n\n\
            Downloads the signature file from GitHub and verifies it using GPG.\n\
            Requires gpg to be installed and available in PATH.\n\n\
            Note: Alpha versions are not signed and cannot be verified.",
        )
        .arg(version_arg())
}

fn alphas_cp_etc_file_command() -> Command {
    cp_etc_file_command("Copy a configuration file to an alpha release's etc/rabbitmq directory")
}

fn cp_etc_file_command(about: &'static str) -> Command {
    Command::new("cp-etc-file")
        .about(about)
        .long_about(format!(
            "{}\n\n\
            Copies a local file to the version's etc/rabbitmq directory.\n\n\
            Supported files: {}",
            about,
            EtcFile::all_names().join(", ")
        ))
        .arg(
            Arg::new("local_file_path")
                .long("local-file-path")
                .help("Path to the local file to copy")
                .required(true)
                .value_name("PATH"),
        )
        .arg(
            Arg::new("etc_file")
                .long("etc-file")
                .help("Target configuration file name")
                .required(true)
                .value_name("FILE")
                .value_parser(EtcFile::all_names()),
        )
        .arg(version_arg())
}

fn alphas_command() -> Command {
    Command::new("alphas")
        .about("Install, manage, rotate alpha RabbitMQ releases")
        .arg_required_else_help(true)
        .subcommand(alphas_list_command())
        .subcommand(alphas_path_command())
        .subcommand(alphas_logs_command())
        .subcommand(alphas_install_command())
        .subcommand(alphas_reinstall_command())
        .subcommand(alphas_uninstall_command())
        .subcommand(alphas_use_command())
        .subcommand(alphas_cp_etc_file_command())
        .subcommand(alphas_prune_command())
        .subcommand(alphas_clean_command())
        .subcommand(alphas_completions_command())
}

fn alphas_use_command() -> Command {
    const HELP: &str = "Alpha version to use (e.g., 4.3.0-alpha.132057c7 or 'latest')";
    Command::new("use")
        .about("Output shell commands to use a specific alpha version")
        .long_about(
            "Output shell commands to use a specific alpha version.\n\n\
            Use 'latest' to select the most recent installed alpha version.\n\n\
            bash/zsh: eval \"$(frm alphas use [version])\"\n\
            nushell:  Use 'frm shell env nu' init script, then call 'frm-use [version]'",
        )
        .arg(positional_version_arg(HELP))
        .arg(version_opt_arg(HELP))
        .arg(
            Arg::new("shell")
                .long("shell")
                .short('s')
                .help("Shell type (bash, zsh, nu)")
                .value_parser(clap::value_parser!(Shell)),
        )
}

fn alphas_completions_command() -> Command {
    Command::new("completions")
        .about("Output installed alpha versions for shell completion")
        .hide(true)
        .arg(
            Arg::new("shell")
                .long("shell")
                .short('s')
                .help("Shell type (bash, zsh, nu)")
                .value_parser(clap::value_parser!(Shell)),
        )
}

fn alphas_list_command() -> Command {
    Command::new("list")
        .visible_alias("ls")
        .about("List installed alpha RabbitMQ releases")
}

fn alphas_path_command() -> Command {
    Command::new("path")
        .about("Show the local path of an installed alpha release")
        .arg(version_arg())
}

fn alphas_logs_command() -> Command {
    Command::new("logs")
        .about("Show RabbitMQ log file information for an alpha release")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("path")
                .about("Show the path to the log file")
                .arg(version_arg()),
        )
        .subcommand(
            Command::new("tail")
                .about("Show the last lines of the log file")
                .arg(version_arg())
                .arg(
                    Arg::new("lines")
                        .long("lines")
                        .short('n')
                        .help("Number of lines to show")
                        .default_value("10")
                        .value_parser(clap::value_parser!(usize)),
                ),
        )
}

fn alphas_install_command() -> Command {
    const HELP: &str = "Alpha version to install (e.g., 4.3.0-alpha.132057c7 or 'latest')";
    Command::new("install")
        .visible_alias("i")
        .about("Install an alpha RabbitMQ release")
        .long_about(
            "Install an alpha RabbitMQ release from rabbitmq/server-packages.\n\n\
            Use 'latest' to automatically install the most recent alpha release.",
        )
        .arg(positional_version_arg(HELP))
        .arg(version_opt_arg(HELP))
        .arg(
            Arg::new("force")
                .long("force")
                .short('f')
                .help("Force reinstallation if version exists")
                .action(ArgAction::SetTrue),
        )
}

fn alphas_reinstall_command() -> Command {
    const HELP: &str = "Alpha version to reinstall (e.g., 4.3.0-alpha.132057c7 or 'latest')";
    Command::new("reinstall")
        .about("Reinstall an alpha RabbitMQ release")
        .long_about(
            "Reinstall an alpha RabbitMQ release.\n\n\
            Removes the existing installation and downloads a fresh copy.\n\n\
            Use 'latest' to reinstall the most recent installed alpha version.",
        )
        .arg(positional_version_arg(HELP))
        .arg(version_opt_arg(HELP))
}

fn alphas_uninstall_command() -> Command {
    const HELP: &str = "Alpha version to uninstall (e.g., 4.3.0-alpha.132057c7 or 'latest')";
    Command::new("uninstall")
        .visible_alias("rm")
        .about("Uninstall an alpha RabbitMQ release")
        .long_about(
            "Uninstall an alpha RabbitMQ release.\n\n\
            Use 'latest' to uninstall the most recent installed alpha version.",
        )
        .arg(positional_version_arg(HELP))
        .arg(version_opt_arg(HELP))
}

fn alphas_prune_command() -> Command {
    Command::new("prune")
        .about("Remove all installed alpha releases")
        .long_about("Remove all installed alpha releases to free up disk space.")
}

fn alphas_clean_command() -> Command {
    Command::new("clean")
        .about("Remove alpha releases older than a specified time")
        .long_about(
            "Remove alpha releases older than a specified time.\n\n\
            The --older-than flag accepts human-readable time strings like:\n\
            - \"2 weeks ago\"\n\
            - \"1 month ago\"\n\
            - \"yesterday\"\n\
            - \"2025-01-01\" (absolute date)",
        )
        .arg(
            Arg::new("older_than")
                .long("older-than")
                .help("Remove alphas installed before this time (e.g., \"2 weeks ago\")")
                .required(true)
                .value_name("TIME"),
        )
}

fn tanzu_command() -> Command {
    Command::new("tanzu")
        .about("Install and manage Tanzu RabbitMQ from local tarballs")
        .arg_required_else_help(true)
        .subcommand(tanzu_install_command())
        .subcommand(tanzu_use_command())
}

fn tanzu_use_command() -> Command {
    const HELP: &str = "Version to use (e.g., 4.2.3 or 'latest')";
    Command::new("use")
        .about("Output shell commands to use a specific Tanzu RabbitMQ version")
        .long_about(
            "Output shell commands to use a specific Tanzu RabbitMQ version.\n\n\
            Use 'latest' to select the most recent installed GA version.\n\n\
            bash/zsh: eval \"$(frm tanzu use [version])\"\n\
            nushell:  Use 'frm shell env nu' init script, then call 'frm-use [version]'",
        )
        .arg(positional_version_arg(HELP))
        .arg(version_opt_arg(HELP))
        .arg(
            Arg::new("shell")
                .long("shell")
                .short('s')
                .help("Shell type (bash, zsh, nu)")
                .value_parser(clap::value_parser!(Shell)),
        )
}

fn tanzu_install_command() -> Command {
    Command::new("install")
        .visible_alias("i")
        .about("Install Tanzu RabbitMQ from a local tarball")
        .long_about(
            "Install Tanzu RabbitMQ from a local tarball.\n\n\
            Requires both the tarball path and the expected version.\n\
            The version in the tarball filename must match the specified version.\n\n\
            Supported formats: .tar.xz, .tar.gz, .tgz",
        )
        .arg(
            Arg::new("tarball_path")
                .long("local-tanzu-rabbitmq-tarball-path")
                .help("Path to the local Tanzu RabbitMQ tarball")
                .required(true)
                .value_name("PATH"),
        )
        .arg(
            Arg::new("version")
                .long("version")
                .short('V')
                .help("Expected RabbitMQ version (e.g., 4.2.3 or 4.2.3-rc.1)")
                .required(true)
                .value_name("VERSION"),
        )
        .arg(
            Arg::new("force")
                .long("force")
                .short('f')
                .help("Force reinstallation if version exists")
                .action(ArgAction::SetTrue),
        )
}

fn conf_command() -> Command {
    Command::new("conf")
        .about("Manage RabbitMQ configuration files")
        .arg_required_else_help(true)
        .subcommand(conf_get_key_command())
        .subcommand(conf_set_key_command())
}

fn conf_get_key_command() -> Command {
    Command::new("get-key")
        .about("Get a configuration key value from rabbitmq.conf")
        .long_about(
            "Get a configuration key value from rabbitmq.conf.\n\n\
            Supports pattern matching with * as a wildcard for a single segment:\n\n \
            * `listeners.tcp.*` matches `listeners.tcp.default`, `listeners.tcp.amqp`, etc.\n \
            * `log.*.level` matches `log.console.level`, `log.file.level`, etc.",
        )
        .arg(
            Arg::new("key")
                .help("Configuration key or pattern (e.g., listeners.tcp.* or heartbeat)")
                .required(true)
                .index(1),
        )
        .arg(version_arg())
}

fn conf_set_key_command() -> Command {
    Command::new("set-key")
        .about("Set a configuration key value in rabbitmq.conf")
        .long_about(
            "Set a configuration key value in rabbitmq.conf.\n\n\
            Keys are validated against the known RabbitMQ configuration schema.\n\
            Use --force to set unknown keys.",
        )
        .arg(
            Arg::new("key")
                .help("Configuration key (e.g., listeners.tcp.default)")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("value")
                .help("Value to set")
                .required(true)
                .index(2),
        )
        .arg(version_arg())
        .arg(
            Arg::new("force")
                .long("force")
                .short('f')
                .help("Set the key even if it's not recognized")
                .action(ArgAction::SetTrue),
        )
}

fn default_command() -> Command {
    const HELP: &str = "Version to set as default (e.g., 4.2.3 or 'latest')";
    Command::new("default")
        .about("Set the default RabbitMQ version")
        .long_about(
            "Set the default RabbitMQ version.\n\n\
            Use 'latest' to select the most recent installed GA version.",
        )
        .arg(positional_version_arg(HELP))
        .arg(version_opt_arg(HELP))
}

fn shell_command() -> Command {
    Command::new("shell")
        .about("Shell-related operations")
        .arg_required_else_help(true)
        .subcommand(shell_completions_command())
        .subcommand(shell_env_command())
}

fn shell_env_command() -> Command {
    Command::new("env")
        .about("Output shell initialization script")
        .long_about(
            "Output shell initialization script.\n\n\
            Add to your shell profile:\n\
            - bash: eval \"$(frm shell env bash)\" in ~/.bashrc\n\
            - zsh: eval \"$(frm shell env zsh)\" in ~/.zshrc\n\
            - nu: frm shell env nu | save -f ~/.local/frm/env.nu, then source in config.nu\n\n\
            After setup, use 'frm-use <version>' to switch versions.",
        )
        .arg(
            Arg::new("shell")
                .help("Shell type (bash, zsh, nu)")
                .required(true)
                .index(1)
                .value_parser(clap::value_parser!(Shell)),
        )
}

fn shell_completions_command() -> Command {
    Command::new("completions")
        .about("Generate shell completions")
        .long_about(
            "Generate shell completions.\n\n\
            If no shell is specified, attempts to detect the current shell from \
            environment variables (SHELL, NU_VERSION, etc.).",
        )
        .arg(
            Arg::new("shell")
                .help("Target shell (bash, elvish, fish, nushell, powershell, zsh); auto-detected if omitted")
                .index(1)
                .value_parser(clap::value_parser!(CompletionShell)),
        )
}

fn cli_command() -> Command {
    Command::new("cli")
        .about("Run a RabbitMQ CLI tool")
        .long_about(format!(
            "Run a RabbitMQ CLI tool from the specified version.\n\n\
            Available tools: {}\n\n\
            Use -- to separate tool arguments from frm options:\n\
            frm cli rabbitmqctl -V 4.2.3 -- status",
            RABBITMQ_CLI_TOOLS.join(", ")
        ))
        .trailing_var_arg(true)
        .arg(Arg::new("tool").help("Tool to run").required(true).index(1))
        .arg(version_arg())
        .arg(
            Arg::new("args")
                .help("Arguments to pass to the tool (after --)")
                .num_args(1..)
                .index(2),
        )
}

fn fg_command() -> Command {
    Command::new("fg")
        .about("Run RabbitMQ nodes in foreground")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("node")
                .about("Start RabbitMQ server in foreground")
                .arg(version_arg()),
        )
}

fn bg_command() -> Command {
    Command::new("bg")
        .about("Start and stop RabbitMQ nodes in background")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("start")
                .about("Start RabbitMQ server in background (detached)")
                .arg(version_arg()),
        )
        .subcommand(
            Command::new("stop")
                .about("Stop a running RabbitMQ node")
                .arg(version_arg()),
        )
}

fn inspect_command() -> Command {
    Command::new("inspect")
        .about("Inspect a RabbitMQ configuration file")
        .long_about(format!(
            "Inspect a RabbitMQ configuration file from the specified version.\n\n\
            Available files: {}",
            CONFIG_FILES.join(", ")
        ))
        .arg(
            Arg::new("file")
                .help("Configuration file to inspect")
                .required(true)
                .index(1),
        )
        .arg(version_arg())
}

fn version_arg() -> Arg {
    Arg::new("version")
        .long("version")
        .short('V')
        .help("RabbitMQ version to use")
        .value_name("VERSION")
}

fn positional_version_arg(help: &'static str) -> Arg {
    Arg::new("version").help(help).index(1)
}

fn version_opt_arg(help: &'static str) -> Arg {
    Arg::new("version_opt")
        .long("version")
        .short('V')
        .help(format!(
            "{}; takes precedence over the positional argument",
            help
        ))
        .value_name("VERSION")
}

pub fn get_version_arg(matches: &clap::ArgMatches) -> Option<&String> {
    matches
        .get_one::<String>("version_opt")
        .or_else(|| matches.get_one::<String>("version"))
}
