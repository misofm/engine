//! The frozen cross-target corpus for this effect.
//!
//! Master plan #83 D5 claims a rendered block is bit-identical across `Scalar`/`Simd4`/`Simd8`
//! **and** across `x86_64`/`aarch64`/`wasm32`. This module is the transient shaper's contribution
//! to that claim: one definition, digested natively by this crate's own test and replayed under
//! wasmtime by `tools/miso-engine-wasm-gates` through `miso-engine-wasm-gate-corpus`, against the
//! pins in [`CROSS_TARGET_DIGESTS`], which live here rather than in the tool so that the gate and
//! the pins it replays cannot drift apart.
//!
//! # Why the words are width independent
//!
//! A transient shaper is a recurrence, so a case cannot be a pure function of its inputs the way
//! the effect-runtime corpus's cases are. Instead a case is [`LANES`] **independent tracks**: at
//! width `W` the corpus is rendered in `LANES / W` groups of an AoSoA block and read back
//! lane-major, so the word stream describes the arithmetic and not the layout. `W = 1`, `4` and `8`
//! must therefore produce the same words, which is exactly the lane-identity claim.
//!
//! # What each case covers
//!
//! One case per link mode, each with per-lane parameters that reach every branch of the law: a
//! bypassing `mix = 0` lane, unity and partial mixes, both signs of both shape amounts, `-0.0` in
//! the first frame (the signed-zero identity contract), impulses, a long decay into the follower's
//! release, and a subnormal input. Every case retargets all three parameters at frame 0, so the
//! D11 ramp prefix runs; blocks are 128 frames, so the prefix spans a block boundary.
//!
//! # No NaN
//!
//! The determinism claim excludes NaN payloads (wasm canonicalises them). Every input here is
//! finite and every output is checked for finiteness by the gate.

use miso_engine_lane::{Lane, Simd4, Simd8};

use crate::{
    PARAMETER_COUNT, RAMP_SAMPLES, Shaper, TRANSIENT_SHAPER_DESCRIPTOR_V1, coefficient_row,
};
use miso_engine_effect_contract::{
    EffectQuality, LatencySamples, LinkMode, PreparedEffectMetadata, PreparedPortsV1,
    PreparedSidechainPort, StatePayloadSizes, TailSamples,
};

/// Independent tracks in every case; a multiple of the widest backend.
pub const LANES: usize = 8;

/// Frames rendered per track: four 128-frame blocks, so the 64-frame ramp prefix spans a boundary.
pub const FRAMES: usize = 512;

/// Frames per render block.
pub const BLOCK: usize = 128;

/// Result words emitted per track: both channels' output, then the four envelope state words.
pub const WORDS_PER_LANE: usize = 2 * FRAMES + 4;

/// Total result words of one case.
pub const WORDS: usize = LANES * WORDS_PER_LANE;

/// Number of corpus cases.
pub const CASE_COUNT: usize = 3;

/// Human-readable name of each case, indexed by case number.
pub const CASE_NAMES: [&str; CASE_COUNT] = [
    "transient_shaper/dual_mono",
    "transient_shaper/maximum",
    "transient_shaper/average",
];

/// The widths every case is digested at.
pub const WIDTHS: [usize; 3] = [1, 4, 8];

