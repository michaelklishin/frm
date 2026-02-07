// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::process::Command;

use tabled::settings::Style;
use tabled::{Table, Tabled};

use crate::Result;
use crate::commands::logs::find_log_file;
use crate::common::cli_tools::RABBITMQ_SERVER;
use crate::common::env_vars::{RABBITMQ_CONFIG_FILES, RABBITMQ_HOME};
use crate::errors::Error;
use crate::paths::Paths;
use crate::version::Version;

#[derive(Tabled)]
struct StartInfo {
    #[tabled(rename = "Property")]
    property: &'static str,
    #[tabled(rename = "Value")]
    value: String,
}

pub fn run(paths: &Paths, version: &Version) -> Result<()> {
    if !paths.version_installed(version) {
        return Err(Error::VersionNotInstalled(version.clone()));
    }

    let server_path = paths.version_sbin_dir(version).join(RABBITMQ_SERVER);
    if !server_path.exists() {
        return Err(Error::FileNotFound(server_path.display().to_string()));
    }

    let status = Command::new(&server_path)
        .arg("-detached")
        .env(RABBITMQ_HOME, paths.version_dir(version))
        .env(RABBITMQ_CONFIG_FILES, paths.version_confd_dir(version))
        .status()
        .map_err(|e| {
            Error::CommandFailed(format!(
                "failed to execute {}: {}",
                server_path.display(),
                e
            ))
        })?;

    if !status.success() {
        return Err(Error::CommandFailed(format!(
            "rabbitmq-server -detached exited with code {}",
            status.code().unwrap_or(-1)
        )));
    }

    print_start_info(paths, version);

    Ok(())
}

fn print_start_info(paths: &Paths, version: &Version) {
    let log_path = find_log_file(paths, version)
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| {
            paths
                .version_var_log_dir(version)
                .join("rabbit@<hostname>.log")
                .display()
                .to_string()
        });

    let tail_cmd = format!("tail -f -n 200 {}", log_path);
    let rows = vec![
        StartInfo {
            property: "Stop node",
            value: format!("frm bg stop --version {}", version),
        },
        StartInfo {
            property: "Check listeners",
            value: format!(
                "frm cli rabbitmq-diagnostics --version {} -- listeners",
                version
            ),
        },
        StartInfo {
            property: "Log file",
            value: log_path,
        },
        StartInfo {
            property: "Tail logs",
            value: tail_cmd,
        },
    ];

    let table = Table::new(rows).with(Style::rounded()).to_string();
    println!("{}", table);
}
