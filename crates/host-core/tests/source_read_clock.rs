//! Issue #1396: a plan's source-read clock, which leads its render clock by the plan's
//! source-read offset `ΣP`.
//!
//! * **Gate 1, anchor on the source-read clock.** A plan prepared with offset `3 * 128` applies a
//!   `seek_at` anchored at source-read sample `A` in the block that renders from `A - 384`: the
//!   output equals a zero-offset reference anchored at `A - 384`.
//! * **Gate 2, an ordinary rebuild inherits.** An ordinary successor of that plan (a fader-only
//!   prepared change) reports offset 384, and a seek held across the swap applies in the same
//!   block as in a run without the rebuild.
//! * **Gate 3, a declaration resets, no frame lost.** A successor prepared at a declared
//!   discontinuity reports offset 0, plays each source's next frame after the predecessor's last
//!   played frame, and the publisher's two clocks are equal from its first block on.
//!
//! The session is the fixture reduced to one audible track `eq0` on [`SOURCE`] and a muted copy
//! of it, with no effect and no filter: its output is a fixed gain of the source PCM, so two runs
//! whose consumers read the same frames in the same blocks render the same bits.

#![cfg(feature = "test-support")]

#[path = "support/successor.rs"]
mod successor;

use engine::realtime::{PlanAdoption, SwapOutcome};
use graph_compiler::Backend;
use host_core::{
    PreparedHost, SourceControlSet, SourceSubmission, SuccessorBase,
    test_only_prepare_host_runtime_successor_on, test_only_set_source_read_offset,
};
use session::{RouteSource, SessionModel, StableId, parse_session_json};
use successor::{QUANTUM, SUCCESSOR_REVISION, Session, caps, exchange, render};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const SOURCE: &str = "fixture-source";
const MUTED_TRACK: &str = "a-muted";
/// `ΣP` of the plans under test: three quanta.
const OFFSET: u64 = 3 * QUANTUM as u64;
/// Blocks in every run.
const BLOCKS: usize = 12;
/// A seek's anchor on the source-read clock, as a block index: the offset plan renders it at
/// block `ANCHOR_BLOCK - 3`.
const ANCHOR_BLOCK: usize = 8;
/// The block before which the seek is queued.
const SEEK_BLOCK: usize = 2;
/// The block the successor renders first.
const SWAP_BLOCK: usize = 3;
/// The source frame the seek jumps to.
const TARGET_FRAME: usize = 40 * QUANTUM;

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("stable ID")
}

/// Track `eq0` on [`SOURCE`] and its muted copy [`MUTED_TRACK`], both console sections empty, no
/// inserts, input filters off. `muted_db` is the muted copy's fader gain.
fn session(muted_db: f32) -> Session {
    let mut model: SessionModel = parse_session_json(FIXTURE).expect("fixture parses");
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    model.tracks.retain(|track| track.id.as_str() == "eq0");
    for track in &mut model.tracks {
        track.console.clear();
        track.inserts.effects.clear();
        for channel in [&mut track.builtins.left, &mut track.builtins.right] {
            channel.hpf_hz = 0.0;
            channel.lpf_hz = 0.0;
        }
    }
    model.routes.retain(|route| route.id.as_str() == "eq0-main");
    let mut muted = model.tracks[0].clone();
    muted.id = id(MUTED_TRACK);
    muted.fader.left_mute = true;
    muted.fader.right_mute = true;
    muted.fader.left_db = muted_db;
    muted.fader.right_db = muted_db;
    model.tracks.push(muted);
    let mut route = model.routes[0].clone();
    route.id = id(&format!("{MUTED_TRACK}-main"));
    let RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("a track route");
    };
    *track_id = id(MUTED_TRACK);
    model.routes.push(route);
    Session::compile(model)
}

/// `session` prepared fresh with source-read offset `offset`.
fn prepared(session: &Session, offset: u64) -> PreparedHost {
    let mut prepared = session.prepare(Backend::current());
    test_only_set_source_read_offset(&mut prepared, offset);
    assert_eq!(prepared.plan.source_read_offset(), offset);
    assert_eq!(prepared.inventory.source_read_offset(), offset);
    prepared
}

