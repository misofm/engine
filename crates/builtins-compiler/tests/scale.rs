//! Builtin preparation scale gate above the former fixed-track ceiling.

use builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use session::{CompileCaps, RouteSource, SendTap, StableId, compile_session, parse_session_toml};

const SESSION: &str = include_str!("../../../fixtures/session/v1/canonical.toml");

fn session_caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

fn builtin_caps() -> BuiltinCompileCaps {
    BuiltinCompileCaps {
        maximum_total_state_bytes: u64::MAX,
        maximum_total_retained_payload_bytes: u64::MAX,
        maximum_total_meter_items: u64::MAX,
        maximum_total_meter_bytes: u64::MAX,
        maximum_single_allocation_bytes: u64::MAX,
        maximum_meter_streams: u64::MAX,
        maximum_period_frames: u32::MAX,
        maximum_peak_hold_frames: u32::MAX,
        maximum_smoothing_samples: u32::MAX,
    }
}

#[test]
fn prepares_65_537_tracks_or_rejects_only_the_configured_resource() {
    let mut model = parse_session_toml(SESSION).expect("fixture");
    let mut template = model.tracks[0].clone();
    template.simd1.effects.clear();
    template.dynamic.effects.clear();
    template.simd2.effects.clear();
    model.automation.clear();
    model.tracks.clear();
    model.tracks.reserve(65_537);
    for index in 0..65_537_u32 {
        let mut track = template.clone();
        track.id = StableId::parse(&format!("track-{index}")).expect("generated ID");
        model.tracks.push(track);
    }
    model.routes[0].source = RouteSource::Track {
        track_id: StableId::parse("track-0").expect("route track"),
        tap: SendTap::PostMatrix,
    };
    let session = compile_session(&model, session_caps()).expect("scale session");

    let prepared = prepare_session_builtins(&session, &[], builtin_caps()).expect("scale builtins");
    assert_eq!(prepared.processor_count(), 65_537 * 3);
    assert_eq!(prepared.tail_count(), 65_537);
    assert_eq!(prepared.resource_report().meter_items, 0);

    let mut constrained = builtin_caps();
    constrained.maximum_total_state_bytes = prepared
        .resource_report()
        .engine_owned_processor_payload_bytes
        .saturating_sub(1);
    let Err(error) = prepare_session_builtins(&session, &[], constrained) else {
        panic!("configured builtin resource cap must reject");
    };
    assert!(
        error
            .0
            .iter()
            .all(|diagnostic| diagnostic.code == "builtin.resource.limit")
    );
}
