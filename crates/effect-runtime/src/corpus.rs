//! The cross-target determinism corpus for this crate's lane-generic functions.
//!
//! Each case evaluates one function over a fixed input set and yields one `u32` result word per
//! input point. Gate G5 (`tools/wasm-gates/tests/g5_native_corpus.rs`) hashes each case at every
//! width and compares it against [`D1_DIGESTS`] natively, and replays the identical corpus under
//! wasmtime, at the wasm `Simd4` and scalar backends, against these same pins. That is the whole of
//! the cross-target claim for the effect runtime: the dB curve a compressor rides, the follower it
//! rides it with, and the level conversions between them produce the same bits in a browser as on a
//! native host (D5).
//!
//! # Why the inputs are built from integers
//!
//! Every point is produced by integer arithmetic and exact conversions, so a target cannot differ
//! on the *inputs* and make a digest mismatch look like a numerics bug. The generator is a
//! `xorshift64*`, which is defined entirely in terms of wrapping integer operations.
//!
//! # Why the domains are narrow
//!
//! A uniformly random `f32` bit pattern has a uniformly random exponent and is therefore almost
//! always astronomically large or small; a corpus built that way would feed the gain computer
//! nothing but its saturating arms. Each case samples the range its function does real work in —
//! `[-160, 24]` dB for a level, `[1e-9, 1e4]` for an amplitude — and the directed edge points that
//! sit exactly on a knee boundary are prepended so a knee-width change cannot hide in the noise.
//!
//! # No NaN payload
//!
//! The determinism claim excludes NaN payloads (wasm canonicalises the NaN an arithmetic operation
//! produces, master plan D5), so every case but one is NaN-free by construction: levels are finite,
//! amplitudes are positive normals, and `exp2_lane`/`log2_lane` clamp their arguments.
//!
//! The one exception is `ramp_toward` (issue #1473). Its contract includes that a NaN `next`
//! passes through rather than being hidden, and a lowering that hid it would otherwise be beyond
//! every pin. `current + step` with a NaN `step` is arithmetic, so its payload may differ by
//! target; the case therefore replaces every NaN result word with one fixed token,
//! [`NAN_TOKEN`], before it is hashed. Its pin depends on which points are NaN, never on a
//! payload. `tests/determinism.rs` checks that every other case is NaN-free and that this case
//! emits NaN only as the token.

use lane::Lane;
use lane::kernels::ramp_toward;

use crate::dynamics::{GainComputerCoef, gain_delta_db, gain_from_db, level_db};
use crate::envelope::{
    HysteresisCoef, HysteresisState, attack_release_coefficient, hysteresis_step, peak_follow,
    retention_coefficient, rms_follow,
};

/// Number of input points in every corpus case. A multiple of the widest backend.
pub const POINTS: usize = 1 << 16;

/// Number of corpus cases.
pub const CASE_COUNT: usize = 10;

/// Human-readable name of each case, indexed by case number.
pub const CASE_NAMES: [&str; CASE_COUNT] = [
    "gain_delta_db_hard_knee",
    "gain_delta_db_soft_knee",
    "gain_delta_db_expander",
    "level_db",
    "gain_from_db",
    "peak_follow",
    "rms_follow",
    "hysteresis_open",
    "hysteresis_hold",
    "ramp_toward",
];

/// The word that stands for every NaN result of the `ramp_toward` case: the canonical quiet NaN,
/// `0x7fc0_0000`. Only that case emits it (see "No NaN payload" above).
pub const NAN_TOKEN: u32 = 0x7fc0_0000;

/// The `ramp_toward` case's directed `current` values: both zeros, and finite values on either
/// side of them.
const RAMP_CURRENT: [f32; 5] = [0.0, -0.0, 1.0, -1.0, 0.25];

/// The `ramp_toward` case's directed `step` values: both zeros (so `next` keeps or changes the
/// sign of a zero `current`), steps that land inside, on and beyond a unit endpoint, and NaN, from
/// which `next` is NaN. The NaN is built from its bits, so every target reads the same input, and
/// its payload is not [`NAN_TOKEN`]'s: a native `current + step` keeps it, so a case that hashed
/// payloads instead of the token would fail `tests/determinism.rs` natively, not only under wasm.
const RAMP_STEP: [f32; 9] = [
    0.0,
    -0.0,
    1.0,
    -1.0,
    0.5,
    -0.5,
    2.0,
    -2.0,
    f32::from_bits(0x7fd0_1473),
];

/// The `ramp_toward` case's directed `target` values: both zeros, so `current` and `target` can be
/// zeros of opposite sign, and finite values that put `next` inside, on and beyond the endpoint.
const RAMP_TARGET: [f32; 6] = [0.0, -0.0, 1.0, -1.0, 0.25, 0.75];

