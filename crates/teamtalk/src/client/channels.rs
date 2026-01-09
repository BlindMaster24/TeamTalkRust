//! Channel management APIs.
use super::Client;
use crate::types::{Channel, ChannelId, TT_STRLEN, UserId};
use crate::utils::{ToTT, strings::tt_buf};
use teamtalk_sys as ffi;

impl Client {
    /// Returns all server channels.
    pub fn get_server_channels(&self) -> Vec<Channel> {
        let mut count: i32 = 0;
        unsafe {
            ffi::api().TT_GetServerChannels(self.ptr, std::ptr::null_mut(), &mut count);
            let mut channels = vec![std::mem::zeroed::<ffi::Channel>(); count as usize];
            if ffi::api().TT_GetServerChannels(self.ptr, channels.as_mut_ptr(), &mut count) == 1 {
                channels.into_iter().map(Channel::from).collect()
            } else {
                vec![]
            }
        }
    }

    /// Returns a channel by id.
    pub fn get_channel(&self, id: ChannelId) -> Option<Channel> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::Channel>() };
        if unsafe { ffi::api().TT_GetChannel(self.ptr, id.0, &mut raw) } == 1 {
            Some(Channel::from(raw))
        } else {
            None
        }
    }

    /// Returns a channel path string for a channel id.
    pub fn get_channel_path(&self, id: ChannelId) -> String {
        let mut buf = tt_buf::<TT_STRLEN>();
        unsafe {
            if ffi::api().TT_GetChannelPath(self.ptr, id.0, buf.as_mut_ptr()) == 1 {
                crate::utils::strings::to_string(&buf)
            } else {
                String::new()
            }
        }
    }

    /// Returns a channel id for a path string.
    pub fn get_channel_id_from_path(&self, path: &str) -> ChannelId {
        ChannelId(unsafe { ffi::api().TT_GetChannelIDFromPath(self.ptr, path.tt().as_ptr()) })
    }

    /// Joins a channel by id.
    pub fn join_channel(&self, id: ChannelId, password: &str) -> i32 {
        let cmd_id =
            unsafe { ffi::api().TT_DoJoinChannelByID(self.ptr, id.0, password.tt().as_ptr()) };
        if cmd_id > 0 {
            self.set_connection_state(crate::events::ConnectionState::Joining(id));
        }
        cmd_id
    }

    /// Joins the root channel.
    pub fn join_root(&self) -> i32 {
        let root_id = self.get_root_channel_id();
        self.join_channel(root_id, "")
    }

    /// Leaves the current channel.
    pub fn leave_channel(&self) -> i32 {
        unsafe { ffi::api().TT_DoLeaveChannel(self.ptr) }
    }

    /// Creates a new channel.
    pub fn make_channel(&self, channel: &Channel) -> i32 {
        unsafe { ffi::api().TT_DoMakeChannel(self.ptr, &channel.to_ffi()) }
    }

    /// Updates an existing channel.
    pub fn update_channel(&self, channel: &Channel) -> i32 {
        unsafe { ffi::api().TT_DoUpdateChannel(self.ptr, &channel.to_ffi()) }
    }

    /// Removes a channel by id.
    pub fn remove_channel(&self, id: ChannelId) -> i32 {
        unsafe { ffi::api().TT_DoRemoveChannel(self.ptr, id.0) }
    }

    /// Moves a user to a channel.
    pub fn move_user(&self, user_id: UserId, channel_id: ChannelId) -> i32 {
        unsafe { ffi::api().TT_DoMoveUser(self.ptr, user_id.0, channel_id.0) }
    }

    /// Returns true if the user is a channel operator.
    pub fn is_operator(&self, user_id: UserId, channel_id: ChannelId) -> bool {
        unsafe { ffi::api().TT_IsChannelOperator(self.ptr, user_id.0, channel_id.0) == 1 }
    }

    pub fn set_channel_operator(
        &self,
        user_id: UserId,
        channel_id: ChannelId,
        make_op: bool,
    ) -> i32 {
        unsafe {
            ffi::api().TT_DoChannelOp(
                self.ptr,
                user_id.0,
                channel_id.0,
                if make_op { 1 } else { 0 },
            )
        }
    }

    /// Returns the current channel id.
    pub fn my_channel_id(&self) -> ChannelId {
        ChannelId(unsafe { ffi::api().TT_GetMyChannelID(self.ptr) })
    }

    /// Returns the root channel id.
    pub fn get_root_channel_id(&self) -> ChannelId {
        ChannelId(unsafe { ffi::api().TT_GetRootChannelID(self.ptr) })
    }

    /// Returns users currently in the channel.
    pub fn get_channel_users(&self, channel_id: ChannelId) -> Vec<crate::types::User> {
        let mut count: i32 = 0;
        unsafe {
            ffi::api().TT_GetChannelUsers(self.ptr, channel_id.0, std::ptr::null_mut(), &mut count);
            let mut users = vec![std::mem::zeroed::<ffi::User>(); count as usize];
            if ffi::api().TT_GetChannelUsers(self.ptr, channel_id.0, users.as_mut_ptr(), &mut count)
                == 1
            {
                users.into_iter().map(crate::types::User::from).collect()
            } else {
                vec![]
            }
        }
    }
}
