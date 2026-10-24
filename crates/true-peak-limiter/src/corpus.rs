//! The cross-target determinism corpus for the limiter kernel (master plan gate G5).
//!
//! Each case runs the crate's `limiter_block` kernel over [`LANES`] independent stereo tracks
//! for [`FRAMES`] frames and yields one `u32` result word per output sample, **lane major**: left
//! channel of lane 0, then lane 1, and so on, then the right channel the same way. At width `W`
//! the corpus is processed in `LANES / W` groups of an AoSoA block and read back lane major before
//! hashing, so a digest describes the arithmetic and not the layout, and `W = 1`, `4` and `8` must
//! produce the same 32 bytes on every target.
//!
//! `tools/wasm-gate-corpus` replays these cases under wasmtime, with and without
//! `simd128`, against [`D90_DIGESTS`]. The pins live here, next to the kernel they describe, so the
//! wasm leg replays this crate's gate rather than a transcription of it.
//!
//! # Why a stateful kernel can be pinned at every width
//!
//! A recurrence digested at width `W` would normally run `W` interleaved sub-sequences and its
//! digest would depend on the width. It does not here because a lane of this bank *is* a track: the
//! AoSoA arena keeps a whole independent limiter per lane, and the group loop hands lane `l` the
//! same signal at every width. Nothing crosses lanes except the `Maximum` link, which links the two
//! channels of one lane and not two lanes of one channel.
//!
//! # The linked pair (#990, #996, #997)
//!
//! Case 5, `maximum/link_unlink_relink`, is the only case that drives the whole prepared core
//! (`LimiterCore::process_block`) block by block instead of one `limiter_block` call, because the
//! linked-pair decision and its record live there. Eight 128-frame blocks, every lane at the same
//! 5 ms lookahead so the bank is uniform and the linked body runs, `LinkMode::Maximum`, and loud
//! noise whose level changes every 96 frames so every limiter limits and its window minimum moves.
//! The retargets (`LINKED_RETARGETS`) land while the output carries delayed audio (from frame
//! 486): a linked ramp of both channels of every lane at block 3, a one-sided retarget of lane 0
//! and of lane 7 at block 4 (each unlinks its bank at every width: lane 0 and the last lane of a
//! four-lane bank, the edge lanes of the eight-lane one), and the other channel made equal again at
//! block 5, after which the pairs must stay dual. Its pin equals the words of a kernel that never
//! links, so it renders what a mirrored right ring leaves behind after an unlink and a pair that
//! relinks on designed agreement alone (`tests/MUTATIONS.md`, section "TA3"). It replaced the
//! host-core 64-track linked-limiter session pin (TA3, 2026-10-19), whose session checks stay in
//! `crates/host-core/src/limiter_linked_session.rs`.
//!
//! # No NaN
//!
//! The determinism claim excludes NaN payloads (wasm canonicalises them). Every case is driven with
//! finite input inside the ranges the boundary check accepts, and the crate's determinism test
//! asserts the outputs are finite rather than assuming it.

use effect_contract::{
    EffectQuality, LinkMode, PreparedEffectMetadata, PreparedPorts, PreparedSidechainPort,
};
use lane::Lane;

use crate::{
    ChannelState, Cursors, LimiterCoef, LimiterCore, PARAMETER_COUNT, QUALITIES, RAMP_UPDATES,
    Shape, TRUE_PEAK_LIMITER_DESCRIPTOR, limit_coefficient, limiter_block, release_coefficient,
    tail_and_rest,
};

/// Independent tracks in every case; a multiple of the widest backend.
pub const LANES: usize = 8;

/// Frames per track: past the 486-sample latency at 48 kHz, so most of the digest is real audio.
pub const FRAMES: usize = 1024;

/// Result words per case: both channels of every lane.
pub const POINTS: usize = 2 * LANES * FRAMES;

/// The rate every case runs at.
const RATE: u32 = 48_000;

/// Number of corpus cases.
pub const CASE_COUNT: usize = 6;

/// Human-readable name of each case, indexed by case number.
pub const CASE_NAMES: [&str; CASE_COUNT] = [
    "dual_mono/noise",
    "maximum/noise",
    "dual_mono/near_nyquist_w_min",
    "dual_mono/impulse_train_long_lookahead",
    "dual_mono/subnormal_release_flush",
    "maximum/link_unlink_relink",
];

