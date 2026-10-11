//! The forbidden-body predicate (A2-D2).

use realtime_policy::Class;

use super::common::{
    BASE_FILES, BASE_REGIONS, BEGIN, END, Edit, expect_failure, expect_valid, region,
};

const FORBIDDEN: Class = Class::ForbiddenBody;
const BUFFER: &str = "crates/engine/src/realtime/buffer.rs";

/// Replace the one-line empty function `old` in `path` with `new`.
fn body<'a>(path: &'a str, old: &'a str, new: &'a str) -> Edit<'a> {
    Edit::ReplaceLine { path, old, new }
}

fn append_region(text: &str) -> Vec<Edit<'_>> {
    vec![Edit::Append { path: BUFFER, text }]
}

/// The script's nine forbidden-body cases. Each puts one forbidden spelling in a marked
/// function, in the files the gate has always scanned and in the marked files outside them.
#[test]
fn a_forbidden_spelling_in_a_region_is_a_finding() {
    let tools_probe = format!("// {BEGIN}\nfn tool() {{ let _ = vec![0u8; 1]; }}\n// {END}\n");
    let rows: [(&str, Edit); 9] = [
        (
            "allocation",
            body(
                BUFFER,
                "fn render() {}",
                "fn render() { let _ = Vec::new(); }",
            ),
        ),
        (
            "lock",
            body(
                "crates/engine/src/realtime/disjoint.rs",
                "fn queue() {}",
                "fn queue() { let _ = Mutex::new(0); }",
            ),
        ),
        (
            "log",
            body(
                "crates/engine/src/realtime/observe.rs",
                "fn buffer() {}",
                "fn buffer() { println!(\"bad\"); }",
            ),
        ),
        (
            "panic-path-expect",
            body(
                "crates/engine/src/realtime/plan.rs",
                "fn exchange() {}",
                "fn exchange() { None::<u8>.expect(\"x\"); }",
            ),
        ),
        (
            "panic-path-macro",
            body(
                "crates/engine/src/realtime/plan.rs",
                "fn exchange() {}",
                "fn exchange() { unreachable!(); }",
            ),
        ),
        (
            "marked-outside-realtime-root",
            body(
                "hosts/host-web/src/lib.rs",
                "fn render_next() {}",
                "fn render_next() { let _ = vec![0u8; 1]; }",
            ),
        ),
        (
            "marked-runtime-execute-op",
            body(
                "crates/graph/src/runtime.rs",
                "    fn execute_op() {}",
                "    fn execute_op() { let _ = vec![0u8; 1]; }",
            ),
        ),
        (
            "marked-effect-control-lane-stage",
            body(
                "crates/effect-contract/src/live.rs",
                "    fn stage() {}",
                "    fn stage() { let _ = vec![0u8; 1]; }",
            ),
        ),
        (
            "marked-tools-root-scanned",
            Edit::Create {
                path: "tools/audit/src/marker_probe.rs",
                text: &tools_probe,
            },
        ),
    ];
    for (name, edit) in rows {
        expect_failure(name, &[edit], &[FORBIDDEN]);
    }
}

/// A finding names the line where the spelling starts, even when its tokens are on two lines.
#[test]
fn a_finding_names_the_line_where_the_spelling_starts() {
    let text = region("fn probe() {\n    let _ = Vec\n        ::new();\n}");
    let report = expect_failure("forbidden-split-vec", &append_region(&text), &[FORBIDDEN]);
    let [finding] = report.findings.as_slice() else {
        panic!("one finding expected: {:#?}", report.findings);
    };
    assert_eq!(finding.path.as_deref(), Some(BUFFER));
    // The base file's three lines, the begin marker, the function's first line, then the `Vec`.
    assert_eq!(finding.line, Some(6));
    assert_eq!(finding.source.as_deref(), Some("let _ = Vec"));
}

/// Two spellings on a line are one finding for the line; two lines are two findings.
#[test]
fn findings_are_one_per_line() {
    let text = region("fn probe() {\n    let _ = (Vec::new(), String::new());\n    drop(1);\n}");
    let report = expect_failure("forbidden-per-line", &append_region(&text), &[FORBIDDEN]);
    let lines: Vec<_> = report.findings.iter().map(|finding| finding.line).collect();
    assert_eq!(lines, [Some(6), Some(7)]);
}

