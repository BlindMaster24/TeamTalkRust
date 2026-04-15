use super::*;
use crate::types::{AudioInputProgress, BannedUser, DesktopInput, MediaFileInfo, RemoteFile};

impl ScriptManager {
    pub(super) fn event_table(&self, event: Event, message: &Message) -> mlua::Result<Table> {
        let table = self.lua.create_table()?;
        table.set("type", event_name(event))?;
        table.set("source", message.source())?;
        match event {
            Event::TextMessage => {
                if let Some(text) = message.text() {
                    table.set("text", self.text_table(&text)?)?;
                }
            }
            Event::UserLoggedIn
            | Event::UserLoggedOut
            | Event::UserUpdate
            | Event::UserJoined
            | Event::UserLeft
            | Event::UserStateChange
            | Event::UserFirstVoiceStreamPacket => {
                if let Some(user) = message.user() {
                    table.set("user", self.user_table(&user)?)?;
                }
            }
            Event::ChannelCreated | Event::ChannelUpdated | Event::ChannelRemoved => {
                if let Some(channel) = message.channel() {
                    table.set("channel", self.channel_table(&channel)?)?;
                }
            }
            Event::ServerUpdate => {
                if let Some(props) = message.server_properties() {
                    table.set("server_properties", self.server_properties_table(&props)?)?;
                }
            }
            Event::ServerStatistics => {
                if let Some(stats) = message.server_statistics() {
                    table.set("server_statistics", self.server_statistics_table(&stats)?)?;
                }
            }
            Event::FileTransfer => {
                if let Some(file_transfer) = message.file_transfer() {
                    table.set("file_transfer", self.file_transfer_table(&file_transfer)?)?;
                }
            }
            Event::FileNew | Event::FileRemove => {
                if let Some(remote_file) = message.remote_file() {
                    table.set("remote_file", self.remote_file_table(&remote_file)?)?;
                }
            }
            Event::BannedUser => {
                if let Some(banned_user) = message.banned_user() {
                    table.set("banned_user", self.banned_user_table(&banned_user)?)?;
                }
            }
            Event::UserAccount | Event::UserAccountCreated | Event::UserAccountRemoved => {
                if let Some(account) = message.account() {
                    table.set("account", self.account_table(&account)?)?;
                }
            }
            Event::DesktopInput => {
                if let Some(desktop_input) = message.desktop_input() {
                    table.set("desktop_input", self.desktop_input_table(&desktop_input)?)?;
                }
            }
            Event::StreamMediaFile | Event::LocalMediaFile => {
                if let Some(media_file_info) = message.media_file_info() {
                    table.set(
                        "media_file_info",
                        self.media_file_info_table(&media_file_info)?,
                    )?;
                }
            }
            Event::AudioInput => {
                if let Some(audio_input) = message.audio_input_progress() {
                    table.set(
                        "audio_input_progress",
                        self.audio_input_progress_table(&audio_input)?,
                    )?;
                }
            }
            Event::None
            | Event::ConnectSuccess
            | Event::ConnectCryptError
            | Event::ConnectFailed
            | Event::ConnectionLost
            | Event::ConnectMaxPayloadUpdated
            | Event::CmdProcessing
            | Event::CmdError
            | Event::CmdSuccess
            | Event::MySelfLoggedIn
            | Event::MySelfLoggedOut
            | Event::MySelfKicked
            | Event::VideoCaptureFrame
            | Event::MediaFileVideo
            | Event::DesktopWindow
            | Event::DesktopCursor
            | Event::UserRecordMediaFile
            | Event::AudioBlock
            | Event::InternalError
            | Event::VoiceActivation
            | Event::Hotkey
            | Event::HotkeyTest
            | Event::DesktopWindowTransfer
            | Event::SoundDeviceAdded
            | Event::SoundDeviceRemoved
            | Event::SoundDeviceUnplugged
            | Event::SoundDeviceNewDefaultInput
            | Event::SoundDeviceNewDefaultOutput
            | Event::SoundDeviceNewDefaultInputComDevice
            | Event::SoundDeviceNewDefaultOutputComDevice
            | Event::BeforeReconnect { .. }
            | Event::Reconnecting { .. }
            | Event::AfterReconnect { .. }
            | Event::ReconnectFailed { .. }
            | Event::BeforeAutoLogin { .. }
            | Event::AutoLoginFailed { .. }
            | Event::BeforeAutoJoin { .. }
            | Event::AutoJoinFailed { .. }
            | Event::AutoRecoverCompleted { .. }
            | Event::Unknown(_) => {}
        }
        Ok(table)
    }

    pub(super) fn text_table(&self, msg: &TextMessage) -> mlua::Result<Table> {
        let table = self.lua.create_table()?;
        table.set("msg_type", msg.msg_type as i32)?;
        table.set("from_id", msg.from_id.raw())?;
        table.set("from_username", msg.from_username.clone())?;
        table.set("to_id", msg.to_id.raw())?;
        table.set("channel_id", msg.channel_id.raw())?;
        table.set("text", msg.text.clone())?;
        table.set("more", msg.more)?;
        Ok(table)
    }

