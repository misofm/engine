//! Gate M1 of issue #950: the banked full meter pass is the scalar meter's loop, bit for bit.
//!
//! `meter_block` computes, per lane of one AoSoA block, the sanitized sample peak, the clipped and
//! sanitized counts, and the energy sum carried sample by sample from a caller-supplied seed. Each
//! vector width is held against two references, lane by lane, every field by bits (the counts
//! also as integers):
//!
//! 1. the same kernel at `Lane = f32` over the de-interleaved lane, the scalar `Lane` oracle of
//!    master plan §3.2; and
//! 2. an independent oracle written here, which is the builtin meter's `ALL` loop: sanitize with
//!    `normal_or_zero`, take the magnitude, the peak's `if a > p`, `e += f64(s) * f64(s)`, and the
//!    two counts.
//!
//! The energy is carried across 64 blocks -- each block's vector result is the next block's seed --
//! from random positive seeds, so an add performed in any order but the scalar loop's, or a seed
//! read from the wrong lane, would move a bit. `peak` must also equal the sample-peak kernel of
//! issue #943 seeded with `+0.0`. Every vector result passes through `black_box` before it is
//! compared: the vector and scalar forms are the same arithmetic, and without the barrier LLVM may
//! prove a comparison true and delete it.

#![allow(missing_docs)]

use core::hint::black_box;
#[cfg(target_feature = "avx2")]
use lane::Simd8;
use lane::kernels::builtins::{meter_block, meter_sample_peak_block};
use lane::{Lane, LaneF64, Simd4, Widen};

/// Xorshift64\*: seeded and portable, so every host sees the same inputs.
struct Rng(u64);

impl Rng {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }

    /// A random positive binary64 between `2^-64` and `2^64`: a running energy of any plausible
    /// size, so the first add of a block is rarely exact.
    fn positive_seed(&mut self) -> f64 {
        let exponent = 1023 - 64 + self.next_u64() % 129;
        f64::from_bits((exponent << 52) | (self.next_u64() & 0x000F_FFFF_FFFF_FFFF))
    }
}

/// `2^k` from the exponent field: exact on every host.
fn pow2(k: i32) -> f32 {
    f32::from_bits(((127 + k) as u32) << 23)
}

/// Issue #943's G1 hostile pool -- NaN payloads, both infinities, both zeros, the extreme
/// subnormals of both signs, `±MIN_POSITIVE`, `±1.0`, `±f32::MAX` and `±2^k` for `k` in
/// `-34..=5` -- plus the clip boundary's neighbours `1 ± ulp` of both signs, `±1e30` and `1e38`.
fn hostile_pool() -> Vec<f32> {
    let mut pool = vec![
        f32::NAN,
        -f32::NAN,
        f32::from_bits(0x7FC0_0001),
        f32::from_bits(0xFF80_0001),
        f32::INFINITY,
        f32::NEG_INFINITY,
        0.0,
        -0.0,
        f32::from_bits(0x0000_0001),
        f32::from_bits(0x8000_0001),
        f32::from_bits(0x007F_FFFF),
        f32::from_bits(0x807F_FFFF),
        f32::MIN_POSITIVE,
        -f32::MIN_POSITIVE,
        1.0,
        -1.0,
        f32::MAX,
        -f32::MAX,
        f32::from_bits(0x3F80_0001),
        f32::from_bits(0x3F7F_FFFF),
        f32::from_bits(0xBF80_0001),
        f32::from_bits(0xBF7F_FFFF),
        1.0e30,
        -1.0e30,
        1.0e38,
    ];
    for k in -34..=5 {
        pool.push(pow2(k));
        pool.push(-pow2(k));
    }
    pool
}

/// The builtin meter's `ALL` loop over one block, written out here rather than borrowed, so this
/// oracle does not share a line with the kernel.
#[derive(Clone, Copy)]
struct Oracle {
    peak: f32,
    energy: f64,
    clipped: u64,
    sanitized: u64,
}

impl Oracle {
    fn block(energy: f64, samples: &[f32]) -> Self {
        let mut oracle = Self {
            peak: 0.0,
            energy,
            clipped: 0,
            sanitized: 0,
        };
        for &x in samples {
            let valid = x.is_finite() && !x.is_subnormal();
            let s = if valid { x } else { 0.0 };
            let a = s.abs();
            if a > oracle.peak {
                oracle.peak = a;
            }
            oracle.energy += f64::from(s) * f64::from(s);
            oracle.sanitized += u64::from(!valid);
            oracle.clipped += u64::from(a >= 1.0);
        }
        oracle
    }
}

/// The input families M1 runs.
#[derive(Clone, Copy, Debug)]
enum Input {
    /// Two thirds from [`hostile_pool`], the rest a small per-lane tone.
    Hostile,
    /// A per-lane triangle tone whose amplitude cycles through 0.375, 0.75, 1.125 and 1.5 by lane
    /// and block, so every width sees clipped and unclipped blocks.
    Tone,
}

