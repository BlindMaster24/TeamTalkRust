//! Runtime loader for TeamTalk SDK binaries.
use regex::Regex;
use reqwest::blocking::Client;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

// For locking and progress formatting
use fd_lock::RwLock;
use human_bytes::human_bytes;

use crate::logging::{info, info_span, warn};

// For backoff retries
use crate::utils::backoff::ExponentialBackoff;
use std::time::Duration;

const DOCS_DIR_NAME: &str = "Documentation";
const DOCS_CAPI_DIR_NAME: &str = "C-API";
const DOCS_MANIFEST_NAME: &str = "TEAMTALK_DOCUMENTATION_MANIFEST.txt";
const SDK_VERSION_URL_ENV: &str = "TEAMTALK_SDK_VERSION_URL";
const REMOTE_SDK_VERSION_URL: &str = "https://raw.githubusercontent.com/BlindMaster24/TeamTalkRust/main/crates/teamtalk/SDK_VERSION.txt";

#[derive(Debug, thiserror::Error)]
pub enum LoaderError {
    #[error("Unsupported platform: OS '{os}' Architecture '{arch}'")]
    UnsupportedPlatform { os: String, arch: String },

    #[error("Offline mode enabled but SDK files or documentation are missing")]
    OfflineMissingFiles,

    #[error("No SDK versions found on the remote server")]
    NoVersionsFound,

    #[error("Documentation directory missing in SDK archive: {0}")]
    DocumentationMissing(String),

    #[error("Failed to fetch remote SDK version: {0}")]
    RemoteVersionFetch(String),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Extraction error: {0}")]
    Extraction(#[from] sevenz_rust2::Error),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("Lock error: {0}")]
    Lock(String),
}

struct PlatformConfig {
    archive_suffix: &'static str,
    dll_name: &'static str,
    lib_name: &'static str,
}

fn get_platform_config() -> Result<PlatformConfig, LoaderError> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    let config = match (os, arch) {
        ("windows", "x86_64") => PlatformConfig {
            archive_suffix: "win64",
            dll_name: "TeamTalk5.dll",
            lib_name: "TeamTalk5.lib",
        },
        ("linux", "x86_64") => PlatformConfig {
            archive_suffix: "ubuntu22_x86_64",
            dll_name: "libTeamTalk5.so",
            lib_name: "libTeamTalk5.a",
        },
        ("linux", "aarch64") => PlatformConfig {
            archive_suffix: "raspbian_arm64",
            dll_name: "libTeamTalk5.so",
            lib_name: "libTeamTalk5.a",
        },
        ("macos", "x86_64" | "aarch64") => PlatformConfig {
            archive_suffix: "macos_universal",
            dll_name: "libTeamTalk5.dylib",
            lib_name: "libTeamTalk5.a",
        },
        _ => {
            return Err(LoaderError::UnsupportedPlatform {
                os: os.to_string(),
                arch: arch.to_string(),
            });
        }
    };
    Ok(config)
}

/// Finds the TeamTalk SDK binaries or downloads them if missing.
pub fn find_or_download_dll() -> Result<PathBuf, LoaderError> {
    let platform = get_platform_config()?;
    let dll_name = platform.dll_name;

    let sdk_dir = PathBuf::from("TEAMTALK_DLL");

    if !sdk_dir.exists() {
        fs::create_dir_all(&sdk_dir)?;
    }

    // --- LOCKING START ---
    // We use a file lock to prevent multiple processes from downloading/extracting simultaneously
    let lock_file_path = sdk_dir.join(".install_lock");
    let lock_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&lock_file_path)?;
    let mut lock = RwLock::new(lock_file);
    let _guard = lock.write().map_err(|e| LoaderError::Lock(e.to_string()))?;
    // --- LOCK ACQUIRED ---

    // Now re-evaluate state after lock is acquired (another process might have just finished)
    let version_file = sdk_dir.join("TEAMTALK_SDK_VERSION.txt");
    let dll_path = sdk_dir.join(dll_name);

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
        return Err(LoaderError::OfflineMissingFiles);
    }

    let requested_version = resolve_requested_version(
        env_version.clone(),
        pinned_version.as_deref(),
        &current_version,
        dll_exists,
        docs_complete,
    )?;

    let download_version = |version: &str| -> Result<(), LoaderError> {
        let _span = info_span!("sdk_install", version = version).entered();
        download_and_extract(&sdk_dir, version, &platform)?;
        fs::write(&version_file, version)?;
        Ok(())
    };

    let mut latest_cache: Option<String> = None;
    let mut latest_version = || -> Result<String, LoaderError> {
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

        if let Err(_err) = download_version(version) {
            warn!(
                version = version,
                error = %_err,
                "Failed to download requested SDK version. Falling back to latest."
            );
        } else {
            return Ok(dll_path);
        }
    } else if requested_version.force_latest {
        let latest = latest_version()?;
        info!(version = %latest, "Downloading latest SDK");
        download_version(&latest)?;
        return Ok(dll_path);
    }

    if dll_exists && !current_version.is_empty() {
        if !docs_complete {
            info!(
                version = %current_version,
                "Documentation missing or incomplete. Re-downloading SDK"
            );
            download_version(&current_version)?;
            return Ok(dll_path);
        }

        let latest = latest_version()?;
        if current_version == latest {
            return Ok(dll_path);
        }
        info!(
            current = %current_version,
            latest = %latest,
            "Updating SDK"
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
        let _repair_reason = if !dll_exists && !docs_complete {
            "SDK binaries and documentation missing"
        } else if !dll_exists {
            "SDK binary missing"
        } else {
            "Documentation is missing or incomplete"
        };
        info!(
            version = %repair_version,
            reason = _repair_reason,
            "Repairing SDK installation"
        );
        download_version(&repair_version)?;
        return Ok(dll_path);
    }

    let latest = latest_version()?;
    info!(version = %latest, "Fresh SDK setup");
    download_version(&latest)?;

    Ok(dll_path)
}

