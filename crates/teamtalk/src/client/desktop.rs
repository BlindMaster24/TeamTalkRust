//! Desktop access and sharing APIs.
use super::Client;
use crate::types::{DesktopInput, UserId};
use teamtalk_sys as ffi;

pub struct DesktopWindowView<'a> {
    inner: &'a ffi::DesktopWindow,
}

impl<'a> DesktopWindowView<'a> {
    #[must_use]
    pub fn width(&self) -> i32 {
        self.inner.nWidth
    }

    #[must_use]
    pub fn height(&self) -> i32 {
        self.inner.nHeight
    }

    #[must_use]
    pub fn bitmap_format(&self) -> ffi::BitmapFormat {
        self.inner.bmpFormat
    }

    #[must_use]
    pub fn bytes_per_line(&self) -> i32 {
        self.inner.nBytesPerLine
    }

    #[must_use]
    pub fn session_id(&self) -> i32 {
        self.inner.nSessionID
    }

    #[must_use]
    pub fn protocol(&self) -> ffi::DesktopProtocol {
        self.inner.nProtocol
    }

    #[must_use]
    pub fn frame_buffer(&self) -> Option<&'a [u8]> {
        if self.inner.frameBuffer.is_null() || self.inner.nFrameBufferSize == 0 {
            None
        } else {
            Some(unsafe {
                std::slice::from_raw_parts(
                    self.inner.frameBuffer as *const u8,
                    self.inner.nFrameBufferSize as usize,
                )
            })
        }
    }
}

pub struct DesktopWindowGuard<'a> {
    client: &'a Client,
    ptr: *mut ffi::DesktopWindow,
}

impl DesktopWindowGuard<'_> {
    #[must_use]
    pub fn window(&self) -> &ffi::DesktopWindow {
        unsafe { &*self.ptr }
    }

    #[must_use]
    pub fn view(&self) -> DesktopWindowView<'_> {
        DesktopWindowView {
            inner: unsafe { &*self.ptr },
        }
    }

    #[must_use]
    pub fn as_ptr(&self) -> *mut ffi::DesktopWindow {
        self.ptr
    }
}

impl Client {
    pub fn close_desktop_window(&self) -> bool {
        self.backend().close_desktop_window(self.ptr.0)
    }

    pub fn send_desktop_cursor_position(&self, x: u16, y: u16) -> bool {
        self.backend()
            .send_desktop_cursor_position(self.ptr.0, x, y)
    }

    pub fn send_desktop_input(&self, user_id: UserId, input: &ffi::DesktopInput) -> bool {
        self.backend()
            .send_desktop_input(self.ptr.0, user_id.raw(), input, 1)
    }

    pub fn close_desktop_window_result(&self) -> crate::events::Result<()> {
        self.bool_to_result(self.close_desktop_window())
    }

    pub fn send_desktop_cursor_position_result(&self, x: u16, y: u16) -> crate::events::Result<()> {
        self.bool_to_result(self.send_desktop_cursor_position(x, y))
    }

    pub fn send_desktop_input_result(
        &self,
        user_id: UserId,
        input: &ffi::DesktopInput,
    ) -> crate::events::Result<()> {
        self.bool_to_result(self.send_desktop_input(user_id, input))
    }

