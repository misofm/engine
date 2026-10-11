//! A pop behind a closure, a macro token tree or a function value (B2a-D1 to D3), and the
//! `debug_assert!` that a file under the roots shadows (B2a-D2).
//!
//! B1a and B1b ported `main`'s rule as it stands: a closure or a macro was transparent, and a
//! function value was a pop like any other. The cases below fail when a pop that runs wherever its
//! closure is called, its macro expands or its value is passed passes again.

use realtime_policy::Finding;

use super::super::common::{BEGIN, END, Edit, Tree, assert_classes};
use super::{DRAIN, accepts, each_row, refuses};

const BUILTINS: &str = "crates/builtins/src/lib.rs";
const ROUTE_TWIN: &str = "crates/graph/src/runtime.rs";

/// D1: a pop in a closure is refused wherever the closure is called, and a closure does not hide
/// the pop from a loop that calls it.
#[test]
fn a_pop_in_a_closure_is_refused() {
    let rows: [(&str, &str); 5] = [
        (
            "closure-in-a-let-called-in-a-loop",
            "fn drain(control: &mut Consumer<Record>) {\n    let mut pop = || control.try_pop();\n    loop {\n        let Ok(record) = pop() else {\n            break;\n        };\n        apply(record);\n    }\n}",
        ),
        (
            "closure-called-once",
            "fn drain(control: &mut Consumer<Record>) {\n    let pop = || control.try_pop();\n    let _ = pop();\n}",
        ),
        (
            "closure-in-a-counted-loop",
            "fn drain(control: &mut Consumer<Record>) {\n    let available = control.available_at_entry();\n    for _ in 0..available {\n        let Ok(record) = (|| control.try_pop())() else {\n            break;\n        };\n        apply(record);\n    }\n}",
        ),
        (
            "closure-argument-of-an-adapter",
            "fn drain(control: &mut Consumer<Record>) {\n    let available = control.available_at_entry();\n    for _ in 0..available {\n        let _ = Some(0).and_then(|_| control.try_pop().ok());\n    }\n}",
        ),
        (
            "closure-in-a-nested-closure-in-array-from-fn",
            "fn drain(control: &mut Consumer<Record>) {\n    let _: [u8; 4] = core::array::from_fn(|_| {\n        let pop = || control.try_pop();\n        let _ = pop();\n        0\n    });\n}",
        ),
    ];
    each_row(rows, refuses);
}

/// D2: a `try_pop` token in a macro's token tree is refused, in a macro call, in the body of a
/// `macro_rules!` and in the arguments of `debug_assert!`, whatever loop surrounds the call.
#[test]
fn a_pop_in_a_macro_token_tree_is_refused() {
    let rows: [(&str, &str); 5] = [
        (
            "macro-body-with-a-pop",
            "macro_rules! drain_once {\n    ($control:ident) => {\n        $control.try_pop()\n    };\n}",
        ),
        (
            "macro-call-in-a-counted-loop",
            "fn drain(control: &mut Consumer<Record>) {\n    let available = control.available_at_entry();\n    for _ in 0..available {\n        note!(control.try_pop());\n    }\n}",
        ),
        (
            "macro-call-in-an-unbounded-loop",
            "fn drain(control: &mut Consumer<Record>) {\n    loop {\n        note!(control.try_pop());\n    }\n}",
        ),
        (
            "macro-call-with-no-loop",
            "fn drain(control: &mut Consumer<Record>) {\n    note!(control.try_pop());\n}",
        ),
        (
            "debug-assert-argument",
            "fn drain(control: &mut Consumer<Record>) {\n    debug_assert!(control.try_pop().is_ok());\n}",
        ),
    ];
    each_row(rows, refuses);
}

