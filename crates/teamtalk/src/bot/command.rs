#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub prefix: char,
    pub name: String,
    pub args: Vec<String>,
    pub raw: String,
}

impl Command {
    pub fn arg(&self, index: usize) -> Option<&str> {
        self.args.get(index).map(String::as_str)
    }

    pub(crate) fn tokens(&self) -> Vec<&str> {
        let mut tokens = Vec::with_capacity(self.args.len() + 1);
        tokens.push(self.name.as_str());
        tokens.extend(self.args.iter().map(String::as_str));
        tokens
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandArgPattern {
    name: String,
    required: bool,
    variadic: bool,
}

impl CommandArgPattern {
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn required(&self) -> bool {
        self.required
    }

    #[must_use]
    pub fn variadic(&self) -> bool {
        self.variadic
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandPattern {
    command: String,
    command_parts: Vec<String>,
    args: Vec<CommandArgPattern>,
}

impl CommandPattern {
    pub fn parse(input: impl AsRef<str>) -> Result<Self, CommandPatternError> {
        let trimmed = input.as_ref().trim();
        if trimmed.is_empty() {
            return Err(CommandPatternError::EmptyPattern);
        }

        let mut command_parts = Vec::new();
        let mut args = Vec::new();
        let mut optional_seen = false;
        let mut variadic_seen = false;

        for token in trimmed.split_whitespace() {
            if variadic_seen {
                return Err(CommandPatternError::VariadicMustBeLast {
                    token: token.to_owned(),
                });
            }

            if let Some(arg) = parse_arg_token(token)? {
                if optional_seen && arg.required {
                    return Err(CommandPatternError::RequiredAfterOptional {
                        token: token.to_owned(),
                    });
                }

                optional_seen |= !arg.required;
                variadic_seen = arg.variadic;
                args.push(arg);
            } else {
                if !args.is_empty() {
                    return Err(CommandPatternError::CommandTokenAfterArgs {
                        token: token.to_owned(),
                    });
                }
                command_parts.push(token.to_ascii_lowercase());
            }
        }

        if command_parts.is_empty() {
            return Err(CommandPatternError::MissingCommandName);
        }

        Ok(Self {
            command: command_parts.join(" "),
            command_parts,
            args,
        })
    }

    #[must_use]
    pub fn command(&self) -> &str {
        &self.command
    }

    #[must_use]
    pub fn command_parts(&self) -> &[String] {
        &self.command_parts
    }

    #[must_use]
    pub fn args(&self) -> &[CommandArgPattern] {
        &self.args
    }

    #[must_use]
    pub fn min_args(&self) -> usize {
        self.args.iter().filter(|arg| arg.required).count()
    }

    pub fn max_args(&self) -> Option<usize> {
        if self.args.iter().any(CommandArgPattern::variadic) {
            None
        } else {
            Some(self.args.len())
        }
    }

    #[must_use]
    pub fn accepts(&self, args: &[String]) -> bool {
        if args.len() < self.min_args() {
            return false;
        }
        if let Some(max_args) = self.max_args() {
            args.len() <= max_args
        } else {
            true
        }
    }

    #[must_use]
    pub fn usage_with_prefix(&self, prefix: char) -> String {
        let mut usage = self.usage();
        usage.insert(0, prefix);
        usage
    }

    #[must_use]
    pub fn usage(&self) -> String {
        let mut usage = self.command.clone();
        for arg in &self.args {
            usage.push(' ');
            if arg.required {
                usage.push('<');
            } else {
                usage.push('[');
            }
            usage.push_str(&arg.name);
            if arg.variadic {
                usage.push_str("...");
            }
            if arg.required {
                usage.push('>');
            } else {
                usage.push(']');
            }
        }
        usage
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandPatternError {
    EmptyPattern,
    MissingCommandName,
    InvalidArgumentToken { token: String },
    RequiredAfterOptional { token: String },
    VariadicMustBeLast { token: String },
    CommandTokenAfterArgs { token: String },
}

impl std::fmt::Display for CommandPatternError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyPattern => f.write_str("command pattern is empty"),
            Self::MissingCommandName => f.write_str("command pattern is missing command name"),
            Self::InvalidArgumentToken { token } => {
                write!(f, "invalid command argument token: {token}")
            }
            Self::RequiredAfterOptional { token } => {
                write!(
                    f,
                    "required argument appears after optional argument: {token}"
                )
            }
            Self::VariadicMustBeLast { token } => {
                write!(
                    f,
                    "variadic argument must be the final token, found: {token}"
                )
            }
            Self::CommandTokenAfterArgs { token } => {
                write!(f, "command token appears after arguments: {token}")
            }
        }
    }
}

impl std::error::Error for CommandPatternError {}

fn parse_arg_token(token: &str) -> Result<Option<CommandArgPattern>, CommandPatternError> {
    let (required, inner) =
        if let Some(inner) = token.strip_prefix('<').and_then(|s| s.strip_suffix('>')) {
            (true, inner)
        } else if let Some(inner) = token.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            (false, inner)
        } else {
            return Ok(None);
        };

    if inner.is_empty() {
        return Err(CommandPatternError::InvalidArgumentToken {
            token: token.to_owned(),
        });
    }

    let (name, variadic) = if let Some(name) = inner.strip_suffix("...") {
        (name, true)
    } else {
        (inner, false)
    };

    if name.is_empty()
        || !name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
    {
        return Err(CommandPatternError::InvalidArgumentToken {
            token: token.to_owned(),
        });
    }

    Ok(Some(CommandArgPattern {
        name: name.to_owned(),
        required,
        variadic,
    }))
}

pub fn parse_command(text: &str, prefixes: &[char]) -> Option<Command> {
    let trimmed = text.trim();
    let mut chars = trimmed.chars();
    let prefix = chars.next()?;
    if !prefixes.contains(&prefix) {
        return None;
    }

    let body = chars.as_str().trim();
    if body.is_empty() {
        return None;
    }

    let mut parts = body.split_whitespace();
    let name = parts.next()?.to_ascii_lowercase();
    if name.is_empty() {
        return None;
    }

    let args = parts.map(ToOwned::to_owned).collect();
    Some(Command {
        prefix,
        name,
        args,
        raw: body.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::{CommandPattern, CommandPatternError};

    #[test]
    fn command_pattern_parses_command_and_args() {
        let pattern = CommandPattern::parse("ban <user> [reason...]").expect("pattern");
        assert_eq!(pattern.command(), "ban");
        assert_eq!(pattern.min_args(), 1);
        assert_eq!(pattern.max_args(), None);
        assert_eq!(pattern.usage_with_prefix('/'), "/ban <user> [reason...]");
    }

    #[test]
    fn command_pattern_rejects_required_after_optional() {
        let err = CommandPattern::parse("ban [reason] <user>").expect_err("must fail");
        assert!(matches!(
            err,
            CommandPatternError::RequiredAfterOptional { .. }
        ));
    }

    #[test]
    fn command_pattern_rejects_command_after_args() {
        let err = CommandPattern::parse("ban <user> now").expect_err("must fail");
        assert!(matches!(
            err,
            CommandPatternError::CommandTokenAfterArgs { .. }
        ));
    }

    #[test]
    fn command_pattern_accepts_argument_bounds() {
        let pattern = CommandPattern::parse("ban <user> [reason]").expect("pattern");
        assert!(!pattern.accepts(&[]));
        assert!(pattern.accepts(&[String::from("alice")]));
        assert!(pattern.accepts(&[String::from("alice"), String::from("spam")]));
        assert!(!pattern.accepts(&[
            String::from("alice"),
            String::from("spam"),
            String::from("extra"),
        ]));
    }
}
