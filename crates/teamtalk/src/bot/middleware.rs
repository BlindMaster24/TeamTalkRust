use super::router::HandlerResult;
use crate::events::Result;
use crate::types::{UserId, UserRights};
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

pub struct RequireCommandPrefix {
    prefix: char,
}

impl RequireCommandPrefix {
    pub fn new(prefix: char) -> Self {
        Self { prefix }
    }
}

impl Middleware for RequireCommandPrefix {
    fn before(&mut self, ctx: &mut super::Context<'_>) -> Result<HandlerResult> {
        Ok(
            if ctx
                .command
                .as_ref()
                .is_some_and(|command| command.prefix == self.prefix)
            {
                HandlerResult::Continue
            } else {
                HandlerResult::Stop
            },
        )
    }
}

pub struct RequireUserIds {
    allowed: Vec<UserId>,
}

impl RequireUserIds {
    pub fn new(allowed: impl Into<Vec<UserId>>) -> Self {
        Self {
            allowed: allowed.into(),
        }
    }
}

impl Middleware for RequireUserIds {
    fn before(&mut self, ctx: &mut super::Context<'_>) -> Result<HandlerResult> {
        Ok(if self.allowed.contains(&ctx.sender_id()) {
            HandlerResult::Continue
        } else {
            HandlerResult::Stop
        })
    }
}

pub struct RequireUserType {
    allowed: Vec<u32>,
}

impl RequireUserType {
    pub fn new(allowed: impl Into<Vec<u32>>) -> Self {
        Self {
            allowed: allowed.into(),
        }
    }
}

impl Middleware for RequireUserType {
    fn before(&mut self, ctx: &mut super::Context<'_>) -> Result<HandlerResult> {
        let mut raw = unsafe { std::mem::zeroed::<teamtalk_sys::User>() };
        if !ctx
            .client
            .backend()
            .get_user(ctx.client.ptr.0, ctx.sender_id().0, &mut raw)
        {
            return Ok(HandlerResult::Stop);
        }
        Ok(if self.allowed.contains(&raw.uUserType) {
            HandlerResult::Continue
        } else {
            HandlerResult::Stop
        })
    }
}

pub struct RequireClientRightsAny {
    rights: UserRights,
}

impl RequireClientRightsAny {
    pub fn new(rights: UserRights) -> Self {
        Self { rights }
    }
}

impl Middleware for RequireClientRightsAny {
    fn before(&mut self, ctx: &mut super::Context<'_>) -> Result<HandlerResult> {
        Ok(if ctx.client.my_user_rights().has_any(self.rights) {
            HandlerResult::Continue
        } else {
            HandlerResult::Stop
        })
    }
}

pub struct RequireClientRightsAll {
    rights: UserRights,
}

impl RequireClientRightsAll {
    pub fn new(rights: UserRights) -> Self {
        Self { rights }
    }
}

impl Middleware for RequireClientRightsAll {
    fn before(&mut self, ctx: &mut super::Context<'_>) -> Result<HandlerResult> {
        Ok(if ctx.client.my_user_rights().has_all(self.rights) {
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
