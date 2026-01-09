//! Media file playback and streaming APIs.
use super::Client;
use crate::types::{AudioPreprocessor, MediaFileInfo, UserId, VideoCodec};
use crate::utils::ToTT;
use teamtalk_sys as ffi;

/// Playback options for media streaming and local playback.
#[derive(Debug, Clone, Default)]
pub struct MediaPlayback {
    pub offset_ms: u32,
    pub paused: bool,
    pub preprocessor: AudioPreprocessor,
}

impl MediaPlayback {
    /// Converts to the raw TeamTalk playback struct.
    pub fn to_ffi(&self) -> ffi::MediaFilePlayback {
        ffi::MediaFilePlayback {
            uOffsetMSec: self.offset_ms,
            bPaused: self.paused as i32,
            audioPreprocessor: self.preprocessor.to_ffi(),
        }
    }
}

impl Client {
    /// Starts streaming a media file to the current channel.
    pub fn start_streaming(&self, file_path: &str, codec: Option<&VideoCodec>) -> bool {
        let raw_codec = codec.map(|c| c.to_ffi());
        let codec_ptr = raw_codec.as_ref().map_or(std::ptr::null(), |c| c);
        unsafe {
            ffi::api().TT_StartStreamingMediaFileToChannel(
                self.ptr,
                file_path.tt().as_ptr(),
                codec_ptr,
            ) == 1
        }
    }

    /// Starts streaming with playback options.
    pub fn start_streaming_ex(
        &self,
        file_path: &str,
        playback: &MediaPlayback,
        codec: Option<&VideoCodec>,
    ) -> bool {
        let raw_codec = codec.map(|c| c.to_ffi());
        let codec_ptr = raw_codec.as_ref().map_or(std::ptr::null(), |c| c);
        unsafe {
            ffi::api().TT_StartStreamingMediaFileToChannelEx(
                self.ptr,
                file_path.tt().as_ptr(),
                &playback.to_ffi(),
                codec_ptr,
            ) == 1
        }
    }

    /// Updates streaming playback and codec settings.
    pub fn update_streaming(&self, playback: &MediaPlayback, codec: Option<&VideoCodec>) -> bool {
        let raw_codec = codec.map(|c| c.to_ffi());
        let codec_ptr = raw_codec.as_ref().map_or(std::ptr::null(), |c| c);
        unsafe {
            ffi::api().TT_UpdateStreamingMediaFileToChannel(self.ptr, &playback.to_ffi(), codec_ptr)
                == 1
        }
    }

    /// Stops media streaming.
    pub fn stop_streaming(&self) -> bool {
        unsafe { ffi::api().TT_StopStreamingMediaFileToChannel(self.ptr) == 1 }
    }

    /// Initializes local playback of a media file.
    pub fn init_local_playback(&self, file_path: &str, playback: &MediaPlayback) -> i32 {
        unsafe {
            ffi::api().TT_InitLocalPlayback(self.ptr, file_path.tt().as_ptr(), &playback.to_ffi())
        }
    }

    /// Updates local playback parameters.
    pub fn update_local_playback(&self, session_id: i32, playback: &MediaPlayback) -> bool {
        unsafe { ffi::api().TT_UpdateLocalPlayback(self.ptr, session_id, &playback.to_ffi()) == 1 }
    }

    /// Stops local playback.
    pub fn stop_local_playback(&self, session_id: i32) -> bool {
        unsafe { ffi::api().TT_StopLocalPlayback(self.ptr, session_id) == 1 }
    }

    /// Returns media file metadata.
    pub fn get_media_file_info(file_path: &str) -> Option<MediaFileInfo> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::MediaFileInfo>() };
        if unsafe { ffi::api().TT_GetMediaFileInfo(file_path.tt().as_ptr(), &mut raw) } == 1 {
            Some(MediaFileInfo::from(raw))
        } else {
            None
        }
    }

    /// Acquires a media video frame for a user.
    pub fn acquire_media_video_frame(&self, user_id: UserId) -> Option<*mut ffi::VideoFrame> {
        unsafe {
            let ptr = ffi::api().TT_AcquireUserMediaVideoFrame(self.ptr, user_id.0);
            if ptr.is_null() { None } else { Some(ptr) }
        }
    }

    #[allow(clippy::missing_safety_doc)]
    /// Releases a previously acquired media video frame.
    ///
    /// # Safety
    /// `frame` must be a valid pointer returned by `acquire_media_video_frame`.
    pub unsafe fn release_media_video_frame(&self, frame: *mut ffi::VideoFrame) -> bool {
        if frame.is_null() {
            return false;
        }
        unsafe { ffi::api().TT_ReleaseUserMediaVideoFrame(self.ptr, frame) == 1 }
    }
}
