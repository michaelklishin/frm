// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::process;

use bel7_cli::{ExitCode, ExitCodeProvider, print_error};
use clap_complete::Shell as CompletionShell;

use frm::cli::build_cli;
use frm::commands;
use frm::errors::Error;
use frm::paths::Paths;
use frm::shell::Shell;
use frm::version::Version;
use frm::version_file;

fn resolve_version(version_arg: Option<&String>) -> Result<Version, Error> {
    if let Some(v) = version_arg {
        return v.parse();
    }

    version_file::find_version().ok_or_else(|| {
        Error::InvalidVersion("no version specified and no .tool-versions found".into())
    })
}

#[tokio::main]
async fn main() {
    let matches = build_cli().get_matches();

    let paths = match Paths::new() {
        Ok(p) => p,
        Err(e) => {
            print_error(e.to_string());
            process::exit(ExitCode::Config as i32);
        }
    };

    let result = match matches.subcommand() {
        Some(("list", _)) => commands::list(&paths),

        Some(("install", sub)) => {
            let version_arg = sub.get_one::<String>("version");
            let force = sub.get_flag("force");

            match resolve_version(version_arg) {
                Ok(version) => commands::install(&paths, &version, force).await,
                Err(e) => Err(e),
            }
        }

        Some(("use", sub)) => {
            let version_arg = sub.get_one::<String>("version");
            let shell = sub.get_one::<Shell>("shell").copied();

            match resolve_version(version_arg) {
                Ok(version) => commands::use_version(&paths, &version, shell),
                Err(e) => Err(e),
            }
        }

        Some(("default", sub)) => {
            let version_str = sub.get_one::<String>("version").unwrap();

            match version_str.parse::<Version>() {
                Ok(version) => commands::default(&paths, &version),
                Err(e) => Err(e),
            }
        }

        Some(("uninstall", sub)) => {
            let version_str = sub.get_one::<String>("version").unwrap();

            match version_str.parse::<Version>() {
                Ok(version) => commands::uninstall(&paths, &version),
                Err(e) => Err(e),
            }
        }

        Some(("reinstall", sub)) => {
            let version_arg = sub.get_one::<String>("version");

            match resolve_version(version_arg) {
                Ok(version) => commands::reinstall(&paths, &version).await,
                Err(e) => Err(e),
            }
        }

        Some(("cli", sub)) => {
            let tool = sub.get_one::<String>("tool").unwrap();
            let version_arg = sub.get_one::<String>("version");
            let args: Vec<String> = sub
                .get_many::<String>("args")
                .map(|v| v.cloned().collect())
                .unwrap_or_default();

            match resolve_version(version_arg) {
                Ok(version) => commands::cli(&paths, &version, tool, &args),
                Err(e) => Err(e),
            }
        }

        Some(("fg", sub)) => match sub.subcommand() {
            Some(("node", fg_sub)) => {
                let version_arg = fg_sub.get_one::<String>("version");

                match resolve_version(version_arg) {
                    Ok(version) => commands::fg_node(&paths, &version),
                    Err(e) => Err(e),
                }
            }
            _ => Ok(()),
        },

        Some(("show", sub)) => {
            let file = sub.get_one::<String>("file").unwrap();
            let version_arg = sub.get_one::<String>("version");

            match resolve_version(version_arg) {
                Ok(version) => commands::show(&paths, &version, file),
                Err(e) => Err(e),
            }
        }

        Some(("logs", sub)) => match sub.subcommand() {
            Some(("path", logs_sub)) => {
                let version_arg = logs_sub.get_one::<String>("version");

                match resolve_version(version_arg) {
                    Ok(version) => commands::logs_path(&paths, &version),
                    Err(e) => Err(e),
                }
            }
            Some(("tail", logs_sub)) => {
                let version_arg = logs_sub.get_one::<String>("version");
                let lines = *logs_sub.get_one::<usize>("lines").unwrap();

                match resolve_version(version_arg) {
                    Ok(version) => commands::logs_tail(&paths, &version, lines),
                    Err(e) => Err(e),
                }
            }
            _ => Ok(()),
        },

        Some(("env", sub)) => {
            let shell = sub.get_one::<Shell>("shell").unwrap();
            commands::env(&paths, *shell)
        }

        Some(("completions", sub)) => {
            let shell = sub.get_one::<CompletionShell>("shell").unwrap();
            commands::completions(*shell)
        }

        _ => Ok(()),
    };

    match result {
        Ok(()) => process::exit(ExitCode::Ok as i32),
        Err(e) => {
            print_error(e.to_string());
            process::exit(e.exit_code() as i32);
        }
    }
}
