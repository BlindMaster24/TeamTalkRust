use super::ffi;

/// Wrapper around a raw `TeamTalk` message with its originating event.
pub struct Message {
    event: crate::events::Event,
    raw: ffi::TTMessage,
}

/// Typed payload extracted from a `TeamTalk` message.
#[non_exhaustive]
pub enum EventData {
    TextMessage(crate::types::TextMessage),
    Channel(crate::types::Channel),
    ServerProperties(crate::types::ServerProperties),
    ServerStatistics(crate::types::ServerStatistics),
    FileTransfer(crate::types::FileTransfer),
    RemoteFile(crate::types::RemoteFile),
    User(crate::types::User),
    BannedUser(crate::types::BannedUser),
    UserAccount(crate::types::UserAccount),
    DesktopInput(crate::types::DesktopInput),
    MediaFileInfo(crate::types::MediaFileInfo),
    AudioInputProgress(crate::types::AudioInputProgress),
    ErrorMessage(crate::types::ErrorMessage),
}

impl Message {
    fn has_tt_type(&self, expected: ffi::TTType) -> bool {
        self.raw.ttType == expected
    }

    /// Wraps a raw `TeamTalk` message.
    #[allow(clippy::large_types_passed_by_value)]
    pub(crate) fn from_raw(event: crate::events::Event, raw: ffi::TTMessage) -> Self {
        Self { event, raw }
    }

    /// Returns the originating event for this message.
    #[must_use]
    pub fn event(&self) -> crate::events::Event {
        self.event
    }

    /// Returns the source user id for the message.
    #[must_use]
    pub fn source(&self) -> i32 {
        self.raw.nSource
    }

    /// Returns the text message payload if present.
    #[must_use]
    pub fn text(&self) -> Option<crate::types::TextMessage> {
        if matches!(self.event, crate::events::Event::TextMessage)
            && self.has_tt_type(ffi::TTType::__TEXTMESSAGE)
        {
            unsafe {
                Some(crate::types::TextMessage::from(
                    self.raw.__bindgen_anon_1.textmessage,
                ))
            }
        } else {
            None
        }
    }

    /// Returns the channel payload if present.
    #[must_use]
    pub fn channel(&self) -> Option<crate::types::Channel> {
        if matches!(
            self.event,
            crate::events::Event::ChannelCreated
                | crate::events::Event::ChannelUpdated
                | crate::events::Event::ChannelRemoved
        ) && self.has_tt_type(ffi::TTType::__CHANNEL)
        {
            unsafe {
                Some(crate::types::Channel::from(
                    self.raw.__bindgen_anon_1.channel,
                ))
            }
        } else {
            None
        }
    }

    /// Returns the server properties payload if present.
    #[must_use]
    pub fn server_properties(&self) -> Option<crate::types::ServerProperties> {
        if matches!(self.event, crate::events::Event::ServerUpdate)
            && self.has_tt_type(ffi::TTType::__SERVERPROPERTIES)
        {
            unsafe {
                Some(crate::types::ServerProperties::from(
                    self.raw.__bindgen_anon_1.serverproperties,
                ))
            }
        } else {
            None
        }
    }

    /// Returns the server statistics payload if present.
    #[must_use]
    pub fn server_statistics(&self) -> Option<crate::types::ServerStatistics> {
        if matches!(self.event, crate::events::Event::ServerStatistics)
            && self.has_tt_type(ffi::TTType::__SERVERSTATISTICS)
        {
            unsafe {
                Some(crate::types::ServerStatistics::from(
                    self.raw.__bindgen_anon_1.serverstatistics,
                ))
            }
        } else {
            None
        }
    }

    /// Returns the file transfer payload if present.
    #[must_use]
    pub fn file_transfer(&self) -> Option<crate::types::FileTransfer> {
        if matches!(self.event, crate::events::Event::FileTransfer)
            && self.has_tt_type(ffi::TTType::__FILETRANSFER)
        {
            unsafe {
                Some(crate::types::FileTransfer::from(
                    self.raw.__bindgen_anon_1.filetransfer,
                ))
            }
        } else {
            None
        }
    }

    /// Returns the remote file payload if present.
    #[must_use]
    pub fn remote_file(&self) -> Option<crate::types::RemoteFile> {
        if matches!(
            self.event,
            crate::events::Event::FileNew | crate::events::Event::FileRemove
        ) && self.has_tt_type(ffi::TTType::__REMOTEFILE)
        {
            unsafe {
                Some(crate::types::RemoteFile::from(
                    self.raw.__bindgen_anon_1.remotefile,
                ))
            }
        } else {
            None
        }
    }

    /// Returns the user payload if present.
    #[must_use]
    pub fn user(&self) -> Option<crate::types::User> {
        if matches!(
            self.event,
            crate::events::Event::UserLoggedIn
                | crate::events::Event::UserLoggedOut
                | crate::events::Event::UserUpdate
                | crate::events::Event::UserJoined
                | crate::events::Event::UserLeft
                | crate::events::Event::UserStateChange
                | crate::events::Event::MySelfKicked
                | crate::events::Event::UserFirstVoiceStreamPacket
        ) && self.has_tt_type(ffi::TTType::__USER)
        {
            unsafe { Some(crate::types::User::from(self.raw.__bindgen_anon_1.user)) }
        } else {
            None
        }
    }

