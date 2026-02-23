use super::command::{Command, CommandPattern, parse_command};
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
pub enum UnknownCommandPolicy {
    Ignore,
    Reply(String),
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
    command_pattern: Option<CommandPattern>,
    dialog_filter: Option<DialogFilter>,
    handler: Box<Handler>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DialogFilter {
    dialog: String,
    step: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HelpEntry {
    usage: String,
    summary: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AutoHelpConfig {
    enabled: bool,
    command: String,
    header: Option<String>,
    footer: Option<String>,
}

impl Default for AutoHelpConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            command: "help".to_owned(),
            header: None,
            footer: None,
        }
    }
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
        let full = join_command_path(&self.namespace, &normalize_command_name(name.into()));
        self.router.push_command_route(full.clone(), None, handler);
        self.router.register_help(full, None);
        self
    }

    pub fn on_command_with_help<F>(
        self,
        name: impl Into<String>,
        summary: impl Into<String>,
        handler: F,
    ) -> Self
    where
        F: FnMut(&mut Context<'_>) -> Result<HandlerResult> + Send + 'static,
    {
        let full = join_command_path(&self.namespace, &normalize_command_name(name.into()));
        let summary = summary.into();
        self.router.push_command_route(full.clone(), None, handler);
        self.router.register_help(full, Some(summary));
        self
    }
}

pub struct Router {
    command_prefixes: Vec<char>,
    middlewares: Vec<Box<dyn Middleware + Send>>,
    routes: Vec<Route>,
    on_unknown_command: Option<Box<Handler>>,
    unknown_command_policy: UnknownCommandPolicy,
    help_entries: Vec<HelpEntry>,
    auto_help: AutoHelpConfig,
}

