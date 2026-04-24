//! Video-related configuration and frame metadata types.

use teamtalk_sys as ffi;

/// Video format configuration.
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub struct VideoFormat {
    /// Frame width in pixels.
    pub width: i32,
    /// Frame height in pixels.
    pub height: i32,
    /// Frame rate numerator.
    pub fps_numerator: i32,
    /// Frame rate denominator.
    pub fps_denominator: i32,
    /// Picture format FourCC identifier.
    pub fourcc: ffi::FourCC,
}

impl Default for VideoFormat {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            fps_numerator: 0,
            fps_denominator: 0,
            fourcc: ffi::FourCC::FOURCC_NONE,
        }
    }
}

impl From<ffi::VideoFormat> for VideoFormat {
    fn from(f: ffi::VideoFormat) -> Self {
        Self {
            width: f.nWidth,
            height: f.nHeight,
            fps_numerator: f.nFPS_Numerator,
            fps_denominator: f.nFPS_Denominator,
            fourcc: f.picFourCC,
        }
    }
}

impl VideoFormat {
    /// Creates a new video format.
    #[must_use]
    pub fn new(
        width: i32,
        height: i32,
        fps_numerator: i32,
        fps_denominator: i32,
        fourcc: ffi::FourCC,
    ) -> Self {
        Self {
            width,
            height,
            fps_numerator,
            fps_denominator,
            fourcc,
        }
    }
    /// Converts to the raw `TeamTalk` struct.
    #[must_use]
    pub fn to_ffi(&self) -> ffi::VideoFormat {
        ffi::VideoFormat {
            nWidth: self.width,
            nHeight: self.height,
            nFPS_Numerator: self.fps_numerator,
            nFPS_Denominator: self.fps_denominator,
            picFourCC: self.fourcc,
        }
    }
}

/// Video codec configuration.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default)]
pub struct VideoCodec {
    /// Target bitrate for the WebM VP8 encoder.
    pub bitrate: i32,
    /// Encode deadline hint for the WebM VP8 encoder.
    pub deadline: u32,
}

impl From<ffi::VideoCodec> for VideoCodec {
    fn from(c: ffi::VideoCodec) -> Self {
        Self {
            bitrate: unsafe {
                c.__bindgen_anon_1
                    .webm_vp8
                    .__bindgen_anon_1
                    .nRcTargetBitrate
            },
            deadline: unsafe { c.__bindgen_anon_1.webm_vp8.nEncodeDeadline },
        }
    }
}

impl VideoCodec {
    /// Creates a new video codec configuration.
    #[must_use]
    pub fn new(bitrate: i32, deadline: u32) -> Self {
        Self { bitrate, deadline }
    }
    /// Converts to the raw `TeamTalk` struct.
    #[must_use]
    pub fn to_ffi(&self) -> ffi::VideoCodec {
        let mut raw = ffi::VideoCodec {
            nCodec: ffi::Codec::WEBM_VP8_CODEC,
            ..Default::default()
        };
        raw.__bindgen_anon_1.webm_vp8.nEncodeDeadline = self.deadline;
        raw.__bindgen_anon_1
            .webm_vp8
            .__bindgen_anon_1
            .nRcTargetBitrate = self.bitrate;
        raw
    }
}

/// Video frame metadata.
#[non_exhaustive]
pub struct VideoFrame {
    /// Frame width in pixels.
    pub width: i32,
    /// Frame height in pixels.
    pub height: i32,
    /// Stream id associated with the frame.
    pub stream_id: i32,
    /// Whether the frame is a keyframe.
    pub key_frame: bool,
    /// Pointer to the decoded frame buffer.
    pub buf: *mut std::ffi::c_void,
    /// Length of the decoded frame buffer in bytes.
    pub buf_len: i32,
}

impl From<ffi::VideoFrame> for VideoFrame {
    fn from(f: ffi::VideoFrame) -> Self {
        Self {
            width: f.nWidth,
            height: f.nHeight,
            stream_id: f.nStreamID,
            key_frame: f.bKeyFrame != 0,
            buf: f.frameBuffer,
            buf_len: f.nFrameBufferSize,
        }
    }
}
