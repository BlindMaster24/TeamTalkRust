use teamtalk_sys as ffi;

/// Speex audio codec configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpeexCodec {
    pub bandmode: i32,
    pub quality: i32,
    pub tx_interval_msec: i32,
    pub stereo_playback: bool,
}

impl From<ffi::SpeexCodec> for SpeexCodec {
    fn from(c: ffi::SpeexCodec) -> Self {
        Self {
            bandmode: c.nBandmode,
            quality: c.nQuality,
            tx_interval_msec: c.nTxIntervalMSec,
            stereo_playback: c.bStereoPlayback != 0,
        }
    }
}

impl SpeexCodec {
    /// Converts to the raw TeamTalk struct.
    pub fn to_ffi(&self) -> ffi::SpeexCodec {
        ffi::SpeexCodec {
            nBandmode: self.bandmode,
            nQuality: self.quality,
            nTxIntervalMSec: self.tx_interval_msec,
            bStereoPlayback: self.stereo_playback as i32,
        }
    }
}

/// Speex VBR audio codec configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpeexVBRCodec {
    pub bandmode: i32,
    pub quality: i32,
    pub bitrate: i32,
    pub max_bitrate: i32,
    pub dtx: bool,
    pub tx_interval_msec: i32,
    pub stereo_playback: bool,
}

impl From<ffi::SpeexVBRCodec> for SpeexVBRCodec {
    fn from(c: ffi::SpeexVBRCodec) -> Self {
        Self {
            bandmode: c.nBandmode,
            quality: c.nQuality,
            bitrate: c.nBitRate,
            max_bitrate: c.nMaxBitRate,
            dtx: c.bDTX != 0,
            tx_interval_msec: c.nTxIntervalMSec,
            stereo_playback: c.bStereoPlayback != 0,
        }
    }
}

impl SpeexVBRCodec {
    /// Converts to the raw TeamTalk struct.
    pub fn to_ffi(&self) -> ffi::SpeexVBRCodec {
        ffi::SpeexVBRCodec {
            nBandmode: self.bandmode,
            nQuality: self.quality,
            nBitRate: self.bitrate,
            nMaxBitRate: self.max_bitrate,
            bDTX: self.dtx as i32,
            nTxIntervalMSec: self.tx_interval_msec,
            bStereoPlayback: self.stereo_playback as i32,
        }
    }
}

/// Opus audio codec configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpusCodec {
    pub sample_rate: i32,
    pub channels: i32,
    pub application: i32,
    pub complexity: i32,
    pub fec: bool,
    pub dtx: bool,
    pub bitrate: i32,
    pub vbr: bool,
    pub vbr_constraint: bool,
    pub tx_interval_msec: i32,
    pub frame_size_msec: i32,
}

impl From<ffi::OpusCodec> for OpusCodec {
    fn from(c: ffi::OpusCodec) -> Self {
        Self {
            sample_rate: c.nSampleRate,
            channels: c.nChannels,
            application: c.nApplication,
            complexity: c.nComplexity,
            fec: c.bFEC != 0,
            dtx: c.bDTX != 0,
            bitrate: c.nBitRate,
            vbr: c.bVBR != 0,
            vbr_constraint: c.bVBRConstraint != 0,
            tx_interval_msec: c.nTxIntervalMSec,
            frame_size_msec: c.nFrameSizeMSec,
        }
    }
}

impl OpusCodec {
    /// Converts to the raw TeamTalk struct.
    pub fn to_ffi(&self) -> ffi::OpusCodec {
        ffi::OpusCodec {
            nSampleRate: self.sample_rate,
            nChannels: self.channels,
            nApplication: self.application,
            nComplexity: self.complexity,
            bFEC: self.fec as i32,
            bDTX: self.dtx as i32,
            nBitRate: self.bitrate,
            bVBR: self.vbr as i32,
            bVBRConstraint: self.vbr_constraint as i32,
            nTxIntervalMSec: self.tx_interval_msec,
            nFrameSizeMSec: self.frame_size_msec,
        }
    }
}

/// Audio codec selection.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AudioCodec {
    #[default]
    None,
    Speex(SpeexCodec),
    SpeexVBR(SpeexVBRCodec),
    Opus(OpusCodec),
}

impl From<ffi::AudioCodec> for AudioCodec {
    fn from(c: ffi::AudioCodec) -> Self {
        match c.nCodec {
            ffi::Codec::SPEEX_CODEC => unsafe {
                Self::Speex(SpeexCodec::from(c.__bindgen_anon_1.speex))
            },
            ffi::Codec::SPEEX_VBR_CODEC => unsafe {
                Self::SpeexVBR(SpeexVBRCodec::from(c.__bindgen_anon_1.speex_vbr))
            },
            ffi::Codec::OPUS_CODEC => unsafe {
                Self::Opus(OpusCodec::from(c.__bindgen_anon_1.opus))
            },
            _ => Self::None,
        }
    }
}

impl AudioCodec {
    /// Converts to the raw TeamTalk struct.
    pub fn to_ffi(&self) -> ffi::AudioCodec {
        let mut raw = ffi::AudioCodec::default();
        match self {
            Self::None => raw.nCodec = ffi::Codec::NO_CODEC,
            Self::Speex(c) => {
                raw.nCodec = ffi::Codec::SPEEX_CODEC;
                raw.__bindgen_anon_1.speex = c.to_ffi();
            }
            Self::SpeexVBR(c) => {
                raw.nCodec = ffi::Codec::SPEEX_VBR_CODEC;
                raw.__bindgen_anon_1.speex_vbr = c.to_ffi();
            }
            Self::Opus(c) => {
                raw.nCodec = ffi::Codec::OPUS_CODEC;
                raw.__bindgen_anon_1.opus = c.to_ffi();
            }
        }
        raw
    }
}

/// Audio input configuration.
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioConfig {
    pub enable_agc: bool,
    pub gain_level: i32,
}

impl From<ffi::AudioConfig> for AudioConfig {
    fn from(c: ffi::AudioConfig) -> Self {
        Self {
            enable_agc: c.bEnableAGC != 0,
            gain_level: c.nGainLevel,
        }
    }
}

impl AudioConfig {
    /// Converts to the raw TeamTalk struct.
    pub fn to_ffi(&self) -> ffi::AudioConfig {
        ffi::AudioConfig {
            bEnableAGC: self.enable_agc as i32,
            nGainLevel: self.gain_level,
        }
    }
}
