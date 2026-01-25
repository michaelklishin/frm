// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::Deserialize;

use crate::Result;
use crate::errors::Error;
use crate::version::Version;

const SERVER_PACKAGES_API_URL: &str =
    "https://api.github.com/repos/rabbitmq/server-packages/releases";

#[derive(Debug, Deserialize)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub name: String,
    pub published_at: String,
}

pub struct AlphaRelease {
    pub version: Version,
    pub tag: String,
    pub published_at: String,
}

pub async fn find_server_packages_release_tag(
    client: &reqwest::Client,
    version: &Version,
) -> Result<String> {
    let version_str = version.to_string();

    let releases: Vec<GitHubRelease> = client
        .get(SERVER_PACKAGES_API_URL)
        .query(&[("per_page", "100")])
        .header("User-Agent", "frm")
        .send()
        .await?
        .json()
        .await?;

    for release in releases {
        if release.name.contains(&version_str) {
            return Ok(release.tag_name);
        }
    }

    Err(Error::ReleaseNotFound(version_str))
}

pub async fn fetch_alpha_releases(client: &reqwest::Client) -> Result<Vec<AlphaRelease>> {
    let releases: Vec<GitHubRelease> = client
        .get(SERVER_PACKAGES_API_URL)
        .query(&[("per_page", "100")])
        .header("User-Agent", "frm")
        .send()
        .await?
        .json()
        .await?;

    let mut alpha_releases = Vec::new();

    for release in releases {
        if let Some(version) = parse_version_from_release_name(&release.name)
            && version.is_distributed_via_server_packages_repository()
        {
            alpha_releases.push(AlphaRelease {
                version,
                tag: release.tag_name,
                published_at: release.published_at,
            });
        }
    }

    Ok(alpha_releases)
}

pub async fn find_latest_alpha(client: &reqwest::Client) -> Result<AlphaRelease> {
    let releases = fetch_alpha_releases(client).await?;

    releases
        .into_iter()
        .max_by(|a, b| a.published_at.cmp(&b.published_at))
        .ok_or(Error::NoAlphaReleasesFound)
}

pub fn parse_version_from_release_name(name: &str) -> Option<Version> {
    let name = name.trim();

    if let Some(rest) = name.strip_prefix("RabbitMQ ") {
        let version_part = rest.split_whitespace().next()?;
        version_part.parse().ok()
    } else {
        None
    }
}
