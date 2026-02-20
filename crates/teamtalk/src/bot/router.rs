use super::command::parse_command;
use super::context::Context;
use super::middleware::Middleware;
use crate::client::{Client, Message};
use crate::events::{Error, Event, Result};
use std::panic::{AssertUnwindSafe, catch_unwind};

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

pub struct RouteGroup<'a> {
    router: &'a mut Router,
    namespace: String,
}

impl<'a> RouteGroup<'a> {
    pub fn on_command<F>(self, name: impl Into<String>, handler: F) -> Self
    where
        F: FnMut(&mut Context<'_>) -> Result<HandlerResult> + Send + 'static,
    {
        let full = format!("{} {}", self.namespace, name.into().to_ascii_lowercase());
        self.router.routes.push(Route {
            matcher: RouteMatcher::Command(full),
            handler: Box::new(handler),
        });
        self
    }
}

pub struct Router {
    command_prefixes: Vec<char>,
    middlewares: Vec<Box<dyn Middleware + Send>>,
    routes: Vec<Route>,
    on_unknown_command: Option<Box<Handler>>,
}

impl Default for Router {
    fn default() -> Self {
        Self {
            command_prefixes: vec!['/', '!'],
            middlewares: Vec::new(),
            routes: Vec::new(),
            on_unknown_command: None,
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

    pub fn command_group<F>(mut self, namespace: impl Into<String>, configure: F) -> Self
    where
        F: FnOnce(RouteGroup<'_>) -> RouteGroup<'_>,
    {
        let group = RouteGroup {
            router: &mut self,
            namespace: namespace.into().to_ascii_lowercase(),
        };
        let _ = configure(group);
        self
    }

    pub fn on_unknown_command<F>(mut self, handler: F) -> Self
    where
        F: FnMut(&mut Context<'_>) -> Result<HandlerResult> + Send + 'static,
    {
        self.on_unknown_command = Some(Box::new(handler));
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
            let result = catch_unwind(AssertUnwindSafe(|| middleware.before(&mut ctx))).map_err(
                |_| Error::IoError {
                    message: "middleware panic in before()".to_owned(),
                },
            )??;
            if matches!(result, HandlerResult::Stop) {
                return Ok(HandlerResult::Stop);
            }
        }

        let mut outcome = HandlerResult::Continue;
        let has_command = ctx.command.is_some();
        let mut matched_command = false;

        for route in &mut self.routes {
            let matches = route.matcher.matches(&ctx);
            if !matches {
                continue;
            }

            if matches!(route.matcher, RouteMatcher::Command(_)) {
                matched_command = true;
            }

            outcome =
                catch_unwind(AssertUnwindSafe(|| (route.handler)(&mut ctx))).map_err(|_| {
                    Error::IoError {
                        message: "handler panic".to_owned(),
                    }
                })??;
            if matches!(outcome, HandlerResult::Stop) {
                break;
            }
        }

        if has_command
            && !matched_command
            && matches!(outcome, HandlerResult::Continue)
            && let Some(fallback) = self.on_unknown_command.as_mut()
        {
            outcome = catch_unwind(AssertUnwindSafe(|| fallback(&mut ctx))).map_err(|_| {
                Error::IoError {
                    message: "unknown-command handler panic".to_owned(),
                }
            })??;
        }

        for middleware in self.middlewares.iter_mut().rev() {
            catch_unwind(AssertUnwindSafe(|| middleware.after(&mut ctx))).map_err(|_| {
                Error::IoError {
                    message: "middleware panic in after()".to_owned(),
                }
            })??;
        }

        Ok(outcome)
    }
}

impl RouteMatcher {
    fn matches(&self, ctx: &Context<'_>) -> bool {
        match self {
            Self::Any => true,
            Self::Event(event) => event == &ctx.event,
            Self::Command(name) => {
                if let Some(command) = ctx.command.as_ref() {
                    if command.name == *name {
                        return true;
                    }

                    if let Some((prefix, rest)) = command.raw.split_once(' ') {
                        let full = format!("{} {}", prefix.to_ascii_lowercase(), rest);
                        return full == *name;
                    }
                }
                false
            }
        }
    }
}
