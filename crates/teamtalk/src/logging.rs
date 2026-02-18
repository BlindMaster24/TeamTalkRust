//! Tracing integration for client events.
use crate::client::Message;
use crate::events::Event;

/// Logs an event and its source id using `tracing`.
pub fn event(event: &Event, message: &Message) {
    let source = message.source();

    match event {
        Event::CmdError => {
            if let Some(err) = message.error_message() {
                tracing::error!(
                    code = err.code,
                    source,
                    message = %err.message,
                    "Command error"
                );
            } else {
                tracing::error!(source, "Unknown command error");
            }
        }
        Event::ConnectFailed => {
            tracing::error!("Connection failed");
        }
        Event::ConnectionLost => {
            tracing::warn!("Connection lost");
        }
        Event::TextMessage => {
            if let Some(msg) = message.text() {
                tracing::info!(
                    from = %msg.from_username,
                    type = ?msg.msg_type,
                    "Text message"
                );
            }
        }
        Event::UserJoined => {
            if let Some(user) = message.user() {
                tracing::info!(nick = %user.nickname, id = user.id.0, "User joined");
            }
        }
        Event::UserLeft => {
            if let Some(user) = message.user() {
                tracing::info!(nick = %user.nickname, id = user.id.0, "User left");
            }
        }
        _ => {
            tracing::debug!(?event, source, "Event received");
        }
    }
}
