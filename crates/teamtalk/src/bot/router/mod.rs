use super::command::{Command, CommandPattern, parse_command};
use super::context::Context;
use super::middleware::Middleware;
use crate::client::{Client, Message};
use crate::events::{Error, Event, Result};
use std::collections::HashMap;
use std::panic::{AssertUnwindSafe, catch_unwind};

mod helpers;

use helpers::{
    edit_distance, join_command_path, match_command_route, normalize_command_name, pattern_error,
};

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

#[derive(Debug, Clone, PartialEq, Eq)]
struct SuggestionConfig {
    enabled: bool,
    limit: usize,
}

impl Default for SuggestionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            limit: 3,
        }
    }
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
    command_aliases: HashMap<String, String>,
    on_unknown_command: Option<Box<Handler>>,
    unknown_command_policy: UnknownCommandPolicy,
    help_entries: Vec<HelpEntry>,
    auto_help: AutoHelpConfig,
    suggestions: SuggestionConfig,
}

impl Default for Router {
    fn default() -> Self {
        Self {
            command_prefixes: vec!['/', '!'],
            middlewares: Vec::new(),
            routes: Vec::new(),
            command_aliases: HashMap::new(),
            on_unknown_command: None,
            unknown_command_policy: UnknownCommandPolicy::Ignore,
            help_entries: Vec::new(),
            auto_help: AutoHelpConfig::default(),
            suggestions: SuggestionConfig::default(),
        }
    }
}

mod builder;
mod dispatch;
mod help;

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

    #[test]
    fn canonicalize_command_uses_alias() {
        let router = Router::new().alias_command("p", "ping");
        let command = Command {
            prefix: '/',
            name: "p".to_owned(),
            args: vec!["now".to_owned()],
            raw: "p now".to_owned(),
        };

        let canonical = router.canonicalize_command(&command);
        assert_eq!(canonical.name, "ping");
        assert_eq!(canonical.args, vec!["now".to_owned()]);
    }

    #[test]
    fn suggest_commands_prefers_close_matches() {
        let router = Router::new()
            .on_command("ping", |_ctx| Ok(HandlerResult::Continue))
            .on_command("pause", |_ctx| Ok(HandlerResult::Continue))
            .with_unknown_command_suggestions(2);

        let suggestions = router.suggest_commands("pnig");
        assert!(suggestions.iter().any(|item| item == "ping"));
    }
}
