use super::ClientHooks;
use crate::client::{Client, Message};
use crate::events::Event;
use crate::types::{ChannelId, TextMessage, User};

impl ClientHooks {
    /// Registers a handler for every event.
    pub fn on_event(mut self, hook: impl FnMut(&Client, Event, &Message) + Send + 'static) -> Self {
        self.on_event = Some(Box::new(hook));
        self
    }

    /// Registers a handler for successful connections.
    pub fn on_connect_success(mut self, hook: impl FnMut(&Client) + Send + 'static) -> Self {
        self.on_connect_success = Some(Box::new(hook));
        self
    }

    /// Registers a handler for failed connections.
    pub fn on_connect_failed(mut self, hook: impl FnMut(&Client) + Send + 'static) -> Self {
        self.on_connect_failed = Some(Box::new(hook));
        self
    }

    /// Registers a handler for connection encryption errors.
    pub fn on_connect_crypt_error(mut self, hook: impl FnMut(&Client) + Send + 'static) -> Self {
        self.on_connect_crypt_error = Some(Box::new(hook));
        self
    }

    /// Registers a handler for max payload updates.
    pub fn on_connect_max_payload_updated(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_connect_max_payload_updated = Some(Box::new(hook));
        self
    }

    /// Registers a handler for connection loss.
    pub fn on_connection_lost(mut self, hook: impl FnMut(&Client) + Send + 'static) -> Self {
        self.on_connection_lost = Some(Box::new(hook));
        self
    }

    /// Registers a handler for command processing notifications.
    pub fn on_cmd_processing(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_cmd_processing = Some(Box::new(hook));
        self
    }

    /// Registers a handler for command errors.
    pub fn on_cmd_error(mut self, hook: impl FnMut(&Client, &Message) + Send + 'static) -> Self {
        self.on_cmd_error = Some(Box::new(hook));
        self
    }

    /// Registers a handler for command success notifications.
    pub fn on_cmd_success(mut self, hook: impl FnMut(&Client, &Message) + Send + 'static) -> Self {
        self.on_cmd_success = Some(Box::new(hook));
        self
    }

    /// Registers a handler for successful login.
    pub fn on_logged_in(mut self, hook: impl FnMut(&Client) + Send + 'static) -> Self {
        self.on_logged_in = Some(Box::new(hook));
        self
    }

    /// Registers a handler for logout.
    pub fn on_logged_out(mut self, hook: impl FnMut(&Client) + Send + 'static) -> Self {
        self.on_logged_out = Some(Box::new(hook));
        self
    }

    /// Registers a handler for being kicked.
    pub fn on_myself_kicked(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_myself_kicked = Some(Box::new(hook));
        self
    }

    /// Registers a handler for user login events.
    pub fn on_user_logged_in(mut self, hook: impl FnMut(&Client, User) + Send + 'static) -> Self {
        self.on_user_logged_in = Some(Box::new(hook));
        self
    }

    /// Registers a handler for user logout events.
    pub fn on_user_logged_out(mut self, hook: impl FnMut(&Client, User) + Send + 'static) -> Self {
        self.on_user_logged_out = Some(Box::new(hook));
        self
    }

    /// Registers a handler for user updates.
    pub fn on_user_update(mut self, hook: impl FnMut(&Client, User) + Send + 'static) -> Self {
        self.on_user_update = Some(Box::new(hook));
        self
    }

    /// Registers a handler for channel joins.
    pub fn on_joined(mut self, hook: impl FnMut(&Client, ChannelId) + Send + 'static) -> Self {
        self.on_joined = Some(Box::new(hook));
        self
    }

    /// Registers a handler for any user join event.
    pub fn on_user_joined(mut self, hook: impl FnMut(&Client, User) + Send + 'static) -> Self {
        self.on_user_joined = Some(Box::new(hook));
        self
    }

    /// Registers a handler for any user leave event.
    pub fn on_user_left(mut self, hook: impl FnMut(&Client, User) + Send + 'static) -> Self {
        self.on_user_left = Some(Box::new(hook));
        self
    }

