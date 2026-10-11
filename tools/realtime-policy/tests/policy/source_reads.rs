//! The sources rustc reads that the walk does not (#1519 D1): `#[path]` attributes and `include!`.
//!
//! Every case writes one file under `crates/lane/src` (the base tree's lane crate) and, for an
//! `include!`, the file it names. The tool never resolves a declaration, so no named module needs
//! to exist.

use realtime_policy::Class;

use super::common::{Edit, Tree, assert_classes, expect_failure, expect_valid};

const PATH: Class = Class::RefusedPathAttribute;
const INCLUDE: Class = Class::RefusedInclude;
const INCLUDE_IMPORT: Class = Class::RefusedIncludeImport;

const READS: &str = "crates/lane/src/reads.rs";

fn create<'a>(path: &'a str, text: &'a str) -> Edit<'a> {
    Edit::Create { path, text }
}

/// A source file that no rule judges as a looping `map` trait would be judged only if rustc read it.
const LOOPING_TRAIT: &str = "pub trait Looper { fn map(&mut self) -> Option<u8>; }\n";

#[test]
fn a_path_to_a_file_that_is_not_rust_is_a_finding() {
    expect_failure(
        "reads-path-txt",
        &[create(READS, "#[path = \"shadow.txt\"]\nmod m;\n")],
        &[PATH],
    );
}

/// The finding names the `path` token's line, not the item's.
#[test]
fn the_finding_names_the_line_of_the_path() {
    let report = expect_failure(
        "reads-path-line",
        &[create(
            READS,
            "// one\n#[allow(dead_code)]\n#[path =\n \"shadow.txt\"]\nmod m;\n",
        )],
        &[PATH],
    );
    assert_eq!(report.findings.len(), 1);
    assert_eq!(report.findings[0].path.as_deref(), Some(READS));
    assert_eq!(report.findings[0].line, Some(3));
}

/// A plain `.rs` literal is accepted, so it is the `cfg_attr` arguments that make these refused:
/// rustc builds the file as production code through them.
#[test]
fn a_path_in_cfg_attr_arguments_is_a_finding_even_for_a_plain_rs_file() {
    let rows = [
        (
            "reads-cfg-attr",
            "#[cfg_attr(all(), path = \"ok.rs\")]\nmod m;\n",
        ),
        (
            "reads-cfg-attr-feature",
            "#[cfg_attr(feature = \"x\", path = \"tests.rs\")]\nmod prod;\n",
        ),
        (
            "reads-cfg-attr-nested",
            "#[cfg_attr(a, cfg_attr(b, path = \"ok.rs\"))]\nmod m;\n",
        ),
        (
            "reads-cfg-attr-inner",
            "#![cfg_attr(all(), path = \"ok.rs\")]\n",
        ),
    ];
    for (name, text) in rows {
        expect_failure(name, &[create(READS, text)], &[PATH]);
    }
}

/// A plain `.rs` literal is accepted outside a macro, so it is the macro token tree that makes
/// these refused: the declaration is invisible to a resolver that reads items.
#[test]
fn a_path_in_a_macro_token_tree_is_a_finding_even_for_a_plain_rs_file() {
    let rows = [
        (
            "reads-macro-rules-body",
            "macro_rules! m { () => { #[path = \"ok.rs\"] mod x; } }\nm!();\n",
        ),
        (
            "reads-macro-invocation",
            "m! {\n    #[path = \"ok.rs\"]\n    mod x;\n}\n",
        ),
        (
            "reads-macro-invocation-paren",
            "m!(#[path = \"ok.rs\"] mod x;);\n",
        ),
        (
            "reads-macro-path-invocation",
            "crate::m![#[path = \"ok.rs\"] mod x;];\n",
        ),
        (
            "reads-macro-decl-body",
            "macro m() { #[path = \"ok.rs\"] mod x; }\n",
        ),
        (
            "reads-macro-decl-body-braced",
            "macro m { () => { #[path = \"ok.rs\"] mod x; } }\n",
        ),
        (
            "reads-macro-in-fn-body",
            "fn f() { m!(#[path = \"ok.rs\"] mod x;); }\n",
        ),
        (
            "reads-macro-in-macro",
            "m!(n!(#[path = \"ok.rs\"] mod x;));\n",
        ),
    ];
    for (name, text) in rows {
        expect_failure(name, &[create(READS, text)], &[PATH]);
    }
}

