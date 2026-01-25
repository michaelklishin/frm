// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use frm::releases::parse_version_from_release_name;
use frm::version::Version;

#[test]
fn parse_version_from_release_name_alpha() {
    let v = parse_version_from_release_name("RabbitMQ 4.3.0-alpha.132057c7 (2025-01-01)");
    assert!(v.is_some());
    let v = v.unwrap();
    assert_eq!(v.to_string(), "4.3.0-alpha.132057c7");
}

#[test]
fn parse_version_from_release_name_ga() {
    let v = parse_version_from_release_name("RabbitMQ 4.2.3");
    assert!(v.is_some());
    let v = v.unwrap();
    assert_eq!(v.to_string(), "4.2.3");
}

#[test]
fn parse_version_from_release_name_invalid() {
    assert!(parse_version_from_release_name("Invalid release").is_none());
    assert!(parse_version_from_release_name("").is_none());
    assert!(parse_version_from_release_name("RabbitMQ").is_none());
}

#[test]
fn version_is_distributed_via_server_packages_repository_alpha() {
    let v = "4.3.0-alpha.132057c7".parse::<Version>().unwrap();
    assert!(v.is_distributed_via_server_packages_repository());
}

#[test]
fn version_is_distributed_via_server_packages_repository_alpha_numeric() {
    let v = "4.3.0-alpha.1".parse::<Version>().unwrap();
    assert!(v.is_distributed_via_server_packages_repository());
}

#[test]
fn version_is_not_server_packages_release_beta() {
    let v = "4.2.0-beta.1".parse::<Version>().unwrap();
    assert!(!v.is_distributed_via_server_packages_repository());
}

#[test]
fn version_is_not_server_packages_release_rc() {
    let v = "4.2.0-rc.1".parse::<Version>().unwrap();
    assert!(!v.is_distributed_via_server_packages_repository());
}

#[test]
fn version_is_not_server_packages_release_ga() {
    let v = "4.2.3".parse::<Version>().unwrap();
    assert!(!v.is_distributed_via_server_packages_repository());
}

#[test]
fn version_download_url_with_tag() {
    let v = "4.3.0-alpha.132057c7".parse::<Version>().unwrap();
    let url = v.download_url_with_tag("alphas.1769107242092");

    assert!(url.contains("github.com/rabbitmq/server-packages/releases"));
    assert!(url.contains("alphas.1769107242092"));
    assert!(url.contains("4.3.0-alpha.132057c7"));
    assert!(url.ends_with(".tar.xz"));
}

#[test]
fn version_download_url_regular() {
    let v = "4.2.3".parse::<Version>().unwrap();
    let url = v.download_url();

    assert!(url.contains("github.com/rabbitmq/rabbitmq-server/releases"));
    assert!(url.contains("v4.2.3"));
    assert!(url.ends_with(".tar.xz"));
}

#[test]
fn version_download_url_beta() {
    let v = "4.2.0-beta.1".parse::<Version>().unwrap();
    let url = v.download_url();

    assert!(url.contains("github.com/rabbitmq/rabbitmq-server/releases"));
    assert!(url.contains("v4.2.0-beta.1"));
}

#[test]
fn version_download_url_rc() {
    let v = "4.2.0-rc.1".parse::<Version>().unwrap();
    let url = v.download_url();

    assert!(url.contains("github.com/rabbitmq/rabbitmq-server/releases"));
    assert!(url.contains("v4.2.0-rc.1"));
}
