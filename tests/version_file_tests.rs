// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;

use tempfile::TempDir;

use frm::version::Version;
use frm::version_file::{find_version_in, parse_tool_versions};

#[test]
fn parse_simple_version() {
    let content = "rabbitmq 4.2.3";
    let version = parse_tool_versions(content).unwrap();
    assert_eq!(version, Version::new(4, 2, 3));
}

#[test]
fn parse_with_other_tools() {
    let content = "nodejs 20.10.0\nrabbitmq 4.1.0\nrust 1.75.0";
    let version = parse_tool_versions(content).unwrap();
    assert_eq!(version, Version::new(4, 1, 0));
}

#[test]
fn parse_with_comments() {
    let content = "# This is a comment\nrabbitmq 3.13.0\n# Another comment";
    let version = parse_tool_versions(content).unwrap();
    assert_eq!(version, Version::new(3, 13, 0));
}

#[test]
fn parse_with_empty_lines() {
    let content = "\n\nrabbitmq 4.0.0\n\n";
    let version = parse_tool_versions(content).unwrap();
    assert_eq!(version, Version::new(4, 0, 0));
}

#[test]
fn parse_with_leading_whitespace() {
    let content = "  rabbitmq 4.2.1  ";
    let version = parse_tool_versions(content).unwrap();
    assert_eq!(version, Version::new(4, 2, 1));
}

#[test]
fn parse_prerelease_alpha() {
    let content = "rabbitmq 4.2.4-alpha.1";
    let version = parse_tool_versions(content).unwrap();
    assert_eq!(version.major, 4);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 4);
    assert!(version.prerelease.is_some());
}

#[test]
fn parse_prerelease_beta() {
    let content = "rabbitmq 4.2.4-beta.2";
    let version = parse_tool_versions(content).unwrap();
    assert!(version.prerelease.is_some());
}

#[test]
fn parse_prerelease_rc() {
    let content = "rabbitmq 4.2.4-rc.1";
    let version = parse_tool_versions(content).unwrap();
    assert!(version.prerelease.is_some());
}

#[test]
fn parse_no_rabbitmq() {
    let content = "nodejs 20.10.0\nrust 1.75.0";
    assert!(parse_tool_versions(content).is_none());
}

#[test]
fn parse_empty_content() {
    let content = "";
    assert!(parse_tool_versions(content).is_none());
}

#[test]
fn parse_only_comments() {
    let content = "# comment 1\n# comment 2";
    assert!(parse_tool_versions(content).is_none());
}

#[test]
fn parse_invalid_version() {
    let content = "rabbitmq invalid";
    assert!(parse_tool_versions(content).is_none());
}

#[test]
fn parse_first_rabbitmq_wins() {
    let content = "rabbitmq 4.0.0\nrabbitmq 4.1.0";
    let version = parse_tool_versions(content).unwrap();
    assert_eq!(version, Version::new(4, 0, 0));
}

#[test]
fn find_version_in_current_dir() {
    let temp = TempDir::new().unwrap();
    let tool_versions = temp.path().join(".tool-versions");
    fs::write(&tool_versions, "rabbitmq 4.2.3").unwrap();

    let version = find_version_in(temp.path()).unwrap();
    assert_eq!(version, Version::new(4, 2, 3));
}

#[test]
fn find_version_in_parent_dir() {
    let temp = TempDir::new().unwrap();
    let tool_versions = temp.path().join(".tool-versions");
    fs::write(&tool_versions, "rabbitmq 4.1.0").unwrap();

    let subdir = temp.path().join("subdir");
    fs::create_dir(&subdir).unwrap();

    let version = find_version_in(&subdir).unwrap();
    assert_eq!(version, Version::new(4, 1, 0));
}

#[test]
fn find_version_in_nested_dir() {
    let temp = TempDir::new().unwrap();
    let tool_versions = temp.path().join(".tool-versions");
    fs::write(&tool_versions, "rabbitmq 3.13.0").unwrap();

    let subdir = temp.path().join("a").join("b").join("c");
    fs::create_dir_all(&subdir).unwrap();

    let version = find_version_in(&subdir).unwrap();
    assert_eq!(version, Version::new(3, 13, 0));
}

#[test]
fn find_version_child_overrides_parent() {
    let temp = TempDir::new().unwrap();
    let parent_versions = temp.path().join(".tool-versions");
    fs::write(&parent_versions, "rabbitmq 4.0.0").unwrap();

    let subdir = temp.path().join("child");
    fs::create_dir(&subdir).unwrap();
    let child_versions = subdir.join(".tool-versions");
    fs::write(&child_versions, "rabbitmq 4.2.0").unwrap();

    let version = find_version_in(&subdir).unwrap();
    assert_eq!(version, Version::new(4, 2, 0));
}

#[test]
fn find_version_no_file() {
    let temp = TempDir::new().unwrap();
    let subdir = temp.path().join("empty");
    fs::create_dir(&subdir).unwrap();

    assert!(find_version_in(&subdir).is_none());
}

#[test]
fn parse_with_tabs() {
    let content = "rabbitmq\t4.2.3";
    let version = parse_tool_versions(content).unwrap();
    assert_eq!(version, Version::new(4, 2, 3));
}

#[test]
fn parse_with_multiple_spaces() {
    let content = "rabbitmq    4.2.3";
    let version = parse_tool_versions(content).unwrap();
    assert_eq!(version, Version::new(4, 2, 3));
}

#[test]
fn parse_with_extra_fields() {
    let content = "rabbitmq 4.2.3 extra garbage";
    let version = parse_tool_versions(content).unwrap();
    assert_eq!(version, Version::new(4, 2, 3));
}

#[test]
fn parse_rabbitmq_case_sensitive() {
    let content = "RabbitMQ 4.2.3";
    assert!(parse_tool_versions(content).is_none());
}