#[test]
fn a_path_to_a_file_outside_the_workspace_is_a_finding() {
    let rows = [
        ("reads-path-parent-dirs", "../../../scripts/x.rs"),
        ("reads-path-parent-inside", "a/../x.rs"),
        ("reads-path-target", "a/target/x.rs"),
        ("reads-path-target-first", "target/x.rs"),
        ("reads-path-node-modules", "node_modules/x.rs"),
        ("reads-path-absolute", "/etc/x.rs"),
        ("reads-path-no-suffix", "x"),
        ("reads-path-rs-directory-name", "x.rs/"),
    ];
    for (name, value) in rows {
        let text = format!("#[path = \"{value}\"]\nmod m;\n");
        expect_failure(name, &[create(READS, &text)], &[PATH]);
    }
}

/// Neither a macro's metavariable nor an expression is a literal. The second row is outside any
/// macro, so only the literal rule refuses it.
#[test]
fn a_path_that_is_not_a_string_literal_is_a_finding() {
    let rows = [
        (
            "reads-path-metavariable",
            "macro_rules! m { ($p:literal) => { #[path = $p] mod x; } }\nm!(\"ok.rs\");\n",
        ),
        (
            "reads-path-concat",
            "#[path = concat!(\"ok\", \".rs\")]\nmod m;\n",
        ),
        ("reads-path-bare", "#[path]\nmod m;\n"),
        ("reads-path-byte-string", "#[path = b\"ok.rs\"]\nmod m;\n"),
        ("reads-path-const", "#[path = DIR]\nmod m;\n"),
        ("reads-path-parenthesised", "#[path(\"ok.rs\")]\nmod m;\n"),
    ];
    for (name, text) in rows {
        expect_failure(name, &[create(READS, text)], &[PATH]);
    }
}

/// rustc reads `r#path` as `path` and `r#include` as `include`.
#[test]
fn raw_identifiers_do_not_hide_a_path_or_an_include() {
    expect_failure(
        "reads-raw-path",
        &[create(READS, "#[r#path = \"shadow.txt\"]\nmod m;\n")],
        &[PATH],
    );
    expect_failure(
        "reads-raw-include",
        &[
            create(READS, "const X: u8 = r#include!(\"hidden.in\");\n"),
            create("crates/lane/src/hidden.in", LOOPING_TRAIT),
        ],
        &[INCLUDE],
    );
    expect_failure(
        "reads-raw-include-import",
        &[create(READS, "use std::r#include as inc;\n")],
        &[INCLUDE_IMPORT],
    );
}

#[test]
fn an_include_of_source_is_a_finding() {
    expect_failure(
        "reads-include-looping-trait",
        &[
            create(READS, "include!(\"looping.in\");\n"),
            create("crates/lane/src/looping.in", LOOPING_TRAIT),
        ],
        &[INCLUDE],
    );
}

/// Each row is an `include!` that names no file of literals and punctuation that the tool read.
#[test]
fn an_include_that_the_tool_cannot_judge_is_a_finding() {
    let rows = [
        (
            "reads-include-missing-file",
            "const X: u8 = include!(\"missing.in\");\n",
        ),
        (
            "reads-include-concat",
            "const X: u8 = include!(concat!(\"digests\", \".in\"));\n",
        ),
        (
            "reads-include-other-macro",
            "const X: u8 = include!(env!(\"FILE\"));\n",
        ),
        ("reads-include-empty", "const X: u8 = include!();\n"),
        (
            "reads-include-two-arguments",
            "const X: u8 = include!(\"digests.in\", \"digests.in\");\n",
        ),
        (
            "reads-include-trailing-comma",
            "const X: u8 = include!(\"digests.in\",);\n",
        ),
        (
            "reads-include-byte-string",
            "const X: u8 = include!(b\"digests.in\");\n",
        ),
        (
            "reads-include-std-path",
            "const X: u8 = std::include!(\"looping.in\");\n",
        ),
        (
            "reads-include-core-path",
            "const X: u8 = ::core::include!(\"looping.in\");\n",
        ),
        (
            "reads-include-brackets",
            "const X: u8 = include![\"looping.in\"];\n",
        ),
        (
            "reads-include-in-macro-argument",
            "m!(include!(\"looping.in\"));\n",
        ),
    ];
    for (name, text) in rows {
        expect_failure(
            name,
            &[
                create(READS, text),
                create("crates/lane/src/looping.in", LOOPING_TRAIT),
                create("crates/lane/src/digests.in", "[1u8, 2, 3]\n"),
            ],
            &[INCLUDE],
        );
    }
}

/// Each row is an included file with a token that is not a literal or punctuation, at the top
/// level, behind groups, or as an item; the last cannot be lexed.
#[test]
fn an_included_file_with_an_identifier_is_a_finding() {
    let rows = [
        ("reads-included-ident", "[1u8, two]\n"),
        (
            "reads-included-nested-ident",
            "[[1u8, 2], [3, (4, five)]]\n",
        ),
        ("reads-included-item", "fn evil() {}\n"),
        ("reads-included-true", "[true, 1u8]\n"),
        ("reads-included-attribute", "#[path = \"x.rs\"] 1u8\n"),
        ("reads-included-lex-error", "[1u8, \"unterminated\n"),
    ];
    for (name, text) in rows {
        expect_failure(
            name,
            &[
                create(READS, "const X: u8 = include!(\"digests.in\");\n"),
                create("crates/lane/src/digests.in", text),
            ],
            &[INCLUDE],
        );
    }
}

