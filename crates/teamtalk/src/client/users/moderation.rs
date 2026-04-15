use super::*;

impl Client {
    pub fn channel_op(&self, user_id: UserId, channel_id: ChannelId, make_op: bool) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(unsafe {
            ffi::api().TT_DoChannelOp(self.ptr.0, user_id.0, channel_id.0, i32::from(make_op))
        })
    }

    pub fn kick_user(&self, user_id: UserId, channel_id: ChannelId) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(unsafe { ffi::api().TT_DoKickUser(self.ptr.0, user_id.0, channel_id.0) })
    }

    /// Bans a user from a channel.
    pub fn ban_user(&self, user_id: UserId, channel_id: ChannelId) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(unsafe { ffi::api().TT_DoBanUser(self.ptr.0, user_id.0, channel_id.0) })
    }

    /// Bans a user with custom ban types.
    pub fn ban_user_ex(&self, user_id: UserId, ban_types: u32) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(unsafe { ffi::api().TT_DoBanUserEx(self.ptr.0, user_id.0, ban_types) })
    }

    /// Removes a ban by IP address.
    pub fn unban_user(&self, ip: &str, channel_id: ChannelId) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(unsafe { ffi::api().TT_DoUnBanUser(self.ptr.0, ip.tt().as_ptr(), channel_id.0) })
    }

    /// Adds a user to the ban list.
    pub fn ban(&self, banned_user: &crate::types::BannedUser) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(unsafe { ffi::api().TT_DoBan(self.ptr.0, &banned_user.to_ffi()) })
    }

    /// Removes a user from the ban list.
    pub fn unban_ex(&self, banned_user: &crate::types::BannedUser) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(unsafe { ffi::api().TT_DoUnBanUserEx(self.ptr.0, &banned_user.to_ffi()) })
    }

    pub fn channel_op_ex(
        &self,
        user_id: UserId,
        channel_id: ChannelId,
        password: &str,
        make_op: bool,
    ) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(unsafe {
            ffi::api().TT_DoChannelOpEx(
                self.ptr.0,
                user_id.0,
                channel_id.0,
                password.tt().as_ptr(),
                if make_op { 1 } else { 0 },
            )
        })
    }

    pub fn set_channel_operator(
        &self,
        user_id: UserId,
        channel_id: ChannelId,
        make_op: bool,
    ) -> CommandId {
        self.channel_op(user_id, channel_id, make_op)
    }

    pub fn set_user_operator(
        &self,
        user_id: UserId,
        channel_id: ChannelId,
        make_op: bool,
    ) -> CommandId {
        self.set_channel_operator(user_id, channel_id, make_op)
    }

    pub fn set_user_operator_ex(
        &self,
        user_id: UserId,
        channel_id: ChannelId,
        password: &str,
        make_op: bool,
    ) -> CommandId {
        self.channel_op_ex(user_id, channel_id, password, make_op)
    }

    /// Grants operator status to a user in a channel.
    pub fn op_user(&self, user_id: UserId, channel_id: ChannelId) -> CommandId {
        self.set_channel_operator(user_id, channel_id, true)
    }

    /// Revokes operator status from a user in a channel.
    pub fn deop_user(&self, user_id: UserId, channel_id: ChannelId) -> CommandId {
        self.set_channel_operator(user_id, channel_id, false)
    }

    /// Grants operator status to a user in a channel (with password).
    pub fn op_user_ex(&self, user_id: UserId, channel_id: ChannelId, password: &str) -> CommandId {
        self.set_user_operator_ex(user_id, channel_id, password, true)
    }

    /// Revokes operator status from a user in a channel (with password).
    pub fn deop_user_ex(
        &self,
        user_id: UserId,
        channel_id: ChannelId,
        password: &str,
    ) -> CommandId {
        self.set_user_operator_ex(user_id, channel_id, password, false)
    }
}
