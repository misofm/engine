//! Issue #1243 D1: host preparation counts VCA groups, reports the count, and refuses a session
//! over the host's configured `maximum_vcas` with the shared count refusal.
//!
//! The bound is the host's own resource, never `maximum_tracks` or `maximum_submixes`: every case
//! below holds the track cap at the session's exact track count and the submix cap at zero, so a
//! VCA counted against either would refuse the at-the-cap session too.

use host_core::{
    HostPrepareCaps, HostShapePolicy, PrepareRejection, compile_host_session,
    compiled_session_shape, prepare_host_session,
};
use session::{DualMonoFader, StableId, Vca, canonical_session_json, parse_session_json};

/// Three tracks, each routed to the main output.
const FIXTURE: &str = include_str!("../../../fixtures/session/v1/observation-frame-shape.json");
const TRACKS: u64 = 3;

fn caps(maximum_vcas: u64) -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 1_024,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: TRACKS,
        maximum_submixes: 0,
        maximum_vcas,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 1_000_000_000,
        maximum_source_total_bytes: 100_000_000,
        maximum_source_overhead_bytes: 100_000_000,
        maximum_effect_state_bytes: 1_000_000_000,
        maximum_effect_scratch_bytes: 1_000_000_000,
        maximum_builtin_retained_bytes: 1_000_000_000,
        maximum_named_allocation_bytes: 1_000_000_000,
        maximum_meter_streams: 1,
        maximum_meter_items: 1,
        maximum_meter_bytes: 1,
    }
}

/// The fixture's three tracks plus `vcas` empty, unity VCA groups.
fn session(vcas: usize) -> String {
    let ids: Vec<String> = (0..vcas).map(|index| format!("vca{index}")).collect();
    session_with(&ids)
}

/// The fixture's three tracks plus one empty, unity VCA group per ID.
fn session_with(ids: &[String]) -> String {
    let mut model = parse_session_json(FIXTURE).expect("fixture parses");
    assert_eq!(model.tracks.len() as u64, TRACKS);
    assert!(model.submixes.is_empty());
    for id in ids {
        model.vcas.push(Vca {
            id: StableId::parse(id).expect("stable id"),
            fader: DualMonoFader {
                left_db: 0.0,
                right_db: 0.0,
                left_mute: false,
                right_mute: false,
            },
            members: Vec::new(),
        });
    }
    canonical_session_json(&model).expect("canonical session")
}

/// Test value: turns red if VCAs go uncounted (bounded only by byte budgets), are counted against
/// `maximum_tracks` (the track cap here equals the track count, and the cap of four exceeds it) or
/// `maximum_submixes` (zero here), or are missing from the report.
#[test]
fn vcas_are_counted_capped_and_reported_apart_from_tracks_and_submixes() {
    for cap in [1_u64, 4] {
        let document = session(cap as usize);
        let (_, prepared) = prepare_host_session(&document, &caps(cap)).unwrap_or_else(|failure| {
            panic!(
                "{cap} VCAs at the cap: {}",
                String::from_utf8_lossy(failure.as_bytes())
            )
        });
        assert_eq!(prepared.report.vca_count, cap);
        assert_eq!(prepared.report.track_count, TRACKS);
        assert_eq!(prepared.report.submix_count, 0);

        let over = session(cap as usize + 1);
        let failure = prepare_host_session(&over, &caps(cap))
            .map(|_| ())
            .expect_err("one VCA over the cap");
        assert_eq!(failure.kind(), PrepareRejection::Resource);
        assert_eq!(failure.as_bytes(), b"host.resource.count\t$\n");
    }

    // A zero cap is a real bound in host-core (the C ABI's zero rule lives in capi): no VCA.
    let failure = prepare_host_session(&session(1), &caps(0))
        .map(|_| ())
        .expect_err("a VCA over a zero cap");
    assert_eq!(failure.as_bytes(), b"host.resource.count\t$\n");
    let (_, prepared) = prepare_host_session(&session(0), &caps(0)).expect("no VCA, zero cap");
    assert_eq!(prepared.report.vca_count, 0);
}

/// Issue #1246 gate 3 (D2): `longest_vca_id_bytes` is the longest VCA ID, here one between two
/// shorter ones in canonical order and longer than every source, track and route ID, which keep
/// their own measures.
///
/// Test value: red if the shape leaves VCA IDs unmeasured, or measures only the first or the last
/// VCA, so a host sizing its ID staging from it would undersize it for a VCA ID it copies.
#[test]
fn the_session_shape_measures_the_longest_vca_id() {
    let long = "mm-the-drums-and-every-room-mic-under-one-hand";
    let ids = ["a".to_owned(), long.to_owned(), "zz".to_owned()];
    let compiled = compile_host_session(&session_with(&ids), &caps(3)).expect("session compiles");
    let shape = compiled_session_shape(&compiled).expect("shape");
    assert_eq!(shape.longest_vca_id_bytes, long.len() as u64);
    assert_eq!(shape.longest_source_id_bytes, "fixture-source".len() as u64);
    assert_eq!(shape.longest_track_id_bytes, "t0".len() as u64);
    assert_eq!(shape.longest_route_id_bytes, "t0-main".len() as u64);

    let none = compile_host_session(&session(0), &caps(0)).expect("session compiles");
    assert_eq!(
        compiled_session_shape(&none)
            .expect("shape")
            .longest_vca_id_bytes,
        0
    );
}
