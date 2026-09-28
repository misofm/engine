//! Issues #996 and #997: the standing 64-track console, fed non-repeating hot noise, renders the
//! pre-#990 limiter's words at every bank width (`Simd8`, `Simd4` and `Scalar`) while its linked
//! stereo pairs link, ramp, unlink and are made equal again mid-run.
//!
//! # Why this test exists
//!
//! Issue #990 computes a `LinkMode::Maximum` pair's gain path once, on the left channel's words,
//! and mirrors every ring write into the right channel so that the right channel holds exactly
//! what a dual run would have left there. Mirroring is class A: a linked block renders the same
//! words either way, and the mirrored words are read only **after** the pair unlinks. So a
//! session sees a mirroring defect only if it links for a while, then unlinks a pair whose
//! limiter is limiting, on material whose gain requirement moves inside the limiter's window.
//!
//! No session test did that. `console-workload` repeats one frozen 128-frame block, shorter than
//! the 241-frame window at 5 ms, so every window minimum is constant; its tone barely limits; and
//! a session exposes no effect state. Under #990's M1 (the mirrored backward pass skips its store
//! into the right ring) every session digest stayed green. This is the session test the #990
//! verification asked for (its finding 1).
//!
//! Issue #997 closed the three gaps the #996 verification found: the test ran only at the host's
//! own width (W8 here, while the browser renders four-lane banks); one linked-ramp trial left
//! #990's M9 (the right ramps not set from the left at block end) green at five of eight seeds;
//! and no retarget unlinked a W8 bank from its lane 0 or its last lane, so a defect confined to a
//! bank's edge lanes passed at W8.
//!
//! # The session
//!
//! `console-sixty-four-track-intended.json`, compiled and prepared through the host facade's own
//! pipeline: sixty-four strips of builtins, EQ and compressor on `simd1`, and a true-peak limiter
//! (`channel: "both"`, `maximum` link) alone on `simd2`. Two edits, each made for a measured
//! reason:
//!
//! * **One stereo stream per track.** The fixture's one source is widened to 128 channels and
//!   track `k` reads channels `2k` and `2k + 1`. With the fixture's one stereo source every lane
//!   of a bank hears the same noise, so an unlink is one trial rather than eight. On that shape,
//!   with the compressors bypassed and three unlinks, M1 stayed green.
//! * **Every compressor bypassed** from block 0, through its live control channel (the rack's
//!   latency-preserving shunt). The fixture's compressors, up to 7.5:1 from -19.5 dB, hold most
//!   tracks under their ceilings: on the shared source, 46 of 64 limiters never raised their
//!   reduction after block 32, because they were only releasing from the onset transient, and a
//!   limiter that only releases never reads its window's older half. With them bypassed, M1 moves
//!   the readings of five to seven of the eight lanes of every W8 bank that unlinks, from its
//!   unlink block on.
//!
//! The streams are seeded SplitMix64 noise, each 96-frame segment at its own level between +6 and
//! +18 dBFS peak. The segment divides neither the quantum nor the window, so nothing repeats.
//! Each limiter's gain-reduction tap is armed at a one-block window from block 0. Every retarget
//! goes through the limiter's own live control channel and lands as a `Point` span at the first
//! sample of its block; `ceiling` is parameter index 0 and `release` index 1. "Both" is two
//! single-channel spans in one block, left then right, which leave the pair's designed words
//! equal.
//!
//! | block | tracks | retarget | the pairs |
//! |---|---|---|---|
//! | 12 | `ch16`-`ch23` | ceiling -3 dB, both | stay linked through the ramp |
//! | 17 | `ch03` | ceiling -4 dB, left | unlinks |
//! | 21 | `ch42` | release 400 ms, right | unlinks |
//! | 26 | `ch48`-`ch55` | release 250 ms, both | stay linked through the ramp |
//! | 34 | `ch03` | ceiling -4 dB, right: equal again | stays dual |
//! | 36 | `ch42` | release back to 112.5 ms, right: equal again | stays dual |
//! | 44 | `ch48`-`ch55` | ceiling -3.5 dB, both | stay linked |
//! | 49 | `ch57` | ceiling -2 dB, left | unlinks, after 49 linked blocks |
//! | 51 | `ch29` | release 30 ms, left | unlinks |
//! | 53 | `ch12` | ceiling -6 dB, left | unlinks |
//! | 60 | `ch16`-`ch23` | ceiling -1.5 dB, both | stay linked |
//! | 66 | `ch23` | ceiling -4 dB, left | unlinks from the bank's last lane |
//! | 66 | `ch57` | ceiling -2 dB, right: equal again | stays dual |
//! | 68 | `ch12` | ceiling -6 dB, right: equal again | stays dual |
//! | 81 | `ch38` | ceiling -3 dB, left | unlinks, after 81 linked blocks |
//! | 83 | `ch05` | release 250 ms, left | unlinks at W4; at W8 its bank is dual since block 17 |
//! | 85 | `ch48` | release 180 ms, right | unlinks from the bank's lane 0, after 85 linked blocks |
//! | 90 | `ch38` | ceiling -3 dB, right: equal again | stays dual |
//! | 90 | `ch23` | ceiling -4 dB, right: equal again | stays dual |
//! | 92 | `ch48` | release back to 250 ms, right: equal again | stays dual |
//!
//! Banks are eight consecutive tracks at W8 and four at W4; at `Scalar` nothing banks and every
//! track is its own one-lane instance. The test reads each track's bank and lane from the
//! prepared plan rather than assuming them.
//!
//! * **Linked ramps.** The four "both" rows retarget every lane of a bank that is still linked
//!   (bank 2 and bank 6 at W8, four W4 banks). The 64-sample ramp runs inside the landing block,
//!   so that block renders linked through the ramping dispatch, and the right ramps must leave it
//!   equal to the left's. Under M9 they do not, the next block finds the designed words apart
//!   and renders dual, and the right channel ramps a block late. At each of eight seeds that
//!   parts three to five of `ch16`-`ch22` and six or seven of `ch49`-`ch55` (the pairs the first
//!   two ramps reach and no one-sided retarget does), at every width, and the channels-equal
//!   check fires.
//! * **Unlinks.** The first one-sided retarget of each bank unlinks it: all eight banks at W8,
//!   nine banks at W4, nine instances at `Scalar`. At W8 those eight retargets land on eight
//!   different lanes, `ch48` on lane 0 and `ch23` on lane 7; at W4 they cover all four lanes
//!   (`ch12` and `ch48` on lane 0, `ch03` and `ch23` on lane 3). The test asserts that every lane
//!   of its width carries one. Every unlink lands where the van Herk phase,
//!   `128 * block mod 241`, is at most 37. So for the first 203 to 235 frames after it, part of
//!   the right channel's window is still answered from the previous van Herk block, which is where
//!   a stale right ring gives a wrong minimum.
//!
//! # What is pinned, and how
//!
//! One SHA-256 over, after every block, the master's 256 output words and every limiter's
//! published window (its sequence number and both channels' reduction words). It was recorded on
//! the **pre-#990 kernel**, which never links: this tree with `crates/true-peak-limiter` put back
//! to its text at `bbcf8ce1`. `Simd8`, `Simd4` and `Scalar` rendered the same digest there, in
//! dev and in release, and so does the batch head. One pin serves every width because banking
//! regroups lanes and never changes a lane's arithmetic.
//!
//! Four checks run ahead of the pin, and all four held on the pre-#990 kernel:
//!
//! * every lane of a bank carries the retarget that unlinks some bank (above);
//! * every limiter is limiting: from block 3, more than 6 dB of reduction on both channels at
//!   every block end, and a left reading that rises in at least four blocks, so no limiter is
//!   only releasing;
//! * under `maximum`, a pair whose designed words agree computes equal reduction words, so a
//!   track's two readings stay bit-equal until a one-sided retarget reaches it;
//! * every one-sided retarget parts them.
//!
//! # A pair relinks only after `reset`, a restore or `desymmetrize`: intended
//!
//! Once a one-sided retarget unlinks a pair, the pair stays unlinked when its designed words come
//! back together (blocks 34, 36, 66, 68, 90 and 92). It links again only after `reset`, a restore
//! whose gain words compare equal, or `desymmetrize`. That is **intended**. It is the #990
//! brief's invariant: equal designed words are no proof of equal gain words. This scenario shows
//! why. When `ch03`'s right ceiling reaches the left's at block 34, the right channel's rings
//! still hold requirements computed at its old ceiling, and its reduction word carries the old
//! value, so relinking there would give the right channel the left channel's gain.
//!
//! The cost is liveness only. A live session has no reset command. A stereo track never
//! collapses, so its bank never desymmetrizes. A restore happens only into a bank being prepared
//! for a replacement plan. So a pair that was ever retargeted on one side renders dual for the
//! rest of its plan. A cheaper proof (for example at silent rest, the #990 verification's L2)
//! would be class A and is out of scope here.
//!
//! What this test asserts about it: a session cannot see *whether* a pair relinked, only the
//! words it rendered. So it asserts that every block after a pair is equal again renders the
//! pre-#990 words. A kernel that relinks on designed agreement alone (row 996-K2 of
//! `crates/true-peak-limiter/tests/MUTATIONS.md`) moves the digest. The relink decision itself is
//! asserted inside the crate, where the kernel's choice is visible
//! (`the_linked_body_engages_exactly_where_the_record_allows`, #990 gate 2).
//!
//! # Width
//!
//! A unit test, so that it can prepare through `prepare_host_runtime_with_console_backend`, the
//! `#[cfg(test)]` seam that is `prepare_host_runtime_with_console` at a named backend. It renders
//! all three widths on every host, CI's `x86-64-v3` arm included. The wasm guest is not covered:
//! it plays one frozen block per track, and a per-block source there is tooling (the #996
//! verdict's owner ruling).

