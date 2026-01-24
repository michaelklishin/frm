// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

use crate::Result;
use crate::errors::Error;
use crate::paths::Paths;
use crate::version::Version;

const LOG_FILE_PATTERN: &str = "rabbit@";

pub fn path(paths: &Paths, version: &Version) -> Result<()> {
    if !paths.version_installed(version) {
        return Err(Error::VersionNotInstalled(version.clone()));
    }

    let log_path = find_log_file(paths, version)?;
    println!("{}", log_path.display());

    Ok(())
}

pub fn tail(paths: &Paths, version: &Version, lines: usize) -> Result<()> {
    if !paths.version_installed(version) {
        return Err(Error::VersionNotInstalled(version.clone()));
    }

    let log_path = find_log_file(paths, version)?;
    let file = File::open(&log_path)?;
    let reader = BufReader::new(file);

    let all_lines: Vec<String> = reader.lines().collect::<io::Result<Vec<_>>>()?;
    let start = all_lines.len().saturating_sub(lines);

    for line in &all_lines[start..] {
        println!("{}", line);
    }

    Ok(())
}

fn find_log_file(paths: &Paths, version: &Version) -> Result<PathBuf> {
    let log_dir = paths.version_var_log_dir(version);

    if !log_dir.exists() {
        return Err(Error::FileNotFound(format!(
            "log directory: {}",
            log_dir.display()
        )));
    }

    for entry in fs::read_dir(&log_dir)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        if name.starts_with(LOG_FILE_PATTERN) && name.ends_with(".log") {
            return Ok(entry.path());
        }
    }

    Err(Error::FileNotFound(format!(
        "no log file found in {}",
        log_dir.display()
    )))
}
