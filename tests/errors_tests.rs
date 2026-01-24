// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use bel7_cli::{ExitCode, ExitCodeProvider};

use frm::errors::Error;
use frm::version::Version;

#[test]
fn exit_code_version_not_installed() {
    let err = Error::VersionNotInstalled(Version::new(4, 2, 3));
    assert_eq!(err.exit_code(), ExitCode::NoInput);
}

#[test]
fn exit_code_version_already_installed() {
    let err = Error::VersionAlreadyInstalled(Version::new(4, 2, 3));
    assert_eq!(err.exit_code(), ExitCode::CantCreat);
}

#[test]
fn exit_code_invalid_version() {
    let err = Error::InvalidVersion("bad".into());
    assert_eq!(err.exit_code(), ExitCode::Usage);
}

#[test]
fn exit_code_download_failed() {
    let err = Error::DownloadFailed("network error".into());
    assert_eq!(err.exit_code(), ExitCode::Unavailable);
}

#[test]
fn exit_code_extraction_failed() {
    let err = Error::ExtractionFailed("corrupt archive".into());
    assert_eq!(err.exit_code(), ExitCode::Software);
}

#[test]
fn exit_code_config() {
    let err = Error::Config("bad config".into());
    assert_eq!(err.exit_code(), ExitCode::Config);
}

#[test]
fn exit_code_unknown_tool() {
    let err = Error::UnknownTool("badtool".into());
    assert_eq!(err.exit_code(), ExitCode::Usage);
}

#[test]
fn exit_code_unknown_config_file() {
    let err = Error::UnknownConfigFile("bad.conf".into());
    assert_eq!(err.exit_code(), ExitCode::Usage);
}

#[test]
fn exit_code_file_not_found() {
    let err = Error::FileNotFound("/path/to/file".into());
    assert_eq!(err.exit_code(), ExitCode::NoInput);
}

#[test]
fn exit_code_command_failed() {
    let err = Error::CommandFailed("segfault".into());
    assert_eq!(err.exit_code(), ExitCode::Software);
}

#[test]
fn exit_code_io() {
    let err = Error::Io(io::Error::new(io::ErrorKind::NotFound, "test"));
    assert_eq!(err.exit_code(), ExitCode::IoErr);
}

#[test]
fn error_display_version_not_installed() {
    let err = Error::VersionNotInstalled(Version::new(4, 2, 3));
    assert_eq!(err.to_string(), "version 4.2.3 is not installed");
}

#[test]
fn error_display_version_already_installed() {
    let err = Error::VersionAlreadyInstalled(Version::new(4, 2, 3));
    assert_eq!(err.to_string(), "version 4.2.3 is already installed");
}

#[test]
fn error_display_invalid_version() {
    let err = Error::InvalidVersion("bad".into());
    assert_eq!(err.to_string(), "invalid version format: bad");
}

#[test]
fn error_display_unknown_config_file() {
    let err = Error::UnknownConfigFile("bad.conf".into());
    assert_eq!(err.to_string(), "unknown config file: bad.conf");
}

#[test]
fn exit_code_release_not_found() {
    let err = Error::ReleaseNotFound("4.2.3".into());
    assert_eq!(err.exit_code(), ExitCode::NoInput);
}

#[test]
fn exit_code_expected_alpha_version() {
    let err = Error::ExpectedAlphaVersion(Version::new(4, 2, 3));
    assert_eq!(err.exit_code(), ExitCode::Usage);
}

#[test]
fn exit_code_expected_non_alpha_version() {
    let v = "4.3.0-alpha.abc123".parse::<Version>().unwrap();
    let err = Error::ExpectedNonAlphaVersion(v);
    assert_eq!(err.exit_code(), ExitCode::Usage);
}

#[test]
fn exit_code_no_alpha_releases_found() {
    let err = Error::NoAlphaReleasesFound;
    assert_eq!(err.exit_code(), ExitCode::NoInput);
}

#[test]
fn exit_code_invalid_datetime() {
    let err = Error::InvalidDateTime("not a date".into());
    assert_eq!(err.exit_code(), ExitCode::Usage);
}

#[test]
fn error_display_expected_alpha_version() {
    let err = Error::ExpectedAlphaVersion(Version::new(4, 2, 3));
    assert_eq!(err.to_string(), "expected alpha version, got: 4.2.3");
}

#[test]
fn error_display_expected_non_alpha_version() {
    let v = "4.3.0-alpha.abc123".parse::<Version>().unwrap();
    let err = Error::ExpectedNonAlphaVersion(v);
    assert_eq!(
        err.to_string(),
        "expected non-alpha version, got: 4.3.0-alpha.abc123"
    );
}

#[test]
fn error_display_no_alpha_releases_found() {
    let err = Error::NoAlphaReleasesFound;
    assert_eq!(err.to_string(), "no alpha releases found");
}

#[test]
fn error_display_invalid_datetime() {
    let err = Error::InvalidDateTime("not valid".into());
    assert_eq!(err.to_string(), "invalid date/time: not valid");
}
