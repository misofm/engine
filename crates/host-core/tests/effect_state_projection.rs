//! Issue #2194 gate A: host preparation refuses a session whose effect state exceeds the cap
//! before it allocates that state.
//!
//! A delay instance retains about 768 KB at 48 kHz while its document entry is under 230 bytes,
//! so 512 tracks with four delays each (about 1.4 MB of JSON) declare about 1.5 GB of effect state.
//! Before #2194 the effect compiler prepared every instance and host-core compared the sum with
//! the cap afterwards: the refusal came after the thread's heap peaked at that 1.5 GB.

use bench_support::alloc as audited_allocator;
use host_core::{HostPrepareCaps, HostShapePolicy, PrepareRejection, prepare_host_session};
use session::{
    Effect, EffectIdentity, EffectQuality, LinkMode, RouteSource, SendTap, SidechainDeclaration,
    StableId, canonical_session_json, parse_session_json,
};

/// The parse transient the boot already admits per document byte
/// (`host_web::PARSE_TRANSIENT_MULTIPLIER`; this crate does not depend on the browser host).
const PARSE_TRANSIENT_MULTIPLIER: usize = 20;

fn id(text: &str) -> StableId {
    StableId::parse(text).expect("stable id")
}

/// `tracks` tracks on the nine-track fixture's first source, each with `delays` default delay
/// inserts and a post-pan route to the main output.
fn delay_document(tracks: usize, delays: usize) -> String {
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/parametric-eq-nine-track.json"
    ))
    .expect("seed fixture parses");
    let mut track = model.tracks[1].clone();
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    track.console.clear();
    track.inserts.effects = (0..delays)
        .map(|index| Effect {
            id: id(&format!("delay-{index}")),
            identity: EffectIdentity::Native {
                effect_id: id("miso.delay"),
            },
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            params: Vec::new(),
            sidechain: SidechainDeclaration::None,
        })
        .collect();
    let route = model.routes[0].clone();
    model.tracks.clear();
    model.routes.clear();
    model.automation.clear();
    for index in 0..tracks {
        let track_id = id(&format!("track-{index:03}"));
        let mut next_track = track.clone();
        next_track.id = track_id.clone();
        model.tracks.push(next_track);
        let mut next_route = route.clone();
        next_route.id = id(&format!("route-{index:03}"));
        next_route.source = RouteSource::Track {
            track_id,
            tap: SendTap::PostPan,
        };
        model.routes.push(next_route);
    }
    canonical_session_json(&model).expect("canonical session")
}

fn caps(maximum_effect_state_bytes: u64) -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 1_024,
        maximum_tracks: u64::MAX,
        maximum_submixes: u64::MAX,
        maximum_vcas: 64,
        maximum_vca_reach_pairs: u64::MAX,
        maximum_sources: u64::MAX,
        maximum_routes: u64::MAX,
        maximum_effects: u64::MAX,
        maximum_graph_session_plus_plan_bytes: u64::MAX,
        maximum_source_total_bytes: u64::MAX,
        maximum_source_overhead_bytes: u64::MAX,
        maximum_effect_state_bytes,
        maximum_effect_scratch_bytes: u64::MAX,
        maximum_builtin_retained_bytes: u64::MAX,
        maximum_named_allocation_bytes: u64::MAX,
        maximum_meter_streams: u64::MAX,
        maximum_meter_items: u64::MAX,
        maximum_meter_bytes: u64::MAX,
    }
}

/// Test value: red when the effect cap is checked only after the instances are prepared (the
/// thread then peaks at about 1.5 GB, 75 times the document's parse transient), or when the
/// refusal stops naming the projected bytes, the cap and the instances. The other effect-cap tests
/// only count instances or check one instance against its own cap.
#[test]
fn effect_state_over_the_cap_is_refused_before_it_is_allocated() {
    audited_allocator::assert_installed();
    let document = delay_document(512, 4);
    let limit: u64 = 512 << 20;
    audited_allocator::current_thread_peak_reset();
    let failure = prepare_host_session(&document, &caps(limit))
        .map(|_| ())
        .expect_err("1.5 GB of delay state over a 512 MiB cap");
    let peak = audited_allocator::current_thread_peak().peak_bytes;
    let transient = (document.len() * PARSE_TRANSIENT_MULTIPLIER) as u64;
    assert!(
        peak <= transient,
        "the refusal peaked at {peak} bytes, over the {transient}-byte parse transient of a {}-byte document",
        document.len()
    );
    assert_eq!(failure.kind(), PrepareRejection::Resource);
    let diagnostic = String::from_utf8_lossy(failure.as_bytes()).into_owned();
    let fields = diagnostic
        .strip_prefix("host.effect.resource.limit\t$.maximum_effect_state_bytes[projected_bytes=")
        .and_then(|rest| rest.strip_suffix(&format!(",limit_bytes={limit},instances=2048]\n")))
        .unwrap_or_else(|| panic!("{diagnostic}"));
    let projected: u64 = fields.parse().expect("projected bytes");
    assert!(projected > limit, "{diagnostic}");
}
