use super::*;

impl Client {
    fn empty_message(event: Event) -> Message {
        Message::from_raw(event, unsafe { std::mem::zeroed::<ffi::TTMessage>() })
    }

    pub(super) fn handle_connect_recovery(&self) {
        let params = {
            let mut auto = self
                .auto_reconnect
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            if !auto.enabled {
                return;
            }

            let params: super::super::connection::ConnectParamsOwned = match auto.params.as_ref() {
                Some(params) => params.clone(),
                None => return,
            };

            let handler: &mut super::super::connection::ReconnectHandler =
                match auto.handler.as_mut() {
                    Some(handler) => handler,
                    None => return,
                };

            if handler.can_attempt() {
                params
            } else if handler.exhausted() {
                let attempts = handler.attempts();
                drop(auto);
                let failed_event = Event::ReconnectFailed { attempts };
                let msg = Self::empty_message(failed_event);
                self.invoke_hooks(failed_event, &msg);

                let mut auto = self
                    .auto_reconnect
                    .lock()
                    .unwrap_or_else(|e| e.into_inner());
                auto.enabled = false;
                auto.handler = None;
                auto.login_handler = None;
                auto.join_handler = None;
                return;
            } else {
                return;
            }
        };

        let _ = self.disconnect();
        if self.has_connection_flags() {
            return;
        }

        let (attempt, delay) = {
            let mut auto = self
                .auto_reconnect
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            if !auto.enabled {
                return;
            }

            let handler = match auto.handler.as_mut() {
                Some(handler) => handler,
                None => return,
            };
            if !handler.can_attempt() {
                return;
            }

            let attempt = handler.attempts() + 1;
            let delay = handler.current_delay();
            handler.record_attempt();
            auto.force_disconnect = false;
            auto.login_gave_up = false;
            auto.join_gave_up = false;
            auto.recovery_completed = false;
            (attempt, delay)
        };

        let before_event = Event::BeforeReconnect { attempt, delay };
        let msg = Self::empty_message(before_event);
        self.invoke_hooks(before_event, &msg);
        self.invoke_hooks(Event::Reconnecting { attempt, delay }, &msg);
        let _ = self.connect(&params.host, params.tcp, params.udp, params.encrypted);
    }

    pub(super) fn handle_login_recovery(&self) {
        let (params, attempt, delay, gave_up_now) = {
            let mut auto = self
                .auto_reconnect
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            if !auto.enabled || auto.login_gave_up {
                return;
            }

            let params = match auto.login.as_ref() {
                Some(params) => params.clone(),
                None => return,
            };

            let login_config = auto.workflow.login.clone();
            let handler = auto.login_handler.get_or_insert_with(|| {
                super::super::connection::ReconnectHandler::new(login_config)
            });
            if handler.can_attempt() {
                let attempt = handler.attempts() + 1;
                let delay = handler.current_delay();
                handler.record_attempt();
                (params, attempt, delay, false)
            } else if handler.exhausted() {
                let attempts = handler.attempts();
                let delay = handler.current_delay();
                auto.login_gave_up = true;
                (params, attempts, delay, true)
            } else {
                return;
            }
        };

        if gave_up_now {
            let event = Event::AutoLoginFailed { attempts: attempt };
            let msg = Self::empty_message(event);
            self.invoke_hooks(event, &msg);
            return;
        }

        let before_event = Event::BeforeAutoLogin { attempt, delay };
        let msg = Self::empty_message(before_event);
        self.invoke_hooks(before_event, &msg);

        let cmd_id = self.login(
            &params.nickname,
            &params.username,
            &params.password,
            &params.client_name,
        );
        if cmd_id <= 0 {
            let mut auto = self
                .auto_reconnect
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            auto.pending_login_cmd = None;
            drop(auto);
            self.set_connection_state(ConnectionState::Connected);
            let failed = Event::AutoLoginFailed { attempts: attempt };
            let msg = Self::empty_message(failed);
            self.invoke_hooks(failed, &msg);
        } else {
            let mut auto = self
                .auto_reconnect
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            auto.pending_login_cmd = Some(cmd_id);
        }
    }

    pub(super) fn handle_join_recovery(&self) {
        let (channel, password, attempt, delay, gave_up_now) = {
            let mut auto = self
                .auto_reconnect
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            if !auto.enabled || auto.join_gave_up {
                return;
            }

            let channel = match auto.last_channel {
                Some(channel) => channel,
                None => return,
            };
            let password = auto.last_channel_password.clone();

            let join_config = auto.workflow.join.clone();
            let handler = auto.join_handler.get_or_insert_with(|| {
                super::super::connection::ReconnectHandler::new(join_config)
            });
            if handler.can_attempt() {
                let attempt = handler.attempts() + 1;
                let delay = handler.current_delay();
                handler.record_attempt();
                (channel, password, attempt, delay, false)
            } else if handler.exhausted() {
                let attempts = handler.attempts();
                let delay = handler.current_delay();
                auto.join_gave_up = true;
                (channel, password, attempts, delay, true)
            } else {
                return;
            }
        };

        if gave_up_now {
            let event = Event::AutoJoinFailed { attempts: attempt };
            let msg = Self::empty_message(event);
            self.invoke_hooks(event, &msg);
            return;
        }

        let before_event = Event::BeforeAutoJoin { attempt, delay };
        let msg = Self::empty_message(before_event);
        self.invoke_hooks(before_event, &msg);

        let cmd_id = self.join_channel(channel, password.as_deref().unwrap_or(""));
        if cmd_id <= 0 {
            let mut auto = self
                .auto_reconnect
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            auto.pending_join_cmd = None;
            drop(auto);
            self.set_connection_state(ConnectionState::LoggedIn);
            let failed = Event::AutoJoinFailed { attempts: attempt };
            let msg = Self::empty_message(failed);
            self.invoke_hooks(failed, &msg);
        } else {
            let mut auto = self
                .auto_reconnect
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            auto.pending_join_cmd = Some(cmd_id);
        }
    }

    pub(super) fn handle_recovery_completed(&self) {
        let (reconnect_attempts, login_attempts, join_attempts) = {
            let mut auto = self
                .auto_reconnect
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            if !auto.enabled || auto.recovery_completed {
                return;
            }

            auto.recovery_completed = true;
            auto.login_gave_up = false;
            auto.join_gave_up = false;
            let reconnect_attempts = auto
                .handler
                .as_ref()
                .map_or(0, |handler| handler.attempts());
            let login_attempts = auto
                .login_handler
                .as_ref()
                .map_or(0, |handler| handler.attempts());
            let join_attempts = auto
                .join_handler
                .as_ref()
                .map_or(0, |handler| handler.attempts());
            if let Some(handler) = auto.login_handler.as_mut() {
                handler.reset();
            }
            if let Some(handler) = auto.join_handler.as_mut() {
                handler.reset();
            }
            (reconnect_attempts, login_attempts, join_attempts)
        };

        let event = Event::AutoRecoverCompleted {
            reconnect_attempts,
            login_attempts,
            join_attempts,
        };
        let msg = Self::empty_message(event);
        self.invoke_hooks(event, &msg);
    }
}
