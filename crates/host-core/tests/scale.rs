//! Issue #1079: host preparation, as the C ABI's preparer runs it (live controls attached), holds a
//! session above the former 65,536-track boundary.
//!
//! Release-only nightly row: the prepare compiles, lays out, binds and attaches live controls to
//! 65,537 strips.

use host_core::{
    HostLiveControlRequest, HostLiveLanes, HostPrepareCaps, HostShapePolicy, LiveEffectAddress,
    LiveValuesSuperseded, compile_host_session, launch_maximum_observation_taps,
    prepare_host_runtime_with_live_lanes_and_spectrum, strip_position,
};
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

/// With every cap open, the preparation `capi`'s `CapiPreparer::prepare` runs (an observation
/// slot for every declared tap, the control plane's lanes with the effect lanes on, no spectrum,
/// no successor) accepts the session and attaches one strip producer per track; the strip at
/// index 65,536 (the last: tracks are in ID order, so it is `track-9999`) is addressable, and so
/// is its console EQ's producer.
#[test]
#[ignore = "release-mode scale gate; runs nightly"]
fn prepares_65_537_live_attached_tracks_as_capi_does() {
    let compiled = compile_host_session(&scale_document(), &caps())
        .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
    let live = HostLiveControlRequest {
        observation_taps: launch_maximum_observation_taps(),
        ..HostLiveControlRequest::default()
    };
    let lanes: HostLiveLanes = controls::live_lanes(compiled.normalized_model()).into();
    let lanes = HostLiveLanes {
        effects: true,
        ..lanes
    };
    let (_prepared, mut handles, spectrum) = prepare_host_runtime_with_live_lanes_and_spectrum(
        &compiled,
        &caps(),
        &live,
        lanes,
        None,
        None,
        &LiveValuesSuperseded::new(),
    )
    .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
    assert!(spectrum.is_none());
    assert_eq!(handles.track_count, TRACKS as usize);
    assert_eq!(handles.strip_controls.len(), TRACKS as usize);
    let last = TRACKS as usize - 1;
    assert_eq!(
        strip_position(
            handles.strips.len(),
            handles.track_count,
            |index| &*handles.strips[index],
            "track-9999",
        ),
        Some(last),
        "the last track in ID order is at index 65,536"
    );
    assert_eq!(&*handles.strip_controls[last].track_id, "track-9999");
    assert!(
        handles
            .effect_control_mut("track-9999", LiveEffectAddress::console(0))
            .is_some(),
        "the console EQ's producer on the strip at index 65,536 resolves"
    );
}
