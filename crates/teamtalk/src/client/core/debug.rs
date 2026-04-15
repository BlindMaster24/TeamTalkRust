use super::{Client, ffi};

impl Client {
    /// Sends a debug input tone to the SDK.
    pub fn dbg_set_input_tone(&self, stream_types: u32, freq: i32) -> bool {
        unsafe { ffi::api().TT_DBG_SetSoundInputTone(self.ptr.0, stream_types, freq) == 1 }
    }

    /// Writes a debug tone into an audio file.
    pub fn dbg_write_audio_file_tone(&self, file_path: &str, freq: i32) -> bool {
        let mut info = unsafe { std::mem::zeroed::<ffi::MediaFileInfo>() };
        let p = crate::utils::ToTT::tt(file_path);
        unsafe {
            std::ptr::copy_nonoverlapping(
                p.as_ptr(),
                info.szFileName.as_mut_ptr(),
                p.len().min(511),
            );
            ffi::api().TT_DBG_WriteAudioFileTone(&raw const info, freq) == 1
        }
    }

    /// Returns the SDK-reported size for a `TeamTalk` type.
    #[must_use]
    pub fn dbg_sizeof(n_type: ffi::TTType) -> i32 {
        unsafe { ffi::api().TT_DBG_SIZEOF(n_type) }
    }

    /// Returns a data pointer for a `TeamTalk` message.
    pub fn dbg_get_data_ptr(msg: &mut ffi::TTMessage) -> *mut std::ffi::c_void {
        unsafe { ffi::api().TT_DBG_GETDATAPTR(msg) }
    }
}
