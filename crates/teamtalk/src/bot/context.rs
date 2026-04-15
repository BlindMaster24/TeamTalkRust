use super::args::Args;
use super::command::Command;
use super::fsm::{DialogFlow, DialogMachine, DialogState, DialogStatus, DialogTimeoutPolicy};
use super::storage::StateStore;
use crate::client::{Client, Message};
use crate::events::{Error, Event, Result};
use crate::types::{ChannelId, CommandId, UserId};
use std::fmt::Display;
use std::str::FromStr;
use std::time::Duration;

#[non_exhaustive]
pub struct Context<'a> {
    pub client: &'a Client,
    pub event: Event,
    pub message: &'a Message,
    pub command: Option<Command>,
    pub state: &'a mut dyn StateStore,
}

impl<'a> Context<'a> {
    /// Creates a new bot context.
    pub fn new(
        client: &'a Client,
        event: Event,
        message: &'a Message,
        command: Option<Command>,
        state: &'a mut dyn StateStore,
    ) -> Self {
        Self {
            client,
            event,
            message,
            command,
            state,
        }
    }
    #[must_use]
    pub fn is_command(&self, name: &str) -> bool {
        self.command_name() == Some(name)
    }

    #[allow(clippy::must_use_candidate)]
    pub fn command_name(&self) -> Option<&str> {
        self.command.as_ref().map(|c| c.name.as_str())
    }

