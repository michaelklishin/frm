// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rabbitmq_conf::RabbitMQConf;
use tempfile::TempDir;

#[test]
fn parse_empty() {
    let conf = RabbitMQConf::parse("").unwrap();
    assert!(conf.keys().next().is_none());
}

#[test]
fn parse_single_setting() {
    let conf = RabbitMQConf::parse("listeners.tcp.default = 5672\n").unwrap();
    assert_eq!(conf.get("listeners.tcp.default"), Some("5672"));
}

#[test]
fn parse_multiple_settings() {
    let content = "listeners.tcp.default = 5672\nlog.console.level = warning\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    assert_eq!(conf.get("listeners.tcp.default"), Some("5672"));
    assert_eq!(conf.get("log.console.level"), Some("warning"));
}

#[test]
fn parse_with_comments() {
    let content = "# This is a comment\nlisteners.tcp.default = 5672\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    assert_eq!(conf.get("listeners.tcp.default"), Some("5672"));
}

#[test]
fn parse_with_inline_comment() {
    let content = "listeners.tcp.default = 5672 # inline comment\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    assert_eq!(conf.get("listeners.tcp.default"), Some("5672"));
}

#[test]
fn parse_quoted_value() {
    let content = "cluster_name = 'my cluster'\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    assert_eq!(conf.get("cluster_name"), Some("my cluster"));
}

#[test]
fn parse_quoted_value_with_hash() {
    let content = "cluster_name = 'my#cluster'\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    assert_eq!(conf.get("cluster_name"), Some("my#cluster"));
}

#[test]
fn parse_quoted_value_with_dots() {
    let content = "cluster_name = 'production.eu.01'\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    assert_eq!(conf.get("cluster_name"), Some("production.eu.01"));
}

#[test]
fn parse_unquoted_value_with_dots() {
    let content = "some.key = value.with.dots\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    assert_eq!(conf.get("some.key"), Some("value.with.dots"));
}

#[test]
fn parse_empty_lines() {
    let content = "\nlisteners.tcp.default = 5672\n\nlog.console.level = warning\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    assert_eq!(conf.get("listeners.tcp.default"), Some("5672"));
    assert_eq!(conf.get("log.console.level"), Some("warning"));
}

#[test]
fn parse_whitespace_around_equals() {
    let content = "listeners.tcp.default   =   5672\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    assert_eq!(conf.get("listeners.tcp.default"), Some("5672"));
}

#[test]
fn parse_no_whitespace_around_equals() {
    let content = "listeners.tcp.default=5672\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    assert_eq!(conf.get("listeners.tcp.default"), Some("5672"));
}

#[test]
fn get_nonexistent_key() {
    let conf = RabbitMQConf::parse("listeners.tcp.default = 5672\n").unwrap();
    assert_eq!(conf.get("nonexistent.key"), None);
}

#[test]
fn set_new_key() {
    let mut conf = RabbitMQConf::new();
    conf.set("listeners.tcp.default", "5672");
    assert_eq!(conf.get("listeners.tcp.default"), Some("5672"));
}

#[test]
fn set_update_existing_key() {
    let mut conf = RabbitMQConf::parse("listeners.tcp.default = 5672\n").unwrap();
    conf.set("listeners.tcp.default", "5673");
    assert_eq!(conf.get("listeners.tcp.default"), Some("5673"));
}

#[test]
fn remove_key() {
    let mut conf = RabbitMQConf::parse("listeners.tcp.default = 5672\n").unwrap();
    assert!(conf.remove("listeners.tcp.default"));
    assert_eq!(conf.get("listeners.tcp.default"), None);
}

#[test]
fn remove_nonexistent_key() {
    let mut conf = RabbitMQConf::parse("listeners.tcp.default = 5672\n").unwrap();
    assert!(!conf.remove("nonexistent.key"));
}

#[test]
fn contains_key() {
    let conf = RabbitMQConf::parse("listeners.tcp.default = 5672\n").unwrap();
    assert!(conf.contains_key("listeners.tcp.default"));
    assert!(!conf.contains_key("nonexistent.key"));
}

#[test]
fn keys_iterator() {
    let content = "listeners.tcp.default = 5672\nlog.console.level = warning\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    let keys: Vec<&str> = conf.keys().collect();
    assert_eq!(keys.len(), 2);
    assert!(keys.contains(&"listeners.tcp.default"));
    assert!(keys.contains(&"log.console.level"));
}

