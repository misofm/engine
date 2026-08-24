//! Accepted parser grammar and text-versus-typed diagnostic provenance.

use miso_engine_session::{
    DiagnosticCode, SessionTomlV1, canonical_session_toml, parse_session_toml,
};

const CANONICAL: &str = include_str!("../../../fixtures/session/v1/canonical.toml");

fn only_diagnostic<'a>(
    error: &'a miso_engine_session::DiagnosticSet,
    code: DiagnosticCode,
    path: &str,
) -> &'a miso_engine_session::Diagnostic {
    let diagnostic = error
        .diagnostics()
        .iter()
        .find(|item| item.code == code && item.path.to_string() == path)
        .unwrap_or_else(|| panic!("missing {code} at {path}: {error}"));
    assert_eq!(error.diagnostics().len(), 1, "unexpected extra diagnostics");
    diagnostic
}

#[test]
fn trailing_comma_and_newline_in_inline_table_parse() {
    let source = CANONICAL.replace(
        "render_profile = { id = \"native\", mode = \"single_thread\" }",
        "render_profile = {\n  id = \"native\",\n  mode = \"single_thread\",\n}",
    );
    let model = parse_session_toml(&source).expect("TOML 1.1 inline-table grammar parses");
    assert_eq!(model.render_profile.id.as_str(), "native");
}

#[test]
fn escape_and_hex_byte_basic_string_escapes_parse() {
    let source = CANONICAL.replace("sha256:demo", r"sha256:\e\x41");
    let model = parse_session_toml(&source).expect("TOML 1.1 basic-string escapes parse");
    assert_eq!(model.sources[0].content.identity, "sha256:\u{1b}A");
}

#[test]
fn duplicate_key_syntax_span_covers_the_second_key() {
    let source = "schema_version = 1\nschema_version = 1\n";
    let error = parse_session_toml(source).expect_err("duplicate keys reject");
    let diagnostic = only_diagnostic(&error, DiagnosticCode::TomlSyntax, "$");
    let second = source
        .match_indices("schema_version")
        .nth(1)
        .expect("second key")
        .0;
    let span = diagnostic
        .span
        .expect("parse syntax diagnostics have spans");
    assert_eq!(
        (span.byte_start, span.byte_end),
        (second, second + "schema_version".len())
    );
}

#[test]
fn textual_u64_max_rejects_at_revision_with_a_span() {
    let source = CANONICAL.replace("revision = 7", "revision = 18446744073709551615");
    let error = parse_session_toml(&source).expect_err("TOML integer exceeds i64");
    let diagnostic = only_diagnostic(
        &error,
        DiagnosticCode::NumericOutOfSchemaRange,
        "$.revision",
    );
    let span = diagnostic.span.expect("text diagnostics have spans");
    let value = source.find("18446744073709551615").expect("large integer");
    assert_eq!((span.byte_start, span.byte_end), (value, value + 20));
}

#[test]
fn typed_u64_above_i64_max_has_matching_code_and_path_without_span() {
    let mut model: SessionTomlV1 = parse_session_toml(CANONICAL).expect("fixture parses");
    model.revision = i64::MAX as u64 + 1;
    let error = canonical_session_toml(&model).expect_err("typed revision exceeds TOML range");
    let diagnostic = only_diagnostic(
        &error,
        DiagnosticCode::NumericOutOfSchemaRange,
        "$.revision",
    );
    assert_eq!(diagnostic.span, None);
}
