//! Launch sample-rate policy coverage.

use session::{
    CompileCaps, DiagnosticCode, canonical_session_json, compile_session, parse_session_json,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/canonical.json");
const MESSAGE: &str = "launch sample_rate_hz must be one of 44100, 48000, 88200, or 96000 Hz";

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

fn source_with_rate(rate: u32) -> String {
    SESSION.replacen(
        "\"sample_rate_hz\": 48000",
        &format!("\"sample_rate_hz\": {rate}"),
        1,
    )
}

fn assert_launch_diagnostic(diagnostics: &session::DiagnosticSet) {
    assert_eq!(diagnostics.diagnostics().len(), 1);
    let diagnostic = &diagnostics.diagnostics()[0];
    assert_eq!(
        diagnostic.code,
        DiagnosticCode::SampleRateUnsupportedAtLaunch
    );
    assert_eq!(diagnostic.path.to_string(), "$.sample_rate_hz");
    assert_eq!(diagnostic.message, MESSAGE);
}

#[test]
fn launch_rates_parse_compile_and_canonicalize() {
    for rate in [44_100, 48_000, 88_200, 96_000] {
        let model = parse_session_json(&source_with_rate(rate)).expect("launch parse");
        let compiled = compile_session(&model, caps()).expect("launch compile");
        assert_eq!(compiled.normalized_model().sample_rate_hz, rate);
        assert_eq!(
            canonical_session_json(compiled.normalized_model()).expect("canonical"),
            compiled.canonical_json()
        );
    }
}

#[test]
fn extended_and_unrelated_engine_rates_reject_with_one_stable_diagnostic() {
    for rate in [176_400, 192_000, 352_800, 384_000, 0, 32_000, 192_001] {
        let parsed = parse_session_json(&source_with_rate(rate));
        assert_launch_diagnostic(&parsed.expect_err("unsupported at launch"));

        let mut typed = parse_session_json(SESSION).expect("valid baseline");
        typed.sample_rate_hz = rate;
        assert_launch_diagnostic(&compile_session(&typed, caps()).expect_err("typed rejection"));
        assert_launch_diagnostic(&canonical_session_json(&typed).expect_err("canonical rejection"));
    }
}
