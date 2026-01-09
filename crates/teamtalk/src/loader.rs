//! Runtime loader for TeamTalk SDK binaries.
use regex::Regex;
use reqwest::blocking::Client;
use sevenz_rust2::decompress;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

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
    let dll_exists = dll_path.exists()
        && fs::metadata(&dll_path)
            .map(|m| m.len() > 1024)
            .unwrap_or(false);

    if cfg!(feature = "offline") {
        if dll_exists {
            return Ok(dll_path);
        }
        return Err("Offline mode enabled but SDK binary not found".into());
    }

    if dll_exists && !current_version.is_empty() {
        let latest = get_latest_sdk_version()?;
        if current_version == latest {
            return Ok(dll_path);
        }
        println!("Updating SDK: {} -> {}", current_version, latest);
        download_and_extract(&sdk_dir, &latest, dll_name)?;
        fs::write(&version_file, &latest)?;
        return Ok(dll_path);
    }

    if !dll_exists && !current_version.is_empty() {
        println!(
            "DLL missing. Downloading version {} from file...",
            current_version
        );
        download_and_extract(&sdk_dir, &current_version, dll_name)?;
        return Ok(dll_path);
    }

    let latest = get_latest_sdk_version()?;
    println!("Fresh SDK setup. Downloading: {}", latest);
    download_and_extract(&sdk_dir, &latest, dll_name)?;
    fs::write(&version_file, &latest)?;

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
