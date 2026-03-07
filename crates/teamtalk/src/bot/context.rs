use super::args::Args;
use super::command::Command;
use super::fsm::{DialogFlow, DialogMachine, DialogState, DialogStatus};
use super::storage::StateStore;
use crate::client::{Client, Message};
use crate::events::{Error, Event, Result};
use crate::types::{ChannelId, UserId};
use std::time::Duration;

pub struct Context<'a> {
    pub client: &'a Client,
    pub event: Event,
    pub message: &'a Message,
    pub command: Option<Command>,
    pub state: &'a mut dyn StateStore,
}

impl<'a> Context<'a> {
    pub fn is_command(&self, name: &str) -> bool {
        self.command_name() == Some(name)
    }

    pub fn command_name(&self) -> Option<&str> {
        self.command.as_ref().map(|c| c.name.as_str())
    }

    pub fn args(&self) -> Option<Args<'_>> {
        self.command.as_ref().map(|c| Args::new(&c.args))
    }

    pub fn command_arg(&self, index: usize) -> Option<&str> {
        self.command.as_ref()?.arg(index)
    }

    pub fn text(&self) -> Option<String> {
        self.message.text().map(|text| text.text)
    }

    pub fn sender_id(&self) -> UserId {
        UserId(self.message.source())
    }

    pub fn channel_id(&self) -> Option<ChannelId> {
        self.message.text().map(|text| text.channel_id)
    }

    pub fn reply_private(&self, text: &str) -> i32 {
        self.client.send_to_user(self.sender_id(), text)
    }

    pub fn reply_channel(&self, text: &str) -> Option<i32> {
        self.channel_id()
            .map(|channel_id| self.client.send_to_channel(channel_id, text))
    }

    pub fn reply(&self, text: &str) -> i32 {
        if let Some(channel_id) = self.channel_id() {
            return self.client.send_to_channel(channel_id, text);
        }
        self.reply_private(text)
    }

    pub fn wait_for_event(&self, event: Event, timeout: Duration) -> Option<Message> {
        self.client
            .wait_for(event, timeout.as_millis().min(i32::MAX as u128) as i32)
    }

    pub fn wait_text_from(&self, from: UserId, timeout: Duration) -> Option<Message> {
        self.client
            .poll_until(
                timeout.as_millis().min(i32::MAX as u128) as i32,
                |event, msg| event == Event::TextMessage && msg.source() == from.0,
            )
            .map(|(_, msg)| msg)
    }

    pub fn wait_command_from_sender(
        &self,
        command_name: &str,
        timeout: Duration,
    ) -> Option<Message> {
        let sender = self.sender_id().0;
        let expected = command_name.to_ascii_lowercase();
        self.client
            .poll_until(
                timeout.as_millis().min(i32::MAX as u128) as i32,
                |event, msg| {
                    if event != Event::TextMessage || msg.source() != sender {
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

    pub fn state_get(&self, key: &str) -> Option<String> {
        self.state.get(key)
    }

    pub fn state_set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.state.set(key.into(), value.into());
    }

    pub fn state_remove(&mut self, key: &str) -> Option<String> {
        self.state.remove(key)
    }

    pub fn user_state_key(&self, key: &str) -> String {
        format!("u:{}:{key}", self.sender_id().0)
    }

    pub fn channel_state_key(&self, key: &str) -> Option<String> {
        self.channel_id()
            .map(|channel| format!("c:{}:{key}", channel.0))
    }

    pub fn global_state_key(&self, key: &str) -> String {
        format!("g:{key}")
    }

    pub fn user_state_get(&self, key: &str) -> Option<String> {
        self.state_get(&self.user_state_key(key))
    }

    pub fn user_state_set(&mut self, key: &str, value: impl Into<String>) {
        self.state_set(self.user_state_key(key), value);
    }

    pub fn user_state_remove(&mut self, key: &str) -> Option<String> {
        self.state_remove(&self.user_state_key(key))
    }

    pub fn channel_state_get(&self, key: &str) -> Option<String> {
        let full = self.channel_state_key(key)?;
        self.state_get(&full)
    }

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

    pub fn global_state_get(&self, key: &str) -> Option<String> {
        self.state_get(&self.global_state_key(key))
    }

    pub fn global_state_set(&mut self, key: &str, value: impl Into<String>) {
        self.state_set(self.global_state_key(key), value);
    }

    pub fn global_state_remove(&mut self, key: &str) -> Option<String> {
        self.state_remove(&self.global_state_key(key))
    }

    pub fn dialog(&mut self) -> DialogMachine<'_> {
        DialogMachine::new(self.state)
    }

    pub fn dialog_start(&mut self, dialog: impl Into<String>, step: impl Into<String>) {
        let source = self.message.source();
        self.dialog().start(source, dialog, step);
    }

    pub fn dialog_start_state(&mut self, state: DialogState) {
        let source = self.message.source();
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
        let source = self.message.source();
        self.dialog().current_active(source)
    }

    pub fn dialog_current_live(&mut self) -> Option<DialogState> {
        let source = self.message.source();
        self.dialog().current_live(source)
    }

    pub fn dialog_advance(&mut self, step: impl Into<String>) -> Option<DialogState> {
        let source = self.message.source();
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

    pub fn dialog_stop(&mut self) -> Option<DialogState> {
        let source = self.message.source();
        self.dialog().stop(source)
    }

    pub fn dialog_pause(&mut self) -> Option<DialogState> {
        let source = self.message.source();
        self.dialog().pause(source)
    }

    pub fn dialog_resume(&mut self) -> Option<DialogState> {
        let source = self.message.source();
        self.dialog().resume(source)
    }

    pub fn dialog_set_timeout(&mut self, timeout: Duration) -> Option<DialogState> {
        let source = self.message.source();
        self.dialog().set_timeout(source, timeout)
    }

    pub fn dialog_clear_timeout(&mut self) -> Option<DialogState> {
        let source = self.message.source();
        self.dialog().clear_timeout(source)
    }

    pub fn dialog_metadata(&mut self, key: &str) -> Option<String> {
        let source = self.message.source();
        self.dialog().metadata(source, key)
    }

    pub fn dialog_set_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Option<DialogState> {
        let source = self.message.source();
        self.dialog().set_metadata(source, key, value)
    }

    pub fn dialog_remove_metadata(&mut self, key: &str) -> Option<(DialogState, Option<String>)> {
        let source = self.message.source();
        self.dialog().remove_metadata(source, key)
    }

    pub fn dialog_is_paused(&mut self) -> bool {
        self.dialog_current_live()
            .is_some_and(|state| state.status == DialogStatus::Paused)
    }

    pub fn dialog_is(&mut self, dialog: &str, step: &str) -> bool {
        let source = self.message.source();
        self.dialog().is_in(source, dialog, step)
    }
}
