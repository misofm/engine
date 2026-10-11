//! The trusted-name scan (B2a-D3, root Amendment 2, SP-A): a name that the drain rule takes at its
//! word is refused wherever a file under the roots gives it another meaning, in a form the rule's
//! text does not allow.
//!
//! Every case puts the second meaning in a file with no marker (or in the nested module of one),
//! beside the drain that the rule would accept without it: the counted drain of
//! `self.control.as_mut().map(Consumer::try_pop)`, which is `live.rs:366`'s form. Each case then
//! fails with the drain class, at the line of the offending name.

use realtime_policy::Finding;

use super::super::common::{Edit, Tree, assert_classes};
use super::{ANCHOR, BUILTINS_COMPILER, DRAIN, each_row};

/// A counted drain that pops through `.map(Consumer::try_pop)` on `<fields>.as_mut()`: accepted.
const DRAIN_THROUGH_MAP: &str = "impl Drain {\n    fn run(&mut self) {\n        let available = self\n            .control\n            .as_ref()\n            .map_or(0, Consumer::available_at_entry);\n        for _ in 0..available {\n            let Some(Ok(record)) = self.control.as_mut().map(Consumer::try_pop) else {\n                break;\n            };\n            apply(record);\n        }\n    }\n}";

const SHADOWING: &str = "crates/shadowing/src/lib.rs";

/// The base tree with `text` in a file with no marker, and the drain through `.map` in its second
/// region. Returns the findings, after asserting that all are of the drain class.
fn findings_of(name: &str, text: &str) -> Vec<Finding> {
    let tree = Tree::base(name);
    tree.apply(&[
        Edit::ReplaceLine {
            path: BUILTINS_COMPILER,
            old: ANCHOR,
            new: DRAIN_THROUGH_MAP,
        },
        Edit::Create {
            path: SHADOWING,
            text,
        },
    ]);
    assert_classes(&tree, &[DRAIN]).findings
}

/// Assert that the case is refused, with every finding in the shadowing file, the first at a line
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

/// The drain through `.map` passes beside text that holds the trusted names in their allowed forms.
fn accepted(name: &str, text: &str) {
    let tree = Tree::base(name);
    tree.apply(&[
        Edit::ReplaceLine {
            path: BUILTINS_COMPILER,
            old: ANCHOR,
            new: DRAIN_THROUGH_MAP,
        },
        Edit::Create {
            path: SHADOWING,
            text,
        },
    ]);
    assert_classes(&tree, &[]);
}

/// (a): F1's reproducer. A user trait `Spin` has a `map` whose `impl` calls its argument in a
/// `while f(self).is_ok() {}` loop; it pops 1000 of 1000 records through `Consumer::try_pop`. It
/// sits in `mod a { mod b { .. } }` of another file. Red if the scan reads only the top-level items
/// of a file, or one file, or drops (a): this shape passes without it.
#[test]
fn a_user_trait_with_a_map_in_a_nested_module_is_refused() {
    let text = "mod a {\n    mod b {\n        trait Spin<T> {\n            fn map<R, E>(self, f: fn(&mut T) -> Result<R, E>) -> usize;\n        }\n        impl<T> Spin<T> for &mut T {\n            fn map<R, E>(self, f: fn(&mut T) -> Result<R, E>) -> usize {\n                let mut n = 0;\n                while f(self).is_ok() {\n                    n += 1;\n                }\n                n\n            }\n        }\n    }\n}\n";
    refused("user-map-trait-in-a-nested-module", text, "fn map");
}

/// (a): the other trusted method name (red if (a) names only `map`), the other keywords that give
/// an item its name (red if (a) looks only after `fn`), and a `use` item that brings a `map` into
/// scope (red if the `use` items are not read).
#[test]
fn a_definition_import_or_rename_of_map_or_as_mut_is_refused() {
    let rows: [(&'static str, &'static str); 5] = [
        ("fn-as-mut", "trait Spin {\n    fn as_mut(&mut self);\n}\n"),
        ("fn-map-in-an-impl", "impl Q {\n    fn map(self) {}\n}\n"),
        ("use-of-map-in-a-group", "use other::{map, Thing};\n"),
        ("const-named-as-mut", "const as_mut: u8 = 0;\n"),
        ("macro-named-map", "macro_rules! map {\n    () => {};\n}\n"),
    ];
    each_row(rows, |name, text| {
        let findings = findings_of(name, text);
        assert!(!findings.is_empty(), "case {name}");
    });
}