/// D3: a path whose last segment is `try_pop` is a pop only where it is called at once: as the
/// callee of a call, or as the only argument of `.map(..)`. Anywhere else the value can be called
/// from a place the walk does not see.
#[test]
fn a_function_value_that_pops_is_refused_unless_it_is_called_in_place() {
    let rows: [(&str, &str); 7] = [
        (
            "function-value-in-a-let",
            "fn drain(control: &mut Consumer<Record>) {\n    let pop = Consumer::try_pop;\n    let _ = pop(control);\n}",
        ),
        (
            "function-value-in-a-counted-loop",
            "fn drain(control: &mut Consumer<Record>) {\n    let available = control.available_at_entry();\n    let pop = Consumer::try_pop;\n    for _ in 0..available {\n        let Ok(record) = pop(control) else {\n            break;\n        };\n        apply(record);\n    }\n}",
        ),
        (
            "function-value-as-an-argument",
            "fn drain(control: &mut Consumer<Record>) {\n    let _ = repeat_until_empty(control, Consumer::try_pop);\n}",
        ),
        (
            "function-value-in-and-then",
            "fn drain(control: &mut Consumer<Record>) {\n    let _ = Some(control).and_then(Consumer::try_pop);\n}",
        ),
        (
            "function-value-in-map-with-another-argument",
            "fn drain(control: &mut Consumer<Record>) {\n    let _ = pool.map(Consumer::try_pop, 4);\n}",
        ),
        (
            "function-value-in-a-field",
            "fn drain(control: &mut Consumer<Record>) {\n    let _ = Popper {\n        pop: Consumer::try_pop,\n    };\n}",
        ),
        (
            "function-value-in-parentheses",
            "fn drain(control: &mut Consumer<Record>) {\n    let _ = (Consumer::try_pop)(control);\n}",
        ),
    ];
    each_row(rows, refuses);
}

/// D3: the callee of a call and the only argument of `.map(..)` stand where they are called, so
/// the loops around them bound them.
#[test]
fn a_function_value_called_in_place_is_judged_by_its_loops() {
    accepts(
        "callee-in-a-counted-loop",
        "fn drain(control: &mut Consumer<Record>) {\n    let available = control.available_at_entry();\n    for _ in 0..available {\n        let Ok(record) = Consumer::try_pop(control) else {\n            break;\n        };\n        apply(record);\n    }\n}",
    );
    accepts(
        "map-argument-in-a-counted-loop",
        "impl Drain {\n    fn run(&mut self) {\n        let available = self\n            .control\n            .as_ref()\n            .map_or(0, Consumer::available_at_entry);\n        for _ in 0..available {\n            let Some(Ok(record)) = self.control.as_mut().map(Consumer::try_pop) else {\n                break;\n            };\n            apply(record);\n        }\n    }\n}",
    );
    refuses(
        "callee-in-an-unbounded-loop",
        "fn drain(control: &mut Consumer<Record>) {\n    loop {\n        let Ok(record) = Consumer::try_pop(control) else {\n            break;\n        };\n        apply(record);\n    }\n}",
    );
    refuses(
        "map-argument-in-an-unbounded-loop",
        "impl Drain {\n    fn run(&mut self) {\n        loop {\n            let Some(Ok(record)) = self.control.as_mut().map(Consumer::try_pop) else {\n                break;\n            };\n            apply(record);\n        }\n    }\n}",
    );
}

/// A file with no markers that shadows `debug_assert`, and the findings that follow: the base
/// tree's valid drain names its count in a `debug_assert!`, and a `debug_assert!` that is not the
/// standard macro is read as any other macro, so the drain is refused (and the route twin, whose
/// loop body holds one, by D10).
fn shadowed_by_file(name: &str, text: &str) -> Vec<Finding> {
    let tree = Tree::base(name);
    tree.apply(&[Edit::Create {
        path: "crates/shadowing/src/lib.rs",
        text,
    }]);
    assert_classes(&tree, &[DRAIN]).findings
}

/// The two findings that a shadowed `debug_assert!` causes in the base tree: the valid drain that
/// names its count in one (D2), and the route twin, whose loop body holds one (D10: while it is not
/// trusted, `debug_assert!` is a macro in statement position like any other).
fn assert_base_drains_refused(name: &str, findings: &[Finding]) {
    assert_eq!(findings.len(), 2, "case {name}: {findings:#?}");
    assert_eq!(findings[0].path.as_deref(), Some(BUILTINS), "case {name}");
    assert_eq!(
        findings[0].source.as_deref(),
        Some("let Ok(record) = control.try_pop() else {"),
        "case {name}"
    );
    assert_eq!(findings[1].path.as_deref(), Some(ROUTE_TWIN), "case {name}");
    assert_eq!(
        findings[1].source.as_deref(),
        Some("let Ok(record) = self.control.consumer.try_pop() else {"),
        "case {name}"
    );
}