    pub(super) fn user_table(&self, user: &User) -> mlua::Result<Table> {
        let table = self.lua.create_table()?;
        table.set("id", user.id.raw())?;
        table.set("username", user.username.clone())?;
        table.set("nickname", user.nickname.clone())?;
        table.set("user_data", user.user_data)?;
        table.set("user_type", user.user_type as i64)?;
        table.set("ip_address", user.ip_address.clone())?;
        table.set("version", user.version as i64)?;
        table.set("channel_id", user.channel_id.raw())?;
        table.set("status_mode", user.status.to_bits() as i64)?;
        table.set("status_msg", user.status_msg.clone())?;
        table.set("state", user.state.raw() as i64)?;
        table.set("local_subscriptions", user.local_subscriptions.raw() as i64)?;
        table.set("peer_subscriptions", user.peer_subscriptions.raw() as i64)?;
        table.set("media_storage_dir", user.media_storage_dir.clone())?;
        table.set("volume_voice", user.volume_voice)?;
        table.set("volume_media", user.volume_media)?;
        table.set("stopped_delay_voice", user.stopped_delay_voice)?;
        table.set("stopped_delay_media", user.stopped_delay_media)?;
        table.set("sound_position_voice", user.sound_position_voice.to_vec())?;
        table.set("sound_position_media", user.sound_position_media.to_vec())?;
        table.set(
            "stereo_playback_voice",
            vec![user.stereo_playback_voice[0], user.stereo_playback_voice[1]],
        )?;
        table.set(
            "stereo_playback_media",
            vec![user.stereo_playback_media[0], user.stereo_playback_media[1]],
        )?;
        table.set("buf_ms_voice", user.buf_ms_voice)?;
        table.set("buf_ms_media", user.buf_ms_media)?;
        table.set("active_adaptive_delay", user.active_adaptive_delay)?;
        table.set("client_name", user.client_name.clone())?;
        Ok(table)
    }

    pub(super) fn channel_table(&self, channel: &Channel) -> mlua::Result<Table> {
        let table = self.lua.create_table()?;
        table.set("id", channel.id.raw())?;
        table.set("parent_id", channel.parent_id.raw())?;
        table.set("name", channel.name.clone())?;
        table.set("topic", channel.topic.clone())?;
        table.set("channel_type", channel.channel_type.raw() as i64)?;
        table.set("has_password", channel.has_password)?;
        table.set("user_data", channel.user_data)?;
        table.set("disk_quota", channel.disk_quota)?;
        table.set("max_users", channel.max_users)?;
        table.set("queue_delay_ms", channel.queue_delay_ms)?;
        table.set("timeout_voice_ms", channel.timeout_voice_ms)?;
        table.set("timeout_media_ms", channel.timeout_media_ms)?;
        let tx = self.lua.create_table()?;
        for (idx, (user_id, stream_type)) in channel.transmit_users.iter().enumerate() {
            let entry = self.lua.create_table()?;
            entry.set("user_id", user_id.raw())?;
            entry.set("stream_type", *stream_type as i64)?;
            tx.set(idx + 1, entry)?;
        }
        table.set("transmit_users", tx)?;
        table.set(
            "transmit_users_queue",
            channel
                .transmit_users_queue
                .iter()
                .map(|id| id.raw())
                .collect::<Vec<_>>(),
        )?;
        Ok(table)
    }

    pub(super) fn file_transfer_table(&self, transfer: &FileTransfer) -> mlua::Result<Table> {
        let table = self.lua.create_table()?;
        table.set("id", transfer.id.raw())?;
        table.set("channel_id", transfer.channel_id.raw())?;
        table.set("local_path", transfer.local_path.clone())?;
        table.set("remote_name", transfer.remote_name.clone())?;
        table.set("size", transfer.size)?;
        table.set("transferred", transfer.transferred)?;
        table.set("status", transfer.status as i64)?;
        table.set("inbound", transfer.inbound)?;
        Ok(table)
    }

    pub(super) fn remote_file_table(&self, file: &RemoteFile) -> mlua::Result<Table> {
        let table = self.lua.create_table()?;
        table.set("channel_id", file.channel_id.raw())?;
        table.set("id", file.id.raw())?;
        table.set("name", file.name.clone())?;
        table.set("size", file.size)?;
        table.set("owner", file.owner.clone())?;
        table.set("upload_time", file.upload_time.clone())?;
        Ok(table)
    }

