use super::*;

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
            .push(Box::new(super::super::FnMiddleware::new(before)));
        self
    }

    pub fn use_middleware_hooks<F, A>(mut self, before: F, after: A) -> Self
    where
        F: FnMut(&mut Context<'_>) -> Result<HandlerResult> + Send + 'static,
        A: FnMut(&mut Context<'_>) -> Result<()> + Send + 'static,
    {
        self.middlewares
            .push(Box::new(super::super::FnMiddleware::with_after(
                before, after,
            )));
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

    pub fn alias_command(mut self, alias: impl Into<String>, target: impl Into<String>) -> Self {
        let alias = normalize_command_name(alias.into());
        let target = normalize_command_name(target.into());
        if !alias.is_empty() && !target.is_empty() {
            self.command_aliases.insert(alias, target);
        }
        self
    }

    pub fn with_unknown_command_policy(mut self, policy: UnknownCommandPolicy) -> Self {
        self.unknown_command_policy = policy;
        self
    }

    pub fn with_unknown_command_suggestions(mut self, limit: usize) -> Self {
        self.suggestions.enabled = true;
        self.suggestions.limit = limit.max(1);
        self
    }

    pub fn with_unknown_command_suggestion_distance(mut self, max_distance: usize) -> Self {
        self.suggestions.enabled = true;
        self.suggestions.max_distance = max_distance.max(1);
        self
    }

    pub fn without_unknown_command_suggestions(mut self) -> Self {
        self.suggestions.enabled = false;
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
}
