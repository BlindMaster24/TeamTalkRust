use super::{Client, ConnectionState, Event, Message, ffi};

impl Drop for Client {
    fn drop(&mut self) {
        self.backend.close(self.ptr.0);
    }
}

impl Client {
    /// Returns the raw TeamTalk instance pointer.
    pub fn raw_ptr(&self) -> *mut ffi::TTInstance {
        self.ptr.0
    }

    /// Returns the SDK version string.
    pub fn version() -> String {
        let _ = crate::init();
        unsafe {
            let ptr = ffi::api().TT_GetVersion();
            if ptr.is_null() {
                "Unknown".to_string()
            } else {
                crate::utils::strings::from_tt(ptr)
            }
        }
    }

    /// Polls the client for the next event.
    pub fn poll(&self, timeout_ms: i32) -> Option<(Event, Message)> {
        let mut msg = unsafe { std::mem::zeroed::<ffi::TTMessage>() };
        let t = timeout_ms;
        if self.backend().get_message(self.ptr.0, &mut msg, &t) {
            let event = Event::from(msg.nClientEvent);
            let message = Message::from_raw(event, msg);
            self.update_state_for_event(event, &message);
            #[cfg(feature = "state")]
            self.update_cache_for_event(event, &message);
            self.invoke_hooks(event, &message);
            self.dispatch_bus(event, &message);
            #[cfg(feature = "scripts")]
            self.dispatch_scripts(event, &message);
            #[cfg(feature = "logging")]
            tracing::trace!(
                event = ?event,
                source = message.source(),
                state = ?self.connection_state(),
                "teamtalk poll event"
            );
            self.handle_auto_reconnect();
            Some((event, message))
        } else {
            self.handle_auto_reconnect();
            None
        }
    }

