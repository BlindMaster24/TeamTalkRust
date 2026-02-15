//! Encryption context management.
use super::Client;
use crate::events::ConnectionState;
pub use crate::types::EncryptionContext;
use teamtalk_sys as ffi;

fn can_set_encryption_context_in_state(state: ConnectionState) -> bool {
    matches!(state, ConnectionState::Idle | ConnectionState::Disconnected)
}

impl Client {
    /// Sets the encryption context for future connections.
    ///
    /// TeamTalk C-API requires this to be configured before `TT_Connect*`.
    /// This helper returns `false` if the client is already in a connect/login
    /// lifecycle state.
    pub fn set_encryption_context(&self, context: &EncryptionContext) -> bool {
        let state = *self.state.lock().expect("client state mutex poisoned");
        if !can_set_encryption_context_in_state(state) {
            return false;
        }
        unsafe { ffi::api().TT_SetEncryptionContext(self.ptr.0, &context.to_ffi()) == 1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encryption_context_guard_allows_only_idle_or_disconnected() {
        assert!(can_set_encryption_context_in_state(ConnectionState::Idle));
        assert!(can_set_encryption_context_in_state(
            ConnectionState::Disconnected
        ));
        assert!(!can_set_encryption_context_in_state(
            ConnectionState::Connecting
        ));
        assert!(!can_set_encryption_context_in_state(
            ConnectionState::Connected
        ));
        assert!(!can_set_encryption_context_in_state(
            ConnectionState::LoggingIn
        ));
        assert!(!can_set_encryption_context_in_state(
            ConnectionState::LoggedIn
        ));
        assert!(!can_set_encryption_context_in_state(
            ConnectionState::Joining(crate::types::ChannelId(1))
        ));
        assert!(!can_set_encryption_context_in_state(
            ConnectionState::Joined(crate::types::ChannelId(1))
        ));
    }
}
