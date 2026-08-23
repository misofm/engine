//! The cross-target determinism corpus for this crate's lane-generic arithmetic.
//!
//! `tests/cross_target_digest.rs` hashes each case and compares it against [`DIGESTS`];
//! `tools/miso-engine-wasm-gate-corpus` replays the identical cases under wasmtime, at the wasm
//! scalar and `simd128` backends, against these same pins. Together that is the cross-target half
//! of decision D5 for this effect: the crossover's fused all-pass tap, the D7 flush of its six
//! recursive words, the lane `log2`/`exp2` its detector and its makeup ride, the branching
//! smoother and the detector link all produce the same bits in a browser as on a native host.
//!
//! # No recurrence across points
//!
//! Every case is a **pure function of its per-point inputs**. A recurrence at width `W` runs `W`
//! interleaved sub-sequences, so its digest would depend on the width and one pin could not serve
//! every backend. The filter and smoother cases therefore take their previous state as an input
//! draw, which is what their numerics actually depend on. The *composition* — rings, per-track
//! detector taps, segment splitting — is not here; it is proven width-independent natively by
//! `tests/identity.rs`, which compares whole rendered blocks at `WIDTH = 1`, 4 and 8 by `to_bits`.
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

use miso_engine_effect_runtime::envelope::retention_coefficient;
use miso_engine_lane::Lane;
use miso_engine_lane::kernels::SvfState;

use crate::shim::{LINK_AVERAGE, LINK_MAXIMUM, branching_smooth, link_levels};
use crate::{BandCoef, Lr4State, band_amplitude, lr4_coefficients, lr4_step};

/// Input points in every case. A multiple of the widest backend.
pub const POINTS: usize = 1 << 14;

/// Number of cases.
pub const CASE_COUNT: usize = 6;

/// Human-readable name of each case, indexed by case number.
pub const CASE_NAMES: [&str; CASE_COUNT] = [
    "lr4_step/low",
    "lr4_step/high",
    "band_amplitude",
    "branching_smooth",
    "link_levels/maximum",
    "link_levels/average",
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
                    let (low, band) = lr4_step(values[0], &coefficients, &mut state);
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

/// Pinned SHA-256 of each case's result words, little-endian, in case order.
///
/// Generated once from the **scalar** `Lane` instantiation on `x86_64` (master plan §8.3: a pin
/// comes from the oracle, never from a SIMD or a wasm run) and checked at all three widths and on
/// every target. A mismatch is never fixed by re-pinning: it means either the corpus changed or a
/// target stopped agreeing with the oracle, which is what this gate exists to catch.
pub const DIGESTS: [[u8; 32]; CASE_COUNT] = include!("corpus_digests.in");