    pub(super) fn server_properties_table(&self, props: &ServerProperties) -> mlua::Result<Table> {
        let table = self.lua.create_table()?;
        table.set("name", props.name.clone())?;
        table.set("motd", props.motd.clone())?;
        table.set("motd_raw", props.motd_raw.clone())?;
        table.set("max_users", props.max_users)?;
        table.set("max_login_attempts", props.max_login_attempts)?;
        table.set("max_logins_per_ip", props.max_logins_per_ip)?;
        table.set("max_voice_tx", props.max_voice_tx)?;
        table.set("max_video_tx", props.max_video_tx)?;
        table.set("max_media_tx", props.max_media_tx)?;
        table.set("max_desktop_tx", props.max_desktop_tx)?;
        table.set("max_total_tx", props.max_total_tx)?;
        table.set("user_timeout", props.user_timeout)?;
        table.set("auto_save", props.auto_save)?;
        table.set("tcp_port", props.tcp_port)?;
        table.set("udp_port", props.udp_port)?;
        table.set("version", props.version.clone())?;
        table.set("protocol_version", props.protocol_version.clone())?;
        table.set("login_delay", props.login_delay)?;
        table.set("access_token", props.access_token.clone())?;
        table.set("log_events", props.log_events as i64)?;
        Ok(table)
    }

    pub(super) fn server_statistics_table(&self, stats: &ServerStatistics) -> mlua::Result<Table> {
        let table = self.lua.create_table()?;
        table.set("total_tx", stats.total_tx)?;
        table.set("total_rx", stats.total_rx)?;
        table.set("voice_tx", stats.voice_tx)?;
        table.set("voice_rx", stats.voice_rx)?;
        table.set("video_tx", stats.video_tx)?;
        table.set("video_rx", stats.video_rx)?;
        table.set("media_tx", stats.media_tx)?;
        table.set("media_rx", stats.media_rx)?;
        table.set("desktop_tx", stats.desktop_tx)?;
        table.set("desktop_rx", stats.desktop_rx)?;
        table.set("users_served", stats.users_served)?;
        table.set("users_peak", stats.users_peak)?;
        table.set("files_tx", stats.files_tx)?;
        table.set("files_rx", stats.files_rx)?;
        table.set("uptime_ms", stats.uptime_ms)?;
        Ok(table)
    }

    pub(super) fn account_table(&self, account: &UserAccount) -> mlua::Result<Table> {
        let table = self.lua.create_table()?;
        table.set("username", account.username.clone())?;
        table.set("password", account.password.clone())?;
        table.set("user_type", account.user_type as i64)?;
        table.set("user_rights", account.user_rights as i64)?;
        table.set("note", account.note.clone())?;
        table.set("init_channel", account.init_channel.clone())?;
        table.set("user_data", account.user_data)?;
        table.set(
            "auto_operator_channels",
            account
                .auto_operator_channels
                .iter()
                .map(|id| id.raw())
                .collect::<Vec<_>>(),
        )?;
        table.set("audio_codec_bps_limit", account.audio_codec_bps_limit)?;
        Ok(table)
    }

    pub(super) fn banned_user_table(&self, banned_user: &BannedUser) -> mlua::Result<Table> {
        let table = self.lua.create_table()?;
        table.set("ip", banned_user.ip.clone())?;
        table.set("channel_path", banned_user.channel_path.clone())?;
        table.set("nickname", banned_user.nickname.clone())?;
        table.set("username", banned_user.username.clone())?;
        table.set("ban_time", banned_user.ban_time.clone())?;
        table.set("ban_types", banned_user.ban_types as i64)?;
        table.set("owner", banned_user.owner.clone())?;
        Ok(table)
    }

    pub(super) fn desktop_input_table(&self, input: &DesktopInput) -> mlua::Result<Table> {
        let table = self.lua.create_table()?;
        table.set("mouse_pos_x", input.mouse_pos_x)?;
        table.set("mouse_pos_y", input.mouse_pos_y)?;
        table.set("key_code", input.key_code as i64)?;
        table.set("key_state", input.key_state as i64)?;
        Ok(table)
    }

    pub(super) fn media_file_info_table(&self, info: &MediaFileInfo) -> mlua::Result<Table> {
        let table = self.lua.create_table()?;
        table.set("status", info.status as i64)?;
        table.set("name", info.name.clone())?;
        table.set("duration_ms", info.duration_ms)?;
        table.set("elapsed_ms", info.elapsed_ms)?;
        let audio_fmt = self.lua.create_table()?;
        audio_fmt.set("format", info.audio_fmt.nAudioFmt as i64)?;
        audio_fmt.set("sample_rate", info.audio_fmt.nSampleRate)?;
        audio_fmt.set("channels", info.audio_fmt.nChannels)?;
        table.set("audio_format", audio_fmt)?;
        let video_fmt = self.lua.create_table()?;
        video_fmt.set("width", info.video_fmt.nWidth)?;
        video_fmt.set("height", info.video_fmt.nHeight)?;
        video_fmt.set("fps_numerator", info.video_fmt.nFPS_Numerator)?;
        video_fmt.set("fps_denominator", info.video_fmt.nFPS_Denominator)?;
        video_fmt.set("fourcc", info.video_fmt.picFourCC as i64)?;
        table.set("video_format", video_fmt)?;
        Ok(table)
    }

    pub(super) fn audio_input_progress_table(
        &self,
        progress: &AudioInputProgress,
    ) -> mlua::Result<Table> {
        let table = self.lua.create_table()?;
        table.set("stream_id", progress.stream_id)?;
        table.set("queue_ms", progress.queue_ms)?;
        table.set("elapsed_ms", progress.elapsed_ms)?;
        Ok(table)
    }
}
