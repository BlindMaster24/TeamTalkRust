//! Global hotkey management.
use super::Client;
use crate::types::HotkeyId;
use teamtalk_sys as ffi;

impl Client {
    /// Registers a global hotkey.
    ///
    /// Returns `false` when the client was created with `Client::new()` instead of
    /// `Client::with_hwnd()`.
    pub fn register_hotkey(&self, id: HotkeyId, vk_codes: &[i32]) -> bool {
        if !self.is_hwnd_client() {
            return false;
        }
        unsafe {
            ffi::api().TT_HotKey_Register(
                self.ptr.0,
                id.raw(),
                vk_codes.as_ptr(),
                vk_codes.len() as i32,
            ) == 1
        }
    }

    /// Unregisters a global hotkey.
    ///
    /// Returns `false` when the client is not HWND-backed.
    pub fn unregister_hotkey(&self, id: HotkeyId) -> bool {
        if !self.is_hwnd_client() {
            return false;
        }
        unsafe { ffi::api().TT_HotKey_Unregister(self.ptr.0, id.raw()) == 1 }
    }

    /// Checks if a hotkey is active.
    ///
    /// Returns `false` when the client is not HWND-backed.
    pub fn is_hotkey_active(&self, id: HotkeyId) -> bool {
        if !self.is_hwnd_client() {
            return false;
        }
        unsafe { ffi::api().TT_HotKey_IsActive(self.ptr.0, id.raw()) == 1 }
    }

    /// Installs a hotkey test hook (Windows only).
    ///
    /// # Safety
    /// - `hwnd` must be a valid window handle.
    /// - `msg` must be a valid message ID routed to `hwnd`.
    /// - The window's message loop must remain alive while the hook is installed.
    #[cfg(windows)]
    pub unsafe fn install_hotkey_test_hook(&self, hwnd: ffi::HWND, msg: u32) -> bool {
        if !self.is_hwnd_client() {
            return false;
        }
        unsafe { ffi::api().TT_HotKey_InstallTestHook(self.ptr.0, hwnd, msg) == 1 }
    }

    /// Removes the hotkey test hook.
    ///
    /// Returns `false` when the client is not HWND-backed.
    pub fn remove_hotkey_test_hook(&self) -> bool {
        if !self.is_hwnd_client() {
            return false;
        }
        unsafe { ffi::api().TT_HotKey_RemoveTestHook(self.ptr.0) == 1 }
    }

    /// Returns the string representation of a key.
    pub fn get_key_string(&self, vk_code: i32) -> String {
        use crate::types::TT_STRLEN;
        use crate::utils::strings::tt_buf;
        let mut buf = tt_buf::<TT_STRLEN>();
        unsafe {
            if ffi::api().TT_HotKey_GetKeyString(self.ptr.0, vk_code, buf.as_mut_ptr()) == 1 {
                crate::utils::strings::to_string(&buf)
            } else {
                String::new()
            }
        }
    }
}
