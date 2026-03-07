use crate::types::{AudioCodec, Channel, ChannelId};
use crate::utils::ToTT;
use teamtalk_sys as ffi;

#[cfg(feature = "mock")]
pub trait TeamTalkBackend: Send + Sync {
    fn init_poll(&self) -> *mut ffi::TTInstance;
    #[cfg(windows)]
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
    fn do_login_ex(
        &self,
        ptr: *mut ffi::TTInstance,
        nickname: &str,
        username: &str,
        password: &str,
        client_name: &str,
    ) -> i32;
    fn do_logout(&self, ptr: *mut ffi::TTInstance) -> i32;
    fn do_join_channel_by_id(
        &self,
        ptr: *mut ffi::TTInstance,
        channel_id: i32,
        password: &str,
    ) -> i32;
    fn do_leave_channel(&self, ptr: *mut ffi::TTInstance) -> i32;
    fn do_text_message(&self, ptr: *mut ffi::TTInstance, message: &ffi::TextMessage) -> i32;
    fn do_change_status(&self, ptr: *mut ffi::TTInstance, status_mode: i32, message: &str) -> i32;
    fn get_channel(&self, ptr: *mut ffi::TTInstance, channel_id: i32) -> Option<Channel>;
    fn get_my_user_id(&self, ptr: *mut ffi::TTInstance) -> i32;
    fn get_my_user_rights(&self, ptr: *mut ffi::TTInstance) -> u32;
    fn get_user(&self, ptr: *mut ffi::TTInstance, user_id: i32, user: &mut ffi::User) -> bool;
    fn get_my_channel_id(&self, ptr: *mut ffi::TTInstance) -> ChannelId;
    fn connect(
        &self,
        ptr: *mut ffi::TTInstance,
        host: &str,
        tcp: i32,
        udp: i32,
        encrypted: bool,
    ) -> bool;
    fn connect_sys_id(
        &self,
        ptr: *mut ffi::TTInstance,
        host: &str,
        tcp: i32,
        udp: i32,
        encrypted: bool,
        sys_id: &str,
    ) -> bool;
    fn connect_ex(
        &self,
        ptr: *mut ffi::TTInstance,
        host: &str,
        tcp: i32,
        udp: i32,
        bind_ip: &str,
        encrypted: bool,
    ) -> bool;
    fn disconnect(&self, ptr: *mut ffi::TTInstance) -> bool;
    fn get_flags(&self, ptr: *mut ffi::TTInstance) -> u32;
}

#[cfg(not(feature = "mock"))]
pub(crate) trait TeamTalkBackend: Send + Sync {
    fn init_poll(&self) -> *mut ffi::TTInstance;
    #[cfg(windows)]
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
    fn do_login_ex(
        &self,
        ptr: *mut ffi::TTInstance,
        nickname: &str,
        username: &str,
        password: &str,
        client_name: &str,
    ) -> i32;
    fn do_logout(&self, ptr: *mut ffi::TTInstance) -> i32;
    fn do_join_channel_by_id(
        &self,
        ptr: *mut ffi::TTInstance,
        channel_id: i32,
        password: &str,
    ) -> i32;
    fn do_leave_channel(&self, ptr: *mut ffi::TTInstance) -> i32;
    fn do_text_message(&self, ptr: *mut ffi::TTInstance, message: &ffi::TextMessage) -> i32;
    fn do_change_status(&self, ptr: *mut ffi::TTInstance, status_mode: i32, message: &str) -> i32;
    fn get_channel(&self, ptr: *mut ffi::TTInstance, channel_id: i32) -> Option<Channel>;
    fn get_my_user_id(&self, ptr: *mut ffi::TTInstance) -> i32;
    fn get_my_user_rights(&self, ptr: *mut ffi::TTInstance) -> u32;
    fn get_user(&self, ptr: *mut ffi::TTInstance, user_id: i32, user: &mut ffi::User) -> bool;
    fn get_my_channel_id(&self, ptr: *mut ffi::TTInstance) -> ChannelId;
    fn connect(
        &self,
        ptr: *mut ffi::TTInstance,
        host: &str,
        tcp: i32,
        udp: i32,
        encrypted: bool,
    ) -> bool;
    fn connect_sys_id(
        &self,
        ptr: *mut ffi::TTInstance,
        host: &str,
        tcp: i32,
        udp: i32,
        encrypted: bool,
        sys_id: &str,
    ) -> bool;
    fn connect_ex(
        &self,
        ptr: *mut ffi::TTInstance,
        host: &str,
        tcp: i32,
        udp: i32,
        bind_ip: &str,
        encrypted: bool,
    ) -> bool;
    fn disconnect(&self, ptr: *mut ffi::TTInstance) -> bool;
    fn get_flags(&self, ptr: *mut ffi::TTInstance) -> u32;
}

pub(crate) struct FfiBackend;

