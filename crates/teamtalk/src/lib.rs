use std::path::Path;

pub mod client;
pub mod events;
pub mod loader;
pub mod types;
pub mod utils;

pub use client::{Client, Message};
pub use events::{Error, Event, Result};

pub fn init() -> Result<()> {
    let dll_path = loader::find_or_download_dll().map_err(|_| Error::InitFailed)?;
    teamtalk_sys::load(dll_path.to_str().unwrap()).map_err(|_| Error::InitFailed)?;
    Ok(())
}

pub fn init_with_path<P: AsRef<Path>>(path: P) -> Result<()> {
    teamtalk_sys::load(path.as_ref().to_str().ok_or(Error::InitFailed)?)
        .map_err(|_| Error::InitFailed)?;
    Ok(())
}
