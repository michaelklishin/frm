// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use frm::version::Version;

#[test]
fn version_is_server_packages_release_alpha() {
    let v = "4.3.0-alpha.132057c7".parse::<Version>().unwrap();
    assert!(v.is_server_packages_release());
}

#[test]
fn version_is_server_packages_release_alpha_numeric() {
    let v = "4.3.0-alpha.1".parse::<Version>().unwrap();
    assert!(v.is_server_packages_release());
}

#[test]
fn version_is_not_server_packages_release_beta() {
    let v = "4.2.0-beta.1".parse::<Version>().unwrap();
    assert!(!v.is_server_packages_release());
}

#[test]
fn version_is_not_server_packages_release_rc() {
    let v = "4.2.0-rc.1".parse::<Version>().unwrap();
    assert!(!v.is_server_packages_release());
}

#[test]
fn version_is_not_server_packages_release_ga() {
    let v = "4.2.3".parse::<Version>().unwrap();
    assert!(!v.is_server_packages_release());
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
