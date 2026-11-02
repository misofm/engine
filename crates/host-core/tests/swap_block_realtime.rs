//! Issue #2229: the swap block of the 64-track app console touches neither the allocator nor the
//! render audit. That block moves every console lane: it puts each predecessor's packed lanes
//! home (`restore_home_lanes`, a series of lane swaps) and carries each effect lane into a
//! shifted successor lane, both by the effect's direct copy and swap rather than the state
//! payload. A direct path that boxed, dropped or logged anything would move the counters here.
//!
//! The subject is `bench swap`'s move (#1286): A is the fixture at 96 kHz, B is A plus one muted
//! track whose ID sorts first, so every strip lands in another lane. Each plan renders
//! [`BEFORE`] blocks, enough for lane packing to displace its lanes, before the next one is
//! adopted; both adoption blocks (A to B, B to A) are measured.

#![cfg(feature = "test-support")]

#[path = "support/successor.rs"]
mod successor;

use core::num::NonZeroUsize;

use bench_support::alloc::{Mode, assert_installed, mode, set_mode};
use engine::realtime::{CarryOutcome, PlanAdoption, SwapOutcome};
use host_core::{
    CompiledSession, HostLiveControlRequest, HostLiveLanes, HostPrepareCaps, HostShapePolicy,
    LiveValuesSuperseded, PreparedHost, SuccessorBase, compile_host_model,
};
use session::{RouteSource, SessionModel, StableId, parse_session_json};
use successor::{Feed, exchange, render};

const FIXTURE: &str =
    include_str!("../../../fixtures/session/v1/console-sixty-four-track-app.json");
const SOURCE: &str = "fixture-source";
const MUTED_TRACK: &str = "aa-muted";
/// Blocks each plan renders before its successor is adopted.
const BEFORE: usize = 32;

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 8_192,
        maximum_tracks: 1_024,
        maximum_submixes: 1_024,
        maximum_vcas: 256,
        maximum_vca_reach_pairs: u64::MAX,
        maximum_sources: 1_024,
        maximum_routes: 4_096,
        maximum_effects: 8_192,
        maximum_graph_session_plus_plan_bytes: 1 << 34,
        maximum_source_total_bytes: 1 << 32,
        maximum_source_overhead_bytes: 1 << 32,
        maximum_effect_state_bytes: 1 << 34,
        maximum_effect_scratch_bytes: 1 << 34,
        maximum_builtin_retained_bytes: 1 << 34,
        maximum_named_allocation_bytes: 1 << 34,
        maximum_meter_streams: 4_096,
        maximum_meter_items: 1 << 20,
        maximum_meter_bytes: 1 << 28,
    }
}

fn compile(model: &SessionModel) -> (SessionModel, CompiledSession) {
    let compile_caps = caps()
        .compile_caps(model.sources.len())
        .unwrap_or_else(|failure| panic!("compile caps: {failure:?}"));
    let compiled = compile_host_model(model, compile_caps)
        .unwrap_or_else(|failure| panic!("compile: {failure:?}"));
    (compiled.normalized_model().clone(), compiled)
}

/// A and B (the module comment).
fn sessions() -> [(SessionModel, CompiledSession); 2] {
    let mut a = parse_session_json(FIXTURE).expect("the app-shape fixture parses");
    a.sample_rate_hz = 96_000;
    assert_eq!(a.tracks.len(), 64, "the 64-track console");
    let mut b = a.clone();
    let mut muted = b.tracks[0].clone();
    muted.id = StableId::parse(MUTED_TRACK).expect("ID");
    muted.fader.left_mute = true;
    muted.fader.right_mute = true;
    b.tracks.push(muted);
    let mut route = b.routes[0].clone();
    route.id = StableId::parse(&format!("{MUTED_TRACK}-main")).expect("ID");
    let RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("the fixture's first route leaves a track");
    };
    *track_id = StableId::parse(MUTED_TRACK).expect("ID");
    b.routes.push(route);
    [compile(&a), compile(&b)]
}

/// Prepared as the C ABI prepares the app console: live lanes and observation taps.
fn prepare(compiled: &CompiledSession, base: Option<SuccessorBase<'_>>) -> PreparedHost {
    let live = HostLiveControlRequest {
        meter_queue_depth: NonZeroUsize::new(host_core::METER_QUEUE_DEPTH)
            .unwrap_or(NonZeroUsize::MIN),
        observation_taps: host_core::launch_maximum_observation_taps(),
        ..HostLiveControlRequest::default()
    };
    let lanes: HostLiveLanes = controls::live_lanes(compiled.normalized_model()).into();
    let lanes = HostLiveLanes {
        effects: true,
        ..lanes
    };
    host_core::prepare_host_runtime_with_live_lanes_and_spectrum(
        compiled,
        &caps(),
        &live,
        lanes,
        None,
        base,
        &LiveValuesSuperseded::new(),
    )
    .unwrap_or_else(|failure| panic!("prepare: {failure:?}"))
    .0
}

#[test]
fn the_sixty_four_track_swap_block_allocates_and_audits_nothing() {
    /// Count a render-scope allocation instead of aborting, so a violation reads as this test's
    /// assertion; restored on exit.
    struct RestoreMode(Mode);
    impl Drop for RestoreMode {
        fn drop(&mut self) {
            set_mode(self.0);
        }
    }
    assert_installed();
    let _restore = RestoreMode(mode());
    set_mode(Mode::Count);
    engine::realtime::audit::warm_up();
    let [a, b] = sessions();
    let feed = Feed::with_blocks(SOURCE, 1, 2 * BEFORE + 3);
    let first = prepare(&a.1, None);
    let rate = first.report.sample_rate_hz;
    let PreparedHost {
        plan,
        mut sources,
        mut inventory,
        ..
    } = first;
    let (mut publisher, mut owner, mut retirer) = exchange(plan);
    let mut committed = a.0.clone();
    let mut block = 0;
    feed.submit(&mut sources, rate, 0);
    for (step, next) in [&b, &a].into_iter().enumerate() {
        for _ in 0..BEFORE {
            block += 1;
            feed.submit(&mut sources, rate, block);
            render(&mut owner);
        }
        let revision = 2 + step as u64;
        let PreparedHost {
            mut plan,
            sources: mut next_sources,
            inventory: next_inventory,
            ..
        } = prepare(
            &next.1,
            Some(SuccessorBase::new(&inventory, &committed, revision)),
        );
        plan.take_revision_gate()
            .expect("the successor's gate")
            .publish(revision);
        let reservation = publisher
            .reserve_replacement(plan, PlanAdoption::Next)
            .unwrap_or_else(|_| panic!("reserve the successor"));
        next_sources.adopt_persisting(&mut sources);
        reservation.commit(revision);
        (sources, inventory, committed) = (next_sources, next_inventory, next.0.clone());
        block += 1;
        feed.submit(&mut sources, rate, block);
        let adoption = render(&mut owner);
        assert_eq!(adoption.swap, SwapOutcome::Applied, "move {step}");
        assert_eq!(adoption.carry, CarryOutcome::Carried, "move {step}");
        assert_eq!(
            adoption.allocator,
            (0, 0, 0),
            "move {step}: allocator calls in the swap block"
        );
        assert_eq!(adoption.audit, (0, 0), "move {step}: render audit");
        while retirer.try_reclaim().is_ok() {}
    }
}
