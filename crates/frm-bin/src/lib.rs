// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! frm - Frakking RabbitMQ version Manager
//!
//! A tool for managing multiple RabbitMQ installations from the generic UNIX packages.

pub mod cli;
pub mod commands;
pub mod config;
pub mod download;
pub mod errors;
pub mod paths;
pub mod releases;
pub mod shell;
pub mod tanzu;
pub mod timestamps;
pub mod version_file;

pub use errors::Error;
pub use rabbitmq_versioning as version;
pub use rabbitmq_versioning::{Prerelease, Version};

pub type Result<T> = std::result::Result<T, Error>;
