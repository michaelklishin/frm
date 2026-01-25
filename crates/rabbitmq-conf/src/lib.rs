// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! rabbitmq-conf - Parser and manipulation library for RabbitMQ configuration files
//!
//! This crate provides functionality to parse, modify, and serialize RabbitMQ
//! configuration files in the cuttlefish format.

pub mod conf;
pub mod errors;
pub mod keys;

pub use conf::RabbitMQConf;
pub use errors::Error;

pub type Result<T> = std::result::Result<T, Error>;
