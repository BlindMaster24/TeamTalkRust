use super::*;

impl Client {
    /// Returns a user by id.
    pub fn get_user(&self, user_id: UserId) -> Option<User> {
        self.backend().get_user_by_id(self.ptr.0, user_id.0)
    }

    /// Returns a user by username.
    pub fn get_user_by_username(&self, username: &str) -> Option<User> {
        self.backend().get_user_by_username(self.ptr.0, username)
    }

    /// Returns user statistics by id.
    pub fn get_user_statistics(&self, user_id: UserId) -> Option<UserStatistics> {
        self.backend().get_user_statistics(self.ptr.0, user_id.0)
    }

    /// Requests a list of user accounts.
    pub fn list_user_accounts(&self, index: i32, count: i32) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        self.backend()
            .do_list_user_accounts(self.ptr.0, index, count)
    }

    /// Creates a user account.
    pub fn create_user_account(&self, account: &UserAccount) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        self.backend().do_new_user_account(self.ptr.0, account)
    }

    /// Deletes a user account by username.
    pub fn delete_user_account(&self, username: &str) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        self.backend().do_delete_user_account(self.ptr.0, username)
    }

    /// Returns the current user's subscription mask.
    pub fn my_subscriptions(&self) -> Subscriptions {
        self.backend().get_my_local_subscriptions(self.ptr.0)
    }
}
