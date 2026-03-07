//! High-level bot framework built on top of TeamTalk client polling.

mod app;
mod args;
mod command;
mod context;
mod fsm;
mod middleware;
mod router;
mod runtime;
mod scheduler;
mod storage;

#[cfg(feature = "async")]
mod runtime_async;

pub use app::BotApp;
pub use args::Args;
pub use command::{Command, CommandArgPattern, CommandPattern, CommandPatternError, parse_command};
pub use context::Context;
pub use fsm::{DialogFlow, DialogMachine, DialogState, DialogStatus, DialogTimeoutPolicy};
pub use middleware::{
    CommandOnly, FnMiddleware, Middleware, RateLimitBySource, RequireChannelMessage,
    RequireCommand, RequireCommandPrefix, RequirePrivateMessage, RequireUserIds, RequireUserType,
};
pub use router::{HandlerResult, RouteMatcher, Router, UnknownCommandPolicy};
pub use runtime::{Bot, BotBuilder, BotConfig};
pub use scheduler::{JobErrorPolicy, Scheduler};
#[cfg(feature = "bot-redis")]
pub use storage::RedisStateStore;
#[cfg(feature = "bot-sqlite")]
pub use storage::SqliteStateStore;
pub use storage::{MemoryStateStore, StateStore};

#[cfg(feature = "async")]
pub use runtime_async::{AsyncBot, AsyncBotBuilder, AsyncBotConfig};
