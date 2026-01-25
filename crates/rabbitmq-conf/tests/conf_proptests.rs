// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proptest::prelude::*;
use rabbitmq_conf::RabbitMQConf;

static REAL_KEYS: &[&str] = &[
    "listeners.tcp.default",
    "listeners.ssl.default",
    "num_acceptors.tcp",
    "num_acceptors.ssl",
    "handshake_timeout",
    "heartbeat",
    "frame_max",
    "channel_max",
    "tcp_listen_options.backlog",
    "tcp_listen_options.nodelay",
    "tcp_listen_options.keepalive",
    "reverse_dns_lookups",
    "log.console.level",
    "log.file.level",
    "log.file",
    "log.dir",
    "cluster_name",
    "cluster_partition_handling",
    "cluster_keepalive_interval",
    "default_vhost",
    "default_user",
    "default_pass",
    "default_permissions.configure",
    "default_permissions.read",
    "default_permissions.write",
    "collect_statistics",
    "collect_statistics_interval",
    "vm_memory_high_watermark.relative",
    "vm_memory_high_watermark.absolute",
    "disk_free_limit.relative",
    "disk_free_limit.absolute",
    "ssl_options.verify",
    "ssl_options.fail_if_no_peer_cert",
    "ssl_options.cacertfile",
    "ssl_options.certfile",
    "ssl_options.keyfile",
    "ssl_options.depth",
    "auth_backends.1",
    "auth_mechanisms.1",
    "management.listener.port",
    "management.listener.ssl",
    "management.path_prefix",
    "mqtt.listeners.tcp.default",
    "mqtt.listeners.ssl.default",
    "mqtt.allow_anonymous",
    "mqtt.vhost",
    "stomp.listeners.tcp.default",
    "stomp.listeners.ssl.default",
    "prometheus.return_per_object_metrics",
    "prometheus.path",
];

static LISTENER_NAMES: &[&str] = &["default", "amqp", "local", "internal", "external"];
static LOG_LEVELS: &[&str] = &["debug", "info", "notice", "warning", "error", "critical"];
static BOOLEAN_VALUES: &[&str] = &["true", "false", "on", "off", "yes", "no"];
static VERIFY_VALUES: &[&str] = &["verify_none", "verify_peer"];
static PARTITION_HANDLING: &[&str] = &["ignore", "pause_minority", "autoheal"];
static STATISTICS_VALUES: &[&str] = &["none", "coarse", "fine"];

fn real_key_strategy() -> impl Strategy<Value = String> {
    prop::sample::select(REAL_KEYS).prop_map(|s| s.to_string())
}

fn listener_key_strategy() -> impl Strategy<Value = String> {
    (
        prop::sample::select(&["tcp", "ssl"][..]),
        prop::sample::select(LISTENER_NAMES),
    )
        .prop_map(|(proto, name)| format!("listeners.{}.{}", proto, name))
}

fn log_key_strategy() -> impl Strategy<Value = String> {
    (
        prop::sample::select(&["console", "file", "syslog"][..]),
        prop::sample::select(&["level", "file", "formatter"][..]),
    )
        .prop_map(|(sink, prop)| format!("log.{}.{}", sink, prop))
}

fn auth_backend_key_strategy() -> impl Strategy<Value = String> {
    (1..=5u8).prop_map(|n| format!("auth_backends.{}", n))
}

fn any_key_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        2 => real_key_strategy(),
        1 => listener_key_strategy(),
        1 => log_key_strategy(),
        1 => auth_backend_key_strategy(),
    ]
}

fn port_value_strategy() -> impl Strategy<Value = String> {
    (1024u16..65535).prop_map(|p| p.to_string())
}

fn integer_value_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        (0i64..1000000).prop_map(|n| n.to_string()),
        (1u32..1024).prop_map(|n| n.to_string()),
    ]
}

fn boolean_value_strategy() -> impl Strategy<Value = String> {
    prop::sample::select(BOOLEAN_VALUES).prop_map(|s| s.to_string())
}

fn log_level_strategy() -> impl Strategy<Value = String> {
    prop::sample::select(LOG_LEVELS).prop_map(|s| s.to_string())
}

fn path_value_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("/var/log/rabbitmq/rabbit.log".to_string()),
        Just("/var/lib/rabbitmq".to_string()),
        Just("/etc/rabbitmq/ssl/cacert.pem".to_string()),
        Just("/etc/rabbitmq/ssl/cert.pem".to_string()),
        Just("/etc/rabbitmq/ssl/key.pem".to_string()),
        Just("./log/rabbit.log".to_string()),
    ]
}

