// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;
use std::path::Path;

use bel7_cli::print_success;
use tool_versions::ToolVersions;

use crate::Result;
use crate::errors::Error;

const TOOL_VERSIONS_FILE: &str = ".tool-versions";

pub fn set_in_tool_versions(
    rabbitmq_version: &str,
    erlang_version: &str,
    path: Option<&Path>,
) -> Result<()> {
    let file_path = match path {
        Some(p) => p.join(TOOL_VERSIONS_FILE),
        None => env::current_dir()?.join(TOOL_VERSIONS_FILE),
    };

    let mut tv =
        ToolVersions::load_or_default(&file_path).map_err(|e| Error::Config(e.to_string()))?;

    tv.set("erlang", erlang_version);
    tv.set("rabbitmq", rabbitmq_version);

    tv.save(&file_path)
        .map_err(|e| Error::Config(e.to_string()))?;

    print_success(format!(
        "Set erlang {} and rabbitmq {} in {}",
        erlang_version,
        rabbitmq_version,
        file_path.display()
    ));

    Ok(())
}
