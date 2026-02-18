//! User management namespace.
use super::define_namespace;
use crate::client::users::{LoginParams, SendTextOptions};
use crate::events::Result;
use crate::types::{
    ChannelId, MessageTarget, Subscriptions, User, UserAccount, UserId, UserStatistics, UserStatus,
};

define_namespace!(UsersNamespace);

impl UsersNamespace {
    /// Logs in to the server.
    pub fn login(&self, nickname: &str, username: &str, password: &str, client_name: &str) -> i32 {
        self.client.login(nickname, username, password, client_name)
    }

    /// Stores login parameters for automatic login.
    pub fn set_login_params(&self, params: LoginParams) {
        self.client.set_login_params(params)
    }

    /// Returns stored login parameters, if any.
    pub fn login_params(&self) -> Option<LoginParams> {
        self.client.login_params()
    }

    /// Logs in using stored login parameters.
    pub fn login_with_params(&self) -> Result<i32> {
        self.client.login_with_params()
    }

    /// Stores login parameters and immediately logs in.
    pub fn login_remember(
        &self,
        nickname: &str,
        username: &str,
        password: &str,
        client_name: &str,
    ) -> i32 {
        self.client
            .login_remember(nickname, username, password, client_name)
    }

    /// Logs in using environment variables.
    pub fn login_from_env(&self) -> i32 {
        self.client.login_from_env()
    }

    /// Logs out from the server.
    pub fn logout(&self) -> i32 {
        self.client.logout()
    }

    /// Returns the current user id.
    pub fn my_id(&self) -> UserId {
        self.client.my_id()
    }

    /// Returns the account of the current user.
    pub fn my_account(&self) -> Option<UserAccount> {
        self.client.get_my_user_account()
    }

    /// Returns the user type of the current user.
    pub fn my_type(&self) -> u32 {
        self.client.get_my_user_type()
    }

    /// Returns the user rights of the current user.
    pub fn my_rights(&self) -> u32 {
        self.client.get_my_user_rights()
    }

    /// Requests user data for the current user.
    pub fn my_data(&self) -> i32 {
        self.client.get_my_user_data()
    }

    /// Changes the current nickname.
    pub fn change_nickname(&self, nick: &str) -> i32 {
        self.client.change_nickname(nick)
    }

    /// Sets the status and status message.
    pub fn set_status(&self, status: UserStatus, msg: &str) -> i32 {
        self.client.set_status(status, msg)
    }

    /// Updates only the status message.
    pub fn set_status_message(&self, msg: &str) -> i32 {
        self.client.set_status_message(msg)
    }

    /// Kicks a user from a channel.
    pub fn kick(&self, user_id: UserId, channel_id: ChannelId) -> i32 {
        self.client.kick_user(user_id, channel_id)
    }

    /// Bans a user from a channel.
    pub fn ban(&self, user_id: UserId, channel_id: ChannelId) -> i32 {
        self.client.ban_user(user_id, channel_id)
    }

    /// Bans a user with custom ban types.
    pub fn ban_ex(&self, user_id: UserId, ban_types: u32) -> i32 {
        self.client.ban_user_ex(user_id, ban_types)
    }

    /// Removes a ban by IP address.
    pub fn unban(&self, ip: &str, channel_id: ChannelId) -> i32 {
        self.client.unban_user(ip, channel_id)
    }

    /// Sends a text message to a target.
    pub fn send_text(&self, target: impl Into<MessageTarget>, text: &str) -> i32 {
        self.client.send_text(target, text)
    }

    /// Sends a text message to a target using explicit options.
    pub fn send_text_with_options(
        &self,
        target: impl Into<MessageTarget>,
        text: &str,
        options: SendTextOptions,
    ) -> i32 {
        self.client.send_text_with_options(target, text, options)
    }

    /// Sends a text message to a user.
    pub fn send_to_user(&self, user_id: UserId, text: &str) -> i32 {
        self.client.send_to_user(user_id, text)
    }

    /// Sends a text message to a channel.
    pub fn send_to_channel(&self, channel_id: ChannelId, text: &str) -> i32 {
        self.client.send_to_channel(channel_id, text)
    }

    /// Sends a text message to all users.
    pub fn send_to_all(&self, text: &str) -> i32 {
        self.client.send_to_all(text)
    }

    /// Adds a user to the ban list.
    pub fn ban_entry(&self, banned_user: &crate::types::BannedUser) -> i32 {
        self.client.ban(banned_user)
    }

    /// Removes a user from the ban list.
    pub fn unban_entry(&self, banned_user: &crate::types::BannedUser) -> i32 {
        self.client.unban_ex(banned_user)
    }

    /// Returns a user by id.
    pub fn get(&self, user_id: UserId) -> Option<User> {
        self.client.get_user(user_id)
    }

    /// Returns a user by username.
    pub fn get_by_username(&self, username: &str) -> Option<User> {
        self.client.get_user_by_username(username)
    }

    /// Returns all users on the server.
    pub fn list(&self) -> Vec<User> {
        self.client.get_server_users()
    }

    /// Returns user statistics by id.
    pub fn statistics(&self, user_id: UserId) -> Option<UserStatistics> {
        self.client.get_user_statistics(user_id)
    }

    /// Requests a list of user accounts.
    pub fn list_accounts(&self, index: i32, count: i32) -> i32 {
        self.client.list_user_accounts(index, count)
    }

    /// Creates a user account.
    pub fn create_account(&self, account: &UserAccount) -> i32 {
        self.client.create_user_account(account)
    }

