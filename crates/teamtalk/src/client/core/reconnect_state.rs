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
    pub(crate) login_gave_up: bool,
    pub(crate) join_gave_up: bool,
    pub(crate) recovery_completed: bool,
    pub(crate) pending_login_cmd: Option<i32>,
    pub(crate) pending_join_cmd: Option<i32>,
    pub(crate) extra_events: Vec<crate::events::Event>,
    pub(crate) force_disconnect: bool,
}
