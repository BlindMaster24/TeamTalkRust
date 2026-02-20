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
