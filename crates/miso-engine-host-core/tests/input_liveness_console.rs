//! Issue #210 phase 3, end to end: a real prepared session, a real drain, a real render.
//!
//! The bank arithmetic and the record's witness contract are gated in their own crates. What only
//! a whole prepared plan can show is the three facts below, and each of them is a property a
//! crate-local test would have to assume.
//!
//! 1. **Class-A OFF, on rendered bits.** A plan that leased a console and sent no trim or polarity
//!    command renders byte-identically to the same session prepared with **no console at all** --
//!    which is the plan the engine built before this phase existed. That is the render-identity
//!    half of the C3 form, in tree, on a real fixture.
//! 2. **The drain reaches the addressed member and only it.** A command pushed onto track `t`'s
//!    input queue moves track `t`'s coefficient, at the block that drains it, and moves no other
//!    track's. On a banked plan the queues are drained by the *bank* over its member lanes, so a
//!    lane-index slip is a command landing on the wrong track -- silent, and inaudible on any
//!    single-track fixture.
//! 3. **The collapse census follows the commands.** An asymmetric ride declines the ridden track
//!    and nothing else; a symmetric ride declines nothing; and putting the ridden track back does
//!    not restore it.
//!
//! The fixture is `parametric-eq-bank-console`, the same eight-track banked console
//! `symmetry_witness.rs` uses, with its right source channel remapped so the structural `SOURCE`
//! term holds and the collapse census is not vacuously zero.

use core::num::{NonZeroU32, NonZeroUsize};
use std::collections::BTreeMap;

use miso_engine_builtins::{BuiltinLaneSelector, MeterTap};
use miso_engine_builtins_compiler::TrackInputRecordV1;
use miso_engine_core::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use miso_engine_host_core::{
    HostConsoleHandlesV1, HostConsoleRequestV1, HostPrepareCaps, HostShapePolicy, PreparedHost,
    SourceSubmission, prepare_host_session, prepare_host_session_with_console,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-bank-console.toml");
const QUANTUM: usize = 128;
const TRACKS: usize = 8;

/// The fixture with both dual-mono lanes reading source channel 0: a mono source mapping, so the
/// structural `SOURCE` term holds and the collapse census is about the live terms.
fn mono_session() -> String {
    let mutated = SESSION.replace("right_source_channel = 1", "right_source_channel = 0");
    assert_ne!(mutated, SESSION, "the fixture's stereo mapping moved");
    mutated
}

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 1_024,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 100,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 100_000_000,
        maximum_source_total_bytes: 10_000_000,
        maximum_source_overhead_bytes: 10_000_000,
        maximum_effect_state_bytes: 100_000_000,
        maximum_effect_scratch_bytes: 100_000_000,
        maximum_builtin_retained_bytes: 100_000_000,
        maximum_named_allocation_bytes: 100_000_000,
        maximum_meter_streams: 64,
        maximum_meter_items: 1 << 16,
        maximum_meter_bytes: 1 << 24,
    }
}

fn console() -> HostConsoleRequestV1 {
    HostConsoleRequestV1 {
        control_queue_depth: Some(NonZeroUsize::new(8).expect("depth")),
        meter_period_frames: Some(NonZeroU32::new(QUANTUM as u32).expect("period")),
        meter_queue_depth: NonZeroUsize::new(16).expect("meter depth"),
        meter_tap: MeterTap::PostMatrix,
        observation_taps: 0,
        master_track: None,
    }
}

struct Host {
    prepared: PreparedHost,
    handles: Option<HostConsoleHandlesV1>,
    block: usize,
    peaks: BTreeMap<String, Vec<[u32; 2]>>,
}

fn prepare_with_console(toml: &str) -> Host {
    let (_, prepared, handles) = prepare_host_session_with_console(toml, &caps(), &console())
        .unwrap_or_else(|failure| {
            panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
        });
    assert_eq!(handles.tracks.len(), TRACKS);
    assert_eq!(handles.track_controls.len(), TRACKS);
    Host {
        prepared,
        handles: Some(handles),
        block: 0,
        peaks: BTreeMap::new(),
    }
}

