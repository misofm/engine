//! Issue #1272 (slice 3 of #1269): a successor plan whose unchanged sources keep playing.
//!
//! Every case renders a predecessor, prepares the post-edit session as its successor
//! (`SuccessorBase` with the predecessor's inventory and committed model), hands the producers
//! over (`SourceControlSet::adopt_persisting`), swaps the successor in through the engine's plan
//! exchange and renders on. The reference is the post-edit session prepared fresh and fed the same
//! PCM from frame 0. The test signal has no exact-zero sample, and an added strip is muted, so it
//! contributes exact `+0.0` and the master sum moves no bit.
//!
//! * **Gate 1, gap-free acceptance, both widths.** Adding a muted track renders every block
//!   bit-identical to the reference; the swap block reports `Carried`. The same run with a fresh
//!   successor differs at the swap block, so the oracle can fail.
//! * **Gate 2, removed source.** A's second track reads a second source and is muted; B removes
//!   that track, its route and its source. The removed source sorts first, so the kept source's
//!   index moves from 1 to 0: a hand-over keyed by index plays the wrong ring.
//! * **Gate 3, changed source.** A changed declaration carries nothing: the successor allocates and
//!   charges its own ring.
//! * **Gate 4, caps.** A successor's allocated plus carried rings are held to the source cap.
//! * **Gate 5, realtime.** Every block after the warm-up, the swap block (carry included) among
//!   them, allocates and frees nothing.

#![cfg(feature = "test-support")]

#[path = "support/successor.rs"]
mod successor;

use engine::realtime::{CarryOutcome, SwapOutcome};
use graph_compiler::Backend;
use host_core::{PrepareRejection, SourceControlError, SourceControlSet, SourceSubmission};
use session::{RouteSource, SessionModel, Source, StableId, parse_session_json};
use successor::{
    BLOCKS, Feed, QUANTUM, SWAP_BLOCK, Session, Successor, caps, first_difference, reference_run,
    swapped_run,
};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const SOURCE: &str = "fixture-source";
/// Sorts before [`SOURCE`], so removing it shifts the kept source's index.
const AUX_SOURCE: &str = "aux-source";
const MUTED_TRACK: &str = "a-muted";

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("stable ID")
}

/// Session A: tracks `eq0` and `eq1` on [`SOURCE`], both console sections empty, no inserts, input
/// filters off, default trim, fader and pan.
fn session_a() -> SessionModel {
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
        }
    }
    model
        .routes
        .retain(|route| matches!(route.id.as_str(), "eq0-main" | "eq1-main"));
    model
}

/// Add a copy of track `from` named `track`, reading `source`, with its own route to the output.
fn add_track(model: &mut SessionModel, from: &str, track: &str, source: &str, muted: bool) {
    let mut added = model
        .tracks
        .iter()
        .find(|existing| existing.id.as_str() == from)
        .expect("template track")
        .clone();
    added.id = id(track);
    added.source_id = id(source);
    added.fader.left_mute = muted;
    added.fader.right_mute = muted;
    model.tracks.push(added);
    let mut route = model
        .routes
        .iter()
        .find(|route| route.id.as_str() == format!("{from}-main"))
        .expect("template route")
        .clone();
    route.id = id(&format!("{track}-main"));
    let RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("a track route");
    };
    *track_id = id(track);
    model.routes.push(route);
}

fn remove_track(model: &mut SessionModel, track: &str) {
    model
        .tracks
        .retain(|existing| existing.id.as_str() != track);
    model
        .routes
        .retain(|route| route.id.as_str() != format!("{track}-main"));
}

/// Add a source declared like [`SOURCE`] under `source`, with its own content.
fn add_source(model: &mut SessionModel, source: &str) {
    let template = model.sources[0].clone();
    model.sources.push(Source {
        id: id(source),
        content: template.content.replace("7e94", "0a0a"),
        ..template
    });
}

fn backends() -> Vec<Backend> {
    vec![
        #[cfg(target_feature = "avx2")]
        Backend::Simd8,
        Backend::Simd4,
    ]
}

/// Gate 1 at one width.
fn added_muted_track_keeps_playing(backend: Backend) {
    let a = Session::compile(session_a());
    let mut b_model = session_a();
    add_track(&mut b_model, "eq0", MUTED_TRACK, SOURCE, true);
    let b = Session::compile(b_model);
    assert_eq!(
        b.compiled.normalized_model().tracks[0].id.as_str(),
        MUTED_TRACK,
        "the added track sorts first"
    );
    let feed = Feed::new(SOURCE, 1);

    let reference = reference_run(&b, &[&feed], backend);
    let mut run = swapped_run(&a, &b, &[&feed], &[&feed], backend, Successor::Carry);
    assert_eq!(run.adopted, 1, "{backend:?}: the producer moved");
    assert_eq!(
        (run.blocks[SWAP_BLOCK].swap, run.blocks[SWAP_BLOCK].carry),
        (SwapOutcome::Applied, CarryOutcome::Carried),
        "{backend:?}: the swap block"
    );
    assert_eq!(
        first_difference(&run.blocks, &reference),
        None,
        "{backend:?}: every block equals the post-edit session rendered from frame 0"
    );
    assert!(
        reference[SWAP_BLOCK].bits.iter().any(|bits| *bits != 0),
        "the reference plays at the swap block, or the comparison is vacuous"
    );
    // The predecessor's producer moved: its set refuses PCM as vacated, not as unknown.
    let planes = [vec![0.25_f32; QUANTUM], vec![0.25_f32; QUANTUM]];
    let refused = run
        .predecessor_sources
        .submit(
            SOURCE.as_bytes(),
            SourceSubmission {
                generation: 1,
                start_frame: (BLOCKS * QUANTUM) as u64,
                sample_rate_hz: 48_000,
                planes: &[&planes[0], &planes[1]],
                frames: QUANTUM as u32,
                end_of_region: false,
            },
        )
        .expect_err("the moved producer");
    assert_eq!(refused, SourceControlError::Vacated);
    assert_eq!(
        run.predecessor_sources.seek(SOURCE.as_bytes(), 2, 0),
        Err(SourceControlError::Vacated)
    );

    // The successor allocated no ring, and charges the carried one: its allocated plus carried
    // source bytes are exactly a fresh plan's.
    let fresh = b.prepare(backend).report;
    let report = run.successor;
    assert!(report.carried_source_total_bytes > 0);
    assert_eq!(
        report.source_total_bytes + report.carried_source_total_bytes,
        fresh.source_total_bytes,
        "{backend:?}: allocated plus carried source bytes"
    );
    assert_eq!(
        report.source_overhead_bytes + report.carried_source_overhead_bytes,
        fresh.source_overhead_bytes
    );
    assert_eq!(
        report.carry_program_retained_bytes,
        core::mem::size_of::<(u32, u32)>() as u64
            + 2 * core::mem::size_of::<graph::GraphLaneMove>() as u64,
        "one (u32, u32) source move and the two unchanged input sections (#1276)"
    );

    // The oracle can fail: today's fresh rebuild leaves the swap block silent.
    let fresh_run = swapped_run(&a, &b, &[&feed], &[&feed], backend, Successor::Fresh);
    assert_eq!(
        fresh_run.blocks[SWAP_BLOCK].carry,
        CarryOutcome::NotRequested
    );
    assert_eq!(
        first_difference(&fresh_run.blocks, &reference),
        Some(SWAP_BLOCK),
        "{backend:?}: a fresh successor differs at the swap block"
    );
}

/// Gate 1 at eight lanes. Red if the successor still allocates and plays a fresh ring, or if the
/// carry or the producer hand-over loses the queued block or the read position.
#[cfg(target_feature = "avx2")]
#[test]
fn an_added_muted_track_keeps_the_source_playing_at_eight_lanes() {
    added_muted_track_keeps_playing(Backend::Simd8);
}

/// Gate 1 at four lanes, the NEON/simd128 width.
#[test]
fn an_added_muted_track_keeps_the_source_playing_at_four_lanes() {
    added_muted_track_keeps_playing(Backend::Simd4);
}

