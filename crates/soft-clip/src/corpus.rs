//! The cross-target determinism corpus for the soft-clip kernel.
//!
//! One definition, run three ways: by `tests/determinism.rs` at all three widths on this host, by
//! `tests/lane_identity.rs` as the width-independence proof, and by `tools/wasm-gates`
//! inside a WebAssembly module against the same pins. A digest difference between the legs is a
//! difference in the *target*, which is what master plan #83 D5 says cannot happen.
//!
//! # Why the results are read back lane-major
//!
//! A case is [`LANES`] independent single-lane signals of [`FRAMES`] frames. At width `W` the
//! corpus is rendered in `LANES / W` groups of an AoSoA block and read back lane by lane before
//! hashing, so the digest describes the arithmetic and not the layout, and `W = 1`, 4 and 8 must
//! produce the same bytes. The kernel's recurrence is over *frames within a lane*, never across
//! lanes, so nothing about the grouping can leak into a result.
//!
//! # No NaN
//!
//! D5 excludes NaN payloads because wasm canonicalises them. Every case here is built from finite
//! inputs and finite gains, and the consumers assert finiteness rather than assuming it.

use lane::Lane;

use crate::kernel::{SoftClipCoef, SoftClipHistory, SoftClipState, soft_clip_block};

/// Independent single-lane signals per case; a multiple of the widest backend.
pub const LANES: usize = 8;

/// Frames per signal: past the 31-sample latency and the 63-sample support, and long enough for a
/// 64-sample ramp to finish and snap inside the case.
pub const FRAMES: usize = 512;

/// Result words per case, one per rendered sample.
pub const POINTS: usize = LANES * FRAMES;

/// Number of corpus cases.
pub const CASE_COUNT: usize = 6;

/// Human-readable name of each case, indexed by case number.
pub const CASE_NAMES: [&str; CASE_COUNT] = [
    "soft_clip/noise",
    "soft_clip/impulse",
    "soft_clip/dc",
    "soft_clip/subnormal",
    "soft_clip/ramped",
    "soft_clip/identity",
];

/// `xorshift64*` (Vigna 2016): integer-only, so every target builds the same input sequence.
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

    /// A value in `[-1, 1)` built from integer arithmetic and one exact conversion.
    fn next_signal(&mut self) -> f32 {
        f32::from((self.next() >> 48) as u16 >> 1) * (2.0 / 32_768.0) - 1.0
    }
}

/// Fills one lane's [`FRAMES`] input samples for a case.
fn fill(case: usize, lane: usize, out: &mut [f32]) {
    let mut rng = Rng::new(0x5C1F_0091_0000_0001 ^ ((case as u64 * 8 + lane as u64) << 8));
    for (frame, sample) in out.iter_mut().enumerate() {
        *sample = match case {
            0 | 4 => rng.next_signal(),
            1 => {
                if frame == lane {
                    1.5
                } else {
                    0.0
                }
            }
            2 => 0.5,
            // Subnormal magnitudes, so the D7 flush has to remove them identically everywhere.
            3 => f32::from_bits((rng.next() as u32 & 0x007F_FFFF) | 1),
            _ => rng.next_signal() * 0.25,
        };
    }
}

/// The three gains a lane starts from, and the per-sample increments it advances by.
struct LaneCoefficients {
    /// Drive gain, output gain and mix, in stable parameter order.
    start: [f32; 3],
    /// Per-sample increments of the same three, `+0.0` where the lane is not ramping.
    step: [f32; 3],
}

/// The three gains one lane of a case runs under, and its per-sample increments.
fn coefficients(case: usize, lane: usize) -> LaneCoefficients {
    let lane = lane as f32;
    match case {
        // Constant gains: drive from -12 to +30 dB, an output trim, a partial mix.
        0..=3 => LaneCoefficients {
            start: [
                crate::db_to_gain_f32(-12.0 + lane * 6.0),
                crate::db_to_gain_f32(3.0 - lane),
                (0.1 + lane * 0.12).min(1.0),
            ],
            step: [0.0; 3],
        },
        // A ramp on every parameter, at a different rate per lane, running for the whole case.
        4 => LaneCoefficients {
            start: [
                crate::db_to_gain_f32(-6.0 + lane),
                crate::db_to_gain_f32(-2.0 + lane * 0.5),
                0.5,
            ],
            step: [
                (lane + 1.0) * 1.0e-4,
                (lane + 1.0) * -5.0e-5,
                (lane + 1.0) * 2.0e-5,
            ],
        },
        // mix = 0, output = 1: the exact identity select, which must return the dry input bits.
        _ => LaneCoefficients {
            start: [crate::db_to_gain_f32(24.0), 1.0, 0.0],
            step: [0.0; 3],
        },
    }
}

/// Renders one case at width `L::WIDTH` and writes [`POINTS`] result words, lane-major.
///
/// # Panics
///
/// Panics if `case >= CASE_COUNT`, if `out` is not [`POINTS`] words, or if `LANES` is not a
/// multiple of the width.
pub fn run_case<L: Lane>(case: usize, out: &mut [u32]) {
    assert!(case < CASE_COUNT, "corpus case index out of range");
    assert_eq!(out.len(), POINTS, "corpus result length");
    let width = L::WIDTH;
    assert!(LANES.is_multiple_of(width), "LANES must divide by a width");

    let mut lanes = [[0.0_f32; FRAMES]; LANES];
    for (lane, signal) in lanes.iter_mut().enumerate() {
        fill(case, lane, signal);
    }
    let all = L::zero().eq(L::zero());
    let no_bypass = L::mask_not(all);
    let mut block = vec![0.0_f32; FRAMES * width];
    let mut words = [0.0_f32; LANES];

    for group in 0..LANES / width {
        for frame in 0..FRAMES {
            for offset in 0..width {
                block[frame * width + offset] = lanes[group * width + offset][frame];
            }
        }
        let load = |ramping: bool, index: usize| {
            let mut values = [0.0_f32; LANES];
            for (offset, slot) in values[..width].iter_mut().enumerate() {
                let lane = coefficients(case, group * width + offset);
                *slot = if ramping {
                    lane.step[index]
                } else {
                    lane.start[index]
                };
            }
            L::load(&values[..width])
        };
        let mut state = SoftClipState::from_lanes(load(false, 0), load(false, 1), load(false, 2));
        let coef = SoftClipCoef {
            drive_step: load(true, 0),
            output_step: load(true, 1),
            mix_step: load(true, 2),
            bypass: no_bypass,
        };
        let mut history = SoftClipHistory::new(width);
        soft_clip_block::<L>(&mut block, FRAMES, &coef, &mut state, &mut history);
        for frame in 0..FRAMES {
            for offset in 0..width {
                words[offset] = block[frame * width + offset];
            }
            for offset in 0..width {
                out[(group * width + offset) * FRAMES + frame] = words[offset].to_bits();
            }
        }
    }
}

/// Pinned SHA-256 of each case's result words, little-endian, in case order.
///
/// A regression guard and the cross-target reference, not an oracle: what makes the values
/// *correct* is `tests/polyphase_identity.rs` against the frozen 63-tap graph and `tests/contract.rs`
/// against the `f64` model in `dsp-reference`. These pins were produced by the scalar
/// `Lane` instantiation on `x86_64` and are checked at all three widths and on both wasm legs.
pub const SOFT_CLIP_DIGESTS: [[u8; 32]; CASE_COUNT] = include!("corpus_digests.in");
