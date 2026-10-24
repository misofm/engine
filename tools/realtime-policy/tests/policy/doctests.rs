//! The doctest rule (#1527 rule (iv), #1831): a compiled doctest block follows the
//! unsafe-ownership rule of its file and holds no `include!`, `include_str!` or `include_bytes!`.

use std::collections::BTreeSet;

use realtime_policy::Class;

use super::common::{Edit, Tree, UNSAFE, expect_valid};

/// Create each row's file in one base tree and assert that the findings are exactly one `class`
/// finding per row, at the row's file and line.
fn assert_rows(name: &str, class: Class, rows: &[(&str, String, usize)]) {
    let tree = Tree::base(name);
    let edits: Vec<Edit<'_>> = rows
        .iter()
        .map(|(path, text, _)| Edit::Create { path, text })
        .collect();
    tree.apply(&edits);
    let found: Vec<(Class, String, usize)> = tree
        .check()
        .findings
        .into_iter()
        .map(|finding| {
            (
                finding.class,
                finding.path.unwrap_or_default(),
                finding.line.unwrap_or_default(),
            )
        })
        .collect();
    let expected: BTreeSet<(Class, String, usize)> = rows
        .iter()
        .map(|(path, _, line)| (class, (*path).to_owned(), *line))
        .collect();
    assert_eq!(found.len(), expected.len(), "case {name}: {found:#?}");
    assert_eq!(
        found.into_iter().collect::<BTreeSet<_>>(),
        expected,
        "case {name}"
    );
}

/// A `///` doc whose one block has the language string `info` and the code line `code`.
fn fenced(info: &str, code: &str) -> String {
    format!("/// Text.\n///\n/// ```{info}\n/// {code}\n/// ```\npub fn item() {{}}\n")
}

/// Every doc form, fence form and Rust language string that rustdoc compiles is read, and the
/// finding names the line of the `unsafe` token.
#[test]
fn an_unsafe_block_in_a_compiled_doctest_is_a_finding() {
    let code = format!("{UNSAFE} {{ f() }}");
    let rows = [
        ("tools/bench/src/d_bare.rs", fenced("", &code), 4),
        (
            "tools/bench/src/d_fail.rs",
            fenced("compile_fail", &code),
            4,
        ),
        ("tools/bench/src/d_norun.rs", fenced("no_run", &code), 4),
        (
            "tools/bench/src/d_panic.rs",
            fenced("should_panic", &code),
            4,
        ),
        (
            "tools/bench/src/d_edition.rs",
            fenced("edition2021", &code),
            4,
        ),
        (
            "tools/bench/src/d_hidden.rs",
            fenced("", &format!("# {code};")),
            4,
        ),
        (
            "tools/bench/src/d_order.rs",
            fenced("no_run,text", &code),
            4,
        ),
        ("tools/bench/src/d_class.rs", fenced("{.rust}", &code), 4),
        (
            "tools/bench/src/d_class_text.rs",
            fenced("{.text}", &code),
            4,
        ),
        ("tools/bench/src/d_entity.rs", fenced("r&#117;st", &code), 4),
        (
            "tools/bench/src/d_inner.rs",
            format!("//! ```\n//! {code}\n//! ```\n"),
            2,
        ),
        (
            "tools/bench/src/d_block.rs",
            format!("/**\n * ```\n * {code}\n * ```\n */\npub fn item() {{}}\n"),
            3,
        ),
        (
            "tools/bench/src/d_attr.rs",
            format!("#[doc = \"```\n{code}\n```\"]\npub fn item() {{}}\n"),
            2,
        ),
        (
            "tools/bench/src/d_tilde.rs",
            format!("/// ~~~\n/// {code}\n/// ~~~\npub fn item() {{}}\n"),
            2,
        ),
        (
            "tools/bench/src/d_four.rs",
            format!("/// ````rust\n/// {code}\n/// ````\npub fn item() {{}}\n"),
            2,
        ),
        (
            "tools/bench/src/d_quote.rs",
            format!("/// > ```\n/// > {code}\n/// > ```\npub fn item() {{}}\n"),
            2,
        ),
        (
            "tools/bench/src/d_list.rs",
            format!("/// - ```\n///   {code}\n///   ```\npub fn item() {{}}\n"),
            2,
        ),
        (
            "tools/bench/src/d_list_indented.rs",
            format!("/// -     {code}\npub fn item() {{}}\n"),
            1,
        ),
        (
            "tools/bench/src/d_indented.rs",
            format!("/// Text.\n///\n///     {code}\npub fn item() {{}}\n"),
            3,
        ),
    ];
    assert_rows("doctest-unsafe", Class::UnsafeInDoctest, &rows);
}

/// Each include form in a compiled block is a finding: rustdoc reads the named file relative to
/// the source, and the walk never sees it.
#[test]
fn an_include_in_a_compiled_doctest_is_a_finding() {
    let rows = [
        (
            "tools/bench/src/d_include.rs",
            fenced("", "include!(\"x.rs\");"),
            4,
        ),
        (
            "tools/bench/src/d_include_str.rs",
            fenced("", "let _ = include_str!(\"x\");"),
            4,
        ),
        (
            "tools/bench/src/d_include_bytes.rs",
            fenced("no_run", "let _ = include_bytes!(\"x\");"),
            4,
        ),
    ];
    assert_rows("doctest-include", Class::IncludeInDoctest, &rows);
}

/// A block that rustdoc does not compile, prose, an indented line that continues a paragraph,
/// inline code and the attribute that only names the word are not findings.
#[test]
fn a_block_that_rustdoc_does_not_compile_is_accepted() {
    let code = format!("{UNSAFE} {{ f() }} include!(\"x.rs\"); \"unbalanced");
    let mut text = String::new();
    for info in [
        "text",
        "toml",
        "sh",
        "ignore",
        "rust,ignore",
        "text,no_run",
        "text,{.rust}",
    ] {
        text.push_str(&fenced(info, &code));
    }
    text.push_str(&format!(
        "/// Prose: {UNSAFE} {{ f() }} and include!(\"x.rs\"), with `{UNSAFE} {{ f() }}` inline.\n\
         /// ```\n/// #![forbid({UNSAFE}_code)]\n/// ```\npub fn last() {{}}\n\
         /// Para\n///     {UNSAFE} {{ f() }}\npub fn para() {{}}\n"
    ));
    expect_valid(
        "doctest-not-compiled",
        &[Edit::Create {
            path: "tools/bench/src/d_accepted.rs",
            text: &text,
        }],
    );
}

/// A file on the unsafe allowlist may hold the same unsafe code in a doctest as in code.
#[test]
fn an_unsafe_doctest_in_an_allowlisted_file_is_accepted() {
    let text = fenced("", &format!("{UNSAFE} {{ f() }}"));
    expect_valid(
        "doctest-allowlisted",
        &[Edit::Append {
            path: "crates/lane/src/fpenv.rs",
            text: &text,
        }],
    );
}

/// A compiled block that does not lex is a finding at its fence, never a skip.
#[test]
fn a_doctest_that_does_not_lex_is_a_finding() {
    let rows = [(
        "tools/bench/src/d_unlexable.rs",
        format!(
            "/// ```compile_fail\n/// let s = \"{UNSAFE} {{ f() }};\n/// ```\npub fn item() {{}}\n"
        ),
        1,
    )];
    assert_rows("doctest-unlexable", Class::DoctestNotLexable, &rows);
}
