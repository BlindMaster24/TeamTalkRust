use super::router::HandlerResult;
use crate::events::Result;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub trait Middleware {
    fn before(&mut self, _ctx: &mut super::Context<'_>) -> Result<HandlerResult> {
        Ok(HandlerResult::Continue)
    }

    fn after(&mut self, _ctx: &mut super::Context<'_>) -> Result<()> {
        Ok(())
    }
}

type BeforeHook = dyn FnMut(&mut super::Context<'_>) -> Result<HandlerResult> + Send;
type AfterHook = dyn FnMut(&mut super::Context<'_>) -> Result<()> + Send;

pub struct FnMiddleware {
    before: Box<BeforeHook>,
    after: Option<Box<AfterHook>>,
}

impl FnMiddleware {
    pub fn new<F>(before: F) -> Self
    where
        F: FnMut(&mut super::Context<'_>) -> Result<HandlerResult> + Send + 'static,
    {
        Self {
            before: Box::new(before),
            after: None,
        }
    }

    pub fn with_after<F, A>(before: F, after: A) -> Self
    where
        F: FnMut(&mut super::Context<'_>) -> Result<HandlerResult> + Send + 'static,
        A: FnMut(&mut super::Context<'_>) -> Result<()> + Send + 'static,
    {
        Self {
            before: Box::new(before),
            after: Some(Box::new(after)),
        }
    }
}

impl Middleware for FnMiddleware {
    fn before(&mut self, ctx: &mut super::Context<'_>) -> Result<HandlerResult> {
        (self.before)(ctx)
    }

    fn after(&mut self, ctx: &mut super::Context<'_>) -> Result<()> {
        if let Some(after) = self.after.as_mut() {
            return after(ctx);
        }
        Ok(())
    }
}

pub struct CommandOnly;

impl Middleware for CommandOnly {
    fn before(&mut self, ctx: &mut super::Context<'_>) -> Result<HandlerResult> {
        Ok(if ctx.command.is_some() {
            HandlerResult::Continue
        } else {
            HandlerResult::Stop
        })
    }
}

pub struct RequirePrivateMessage;

impl Middleware for RequirePrivateMessage {
    fn before(&mut self, ctx: &mut super::Context<'_>) -> Result<HandlerResult> {
        Ok(if ctx.channel_id().is_none() {
            HandlerResult::Continue
        } else {
            HandlerResult::Stop
        })
    }
}

pub struct RequireChannelMessage;

impl Middleware for RequireChannelMessage {
    fn before(&mut self, ctx: &mut super::Context<'_>) -> Result<HandlerResult> {
        Ok(if ctx.channel_id().is_some() {
            HandlerResult::Continue
        } else {
            HandlerResult::Stop
        })
    }
}

pub struct RequireCommand {
    command: String,
}

impl RequireCommand {
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
        }
    }
}

impl Middleware for RequireCommand {
    fn before(&mut self, ctx: &mut super::Context<'_>) -> Result<HandlerResult> {
        Ok(if ctx.is_command(&self.command) {
            HandlerResult::Continue
        } else {
            HandlerResult::Stop
        })
    }
}

pub struct RateLimitBySource {
    period: Duration,
    seen: HashMap<i32, Instant>,
}

impl RateLimitBySource {
    pub fn new(period: Duration) -> Self {
        Self {
            period: period.max(Duration::from_millis(50)),
            seen: HashMap::new(),
        }
    }
}

impl Middleware for RateLimitBySource {
    fn before(&mut self, ctx: &mut super::Context<'_>) -> Result<HandlerResult> {
        let source = ctx.message.source();
        let now = Instant::now();
        if let Some(last) = self.seen.get(&source)
            && now.duration_since(*last) < self.period
        {
            return Ok(HandlerResult::Stop);
        }
        self.seen.insert(source, now);
        Ok(HandlerResult::Continue)
    }
}
