use crate::types::{AudioCodec, ChannelId};
use teamtalk_sys as ffi;

/// Target for a managed recording session.
#[derive(Clone, Debug)]
pub enum RecordingTarget {
    /// Record the current channel audio (server codec).
    Channel(ChannelId),
    /// Record muxed streams using a specific codec.
    Streams {
        stream_types: u32,
        codec: AudioCodec,
    },
    /// Record muxed audio using a specific codec.
    Muxed { codec: AudioCodec },
}

/// Configuration for managed recordings.
#[derive(Clone, Debug)]
pub struct RecordingOptions {
    /// Output file template. Use `{index}` to control segment numbering.
    pub template: String,
    /// Audio file format for the SDK muxed recorder.
    pub format: ffi::AudioFileFormat,
    /// First segment index to use.
    pub start_index: u32,
}

impl RecordingOptions {
    /// Creates options with a template and format.
    pub fn new(template: impl Into<String>, format: ffi::AudioFileFormat) -> Self {
        Self {
            template: template.into(),
            format,
            start_index: 1,
        }
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
