// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use clap::{Arg, ArgAction, Command, ValueEnum};

use crate::commands::{CONFIG_FILES, RABBITMQ_TOOLS};
use crate::shell::Shell;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum CompletionShell {
    Bash,
    Elvish,
    Fish,
    #[value(name = "nushell", alias = "nu")]
    Nushell,
    #[value(name = "powershell")]
    PowerShell,
    Zsh,
}

pub fn build_cli() -> Command {
    Command::new("frm")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Michael S. Klishin")
        .about("Frakking RabbitMQ version Manager")
        .arg_required_else_help(true)
        .subcommand(releases_command())
        .subcommand(alphas_command())
        .subcommand(tanzu_command())
        .subcommand(conf_command())
        .subcommand(use_command())
        .subcommand(default_command())
        .subcommand(cli_command())
        .subcommand(fg_command())
        .subcommand(inspect_command())
        .subcommand(env_command())
        .subcommand(completions_command())
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
        .subcommand(releases_completions_command())
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
        .long_about(
            "Show the local path of an installed release.\n\n\
            If no version is specified, tries to use the local .tool-versions file.",
        )
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
    Command::new("install")
        .visible_alias("i")
        .about("Install a stable RabbitMQ release")
        .long_about(
            "Install a stable RabbitMQ release (beta, rc, or GA).\n\n\
            If no version is specified, tries to use the local .tool-versions file.\n\n\
            Alpha versions are not allowed; use 'frm alphas install' instead.",
        )
        .arg(
            Arg::new("version")
                .help("Version to install (e.g., 4.2.3 or 4.2.0-rc.1)")
                .index(1),
        )
        .arg(
            Arg::new("force")
                .long("force")
                .short('f')
                .help("Force reinstallation if version exists")
                .action(ArgAction::SetTrue),
        )
}

fn releases_reinstall_command() -> Command {
    Command::new("reinstall")
        .about("Reinstall a stable RabbitMQ release")
        .long_about(
            "Reinstall a stable RabbitMQ release.\n\n\
            Removes the existing installation and downloads a fresh copy.\n\n\
            If no version is specified, tries to use the local .tool-versions file.",
        )
        .arg(
            Arg::new("version")
                .help("Version to reinstall (e.g., 4.2.3)")
                .index(1),
        )
}

fn releases_uninstall_command() -> Command {
    Command::new("uninstall")
        .visible_alias("rm")
        .about("Uninstall a stable RabbitMQ release")
        .long_about(
            "Uninstall a stable RabbitMQ release.\n\n\
            Use 'latest' to uninstall the most recent installed GA version.\n\n\
            If no version is specified, tries to use the local .tool-versions file.",
        )
        .arg(
            Arg::new("version")
                .help("Version to uninstall (e.g., 4.2.3 or 'latest')")
                .index(1),
        )
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
        .subcommand(alphas_prune_command())
        .subcommand(alphas_clean_command())
        .subcommand(alphas_completions_command())
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
        .long_about(
            "Show the local path of an installed alpha release.\n\n\
            If no version is specified, tries to use the local .tool-versions file.",
        )
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
    Command::new("install")
        .visible_alias("i")
        .about("Install an alpha RabbitMQ release")
        .long_about(
            "Install an alpha RabbitMQ release from rabbitmq/server-packages.\n\n\
            Use --latest to automatically install the most recent alpha release.",
        )
        .arg(
            Arg::new("version")
                .help("Alpha version to install (e.g., 4.3.0-alpha.132057c7)")
                .index(1)
                .conflicts_with("latest"),
        )
        .arg(
            Arg::new("latest")
                .long("latest")
                .short('l')
                .help("Install the most recent alpha release")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("force")
                .long("force")
                .short('f')
                .help("Force reinstallation if version exists")
                .action(ArgAction::SetTrue),
        )
}

fn alphas_reinstall_command() -> Command {
    Command::new("reinstall")
        .about("Reinstall an alpha RabbitMQ release")
        .long_about(
            "Reinstall an alpha RabbitMQ release.\n\n\
            Removes the existing installation and downloads a fresh copy.\n\n\
            Use 'latest' to reinstall the most recent installed alpha version.\n\n\
            If no version is specified, tries to use the local .tool-versions file.",
        )
        .arg(
            Arg::new("version")
                .help("Alpha version to reinstall (e.g., 4.3.0-alpha.132057c7 or 'latest')")
                .index(1),
        )
}

