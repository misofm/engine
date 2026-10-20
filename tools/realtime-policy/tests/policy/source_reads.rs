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
const LITERAL_IN_MACRO: Class = Class::RefusedIncludeLiteralInMacro;
const SKIPPED_MODULE: Class = Class::RefusedSkippedModule;

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
        // On a case-insensitive file system these read `target/x.rs` and `node_modules/x.rs`.
        ("reads-path-target-case", "Target/x.rs"),
        ("reads-path-node-modules-case", "a/Node_Modules/x.rs"),
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

/// Root Amendment 1, rule 2: a macro can splice the `!` on, so the word alone is refused. The
/// first row is the reproduced bypass: it reads `z.txt`, which no `include!` the scan sees names.
#[test]
fn an_include_word_without_a_bang_is_a_finding() {
    let rows = [
        (
            "reads-include-spliced",
            "macro_rules! inc { ($i:ident) => { $i!(\"z.txt\") } }\nconst X: u8 = inc!(include);\n",
        ),
        ("reads-include-function", "fn include() {}\n"),
        ("reads-include-call", "fn f() { include(); }\n"),
        ("reads-include-binding", "fn f() { let include = 1; }\n"),
        ("reads-include-method", "fn f(x: X) { x.include(); }\n"),
        ("reads-include-raw", "fn f() { let r#include = 1; }\n"),
        ("reads-include-path-prefix", "const X: u8 = std::include;\n"),
    ];
    for (name, text) in rows {
        expect_failure(
            name,
            &[
                create(READS, text),
                create("crates/lane/src/z.txt", LOOPING_TRAIT),
            ],
            &[INCLUDE],
        );
    }
}

/// Root Amendment 1, rule 3: an `include!` in a macro token tree is read relative to the file that
/// expands the macro, so it is refused though `digests.in` is a file of literals that the same
/// `include!` outside a macro passes.
#[test]
fn an_include_in_a_macro_token_tree_is_a_finding_even_for_a_file_of_literals() {
    let rows = [
        (
            "reads-include-macro-invocation",
            "const X: u8 = m!(include!(\"digests.in\"));\n",
        ),
        (
            "reads-include-macro-invocation-nested",
            "const X: u8 = m!([n!({ std::include!(\"digests.in\") })]);\n",
        ),
        (
            "reads-include-macro-rules-body",
            "macro_rules! m { () => { include!(\"digests.in\") } }\nconst X: u8 = m!();\n",
        ),
        (
            "reads-include-macro-decl-body",
            "macro m() { include!(\"digests.in\") }\n",
        ),
        (
            "reads-include-macro-raw",
            "const X: u8 = m!(r#include!(\"digests.in\"));\n",
        ),
        // The word in any form, with a `!` that no group follows.
        (
            "reads-include-macro-ne",
            "const X: bool = m!(include != 1);\n",
        ),
        (
            "reads-include-macro-bang-word",
            "const X: bool = m!(include ! x);\n",
        ),
    ];
    for (name, text) in rows {
        expect_failure(
            name,
            &[
                create(READS, text),
                create("crates/lane/src/digests.in", "[1u8]\n"),
            ],
            &[INCLUDE],
        );
    }
}

