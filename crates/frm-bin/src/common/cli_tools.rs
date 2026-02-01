// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub const RABBITMQ_SERVER: &str = "rabbitmq-server";
pub const RABBITMQCTL: &str = "rabbitmqctl";
pub const LOG_FILE_PREFIX: &str = "rabbit@";

pub const RABBITMQ_CLI_TOOLS: &[&str] = &[
    RABBITMQCTL,
    "rabbitmq-diagnostics",
    "rabbitmq-plugins",
    "rabbitmq-queues",
    "rabbitmq-streams",
    "rabbitmq-upgrade",
];
