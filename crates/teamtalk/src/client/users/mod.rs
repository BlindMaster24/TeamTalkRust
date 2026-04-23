//! User management APIs.
use super::Client;
pub(super) use super::guards::can_issue_logged_in_command;
use crate::types::{
    ChannelId, CommandId, MessageTarget, Subscriptions, TT_STRLEN, User, UserAccount, UserId,
    UserStatistics, UserStatus,
};
use crate::utils::ToTT;
use std::env;
use teamtalk_sys as ffi;

const TT_TEXT_MAX_PAYLOAD: usize = TT_STRLEN - 1;

fn can_login_in_state(state: crate::events::ConnectionState) -> bool {
    matches!(state, crate::events::ConnectionState::Connected)
}

fn can_logout_in_state(state: crate::events::ConnectionState) -> bool {
    matches!(
        state,
        crate::events::ConnectionState::LoggedIn
            | crate::events::ConnectionState::Joining(_)
            | crate::events::ConnectionState::Joined(_)
    )
}

/// Options for multipart text sending.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SendTextOptions {
    /// Additional retry attempts for the first chunk only.
    pub first_chunk_retries: u32,
}

impl SendTextOptions {
    /// Creates default send options.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            first_chunk_retries: 0,
        }
    }

    /// Sets the number of extra retries for the first chunk.
    #[must_use]
    pub const fn with_first_chunk_retries(mut self, retries: u32) -> Self {
        self.first_chunk_retries = retries;
        self
    }
}

impl Default for SendTextOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(windows)]
fn ttchar_units(ch: char) -> usize {
    ch.len_utf16()
}

#[cfg(not(windows))]
fn ttchar_units(ch: char) -> usize {
    ch.len_utf8()
}

fn split_text_chunks(text: &str, max_units: usize) -> Vec<String> {
    if text.is_empty() {
        return vec![String::new()];
    }

    let mut chunks = Vec::new();
    let mut current = String::new();
    let mut current_tt_units = 0usize;
    #[cfg(windows)]
    let mut current_utf8_bytes = 0usize;

    for ch in text.chars() {
        let ch_tt_units = ttchar_units(ch);
        #[cfg(windows)]
        let ch_utf8_bytes = ch.len_utf8();
        #[cfg(windows)]
        let would_exceed = current_tt_units + ch_tt_units > max_units
            || current_utf8_bytes + ch_utf8_bytes > max_units;
        #[cfg(not(windows))]
        let would_exceed = current_tt_units + ch_tt_units > max_units;

        if would_exceed && !current.is_empty() {
            chunks.push(std::mem::take(&mut current));
            current_tt_units = 0;
            #[cfg(windows)]
            {
                current_utf8_bytes = 0;
            }
        }

        current.push(ch);
        current_tt_units += ch_tt_units;
        #[cfg(windows)]
        {
            current_utf8_bytes += ch_utf8_bytes;
        }
    }

    if !current.is_empty() {
        chunks.push(current);
    }

    if chunks.is_empty() {
        chunks.push(String::new());
    }

    chunks
}

fn text_message_for_target(target: MessageTarget) -> ffi::TextMessage {
    let mut msg = ffi::TextMessage::default();
    match target {
        MessageTarget::User(id) => {
            msg.nMsgType = ffi::TextMsgType::MSGTYPE_USER;
            msg.nToUserID = id.raw();
        }
        MessageTarget::Channel(id) => {
            msg.nMsgType = ffi::TextMsgType::MSGTYPE_CHANNEL;
            msg.nChannelID = id.raw();
        }
        MessageTarget::Broadcast => {
            msg.nMsgType = ffi::TextMsgType::MSGTYPE_BROADCAST;
        }
    }
    msg
}

/// Stored login parameters for automatic login.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct LoginParams {
    pub nickname: String,
    pub username: String,
    pub password: String,
    pub client_name: String,
}

impl LoginParams {
    pub fn new(
        nickname: impl Into<String>,
        username: impl Into<String>,
        password: impl Into<String>,
        client_name: impl Into<String>,
    ) -> Self {
        Self {
            nickname: nickname.into(),
            username: username.into(),
            password: password.into(),
            client_name: client_name.into(),
        }
    }

    #[must_use]
    pub fn from_env() -> Self {
        let nickname = env::var("TT_NICK").unwrap_or_default();
        let username = env::var("TT_USER").unwrap_or_default();
        let password = env::var("TT_PASS").unwrap_or_default();
        let client_name = env::var("TT_CLIENT").unwrap_or_default();
        Self::new(nickname, username, password, client_name)
    }
}

mod auth;
mod directory;
mod messaging;
mod moderation;
mod profile;
mod subscriptions;