    #[allow(clippy::must_use_candidate)]
    pub fn args(&self) -> Option<Args<'_>> {
        self.command.as_ref().map(|c| Args::new(&c.args))
    }

    #[allow(clippy::must_use_candidate)]
    pub fn command_arg(&self, index: usize) -> Option<&str> {
        self.command.as_ref()?.arg(index)
    }

    #[allow(clippy::must_use_candidate)]
    pub fn text(&self) -> Option<String> {
        self.message.text().map(|text| text.text)
    }

    #[must_use]
    pub fn sender_id(&self) -> UserId {
        self.message.user_id()
    }

    #[allow(clippy::must_use_candidate)]
    pub fn channel_id(&self) -> Option<ChannelId> {
        self.message.text().map(|text| text.channel_id)
    }

    #[must_use]
    pub fn reply_private(&self, text: &str) -> CommandId {
        self.client.send_to_user(self.sender_id(), text)
    }

    #[allow(clippy::must_use_candidate)]
    pub fn reply_channel(&self, text: &str) -> Option<CommandId> {
        self.channel_id()
            .map(|channel_id| self.client.send_to_channel(channel_id, text))
    }

    #[must_use]
    pub fn reply(&self, text: &str) -> CommandId {
        if let Some(channel_id) = self.channel_id() {
            return self.client.send_to_channel(channel_id, text);
        }
        self.reply_private(text)
    }

    #[allow(clippy::must_use_candidate)]
    pub fn wait_for_event(&self, event: Event, timeout: Duration) -> Option<Message> {
        self.client
            .wait_for(event, timeout.as_millis().min(i32::MAX as u128) as i32)
    }

    #[allow(clippy::must_use_candidate)]
    pub fn wait_text_from(&self, from: UserId, timeout: Duration) -> Option<Message> {
        self.client
            .poll_until(
                timeout.as_millis().min(i32::MAX as u128) as i32,
                |event, msg| event == Event::TextMessage && msg.user_id() == from,
            )
            .map(|(_, msg)| msg)
    }

    #[allow(clippy::must_use_candidate)]
    pub fn wait_command_from_sender(
        &self,
        command_name: &str,
        timeout: Duration,
    ) -> Option<Message> {
        let sender = self.sender_id();
        let expected = command_name.to_ascii_lowercase();
        self.client
            .poll_until(
                timeout.as_millis().min(i32::MAX as u128) as i32,
                |event, msg| {
                    if event != Event::TextMessage || msg.user_id() != sender {
                        return false;
                    }
                    let Some(text) = msg.text() else {
                        return false;
                    };
                    let Some(parsed) = super::parse_command(&text.text, &['/', '!']) else {
                        return false;
                    };
                    parsed.name == expected
                },
            )
            .map(|(_, msg)| msg)
    }

    #[allow(clippy::must_use_candidate)]
    pub fn state_get(&self, key: &str) -> Option<String> {
        self.state.get(key)
    }

    pub fn state_set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.state.set(key.into(), value.into());
    }

    pub fn state_remove(&mut self, key: &str) -> Option<String> {
        self.state.remove(key)
    }

    pub fn state_parse<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: FromStr,
    {
        self.state_get(key)
            .map(|value| value.parse::<T>().map_err(|_| Error::InvalidParam))
            .transpose()
    }

    pub fn state_set_typed<T>(&mut self, key: impl Into<String>, value: T)
    where
        T: Display,
    {
        self.state_set(key, value.to_string());
    }

    #[cfg(feature = "bot-serde")]
    pub fn state_get_json<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        self.state_get(key)
            .map(|value| serde_json::from_str(&value).map_err(|_| Error::InvalidParam))
            .transpose()
    }

    #[cfg(feature = "bot-serde")]
    fn json_or_default<T>(value: Result<Option<T>>) -> Result<T>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        Ok(value?.unwrap_or_default())
    }

    #[cfg(feature = "bot-serde")]
    pub fn state_get_json_or_default<T>(&self, key: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        Self::json_or_default(self.state_get_json(key))
    }

    #[cfg(feature = "bot-serde")]
    pub fn state_set_json<T>(&mut self, key: impl Into<String>, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let value = serde_json::to_string(value).map_err(|_| Error::InvalidParam)?;
        self.state_set(key, value);
        Ok(())
    }

    #[must_use]
    pub fn user_state_key(&self, key: &str) -> String {
        format!("u:{}:{key}", self.sender_id().raw())
    }

    #[allow(clippy::must_use_candidate)]
    pub fn channel_state_key(&self, key: &str) -> Option<String> {
        self.channel_id()
            .map(|channel| format!("c:{}:{key}", channel.raw()))
    }

    #[must_use]
    pub fn global_state_key(&self, key: &str) -> String {
        format!("g:{key}")
    }

    #[allow(clippy::must_use_candidate)]
    pub fn user_state_get(&self, key: &str) -> Option<String> {
        self.state_get(&self.user_state_key(key))
    }

    pub fn user_state_set(&mut self, key: &str, value: impl Into<String>) {
        self.state_set(self.user_state_key(key), value);
    }

    pub fn user_state_remove(&mut self, key: &str) -> Option<String> {
        self.state_remove(&self.user_state_key(key))
    }

    pub fn user_state_parse<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: FromStr,
    {
        self.user_state_get(key)
            .map(|value| value.parse::<T>().map_err(|_| Error::InvalidParam))
            .transpose()
    }

    pub fn user_state_set_typed<T>(&mut self, key: &str, value: T)
    where
        T: Display,
    {
        self.user_state_set(key, value.to_string());
    }

    #[cfg(feature = "bot-serde")]
    pub fn user_state_get_json<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        self.user_state_get(key)
            .map(|value| serde_json::from_str(&value).map_err(|_| Error::InvalidParam))
            .transpose()
    }

    #[cfg(feature = "bot-serde")]
    pub fn user_state_get_json_or_default<T>(&self, key: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        Self::json_or_default(self.user_state_get_json(key))
    }

    #[cfg(feature = "bot-serde")]
    pub fn user_state_set_json<T>(&mut self, key: &str, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let value = serde_json::to_string(value).map_err(|_| Error::InvalidParam)?;
        self.user_state_set(key, value);
        Ok(())
    }

    #[allow(clippy::must_use_candidate)]
    pub fn channel_state_get(&self, key: &str) -> Option<String> {
        let full = self.channel_state_key(key)?;
        self.state_get(&full)
    }

    #[must_use]
    pub fn channel_state_set(&mut self, key: &str, value: impl Into<String>) -> bool {
        let Some(full) = self.channel_state_key(key) else {
            return false;
        };
        self.state_set(full, value);
        true
    }

    pub fn channel_state_remove(&mut self, key: &str) -> Option<String> {
        let full = self.channel_state_key(key)?;
        self.state_remove(&full)
    }

    pub fn channel_state_parse<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: FromStr,
    {
        self.channel_state_get(key)
            .map(|value| value.parse::<T>().map_err(|_| Error::InvalidParam))
            .transpose()
    }

    #[must_use]
    pub fn channel_state_set_typed<T>(&mut self, key: &str, value: T) -> bool
    where
        T: Display,
    {
        self.channel_state_set(key, value.to_string())
    }

    #[cfg(feature = "bot-serde")]
    pub fn channel_state_get_json<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        self.channel_state_get(key)
            .map(|value| serde_json::from_str(&value).map_err(|_| Error::InvalidParam))
            .transpose()
    }

    #[cfg(feature = "bot-serde")]
    pub fn channel_state_get_json_or_default<T>(&self, key: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        Self::json_or_default(self.channel_state_get_json(key))
    }

    #[cfg(feature = "bot-serde")]
    pub fn channel_state_set_json<T>(&mut self, key: &str, value: &T) -> Result<bool>
    where
        T: serde::Serialize,
    {
        let value = serde_json::to_string(value).map_err(|_| Error::InvalidParam)?;
        Ok(self.channel_state_set(key, value))
    }

    #[allow(clippy::must_use_candidate)]
    pub fn global_state_get(&self, key: &str) -> Option<String> {
        self.state_get(&self.global_state_key(key))
    }

    pub fn global_state_set(&mut self, key: &str, value: impl Into<String>) {
        self.state_set(self.global_state_key(key), value);
    }

    pub fn global_state_remove(&mut self, key: &str) -> Option<String> {
        self.state_remove(&self.global_state_key(key))
    }

    pub fn global_state_parse<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: FromStr,
    {
        self.global_state_get(key)
            .map(|value| value.parse::<T>().map_err(|_| Error::InvalidParam))
            .transpose()
    }

    pub fn global_state_set_typed<T>(&mut self, key: &str, value: T)
    where
        T: Display,
    {
        self.global_state_set(key, value.to_string());
    }

    #[cfg(feature = "bot-serde")]
    pub fn global_state_get_json<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        self.global_state_get(key)
            .map(|value| serde_json::from_str(&value).map_err(|_| Error::InvalidParam))
            .transpose()
    }

    #[cfg(feature = "bot-serde")]
    pub fn global_state_get_json_or_default<T>(&self, key: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        Self::json_or_default(self.global_state_get_json(key))
    }

    #[cfg(feature = "bot-serde")]
    pub fn global_state_set_json<T>(&mut self, key: &str, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let value = serde_json::to_string(value).map_err(|_| Error::InvalidParam)?;
        self.global_state_set(key, value);
        Ok(())
    }

    pub fn dialog_state_key(&mut self, key: &str) -> Option<String> {
        let state = self.dialog_current_live()?;
        let session = state.session_id().unwrap_or("0");
        Some(format!(
            "d:{}:{}:{}:{key}",
            self.sender_id().raw(),
            state.dialog,
            session
        ))
    }

    pub fn dialog_state_get(&mut self, key: &str) -> Option<String> {
        let full = self.dialog_state_key(key)?;
        self.state_get(&full)
    }

    #[must_use]
    pub fn dialog_state_set(&mut self, key: &str, value: impl Into<String>) -> bool {
        let Some(full) = self.dialog_state_key(key) else {
            return false;
        };
        self.state_set(full, value);
        true
    }

    pub fn dialog_state_remove(&mut self, key: &str) -> Option<String> {
        let full = self.dialog_state_key(key)?;
        self.state_remove(&full)
    }

    pub fn dialog_state_parse<T>(&mut self, key: &str) -> Result<Option<T>>
    where
        T: FromStr,
    {
        self.dialog_state_get(key)
            .map(|value| value.parse::<T>().map_err(|_| Error::InvalidParam))
            .transpose()
    }

    #[must_use]
    pub fn dialog_state_set_typed<T>(&mut self, key: &str, value: T) -> bool
    where
        T: Display,
    {
        self.dialog_state_set(key, value.to_string())
    }

    #[cfg(feature = "bot-serde")]
    pub fn dialog_state_get_json<T>(&mut self, key: &str) -> Result<Option<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        self.dialog_state_get(key)
            .map(|value| serde_json::from_str(&value).map_err(|_| Error::InvalidParam))
            .transpose()
    }

    #[cfg(feature = "bot-serde")]
    pub fn dialog_state_get_json_or_default<T>(&mut self, key: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        Self::json_or_default(self.dialog_state_get_json(key))
    }

    #[cfg(feature = "bot-serde")]
    pub fn dialog_state_set_json<T>(&mut self, key: &str, value: &T) -> Result<bool>
    where
        T: serde::Serialize,
    {
        let value = serde_json::to_string(value).map_err(|_| Error::InvalidParam)?;
        Ok(self.dialog_state_set(key, value))
    }

    #[must_use]
    pub fn dialog(&mut self) -> DialogMachine<'_> {
        DialogMachine::new(self.state)
    }

    pub fn dialog_start(&mut self, dialog: impl Into<String>, step: impl Into<String>) {
        let source = self.sender_id();
        self.dialog().start(source, dialog, step);
    }

    pub fn dialog_start_state(&mut self, state: DialogState) {
        let source = self.sender_id();
        self.dialog().start_state(source, state);
    }

    pub fn dialog_start_flow(&mut self, flow: &DialogFlow) {
        self.dialog_start(flow.name(), flow.start_step());
    }

    pub fn dialog_start_checked(&mut self, flow: &DialogFlow) -> Result<()> {
        if flow.name().is_empty() || flow.start_step().is_empty() {
            return Err(Error::InvalidParam);
        }
        self.dialog_start_flow(flow);
        Ok(())
    }

    pub fn dialog_current(&mut self) -> Option<DialogState> {
        let source = self.sender_id();
        self.dialog().current_active(source)
    }

    pub fn dialog_current_live(&mut self) -> Option<DialogState> {
        let source = self.sender_id();
        self.dialog().current_live(source)
    }

    pub fn dialog_advance(&mut self, step: impl Into<String>) -> Option<DialogState> {
        let source = self.sender_id();
        self.dialog().advance(source, step)
    }

    pub fn dialog_advance_checked(
        &mut self,
        flow: &DialogFlow,
        step: impl Into<String>,
    ) -> Result<Option<DialogState>> {
        let step = step.into();
        if !flow.contains_step(&step) {
            return Err(Error::InvalidParam);
        }
        Ok(self.dialog_advance(step))
    }

    pub fn dialog_restart_flow(&mut self, flow: &DialogFlow) -> Result<DialogState> {
        if flow.name().is_empty() || flow.start_step().is_empty() {
            return Err(Error::InvalidParam);
        }
        let source = self.sender_id();
        Ok(self.dialog().restart_flow(source, flow))
    }

    pub fn dialog_advance_next(&mut self, flow: &DialogFlow) -> Result<Option<DialogState>> {
        let source = self.sender_id();
        if let Some(current) = self.dialog().current_live(source) {
            if !current.dialog.eq_ignore_ascii_case(flow.name()) {
                return Err(Error::InvalidParam);
            }
            if !flow.contains_step(&current.step) {
                return Err(Error::InvalidParam);
            }
        }
        Ok(self.dialog().advance_flow(source, flow))
    }

    pub fn dialog_stop(&mut self) -> Option<DialogState> {
        let source = self.sender_id();
        self.dialog().stop(source)
    }

    pub fn dialog_cancel(&mut self) -> Option<DialogState> {
        self.dialog_stop()
    }

    pub fn dialog_finish(&mut self) -> Option<DialogState> {
        self.dialog_stop()
    }

    pub fn dialog_pause(&mut self) -> Option<DialogState> {
        let source = self.sender_id();
        self.dialog().pause(source)
    }

    pub fn dialog_resume(&mut self) -> Option<DialogState> {
        let source = self.sender_id();
        self.dialog().resume(source)
    }

    pub fn dialog_set_timeout(&mut self, timeout: Duration) -> Option<DialogState> {
        let source = self.sender_id();
        self.dialog().set_timeout(source, timeout)
    }

    pub fn dialog_clear_timeout(&mut self) -> Option<DialogState> {
        let source = self.sender_id();
        self.dialog().clear_timeout(source)
    }

    pub fn dialog_set_timeout_policy(
        &mut self,
        policy: DialogTimeoutPolicy,
    ) -> Option<DialogState> {
        let source = self.sender_id();
        self.dialog().set_timeout_policy(source, policy)
    }

    pub fn dialog_timeout_policy(&mut self) -> Option<DialogTimeoutPolicy> {
        let source = self.sender_id();
        self.dialog().timeout_policy(source)
    }

    pub fn dialog_metadata(&mut self, key: &str) -> Option<String> {
        let source = self.sender_id();
        self.dialog().metadata(source, key)
    }

    pub fn dialog_set_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Option<DialogState> {
        let source = self.sender_id();
        self.dialog().set_metadata(source, key, value)
    }

    pub fn dialog_remove_metadata(&mut self, key: &str) -> Option<(DialogState, Option<String>)> {
        let source = self.sender_id();
        self.dialog().remove_metadata(source, key)
    }

    pub fn dialog_metadata_parse<T>(&mut self, key: &str) -> Result<Option<T>>
    where
        T: FromStr,
    {
        self.dialog_metadata(key)
            .map(|value| value.parse::<T>().map_err(|_| Error::InvalidParam))
            .transpose()
    }

    #[must_use]
    pub fn dialog_is_paused(&mut self) -> bool {
        self.dialog_current_live()
            .is_some_and(|state| state.status == DialogStatus::Paused)
    }

    #[must_use]
    pub fn dialog_is(&mut self, dialog: &str, step: &str) -> bool {
        let source = self.sender_id();
        self.dialog().is_in(source, dialog, step)
    }

    pub fn dialog_name(&mut self) -> Option<String> {
        self.dialog_current_live().map(|state| state.dialog)
    }

    pub fn dialog_step(&mut self) -> Option<String> {
        self.dialog_current_live().map(|state| state.step)
    }
}
