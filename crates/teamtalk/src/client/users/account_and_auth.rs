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

    /// Returns the current user id.
    pub fn my_id(&self) -> UserId {
        UserId(self.backend().get_my_user_id(self.ptr.0))
    }

    /// Returns the account of the current user.
    pub fn get_my_user_account(&self) -> Option<UserAccount> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::UserAccount>() };
        if unsafe { ffi::api().TT_GetMyUserAccount(self.ptr.0, &mut raw) } == 1 {
            Some(UserAccount::from(raw))
        } else {
            None
        }
    }

    /// Returns the user type of the current user.
    pub fn get_my_user_type(&self) -> u32 {
        unsafe { ffi::api().TT_GetMyUserType(self.ptr.0) }
    }

    /// Returns the user rights of the current user.
    pub fn get_my_user_rights(&self) -> u32 {
        unsafe { ffi::api().TT_GetMyUserRights(self.ptr.0) }
    }

    /// Requests user data for the current user.
    pub fn get_my_user_data(&self) -> i32 {
        unsafe { ffi::api().TT_GetMyUserData(self.ptr.0) }
    }

    /// Changes the current nickname.
    pub fn change_nickname(&self, nick: &str) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        unsafe { ffi::api().TT_DoChangeNickname(self.ptr.0, nick.tt().as_ptr()) }
    }

    /// Sets the status and status message.
    pub fn set_status(&self, status: UserStatus, msg: &str) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        unsafe {
            ffi::api().TT_DoChangeStatus(self.ptr.0, status.to_bits() as i32, msg.tt().as_ptr())
        }
    }

    /// Updates only the status message.
    pub fn set_status_message(&self, msg: &str) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        let mut user = unsafe { std::mem::zeroed::<ffi::User>() };
        let my_id = self.my_id();
        let bits = if self.backend().get_user(self.ptr.0, my_id.0, &mut user) {
            user.nStatusMode as u32
        } else {
            UserStatus::default().to_bits()
        };
        self.backend()
            .do_change_status(self.ptr.0, bits as i32, msg)
    }
}
