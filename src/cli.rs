// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use clap::{Arg, ArgAction, Command};
use clap_complete::Shell as CompletionShell;

use crate::commands::{CONFIG_FILES, RABBITMQ_TOOLS};
use crate::shell::Shell;

pub fn build_cli() -> Command {
    Command::new("frm")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Michael S. Klishin")
        .about("Frakking RabbitMQ version Manager")
        .arg_required_else_help(true)
        .subcommand(list_command())
        .subcommand(install_command())
        .subcommand(use_command())
        .subcommand(default_command())
        .subcommand(uninstall_command())
        .subcommand(reinstall_command())
        .subcommand(cli_command())
        .subcommand(fg_command())
        .subcommand(show_command())
        .subcommand(logs_command())
        .subcommand(env_command())
        .subcommand(completions_command())
}

fn list_command() -> Command {
    Command::new("list")
        .visible_alias("ls")
        .about("List installed RabbitMQ versions")
}

fn install_command() -> Command {
    Command::new("install")
        .about("Install a RabbitMQ version")
        .long_about(
            "Install a RabbitMQ version.\n\n\
            If no version is specified, reads from .tool-versions file.",
        )
        .arg(
            Arg::new("version")
                .help("Version to install (e.g., 4.2.3), or reads from .tool-versions")
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

fn use_command() -> Command {
    Command::new("use")
        .about("Output shell commands to use a specific version")
        .long_about(
            "Output shell commands to use a specific version.\n\n\
            If no version is specified, reads from .tool-versions file.\n\n\
            bash/zsh: eval \"$(frm use [version])\"\n\
            nushell:  Use 'frm env nu' init script, then call 'frm-use [version]'",
        )
        .arg(
            Arg::new("version")
                .help("Version to use (e.g., 4.2.3), or reads from .tool-versions")
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
        .arg(
            Arg::new("version")
                .help("Version to set as default (e.g., 4.2.3)")
                .required(true)
                .index(1),
        )
}

fn uninstall_command() -> Command {
    Command::new("uninstall")
        .visible_alias("rm")
        .about("Uninstall a RabbitMQ version")
        .arg(
            Arg::new("version")
                .help("Version to uninstall (e.g., 4.2.3)")
                .required(true)
                .index(1),
        )
}

fn reinstall_command() -> Command {
    Command::new("reinstall")
        .about("Reinstall a RabbitMQ version")
        .long_about(
            "Reinstall a RabbitMQ version.\n\n\
            Removes the existing installation and downloads a fresh copy.\n\n\
            If no version is specified, reads from .tool-versions file.",
        )
        .arg(
            Arg::new("version")
                .help("Version to reinstall (e.g., 4.2.3), or reads from .tool-versions")
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
            If no version is specified, reads from .tool-versions file.\n\n\
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
                    If no version is specified, reads from .tool-versions file.",
                )
                .arg(version_arg()),
        )
}

fn show_command() -> Command {
    Command::new("show")
        .about("Show a RabbitMQ configuration file")
        .long_about(format!(
            "Show a RabbitMQ configuration file from the specified version.\n\n\
            Available files: {}\n\n\
            If no version is specified, reads from .tool-versions file.",
            CONFIG_FILES.join(", ")
        ))
        .arg(
            Arg::new("file")
                .help("Configuration file to show")
                .required(true)
                .index(1),
        )
        .arg(version_arg())
}

fn logs_command() -> Command {
    Command::new("logs")
        .about("Show RabbitMQ log file information")
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

fn version_arg() -> Arg {
    Arg::new("version")
        .long("version")
        .short('V')
        .help("RabbitMQ version to use, or reads from .tool-versions")
        .value_name("VERSION")
}

fn completions_command() -> Command {
    Command::new("completions")
        .about("Generate shell completions")
        .arg(
            Arg::new("shell")
                .help("Target shell")
                .required(true)
                .index(1)
                .value_parser(clap::value_parser!(CompletionShell)),
        )
}
