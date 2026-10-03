//! Issue #1207 gate 2: the live-control handles list every strip, tracks first.
//!
//! `HostLiveControlHandles::strips` is the addressing authority: the tracks in canonical ID order,
//! then the submixes in canonical ID order. The builtin control producers (`strip_controls`) are
//! parallel to every strip (issue #1211 D1), the default meters stay parallel to the track prefix,
//! and a submix's effects get a live channel and an observation handle exactly as a track's do (D5
//! removed the K1 interim).

use core::num::{NonZeroU32, NonZeroUsize};

use builtins::MeterTap;
use host_core::{
    HostLiveControlRequest, HostPrepareCaps, HostShapePolicy,
    prepare_host_session_with_live_controls,
};
use session::{
    RouteDestination, RouteSource, SendTap, StableId, Submix, canonical_session_json,
    parse_session_json,
};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/observation-frame-shape.json");
const QUANTUM: u32 = 128;

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 1_024,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 100,
        maximum_submixes: 100,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 100_000_000,
        maximum_source_total_bytes: 10_000_000,
        maximum_source_overhead_bytes: 10_000_000,
        maximum_effect_state_bytes: 100_000_000,
        maximum_effect_scratch_bytes: 100_000_000,
        maximum_builtin_retained_bytes: 100_000_000,
        maximum_named_allocation_bytes: 100_000_000,
        maximum_meter_streams: 64,
        maximum_meter_items: 1 << 16,
        maximum_meter_bytes: 1 << 24,
    }
}

/// The three-track observation fixture (`t0` compressor, `t1` EQ, `t2` gate) plus two submixes,
/// declared out of order: `zzz-bus` sorts after every track and `aaa-bus` before every one, so
/// a strip list that merged the two segments by sorting would put `aaa-bus` first. Each bus
/// carries a compressor insert; `t0` feeds `zzz-bus`, `t1` feeds `aaa-bus`, and both buses feed
/// the output.
fn document() -> String {
    let mut model = parse_session_json(FIXTURE).expect("observation fixture parses");
    let compressor = model.tracks[0].inserts.effects[0].clone();
    for (bus, feeder) in [("zzz-bus", 0_usize), ("aaa-bus", 1)] {
        let id = StableId::parse(bus).expect("bus id");
        let mut submix = Submix::unity(id.clone(), &model.console);
        let mut insert = compressor.clone();
        insert.id = StableId::parse("bus-comp").expect("insert id");
        submix.inserts.effects.push(insert);
        model.submixes.push(submix);
        let mut out = model.routes[feeder].clone();
        model.routes[feeder].destination = RouteDestination::SubmixInput {
            submix_id: id.clone(),
        };
        out.id = StableId::parse(&format!("{bus}-main")).expect("route id");
        out.source = RouteSource::Submix {
            submix_id: id,
            tap: SendTap::PostPan,
        };
        model.routes.push(out);
    }
    canonical_session_json(&model).expect("canonical bus session")
}

/// Red if a submix lands inside the track prefix (or the strips stop being tracks-then-submixes),
/// if the builtin controls stop being parallel to the strips (issue #1211 D1), if the default
/// meters start to cover buses, or if a bus effect stays without a live channel or an observation
/// handle.
#[test]
fn handles_list_tracks_then_submixes_and_file_bus_effects() {
    let (_, _, handles) = prepare_host_session_with_live_controls(
        &document(),
        &caps(),
        &HostLiveControlRequest {
            control_queue_depth: Some(NonZeroUsize::new(16).expect("depth")),
            meter_period_frames: Some(NonZeroU32::new(QUANTUM).expect("period")),
            meter_queue_depth: NonZeroUsize::new(16).expect("meter depth"),
            meter_tap: MeterTap::PostMatrix,
            observation_taps: 1,
            master_track: None,
        },
    )
    .unwrap_or_else(|failure| panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes())));

    let strips: Vec<&str> = handles.strips.iter().map(AsRef::as_ref).collect();
    assert_eq!(strips, ["t0", "t1", "t2", "aaa-bus", "zzz-bus"]);
    assert_eq!(handles.track_count, 3);

    let tracks = &strips[..handles.track_count];
    let controls: Vec<&str> = handles
        .strip_controls
        .iter()
        .map(|control| control.track_id.as_ref())
        .collect();
    assert_eq!(
        controls, strips,
        "one builtin channel per strip, in strip order"
    );
    let meters: Vec<&str> = handles
        .meters
        .iter()
        .map(|meter| meter.track_id.as_ref())
        .collect();
    assert_eq!(meters, tracks, "one default meter per track, none per bus");

    let mut effects: Vec<(&str, &str)> = handles
        .effect_controls
        .iter()
        .map(|producer| (producer.track_id.as_ref(), producer.effect_id.as_ref()))
        .collect();
    effects.sort_unstable();
    assert_eq!(
        effects,
        [
            ("aaa-bus", "bus-comp"),
            ("t0", "comp"),
            ("t1", "eq"),
            ("t2", "gate"),
            ("zzz-bus", "bus-comp"),
        ],
        "every prepared effect, the buses' included, has a live channel"
    );
    let mut observed: Vec<(&str, &str)> = handles
        .effect_observations
        .iter()
        .map(|handle| (handle.track_id.as_ref(), handle.effect_id.as_ref()))
        .collect();
    observed.sort_unstable();
    assert_eq!(
        observed,
        [
            ("aaa-bus", "bus-comp"),
            ("t0", "comp"),
            ("t2", "gate"),
            ("zzz-bus", "bus-comp"),
        ],
        "every tapped effect, the buses' included, has an observation handle (the EQ has no tap)"
    );
}
