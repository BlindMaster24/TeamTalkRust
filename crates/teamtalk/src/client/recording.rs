//! Recording APIs for channels and streams.
use super::Client;
use crate::events::{Error, Result};
use crate::types::{AudioCodec, ChannelId};
use crate::utils::ToTT;
use teamtalk_sys as ffi;

impl Client {
    /// Starts recording a muxed audio file using a codec.
    pub fn start_recording_muxed(
        &self,
        codec: &AudioCodec,
        file_path: &str,
        format: ffi::AudioFileFormat,
    ) -> bool {
        let p = file_path.tt();
        let raw_codec = codec.to_ffi();
        unsafe {
            ffi::api().TT_StartRecordingMuxedAudioFile(self.ptr, &raw_codec, p.as_ptr(), format)
                == 1
        }
    }

    /// Starts recording the specified channel.
    pub fn start_recording_channel(
        &self,
        channel_id: i32,
        file_path: &str,
        format: ffi::AudioFileFormat,
    ) -> bool {
        let p = file_path.tt();
        unsafe {
            ffi::api().TT_StartRecordingMuxedAudioFileEx(self.ptr, channel_id, p.as_ptr(), format)
                == 1
        }
    }

    /// Starts recording a set of stream types.
    pub fn start_recording_streams(
        &self,
        stream_types: u32,
        codec: &AudioCodec,
        file_path: &str,
        format: ffi::AudioFileFormat,
    ) -> bool {
        let p = file_path.tt();
        let raw_codec = codec.to_ffi();
        unsafe {
            ffi::api().TT_StartRecordingMuxedStreams(
                self.ptr,
                stream_types,
                &raw_codec,
                p.as_ptr(),
                format,
            ) == 1
        }
    }

    /// Stops recording a muxed audio file.
    pub fn stop_recording(&self) -> bool {
        unsafe { ffi::api().TT_StopRecordingMuxedAudioFile(self.ptr) == 1 }
    }

    /// Stops recording for a channel.
    pub fn stop_recording_channel(&self, channel_id: i32) -> bool {
        unsafe { ffi::api().TT_StopRecordingMuxedAudioFileEx(self.ptr, channel_id) == 1 }
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
        if client.start_recording_channel(channel_id.0, file_path, format) {
            Ok(Self {
                client,
                channel_id,
                active: true,
            })
        } else {
            Err(Error::CommandFailed(0))
        }
    }

    /// Stops the recording and returns whether it succeeded.
    pub fn stop(mut self) -> bool {
        let ok = self.client.stop_recording_channel(self.channel_id.0);
        self.active = false;
        ok
    }
}

impl Drop for RecordSession<'_> {
    fn drop(&mut self) {
        if self.active {
            let _ = self.client.stop_recording_channel(self.channel_id.0);
        }
    }
}