#[test]
fn to_string_preserves_settings() {
    let content = "listeners.tcp.default = 5672\nlog.console.level = warning\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    let output = conf.to_string();
    assert!(output.contains("listeners.tcp.default = 5672"));
    assert!(output.contains("log.console.level = warning"));
}

#[test]
fn to_string_preserves_comments() {
    let content = "# Comment line\nlisteners.tcp.default = 5672\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    let output = conf.to_string();
    assert!(output.contains("# Comment line"));
    assert!(output.contains("listeners.tcp.default = 5672"));
}

#[test]
fn to_string_quotes_values_with_spaces() {
    let mut conf = RabbitMQConf::new();
    conf.set("cluster_name", "my cluster");
    let output = conf.to_string();
    assert!(output.contains("cluster_name = 'my cluster'"));
}

#[test]
fn to_string_quotes_values_with_hash() {
    let mut conf = RabbitMQConf::new();
    conf.set("cluster_name", "my#cluster");
    let output = conf.to_string();
    assert!(output.contains("cluster_name = 'my#cluster'"));
}

#[test]
fn save_and_load() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("rabbitmq.conf");

    let mut conf = RabbitMQConf::new();
    conf.set("listeners.tcp.default", "5672");
    conf.set("log.console.level", "warning");
    conf.save(&path).unwrap();

    let loaded = RabbitMQConf::load(&path).unwrap();
    assert_eq!(loaded.get("listeners.tcp.default"), Some("5672"));
    assert_eq!(loaded.get("log.console.level"), Some("warning"));
}

#[test]
fn load_nonexistent_file() {
    let result = RabbitMQConf::load("/nonexistent/path/rabbitmq.conf");
    assert!(result.is_err());
}

#[test]
fn parse_complex_file() {
    let content = r#"# RabbitMQ configuration
listeners.tcp.default = 5672
listeners.ssl.default = 5671

# Logging
log.console.level = warning
log.file = /var/log/rabbitmq/rabbit.log

# Cluster
cluster_name = 'prod.ca.01'
"#;
    let conf = RabbitMQConf::parse(content).unwrap();
    assert_eq!(conf.get("listeners.tcp.default"), Some("5672"));
    assert_eq!(conf.get("listeners.ssl.default"), Some("5671"));
    assert_eq!(conf.get("log.console.level"), Some("warning"));
    assert_eq!(conf.get("log.file"), Some("/var/log/rabbitmq/rabbit.log"));
    assert_eq!(conf.get("cluster_name"), Some("prod.ca.01"));
}

#[test]
fn parse_value_with_equals_sign() {
    let content = "some.key = value=with=equals\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    assert_eq!(conf.get("some.key"), Some("value=with=equals"));
}

#[test]
fn roundtrip_preserves_order() {
    let content = "key1 = value1\nkey2 = value2\nkey3 = value3\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    let output = conf.to_string();
    let lines: Vec<&str> = output.lines().collect();
    assert!(lines[0].starts_with("key1"));
    assert!(lines[1].starts_with("key2"));
    assert!(lines[2].starts_with("key3"));
}

#[test]
fn get_int() {
    let content = "listeners.tcp.default = 5672\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    assert_eq!(conf.get_int("listeners.tcp.default"), Some(5672));
}

#[test]
fn get_int_negative() {
    let content = "some.key = -42\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    assert_eq!(conf.get_int("some.key"), Some(-42));
}

#[test]
fn get_int_invalid() {
    let content = "some.key = not_a_number\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    assert_eq!(conf.get_int("some.key"), None);
}

#[test]
fn get_bool_true() {
    let conf = RabbitMQConf::parse("key = true\n").unwrap();
    assert_eq!(conf.get_bool("key"), Some(true));

    let conf = RabbitMQConf::parse("key = on\n").unwrap();
    assert_eq!(conf.get_bool("key"), Some(true));

    let conf = RabbitMQConf::parse("key = yes\n").unwrap();
    assert_eq!(conf.get_bool("key"), Some(true));

    let conf = RabbitMQConf::parse("key = 1\n").unwrap();
    assert_eq!(conf.get_bool("key"), Some(true));
}

