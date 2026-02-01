// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

use futures_util::StreamExt;

use crate::Result;
use crate::common::http::USER_AGENT;
use crate::common::urls::RABBITMQ_SIGNING_KEY_URL;
use crate::errors::Error;
use crate::paths::Paths;
use crate::version::Version;

pub async fn run(paths: &Paths, version: &Version) -> Result<()> {
    if version.is_distributed_via_server_packages_repository() {
        return Err(Error::AlphaVersionNotSupported);
    }

    if !paths.version_installed(version) {
        return Err(Error::VersionNotInstalled(version.clone()));
    }

    let client = reqwest::Client::new();

    paths.ensure_dirs()?;

    let key_path = paths
        .downloads_dir()
        .join("rabbitmq-release-signing-key.asc");
    if !key_path.exists() {
        download_file(&client, RABBITMQ_SIGNING_KEY_URL, &key_path).await?;
    }

    import_gpg_key(&key_path)?;

    let archive_url = version.download_url();
    let signature_url = format!("{}.asc", archive_url);
    let archive_path = paths.downloads_dir().join(version.archive_name());
    let signature_path = paths
        .downloads_dir()
        .join(format!("{}.asc", version.archive_name()));

    if !archive_path.exists() {
        download_file(&client, &archive_url, &archive_path).await?;
    }

    download_file(&client, &signature_url, &signature_path).await?;

    verify_signature(&archive_path, &signature_path)?;

    fs::remove_file(&signature_path).ok();

    println!("OK");

    Ok(())
}

async fn download_file(client: &reqwest::Client, url: &str, dest: &Path) -> Result<()> {
    let response = client
        .get(url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await
        .map_err(|e| Error::DownloadFailed(e.to_string()))?;

    if !response.status().is_success() {
        return Err(Error::DownloadFailed(format!(
            "HTTP {}: {}",
            response.status(),
            url
        )));
    }

    let mut file = File::create(dest)?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| Error::DownloadFailed(e.to_string()))?;
        file.write_all(&chunk)?;
    }

    Ok(())
}

fn import_gpg_key(key_path: &Path) -> Result<()> {
    let output = Command::new("gpg")
        .args(["--import", &key_path.display().to_string()])
        .output()
        .map_err(|e| Error::CommandFailed(format!("failed to run gpg --import: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.contains("not changed") {
            return Err(Error::CommandFailed(format!(
                "gpg --import failed: {}",
                stderr
            )));
        }
    }

    Ok(())
}

fn verify_signature(archive_path: &Path, signature_path: &Path) -> Result<()> {
    let output = Command::new("gpg")
        .args([
            "--verify",
            &signature_path.display().to_string(),
            &archive_path.display().to_string(),
        ])
        .output()
        .map_err(|e| Error::CommandFailed(format!("failed to run gpg --verify: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::SignatureVerificationFailed(stderr.to_string()));
    }

    Ok(())
}
