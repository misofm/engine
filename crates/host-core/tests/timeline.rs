//! Issue #1641: every plan carries one timeline consumer, the session's playhead, which the
//! source driver advances at block entry before the sources, and which a successor carries as it
//! carries an unchanged source.
//!
//! * **Gate 1, source and timeline agree.** The same anchored and then plain seek, pushed to a
//!   playing source and to the timeline, puts the frame each block's source read starts at equal
//!   to the timeline at that block's source-read sample, for ten blocks after each.
//! * **Gate 2, carried across a rebuild.** A successor continues the timeline with no gap or
//!   repeat, answers the predecessor's values over its own window, sizes its history by its own
//!   latency, and takes the predecessor's producer.
//! * **Gate 3, carried at a discontinuity and a prime.** A declared discontinuity keeps sources
//!   and timeline in step under later seeks; a prime of two blocks advances the timeline as it
//!   advances a source consumer's frames, and both refuse the same windows (B16-R3).
//!
//! The session is the fixture reduced to one audible track `eq0` on [`SOURCE`], with no effect
//! and no filter, fed a ramp whose every sample of source frame `f` is `f`: the output is a fixed
//! gain of the frame, so each block's first output sample names the frame its source read
//! started at.

#![cfg(feature = "test-support")]

#[path = "support/successor.rs"]
mod successor;

use core::num::NonZeroUsize;

use bench_support::alloc as bench_alloc;
use engine::QuantumFrames;
use engine::realtime::{CarryOutcome, PlanarBufferMut, PreparedRenderPlan, RenderIo};
use graph_compiler::Backend;
use host_core::{
    PlanStateInventory, PreparedHost, SourceControlError, SourceControlSet, SourceSubmission,
    SuccessorBase, test_only_prepare_host_runtime_successor_on, test_only_set_source_read_offset,
    test_only_timeline_at,
};
use session::{
    Effect, EffectIdentity, EffectParam, EffectQuality, LinkMode, ParameterChannel, ParameterUnit,
    RouteSource, SessionModel, SidechainDeclaration, StableId, parse_session_json,
};
use source::{
    PcmSourceRing, PcmSourceRingConfig, SourceCommand, SourceFrame, SourceGeneration,
    SourceSeekReport, SourceTimeline, TimelineProducer,
};
use successor::{QUANTUM, SUCCESSOR_REVISION, Session, caps};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const SOURCE: &str = "fixture-source";
const ADDED_TRACK: &str = "added";
const Q: u64 = QUANTUM as u64;
/// The frame the anchored seek jumps to.
const ANCHORED_FRAME: u64 = 160 * Q;
/// The frame the plain seek jumps to.
const PLAIN_FRAME: u64 = 240 * Q;
/// Blocks checked after each seek.
const CHECKED: usize = 10;

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("stable ID")
}

/// Track `eq0` on [`SOURCE`], both console sections empty, no inserts, input filters off.
fn one_track() -> SessionModel {
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
    model
}

/// `model` plus a muted copy of `eq0` on [`SOURCE`], with its own route: the successor's edit.
fn with_added_track(mut model: SessionModel) -> SessionModel {
    let mut added = model.tracks[0].clone();
    added.id = id(ADDED_TRACK);
    added.fader.left_mute = true;
    added.fader.right_mute = true;
    model.tracks.push(added);
    let mut route = model.routes[0].clone();
    route.id = id(&format!("{ADDED_TRACK}-main"));
    let RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("a track route");
    };
    *track_id = id(ADDED_TRACK);
    model.routes.push(route);
    model
}

