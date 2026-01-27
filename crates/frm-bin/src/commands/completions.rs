// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use bel7_cli::generate_completions_to_stdout;

use crate::Result;
use crate::cli::{CompletionShell, build_cli};

pub fn run(shell: CompletionShell) -> Result<()> {
    let mut cmd = build_cli();
    generate_completions_to_stdout(shell, &mut cmd, "frm");
    Ok(())
}
