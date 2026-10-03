//! Issue #1206 D1: host preparation counts submix strips, reports the count, and refuses a session
//! over the host's configured `maximum_submixes` with the shared count refusal.
//!
//! The bound is the host's own resource, never `maximum_tracks`: every case below holds the track
//! cap at the session's exact track count, so a submix counted against tracks would refuse the
//! at-the-cap session too.

use host_core::{HostPrepareCaps, HostShapePolicy, PrepareRejection, prepare_host_session};
use session::{StableId, Submix, canonical_session_json, parse_session_json};

/// Three tracks, each routed to the main output.
const FIXTURE: &str = include_str!("../../../fixtures/session/v1/observation-frame-shape.json");
const TRACKS: u64 = 3;

fn caps(maximum_submixes: u64) -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 1_024,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: TRACKS,
        maximum_submixes,
        maximum_vcas: 100,
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

/// The fixture's three tracks plus `submixes` transparent, unrouted submix strips.
fn session(submixes: usize) -> String {
    let mut model = parse_session_json(FIXTURE).expect("fixture parses");
    assert_eq!(model.tracks.len() as u64, TRACKS);
    for index in 0..submixes {
        let id = StableId::parse(&format!("bus{index}")).expect("stable id");
        model.submixes.push(Submix::unity(id, &model.console));
    }
    canonical_session_json(&model).expect("canonical session")
}

/// Test value: turns red if submixes go uncounted (bounded only by byte budgets), are counted
/// against `maximum_tracks` (the track cap here equals the track count, so any submix would
/// refuse), or are missing from the report.
#[test]
fn submixes_are_counted_capped_and_reported_apart_from_tracks() {
    for cap in [1_u64, 4] {
        let document = session(cap as usize);
        let (_, prepared) = prepare_host_session(&document, &caps(cap)).unwrap_or_else(|failure| {
            panic!(
                "{cap} submixes at the cap: {}",
                String::from_utf8_lossy(failure.as_bytes())
            )
        });
        assert_eq!(prepared.report.submix_count, cap);
        assert_eq!(prepared.report.track_count, TRACKS);

        let over = session(cap as usize + 1);
        let failure = prepare_host_session(&over, &caps(cap))
            .map(|_| ())
            .expect_err("one submix over the cap");
        assert_eq!(failure.kind(), PrepareRejection::Resource);
        assert_eq!(failure.as_bytes(), b"host.resource.count\t$\n");
    }

    // A zero cap is a real bound in host-core (the C ABI's zero rule lives in capi): no submix.
    let failure = prepare_host_session(&session(1), &caps(0))
        .map(|_| ())
        .expect_err("a submix over a zero cap");
    assert_eq!(failure.as_bytes(), b"host.resource.count\t$\n");
    let (_, prepared) = prepare_host_session(&session(0), &caps(0)).expect("no submix, zero cap");
    assert_eq!(prepared.report.submix_count, 0);
}