fn ip_port_value_strategy() -> impl Strategy<Value = String> {
    (
        prop::sample::select(&["127.0.0.1", "0.0.0.0", "192.168.1.1", "10.0.0.1", "::"][..]),
        1024u16..65535,
    )
        .prop_map(|(ip, port)| format!("{}:{}", ip, port))
}

fn memory_value_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        (1u32..100).prop_map(|n| format!("0.{}", n)),
        prop::sample::select(&["128MB", "256MB", "512MB", "1GB", "2GB", "4GB"][..])
            .prop_map(|s| s.to_string()),
    ]
}

fn cluster_name_strategy() -> impl Strategy<Value = String> {
    (
        prop::sample::select(&["prod", "staging", "dev", "test"][..]),
        prop::sample::select(&["us", "eu", "ap", ""][..]),
        (1u8..10),
    )
        .prop_map(|(env, region, n)| {
            if region.is_empty() {
                format!("{}-{}", env, n)
            } else {
                format!("{}.{}.{}", env, region, n)
            }
        })
}

fn vhost_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("/".to_string()),
        Just("/prod".to_string()),
        Just("/staging".to_string()),
        "[a-z]{3,10}".prop_map(|s| format!("/{}", s)),
    ]
}

fn username_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("guest".to_string()),
        Just("admin".to_string()),
        Just("rabbitmq".to_string()),
        "[a-z][a-z0-9_]{2,15}".prop_map(|s| s.to_string()),
    ]
}

fn permission_pattern_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just(".*".to_string()),
        Just("^$".to_string()),
        Just("^amq\\.".to_string()),
        "[a-z]{3,10}".prop_map(|s| format!("^{}.*", s)),
    ]
}

fn verify_value_strategy() -> impl Strategy<Value = String> {
    prop::sample::select(VERIFY_VALUES).prop_map(|s| s.to_string())
}

fn partition_handling_strategy() -> impl Strategy<Value = String> {
    prop::sample::select(PARTITION_HANDLING).prop_map(|s| s.to_string())
}

fn statistics_strategy() -> impl Strategy<Value = String> {
    prop::sample::select(STATISTICS_VALUES).prop_map(|s| s.to_string())
}

fn auth_backend_strategy() -> impl Strategy<Value = String> {
    prop::sample::select(
        &[
            "rabbit_auth_backend_internal",
            "rabbit_auth_backend_ldap",
            "rabbit_auth_backend_http",
            "rabbit_auth_backend_cache",
        ][..],
    )
    .prop_map(|s| s.to_string())
}

fn any_value_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        2 => port_value_strategy(),
        2 => integer_value_strategy(),
        2 => boolean_value_strategy(),
        1 => log_level_strategy(),
        1 => path_value_strategy(),
        1 => ip_port_value_strategy(),
        1 => memory_value_strategy(),
        1 => cluster_name_strategy(),
        1 => vhost_strategy(),
        1 => username_strategy(),
        1 => permission_pattern_strategy(),
        1 => verify_value_strategy(),
        1 => partition_handling_strategy(),
        1 => statistics_strategy(),
        1 => auth_backend_strategy(),
    ]
}

fn quoted_value_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        cluster_name_strategy(),
        "[a-zA-Z0-9 #._-]{5,30}".prop_map(|s| s.to_string()),
    ]
}

fn conf_line_strategy() -> impl Strategy<Value = String> {
    (any_key_strategy(), any_value_strategy()).prop_map(|(k, v)| format!("{} = {}\n", k, v))
}

fn comment_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("# RabbitMQ configuration\n".to_string()),
        Just("# Listeners\n".to_string()),
        Just("# Logging\n".to_string()),
        Just("# Cluster settings\n".to_string()),
        Just("# SSL/TLS\n".to_string()),
        Just("#\n".to_string()),
    ]
}

