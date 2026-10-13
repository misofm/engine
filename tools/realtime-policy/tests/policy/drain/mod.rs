//! The drain-bound rule (B1a): how a pop maps to its region, the scope resolver, the visited
//! syntax, pops, count bindings, reads before the loop and the innermost bounding loop.

mod declarations;
mod forms;
mod indirect;
mod outer;
mod per_queue;
mod ported;
mod probes;
mod trusted;

use realtime_policy::Class;

use super::common::{
    BEGIN, END, Edit, FILE_FLOOR, REGION_FLOOR, Tree, expect_failure, expect_valid,
};

const BUILTINS_COMPILER: &str = "crates/builtins-compiler/src/lib.rs";
/// The second region's only line, which the cases replace.
const ANCHOR: &str = "fn drain_matrix_controls() {}";
const DRAIN: Class = Class::UnboundedTryPop;

/// One ported case: the base tree with one edit.
pub(super) struct Ported {
    pub(super) name: &'static str,
    /// `edit` replaces the line `anchor` in `path` with `text`; `append` appends `text`.
    pub(super) kind: &'static str,
    pub(super) path: &'static str,
    pub(super) anchor: &'static str,
    pub(super) text: &'static str,
}

impl Ported {
    fn tree(&self) -> Tree {
        let tree = Tree::base(self.name);
        let text = self.text.replace("@BEGIN@", BEGIN).replace("@END@", END);
        let edit = if self.kind == "append" {
            Edit::Append {
                path: self.path,
                text: &text,
            }
        } else {
            Edit::ReplaceLine {
                path: self.path,
                old: self.anchor,
                new: &text,
            }
        };
        tree.apply(&[edit]);
        tree
    }
}

/// Run `check` on every row, then fail once naming every row that failed, so a broken rule shows
/// every case it breaks and not only the first.
pub(super) fn each_row<T>(
    rows: impl IntoIterator<Item = (&'static str, T)>,
    check: impl Fn(&'static str, T),
) {
    let mut failed = Vec::new();
    for (name, row) in rows {
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| check(name, row)));
        if outcome.is_err() {
            failed.push(name);
        }
    }
    assert!(failed.is_empty(), "rows failed: {failed:?}");
}

/// Assert that `text` in the second region is a drain the rule refuses, and nothing else.
fn refuses(name: &str, text: &str) {
    expect_failure(
        name,
        &[Edit::ReplaceLine {
            path: BUILTINS_COMPILER,
            old: ANCHOR,
            new: text,
        }],
        &[DRAIN],
    );
}

/// Assert that `text` in the second region passes.
fn accepts(name: &str, text: &str) {
    expect_valid(
        name,
        &[Edit::ReplaceLine {
            path: BUILTINS_COMPILER,
            old: ANCHOR,
            new: text,
        }],
    );
}

/// Every ported awk drain case that this tool refuses fails with the drain class, except the two
/// that never reach a drain: out-of-order markers leave their file without regions (the awk's
/// open-tail allowance is gone), and a region that ends inside a comment has a malformed marker.
/// The cases hold the shapes of the awk's lexer era (wrapped headers, braces in literals, raw
/// strings) as a regression corpus, and the count, loop and pop shapes the rule decides.
#[test]
fn the_ported_drain_cases_are_refused() {
    each_row(
        ported::PORTED.iter().map(|case| (case.name, case)),
        |_, case| {
            let tree = case.tree();
            let expected: &[Class] = match case.name {
                "open-tail-last-file" => &[
                    Class::MisorderedMarkers,
                    Class::UnmarkedTryPop,
                    FILE_FLOOR,
                    REGION_FLOOR,
                ],
                "region-ends-in-comment" => &[Class::MalformedMarker],
                _ => &[DRAIN],
            };
            super::common::assert_classes(&tree, expected);
        },
    );
}
