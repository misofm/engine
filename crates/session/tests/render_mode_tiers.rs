//! Launch render-mode policy coverage.
//!
//! `single_thread` is the only V1 render-mode token. The retired `dependency_waves` named a native
//! dependency-wave executor that was removed as production-unreachable, and #1063 removed the token
//! too: the model, the parser, the canonical writer and the protocol wire no longer know it, so a
//! session that spells it is an unknown value like any other misspelling.

use session::{DiagnosticCode, parse_session_json};

const SESSION: &str = include_str!("../../../fixtures/session/v1/canonical.json");

fn source_with_mode(mode: &str) -> String {
    SESSION.replacen(
        "\"mode\": \"single_thread\"",
        &format!("\"mode\": \"{mode}\""),
        1,
    )
}

/// The retired token refuses at parse exactly as an unallocated spelling does: one
/// `schema.invalid_enum` at the mode field, whose message names `single_thread` as the only token.
#[test]
fn retired_dependency_waves_is_an_unknown_enum_value() {
    for mode in ["dependency_waves", "wave_farm"] {
        let error = parse_session_json(&source_with_mode(mode)).expect_err("unknown render mode");
        assert_eq!(error.diagnostics().len(), 1, "{mode}");
        let diagnostic = &error.diagnostics()[0];
        assert_eq!(diagnostic.code, DiagnosticCode::InvalidEnum, "{mode}");
        assert_eq!(
            diagnostic.path.to_string(),
            "$.render_profile.mode",
            "{mode}"
        );
        assert_eq!(
            diagnostic.message, "expected one of: single_thread",
            "{mode}"
        );
    }
}
