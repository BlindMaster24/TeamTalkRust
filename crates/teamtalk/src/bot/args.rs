use crate::events::{Error, Result};
use std::str::FromStr;

/// Typed helpers for command arguments.
#[derive(Debug, Clone, Copy)]
pub struct Args<'a> {
    args: &'a [String],
}

impl<'a> Args<'a> {
    #[allow(clippy::must_use_candidate)]
    pub fn new(args: &'a [String]) -> Self {
        Self { args }
    }

    /// Returns the raw argument at `index`.
    pub fn raw(&self, index: usize) -> Option<&'a str> {
        self.args.get(index).map(String::as_str)
    }

    /// Parses the argument at `index` into `T`.
    pub fn get<T>(&self, index: usize) -> Result<Option<T>>
    where
        T: FromStr,
    {
        match self.raw(index) {
            Some(raw) => raw.parse::<T>().map(Some).map_err(|_| Error::InvalidParam),
            None => Ok(None),
        }
    }

    /// Parses the argument at `index` and returns an error when it's missing.
    pub fn require<T>(&self, index: usize, usage: &str) -> Result<T>
    where
        T: FromStr,
    {
        let Some(raw) = self.raw(index) else {
            return Err(Error::IoError {
                message: format!("missing argument at index {index}. usage: {usage}"),
            });
        };
        raw.parse::<T>().map_err(|_| Error::InvalidParam)
    }

    /// Returns arguments from `index` to the end as one space-separated string.
    #[allow(clippy::must_use_candidate)]
    pub fn rest(&self, index: usize) -> Option<String> {
        if index >= self.args.len() {
            return None;
        }
        Some(self.args[index..].join(" "))
    }

    /// Returns all raw arguments.
    #[must_use]
    pub fn all(&self) -> &'a [String] {
        self.args
    }
}