use core::num::NonZeroUsize;
use core::ops::RangeInclusive;

use bench_support::digest::Sha256Sink;
use effect_contract::{EffectControlRecord, ParameterChannel};
use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use graph_compiler::Backend;
use session::{canonical_session_json, parse_session_json};

use crate::prepare::{compile_host_session, prepare_host_runtime_with_console_backend};
use crate::{
    EffectRack, HostConsoleHandles, HostConsoleRequest, HostPrepareCaps, HostShapePolicy,
    PrepareDiagnostics, PreparedHost, SourceSubmission,
};

const FIXTURE: &str =
    include_str!("../../../fixtures/session/v1/console-sixty-four-track-intended.json");
const SOURCE: &[u8] = b"fixture-source";
const LIMITER: &str = "miso.true-peak-limiter";
const COMPRESSOR: &str = "miso.compressor";
const TRACKS: usize = 64;
/// Source channels: one independent stereo stream per track.
const CHANNELS: usize = 2 * TRACKS;
const QUANTUM: usize = 128;
const BLOCKS: usize = 96;
/// Frames per noise segment. Divides neither the quantum nor the limiter's 241-frame window.
const SEGMENT: u64 = 96;
const SEED: u64 = 0x0996;
const CEILING: u32 = 0;
const RELEASE: u32 = 1;
/// The first block whose readings the non-vacuity checks hold to: the limiters' onset.
const LIMITING_FROM: usize = 3;
/// A reduction word above 0.5 is more than 6 dB of gain reduction.
const REDUCTION_FLOOR: f32 = 0.5;
/// Blocks, from [`LIMITING_FROM`], at whose end each limiter's reduction must have risen.
const MINIMUM_RISES: usize = 4;