/// `model` with a true-peak limiter insert of `lookahead_ms` lookahead on `track`: a latent node.
fn with_limiter(mut model: SessionModel, track: &str, lookahead_ms: f32) -> SessionModel {
    let param = |parameter_id, unit, value| EffectParam {
        parameter_id,
        channel: ParameterChannel::Both,
        unit,
        value,
    };
    let strip = model
        .tracks
        .iter_mut()
        .find(|strip| strip.id.as_str() == track)
        .expect("the track");
    strip.inserts.effects.push(Effect {
        id: id(&format!("{track}-limiter")),
        identity: EffectIdentity::Native {
            effect_id: id("miso.true-peak-limiter"),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::Maximum,
        params: vec![
            param(1, ParameterUnit::Db, -8.0),
            param(2, ParameterUnit::Milliseconds, 60.0),
            param(3, ParameterUnit::Milliseconds, lookahead_ms),
        ],
        sidechain: SidechainDeclaration::None,
    });
    model
}

/// The timeline's `K` the plan's inventory implies (#1641 D1): `⌈A_max / q⌉ + 1`.
fn history_blocks(inventory: &PlanStateInventory) -> u64 {
    let largest = inventory
        .input_floors()
        .iter()
        .map(|(_, arrival)| *arrival)
        .max()
        .unwrap_or(0);
    largest.div_ceil(Q) + 1
}

/// Submit source frames `frames` of the ramp at `generation`, one quantum per chunk.
fn submit(sources: &mut SourceControlSet, generation: u64, frames: core::ops::Range<u64>) {
    for start in frames.step_by(QUANTUM) {
        let plane: Vec<f32> = (start..start + Q).map(|frame| frame as f32).collect();
        sources
            .submit(
                SOURCE.as_bytes(),
                SourceSubmission {
                    generation,
                    start_frame: start,
                    sample_rate_hz: 48_000,
                    planes: &[&plane, &plane],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .unwrap_or_else(|error| panic!("generation {generation} frame {start}: {error:?}"));
    }
}

/// One rendered block: its render sample and its right output channel (the fixture pans `eq0`
/// right, so its left output is near zero).
struct Block {
    sample: u64,
    right: Vec<f32>,
    allocator: (u64, u64, u64),
}

fn render(plan: &mut PreparedRenderPlan) -> Block {
    let mut output = [f32::NAN; QUANTUM * 2];
    let sample = plan.next_absolute_sample();
    let mark = bench_alloc::current_thread_counters();
    plan.render_contiguous(
        RenderIo {
            output: PlanarBufferMut::try_new(&mut output, 2, QUANTUM, QUANTUM).expect("output"),
        },
        sample,
    )
    .unwrap_or_else(|error| panic!("render: {error:?}"));
    let delta = bench_alloc::current_thread_delta_since(mark);
    Block {
        sample,
        right: output[QUANTUM..].to_vec(),
        allocator: (delta.allocations, delta.reallocations, delta.deallocations),
    }
}

/// The output gain of the ramp: block 1 of a fresh run starts at frame `Q`.
fn gain_of(block: &Block) -> f64 {
    let gain = f64::from(block.right[0]) / Q as f64;
    assert!(gain > 0.1, "the source plays: gain {gain}");
    gain
}

/// The frame a block's source read started at, from its first output sample; checks that its
/// last sample is the same read, `Q - 1` frames on.
fn frame_of(block: &Block, gain: f64) -> i64 {
    let decode = |sample: f32| (f64::from(sample) / gain).round() as i64;
    let first = decode(block.right[0]);
    assert_eq!(
        decode(block.right[QUANTUM - 1]),
        first + Q as i64 - 1,
        "block at {}: one contiguous read",
        block.sample
    );
    first
}

fn timeline(plan: &mut PreparedRenderPlan, source_read_sample: u64) -> i64 {
    test_only_timeline_at(plan, source_read_sample).expect("a graph plan with a timeline")
}

fn producer(sources: &mut SourceControlSet) -> &mut TimelineProducer {
    sources
        .test_only_timeline_producer()
        .expect("the set holds the timeline producer")
}

/// Seek the source and the timeline with one session seek (#1642 D1): `command`'s generation,
/// its frame as the timeline sample (inside the source's region, so the source takes it too) and
/// its anchor, if any.
fn seek_both(sources: &mut SourceControlSet, command: SourceCommand) {
    let (generation, frame, anchor) = match command {
        SourceCommand::Seek { generation, frame } => (generation, frame, None),
        SourceCommand::SeekAt {
            generation,
            frame,
            anchor_sample,
        } => (generation, frame, Some(anchor_sample)),
    };
    sources
        .seek_session(generation.0, frame.0, anchor)
        .expect("session seek");
}

/// The blocks a plain seek's checks submit: [`CHECKED`], and at least the ring's seek runway, so
/// the seek lands at the next block (#2205).
fn plain_runway() -> u64 {
    let runway = host_core::source_seek_runway_blocks(48_000, QUANTUM as u32, 4_096);
    (CHECKED as u64).max(u64::from(runway))
}

/// Render `blocks` blocks of `plan` (whose source-read clock leads its render clock by `offset`)
/// and check, after each, that the frame its source read started at is the timeline at its
/// source-read sample. Returns the frames.
fn render_in_step(
    plan: &mut PreparedRenderPlan,
    offset: u64,
    gain: f64,
    blocks: usize,
) -> Vec<i64> {
    (0..blocks)
        .map(|_| {
            let block = render(plan);
            let frame = frame_of(&block, gain);
            assert_eq!(
                timeline(plan, block.sample + offset),
                frame,
                "block at render sample {}: the timeline and the source's frame",
                block.sample
            );
            frame
        })
        .collect()
}

/// The timeline's seek report under the reserved ID both hosts pass through (#1704 D5).
fn timeline_report(sources: &SourceControlSet) -> Result<SourceSeekReport, SourceControlError> {
    sources.seek_report(b"$timeline")
}

/// The report the timeline publishes at the block a generation first plays in (#1704 D2).
fn timeline_landed(generation: u64, first_sample: u64, timeline: u64) -> SourceSeekReport {
    SourceSeekReport {
        generation,
        first_sample,
        source_frame: timeline,
        ..SourceSeekReport::default()
    }
}

/// Gate 1.
///
/// Test value: red if the timeline observes a seek one block before or after a source given the
/// same seek (an automation curve would sit one quantum off its audio), or ignores the anchor;
/// or (#1704) if the timeline's seek report is never published, is stamped on the wrong block or
/// value, disagrees with the source's report for the same seek, or the reserved ID reaches the
/// timeline through a source seek.
#[test]
fn a_source_and_the_timeline_given_the_same_seeks_read_the_same_frame_in_every_block() {
    let session = Session::compile(one_track());
    let mut host = session.prepare(Backend::current());
    const SEEK_BLOCK: u64 = 2;
    const ANCHOR_BLOCK: u64 = 6;
    // Generation 1 up to the anchor, queued before the seek makes it stale.
    submit(&mut host.sources, 1, 0..ANCHOR_BLOCK * Q);
    render(&mut host.plan);
    let gain = gain_of(&render(&mut host.plan));
    assert_eq!(host.plan.next_absolute_sample(), SEEK_BLOCK * Q);

    let anchor = ANCHOR_BLOCK * Q;
    seek_both(
        &mut host.sources,
        SourceCommand::SeekAt {
            generation: SourceGeneration(2),
            frame: SourceFrame(ANCHORED_FRAME),
            anchor_sample: anchor,
        },
    );
    submit(
        &mut host.sources,
        2,
        ANCHORED_FRAME..ANCHORED_FRAME + CHECKED as u64 * Q,
    );
    let frames = render_in_step(&mut host.plan, 0, gain, CHECKED);
    // Not vacuous: the source plays on to the anchor block and jumps there.
    let jump = (ANCHOR_BLOCK - SEEK_BLOCK) as usize;
    assert_eq!(frames[jump - 1], ((ANCHOR_BLOCK - 1) * Q) as i64);
    assert_eq!(frames[jump], ANCHORED_FRAME as i64);
    let landed = timeline_report(&host.sources).expect("timeline report");
    assert_eq!(landed, timeline_landed(2, anchor, ANCHORED_FRAME));
    let source = host
        .sources
        .seek_report(SOURCE.as_bytes())
        .expect("source report");
    assert_eq!(
        (source.generation, source.first_sample, source.source_frame),
        (landed.generation, landed.first_sample, landed.source_frame),
        "the source and the timeline report the same landing"
    );

    let plain_sample = host.plan.next_absolute_sample();
    seek_both(
        &mut host.sources,
        SourceCommand::Seek {
            generation: SourceGeneration(3),
            frame: SourceFrame(PLAIN_FRAME),
        },
    );
    submit(
        &mut host.sources,
        3,
        PLAIN_FRAME..PLAIN_FRAME + plain_runway() * Q,
    );
    let frames = render_in_step(&mut host.plan, 0, gain, CHECKED);
    assert_eq!(
        frames[0], PLAIN_FRAME as i64,
        "a plain seek with its runway queued lands in the next block"
    );
    assert_eq!(
        timeline_report(&host.sources),
        Ok(timeline_landed(3, plain_sample, PLAIN_FRAME))
    );
    // The timeline is seeked by the session seek alone.
    assert_eq!(
        host.sources.seek(b"$timeline", 4, 0),
        Err(SourceControlError::UnknownSource)
    );
}

/// `b` prepared as the successor of `a`'s plan `inventory` at `discontinuity`.
fn successor_of(
    b: &Session,
    a: &Session,
    inventory: &PlanStateInventory,
    discontinuity: bool,
) -> PreparedHost {
    test_only_prepare_host_runtime_successor_on(
        &b.compiled,
        &caps(),
        SuccessorBase {
            discontinuity,
            ..SuccessorBase::new(inventory, &a.model, SUCCESSOR_REVISION)
        },
        Backend::current(),
    )
    .unwrap_or_else(|failure| panic!("successor: {failure:?}"))
}

/// Hand `predecessor`'s producers and plan state to `successor` as the C ABI does at commit and
/// at the swap block. Returns the swap block's carry outcome.
fn hand_over(successor: &mut PreparedHost, predecessor: &mut PreparedHost) -> CarryOutcome {
    successor.sources.adopt_persisting(&mut predecessor.sources);
    successor
        .plan
        .take_revision_gate()
        .expect("the successor's gate handle")
        .publish(SUCCESSOR_REVISION);
    successor.plan.adopt_predecessor_plan(&mut predecessor.plan)
}

/// Render `blocks` blocks of `host`, feeding generation 1's ramp one block ahead of the source's
/// frame `*fed`.
fn render_fed(host: &mut PreparedHost, fed: &mut u64, blocks: usize) {
    for _ in 0..blocks {
        submit(&mut host.sources, 1, *fed..*fed + Q);
        *fed += Q;
        render(&mut host.plan);
    }
}

/// Gate 2, the edit the spec names: one added track, no latency on either side.
///
/// Test value: red if a successor restarts the timeline at 0, drops its history or its producer,
/// or repeats or skips a block at the swap.
#[test]
fn a_rebuild_carries_the_timeline_its_history_and_its_producer() {
    let a = Session::compile(one_track());
    let b = Session::compile(with_added_track(one_track()));
    let mut predecessor = a.prepare(Backend::current());
    let mut fed = 0;
    submit(&mut predecessor.sources, 1, 0..Q);
    fed += Q;
    render_fed(&mut predecessor, &mut fed, 20);
    const SOUGHT: u64 = 1_000 * Q;
    producer(&mut predecessor.sources)
        .try_seek(SourceCommand::Seek {
            generation: SourceGeneration(2),
            frame: SourceFrame(SOUGHT),
        })
        .expect("timeline seek");
    render_fed(&mut predecessor, &mut fed, 5);
    let now = predecessor.plan.next_absolute_sample();
    assert_eq!(now, 25 * Q);
    // The predecessor's value at every sample of the window a successor of `K` blocks keeps.
    let mut successor = successor_of(&b, &a, &predecessor.inventory, false);
    let k = history_blocks(&successor.inventory);
    assert_eq!(k, 1, "no latency: one block of history");
    let window: Vec<(u64, i64)> = (now - k * Q..now)
        .map(|sample| (sample, timeline(&mut predecessor.plan, sample)))
        .collect();
    assert_eq!(
        window[0].1,
        (SOUGHT + 4 * Q) as i64,
        "the predecessor played the seek"
    );
    let last = timeline_report(&predecessor.sources).expect("predecessor report");
    assert_eq!(last, timeline_landed(2, 20 * Q, SOUGHT));

    assert_eq!(
        hand_over(&mut successor, &mut predecessor),
        CarryOutcome::Carried
    );
    for &(sample, value) in &window {
        assert_eq!(
            timeline(&mut successor.plan, sample),
            value,
            "sample {sample}"
        );
    }
    // No gap or repeat: ten blocks continue a quantum apart.
    for block in 0..10_u64 {
        submit(&mut successor.sources, 1, fed..fed + Q);
        fed += Q;
        let sample = render(&mut successor.plan).sample;
        assert_eq!(
            timeline(&mut successor.plan, sample),
            (SOUGHT + (5 + block) * Q) as i64,
            "successor block {block}"
        );
    }
    // #1704 D3: the record moved with the timeline; its writer and its published copy too, so
    // the successor's blocks of the same generation leave the predecessor's words.
    assert_eq!(timeline_report(&successor.sources), Ok(last));
    assert_eq!(
        timeline_report(&predecessor.sources),
        Err(SourceControlError::Vacated)
    );
    // The producer moved: the predecessor's set holds none, and a seek through the successor's
    // set (its generation strictly above the predecessor's last) reaches the carried consumer.
    assert!(predecessor.sources.test_only_timeline_producer().is_none());
    let moved = producer(&mut successor.sources);
    assert_eq!(moved.active_generation(), SourceGeneration(2));
    moved
        .try_seek(SourceCommand::Seek {
            generation: SourceGeneration(3),
            frame: SourceFrame(7 * Q),
        })
        .expect("a seek through the successor's set");
    let sample = render(&mut successor.plan).sample;
    assert_eq!(timeline(&mut successor.plan, sample), (7 * Q) as i64);
    assert_eq!(
        timeline_report(&successor.sources),
        Ok(timeline_landed(3, sample, 7 * Q))
    );
}

/// The command queue and seek-report record a successor's carried timeline brings (#1641 D7,
/// #1704 D4): a successor that carries no ring carries exactly these source bytes.
fn timeline_queue_bytes() -> u64 {
    let report = SourceTimeline::resource_report(QuantumFrames(QUANTUM as u32), NonZeroUsize::MIN)
        .expect("timeline report");
    report.command_queue_bytes + report.seek_report_bytes
}

/// Gate 2 with nothing else carried (batch 16a fold M2): the successor changes the source's
/// declaration, so it prepares a ring of its own, and renames the track, so no strip state
/// carries either: the timeline is the carry program's only move.
///
/// Test value: red if preparation builds a carry program only when a source or strip carries
/// (the D3 `timeline_carries` condition dropped: the carry is not requested), or the graph calls
/// the driver's adopt step only for a source move (`program.timeline` dropped: the successor's
/// timeline restarts at 0 and a seek through the moved producer is lost).
#[test]
fn a_rebuild_that_carries_no_source_still_carries_the_timeline() {
    let a = Session::compile(one_track());
    let mut b_model = one_track();
    b_model.sources[0].frames *= 2;
    b_model.tracks[0].id = id(ADDED_TRACK);
    let RouteSource::Track { track_id, .. } = &mut b_model.routes[0].source else {
        panic!("a track route");
    };
    *track_id = id(ADDED_TRACK);
    let b = Session::compile(b_model);
    let mut predecessor = a.prepare(Backend::current());
    let mut fed = 0;
    submit(&mut predecessor.sources, 1, 0..Q);
    fed += Q;
    render_fed(&mut predecessor, &mut fed, 6);
    const SOUGHT: u64 = 300 * Q;
    producer(&mut predecessor.sources)
        .try_seek(SourceCommand::Seek {
            generation: SourceGeneration(2),
            frame: SourceFrame(SOUGHT),
        })
        .expect("timeline seek");
    render_fed(&mut predecessor, &mut fed, 2);
    let now = predecessor.plan.next_absolute_sample();
    assert_eq!(
        timeline(&mut predecessor.plan, now - 1),
        (SOUGHT + 2 * Q - 1) as i64
    );
    let mut successor = successor_of(&b, &a, &predecessor.inventory, false);
    assert_eq!(
        successor.report.carried_source_total_bytes,
        timeline_queue_bytes(),
        "not vacuous: the successor carries no ring, only the timeline's queue"
    );
    assert_eq!(
        hand_over(&mut successor, &mut predecessor),
        CarryOutcome::Carried
    );
    // No gap or repeat: the successor's blocks continue the predecessor's timeline.
    for block in 0..3_u64 {
        let sample = render(&mut successor.plan).sample;
        assert_eq!(
            timeline(&mut successor.plan, sample),
            (SOUGHT + (2 + block) * Q) as i64,
            "successor block {block}"
        );
    }
    // A seek through the moved producer lands in the next block.
    producer(&mut successor.sources)
        .try_seek(SourceCommand::Seek {
            generation: SourceGeneration(3),
            frame: SourceFrame(7 * Q),
        })
        .expect("a seek through the successor's set");
    let sample = render(&mut successor.plan).sample;
    assert_eq!(timeline(&mut successor.plan, sample), (7 * Q) as i64);
}

/// Gate 2, a seek accepted but not yet popped at the swap block (batch 16a fold MINOR 3): the
/// predecessor renders no block after the push, so the command is still in the queue that the
/// carry moves.
///
/// Test value: red if the carry moves the timeline without its command queue, or the successor
/// drains it without applying it (an acked seek would be dropped).
#[test]
fn a_seek_queued_at_the_swap_block_lands_in_the_successors_first_block() {
    let a = Session::compile(one_track());
    let b = Session::compile(with_added_track(one_track()));
    let mut predecessor = a.prepare(Backend::current());
    let mut fed = 0;
    submit(&mut predecessor.sources, 1, 0..Q);
    fed += Q;
    render_fed(&mut predecessor, &mut fed, 4);
    let mut successor = successor_of(&b, &a, &predecessor.inventory, false);
    producer(&mut predecessor.sources)
        .try_seek(SourceCommand::Seek {
            generation: SourceGeneration(2),
            frame: SourceFrame(40 * Q),
        })
        .expect("timeline seek");
    assert_eq!(
        hand_over(&mut successor, &mut predecessor),
        CarryOutcome::Carried
    );
    let sample = render(&mut successor.plan).sample;
    assert_eq!(timeline(&mut successor.plan, sample), (40 * Q) as i64);
    assert_eq!(
        producer(&mut successor.sources).active_generation(),
        SourceGeneration(2)
    );
}

/// The D3 guard (batch 16a fold MINOR 3): a successor prepared by hand at another rate or quantum
/// gets a fresh timeline with a producer of its own, not a vacant one whose carry the swap block
/// would refuse.
///
/// Test value: red if the guard is dropped (every successor's timeline vacant): this successor
/// would hold no timeline producer until a carry that cannot happen.
#[test]
fn a_successor_at_another_rate_or_quantum_starts_a_fresh_timeline() {
    let a = Session::compile(one_track());
    let predecessor = a.prepare(Backend::current());
    for (rate, quantum) in [(44_100, QUANTUM as u32), (48_000, 2 * QUANTUM as u32)] {
        let mut model = one_track();
        model.sample_rate_hz = rate;
        model.quantum_frames = quantum;
        let b = Session::compile(model);
        let mut successor = successor_of(&b, &a, &predecessor.inventory, false);
        let fresh = producer(&mut successor.sources);
        assert_eq!(
            fresh.active_generation(),
            SourceGeneration(1),
            "{rate} Hz, {quantum} frames"
        );
    }
}

/// Gate 2, a successor with a larger `A_max`: its history is its own, of its own `K`.
///
/// Test value: red if the successor sizes its history from the predecessor's `K` (a seek its
/// latent node can still be reading falls out of the window), or does not keep the predecessor's
/// newest entries.
#[test]
fn a_latent_successor_keeps_the_newest_entries_in_a_history_of_its_own_size() {
    // `eq0` behind a limiter in both plans; the successor's added track has a longer one.
    let a_model = with_limiter(one_track(), "eq0", 4.0);
    let a = Session::compile(a_model.clone());
    let b = Session::compile(with_limiter(with_added_track(a_model), ADDED_TRACK, 10.0));
    let mut predecessor = a.prepare(Backend::current());
    let k_a = history_blocks(&predecessor.inventory);
    let mut fed = 0;
    submit(&mut predecessor.sources, 1, 0..Q);
    fed += Q;
    render_fed(&mut predecessor, &mut fed, 8);
    producer(&mut predecessor.sources)
        .try_seek(SourceCommand::Seek {
            generation: SourceGeneration(2),
            frame: SourceFrame(500 * Q),
        })
        .expect("timeline seek");
    render_fed(&mut predecessor, &mut fed, 2);
    let mut successor = successor_of(&b, &a, &predecessor.inventory, false);
    let k_b = history_blocks(&successor.inventory);
    assert!(
        k_a >= 2 && k_b >= k_a + 2,
        "the successor reads further back: K {k_a} -> {k_b}"
    );
    let now = predecessor.plan.next_absolute_sample();
    // The successor's whole window: the predecessor's `K` newest entries, the rest extrapolated
    // back from the oldest of them, exactly as the predecessor answers.
    let window: Vec<(u64, i64)> = (now - k_b * Q..now)
        .map(|sample| (sample, timeline(&mut predecessor.plan, sample)))
        .collect();
    assert_eq!(
        hand_over(&mut successor, &mut predecessor),
        CarryOutcome::Carried
    );
    for &(sample, value) in &window {
        assert_eq!(
            timeline(&mut successor.plan, sample),
            value,
            "sample {sample}"
        );
    }

    // A seek after the swap, then `K_b - 2` blocks: the block before the seek is still in the
    // successor's window, so its value is exact, not extrapolated back from the seek.
    let before = successor.plan.next_absolute_sample();
    render_fed(&mut successor, &mut fed, 1);
    let unsought = timeline(&mut successor.plan, before);
    producer(&mut successor.sources)
        .try_seek(SourceCommand::Seek {
            generation: SourceGeneration(3),
            frame: SourceFrame(9 * Q),
        })
        .expect("timeline seek");
    render_fed(&mut successor, &mut fed, (k_b - 1) as usize);
    assert_eq!(timeline(&mut successor.plan, before), unsought);
    assert_eq!(timeline(&mut successor.plan, before + Q), (9 * Q) as i64);
    assert_ne!(
        unsought,
        (9 * Q) as i64 - Q as i64,
        "the exact value differs from the extrapolation, so the check discriminates"
    );
}

/// Gate 3, a declared discontinuity.
///
/// Test value: red if a discontinuity successor moves the sources and not the timeline (it
/// restarts it, or steps it back with the source-read clock). The plan has no latency, so its
/// history holds one entry and no overlap: the reader's overlap rule is the `source` unit tests'
/// (`a_node_block_reports_the_step_of_a_seek_inside_it_or_at_its_first_sample`).
#[test]
fn a_discontinuity_keeps_the_sources_and_the_timeline_in_step() {
    const OFFSET: u64 = 3 * Q;
    let a = Session::compile(one_track());
    let mut predecessor = a.prepare(Backend::current());
    test_only_set_source_read_offset(&mut predecessor, OFFSET);
    let mut fed = 0;
    submit(&mut predecessor.sources, 1, 0..2 * Q);
    fed += 2 * Q;
    render(&mut predecessor.plan);
    let gain = gain_of(&render(&mut predecessor.plan));
    for _ in 0..4 {
        submit(&mut predecessor.sources, 1, fed..fed + Q);
        fed += Q;
        render_in_step(&mut predecessor.plan, OFFSET, gain, 1);
    }
    let mut successor = successor_of(&a, &a, &predecessor.inventory, true);
    assert_eq!(successor.plan.source_read_offset(), 0);
    assert_eq!(
        hand_over(&mut successor, &mut predecessor),
        CarryOutcome::Carried
    );
    // The source-read clock steps back by `OFFSET`; the timeline and the source continue.
    for _ in 0..2 {
        submit(&mut successor.sources, 1, fed..fed + Q);
        fed += Q;
        let frames = render_in_step(&mut successor.plan, 0, gain, 1);
        assert_eq!(frames[0], (fed - Q) as i64, "no frame lost or repeated");
    }

    let anchor = successor.plan.next_absolute_sample() + 3 * Q;
    // Generation 1 up to the anchor, queued before the seek makes it stale.
    submit(&mut successor.sources, 1, fed..fed + 3 * Q);
    seek_both(
        &mut successor.sources,
        SourceCommand::SeekAt {
            generation: SourceGeneration(2),
            frame: SourceFrame(ANCHORED_FRAME),
            anchor_sample: anchor,
        },
    );
    submit(
        &mut successor.sources,
        2,
        ANCHORED_FRAME..ANCHORED_FRAME + CHECKED as u64 * Q,
    );
    let frames = render_in_step(&mut successor.plan, 0, gain, CHECKED);
    assert_eq!(frames[3], ANCHORED_FRAME as i64, "the anchored seek lands");
    seek_both(
        &mut successor.sources,
        SourceCommand::Seek {
            generation: SourceGeneration(3),
            frame: SourceFrame(PLAIN_FRAME),
        },
    );
    submit(
        &mut successor.sources,
        3,
        PLAIN_FRAME..PLAIN_FRAME + plain_runway() * Q,
    );
    let frames = render_in_step(&mut successor.plan, 0, gain, CHECKED);
    assert_eq!(frames[0], PLAIN_FRAME as i64, "the plain seek lands");
}

/// Gate 3, a prime (#1641 D5, B16-R3), on the two consumers directly: no prime path above the
/// consumer exists until #1355.
///
/// Test value: red if a prime moves a source and not the timeline, or the timeline primes over a
/// queued command or a held anchored seek inside the window (it would skip that seek).
#[test]
fn a_prime_advances_the_timeline_as_it_advances_a_source() {
    let quantum = QuantumFrames(QUANTUM as u32);
    // One session: the ring and the timeline share its published generation (#1642 B16-R2).
    let word = source::PublishedGeneration::new(SourceGeneration(1));
    let (producer, mut source, _) = PcmSourceRing::prepare_host_region(
        PcmSourceRingConfig {
            channel_count: 1,
            quantum_frames: quantum,
            frame_capacity: 8 * Q,
            initial_generation: SourceGeneration(1),
            seek_runway_blocks: 0,
        },
        SourceFrame(0),
        word.clone(),
    )
    .expect("ring");
    let mut host = producer.into_host_chunk_provider(engine::SampleRateHz(48_000));
    for start in (0..6 * Q).step_by(QUANTUM) {
        let plane: Vec<f32> = (start..start + Q).map(|frame| frame as f32).collect();
        host.submit(source::HostPlanarChunk {
            sample_rate_hz: engine::SampleRateHz(48_000),
            generation: SourceGeneration(1),
            start_frame: SourceFrame(start),
            planes: &[&plane],
            frames: QUANTUM as u32,
            end_of_region: false,
        })
        .expect("source PCM");
    }
    let (mut timeline_producer, mut timeline, _) =
        SourceTimeline::prepare(quantum, NonZeroUsize::new(4).expect("four"), word)
            .expect("timeline");
    // Two ordinary blocks.
    for sample in [0, Q] {
        source.begin_block_at(sample);
        timeline.begin_block_at(sample);
    }
    let first = 2 * Q;
    let (k, lead) = (2_u64, 1_u32);
    assert!(source.prime_ready(first, lead) && timeline.prime_ready(first, lead));
    let before = (
        source.telemetry().cumulative_read_frames,
        timeline.position(),
    );
    for j in 0..k {
        let sample = first + j * Q;
        source.prime_block_at(sample);
        timeline.prime_block_at(sample);
        let played = source.played_plane(0).expect("a played block")[0];
        assert_eq!(
            timeline.timeline_at(sample),
            played as i64,
            "prime block {j}"
        );
    }
    assert_eq!(
        source.telemetry().cumulative_read_frames - before.0,
        k * Q,
        "the source primes 2q frames"
    );
    assert_eq!(
        timeline.position() - before.1,
        k * Q,
        "the timeline primes 2q"
    );

    // B16-R3: a held anchored seek inside the window refuses both; one past the window does not.
    let next = first + k * Q;
    let held = |generation, anchor_sample| SourceCommand::SeekAt {
        generation: SourceGeneration(generation),
        frame: SourceFrame(0),
        anchor_sample,
    };
    // Queued and not yet observed: a queued command refuses both.
    host.try_seek(held(2, next + Q)).expect("source seek");
    timeline_producer
        .try_seek(held(2, next + Q))
        .expect("timeline seek");
    assert!(!source.prime_ready(next, lead) && !timeline.prime_ready(next, lead));
    // Observed and held, anchored at the window's last block: still refused, by both.
    source.begin_block_at(next);
    timeline.begin_block_at(next);
    assert!(!source.prime_ready(next + Q, 0) && !timeline.prime_ready(next + Q, 0));
    // Anchored past the window: ready.
    timeline_producer
        .try_seek(held(3, next + 4 * Q))
        .expect("timeline seek");
    timeline.begin_block_at(next + Q);
    assert!(timeline.prime_ready(next + 2 * Q, lead));
    assert!(!timeline.prime_ready(next + 2 * Q, lead + 1));
}

/// Gate 4 on the direct path: a carry that copies a history of its own size and moves a held
/// anchored seek, and the ten blocks after it, allocate and free nothing.
///
/// Test value: red if the carry or the history write allocates or frees on the render thread.
#[test]
fn the_timeline_carry_and_the_blocks_after_it_allocate_and_free_nothing() {
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
    let a_model = with_limiter(one_track(), "eq0", 4.0);
    let a = Session::compile(a_model.clone());
    let b = Session::compile(with_limiter(with_added_track(a_model), ADDED_TRACK, 10.0));
    let mut predecessor = a.prepare(Backend::current());
    let mut fed = 0;
    submit(&mut predecessor.sources, 1, 0..Q);
    fed += Q;
    render_fed(&mut predecessor, &mut fed, 4);
    let anchor = predecessor.plan.next_absolute_sample() + 3 * Q;
    producer(&mut predecessor.sources)
        .try_seek(SourceCommand::SeekAt {
            generation: SourceGeneration(2),
            frame: SourceFrame(77 * Q),
            anchor_sample: anchor,
        })
        .expect("timeline seek");
    render_fed(&mut predecessor, &mut fed, 1);
    let mut successor = successor_of(&b, &a, &predecessor.inventory, false);
    successor.sources.adopt_persisting(&mut predecessor.sources);
    successor
        .plan
        .take_revision_gate()
        .expect("the successor's gate handle")
        .publish(SUCCESSOR_REVISION);
    submit(&mut successor.sources, 1, fed..fed + 11 * Q);
    let mark = bench_alloc::current_thread_counters();
    let carry = successor.plan.adopt_predecessor_plan(&mut predecessor.plan);
    let delta = bench_alloc::current_thread_delta_since(mark);
    assert_eq!(carry, CarryOutcome::Carried);
    assert_eq!(
        (delta.allocations, delta.reallocations, delta.deallocations),
        (0, 0, 0),
        "the carry"
    );
    for block in 0..11 {
        let rendered = render(&mut successor.plan);
        assert_eq!(rendered.allocator, (0, 0, 0), "successor block {block}");
    }
    // The held seek moved with its consumer (D9) and applied at its anchor.
    assert_eq!(timeline(&mut successor.plan, anchor), (77 * Q) as i64);
}
