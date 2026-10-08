//! The cross-target determinism corpus for this crate's lane-generic arithmetic.
//!
//! Gate G5 (`tools/wasm-gates/tests/g5_native_corpus.rs`) hashes each case at every width and
//! compares it against [`DIGESTS`] natively; `tools/wasm-gate-corpus` replays the identical cases
//! under wasmtime, at the wasm `simd128` backend, against these same pins. Together
//! that is the cross-target half of decision D5 for this effect: the crossover's recursive all-pass
//! sections, the D7 flush of its six recursive words, the lane `log2`/`exp2` its detector and its
//! makeup ride, the branching smoother and the detector link all produce the same bits in a browser
//! as on a native host.
//!
//! # No recurrence across points, but one rendered case
//!
//! Every case but the last is a **pure function of its per-point inputs**. A recurrence across
//! points at width `W` runs `W` interleaved sub-sequences, so its digest would depend on the width
//! and one pin could not serve every backend. The filter and smoother cases therefore take their
//! previous state as an input draw, which is what their numerics actually depend on.
//!
//! The last case, `process_block/segment_dispatch` (issue #1473), renders the composition: eight
//! independent tracks through the product `process_block`, in groups of `W`, read back **lane
//! major** before hashing, as the compressor's corpus does. Every recurrence there is per lane and
//! never crosses lanes, so its digest is width independent too. It is the one wasm digest that
//! reaches `process_block`'s per-segment dispatch (issue #1455): which segments run the armed
//! crossover and which the unarmed one, and the unarmed form's bits. The whole-block width
//! identity is also proven natively by `tests/identity.rs`, which compares rendered blocks at
//! `WIDTH = 1`, 4 and 8 by `to_bits`.
//!
//! # No NaN, no infinity
//!
//! D5 excludes NaN payloads because wasm canonicalises them. Every draw is a finite value in the
//! domain the function actually works in, and `tests/cross_target_digest.rs` asserts finiteness
//! rather than assuming it.
//!
//! # The inputs are built from integers
//!
//! The generator is a `xorshift64*`, defined entirely in wrapping integer operations, and every
//! conversion from it is exact. A target cannot differ on the *inputs* and make a digest mismatch
//! look like a numerics bug.

use effect_contract::{
    AutomationSpanKind, EffectQuality, InitialParameterValue, LinkMode, NativeEffectFactory,
    NativeEffectRegistry, ParameterChannel, PrepareEffectLimits, PrepareEffectRequest,
    PreparedAutomationSpan, PreparedPorts, PreparedSidechainPort, ProcessReport, StatePayloadInput,
    StatePayloadOutput, expected_prepared_metadata,
};
use effect_runtime::envelope::retention_coefficient;
use effect_runtime::state_payload::write_f32;
use lane::Lane;
use lane::kernels::SvfState;

use crate::shim::{LINK_AVERAGE, LINK_MAXIMUM, branching_smooth, link_levels};
use crate::{
    BandCoef, FILTER_WORD, Instance, Lr4State, MULTIBAND_COMPRESSOR_DESCRIPTOR,
    MultibandCompressorFactory, PARAMETER_COUNT, SILENCE_WORD, STATE_LAYOUT_VERSION,
    band_amplitude, initial_defaults, lr4_coefficients, lr4_step, render,
};

/// Input points in every case. A multiple of the widest backend.
pub const POINTS: usize = 1 << 14;

/// Number of cases.
pub const CASE_COUNT: usize = 7;

/// Human-readable name of each case, indexed by case number.
pub const CASE_NAMES: [&str; CASE_COUNT] = [
    "lr4_step/low",
    "lr4_step/high",
    "band_amplitude",
    "branching_smooth",
    "link_levels/maximum",
    "link_levels/average",
    "process_block/segment_dispatch",
];

/// `xorshift64*`. Integer-only, so every target builds the same sequence.
struct Rng(u64);

impl Rng {
    const fn new(case: usize) -> Self {
        Self(0x9e37_79b9_7f4a_7c15 ^ ((case as u64).wrapping_mul(0x0004_9b1f_2c07_1d33) | 1))
    }

    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }
}