fn multi_line_conf_strategy() -> impl Strategy<Value = String> {
    prop::collection::vec(
        prop_oneof![
            3 => conf_line_strategy(),
            1 => comment_strategy(),
            1 => Just("\n".to_string()),
        ],
        1..20,
    )
    .prop_map(|lines| lines.join(""))
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(500))]

    #[test]
    fn roundtrip_set_get(key in any_key_strategy(), value in any_value_strategy()) {
        let mut conf = RabbitMQConf::new();
        conf.set(&key, &value);
        prop_assert_eq!(conf.get(&key), Some(value.as_str()));
    }

    #[test]
    fn roundtrip_parse_to_string(key in any_key_strategy(), value in any_value_strategy()) {
        let content = format!("{} = {}\n", key, value);
        let conf = RabbitMQConf::parse(&content).unwrap();
        let output = conf.to_string();
        let reparsed = RabbitMQConf::parse(&output).unwrap();
        prop_assert_eq!(reparsed.get(&key), Some(value.as_str()));
    }

    #[test]
    fn set_overwrite(key in any_key_strategy(), value1 in any_value_strategy(), value2 in any_value_strategy()) {
        let mut conf = RabbitMQConf::new();
        conf.set(&key, &value1);
        conf.set(&key, &value2);
        prop_assert_eq!(conf.get(&key), Some(value2.as_str()));
    }

    #[test]
    fn remove_then_get(key in any_key_strategy(), value in any_value_strategy()) {
        let mut conf = RabbitMQConf::new();
        conf.set(&key, &value);
        conf.remove(&key);
        prop_assert_eq!(conf.get(&key), None);
    }

    #[test]
    fn contains_after_set(key in any_key_strategy(), value in any_value_strategy()) {
        let mut conf = RabbitMQConf::new();
        prop_assert!(!conf.contains_key(&key));
        conf.set(&key, &value);
        prop_assert!(conf.contains_key(&key));
    }

    #[test]
    fn multiple_keys_independent(
        key1 in any_key_strategy(),
        value1 in any_value_strategy(),
        value2 in any_value_strategy()
    ) {
        let key2 = format!("{}.extra", key1);

        let mut conf = RabbitMQConf::new();
        conf.set(&key1, &value1);
        conf.set(&key2, &value2);

        prop_assert_eq!(conf.get(&key1), Some(value1.as_str()));
        prop_assert_eq!(conf.get(&key2), Some(value2.as_str()));
    }

    #[test]
    fn parse_multiline_conf(content in multi_line_conf_strategy()) {
        let result = RabbitMQConf::parse(&content);
        prop_assert!(result.is_ok());
    }

    #[test]
    fn multiline_roundtrip(content in multi_line_conf_strategy()) {
        let conf = RabbitMQConf::parse(&content).unwrap();
        let output = conf.to_string();
        let reparsed = RabbitMQConf::parse(&output).unwrap();

        for key in conf.keys() {
            prop_assert_eq!(reparsed.get(key), conf.get(key));
        }
    }

    #[test]
    fn quoted_values_preserved(key in any_key_strategy(), value in quoted_value_strategy()) {
        let content = format!("{} = '{}'\n", key, value);
        let conf = RabbitMQConf::parse(&content).unwrap();
        prop_assert_eq!(conf.get(&key), Some(value.as_str()));
    }

    #[test]
    fn inline_comments_ignored(key in any_key_strategy(), value in any_value_strategy()) {
        let content = format!("{} = {} # some comment\n", key, value);
        let conf = RabbitMQConf::parse(&content).unwrap();
        prop_assert_eq!(conf.get(&key), Some(value.as_str()));
    }

    #[test]
    fn port_values_parse_as_int(name in prop::sample::select(LISTENER_NAMES), port in 1024u16..65535) {
        let key = format!("listeners.tcp.{}", name);
        let content = format!("{} = {}\n", key, port);
        let conf = RabbitMQConf::parse(&content).unwrap();
        prop_assert_eq!(conf.get_int(&key), Some(port as i64));
    }

    #[test]
    fn boolean_values_parse(key in any_key_strategy(), bool_str in boolean_value_strategy()) {
        let content = format!("{} = {}\n", key, bool_str);
        let conf = RabbitMQConf::parse(&content).unwrap();
        let parsed = conf.get_bool(&key);
        prop_assert!(parsed.is_some());
    }

    #[test]
    fn keys_count_matches(keys in prop::collection::hash_set(any_key_strategy(), 1..20)) {
        let mut conf = RabbitMQConf::new();
        for key in &keys {
            conf.set(key, "value");
        }
        prop_assert_eq!(conf.keys().count(), keys.len());
    }

    #[test]
    fn whitespace_variations_parse(
        key in any_key_strategy(),
        value in any_value_strategy(),
        leading_spaces in 0usize..5,
        spaces_before_eq in 0usize..5,
        spaces_after_eq in 0usize..5
    ) {
        let leading = " ".repeat(leading_spaces);
        let before = " ".repeat(spaces_before_eq);
        let after = " ".repeat(spaces_after_eq);
        let content = format!("{}{}{}={}{}\n", leading, key, before, after, value);
        let conf = RabbitMQConf::parse(&content).unwrap();
        prop_assert_eq!(conf.get(&key), Some(value.as_str()));
    }
}