    /// Registers a handler for channel or user text messages.
    pub fn on_text_message(
        mut self,
        hook: impl FnMut(&Client, TextMessage) + Send + 'static,
    ) -> Self {
        self.on_text_message = Some(Box::new(hook));
        self
    }

    /// Registers a handler for channel creation events.
    pub fn on_channel_created(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_channel_created = Some(Box::new(hook));
        self
    }

    /// Registers a handler for channel update events.
    pub fn on_channel_updated(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_channel_updated = Some(Box::new(hook));
        self
    }

    /// Registers a handler for channel removal events.
    pub fn on_channel_removed(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_channel_removed = Some(Box::new(hook));
        self
    }

    /// Registers a handler for server updates.
    pub fn on_server_update(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_server_update = Some(Box::new(hook));
        self
    }

    /// Registers a handler for server statistics updates.
    pub fn on_server_statistics(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_server_statistics = Some(Box::new(hook));
        self
    }

    /// Registers a handler for new file events.
    pub fn on_file_new(mut self, hook: impl FnMut(&Client, &Message) + Send + 'static) -> Self {
        self.on_file_new = Some(Box::new(hook));
        self
    }

    /// Registers a handler for file removal events.
    pub fn on_file_remove(mut self, hook: impl FnMut(&Client, &Message) + Send + 'static) -> Self {
        self.on_file_remove = Some(Box::new(hook));
        self
    }

    /// Registers a handler for user account events.
    pub fn on_user_account(mut self, hook: impl FnMut(&Client, &Message) + Send + 'static) -> Self {
        self.on_user_account = Some(Box::new(hook));
        self
    }

    /// Registers a handler for banned user events.
    pub fn on_banned_user(mut self, hook: impl FnMut(&Client, &Message) + Send + 'static) -> Self {
        self.on_banned_user = Some(Box::new(hook));
        self
    }

    /// Registers a handler for user account creation events.
    pub fn on_user_account_created(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_user_account_created = Some(Box::new(hook));
        self
    }

    /// Registers a handler for user account removal events.
    pub fn on_user_account_removed(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_user_account_removed = Some(Box::new(hook));
        self
    }

    /// Registers a handler for user state changes.
    pub fn on_user_state_change(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_user_state_change = Some(Box::new(hook));
        self
    }

    /// Registers a handler for video capture frames.
    pub fn on_video_capture_frame(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_video_capture_frame = Some(Box::new(hook));
        self
    }

    /// Registers a handler for media file video frames.
    pub fn on_media_file_video(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_media_file_video = Some(Box::new(hook));
        self
    }

    /// Registers a handler for desktop window updates.
    pub fn on_desktop_window(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_desktop_window = Some(Box::new(hook));
        self
    }

    /// Registers a handler for desktop cursor updates.
    pub fn on_desktop_cursor(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_desktop_cursor = Some(Box::new(hook));
        self
    }

    /// Registers a handler for desktop input updates.
    pub fn on_desktop_input(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_desktop_input = Some(Box::new(hook));
        self
    }

    /// Registers a handler for recorded media file events.
    pub fn on_user_record_media_file(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_user_record_media_file = Some(Box::new(hook));
        self
    }

    /// Registers a handler for audio block events.
    pub fn on_audio_block(mut self, hook: impl FnMut(&Client, &Message) + Send + 'static) -> Self {
        self.on_audio_block = Some(Box::new(hook));
        self
    }

    /// Registers a handler for internal error events.
    pub fn on_internal_error(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_internal_error = Some(Box::new(hook));
        self
    }

    /// Registers a handler for voice activation events.
    pub fn on_voice_activation(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_voice_activation = Some(Box::new(hook));
        self
    }

    /// Registers a handler for hotkey events.
    pub fn on_hotkey(mut self, hook: impl FnMut(&Client, &Message) + Send + 'static) -> Self {
        self.on_hotkey = Some(Box::new(hook));
        self
    }

