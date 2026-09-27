//! Issue #996: the standing 64-track console, fed non-repeating hot noise, renders the pre-#990
//! limiter's words while its linked stereo pairs link, unlink and are made equal again mid-run.
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
//! # The session
//!
//! `console-sixty-four-track-intended.json`, prepared through the host facade as a host prepares
//! it: sixty-four strips of builtins, EQ and compressor on `simd1`, and a true-peak limiter
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
//!   limiter that only releases never reads its window's older half. With them bypassed, every
//!   unlink below turns M1 red on five to seven of its bank's eight lanes.
//!
//! The streams are seeded SplitMix64 noise, each 96-frame segment at its own level between +6 and
//! +18 dBFS peak. The segment divides neither the quantum nor the window, so nothing repeats.
//! Each limiter's gain-reduction tap is armed at a one-block window from block 0. Every retarget
//! goes through the limiter's own live control channel and lands as a `Point` span at the first
//! sample of its block; `ceiling` is parameter index 0 and `release` index 1.
//!
//! | block | track | retarget | its pair |
//! |---|---|---|---|
//! | 12 | `ch18` | ceiling -3 dB, left **and** right in one block | stays linked through the ramp |
//! | 17 | `ch03` | ceiling -4 dB, left | unlinks |
//! | 21 | `ch42` | release 400 ms, right | unlinks |
//! | 34 | `ch03` | ceiling -4 dB, right: equal again | stays dual |
//! | 36 | `ch42` | release back to 112.5 ms, right: equal again | stays dual |
//! | 49 | `ch57` | ceiling -2 dB, left | unlinks, after 49 linked blocks |
//! | 51 | `ch29` | release 30 ms, left | unlinks |
//! | 53 | `ch12` | ceiling -6 dB, left | unlinks |
//! | 60 | `ch18` | ceiling back to -1.0625 dB, both in one block | stays linked |
//! | 66 | `ch57` | ceiling -2 dB, right: equal again | stays dual |
//! | 68 | `ch12` | ceiling -6 dB, right: equal again | stays dual |
//! | 81 | `ch37` | ceiling -3 dB, left | unlinks, after 81 linked blocks |
//! | 83 | `ch05` | release 250 ms, left | unlinks at W4; at W8 its bank is dual since block 17 |
//! | 90 | `ch37` | ceiling -3 dB, right: equal again | stays dual |
//!
//! Each event lands in a different bank at both launch widths (eight tracks per bank at W8, four
//! at W4). At W8, `ch16`-`ch23` and `ch48`-`ch55` stay linked for the whole run. Every unlink
//! lands where the van Herk phase, `128 * block mod 241`, is at most 37. So for the first 203 to
//! 235 frames after it, part of the right channel's window is still answered from the previous
//! van Herk block, which is where a stale right ring gives a wrong minimum.
//!
//! # What is pinned, and how
//!
//! One SHA-256 over, after every block, the master's 256 output words and every limiter's
//! published window (its sequence number and both channels' reduction words). It was recorded on
//! the **pre-#990 kernel**: this tree with `crates/true-peak-limiter/src/lib.rs` put back to its
//! text at `bbcf8ce1`, which never links. The batch head renders the same digest.
//!
//! Three checks run ahead of the pin, and all three held on the pre-#990 kernel:
//!
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
//! back together (blocks 34, 36, 66, 68 and 90). It links again only after `reset`, a restore
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
//! `crates/true-peak-limiter/tests/MUTATIONS.md`) moves the readings of exactly those five pairs,
//! and the digest. The relink decision itself is asserted inside the crate, where the kernel's
//! choice is visible (`the_linked_body_engages_exactly_where_the_record_allows`, #990 gate 2).
//!
//! # Width
//!
//! The host facade prepares at `Backend::current()`, so this renders W8 on `x86-64-v3` (the only
//! arm CI runs it on) and W4 on AArch64. The pin was recorded at W8. Banking regroups lanes and
//! never changes a lane's arithmetic, so a W4 host must render the same digest. The kernel's own
//! pinned scenario, `crates/true-peak-limiter/tests/linked.rs`, covers W4 and scalar per width.

use core::num::NonZeroUsize;

use bench_support::digest::Sha256Sink;
use effect_contract::{EffectControlRecord, ParameterChannel};
use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use host_core::{
    EffectRack, HostConsoleHandles, HostConsoleRequest, HostPrepareCaps, HostShapePolicy,
    PreparedHost, SourceSubmission, prepare_host_session_with_console,
};
use session::{canonical_session_json, parse_session_json};

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

