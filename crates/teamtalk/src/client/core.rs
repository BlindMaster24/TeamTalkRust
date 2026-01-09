//! Core polling and client state utilities.
use super::{Client, Message};
use crate::events::Event;
use crate::types::{ClientFlags, TT_STRLEN};
use crate::utils::strings::tt_buf;
use teamtalk_sys as ffi;

impl Client {
    /// Returns the raw TeamTalk instance pointer.
    pub fn raw_ptr(&self) -> *mut ffi::TTInstance {
        self.ptr
    }

    /// Returns the SDK version string.
    pub fn version() -> String {
        let _ = crate::init();
        unsafe {
            let ptr = ffi::api().TT_GetVersion();
            if ptr.is_null() {
                "Unknown".to_string()
            } else {
                crate::utils::strings::from_tt(ptr)
            }
        }
    }

    /// Polls the client for the next event.
    pub fn poll(&self, timeout_ms: i32) -> Option<(Event, Message)> {
        let mut msg = unsafe { std::mem::zeroed::<ffi::TTMessage>() };
        let t = timeout_ms;
        if unsafe { ffi::api().TT_GetMessage(self.ptr, &mut msg, &t) } == 1 {
            Some((Event::from(msg.nClientEvent), Message::from_raw(msg)))
        } else {
            None
        }
    }

    /// Returns the current client flags.
    pub fn get_flags(&self) -> ClientFlags {
        ClientFlags::from_raw(unsafe { ffi::api().TT_GetFlags(self.ptr) })
    }

    /// Returns a human-readable error message for a TeamTalk error code.
    pub fn get_error_message(&self, code: i32) -> String {
        let mut buf = tt_buf::<TT_STRLEN>();
        unsafe {
            ffi::api().TT_GetErrorMessage(code, buf.as_mut_ptr());
            crate::utils::strings::to_string(&buf)
        }
    }
}
