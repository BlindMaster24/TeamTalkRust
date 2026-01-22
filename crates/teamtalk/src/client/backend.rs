use crate::types::{AudioCodec, Channel, ChannelId};
use crate::utils::ToTT;
use teamtalk_sys as ffi;

#[cfg(feature = "mock")]
pub trait TeamTalkBackend: Send + Sync {
    fn init_poll(&self) -> *mut ffi::TTInstance;
    fn init_hwnd(&self, hwnd: ffi::HWND, msg: u32) -> *mut ffi::TTInstance;
    fn close(&self, ptr: *mut ffi::TTInstance);
    fn start_recording_muxed(
        &self,
        ptr: *mut ffi::TTInstance,
        codec: &AudioCodec,
        file_path: &str,
        format: ffi::AudioFileFormat,
    ) -> bool;
    fn start_recording_channel(
        &self,
        ptr: *mut ffi::TTInstance,
        channel_id: i32,
        file_path: &str,
        format: ffi::AudioFileFormat,
    ) -> bool;
    fn start_recording_streams(
        &self,
        ptr: *mut ffi::TTInstance,
        stream_types: u32,
        codec: &AudioCodec,
        file_path: &str,
        format: ffi::AudioFileFormat,
    ) -> bool;
    fn stop_recording(&self, ptr: *mut ffi::TTInstance) -> bool;
    fn stop_recording_channel(&self, ptr: *mut ffi::TTInstance, channel_id: i32) -> bool;
    fn get_channel(&self, ptr: *mut ffi::TTInstance, channel_id: i32) -> Option<Channel>;
    fn get_my_channel_id(&self, ptr: *mut ffi::TTInstance) -> ChannelId;
}

#[cfg(not(feature = "mock"))]
pub(crate) trait TeamTalkBackend: Send + Sync {
    fn init_poll(&self) -> *mut ffi::TTInstance;
    fn init_hwnd(&self, hwnd: ffi::HWND, msg: u32) -> *mut ffi::TTInstance;
    fn close(&self, ptr: *mut ffi::TTInstance);
    fn start_recording_muxed(
        &self,
        ptr: *mut ffi::TTInstance,
        codec: &AudioCodec,
        file_path: &str,
        format: ffi::AudioFileFormat,
    ) -> bool;
    fn start_recording_channel(
        &self,
        ptr: *mut ffi::TTInstance,
        channel_id: i32,
        file_path: &str,
        format: ffi::AudioFileFormat,
    ) -> bool;
    fn start_recording_streams(
        &self,
        ptr: *mut ffi::TTInstance,
        stream_types: u32,
        codec: &AudioCodec,
        file_path: &str,
        format: ffi::AudioFileFormat,
    ) -> bool;
    fn stop_recording(&self, ptr: *mut ffi::TTInstance) -> bool;
    fn stop_recording_channel(&self, ptr: *mut ffi::TTInstance, channel_id: i32) -> bool;
    fn get_channel(&self, ptr: *mut ffi::TTInstance, channel_id: i32) -> Option<Channel>;
    fn get_my_channel_id(&self, ptr: *mut ffi::TTInstance) -> ChannelId;
}

pub(crate) struct FfiBackend;

impl TeamTalkBackend for FfiBackend {
    fn init_poll(&self) -> *mut ffi::TTInstance {
        unsafe { ffi::api().TT_InitTeamTalkPoll() }
    }

    fn init_hwnd(&self, hwnd: ffi::HWND, msg: u32) -> *mut ffi::TTInstance {
        unsafe { ffi::api().TT_InitTeamTalk(hwnd, msg) }
    }

    fn close(&self, ptr: *mut ffi::TTInstance) {
        unsafe {
            ffi::api().TT_CloseTeamTalk(ptr);
        }
    }

    fn start_recording_muxed(
        &self,
        ptr: *mut ffi::TTInstance,
        codec: &AudioCodec,
        file_path: &str,
        format: ffi::AudioFileFormat,
    ) -> bool {
        let p = file_path.tt();
        let raw_codec = codec.to_ffi();
        unsafe {
            ffi::api().TT_StartRecordingMuxedAudioFile(ptr, &raw_codec, p.as_ptr(), format) == 1
        }
    }

    fn start_recording_channel(
        &self,
        ptr: *mut ffi::TTInstance,
        channel_id: i32,
        file_path: &str,
        format: ffi::AudioFileFormat,
    ) -> bool {
        let p = file_path.tt();
        unsafe {
            ffi::api().TT_StartRecordingMuxedAudioFileEx(ptr, channel_id, p.as_ptr(), format) == 1
        }
    }

