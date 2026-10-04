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

#[path = "support/successor.rs"]
mod successor;

use engine::realtime::{CarryOutcome, SwapOutcome};
use graph_compiler::Backend;
use host_core::{PrepareRejection, SourceControlError, SourceSubmission};
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
        report.carry_program_retained_bytes, 8,
        "one (u32, u32) move"
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
        assert_eq!(prepared.report.carry_program_retained_bytes, 0);
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
