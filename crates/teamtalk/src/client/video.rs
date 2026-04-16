//! Video capture and transmission APIs.
use super::Client;
use crate::types::{UserId, VideoCodec, VideoFormat};
use crate::utils::ToTT;
use teamtalk_sys as ffi;

#[non_exhaustive]
pub struct VideoCaptureDevice {
    pub id: String,
    pub name: String,
    pub api: String,
    pub formats: Vec<VideoFormat>,
}

impl From<ffi::VideoCaptureDevice> for VideoCaptureDevice {
    fn from(d: ffi::VideoCaptureDevice) -> Self {
        let mut formats = Vec::new();
        for i in 0..(d.nVideoFormatsCount as usize).min(1024) {
            formats.push(VideoFormat::from(d.videoFormats[i]));
        }
        Self {
            id: crate::utils::strings::to_string(&d.szDeviceID),
            name: crate::utils::strings::to_string(&d.szDeviceName),
            api: crate::utils::strings::to_string(&d.szCaptureAPI),
            formats,
        }
    }
}

pub struct VideoFrameView<'a> {
    pub(crate) inner: &'a ffi::VideoFrame,
}

impl<'a> VideoFrameView<'a> {
    #[must_use]
    pub fn width(&self) -> i32 {
        self.inner.nWidth
    }

    #[must_use]
    pub fn height(&self) -> i32 {
        self.inner.nHeight
    }

    #[must_use]
    pub fn stream_id(&self) -> i32 {
        self.inner.nStreamID
    }

    #[must_use]
    pub fn key_frame(&self) -> bool {
        self.inner.bKeyFrame != 0
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

pub struct VideoFrameGuard<'a> {
    client: &'a Client,
    ptr: *mut ffi::VideoFrame,
}

impl VideoFrameGuard<'_> {
    #[must_use]
    pub fn frame(&self) -> &ffi::VideoFrame {
        unsafe { &*self.ptr }
    }

    #[must_use]
    pub fn view(&self) -> VideoFrameView<'_> {
        VideoFrameView {
            inner: unsafe { &*self.ptr },
        }
    }

    #[must_use]
    pub fn as_ptr(&self) -> *mut ffi::VideoFrame {
        self.ptr
    }
}

impl Client {
    pub fn get_video_capture_devices(&self) -> Vec<VideoCaptureDevice> {
        let mut count: i32 = 0;
        self.backend()
            .get_video_capture_devices(std::ptr::null_mut(), &raw mut count);
        if count <= 0 {
            return vec![];
        }
        let mut devices =
            vec![unsafe { std::mem::zeroed::<ffi::VideoCaptureDevice>() }; count as usize];
        if self
            .backend()
            .get_video_capture_devices(devices.as_mut_ptr(), &raw mut count)
            == 1
        {
            devices.into_iter().map(VideoCaptureDevice::from).collect()
        } else {
            vec![]
        }
    }

    pub fn init_video_capture_device(&self, device_id: &str, format: &VideoFormat) -> bool {
        let id = device_id.tt();
        let raw_fmt = format.to_ffi();
        self.backend()
            .init_video_capture_device(self.ptr.0, id.as_ptr(), &raw const raw_fmt)
    }

    pub fn close_video_capture_device(&self) -> bool {
        self.backend().close_video_capture_device(self.ptr.0)
    }

    pub fn start_video_transmission(&self, codec: &VideoCodec) -> bool {
        self.backend()
            .start_video_transmission(self.ptr.0, &codec.to_ffi())
    }

    pub fn stop_video_transmission(&self) -> bool {
        self.backend().stop_video_transmission(self.ptr.0)
    }

    pub fn init_video_capture_device_result(
        &self,
        device_id: &str,
        format: &VideoFormat,
    ) -> crate::events::Result<()> {
        self.bool_to_result(self.init_video_capture_device(device_id, format))
    }

    pub fn close_video_capture_device_result(&self) -> crate::events::Result<()> {
        self.bool_to_result(self.close_video_capture_device())
    }

    pub fn start_video_transmission_result(&self, codec: &VideoCodec) -> crate::events::Result<()> {
        self.bool_to_result(self.start_video_transmission(codec))
    }

    pub fn stop_video_transmission_result(&self) -> crate::events::Result<()> {
        self.bool_to_result(self.stop_video_transmission())
    }

    pub fn acquire_video_frame(&self, user_id: UserId) -> Option<*mut ffi::VideoFrame> {
        let ptr = self
            .backend()
            .acquire_video_frame(self.ptr.0, user_id.raw());
        if ptr.is_null() { None } else { Some(ptr) }
    }

    pub fn acquire_video_frame_guard(&self, user_id: UserId) -> Option<VideoFrameGuard<'_>> {
        self.acquire_video_frame(user_id)
            .map(|ptr| VideoFrameGuard { client: self, ptr })
    }

    /// # Safety
    /// - `frame` must be a pointer returned by `acquire_video_frame`.
    /// - The frame must not be released more than once.
    /// - The pointer must not be used after release.
    pub unsafe fn release_video_frame(&self, frame: *mut ffi::VideoFrame) -> bool {
        if frame.is_null() {
            return false;
        }
        self.backend().release_video_frame(self.ptr.0, frame)
    }

    #[cfg(windows)]
    /// # Safety
    /// - `hdc` must be a valid device context for the full duration of the call.
    /// - `frame` must be a valid pointer returned by the SDK and remain alive for the call.
    pub unsafe fn paint_video_frame(
        &self,
        hdc: ffi::HDC,
        x_dest: i32,
        y_dest: i32,
        dest_width: i32,
        dest_height: i32,
        frame: *mut ffi::VideoFrame,
    ) -> bool {
        unsafe {
            ffi::api().TT_PaintVideoFrame(hdc, x_dest, y_dest, dest_width, dest_height, frame) == 1
        }
    }

    #[cfg(windows)]
    /// # Safety
    /// - `hdc` must be a valid device context for the full duration of the call.
    /// - `frame` must be a valid pointer returned by the SDK and remain alive for the call.
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn paint_video_frame_ex(
        &self,
        hdc: ffi::HDC,
        x_dest: i32,
        y_dest: i32,
        dest_width: i32,
        dest_height: i32,
        x_src: i32,
        y_src: i32,
        src_width: i32,
        src_height: i32,
        frame: *mut ffi::VideoFrame,
    ) -> bool {
        unsafe {
            ffi::api().TT_PaintVideoFrameEx(
                hdc,
                x_dest,
                y_dest,
                dest_width,
                dest_height,
                x_src,
                y_src,
                src_width,
                src_height,
                frame,
            ) == 1
        }
    }
}

impl Drop for VideoFrameGuard<'_> {
    fn drop(&mut self) {
        let _ = unsafe { self.client.release_video_frame(self.ptr) };
    }
}
