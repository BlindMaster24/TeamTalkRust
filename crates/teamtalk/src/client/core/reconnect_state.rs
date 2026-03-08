use std::time::Instant;

#[derive(Default)]
pub(crate) struct AutoReconnectState {
    pub(crate) enabled: bool,
    pub(crate) handler: Option<super::super::connection::ReconnectHandler>,
    pub(crate) login_handler: Option<super::super::connection::ReconnectHandler>,
    pub(crate) join_handler: Option<super::super::connection::ReconnectHandler>,
    pub(crate) params: Option<super::super::connection::ConnectParamsOwned>,
    pub(crate) last_channel: Option<crate::types::ChannelId>,
    pub(crate) last_channel_password: Option<String>,
    pub(crate) login: Option<super::super::users::LoginParams>,
    pub(crate) workflow: super::super::connection::ReconnectWorkflowConfig,
    pub(crate) phase_timeouts: super::super::connection::ReconnectPhaseTimeouts,
    pub(crate) login_gave_up: bool,
    pub(crate) join_gave_up: bool,
    pub(crate) recovery_completed: bool,
    pub(crate) pending_login_cmd: Option<i32>,
    pub(crate) pending_join_cmd: Option<i32>,
    pub(crate) connect_started_at: Option<Instant>,
    pub(crate) login_started_at: Option<Instant>,
    pub(crate) join_started_at: Option<Instant>,
    pub(crate) extra_events: Vec<crate::events::Event>,
    pub(crate) force_disconnect: bool,
}

impl AutoReconnectState {
    pub(crate) fn clear_connect_phase(&mut self) {
        self.connect_started_at = None;
    }

    pub(crate) fn clear_login_phase(&mut self) {
        self.login_started_at = None;
        self.pending_login_cmd = None;
    }

    pub(crate) fn clear_join_phase(&mut self) {
        self.join_started_at = None;
        self.pending_join_cmd = None;
    }

    pub(crate) fn clear_phase_tracking(&mut self) {
        self.clear_connect_phase();
        self.clear_login_phase();
        self.clear_join_phase();
    }
}
