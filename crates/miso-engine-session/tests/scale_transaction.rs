//! Large-session estimation and transactional compiler-boundary checks.

use miso_engine_session::{
    CompileCaps, DiagnosticCode, StableId, canonical_session_toml, compile_session,
    estimate_session_resources, parse_session_toml,
};

const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.toml");

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

#[test]
fn compiles_65_537_tracks_without_a_product_track_limit() {
    let mut session = parse_session_toml(EXAMPLE).expect("fixture parses");
    let template = session.tracks[0].clone();
    session.tracks.clear();
    session.tracks.reserve(65_537);
    for index in 0..65_537_u32 {
        let mut track = template.clone();
        track.id = StableId::parse(&format!("track-{index}")).expect("generated stable ID");
        session.tracks.push(track);
    }
    session.routes[0].source = miso_engine_session::RouteSource::Track {
        track_id: StableId::parse("track-0").expect("generated route ID"),
        tap: miso_engine_session::SendTap::PostMatrix,
    };
    session.automation[0].target.entity_id =
        StableId::parse("track-0").expect("generated automation entity ID");

    let estimate =
        estimate_session_resources(&session).expect("large estimate uses checked arithmetic");
    assert_eq!(estimate.track_count, 65_537);

    let compiled = compile_session(&session, caps()).expect("adequate caps permit large compile");
    assert_eq!(compiled.resource_estimate().track_count, 65_537);
    assert_eq!(compiled.normalized_model().tracks.len(), 65_537);

    let mut constrained = caps();
    constrained.max_compiled_model_bytes = 1;
    let error =
        compile_session(&session, constrained).expect_err("configured resource cap rejects");
    assert!(
        error
            .diagnostics()
            .iter()
            .all(|diagnostic| diagnostic.code == DiagnosticCode::ResourceLimitExceeded)
    );
}

#[test]
fn failed_compile_does_not_mutate_input_or_construct_a_partial_artifact() {
    let mut session = parse_session_toml(EXAMPLE).expect("fixture parses");
    let before = canonical_session_toml(&session).expect("valid snapshot");
    session.routes[0].destination = miso_engine_session::RouteDestination::OutputInput {
        output_id: StableId::parse("missing-output").expect("stable ID"),
    };
    let invalid_before = session.clone();
    assert!(compile_session(&session, caps()).is_err());
    assert_eq!(
        session, invalid_before,
        "compiler may not mutate caller input"
    );
    assert_ne!(
        before,
        canonical_session_toml(&invalid_before).unwrap_or_default()
    );
}

#[test]
fn preflight_resource_failure_precedes_clone_and_index_construction() {
    let mut session = parse_session_toml(EXAMPLE).expect("fixture parses");
    session.tracks[0].source_id = StableId::parse("missing-source").expect("stable ID");
    let mut constrained = caps();
    constrained.max_compiled_model_bytes = 0;
    let error = compile_session(&session, constrained).expect_err("preflight rejects");
    assert!(
        error
            .diagnostics()
            .iter()
            .all(|item| item.code == DiagnosticCode::ResourceLimitExceeded)
    );
}