    /// Registers a handler for hotkey test events.
    pub fn on_hotkey_test(mut self, hook: impl FnMut(&Client, &Message) + Send + 'static) -> Self {
        self.on_hotkey_test = Some(Box::new(hook));
        self
    }

    /// Registers a handler for file transfer events.
    pub fn on_file_transfer(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_file_transfer = Some(Box::new(hook));
        self
    }

    /// Registers a handler for desktop window transfer events.
    pub fn on_desktop_window_transfer(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_desktop_window_transfer = Some(Box::new(hook));
        self
    }

    /// Registers a handler for stream media file events.
    pub fn on_stream_media_file(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_stream_media_file = Some(Box::new(hook));
        self
    }

    /// Registers a handler for local media file events.
    pub fn on_local_media_file(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_local_media_file = Some(Box::new(hook));
        self
    }

    /// Registers a handler for audio input events.
    pub fn on_audio_input(mut self, hook: impl FnMut(&Client, &Message) + Send + 'static) -> Self {
        self.on_audio_input = Some(Box::new(hook));
        self
    }

    /// Registers a handler for first voice stream packet events.
    pub fn on_user_first_voice_stream_packet(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_user_first_voice_stream_packet = Some(Box::new(hook));
        self
    }

    /// Registers a handler for sound device added events.
    pub fn on_sound_device_added(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_sound_device_added = Some(Box::new(hook));
        self
    }

    /// Registers a handler for sound device removed events.
    pub fn on_sound_device_removed(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_sound_device_removed = Some(Box::new(hook));
        self
    }

    /// Registers a handler for sound device unplugged events.
    pub fn on_sound_device_unplugged(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_sound_device_unplugged = Some(Box::new(hook));
        self
    }

    /// Registers a handler for default sound input device changes.
    pub fn on_sound_device_new_default_input(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_sound_device_new_default_input = Some(Box::new(hook));
        self
    }

    /// Registers a handler for default sound output device changes.
    pub fn on_sound_device_new_default_output(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_sound_device_new_default_output = Some(Box::new(hook));
        self
    }

    /// Registers a handler for default sound input communications device changes.
    pub fn on_sound_device_new_default_input_com_device(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_sound_device_new_default_input_com_device = Some(Box::new(hook));
        self
    }

    /// Registers a handler for default sound output communications device changes.
    pub fn on_sound_device_new_default_output_com_device(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_sound_device_new_default_output_com_device = Some(Box::new(hook));
        self
    }

    /// Registers a handler for reconnecting notifications.
    pub fn on_reconnecting(mut self, hook: impl FnMut(&Client, &Message) + Send + 'static) -> Self {
        self.on_reconnecting = Some(Box::new(hook));
        self
    }

    /// Registers a handler before an automatic reconnect attempt.
    pub fn on_before_reconnect(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_before_reconnect = Some(Box::new(hook));
        self
    }

    /// Registers a handler after an automatic reconnect succeeds.
    pub fn on_after_reconnect(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_after_reconnect = Some(Box::new(hook));
        self
    }

    /// Registers a handler when automatic reconnect gives up.
    pub fn on_reconnect_failed(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_reconnect_failed = Some(Box::new(hook));
        self
    }

    /// Registers a handler before an automatic login retry.
    pub fn on_before_auto_login(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_before_auto_login = Some(Box::new(hook));
        self
    }

    /// Registers a handler when automatic login gives up.
    pub fn on_auto_login_failed(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_auto_login_failed = Some(Box::new(hook));
        self
    }

    /// Registers a handler before an automatic join retry.
    pub fn on_before_auto_join(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_before_auto_join = Some(Box::new(hook));
        self
    }

    /// Registers a handler when automatic join gives up.
    pub fn on_auto_join_failed(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_auto_join_failed = Some(Box::new(hook));
        self
    }

    /// Registers a handler after full in-session recovery reaches Joined.
    pub fn on_auto_recover_completed(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_auto_recover_completed = Some(Box::new(hook));
        self
    }
}
