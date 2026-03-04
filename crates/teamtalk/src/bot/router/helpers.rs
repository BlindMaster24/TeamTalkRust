use crate::bot::command::{Command, CommandPatternError};
use crate::events::Error;

pub(super) fn normalize_command_name(name: impl AsRef<str>) -> String {
    name.as_ref()
        .split_whitespace()
        .map(|part| part.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn join_command_path(namespace: &str, name: &str) -> String {
    if namespace.is_empty() {
        return name.to_owned();
    }
    if name.is_empty() {
        return namespace.to_owned();
    }
    format!("{namespace} {name}")
}

pub(super) fn match_command_route(command: &Command, route_name: &str) -> Option<Command> {
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

pub(super) fn pattern_error(err: CommandPatternError) -> Error {
    Error::IoError {
        message: err.to_string(),
    }
}
