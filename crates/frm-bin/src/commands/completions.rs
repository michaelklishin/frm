// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use clap_complete::Shell as ClapShell;
use clap_complete::generate;
use clap_complete_nushell::Nushell;

use crate::Result;
use crate::cli::{CompletionShell, build_cli};

pub fn run(shell: CompletionShell) -> Result<()> {
    let mut cmd = build_cli();
    match shell {
        CompletionShell::Bash => generate(ClapShell::Bash, &mut cmd, "frm", &mut io::stdout()),
        CompletionShell::Elvish => generate(ClapShell::Elvish, &mut cmd, "frm", &mut io::stdout()),
        CompletionShell::Fish => generate(ClapShell::Fish, &mut cmd, "frm", &mut io::stdout()),
        CompletionShell::Nushell => generate(Nushell, &mut cmd, "frm", &mut io::stdout()),
        CompletionShell::PowerShell => {
            generate(ClapShell::PowerShell, &mut cmd, "frm", &mut io::stdout())
        }
        CompletionShell::Zsh => generate(ClapShell::Zsh, &mut cmd, "frm", &mut io::stdout()),
    }
    Ok(())
}
