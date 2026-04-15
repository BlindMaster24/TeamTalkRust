use crate::types::{
    AudioCodec, Channel, ChannelId, ClientStatistics, FileId, RemoteFile, ServerProperties,
    Subscriptions, TransferId, User, UserAccount, UserId, UserStatistics,
};
use crate::utils::ToTT;
use teamtalk_sys as ffi;

#[cfg(feature = "mock")]
pub(crate) mod sealed {
    pub trait Sealed {}
}

#[cfg(feature = "mock")]
/// Internal backend abstraction used by crate-provided mock and runtime paths.
///
/// This trait is sealed and is not intended for downstream implementations.
pub trait TeamTalkBackend: sealed::Sealed + Send + Sync {
    fn init_poll(&self) -> *mut ffi::TTInstance;
    #[cfg(windows)]
    fn init_hwnd(&self, hwnd: ffi::HWND, msg: u32) -> *mut ffi::TTInstance;
    fn get_message(
        &self,
        ptr: *mut ffi::TTInstance,
        msg: &mut ffi::TTMessage,
        timeout_ms: &i32,
    ) -> bool;
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
        channel_id: ChannelId,
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
    fn stop_recording_channel(&self, ptr: *mut ffi::TTInstance, channel_id: ChannelId) -> bool;
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
        channel_id: ChannelId,
        password: &str,
    ) -> i32;
    fn do_leave_channel(&self, ptr: *mut ffi::TTInstance) -> i32;
    fn do_send_file(
        &self,
        ptr: *mut ffi::TTInstance,
        channel_id: ChannelId,
        local_path: &str,
    ) -> i32;
    fn do_recv_file(
        &self,
        ptr: *mut ffi::TTInstance,
        channel_id: ChannelId,
        file_id: FileId,
        local_path: &str,
    ) -> i32;
    fn do_delete_file(
        &self,
        ptr: *mut ffi::TTInstance,
        channel_id: ChannelId,
        file_id: FileId,
    ) -> i32;
    fn get_file_transfer_info(
        &self,
        ptr: *mut ffi::TTInstance,
        transfer_id: TransferId,
    ) -> Option<crate::types::FileTransfer>;
    fn cancel_file_transfer(&self, ptr: *mut ffi::TTInstance, transfer_id: TransferId) -> bool;
    fn do_text_message(&self, ptr: *mut ffi::TTInstance, message: &ffi::TextMessage) -> i32;
    fn do_change_status(&self, ptr: *mut ffi::TTInstance, status_mode: i32, message: &str) -> i32;
    fn get_channel(&self, ptr: *mut ffi::TTInstance, channel_id: ChannelId) -> Option<Channel>;
    fn get_server_channels(&self, ptr: *mut ffi::TTInstance) -> Vec<Channel>;
    fn get_channel_file(
        &self,
        ptr: *mut ffi::TTInstance,
        channel_id: ChannelId,
        file_id: FileId,
    ) -> Option<RemoteFile>;
    fn get_channel_path(&self, ptr: *mut ffi::TTInstance, channel_id: ChannelId) -> Option<String>;
    fn get_channel_id_from_path(&self, ptr: *mut ffi::TTInstance, path: &str) -> ChannelId;
    fn get_my_user_id(&self, ptr: *mut ffi::TTInstance) -> UserId;
    fn get_my_user_rights(&self, ptr: *mut ffi::TTInstance) -> u32;
    fn get_user(&self, ptr: *mut ffi::TTInstance, user_id: UserId, user: &mut ffi::User) -> bool;
    fn get_user_by_id(&self, ptr: *mut ffi::TTInstance, user_id: UserId) -> Option<User>;
    fn get_user_by_username(&self, ptr: *mut ffi::TTInstance, username: &str) -> Option<User>;
    fn get_user_statistics(
        &self,
        ptr: *mut ffi::TTInstance,
        user_id: UserId,
    ) -> Option<UserStatistics>;
    fn get_server_users(&self, ptr: *mut ffi::TTInstance) -> Vec<User>;
    fn get_server_properties(&self, ptr: *mut ffi::TTInstance) -> Option<ServerProperties>;
    fn get_client_statistics(&self, ptr: *mut ffi::TTInstance) -> Option<ClientStatistics>;
    fn get_my_user_account(&self, ptr: *mut ffi::TTInstance) -> Option<UserAccount>;
    fn get_my_user_type(&self, ptr: *mut ffi::TTInstance) -> u32;
    fn get_my_user_data(&self, ptr: *mut ffi::TTInstance) -> i32;
    fn get_my_local_subscriptions(&self, ptr: *mut ffi::TTInstance) -> Subscriptions;
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
    fn do_make_channel(&self, ptr: *mut ffi::TTInstance, channel: &Channel) -> i32;
    fn do_update_channel(&self, ptr: *mut ffi::TTInstance, channel: &Channel) -> i32;
    fn do_remove_channel(&self, ptr: *mut ffi::TTInstance, channel_id: ChannelId) -> i32;
    fn do_move_user(
        &self,
        ptr: *mut ffi::TTInstance,
        user_id: UserId,
        channel_id: ChannelId,
    ) -> i32;
    fn is_channel_operator(
        &self,
        ptr: *mut ffi::TTInstance,
        user_id: UserId,
        channel_id: ChannelId,
    ) -> bool;
    fn get_root_channel_id(&self, ptr: *mut ffi::TTInstance) -> ChannelId;
    fn get_channel_users(&self, ptr: *mut ffi::TTInstance, channel_id: ChannelId) -> Vec<User>;
    fn do_list_user_accounts(&self, ptr: *mut ffi::TTInstance, index: i32, count: i32) -> i32;
    fn do_new_user_account(&self, ptr: *mut ffi::TTInstance, account: &UserAccount) -> i32;
    fn do_delete_user_account(&self, ptr: *mut ffi::TTInstance, username: &str) -> i32;
    fn do_change_nickname(&self, ptr: *mut ffi::TTInstance, nickname: &str) -> i32;
    fn do_ban_ip_address(&self, ptr: *mut ffi::TTInstance, ip: &str, ban_type: i32) -> i32;
    fn do_list_bans(
        &self,
        ptr: *mut ffi::TTInstance,
        channel_id: ChannelId,
        index: i32,
        count: i32,
    ) -> i32;
    fn do_update_server(&self, ptr: *mut ffi::TTInstance, props: &ServerProperties) -> i32;
    fn do_save_config(&self, ptr: *mut ffi::TTInstance) -> i32;
    fn do_query_server_stats(&self, ptr: *mut ffi::TTInstance) -> i32;
    fn do_ping(&self, ptr: *mut ffi::TTInstance) -> i32;
    fn query_max_payload(&self, ptr: *mut ffi::TTInstance, user_id: UserId) -> bool;
    fn do_quit(&self, ptr: *mut ffi::TTInstance) -> i32;
}

