use super::*;

impl Router {
    pub(super) fn push_command_route<F>(
        &mut self,
        command: &str,
        command_pattern: Option<CommandPattern>,
        handler: F,
    ) where
        F: FnMut(&mut Context<'_>) -> Result<HandlerResult> + Send + 'static,
    {
        self.routes.push(Route {
            matcher: RouteMatcher::Command(command.to_string()),
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

    pub(super) fn canonicalize_command(&self, command: &Command) -> Command {
        let tokens = command.tokens();
        if tokens.is_empty() {
            return command.clone();
        }
        let normalized = tokens
            .iter()
            .map(|token| token.to_ascii_lowercase())
            .collect::<Vec<_>>();

        for prefix_len in (1..=tokens.len()).rev() {
            let alias = normalized[..prefix_len].join(" ");
            let Some(target) = self.command_aliases.get(&alias) else {
                continue;
            };
            let mut new_tokens = target
                .split_whitespace()
                .map(str::to_owned)
                .collect::<Vec<_>>();
            new_tokens.extend(tokens[prefix_len..].iter().map(|token| (*token).to_owned()));
            if new_tokens.is_empty() {
                return command.clone();
            }
            let name = new_tokens.remove(0);
            return Command {
                prefix: command.prefix,
                name,
                args: new_tokens,
                raw: command.raw.clone(),
            };
        }

        command.clone()
    }

    pub(super) fn suggest_commands(&self, query: &str) -> Vec<String> {
        if !self.suggestions.enabled || query.is_empty() {
            return Vec::new();
        }

        let mut commands = self
            .help_entries
            .iter()
            .map(|entry| {
                entry
                    .usage
                    .split_whitespace()
                    .take(2)
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .filter(|usage| !usage.is_empty())
            .collect::<Vec<_>>();
        commands.extend(self.command_aliases.keys().cloned());
        commands.sort();
        commands.dedup_by(|left, right| left.eq_ignore_ascii_case(right));

        let lower = query.to_ascii_lowercase();
        let max_distance = self.suggestions.max_distance.max(1);
        let mut ranked = commands
            .into_iter()
            .map(|candidate| {
                let candidate_lower = candidate.to_ascii_lowercase();
                let distance = edit_distance(&candidate_lower, &lower);
                let starts = candidate_lower.starts_with(&lower);
                (distance, !starts, candidate)
            })
            .filter(|(distance, _, candidate)| {
                *distance <= max_distance || candidate.to_ascii_lowercase().contains(&lower)
            })
            .collect::<Vec<_>>();
        ranked.sort();
        ranked
            .into_iter()
            .take(self.suggestions.limit)
            .map(|(_, _, candidate)| candidate)
            .collect()
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
