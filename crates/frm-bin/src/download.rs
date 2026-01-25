// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs::{self, File};
use std::io::{self, BufReader, Write};
use std::path::Path;

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use tar::Archive;
use xz2::read::XzDecoder;

use crate::Result;
use crate::errors::Error;
use crate::paths::Paths;
use crate::releases::find_server_packages_release_tag;
use crate::version::Version;

const TEMPLATE_RABBITMQ_CONF: &str =
    include_str!("../templates/etc/rabbitmq/template.rabbitmq.conf");
const TEMPLATE_ENABLED_PLUGINS: &str =
    include_str!("../templates/etc/rabbitmq/template.enabled_plugins");

pub struct Downloader {
    client: reqwest::Client,
}

impl Downloader {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn download(&self, version: &Version, paths: &Paths) -> Result<()> {
        let url = if version.is_distributed_via_server_packages_repository() {
            let tag = find_server_packages_release_tag(&self.client, version).await?;
            version.download_url_with_tag(&tag)
        } else {
            version.download_url()
        };

        let archive_path = paths.downloads_dir().join(version.archive_name());

        paths.ensure_dirs()?;

        if !archive_path.exists() {
            self.fetch_archive(&url, &archive_path).await?;
        }

        self.extract_archive(&archive_path, version, paths)?;

        Ok(())
    }

    async fn fetch_archive(&self, url: &str, dest: &Path) -> Result<()> {
        let response = self
            .client
            .get(url)
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

        let total_size = response.content_length().unwrap_or(0);
        let progress = if total_size > 0 {
            let pb = ProgressBar::new(total_size);
            pb.set_style(
                ProgressStyle::with_template(
                    "{elapsed_precise:.dim} {wide_bar:.cyan} {bytes}/{total_bytes} ({bytes_per_sec})",
                )
                .unwrap()
                .progress_chars("█▉▊▋▌▍▎▏  "),
            );
            Some(pb)
        } else {
            None
        };

        let mut file = File::create(dest)?;
        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| Error::DownloadFailed(e.to_string()))?;
            file.write_all(&chunk)?;
            if let Some(ref pb) = progress {
                pb.inc(chunk.len() as u64);
            }
        }

        if let Some(pb) = progress {
            pb.finish_and_clear();
        }

        Ok(())
    }

    fn extract_archive(&self, archive_path: &Path, version: &Version, paths: &Paths) -> Result<()> {
        let file = File::open(archive_path)?;
        let reader = BufReader::new(file);
        let decoder = XzDecoder::new(reader);
        let mut archive = Archive::new(decoder);

        let temp_dir = paths
            .versions_dir()
            .join(format!(".{}-extracting", version));
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir)?;
        }
        fs::create_dir_all(&temp_dir)?;

        archive
            .unpack(&temp_dir)
            .map_err(|e| Error::ExtractionFailed(e.to_string()))?;

        let extracted_name = version.extracted_dir_name();
        let extracted_path = temp_dir.join(&extracted_name);
        let final_path = paths.version_dir(version);

        if final_path.exists() {
            fs::remove_dir_all(&final_path)?;
        }

        fs::rename(&extracted_path, &final_path).map_err(|e| {
            Error::ExtractionFailed(format!("failed to move extracted directory: {}", e))
        })?;

        fs::remove_dir_all(&temp_dir)?;

        Ok(())
    }

    pub fn cleanup_archive(&self, version: &Version, paths: &Paths) -> Result<()> {
        let archive_path = paths.downloads_dir().join(version.archive_name());
        if archive_path.exists() {
            fs::remove_file(archive_path)?;
        }
        Ok(())
    }
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new()
    }
}

pub fn copy_default_config(paths: &Paths, version: &Version) -> Result<()> {
    let etc_src = paths.etc_dir();
    let etc_dest = paths.version_etc_dir(version);

    fs::create_dir_all(&etc_dest)?;

    let rabbitmq_conf = etc_dest.join("rabbitmq.conf");
    if !rabbitmq_conf.exists() {
        fs::write(&rabbitmq_conf, TEMPLATE_RABBITMQ_CONF)?;
    }

    let enabled_plugins = etc_dest.join("enabled_plugins");
    if !enabled_plugins.exists() {
        fs::write(&enabled_plugins, TEMPLATE_ENABLED_PLUGINS)?;
    }

    if !etc_src.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(&etc_src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let file_name = entry.file_name();
        let dest_path = etc_dest.join(&file_name);

        if file_type.is_file() {
            fs::copy(&src_path, &dest_path)?;
        } else if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dest_path)?;
        }
    }

    Ok(())
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> io::Result<()> {
    fs::create_dir_all(dest)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if file_type.is_file() {
            fs::copy(&src_path, &dest_path)?;
        } else if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dest_path)?;
        }
    }

    Ok(())
}
