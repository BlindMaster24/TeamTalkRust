use super::{
    Client, CommandId, Subscriptions, User, UserAccount, UserId, UserStatistics,
    can_issue_logged_in_command,
};
use std::time::{Duration, Instant};

fn wait_slice(deadline: Instant) -> i32 {
    deadline
        .saturating_duration_since(Instant::now())
        .min(Duration::from_millis(50))
        .as_millis()
        .min(i32::MAX as u128) as i32
}

impl Client {
    /// Returns a user by id.
    pub fn get_user(&self, user_id: UserId) -> Option<User> {
        self.backend().get_user_by_id(self.ptr.0, user_id)
    }

    /// Returns a user by username.
    pub fn get_user_by_username(&self, username: &str) -> Option<User> {
        self.backend().get_user_by_username(self.ptr.0, username)
    }

    /// Returns user statistics by id.
    pub fn get_user_statistics(&self, user_id: UserId) -> Option<UserStatistics> {
        self.backend().get_user_statistics(self.ptr.0, user_id)
    }

    /// Requests a list of user accounts.
    #[must_use]
    pub fn list_user_accounts(&self, index: i32, count: i32) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(
            self.backend()
                .do_list_user_accounts(self.ptr.0, index, count),
        )
    }

    /// Requests a list of user accounts and waits for the matching list to complete.
    pub fn list_user_accounts_and_wait(
        &self,
        index: i32,
        count: i32,
        timeout_ms: i32,
    ) -> crate::events::Result<Vec<UserAccount>> {
        let cmd_id = self.list_user_accounts(index, count);
        let mut accounts = Vec::new();
        self.poll_command_completion(
            cmd_id,
            timeout_ms,
            "user account list command",
            |event, message| {
                if matches!(event, crate::events::Event::UserAccount)
                    && let Some(account) = message.account()
                {
                    accounts.push(account);
                }
            },
        )?;
        Ok(accounts)
    }

    /// Creates a user account.
    #[must_use]
    pub fn create_user_account(&self, account: &UserAccount) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(self.backend().do_new_user_account(self.ptr.0, account))
    }

    /// Creates a user account and waits for the matching account-created event.
    pub fn create_user_account_and_wait(
        &self,
        account: &UserAccount,
        timeout_ms: i32,
    ) -> crate::events::Result<UserAccount> {
        let cmd_id = self.create_user_account(account);
        if !cmd_id.is_ok() {
            return Err(crate::events::Error::CommandFailed {
                code: 0,
                message: "create user account command rejected in current state".to_string(),
            });
        }
        if timeout_ms < 0 {
            loop {
                if let Some((event, message)) = self.poll(50) {
                    match event {
                        crate::events::Event::UserAccountCreated => {
                            if let Some(created) = message.account()
                                && created.username == account.username
                            {
                                return Ok(created);
                            }
                        }
                        crate::events::Event::CmdError if cmd_id == message.command_id() => {
                            return Err(crate::events::Error::CommandFailed {
                                code: message.source(),
                                message: "create user account command failed".to_string(),
                            });
                        }
                        _ => {}
                    }
                }
            }
        }

        let deadline = Instant::now() + Duration::from_millis(timeout_ms as u64);
        loop {
            let wait_ms = wait_slice(deadline);
            if wait_ms <= 0 {
                return Err(crate::events::Error::timeout(
                    crate::events::TimeoutKind::Command,
                ));
            }
            if let Some((event, message)) = self.poll(wait_ms) {
                match event {
                    crate::events::Event::UserAccountCreated => {
                        if let Some(created) = message.account()
                            && created.username == account.username
                        {
                            return Ok(created);
                        }
                    }
                    crate::events::Event::CmdError if cmd_id == message.command_id() => {
                        return Err(crate::events::Error::CommandFailed {
                            code: message.source(),
                            message: "create user account command failed".to_string(),
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    /// Deletes a user account by username.
    #[must_use]
    pub fn delete_user_account(&self, username: &str) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(self.backend().do_delete_user_account(self.ptr.0, username))
    }

    /// Deletes a user account and waits for the matching account-removed event.
    pub fn delete_user_account_and_wait(
        &self,
        username: &str,
        timeout_ms: i32,
    ) -> crate::events::Result<UserAccount> {
        let cmd_id = self.delete_user_account(username);
        if !cmd_id.is_ok() {
            return Err(crate::events::Error::CommandFailed {
                code: 0,
                message: "delete user account command rejected in current state".to_string(),
            });
        }
        if timeout_ms < 0 {
            loop {
                if let Some((event, message)) = self.poll(50) {
                    match event {
                        crate::events::Event::UserAccountRemoved => {
                            if let Some(removed) = message.account()
                                && removed.username == username
                            {
                                return Ok(removed);
                            }
                        }
                        crate::events::Event::CmdError if cmd_id == message.command_id() => {
                            return Err(crate::events::Error::CommandFailed {
                                code: message.source(),
                                message: "delete user account command failed".to_string(),
                            });
                        }
                        _ => {}
                    }
                }
            }
        }

        let deadline = Instant::now() + Duration::from_millis(timeout_ms as u64);
        loop {
            let wait_ms = wait_slice(deadline);
            if wait_ms <= 0 {
                return Err(crate::events::Error::timeout(
                    crate::events::TimeoutKind::Command,
                ));
            }
            if let Some((event, message)) = self.poll(wait_ms) {
                match event {
                    crate::events::Event::UserAccountRemoved => {
                        if let Some(removed) = message.account()
                            && removed.username == username
                        {
                            return Ok(removed);
                        }
                    }
                    crate::events::Event::CmdError if cmd_id == message.command_id() => {
                        return Err(crate::events::Error::CommandFailed {
                            code: message.source(),
                            message: "delete user account command failed".to_string(),
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    /// Returns the current user's subscription mask.
    #[must_use]
    pub fn my_subscriptions(&self) -> Subscriptions {
        self.backend().get_my_local_subscriptions(self.ptr.0)
    }
}
