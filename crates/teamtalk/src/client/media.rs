//! Media file playback and streaming APIs.
use super::Client;
use crate::types::{PlaybackSessionId, UserId, VideoCodec};
use crate::utils::ToTT;
use teamtalk_sys as ffi;

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
    pub fn view(&self) -> super::video::VideoFrameView<'_> {
        super::video::VideoFrameView {
            inner: unsafe { &*self.ptr },
        }
    }

    #[must_use]
    pub fn as_ptr(&self) -> *mut ffi::VideoFrame {
        self.ptr
    }
}

impl Client {
    pub fn get_media_file_info(&self, file_path: &str) -> Option<crate::types::MediaFileInfo> {
        let mut info = ffi::MediaFileInfo::default();
        let path = file_path.tt();
        if self
            .backend()
            .get_media_file_info(path.as_ptr(), &raw mut info)
        {
            Some(crate::types::MediaFileInfo::from(info))
        } else {
            None
        }
    }

    pub fn get_palette_color(
        &self,
        bitmap_format: ffi::BitmapFormat,
        index: i32,
    ) -> Option<[u8; 3]> {
        let ptr = self.backend().get_palette_color(bitmap_format, index);
        if ptr.is_null() {
            return None;
        }
        let bytes = unsafe { std::slice::from_raw_parts(ptr, 3) };
        Some([bytes[0], bytes[1], bytes[2]])
    }

    pub fn start_streaming_media_file_to_channel(
        &self,
        file_path: &str,
        codec: Option<&VideoCodec>,
    ) -> bool {
        let codec_ptr = codec.map_or(std::ptr::null(), |c| &c.to_ffi());
        let path = file_path.tt();
        self.backend()
            .start_streaming_media_file_to_channel(self.ptr.0, path.as_ptr(), codec_ptr)
    }

    pub fn start_streaming_media_file_to_channel_ex(
        &self,
        file_path: &str,
        playback: &MediaFilePlayback,
        codec: Option<&VideoCodec>,
    ) -> bool {
        let codec_ptr = codec.map_or(std::ptr::null(), |c| &c.to_ffi());
        let path = file_path.tt();
        self.backend().start_streaming_media_file_to_channel_ex(
            self.ptr.0,
            path.as_ptr(),
            &playback.to_ffi(),
            codec_ptr,
        )
    }

    pub fn update_streaming_media_file_to_channel(
        &self,
        playback: &MediaFilePlayback,
        codec: Option<&VideoCodec>,
    ) -> bool {
        let codec_ptr = codec.map_or(std::ptr::null(), |c| &c.to_ffi());
        self.backend().update_streaming_media_file_to_channel(
            self.ptr.0,
            &playback.to_ffi(),
            codec_ptr,
        )
    }

    pub fn stop_streaming_media_file_to_channel(&self) -> bool {
        self.backend()
            .stop_streaming_media_file_to_channel(self.ptr.0)
    }

    pub fn init_local_playback(
        &self,
        file_path: &str,
        playback: &MediaFilePlayback,
    ) -> PlaybackSessionId {
        let path = file_path.tt();
        PlaybackSessionId(self.backend().init_local_playback(
            self.ptr.0,
            path.as_ptr(),
            &playback.to_ffi(),
        ))
    }

    pub fn update_local_playback(
        &self,
        session_id: PlaybackSessionId,
        playback: &MediaFilePlayback,
    ) -> bool {
        self.backend()
            .update_local_playback(self.ptr.0, session_id.raw(), &playback.to_ffi())
    }

    pub fn update_local_playback_result(
        &self,
        session_id: PlaybackSessionId,
        playback: &MediaFilePlayback,
    ) -> crate::events::Result<()> {
        self.bool_to_result(self.update_local_playback(session_id, playback))
    }

    pub fn stop_local_playback(&self, session_id: PlaybackSessionId) -> bool {
        self.backend()
            .stop_local_playback(self.ptr.0, session_id.raw())
    }

    pub fn start_streaming_media_file_to_channel_result(
        &self,
        file_path: &str,
        codec: Option<&VideoCodec>,
    ) -> crate::events::Result<()> {
        self.bool_to_result(self.start_streaming_media_file_to_channel(file_path, codec))
    }

    pub fn stop_streaming_media_file_to_channel_result(&self) -> crate::events::Result<()> {
        self.bool_to_result(self.stop_streaming_media_file_to_channel())
    }

    pub fn stop_local_playback_result(
        &self,
        session_id: PlaybackSessionId,
    ) -> crate::events::Result<()> {
        self.bool_to_result(self.stop_local_playback(session_id))
    }

    pub fn acquire_user_media_video_frame(&self, user_id: UserId) -> Option<*mut ffi::VideoFrame> {
        let ptr = self
            .backend()
            .acquire_user_media_video_frame(self.ptr.0, user_id.raw());
        if ptr.is_null() { None } else { Some(ptr) }
    }

    pub fn acquire_user_media_video_frame_guard(
        &self,
        user_id: UserId,
    ) -> Option<MediaVideoFrameGuard<'_>> {
        self.acquire_user_media_video_frame(user_id)
            .map(|ptr| MediaVideoFrameGuard { client: self, ptr })
    }

    /// # Safety
    /// - `frame` must be a pointer returned by `acquire_user_media_video_frame`.
    /// - The frame must not be released more than once.
    /// - The pointer must not be used after release.
    pub unsafe fn release_user_media_video_frame(&self, frame: *mut ffi::VideoFrame) -> bool {
        if frame.is_null() {
            return false;
        }
        self.backend()
            .release_user_media_video_frame(self.ptr.0, frame)
    }
}

impl Drop for MediaVideoFrameGuard<'_> {
    fn drop(&mut self) {
        let _ = unsafe { self.client.release_user_media_video_frame(self.ptr) };
    }
}
