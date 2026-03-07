use super::*;

impl Client {
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
