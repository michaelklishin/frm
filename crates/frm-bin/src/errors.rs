// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use bel7_cli::{ExitCode, ExitCodeProvider};
use thiserror::Error;

use crate::version::Version;

#[derive(Error, Debug)]
pub enum Error {
    #[error("version {0} is not installed")]
    VersionNotInstalled(Version),

    #[error("version {0} is already installed")]
    VersionAlreadyInstalled(Version),

    #[error("invalid version format: {0}")]
    InvalidVersion(String),

    #[error("download failed: {0}")]
    DownloadFailed(String),

    #[error("release not found: {0}")]
    ReleaseNotFound(String),

    #[error("extraction failed: {0}")]
    ExtractionFailed(String),

    #[error("configuration error: {0}")]
    Config(String),

    #[error("unknown tool: {0}")]
    UnknownTool(String),

    #[error("unknown config file: {0}")]
    UnknownConfigFile(String),

    #[error("file not found: {0}")]
    FileNotFound(String),

    #[error("command failed: {0}")]
    CommandFailed(String),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("TOML serialization error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("expected an alpha version, got: {0}")]
    ExpectedAlphaVersion(Version),

    #[error("expected a non-alpha version, got: {0}")]
    ExpectedNonAlphaVersion(Version),

    #[error("no alpha releases found")]
    NoAlphaReleasesFound,

    #[error("invalid date/time: {0}")]
    InvalidDateTime(String),

    #[error("version mismatch: expected {expected}, detected {detected} in tarball filename")]
    TanzuVersionMismatch {
        expected: Version,
        detected: Version,
    },
}

impl ExitCodeProvider for Error {
    fn exit_code(&self) -> ExitCode {
        match self {
            Error::VersionNotInstalled(_) => ExitCode::NoInput,
            Error::VersionAlreadyInstalled(_) => ExitCode::CantCreat,
            Error::InvalidVersion(_) => ExitCode::Usage,
            Error::DownloadFailed(_) => ExitCode::Unavailable,
            Error::ReleaseNotFound(_) => ExitCode::NoInput,
            Error::ExtractionFailed(_) => ExitCode::Software,
            Error::Config(_) => ExitCode::Config,
            Error::UnknownTool(_) => ExitCode::Usage,
            Error::UnknownConfigFile(_) => ExitCode::Usage,
            Error::FileNotFound(_) => ExitCode::NoInput,
            Error::CommandFailed(_) => ExitCode::Software,
            Error::Io(_) => ExitCode::IoErr,
            Error::Http(_) => ExitCode::Protocol,
            Error::TomlParse(_) => ExitCode::DataErr,
            Error::TomlSerialize(_) => ExitCode::Software,
            Error::Json(_) => ExitCode::DataErr,
            Error::ExpectedAlphaVersion(_) => ExitCode::Usage,
            Error::ExpectedNonAlphaVersion(_) => ExitCode::Usage,
            Error::NoAlphaReleasesFound => ExitCode::NoInput,
            Error::InvalidDateTime(_) => ExitCode::Usage,
            Error::TanzuVersionMismatch { .. } => ExitCode::DataErr,
        }
    }
}
