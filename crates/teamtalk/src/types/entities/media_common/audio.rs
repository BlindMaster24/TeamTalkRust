//! Audio-related configuration, format, and input-progress types.

use teamtalk_sys as ffi;

/// Jitter control configuration.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default)]
pub struct JitterConfig {
    /// Fixed jitter delay in milliseconds.
    pub fixed_delay_ms: i32,
    /// Whether the adaptive dejitter algorithm is enabled.
    pub use_adaptive: bool,
    /// Maximum adaptive delay in milliseconds.
    pub max_adaptive_delay_ms: i32,
    /// Currently active adaptive delay in milliseconds.
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
    #[must_use]
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
    /// Converts to the raw `TeamTalk` struct.
    #[must_use]
    pub fn to_ffi(&self) -> ffi::JitterConfig {
        ffi::JitterConfig {
            nFixedDelayMSec: self.fixed_delay_ms,
            bUseAdativeDejitter: i32::from(self.use_adaptive),
            nMaxAdaptiveDelayMSec: self.max_adaptive_delay_ms,
            nActiveAdaptiveDelayMSec: self.active_adaptive_delay_ms,
        }
    }
}

/// Audio format description.
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub struct AudioFormat {
    /// Audio file format container.
    pub format: ffi::AudioFileFormat,
    /// Sample rate in Hz.
    pub sample_rate: i32,
    /// Number of channels.
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
    #[must_use]
    pub fn new(format: ffi::AudioFileFormat, sample_rate: i32, channels: i32) -> Self {
        Self {
            format,
            sample_rate,
            channels,
        }
    }
    /// Converts to the raw `TeamTalk` struct.
    #[must_use]
    pub fn to_ffi(&self) -> ffi::AudioFormat {
        ffi::AudioFormat {
            nAudioFmt: self.format,
            nSampleRate: self.sample_rate,
            nChannels: self.channels,
        }
    }
}

/// Audio input progress information.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioInputProgress {
    /// Stream id associated with the progress report.
    pub stream_id: i32,
    /// Queued audio duration in milliseconds.
    pub queue_ms: u32,
    /// Elapsed audio duration in milliseconds.
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
