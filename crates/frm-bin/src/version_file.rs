// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;
use std::fs;
use std::path::Path;

use crate::version::Version;

const TOOL_NAME: &str = "rabbitmq";

pub fn find_version() -> Option<Version> {
    let cwd = env::current_dir().ok()?;
    find_version_in(&cwd)
}

pub fn find_version_in(start_dir: &Path) -> Option<Version> {
    if let Some(version) = read_tool_versions(start_dir) {
        return Some(version);
    }

    start_dir.parent().and_then(find_version_in)
}

pub fn parse_tool_versions(content: &str) -> Option<Version> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut parts = line.split_whitespace();
        if let (Some(tool), Some(version_str)) = (parts.next(), parts.next())
            && tool == TOOL_NAME
        {
            return version_str.parse().ok();
        }
    }

    None
}

fn read_tool_versions(dir: &Path) -> Option<Version> {
    let file_path = dir.join(".tool-versions");
    let content = fs::read_to_string(file_path).ok()?;
    parse_tool_versions(&content)
}
