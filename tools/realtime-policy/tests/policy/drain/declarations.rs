//! Macros and attributes that can declare an item between a binding and its loop (B1b-D10).
//!
//! A `macro_rules!` item name is not hygienic, and a derive or an attribute macro emits items
//! beside its item: either can declare a `const`, a `static` or a unit struct that shadows the
//! count, its alias source or the slice parameter for the whole block it lands in. D2's item scan
//! reads the tokens of the region and cannot see an expansion. The cases below put the macro
//! outside the region, as the production shape would, and a pop in a loop whose bound the rule can
//! prove from the syntax alone. A `let` is shadowed from inside its own block only through a
//! deeper block, so its span ends with the statement that holds the pop; a parameter's block is
//! the whole `fn` body, so its span is the body.

use realtime_policy::Finding;

use super::super::common::{Edit, Tree, assert_classes, expect_failure, expect_valid, region};
use super::{DRAIN, each_row};

const BUFFER: &str = "crates/engine/src/realtime/buffer.rs";
const POP: &str = "let Ok(record) = control.try_pop() else {\n                break;\n            };\n            apply(record);";
/// A macro that declares a `const` named like the count, and one named like a slice parameter.
const SHADOWS_COUNT: &str =
    "macro_rules! shadow {\n    () => {\n        const available: usize = usize::MAX;\n    };\n}\n";
const SHADOWS_SLICE: &str =
    "macro_rules! shadow {\n    () => {\n        const lanes: [u8; 1] = [0];\n    };\n}\n";
/// A macro that declares a unit struct named like a collection root, with the `iter_mut` and the
/// `iter` that repeat (D9).
const SHADOWS_ROOT: &str = "macro_rules! shadow {\n    () => {\n        struct controls;\n        impl controls {\n            fn iter_mut(&self) -> core::iter::Repeat<u8> {\n                core::iter::repeat(0)\n            }\n        }\n    };\n}\n";
const SHADOWS_LANES: &str = "macro_rules! shadow {\n    () => {\n        struct lanes;\n        impl lanes {\n            fn iter(&self) -> core::iter::Repeat<u8> {\n                core::iter::repeat(0)\n            }\n        }\n    };\n}\n";
/// A macro that declares a struct named like the queue root `queues` of a per-queue form, and one
/// named like its counter root `counts` (D9).
const SHADOWS_QUEUES: &str =
    "macro_rules! shadow {\n    () => {\n        struct queues;\n    };\n}\n";
const SHADOWS_COUNTS: &str =
    "macro_rules! shadow {\n    () => {\n        struct counts;\n    };\n}\n";
/// A macro that declares an unbounded range named like the slice parameter `rounds`.
const SHADOWS_ROUNDS: &str = "macro_rules! shadow {\n    () => {\n        const rounds: core::ops::RangeFrom<u64> = 0..;\n    };\n}\n";

/// The base tree plus `outside` (items before the region) and a region around `body`.
fn appended(outside: &str, body: &str) -> String {
    format!("{outside}{}", region(body))
}

/// Assert that the text is refused, with one finding: its pop, in the buffer file.
fn refused(name: &str, text: &str) {
    let report = expect_failure(name, &[Edit::Append { path: BUFFER, text }], &[DRAIN]);
    let [finding] = report.findings.as_slice() else {
        panic!("case {name}: one finding expected: {:#?}", report.findings);
    };
    assert_eq!(finding.path.as_deref(), Some(BUFFER), "case {name}");
    assert!(
        finding
            .source
            .as_deref()
            .is_some_and(|line| line.contains("try_pop")),
        "case {name}: {finding:#?}"
    );
}

/// Assert that the text passes.
fn accepted(name: &str, text: &str) {
    expect_valid(name, &[Edit::Append { path: BUFFER, text }]);
}

/// A counted drain in the function `drain`, with `before` the statements between the count and the
/// loop, and `after` the statements after the loop in the same block as it.
fn counted(before: &str, after: &str) -> String {
    format!(
        "fn drain(control: &mut Consumer<Record>) {{\n    let available = control.available_at_entry();\n    {{\n{before}        for _ in 0..available {{\n            {POP}\n        }}\n{after}    }}\n}}"
    )
}

/// A drain whose only bound is a loop over the slice parameter `lanes`, with `before` first in the
/// function body and `after` last in it, at the body's own level. The count is
/// `available_at_entry()` written in the header, so no `let` binds it.
fn over_slice(before: &str, after: &str) -> String {
    format!(
        "fn drain(control: &mut Consumer<Record>, lanes: &[u8]) {{\n{before}    for _ in lanes {{\n        for _ in 0..control.available_at_entry() {{\n            {POP}\n        }}\n    }}\n{after}}}"
    )
}

