//! Runtime loader for TeamTalk SDK binaries.
use reqwest::blocking::Client;
use sevenz_rust2::decompress;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

const DOCS_DIR_NAME: &str = "Documentation";
const DOCS_CAPI_DIR_NAME: &str = "C-API";
const DOCS_MANIFEST_NAME: &str = "TEAMTALK_DOCUMENTATION_MANIFEST.txt";
const SDK_VERSION_URL_ENV: &str = "TEAMTALK_SDK_VERSION_URL";
const REMOTE_SDK_VERSION_URL: &str = "https://raw.githubusercontent.com/BlindMaster24/TeamTalkRust/main/crates/teamtalk/SDK_VERSION.txt";
mod versions;
use versions::{
    env_sdk_version, get_latest_sdk_version, pinned_sdk_version, resolve_requested_version,
};
mod download;
use download::{documentation_is_complete, download_and_extract};

enum LoaderLogLevel {
    Info,
    Warn,
}

fn loader_log(level: LoaderLogLevel, message: &str) {
    #[cfg(feature = "logging")]
    match level {
        LoaderLogLevel::Info => tracing::info!("{message}"),
        LoaderLogLevel::Warn => tracing::warn!("{message}"),
    }

    #[cfg(not(feature = "logging"))]
    let _ = (level, message);
}

/// Finds the TeamTalk SDK binaries or downloads them if missing.
pub fn find_or_download_dll() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let dll_name = if cfg!(windows) {
        "TeamTalk5.dll"
    } else {
        "libTeamTalk5.so"
    };
    let sdk_dir = PathBuf::from("TEAMTALK_DLL");
    let version_file = sdk_dir.join("TEAMTALK_SDK_VERSION.txt");
    let dll_path = sdk_dir.join(dll_name);

    if !sdk_dir.exists() {
        fs::create_dir_all(&sdk_dir)?;
    }

    let current_version = fs::read_to_string(&version_file)
        .unwrap_or_default()
        .trim()
        .to_string();
    let env_version = env_sdk_version();
    let pinned_version = pinned_sdk_version();
    let dll_exists = dll_path.exists()
        && fs::metadata(&dll_path)
            .map(|m| m.len() > 1024)
            .unwrap_or(false);
    let docs_complete = documentation_is_complete(&sdk_dir);

    if cfg!(feature = "offline") {
        if dll_exists && docs_complete {
            return Ok(dll_path);
        }
        return Err("Offline mode enabled but SDK files or documentation are missing".into());
    }

    let requested_version = resolve_requested_version(
        env_version.clone(),
        pinned_version.as_deref(),
        &current_version,
        dll_exists,
        docs_complete,
    )?;

    let download_version = |version: &str| -> Result<(), Box<dyn std::error::Error>> {
        download_and_extract(&sdk_dir, version, dll_name)?;
        fs::write(&version_file, version)?;
        Ok(())
    };
    let mut latest_cache: Option<String> = None;
    let mut latest_version = || -> Result<String, Box<dyn std::error::Error>> {
        if let Some(version) = latest_cache.as_ref() {
            return Ok(version.clone());
        }
        let version = get_latest_sdk_version()?;
        latest_cache = Some(version.clone());
        Ok(version)
    };

    if let Some(version) = requested_version.requested.as_deref() {
        if dll_exists && docs_complete && current_version == version {
            return Ok(dll_path);
        }

        if let Err(err) = download_version(version) {
            loader_log(
                LoaderLogLevel::Warn,
                &format!(
                    "Failed to download requested SDK version {}: {}. Falling back to latest.",
                    version, err
                ),
            );
        } else {
            return Ok(dll_path);
        }
    } else if requested_version.force_latest {
        let latest = latest_version()?;
        loader_log(
            LoaderLogLevel::Info,
            &format!("Downloading latest SDK: {}", latest),
        );
        download_version(&latest)?;
        return Ok(dll_path);
    }

    if dll_exists && !current_version.is_empty() {
        if !docs_complete {
            loader_log(
                LoaderLogLevel::Info,
                &format!(
                    "Documentation missing or incomplete. Re-downloading SDK: {}",
                    current_version
                ),
            );
            download_version(&current_version)?;
            return Ok(dll_path);
        }

        let latest = latest_version()?;
        if current_version == latest {
            return Ok(dll_path);
        }
        loader_log(
            LoaderLogLevel::Info,
            &format!("Updating SDK: {} -> {}", current_version, latest),
        );
        download_version(&latest)?;
        return Ok(dll_path);
    }

    if !dll_exists || !docs_complete {
        let repair_version = if current_version.is_empty() {
            latest_version()?
        } else {
            current_version.clone()
        };
        let repair_reason = if !dll_exists && !docs_complete {
            "SDK binaries or documentation are missing"
        } else if !dll_exists {
            "SDK binary is missing"
        } else {
            "Documentation is missing or incomplete"
        };
        loader_log(
            LoaderLogLevel::Info,
            &format!("{}. Downloading SDK: {}", repair_reason, repair_version),
        );
        download_version(&repair_version)?;
        return Ok(dll_path);
    }

    let latest = latest_version()?;
    loader_log(
        LoaderLogLevel::Info,
        &format!("Fresh SDK setup. Downloading: {}", latest),
    );
    download_version(&latest)?;

    Ok(dll_path)
}