fn get_latest_sdk_version() -> Result<String, LoaderError> {
    let body = Client::new()
        .get("https://bearware.dk/teamtalksdk/")
        .send()?
        .text()?;
    let re = Regex::new(r##"href="(v(\d+)\.(\d+)([a-z]?))/"##)?;
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
        .ok_or(LoaderError::NoVersionsFound)
}

fn fetch_remote_sdk_version() -> Result<String, LoaderError> {
    let url = env_sdk_version_url();
    let response = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?
        .get(&url)
        .send()?;
    if !response.status().is_success() {
        return Err(LoaderError::RemoteVersionFetch(format!(
            "Remote SDK_VERSION request failed from {} with status {}",
            url,
            response.status(),
        )));
    }
    let body = response.text()?;
    let version = body.trim();
    if version.is_empty() {
        return Err(LoaderError::RemoteVersionFetch(format!(
            "Remote SDK_VERSION.txt is empty at {}",
            url
        )));
    }
    Ok(version.to_string())
}

fn env_sdk_version() -> Option<String> {
    env::var("TEAMTALK_SDK_VERSION").ok().and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

fn env_sdk_version_url() -> String {
    env::var(SDK_VERSION_URL_ENV)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| REMOTE_SDK_VERSION_URL.to_string())
}

fn pinned_sdk_version() -> Option<String> {
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

struct RequestedVersion {
    requested: Option<String>,
    force_latest: bool,
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

fn resolve_requested_version(
    env_version: Option<String>,
    pinned_version: Option<&str>,
    file_version: &str,
    dll_exists: bool,
    docs_complete: bool,
) -> Result<RequestedVersion, LoaderError> {
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
                        warn!(
                            error = %err,
                            version = %file_version,
                            "Failed to fetch remote SDK_VERSION.txt. Using installed SDK."
                        );
                        return Ok(RequestedVersion {
                            requested: Some(file_version.trim().to_string()),
                            force_latest: false,
                        });
                    }
                    return Err(LoaderError::RemoteVersionFetch(format!(
                        "Failed to fetch remote SDK_VERSION.txt and no installed SDK is available: {}",
                        err
                    )));
                }
            }
        }
    }

    Ok(requested_version(None, pinned_version, file_version))
}

