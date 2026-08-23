//! The frozen cross-target corpus of this crate (gate E9).
//!
//! One definition, run three ways: natively at `WIDTH` 1, 4 and 8 by `tests/determinism.rs`, and
//! again inside a WebAssembly module by `tools/miso-engine-wasm-gate-corpus`, which references
//! [`E9_DIGESTS`] rather than copying it. Master plan #83 D5 claims a rendered block is
//! bit-identical across backends and across targets; this is the parametric EQ's half of that
//! claim, and the pins come from the scalar `Lane` oracle (master plan §8).
//!
//! # Why the byte stream is width independent
//!
//! A case is [`LANES`] independent single-track EQ configurations over [`FRAMES`] frames. At width
//! `W` the corpus runs in `LANES / W` groups of one AoSoA block and is read back **lane major**
//! before hashing, so the digest describes the arithmetic and not the layout. Every lane carries its
//! own four-band configuration, its own signal and — in the ramped case — its own ramp length, so a
//! cross-lane leak or a mis-split segment cannot pass as agreement.
//!
//! The once-per-block boundary check is deliberately **not** in the corpus: it inspects a whole
//! AoSoA block and would therefore couple lanes that the rest of the path keeps independent. Its
//! behaviour is a per-block property proven by `tests/faults.rs`, not a cross-target digest.
//!
//! # No NaN
//!
//! D5 excludes NaN payloads because wasm canonicalises them. Every case is bounded by construction —
//! stable sections, inputs in `[-1, 1]` — and `tests/determinism.rs` asserts finiteness rather than
//! assuming it.

use crate::{BandTarget, Channel, EQ_SECTION_COUNT_V1, EqBandKindV1, RAMP_SAMPLES, SampleRateHz};
use miso_engine_lane::{Lane, Simd4, Simd8};

/// Independent single-track configurations in every case; a multiple of the widest backend.
pub const LANES: usize = 8;

/// Frames per track: long enough for the four-section cascade to settle and for a 64-sample word
/// ramp to finish and snap well inside the case.
pub const FRAMES: usize = 512;

/// Result words per case: one per track per frame, lane major.
pub const POINTS: usize = LANES * FRAMES;

/// Number of corpus cases.
pub const CASE_COUNT: usize = 3;

/// Human-readable name of each case, indexed by case number.
pub const CASE_NAMES: [&str; CASE_COUNT] = [
    "cascade/noise",
    "cascade/ramped_noise",
    "cascade/impulse_subnormal_state",
];

/// The sample rate every case designs at.
const CORPUS_RATE: SampleRateHz = SampleRateHz(48_000);

/// The four-band configuration of one track, spread across the frozen parameter domain.
fn bands(track: usize) -> [BandTarget; EQ_SECTION_COUNT_V1] {
    const KINDS: [EqBandKindV1; EQ_SECTION_COUNT_V1] = [
        EqBandKindV1::Bell,
        EqBandKindV1::LowShelf,
        EqBandKindV1::HighPass,
        EqBandKindV1::Notch,
    ];
    const FREQUENCIES: [f32; LANES] = [
        20.0, 60.0, 180.0, 540.0, 1_620.0, 4_860.0, 14_580.0, 19_000.0,
    ];
    const GAINS: [f32; LANES] = [-24.0, -12.0, -6.0, -1.5, 1.5, 6.0, 12.0, 24.0];
    const QS: [f32; LANES] = [0.1, 0.25, 0.5, 0.70710677, 1.0, 2.5, 7.0, 18.0];
    const SLOPES: [f32; 4] = [0.1, 0.4, 0.7, 1.0];
    core::array::from_fn(|section| BandTarget {
        enabled: true,
        kind: KINDS[section],
        frequency: FREQUENCIES[(track + section * 2) % LANES],
        gain: GAINS[(track + section * 3) % LANES],
        q: QS[(track + section * 5) % LANES],
        slope: SLOPES[(track + section) % SLOPES.len()],
    })
}

/// The band a ramped case re-targets, and the parameters it ramps to. Both endpoints are frozen
/// grid points, so the whole ramp stays inside the parameter domain by construction.
fn ramp_target(track: usize, section: usize) -> BandTarget {
    let mut target = bands(track)[section];
    let shifted = bands((track + 3) % LANES)[section];
    target.frequency = shifted.frequency;
    target.gain = -target.gain;
    target.q = shifted.q;
    target
}

/// `xorshift64*`. Integer only, so every target builds the same signal.
struct Rng(u64);

impl Rng {
    const fn new(seed: u64) -> Self {
        Self(seed | 1)
    }

    fn next(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        (x.wrapping_mul(0x2545_f491_4f6c_dd1d) >> 32) as u32
    }
}