#[test]
fn get_bool_false() {
    let conf = RabbitMQConf::parse("key = false\n").unwrap();
    assert_eq!(conf.get_bool("key"), Some(false));

    let conf = RabbitMQConf::parse("key = off\n").unwrap();
    assert_eq!(conf.get_bool("key"), Some(false));

    let conf = RabbitMQConf::parse("key = no\n").unwrap();
    assert_eq!(conf.get_bool("key"), Some(false));

    let conf = RabbitMQConf::parse("key = 0\n").unwrap();
    assert_eq!(conf.get_bool("key"), Some(false));
}

#[test]
fn get_bool_invalid() {
    let conf = RabbitMQConf::parse("key = maybe\n").unwrap();
    assert_eq!(conf.get_bool("key"), None);
}

#[test]
fn get_float() {
    let conf = RabbitMQConf::parse("key = 3.14\n").unwrap();
    assert_eq!(conf.get_float("key"), Some(3.14));
}

#[test]
fn get_float_integer() {
    let conf = RabbitMQConf::parse("key = 42\n").unwrap();
    assert_eq!(conf.get_float("key"), Some(42.0));
}

#[test]
fn get_float_invalid() {
    let conf = RabbitMQConf::parse("key = not_a_float\n").unwrap();
    assert_eq!(conf.get_float("key"), None);
}

#[test]
fn get_matching_single_wildcard() {
    let content =
        "listeners.tcp.default = 5672\nlisteners.tcp.amqp = 5673\nlisteners.ssl.default = 5671\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    let matches = conf.get_matching("listeners.tcp.*");
    assert_eq!(matches.len(), 2);
    assert!(
        matches
            .iter()
            .any(|(k, v)| *k == "listeners.tcp.default" && *v == "5672")
    );
    assert!(
        matches
            .iter()
            .any(|(k, v)| *k == "listeners.tcp.amqp" && *v == "5673")
    );
}

#[test]
fn get_matching_no_matches() {
    let content = "listeners.tcp.default = 5672\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    let matches = conf.get_matching("listeners.ssl.*");
    assert!(matches.is_empty());
}

#[test]
fn get_matching_multiple_wildcards() {
    let content = "log.console.level = warning\nlog.file.level = info\nlog.syslog.level = error\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    let matches = conf.get_matching("log.*.level");
    assert_eq!(matches.len(), 3);
}

#[test]
fn get_matching_exact_match() {
    let content = "heartbeat = 60\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    let matches = conf.get_matching("heartbeat");
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0], ("heartbeat", "60"));
}

#[test]
fn is_pattern_with_wildcard() {
    assert!(RabbitMQConf::is_pattern("listeners.tcp.*"));
    assert!(RabbitMQConf::is_pattern("*.level"));
    assert!(RabbitMQConf::is_pattern("log.*.level"));
}

#[test]
fn is_pattern_without_wildcard() {
    assert!(!RabbitMQConf::is_pattern("listeners.tcp.default"));
    assert!(!RabbitMQConf::is_pattern("heartbeat"));
}

#[test]
fn parse_empty_quoted_value() {
    let conf = RabbitMQConf::parse("key = ''\n").unwrap();
    assert_eq!(conf.get("key"), Some(""));
}

#[test]
fn parse_quoted_value_with_inline_comment() {
    let content = "key = 'value # not comment' # real comment\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    assert_eq!(conf.get("key"), Some("value # not comment"));
}

#[test]
fn parse_leading_whitespace() {
    let content = "  listeners.tcp.default = 5672\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    assert_eq!(conf.get("listeners.tcp.default"), Some("5672"));
}

#[test]
fn parse_tab_whitespace() {
    let content = "key\t=\tvalue\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    assert_eq!(conf.get("key"), Some("value"));
}

#[test]
fn parse_unclosed_quote_treated_as_literal() {
    let content = "key = 'unclosed\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    assert_eq!(conf.get("key"), Some("'unclosed"));
}

#[test]
fn parse_encrypted_value_preserved() {
    let content = "default_pass = encrypted:abc123def456\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    assert_eq!(conf.get("default_pass"), Some("encrypted:abc123def456"));
}

#[test]
fn parse_env_variable_syntax_preserved() {
    let content = "cluster_name = deployment-$(DEPLOYMENT_ID)\n";
    let conf = RabbitMQConf::parse(content).unwrap();
    assert_eq!(
        conf.get("cluster_name"),
        Some("deployment-$(DEPLOYMENT_ID)")
    );
}