/// Gate 2. Red if the carry pairs sources, or the producer hand-over finds them, by index: the
/// kept source is index 1 in A and index 0 in B.
#[test]
fn a_removed_source_leaves_the_kept_source_playing() {
    let mut a_model = session_a();
    remove_track(&mut a_model, "eq1");
    add_source(&mut a_model, AUX_SOURCE);
    // Muted, so A's output is B's before the swap too.
    add_track(&mut a_model, "eq0", "aux", AUX_SOURCE, true);
    let a = Session::compile(a_model);
    assert_eq!(
        a.compiled.normalized_model().sources[0].id.as_str(),
        AUX_SOURCE
    );
    let mut b_model = session_a();
    remove_track(&mut b_model, "eq1");
    let b = Session::compile(b_model);
    let feed = Feed::new(SOURCE, 1);
    let aux = Feed::new(AUX_SOURCE, 2);
    for backend in backends() {
        let reference = reference_run(&b, &[&feed], backend);
        let mut run = swapped_run(&a, &b, &[&aux, &feed], &[&feed], backend, Successor::Carry);
        assert_eq!(run.blocks[SWAP_BLOCK].carry, CarryOutcome::Carried);
        assert_eq!(
            first_difference(&run.blocks, &reference),
            None,
            "{backend:?}: every block equals B fed the kept source from frame 0"
        );
        let planes = [vec![0.25_f32; QUANTUM], vec![0.25_f32; QUANTUM]];
        assert_eq!(
            run.successor_sources.submit(
                AUX_SOURCE.as_bytes(),
                SourceSubmission {
                    generation: 1,
                    start_frame: 0,
                    sample_rate_hz: 48_000,
                    planes: &[&planes[0], &planes[1]],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            ),
            Err(SourceControlError::UnknownSource)
        );
    }
}

/// Gate 3. Red if a source whose declaration changed is carried.
#[test]
fn a_changed_source_gets_its_own_ring() {
    let a = Session::compile(session_a());
    let mut b_model = session_a();
    b_model.sources[0].frames -= 1;
    let b = Session::compile(b_model);
    for backend in backends() {
        let predecessor = a.prepare(backend);
        let mut prepared = b
            .prepare_successor(&predecessor.inventory, &a.model, &caps(), backend)
            .expect("successor");
        let fresh = b.prepare(backend).report;
        assert_eq!(prepared.report.carried_source_total_bytes, 0);
        assert_eq!(
            prepared.report.carry_program_retained_bytes,
            2 * core::mem::size_of::<graph::GraphLaneMove>() as u64,
            "no source move; only the two unchanged input sections (#1276)"
        );
        assert_eq!(
            prepared.report.source_total_bytes, fresh.source_total_bytes,
            "{backend:?}: the successor allocates and charges a new ring"
        );
        let mut predecessor_sources = predecessor.sources;
        assert_eq!(
            prepared.sources.adopt_persisting(&mut predecessor_sources),
            0
        );
        Feed::new(SOURCE, 1).submit(&mut prepared.sources, 48_000, 0);
    }
}

/// Gate 4. Red if the source cap counts only the rings the successor allocates: its active plan
/// would exceed the cap after the swap.
#[test]
fn a_successor_charges_allocated_and_carried_rings_to_the_source_cap() {
    const NEW_SOURCE: &str = "new-source";
    let a = Session::compile(session_a());
    let mut b_model = session_a();
    add_source(&mut b_model, NEW_SOURCE);
    add_track(&mut b_model, "eq0", "new", NEW_SOURCE, true);
    let b = Session::compile(b_model);
    let backend = Backend::current();
    let predecessor = a.prepare(backend);
    let roomy = b
        .prepare_successor(&predecessor.inventory, &a.model, &caps(), backend)
        .expect("successor")
        .report;
    assert!(roomy.source_total_bytes > 0 && roomy.carried_source_total_bytes > 0);
    let required = roomy.source_total_bytes + roomy.carried_source_total_bytes;
    let mut exact = caps();
    exact.maximum_source_total_bytes = required;
    b.prepare_successor(&predecessor.inventory, &a.model, &exact, backend)
        .expect("the cap admits exactly allocated plus carried");
    let mut below = caps();
    below.maximum_source_total_bytes = required - 1;
    let refused = b
        .prepare_successor(&predecessor.inventory, &a.model, &below, backend)
        .expect_err("one byte below allocated plus carried");
    assert_eq!(refused.kind(), PrepareRejection::Resource);
    assert_eq!(refused.as_bytes(), b"host.source.resource.limit\t$\n");

    // The overhead cap counts carried overhead too.
    assert!(roomy.carried_source_overhead_bytes > 0);
    let mut below = caps();
    below.maximum_source_overhead_bytes =
        roomy.source_overhead_bytes + roomy.carried_source_overhead_bytes - 1;
    let refused = b
        .prepare_successor(&predecessor.inventory, &a.model, &below, backend)
        .expect_err("one byte below allocated plus carried overhead");
    assert_eq!(refused.as_bytes(), b"host.source.resource.limit\t$\n");
}

/// #1272 follow-up MINOR 3. Red if the carry rule stops comparing the ring configuration (the
/// carried bytes would be charged at the successor's ring size, not the ring carried) or compares
/// only part of the declaration (a content change would carry the old stem's ring).
#[test]
fn another_ring_size_or_changed_content_carries_no_ring() {
    let a = Session::compile(session_a());
    let mut b_model = session_a();
    add_track(&mut b_model, "eq0", MUTED_TRACK, SOURCE, true);
    let b = Session::compile(b_model);
    let backend = Backend::current();
    let predecessor = a.prepare(backend);
    let same = b
        .prepare_successor(&predecessor.inventory, &a.model, &caps(), backend)
        .expect("successor");
    assert!(
        same.report.carried_source_total_bytes > 0,
        "the control case carries"
    );
    let mut smaller = caps();
    smaller.source_ring_frames /= 2;
    let resized = b
        .prepare_successor(&predecessor.inventory, &a.model, &smaller, backend)
        .expect("successor with smaller rings");
    assert_eq!(resized.report.carried_source_total_bytes, 0);

    let mut c_model = session_a();
    let template = c_model.sources[0].clone();
    c_model.sources[0] = Source {
        content: template.content.replace("7e94", "0b0b"),
        ..template
    };
    assert_ne!(c_model.sources[0].content, a.model.sources[0].content);
    let c = Session::compile(c_model);
    let changed = c
        .prepare_successor(&predecessor.inventory, &a.model, &caps(), backend)
        .expect("successor with changed content");
    assert_eq!(changed.report.carried_source_total_bytes, 0);
}

/// #1272 follow-up MINOR 4. Red if the largest-allocation fold leaves out a carried ring: with
/// rings long enough to be the session's largest allocation, a successor that carries its only
/// ring would report less than a fresh plan, and the named-allocation cap would not see it.
#[test]
fn the_largest_allocation_counts_a_carried_ring() {
    let mut long_rings = caps();
    long_rings.source_ring_frames *= 256;
    let a = Session::compile(session_a());
    let mut b_model = session_a();
    add_track(&mut b_model, "eq0", MUTED_TRACK, SOURCE, true);
    let b = Session::compile(b_model);
    let backend = Backend::current();
    let prepare = |session: &Session| {
        host_core::test_only_prepare_host_runtime_with_live_controls_on(
            &session.compiled,
            &long_rings,
            &host_core::HostLiveControlRequest::default(),
            backend,
        )
        .unwrap_or_else(|failure| panic!("prepare: {failure:?}"))
        .0
    };
    let predecessor = prepare(&a);
    let fresh = prepare(&b).report;
    let successor = b
        .prepare_successor(&predecessor.inventory, &a.model, &long_rings, backend)
        .expect("successor")
        .report;
    assert!(successor.carried_source_total_bytes > 0);
    assert_eq!(
        successor.largest_engine_allocation_bytes,
        fresh.largest_engine_allocation_bytes
    );
}

/// #1272 follow-up MINOR 2. Red if the carry program's bytes are left out of the graph cap: the
/// successor's smallest admitting `maximum_graph_session_plus_plan_bytes` would equal a fresh
/// plan's although the successor retains the program too.
#[test]
fn the_carry_program_is_charged_to_the_graph_cap() {
    fn smallest_admitting(admits: impl Fn(u64) -> bool) -> u64 {
        let (mut low, mut high) = (0_u64, caps().maximum_graph_session_plus_plan_bytes);
        assert!(admits(high));
        while low + 1 < high {
            let middle = low + (high - low) / 2;
            if admits(middle) {
                high = middle;
            } else {
                low = middle;
            }
        }
        high
    }
    let a = Session::compile(session_a());
    let mut b_model = session_a();
    add_track(&mut b_model, "eq0", MUTED_TRACK, SOURCE, true);
    let b = Session::compile(b_model);
    let backend = Backend::current();
    let predecessor = a.prepare(backend);
    let program = b
        .prepare_successor(&predecessor.inventory, &a.model, &caps(), backend)
        .expect("successor")
        .report
        .carry_program_retained_bytes;
    assert!(program > 0);
    let with_graph_cap = |cap| {
        let mut caps = caps();
        caps.maximum_graph_session_plus_plan_bytes = cap;
        caps
    };
    let fresh = smallest_admitting(|cap| {
        host_core::test_only_prepare_host_runtime_with_live_controls_on(
            &b.compiled,
            &with_graph_cap(cap),
            &host_core::HostLiveControlRequest::default(),
            backend,
        )
        .is_ok()
    });
    let successor = smallest_admitting(|cap| {
        b.prepare_successor(
            &predecessor.inventory,
            &a.model,
            &with_graph_cap(cap),
            backend,
        )
        .is_ok()
    });
    assert_eq!(successor, fresh + program);
}

/// Gate 5. After the warm-up block, every block of the swapped run -- the swap block, which runs
/// the carry, among them -- makes no allocator call. Red if the hand-over clones, boxes or drops
/// anything on the render thread.
#[test]
fn the_swap_block_allocates_and_frees_nothing() {
    use bench_support::alloc::{Mode, assert_installed, mode, set_mode};
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
    let a = Session::compile(session_a());
    let mut b_model = session_a();
    add_track(&mut b_model, "eq0", MUTED_TRACK, SOURCE, true);
    let b = Session::compile(b_model);
    let feed = Feed::new(SOURCE, 1);
    let run = swapped_run(
        &a,
        &b,
        &[&feed],
        &[&feed],
        Backend::current(),
        Successor::Carry,
    );
    assert_eq!(run.blocks[SWAP_BLOCK].carry, CarryOutcome::Carried);
    for (index, block) in run.blocks.iter().enumerate().skip(1) {
        assert_eq!(block.allocator, (0, 0, 0), "block {index}: allocator calls");
        assert_eq!(block.audit, (0, 0), "block {index}: render audit");
    }
}

// Issue #1274 (slice 5 of #1269): an anchored seek, `SourceControlSet::seek_at`, holds a source's
// new generation until the block that starts at its anchor sample on the plan's absolute render
// clock, which the swap continues.

/// The source a structural edit adds in the anchored-seek cases.
const ADDED_SOURCE: &str = "added-source";
/// The audible track on [`ADDED_SOURCE`].
const ADDED_TRACK: &str = "added";

/// Session A with one track, `eq0`, on [`SOURCE`].
fn one_track_session() -> SessionModel {
    let mut model = session_a();
    remove_track(&mut model, "eq1");
    model
}

/// [`one_track_session`] plus [`ADDED_SOURCE`], an audible track on it and its route.
fn added_source_session() -> SessionModel {
    let mut model = one_track_session();
    add_source(&mut model, ADDED_SOURCE);
    add_track(&mut model, "eq0", ADDED_TRACK, ADDED_SOURCE, false);
    model
}

/// Submit `feed`'s PCM for blocks `blocks` to `id` at `generation`, from source frame
/// `blocks.start * QUANTUM` on.
fn submit_blocks(
    sources: &mut SourceControlSet,
    id: &str,
    feed: &Feed,
    generation: u64,
    blocks: core::ops::Range<usize>,
) {
    for block in blocks {
        let range = block * QUANTUM..(block + 1) * QUANTUM;
        sources
            .submit(
                id.as_bytes(),
                SourceSubmission {
                    generation,
                    start_frame: range.start as u64,
                    sample_rate_hz: 48_000,
                    planes: &[&feed.planes[0][range.clone()], &feed.planes[1][range]],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .unwrap_or_else(|error| {
                panic!("{id} generation {generation} block {block}: {error:?}")
            });
    }
}

/// A reference feed for `id` whose block `b` carries `feed`'s block `source_block(b)`, or exact
/// zeros where that is `None`. [`reference_run`] submits it at generation 1 from frame 0.
fn spliced(id: &'static str, feed: &Feed, source_block: impl Fn(usize) -> Option<usize>) -> Feed {
    let plane = |channel: usize| {
        (0..BLOCKS)
            .flat_map(|block| match source_block(block) {
                Some(from) => feed.planes[channel][from * QUANTUM..(from + 1) * QUANTUM].to_vec(),
                None => vec![0.0; QUANTUM],
            })
            .collect::<Vec<f32>>()
    };
    Feed {
        id,
        planes: [plane(0), plane(1)],
    }
}

fn render_block(owner: &mut engine::realtime::RealtimePlanOwner) -> successor::Block {
    use bench_support::alloc as bench_alloc;
    use engine::realtime::{PlanarBufferMut, RenderIo, audit};
    let mut output = [f32::NAN; QUANTUM * 2];
    let sample = owner.next_absolute_sample();
    audit::reset();
    let mark = bench_alloc::current_thread_counters();
    let report = owner
        .render_contiguous(
            RenderIo {
                output: PlanarBufferMut::try_new(&mut output, 2, QUANTUM, QUANTUM).expect("output"),
            },
            sample,
        )
        .unwrap_or_else(|error| panic!("render: {error:?}"));
    let delta = bench_alloc::current_thread_delta_since(mark);
    let snapshot = audit::snapshot();
    successor::Block {
        bits: output.iter().map(|sample| sample.to_bits()).collect(),
        swap: report.swap,
        carry: report.carry,
        allocator: (delta.allocations, delta.reallocations, delta.deallocations),
        audit: (snapshot.allocations, snapshot.deallocations),
    }
}

type Exchange = (
    engine::realtime::PlanPublisher,
    engine::realtime::RealtimePlanOwner,
    engine::realtime::PlanRetirer,
);

fn exchange(plan: engine::realtime::PreparedRenderPlan) -> Exchange {
    use core::num::NonZeroUsize;
    engine::realtime::plan_exchange(
        plan,
        engine::realtime::PlanExchangeConfig {
            publication_capacity: NonZeroUsize::MIN,
            retirement_capacity: NonZeroUsize::MIN,
        },
    )
    .unwrap_or_else(|_| panic!("plan exchange"))
}

/// Render [`one_track_session`] fed `kept` for [`SWAP_BLOCK`] blocks, swap in
/// [`added_source_session`] as its successor, and render to [`BLOCKS`]. Before block
/// `seek_before` (at or after the swap block) renders, `seek_at(ADDED_SOURCE, 2, A, A)` with
/// `A = anchor_block * QUANTUM`, then submit `added` at generation 2 from frame `A` to the end.
fn anchored_swap_run(
    kept: &Feed,
    added: &Feed,
    anchor_block: usize,
    seek_before: usize,
    backend: Backend,
) -> Vec<successor::Block> {
    assert!(seek_before >= SWAP_BLOCK);
    let a = Session::compile(one_track_session());
    let b = Session::compile(added_source_session());
    let predecessor = a.prepare(backend);
    let mut a_sources = predecessor.sources;
    let inventory = predecessor.inventory;
    let (mut publisher, mut owner, _retirer) = exchange(predecessor.plan);
    let mut blocks = Vec::with_capacity(BLOCKS);
    submit_blocks(&mut a_sources, SOURCE, kept, 1, 0..1);
    for block in 0..SWAP_BLOCK {
        submit_blocks(&mut a_sources, SOURCE, kept, 1, block + 1..block + 2);
        blocks.push(render_block(&mut owner));
    }
    let mut prepared = b
        .prepare_successor(&inventory, &a.model, &caps(), backend)
        .unwrap_or_else(|failure| panic!("successor: {failure:?}"));
    assert_eq!(prepared.sources.adopt_persisting(&mut a_sources), 1);
    let mut b_sources = prepared.sources;
    publisher
        .reserve_replacement(prepared.plan)
        .unwrap_or_else(|_| panic!("reserve the successor"))
        .commit();
    let anchor = (anchor_block * QUANTUM) as u64;
    for block in SWAP_BLOCK..BLOCKS {
        if block + 1 < BLOCKS {
            submit_blocks(&mut b_sources, SOURCE, kept, 1, block + 1..block + 2);
        }
        if block == seek_before {
            b_sources
                .seek_at(ADDED_SOURCE.as_bytes(), 2, anchor, anchor)
                .expect("anchored seek");
            submit_blocks(&mut b_sources, ADDED_SOURCE, added, 2, anchor_block..BLOCKS);
        }
        blocks.push(render_block(&mut owner));
    }
    assert_eq!(blocks[SWAP_BLOCK].carry, CarryOutcome::Carried);
    blocks
}

/// Gate 1 of #1274, at one width: the anchor is three blocks past the next render and the ring is
/// primed from it at once.
fn future_anchor_starts_the_added_stem_on_its_block(backend: Backend) {
    const ANCHOR_BLOCK: usize = SWAP_BLOCK + 4;
    let kept = Feed::new(SOURCE, 1);
    let added = Feed::new(ADDED_SOURCE, 2);
    let b = Session::compile(added_source_session());
    let reference_added = spliced(ADDED_SOURCE, &added, |block| {
        (block >= ANCHOR_BLOCK).then_some(block)
    });
    let reference = reference_run(&b, &[&kept, &reference_added], backend);
    let run = anchored_swap_run(&kept, &added, ANCHOR_BLOCK, SWAP_BLOCK + 1, backend);
    assert_eq!(
        first_difference(&run, &reference),
        None,
        "{backend:?}: the added stem starts exactly at its anchor block"
    );
    // The oracle sees the stem: without it the anchor block differs.
    let silent = reference_run(
        &b,
        &[&kept, &spliced(ADDED_SOURCE, &added, |_| None)],
        backend,
    );
    assert_eq!(first_difference(&silent, &reference), Some(ANCHOR_BLOCK));
}

/// Gate 1 of #1274 at eight lanes. Red if a held seek discards the primed generation (the stem is
/// silent at the anchor) or applies when observed (the stem starts early).
#[cfg(target_feature = "avx2")]
#[test]
fn a_future_anchor_starts_the_added_stem_on_its_block_at_eight_lanes() {
    future_anchor_starts_the_added_stem_on_its_block(Backend::Simd8);
}

/// Gate 1 of #1274 at four lanes.
#[test]
fn a_future_anchor_starts_the_added_stem_on_its_block_at_four_lanes() {
    future_anchor_starts_the_added_stem_on_its_block(Backend::Simd4);
}

/// Gate 2 of #1274: the anchor is two blocks before the swap block, so the render observes the
/// seek late and starts the stem at the swap block's own frame. Red if the lateness is measured
/// on any clock but the plan's absolute render sample, or not added at all.
#[test]
fn a_past_anchor_starts_the_added_stem_in_time() {
    const ANCHOR_BLOCK: usize = SWAP_BLOCK - 2;
    let kept = Feed::new(SOURCE, 1);
    let added = Feed::new(ADDED_SOURCE, 2);
    let b = Session::compile(added_source_session());
    let reference_added = spliced(ADDED_SOURCE, &added, |block| {
        (block >= SWAP_BLOCK).then_some(block)
    });
    for backend in backends() {
        let reference = reference_run(&b, &[&kept, &reference_added], backend);
        let run = anchored_swap_run(&kept, &added, ANCHOR_BLOCK, SWAP_BLOCK, backend);
        assert_eq!(
            first_difference(&run, &reference),
            None,
            "{backend:?}: the stem is in time from the swap block on"
        );
        assert!(reference[SWAP_BLOCK].bits.iter().any(|bits| *bits != 0));
    }
}

/// Gate 3 of #1274: two playing sources, each given `seek_at` to one anchor before a different
/// render, both jump in the anchor block. Each keeps playing its queued old generation until then.
/// Red if a seek applies when observed, if a held seek drops the old generation's queued PCM, or
/// if it discards the new generation's primed block.
#[test]
fn two_playing_sources_move_to_one_block() {
    const SECOND_SOURCE: &str = "second-source";
    const ANCHOR_BLOCK: usize = 8;
    /// Both sources jump to this source block.
    const TARGET_BLOCK: usize = 2;
    let mut model = session_a();
    add_source(&mut model, SECOND_SOURCE);
    model
        .tracks
        .iter_mut()
        .find(|track| track.id.as_str() == "eq1")
        .expect("eq1")
        .source_id = id(SECOND_SOURCE);
    let session = Session::compile(model);
    let first = Feed::new(SOURCE, 1);
    let second = Feed::new(SECOND_SOURCE, 3);
    let jump = |block: usize| {
        Some(if block < ANCHOR_BLOCK {
            block
        } else {
            TARGET_BLOCK + block - ANCHOR_BLOCK
        })
    };
    let anchor = (ANCHOR_BLOCK * QUANTUM) as u64;
    for backend in backends() {
        let reference = reference_run(
            &session,
            &[
                &spliced(SOURCE, &first, jump),
                &spliced(SECOND_SOURCE, &second, jump),
            ],
            backend,
        );
        let prepared = session.prepare(backend);
        let mut sources = prepared.sources;
        let (_publisher, mut owner, _retirer) = exchange(prepared.plan);
        // The old generation is queued up to the anchor before anything renders.
        submit_blocks(&mut sources, SOURCE, &first, 1, 0..ANCHOR_BLOCK);
        submit_blocks(&mut sources, SECOND_SOURCE, &second, 1, 0..ANCHOR_BLOCK);
        let tail = TARGET_BLOCK..TARGET_BLOCK + BLOCKS - ANCHOR_BLOCK;
        let mut blocks = Vec::with_capacity(BLOCKS);
        for block in 0..BLOCKS {
            for (moment, id, feed) in [(3, SOURCE, &first), (6, SECOND_SOURCE, &second)] {
                if block == moment {
                    let target = (TARGET_BLOCK * QUANTUM) as u64;
                    sources
                        .seek_at(id.as_bytes(), 2, target, anchor)
                        .expect("anchored seek");
                    submit_blocks(&mut sources, id, feed, 2, tail.clone());
                }
            }
            blocks.push(render_block(&mut owner));
        }
        assert_eq!(
            first_difference(&blocks, &reference),
            None,
            "{backend:?}: both sources jump in the anchor block"
        );
    }
}

/// Gate 5 of #1274: the blocks that hold, then apply, an anchored seek -- observed early and
/// observed late -- make no allocator call. Red if holding or applying the seek, or keeping the
/// pending block, allocates or frees on the render thread.
#[test]
fn holding_and_applying_an_anchored_seek_allocates_nothing() {
    use bench_support::alloc::{Mode, assert_installed, mode, set_mode};
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
    let kept = Feed::new(SOURCE, 1);
    let added = Feed::new(ADDED_SOURCE, 2);
    for (anchor_block, seek_before) in [
        (SWAP_BLOCK + 4, SWAP_BLOCK + 1),
        (SWAP_BLOCK - 2, SWAP_BLOCK),
    ] {
        let run = anchored_swap_run(&kept, &added, anchor_block, seek_before, Backend::current());
        for (index, block) in run.iter().enumerate().skip(1) {
            assert_eq!(block.allocator, (0, 0, 0), "block {index}: allocator calls");
            assert_eq!(block.audit, (0, 0), "block {index}: render audit");
        }
    }
}

/// D5 of #1274: `seek_at` keeps `seek`'s region and generation rules and refuses an anchor no
/// block starts at.
#[test]
fn seek_at_refuses_an_unaligned_anchor() {
    let session = Session::compile(one_track_session());
    let mut sources = session.prepare(Backend::current()).sources;
    let refused = sources
        .seek_at(SOURCE.as_bytes(), 2, 0, QUANTUM as u64 + 1)
        .expect_err("unaligned anchor");
    assert_eq!(refused.diagnostic(), "source.seek.anchor_unaligned");
    assert_eq!(
        sources.seek_at(SOURCE.as_bytes(), 2, 48_001, 0),
        Err(SourceControlError::OutsideRegion)
    );
    assert_eq!(
        sources.seek_at(SOURCE.as_bytes(), 0, 0, 0),
        Err(SourceControlError::GenerationZero)
    );
    // The refusals left generation 1 active: the same seek, aligned, is accepted.
    sources
        .seek_at(SOURCE.as_bytes(), 2, 0, QUANTUM as u64)
        .expect("aligned anchor");
}

// Issue #1276 (slice 7 of #1269): a strip input section the transaction left unchanged -- owner
// key `(strip ID, PostInputBuiltins)` -- keeps its exact state through the swap: the filter
// integrators, the coefficients in use and any coefficient ramp, and the trim and polarity ramp.
// Lanes are ordered by strip ID, so the added muted track, which sorts first, moves every kept
// lane: a copy keyed by lane or index lands on the wrong strip.

/// The nine-track fixture with both console sections and every insert rack empty, every track's
/// high-pass and low-pass enabled at its own cutoffs and a nonzero trim. `mono` maps both channels
/// of every track to the source's left channel and makes every per-channel value equal, so the
/// banks collapse; otherwise the cutoffs and trims differ per channel and track `eq3`'s right
/// channel is polarity-inverted.
fn filtered_session(mono: bool) -> SessionModel {
    let mut model = parse_session_json(FIXTURE).expect("fixture parses");
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    for (index, track) in model.tracks.iter_mut().enumerate() {
        track.console.clear();
        track.inserts.effects.clear();
        if mono {
            track.right_source_channel = track.left_source_channel;
        }
        let step = index as f32;
        for (channel, builtins) in [&mut track.builtins.left, &mut track.builtins.right]
            .into_iter()
            .enumerate()
        {
            let side = if mono { 0.0 } else { channel as f32 };
            builtins.hpf_hz = 30.0 + 11.0 * step + 7.0 * side;
            builtins.lpf_hz = 9_000.0 - 450.0 * step - 600.0 * side;
            builtins.trim_db = -2.25 + 0.5 * step + 0.125 * side;
            builtins.polarity_invert = !mono && index == 3 && channel == 1;
        }
    }
    model
}

/// `model` plus [`MUTED_TRACK`], a muted copy of `eq0` that sorts first.
fn with_muted_track(mut model: SessionModel) -> SessionModel {
    add_track(&mut model, "eq0", MUTED_TRACK, SOURCE, true);
    model
}

/// Gate 1 of #1276, at one width.
fn filtered_strips_keep_their_state(backend: Backend) {
    let a = Session::compile(filtered_session(false));
    let b = Session::compile(with_muted_track(filtered_session(false)));
    assert_eq!(
        b.compiled.normalized_model().tracks[0].id.as_str(),
        MUTED_TRACK
    );
    let feed = Feed::new(SOURCE, 3);
    let reference = reference_run(&b, &[&feed], backend);
    let run = swapped_run(&a, &b, &[&feed], &[&feed], backend, Successor::Carry);
    assert_eq!(
        (run.blocks[SWAP_BLOCK].swap, run.blocks[SWAP_BLOCK].carry),
        (SwapOutcome::Applied, CarryOutcome::Carried),
        "{backend:?}: the swap block"
    );
    assert_eq!(
        first_difference(&run.blocks, &reference),
        None,
        "{backend:?}: every block equals the post-edit session rendered from frame 0"
    );
    assert!(reference[SWAP_BLOCK].bits.iter().any(|bits| *bits != 0));
    // Nine input sections recorded, and all nine carry: one source move and nine lane moves.
    assert_eq!(a.prepare(backend).inventory.input_section_count(), 9);
    assert_eq!(
        run.successor.carry_program_retained_bytes,
        8 + 9 * core::mem::size_of::<graph::GraphLaneMove>() as u64
    );
}

/// Gate 1 at eight lanes: banks of eight and one become eight and two, so every kept lane moves
/// and two cross into the next bank. Red if a filter restarts at rest, or a lane is imported at
/// its old lane index instead of its strip's new one.
#[cfg(target_feature = "avx2")]
#[test]
fn filtered_strips_keep_their_state_across_a_swap_at_eight_lanes() {
    filtered_strips_keep_their_state(Backend::Simd8);
}

/// Gate 1 at four lanes: banks of four, four and one become four, four and two.
#[test]
fn filtered_strips_keep_their_state_across_a_swap_at_four_lanes() {
    filtered_strips_keep_their_state(Backend::Simd4);
}

/// A live input record pushed into a run's queue for `strip` before block `block` renders.
struct LiveWrite {
    block: usize,
    strip: &'static str,
    record: builtins_compiler::TrackInputRecord,
}

/// One plan prepared with a live-control queue on every strip, and its handles.
struct LivePlan {
    prepared: host_core::PreparedHost,
    handles: host_core::HostLiveControlHandles,
}

impl LivePlan {
    fn request() -> host_core::HostLiveControlRequest {
        host_core::HostLiveControlRequest {
            control_queue_depth: Some(core::num::NonZeroUsize::new(8).expect("depth")),
            ..host_core::HostLiveControlRequest::default()
        }
    }

    fn fresh(session: &Session, backend: Backend) -> Self {
        let (prepared, handles) = host_core::test_only_prepare_host_runtime_with_live_controls_on(
            &session.compiled,
            &caps(),
            &Self::request(),
            backend,
        )
        .unwrap_or_else(|failure| panic!("prepare: {failure:?}"));
        Self { prepared, handles }
    }

    fn successor(
        session: &Session,
        predecessor: &host_core::PlanStateInventory,
        committed: &SessionModel,
        backend: Backend,
    ) -> Self {
        let (prepared, handles) =
            host_core::test_only_prepare_host_runtime_with_live_controls_successor_on(
                &session.compiled,
                &caps(),
                &Self::request(),
                host_core::SuccessorBase {
                    inventory: predecessor,
                    committed,
                },
                backend,
            )
            .unwrap_or_else(|failure| panic!("successor: {failure:?}"));
        Self { prepared, handles }
    }

    fn push(&mut self, write: &LiveWrite) {
        let index = self
            .handles
            .strips
            .iter()
            .position(|strip| &**strip == write.strip)
            .expect("strip");
        self.handles.strip_controls[index]
            .input
            .as_mut()
            .expect("input lane")
            .try_push(write.record)
            .unwrap_or_else(|_| panic!("queue room"));
    }

    fn submit(&mut self, feed: &Feed, quantum: usize, block: usize) {
        let range = block * quantum..(block + 1) * quantum;
        self.prepared
            .sources
            .submit(
                SOURCE.as_bytes(),
                SourceSubmission {
                    generation: 1,
                    start_frame: range.start as u64,
                    sample_rate_hz: 48_000,
                    planes: &[&feed.planes[0][range.clone()], &feed.planes[1][range]],
                    frames: quantum as u32,
                    end_of_region: false,
                },
            )
            .unwrap_or_else(|error| panic!("block {block}: {error:?}"));
    }

    fn render(&mut self, quantum: usize) -> Vec<u32> {
        let mut output = vec![f32::NAN; quantum * 2];
        self.render_into(&mut output, quantum);
        output.iter().map(|sample| sample.to_bits()).collect()
    }

    /// Render one block into `output`, which holds `quantum * 2` samples. Allocation-free.
    fn render_into(&mut self, output: &mut [f32], quantum: usize) {
        let sample = self.prepared.plan.next_absolute_sample();
        self.prepared
            .plan
            .render_contiguous(
                engine::realtime::RenderIo {
                    output: engine::realtime::PlanarBufferMut::try_new(output, 2, quantum, quantum)
                        .expect("output"),
                },
                sample,
            )
            .unwrap_or_else(|error| panic!("render: {error:?}"));
    }
}

/// What a direct run (a synchronous host: the successor adopts its predecessor itself) hands back.
struct DirectRun {
    blocks: Vec<Vec<u32>>,
    carry: CarryOutcome,
    /// `(allocations, reallocations, deallocations)` and the render audit's `(allocations,
    /// deallocations)` over the hand-over plus the successor's first block.
    swap_allocator: ((u64, u64, u64), (u64, u64)),
    /// `bank_collapse_counters()` of the predecessor at the swap and of the last plan at the end.
    collapses: [[u64; 2]; 2],
    /// `bank_collapse_transitions()` of the last plan at the end.
    transitions: [u64; 3],
}

/// Render `a` for [`SWAP_BLOCK`] blocks of `quantum` frames, then -- when `b` is given -- prepare
/// it as `a`'s successor from `committed`, hand over, and render to [`BLOCKS`]. `writes` go to
/// whichever plan renders their block next, so a write for the swap block is pending in `a`'s
/// queue at the hand-over. `successor_dual` forces the successor's mono collapse off.
fn direct_run(
    a: &Session,
    b: Option<(&Session, &SessionModel)>,
    feed: &Feed,
    quantum: usize,
    writes: &[LiveWrite],
    backend: Backend,
    successor_dual: bool,
) -> DirectRun {
    use bench_support::alloc as bench_alloc;
    let mut plan = LivePlan::fresh(a, backend);
    let mut blocks = Vec::with_capacity(BLOCKS);
    let mut carry = CarryOutcome::NotRequested;
    let mut swap_allocator = ((0, 0, 0), (0, 0));
    let mut collapses = [[0; 2]; 2];
    plan.submit(feed, quantum, 0);
    for block in 0..BLOCKS {
        for write in writes.iter().filter(|write| write.block == block) {
            plan.push(write);
        }
        let swap = block == SWAP_BLOCK && b.is_some();
        if let (true, Some((session, committed))) = (swap, b) {
            collapses[0] = plan.prepared.plan.bank_collapse_counters();
            let mut successor =
                LivePlan::successor(session, &plan.prepared.inventory, committed, backend);
            successor
                .prepared
                .plan
                .force_mono_collapse_off(successor_dual);
            assert_eq!(
                successor
                    .prepared
                    .sources
                    .adopt_persisting(&mut plan.prepared.sources),
                1
            );
            if block + 1 < BLOCKS {
                successor.submit(feed, quantum, block + 1);
            }
            let mut output = vec![f32::NAN; quantum * 2];
            engine::realtime::audit::reset();
            let mark = bench_alloc::current_thread_counters();
            carry = successor
                .prepared
                .plan
                .adopt_predecessor_plan(&mut plan.prepared.plan);
            successor.render_into(&mut output, quantum);
            let delta = bench_alloc::current_thread_delta_since(mark);
            let audit = engine::realtime::audit::snapshot();
            swap_allocator = (
                (delta.allocations, delta.reallocations, delta.deallocations),
                (audit.allocations, audit.deallocations),
            );
            blocks.push(output.iter().map(|sample| sample.to_bits()).collect());
            plan = successor;
            continue;
        }
        if block + 1 < BLOCKS {
            plan.submit(feed, quantum, block + 1);
        }
        blocks.push(plan.render(quantum));
    }
    collapses[1] = plan.prepared.plan.bank_collapse_counters();
    DirectRun {
        blocks,
        carry,
        swap_allocator,
        collapses,
        transitions: plan.prepared.plan.bank_collapse_transitions(),
    }
}

/// [`filtered_session`] with every track at `quantum` frames.
fn at_quantum(mut model: SessionModel, quantum: usize) -> SessionModel {
    model.quantum_frames = quantum as u32;
    model
}

/// The first block whose bits differ between two direct runs.
fn first_direct_difference(left: &DirectRun, right: &DirectRun) -> Option<usize> {
    left.blocks
        .iter()
        .zip(&right.blocks)
        .position(|(left, right)| left != right)
}

/// Gate 2 of #1276, first case: a trim record admitted to A after block 5, committed in the model
/// B is prepared from, is pending in A's queue at the swap. The carry drains it into the lane
/// before copying, so B's first block starts the trim ramp exactly where A would have.
fn a_pending_trim_record_survives_the_swap(backend: Backend) {
    const NEW_TRIM_DB: f32 = 4.5;
    let record = builtins_compiler::TrackInputRecord::TrimDb {
        lanes: builtins::BuiltinLaneSelector::Both,
        db: NEW_TRIM_DB,
        smoothing_samples: 300,
    };
    let write = |block| LiveWrite {
        block,
        strip: "eq2",
        record,
    };
    let a = Session::compile(filtered_session(false));
    let mut committed = filtered_session(false);
    let strip = committed
        .tracks
        .iter_mut()
        .find(|track| track.id.as_str() == "eq2")
        .expect("eq2");
    strip.builtins.left.trim_db = NEW_TRIM_DB;
    strip.builtins.right.trim_db = NEW_TRIM_DB;
    let b = Session::compile(with_muted_track(committed.clone()));
    let reference_session = Session::compile(with_muted_track(filtered_session(false)));
    let feed = Feed::new(SOURCE, 5);
    let writes = [write(SWAP_BLOCK)];
    let reference = direct_run(
        &reference_session,
        None,
        &feed,
        QUANTUM,
        &writes,
        backend,
        false,
    );
    let run = direct_run(
        &a,
        Some((&b, &committed)),
        &feed,
        QUANTUM,
        &writes,
        backend,
        false,
    );
    assert_eq!(run.carry, CarryOutcome::Carried, "{backend:?}");
    assert_eq!(
        first_direct_difference(&run, &reference),
        None,
        "{backend:?}: the record takes effect on the swap block's first sample"
    );
    // The record moved the output: without it the swap block differs.
    let unwritten = direct_run(
        &reference_session,
        None,
        &feed,
        QUANTUM,
        &[],
        backend,
        false,
    );
    assert_eq!(
        first_direct_difference(&unwritten, &reference),
        Some(SWAP_BLOCK)
    );
}

/// Gate 2 at eight lanes. Red if the carry exports a lane before draining its queue: the record
/// is lost with the retired plan.
#[cfg(target_feature = "avx2")]
#[test]
fn a_pending_trim_record_survives_the_swap_at_eight_lanes() {
    a_pending_trim_record_survives_the_swap(Backend::Simd8);
}

/// Gate 2 at four lanes.
#[test]
fn a_pending_trim_record_survives_the_swap_at_four_lanes() {
    a_pending_trim_record_survives_the_swap(Backend::Simd4);
}

/// Gate 2 of #1276, second case: at a 32-frame quantum, a high-pass target admitted before block
/// 5 starts its 64-sample coefficient ramp in A's block 5, so the ramp is half done at the swap.
/// B's first block must finish it from the carried coefficients, target, step and countdown.
fn a_high_pass_ramp_in_flight_finishes_after_the_swap(backend: Backend) {
    const QUANTUM_32: usize = 32;
    const NEW_HPF_HZ: f32 = 210.0;
    let base = at_quantum(filtered_session(false), QUANTUM_32);
    let track = base
        .tracks
        .iter()
        .find(|track| track.id.as_str() == "eq4")
        .expect("eq4")
        .clone();
    let mut writes = Vec::new();
    for (lanes, channel) in [
        (builtins::BuiltinLaneSelector::Left, &track.builtins.left),
        (builtins::BuiltinLaneSelector::Right, &track.builtins.right),
    ] {
        let mut target = builtins::prepare_input_filter_pair(48_000, NEW_HPF_HZ, channel.lpf_hz)
            .expect("designed")
            .targets[0];
        target.lanes = lanes;
        writes.push(LiveWrite {
            block: SWAP_BLOCK - 1,
            strip: "eq4",
            record: builtins_compiler::TrackInputRecord::PreparedFilter { target },
        });
    }
    let a = Session::compile(base.clone());
    let mut committed = base.clone();
    let strip = committed
        .tracks
        .iter_mut()
        .find(|track| track.id.as_str() == "eq4")
        .expect("eq4");
    strip.builtins.left.hpf_hz = NEW_HPF_HZ;
    strip.builtins.right.hpf_hz = NEW_HPF_HZ;
    let b = Session::compile(with_muted_track(committed.clone()));
    let reference_session = Session::compile(with_muted_track(base));
    let feed = Feed::new(SOURCE, 7);
    let reference = direct_run(
        &reference_session,
        None,
        &feed,
        QUANTUM_32,
        &writes,
        backend,
        false,
    );
    let run = direct_run(
        &a,
        Some((&b, &committed)),
        &feed,
        QUANTUM_32,
        &writes,
        backend,
        false,
    );
    assert_eq!(run.carry, CarryOutcome::Carried, "{backend:?}");
    assert_eq!(
        first_direct_difference(&run, &reference),
        None,
        "{backend:?}: the ramp in flight finishes after the swap"
    );
}

/// Gate 2, ramp case, at eight lanes. Red if the coefficient target, step or countdown is not
/// carried: the successor's lane would jump to its prepared coefficients or freeze mid-ramp.
#[cfg(target_feature = "avx2")]
#[test]
fn a_high_pass_ramp_in_flight_finishes_after_the_swap_at_eight_lanes() {
    a_high_pass_ramp_in_flight_finishes_after_the_swap(Backend::Simd8);
}

/// Gate 2, ramp case, at four lanes.
#[test]
fn a_high_pass_ramp_in_flight_finishes_after_the_swap_at_four_lanes() {
    a_high_pass_ramp_in_flight_finishes_after_the_swap(Backend::Simd4);
}

/// Gate 3 of #1276, at one width. Every track reads one source channel, so A's chains collapse,
/// and B adds a muted mono track that sorts first. The successor's chains keep collapsing on
/// every block after the swap, as the reference's do, and engage on the agreement they inherit
/// rather than on a proof. With `successor_dual` the successor's collapse is forced off, so every
/// carried lane's right channel is read from the swap block on: it must hold the state the
/// collapsed predecessor never wrote there until the carry's disengage copy.
fn collapsed_strips_keep_collapsing(backend: Backend, successor_dual: bool) {
    let a = Session::compile(filtered_session(true));
    let b = Session::compile(with_muted_track(filtered_session(true)));
    let feed = Feed::new(SOURCE, 9);
    let reference = direct_run(&b, None, &feed, QUANTUM, &[], backend, false);
    let run = direct_run(
        &a,
        Some((&b, &a.model)),
        &feed,
        QUANTUM,
        &[],
        backend,
        successor_dual,
    );
    assert_eq!(run.carry, CarryOutcome::Carried, "{backend:?}");
    assert!(
        run.collapses[0][0] > 0,
        "{backend:?}: the predecessor rendered collapsed before the swap"
    );
    assert_eq!(
        first_direct_difference(&run, &reference),
        None,
        "{backend:?}, successor dual {successor_dual}"
    );
    assert!(reference.collapses[1][0] > 0);
    if successor_dual {
        assert_eq!(run.collapses[1][0], 0, "{backend:?}: forced dual");
    } else {
        // Every chain that collapses in the reference collapses on every block after the swap,
        // and none needed an agreement proof to engage.
        assert_eq!(
            run.collapses[1][0] * BLOCKS as u64,
            reference.collapses[1][0] * (BLOCKS - SWAP_BLOCK) as u64,
            "{backend:?}: successor collapsed blocks"
        );
        assert_eq!(run.transitions[2], 0, "{backend:?}: agreement proofs");
    }
}

/// Gate 3 at eight lanes. Red if a successor chain that receives carried lanes clears its
/// channel-agreement flag (it needs a proof to engage, and a chain with an effect in its prefix,
/// which declines the proof, would never collapse again), or if the carry skips the disengage
/// copy of a collapsed predecessor (the dual successor reads a stale right channel).
#[cfg(target_feature = "avx2")]
#[test]
fn collapsed_strips_keep_collapsing_across_a_swap_at_eight_lanes() {
    collapsed_strips_keep_collapsing(Backend::Simd8, false);
    collapsed_strips_keep_collapsing(Backend::Simd8, true);
}

/// Gate 3 at four lanes.
#[test]
fn collapsed_strips_keep_collapsing_across_a_swap_at_four_lanes() {
    collapsed_strips_keep_collapsing(Backend::Simd4, false);
    collapsed_strips_keep_collapsing(Backend::Simd4, true);
}

/// Gate 4 of #1276, one edit. B applies `edit` to `eq0`'s input section in the same transaction
/// that adds a muted track, so `eq0`'s section does not carry (Q1's default): from the swap block
/// on it renders exactly what a fresh plan of B renders from rest on the same PCM.
fn a_changed_section_starts_at_rest(name: &str, edit: fn(&mut session::DualMonoBuiltins)) {
    let one_track = |model: SessionModel| {
        let mut model = model;
        model.tracks.retain(|track| track.id.as_str() == "eq0");
        model.routes.retain(|route| route.id.as_str() == "eq0-main");
        model
    };
    let a = Session::compile(one_track(filtered_session(false)));
    let mut b_model = one_track(filtered_session(false));
    edit(&mut b_model.tracks[0].builtins);
    let b = Session::compile(with_muted_track(b_model));
    let feed = Feed::new(SOURCE, 11);
    for backend in backends() {
        let before = reference_run(&a, &[&feed], backend);
        let tail_feed = spliced(SOURCE, &feed, |block| {
            (block + SWAP_BLOCK < BLOCKS).then_some(block + SWAP_BLOCK)
        });
        let at_rest = reference_run(&b, &[&tail_feed], backend);
        let run = swapped_run(&a, &b, &[&feed], &[&feed], backend, Successor::Carry);
        assert_eq!(run.blocks[SWAP_BLOCK].carry, CarryOutcome::Carried);
        assert_eq!(
            run.successor.carry_program_retained_bytes,
            core::mem::size_of::<(u32, u32)>() as u64,
            "{name}, {backend:?}: the source moves, the changed section does not"
        );
        for block in 0..BLOCKS {
            let expected = if block < SWAP_BLOCK {
                &before[block].bits
            } else {
                &at_rest[block - SWAP_BLOCK].bits
            };
            assert_eq!(
                &run.blocks[block].bits, expected,
                "{name}, {backend:?}: block {block}"
            );
        }
    }
}

/// Gate 4 for each value D1 compares. Red if the comparison of the changed value is dropped: the
/// section carries, the old high-pass or low-pass keeps running, or the old trim or polarity
/// stays on the strip after an acknowledged edit.
#[test]
fn a_strip_whose_input_section_changed_starts_at_rest() {
    a_changed_section_starts_at_rest("hpf_hz", |builtins| {
        builtins.left.hpf_hz = 160.0;
        builtins.right.hpf_hz = 170.0;
    });
    a_changed_section_starts_at_rest("lpf_hz", |builtins| {
        builtins.right.lpf_hz = 3_500.0;
    });
    a_changed_section_starts_at_rest("trim_db", |builtins| {
        builtins.left.trim_db += 1.5;
    });
    a_changed_section_starts_at_rest("polarity_invert", |builtins| {
        builtins.right.polarity_invert = true;
    });
}

/// D1's control-kind clause: a successor that attaches a live queue to its strips when the
/// predecessor did not, or the reverse, carries no input section; only the source moves. Red if
/// the clause is dropped (nine lane moves).
#[test]
fn a_changed_control_kind_carries_no_input_section() {
    let a = Session::compile(filtered_session(false));
    let b = Session::compile(with_muted_track(filtered_session(false)));
    for backend in backends() {
        a_changed_control_kind_carries_no_input_section_at(&a, &b, backend);
    }
}

fn a_changed_control_kind_carries_no_input_section_at(a: &Session, b: &Session, backend: Backend) {
    let source_only = core::mem::size_of::<(u32, u32)>() as u64;
    // Predecessor without live controls, successor with them.
    let plain = a.prepare(backend);
    let live = LivePlan::successor(b, &plain.inventory, &a.model, backend);
    assert_eq!(
        live.prepared.report.carry_program_retained_bytes, source_only,
        "plain -> live"
    );
    // Predecessor with live controls, successor without them.
    let live = LivePlan::fresh(a, backend);
    let plain = b
        .prepare_successor(&live.prepared.inventory, &a.model, &caps(), backend)
        .unwrap_or_else(|failure| panic!("successor: {failure:?}"));
    assert_eq!(
        plain.report.carry_program_retained_bytes, source_only,
        "live -> plain"
    );
    // The control: the same kind on both sides carries all nine sections.
    let plain = b
        .prepare_successor(&a.prepare(backend).inventory, &a.model, &caps(), backend)
        .unwrap_or_else(|failure| panic!("successor: {failure:?}"));
    assert_eq!(
        plain.report.carry_program_retained_bytes,
        source_only + 9 * core::mem::size_of::<graph::GraphLaneMove>() as u64
    );
}

/// D1's strip-kind clause: B removes track `eq0` and adds a submix named `eq0`, with `eq0`'s input
/// section, that `eq1` feeds. The submix's section is not the track's and does not carry; `eq1`'s
/// does. Red if a strip is looked up by ID alone, whatever its kind (two lane moves).
#[test]
fn a_track_replaced_by_a_submix_of_its_name_does_not_carry() {
    let two_tracks = || {
        let mut model = filtered_session(false);
        model
            .tracks
            .retain(|track| matches!(track.id.as_str(), "eq0" | "eq1"));
        model
            .routes
            .retain(|route| matches!(route.id.as_str(), "eq0-main" | "eq1-main"));
        model
    };
    let a = Session::compile(two_tracks());
    let mut b_model = two_tracks();
    let track = b_model.tracks.remove(0);
    assert_eq!(track.id.as_str(), "eq0");
    let mut submix = session::Submix::unity(id("eq0"), &b_model.console);
    submix.builtins = track.builtins.clone();
    b_model.submixes.push(submix);
    let template = b_model
        .routes
        .iter()
        .position(|route| route.id.as_str() == "eq0-main")
        .expect("eq0-main");
    let mut out = b_model.routes.remove(template);
    let RouteSource::Track { tap, .. } = out.source else {
        panic!("a track route");
    };
    out.source = RouteSource::Submix {
        submix_id: id("eq0"),
        tap,
    };
    let feed_route = b_model
        .routes
        .iter_mut()
        .find(|route| route.id.as_str() == "eq1-main")
        .expect("eq1-main");
    feed_route.destination = session::RouteDestination::SubmixInput {
        submix_id: id("eq0"),
    };
    b_model.routes.push(out);
    let b = Session::compile(b_model);
    for backend in backends() {
        let prepared = b
            .prepare_successor(&a.prepare(backend).inventory, &a.model, &caps(), backend)
            .unwrap_or_else(|failure| panic!("successor: {failure:?}"));
        assert_eq!(
            prepared.report.carry_program_retained_bytes,
            8 + core::mem::size_of::<graph::GraphLaneMove>() as u64,
            "{backend:?}: the source and eq1's section move, the submix's section does not"
        );
    }
}

/// Gate 5 of #1276. The hand-over that drains a pending record, disengages collapsed predecessor
/// chains and copies every input lane, plus the successor's first block, makes no allocator call
/// and no audited allocation. Red if the carry clones, boxes or drops anything on the render
/// thread.
#[test]
fn the_input_carry_allocates_and_frees_nothing() {
    use bench_support::alloc::{Mode, assert_installed, mode, set_mode};
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
    let a = Session::compile(filtered_session(true));
    let mut committed = filtered_session(true);
    let builtins = &mut committed.tracks[1].builtins;
    for channel in [&mut builtins.left, &mut builtins.right] {
        channel.trim_db = 3.0;
    }
    let b = Session::compile(with_muted_track(committed.clone()));
    let writes = [LiveWrite {
        block: SWAP_BLOCK,
        strip: "eq1",
        record: builtins_compiler::TrackInputRecord::TrimDb {
            lanes: builtins::BuiltinLaneSelector::Both,
            db: 3.0,
            smoothing_samples: 200,
        },
    }];
    let feed = Feed::new(SOURCE, 13);
    for backend in backends() {
        let run = direct_run(
            &a,
            Some((&b, &committed)),
            &feed,
            QUANTUM,
            &writes,
            backend,
            false,
        );
        assert_eq!(run.carry, CarryOutcome::Carried);
        assert!(run.collapses[0][0] > 0, "the predecessor was collapsed");
        assert_eq!(
            run.swap_allocator,
            ((0, 0, 0), (0, 0)),
            "{backend:?}: allocator calls and audited allocations at the swap block"
        );
    }
}

/// Gate 3 of #1276, live term. On the all-mono session a left-only trim record that leaves the
/// trim where it was is pending in A's queue at the swap. It moves no word, but it is a one-channel
/// write, so its strip's chain must render dual from the swap block on, as the reference's does
/// from the block it drains the record. Red if the carry drops the lane's live symmetry terms: the
/// successor would collapse a chain the predecessor had retired from collapsing.
#[test]
fn a_one_channel_record_keeps_its_chain_dual_after_the_swap() {
    let a = Session::compile(filtered_session(true));
    let b = Session::compile(with_muted_track(filtered_session(true)));
    let trim_db = a.model.tracks[2].builtins.left.trim_db;
    let writes = [LiveWrite {
        block: SWAP_BLOCK,
        strip: "eq2",
        record: builtins_compiler::TrackInputRecord::TrimDb {
            lanes: builtins::BuiltinLaneSelector::Left,
            db: trim_db,
            smoothing_samples: 0,
        },
    }];
    let feed = Feed::new(SOURCE, 15);
    for backend in backends() {
        let reference = direct_run(&b, None, &feed, QUANTUM, &writes, backend, false);
        let run = direct_run(
            &a,
            Some((&b, &a.model)),
            &feed,
            QUANTUM,
            &writes,
            backend,
            false,
        );
        assert_eq!(run.carry, CarryOutcome::Carried);
        assert_eq!(
            first_direct_difference(&run, &reference),
            None,
            "{backend:?}"
        );
        // Every collapsible chain collapsed before the swap block; one fewer after it.
        let [collapsed, chains] = reference.collapses[1];
        assert_eq!(
            collapsed,
            chains * BLOCKS as u64 - (BLOCKS - SWAP_BLOCK) as u64,
            "{backend:?}: the reference retires exactly one chain at the record"
        );
        assert_eq!(
            run.collapses[1][0],
            (chains - 1) * (BLOCKS - SWAP_BLOCK) as u64,
            "{backend:?}: the successor keeps that chain dual"
        );
    }
}

/// The all-mono [`filtered_session`] with `eq6`'s right trim 6 dB below its left unless
/// `eq6_symmetric`, so `eq6`'s two channels are driven apart; with `stereo_neighbour`, `eq5` reads
/// two source channels, so the chain that pools `eq6` beside it can never collapse.
fn diverged_session(stereo_neighbour: bool, eq6_symmetric: bool) -> SessionModel {
    let mut model = filtered_session(true);
    for track in &mut model.tracks {
        match track.id.as_str() {
            "eq5" if stereo_neighbour => track.right_source_channel = 1,
            "eq6" if !eq6_symmetric => {
                track.builtins.right.trim_db = track.builtins.left.trim_db - 6.0;
            }
            _ => {}
        }
    }
    model
}

/// The carried channel agreement, at one width (#1276 attempt 1 MAJOR-1). `eq6` renders with
/// asymmetric trims until a `Both` record two blocks before the swap equalises them; its filter
/// integrators still disagree at the swap. The transaction commits the equal trims and adds a
/// muted track, so `eq6`'s section carries into a successor chain that can collapse.
///
/// * With `stereo_neighbour`, `eq6`'s predecessor chain can never collapse and never maintained
///   its agreement flag; it must hand over `false`, not the `true` it was bound with.
/// * Without it, `eq6`'s predecessor chain is armed and its flag was cleared; the successor must
///   keep it cleared.
///
/// Either way the successor waits for a proof, which the diverged lane cannot give, and renders
/// the reference's bits: A with the muted track, fed the same record at the same block.
fn a_diverged_lane_keeps_its_chain_dual_after_the_swap(backend: Backend, stereo_neighbour: bool) {
    let a = Session::compile(diverged_session(stereo_neighbour, false));
    let trim_db = a
        .model
        .tracks
        .iter()
        .find(|track| track.id.as_str() == "eq6")
        .expect("eq6")
        .builtins
        .left
        .trim_db;
    let writes = [LiveWrite {
        block: SWAP_BLOCK - 2,
        strip: "eq6",
        record: builtins_compiler::TrackInputRecord::TrimDb {
            lanes: builtins::BuiltinLaneSelector::Both,
            db: trim_db,
            smoothing_samples: 0,
        },
    }];
    let committed = diverged_session(stereo_neighbour, true);
    let b = Session::compile(with_muted_track(committed.clone()));
    let reference_session =
        Session::compile(with_muted_track(diverged_session(stereo_neighbour, false)));
    let feed = Feed::new(SOURCE, 21);
    let reference = direct_run(
        &reference_session,
        None,
        &feed,
        QUANTUM,
        &writes,
        backend,
        false,
    );
    for successor_dual in [false, true] {
        let run = direct_run(
            &a,
            Some((&b, &committed)),
            &feed,
            QUANTUM,
            &writes,
            backend,
            successor_dual,
        );
        assert_eq!(run.carry, CarryOutcome::Carried, "{backend:?}");
        assert_eq!(
            first_direct_difference(&run, &reference),
            None,
            "{backend:?}, stereo neighbour {stereo_neighbour}, successor dual {successor_dual}"
        );
        if !successor_dual {
            assert!(
                run.collapses[1][0] > 0,
                "{backend:?}: the successor's other chains still collapse"
            );
        }
    }
}

/// The carried agreement from a chain that can never collapse, with no live record: `eq6` has an
/// asymmetric input delay in A, so no chain holding it is armed and its filter integrators
/// diverge. B makes the delay symmetric and adds a muted track; delay is not a value D1 compares,
/// so the section carries (a C ABI structural transaction prepares exactly this successor). The
/// delay line itself does not carry, so the oracle is the same successor forced dual: collapse is
/// class A and must not move a bit.
fn a_lane_from_a_delayed_strip_keeps_its_chain_dual_after_the_swap(backend: Backend) {
    let mut a_model = filtered_session(true);
    let eq6 = a_model
        .tracks
        .iter_mut()
        .find(|track| track.id.as_str() == "eq6")
        .expect("eq6");
    eq6.builtins.right.delay_samples = 37;
    let a = Session::compile(a_model);
    let b = Session::compile(with_muted_track(filtered_session(true)));
    let feed = Feed::new(SOURCE, 23);
    let dual = direct_run(&a, Some((&b, &a.model)), &feed, QUANTUM, &[], backend, true);
    let run = direct_run(
        &a,
        Some((&b, &a.model)),
        &feed,
        QUANTUM,
        &[],
        backend,
        false,
    );
    assert_eq!(run.carry, CarryOutcome::Carried, "{backend:?}");
    assert!(
        run.collapses[1][0] > 0,
        "{backend:?}: the successor collapses"
    );
    assert_eq!(first_direct_difference(&run, &dual), None, "{backend:?}");
}

/// The carried agreement at eight lanes. Red if a predecessor chain that can never collapse hands
/// over the unmaintained `true` it was bound with (the stereo-neighbour and delay cases), or if
/// the successor ignores a cleared flag (the armed case): the successor collapses a lane whose
/// channels disagree.
#[cfg(target_feature = "avx2")]
#[test]
fn a_diverged_lane_keeps_its_chain_dual_after_the_swap_at_eight_lanes() {
    a_diverged_lane_keeps_its_chain_dual_after_the_swap(Backend::Simd8, true);
    a_diverged_lane_keeps_its_chain_dual_after_the_swap(Backend::Simd8, false);
    a_lane_from_a_delayed_strip_keeps_its_chain_dual_after_the_swap(Backend::Simd8);
}

/// The carried agreement at four lanes.
#[test]
fn a_diverged_lane_keeps_its_chain_dual_after_the_swap_at_four_lanes() {
    a_diverged_lane_keeps_its_chain_dual_after_the_swap(Backend::Simd4, true);
    a_diverged_lane_keeps_its_chain_dual_after_the_swap(Backend::Simd4, false);
    a_lane_from_a_delayed_strip_keeps_its_chain_dual_after_the_swap(Backend::Simd4);
}

/// D6 and P11 of #1276. Installing the input section refuses a plan with no carry program (no
/// predecessor named), a padding lane, and a repeated successor or predecessor lane. At the swap
/// block a predecessor lane that does not resolve refuses the whole hand-over, before any lane
/// moves. Red if install accepts a padding lane, or if the swap block copies the lanes that
/// resolve and skips the rest.
#[test]
fn a_bad_input_move_is_refused_whole() {
    use graph::{
        GraphCarryInstallError, GraphCarryProgram, GraphLaneLocation, GraphLaneMove,
        install_builtin_input_carry, install_carry_program,
    };
    let backend = Backend::Simd4;
    let a = Session::compile(filtered_session(false));
    let b = Session::compile(with_muted_track(filtered_session(false)));
    let mut predecessor = a.prepare(backend);
    let lanes = graph::builtin_input_lanes(&mut predecessor.plan).expect("a graph plan");
    assert_eq!(lanes.len(), 9);
    let at = |strip: &str| {
        lanes
            .iter()
            .find(|(id, _)| &**id == strip)
            .expect("strip")
            .1
    };
    // Four-lane banks of eq0-eq3, eq4-eq7 and eq8 alone.
    let (eq0, eq8) = (at("eq0"), at("eq8"));
    assert_eq!(eq8.lane, 0);
    let step = |successor, predecessor| GraphLaneMove {
        successor,
        predecessor,
    };

    let mut successor = b.prepare(backend);
    let successor_lanes = graph::builtin_input_lanes(&mut successor.plan).expect("a graph plan");
    let into = successor_lanes
        .iter()
        .find(|(id, _)| &**id == "eq0")
        .expect("eq0")
        .1;
    assert_eq!(
        install_builtin_input_carry(&mut successor.plan, vec![step(into, eq0)]),
        Err(GraphCarryInstallError::NoCarryProgram)
    );
    install_carry_program(
        &mut successor.plan,
        GraphCarryProgram {
            predecessor: predecessor.inventory.plan_identity(),
            sources: Box::default(),
        },
    )
    .expect("a program that moves no source");
    // The successor's last bank holds eq7 and eq8; its lanes 2 and 3 are padding.
    let last = successor_lanes
        .iter()
        .find(|(id, _)| &**id == "eq8")
        .expect("eq8")
        .1;
    let members = successor_lanes
        .iter()
        .filter(|(_, location)| location.unit == last.unit)
        .count();
    assert_eq!(members, 2);
    let padding = GraphLaneLocation { lane: 2, ..last };
    assert_eq!(
        install_builtin_input_carry(&mut successor.plan, vec![step(padding, eq0)]),
        Err(GraphCarryInstallError::InputLaneOutOfRange)
    );
    assert_eq!(
        install_builtin_input_carry(&mut successor.plan, vec![step(into, eq0), step(into, eq8)]),
        Err(GraphCarryInstallError::DuplicateInputLane)
    );
    // A predecessor location past every unit: it sorts after eq0's valid move.
    let nowhere = GraphLaneLocation {
        unit: u32::MAX,
        ..eq8
    };
    let other = successor_lanes
        .iter()
        .find(|(id, _)| &**id == "eq1")
        .expect("eq1")
        .1;
    install_builtin_input_carry(
        &mut successor.plan,
        vec![step(into, eq0), step(other, nowhere)],
    )
    .expect("successor lanes resolve; predecessor lanes are checked at the swap block");

    // Render the predecessor so eq0's filters hold state, then hand over.
    let feed = Feed::new(SOURCE, 17);
    let rate = predecessor.report.sample_rate_hz;
    for block in 0..SWAP_BLOCK {
        feed.submit(&mut predecessor.sources, rate, block);
        let mut output = [0.0_f32; QUANTUM * 2];
        let sample = predecessor.plan.next_absolute_sample();
        predecessor
            .plan
            .render_contiguous(
                engine::realtime::RenderIo {
                    output: engine::realtime::PlanarBufferMut::try_new(
                        &mut output,
                        2,
                        QUANTUM,
                        QUANTUM,
                    )
                    .expect("output"),
                },
                sample,
            )
            .expect("render");
    }
    assert_eq!(
        successor.plan.adopt_predecessor_plan(&mut predecessor.plan),
        CarryOutcome::PredecessorMismatch
    );
    // Nothing moved: the successor's first block is a fresh plan's first block on the same PCM.
    let reference = reference_run(&b, &[&feed], backend);
    feed.submit(&mut successor.sources, rate, 0);
    let mut output = [f32::NAN; QUANTUM * 2];
    let sample = successor.plan.next_absolute_sample();
    successor
        .plan
        .render_contiguous(
            engine::realtime::RenderIo {
                output: engine::realtime::PlanarBufferMut::try_new(
                    &mut output,
                    2,
                    QUANTUM,
                    QUANTUM,
                )
                .expect("output"),
            },
            sample,
        )
        .expect("render");
    let bits: Vec<u32> = output.iter().map(|sample| sample.to_bits()).collect();
    assert_eq!(bits, reference[0].bits);
}

/// #1276 attempt-2 MINOR-1. Red if a successor's preparation accepts a committed model whose
/// strips are not in canonical ID order: the binary-search join would then find no committed
/// section and silently restart every strip at rest, the click this slice removes.
#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "a successor's committed model is normalized")]
fn a_successor_refuses_an_unnormalized_committed_model() {
    let a = Session::compile(session_a());
    let mut b_model = session_a();
    add_track(&mut b_model, "eq0", MUTED_TRACK, SOURCE, true);
    let b = Session::compile(b_model);
    let backend = Backend::current();
    let predecessor = a.prepare(backend);
    let mut reversed = a.model.clone();
    reversed.tracks.reverse();
    let _ = b.prepare_successor(&predecessor.inventory, &reversed, &caps(), backend);
}