/// The pinned digest of each case, generated from the scalar `Lane` instantiation.
///
/// Re-pinning is permitted only from that oracle and only with the deviation stated in the commit
/// message (master plan §8). A wasm mismatch is never fixed by re-pinning: it means a target
/// stopped agreeing with the scalar law, which is the whole reason the gate exists.
///
/// G5 owns these scalar-rendered pins and cross-target bit identity. Independent numerical
/// validation lives in `tests/gain_law.rs`: the `f64` reference checks the gain law and Annex-2
/// output ceiling. Those tolerance-based checks complement the digest and do not establish its
/// exact words.
pub const D90_DIGESTS: [[u8; 32]; CASE_COUNT] = [
    // dual_mono/noise
    [
        0x50, 0x58, 0x50, 0x2b, 0x14, 0xa4, 0xad, 0x0b, 0x26, 0x84, 0xae, 0xec, 0xbe, 0x2f, 0xe5,
        0xb9, 0x73, 0x9d, 0xe2, 0x3e, 0xc3, 0x11, 0xa6, 0x6b, 0xd5, 0xf0, 0x4f, 0x7d, 0xd8, 0x41,
        0x9a, 0x12,
    ],
    // maximum/noise
    [
        0x97, 0x83, 0xa0, 0x8e, 0x15, 0x78, 0x5a, 0x35, 0x96, 0x4a, 0xd1, 0xb5, 0x81, 0xcd, 0xc0,
        0xe4, 0x86, 0xb9, 0x63, 0x2e, 0x8c, 0x30, 0xfa, 0x74, 0xdc, 0x20, 0x31, 0xb4, 0x65, 0xda,
        0xc9, 0x62,
    ],
    // dual_mono/near_nyquist_w_min
    [
        0x68, 0x99, 0x38, 0xbc, 0x6e, 0x8e, 0x5f, 0xa4, 0xd0, 0xb4, 0x0f, 0x25, 0x9a, 0xe4, 0xbe,
        0xbf, 0xf4, 0xb0, 0x22, 0x1a, 0xf7, 0x0f, 0x71, 0xd2, 0xe0, 0x70, 0x74, 0x39, 0xb1, 0xb0,
        0x68, 0x72,
    ],
    // dual_mono/impulse_train_long_lookahead
    [
        0x95, 0x6c, 0xa8, 0xf3, 0x31, 0x6a, 0x53, 0x37, 0xfd, 0xd0, 0xd3, 0x70, 0x17, 0xcc, 0x47,
        0xae, 0x25, 0x90, 0x76, 0xf8, 0xc3, 0x09, 0x73, 0x89, 0xa4, 0xfb, 0xfd, 0x54, 0x11, 0xda,
        0xe6, 0x20,
    ],
    // dual_mono/subnormal_release_flush
    [
        0x54, 0x6b, 0x4a, 0x87, 0x98, 0x61, 0xe7, 0x8d, 0x96, 0x22, 0x62, 0x9b, 0x0a, 0x14, 0xb1,
        0x42, 0x93, 0xca, 0x05, 0x80, 0x2e, 0x25, 0x30, 0x46, 0xe2, 0x79, 0xb5, 0x58, 0xba, 0x6a,
        0xe0, 0x26,
    ],
    // maximum/link_unlink_relink
    [
        0x31, 0xb0, 0x96, 0x72, 0xde, 0x3d, 0xfa, 0x2d, 0x04, 0x2d, 0x87, 0x0e, 0x8a, 0x1e, 0xca,
        0x84, 0x9e, 0xf8, 0x79, 0x50, 0x9f, 0x1c, 0xc1, 0xbd, 0x8d, 0xa7, 0xcf, 0x71, 0x6a, 0xb0,
        0x69, 0xcd,
    ],
];

/// `xorshift64*`. Integer-only, so every target builds the same sequence.
struct Rng(u64);

