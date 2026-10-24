//! Issue #1402 (a slice of #1287's latency growth): a source-claim line keeps its pending raw
//! frames through an ordinary plan swap.
//!
//! * **Gate 1, M1 at carry level, 48 kHz, every vector backend.** W1 is a two-track session
//!   prepared through the `test-support` floors hook (D3) with every node at its arrival plus `P`:
//!   it stands in for a plan after a warm adoption, with a `P`-sample claim line on each track and
//!   no declared delay. After 8 blocks an ordinary successor that adds a muted track is prepared
//!   from W1's inventory and adopted; the next 64 blocks equal W1 never swapped, bit for bit, and
//!   the adoption block allocates and frees nothing on the render thread (gate 3, folded in).
//!
//! The fill of a grown line (lemma L2, L3) is `graph`'s unit test; its adoption is #1355's.

#![cfg(feature = "test-support")]

#[path = "support/successor.rs"]
mod successor;

use bench_support::alloc::{Mode, assert_installed, mode, set_mode};
use engine::realtime::{CarryOutcome, PlanAdoption};
use graph::{GraphNodeId, StableGraphId, TrackStage};
use graph_compiler::Backend;
use host_core::{PlanStateInventory, PreparedHost, SuccessorBase};
use session::{RouteSource, SessionModel, StableId, parse_session_json};
use successor::{Feed, QUANTUM, Session, caps, exchange, render};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const SOURCE: &str = "fixture-source";
/// The muted track the ordinary successor adds; it sorts first, so every strip's index moves.
const MUTED_TRACK: &str = "a-muted";
/// The lead: two blocks.
const P: u64 = 2 * QUANTUM as u64;
/// Blocks W1 renders before the swap, and blocks compared after it.
const BEFORE: usize = 8;
const AFTER: usize = 64;

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("stable ID")
}

/// Tracks `eq0` and `eq1` on [`SOURCE`], with no console slot, insert, input filter or declared
/// delay: a track's ring is its claim line alone.
fn session_w() -> SessionModel {
    let mut model = parse_session_json(FIXTURE).expect("fixture parses");
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    model
        .tracks
        .retain(|track| matches!(track.id.as_str(), "eq0" | "eq1"));
    for track in &mut model.tracks {
        track.console.clear();
        track.inserts.effects.clear();
        for channel in [&mut track.builtins.left, &mut track.builtins.right] {
            channel.hpf_hz = 0.0;
            channel.lpf_hz = 0.0;
            channel.delay_samples = 0;
        }
    }
    model
        .routes
        .retain(|route| matches!(route.id.as_str(), "eq0-main" | "eq1-main"));
    model
}

/// `model` plus [`MUTED_TRACK`], a muted copy of `eq0` with its own route to the output.
fn with_muted_track(mut model: SessionModel) -> SessionModel {
    let mut added = model.tracks[0].clone();
    added.id = id(MUTED_TRACK);
    added.fader.left_mute = true;
    added.fader.right_mute = true;
    model.tracks.push(added);
    let mut route = model
        .routes
        .iter()
        .find(|route| route.id.as_str() == "eq0-main")
        .expect("template route")
        .clone();
    route.id = id(&format!("{MUTED_TRACK}-main"));
    let RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("a track route");
    };
    *track_id = id(MUTED_TRACK);
    model.routes.push(route);
    model
}

/// `session` prepared as the successor, at `revision`, of the plan `inventory` describes, whose
/// committed model is `committed`; the revision is published on the plan's gate, the commit's
/// last write.
fn successor(
    session: &Session,
    inventory: &PlanStateInventory,
    committed: &SessionModel,
    revision: u64,
    backend: Backend,
) -> PreparedHost {
    let mut prepared = host_core::test_only_prepare_host_runtime_successor_on(
        &session.compiled,
        &caps(),
        SuccessorBase::new(inventory, committed, revision),
        backend,
    )
    .unwrap_or_else(|failure| panic!("successor: {failure:?}"));
    prepared
        .plan
        .take_revision_gate()
        .expect("the successor's handle")
        .publish(revision);
    prepared
}

