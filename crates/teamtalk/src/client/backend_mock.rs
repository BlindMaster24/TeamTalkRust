use super::*;

#[cfg(feature = "mock")]
#[derive(Default)]
pub struct MockBackend {
    state: std::sync::Mutex<MockBackendState>,
}

#[cfg(feature = "mock")]
#[derive(Default)]
struct MockBackendState {
    queued_messages: std::collections::VecDeque<ffi::TTMessage>,
    channels: std::collections::HashMap<i32, Channel>,
    channel_files: std::collections::HashMap<(i32, i32), ffi::RemoteFile>,
    channel_paths: std::collections::HashMap<i32, String>,
    channel_users: std::collections::HashMap<i32, Vec<ffi::User>>,
    root_channel_id: ChannelId,
    my_channel_id: ChannelId,
    my_user_id: i32,
    my_user_rights: u32,
    my_user_account: Option<ffi::UserAccount>,
    my_user_type: u32,
    my_user_data: i32,
    user: Option<ffi::User>,
    users_by_id: std::collections::HashMap<i32, ffi::User>,
    users_by_username: std::collections::HashMap<String, ffi::User>,
    user_stats: std::collections::HashMap<i32, ffi::UserStatistics>,
    server_properties: Option<ffi::ServerProperties>,
    client_statistics: Option<ffi::ClientStatistics>,
    server_users: Vec<ffi::User>,
    start_ok: bool,
    stop_ok: bool,
    login_result: i32,
    logout_result: i32,
    join_result: i32,
    leave_result: i32,
    send_file_result: i32,
    recv_file_result: i32,
    delete_file_result: i32,
    make_channel_result: i32,
    update_channel_result: i32,
    remove_channel_result: i32,
    move_user_result: i32,
    list_user_accounts_result: i32,
    new_user_account_result: i32,
    delete_user_account_result: i32,
    change_nickname_result: i32,
    ban_ip_result: i32,
    list_bans_result: i32,
    update_server_result: i32,
    save_config_result: i32,
    query_server_stats_result: i32,
    ping_result: i32,
    query_max_payload_ok: bool,
    quit_result: i32,
    file_transfers: std::collections::HashMap<i32, ffi::FileTransfer>,
    cancelled_transfers: Vec<i32>,
    last_login: Option<(String, String, String, String)>,
    last_text_message: Option<ffi::TextMessage>,
    text_messages: Vec<ffi::TextMessage>,
    text_message_results: std::collections::VecDeque<i32>,
    last_status: Option<(i32, String)>,
    flags: u32,
    connect_ok: bool,
    disconnect_ok: bool,
    call_log: Vec<&'static str>,
}

#[cfg(feature = "mock")]
impl MockBackend {
    pub fn new() -> Self {
        Self {
            state: std::sync::Mutex::new(MockBackendState {
                start_ok: true,
                stop_ok: true,
                login_result: 1,
                logout_result: 1,
                join_result: 1,
                leave_result: 1,
                send_file_result: 1,
                recv_file_result: 1,
                delete_file_result: 1,
                make_channel_result: 1,
                update_channel_result: 1,
                remove_channel_result: 1,
                move_user_result: 1,
                list_user_accounts_result: 1,
                new_user_account_result: 1,
                delete_user_account_result: 1,
                change_nickname_result: 1,
                ban_ip_result: 1,
                list_bans_result: 1,
                update_server_result: 1,
                save_config_result: 1,
                query_server_stats_result: 1,
                ping_result: 1,
                query_max_payload_ok: true,
                quit_result: 1,
                connect_ok: true,
                disconnect_ok: true,
                ..MockBackendState::default()
            }),
        }
    }

