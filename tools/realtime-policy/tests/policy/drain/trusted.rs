//! The trusted-name scan (B2a-D3, root Amendment 4): a name that the drain rule takes at its word
//! is refused wherever a file under the roots gives it another meaning, in a form the rule's text
//! does not allow, and the written escapes (`#[macro_use]`, `no_implicit_prelude`, `feature(..)`,
//! `proc_macro*`) that could rebind a name without a written definition.
//!
//! Every case puts the second meaning in a file with no marker (or in the nested module of one),
//! beside the base tree's valid drains. Each case then fails with the drain class, in the
//! offending file, at the line of the offending name. Names that a macro expansion builds from
//! parts are limit 4 of the threat model and have no case here.

use realtime_policy::Finding;

use super::super::common::{Edit, Tree, assert_classes};
use super::{DRAIN, each_row};

const SHADOWING: &str = "crates/shadowing/src/lib.rs";
/// The one file that defines the pop and the count (the base tree's copy holds `fn try_pop`).
const SPSC: &str = "crates/engine/src/realtime/spsc.rs";

/// The base tree with `text` in a file with no marker. Returns the findings, after asserting that
/// all are of the drain class.
fn findings_of(name: &str, text: &str) -> Vec<Finding> {
    let tree = Tree::base(name);
    tree.apply(&[Edit::Create {
        path: SHADOWING,
        text,
    }]);
    assert_classes(&tree, &[DRAIN]).findings
}

/// Assert that the case is refused with every finding in the shadowing file, the first at a line
/// that holds `needle`.
fn refused(name: &str, text: &str, needle: &str) {
    let findings = findings_of(name, text);
    assert!(
        findings
            .iter()
            .all(|finding| finding.path.as_deref() == Some(SHADOWING)),
        "case {name}: {findings:#?}"
    );
    assert!(
        findings[0]
            .source
            .as_deref()
            .is_some_and(|line| line.contains(needle)),
        "case {name}: {findings:#?}"
    );
}

/// The base tree beside `text` in a file with no marker passes.
fn accepted(name: &str, text: &str) {
    let tree = Tree::base(name);
    tree.apply(&[Edit::Create {
        path: SHADOWING,
        text,
    }]);
    assert_classes(&tree, &[]);
}

