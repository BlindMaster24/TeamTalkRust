//! Global hotkey management.
use super::Client;
use teamtalk_sys as ffi;

impl Client {
    /// Registers a global hotkey.
    pub fn register_hotkey(&self, id: i32, vk_codes: &[i32]) -> bool {
        #[cfg(windows)]
        unsafe {
            ffi::api().TT_HotKey_Register(self.ptr.0, id, vk_codes.as_ptr(), vk_codes.len() as i32)
                == 1
        }
        #[cfg(not(windows))]
        false
    }

    /// Unregisters a global hotkey.
    pub fn unregister_hotkey(&self, id: i32) -> bool {
        #[cfg(windows)]
        unsafe { ffi::api().TT_HotKey_Unregister(self.ptr.0, id) == 1 }
        #[cfg(not(windows))]
        false
    }

    /// Checks if a hotkey is active.
    pub fn is_hotkey_active(&self, id: i32) -> bool {
        #[cfg(windows)]
        unsafe { ffi::api().TT_HotKey_IsActive(self.ptr.0, id) == 1 }
        #[cfg(not(windows))]
        false
    }

    /// Installs a hotkey test hook (Windows only).
    ///
    /// # Safety
    /// `hwnd` must be a valid window handle.
    #[cfg(windows)]
    pub unsafe fn install_hotkey_test_hook(&self, hwnd: ffi::HWND, msg: u32) -> bool {
        unsafe { ffi::api().TT_HotKey_InstallTestHook(self.ptr.0, hwnd, msg) == 1 }
    }

    /// Removes the hotkey test hook.
    pub fn remove_hotkey_test_hook(&self) -> bool {
        #[cfg(windows)]
        unsafe { ffi::api().TT_HotKey_RemoveTestHook(self.ptr.0) == 1 }
        #[cfg(not(windows))]
        false
    }

    /// Returns the string representation of a key.
    pub fn get_key_string(&self, vk_code: i32) -> String {
        #[cfg(windows)]
        {
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
        #[cfg(not(windows))]
        String::new()
    }
}
