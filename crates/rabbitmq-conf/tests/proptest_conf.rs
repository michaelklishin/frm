// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proptest::prelude::*;
use rabbitmq_conf::RabbitMQConf;

fn key_part_strategy() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9_]{0,15}".prop_map(|s| s.to_string())
}

fn key_strategy() -> impl Strategy<Value = String> {
    prop::collection::vec(key_part_strategy(), 1..=4).prop_map(|parts| parts.join("."))
}

fn simple_value_strategy() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9_/.-]{1,50}".prop_map(|s| s.to_string())
}

proptest! {
    #[test]
    fn roundtrip_set_get(key in key_strategy(), value in simple_value_strategy()) {
        let mut conf = RabbitMQConf::new();
        conf.set(&key, &value);
        prop_assert_eq!(conf.get(&key), Some(value.as_str()));
    }

    #[test]
    fn roundtrip_parse_to_string(key in key_strategy(), value in simple_value_strategy()) {
        let content = format!("{} = {}\n", key, value);
        let conf = RabbitMQConf::parse(&content).unwrap();
        let output = conf.to_string();
        let reparsed = RabbitMQConf::parse(&output).unwrap();
        prop_assert_eq!(reparsed.get(&key), Some(value.as_str()));
    }

    #[test]
    fn set_overwrite(key in key_strategy(), value1 in simple_value_strategy(), value2 in simple_value_strategy()) {
        let mut conf = RabbitMQConf::new();
        conf.set(&key, &value1);
        conf.set(&key, &value2);
        prop_assert_eq!(conf.get(&key), Some(value2.as_str()));
    }

    #[test]
    fn remove_then_get(key in key_strategy(), value in simple_value_strategy()) {
        let mut conf = RabbitMQConf::new();
        conf.set(&key, &value);
        conf.remove(&key);
        prop_assert_eq!(conf.get(&key), None);
    }

    #[test]
    fn contains_after_set(key in key_strategy(), value in simple_value_strategy()) {
        let mut conf = RabbitMQConf::new();
        prop_assert!(!conf.contains_key(&key));
        conf.set(&key, &value);
        prop_assert!(conf.contains_key(&key));
    }

    #[test]
    fn multiple_keys_independent(
        key1 in key_strategy(),
        value1 in simple_value_strategy(),
        value2 in simple_value_strategy()
    ) {
        // Ensure we have different keys
        let key2 = format!("{}.extra", key1);

        let mut conf = RabbitMQConf::new();
        conf.set(&key1, &value1);
        conf.set(&key2, &value2);

        prop_assert_eq!(conf.get(&key1), Some(value1.as_str()));
        prop_assert_eq!(conf.get(&key2), Some(value2.as_str()));
    }
}