/// SHA-256 of each case's word stream, pinned from the scalar `Lane` instantiation.
///
/// Master plan §8: a fixture is regenerated only from an independent oracle or from the scalar
/// `Lane` instantiation when the property is lane identity. This is the latter — the pin is what
/// `Simd4`, `Simd8`, wasm and AArch64 must reproduce.
pub const CROSS_TARGET_DIGESTS: [[u8; 32]; CASE_COUNT] = [
    [
        0xa9, 0xc4, 0x11, 0x4c, 0x90, 0x61, 0x9c, 0xf3, 0xda, 0xfa, 0x60, 0x1a, 0xbb, 0x1e, 0xb3,
        0xc1, 0x15, 0x28, 0xe9, 0x58, 0x93, 0x8b, 0x99, 0x7a, 0x32, 0x23, 0x67, 0xc2, 0xa0, 0x49,
        0xc0, 0x63,
    ],
    [
        0x95, 0x07, 0xfa, 0x98, 0x96, 0xaa, 0x59, 0x06, 0xb5, 0x58, 0x25, 0x05, 0x75, 0xab, 0x9d,
        0xa4, 0x5d, 0xd3, 0xe3, 0x20, 0x03, 0x64, 0x70, 0x7f, 0xa9, 0xf1, 0xd5, 0x70, 0xf1, 0x0b,
        0xd9, 0xcb,
    ],
    [
        0x92, 0xa1, 0xd2, 0x81, 0x1f, 0x40, 0x52, 0xbb, 0x87, 0x8e, 0x27, 0x05, 0x61, 0xe1, 0x3c,
        0x91, 0xff, 0x19, 0xf3, 0xa0, 0xa9, 0xd3, 0xf2, 0xdf, 0x83, 0xbc, 0x1f, 0xa0, 0xe9, 0x36,
        0xbb, 0x0e,
    ],
];

/// Per-track attack amounts, in `[-1, 1]`.
const ATTACK: [f32; LANES] = [1.0, 0.75, 0.5, 0.25, 0.0, -0.25, -0.5, -1.0];

/// Per-track sustain amounts, in `[-1, 1]`.
const SUSTAIN: [f32; LANES] = [-1.0, -0.5, 0.0, 0.5, 1.0, 0.75, -0.75, 0.25];

/// Per-track wet mixes, in `[0, 1]`. Track 4 is a `mix = 0` identity lane.
const MIX: [f32; LANES] = [1.0, 0.75, 0.5, 0.25, 0.0, 1.0, 0.5, 0.25];

/// Per-track automation targets applied at frame 0, so every ramp runs its 64-frame prefix.
const RETARGET: [f32; PARAMETER_COUNT] = [-0.625, 0.375, 0.875];

/// `xorshift64*`, integer only, so every target builds the same signal.
struct Rng(u64);

impl Rng {
    const fn new(seed: u64) -> Self {
        Self(0x9e37_79b9_7f4a_7c15 ^ (seed | 1))
    }

    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    /// A value in `[-0.5, 0.5)` built by integer arithmetic and one exact conversion.
    fn sample(&mut self) -> f32 {
        ((self.next() >> 40) as i32 - 8_388_608 / 2) as f32 / 16_777_216.0
    }
}

/// One track's stereo signal.
///
/// Frame 0 is `-0.0` (the signed-zero identity row), frames 1 and 2 are subnormal, every 64th frame
/// is a full-scale impulse, the last quarter is an exponential decay into the followers' release,
/// and the rest is integer-built noise. The right channel uses a second draw so the link modes are
/// not degenerate.
fn signal(track: usize, channel: usize) -> Vec<f32> {
    let mut rng = Rng::new((track as u64) << 8 | channel as u64);
    let mut decay = 1.0_f32;
    (0..FRAMES)
        .map(|index| {
            let noise = rng.sample();
            match index {
                0 => -0.0,
                1 => f32::from_bits(1),
                2 => f32::from_bits(0x0040_0000),
                _ if index % 64 == 0 => {
                    if channel == 0 {
                        1.0
                    } else {
                        0.25
                    }
                }
                _ if index >= FRAMES * 3 / 4 => {
                    decay *= 0.99;
                    decay * (0.5 + noise)
                }
                _ => noise * (1.0 + track as f32 * 0.125),
            }
        })
        .collect()
}

