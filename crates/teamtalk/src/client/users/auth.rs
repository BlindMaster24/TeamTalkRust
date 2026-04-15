use super::{Client, CommandId, LoginParams, can_login_in_state, can_logout_in_state};
use std::time::{Duration, Instant};

fn wait_slice(deadline: Instant) -> i32 {
    deadline
        .saturating_duration_since(Instant::now())
        .min(Duration::from_millis(50))
        .as_millis()
        .min(i32::MAX as u128) as i32
}

impl Client {
    /// Logs in to the server.
    #[must_use]
    pub fn login(
        &self,
        nickname: &str,
        username: &str,
        password: &str,
        client_name: &str,
    ) -> CommandId {
        if !can_login_in_state(self.connection_state()) {
            return CommandId::ZERO;
        }
        let cmd_id = CommandId(self.backend().do_login_ex(
            self.ptr.0,
            nickname,
            username,
            password,
            client_name,
        ));
        if cmd_id.is_ok() {
            self.set_connection_state(crate::events::ConnectionState::LoggingIn);
            self.mark_login_phase_started(cmd_id.raw());
        }
        cmd_id
    }

    /// Stores login parameters for automatic login.
    pub fn set_login_params(&self, params: LoginParams) {
        let mut auto = self.auto_reconnect.lock();

        auto.login = Some(params);
        auto.login_gave_up = false;
        auto.clear_login_phase();
    }

    /// Returns stored login parameters, if any.
    pub fn login_params(&self) -> Option<LoginParams> {
        self.auto_reconnect.lock().login.clone()
    }

    /// Logs in using stored login parameters.
    pub fn login_with_params(&self) -> Result<CommandId, crate::events::Error> {
        let params = self
            .login_params()
            .ok_or(crate::events::Error::MissingLoginParams)?;
        Ok(self.login(
            &params.nickname,
            &params.username,
            &params.password,
            &params.client_name,
        ))
    }

    /// Logs in and waits for the login event or command error.
    pub fn login_and_wait(
        &self,
        nickname: &str,
        username: &str,
        password: &str,
        client_name: &str,
        timeout_ms: i32,
    ) -> Result<crate::client::Message, crate::events::Error> {
        let cmd_id = self.login(nickname, username, password, client_name);
        if !cmd_id.is_ok() {
            return Err(crate::events::Error::AuthFailed);
        }
        if timeout_ms < 0 {
            loop {
                if let Some((event, message)) = self.poll(50) {
                    match event {
                        crate::events::Event::MySelfLoggedIn => return Ok(message),
                        crate::events::Event::CmdError if cmd_id == message.command_id() => {
                            return Err(crate::events::Error::AuthFailed);
                        }
                        _ => {}
                    }
                }
                if self.connection_state() != crate::events::ConnectionState::LoggingIn {
                    return Err(crate::events::Error::AuthFailed);
                }
            }
        }

        let deadline = Instant::now() + Duration::from_millis(timeout_ms as u64);
        loop {
            let wait_ms = wait_slice(deadline);
            if wait_ms <= 0 {
                return Err(crate::events::Error::Timeout);
            }
            if let Some((event, message)) = self.poll(wait_ms) {
                match event {
                    crate::events::Event::MySelfLoggedIn => return Ok(message),
                    crate::events::Event::CmdError if cmd_id == message.command_id() => {
                        return Err(crate::events::Error::AuthFailed);
                    }
                    _ => {}
                }
            }
            if self.connection_state() != crate::events::ConnectionState::LoggingIn {
                return Err(crate::events::Error::AuthFailed);
            }
        }
    }

    /// Stores login parameters and immediately logs in.
    #[must_use]
    pub fn login_remember(
        &self,
        nickname: &str,
        username: &str,
        password: &str,
        client_name: &str,
    ) -> CommandId {
        let params = LoginParams::new(nickname, username, password, client_name);
        self.set_login_params(params);
        self.login(nickname, username, password, client_name)
    }

    #[must_use]
    pub fn login_from_env(&self) -> CommandId {
        let params = LoginParams::from_env();
        self.set_login_params(params.clone());
        self.login(
            &params.nickname,
            &params.username,
            &params.password,
            &params.client_name,
        )
    }

    /// Logs out from the server.
    #[must_use]
    pub fn logout(&self) -> CommandId {
        if !can_logout_in_state(self.connection_state()) {
            return CommandId::ZERO;
        }
        let cmd_id = CommandId(self.backend().do_logout(self.ptr.0));
        if cmd_id.is_ok() {
            self.set_connection_state(crate::events::ConnectionState::Connected);
        }
        cmd_id
    }
}