/// Spellings that a layout or a macro would hide from a line match.
#[test]
fn a_spelling_the_line_scan_missed_is_a_finding() {
    let rows = [
        (
            "forbidden-in-macro-arguments",
            "fn probe() { emit!(Vec::new()); }",
        ),
        (
            "forbidden-in-macro-braces",
            "fn probe() { emit! { Box::new(1) } }",
        ),
        (
            "forbidden-in-nested-macro-group",
            "fn probe() { emit!([ (String::new()) ]); }",
        ),
        (
            "forbidden-turbofish-collect",
            "fn probe() { let _ = items.iter().collect::<Vec<_>>(); }",
        ),
        (
            "forbidden-plain-collect",
            "fn probe() { let _ = items.iter().collect(); }",
        ),
        (
            "forbidden-split-across-lines",
            "fn probe() {\n    let _ = Vec\n        ::new();\n}",
        ),
        (
            "forbidden-spaced-path",
            "fn probe() { let _ = Vec :: new(); }",
        ),
        ("forbidden-spaced-call", "fn probe() { drop (value); }"),
        (
            "forbidden-spaced-macro",
            "fn probe() { println ! (\"x\"); }",
        ),
        (
            "forbidden-async-no-space",
            "fn probe() { let _ = async{ 1 }; }",
        ),
        ("forbidden-async-fn", "async fn probe() {}"),
        (
            "forbidden-async-move",
            "fn probe() { let _ = async move { 1 }; }",
        ),
        ("forbidden-await", "async fn probe() { other().await; }"),
    ];
    for (name, code) in rows {
        let text = region(code);
        expect_failure(name, &append_region(&text), &[FORBIDDEN]);
    }
}

/// A longer identifier still matches where the line scan matched it: the predicate reads words
/// the way the regex read text, and does not add word boundaries.
#[test]
fn a_longer_identifier_still_matches() {
    let rows = [
        (
            "forbidden-small-vec",
            "fn probe() { let _ = SmallVec::new(); }",
        ),
        (
            "forbidden-c-string",
            "fn probe() { let _ = CString::new(1); }",
        ),
        ("forbidden-do-drop", "fn probe() { do_drop(1); }"),
        ("forbidden-mutex-guard", "fn probe(guard: MutexGuard) {}"),
    ];
    for (name, code) in rows {
        let text = region(code);
        expect_failure(name, &append_region(&text), &[FORBIDDEN]);
    }
}

/// Words in comments and in literals are not spellings.
#[test]
fn a_forbidden_word_in_a_comment_or_a_literal_passes() {
    let text = region(
        "fn probe() {\n    // Vec::new() and drop(x) and async fn and .unwrap() in a comment\n    /* panic!(\"x\") */\n    let _ = \"Vec::new() format!() .collect::<X>()\";\n    let _ = r#\"drop(x) .unwrap()\"#;\n    let _ = b\"Mutex\";\n    let _ = 'x';\n    /// A doc comment says Vec::new() too.\n    let _ = 1;\n}",
    );
    let report = expect_valid(
        "forbidden-words-in-comments-and-literals",
        &append_region(&text),
    );
    assert_eq!(
        (report.regions, report.files),
        (BASE_REGIONS + 1, BASE_FILES)
    );
}

/// Two words stay two words: the rendering never joins them into a spelling.
#[test]
fn adjacent_words_do_not_join_into_a_spelling() {
    let text = region("fn probe() { emit!(Mu tex, Rw Lock, Con dvar, to do ! ()); }");
    expect_valid("forbidden-adjacent-words", &append_region(&text));
}

/// Only the region's own tokens are read: a spelling just outside it passes, and every region of
/// a file is read.
#[test]
fn only_the_tokens_between_the_markers_are_read() {
    let outside = format!(
        "fn before() {{ let _ = Vec::new(); }}\n// {BEGIN}\nfn inside() {{}}\n// {END}\nfn after() {{ let _ = Vec::new(); }}\n"
    );
    expect_valid("forbidden-outside-the-region", &append_region(&outside));
    let second = format!(
        "// {BEGIN}\nfn first() {{}}\n// {END}\nfn between() {{}}\n// {BEGIN}\nfn second() {{ drop(1); }}\n// {END}\n"
    );
    expect_failure(
        "forbidden-in-second-region",
        &append_region(&second),
        &[FORBIDDEN],
    );
}