/// SHA-256 of the scenario, recorded on the pre-#990 kernel (see the module note).
const PIN: &str = "b22ef17ce523cc4bfa79ef3948469983636ffabffedad01d9de34203934f54d5";

/// One scheduled retarget: `(block, track, parameter index, channels, value)`.
type Retarget = (usize, usize, u32, &'static [ParameterChannel], f32);

const LEFT: &[ParameterChannel] = &[ParameterChannel::Left];
const RIGHT: &[ParameterChannel] = &[ParameterChannel::Right];
const BOTH: &[ParameterChannel] = &[ParameterChannel::Left, ParameterChannel::Right];

/// The module note's table, in block order.
const RETARGETS: [Retarget; 14] = [
    (12, 18, CEILING, BOTH, -3.0),
    (17, 3, CEILING, LEFT, -4.0),
    (21, 42, RELEASE, RIGHT, 400.0),
    (34, 3, CEILING, RIGHT, -4.0),
    (36, 42, RELEASE, RIGHT, 112.5),
    (49, 57, CEILING, LEFT, -2.0),
    (51, 29, RELEASE, LEFT, 30.0),
    (53, 12, CEILING, LEFT, -6.0),
    (60, 18, CEILING, BOTH, -1.062_5),
    (66, 57, CEILING, RIGHT, -2.0),
    (68, 12, CEILING, RIGHT, -6.0),
    (81, 37, CEILING, LEFT, -3.0),
    (83, 5, RELEASE, LEFT, 250.0),
    (90, 37, CEILING, RIGHT, -3.0),
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

fn prepare() -> Console {
    let (_session, prepared, handles) =
        prepare_host_session_with_console(&session(), &caps(), &console()).unwrap_or_else(
            |failure| panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes())),
        );
    assert_eq!(handles.tracks.len(), TRACKS);
    assert!(
        prepared.report.effect_bank_scratch_bytes > 0,
        "the cohort planner bound homogeneous banks on this host"
    );
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
}

fn run() -> Run {
    let mut console = prepare();
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
        for &(at, track, parameter_index, channels, value) in &RETARGETS {
            if at == block {
                for &channel in channels {
                    push(
                        &mut console.handles,
                        console.limiters[track],
                        EffectControlRecord::Parameter {
                            parameter_index,
                            channel,
                            value,
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
    }
}

/// The first block at which `track`'s two channels were designed apart, if any.
fn first_asymmetric_block(track: usize) -> Option<usize> {
    RETARGETS
        .iter()
        .filter(|&&(_, retargeted, _, channels, _)| retargeted == track && channels.len() == 1)
        .map(|&(block, ..)| block)
        .min()
}

#[test]
fn the_hot_console_renders_the_pre_990_words_through_link_and_unlink() {
    let run = run();

    // Every limiter is limiting, on both channels, all run: more than 6 dB of reduction at every
    // block end, and a reduction that keeps being raised rather than only released (read on the
    // left channel; the right follows it on every pair the table never parts).
    let mut shallowest = f32::INFINITY;
    for (block, row) in run.readings.iter().enumerate().skip(LIMITING_FROM) {
        for (track, &(left, right)) in row.iter().enumerate() {
            assert!(
                left > REDUCTION_FLOOR && right > REDUCTION_FLOOR,
                "block {block}: track {track}'s limiter reduces too little ({left}, {right})"
            );
            shallowest = shallowest.min(left).min(right);
        }
    }
    for track in 0..TRACKS {
        let rises = (LIMITING_FROM + 1..BLOCKS)
            .filter(|&block| run.readings[block][track].0 > run.readings[block - 1][track].0)
            .count();
        assert!(
            rises >= MINIMUM_RISES,
            "track {track}'s limiter raised its reduction in {rises} blocks: it is only releasing"
        );
    }
    println!("shallowest reduction word from block {LIMITING_FROM}: {shallowest}");

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
                "track {track} was never retargeted on one side and its channels stay equal"
            ),
            Some(block) => {
                let split = first_split.unwrap_or_else(|| {
                    panic!("track {track}'s one-sided retarget at block {block} parted nothing")
                });
                assert!(
                    split >= block,
                    "track {track}'s channels parted at block {split}, before its retarget"
                );
            }
        }
    }

    if std::env::var_os("MISO_ENGINE_REPIN_LIMITER_LINKED_SESSION").is_some() {
        println!("limiter linked session digest: {}", run.digest);
        return;
    }
    assert_eq!(
        run.digest, PIN,
        "the hot console's words moved from the pre-#990 kernel's"
    );
}
