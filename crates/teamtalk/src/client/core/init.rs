use super::Client;
use crate::events::Result;
use std::sync::Arc;

pub(super) fn new_client() -> Result<Client> {
    crate::init()?;
    let backend: Arc<dyn super::super::backend::TeamTalkBackend> =
        Arc::new(super::super::backend::FfiBackend);
    let ptr = backend.init_poll();
    Client::with_initialized_backend(backend, ptr)
}

#[cfg(windows)]
pub(super) unsafe fn new_client_with_hwnd(hwnd: super::ffi::HWND, msg: u32) -> Result<Client> {
    crate::init()?;
    let backend: Arc<dyn super::super::backend::TeamTalkBackend> =
        Arc::new(super::super::backend::FfiBackend);
    let ptr = backend.init_hwnd(hwnd, msg);
    Client::with_initialized_backend(backend, ptr)
}

#[cfg(feature = "mock")]
pub(super) fn new_client_with_backend(
    backend: Arc<dyn super::super::backend::TeamTalkBackend>,
) -> Result<Client> {
    let ptr = backend.init_poll();
    Client::with_initialized_backend(backend, ptr)
}