#[cfg(not(feature = "mock"))]
pub(crate) trait TeamTalkBackend: Send + Sync {
    fn init_poll(&self) -> *mut ffi::TTInstance;
    #[cfg(windows)]
    fn init_hwnd(&self, hwnd: ffi::HWND, msg: u32) -> *mut ffi::TTInstance;
    fn get_message(
        &self,
        ptr: *mut ffi::TTInstance,
        msg: &mut ffi::TTMessage,
        timeout_ms: &i32,
    ) -> bool;
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
        channel_id: ChannelId,
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
    fn stop_recording_channel(&self, ptr: *mut ffi::TTInstance, channel_id: ChannelId) -> bool;
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
        channel_id: ChannelId,
        password: &str,
    ) -> i32;
    fn do_leave_channel(&self, ptr: *mut ffi::TTInstance) -> i32;
    fn do_send_file(
        &self,
        ptr: *mut ffi::TTInstance,
        channel_id: ChannelId,
        local_path: &str,
    ) -> i32;
    fn do_recv_file(
        &self,
        ptr: *mut ffi::TTInstance,
        channel_id: ChannelId,
        file_id: FileId,
        local_path: &str,
    ) -> i32;
    fn do_delete_file(
        &self,
        ptr: *mut ffi::TTInstance,
        channel_id: ChannelId,
        file_id: FileId,
    ) -> i32;
    fn get_file_transfer_info(
        &self,
        ptr: *mut ffi::TTInstance,
        transfer_id: TransferId,
    ) -> Option<crate::types::FileTransfer>;
    fn cancel_file_transfer(&self, ptr: *mut ffi::TTInstance, transfer_id: TransferId) -> bool;
    fn do_text_message(&self, ptr: *mut ffi::TTInstance, message: &ffi::TextMessage) -> i32;
    fn do_change_status(&self, ptr: *mut ffi::TTInstance, status_mode: i32, message: &str) -> i32;
    fn get_channel(&self, ptr: *mut ffi::TTInstance, channel_id: ChannelId) -> Option<Channel>;
    fn get_server_channels(&self, ptr: *mut ffi::TTInstance) -> Vec<Channel>;
    fn get_channel_file(
        &self,
        ptr: *mut ffi::TTInstance,
        channel_id: ChannelId,
        file_id: FileId,
    ) -> Option<RemoteFile>;
    fn get_channel_path(&self, ptr: *mut ffi::TTInstance, channel_id: ChannelId) -> Option<String>;
    fn get_channel_id_from_path(&self, ptr: *mut ffi::TTInstance, path: &str) -> ChannelId;
    fn get_my_user_id(&self, ptr: *mut ffi::TTInstance) -> UserId;
    fn get_my_user_rights(&self, ptr: *mut ffi::TTInstance) -> u32;
    fn get_user(&self, ptr: *mut ffi::TTInstance, user_id: UserId, user: &mut ffi::User) -> bool;
    fn get_user_by_id(&self, ptr: *mut ffi::TTInstance, user_id: UserId) -> Option<User>;
    fn get_user_by_username(&self, ptr: *mut ffi::TTInstance, username: &str) -> Option<User>;
    fn get_user_statistics(
        &self,
        ptr: *mut ffi::TTInstance,
        user_id: UserId,
    ) -> Option<UserStatistics>;
    fn get_server_users(&self, ptr: *mut ffi::TTInstance) -> Vec<User>;
    fn get_server_properties(&self, ptr: *mut ffi::TTInstance) -> Option<ServerProperties>;
    fn get_client_statistics(&self, ptr: *mut ffi::TTInstance) -> Option<ClientStatistics>;
    fn get_my_user_account(&self, ptr: *mut ffi::TTInstance) -> Option<UserAccount>;
    fn get_my_user_type(&self, ptr: *mut ffi::TTInstance) -> u32;
    fn get_my_user_data(&self, ptr: *mut ffi::TTInstance) -> i32;
    fn get_my_local_subscriptions(&self, ptr: *mut ffi::TTInstance) -> Subscriptions;
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
    fn do_make_channel(&self, ptr: *mut ffi::TTInstance, channel: &Channel) -> i32;
    fn do_update_channel(&self, ptr: *mut ffi::TTInstance, channel: &Channel) -> i32;
    fn do_remove_channel(&self, ptr: *mut ffi::TTInstance, channel_id: ChannelId) -> i32;
    fn do_move_user(
        &self,
        ptr: *mut ffi::TTInstance,
        user_id: UserId,
        channel_id: ChannelId,
    ) -> i32;
    fn is_channel_operator(
        &self,
        ptr: *mut ffi::TTInstance,
        user_id: UserId,
        channel_id: ChannelId,
    ) -> bool;
    fn get_root_channel_id(&self, ptr: *mut ffi::TTInstance) -> ChannelId;
    fn get_channel_users(&self, ptr: *mut ffi::TTInstance, channel_id: ChannelId) -> Vec<User>;
    fn do_list_user_accounts(&self, ptr: *mut ffi::TTInstance, index: i32, count: i32) -> i32;
    fn do_new_user_account(&self, ptr: *mut ffi::TTInstance, account: &UserAccount) -> i32;
    fn do_delete_user_account(&self, ptr: *mut ffi::TTInstance, username: &str) -> i32;
    fn do_change_nickname(&self, ptr: *mut ffi::TTInstance, nickname: &str) -> i32;
    fn do_ban_ip_address(&self, ptr: *mut ffi::TTInstance, ip: &str, ban_type: i32) -> i32;
    fn do_list_bans(
        &self,
        ptr: *mut ffi::TTInstance,
        channel_id: ChannelId,
        index: i32,
        count: i32,
    ) -> i32;
    fn do_update_server(&self, ptr: *mut ffi::TTInstance, props: &ServerProperties) -> i32;
    fn do_save_config(&self, ptr: *mut ffi::TTInstance) -> i32;
    fn do_query_server_stats(&self, ptr: *mut ffi::TTInstance) -> i32;
    fn do_ping(&self, ptr: *mut ffi::TTInstance) -> i32;
    fn query_max_payload(&self, ptr: *mut ffi::TTInstance, user_id: UserId) -> bool;
    fn do_quit(&self, ptr: *mut ffi::TTInstance) -> i32;
}