impl TeamTalkBackend for FfiBackend {
    fn init_poll(&self) -> *mut ffi::TTInstance {
        unsafe { ffi::api().TT_InitTeamTalkPoll() }
    }

    #[cfg(windows)]
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

    fn do_login_ex(
        &self,
        ptr: *mut ffi::TTInstance,
        nickname: &str,
        username: &str,
        password: &str,
        client_name: &str,
    ) -> i32 {
        unsafe {
            ffi::api().TT_DoLoginEx(
                ptr,
                nickname.tt().as_ptr(),
                username.tt().as_ptr(),
                password.tt().as_ptr(),
                client_name.tt().as_ptr(),
            )
        }
    }

    fn do_logout(&self, ptr: *mut ffi::TTInstance) -> i32 {
        unsafe { ffi::api().TT_DoLogout(ptr) }
    }

    fn do_join_channel_by_id(
        &self,
        ptr: *mut ffi::TTInstance,
        channel_id: i32,
        password: &str,
    ) -> i32 {
        unsafe { ffi::api().TT_DoJoinChannelByID(ptr, channel_id, password.tt().as_ptr()) }
    }

    fn do_leave_channel(&self, ptr: *mut ffi::TTInstance) -> i32 {
        unsafe { ffi::api().TT_DoLeaveChannel(ptr) }
    }

    fn do_text_message(&self, ptr: *mut ffi::TTInstance, message: &ffi::TextMessage) -> i32 {
        unsafe { ffi::api().TT_DoTextMessage(ptr, message) }
    }

    fn do_change_status(&self, ptr: *mut ffi::TTInstance, status_mode: i32, message: &str) -> i32 {
        unsafe { ffi::api().TT_DoChangeStatus(ptr, status_mode, message.tt().as_ptr()) }
    }

    fn get_channel(&self, ptr: *mut ffi::TTInstance, channel_id: i32) -> Option<Channel> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::Channel>() };
        if unsafe { ffi::api().TT_GetChannel(ptr, channel_id, &mut raw) } == 1 {
            Some(Channel::from(raw))
        } else {
            None
        }
    }

    fn get_my_user_id(&self, ptr: *mut ffi::TTInstance) -> i32 {
        unsafe { ffi::api().TT_GetMyUserID(ptr) }
    }

    fn get_my_user_rights(&self, ptr: *mut ffi::TTInstance) -> u32 {
        unsafe { ffi::api().TT_GetMyUserRights(ptr) }
    }

    fn get_user(&self, ptr: *mut ffi::TTInstance, user_id: i32, user: &mut ffi::User) -> bool {
        unsafe { ffi::api().TT_GetUser(ptr, user_id, user) == 1 }
    }

    fn get_my_channel_id(&self, ptr: *mut ffi::TTInstance) -> ChannelId {
        ChannelId(unsafe { ffi::api().TT_GetMyChannelID(ptr) })
    }

    fn connect(
        &self,
        ptr: *mut ffi::TTInstance,
        host: &str,
        tcp: i32,
        udp: i32,
        encrypted: bool,
    ) -> bool {
        unsafe {
            ffi::api().TT_Connect(
                ptr,
                host.tt().as_ptr(),
                tcp,
                udp,
                0,
                0,
                if encrypted { 1 } else { 0 },
            ) == 1
        }
    }

    fn connect_sys_id(
        &self,
        ptr: *mut ffi::TTInstance,
        host: &str,
        tcp: i32,
        udp: i32,
        encrypted: bool,
        sys_id: &str,
    ) -> bool {
        unsafe {
            ffi::api().TT_ConnectSysID(
                ptr,
                host.tt().as_ptr(),
                tcp,
                udp,
                0,
                0,
                if encrypted { 1 } else { 0 },
                sys_id.tt().as_ptr(),
            ) == 1
        }
    }

    fn connect_ex(
        &self,
        ptr: *mut ffi::TTInstance,
        host: &str,
        tcp: i32,
        udp: i32,
        bind_ip: &str,
        encrypted: bool,
    ) -> bool {
        unsafe {
            ffi::api().TT_ConnectEx(
                ptr,
                host.tt().as_ptr(),
                tcp,
                udp,
                bind_ip.tt().as_ptr(),
                0,
                0,
                if encrypted { 1 } else { 0 },
            ) == 1
        }
    }

    fn disconnect(&self, ptr: *mut ffi::TTInstance) -> bool {
        unsafe { ffi::api().TT_Disconnect(ptr) == 1 }
    }

    fn get_flags(&self, ptr: *mut ffi::TTInstance) -> u32 {
        unsafe { ffi::api().TT_GetFlags(ptr) }
    }
}

#[cfg(feature = "mock")]
#[path = "backend_mock.rs"]
mod mock;
#[cfg(feature = "mock")]
pub use mock::MockBackend;