fn download_and_extract(
    target_dir: &Path,
    version: &str,
    platform: &PlatformConfig,
) -> Result<(), LoaderError> {
    // 1. Setup temporary directory for atomic installation
    let temp_install_dir = target_dir.join(".tmp_install");
    if temp_install_dir.exists() {
        fs::remove_dir_all(&temp_install_dir)?;
    }
    fs::create_dir_all(&temp_install_dir)?;

    let archive_name = format!("tt5sdk_{}_{}.7z", version, platform.archive_suffix);
    let url = format!(
        "https://bearware.dk/teamtalksdk/{}/{}",
        version, archive_name
    );

    // 2. Global Cache Setup
    let cache_dir = dirs::cache_dir()
        .unwrap_or_else(env::temp_dir)
        .join("teamtalk-rs");
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir)?;
    }
    let archive_path = cache_dir.join(&archive_name);

    // Check if valid archive is already in cache (> 1MB)
    let is_cached = archive_path.exists()
        && fs::metadata(&archive_path)
            .map(|m| m.len() > 1024 * 1024)
            .unwrap_or(false);

    if is_cached {
        info!(
            path = %archive_path.display(),
            "Using cached SDK archive"
        );
    } else {
        info!(url = %url, "Downloading SDK archive");

        let mut backoff =
            ExponentialBackoff::new(Duration::from_secs(1), Duration::from_secs(15), 2.0, 0.1);
        let max_attempts = 5;
        let mut attempt = 0;

        let mut response = loop {
            attempt += 1;
            match Client::builder()
                .timeout(Duration::from_secs(300))
                .build()?
                .get(&url)
                .send()
            {
                Ok(resp) if resp.status().is_success() => break resp,
                Ok(resp) => {
                    let err = LoaderError::Network(resp.error_for_status().unwrap_err());
                    if attempt >= max_attempts {
                        let _ = fs::remove_dir_all(&temp_install_dir);
                        return Err(err);
                    }
                }
                Err(e) => {
                    if attempt >= max_attempts {
                        let _ = fs::remove_dir_all(&temp_install_dir);
                        return Err(LoaderError::Network(e));
                    }
                    let delay = backoff.next_delay();
                    warn!(
                        attempt = attempt,
                        max_attempts = max_attempts,
                        error = %e,
                        delay_sec = delay.as_secs_f32(),
                        "Download failed, retrying"
                    );
                    std::thread::sleep(delay);
                }
            }
        };

        // 3. Chunked Download with Progress
        let _total_size = response.content_length().unwrap_or(0);
        let _total_str = if _total_size > 0 {
            human_bytes(_total_size as f64)
        } else {
            "Unknown".to_string()
        };

        // Download to a temporary file first, to prevent corrupted cache
        let temp_archive_path = cache_dir.join(format!("{}.part", archive_name));
        let mut out_file = File::create(&temp_archive_path)?;

        let mut buffer = [0; 8192];
        let mut downloaded: u64 = 0;
        let mut last_reported: u64 = 0;
        let report_interval = 2 * 1024 * 1024; // Report every 2 MB

        loop {
            let n = response.read(&mut buffer)?;
            if n == 0 {
                break; // EOF
            }
            out_file.write_all(&buffer[..n])?;
            downloaded += n as u64;

            if downloaded - last_reported >= report_interval {
                let _percent = if _total_size > 0 {
                    (downloaded as f64 / _total_size as f64) * 100.0
                } else {
                    0.0
                };
                info!(
                    downloaded = %human_bytes(downloaded as f64),
                    total = %_total_str,
                    percent = %format!("{:.0}%", _percent),
                    "Downloading..."
                );
                last_reported = downloaded;
            }
        }

        // Final progress report
        info!("Download complete. Extracting...");

        // Move fully downloaded file to the actual cache path
        fs::rename(&temp_archive_path, &archive_path)?;
    } // End of download block

    // 4. Decompress from cache to the temporary install directory
    let decompress_result = sevenz_rust2::decompress_file(&archive_path, &temp_install_dir);

    // If decompression fails, the cached file might be corrupted. Remove it so it downloads fresh next time.
    if decompress_result.is_err() {
        let _ = fs::remove_file(&archive_path);
    }
    decompress_result?; // Bubble up extraction error if any

    // 5. Find the necessary files in the extracted directory
    let mut f_dll = None;
    let mut f_lib = None;
    let mut f_h = None;

    find_files_recursive(
        &temp_install_dir,
        platform.dll_name,
        platform.lib_name,
        "TeamTalk.h",
        &mut f_dll,
        &mut f_lib,
        &mut f_h,
    );

    // 5. Move files from their nested locations to the root of the temporary install dir
    if let Some(src) = f_dll {
        fs::rename(&src, temp_install_dir.join(platform.dll_name))?;
    }
    if let Some(src) = f_lib {
        fs::rename(&src, temp_install_dir.join(platform.lib_name))?;
    }
    if let Some(src) = f_h {
        fs::rename(&src, temp_install_dir.join("TeamTalk.h"))?;
    }

    // 6. Handle Documentation
    let docs_src = find_directory_recursive(&temp_install_dir, DOCS_DIR_NAME).ok_or_else(|| {
        LoaderError::DocumentationMissing("Documentation folder missing".to_string())
    })?;

    let docs_capi_src = docs_src.join(DOCS_CAPI_DIR_NAME);
    if !docs_capi_src.is_dir() {
        return Err(LoaderError::DocumentationMissing(format!(
            "{}/{}",
            DOCS_DIR_NAME, DOCS_CAPI_DIR_NAME
        )));
    }

    let docs_root_dst = temp_install_dir.join(DOCS_DIR_NAME);
    let docs_capi_dst = docs_root_dst.join(DOCS_CAPI_DIR_NAME);
    let mut docs_files = copy_directory_recursive(&docs_capi_src, &docs_capi_dst)?;
    docs_files = docs_files
        .into_iter()
        .map(|rel| format!("{DOCS_CAPI_DIR_NAME}/{rel}"))
        .collect();
    if docs_files.is_empty() {
        return Err(LoaderError::DocumentationMissing(
            "Documentation directory is empty".to_string(),
        ));
    }

    // Write manifest to the temporary install directory
    let manifest = docs_files.join("\n");
    fs::write(docs_manifest_path(&temp_install_dir), manifest)?;

    // 7. Atomic Install (Rename temp_install_dir contents to target_dir)
    // We move the necessary files from temp_install_dir directly to target_dir
    fs::rename(
        temp_install_dir.join(platform.dll_name),
        target_dir.join(platform.dll_name),
    )?;

    // lib file might not be strictly required on all systems if using dylib directly, but we assume it's copied
    if temp_install_dir.join(platform.lib_name).exists() {
        fs::rename(
            temp_install_dir.join(platform.lib_name),
            target_dir.join(platform.lib_name),
        )?;
    }

    fs::rename(
        temp_install_dir.join("TeamTalk.h"),
        target_dir.join("TeamTalk.h"),
    )?;
    fs::rename(
        docs_manifest_path(&temp_install_dir),
        docs_manifest_path(target_dir),
    )?;

    // Atomic move of documentation folder
    let final_docs_dir = target_dir.join(DOCS_DIR_NAME);
    if final_docs_dir.exists() {
        fs::remove_dir_all(&final_docs_dir)?;
    }
    fs::rename(docs_root_dst, final_docs_dir)?;

    // Cleanup temp directory
    let _ = fs::remove_dir_all(&temp_install_dir);

    Ok(())
}

