use super::*;

impl Client {
    /// Logs in to the server.
    pub fn login(&self, nickname: &str, username: &str, password: &str, client_name: &str) -> i32 {
        if !can_login_in_state(self.connection_state()) {
            return 0;
        }
        let cmd_id =
            self.backend()
                .do_login_ex(self.ptr.0, nickname, username, password, client_name);
        if cmd_id > 0 {
            self.set_connection_state(crate::events::ConnectionState::LoggingIn);
        }
        cmd_id
    }

    /// Stores login parameters for automatic login.
    pub fn set_login_params(&self, params: LoginParams) {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        auto.login = Some(params);
        auto.login_gave_up = false;
        auto.pending_login_cmd = None;
    }

    /// Returns stored login parameters, if any.
    pub fn login_params(&self) -> Option<LoginParams> {
        self.auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .login
            .clone()
    }

    /// Logs in using stored login parameters.
    pub fn login_with_params(&self) -> Result<i32, crate::events::Error> {
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
        if cmd_id <= 0 {
            return Err(crate::events::Error::AuthFailed);
        }
        let waited = self.poll_until(timeout_ms, |event, msg| match event {
            crate::events::Event::MySelfLoggedIn => true,
            crate::events::Event::CmdError => msg.source() == cmd_id,
            _ => false,
        });
        let Some((event, message)) = waited else {
            return Err(crate::events::Error::Timeout);
        };
        if matches!(event, crate::events::Event::CmdError) {
            return Err(crate::events::Error::AuthFailed);
        }
        Ok(message)
    }

    /// Stores login parameters and immediately logs in.
    pub fn login_remember(
        &self,
        nickname: &str,
        username: &str,
        password: &str,
        client_name: &str,
    ) -> i32 {
        let params = LoginParams::new(nickname, username, password, client_name);
        self.set_login_params(params);
        self.login(nickname, username, password, client_name)
    }

    pub fn login_from_env(&self) -> i32 {
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
    pub fn logout(&self) -> i32 {
        if !can_logout_in_state(self.connection_state()) {
            return 0;
        }
        let cmd_id = self.backend().do_logout(self.ptr.0);
        if cmd_id > 0 {
            self.set_connection_state(crate::events::ConnectionState::Connected);
        }
        cmd_id
    }
}