    pub fn set_channel(&self, channel: Channel) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.channels.insert(channel.id.0, channel);
    }

    pub fn set_channel_path(&self, channel_id: ChannelId, path: &str) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.channel_paths.insert(channel_id.0, path.to_string());
    }

    pub fn set_channel_file(&self, file: ffi::RemoteFile) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state
            .channel_files
            .insert((file.nChannelID, file.nFileID), file);
    }

    pub fn set_channel_users(&self, channel_id: ChannelId, users: Vec<ffi::User>) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.channel_users.insert(channel_id.0, users);
    }

    pub fn set_root_channel_id(&self, channel_id: ChannelId) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.root_channel_id = channel_id;
    }

    pub fn set_my_channel_id(&self, channel_id: ChannelId) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.my_channel_id = channel_id;
    }

    pub fn set_my_user_id(&self, user_id: i32) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.my_user_id = user_id;
    }

    pub fn set_my_user_rights(&self, rights: u32) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.my_user_rights = rights;
    }

    pub fn set_my_user_account(&self, account: ffi::UserAccount) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.my_user_account = Some(account);
    }

    pub fn set_my_user_type(&self, user_type: u32) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.my_user_type = user_type;
    }

    pub fn set_my_user_data(&self, user_data: i32) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.my_user_data = user_data;
    }

    pub fn set_user(&self, user: ffi::User) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.user = Some(user);
    }

    pub fn set_user_by_id(&self, user_id: i32, user: ffi::User) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.users_by_id.insert(user_id, user);
    }

    pub fn set_user_by_username(&self, username: &str, user: ffi::User) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.users_by_username.insert(username.to_string(), user);
    }

    pub fn set_user_statistics(&self, user_id: i32, statistics: ffi::UserStatistics) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.user_stats.insert(user_id, statistics);
    }

    pub fn set_server_properties(&self, properties: ffi::ServerProperties) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.server_properties = Some(properties);
    }

    pub fn set_client_statistics(&self, statistics: ffi::ClientStatistics) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.client_statistics = Some(statistics);
    }

    pub fn set_server_users(&self, users: Vec<ffi::User>) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.server_users = users;
    }

    pub fn set_start_ok(&self, ok: bool) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.start_ok = ok;
    }

    pub fn set_stop_ok(&self, ok: bool) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.stop_ok = ok;
    }

    pub fn set_login_result(&self, cmd_id: i32) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.login_result = cmd_id;
    }

    pub fn set_logout_result(&self, cmd_id: i32) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.logout_result = cmd_id;
    }

    pub fn set_join_result(&self, cmd_id: i32) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.join_result = cmd_id;
    }

    pub fn set_leave_result(&self, cmd_id: i32) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.leave_result = cmd_id;
    }

    pub fn set_send_file_result(&self, cmd_id: i32) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.send_file_result = cmd_id;
    }

    pub fn set_recv_file_result(&self, cmd_id: i32) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.recv_file_result = cmd_id;
    }

    pub fn set_delete_file_result(&self, cmd_id: i32) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.delete_file_result = cmd_id;
    }

    pub fn set_list_user_accounts_result(&self, cmd_id: i32) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.list_user_accounts_result = cmd_id;
    }

    pub fn set_new_user_account_result(&self, cmd_id: i32) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.new_user_account_result = cmd_id;
    }

    pub fn set_delete_user_account_result(&self, cmd_id: i32) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.delete_user_account_result = cmd_id;
    }

    pub fn set_list_bans_result(&self, cmd_id: i32) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.list_bans_result = cmd_id;
    }

    pub fn set_update_server_result(&self, cmd_id: i32) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.update_server_result = cmd_id;
    }

    pub fn set_save_config_result(&self, cmd_id: i32) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.save_config_result = cmd_id;
    }

    pub fn last_login(&self) -> Option<(String, String, String, String)> {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .last_login
            .clone()
    }

    pub fn last_text_message(&self) -> Option<ffi::TextMessage> {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .last_text_message
    }

    pub fn text_messages(&self) -> Vec<ffi::TextMessage> {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .text_messages
            .clone()
    }

    pub fn set_text_message_results(&self, results: impl IntoIterator<Item = i32>) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.text_message_results = results.into_iter().collect();
    }

    pub fn last_status(&self) -> Option<(i32, String)> {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .last_status
            .clone()
    }

    pub fn set_flags(&self, flags: u32) {
        self.state.lock().unwrap_or_else(|e| e.into_inner()).flags = flags;
    }

    pub fn set_connect_ok(&self, ok: bool) {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .connect_ok = ok;
    }

    pub fn set_disconnect_ok(&self, ok: bool) {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .disconnect_ok = ok;
    }

    pub fn call_log(&self) -> Vec<&'static str> {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .call_log
            .clone()
    }

    pub fn set_query_server_stats_result(&self, cmd_id: i32) {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .query_server_stats_result = cmd_id;
    }

    pub fn set_ping_result(&self, cmd_id: i32) {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .ping_result = cmd_id;
    }

    pub fn set_query_max_payload_ok(&self, ok: bool) {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .query_max_payload_ok = ok;
    }

    pub fn set_quit_result(&self, cmd_id: i32) {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .quit_result = cmd_id;
    }

    pub fn set_file_transfer_info(&self, transfer: ffi::FileTransfer) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.file_transfers.insert(transfer.nTransferID, transfer);
    }

    pub fn cancelled_transfers(&self) -> Vec<i32> {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .cancelled_transfers
            .clone()
    }

    pub fn push_file_transfer_event(&self, transfer: ffi::FileTransfer) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut msg = unsafe { std::mem::zeroed::<ffi::TTMessage>() };
        msg.nClientEvent = ffi::ClientEvent::CLIENTEVENT_FILETRANSFER;
        msg.ttType = ffi::TTType::__FILETRANSFER;
        msg.__bindgen_anon_1.filetransfer = transfer;
        state.queued_messages.push_back(msg);
    }

    pub fn push_server_statistics_event(&self) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut msg = unsafe { std::mem::zeroed::<ffi::TTMessage>() };
        msg.nClientEvent = ffi::ClientEvent::CLIENTEVENT_CMD_SERVERSTATISTICS;
        msg.ttType = ffi::TTType::__SERVERSTATISTICS;
        state.queued_messages.push_back(msg);
    }

    pub fn push_cmd_success_event(&self, source: i32) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut msg = unsafe { std::mem::zeroed::<ffi::TTMessage>() };
        msg.nClientEvent = ffi::ClientEvent::CLIENTEVENT_CMD_SUCCESS;
        msg.nSource = source;
        state.queued_messages.push_back(msg);
    }

    pub fn push_cmd_error_event(&self, source: i32) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut msg = unsafe { std::mem::zeroed::<ffi::TTMessage>() };
        msg.nClientEvent = ffi::ClientEvent::CLIENTEVENT_CMD_ERROR;
        msg.nSource = source;
        state.queued_messages.push_back(msg);
    }

    pub fn push_user_account_event(&self, account: ffi::UserAccount) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut msg = unsafe { std::mem::zeroed::<ffi::TTMessage>() };
        msg.nClientEvent = ffi::ClientEvent::CLIENTEVENT_CMD_USERACCOUNT;
        msg.ttType = ffi::TTType::__USERACCOUNT;
        msg.__bindgen_anon_1.useraccount = account;
        state.queued_messages.push_back(msg);
    }

    pub fn push_user_account_created_event(&self, account: ffi::UserAccount) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut msg = unsafe { std::mem::zeroed::<ffi::TTMessage>() };
        msg.nClientEvent = ffi::ClientEvent::CLIENTEVENT_CMD_USERACCOUNT_NEW;
        msg.ttType = ffi::TTType::__USERACCOUNT;
        msg.__bindgen_anon_1.useraccount = account;
        state.queued_messages.push_back(msg);
    }

    pub fn push_user_account_removed_event(&self, account: ffi::UserAccount) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut msg = unsafe { std::mem::zeroed::<ffi::TTMessage>() };
        msg.nClientEvent = ffi::ClientEvent::CLIENTEVENT_CMD_USERACCOUNT_REMOVE;
        msg.ttType = ffi::TTType::__USERACCOUNT;
        msg.__bindgen_anon_1.useraccount = account;
        state.queued_messages.push_back(msg);
    }

    pub fn push_banned_user_event(&self, banned_user: ffi::BannedUser) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut msg = unsafe { std::mem::zeroed::<ffi::TTMessage>() };
        msg.nClientEvent = ffi::ClientEvent::CLIENTEVENT_CMD_BANNEDUSER;
        msg.ttType = ffi::TTType::__BANNEDUSER;
        msg.__bindgen_anon_1.banneduser = banned_user;
        state.queued_messages.push_back(msg);
    }

    pub fn push_server_update_event(&self, properties: ffi::ServerProperties) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut msg = unsafe { std::mem::zeroed::<ffi::TTMessage>() };
        msg.nClientEvent = ffi::ClientEvent::CLIENTEVENT_CMD_SERVER_UPDATE;
        msg.ttType = ffi::TTType::__SERVERPROPERTIES;
        msg.__bindgen_anon_1.serverproperties = properties;
        state.queued_messages.push_back(msg);
    }
}

