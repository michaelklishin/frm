// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use clap_complete::{Shell, generate};

use crate::Result;
use crate::cli::build_cli;

pub fn run(shell: Shell) -> Result<()> {
    let mut cmd = build_cli();
    generate(shell, &mut cmd, "frm", &mut io::stdout());
    Ok(())
}
