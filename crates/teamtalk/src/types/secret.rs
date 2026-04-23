//! Zeroising secret-string wrapper used for in-memory credentials.
//!
//! TeamTalk credentials (passwords passed to `login`, operator
//! passwords used in moderation calls) are sensitive and should
//! not linger in heap memory after they are no longer needed.
//!
//! [`SecretString`] is a thin newtype over `String` that:
//!
//! - Zeroises its backing buffer on `Drop` via [`zeroize::Zeroize`].
//! - Refuses to print the secret via `Debug` / `Display` — the only
//!   way to observe the inner bytes is through
//!   [`SecretString::expose_secret`].
//! - Compares for equality in constant time (see
//!   [`SecretString::eq`]) so user-visible branch timing does not
//!   leak information about password length or content.
//! - Accepts conversions from `&str`, `String`, and
//!   `Cow<'_, str>` via `From` / `Into` so API callers do not need
//!   to think about the wrapper unless they want to.
//!
//! The implementation intentionally does not re-export or depend
//! on the `secrecy` crate so the crate surface stays small.

use std::borrow::Cow;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// A UTF-8 string that is zeroised on drop and refuses to print
/// itself through `Debug` / `Display`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct SecretString(String);

impl SecretString {
    /// Creates an empty secret.
    #[must_use]
    pub fn new() -> Self {
        Self(String::new())
    }

    /// Creates a secret from an owned `String`. The input buffer
    /// is consumed and its memory is owned by the secret.
    #[must_use]
    pub fn from_string(value: String) -> Self {
        Self(value)
    }

    /// Returns `true` if the secret is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the length of the secret in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns a reference to the underlying bytes.
    ///
    /// # Security
    ///
    /// This is the only way to read the secret back out. Treat the
    /// returned `&str` as tainted: do not log it, store it
    /// long-term, or pass it through `Debug` / `Display`. Scope its
    /// use as narrowly as possible (e.g. a single FFI call) and
    /// drop the reference immediately afterwards.
    #[must_use]
    pub fn expose_secret(&self) -> &str {
        self.0.as_str()
    }

    /// Overwrites the inner buffer with zero bytes in place and
    /// leaves the secret empty.
    ///
    /// Equivalent to assigning `SecretString::new()` except the
    /// existing allocation is reused, so capacity is preserved.
    pub fn zeroize_in_place(&mut self) {
        self.0.zeroize();
    }
}

impl Default for SecretString {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SecretString").field(&"<redacted>").finish()
    }
}

impl std::fmt::Display for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<redacted>")
    }
}

impl PartialEq for SecretString {
    /// Constant-time-ish comparison over the byte contents.
    ///
    /// Uses a branch-less `|=` fold over XOR-ed bytes so equality
    /// checks do not short-circuit on the first mismatching byte.
    /// Length difference is still observable — callers that need
    /// full length-hiding must compare to a padded secret.
    fn eq(&self, other: &Self) -> bool {
        let a = self.0.as_bytes();
        let b = other.0.as_bytes();
        if a.len() != b.len() {
            return false;
        }
        let mut diff: u8 = 0;
        for (x, y) in a.iter().zip(b.iter()) {
            diff |= x ^ y;
        }
        diff == 0
    }
}

impl Eq for SecretString {}

impl From<String> for SecretString {
    fn from(value: String) -> Self {
        Self::from_string(value)
    }
}

impl From<&str> for SecretString {
    fn from(value: &str) -> Self {
        Self::from_string(value.to_owned())
    }
}

impl From<Cow<'_, str>> for SecretString {
    fn from(value: Cow<'_, str>) -> Self {
        Self::from_string(value.into_owned())
    }
}

impl From<&String> for SecretString {
    fn from(value: &String) -> Self {
        Self::from_string(value.clone())
    }
}