#[cfg(feature = "mock")]
impl sealed::Sealed for MockBackend {}

#[cfg(feature = "mock")]
impl TeamTalkBackend for MockBackend {
    fn init_poll(&self) -> *mut ffi::TTInstance {
        std::ptr::dangling_mut()
    }

    #[cfg(windows)]
    fn init_hwnd(&self, _hwnd: ffi::HWND, _msg: u32) -> *mut ffi::TTInstance {
        self.init_poll()
    }

    fn get_message(
        &self,
        _ptr: *mut ffi::TTInstance,
        msg: &mut ffi::TTMessage,
        _timeout_ms: &i32,
    ) -> bool {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(next) = state.queued_messages.pop_front() {
            *msg = next;
            true
        } else {
            false
        }
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
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .start_ok
    }

    fn start_recording_channel(
        &self,
        _ptr: *mut ffi::TTInstance,
        _channel_id: i32,
        _file_path: &str,
        _format: ffi::AudioFileFormat,
    ) -> bool {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .start_ok
    }

    fn start_recording_streams(
        &self,
        _ptr: *mut ffi::TTInstance,
        _stream_types: u32,
        _codec: &AudioCodec,
        _file_path: &str,
        _format: ffi::AudioFileFormat,
    ) -> bool {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .start_ok
    }

