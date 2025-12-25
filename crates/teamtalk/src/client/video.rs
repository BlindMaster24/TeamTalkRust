use super::Client;
use crate::types::{UserId, VideoCodec, VideoFormat};
use teamtalk_sys as ffi;

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

impl Client {
    pub fn get_video_capture_devices(&self) -> Vec<VideoCaptureDevice> {
        let mut count: i32 = 0;
        unsafe {
            ffi::api().TT_GetVideoCaptureDevices(std::ptr::null_mut(), &mut count);
            let mut devices = vec![std::mem::zeroed::<ffi::VideoCaptureDevice>(); count as usize];
            if ffi::api().TT_GetVideoCaptureDevices(devices.as_mut_ptr(), &mut count) == 1 {
                devices.into_iter().map(VideoCaptureDevice::from).collect()
            } else {
                vec![]
            }
        }
    }

    pub fn init_video_capture_device(&self, device_id: &str, format: &VideoFormat) -> bool {
        let id = crate::utils::ToTT::tt(device_id);
        let raw_fmt = format.to_ffi();
        unsafe { ffi::api().TT_InitVideoCaptureDevice(self.ptr, id.as_ptr(), &raw_fmt) == 1 }
    }

    pub fn close_video_capture_device(&self) -> bool {
        unsafe { ffi::api().TT_CloseVideoCaptureDevice(self.ptr) == 1 }
    }

    pub fn start_video_transmission(&self, codec: &VideoCodec) -> bool {
        unsafe { ffi::api().TT_StartVideoCaptureTransmission(self.ptr, &codec.to_ffi()) == 1 }
    }

    pub fn stop_video_transmission(&self) -> bool {
        unsafe { ffi::api().TT_StopVideoCaptureTransmission(self.ptr) == 1 }
    }

    pub fn acquire_video_frame(&self, user_id: UserId) -> Option<*mut ffi::VideoFrame> {
        unsafe {
            let ptr = ffi::api().TT_AcquireUserVideoCaptureFrame(self.ptr, user_id.0);
            if ptr.is_null() { None } else { Some(ptr) }
        }
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn release_video_frame(&self, frame: *mut ffi::VideoFrame) -> bool {
        if frame.is_null() {
            return false;
        }
        unsafe { ffi::api().TT_ReleaseUserVideoCaptureFrame(self.ptr, frame) == 1 }
    }
}
