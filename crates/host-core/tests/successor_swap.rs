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
#[path = "support/mod.rs"]
mod support;

use builtins_compiler::TestOnlyWriteGate;
use engine::realtime::{CarryOutcome, SwapOutcome};
use graph_compiler::Backend;
use host_core::{
    LiveRampRow, LiveRamps, PrepareRejection, SourceControlError, SourceControlSet,
    SourceSubmission,
};
use session::{EditRamps, RouteSource, SessionModel, Source, StableId, parse_session_json};
use successor::{
    BLOCKS, Feed, QUANTUM, SWAP_BLOCK, Session, Successor, caps, first_difference, reference_run,
    swapped_run,
};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const SOURCE: &str = "fixture-source";
/// Sorts before [`SOURCE`], so removing it shifts the kept source's index.
const AUX_SOURCE: &str = "aux-source";
const MUTED_TRACK: &str = "a-muted";

/// The carry program's table entries, `[source moves, input-section lane moves, strip-stage
/// moves, effect-lane moves]` (#1276, #1277 D8, #1280): the first four of
/// `graph::test_only_carry_counts`.
fn carry_counts(prepared: &mut host_core::PreparedHost) -> [usize; 4] {
    let [sources, inputs, strips, effects, _] = graph::test_only_carry_counts(&mut prepared.plan);
    [sources, inputs, strips, effects]
}

/// The command queue and seek-report record a successor's carried timeline brings (#1641 D7,
/// #1704 D4): every successor's carried source bytes hold them, as they hold a carried ring's
/// overhead, so a successor that carries no ring still carries these bytes.
fn timeline_queue_bytes() -> u64 {
    let report = source::SourceTimeline::resource_report(
        engine::QuantumFrames(QUANTUM as u32),
        core::num::NonZeroUsize::MIN,
    )
    .expect("timeline report");
    report.command_queue_bytes + report.seek_report_bytes
}

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
    assert!(report.carried_source_total_bytes > timeline_queue_bytes());
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
        run.carry_counts,
        [1, 2, 4, 0],
        "one source move, the two unchanged input sections (#1276) and the fader and \
         matrix stage of both kept strips (#1277)"
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

/// Gate 1 on every width. Red if the successor still allocates and plays a fresh ring, or if the
/// carry or the producer hand-over loses the queued block or the read position.
#[test]
fn an_added_muted_track_keeps_the_source_playing_on_every_width() {
    for backend in backends() {
        added_muted_track_keeps_playing(backend);
    }
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
        // The timeline's carried command queue only (#1641 D7).
        assert_eq!(
            prepared.report.carried_source_total_bytes,
            timeline_queue_bytes()
        );
        assert_eq!(
            carry_counts(&mut prepared),
            [0, 2, 4, 0],
            "no source move; the two unchanged input sections (#1276) and the fader and matrix \
             stage of both strips (#1277)"
        );
        // Allocated plus carried is a fresh plan's: the ring is new, and only the timeline's
        // command queue is carried (#1641 D7).
        assert_eq!(
            prepared.report.source_total_bytes + prepared.report.carried_source_total_bytes,
            fresh.source_total_bytes,
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
    assert!(
        roomy.source_total_bytes > 0 && roomy.carried_source_total_bytes > timeline_queue_bytes()
    );
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
    assert!(roomy.carried_source_overhead_bytes > timeline_queue_bytes());
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
        same.report.carried_source_total_bytes > timeline_queue_bytes(),
        "the control case carries"
    );
    let mut smaller = caps();
    smaller.source_ring_frames /= 2;
    let resized = b
        .prepare_successor(&predecessor.inventory, &a.model, &smaller, backend)
        .expect("successor with smaller rings");
    // The timeline's carried command queue only (#1641 D7).
    assert_eq!(
        resized.report.carried_source_total_bytes,
        timeline_queue_bytes()
    );

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
    assert_eq!(
        changed.report.carried_source_total_bytes,
        timeline_queue_bytes()
    );
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
        host_core::test_only_prepare_host_runtime_on(&session.compiled, &long_rings, backend)
            .unwrap_or_else(|failure| panic!("prepare: {failure:?}"))
    };
    let predecessor = prepare(&a);
    let fresh = prepare(&b).report;
    let successor = b
        .prepare_successor(&predecessor.inventory, &a.model, &long_rings, backend)
        .expect("successor")
        .report;
    assert!(successor.carried_source_total_bytes > timeline_queue_bytes());
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
        host_core::test_only_prepare_host_runtime_on(&b.compiled, &with_graph_cap(cap), backend)
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
        .reserve_replacement(prepared.plan, engine::realtime::PlanAdoption::Next)
        .unwrap_or_else(|_| panic!("reserve the successor"))
        .commit(0);
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

/// Gate 1 of #1274 on every width. Red if a held seek discards the primed generation (the stem is
/// silent at the anchor) or applies when observed (the stem starts early).
#[test]
fn a_future_anchor_starts_the_added_stem_on_its_block_on_every_width() {
    for backend in backends() {
        future_anchor_starts_the_added_stem_on_its_block(backend);
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

/// `model` without track `id` and the routes it sources: as a committed base, B's strip of that ID
/// is new to the join, so its input section restarts at the swap. A live value difference no
/// longer restarts one (#1630 D1).
fn without_track(mut model: SessionModel, id: &str) -> SessionModel {
    model.tracks.retain(|track| track.id.as_str() != id);
    model.routes.retain(
        |route| !matches!(&route.source, RouteSource::Track { track_id, .. } if track_id.as_str() == id),
    );
    model
}

/// `model` plus [`MUTED_TRACK`], a muted copy of `eq0` that sorts first.
fn with_muted_track(mut model: SessionModel) -> SessionModel {
    add_track(&mut model, "eq0", MUTED_TRACK, SOURCE, true);
    model
}

/// A live input record pushed into a run's queue for `strip` before block `block` renders.
struct LiveWrite {
    block: usize,
    strip: &'static str,
    record: builtins_compiler::TrackInputRecord,
}

/// One plan prepared with live controls on every strip, its handles, and its gate handle: every
/// input write is a transaction stamped from the plan's own gate (#1346 D7).
struct LivePlan {
    prepared: host_core::PreparedHost,
    handles: host_core::HostLiveControlHandles,
    gate: engine::realtime::PlanRevisionGate,
}

impl LivePlan {
    fn request() -> host_core::HostLiveControlRequest {
        host_core::HostLiveControlRequest::default()
    }

    fn new(
        mut prepared: host_core::PreparedHost,
        handles: host_core::HostLiveControlHandles,
    ) -> Self {
        let gate = prepared
            .plan
            .take_revision_gate()
            .expect("the plan's gate handle");
        Self {
            prepared,
            handles,
            gate,
        }
    }

    fn fresh(session: &Session, backend: Backend) -> Self {
        Self::fresh_with(session, &Self::request(), backend)
    }

    fn fresh_with(
        session: &Session,
        request: &host_core::HostLiveControlRequest,
        backend: Backend,
    ) -> Self {
        let (prepared, handles) = host_core::test_only_prepare_host_runtime_with_live_controls_on(
            &session.compiled,
            &caps(),
            request,
            backend,
        )
        .unwrap_or_else(|failure| panic!("prepare: {failure:?}"));
        Self::new(prepared, handles)
    }

    fn successor(
        session: &Session,
        predecessor: &host_core::PlanStateInventory,
        committed: &SessionModel,
        backend: Backend,
    ) -> Self {
        Self::successor_with(session, &Self::request(), predecessor, committed, backend)
    }

    fn successor_with(
        session: &Session,
        request: &host_core::HostLiveControlRequest,
        predecessor: &host_core::PlanStateInventory,
        committed: &SessionModel,
        backend: Backend,
    ) -> Self {
        Self::successor_observed(
            session,
            request,
            predecessor,
            committed,
            &host_core::NO_OBSERVATION,
            &EditRamps::new(),
            backend,
        )
    }

    /// A successor prepared with the taps the host armed (#1584 D6a) and the ramp lengths the
    /// transaction's edits gave their rows (#1394 D6).
    fn successor_observed(
        session: &Session,
        request: &host_core::HostLiveControlRequest,
        predecessor: &host_core::PlanStateInventory,
        committed: &SessionModel,
        observation: &controls::ObservationBindings,
        edit_ramps: &EditRamps,
        backend: Backend,
    ) -> Self {
        let (prepared, handles) =
            host_core::test_only_prepare_host_runtime_with_live_controls_successor_on(
                &session.compiled,
                &caps(),
                request,
                host_core::SuccessorBase {
                    edit_ramps,
                    observation,
                    ..host_core::SuccessorBase::new(
                        predecessor,
                        committed,
                        successor::SUCCESSOR_REVISION,
                    )
                },
                backend,
            )
            .unwrap_or_else(|failure| panic!("successor: {failure:?}"));
        let mut successor = Self::new(prepared, handles);
        // The commit's gate steps (#1277 D10): the handle inherits the preparation's open stamp
        // (a retarget's), stamps the revision again and publishes it.
        let _ = successor
            .gate
            .stamp(successor::SUCCESSOR_REVISION)
            .expect("the open revision stamps again");
        successor.gate.publish(successor::SUCCESSOR_REVISION);
        successor
    }

    fn push(&mut self, write: &LiveWrite) {
        let index = self
            .handles
            .strips
            .iter()
            .position(|strip| &**strip == write.strip)
            .expect("strip");
        let input = self.handles.strip_controls[index]
            .input
            .as_mut()
            .expect("input lane");
        self.gate
            .write_published(|stamp| input.write(stamp, write.record));
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
                    sample_rate_hz: self.prepared.report.sample_rate_hz,
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
    /// The predecessor's `live_values_superseded` count after the hand-over (#1312 D2).
    carry_superseded: u64,
    /// The successor's own `live_values_superseded` count after its first block: the carried
    /// values its adopting block replaced (#1630 f). The test seam gives each plan its own count;
    /// a host passes both plans the session's.
    successor_superseded: u64,
    /// The last plan's own count after the final block: the successor's, when `b` is given. A
    /// carried value counted again by a later drain shows here and not in `successor_superseded`.
    final_superseded: u64,
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
    direct_run_ramped(
        a,
        b,
        &EditRamps::new(),
        feed,
        quantum,
        writes,
        backend,
        successor_dual,
    )
}

/// [`direct_run`] whose successor is prepared with the transaction's `edit_ramps` (#1394 D6).
#[allow(clippy::too_many_arguments)]
fn direct_run_ramped(
    a: &Session,
    b: Option<(&Session, &SessionModel)>,
    edit_ramps: &EditRamps,
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
    let mut carry_superseded = 0;
    let mut successor_superseded = 0;
    plan.submit(feed, quantum, 0);
    for block in 0..BLOCKS {
        for write in writes.iter().filter(|write| write.block == block) {
            plan.push(write);
        }
        let swap = block == SWAP_BLOCK && b.is_some();
        if let (true, Some((session, committed))) = (swap, b) {
            collapses[0] = plan.prepared.plan.bank_collapse_counters();
            let mut successor = LivePlan::successor_observed(
                session,
                &LivePlan::request(),
                &plan.prepared.inventory,
                committed,
                &host_core::NO_OBSERVATION,
                edit_ramps,
                backend,
            );
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
            carry_superseded = plan.handles.live_values_superseded.get();
            successor_superseded = successor.handles.live_values_superseded.get();
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
        carry_superseded,
        successor_superseded,
        final_superseded: plan.handles.live_values_superseded.get(),
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
/// before copying, so B's first block starts the trim ramp exactly where A would have. `strip` is
/// the written strip: the first bank's `eq2`, or a strip in a later predecessor bank.
/// `restarting` names a strip the committed model lacks ([`without_track`]), so B's strip of that
/// ID is new to the join and its input section restarts at the swap; it sits in the first bank at
/// the lane of the written strip in its own bank, and it is muted in every session so that its
/// restart leaves the output bits where the unrestarted reference puts them.
fn a_pending_trim_record_survives_the_swap(
    backend: Backend,
    strip: &'static str,
    restarting: Option<&'static str>,
) {
    const NEW_TRIM_DB: f32 = 4.5;
    let record = builtins_compiler::TrackInputRecord::TrimDb {
        lanes: builtins::BuiltinLaneSelector::Both,
        db: NEW_TRIM_DB,
        smoothing_samples: 300,
    };
    let write = |block| LiveWrite {
        block,
        strip,
        record,
    };
    let session = || {
        let mut model = filtered_session(false);
        if let Some(restarting) = restarting {
            let track = model
                .tracks
                .iter_mut()
                .find(|track| track.id.as_str() == restarting)
                .expect("the restarting strip");
            track.fader.left_mute = true;
            track.fader.right_mute = true;
        }
        model
    };
    let a = Session::compile(session());
    let mut committed = session();
    let track = committed
        .tracks
        .iter_mut()
        .find(|track| track.id.as_str() == strip)
        .expect("the written strip");
    track.builtins.left.trim_db = NEW_TRIM_DB;
    track.builtins.right.trim_db = NEW_TRIM_DB;
    let b = Session::compile(with_muted_track(committed.clone()));
    if let Some(restarting) = restarting {
        // A live value difference carries and retargets (#1630 D1); a strip the committed model
        // lacks has nothing to carry, so that input section restarts.
        committed = without_track(committed, restarting);
    }
    let reference_session = Session::compile(with_muted_track(session()));
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
        "{backend:?}: the record to {strip} takes effect on the swap block's first sample"
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

/// #1312 D2 on the input cells: `live_values_superseded` counts the writes the carry drain
/// replaces on input lanes that carry, and none on an input lane the swap restarts, whichever
/// lanes share their bank. The committed model lacks the first strip ([`without_track`]), so that
/// strip's input section restarts while the strip beside it, in the same predecessor bank at four
/// and at eight lanes, carries. Three trim writes sit unread in each strip's cells at the
/// hand-over.
fn a_restarting_input_lane_does_not_count_its_replaced_writes(backend: Backend) {
    const RESTARTING: &str = "eq0";
    const CARRYING: &str = "eq1";
    let a = Session::compile(filtered_session(false));
    let b = Session::compile(with_muted_track(filtered_session(false)));
    let committed = without_track(filtered_session(false), RESTARTING);
    let feed = Feed::new(SOURCE, 5);
    let run = |strips: &[&'static str]| {
        let writes: Vec<LiveWrite> = strips
            .iter()
            .flat_map(|&strip| {
                [1.0_f32, 2.0, 3.0].map(|db| LiveWrite {
                    block: SWAP_BLOCK,
                    strip,
                    record: builtins_compiler::TrackInputRecord::TrimDb {
                        lanes: builtins::BuiltinLaneSelector::Both,
                        db,
                        smoothing_samples: 0,
                    },
                })
            })
            .collect();
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
        run
    };
    let restarting = run(&[RESTARTING]);
    let carrying = run(&[CARRYING]);
    let both = run(&[RESTARTING, CARRYING]);
    assert_eq!(
        restarting.carry_superseded, 0,
        "{backend:?}: writes to a restarting input lane are not counted"
    );
    // Three writes to a cell replace two, and a Both trim write fills the left and the right cell.
    assert_eq!(
        carrying.carry_superseded,
        2 * 2,
        "{backend:?}: a carried input lane counts the writes its drain replaced"
    );
    assert_eq!(
        both.carry_superseded, carrying.carry_superseded,
        "{backend:?}: the restarting lane adds nothing to its bank-mate's count"
    );
}

#[test]
fn a_restarting_input_lane_does_not_count_its_replaced_writes_on_every_width() {
    for backend in backends() {
        a_restarting_input_lane_does_not_count_its_replaced_writes(backend);
    }
}

/// Gate 2, at every width. Red if the carry exports a lane before draining its queue: the record
/// is lost with the retired plan.
///
/// The later-bank rows write a carried strip in the second predecessor bank while a strip at the
/// same lane of the first bank restarts at the swap. Red if the input drain's lane mask is built
/// from the first bank's moves instead of the moves of the bank being drained: the first bank's
/// mask lacks that lane, so the pending trim write of the later bank's lane is lost at the swap.
/// The first-bank rows cannot see it, and a first bank whose lanes all carry (a full mask) would
/// not either.
#[test]
fn a_pending_trim_record_survives_the_swap_in_every_bank_on_every_width() {
    let rows = [
        #[cfg(target_feature = "avx2")]
        (Backend::Simd8, "eq2", None),
        (Backend::Simd4, "eq2", None),
        // `eq8` is lane 0 of the second bank at eight lanes; `eq0`, lane 0 of the first, restarts.
        #[cfg(target_feature = "avx2")]
        (Backend::Simd8, "eq8", Some("eq0")),
        // `eq5` is lane 1 of the second bank at four lanes; `eq1`, lane 1 of the first, restarts.
        (Backend::Simd4, "eq5", Some("eq1")),
    ];
    for (backend, strip, restarting) in rows {
        a_pending_trim_record_survives_the_swap(backend, strip, restarting);
    }
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

/// Gate 3 on every width. Red if a successor chain that receives carried lanes clears its
/// channel-agreement flag (it needs a proof to engage, and a chain with an effect in its prefix,
/// which declines the proof, would never collapse again), or if the carry skips the disengage
/// copy of a collapsed predecessor (the dual successor reads a stale right channel).
#[test]
fn collapsed_strips_keep_collapsing_across_a_swap_on_every_width() {
    for backend in backends() {
        collapsed_strips_keep_collapsing(backend, false);
        collapsed_strips_keep_collapsing(backend, true);
    }
}

/// Gate 4 of #1276, one edit. B applies `edit` to `eq0`'s input section in the same transaction
/// that adds a muted track, so `eq0`'s section does not carry (Q1's default): from the swap block
/// on it renders exactly what a fresh plan of B renders from rest on the same PCM. These plans
/// attach no input cells, so they cannot retarget a carried section; on plans that do, the section
/// carries and retargets (#1630, [`an_input_difference_carries_and_retargets`]).
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
            run.carry_counts,
            [1, 0, 2, 0],
            "{name}, {backend:?}: the source moves, the changed section does not; the strip's \
             fader and matrix stage still do (#1277)"
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

/// A live input section whose new values the live path refuses -- a high-pass above the strip's
/// low-pass, which the session compiles -- is not an internal fault of the input join: the
/// successor's preparation returns its own typed refusal (`builtin.filter.order`).
///
/// Test value: red if the join asserts on the refusal: a debug build then panicked in the middle
/// of a rebuild, and a host that caught the panic lost the candidate the rebuild had withdrawn
/// (#1324a: capi's in-place parity stream then fed a ring nothing read, `source.backpressure`).
#[test]
fn an_input_section_the_live_path_refuses_is_a_typed_refusal() {
    let a = Session::compile(filtered_session(false));
    let mut b_model = filtered_session(false);
    let left = &mut b_model.tracks[0].builtins.left;
    left.hpf_hz = left.lpf_hz + 1_000.0;
    let b = Session::compile(b_model);
    for backend in backends() {
        let predecessor = LivePlan::fresh(&a, backend).prepared;
        let refused = host_core::test_only_prepare_host_runtime_with_live_controls_successor_on(
            &b.compiled,
            &caps(),
            &LivePlan::request(),
            host_core::SuccessorBase::new(
                &predecessor.inventory,
                &a.model,
                successor::SUCCESSOR_REVISION,
            ),
            backend,
        )
        .err()
        .expect("the filter order is refused");
        assert!(
            String::from_utf8_lossy(refused.as_bytes()).starts_with("builtin.filter.order\t"),
            "{backend:?}: {refused:?}"
        );
    }
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
    let source_only = [1, 0, 0, 0];
    // Predecessor without live controls, successor with them.
    let plain = a.prepare(backend);
    let mut live = LivePlan::successor(b, &plain.inventory, &a.model, backend);
    assert_eq!(
        carry_counts(&mut live.prepared),
        source_only,
        "plain -> live"
    );
    // Predecessor with live controls, successor without them.
    let live = LivePlan::fresh(a, backend);
    let mut plain = b
        .prepare_successor(&live.prepared.inventory, &a.model, &caps(), backend)
        .unwrap_or_else(|failure| panic!("successor: {failure:?}"));
    assert_eq!(carry_counts(&mut plain), source_only, "live -> plain");
    // The control: the same kind on both sides carries all nine sections.
    let mut plain = b
        .prepare_successor(&a.prepare(backend).inventory, &a.model, &caps(), backend)
        .unwrap_or_else(|failure| panic!("successor: {failure:?}"));
    assert_eq!(carry_counts(&mut plain), [1, 9, 18, 0]);
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
        let mut prepared = b
            .prepare_successor(&a.prepare(backend).inventory, &a.model, &caps(), backend)
            .unwrap_or_else(|failure| panic!("successor: {failure:?}"));
        assert_eq!(
            carry_counts(&mut prepared),
            [1, 1, 2, 0],
            "{backend:?}: the source and eq1's section move, the submix's section does not; eq1's \
             fader and matrix stage move, the submix's do not (#1277)"
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

/// #1630 gates 1-3, one edit. B applies `edit` to [`RETARGETED`]'s input section in the same
/// transaction that adds a muted track, on plans with live input cells. Run 1 prepares B from A's
/// committed model: the section carries and the successor's cells retarget it (D1, D2). Run 2
/// writes the classifier's records for the same edit to A's cells just before the swap and
/// prepares B from a base that holds them: a plan that made the edit live. The runs are
/// bit-identical in every block, and the swap block allocates and frees nothing (D3).
#[cfg(feature = "control-provider")]
fn an_input_difference_carries_and_retargets(name: &str, edit: fn(&mut session::DualMonoBuiltins)) {
    use bench_support::alloc::{Mode, assert_installed, mode, set_mode};
    struct RestoreMode(Mode);
    impl Drop for RestoreMode {
        fn drop(&mut self) {
            set_mode(self.0);
        }
    }
    const RETARGETED: &str = "eq2";
    assert_installed();
    let _restore = RestoreMode(mode());
    set_mode(Mode::Count);
    engine::realtime::audit::warm_up();
    let a = Session::compile(filtered_session(false));
    let mut moved = filtered_session(false);
    edit(
        &mut moved
            .tracks
            .iter_mut()
            .find(|track| track.id.as_str() == RETARGETED)
            .expect("the retargeted strip")
            .builtins,
    );
    let moved = Session::compile(moved).model;
    let b = Session::compile(with_muted_track(moved.clone()));
    // The records the live path gives the same edit (#1556), with the session's ramps.
    let delta = host_core::live_delta::classify_live_delta(
        &a.model,
        &moved,
        LiveRamps::for_session(&moved),
        &EditRamps::new(),
    )
    .unwrap_or_else(|reason| panic!("{name}: the edit is live: {reason:?}"));
    assert_eq!(delta.strips.len(), 1, "{name}");
    assert!(!delta.strips[0].input.is_empty(), "{name}: an input record");
    let writes: Vec<LiveWrite> = delta.strips[0]
        .input
        .iter()
        .map(|&record| LiveWrite {
            block: SWAP_BLOCK,
            strip: RETARGETED,
            record,
        })
        .collect();
    let feed = Feed::new(SOURCE, 17);
    for backend in backends() {
        let run = |committed: &SessionModel, writes: &[LiveWrite]| {
            let run = direct_run(
                &a,
                Some((&b, committed)),
                &feed,
                QUANTUM,
                writes,
                backend,
                false,
            );
            assert_eq!(run.carry, CarryOutcome::Carried, "{name}, {backend:?}");
            run
        };
        let retargeted = run(&a.model, &[]);
        let written = run(&moved, &writes);
        assert_eq!(
            retargeted.swap_allocator,
            ((0, 0, 0), (0, 0)),
            "{name}, {backend:?}: allocator calls and audited allocations at the swap block"
        );
        assert_eq!(
            first_direct_difference(&retargeted, &written),
            None,
            "{name}, {backend:?}: carry then retarget sounds like live edit then structural edit"
        );
        // Not vacuous: a section that restarts at the new values is audible at the swap block,
        // and so is one that carries without the retarget (B without the edit).
        let restarted = run(&without_track(a.model.clone(), RETARGETED), &[]);
        assert_eq!(
            first_direct_difference(&retargeted, &restarted),
            Some(SWAP_BLOCK),
            "{name}, {backend:?}: a restart is a transient the carry avoids"
        );
        let unmoved = direct_run(
            &a,
            Some((
                &Session::compile(with_muted_track(a.model.clone())),
                &a.model,
            )),
            &feed,
            QUANTUM,
            &[],
            backend,
            false,
        );
        assert_eq!(
            first_direct_difference(&retargeted, &unmoved),
            Some(SWAP_BLOCK),
            "{name}, {backend:?}: the retarget moves the output from the swap block on"
        );
        // #1394 D6 through the successor path (#1630 g): the transaction gave the strip's
        // `(Strip, Input)` row a 37-sample ramp. The retarget is the record of that edit, so the
        // run equals the one that writes the classifier's records for it. Red if the join reads
        // another strip's lengths, or none.
        let mut edit_ramps = EditRamps::new();
        edit_ramps.set(
            &session::EditRampTarget::Strip(id(RETARGETED)),
            session::EditRampRow::Input,
            Some(37),
        );
        let ramped_delta = host_core::live_delta::classify_live_delta(
            &a.model,
            &moved,
            LiveRamps::for_session(&moved),
            &edit_ramps,
        )
        .unwrap_or_else(|reason| panic!("{name}: the ramped edit is live: {reason:?}"));
        let ramped_writes: Vec<LiveWrite> = ramped_delta.strips[0]
            .input
            .iter()
            .map(|&record| LiveWrite {
                block: SWAP_BLOCK,
                strip: RETARGETED,
                record,
            })
            .collect();
        let ramped = direct_run_ramped(
            &a,
            Some((&b, &a.model)),
            &edit_ramps,
            &feed,
            QUANTUM,
            &[],
            backend,
            false,
        );
        assert_eq!(ramped.carry, CarryOutcome::Carried, "{name}, {backend:?}");
        let ramped_written = run(&moved, &ramped_writes);
        assert_eq!(
            first_direct_difference(&ramped, &ramped_written),
            None,
            "{name}, {backend:?}: the retarget ramps over the edit's `(Strip, Input)` length"
        );
        // Not vacuous where the row ramps (trim and polarity; a filter target does not take it):
        // the edit's length is audible against the session's.
        if ramped_delta.strips[0].input != delta.strips[0].input {
            assert_eq!(
                first_direct_difference(&ramped, &retargeted),
                Some(SWAP_BLOCK),
                "{name}, {backend:?}: the edit's length is audible"
            );
        }
    }
}

/// #1630 gate 1: a trim change. Red if the join still restarts a section whose trim differs (the
/// restart transient), or if the carried section keeps the old trim (no retarget).
#[cfg(feature = "control-provider")]
#[test]
fn a_trim_change_carries_and_retargets_the_input_section() {
    an_input_difference_carries_and_retargets("trim_db", |builtins| {
        builtins.left.trim_db += 1.5;
        builtins.right.trim_db += 0.75;
    });
}

/// #1630 gate 2: input filter cutoffs, one-sided and on both sides. Red if a filter retarget is
/// dropped or designed from other seeds than the live path's.
#[cfg(feature = "control-provider")]
#[test]
fn a_filter_cutoff_change_carries_and_retargets_the_input_section() {
    an_input_difference_carries_and_retargets("hpf_hz", |builtins| {
        builtins.left.hpf_hz = 160.0;
        builtins.right.hpf_hz = 170.0;
    });
    an_input_difference_carries_and_retargets("lpf_hz", |builtins| {
        builtins.right.lpf_hz = 3_500.0;
    });
}

/// #1630 gate 2: a polarity change.
#[cfg(feature = "control-provider")]
#[test]
fn a_polarity_change_carries_and_retargets_the_input_section() {
    an_input_difference_carries_and_retargets("polarity_invert", |builtins| {
        builtins.right.polarity_invert = true;
    });
}

/// #1630 f, D15-2 on the input cells: a carried live value that the successor's retarget replaces
/// in the adopting block is counted once. A left-only trim write sits unread in A's cells at the
/// hand-over and is committed in the base; B moves that trim again. The carry drain applies the
/// written value, and the successor's retarget replaces it before any sample renders: one value,
/// one cell, superseded. The run is bit-identical to one whose base never held the write, and with
/// no retarget (B leaves the trim at the written value) nothing is counted. Red without the
/// adopting block's count (0, not 1).
fn a_carried_input_value_the_retarget_replaces_is_counted(backend: Backend) {
    const STRIP: &str = "eq2";
    const WRITTEN_DB: f32 = 1.75;
    const MOVED_DB: f32 = -3.5;
    let with_left_trim = |db: f32| {
        let mut model = filtered_session(false);
        model
            .tracks
            .iter_mut()
            .find(|track| track.id.as_str() == STRIP)
            .expect("the strip")
            .builtins
            .left
            .trim_db = db;
        model
    };
    let a = Session::compile(filtered_session(false));
    let written = with_left_trim(WRITTEN_DB);
    let b = Session::compile(with_muted_track(with_left_trim(MOVED_DB)));
    // The second write: a second value to the same cell, two blocks after the adopting block,
    // through the successor's normal write path. The carry marks are spent by then, so it is a plain
    // write, never a second carried value.
    let writes = [
        LiveWrite {
            block: SWAP_BLOCK,
            strip: STRIP,
            record: builtins_compiler::TrackInputRecord::TrimDb {
                lanes: builtins::BuiltinLaneSelector::Left,
                db: WRITTEN_DB,
                smoothing_samples: 64,
            },
        },
        LiveWrite {
            block: SWAP_BLOCK + 2,
            strip: STRIP,
            record: builtins_compiler::TrackInputRecord::TrimDb {
                lanes: builtins::BuiltinLaneSelector::Left,
                db: 0.5,
                smoothing_samples: 64,
            },
        },
    ];
    let feed = Feed::new(SOURCE, 19);
    let run = |b: &Session, committed: &SessionModel, writes: &[LiveWrite]| {
        let run = direct_run(
            &a,
            Some((b, committed)),
            &feed,
            QUANTUM,
            writes,
            backend,
            false,
        );
        assert_eq!(run.carry, CarryOutcome::Carried, "{backend:?}");
        run
    };
    let replaced = run(&b, &written, &writes);
    assert_eq!(
        (replaced.carry_superseded, replaced.successor_superseded),
        (0, 1),
        "{backend:?}: the carried value the retarget replaced is counted once"
    );
    assert_eq!(
        replaced.final_superseded, 1,
        "{backend:?}: a later write to the cell is not a second carried value"
    );
    let unwritten = run(&b, &a.model, &writes[1..]);
    assert_eq!(unwritten.successor_superseded, 0, "{backend:?}");
    assert_eq!(unwritten.final_superseded, 0, "{backend:?}");
    assert_eq!(
        first_direct_difference(&replaced, &unwritten),
        None,
        "{backend:?}: the replaced carried value moves no bit"
    );
    // A carried value with no retarget is heard, not superseded.
    let kept = run(
        &Session::compile(with_muted_track(written.clone())),
        &written,
        &writes,
    );
    assert_eq!(
        (kept.carry_superseded, kept.successor_superseded),
        (0, 0),
        "{backend:?}: a carried value nothing replaces is not counted"
    );
    assert_eq!(kept.final_superseded, 0, "{backend:?}");
}

#[test]
fn a_carried_input_value_the_retarget_replaces_is_counted_on_every_width() {
    for backend in backends() {
        a_carried_input_value_the_retarget_replaces_is_counted(backend);
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

/// The carried agreement on every width. Red if a predecessor chain that can never collapse hands
/// over the unmaintained `true` it was bound with (the stereo-neighbour and delay cases), or if
/// the successor ignores a cleared flag (the armed case): the successor collapses a lane whose
/// channels disagree.
#[test]
fn a_diverged_lane_keeps_its_chain_dual_after_the_swap_on_every_width() {
    for backend in backends() {
        a_diverged_lane_keeps_its_chain_dual_after_the_swap(backend, true);
        a_diverged_lane_keeps_its_chain_dual_after_the_swap(backend, false);
        a_lane_from_a_delayed_strip_keeps_its_chain_dual_after_the_swap(backend);
    }
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
            timeline: false,
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

/// One plan's collapse counters, summed over every plan of a [`swapping_run`].
#[derive(Clone, Copy, Debug, Default)]
struct CollapseTally {
    /// Collapsed chain-blocks.
    collapsed: u64,
    /// `[disengages, re-engages, agreement proofs]`.
    transitions: [u64; 3],
    /// Swaps whose input lanes carried.
    carried: usize,
}

/// Render `sessions[0]`, then swap to `sessions[1]`, `sessions[0]`, ... every `swap_every`
/// blocks, each successor prepared from its predecessor and adopting it, for `blocks` blocks of
/// `quantum` frames (the shape is `[quantum, blocks, swap_every]`). `writes` go to whichever plan
/// renders their block next. `forced_dual` forces every plan's mono collapse off: the class-A
/// oracle.
fn swapping_run(
    sessions: [&Session; 2],
    feed: &Feed,
    [quantum, blocks, swap_every]: [usize; 3],
    writes: &[LiveWrite],
    backend: Backend,
    forced_dual: bool,
) -> (Vec<Vec<u32>>, CollapseTally) {
    let mut plan = LivePlan::fresh(sessions[0], backend);
    plan.prepared.plan.force_mono_collapse_off(forced_dual);
    let mut current = 0_usize;
    let mut tally = CollapseTally::default();
    let mut output = Vec::with_capacity(blocks);
    let mut next = 0_usize;
    plan.submit(feed, quantum, 0);
    for block in 0..blocks {
        while next < writes.len() && writes[next].block == block {
            plan.push(&writes[next]);
            next += 1;
        }
        if block > 0 && block % swap_every == 0 {
            tally.collapsed += plan.prepared.plan.bank_collapse_counters()[0];
            let [disengages, reengages, proofs] = plan.prepared.plan.bank_collapse_transitions();
            tally.transitions[0] += disengages;
            tally.transitions[1] += reengages;
            tally.transitions[2] += proofs;
            let following = 1 - current;
            let mut successor = LivePlan::successor(
                sessions[following],
                &plan.prepared.inventory,
                &sessions[current].model,
                backend,
            );
            successor.prepared.plan.force_mono_collapse_off(forced_dual);
            assert_eq!(
                successor
                    .prepared
                    .sources
                    .adopt_persisting(&mut plan.prepared.sources),
                1
            );
            successor.submit(feed, quantum, block);
            let carry = successor
                .prepared
                .plan
                .adopt_predecessor_plan(&mut plan.prepared.plan);
            if carry == CarryOutcome::Carried {
                tally.carried += 1;
            }
            plan = successor;
            current = following;
        }
        output.push(plan.render(quantum));
        if block + 1 < blocks && (block + 1) % swap_every != 0 {
            plan.submit(feed, quantum, block + 1);
        }
    }
    tally.collapsed += plan.prepared.plan.bank_collapse_counters()[0];
    let [disengages, reengages, proofs] = plan.prepared.plan.bank_collapse_transitions();
    tally.transitions[0] += disengages;
    tally.transitions[1] += reengages;
    tally.transitions[2] += proofs;
    (output, tally)
}

/// Random live input-filter records for [`a_collapsed_chain_renders_the_forced_dual_bits`], every
/// one a valid prepared target at 48 kHz.
///
/// Each `swap_every`-block plan segment carries background `Both` records -- enables to a random
/// design, disables and re-sends of the strip's last record, on either section -- which keep a
/// mono strip's channels in agreement. Every second segment also drives one strip apart with a
/// one-channel record, then disables both its sections on both channels, so the lane is back at
/// rest, agreeing, before the swap: its successor chain must earn the collapse back with an M3
/// proof over carried state.
fn random_filter_writes(seed: u64, blocks: usize, swap_every: usize) -> Vec<LiveWrite> {
    use builtins::BuiltinLaneSelector;
    const STRIPS: [&str; 9] = [
        "eq0", "eq1", "eq2", "eq3", "eq4", "eq5", "eq6", "eq7", "eq8",
    ];
    let mut state = seed ^ 0x1407_c011_a95e;
    let mut next = move || {
        state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    };
    let design = |section: usize, unit: f32| {
        let (hpf, lpf) = if section == 0 {
            (25.0 + 400.0 * unit, 0.0)
        } else {
            (0.0, 2_000.0 + 14_000.0 * unit)
        };
        builtins::prepare_input_filter_pair(48_000, hpf, lpf)
            .expect("design")
            .targets[section]
    };
    let disable = |section: usize| {
        builtins::prepare_input_filter_pair(48_000, 0.0, 0.0)
            .expect("disable")
            .targets[section]
    };
    let record = |lanes, target: builtins::PreparedInputFilterTarget| {
        builtins_compiler::TrackInputRecord::PreparedFilter {
            target: builtins::PreparedInputFilterTarget { lanes, ..target },
        }
    };
    let mut last = [[None::<builtins::PreparedInputFilterTarget>; 2]; STRIPS.len()];
    let mut writes = Vec::new();
    // The strip a segment drives apart, and the block range it keeps background records off it.
    let mut apart: Option<(usize, core::ops::Range<usize>)> = None;
    for block in 1..blocks {
        let offset = block % swap_every;
        if offset == 0 {
            apart = None;
        }
        if offset == 4 && (block / swap_every) % 2 == 1 {
            let strip = (next() % STRIPS.len() as u64) as usize;
            let section = (next() % 2) as usize;
            let lanes = if next() % 2 == 0 {
                BuiltinLaneSelector::Left
            } else {
                BuiltinLaneSelector::Right
            };
            let unit = (next() % 10_000) as f32 / 10_000.0;
            writes.push(LiveWrite {
                block,
                strip: STRIPS[strip],
                record: record(lanes, design(section, unit)),
            });
            // Back to rest, on both channels, well before the swap.
            for section in 0..2 {
                writes.push(LiveWrite {
                    block: block + 12,
                    strip: STRIPS[strip],
                    record: record(BuiltinLaneSelector::Both, disable(section)),
                });
            }
            last[strip] = [None; 2];
            apart = Some((strip, block..swap_every * (block / swap_every + 1)));
        }
        for (strip, name) in STRIPS.iter().enumerate() {
            if apart
                .as_ref()
                .is_some_and(|(held, range)| *held == strip && range.contains(&block))
            {
                continue;
            }
            if next() % 100 >= 5 {
                continue;
            }
            let section = (next() % 2) as usize;
            let target = match next() % 10 {
                0..=3 => design(section, (next() % 10_000) as f32 / 10_000.0),
                4..=7 => disable(section),
                _ => match last[strip][section] {
                    Some(target) => target,
                    None => continue,
                },
            };
            last[strip][section] = Some(target);
            writes.push(LiveWrite {
                block,
                strip: name,
                record: record(BuiltinLaneSelector::Both, target),
            });
        }
    }
    writes.sort_by_key(|write| write.block);
    writes
}

/// #1407 attempt 2, the collapsed-stage invariant: the mono collapse is class A, so a chain that
/// collapses renders, block for block, exactly the bits of the same plans forced dual -- under
/// random live filter enables, disables and re-sends (`Both` and one-channel), collapse engages,
/// disengages and M3 re-engages, and plan swaps that carry each lane's live filter, integrators
/// included, into a successor that collapses at once.
///
/// Red (attempt-1 code, f44cf54bf): the rule-3 predicate reads channel `1`'s integrators, frozen
/// since the collapse engaged; a symmetric disable that completes collapsed then a symmetric
/// enable takes rule 3 on channel `0` and rule 4 on channel `1`. Red on a mutant that decides
/// channel `1`'s elision over its own frozen integrators while collapsed (the plan splits).
fn a_collapsed_chain_renders_the_forced_dual_bits(backend: Backend) {
    const QUANTUM_FRAMES: usize = 16;
    const RUN_BLOCKS: usize = 480;
    const SWAP_EVERY: usize = 40;
    let a = Session::compile(at_quantum(filtered_session(true), QUANTUM_FRAMES));
    let b = Session::compile(at_quantum(
        with_muted_track(filtered_session(true)),
        QUANTUM_FRAMES,
    ));
    let plane = |channel: u32| {
        (0..(QUANTUM_FRAMES * RUN_BLOCKS) as u32)
            .map(|frame| {
                let step = frame
                    .wrapping_mul(7_919)
                    .wrapping_add(channel * 31)
                    .wrapping_add(41)
                    % 1_999;
                (step as f32 - 999.5) / 2_000.0
            })
            .collect::<Vec<f32>>()
    };
    let feed = Feed {
        id: SOURCE,
        planes: [plane(0), plane(1)],
    };
    let mut reached = CollapseTally::default();
    // Seeds 0, 7 and 9 are each red on the channel-1 rule-3 predicate (attempt-1 semantics) at
    // both widths, first diverging at blocks 61, 56 and 52; seeds 1 and 2 were red at four lanes
    // only.
    for seed in [0_u64, 7, 9] {
        let writes = random_filter_writes(seed, RUN_BLOCKS, SWAP_EVERY);
        let (dual, _) = swapping_run(
            [&a, &b],
            &feed,
            [QUANTUM_FRAMES, RUN_BLOCKS, SWAP_EVERY],
            &writes,
            backend,
            true,
        );
        let (run, tally) = swapping_run(
            [&a, &b],
            &feed,
            [QUANTUM_FRAMES, RUN_BLOCKS, SWAP_EVERY],
            &writes,
            backend,
            false,
        );
        let first = run.iter().zip(&dual).position(|(run, dual)| run != dual);
        assert_eq!(
            first, None,
            "{backend:?} seed {seed}: the collapsed run left the forced-dual bits ({tally:?})"
        );
        reached.collapsed += tally.collapsed;
        reached.carried += tally.carried;
        for (sum, value) in reached.transitions.iter_mut().zip(tally.transitions) {
            *sum += value;
        }
    }
    // The generator reaches what the invariant is about: collapsed blocks, disengages, carried
    // swaps, and M3 proofs that re-engage a carried lane. (A within-plan re-engage is out of reach
    // for this chain: a one-channel input record latches its lane's `LIVE` term for the plan.)
    assert!(reached.collapsed > 0, "{backend:?}: {reached:?}");
    assert!(reached.transitions[0] > 0, "{backend:?}: {reached:?}");
    assert!(reached.transitions[2] > 0, "{backend:?}: {reached:?}");
    assert!(reached.carried > 0, "{backend:?}: {reached:?}");
}

#[test]
fn a_collapsed_chain_renders_the_forced_dual_bits_on_every_width() {
    for backend in backends() {
        a_collapsed_chain_renders_the_forced_dual_bits(backend);
    }
}

// Issue #1277 (slice 8 of #1269): a strip keeps its fader, mute and pan or matrix state through a
// plan swap, a ramp in flight at the swap block included, and a structural transaction that also
// changes a carried strip's live values sounds like "live edit, then structural edit" (D15-7).
//
// The cases run on live plans: every strip has latest-target cells, written with records stamped
// from the plan's own gate handle, as the C ABI does. A swap hands the predecessor's lane state to
// the successor, applies the predecessor's pending cells first (D8), and the successor applies its
// own dirty cells at its first block (D5).

use builtins::BuiltinLaneSelector;
use builtins_compiler::{TrackControlRecord, TrackFaderRecord};
use session::{DualMonoFader, MatrixOrPan, Vca};

/// The strip the ramp and retarget cases move: the first lane of the first bank of the predecessor
/// at either width, so a carry that exports a lane before its bank's pending cells apply loses it.
const MOVED: &str = "eq0";
/// The strip the mute cases fade: the only lane of the last bank at either width.
const FADED: &str = "eq8";

/// The nine-track session of gate 1 of #1276 with non-unity faders and non-centre pans, per track
/// and per channel.
fn strip_state_session() -> SessionModel {
    let mut model = filtered_session(false);
    for (index, track) in model.tracks.iter_mut().enumerate() {
        let step = index as f32;
        track.fader.left_db = -1.0 - 0.5 * step;
        track.fader.right_db = -2.0 + 0.25 * step;
        track.matrix_or_pan = MatrixOrPan::Pan {
            left: 0.95 - 0.05 * step,
            right: 0.5 + 0.04 * step,
            smoothing_samples: 16,
        };
    }
    model
}

fn track_mut<'a>(model: &'a mut SessionModel, track: &str) -> &'a mut session::Track {
    model
        .tracks
        .iter_mut()
        .find(|candidate| candidate.id.as_str() == track)
        .expect("track")
}

/// `model` with `track`'s fader dB set per channel.
fn with_fader(mut model: SessionModel, track: &str, db: [f32; 2]) -> SessionModel {
    let fader = &mut track_mut(&mut model, track).fader;
    fader.left_db = db[0];
    fader.right_db = db[1];
    model
}

/// `model` with `track`'s pan set, over `smoothing_samples`.
fn with_pan(
    mut model: SessionModel,
    track: &str,
    pan: [f32; 2],
    smoothing_samples: u32,
) -> SessionModel {
    track_mut(&mut model, track).matrix_or_pan = MatrixOrPan::Pan {
        left: pan[0],
        right: pan[1],
        smoothing_samples,
    };
    model
}

/// `model` with `track`'s mute set on both channels.
fn with_mute(mut model: SessionModel, track: &str, muted: bool) -> SessionModel {
    let fader = &mut track_mut(&mut model, track).fader;
    fader.left_mute = muted;
    fader.right_mute = muted;
    model
}

fn fader_both(db: f32, smoothing_samples: u32) -> TrackFaderRecord {
    TrackFaderRecord::FaderDb {
        lanes: BuiltinLaneSelector::Both,
        db,
        smoothing_samples,
    }
}

fn mute_both(muted: bool, smoothing_samples: u32) -> TrackFaderRecord {
    TrackFaderRecord::Mute {
        lanes: BuiltinLaneSelector::Both,
        muted,
        smoothing_samples,
    }
}

/// The record that moves a strip's pan to `pan` over `smoothing_samples`, as the classifier
/// derives it.
fn pan_record(pan: [f32; 2], smoothing_samples: u32) -> TrackControlRecord {
    let (matrix, _) = builtins_compiler::lower_matrix_or_pan(&MatrixOrPan::Pan {
        left: pan[0],
        right: pan[1],
        smoothing_samples,
    })
    .expect("a pan in domain");
    TrackControlRecord {
        matrix,
        smoothing_samples,
    }
}

/// One value written to a strip's cells.
#[derive(Clone, Copy, Debug)]
enum CellRecord {
    Fader(TrackFaderRecord),
    Matrix(TrackControlRecord),
    /// A live send's record, written to the cell of the route with this ID (#1284): the record a
    /// live commit writes to a send that follows its source strip's mute.
    Route(&'static str, host_core::RouteControlRecord),
}

/// A record written to `strip`'s cells before block `block` renders.
#[derive(Clone, Copy, Debug)]
struct CellWrite {
    block: usize,
    strip: &'static str,
    record: CellRecord,
}

/// A live plan with the handle that stamps and publishes its writes.
struct CellPlan {
    /// The plan, its handles and the gate handle every transaction below stamps from.
    live: LivePlan,
    /// The next revision a transaction stamps.
    next: u64,
}

impl CellPlan {
    fn prepared(
        prepared: Result<
            (host_core::PreparedHost, host_core::HostLiveControlHandles),
            host_core::PrepareDiagnostics,
        >,
    ) -> (host_core::PreparedHost, host_core::HostLiveControlHandles) {
        prepared.unwrap_or_else(|failure| panic!("prepare: {failure:?}"))
    }

    fn fresh(session: &Session, backend: Backend) -> Self {
        let (mut prepared, handles) = Self::prepared(
            host_core::test_only_prepare_host_runtime_with_live_controls_on(
                &session.compiled,
                &caps(),
                &LivePlan::request(),
                backend,
            ),
        );
        let gate = prepared
            .plan
            .take_revision_gate()
            .expect("a fresh plan's handle");
        Self {
            live: LivePlan {
                prepared,
                handles,
                gate,
            },
            next: 1,
        }
    }

    /// `session` prepared as the successor of the plan `inventory` describes, committing as
    /// `revision`, with the commit's gate steps a host takes: stamp `revision` again on the
    /// handle, which inherits the preparation's open stamp, and publish it.
    fn successor_at(
        session: &Session,
        base: host_core::SuccessorBase<'_>,
        backend: Backend,
    ) -> Result<Self, host_core::PrepareDiagnostics> {
        let revision = base.revision;
        let (mut prepared, handles) =
            host_core::test_only_prepare_host_runtime_with_live_controls_successor_on(
                &session.compiled,
                &caps(),
                &LivePlan::request(),
                base,
                backend,
            )?;
        let mut gate = prepared
            .plan
            .take_revision_gate()
            .expect("the successor's handle");
        let _ = gate
            .stamp(revision)
            .expect("the open revision stamps again");
        gate.publish(revision);
        Ok(Self {
            live: LivePlan {
                prepared,
                handles,
                gate,
            },
            next: revision + 1,
        })
    }

    fn successor(
        session: &Session,
        predecessor: &host_core::PlanStateInventory,
        committed: &SessionModel,
        edit_ramps: &EditRamps,
        backend: Backend,
    ) -> Self {
        Self::successor_at(
            session,
            host_core::SuccessorBase {
                edit_ramps,
                ..host_core::SuccessorBase::new(
                    predecessor,
                    committed,
                    successor::SUCCESSOR_REVISION,
                )
            },
            backend,
        )
        .unwrap_or_else(|failure| panic!("successor: {failure:?}"))
    }

    /// One transaction: every record stamped with one revision, then published.
    fn write_all(&mut self, writes: &[(&str, CellRecord)]) {
        if writes.is_empty() {
            return;
        }
        let revision = self.next;
        let stamp = self.live.gate.stamp(revision).expect("a legal revision");
        for (strip, record) in writes {
            if let CellRecord::Route(route, record) = *record {
                let position =
                    host_core::route_producer_position(&self.live.handles.route_controls, route)
                        .unwrap_or_else(|| panic!("route {route} is a live send"));
                self.live.handles.route_controls[position].write(stamp, record);
                continue;
            }
            let index = self
                .live
                .handles
                .strips
                .iter()
                .position(|candidate| &**candidate == *strip)
                .expect("strip");
            let producer = &mut self.live.handles.strip_controls[index];
            match *record {
                CellRecord::Fader(record) => producer.fader.write(stamp, record),
                CellRecord::Matrix(record) => producer.producer.write(stamp, record),
                CellRecord::Route(..) => unreachable!("written above"),
            }
        }
        self.live.gate.publish(revision);
        self.next = revision + 1;
    }

    fn write_block(&mut self, writes: &[CellWrite], block: usize) {
        let now: Vec<(&str, CellRecord)> = writes
            .iter()
            .filter(|write| write.block == block)
            .map(|write| (write.strip, write.record))
            .collect();
        self.write_all(&now);
    }
}

/// The successor of a [`cell_run`] swap.
struct Swap<'a> {
    /// The post-edit session.
    to: &'a Session,
    /// The predecessor's base: the model it was prepared from plus every record written to it.
    committed: &'a SessionModel,
    /// Records written to the successor's cells after it is prepared, before it is swapped in.
    successor_writes: &'a [(&'static str, CellRecord)],
    /// The ramp lengths the transaction's edits gave their rows (#1394 D6).
    edit_ramps: &'a EditRamps,
}

struct CellRun {
    blocks: Vec<Vec<u32>>,
    carry: CarryOutcome,
    /// The successor's `restarted_strips()`.
    restarted: Vec<Box<str>>,
    /// `(allocations, reallocations, deallocations)` and the render audit's `(allocations,
    /// deallocations)` over the hand-over plus the successor's first block.
    swap_allocator: ((u64, u64, u64), (u64, u64)),
    /// The predecessor's `live_values_superseded` count after the hand-over: the writes its
    /// cells held unread that the carry drain replaced (#1312 D2). Zero for a run without a swap
    /// that rendered no block after its writes.
    carry_superseded: u64,
    /// The successor's own count after its first block, as [`DirectRun::successor_superseded`].
    successor_superseded: u64,
    /// The last plan's count after the final block, as [`DirectRun::final_superseded`].
    final_superseded: u64,
}

/// Render `a` and write `writes` to the plan that renders their block next; with a `swap`, prepare
/// its successor at [`SWAP_BLOCK`], hand over and render on. Without one this is the reference
/// run: `a` rendered from frame 0 to [`BLOCKS`], the post-edit session fed the same records at the
/// same blocks.
fn cell_run(
    a: &Session,
    swap: Option<Swap<'_>>,
    feed: &Feed,
    writes: &[CellWrite],
    backend: Backend,
) -> CellRun {
    use bench_support::alloc as bench_alloc;
    let mut plan = CellPlan::fresh(a, backend);
    let mut blocks = Vec::with_capacity(BLOCKS);
    let mut carry = CarryOutcome::NotRequested;
    let mut restarted = Vec::new();
    let mut swap_allocator = ((0, 0, 0), (0, 0));
    let mut carry_superseded = 0;
    let mut successor_superseded = 0;
    plan.live.submit(feed, QUANTUM, 0);
    for block in 0..BLOCKS {
        plan.write_block(writes, block);
        if let (SWAP_BLOCK, Some(swap)) = (block, &swap) {
            let mut successor = CellPlan::successor(
                swap.to,
                &plan.live.prepared.inventory,
                swap.committed,
                swap.edit_ramps,
                backend,
            );
            restarted = successor.live.prepared.restarted_strips().to_vec();
            successor.write_all(swap.successor_writes);
            assert_eq!(
                successor
                    .live
                    .prepared
                    .sources
                    .adopt_persisting(&mut plan.live.prepared.sources),
                1
            );
            if block + 1 < BLOCKS {
                successor.live.submit(feed, QUANTUM, block + 1);
            }
            let mut output = vec![f32::NAN; QUANTUM * 2];
            engine::realtime::audit::reset();
            let mark = bench_alloc::current_thread_counters();
            carry = successor
                .live
                .prepared
                .plan
                .adopt_predecessor_plan(&mut plan.live.prepared.plan);
            successor.live.render_into(&mut output, QUANTUM);
            let delta = bench_alloc::current_thread_delta_since(mark);
            let audit = engine::realtime::audit::snapshot();
            swap_allocator = (
                (delta.allocations, delta.reallocations, delta.deallocations),
                (audit.allocations, audit.deallocations),
            );
            carry_superseded = plan.live.handles.live_values_superseded.get();
            successor_superseded = successor.live.handles.live_values_superseded.get();
            blocks.push(output.iter().map(|sample| sample.to_bits()).collect());
            plan = successor;
            continue;
        }
        if block + 1 < BLOCKS {
            plan.live.submit(feed, QUANTUM, block + 1);
        }
        blocks.push(plan.live.render(QUANTUM));
    }
    CellRun {
        blocks,
        carry,
        restarted,
        swap_allocator,
        carry_superseded,
        successor_superseded,
        final_superseded: plan.live.handles.live_values_superseded.get(),
    }
}

fn first_cell_difference(left: &CellRun, right: &CellRun) -> Option<usize> {
    assert_eq!(left.blocks.len(), right.blocks.len());
    left.blocks
        .iter()
        .zip(&right.blocks)
        .position(|(left, right)| left != right)
}

fn assert_swapped_like(run: &CellRun, reference: &CellRun, what: &str) {
    assert_eq!(run.carry, CarryOutcome::Carried, "{what}: the swap block");
    assert_eq!(
        first_cell_difference(run, reference),
        None,
        "{what}: every block equals the reference"
    );
}

// Issue #1288: a strip a plan swap adds during playback fades in from its first played block,
// over the session mute ramp, `D` samples late.

/// The track the fade-in cases add.
const FADE_TRACK: &str = "fade";

/// [`session_a`] plus [`AUX_SOURCE`] and a muted track `aux` on it, so the source persists into
/// a successor that adds a track on it. `aux` holds a true-peak limiter, so the output already
/// arrives 486 samples late and an added limited track grows no latency (#1287's case).
fn fade_base() -> SessionModel {
    let mut model = unlimited_fade_base();
    add_limiter(&mut model, "aux", "lim");
    model
}

/// [`fade_base`] without the limiter: the output arrives at once, so a fade at block 9 reaches
/// the output inside the run (#1392's anchored cases).
fn unlimited_fade_base() -> SessionModel {
    let mut model = session_a();
    add_source(&mut model, AUX_SOURCE);
    add_track(&mut model, "eq0", "aux", AUX_SOURCE, true);
    model
}

/// `model` plus [`FADE_TRACK`], a copy of `eq0` on `source`, with a default true-peak limiter
/// insert when `limited`.
fn with_fade_track(
    mut model: SessionModel,
    source: &str,
    limited: bool,
    muted: bool,
) -> SessionModel {
    add_track(&mut model, "eq0", FADE_TRACK, source, muted);
    if limited {
        track_mut(&mut model, FADE_TRACK)
            .inserts
            .effects
            .push(Effect {
                id: id("lim"),
                identity: EffectIdentity::Native {
                    effect_id: id("miso.true-peak-limiter"),
                },
                quality: EffectQuality::Normal,
                bypass: false,
                link_mode: LinkMode::Maximum,
                params: Vec::new(),
                sidechain: SidechainDeclaration::None,
            });
    }
    model
}

/// What a fade run rendered: every block's bits, each successor's fade arms (`(D, ramp)`),
/// each successor's `restarted_strips()`, and the render thread's `(allocations, reallocations,
/// deallocations)` over every hand-over and every block from the first swap on.
struct FadeRun {
    blocks: Vec<Vec<u32>>,
    arms: Vec<Vec<(u32, u32)>>,
    restarted: Vec<Vec<Box<str>>>,
    /// Each successor's `graph::test_only_carry_counts`.
    carries: Vec<[usize; 5]>,
    allocator: (u64, u64, u64),
}

/// One structural swap of a [`fade_run`]: before block `block` renders, `to` is prepared as the
/// running plan's successor, with `committed` (the running plan's model) as its base, and adopted.
struct FadeSwap<'a> {
    block: usize,
    to: &'a Session,
    committed: &'a SessionModel,
    /// The block before which the successor is prepared, when earlier than `block`: it is prepared
    /// against the plan running then, which renders on until `block`.
    prepared_at: Option<usize>,
}

/// A source a [`fade_run`]'s first successor adds and the host starts with
/// `seek_at(id, 2, A, A)` (#1288 gate 4(b)) before the first successor's first block, then feeds
/// from `A`.
struct Anchored<'a> {
    feed: &'a Feed,
    anchor_block: usize,
}

/// Render `a` with live controls, fed `feeds` (generation 1) one block ahead, and adopt each of
/// `swaps` directly at its block. `writes` go to the plan rendering their block. Without swaps
/// this is a reference: `a` fresh from frame 0.
fn fade_run(
    a: &Session,
    swaps: &[FadeSwap<'_>],
    feeds: &[&Feed],
    anchored: Option<Anchored<'_>>,
    writes: &[CellWrite],
    backend: Backend,
) -> FadeRun {
    use bench_support::alloc as bench_alloc;
    let rate = a.compiled.sample_rate().0;
    let mut plan = CellPlan::fresh(a, backend);
    let mut arms = Vec::new();
    let mut restarted = Vec::new();
    let mut carries = Vec::new();
    let mut allocator = (0, 0, 0);
    let mut add = |delta: bench_alloc::Counters| {
        allocator.0 += delta.allocations;
        allocator.1 += delta.reallocations;
        allocator.2 += delta.deallocations;
    };
    for feed in feeds {
        feed.submit(&mut plan.live.prepared.sources, rate, 0);
    }
    let mut blocks = Vec::with_capacity(BLOCKS);
    // A successor prepared before its swap block (`FadeSwap::prepared_at`).
    let mut early: Option<CellPlan> = None;
    for block in 0..BLOCKS {
        if let Some(swap) = swaps.iter().find(|swap| swap.block == block) {
            let first = arms.is_empty();
            let mut successor = match early.take() {
                Some(successor) => successor,
                None => CellPlan::successor(
                    swap.to,
                    &plan.live.prepared.inventory,
                    swap.committed,
                    &EditRamps::new(),
                    backend,
                ),
            };
            arms.push(
                graph::test_only_fade_arms(&mut successor.live.prepared.plan)
                    .expect("a graph plan"),
            );
            restarted.push(successor.live.prepared.restarted_strips().to_vec());
            carries.push(graph::test_only_carry_counts(
                &mut successor.live.prepared.plan,
            ));
            successor
                .live
                .prepared
                .sources
                .adopt_persisting(&mut plan.live.prepared.sources);
            if first && let Some(anchored) = &anchored {
                let anchor = (anchored.anchor_block * QUANTUM) as u64;
                let sources = &mut successor.live.prepared.sources;
                sources
                    .seek_at(anchored.feed.id.as_bytes(), 2, anchor, anchor)
                    .expect("anchored seek");
                submit_blocks(
                    sources,
                    anchored.feed.id,
                    anchored.feed,
                    2,
                    anchored.anchor_block..BLOCKS,
                );
            }
            let mark = bench_alloc::current_thread_counters();
            let carry = successor
                .live
                .prepared
                .plan
                .adopt_predecessor_plan(&mut plan.live.prepared.plan);
            add(bench_alloc::current_thread_delta_since(mark));
            assert_eq!(carry, CarryOutcome::Carried, "the swap at block {block}");
            plan = successor;
        }
        if let Some(swap) = swaps.iter().find(|swap| {
            swap.prepared_at
                .is_some_and(|at| at == block && at < swap.block)
        }) {
            early = Some(CellPlan::successor(
                swap.to,
                &plan.live.prepared.inventory,
                swap.committed,
                &EditRamps::new(),
                backend,
            ));
        }
        plan.write_block(writes, block);
        if block + 1 < BLOCKS {
            for feed in feeds {
                feed.submit(&mut plan.live.prepared.sources, rate, block + 1);
            }
        }
        let mut output = vec![f32::NAN; QUANTUM * 2];
        let mark = bench_alloc::current_thread_counters();
        plan.live.render_into(&mut output, QUANTUM);
        let delta = bench_alloc::current_thread_delta_since(mark);
        if swaps.first().is_some_and(|swap| block >= swap.block) {
            add(delta);
        }
        blocks.push(output.iter().map(|sample| sample.to_bits()).collect());
    }
    FadeRun {
        blocks,
        arms,
        restarted,
        carries,
        allocator,
    }
}

/// The unmute a fade reference writes to [`FADE_TRACK`] before block `block`, over `ramp`.
fn unmute_fade(block: usize, ramp: u32) -> CellWrite {
    CellWrite {
        block,
        strip: FADE_TRACK,
        record: CellRecord::Fader(mute_both(false, ramp)),
    }
}

fn first_fade_difference(left: &FadeRun, right: &FadeRun) -> Option<usize> {
    left.blocks
        .iter()
        .zip(&right.blocks)
        .position(|(left, right)| left != right)
}

/// Issue #1288 gates 3, 4(a), 4(c) and 6 at one width: B adds [`FADE_TRACK`] (a) on the playing
/// [`SOURCE`], `D = 0`, and (c) with a 486-sample true-peak limiter on [`AUX_SOURCE`], fed exact
/// zeros before the swap, `D = 486`. Every block of the swapped run equals B with the track
/// muted, rendered fresh from frame 0, given a live unmute over the session mute ramp at the
/// first block at or after `S + D`: the added track fades in there and every other strip is
/// untouched. The hand-over and every successor block allocate and free nothing.
fn an_added_track_fades_in_from_its_first_played_block(backend: Backend) {
    let ramp = LiveRamps::for_session(&fade_base()).mute_samples;
    assert!(ramp > 0, "the session mute ramp is not a step");
    let (kept, aux) = fade_feeds();
    let a = Session::compile(fade_base());
    for (case, source, limited, delay) in [
        ("(a) a playing source", SOURCE, false, 0_u32),
        ("(c) a limiter insert", AUX_SOURCE, true, 486),
    ] {
        let b = Session::compile(with_fade_track(fade_base(), source, limited, false));
        let muted = Session::compile(with_fade_track(fade_base(), source, limited, true));
        let fire_block = ((SWAP_BLOCK * QUANTUM) as u32 + delay).div_ceil(QUANTUM as u32) as usize;
        let swap = [FadeSwap {
            block: SWAP_BLOCK,
            to: &b,
            committed: &a.model,
            prepared_at: None,
        }];
        let feeds = [&kept, &aux];
        let run = fade_run(&a, &swap, &feeds, None, &[], backend);
        assert_eq!(
            run.arms[0],
            vec![(delay, ramp)],
            "{backend:?} {case}: D and the ramp"
        );
        assert_eq!(
            run.allocator,
            (0, 0, 0),
            "{backend:?} {case}: allocator calls"
        );
        let reference = fade_run(
            &muted,
            &[],
            &feeds,
            None,
            &[unmute_fade(fire_block, ramp)],
            backend,
        );
        assert_eq!(
            first_fade_difference(&run, &reference),
            None,
            "{backend:?} {case}: every block equals the reference"
        );
        // The oracle can fail: a fade one block early is different bits.
        let early = fade_run(
            &muted,
            &[],
            &feeds,
            None,
            &[unmute_fade(fire_block - 1, ramp)],
            backend,
        );
        assert_ne!(
            early.blocks, reference.blocks,
            "{backend:?} {case}: the fire block is observable"
        );
    }
}

/// [`SOURCE`]'s feed, and [`AUX_SOURCE`]'s: exact zeros before [`SWAP_BLOCK`], audio from it.
fn fade_feeds() -> (Feed, Feed) {
    let aux = Feed::new(AUX_SOURCE, 2);
    let aux = spliced(AUX_SOURCE, &aux, |block| {
        (block >= SWAP_BLOCK).then_some(block)
    });
    (Feed::new(SOURCE, 1), aux)
}

/// Issue #1288 gates 3, 4 and 6 on every width. Red if every strip dips at the swap, if the added
/// track enters at full gain, or if its fade runs while its latent insert still emits its at-rest
/// zeros.
#[test]
fn an_added_track_fades_in_from_its_first_played_block_on_every_width() {
    for backend in backends() {
        an_added_track_fades_in_from_its_first_played_block(backend);
    }
}

// Issue #1363 (slice 1): a route a plan swap adds during playback fades in over the session mute
// ramp, as #1288's added strips do.

/// The submix the #1363 cases route into.
const ROUTE_BUS: &str = "bus";

/// `model` plus route `route` from track `track`'s `tap` into `destination` (a copy of `eq0-main`'s
/// values), muted when `muted`.
fn with_route(
    mut model: SessionModel,
    route: &str,
    track: &str,
    tap: session::SendTap,
    destination: session::RouteDestination,
    muted: bool,
) -> SessionModel {
    let mut added = model
        .routes
        .iter()
        .find(|route| route.id.as_str() == "eq0-main")
        .expect("template route")
        .clone();
    added.id = id(route);
    added.source = RouteSource::Track {
        track_id: id(track),
        tap,
    };
    added.destination = destination;
    added.mute = muted;
    model.routes.push(added);
    model
}

fn into_bus() -> session::RouteDestination {
    session::RouteDestination::SubmixInput {
        submix_id: id(ROUTE_BUS),
    }
}

fn into_output() -> session::RouteDestination {
    session::RouteDestination::OutputInput {
        output_id: id("main-out"),
    }
}

/// The #1363 base: [`session_a`] plus a playing track `aux` on [`AUX_SOURCE`], and the submix
/// [`ROUTE_BUS`] fed by a muted track `late` whose true-peak limiter makes the bus input arrive 486
/// samples late, so a route added into the bus from an unlimited track takes a 486-sample
/// compensation line on its destination edge. The output arrives 486 samples late already, so a
/// limited added track grows no latency.
fn route_fade_base() -> SessionModel {
    route_fade_base_limited(true)
}

/// [`route_fade_base`], with `late`'s limiter only when `limited`: without it the output arrives
/// at once, so a fade late in the run reaches the output inside it.
fn route_fade_base_limited(limited: bool) -> SessionModel {
    let mut model = session_a();
    add_source(&mut model, AUX_SOURCE);
    add_track(&mut model, "eq0", "aux", AUX_SOURCE, false);
    add_track(&mut model, "eq0", "late", SOURCE, true);
    if limited {
        add_limiter(&mut model, "late", "lim");
    }
    model
        .submixes
        .push(session::Submix::unity(id(ROUTE_BUS), &model.console));
    let model = with_route(
        model,
        "late-bus",
        "late",
        session::SendTap::PostFader,
        into_bus(),
        false,
    );
    let mut model = with_route(
        model,
        "bus-main",
        "eq0",
        session::SendTap::PostPan,
        into_output(),
        false,
    );
    let bus = model
        .routes
        .iter_mut()
        .find(|route| route.id.as_str() == "bus-main")
        .expect("the bus route");
    bus.source = RouteSource::Submix {
        submix_id: id(ROUTE_BUS),
        tap: session::SendTap::PostPan,
    };
    model
}

/// The live unmute a #1363 reference writes to route `route` of `model` before block `block`,
/// over `ramp`: the record a live commit of `mute: false` writes.
fn unmute_route(model: &SessionModel, route: &'static str, block: usize, ramp: u32) -> CellWrite {
    route_mute_write(model, route, block, ramp, false)
}

/// The live record a commit of `mute: muted` on route `route` of `model` writes before block
/// `block`, over `ramp`.
fn route_mute_write(
    model: &SessionModel,
    route: &'static str,
    block: usize,
    ramp: u32,
    muted: bool,
) -> CellWrite {
    let values = model
        .routes
        .iter()
        .find(|candidate| candidate.id.as_str() == route)
        .expect("the route");
    let matrix = &values.channel_matrix;
    let record = host_core::route_record(
        values.gain_db,
        [matrix.ll, matrix.lr, matrix.rl, matrix.rr],
        muted,
        [false; 2],
        ramp,
    )
    .expect("a send record");
    CellWrite {
        block,
        strip: "aux",
        record: CellRecord::Route(route, record),
    }
}

/// Issue #1363 gates 1, 3 and 8 at one width. B adds route `aux-bus` from the playing track `aux`
/// into [`ROUTE_BUS`], whose destination edge carries a 486-sample compensation line that starts
/// at rest. The route waits for nothing (`D = 0`: the line sits after the route's op, so the fade
/// enters it and the bus hears the fade start as the line delivers, 486 samples after the swap).
/// Every block equals B with the route muted, rendered fresh from frame 0, given a live unmute
/// over the session mute ramp at the swap block, and the hand-over and every successor block
/// allocate and free nothing. Inner claims:
///
/// * (b) the same route `post_fader` into the output, which has no lane (a transition route):
///   every block equals the reference of the same route from `pre_fader` (a unity fader moves no
///   bit), which has a lane;
/// * (c) with `aux` fed exact zeros, both swaps leave every block equal to A's own run: the arm
///   touches no other route and no neighbour's fold.
fn an_added_route_fades_in_after_its_line_fills(backend: Backend) {
    let base = route_fade_base();
    let ramp = LiveRamps::for_session(&base).mute_samples;
    let a = Session::compile(base.clone());
    let (kept, aux) = (Feed::new(SOURCE, 1), Feed::new(AUX_SOURCE, 2));
    let zeros = spliced(AUX_SOURCE, &aux, |_| None);
    let swap_to = |to: &Session| {
        fade_run(
            &a,
            &[FadeSwap {
                block: SWAP_BLOCK,
                to,
                committed: &a.model,
                prepared_at: None,
            }],
            &[&kept, &aux],
            None,
            &[],
            backend,
        )
    };
    let post = session::SendTap::PostFader;
    for (case, route, destination, reference_tap) in [
        ("(a) into a submix", "aux-bus", into_bus(), post),
        (
            "(b) into the output",
            "aux-out",
            into_output(),
            session::SendTap::PreFader,
        ),
    ] {
        let added =
            |tap, muted| with_route(base.clone(), route, "aux", tap, destination.clone(), muted);
        let b = Session::compile(added(post, false));
        let run = swap_to(&b);
        assert_eq!(
            run.arms[0],
            vec![(0, ramp)],
            "{backend:?} {case}: D and the ramp"
        );
        assert_eq!(
            run.allocator,
            (0, 0, 0),
            "{backend:?} {case}: allocator calls"
        );
        // #2069: the route added muted arms too when its mute is live (a lane: (a)), so an
        // unmute before its fire opens it at the fire; with no lane ((b)) nothing can unmute it
        // before the fire, and it arms nothing.
        let lane = case.starts_with("(a)");
        assert_eq!(
            swap_to(&Session::compile(added(post, true))).arms[0],
            if lane { vec![(0, ramp)] } else { Vec::new() },
            "{backend:?} {case}: a route added muted arms only when its mute is live"
        );
        let muted_model = added(reference_tap, true);
        let muted = Session::compile(muted_model.clone());
        let reference = |block| {
            fade_run(
                &muted,
                &[],
                &[&kept, &aux],
                None,
                &[unmute_route(&muted_model, route, block, ramp)],
                backend,
            )
        };
        let at_swap = reference(SWAP_BLOCK);
        assert_eq!(
            first_fade_difference(&run, &at_swap),
            None,
            "{backend:?} {case}: every block equals the reference"
        );
        // The oracle can fail: a fade one block late is different bits.
        assert_ne!(
            reference(SWAP_BLOCK + 1).blocks,
            at_swap.blocks,
            "{backend:?} {case}: the fire block is observable"
        );
        // (c) Only the route moves.
        let silent = |to: Option<&Session>| {
            let swaps: Vec<FadeSwap<'_>> = to
                .into_iter()
                .map(|to| FadeSwap {
                    block: SWAP_BLOCK,
                    to,
                    committed: &a.model,
                    prepared_at: None,
                })
                .collect();
            fade_run(&a, &swaps, &[&kept, &zeros], None, &[], backend)
        };
        assert_eq!(
            first_fade_difference(&silent(Some(&b)), &silent(None)),
            None,
            "{backend:?} {case}: with the route's source silent, the swap moves no bit"
        );
    }
}

/// Issue #1363 gates 1, 3 and 8 on every width. Red if an added route enters at a step at the
/// swap, if a route with no lane still folds and steps, or if the arm moves a neighbour's bits;
/// or (#2069) if preparation skips a muted added route with a lane (no arm: an unmute before the
/// fire opens it at once) or arms a muted one with none.
#[test]
fn an_added_route_fades_in_after_its_line_fills_on_every_width() {
    for backend in backends() {
        an_added_route_fades_in_after_its_line_fills(backend);
    }
}

/// Issue #1363 gate 6 at one width: B adds [`FADE_TRACK`] with a `pre_fader` send `fade-bus` into
/// [`ROUTE_BUS`], (a) on the playing [`SOURCE`], `D = 0`, and (c) with a 486-sample true-peak
/// limiter on [`AUX_SOURCE`], fed exact zeros before the swap, `D = 486`. The send's tap
/// precedes the track's armed fader, so it fades with it: every block equals B with the track and
/// the send muted, rendered fresh, given both live unmutes at the fader's fire block.
fn an_added_strips_pre_fader_send_fades_with_it(backend: Backend) {
    let base = route_fade_base();
    let ramp = LiveRamps::for_session(&base).mute_samples;
    let a = Session::compile(base.clone());
    let kept = Feed::new(SOURCE, 1);
    let aux = Feed::new(AUX_SOURCE, 2);
    let aux = spliced(AUX_SOURCE, &aux, |block| {
        (block >= SWAP_BLOCK).then_some(block)
    });
    for (case, source, limited, delay) in [
        ("(a) a playing source", SOURCE, false, 0_u32),
        ("(c) a limiter insert", AUX_SOURCE, true, 486),
    ] {
        let model = |muted| {
            with_route(
                with_fade_track(base.clone(), source, limited, muted),
                "fade-bus",
                FADE_TRACK,
                session::SendTap::PreFader,
                into_bus(),
                muted,
            )
        };
        let b = Session::compile(model(false));
        let muted_model = model(true);
        let muted = Session::compile(muted_model.clone());
        let fire_block = ((SWAP_BLOCK * QUANTUM) as u32 + delay).div_ceil(QUANTUM as u32) as usize;
        let run = fade_run(
            &a,
            &[FadeSwap {
                block: SWAP_BLOCK,
                to: &b,
                committed: &a.model,
                prepared_at: None,
            }],
            &[&kept, &aux],
            None,
            &[],
            backend,
        );
        assert_eq!(
            run.arms[0],
            vec![(delay, ramp), (delay, ramp)],
            "{backend:?} {case}: the fader's and the send's D and ramp"
        );
        let reference = fade_run(
            &muted,
            &[],
            &[&kept, &aux],
            None,
            &[
                unmute_fade(fire_block, ramp),
                unmute_route(&muted_model, "fade-bus", fire_block, ramp),
            ],
            backend,
        );
        assert_eq!(
            first_fade_difference(&run, &reference),
            None,
            "{backend:?} {case}: every block equals the reference"
        );
    }
    // (d) The send is added by a later swap, before the strip's fade fires: B adds the track on
    // [`ADDED_SOURCE`], anchored at [`FADE_ANCHOR_BLOCK`]; C, adopted one block after B, adds the
    // send. The send waits with the strip's claim, so both fire at the anchor.
    let mut unlimited = route_fade_base_limited(false);
    add_source(&mut unlimited, ADDED_SOURCE);
    let a = Session::compile(unlimited.clone());
    let track = |muted| with_fade_track(unlimited.clone(), ADDED_SOURCE, false, muted);
    let send = |model, muted| {
        with_route(
            model,
            "fade-bus",
            FADE_TRACK,
            session::SendTap::PreFader,
            into_bus(),
            muted,
        )
    };
    let b1 = Session::compile(track(false));
    let b2 = Session::compile(send(track(false), false));
    let muted_model = send(track(true), true);
    let muted = Session::compile(muted_model.clone());
    let (added, reference_added) = anchored_feeds();
    let run = two_swap_run(
        &a,
        &b1,
        &b2,
        SWAP_BLOCK + 1,
        Some(Anchored {
            feed: &added,
            anchor_block: FADE_ANCHOR_BLOCK,
        }),
        &[&kept, &aux],
        backend,
    );
    let reference = fade_run(
        &muted,
        &[],
        &[&kept, &aux, &reference_added],
        None,
        &[
            unmute_fade(FADE_ANCHOR_BLOCK, ramp),
            unmute_route(&muted_model, "fade-bus", FADE_ANCHOR_BLOCK, ramp),
        ],
        backend,
    );
    assert_eq!(
        first_fade_difference(&run, &reference),
        None,
        "{backend:?} (d): a send added before its strip's fire waits for the strip"
    );
}

/// Issue #1363 gate 6 on every width. Red if a pre-fader send of an added strip is left unarmed
/// beside its armed fader (it enters the bus at full level at adoption, a step), or if a send that
/// a later swap adds before the strip's fire does not wait for the strip.
#[test]
fn an_added_strips_pre_fader_send_fades_with_it_on_every_width() {
    for backend in backends() {
        an_added_strips_pre_fader_send_fades_with_it(backend);
    }
}

// Issue #1392: an added strip's pending fade-in travels with its fader lane across a later swap.

/// The block [`ADDED_SOURCE`] is anchored to in the #1392 cases: three blocks after the swap.
const FADE_ANCHOR_BLOCK: usize = SWAP_BLOCK + 3;

/// [`fade_base`] plus [`ADDED_SOURCE`] and [`FADE_TRACK`] on it.
fn anchored_fade_model(muted: bool) -> SessionModel {
    let mut model = unlimited_fade_base();
    add_source(&mut model, ADDED_SOURCE);
    with_fade_track(model, ADDED_SOURCE, false, muted)
}

/// [`ADDED_SOURCE`]'s feed in the swapped run, and the reference's: exact zeros before
/// [`FADE_ANCHOR_BLOCK`], the same blocks from it.
fn anchored_feeds() -> (Feed, Feed) {
    let added = Feed::new(ADDED_SOURCE, 3);
    let reference = spliced(ADDED_SOURCE, &added, |block| {
        (block >= FADE_ANCHOR_BLOCK).then_some(block)
    });
    (added, reference)
}

/// The two-swap run of the #1392 cases: `b1` at [`SWAP_BLOCK`], `b2` at `second`.
fn two_swap_run(
    a: &Session,
    b1: &Session,
    b2: &Session,
    second: usize,
    anchored: Option<Anchored<'_>>,
    feeds: &[&Feed],
    backend: Backend,
) -> FadeRun {
    two_swap_run_prepared(a, b1, b2, (second, None), anchored, feeds, backend)
}

/// [`two_swap_run`] whose second successor is prepared before block `second.1` when given.
fn two_swap_run_prepared(
    a: &Session,
    b1: &Session,
    b2: &Session,
    second: (usize, Option<usize>),
    anchored: Option<Anchored<'_>>,
    feeds: &[&Feed],
    backend: Backend,
) -> FadeRun {
    let swaps = [
        FadeSwap {
            block: SWAP_BLOCK,
            to: b1,
            committed: &a.model,
            prepared_at: None,
        },
        FadeSwap {
            block: second.0,
            to: b2,
            committed: &b1.model,
            prepared_at: second.1,
        },
    ];
    fade_run(a, &swaps, feeds, anchored, &[], backend)
}

/// Issue #1392 gates 1 and 5 at one width. (b) The added track waits for its anchored source at
/// block 9 while a second transaction (a muted track) is adopted at block 7: the carried entry
/// keeps waiting and fires at the anchor. (c) The added limited track's source plays from block 6,
/// so its fade is scheduled for sample 1254 (block 10); the second swap at block 8 keeps that
/// sample. Each run equals the post-edit session with the track muted, given a live unmute at the
/// one-swap fire block; neither swap block allocates or frees.
fn a_pending_fade_survives_a_second_swap(backend: Backend) {
    let ramp = LiveRamps::for_session(&fade_base()).mute_samples;
    let (kept, aux) = fade_feeds();
    // (b) Waiting.
    let a = Session::compile(unlimited_fade_base());
    let (added, reference_added) = anchored_feeds();
    let b1 = Session::compile(anchored_fade_model(false));
    let b2 = Session::compile(with_muted_track(anchored_fade_model(false)));
    let muted = Session::compile(with_muted_track(anchored_fade_model(true)));
    let anchored = Anchored {
        feed: &added,
        anchor_block: FADE_ANCHOR_BLOCK,
    };
    let run = two_swap_run(
        &a,
        &b1,
        &b2,
        SWAP_BLOCK + 1,
        Some(anchored),
        &[&kept, &aux],
        backend,
    );
    let reference = fade_run(
        &muted,
        &[],
        &[&kept, &aux, &reference_added],
        None,
        &[unmute_fade(FADE_ANCHOR_BLOCK, ramp)],
        backend,
    );
    assert_eq!(
        first_fade_difference(&run, &reference),
        None,
        "{backend:?} (b): a waiting fade fires at the anchor after a second swap"
    );
    assert_eq!(run.allocator, (0, 0, 0), "{backend:?} (b): allocator calls");
    assert_audible(
        &muted,
        &[&kept, &aux, &reference_added],
        &reference,
        backend,
    );
    // (c) FireAt.
    let a = Session::compile(fade_base());
    let limited = |muted: bool| with_fade_track(fade_base(), AUX_SOURCE, true, muted);
    let b1 = Session::compile(limited(false));
    let b2 = Session::compile(with_muted_track(limited(false)));
    let muted = Session::compile(with_muted_track(limited(true)));
    let fire_block = (SWAP_BLOCK * QUANTUM + 486).div_ceil(QUANTUM);
    let run = two_swap_run(&a, &b1, &b2, SWAP_BLOCK + 2, None, &[&kept, &aux], backend);
    let reference = fade_run(
        &muted,
        &[],
        &[&kept, &aux],
        None,
        &[unmute_fade(fire_block, ramp)],
        backend,
    );
    assert_eq!(
        first_fade_difference(&run, &reference),
        None,
        "{backend:?} (c): a scheduled fade keeps its sample across a second swap"
    );
    assert_eq!(run.allocator, (0, 0, 0), "{backend:?} (c): allocator calls");
}

/// Issue #1392 gates 1 and 5 on every width. Red if the successor does not re-arm from the
/// inventory (the stem enters at full gain), restarts a carried `FireAt` as `Waiting` (the fade
/// fires `D` late), or allocates at either swap block.
#[test]
fn a_pending_fade_survives_a_second_swap_on_every_width() {
    for backend in backends() {
        a_pending_fade_survives_a_second_swap(backend);
    }
}

/// Issue #1392 gate 2 at one width: the added track on the playing [`SOURCE`] fires at the swap
/// block (no output latency, so the fade is heard at once); a second transaction adopted
/// mid-ramp (block 7) or after the ramp (block 10) re-arms nothing, since the fired entry's row is
/// dropped, and its lane import overwrites the arm's mute with the carried, unmuted state: the run
/// equals the one-swap fade, and the track is heard after the second swap.
fn a_fired_fade_does_not_repeat(backend: Backend) {
    let ramp = LiveRamps::for_session(&unlimited_fade_base()).mute_samples;
    assert!(
        ramp as usize > QUANTUM && SWAP_BLOCK * QUANTUM + (ramp as usize) < 10 * QUANTUM,
        "block 7 is inside the ramp and block 10 after it"
    );
    let (kept, aux) = fade_feeds();
    let feeds = [&kept, &aux];
    let model = |extra: bool, muted: bool| {
        let model = with_fade_track(unlimited_fade_base(), SOURCE, false, muted);
        if extra {
            with_muted_track(model)
        } else {
            model
        }
    };
    let a = Session::compile(unlimited_fade_base());
    let b1 = Session::compile(model(false, false));
    let b2 = Session::compile(model(true, false));
    let muted = Session::compile(model(true, true));
    let reference = fade_run(
        &muted,
        &[],
        &feeds,
        None,
        &[unmute_fade(SWAP_BLOCK, ramp)],
        backend,
    );
    let silent = fade_run(&muted, &[], &feeds, None, &[], backend);
    // The second successor prepared after the fire drops the fired row; one prepared before it
    // (at the first swap block, before that block renders) re-arms the strip, and its carry copies
    // `Done` while the lane import must overwrite the arm's mute with the carried lane's state.
    for (second, prepared_at) in [
        (SWAP_BLOCK + 1, None),
        (10, None),
        (SWAP_BLOCK + 1, Some(SWAP_BLOCK)),
        (10, Some(SWAP_BLOCK)),
    ] {
        let run = two_swap_run_prepared(&a, &b1, &b2, (second, prepared_at), None, &feeds, backend);
        // #2080: `b2`'s added muted track arms too (its fire stays muted): one arm in every
        // case. The fade track's re-arm is the second, only when prepared before the fire.
        let muted_track_arm = (0, ramp);
        let expected = if prepared_at.is_some() {
            vec![muted_track_arm, (0, ramp)]
        } else {
            vec![muted_track_arm]
        };
        assert_eq!(
            run.arms[1], expected,
            "{backend:?}: only a successor prepared before the fire re-arms the strip, and the \
             added muted track arms"
        );
        assert_eq!(
            first_fade_difference(&run, &reference),
            None,
            "{backend:?}: a second swap at block {second} repeats no fade"
        );
        assert!(
            (second..BLOCKS).all(|block| reference.blocks[block] != silent.blocks[block]),
            "{backend:?}: the track is heard in every block from the second swap at {second}"
        );
    }
}

/// Issue #1392 gate 2 on every width. Red if the second swap's lane import does not overwrite the
/// arm's mute with the carried lane's state (the playing strip is muted again after the swap).
#[test]
fn a_fired_fade_does_not_repeat_on_every_width() {
    for backend in backends() {
        a_fired_fade_does_not_repeat(backend);
    }
}

/// Issue #1877 gate 4 at one width: #1363 gate 6's added track [`FADE_TRACK`] with a `pre_fader`
/// send `fade-bus` into [`ROUTE_BUS`], and a second transaction (a muted track) adopted before the
/// send's fade fires. (a) Waiting: the track is on [`ADDED_SOURCE`], anchored at
/// [`FADE_ANCHOR_BLOCK`], and the second swap lands one block after the first. (b) FireAt: the
/// track holds a 486-sample true-peak limiter on [`AUX_SOURCE`], fed exact zeros before the swap,
/// so both fades are scheduled for block 10, and the second swap lands at block 8. (c) Fired: (b)'s
/// send has fired at block 10 when the second swap lands at block 11, its successor prepared then
/// or at block 8, before the fire. Every run equals the one-swap run straight to the second
/// session, and neither swap block allocates or frees. In (b) and (c) a live mute of the send at
/// block 11 is heard: the second plan holds no arm left over that would swallow it. (d) The lane-less
/// transition route: an added `post_fader` route from the playing `aux` into the output fires at
/// the first swap; the second successor, prepared before that fire and adopted mid-ramp at block
/// 7, re-arms it and continues its ramp; so does one prepared after the fire (#1953b: the entry
/// fired and has not settled) and adopted mid-ramp at block 8. (e) The second transaction restarts the track (#1324)
/// before the fire: the send takes the restarted strip's fresh arm, as a send the second
/// transaction adds does.
fn a_pending_route_fade_survives_a_second_swap(backend: Backend) {
    let send = |model| {
        with_route(
            model,
            "fade-bus",
            FADE_TRACK,
            session::SendTap::PreFader,
            into_bus(),
            false,
        )
    };
    let one_swap = |a: &Session, b: &Session, anchored, feeds: &[&Feed], writes: &[CellWrite]| {
        let swap = [FadeSwap {
            block: SWAP_BLOCK,
            to: b,
            committed: &a.model,
            prepared_at: None,
        }];
        fade_run(a, &swap, feeds, anchored, writes, backend)
    };
    // (a) Waiting.
    let kept = Feed::new(SOURCE, 1);
    let mut unlimited = route_fade_base_limited(false);
    add_source(&mut unlimited, ADDED_SOURCE);
    let a = Session::compile(unlimited.clone());
    let b1 = Session::compile(send(with_fade_track(
        unlimited.clone(),
        ADDED_SOURCE,
        false,
        false,
    )));
    let b2 = Session::compile(with_muted_track(b1.model.clone()));
    let (added, _) = anchored_feeds();
    let anchored = || {
        Some(Anchored {
            feed: &added,
            anchor_block: FADE_ANCHOR_BLOCK,
        })
    };
    let run = two_swap_run(&a, &b1, &b2, SWAP_BLOCK + 1, anchored(), &[&kept], backend);
    let reference = one_swap(&a, &b2, anchored(), &[&kept], &[]);
    assert_eq!(
        first_fade_difference(&run, &reference),
        None,
        "{backend:?} (a): a waiting send fades in at the anchor after a second swap"
    );
    assert_eq!(run.allocator, (0, 0, 0), "{backend:?} (a): allocator calls");
    // (b) FireAt.
    let aux = Feed::new(AUX_SOURCE, 2);
    let aux = spliced(AUX_SOURCE, &aux, |block| {
        (block >= SWAP_BLOCK).then_some(block)
    });
    let feeds = [&kept, &aux];
    let base = route_fade_base_limited(false);
    let a = Session::compile(base.clone());
    let b1 = Session::compile(send(with_fade_track(base, AUX_SOURCE, true, false)));
    let b2 = Session::compile(with_muted_track(b1.model.clone()));
    let fire_block = (SWAP_BLOCK * QUANTUM + 486).div_ceil(QUANTUM);
    assert_eq!(fire_block, 10, "the fades are scheduled for block 10");
    let ramp = LiveRamps::for_session(&b2.model).mute_samples;
    let mut mute = unmute_route(&b2.model, "fade-bus", BLOCKS - 1, ramp);
    let CellRecord::Route(_, record) = &mut mute.record else {
        unreachable!("a send record")
    };
    let values = b2
        .model
        .routes
        .iter()
        .find(|route| route.id.as_str() == "fade-bus")
        .expect("the send");
    let matrix = &values.channel_matrix;
    *record = host_core::route_record(
        values.gain_db,
        [matrix.ll, matrix.lr, matrix.rl, matrix.rr],
        true,
        [false; 2],
        ramp,
    )
    .expect("a send record");
    let writes = [mute];
    let reference = one_swap(&a, &b2, None, &feeds, &writes);
    assert_ne!(
        reference.blocks[BLOCKS - 1],
        one_swap(&a, &b2, None, &feeds, &[]).blocks[BLOCKS - 1],
        "{backend:?}: the live mute is heard"
    );
    for (case, second, prepared_at) in [
        ("(b) scheduled", SWAP_BLOCK + 2, None),
        ("(c) fired", fire_block + 1, None),
        (
            "(c) fired after its successor's preparation",
            fire_block + 1,
            Some(SWAP_BLOCK + 2),
        ),
    ] {
        let swaps = [
            FadeSwap {
                block: SWAP_BLOCK,
                to: &b1,
                committed: &a.model,
                prepared_at: None,
            },
            FadeSwap {
                block: second,
                to: &b2,
                committed: &b1.model,
                prepared_at,
            },
        ];
        let run = fade_run(&a, &swaps, &feeds, None, &writes, backend);
        assert_eq!(
            first_fade_difference(&run, &reference),
            None,
            "{backend:?} {case}: a second swap at block {second} moves no bit"
        );
        assert_eq!(
            run.allocator,
            (0, 0, 0),
            "{backend:?} {case}: allocator calls"
        );
    }
    // #2069: a waiting send the second transaction mutes is re-armed too, carrying its entry, as
    // any value change: the second plan holds the same arms as with the send unmuted.
    let mut muted_send = b2.model.clone();
    route_mut(&mut muted_send, "fade-bus").mute = true;
    let muted_send = Session::compile(muted_send);
    let second = SWAP_BLOCK + 2;
    assert_eq!(
        two_swap_run(&a, &b1, &muted_send, second, None, &feeds, backend).arms[1],
        two_swap_run(&a, &b1, &b2, second, None, &feeds, backend).arms[1],
        "{backend:?}: a waiting send the second transaction mutes is re-armed"
    );
    // (e) The second transaction restarts the track (#1324): a transparent insert. Its pending
    // entries are replaced by fresh arms, the send's with the fader's (#1392 D2), so both wait out
    // the restarted chain's line. The fresh-arm run: the same swaps, the send added by the second
    // transaction, so it is armed only afresh. The restart lands at block 8, before the fire at
    // block 10, its successor prepared then or one block earlier. (A restart after the fire is not
    // observable in this run: the restarted limiter's line holds the strip silent past its end.)
    let restarted = |model| {
        let mut model = send(model);
        track_mut(&mut model, FADE_TRACK)
            .inserts
            .effects
            .push(transparent_insert());
        model
    };
    let track = with_fade_track(route_fade_base_limited(false), AUX_SOURCE, true, false);
    let without_send = Session::compile(track.clone());
    let b2 = Session::compile(restarted(track));
    let restart = |b1: &Session, second, prepared_at| {
        let run = two_swap_run_prepared(&a, b1, &b2, (second, prepared_at), None, &feeds, backend);
        assert!(
            run.restarted[1].iter().any(|id| &**id == FADE_TRACK),
            "the insert restarts the strip"
        );
        run
    };
    for prepared_at in [None, Some(SWAP_BLOCK + 1)] {
        assert_eq!(
            first_fade_difference(
                &restart(&b1, SWAP_BLOCK + 2, prepared_at),
                &restart(&without_send, SWAP_BLOCK + 2, prepared_at),
            ),
            None,
            "{backend:?} (e) restart before the fire, prepared at {prepared_at:?}: the send \
             takes the restarted strip's fresh arm"
        );
    }
    // (d) A transition route.
    let aux = Feed::new(AUX_SOURCE, 2);
    let feeds = [&kept, &aux];
    let base = route_fade_base_limited(false);
    let a = Session::compile(base.clone());
    let b1 = Session::compile(with_route(
        base,
        "aux-out",
        "aux",
        session::SendTap::PostFader,
        into_output(),
        false,
    ));
    let b2 = Session::compile(with_muted_track(b1.model.clone()));
    let reference = one_swap(&a, &b2, None, &feeds, &[]);
    // #1953b: also prepared after the fire (its entry fired, not settled) and adopted mid-ramp.
    for second in [
        (SWAP_BLOCK + 1, Some(SWAP_BLOCK)),
        (SWAP_BLOCK + 2, Some(SWAP_BLOCK + 1)),
    ] {
        let run = two_swap_run_prepared(&a, &b1, &b2, second, None, &feeds, backend);
        assert_eq!(
            first_fade_difference(&run, &reference),
            None,
            "{backend:?} (d) {second:?}: a transition route's ramp continues across a second swap"
        );
        assert_eq!(run.allocator, (0, 0, 0), "{backend:?} (d): allocator calls");
    }
}

/// Issue #1877 gate 4 on every width. Red if the successor does not re-arm the send from the
/// inventory (it fires at the second swap and the bus hears the source enter at a step), re-arms a
/// `FireAt` send as `Waiting` (it fires `D` late), fades a fired send again, carries a restarted
/// strip's send entry over its fresh arm (it fires before the restarted line refills), folds a
/// fired transition route whose ramp is still in flight (it steps to its constant, #1953b), or
/// allocates at either swap block; or (#2069) if a waiting send the second transaction mutes is
/// not re-armed (its displaced arm fires at the swap, and an unmute before its fire opens it at
/// once).
#[test]
fn a_pending_route_fade_survives_a_second_swap_on_every_width() {
    for backend in backends() {
        a_pending_route_fade_survives_a_second_swap(backend);
    }
}

/// Issue #1953b gates (b1, b2) at one width. A holds `aux-out`, a `post_fader` route from the
/// playing track `aux` into the output, which has no lane (decision 14's fold); B removes it. The successor
/// compiles a fading copy of it that fades out from the swap block (`D = 0`, the session mute
/// ramp), its destination line (486 samples when `late` is limited) carried from the removed
/// route's. Every block equals A with the route from `pre_fader` (a unity fader moves no bit,
/// and that route has a lane), rendered fresh from frame 0, given a live mute at the swap block;
/// neither swap block allocates or frees. Inner claims:
///
/// * (b) a second swap prepared after the fire and adopted mid-ramp keeps the copy, carrying its
///   ramp and its fade entry, also when it adds a user route under the copy's ID with the copy's
///   endpoints: the copy is renamed and the user route fades in on its own;
/// * (c) a second swap at block 11 drops the copy once it has settled (without the line: its ramp
///   ended at 1248) and keeps it while its line still delivers the fade (with the 486-sample line,
///   it settles at 1734): either way every block equals the reference;
/// * (d) a re-point into the bus ramps both halves (D15-9: a removal plus an addition).
fn a_removed_or_repointed_lane_less_route_fades_out_through_its_line(backend: Backend) {
    let (kept, aux) = (Feed::new(SOURCE, 1), Feed::new(AUX_SOURCE, 2));
    let feeds = [&kept, &aux];
    for limited in [true, false] {
        let base = route_fade_base_limited(limited);
        let ramp = LiveRamps::for_session(&base).mute_samples;
        let with = |model: &SessionModel, tap| {
            with_route(model.clone(), "aux-out", "aux", tap, into_output(), false)
        };
        let a = Session::compile(with(&base, session::SendTap::PostFader));
        let b = Session::compile(base.clone());
        let b2 = Session::compile(with_muted_track(base.clone()));
        let reference = |model: &SessionModel, block| {
            let model = with(model, session::SendTap::PreFader);
            let mute = route_mute_write(&model, "aux-out", block, ramp, true);
            fade_run(
                &Session::compile(model),
                &[],
                &feeds,
                None,
                &[mute],
                backend,
            )
        };
        let expected = reference(&base, SWAP_BLOCK);
        let swap = |block, to, committed, prepared_at| FadeSwap {
            block,
            to,
            committed,
            prepared_at,
        };
        let case = |limited| if limited { "with its line" } else { "no line" };
        let run = fade_run(
            &a,
            &[swap(SWAP_BLOCK, &b, &a.model, None)],
            &feeds,
            None,
            &[],
            backend,
        );
        assert_eq!(
            run.arms[0],
            vec![(0, ramp)],
            "{backend:?} {}: the copy's D and ramp",
            case(limited)
        );
        assert_eq!(
            first_fade_difference(&run, &expected),
            None,
            "{backend:?} {}: every block equals the live mute at the swap",
            case(limited)
        );
        assert_eq!(run.allocator, (0, 0, 0), "{backend:?}: allocator calls");
        // The oracle can fail: the mute one block late is different bits.
        assert_ne!(
            reference(&base, SWAP_BLOCK + 1).blocks,
            expected.blocks,
            "{backend:?}: the fire block is observable"
        );
        let muted_expected = reference(&with_muted_track(base.clone()), SWAP_BLOCK);
        for (claim, second, prepared_at, copies) in [
            ("(b) mid-ramp", SWAP_BLOCK + 2, Some(SWAP_BLOCK + 1), 1),
            ("(c) at block 11", BLOCKS - 1, None, usize::from(limited)),
        ] {
            let run = fade_run(
                &a,
                &[
                    swap(SWAP_BLOCK, &b, &a.model, None),
                    swap(second, &b2, &b.model, prepared_at),
                ],
                &feeds,
                None,
                &[],
                backend,
            );
            // #2080: `b2`'s added muted track arms (`(0, ramp)`, first), then each kept copy.
            assert_eq!(
                run.arms[1],
                vec![(0, ramp); 1 + copies],
                "{backend:?} {} {claim}: the added muted track's arm and the copies the second \
                 successor keeps",
                case(limited)
            );
            assert_eq!(
                first_fade_difference(&run, &muted_expected),
                None,
                "{backend:?} {} {claim}: every block equals the live mute at the first swap",
                case(limited)
            );
            assert_eq!(
                run.allocator,
                (0, 0, 0),
                "{backend:?} {claim}: allocator calls"
            );
        }
        // (b) The second transaction, prepared after the fire and adopted mid-ramp, adds a user
        // route `aux-out-` (the copy's ID) with the copy's endpoints. The copy stays, renamed
        // (`aux-out--`); the user route is an added route that fades in from an empty line, with
        // neither the copy's line nor its op. Every block equals A with the old route under the
        // renamed copy's ID and the user route, both from `pre_fader`, given a live mute of the
        // old at the first swap and a live unmute of the user route (muted) at the second.
        let user = |model: &SessionModel, tap, muted| {
            with_route(model.clone(), "aux-out-", "aux", tap, into_output(), muted)
        };
        let b3 = Session::compile(user(&base, session::SendTap::PostFader, false));
        let run = fade_run(
            &a,
            &[
                swap(SWAP_BLOCK, &b, &a.model, None),
                swap(SWAP_BLOCK + 2, &b3, &b.model, Some(SWAP_BLOCK + 1)),
            ],
            &feeds,
            None,
            &[],
            backend,
        );
        assert_eq!(
            run.arms[1],
            vec![(0, ramp), (0, ramp)],
            "{backend:?} {} (b): the user route's fade-in, then the kept copy's",
            case(limited)
        );
        let named = with_route(
            user(&base, session::SendTap::PreFader, true),
            "aux-out--",
            "aux",
            session::SendTap::PreFader,
            into_output(),
            false,
        );
        let writes = [
            route_mute_write(&named, "aux-out--", SWAP_BLOCK, ramp, true),
            route_mute_write(&named, "aux-out-", SWAP_BLOCK + 2, ramp, false),
        ];
        let reference = fade_run(
            &Session::compile(named),
            &[],
            &feeds,
            None,
            &writes,
            backend,
        );
        assert_eq!(
            first_fade_difference(&run, &reference),
            None,
            "{backend:?} {} (b): a user route under the copy's ID fades in on its own",
            case(limited)
        );
        assert_eq!(run.allocator, (0, 0, 0), "{backend:?} (b): allocator calls");
        // (d) B re-points `aux-out` into the bus, where it has a lane: both halves ramp. The
        // successor arms the re-pointed route (`D = 0`: the bus line sits after its op) and the
        // old route's copy (`aux-out-`), and every block equals A with the old route under the
        // copy's ID from `pre_fader` and the new one muted, given a live mute of the old and a
        // live unmute of the new at the swap block.
        let mut repointed = with(&base, session::SendTap::PostFader);
        route_mut(&mut repointed, "aux-out").destination = into_bus();
        let repointed = Session::compile(repointed);
        let run = fade_run(
            &a,
            &[swap(SWAP_BLOCK, &repointed, &a.model, None)],
            &feeds,
            None,
            &[],
            backend,
        );
        assert_eq!(
            run.arms[0],
            vec![(0, ramp), (0, ramp)],
            "{backend:?} {} (d): the re-pointed route's arm, then its copy's",
            case(limited)
        );
        let old = with_route(
            base.clone(),
            "aux-out-",
            "aux",
            session::SendTap::PreFader,
            into_output(),
            false,
        );
        let both = with_route(
            old,
            "aux-out",
            "aux",
            session::SendTap::PostFader,
            into_bus(),
            true,
        );
        let writes = [
            route_mute_write(&both, "aux-out-", SWAP_BLOCK, ramp, true),
            route_mute_write(&both, "aux-out", SWAP_BLOCK, ramp, false),
        ];
        let reference = fade_run(&Session::compile(both), &[], &feeds, None, &writes, backend);
        assert_eq!(
            first_fade_difference(&run, &reference),
            None,
            "{backend:?} {} (d): the old route fades out and the new one in at the swap",
            case(limited)
        );
        assert_eq!(run.allocator, (0, 0, 0), "{backend:?} (d): allocator calls");
    }
}

/// Issue #1953b gates (b1, b2) on every width. Red if a removed lane-less route still steps out at
/// the swap, if its copy does not take the removed route's line (the 486 samples the line held are
/// lost), if a later swap restarts the copy's ramp or drops it before its line has delivered the
/// fade, if a settled copy is never dropped, or if a re-point from a lane-less route steps on
/// either side (its old half not copied, or its new half not armed).
#[test]
fn a_removed_or_repointed_lane_less_route_fades_out_through_its_line_on_every_width() {
    for backend in backends() {
        a_removed_or_repointed_lane_less_route_fades_out_through_its_line(backend);
    }
}

/// Issue #2077 gates at one width. A holds `aux-out`, a `post_fader` route from the playing track
/// `aux` into the output, which has no lane; B keeps it (same ID, source and destination) at other
/// values. The successor crossfades at the swap block (R1: the swap is the route's `S`): a fading
/// copy `aux-out-` at the old values fades out through the predecessor's line, and the kept route
/// fades in at the new values from a cold line (both `D = 0`, the session mute ramp).
///
/// * (a) a gain change, with and without the 486-sample line: every block equals A with both
///   routes from `pre_fader` (a unity fader moves no bit, and those routes have lanes), `aux-out`
///   at the new values muted and `aux-out-` at the old ones, given a live unmute of the first and
///   a live mute of the second at the swap block;
/// * (b) R3, a flip of `mute` alone: an unmute arms the kept route only and equals the live
///   unmute at the swap; a mute makes the copy only and equals the live mute at the swap (the
///   removal reference of #1953b);
/// * (c) a lane-less route of an added track still waiting for its anchored source, changed by a
///   second rebuild before the track's fire, takes no copy and no arm (the second successor arms
///   only the track's carried fade): the run equals the one that added it at its new values.
///   "unmuted" defends the `(!armed_source || tap.precedes_fader())` guard on the `retargeted`
///   path of `added_route_fades`; "another gain" defends R2's copy skip;
/// * (d) P2 is prepared before P1's first block and adopted after it: P1 adds `aux-out` at the
///   swap block, and P2, prepared while P1's arm is unfired, sets it to another gain at the next
///   block. P2 both copies the route (aliasing P1's op mid-ramp) and re-arms it at its new values,
///   carrying P1's entry: every block equals both routes from `pre_fader`, muted, given a live
///   unmute of the old at the swap block, then its mute and the new one's unmute at the next. The
///   sub-case where P1 is replaced before it renders (its copy fades out silence) has no row:
///   `fade_run` cannot adopt two plans in one block.
///
/// Neither swap block allocates or frees.
fn a_lane_less_route_value_change_crossfades_at_the_swap(backend: Backend) {
    let (kept, aux) = (Feed::new(SOURCE, 1), Feed::new(AUX_SOURCE, 2));
    let feeds = [&kept, &aux];
    let post = session::SendTap::PostFader;
    let pre = session::SendTap::PreFader;
    for limited in [true, false] {
        let base = route_fade_base_limited(limited);
        let ramp = LiveRamps::for_session(&base).mute_samples;
        let line = if limited { "with its line" } else { "no line" };
        let route = |model: &SessionModel, id, tap, muted, gain: Option<f32>| {
            let mut model = with_route(model.clone(), id, "aux", tap, into_output(), muted);
            if let Some(gain) = gain {
                route_mut(&mut model, id).gain_db = gain;
            }
            model
        };
        let swap_to = |a: &Session, b: &Session| {
            fade_run(
                a,
                &[FadeSwap {
                    block: SWAP_BLOCK,
                    to: b,
                    committed: &a.model,
                    prepared_at: None,
                }],
                &feeds,
                None,
                &[],
                backend,
            )
        };
        let reference = |model: SessionModel,
                         writes: &dyn Fn(&SessionModel, usize) -> Vec<CellWrite>,
                         block| {
            let writes = writes(&model, block);
            fade_run(
                &Session::compile(model),
                &[],
                &feeds,
                None,
                &writes,
                backend,
            )
        };
        // (a) A gain change.
        let a = Session::compile(route(&base, "aux-out", post, false, None));
        let b = Session::compile(route(&base, "aux-out", post, false, Some(-9.0)));
        let run = swap_to(&a, &b);
        assert_eq!(
            run.arms[0],
            vec![(0, ramp), (0, ramp)],
            "{backend:?} (a) {line}: the kept route's fade-in, then the copy's fade-out"
        );
        assert_eq!(run.allocator, (0, 0, 0), "{backend:?} (a): allocator calls");
        let both = route(
            &route(&base, "aux-out", pre, true, Some(-9.0)),
            "aux-out-",
            pre,
            false,
            None,
        );
        let crossfade = |model: &SessionModel, block| {
            vec![
                route_mute_write(model, "aux-out", block, ramp, false),
                route_mute_write(model, "aux-out-", block, ramp, true),
            ]
        };
        let expected = reference(both.clone(), &crossfade, SWAP_BLOCK);
        assert_eq!(
            first_fade_difference(&run, &expected),
            None,
            "{backend:?} (a) {line}: every block equals the live crossfade at the swap"
        );
        // The oracle can fail: the crossfade one block late is different bits.
        assert_ne!(
            reference(both, &crossfade, SWAP_BLOCK + 1).blocks,
            expected.blocks,
            "{backend:?} (a) {line}: the crossfade block is observable"
        );
        // (d) P2 is prepared before P1's first block and adopted after it.
        let p0 = Session::compile(base.clone());
        let p1 = Session::compile(route(&base, "aux-out", post, false, None));
        let p2 = Session::compile(route(&base, "aux-out", post, false, Some(-9.0)));
        let run = fade_run(
            &p0,
            &[
                FadeSwap {
                    block: SWAP_BLOCK,
                    to: &p1,
                    committed: &p0.model,
                    prepared_at: None,
                },
                FadeSwap {
                    block: SWAP_BLOCK + 1,
                    to: &p2,
                    committed: &p1.model,
                    prepared_at: Some(SWAP_BLOCK),
                },
            ],
            &feeds,
            None,
            &[],
            backend,
        );
        assert_eq!(
            run.arms[1],
            vec![(0, ramp), (0, ramp)],
            "{backend:?} (d) {line}: the carried re-arm at the new values, then the copy"
        );
        assert_eq!(run.allocator, (0, 0, 0), "{backend:?} (d): allocator calls");
        let muted_both = route(
            &route(&base, "aux-out", pre, true, Some(-9.0)),
            "aux-out-",
            pre,
            true,
            None,
        );
        let replaced = |model: &SessionModel, block| {
            vec![
                route_mute_write(model, "aux-out-", block, ramp, false),
                route_mute_write(model, "aux-out-", block + 1, ramp, true),
                route_mute_write(model, "aux-out", block + 1, ramp, false),
            ]
        };
        let expected = reference(muted_both, &replaced, SWAP_BLOCK);
        assert_eq!(
            first_fade_difference(&run, &expected),
            None,
            "{backend:?} (d) {line}: the old route fades out mid-ramp as the new one fades in"
        );
        // (b) R3: the rebuild flips `mute` alone.
        for (claim, before, after) in [("unmute", true, false), ("mute", false, true)] {
            let a = Session::compile(route(&base, "aux-out", post, before, None));
            let b = Session::compile(route(&base, "aux-out", post, after, None));
            let run = swap_to(&a, &b);
            assert_eq!(
                run.arms[0],
                vec![(0, ramp)],
                "{backend:?} (b) {claim} {line}: one arm (the kept route's, or the copy's)"
            );
            assert_eq!(run.allocator, (0, 0, 0), "{backend:?} (b): allocator calls");
            let flip = |model: &SessionModel, block| {
                vec![route_mute_write(model, "aux-out", block, ramp, after)]
            };
            let expected = reference(
                route(&base, "aux-out", pre, before, None),
                &flip,
                SWAP_BLOCK,
            );
            assert_eq!(
                first_fade_difference(&run, &expected),
                None,
                "{backend:?} (b) {claim} {line}: every block equals the live {claim} at the swap"
            );
        }
    }
    // (c) R2: the source strip waits for its anchored source (block 9); the second rebuild is
    // adopted at block 7, before its fire.
    let ramp = LiveRamps::for_session(&unlimited_fade_base()).mute_samples;
    let (added, _) = anchored_feeds();
    let anchored = || Anchored {
        feed: &added,
        anchor_block: FADE_ANCHOR_BLOCK,
    };
    let a = Session::compile(unlimited_fade_base());
    let with_out = |muted, gain: Option<f32>| {
        let mut model = with_route(
            anchored_fade_model(false),
            "fade-out",
            FADE_TRACK,
            post,
            into_output(),
            muted,
        );
        if let Some(gain) = gain {
            route_mut(&mut model, "fade-out").gain_db = gain;
        }
        Session::compile(model)
    };
    let all = [&kept, &aux];
    for (claim, b1, b2) in [
        ("unmuted", with_out(true, None), with_out(false, None)),
        (
            "another gain",
            with_out(false, None),
            with_out(false, Some(-9.0)),
        ),
    ] {
        let run = fade_run(
            &a,
            &[
                FadeSwap {
                    block: SWAP_BLOCK,
                    to: &b1,
                    committed: &a.model,
                    prepared_at: None,
                },
                FadeSwap {
                    block: SWAP_BLOCK + 1,
                    to: &b2,
                    committed: &b1.model,
                    prepared_at: None,
                },
            ],
            &all,
            Some(anchored()),
            &[],
            backend,
        );
        assert_eq!(
            run.arms[1],
            vec![(0, ramp)],
            "{backend:?} (c) {claim}: the second successor arms only the track's carried fade"
        );
        assert_eq!(run.allocator, (0, 0, 0), "{backend:?} (c): allocator calls");
        let expected = fade_run(
            &a,
            &[FadeSwap {
                block: SWAP_BLOCK,
                to: &b2,
                committed: &a.model,
                prepared_at: None,
            }],
            &all,
            Some(anchored()),
            &[],
            backend,
        );
        assert_eq!(
            first_fade_difference(&run, &expected),
            None,
            "{backend:?} (c) {claim}: every block equals the run that added the route at its new \
             values"
        );
    }
}

/// Issue #2077 gates on every width. Red if a kept lane-less route steps to its new values at the
/// swap (no copy, or no fade-in), crossfades late or misaligned with its line (the copy does not
/// take the old line, or the kept route keeps it), steps on a flip of `mute` alone, or (R2)
/// crossfades a route whose waiting source strip's fader still owns it.
#[test]
fn a_lane_less_route_value_change_crossfades_at_the_swap_on_every_width() {
    for backend in backends() {
        a_lane_less_route_value_change_crossfades_at_the_swap(backend);
    }
}

/// Issue #1392 gate 3, as #1324a reads it (D2, D3): the second transaction adds a transparent
/// insert to the added track while its fade waits for the anchor. The strip restarts for its chain
/// (`restarted_strips()` lists it), so its fader lane does not carry and its pending entry is
/// replaced by a fresh arm, still `Waiting` for the anchored source: the fade still fires at the
/// anchor, and the run equals the post-edit session with the track muted and unmuted live there.
/// Red if a swap that restarts the strip's fader drops its Waiting arm (the track enters at full
/// gain at the swap), or carries the fader beside a fresh arm.
#[test]
fn a_chain_restarted_strip_still_fades() {
    let backend = Backend::current();
    let ramp = LiveRamps::for_session(&fade_base()).mute_samples;
    let (kept, aux) = fade_feeds();
    let (added, reference_added) = anchored_feeds();
    let with_insert = |muted: bool| {
        let mut model = anchored_fade_model(muted);
        track_mut(&mut model, FADE_TRACK)
            .inserts
            .effects
            .push(transparent_insert());
        model
    };
    let a = Session::compile(unlimited_fade_base());
    let b1 = Session::compile(anchored_fade_model(false));
    let b2 = Session::compile(with_insert(false));
    let muted = Session::compile(with_insert(true));
    let anchored = Anchored {
        feed: &added,
        anchor_block: FADE_ANCHOR_BLOCK,
    };
    let run = two_swap_run(
        &a,
        &b1,
        &b2,
        SWAP_BLOCK + 1,
        Some(anchored),
        &[&kept, &aux],
        backend,
    );
    assert!(
        run.restarted[1].iter().any(|id| &**id == FADE_TRACK),
        "the insert restarts the strip"
    );
    // Every strip's fader and matrix stage move, but the restarted strip's fader (#1324 D2).
    assert_eq!(
        run.carries[1][2],
        2 * b2.model.strips().count() - 1,
        "the restarted strip's fader lane does not carry"
    );
    let reference = fade_run(
        &muted,
        &[],
        &[&kept, &aux, &reference_added],
        None,
        &[unmute_fade(FADE_ANCHOR_BLOCK, ramp)],
        backend,
    );
    assert_eq!(
        first_fade_difference(&run, &reference),
        None,
        "the restarted strip still fades in at the anchor"
    );
    assert_audible(
        &muted,
        &[&kept, &aux, &reference_added],
        &reference,
        backend,
    );
}

/// #1324a gate 4's session: the #1363 base, with `aux` holding a true-peak limiter and then a
/// transparent compressor `second`, and a route `aux-in` from `aux`'s input tap into the bus, whose
/// input arrives 486 samples late, so the route's line is 486 samples long; the untouched `eq0`
/// holds a transparent compressor insert.
fn whole_restart_model(second: &str) -> SessionModel {
    let mut model = route_fade_base_limited(true);
    add_limiter(&mut model, "aux", "lim");
    let mut insert = transparent_insert();
    insert.id = id(second);
    track_mut(&mut model, "aux").inserts.effects.push(insert);
    track_mut(&mut model, "eq0")
        .inserts
        .effects
        .push(transparent_insert());
    with_route(
        model,
        "aux-in",
        "aux",
        session::SendTap::Input,
        into_bus(),
        false,
    )
}

/// #1324a gate 4 (D2, D3) at one width, the successor prepared between two render calls: B
/// replaces `aux`'s second insert. `aux` is restarted, and none of its owners before its fader
/// carries: its input section, its fader, both inserts (the surviving limiter too) and the line of
/// its route from the input tap; its matrix carries. It is armed with `D` its limiter's 486
/// samples, and its `input`-tap route with it at `D = 0`. Every other strip carries every owner:
/// against a successor that changes nothing, the tables hold exactly `aux`'s entries fewer.
fn a_restarted_strip_carries_nothing_before_its_fader(backend: Backend) {
    let a = Session::compile(whole_restart_model("tc"));
    let b = Session::compile(whole_restart_model("tc2"));
    let ramp = LiveRamps::for_session(&a.model).mute_samples;
    let mut plan = CellPlan::fresh(&a, backend);
    let mut output = vec![f32::NAN; QUANTUM * 2];
    for _ in 0..SWAP_BLOCK {
        plan.live.render_into(&mut output, QUANTUM);
    }
    let mut unchanged = CellPlan::successor(
        &a,
        &plan.live.prepared.inventory,
        &a.model,
        &EditRamps::new(),
        backend,
    );
    let mut replaced = CellPlan::successor(
        &b,
        &plan.live.prepared.inventory,
        &a.model,
        &EditRamps::new(),
        backend,
    );
    let what = format!("{backend:?}");
    assert!(
        unchanged.live.prepared.restarted_strips().is_empty(),
        "{what}"
    );
    assert_eq!(
        replaced.live.prepared.restarted_strips(),
        [Box::<str>::from("aux")],
        "{what}"
    );
    let full = graph::test_only_carry_counts(&mut unchanged.live.prepared.plan);
    let counts = graph::test_only_carry_counts(&mut replaced.live.prepared.plan);
    assert_eq!(
        [0, 1, 2, 3, 4].map(|table| full[table] - counts[table]),
        [0, 1, 1, 2, 1],
        "{what}: aux's input section, fader, two inserts and input-tap line do not move ({full:?})"
    );
    assert_eq!(
        graph::test_only_fade_arms(&mut unchanged.live.prepared.plan).expect("a graph plan"),
        []
    );
    assert_eq!(
        graph::test_only_fade_arms(&mut replaced.live.prepared.plan).expect("a graph plan"),
        [(486, ramp), (0, ramp)],
        "{what}: aux's fader waits for its limiter, its input-tap route for nothing"
    );
    assert_eq!(
        replaced
            .live
            .prepared
            .plan
            .adopt_predecessor_plan(&mut plan.live.prepared.plan),
        CarryOutcome::Carried,
        "{what}"
    );
}

/// #1324a gate 4 on every width. Red if a restarted strip carries an owner before its fader (a
/// surviving latent insert, its input section, a pre-fader line: the hole in front of the
/// restarted owner, D2) or its fader, if it is not armed, or if an untouched strip loses a carry.
#[test]
fn a_restarted_strip_carries_nothing_before_its_fader_on_every_width() {
    for backend in backends() {
        a_restarted_strip_carries_nothing_before_its_fader(backend);
    }
}

/// How [`follow_send_model`] builds `aux`: behind its 486-sample limiter or not, restarted by a
/// transparent insert (a duck-swap, #1324) or not, and with its send into the bus or not.
#[derive(Clone, Copy)]
struct FollowSend {
    limited: bool,
    restarted: bool,
    sent: bool,
}

/// #2011's session: the #1363 base without `late`'s limiter, `aux` (behind a 486-sample true-peak
/// limiter when `limited`) with its own route to the output muted (so the bus carries all of
/// `aux` the output hears), and, when `sent`, a `pre_fader` send `aux-bus` from `aux` into the
/// bus that follows `aux`'s mute. With `curve = Some((end, from, to))`, `aux`'s mute is a stored
/// automation curve, one step segment that holds `from` from sample 0 and steps to `to` at
/// `end`: the send then binds a following route. `restarted` adds a transparent insert to `aux`.
fn follow_send_model(curve: Option<(u64, f32, f32)>, edit: FollowSend) -> SessionModel {
    let mut model = route_fade_base_limited(false);
    if edit.limited {
        add_limiter(&mut model, "aux", "lim");
    }
    if edit.restarted {
        track_mut(&mut model, "aux")
            .inserts
            .effects
            .push(transparent_insert());
    }
    if edit.sent {
        model = with_route(
            model,
            "aux-bus",
            "aux",
            session::SendTap::PreFader,
            into_bus(),
            false,
        );
    }
    for route in &mut model.routes {
        match route.id.as_str() {
            "aux-bus" => route.follows_mute = true,
            "aux-main" => route.mute = true,
            _ => {}
        }
    }
    let Some((end, from, to)) = curve else {
        return model;
    };
    let mut model = with_mute_automation(model, "aux", None);
    let segment = &mut model.automation.last_mut().expect("the curve").segments[0];
    segment.end_sample = end;
    segment.start_value = from;
    segment.end_value = to;
    model
}

/// Issue #2011 at one width. A duck-swap at block 6 restarts `aux` (a transparent insert), and its
/// `pre_fader` send into the bus, which follows `aux`'s automated mute, is armed with it: the fire
/// waits for the limiter's 486 samples and lands at block 10. `aux` is fed exact zeros before the
/// swap, so a fresh plan of the successor has the restarted chain's state. Arms:
///
/// * (a) the curve open throughout, and the curve muting `aux` until sample 500, which the send
///   hears inside the wait (its tap's node time runs 486 samples behind the block): every block
///   equals the same swap with no curve (the six-word send, whose fire ramps from zero to its
///   prepared target): the following send waits silent and fires to the heard, open target over
///   the same ramp.
/// * (b) the curve muting `aux` until sample 900, which the send hears after the fire: every
///   block equals a plan booted on the successor model with the same curve, in which the send
///   opens at the curve's own event (the oracle: that opening is heard).
/// * (c) the hand-over and every block from the swap on allocate and free nothing.
/// * (d) with no limiter, the swap adds the send while the curve, which muted `aux` from sample
///   0, is open: the send fires on its first block, and every block equals the same swap with no
///   curve: the fire composes with the curve its first block hears, not its seed, and does not
///   step.
fn a_following_send_of_a_restarted_strip_fades_in(backend: Backend) {
    let restart = |restarted| FollowSend {
        limited: true,
        restarted,
        sent: true,
    };
    let kept = Feed::new(SOURCE, 1);
    let aux = spliced(AUX_SOURCE, &Feed::new(AUX_SOURCE, 2), |block| {
        (block >= SWAP_BLOCK).then_some(block)
    });
    let feeds = [&kept, &aux];
    let swap_run = |curve| {
        let a = Session::compile(follow_send_model(curve, restart(false)));
        let b = Session::compile(follow_send_model(curve, restart(true)));
        let swap = [FadeSwap {
            block: SWAP_BLOCK,
            to: &b,
            committed: &a.model,
            prepared_at: None,
        }];
        let run = fade_run(&a, &swap, &feeds, None, &[], backend);
        assert_eq!(run.restarted[0], [Box::<str>::from("aux")], "{backend:?}");
        assert_eq!(
            run.allocator,
            (0, 0, 0),
            "{backend:?} {curve:?}: (c) allocator calls"
        );
        run
    };
    let six_word = swap_run(None);
    let ramp = LiveRamps::for_session(&follow_send_model(None, restart(false))).mute_samples;
    assert_eq!(
        six_word.arms[0],
        [(486, ramp), (486, ramp)],
        "{backend:?}: aux's fader and its send wait for the limiter"
    );
    // #2069: a send the session mutes is armed with its restarted strip too.
    let muted_send = |restarted| {
        let mut model = follow_send_model(None, restart(restarted));
        route_mut(&mut model, "aux-bus").mute = true;
        Session::compile(model)
    };
    let (a, b) = (muted_send(false), muted_send(true));
    let swap = [FadeSwap {
        block: SWAP_BLOCK,
        to: &b,
        committed: &a.model,
        prepared_at: None,
    }];
    assert_eq!(
        fade_run(&a, &swap, &feeds, None, &[], backend).arms[0],
        six_word.arms[0],
        "{backend:?}: a muted send of a restarted strip is armed"
    );
    // A curve that mutes `aux` at preparation arms its fader too (#2080: its fire goes to the
    // curve's mute unless the curve opens it first), and arms its send (R2: the arm refuses on
    // the route's own gate only).
    for (case, curve) in [
        ("open throughout", (1, 0.0, 0.0)),
        ("opening inside the wait", (500, 1.0, 0.0)),
    ] {
        let run = swap_run(Some(curve));
        assert_eq!(
            run.arms[0], six_word.arms[0],
            "{backend:?} (a) {case}: the fader and the send are armed"
        );
        assert_eq!(
            first_fade_difference(&run, &six_word),
            None,
            "{backend:?} (a) {case}: the following send fades in as the six-word one"
        );
    }
    let curve = Some((900, 1.0, 0.0));
    let run = swap_run(curve);
    let booted = |curve| {
        let b = Session::compile(follow_send_model(curve, restart(true)));
        fade_run(&b, &[], &feeds, None, &[], backend)
    };
    let reference = booted(curve);
    assert_eq!(
        first_fade_difference(&run, &reference),
        None,
        "{backend:?} (b): the send stays silent through the fire and opens at the curve's event"
    );
    assert_ne!(
        reference.blocks,
        booted(Some((900, 1.0, 1.0))).blocks,
        "{backend:?} (b): the curve's opening is heard"
    );
    let feeds = [&kept, &Feed::new(AUX_SOURCE, 2)];
    let added_run = |curve| {
        let edit = |sent| FollowSend {
            limited: false,
            restarted: false,
            sent,
        };
        let a = Session::compile(follow_send_model(curve, edit(false)));
        let b = Session::compile(follow_send_model(curve, edit(true)));
        let swap = [FadeSwap {
            block: SWAP_BLOCK,
            to: &b,
            committed: &a.model,
            prepared_at: None,
        }];
        let run = fade_run(&a, &swap, &feeds, None, &[], backend);
        assert_eq!(run.arms[0], [(0, ramp)], "{backend:?} (d) {curve:?}: armed");
        assert_eq!(
            run.allocator,
            (0, 0, 0),
            "{backend:?} (d) {curve:?}: (c) allocator calls"
        );
        run
    };
    assert_eq!(
        first_fade_difference(&added_run(Some((300, 1.0, 0.0))), &added_run(None)),
        None,
        "{backend:?} (d): an added following send fires from rest to the curve it hears"
    );
}

/// Issue #2011 on every width. Red if a following send of a restarted strip is not armed (it
/// enters the bus at its prepared value while the six-word send fades), if its fire ignores the
/// arm's `D` or ramp, if the fire takes the prepared target instead of the curve heard (a step
/// while the curve says silent), if the arm refuses on the curve's current silence or a curve
/// event during the wait lifts its ramps off zero (it opens before the fire), or if the arm or
/// the fire allocates on render; or (d) if an added following send's first-block fire composes
/// with its seed instead of the curve its block hears (the curve's first-block `Set` then steps it
/// open); or (#2069) if a send the session mutes is left unarmed by its strip's restart (an
/// unmute before the fire opens it at once). Render-scope allocations are counted, not aborted, so
/// (c) reads them.
#[test]
fn a_following_send_of_a_restarted_strip_fades_in_on_every_width() {
    use bench_support::alloc::{Mode, assert_installed, mode, set_mode};
    /// Restores the allocator's violation mode on exit.
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
    for backend in backends() {
        a_following_send_of_a_restarted_strip_fades_in(backend);
    }
}

/// A compressor `name` whose `sidechain-in` port reads `tap` of strip `source`.
fn keyed_insert(name: &str, source: RouteSource) -> Effect {
    Effect {
        id: id(name),
        sidechain: SidechainDeclaration::Routed(session::Sidechain {
            source,
            port_id: id("sidechain-in"),
        }),
        ..transparent_insert()
    }
}

fn track_key(track: &str, tap: session::SendTap) -> RouteSource {
    RouteSource::Track {
        track_id: id(track),
        tap,
    }
}

/// How a row of [`a_strip_keyed_from_a_restarted_strip_restarts_with_it_on_every_width`] edits A.
#[derive(Clone, Copy, PartialEq)]
enum Restart {
    /// An insert added to `eq0`.
    Insert,
    /// `eq0`'s `delay_samples` changed (its delay line restarts).
    Delay,
    /// An insert added to the bus.
    Bus,
    /// An insert added to the bus, and a latent insert to the muted `late` that feeds it, so the
    /// bus input, which carried `aux` also feeds, arrives at `eq1`'s limited key: the key's line
    /// shrinks from the limiter's latency to 0.
    BusLater,
    /// [`Self::BusLater`] with no `aux` route: the bus's only input is `late`'s `pre_fader` route,
    /// which follows `late`'s stored mute automation; it is armed with the restarted `late`
    /// (#2011), so the bus input is quiet.
    Following,
    /// The bus's `delay_samples` changed (its delay line restarts).
    BusDelay,
}

/// #1324b gate 7 and D1's sidechain closure on every width, on the control thread. B restarts
/// `eq0` (an insert, or its `delay_samples`) or the bus; `keys` are the keyed inserts A and B share.
/// The restart set is closed over the sidechain edges: a strip keyed from a restarted strip's
/// `post_input`, `insert_send`, `insert_return` or `pre_fader` tap joins it, and so, on a second
/// round, does a strip keyed from that one; a `post_fader` tap adds nothing, and an `input` tap
/// (a track's or the bus's) adds nothing unless its delay line restarts. A submix's `input` tap
/// adds nothing while its line keeps its length, and adds its consumer when the line changes and
/// a carried strip feeds the submix (a following route of a restarted strip is armed, #2011).
/// Each strip in the set is armed. Red if the closure ignores sidechains or a tap kind,
/// stops after one round, adds a `post_fader` or a plain `input`-tap consumer, misses a restarted
/// delay line, or gets either submix rule wrong.
#[test]
fn a_strip_keyed_from_a_restarted_strip_restarts_with_it_on_every_width() {
    let bus = || RouteSource::Submix {
        submix_id: id(ROUTE_BUS),
        tap: session::SendTap::Input,
    };
    // A row: its name, its edit, the keyed inserts (strip, key source) and the restart set.
    type Row<'a> = (&'a str, Restart, Vec<(&'a str, RouteSource)>, &'a [&'a str]);
    let keyed = |tap| vec![("eq1", track_key("eq0", tap))];
    let rows: [Row<'_>; 11] = [
        (
            "insert_return",
            Restart::Insert,
            keyed(session::SendTap::InsertReturn),
            &["eq0", "eq1"],
        ),
        (
            "post_input",
            Restart::Insert,
            keyed(session::SendTap::PostInput),
            &["eq0", "eq1"],
        ),
        (
            "insert_send",
            Restart::Insert,
            keyed(session::SendTap::InsertSend),
            &["eq0", "eq1"],
        ),
        (
            "input",
            Restart::Insert,
            keyed(session::SendTap::Input),
            &["eq0"],
        ),
        (
            "input behind a restarted delay line",
            Restart::Delay,
            keyed(session::SendTap::Input),
            &["eq0", "eq1"],
        ),
        (
            "post_fader",
            Restart::Insert,
            keyed(session::SendTap::PostFader),
            &["eq0"],
        ),
        (
            "a chain",
            Restart::Insert,
            vec![
                ("eq1", track_key("eq0", session::SendTap::PreFader)),
                ("aux", track_key("eq1", session::SendTap::InsertReturn)),
            ],
            &["aux", "eq0", "eq1"],
        ),
        (
            "a kept bus line",
            Restart::Bus,
            vec![("eq1", bus())],
            &[ROUTE_BUS],
        ),
        (
            "a moved bus line",
            Restart::BusLater,
            vec![("eq1", bus())],
            &[ROUTE_BUS, "eq1", "late"],
        ),
        (
            "a following pre-fader route into the bus",
            Restart::Following,
            vec![("eq1", bus())],
            &[ROUTE_BUS, "late"],
        ),
        (
            "the bus input behind a restarted delay line",
            Restart::BusDelay,
            vec![("eq1", bus())],
            &[ROUTE_BUS, "eq1"],
        ),
    ];
    for backend in backends() {
        for (what, edit, keys, restarted) in &rows {
            let mut a_model = if *edit == Restart::Following {
                let mut model = with_mute_automation(route_fade_base_limited(false), "late", None);
                for route in &mut model.routes {
                    if route.id.as_str() == "late-bus" {
                        route.source = track_key("late", session::SendTap::PreFader);
                    }
                }
                model
            } else {
                with_route(
                    route_fade_base_limited(false),
                    "aux-bus",
                    "aux",
                    session::SendTap::PostFader,
                    into_bus(),
                    false,
                )
            };
            if matches!(edit, Restart::BusLater | Restart::Following) {
                add_limiter(&mut a_model, "eq1", "eq1-lim");
            }
            for (strip, source) in keys {
                track_mut(&mut a_model, strip)
                    .inserts
                    .effects
                    .push(keyed_insert("key", source.clone()));
            }
            let mut b_model = a_model.clone();
            match edit {
                Restart::Insert => track_mut(&mut b_model, "eq0")
                    .inserts
                    .effects
                    .push(transparent_insert()),
                Restart::Delay => {
                    let builtins = &mut track_mut(&mut b_model, "eq0").builtins;
                    builtins.left.delay_samples = 32;
                    builtins.right.delay_samples = 32;
                }
                Restart::Bus | Restart::BusLater | Restart::Following => b_model.submixes[0]
                    .inserts
                    .effects
                    .push(transparent_insert()),
                Restart::BusDelay => {
                    let builtins = &mut b_model.submixes[0].builtins;
                    builtins.left.delay_samples = 32;
                    builtins.right.delay_samples = 32;
                }
            }
            if matches!(edit, Restart::BusLater | Restart::Following) {
                add_limiter(&mut b_model, "late", "lim");
            }
            let a = Session::compile(a_model);
            let b = Session::compile(b_model);
            let plan = CellPlan::fresh(&a, backend);
            let mut successor = CellPlan::successor(
                &b,
                &plan.live.prepared.inventory,
                &a.model,
                &EditRamps::new(),
                backend,
            );
            let what = format!("{what} at {backend:?}");
            let set: Vec<&str> = successor
                .live
                .prepared
                .restarted_strips()
                .iter()
                .map(|strip| &**strip)
                .collect();
            assert_eq!(set, *restarted, "{what}");
            let arms = graph::test_only_fade_arms(&mut successor.live.prepared.plan)
                .expect("a graph plan");
            // #2011: `late`'s following `pre_fader` send is armed with it.
            // #2080: the muted `late` arms too (its fire stays muted).
            assert_eq!(
                arms.len(),
                restarted.len() + usize::from(*edit == Restart::Following),
                "{what}: each restarted strip is armed, the muted `late` too, and `late`'s \
                 following send is"
            );
        }
    }
}

/// The oracle sees the fade: `reference` differs from `muted` rendered with no unmute.
fn assert_audible(muted: &Session, feeds: &[&Feed], reference: &FadeRun, backend: Backend) {
    let silent = fade_run(muted, &[], feeds, None, &[], backend);
    assert_ne!(
        silent.blocks, reference.blocks,
        "the fade reaches the output"
    );
}

/// `model` with a stored mute automation curve on track `track` that holds it unmuted, or, with
/// `mute_at`, steps it to muted at that sample: the lane becomes an automated lane, whose live
/// mute goes through the cell's mute terms.
fn with_mute_automation(
    mut model: SessionModel,
    track: &str,
    mute_at: Option<u64>,
) -> SessionModel {
    let owner = model
        .tracks
        .iter()
        .find(|candidate| candidate.id.as_str() == track)
        .expect("the track")
        .id
        .clone();
    model.automation.push(session::Automation {
        id: id("fade-mute"),
        target: session::ControlAddress {
            owner: session::Owner::Strip(owner),
            section: session::Section::Builtins,
            control: 6,
            lane: ParameterChannel::Both,
        },
        segments: vec![session::AutomationSegment {
            shape: session::AutomationShape::Step,
            start_sample: 0,
            end_sample: mute_at.unwrap_or(1),
            start_value: 0.0,
            end_value: if mute_at.is_some() { 1.0 } else { 0.0 },
            unit: ParameterUnit::Linear,
        }],
        offset: session::AutomationOffset::default(),
    });
    model
}

/// Issue #1288 gate 1 on an automated lane (attempt 3): the added track carries stored mute
/// automation and waits for its anchored source at block 9. Two rows: the curve holds it unmuted
/// and a user mute lands at block 7; or the curve itself steps to muted at sample 896 (block 7).
/// Either mute must end the arm, so the fire time passes and the track stays muted: the run
/// equals the post-edit session with the track muted from frame 0, and no fade. Red if an armed
/// lane's mute terms, or the curve driver's hold, compare with the arm's own mute and skip the
/// mute (the fire then unmutes a strip the user or the curve muted).
#[test]
fn a_user_mute_on_an_automated_armed_lane_outlasts_the_fire() {
    let backend = Backend::current();
    let ramp = LiveRamps::for_session(&fade_base()).mute_samples;
    let (kept, aux) = fade_feeds();
    let (added, reference_added) = anchored_feeds();
    let a = Session::compile(unlimited_fade_base());
    let mute = CellWrite {
        block: SWAP_BLOCK + 1,
        strip: FADE_TRACK,
        record: CellRecord::Fader(mute_both(true, ramp)),
    };
    let curve_mute = ((SWAP_BLOCK + 1) * QUANTUM) as u64;
    let reference_feeds = [&kept, &aux, &reference_added];
    for (case, mute_at, writes) in [
        ("a user mute", None, vec![mute]),
        ("a curve mute", Some(curve_mute), Vec::new()),
    ] {
        let b = Session::compile(with_mute_automation(
            anchored_fade_model(false),
            FADE_TRACK,
            mute_at,
        ));
        let muted = Session::compile(with_mute_automation(
            anchored_fade_model(true),
            FADE_TRACK,
            mute_at,
        ));
        let swap = [FadeSwap {
            block: SWAP_BLOCK,
            to: &b,
            committed: &a.model,
            prepared_at: None,
        }];
        let anchored = Anchored {
            feed: &added,
            anchor_block: FADE_ANCHOR_BLOCK,
        };
        let run = fade_run(&a, &swap, &[&kept, &aux], Some(anchored), &writes, backend);
        assert_eq!(run.arms[0].len(), 1, "{case}: the automated track is armed");
        let reference = fade_run(&muted, &[], &reference_feeds, None, &[], backend);
        assert_eq!(
            first_fade_difference(&run, &reference),
            None,
            "{case} outlasts the fire time"
        );
    }
    // The oracle can fail: the muted reference given an unmute at the anchor is heard.
    let muted = Session::compile(with_mute_automation(
        anchored_fade_model(true),
        FADE_TRACK,
        None,
    ));
    let reference = fade_run(&muted, &[], &reference_feeds, None, &[], backend);
    let unmuted = fade_run(
        &muted,
        &[],
        &reference_feeds,
        None,
        &[unmute_fade(FADE_ANCHOR_BLOCK, ramp)],
        backend,
    );
    assert_ne!(unmuted.blocks, reference.blocks, "the fade would be heard");
}

/// Issue #1288 D4 for a submix, on the control thread: B adds submix `bus` and sends `eq0`
/// (behind a 486-sample limiter) and `eq1` (no latency) to it. The bus's input sums at 486, so
/// `eq1`'s route into it gets a 486-sample compensation line that starts at rest; the bus has no
/// latent node of its own, so its `D` is that line, 486. Red if `D` drops the route compensation
/// term (0 here).
#[test]
fn an_added_submix_delays_its_fade_by_its_longest_route_line() {
    let mut a_model = fade_base();
    add_limiter(&mut a_model, "eq0", "lim");
    let a = Session::compile(a_model.clone());
    let mut b_model = a_model;
    add_bus(&mut b_model, "bus");
    send_route_to(&mut b_model, "eq0", "bus");
    send_route_to(&mut b_model, "eq1", "bus");
    let b = Session::compile(b_model);
    let backend = Backend::current();
    let predecessor = CellPlan::fresh(&a, backend);
    let mut successor = CellPlan::successor(
        &b,
        &predecessor.live.prepared.inventory,
        &a.model,
        &EditRamps::new(),
        backend,
    );
    let arms = graph::test_only_fade_arms(&mut successor.live.prepared.plan).expect("a graph plan");
    // #1953b: then the fading copies of the two lane-less routes re-pointed into it, `D = 0`.
    assert_eq!(arms.len(), 3, "one added strip, the bus, then two copies");
    assert_eq!(arms[0].0, 486, "D is the bus's longest route line");
    assert_eq!(
        (arms[1].0, arms[2].0),
        (0, 0),
        "the copies wait for nothing"
    );
}

/// Issue #1288 D4 (gate 4(d) on the control thread): an added track whose compressor is keyed
/// from the `pre_fader` tap of a carried track behind a 486-sample true-peak limiter has no latent
/// node of its own, but PDC puts a 486-sample compensation line on its main path, so its `D` is
/// 486. Red if `D` counts node latencies only (0 here).
#[test]
fn a_late_sidechain_key_delays_the_fade_by_its_compensation_line() {
    let mut a_model = fade_base();
    add_limiter(&mut a_model, "eq1", "lim");
    let a = Session::compile(a_model.clone());
    let mut b_model = with_fade_track(a_model, SOURCE, false, false);
    track_mut(&mut b_model, FADE_TRACK)
        .inserts
        .effects
        .push(Effect {
            id: id("duck"),
            identity: EffectIdentity::Native {
                effect_id: id("miso.compressor"),
            },
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::Maximum,
            params: Vec::new(),
            sidechain: SidechainDeclaration::Routed(session::Sidechain {
                source: RouteSource::Track {
                    track_id: id("eq1"),
                    tap: session::SendTap::PreFader,
                },
                port_id: id("sidechain-in"),
            }),
        });
    let b = Session::compile(b_model);
    let backend = Backend::current();
    let predecessor = CellPlan::fresh(&a, backend);
    let mut successor = CellPlan::successor(
        &b,
        &predecessor.live.prepared.inventory,
        &a.model,
        &EditRamps::new(),
        backend,
    );
    let arms = graph::test_only_fade_arms(&mut successor.live.prepared.plan).expect("a graph plan");
    assert_eq!(arms.len(), 1, "one added track");
    assert_eq!(arms[0].0, 486, "D is the key's compensation line");
}

/// Gate 1: nine tracks with non-unity faders and non-centre pans, plus #1276's input settings; B
/// adds a muted track whose ID sorts first. Every block of the swapped run equals a fresh B fed
/// from frame 0, nothing restarts, and all eighteen strip stages move.
fn strip_state_survives_a_swap(backend: Backend) {
    let a = Session::compile(strip_state_session());
    let b = Session::compile(with_muted_track(strip_state_session()));
    assert_eq!(
        b.compiled.normalized_model().tracks[0].id.as_str(),
        MUTED_TRACK
    );
    let feed = Feed::new(SOURCE, 5);
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
    assert!(run.restarted.is_empty(), "{backend:?}: {:?}", run.restarted);
    // Nine strips: one source move, nine input lanes, and a fader and a matrix move each.
    assert_eq!(a.prepare(backend).inventory.strip_stage_count(), 18);
    assert_eq!(run.carry_counts, [1, 9, 18, 0]);
}

/// Red if a stage is keyed by lane or index, not strip ID (the added track sorts first, so every
/// kept lane moves and two cross into the next bank), if a stage is left out of the program, or
/// if the join lists a strip that carried.
#[test]
fn strip_state_survives_a_swap_on_every_width() {
    for backend in backends() {
        strip_state_survives_a_swap(backend);
    }
}

/// A strip whose chain has an effect between its input section and its fader still carries its
/// fader and matrix stage: the fixture's nine tracks keep their console slots and inserts, so the
/// builtin stages sit behind effect slots of the same bank chain. Red if the walk over a chain's
/// slots stops at the first slot that is not a builtin stage: no row is recorded, nothing carries,
/// and every strip is listed as restarted.
#[cfg(target_feature = "avx2")]
fn a_strip_behind_an_effect_still_carries(backend: Backend) {
    let mut model = parse_session_json(FIXTURE).expect("fixture parses");
    for track in &mut model.tracks {
        assert!(
            !track.console.is_empty() || !track.inserts.effects.is_empty(),
            "the fixture's tracks carry effects"
        );
    }
    let a = Session::compile(model.clone());
    add_track(&mut model, "eq0", MUTED_TRACK, SOURCE, true);
    let b = Session::compile(model);
    assert_eq!(a.prepare(backend).inventory.strip_stage_count(), 18);
    let feed = Feed::new(SOURCE, 15);
    let run = swapped_run(&a, &b, &[&feed], &[&feed], backend, Successor::Carry);
    assert_eq!(run.blocks[SWAP_BLOCK].carry, CarryOutcome::Carried);
    assert!(run.restarted.is_empty(), "{backend:?}: {:?}", run.restarted);
    assert_eq!(
        run.carry_counts[2], 18,
        "{backend:?}: all eighteen strip stages move"
    );
}

/// Eight lanes only: the fixture's console equalizer is not banked at four
/// (`console.slot.foreign_width`).
#[cfg(target_feature = "avx2")]
#[test]
fn a_strip_behind_an_effect_still_carries_at_eight_lanes() {
    a_strip_behind_an_effect_still_carries(Backend::Simd8);
}

/// The model, the records and the sessions of gate 2: a fader ramp over `RAMP_FRAMES` frames, a
/// pan ramp and a mute fade, all in flight across the swap.
const RAMP_FRAMES: u32 = 400;

fn ramp_writes() -> Vec<CellWrite> {
    [
        (MOVED, CellRecord::Fader(fader_both(-12.0, RAMP_FRAMES))),
        (MOVED, CellRecord::Matrix(pan_record([0.2, 0.9], 300))),
        (FADED, CellRecord::Fader(mute_both(true, 350))),
    ]
    .into_iter()
    .map(|(strip, record)| CellWrite {
        block: SWAP_BLOCK,
        strip,
        record,
    })
    .collect()
}

/// A's model after [`ramp_writes`]: the base a structural transaction commits over.
fn ramped_model() -> SessionModel {
    let model = with_fader(strip_state_session(), MOVED, [-12.0, -12.0]);
    let model = with_pan(model, MOVED, [0.2, 0.9], 300);
    with_mute(model, FADED, true)
}

fn feed_for_strips() -> Feed {
    Feed::new(SOURCE, 7)
}

/// Gate 2/3, first case: the records are written to A's cells between blocks 5 and 6, committed in
/// the base, and A is swapped out at block 6 before it renders again, so they are still dirty in
/// A's cells and only the carry's `apply_pending` applies them. The reference is A plus the muted
/// track, fed the same records at the same blocks.
fn a_ramp_in_flight_finishes_after_the_swap(backend: Backend) {
    let a = Session::compile(strip_state_session());
    let b_after = Session::compile(with_muted_track(ramped_model()));
    let b_before = Session::compile(with_muted_track(strip_state_session()));
    let committed = ramped_model();
    let feed = feed_for_strips();
    let writes = ramp_writes();
    let reference = cell_run(&b_before, None, &feed, &writes, backend);
    let silent = cell_run(&b_before, None, &feed, &[], backend);
    assert!(
        reference.blocks[SWAP_BLOCK..SWAP_BLOCK + 3]
            .iter()
            .zip(&silent.blocks[SWAP_BLOCK..SWAP_BLOCK + 3])
            .all(|(moved, rest)| moved != rest),
        "{backend:?}: the ramp is audible in the three blocks after the swap"
    );
    let run = cell_run(
        &a,
        Some(Swap {
            to: &b_after,
            committed: &committed,
            successor_writes: &[],
            edit_ramps: &EditRamps::new(),
        }),
        &feed,
        &writes,
        backend,
    );
    assert_swapped_like(&run, &reference, &format!("{backend:?}"));
    assert!(run.restarted.is_empty(), "{:?}", run.restarted);
}

/// Gate 2/3, second case: the records are written to the pending successor's cells (a live edit
/// committed after the structural transaction, #1053 D7), so the carry's lane state is at rest and
/// the successor applies the records at its first block.
fn records_in_the_pending_successor_apply_at_its_first_block(backend: Backend) {
    let a = Session::compile(strip_state_session());
    let b = Session::compile(with_muted_track(strip_state_session()));
    let committed = strip_state_session();
    let feed = feed_for_strips();
    let writes = ramp_writes();
    let reference = cell_run(&b, None, &feed, &writes, backend);
    let successor_writes: Vec<(&'static str, CellRecord)> = writes
        .iter()
        .map(|write| (write.strip, write.record))
        .collect();
    let run = cell_run(
        &a,
        Some(Swap {
            to: &b,
            committed: &committed,
            successor_writes: &successor_writes,
            edit_ramps: &EditRamps::new(),
        }),
        &feed,
        &[],
        backend,
    );
    assert_swapped_like(&run, &reference, &format!("{backend:?}"));
}

#[test]
fn a_ramp_in_flight_finishes_after_the_swap_on_every_width() {
    for backend in backends() {
        a_ramp_in_flight_finishes_after_the_swap(backend);
    }
}

#[test]
fn records_in_the_pending_successor_apply_at_its_first_block_on_every_width() {
    for backend in backends() {
        records_in_the_pending_successor_apply_at_its_first_block(backend);
    }
}

/// Gate 4: B also changes strip X's fader and pan, live values. Run 1 prepares B from A's base and
/// X's stage carries, then D5's records retarget it. Run 2 writes the same records to A just
/// before the swap and prepares B from a base that already holds them. The runs are bit-identical
/// in every block, and X is not in the restart set.
fn a_live_difference_retargets_the_carried_stage(backend: Backend) {
    let after = |model: SessionModel| {
        let model = with_fader(model, MOVED, [-4.0, -6.0]);
        with_pan(model, MOVED, [0.8, 0.6], 300)
    };
    let a = Session::compile(strip_state_session());
    let b = Session::compile(with_muted_track(after(strip_state_session())));
    let before = strip_state_session();
    let moved = after(strip_state_session());
    let feed = feed_for_strips();
    // A retarget carries the ramp the classifier would give the same edit: the session's own
    // fader length (#1054 D4); the pan carries the model's own 300-sample window (#1054 D5).
    let fader_samples = LiveRamps::for_session(&moved).for_row(LiveRampRow::Fader);
    assert_ne!(fader_samples, 0, "the default table ramps a fader");
    let records = [
        CellRecord::Fader(TrackFaderRecord::FaderDb {
            lanes: BuiltinLaneSelector::Left,
            db: -4.0,
            smoothing_samples: fader_samples,
        }),
        CellRecord::Fader(TrackFaderRecord::FaderDb {
            lanes: BuiltinLaneSelector::Right,
            db: -6.0,
            smoothing_samples: fader_samples,
        }),
        CellRecord::Matrix(pan_record([0.8, 0.6], 300)),
    ];
    let writes: Vec<CellWrite> = records
        .iter()
        .map(|record| CellWrite {
            block: SWAP_BLOCK,
            strip: MOVED,
            record: *record,
        })
        .collect();
    let retargeted = cell_run(
        &a,
        Some(Swap {
            to: &b,
            committed: &before,
            successor_writes: &[],
            edit_ramps: &EditRamps::new(),
        }),
        &feed,
        &[],
        backend,
    );
    let written = cell_run(
        &a,
        Some(Swap {
            to: &b,
            committed: &moved,
            successor_writes: &[],
            edit_ramps: &EditRamps::new(),
        }),
        &feed,
        &writes,
        backend,
    );
    assert_eq!(retargeted.carry, CarryOutcome::Carried);
    assert_eq!(written.carry, CarryOutcome::Carried);
    assert!(
        retargeted.restarted.is_empty() && written.restarted.is_empty(),
        "{backend:?}: {:?} {:?}",
        retargeted.restarted,
        written.restarted
    );
    assert_eq!(
        first_cell_difference(&retargeted, &written),
        None,
        "{backend:?}: carry then retarget sounds like live edit then structural edit"
    );
    // Not vacuous: the pan ramp is audible after the swap, so a retarget that is dropped or a
    // stage that restarts at the new values differs from the written run.
    let unmoved = cell_run(
        &a,
        Some(Swap {
            to: &Session::compile(with_muted_track(strip_state_session())),
            committed: &before,
            successor_writes: &[],
            edit_ramps: &EditRamps::new(),
        }),
        &feed,
        &[],
        backend,
    );
    assert_ne!(
        unmoved.blocks[SWAP_BLOCK + 1],
        retargeted.blocks[SWAP_BLOCK + 1]
    );
}

#[test]
fn a_live_difference_retargets_the_carried_stage_on_every_width() {
    for backend in backends() {
        a_live_difference_retargets_the_carried_stage(backend);
    }
}

/// #1394 D6: the same transaction as gate 4, but its edits gave strip X's fader a 37-sample ramp,
/// its mute none, and its pan an explicit 100 (against the model's own 300-sample window). A
/// carry retarget is the record of that edit, so the run equals the one that writes the records
/// the classifier derives for it: `FaderDb` over 37 and the pan over 100. Red if the join reads
/// the session's lengths and ignores the transaction's: the retarget then ramps 480 and 300, and
/// differs from the written run.
fn a_retarget_carries_the_edit_ramp_of_its_transaction(backend: Backend) {
    let after = |model: SessionModel| {
        let model = with_fader(model, MOVED, [-4.0, -6.0]);
        with_pan(model, MOVED, [0.8, 0.6], 300)
    };
    let a = Session::compile(strip_state_session());
    let b = Session::compile(with_muted_track(after(strip_state_session())));
    let before = strip_state_session();
    let moved = after(strip_state_session());
    let feed = feed_for_strips();
    let mut edit_ramps = EditRamps::new();
    let strip = session::EditRampTarget::Strip(id(MOVED));
    edit_ramps.set(&strip, session::EditRampRow::Fader, Some(37));
    edit_ramps.set(&strip, session::EditRampRow::Matrix, Some(100));
    let writes: Vec<CellWrite> = [
        CellRecord::Fader(TrackFaderRecord::FaderDb {
            lanes: BuiltinLaneSelector::Left,
            db: -4.0,
            smoothing_samples: 37,
        }),
        CellRecord::Fader(TrackFaderRecord::FaderDb {
            lanes: BuiltinLaneSelector::Right,
            db: -6.0,
            smoothing_samples: 37,
        }),
        CellRecord::Matrix(pan_record([0.8, 0.6], 100)),
    ]
    .into_iter()
    .map(|record| CellWrite {
        block: SWAP_BLOCK,
        strip: MOVED,
        record,
    })
    .collect();
    let run = |committed: &SessionModel, edit_ramps: &EditRamps, writes: &[CellWrite]| {
        cell_run(
            &a,
            Some(Swap {
                to: &b,
                committed,
                successor_writes: &[],
                edit_ramps,
            }),
            &feed,
            writes,
            backend,
        )
    };
    let retargeted = run(&before, &edit_ramps, &[]);
    let written = run(&moved, &EditRamps::new(), &writes);
    assert_eq!(retargeted.carry, CarryOutcome::Carried);
    assert_eq!(
        first_cell_difference(&retargeted, &written),
        None,
        "{backend:?}: the retarget ramps over the edit's lengths"
    );
    // Not vacuous: without the transaction's lengths the same retarget ramps over the session's.
    let session_ramps = run(&before, &EditRamps::new(), &[]);
    assert_ne!(
        session_ramps.blocks[SWAP_BLOCK], retargeted.blocks[SWAP_BLOCK],
        "{backend:?}: the edit's lengths are audible"
    );
}

#[test]
fn a_retarget_carries_the_edit_ramp_of_its_transaction_on_every_width() {
    for backend in backends() {
        a_retarget_carries_the_edit_ramp_of_its_transaction(backend);
    }
}

/// `model` with a VCA over `track`, offset `db` on both channels.
fn with_vca(mut model: SessionModel, track: &str, db: f32) -> SessionModel {
    model.vcas.push(Vca {
        id: id("trim-group"),
        fader: DualMonoFader {
            left_db: db,
            right_db: db,
            left_mute: false,
            right_mute: false,
        },
        members: vec![id(track)],
    });
    model
}

/// The strip beside [`MOVED`] in the predecessor's first bank at either width, which carries.
const CARRIED: &str = "eq1";

/// #1312 D2: `live_values_superseded` counts the writes a carry drain replaces on every lane that
/// carries, whichever lanes share their bank. B adds a VCA over [`MOVED`], a live difference
/// (#1558): its stage carries and is retargeted, so its writes are counted like [`CARRIED`]'s.
/// Three fader writes sit unread in [`MOVED`]'s cells and
/// three fader and three pan writes in [`CARRIED`]'s when the swap block hands over. The count of a
/// stage that restarts instead (an input lane, `a_restarting_input_lane_...`) stays zero. A fourth
/// run commits a base without [`MOVED`], so its stage has no join and restarts while [`CARRIED`],
/// its bank-mate, carries: a bank that mixes a carried and a non-carried lane counts only the
/// carried lane's writes. That run is red when the strip drain's lane mask covers the whole bank
/// instead of the moves.
fn a_carried_lane_counts_its_replaced_writes(backend: Backend) {
    let a = Session::compile(strip_state_session());
    let b = Session::compile(with_vca(
        with_muted_track(strip_state_session()),
        MOVED,
        3.0,
    ));
    let feed = feed_for_strips();
    let without_moved = without_track(strip_state_session(), MOVED);
    let run = |strips: &[&'static str], committed: &SessionModel| {
        let faders = |strip| {
            [-3.0, -4.0, -5.0].map(|db| CellWrite {
                block: SWAP_BLOCK,
                strip,
                record: CellRecord::Fader(fader_both(db, 0)),
            })
        };
        let mut writes = Vec::new();
        for &strip in strips {
            writes.extend(faders(strip));
            if strip == CARRIED {
                writes.extend([0.9, 0.8, 0.7].map(|pan| CellWrite {
                    block: SWAP_BLOCK,
                    strip,
                    record: CellRecord::Matrix(pan_record([pan, 0.5], 0)),
                }));
            }
        }
        let run = cell_run(
            &a,
            Some(Swap {
                to: &b,
                committed,
                successor_writes: &[],
                edit_ramps: &EditRamps::new(),
            }),
            &feed,
            &writes,
            backend,
        );
        assert_eq!(run.carry, CarryOutcome::Carried, "{backend:?}");
        run
    };
    let carried_run = |strips: &[&'static str]| {
        let run = run(strips, &a.model);
        assert!(
            run.restarted.is_empty(),
            "{backend:?}: a VCA added over a strip restarts nothing: {:?}",
            run.restarted
        );
        run
    };
    let moved = carried_run(&[MOVED]);
    let carried = carried_run(&[CARRIED]);
    let both = carried_run(&[MOVED, CARRIED]);
    let restarting = run(&[MOVED, CARRIED], &without_moved);
    // Three writes to a cell replace two; a Both fader write fills the left and the right cell,
    // and the pan write fills the one matrix cell: two cells of the fader and one of the pan.
    assert_eq!(
        moved.carry_superseded,
        2 * 2,
        "{backend:?}: the retargeted lane counts the writes its drain replaced"
    );
    assert_eq!(
        carried.carry_superseded,
        2 * 2 + 2,
        "{backend:?}: a carried lane counts the writes its drain replaced"
    );
    assert_eq!(
        both.carry_superseded,
        moved.carry_superseded + carried.carry_superseded,
        "{backend:?}: the counts of two lanes of one bank add"
    );
    assert_eq!(
        restarting.carry_superseded, carried.carry_superseded,
        "{backend:?}: a lane the swap does not carry adds nothing to its bank-mate's count"
    );
}

#[test]
fn a_carried_lane_counts_its_replaced_writes_on_every_width() {
    for backend in backends() {
        a_carried_lane_counts_its_replaced_writes(backend);
    }
}

/// #1630 f, D15-2 on the strip cells (the #1277 carry): a carried live value that the successor's
/// retarget replaces in the adopting block is counted once. A left-only fader write sits unread in
/// [`MOVED`]'s cells at the hand-over and is committed in the base; B moves that fader again. The
/// carry drain applies the written value, and the successor's retarget replaces it before any
/// sample renders: one value, one cell, superseded. The run is bit-identical to one whose base
/// never held the write, and with no retarget nothing is counted. Red without the adopting block's
/// count (0, not 1).
fn a_carried_strip_value_the_retarget_replaces_is_counted(backend: Backend) {
    const WRITTEN_DB: f32 = -7.5;
    const MOVED_DB: f32 = -2.25;
    let base = strip_state_session();
    let right = base
        .tracks
        .iter()
        .find(|track| track.id.as_str() == MOVED)
        .expect("the moved strip")
        .fader
        .right_db;
    let a = Session::compile(base.clone());
    let written = with_fader(base.clone(), MOVED, [WRITTEN_DB, right]);
    let b = Session::compile(with_muted_track(with_fader(
        base.clone(),
        MOVED,
        [MOVED_DB, right],
    )));
    let feed = feed_for_strips();
    // The second write: a second value to the same cell, two blocks after the adopting block,
    // through the successor's normal write path. The carry marks are spent by then, so it is a plain
    // write, never a second carried value.
    let writes = [
        CellWrite {
            block: SWAP_BLOCK,
            strip: MOVED,
            record: CellRecord::Fader(TrackFaderRecord::FaderDb {
                lanes: BuiltinLaneSelector::Left,
                db: WRITTEN_DB,
                smoothing_samples: 64,
            }),
        },
        CellWrite {
            block: SWAP_BLOCK + 2,
            strip: MOVED,
            record: CellRecord::Fader(TrackFaderRecord::FaderDb {
                lanes: BuiltinLaneSelector::Left,
                db: -4.0,
                smoothing_samples: 64,
            }),
        },
    ];
    let run = |to: &Session, committed: &SessionModel, writes: &[CellWrite]| {
        let run = cell_run(
            &a,
            Some(Swap {
                to,
                committed,
                successor_writes: &[],
                edit_ramps: &EditRamps::new(),
            }),
            &feed,
            writes,
            backend,
        );
        assert_eq!(run.carry, CarryOutcome::Carried, "{backend:?}");
        assert!(run.restarted.is_empty(), "{backend:?}");
        run
    };
    let replaced = run(&b, &written, &writes);
    assert_eq!(
        (replaced.carry_superseded, replaced.successor_superseded),
        (0, 1),
        "{backend:?}: the carried value the retarget replaced is counted once"
    );
    assert_eq!(
        replaced.final_superseded, 1,
        "{backend:?}: a later write to the cell is not a second carried value"
    );
    let unwritten = run(&b, &base, &writes[1..]);
    assert_eq!(unwritten.successor_superseded, 0, "{backend:?}");
    assert_eq!(unwritten.final_superseded, 0, "{backend:?}");
    assert_eq!(
        first_cell_difference(&replaced, &unwritten),
        None,
        "{backend:?}: the replaced carried value moves no bit"
    );
    let kept = run(
        &Session::compile(with_muted_track(written.clone())),
        &written,
        &writes,
    );
    assert_eq!(
        (kept.carry_superseded, kept.successor_superseded),
        (0, 0),
        "{backend:?}: a carried value nothing replaces is not counted"
    );
    assert_eq!(kept.final_superseded, 0, "{backend:?}");
}

#[test]
fn a_carried_strip_value_the_retarget_replaces_is_counted_on_every_width() {
    for backend in backends() {
        a_carried_strip_value_the_retarget_replaces_is_counted(backend);
    }
}

/// #1558 D4 (was #1276's gate 5, when a VCA was a prepared difference): B adds a VCA with a +3 dB
/// offset over X. X's stage carries and is retargeted to the effective value, over the `VcaOffset`
/// row's session length, and `restarted_strips()` is empty. Run 1 prepares B from A's base. Run 2
/// writes the same effective records to A just before the swap and prepares B from a base that
/// already holds them. The runs are bit-identical in every block. Red if the join compares X's own
/// fader (no difference, so no retarget) or still prepares a strip under a VCA.
fn a_vca_added_over_a_strip_retargets_its_carried_stage(backend: Backend) {
    let before = strip_state_session();
    let a = Session::compile(before.clone());
    let b_model = with_vca(with_muted_track(strip_state_session()), MOVED, 3.0);
    let b = Session::compile(b_model);
    // X's effective fader: its own plus the offset, on each lane.
    let own = [-1.0, -2.0];
    let effective = [own[0] + 3.0, own[1] + 3.0];
    let moved = with_fader(strip_state_session(), MOVED, effective);
    let feed = feed_for_strips();
    let fader_samples = LiveRamps::for_session(&moved).for_row(LiveRampRow::VcaOffset);
    assert_ne!(fader_samples, 0, "the default table ramps a VCA offset");
    let writes: Vec<CellWrite> = [
        (BuiltinLaneSelector::Left, effective[0]),
        (BuiltinLaneSelector::Right, effective[1]),
    ]
    .into_iter()
    .map(|(lanes, db)| CellWrite {
        block: SWAP_BLOCK,
        strip: MOVED,
        record: CellRecord::Fader(TrackFaderRecord::FaderDb {
            lanes,
            db,
            smoothing_samples: fader_samples,
        }),
    })
    .collect();
    let no_ramps = EditRamps::new();
    let retargeted = cell_run(
        &a,
        Some(Swap {
            to: &b,
            committed: &before,
            successor_writes: &[],
            edit_ramps: &no_ramps,
        }),
        &feed,
        &[],
        backend,
    );
    let written = cell_run(
        &a,
        Some(Swap {
            to: &b,
            committed: &moved,
            successor_writes: &[],
            edit_ramps: &no_ramps,
        }),
        &feed,
        &writes,
        backend,
    );
    assert_eq!(retargeted.carry, CarryOutcome::Carried);
    assert!(
        retargeted.restarted.is_empty(),
        "{backend:?}: {:?}",
        retargeted.restarted
    );
    assert_eq!(
        first_cell_difference(&retargeted, &written),
        None,
        "{backend:?}: the retarget is the live record"
    );
    // Not vacuous: the offset is audible after the swap.
    let without = cell_run(
        &a,
        Some(Swap {
            to: &Session::compile(with_muted_track(strip_state_session())),
            committed: &before,
            successor_writes: &[],
            edit_ramps: &EditRamps::new(),
        }),
        &feed,
        &[],
        backend,
    );
    assert_ne!(
        retargeted.blocks[SWAP_BLOCK + 1],
        without.blocks[SWAP_BLOCK + 1],
        "{backend:?}: the VCA offset is audible"
    );
}

#[test]
fn a_vca_added_over_a_strip_retargets_its_carried_stage_on_every_width() {
    for backend in backends() {
        a_vca_added_over_a_strip_retargets_its_carried_stage(backend);
    }
}

/// `model` with `track`'s main route sent through a new submix, `follows_mute` set as given.
fn with_send_through_submix(
    mut model: SessionModel,
    track: &str,
    follows_mute: bool,
) -> SessionModel {
    let main = format!("{track}-main");
    let route = model
        .routes
        .iter_mut()
        .find(|route| route.id.as_str() == main)
        .expect("the track's main route");
    let mut onward = route.clone();
    route.destination = session::RouteDestination::SubmixInput {
        submix_id: id("bus"),
    };
    route.follows_mute = follows_mute;
    let RouteSource::Track { tap, .. } = onward.source.clone() else {
        panic!("a track route");
    };
    onward.id = id("bus-main");
    onward.source = RouteSource::Submix {
        submix_id: id("bus"),
        tap,
    };
    onward.follows_mute = false;
    model.routes.push(onward);
    model
        .submixes
        .push(session::Submix::unity(id("bus"), &model.console));
    model
}

/// The writes of a live mute of [`MOVED`] on `lanes` (the lanes whose mute turns on) at the swap
/// block: the strip's mute record and, when the send through the submix follows the strip's mute,
/// the send's gate record over the same ramp (#1558 D1: a live commit writes both).
fn followed_mute_writes(follows_mute: bool, lanes: [bool; 2], mute_samples: u32) -> Vec<CellWrite> {
    let selector = match lanes {
        [true, true] => BuiltinLaneSelector::Both,
        [true, false] => BuiltinLaneSelector::Left,
        [false, true] => BuiltinLaneSelector::Right,
        [false, false] => panic!("a mute on no lane"),
    };
    let mut writes = vec![CellWrite {
        block: SWAP_BLOCK,
        strip: MOVED,
        record: CellRecord::Fader(TrackFaderRecord::Mute {
            lanes: selector,
            muted: true,
            smoothing_samples: mute_samples,
        }),
    }];
    if follows_mute {
        let record = host_core::route_record(0.0, [1.0, 0.0, 0.0, 1.0], false, lanes, mute_samples)
            .expect("a send record");
        writes.push(CellWrite {
            block: SWAP_BLOCK,
            strip: MOVED,
            record: CellRecord::Route("eq0-main", record),
        });
    }
    writes
}

/// A mute change on a track that a `follows_mute` route follows is a live difference like any other
/// (#1558 D4): the strip's stage carries and is retargeted over the session's mute length, and the
/// strip is not in the restart set, whether or not a route follows it. The following route is
/// prepared from the successor's model, so it is gated from the swap block on, in both runs. Run 1
/// prepares B from A's base; run 2 writes the same mute record to A just before the swap and
/// prepares B from a base that already holds it; the runs are bit-identical. Red if the join still
/// treats the followed mute as prepared: X would restart at rest at the new mute and drop the ramp
/// a live mute record has.
fn a_followed_mute_change_carries_and_retargets_the_stage(backend: Backend) {
    let a_model =
        |follows_mute| with_send_through_submix(strip_state_session(), MOVED, follows_mute);
    let b_model = |follows_mute| with_mute(with_muted_track(a_model(follows_mute)), MOVED, true);
    let feed = feed_for_strips();
    for follows_mute in [true, false] {
        let (a, b) = (
            Session::compile(a_model(follows_mute)),
            Session::compile(b_model(follows_mute)),
        );
        let reference = cell_run(&b, None, &feed, &[], backend);
        let run = cell_run(
            &a,
            Some(Swap {
                to: &b,
                committed: &a.model,
                successor_writes: &[],
                edit_ramps: &EditRamps::new(),
            }),
            &feed,
            &[],
            backend,
        );
        assert_eq!(run.carry, CarryOutcome::Carried);
        assert!(
            run.restarted.is_empty(),
            "{backend:?} follows {follows_mute}: {:?}",
            run.restarted
        );
        // The retarget ramps over the session's mute length (#1054 D4), so it sounds like the live
        // mute record written to the predecessor, and unlike a mute prepared at rest.
        let moved = with_mute(a_model(follows_mute), MOVED, true);
        let mute_samples = LiveRamps::for_session(&moved).for_row(LiveRampRow::Mute);
        assert_ne!(mute_samples, 0, "the default table ramps a mute");
        let written = cell_run(
            &a,
            Some(Swap {
                to: &b,
                committed: &moved,
                successor_writes: &[],
                edit_ramps: &EditRamps::new(),
            }),
            &feed,
            &followed_mute_writes(follows_mute, [true; 2], mute_samples),
            backend,
        );
        assert_eq!(
            run.blocks[SWAP_BLOCK..],
            written.blocks[SWAP_BLOCK..],
            "{backend:?} follows {follows_mute}: a live mute carries and retargets over \
             the session's mute ramp"
        );
        // A strip whose only route follows its mute is silent at the swap block either way; the
        // ramp is heard on the strip that nothing follows.
        if !follows_mute {
            assert_ne!(
                run.blocks[SWAP_BLOCK..],
                reference.blocks[SWAP_BLOCK..],
                "{backend:?}: the mute ramp is audible, not a step"
            );
        }
    }
}

/// When X's followed mute changes and X's pan changes too (#1558 D4), both stages carry and are
/// retargeted: nothing restarts. The reference writes the pan record to A just before the swap, as
/// gate 4 does, so the mute retarget is identical in both runs and the matrix carry is what is
/// compared. Red if the followed mute makes the matrix family prepared: the matrix
/// stage then restarts at the new pan, and the pan ramp the carry continues is lost.
fn a_followed_mute_change_leaves_the_matrix_stage_carrying(backend: Backend) {
    let before = with_send_through_submix(strip_state_session(), MOVED, true);
    let moved = with_pan(before.clone(), MOVED, [0.8, 0.6], 300);
    // Only the left channel mutes, so X's right channel stays audible and its pan ramp is heard.
    let after = |base: &SessionModel| {
        let mut model = with_muted_track(base.clone());
        track_mut(&mut model, MOVED).fader.left_mute = true;
        model
    };
    let (a, b) = (
        Session::compile(before.clone()),
        Session::compile(after(&moved)),
    );
    let feed = feed_for_strips();
    let run = cell_run(
        &a,
        Some(Swap {
            to: &b,
            committed: &before,
            successor_writes: &[],
            edit_ramps: &EditRamps::new(),
        }),
        &feed,
        &[],
        backend,
    );
    let written = [CellWrite {
        block: SWAP_BLOCK,
        strip: MOVED,
        record: CellRecord::Matrix(pan_record([0.8, 0.6], 300)),
    }];
    let reference = cell_run(
        &a,
        Some(Swap {
            to: &b,
            committed: &moved,
            successor_writes: &[],
            edit_ramps: &EditRamps::new(),
        }),
        &feed,
        &written,
        backend,
    );
    assert_eq!(run.carry, CarryOutcome::Carried);
    assert_eq!(reference.carry, CarryOutcome::Carried);
    assert!(
        run.restarted.is_empty() && reference.restarted.is_empty(),
        "{backend:?}: the followed mute restarts nothing: {:?}",
        run.restarted
    );
    assert_eq!(
        first_cell_difference(&run, &reference),
        None,
        "{backend:?}: X's matrix stage carries and is retargeted, with its fader/mute stage"
    );
    // Not vacuous: the pan ramp is audible after the swap, so a matrix stage that restarts at the
    // new pan sounds different.
    let unmoved = cell_run(
        &a,
        Some(Swap {
            to: &Session::compile(after(&before)),
            committed: &before,
            successor_writes: &[],
            edit_ramps: &EditRamps::new(),
        }),
        &feed,
        &[],
        backend,
    );
    assert_ne!(unmoved.blocks[SWAP_BLOCK + 1], run.blocks[SWAP_BLOCK + 1]);
}

#[test]
fn a_followed_mute_change_leaves_the_matrix_stage_carrying_on_every_width() {
    for backend in backends() {
        a_followed_mute_change_leaves_the_matrix_stage_carrying(backend);
    }
}

#[test]
fn a_followed_mute_change_carries_and_retargets_the_stage_on_every_width() {
    for backend in backends() {
        a_followed_mute_change_carries_and_retargets_the_stage(backend);
    }
}

/// Gate 6: with a ramp in flight, a pending cell value in the predecessor and a retarget in the
/// successor, the swap block makes no allocator call and no audited allocation.
#[test]
fn the_strip_carry_allocates_and_frees_nothing() {
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
    let a = Session::compile(strip_state_session());
    let committed = with_fader(strip_state_session(), MOVED, [-12.0, -12.0]);
    let b_model = with_pan(
        with_muted_track(with_fader(committed.clone(), MOVED, [-12.0, -12.0])),
        FADED,
        [0.3, 0.7],
        200,
    );
    let b = Session::compile(b_model);
    let feed = feed_for_strips();
    let writes = [CellWrite {
        block: SWAP_BLOCK,
        strip: MOVED,
        record: CellRecord::Fader(fader_both(-12.0, RAMP_FRAMES)),
    }];
    for backend in backends() {
        let run = cell_run(
            &a,
            Some(Swap {
                to: &b,
                committed: &committed,
                successor_writes: &[],
                edit_ramps: &EditRamps::new(),
            }),
            &feed,
            &writes,
            backend,
        );
        assert_eq!(run.carry, CarryOutcome::Carried);
        assert_eq!(
            run.swap_allocator,
            ((0, 0, 0), (0, 0)),
            "{backend:?}: allocator calls and audited allocations at the swap block"
        );
    }
}

/// D4: a stage carries only between plans that attach the same control kind. A plan without live
/// controls has no cells to retarget, so the successor's stages start at rest and every strip is
/// listed.
#[test]
fn a_changed_control_kind_carries_no_strip_stage() {
    let a = Session::compile(strip_state_session());
    let b = Session::compile(with_muted_track(strip_state_session()));
    for backend in backends() {
        let predecessor = a.prepare(backend);
        let mut successor = CellPlan::successor(
            &b,
            &predecessor.inventory,
            &a.model,
            &EditRamps::new(),
            backend,
        );
        assert_eq!(successor.live.prepared.restarted_strips().len(), 9);
        assert_eq!(
            carry_counts(&mut successor.live.prepared),
            // The source only: the control kind differs, so no input lane and no strip stage moves.
            [1, 0, 0, 0],
            "{backend:?}"
        );
    }
}

/// D10: a successor's revision is one its gate can stamp. Refused before anything is built.
#[test]
fn a_successor_revision_its_gate_cannot_stamp_is_refused() {
    let a = Session::compile(strip_state_session());
    let b = Session::compile(with_muted_track(strip_state_session()));
    let backend = Backend::current();
    let predecessor = a.prepare(backend);
    for revision in [0, engine::realtime::GATE_REVISION_MAX + 1, u64::MAX] {
        let refused = CellPlan::successor_at(
            &b,
            host_core::SuccessorBase::new(&predecessor.inventory, &a.model, revision),
            backend,
        )
        .err()
        .unwrap_or_else(|| panic!("revision {revision} was accepted"));
        assert_eq!(refused.kind(), PrepareRejection::Session);
        assert_eq!(
            refused.as_bytes(),
            b"host.successor.revision\t$\n",
            "revision {revision}"
        );
    }
    let accepted = engine::realtime::GATE_REVISION_MAX;
    assert!(
        CellPlan::successor_at(
            &b,
            host_core::SuccessorBase::new(&predecessor.inventory, &a.model, accepted),
            backend,
        )
        .is_ok()
    );
}

/// The revision-gate handle of a successor of the nine-track session, prepared with live cells
/// from `committed` to `next`.
fn successor_handle(
    committed: &SessionModel,
    next: SessionModel,
) -> engine::realtime::PlanRevisionGate {
    let a = Session::compile(strip_state_session());
    let b = Session::compile(next);
    let backend = Backend::current();
    let predecessor = CellPlan::fresh(&a, backend);
    let (mut prepared, _handles) =
        host_core::test_only_prepare_host_runtime_with_live_controls_successor_on(
            &b.compiled,
            &caps(),
            &LivePlan::request(),
            host_core::SuccessorBase::new(
                &predecessor.live.prepared.inventory,
                committed,
                successor::SUCCESSOR_REVISION,
            ),
            backend,
        )
        .unwrap_or_else(|failure| panic!("successor: {failure:?}"));
    prepared
        .plan
        .take_revision_gate()
        .expect("the successor's handle")
}

/// D5: a preparation that writes no retarget does not stamp. Red if every successor stamps its
/// revision: the handle would hold it open, and a debug build refuses another revision stamped
/// before it is published.
#[test]
fn a_preparation_without_a_retarget_stamps_nothing() {
    let a = strip_state_session();
    let mut gate = successor_handle(&a, with_muted_track(strip_state_session()));
    gate.stamp(successor::SUCCESSOR_REVISION + 5)
        .expect("a different revision stamps on a handle with no open stamp");
}

// ---------------------------------------------------------------------------------------------
// Issue #1279: a console effect lane keeps its state through a plan swap.
//
// A console slot with no live control lane is an `EffectBankStage`; the carry moves each lane
// through the payload calls (D3), recomputes the witness the stage caches for it (D5) and counts a
// refusal (D6). These cases prepare their plans with no live controls, the form that stage has.
// A slot with live controls is a `LiveControlEffectBankStage` and moves the same way (#1280, the
// cases at the end of this file).

use session::{
    ConsoleEntry, ConsoleSlot, Effect, EffectIdentity, EffectParam, EffectQuality, LinkMode,
    ParameterChannel, ParameterUnit, SidechainDeclaration,
};

/// One slot of the console the cases build.
struct ConsoleEffect {
    slot: &'static str,
    effect: &'static str,
    /// In `post_insert`; else in `pre_insert`.
    post: bool,
    link_mode: LinkMode,
}

/// All six console-eligible effects: the EQ, compressor, gate/expander and transient shaper
/// before the inserts, the soft-clip and the true-peak limiter after them.
const CONSOLE_EFFECTS: [ConsoleEffect; 6] = [
    ConsoleEffect {
        slot: "eq",
        effect: "miso.parametric-eq",
        post: false,
        link_mode: LinkMode::DualMono,
    },
    ConsoleEffect {
        slot: "comp",
        effect: "miso.compressor",
        post: false,
        link_mode: LinkMode::DualMono,
    },
    ConsoleEffect {
        slot: "gate",
        effect: "miso.gate-expander",
        post: false,
        link_mode: LinkMode::DualMono,
    },
    ConsoleEffect {
        slot: "transient",
        effect: "miso.transient-shaper",
        post: false,
        link_mode: LinkMode::DualMono,
    },
    ConsoleEffect {
        slot: "clip",
        effect: "miso.soft-clip",
        post: true,
        link_mode: LinkMode::DualMono,
    },
    ConsoleEffect {
        slot: "limiter",
        effect: "miso.true-peak-limiter",
        post: true,
        link_mode: LinkMode::Maximum,
    },
];

fn param(parameter_id: u32, unit: ParameterUnit, value: f32) -> EffectParam {
    EffectParam {
        parameter_id,
        channel: ParameterChannel::Both,
        unit,
        value,
    }
}

fn param_on(
    parameter_id: u32,
    channel: ParameterChannel,
    unit: ParameterUnit,
    value: f32,
) -> EffectParam {
    EffectParam {
        channel,
        ..param(parameter_id, unit, value)
    }
}

/// Strip `index`'s knobs for console slot `slot`: every strip differs, so a lane carried to the
/// wrong strip is heard. `mono` makes the two channels' values equal, so the chain can collapse.
fn console_params(slot: &str, index: usize, mono: bool) -> Vec<EffectParam> {
    use ParameterChannel::{Left, Right};
    use ParameterUnit::{Db, Hz, Linear, Milliseconds, Ratio};
    let step = index as f32;
    match slot {
        "eq" => vec![
            param(1, Linear, 1.0),
            param(2, Linear, 1.0),
            param_on(3, Left, Hz, 90.0 + 10.0 * step),
            param_on(
                3,
                Right,
                Hz,
                if mono { 90.0 + 10.0 * step } else { 2_400.0 },
            ),
            param_on(4, Left, Db, 5.0 - 0.5 * step),
            param_on(4, Right, Db, if mono { 5.0 - 0.5 * step } else { -4.0 }),
            param(5, Ratio, 0.7),
            param(6, Ratio, 1.0),
        ],
        "comp" => vec![
            param(1, Db, -14.0 - 0.5 * step),
            param(2, Ratio, 4.0),
            param(3, Db, 3.0),
            param(4, Milliseconds, 2.0),
            param(5, Milliseconds, 40.0),
            param(6, Db, 0.0),
            param(7, Linear, 1.0),
        ],
        "gate" => vec![
            param(1, Db, -14.0 - 0.25 * step),
            param(2, Ratio, 4.0),
            param(3, Db, 30.0),
            param(4, Db, 3.0),
            param(5, Milliseconds, 1.0),
            param(6, Milliseconds, 5.0),
            param(7, Milliseconds, 20.0),
        ],
        "transient" => vec![
            param(1, Linear, 0.6 - 0.05 * step),
            param(2, Linear, -0.4),
            param(3, Linear, 1.0),
        ],
        "clip" => vec![
            param(1, Db, 9.0 + 0.5 * step),
            param(2, Db, -3.0),
            param(3, Linear, 1.0),
        ],
        "limiter" => vec![
            param(1, Db, -8.0 - 0.25 * step),
            param(2, Milliseconds, 60.0),
            param(3, Milliseconds, 5.0),
        ],
        other => panic!("no parameters for slot {other}"),
    }
}

fn console_slot_declaration(effect: &ConsoleEffect) -> ConsoleSlot {
    ConsoleSlot {
        slot: id(effect.slot),
        identity: EffectIdentity::Native {
            effect_id: id(effect.effect),
        },
        quality: EffectQuality::Normal,
        link_mode: effect.link_mode,
    }
}

/// [`filtered_session`] with the console slots named in `slots` (in [`CONSOLE_EFFECTS`] order)
/// declared and every track configured for them.
fn console_session(mono: bool, slots: &[&str]) -> SessionModel {
    let mut model = filtered_session(mono);
    let chosen: Vec<&ConsoleEffect> = CONSOLE_EFFECTS
        .iter()
        .filter(|effect| slots.contains(&effect.slot))
        .collect();
    assert_eq!(chosen.len(), slots.len(), "every named slot exists");
    model.console.pre_insert = chosen
        .iter()
        .filter(|effect| !effect.post)
        .map(|effect| console_slot_declaration(effect))
        .collect();
    model.console.post_insert = chosen
        .iter()
        .filter(|effect| effect.post)
        .map(|effect| console_slot_declaration(effect))
        .collect();
    for (index, track) in model.tracks.iter_mut().enumerate() {
        track.console = chosen
            .iter()
            .map(|effect| ConsoleEntry {
                slot: id(effect.slot),
                bypass: false,
                link_mode: session::ConsoleLinkMode::Slot,
                params: console_params(effect.slot, index, mono),
            })
            .collect();
    }
    model
}

/// [`Feed::new`] with an envelope: loud over frames 0..300, 500..740 and from 1100, quiet (a
/// fiftieth of the level) between, so a gate opens and closes and a dynamics effect has an
/// envelope in flight when the swap block renders (frame 768). The gate is in its release at the
/// swap, not fully open, so its state there differs from its state at rest.
fn bursty_feed(id: &'static str, seed: u32) -> Feed {
    let mut feed = Feed::new(id, seed);
    for plane in &mut feed.planes {
        for (frame, sample) in plane.iter_mut().enumerate() {
            if !(frame < 300 || (500..740).contains(&frame) || frame >= 1100) {
                *sample *= 0.02;
            }
        }
    }
    feed
}

const ALL_SLOTS: [&str; 6] = ["eq", "comp", "gate", "transient", "clip", "limiter"];

/// What a plain run (plans prepared with no live controls, the successor adopting its predecessor
/// itself) hands back.
struct PlainRun {
    blocks: Vec<Vec<u32>>,
    carry: CarryOutcome,
    restarted: Vec<Box<str>>,
    /// `(allocations, reallocations, deallocations)` and the render audit's `(allocations,
    /// deallocations)` over the hand-over plus the successor's first block.
    swap_allocator: ((u64, u64, u64), (u64, u64)),
    /// `bank_collapse_counters()` of the predecessor at the swap and of the last plan at the end.
    collapses: [[u64; 2]; 2],
    /// `bank_collapse_transitions()` of the last plan at the end.
    transitions: [u64; 3],
    /// The successor's `carry_refusal_count()` after the last block (D6).
    refusals: u64,
    /// The successor's carry-table entries ([`carry_counts`]).
    carry_counts: [usize; 4],
    /// Effect lanes the predecessor's inventory records.
    effect_lanes: usize,
}

fn plain_render(prepared: &mut host_core::PreparedHost, output: &mut [f32]) {
    let sample = prepared.plan.next_absolute_sample();
    prepared
        .plan
        .render_contiguous(
            engine::realtime::RenderIo {
                output: engine::realtime::PlanarBufferMut::try_new(output, 2, QUANTUM, QUANTUM)
                    .expect("output"),
            },
            sample,
        )
        .unwrap_or_else(|error| panic!("render: {error:?}"));
}

/// Render `a` for [`SWAP_BLOCK`] blocks fed `feed`, then -- when `b` is given -- prepare it as
/// `a`'s successor from `committed`, hand over, and render to [`BLOCKS`]. `successor_dual` forces
/// the successor's mono collapse off. Without `b` this is the reference: `a` rendered from frame 0.
fn plain_run(
    a: &Session,
    b: Option<(&Session, &SessionModel)>,
    feed: &Feed,
    backend: Backend,
    successor_dual: bool,
) -> PlainRun {
    use bench_support::alloc as bench_alloc;
    let mut plan = a.prepare(backend);
    let effect_lanes = plan.inventory.effect_lane_count();
    let mut blocks = Vec::with_capacity(BLOCKS);
    let mut carry = CarryOutcome::NotRequested;
    let mut restarted = Vec::new();
    let mut swap_allocator = ((0, 0, 0), (0, 0));
    let mut collapses = [[0; 2]; 2];
    let mut carry_counts = [0; 4];
    feed.submit(&mut plan.sources, 48_000, 0);
    for block in 0..BLOCKS {
        if let (SWAP_BLOCK, Some((session, committed))) = (block, b) {
            collapses[0] = plan.plan.bank_collapse_counters();
            let mut successor = session
                .prepare_successor(&plan.inventory, committed, &caps(), backend)
                .unwrap_or_else(|failure| panic!("successor: {failure:?}"));
            successor.plan.force_mono_collapse_off(successor_dual);
            restarted = successor.restarted_strips().to_vec();
            carry_counts = self::carry_counts(&mut successor);
            assert_eq!(successor.sources.adopt_persisting(&mut plan.sources), 1);
            if block + 1 < BLOCKS {
                feed.submit(&mut successor.sources, 48_000, block + 1);
            }
            let mut output = vec![f32::NAN; QUANTUM * 2];
            engine::realtime::audit::reset();
            let mark = bench_alloc::current_thread_counters();
            carry = successor.plan.adopt_predecessor_plan(&mut plan.plan);
            plain_render(&mut successor, &mut output);
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
            feed.submit(&mut plan.sources, 48_000, block + 1);
        }
        let mut output = vec![f32::NAN; QUANTUM * 2];
        plain_render(&mut plan, &mut output);
        blocks.push(output.iter().map(|sample| sample.to_bits()).collect());
    }
    collapses[1] = plan.plan.bank_collapse_counters();
    PlainRun {
        blocks,
        carry,
        restarted,
        swap_allocator,
        collapses,
        transitions: plan.plan.bank_collapse_transitions(),
        refusals: graph::carry_refusal_count(&mut plan.plan).expect("a graph plan"),
        carry_counts,
        effect_lanes,
    }
}

fn first_plain_difference(left: &PlainRun, right: &PlainRun) -> Option<usize> {
    assert_eq!(left.blocks.len(), right.blocks.len());
    left.blocks
        .iter()
        .zip(&right.blocks)
        .position(|(left, right)| left != right)
}

/// Gate 1: nine tracks with all six console effects, each doing work on the test signal and each
/// track configured differently; B adds a muted track whose ID sorts first, so every lane shifts
/// and two cross into the next bank. Every block of the swapped run equals a fresh B fed from
/// frame 0, nothing restarts, and no restore is refused.
fn console_effect_state_survives_a_swap(backend: Backend) {
    let a = Session::compile(console_session(false, &ALL_SLOTS));
    let b = Session::compile(with_muted_track(console_session(false, &ALL_SLOTS)));
    let feed = bursty_feed(SOURCE, 5);
    let reference = plain_run(&b, None, &feed, backend, false);
    let run = plain_run(&a, Some((&b, &a.model)), &feed, backend, false);
    assert_eq!(
        run.carry,
        CarryOutcome::Carried,
        "{backend:?}: the swap block"
    );
    assert_eq!(
        first_plain_difference(&run, &reference),
        None,
        "{backend:?}: every block equals the post-edit session rendered from frame 0"
    );
    assert!(reference.blocks[SWAP_BLOCK].iter().any(|bits| *bits != 0));
    assert_eq!(run.restarted, Vec::<Box<str>>::new(), "{backend:?}");
    assert_eq!(run.refusals, 0, "{backend:?}: no restore was refused");
    // Not vacuous: six console effects on nine strips are 54 recorded lanes.
    assert_eq!(run.effect_lanes, 54, "{backend:?}");
}

/// At the build's own width, `Backend::current()`: a console effect bank binds only at the width
/// its build executes (#1017).
#[test]
fn console_effect_state_survives_a_swap_at_the_build_width() {
    console_effect_state_survives_a_swap(Backend::current());
}

/// For each of the six effects, alone on every strip, the state it holds at the swap block is
/// heard: the swapped run equals a fresh B fed from frame 0 (its state carried), and differs from
/// a B that starts the slot at rest at the swap block on the same PCM (a lane it fails to carry
/// would render as that). With the input section flat, the slot is the only state in the strip, so
/// the second oracle is exact. An effect whose state at the swap equals its state at rest -- an
/// open gate, a feed that never moves it -- fails here by name, and not silently in gate 1.
fn every_console_effect_does_work(backend: Backend) {
    let feed = bursty_feed(SOURCE, 5);
    let from_swap = spliced(SOURCE, &feed, |block| {
        (block + SWAP_BLOCK < BLOCKS).then_some(block + SWAP_BLOCK)
    });
    for slot in ALL_SLOTS {
        let a_model = unfiltered(console_session(false, &[slot]));
        let a = Session::compile(a_model.clone());
        let b = Session::compile(with_muted_track(a_model));
        let reference = plain_run(&b, None, &feed, backend, false);
        let at_rest = plain_run(&b, None, &from_swap, backend, false);
        let run = plain_run(&a, Some((&b, &a.model)), &feed, backend, false);
        assert_eq!(run.carry, CarryOutcome::Carried, "{backend:?}: {slot}");
        assert_eq!(run.refusals, 0, "{backend:?}: {slot}");
        assert_eq!(
            first_plain_difference(&run, &reference),
            None,
            "{backend:?}: {slot} carried equals a fresh B from frame 0"
        );
        assert_ne!(
            run.blocks[SWAP_BLOCK..],
            at_rest.blocks[..BLOCKS - SWAP_BLOCK],
            "{backend:?}: {slot} at the swap holds state its rest does not"
        );
    }
}

/// At the build's own width, `Backend::current()`: a console effect bank binds only at the width
/// its build executes (#1017).
#[test]
fn every_console_effect_does_work_at_the_build_width() {
    every_console_effect_does_work(Backend::current());
}

/// Gate 4: the swap block of gate 1, with every console effect carried, allocates and frees
/// nothing: not on the allocator and not in the engine's render audit.
fn the_effect_carry_allocates_and_frees_nothing(backend: Backend) {
    let a = Session::compile(console_session(false, &ALL_SLOTS));
    let b = Session::compile(with_muted_track(console_session(false, &ALL_SLOTS)));
    let feed = bursty_feed(SOURCE, 5);
    let run = plain_run(&a, Some((&b, &a.model)), &feed, backend, false);
    assert_eq!(run.carry, CarryOutcome::Carried, "{backend:?}");
    assert_eq!(run.effect_lanes, 54, "{backend:?}: every lane is carried");
    assert_eq!(
        run.swap_allocator,
        ((0, 0, 0), (0, 0)),
        "{backend:?}: allocator and render audit over the swap block"
    );
}

/// At the build's own width, `Backend::current()`: a console effect bank binds only at the width
/// its build executes (#1017).
#[test]
fn the_effect_carry_allocates_and_frees_nothing_at_the_build_width() {
    the_effect_carry_allocates_and_frees_nothing(Backend::current());
}

/// `model` with every track's input section flat: no filters, no trim, no polarity inversion.
fn unfiltered(mut model: SessionModel) -> SessionModel {
    for track in &mut model.tracks {
        for channel in [&mut track.builtins.left, &mut track.builtins.right] {
            channel.hpf_hz = 0.0;
            channel.lpf_hz = 0.0;
            channel.trim_db = 0.0;
            channel.polarity_invert = false;
        }
    }
    model
}

/// `model` with every track's trim set to `-0.0` on both channels: a bit pattern that differs from
/// the `+0.0` of an [`unfiltered`] session, and so an input section that does not carry (#1276
/// D1), with the same gain.
fn with_negative_zero_trims(mut model: SessionModel) -> SessionModel {
    for track in &mut model.tracks {
        for channel in [&mut track.builtins.left, &mut track.builtins.right] {
            channel.trim_db = -0.0;
        }
    }
    model
}

/// Gate 2: a mono-source session whose console is the collapse-capable subset (EQ, compressor,
/// limiter; the gate/expander, soft-clip and transient shaper declare no channel symmetry, so a
/// chain that holds one never collapses). Its chains collapse before the swap and, because the
/// carry disengages them and the successor inherits their agreement, on every block after it, with
/// no agreement proof; with the successor's collapse forced off the carried lanes' right channels
/// must hold the state the collapsed predecessor never wrote there. Output is bit-identical to a
/// fresh B either way.
///
/// With `inputs_restart`, B also changes every track's trim to `-0.0` -- the same gain, a different
/// bit pattern -- so no input section carries and the effect carry alone disengages the
/// predecessor's collapsed chains.
fn console_chains_keep_collapsing(backend: Backend, successor_dual: bool, inputs_restart: bool) {
    const SLOTS: [&str; 3] = ["eq", "comp", "limiter"];
    let after = |model: SessionModel| {
        if inputs_restart {
            with_negative_zero_trims(model)
        } else {
            model
        }
    };
    let base = || unfiltered(console_session(true, &SLOTS));
    let a = Session::compile(base());
    let b = Session::compile(after(with_muted_track(base())));
    let feed = bursty_feed(SOURCE, 9);
    let reference = plain_run(&b, None, &feed, backend, false);
    let run = plain_run(&a, Some((&b, &a.model)), &feed, backend, successor_dual);
    assert_eq!(run.carry, CarryOutcome::Carried, "{backend:?}");
    assert!(
        run.collapses[0][0] > 0,
        "{backend:?}: the predecessor rendered collapsed before the swap"
    );
    if inputs_restart {
        // Not vacuous: the nine input sections did not carry.
        let kept = Session::compile(with_muted_track(base()));
        let carried = plain_run(&a, Some((&kept, &a.model)), &feed, backend, false);
        assert_eq!(
            carried.carry_counts[1] - run.carry_counts[1],
            9,
            "{backend:?}"
        );
    }
    assert_eq!(
        first_plain_difference(&run, &reference),
        None,
        "{backend:?}, successor dual {successor_dual}, inputs restart {inputs_restart}"
    );
    assert_eq!(run.refusals, 0, "{backend:?}");
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

/// At the build's own width, `Backend::current()`: a console effect bank binds only at the width
/// its build executes (#1017).
#[test]
fn console_chains_keep_collapsing_across_a_swap_at_the_build_width() {
    let backend = Backend::current();
    for inputs_restart in [false, true] {
        console_chains_keep_collapsing(backend, false, inputs_restart);
        console_chains_keep_collapsing(backend, true, inputs_restart);
    }
}

/// `model` with every track's right channel read from the source's right channel: a stereo-source
/// session, whose strips cannot collapse.
fn stereo_source(mut model: SessionModel) -> SessionModel {
    for track in &mut model.tracks {
        track.right_source_channel = 1;
    }
    model
}

/// The AND rule of the agreement flag (#1276 D5, #1279 D3): a chain that could not collapse hands
/// its carried lanes' state over without a claim that the two channels agree, because they were fed
/// different PCM. B maps every track mono, so its chains can collapse, and changes every trim to
/// `-0.0` so that no input section carries and the effect carry alone speaks for the chain. The
/// successor must stay dual -- and render exactly what a successor forced dual renders -- instead
/// of collapsing onto a left channel its carried right state no longer matches.
fn a_chain_that_could_not_collapse_hands_over_no_agreement(backend: Backend) {
    const SLOTS: [&str; 3] = ["eq", "comp", "limiter"];
    let base = || unfiltered(console_session(true, &SLOTS));
    let a = Session::compile(stereo_source(base()));
    let b = Session::compile(with_negative_zero_trims(base()));
    let feed = bursty_feed(SOURCE, 9);
    let free = plain_run(&a, Some((&b, &a.model)), &feed, backend, false);
    let dual = plain_run(&a, Some((&b, &a.model)), &feed, backend, true);
    let mono = plain_run(&b, None, &feed, backend, false);
    assert_eq!(free.carry, CarryOutcome::Carried, "{backend:?}");
    assert_eq!(
        free.collapses[0][0], 0,
        "{backend:?}: the predecessor could not collapse"
    );
    assert!(
        mono.collapses[1][0] > 0,
        "{backend:?}: the successor's chains could"
    );
    assert_eq!(
        first_plain_difference(&free, &dual),
        None,
        "{backend:?}: the successor does not collapse onto a left channel it does not match"
    );
    assert_eq!(free.collapses[1][0], 0, "{backend:?}: it stays dual");
    assert_eq!(free.refusals, 0, "{backend:?}");
    // The claim, not the bits: the channels of a mirrored source agree in the predecessor too, so
    // collapsing would be sound. The chain still hands over no claim of agreement, because it
    // never kept the flag the claim rests on, and the successor stays dual (every launch effect
    // declines the proof that would bring a collapse back).
    let mut mirrored = bursty_feed(SOURCE, 9);
    mirrored.planes[1] = mirrored.planes[0].clone();
    let run = plain_run(&a, Some((&b, &a.model)), &mirrored, backend, false);
    assert_eq!(run.collapses[1][0], 0, "{backend:?}: a mirrored source");
}

/// At the build's own width, `Backend::current()`: a console effect bank binds only at the width
/// its build executes (#1017).
#[test]
fn a_chain_that_could_not_collapse_hands_over_no_agreement_at_the_build_width() {
    a_chain_that_could_not_collapse_hands_over_no_agreement(Backend::current());
}

/// [`console_session`] with the input section's filters, trim and polarity off, so nothing
/// before the console holds state, and every strip's matrix routes it to one output plane: `solo`
/// to the left plane, every other strip to the right. A plane then depends on those strips only.
fn isolated_session(slots: &[&str], solo: &str) -> SessionModel {
    let mut model = unfiltered(console_session(false, slots));
    for track in &mut model.tracks {
        track.matrix_or_pan = if track.id.as_str() == solo {
            MatrixOrPan::Matrix {
                ll: 1.0,
                lr: 0.0,
                rl: 0.0,
                rr: 0.0,
                smoothing_samples: 16,
            }
        } else {
            MatrixOrPan::Matrix {
                ll: 0.0,
                lr: 0.0,
                rl: 0.0,
                rr: 1.0,
                smoothing_samples: 16,
            }
        };
    }
    model
}

/// `model` with `track`'s `value` of parameter `parameter_id` of console slot `slot` replaced.
fn with_console_param(
    mut model: SessionModel,
    track: &str,
    slot: &str,
    parameter_id: u32,
    value: f32,
) -> SessionModel {
    let entry = track_mut(&mut model, track)
        .console
        .iter_mut()
        .find(|entry| entry.slot.as_str() == slot)
        .expect("console slot");
    entry
        .params
        .iter_mut()
        .find(|param| param.parameter_id == parameter_id)
        .expect("parameter")
        .value = value;
    model
}

/// One plane (`0` left, `1` right) of every block of a run.
fn plane_of(run: &PlainRun, plane: usize) -> Vec<&[u32]> {
    run.blocks
        .iter()
        .map(|block| &block[plane * QUANTUM..(plane + 1) * QUANTUM])
        .collect()
}

/// Gate 3, first case. B changes the compressor threshold of strip `eq3` -- a prepared value, since
/// an effect bank stage has no live lane -- and adds a muted track. `restarted_strips()` is exactly
/// `eq3`, so every slot of `eq3` restarts with it (#1324 D2) and every other strip's lane carries:
///
/// * the right plane, which the other strips alone feed, equals a fresh B on every block: their
///   lanes carried;
/// * the carry program holds exactly `eq3`'s slots fewer moves than the same swap without the
///   threshold change;
/// * the left plane, which `eq3` alone feeds, differs from the swap that carries the old
///   threshold: the slot restarted at the new one.
fn a_prepared_change_to_one_slot_restarts_only_its_strip(backend: Backend, slots: &[&str]) {
    let a_model = isolated_session(slots, "eq3");
    let changed = with_console_param(with_muted_track(a_model.clone()), "eq3", "comp", 1, -30.0);
    let a = Session::compile(a_model.clone());
    let b = Session::compile(changed);
    let unchanged = Session::compile(with_muted_track(a_model));
    let feed = bursty_feed(SOURCE, 5);
    let reference = plain_run(&b, None, &feed, backend, false);
    let run = plain_run(&a, Some((&b, &a.model)), &feed, backend, false);
    let carried_all = plain_run(&a, Some((&unchanged, &a.model)), &feed, backend, false);
    assert_eq!(run.carry, CarryOutcome::Carried, "{backend:?}");
    assert_eq!(run.restarted, vec![Box::<str>::from("eq3")], "{backend:?}");
    assert_eq!(carried_all.restarted, Vec::<Box<str>>::new(), "{backend:?}");
    assert_eq!(
        carried_all.carry_counts[3] - run.carry_counts[3],
        slots.len(),
        "{backend:?}: none of eq3's slots moves"
    );
    assert_eq!(
        plane_of(&run, 1),
        plane_of(&reference, 1),
        "{backend:?}: the other strips are unmoved"
    );
    assert_ne!(
        plane_of(&run, 0)[SWAP_BLOCK..],
        plane_of(&carried_all, 0)[SWAP_BLOCK..],
        "{backend:?}: eq3 restarted at the new threshold"
    );
    assert_eq!(run.refusals, 0, "{backend:?}");
}

/// The first case with a lone compressor slot, where the restarted slot has an exact oracle: the
/// left plane after the swap is what a fresh B renders from rest on the same PCM, the compressor at
/// the new threshold with no history. A rule that carries the slot despite its changed threshold
/// restores the old threshold's designed words and envelope, and differs.
fn a_restarted_slot_starts_at_rest_at_its_new_value(backend: Backend) {
    let a_model = isolated_session(&["comp"], "eq3");
    let changed = with_console_param(with_muted_track(a_model.clone()), "eq3", "comp", 1, -30.0);
    let a = Session::compile(a_model);
    let b = Session::compile(changed);
    let feed = bursty_feed(SOURCE, 5);
    let from_swap = spliced(SOURCE, &feed, |block| {
        (block + SWAP_BLOCK < BLOCKS).then_some(block + SWAP_BLOCK)
    });
    let at_rest = plain_run(&b, None, &from_swap, backend, false);
    let run = plain_run(&a, Some((&b, &a.model)), &feed, backend, false);
    assert_eq!(run.restarted, vec![Box::<str>::from("eq3")], "{backend:?}");
    assert_eq!(
        plane_of(&run, 0)[SWAP_BLOCK..],
        plane_of(&at_rest, 0)[..BLOCKS - SWAP_BLOCK],
        "{backend:?}: eq3's compressor starts at rest"
    );
    // Not vacuous: the new threshold is heard.
    let carried_old = plain_run(
        &a,
        Some((
            &Session::compile(with_muted_track(isolated_session(&["comp"], "eq3"))),
            &a.model,
        )),
        &feed,
        backend,
        false,
    );
    assert_ne!(
        plane_of(&run, 0)[SWAP_BLOCK..],
        plane_of(&carried_old, 0)[SWAP_BLOCK..],
        "{backend:?}"
    );
}

/// At the build's own width, `Backend::current()`: a console effect bank binds only at the width
/// its build executes (#1017).
#[test]
fn a_prepared_change_to_one_slot_restarts_only_its_strip_at_the_build_width() {
    let backend = Backend::current();
    a_prepared_change_to_one_slot_restarts_only_its_strip(backend, &ALL_SLOTS);
    a_restarted_slot_starts_at_rest_at_its_new_value(backend);
}

/// A compressor insert that changes no sample: threshold at 0 dB and ratio 1.
fn transparent_insert() -> Effect {
    Effect {
        id: id("tc"),
        identity: EffectIdentity::Native {
            effect_id: id("miso.compressor"),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        params: vec![
            param(1, ParameterUnit::Db, 0.0),
            param(2, ParameterUnit::Ratio, 1.0),
            param(3, ParameterUnit::Db, 0.0),
            param(4, ParameterUnit::Milliseconds, 2.0),
            param(5, ParameterUnit::Milliseconds, 40.0),
            param(6, ParameterUnit::Db, 0.0),
            param(7, ParameterUnit::Linear, 1.0),
        ],
        sidechain: SidechainDeclaration::None,
    }
}

/// An insert carries when it is a lane of a bank in both plans (#1280a), and the compressor insert
/// here is not: it sits on one of nine strips, below the padding threshold at every width (#888:
/// 2 members at 4 lanes, 3 at 8), so it runs per node, and a per-node instance carries only with
/// live controls (#1282), which a plain plan has not. A strip whose insert survives the swap but
/// is not banked restarts (with every owner before its fader, #1324 D2) and is in
/// `restarted_strips()`, while a strip without an insert is not. A structural swap adds a muted
/// track to a session whose `eq3` holds an unchanged insert: the list is exactly `eq3`.
fn a_surviving_insert_restarts_its_strip(backend: Backend) {
    let mut with_inserts = console_session(false, &ALL_SLOTS);
    track_mut(&mut with_inserts, "eq3")
        .inserts
        .effects
        .push(transparent_insert());
    let b_model = with_muted_track(with_inserts.clone());
    let a = Session::compile(with_inserts);
    let b = Session::compile(b_model);
    let feed = bursty_feed(SOURCE, 5);
    let run = plain_run(&a, Some((&b, &a.model)), &feed, backend, false);
    assert_eq!(run.carry, CarryOutcome::Carried, "{backend:?}");
    assert_eq!(
        run.restarted,
        vec![Box::<str>::from("eq3")],
        "{backend:?}: the strip with a surviving insert, and no other"
    );
    assert_eq!(run.refusals, 0, "{backend:?}");
}

/// At the build's own width, `Backend::current()`: a console effect bank binds only at the width
/// its build executes (#1017).
#[test]
fn a_surviving_insert_restarts_its_strip_at_the_build_width() {
    a_surviving_insert_restarts_its_strip(Backend::current());
}

/// `model` without console slot `slot`: its declaration and every strip's entry.
fn without_console_slot(mut model: SessionModel, slot: &str) -> SessionModel {
    model
        .console
        .pre_insert
        .retain(|declared| declared.slot.as_str() != slot);
    model
        .console
        .post_insert
        .retain(|declared| declared.slot.as_str() != slot);
    for track in &mut model.tracks {
        track.console.retain(|entry| entry.slot.as_str() != slot);
    }
    model
}

/// A session console slot removed changes every strip's chain: every strip restarts (D15-9), with
/// every slot (#1324 D2).
fn a_removed_console_slot_restarts_every_strip(backend: Backend) {
    let a = Session::compile(console_session(false, &ALL_SLOTS));
    let removed = Session::compile(without_console_slot(
        console_session(false, &ALL_SLOTS),
        "gate",
    ));
    let feed = bursty_feed(SOURCE, 5);
    let run = plain_run(&a, Some((&removed, &a.model)), &feed, backend, false);
    let kept = Session::compile(console_session(false, &ALL_SLOTS));
    let unchanged = plain_run(&a, Some((&kept, &a.model)), &feed, backend, false);
    assert_eq!(run.carry, CarryOutcome::Carried, "{backend:?}");
    let every_strip: Vec<Box<str>> = (0..9).map(|index| format!("eq{index}").into()).collect();
    assert_eq!(run.restarted, every_strip, "{backend:?}");
    assert_eq!(unchanged.restarted, Vec::<Box<str>>::new(), "{backend:?}");
    assert_eq!(run.refusals, 0, "{backend:?}");
}

/// At the build's own width, `Backend::current()`: a console effect bank binds only at the width
/// its build executes (#1017).
#[test]
fn a_removed_console_slot_restarts_every_strip_at_the_build_width() {
    a_removed_console_slot_restarts_every_strip(Backend::current());
}

// ---------------------------------------------------------------------------------------------
// Issue #1280a: a live-controlled effect lane keeps its state, its unread live values, its bypass
// crossfade and its dry shunt through a plan swap.
//
// A console slot or a banked insert of a plan prepared with live controls is a
// `LiveControlEffectBankStage` lane. Every case writes through the plan's own producers, stamped
// on the plan's own gate, and compares the swapped run with the post-edit session prepared fresh
// and fed the same values at the same blocks. These cases leave the sessions' values equal and
// move their values through the cells; the last case (#1280c) differs in live values and shows
// that the successor retargets the carried lane.

use effect_contract::{BypassSwitch, EffectControlRecord};
use host_core::LiveEffectAddress;

/// The parametric EQ's `band-1-gain`: parameter id 4, index 3 of its table.
const BAND_GAIN_INDEX: u32 = 3;

/// What an [`EffectWrite`] puts in a cell.
#[derive(Clone, Copy, Debug)]
enum EffectValue {
    /// A `PerLane` block-rate parameter, by index into the descriptor's table, set to one value
    /// on both channels (a left and a right cell).
    Parameter { index: u32, value: f32 },
    /// The parametric EQ's first band gain, on both channels (two prepared targets).
    BandGain(f32),
    /// The same band gain as one owner transaction for both channels, which is what a live commit
    /// of a session edit publishes.
    BandGainBoth(f32),
    /// The bypass cell with its crossfade ramp.
    Bypass(BypassSwitch),
    /// A live send's gain (#1284), as one record over `length` samples: the route's other values
    /// stay the identity matrix, unmuted, with no source lane muted. The write's `strip` and
    /// `address` are not read.
    SendGain {
        route: &'static str,
        gain_db: f32,
        length: u32,
    },
}

/// A value written to one effect's cells before block `block` renders.
#[derive(Clone, Copy, Debug)]
struct EffectWrite {
    block: usize,
    strip: &'static str,
    address: LiveEffectAddress,
    value: EffectValue,
}

/// One transaction on `plan`'s own gate: the value is written to the addressed effect's cells and
/// its revision is published.
fn write_effect_value(plan: &mut LivePlan, write: &EffectWrite) {
    let LivePlan { handles, gate, .. } = plan;
    if let EffectValue::SendGain {
        route,
        gain_db,
        length,
    } = write.value
    {
        let position = host_core::route_producer_position(&handles.route_controls, route)
            .unwrap_or_else(|| panic!("route {route} is a live send"));
        let producer = &mut handles.route_controls[position];
        gate.write_published(|stamp| {
            producer.set(
                stamp,
                gain_db,
                [1.0, 0.0, 0.0, 1.0],
                false,
                [false; 2],
                length,
            )
        })
        .expect("a send record the cell holds");
        return;
    }
    let producer = handles
        .effect_control_mut(write.strip, write.address)
        .expect("a control channel for the addressed effect");
    match write.value {
        EffectValue::Parameter { index, value } => {
            for channel in [
                effect_contract::ParameterChannel::Left,
                effect_contract::ParameterChannel::Right,
            ] {
                support::write_effect(
                    producer,
                    gate,
                    EffectControlRecord::Parameter {
                        parameter_index: index,
                        channel,
                        value,
                    },
                );
            }
        }
        EffectValue::BandGain(value) => {
            for channel in [
                effect_contract::ParameterChannel::Left,
                effect_contract::ParameterChannel::Right,
            ] {
                support::push_eq_parameter(producer, gate, BAND_GAIN_INDEX, channel, value);
            }
        }
        EffectValue::BandGainBoth(value) => support::push_eq_parameter(
            producer,
            gate,
            BAND_GAIN_INDEX,
            effect_contract::ParameterChannel::Both,
            value,
        ),
        EffectValue::SendGain { .. } => unreachable!("written above"),
        EffectValue::Bypass(switch) => {
            producer
                .preflight(EffectControlRecord::Bypass(switch))
                .expect("a ramp the cell holds");
            let revision = gate.last_published() + 1;
            let stamp = gate.stamp(revision).expect("a legal revision");
            producer
                .write(stamp, EffectControlRecord::Bypass(switch))
                .expect("preflighted");
            gate.publish(revision);
        }
    }
}

/// What an [`effect_run`] hands back.
struct EffectRun {
    blocks: Vec<Vec<u32>>,
    carry: CarryOutcome,
    /// The successor's `restarted_strips()`.
    restarted: Vec<Box<str>>,
    /// `(allocations, reallocations, deallocations)` and the render audit's `(allocations,
    /// deallocations)` over the hand-over plus the successor's first block.
    swap_allocator: ((u64, u64, u64), (u64, u64)),
    /// The `live_values_superseded` count of the plan that rendered the swap block, read just
    /// after it: the successor's own count, which counts for the retired plan's cells too (#1280
    /// D2).
    superseded: u64,
    /// The successor's `carry_refusal_count()` after the swap block.
    refusals: u64,
    /// Effect lanes the predecessor's inventory records.
    effect_lanes: usize,
    /// Bytes of effect state the swap block's carry passed through the payload scratch (#1282
    /// gate 4); an instance moved by pointer adds none.
    payload_bytes: u64,
}

/// Render `a` for [`SWAP_BLOCK`] blocks and write `writes` to the plan that renders their block
/// next; with `b`, prepare it as `a`'s successor at [`SWAP_BLOCK`], write `successor_writes` to
/// its cells, hand over and render on. Without `b` this is the reference run: `a` rendered from
/// frame 0 to [`BLOCKS`], fed `writes` and `successor_writes` at the same blocks.
fn effect_run(
    a: &Session,
    b: Option<(&Session, &SessionModel)>,
    feed: &Feed,
    writes: &[EffectWrite],
    successor_writes: &[EffectWrite],
    backend: Backend,
) -> EffectRun {
    effect_run_blocks(a, b, feed, writes, successor_writes, backend, BLOCKS)
}

/// [`effect_run`] over `blocks` blocks; `feed` holds at least that many.
fn effect_run_blocks(
    a: &Session,
    b: Option<(&Session, &SessionModel)>,
    feed: &Feed,
    writes: &[EffectWrite],
    successor_writes: &[EffectWrite],
    backend: Backend,
    blocks: usize,
) -> EffectRun {
    use bench_support::alloc as bench_alloc;
    let quantum = a.model.quantum_frames as usize;
    let mut plan = LivePlan::fresh(a, backend);
    let effect_lanes = plan.prepared.inventory.effect_lane_count();
    let mut rendered = Vec::with_capacity(blocks);
    let mut carry = CarryOutcome::NotRequested;
    let mut restarted = Vec::new();
    let mut swap_allocator = ((0, 0, 0), (0, 0));
    let mut superseded = 0;
    let mut refusals = 0;
    let mut payload_bytes = 0;
    plan.submit(feed, quantum, 0);
    for block in 0..blocks {
        for write in writes.iter().filter(|write| write.block == block) {
            write_effect_value(&mut plan, write);
        }
        if let (SWAP_BLOCK, Some((session, committed))) = (block, b) {
            let mut successor =
                LivePlan::successor(session, &plan.prepared.inventory, committed, backend);
            restarted = successor.prepared.restarted_strips().to_vec();
            for write in successor_writes {
                write_effect_value(&mut successor, write);
            }
            assert_eq!(
                successor
                    .prepared
                    .sources
                    .adopt_persisting(&mut plan.prepared.sources),
                1
            );
            if block + 1 < blocks {
                successor.submit(feed, quantum, block + 1);
            }
            let mut output = vec![f32::NAN; quantum * 2];
            engine::realtime::audit::reset();
            graph::test_only_reset_carry_payload_bytes();
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
            payload_bytes = graph::test_only_carry_payload_bytes();
            superseded = successor.handles.live_values_superseded.get();
            refusals =
                graph::carry_refusal_count(&mut successor.prepared.plan).expect("a graph plan");
            rendered.push(output.iter().map(|sample| sample.to_bits()).collect());
            plan = successor;
            continue;
        }
        if b.is_none() && block == SWAP_BLOCK {
            for write in successor_writes {
                write_effect_value(&mut plan, write);
            }
        }
        if block + 1 < blocks {
            plan.submit(feed, quantum, block + 1);
        }
        rendered.push(plan.render(quantum));
        if block == SWAP_BLOCK {
            superseded = plan.handles.live_values_superseded.get();
        }
    }
    EffectRun {
        blocks: rendered,
        carry,
        restarted,
        swap_allocator,
        superseded,
        refusals,
        effect_lanes,
        payload_bytes,
    }
}

fn first_effect_difference(left: &EffectRun, right: &EffectRun) -> Option<usize> {
    assert_eq!(left.blocks.len(), right.blocks.len());
    left.blocks
        .iter()
        .zip(&right.blocks)
        .position(|(left, right)| left != right)
}

/// The console slot addresses of [`ALL_SLOTS`].
const EQ_ADDRESS: LiveEffectAddress = LiveEffectAddress::console(0);
const COMP_ADDRESS: LiveEffectAddress = LiveEffectAddress::console(1);
/// The compressor's `threshold`: parameter id 1, index 0.
const THRESHOLD_INDEX: u32 = 0;

fn live_console_pair() -> (Session, Session) {
    let slots = ["eq", "comp"];
    (
        Session::compile(console_session(false, &slots)),
        Session::compile(with_muted_track(console_session(false, &slots))),
    )
}

/// Gate 1: nine tracks with a live console EQ and compressor. A value is rendered in A, and a
/// different one is written to A's cells just before the swap and not rendered. B is the same
/// session plus a muted track, so every lane moves. Every block of the swapped run equals B
/// prepared fresh and fed the same values at the same blocks, nothing restarts, and no restore is
/// refused.
fn unrendered_live_writes_survive_the_swap(backend: Backend, address: LiveEffectAddress) {
    let (a, b) = live_console_pair();
    let feed = bursty_feed(SOURCE, 5);
    let value = |level: f32| match address {
        COMP_ADDRESS => EffectValue::Parameter {
            index: THRESHOLD_INDEX,
            value: level,
        },
        _ => EffectValue::BandGain(level),
    };
    let (rendered, unread) = match address {
        COMP_ADDRESS => (-30.0, -5.0),
        _ => (-6.0, 3.0),
    };
    let writes = |unread_too: bool| {
        let mut writes = Vec::new();
        for strip in ["eq1", "eq4", "eq8"] {
            writes.push(EffectWrite {
                block: 3,
                strip,
                address,
                value: value(rendered),
            });
            if unread_too {
                writes.push(EffectWrite {
                    block: SWAP_BLOCK,
                    strip,
                    address,
                    value: value(unread),
                });
            }
        }
        writes
    };
    let reference = effect_run(&b, None, &feed, &writes(true), &[], backend);
    let run = effect_run(&a, Some((&b, &a.model)), &feed, &writes(true), &[], backend);
    assert_eq!(
        run.carry,
        CarryOutcome::Carried,
        "{backend:?} {address:?}: the swap block"
    );
    assert_eq!(
        first_effect_difference(&run, &reference),
        None,
        "{backend:?} {address:?}: every block equals the post-edit session fed the same values"
    );
    assert_eq!(
        run.restarted,
        Vec::<Box<str>>::new(),
        "{backend:?} {address:?}"
    );
    assert_eq!(run.refusals, 0, "{backend:?} {address:?}");
    // Not vacuous: the value that stays unread in A at the swap changes B's output, so a carry
    // that loses it (or applies the value A rendered) renders a different block.
    let lost = effect_run(&b, None, &feed, &writes(false), &[], backend);
    assert_ne!(
        lost.blocks[SWAP_BLOCK], reference.blocks[SWAP_BLOCK],
        "{backend:?} {address:?}: the unread value is audible in the swap block"
    );
}

/// At the build's own width, `Backend::current()`: a console effect bank binds only at the width
/// its build executes (#1017).
#[test]
fn unrendered_live_writes_survive_the_swap_at_the_build_width() {
    let backend = Backend::current();
    unrendered_live_writes_survive_the_swap(backend, EQ_ADDRESS);
    unrendered_live_writes_survive_the_swap(backend, COMP_ADDRESS);
}

/// Every swap block of the gates below makes no allocator call and no render-audit allocation or
/// free (gate 6), and the carry refuses no restore.
fn assert_clean_swap(run: &EffectRun, what: &str) {
    assert_eq!(run.carry, CarryOutcome::Carried, "{what}: the swap block");
    assert_eq!(
        run.swap_allocator,
        ((0, 0, 0), (0, 0)),
        "{what}: allocator and render audit over the swap block"
    );
    assert_eq!(run.refusals, 0, "{what}: no restore was refused");
}

/// Gate 2: at quantum 32 a compressor threshold change written at block 5 ramps over 64 samples,
/// so the ramp is half done at the swap block and finishes in its first block. A carry that
/// re-derives the ramp, or that applies the written value as a step, moves bits there.
fn a_live_ramp_in_flight_finishes_after_the_swap(backend: Backend) {
    let slots = ["eq", "comp"];
    let a = Session::compile(at_quantum(console_session(false, &slots), 32));
    let b = Session::compile(at_quantum(
        with_muted_track(console_session(false, &slots)),
        32,
    ));
    let feed = bursty_feed(SOURCE, 5);
    let writes: Vec<EffectWrite> = ["eq1", "eq4", "eq8"]
        .into_iter()
        .map(|strip| EffectWrite {
            block: SWAP_BLOCK - 1,
            strip,
            address: COMP_ADDRESS,
            value: EffectValue::Parameter {
                index: THRESHOLD_INDEX,
                value: -40.0,
            },
        })
        .collect();
    let reference = effect_run(&b, None, &feed, &writes, &[], backend);
    let run = effect_run(&a, Some((&b, &a.model)), &feed, &writes, &[], backend);
    assert_clean_swap(&run, &format!("{backend:?}"));
    assert_eq!(
        first_effect_difference(&run, &reference),
        None,
        "{backend:?}: every block equals the post-edit session fed the same value"
    );
    assert_eq!(run.restarted, Vec::<Box<str>>::new(), "{backend:?}");
    // Not vacuous: the threshold change is audible in the blocks the ramp spans.
    let unchanged = effect_run(&b, None, &feed, &[], &[], backend);
    assert_ne!(
        unchanged.blocks[SWAP_BLOCK], reference.blocks[SWAP_BLOCK],
        "{backend:?}: the ramp is in flight in the swap block"
    );
}

/// At the build's own width, `Backend::current()`: a console effect bank binds only at the width
/// its build executes (#1017).
#[test]
fn a_live_ramp_in_flight_finishes_after_the_swap_at_the_build_width() {
    a_live_ramp_in_flight_finishes_after_the_swap(Backend::current());
}

/// Gate 3: 40 values of one compressor parameter are written to A's cells after A's last block,
/// the successor is published, and 40 more are written to its cells. Every write succeeds. The
/// successor's first block equals a twin that was written only the last value, and the count
/// grows by 40 for A's cell and 39 for the successor's own, per cell (the parameter has a left
/// and a right cell). With nothing written to the successor, the carried value applies and the
/// count grows by 39 per cell.
fn the_newest_value_wins_and_the_count_is_exact(backend: Backend) {
    let (a, b) = live_console_pair();
    let feed = bursty_feed(SOURCE, 5);
    let strip = "eq4";
    let thresholds =
        |first: f32| -> Vec<f32> { (0..40).map(|i| first - 0.25 * i as f32).collect() };
    let write = |block: usize, value: f32| EffectWrite {
        block,
        strip,
        address: COMP_ADDRESS,
        value: EffectValue::Parameter {
            index: THRESHOLD_INDEX,
            value,
        },
    };
    let a_writes: Vec<EffectWrite> = thresholds(-20.0)
        .into_iter()
        .map(|value| write(SWAP_BLOCK, value))
        .collect();
    let b_writes: Vec<EffectWrite> = thresholds(-25.0)
        .into_iter()
        .map(|value| write(SWAP_BLOCK, value))
        .collect();
    let cells = 2_u64;
    // Both written: the successor's newer values win.
    let twin = effect_run(&b, None, &feed, &[], &b_writes[39..], backend);
    let run = effect_run(
        &a,
        Some((&b, &a.model)),
        &feed,
        &a_writes,
        &b_writes,
        backend,
    );
    assert_clean_swap(&run, &format!("{backend:?} both"));
    assert_eq!(
        first_effect_difference(&run, &twin),
        None,
        "{backend:?}: the successor's newest value wins"
    );
    assert_eq!(twin.superseded, 0, "{backend:?}: the twin replaced nothing");
    assert_eq!(
        run.superseded,
        cells * (40 + 39),
        "{backend:?}: A's 40 carried values and 39 of the successor's own, per cell"
    );
    // Only A written: the carried value applies.
    let twin = effect_run(&b, None, &feed, &[], &a_writes[39..], backend);
    let run = effect_run(&a, Some((&b, &a.model)), &feed, &a_writes, &[], backend);
    assert_clean_swap(&run, &format!("{backend:?} carried"));
    assert_eq!(
        first_effect_difference(&run, &twin),
        None,
        "{backend:?}: the carried value applies"
    );
    assert_eq!(
        run.superseded,
        cells * 39,
        "{backend:?}: the 39 values the carried one replaced, per cell"
    );
    // Not vacuous: the carried and the successor's values differ in the swap block.
    let other = effect_run(&b, None, &feed, &[], &b_writes[39..], backend);
    assert_ne!(
        twin.blocks[SWAP_BLOCK], other.blocks[SWAP_BLOCK],
        "{backend:?}: the two last values sound different"
    );
}

/// At the build's own width, `Backend::current()`: a console effect bank binds only at the width
/// its build executes (#1017).
#[test]
fn the_newest_value_wins_and_the_count_is_exact_at_the_build_width() {
    the_newest_value_wins_and_the_count_is_exact(Backend::current());
}

/// The filtered fixture cut to its first `tracks` tracks, each with a true-peak limiter insert and
/// its own knobs. The limiter reports a lookahead latency, so its bank keeps a dry shunt line. Every
/// track has the insert, so no route needs a delay to match another's latency: a route's delay
/// line is a state of its own, which no carry moves yet.
fn limiter_insert_session(tracks: usize) -> SessionModel {
    let mut model = filtered_session(false);
    let cut: Vec<String> = model
        .tracks
        .iter()
        .skip(tracks)
        .map(|track| track.id.as_str().to_owned())
        .collect();
    for track in cut {
        remove_track(&mut model, &track);
    }
    for (index, track) in model.tracks.iter_mut().enumerate() {
        track.inserts.effects.push(Effect {
            id: id("lim"),
            identity: EffectIdentity::Native {
                effect_id: id("miso.true-peak-limiter"),
            },
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::Maximum,
            params: console_params("limiter", index, false),
            sidechain: SidechainDeclaration::None,
        });
    }
    model
}

/// Gate 4: a true-peak limiter insert on a full bank of tracks (four at four lanes, eight at eight),
/// one lane bypassed live before the swap. The swap block is the sixth, so the predecessor's line
/// cursor sits a nonzero number of frames into the line (768 frames against the limiter's 240
/// frames of lookahead), while the successor's starts at zero. The added track has no insert, so the
/// bank stays full. The bypassed lane keeps its delayed dry signal, so the swapped run equals the
/// post-edit session fed the same switch.
fn a_bypassed_lane_keeps_its_delayed_dry_signal(backend: Backend, tracks: usize) {
    let a = Session::compile(limiter_insert_session(tracks));
    let mut b_model = with_muted_track(limiter_insert_session(tracks));
    track_mut(&mut b_model, MUTED_TRACK).inserts.effects.clear();
    let b = Session::compile(b_model);
    let feed = bursty_feed(SOURCE, 5);
    let switch = |strip, block, bypassed| EffectWrite {
        block,
        strip,
        address: LiveEffectAddress::insert(0),
        value: EffectValue::Bypass(BypassSwitch::step(bypassed)),
    };
    let writes = [switch("eq2", 3, true)];
    let reference = effect_run(&b, None, &feed, &writes, &[], backend);
    let run = effect_run(&a, Some((&b, &a.model)), &feed, &writes, &[], backend);
    let what = format!("{backend:?}");
    assert_clean_swap(&run, &what);
    assert_eq!(
        run.effect_lanes, tracks,
        "{what}: the inserts are banked lanes of the predecessor"
    );
    assert_eq!(run.restarted, Vec::<Box<str>>::new(), "{what}");
    assert_eq!(
        first_effect_difference(&run, &reference),
        None,
        "{what}: every block equals the post-edit session fed the same switch"
    );
    // Not vacuous: the bypass is audible, and so is a dry line that starts at rest.
    let active = effect_run(&b, None, &feed, &[], &[], backend);
    assert_ne!(
        active.blocks[SWAP_BLOCK], reference.blocks[SWAP_BLOCK],
        "{what}: the bypass is audible in the swap block"
    );
}

/// At the build's own width, `Backend::current()`, on a full bank: a console effect bank binds only
/// at the width its build executes (#1017).
#[test]
fn a_bypassed_lane_keeps_its_delayed_dry_signal_at_the_build_width() {
    let backend = Backend::current();
    a_bypassed_lane_keeps_its_delayed_dry_signal(backend, backend.width());
}

/// #1341 gate 6: a swap at a block inside a bypass crossfade renders bit-identically to the same
/// session without the swap. The 480-frame ramp starts at block 4 (frame 512) and the swap block
/// starts at frame 768, 256 frames into it. Once for a zero-latency effect (the console
/// compressor) and once for a lane with latency (the console limiter, whose dry line the first
/// blocks after the swap read).
fn a_bypass_crossfade_in_flight_continues_after_the_swap(backend: Backend, slot: &str) {
    let slots = ["comp", "limiter"];
    let a = Session::compile(console_session(false, &slots));
    let b = Session::compile(with_muted_track(console_session(false, &slots)));
    let feed = bursty_feed(SOURCE, 5);
    let address = LiveEffectAddress::console(slots.iter().position(|s| *s == slot).unwrap() as u32);
    let writes: Vec<EffectWrite> = ["eq1", "eq4", "eq8"]
        .into_iter()
        .map(|strip| EffectWrite {
            block: 4,
            strip,
            address,
            value: EffectValue::Bypass(BypassSwitch {
                bypassed: true,
                ramp_samples: 480,
            }),
        })
        .collect();
    let reference = effect_run(&b, None, &feed, &writes, &[], backend);
    let run = effect_run(&a, Some((&b, &a.model)), &feed, &writes, &[], backend);
    let what = format!("{backend:?} {slot}");
    assert_clean_swap(&run, &what);
    assert_eq!(run.restarted, Vec::<Box<str>>::new(), "{what}");
    assert_eq!(
        first_effect_difference(&run, &reference),
        None,
        "{what}: every block equals the session without the swap"
    );
    // Not vacuous: the crossfade is audible after the swap, and it is not a step.
    let step: Vec<EffectWrite> = writes
        .iter()
        .map(|write| EffectWrite {
            value: EffectValue::Bypass(BypassSwitch::step(true)),
            ..*write
        })
        .collect();
    let stepped = effect_run(&b, None, &feed, &step, &[], backend);
    assert_ne!(
        stepped.blocks[SWAP_BLOCK], reference.blocks[SWAP_BLOCK],
        "{what}: the ramp is in flight in the swap block"
    );
}

/// At the build's own width, `Backend::current()`: a console effect bank binds only at the width
/// its build executes (#1017).
#[test]
fn a_bypass_crossfade_in_flight_continues_after_the_swap_at_the_build_width() {
    let backend = Backend::current();
    for slot in ["comp", "limiter"] {
        a_bypass_crossfade_in_flight_continues_after_the_swap(backend, slot);
    }
}

/// D1: a lane carries into a lane of its own kind only. A predecessor with live controls and a
/// successor without them (or the reverse) hold the same console slots as different stage types, so
/// the join moves no effect lane, every strip with a slot restarts, and the hand-over still
/// succeeds: a join that ignored the kind would name a move the swap block refuses, which refuses
/// the whole hand-over, sources included.
fn a_changed_control_kind_carries_no_effect_lane(backend: Backend) {
    let (a, b) = live_console_pair();
    let every_strip: Vec<Box<str>> = (0..9).map(|index| format!("eq{index}").into()).collect();
    // Live -> plain.
    let live = LivePlan::fresh(&a, backend);
    let plain = b
        .prepare_successor(&live.prepared.inventory, &a.model, &caps(), backend)
        .unwrap_or_else(|failure| panic!("successor: {failure:?}"));
    assert_eq!(plain.restarted_strips(), &every_strip[..], "{backend:?}");
    // Plain -> live.
    let plain = a.prepare(backend);
    let live = LivePlan::successor(&b, &plain.inventory, &a.model, backend);
    assert_eq!(
        live.prepared.restarted_strips(),
        &every_strip[..],
        "{backend:?}"
    );
    // The control: the same kind on both sides carries every slot of every strip.
    let live = LivePlan::fresh(&a, backend);
    let successor = LivePlan::successor(&b, &live.prepared.inventory, &a.model, backend);
    assert!(
        successor.prepared.restarted_strips().is_empty(),
        "{backend:?}"
    );
    assert_eq!(
        live.prepared.inventory.effect_lane_count(),
        18,
        "{backend:?}"
    );
}

#[test]
#[cfg_attr(
    not(any(target_feature = "neon", target_feature = "simd128")),
    ignore = "a console effect bank binds only at the width its build executes, and four lanes \
              is the NEON and simd128 width (#1017)"
)]
fn a_changed_control_kind_carries_no_effect_lane_at_four_lanes() {
    a_changed_control_kind_carries_no_effect_lane(Backend::Simd4);
}

// ---------------------------------------------------------------------------------------------
// Issue #1280c: a live difference on a carried lane no longer restarts the strip. The lane
// carries, then the successor retargets it to the new committed value (D4, D15-7).

/// `model` with every entry of `track`'s console slot `slot` for parameter `parameter_id` set to
/// `value` (a per-lane parameter has one entry per channel).
fn with_console_param_all(
    mut model: SessionModel,
    track: &str,
    slot: &str,
    parameter_id: u32,
    value: f32,
) -> SessionModel {
    let entry = track_mut(&mut model, track)
        .console
        .iter_mut()
        .find(|entry| entry.slot.as_str() == slot)
        .expect("console slot");
    let mut found = false;
    for param in entry
        .params
        .iter_mut()
        .filter(|param| param.parameter_id == parameter_id)
    {
        param.value = value;
        found = true;
    }
    assert!(found, "parameter {parameter_id} of {slot}");
    model
}

/// `model` with `track`'s console slot `slot` bypassed.
fn with_console_bypass(mut model: SessionModel, track: &str, slot: &str) -> SessionModel {
    track_mut(&mut model, track)
        .console
        .iter_mut()
        .find(|entry| entry.slot.as_str() == slot)
        .expect("console slot")
        .bypass = true;
    model
}

/// One live edit of gate 5: how it edits the model, and the writes that make the same edit on a
/// running plan (the control plane's live commit).
struct LiveEdit {
    name: &'static str,
    edit: fn(SessionModel, &str) -> SessionModel,
    write: fn(&'static str, usize, u32) -> Vec<EffectWrite>,
}

const RETARGETED: [&str; 3] = ["eq1", "eq4", "eq8"];

/// Gate 5: B changes a carried lane's compressor threshold, an EQ band gain (live values), or
/// bypasses it. Run 1 is the structural swap: B is prepared from the post-edit session, the lanes
/// carry and the successor retargets them to the new values. Run 2 writes the same edits to A just
/// before the swap and prepares B from a base that holds them. The two runs are bit-identical in
/// every block, and no strip is in the restart set. Without the retarget the carried lane keeps
/// the old value, which the last assertion shows is audible.
fn a_live_difference_on_a_carried_lane_is_retargeted(backend: Backend) {
    let slots = ["eq", "comp"];
    let ramp = LiveRamps::for_session(&console_session(false, &slots))
        .for_row(LiveRampRow::BypassCrossfade);
    let edits = [
        LiveEdit {
            name: "compressor threshold",
            edit: |model, strip| with_console_param_all(model, strip, "comp", 1, -40.0),
            write: |strip, block, _| {
                vec![EffectWrite {
                    block,
                    strip,
                    address: COMP_ADDRESS,
                    value: EffectValue::Parameter {
                        index: THRESHOLD_INDEX,
                        value: -40.0,
                    },
                }]
            },
        },
        LiveEdit {
            name: "EQ band gain",
            edit: |model, strip| with_console_param_all(model, strip, "eq", 4, 2.0),
            write: |strip, block, _| {
                vec![EffectWrite {
                    block,
                    strip,
                    address: EQ_ADDRESS,
                    value: EffectValue::BandGainBoth(2.0),
                }]
            },
        },
        LiveEdit {
            name: "compressor bypass",
            edit: |model, strip| with_console_bypass(model, strip, "comp"),
            write: |strip, block, ramp| {
                vec![EffectWrite {
                    block,
                    strip,
                    address: COMP_ADDRESS,
                    value: EffectValue::Bypass(BypassSwitch {
                        bypassed: true,
                        ramp_samples: ramp,
                    }),
                }]
            },
        },
    ];
    let feed = bursty_feed(SOURCE, 5);
    for LiveEdit { name, edit, write } in edits {
        let what = format!("{backend:?} {name}");
        let (a, b) = live_console_pair();
        let edit_all = |model: &SessionModel| RETARGETED.into_iter().fold(model.clone(), edit);
        let b_edited = Session::compile(edit_all(&b.model));
        let a_edited = edit_all(&a.model);
        let writes: Vec<EffectWrite> = RETARGETED
            .into_iter()
            .flat_map(|strip| write(strip, SWAP_BLOCK, ramp))
            .collect();
        // Run 1: the structural swap, with the retarget.
        let run = effect_run(&a, Some((&b_edited, &a.model)), &feed, &[], &[], backend);
        assert_clean_swap(&run, &what);
        assert_eq!(
            run.restarted,
            Vec::<Box<str>>::new(),
            "{what}: nothing restarts"
        );
        // Run 2: live edit on A, then the structural swap from a base that holds it.
        let reference = effect_run(
            &a,
            Some((&b_edited, &a_edited)),
            &feed,
            &writes,
            &[],
            backend,
        );
        assert_clean_swap(&reference, &format!("{what} reference"));
        assert_eq!(
            first_effect_difference(&run, &reference),
            None,
            "{what}: carry, then retarget equals live edit, then structural edit"
        );
        // Not vacuous: the carried lane keeps its old value without the retarget, and the edit
        // is audible in the swap block.
        let stale = effect_run(&a, Some((&b, &a.model)), &feed, &[], &[], backend);
        assert_ne!(
            stale.blocks[SWAP_BLOCK + 1],
            run.blocks[SWAP_BLOCK + 1],
            "{what}: the edit is audible after the swap"
        );
    }
}

/// At the build's own width, `Backend::current()`: a console effect bank binds only at the width
/// its build executes (#1017).
#[test]
fn a_live_difference_on_a_carried_lane_is_retargeted_at_the_build_width() {
    a_live_difference_on_a_carried_lane_is_retargeted(Backend::current());
}

// ---------------------------------------------------------------------------------------------
// #1280b gate 8: the Observe records a carried lane's retired queue still holds.
//
// A subscribe is acknowledged when it is queued. After the predecessor's last block nothing pops
// its queue, so the carry applies what the queue holds to the predecessor's tap, bounded by the
// count at entry, at the sample the predecessor's next block would have applied it.

/// A compressor insert on each of `tracks` tracks, so the inserts form one bank of lanes with an
/// observation tap each.
fn compressor_insert_session(tracks: usize) -> SessionModel {
    let mut model = filtered_session(false);
    let cut: Vec<String> = model
        .tracks
        .iter()
        .skip(tracks)
        .map(|track| track.id.as_str().to_owned())
        .collect();
    for track in cut {
        remove_track(&mut model, &track);
    }
    for (index, track) in model.tracks.iter_mut().enumerate() {
        track.inserts.effects.push(Effect {
            id: id("comp"),
            identity: EffectIdentity::Native {
                effect_id: id("miso.compressor"),
            },
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            params: console_params("comp", index, false),
            sidechain: SidechainDeclaration::None,
        });
    }
    model
}

/// What sits in the retired queues when the swap block starts.
#[derive(Clone, Copy, Debug)]
enum PendingObserve {
    /// One subscribe, in the first lane's queue; every other queue is empty.
    OneSubscribe,
    /// Every lane's queue filled to its capacity, so a further push is refused.
    FullQueues,
}

/// The records `pending` puts in `plan`'s queues; returns the strips that now hold some.
fn admit_observes(plan: &mut LivePlan, strips: &[&str], pending: PendingObserve) -> Vec<String> {
    let admitted: &[&str] = match pending {
        PendingObserve::OneSubscribe => &strips[..1],
        PendingObserve::FullQueues => strips,
    };
    for strip in admitted {
        let producer = plan
            .handles
            .effect_control_mut(strip, LiveEffectAddress::insert(0))
            .expect("a control channel for the insert");
        let capacity = producer.observe_capacity();
        assert!(capacity > 0, "{strip}: the insert has an observation queue");
        let records = match pending {
            PendingObserve::OneSubscribe => 1,
            PendingObserve::FullQueues => capacity,
        };
        for _ in 0..records {
            producer
                .try_push_observe(0, true, 1, 0)
                .expect("room in the queue");
        }
        if let PendingObserve::FullQueues = pending {
            assert!(
                producer.try_push_observe(0, true, 1, 0).is_err(),
                "{strip}: the queue is full"
            );
        }
        assert_eq!(
            producer.observe_available_capacity(),
            capacity - records,
            "{strip}: the records are queued"
        );
    }
    admitted.iter().map(|strip| (*strip).to_owned()).collect()
}

fn observed_request() -> host_core::HostLiveControlRequest {
    host_core::HostLiveControlRequest {
        observation_taps: 1,
        ..LivePlan::request()
    }
}

fn pending_observe_records_are_applied_at_the_carry(
    backend: Backend,
    tracks: usize,
    pending: PendingObserve,
) {
    let mut b_model = with_muted_track(compressor_insert_session(tracks));
    track_mut(&mut b_model, MUTED_TRACK).inserts.effects.clear();
    pending_observe_swap(
        backend,
        compressor_insert_session(tracks),
        b_model,
        pending,
        None,
    );
}

/// The swap of [`pending_observe_records_are_applied_at_the_carry`] between any two sessions
/// whose inserts all carry. `form` is `(predecessor, successor)` as `(per node, banked)` flags
/// when the swap changes the inserts' representation (#1281), which the test checks first.
fn pending_observe_swap(
    backend: Backend,
    a_model: SessionModel,
    b_model: SessionModel,
    pending: PendingObserve,
    form: Option<(bool, bool)>,
) {
    use bench_support::alloc as bench_alloc;
    let a = Session::compile(a_model);
    let b = Session::compile(b_model);
    if let Some((a_per_node, b_per_node)) = form {
        assert_eq!(
            insert_forms(&a, backend),
            vec![a_per_node; a.model.tracks.len()]
        );
        assert_eq!(
            insert_forms(&b, backend),
            vec![b_per_node; b.model.tracks.len()]
        );
    }
    let feed = bursty_feed(SOURCE, 5);
    let quantum = a.model.quantum_frames as usize;
    let request = observed_request();
    // A strip B removes has no lane to carry, and its tap goes with it.
    let strips: Vec<&str> = a
        .model
        .tracks
        .iter()
        .map(|track| track.id.as_str())
        .filter(|id| b.model.tracks.iter().any(|track| track.id.as_str() == *id))
        .collect();
    let what = format!("{backend:?} {pending:?}");

    // The reference: the same session, records admitted after the same last block, and the block
    // the swap block would have been rendered by the predecessor itself.
    let mut reference = LivePlan::fresh_with(&a, &request, backend);
    for block in 0..SWAP_BLOCK {
        reference.submit(&feed, quantum, block);
        reference.render(quantum);
    }
    let admitted = admit_observes(&mut reference, &strips, pending);
    reference.submit(&feed, quantum, SWAP_BLOCK);
    reference.render(quantum);
    let armed_at_reference = reference.prepared.plan.observation_binding_counts()[2];
    assert_eq!(armed_at_reference, admitted.len() as u64, "{what}");
    let window_of = |plan: &LivePlan, strip: &str| {
        plan.handles
            .effect_observations
            .iter()
            .find(|handle| {
                handle.track_id.as_ref() == strip && handle.address == LiveEffectAddress::insert(0)
            })
            .and_then(|handle| handle.readers[0].read())
    };

    // The swap: A renders its blocks, the records are admitted, B is prepared and adopts.
    let mut plan = LivePlan::fresh_with(&a, &request, backend);
    for block in 0..SWAP_BLOCK {
        plan.submit(&feed, quantum, block);
        plan.render(quantum);
    }
    let admitted = admit_observes(&mut plan, &strips, pending);
    assert_eq!(
        plan.prepared.plan.observation_binding_counts()[2],
        0,
        "{what}: nothing is armed before the carry: the predecessor drains no more"
    );
    let mut successor =
        LivePlan::successor_with(&b, &request, &plan.prepared.inventory, &a.model, backend);
    assert_eq!(
        successor.prepared.restarted_strips(),
        &[] as &[Box<str>],
        "{what}: every insert lane carries"
    );
    assert_eq!(
        successor
            .prepared
            .sources
            .adopt_persisting(&mut plan.prepared.sources),
        1
    );
    successor.submit(&feed, quantum, SWAP_BLOCK);
    let mut output = vec![f32::NAN; quantum * 2];
    engine::realtime::audit::reset();
    let mark = bench_alloc::current_thread_counters();
    let carry = successor
        .prepared
        .plan
        .adopt_predecessor_plan(&mut plan.prepared.plan);
    successor.render_into(&mut output, quantum);
    let delta = bench_alloc::current_thread_delta_since(mark);
    let audit = engine::realtime::audit::snapshot();
    assert_eq!(carry, CarryOutcome::Carried, "{what}: the swap block");
    assert_eq!(
        (
            (delta.allocations, delta.reallocations, delta.deallocations),
            (audit.allocations, audit.deallocations)
        ),
        ((0, 0, 0), (0, 0)),
        "{what}: allocator and render audit over the swap block"
    );
    assert_eq!(
        graph::carry_refusal_count(&mut successor.prepared.plan).expect("a graph plan"),
        0,
        "{what}: no restore was refused"
    );

    // A's queues read empty: the carry popped every record.
    for strip in &strips {
        let producer = plan
            .handles
            .effect_control_mut(strip, LiveEffectAddress::insert(0))
            .expect("a control channel for the insert");
        assert_eq!(
            producer.observe_available_capacity(),
            producer.observe_capacity(),
            "{what}: {strip}: the retired queue reads empty"
        );
    }
    // The acknowledged subscribe is not dropped at the swap (#1327b): the carry applied it to
    // A's tap as A's next block would have, and the tap then moved to the successor with its
    // window and its publisher, so A keeps the successor's fresh, unarmed taps.
    assert_eq!(
        successor.prepared.plan.observation_binding_counts()[2],
        armed_at_reference,
        "{what}: every admitted subscribe is armed on the successor's tap"
    );
    assert_eq!(
        plan.prepared.plan.observation_binding_counts()[2],
        0,
        "{what}: the retiring predecessor holds the fresh taps"
    );
    // The strip meters (#1581 D1) carry beside the taps; the record checked is the taps'.
    let carries: Vec<_> = successor
        .prepared
        .plan
        .observer_carries()
        .iter()
        .filter(|carry| carry.kind == engine::realtime::ObserverKind::Observation)
        .copied()
        .collect();
    assert_eq!(
        carries.len(),
        strips.len(),
        "{what}: one record per carried instance"
    );
    host_core::pair_carried_observations(
        &mut successor.handles.effect_observations,
        &mut plan.handles.effect_observations,
        &carries,
    );
    // The successor's swap block published the window the predecessor's next block would have:
    // the reference's, field for field, from the reader the host now holds for the successor.
    for strip in &strips {
        let expected = window_of(&reference, strip);
        assert_eq!(
            expected.is_some(),
            admitted.iter().any(|admitted| admitted == strip),
            "{what}: {strip}: the reference publishes exactly the subscribed windows"
        );
        assert_eq!(
            window_of(&successor, strip),
            expected,
            "{what}: {strip}: the successor publishes the window the predecessor would have"
        );
    }
}

/// The acknowledged Observe records in the retired queues -- one subscribe, and every queue full --
/// at the build's own width, `Backend::current()`, on a full bank of insert lanes: a console effect
/// bank binds only at the width its build executes (#1017). Then the same records of per-node
/// instances that carry to per-node instances (#1816): a group one member below the insert padding
/// threshold (#888) on both sides of the swap.
#[test]
fn pending_observe_records_are_applied_at_the_carry_at_the_build_width() {
    let backend = Backend::current();
    let (short, _) = short_and_full(Insert::Compressor, backend);
    for pending in [PendingObserve::OneSubscribe, PendingObserve::FullQueues] {
        pending_observe_records_are_applied_at_the_carry(backend, backend.width(), pending);
        pending_observe_swap(
            backend,
            short.clone(),
            short.clone(),
            pending,
            Some((true, true)),
        );
    }
}

// ---------------------------------------------------------------------------------------------
// #1281: an insert lane that moves between a bank and a per-node instance.
//
// A partial insert group banks, padded, from the insert padding threshold (#888: three members at
// eight lanes, two at four), so adding the insert that reaches it turns the per-node instances
// below it into one padded bank, and removing one turns the bank back. Every lane keeps its exact
// state, live state included, in both directions.

/// Which insert the form-change gates run.
#[derive(Clone, Copy, Debug)]
enum Insert {
    Compressor,
    Limiter,
}

impl Insert {
    fn session(self, tracks: usize) -> SessionModel {
        match self {
            Self::Compressor => compressor_insert_session(tracks),
            Self::Limiter => limiter_insert_session(tracks),
        }
    }
}

/// The lanes of one bank at `backend`.
fn bank_width(backend: Backend) -> usize {
    backend.width()
}

/// The fewest members of a partial insert group the planner pads into a bank at `backend`'s width
/// (#888: `graph_compiler`'s `min_padded_insert_members`, the H5 arithmetic): below it the group
/// renders per node.
fn insert_pad_threshold(backend: Backend) -> usize {
    match backend.width() {
        4 => 2,
        8 => 3,
        other => panic!("no insert padding threshold recorded for {other} lanes"),
    }
}

/// `model` with [`MUTED_TRACK`], a muted copy of `eq0` whose compressor insert follows an EQ: its
/// longer program leads the cohort, the other tracks' compressors join it as a subsequence one
/// dependency level early, and a bank may not cross a level, so every compressor of the group and
/// the lone EQ render per node however many members it has (#888: no remainder size decides it).
fn with_misaligned_muted_track(model: SessionModel) -> SessionModel {
    let mut model = with_muted_track(model);
    track_mut(&mut model, MUTED_TRACK).inserts.effects.insert(
        0,
        native_insert("eq", "miso.parametric-eq", console_params("eq", 0, false)),
    );
    model
}

/// Whether each insert of `session` is a per-node instance (`true`) or a lane of a bank (`false`),
/// in strip order.
fn insert_forms(session: &Session, backend: Backend) -> Vec<bool> {
    let mut plan = LivePlan::fresh(session, backend);
    let mut forms: Vec<(Box<str>, bool)> = graph::bank_effect_lanes(&mut plan.prepared.plan)
        .expect("a graph plan")
        .into_iter()
        .filter(|(_, lane)| lane.rack == graph::RackId::Dynamic)
        .map(|(strip, lane)| {
            (
                strip,
                matches!(lane.location, graph::GraphEffectSite::Node(_)),
            )
        })
        .collect();
    forms.sort();
    forms.into_iter().map(|(_, per_node)| per_node).collect()
}

/// A group one member below the insert padding threshold (per-node instances) and the padded bank
/// it becomes when [`MUTED_TRACK`], a muted copy of `eq0` with the same insert, joins it (#888).
fn short_and_full(insert: Insert, backend: Backend) -> (SessionModel, SessionModel) {
    let short = insert.session(insert_pad_threshold(backend) - 1);
    let full = with_muted_track(short.clone());
    (short, full)
}

/// `model` with parameter `parameter_id` of `track`'s insert `insert` set to `value` on every
/// channel entry.
fn with_insert_param_all(
    mut model: SessionModel,
    track: &str,
    insert: &str,
    parameter_id: u32,
    value: f32,
) -> SessionModel {
    let entry = track_mut(&mut model, track)
        .inserts
        .effects
        .iter_mut()
        .find(|entry| entry.id.as_str() == insert)
        .expect("insert");
    let mut found = false;
    for param in entry
        .params
        .iter_mut()
        .filter(|param| param.parameter_id == parameter_id)
    {
        param.value = value;
        found = true;
    }
    assert!(found, "parameter {parameter_id} of {insert}");
    model
}

/// The sessions of a swap in one direction: forward is short to full (per node to bank), the
/// reverse full to short (bank to per node). Returns `(a, b, a per node, b per node)`.
fn form_pair(insert: Insert, backend: Backend, forward: bool) -> (Session, Session) {
    let (short, full) = short_and_full(insert, backend);
    let (a, b) = if forward {
        (short, full)
    } else {
        (full, short)
    };
    let (a, b) = (Session::compile(a), Session::compile(b));
    let k = insert_pad_threshold(backend);
    let (a_tracks, b_tracks) = if forward { (k - 1, k) } else { (k, k - 1) };
    assert_eq!(
        insert_forms(&a, backend),
        vec![forward; a_tracks],
        "{backend:?} {insert:?}: A's inserts are {}",
        if forward { "per node" } else { "one bank" }
    );
    assert_eq!(
        insert_forms(&b, backend),
        vec![!forward; b_tracks],
        "{backend:?} {insert:?}: B's inserts are {}",
        if forward { "one bank" } else { "per node" }
    );
    (a, b)
}

/// Gates 1 and 2: every block of the swapped run equals B rendered from frame 0, nothing
/// restarts, the carry refuses nothing and the swap block makes no allocator call. A swap whose
/// strips all restart renders other bits, which the last assertion shows.
fn an_insert_group_changes_form(backend: Backend, insert: Insert, forward: bool) {
    let what = format!("{backend:?} {insert:?} forward={forward}");
    let (a, b) = form_pair(insert, backend, forward);
    let feed = bursty_feed(SOURCE, 5);
    let reference = effect_run(&b, None, &feed, &[], &[], backend);
    let run = effect_run(&a, Some((&b, &a.model)), &feed, &[], &[], backend);
    assert_clean_swap(&run, &what);
    assert_eq!(run.restarted, Vec::<Box<str>>::new(), "{what}");
    assert_eq!(
        first_effect_difference(&run, &reference),
        None,
        "{what}: every block equals B rendered from frame 0"
    );
    // Not vacuous: with the inserts declared another quality in A's committed base, every strip
    // restarts, the lanes start at rest, and the blocks differ.
    let mut committed = a.model.clone();
    for track in &mut committed.tracks {
        for entry in &mut track.inserts.effects {
            entry.quality = session::EffectQuality::High;
        }
    }
    let restarted = effect_run(&a, Some((&b, &committed)), &feed, &[], &[], backend);
    assert!(
        !restarted.restarted.is_empty(),
        "{what}: the strips restart"
    );
    assert_ne!(
        first_effect_difference(&restarted, &reference),
        None,
        "{what}: the insert state is audible in the blocks after the swap"
    );
}

/// At the build's own width, `Backend::current()`: a console effect bank binds only at the width
/// its build executes (#1017).
#[test]
fn a_group_of_inserts_becomes_a_bank_at_the_build_width() {
    let backend = Backend::current();
    for insert in [Insert::Compressor, Insert::Limiter] {
        an_insert_group_changes_form(backend, insert, true);
    }
}

/// At the build's own width, `Backend::current()`: a console effect bank binds only at the width
/// its build executes (#1017).
#[test]
fn a_bank_of_inserts_falls_apart_at_the_build_width() {
    let backend = Backend::current();
    for insert in [Insert::Compressor, Insert::Limiter] {
        an_insert_group_changes_form(backend, insert, false);
    }
}

/// The compressor's `threshold` as a session parameter ID.
const THRESHOLD_PARAMETER_ID: u32 = 1;
/// The insert strips gate 3 writes to and retargets, every one in both sessions of a pair and
/// changing form: the per-node side's members, at most two (one at four lanes, where the per-node
/// side is one member below the padding threshold of two; #888).
fn live_strips(backend: Backend) -> Vec<&'static str> {
    let mut strips = vec!["eq0", "eq1"];
    strips.truncate(insert_pad_threshold(backend) - 1);
    strips
}

/// Gate 3, first case: a threshold is rendered in A, and another is written to A's cells just
/// before the swap and not rendered. Every block equals B rendered from frame 0 and fed the same
/// values at the same blocks.
fn an_unread_value_crosses_the_form_change(backend: Backend, forward: bool) {
    let what = format!("{backend:?} forward={forward}");
    let (a, b) = form_pair(Insert::Compressor, backend, forward);
    let feed = bursty_feed(SOURCE, 5);
    let writes = |unread_too: bool| {
        let mut writes = Vec::new();
        for strip in live_strips(backend) {
            let threshold = |block, value| EffectWrite {
                block,
                strip,
                address: LiveEffectAddress::insert(0),
                value: EffectValue::Parameter {
                    index: THRESHOLD_INDEX,
                    value,
                },
            };
            // The block-5 signal is quiet: only the lower threshold compresses it.
            writes.push(threshold(3, -5.0));
            if unread_too {
                writes.push(threshold(SWAP_BLOCK, -50.0));
            }
        }
        writes
    };
    let reference = effect_run(&b, None, &feed, &writes(true), &[], backend);
    let run = effect_run(&a, Some((&b, &a.model)), &feed, &writes(true), &[], backend);
    assert_clean_swap(&run, &what);
    assert_eq!(run.restarted, Vec::<Box<str>>::new(), "{what}");
    assert_eq!(
        first_effect_difference(&run, &reference),
        None,
        "{what}: every block equals B fed the same values"
    );
    // Not vacuous: the value left unread in A changes the swap block.
    let lost = effect_run(&b, None, &feed, &writes(false), &[], backend);
    assert_ne!(
        lost.blocks[SWAP_BLOCK], reference.blocks[SWAP_BLOCK],
        "{what}: the unread value is audible in the swap block"
    );
}

/// Gate 3, second case: B also changes the threshold of the lanes that change form
/// ([`live_strips`]). Run 1 is the
/// structural swap, where the successor retargets the carried lanes. Run 2 writes the same edit
/// to A just before the swap and prepares B from a base that holds it ("live edit, then
/// structural edit"). The two are bit-identical in every block and no strip restarts.
fn a_live_difference_crosses_the_form_change(backend: Backend, forward: bool) {
    let what = format!("{backend:?} forward={forward}");
    let (a, b) = form_pair(Insert::Compressor, backend, forward);
    let feed = bursty_feed(SOURCE, 5);
    let edit_all = |model: &SessionModel| {
        live_strips(backend)
            .into_iter()
            .fold(model.clone(), |model, strip| {
                with_insert_param_all(model, strip, "comp", THRESHOLD_PARAMETER_ID, -40.0)
            })
    };
    let b_edited = Session::compile(edit_all(&b.model));
    let a_edited = edit_all(&a.model);
    let writes: Vec<EffectWrite> = live_strips(backend)
        .into_iter()
        .map(|strip| EffectWrite {
            block: SWAP_BLOCK,
            strip,
            address: LiveEffectAddress::insert(0),
            value: EffectValue::Parameter {
                index: THRESHOLD_INDEX,
                value: -40.0,
            },
        })
        .collect();
    let run = effect_run(&a, Some((&b_edited, &a.model)), &feed, &[], &[], backend);
    assert_clean_swap(&run, &what);
    assert_eq!(
        run.restarted,
        Vec::<Box<str>>::new(),
        "{what}: nothing restarts"
    );
    let reference = effect_run(
        &a,
        Some((&b_edited, &a_edited)),
        &feed,
        &writes,
        &[],
        backend,
    );
    assert_clean_swap(&reference, &format!("{what} reference"));
    assert_eq!(
        first_effect_difference(&run, &reference),
        None,
        "{what}: carry, then retarget equals live edit, then structural edit"
    );
    // Not vacuous: without the retarget the carried lanes keep their old threshold.
    let stale = effect_run(&a, Some((&b, &a.model)), &feed, &[], &[], backend);
    assert_ne!(
        stale.blocks[SWAP_BLOCK + 1],
        run.blocks[SWAP_BLOCK + 1],
        "{what}: the edit is audible after the swap"
    );
}

/// At the build's own width, `Backend::current()`: a console effect bank binds only at the width
/// its build executes (#1017).
#[test]
fn live_values_cross_the_form_change_at_the_build_width() {
    let backend = Backend::current();
    for forward in [true, false] {
        an_unread_value_crosses_the_form_change(backend, forward);
        a_live_difference_crosses_the_form_change(backend, forward);
    }
}

/// A pending Observe record of a lane that changes form is applied to the predecessor's tap at the
/// carry (#1280 D3), for a per-node predecessor too: its queue is read at the sample its next block
/// would have started at.
#[cfg(target_feature = "avx2")]
#[test]
fn pending_observe_records_cross_the_form_change_at_eight_lanes() {
    let (short, full) = short_and_full(Insert::Compressor, Backend::Simd8);
    for pending in [PendingObserve::OneSubscribe, PendingObserve::FullQueues] {
        pending_observe_swap(
            Backend::Simd8,
            short.clone(),
            full.clone(),
            pending,
            Some((true, false)),
        );
        pending_observe_swap(
            Backend::Simd8,
            full.clone(),
            short.clone(),
            pending,
            Some((false, true)),
        );
    }
}

// ---------------------------------------------------------------------------------------------
// #1282: per-node effect instances move by pointer across a plan swap.
// ---------------------------------------------------------------------------------------------

/// Blocks of the tail runs: long enough for a 50 ms delay at high feedback to ring through
/// several repeats after the swap.
const TAIL_BLOCKS: usize = 64;

/// A native insert `instance` of `effect` with `params`, dual-mono linked.
fn native_insert(instance: &str, effect: &str, params: Vec<EffectParam>) -> Effect {
    Effect {
        id: id(instance),
        identity: EffectIdentity::Native {
            effect_id: id(effect),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        params,
        sidechain: SidechainDeclaration::None,
    }
}

/// Three tracks, each with one insert that no plan banks (a group of one never fills a bank, and
/// the delay has no bank kernel): `eq0` a 50 ms delay at high feedback, `eq1` a multiband
/// compressor, `eq2` a compressor.
fn tail_session() -> SessionModel {
    use ParameterUnit::{Linear, Milliseconds};
    let mut model = filtered_session(false);
    let cut: Vec<String> = model
        .tracks
        .iter()
        .skip(3)
        .map(|track| track.id.as_str().to_owned())
        .collect();
    for track in cut {
        remove_track(&mut model, &track);
    }
    let inserts = [
        native_insert(
            "dly",
            "miso.delay",
            vec![
                param(1, Milliseconds, 50.0),
                param(2, Linear, 0.85),
                param(3, Linear, 0.2),
                param(4, Linear, 0.5),
            ],
        ),
        native_insert("mb", "miso.multiband-compressor", Vec::new()),
        native_insert("comp", "miso.compressor", console_params("comp", 0, false)),
    ];
    for (track, insert) in model.tracks.iter_mut().zip(inserts) {
        track.inserts.effects.push(insert);
    }
    model
}

/// [`tail_session`] and its successor: the same session plus the muted track [`MUTED_TRACK`],
/// which sorts first and holds no insert.
fn tail_pair() -> (Session, Session) {
    let mut b = with_muted_track(tail_session());
    track_mut(&mut b, MUTED_TRACK).inserts.effects.clear();
    (Session::compile(tail_session()), Session::compile(b))
}

/// The source plays for four blocks, then drops to a deterministic noise near -80 dBFS that is
/// never an exact zero, so no stage skips its work for silence.
fn tail_feed() -> Feed {
    let mut feed = Feed::with_blocks(SOURCE, 9, TAIL_BLOCKS);
    for plane in &mut feed.planes {
        for sample in &mut plane[4 * QUANTUM..] {
            *sample *= 0.0002;
        }
    }
    assert!(feed.planes.iter().flatten().all(|sample| *sample != 0.0));
    feed
}

/// Every insert of `session` is a per-node instance.
fn assert_inserts_per_node(session: &Session, backend: Backend, count: usize, what: &str) {
    assert_eq!(
        insert_forms(session, backend),
        vec![true; count],
        "{what}: every insert renders per node"
    );
}

fn loudest(block: &[u32]) -> f32 {
    block
        .iter()
        .map(|bits| f32::from_bits(*bits).abs())
        .fold(0.0, f32::max)
}

/// Gate 1: a delay's echoes and a multiband compressor's envelope ring on through a swap that
/// adds a muted track, at the sixth block, while the source is down at the noise floor. The
/// swapped run is bit-identical to B fed the same PCM from frame 0, nothing restarts, the swap
/// block makes no allocator call, and the carry passes nothing through the payload scratch. The
/// same swap with every strip restarted loses the echoes.
fn a_delay_tail_rings_on_through_the_swap(backend: Backend) {
    let what = format!("{backend:?}");
    let (a, b) = tail_pair();
    assert_inserts_per_node(&a, backend, 3, &what);
    assert_inserts_per_node(&b, backend, 3, &what);
    let feed = tail_feed();
    let reference = effect_run_blocks(&b, None, &feed, &[], &[], backend, TAIL_BLOCKS);
    let run = effect_run_blocks(
        &a,
        Some((&b, &a.model)),
        &feed,
        &[],
        &[],
        backend,
        TAIL_BLOCKS,
    );
    assert_clean_swap(&run, &what);
    assert_eq!(run.effect_lanes, 3, "{what}: three per-node instances");
    assert_eq!(run.restarted, Vec::<Box<str>>::new(), "{what}");
    assert_eq!(
        run.payload_bytes, 0,
        "{what}: nothing passed through the scratch"
    );
    assert_eq!(
        first_effect_difference(&run, &reference),
        None,
        "{what}: every block equals B rendered from frame 0"
    );
    // Not vacuous. The first echo of the source's loud blocks is audible long after the swap...
    assert!(
        loudest(&reference.blocks[20]) > 1e-3,
        "{what}: the echo rings above the noise floor"
    );
    // ...and a swap whose strips all restart cuts it.
    let mut committed = a.model.clone();
    for track in &mut committed.tracks {
        for entry in &mut track.inserts.effects {
            entry.quality = session::EffectQuality::High;
        }
    }
    let restarted = effect_run_blocks(
        &a,
        Some((&b, &committed)),
        &feed,
        &[],
        &[],
        backend,
        TAIL_BLOCKS,
    );
    assert_eq!(restarted.restarted.len(), 3, "{what}: the strips restart");
    let cut = first_effect_difference(&restarted, &reference).expect("the echoes are cut");
    assert!(cut <= 20, "{what}: the echo is lost at block {cut}");
}

#[test]
fn a_delay_tail_rings_on_through_the_swap_at_every_width() {
    for backend in backends() {
        a_delay_tail_rings_on_through_the_swap(backend);
    }
}

/// Gate 2: a delay-time value written to A's cell just before the swap (never rendered by A) and
/// a ramped live bypass of the per-node compressor a block earlier both reach the successor. The
/// reference is B fed the same writes at the same blocks.
fn unread_values_and_a_live_bypass_cross_the_swap(backend: Backend) {
    let what = format!("{backend:?}");
    let (a, b) = tail_pair();
    let feed = tail_feed();
    let writes = [
        EffectWrite {
            block: SWAP_BLOCK - 1,
            strip: "eq2",
            address: LiveEffectAddress::insert(0),
            value: EffectValue::Bypass(BypassSwitch {
                bypassed: true,
                ramp_samples: 300,
            }),
        },
        EffectWrite {
            block: SWAP_BLOCK,
            strip: "eq0",
            address: LiveEffectAddress::insert(0),
            // The delay time, in milliseconds.
            value: EffectValue::Parameter {
                index: 0,
                value: 80.0,
            },
        },
    ];
    let reference = effect_run_blocks(&b, None, &feed, &writes, &[], backend, TAIL_BLOCKS);
    let run = effect_run_blocks(
        &a,
        Some((&b, &a.model)),
        &feed,
        &writes,
        &[],
        backend,
        TAIL_BLOCKS,
    );
    assert_clean_swap(&run, &what);
    assert_eq!(run.restarted, Vec::<Box<str>>::new(), "{what}");
    assert_eq!(run.payload_bytes, 0, "{what}");
    assert_eq!(
        first_effect_difference(&run, &reference),
        None,
        "{what}: every block equals B fed the same values at the same blocks"
    );
    // Not vacuous: each write is audible after the swap on its own.
    for kept in 0..writes.len() {
        let one = [writes[kept]];
        let alone = effect_run_blocks(&b, None, &feed, &one, &[], backend, TAIL_BLOCKS);
        let other = [writes[1 - kept]];
        let without = effect_run_blocks(&b, None, &feed, &other, &[], backend, TAIL_BLOCKS);
        assert_ne!(
            first_effect_difference(&alone, &without),
            None,
            "{what}: write {kept} changes the output"
        );
    }
}

#[test]
fn unread_values_and_a_live_bypass_cross_the_swap_at_every_width() {
    for backend in backends() {
        unread_values_and_a_live_bypass_cross_the_swap(backend);
    }
}

/// Gate 3: every committed bypass change is lane state (#1339 D5 for the delay, #1340 for the
/// multiband compressor), so a rebuild that also lifts one carries the instance (the delay's ring
/// and feedback tail, the multiband's crossover and smoother) and applies the committed bypass as
/// a lane record over the session's bypass crossfade. For each, A runs it bypassed; B lifts it and
/// adds the muted track. Nothing restarts, and every block equals A lifting the bypass live at the
/// swap block.
fn a_lifted_bypass_carries_across_a_rebuild(backend: Backend) {
    let what = format!("{backend:?}");
    let feed = tail_feed();
    for (strip, effect) in [("eq0", "delay"), ("eq1", "multiband")] {
        let what = format!("{what} {effect}");
        let mut a_model = tail_session();
        track_mut(&mut a_model, strip).inserts.effects[0].bypass = true;
        let a = Session::compile(a_model);
        let (_, b) = tail_pair();
        let run = effect_run_blocks(
            &a,
            Some((&b, &a.model)),
            &feed,
            &[],
            &[],
            backend,
            TAIL_BLOCKS,
        );
        assert_clean_swap(&run, &what);
        assert_eq!(
            run.restarted,
            Vec::<Box<str>>::new(),
            "{what}: the bypass carries"
        );
        let ramp = LiveRamps::for_session(&a.model).for_row(LiveRampRow::BypassCrossfade);
        let lift = EffectWrite {
            block: SWAP_BLOCK,
            strip,
            address: LiveEffectAddress::insert(0),
            value: EffectValue::Bypass(BypassSwitch {
                bypassed: false,
                ramp_samples: ramp,
            }),
        };
        let live = effect_run_blocks(&a, None, &feed, &[lift], &[], backend, TAIL_BLOCKS);
        assert_eq!(
            first_effect_difference(&run, &live),
            None,
            "{what}: the rebuild equals A lifting the bypass live at the swap block"
        );
        // Not vacuous: the lift is heard.
        let kept = effect_run_blocks(&a, None, &feed, &[], &[], backend, TAIL_BLOCKS);
        assert_ne!(
            first_effect_difference(&live, &kept),
            None,
            "{what}: the lift changes the output"
        );
    }
}

#[test]
fn a_lifted_bypass_carries_across_a_rebuild_at_every_width() {
    for backend in backends() {
        a_lifted_bypass_carries_across_a_rebuild(backend);
    }
}

/// D3: a plan that holds the owners without live controls has no row for them, so a successor
/// built from live-controlled predecessor lanes restarts their strips.
fn an_owner_without_live_controls_on_one_side_restarts(backend: Backend) {
    let what = format!("{backend:?}");
    let (a, b) = tail_pair();
    let mut plan = LivePlan::fresh(&a, backend);
    let successor = b
        .prepare_successor(&plan.prepared.inventory, &a.model, &caps(), backend)
        .unwrap_or_else(|failure| panic!("{what}: {failure:?}"));
    let mut restarted = successor.restarted_strips().to_vec();
    restarted.sort();
    assert_eq!(
        restarted,
        ["eq0", "eq1", "eq2"].map(Box::<str>::from).to_vec(),
        "{what}: every strip with a per-node insert restarts"
    );
    drop(plan.prepared.plan.take_revision_gate());
}

#[test]
fn an_owner_without_live_controls_on_one_side_restarts_at_every_width() {
    for backend in backends() {
        an_owner_without_live_controls_on_one_side_restarts(backend);
    }
}

/// Gate 4: at 96 kHz the delay holds two seconds a channel, about 1.5 MiB. Its owner moves by
/// pointer, so the carry passes no byte of it through the payload scratch, and the swap block
/// makes no allocation and no free. The bits equal B's from frame 0.
fn a_delay_moves_by_pointer_at_96_khz(backend: Backend) {
    let what = format!("{backend:?}");
    let at_96k = |mut model: SessionModel| {
        model.sample_rate_hz = 96_000;
        model
    };
    let a = Session::compile(at_96k(tail_session()));
    let mut b_model = with_muted_track(tail_session());
    track_mut(&mut b_model, MUTED_TRACK).inserts.effects.clear();
    let b = Session::compile(at_96k(b_model));
    let feed = tail_feed();
    let reference = effect_run_blocks(&b, None, &feed, &[], &[], backend, TAIL_BLOCKS);
    let run = effect_run_blocks(
        &a,
        Some((&b, &a.model)),
        &feed,
        &[],
        &[],
        backend,
        TAIL_BLOCKS,
    );
    assert_clean_swap(&run, &what);
    assert_eq!(run.restarted, Vec::<Box<str>>::new(), "{what}");
    assert_eq!(
        run.payload_bytes, 0,
        "{what}: the delay's state was not copied"
    );
    assert_eq!(first_effect_difference(&run, &reference), None, "{what}");
}

#[test]
fn a_delay_moves_by_pointer_at_96_khz_at_every_width() {
    for backend in backends() {
        a_delay_moves_by_pointer_at_96_khz(backend);
    }
}

/// The payload counter reads what a lane move passes through the scratch: a per-node compressor
/// that joins a bank is copied, so the counter is nonzero there (gate 4's oracle can fail).
#[cfg(target_feature = "avx2")]
#[test]
fn the_payload_counter_counts_a_lane_move() {
    let backend = Backend::Simd8;
    let (a, b) = form_pair(Insert::Compressor, backend, true);
    let feed = bursty_feed(SOURCE, 5);
    let run = effect_run(&a, Some((&b, &a.model)), &feed, &[], &[], backend);
    assert_clean_swap(&run, "per node to bank");
    assert!(
        run.payload_bytes > 0,
        "a state that changes form goes through the scratch"
    );
}

/// A live difference on a moved per-node instance retargets it (#1280c D4): the delay's time and
/// a compressor's session bypass, each as one committed edit whose successor takes the instance
/// by pointer and then writes the new value. Run 1 is the structural swap with the retarget;
/// run 2 writes the same edit to A before the swap and prepares B from a base that holds it.
fn a_live_difference_on_a_moved_instance_is_retargeted(backend: Backend) {
    let ramp = LiveRamps::for_session(&tail_session()).for_row(LiveRampRow::BypassCrossfade);
    type Edit = fn(SessionModel) -> SessionModel;
    let edits: [(&str, Edit, EffectWrite); 2] = [
        (
            "delay time",
            |model| with_insert_param_all(model, "eq0", "dly", 1, 80.0),
            EffectWrite {
                block: SWAP_BLOCK,
                strip: "eq0",
                address: LiveEffectAddress::insert(0),
                value: EffectValue::Parameter {
                    index: 0,
                    value: 80.0,
                },
            },
        ),
        (
            "compressor bypass",
            |mut model| {
                track_mut(&mut model, "eq2").inserts.effects[0].bypass = true;
                model
            },
            EffectWrite {
                block: SWAP_BLOCK,
                strip: "eq2",
                address: LiveEffectAddress::insert(0),
                value: EffectValue::Bypass(BypassSwitch {
                    bypassed: true,
                    ramp_samples: ramp,
                }),
            },
        ),
    ];
    let feed = tail_feed();
    for (name, edit, write) in edits {
        let what = format!("{backend:?} {name}");
        let (a, b) = tail_pair();
        let b_edited = Session::compile(edit(b.model.clone()));
        let a_edited = edit(a.model.clone());
        let run = effect_run_blocks(
            &a,
            Some((&b_edited, &a.model)),
            &feed,
            &[],
            &[],
            backend,
            TAIL_BLOCKS,
        );
        assert_clean_swap(&run, &what);
        assert_eq!(run.restarted, Vec::<Box<str>>::new(), "{what}");
        assert_eq!(run.payload_bytes, 0, "{what}");
        let reference = effect_run_blocks(
            &a,
            Some((&b_edited, &a_edited)),
            &feed,
            &[write],
            &[],
            backend,
            TAIL_BLOCKS,
        );
        assert_clean_swap(&reference, &format!("{what} reference"));
        assert_eq!(
            first_effect_difference(&run, &reference),
            None,
            "{what}: move, then retarget equals live edit, then structural edit"
        );
        // Not vacuous: the moved instance keeps its old value without the retarget.
        let stale = effect_run_blocks(
            &a,
            Some((&b, &a.model)),
            &feed,
            &[],
            &[],
            backend,
            TAIL_BLOCKS,
        );
        assert_ne!(
            first_effect_difference(&stale, &run),
            None,
            "{what}: the edit is audible after the swap"
        );
    }
}

#[test]
fn a_live_difference_on_a_moved_instance_is_retargeted_at_every_width() {
    for backend in backends() {
        a_live_difference_on_a_moved_instance_is_retargeted(backend);
    }
}

// Issue #1327a (stream A of decision 15, D15-14): builtin meters carry across a plan swap. A
// meter whose strip, tap, metric set and configuration are the predecessor's keeps its open
// window, its counters and its sequence, and the host's reader keeps receiving. The oracle is the
// post-edit session prepared fresh and fed from frame 0, with the same meters.

mod meter_carry {
    use core::num::NonZeroUsize;
    use std::collections::BTreeMap;

    use builtins::{MeterMetricSet, MeterSnapshot, MeterTap};
    use builtins_compiler::MeterConsumer;
    use engine::realtime::{HostAttachment, ObserverCarry, ObserverKind, PlanAdoption};
    use host_core::{
        HostLiveControlRequest, HostMeterRequest, PreparedHost, SuccessorBase, pair_carried_meters,
        test_only_prepare_host_runtime_with_selected_meters_on,
    };

    use super::successor::{self, SUCCESSOR_REVISION};
    use super::*;

    const TAPS: [MeterTap; 7] = [
        MeterTap::Input,
        MeterTap::PostInputBuiltins,
        MeterTap::PostSimd1,
        MeterTap::PostDynamic,
        MeterTap::PostSimd2PreFader,
        MeterTap::PostFader,
        MeterTap::PostMatrix,
    ];
    /// Longer than the swap block's start, in blocks (a request's window is a whole number of
    /// blocks, #1581 D2): the window opened at sample 0 closes at 640 and the next one is open
    /// across the swap at sample 768.
    const PERIOD: u32 = 5;
    /// Another window length: two blocks.
    const SHORT_PERIOD: u32 = 2;
    const THIRD_TRACK: &str = "eq2";

    type Key = (String, MeterTap);

    fn live(blocks: u32) -> HostLiveControlRequest {
        HostLiveControlRequest {
            meter_window_blocks: blocks,
            meter_queue_depth: NonZeroUsize::new(16).expect("depth"),
            ..HostLiveControlRequest::default()
        }
    }

    fn requests(strips: &[&str]) -> Vec<HostMeterRequest> {
        strips
            .iter()
            .flat_map(|strip| {
                TAPS.iter().map(|tap| HostMeterRequest {
                    strip_id: (*strip).into(),
                    tap: *tap,
                    metrics: MeterMetricSet::ALL,
                })
            })
            .collect()
    }

    fn key(request: &HostMeterRequest) -> Key {
        (request.strip_id.to_string(), request.tap)
    }

    /// Three audible tracks.
    fn session_three() -> SessionModel {
        let mut model = session_a();
        add_track(&mut model, "eq0", THIRD_TRACK, SOURCE, false);
        model
    }

    /// The test's host attachment, as the browser's is: it holds the plan's meter readers and
    /// pairs them with the predecessor's in the swap block.
    struct Rig {
        meters: Vec<MeterConsumer>,
        carries: Vec<ObserverCarry>,
    }

    impl Rig {
        fn new(meters: Vec<MeterConsumer>) -> Box<Self> {
            Box::new(Self {
                meters,
                carries: Vec::with_capacity(256),
            })
        }
    }

    impl HostAttachment for Rig {
        fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
            self
        }

        fn adopt_predecessor(
            &mut self,
            predecessor: &mut dyn HostAttachment,
            carries: &[ObserverCarry],
        ) {
            let Some(predecessor) = predecessor.as_any_mut().downcast_mut::<Self>() else {
                return;
            };
            pair_carried_meters(&mut self.meters, &mut predecessor.meters, carries);
            // Within the capacity reserved at construction: no allocation.
            self.carries.extend_from_slice(carries);
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Reader {
        /// The host keeps the predecessor's consumers and the successor's, and reads both
        /// (gate 1): a carried meter's snapshots keep arriving on the predecessor's consumer.
        Direct,
        /// The readers live in the plans' attachments and pair in the swap block (gate 3).
        Paired,
    }

    struct Run {
        blocks: Vec<successor::Block>,
        /// Every snapshot read, by the reader's strip and tap, in arrival order.
        streams: BTreeMap<Key, Vec<MeterSnapshot>>,
        /// The successor's carry record after the swap block ([`Reader::Paired`] only).
        carries: Vec<ObserverCarry>,
        /// The successor's preparation-time pairing list (#1674 R1).
        pairing: Vec<ObserverCarry>,
    }

    fn drain(consumers: &mut [MeterConsumer], streams: &mut BTreeMap<Key, Vec<MeterSnapshot>>) {
        for consumer in consumers {
            while let Ok(snapshot) = consumer.consumer.try_pop() {
                streams
                    .entry((consumer.track_id.to_string(), consumer.tap))
                    .or_default()
                    .push(snapshot);
            }
        }
    }

    fn prepare(
        session: &Session,
        live: &HostLiveControlRequest,
        meters: &[HostMeterRequest],
        base: Option<SuccessorBase<'_>>,
        backend: Backend,
    ) -> (PreparedHost, host_core::HostLiveControlHandles) {
        test_only_prepare_host_runtime_with_selected_meters_on(
            &session.compiled,
            &caps(),
            live,
            meters,
            base,
            backend,
        )
        .unwrap_or_else(|failure| panic!("prepare: {failure:?}"))
    }

    /// `a` rendered for [`SWAP_BLOCK`] blocks, `b` prepared as its successor with its own meter
    /// request and swapped in, rendered to [`BLOCKS`].
    #[allow(clippy::too_many_arguments)]
    fn swapped(
        backend: Backend,
        a: &Session,
        a_live: &HostLiveControlRequest,
        a_meters: &[HostMeterRequest],
        b: &Session,
        b_live: &HostLiveControlRequest,
        b_meters: &[HostMeterRequest],
        reader: Reader,
    ) -> Run {
        let feed = Feed::new(SOURCE, 1);
        let (mut a_prepared, mut a_handles) = prepare(a, a_live, a_meters, None, backend);
        let mut a_direct = Vec::new();
        match reader {
            Reader::Direct => a_direct = core::mem::take(&mut a_handles.meters),
            Reader::Paired => a_prepared
                .plan
                .set_host_attachment(Rig::new(core::mem::take(&mut a_handles.meters))),
        }
        let rate = a_prepared.report.sample_rate_hz;
        let PreparedHost {
            plan,
            sources: mut a_sources,
            inventory,
            ..
        } = a_prepared;
        let (mut publisher, mut owner, retirer) = successor::exchange(plan);
        let mut streams = BTreeMap::new();
        let mut blocks = Vec::new();
        feed.submit(&mut a_sources, rate, 0);
        for block in 0..SWAP_BLOCK {
            feed.submit(&mut a_sources, rate, block + 1);
            blocks.push(successor::render(&mut owner));
            match reader {
                Reader::Direct => drain(&mut a_direct, &mut streams),
                Reader::Paired => {
                    let rig = owner.active_attachment_mut::<Rig>().expect("attachment");
                    drain(&mut rig.meters, &mut streams);
                }
            }
        }
        let (mut b_prepared, mut b_handles) = prepare(
            b,
            b_live,
            b_meters,
            Some(SuccessorBase::new(&inventory, &a.model, SUCCESSOR_REVISION)),
            backend,
        );
        b_prepared.sources.adopt_persisting(&mut a_sources);
        let mut b_direct = Vec::new();
        match reader {
            Reader::Direct => b_direct = core::mem::take(&mut b_handles.meters),
            Reader::Paired => b_prepared
                .plan
                .set_host_attachment(Rig::new(core::mem::take(&mut b_handles.meters))),
        }
        let PreparedHost {
            mut plan,
            sources: mut b_sources,
            inventory: b_inventory,
            ..
        } = b_prepared;
        plan.take_revision_gate()
            .expect("the successor's handle")
            .publish(SUCCESSOR_REVISION);
        publisher
            .reserve_replacement(plan, PlanAdoption::Next)
            .unwrap_or_else(|_| panic!("reserve the successor"))
            .commit(SUCCESSOR_REVISION);
        for block in SWAP_BLOCK..BLOCKS {
            if block + 1 < BLOCKS {
                feed.submit(&mut b_sources, rate, block + 1);
            }
            blocks.push(successor::render(&mut owner));
            match reader {
                Reader::Direct => {
                    drain(&mut a_direct, &mut streams);
                    drain(&mut b_direct, &mut streams);
                }
                Reader::Paired => {
                    let rig = owner.active_attachment_mut::<Rig>().expect("attachment");
                    drain(&mut rig.meters, &mut streams);
                }
            }
        }
        let carries = match reader {
            Reader::Direct => Vec::new(),
            Reader::Paired => owner
                .active_attachment_mut::<Rig>()
                .expect("attachment")
                .carries
                .clone(),
        };
        drop((retirer, publisher, a_handles, b_handles));
        Run {
            blocks,
            streams,
            carries,
            pairing: b_inventory.meter_pairing().to_vec(),
        }
    }

    /// `b` prepared fresh with `meters` and fed from frame 0.
    fn reference(
        backend: Backend,
        b: &Session,
        live: &HostLiveControlRequest,
        meters: &[HostMeterRequest],
    ) -> BTreeMap<Key, Vec<MeterSnapshot>> {
        let feed = Feed::new(SOURCE, 1);
        let (prepared, mut handles) = prepare(b, live, meters, None, backend);
        let rate = prepared.report.sample_rate_hz;
        let PreparedHost {
            plan, mut sources, ..
        } = prepared;
        let (_publisher, mut owner, _retirer) = successor::exchange(plan);
        let mut streams = BTreeMap::new();
        feed.submit(&mut sources, rate, 0);
        for block in 0..BLOCKS {
            if block + 1 < BLOCKS {
                feed.submit(&mut sources, rate, block + 1);
            }
            successor::render(&mut owner);
            drain(&mut handles.meters, &mut streams);
        }
        streams
    }

    /// A snapshot with the fields a swap legitimately changes set to the reference's: a snapshot
    /// the predecessor published before the swap names the predecessor's handle.
    fn same_but_handle(snapshot: &MeterSnapshot, reference: &MeterSnapshot) -> MeterSnapshot {
        MeterSnapshot {
            handle: reference.handle,
            ..*snapshot
        }
    }

    const SWAP_SAMPLE: u64 = (SWAP_BLOCK * QUANTUM) as u64;

    /// Gates 1 and 3 at one width and reader. Strips: A has three tracks, B adds a muted one that
    /// sorts first, so every carried meter's reader index shifts by [`TAPS`].len().
    fn meters_carry(backend: Backend, reader: Reader) {
        let a = Session::compile(session_three());
        let mut b_model = session_three();
        add_track(&mut b_model, "eq0", MUTED_TRACK, SOURCE, true);
        let b = Session::compile(b_model);
        let a_strips = ["eq0", "eq1", THIRD_TRACK];
        let b_strips = [MUTED_TRACK, "eq0", "eq1", THIRD_TRACK];
        let (a_meters, b_meters) = (requests(&a_strips), requests(&b_strips));
        let expected = reference(backend, &b, &live(PERIOD), &b_meters);
        let run = swapped(
            backend,
            &a,
            &live(PERIOD),
            &a_meters,
            &b,
            &live(PERIOD),
            &b_meters,
            reader,
        );
        assert_eq!(run.blocks[SWAP_BLOCK].carry, CarryOutcome::Carried);
        assert_eq!(run.blocks[SWAP_BLOCK].swap, SwapOutcome::Applied);
        assert!(!expected.is_empty());

        let carried: Vec<Key> = a_meters.iter().map(key).collect();
        for k in &carried {
            let got = &run.streams[k];
            let want = &expected[k];
            assert_eq!(
                got.len(),
                want.len(),
                "{backend:?} {reader:?} {k:?}: every window arrives once, none lost or doubled"
            );
            for (index, (got, want)) in got.iter().zip(want).enumerate() {
                assert_eq!(
                    got.window_sequence, index as u64,
                    "{backend:?} {reader:?} {k:?}: no gap in the sequence"
                );
                assert_eq!(got.cumulative_discontinuities, 0, "{k:?}: no discontinuity");
                assert_eq!(got.reset_generation, want.reset_generation, "{k:?}");
                assert_eq!(
                    same_but_handle(got, want),
                    *want,
                    "{backend:?} {reader:?} {k:?}: window {index} equals the reference field by field"
                );
                if got.end_sample > SWAP_SAMPLE {
                    assert_eq!(
                        got.handle, want.handle,
                        "{k:?}: the carried state names the reader that receives it"
                    );
                }
            }
            assert!(
                got.iter()
                    .any(|s| s.start_sample < SWAP_SAMPLE && s.end_sample > SWAP_SAMPLE),
                "{k:?}: a window spans the swap, or the carry is not tested"
            );
        }

        // The added muted track's meters join the carried grid (D6). Its pre-fader taps see the
        // source, which the strip did not have before adoption, so only the grid and, where the
        // strip is silent in the reference too, every field equal the reference's.
        for tap in TAPS {
            let k: Key = (MUTED_TRACK.to_string(), tap);
            let got = &run.streams[&k];
            assert!(!got.is_empty(), "{k:?}: the added meter publishes");
            for snapshot in got {
                let want = expected[&k]
                    .iter()
                    .find(|want| want.window_sequence == snapshot.window_sequence)
                    .unwrap_or_else(|| {
                        panic!(
                            "{k:?}: the reference has window {}",
                            snapshot.window_sequence
                        )
                    });
                assert_eq!(
                    (
                        snapshot.start_sample,
                        snapshot.end_sample,
                        snapshot.frames,
                        snapshot.handle
                    ),
                    (want.start_sample, want.end_sample, want.frames, want.handle),
                    "{backend:?} {reader:?} {k:?}: on the carried window grid from its first snapshot"
                );
                assert_eq!(snapshot.cumulative_discontinuities, 0);
                if matches!(tap, MeterTap::PostFader | MeterTap::PostMatrix) {
                    assert_eq!(snapshot, want, "{k:?}: a muted strip is silent in both");
                }
            }
            assert_eq!(
                got[0].window_sequence, 1,
                "{k:?}: it joined the open window"
            );
        }

        if reader == Reader::Paired {
            let index_in = |meters: &[HostMeterRequest], k: &Key| {
                meters
                    .iter()
                    .position(|request| key(request) == *k)
                    .expect("key")
            };
            assert_eq!(
                run.carries.len(),
                a_meters.len(),
                "one record per carried meter"
            );
            let meter_entries: Vec<ObserverCarry> = run
                .carries
                .iter()
                .filter(|carry| carry.kind == ObserverKind::Meter)
                .copied()
                .collect();
            assert_eq!(
                run.pairing, meter_entries,
                "#1674 R1: the meter rows equal the swap block's record's meter entries"
            );
            for carry in &run.carries {
                assert_eq!(carry.kind, ObserverKind::Meter);
                let k = key(&a_meters[carry.predecessor]);
                assert_eq!(
                    index_in(&b_meters, &k),
                    carry.successor,
                    "{k:?}: the record names the successor's reader of the same strip and tap"
                );
            }
        }
    }

    #[test]
    fn meters_carry_through_the_predecessors_readers_on_every_width() {
        for backend in backends() {
            meters_carry(backend, Reader::Direct);
        }
    }

    #[test]
    fn paired_readers_follow_the_record_on_every_width() {
        for backend in backends() {
            meters_carry(backend, Reader::Paired);
        }
    }

    /// Gate 2: a changed window length starts every meter fresh, and a changed key starts that
    /// meter fresh alone.
    #[test]
    fn a_changed_meter_starts_fresh_and_the_others_carry() {
        let backend = Backend::Simd4;
        let a = Session::compile(session_three());
        let strips = ["eq0", "eq1", THIRD_TRACK];
        let meters = requests(&strips);

        // A different window length: nothing carries, and the first window opens at adoption.
        let run = swapped(
            backend,
            &a,
            &live(PERIOD),
            &meters,
            &a,
            &live(SHORT_PERIOD),
            &meters,
            Reader::Paired,
        );
        assert!(
            run.carries.is_empty(),
            "no meter carries across a changed window length"
        );
        for (k, stream) in &run.streams {
            let fresh: Vec<_> = stream
                .iter()
                .filter(|s| s.start_sample >= SWAP_SAMPLE)
                .collect();
            assert_eq!(fresh[0].window_sequence, 0, "{k:?}: a new sequence");
            assert_eq!(
                fresh[0].start_sample, SWAP_SAMPLE,
                "{k:?}: opened at the adoption block"
            );
            assert_eq!(
                fresh[0].frames,
                SHORT_PERIOD * QUANTUM as u32,
                "{k:?}: the new window length"
            );
        }

        // One meter keeps another metric set: that meter alone is not in the record.
        let mut changed = requests(&strips);
        let at = 9;
        assert_eq!(key(&changed[at]), ("eq1".to_string(), TAPS[2]));
        changed[at].metrics = MeterMetricSet::SAMPLE_PEAK;
        let reference_streams = reference(backend, &a, &live(PERIOD), &changed);
        let run = swapped(
            backend,
            &a,
            &live(PERIOD),
            &meters,
            &a,
            &live(PERIOD),
            &changed,
            Reader::Paired,
        );
        assert_eq!(run.carries.len(), meters.len() - 1);
        assert!(
            run.carries
                .iter()
                .all(|carry| carry.successor != at && carry.predecessor != at)
        );
        for (index, request) in changed.iter().enumerate() {
            let k = key(request);
            let got = &run.streams[&k];
            let want = &reference_streams[&k];
            if index == at {
                // Fresh, on the carried grid (D6): the window it joined holds the swap block on.
                // [the predecessor's window 0, the successor's fresh meter's window 1]
                assert_eq!(got.len(), 2, "{k:?}");
                assert_eq!(got[1].window_sequence, 1);
                assert_eq!(got[1].present_metrics, MeterMetricSet::SAMPLE_PEAK);
            } else {
                assert_eq!(got.len(), want.len(), "{k:?}");
                for (got, want) in got.iter().zip(want) {
                    assert_eq!(same_but_handle(got, want), *want, "{k:?}");
                }
            }
        }
    }

    /// #1891: B removes track `eq0` and adds a silent, unfed submix named `eq0`. A meter's key is
    /// the strip ID, tap and metric set, which the submix's meters share with the track's; but
    /// the submix is another strip, so its meters start fresh (on the carried window grid, D6)
    /// and equal a fresh B's, and no carry record names their readers. Red if a meter pairs by
    /// strip ID alone: the submix's reader receives the track's open window, with its peak. B
    /// also changes `eq1`'s trim, a track in both: its meters still carry. Red if the kind test
    /// compares the strips' contents, not their kinds (`eq1`'s seven meters restart).
    #[test]
    fn a_track_replaced_by_a_submix_of_its_name_carries_no_meter() {
        let a = Session::compile(session_three());
        let mut b_model = session_three();
        let template = b_model
            .routes
            .iter()
            .find(|route| route.id.as_str() == "eq0-main")
            .expect("eq0-main")
            .clone();
        remove_track(&mut b_model, "eq0");
        let mut out = template;
        let RouteSource::Track { tap, .. } = out.source else {
            panic!("a track route");
        };
        out.source = RouteSource::Submix {
            submix_id: id("eq0"),
            tap,
        };
        b_model.routes.push(out);
        b_model
            .submixes
            .push(session::Submix::unity(id("eq0"), &b_model.console));
        track_mut(&mut b_model, "eq1").builtins.left.trim_db -= 1.0;
        let b = Session::compile(b_model);
        let meters = requests(&["eq0", "eq1", THIRD_TRACK]);
        // The submix's meters are the first readers.
        assert_eq!(key(&meters[0]), ("eq0".to_string(), TAPS[0]));
        for backend in backends() {
            let expected = reference(backend, &b, &live(PERIOD), &meters);
            let run = swapped(
                backend,
                &a,
                &live(PERIOD),
                &meters,
                &b,
                &live(PERIOD),
                &meters,
                Reader::Paired,
            );
            assert_eq!(run.blocks[SWAP_BLOCK].carry, CarryOutcome::Carried);
            assert_eq!(
                run.carries.len(),
                meters.len() - TAPS.len(),
                "{backend:?}: every track meter carries, the submix's do not"
            );
            assert!(
                run.carries
                    .iter()
                    .all(|carry| carry.successor >= TAPS.len()),
                "{backend:?}: no record names the submix's readers"
            );
            for tap in TAPS {
                let k: Key = ("eq0".to_string(), tap);
                let after: Vec<&MeterSnapshot> = run.streams[&k]
                    .iter()
                    .filter(|snapshot| snapshot.end_sample > SWAP_SAMPLE)
                    .collect();
                assert!(!after.is_empty(), "{backend:?} {k:?}: the submix publishes");
                assert_eq!(
                    after[0].window_sequence, 1,
                    "{backend:?} {k:?}: it joined the open window of the grid"
                );
                for snapshot in after {
                    let want = expected[&k]
                        .iter()
                        .find(|want| want.window_sequence == snapshot.window_sequence)
                        .unwrap_or_else(|| panic!("{k:?}: window {}", snapshot.window_sequence));
                    assert_eq!(
                        snapshot, want,
                        "{backend:?} {k:?}: the silent submix's window equals a fresh B's"
                    );
                }
            }
        }
    }

    /// Gate 5: the swap block, with the hook's pairing in it, allocates and frees nothing.
    #[test]
    fn the_meter_swap_block_allocates_and_frees_nothing() {
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
        let a = Session::compile(session_three());
        let mut b_model = session_three();
        add_track(&mut b_model, "eq0", MUTED_TRACK, SOURCE, true);
        let b = Session::compile(b_model);
        let run = swapped(
            Backend::current(),
            &a,
            &live(PERIOD),
            &requests(&["eq0", "eq1", THIRD_TRACK]),
            &b,
            &live(PERIOD),
            &requests(&[MUTED_TRACK, "eq0", "eq1", THIRD_TRACK]),
            Reader::Paired,
        );
        assert_eq!(run.blocks[SWAP_BLOCK].carry, CarryOutcome::Carried);
        assert!(!run.carries.is_empty());
        for (index, block) in run.blocks.iter().enumerate().skip(1) {
            assert_eq!(block.allocator, (0, 0, 0), "block {index}: allocator calls");
            assert_eq!(block.audit, (0, 0), "block {index}: render audit");
        }
    }
}

// ---------------------------------------------------------------------------------------------
// #1327b: an effect's observation taps follow their effect instance across a swap.
//
// The tap's whole state moves with the instance: its arm state, its open window, its sequence and
// its publisher. The reader the host holds for the predecessor's tap is paired with the
// successor's in the swap block, so it keeps receiving, and a subscribe that was acknowledged
// before the swap is armed on the successor's tap after it.

mod observation_carry {
    use std::collections::BTreeMap;

    use engine::realtime::{HostAttachment, ObservationWindow, ObserverCarry, ObserverKind};
    use host_core::{EffectObservationHandle, pair_carried_observations};

    use super::*;

    /// Blocks per window. The swap block (6) falls inside the window of blocks 4..8, so a window
    /// is open across the swap and it publishes after it.
    const WINDOW_BLOCKS: u32 = 4;

    /// The test's host attachment, as the browser's is: it holds the plan's observation readers
    /// and pairs them with the predecessor's in the swap block.
    struct Rig {
        observations: Vec<EffectObservationHandle>,
        carries: Vec<ObserverCarry>,
    }

    impl Rig {
        fn new(observations: Vec<EffectObservationHandle>) -> Box<Self> {
            let capacity = observations.len() + 8;
            Box::new(Self {
                observations,
                carries: Vec::with_capacity(capacity),
            })
        }

        /// The strip of each reader set, in reader order.
        fn strips(&self) -> Vec<String> {
            self.observations
                .iter()
                .map(|handle| handle.track_id.to_string())
                .collect()
        }
    }

    impl HostAttachment for Rig {
        fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
            self
        }

        fn adopt_predecessor(
            &mut self,
            predecessor: &mut dyn HostAttachment,
            carries: &[ObserverCarry],
        ) {
            let Some(predecessor) = predecessor.as_any_mut().downcast_mut::<Self>() else {
                return;
            };
            pair_carried_observations(
                &mut self.observations,
                &mut predecessor.observations,
                carries,
            );
            // Within the capacity reserved at construction: no allocation. The strip meters
            // (#1581 D1) carry beside the taps; the record kept is the taps'.
            self.carries.extend(
                carries
                    .iter()
                    .filter(|carry| carry.kind == ObserverKind::Observation),
            );
        }
    }

    type Streams = BTreeMap<String, Vec<ObservationWindow>>;

    /// The windows the readers of `plan` hold that they have not shown before, by strip.
    fn read_all(plan: &mut LivePlan, streams: &mut Streams) {
        let rig = plan
            .prepared
            .plan
            .host_attachment_mut::<Rig>()
            .expect("the plan's attachment");
        for handle in &rig.observations {
            let Some(window) = handle.readers[0].read() else {
                continue;
            };
            let stream = streams.entry(handle.track_id.to_string()).or_default();
            // A window is new when its sequence or its start differs: a restarted tap's sequence
            // begins at 1 again, after the predecessor's.
            if stream.last().is_none_or(|last| *last != window) {
                stream.push(window);
            }
        }
    }

    /// Subscribe to the gain-reduction tap of each of `strips`' compressor insert, as an app
    /// does: a record in the insert's observation queue, acknowledged when it is queued.
    fn subscribe(plan: &mut LivePlan, strips: &[String]) {
        for strip in strips {
            plan.handles
                .effect_control_mut(strip, LiveEffectAddress::insert(0))
                .expect("a control channel for the insert")
                .try_push_observe(0, true, WINDOW_BLOCKS, 0)
                .expect("room in the queue");
        }
    }

    /// Disarm the same tap, as an app does.
    fn unsubscribe(plan: &mut LivePlan, strips: &[String]) {
        for strip in strips {
            plan.handles
                .effect_control_mut(strip, LiveEffectAddress::insert(0))
                .expect("a control channel for the insert")
                .try_push_observe(0, false, WINDOW_BLOCKS, 0)
                .expect("room in the queue");
        }
    }

    /// The bindings of `strips`' compressor tap 1 (the gain-reduction tap), each armed with
    /// [`WINDOW_BLOCKS`]: what the host's table holds after the subscribes above.
    fn bindings_of(strips: &[String]) -> controls::ObservationBindings {
        let mut bindings = controls::ObservationBindings::new();
        for strip in strips {
            bindings.set(tap_key(strip), WINDOW_BLOCKS);
        }
        bindings
    }

    /// The stable key of `strip`'s compressor insert `comp`, gain-reduction tap.
    fn tap_key(strip: &str) -> controls::TapKey {
        controls::TapKey {
            strip: id(strip),
            section: controls::TapSection::Inserts(id("comp")),
            tap_id: 1,
        }
    }

    /// Attach `plan`'s observation readers to it, as the browser does.
    fn attach(plan: &mut LivePlan) {
        let observations = core::mem::take(&mut plan.handles.effect_observations);
        plan.prepared
            .plan
            .set_host_attachment(Rig::new(observations));
    }

    /// `session` prepared fresh, subscribed to `subscribed` at its first block, and fed from
    /// frame 0: every window each strip's tap publishes.
    fn reference(backend: Backend, session: &Session, subscribed: &[String]) -> Streams {
        let feed = bursty_feed(SOURCE, 5);
        let mut plan = LivePlan::fresh_with(session, &observed_request(), backend);
        subscribe(&mut plan, subscribed);
        attach(&mut plan);
        let mut streams = Streams::new();
        for block in 0..BLOCKS {
            plan.submit(&feed, QUANTUM, block);
            plan.render(QUANTUM);
            read_all(&mut plan, &mut streams);
        }
        streams
    }

    struct Swapped {
        streams: Streams,
        carry: CarryOutcome,
        /// The successor's carry record, as its attachment's hook saw it in the swap block.
        carries: Vec<ObserverCarry>,
        /// The strip of each reader set of A and of B.
        a_strips: Vec<String>,
        b_strips: Vec<String>,
        restarted: Vec<Box<str>>,
        /// Armed taps of the successor and of the predecessor after the swap block.
        armed: [u64; 2],
        /// Each successor tap's published state at the end of the run, by strip (#1585 D2).
        states: BTreeMap<String, engine::realtime::ObservationState>,
        /// The allocator's `(allocations, reallocations, deallocations)` and the render audit's
        /// `(allocations, deallocations)` over the swap block: hand-over, hook and render.
        allocator: ((u64, u64, u64), (u64, u64)),
    }

    /// A rendered for [`SWAP_BLOCK`] blocks with `subscribed` acknowledged before its first, B
    /// prepared as its successor and swapped in at [`SWAP_BLOCK`], rendered to [`BLOCKS`]. B
    /// subscribes to nothing: whatever its taps hold at the swap is what the carry brought.
    fn swapped(backend: Backend, a: &Session, b: &Session, subscribed: &[String]) -> Swapped {
        swapped_with(
            backend,
            a,
            b,
            &Swap {
                early: subscribed,
                ..Swap::default()
            },
        )
    }

    /// What a swapped run subscribes, when, and what the host's table holds at the swap.
    #[derive(Default)]
    struct Swap<'a> {
        /// Subscribes acknowledged before A's first block.
        early: &'a [String],
        /// Subscribes acknowledged after A's last block: still in A's queue at the swap block.
        late: &'a [String],
        /// Disarms acknowledged after A's last block, likewise.
        late_disarm: &'a [String],
        /// Subscribes pushed into B's own queue before its first block: the reference for a tap
        /// armed at preparation.
        successor_records: &'a [String],
        /// Disarms pushed into B's own queue before the swap (#1585): the browser's unsubscribe
        /// admitted against the successor between its commit and its adoption block.
        successor_disarms: &'a [String],
        /// The taps the host's table holds (#1584 D6a); none by default.
        bindings: Option<&'a controls::ObservationBindings>,
    }

    fn swapped_with(backend: Backend, a: &Session, b: &Session, swap: &Swap<'_>) -> Swapped {
        use bench_support::alloc as bench_alloc;
        let feed = bursty_feed(SOURCE, 5);
        let request = observed_request();
        let mut streams = Streams::new();
        let mut plan = LivePlan::fresh_with(a, &request, backend);
        subscribe(&mut plan, swap.early);
        attach(&mut plan);
        for block in 0..SWAP_BLOCK {
            plan.submit(&feed, QUANTUM, block);
            plan.render(QUANTUM);
            read_all(&mut plan, &mut streams);
        }
        subscribe(&mut plan, swap.late);
        unsubscribe(&mut plan, swap.late_disarm);
        let mut successor = LivePlan::successor_observed(
            b,
            &request,
            &plan.prepared.inventory,
            &a.model,
            swap.bindings.unwrap_or(&host_core::NO_OBSERVATION),
            &EditRamps::new(),
            backend,
        );
        subscribe(&mut successor, swap.successor_records);
        unsubscribe(&mut successor, swap.successor_disarms);
        attach(&mut successor);
        assert_eq!(
            successor
                .prepared
                .sources
                .adopt_persisting(&mut plan.prepared.sources),
            1
        );
        successor.submit(&feed, QUANTUM, SWAP_BLOCK);
        let mut output = vec![f32::NAN; QUANTUM * 2];
        engine::realtime::audit::reset();
        let mark = bench_alloc::current_thread_counters();
        let carry = successor
            .prepared
            .plan
            .adopt_predecessor_plan(&mut plan.prepared.plan);
        successor.render_into(&mut output, QUANTUM);
        let delta = bench_alloc::current_thread_delta_since(mark);
        let audit = engine::realtime::audit::snapshot();
        read_all(&mut successor, &mut streams);
        for block in SWAP_BLOCK + 1..BLOCKS {
            successor.submit(&feed, QUANTUM, block);
            successor.render(QUANTUM);
            read_all(&mut successor, &mut streams);
        }
        let strips = |plan: &mut LivePlan| {
            plan.prepared
                .plan
                .host_attachment_mut::<Rig>()
                .expect("attachment")
                .strips()
        };
        Swapped {
            streams,
            carry,
            carries: successor
                .prepared
                .plan
                .host_attachment_mut::<Rig>()
                .expect("attachment")
                .carries
                .clone(),
            a_strips: strips(&mut plan),
            b_strips: strips(&mut successor),
            restarted: successor.prepared.restarted_strips().to_vec(),
            armed: [
                successor.prepared.plan.observation_binding_counts()[2],
                plan.prepared.plan.observation_binding_counts()[2],
            ],
            states: successor
                .prepared
                .plan
                .host_attachment_mut::<Rig>()
                .expect("attachment")
                .observations
                .iter()
                .map(|handle| {
                    let state = handle.readers[0].read_state().expect("a whole read");
                    (handle.track_id.to_string(), state)
                })
                .collect(),
            allocator: (
                (delta.allocations, delta.reallocations, delta.deallocations),
                (audit.allocations, audit.deallocations),
            ),
        }
    }

    fn strips_of(session: &Session) -> Vec<String> {
        session
            .model
            .tracks
            .iter()
            .map(|track| track.id.as_str().to_owned())
            .collect()
    }

    /// Which sessions the swap runs between.
    #[derive(Clone, Copy, Debug)]
    enum Form {
        /// Per-node instances on both sides: A's inserts and B's, with one more.
        NodeToNode,
        /// A bank of lanes on both sides: B adds a track with no insert.
        BankToBank,
        /// Per-node instances become a bank of lanes.
        NodeToBank,
        /// A bank of lanes becomes per-node instances.
        BankToNode,
    }

    fn pair(backend: Backend, form: Form) -> (Session, Session) {
        let width = bank_width(backend);
        match form {
            Form::NodeToNode => {
                // Per node on both sides (#888): A's group is one member below the insert padding
                // threshold, and B's added track misaligns the group it reaches (its EQ and
                // compressor are B's two extra per-node instances).
                let k = insert_pad_threshold(backend);
                let short = compressor_insert_session(k - 1);
                let (a, b) = (
                    Session::compile(short.clone()),
                    Session::compile(with_misaligned_muted_track(short)),
                );
                assert_eq!(insert_forms(&a, backend), vec![true; k - 1]);
                assert_eq!(insert_forms(&b, backend), vec![true; k + 1]);
                (a, b)
            }
            Form::BankToBank => {
                let full = compressor_insert_session(width);
                let mut bare = with_muted_track(full.clone());
                track_mut(&mut bare, MUTED_TRACK).inserts.effects.clear();
                let (a, b) = (Session::compile(full), Session::compile(bare));
                assert_eq!(insert_forms(&a, backend), vec![false; width]);
                assert_eq!(insert_forms(&b, backend), vec![false; width]);
                (a, b)
            }
            Form::NodeToBank => form_pair(Insert::Compressor, backend, true),
            Form::BankToNode => form_pair(Insert::Compressor, backend, false),
        }
    }

    /// Gate 4, and the acknowledged subscribe: a tap subscribed before the swap publishes the
    /// windows a fresh B subscribed from frame 0 does, field for field, across the swap, with
    /// no restart and no gap in its sequence; the host's reader was paired by the record, which
    /// names each tap by its effect owner, not its position.
    fn taps_carry(backend: Backend, form: Form) {
        let what = format!("{backend:?} {form:?}");
        let (a, b) = pair(backend, form);
        let subscribed = strips_of(&a);
        // A strip B removes has no instance to carry, and its tap goes with it.
        let kept: Vec<String> = subscribed
            .iter()
            .filter(|strip| strips_of(&b).contains(strip))
            .cloned()
            .collect();
        let expected = reference(backend, &b, &kept);
        let run = swapped(backend, &a, &b, &subscribed);
        assert_eq!(run.carry, CarryOutcome::Carried, "{what}");
        assert_eq!(run.restarted, Vec::<Box<str>>::new(), "{what}");
        assert_eq!(
            run.armed,
            [kept.len() as u64, (subscribed.len() - kept.len()) as u64],
            "{what}: the subscribes are armed on the successor's taps; the retiring predecessor \
             holds fresh unarmed ones and the tap of a removed strip"
        );
        assert_eq!(
            run.allocator,
            ((0, 0, 0), (0, 0)),
            "{what}: allocator and render audit over the swap block"
        );

        assert_eq!(
            run.carries.len(),
            kept.len(),
            "{what}: one record per tap set"
        );
        let mut shifted = false;
        for carry in &run.carries {
            assert_eq!(carry.kind, ObserverKind::Observation, "{what}");
            assert_eq!(
                run.a_strips[carry.predecessor], run.b_strips[carry.successor],
                "{what}: the record joins the two reader sets of one strip"
            );
            shifted |= carry.predecessor != carry.successor;
        }
        assert_eq!(
            shifted,
            matches!(form, Form::NodeToNode | Form::NodeToBank | Form::BankToNode),
            "{what}: the added track shifts every reader index, or this swap shifts none"
        );

        for strip in &kept {
            let got = &run.streams[strip];
            let want = &expected[strip];
            assert_eq!(
                got, want,
                "{what} {strip}: the windows equal a fresh B's, the one open at the swap included"
            );
            assert_eq!(
                got.iter().map(|window| window.sequence).collect::<Vec<_>>(),
                vec![1, 2, 3],
                "{what} {strip}: no gap and no restart"
            );
            assert!(
                got.iter()
                    .any(|window| window.left > 0.0 || window.right > 0.0),
                "{what} {strip}: the compressor reduces gain, or the readings are vacuous"
            );
        }
        assert!(
            expected
                .values()
                .any(|windows| windows[1].first_sample < SWAP_SAMPLE
                    && windows[1].end_sample > SWAP_SAMPLE),
            "{what}: a window spans the swap"
        );
        // The added track's tap was not subscribed and is no carried one's: it publishes nothing.
        for strip in run
            .b_strips
            .iter()
            .filter(|strip| !subscribed.contains(strip))
        {
            assert!(!run.streams.contains_key(strip), "{what}: {strip} is fresh");
        }
    }

    const SWAP_SAMPLE: u64 = (SWAP_BLOCK * QUANTUM) as u64;

    /// At the build's own width, `Backend::current()`: a console effect bank binds only at the
    /// width its build executes (#1017).
    #[test]
    fn taps_follow_their_effect_through_every_form_at_the_build_width() {
        let backend = Backend::current();
        for form in [
            Form::NodeToNode,
            Form::BankToBank,
            Form::NodeToBank,
            Form::BankToNode,
        ] {
            taps_carry(backend, form);
        }
    }

    /// Gate 4, second case: a compressor whose prepared setup changed (its sidechain) restarts,
    /// and so does its tap: it is not in the record, and its successor reader receives nothing
    /// while every other strip's tap carries.
    fn a_restarted_effect_starts_its_tap_fresh(backend: Backend) {
        let what = format!("{backend:?}");
        let width = bank_width(backend);
        let base = compressor_insert_session(width - 2);
        let mut changed = with_muted_track(base.clone());
        route_own_sidechain(&mut changed, "eq1");
        let (a, b) = (Session::compile(base), Session::compile(changed));
        let subscribed = strips_of(&a);
        let run = swapped(backend, &a, &b, &subscribed);
        assert_eq!(run.carry, CarryOutcome::Carried, "{what}");
        assert_eq!(run.restarted, vec![Box::<str>::from("eq1")], "{what}");
        assert_eq!(run.carries.len(), subscribed.len() - 1, "{what}");
        assert!(
            run.carries
                .iter()
                .all(|carry| run.a_strips[carry.predecessor] != "eq1"
                    && run.b_strips[carry.successor] != "eq1"),
            "{what}: eq1's taps are not in the record"
        );
        let expected = reference(backend, &b, &subscribed);
        for strip in subscribed.iter().filter(|strip| *strip != "eq1") {
            assert_eq!(run.streams[strip], expected[strip], "{what} {strip}");
        }
        assert_eq!(
            run.streams["eq1"]
                .iter()
                .map(|window| window.sequence)
                .collect::<Vec<_>>(),
            vec![1],
            "{what}: eq1 published its first window in A, and its fresh tap publishes none"
        );
        assert_eq!(
            run.armed[0],
            subscribed.len() as u64 - 1,
            "{what}: the restarted effect's tap starts unarmed"
        );
    }

    /// At the build's own width, `Backend::current()`: a console effect bank binds only at the
    /// width its build executes (#1017).
    #[test]
    fn a_restarted_effect_starts_its_tap_fresh_at_the_build_width() {
        a_restarted_effect_starts_its_tap_fresh(Backend::current());
    }

    /// Route strip `strip`'s first insert's sidechain from the strip's own input: a prepared
    /// change of the instance (a new graph edge), which restarts it.
    fn route_own_sidechain(model: &mut SessionModel, strip: &str) {
        track_mut(model, strip).inserts.effects[0].sidechain =
            SidechainDeclaration::Routed(session::Sidechain {
                source: RouteSource::Track {
                    track_id: id(strip),
                    tap: session::SendTap::Input,
                },
                port_id: id("sidechain-in"),
            });
    }

    /// A compressor insert whose sidechain declaration changed restarts, with a base that holds it
    /// and a successor that is the base plus one track. (Its link mode is a live value since
    /// #1371, so a link change carries.)
    fn restarted_pair(backend: Backend) -> (Session, Session) {
        let base = compressor_insert_session(bank_width(backend) - 2);
        let mut changed = with_muted_track(base.clone());
        route_own_sidechain(&mut changed, "eq1");
        (Session::compile(base), Session::compile(changed))
    }

    /// #1584 D6a gates 1 and 2: a subscription the host acknowledged survives a rebuild that
    /// restarts its effect, whether the record was applied before the swap or still waits in the
    /// predecessor's queue. The restarted tap is armed at preparation and opens its first window at
    /// the adoption block's first sample, and every window equals the one a record in the
    /// successor's own queue, applied at that block, produces from the same state and input.
    ///
    /// Red mutation: prepare the successor's taps unarmed (drop the arm in
    /// `attach_effect_observation`) -> eq1 publishes no window after the swap, and a record that
    /// waited in the predecessor's queue is lost with it.
    fn a_restarted_effects_subscription_survives_the_rebuild(backend: Backend, pending: bool) {
        let what = format!("{backend:?} pending={pending}");
        let (a, b) = restarted_pair(backend);
        let all = strips_of(&a);
        let restarted = vec!["eq1".to_owned()];
        let others: Vec<String> = all.iter().filter(|s| *s != "eq1").cloned().collect();
        // The reference: B's own queue holds the record for eq1 at its first block.
        let reference = swapped_with(
            backend,
            &a,
            &b,
            &Swap {
                early: &others,
                successor_records: &restarted,
                ..Swap::default()
            },
        );
        // The run: the host's table holds eq1's tap (and the carried ones).
        let bindings = bindings_of(&all);
        let run = if pending {
            swapped_with(
                backend,
                &a,
                &b,
                &Swap {
                    early: &others,
                    late: &restarted,
                    bindings: Some(&bindings),
                    ..Swap::default()
                },
            )
        } else {
            swapped_with(
                backend,
                &a,
                &b,
                &Swap {
                    early: &all,
                    bindings: Some(&bindings),
                    ..Swap::default()
                },
            )
        };
        assert_eq!(run.carry, CarryOutcome::Carried, "{what}");
        assert_eq!(run.restarted, vec![Box::<str>::from("eq1")], "{what}");
        assert_eq!(
            run.armed[0],
            all.len() as u64,
            "{what}: the restarted effect's tap is armed on the successor, with the carried ones"
        );
        let eq1 = run.streams["eq1"]
            .iter()
            .find(|window| window.first_sample >= SWAP_SAMPLE)
            .copied()
            .expect("eq1 publishes after the swap");
        assert_eq!(
            eq1.first_sample, SWAP_SAMPLE,
            "{what}: the first window opens at the adoption block"
        );
        assert_eq!(eq1.sequence, 1, "{what}: a fresh tap starts its sequence");
        assert!(
            eq1.left > 0.0 || eq1.right > 0.0,
            "{what}: the compressor reduces gain, or the reading is vacuous"
        );
        for strip in &all {
            let want = reference.streams[strip]
                .iter()
                .filter(|window| window.first_sample >= SWAP_SAMPLE || strip != "eq1");
            let got = run.streams[strip]
                .iter()
                .filter(|window| window.first_sample >= SWAP_SAMPLE || strip != "eq1");
            assert_eq!(
                got.collect::<Vec<_>>(),
                want.collect::<Vec<_>>(),
                "{what} {strip}: the windows equal a record applied at the adoption block"
            );
        }
        assert_eq!(
            run.allocator,
            ((0, 0, 0), (0, 0)),
            "{what}: allocator and render audit over the swap block"
        );
    }

    /// At the build's own width, `Backend::current()`: a console effect bank binds only at the
    /// width its build executes (#1017).
    #[test]
    fn a_restarted_effects_subscription_survives_the_rebuild_at_the_build_width() {
        let backend = Backend::current();
        for pending in [false, true] {
            a_restarted_effects_subscription_survives_the_rebuild(backend, pending);
        }
    }

    /// #1584 D6a gate 3: a disarm still waiting in the predecessor's queue at the swap leaves the
    /// restarted effect's successor tap unarmed: the host's table no longer holds it. And #1585: a
    /// disarm queued on the successor before the swap leaves a carried tap the predecessor had
    /// armed unarmed, in its slot and in the armed count: the carry's state comes first, the
    /// successor's own record after it.
    ///
    /// Red mutation: arm every tap the predecessor had armed -> eq1's successor tap is armed and
    /// publishes after the swap. Drain the successor's own queue in the carry, before the taps
    /// exchange -> the carried tap comes back armed over the successor's disarm.
    fn a_pending_disarm_leaves_the_restarted_tap_unarmed(backend: Backend) {
        let what = format!("{backend:?}");
        let (a, b) = restarted_pair(backend);
        let all = strips_of(&a);
        let restarted = vec!["eq1".to_owned()];
        let others: Vec<String> = all.iter().filter(|s| *s != "eq1").cloned().collect();
        let bindings = bindings_of(&others);
        // A carried tap the predecessor armed early, disarmed in the successor's own queue.
        let carried_disarm = vec![others[0].clone()];
        let run = swapped_with(
            backend,
            &a,
            &b,
            &Swap {
                early: &all,
                late_disarm: &restarted,
                successor_disarms: &carried_disarm,
                bindings: Some(&bindings),
                ..Swap::default()
            },
        );
        assert_eq!(run.restarted, vec![Box::<str>::from("eq1")], "{what}");
        assert_eq!(
            run.armed[0],
            others.len() as u64 - 1,
            "{what}: the disarmed taps start unarmed"
        );
        assert!(
            !run.states[&carried_disarm[0]].armed,
            "{what}: the successor's disarm of a carried tap is in its slot"
        );
        assert!(
            run.streams["eq1"]
                .iter()
                .all(|window| window.end_sample <= SWAP_SAMPLE),
            "{what}: eq1 publishes nothing after the swap"
        );
    }

    /// At the build's own width, `Backend::current()`: a console effect bank binds only at the
    /// width its build executes (#1017).
    #[test]
    fn a_pending_disarm_leaves_the_restarted_tap_unarmed_at_the_build_width() {
        a_pending_disarm_leaves_the_restarted_tap_unarmed(Backend::current());
    }

    /// #1584 D6a gate 6: the table arms every carried tap too, and the prepared arm never wins
    /// over the state a carried tap brings: the windows continue (sequences 1, 2, 3, the one open
    /// at the swap included) exactly as with an empty table.
    ///
    /// Red mutation: let `arm_prepared` survive `swap_state` (restart the window at the first
    /// block) -> the window spanning the swap restarts and the sequence breaks.
    fn a_carried_tap_is_not_restarted_by_the_prepared_arm(backend: Backend, form: Form) {
        let what = format!("{backend:?} {form:?}");
        let (a, b) = pair(backend, form);
        let subscribed = strips_of(&a);
        let kept: Vec<String> = subscribed
            .iter()
            .filter(|strip| strips_of(&b).contains(strip))
            .cloned()
            .collect();
        let bindings = bindings_of(&kept);
        let plain = swapped(backend, &a, &b, &subscribed);
        let armed = swapped_with(
            backend,
            &a,
            &b,
            &Swap {
                early: &subscribed,
                bindings: Some(&bindings),
                ..Swap::default()
            },
        );
        assert_eq!(armed.carry, CarryOutcome::Carried, "{what}");
        assert_eq!(armed.restarted, Vec::<Box<str>>::new(), "{what}");
        assert_eq!(armed.streams, plain.streams, "{what}: no tap restarted");
        // The retiring predecessor holds whatever the successor's lanes held before the exchange.
        assert_eq!(armed.armed[0], plain.armed[0], "{what}");
        for strip in &kept {
            assert_eq!(
                armed.streams[strip]
                    .iter()
                    .map(|window| window.sequence)
                    .collect::<Vec<_>>(),
                vec![1, 2, 3],
                "{what} {strip}: no gap and no restart"
            );
        }
        assert_eq!(armed.allocator, ((0, 0, 0), (0, 0)), "{what}");
    }

    /// At the build's own width, `Backend::current()`: a console effect bank binds only at the
    /// width its build executes (#1017).
    #[test]
    fn a_carried_tap_is_not_restarted_by_the_prepared_arm_at_the_build_width() {
        let backend = Backend::current();
        for form in [Form::NodeToNode, Form::BankToBank, Form::NodeToBank] {
            a_carried_tap_is_not_restarted_by_the_prepared_arm(backend, form);
        }
    }

    /// #1584 D6a gate 4, the plan half: a table that still names a removed insert, a removed
    /// strip or an ID reused for an effect without the tap arms nothing and fails nothing. (The
    /// control plane drops such a binding and ends it with an event; this is the plan's own
    /// guard.)
    #[test]
    fn a_stale_binding_arms_nothing() {
        let backend = Backend::current();
        let base = compressor_insert_session(bank_width(backend) - 2);
        let mut stripped = with_muted_track(base.clone());
        track_mut(&mut stripped, "eq1").inserts.effects.clear();
        let (a, b) = (Session::compile(base), Session::compile(stripped));
        let all = strips_of(&a);
        let mut bindings = bindings_of(&all);
        // A strip B does not have, and an insert B does not have on a strip it does.
        bindings.set(tap_key("gone"), WINDOW_BLOCKS);
        let run = swapped_with(
            backend,
            &a,
            &b,
            &Swap {
                early: &all,
                bindings: Some(&bindings),
                ..Swap::default()
            },
        );
        assert_eq!(run.carry, CarryOutcome::Carried);
        assert_eq!(
            run.armed[0],
            all.len() as u64 - 1,
            "every tap B still has is armed; eq1's insert is gone"
        );
    }

    /// Gate 5 for the taps: the swap block -- carry, the attachment hook's pairing and the first
    /// render -- allocates and frees nothing, counted rather than aborted.
    #[test]
    fn the_tap_swap_block_allocates_and_frees_nothing() {
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
        let backend = Backend::current();
        let (a, b) = pair(backend, Form::NodeToNode);
        let subscribed = strips_of(&a);
        let run = swapped(backend, &a, &b, &subscribed);
        assert_eq!(run.carry, CarryOutcome::Carried);
        assert_eq!(run.carries.len(), subscribed.len());
        assert_eq!(run.allocator, ((0, 0, 0), (0, 0)));
    }
}

// Issue #1283 (slice 13 of #1269): the plug-in delay compensation lines. PDC delays each edge into
// a node with several inputs by that node's latest arrival minus the edge's own, so one strip with
// a latency insert gives every other route into the same node a line. A line is keyed by its edge
// ID and the edge's endpoints: it carries when both survive, at the output and inside a submix
// alike, and a re-pointed route carries nothing.

/// The tracks `eq0` to `eq3` with `limited` strips behind a true-peak limiter insert. The
/// limiter reports `Fs / 100 + 6` samples of latency, so every route into a node the limited
/// strip also reaches has a compensation line.
fn pdc_session(rate: u32, limited: &[&str]) -> SessionModel {
    let mut model = filtered_session(false);
    model.sample_rate_hz = rate;
    let cut: Vec<String> = model
        .tracks
        .iter()
        .skip(4)
        .map(|track| track.id.as_str().to_owned())
        .collect();
    for track in cut {
        remove_track(&mut model, &track);
    }
    for (index, track) in model.tracks.iter_mut().enumerate() {
        if limited.contains(&track.id.as_str()) {
            track.inserts.effects.push(Effect {
                id: id("lim"),
                identity: EffectIdentity::Native {
                    effect_id: id("miso.true-peak-limiter"),
                },
                quality: EffectQuality::Normal,
                bypass: false,
                link_mode: LinkMode::Maximum,
                params: console_params("limiter", index, false),
                sidechain: SidechainDeclaration::None,
            });
        }
    }
    model
}

/// `model` with the route of `track` sent to submix `bus` instead of where it went.
fn send_route_to(model: &mut SessionModel, track: &str, bus: &str) {
    let main = format!("{track}-main");
    let route = model
        .routes
        .iter_mut()
        .find(|route| route.id.as_str() == main)
        .expect("the track's main route");
    route.destination = session::RouteDestination::SubmixInput { submix_id: id(bus) };
}

/// A unity submix `bus` that forwards to the output.
fn add_bus(model: &mut SessionModel, bus: &str) {
    let template = model
        .routes
        .iter()
        .find(|route| route.id.as_str() == "eq0-main")
        .expect("a track route")
        .clone();
    let mut onward = template.clone();
    let RouteSource::Track { tap, .. } = template.source else {
        panic!("a track route");
    };
    onward.id = id(&format!("{bus}-main"));
    onward.source = RouteSource::Submix {
        submix_id: id(bus),
        tap,
    };
    onward.follows_mute = false;
    model.routes.push(onward);
    model
        .submixes
        .push(session::Submix::unity(id(bus), &model.console));
}

/// The rates the launch supports.
const PDC_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];

/// A swapped run of `a` into `b` and the fresh reference of `b`, with live controls attached (the
/// form whose inserts carry), and the checks every case shares: the hand-over carried, the swap
/// block made no allocator call and no restore was refused.
fn pdc_runs(
    a: &Session,
    b: &Session,
    feed: &Feed,
    backend: Backend,
    what: &str,
) -> (EffectRun, EffectRun) {
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
    let reference = effect_run(b, None, feed, &[], &[], backend);
    let run = effect_run(a, Some((b, &a.model)), feed, &[], &[], backend);
    assert_clean_swap(&run, what);
    (run, reference)
}

/// Gate 1: three tracks, the first behind a limiter, so the other two routes into the output
/// carry lines. B adds a muted track with no latent effect, so no arrival moves. The swapped run
/// equals B rendered from frame 0 in every block, at every launch rate. Red if a line is keyed by
/// its index in the delay vector (the added track sorts first and shifts every index) or left at
/// rest: the two compensated tracks drop out for the length of their line.
fn compensated_output_lines_keep_their_audio(backend: Backend, rate: u32) {
    let what = format!("{backend:?} at {rate} Hz");
    let mut a_model = pdc_session(rate, &["eq0"]);
    remove_track(&mut a_model, "eq3");
    let mut b_model = a_model.clone();
    add_track(&mut b_model, "eq1", MUTED_TRACK, SOURCE, true);
    let (a, b) = (Session::compile(a_model), Session::compile(b_model));
    assert_eq!(
        a.prepare(backend).inventory.delay_line_count(),
        2,
        "{what}: the two tracks without the limiter are compensated"
    );
    let feed = Feed::new(SOURCE, 1);
    let (run, reference) = pdc_runs(&a, &b, &feed, backend, &what);
    assert_eq!(run.restarted, Vec::<Box<str>>::new(), "{what}");
    assert_eq!(
        first_effect_difference(&run, &reference),
        None,
        "{what}: every block equals the post-edit session rendered from frame 0"
    );
    // At 96 kHz the limiter's lookahead is longer than the blocks before the swap, so the first
    // audio comes out after it; the lines hold what was fed before.
    assert!(
        reference.blocks[SWAP_BLOCK + 2..]
            .iter()
            .flatten()
            .any(|bits| *bits != 0),
        "{what}: the reference plays after the swap block"
    );
}

#[test]
fn compensated_output_lines_keep_their_audio_at_every_rate_and_width() {
    for backend in backends() {
        for rate in PDC_RATES {
            compensated_output_lines_keep_their_audio(backend, rate);
        }
    }
}

/// The submix `bus1` takes `eq0` (behind a limiter) and `eq1`; the output takes `bus1` and `eq2`.
/// The edge of `eq1` into the bus and the edge of `eq2` into the output both carry a line.
fn bus_session(rate: u32) -> SessionModel {
    let mut model = pdc_session(rate, &["eq0"]);
    remove_track(&mut model, "eq3");
    add_bus(&mut model, "bus1");
    send_route_to(&mut model, "eq0", "bus1");
    send_route_to(&mut model, "eq1", "bus1");
    model
}

/// Gate 2: B adds a muted zero-latency track to the submix. The submix's own line and the output's
/// line both carry. Red if the carry handles only the edges into the output: the submix input
/// line starts at rest and `eq1` drops out of the bus for the length of its line.
fn compensated_submix_lines_keep_their_audio(backend: Backend, rate: u32) {
    let what = format!("{backend:?} at {rate} Hz");
    let a_model = bus_session(rate);
    let mut b_model = a_model.clone();
    add_track(&mut b_model, "eq1", MUTED_TRACK, SOURCE, true);
    let (a, b) = (Session::compile(a_model), Session::compile(b_model));
    assert_eq!(
        a.prepare(backend).inventory.delay_line_count(),
        2,
        "{what}: eq1 into the bus and eq2 into the output are compensated"
    );
    let feed = Feed::new(SOURCE, 1);
    let (run, reference) = pdc_runs(&a, &b, &feed, backend, &what);
    assert_eq!(run.restarted, Vec::<Box<str>>::new(), "{what}");
    assert_eq!(
        first_effect_difference(&run, &reference),
        None,
        "{what}: every block equals the post-edit session rendered from frame 0"
    );
}

#[test]
fn compensated_submix_lines_keep_their_audio_at_every_rate_and_width() {
    for backend in backends() {
        for rate in PDC_RATES {
            compensated_submix_lines_keep_their_audio(backend, rate);
        }
    }
}

/// Two submixes, each with a limited and an unlimited track: `eq0` (limited) and `eq1` into
/// `bus1`, `eq3` (limited) and `eq2` into `bus2`. Both unlimited routes carry a line.
fn two_bus_session(rate: u32, silent: Option<&str>) -> SessionModel {
    let mut model = pdc_session(rate, &["eq0", "eq3"]);
    add_bus(&mut model, "bus1");
    add_bus(&mut model, "bus2");
    send_route_to(&mut model, "eq0", "bus1");
    send_route_to(&mut model, "eq1", "bus1");
    send_route_to(&mut model, "eq2", "bus2");
    send_route_to(&mut model, "eq3", "bus2");
    if let Some(track) = silent {
        model = with_mute(model, track, true);
    }
    model
}

/// Gate 4: B re-points `eq1`'s route from `bus1` to `bus2`. The route keeps its ID and gets
/// another destination, so its line does not carry and its strip is not restarted.
///
/// With `eq1` muted in both sessions the re-pointed line holds only zeros, so the run equals B
/// from frame 0 exactly when every other line carried: `eq2`'s line into `bus2` is the one that
/// does. With `eq1` playing, the new route's line starts at rest and the swap block misses the
/// audio B's line holds: red if the line followed the route ID into the other bus, where it
/// would play the old pending audio and match B.
fn a_repointed_route_does_not_carry_its_line(backend: Backend, rate: u32) {
    let what = format!("{backend:?} at {rate} Hz");
    let feed = Feed::new(SOURCE, 1);
    for silent in [Some("eq1"), None] {
        let a_model = two_bus_session(rate, silent);
        let mut b_model = a_model.clone();
        send_route_to(&mut b_model, "eq1", "bus2");
        let (a, b) = (Session::compile(a_model), Session::compile(b_model));
        let (run, reference) = pdc_runs(&a, &b, &feed, backend, &what);
        assert_eq!(
            run.restarted,
            Vec::<Box<str>>::new(),
            "{what}: the re-pointed route's strip keeps its state"
        );
        // Before the swap A renders, and A routes `eq1` through the other bus.
        let difference = run.blocks[SWAP_BLOCK..]
            .iter()
            .zip(&reference.blocks[SWAP_BLOCK..])
            .position(|(swapped, fresh)| swapped != fresh);
        if silent.is_some() {
            assert_eq!(
                difference, None,
                "{what}: every other line carried, and the re-pointed one held silence"
            );
        } else {
            assert!(
                difference.is_some(),
                "{what}: the new route's line starts at rest, so B's audio is missing after the swap"
            );
        }
    }
}

#[test]
fn a_repointed_route_does_not_carry_its_line_at_every_rate_and_width() {
    for backend in backends() {
        for rate in PDC_RATES {
            a_repointed_route_does_not_carry_its_line(backend, rate);
        }
    }
}

/// #1891: B removes track `eq1` and adds an unfed submix named `eq1`, keeping route `eq1-main`
/// into `bus1` with its ID and tap. The graph names a submix's stage as it names a track's, so the
/// route's edge and endpoints are the same on both sides; but its source is another strip, so its
/// line starts at rest, holds the silent submix's zeros, and the run equals B from frame 0 from
/// the swap block on, exactly when every other line carried. Red if a line joins by graph node
/// alone: `bus1` plays the removed track's pending audio after the swap.
#[test]
fn a_route_whose_source_changed_kind_does_not_carry_its_line() {
    let rate = 48_000;
    let feed = Feed::new(SOURCE, 1);
    let a_model = two_bus_session(rate, None);
    let mut b_model = a_model.clone();
    b_model.tracks.retain(|track| track.id.as_str() != "eq1");
    let route = b_model
        .routes
        .iter_mut()
        .find(|route| route.id.as_str() == "eq1-main")
        .expect("eq1-main");
    let RouteSource::Track { tap, .. } = route.source else {
        panic!("a track route");
    };
    route.source = RouteSource::Submix {
        submix_id: id("eq1"),
        tap,
    };
    b_model
        .submixes
        .push(session::Submix::unity(id("eq1"), &b_model.console));
    let (a, b) = (Session::compile(a_model), Session::compile(b_model));
    for backend in backends() {
        let what = format!("{backend:?}");
        // The control: both sides compensate `eq1-main` and `eq2-main`, so the route's line has
        // a predecessor of its edge ID to carry from.
        for session in [&a, &b] {
            assert_eq!(
                session.prepare(backend).inventory.delay_line_count(),
                2,
                "{what}"
            );
        }
        let (run, reference) = pdc_runs(&a, &b, &feed, backend, &what);
        let difference = run.blocks[SWAP_BLOCK..]
            .iter()
            .zip(&reference.blocks[SWAP_BLOCK..])
            .position(|(swapped, fresh)| swapped != fresh);
        assert_eq!(
            difference, None,
            "{what}: the line of the route from the submix starts at rest; the others carried"
        );
    }
}

/// #1891, the effect-source case: a strip's main edge into an effect whose key arrives late
/// starts at the strip's own effect node and holds the strip's own audio. A has track `eq1` with
/// a transparent compressor and then a compressor keyed from `eq0`'s insert return, behind
/// `eq0`'s limiter, so the main edge into the keyed compressor has a line; B replaces the track
/// with an unfed submix `eq1` with the same inserts. The line's source is another strip, so it
/// starts at rest and the swapped run equals fresh B from the swap block on. Red if a line whose
/// source is an `Effect` node joins by graph node alone: the submix plays the track's audio.
#[test]
fn a_line_from_an_effect_of_a_strip_of_another_kind_does_not_carry() {
    let rate = 48_000;
    let feed = Feed::new(SOURCE, 1);
    let mut a_model = pdc_session(rate, &["eq0"]);
    remove_track(&mut a_model, "eq2");
    remove_track(&mut a_model, "eq3");
    let inserts = &mut track_mut(&mut a_model, "eq1").inserts.effects;
    inserts.push(transparent_insert());
    inserts.push(Effect {
        id: id("duck"),
        identity: EffectIdentity::Native {
            effect_id: id("miso.compressor"),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::Maximum,
        params: console_params("comp", 1, false),
        sidechain: SidechainDeclaration::Routed(session::Sidechain {
            source: RouteSource::Track {
                track_id: id("eq0"),
                tap: session::SendTap::InsertReturn,
            },
            port_id: id("sidechain-in"),
        }),
    });
    let mut b_model = a_model.clone();
    let track = b_model.tracks.remove(
        b_model
            .tracks
            .iter()
            .position(|track| track.id.as_str() == "eq1")
            .expect("eq1"),
    );
    let mut submix = session::Submix::unity(id("eq1"), &b_model.console);
    submix.inserts = track.inserts;
    b_model.submixes.push(submix);
    let route = b_model
        .routes
        .iter_mut()
        .find(|route| route.id.as_str() == "eq1-main")
        .expect("eq1-main");
    let RouteSource::Track { tap, .. } = route.source else {
        panic!("a track route");
    };
    route.source = RouteSource::Submix {
        submix_id: id("eq1"),
        tap,
    };
    let (a, b) = (Session::compile(a_model), Session::compile(b_model));
    for backend in backends() {
        let what = format!("{backend:?}");
        // The control: both sides delay the main edge into the keyed compressor.
        for session in [&a, &b] {
            assert_eq!(
                session.prepare(backend).inventory.delay_line_count(),
                1,
                "{what}"
            );
        }
        let (run, reference) = pdc_runs(&a, &b, &feed, backend, &what);
        let difference = run.blocks[SWAP_BLOCK..]
            .iter()
            .zip(&reference.blocks[SWAP_BLOCK..])
            .position(|(swapped, fresh)| swapped != fresh);
        assert_eq!(
            difference, None,
            "{what}: the submix's line from its own effect starts at rest"
        );
    }
}

// Issue #1285 (slice 15 of #1269): latency floors. A successor is compiled with every node's
// input arrival in its predecessor as a floor, so removing a latent effect, or the track that
// holds it, moves no surviving path in time: every compensation line keeps its length and carries.

/// B prepared against an inventory that holds only `floors`' input floors, rendered from frame 0
/// and fed `feed`: the fresh reference of B compiled with those floors (#1285 gate 1).
fn floored_reference(
    b: &Session,
    floors: &host_core::PlanStateInventory,
    feed: &Feed,
    backend: Backend,
) -> Vec<Vec<u32>> {
    let quantum = b.model.quantum_frames as usize;
    let floors = floors.test_only_floors_only();
    let mut plan = LivePlan::successor(b, &floors, &b.model, backend);
    plan.submit(feed, quantum, 0);
    (0..BLOCKS)
        .map(|block| {
            if block + 1 < BLOCKS {
                plan.submit(feed, quantum, block + 1);
            }
            plan.render(quantum)
        })
        .collect()
}

/// `model` with one more true-peak limiter insert, `insert`, on `track`, after its others.
fn add_limiter(model: &mut SessionModel, track: &str, insert: &str) {
    let (index, strip) = model
        .tracks
        .iter_mut()
        .enumerate()
        .find(|(_, strip)| strip.id.as_str() == track)
        .expect("the track");
    strip.inserts.effects.push(Effect {
        id: id(insert),
        identity: EffectIdentity::Native {
            effect_id: id("miso.true-peak-limiter"),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::Maximum,
        params: console_params("limiter", index, false),
        sidechain: SidechainDeclaration::None,
    });
}

/// The checks gates 1 and 2 share: B removes the limited track `eq0` from `a_model`. The swapped
/// run equals B compiled with A's floors from frame 0 in every block, its latency is A's, nothing
/// restarts, and B without floors renders the remaining tracks earlier. `eq0` is muted in A, so it
/// adds exact `+0.0` and A's blocks before the swap are B's too.
fn a_removed_latency_moves_no_surviving_path(a_model: SessionModel, backend: Backend, what: &str) {
    let a_model = with_mute(a_model, "eq0", true);
    let mut b_model = a_model.clone();
    remove_track(&mut b_model, "eq0");
    let (a, b) = (Session::compile(a_model), Session::compile(b_model));
    let predecessor = LivePlan::fresh(&a, backend).prepared;
    let latency = predecessor.report.latency_samples;
    assert!(latency > 0, "{what}: the limiter delays A");
    let successor = LivePlan::successor(&b, &predecessor.inventory, &a.model, backend).prepared;
    assert_eq!(
        successor.report.latency_samples, latency,
        "{what}: the successor reports A's latency (D4)"
    );
    assert!(
        b.prepare(backend).report.latency_samples < latency,
        "{what}: B's natural latency is lower"
    );
    let feed = Feed::new(SOURCE, 1);
    let (run, unfloored) = pdc_runs(&a, &b, &feed, backend, what);
    assert_eq!(run.restarted, Vec::<Box<str>>::new(), "{what}");
    let reference = floored_reference(&b, &predecessor.inventory, &feed, backend);
    assert_eq!(
        run.blocks
            .iter()
            .zip(&reference)
            .position(|(swapped, fresh)| swapped != fresh),
        None,
        "{what}: every block equals B compiled with A's floors from frame 0"
    );
    assert!(
        reference[SWAP_BLOCK + 2..]
            .iter()
            .flatten()
            .any(|bits| *bits != 0),
        "{what}: the reference plays after the swap block"
    );
    assert_ne!(
        unfloored.blocks[SWAP_BLOCK + 1..],
        reference[SWAP_BLOCK + 1..],
        "{what}: without floors the remaining tracks move by the limiter's latency"
    );
}

/// Gate 1: three tracks, the first behind a limiter; B removes the first track. Red if a
/// successor is compiled without floors: the output's two compensation lines vanish and the
/// remaining tracks jump forward by the limiter's latency at the swap.
#[test]
fn removing_a_limited_track_keeps_the_output_timing_at_every_rate_and_width() {
    for backend in backends() {
        for rate in PDC_RATES {
            let mut a_model = pdc_session(rate, &["eq0"]);
            remove_track(&mut a_model, "eq3");
            a_removed_latency_moves_no_surviving_path(
                a_model,
                backend,
                &format!("{backend:?} at {rate} Hz"),
            );
        }
    }
}

/// Gate 2: `eq0` (behind a limiter) and `eq1` feed the submix `bus1`; the output takes `bus1` and
/// `eq2`. B removes `eq0`. Red if floors are applied only at the output: the bus input's natural
/// arrival drops to 0, `eq1`'s line into the bus vanishes and the bus's own route into the output
/// gets a new line that starts at rest.
#[test]
fn removing_a_limited_track_keeps_the_submix_timing_at_every_rate_and_width() {
    for backend in backends() {
        for rate in PDC_RATES {
            a_removed_latency_moves_no_surviving_path(
                bus_session(rate),
                backend,
                &format!("{backend:?} at {rate} Hz, submix"),
            );
        }
    }
}

/// Gate 5: after gate 1's swap, C removes `eq1` from B. C keeps A's output latency, because B's
/// inventory records the floored arrivals. Red if the inventory records the natural `max`: B's
/// output arrival is naturally 0, so C's latency drops to it.
#[test]
fn a_second_removal_keeps_the_first_floors() {
    for backend in backends() {
        let mut a_model = pdc_session(48_000, &["eq0"]);
        remove_track(&mut a_model, "eq3");
        let mut b_model = a_model.clone();
        remove_track(&mut b_model, "eq0");
        let mut c_model = b_model.clone();
        remove_track(&mut c_model, "eq1");
        let (a, b, c) = (
            Session::compile(a_model),
            Session::compile(b_model),
            Session::compile(c_model),
        );
        let first = a.prepare(backend);
        let latency = first.report.latency_samples;
        assert!(latency > 0, "{backend:?}");
        let second = b
            .prepare_successor(&first.inventory, &a.model, &caps(), backend)
            .expect("B");
        let third = c
            .prepare_successor(&second.inventory, &b.model, &caps(), backend)
            .expect("C");
        assert_eq!(second.report.latency_samples, latency, "{backend:?}: B");
        assert_eq!(third.report.latency_samples, latency, "{backend:?}: C");
        assert_eq!(
            third.inventory.input_floors(),
            second.inventory.input_floors(),
            "{backend:?}: C renders at B's arrivals"
        );
    }
}

/// #1344 with floors: P0 has `eq0` behind a limiter. The candidate A adds a second limiter to
/// `eq1`, so its own floors are higher than P0's, and is withdrawn. B removes `eq0` from P0 and is
/// prepared against P0 across A. Render runs P0, so B keeps P0's timing: its latency is P0's and
/// its floors are P0's arrivals. Red if the base across a withdrawn plan drops the floors (B's
/// latency falls to its natural value) or takes A's (B's latency is A's).
#[test]
fn a_replacement_across_a_withdrawn_candidate_keeps_the_running_floors() {
    for backend in backends() {
        let mut p0_model = pdc_session(48_000, &["eq0"]);
        remove_track(&mut p0_model, "eq3");
        let mut a_model = p0_model.clone();
        add_limiter(&mut a_model, "eq1", "lim");
        add_limiter(&mut a_model, "eq1", "lim2");
        let mut b_model = p0_model.clone();
        remove_track(&mut b_model, "eq0");
        let (p0, a, b) = (
            Session::compile(p0_model),
            Session::compile(a_model),
            Session::compile(b_model),
        );
        let running = p0.prepare(backend);
        let latency = running.report.latency_samples;
        let withdrawn = a
            .prepare_successor(&running.inventory, &p0.model, &caps(), backend)
            .expect("A");
        assert!(
            withdrawn.report.latency_samples > latency,
            "{backend:?}: A's two limiters delay it past P0"
        );
        let base = running
            .inventory
            .across_withdrawn(&withdrawn.inventory, &withdrawn.plan);
        let replacement = b
            .prepare_successor(&base, &p0.model, &caps(), backend)
            .expect("B");
        assert_eq!(
            replacement.report.latency_samples, latency,
            "{backend:?}: B keeps the running plan's latency"
        );
        let direct = b
            .prepare_successor(&running.inventory, &p0.model, &caps(), backend)
            .expect("P0 -> B");
        assert_eq!(
            replacement.inventory.input_floors(),
            direct.inventory.input_floors(),
            "{backend:?}: B renders at the arrivals of a direct P0 -> B swap"
        );
    }
}

// Issue #1323: a host-declared discontinuity. Its successor is compiled with no floors and
// carries only its source consumers, so it renders as the session compiled fresh, fed from the
// frames the carried consumers stood at.

/// The block the discontinuity successor renders first.
const DISCONTINUITY_BLOCK: usize = 10;
/// Blocks in a discontinuity run.
const DISCONTINUITY_BLOCKS: usize = 16;

/// `session` prepared with live controls as the successor of `predecessor` at a declared
/// discontinuity, its gate stamped and published with the committed revision, as the control
/// plane's publication does.
fn discontinuity_successor(
    session: &Session,
    predecessor: &host_core::PlanStateInventory,
    committed: &SessionModel,
    backend: Backend,
) -> LivePlan {
    let (prepared, handles) =
        host_core::test_only_prepare_host_runtime_with_live_controls_successor_on(
            &session.compiled,
            &caps(),
            &LivePlan::request(),
            host_core::SuccessorBase {
                discontinuity: true,
                ..host_core::SuccessorBase::new(
                    predecessor,
                    committed,
                    successor::SUCCESSOR_REVISION,
                )
            },
            backend,
        )
        .unwrap_or_else(|failure| panic!("discontinuity successor: {failure:?}"));
    let mut successor = LivePlan::new(prepared, handles);
    let _ = successor
        .gate
        .stamp(successor::SUCCESSOR_REVISION)
        .expect("a fresh gate stamps the committed revision");
    successor.gate.publish(successor::SUCCESSOR_REVISION);
    successor
}

/// `successor` adopts `plan` and renders one block; the run goes on with it.
fn adopt_and_render(
    plan: &mut LivePlan,
    mut successor: LivePlan,
    feed: &Feed,
    quantum: usize,
    block: usize,
) -> (Vec<u32>, CarryOutcome) {
    assert_eq!(
        successor
            .prepared
            .sources
            .adopt_persisting(&mut plan.prepared.sources),
        1,
        "the source's producer moves"
    );
    successor.submit(feed, quantum, block + 1);
    let carry = successor
        .prepared
        .plan
        .adopt_predecessor_plan(&mut plan.prepared.plan);
    let bits = successor.render(quantum);
    *plan = successor;
    (bits, carry)
}

/// Gate 1: A has three tracks, the first behind a limiter; B removes it and is swapped in at
/// [`SWAP_BLOCK`] with A's floors (#1285). B is then prepared again against itself with
/// `discontinuity: true` and swapped in at [`DISCONTINUITY_BLOCK`]. From that block on, every
/// block equals B compiled fresh, with no floors, fed the PCM from the frame its carried source
/// consumer stood at; its latency is B's natural latency, and its inventory has no raised floor.
///
/// Test value: red if the discontinuity successor keeps the floors (its latency stays A's and the
/// remaining tracks stay late), or if it carries any DSP state but the source consumer (the EQs'
/// filter state and the compensation lines differ from a fresh plan's at rest), or if it carries
/// no source (it reads its ring from frame 0, a repeat).
fn a_declared_discontinuity_resets_the_floors(backend: Backend, rate: u32) {
    let what = format!("{backend:?} at {rate} Hz");
    let mut a_model = pdc_session(rate, &["eq0"]);
    remove_track(&mut a_model, "eq3");
    let a_model = with_mute(a_model, "eq0", true);
    let mut b_model = a_model.clone();
    remove_track(&mut b_model, "eq0");
    let (a, b) = (Session::compile(a_model), Session::compile(b_model));
    let quantum = a.model.quantum_frames as usize;
    let feed = Feed::with_blocks(SOURCE, 1, DISCONTINUITY_BLOCKS + DISCONTINUITY_BLOCK + 1);

    let mut plan = LivePlan::fresh(&a, backend);
    let latency = plan.prepared.report.latency_samples;
    let natural = LivePlan::fresh(&b, backend);
    let natural_latency = natural.prepared.report.latency_samples;
    assert!(
        natural_latency < latency,
        "{what}: B's natural latency is lower"
    );
    assert!(
        !plan.prepared.inventory.runs_behind_natural_timing(),
        "{what}: a first plan has no floor"
    );
    let mut blocks = Vec::with_capacity(DISCONTINUITY_BLOCKS);
    plan.submit(&feed, quantum, 0);
    for block in 0..DISCONTINUITY_BLOCKS {
        let bits = match block {
            SWAP_BLOCK => {
                let floored = LivePlan::successor(&b, &plan.prepared.inventory, &a.model, backend);
                assert_eq!(floored.prepared.report.latency_samples, latency, "{what}");
                assert!(
                    floored.prepared.inventory.runs_behind_natural_timing(),
                    "{what}: B with A's floors runs behind its natural timing"
                );
                adopt_and_render(&mut plan, floored, &feed, quantum, block).0
            }
            DISCONTINUITY_BLOCK => {
                let reset =
                    discontinuity_successor(&b, &plan.prepared.inventory, &b.model, backend);
                assert_eq!(
                    reset.prepared.report.latency_samples, natural_latency,
                    "{what}: the discontinuity successor runs at B's natural latency"
                );
                assert!(
                    !reset.prepared.inventory.runs_behind_natural_timing(),
                    "{what}: and has no raised floor"
                );
                assert_eq!(
                    reset.prepared.inventory.input_floors(),
                    natural.prepared.inventory.input_floors(),
                    "{what}: its arrivals are a fresh B's"
                );
                let (bits, carry) = adopt_and_render(&mut plan, reset, &feed, quantum, block);
                assert!(
                    carry == CarryOutcome::Carried,
                    "{what}: the source consumer carries: {carry:?}"
                );
                bits
            }
            _ => {
                if block + 1 < DISCONTINUITY_BLOCKS {
                    plan.submit(&feed, quantum, block + 1);
                }
                plan.render(quantum)
            }
        };
        blocks.push(bits);
    }

    // The fresh reference: B with no floors, rendered from frame 0, fed the PCM from the frame the
    // carried consumer stood at when the discontinuity successor adopted.
    let offset = DISCONTINUITY_BLOCK * quantum;
    let shifted = Feed {
        id: SOURCE,
        planes: [
            feed.planes[0][offset..].to_vec(),
            feed.planes[1][offset..].to_vec(),
        ],
    };
    let mut reference = natural;
    let tail = DISCONTINUITY_BLOCKS - DISCONTINUITY_BLOCK;
    reference.submit(&shifted, quantum, 0);
    let expected: Vec<Vec<u32>> = (0..tail)
        .map(|block| {
            reference.submit(&shifted, quantum, block + 1);
            reference.render(quantum)
        })
        .collect();
    assert!(
        expected.iter().flatten().any(|bits| *bits != 0),
        "{what}: the reference plays"
    );
    assert_eq!(
        blocks[DISCONTINUITY_BLOCK..]
            .iter()
            .zip(&expected)
            .position(|(reset, fresh)| reset != fresh),
        None,
        "{what}: every block from the discontinuity equals a fresh B"
    );
}

#[test]
fn a_declared_discontinuity_resets_the_floors_at_every_rate_and_width() {
    for backend in backends() {
        for rate in PDC_RATES {
            a_declared_discontinuity_resets_the_floors(backend, rate);
        }
    }
}

// Issue #1284 (slice 14 of #1269): the strip input alignment lines (`delay_samples` of a track and
// of a submix) and the live sends. A line is keyed by its strip and lane, a send by its route ID;
// a lane whose delay changed starts at rest and its strip goes into the restart set, and a
// re-pointed send is a route removed and a route added (#1363), which restarts nothing.

/// The send whose level the cases move.
const SEND: &str = "eq3-main";
/// A send's ramp, in samples: it spans six blocks, so it is in flight at the swap block.
const LONG_RAMP: u32 = 6 * QUANTUM as u32;

/// Set the declared input delay of strip `strip` (a track or a submix).
fn with_strip_delay(mut model: SessionModel, strip: &str, delays: [u32; 2]) -> SessionModel {
    let builtins = if let Some(track) = model.tracks.iter_mut().find(|t| t.id.as_str() == strip) {
        &mut track.builtins
    } else {
        &mut model
            .submixes
            .iter_mut()
            .find(|s| s.id.as_str() == strip)
            .expect("a strip")
            .builtins
    };
    builtins.left.delay_samples = delays[0];
    builtins.right.delay_samples = delays[1];
    model
}

/// Gate 1's session: the track `eq0` with two different lane delays straight to the output; `eq1`
/// and `eq2` (a delay on its right lane only) and `eq3` into the submix `bus1`, which has its own
/// input delay; `eq3`'s route [`SEND`] is the live send the cases move. `bus2` is a second submix
/// that nothing feeds. `bus_delays` gives the submixes their input delays (zero for a case that
/// needs a memoryless bus).
fn send_session(bus_delays: bool) -> SessionModel {
    let mut model = pdc_session(48_000, &[]);
    add_bus(&mut model, "bus1");
    add_bus(&mut model, "bus2");
    for track in ["eq1", "eq2", "eq3"] {
        send_route_to(&mut model, track, "bus1");
    }
    let model = with_strip_delay(model, "eq0", [37, 53]);
    let model = with_strip_delay(model, "eq2", [0, 19]);
    if bus_delays {
        with_strip_delay(model, "bus1", [41, 29])
    } else {
        model
    }
}

/// `model` plus [`MUTED_TRACK`], a muted copy of `eq0` that sorts first and goes to the output.
fn with_muted_output_track(mut model: SessionModel) -> SessionModel {
    add_track(&mut model, "eq0", MUTED_TRACK, SOURCE, true);
    model
}

fn route_mut<'a>(model: &'a mut SessionModel, route: &str) -> &'a mut session::Route {
    model
        .routes
        .iter_mut()
        .find(|candidate| candidate.id.as_str() == route)
        .expect("a route")
}

fn send_writes() -> Vec<EffectWrite> {
    // One ramp in flight at the swap block, and a second record admitted after the last block A
    // renders, which is pending at the swap.
    let write = |block, gain_db, length| EffectWrite {
        block,
        strip: "",
        address: EQ_ADDRESS,
        value: EffectValue::SendGain {
            route: SEND,
            gain_db,
            length,
        },
    };
    vec![
        write(SWAP_BLOCK - 2, -9.0, LONG_RAMP),
        write(SWAP_BLOCK, -2.5, 3 * QUANTUM as u32 + 5),
    ]
}

/// Gate 1: every block of the swapped run equals the post-edit session rendered from frame 0 and
/// fed the same records at the same blocks. Red if any strip delay line (a track's two unequal
/// lanes, a lane with no delay, the submix's) is left at rest (the delayed strip drops out for the
/// length of its line), if the send's ramp restarts from its target (the level steps), or if the
/// record pending at the swap is lost (the copy ran before the drain).
fn strip_delays_and_a_send_ramp_cross_the_swap(backend: Backend) {
    let a = Session::compile(send_session(true));
    let b = Session::compile(with_muted_output_track(send_session(true)));
    let feed = Feed::new(SOURCE, 1);
    let writes = send_writes();
    let probe = LivePlan::fresh(&a, backend).prepared;
    assert_eq!(
        probe.inventory.strip_delay_count(),
        3,
        "eq0, eq2 and bus1 declare delays"
    );
    assert_eq!(
        probe.inventory.live_route_count(),
        3,
        "the three sends into bus1; the routes into the output are not live"
    );
    drop(probe);
    let reference = effect_run(&b, None, &feed, &writes, &[], backend);
    let run = effect_run(&a, Some((&b, &a.model)), &feed, &writes, &[], backend);
    assert_clean_swap(&run, &format!("{backend:?}"));
    assert_eq!(run.restarted, Vec::<Box<str>>::new(), "{backend:?}");
    assert_eq!(
        first_effect_difference(&run, &reference),
        None,
        "{backend:?}: every block equals the post-edit session fed the same records"
    );
    // Not vacuous: the records are audible in the blocks after the swap, and so are the lines.
    let unmoved = effect_run(&b, None, &feed, &[], &[], backend);
    assert_ne!(
        unmoved.blocks[SWAP_BLOCK + 1],
        reference.blocks[SWAP_BLOCK + 1],
        "{backend:?}: the ramp is in flight after the swap block"
    );
    let undelayed = Session::compile(with_muted_output_track(undelayed_send_session()));
    let undelayed = effect_run(&undelayed, None, &feed, &writes, &[], backend);
    assert_ne!(
        undelayed.blocks[SWAP_BLOCK], reference.blocks[SWAP_BLOCK],
        "{backend:?}: the delays are audible in the swap block"
    );
}

/// [`send_session`] with no delay at all, for the case that shows the delays matter.
fn undelayed_send_session() -> SessionModel {
    let model = with_strip_delay(send_session(false), "eq0", [0, 0]);
    with_strip_delay(model, "eq2", [0, 0])
}

#[test]
fn strip_delays_and_a_send_ramp_cross_the_swap_on_every_width() {
    for backend in backends() {
        strip_delays_and_a_send_ramp_cross_the_swap(backend);
    }
}

/// #1391: [`send_session`] with `eq0`'s route into the output taken from its `pre_fader` tap, so it
/// has a live lane, and the cases' send records written to it. Red if the swap's route join keeps
/// only routes into a submix: B's lane then starts at its prepared value, and the level steps at
/// the swap block instead of finishing the ramp.
fn a_pre_fader_output_ramp_crosses_the_swap(backend: Backend) {
    const DIRECT: &str = "eq0-main";
    // The tap moves after B's muted track copies `eq0`'s route, so that copy stays `post_pan`
    // and silent.
    let pre_fader = |mut model: SessionModel| {
        let route = route_mut(&mut model, DIRECT);
        let RouteSource::Track { track_id, .. } = route.source.clone() else {
            panic!("a track route");
        };
        route.source = RouteSource::Track {
            track_id,
            tap: session::SendTap::PreFader,
        };
        model
    };
    let a = Session::compile(pre_fader(send_session(false)));
    let b = Session::compile(pre_fader(with_muted_output_track(send_session(false))));
    let feed = Feed::new(SOURCE, 1);
    let writes: Vec<EffectWrite> = send_writes()
        .into_iter()
        .map(|mut write| {
            if let EffectValue::SendGain { route, .. } = &mut write.value {
                *route = DIRECT;
            }
            write
        })
        .collect();
    let probe = LivePlan::fresh(&a, backend).prepared;
    assert_eq!(
        probe.inventory.live_route_count(),
        4,
        "the three sends into bus1 and the pre-fader route into the output"
    );
    drop(probe);
    let reference = effect_run(&b, None, &feed, &writes, &[], backend);
    let run = effect_run(&a, Some((&b, &a.model)), &feed, &writes, &[], backend);
    assert_clean_swap(&run, &format!("{backend:?}"));
    assert_eq!(
        first_effect_difference(&run, &reference),
        None,
        "{backend:?}: the pre-fader output route's ramp carries across the swap"
    );
    let unmoved = effect_run(&b, None, &feed, &[], &[], backend);
    assert_ne!(
        unmoved.blocks[SWAP_BLOCK + 1],
        reference.blocks[SWAP_BLOCK + 1],
        "{backend:?}: the ramp is in flight after the swap block"
    );
}

#[test]
fn a_pre_fader_output_ramp_crosses_the_swap_on_every_width() {
    for backend in backends() {
        a_pre_fader_output_ramp_crosses_the_swap(backend);
    }
}

/// The left and the right plane of one rendered block.
fn lane_planes(bits: &[u32]) -> (&[u32], &[u32]) {
    bits.split_at(QUANTUM)
}

/// Gate 2a: B moves `eq0`'s right lane from 53 to 60 samples of delay and leaves its left lane at
/// 37. The strip is in the restart set and nothing else is. The left lane's ring carries, so the
/// output's left plane equals the post-edit session rendered from frame 0 in every block; the right
/// lane starts at rest, so the swap block plays silence for the length of the new delay. Red if the
/// right ring is carried across a length change (the old alignment plays), if the lanes are keyed
/// together (the left lane restarts as well), or if the strip is not listed.
fn a_changed_lane_delay_restarts_only_its_lane(backend: Backend) {
    let mut one = pdc_session(48_000, &[]);
    for track in ["eq1", "eq2", "eq3"] {
        remove_track(&mut one, track);
    }
    let builtins = &mut one.tracks[0].builtins;
    for channel in [&mut builtins.left, &mut builtins.right] {
        channel.hpf_hz = 0.0;
        channel.lpf_hz = 0.0;
    }
    // The lanes reach the output independently, so a plane shows one lane's line.
    one.tracks[0].matrix_or_pan = session::MatrixOrPan::Matrix {
        ll: 1.0,
        lr: 0.0,
        rl: 0.0,
        rr: 1.0,
        smoothing_samples: 0,
    };
    let a_model = with_strip_delay(one, "eq0", [37, 53]);
    let b_model = with_strip_delay(a_model.clone(), "eq0", [37, 60]);
    let (a, b) = (Session::compile(a_model), Session::compile(b_model));
    let feed = Feed::new(SOURCE, 1);
    // Plain plans: a live plan arms the restarted strip's fader (#1324 D3), which would hide the
    // lanes' lines behind its fade.
    let reference = plain_run(&b, None, &feed, backend, false);
    let run = plain_run(&a, Some((&b, &a.model)), &feed, backend, false);
    assert_eq!(run.carry, CarryOutcome::Carried, "{backend:?}");
    assert_eq!(run.restarted, vec![Box::<str>::from("eq0")], "{backend:?}");
    for block in SWAP_BLOCK..BLOCKS {
        let (swapped_left, swapped_right) = lane_planes(&run.blocks[block]);
        let (fresh_left, fresh_right) = lane_planes(&reference.blocks[block]);
        assert_eq!(
            swapped_left
                .iter()
                .zip(fresh_left)
                .position(|(a, b)| a != b),
            None,
            "{backend:?}: block {block}: the left lane's ring carried ({} samples)",
            swapped_left.len()
        );
        if block == SWAP_BLOCK {
            assert!(
                swapped_right[..60]
                    .iter()
                    .all(|bits| f32::from_bits(*bits) == 0.0),
                "{backend:?}: the right lane's new line starts at rest"
            );
            assert_ne!(
                swapped_right, fresh_right,
                "{backend:?}: the right lane differs"
            );
        }
    }
}

#[test]
fn a_changed_lane_delay_restarts_only_its_lane_on_every_width() {
    for backend in backends() {
        a_changed_lane_delay_restarts_only_its_lane(backend);
    }
}

/// Gate 2b, #1953a (D15-9: a re-point is a removal plus an addition): B re-points the send
/// [`SEND`] (a) from `bus1` to `bus2`, or (b) to `bus1` from its `pre_fader` tap (no submix has a
/// delay and the buses hold no state, so the
/// route's ramp is the only memory) while A's cells hold a gain ramp on it in flight.
/// `restarted_strips()` is empty: the re-pointed send's source strip `eq3` keeps its state. The
/// successor arms the send as an added route (`D = 0`, the session mute ramp), and every block from
/// the swap block on equals B with the send muted, rendered fresh from frame 0, given a live unmute
/// at the swap block: the new destination hears a fade from the fire block and A's ramp does not
/// carry. The oracle can fail: the unmute one block late differs, and the control, which keeps the
/// send where it was, carries the ramp in flight. A re-point from a lane-less route is #1953b's
/// (`a_removed_or_repointed_lane_less_route_fades_out_through_its_line`).
fn a_repointed_send_fades_in_at_its_new_destination(backend: Backend) {
    let feed = Feed::new(SOURCE, 1);
    let a_model = send_session(false);
    let ramp = LiveRamps::for_session(&a_model).mute_samples;
    let template = a_model
        .routes
        .iter()
        .find(|route| route.id.as_str() == SEND)
        .expect("the send")
        .clone();
    let send = |block, gain_db: f32, muted, length| {
        let matrix = &template.channel_matrix;
        CellWrite {
            block,
            strip: "",
            record: CellRecord::Route(
                SEND,
                host_core::route_record(
                    gain_db,
                    [matrix.ll, matrix.lr, matrix.rl, matrix.rr],
                    muted,
                    [false; 2],
                    length,
                )
                .expect("a send record"),
            ),
        }
    };
    let in_flight = [send(SWAP_BLOCK - 2, -9.0, false, LONG_RAMP)];
    // (a) another destination; (b) the same destination from another tap: `known`'s source and
    // tap check.
    type Repoint = fn(&mut session::Route);
    let repoints: [(&str, Repoint); 2] = [
        ("(a) to bus2", |route| {
            route.destination = session::RouteDestination::SubmixInput {
                submix_id: id("bus2"),
            };
        }),
        ("(b) from pre_fader", |route| {
            let RouteSource::Track { track_id, tap } = route.source.clone() else {
                panic!("a track route");
            };
            assert_ne!(tap, session::SendTap::PreFader, "the tap moves");
            route.source = RouteSource::Track {
                track_id,
                tap: session::SendTap::PreFader,
            };
        }),
    ];
    // The control keeps the send where it was.
    let kept_model = with_muted_output_track(a_model.clone());
    let (a, kept) = (
        Session::compile(a_model.clone()),
        Session::compile(kept_model),
    );
    let swap_to = |to: &Session| {
        fade_run(
            &a,
            &[FadeSwap {
                block: SWAP_BLOCK,
                to,
                committed: &a.model,
                prepared_at: None,
            }],
            &[&feed],
            None,
            &in_flight,
            backend,
        )
    };
    for (case, change) in repoints {
        let repoint = |muted| {
            let mut model = with_muted_output_track(a_model.clone());
            let route = route_mut(&mut model, SEND);
            change(route);
            route.mute = muted;
            model
        };
        let (b, muted) = (
            Session::compile(repoint(false)),
            Session::compile(repoint(true)),
        );
        let run = swap_to(&b);
        assert_eq!(
            run.restarted[0],
            Vec::<Box<str>>::new(),
            "{backend:?} {case}: the re-pointed send's source strip keeps its state"
        );
        // #2080: the added muted output track's fader arm (first), then the send's.
        assert_eq!(
            run.arms[0],
            vec![(0, ramp), (0, ramp)],
            "{backend:?} {case}: the muted track's and the send's D and ramp"
        );
        assert_eq!(
            run.allocator,
            (0, 0, 0),
            "{backend:?} {case}: allocator calls"
        );
        let reference = |block| {
            fade_run(
                &muted,
                &[],
                &[&feed],
                None,
                &[send(block, template.gain_db, false, ramp)],
                backend,
            )
        };
        let at_swap = reference(SWAP_BLOCK);
        for block in SWAP_BLOCK..BLOCKS {
            assert_eq!(
                run.blocks[block], at_swap.blocks[block],
                "{backend:?} {case}: block {block}: the re-pointed send fades in from the swap"
            );
        }
        assert_ne!(
            reference(SWAP_BLOCK + 1).blocks[SWAP_BLOCK],
            at_swap.blocks[SWAP_BLOCK],
            "{backend:?} {case}: the fire block is observable"
        );
    }
    let carried = swap_to(&kept);
    let unmoved = fade_run(&kept, &[], &[&feed], None, &[], backend);
    assert_ne!(
        carried.blocks[SWAP_BLOCK + 1],
        unmoved.blocks[SWAP_BLOCK + 1],
        "{backend:?}: a send that keeps its destination carries its ramp"
    );
}

#[test]
fn a_repointed_send_fades_in_at_its_new_destination_on_every_width() {
    for backend in backends() {
        a_repointed_send_fades_in_at_its_new_destination(backend);
    }
}

/// Gate 2c: B changes `eq0`'s delay and re-points `eq3`'s send. The restart set is exactly the
/// delay-changed strip.
fn only_the_delay_changed_strip_is_restarted(backend: Backend) {
    let feed = Feed::new(SOURCE, 1);
    let a_model = send_session(true);
    let mut b_model = with_strip_delay(a_model.clone(), "eq0", [37, 60]);
    route_mut(&mut b_model, SEND).destination = session::RouteDestination::SubmixInput {
        submix_id: id("bus2"),
    };
    let (a, b) = (Session::compile(a_model), Session::compile(b_model));
    let run = effect_run(
        &a,
        Some((&b, &a.model)),
        &feed,
        &send_writes(),
        &[],
        backend,
    );
    assert_clean_swap(&run, &format!("{backend:?}"));
    assert_eq!(run.restarted, vec![Box::<str>::from("eq0")], "{backend:?}");
}

#[test]
fn only_the_delay_changed_strip_is_restarted_on_every_width() {
    for backend in backends() {
        only_the_delay_changed_strip_is_restarted(backend);
    }
}

/// Gate 5 (#1225 D8): B is A plus a muted track that sorts first, and changes the carried send's
/// gain. Run 1 is the structural swap, whose successor retargets the carried send to B's gain over
/// the session's send ramp. Run 2 writes the same value to A just before the swap and prepares B
/// from a base that already holds it. Every block of the two runs is bit-identical. Red if a
/// carried send keeps the predecessor's level (a missing retarget), or if the retarget lands a
/// block early or late.
fn a_carried_send_is_retargeted_to_the_successors_gain(backend: Backend) {
    const GAIN: f32 = -7.5;
    let feed = Feed::new(SOURCE, 1);
    let a_model = send_session(true);
    let mut changed = a_model.clone();
    route_mut(&mut changed, SEND).gain_db = GAIN;
    let b_model = with_muted_output_track(changed.clone());
    let (a, b) = (Session::compile(a_model), Session::compile(b_model));
    let holds = Session::compile(changed);
    let length = LiveRamps::for_session(&b.model).resolve(LiveRampRow::SendGain, None);
    assert!(length > 0, "the session's send ramp is not a step");
    // The record in flight at the swap, as in gate 1.
    let mut run_one_writes = send_writes();
    run_one_writes.pop();
    let mut run_two_writes = run_one_writes.clone();
    run_two_writes.push(EffectWrite {
        block: SWAP_BLOCK,
        strip: "",
        address: EQ_ADDRESS,
        value: EffectValue::SendGain {
            route: SEND,
            gain_db: GAIN,
            length,
        },
    });
    let one = effect_run(
        &a,
        Some((&b, &a.model)),
        &feed,
        &run_one_writes,
        &[],
        backend,
    );
    let two = effect_run(
        &a,
        Some((&b, &holds.model)),
        &feed,
        &run_two_writes,
        &[],
        backend,
    );
    assert_clean_swap(&one, &format!("{backend:?} run 1"));
    assert_clean_swap(&two, &format!("{backend:?} run 2"));
    assert_eq!(one.restarted, Vec::<Box<str>>::new(), "{backend:?}");
    assert_eq!(two.restarted, Vec::<Box<str>>::new(), "{backend:?}");
    assert_eq!(
        first_effect_difference(&one, &two),
        None,
        "{backend:?}: the retarget equals a live write before the swap"
    );
    // Not vacuous: a swap that leaves the send at the predecessor's level differs.
    let stale_b = Session::compile(with_muted_output_track(send_session(true)));
    let stale = effect_run(
        &a,
        Some((&stale_b, &a.model)),
        &feed,
        &run_one_writes,
        &[],
        backend,
    );
    assert_ne!(
        stale.blocks[SWAP_BLOCK + 1],
        one.blocks[SWAP_BLOCK + 1],
        "{backend:?}: the new gain is audible after the swap block"
    );
}

#[test]
fn a_carried_send_is_retargeted_to_the_successors_gain_on_every_width() {
    for backend in backends() {
        a_carried_send_is_retargeted_to_the_successors_gain(backend);
    }
}

// ---------------------------------------------------------------------------------------------
// #1395a: a single spectrum capture keeps its history across a plan swap.
//
// A capture whose target and channels are the predecessor's keeps its analysis history, its open
// window, its stream epoch, its sequence and its one-shot state: the observer moves into the
// successor's plan in the swap block, and the control side keeps the reader that is connected to
// it, pairing it at promotion from the list the successor's preparation built. The oracle is the
// post-edit session prepared fresh with the same capture, started at the same block and fed from
// frame 0.

mod spectrum_carry {
    use engine::realtime::{HostAttachment, ObserverCarry, ObserverKind, PlanAdoption};
    use host_core::{
        HostLiveControlRequest, HostLiveLanes, LiveValuesSuperseded, PreparedHost,
        PreparedSpectrumCaptures, SpectrumCaptureCollectionEntry, SpectrumCaptureCollectionRequest,
        SpectrumCaptureRequest, SpectrumChannels, SpectrumContinuousWindow, SpectrumHop,
        SpectrumPair, SpectrumPreparation, SpectrumService, SpectrumServiceError,
        SpectrumSmoothingConfig, SpectrumSnapshot, SpectrumStreamStatus, SpectrumTarget,
        SuccessorBase, prepare_host_runtime_with_live_lanes_and_spectrum,
        spectrum_capture_collection_resources, spectrum_capture_resources_for,
    };

    use super::successor::{self, Block, SUCCESSOR_REVISION};
    use super::*;

    /// Blocks per run: 8192 frames, four windows of the 2048-frame capture and then some.
    const TOTAL: usize = 64;
    /// The swap block: sample 5120, inside the window that opened at 4608 (hop 512).
    const SWAP: usize = 40;
    const SWAP_SAMPLE: u64 = (SWAP * QUANTUM) as u64;
    /// A one-shot is armed at block 30 and completes 16 blocks later, past the swap.
    const ARM: usize = 30;
    const TRACK: &str = "eq0";

    fn request(target: SpectrumTarget, channels: SpectrumChannels) -> SpectrumCaptureRequest {
        SpectrumCaptureRequest {
            maximum_capture_bytes: spectrum_capture_resources_for(&target).retained_bytes,
            target,
            channels,
        }
    }

    fn stereo_post_matrix() -> SpectrumCaptureRequest {
        request(
            SpectrumTarget::TrackPostMatrix(TRACK.into()),
            SpectrumChannels::Stereo,
        )
    }

    /// What the host does to the service of a collection: which entry it selects before the stream
    /// starts, and which entry it switches the running stream to, at which block.
    #[derive(Clone, Default)]
    struct Script {
        select: Option<(SpectrumTarget, SpectrumChannels)>,
        switch: Option<(usize, SpectrumTarget, SpectrumChannels)>,
    }

    /// A capture request of either kind, prepared fresh or as a successor.
    trait Spec {
        fn prepare(
            &self,
            session: &Session,
            base: Option<SuccessorBase<'_>>,
        ) -> (PreparedHost, PreparedSpectrumCaptures);

        fn script(&self) -> Script {
            Script::default()
        }
    }

    impl Spec for SpectrumCaptureRequest {
        fn prepare(
            &self,
            session: &Session,
            base: Option<SuccessorBase<'_>>,
        ) -> (PreparedHost, PreparedSpectrumCaptures) {
            let live = HostLiveControlRequest::default();
            let superseded = LiveValuesSuperseded::default();
            let (host, _handles, captures) = prepare_host_runtime_with_live_lanes_and_spectrum(
                &session.compiled,
                &caps(),
                &live,
                HostLiveLanes::ALL,
                Some(&SpectrumPreparation::Single(self.clone())),
                base,
                &superseded,
            )
            .unwrap_or_else(|failure| panic!("prepare: {failure:?}"));
            (host, captures.expect("the captures"))
        }
    }

    /// A collection request with the host's script for it.
    struct Many {
        request: SpectrumCaptureCollectionRequest,
        script: Script,
    }

    impl Spec for Many {
        fn prepare(
            &self,
            session: &Session,
            base: Option<SuccessorBase<'_>>,
        ) -> (PreparedHost, PreparedSpectrumCaptures) {
            let live = HostLiveControlRequest::default();
            let superseded = LiveValuesSuperseded::default();
            let (host, _handles, captures) = prepare_host_runtime_with_live_lanes_and_spectrum(
                &session.compiled,
                &caps(),
                &live,
                HostLiveLanes::ALL,
                Some(&SpectrumPreparation::Collection(self.request.clone())),
                base,
                &superseded,
            )
            .unwrap_or_else(|failure| panic!("prepare: {failure:?}"));
            (host, captures.expect("the captures"))
        }

        fn script(&self) -> Script {
            self.script.clone()
        }
    }

    /// The test's host attachment: it records the carry record the swap block hands it, within the
    /// capacity reserved at construction.
    struct Rig {
        carries: Vec<ObserverCarry>,
    }

    impl Rig {
        fn new() -> Box<Self> {
            Box::new(Self {
                carries: Vec::with_capacity(16),
            })
        }
    }

    impl HostAttachment for Rig {
        fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
            self
        }

        fn adopt_predecessor(
            &mut self,
            predecessor: &mut dyn HostAttachment,
            carries: &[ObserverCarry],
        ) {
            if predecessor.as_any_mut().downcast_mut::<Self>().is_some() {
                self.carries.extend_from_slice(carries);
            }
        }
    }

    /// What one read of the stream returned.
    #[derive(Clone, Copy)]
    struct Read {
        status: SpectrumStreamStatus,
        window: Option<SpectrumContinuousWindow>,
    }

    #[derive(Clone, Copy, Debug)]
    enum Drive {
        /// A continuous stream started before block 0 and read after every block.
        Stream,
        /// A one-shot armed before the given block and read after every block from there on.
        OneShot(usize),
        /// As `OneShot`, but a refused read is recorded, and after `Dropped` on a capture that can
        /// arm the host arms again at once (#1618, #1593 D6).
        OneShotRearm(usize),
    }

    /// The host's reader of one run: the service and what it read.
    struct Driver {
        service: SpectrumService,
        drive: Drive,
        script: Script,
        reads: Vec<Read>,
        shot: Option<SpectrumSnapshot>,
        /// The refusals the one-shot's reads answered, with the block after which each came.
        refused: Vec<(usize, SpectrumServiceError)>,
        /// The block after which the one-shot completed.
        shot_at: Option<usize>,
    }

    impl Driver {
        fn new(
            captures: PreparedSpectrumCaptures,
            rate: u32,
            drive: Drive,
            script: Script,
        ) -> Self {
            let mut service = SpectrumService::new(
                captures,
                rate,
                QUANTUM as u32,
                Some(SpectrumHop::new(512).expect("hop")),
            );
            if let Some((target, channels)) = &script.select {
                service
                    .select(target.as_ref(), *channels)
                    .expect("select the first entry");
            }
            if matches!(drive, Drive::Stream) {
                service
                    .start_stream(SpectrumSmoothingConfig::default(), None)
                    .expect("start the stream");
            }
            Self {
                service,
                drive,
                script,
                reads: Vec::new(),
                shot: None,
                refused: Vec::new(),
                shot_at: None,
            }
        }

        fn before(&mut self, block: usize) {
            if let Some((at, target, channels)) = &self.script.switch
                && *at == block
            {
                self.service
                    .select_stream(target.as_ref(), *channels, None, None)
                    .expect("switch the stream to the other entry");
            }
            if matches!(self.drive, Drive::OneShot(at) | Drive::OneShotRearm(at) if at == block) {
                if matches!(self.drive, Drive::OneShotRearm(_)) {
                    // A collection's select armed its entry for nobody: the host's own arm
                    // replaces that one.
                    let _ = self.service.cancel();
                }
                self.service.arm().expect("arm");
            }
        }

        fn after(&mut self, block: usize) {
            match self.drive {
                Drive::Stream => {
                    self.service.read_stream(None).expect("read the stream");
                    self.reads.push(Read {
                        status: self.service.stream_metadata().status,
                        window: self.service.stream_window().copied(),
                    });
                }
                Drive::OneShot(at) if block >= at && self.shot.is_none() => {
                    self.shot = self
                        .service
                        .read(SpectrumChannels::Stereo)
                        .expect("read the one-shot");
                }
                Drive::OneShotRearm(at) if block >= at && self.shot.is_none() => {
                    // Without a selection the read refuses before it looks at the mask.
                    let channels = self.service.channels().unwrap_or(SpectrumChannels::Stereo);
                    match self.service.read(channels) {
                        Ok(Some(snapshot)) => {
                            self.shot = Some(snapshot);
                            self.shot_at = Some(block);
                        }
                        Ok(None) => {}
                        Err(error) => {
                            self.refused.push((block, error));
                            if error == SpectrumServiceError::Dropped
                                && self.service.channels().is_some()
                            {
                                self.service.arm().expect("arm again after the refusal");
                            }
                        }
                    }
                }
                Drive::OneShot(_) | Drive::OneShotRearm(_) => {}
            }
        }
    }

    /// How the host promotes the successor after its adoption block.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Promote {
        /// As the engine reported the block: `carried` is `CarryOutcome::Carried`.
        AsReported,
        /// As after a refused hand-over: `carried` is false although the list names a pair.
        Refused,
    }

    struct Run {
        blocks: Vec<Block>,
        driver: Driver,
        /// The spectrum entries of the carry record the swap block wrote.
        record: Vec<ObserverCarry>,
        /// The successor's pairing list.
        pairing: Vec<SpectrumPair>,
        /// `(allocations, reallocations, deallocations)` of the promotion call.
        adopt_counts: (u64, u64, u64),
        /// The captures the promotion displaced, dropped last.
        displaced: PreparedSpectrumCaptures,
        _exchange: (
            engine::realtime::PlanPublisher,
            engine::realtime::PlanRetirer,
        ),
    }

    /// `a` rendered to [`SWAP`], `b` prepared as its successor and swapped in, promoted after its
    /// adoption block, rendered to [`TOTAL`].
    fn swapped(
        a: &Session,
        a_request: &impl Spec,
        b: &Session,
        b_request: &impl Spec,
        drive: Drive,
        promote: Promote,
    ) -> Run {
        let backend = Backend::current();
        let _ = backend;
        let feed = Feed::with_blocks(SOURCE, 1, TOTAL);
        let (mut a_prepared, a_capture) = a_request.prepare(a, None);
        a_prepared.plan.set_host_attachment(Rig::new());
        let rate = a_prepared.report.sample_rate_hz;
        let PreparedHost {
            plan,
            sources: mut a_sources,
            inventory,
            ..
        } = a_prepared;
        let (mut publisher, mut owner, retirer) = successor::exchange(plan);
        let mut driver = Driver::new(a_capture, rate, drive, a_request.script());
        let mut blocks = Vec::new();
        feed.submit(&mut a_sources, rate, 0);
        for block in 0..SWAP {
            feed.submit(&mut a_sources, rate, block + 1);
            driver.before(block);
            blocks.push(successor::render(&mut owner));
            driver.after(block);
        }
        let (mut b_prepared, b_capture) = b_request.prepare(
            b,
            Some(SuccessorBase::new(&inventory, &a.model, SUCCESSOR_REVISION)),
        );
        b_prepared.sources.adopt_persisting(&mut a_sources);
        let pairing = b_prepared.inventory.spectrum_pairing().to_vec();
        b_prepared.plan.set_host_attachment(Rig::new());
        let PreparedHost {
            mut plan,
            sources: mut b_sources,
            ..
        } = b_prepared;
        plan.take_revision_gate()
            .expect("the successor's handle")
            .publish(SUCCESSOR_REVISION);
        publisher
            .reserve_replacement(plan, PlanAdoption::Next)
            .unwrap_or_else(|_| panic!("reserve the successor"))
            .commit(SUCCESSOR_REVISION);
        // From the swap on the host acts on the successor's collection, in the successor's terms.
        driver.script = b_request.script();
        let mut b_capture = Some(b_capture);
        let mut adopt_counts = (0, 0, 0);
        let mut displaced = None;
        for block in SWAP..TOTAL {
            if block + 1 < TOTAL {
                feed.submit(&mut b_sources, rate, block + 1);
            }
            driver.before(block);
            blocks.push(successor::render(&mut owner));
            if block == SWAP {
                let carried =
                    blocks[SWAP].carry == CarryOutcome::Carried && promote == Promote::AsReported;
                let mark = bench_support::alloc::current_thread_counters();
                displaced = Some(driver.service.adopt_successor(
                    b_capture.take().expect("the successor's capture"),
                    &pairing,
                    carried,
                    None,
                ));
                let delta = bench_support::alloc::current_thread_delta_since(mark);
                adopt_counts = (delta.allocations, delta.reallocations, delta.deallocations);
            }
            driver.after(block);
        }
        let record = owner
            .active_attachment_mut::<Rig>()
            .expect("attachment")
            .carries
            .iter()
            .copied()
            .filter(|carry| carry.kind == ObserverKind::Spectrum)
            .collect();
        Run {
            blocks,
            driver,
            record,
            pairing,
            adopt_counts,
            displaced: displaced.expect("promoted"),
            _exchange: (publisher, retirer),
        }
    }

    /// `b` prepared fresh with `request` and read as `drive` from frame 0.
    fn reference(b: &Session, request: &impl Spec, drive: Drive) -> Driver {
        let feed = Feed::with_blocks(SOURCE, 1, TOTAL);
        let (prepared, capture) = request.prepare(b, None);
        let rate = prepared.report.sample_rate_hz;
        let PreparedHost {
            plan, mut sources, ..
        } = prepared;
        let (_publisher, mut owner, _retirer) = successor::exchange(plan);
        let mut driver = Driver::new(capture, rate, drive, request.script());
        feed.submit(&mut sources, rate, 0);
        for block in 0..TOTAL {
            if block + 1 < TOTAL {
                feed.submit(&mut sources, rate, block + 1);
            }
            driver.before(block);
            successor::render(&mut owner);
            driver.after(block);
        }
        driver
    }

    /// A: `eq0` and `eq1`. B adds a muted track that sorts first, so every observer of the
    /// successor sits at another place than the predecessor's.
    fn sessions() -> (Session, Session) {
        let a = Session::compile(session_a());
        let mut b_model = session_a();
        add_track(&mut b_model, "eq0", MUTED_TRACK, SOURCE, true);
        (a, Session::compile(b_model))
    }

    fn assert_same_window(
        got: &SpectrumContinuousWindow,
        want: &SpectrumContinuousWindow,
        label: &str,
    ) {
        assert_eq!(got.first_sample, want.first_sample, "{label}: first_sample");
        assert_eq!(got.channels, want.channels, "{label}: channels");
        assert_eq!(got.source_underrun, want.source_underrun, "{label}");
        assert_eq!(got.stream_epoch, want.stream_epoch, "{label}: epoch");
        assert_eq!(got.sequence, want.sequence, "{label}: sequence");
        assert_eq!(got.dropped_captures, want.dropped_captures, "{label}");
        assert!(
            got.left
                .iter()
                .zip(&want.left)
                .all(|(a, b)| a.to_bits() == b.to_bits())
                && got
                    .right
                    .iter()
                    .zip(&want.right)
                    .all(|(a, b)| a.to_bits() == b.to_bits()),
            "{label}: the planes equal the reference's bit for bit"
        );
    }

    /// Gate 1: a continuous capture keeps reading, window for window, across the swap.
    #[test]
    fn a_continuous_capture_carries_and_its_reader_keeps_receiving() {
        let (a, b) = sessions();
        let request = stereo_post_matrix();
        let expected = reference(&b, &request, Drive::Stream);
        let run = swapped(
            &a,
            &request,
            &b,
            &request,
            Drive::Stream,
            Promote::AsReported,
        );
        assert_eq!(run.blocks[SWAP].carry, CarryOutcome::Carried);
        assert_eq!(run.blocks[SWAP].swap, SwapOutcome::Applied);
        assert_eq!(
            run.pairing,
            vec![SpectrumPair {
                successor: 0,
                predecessor: 0
            }]
        );
        assert_eq!(
            run.record,
            vec![ObserverCarry {
                kind: ObserverKind::Spectrum,
                successor: 0,
                predecessor: 0
            }],
            "the swap block moved exactly the observers the control side's list pairs"
        );

        let reads = &run.driver.reads;
        assert_eq!(reads.len(), TOTAL);
        let first_ready = reads
            .iter()
            .position(|read| read.status == SpectrumStreamStatus::Ready)
            .expect("the stream produces before the swap");
        assert!(first_ready < SWAP, "a window is complete before the swap");
        let mut next_sequence = 0;
        let mut spanning = false;
        let mut after_swap = 0;
        for (block, (got, want)) in reads.iter().zip(&expected.reads).enumerate() {
            assert_eq!(got.status, want.status, "block {block}: status");
            if block > first_ready {
                assert!(
                    matches!(
                        got.status,
                        SpectrumStreamStatus::Ready | SpectrumStreamStatus::Pending
                    ),
                    "block {block}: no Failed, Gap or Warming after the stream first produced, got {:?}",
                    got.status
                );
            }
            match (&got.window, &want.window) {
                (Some(got), Some(want)) => {
                    assert_same_window(got, want, &format!("block {block}"));
                    assert_eq!(got.sequence, next_sequence, "block {block}: no gap");
                    assert_eq!(got.stream_epoch, 1, "block {block}: the epoch never moved");
                    next_sequence += 1;
                    spanning |= got.first_sample < SWAP_SAMPLE
                        && got.end_sample().expect("end") > SWAP_SAMPLE;
                    after_swap += usize::from(block >= SWAP);
                }
                (None, None) => {}
                _ => panic!("block {block}: a window on one side only"),
            }
        }
        assert!(
            spanning,
            "a window spans the swap, or the carry is not tested"
        );
        assert!(after_swap >= 3, "the reader keeps receiving after the swap");
    }

    /// Without the pairing the host reads the predecessor's capture, whose observer sits in the
    /// displaced plan: the oracle above can fail.
    #[test]
    fn a_refused_hand_over_installs_the_fresh_capture() {
        let (a, b) = sessions();
        let request = stereo_post_matrix();
        let run = swapped(&a, &request, &b, &request, Drive::Stream, Promote::Refused);
        assert_eq!(run.blocks[SWAP].carry, CarryOutcome::Carried);
        // The service reads the successor's own capture now. Its observer sits in a plan that
        // carried its predecessor's observer away, so it renders nothing: the stream the service
        // still believes it runs has no epoch and no window.
        let after = &run.driver.reads[SWAP + 1..];
        assert!(
            after.iter().all(|read| read.window.is_none()),
            "the fresh capture has no stream: the carried observer is not its producer"
        );
        assert_eq!(
            after.last().expect("read").status,
            SpectrumStreamStatus::Inactive
        );
        // The capture that was displaced is the predecessor's, still connected to the observer
        // that renders on, and still on its stream (the host drops it off the audio thread).
        let PreparedSpectrumCaptures::Single(old) = &run.displaced else {
            panic!("a single capture");
        };
        assert_eq!(old.stream_epoch(), Some(1));
        // The stream the service ran on the old capture ended with it (#1618): the host starts
        // one anew on the successor's capture.
        let mut run = run;
        assert!(!run.driver.service.stream_active());
        assert_eq!(run.driver.service.stream_cadence(), None);
        run.driver
            .service
            .start_stream(SpectrumSmoothingConfig::default(), None)
            .expect("a stream starts on the fresh capture");
    }

    /// Gate 3, one-shot: a capture armed before the swap and carried completes with a window that
    /// began before the swap block.
    #[test]
    fn an_armed_one_shot_completes_across_the_swap() {
        let (a, b) = sessions();
        let request = stereo_post_matrix();
        let expected = reference(&b, &request, Drive::OneShot(ARM));
        let run = swapped(
            &a,
            &request,
            &b,
            &request,
            Drive::OneShot(ARM),
            Promote::AsReported,
        );
        let got = run.driver.shot.expect("the carried one-shot completes");
        let want = expected.shot.expect("the reference completes");
        assert!(
            got.window.first_sample < SWAP_SAMPLE
                && got.window.end_sample().expect("end") > SWAP_SAMPLE,
            "the window spans the swap block"
        );
        assert_eq!(got.window.first_sample, want.window.first_sample);
        assert_eq!(got.window.channels, want.window.channels);
        assert!(
            got.window
                .left
                .iter()
                .zip(&want.window.left)
                .chain(got.window.right.iter().zip(&want.window.right))
                .all(|(a, b)| a.to_bits() == b.to_bits()),
            "the planes equal the reference's bit for bit"
        );
    }

    /// Gate 3, key: a changed channel mask carries nothing, the list is empty, and the fresh
    /// capture starts its own stream.
    #[test]
    fn a_changed_key_starts_fresh() {
        let (a, b) = sessions();
        let a_request = stereo_post_matrix();
        let b_request = request(
            SpectrumTarget::TrackPostMatrix(TRACK.into()),
            SpectrumChannels::Left,
        );
        let mut run = swapped(
            &a,
            &a_request,
            &b,
            &b_request,
            Drive::Stream,
            Promote::AsReported,
        );
        assert!(
            run.pairing.is_empty(),
            "no pair across a changed channel mask"
        );
        assert!(run.record.is_empty(), "the swap block moved no capture");
        // The host restarts the stream it had on the old capture, on the successor's.
        run.driver.service.stop_stream(None);
        run.driver
            .service
            .start_stream(SpectrumSmoothingConfig::default(), None)
            .expect("restart");
        // The successor's plan renders on past TOTAL: not available here, so the proof is the
        // service's own: it reports the fresh capture's mask, not the carried one's.
        assert_eq!(run.driver.service.channels(), Some(SpectrumChannels::Left));
        assert_eq!(
            run.driver.service.stream_epoch(),
            Some(1),
            "a fresh epoch, not the predecessor's"
        );
    }

    /// #1618 gate 1: a one-shot armed on a capture that does not carry (its channel mask changed)
    /// would wait for an observer that no longer renders. The service drops it: the read after
    /// the promotion answers `Dropped` once (#1593 D6), and the arm that follows completes on the
    /// successor with the window a fresh arm there gives.
    #[test]
    fn a_one_shot_that_does_not_carry_is_dropped_and_a_new_arm_completes() {
        let (a, b) = sessions();
        let a_request = stereo_post_matrix();
        let b_request = request(
            SpectrumTarget::TrackPostMatrix(TRACK.into()),
            SpectrumChannels::Left,
        );
        let run = swapped(
            &a,
            &a_request,
            &b,
            &b_request,
            Drive::OneShotRearm(ARM),
            Promote::AsReported,
        );
        assert!(run.pairing.is_empty(), "the changed key carries nothing");
        assert_eq!(
            run.driver.refused,
            vec![(SWAP, SpectrumServiceError::Dropped)],
            "pending until the promotion, one refusal at it, none after"
        );
        let got = run
            .driver
            .shot
            .expect("the new arm completes on the successor");
        let shot_at = run.driver.shot_at.expect("completed");
        assert!(shot_at > SWAP && shot_at < TOTAL, "completed at {shot_at}");
        assert!(
            got.window.first_sample > SWAP_SAMPLE,
            "a window of the successor, armed after the promotion"
        );
        assert_eq!(got.window.channels, SpectrumChannels::Left);
        // The window equals what a fresh arm on the same plan gives at the same block.
        let expected = reference(&b, &b_request, Drive::OneShotRearm(SWAP + 1));
        let want = expected.shot.expect("the reference completes");
        assert_eq!(got.window.first_sample, want.window.first_sample);
        assert!(
            got.window
                .left
                .iter()
                .zip(&want.window.left)
                .all(|(a, b)| a.to_bits() == b.to_bits()),
            "the plane equals the reference's bit for bit"
        );
    }

    /// #1618 gate 1, refused hand-over: the list names a pair but the adoption did not carry, so
    /// the arm is dropped the same way. (The engine did carry the observer here, so the
    /// re-armed fresh capture has no producer and stays pending: the point is the refusal.)
    #[test]
    fn a_one_shot_after_a_refused_hand_over_is_dropped() {
        let (a, b) = sessions();
        let request = stereo_post_matrix();
        let run = swapped(
            &a,
            &request,
            &b,
            &request,
            Drive::OneShotRearm(ARM),
            Promote::Refused,
        );
        assert_eq!(
            run.driver.refused,
            vec![(SWAP, SpectrumServiceError::Dropped)]
        );
        assert!(run.driver.shot.is_none());
    }

    /// A one-shot that completed and was read before the swap leaves nothing to settle: the next
    /// read after the promotion is not a refusal, and a stream may start on the fresh capture.
    #[test]
    fn an_idle_capture_that_does_not_carry_settles_nothing() {
        let (a, b) = sessions();
        let a_request = stereo_post_matrix();
        let b_request = request(
            SpectrumTarget::TrackPostMatrix(TRACK.into()),
            SpectrumChannels::Left,
        );
        let mut run = swapped(
            &a,
            &a_request,
            &b,
            &b_request,
            Drive::OneShotRearm(0),
            Promote::AsReported,
        );
        // Armed at block 0: it completed at block 15, long before the swap.
        assert!(
            run.driver.refused.is_empty(),
            "nothing in flight at the swap"
        );
        run.driver
            .service
            .start_stream(SpectrumSmoothingConfig::default(), None)
            .expect("the fresh capture is idle");
    }

    /// Gate 4: the swap block and the promotion allocate and free nothing.
    #[test]
    fn the_swap_block_and_the_promotion_allocate_and_free_nothing() {
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
        let (a, b) = sessions();
        let request = stereo_post_matrix();
        let run = swapped(
            &a,
            &request,
            &b,
            &request,
            Drive::Stream,
            Promote::AsReported,
        );
        assert_eq!(run.blocks[SWAP].carry, CarryOutcome::Carried);
        assert!(
            !run.record.is_empty(),
            "a capture carried through the measured block"
        );
        for (index, block) in run.blocks.iter().enumerate().skip(1) {
            assert_eq!(block.allocator, (0, 0, 0), "block {index}: allocator calls");
            assert_eq!(block.audit, (0, 0), "block {index}: render audit");
        }
        assert_eq!(
            run.adopt_counts,
            (0, 0, 0),
            "the promotion allocates and frees nothing"
        );
    }

    // -----------------------------------------------------------------------------------------
    // #1395b: a collection of captures carries entry by entry, matched by key.

    /// The block at which the host switches the running stream to the other entry.
    const SWITCH: usize = 44;

    fn entry(target: SpectrumTarget, channels: SpectrumChannels) -> SpectrumCaptureCollectionEntry {
        SpectrumCaptureCollectionEntry { target, channels }
    }

    /// Track `eq0` after its matrix, both channels: the entry the stream runs on.
    fn t_stereo() -> SpectrumCaptureCollectionEntry {
        entry(
            SpectrumTarget::TrackPostMatrix(TRACK.into()),
            SpectrumChannels::Stereo,
        )
    }

    /// Track `eq1` after its input builtins, one channel.
    fn u_entry(channels: SpectrumChannels) -> SpectrumCaptureCollectionEntry {
        entry(
            SpectrumTarget::TrackPostInputBuiltins("eq1".into()),
            channels,
        )
    }

    fn many(
        entries: Vec<SpectrumCaptureCollectionEntry>,
        switch: Option<SpectrumCaptureCollectionEntry>,
    ) -> Many {
        let maximum_capture_bytes = spectrum_capture_collection_resources(&entries)
            .expect("collection resources")
            .retained_bytes;
        Many {
            request: SpectrumCaptureCollectionRequest {
                entries,
                maximum_capture_bytes,
            },
            script: Script {
                select: Some((t_stereo().target, t_stereo().channels)),
                switch: switch.map(|entry| (SWITCH, entry.target, entry.channels)),
            },
        }
    }

    fn sorted(mut pairs: Vec<SpectrumPair>) -> Vec<(usize, usize)> {
        pairs.sort_by_key(|pair| pair.successor);
        pairs
            .into_iter()
            .map(|pair| (pair.successor, pair.predecessor))
            .collect()
    }

    /// The reads of `got` equal `want`'s block for block from `from` on, windows bit for bit.
    fn assert_reads_equal(got: &Driver, want: &Driver, from: usize) -> usize {
        let mut windows = 0;
        for (block, (got, want)) in got.reads.iter().zip(&want.reads).enumerate().skip(from) {
            assert_eq!(got.status, want.status, "block {block}: status");
            match (&got.window, &want.window) {
                (Some(got), Some(want)) => {
                    assert_same_window(got, want, &format!("block {block}"));
                    windows += 1;
                }
                (None, None) => {}
                _ => panic!("block {block}: a window on one side only"),
            }
        }
        windows
    }

    /// The stream on `t` kept receiving across the swap, as in gate 1: no `Failed`, `Gap` or
    /// `Warming` after it first produced, one epoch, consecutive sequences, a window across the
    /// swap, and windows after it.
    fn assert_stream_kept_receiving(driver: &Driver) {
        let reads = &driver.reads[..SWITCH];
        let first_ready = reads
            .iter()
            .position(|read| read.status == SpectrumStreamStatus::Ready)
            .expect("the stream produces before the swap");
        assert!(first_ready < SWAP);
        let mut next_sequence = 0;
        let mut spanning = false;
        let mut after_swap = 0;
        for (block, read) in reads.iter().enumerate().skip(first_ready) {
            assert!(
                matches!(
                    read.status,
                    SpectrumStreamStatus::Ready | SpectrumStreamStatus::Pending
                ),
                "block {block}: no Failed, Gap or Warming, got {:?}",
                read.status
            );
            if let Some(window) = &read.window {
                assert_eq!(window.sequence, next_sequence, "block {block}: no gap");
                assert_eq!(
                    window.stream_epoch, 1,
                    "block {block}: the epoch never moved"
                );
                next_sequence += 1;
                spanning |= window.first_sample < SWAP_SAMPLE
                    && window.end_sample().expect("end") > SWAP_SAMPLE;
                after_swap += usize::from(block >= SWAP);
            }
        }
        assert!(
            spanning,
            "a window spans the swap, or the carry is not tested"
        );
        assert!(after_swap >= 1, "the reader keeps receiving after the swap");
    }

    /// Gate 2: B lists the entries in the other order. Each pairs by key, the selection follows
    /// its entry to the new index with its epoch, the stream reads on, and a switch to the other
    /// entry (which also carried, idle) starts it as the reference does.
    #[test]
    fn a_reordered_collection_pairs_by_key_and_its_reader_keeps_receiving() {
        let (a, b) = sessions();
        let a_request = many(vec![t_stereo(), u_entry(SpectrumChannels::Left)], None);
        let b_request = many(
            vec![u_entry(SpectrumChannels::Left), t_stereo()],
            Some(u_entry(SpectrumChannels::Left)),
        );
        let expected = reference(&b, &b_request, Drive::Stream);
        let run = swapped(
            &a,
            &a_request,
            &b,
            &b_request,
            Drive::Stream,
            Promote::AsReported,
        );
        assert_eq!(run.blocks[SWAP].carry, CarryOutcome::Carried);
        assert_eq!(
            sorted(run.pairing.clone()),
            vec![(0, 1), (1, 0)],
            "paired by key, not by position"
        );
        assert_eq!(
            sorted(
                run.record
                    .iter()
                    .map(|carry| SpectrumPair {
                        successor: carry.successor,
                        predecessor: carry.predecessor
                    })
                    .collect()
            ),
            vec![(0, 1), (1, 0)],
            "the swap block moved exactly the observers the control side's list pairs"
        );
        assert_stream_kept_receiving(&run.driver);
        let windows = assert_reads_equal(&run.driver, &expected, 0);
        assert!(windows >= 8, "windows on both entries compared: {windows}");
        let switched = run.driver.reads[SWITCH..]
            .iter()
            .filter_map(|read| read.window.as_ref())
            .collect::<Vec<_>>();
        assert!(
            !switched.is_empty()
                && switched
                    .iter()
                    .all(|window| window.channels == SpectrumChannels::Left),
            "the second entry produced its own windows after the switch"
        );
        // The selection moved with its entry: still `t`, with the epoch it had.
        assert_eq!(
            run.driver.service.selection_epoch(),
            expected.service.selection_epoch()
        );
    }

    /// Gate 3, collection: one entry's channels changed. That entry starts fresh, the other
    /// carries as in gate 1.
    #[test]
    fn a_collection_entry_with_a_changed_key_starts_fresh() {
        let (a, b) = sessions();
        let a_request = many(vec![t_stereo(), u_entry(SpectrumChannels::Left)], None);
        let b_request = many(
            vec![t_stereo(), u_entry(SpectrumChannels::Right)],
            Some(u_entry(SpectrumChannels::Right)),
        );
        let expected = reference(&b, &b_request, Drive::Stream);
        let run = swapped(
            &a,
            &a_request,
            &b,
            &b_request,
            Drive::Stream,
            Promote::AsReported,
        );
        assert_eq!(sorted(run.pairing.clone()), vec![(0, 0)]);
        assert_eq!(run.record.len(), 1, "only the unchanged key moved");
        assert_stream_kept_receiving(&run.driver);
        assert!(assert_reads_equal(&run.driver, &expected, 0) >= 8);
        assert!(
            run.driver.reads[SWITCH..]
                .iter()
                .filter_map(|read| read.window.as_ref())
                .all(|window| window.channels == SpectrumChannels::Right),
            "the changed entry reads its own channel, from the plane it copies"
        );
    }

    /// A refused hand-over moved no observer: the successor's fresh collection is installed,
    /// without a selection, and its epoch says the selection was lost.
    #[test]
    fn a_refused_hand_over_installs_the_fresh_collection_without_a_selection() {
        let (a, b) = sessions();
        let a_request = many(vec![t_stereo(), u_entry(SpectrumChannels::Left)], None);
        let b_request = many(vec![u_entry(SpectrumChannels::Left), t_stereo()], None);
        let run = swapped(
            &a,
            &a_request,
            &b,
            &b_request,
            Drive::Stream,
            Promote::Refused,
        );
        assert_eq!(run.driver.service.target(), None, "no selection");
        assert_eq!(
            run.driver.service.selection_epoch(),
            2,
            "the epoch the predecessor's selection reached, plus one"
        );
        assert!(
            run.driver.reads[SWAP + 1..]
                .iter()
                .all(|read| read.window.is_none()),
            "the fresh collection has no stream"
        );
        // The displaced collection is the predecessor's, still on its selection.
        let PreparedSpectrumCaptures::Collection(old) = &run.displaced else {
            panic!("a collection");
        };
        assert_eq!(old.selected_index(), Some(0));
        assert_eq!(old.selection_epoch(), 1);
    }

    /// #1618 / #1395b: the selected entry did not carry, so the service holds no stream any more.
    /// `select_stream` answers `StreamInactive`; a plain select and a new start run the cadence
    /// again on the entry (it does not arm a one-shot in the stream's place).
    #[test]
    fn a_lost_selection_ends_the_stream_and_a_new_start_restarts_the_cadence() {
        let (a, b) = sessions();
        let a_request = many(vec![t_stereo(), u_entry(SpectrumChannels::Left)], None);
        let b_request = many(vec![u_entry(SpectrumChannels::Left), t_stereo()], None);
        let mut run = swapped(
            &a,
            &a_request,
            &b,
            &b_request,
            Drive::Stream,
            Promote::Refused,
        );
        let service = &mut run.driver.service;
        assert!(!service.stream_active(), "no stream without a selection");
        assert_eq!(service.stream_cadence(), None);
        assert_eq!(
            service
                .select_stream(
                    t_stereo().target.as_ref(),
                    SpectrumChannels::Stereo,
                    None,
                    None
                )
                .expect_err("nothing to restart"),
            SpectrumServiceError::StreamInactive
        );
        service
            .select(t_stereo().target.as_ref(), SpectrumChannels::Stereo)
            .expect("select the entry");
        service
            .start_stream(SpectrumSmoothingConfig::default(), None)
            .expect("the cadence starts again");
        assert!(service.stream_active());
        assert!(service.stream_cadence().is_some());
        assert_eq!(
            service.stream_metadata().status,
            SpectrumStreamStatus::Warming
        );
    }

    /// #1618: a one-shot armed on the selected entry of a collection whose selection did not carry
    /// ends with the selection. The read answers `Dropped` once (#1593 D6), the caller's arm is
    /// released (a select and a stream start are not `Busy`), and the epoch says the selection was
    /// lost.
    #[test]
    fn a_one_shot_on_a_lost_selection_is_released() {
        let (a, b) = sessions();
        let a_request = many(vec![t_stereo(), u_entry(SpectrumChannels::Left)], None);
        let b_request = many(vec![u_entry(SpectrumChannels::Left), t_stereo()], None);
        let mut run = swapped(
            &a,
            &a_request,
            &b,
            &b_request,
            Drive::OneShotRearm(ARM),
            Promote::Refused,
        );
        assert_eq!(
            run.driver.refused.first(),
            Some(&(SWAP, SpectrumServiceError::Dropped))
        );
        assert!(run.driver.shot.is_none());
        let service = &mut run.driver.service;
        assert_eq!(service.selection_epoch(), 2);
        service
            .select(t_stereo().target.as_ref(), SpectrumChannels::Stereo)
            .expect("the arm is released: select is not Busy");
        service
            .start_stream(SpectrumSmoothingConfig::default(), None)
            .expect("and neither is a stream start");
    }

    /// Gate 4, collection: the swap block and the promotion allocate and free nothing.
    #[test]
    fn a_collection_swap_block_and_promotion_allocate_and_free_nothing() {
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
        let (a, b) = sessions();
        let a_request = many(vec![t_stereo(), u_entry(SpectrumChannels::Left)], None);
        let b_request = many(vec![u_entry(SpectrumChannels::Left), t_stereo()], None);
        let run = swapped(
            &a,
            &a_request,
            &b,
            &b_request,
            Drive::Stream,
            Promote::AsReported,
        );
        assert_eq!(run.blocks[SWAP].carry, CarryOutcome::Carried);
        assert_eq!(run.record.len(), 2, "both captures carried");
        for (index, block) in run.blocks.iter().enumerate().skip(1) {
            assert_eq!(block.allocator, (0, 0, 0), "block {index}: allocator calls");
            assert_eq!(block.audit, (0, 0), "block {index}: render audit");
        }
        assert_eq!(
            run.adopt_counts,
            (0, 0, 0),
            "the promotion allocates and frees nothing"
        );
    }
}

// Issue #1648b: a fader stage's stored automation event state carries across a swap (D2).
mod fader_automation_carry {
    use super::*;
    use session::{
        Automation, AutomationOffset, AutomationSegment, AutomationShape, ControlAddress, Owner,
        ParameterChannel, ParameterUnit, Section,
    };

    /// `model` with a linear fader ride on `track`'s `lane`, `0` dB at timeline 0 to `end_db` at
    /// 3,000: it moves across [`SWAP_BLOCK`].
    fn ride(model: &mut SessionModel, track: &str, lane: ParameterChannel, end_db: f32) {
        model.automation.push(Automation {
            id: id(&format!("{track}-ride")),
            target: ControlAddress {
                owner: Owner::Strip(id(track)),
                section: Section::Builtins,
                control: 5,
                lane,
            },
            segments: vec![AutomationSegment {
                shape: AutomationShape::Linear,
                start_sample: 0,
                end_sample: 3_000,
                start_value: 0.0,
                end_value: end_db,
                unit: ParameterUnit::Db,
            }],
            offset: AutomationOffset::default(),
        });
        model.automation.sort_by(|a, b| a.id.cmp(&b.id));
    }

    /// Gate 6. A live-controlled swap that carries a changed ride on `eq0`, a cell `eq1` gains
    /// on its left lane and a cell `eq1` loses on its right lane (the release record) makes no
    /// allocator call over the hand-over and the successor's first block, at every width. The
    /// same swap with the ride kept and a muted track added renders every block bit-identical to
    /// the uninterrupted predecessor.
    ///
    /// Test value: red if the carry of cell event state, the adoption jump or the release
    /// allocates or frees on the render thread, or if a carried cell with an unchanged table does
    /// not continue its events (the lane would restart or jump).
    #[test]
    fn carrying_fader_automation_allocates_and_frees_nothing() {
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
        let mut a_model = session_a();
        ride(&mut a_model, "eq0", ParameterChannel::Both, -12.0);
        ride(&mut a_model, "eq1", ParameterChannel::Right, -6.0);
        let mut changed = session_a();
        ride(&mut changed, "eq0", ParameterChannel::Both, -24.0);
        ride(&mut changed, "eq1", ParameterChannel::Left, -9.0);
        let mut kept = a_model.clone();
        add_track(&mut kept, "eq0", MUTED_TRACK, SOURCE, true);
        let a = Session::compile(a_model);
        let (changed, kept) = (Session::compile(changed), Session::compile(kept));
        let feed = Feed::new(SOURCE, 1);
        for backend in backends() {
            let run = direct_run(
                &a,
                Some((&changed, &a.model)),
                &feed,
                QUANTUM,
                &[],
                backend,
                false,
            );
            assert_eq!(run.carry, CarryOutcome::Carried, "{backend:?}");
            assert_eq!(
                run.swap_allocator,
                ((0, 0, 0), (0, 0)),
                "{backend:?}: the hand-over and the successor's first block"
            );
            let uninterrupted = direct_run(&a, None, &feed, QUANTUM, &[], backend, false);
            assert_ne!(
                run.blocks[SWAP_BLOCK..],
                uninterrupted.blocks[SWAP_BLOCK..],
                "{backend:?}: the changed rides are heard"
            );
            let run = direct_run(
                &a,
                Some((&kept, &a.model)),
                &feed,
                QUANTUM,
                &[],
                backend,
                false,
            );
            assert_eq!(run.carry, CarryOutcome::Carried, "{backend:?}");
            assert_eq!(
                first_direct_difference(&run, &uninterrupted),
                None,
                "{backend:?}: a kept ride continues"
            );
        }
    }
}