/// C4 (t2): a written `fn try_pop` outside `spsc.rs`, with a body that loops, is a pop the rule
/// would take as the one pop. Red if a definition of `try_pop` outside `spsc.rs` is allowed, for
/// example if (q) keeps the old rule that allowed any `fn try_pop(&mut self`.
#[test]
fn a_definition_of_try_pop_outside_the_queue_is_refused() {
    let rows: [(&'static str, &'static str, &'static str); 4] = [
        (
            "try-pop-looping-on-a-ring",
            "trait Spin {\n    fn try_pop(&mut self) -> Result<u32, ()>;\n}\nimpl Spin for Ring {\n    fn try_pop(&mut self) -> Result<u32, ()> {\n        while self.n > 0 {\n            self.n -= 1;\n        }\n        Ok(1)\n    }\n}\n",
            "fn try_pop",
        ),
        (
            "try-pop-by-value-on-a-reference-impl",
            "impl Pop for &Queue {\n    fn try_pop(self) -> Result<u8, ()> {\n        Err(())\n    }\n}\n",
            "fn try_pop",
        ),
        (
            "try-pop-as-a-free-fn",
            "fn try_pop(queue: &mut Q) {}\n",
            "fn try_pop",
        ),
        (
            "try-pop-imported-in-a-group",
            "use other::{try_pop, Thing};\n",
            "use other",
        ),
    ];
    each_row(
        rows.map(|(name, text, needle)| (name, (text, needle))),
        |name, (text, needle)| {
            refused(name, text, needle);
        },
    );
}

/// C5 (t7): a written `fn available_at_entry` outside `spsc.rs` that returns `usize::MAX` makes
/// every counted drain unbounded. Red if `available_at_entry` is not one of the trusted names.
#[test]
fn a_definition_of_available_at_entry_outside_the_queue_is_refused() {
    let text = "trait Spin {\n    fn available_at_entry(&self) -> usize;\n}\nimpl Spin for Ring {\n    fn available_at_entry(&self) -> usize {\n        usize::MAX\n    }\n}\n";
    refused(
        "available-at-entry-returning-max",
        text,
        "fn available_at_entry",
    );
}

/// (q), second definition in `spsc.rs`: the file that defines the pop and the count gets exactly
/// one written `fn` token for each name, and no second one. Red if the allowance is not one per
/// name: a looping `fn try_pop` or `fn available_at_entry` added to `spsc.rs` beside the real one
/// passes. The first `fn available_at_entry` of the file is the definition (accepted); the second
/// is refused at its own line.
#[test]
fn a_second_definition_in_the_defining_file_is_refused() {
    let define = |name: &str| format!("fn {name}(&self) -> usize {{ 0 }}\n");
    let appended = |case: &str, text: &str| {
        let tree = Tree::base(case);
        tree.apply(&[Edit::Append { path: SPSC, text }]);
        tree
    };
    // The base copy of `spsc.rs` holds one `fn try_pop`: a second is refused, at its line.
    let tree = appended("second-try-pop", &define("try_pop"));
    let line = tree.read(SPSC).lines().count();
    let findings = assert_classes(&tree, &[DRAIN]).findings;
    let [finding] = findings.as_slice() else {
        panic!("one finding expected: {findings:#?}");
    };
    assert_eq!(finding.path.as_deref(), Some(SPSC));
    assert_eq!(finding.line, Some(line));
    // The first `fn available_at_entry` is the definition; the second is not.
    assert_classes(
        &appended("first-available-at-entry", &define("available_at_entry")),
        &[],
    );
    let both = format!(
        "{}{}",
        define("available_at_entry"),
        define("available_at_entry")
    );
    let tree = appended("second-available-at-entry", &both);
    let line = tree.read(SPSC).lines().count();
    let findings = assert_classes(&tree, &[DRAIN]).findings;
    let [finding] = findings.as_slice() else {
        panic!("one finding expected: {findings:#?}");
    };
    assert_eq!(finding.line, Some(line));
}

/// C11: `#[macro_use] extern crate x;` brings a registry crate's macros, `debug_assert` among
/// them, into every module. Red if (u) is dropped. Its accepted half, `#[macro_use] mod` (the real
/// `crates/math/src/lib.rs:52`), is red if the exemption is dropped.
#[test]
fn macro_use_is_refused_except_on_a_module() {
    let rows: [(&'static str, &'static str); 3] = [
        (
            "macro-use-on-extern-crate",
            "#[macro_use]\nextern crate shadowing_macros;\n",
        ),
        ("inner-macro-use", "#![macro_use]\n"),
        (
            "macro-use-on-another-item",
            "#[macro_use]\nuse other::thing;\n",
        ),
    ];
    each_row(rows, |name, text| refused(name, text, "macro_use"));
    accepted("macro-use-on-a-module", "#[macro_use]\nmod vendored;\n");
}

/// C12: `#[no_implicit_prelude]` changes what the implicit prelude names in its module, so a
/// `debug_assert!` there may be another crate's. Red if (n) is dropped.
#[test]
fn no_implicit_prelude_is_refused() {
    let rows: [(&'static str, &'static str); 2] = [
        (
            "no-implicit-prelude-on-a-module",
            "#[no_implicit_prelude]\nmod quiet {}\n",
        ),
        ("inner-no-implicit-prelude", "#![no_implicit_prelude]\n"),
    ];
    each_row(rows, |name, text| {
        refused(name, text, "no_implicit_prelude")
    });
}

/// C13: `feature(..)` enables the nightly features that build names from parts, and
/// `#![cfg_attr(x, feature(..))]` does it with no `#![feature(` in a row. A `$` followed by a
/// `{` group is a metavariable expression (`${concat(..)}`). Red if the rule matches only the token
/// sequence `#![feature(`.
#[test]
fn feature_enabling_and_metavariable_expressions_are_refused() {
    let rows: [(&'static str, &'static str, &'static str); 3] = [
        (
            "feature-attribute",
            "#![feature(macro_metavar_expr_concat)]\n",
            "feature",
        ),
        (
            "cfg-attr-feature",
            "#![cfg_attr(nightly, feature(macro_metavar_expr_concat))]\n",
            "feature",
        ),
        (
            "metavariable-expression",
            "macro_rules! mk {\n    ($a:ident) => {\n        fn ${concat($a, _x)}() {}\n    };\n}\n",
            "${concat",
        ),
    ];
    each_row(
        rows.map(|(name, text, needle)| (name, (text, needle))),
        |name, (text, needle)| {
            refused(name, text, needle);
        },
    );
    // A word `feature` that no group follows, and a `$` metavariable, are not features.
    accepted(
        "feature-as-a-word",
        "fn describe(feature: &str) -> &str {\n    feature\n}\nmacro_rules! keep {\n    ($feature:expr) => {\n        $feature\n    };\n}\n",
    );
}

/// (d): a `#[proc_macro]` that a macro makes. The name `emit` is no trusted name. A workspace proc
/// macro can emit any name, pop or shadow, and the scan cannot see it. Red if (d) is dropped.
#[test]
fn a_proc_macro_that_a_macro_makes_is_refused() {
    let text = "macro_rules! mk {\n    ($n:ident !) => {\n        #[proc_macro]\n        pub fn $n(input: TokenStream) -> TokenStream {\n            input\n        }\n    };\n}\nmk!(emit !);\n";
    refused("macro-made-proc-macro", text, "#[proc_macro]");
    let rows: [(&'static str, &'static str); 2] = [
        (
            "proc-macro-attribute",
            "#[proc_macro_attribute]\npub fn note(a: TokenStream, b: TokenStream) -> TokenStream {\n    b\n}\n",
        ),
        (
            "proc-macro-derive",
            "#[proc_macro_derive(Note)]\npub fn note(a: TokenStream) -> TokenStream {\n    a\n}\n",
        ),
    ];
    each_row(rows, |name, text| {
        let findings = findings_of(name, text);
        assert!(!findings.is_empty(), "case {name}");
    });
}

/// The trusted names in the forms the rule allows are not findings: `.try_pop()` and
/// `.available_at_entry()` calls, `Consumer::try_pop(c)` as a callee, a longer name that starts
/// like a trusted one, `#[macro_use] mod`, a word `feature` that no group follows, a `$` that
/// opens no brace group, and a macro that emits no name.
#[test]
fn the_trusted_names_in_their_allowed_forms_pass() {
    let text = "fn forms(c: &mut Consumer, o: Option<u8>) {\n    let _ = c.try_pop();\n    let _ = c.available_at_entry();\n    let _ = Consumer::try_pop(c);\n    let _ = Consumer::available_at_entry(c);\n    let _ = o.map(|x| x + 1);\n    let try_pop_count = 3;\n    let feature = try_pop_count + 1;\n    let _ = feature;\n}\n#[macro_use]\nmod vendored;\nmacro_rules! twice {\n    ($e:expr) => {\n        ($e, $e)\n    };\n}\n";
    accepted("trusted-names-in-their-allowed-forms", text);
}
