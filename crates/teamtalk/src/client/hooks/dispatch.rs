use super::ClientHooks;
use crate::client::{Client, Message};
use crate::events::Event;
use crate::types::ChannelId;

impl ClientHooks {
    #[allow(clippy::too_many_lines)]
    pub(crate) fn fire(&mut self, client: &Client, event: Event, msg: &Message) {
        match event {
            Event::ConnectSuccess => {
                if let Some(hook) = self.on_connect_success.as_mut() {
                    hook(client);
                }
            }
            Event::ConnectFailed => {
                if let Some(hook) = self.on_connect_failed.as_mut() {
                    hook(client);
                }
            }
            Event::ConnectCryptError => {
                if let Some(hook) = self.on_connect_crypt_error.as_mut() {
                    hook(client);
                }
            }
            Event::ConnectMaxPayloadUpdated => {
                if let Some(hook) = self.on_connect_max_payload_updated.as_mut() {
                    hook(client, msg);
                }
            }
            Event::ConnectionLost => {
                if let Some(hook) = self.on_connection_lost.as_mut() {
                    hook(client);
                }
            }
            Event::CmdProcessing => {
                if let Some(hook) = self.on_cmd_processing.as_mut() {
                    hook(client, msg);
                }
            }
            Event::CmdError => {
                if let Some(hook) = self.on_cmd_error.as_mut() {
                    hook(client, msg);
                }
            }
            Event::CmdSuccess => {
                if let Some(hook) = self.on_cmd_success.as_mut() {
                    hook(client, msg);
                }
            }
            Event::MySelfLoggedIn => {
                if let Some(hook) = self.on_logged_in.as_mut() {
                    hook(client);
                }
            }
            Event::MySelfLoggedOut => {
                if let Some(hook) = self.on_logged_out.as_mut() {
                    hook(client);
                }
            }
            Event::MySelfKicked => {
                if let Some(hook) = self.on_myself_kicked.as_mut() {
                    hook(client, msg);
                }
            }
            Event::UserLoggedIn => {
                if let Some(user) = msg.user()
                    && let Some(hook) = self.on_user_logged_in.as_mut()
                {
                    hook(client, user);
                }
            }
            Event::UserLoggedOut => {
                if let Some(user) = msg.user()
                    && let Some(hook) = self.on_user_logged_out.as_mut()
                {
                    hook(client, user);
                }
            }
            Event::UserUpdate => {
                if let Some(user) = msg.user()
                    && let Some(hook) = self.on_user_update.as_mut()
                {
                    hook(client, user);
                }
            }
            Event::UserJoined => {
                if let Some(user) = msg.user()
                    && let Some(hook) = self.on_user_joined.as_mut()
                {
                    hook(client, user);
                }
            }
            Event::UserLeft => {
                if let Some(user) = msg.user()
                    && let Some(hook) = self.on_user_left.as_mut()
                {
                    hook(client, user);
                }
            }
            Event::TextMessage => {
                if let Some(text) = msg.text()
                    && let Some(hook) = self.on_text_message.as_mut()
                {
                    hook(client, text);
                }
            }
            Event::ChannelCreated => {
                if let Some(hook) = self.on_channel_created.as_mut() {
                    hook(client, msg);
                }
            }
            Event::ChannelUpdated => {
                if let Some(hook) = self.on_channel_updated.as_mut() {
                    hook(client, msg);
                }
            }
            Event::ChannelRemoved => {
                if let Some(hook) = self.on_channel_removed.as_mut() {
                    hook(client, msg);
                }
            }
            Event::ServerUpdate => {
                if let Some(hook) = self.on_server_update.as_mut() {
                    hook(client, msg);
                }
            }
            Event::ServerStatistics => {
                if let Some(hook) = self.on_server_statistics.as_mut() {
                    hook(client, msg);
                }
            }
            Event::FileNew => {
                if let Some(hook) = self.on_file_new.as_mut() {
                    hook(client, msg);
                }
            }
            Event::FileRemove => {
                if let Some(hook) = self.on_file_remove.as_mut() {
                    hook(client, msg);
                }
            }
            Event::UserAccount => {
                if let Some(hook) = self.on_user_account.as_mut() {
                    hook(client, msg);
                }
            }
            Event::BannedUser => {
                if let Some(hook) = self.on_banned_user.as_mut() {
                    hook(client, msg);
                }
            }
            Event::UserAccountCreated => {
                if let Some(hook) = self.on_user_account_created.as_mut() {
                    hook(client, msg);
                }
            }
            Event::UserAccountRemoved => {
                if let Some(hook) = self.on_user_account_removed.as_mut() {
                    hook(client, msg);
                }
            }
            Event::UserStateChange => {
                if let Some(hook) = self.on_user_state_change.as_mut() {
                    hook(client, msg);
                }
            }
            Event::VideoCaptureFrame => {
                if let Some(hook) = self.on_video_capture_frame.as_mut() {
                    hook(client, msg);
                }
            }
            Event::MediaFileVideo => {
                if let Some(hook) = self.on_media_file_video.as_mut() {
                    hook(client, msg);
                }
            }
            Event::DesktopWindow => {
                if let Some(hook) = self.on_desktop_window.as_mut() {
                    hook(client, msg);
                }
            }
            Event::DesktopCursor => {
                if let Some(hook) = self.on_desktop_cursor.as_mut() {
                    hook(client, msg);
                }
            }
            Event::DesktopInput => {
                if let Some(hook) = self.on_desktop_input.as_mut() {
                    hook(client, msg);
                }
            }
            Event::UserRecordMediaFile => {
                if let Some(hook) = self.on_user_record_media_file.as_mut() {
                    hook(client, msg);
                }
            }
            Event::AudioBlock => {
                if let Some(hook) = self.on_audio_block.as_mut() {
                    hook(client, msg);
                }
            }
            Event::InternalError => {
                if let Some(hook) = self.on_internal_error.as_mut() {
                    hook(client, msg);
                }
            }
            Event::VoiceActivation => {
                if let Some(hook) = self.on_voice_activation.as_mut() {
                    hook(client, msg);
                }
            }
            Event::Hotkey => {
                if let Some(hook) = self.on_hotkey.as_mut() {
                    hook(client, msg);
                }
            }
            Event::HotkeyTest => {
                if let Some(hook) = self.on_hotkey_test.as_mut() {
                    hook(client, msg);
                }
            }
            Event::FileTransfer => {
                if let Some(hook) = self.on_file_transfer.as_mut() {
                    hook(client, msg);
                }
            }
            Event::DesktopWindowTransfer => {
                if let Some(hook) = self.on_desktop_window_transfer.as_mut() {
                    hook(client, msg);
                }
            }
            Event::StreamMediaFile => {
                if let Some(hook) = self.on_stream_media_file.as_mut() {
                    hook(client, msg);
                }
            }
            Event::LocalMediaFile => {
                if let Some(hook) = self.on_local_media_file.as_mut() {
                    hook(client, msg);
                }
            }
            Event::AudioInput => {
                if let Some(hook) = self.on_audio_input.as_mut() {
                    hook(client, msg);
                }
            }
            Event::UserFirstVoiceStreamPacket => {
                if let Some(hook) = self.on_user_first_voice_stream_packet.as_mut() {
                    hook(client, msg);
                }
            }
            Event::SoundDeviceAdded => {
                if let Some(hook) = self.on_sound_device_added.as_mut() {
                    hook(client, msg);
                }
            }
            Event::SoundDeviceRemoved => {
                if let Some(hook) = self.on_sound_device_removed.as_mut() {
                    hook(client, msg);
                }
            }
            Event::SoundDeviceUnplugged => {
                if let Some(hook) = self.on_sound_device_unplugged.as_mut() {
                    hook(client, msg);
                }
            }
            Event::SoundDeviceNewDefaultInput => {
                if let Some(hook) = self.on_sound_device_new_default_input.as_mut() {
                    hook(client, msg);
                }
            }
            Event::SoundDeviceNewDefaultOutput => {
                if let Some(hook) = self.on_sound_device_new_default_output.as_mut() {
                    hook(client, msg);
                }
            }
            Event::SoundDeviceNewDefaultInputComDevice => {
                if let Some(hook) = self.on_sound_device_new_default_input_com_device.as_mut() {
                    hook(client, msg);
                }
            }
            Event::SoundDeviceNewDefaultOutputComDevice => {
                if let Some(hook) = self.on_sound_device_new_default_output_com_device.as_mut() {
                    hook(client, msg);
                }
            }
            Event::BeforeReconnect { .. } => {
                if let Some(hook) = self.on_before_reconnect.as_mut() {
                    hook(client, msg);
                }
            }
            Event::Reconnecting { .. } => {
                if let Some(hook) = self.on_reconnecting.as_mut() {
                    hook(client, msg);
                }
            }
            Event::AfterReconnect { .. } => {
                if let Some(hook) = self.on_after_reconnect.as_mut() {
                    hook(client, msg);
                }
            }
            Event::ReconnectFailed { .. } => {
                if let Some(hook) = self.on_reconnect_failed.as_mut() {
                    hook(client, msg);
                }
            }
            Event::BeforeAutoLogin { .. } => {
                if let Some(hook) = self.on_before_auto_login.as_mut() {
                    hook(client, msg);
                }
            }
            Event::AutoLoginFailed { .. } => {
                if let Some(hook) = self.on_auto_login_failed.as_mut() {
                    hook(client, msg);
                }
            }
            Event::BeforeAutoJoin { .. } => {
                if let Some(hook) = self.on_before_auto_join.as_mut() {
                    hook(client, msg);
                }
            }
            Event::AutoJoinFailed { .. } => {
                if let Some(hook) = self.on_auto_join_failed.as_mut() {
                    hook(client, msg);
                }
            }
            Event::AutoRecoverCompleted { .. } => {
                if let Some(hook) = self.on_auto_recover_completed.as_mut() {
                    hook(client, msg);
                }
            }
            _ => {}
        }

        if let Some(hook) = self.on_event.as_mut() {
            hook(client, event, msg);
        }
    }

    pub(crate) fn fire_joined(&mut self, client: &Client, channel_id: ChannelId) {
        if let Some(hook) = self.on_joined.as_mut() {
            hook(client, channel_id);
        }
    }
}
