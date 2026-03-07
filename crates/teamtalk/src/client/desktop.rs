//! Desktop access and sharing APIs.
use super::Client;
use crate::types::{DesktopInput, UserId};
use teamtalk_sys as ffi;

impl Client {
    /// Closes a desktop window session.
    pub fn close_desktop_window(&self) -> bool {
        unsafe { ffi::api().TT_CloseDesktopWindow(self.ptr.0) == 1 }
    }

    /// Sends mouse cursor position to the desktop sharer.
    pub fn send_desktop_cursor_position(&self, x: u16, y: u16) -> bool {
        unsafe { ffi::api().TT_SendDesktopCursorPosition(self.ptr.0, x, y) == 1 }
    }

    /// Sends keyboard or mouse input to the desktop sharer.
    pub fn send_desktop_input(&self, user_id: UserId, input: &ffi::DesktopInput) -> bool {
        unsafe { ffi::api().TT_SendDesktopInput(self.ptr.0, user_id.0, input, 1) == 1 }
    }

    /// Sends a batch of keyboard or mouse input packets to the desktop sharer.
    pub fn send_desktop_inputs(&self, user_id: UserId, inputs: &[DesktopInput]) -> bool {
        if inputs.is_empty() || inputs.len() > ffi::TT_DESKTOPINPUT_MAX as usize {
            return false;
        }
        let raw_inputs: Vec<_> = inputs.iter().map(DesktopInput::to_ffi).collect();
        unsafe {
            ffi::api().TT_SendDesktopInput(
                self.ptr.0,
                user_id.0,
                raw_inputs.as_ptr(),
                raw_inputs.len() as i32,
            ) == 1
        }
    }

    /// Converts desktop input packets using the platform translation helper.
    pub fn desktop_input_key_translate(
        &self,
        translate: ffi::TTKeyTranslate,
        inputs: &[DesktopInput],
    ) -> Option<Vec<DesktopInput>> {
        if inputs.is_empty() || inputs.len() > ffi::TT_DESKTOPINPUT_MAX as usize {
            return None;
        }
        let raw_inputs: Vec<_> = inputs.iter().map(DesktopInput::to_ffi).collect();
        let mut translated = vec![ffi::DesktopInput::default(); raw_inputs.len()];
        let count = unsafe {
            ffi::api().TT_DesktopInput_KeyTranslate(
                translate,
                raw_inputs.as_ptr(),
                translated.as_mut_ptr(),
                raw_inputs.len() as i32,
            )
        };
        if count < 0 {
            return None;
        }
        translated.truncate(count as usize);
        Some(translated.into_iter().map(DesktopInput::from).collect())
    }

    /// Executes desktop input packets locally through the SDK helper.
    pub fn execute_desktop_input(&self, inputs: &[DesktopInput]) -> i32 {
        if inputs.is_empty() || inputs.len() > ffi::TT_DESKTOPINPUT_MAX as usize {
            return -1;
        }
        let raw_inputs: Vec<_> = inputs.iter().map(DesktopInput::to_ffi).collect();
        unsafe { ffi::api().TT_DesktopInput_Execute(raw_inputs.as_ptr(), raw_inputs.len() as i32) }
    }

    /// Sends a desktop window frame to other users.
    pub fn send_desktop_window(
        &self,
        window: &ffi::DesktopWindow,
        bitmap_format: ffi::BitmapFormat,
    ) -> i32 {
        unsafe { ffi::api().TT_SendDesktopWindow(self.ptr.0, window, bitmap_format) }
    }

    #[cfg(windows)]
    /// Sends a desktop window directly from a Win32 window handle.
    ///
    /// # Safety
    /// `hwnd` must be a valid window handle for the current process and remain valid
    /// for the duration of the SDK call.
    pub unsafe fn send_desktop_window_from_hwnd(
        &self,
        hwnd: ffi::HWND,
        bitmap_format: ffi::BitmapFormat,
        protocol: ffi::DesktopProtocol,
    ) -> i32 {
        unsafe {
            ffi::api().TT_SendDesktopWindowFromHWND(self.ptr.0, hwnd, bitmap_format, protocol)
        }
    }

    /// Acquires a desktop window update bitmap.
    pub fn acquire_user_desktop_window(&self, user_id: UserId) -> Option<*mut ffi::DesktopWindow> {
        unsafe {
            let ptr = ffi::api().TT_AcquireUserDesktopWindow(self.ptr.0, user_id.0);
            if ptr.is_null() { None } else { Some(ptr) }
        }
    }

    #[allow(clippy::missing_safety_doc)]
    /// Releases a previously acquired desktop window.
    ///
    /// # Safety
    /// `window` must be a valid pointer returned by `acquire_user_desktop_window`.
    pub unsafe fn release_user_desktop_window(&self, window: *mut ffi::DesktopWindow) -> bool {
        if window.is_null() {
            return false;
        }
        unsafe { ffi::api().TT_ReleaseUserDesktopWindow(self.ptr.0, window) == 1 }
    }
}
