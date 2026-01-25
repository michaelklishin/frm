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

use crate::errors::Error;
use crate::paths::Paths;
use crate::version::Version;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Nu,
}

impl Shell {
    pub fn detect() -> Option<Self> {
        if let Ok(shell) = env::var("FRM_SHELL") {
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
        let sbin_path = paths.version_sbin_dir(version);
        let sbin_str = sbin_path.display();
        let base_dir = paths.base_dir().display();

        match self {
            Shell::Bash | Shell::Zsh => {
                format!(
                    r#"export PATH="{sbin_str}:${{PATH//*{base_dir}\/versions\/*/}}"
export RABBITMQ_HOME="{version_dir}"
"#,
                    sbin_str = sbin_str,
                    base_dir = base_dir,
                    version_dir = paths.version_dir(version).display()
                )
            }
            Shell::Nu => {
                format!(
                    r#"$env.PATH = ("{sbin_str}" | split row (char esep)) ++ ($env.PATH | where {{ |p| not ($p | str contains "{base_dir}/versions") }})
$env.RABBITMQ_HOME = "{version_dir}"
"#,
                    sbin_str = sbin_str,
                    base_dir = base_dir,
                    version_dir = paths.version_dir(version).display()
                )
            }
        }
    }

    pub fn init_script(&self, paths: &Paths) -> String {
        let base_dir = paths.base_dir().display();

        match self {
            Shell::Bash => {
                format!(
                    r#"# frm initialization for bash
# Add to ~/.bashrc or ~/.bash_profile:
#   eval "$(frm env bash)"

__frm_use() {{
    local version="$1"
    if [ -z "$version" ]; then
        version=$(cat "{base_dir}/default" 2>/dev/null)
    fi
    if [ -n "$version" ] && [ -d "{base_dir}/versions/$version/sbin" ]; then
        export PATH="{base_dir}/versions/$version/sbin:${{PATH//*{base_dir}\/versions\/*/}}"
        export RABBITMQ_HOME="{base_dir}/versions/$version"
    fi
}}

# Load default version if set
__frm_use
"#,
                    base_dir = base_dir
                )
            }
            Shell::Zsh => {
                format!(
                    r#"# frm initialization for zsh
# Add to ~/.zshrc:
#   eval "$(frm env zsh)"

__frm_use() {{
    local version="$1"
    if [[ -z "$version" ]]; then
        version=$(cat "{base_dir}/default" 2>/dev/null)
    fi
    if [[ -n "$version" ]] && [[ -d "{base_dir}/versions/$version/sbin" ]]; then
        export PATH="{base_dir}/versions/$version/sbin:${{PATH//*{base_dir}\/versions\/*/}}"
        export RABBITMQ_HOME="{base_dir}/versions/$version"
    fi
}}

# Load default version if set
__frm_use
"#,
                    base_dir = base_dir
                )
            }
            Shell::Nu => {
                format!(
                    r#"# frm initialization for nushell
# Add to ~/.config/nushell/config.nu:
#   source ~/.local/frm/env.nu
# Or run: frm env nu | save -f ~/.local/frm/env.nu

def --env frm-use [version?: string] {{
    let ver = if ($version | is-empty) {{
        open "{base_dir}/default" | str trim
    }} else {{
        $version
    }}

    let sbin = $"{base_dir}/versions/($ver)/sbin"
    if ($sbin | path exists) {{
        $env.PATH = ($sbin | split row (char esep)) ++ ($env.PATH | where {{ |p| not ($p | str contains "{base_dir}/versions") }})
        $env.RABBITMQ_HOME = $"{base_dir}/versions/($ver)"
    }}
}}

# Load default version if set
if ("{base_dir}/default" | path exists) {{
    frm-use
}}
"#,
                    base_dir = base_dir
                )
            }
        }
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
