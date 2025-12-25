use super::Client;
use crate::types::{
    ChannelId, MessageTarget, Subscriptions, User, UserAccount, UserId, UserStatistics, UserStatus,
};
use crate::utils::ToTT;
use teamtalk_sys as ffi;

impl Client {
    pub fn login(&self, nickname: &str, username: &str, password: &str, client_name: &str) -> i32 {
        unsafe {
            ffi::api().TT_DoLoginEx(
                self.ptr,
                nickname.tt().as_ptr(),
                username.tt().as_ptr(),
                password.tt().as_ptr(),
                client_name.tt().as_ptr(),
            )
        }
    }

    pub fn logout(&self) -> i32 {
        unsafe { ffi::api().TT_DoLogout(self.ptr) }
    }

    pub fn my_id(&self) -> UserId {
        UserId(unsafe { ffi::api().TT_GetMyUserID(self.ptr) })
    }

    pub fn get_my_user_account(&self) -> Option<UserAccount> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::UserAccount>() };
        if unsafe { ffi::api().TT_GetMyUserAccount(self.ptr, &mut raw) } == 1 {
            Some(UserAccount::from(raw))
        } else {
            None
        }
    }

    pub fn get_my_user_type(&self) -> u32 {
        unsafe { ffi::api().TT_GetMyUserType(self.ptr) }
    }

    pub fn get_my_user_rights(&self) -> u32 {
        unsafe { ffi::api().TT_GetMyUserRights(self.ptr) }
    }

    pub fn get_my_user_data(&self) -> i32 {
        unsafe { ffi::api().TT_GetMyUserData(self.ptr) }
    }

    pub fn change_nickname(&self, nick: &str) -> i32 {
        unsafe { ffi::api().TT_DoChangeNickname(self.ptr, nick.tt().as_ptr()) }
    }

    pub fn set_status(&self, status: UserStatus, msg: &str) -> i32 {
        unsafe {
            ffi::api().TT_DoChangeStatus(self.ptr, status.to_bits() as i32, msg.tt().as_ptr())
        }
    }

    pub fn set_status_message(&self, msg: &str) -> i32 {
        let mut user = unsafe { std::mem::zeroed::<ffi::User>() };
        let my_id = self.my_id();
        let bits = if unsafe { ffi::api().TT_GetUser(self.ptr, my_id.0, &mut user) } == 1 {
            user.nStatusMode as u32
        } else {
            UserStatus::default().to_bits()
        };
        unsafe { ffi::api().TT_DoChangeStatus(self.ptr, bits as i32, msg.tt().as_ptr()) }
    }

    pub fn kick_user(&self, user_id: UserId, channel_id: ChannelId) -> i32 {
        unsafe { ffi::api().TT_DoKickUser(self.ptr, user_id.0, channel_id.0) }
    }

    pub fn ban_user(&self, user_id: UserId, channel_id: ChannelId) -> i32 {
        unsafe { ffi::api().TT_DoBanUser(self.ptr, user_id.0, channel_id.0) }
    }

    pub fn ban_user_ex(&self, user_id: UserId, ban_types: u32) -> i32 {
        unsafe { ffi::api().TT_DoBanUserEx(self.ptr, user_id.0, ban_types) }
    }

    pub fn unban_user(&self, ip: &str, channel_id: ChannelId) -> i32 {
        unsafe { ffi::api().TT_DoUnBanUser(self.ptr, ip.tt().as_ptr(), channel_id.0) }
    }

    pub fn send_text<T: Into<MessageTarget>>(&self, target: T, text: &str) -> i32 {
        let mut msg = unsafe { std::mem::zeroed::<ffi::TextMessage>() };
        match target.into() {
            MessageTarget::User(id) => {
                msg.nMsgType = ffi::TextMsgType::MSGTYPE_USER;
                msg.nToUserID = id.0;
            }
            MessageTarget::Channel(id) => {
                msg.nMsgType = ffi::TextMsgType::MSGTYPE_CHANNEL;
                msg.nChannelID = id.0;
            }
            MessageTarget::Broadcast => {
                msg.nMsgType = ffi::TextMsgType::MSGTYPE_BROADCAST;
            }
        }
        let t = text.tt();
        unsafe {
            let len = t.len().min(511);
            std::ptr::copy_nonoverlapping(t.as_ptr(), msg.szMessage.as_mut_ptr(), len);
            ffi::api().TT_DoTextMessage(self.ptr, &msg)
        }
    }

    pub fn send_to_user(&self, user_id: UserId, text: &str) -> i32 {
        self.send_text(MessageTarget::User(user_id), text)
    }

    pub fn send_to_channel(&self, channel_id: ChannelId, text: &str) -> i32 {
        self.send_text(MessageTarget::Channel(channel_id), text)
    }

    pub fn send_to_all(&self, text: &str) -> i32 {
        self.send_text(MessageTarget::Broadcast, text)
    }

    pub fn ban(&self, banned_user: &crate::types::BannedUser) -> i32 {
        unsafe { ffi::api().TT_DoBan(self.ptr, &banned_user.to_ffi()) }
    }

    pub fn unban_ex(&self, banned_user: &crate::types::BannedUser) -> i32 {
        unsafe { ffi::api().TT_DoUnBanUserEx(self.ptr, &banned_user.to_ffi()) }
    }

    pub fn get_user(&self, user_id: UserId) -> Option<User> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::User>() };
        if unsafe { ffi::api().TT_GetUser(self.ptr, user_id.0, &mut raw) } == 1 {
            Some(User::from(raw))
        } else {
            None
        }
    }

    pub fn get_user_by_username(&self, username: &str) -> Option<User> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::User>() };
        if unsafe { ffi::api().TT_GetUserByUsername(self.ptr, username.tt().as_ptr(), &mut raw) }
            == 1
        {
            Some(User::from(raw))
        } else {
            None
        }
    }

    pub fn get_user_statistics(&self, user_id: UserId) -> Option<UserStatistics> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::UserStatistics>() };
        if unsafe { ffi::api().TT_GetUserStatistics(self.ptr, user_id.0, &mut raw) } == 1 {
            Some(UserStatistics::from(raw))
        } else {
            None
        }
    }

    pub fn list_user_accounts(&self, index: i32, count: i32) -> i32 {
        unsafe { ffi::api().TT_DoListUserAccounts(self.ptr, index, count) }
    }

    pub fn create_user_account(&self, account: &UserAccount) -> i32 {
        unsafe { ffi::api().TT_DoNewUserAccount(self.ptr, &account.to_ffi()) }
    }

    pub fn delete_user_account(&self, username: &str) -> i32 {
        unsafe { ffi::api().TT_DoDeleteUserAccount(self.ptr, username.tt().as_ptr()) }
    }

    pub fn subscribe(&self, user_id: UserId, mask: Subscriptions) -> i32 {
        unsafe { ffi::api().TT_DoSubscribe(self.ptr, user_id.0, mask.raw()) }
    }

    pub fn unsubscribe(&self, user_id: UserId, mask: Subscriptions) -> i32 {
        unsafe { ffi::api().TT_DoUnsubscribe(self.ptr, user_id.0, mask.raw()) }
    }

    pub fn unsubscribe_all_from_user(&self, user_id: UserId) -> i32 {
        unsafe { ffi::api().TT_DoUnsubscribe(self.ptr, user_id.0, Subscriptions::ALL) }
    }

    pub fn unsubscribe_all(&self) -> i32 {
        unsafe { ffi::api().TT_DoUnsubscribe(self.ptr, 0, Subscriptions::ALL) }
    }

    pub fn channel_op_ex(
        &self,
        user_id: UserId,
        channel_id: ChannelId,
        password: &str,
        make_op: bool,
    ) -> i32 {
        unsafe {
            ffi::api().TT_DoChannelOpEx(
                self.ptr,
                user_id.0,
                channel_id.0,
                password.tt().as_ptr(),
                if make_op { 1 } else { 0 },
            )
        }
    }

    pub fn my_subscriptions(&self) -> Subscriptions {
        let mut user = unsafe { std::mem::zeroed::<ffi::User>() };
        let my_id = self.my_id();
        if unsafe { ffi::api().TT_GetUser(self.ptr, my_id.0, &mut user) } == 1 {
            Subscriptions::from_raw(user.uLocalSubscriptions)
        } else {
            Subscriptions::new()
        }
    }
}
