// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg(feature = "serde")]

use tool_versions::ToolEntry;

#[test]
fn tool_entry_serialize() {
    let entry = ToolEntry::new("erlang", "26.2.1");
    let json = serde_json::to_string(&entry).unwrap();
    assert!(json.contains("erlang"));
    assert!(json.contains("26.2.1"));
}

#[test]
fn tool_entry_deserialize() {
    let json = r#"{"name":"erlang","versions":["26.2.1"]}"#;
    let entry: ToolEntry = serde_json::from_str(json).unwrap();
    assert_eq!(entry.name, "erlang");
    assert_eq!(entry.versions, vec!["26.2.1"]);
}

#[test]
fn tool_entry_roundtrip() {
    let entry =
        ToolEntry::with_versions("python", vec!["3.12.0".to_string(), "3.11.0".to_string()]);
    let json = serde_json::to_string(&entry).unwrap();
    let deserialized: ToolEntry = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, entry);
}
