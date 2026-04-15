//! Media file playback and streaming APIs.
use super::Client;
use crate::types::{PlaybackSessionId, UserId, VideoCodec};
use crate::utils::ToTT;
use teamtalk_sys as ffi;

/// Controls media file playback behavior.
#[non_exhaustive]
#[derive(Debug, Clone, Default)]
pub struct MediaFilePlayback {
    pub offset_ms: u32,
    pub paused: bool,
}

impl MediaFilePlayback {
    #[must_use]
    pub fn to_ffi(&self) -> ffi::MediaFilePlayback {
        ffi::MediaFilePlayback {
            uOffsetMSec: self.offset_ms,
            bPaused: i32::from(self.paused),
            ..Default::default()
        }
    }
}

/// Guard around a media video frame acquired from the SDK.
pub struct MediaVideoFrameGuard<'a> {
    client: &'a Client,
    ptr: *mut ffi::VideoFrame,
}

impl MediaVideoFrameGuard<'_> {
    #[must_use]
    pub fn frame(&self) -> &ffi::VideoFrame {
        unsafe { &*self.ptr }
    }

    #[must_use]
    pub fn as_ptr(&self) -> *mut ffi::VideoFrame {
        self.ptr
    }
}

impl Client {
    /// Queries media-file metadata before playback or streaming.
    pub fn get_media_file_info(&self, file_path: &str) -> Option<crate::types::MediaFileInfo> {
        let mut info = ffi::MediaFileInfo::default();
        let path = file_path.tt();
        unsafe {
            (ffi::api().TT_GetMediaFileInfo(path.as_ptr(), &raw mut info) == 1)
                .then(|| crate::types::MediaFileInfo::from(info))
        }
    }

    /// Returns a palette entry for 8-bit desktop bitmaps.
    pub fn get_palette_color(
        &self,
        bitmap_format: ffi::BitmapFormat,
        index: i32,
    ) -> Option<[u8; 3]> {
        let ptr = unsafe { ffi::api().TT_Palette_GetColorTable(bitmap_format, index) };
        if ptr.is_null() {
            return None;
        }
        let bytes = unsafe { std::slice::from_raw_parts(ptr, 3) };
        Some([bytes[0], bytes[1], bytes[2]])
    }

    /// Starts streaming a media file to the channel.
    pub fn start_streaming_media_file_to_channel(
        &self,
        file_path: &str,
        codec: Option<&VideoCodec>,
    ) -> bool {
        let codec_ptr = codec.map_or(std::ptr::null(), |c| &c.to_ffi());
        unsafe {
            ffi::api().TT_StartStreamingMediaFileToChannel(
                self.ptr.0,
                file_path.tt().as_ptr(),
                codec_ptr,
            ) == 1
        }
    }

    /// Starts streaming a media file with advanced playback options.
    pub fn start_streaming_media_file_to_channel_ex(
        &self,
        file_path: &str,
        playback: &MediaFilePlayback,
        codec: Option<&VideoCodec>,
    ) -> bool {
        let codec_ptr = codec.map_or(std::ptr::null(), |c| &c.to_ffi());
        unsafe {
            ffi::api().TT_StartStreamingMediaFileToChannelEx(
                self.ptr.0,
                file_path.tt().as_ptr(),
                &playback.to_ffi(),
                codec_ptr,
            ) == 1
        }
    }

    /// Updates the currently streaming media file info.
    pub fn update_streaming_media_file_to_channel(
        &self,
        playback: &MediaFilePlayback,
        codec: Option<&VideoCodec>,
    ) -> bool {
        let codec_ptr = codec.map_or(std::ptr::null(), |c| &c.to_ffi());
        unsafe {
            ffi::api().TT_UpdateStreamingMediaFileToChannel(
                self.ptr.0,
                &playback.to_ffi(),
                codec_ptr,
            ) == 1
        }
    }

    /// Stops media file streaming.
    pub fn stop_streaming_media_file_to_channel(&self) -> bool {
        unsafe { ffi::api().TT_StopStreamingMediaFileToChannel(self.ptr.0) == 1 }
    }

    /// Initializes local media playback.
    pub fn init_local_playback(
        &self,
        file_path: &str,
        playback: &MediaFilePlayback,
    ) -> PlaybackSessionId {
        unsafe {
            PlaybackSessionId(ffi::api().TT_InitLocalPlayback(
                self.ptr.0,
                file_path.tt().as_ptr(),
                &playback.to_ffi(),
            ))
        }
    }

    /// Updates local playback info.
    pub fn update_local_playback(
        &self,
        session_id: PlaybackSessionId,
        playback: &MediaFilePlayback,
    ) -> bool {
        unsafe {
            ffi::api().TT_UpdateLocalPlayback(self.ptr.0, session_id.raw(), &playback.to_ffi()) == 1
        }
    }

    /// Stops local playback.
    pub fn stop_local_playback(&self, session_id: PlaybackSessionId) -> bool {
        unsafe { ffi::api().TT_StopLocalPlayback(self.ptr.0, session_id.raw()) == 1 }
    }

    /// Acquires a media video frame for a user.
    pub fn acquire_user_media_video_frame(&self, user_id: UserId) -> Option<*mut ffi::VideoFrame> {
        unsafe {
            let ptr = ffi::api().TT_AcquireUserMediaVideoFrame(self.ptr.0, user_id.raw());
            if ptr.is_null() { None } else { Some(ptr) }
        }
    }

    /// Acquires a media video frame and releases it automatically on drop.
    pub fn acquire_user_media_video_frame_guard(
        &self,
        user_id: UserId,
    ) -> Option<MediaVideoFrameGuard<'_>> {
        self.acquire_user_media_video_frame(user_id)
            .map(|ptr| MediaVideoFrameGuard { client: self, ptr })
    }

    /// Releases a previously acquired media video frame.
    ///
    /// # Safety
    /// - `frame` must be a pointer returned by `acquire_user_media_video_frame`.
    /// - The frame must not be released more than once.
    /// - The pointer must not be used after release.
    pub unsafe fn release_user_media_video_frame(&self, frame: *mut ffi::VideoFrame) -> bool {
        if frame.is_null() {
            return false;
        }
        unsafe { ffi::api().TT_ReleaseUserMediaVideoFrame(self.ptr.0, frame) == 1 }
    }
}

impl Drop for MediaVideoFrameGuard<'_> {
    fn drop(&mut self) {
        let _ = unsafe { self.client.release_user_media_video_frame(self.ptr) };
    }
}