    /// Deletes a user account by username.
    pub fn delete_account(&self, username: &str) -> i32 {
        self.client.delete_user_account(username)
    }

    /// Subscribes to a user's streams.
    pub fn subscribe(&self, user_id: UserId, mask: Subscriptions) -> i32 {
        self.client.subscribe(user_id, mask)
    }

    /// Unsubscribes from a user's streams.
    pub fn unsubscribe(&self, user_id: UserId, mask: Subscriptions) -> i32 {
        self.client.unsubscribe(user_id, mask)
    }

    /// Unsubscribes from all streams for a user.
    pub fn unsubscribe_all_from(&self, user_id: UserId) -> i32 {
        self.client.unsubscribe_all_from_user(user_id)
    }

    /// Unsubscribes from all streams for all users.
    pub fn unsubscribe_all(&self) -> i32 {
        self.client.unsubscribe_all()
    }

    /// Manages channel operator status for a user.
    pub fn set_operator(&self, user_id: UserId, channel_id: ChannelId, make_op: bool) -> i32 {
        self.client
            .set_channel_operator(user_id, channel_id, make_op)
    }

    /// Manages channel operator status for a user (with password).
    pub fn set_operator_ex(
        &self,
        user_id: UserId,
        channel_id: ChannelId,
        password: &str,
        make_op: bool,
    ) -> i32 {
        self.client
            .channel_op_ex(user_id, channel_id, password, make_op)
    }

    /// Mutes or unmutes text messages from a user.
    pub fn set_text_mute(&self, user_id: UserId, mute: bool) -> i32 {
        self.client.set_user_text_mute(user_id, mute)
    }

    /// Mutes voice streams from a user.
    pub fn mute_voice(&self, user_id: UserId) -> i32 {
        self.client.mute_user_voice(user_id)
    }

    /// Unmutes voice streams from a user.
    pub fn unmute_voice(&self, user_id: UserId) -> i32 {
        self.client.unmute_user_voice(user_id)
    }

    /// Mutes media file streams from a user.
    pub fn mute_media(&self, user_id: UserId) -> i32 {
        self.client.mute_user_media(user_id)
    }

    /// Unmutes media file streams from a user.
    pub fn unmute_media(&self, user_id: UserId) -> i32 {
        self.client.unmute_user_media(user_id)
    }

    /// Returns the current user's subscription mask.
    pub fn subscriptions(&self) -> Subscriptions {
        self.client.my_subscriptions()
    }
}

#[cfg(feature = "async")]
use super::define_async_namespace;
#[cfg(feature = "async")]
use crate::events::Event;

#[cfg(feature = "async")]
define_async_namespace!(AsyncUsersNamespace);

#[cfg(feature = "async")]
impl AsyncUsersNamespace {
    /// Logs in to the server and waits for confirmation.
    #[cfg(feature = "async")]
    pub async fn login(
        &self,
        nickname: &str,
        username: &str,
        password: &str,
        client_name: &str,
    ) -> Result<User> {
        self.client
            .execute_command(Event::MySelfLoggedIn, || {
                self.client.login(nickname, username, password, client_name)
            })
            .await
    }

    /// Logs in using stored login parameters and waits for confirmation.
    #[cfg(feature = "async")]
    pub async fn login_with_params(&self) -> Result<User> {
        let params = self
            .client
            .login_params()
            .ok_or(crate::events::Error::MissingLoginParams)?;
        self.login(
            &params.nickname,
            &params.username,
            &params.password,
            &params.client_name,
        )
        .await
    }

    /// Logs out from the server and waits for confirmation.
    #[cfg(feature = "async")]
    pub async fn logout(&self) -> Result<()> {
        self.client
            .execute_void_command(Event::MySelfLoggedOut, || self.client.logout())
            .await
    }

    /// Returns the current user id (from cache).
    pub fn my_id(&self) -> UserId {
        self.client.my_id()
    }

    /// Returns the current user's account (from SDK).
    pub fn my_account(&self) -> Option<UserAccount> {
        self.client.get_my_user_account()
    }

    /// Changes nickname and waits for update.
    #[cfg(feature = "async")]
    pub async fn change_nickname(&self, nick: &str) -> Result<User> {
        self.client
            .execute_command(Event::UserUpdate, || self.client.change_nickname(nick))
            .await
    }

    /// Sets status and waits for update.
    #[cfg(feature = "async")]
    pub async fn set_status(&self, status: UserStatus, msg: &str) -> Result<User> {
        self.client
            .execute_command(Event::UserUpdate, || self.client.set_status(status, msg))
            .await
    }

    /// Kicks a user and waits for them to leave.
    #[cfg(feature = "async")]
    pub async fn kick(&self, user_id: UserId, channel_id: ChannelId) -> Result<User> {
        self.client
            .execute_command(Event::UserLeft, || {
                self.client.kick_user(user_id, channel_id)
            })
            .await
    }

    /// Sends a text message.
    pub fn send_text(&self, target: impl Into<MessageTarget>, text: &str) -> i32 {
        self.client.send_text(target, text)
    }

    /// Sends a text message to a user.
    pub fn send_to_user(&self, user_id: UserId, text: &str) -> i32 {
        self.client.send_to_user(user_id, text)
    }

    /// Returns a user by id (from cache).
    pub fn get(&self, user_id: UserId) -> Option<User> {
        self.client.get_user(user_id)
    }

    /// Returns all users on the server (from cache).
    pub fn list(&self) -> Vec<User> {
        self.client.get_server_users()
    }
}
