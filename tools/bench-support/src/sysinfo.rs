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
        .map(|output| output.trim().to_owned())
        .filter(|output| !output.is_empty())
    else {
        return "unknown".to_owned();
    };
    let cores = output
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
    use super::physical_core_count;

    #[test]
    fn returns_a_decimal_count_or_the_unknown_sentinel() {
        let count = physical_core_count();
        assert!(count == "unknown" || count.parse::<usize>().is_ok());
    }
}
