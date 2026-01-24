// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::Result;
use crate::errors::Error;
use crate::paths::Paths;
use crate::shell::Shell;
use crate::version::Version;

pub fn run(paths: &Paths, version: &Version, shell: Option<Shell>) -> Result<()> {
    if !paths.version_installed(version) {
        let versions = paths.installed_versions()?;
        if versions.is_empty() {
            eprintln!("No versions installed. Install one with:");
            eprintln!("  frm install {}", version);
        } else {
            eprintln!("Installed versions:");
            for v in &versions {
                eprintln!("  {}", v);
            }
            eprintln!("\nInstall this version with:");
            eprintln!("  frm install {}", version);
        }

        return Err(Error::VersionNotInstalled(version.clone()));
    }

    let shell = shell.or_else(Shell::detect).unwrap_or(Shell::Bash);
    print!("{}", shell.env_script(paths, version));

    Ok(())
}