fn prepare_without_console(toml: &str) -> Host {
    let (_, prepared) = prepare_host_session(toml, &caps()).unwrap_or_else(|failure| {
        panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
    });
    Host {
        prepared,
        handles: None,
        block: 0,
        peaks: BTreeMap::new(),
    }
}

/// One block of a signal with a `-0.0` and a sign flip in it, rendered; returns the output bits.
fn render(host: &mut Host, blocks: usize) -> Vec<u32> {
    let mut digest = Vec::with_capacity(blocks * QUANTUM * 2);
    for step in 0..blocks {
        let block = host.block + step;
        let plane: Vec<f32> = (0..QUANTUM)
            .map(|frame| match (frame + block) % 5 {
                0 => 0.75,
                1 => -0.75,
                2 => -0.0,
                3 => 0.125,
                _ => -0.5,
            })
            .collect();
        host.prepared
            .sources
            .submit(
                b"fixture-source",
                SourceSubmission {
                    generation: 1,
                    start_frame: (block * QUANTUM) as u64,
                    sample_rate_hz: 48_000,
                    planes: &[&plane, &plane],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
        let mut samples = [0.0_f32; QUANTUM * 2];
        let output =
            PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM).expect("output planes");
        host.prepared
            .plan
            .render(
                RenderIo {
                    input: None,
                    output,
                },
                RenderTime {
                    absolute_sample: (block * QUANTUM) as u64,
                },
            )
            .expect("render");
        digest.extend(samples.iter().map(|value| value.to_bits()));
        if let Some(handles) = host.handles.as_mut() {
            for meter in handles.meters.iter_mut() {
                while let Ok(snapshot) = meter.consumer.try_pop() {
                    host.peaks
                        .entry(meter.track_id.to_string())
                        .or_default()
                        .push([
                            snapshot.left.sample_peak.to_bits(),
                            snapshot.right.sample_peak.to_bits(),
                        ]);
                }
            }
        }
    }
    host.block += blocks;
    digest
}

/// Per-track post-matrix peaks, which is how a single track's contribution is observed in a mix
/// eight identical tracks sum into.
///
/// The mix alone cannot tell tracks apart -- this fixture's eight tracks are identical, so
/// silencing any one of them produces the same sum, and a drain that landed a command on the wrong
/// lane would be invisible in the output. The per-track meter tap is what makes the lane index
/// observable at all, and it is why the tests below read it rather than the mix.
fn peaks(host: &Host) -> &BTreeMap<String, Vec<[u32; 2]>> {
    &host.peaks
}

fn push(host: &mut Host, track: usize, record: TrackInputRecordV1) {
    host.handles
        .as_mut()
        .expect("a console")
        .track_controls
        .get_mut(track)
        .expect("a control channel for the addressed track")
        .input
        .try_push(record)
        .expect("bounded queue room");
}

fn census(host: &Host) -> [u64; 2] {
    host.prepared.plan.symmetry_counters()
}

// ---------------------------------------------------------------------------------------------
// 1. Class-A OFF, on rendered bits.
// ---------------------------------------------------------------------------------------------

/// A console-attached plan that sends no input command renders the console-free plan's bytes.
///
/// The console-free arm is the plan the engine built before this phase existed: it binds
/// `InputProcessor` (or a bank with no consumers), it has no ramp in flight, and its input stage
/// is prepared from the same coefficients. So this is the render-identity half of the class-A OFF
/// gate, measured rather than argued.
///
/// Red mutation: initialise `InputStage::ramp.current` from anything but `coef.trim` -> every
/// block of the console arm differs. Red mutation: make `InputStage::process` take the ramping arm
/// unconditionally -> this stays green (the two arms are bit-identical, which is the point) and
/// `miso-engine-builtins`' `the_settled_arm_leaves_the_ramp_words_untouched` is what refuses it.
#[test]
fn an_uncommanded_console_renders_the_console_free_bytes() {
    for session in [SESSION.to_owned(), mono_session()] {
        let mut with_console = prepare_with_console(&session);
        let mut without = prepare_without_console(&session);
        assert_eq!(
            render(&mut with_console, 12),
            render(&mut without, 12),
            "leasing a console and sending no input command changes no rendered byte"
        );
    }
}

// ---------------------------------------------------------------------------------------------
// 2. The drain reaches the addressed member and only it.
// ---------------------------------------------------------------------------------------------

/// A command addressed to one track moves that track and no other.
///
/// The failure this refuses is the banked drain's characteristic one: `BuiltinBankProcessor`
/// drains its members' queues in a loop and passes the loop index as the bank lane, so a slip
/// between the two -- draining lane `l`'s queue into lane `l + 1`, or skipping a `None` control
/// without advancing the index -- lands a command on the wrong track.
///
/// The **mix cannot see that**: this fixture's eight tracks are identical and sum into one output,
/// so silencing any one of them produces the same sum. The per-track post-matrix meter is what
/// makes the lane index observable, and that is why this test reads meters rather than samples.
///
/// Red mutation: drain `controls.iter_mut().flatten().enumerate()` instead of
/// `iter_mut().enumerate()` in `BuiltinBankProcessor::begin_block` -> on a bank with any
/// unaddressed member the lane indices shift and the wrong track is silenced.
///
/// Red mutation: pass a constant `0` as the bank lane in the drain -> every command lands on the
/// bank's first member, and every track but that one fails its "unchanged" assertion.
#[test]
fn a_command_moves_exactly_the_addressed_track() {
    let session = mono_session();
    let mut baseline = prepare_with_console(&session);
    let _ = render(&mut baseline, 4);
    let quiet = peaks(&baseline).clone();
    assert_eq!(quiet.len(), TRACKS, "one meter per track");
    assert!(
        quiet
            .values()
            .all(|windows| windows.iter().any(|peak| peak[0] != 0)),
        "every track must be audible in the baseline, or silencing one proves nothing"
    );

    for (index, addressed) in quiet.keys().cloned().enumerate() {
        let mut host = prepare_with_console(&session);
        push(
            &mut host,
            index,
            TrackInputRecordV1::TrimDb {
                lanes: BuiltinLaneSelector::Both,
                db: -144.0,
                smoothing_samples: 0,
            },
        );
        let _ = render(&mut host, 4);
        for (track, windows) in peaks(&host) {
            if *track == addressed {
                assert_ne!(
                    windows, &quiet[track],
                    "track {track} was addressed by index {index} and must have moved"
                );
                assert!(
                    windows.iter().all(|peak| peak[0] < quiet[track][0][0]),
                    "a -144 dB trim drives the addressed track's peak down"
                );
            } else {
                assert_eq!(
                    windows, &quiet[track],
                    "track {track} was not addressed and must be bit-identical: a command \
                     addressed to index {index} reached the wrong bank lane"
                );
            }
        }
    }
}

/// Two tracks commanded to different values do not swap.
///
/// The sharpest form of the lane-index test: a transposing drain renders the same *mix* either
/// way, so the two arms are compared per track.
#[test]
fn two_commands_are_not_interchangeable() {
    let session = mono_session();
    let build = |first: usize, second: usize| {
        let mut host = prepare_with_console(&session);
        push(
            &mut host,
            first,
            TrackInputRecordV1::TrimDb {
                lanes: BuiltinLaneSelector::Both,
                db: -144.0,
                smoothing_samples: 0,
            },
        );
        push(
            &mut host,
            second,
            TrackInputRecordV1::PolarityInvert {
                lanes: BuiltinLaneSelector::Both,
                inverted: true,
                smoothing_samples: 0,
            },
        );
        let _ = render(&mut host, 4);
        peaks(&host).clone()
    };
    let forward = build(0, 5);
    let reversed = build(5, 0);
    let names: Vec<String> = forward.keys().cloned().collect();
    assert_ne!(
        forward[&names[0]], reversed[&names[0]],
        "silencing track {} is not the same as flipping it",
        names[0]
    );
    assert_ne!(
        forward[&names[5]], reversed[&names[5]],
        "and the same holds for track {}",
        names[5]
    );
    // The six tracks nobody addressed are identical in both arms, which is what says the two
    // commands went where they were addressed rather than somewhere else consistently.
    for index in [1_usize, 2, 3, 4, 6, 7] {
        assert_eq!(
            forward[&names[index]], reversed[&names[index]],
            "track {} was addressed by neither arm",
            names[index]
        );
    }
}

/// A command takes effect on the **first sample of the block that drains it**.
///
/// The drain contract every console gate in the tree rests on. With a zero-length window the whole
/// block after the push is rendered at the new coefficient, and the block before it at the old
/// one.
#[test]
fn a_command_takes_effect_on_the_whole_block_that_drains_it() {
    let session = mono_session();
    let mut host = prepare_with_console(&session);
    let before = render(&mut host, 1);
    push(
        &mut host,
        3,
        TrackInputRecordV1::TrimDb {
            lanes: BuiltinLaneSelector::Both,
            db: -40.0,
            smoothing_samples: 0,
        },
    );
    let after = render(&mut host, 1);
    assert_ne!(
        before, after,
        "the block that drains the record renders at the new coefficient"
    );

    // The oracle: a plan whose session declared that trim from the start renders the same block.
    let mut declared = prepare_with_console(&session);
    push(
        &mut declared,
        3,
        TrackInputRecordV1::TrimDb {
            lanes: BuiltinLaneSelector::Both,
            db: -40.0,
            smoothing_samples: 0,
        },
    );
    let declared_first = render(&mut declared, 1);
    let mut second = render(&mut declared, 1);
    second.truncate(after.len());
    assert_eq!(
        after, second,
        "the second block of a plan commanded at block 0 is the block a plan commanded at block 1 \
         renders -- the coefficient moved at the boundary, not inside a block"
    );
    assert_ne!(declared_first, before);
}

// ---------------------------------------------------------------------------------------------
// 3. The collapse census follows the commands.
// ---------------------------------------------------------------------------------------------

/// An asymmetric ride declines exactly the ridden track; a symmetric one declines nothing; and a
/// re-equalising ride does not bring the declined track back.
///
/// Red mutation: drop the `witness.admit(&record)` call from `BuiltinBankProcessor::begin_block`
/// -> the asymmetric arm stops declining and the census stays at eight.
///
/// Red mutation: drain in `process` rather than `begin_block` -> `BankChain::run` reads the
/// witness *before* the drain, so the declining block is one late and the census after a single
/// block still reports eight eligible lanes.
#[test]
fn the_collapse_census_follows_the_commands() {
    let session = mono_session();

    // A symmetric ride declines nothing, however long it runs.
    //
    // The census counts **bank lanes**, not tracks: this fixture binds an eight-lane input bank, an
    // eight-lane fader bank, an eight-lane matrix bank and an EQ cohort, so the total is larger
    // than the track count and the assertions below are on the delta. What matters is that one
    // one-lane command declines exactly one lane.
    let mut symmetric = prepare_with_console(&session);
    let before = census(&symmetric);
    assert_eq!(
        before[0], before[1],
        "the mono fixture starts fully eligible, or the negatives below prove nothing"
    );
    assert!(before[1] >= TRACKS as u64);
    for track in 0..TRACKS {
        push(
            &mut symmetric,
            track,
            TrackInputRecordV1::TrimDb {
                lanes: BuiltinLaneSelector::Both,
                db: -12.0,
                smoothing_samples: 256,
            },
        );
    }
    let _ = render(&mut symmetric, 6);
    assert_eq!(
        census(&symmetric),
        before,
        "a `Both` ride is symmetry-preserving: every lane stays eligible"
    );

    // One asymmetric ride declines exactly one lane, on the block that drains it.
    let mut asymmetric = prepare_with_console(&session);
    push(
        &mut asymmetric,
        2,
        TrackInputRecordV1::TrimDb {
            lanes: BuiltinLaneSelector::Left,
            db: -12.0,
            smoothing_samples: 256,
        },
    );
    let _ = render(&mut asymmetric, 1);
    assert_eq!(
        census(&asymmetric),
        [before[0] - 1, before[1]],
        "one one-lane command declines exactly one lane, on the first block that drains it"
    );

    // Putting it back does not bring it back.
    push(
        &mut asymmetric,
        2,
        TrackInputRecordV1::TrimDb {
            lanes: BuiltinLaneSelector::Right,
            db: -12.0,
            smoothing_samples: 256,
        },
    );
    let _ = render(&mut asymmetric, 8);
    assert_eq!(
        census(&asymmetric),
        [before[0] - 1, before[1]],
        "re-equalising the two channels' words does not re-arm the lane: the `LIVE` term is a \
         latch, and only a rebind clears it"
    );

    // A polarity flip is the same class of event.
    let mut flipped = prepare_with_console(&session);
    push(
        &mut flipped,
        7,
        TrackInputRecordV1::PolarityInvert {
            lanes: BuiltinLaneSelector::Right,
            inverted: true,
            smoothing_samples: 64,
        },
    );
    let _ = render(&mut flipped, 1);
    assert_eq!(
        census(&flipped),
        [before[0] - 1, before[1]],
        "a one-lane polarity flip declines its lane exactly as a one-lane trim ride does"
    );
}

/// A symmetric ride's rendered bits are the bits a never-collapsing plan renders.
///
/// The stereo fixture never collapses (its two lanes read different source channels), so it is the
/// never-collapsed oracle for the mono one -- except that the two sessions differ, so the
/// comparison is not of mixes but of the *mono* plan against itself with the collapse forced off.
/// The engine has no such switch, so what is compared instead is the mono plan's output under a
/// symmetric ride against the same plan's output under the same ride issued as two per-lane
/// commands, which declines the collapse and renders dual throughout.
///
/// Red mutation: drop `mirror_trim_ramp` from `InputStage::process_mono` -> the collapsed arm's
/// right channel freezes mid-ride and the two arms diverge.
#[test]
fn a_symmetric_ride_renders_the_same_bits_collapsed_or_not() {
    let session = mono_session();

    let mut collapsed = prepare_with_console(&session);
    let mut dual = prepare_with_console(&session);
    for track in 0..TRACKS {
        push(
            &mut collapsed,
            track,
            TrackInputRecordV1::TrimDb {
                lanes: BuiltinLaneSelector::Both,
                db: -9.0,
                smoothing_samples: 512,
            },
        );
        // The same coefficient, reached by two per-lane records: identical arithmetic, and a
        // declined collapse.
        for lanes in [BuiltinLaneSelector::Left, BuiltinLaneSelector::Right] {
            push(
                &mut dual,
                track,
                TrackInputRecordV1::TrimDb {
                    lanes,
                    db: -9.0,
                    smoothing_samples: 512,
                },
            );
        }
    }
    let eligible_before = census(&dual)[0];
    let collapsed_bits = render(&mut collapsed, 8);
    let dual_bits = render(&mut dual, 8);
    assert_eq!(
        census(&dual)[0],
        eligible_before - TRACKS as u64,
        "the two-record arm declines all eight input-bank lanes, so those tracks render dual \
         throughout"
    );
    assert_eq!(
        census(&collapsed)[0],
        eligible_before,
        "the one-record arm declines nothing"
    );
    assert_eq!(
        collapsed_bits, dual_bits,
        "a collapsed run through a symmetric ride renders the never-collapsed bits"
    );
}
