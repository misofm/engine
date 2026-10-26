//! Issue #1733 item 2: host preparation counts every console slot and every insert of every strip,
//! reports the count, and refuses a session over the host's configured `maximum_effects` with the
//! shared count refusal.

use host_core::{HostPrepareCaps, HostShapePolicy, PrepareRejection, prepare_host_session};
use session::{
    ConsoleEntry, ConsoleLinkMode, ConsoleSlot, EffectIdentity, EffectQuality, LinkMode, StableId,
    canonical_session_json, parse_session_json,
};

/// Three tracks, each with one compressor insert and routed to the main output.
const FIXTURE: &str = include_str!("../../../fixtures/session/v1/observation-frame-shape.json");

/// Three strips carry the one post-insert console slot (3), the fixture's three inserts (3) and a
/// second insert on the first track (1).
const EXPECTED: u64 = 3 + 3 + 1;

fn caps(maximum_effects: u64) -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 1_024,
        maximum_tracks: 100,
        maximum_submixes: 100,
        maximum_vcas: 100,
        maximum_vca_reach_pairs: u64::MAX,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects,
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

fn id(text: &str) -> StableId {
    StableId::parse(text).expect("stable id")
}

fn session() -> String {
    let mut model = parse_session_json(FIXTURE).expect("fixture parses");
    assert_eq!(model.tracks.len(), 3);
    assert!(
        model
            .tracks
            .iter()
            .all(|track| track.inserts.effects.len() == 1)
    );
    model.console.post_insert = vec![ConsoleSlot {
        slot: id("post-clip"),
        identity: EffectIdentity::Native {
            effect_id: id("miso.soft-clip"),
        },
        quality: EffectQuality::Normal,
        link_mode: LinkMode::DualMono,
    }];
    for track in &mut model.tracks {
        track.console.push(ConsoleEntry {
            slot: id("post-clip"),
            bypass: false,
            link_mode: ConsoleLinkMode::Slot,
            params: Vec::new(),
        });
    }
    let mut second = model.tracks[0].inserts.effects[0].clone();
    second.id = id("comp-b");
    model.tracks[0].inserts.effects.push(second);
    canonical_session_json(&model).expect("canonical session")
}

/// Test value: turns red if console-slot or insert effects go uncounted or unreported (a count of
/// zero passes every other caps test, which all set a generous `maximum_effects`).
#[test]
fn console_slots_and_inserts_are_counted_capped_and_reported() {
    let document = session();
    let (_, prepared) =
        prepare_host_session(&document, &caps(EXPECTED)).unwrap_or_else(|failure| {
            panic!(
                "at the cap: {}",
                String::from_utf8_lossy(failure.as_bytes())
            )
        });
    assert_eq!(prepared.report.effect_count, EXPECTED);

    let failure = prepare_host_session(&document, &caps(EXPECTED - 1))
        .map(|_| ())
        .expect_err("one effect over the cap");
    assert_eq!(failure.kind(), PrepareRejection::Resource);
    assert_eq!(failure.as_bytes(), b"host.resource.count\t$\n");
}