/// A statement macro, in any delimiter, between the count and its loop, or after the loop in an
/// inner block, can declare an item that shadows the count: the drain is refused.
#[test]
fn a_statement_macro_between_the_count_and_the_end_of_its_block_is_refused() {
    let before = |statement: &str| {
        appended(
            SHADOWS_COUNT,
            &counted(&format!("        {statement}\n"), ""),
        )
    };
    let rows: Vec<(&'static str, String)> = vec![
        ("item-macro-shadows-the-count", before("shadow!();")),
        (
            "bracketed-item-macro-shadows-the-count",
            before("shadow![];"),
        ),
        ("braced-item-macro-shadows-the-count", before("shadow! {}")),
        (
            // The macro stands after the loop, in the inner block that holds it: items of a block
            // are visible in the whole block, so the expansion reaches the loop's header.
            "item-macro-after-the-loop-in-an-inner-block",
            appended(SHADOWS_COUNT, &counted("", "        shadow!();\n")),
        ),
    ];
    each_row(rows, |name, text| refused(name, &text));
}

/// The same hole through a slice parameter, whose block is the whole `fn` body (root ruling
/// 2026-10-11): a statement macro or a derive anywhere in the body, the places after the loop at
/// the body's level included, declares an item that shadows the parameter. Red if the span of a
/// parameter ends at the body statement that holds the pop, or does not start at the body, or if
/// a slice parameter is not noted as a read.
#[test]
fn a_statement_macro_or_a_derive_in_the_body_of_a_function_with_a_slice_loop_is_refused() {
    let rows: Vec<(&'static str, String)> = vec![
        (
            "braced-item-macro-shadows-the-slice-parameter",
            appended(SHADOWS_SLICE, &over_slice("    shadow! {}\n", "")),
        ),
        (
            // The attempt-2 verdict's b04: an unbounded `const lanes` after the loop.
            "item-macro-after-the-slice-loop-shadows-the-slice-parameter",
            appended(SHADOWS_SLICE, &over_slice("", "    shadow!();\n")),
        ),
        (
            // The attempt-2 verdict's b05: the derive emits `const lanes` beside its item.
            "derive-after-the-slice-loop-shadows-the-slice-parameter",
            appended(
                "",
                &over_slice("", "    #[derive(MakeLanes)]\n    struct Unrelated;\n"),
            ),
        ),
    ];
    each_row(rows, |name, text| refused(name, &text));
}

/// A per-queue form (D8b) whose queue root is the parameter `queues` and whose counter root is the
/// parameter `counts`, with `before` first in the body.
fn per_queue_form(before: &str) -> String {
    format!(
        "fn poll(queues: &mut Meters, counts: &mut Counts) {{\n{before}    for (lane, left) in queues.lanes.iter().zip(&mut counts.remaining) {{\n        *left = lane.consumer.available_at_entry();\n    }}\n    for (lane, left) in queues.lanes.iter_mut().zip(&mut counts.remaining) {{\n        if *left == 0 {{\n            continue;\n        }}\n        *left -= 1;\n        if let Ok(record) = lane.consumer.try_pop() {{\n            apply(record);\n        }}\n    }}\n}}"
    )
}

/// D9 + D10 (root Amendment 3, SP-2): a statement macro that can declare an item shadows the root
/// of an outer loop's collection, or a root of a per-queue form, as it shadows the count. The macro
/// stands outside the region, so only D10's span can see it: the attempt-2 verdict's `r02` (the
/// root is destructured by a `let`, and the macro runs before the loop) and `b01` (a slice
/// parameter iterated with `.iter()`, the macro after the loop), and the same macro against each
/// root of a per-queue form. Red if the collection root, or `<Q>` or `<K>`, is not noted as a read.
#[test]
fn a_statement_macro_that_can_declare_a_collection_root_refuses_the_drain() {
    let over_root = "fn drain(state: &mut State) {\n    let State { controls, .. } = state;\n    {\n        shadow!();\n        for control in controls.iter_mut() {\n            let available = control.available_at_entry();\n            for _ in 0..available {\n                POP\n            }\n        }\n    }\n}".replace("POP", POP);
    let over_lanes = "fn drain(control: &mut Consumer<Record>, lanes: &[u8]) {\n    for _ in lanes.iter() {\n        let available = control.available_at_entry();\n        for _ in 0..available {\n            POP\n        }\n    }\n    shadow!();\n}".replace("POP", POP);
    let rows: Vec<(&'static str, String)> = vec![
        (
            "macro-emitted-struct-shadows-the-collection-root",
            appended(SHADOWS_ROOT, &over_root),
        ),
        (
            "macro-emitted-struct-shadows-a-slice-parameter-collection-root",
            appended(SHADOWS_LANES, &over_lanes),
        ),
        (
            "macro-emitted-struct-shadows-the-queue-root",
            appended(SHADOWS_QUEUES, &per_queue_form("    shadow!();\n")),
        ),
        (
            "macro-emitted-struct-shadows-the-counter-root",
            appended(SHADOWS_COUNTS, &per_queue_form("    shadow!();\n")),
        ),
    ];
    each_row(rows, |name, text| refused(name, &text));
    // The same forms pass when the macro is not there.
    accepted(
        "collection-root-with-no-macro",
        &appended("", &over_root.replace("        shadow!();\n", "")),
    );
    accepted(
        "per-queue-roots-with-no-macro",
        &appended("", &per_queue_form("")),
    );
}