/// One signal sample in `[-2, 2)`, plus the edge values a crossover has to survive.
fn signal_point(rng: &mut Rng, index: usize) -> f32 {
    const EDGES: [f32; 6] = [0.0, -0.0, 1.0, -1.0, 1.0e-30, -1.0e-30];
    if index < EDGES.len() {
        return EDGES[index];
    }
    ((rng.next() >> 40) as f32 / 16_777_216.0) * 4.0 - 2.0
}

/// One recursive state word in `[-1, 1)`, seeded subnormal often enough that D7's flush decides
/// the bits on a real share of the points.
fn state_point(rng: &mut Rng, index: usize) -> f32 {
    const EDGES: [f32; 4] = [0.0, -0.0, 1.0e-40, -1.0e-40];
    if index < EDGES.len() {
        return EDGES[index];
    }
    let draw = rng.next();
    if draw.is_multiple_of(8) {
        // A magnitude below FLUSH_EPS, so the flush has to remove it identically everywhere.
        return f32::from_bits(((draw >> 8) as u32 & 0x007f_ffff) | ((draw as u32 & 1) << 31));
    }
    ((draw >> 40) as f32 / 16_777_216.0) * 2.0 - 1.0
}

/// One positive amplitude in roughly `[2^-30, 2^3]`, uniform over octaves.
fn amplitude_point(rng: &mut Rng, index: usize) -> f32 {
    const EDGES: [f32; 4] = [1.0, 0.5, 1.0e-8, f32::MIN_POSITIVE];
    if index < EDGES.len() {
        return EDGES[index];
    }
    let draw = rng.next();
    let exponent = 97 + (draw % 34) as u32;
    let mantissa = ((draw >> 16) as u32) & 0x007f_ffff;
    f32::from_bits((exponent << 23) | mantissa)
}

/// One value in `[low, high]`, at a thousandth-of-a-unit step.
fn ranged_point(rng: &mut Rng, low: f32, high: f32) -> f32 {
    let steps = ((high - low) * 1_000.0) as u64 + 1;
    low + (rng.next() % steps) as u32 as f32 * 0.001
}

/// Runs one corpus case, writing [`POINTS`] result words to `out`.
///
/// # Panics
///
/// Panics if `out.len()` is not [`POINTS`], or if `case` is not below [`CASE_COUNT`].
pub fn run_case<L: Lane>(case: usize, out: &mut [u32]) {
    assert!(case < CASE_COUNT, "corpus case out of range");
    assert_eq!(out.len(), POINTS, "corpus output must be POINTS long");
    let mut rng = Rng::new(case);
    match case {
        0 | 1 => {
            let coefficients = lr4_coefficients::<L>(48_000, 1_000.0).expect("frozen design");
            let high = case == 1;
            map_case::<L, _, _>(
                out,
                |index| {
                    [
                        signal_point(&mut rng, index),
                        state_point(&mut rng, index),
                        state_point(&mut rng, index + POINTS),
                        state_point(&mut rng, index + 2 * POINTS),
                        state_point(&mut rng, index + 3 * POINTS),
                    ]
                },
                |values| {
                    let mut state = Lr4State {
                        a: SvfState {
                            ic1: values[1],
                            ic2: values[2],
                        },
                        b: SvfState {
                            ic1: values[3],
                            ic2: values[4],
                        },
                    };
                    // A one-frame split whose input has no history: a fresh silence counter, so
                    // the joint flush is never armed here (issue #1328, amendment A9).
                    let mut run = L::zero();
                    let rest = lane::silence_step(
                        values[0],
                        &mut run,
                        L::splat(lane::silence_frames(48_000) as f32),
                    );
                    let (low, band) = lr4_step(values[0], rest, &coefficients, &mut state);
                    if high { band } else { low }
                },
            );
        }
        2 => {
            let coefficients = BandCoef {
                inv_ratio_minus_one: L::splat(1.0 / 4.0 - 1.0),
                attack: L::splat(retention_coefficient(10.0, 48_000)),
                release: L::splat(retention_coefficient(100.0, 48_000)),
            };
            map_case::<L, _, _>(
                out,
                |index| {
                    [
                        amplitude_point(&mut rng, index),
                        ranged_point(&mut rng, -80.0, 0.0),
                        ranged_point(&mut rng, -24.0, 24.0),
                        ranged_point(&mut rng, -100.0, 0.0),
                        0.0,
                    ]
                },
                |values| {
                    let mut state = values[3];
                    band_amplitude(values[0], values[1], values[2], &coefficients, &mut state)
                },
            );
        }
        3 => {
            let attack = L::splat(retention_coefficient(0.1, 48_000));
            let release = L::splat(retention_coefficient(5_000.0, 48_000));
            map_case::<L, _, _>(
                out,
                |index| {
                    let _ = index;
                    [
                        ranged_point(&mut rng, -100.0, 0.0),
                        ranged_point(&mut rng, -100.0, 0.0),
                        0.0,
                        0.0,
                        0.0,
                    ]
                },
                |values| branching_smooth(values[0], values[1], attack, release),
            );
        }
        6 => match L::WIDTH {
            1 => dispatch_case::<L, 1>(out),
            4 => dispatch_case::<L, 4>(out),
            8 => dispatch_case::<L, 8>(out),
            _ => unreachable!("a lane width the bank does not ship"),
        },
        _ => {
            let average = case == 5;
            map_case::<L, _, _>(
                out,
                |index| {
                    [
                        signal_point(&mut rng, index),
                        signal_point(&mut rng, index + POINTS),
                        0.0,
                        0.0,
                        0.0,
                    ]
                },
                |values| {
                    let (near, far) = if average {
                        link_levels::<L, LINK_AVERAGE>(values[0], values[1])
                    } else {
                        link_levels::<L, LINK_MAXIMUM>(values[0], values[1])
                    };
                    near.add(far.mul(L::splat(3.0)))
                },
            );
        }
    }
}

