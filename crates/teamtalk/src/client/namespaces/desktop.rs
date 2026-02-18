//! Desktop access and sharing namespace.
use super::define_namespace;
use crate::types::UserId;
use teamtalk_sys as ffi;

define_namespace!(DesktopNamespace);

impl DesktopNamespace {
    /// Closes a desktop window session.
    pub fn close(&self) -> bool {
        self.client.close_desktop_window()
    }

    /// Sends mouse cursor position to the desktop sharer.
    pub fn send_cursor(&self, x: u16, y: u16) -> bool {
        self.client.send_desktop_cursor_position(x, y)
    }

    /// Sends keyboard or mouse input to the desktop sharer.
    pub fn send_input(&self, user_id: UserId, input: &ffi::DesktopInput) -> bool {
        self.client.send_desktop_input(user_id, input)
    }

    /// Acquires a desktop window update bitmap.
    pub fn acquire_window(&self, user_id: UserId) -> Option<*mut ffi::DesktopWindow> {
        self.client.acquire_user_desktop_window(user_id)
    }

    /// Releases a previously acquired desktop window.
    ///
    /// # Safety
    /// `window` must be a valid pointer returned by `acquire_window`.
    pub unsafe fn release_window(&self, window: *mut ffi::DesktopWindow) -> bool {
        unsafe { self.client.release_user_desktop_window(window) }
    }
}

#[cfg(feature = "async")]
use super::define_async_namespace;

#[cfg(feature = "async")]
define_async_namespace!(AsyncDesktopNamespace);

#[cfg(feature = "async")]
impl AsyncDesktopNamespace {
    // TODO: Implement proper async commands with success confirmation where applicable

    /// Closes a desktop window session.
    pub fn close(&self) -> bool {
        self.client.close_desktop_window()
    }

    /// Sends mouse cursor position.
    pub fn send_cursor(&self, x: u16, y: u16) -> bool {
        self.client.send_desktop_cursor_position(x, y)
    }
}
