use super::super::Client;
use crate::events::{Error, Result};
use crate::types::{AudioCodec, ChannelId, StreamTypes};
use teamtalk_sys as ffi;

impl Client {
    /// Starts recording a muxed audio file using a codec.
    #[must_use]
    pub fn start_recording_muxed(
        &self,
        codec: &AudioCodec,
        file_path: &str,
        format: ffi::AudioFileFormat,
    ) -> bool {
        self.backend()
            .start_recording_muxed(self.ptr.0, codec, file_path, format)
    }

    /// Starts recording the specified channel.
    #[must_use]
    pub fn start_recording_channel(
        &self,
        channel_id: ChannelId,
        file_path: &str,
        format: ffi::AudioFileFormat,
    ) -> bool {
        self.backend()
            .start_recording_channel(self.ptr.0, channel_id, file_path, format)
    }

    /// Starts recording a set of stream types.
    ///
    /// `stream_types` accepts both a raw `u32` bitmask and any
    /// [`StreamTypes`] combination via the `Into<StreamTypes>` bound.
    #[must_use]
    pub fn start_recording_streams(
        &self,
        stream_types: impl Into<StreamTypes>,
        codec: &AudioCodec,
        file_path: &str,
        format: ffi::AudioFileFormat,
    ) -> bool {
        self.backend().start_recording_streams(
            self.ptr.0,
            stream_types.into().raw(),
            codec,
            file_path,
            format,
        )
    }

    /// Stops recording a muxed audio file.
    #[must_use]
    pub fn stop_recording(&self) -> bool {
        self.backend().stop_recording(self.ptr.0)
    }

    /// Stops recording for a channel.
    #[must_use]
    pub fn stop_recording_channel(&self, channel_id: ChannelId) -> bool {
        self.backend()
            .stop_recording_channel(self.ptr.0, channel_id)
    }
}

/// Guard that stops a channel recording when dropped.
pub struct RecordSession<'a> {
    client: &'a Client,
    channel_id: ChannelId,
    active: bool,
}

impl<'a> RecordSession<'a> {
    /// Starts recording a channel and returns a guard that stops on drop.
    pub fn start_channel(
        client: &'a Client,
        channel_id: ChannelId,
        file_path: &str,
        format: ffi::AudioFileFormat,
    ) -> Result<Self> {
        if client.start_recording_channel(channel_id, file_path, format) {
            Ok(Self {
                client,
                channel_id,
                active: true,
            })
        } else {
            Err(Error::CommandFailed {
                code: -1,
                message: "Recording start failed".to_string(),
            })
        }
    }

    /// Stops the recording and returns whether it succeeded.
    #[must_use]
    pub fn stop(mut self) -> bool {
        let ok = self.client.stop_recording_channel(self.channel_id);
        self.active = false;
        ok
    }
}

impl Drop for RecordSession<'_> {
    fn drop(&mut self) {
        if self.active {
            let _ = self.client.stop_recording_channel(self.channel_id);
        }
    }
}
