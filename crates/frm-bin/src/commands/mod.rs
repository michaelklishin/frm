// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod clean;
mod cli_cmd;
mod completions;
mod conf;
mod cp_etc_file;
mod default;
mod env;
mod fg_node;
mod install;
mod list;
mod logs;
mod path;
mod prune;
mod reinstall;
mod show;
mod tanzu_install;
mod uninstall;
mod use_cmd;

pub use clean::run as clean_alphas;
pub use cli_cmd::RABBITMQ_TOOLS;
pub use cli_cmd::run as cli;
pub use completions::run as completions;
pub use conf::get_key as conf_get_key;
pub use conf::set_key as conf_set_key;
pub use cp_etc_file::EtcFile;
pub use cp_etc_file::run_alpha as cp_etc_file_alpha;
pub use cp_etc_file::run_release as cp_etc_file_release;
pub use default::run as default;
pub use env::run as env;
pub use fg_node::run as fg_node;
pub use install::run_alpha as install_alpha;
pub use install::run_release as install_release;
pub use list::completions_alphas;
pub use list::completions_releases;
pub use list::run_alphas as list_alphas;
pub use list::run_releases as list_releases;
pub use logs::path_alpha as logs_path_alpha;
pub use logs::path_release as logs_path_release;
pub use logs::tail_alpha as logs_tail_alpha;
pub use logs::tail_release as logs_tail_release;
pub use path::run_alpha as path_alpha;
pub use path::run_release as path_release;
pub use prune::run as prune_alphas;
pub use reinstall::run_alpha as reinstall_alpha;
pub use reinstall::run_release as reinstall_release;
pub use show::CONFIG_FILES;
pub use show::run as inspect;
pub use tanzu_install::run as tanzu_install;
pub use uninstall::run_alpha as uninstall_alpha;
pub use uninstall::run_release as uninstall_release;
pub use use_cmd::run as use_version;
