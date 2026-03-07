use super::*;

impl Client {
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

    pub fn set_user_text_mute(&self, user_id: UserId, mute: bool) -> i32 {
        if mute {
            self.unsubscribe(user_id, Subscriptions::all_text())
        } else {
            self.subscribe(user_id, Subscriptions::all_text())
        }
    }

    /// Mutes text messages from a user (local subscription).
    pub fn mute_user_text(&self, user_id: UserId) -> i32 {
        self.set_user_text_mute(user_id, true)
    }

    /// Unmutes text messages from a user (local subscription).
    pub fn unmute_user_text(&self, user_id: UserId) -> i32 {
        self.set_user_text_mute(user_id, false)
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
}