/// A derive or an attribute macro emits items beside its item, `cfg_attr` can expand to a derive,
/// and a path attribute may be any macro. `cfg`, lint levels and `doc` emit nothing.
#[test]
fn an_attribute_that_can_emit_an_item_refuses_the_drain() {
    let attributed = |attribute: &str| {
        appended(
            "",
            &counted(
                &format!("        {attribute}\n        struct Marker;\n"),
                "",
            ),
        )
    };
    let refuses_rows: Vec<(&'static str, String)> = vec![
        (
            "derive-on-an-item-in-the-drain",
            attributed("#[derive(Clone)]"),
        ),
        (
            "cfg-attr-on-an-item-in-the-drain",
            attributed("#[cfg_attr(all(), derive(Clone))]"),
        ),
        ("plain-attribute-on-an-item", attributed("#[inline]")),
        (
            "path-attribute-on-an-item",
            attributed("#[other::generate]"),
        ),
        (
            "derive-after-an-allowed-attribute",
            attributed("#[allow(dead_code)]\n        #[derive(Clone)]"),
        ),
    ];
    each_row(refuses_rows, |name, text| refused(name, &text));
    let accepts_rows: Vec<(&'static str, String)> = vec![
        ("cfg-on-an-item-in-the-drain", attributed("#[cfg(test)]")),
        ("allow-on-an-item", attributed("#[allow(dead_code)]")),
        ("expect-on-an-item", attributed("#[expect(dead_code)]")),
        ("warn-on-an-item", attributed("#[warn(dead_code)]")),
        ("deny-on-an-item", attributed("#[deny(dead_code)]")),
        ("forbid-on-an-item", attributed("#[forbid(dead_code)]")),
        ("doc-on-an-item", attributed("#[doc = \"A marker.\"]")),
        ("doc-comment-on-an-item", attributed("/// A marker.")),
    ];
    each_row(accepts_rows, |name, text| accepted(name, &text));
}

/// What is not a statement or item macro, or lies outside the span, is not refused: a macro in
/// expression position (also a trailing one with no `;`, which cannot expand to an item), a macro
/// before the count (an item there cannot shadow a `let` that follows in its block), a `fn`
/// signature's neighbour, and a trusted `debug_assert!`.
#[test]
fn a_macro_that_cannot_declare_an_item_in_the_span_is_not_refused() {
    let rows: Vec<(&'static str, String)> = vec![
        (
            "expression-macro-in-the-drain",
            appended("", &counted("        let _ = matches!(1, 1);\n", "")),
        ),
        (
            "trailing-macro-expression-in-the-drain",
            appended(SHADOWS_COUNT, &counted("", "        shadow!()\n")),
        ),
        (
            "item-macro-before-the-count",
            appended(
                SHADOWS_COUNT,
                &format!(
                    "fn drain(control: &mut Consumer<Record>) {{\n    shadow!();\n    let available = control.available_at_entry();\n    for _ in 0..available {{\n        {POP}\n    }}\n}}"
                ),
            ),
        ),
        (
            "item-macro-in-another-function",
            appended(
                SHADOWS_COUNT,
                &format!(
                    "fn other() {{\n    shadow!();\n}}\nfn drain(control: &mut Consumer<Record>) {{\n    let available = control.available_at_entry();\n    for _ in 0..available {{\n        {POP}\n    }}\n}}"
                ),
            ),
        ),
        (
            "trusted-debug-assert-in-the-loop",
            appended(
                "",
                &counted(
                    "        debug_assert!(true);\n",
                    "        debug_assert!(true);\n",
                ),
            ),
        ),
    ];
    each_row(rows, |name, text| accepted(name, &text));
}

