//! Issue #1079: host preparation, the facade the C ABI's prepare calls, holds a session above the
//! former 65,536-track boundary.
//!
//! Release-only nightly row: the document is about 94 MB and the prepare compiles, lays out and
//! binds 65,537 strips.

use host_core::{HostPrepareCaps, HostShapePolicy, PrepareRejection, prepare_host_session};
use session::{SessionModel, StableId, canonical_session_json, parse_session_json};

const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");

/// One past the former 65,536 boundary.
const TRACKS: u32 = 65_537;

/// Every count and byte cap open, so only a fixed limit can refuse.
fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 1_024,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: u64::MAX,
        maximum_submixes: u64::MAX,
        maximum_vcas: u64::MAX,
        maximum_vca_reach_pairs: u64::MAX,
        maximum_sources: u64::MAX,
        maximum_routes: u64::MAX,
        maximum_effects: u64::MAX,
        maximum_graph_session_plus_plan_bytes: u64::MAX,
        maximum_source_total_bytes: u64::MAX,
        maximum_source_overhead_bytes: u64::MAX,
        maximum_effect_state_bytes: u64::MAX,
        maximum_effect_scratch_bytes: u64::MAX,
        maximum_builtin_retained_bytes: u64::MAX,
        maximum_named_allocation_bytes: u64::MAX,
        maximum_meter_streams: u64::MAX,
        maximum_meter_items: u64::MAX,
        maximum_meter_bytes: u64::MAX,
    }
}

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("generated stable ID")
}

/// The fixture's first track, with the console's EQ slot, [`TRACKS`] times; one route, out of
/// `track-0`.
fn scale_document() -> String {
    let mut model: SessionModel = parse_session_json(EXAMPLE).expect("fixture parses");
    let template = model.tracks[0].clone();
    model.tracks = (0..TRACKS)
        .map(|index| {
            let mut track = template.clone();
            track.id = id(&format!("track-{index}"));
            track
        })
        .collect();
    model.routes.truncate(1);
    model.routes[0].source = session::RouteSource::Track {
        track_id: id("track-0"),
        tap: session::SendTap::PostPan,
    };
    canonical_session_json(&model).expect("the scale model is canonical")
}

/// With every cap open, preparation accepts the session, or refuses it only as a resource.
#[test]
#[ignore = "release-mode scale gate; runs nightly"]
fn prepares_65_537_tracks_or_refuses_only_a_resource() {
    match prepare_host_session(&scale_document(), &caps()) {
        Ok((compiled, _prepared)) => {
            assert_eq!(compiled.normalized_model().tracks.len(), TRACKS as usize);
        }
        Err(failure) => assert_eq!(
            failure.kind(),
            PrepareRejection::Resource,
            "{}",
            String::from_utf8_lossy(failure.as_bytes())
        ),
    }
}
