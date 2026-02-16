//! Runtime loader for TeamTalk SDK binaries.
use regex::Regex;
use reqwest::blocking::Client;
use sevenz_rust2::decompress;
use std::env;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

const DOCS_DIR_NAME: &str = "Documentation";
const DOCS_CAPI_DIR_NAME: &str = "C-API";
const DOCS_MANIFEST_NAME: &str = "TEAMTALK_DOCUMENTATION_MANIFEST.txt";

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
    let requested_version = requested_version(
        env_version.clone(),
        pinned_version.as_deref(),
        &current_version,
    );
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
            eprintln!(
                "Failed to download requested SDK version {}: {}. Falling back to latest.",
                version, err
            );
        } else {
            return Ok(dll_path);
        }
    } else if requested_version.force_latest {
        let latest = latest_version()?;
        println!("Downloading latest SDK: {}", latest);
        download_version(&latest)?;
        return Ok(dll_path);
    }

    if dll_exists && !current_version.is_empty() {
        if !docs_complete {
            println!(
                "Documentation missing or incomplete. Re-downloading SDK: {}",
                current_version
            );
            download_version(&current_version)?;
            return Ok(dll_path);
        }

        let latest = latest_version()?;
        if current_version == latest {
            return Ok(dll_path);
        }
        println!("Updating SDK: {} -> {}", current_version, latest);
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
        println!("{}. Downloading SDK: {}", repair_reason, repair_version);
        download_version(&repair_version)?;
        return Ok(dll_path);
    }

    let latest = latest_version()?;
    println!("Fresh SDK setup. Downloading: {}", latest);
    download_version(&latest)?;

    Ok(dll_path)
}

fn get_latest_sdk_version() -> Result<String, Box<dyn std::error::Error>> {
    let body = Client::new()
        .get("https://bearware.dk/teamtalksdk/")
        .send()?
        .text()?;
    let re = Regex::new(r#"href="(v(\d+)\.(\d+)([a-z]?))/""#)?;
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

fn download_and_extract(
    target_dir: &Path,
    version: &str,
    dll_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = target_dir.join("tmp_ext");
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }
    fs::create_dir_all(&temp_dir)?;
    let url = if cfg!(windows) {
        format!(
            "https://bearware.dk/teamtalksdk/{}/tt5sdk_{}_win64.7z",
            version, version
        )
    } else {
        format!(
            "https://bearware.dk/teamtalksdk/{}/tt5sdk_{}_ubuntu22_x86_64.7z",
            version, version
        )
    };
    let response = Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()?
        .get(url)
        .send()?;
    decompress(Cursor::new(response.bytes()?), &temp_dir)?;

    let lib_name = if cfg!(windows) {
        "TeamTalk5.lib"
    } else {
        "libTeamTalk5.a"
    };
    let header_name = "TeamTalk.h";

    let mut f_dll = None;
    let mut f_lib = None;
    let mut f_h = None;

    find_files_recursive(
        &temp_dir,
        dll_name,
        lib_name,
        header_name,
        &mut f_dll,
        &mut f_lib,
        &mut f_h,
    );

    if let Some(src) = f_dll {
        fs::copy(&src, target_dir.join(dll_name))?;
    }
    if let Some(src) = f_lib {
        fs::copy(&src, target_dir.join(lib_name))?;
    }
    if let Some(src) = f_h {
        fs::copy(&src, target_dir.join(header_name))?;
    }

    let docs_src = find_directory_recursive(&temp_dir, DOCS_DIR_NAME)
        .ok_or("Documentation directory not found in SDK archive")?;
    let docs_root_dst = target_dir.join(DOCS_DIR_NAME);
    if docs_root_dst.exists() {
        fs::remove_dir_all(&docs_root_dst)?;
    }
    let docs_capi_src = docs_src.join(DOCS_CAPI_DIR_NAME);
    if !docs_capi_src.is_dir() {
        return Err("Documentation/C-API directory not found in SDK archive".into());
    }
    let docs_capi_dst = docs_root_dst.join(DOCS_CAPI_DIR_NAME);
    let mut docs_files = copy_directory_recursive(&docs_capi_src, &docs_capi_dst)?;
    docs_files = docs_files
        .into_iter()
        .map(|rel| format!("{DOCS_CAPI_DIR_NAME}/{rel}"))
        .collect();
    if docs_files.is_empty() {
        return Err("Documentation/C-API directory is empty in SDK archive".into());
    }
    write_documentation_manifest(target_dir, &docs_files)?;

    fs::remove_dir_all(&temp_dir)?;
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

fn copy_directory_recursive(
    src_dir: &Path,
    dst_dir: &Path,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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

fn write_documentation_manifest(
    sdk_dir: &Path,
    docs_files: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let manifest = docs_files.join("\n");
    fs::write(docs_manifest_path(sdk_dir), manifest)?;
    Ok(())
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