    /// Returns the user account payload if present.
    #[must_use]
    pub fn account(&self) -> Option<crate::types::UserAccount> {
        if matches!(
            self.event,
            crate::events::Event::UserAccount
                | crate::events::Event::UserAccountCreated
                | crate::events::Event::UserAccountRemoved
        ) && self.has_tt_type(ffi::TTType::__USERACCOUNT)
        {
            unsafe {
                Some(crate::types::UserAccount::from(
                    self.raw.__bindgen_anon_1.useraccount,
                ))
            }
        } else {
            None
        }
    }

    /// Returns the banned-user payload if present.
    #[must_use]
    pub fn banned_user(&self) -> Option<crate::types::BannedUser> {
        if matches!(self.event, crate::events::Event::BannedUser)
            && self.has_tt_type(ffi::TTType::__BANNEDUSER)
        {
            unsafe {
                Some(crate::types::BannedUser::from(
                    self.raw.__bindgen_anon_1.banneduser,
                ))
            }
        } else {
            None
        }
    }

    /// Returns the desktop input payload if present.
    #[must_use]
    pub fn desktop_input(&self) -> Option<crate::types::DesktopInput> {
        if matches!(self.event, crate::events::Event::DesktopInput)
            && self.has_tt_type(ffi::TTType::__DESKTOPINPUT)
        {
            unsafe {
                Some(crate::types::DesktopInput::from(
                    self.raw.__bindgen_anon_1.desktopinput,
                ))
            }
        } else {
            None
        }
    }

    /// Returns the media-file payload if present.
    #[must_use]
    pub fn media_file_info(&self) -> Option<crate::types::MediaFileInfo> {
        if matches!(
            self.event,
            crate::events::Event::StreamMediaFile | crate::events::Event::LocalMediaFile
        ) && self.has_tt_type(ffi::TTType::__MEDIAFILEINFO)
        {
            unsafe {
                Some(crate::types::MediaFileInfo::from(
                    self.raw.__bindgen_anon_1.mediafileinfo,
                ))
            }
        } else {
            None
        }
    }

    /// Returns the audio-input progress payload if present.
    #[must_use]
    pub fn audio_input_progress(&self) -> Option<crate::types::AudioInputProgress> {
        if matches!(self.event, crate::events::Event::AudioInput)
            && self.has_tt_type(ffi::TTType::__AUDIOINPUTPROGRESS)
        {
            unsafe {
                Some(crate::types::AudioInputProgress::from(
                    self.raw.__bindgen_anon_1.audioinputprogress,
                ))
            }
        } else {
            None
        }
    }

    /// Returns the SDK error payload if present.
    #[must_use]
    pub fn error_message(&self) -> Option<crate::types::ErrorMessage> {
        if matches!(
            self.event,
            crate::events::Event::ConnectCryptError
                | crate::events::Event::CmdError
                | crate::events::Event::InternalError
        ) && self.has_tt_type(ffi::TTType::__CLIENTERRORMSG)
        {
            unsafe {
                Some(crate::types::ErrorMessage::from(
                    self.raw.__bindgen_anon_1.clienterrormsg,
                ))
            }
        } else {
            None
        }
    }

    /// Returns the raw `TeamTalk` message.
    #[must_use]
    pub fn raw(&self) -> &ffi::TTMessage {
        &self.raw
    }

    /// Returns typed payload for the current event, if available.
    pub fn data(&self) -> Option<EventData> {
        self.text()
            .map(EventData::TextMessage)
            .or_else(|| self.channel().map(EventData::Channel))
            .or_else(|| self.server_properties().map(EventData::ServerProperties))
            .or_else(|| self.server_statistics().map(EventData::ServerStatistics))
            .or_else(|| self.file_transfer().map(EventData::FileTransfer))
            .or_else(|| self.remote_file().map(EventData::RemoteFile))
            .or_else(|| self.user().map(EventData::User))
            .or_else(|| self.banned_user().map(EventData::BannedUser))
            .or_else(|| self.account().map(EventData::UserAccount))
            .or_else(|| self.desktop_input().map(EventData::DesktopInput))
            .or_else(|| self.media_file_info().map(EventData::MediaFileInfo))
            .or_else(|| {
                self.audio_input_progress()
                    .map(EventData::AudioInputProgress)
            })
            .or_else(|| self.error_message().map(EventData::ErrorMessage))
    }

    /// Returns the text message payload when this message carries it.
    #[must_use]
    pub fn try_as_text_message(&self) -> Option<crate::types::TextMessage> {
        self.text()
    }

    /// Returns the user payload when this message carries it.
    #[must_use]
    pub fn try_as_user(&self) -> Option<crate::types::User> {
        self.user()
    }

    /// Returns the channel payload when this message carries it.
    #[must_use]
    pub fn try_as_channel(&self) -> Option<crate::types::Channel> {
        self.channel()
    }

    /// Returns the remote-file payload when this message carries it.
    #[must_use]
    pub fn try_as_remote_file(&self) -> Option<crate::types::RemoteFile> {
        self.remote_file()
    }

    /// Returns the banned-user payload when this message carries it.
    #[must_use]
    pub fn try_as_banned_user(&self) -> Option<crate::types::BannedUser> {
        self.banned_user()
    }

    /// Returns the desktop-input payload when this message carries it.
    #[must_use]
    pub fn try_as_desktop_input(&self) -> Option<crate::types::DesktopInput> {
        self.desktop_input()
    }

    /// Returns the media-file payload when this message carries it.
    #[must_use]
    pub fn try_as_media_file_info(&self) -> Option<crate::types::MediaFileInfo> {
        self.media_file_info()
    }

    /// Returns the audio-input progress payload when this message carries it.
    #[must_use]
    pub fn try_as_audio_input_progress(&self) -> Option<crate::types::AudioInputProgress> {
        self.audio_input_progress()
    }
}
