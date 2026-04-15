use super::*;

impl Router {
    #[allow(clippy::too_many_lines)]
    pub fn dispatch(
        &mut self,
        client: &Client,
        event: Event,
        message: &Message,
        state: &mut dyn super::super::StateStore,
    ) -> Result<HandlerResult> {
        let parsed_command = message
            .text()
            .and_then(|text| parse_command(&text.text, &self.command_prefixes));
        let parsed_command = parsed_command
            .as_ref()
            .map(|command| self.canonicalize_command(command));

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
            ctx.command.clone_from(&parsed_command);

            if has_command && matched_command && route.dialog_filter.is_some() {
                continue;
            }

            let mut should_run = match &route.matcher {
                RouteMatcher::Any => true,
                RouteMatcher::Event(expected) => expected == &ctx.event,
                RouteMatcher::Command(name) => {
                    if let Some(command) = parsed_command.as_ref() {
                        if let Some(adjusted) = match_command_route(command, name) {
                            matched_command_path = true;
                            if let Some(pattern) = route.command_pattern.as_ref() {
                                if pattern.accepts(&adjusted.args) {
                                    matched_command = true;
                                    ctx.command = Some(adjusted);
                                    true
                                } else {
                                    if usage_hint.is_none() {
                                        usage_hint =
                                            Some(pattern.usage_with_prefix(primary_prefix));
                                    }
                                    false
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
                let mut reply = reply.clone();
                if let Some(command) = parsed_command.as_ref() {
                    let suggestions = self.suggest_commands(&command.name);
                    if !suggestions.is_empty() {
                        reply.push_str("\nDid you mean: ");
                        reply.push_str(&suggestions.join(", "));
                        reply.push('?');
                    }
                }
                let _ = ctx.reply_private(&reply);
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
}
