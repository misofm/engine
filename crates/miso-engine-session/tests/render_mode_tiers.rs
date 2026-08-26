//! Launch render-mode policy coverage.
//!
//! `dependency_waves` stays a parseable V1 token -- the model, the parser and the protocol wire
//! all still carry it, and canonical round-trip doctrine forbids normalizing it away. What it no
//! longer does is launch: the native dependency-wave executor it named was removed as
//! production-unreachable, so declaring it is now a typed rejection rather than a declaration
//! that silently renders single-threaded.

use miso_engine_session::{
    CompileCaps, DiagnosticCode, canonical_session_toml, compile_session, parse_session_toml,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/canonical.toml");
const MESSAGE: &str = "launch render_profile.mode must be single_thread";

fn caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

fn source_with_mode(mode: &str) -> String {
    SESSION.replacen("mode = \"single_thread\"", &format!("mode = \"{mode}\""), 1)
}

fn assert_launch_diagnostic(diagnostics: &miso_engine_session::DiagnosticSet) {
    assert_eq!(diagnostics.diagnostics().len(), 1);
    let diagnostic = &diagnostics.diagnostics()[0];
    assert_eq!(
        diagnostic.code,
        DiagnosticCode::RenderModeUnsupportedAtLaunch
    );
    assert_eq!(diagnostic.path.to_string(), "$.render_profile.mode");
    assert_eq!(diagnostic.message, MESSAGE);
}

#[test]
fn single_thread_parses_compiles_and_canonicalizes() {
    let model = parse_session_toml(&source_with_mode("single_thread")).expect("launch parse");
    let compiled = compile_session(&model, caps()).expect("launch compile");
    assert_eq!(
        canonical_session_toml(compiled.normalized_model()).expect("canonical"),
        compiled.canonical_toml()
    );
}

/// The same three entry points the sample-rate tier gates, so no caller reaches a prepared plan
/// through a door that forgot to check.
#[test]
fn dependency_waves_rejects_with_one_stable_diagnostic_at_every_entry_point() {
    let parsed = parse_session_toml(&source_with_mode("dependency_waves"));
    assert_launch_diagnostic(&parsed.expect_err("unsupported at launch"));

    let mut typed = parse_session_toml(SESSION).expect("valid baseline");
    typed.render_profile.mode = miso_engine_session::RenderMode::DependencyWaves;
    assert_launch_diagnostic(&compile_session(&typed, caps()).expect_err("typed rejection"));
    assert_launch_diagnostic(&canonical_session_toml(&typed).expect_err("canonical rejection"));
}

/// The token is rejected, not deleted: it still parses as a known token rather than an unknown
/// enum value, which is what keeps the wire and the canonical form lossless.
#[test]
fn dependency_waves_is_still_a_known_token_and_not_an_unknown_enum() {
    let error = parse_session_toml(&source_with_mode("dependency_waves"))
        .expect_err("unsupported at launch");
    assert_eq!(
        error.diagnostics()[0].code,
        DiagnosticCode::RenderModeUnsupportedAtLaunch
    );

    let unknown = parse_session_toml(&source_with_mode("wave_farm"))
        .expect_err("unknown token is a different failure");
    assert_eq!(unknown.diagnostics()[0].code, DiagnosticCode::InvalidEnum);
}
