#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::all)]

use once_cell::sync::OnceCell;
use std::sync::Arc;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

static INSTANCE: OnceCell<Arc<TeamTalk5>> = OnceCell::new();

pub fn load(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let lib = unsafe { TeamTalk5::new(path)? };
    INSTANCE.set(Arc::new(lib)).map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "TeamTalk DLL is already loaded for this process; multiple Client instances are supported, but loading a second DLL instance/path is not supported",
        )
    })?;
    Ok(())
}

pub fn api() -> Arc<TeamTalk5> {
    INSTANCE
        .get()
        .expect("TeamTalk DLL not loaded! Call teamtalk_sys::load() first.")
        .clone()
}
