//! Strict unknown-key rejection at every nested Session V1 object family.

use session::{DiagnosticCode, parse_session_json};

const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.json");

#[test]
fn every_nested_object_family_rejects_an_exact_unknown_leaf() {
    let cases = [
        ("", "$.unknown"),
        ("/render_profile", "$.render_profile.unknown"),
        ("/output_profile", "$.output_profile.unknown"),
        ("/sources/0", "$.sources[0].unknown"),
        ("/tracks/0", "$.tracks[0].unknown"),
        ("/tracks/0/builtins", "$.tracks[0].builtins.unknown"),
        (
            "/tracks/0/builtins/left",
            "$.tracks[0].builtins.left.unknown",
        ),
        (
            "/tracks/0/builtins/right",
            "$.tracks[0].builtins.right.unknown",
        ),
        ("/tracks/0/simd1", "$.tracks[0].simd1.unknown"),
        (
            "/tracks/0/dynamic/effects/0",
            "$.tracks[0].dynamic.effects[0].unknown",
        ),
        (
            "/tracks/0/dynamic/effects/0/identity",
            "$.tracks[0].dynamic.effects[0].identity.unknown",
        ),
        (
            "/tracks/0/dynamic/effects/0/params/0",
            "$.tracks[0].dynamic.effects[0].params[0].unknown",
        ),
        (
            "/tracks/0/dynamic/effects/0/sidechain",
            "$.tracks[0].dynamic.effects[0].sidechain.unknown",
        ),
        ("/tracks/0/fader", "$.tracks[0].fader.unknown"),
        ("/tracks/0/pan", "$.tracks[0].pan.unknown"),
        ("/outputs/0", "$.outputs[0].unknown"),
        ("/routes/0", "$.routes[0].unknown"),
        ("/routes/0/source", "$.routes[0].source.unknown"),
        ("/routes/0/destination", "$.routes[0].destination.unknown"),
        (
            "/routes/0/channel_matrix",
            "$.routes[0].channel_matrix.unknown",
        ),
        ("/automation/0", "$.automation[0].unknown"),
        ("/automation/0/target", "$.automation[0].target.unknown"),
        (
            "/automation/0/segments/0",
            "$.automation[0].segments[0].unknown",
        ),
    ];
    for (pointer, expected_path) in cases {
        let mut value: serde_json::Value = serde_json::from_str(EXAMPLE).expect("fixture JSON");
        value
            .pointer_mut(pointer)
            .expect("pointer")
            .as_object_mut()
            .expect("object")
            .insert("unknown".to_owned(), 0.into());
        let source = serde_json::to_string(&value).expect("JSON");
        let error = parse_session_json(&source).expect_err("unknown field");
        assert!(
            error
                .diagnostics()
                .iter()
                .any(|d| d.code == DiagnosticCode::UnknownField
                    && d.path.to_string() == expected_path),
            "missing unknown at {expected_path}: {error}"
        );
    }
}
