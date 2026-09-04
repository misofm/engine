//! The one physical-core-count probe.
//!
//! Audit #104 F4 found this same `lscpu -p=CORE,SOCKET` probe, byte-identical, duplicated between
//! the session and conformance benchmarks. One definition, so a change to how a physical core is
//! counted is a change to one function, not two.

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
