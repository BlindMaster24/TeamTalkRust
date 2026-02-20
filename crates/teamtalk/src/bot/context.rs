use super::command::Command;
use super::fsm::{DialogMachine, DialogState};
use super::storage::StateStore;
use crate::client::{Client, Message};
use crate::events::Event;

pub struct Context<'a> {
    pub client: &'a Client,
    pub event: Event,
    pub message: &'a Message,
    pub command: Option<Command>,
    pub state: &'a mut dyn StateStore,
}

impl<'a> Context<'a> {
    pub fn command_name(&self) -> Option<&str> {
        self.command.as_ref().map(|c| c.name.as_str())
    }

    pub fn command_arg(&self, index: usize) -> Option<&str> {
        self.command.as_ref()?.arg(index)
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