/// Directed points of the `ramp_toward` case: every `(current, step, target)` triple of the three
/// tables above.
const RAMP_DIRECTED: usize = RAMP_CURRENT.len() * RAMP_STEP.len() * RAMP_TARGET.len();

/// `xorshift64*`. Integer-only, so every target builds the same sequence.
struct Rng(u64);

impl Rng {
    const fn new(case: usize) -> Self {
        Self(0x9e37_79b9_7f4a_7c15 ^ ((case as u64).wrapping_mul(0x0002_5a2f_1c3d_0b41) | 1))
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

/// Levels in dB that sit exactly on a threshold, a knee edge or a domain limit.
const EDGE_DB: [f32; 16] = [
    0.0,
    -0.0,
    -18.0,
    -21.0,
    -15.0,
    -24.0,
    -12.0,
    -160.0,
    24.0,
    -96.0,
    -6.0,
    -30.0,
    -36.0,
    -34.0,
    -20.999_999,
    -17.000_001,
];

/// One level in dB in `[-160, 24]`, from a raw integer draw.
fn level_point(rng: &mut Rng, index: usize) -> f32 {
    if index < EDGE_DB.len() {
        return EDGE_DB[index];
    }
    // 184_001 values at a milli-dB step covers [-160, 24] inclusive; `steps as f32` is exact.
    let steps = (rng.next() % 184_001) as u32;
    -160.0 + steps as f32 * 0.001
}

/// One `(current, step, target)` point of the `ramp_toward` case, from a raw integer draw.
///
/// The first [`RAMP_DIRECTED`] points are the directed triples, where operand order decides the
/// result bits. The rest are ordinary points on a `2^-16` grid: `current` and `target` in
/// `[-2, 2]`, and a `step` in `[-1, 1]` that leaves `next` inside the range on some points and
/// beyond the target on others, so the clamp and the unclamped law both carry the digest.
fn ramp_point(rng: &mut Rng, index: usize) -> [f32; 3] {
    if index < RAMP_DIRECTED {
        let target = index % RAMP_TARGET.len();
        let step = (index / RAMP_TARGET.len()) % RAMP_STEP.len();
        let current = index / (RAMP_TARGET.len() * RAMP_STEP.len());
        return [RAMP_CURRENT[current], RAMP_STEP[step], RAMP_TARGET[target]];
    }
    // 262_145 and 131_073 grid points: every value is an integer below `2^24` times `2^-16`, and
    // the offset is exact, so the conversion is exact on every target.
    let mut grid = |points: u64| (rng.next() % points) as u32 as f32 * (1.0 / 65_536.0);
    let current = grid(262_145) - 2.0;
    let step = grid(131_073) - 1.0;
    let target = grid(262_145) - 2.0;
    [current, step, target]
}

/// One positive normal amplitude in roughly `[2^-30, 2^13]`, from a raw integer draw.
///
/// Built by choosing an exponent field and a mantissa directly, so the sample is uniform over
/// *octaves* rather than over the real line — which is how audio amplitudes are actually
/// distributed, and where `log2_lane` has to be right.
fn amplitude_point(rng: &mut Rng, index: usize) -> f32 {
    if index < 4 {
        return [1.0, 0.5, 2.0, f32::MIN_POSITIVE][index];
    }
    let draw = rng.next();
    let exponent = 97 + (draw % 44) as u32;
    let mantissa = ((draw >> 16) as u32) & 0x007f_ffff;
    f32::from_bits((exponent << 23) | mantissa)
}

/// Runs one corpus case, writing [`POINTS`] result words to `out`.
///
/// Every case is a **pure function of its per-point inputs**: no case carries a recurrence across
/// points, because a recurrence at width `W` would run `W` interleaved sub-sequences and its
/// digest would depend on the width. The follower and hysteresis cases therefore take their
/// previous state as an input draw, which is what a follower's numerics actually depend on. As a
/// result the words do not depend on `L`, and one pinned digest serves every backend and every
/// target.
///
/// # Panics
///
/// Panics if `out.len()` is not [`POINTS`], or if `case` is not below [`CASE_COUNT`].
pub fn run_case<L: Lane>(case: usize, out: &mut [u32]) {
    assert!(case < CASE_COUNT, "corpus case out of range");
    assert_eq!(out.len(), POINTS, "corpus output must be POINTS long");
    let mut rng = Rng::new(case);
    match case {
        0..=2 => {
            let coefficients: GainComputerCoef<L> = match case {
                0 => GainComputerCoef::new(-18.0, 4.0, 0.0),
                1 => GainComputerCoef::new(-18.0, 4.0, 6.0),
                _ => GainComputerCoef::new(-40.0, 0.5, 12.0),
            };
            map_case::<L, _, _>(
                out,
                |index| [level_point(&mut rng, index), 0.0, 0.0],
                |x, _, _| gain_delta_db(x, &coefficients),
            );
        }
        3 => map_case::<L, _, _>(
            out,
            |index| [amplitude_point(&mut rng, index), 0.0, 0.0],
            |x, _, _| level_db(x),
        ),
        4 => map_case::<L, _, _>(
            out,
            |index| [level_point(&mut rng, index), 0.0, 0.0],
            |x, _, _| gain_from_db(x),
        ),
        5 => {
            let coefficient = L::splat(retention_coefficient(50.0, 48_000));
            map_case::<L, _, _>(
                out,
                |index| {
                    [
                        amplitude_point(&mut rng, index),
                        amplitude_point(&mut rng, index + POINTS),
                        0.0,
                    ]
                },
                |x, y, _| peak_follow(x.abs(), y, coefficient),
            );
        }
        6 => {
            let coefficient = L::splat(attack_release_coefficient(10.0, 48_000));
            map_case::<L, _, _>(
                out,
                |index| {
                    [
                        amplitude_point(&mut rng, index),
                        amplitude_point(&mut rng, index + POINTS),
                        0.0,
                    ]
                },
                |x, y, _| rms_follow(x.mul(x), y, coefficient),
            );
        }
        7 | 8 => {
            let coefficients = HysteresisCoef {
                open_db: L::splat(-30.0),
                close_db: L::splat(-36.0),
                hold_samples: L::splat(7.0),
            };
            let want_hold = case == 8;
            map_case::<L, _, _>(
                out,
                |index| {
                    let draw = rng.next();
                    [
                        level_point(&mut rng, index),
                        (draw & 1) as u32 as f32,
                        ((draw >> 8) % 9) as u32 as f32,
                    ]
                },
                |level, open, hold| {
                    let mut state = HysteresisState { open, hold };
                    let opened = hysteresis_step(level, &coefficients, &mut state);
                    if want_hold { state.hold } else { opened }
                },
            );
        }
        9 => {
            map_case::<L, _, _>(
                out,
                |index| ramp_point(&mut rng, index),
                |current, step, target| ramp_toward(current, step, target),
            );
            // D5: a NaN's payload is not part of the claim, only that the point is NaN. An integer
            // test on the bits, so no target's float semantics decide it.
            for word in out.iter_mut() {
                if *word & 0x7fff_ffff > 0x7f80_0000 {
                    *word = NAN_TOKEN;
                }
            }
        }
        _ => unreachable!(),
    }
}

/// Evaluates one case block by block: fill `L::WIDTH` inputs, apply, store the result words.
fn map_case<L: Lane, P, F>(out: &mut [u32], mut point: P, mut apply: F)
where
    P: FnMut(usize) -> [f32; 3],
    F: FnMut(L, L, L) -> L,
{
    let width = L::WIDTH;
    let mut first = [0.0f32; 32];
    let mut second = [0.0f32; 32];
    let mut third = [0.0f32; 32];
    let mut bits = [0u32; 32];
    let mut index = 0;
    while index < POINTS {
        for offset in 0..width {
            let values = point(index + offset);
            first[offset] = values[0];
            second[offset] = values[1];
            third[offset] = values[2];
        }
        apply(
            L::load(&first[..width]),
            L::load(&second[..width]),
            L::load(&third[..width]),
        )
        .store_bits(&mut bits[..width]);
        out[index..index + width].copy_from_slice(&bits[..width]);
        index += width;
    }
}

/// Pinned SHA-256 of each case's result words, little-endian, in case order.
///
/// A regression guard and the cross-target reference, not an oracle: what makes the values
/// *correct* is `tests/dynamics.rs` against the `f64` form of equation 4, `tests/envelope.rs`
/// against an `f64` one-pole, and, for `ramp_toward`, `lane`'s `tests/ramp_endpoint.rs`, which
/// states the clamp's in-range and NaN bits directly. These pins were produced by this crate on `x86_64` and are checked
/// at all three widths.
pub const D1_DIGESTS: [[u8; 32]; CASE_COUNT] = [
    // gain_delta_db_hard_knee
    [
        0x21, 0xc9, 0x21, 0x3b, 0xe1, 0x5b, 0x5c, 0x31, 0xe1, 0x80, 0x93, 0xbd, 0xd1, 0x15, 0x25,
        0x3e, 0xcb, 0xb7, 0xaa, 0x48, 0x1e, 0xd8, 0x7d, 0xf7, 0x67, 0xf9, 0x74, 0x5c, 0x0f, 0x49,
        0xb8, 0xe4,
    ],
    // gain_delta_db_soft_knee
    [
        0x4c, 0x64, 0x73, 0x0a, 0x3a, 0xf4, 0x19, 0x0e, 0x25, 0x5f, 0x5a, 0x07, 0x19, 0x44, 0xb5,
        0xf5, 0xfb, 0xf0, 0xd7, 0x68, 0x6b, 0x54, 0xc1, 0xc8, 0x82, 0xf1, 0x49, 0x7c, 0xf1, 0xe1,
        0x6f, 0x18,
    ],
    // gain_delta_db_expander
    [
        0x28, 0x98, 0xb2, 0xb5, 0xb9, 0x62, 0x17, 0x14, 0x43, 0x54, 0x94, 0xc1, 0x9b, 0x2e, 0x00,
        0xf8, 0xcc, 0x5b, 0x56, 0xdf, 0x00, 0x0d, 0x72, 0xe1, 0x96, 0xe9, 0x57, 0x63, 0xb9, 0x55,
        0xfa, 0x2b,
    ],
    // level_db
    [
        0xc7, 0x57, 0xe8, 0x3d, 0x38, 0xd5, 0xcf, 0x74, 0xae, 0xa6, 0xff, 0x5c, 0x2b, 0xfb, 0xa6,
        0x6b, 0xbc, 0xb0, 0xa4, 0x6f, 0xdc, 0xb7, 0xf9, 0x6b, 0x60, 0xa3, 0x73, 0x5c, 0xea, 0x02,
        0xe4, 0x60,
    ],
    // gain_from_db
    [
        0xa4, 0xe0, 0xab, 0x18, 0x31, 0xd0, 0x70, 0x6b, 0xac, 0x5a, 0xd0, 0xee, 0x1b, 0x24, 0xc2,
        0x67, 0xff, 0x43, 0x8e, 0x6b, 0x6b, 0xf7, 0x53, 0x16, 0x2a, 0xa4, 0xa2, 0x7d, 0x95, 0xc7,
        0x65, 0x01,
    ],
    // peak_follow
    [
        0x61, 0x3e, 0x39, 0x41, 0xeb, 0xc2, 0x60, 0xfc, 0x5b, 0x95, 0x63, 0x9b, 0xf5, 0xe7, 0xde,
        0x6b, 0x1e, 0x2d, 0x74, 0x5c, 0x31, 0xe6, 0xd1, 0xfb, 0x01, 0x4e, 0xc8, 0x53, 0x20, 0x78,
        0xe3, 0x60,
    ],
    // rms_follow
    [
        0xcd, 0xa0, 0x1b, 0x83, 0x09, 0xe9, 0x2b, 0x95, 0xca, 0xe2, 0xee, 0xae, 0x81, 0x77, 0x73,
        0x5d, 0x7f, 0x49, 0x6e, 0x63, 0x03, 0xeb, 0x96, 0x6f, 0xeb, 0x97, 0xc1, 0xe5, 0xc1, 0x21,
        0x67, 0xe7,
    ],
    // hysteresis_open
    [
        0xb6, 0x29, 0x91, 0xfa, 0xc9, 0x4a, 0x86, 0xb9, 0xa8, 0x02, 0x0c, 0xa9, 0xca, 0xc9, 0xaa,
        0x90, 0x21, 0xe7, 0x39, 0x22, 0x97, 0xfd, 0xe3, 0xca, 0xae, 0xdc, 0xfd, 0x72, 0x40, 0xe4,
        0x9e, 0x3b,
    ],
    // hysteresis_hold
    [
        0x7d, 0x19, 0xbd, 0xe4, 0xc6, 0x83, 0xc8, 0xe2, 0xd7, 0x2f, 0x76, 0x12, 0xca, 0xd1, 0x84,
        0xed, 0x8e, 0x1f, 0xc7, 0x12, 0x54, 0x7d, 0xa9, 0xca, 0x95, 0x5b, 0xc9, 0x52, 0x8a, 0xad,
        0xaa, 0xce,
    ],
    // ramp_toward
    [
        0xae, 0x13, 0x40, 0x91, 0x5e, 0x1d, 0x83, 0xc3, 0xb9, 0xe1, 0x34, 0xd2, 0x4f, 0x4a, 0x3a,
        0xe9, 0x62, 0x95, 0x46, 0x70, 0x72, 0x86, 0x7a, 0xe8, 0xe9, 0xf6, 0xad, 0xf6, 0x97, 0xf2,
        0x4d, 0xe6,
    ],
];