    fn stop_recording(&self, _ptr: *mut ffi::TTInstance) -> bool {
        self.state.lock().unwrap_or_else(|e| e.into_inner()).stop_ok
    }

    fn stop_recording_channel(&self, _ptr: *mut ffi::TTInstance, _channel_id: i32) -> bool {
        self.state.lock().unwrap_or_else(|e| e.into_inner()).stop_ok
    }

    fn do_login_ex(
        &self,
        _ptr: *mut ffi::TTInstance,
        nickname: &str,
        username: &str,
        password: &str,
        client_name: &str,
    ) -> i32 {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.last_login = Some((
            nickname.to_string(),
            username.to_string(),
            password.to_string(),
            client_name.to_string(),
        ));
        state.login_result
    }

    fn do_logout(&self, _ptr: *mut ffi::TTInstance) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .logout_result
    }

    fn do_join_channel_by_id(
        &self,
        _ptr: *mut ffi::TTInstance,
        _channel_id: i32,
        _password: &str,
    ) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .join_result
    }

    fn do_leave_channel(&self, _ptr: *mut ffi::TTInstance) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .leave_result
    }

    fn do_send_file(&self, _ptr: *mut ffi::TTInstance, _channel_id: i32, _local_path: &str) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .send_file_result
    }

    fn do_recv_file(
        &self,
        _ptr: *mut ffi::TTInstance,
        _channel_id: i32,
        _file_id: i32,
        _local_path: &str,
    ) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .recv_file_result
    }

    fn do_delete_file(&self, _ptr: *mut ffi::TTInstance, _channel_id: i32, _file_id: i32) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .delete_file_result
    }

    fn get_file_transfer_info(
        &self,
        _ptr: *mut ffi::TTInstance,
        transfer_id: i32,
    ) -> Option<crate::types::FileTransfer> {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .file_transfers
            .get(&transfer_id)
            .copied()
            .map(crate::types::FileTransfer::from)
    }

    fn cancel_file_transfer(&self, _ptr: *mut ffi::TTInstance, transfer_id: i32) -> bool {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.cancelled_transfers.push(transfer_id);
        true
    }

    fn do_text_message(&self, _ptr: *mut ffi::TTInstance, message: &ffi::TextMessage) -> i32 {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.last_text_message = Some(*message);
        state.text_messages.push(*message);
        state.text_message_results.pop_front().unwrap_or(1)
    }

    fn do_change_status(&self, _ptr: *mut ffi::TTInstance, status_mode: i32, message: &str) -> i32 {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.last_status = Some((status_mode, message.to_string()));
        1
    }

    fn get_channel(&self, _ptr: *mut ffi::TTInstance, channel_id: i32) -> Option<Channel> {
        let state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.channels.get(&channel_id).cloned()
    }

    fn get_server_channels(&self, _ptr: *mut ffi::TTInstance) -> Vec<Channel> {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .channels
            .values()
            .cloned()
            .collect()
    }

    fn get_channel_file(
        &self,
        _ptr: *mut ffi::TTInstance,
        channel_id: i32,
        file_id: i32,
    ) -> Option<RemoteFile> {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .channel_files
            .get(&(channel_id, file_id))
            .copied()
            .map(RemoteFile::from)
    }

    fn get_channel_path(&self, _ptr: *mut ffi::TTInstance, channel_id: i32) -> Option<String> {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .channel_paths
            .get(&channel_id)
            .cloned()
    }

    fn get_channel_id_from_path(&self, _ptr: *mut ffi::TTInstance, path: &str) -> ChannelId {
        let state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state
            .channel_paths
            .iter()
            .find_map(|(id, stored)| (stored == path).then_some(ChannelId(*id)))
            .unwrap_or_default()
    }

    fn get_my_user_id(&self, _ptr: *mut ffi::TTInstance) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .my_user_id
    }

    fn get_my_user_rights(&self, _ptr: *mut ffi::TTInstance) -> u32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .my_user_rights
    }

    fn get_user(&self, _ptr: *mut ffi::TTInstance, _user_id: i32, user: &mut ffi::User) -> bool {
        let state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(raw) = state.user {
            *user = raw;
            true
        } else {
            false
        }
    }

    fn get_user_by_id(&self, _ptr: *mut ffi::TTInstance, user_id: i32) -> Option<User> {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .users_by_id
            .get(&user_id)
            .copied()
            .map(User::from)
    }

    fn get_user_by_username(&self, _ptr: *mut ffi::TTInstance, username: &str) -> Option<User> {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .users_by_username
            .get(username)
            .copied()
            .map(User::from)
    }

    fn get_user_statistics(
        &self,
        _ptr: *mut ffi::TTInstance,
        user_id: i32,
    ) -> Option<UserStatistics> {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .user_stats
            .get(&user_id)
            .copied()
            .map(UserStatistics::from)
    }

    fn get_server_users(&self, _ptr: *mut ffi::TTInstance) -> Vec<User> {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .server_users
            .iter()
            .copied()
            .map(User::from)
            .collect()
    }

    fn get_server_properties(&self, _ptr: *mut ffi::TTInstance) -> Option<ServerProperties> {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .server_properties
            .map(ServerProperties::from)
    }

    fn get_client_statistics(&self, _ptr: *mut ffi::TTInstance) -> Option<ClientStatistics> {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .client_statistics
            .map(ClientStatistics::from)
    }

    fn get_my_user_account(&self, _ptr: *mut ffi::TTInstance) -> Option<UserAccount> {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .my_user_account
            .map(UserAccount::from)
    }

    fn get_my_user_type(&self, _ptr: *mut ffi::TTInstance) -> u32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .my_user_type
    }

    fn get_my_user_data(&self, _ptr: *mut ffi::TTInstance) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .my_user_data
    }

    fn get_my_local_subscriptions(&self, _ptr: *mut ffi::TTInstance) -> Subscriptions {
        let state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let raw = state
            .user
            .filter(|user| user.nUserID == state.my_user_id)
            .map(|user| user.uLocalSubscriptions)
            .or_else(|| {
                state
                    .users_by_id
                    .get(&state.my_user_id)
                    .map(|user| user.uLocalSubscriptions)
            })
            .unwrap_or_default();
        Subscriptions::from_raw(raw)
    }

    fn get_my_channel_id(&self, _ptr: *mut ffi::TTInstance) -> ChannelId {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .my_channel_id
    }

    fn connect(
        &self,
        _ptr: *mut ffi::TTInstance,
        _host: &str,
        _tcp: i32,
        _udp: i32,
        _encrypted: bool,
    ) -> bool {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.call_log.push("connect");
        if state.connect_ok {
            state.flags |= ffi::ClientFlag::CLIENT_CONNECTING as u32;
            true
        } else {
            false
        }
    }

    fn connect_sys_id(
        &self,
        ptr: *mut ffi::TTInstance,
        host: &str,
        tcp: i32,
        udp: i32,
        encrypted: bool,
        _sys_id: &str,
    ) -> bool {
        self.connect(ptr, host, tcp, udp, encrypted)
    }

    fn connect_ex(
        &self,
        ptr: *mut ffi::TTInstance,
        host: &str,
        tcp: i32,
        udp: i32,
        _bind_ip: &str,
        encrypted: bool,
    ) -> bool {
        self.connect(ptr, host, tcp, udp, encrypted)
    }

    fn disconnect(&self, _ptr: *mut ffi::TTInstance) -> bool {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.call_log.push("disconnect");
        if state.disconnect_ok {
            state.flags = 0;
            true
        } else {
            false
        }
    }

    fn get_flags(&self, _ptr: *mut ffi::TTInstance) -> u32 {
        self.state.lock().unwrap_or_else(|e| e.into_inner()).flags
    }

    fn do_make_channel(&self, _ptr: *mut ffi::TTInstance, _channel: &Channel) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .make_channel_result
    }

    fn do_update_channel(&self, _ptr: *mut ffi::TTInstance, _channel: &Channel) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .update_channel_result
    }

    fn do_remove_channel(&self, _ptr: *mut ffi::TTInstance, _channel_id: i32) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove_channel_result
    }

    fn do_move_user(&self, _ptr: *mut ffi::TTInstance, _user_id: i32, _channel_id: i32) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .move_user_result
    }

    fn is_channel_operator(
        &self,
        _ptr: *mut ffi::TTInstance,
        _user_id: i32,
        _channel_id: i32,
    ) -> bool {
        false
    }

    fn get_root_channel_id(&self, _ptr: *mut ffi::TTInstance) -> ChannelId {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .root_channel_id
    }

    fn get_channel_users(&self, _ptr: *mut ffi::TTInstance, channel_id: i32) -> Vec<User> {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .channel_users
            .get(&channel_id)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(User::from)
            .collect()
    }

    fn do_list_user_accounts(&self, _ptr: *mut ffi::TTInstance, _index: i32, _count: i32) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .list_user_accounts_result
    }

    fn do_new_user_account(&self, _ptr: *mut ffi::TTInstance, _account: &UserAccount) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .new_user_account_result
    }

    fn do_delete_user_account(&self, _ptr: *mut ffi::TTInstance, _username: &str) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .delete_user_account_result
    }

    fn do_change_nickname(&self, _ptr: *mut ffi::TTInstance, _nickname: &str) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .change_nickname_result
    }

    fn do_ban_ip_address(&self, _ptr: *mut ffi::TTInstance, _ip: &str, _ban_type: i32) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .ban_ip_result
    }

    fn do_list_bans(
        &self,
        _ptr: *mut ffi::TTInstance,
        _channel_id: i32,
        _index: i32,
        _count: i32,
    ) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .list_bans_result
    }

    fn do_update_server(&self, _ptr: *mut ffi::TTInstance, _props: &ServerProperties) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .update_server_result
    }

    fn do_save_config(&self, _ptr: *mut ffi::TTInstance) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .save_config_result
    }

    fn do_query_server_stats(&self, _ptr: *mut ffi::TTInstance) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .query_server_stats_result
    }

    fn do_ping(&self, _ptr: *mut ffi::TTInstance) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .ping_result
    }

    fn query_max_payload(&self, _ptr: *mut ffi::TTInstance, _user_id: i32) -> bool {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .query_max_payload_ok
    }

    fn do_quit(&self, _ptr: *mut ffi::TTInstance) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .quit_result
    }
}
