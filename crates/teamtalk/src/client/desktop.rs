//! Desktop sharing APIs.
use super::Client;
use crate::types::UserId;
use teamtalk_sys as ffi;

/// Metadata about a shareable desktop window.
pub struct ShareWindow {
    #[cfg(windows)]
    pub hwnd: ffi::HWND,
    #[cfg(not(windows))]
    pub window_id: i64,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub title: String,
}

#[cfg(windows)]
impl From<ffi::ShareWindow> for ShareWindow {
    fn from(w: ffi::ShareWindow) -> Self {
        Self {
            hwnd: w.hWnd,
            x: w.nWndX,
            y: w.nWndY,
            width: w.nWidth,
            height: w.nHeight,
            title: crate::utils::strings::to_string(&w.szWindowTitle),
        }
    }
}

impl Client {
    /// Closes the desktop sharing session.
    pub fn close_desktop_window(&self) -> bool {
        unsafe { ffi::api().TT_CloseDesktopWindow(self.ptr) == 1 }
    }

    /// Sends a desktop cursor position update.
    pub fn send_desktop_cursor_position(&self, x: u16, y: u16) -> bool {
        unsafe { ffi::api().TT_SendDesktopCursorPosition(self.ptr, x, y) == 1 }
    }

    /// Sends desktop input to a user.
    pub fn send_desktop_input(&self, user_id: UserId, inputs: &[ffi::DesktopInput]) -> bool {
        unsafe {
            ffi::api().TT_SendDesktopInput(
                self.ptr,
                user_id.0,
                inputs.as_ptr(),
                inputs.len() as i32,
            ) == 1
        }
    }

    /// Executes desktop input locally.
    pub fn execute_desktop_input(&self, inputs: &[ffi::DesktopInput]) -> i32 {
        unsafe { ffi::api().TT_DesktopInput_Execute(inputs.as_ptr(), inputs.len() as i32) }
    }

    /// Translates desktop inputs for a target layout.
    pub fn key_translate(
        &self,
        translate: ffi::TTKeyTranslate,
        inputs: &[ffi::DesktopInput],
    ) -> Vec<ffi::DesktopInput> {
        let mut translated = vec![unsafe { std::mem::zeroed::<ffi::DesktopInput>() }; inputs.len()];
        unsafe {
            let count = ffi::api().TT_DesktopInput_KeyTranslate(
                translate,
                inputs.as_ptr(),
                translated.as_mut_ptr(),
                inputs.len() as i32,
            );
            translated.truncate(count as usize);
        }
        translated
    }

    /// Returns a color table for a bitmap format.
    pub fn get_color_table(&self, format: ffi::BitmapFormat, index: i32) -> *mut u8 {
        unsafe { ffi::api().TT_Palette_GetColorTable(format, index) }
    }

    #[cfg(windows)]
    /// Returns shareable desktop windows (Windows).
    pub fn get_desktop_windows(&self) -> Vec<ShareWindow> {
        let mut i = 0;
        let mut windows = Vec::new();
        loop {
            let mut hwnd = std::ptr::null_mut();
            if unsafe { ffi::api().TT_Windows_GetDesktopWindowHWND(i, &mut hwnd) } == 1 {
                let mut raw = unsafe { std::mem::zeroed::<ffi::ShareWindow>() };
                if unsafe { ffi::api().TT_Windows_GetWindow(hwnd, &mut raw) } == 1 {
                    windows.push(ShareWindow::from(raw));
                }
                i += 1;
            } else {
                break;
            }
        }
        windows
    }

    #[cfg(windows)]
    /// Returns the active desktop window (Windows).
    pub fn get_active_window(&self) -> Option<ShareWindow> {
        unsafe {
            let hwnd = ffi::api().TT_Windows_GetDesktopActiveHWND();
            if hwnd.is_null() {
                None
            } else {
                let mut raw = std::mem::zeroed::<ffi::ShareWindow>();
                if ffi::api().TT_Windows_GetWindow(hwnd, &mut raw) == 1 {
                    Some(ShareWindow::from(raw))
                } else {
                    None
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    /// Returns shareable desktop windows (macOS).
    pub fn get_desktop_windows(&self) -> Vec<ShareWindow> {
        let mut i = 0;
        let mut windows = Vec::new();
        loop {
            let mut raw = unsafe { std::mem::zeroed::<ffi::ShareWindow>() };
            if unsafe { ffi::api().TT_MacOS_GetWindow(i, &mut raw) } == 1 {
                windows.push(ShareWindow {
                    window_id: raw.nWindowID,
                    x: raw.nWindowX,
                    y: raw.nWindowY,
                    width: raw.nWidth,
                    height: raw.nHeight,
                    title: crate::utils::strings::to_string(&raw.szWindowTitle),
                });
                i += 1;
            } else {
                break;
            }
        }
        windows
    }

    #[cfg(target_os = "macos")]
    /// Starts desktop sharing from a window id (macOS).
    pub fn send_window(&self, window_id: i64, format: ffi::BitmapFormat) -> i32 {
        unsafe {
            ffi::api().TT_SendDesktopFromWindowID(
                self.ptr,
                window_id,
                format,
                ffi::DesktopProtocol::DESKTOPPROTOCOL_ZLIB_1,
            )
        }
    }

    #[allow(clippy::missing_safety_doc)]
    #[cfg(windows)]
    /// Starts desktop sharing from a window handle (Windows).
    ///
    /// # Safety
    /// `hwnd` must be a valid window handle.
    pub unsafe fn send_window(&self, hwnd: ffi::HWND, format: ffi::BitmapFormat) -> i32 {
        unsafe {
            ffi::api().TT_SendDesktopWindowFromHWND(
                self.ptr,
                hwnd,
                format,
                ffi::DesktopProtocol::DESKTOPPROTOCOL_ZLIB_1,
            )
        }
    }
}
