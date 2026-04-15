use teamtalk_sys as ffi;

/// Jitter control configuration.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default)]
pub struct JitterConfig {
    pub fixed_delay_ms: i32,
    pub use_adaptive: bool,
    pub max_adaptive_delay_ms: i32,
    pub active_adaptive_delay_ms: i32,
}

impl From<ffi::JitterConfig> for JitterConfig {
    fn from(c: ffi::JitterConfig) -> Self {
        Self {
            fixed_delay_ms: c.nFixedDelayMSec,
            use_adaptive: c.bUseAdativeDejitter != 0,
            max_adaptive_delay_ms: c.nMaxAdaptiveDelayMSec,
            active_adaptive_delay_ms: c.nActiveAdaptiveDelayMSec,
        }
    }
}

impl JitterConfig {
    /// Creates a new jitter configuration.
    pub fn new(
        fixed_delay_ms: i32,
        use_adaptive: bool,
        max_adaptive_delay_ms: i32,
        active_adaptive_delay_ms: i32,
    ) -> Self {
        Self {
            fixed_delay_ms,
            use_adaptive,
            max_adaptive_delay_ms,
            active_adaptive_delay_ms,
        }
    }
    /// Converts to the raw TeamTalk struct.
    pub fn to_ffi(&self) -> ffi::JitterConfig {
        ffi::JitterConfig {
            nFixedDelayMSec: self.fixed_delay_ms,
            bUseAdativeDejitter: self.use_adaptive as i32,
            nMaxAdaptiveDelayMSec: self.max_adaptive_delay_ms,
            nActiveAdaptiveDelayMSec: self.active_adaptive_delay_ms,
        }
    }
}