/// Fills one track's input signal.
fn fill(case: usize, track: usize, lane: &mut [f32; FRAMES]) {
    let mut random = Rng::new(0xA5A5_5A5A_1234_0001 ^ (track as u64).wrapping_mul(0x9E37_79B9));
    for (frame, sample) in lane.iter_mut().enumerate() {
        *sample = if case == 2 {
            if frame == 0 { 1.0 } else { 0.0 }
        } else {
            f32::from((random.next() >> 16) as u16) * (2.0 / 65_536.0) - 1.0
        };
    }
}

/// Runs one corpus case at one width and writes `POINTS` result words, lane major.
///
/// `L` selects the width; the case list, the configurations, the signals and the operation order are
/// the pin.
///
/// # Panics
///
/// Panics if `case >= CASE_COUNT` or `out.len() != POINTS`.
pub fn run_case<L: Lane>(case: usize, out: &mut [u32]) {
    assert!(case < CASE_COUNT, "corpus case index out of range");
    assert_eq!(out.len(), POINTS, "corpus output length");
    match L::WIDTH {
        1 => run::<f32, 1>(case, out),
        4 => run::<Simd4, 4>(case, out),
        _ => run::<Simd8, 8>(case, out),
    }
}

fn run<L: Lane, const W: usize>(case: usize, out: &mut [u32]) {
    let mut lanes = [[0.0_f32; FRAMES]; LANES];
    for (track, lane) in lanes.iter_mut().enumerate() {
        fill(case, track, lane);
    }
    let mut block = vec![0.0_f32; FRAMES * W];
    for group in 0..LANES / W {
        let targets = core::array::from_fn(|lane| bands(group * W + lane));
        let mut channel =
            Channel::<L, W>::new(targets, CORPUS_RATE).expect("every corpus row is a legal design");
        if case == 1 {
            for lane in 0..W {
                let track = group * W + lane;
                let section = track % EQ_SECTION_COUNT_V1;
                let words = ramp_target(track, section)
                    .words(CORPUS_RATE)
                    .expect("every corpus ramp target is a legal design");
                channel.start_ramp(section, lane, words);
                // Staggered ends, so the block is cut in a different place for every lane and the
                // segment split itself is inside the digest.
                channel.remaining[section][lane] = RAMP_SAMPLES - 7 * track as u32;
            }
        }
        if case == 2 {
            for section in &mut channel.sections {
                section.state.ic1 = L::splat(1.0e-40);
                section.state.ic2 = L::splat(-1.0e-41);
            }
        }
        for frame in 0..FRAMES {
            for offset in 0..W {
                block[frame * W + offset] = lanes[group * W + offset][frame];
            }
        }
        channel.process_block(&mut block, FRAMES);
        for frame in 0..FRAMES {
            for offset in 0..W {
                lanes[group * W + offset][frame] = block[frame * W + offset];
            }
        }
    }
    for (track, lane) in lanes.iter().enumerate() {
        for (frame, sample) in lane.iter().enumerate() {
            out[track * FRAMES + frame] = sample.to_bits();
        }
    }
}

/// SHA-256 of each case's result words, generated once from the scalar `Lane` oracle on `x86_64`
/// and frozen (master plan §8: a pin comes from an oracle, never from copying production output).
///
/// A mismatch is never fixed by re-pinning. It means the corpus changed, or a backend or a target
/// stopped agreeing with the scalar instantiation — which is what this gate exists to catch.
pub const E9_DIGESTS: [[u8; 32]; CASE_COUNT] = [
    // cascade/noise
    [
        0x3a, 0x19, 0xa7, 0x67, 0x08, 0x80, 0x4b, 0xcf, 0xcc, 0xb4, 0xb2, 0x56, 0xa4, 0xc5, 0xd1,
        0x82, 0xd1, 0x2e, 0x39, 0xc9, 0x00, 0x3c, 0x22, 0x69, 0x90, 0x81, 0x6f, 0x5c, 0x41, 0xef,
        0xac, 0x4d,
    ],
    // cascade/ramped_noise
    [
        0x54, 0xa2, 0xae, 0x22, 0xf1, 0x40, 0x64, 0x89, 0xee, 0x62, 0x28, 0x68, 0xea, 0x2d, 0xd6,
        0x17, 0x73, 0x05, 0x00, 0xe4, 0xde, 0x69, 0x3e, 0xc3, 0x6e, 0x1f, 0xa4, 0x2a, 0xae, 0xe4,
        0xd3, 0x52,
    ],
    // cascade/impulse_subnormal_state
    [
        0x99, 0x32, 0xef, 0x28, 0x06, 0x69, 0x8d, 0x41, 0x6c, 0x76, 0x55, 0xe5, 0xc7, 0x98, 0xd1,
        0x5a, 0x89, 0x39, 0x62, 0x2c, 0x76, 0x5b, 0xbe, 0x04, 0x1d, 0x0f, 0xe5, 0x7d, 0x5b, 0xb6,
        0x7d, 0xc8,
    ],
];
