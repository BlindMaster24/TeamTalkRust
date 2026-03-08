use super::*;

impl Client {
    /// Enables automatic reconnection using the provided config.
    pub fn enable_auto_reconnect(&self, config: ReconnectConfig) {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        auto.enabled = true;
        auto.handler = Some(ReconnectHandler::new(config));
        auto.extra_events.clear();
        auto.force_disconnect = false;
        reset_auto_recovery_handlers(&mut auto);
    }

    /// Enables automatic reconnection and adds extra events that trigger reconnect.
    pub fn enable_auto_reconnect_with_events(
        &self,
        config: ReconnectConfig,
        extra_events: Vec<crate::events::Event>,
    ) {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        auto.enabled = true;
        auto.handler = Some(ReconnectHandler::new(config));
        auto.extra_events = dedupe_events(extra_events);
        auto.force_disconnect = false;
        reset_auto_recovery_handlers(&mut auto);
    }

    /// Disables automatic reconnection.
    pub fn disable_auto_reconnect(&self) {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        auto.enabled = false;
        auto.handler = None;
        auto.login_handler = None;
        auto.join_handler = None;
        auto.login_gave_up = false;
        auto.join_gave_up = false;
        auto.recovery_completed = false;
        auto.clear_phase_tracking();
        auto.extra_events.clear();
        auto.force_disconnect = false;
    }

    /// Sets per-phase retry behavior for in-session recovery.
    pub fn set_reconnect_workflow_config(&self, workflow: ReconnectWorkflowConfig) {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        auto.workflow = workflow;
        reset_auto_recovery_handlers(&mut auto);
    }

    /// Returns per-phase retry behavior for in-session recovery.
    pub fn reconnect_workflow_config(&self) -> ReconnectWorkflowConfig {
        self.auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .workflow
            .clone()
    }

    /// Sets watchdog timeouts for stalled connect/login/join phases.
    ///
    /// Timeouts are evaluated from the normal `poll()` loop; keep polling the
    /// client for recovery supervision to progress.
    pub fn set_reconnect_phase_timeouts(
        &self,
        timeouts: ReconnectPhaseTimeouts,
    ) -> Result<(), crate::events::Error> {
        validate_phase_timeouts(&timeouts)?;
        self.auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .phase_timeouts = timeouts;
        Ok(())
    }

    /// Returns watchdog timeouts for stalled connect/login/join phases.
    pub fn reconnect_phase_timeouts(&self) -> ReconnectPhaseTimeouts {
        self.auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .phase_timeouts
            .clone()
    }

    /// Enables full in-session recovery and stores connect/login params in memory.
    pub fn enable_full_auto_reconnect(
        &self,
        connect_config: ReconnectConfig,
        workflow: ReconnectWorkflowConfig,
        connect_params: ConnectParamsOwned,
        login_params: crate::client::users::LoginParams,
    ) {
        self.enable_auto_reconnect(connect_config);
        self.set_reconnect_workflow_config(workflow);
        self.set_reconnect_params(connect_params);
        self.set_login_params(login_params);
    }

    /// Returns true if automatic reconnection is enabled.
    pub fn auto_reconnect_enabled(&self) -> bool {
        self.auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .enabled
    }

    /// Stores connection parameters for automatic reconnection.
    pub fn set_reconnect_params(&self, params: ConnectParamsOwned) {
        self.auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .params = Some(params);
    }

    /// Returns the stored reconnection parameters, if any.
    pub fn reconnect_params(&self) -> Option<ConnectParamsOwned> {
        self.auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .params
            .clone()
    }

    /// Sets extra events that should trigger automatic reconnection.
    pub fn set_auto_reconnect_events(&self, extra_events: Vec<crate::events::Event>) {
        self.auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .extra_events = dedupe_events(extra_events);
    }

    /// Adds one extra event that should trigger automatic reconnection.
    pub fn add_auto_reconnect_event(&self, event: crate::events::Event) {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if auto
            .extra_events
            .iter()
            .any(|existing| std::mem::discriminant(existing) == std::mem::discriminant(&event))
        {
            return;
        }
        auto.extra_events.push(event);
    }

    /// Removes one extra event trigger by event kind.
    pub fn remove_auto_reconnect_event(&self, event: crate::events::Event) -> bool {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let before = auto.extra_events.len();
        auto.extra_events
            .retain(|existing| std::mem::discriminant(existing) != std::mem::discriminant(&event));
        auto.extra_events.len() != before
    }

    /// Returns the extra events that trigger automatic reconnection.
    pub fn auto_reconnect_events(&self) -> Vec<crate::events::Event> {
        self.auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .extra_events
            .clone()
    }

    /// Returns the last remembered channel, if any.
    pub fn last_channel(&self) -> Option<crate::types::ChannelId> {
        self.auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .last_channel
    }

    /// Remembers the channel and optional password used for auto-join after reconnect.
    pub fn set_last_channel(&self, channel: crate::types::ChannelId, password: Option<&str>) {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        auto.last_channel = Some(channel);
        auto.last_channel_password = match password {
            Some(value) if !value.is_empty() => Some(value.to_string()),
            _ => None,
        };
        auto.join_gave_up = false;
        auto.clear_join_phase();
    }

    /// Clears the remembered channel.
    pub fn clear_last_channel(&self) {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        auto.last_channel = None;
        auto.last_channel_password = None;
        auto.clear_join_phase();
        auto.join_gave_up = false;
    }
}
