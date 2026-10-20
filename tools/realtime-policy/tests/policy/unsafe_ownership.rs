//! The unsafe-ownership rule (D5).

use realtime_policy::Class;

use super::common::{Edit, UNSAFE, expect_failure, expect_valid};

const OUTSIDE: Class = Class::UnsafeOutsideOwnership;

fn bad_fn() -> String {
    format!("{UNSAFE} fn bad() {{}}\n")
}

fn create<'a>(path: &'a str, text: &'a str) -> Edit<'a> {
    Edit::Create { path, text }
}

/// The finding names the file, the line of the `unsafe` token and that line's text.
#[test]
fn a_finding_carries_its_file_line_and_source_line() {
    let bad = bad_fn();
    let report = expect_failure(
        "unsafe-finding-place",
        &[Edit::Append {
            path: "crates/engine/src/realtime/buffer.rs",
            text: &bad,
        }],
        &[OUTSIDE],
    );
    let [finding] = report.findings.as_slice() else {
        panic!("one finding expected: {:#?}", report.findings);
    };
    assert_eq!(
        finding.path.as_deref(),
        Some("crates/engine/src/realtime/buffer.rs")
    );
    assert_eq!(finding.line, Some(4));
    assert_eq!(finding.source.as_deref(), Some(bad.trim()));
}

/// Token-tree contents are code: a macro body that expands to an unsafe block is found, and so is
/// the argument of a macro call.
#[test]
fn unsafe_code_inside_a_macro_body_is_a_finding() {
    let definition =
        format!("macro_rules! m {{\n    () => {{\n        {UNSAFE} {{ }}\n    }};\n}}\n");
    expect_failure(
        "unsafe-in-macro",
        &[create("tools/bench/src/macro_probe.rs", &definition)],
        &[OUTSIDE],
    );
    let call = format!("fn call() {{\n    m! {{ {UNSAFE} {{ }} }}\n}}\n");
    expect_failure(
        "unsafe-in-macro-call",
        &[create("tools/bench/src/macro_call_probe.rs", &call)],
        &[OUTSIDE],
    );
}

/// A comment, a string and the other spellings are not unsafe code: the rule reads tokens.
#[test]
fn mentions_and_the_other_spellings_are_not_unsafe_code() {
    let text = format!(
        "// {UNSAFE} {{ in a comment }}\n\
         /// {UNSAFE} fn in a doc comment\n\
         const TEXT: &str = \"{UNSAFE} {{ in a string }} {UNSAFE} impl\";\n\
         {UNSAFE} trait Marker {{}}\n\
         #[{UNSAFE}(no_mangle)]\n\
         pub extern \"C\" fn exported() {{}}\n"
    );
    expect_valid(
        "unsafe-mentions",
        &[create("tools/bench/src/mentions.rs", &text)],
    );
}
