use super::*;

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
    my_user_id: i32,
    user: Option<ffi::User>,
    start_ok: bool,
    stop_ok: bool,
    login_result: i32,
    logout_result: i32,
    join_result: i32,
    leave_result: i32,
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

    pub fn set_my_channel_id(&self, channel_id: ChannelId) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.my_channel_id = channel_id;
    }

    pub fn set_my_user_id(&self, user_id: i32) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.my_user_id = user_id;
    }

    pub fn set_user(&self, user: ffi::User) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.user = Some(user);
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
}

#[cfg(feature = "mock")]
impl TeamTalkBackend for MockBackend {
    fn init_poll(&self) -> *mut ffi::TTInstance {
        std::ptr::dangling_mut()
    }

    #[cfg(windows)]
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

    fn get_my_user_id(&self, _ptr: *mut ffi::TTInstance) -> i32 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .my_user_id
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
}