fn fill(rng: &mut Rng, pool: &[f32], words: &mut [f32], width: usize, block: usize, input: Input) {
    for (index, word) in words.iter_mut().enumerate() {
        let lane = index % width;
        let frame = index / width + block * 131;
        *word = match input {
            Input::Hostile if rng.below(3) != 0 => pool[rng.below(pool.len())],
            Input::Hostile => {
                let phase = (frame * 7) as f32;
                (0.125 + 0.0625 * lane as f32) * (((phase * 0.37) % 2.0) - 1.0)
            }
            Input::Tone => {
                let step = (frame * (3 + lane)) % 256;
                let triangle = (step as f32 / 64.0 - 2.0).abs() - 1.0;
                (0.375 * (1 + (lane + block) % 4) as f32) * triangle
            }
        };
    }
}

fn store<L: Lane>(value: L) -> Vec<f32> {
    let mut out = vec![0.0_f32; L::WIDTH];
    value.store(&mut out);
    out
}

fn store_f64<F: LaneF64>(value: F) -> Vec<f64> {
    let mut out = vec![0.0_f64; F::WIDTH];
    value.store(&mut out);
    out
}

/// An `f32` count as an integer, asserting it is one.
fn count(value: f32, context: &str) -> u64 {
    assert!(
        value >= 0.0 && value.fract() == 0.0,
        "{context}: count {value} is not an exact non-negative integer"
    );
    value as u64
}

/// Runs 64 carried blocks at each frame count and returns the number of lane-blocks compared.
fn identity_at<L: Widen>(seed: u64, input: Input) -> usize {
    let pool = hostile_pool();
    let width = L::WIDTH;
    let mut rng = Rng(seed);
    let mut compared = 0;
    let mut clipped_seen = 0_u64;
    let mut sanitized_seen = 0_u64;
    for frames in [1_usize, 2, 3, 127, 128, 129] {
        let seeds: Vec<f64> = (0..width).map(|_| rng.positive_seed()).collect();
        let mut vector_energy = seeds.clone();
        let mut scalar_energy = seeds.clone();
        let mut oracle_energy = seeds;
        for block in 0..64 {
            let mut words = vec![0.0_f32; frames * width];
            fill(&mut rng, &pool, &mut words, width, block, input);
            let result = black_box(meter_block::<L>(
                &words,
                frames,
                <L::F64 as LaneF64>::load(&vector_energy),
            ));
            let reference = black_box(meter_sample_peak_block::<L>(&words, frames, L::zero()));
            let peak = store(result.peak);
            let clipped = store(result.clipped);
            let sanitized = store(result.sanitized);
            let energy = store_f64(result.energy);
            let reference = store(reference);
            for lane in 0..width {
                let deinterleaved: Vec<f32> = (0..frames)
                    .map(|frame| words[frame * width + lane])
                    .collect();
                let scalar = meter_block::<f32>(&deinterleaved, frames, scalar_energy[lane]);
                let oracle = Oracle::block(oracle_energy[lane], &deinterleaved);
                let context =
                    format!("width {width} {input:?} frames {frames} block {block} lane {lane}");
                assert_eq!(
                    peak[lane].to_bits(),
                    reference[lane].to_bits(),
                    "{context}: peak against meter_sample_peak_block seeded with +0.0"
                );
                assert_eq!(
                    peak[lane].to_bits(),
                    scalar.peak.to_bits(),
                    "{context}: peak against the f32 kernel"
                );
                assert_eq!(
                    peak[lane].to_bits(),
                    oracle.peak.to_bits(),
                    "{context}: peak against the meter's serial loop"
                );
                assert_eq!(
                    clipped[lane].to_bits(),
                    scalar.clipped.to_bits(),
                    "{context}: clipped against the f32 kernel"
                );
                assert_eq!(
                    count(clipped[lane], &context),
                    oracle.clipped,
                    "{context}: clipped against the meter's serial loop"
                );
                assert_eq!(
                    sanitized[lane].to_bits(),
                    scalar.sanitized.to_bits(),
                    "{context}: sanitized against the f32 kernel"
                );
                assert_eq!(
                    count(sanitized[lane], &context),
                    oracle.sanitized,
                    "{context}: sanitized against the meter's serial loop"
                );
                assert_eq!(
                    energy[lane].to_bits(),
                    scalar.energy.to_bits(),
                    "{context}: energy against the f32 kernel"
                );
                assert_eq!(
                    energy[lane].to_bits(),
                    oracle.energy.to_bits(),
                    "{context}: energy against the meter's serial loop"
                );
                clipped_seen += oracle.clipped;
                sanitized_seen += oracle.sanitized;
                vector_energy[lane] = energy[lane];
                scalar_energy[lane] = scalar.energy;
                oracle_energy[lane] = oracle.energy;
                compared += 1;
            }
        }
    }
    assert!(
        clipped_seen > 0,
        "width {width} {input:?}: some sample clipped"
    );
    if matches!(input, Input::Hostile) {
        assert!(
            sanitized_seen > 0,
            "width {width} {input:?}: some word was sanitized"
        );
    }
    compared
}