fn find_files_recursive(
    dir: &Path,
    dll: &str,
    lib: &str,
    h: &str,
    f_dll: &mut Option<PathBuf>,
    f_lib: &mut Option<PathBuf>,
    f_h: &mut Option<PathBuf>,
) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                find_files_recursive(&path, dll, lib, h, f_dll, f_lib, f_h);
            } else {
                let name = path.file_name().and_then(|n| n.to_str());
                if name == Some(dll) {
                    *f_dll = Some(path.clone());
                }
                if name == Some(lib) {
                    *f_lib = Some(path.clone());
                }
                if name == Some(h) {
                    *f_h = Some(path.clone());
                }
            }
        }
    }
}

fn find_directory_recursive(dir: &Path, dir_name: &str) -> Option<PathBuf> {
    if dir.file_name().and_then(|name| name.to_str()) == Some(dir_name) {
        return Some(dir.to_path_buf());
    }
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            if let Some(found) = find_directory_recursive(&path, dir_name) {
                return Some(found);
            }
        }
    }
    None
}

fn copy_directory_recursive(src_dir: &Path, dst_dir: &Path) -> Result<Vec<String>, std::io::Error> {
    if dst_dir.exists() {
        fs::remove_dir_all(dst_dir)?;
    }
    let mut copied_files = Vec::new();
    copy_directory_recursive_inner(src_dir, src_dir, dst_dir, &mut copied_files)?;
    copied_files.sort();
    Ok(copied_files)
}

fn copy_directory_recursive_inner(
    root_src: &Path,
    src_dir: &Path,
    dst_dir: &Path,
    copied_files: &mut Vec<String>,
) -> std::io::Result<()> {
    fs::create_dir_all(dst_dir)?;
    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst_dir.join(entry.file_name());
        if src_path.is_dir() {
            copy_directory_recursive_inner(root_src, &src_path, &dst_path, copied_files)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
            if let Ok(relative) = src_path.strip_prefix(root_src) {
                copied_files.push(relative.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    Ok(())
}

fn docs_manifest_path(sdk_dir: &Path) -> PathBuf {
    sdk_dir.join(DOCS_MANIFEST_NAME)
}

fn documentation_is_complete(sdk_dir: &Path) -> bool {
    let docs_dir = sdk_dir.join(DOCS_DIR_NAME);
    if !docs_dir.is_dir() {
        return false;
    }
    let manifest = match fs::read_to_string(docs_manifest_path(sdk_dir)) {
        Ok(manifest) => manifest,
        Err(_) => return false,
    };
    let mut has_entries = false;
    for line in manifest.lines() {
        let rel = line.trim();
        if rel.is_empty() {
            continue;
        }
        has_entries = true;
        let file_path: PathBuf = rel.split('/').collect();
        let full_path = docs_dir.join(file_path);
        if !full_path.is_file() {
            return false;
        }
        if fs::metadata(full_path).map(|meta| meta.len()).unwrap_or(0) == 0 {
            return false;
        }
    }
    has_entries
}
