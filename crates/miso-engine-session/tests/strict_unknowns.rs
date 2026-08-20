//! Strict unknown-key rejection at every nested V1 table family.

use miso_engine_session::{DiagnosticCode, parse_session_toml};

const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.toml");

#[test]
fn every_nested_table_family_rejects_an_exact_unknown_leaf() {
    let cases = [
        ("session_id =", "unknown = 0\nsession_id =", "$.unknown"),
        (
            "render_profile = {",
            "render_profile = { unknown = 0,",
            "$.render_profile.unknown",
        ),
        (
            "output_profile = {",
            "output_profile = { unknown = 0,",
            "$.output_profile.unknown",
        ),
        ("limits = {", "limits = { unknown = 0,", "$.limits.unknown"),
        (
            "{ id = \"voice\", sample_rate_hz",
            "{ unknown = 0, id = \"voice\", sample_rate_hz",
            "$.sources[0].unknown",
        ),
        (
            "content = {",
            "content = { unknown = 0,",
            "$.sources[0].content.unknown",
        ),
        (
            "mapping = {",
            "mapping = { unknown = 0,",
            "$.sources[0].mapping.unknown",
        ),
        (
            "region = {",
            "region = { unknown = 0,",
            "$.sources[0].mapping.region.unknown",
        ),
        (
            "{ id = \"vocal\", source_id",
            "{ unknown = 0, id = \"vocal\", source_id",
            "$.tracks[0].unknown",
        ),
        (
            "builtins = {",
            "builtins = { unknown = 0,",
            "$.tracks[0].builtins.unknown",
        ),
        (
            "left = { polarity_invert",
            "left = { unknown = 0, polarity_invert",
            "$.tracks[0].builtins.left.unknown",
        ),
        (
            "right = { polarity_invert",
            "right = { unknown = 0, polarity_invert",
            "$.tracks[0].builtins.right.unknown",
        ),
        (
            "simd1 = {",
            "simd1 = { unknown = 0,",
            "$.tracks[0].simd1.unknown",
        ),
        (
            "[{ id = \"eq\", identity",
            "[{ unknown = 0, id = \"eq\", identity",
            "$.tracks[0].dynamic.effects[0].unknown",
        ),
        (
            "identity = {",
            "identity = { unknown = 0,",
            "$.tracks[0].dynamic.effects[0].identity.unknown",
        ),
        (
            "params = [{",
            "params = [{ unknown = 0,",
            "$.tracks[0].dynamic.effects[0].params[0].unknown",
        ),
        (
            "sidechain = {",
            "sidechain = { unknown = 0,",
            "$.tracks[0].dynamic.effects[0].sidechain.unknown",
        ),
        (
            "fader = {",
            "fader = { unknown = 0,",
            "$.tracks[0].fader.unknown",
        ),
        ("pan = {", "pan = { unknown = 0,", "$.tracks[0].pan.unknown"),
        (
            "{ id = \"main-out\" },",
            "{ unknown = 0, id = \"main-out\" },",
            "$.outputs[0].unknown",
        ),
        (
            "{ id = \"to-main\", source",
            "{ unknown = 0, id = \"to-main\", source",
            "$.routes[0].unknown",
        ),
        (
            "source = { kind",
            "source = { unknown = 0, kind",
            "$.routes[0].source.unknown",
        ),
        (
            "destination = { kind",
            "destination = { unknown = 0, kind",
            "$.routes[0].destination.unknown",
        ),
        (
            "channel_matrix = {",
            "channel_matrix = { unknown = 0,",
            "$.routes[0].channel_matrix.unknown",
        ),
        (
            "{ id = \"eq-gain\", target",
            "{ unknown = 0, id = \"eq-gain\", target",
            "$.automation[0].unknown",
        ),
        (
            "target = {",
            "target = { unknown = 0,",
            "$.automation[0].target.unknown",
        ),
        (
            "segments = [{",
            "segments = [{ unknown = 0,",
            "$.automation[0].segments[0].unknown",
        ),
    ];

    for (needle, replacement, expected_path) in cases {
        let source = EXAMPLE.replacen(needle, replacement, 1);
        assert_ne!(source, EXAMPLE, "fixture replacement must apply");
        let error = parse_session_toml(&source).expect_err("unknown field must reject");
        assert!(
            error.diagnostics().iter().any(|diagnostic| {
                diagnostic.code == DiagnosticCode::UnknownField
                    && diagnostic.path.to_string() == expected_path
            }),
            "missing schema.unknown_field at {expected_path}: {error}"
        );
    }
}