fn alphas_uninstall_command() -> Command {
    Command::new("uninstall")
        .visible_alias("rm")
        .about("Uninstall an alpha RabbitMQ release")
        .long_about(
            "Uninstall an alpha RabbitMQ release.\n\n\
            Use 'latest' to uninstall the most recent installed alpha version.\n\n\
            If no version is specified, tries to use the local .tool-versions file.",
        )
        .arg(
            Arg::new("version")
                .help("Alpha version to uninstall (e.g., 4.3.0-alpha.132057c7 or 'latest')")
                .index(1),
        )
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
        .about("Install Tanzu RabbitMQ from local tarballs")
        .arg_required_else_help(true)
        .subcommand(tanzu_install_command())
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
            * `log.*.level` matches `log.console.level`, `log.file.level`, etc.\n\n\
            If no version is specified, tries to use the local .tool-versions file.",
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
            If no version is specified, tries to use the local .tool-versions file.\n\n\
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

fn use_command() -> Command {
    Command::new("use")
        .about("Output shell commands to use a specific version")
        .long_about(
            "Output shell commands to use a specific version.\n\n\
            Use 'latest' to select the most recent installed GA version.\n\n\
            If no version is specified, tries to use the local .tool-versions file.\n\n\
            bash/zsh: eval \"$(frm use [version])\"\n\
            nushell:  Use 'frm env nu' init script, then call 'frm-use [version]'",
        )
        .arg(
            Arg::new("version")
                .help("Version to use (e.g., 4.2.3 or 'latest'), or uses .tool-versions")
                .index(1),
        )
        .arg(
            Arg::new("shell")
                .long("shell")
                .short('s')
                .help("Shell type (bash, zsh, nu)")
                .value_parser(clap::value_parser!(Shell)),
        )
}

fn default_command() -> Command {
    Command::new("default")
        .about("Set the default RabbitMQ version")
        .long_about(
            "Set the default RabbitMQ version.\n\n\
            Use 'latest' to select the most recent installed GA version.",
        )
        .arg(
            Arg::new("version")
                .help("Version to set as default (e.g., 4.2.3 or 'latest')")
                .required(true)
                .index(1),
        )
}

fn env_command() -> Command {
    Command::new("env")
        .about("Output shell initialization script")
        .long_about(
            "Output shell initialization script.\n\n\
            Add to your shell profile:\n\
            - bash: eval \"$(frm env bash)\" in ~/.bashrc\n\
            - zsh: eval \"$(frm env zsh)\" in ~/.zshrc\n\
            - nu: frm env nu | save -f ~/.local/frm/env.nu, then source in config.nu\n\n\
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

fn cli_command() -> Command {
    Command::new("cli")
        .about("Run a RabbitMQ CLI tool")
        .long_about(format!(
            "Run a RabbitMQ CLI tool from the specified version.\n\n\
            Available tools: {}\n\n\
            If no version is specified, tries to use the local .tool-versions file.\n\n\
            Use -- to separate tool arguments from frm options:\n\
            frm cli rabbitmqctl -V 4.2.3 -- status",
            RABBITMQ_TOOLS.join(", ")
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
                .long_about(
                    "Start RabbitMQ server in foreground.\n\n\
                    If no version is specified, tries to use the local .tool-versions file.",
                )
                .arg(version_arg()),
        )
}

fn inspect_command() -> Command {
    Command::new("inspect")
        .about("Inspect a RabbitMQ configuration file")
        .long_about(format!(
            "Inspect a RabbitMQ configuration file from the specified version.\n\n\
            Available files: {}\n\n\
            If no version is specified, tries to use the local .tool-versions file.",
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
        .help("RabbitMQ version to use, or uses .tool-versions")
        .value_name("VERSION")
}

fn completions_command() -> Command {
    Command::new("completions")
        .about("Generate shell completions")
        .arg(
            Arg::new("shell")
                .help("Target shell (bash, elvish, fish, nushell, powershell, zsh)")
                .required(true)
                .index(1)
                .value_parser(clap::value_parser!(CompletionShell)),
        )
}