    pub fn send_desktop_inputs(&self, user_id: UserId, inputs: &[DesktopInput]) -> bool {
        if inputs.is_empty() || inputs.len() > ffi::TT_DESKTOPINPUT_MAX as usize {
            return false;
        }
        let raw_inputs: Vec<_> = inputs.iter().map(DesktopInput::to_ffi).collect();
        self.backend().send_desktop_input(
            self.ptr.0,
            user_id.raw(),
            raw_inputs.as_ptr(),
            raw_inputs.len() as i32,
        )
    }

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
        let count = self.backend().desktop_input_key_translate(
            translate,
            raw_inputs.as_ptr(),
            translated.as_mut_ptr(),
            raw_inputs.len() as i32,
        );
        if count < 0 {
            return None;
        }
        translated.truncate(count as usize);
        Some(translated.into_iter().map(DesktopInput::from).collect())
    }

    pub fn execute_desktop_input(&self, inputs: &[DesktopInput]) -> i32 {
        if inputs.is_empty() || inputs.len() > ffi::TT_DESKTOPINPUT_MAX as usize {
            return -1;
        }
        let raw_inputs: Vec<_> = inputs.iter().map(DesktopInput::to_ffi).collect();
        self.backend()
            .execute_desktop_input(raw_inputs.as_ptr(), raw_inputs.len() as i32)
    }

    pub fn send_desktop_window(
        &self,
        window: &ffi::DesktopWindow,
        bitmap_format: ffi::BitmapFormat,
    ) -> i32 {
        self.backend()
            .send_desktop_window(self.ptr.0, window, bitmap_format)
    }

    #[cfg(windows)]
    pub fn get_desktop_active_hwnd(&self) -> ffi::HWND {
        unsafe { ffi::api().TT_Windows_GetDesktopActiveHWND() }
    }

    #[cfg(windows)]
    pub fn get_desktop_hwnd(&self) -> ffi::HWND {
        unsafe { ffi::api().TT_Windows_GetDesktopHWND() }
    }

    #[cfg(windows)]
    pub fn get_desktop_window_hwnd(&self, index: i32) -> Option<ffi::HWND> {
        let mut hwnd = std::ptr::null_mut();
        let ok = unsafe { ffi::api().TT_Windows_GetDesktopWindowHWND(index, &raw mut hwnd) == 1 };
        if ok { Some(hwnd) } else { None }
    }

    #[cfg(windows)]
    /// # Safety
    /// `hwnd` must be a valid live window handle.
    pub unsafe fn get_share_window(&self, hwnd: ffi::HWND) -> Option<ffi::ShareWindow> {
        let mut window = ffi::ShareWindow::default();
        let ok = unsafe { ffi::api().TT_Windows_GetWindow(hwnd, &raw mut window) == 1 };
        if ok { Some(window) } else { None }
    }

    #[cfg(windows)]
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

    pub fn acquire_user_desktop_window(&self, user_id: UserId) -> Option<*mut ffi::DesktopWindow> {
        let ptr = self
            .backend()
            .acquire_user_desktop_window(self.ptr.0, user_id.raw());
        if ptr.is_null() { None } else { Some(ptr) }
    }

    pub fn acquire_user_desktop_window_guard(
        &self,
        user_id: UserId,
    ) -> Option<DesktopWindowGuard<'_>> {
        self.acquire_user_desktop_window(user_id)
            .map(|ptr| DesktopWindowGuard { client: self, ptr })
    }

    pub fn acquire_user_desktop_window_ex(
        &self,
        user_id: UserId,
        bitmap_format: ffi::BitmapFormat,
    ) -> Option<*mut ffi::DesktopWindow> {
        let ptr =
            self.backend()
                .acquire_user_desktop_window_ex(self.ptr.0, user_id.raw(), bitmap_format);
        if ptr.is_null() { None } else { Some(ptr) }
    }

    pub fn acquire_user_desktop_window_guard_ex(
        &self,
        user_id: UserId,
        bitmap_format: ffi::BitmapFormat,
    ) -> Option<DesktopWindowGuard<'_>> {
        self.acquire_user_desktop_window_ex(user_id, bitmap_format)
            .map(|ptr| DesktopWindowGuard { client: self, ptr })
    }

    #[allow(clippy::missing_safety_doc)]
    /// # Safety
    /// `window` must be a valid pointer returned by `acquire_user_desktop_window`.
    pub unsafe fn release_user_desktop_window(&self, window: *mut ffi::DesktopWindow) -> bool {
        if window.is_null() {
            return false;
        }
        self.backend()
            .release_user_desktop_window(self.ptr.0, window)
    }
}

impl Drop for DesktopWindowGuard<'_> {
    fn drop(&mut self) {
        let _ = unsafe { self.client.release_user_desktop_window(self.ptr) };
    }
}