/// (b): `impl Pop for &Q { fn try_pop(self) }`. The receiver is `self`, by value on a reference
/// type, so a check for `&self` alone does not see it. Red if (b) allows any `fn try_pop` with a
/// receiver other than `&mut self`.
#[test]
fn a_try_pop_taken_by_value_on_a_reference_impl_is_refused() {
    let text = "trait Pop {\n    fn try_pop(self) -> Result<u8, ()>;\n}\nimpl Pop for &Queue {\n    fn try_pop(self) -> Result<u8, ()> {\n        Err(())\n    }\n}\n";
    refused("try-pop-by-value-on-a-reference-impl", text, "fn try_pop");
    let rows: [(&'static str, &'static str); 3] = [
        (
            "try-pop-by-shared-reference",
            "impl Q {\n    fn try_pop(&self) {}\n}\n",
        ),
        ("try-pop-as-a-free-fn", "fn try_pop(queue: &mut Q) {}\n"),
        (
            "use-of-try-pop-in-a-group",
            "use other::{try_pop, Thing};\n",
        ),
    ];
    each_row(rows, |name, text| {
        let findings = findings_of(name, text);
        assert!(!findings.is_empty(), "case {name}");
    });
}

/// (c): `mk!(x.map)` where `mk!` emits `fn $n`: a trait whose `map` loops. The text `fn map` is not
/// in the file, and `.map` follows a `.`. It pops 1000 of 1000. Red if (c) is dropped or matches
/// only the text `fn map`.
#[test]
fn a_map_name_that_a_macro_makes_is_refused() {
    let text = "macro_rules! mk {\n    ($a:ident . $n:ident) => {\n        trait Spin<T> {\n            fn $n<R, E>(self, f: fn(&mut T) -> Result<R, E>) -> usize;\n        }\n        impl<T> Spin<T> for &mut T {\n            fn $n<R, E>(self, f: fn(&mut T) -> Result<R, E>) -> usize {\n                let mut n = 0;\n                while f(self).is_ok() {\n                    n += 1;\n                }\n                n\n            }\n        }\n    };\n}\nmk!(x.map);\n";
    refused("macro-made-map-name", text, "mk!(x.map)");
}

/// (c): an outer macro passes `$n` to an inner macro that emits `fn $n`. Red if the set of
/// name-emitting macros is not closed transitively.
#[test]
fn a_name_passed_to_an_inner_name_emitting_macro_is_refused() {
    let text = "macro_rules! inner {\n    ($a:ident . $n:ident) => {\n        trait Spin {\n            fn $n(self);\n        }\n    };\n}\nmacro_rules! outer {\n    ($a:ident . $n:ident) => {\n        inner!($a . $n);\n    };\n}\nouter!(x.map);\n";
    refused(
        "macro-passes-name-to-an-inner-name-emitting-macro",
        text,
        "outer!(x.map)",
    );
}

/// (c): an invocation through a name that a `use` item renames counts as an invocation. Red if the
/// renames are not followed.
#[test]
fn a_name_emitting_macro_invoked_through_a_rename_is_refused() {
    let text = "macro_rules! mk {\n    ($a:ident . $n:ident) => {\n        trait Spin {\n            fn $n(self);\n        }\n    };\n}\nuse crate::mk as spin;\nspin!(x.as_mut);\n";
    refused(
        "name-emitting-macro-through-a-rename",
        text,
        "spin!(x.as_mut)",
    );
}

/// (d): a `#[proc_macro]` that a macro makes. The name `emit` is no trusted name, so (c) is silent.
/// A workspace proc macro can emit any name, pop or shadow. Red if (d) is dropped.
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

/// The trusted names in the forms the rule allows are not findings: `.map` and `.as_mut` calls, a
/// path that ends in `map`, the real `try_pop` definition, a call through `Consumer::try_pop`, a
/// macro that emits no name, and a local variable called `map`.
#[test]
fn the_trusted_names_in_their_allowed_forms_pass() {
    let text = "impl Consumer {\n    pub fn try_pop(&mut self) -> Result<u8, ()> {\n        Err(())\n    }\n}\nfn forms(v: Vec<u8>, o: Option<u8>, c: &mut Consumer) {\n    let _ = v.iter().map(|x| x + 1).count();\n    let _ = o.as_ref().map(|x| x + 1);\n    let _ = Option::map(o, |x| x + 1);\n    let _ = Consumer::try_pop(c);\n    let map = 3;\n    let _ = map + 1;\n}\nmacro_rules! twice {\n    ($e:expr) => {\n        ($e, $e)\n    };\n}\nfn macros(v: Vec<u8>) {\n    let _ = twice!(v.iter().map(|x| x + 1).count());\n}\n";
    accepted("trusted-names-in-their-allowed-forms", text);
}
