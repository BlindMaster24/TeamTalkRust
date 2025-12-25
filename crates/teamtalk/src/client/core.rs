use super::{Client, Message};
use crate::events::Event;
use crate::types::{ClientFlags, TT_STRLEN};
use crate::utils::strings::tt_buf;
use teamtalk_sys as ffi;

impl Client {
    pub fn raw_ptr(&self) -> *mut ffi::TTInstance {
        self.ptr
    }

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

    pub fn poll(&self, timeout_ms: i32) -> Option<(Event, Message)> {
        let mut msg = unsafe { std::mem::zeroed::<ffi::TTMessage>() };
        let t = timeout_ms;
        if unsafe { ffi::api().TT_GetMessage(self.ptr, &mut msg, &t) } == 1 {
            Some((Event::from(msg.nClientEvent), Message(msg)))
        } else {
            None
        }
    }

    pub fn get_flags(&self) -> ClientFlags {
        ClientFlags::from_raw(unsafe { ffi::api().TT_GetFlags(self.ptr) })
    }

    pub fn get_error_message(&self, code: i32) -> String {
        let mut buf = tt_buf::<TT_STRLEN>();
        unsafe {
            ffi::api().TT_GetErrorMessage(code, buf.as_mut_ptr());
            crate::utils::strings::to_string(&buf)
        }
    }
}
