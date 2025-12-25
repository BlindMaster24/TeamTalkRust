use super::Client;
use crate::types::AudioCodec;
use crate::utils::ToTT;
use teamtalk_sys as ffi;

impl Client {
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

    pub fn stop_recording(&self) -> bool {
        unsafe { ffi::api().TT_StopRecordingMuxedAudioFile(self.ptr) == 1 }
    }

    pub fn stop_recording_channel(&self, channel_id: i32) -> bool {
        unsafe { ffi::api().TT_StopRecordingMuxedAudioFileEx(self.ptr, channel_id) == 1 }
    }
}
