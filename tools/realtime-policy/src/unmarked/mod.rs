//! Every non-test `try_pop` is in a marked region or in a function on the control-side allowlist
//! (#1443 D1 to D3).
//!
//! The drain rule ([`crate::drain`]) judges the pops that stand between a region's markers. This
//! pass covers the rest, so that a pop in a helper outside every region is never silently
//! skipped:
//!
//! - **Test code (D1)** is ignored: a file under `<package>/tests/`, a node that carries `#[test]`
//!   or a `#[cfg(P)]` whose `P` is false when `test` is false and every other atom, every
//!   `feature = ".."` included, is unknown, and a file that only test code declares ([`modules`], [`cfg`]).
//!   No module is exempt by its name, and `#[cfg(any(test, feature = "test-support"))]` is not
//!   test code: a test-support build ships it.
//! - **Every other pop (D2)** must be in a marked region (where the drain rule judges it) or in
//!   the own body of a function that a [`ControlSideEntry`] admits (D3), with exactly the entry's
//!   count of sites. A pop in a closure or an `async` block of a listed function is not admitted.
//! - **The census (D2)** classifies every `try_pop` identifier that the lexer finds: a pop, a
//!   `fn try_pop` definition, a token of test code, or nothing. A token that is nothing, such as a
//!   `try_pop` in an attribute's tokens or a use of the name as a field, is a finding.
//! - **A stale entry (D3)** is a finding: its file is gone, its path matches no function or more
//!   than one, or its site count differs from the function's.

mod cfg;
mod modules;
mod walk;

use std::collections::{BTreeSet, HashSet};
use std::path::Path;

use crate::drain::Pos;
use crate::markers::Region;
use crate::policy::{ControlSideEntry, Policy};
use crate::report::{Class, Finding};
use crate::source::Source;
use modules::Modules;
use walk::identifiers;

/// A parsed file with its regions.
pub(crate) struct Unit<'a> {
    pub(crate) source: &'a Source,
    pub(crate) file: &'a syn::File,
    pub(crate) regions: &'a [Region],
}

/// Whether the file holds a `try_pop` identifier token.
pub(crate) fn holds_pop(source: &Source) -> bool {
    let mut tokens = Vec::new();
    identifiers(&source.tokens, &mut tokens);
    !tokens.is_empty()
}

/// Whether the file declares an out-of-line module (`mod name;`).
pub(crate) fn declares_module(source: &Source) -> bool {
    walk::declares_module(&source.tokens)
}

/// Judge every non-test pop of `units` that stands outside every region. `files` holds every
/// `.rs` path of the walk.
pub(crate) fn check(
    root: &Path,
    files: &BTreeSet<String>,
    units: &[Unit<'_>],
    policy: &Policy,
    findings: &mut Vec<Finding>,
) {
    let modules = Modules::new(
        root,
        files,
        units
            .iter()
            .map(|unit| (unit.source.path.as_str(), unit.file)),
    );
    let mut judged: HashSet<usize> = HashSet::new();
    for unit in units {
        let mut tokens = Vec::new();
        identifiers(&unit.source.tokens, &mut tokens);
        if modules.is_test(&unit.source.path) {
            // A file of test code accounts for every token in it, and no entry can name a
            // function of it.
            for (index, entry) in entries_of(policy, &unit.source.path) {
                judged.insert(index);
                findings.push(stale(entry, None));
            }
            continue;
        }
        if tokens.is_empty() && entries_of(policy, &unit.source.path).next().is_none() {
            continue;
        }
        judge(unit, &tokens, policy, &mut judged, findings);
    }
    for (index, entry) in policy.control_side_allowlist.iter().enumerate() {
        if !judged.contains(&index) {
            findings.push(stale(entry, None));
        }
    }
}

fn entries_of<'p>(
    policy: &'p Policy,
    path: &'p str,
) -> impl Iterator<Item = (usize, &'p ControlSideEntry)> {
    policy
        .control_side_allowlist
        .iter()
        .enumerate()
        .filter(move |(_, entry)| entry.path == path)
}

fn stale(entry: &ControlSideEntry, line: Option<(&Source, usize)>) -> Finding {
    let finding = match line {
        Some((source, line)) => Finding::at_line(
            Class::StaleAllowlistEntry,
            &source.path,
            line,
            source.line_text(line),
        ),
        None => Finding::at_path(Class::StaleAllowlistEntry, &entry.path),
    };
    finding.about_entry(&entry.function)
}

/// Judge one file that is not test code and holds a `try_pop` token.
fn judge(
    unit: &Unit<'_>,
    tokens: &[Pos],
    policy: &Policy,
    judged: &mut HashSet<usize>,
    findings: &mut Vec<Finding>,
) {
    let source = unit.source;
    let walk = walk::walk(unit.file);
    let in_region = |line: usize| {
        unit.regions
            .iter()
            .any(|region| region.begin < line && line < region.end)
    };
    // The pops that stand outside every region, per function.
    let outside = |pop: &&walk::Pop| !in_region(pop.site.pos.0);
    // The functions that an entry accounts for, whatever its count.
    let mut accounted: HashSet<usize> = HashSet::new();
    let mut seen: HashSet<&str> = HashSet::new();
    for (index, entry) in entries_of(policy, &source.path) {
        judged.insert(index);
        let matches: Vec<usize> = walk
            .functions
            .iter()
            .enumerate()
            .filter(|(_, function)| function.path == entry.function)
            .map(|(at, _)| at)
            .collect();
        let [function] = matches[..] else {
            findings.push(stale(entry, None));
            continue;
        };
        let line = walk.functions[function].line;
        let count = walk
            .pops
            .iter()
            .filter(outside)
            .filter(|pop| pop.function == Some(function) && !pop.deferred)
            .count();
        if !seen.insert(entry.function.as_str()) || count != entry.sites {
            findings.push(stale(entry, Some((source, line))));
        }
        accounted.insert(function);
    }
    let mut unmarked = BTreeSet::new();
    for pop in walk.pops.iter().filter(outside) {
        let admitted = pop
            .function
            .is_some_and(|function| accounted.contains(&function))
            && !pop.deferred;
        if !admitted {
            unmarked.insert(pop.site.pos.0);
        }
    }
    findings.extend(unmarked.into_iter().map(|line| {
        Finding::at_line(
            Class::UnmarkedTryPop,
            &source.path,
            line,
            source.line_text(line),
        )
    }));
    // The census: every `try_pop` identifier is a pop, a definition or test code.
    let pops: BTreeSet<Pos> = walk.pops.iter().map(|pop| pop.site.pos).collect();
    let mut lines = BTreeSet::new();
    for &token in tokens {
        let in_test = walk
            .test_ranges
            .iter()
            .any(|&(start, end)| start <= token && token < end);
        if !(in_test || pops.contains(&token) || walk.definitions.contains(&token)) {
            lines.insert(token.0);
        }
    }
    findings.extend(lines.into_iter().map(|line| {
        Finding::at_line(
            Class::UnclassifiedTryPop,
            &source.path,
            line,
            source.line_text(line),
        )
    }));
}