#[test]
fn m1_banked_meter_block_is_the_scalar_meters_loop_at_every_width() {
    for input in [Input::Hostile, Input::Tone] {
        #[cfg(target_feature = "avx2")]
        assert_eq!(
            identity_at::<Simd8>(0x0950_9E37_79B9_7F4A, input),
            6 * 64 * 8
        );
        assert_eq!(
            identity_at::<Simd4>(0x0950_1234_5678_9ABC, input),
            6 * 64 * 4
        );
        assert_eq!(identity_at::<f32>(0x0950_DEAD_BEEF_CAFE, input), 6 * 64);
    }
}

/// The count boundaries, swept on the bits: every exponent of both signs, with the mantissas that
/// sit on each boundary, one frame per lane. `sanitized` must count exactly NaN, `±inf` and the
/// nonzero subnormals -- not the zeros -- and `clipped` exactly the magnitudes `>= 1.0`.
fn boundary_sweep_at<L: Widen>() {
    let mantissas = [0, 1, 2, 0x0040_0000, 0x007F_FFFE, 0x007F_FFFF];
    let mut values = Vec::new();
    for sign in [0_u32, 0x8000_0000] {
        for exponent in 0_u32..=255 {
            for mantissa in mantissas {
                values.push(f32::from_bits(sign | (exponent << 23) | mantissa));
            }
        }
    }
    while values.len() % L::WIDTH != 0 {
        values.push(0.0);
    }
    let zero = [0.0_f64; 8];
    for chunk in values.chunks_exact(L::WIDTH) {
        let result = black_box(meter_block::<L>(
            chunk,
            1,
            <L::F64 as LaneF64>::load(&zero[..L::WIDTH]),
        ));
        let peak = store(result.peak);
        let clipped = store(result.clipped);
        let sanitized = store(result.sanitized);
        let energy = store_f64(result.energy);
        for (lane, &x) in chunk.iter().enumerate() {
            let oracle = Oracle::block(0.0, &[x]);
            let context = format!("width {} input {:#010x}", L::WIDTH, x.to_bits());
            assert_eq!(peak[lane].to_bits(), oracle.peak.to_bits(), "{context}");
            assert_eq!(count(clipped[lane], &context), oracle.clipped, "{context}");
            assert_eq!(
                count(sanitized[lane], &context),
                oracle.sanitized,
                "{context}"
            );
            assert_eq!(energy[lane].to_bits(), oracle.energy.to_bits(), "{context}");
        }
    }
}

#[test]
fn m1_count_boundaries_match_the_meters_classification() {
    #[cfg(target_feature = "avx2")]
    boundary_sweep_at::<Simd8>();
    boundary_sweep_at::<Simd4>();
    boundary_sweep_at::<f32>();
}

/// The reassociation hazard the kernel refuses (issue #950, R3), witnessed on its own inputs: a
/// block's energy added to the carried seed as one zero-seeded partial is not, in general, the
/// seeded sample-serial sum. If this ever stopped failing somewhere, the carried-seed identity above
/// would be the weaker for it, so it is asserted to fail at least once.
#[cfg(target_feature = "avx2")]
#[test]
fn m1_a_zero_seeded_partial_plus_the_seed_is_not_the_seeded_sum() {
    let mut rng = Rng(0x0950_0000_0000_0001);
    let pool = hostile_pool();
    let mut differ = 0;
    for block in 0..64 {
        let mut words = vec![0.0_f32; 128 * 8];
        fill(&mut rng, &pool, &mut words, 8, block, Input::Tone);
        let seeds: Vec<f64> = (0..8).map(|_| rng.positive_seed().min(64.0)).collect();
        let seeded = store_f64(
            black_box(meter_block::<Simd8>(
                &words,
                128,
                <Simd8 as Widen>::F64::load(&seeds),
            ))
            .energy,
        );
        let partial = store_f64(
            black_box(meter_block::<Simd8>(
                &words,
                128,
                <Simd8 as Widen>::F64::load(&[0.0; 8]),
            ))
            .energy,
        );
        for lane in 0..8 {
            differ +=
                usize::from((seeds[lane] + partial[lane]).to_bits() != seeded[lane].to_bits());
        }
    }
    assert!(
        differ > 0,
        "the reassociated sum never differed, so it witnesses nothing"
    );
}
