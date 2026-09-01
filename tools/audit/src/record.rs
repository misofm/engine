//! The one JSONL record vocabulary the audit subjects share.
//!
//! Audit #104 F4's rule is that the second copy is the defect, and F2 is what a second copy cost:
//! a runner and its binary drifted until every accepted record carried all-null environment
//! metadata and still validated. `bench-support` owns the escaper, the percentile, the
//! allocator, the digest sink and the clock; what it does not own is the two-line array formatting
//! and the metadata lookup that every subject's record needs, and those had started to accumulate
//! copies. Issue #146 needed a third one, so they moved here instead.

use bench_support::json::escape as json_escape;
use bench_support::metadata::Metadata;

/// One runner-supplied environment fact, JSON-escaped, or `"unknown"` when it was not supplied.
pub(crate) fn metadata(name: &str) -> String {
    Metadata::gather()
        .var(name)
        .ok()
        .filter(|value| !value.is_empty())
        .map(|value| json_escape(&value))
        .unwrap_or_else(|| "unknown".to_owned())
}

/// A JSON array of integers.
pub(crate) fn json_integer_array(values: impl Iterator<Item = u128>) -> String {
    format!(
        "[{}]",
        values
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}

/// A JSON array of reals, fixed at six decimal places so a record is byte-stable to read.
pub(crate) fn json_f64_array(values: impl Iterator<Item = f64>) -> String {
    format!(
        "[{}]",
        values
            .map(|value| format!("{value:.6}"))
            .collect::<Vec<_>>()
            .join(",")
    )
}