    /// Polls until the predicate matches or the timeout expires.
    pub fn poll_until<F>(&self, timeout_ms: i32, mut predicate: F) -> Option<(Event, Message)>
    where
        F: FnMut(Event, &Message) -> bool,
    {
        use std::time::{Duration, Instant};
        if timeout_ms < 0 {
            loop {
                if let Some((event, msg)) = self.poll(timeout_ms)
                    && predicate(event, &msg)
                {
                    return Some((event, msg));
                }
            }
        }

        let deadline = Instant::now() + Duration::from_millis(timeout_ms as u64);
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return None;
            }
            let wait_ms = remaining.as_millis().min(i32::MAX as u128) as i32;
            if let Some((event, msg)) = self.poll(wait_ms)
                && predicate(event, &msg)
            {
                return Some((event, msg));
            }
        }
    }

    /// Polls until a specific event arrives or the timeout expires.
    pub fn wait_for(&self, event: Event, timeout_ms: i32) -> Option<Message> {
        self.poll_until(timeout_ms, |incoming, _| incoming == event)
            .map(|(_, msg)| msg)
    }

    /// Polls until the predicate matches or the timeout expires.
    pub fn wait_for_predicate<F>(&self, timeout_ms: i32, predicate: F) -> Option<(Event, Message)>
    where
        F: FnMut(Event, &Message) -> bool,
    {
        self.poll_until(timeout_ms, predicate)
    }

    /// Polls until an event carries typed payload data or the timeout expires.
    pub fn wait_for_data(&self, timeout_ms: i32) -> Option<(Event, Message, super::EventData)> {
        self.poll_until(timeout_ms, |_, msg| msg.data().is_some())
            .and_then(|(event, msg)| msg.data().map(|data| (event, msg, data)))
    }

    /// Polls until a specific event arrives or the timeout expires.
    pub fn poll_until_event(&self, event: Event, timeout_ms: i32) -> Option<Message> {
        self.wait_for(event, timeout_ms)
    }

    pub(super) fn update_state_for_event(&self, event: Event, msg: &Message) {
        #[cfg(feature = "logging")]
        let prev_state = self.connection_state();
        match event {
            Event::ConnectSuccess => {
                self.set_connection_state(ConnectionState::Connected);
                let mut auto = self.auto_reconnect.lock();

                auto.clear_connect_phase();
                if let Some(handler) = auto.handler.as_mut() {
                    handler.mark_connected();
                }
                auto.login_gave_up = false;
                auto.join_gave_up = false;
                if auto.enabled {
                    let msg =
                        Message::from_raw(event, unsafe { std::mem::zeroed::<ffi::TTMessage>() });
                    let attempts = auto.handler.as_ref().map(|h| h.attempts()).unwrap_or(0);
                    drop(auto);
                    if attempts > 0 {
                        self.invoke_hooks(Event::AfterReconnect { attempt: attempts }, &msg);
                    }
                } else {
                    drop(auto);
                }
            }
            Event::ConnectFailed | Event::ConnectionLost | Event::ConnectCryptError => {
                self.set_connection_state(ConnectionState::Disconnected);
                let mut auto = self.auto_reconnect.lock();

                auto.clear_phase_tracking();
                if let Some(handler) = auto.handler.as_mut() {
                    handler.mark_disconnected();
                }
            }
            Event::MySelfLoggedIn => {
                self.set_connection_state(ConnectionState::LoggedIn);
                let mut auto = self.auto_reconnect.lock();

                auto.clear_login_phase();
                if let Some(handler) = auto.login_handler.as_mut() {
                    handler.mark_connected();
                }
            }
            Event::MySelfLoggedOut => {
                self.set_connection_state(ConnectionState::Connected);
                let mut auto = self.auto_reconnect.lock();

                auto.clear_join_phase();
                if let Some(handler) = auto.join_handler.as_mut() {
                    handler.mark_disconnected();
                }
            }
            Event::UserJoined => {
                if let Some(user) = msg.user()
                    && user.id == self.my_id()
                {
                    self.set_connection_state(ConnectionState::Joined(user.channel_id));
                    let mut auto = self.auto_reconnect.lock();

                    auto.clear_join_phase();
                    if let Some(handler) = auto.join_handler.as_mut() {
                        handler.mark_connected();
                    }
                    self.invoke_joined_hook(user.channel_id);
                }
            }
            Event::UserLeft => {
                if let Some(user) = msg.user()
                    && user.id == self.my_id()
                {
                    self.set_connection_state(ConnectionState::LoggedIn);
                    let mut auto = self.auto_reconnect.lock();

                    auto.clear_join_phase();
                    if let Some(handler) = auto.join_handler.as_mut() {
                        handler.mark_disconnected();
                    }
                }
            }
            Event::MySelfKicked => {
                let next_state = kicked_next_state(msg.source());
                self.set_connection_state(next_state);
                let mut auto = self.auto_reconnect.lock();

                auto.clear_join_phase();
                if matches!(next_state, ConnectionState::Connected) {
                    auto.clear_login_phase();
                }
                if let Some(handler) = auto.join_handler.as_mut() {
                    handler.mark_disconnected();
                }
                if matches!(next_state, ConnectionState::Connected)
                    && let Some(handler) = auto.login_handler.as_mut()
                {
                    handler.mark_disconnected();
                }
            }
            Event::CmdError => {
                let source = msg.source();
                let mut next_state = None;
                let mut auto = self.auto_reconnect.lock();

                if auto.pending_login_cmd == Some(source) {
                    auto.clear_login_phase();
                    if let Some(handler) = auto.login_handler.as_mut() {
                        handler.mark_disconnected();
                    }
                    next_state = Some(ConnectionState::Connected);
                } else if auto.pending_join_cmd == Some(source) {
                    auto.clear_join_phase();
                    if let Some(handler) = auto.join_handler.as_mut() {
                        handler.mark_disconnected();
                    }
                    next_state = Some(ConnectionState::LoggedIn);
                }
                drop(auto);
                if let Some(state) = next_state {
                    self.set_connection_state(state);
                }
            }
            Event::CmdSuccess => {
                let source = msg.source();
                let mut auto = self.auto_reconnect.lock();

                if auto.pending_login_cmd == Some(source) {
                    auto.clear_login_phase();
                }
                if auto.pending_join_cmd == Some(source) {
                    auto.clear_join_phase();
                }
            }
            _ => {}
        }

        let mut auto = self.auto_reconnect.lock();

        if auto.enabled && event.is_reconnect_needed_with(&auto.extra_events) {
            auto.force_disconnect = true;
            drop(auto);
            self.set_connection_state(ConnectionState::Disconnected);
        }
        #[cfg(feature = "logging")]
        {
            let next_state = self.connection_state();
            if next_state != prev_state {
                tracing::debug!(
                    event = ?event,
                    previous = ?prev_state,
                    current = ?next_state,
                    "connection state transition"
                );
            }
        }
    }

    /// Returns the current client flags.
    pub fn get_flags(&self) -> crate::types::ClientFlags {
        crate::types::ClientFlags::from_raw(self.backend().get_flags(self.ptr.0))
    }

    /// Returns a human-readable error message for a TeamTalk error code.
    pub fn get_error_message(&self, code: i32) -> String {
        use crate::types::TT_STRLEN;
        use crate::utils::strings::tt_buf;
        let mut buf = tt_buf::<TT_STRLEN>();
        unsafe {
            ffi::api().TT_GetErrorMessage(code, buf.as_mut_ptr());
            crate::utils::strings::to_string(&buf)
        }
    }

    /// Builds a typed SDK error with the resolved message.
    pub fn client_error(&self, code: i32) -> crate::events::Error {
        crate::events::Error::ClientError {
            code,
            message: self.get_error_message(code),
        }
    }
}

fn kicked_next_state(source: i32) -> ConnectionState {
    if source > 0 {
        ConnectionState::LoggedIn
    } else {
        ConnectionState::Connected
    }
}
