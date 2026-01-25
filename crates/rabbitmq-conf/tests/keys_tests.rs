// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rabbitmq_conf::keys;

#[test]
fn valid_key_format_simple() {
    assert!(keys::is_valid_key_format("heartbeat"));
    assert!(keys::is_valid_key_format("cluster_name"));
    assert!(keys::is_valid_key_format("frame_max"));
}

#[test]
fn valid_key_format_dotted() {
    assert!(keys::is_valid_key_format("listeners.tcp.default"));
    assert!(keys::is_valid_key_format("log.console.level"));
    assert!(keys::is_valid_key_format("ssl_options.verify"));
}

#[test]
fn valid_key_format_with_numbers() {
    assert!(keys::is_valid_key_format("auth_backends.1"));
    assert!(keys::is_valid_key_format("ssl_options.crl_sources.0"));
}

#[test]
fn valid_key_format_with_hyphens() {
    assert!(keys::is_valid_key_format("some-key"));
    assert!(keys::is_valid_key_format("key.with-hyphen"));
}

#[test]
fn invalid_key_format_empty() {
    assert!(!keys::is_valid_key_format(""));
}

#[test]
fn invalid_key_format_starts_with_dot() {
    assert!(!keys::is_valid_key_format(".listeners"));
}

#[test]
fn invalid_key_format_ends_with_dot() {
    assert!(!keys::is_valid_key_format("listeners."));
}

#[test]
fn invalid_key_format_double_dot() {
    assert!(!keys::is_valid_key_format("listeners..tcp"));
}

#[test]
fn invalid_key_format_starts_with_number() {
    assert!(!keys::is_valid_key_format("1listeners"));
}

#[test]
fn invalid_key_format_special_chars() {
    assert!(!keys::is_valid_key_format("key@value"));
    assert!(!keys::is_valid_key_format("key!value"));
    assert!(!keys::is_valid_key_format("key value"));
}

#[test]
fn known_key_simple() {
    assert!(keys::is_known_key("heartbeat"));
    assert!(keys::is_known_key("cluster_name"));
    assert!(keys::is_known_key("frame_max"));
}

#[test]
fn known_key_dotted() {
    assert!(keys::is_known_key("listeners.tcp.default"));
    assert!(keys::is_known_key("log.console.level"));
    assert!(keys::is_known_key("ssl_options.verify"));
}

#[test]
fn known_key_with_wildcard() {
    assert!(keys::is_known_key("listeners.tcp.default"));
    assert!(keys::is_known_key("listeners.tcp.amqp"));
    assert!(keys::is_known_key("auth_backends.1"));
    assert!(keys::is_known_key("auth_backends.2"));
}

#[test]
fn unknown_key() {
    assert!(!keys::is_known_key("totally_unknown_key"));
    assert!(!keys::is_known_key("some.random.path"));
}

#[test]
fn suggest_similar_keys_listeners() {
    let suggestions = keys::suggest_similar_keys("listeners.tcp.invalid");
    assert!(!suggestions.is_empty());
    assert!(suggestions.iter().any(|s| s.starts_with("listeners.")));
}

#[test]
fn suggest_similar_keys_log() {
    let suggestions = keys::suggest_similar_keys("log.unknown.setting");
    assert!(!suggestions.is_empty());
    assert!(suggestions.iter().any(|s| s.starts_with("log.")));
}

#[test]
fn suggest_similar_keys_unknown() {
    let suggestions = keys::suggest_similar_keys("zzz_unknown");
    assert!(suggestions.is_empty());
}