/// SHA-256 of the scenario, recorded on the pre-#990 kernel at every width (see the module note).
const PIN: &str = "6d87267b7502a4b4cb663315629d777350ce6ecc62a2d14e0eaebca9b88d9ff9";

const LEFT: &[ParameterChannel] = &[ParameterChannel::Left];
const RIGHT: &[ParameterChannel] = &[ParameterChannel::Right];
const BOTH: &[ParameterChannel] = &[ParameterChannel::Left, ParameterChannel::Right];

/// One scheduled retarget: every track in `tracks` moves `parameter` to `value` on `channels`, at
/// the first sample of `block`.
struct Retarget {
    block: usize,
    tracks: RangeInclusive<usize>,
    parameter: u32,
    channels: &'static [ParameterChannel],
    value: f32,
}

impl Retarget {
    /// A retarget of one channel: it designs the pair apart (or back together).
    fn one_sided(&self) -> bool {
        self.channels.len() == 1
    }
}

const fn at(
    block: usize,
    tracks: RangeInclusive<usize>,
    parameter: u32,
    channels: &'static [ParameterChannel],
    value: f32,
) -> Retarget {
    Retarget {
        block,
        tracks,
        parameter,
        channels,
        value,
    }
}

/// The module note's table, in block order.
const RETARGETS: [Retarget; 20] = [
    at(12, 16..=23, CEILING, BOTH, -3.0),
    at(17, 3..=3, CEILING, LEFT, -4.0),
    at(21, 42..=42, RELEASE, RIGHT, 400.0),
    at(26, 48..=55, RELEASE, BOTH, 250.0),
    at(34, 3..=3, CEILING, RIGHT, -4.0),
    at(36, 42..=42, RELEASE, RIGHT, 112.5),
    at(44, 48..=55, CEILING, BOTH, -3.5),
    at(49, 57..=57, CEILING, LEFT, -2.0),
    at(51, 29..=29, RELEASE, LEFT, 30.0),
    at(53, 12..=12, CEILING, LEFT, -6.0),
    at(60, 16..=23, CEILING, BOTH, -1.5),
    at(66, 23..=23, CEILING, LEFT, -4.0),
    at(66, 57..=57, CEILING, RIGHT, -2.0),
    at(68, 12..=12, CEILING, RIGHT, -6.0),
    at(81, 38..=38, CEILING, LEFT, -3.0),
    at(83, 5..=5, RELEASE, LEFT, 250.0),
    at(85, 48..=48, RELEASE, RIGHT, 180.0),
    at(90, 38..=38, CEILING, RIGHT, -3.0),
    at(90, 23..=23, CEILING, RIGHT, -4.0),
    at(92, 48..=48, RELEASE, RIGHT, 250.0),
];

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 1_024,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 256,
        maximum_sources: 16,
        maximum_routes: 256,
        maximum_effects: 256,
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

