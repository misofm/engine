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
            // Root Amendment 3, F3. A counted drain, correct on its own, in a closure that stands in
            // the closure of `array::from_fn`. A rule that exempts every closure below a
            // `from_fn` closure accepts it, and the row is red then: the drain bounds itself.
            "closure-in-a-nested-closure-in-array-from-fn",
            "fn drain(control: &mut Consumer<Record>) {\n    let _: [u8; 4] = core::array::from_fn(|_| {\n        let drain_once = || {\n            let available = control.available_at_entry();\n            for _ in 0..available {\n                let Ok(record) = control.try_pop() else {\n                    break;\n                };\n                apply(record);\n            }\n        };\n        drain_once();\n        0\n    });\n}",
        ),
    ];
    each_row(rows, refuses);
}

/// D1 (root Amendment 3, MAJOR-1): the closure of `core::array::from_fn` and `std::array::from_fn`
/// is a closure like any other. The path roots `core` and `std` can be redirected (a workspace
/// `mod core`, a `use .. as core`, a macro-made module, a Cargo dependency key), so a counted drain
/// in such a closure is not a finite outer loop around a drain: it is refused, as is a pop that
/// stands directly in the closure. Red if the `array::from_fn` outer-loop exception for a closure
/// returns, in any spelling of the path.
#[test]
fn a_drain_in_the_closure_of_array_from_fn_is_refused() {
    let drain = |path: &str| {
        format!(
            "fn drain(control: &mut Consumer<Record>) {{\n    let _lanes: [u8; 4] = {path}(|_| {{\n        let available = control.available_at_entry();\n        for _ in 0..available {{\n            let Ok(record) = control.try_pop() else {{\n                break;\n            }};\n            apply(record);\n        }}\n        0\n    }});\n}}"
        )
    };
    let rows: Vec<(&'static str, String)> = vec![
        (
            "from-fn-closure-drain-is-refused",
            drain("core::array::from_fn"),
        ),
        ("std-from-fn-closure-drain", drain("std::array::from_fn")),
        (
            "rooted-from-fn-closure-drain",
            drain("::core::array::from_fn"),
        ),
        (
            "from-fn-closure-drain-with-generic-arguments",
            drain("core::array::from_fn::<u8, 4, _>"),
        ),
        (
            "from-fn-closure-pop-with-no-loop",
            "fn drain(control: &mut Consumer<Record>) {\n    let _: [Result<Record, Empty>; 4] = std::array::from_fn(|_| control.try_pop());\n}"
                .to_owned(),
        ),
    ];
    each_row(rows, |name, text| refuses(name, &text));
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

/// D3 (root Amendment 4): a path whose last segment is `try_pop` is a pop only as the callee of a
/// call. Anywhere else the value can be called from a place the walk does not see: an argument, a
/// `let`, a field, a parenthesised value, and the operand of `.map(..)` on any receiver, the
/// `Option` of `<fields>.as_mut()` included (C1: `live.rs:366`'s form, withdrawn because `.map` can
/// be a user trait method that loops, or an iterator adapter that pops once per item).
#[test]
fn a_function_value_that_pops_is_refused_unless_it_is_a_callee() {
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
            "function-value-in-a-field",
            "fn drain(control: &mut Consumer<Record>) {\n    let _ = Popper {\n        pop: Consumer::try_pop,\n    };\n}",
        ),
        (
            "function-value-in-parentheses",
            "fn drain(control: &mut Consumer<Record>) {\n    let _ = (Consumer::try_pop)(control);\n}",
        ),
        (
            // C1 (k0). A counted drain that pops through `.map(Consumer::try_pop)` on
            // `<fields>.as_mut()`. Red if `.map(<path>)` on that receiver is accepted again.
            "map-path-on-as-mut",
            "impl Drain {\n    fn run(&mut self) {\n        let available = self.control.available_at_entry();\n        for _ in 0..available {\n            let Some(Ok(record)) = self.control.as_mut().map(Consumer::try_pop) else {\n                break;\n            };\n            apply(record);\n        }\n    }\n}",
        ),
    ];
    each_row(rows, refuses);
}

/// C2 (t5): `<recv>.map_or(0, <Type>::available_at_entry)` is not an entry count. A user trait
/// `map_or` that returns `usize::MAX` makes the count unbounded, and the tool cannot see which
/// `map_or` the receiver has. A count is `<recv>.available_at_entry()[.min(<cap>)]`. Red if the
/// `map_or(0, <Type>::available_at_entry)` count form is accepted again.
#[test]
fn the_map_or_count_form_is_not_a_count() {
    refuses(
        "map-or-count-form",
        "impl Drain {\n    fn run(&mut self) {\n        let available = self\n            .control\n            .as_ref()\n            .map_or(0, Consumer::available_at_entry);\n        for _ in 0..available {\n            let Ok(record) = self.control.try_pop() else {\n                break;\n            };\n            apply(record);\n        }\n    }\n}",
    );
}

/// D3: the callee of a call stands where it is called, so the loops around it bound it.
#[test]
fn a_function_value_called_in_place_is_judged_by_its_loops() {
    accepts(
        "callee-in-a-counted-loop",
        "fn drain(control: &mut Consumer<Record>) {\n    let available = control.available_at_entry();\n    for _ in 0..available {\n        let Ok(record) = Consumer::try_pop(control) else {\n            break;\n        };\n        apply(record);\n    }\n}",
    );
    refuses(
        "callee-in-an-unbounded-loop",
        "fn drain(control: &mut Consumer<Record>) {\n    loop {\n        let Ok(record) = Consumer::try_pop(control) else {\n            break;\n        };\n        apply(record);\n    }\n}",
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

/// C10 (t6): only the single identifier `debug_assert!` is taken at its word (B1a-D7, root
/// Amendment 4). A path (`math::debug_assert!`, `::debug_assert!`) can name any macro, so a count
/// named in it is read as in any other macro and the drain is refused. Red if the exemption matches
/// by the last path segment. The bare `debug_assert!` stays accepted.
#[test]
fn a_debug_assert_spelled_as_a_path_is_not_taken_at_its_word() {
    let drain = |spelling: &str| {
        format!(
            "fn drain(control: &mut Consumer<Record>) {{\n    let available = control.available_at_entry();\n    {spelling}(available <= 64);\n    for _ in 0..available {{\n        let Ok(record) = control.try_pop() else {{\n            break;\n        }};\n        apply(record);\n    }}\n}}"
        )
    };
    let rows: [(&str, String); 2] = [
        ("path-debug-assert", drain("math::debug_assert!")),
        ("rooted-debug-assert", drain("::math::debug_assert!")),
    ];
    each_row(rows, |name, text| refuses(name, &text));
    accepts("bare-debug-assert", &drain("debug_assert!"));
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
