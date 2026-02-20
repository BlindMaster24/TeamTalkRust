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
