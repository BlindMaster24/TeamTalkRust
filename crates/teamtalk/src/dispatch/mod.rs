//! Event dispatcher built on top of `Client::poll`.
use crate::client::{Client, ConnectParams, Message, ReconnectConfig, ReconnectHandler};
use crate::events::Event;

mod dispatcher;
mod source;
mod types;

pub use dispatcher::Dispatcher;
pub use source::EventSource;
pub use types::{ClientConfig, ConnectParamsOwned, DispatchFlow, EventContext, ReconnectSettings};
