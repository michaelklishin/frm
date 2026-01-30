// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;

use tempfile::TempDir;
use tool_versions::ToolVersions;

use frm::commands;

#[test]
fn set_in_tool_versions_creates_file() {
    let temp = TempDir::new().unwrap();

    commands::erlang_set_in_tool_versions("4.2.3", "26.2.1", Some(temp.path())).unwrap();

    let file_path = temp.path().join(".tool-versions");
    assert!(file_path.exists());

    let tv = ToolVersions::load(&file_path).unwrap();
    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
    assert_eq!(tv.get_version("rabbitmq"), Some("4.2.3"));
}

#[test]
fn set_in_tool_versions_updates_existing() {
    let temp = TempDir::new().unwrap();
    let file_path = temp.path().join(".tool-versions");

    fs::write(&file_path, "nodejs 20.10.0\nerlang 25.0.0\n").unwrap();

    commands::erlang_set_in_tool_versions("4.2.3", "26.2.1", Some(temp.path())).unwrap();

    let tv = ToolVersions::load(&file_path).unwrap();
    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
    assert_eq!(tv.get_version("rabbitmq"), Some("4.2.3"));
    assert_eq!(tv.get_version("nodejs"), Some("20.10.0"));
}

#[test]
fn set_in_tool_versions_preserves_other_tools() {
    let temp = TempDir::new().unwrap();
    let file_path = temp.path().join(".tool-versions");

    fs::write(
        &file_path,
        "# Development tools\nnodejs 20.10.0\nrust 1.75.0\n",
    )
    .unwrap();

    commands::erlang_set_in_tool_versions("4.2.3", "26.2.1", Some(temp.path())).unwrap();

    let tv = ToolVersions::load(&file_path).unwrap();
    assert_eq!(tv.get_version("nodejs"), Some("20.10.0"));
    assert_eq!(tv.get_version("rust"), Some("1.75.0"));
    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
    assert_eq!(tv.get_version("rabbitmq"), Some("4.2.3"));
}

#[test]
fn set_in_tool_versions_prerelease_rabbitmq() {
    let temp = TempDir::new().unwrap();

    commands::erlang_set_in_tool_versions("4.3.0-alpha.1", "27.0.0", Some(temp.path())).unwrap();

    let file_path = temp.path().join(".tool-versions");
    let tv = ToolVersions::load(&file_path).unwrap();
    assert_eq!(tv.get_version("erlang"), Some("27.0.0"));
    assert_eq!(tv.get_version("rabbitmq"), Some("4.3.0-alpha.1"));
}

#[test]
fn set_in_tool_versions_preserves_comments() {
    let temp = TempDir::new().unwrap();
    let file_path = temp.path().join(".tool-versions");

    fs::write(&file_path, "# Project tools\nnodejs 20.10.0\n").unwrap();

    commands::erlang_set_in_tool_versions("4.2.3", "26.2.1", Some(temp.path())).unwrap();

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("# Project tools"));
}

#[test]
fn set_in_tool_versions_updates_both_erlang_and_rabbitmq() {
    let temp = TempDir::new().unwrap();
    let file_path = temp.path().join(".tool-versions");

    fs::write(&file_path, "erlang 25.0.0\nrabbitmq 4.0.0\n").unwrap();

    commands::erlang_set_in_tool_versions("4.2.3", "26.2.1", Some(temp.path())).unwrap();

    let tv = ToolVersions::load(&file_path).unwrap();
    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
    assert_eq!(tv.get_version("rabbitmq"), Some("4.2.3"));
}
