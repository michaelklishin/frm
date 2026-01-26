// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! rabbitmq-versioning: RabbitMQ version parsing, comparison, and artifact URL generation.
//!
//! This crate provides operations on RabbitMQ versions (version strings),
//! including support for prerelease versions (alphas, betas, RCs).
//!
//! # Examples
//!
//! ```
//! use rabbitmq_versioning::{Version, Prerelease};
//!
//! // Parse a GA version
//! let v: Version = "4.2.3".parse().unwrap();
//! assert!(v.is_ga());
//!
//! // Parse a prerelease version
//! let alpha: Version = "4.3.0-alpha.1".parse().unwrap();
//! assert!(alpha.is_alpha());
//! assert!(!alpha.is_ga());
//!
//! // Versions are comparable
//! assert!(v < alpha.base_version());
//! ```

pub mod errors;
pub mod prerelease;
pub mod version;

pub use errors::Error;
pub use prerelease::Prerelease;
pub use version::Version;

pub type Result<T> = std::result::Result<T, Error>;
