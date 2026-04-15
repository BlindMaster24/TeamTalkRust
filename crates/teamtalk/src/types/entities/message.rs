use crate::types::{ChannelId, CommandId, UserId};
use teamtalk_sys as ffi;

/// Text message payload.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct TextMessage {
    pub msg_type: ffi::TextMsgType,
    pub from_id: UserId,
    pub from_username: String,
    pub to_id: UserId,
    pub channel_id: ChannelId,
    pub text: String,
    pub more: bool,
}

impl From<ffi::TextMessage> for TextMessage {
    fn from(m: ffi::TextMessage) -> Self {
        Self {
            msg_type: m.nMsgType,
            from_id: UserId(m.nFromUserID),
            from_username: crate::utils::strings::to_string(&m.szFromUsername),
            to_id: UserId(m.nToUserID),
            channel_id: ChannelId(m.nChannelID),
            text: unsafe { crate::utils::strings::from_tt(m.szMessage.as_ptr()) },
            more: m.bMore != 0,
        }
    }
}

impl TextMessage {
    pub fn send_to_user(client: &crate::client::Client, user_id: UserId, text: &str) -> CommandId {
        MessageBuilder::new(user_id).text(text).send_cmd(client)
    }

    pub fn send_to_channel(
        client: &crate::client::Client,
        channel_id: ChannelId,
        text: &str,
    ) -> CommandId {
        MessageBuilder::new(channel_id).text(text).send_cmd(client)
    }

    pub fn send_broadcast(client: &crate::client::Client, text: &str) -> CommandId {
        MessageBuilder::new(MessageTarget::Broadcast)
            .text(text)
            .send_cmd(client)
    }

    /// Sends a private reply to the sender of this message.
    pub fn send_private(&self, client: &crate::client::Client, text: &str) -> CommandId {
        MessageBuilder::new(self.from_id)
            .text(text)
            .send_cmd(client)
    }
}

/// Destination for sending text messages.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageTarget {
    User(UserId),
    Channel(ChannelId),
    Broadcast,
}

impl From<UserId> for MessageTarget {
    fn from(id: UserId) -> Self {
        Self::User(id)
    }
}
impl From<ChannelId> for MessageTarget {
    fn from(id: ChannelId) -> Self {
        Self::Channel(id)
    }
}
impl From<&TextMessage> for MessageTarget {
    fn from(m: &TextMessage) -> Self {
        Self::User(m.from_id)
    }
}

/// Builder for outgoing text messages.
pub struct MessageBuilder {
    target: MessageTarget,
    text: String,
}

impl MessageBuilder {
    /// Creates a new builder for the target.
    pub fn new(target: impl Into<MessageTarget>) -> Self {
        Self {
            target: target.into(),
            text: String::new(),
        }
    }

    /// Sets the message body.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    /// Sends the message using the provided client.
    pub fn send(self, client: &crate::client::Client) -> CommandId {
        client.send_text(self.target, &self.text)
    }

    /// Sends the message and wraps the result in a CommandId.
    pub fn send_cmd(self, client: &crate::client::Client) -> CommandId {
        self.send(client)
    }
}