/// Evaluates one case block by block: fill `L::WIDTH` points, apply, store the result words.
fn map_case<L: Lane, P, F>(out: &mut [u32], mut point: P, mut apply: F)
where
    P: FnMut(usize) -> [f32; 5],
    F: FnMut([L; 5]) -> L,
{
    let width = L::WIDTH;
    let mut inputs = [[0.0f32; 32]; 5];
    let mut bits = [0u32; 32];
    let mut index = 0;
    while index < POINTS {
        for offset in 0..width {
            let values = point(index + offset);
            for (slot, value) in inputs.iter_mut().zip(values) {
                slot[offset] = value;
            }
        }
        let lanes = core::array::from_fn(|slot| L::load(&inputs[slot][..width]));
        apply(lanes).store_bits(&mut bits[..width]);
        out[index..index + width].copy_from_slice(&bits[..width]);
        index += width;
    }
}

// ---------------------------------------------------------------------------------------------
// The rendered case: `process_block`'s per-segment dispatch (issue #1473)
// ---------------------------------------------------------------------------------------------

/// Independent tracks the rendered case runs. A multiple of the widest backend.
const DISPATCH_TRACKS: usize = 8;

/// Frames rendered per track and channel, in each link mode. With two link modes, two channels and
/// [`DISPATCH_TRACKS`] tracks this is exactly [`POINTS`] result words.
const DISPATCH_FRAMES: usize = POINTS / (2 * 2 * DISPATCH_TRACKS);

/// The block partition: four blocks, each one segment unless an automation point splits it.
const DISPATCH_BLOCK: usize = 128;

/// The rate the case is prepared at; `lane::silence_frames` is 4,096 there.
const DISPATCH_RATE: u32 = 48_000;

/// The two link modes the case renders, one after the other: the arming test is per channel in
/// both, while the detector link makes each channel's gain depend on the other's.
const DISPATCH_LINKS: [LinkMode; 2] = [LinkMode::DualMono, LinkMode::Maximum];

