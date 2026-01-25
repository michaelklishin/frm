// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::Path;

use crate::Result;
use crate::errors::Error;
use crate::keys;

/// Represents a line in a rabbitmq.conf file
#[derive(Debug, Clone)]
enum Line {
    /// A key-value setting
    Setting { key: String, value: String },
    /// A comment line (including the # prefix)
    Comment(String),
    /// An empty or whitespace-only line
    Empty,
}

/// A parsed RabbitMQ configuration file
#[derive(Debug, Clone)]
pub struct RabbitMQConf {
    lines: Vec<Line>,
    /// Index from key to line position for quick lookups
    key_index: BTreeMap<String, usize>,
}

impl RabbitMQConf {
    /// Create an empty configuration
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            key_index: BTreeMap::new(),
        }
    }

    /// Load configuration from a file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Self::parse(&content)
    }

    /// Parse configuration from a string
    pub fn parse(content: &str) -> Result<Self> {
        let mut lines = Vec::new();
        let mut key_index = BTreeMap::new();

        for (line_num, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                lines.push(Line::Empty);
            } else if trimmed.starts_with('#') {
                lines.push(Line::Comment(line.to_string()));
            } else if let Some((key, value)) = Self::parse_setting(line, line_num + 1)? {
                key_index.insert(key.clone(), lines.len());
                lines.push(Line::Setting { key, value });
            } else {
                return Err(Error::ParseError {
                    line: line_num + 1,
                    message: format!("invalid line: {}", line),
                });
            }
        }

        Ok(Self { lines, key_index })
    }

    /// Parse a single setting line
    fn parse_setting(line: &str, line_num: usize) -> Result<Option<(String, String)>> {
        // Handle inline comments
        let line_without_comment = if let Some(hash_pos) = Self::find_unquoted_hash(line) {
            &line[..hash_pos]
        } else {
            line
        };

        let Some(eq_pos) = line_without_comment.find('=') else {
            return Ok(None);
        };

        let key = line_without_comment[..eq_pos].trim();
        let value_part = line_without_comment[eq_pos + 1..].trim();

        if key.is_empty() {
            return Err(Error::ParseError {
                line: line_num,
                message: "empty key".to_string(),
            });
        }

        // Validate key format: dot-separated identifiers
        if !keys::is_valid_key_format(key) {
            return Err(Error::ParseError {
                line: line_num,
                message: format!("invalid key format: {}", key),
            });
        }

        // Handle quoted values
        let value = Self::parse_value(value_part);

        Ok(Some((key.to_string(), value)))
    }

    /// Find the position of a # that is not inside quotes
    fn find_unquoted_hash(line: &str) -> Option<usize> {
        let mut in_single_quote = false;

        for (i, c) in line.char_indices() {
            match c {
                '\'' => {
                    in_single_quote = !in_single_quote;
                }
                '#' if !in_single_quote => {
                    return Some(i);
                }
                _ => {}
            }
        }

        None
    }

    /// Parse a value, handling single-quoted strings
    fn parse_value(value: &str) -> String {
        let trimmed = value.trim();
        if trimmed.starts_with('\'') && trimmed.ends_with('\'') && trimmed.len() >= 2 {
            trimmed[1..trimmed.len() - 1].to_string()
        } else {
            trimmed.to_string()
        }
    }

    /// Get the value for a key as a string
    pub fn get(&self, key: &str) -> Option<&str> {
        let idx = self.key_index.get(key)?;
        if let Line::Setting { value, .. } = &self.lines[*idx] {
            Some(value)
        } else {
            None
        }
    }

    /// Get all keys matching a pattern (with `*` as wildcard for a single segment)
    pub fn get_matching(&self, pattern: &str) -> Vec<(&str, &str)> {
        let pattern_parts: Vec<&str> = pattern.split('.').collect();

        self.key_index
            .iter()
            .filter_map(|(key, idx)| {
                let key_parts: Vec<&str> = key.split('.').collect();

                if key_parts.len() != pattern_parts.len() {
                    return None;
                }

                for (k, p) in key_parts.iter().zip(pattern_parts.iter()) {
                    if *p != "*" && k != p {
                        return None;
                    }
                }

                if let Line::Setting { value, .. } = &self.lines[*idx] {
                    Some((key.as_str(), value.as_str()))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Check if a pattern contains wildcards
    pub fn is_pattern(key: &str) -> bool {
        key.contains('*')
    }

    /// Get the value for a key as an integer
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key)?.parse().ok()
    }

    /// Get the value for a key as a boolean
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.get(key)? {
            "true" | "on" | "yes" | "1" => Some(true),
            "false" | "off" | "no" | "0" => Some(false),
            _ => None,
        }
    }

    /// Get the value for a key as a float
    pub fn get_float(&self, key: &str) -> Option<f64> {
        self.get(key)?.parse().ok()
    }

    /// Set a key to a value, updating existing or appending new
    pub fn set(&mut self, key: &str, value: &str) {
        if let Some(&idx) = self.key_index.get(key) {
            self.lines[idx] = Line::Setting {
                key: key.to_string(),
                value: value.to_string(),
            };
        } else {
            let idx = self.lines.len();
            self.key_index.insert(key.to_string(), idx);
            self.lines.push(Line::Setting {
                key: key.to_string(),
                value: value.to_string(),
            });
        }
    }

    /// Remove a key from the configuration
    pub fn remove(&mut self, key: &str) -> bool {
        if let Some(idx) = self.key_index.remove(key) {
            self.lines[idx] = Line::Empty;
            true
        } else {
            false
        }
    }

    /// List all keys in the configuration
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.key_index.keys().map(|s| s.as_str())
    }

    /// Check if a key exists
    pub fn contains_key(&self, key: &str) -> bool {
        self.key_index.contains_key(key)
    }

    /// Format a value, quoting if necessary
    fn format_value(value: &str) -> String {
        if value.contains('#') || value.contains('\'') || value.contains(' ') {
            format!("'{}'", value)
        } else {
            value.to_string()
        }
    }

    /// Save the configuration to a file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fs::write(path, self.to_string())?;
        Ok(())
    }
}

impl fmt::Display for RabbitMQConf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.lines {
            match line {
                Line::Setting { key, value } => {
                    let formatted_value = Self::format_value(value);
                    writeln!(f, "{} = {}", key, formatted_value)?;
                }
                Line::Comment(text) => {
                    writeln!(f, "{}", text)?;
                }
                Line::Empty => {
                    writeln!(f)?;
                }
            }
        }
        Ok(())
    }
}

impl Default for RabbitMQConf {
    fn default() -> Self {
        Self::new()
    }
}
