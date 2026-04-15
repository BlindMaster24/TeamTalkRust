use crate::types::{AudioCodec, ChannelId};
use std::time::Duration;
use teamtalk_sys as ffi;

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum RecordingSampleFormat {
    PcmS16Le,
    WavS16Le,
}

/// Target for a managed recording session.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum RecordingTarget {
    /// Record the current channel audio (server codec).
    Channel(ChannelId),
    /// Record the channel you are currently joined to.
    CurrentChannel,
    /// Record muxed streams using a specific codec.
    Streams {
        stream_types: u32,
        codec: AudioCodec,
    },
    /// Record muxed audio using a specific codec.
    Muxed { codec: AudioCodec },
}

/// Configuration for managed recordings.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct RecordingOptions {
    /// Output file template. Use `{index}` to control segment numbering.
    pub template: String,
    /// Audio file format for the SDK muxed recorder.
    pub format: ffi::AudioFileFormat,
    /// First segment index to use.
    pub start_index: u32,
    /// Rotate after the given duration.
    pub max_duration: Option<Duration>,
    /// Rotate after the file exceeds the given size in bytes.
    pub max_size_bytes: Option<u64>,
    /// Rotate when the current channel changes.
    pub rotate_on_channel_change: bool,
    /// Rotate when the channel codec changes.
    pub rotate_on_codec_change: bool,
}

impl RecordingOptions {
    /// Creates options with a template and format.
    pub fn new(template: impl Into<String>, format: ffi::AudioFileFormat) -> Self {
        Self {
            template: template.into(),
            format,
            start_index: 1,
            max_duration: None,
            max_size_bytes: None,
            rotate_on_channel_change: true,
            rotate_on_codec_change: true,
        }
    }

    /// Sets a maximum segment duration.
    pub fn with_max_duration(mut self, duration: Duration) -> Self {
        self.max_duration = Some(duration);
        self
    }

    /// Sets a maximum segment size in bytes.
    pub fn with_max_size_bytes(mut self, size: u64) -> Self {
        self.max_size_bytes = Some(size);
        self
    }
}

pub(crate) fn segment_path(template: &str, index: u32) -> String {
    if template.contains("{index}") {
        return template.replace("{index}", &index.to_string());
    }

    let base = template.to_string();
    if let Some(pos) = base.rfind('.') {
        let (stem, ext) = base.split_at(pos);
        format!("{stem}.part{index}{ext}")
    } else {
        format!("{base}.part{index}")
    }
}