/// Root Amendment 1, rule 4: `include_str!` and `include_bytes!` are allowed outside a macro token
/// tree (the accepted-forms test) and refused inside one, where the file is read relative to the
/// expanding file.
#[test]
fn include_str_and_include_bytes_in_a_macro_token_tree_are_findings() {
    let rows = [
        (
            "reads-literal-invocation",
            "const X: &str = m!(include_str!(\"looping.in\"));\n",
        ),
        (
            "reads-literal-vec",
            "const X: &[&[u8]] = &vec![include_bytes!(\"looping.in\")];\n",
        ),
        (
            "reads-literal-macro-rules-body",
            "macro_rules! m { () => { include_str!(\"looping.in\") } }\nconst X: &str = m!();\n",
        ),
        (
            "reads-literal-nested",
            "const X: &str = m!([n!({ core::include_bytes!(\"looping.in\") })]);\n",
        ),
        (
            "reads-literal-raw",
            "const X: &str = m!(r#include_str!(\"looping.in\"));\n",
        ),
        // The identifier alone, which a macro can splice a `!` onto.
        ("reads-literal-word", "const X: &str = m!(include_str);\n"),
    ];
    for (name, text) in rows {
        expect_failure(
            name,
            &[
                create(READS, text),
                create("crates/lane/src/looping.in", LOOPING_TRAIT),
            ],
            &[LITERAL_IN_MACRO],
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

/// The walk skips `target` and `node_modules`, so the files rustc resolves from a `mod` named like
/// one are never read. The first row is the false pass of attempt 2: its `#[path = "x.rs"]` is a
/// plain literal that (a) accepts, and rustc reads `target/x.rs`.
#[test]
fn a_module_named_like_a_skipped_directory_is_a_finding() {
    let rows = [
        (
            "reads-mod-target-path",
            "mod target {\n    #[path = \"x.rs\"]\n    mod y;\n}\n",
        ),
        ("reads-mod-raw-target", "mod r#target;\n"),
        (
            "reads-mod-node-modules",
            "mod node_modules {\n    mod y;\n}\n",
        ),
        ("reads-mod-out-of-line", "mod target;\n"),
        ("reads-mod-pub", "pub(crate) mod node_modules;\n"),
        (
            "reads-mod-nested",
            "mod a {\n    mod b {\n        mod target;\n    }\n}\n",
        ),
        (
            "reads-mod-in-macro-body",
            "macro_rules! m { () => { mod target; } }\nm!();\n",
        ),
        (
            "reads-mod-in-macro-call",
            "m!(mod node_modules { mod y; });\n",
        ),
        ("reads-mod-in-fn", "fn f() {\n    mod target {}\n}\n"),
    ];
    for (name, text) in rows {
        expect_failure(name, &[create(READS, text)], &[SKIPPED_MODULE]);
    }
}

/// On a case-insensitive file system `mod Target;` reads `target/mod.rs`.
#[test]
fn a_skipped_directory_name_is_matched_ignoring_case() {
    let rows = [
        ("reads-mod-case-target", "mod Target;\n"),
        (
            "reads-mod-case-node-modules",
            "mod Node_Modules {\n    mod y;\n}\n",
        ),
        ("reads-mod-case-raw", "mod r#TARGET;\n"),
    ];
    for (name, text) in rows {
        expect_failure(name, &[create(READS, text)], &[SKIPPED_MODULE]);
    }
}

/// The finding sits on the module's name.
#[test]
fn the_finding_names_the_line_of_the_module() {
    let report = expect_failure(
        "reads-mod-line",
        &[create(
            READS,
            "// one\n#[allow(dead_code)]\nmod\n    target;\n",
        )],
        &[SKIPPED_MODULE],
    );
    assert_eq!(report.findings.len(), 1);
    assert_eq!(report.findings[0].line, Some(4));
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
                // The literal macros as an argument, outside a macro token tree.
                "fn l() -> &'static str { g(include_str!(\"looping.in\")) }\n",
                // A word that only starts with `include`.
                "fn include_nothing() {}\nmacro_rules! include_more { () => {} }\n",
                // A `path` or `include` word that is not in a written attribute or an include call.
                "fn m(entry: &E, path: &P) {\n    assert_eq!(entry.path, path);\n    let _ = json!({\"path\": path});\n}\n",
                "macro_rules! n { ($path:literal) => { f($path) } }\n",
                // Names that only contain or start like a skipped directory, and uses of one.
                "mod targets;\nmod target_dir;\nmod node_modules_x;\nmod my_target;\n",
                "use crate::target;\nfn target() {}\nstruct Target;\nfn o() { let target = 1; let _ = target; }\n",
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