/// A restored crossover state on one channel of one track: the plan-swap state a long silence
/// after a quiet low-crossover tail hands over (issue #1328's payload carries the counter).
///
/// Each seed's channel is silent from frame 0 until `silent_until`, and its counter reaches
/// `N_SILENCE` = 4,096 inside exactly one segment, where its recursive words lie in
/// `[FLUSH_EPS, REST_EPS)`, so the joint flush zeroes them there and the per-word flush would not.
/// No other lane of the bank can arm in that segment, so each segment's dispatch rests on one
/// channel and one stage:
///
/// | track | channel | counter | held stages | arming segment                    | the dispatch reads |
/// |-------|---------|---------|-------------|-----------------------------------|--------------------|
/// | 0     | left    | 3,990   | `a` only    | block 0, the first after restore  | left, stage `a`    |
/// | 1     | left    | 3,900   | `b` only    | block 1                           | left, stage `b`    |
/// | 2     | right   | 3,800   | both        | block 2's ramping head            | right              |
/// | 3     | right   | 3,620   | both        | block 3's flat tail               | right              |
///
/// Track 0's segment has to be the first after the restore: one frame later its stage `a` has fed
/// stage `b`, and both are held. Each arming segment is the same at every width, because the only
/// automation points are on the seeded track whose segment they split ([`DISPATCH_POINTS`]), and
/// a block another seed's point splits holds no arming segment of its own.
///
/// Every other channel is live, with 23-frame gaps of exact zeros that reset no arming.
struct Seed {
    track: usize,
    channel: usize,
    silence: f32,
    /// `a.ic1, a.ic2, b.ic1, b.ic2`, the payload's filter-word order.
    filter: [f32; 4],
    silent_until: usize,
}

const SEEDS: [Seed; 4] = [
    Seed {
        track: 0,
        channel: 0,
        silence: 3_990.0,
        filter: [3.0e-16, -2.0e-16, 0.0, 0.0],
        silent_until: 256,
    },
    Seed {
        track: 1,
        channel: 0,
        silence: 3_900.0,
        filter: [0.0, 0.0, 4.0e-16, 1.0e-16],
        silent_until: 384,
    },
    Seed {
        track: 2,
        channel: 1,
        silence: 3_800.0,
        filter: [5.0e-16, 2.0e-16, -3.0e-16, 4.0e-16],
        silent_until: 320,
    },
    Seed {
        track: 3,
        channel: 1,
        silence: 3_620.0,
        filter: [-6.0e-16, 3.0e-16, 2.0e-16, -5.0e-16],
        silent_until: DISPATCH_FRAMES,
    },
];

/// The automation points: `(track, channel, parameter, value, block)`. Each one ramps for 64
/// frames, so it splits its block into a ramping head of 63 frames and a flat tail. Block 2's head
/// runs armed and its tail unarmed; block 3's head runs unarmed (track 3's counter, 4,004, cannot
/// reach 4,096 in 63 frames) and its tail armed, with the counter advanced over the head by
/// `silence_skip_block`.
const DISPATCH_POINTS: [(usize, ParameterChannel, u32, f32, usize); 2] = [
    (2, ParameterChannel::Right, 6, -30.0, 2),
    (3, ParameterChannel::Right, 1, -50.0, 3),
];

/// The seed of `(track, channel)`, if it has one.
fn seed_of(track: usize, channel: usize) -> Option<&'static Seed> {
    SEEDS
        .iter()
        .find(|seed| seed.track == track && seed.channel == channel)
}

/// One track's prepared parameters, in the descriptor's `parameter * 2 + channel` order.
///
/// A seeded channel has an 80 Hz crossover, whose tail is the one a long silence leaves near rest,
/// and unequal band makeups, so a crossover word that is not zeroed reaches the output as
/// `low * (a_low - a_high)` rather than cancelling between the bands. The live tracks spread the
/// crossover over the domain and the curve over its arms.
fn dispatch_values(track: usize) -> [InitialParameterValue; PARAMETER_COUNT * 2] {
    const LIVE: [[f32; PARAMETER_COUNT]; 4] = [
        [
            250.0, -30.0, 4.0, 1.0, 50.0, 3.0, -24.0, 2.0, 5.0, 80.0, -2.0,
        ],
        [
            1_000.0, -42.0, 12.0, 0.2, 20.0, 6.0, -36.0, 8.0, 0.3, 200.0, 0.0,
        ],
        [
            3_000.0, -18.0, 20.0, 0.1, 5.0, -3.0, -12.0, 1.5, 20.0, 1_000.0, 4.0,
        ],
        [
            8_000.0, -6.0, 1.0, 10.0, 100.0, 0.0, -50.0, 6.0, 2.0, 500.0, -6.0,
        ],
    ];
    const SEEDED: [f32; PARAMETER_COUNT] = [
        80.0, -40.0, 4.0, 5.0, 200.0, 6.0, -40.0, 4.0, 5.0, 200.0, -6.0,
    ];
    core::array::from_fn(|index| {
        let parameter = index / 2;
        let channel = index % 2;
        let value = if seed_of(track, channel).is_some() {
            SEEDED[parameter]
        } else {
            LIVE[track % 4][parameter]
        };
        InitialParameterValue {
            parameter_index: parameter as u32,
            channel: if channel == 0 {
                ParameterChannel::Left
            } else {
                ParameterChannel::Right
            },
            value,
        }
    })
}

