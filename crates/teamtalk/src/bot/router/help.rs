use super::*;

impl Router {
    pub(super) fn push_command_route<F>(
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

    pub(super) fn register_help(&mut self, usage: String, summary: Option<String>) {
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

    pub(super) fn primary_command_prefix(&self) -> char {
        self.command_prefixes.first().copied().unwrap_or('/')
    }

    pub(super) fn render_auto_help(&self, command: &Command) -> Option<String> {
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
