//! Desktop access and sharing APIs.
use super::Client;
use crate::types::{DesktopInput, UserId};
use teamtalk_sys as ffi;

/// Guard around a desktop window acquired from the SDK.
pub struct DesktopWindowGuard<'a> {
    client: &'a Client,
    ptr: *mut ffi::DesktopWindow,
}

impl DesktopWindowGuard<'_> {
    pub fn window(&self) -> &ffi::DesktopWindow {
        unsafe { &*self.ptr }
    }

    pub fn as_ptr(&self) -> *mut ffi::DesktopWindow {
        self.ptr
    }
}

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
        unsafe { ffi::api().TT_SendDesktopInput(self.ptr.0, user_id.raw(), input, 1) == 1 }
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
                user_id.raw(),
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
    /// Returns the active desktop window handle.
    pub fn get_desktop_active_hwnd(&self) -> ffi::HWND {
        unsafe { ffi::api().TT_Windows_GetDesktopActiveHWND() }
    }

    #[cfg(windows)]
    /// Returns the desktop root window handle.
    pub fn get_desktop_hwnd(&self) -> ffi::HWND {
        unsafe { ffi::api().TT_Windows_GetDesktopHWND() }
    }

    #[cfg(windows)]
    /// Returns a desktop window handle by index.
    pub fn get_desktop_window_hwnd(&self, index: i32) -> Option<ffi::HWND> {
        let mut hwnd = std::ptr::null_mut();
        let ok = unsafe { ffi::api().TT_Windows_GetDesktopWindowHWND(index, &mut hwnd) == 1 };
        if ok { Some(hwnd) } else { None }
    }

    #[cfg(windows)]
    /// Returns shareable window metadata for a window handle.
    ///
    /// # Safety
    /// `hwnd` must be a valid live window handle.
    pub unsafe fn get_share_window(&self, hwnd: ffi::HWND) -> Option<ffi::ShareWindow> {
        let mut window = ffi::ShareWindow::default();
        let ok = unsafe { ffi::api().TT_Windows_GetWindow(hwnd, &mut window) == 1 };
        if ok { Some(window) } else { None }
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

    #[cfg(windows)]
    /// Paints the current desktop frame for a user to a Win32 device context.
    ///
    /// # Safety
    /// `hdc` must be a valid device context for the full duration of the call.
    pub unsafe fn paint_desktop_window(
        &self,
        user_id: UserId,
        hdc: ffi::HDC,
        x_dest: i32,
        y_dest: i32,
        dest_width: i32,
        dest_height: i32,
    ) -> bool {
        unsafe {
            ffi::api().TT_PaintDesktopWindow(
                self.ptr.0,
                user_id.raw(),
                hdc,
                x_dest,
                y_dest,
                dest_width,
                dest_height,
            ) == 1
        }
    }

    #[cfg(windows)]
    /// Paints a cropped desktop frame for a user to a Win32 device context.
    ///
    /// # Safety
    /// `hdc` must be a valid device context for the full duration of the call.
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn paint_desktop_window_ex(
        &self,
        user_id: UserId,
        hdc: ffi::HDC,
        x_dest: i32,
        y_dest: i32,
        dest_width: i32,
        dest_height: i32,
        x_src: i32,
        y_src: i32,
        src_width: i32,
        src_height: i32,
    ) -> bool {
        unsafe {
            ffi::api().TT_PaintDesktopWindowEx(
                self.ptr.0,
                user_id.raw(),
                hdc,
                x_dest,
                y_dest,
                dest_width,
                dest_height,
                x_src,
                y_src,
                src_width,
                src_height,
            ) == 1
        }
    }

    /// Acquires a desktop window update bitmap.
    pub fn acquire_user_desktop_window(&self, user_id: UserId) -> Option<*mut ffi::DesktopWindow> {
        unsafe {
            let ptr = ffi::api().TT_AcquireUserDesktopWindow(self.ptr.0, user_id.raw());
            if ptr.is_null() { None } else { Some(ptr) }
        }
    }

    /// Acquires a desktop window update bitmap and releases it automatically on drop.
    pub fn acquire_user_desktop_window_guard(
        &self,
        user_id: UserId,
    ) -> Option<DesktopWindowGuard<'_>> {
        self.acquire_user_desktop_window(user_id)
            .map(|ptr| DesktopWindowGuard { client: self, ptr })
    }

    /// Acquires a desktop window update bitmap converted to a specific bitmap format.
    pub fn acquire_user_desktop_window_ex(
        &self,
        user_id: UserId,
        bitmap_format: ffi::BitmapFormat,
    ) -> Option<*mut ffi::DesktopWindow> {
        unsafe {
            let ptr =
                ffi::api().TT_AcquireUserDesktopWindowEx(self.ptr.0, user_id.raw(), bitmap_format);
            if ptr.is_null() { None } else { Some(ptr) }
        }
    }

    /// Acquires a desktop window update bitmap in a specific format and releases it on drop.
    pub fn acquire_user_desktop_window_guard_ex(
        &self,
        user_id: UserId,
        bitmap_format: ffi::BitmapFormat,
    ) -> Option<DesktopWindowGuard<'_>> {
        self.acquire_user_desktop_window_ex(user_id, bitmap_format)
            .map(|ptr| DesktopWindowGuard { client: self, ptr })
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

impl Drop for DesktopWindowGuard<'_> {
    fn drop(&mut self) {
        let _ = unsafe { self.client.release_user_desktop_window(self.ptr) };
    }
}
