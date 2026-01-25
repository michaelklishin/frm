// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;

use flate2::read::GzDecoder;
use tar::Archive;
use xz2::read::XzDecoder;

use crate::Result;
use crate::errors::Error;
use crate::paths::Paths;
use crate::version::Version;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionFormat {
    Xz,
    Gzip,
}

impl CompressionFormat {
    pub fn from_path(path: &Path) -> Option<Self> {
        let name = path.file_name()?.to_str()?;
        if name.ends_with(".tar.xz") {
            Some(CompressionFormat::Xz)
        } else if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
            Some(CompressionFormat::Gzip)
        } else {
            None
        }
    }
}

pub fn extract_version_from_tarball_name(path: &Path) -> Option<Version> {
    let name = path.file_name()?.to_str()?;

    let name_without_ext = name
        .strip_suffix(".tar.xz")
        .or_else(|| name.strip_suffix(".tar.gz"))
        .or_else(|| name.strip_suffix(".tgz"))?;

    extract_version_from_stem(name_without_ext)
}

fn extract_version_from_stem(stem: &str) -> Option<Version> {
    for (i, _) in stem.char_indices() {
        let suffix = &stem[i..];
        if let Ok(version) = suffix.parse::<Version>() {
            return Some(version);
        }
        if let Some(stripped) = suffix.strip_prefix('-')
            && let Ok(version) = stripped.parse::<Version>()
        {
            return Some(version);
        }
    }
    None
}

pub fn extract_tarball(tarball_path: &Path, version: &Version, paths: &Paths) -> Result<()> {
    let format = CompressionFormat::from_path(tarball_path).ok_or_else(|| {
        Error::ExtractionFailed(format!(
            "unsupported archive format: {}",
            tarball_path.display()
        ))
    })?;

    let file = File::open(tarball_path)?;
    let reader = BufReader::new(file);

    let temp_dir = paths
        .versions_dir()
        .join(format!(".{}-extracting", version));
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }
    fs::create_dir_all(&temp_dir)?;

    match format {
        CompressionFormat::Xz => {
            let decoder = XzDecoder::new(reader);
            let mut archive = Archive::new(decoder);
            archive
                .unpack(&temp_dir)
                .map_err(|e| Error::ExtractionFailed(e.to_string()))?;
        }
        CompressionFormat::Gzip => {
            let decoder = GzDecoder::new(reader);
            let mut archive = Archive::new(decoder);
            archive
                .unpack(&temp_dir)
                .map_err(|e| Error::ExtractionFailed(e.to_string()))?;
        }
    }

    let extracted_dir = find_extracted_rabbitmq_dir(&temp_dir)?;
    let final_path = paths.version_dir(version);

    if final_path.exists() {
        fs::remove_dir_all(&final_path)?;
    }

    fs::rename(&extracted_dir, &final_path).map_err(|e| {
        Error::ExtractionFailed(format!("failed to move extracted directory: {}", e))
    })?;

    fs::remove_dir_all(&temp_dir)?;

    Ok(())
}

fn find_extracted_rabbitmq_dir(temp_dir: &Path) -> Result<std::path::PathBuf> {
    let mut fallback = None;

    for entry in fs::read_dir(temp_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.contains("rabbitmq") {
                return Ok(path);
            }
            if fallback.is_none() {
                fallback = Some(path);
            }
        }
    }

    fallback
        .ok_or_else(|| Error::ExtractionFailed("no directory found in extracted archive".into()))
}

pub fn verify_extracted_version(paths: &Paths, expected: &Version) -> Result<()> {
    let version_dir = paths.version_dir(expected);
    if !version_dir.exists() {
        return Err(Error::ExtractionFailed(format!(
            "extracted directory not found: {}",
            version_dir.display()
        )));
    }

    let sbin_dir = version_dir.join("sbin");
    if !sbin_dir.exists() {
        return Err(Error::ExtractionFailed(
            "extracted archive does not contain sbin directory".into(),
        ));
    }

    Ok(())
}
