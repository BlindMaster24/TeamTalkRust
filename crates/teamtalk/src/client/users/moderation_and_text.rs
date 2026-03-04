use super::*;

impl Client {
    pub fn kick_user(&self, user_id: UserId, channel_id: ChannelId) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        unsafe { ffi::api().TT_DoKickUser(self.ptr.0, user_id.0, channel_id.0) }
    }

    /// Bans a user from a channel.
    pub fn ban_user(&self, user_id: UserId, channel_id: ChannelId) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        unsafe { ffi::api().TT_DoBanUser(self.ptr.0, user_id.0, channel_id.0) }
    }

    /// Bans a user with custom ban types.
    pub fn ban_user_ex(&self, user_id: UserId, ban_types: u32) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        unsafe { ffi::api().TT_DoBanUserEx(self.ptr.0, user_id.0, ban_types) }
    }

    /// Removes a ban by IP address.
    pub fn unban_user(&self, ip: &str, channel_id: ChannelId) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        unsafe { ffi::api().TT_DoUnBanUser(self.ptr.0, ip.tt().as_ptr(), channel_id.0) }
    }

    /// Sends a text message to a target.
    pub fn send_text<T: Into<MessageTarget>>(&self, target: T, text: &str) -> i32 {
        self.send_text_with_options(target, text, SendTextOptions::default())
    }

    /// Sends a text message to a target using explicit options.
    pub fn send_text_with_options<T: Into<MessageTarget>>(
        &self,
        target: T,
        text: &str,
        options: SendTextOptions,
    ) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        let target = target.into();
        let chunks = split_text_chunks(text, TT_TEXT_MAX_PAYLOAD);
        let total_chunks = chunks.len();
        let mut last_cmd_id = 0;

        for (index, chunk) in chunks.into_iter().enumerate() {
            let mut msg = text_message_for_target(target);
            msg.bMore = if index + 1 < total_chunks { 1 } else { 0 };
            let tt = chunk.tt();
            unsafe {
                let len = tt.len().min(TT_TEXT_MAX_PAYLOAD);
                std::ptr::copy_nonoverlapping(tt.as_ptr(), msg.szMessage.as_mut_ptr(), len);
            }

            let mut retries_left = if index == 0 {
                options.first_chunk_retries
            } else {
                0
            };
            loop {
                let cmd_id = self.backend().do_text_message(self.ptr.0, &msg);
                if cmd_id > 0 {
                    last_cmd_id = cmd_id;
                    break;
                }
                if retries_left == 0 {
                    return cmd_id;
                }
                retries_left -= 1;
            }
        }

        last_cmd_id
    }

    /// Sends a text message to a user.
    pub fn send_to_user(&self, user_id: UserId, text: &str) -> i32 {
        self.send_text(MessageTarget::User(user_id), text)
    }

    /// Sends a text message to a channel.
    pub fn send_to_channel(&self, channel_id: ChannelId, text: &str) -> i32 {
        self.send_text(MessageTarget::Channel(channel_id), text)
    }

    /// Sends a text message to all users.
    pub fn send_to_all(&self, text: &str) -> i32 {
        self.send_text(MessageTarget::Broadcast, text)
    }

    /// Adds a user to the ban list.
    pub fn ban(&self, banned_user: &crate::types::BannedUser) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        unsafe { ffi::api().TT_DoBan(self.ptr.0, &banned_user.to_ffi()) }
    }

    /// Removes a user from the ban list.
    pub fn unban_ex(&self, banned_user: &crate::types::BannedUser) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        unsafe { ffi::api().TT_DoUnBanUserEx(self.ptr.0, &banned_user.to_ffi()) }
    }

    /// Returns a user by id.
    pub fn get_user(&self, user_id: UserId) -> Option<User> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::User>() };
        if unsafe { ffi::api().TT_GetUser(self.ptr.0, user_id.0, &mut raw) } == 1 {
            Some(User::from(raw))
        } else {
            None
        }
    }

    /// Returns a user by username.
    pub fn get_user_by_username(&self, username: &str) -> Option<User> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::User>() };
        if unsafe { ffi::api().TT_GetUserByUsername(self.ptr.0, username.tt().as_ptr(), &mut raw) }
            == 1
        {
            Some(User::from(raw))
        } else {
            None
        }
    }

    /// Returns user statistics by id.
    pub fn get_user_statistics(&self, user_id: UserId) -> Option<UserStatistics> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::UserStatistics>() };
        if unsafe { ffi::api().TT_GetUserStatistics(self.ptr.0, user_id.0, &mut raw) } == 1 {
            Some(UserStatistics::from(raw))
        } else {
            None
        }
    }

    /// Requests a list of user accounts.
    pub fn list_user_accounts(&self, index: i32, count: i32) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        unsafe { ffi::api().TT_DoListUserAccounts(self.ptr.0, index, count) }
    }

    /// Creates a user account.
    pub fn create_user_account(&self, account: &UserAccount) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        unsafe { ffi::api().TT_DoNewUserAccount(self.ptr.0, &account.to_ffi()) }
    }

    /// Deletes a user account by username.
    pub fn delete_user_account(&self, username: &str) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        unsafe { ffi::api().TT_DoDeleteUserAccount(self.ptr.0, username.tt().as_ptr()) }
    }

    /// Subscribes to a user's streams.
    pub fn subscribe(&self, user_id: UserId, mask: Subscriptions) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        unsafe { ffi::api().TT_DoSubscribe(self.ptr.0, user_id.0, mask.raw()) }
    }

    /// Unsubscribes from a user's streams.
    pub fn unsubscribe(&self, user_id: UserId, mask: Subscriptions) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        unsafe { ffi::api().TT_DoUnsubscribe(self.ptr.0, user_id.0, mask.raw()) }
    }

    /// Unsubscribes from all streams for a user.
    pub fn unsubscribe_all_from_user(&self, user_id: UserId) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        unsafe { ffi::api().TT_DoUnsubscribe(self.ptr.0, user_id.0, Subscriptions::ALL) }
    }

    /// Unsubscribes from all streams for all users.
    pub fn unsubscribe_all(&self) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        unsafe { ffi::api().TT_DoUnsubscribe(self.ptr.0, 0, Subscriptions::ALL) }
    }

    pub fn channel_op_ex(
        &self,
        user_id: UserId,
        channel_id: ChannelId,
        password: &str,
        make_op: bool,
    ) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        unsafe {
            ffi::api().TT_DoChannelOpEx(
                self.ptr.0,
                user_id.0,
                channel_id.0,
                password.tt().as_ptr(),
                if make_op { 1 } else { 0 },
            )
        }
    }

    pub fn set_channel_operator(
        &self,
        user_id: UserId,
        channel_id: ChannelId,
        make_op: bool,
    ) -> i32 {
        self.channel_op_ex(user_id, channel_id, "", make_op)
    }

    pub fn set_user_operator(&self, user_id: UserId, channel_id: ChannelId, make_op: bool) -> i32 {
        self.set_channel_operator(user_id, channel_id, make_op)
    }

    pub fn set_user_operator_ex(
        &self,
        user_id: UserId,
        channel_id: ChannelId,
        password: &str,
        make_op: bool,
    ) -> i32 {
        self.channel_op_ex(user_id, channel_id, password, make_op)
    }

    pub fn set_user_text_mute(&self, user_id: UserId, mute: bool) -> i32 {
        if mute {
            self.unsubscribe(user_id, Subscriptions::all_text())
        } else {
            self.subscribe(user_id, Subscriptions::all_text())
        }
    }

    /// Grants operator status to a user in a channel.
    pub fn op_user(&self, user_id: UserId, channel_id: ChannelId) -> i32 {
        self.set_channel_operator(user_id, channel_id, true)
    }

    /// Revokes operator status from a user in a channel.
    pub fn deop_user(&self, user_id: UserId, channel_id: ChannelId) -> i32 {
        self.set_channel_operator(user_id, channel_id, false)
    }

    /// Mutes text messages from a user (local subscription).
    pub fn mute_user_text(&self, user_id: UserId) -> i32 {
        self.set_user_text_mute(user_id, true)
    }

    /// Unmutes text messages from a user (local subscription).
    pub fn unmute_user_text(&self, user_id: UserId) -> i32 {
        self.set_user_text_mute(user_id, false)
    }

    /// Grants operator status to a user in a channel (with password).
    pub fn op_user_ex(&self, user_id: UserId, channel_id: ChannelId, password: &str) -> i32 {
        self.set_user_operator_ex(user_id, channel_id, password, true)
    }

    /// Revokes operator status from a user in a channel (with password).
    pub fn deop_user_ex(&self, user_id: UserId, channel_id: ChannelId, password: &str) -> i32 {
        self.set_user_operator_ex(user_id, channel_id, password, false)
    }

    /// Mutes voice streams from a user (local subscription).
    pub fn mute_user_voice(&self, user_id: UserId) -> i32 {
        self.unsubscribe(user_id, Subscriptions::from_raw(Subscriptions::VOICE))
    }

    /// Unmutes voice streams from a user (local subscription).
    pub fn unmute_user_voice(&self, user_id: UserId) -> i32 {
        self.subscribe(user_id, Subscriptions::from_raw(Subscriptions::VOICE))
    }

    /// Mutes media file streams from a user (local subscription).
    pub fn mute_user_media(&self, user_id: UserId) -> i32 {
        self.unsubscribe(user_id, Subscriptions::from_raw(Subscriptions::MEDIAFILE))
    }

    /// Unmutes media file streams from a user (local subscription).
    pub fn unmute_user_media(&self, user_id: UserId) -> i32 {
        self.subscribe(user_id, Subscriptions::from_raw(Subscriptions::MEDIAFILE))
    }

    /// Returns the current user's subscription mask.
    pub fn my_subscriptions(&self) -> Subscriptions {
        let mut user = unsafe { std::mem::zeroed::<ffi::User>() };
        let my_id = self.my_id();
        if unsafe { ffi::api().TT_GetUser(self.ptr.0, my_id.0, &mut user) } == 1 {
            Subscriptions::from_raw(user.uLocalSubscriptions)
        } else {
            Subscriptions::new()
        }
    }
}
