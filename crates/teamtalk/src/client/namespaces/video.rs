//! Video capture and transmission namespace.
use super::define_namespace;
use crate::client::video::VideoCaptureDevice;
use crate::types::{UserId, VideoCodec, VideoFormat};
use teamtalk_sys as ffi;

define_namespace!(VideoNamespace);

impl VideoNamespace {
    /// Returns available video capture devices.
    pub fn devices(&self) -> Vec<VideoCaptureDevice> {
        self.client.get_video_capture_devices()
    }

    /// Initializes a video capture device.
    pub fn init(&self, device_id: &str, format: &VideoFormat) -> bool {
        self.client.init_video_capture_device(device_id, format)
    }

    /// Closes the active video capture device.
    pub fn close(&self) -> bool {
        self.client.close_video_capture_device()
    }

    /// Starts video transmission.
    pub fn start_transmission(&self, codec: &VideoCodec) -> bool {
        self.client.start_video_transmission(codec)
    }

    /// Stops video transmission.
    pub fn stop_transmission(&self) -> bool {
        self.client.stop_video_transmission()
    }

    /// Acquires the latest video frame for a user.
    pub fn acquire_frame(&self, user_id: UserId) -> Option<*mut ffi::VideoFrame> {
        self.client.acquire_video_frame(user_id)
    }

    /// Releases a previously acquired video frame.
    ///
    /// # Safety
    /// `frame` must be a valid pointer returned by `acquire_frame`.
    pub unsafe fn release_frame(&self, frame: *mut ffi::VideoFrame) -> bool {
        unsafe { self.client.release_video_frame(frame) }
    }
}

#[cfg(feature = "async")]
use super::define_async_namespace;

#[cfg(feature = "async")]
define_async_namespace!(AsyncVideoNamespace);

#[cfg(feature = "async")]
impl AsyncVideoNamespace {
    // TODO: Implement proper async commands with success confirmation where applicable

    /// Returns available video capture devices.
    pub fn devices(&self) -> Vec<VideoCaptureDevice> {
        self.client.get_video_capture_devices()
    }

    /// Starts video transmission.
    pub fn start_transmission(&self, codec: &VideoCodec) -> bool {
        self.client.start_video_transmission(codec)
    }

    /// Stops video transmission.
    pub fn stop_transmission(&self) -> bool {
        self.client.stop_video_transmission()
    }
}
