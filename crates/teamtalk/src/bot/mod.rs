//! High-level bot framework built on top of TeamTalk client polling.

mod command;
mod context;
mod middleware;
mod router;
mod runtime;
mod scheduler;
mod storage;

#[cfg(feature = "async")]
mod runtime_async;

pub use command::{Command, parse_command};
pub use context::Context;
pub use middleware::Middleware;
pub use router::{HandlerResult, RouteMatcher, Router};
pub use runtime::{Bot, BotBuilder, BotConfig};
pub use scheduler::{JobErrorPolicy, Scheduler};
pub use storage::{MemoryStateStore, StateStore};

#[cfg(feature = "async")]
pub use runtime_async::{AsyncBot, AsyncBotBuilder, AsyncBotConfig};
