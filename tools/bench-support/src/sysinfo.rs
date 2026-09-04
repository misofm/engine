//! The one physical-core-count probe.
//!
//! Audit #104 F4 found this same `lscpu -p=CORE,SOCKET` probe duplicated between the session and
//! conformance benchmarks. The two copies were *not* byte-identical -- `session.rs` dispatched
//! through a local `command()` that returns the `"unknown"` sentinel on any failure, while
//! `conformance.rs` dispatched through a local `command_allow_empty()` that returns `Option<String>`
//! -- but the parsing downstream of that dispatch (trim, split into `(core, socket)` pairs, dedupe,
//! count) was identical in both. This merges the two: one dispatch, written out here rather than
//! reused from either file's now-deleted helper, and the identical parsing tail.
//!
//! One definition, so a change to how a physical core is counted is a change to one function, not
//! two.

use std::collections::BTreeSet;
use std::process::Command;

/// The number of distinct physical cores this host reports, or `"unknown"` if the count cannot be
/// determined (no `lscpu`, no permission, or unparsable output).
#[must_use]
pub fn physical_core_count() -> String {
    let Some(output) = Command::new("lscpu")
        .arg("-p=CORE,SOCKET")
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
    else {
        return "unknown".to_owned();
    };
    count_cores(&output)
}

/// The parsing tail, split out so the restored `.trim()`/empty-output behaviour has a unit test
/// instead of only a doc comment. `lscpu_stdout` is the raw (possibly untrimmed, possibly empty)
/// standard output of `lscpu -p=CORE,SOCKET`; every non-comment line is `core,socket`, and the
/// count is the number of distinct pairs.
#[must_use]
fn count_cores(lscpu_stdout: &str) -> String {
    let trimmed = lscpu_stdout.trim();
    if trimmed.is_empty() {
        return "unknown".to_owned();
    }
    let cores = trimmed
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| {
            let mut fields = line.split(',');
            Some((fields.next()?.to_owned(), fields.next()?.to_owned()))
        })
        .collect::<BTreeSet<_>>();
    if cores.is_empty() {
        "unknown".to_owned()
    } else {
        cores.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{count_cores, physical_core_count};

    #[test]
    fn returns_a_decimal_count_or_the_unknown_sentinel() {
        let count = physical_core_count();
        assert!(count == "unknown" || count.parse::<usize>().is_ok());
    }

    #[test]
    fn empty_output_is_unknown() {
        assert_eq!(count_cores(""), "unknown");
    }

    #[test]
    fn whitespace_only_output_is_unknown() {
        assert_eq!(count_cores("  \n"), "unknown");
    }

    #[test]
    fn header_only_output_is_unknown() {
        assert_eq!(count_cores("# header only\n"), "unknown");
    }

    #[test]
    fn distinct_core_socket_pairs_are_counted_once_each() {
        assert_eq!(count_cores("0,0\n1,0\n0,0\n"), "2");
    }
}