/// The file is read from the directory of the file that invokes `include!`: the copy beside the
/// invoking file is judged, and a copy elsewhere does not stand in for it.
#[test]
fn an_include_is_read_from_the_directory_of_the_invoking_file() {
    let invoking = "const X: u8 = include!(\"digests.in\");\n";
    expect_valid(
        "reads-include-beside-file",
        &[
            create("crates/lane/src/nested/reads.rs", invoking),
            create("crates/lane/src/nested/digests.in", "[1u8]\n"),
        ],
    );
    expect_failure(
        "reads-include-elsewhere",
        &[
            create("crates/lane/src/nested/reads.rs", invoking),
            create("crates/lane/src/digests.in", "[1u8]\n"),
        ],
        &[INCLUDE],
    );
}

/// The file exists and holds literals only, so only its absolute spelling is refused.
#[test]
fn an_absolute_include_is_a_finding() {
    let tree = Tree::base("reads-include-absolute");
    let digests = tree.path("crates/lane/src/digests.in");
    tree.apply(&[
        create(
            READS,
            &format!(
                "const X: u8 = include!({:?});\n",
                digests.display().to_string()
            ),
        ),
        create("crates/lane/src/digests.in", "[1u8]\n"),
    ]);
    assert_classes(&tree, &[INCLUDE]);
}

#[test]
fn naming_include_in_a_use_item_is_a_finding() {
    let rows = [
        (
            "reads-use-rename",
            "use std::include as inc;\nconst X: u8 = inc!(\"z.txt\");\n",
        ),
        ("reads-use-pub", "pub use core::include as inc;\n"),
        (
            "reads-use-pub-crate",
            "pub(crate) use ::std::include as inc;\n",
        ),
        (
            "reads-use-group",
            "use std::{string::String, include as inc};\n",
        ),
        ("reads-use-nested-group", "use std::{a::{b, include}};\n"),
        ("reads-use-no-rename", "use std::include;\n"),
        ("reads-use-in-fn", "fn f() { use std::include as inc; }\n"),
        ("reads-use-in-macro", "m!(use std::include as inc;);\n"),
    ];
    for (name, text) in rows {
        expect_failure(
            name,
            &[
                create(READS, text),
                create("crates/lane/src/z.txt", LOOPING_TRAIT),
            ],
            &[INCLUDE_IMPORT],
        );
    }
}

/// Every form here reads nothing a rule needs to judge, or reads a literal only.
#[test]
fn the_accepted_forms_pass() {
    let tree = Tree::base("reads-accepted");
    tree.apply(&[
        create(
            READS,
            concat!(
                "use std::string::String;\n",
                // The one real `path` attribute: a relative `.rs` file, at file level and in an
                // inline module, with names that only contain `target` and `node_modules`.
                "#[path = \"support/successor.rs\"]\nmod a;\n",
                "mod inline {\n    #[path = \"targets/node_modules_x.rs\"]\n    mod b;\n}\n",
                "#[path = \"target.rs\"]\nmod c;\n",
                "#[path = r\"raw.rs\"]\nmod d;\n",
                // An attribute that has a `path` word only in a string, or another name.
                "#[cfg(feature = \"path\")]\nfn e() {}\n",
                "#[allow(clippy::path_buf_push_overwrite)]\nfn f() {}\n",
                // `include!` of a file of literals, with a path prefix, in any delimiter.
                "const G: [[u8; 2]; 2] = include!(\"digests.in\");\n",
                "const H: [[u8; 2]; 2] = std::include!(\"digests.in\");\n",
                "const I: [[u8; 2]; 2] = include![\"digests.in\"];\n",
                // The macros that produce a literal.
                "const J: &str = include_str!(\"looping.in\");\n",
                "const K: &[u8] = include_bytes!(\"looping.in\");\n",
                // A word `include` that is not a macro call or a use item.
                "fn include() {}\nconst L: u8 = 0;\nfn m() { include(); }\n",
                "macro_rules! include_nothing { () => {} }\n",
            ),
        ),
        create(
            "crates/lane/src/digests.in",
            "[[1u8, 2], [0xff, -3]] // two\n",
        ),
        create("crates/lane/src/looping.in", LOOPING_TRAIT),
    ]);
    assert_classes(&tree, &[]);
}