    fn start_recording_streams(
        &self,
        ptr: *mut ffi::TTInstance,
        stream_types: u32,
        codec: &AudioCodec,
        file_path: &str,
        format: ffi::AudioFileFormat,
    ) -> bool {
        let p = file_path.tt();
        let raw_codec = codec.to_ffi();
        unsafe {
            ffi::api().TT_StartRecordingMuxedStreams(
                ptr,
                stream_types,
                &raw_codec,
                p.as_ptr(),
                format,
            ) == 1
        }
    }

    fn stop_recording(&self, ptr: *mut ffi::TTInstance) -> bool {
        unsafe { ffi::api().TT_StopRecordingMuxedAudioFile(ptr) == 1 }
    }

    fn stop_recording_channel(&self, ptr: *mut ffi::TTInstance, channel_id: i32) -> bool {
        unsafe { ffi::api().TT_StopRecordingMuxedAudioFileEx(ptr, channel_id) == 1 }
    }

    fn get_channel(&self, ptr: *mut ffi::TTInstance, channel_id: i32) -> Option<Channel> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::Channel>() };
        if unsafe { ffi::api().TT_GetChannel(ptr, channel_id, &mut raw) } == 1 {
            Some(Channel::from(raw))
        } else {
            None
        }
    }

    fn get_my_channel_id(&self, ptr: *mut ffi::TTInstance) -> ChannelId {
        ChannelId(unsafe { ffi::api().TT_GetMyChannelID(ptr) })
    }
}

#[cfg(feature = "mock")]
#[derive(Default)]
pub struct MockBackend {
    state: std::sync::Mutex<MockBackendState>,
}

#[cfg(feature = "mock")]
#[derive(Default)]
struct MockBackendState {
    channels: std::collections::HashMap<i32, Channel>,
    my_channel_id: ChannelId,
    start_ok: bool,
    stop_ok: bool,
}

#[cfg(feature = "mock")]
impl MockBackend {
    pub fn new() -> Self {
        Self {
            state: std::sync::Mutex::new(MockBackendState {
                start_ok: true,
                stop_ok: true,
                ..MockBackendState::default()
            }),
        }
    }

    pub fn set_channel(&self, channel: Channel) {
        let mut state = self.state.lock().unwrap();
        state.channels.insert(channel.id.0, channel);
    }

    pub fn set_my_channel_id(&self, channel_id: ChannelId) {
        let mut state = self.state.lock().unwrap();
        state.my_channel_id = channel_id;
    }

    pub fn set_start_ok(&self, ok: bool) {
        let mut state = self.state.lock().unwrap();
        state.start_ok = ok;
    }

    pub fn set_stop_ok(&self, ok: bool) {
        let mut state = self.state.lock().unwrap();
        state.stop_ok = ok;
    }
}

#[cfg(feature = "mock")]
impl TeamTalkBackend for MockBackend {
    fn init_poll(&self) -> *mut ffi::TTInstance {
        std::ptr::dangling_mut()
    }

    fn init_hwnd(&self, _hwnd: ffi::HWND, _msg: u32) -> *mut ffi::TTInstance {
        self.init_poll()
    }

    fn close(&self, ptr: *mut ffi::TTInstance) {
        let _ = ptr;
    }

    fn start_recording_muxed(
        &self,
        _ptr: *mut ffi::TTInstance,
        _codec: &AudioCodec,
        _file_path: &str,
        _format: ffi::AudioFileFormat,
    ) -> bool {
        self.state.lock().unwrap().start_ok
    }

    fn start_recording_channel(
        &self,
        _ptr: *mut ffi::TTInstance,
        _channel_id: i32,
        _file_path: &str,
        _format: ffi::AudioFileFormat,
    ) -> bool {
        self.state.lock().unwrap().start_ok
    }

    fn start_recording_streams(
        &self,
        _ptr: *mut ffi::TTInstance,
        _stream_types: u32,
        _codec: &AudioCodec,
        _file_path: &str,
        _format: ffi::AudioFileFormat,
    ) -> bool {
        self.state.lock().unwrap().start_ok
    }

    fn stop_recording(&self, _ptr: *mut ffi::TTInstance) -> bool {
        self.state.lock().unwrap().stop_ok
    }

    fn stop_recording_channel(&self, _ptr: *mut ffi::TTInstance, _channel_id: i32) -> bool {
        self.state.lock().unwrap().stop_ok
    }

    fn get_channel(&self, _ptr: *mut ffi::TTInstance, channel_id: i32) -> Option<Channel> {
        let state = self.state.lock().unwrap();
        state.channels.get(&channel_id).cloned()
    }

    fn get_my_channel_id(&self, _ptr: *mut ffi::TTInstance) -> ChannelId {
        self.state.lock().unwrap().my_channel_id
    }
}
