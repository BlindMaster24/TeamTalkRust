use crate::events::{Error, Result};
pub use teamtalk_sys as ffi;

pub mod audio;
pub mod channels;
pub mod connection;
pub mod core;
pub mod desktop;
pub mod encryption;
pub mod files;
pub mod hotkeys;
pub mod media;
pub mod recording;
pub mod server;
pub mod system;
pub mod users;
pub mod video;

pub use connection::{ConnectParams, ReconnectConfig, ReconnectHandler};

pub struct Client {
    pub name: Option<String>,
    ptr: *mut ffi::TTInstance,
}

unsafe impl Send for Client {}

impl Client {
    pub fn new() -> Result<Self> {
        crate::init()?;
        let ptr = unsafe { ffi::api().TT_InitTeamTalkPoll() };
        if ptr.is_null() {
            Err(Error::InitFailed)
        } else {
            Ok(Self { name: None, ptr })
        }
    }

    #[allow(clippy::missing_safety_doc)]
    #[cfg(windows)]
    pub unsafe fn with_hwnd(hwnd: ffi::HWND, msg: u32) -> Result<Self> {
        crate::init()?;
        let ptr = unsafe { ffi::api().TT_InitTeamTalk(hwnd, msg) };
        if ptr.is_null() {
            Err(Error::InitFailed)
        } else {
            Ok(Self { name: None, ptr })
        }
    }

    #[allow(clippy::missing_safety_doc)]
    #[cfg(windows)]
    pub unsafe fn swap_hwnd(&self, hwnd: ffi::HWND) -> bool {
        unsafe { ffi::api().TT_SwapTeamTalkHWND(self.ptr, hwnd) == 1 }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn dbg_set_input_tone(&self, stream_types: u32, freq: i32) -> bool {
        unsafe { ffi::api().TT_DBG_SetSoundInputTone(self.ptr, stream_types, freq) == 1 }
    }

    pub fn dbg_write_audio_file_tone(&self, file_path: &str, freq: i32) -> bool {
        let mut info = unsafe { std::mem::zeroed::<ffi::MediaFileInfo>() };
        let p = crate::utils::ToTT::tt(file_path);
        unsafe {
            std::ptr::copy_nonoverlapping(
                p.as_ptr(),
                info.szFileName.as_mut_ptr(),
                p.len().min(511),
            );
            ffi::api().TT_DBG_WriteAudioFileTone(&info, freq) == 1
        }
    }

    pub fn dbg_sizeof(n_type: ffi::TTType) -> i32 {
        unsafe { ffi::api().TT_DBG_SIZEOF(n_type) }
    }

    pub fn dbg_get_data_ptr(msg: &mut ffi::TTMessage) -> *mut std::ffi::c_void {
        unsafe { ffi::api().TT_DBG_GETDATAPTR(msg) }
    }
}

pub struct Message(ffi::TTMessage);

impl Message {
    pub fn source(&self) -> i32 {
        self.0.nSource
    }

    pub fn text(&self) -> Option<crate::types::TextMessage> {
        unsafe {
            Some(crate::types::TextMessage::from(
                self.0.__bindgen_anon_1.textmessage,
            ))
        }
    }

    pub fn user(&self) -> Option<crate::types::User> {
        unsafe { Some(crate::types::User::from(self.0.__bindgen_anon_1.user)) }
    }

    pub fn account(&self) -> Option<crate::types::UserAccount> {
        unsafe {
            Some(crate::types::UserAccount::from(
                self.0.__bindgen_anon_1.useraccount,
            ))
        }
    }

    pub fn raw(&self) -> &ffi::TTMessage {
        &self.0
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        unsafe {
            ffi::api().TT_CloseTeamTalk(self.ptr);
        }
    }
}
