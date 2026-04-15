use teamtalk_sys as ffi;

/// Speex DSP preprocessing settings.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default)]
pub struct SpeexDSP {
    pub enable_agc: bool,
    pub gain_level: i32,
    pub max_inc_db_sec: i32,
    pub max_dec_db_sec: i32,
    pub max_gain_db: i32,
    pub enable_denoise: bool,
    pub max_noise_suppress_db: i32,
    pub enable_aec: bool,
    pub echo_suppress: i32,
    pub echo_suppress_active: i32,
}

impl From<ffi::SpeexDSP> for SpeexDSP {
    fn from(d: ffi::SpeexDSP) -> Self {
        Self {
            enable_agc: d.bEnableAGC != 0,
            gain_level: d.nGainLevel,
            max_inc_db_sec: d.nMaxIncDBSec,
            max_dec_db_sec: d.nMaxDecDBSec,
            max_gain_db: d.nMaxGainDB,
            enable_denoise: d.bEnableDenoise != 0,
            max_noise_suppress_db: d.nMaxNoiseSuppressDB,
            enable_aec: d.bEnableEchoCancellation != 0,
            echo_suppress: d.nEchoSuppress,
            echo_suppress_active: d.nEchoSuppressActive,
        }
    }
}

impl SpeexDSP {
    /// Creates new Speex DSP preprocessing settings.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        enable_agc: bool,
        gain_level: i32,
        max_inc_db_sec: i32,
        max_dec_db_sec: i32,
        max_gain_db: i32,
        enable_denoise: bool,
        max_noise_suppress_db: i32,
        enable_aec: bool,
        echo_suppress: i32,
        echo_suppress_active: i32,
    ) -> Self {
        Self {
            enable_agc,
            gain_level,
            max_inc_db_sec,
            max_dec_db_sec,
            max_gain_db,
            enable_denoise,
            max_noise_suppress_db,
            enable_aec,
            echo_suppress,
            echo_suppress_active,
        }
    }
    /// Converts to the raw TeamTalk struct.
    pub fn to_ffi(&self) -> ffi::SpeexDSP {
        ffi::SpeexDSP {
            bEnableAGC: self.enable_agc as i32,
            nGainLevel: self.gain_level,
            nMaxIncDBSec: self.max_inc_db_sec,
            nMaxDecDBSec: self.max_dec_db_sec,
            nMaxGainDB: self.max_gain_db,
            bEnableDenoise: self.enable_denoise as i32,
            nMaxNoiseSuppressDB: self.max_noise_suppress_db,
            bEnableEchoCancellation: self.enable_aec as i32,
            nEchoSuppress: self.echo_suppress,
            nEchoSuppressActive: self.echo_suppress_active,
        }
    }
}

/// WebRTC audio preprocessing settings.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default)]
pub struct WebRTCConfig {
    pub preamplifier_enable: bool,
    pub preamplifier_gain: f32,
    pub aec_enable: bool,
    pub ns_enable: bool,
    pub ns_level: i32,
    pub agc2_enable: bool,
    pub agc2_gain_db: f32,
}

impl From<ffi::WebRTCAudioPreprocessor> for WebRTCConfig {
    fn from(w: ffi::WebRTCAudioPreprocessor) -> Self {
        Self {
            preamplifier_enable: w.preamplifier.bEnable != 0,
            preamplifier_gain: w.preamplifier.fFixedGainFactor,
            aec_enable: w.echocanceller.bEnable != 0,
            ns_enable: w.noisesuppression.bEnable != 0,
            ns_level: w.noisesuppression.nLevel,
            agc2_enable: w.gaincontroller2.bEnable != 0,
            agc2_gain_db: w.gaincontroller2.fixeddigital.fGainDB,
        }
    }
}

impl WebRTCConfig {
    /// Creates new WebRTC audio preprocessing settings.
    pub fn new(
        preamplifier_enable: bool,
        preamplifier_gain: f32,
        aec_enable: bool,
        ns_enable: bool,
        ns_level: i32,
        agc2_enable: bool,
        agc2_gain_db: f32,
    ) -> Self {
        Self {
            preamplifier_enable,
            preamplifier_gain,
            aec_enable,
            ns_enable,
            ns_level,
            agc2_enable,
            agc2_gain_db,
        }
    }
    /// Converts to the raw TeamTalk struct.
    pub fn to_ffi(&self) -> ffi::WebRTCAudioPreprocessor {
        let mut raw = ffi::WebRTCAudioPreprocessor::default();
        raw.preamplifier.bEnable = self.preamplifier_enable as i32;
        raw.preamplifier.fFixedGainFactor = self.preamplifier_gain;
        raw.echocanceller.bEnable = self.aec_enable as i32;
        raw.noisesuppression.bEnable = self.ns_enable as i32;
        raw.noisesuppression.nLevel = self.ns_level;
        raw.gaincontroller2.bEnable = self.agc2_enable as i32;
        raw.gaincontroller2.fixeddigital.fGainDB = self.agc2_gain_db;
        raw
    }
}

/// Audio preprocessing configuration.
#[derive(Debug, Clone, Copy, Default)]
pub enum AudioPreprocessor {
    #[default]
    None,
    Speex(SpeexDSP),
    WebRTC(WebRTCConfig),
}

impl From<ffi::AudioPreprocessor> for AudioPreprocessor {
    fn from(p: ffi::AudioPreprocessor) -> Self {
        match p.nPreprocessor {
            ffi::AudioPreprocessorType::SPEEXDSP_AUDIOPREPROCESSOR => {
                Self::Speex(SpeexDSP::from(unsafe { p.__bindgen_anon_1.speexdsp }))
            }
            ffi::AudioPreprocessorType::WEBRTC_AUDIOPREPROCESSOR => {
                Self::WebRTC(WebRTCConfig::from(unsafe { p.__bindgen_anon_1.webrtc }))
            }
            _ => Self::None,
        }
    }
}

impl AudioPreprocessor {
    /// Converts to the raw TeamTalk struct.
    pub fn to_ffi(&self) -> ffi::AudioPreprocessor {
        let mut raw = ffi::AudioPreprocessor::default();
        match self {
            Self::None => raw.nPreprocessor = ffi::AudioPreprocessorType::NO_AUDIOPREPROCESSOR,
            Self::Speex(s) => {
                raw.nPreprocessor = ffi::AudioPreprocessorType::SPEEXDSP_AUDIOPREPROCESSOR;
                raw.__bindgen_anon_1.speexdsp = s.to_ffi();
            }
            Self::WebRTC(w) => {
                raw.nPreprocessor = ffi::AudioPreprocessorType::WEBRTC_AUDIOPREPROCESSOR;
                raw.__bindgen_anon_1.webrtc = w.to_ffi();
            }
        }
        raw
    }
}