/// W1: `w` prepared with every node at its arrival in `a` plus `P` (D3). A track's `Input` node,
/// at arrival 0, is floored at `P`, so every node after it arrives `P` later.
fn floored(w: &Session, a: &PreparedHost, backend: Backend) -> PreparedHost {
    let mut floors: std::collections::BTreeMap<GraphNodeId, u64> = a
        .inventory
        .input_floors()
        .iter()
        .map(|(node, arrival)| (node.clone(), arrival + P))
        .collect();
    for track in &w.model.tracks {
        floors
            .entry(GraphNodeId::TrackStage {
                track_id: StableGraphId::parse(track.id.as_str()).expect("graph ID"),
                stage: TrackStage::Input,
            })
            .or_insert(P);
    }
    let floors: Vec<(GraphNodeId, u64)> = floors.into_iter().collect();
    successor(
        w,
        &a.inventory.test_only_with_input_floors(&floors),
        &w.model,
        2,
        backend,
    )
}

/// Gate 1. Red if the strip delay join needs a lane by its declared delay rather than its ring:
/// each claim line then starts at rest and every source drops out for `P` samples after the swap.
#[test]
fn an_ordinary_rebuild_carries_every_claim_line() {
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
    let w = Session::compile(session_w());
    let b = Session::compile(with_muted_track(session_w()));
    assert_eq!(w.model.sample_rate_hz, 48_000, "R-C: gate 1 runs at 48 kHz");
    let feed = Feed::with_blocks(SOURCE, 1, BEFORE + AFTER + 1);
    for &backend in Backend::VECTOR {
        let a = w.prepare(backend);
        let rate = a.report.sample_rate_hz;
        // The reference: W1 never swapped.
        let reference = {
            let PreparedHost {
                plan, mut sources, ..
            } = floored(&w, &a, backend);
            let (_publisher, mut owner, _retirer) = exchange(plan);
            feed.submit(&mut sources, rate, 0);
            (0..BEFORE + AFTER)
                .map(|block| {
                    feed.submit(&mut sources, rate, block + 1);
                    render(&mut owner).bits
                })
                .collect::<Vec<_>>()
        };
        let w1 = floored(&w, &a, backend);
        assert_eq!(
            w1.report.latency_samples,
            a.report.latency_samples + P,
            "{backend:?}: the floors lead W1 by P"
        );
        let PreparedHost {
            plan,
            mut sources,
            inventory,
            ..
        } = w1;
        let (mut publisher, mut owner, _retirer) = exchange(plan);
        let mut blocks = Vec::with_capacity(BEFORE + AFTER);
        feed.submit(&mut sources, rate, 0);
        for block in 0..BEFORE {
            feed.submit(&mut sources, rate, block + 1);
            blocks.push(render(&mut owner));
        }
        let mut w2 = successor(&b, &inventory, &w.model, 3, backend);
        assert_eq!(
            w2.restarted_strips(),
            &[] as &[Box<str>],
            "{backend:?}: no strip restarts"
        );
        w2.sources.adopt_persisting(&mut sources);
        let PreparedHost {
            plan,
            sources: mut w2_sources,
            ..
        } = w2;
        publisher
            .reserve_replacement(plan, PlanAdoption::Next)
            .unwrap_or_else(|_| panic!("reserve the successor"))
            .commit(3);
        for block in BEFORE..BEFORE + AFTER {
            feed.submit(&mut w2_sources, rate, block + 1);
            blocks.push(render(&mut owner));
        }
        let adoption = &blocks[BEFORE];
        assert_eq!(adoption.carry, CarryOutcome::Carried, "{backend:?}");
        assert_eq!(
            adoption.allocator,
            (0, 0, 0),
            "{backend:?}: allocator calls in the adoption block"
        );
        assert_eq!(adoption.audit, (0, 0), "{backend:?}: render audit");
        assert!(
            reference[BEFORE..BEFORE + 2]
                .iter()
                .flatten()
                .any(|bits| *bits != 0),
            "{backend:?}: the claim lines' pending frames play after the swap"
        );
        assert_eq!(
            blocks[BEFORE..]
                .iter()
                .zip(&reference[BEFORE..])
                .position(|(swapped, unswapped)| swapped.bits != *unswapped),
            None,
            "{backend:?}: after the swap, W1's run never swapped"
        );
    }
}