/// One channel's input: exact zeros while a seed holds it silent, then a signal in `[-1, 1)` with a
/// 23-frame run of exact zeros every 97 frames, short enough that no live counter nears 4,096.
fn dispatch_input(track: usize, channel: usize) -> [f32; DISPATCH_FRAMES] {
    let silent_until = seed_of(track, channel).map_or(0, |seed| seed.silent_until);
    let mut rng = Rng::new(0x100 + track * 2 + channel);
    core::array::from_fn(|frame| {
        let draw = ((rng.next() >> 40) as f32 / 16_777_216.0) * 2.0 - 1.0;
        if frame < silent_until || frame % 97 < 23 {
            0.0
        } else {
            draw
        }
    })
}

/// The rendered case at width `W` (`L::WIDTH == W`): each link mode, each group of `W` tracks
/// prepared as one bank, the seeds restored through the state payload, then four blocks rendered
/// through `render` and so `process_block`, and read back lane major.
fn dispatch_case<L: Lane, const W: usize>(out: &mut [u32]) {
    debug_assert_eq!(L::WIDTH, W);
    // The request carries the registry's tail-bound entry, as every product prepare does.
    let factory: Box<dyn NativeEffectFactory> = Box::new(MultibandCompressorFactory);
    let tail_bound = NativeEffectRegistry::new([factory])
        .expect("the registry admits the multiband")
        .tail_bound(
            MULTIBAND_COMPRESSOR_DESCRIPTOR.id,
            DISPATCH_RATE,
            EffectQuality::Normal,
        )
        .expect("a launch rate");
    for (mode, link_mode) in DISPATCH_LINKS.into_iter().enumerate() {
        for group in 0..DISPATCH_TRACKS / W {
            let tracks = group * W..(group + 1) * W;
            let values: [[InitialParameterValue; PARAMETER_COUNT * 2]; W] =
                core::array::from_fn(|lane| dispatch_values(group * W + lane));
            let metadata = expected_prepared_metadata(
                &MULTIBAND_COMPRESSOR_DESCRIPTOR,
                PrepareEffectRequest {
                    sample_rate: DISPATCH_RATE,
                    quantum: DISPATCH_BLOCK as u32,
                    quality: EffectQuality::Normal,
                    bypass: false,
                    link_mode,
                    ports: PreparedPorts {
                        sidechain: PreparedSidechainPort::None,
                    },
                    initial_values: &values[0],
                    limits: PrepareEffectLimits {
                        maximum_total_state_bytes: u64::MAX,
                        maximum_scratch_bytes: u64::MAX,
                        maximum_automation_spans_per_block: 32,
                    },
                    tail_bound,
                },
            )
            .expect("frozen request");
            let mut left_defaults = [[0.0; PARAMETER_COUNT]; W];
            let mut right_defaults = [[0.0; PARAMETER_COUNT]; W];
            for lane in 0..W {
                (left_defaults[lane], right_defaults[lane]) =
                    initial_defaults(&values[lane]).expect("frozen parameters");
            }
            let mut instance = Instance::<L, W>::new(left_defaults, right_defaults, metadata)
                .expect("frozen crossovers");

            // The seeds go in through the plan-swap state door, validated like any payload.
            let sizes = metadata.state_sizes;
            for lane in 0..W {
                let track = group * W + lane;
                if seed_of(track, 0).is_none() && seed_of(track, 1).is_none() {
                    continue;
                }
                let mut common = vec![0u8; sizes.common_bytes as usize];
                let mut sections = [
                    vec![0u8; sizes.left_bytes as usize],
                    vec![0u8; sizes.right_bytes as usize],
                ];
                let [left, right] = &mut sections;
                instance
                    .snapshot(
                        lane,
                        StatePayloadOutput::new(&mut common, left, right, sizes)
                            .expect("prepared sizes"),
                        sizes,
                    )
                    .expect("snapshot");
                for (channel, section) in sections.iter_mut().enumerate() {
                    if let Some(seed) = seed_of(track, channel) {
                        for (index, word) in seed.filter.into_iter().enumerate() {
                            write_f32(section, FILTER_WORD + index, word);
                        }
                        write_f32(section, SILENCE_WORD, seed.silence);
                    }
                }
                instance
                    .restore(
                        lane,
                        STATE_LAYOUT_VERSION,
                        StatePayloadInput::new(&common, &sections[0], &sections[1], sizes)
                            .expect("prepared sizes"),
                    )
                    .expect("a valid seed");
            }

            let inputs: [[[f32; DISPATCH_FRAMES]; 2]; W] = core::array::from_fn(|lane| {
                let track = group * W + lane;
                [dispatch_input(track, 0), dispatch_input(track, 1)]
            });
            let mut left = vec![0.0f32; DISPATCH_FRAMES * W];
            let mut right = vec![0.0f32; DISPATCH_FRAMES * W];
            for frame in 0..DISPATCH_FRAMES {
                for lane in 0..W {
                    left[frame * W + lane] = inputs[lane][0][frame];
                    right[frame * W + lane] = inputs[lane][1][frame];
                }
            }

            let mut reports = [ProcessReport::default(); W];
            for block in 0..DISPATCH_FRAMES / DISPATCH_BLOCK {
                let first_sample = (block * DISPATCH_BLOCK) as u64;
                for (track, channel, parameter, value, at) in DISPATCH_POINTS {
                    if at != block || !tracks.contains(&track) {
                        continue;
                    }
                    let span = PreparedAutomationSpan {
                        kind: AutomationSpanKind::Point,
                        channel,
                        parameter_index: parameter,
                        start_sample: first_sample,
                        end_sample: first_sample,
                        start_value: value,
                        end_value: value,
                    };
                    let lane = track - group * W;
                    instance.apply_automation(
                        lane,
                        &[span],
                        metadata.automation_capacity,
                        first_sample,
                        &mut reports[lane],
                    );
                }
                let span = block * DISPATCH_BLOCK * W..(block + 1) * DISPATCH_BLOCK * W;
                render::<L, W, false>(
                    &mut instance,
                    &mut left[span.clone()],
                    &mut right[span],
                    DISPATCH_BLOCK,
                    &mut reports,
                );
            }
            assert!(
                reports
                    .iter()
                    .all(|report| *report == ProcessReport::default()),
                "the frozen render reports nothing"
            );

            for lane in 0..W {
                let track = group * W + lane;
                for (channel, plane) in [&left, &right].into_iter().enumerate() {
                    let base = ((mode * DISPATCH_TRACKS + track) * 2 + channel) * DISPATCH_FRAMES;
                    for frame in 0..DISPATCH_FRAMES {
                        out[base + frame] = plane[frame * W + lane].to_bits();
                    }
                }
            }
        }
    }
}

/// Pinned SHA-256 of each case's result words, little-endian, in case order.
///
/// Generated once from the **scalar** `Lane` instantiation on `x86_64` (master plan §8.3: a pin
/// comes from the oracle, never from a SIMD or a wasm run) and checked at all three widths and on
/// every target. A mismatch is never fixed by re-pinning: it means either the corpus changed or a
/// target stopped agreeing with the oracle, which is what this gate exists to catch.
pub const DIGESTS: [[u8; 32]; CASE_COUNT] = include!("corpus_digests.in");
