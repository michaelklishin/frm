// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This module implements key validation for RabbitMQ configuration files.
//!
//! The validation module uses known keys and patterns
//! from the RabbitMQ Cuttlefish schema files (`priv/*.schema`).

/// Key patterns from the RabbitMQ Cuttlefish schema files.
/// Patterns with `$name`, `$id`, etc. are represented with a `*` wildcard.
static KNOWN_KEY_PATTERNS: &[&str] = &[
    // Listeners
    "listeners.tcp",
    "listeners.tcp.*",
    "listeners.ssl",
    "listeners.ssl.*",
    "num_acceptors.ssl",
    "num_acceptors.tcp",
    // Networking
    "socket_writer.gc_threshold",
    "handshake_timeout",
    "reverse_dns_lookups",
    "tcp_listen_options",
    "tcp_listen_options.backlog",
    "tcp_listen_options.nodelay",
    "tcp_listen_options.buffer",
    "tcp_listen_options.delay_send",
    "tcp_listen_options.dontroute",
    "tcp_listen_options.exit_on_close",
    "tcp_listen_options.fd",
    "tcp_listen_options.high_msgq_watermark",
    "tcp_listen_options.high_watermark",
    "tcp_listen_options.keepalive",
    "tcp_listen_options.low_msgq_watermark",
    "tcp_listen_options.low_watermark",
    "tcp_listen_options.port",
    "tcp_listen_options.priority",
    "tcp_listen_options.recbuf",
    "tcp_listen_options.send_timeout",
    "tcp_listen_options.send_timeout_close",
    "tcp_listen_options.sndbuf",
    "tcp_listen_options.tos",
    "tcp_listen_options.linger.on",
    "tcp_listen_options.linger.timeout",
    // Erlang
    "erlang.K",
    // Definitions
    "load_definitions",
    "definitions.local.path",
    "definitions.import_backend",
    "definitions.skip_if_unchanged",
    "definitions.hashing.algorithm",
    "definitions.https.url",
    "definitions.tls.verify",
    "definitions.tls.fail_if_no_peer_cert",
    "definitions.tls.cacertfile",
    "definitions.tls.certfile",
    "definitions.tls.cert",
    "definitions.tls.reuse_session",
    "definitions.tls.crl_check",
    "definitions.tls.depth",
    "definitions.tls.dh",
    "definitions.tls.keyfile",
    "definitions.tls.log_alert",
    "definitions.tls.password",
    "definitions.tls.secure_renegotiate",
    "definitions.tls.reuse_sessions",
    "definitions.tls.versions.*",
    "definitions.tls.ciphers.*",
    "definitions.tls.log_level",
    // Loopback users
    "loopback_users",
    "loopback_users.*",
    // SSL
    "ssl_allow_poodle_attack",
    "ssl_options",
    "ssl_options.verify",
    "ssl_options.fail_if_no_peer_cert",
    "ssl_options.cacertfile",
    "ssl_options.certfile",
    "ssl_options.cert",
    "ssl_options.client_renegotiation",
    "ssl_options.crl_check",
    "ssl_options.crl_sources.*",
    "ssl_options.crl_sources.*.timeout",
    "ssl_options.crl_sources.*.path",
    "ssl_options.depth",
    "ssl_options.dh",
    "ssl_options.dhfile",
    "ssl_options.honor_cipher_order",
    "ssl_options.honor_ecc_order",
    "ssl_options.key.RSAPrivateKey",
    "ssl_options.key.DSAPrivateKey",
    "ssl_options.key.PrivateKeyInfo",
    "ssl_options.keyfile",
    "ssl_options.log_level",
    "ssl_options.log_alert",
    "ssl_options.password",
    "ssl_options.psk_identity",
    "ssl_options.reuse_sessions",
    "ssl_options.secure_renegotiate",
    "ssl_options.versions.*",
    "ssl_options.ciphers.*",
    "ssl_options.bypass_pem_cache",
    // Metadata store
    "metadata_store.khepri.default_timeout",
    // Auth
    "auth_mechanisms.*",
    "auth_backends.*",
    "auth_backends.*.authn",
    "auth_backends.*.authz",
    "ssl_cert_login_from",
    "ssl_cert_login_san_type",
    "ssl_cert_login_san_index",
    "ssl_handshake_timeout",
    // Cluster
    "cluster_name",
    "cluster_partition_handling",
    "cluster_partition_handling.pause_if_all_down.recover",
    "cluster_partition_handling.pause_if_all_down.nodes.*",
    "cluster_formation.peer_discovery_backend",
    "cluster_formation.node_type",
    "cluster_formation.registration.enabled",
    "cluster_formation.internal_lock_retries",
    "cluster_formation.lock_retry_limit",
    "cluster_formation.lock_retry_timeout",
    "cluster_formation.discovery_retry_limit",
    "cluster_formation.discovery_retry_interval",
    "cluster_formation.target_cluster_size_hint",
    "cluster_formation.classic_config.nodes.*",
    "cluster_formation.dns.hostname",
    "cluster_queue_limit",
    "cluster_keepalive_interval",
    "cluster_exchange_limit",
    // Workers
    "default_worker_pool_size",
    // Password
    "password_hashing_module",
    "credential_validator.validation_backend",
    "credential_validator.min_length",
    "credential_validator.regexp",
    // Defaults
    "default_vhost",
    "default_user",
    "default_pass",
    "default_permissions.configure",
    "default_permissions.read",
    "default_permissions.write",
    "default_users.*.vhost_pattern",
    "default_users.*.password",
    "default_users.*.configure",
    "default_users.*.read",
    "default_users.*.write",
    "default_users.*.tags",
    "anonymous_login_user",
    "anonymous_login_pass",
    "default_user_tags.*",
    // Policies
    "default_policies.operator.*.vhost_pattern",
    "default_policies.operator.*.queue_pattern",
    "default_policies.operator.*.apply_to",
    "default_policies.operator.*.expires",
    "default_policies.operator.*.message_ttl",
    "default_policies.operator.*.max_length",
    "default_policies.operator.*.max_length_bytes",
    "default_policies.operator.*.max_in_memory_bytes",
    "default_policies.operator.*.max_in_memory_length",
    "default_policies.operator.*.delivery_limit",
    "default_policies.operator.*.classic_queues.ha_mode",
    "default_policies.operator.*.classic_queues.ha_params",
    "default_policies.operator.*.classic_queues.ha_sync_mode",
    "default_policies.operator.*.classic_queues.queue_version",
    // Limits
    "default_limits.vhosts.*.pattern",
    "default_limits.vhosts.*.max_connections",
    "default_limits.vhosts.*.max_queues",
    // Protocol
    "heartbeat",
    "frame_max",
    "initial_frame_max",
    "channel_max",
    "channel_max_per_node",
    "consumer_max_per_channel",
    "session_max_per_connection",
    "link_max_per_session",
    "connection_max",
    "ranch_connection_max",
    "vhost_max",
    "max_message_size",
    // Memory
    "vm_memory_high_watermark.relative",
    "vm_memory_high_watermark.absolute",
    "vm_memory_high_watermark_paging_ratio",
    "memory_monitor_interval",
    "vm_memory_calculation_strategy",
    "total_memory_available_override_value",
    // Disk
    "disk_free_limit.relative",
    "disk_free_limit.absolute",
    // Delegates
    "delegate_count",
    // Mirroring
    "mirroring_sync_batch_size",
    "mirroring_sync_max_throughput",
    // Queue
    "queue_master_locator",
    "queue_leader_locator",
    "queue_index_embed_msgs_below",
    "default_queue_type",
    // Classic queue
    "classic_queue.default_version",
    "queue_types.classic.enabled",
    "queue_types.stream.enabled",
    "queue_types.quorum.enabled",
    // Statistics
    "collect_statistics",
    "collect_statistics_interval",
    // Misc
    "hipe_compile",
    "mnesia_table_loading_retry_timeout",
    "mnesia_table_loading_retry_limit",
    "message_store_shutdown_timeout",
    "background_gc_enabled",
    "background_gc_target_interval",
    "proxy_protocol",
    "vhost_restart_strategy",
    "consumer_timeout",
    "product.name",
    "product.version",
    "motd_file",
    "prevent_startup_if_node_was_reset",
    // Logging
    "log.summarize_process_state",
    "log.error_logger_format_depth",
    "log.dir",
    "log.console",
    "log.console.level",
    "log.console.stdio",
    "log.console.use_colors",
    "log.console.color_esc_seqs.debug",
    "log.console.color_esc_seqs.info",
    "log.console.color_esc_seqs.notice",
    "log.console.color_esc_seqs.warning",
    "log.console.color_esc_seqs.error",
    "log.console.color_esc_seqs.critical",
    "log.console.color_esc_seqs.alert",
    "log.console.color_esc_seqs.emergency",
    "log.console.formatter",
    "log.console.formatter.time_format",
    "log.console.formatter.level_format",
    "log.console.formatter.single_line",
    "log.console.formatter.plaintext.format",
    "log.console.formatter.json.field_map",
    "log.console.formatter.json.verbosity_map",
    "log.exchange",
    "log.exchange.level",
    "log.exchange.formatter",
    "log.exchange.formatter.time_format",
    "log.exchange.formatter.level_format",
    "log.exchange.formatter.single_line",
    "log.exchange.formatter.plaintext.format",
    "log.exchange.formatter.json.field_map",
    "log.exchange.formatter.json.verbosity_map",
    "log.journald",
    "log.journald.level",
    "log.journald.fields",
    "log.syslog",
    "log.syslog.level",
    "log.syslog.formatter",
    "log.syslog.formatter.time_format",
    "log.syslog.formatter.level_format",
    "log.syslog.formatter.single_line",
    "log.syslog.formatter.plaintext.format",
    "log.syslog.formatter.json.field_map",
    "log.syslog.formatter.json.verbosity_map",
    "log.syslog.identity",
    "log.syslog.facility",
    "log.syslog.multiline_mode",
    "log.syslog.ip",
    "log.syslog.host",
    "log.syslog.port",
    "log.syslog.transport",
    "log.syslog.protocol",
    "log.syslog.ssl_options.verify",
    "log.syslog.ssl_options.fail_if_no_peer_cert",
    "log.syslog.ssl_options.cacertfile",
    "log.syslog.ssl_options.certfile",
    "log.syslog.ssl_options.cert",
    "log.syslog.ssl_options.client_renegotiation",
    "log.syslog.ssl_options.crl_check",
    "log.syslog.ssl_options.depth",
    "log.syslog.ssl_options.dh",
    "log.syslog.ssl_options.dhfile",
    "log.syslog.ssl_options.honor_cipher_order",
    "log.syslog.ssl_options.honor_ecc_order",
    "log.syslog.ssl_options.key.RSAPrivateKey",
    "log.syslog.ssl_options.key.DSAPrivateKey",
    "log.syslog.ssl_options.key.PrivateKeyInfo",
    "log.syslog.ssl_options.keyfile",
    "log.syslog.ssl_options.log_alert",
    "log.syslog.ssl_options.password",
    "log.syslog.ssl_options.psk_identity",
    "log.syslog.ssl_options.reuse_sessions",
    "log.syslog.ssl_options.secure_renegotiate",
    "log.syslog.ssl_options.versions.*",
    "log.file",
    "log.file.level",
    "log.file.rotation.date",
    "log.file.rotation.compress",
    "log.file.rotation.size",
    "log.file.rotation.count",
    "log.file.formatter",
    "log.file.formatter.time_format",
    "log.file.formatter.level_format",
    "log.file.formatter.single_line",
    "log.file.formatter.plaintext.format",
    "log.file.formatter.json.field_map",
    "log.file.formatter.json.verbosity_map",
    "log.connection.level",
    "log.connection.file",
    "log.connection.rotation.date",
    "log.connection.rotation.compress",
    "log.connection.rotation.size",
    "log.connection.rotation.count",
    "log.channel.level",
    "log.channel.file",
    "log.channel.rotation.date",
    "log.channel.rotation.compress",
    "log.channel.rotation.size",
    "log.channel.rotation.count",
    "log.mirroring.level",
    "log.mirroring.file",
    "log.mirroring.rotation.date",
    "log.mirroring.rotation.compress",
    "log.mirroring.rotation.size",
    "log.mirroring.rotation.count",
    "log.queue.level",
    "log.queue.file",
    "log.queue.rotation.date",
    "log.queue.rotation.compress",
    "log.queue.rotation.size",
    "log.queue.rotation.count",
    "log.federation.level",
    "log.federation.file",
    "log.federation.rotation.date",
    "log.federation.rotation.compress",
    "log.federation.rotation.size",
    "log.federation.rotation.count",
    "log.upgrade.level",
    "log.upgrade.file",
    "log.upgrade.rotation.date",
    "log.upgrade.rotation.compress",
    "log.upgrade.rotation.size",
    "log.upgrade.rotation.count",
    "log.ra.level",
    "log.ra.file",
    "log.ra.rotation.date",
    "log.ra.rotation.compress",
    "log.ra.rotation.size",
    "log.ra.rotation.count",
    "log.default.level",
    "log.default.rotation.date",
    "log.default.rotation.compress",
    "log.default.rotation.size",
    "log.default.rotation.count",
    // Network/distribution
    "net_ticktime",
    "distribution.listener.port_range.min",
    "distribution.listener.port_range.max",
    "distribution.listener.interface",
    // Sysmon
    "sysmon_handler.thresholds.busy_processes",
    "sysmon_handler.thresholds.busy_ports",
    "sysmon_handler.triggers.process.garbage_collection",
    "sysmon_handler.triggers.process.long_scheduled_execution",
    "sysmon_handler.triggers.process.heap_size",
    "sysmon_handler.triggers.port",
    "sysmon_handler.triggers.distribution_port",
    // Raft
    "raft.segment_max_entries",
    "raft.wal_max_size_bytes",
    "raft.wal_max_entries",
    "raft.wal_hibernate_after",
    "raft.wal_max_batch_size",
    "raft.snapshot_chunk_size",
    "raft.data_dir",
    "raft.adaptive_failure_detector.poll_interval",
    // Exchange types
    "exchange_types.local_random.enabled",
    // Quorum queues
    "quorum_queue.compute_checksums",
    "quorum_queue.property_equivalence.relaxed_checks_on_redeclaration",
    "quorum_queue.initial_cluster_size",
    "quorum_queue.commands_soft_limit",
    "quorum_queue.continuous_membership_reconciliation.enabled",
    "quorum_queue.continuous_membership_reconciliation.auto_remove",
    "quorum_queue.continuous_membership_reconciliation.interval",
    "quorum_queue.continuous_membership_reconciliation.trigger_interval",
    "quorum_queue.continuous_membership_reconciliation.target_group_size",
    // Runtime parameters
    "runtime_parameters.limits.*",
    // Message interceptors
    "message_interceptors.*.*.*",
    // Streams
    "stream.replication.address_family",
    "stream.replication.port_range.min",
    "stream.replication.port_range.max",
    "stream.data_dir",
    "stream.read_ahead",
    "stream.read_ahead_limit",
    // Tags
    "cluster_tags.*",
    "node_tags.*",
];

