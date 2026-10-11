//! Macros and attributes that can declare an item between a binding and its loop (B1b-D10).
//!
//! A `macro_rules!` item name is not hygienic, and a derive or an attribute macro emits items
//! beside its item: either can declare a `const`, a `static` or a unit struct that shadows the
//! count, its alias source or the slice parameter for the whole block it lands in. D2's item scan
//! reads the tokens of the region and cannot see an expansion. The cases below put the macro
//! outside the region, as the production shape would, and a pop in a loop whose bound the rule can
//! prove from the syntax alone.

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
/// function body. The count is `available_at_entry()` written in the header, so no `let` binds it.
fn over_slice(before: &str) -> String {
    format!(
        "fn drain(control: &mut Consumer<Record>, lanes: &[u8]) {{\n{before}    for _ in lanes {{\n        for _ in 0..control.available_at_entry() {{\n            {POP}\n        }}\n    }}\n}}"
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

/// The same hole through a slice parameter: the span runs from the start of the `fn` body, and a
/// braced macro statement is a macro statement.
#[test]
fn a_statement_macro_in_the_body_of_a_function_with_a_slice_loop_is_refused() {
    let text = appended(SHADOWS_SLICE, &over_slice("    shadow! {}\n"));
    refused("braced-item-macro-shadows-the-slice-parameter", &text);
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
