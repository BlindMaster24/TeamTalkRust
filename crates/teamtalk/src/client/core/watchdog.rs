use super::*;
use std::time::Instant;

impl Client {
    pub(crate) fn mark_connect_phase_started(&self) {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if !auto.enabled {
            return;
        }
        auto.clear_phase_tracking();
        auto.connect_started_at = Some(Instant::now());
        auto.recovery_completed = false;
    }

    pub(crate) fn mark_login_phase_started(&self, cmd_id: i32) {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if !auto.enabled {
            return;
        }
        auto.clear_join_phase();
        auto.login_started_at = Some(Instant::now());
        auto.pending_login_cmd = Some(cmd_id);
        auto.recovery_completed = false;
    }

    pub(crate) fn mark_join_phase_started(&self, cmd_id: i32) {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if !auto.enabled {
            return;
        }
        auto.join_started_at = Some(Instant::now());
        auto.pending_join_cmd = Some(cmd_id);
        auto.recovery_completed = false;
    }

    pub(crate) fn clear_all_reconnect_phase_tracking(&self) {
        self.auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clear_phase_tracking();
    }

    pub(crate) fn handle_pending_phase_timeout(&self) -> bool {
        let timed_out = {
            let mut auto = self
                .auto_reconnect
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            if !auto.enabled {
                return false;
            }

            match self.connection_state() {
                ConnectionState::Connecting => auto
                    .phase_timeouts
                    .connect
                    .zip(auto.connect_started_at)
                    .filter(|(timeout, started_at)| started_at.elapsed() >= *timeout)
                    .map(|(timeout, _)| {
                        if let Some(handler) = auto.handler.as_mut() {
                            handler.mark_disconnected();
                        }
                        auto.clear_phase_tracking();
                        ("connect", timeout)
                    }),
                ConnectionState::LoggingIn => auto
                    .phase_timeouts
                    .login
                    .zip(auto.login_started_at)
                    .filter(|(timeout, started_at)| started_at.elapsed() >= *timeout)
                    .map(|(timeout, _)| {
                        if let Some(handler) = auto.handler.as_mut() {
                            handler.mark_disconnected();
                        }
                        if let Some(handler) = auto.login_handler.as_mut() {
                            handler.mark_disconnected();
                        }
                        auto.clear_login_phase();
                        auto.clear_join_phase();
                        ("login", timeout)
                    }),
                ConnectionState::Joining(_) => auto
                    .phase_timeouts
                    .join
                    .zip(auto.join_started_at)
                    .filter(|(timeout, started_at)| started_at.elapsed() >= *timeout)
                    .map(|(timeout, _)| {
                        if let Some(handler) = auto.handler.as_mut() {
                            handler.mark_disconnected();
                        }
                        if let Some(handler) = auto.join_handler.as_mut() {
                            handler.mark_disconnected();
                        }
                        auto.clear_join_phase();
                        ("join", timeout)
                    }),
                _ => None,
            }
        };

        let Some((phase, timeout)) = timed_out else {
            return false;
        };

        let _ = self.disconnect();
        self.set_connection_state(ConnectionState::Disconnected);
        #[cfg(feature = "logging")]
        tracing::warn!(
            phase,
            timeout_ms = timeout.as_millis() as u64,
            "teamtalk recovery phase watchdog timed out"
        );
        #[cfg(not(feature = "logging"))]
        let _ = (phase, timeout);
        true
    }

    #[cfg(feature = "mock")]
    pub fn mock_run_auto_reconnect_for_tests(&self) {
        self.handle_auto_reconnect();
    }

    #[cfg(feature = "mock")]
    pub fn mock_age_connect_phase_for_tests(&self, elapsed: std::time::Duration) {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        auto.connect_started_at = Instant::now().checked_sub(elapsed);
    }

    #[cfg(feature = "mock")]
    pub fn mock_age_login_phase_for_tests(&self, elapsed: std::time::Duration) {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        auto.login_started_at = Instant::now().checked_sub(elapsed);
    }

    #[cfg(feature = "mock")]
    pub fn mock_age_join_phase_for_tests(&self, elapsed: std::time::Duration) {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        auto.join_started_at = Instant::now().checked_sub(elapsed);
    }
}