/// Video format configuration.
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub struct VideoFormat {
    pub width: i32,
    pub height: i32,
    pub fps_numerator: i32,
    pub fps_denominator: i32,
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
    /// Converts to the raw TeamTalk struct.
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
    pub bitrate: i32,
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
    pub fn new(bitrate: i32, deadline: u32) -> Self {
        Self { bitrate, deadline }
    }
    /// Converts to the raw TeamTalk struct.
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

/// TLS encryption context settings.
#[non_exhaustive]
#[derive(Debug, Clone, Default)]
pub struct EncryptionContext {
    pub cert_file: String,
    pub key_file: String,
    pub ca_file: String,
    pub ca_dir: String,
    pub verify_peer: bool,
    pub verify_client_once: bool,
    pub verify_depth: i32,
}

impl EncryptionContext {
    /// Creates a new encryption context.
    pub fn new(
        cert_file: impl Into<String>,
        key_file: impl Into<String>,
        ca_file: impl Into<String>,
        ca_dir: impl Into<String>,
        verify_peer: bool,
        verify_client_once: bool,
        verify_depth: i32,
    ) -> Self {
        Self {
            cert_file: cert_file.into(),
            key_file: key_file.into(),
            ca_file: ca_file.into(),
            ca_dir: ca_dir.into(),
            verify_peer,
            verify_client_once,
            verify_depth,
        }
    }
    /// Converts to the raw TeamTalk struct.
    pub fn to_ffi(&self) -> ffi::EncryptionContext {
        let mut raw = ffi::EncryptionContext::default();
        let cert = crate::utils::ToTT::tt(&self.cert_file);
        let key = crate::utils::ToTT::tt(&self.key_file);
        let ca = crate::utils::ToTT::tt(&self.ca_file);
        let cadir = crate::utils::ToTT::tt(&self.ca_dir);
        unsafe {
            let n_len = cert.len().min(511);
            std::ptr::copy_nonoverlapping(cert.as_ptr(), raw.szCertificateFile.as_mut_ptr(), n_len);
            let k_len = key.len().min(511);
            std::ptr::copy_nonoverlapping(key.as_ptr(), raw.szPrivateKeyFile.as_mut_ptr(), k_len);
            let ca_len = ca.len().min(511);
            std::ptr::copy_nonoverlapping(ca.as_ptr(), raw.szCAFile.as_mut_ptr(), ca_len);
            let cd_len = cadir.len().min(511);
            std::ptr::copy_nonoverlapping(cadir.as_ptr(), raw.szCADir.as_mut_ptr(), cd_len);
        }
        raw.bVerifyPeer = self.verify_peer as i32;
        raw.bVerifyClientOnce = self.verify_client_once as i32;
        raw.nVerifyDepth = self.verify_depth;
        raw
    }
}

/// Keep-alive configuration for client connections.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default)]
pub struct ClientKeepAlive {
    pub lost_ms: i32,
    pub tcp_interval_ms: i32,
    pub udp_interval_ms: i32,
    pub udp_rtx_ms: i32,
    pub udp_connect_rtx_ms: i32,
    pub udp_timeout_ms: i32,
}

impl From<ffi::ClientKeepAlive> for ClientKeepAlive {
    fn from(c: ffi::ClientKeepAlive) -> Self {
        Self {
            lost_ms: c.nConnectionLostMSec,
            tcp_interval_ms: c.nTcpKeepAliveIntervalMSec,
            udp_interval_ms: c.nUdpKeepAliveIntervalMSec,
            udp_rtx_ms: c.nUdpKeepAliveRTXMSec,
            udp_connect_rtx_ms: c.nUdpConnectRTXMSec,
            udp_timeout_ms: c.nUdpConnectTimeoutMSec,
        }
    }
}

impl ClientKeepAlive {
    /// Creates a new keep-alive configuration.
    pub fn new(
        lost_ms: i32,
        tcp_interval_ms: i32,
        udp_interval_ms: i32,
        udp_rtx_ms: i32,
        udp_connect_rtx_ms: i32,
        udp_timeout_ms: i32,
    ) -> Self {
        Self {
            lost_ms,
            tcp_interval_ms,
            udp_interval_ms,
            udp_rtx_ms,
            udp_connect_rtx_ms,
            udp_timeout_ms,
        }
    }
    /// Converts to the raw TeamTalk struct.
    pub fn to_ffi(&self) -> ffi::ClientKeepAlive {
        ffi::ClientKeepAlive {
            nConnectionLostMSec: self.lost_ms,
            nTcpKeepAliveIntervalMSec: self.tcp_interval_ms,
            nUdpKeepAliveIntervalMSec: self.udp_interval_ms,
            nUdpKeepAliveRTXMSec: self.udp_rtx_ms,
            nUdpConnectRTXMSec: self.udp_connect_rtx_ms,
            nUdpConnectTimeoutMSec: self.udp_timeout_ms,
        }
    }
}

/// Abuse prevention configuration.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default)]
pub struct AbusePrevention {
    pub commands_limit: i32,
    pub commands_interval_ms: i32,
}

impl From<ffi::AbusePrevention> for AbusePrevention {
    fn from(a: ffi::AbusePrevention) -> Self {
        Self {
            commands_limit: a.nCommandsLimit,
            commands_interval_ms: a.nCommandsIntervalMSec,
        }
    }
}

impl AbusePrevention {
    /// Creates a new abuse prevention configuration.
    pub fn new(commands_limit: i32, commands_interval_ms: i32) -> Self {
        Self {
            commands_limit,
            commands_interval_ms,
        }
    }
    /// Converts to the raw TeamTalk struct.
    pub fn to_ffi(&self) -> ffi::AbusePrevention {
        ffi::AbusePrevention {
            nCommandsLimit: self.commands_limit,
            nCommandsIntervalMSec: self.commands_interval_ms,
        }
    }
}

/// Audio format description.
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub struct AudioFormat {
    pub format: ffi::AudioFileFormat,
    pub sample_rate: i32,
    pub channels: i32,
}

impl Default for AudioFormat {
    fn default() -> Self {
        Self {
            format: ffi::AudioFileFormat::AFF_NONE,
            sample_rate: 0,
            channels: 0,
        }
    }
}

impl From<ffi::AudioFormat> for AudioFormat {
    fn from(f: ffi::AudioFormat) -> Self {
        Self {
            format: f.nAudioFmt,
            sample_rate: f.nSampleRate,
            channels: f.nChannels,
        }
    }
}

impl AudioFormat {
    /// Creates a new audio format.
    pub fn new(format: ffi::AudioFileFormat, sample_rate: i32, channels: i32) -> Self {
        Self {
            format,
            sample_rate,
            channels,
        }
    }
    /// Converts to the raw TeamTalk struct.
    pub fn to_ffi(&self) -> ffi::AudioFormat {
        ffi::AudioFormat {
            nAudioFmt: self.format,
            nSampleRate: self.sample_rate,
            nChannels: self.channels,
        }
    }
}

/// Video frame metadata.
#[non_exhaustive]
pub struct VideoFrame {
    pub width: i32,
    pub height: i32,
    pub stream_id: i32,
    pub key_frame: bool,
    pub buf: *mut std::ffi::c_void,
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

/// Audio input progress information.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioInputProgress {
    pub stream_id: i32,
    pub queue_ms: u32,
    pub elapsed_ms: u32,
}

impl From<ffi::AudioInputProgress> for AudioInputProgress {
    fn from(p: ffi::AudioInputProgress) -> Self {
        Self {
            stream_id: p.nStreamID,
            queue_ms: p.uQueueMSec,
            elapsed_ms: p.uElapsedMSec,
        }
    }
}

/// Desktop input packet.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default)]
pub struct DesktopInput {
    pub mouse_pos_x: u16,
    pub mouse_pos_y: u16,
    pub key_code: u32,
    pub key_state: ffi::DesktopKeyStates,
}

impl From<ffi::DesktopInput> for DesktopInput {
    fn from(input: ffi::DesktopInput) -> Self {
        Self {
            mouse_pos_x: input.uMousePosX,
            mouse_pos_y: input.uMousePosY,
            key_code: input.uKeyCode,
            key_state: input.uKeyState,
        }
    }
}

impl DesktopInput {
    /// Converts to the raw TeamTalk struct.
    pub fn to_ffi(&self) -> ffi::DesktopInput {
        ffi::DesktopInput {
            uMousePosX: self.mouse_pos_x,
            uMousePosY: self.mouse_pos_y,
            uKeyCode: self.key_code,
            uKeyState: self.key_state,
        }
    }
}

/// SDK error message payload.
#[non_exhaustive]
#[derive(Debug, Clone, Default)]
pub struct ErrorMessage {
    pub code: i32,
    pub message: String,
}

impl From<ffi::ClientErrorMsg> for ErrorMessage {
    fn from(e: ffi::ClientErrorMsg) -> Self {
        Self {
            code: e.nErrorNo,
            message: crate::utils::strings::to_string(&e.szErrorMsg),
        }
    }
}