/// A rule that does not bound a loop takes back the reads it noted, and only those. Two drains,
/// each separating a rewind that stops at its own mark from "no rewind" and from "rewind to zero":
///
/// - A per-queue drain inside `for _ in rounds`, a slice-parameter loop, with a statement macro
///   first in the body. The slice loop notes `rounds`; the pass-count loop `0..passes` then fails
///   `count::bounds`, and its rewind must stop at its own mark. Rewound to zero it would forget
///   `rounds`, and the macro, which declares an unbounded `const rounds`, would pass.
/// - A drain whose outer loop `0..lanes` first reads `lanes` as the alias of an entry count
///   (`let lanes = first;` notes `first`), fails because `apply(lanes)` uses the alias, and is
///   finite by its form anyway. With no rewind the stale read of `first` would extend a span over
///   a statement macro that cannot shadow anything the drain reads, and refuse it.
#[test]
fn a_loop_that_does_not_bound_takes_back_only_its_own_reads() {
    let per_queue = appended(
        SHADOWS_ROUNDS,
        "fn poll(ready: &mut Ready, rounds: &[u8]) -> u32 {\n    shadow!();\n    for _ in rounds {\n        for (lane, left) in ready.lanes.iter().zip(&mut ready.remaining) {\n            *left = lane.consumer.available_at_entry().min(DEPTH);\n        }\n        let mut passes = 1_usize;\n        for left in ready.remaining.iter() {\n            passes = passes.saturating_add(*left as usize);\n        }\n        for _ in 0..passes {\n            for ((lane, slot), left) in ready.lanes.iter_mut().zip(&mut ready.slots).zip(&mut ready.remaining) {\n                if slot.is_some() || *left == 0 {\n                    continue;\n                }\n                *left -= 1;\n                if let Ok(record) = lane.consumer.try_pop() {\n                    *slot = Some(record);\n                }\n            }\n            if ready.slots.iter().any(Option::is_none) {\n                break;\n            }\n            return 0;\n        }\n    }\n    0\n}",
    );
    refused(
        "per-queue-in-a-slice-loop-with-a-shadowing-macro",
        &per_queue,
    );
    let stale = appended(
        SHADOWS_COUNT,
        &format!(
            "fn drain(control: &mut Consumer<Record>) {{\n    let first = control.available_at_entry();\n    let lanes = first;\n    apply(lanes);\n    shadow!();\n    for _ in 0..lanes {{\n        let available = control.available_at_entry();\n        for _ in 0..available {{\n            {POP}\n        }}\n    }}\n}}"
        ),
    );
    accepted("failed-alias-read-does-not-extend-the-span", &stale);
}

/// `debug_assert!` is exempt only while no file under the roots defines or imports a macro of that
/// name (B2a-D2). Otherwise it is any other macro in statement position. The `debug_assert!` here
/// names no count, so D2 alone does not refuse it, and the two base-tree drains that hold one are
/// refused as well.
#[test]
fn an_untrusted_debug_assert_in_the_loop_refuses_the_drain() {
    let name = "untrusted-debug-assert-in-the-loop";
    let text = appended(
        "macro_rules! debug_assert {\n    ($($rest:tt)*) => {};\n}\n",
        &counted("", "").replace(
            &format!("            {POP}"),
            &format!("            debug_assert!(true);\n            {POP}"),
        ),
    );
    let tree = Tree::base(name);
    tree.apply(&[Edit::Append {
        path: BUFFER,
        text: &text,
    }]);
    let findings = assert_classes(&tree, &[DRAIN]).findings;
    let paths: Vec<_> = findings
        .iter()
        .filter_map(|finding: &Finding| finding.path.as_deref())
        .collect();
    assert_eq!(
        paths,
        [
            "crates/builtins/src/lib.rs",
            BUFFER,
            "crates/graph/src/runtime.rs"
        ],
        "{findings:#?}"
    );
    // The new drain's pop is the one in the buffer file, under the `debug_assert!`.
    let ours = findings
        .iter()
        .find(|finding| finding.path.as_deref() == Some(BUFFER))
        .expect("a finding in the buffer file");
    assert!(
        ours.source
            .as_deref()
            .is_some_and(|line| line.contains("try_pop")),
        "{ours:#?}"
    );
}
