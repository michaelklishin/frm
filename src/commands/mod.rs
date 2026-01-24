// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod cli_cmd;
mod completions;
mod default;
mod env;
mod fg_node;
mod install;
mod list;
mod logs;
mod reinstall;
mod show;
mod uninstall;
mod use_cmd;

pub use cli_cmd::RABBITMQ_TOOLS;
pub use cli_cmd::run as cli;
pub use completions::run as completions;
pub use default::run as default;
pub use env::run as env;
pub use fg_node::run as fg_node;
pub use install::run as install;
pub use list::run as list;
pub use logs::path as logs_path;
pub use logs::tail as logs_tail;
pub use reinstall::run as reinstall;
pub use show::CONFIG_FILES;
pub use show::run as show;
pub use uninstall::run as uninstall;
pub use use_cmd::run as use_version;