/// Ramp PCM: every sample of source frame `f` is `f`, so a repeated or skipped frame moves a bit.
fn ramp(frames: core::ops::Range<usize>) -> Vec<f32> {
    frames.map(|frame| frame as f32).collect()
}

/// Submit source frames `frames` of the ramp at `generation`, one quantum per chunk.
fn submit(sources: &mut SourceControlSet, generation: u64, frames: core::ops::Range<usize>) {
    for start in frames.step_by(QUANTUM) {
        let plane = ramp(start..start + QUANTUM);
        sources
            .submit(
                SOURCE.as_bytes(),
                SourceSubmission {
                    generation,
                    start_frame: start as u64,
                    sample_rate_hz: 48_000,
                    planes: &[&plane, &plane],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .unwrap_or_else(|error| panic!("generation {generation} frame {start}: {error:?}"));
    }
}

/// How a run changes its plan at [`SWAP_BLOCK`].
#[derive(Clone, Copy)]
enum Swap {
    /// No rebuild.
    None,
    /// An ordinary successor of [`session`]`(-6.0)`.
    Ordinary,
    /// A successor at a declared discontinuity, of the same session.
    Discontinuity,
}

/// One run of `session(0.0)` prepared with `offset`, its blocks' bits, and per block the
/// publisher's `(render, source-read)` clocks after it.
///
/// With `anchor` it queues, before [`SEEK_BLOCK`] renders, `seek_at(SOURCE, 2, TARGET_FRAME,
/// anchor)` and generation 2's PCM from [`TARGET_FRAME`]; generation 1 is queued, before anything
/// renders, for exactly the blocks before the anchor's render block, so a seek that applies late
/// renders held blocks. Without one it feeds the ramp one block ahead.
fn run(offset: u64, anchor: Option<u64>, swap: Swap) -> (Vec<Vec<u32>>, Vec<(u64, u64)>) {
    let a = session(0.0);
    let PreparedHost {
        plan,
        mut sources,
        inventory,
        ..
    } = prepared(&a, offset);
    let (mut publisher, mut owner, mut retirer) = exchange(plan);
    let render_blocks = match anchor {
        Some(anchor) => usize::try_from(anchor - offset).expect("anchor") / QUANTUM,
        None => 1,
    };
    submit(&mut sources, 1, 0..render_blocks * QUANTUM);
    let mut successor_sources = None;
    let mut bits = Vec::with_capacity(BLOCKS);
    let mut clocks = Vec::with_capacity(BLOCKS);
    for block in 0..BLOCKS {
        if block == SEEK_BLOCK
            && let Some(anchor) = anchor
        {
            sources
                .seek_at(SOURCE.as_bytes(), 2, TARGET_FRAME as u64, anchor)
                .expect("anchored seek");
            submit(
                &mut sources,
                2,
                TARGET_FRAME..TARGET_FRAME + (BLOCKS - render_blocks) * QUANTUM,
            );
        }
        if block == SWAP_BLOCK && !matches!(swap, Swap::None) {
            let (b, discontinuity) = match swap {
                Swap::Ordinary => (session(-6.0), false),
                _ => (session(0.0), true),
            };
            let mut next = test_only_prepare_host_runtime_successor_on(
                &b.compiled,
                &caps(),
                SuccessorBase {
                    discontinuity,
                    ..SuccessorBase::new(&inventory, &a.model, SUCCESSOR_REVISION)
                },
                Backend::current(),
            )
            .unwrap_or_else(|failure| panic!("successor: {failure:?}"));
            let expected = if discontinuity { 0 } else { OFFSET };
            assert_eq!(
                next.plan.source_read_offset(),
                expected,
                "the plan's offset"
            );
            assert_eq!(
                next.inventory.source_read_offset(),
                expected,
                "the inventory row's offset"
            );
            assert_eq!(next.sources.adopt_persisting(&mut sources), 1);
            next.plan
                .take_revision_gate()
                .expect("the successor's handle")
                .publish(SUCCESSOR_REVISION);
            publisher
                .reserve_replacement(next.plan, PlanAdoption::Next)
                .unwrap_or_else(|_| panic!("reserve the successor"))
                .commit(SUCCESSOR_REVISION);
            successor_sources = Some(next.sources);
        }
        if anchor.is_none() && block + 1 < BLOCKS {
            let feeding = successor_sources.as_mut().unwrap_or(&mut sources);
            submit(feeding, 1, (block + 1) * QUANTUM..(block + 2) * QUANTUM);
        }
        let rendered = render(&mut owner);
        if block == SWAP_BLOCK && !matches!(swap, Swap::None) {
            assert_eq!(rendered.swap, SwapOutcome::Applied);
        }
        bits.push(rendered.bits);
        clocks.push((publisher.render_clock(), publisher.source_read_clock()));
        while retirer.try_reclaim().is_ok() {}
    }
    (bits, clocks)
}

/// The first block whose bits differ, if any.
fn first_difference(left: &[Vec<u32>], right: &[Vec<u32>]) -> Option<usize> {
    left.iter()
        .zip(right)
        .position(|(left, right)| left != right)
}

/// Gate 1 (the old #1287 gate 4, per D15-12).
///
/// Test value: red if the graph compares a held seek with the render clock: the offset plan then
/// applies it three blocks late, and its blocks between hold.
#[test]
fn a_seek_applies_on_the_source_read_clock() {
    let anchor = (ANCHOR_BLOCK * QUANTUM) as u64;
    let (offset, clocks) = run(OFFSET, Some(anchor), Swap::None);
    let (reference, _) = run(0, Some(anchor - OFFSET), Swap::None);
    assert_eq!(first_difference(&offset, &reference), None);
    // The jump is audible, and the run plays before and after it.
    let jump = ANCHOR_BLOCK - 3;
    assert_ne!(offset[jump - 1], offset[jump]);
    assert!(
        offset
            .iter()
            .all(|block| block.iter().any(|bits| *bits != 0))
    );
    for (block, (render, source_read)) in clocks.into_iter().enumerate() {
        let next = ((block + 1) * QUANTUM) as u64;
        assert_eq!(
            (render, source_read),
            (next, next + OFFSET),
            "block {block}"
        );
    }
}

/// Gate 2.
///
/// Test value: red if an ordinary successor defaults its offset to 0: the seek held across the
/// swap then applies three blocks late, and the blocks between hold.
#[test]
fn an_ordinary_successor_inherits_the_offset() {
    let anchor = (ANCHOR_BLOCK * QUANTUM) as u64;
    let (rebuilt, clocks) = run(OFFSET, Some(anchor), Swap::Ordinary);
    let (unbroken, _) = run(OFFSET, Some(anchor), Swap::None);
    assert_eq!(first_difference(&rebuilt, &unbroken), None);
    for (block, (render, source_read)) in clocks.into_iter().enumerate() {
        let next = ((block + 1) * QUANTUM) as u64;
        assert_eq!(
            (render, source_read),
            (next, next + OFFSET),
            "block {block}"
        );
    }
}

/// Gate 3.
///
/// Test value: red if the reset rewinds or advances a consumer (a frame repeats or is skipped,
/// and the ramp moves a bit), or if the discontinuity successor keeps the predecessor's offset
/// (the source-read clock stays ahead).
#[test]
fn a_declaration_resets_the_offset_and_loses_no_frame() {
    let (declared, clocks) = run(OFFSET, None, Swap::Discontinuity);
    let (unbroken, _) = run(0, None, Swap::None);
    assert_eq!(first_difference(&declared, &unbroken), None);
    assert!(declared[SWAP_BLOCK].iter().any(|bits| *bits != 0));
    for (block, (render, source_read)) in clocks.into_iter().enumerate() {
        let next = ((block + 1) * QUANTUM) as u64;
        let expected = if block < SWAP_BLOCK {
            next + OFFSET
        } else {
            next
        };
        assert_eq!((render, source_read), (next, expected), "block {block}");
    }
}