impl Default for Router {
    fn default() -> Self {
        Self {
            command_prefixes: vec!['/', '!'],
            middlewares: Vec::new(),
            routes: Vec::new(),
            on_unknown_command: None,
            unknown_command_policy: UnknownCommandPolicy::Ignore,
            help_entries: Vec::new(),
            auto_help: AutoHelpConfig::default(),
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

    pub fn use_middleware_fn<F>(mut self, before: F) -> Self
    where
        F: FnMut(&mut Context<'_>) -> Result<HandlerResult> + Send + 'static,
    {
        self.middlewares
            .push(Box::new(super::FnMiddleware::new(before)));
        self
    }

    pub fn use_middleware_hooks<F, A>(mut self, before: F, after: A) -> Self
    where
        F: FnMut(&mut Context<'_>) -> Result<HandlerResult> + Send + 'static,
        A: FnMut(&mut Context<'_>) -> Result<()> + Send + 'static,
    {
        self.middlewares
            .push(Box::new(super::FnMiddleware::with_after(before, after)));
        self
    }

    pub fn on_event<F>(mut self, event: Event, handler: F) -> Self
    where
        F: FnMut(&mut Context<'_>) -> Result<HandlerResult> + Send + 'static,
    {
        self.routes.push(Route {
            matcher: RouteMatcher::Event(event),
            command_pattern: None,
            dialog_filter: None,
            handler: Box::new(handler),
        });
        self
    }

    pub fn on_command<F>(mut self, name: impl Into<String>, handler: F) -> Self
    where
        F: FnMut(&mut Context<'_>) -> Result<HandlerResult> + Send + 'static,
    {
        let command = normalize_command_name(name.into());
        self.push_command_route(command.clone(), None, handler);
        self.register_help(command, None);
        self
    }

    pub fn on_command_with_help<F>(
        mut self,
        name: impl Into<String>,
        summary: impl Into<String>,
        handler: F,
    ) -> Self
    where
        F: FnMut(&mut Context<'_>) -> Result<HandlerResult> + Send + 'static,
    {
        let command = normalize_command_name(name.into());
        let summary = summary.into();
        self.push_command_route(command.clone(), None, handler);
        self.register_help(command, Some(summary));
        self
    }

    pub fn on_command_pattern<F>(mut self, pattern: CommandPattern, handler: F) -> Self
    where
        F: FnMut(&mut Context<'_>) -> Result<HandlerResult> + Send + 'static,
    {
        self.push_command_route(pattern.command().to_owned(), Some(pattern.clone()), handler);
        self.register_help(pattern.usage(), None);
        self
    }

    pub fn on_command_pattern_with_help<F>(
        mut self,
        pattern: CommandPattern,
        summary: impl Into<String>,
        handler: F,
    ) -> Self
    where
        F: FnMut(&mut Context<'_>) -> Result<HandlerResult> + Send + 'static,
    {
        let summary = summary.into();
        self.push_command_route(pattern.command().to_owned(), Some(pattern.clone()), handler);
        self.register_help(pattern.usage(), Some(summary));
        self
    }

    pub fn try_on_command_pattern<F>(self, pattern: impl AsRef<str>, handler: F) -> Result<Self>
    where
        F: FnMut(&mut Context<'_>) -> Result<HandlerResult> + Send + 'static,
    {
        let parsed = CommandPattern::parse(pattern.as_ref()).map_err(pattern_error)?;
        Ok(self.on_command_pattern(parsed, handler))
    }

    pub fn try_on_command_pattern_with_help<F>(
        self,
        pattern: impl AsRef<str>,
        summary: impl Into<String>,
        handler: F,
    ) -> Result<Self>
    where
        F: FnMut(&mut Context<'_>) -> Result<HandlerResult> + Send + 'static,
    {
        let parsed = CommandPattern::parse(pattern.as_ref()).map_err(pattern_error)?;
        Ok(self.on_command_pattern_with_help(parsed, summary, handler))
    }

    pub fn on_dialog_step<F>(
        mut self,
        dialog: impl Into<String>,
        step: impl Into<String>,
        handler: F,
    ) -> Self
    where
        F: FnMut(&mut Context<'_>) -> Result<HandlerResult> + Send + 'static,
    {
        self.routes.push(Route {
            matcher: RouteMatcher::Any,
            command_pattern: None,
            dialog_filter: Some(DialogFilter {
                dialog: dialog.into(),
                step: Some(step.into()),
            }),
            handler: Box::new(handler),
        });
        self
    }

    pub fn on_dialog<F>(mut self, dialog: impl Into<String>, handler: F) -> Self
    where
        F: FnMut(&mut Context<'_>) -> Result<HandlerResult> + Send + 'static,
    {
        self.routes.push(Route {
            matcher: RouteMatcher::Any,
            command_pattern: None,
            dialog_filter: Some(DialogFilter {
                dialog: dialog.into(),
                step: None,
            }),
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
            namespace: normalize_command_name(namespace.into()),
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

    pub fn with_unknown_command_policy(mut self, policy: UnknownCommandPolicy) -> Self {
        self.unknown_command_policy = policy;
        self
    }

    pub fn on_any<F>(mut self, handler: F) -> Self
    where
        F: FnMut(&mut Context<'_>) -> Result<HandlerResult> + Send + 'static,
    {
        self.routes.push(Route {
            matcher: RouteMatcher::Any,
            command_pattern: None,
            dialog_filter: None,
            handler: Box::new(handler),
        });
        self
    }

    pub fn with_auto_help(mut self) -> Self {
        self.auto_help.enabled = true;
        self
    }

    pub fn without_auto_help(mut self) -> Self {
        self.auto_help.enabled = false;
        self
    }

    pub fn with_help_command(mut self, command: impl Into<String>) -> Self {
        let command = normalize_command_name(command.into());
        if !command.is_empty() {
            self.auto_help.command = command;
        }
        self
    }

    pub fn with_auto_help_command(mut self, command: impl Into<String>) -> Self {
        self.auto_help.enabled = true;
        self = self.with_help_command(command);
        self
    }

    pub fn with_help_header(mut self, header: impl Into<String>) -> Self {
        self.auto_help.header = Some(header.into());
        self
    }

    pub fn with_help_footer(mut self, footer: impl Into<String>) -> Self {
        self.auto_help.footer = Some(footer.into());
        self
    }

    pub fn dispatch(
        &mut self,
        client: &Client,
        event: Event,
        message: &Message,
        state: &mut dyn super::StateStore,
    ) -> Result<HandlerResult> {
        let parsed_command = message
            .text()
            .and_then(|text| parse_command(&text.text, &self.command_prefixes));

        let mut ctx = Context {
            client,
            event,
            message,
            command: parsed_command.clone(),
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
        let has_command = parsed_command.is_some();
        let mut matched_command = false;
        let mut matched_command_path = false;
        let mut usage_hint: Option<String> = None;
        let primary_prefix = self.primary_command_prefix();

        for route in &mut self.routes {
            ctx.command = parsed_command.clone();

            let mut should_run = match &route.matcher {
                RouteMatcher::Any => true,
                RouteMatcher::Event(expected) => expected == &ctx.event,
                RouteMatcher::Command(name) => {
                    if let Some(command) = parsed_command.as_ref() {
                        if let Some(adjusted) = match_command_route(command, name) {
                            matched_command_path = true;
                            if let Some(pattern) = route.command_pattern.as_ref() {
                                if !pattern.accepts(&adjusted.args) {
                                    if usage_hint.is_none() {
                                        usage_hint =
                                            Some(pattern.usage_with_prefix(primary_prefix));
                                    }
                                    false
                                } else {
                                    matched_command = true;
                                    ctx.command = Some(adjusted);
                                    true
                                }
                            } else {
                                matched_command = true;
                                ctx.command = Some(adjusted);
                                true
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
            };

            if should_run && let Some(filter) = route.dialog_filter.as_ref() {
                should_run = if let Some(state) = ctx.dialog_current() {
                    if !state.dialog.eq_ignore_ascii_case(&filter.dialog) {
                        false
                    } else if let Some(expected_step) = filter.step.as_ref() {
                        state.step.eq_ignore_ascii_case(expected_step)
                    } else {
                        true
                    }
                } else {
                    false
                };
            }

            if !should_run {
                continue;
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
            && let Some(command) = parsed_command.as_ref()
            && let Some(help) = self.render_auto_help(command)
        {
            let _ = ctx.reply_private(&help);
            matched_command = true;
            matched_command_path = true;
        }

        if has_command
            && !matched_command
            && matched_command_path
            && matches!(outcome, HandlerResult::Continue)
            && usage_hint.is_some()
            && let Some(usage) = usage_hint
        {
            let _ = ctx.reply_private(&format!("Usage: {usage}"));
            matched_command = true;
        }

        if has_command && !matched_command && matches!(outcome, HandlerResult::Continue) {
            if let Some(fallback) = self.on_unknown_command.as_mut() {
                outcome =
                    catch_unwind(AssertUnwindSafe(|| fallback(&mut ctx))).map_err(|_| {
                        Error::IoError {
                            message: "unknown-command handler panic".to_owned(),
                        }
                    })??;
            } else if let UnknownCommandPolicy::Reply(reply) = &self.unknown_command_policy {
                let _ = ctx.reply_private(reply);
            }
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

    fn push_command_route<F>(
        &mut self,
        command: String,
        command_pattern: Option<CommandPattern>,
        handler: F,
    ) where
        F: FnMut(&mut Context<'_>) -> Result<HandlerResult> + Send + 'static,
    {
        self.routes.push(Route {
            matcher: RouteMatcher::Command(command.clone()),
            command_pattern,
            dialog_filter: None,
            handler: Box::new(handler),
        });
    }

    fn register_help(&mut self, usage: String, summary: Option<String>) {
        if usage.is_empty() {
            return;
        }

        if let Some(entry) = self
            .help_entries
            .iter_mut()
            .find(|entry| entry.usage.eq_ignore_ascii_case(&usage))
        {
            if summary.is_some() {
                entry.summary = summary;
            }
            return;
        }

        self.help_entries.push(HelpEntry { usage, summary });
    }

    fn primary_command_prefix(&self) -> char {
        self.command_prefixes.first().copied().unwrap_or('/')
    }

    fn render_auto_help(&self, command: &Command) -> Option<String> {
        if !self.auto_help.enabled {
            return None;
        }

        let help_command = match_command_route(command, &self.auto_help.command)?;
        let query = help_command
            .args
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(" ")
            .to_ascii_lowercase();

        let mut entries = self.help_entries.clone();
        entries.push(HelpEntry {
            usage: format!("{} [command]", self.auto_help.command),
            summary: Some("Show command help".to_owned()),
        });

        entries.sort_by(|a, b| a.usage.cmp(&b.usage));
        entries.dedup_by(|a, b| a.usage.eq_ignore_ascii_case(&b.usage));

        let filtered = if query.is_empty() {
            entries
        } else {
            entries
                .into_iter()
                .filter(|entry| {
                    let usage = entry.usage.to_ascii_lowercase();
                    usage == query
                        || usage.starts_with(&format!("{query} "))
                        || usage.contains(&query)
                })
                .collect::<Vec<_>>()
        };

        let prefix = self.primary_command_prefix();
        let mut lines = Vec::new();
        if let Some(header) = &self.auto_help.header {
            lines.push(header.clone());
        } else {
            lines.push("Available commands:".to_owned());
        }

        if filtered.is_empty() {
            if query.is_empty() {
                lines.push("(no commands registered)".to_owned());
            } else {
                lines.push(format!("No commands found for '{query}'."));
            }
        } else {
            for entry in filtered {
                if let Some(summary) = &entry.summary {
                    lines.push(format!("- {prefix}{}  -  {summary}", entry.usage));
                } else {
                    lines.push(format!("- {prefix}{}", entry.usage));
                }
            }
        }

        if let Some(footer) = &self.auto_help.footer {
            lines.push(footer.clone());
        }

        Some(lines.join("\n"))
    }
}

fn normalize_command_name(name: impl AsRef<str>) -> String {
    name.as_ref()
        .split_whitespace()
        .map(|part| part.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join(" ")
}

fn join_command_path(namespace: &str, name: &str) -> String {
    if namespace.is_empty() {
        return name.to_owned();
    }
    if name.is_empty() {
        return namespace.to_owned();
    }
    format!("{namespace} {name}")
}

fn match_command_route(command: &Command, route_name: &str) -> Option<Command> {
    let route = normalize_command_name(route_name);
    if route.is_empty() {
        return None;
    }

    let route_parts = route.split_whitespace().collect::<Vec<_>>();
    let command_parts = command.tokens();
    if command_parts.len() < route_parts.len() {
        return None;
    }

    for (actual, expected) in command_parts.iter().zip(route_parts.iter()) {
        if !actual.eq_ignore_ascii_case(expected) {
            return None;
        }
    }

    let remaining = command_parts[route_parts.len()..]
        .iter()
        .map(|item| (*item).to_owned())
        .collect::<Vec<_>>();

    Some(Command {
        prefix: command.prefix,
        name: route,
        args: remaining,
        raw: command.raw.clone(),
    })
}

fn pattern_error(err: super::command::CommandPatternError) -> Error {
    Error::IoError {
        message: err.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::{HandlerResult, Router, match_command_route};
    use crate::bot::{Command, CommandPattern};

    #[test]
    fn command_route_matching_trims_namespace_tokens() {
        let command = Command {
            prefix: '/',
            name: "admin".to_owned(),
            args: vec!["ban".to_owned(), "alice".to_owned()],
            raw: "admin ban alice".to_owned(),
        };

        let routed = match_command_route(&command, "admin ban").expect("match route");
        assert_eq!(routed.name, "admin ban");
        assert_eq!(routed.args, vec!["alice".to_owned()]);
    }

    #[test]
    fn auto_help_uses_custom_command_name() {
        let router = Router::new()
            .on_command_with_help("ping", "Ping command", |_ctx| Ok(HandlerResult::Continue))
            .on_command_pattern_with_help(
                CommandPattern::parse("ban <user> [reason...]").expect("pattern"),
                "Ban user",
                |_ctx| Ok(HandlerResult::Continue),
            )
            .with_auto_help_command("commands")
            .with_help_header("Bot commands");

        let command = Command {
            prefix: '/',
            name: "commands".to_owned(),
            args: vec!["ban".to_owned()],
            raw: "commands ban".to_owned(),
        };

        let help = router.render_auto_help(&command).expect("help text");
        assert!(help.contains("Bot commands"));
        assert!(help.contains("/ban <user> [reason...]"));
        assert!(!help.contains("/ping"));
    }
}
