use super::{Client, Message};
use crate::events::Event;
use crate::types::{ChannelId, TextMessage, User};

pub type EventHook = Box<dyn FnMut(&Client, Event, &Message) + Send>;
pub type ClientHook = Box<dyn FnMut(&Client) + Send>;
pub type ChannelHook = Box<dyn FnMut(&Client, ChannelId) + Send>;
pub type UserHook = Box<dyn FnMut(&Client, User) + Send>;
pub type TextHook = Box<dyn FnMut(&Client, TextMessage) + Send>;
pub type MessageHook = Box<dyn FnMut(&Client, &Message) + Send>;
