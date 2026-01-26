// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::path::PathBuf;
use std::process;

use bel7_cli::{ExitCode, ExitCodeProvider, print_error, print_info};
use clap_complete::Shell as CompletionShell;

use frm::cli::build_cli;
use frm::commands;
use frm::errors::Error;
use frm::paths::Paths;
use frm::releases::find_latest_alpha;
use frm::shell::Shell;
use frm::version::Version;
use frm::version_file;

fn resolve_version(paths: &Paths, version_arg: Option<&String>) -> Result<Version, Error> {
    if let Some(v) = version_arg {
        let v = v.trim();
        if v.eq_ignore_ascii_case("latest") {
            return paths
                .latest_ga_version()?
                .ok_or(Error::NoGAVersionsInstalled);
        }
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
        Some(("releases", sub)) => match sub.subcommand() {
            Some(("list", _)) => commands::list_releases(&paths),
            Some(("path", path_sub)) => {
                let version_arg = path_sub.get_one::<String>("version");

                match resolve_version(&paths, version_arg) {
                    Ok(version) => commands::path_release(&paths, &version),
                    Err(e) => Err(e),
                }
            }
            Some(("install", install_sub)) => {
                let version_arg = install_sub.get_one::<String>("version");
                let force = install_sub.get_flag("force");

                match resolve_version(&paths, version_arg) {
                    Ok(version) => commands::install_release(&paths, &version, force).await,
                    Err(e) => Err(e),
                }
            }
            Some(("reinstall", reinstall_sub)) => {
                let version_arg = reinstall_sub.get_one::<String>("version");

                match resolve_version(&paths, version_arg) {
                    Ok(version) => commands::reinstall_release(&paths, &version).await,
                    Err(e) => Err(e),
                }
            }
            Some(("uninstall", uninstall_sub)) => {
                let version_str = uninstall_sub.get_one::<String>("version").unwrap();

                match version_str.parse::<Version>() {
                    Ok(version) => commands::uninstall_release(&paths, &version),
                    Err(e) => Err(e),
                }
            }
            Some(("logs", logs_sub)) => match logs_sub.subcommand() {
                Some(("path", path_sub)) => {
                    let version_arg = path_sub.get_one::<String>("version");

                    match resolve_version(&paths, version_arg) {
                        Ok(version) => commands::logs_path_release(&paths, &version),
                        Err(e) => Err(e),
                    }
                }
                Some(("tail", tail_sub)) => {
                    let version_arg = tail_sub.get_one::<String>("version");
                    let lines = *tail_sub.get_one::<usize>("lines").unwrap();

                    match resolve_version(&paths, version_arg) {
                        Ok(version) => commands::logs_tail_release(&paths, &version, lines),
                        Err(e) => Err(e),
                    }
                }
                _ => Ok(()),
            },
            _ => Ok(()),
        },

        Some(("alphas", sub)) => match sub.subcommand() {
            Some(("list", _)) => commands::list_alphas(&paths),
            Some(("path", path_sub)) => {
                let version_arg = path_sub.get_one::<String>("version");

                match resolve_version(&paths, version_arg) {
                    Ok(version) => commands::path_alpha(&paths, &version),
                    Err(e) => Err(e),
                }
            }
            Some(("install", install_sub)) => {
                let version_arg = install_sub.get_one::<String>("version");
                let latest = install_sub.get_flag("latest");
                let force = install_sub.get_flag("force");

                if latest {
                    print_info("Fetching latest alpha release...");
                    let client = reqwest::Client::new();
                    match find_latest_alpha(&client).await {
                        Ok(alpha) => {
                            print_info(format!("Found: {}", alpha.version));
                            commands::install_alpha(&paths, &alpha.version, force).await
                        }
                        Err(e) => Err(e),
                    }
                } else {
                    match version_arg {
                        Some(v) => match v.parse::<Version>() {
                            Ok(version) => commands::install_alpha(&paths, &version, force).await,
                            Err(e) => Err(e),
                        },
                        None => Err(Error::InvalidVersion(
                            "specify a version or use --latest".into(),
                        )),
                    }
                }
            }
            Some(("reinstall", reinstall_sub)) => {
                let version_str = reinstall_sub.get_one::<String>("version").unwrap();

                match version_str.parse::<Version>() {
                    Ok(version) => commands::reinstall_alpha(&paths, &version).await,
                    Err(e) => Err(e),
                }
            }
            Some(("uninstall", uninstall_sub)) => {
                let version_str = uninstall_sub.get_one::<String>("version").unwrap();

                match version_str.parse::<Version>() {
                    Ok(version) => commands::uninstall_alpha(&paths, &version),
                    Err(e) => Err(e),
                }
            }
            Some(("prune", _)) => commands::prune_alphas(&paths),
            Some(("clean", clean_sub)) => {
                let older_than = clean_sub.get_one::<String>("older_than").unwrap();
                commands::clean_alphas(&paths, older_than)
            }
            Some(("logs", logs_sub)) => match logs_sub.subcommand() {
                Some(("path", path_sub)) => {
                    let version_arg = path_sub.get_one::<String>("version");

                    match resolve_version(&paths, version_arg) {
                        Ok(version) => commands::logs_path_alpha(&paths, &version),
                        Err(e) => Err(e),
                    }
                }
                Some(("tail", tail_sub)) => {
                    let version_arg = tail_sub.get_one::<String>("version");
                    let lines = *tail_sub.get_one::<usize>("lines").unwrap();

                    match resolve_version(&paths, version_arg) {
                        Ok(version) => commands::logs_tail_alpha(&paths, &version, lines),
                        Err(e) => Err(e),
                    }
                }
                _ => Ok(()),
            },
            _ => Ok(()),
        },

        Some(("tanzu", sub)) => match sub.subcommand() {
            Some(("install", install_sub)) => {
                let tarball_path = install_sub
                    .get_one::<String>("tarball_path")
                    .map(PathBuf::from)
                    .unwrap();
                let version_str = install_sub.get_one::<String>("version").unwrap();
                let force = install_sub.get_flag("force");

                match version_str.parse::<Version>() {
                    Ok(version) => commands::tanzu_install(&paths, &tarball_path, &version, force),
                    Err(e) => Err(e),
                }
            }
            _ => Ok(()),
        },

        Some(("conf", sub)) => match sub.subcommand() {
            Some(("get-key", get_sub)) => {
                let key = get_sub.get_one::<String>("key").unwrap();
                let version_arg = get_sub.get_one::<String>("version");

                match resolve_version(&paths, version_arg) {
                    Ok(version) => commands::conf_get_key(&paths, &version, key),
                    Err(e) => Err(e),
                }
            }
            Some(("set-key", set_sub)) => {
                let key = set_sub.get_one::<String>("key").unwrap();
                let value = set_sub.get_one::<String>("value").unwrap();
                let version_arg = set_sub.get_one::<String>("version");
                let force = set_sub.get_flag("force");

                match resolve_version(&paths, version_arg) {
                    Ok(version) => commands::conf_set_key(&paths, &version, key, value, force),
                    Err(e) => Err(e),
                }
            }
            _ => Ok(()),
        },

        Some(("use", sub)) => {
            let version_arg = sub.get_one::<String>("version");
            let shell = sub.get_one::<Shell>("shell").copied();

            match resolve_version(&paths, version_arg) {
                Ok(version) => commands::use_version(&paths, &version, shell),
                Err(e) => Err(e),
            }
        }

        Some(("default", sub)) => {
            let version_arg = sub.get_one::<String>("version");

            match resolve_version(&paths, version_arg) {
                Ok(version) => commands::default(&paths, &version),
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

            match resolve_version(&paths, version_arg) {
                Ok(version) => commands::cli(&paths, &version, tool, &args),
                Err(e) => Err(e),
            }
        }

        Some(("fg", sub)) => match sub.subcommand() {
            Some(("node", fg_sub)) => {
                let version_arg = fg_sub.get_one::<String>("version");

                match resolve_version(&paths, version_arg) {
                    Ok(version) => commands::fg_node(&paths, &version),
                    Err(e) => Err(e),
                }
            }
            _ => Ok(()),
        },

        Some(("inspect", sub)) => {
            let file = sub.get_one::<String>("file").unwrap();
            let version_arg = sub.get_one::<String>("version");

            match resolve_version(&paths, version_arg) {
                Ok(version) => commands::inspect(&paths, &version, file),
                Err(e) => Err(e),
            }
        }

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
