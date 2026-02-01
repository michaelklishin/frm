// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;
use std::fmt;
use std::str::FromStr;

use clap::ValueEnum;

use crate::common::env_vars::FRM_SHELL;
use crate::errors::Error;
use crate::paths::Paths;
use crate::version::Version;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Nu,
}

const ENV_BASH_TEMPLATE: &str = include_str!("../shells/env/bash.template");
const ENV_ZSH_TEMPLATE: &str = include_str!("../shells/env/zsh.template");
const ENV_NU_TEMPLATE: &str = include_str!("../shells/env/nu.template");

const INIT_BASH_TEMPLATE: &str = include_str!("../shells/init/bash.template");
const INIT_ZSH_TEMPLATE: &str = include_str!("../shells/init/zsh.template");
const INIT_NU_TEMPLATE: &str = include_str!("../shells/init/nu.template");

impl Shell {
    pub fn detect() -> Option<Self> {
        if let Ok(shell) = env::var(FRM_SHELL) {
            return shell.parse().ok();
        }

        if env::var("NU_VERSION").is_ok() {
            return Some(Shell::Nu);
        }

        env::var("SHELL").ok().and_then(|s| {
            if s.contains("bash") {
                Some(Shell::Bash)
            } else if s.contains("zsh") {
                Some(Shell::Zsh)
            } else if s.contains("nu") {
                Some(Shell::Nu)
            } else {
                None
            }
        })
    }

    pub fn env_script(&self, paths: &Paths, version: &Version) -> String {
        let sbin_path = paths.version_sbin_dir(version).display().to_string();
        let base_dir = paths.base_dir().display().to_string();
        let version_dir = paths.version_dir(version).display().to_string();

        let template = match self {
            Shell::Bash => ENV_BASH_TEMPLATE,
            Shell::Zsh => ENV_ZSH_TEMPLATE,
            Shell::Nu => ENV_NU_TEMPLATE,
        };

        template
            .replace("{{sbin_path}}", &sbin_path)
            .replace("{{base_dir}}", &base_dir)
            .replace("{{version_dir}}", &version_dir)
    }

    pub fn init_script(&self, paths: &Paths) -> String {
        let base_dir = paths.base_dir().display().to_string();

        let template = match self {
            Shell::Bash => INIT_BASH_TEMPLATE,
            Shell::Zsh => INIT_ZSH_TEMPLATE,
            Shell::Nu => INIT_NU_TEMPLATE,
        };

        template.replace("{{base_dir}}", &base_dir)
    }
}

impl fmt::Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shell::Bash => write!(f, "bash"),
            Shell::Zsh => write!(f, "zsh"),
            Shell::Nu => write!(f, "nu"),
        }
    }
}

impl FromStr for Shell {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bash" => Ok(Shell::Bash),
            "zsh" => Ok(Shell::Zsh),
            "nu" | "nushell" => Ok(Shell::Nu),
            _ => Err(Error::Config(format!("unsupported shell: {}", s))),
        }
    }
}
