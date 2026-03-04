use super::ffi;

/// Wrapper around a raw TeamTalk message with its originating event.
pub struct Message {
    event: crate::events::Event,
    raw: ffi::TTMessage,
}

/// Typed payload extracted from a TeamTalk message.
pub enum EventData {
    TextMessage(crate::types::TextMessage),
    Channel(crate::types::Channel),
    ServerProperties(crate::types::ServerProperties),
    ServerStatistics(crate::types::ServerStatistics),
    FileTransfer(crate::types::FileTransfer),
    User(crate::types::User),
    UserAccount(crate::types::UserAccount),
    ErrorMessage(crate::types::ErrorMessage),
}

impl Message {
    fn has_tt_type(&self, expected: ffi::TTType) -> bool {
        self.raw.ttType == expected
    }

    /// Wraps a raw TeamTalk message.
    pub(crate) fn from_raw(event: crate::events::Event, raw: ffi::TTMessage) -> Self {
        Self { event, raw }
    }

    /// Returns the originating event for this message.
    pub fn event(&self) -> crate::events::Event {
        self.event
    }

    /// Returns the source user id for the message.
    pub fn source(&self) -> i32 {
        self.raw.nSource
    }

    /// Returns the text message payload if present.
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

    /// Returns the user payload if present.
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

    /// Returns the SDK error payload if present.
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

    /// Returns the raw TeamTalk message.
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
            .or_else(|| self.user().map(EventData::User))
            .or_else(|| self.account().map(EventData::UserAccount))
            .or_else(|| self.error_message().map(EventData::ErrorMessage))
    }

    /// Returns the text message payload when this message carries it.
    pub fn try_as_text_message(&self) -> Option<crate::types::TextMessage> {
        self.text()
    }

    /// Returns the user payload when this message carries it.
    pub fn try_as_user(&self) -> Option<crate::types::User> {
        self.user()
    }

    /// Returns the channel payload when this message carries it.
    pub fn try_as_channel(&self) -> Option<crate::types::Channel> {
        self.channel()
    }
}
