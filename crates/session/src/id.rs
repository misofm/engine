//! Stable session identifiers.

use core::fmt;

/// The longest a stable ID is, in bytes: `[a-z][a-z0-9._-]{0,126}`. A host sizes any staging that
/// must hold an ID of a session it has not yet seen (a rebuild can add one) by this.
pub const STABLE_ID_MAXIMUM_BYTES: usize = 127;

/// A schema-validated, lowercase stable session identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StableId(String);

impl StableId {
    /// Validate `[a-z][a-z0-9._-]{0,126}`.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        let bytes = value.as_bytes();
        if !(1..=STABLE_ID_MAXIMUM_BYTES).contains(&bytes.len()) || !bytes[0].is_ascii_lowercase() {
            return None;
        }
        if bytes[1..].iter().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
        }) {
            Some(Self(value.to_owned()))
        } else {
            None
        }
    }

    /// Borrow the canonical ID text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for StableId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}
