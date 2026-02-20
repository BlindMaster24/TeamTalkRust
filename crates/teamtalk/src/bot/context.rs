use super::args::Args;
use super::command::Command;
use super::fsm::{DialogMachine, DialogState};
use super::storage::StateStore;
use crate::client::{Client, Message};
use crate::events::Event;
use crate::types::{ChannelId, UserId};

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

    pub fn dialog_current(&mut self) -> Option<DialogState> {
        let source = self.message.source();
        self.dialog().current(source)
    }

    pub fn dialog_advance(&mut self, step: impl Into<String>) -> Option<DialogState> {
        let source = self.message.source();
        self.dialog().advance(source, step)
    }

    pub fn dialog_stop(&mut self) -> Option<DialogState> {
        let source = self.message.source();
        self.dialog().stop(source)
    }
}
