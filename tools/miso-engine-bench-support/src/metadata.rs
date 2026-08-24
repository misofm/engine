//! One in-process snapshot of the benchmark runner's environment metadata.

use std::collections::BTreeMap;
use std::env::VarError;
use std::ffi::{OsStr, OsString};
use std::sync::OnceLock;

/// The immutable environment seen by a benchmark or audit process.
///
/// Subjects deliberately keep their historical field names, required/optional rules, and record
/// projections. This type only removes repeated calls into the process environment and guarantees
/// that every field in one run comes from the same snapshot.
pub struct Metadata {
    values: BTreeMap<OsString, OsString>,
}

impl Metadata {
    /// Gather the process environment once, on the control plane.
    #[must_use]
    pub fn gather() -> &'static Self {
        static SNAPSHOT: OnceLock<Metadata> = OnceLock::new();
        SNAPSHOT.get_or_init(|| Metadata {
            values: std::env::vars_os().collect(),
        })
    }

    /// Read one value from the frozen snapshot with `std::env::var`-compatible errors.
    pub fn var(&self, name: &str) -> Result<String, VarError> {
        self.values
            .get(OsStr::new(name))
            .cloned()
            .ok_or(VarError::NotPresent)?
            .into_string()
            .map_err(VarError::NotUnicode)
    }
}

#[cfg(test)]
mod tests {
    use super::Metadata;

    #[test]
    fn gather_is_memoized_and_matches_the_process_environment() {
        let first = Metadata::gather();
        let second = Metadata::gather();
        assert!(std::ptr::eq(first, second));
        if let Ok(path) = first.var("PATH") {
            assert!(!path.is_empty());
        }
    }
}