impl Rng {
    const fn new(seed: u64) -> Self {
        Self(seed | 1)
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

/// The per-lane parameter table of one case: ceiling dB, release ms, lookahead ms.
fn lane_parameters(case: usize, lane: usize) -> [f32; PARAMETER_COUNT] {
    const CEILINGS: [f32; LANES] = [-1.0, -6.0, -12.0, -3.0, -0.0, -24.0, -2.0, -9.0];
    const RELEASES: [f32; LANES] = [10.0, 100.0, 2000.0, 50.0, 400.0, 1000.0, 25.0, 250.0];
    const LOOKAHEADS: [f32; LANES] = [0.0, 5.0, 10.0, 1.0, 2.5, 7.5, 0.0, 10.0];
    match case {
        2 => [-6.0, 10.0, 0.0],
        3 => [-12.0, 2000.0, 10.0],
        _ => [CEILINGS[lane], RELEASES[lane], LOOKAHEADS[lane]],
    }
}

/// Fills one lane's stereo pair for one case.
fn fill(case: usize, lane: usize, left: &mut [f32], right: &mut [f32]) {
    let mut rng = Rng::new(0xA5A5_5A5A_0090_0001 ^ (lane as u64).wrapping_mul(0x9E37_79B9));
    for frame in 0..FRAMES {
        let (l, r) = match case {
            0 | 1 => {
                let a = f32::from((rng.next() >> 48) as u16) * (2.0 / 65_536.0) - 1.0;
                let b = f32::from((rng.next() >> 48) as u16) * (2.0 / 65_536.0) - 1.0;
                (a * 2.0, b * 2.0)
            }
            2 => {
                // 0.49 * Fs at +3 dB: the classic inter-sample overshoot generator, with a
                // per-lane phase so no two lanes are the same signal.
                let phase = 0.98 * core::f64::consts::PI * frame as f64 + lane as f64 * 0.37;
                (
                    (libm_free_sine(phase) * 1.4125) as f32,
                    (libm_free_sine(phase + 1.1) * 1.4125) as f32,
                )
            }
            3 => {
                // An impulse train of varying height on a quiet bed: the long window has to hold
                // its minimum across many impulses and release between them, and the bed keeps the
                // case from collapsing to three distinct output words.
                let period = 48 + lane * 7;
                let bed = f32::from((rng.next() >> 48) as u16) * (2.0 / 65_536.0) - 1.0;
                let height = 1.0 + f32::from((rng.next() >> 52) as u16) * (3.0 / 16.0);
                if frame.is_multiple_of(period) {
                    let sign = if (frame / period).is_multiple_of(2) {
                        1.0
                    } else {
                        -1.0
                    };
                    (height * sign, -height * sign)
                } else {
                    (bed * 0.05, bed * -0.05)
                }
            }
            _ => {
                // Subnormal-magnitude noise with three loud bursts: the reduction word decays into
                // the flush band between them, so the D7 flush has to remove it identically on
                // every target and the identity has to come back exactly.
                let burst = frame % 320 < 4 && frame > 16;
                let tiny = f32::from_bits(((rng.next() >> 40) as u32 & 0x007F_FFFF) | 1);
                if burst { (3.0, -3.0) } else { (tiny, -tiny) }
            }
        };
        left[frame] = l;
        right[frame] = r;
    }
}

/// `sin` from its Taylor-reduced series, so the corpus never calls the platform libm (D6).
///
/// The corpus only needs *a* deterministic oscillator, not an accurate one: what matters is that
/// every target builds bit-identical input, which basic arithmetic guarantees and a platform `sin`
/// does not.
fn libm_free_sine(phase: f64) -> f64 {
    let turns = phase * (0.5 / core::f64::consts::PI);
    let wrapped = turns - (turns + 0.5).floor();
    let x = wrapped * core::f64::consts::TAU;
    let x2 = x * x;
    // Minimax-free Taylor series to x^13; |x| <= pi, so the truncation error is under 1e-8, which
    // is far below anything the limiter can resolve and is identical on every target.
    x * (1.0
        + x2 * (-1.0 / 6.0
            + x2 * (1.0 / 120.0
                + x2 * (-1.0 / 5040.0 + x2 * (1.0 / 362_880.0 + x2 * (-1.0 / 39_916_800.0))))))
}

/// Runs one corpus case at width `L::WIDTH`, writing [`POINTS`] result words to `out`.
///
/// # Panics
///
/// Panics if `case >= CASE_COUNT`, if `out` is not [`POINTS`] long, or if `L::WIDTH` does not
/// divide [`LANES`].
pub fn run_case<L: Lane>(case: usize, out: &mut [u32]) {
    assert!(case < CASE_COUNT, "corpus case out of range");
    assert_eq!(out.len(), POINTS, "corpus output must be POINTS long");
    let width = L::WIDTH;
    assert!(LANES.is_multiple_of(width), "LANES must divide by a width");
    if case == LINKED_CASE {
        run_linked::<L>(out);
        return;
    }

    let shape = Shape::new(RATE).expect("launch rate");
    let coefficients = LimiterCoef::<L>::new(
        if case == 1 {
            LinkMode::Maximum
        } else {
            LinkMode::DualMono
        },
        false,
    );
    let mut left_lanes = vec![0.0_f32; LANES * FRAMES];
    let mut right_lanes = vec![0.0_f32; LANES * FRAMES];
    for lane in 0..LANES {
        let (left, right) = (
            &mut left_lanes[lane * FRAMES..(lane + 1) * FRAMES],
            &mut right_lanes[lane * FRAMES..(lane + 1) * FRAMES],
        );
        fill(case, lane, left, right);
    }

    let mut left_block = vec![0.0_f32; FRAMES * width];
    let mut right_block = vec![0.0_f32; FRAMES * width];
    for group in 0..LANES / width {
        let defaults: Vec<[f32; PARAMETER_COUNT]> = (0..width)
            .map(|offset| lane_parameters(case, group * width + offset))
            .collect();
        let mut left_state = ChannelState::new::<L>(width, &shape, &defaults, RATE);
        let mut right_state = ChannelState::new::<L>(width, &shape, &defaults, RATE);
        let mut cursors = Cursors::default();
        for frame in 0..FRAMES {
            for offset in 0..width {
                let lane = group * width + offset;
                left_block[frame * width + offset] = left_lanes[lane * FRAMES + frame];
                right_block[frame * width + offset] = right_lanes[lane * FRAMES + frame];
            }
        }
        limiter_block::<L>(
            &mut left_block,
            &mut right_block,
            FRAMES,
            &coefficients,
            &shape,
            &mut left_state,
            &mut right_state,
            &mut cursors,
        );
        for frame in 0..FRAMES {
            for offset in 0..width {
                let lane = group * width + offset;
                out[lane * FRAMES + frame] = left_block[frame * width + offset].to_bits();
                out[(LANES + lane) * FRAMES + frame] =
                    right_block[frame * width + offset].to_bits();
            }
        }
    }
}

/// The case that drives the prepared core block by block (see the module note).
const LINKED_CASE: usize = 5;

/// Frames per block of [`LINKED_CASE`]: eight blocks make [`FRAMES`].
const LINKED_BLOCK: usize = 128;

/// Frames per noise segment of [`LINKED_CASE`]: divides neither the block nor the 241-frame window.
const LINKED_SEGMENT: usize = 96;

/// Per-lane ceiling (dB) and release (ms) of [`LINKED_CASE`], equal on both channels; every lane
/// at the 5 ms lookahead, so the bank is uniform.
const LINKED_CEILINGS: [f32; LANES] = [-1.0, -3.0, -6.0, -2.0, -4.0, -1.5, -5.0, -2.5];
const LINKED_RELEASES: [f32; LANES] = [50.0, 100.0, 200.0, 30.0, 400.0, 75.0, 150.0, 250.0];
const LINKED_LOOKAHEAD: f32 = 5.0;

/// The parameter a [`LinkedRetarget`] moves.
#[derive(Clone, Copy)]
enum LinkedParameter {
    Ceiling,
    Release,
}

/// Before `block`, lane `lane` (every lane when `None`) moves `parameter` to `value` on the left
/// channel when `left` and on the right channel when `right`.
struct LinkedRetarget {
    block: usize,
    lane: Option<usize>,
    parameter: LinkedParameter,
    left: bool,
    right: bool,
    value: f32,
}

/// The schedule of [`LINKED_CASE`] (see the module note).
const LINKED_RETARGETS: [LinkedRetarget; 5] = [
    // A linked ramp on both channels of every lane.
    LinkedRetarget {
        block: 3,
        lane: None,
        parameter: LinkedParameter::Ceiling,
        left: true,
        right: true,
        value: -7.0,
    },
    // Lane 0 unlinks on its left channel.
    LinkedRetarget {
        block: 4,
        lane: Some(0),
        parameter: LinkedParameter::Ceiling,
        left: true,
        right: false,
        value: -9.0,
    },
    // The last lane unlinks on its right channel.
    LinkedRetarget {
        block: 4,
        lane: Some(LANES - 1),
        parameter: LinkedParameter::Release,
        left: false,
        right: true,
        value: 20.0,
    },
    // Both made equal again: the pairs stay dual.
    LinkedRetarget {
        block: 5,
        lane: Some(0),
        parameter: LinkedParameter::Ceiling,
        left: false,
        right: true,
        value: -9.0,
    },
    LinkedRetarget {
        block: 5,
        lane: Some(LANES - 1),
        parameter: LinkedParameter::Release,
        left: true,
        right: false,
        value: 20.0,
    },
];

/// One lane's stereo pair for [`LINKED_CASE`]: uniform noise, each channel at its own level
/// between +6 and +18 dBFS peak, redrawn every [`LINKED_SEGMENT`] frames.
fn fill_linked(lane: usize, left: &mut [f32], right: &mut [f32]) {
    let mut rng = Rng::new(0x0996_0997_0990_0001 ^ (lane as u64).wrapping_mul(0x9E37_79B9));
    let unit = |rng: &mut Rng| f32::from((rng.next() >> 48) as u16) * (1.0 / 65_536.0);
    let (mut left_level, mut right_level) = (0.0, 0.0);
    for frame in 0..FRAMES {
        if frame.is_multiple_of(LINKED_SEGMENT) {
            left_level = 2.0 + 6.0 * unit(&mut rng);
            right_level = 2.0 + 6.0 * unit(&mut rng);
        }
        left[frame] = left_level * (unit(&mut rng) * 2.0 - 1.0);
        right[frame] = right_level * (unit(&mut rng) * 2.0 - 1.0);
    }
}

/// Runs [`LINKED_CASE`] at width `L::WIDTH` through the prepared core.
fn run_linked<L: Lane>(out: &mut [u32]) {
    let width = L::WIDTH;
    let mut left_lanes = vec![0.0_f32; LANES * FRAMES];
    let mut right_lanes = vec![0.0_f32; LANES * FRAMES];
    for lane in 0..LANES {
        fill_linked(
            lane,
            &mut left_lanes[lane * FRAMES..(lane + 1) * FRAMES],
            &mut right_lanes[lane * FRAMES..(lane + 1) * FRAMES],
        );
    }
    let quality = QUALITIES
        .iter()
        .find(|quality| quality.sample_rate == RATE)
        .expect("launch rate");
    let mut left_block = vec![0.0_f32; LINKED_BLOCK * width];
    let mut right_block = vec![0.0_f32; LINKED_BLOCK * width];
    for group in 0..LANES / width {
        // Both channels of a lane start equal: the pair is linked from block 0.
        let defaults: Vec<[f32; PARAMETER_COUNT]> = (0..width)
            .map(|offset| {
                let lane = group * width + offset;
                [
                    LINKED_CEILINGS[lane],
                    LINKED_RELEASES[lane],
                    LINKED_LOOKAHEAD,
                ]
            })
            .collect();
        let bound = tail_and_rest(RATE, EffectQuality::Normal);
        let metadata = PreparedEffectMetadata {
            descriptor: &TRUE_PEAK_LIMITER_DESCRIPTOR,
            sample_rate: RATE,
            quantum: LINKED_BLOCK as u32,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::Maximum,
            ports: PreparedPorts {
                sidechain: PreparedSidechainPort::None,
            },
            latency: quality.latency,
            tail: bound.tail,
            tail_every_peak: bound.tail_every_peak,
            rest: bound.rest,
            composition: bound.composition,
            state_sizes: quality.maximum_state,
            scratch_bytes: quality.scratch_fixed_bytes,
            automation_capacity: 16,
        };
        let mut core = LimiterCore::<L>::new(
            metadata,
            defaults.clone().into_boxed_slice(),
            defaults.into_boxed_slice(),
        )
        .expect("corpus core");
        for block in 0..FRAMES / LINKED_BLOCK {
            for retarget in LINKED_RETARGETS.iter().filter(|r| r.block == block) {
                for offset in 0..width {
                    let lane = group * width + offset;
                    if retarget.lane.is_some_and(|target| target != lane) {
                        continue;
                    }
                    for (moves, state) in [
                        (retarget.left, &mut core.left),
                        (retarget.right, &mut core.right),
                    ] {
                        if !moves {
                            continue;
                        }
                        match retarget.parameter {
                            LinkedParameter::Ceiling => state.limit[offset]
                                .set_target(limit_coefficient(retarget.value), RAMP_UPDATES),
                            LinkedParameter::Release => state.release[offset].set_target(
                                release_coefficient(retarget.value, RATE),
                                RAMP_UPDATES,
                            ),
                        }
                    }
                }
            }
            let first = block * LINKED_BLOCK;
            for frame in 0..LINKED_BLOCK {
                for offset in 0..width {
                    let lane = group * width + offset;
                    left_block[frame * width + offset] = left_lanes[lane * FRAMES + first + frame];
                    right_block[frame * width + offset] =
                        right_lanes[lane * FRAMES + first + frame];
                }
            }
            core.process_block(&mut left_block, &mut right_block, LINKED_BLOCK);
            for frame in 0..LINKED_BLOCK {
                for offset in 0..width {
                    let lane = group * width + offset;
                    out[lane * FRAMES + first + frame] =
                        left_block[frame * width + offset].to_bits();
                    out[(LANES + lane) * FRAMES + first + frame] =
                        right_block[frame * width + offset].to_bits();
                }
            }
        }
    }
}
