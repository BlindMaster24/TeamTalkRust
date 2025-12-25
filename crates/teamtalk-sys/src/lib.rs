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
    let _ = INSTANCE.set(Arc::new(lib));
    Ok(())
}

pub fn api() -> Arc<TeamTalk5> {
    INSTANCE
        .get()
        .expect("TeamTalk DLL not loaded! Call teamtalk_sys::load() first.")
        .clone()
}
