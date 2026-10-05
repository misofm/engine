//! Issue #1237 gate 1: a route's `gain_db` is bounded to `[-144, 24]` dB and each
//! `channel_matrix` coefficient to `[-1, 1]`, on the typed path (`compile_session`) and the text
//! path (`parse_session_json`), with the refusal at the value's own path. A non-finite value keeps
//! `numeric.non_finite`.

use session::{
    CompileCaps, DiagnosticCode, SessionModel, canonical_session_json, compile_session,
    parse_session_json,
};

const CANONICAL: &str = include_str!("../../../fixtures/session/v1/canonical.json");

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

/// Writes one route value into a session model.
type Setter = fn(&mut SessionModel, f32);

/// The route fields under test: their schema path below `$.routes[0]`, and a setter.
const FIELDS: [(&str, Setter); 5] = [
    ("gain_db", |s, v| s.routes[0].gain_db = v),
    ("channel_matrix.ll", |s, v| {
        s.routes[0].channel_matrix.ll = v
    }),
    ("channel_matrix.lr", |s, v| {
        s.routes[0].channel_matrix.lr = v
    }),
    ("channel_matrix.rl", |s, v| {
        s.routes[0].channel_matrix.rl = v
    }),
    ("channel_matrix.rr", |s, v| {
        s.routes[0].channel_matrix.rr = v
    }),
];

/// `(accepted, refused)` boundary values for `field`.
fn boundaries(field: &str) -> ([f32; 2], [f32; 2]) {
    if field == "gain_db" {
        ([24.0, -144.0], [24.5, -144.5])
    } else {
        ([1.0, -1.0], [1.5, -1.5])
    }
}

/// The codes the typed and the text paths report at `path`, or `None` when both accept.
fn outcome(model: &SessionModel, path: &str) -> Option<DiagnosticCode> {
    let typed = compile_session(model, caps());
    let text = canonical_session_json(model).map(|json| parse_session_json(&json));
    let at = |set: &session::DiagnosticSet| {
        let codes: Vec<DiagnosticCode> = set
            .diagnostics()
            .iter()
            .filter(|item| item.path.to_string() == path)
            .map(|item| item.code)
            .collect();
        assert_eq!(codes.len(), 1, "exactly one diagnostic at {path}: {set}");
        codes[0]
    };
    let typed = typed.err().map(|set| at(&set));
    // The canonical writer refuses a non-finite value itself; only finite values reach the text
    // parser.
    if let Ok(text) = text {
        assert_eq!(
            text.err().map(|set| at(&set)),
            typed,
            "text vs typed at {path}"
        );
    }
    typed
}

/// Test value: red if `validate_routes` checks a route's gain or any one coefficient with
/// `validate_finite` alone (the out-of-range values then compile), bounds one with the wrong limit
/// or an exclusive comparison (a boundary refused, or a half-step outside accepted), reports it at
/// another path, or lets the range check pre-empt `numeric.non_finite` for NaN or infinity.
#[test]
fn route_gain_and_matrix_values_are_bounded_at_their_own_path() {
    let base = parse_session_json(CANONICAL).expect("fixture parses");
    for (field, set) in FIELDS {
        let path = format!("$.routes[0].{field}");
        let (accepted, refused) = boundaries(field);
        for value in accepted {
            let mut model = base.clone();
            set(&mut model, value);
            assert_eq!(
                outcome(&model, &path),
                None,
                "{path} = {value} is in domain"
            );
        }
        for value in refused {
            let mut model = base.clone();
            set(&mut model, value);
            assert_eq!(
                outcome(&model, &path),
                Some(DiagnosticCode::NumericOutOfSchemaRange),
                "{path} = {value}"
            );
        }
        for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let mut model = base.clone();
            set(&mut model, value);
            assert_eq!(
                outcome(&model, &path),
                Some(DiagnosticCode::NumericNonFinite),
                "{path} = {value}"
            );
        }
    }
}