/// Runs one case at one width, writing [`WORDS`] result words to `out`, lane-major.
///
/// # Panics
///
/// Panics if `case` is not below [`CASE_COUNT`], if `width` is not one of [`WIDTHS`], or if
/// `out.len()` is not [`WORDS`].
pub fn run_case(case: usize, width: usize, out: &mut [u32]) {
    assert!(case < CASE_COUNT, "corpus case out of range");
    assert_eq!(out.len(), WORDS, "corpus output must be WORDS long");
    match width {
        1 => run::<f32, 1>(case, out),
        4 => run::<Simd4, 4>(case, out),
        8 => run::<Simd8, 8>(case, out),
        _ => panic!("corpus width must be 1, 4 or 8"),
    }
}

/// A prepared-metadata value for the corpus: the launch descriptor at 48 kHz, one link mode per
/// case, never bypassed.
fn metadata(case: usize) -> PreparedEffectMetadata {
    let quality = TRANSIENT_SHAPER_DESCRIPTOR_V1.qualities[1];
    PreparedEffectMetadata {
        descriptor: &TRANSIENT_SHAPER_DESCRIPTOR_V1,
        sample_rate: 48_000,
        quantum: BLOCK as u32,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: match case {
            0 => LinkMode::DualMono,
            1 => LinkMode::Maximum,
            _ => LinkMode::Average,
        },
        ports: PreparedPortsV1 {
            sidechain: PreparedSidechainPort::None,
        },
        latency: LatencySamples(0),
        tail: TailSamples::Finite(0),
        state_sizes: StatePayloadSizes {
            common_bytes: 0,
            left_bytes: crate::LANE_STATE_BYTES,
            right_bytes: crate::LANE_STATE_BYTES,
        },
        scratch_bytes: quality.scratch_fixed_bytes,
        automation_capacity: 16,
    }
}

fn run<L: Lane, const W: usize>(case: usize, out: &mut [u32]) {
    let row = coefficient_row(48_000).expect("launch rate");
    let signals: Vec<(Vec<f32>, Vec<f32>)> = (0..LANES)
        .map(|track| (signal(track, 0), signal(track, 1)))
        .collect();
    for group in 0..LANES / W {
        let base = group * W;
        let mut defaults = [[0.0_f32; PARAMETER_COUNT]; W];
        for (lane, values) in defaults.iter_mut().enumerate() {
            *values = [ATTACK[base + lane], SUSTAIN[base + lane], MIX[base + lane]];
        }
        let mut shaper = Shaper::<L, W>::new(metadata(case), row, defaults, defaults);
        for lane in 0..W {
            for (index, target) in RETARGET.into_iter().enumerate() {
                shaper.left.ramps[lane][index].set_target(target, RAMP_SAMPLES);
                shaper.right.ramps[lane][index].set_target(-target, RAMP_SAMPLES);
            }
        }
        let mut left = vec![0.0_f32; BLOCK * W];
        let mut right = vec![0.0_f32; BLOCK * W];
        for block in 0..FRAMES / BLOCK {
            for frame in 0..BLOCK {
                for lane in 0..W {
                    left[frame * W + lane] = signals[base + lane].0[block * BLOCK + frame];
                    right[frame * W + lane] = signals[base + lane].1[block * BLOCK + frame];
                }
            }
            shaper.process_block(&mut left, &mut right, BLOCK);
            for frame in 0..BLOCK {
                for lane in 0..W {
                    let slot = (base + lane) * WORDS_PER_LANE + block * BLOCK + frame;
                    out[slot] = left[frame * W + lane].to_bits();
                    out[slot + FRAMES] = right[frame * W + lane].to_bits();
                }
            }
        }
        let mut words = [0.0_f32; 8];
        for (envelope, offset) in [
            (shaper.left_env.fast, 0),
            (shaper.left_env.slow, 1),
            (shaper.right_env.fast, 2),
            (shaper.right_env.slow, 3),
        ] {
            envelope.store(&mut words);
            for lane in 0..W {
                out[(base + lane) * WORDS_PER_LANE + 2 * FRAMES + offset] = words[lane].to_bits();
            }
        }
    }
}
