//! Windows hotkey APIs.
#[cfg(windows)]
use super::Client;
#[cfg(windows)]
use crate::types::TT_STRLEN;
#[cfg(windows)]
use crate::utils::strings::tt_buf;
#[cfg(windows)]
use teamtalk_sys as ffi;

#[cfg(windows)]
impl Client {
    /// Registers a hotkey with the provided virtual key codes.
    pub fn register_hotkey(&self, id: i32, vk_codes: &[i32]) -> bool {
        unsafe {
            ffi::api().TT_HotKey_Register(self.ptr, id, vk_codes.as_ptr(), vk_codes.len() as i32)
                == 1
        }
    }

    /// Unregisters a hotkey by id.
    pub fn unregister_hotkey(&self, id: i32) -> bool {
        unsafe { ffi::api().TT_HotKey_Unregister(self.ptr, id) == 1 }
    }

    /// Returns whether a hotkey is active.
    pub fn is_hotkey_active(&self, id: i32) -> i32 {
        unsafe { ffi::api().TT_HotKey_IsActive(self.ptr, id) }
    }

    #[allow(clippy::missing_safety_doc)]
    #[cfg(windows)]
    /// Installs a hotkey test hook.
    ///
    /// # Safety
    /// `hwnd` and `msg` must be valid for the target window.
    pub unsafe fn install_hotkey_test_hook(&self, hwnd: ffi::HWND, msg: u32) -> bool {
        unsafe { ffi::api().TT_HotKey_InstallTestHook(self.ptr, hwnd, msg) == 1 }
    }

    /// Removes the hotkey test hook.
    pub fn remove_hotkey_test_hook(&self) -> bool {
        unsafe { ffi::api().TT_HotKey_RemoveTestHook(self.ptr) == 1 }
    }

    /// Returns a display string for a virtual key code.
    pub fn get_key_string(&self, vk_code: i32) -> String {
        let mut buf = tt_buf::<TT_STRLEN>();
        unsafe {
            if ffi::api().TT_HotKey_GetKeyString(self.ptr, vk_code, buf.as_mut_ptr()) == 1 {
                crate::utils::strings::to_string(&buf)
            } else {
                String::new()
            }
        }
    }
}
