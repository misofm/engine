//! The verdict probe corpus of #1302 and #1418 (B2a-D5): every compiled probe that `main`'s awk
//! gate refused, and every bounded probe it passed, plus the six probes that passed it and pop
//! until the queue is empty (the closure and macro limits B2a closes).
//!
//! A probe is a file `tests/fixtures/probes/<group>/<name>.<verdict>.rs.txt`. The case replaces
//! the base tree's `fn drain_matrix_controls() {}`, inside its region, with the probe's text, whose
//! own marker lines were removed. The verdict in the file name is the set of finding classes the
//! case expects, fixed by the issue and not read off the tool: `drain` is the drain class alone,
//! `accepted` is none, `forbidden-body+drain` and `unsafe+drain` are the two probes that another
//! class also refuses.

use std::fs;
use std::path::{Path, PathBuf};

use realtime_policy::Class;

use super::super::common::{Edit, expect_failure, expect_valid};
use super::{ANCHOR, BUILTINS_COMPILER, DRAIN, each_row};

/// One probe file: the case name (group and probe), its text and its expected classes.
struct Probe {
    name: &'static str,
    text: String,
    expected: Vec<Class>,
}

fn probes() -> Vec<Probe> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/probes");
    let mut paths: Vec<PathBuf> = Vec::new();
    for group in fs::read_dir(&root).expect("list the probe groups") {
        let group = group.expect("read a group entry").path();
        for file in fs::read_dir(&group).expect("list a probe group") {
            paths.push(file.expect("read a probe entry").path());
        }
    }
    paths.sort();
    paths
        .into_iter()
        .map(|path| {
            let name = path
                .strip_prefix(&root)
                .expect("under the probes")
                .to_string_lossy()
                .into_owned();
            let stem = name
                .strip_suffix(".rs.txt")
                .unwrap_or_else(|| panic!("probe {name} ends in .rs.txt"));
            let (_, verdict) = stem
                .rsplit_once('.')
                .unwrap_or_else(|| panic!("probe {name} names its verdict"));
            let expected = match verdict {
                "accepted" => vec![],
                "drain" => vec![DRAIN],
                "forbidden-body+drain" => vec![Class::ForbiddenBody, DRAIN],
                "unsafe+drain" => vec![Class::UnsafeOutsideOwnership, DRAIN],
                other => panic!("probe {name}: unknown verdict {other}"),
            };
            let text = fs::read_to_string(&path).expect("read a probe");
            Probe {
                name: Box::leak(name.into_boxed_str()),
                text: text.strip_suffix('\n').unwrap_or(&text).to_owned(),
                expected,
            }
        })
        .collect()
}

fn edit(text: &str) -> [Edit<'_>; 1] {
    [Edit::ReplaceLine {
        path: BUILTINS_COMPILER,
        old: ANCHOR,
        new: text,
    }]
}

/// Every refused probe fails with exactly its classes. Each is a drain that pops until the queue is
/// empty, or that hides its bound, in a position where a rule that accepts too much passes it:
/// a literal form, a let chain, an or-pattern, a struct pattern; a labelled, `const` or `unsafe`
/// block, a closure in a header; a `cfg` attribute; a closure or a macro. (A walk that merely
/// skips a node leaves its pop unreached, and an unreached pop is refused, so the bounded probes
/// below are what catch a skipped node.)
#[test]
fn the_refused_probes_are_refused() {
    let rows: Vec<_> = probes()
        .into_iter()
        .filter(|probe| !probe.expected.is_empty())
        .map(|probe| (probe.name, probe))
        .collect();
    assert!(!rows.is_empty(), "the refused probes are in the corpus");
    each_row(rows, |name, probe| {
        expect_failure(name, &edit(&probe.text), &probe.expected);
    });
}

/// Every bounded probe passes. Each is a form `main` accepts: an early return, a capped count, a
/// typed binding, `while >`, a `zip` or slice-parameter outer loop, an indexed receiver, an
/// attributed loop, a `debug_assert!` that names the count. A port that refuses one over-refuses.
#[test]
fn the_bounded_probes_pass() {
    let rows: Vec<_> = probes()
        .into_iter()
        .filter(|probe| probe.expected.is_empty())
        .map(|probe| (probe.name, probe))
        .collect();
    assert!(!rows.is_empty(), "the bounded probes are in the corpus");
    each_row(rows, |name, probe| {
        expect_valid(name, &edit(&probe.text));
    });
}