/// D2: the base tree's `debug_assert!(available <= ..)` is taken at its word, until any file under
/// the roots, marked or not, defines or imports a macro of that name.
#[test]
fn a_debug_assert_that_a_file_shadows_is_not_taken_at_its_word() {
    let rows: [(&'static str, &'static str); 8] = [
        (
            "use-renamed-to-debug-assert",
            "use core::debug_assert as debug_assert;\n",
        ),
        ("use-of-debug-assert", "use other::debug_assert;\n"),
        (
            "use-under-another-name",
            "use other::assert as debug_assert;\n",
        ),
        (
            "use-in-a-group",
            "pub(crate) use other::{debug_assert, Other};\n",
        ),
        (
            "macro-rules-at-the-top",
            "macro_rules! debug_assert {\n    ($x:ident $($rest:tt)*) => {\n        let $x = usize::MAX;\n    };\n}\n",
        ),
        (
            "macro-rules-in-a-function-body",
            "fn unrelated() {\n    macro_rules! debug_assert {\n        ($x:ident) => {};\n    }\n}\n",
        ),
        ("declarative-macro-2", "macro debug_assert($x:ident) {}\n"),
        (
            "macro-rules-in-a-macro-body",
            "macro_rules! outer {\n    () => {\n        macro_rules! debug_assert {\n            ($x:ident) => {};\n        }\n    };\n}\n",
        ),
    ];
    each_row(rows, |name, text| {
        assert_base_drains_refused(name, &shadowed_by_file(name, text));
    });
}

/// D2: text that is not a definition or an import of the name leaves `debug_assert!` trusted: a
/// string, a comment, a longer name, a `use` of another macro, a `use<..>` bound.
#[test]
fn text_that_does_not_shadow_debug_assert_leaves_it_trusted() {
    let rows: [(&'static str, &'static str); 5] = [
        (
            "string-and-comment",
            "// use other::debug_assert;\nconst NOTE: &str = \"macro_rules! debug_assert { }\";\n",
        ),
        (
            "longer-names",
            "use core::debug_assert_eq;\nmacro_rules! debug_assert_ne2 {\n    () => {};\n}\n",
        ),
        (
            "use-of-another-name",
            "use other::{assert, debug_assertion};\n",
        ),
        (
            "use-then-a-call-of-debug-assert",
            "use other::Thing;\nfn unrelated() {\n    debug_assert!(true);\n}\n",
        ),
        (
            "precise-capturing-bound",
            "fn unrelated<'a>(x: &'a u8) -> impl Sized + use<'a> {\n    debug_assert!(*x == 1);\n    x\n}\n",
        ),
    ];
    each_row(rows, |name, text| {
        let tree = Tree::base(name);
        tree.apply(&[Edit::Create {
            path: "crates/shadowing/src/lib.rs",
            text,
        }]);
        assert_classes(&tree, &[]);
    });
}

/// D2: e07's escape across files. `lib.rs` defines `macro_rules! debug_assert` and then declares
/// `mod drain;`, whose file defines no macro and has no `use`. Judged one file at a time, the
/// drain's `debug_assert!` would still be the standard macro.
#[test]
fn a_debug_assert_shadowed_in_the_parent_module_is_shadowed_in_the_child() {
    let tree = Tree::base("debug-assert-shadowed-in-the-parent");
    let marked = format!(
        "// {BEGIN}\nfn drain_matrix_controls(control: &mut Consumer<Record>) {{\n    let available = control.available_at_entry();\n    debug_assert!(available <= 64);\n    for _ in 0..available {{\n        let Ok(record) = control.try_pop() else {{\n            break;\n        }};\n        apply(record);\n    }}\n}}\n// {END}\n"
    );
    tree.apply(&[
        Edit::Create {
            path: "crates/shadowing/src/lib.rs",
            text: "macro_rules! debug_assert {\n    ($x:ident $($rest:tt)*) => {\n        let $x = usize::MAX;\n    };\n}\n\nmod drain;\n",
        },
        Edit::Create {
            path: "crates/shadowing/src/drain.rs",
            text: &marked,
        },
    ]);
    let findings = assert_classes(&tree, &[DRAIN]).findings;
    let paths: Vec<_> = findings
        .iter()
        .filter_map(|finding| finding.path.as_deref())
        .collect();
    assert_eq!(
        paths,
        [BUILTINS, ROUTE_TWIN, "crates/shadowing/src/drain.rs"],
        "{findings:#?}"
    );
}
