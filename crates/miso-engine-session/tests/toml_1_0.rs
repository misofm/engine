//! TOML 1.0 parser behavior fixtures independent of canonical writer spelling.

use miso_engine_session::{DiagnosticCode, canonical_session_toml, parse_session_toml};

const ESCAPES: &str = include_str!("../../../fixtures/session/v1/toml-1.0-escapes.toml");
const DUPLICATE_KEY: &str =
    include_str!("../../../fixtures/session/v1/toml-1.0-invalid-duplicate-key.toml");

#[test]
fn toml_1_0_basic_unicode_escape_and_literal_string_parse() {
    let model = parse_session_toml(ESCAPES).expect("TOML 1.0 strings parse");
    assert_eq!(
        model.sources[0].content,
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    );
    let canonical = canonical_session_toml(&model).expect("parsed model canonicalizes");
    let reparsed = parse_session_toml(&canonical).expect("canonical TOML reparses");
    assert_eq!(reparsed, model);
}

#[test]
fn toml_1_0_duplicate_keys_are_syntax_errors() {
    let error = parse_session_toml(DUPLICATE_KEY).expect_err("duplicate keys reject");
    assert!(
        error
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.code == DiagnosticCode::TomlSyntax)
    );
}
