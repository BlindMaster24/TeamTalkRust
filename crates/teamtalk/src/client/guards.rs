//! Internal connection-state guards shared across `client` sub-modules.
//!
//! These helpers capture the "is it legal to issue this command right now?"
//! predicates that several command-dispatch modules (`server`, `channels`,
//! `users`, `files`, ...) need to gate FFI calls on. Keeping a single
//! source of truth here prevents the obvious drift class where one of the
//! four historical copies gets updated (for example to accept a new
//! `ConnectionState` variant) while the others are forgotten.
//!
//! All guards are `pub(crate)` on purpose: they are implementation
//! details of the client surface and must not leak into downstream
//! crates.

use crate::events::ConnectionState;

/// Returns `true` when the client's connection state permits issuing any
/// TeamTalk command that requires the user to be logged in on the server.
///
/// The predicate matches the three states in which the SDK considers the
/// user authenticated on a server:
///
/// * [`ConnectionState::LoggedIn`] - fully logged in but not in any channel.
/// * [`ConnectionState::Joining`] - in-flight channel join, still logged in.
/// * [`ConnectionState::Joined`] - joined to a channel.
pub(crate) fn can_issue_logged_in_command(state: ConnectionState) -> bool {
    matches!(
        state,
        ConnectionState::LoggedIn | ConnectionState::Joining(_) | ConnectionState::Joined(_)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ChannelId;

    #[test]
    fn logged_in_states_allow_commands() {
        assert!(can_issue_logged_in_command(ConnectionState::LoggedIn));
        assert!(can_issue_logged_in_command(ConnectionState::Joining(
            ChannelId(0)
        )));
        assert!(can_issue_logged_in_command(ConnectionState::Joined(
            ChannelId(0)
        )));
    }

    #[test]
    fn pre_login_and_disconnected_states_block_commands() {
        assert!(!can_issue_logged_in_command(ConnectionState::Idle));
        assert!(!can_issue_logged_in_command(ConnectionState::Connecting));
        assert!(!can_issue_logged_in_command(ConnectionState::Connected));
        assert!(!can_issue_logged_in_command(ConnectionState::LoggingIn));
        assert!(!can_issue_logged_in_command(ConnectionState::Disconnected));
    }
}