pub(crate) struct FfiBackend;

#[cfg(feature = "mock")]
impl sealed::Sealed for FfiBackend {}

impl TeamTalkBackend for FfiBackend {
    fn init_poll(&self) -> *mut ffi::TTInstance {
        unsafe { ffi::api().TT_InitTeamTalkPoll() }
    }

    #[cfg(windows)]
    fn init_hwnd(&self, hwnd: ffi::HWND, msg: u32) -> *mut ffi::TTInstance {
        unsafe { ffi::api().TT_InitTeamTalk(hwnd, msg) }
    }

    fn get_message(
        &self,
        ptr: *mut ffi::TTInstance,
        msg: &mut ffi::TTMessage,
        timeout_ms: &i32,
    ) -> bool {
        unsafe { ffi::api().TT_GetMessage(ptr, msg, timeout_ms) == 1 }
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
            ffi::api().TT_StartRecordingMuxedAudioFile(
                ptr,
                &raw const raw_codec,
                p.as_ptr(),
                format,
            ) == 1
        }
    }

    fn start_recording_channel(
        &self,
        ptr: *mut ffi::TTInstance,
        channel_id: ChannelId,
        file_path: &str,
        format: ffi::AudioFileFormat,
    ) -> bool {
        let p = file_path.tt();
        unsafe {
            ffi::api().TT_StartRecordingMuxedAudioFileEx(ptr, channel_id.raw(), p.as_ptr(), format)
                == 1
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
                &raw const raw_codec,
                p.as_ptr(),
                format,
            ) == 1
        }
    }

    fn stop_recording(&self, ptr: *mut ffi::TTInstance) -> bool {
        unsafe { ffi::api().TT_StopRecordingMuxedAudioFile(ptr) == 1 }
    }

    fn stop_recording_channel(&self, ptr: *mut ffi::TTInstance, channel_id: ChannelId) -> bool {
        unsafe { ffi::api().TT_StopRecordingMuxedAudioFileEx(ptr, channel_id.raw()) == 1 }
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
        channel_id: ChannelId,
        password: &str,
    ) -> i32 {
        unsafe { ffi::api().TT_DoJoinChannelByID(ptr, channel_id.raw(), password.tt().as_ptr()) }
    }

    fn do_leave_channel(&self, ptr: *mut ffi::TTInstance) -> i32 {
        unsafe { ffi::api().TT_DoLeaveChannel(ptr) }
    }

    fn do_send_file(
        &self,
        ptr: *mut ffi::TTInstance,
        channel_id: ChannelId,
        local_path: &str,
    ) -> i32 {
        unsafe { ffi::api().TT_DoSendFile(ptr, channel_id.raw(), local_path.tt().as_ptr()) }
    }

    fn do_recv_file(
        &self,
        ptr: *mut ffi::TTInstance,
        channel_id: ChannelId,
        file_id: FileId,
        local_path: &str,
    ) -> i32 {
        unsafe {
            ffi::api().TT_DoRecvFile(
                ptr,
                channel_id.raw(),
                file_id.raw(),
                local_path.tt().as_ptr(),
            )
        }
    }

    fn do_delete_file(
        &self,
        ptr: *mut ffi::TTInstance,
        channel_id: ChannelId,
        file_id: FileId,
    ) -> i32 {
        unsafe { ffi::api().TT_DoDeleteFile(ptr, channel_id.raw(), file_id.raw()) }
    }

    fn get_file_transfer_info(
        &self,
        ptr: *mut ffi::TTInstance,
        transfer_id: TransferId,
    ) -> Option<crate::types::FileTransfer> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::FileTransfer>() };
        if unsafe { ffi::api().TT_GetFileTransferInfo(ptr, transfer_id.raw(), &raw mut raw) } == 1 {
            Some(crate::types::FileTransfer::from(raw))
        } else {
            None
        }
    }

    fn cancel_file_transfer(&self, ptr: *mut ffi::TTInstance, transfer_id: TransferId) -> bool {
        unsafe { ffi::api().TT_CancelFileTransfer(ptr, transfer_id.raw()) == 1 }
    }

    fn do_text_message(&self, ptr: *mut ffi::TTInstance, message: &ffi::TextMessage) -> i32 {
        unsafe { ffi::api().TT_DoTextMessage(ptr, message) }
    }

    fn do_change_status(&self, ptr: *mut ffi::TTInstance, status_mode: i32, message: &str) -> i32 {
        unsafe { ffi::api().TT_DoChangeStatus(ptr, status_mode, message.tt().as_ptr()) }
    }

    fn get_channel(&self, ptr: *mut ffi::TTInstance, channel_id: ChannelId) -> Option<Channel> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::Channel>() };
        if unsafe { ffi::api().TT_GetChannel(ptr, channel_id.raw(), &raw mut raw) } == 1 {
            Some(Channel::from(raw))
        } else {
            None
        }
    }

    fn get_server_channels(&self, ptr: *mut ffi::TTInstance) -> Vec<Channel> {
        let mut count: i32 = 0;
        unsafe {
            ffi::api().TT_GetServerChannels(ptr, std::ptr::null_mut(), &raw mut count);
            let mut channels = vec![std::mem::zeroed::<ffi::Channel>(); count as usize];
            if ffi::api().TT_GetServerChannels(ptr, channels.as_mut_ptr(), &raw mut count) == 1 {
                channels.into_iter().map(Channel::from).collect()
            } else {
                vec![]
            }
        }
    }

    fn get_channel_file(
        &self,
        ptr: *mut ffi::TTInstance,
        channel_id: ChannelId,
        file_id: FileId,
    ) -> Option<RemoteFile> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::RemoteFile>() };
        if unsafe {
            ffi::api().TT_GetChannelFile(ptr, channel_id.raw(), file_id.raw(), &raw mut raw)
        } == 1
        {
            Some(RemoteFile::from(raw))
        } else {
            None
        }
    }

    fn get_channel_path(&self, ptr: *mut ffi::TTInstance, channel_id: ChannelId) -> Option<String> {
        use crate::types::TT_STRLEN;
        use crate::utils::strings::tt_buf;
        let mut buf = tt_buf::<TT_STRLEN>();
        unsafe {
            if ffi::api().TT_GetChannelPath(ptr, channel_id.raw(), buf.as_mut_ptr()) == 1 {
                Some(crate::utils::strings::to_string(&buf))
            } else {
                None
            }
        }
    }

    fn get_channel_id_from_path(&self, ptr: *mut ffi::TTInstance, path: &str) -> ChannelId {
        ChannelId(unsafe { ffi::api().TT_GetChannelIDFromPath(ptr, path.tt().as_ptr()) })
    }

    fn get_my_user_id(&self, ptr: *mut ffi::TTInstance) -> UserId {
        UserId(unsafe { ffi::api().TT_GetMyUserID(ptr) })
    }

    fn get_my_user_rights(&self, ptr: *mut ffi::TTInstance) -> u32 {
        unsafe { ffi::api().TT_GetMyUserRights(ptr) }
    }

    fn get_user(&self, ptr: *mut ffi::TTInstance, user_id: UserId, user: &mut ffi::User) -> bool {
        unsafe { ffi::api().TT_GetUser(ptr, user_id.raw(), user) == 1 }
    }

    fn get_user_by_id(&self, ptr: *mut ffi::TTInstance, user_id: UserId) -> Option<User> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::User>() };
        if unsafe { ffi::api().TT_GetUser(ptr, user_id.raw(), &raw mut raw) } == 1 {
            Some(User::from(raw))
        } else {
            None
        }
    }

    fn get_user_by_username(&self, ptr: *mut ffi::TTInstance, username: &str) -> Option<User> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::User>() };
        if unsafe { ffi::api().TT_GetUserByUsername(ptr, username.tt().as_ptr(), &raw mut raw) }
            == 1
        {
            Some(User::from(raw))
        } else {
            None
        }
    }

    fn get_user_statistics(
        &self,
        ptr: *mut ffi::TTInstance,
        user_id: UserId,
    ) -> Option<UserStatistics> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::UserStatistics>() };
        if unsafe { ffi::api().TT_GetUserStatistics(ptr, user_id.raw(), &raw mut raw) } == 1 {
            Some(UserStatistics::from(raw))
        } else {
            None
        }
    }

    fn get_server_users(&self, ptr: *mut ffi::TTInstance) -> Vec<User> {
        let mut count: i32 = 0;
        unsafe {
            ffi::api().TT_GetServerUsers(ptr, std::ptr::null_mut(), &raw mut count);
            let mut users = vec![std::mem::zeroed::<ffi::User>(); count as usize];
            if ffi::api().TT_GetServerUsers(ptr, users.as_mut_ptr(), &raw mut count) == 1 {
                users.into_iter().map(User::from).collect()
            } else {
                vec![]
            }
        }
    }

    fn get_server_properties(&self, ptr: *mut ffi::TTInstance) -> Option<ServerProperties> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::ServerProperties>() };
        if unsafe { ffi::api().TT_GetServerProperties(ptr, &raw mut raw) } == 1 {
            Some(ServerProperties::from(raw))
        } else {
            None
        }
    }

    fn get_client_statistics(&self, ptr: *mut ffi::TTInstance) -> Option<ClientStatistics> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::ClientStatistics>() };
        if unsafe { ffi::api().TT_GetClientStatistics(ptr, &raw mut raw) } == 1 {
            Some(ClientStatistics::from(raw))
        } else {
            None
        }
    }

    fn get_my_user_account(&self, ptr: *mut ffi::TTInstance) -> Option<UserAccount> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::UserAccount>() };
        if unsafe { ffi::api().TT_GetMyUserAccount(ptr, &raw mut raw) } == 1 {
            Some(UserAccount::from(raw))
        } else {
            None
        }
    }

    fn get_my_user_type(&self, ptr: *mut ffi::TTInstance) -> u32 {
        unsafe { ffi::api().TT_GetMyUserType(ptr) }
    }

    fn get_my_user_data(&self, ptr: *mut ffi::TTInstance) -> i32 {
        unsafe { ffi::api().TT_GetMyUserData(ptr) }
    }

    fn get_my_local_subscriptions(&self, ptr: *mut ffi::TTInstance) -> Subscriptions {
        let my_id = self.get_my_user_id(ptr);
        let mut user = unsafe { std::mem::zeroed::<ffi::User>() };
        if self.get_user(ptr, my_id, &mut user) {
            Subscriptions::from_raw(user.uLocalSubscriptions)
        } else {
            Subscriptions::new()
        }
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
                i32::from(encrypted),
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
                i32::from(encrypted),
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
                i32::from(encrypted),
            ) == 1
        }
    }

    fn disconnect(&self, ptr: *mut ffi::TTInstance) -> bool {
        unsafe { ffi::api().TT_Disconnect(ptr) == 1 }
    }

    fn get_flags(&self, ptr: *mut ffi::TTInstance) -> u32 {
        unsafe { ffi::api().TT_GetFlags(ptr) }
    }

    fn do_make_channel(&self, ptr: *mut ffi::TTInstance, channel: &Channel) -> i32 {
        unsafe { ffi::api().TT_DoMakeChannel(ptr, &channel.to_ffi()) }
    }

    fn do_update_channel(&self, ptr: *mut ffi::TTInstance, channel: &Channel) -> i32 {
        unsafe { ffi::api().TT_DoUpdateChannel(ptr, &channel.to_ffi()) }
    }

    fn do_remove_channel(&self, ptr: *mut ffi::TTInstance, channel_id: ChannelId) -> i32 {
        unsafe { ffi::api().TT_DoRemoveChannel(ptr, channel_id.raw()) }
    }

    fn do_move_user(
        &self,
        ptr: *mut ffi::TTInstance,
        user_id: UserId,
        channel_id: ChannelId,
    ) -> i32 {
        unsafe { ffi::api().TT_DoMoveUser(ptr, user_id.raw(), channel_id.raw()) }
    }

    fn is_channel_operator(
        &self,
        ptr: *mut ffi::TTInstance,
        user_id: UserId,
        channel_id: ChannelId,
    ) -> bool {
        unsafe { ffi::api().TT_IsChannelOperator(ptr, user_id.raw(), channel_id.raw()) == 1 }
    }

    fn get_root_channel_id(&self, ptr: *mut ffi::TTInstance) -> ChannelId {
        ChannelId(unsafe { ffi::api().TT_GetRootChannelID(ptr) })
    }

    fn get_channel_users(&self, ptr: *mut ffi::TTInstance, channel_id: ChannelId) -> Vec<User> {
        let mut count: i32 = 0;
        unsafe {
            ffi::api().TT_GetChannelUsers(
                ptr,
                channel_id.raw(),
                std::ptr::null_mut(),
                &raw mut count,
            );
            let mut users = vec![std::mem::zeroed::<ffi::User>(); count as usize];
            if ffi::api().TT_GetChannelUsers(
                ptr,
                channel_id.raw(),
                users.as_mut_ptr(),
                &raw mut count,
            ) == 1
            {
                users.into_iter().map(User::from).collect()
            } else {
                vec![]
            }
        }
    }

    fn do_list_user_accounts(&self, ptr: *mut ffi::TTInstance, index: i32, count: i32) -> i32 {
        unsafe { ffi::api().TT_DoListUserAccounts(ptr, index, count) }
    }

    fn do_new_user_account(&self, ptr: *mut ffi::TTInstance, account: &UserAccount) -> i32 {
        unsafe { ffi::api().TT_DoNewUserAccount(ptr, &account.to_ffi()) }
    }

    fn do_delete_user_account(&self, ptr: *mut ffi::TTInstance, username: &str) -> i32 {
        unsafe { ffi::api().TT_DoDeleteUserAccount(ptr, username.tt().as_ptr()) }
    }

    fn do_change_nickname(&self, ptr: *mut ffi::TTInstance, nickname: &str) -> i32 {
        unsafe { ffi::api().TT_DoChangeNickname(ptr, nickname.tt().as_ptr()) }
    }

    fn do_ban_ip_address(&self, ptr: *mut ffi::TTInstance, ip: &str, ban_type: i32) -> i32 {
        unsafe { ffi::api().TT_DoBanIPAddress(ptr, ip.tt().as_ptr(), ban_type) }
    }

    fn do_list_bans(
        &self,
        ptr: *mut ffi::TTInstance,
        channel_id: ChannelId,
        index: i32,
        count: i32,
    ) -> i32 {
        unsafe { ffi::api().TT_DoListBans(ptr, channel_id.raw(), index, count) }
    }

    fn do_update_server(&self, ptr: *mut ffi::TTInstance, props: &ServerProperties) -> i32 {
        unsafe { ffi::api().TT_DoUpdateServer(ptr, &props.to_ffi()) }
    }

    fn do_save_config(&self, ptr: *mut ffi::TTInstance) -> i32 {
        unsafe { ffi::api().TT_DoSaveConfig(ptr) }
    }

    fn do_query_server_stats(&self, ptr: *mut ffi::TTInstance) -> i32 {
        unsafe { ffi::api().TT_DoQueryServerStats(ptr) }
    }

    fn do_ping(&self, ptr: *mut ffi::TTInstance) -> i32 {
        unsafe { ffi::api().TT_DoPing(ptr) }
    }

    fn query_max_payload(&self, ptr: *mut ffi::TTInstance, user_id: UserId) -> bool {
        unsafe { ffi::api().TT_QueryMaxPayload(ptr, user_id.raw()) == 1 }
    }

    fn do_quit(&self, ptr: *mut ffi::TTInstance) -> i32 {
        unsafe { ffi::api().TT_DoQuit(ptr) }
    }
}

#[cfg(feature = "mock")]
#[path = "backend_mock.rs"]
mod mock;
#[cfg(feature = "mock")]
pub use mock::MockBackend;
