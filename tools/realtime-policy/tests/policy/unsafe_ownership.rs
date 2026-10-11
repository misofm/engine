//! The unsafe-ownership rule (D5).

use realtime_policy::Class;

use super::common::{Edit, UNSAFE, expect_failure, expect_valid};

const OUTSIDE: Class = Class::UnsafeOutsideOwnership;

fn bad_fn() -> String {
    format!("{UNSAFE} fn bad() {{}}\n")
}

/// Each row puts unsafe code in a file the allowlist does not name. Several rows are one directory
/// or one file stem away from a listed file: a prefix or directory match would let them through.
#[test]
fn unsafe_code_outside_the_allowlist_is_a_finding() {
    let bad = bad_fn();
    let extern_fn = format!("pub {UNSAFE} extern \"C\" fn bad() {{}}\n");
    let rows: [(&str, Edit); 16] = [
        (
            "unsafe-scope",
            Edit::Append {
                path: "crates/engine/src/realtime/buffer.rs",
                text: &bad,
            },
        ),
        (
            "unsafe-outside-exact-allowlist",
            create("tools/bench/src/other.rs", &bad),
        ),
        (
            "unsafe-outside-capi-audit-main",
            create("tools/audit/src/other.rs", &bad),
        ),
        (
            "unsafe-in-deleted-native-pcm-runner-lib",
            create("tools/native-pcm-runner/src/lib.rs", &bad),
        ),
        (
            "unsafe-in-deleted-bench-protocol",
            create("tools/bench/src/protocol.rs", &bad),
        ),
        (
            "unsafe-in-deleted-core-arch",
            create("crates/engine/src/arch/x86.rs", &bad),
        ),
        (
            "unsafe-outside-capi-ffi",
            create("crates/capi/src/lib.rs", &extern_fn),
        ),
        (
            "unsafe-in-second-capi-ffi-path",
            create("crates/capi/src/ffi/other.rs", &bad),
        ),
        (
            "unsafe-outside-capi-lifecycle-audit",
            create("crates/capi/tests/other.rs", &bad),
        ),
        (
            "unsafe-outside-session-allocation-budget",
            create("crates/session/tests/other.rs", &bad),
        ),
        (
            "unsafe-outside-web-ffi",
            Edit::Append {
                path: "hosts/host-web/src/lib.rs",
                text: &extern_fn,
            },
        ),
        (
            "unsafe-in-second-web-ffi-path",
            create("hosts/host-web/src/ffi/other.rs", &bad),
        ),
        (
            "unsafe-outside-web-peak-audit",
            create("hosts/host-web/tests/other.rs", &bad),
        ),
        (
            "unsafe-outside-disjoint-arena",
            create("crates/engine/src/realtime/disjoint_extra.rs", &bad),
        ),
        (
            "unsafe-outside-lane-softfma",
            create("crates/lane/src/kernels.rs", &bad),
        ),
        (
            "unsafe-outside-lane-fpenv",
            create("crates/lane/src/fpenv_extra.rs", &bad),
        ),
    ];
    for (name, edit) in rows {
        expect_failure(name, &[edit], &[OUTSIDE]);
    }
}

fn create<'a>(path: &'a str, text: &'a str) -> Edit<'a> {
    Edit::Create { path, text }
}

/// The five files that the 2026-10-08 amendment dropped from the allowlist must stay dropped:
/// each holds no unsafe code today, and a listed file lets a later change add some unreviewed.
#[test]
fn the_dropped_allowlist_entries_are_findings() {
    let bad = bad_fn();
    for path in [
        "crates/soft-clip/tests/allocation.rs",
        "crates/transient-shaper/tests/allocation.rs",
        "crates/true-peak-limiter/tests/allocation.rs",
        "crates/multiband-compressor/tests/no_alloc_render.rs",
        "tools/wasm-gate-guest/src/lib.rs",
    ] {
        expect_failure(
            &format!("dropped-{}", path.replace('/', "-")),
            &[create(path, &bad)],
            &[OUTSIDE],
        );
    }
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

/// Token-tree contents are code: a macro body that expands to an unsafe block is found.
#[test]
fn unsafe_code_inside_a_macro_body_is_a_finding() {
    let text = format!("macro_rules! m {{\n    () => {{\n        {UNSAFE} {{ }}\n    }};\n}}\n");
    expect_failure(
        "unsafe-in-macro",
        &[create("tools/bench/src/macro_probe.rs", &text)],
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