/// A live control channel per effect and one observation tap per effect; no meters.
fn console() -> HostConsoleRequest {
    HostConsoleRequest {
        control_queue_depth: Some(NonZeroUsize::new(8).expect("depth")),
        observation_taps: 1,
        ..HostConsoleRequest::default()
    }
}

/// The standing fixture with its one stereo source widened to a stereo stream per track.
fn session() -> String {
    let mut model = parse_session_json(FIXTURE).expect("the fixture parses");
    assert_eq!(model.sources.len(), 1, "the fixture has one source");
    model.sources[0].channels = u8::try_from(CHANNELS).expect("channel count");
    for (index, track) in model.tracks.iter_mut().enumerate() {
        track.left_source_channel = u8::try_from(2 * index).expect("left channel");
        track.right_source_channel = u8::try_from(2 * index + 1).expect("right channel");
    }
    canonical_session_json(&model).expect("the widened fixture canonicalizes")
}

/// SplitMix64's finaliser over one counter.
const fn mix(counter: u64) -> u64 {
    let mut z = counter.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// A uniform `f32` in `[0, 1)` on a 2^-24 grid: exact, and the same on every target.
fn unit(bits: u64) -> f32 {
    (bits >> 40) as f32 * (1.0 / 16_777_216.0)
}

/// Source channel `channel` at absolute frame `frame`: uniform noise times its segment's level.
///
/// The level is `2 + 6u`, +6 to +18 dBFS peak, drawn per channel per [`SEGMENT`] frames.
fn source_sample(channel: u64, frame: u64) -> f32 {
    let stream = (SEED << 52) ^ (channel << 40);
    let level = 2.0 + 6.0 * unit(mix(stream ^ (1 << 39) ^ (frame / SEGMENT)));
    let noise = unit(mix(stream ^ (frame + 1))) * 2.0 - 1.0;
    level * noise
}

struct Console {
    prepared: PreparedHost,
    handles: HostConsoleHandles,
    /// `limiters[t]` indexes `handles.effect_controls` for track `t`'s limiter.
    limiters: Vec<usize>,
    /// `compressors[t]` indexes `handles.effect_controls` for track `t`'s compressor.
    compressors: Vec<usize>,
    /// `observers[t]` indexes `handles.effect_observations` for track `t`'s limiter.
    observers: Vec<usize>,
}

fn refused(failure: &PrepareDiagnostics) -> ! {
    panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
}

fn prepare(backend: Backend) -> Console {
    let compiled = compile_host_session(&session(), &caps()).unwrap_or_else(|f| refused(&f));
    let (prepared, handles) =
        prepare_host_runtime_with_console_backend(&compiled, &caps(), &console(), backend)
            .unwrap_or_else(|f| refused(&f));
    assert_eq!(handles.tracks.len(), TRACKS);
    for (index, track) in handles.tracks.iter().enumerate() {
        assert_eq!(
            **track,
            *format!("ch{index:02}"),
            "track {index} is the table's ch{index:02}"
        );
    }
    let controls = |rack: EffectRack, effect_index: u32, effect: &str| -> Vec<usize> {
        handles
            .tracks
            .iter()
            .map(|track| {
                handles
                    .effect_controls
                    .iter()
                    .position(|producer| {
                        producer.track_id == *track
                            && producer.rack == rack
                            && producer.effect_index == effect_index
                            && producer.descriptor.id.as_str() == effect
                    })
                    .unwrap_or_else(|| panic!("track {track}'s {effect} has a control channel"))
            })
            .collect()
    };
    let limiters = controls(EffectRack::Simd2, 0, LIMITER);
    let compressors = controls(EffectRack::Simd1, 1, COMPRESSOR);
    let observers = handles
        .tracks
        .iter()
        .map(|track| {
            handles
                .effect_observations
                .iter()
                .position(|handle| {
                    handle.track_id == *track
                        && handle.rack == EffectRack::Simd2
                        && handle.effect_index == 0
                        && handle.descriptor.id.as_str() == LIMITER
                })
                .unwrap_or_else(|| panic!("track {track}'s limiter has an observation handle"))
        })
        .collect();
    Console {
        prepared,
        handles,
        limiters,
        compressors,
        observers,
    }
}

/// Where each track renders at `backend`'s width, read from the prepared plan: `(bank, lane)`,
/// with a bank named by the track on its lane 0.
///
/// Every banked unit that renders a track must agree on its bank-mates and its lane, and every
/// bank must be full: `width` lanes. At `Scalar` nothing banks, and every track is its own
/// one-lane instance, `(track, 0)`.
fn placement(console: &Console, backend: Backend) -> Vec<(usize, usize)> {
    let width = backend.width();
    let [chains, _slots] = console.prepared.plan.bank_shape();
    let units = console.prepared.plan.unit_eligibility();
    let banks: Vec<&[Box<str>]> = units
        .iter()
        .filter(|unit| unit.banked)
        .map(|unit| &unit.lane_tracks[..])
        .collect();
    if backend == Backend::Scalar {
        assert_eq!(chains, 0, "nothing banks at Scalar");
        assert!(banks.is_empty(), "nothing banks at Scalar");
        return (0..TRACKS).map(|track| (track, 0)).collect();
    }
    assert!(chains > 0, "the cohort planner bound banks at {backend:?}");
    console
        .handles
        .tracks
        .iter()
        .map(|track| {
            let mut found: Option<(&[Box<str>], usize)> = None;
            for bank in &banks {
                let Some(lane) = bank.iter().position(|member| member == track) else {
                    continue;
                };
                assert_eq!(bank.len(), width, "{track}'s bank at {backend:?} is full");
                if let Some((mates, known)) = found {
                    assert!(
                        mates == *bank && known == lane,
                        "every unit renders {track} in one bank and lane at {backend:?}"
                    );
                }
                found = Some((bank, lane));
            }
            let (mates, lane) =
                found.unwrap_or_else(|| panic!("{track} renders in a bank at {backend:?}"));
            let first = console
                .handles
                .tracks
                .iter()
                .position(|member| *member == mates[0])
                .expect("a bank-mate is a track");
            (first, lane)
        })
        .collect()
}

fn push(handles: &mut HostConsoleHandles, channel: usize, record: EffectControlRecord) {
    handles.effect_controls[channel]
        .try_push(record)
        .unwrap_or_else(|_| panic!("room in control channel {channel}'s queue"));
}

/// What the run saw, besides its digest.
struct Run {
    digest: String,
    /// `[block][track]` published reduction words, `(left, right)`.
    readings: Vec<Vec<(f32, f32)>>,
    /// Track -> `(bank, lane)` at the run's width (see [`placement`]).
    placement: Vec<(usize, usize)>,
}

fn run(backend: Backend) -> Run {
    let mut console = prepare(backend);
    let placement = placement(&console, backend);
    let mut hash = Sha256Sink::new();
    let mut readings = Vec::with_capacity(BLOCKS);
    // Before block 0, so both land at its first sample.
    for track in 0..TRACKS {
        push(
            &mut console.handles,
            console.compressors[track],
            EffectControlRecord::Bypass(true),
        );
        push(
            &mut console.handles,
            console.limiters[track],
            EffectControlRecord::Observe {
                tap_index: 0,
                armed: true,
                window_blocks: 1,
            },
        );
    }
    let mut planes = vec![[0.0_f32; QUANTUM]; CHANNELS];
    for block in 0..BLOCKS {
        for retarget in RETARGETS.iter().filter(|retarget| retarget.block == block) {
            for track in retarget.tracks.clone() {
                for &channel in retarget.channels {
                    push(
                        &mut console.handles,
                        console.limiters[track],
                        EffectControlRecord::Parameter {
                            parameter_index: retarget.parameter,
                            channel,
                            value: retarget.value,
                        },
                    );
                }
            }
        }
        let first = (block * QUANTUM) as u64;
        for (channel, plane) in planes.iter_mut().enumerate() {
            for (frame, sample) in plane.iter_mut().enumerate() {
                *sample = source_sample(channel as u64, first + frame as u64);
            }
        }
        let views: Vec<&[f32]> = planes.iter().map(|plane| &plane[..]).collect();
        console
            .prepared
            .sources
            .submit(
                SOURCE,
                SourceSubmission {
                    generation: 1,
                    start_frame: first,
                    sample_rate_hz: 48_000,
                    planes: &views,
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
        let mut samples = [0.0_f32; QUANTUM * 2];
        let output =
            PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM).expect("output planes");
        console
            .prepared
            .plan
            .render(
                RenderIo {
                    input: None,
                    output,
                },
                RenderTime {
                    absolute_sample: first,
                },
            )
            .expect("render");
        assert!(
            samples.iter().all(|word| word.is_finite()),
            "block {block}: the master is finite"
        );
        for word in samples {
            hash.update(word.to_bits().to_le_bytes());
        }
        let mut row = Vec::with_capacity(TRACKS);
        for track in 0..TRACKS {
            let handle = &console.handles.effect_observations[console.observers[track]];
            let window = handle.readers[0]
                .read()
                .unwrap_or_else(|| panic!("block {block}: track {track} published a window"));
            assert_eq!(
                window.sequence,
                block as u64 + 1,
                "block {block}: track {track} publishes one window per block"
            );
            hash.update(window.sequence.to_le_bytes());
            hash.update(window.left.to_bits().to_le_bytes());
            hash.update(window.right.to_bits().to_le_bytes());
            row.push((window.left, window.right));
        }
        readings.push(row);
    }
    Run {
        digest: hash.finish_hex(),
        readings,
        placement,
    }
}

/// The first block at which `track`'s two channels were designed apart, if any.
fn first_asymmetric_block(track: usize) -> Option<usize> {
    RETARGETS
        .iter()
        .filter(|retarget| retarget.one_sided() && retarget.tracks.contains(&track))
        .map(|retarget| retarget.block)
        .min()
}

/// The lanes that receive the first one-sided retarget of their bank (the one that unlinks it).
fn unlinking_lanes(placement: &[(usize, usize)]) -> Vec<usize> {
    let mut first: Vec<(usize, usize, usize)> = Vec::new(); // (bank, block, lane)
    for retarget in RETARGETS.iter().filter(|retarget| retarget.one_sided()) {
        for track in retarget.tracks.clone() {
            let (bank, lane) = placement[track];
            match first.iter_mut().find(|(known, ..)| *known == bank) {
                Some(entry) if retarget.block < entry.1 => *entry = (bank, retarget.block, lane),
                Some(_) => {}
                None => first.push((bank, retarget.block, lane)),
            }
        }
    }
    first.into_iter().map(|(_, _, lane)| lane).collect()
}

#[test]
fn the_hot_console_renders_the_pre_990_words_at_simd8() {
    check(Backend::Simd8);
}

#[test]
fn the_hot_console_renders_the_pre_990_words_at_simd4() {
    check(Backend::Simd4);
}

#[test]
fn the_hot_console_renders_the_pre_990_words_at_scalar() {
    check(Backend::Scalar);
}

fn check(backend: Backend) {
    let run = run(backend);

    println!(
        "limiter linked session digest at {backend:?}: {}",
        run.digest
    );

    // Every lane of a bank, the two edge lanes included, carries the retarget that unlinks some
    // bank.
    let lanes = unlinking_lanes(&run.placement);
    for lane in 0..backend.width() {
        assert!(
            lanes.contains(&lane),
            "at {backend:?}, some bank is unlinked by a retarget on its lane {lane}: {lanes:?}"
        );
    }

    // Every limiter is limiting, on both channels, all run: more than 6 dB of reduction at every
    // block end, and a reduction that keeps being raised rather than only released (read on the
    // left channel; the right follows it on every pair the table never parts).
    let mut shallowest = f32::INFINITY;
    for (block, row) in run.readings.iter().enumerate().skip(LIMITING_FROM) {
        for (track, &(left, right)) in row.iter().enumerate() {
            assert!(
                left > REDUCTION_FLOOR && right > REDUCTION_FLOOR,
                "{backend:?} block {block}: track {track}'s limiter reduces too little ({left}, {right})"
            );
            shallowest = shallowest.min(left).min(right);
        }
    }
    let mut fewest = usize::MAX;
    for track in 0..TRACKS {
        let rises = (LIMITING_FROM + 1..BLOCKS)
            .filter(|&block| run.readings[block][track].0 > run.readings[block - 1][track].0)
            .count();
        assert!(
            rises >= MINIMUM_RISES,
            "{backend:?}: track {track}'s limiter raised its reduction in {rises} blocks: it is only releasing"
        );
        fewest = fewest.min(rises);
    }
    println!(
        "{backend:?}: shallowest reduction word from block {LIMITING_FROM}: {shallowest}, fewest rises {fewest}"
    );

    // The retargets landed where the table says, and no pair parted anywhere else.
    for track in 0..TRACKS {
        let apart = first_asymmetric_block(track);
        let first_split = run
            .readings
            .iter()
            .position(|row| row[track].0.to_bits() != row[track].1.to_bits());
        match apart {
            None => assert_eq!(
                first_split, None,
                "{backend:?}: track {track} was never retargeted on one side and its channels stay equal"
            ),
            Some(block) => {
                let split = first_split.unwrap_or_else(|| {
                    panic!("{backend:?}: track {track}'s one-sided retarget at block {block} parted nothing")
                });
                assert!(
                    split >= block,
                    "{backend:?}: track {track}'s channels parted at block {split}, before its retarget"
                );
            }
        }
    }

    if std::env::var_os("MISO_ENGINE_REPIN_LIMITER_LINKED_SESSION").is_some() {
        println!("limiter linked session checks pass at {backend:?}");
        return;
    }
    assert_eq!(
        run.digest, PIN,
        "{backend:?}: the hot console's words moved from the pre-#990 kernel's"
    );
}