/// Check if a key follows the valid format (dot-separated identifiers)
pub fn is_valid_key_format(key: &str) -> bool {
    if key.is_empty() {
        return false;
    }

    for part in key.split('.') {
        if part.is_empty() {
            return false;
        }
        // Allow purely numeric segments (e.g., auth_backends.1)
        if part.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        // Each part must start with a letter or underscore, followed by
        // alphanumeric, underscore, or hyphen characters
        let mut chars = part.chars();
        let first = chars.next().unwrap();
        if !first.is_ascii_alphabetic() && first != '_' {
            return false;
        }
        for c in chars {
            if !c.is_ascii_alphanumeric() && c != '_' && c != '-' {
                return false;
            }
        }
    }

    true
}

/// Check if a key matches any known cuttlefish schema pattern
pub fn is_known_key(key: &str) -> bool {
    KNOWN_KEY_PATTERNS
        .iter()
        .any(|pattern| matches_pattern(key, pattern))
}

/// Check if a key matches a pattern (with `*` as wildcard for a single segment)
fn matches_pattern(key: &str, pattern: &str) -> bool {
    let key_parts: Vec<&str> = key.split('.').collect();
    let pattern_parts: Vec<&str> = pattern.split('.').collect();

    if key_parts.len() != pattern_parts.len() {
        return false;
    }

    for (k, p) in key_parts.iter().zip(pattern_parts.iter()) {
        if *p != "*" && k != p {
            return false;
        }
    }

    true
}

/// Suggest similar keys for an unknown key
pub fn suggest_similar_keys(key: &str) -> Vec<&'static str> {
    let key_parts: Vec<&str> = key.split('.').collect();

    KNOWN_KEY_PATTERNS
        .iter()
        .filter(|pattern| {
            let pattern_parts: Vec<&str> = pattern.split('.').collect();
            // Match if the first part is the same
            if let (Some(k), Some(p)) = (key_parts.first(), pattern_parts.first()) {
                *k == *p || *p == "*"
            } else {
                false
            }
        })
        .take(5)
        .copied()
        .collect()
}
