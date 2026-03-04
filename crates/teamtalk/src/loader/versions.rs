use super::{LoaderLogLevel, REMOTE_SDK_VERSION_URL, SDK_VERSION_URL_ENV, loader_log};
use regex::Regex;
use reqwest::blocking::Client;
use std::env;
use std::fs;
use std::path::Path;

pub(super) struct RequestedVersion {
    pub(super) requested: Option<String>,
    pub(super) force_latest: bool,
}

pub(super) fn get_latest_sdk_version() -> Result<String, Box<dyn std::error::Error>> {
    let body = Client::new()
        .get("https://bearware.dk/teamtalksdk/")
        .send()?
        .text()?;
    let re = Regex::new(r#"href=\"(v(\d+)\.(\d+)([a-z]?))/\""#)?;
    let mut versions: Vec<(i32, i32, String, String)> = re
        .captures_iter(&body)
        .map(|cap| {
            let major = cap[2].parse::<i32>().unwrap_or(0);
            let minor = cap[3].parse::<i32>().unwrap_or(0);
            let suffix = cap[4].to_string();
            let full = cap[1].to_string();
            (major, minor, suffix, full)
        })
        .collect();
    versions.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)));
    versions
        .last()
        .map(|v| v.3.clone())
        .ok_or("No SDK versions found".into())
}

pub(super) fn env_sdk_version() -> Option<String> {
    env::var("TEAMTALK_SDK_VERSION").ok().and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

pub(super) fn pinned_sdk_version() -> Option<String> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").ok()?;
    let path = Path::new(&manifest_dir).join("SDK_VERSION.txt");
    let contents = fs::read_to_string(path).ok()?;
    let trimmed = contents.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn env_sdk_version_url() -> String {
    env::var(SDK_VERSION_URL_ENV)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| REMOTE_SDK_VERSION_URL.to_string())
}

fn fetch_remote_sdk_version() -> Result<String, Box<dyn std::error::Error>> {
    let url = env_sdk_version_url();
    let response = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?
        .get(&url)
        .send()?;
    if !response.status().is_success() {
        return Err(format!(
            "Remote SDK_VERSION request failed from {} with status {}",
            url,
            response.status(),
        )
        .into());
    }
    let body = response.text()?;
    let version = body.trim();
    if version.is_empty() {
        return Err(format!("Remote SDK_VERSION.txt is empty at {}", url).into());
    }
    Ok(version.to_string())
}

fn requested_version(
    env_version: Option<String>,
    pinned_version: Option<&str>,
    file_version: &str,
) -> RequestedVersion {
    if let Some(version) = env_version {
        if version.eq_ignore_ascii_case("latest") {
            return RequestedVersion {
                requested: None,
                force_latest: true,
            };
        }
        return RequestedVersion {
            requested: Some(version),
            force_latest: false,
        };
    }
    if let Some(version) = pinned_version
        && !version.trim().is_empty()
    {
        return RequestedVersion {
            requested: Some(version.trim().to_string()),
            force_latest: false,
        };
    }
    let trimmed = file_version.trim();
    let requested = if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    };
    RequestedVersion {
        requested,
        force_latest: false,
    }
}

pub(super) fn resolve_requested_version(
    env_version: Option<String>,
    pinned_version: Option<&str>,
    file_version: &str,
    dll_exists: bool,
    docs_complete: bool,
) -> Result<RequestedVersion, Box<dyn std::error::Error>> {
    if let Some(version) = env_version {
        return Ok(requested_version(
            Some(version),
            pinned_version,
            file_version,
        ));
    }

    if let Some(version) = pinned_version {
        let trimmed = version.trim();
        if trimmed.eq_ignore_ascii_case("latest") {
            match fetch_remote_sdk_version() {
                Ok(remote) => {
                    return Ok(requested_version(Some(remote), None, file_version));
                }
                Err(err) => {
                    if dll_exists && docs_complete && !file_version.trim().is_empty() {
                        loader_log(
                            LoaderLogLevel::Warn,
                            &format!(
                                "Failed to fetch remote SDK_VERSION.txt: {}. Using installed SDK: {}",
                                err, file_version
                            ),
                        );
                        return Ok(RequestedVersion {
                            requested: Some(file_version.trim().to_string()),
                            force_latest: false,
                        });
                    }
                    return Err(format!(
                        "Failed to fetch remote SDK_VERSION.txt and no installed SDK is available: {}",
                        err
                    )
                    .into());
                }
            }
        }
    }

    Ok(requested_version(None, pinned_version, file_version))
}
