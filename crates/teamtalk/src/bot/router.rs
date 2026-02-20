use super::command::parse_command;
use super::context::Context;
use super::middleware::Middleware;
use crate::client::{Client, Message};
use crate::events::{Event, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlerResult {
    Continue,
    Stop,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteMatcher {
    Any,
    Event(Event),
    Command(String),
}

type Handler = dyn FnMut(&mut Context<'_>) -> Result<HandlerResult> + Send;

struct Route {
    matcher: RouteMatcher,
    handler: Box<Handler>,
}

pub struct Router {
    command_prefixes: Vec<char>,
    middlewares: Vec<Box<dyn Middleware + Send>>,
    routes: Vec<Route>,
}

impl Default for Router {
    fn default() -> Self {
        Self {
            command_prefixes: vec!['/', '!'],
            middlewares: Vec::new(),
            routes: Vec::new(),
        }
    }
}

impl Router {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_command_prefixes(mut self, prefixes: impl Into<Vec<char>>) -> Self {
        self.command_prefixes = prefixes.into();
        self
    }

    pub fn use_middleware<M>(mut self, middleware: M) -> Self
    where
        M: Middleware + Send + 'static,
    {
        self.middlewares.push(Box::new(middleware));
        self
    }

    pub fn on_event<F>(mut self, event: Event, handler: F) -> Self
    where
        F: FnMut(&mut Context<'_>) -> Result<HandlerResult> + Send + 'static,
    {
        self.routes.push(Route {
            matcher: RouteMatcher::Event(event),
            handler: Box::new(handler),
        });
        self
    }

    pub fn on_command<F>(mut self, name: impl Into<String>, handler: F) -> Self
    where
        F: FnMut(&mut Context<'_>) -> Result<HandlerResult> + Send + 'static,
    {
        self.routes.push(Route {
            matcher: RouteMatcher::Command(name.into().to_ascii_lowercase()),
            handler: Box::new(handler),
        });
        self
    }

    pub fn on_any<F>(mut self, handler: F) -> Self
    where
        F: FnMut(&mut Context<'_>) -> Result<HandlerResult> + Send + 'static,
    {
        self.routes.push(Route {
            matcher: RouteMatcher::Any,
            handler: Box::new(handler),
        });
        self
    }

    pub fn dispatch(
        &mut self,
        client: &Client,
        event: Event,
        message: &Message,
        state: &mut dyn super::StateStore,
    ) -> Result<HandlerResult> {
        let command = message
            .text()
            .and_then(|text| parse_command(&text.text, &self.command_prefixes));

        let mut ctx = Context {
            client,
            event,
            message,
            command,
            state,
        };

        for middleware in &mut self.middlewares {
            if matches!(middleware.before(&mut ctx)?, HandlerResult::Stop) {
                return Ok(HandlerResult::Stop);
            }
        }

        let mut outcome = HandlerResult::Continue;
        for route in &mut self.routes {
            if !route.matcher.matches(&ctx) {
                continue;
            }

            outcome = (route.handler)(&mut ctx)?;
            if matches!(outcome, HandlerResult::Stop) {
                break;
            }
        }

        for middleware in self.middlewares.iter_mut().rev() {
            middleware.after(&mut ctx)?;
        }

        Ok(outcome)
    }
}

impl RouteMatcher {
    fn matches(&self, ctx: &Context<'_>) -> bool {
        match self {
            Self::Any => true,
            Self::Event(event) => event == &ctx.event,
            Self::Command(name) => ctx
                .command
                .as_ref()
                .is_some_and(|command| command.name == *name),
        }
    }
}
