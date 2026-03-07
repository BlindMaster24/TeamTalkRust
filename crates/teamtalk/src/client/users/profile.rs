use super::*;
use crate::types::UserRights;

impl Client {
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
        self.backend().get_my_user_rights(self.ptr.0)
    }

    /// Returns the user rights of the current user as a typed bitmask wrapper.
    pub fn my_user_rights(&self) -> UserRights {
        UserRights::from_raw(self.get_my_user_rights())
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
