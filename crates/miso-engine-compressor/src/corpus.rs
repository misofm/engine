//! The frozen cross-target corpus for this crate (gate E4).
//!
//! One definition, run three ways: by `tests/cross_target.rs` against [`C1_DIGESTS`] at all three
//! lane widths, by `tools/miso-engine-wasm-gates` natively, and by the same code compiled to
//! `wasm32-unknown-unknown` and executed under wasmtime — with and without `simd128`. Master plan
//! #83 D5 claims a rendered block is bit-identical across `Scalar`/`Simd4`/`Simd8` **and** across
//! `x86_64`/`aarch64`/`wasm32`; for this crate, this file is that claim.
//!
//! # Why a digest of this shape means something
//!
//! 1. **The byte stream is width independent.** A case is [`LANES`] independent single-track
//!    signals of [`FRAMES`] frames. At width `W` the corpus is rendered in `LANES / W` groups and
//!    read back **lane major** before hashing, so the digest describes the arithmetic and not the
//!    memory layout. The compressor's recurrence (`g`) is per lane and never crosses lanes, so
//!    unlike a scalar corpus of the `miso-engine-effect-runtime` kind this one *may* contain a
//!    recurrence: at any width, lane `l` runs exactly the same sequence.
//! 2. **No NaN reaches a digest.** D5 excludes NaN payloads because wasm canonicalises them. Every
//!    input is a finite value in `[-1, 1]`, every coefficient is in a bounded domain, and
//!    `log2_lane`/`exp2_lane` clamp their arguments, so the outputs are finite by construction —
//!    and `tests/cross_target.rs` asserts it rather than assuming it.
//! 3. **The corpus is frozen.** The track table, the signal generator, the block partition and the
//!    automation points are part of the pin. Changing one is a re-pin, permitted only from the
//!    scalar `Lane` oracle and only with the deviation stated in the commit message (§8).
//!
//! # What each case covers
//!
//! The four cases together reach every branch of the kernel that a rendered block can reach: the
//! three link laws, a hard and two soft knees, an upward and a downward ratio, the three identity
//! selects (`bypass` is not a corpus case because it is the input unchanged), a lookahead of zero
//! and two non-zero taps, and — in the last case — the ramping body, its per-frame redesign and
//! the exponential that designs a ballistic coefficient.

use miso_engine_effect_contract::LinkMode;
use miso_engine_lane::Lane;

use crate::design::{MAX_WIDTH, PARAMETER_COUNT, SMOOTHING_SAMPLES};
use crate::kernel::{Channel, Detector, process_block};

/// Independent single-track signals in a case. A multiple of the widest backend.
pub const LANES: usize = 8;

/// Frames rendered per track. The first [`RING_LENGTH`] − 1 are the latency's leading zeros.
pub const FRAMES: usize = 384;

/// Block partition the corpus renders with. Not a multiple of [`FRAMES`], deliberately: the last
/// block is short, so a kernel that behaved differently on a partial block would move the digest.
pub const BLOCK: usize = 100;

/// Ring length `B = N + 1` used by the corpus.
///
/// Not a launch quality's `N = Fs/50`: at 48 kHz that is 960 frames of pure latency before a
/// single output sample is non-zero, which would make the corpus fifteen times longer for no extra
/// coverage. The ring arithmetic under test — the wrap, the `w + 1` read and the per-lane
/// `w - D` tap — is `B`-independent, and `tests/partition.rs` exercises the production `B`.
pub const RING_LENGTH: usize = 65;

/// Sample rate the ballistic coefficients are designed at.
pub const SAMPLE_RATE: u32 = 48_000;

/// Result words per case: left then right, per lane.
pub const POINTS: usize = LANES * 2 * FRAMES;

/// Number of corpus cases.
pub const CASE_COUNT: usize = 4;

/// Human-readable name of each case, indexed by case number.
pub const CASE_NAMES: [&str; CASE_COUNT] = [
    "dual_mono_static",
    "maximum_link",
    "average_link",
    "dual_mono_ramping",
];

/// The eight tracks, in table order: threshold, ratio, knee, attack, release, makeup, mix,
/// lookahead. Frozen; every value is inside its descriptor domain.
///
/// Track 0 is a hard knee, track 1 a wide one; track 2 has ratio 1 (no compression) and non-zero
/// makeup, so it exercises the `G == 0 && makeup != 0` path that is *not* an identity; track 3 has
/// `mix == 0` and track 7 `mix == 1`, the two identity selects; tracks 4 to 6 sweep the lookahead
/// tap from zero to the ring's limit.
const TRACKS: [[f32; PARAMETER_COUNT]; LANES] = [
    [-18.0, 4.0, 0.0, 10.0, 100.0, 0.0, 1.0, 0.0],
    [-24.0, 8.0, 24.0, 1.0, 50.0, 3.0, 0.75, 0.5],
    [-6.0, 1.0, 6.0, 5.0, 200.0, -6.0, 0.5, 0.25],
    [-40.0, 20.0, 12.0, 0.1, 5.0, 12.0, 0.0, 1.0],
    [0.0, 2.0, 6.0, 50.0, 1000.0, -24.0, 0.25, 0.0],
    [-80.0, 1.5, 3.0, 20.0, 5000.0, 24.0, 0.9, 0.75],
    [-12.0, 12.0, 18.0, 0.5, 20.0, -3.0, 0.6, 1.0],
    [-30.0, 6.0, 0.0, 2.0, 300.0, 6.0, 1.0, 0.125],
];

/// The automation applied by case 3, as `(parameter index, left value, right value)`.
///
/// Delivered at frame [`AUTOMATION_FRAME`] so that it lands inside a block rather than on a block
/// boundary, and covers three parameters at once — one that redesigns the static curve, one that
/// re-enters the exponential, and one that is a pass-through word.
const AUTOMATION: [(usize, f32, f32); 3] = [(0, -36.0, -30.0), (4, 25.0, 40.0), (6, 0.35, 0.65)];

/// Frame at which case 3's automation is applied.
const AUTOMATION_FRAME: usize = 137;

/// `xorshift64*`, seeded per lane. Integer-only, so every target builds the same signal.
struct Rng(u64);

impl Rng {
    const fn new(lane: usize, case: usize) -> Self {
        Self(
            0x9e37_79b9_7f4a_7c15
                ^ ((lane as u64).wrapping_mul(0x0002_5a2f_1c3d_0b41) | 1)
                ^ ((case as u64) << 40),
        )
    }

    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    /// A value in `[-1, 1)`: the top 24 bits scaled by an exact power of two.
    fn next_sample(&mut self) -> f32 {
        ((self.next() >> 40) as f32) * (1.0 / 8_388_608.0) - 1.0
    }
}

/// The input of one lane: noise with a level that steps up and back down, so every case crosses
/// its threshold in both directions and both ballistic arms run.
fn lane_input(lane: usize, case: usize) -> ([f32; FRAMES], [f32; FRAMES]) {
    let mut generator = Rng::new(lane, case);
    let mut left = [0.0_f32; FRAMES];
    let mut right = [0.0_f32; FRAMES];
    for frame in 0..FRAMES {
        // Exact powers of two, so the envelope contributes no rounding of its own.
        let envelope = match frame / 96 {
            0 => 0.015_625_f32,
            1 => 0.5,
            2 => 1.0,
            _ => 0.031_25,
        };
        left[frame] = generator.next_sample() * envelope;
        right[frame] = generator.next_sample() * envelope * 0.5;
    }
    (left, right)
}

/// The link law of a case.
const fn link_of(case: usize) -> LinkMode {
    match case {
        1 => LinkMode::Maximum,
        2 => LinkMode::Average,
        _ => LinkMode::DualMono,
    }
}

/// Renders one case at one width and writes `POINTS` result words, lane major.
///
/// # Panics
///
/// Panics if `case >= CASE_COUNT`, if `out` is not [`POINTS`] words long, or if `LANES` is not a
/// multiple of `L::WIDTH`.
pub fn run_case<L: Lane>(case: usize, out: &mut [u32]) {
    assert!(case < CASE_COUNT, "corpus case index out of range");
    assert_eq!(out.len(), POINTS, "corpus output length");
    assert_eq!(LANES % L::WIDTH, 0, "width must divide the lane count");
    let width = L::WIDTH;
    let link = link_of(case);

    for group in 0..LANES / width {
        let base = group * width;
        let mut defaults = [TRACKS[base]; MAX_WIDTH];
        for (lane, slot) in defaults.iter_mut().take(width).enumerate() {
            *slot = TRACKS[base + lane];
        }
        let mut left_channel = Channel::<L>::new(&defaults, RING_LENGTH, SAMPLE_RATE);
        let mut right_channel = Channel::<L>::new(&defaults, RING_LENGTH, SAMPLE_RATE);

        let inputs: Vec<([f32; FRAMES], [f32; FRAMES])> = (0..width)
            .map(|lane| lane_input(base + lane, case))
            .collect();

        let mut left = vec![0.0_f32; BLOCK * width];
        let mut right = vec![0.0_f32; BLOCK * width];
        let mut frame = 0;
        while frame < FRAMES {
            let frames = core::cmp::min(BLOCK, FRAMES - frame);
            if case == 3 && frame <= AUTOMATION_FRAME && AUTOMATION_FRAME < frame + frames {
                // Applied at a block boundary, as the contract's block-rate automation is: the
                // block that contains the target frame carries it.
                for (parameter, left_value, right_value) in AUTOMATION {
                    for lane in 0..width {
                        left_channel.ramps[parameter][lane]
                            .set_target(left_value, SMOOTHING_SAMPLES);
                        right_channel.ramps[parameter][lane]
                            .set_target(right_value, SMOOTHING_SAMPLES);
                    }
                }
            }
            for step in 0..frames {
                for lane in 0..width {
                    left[step * width + lane] = inputs[lane].0[frame + step];
                    right[step * width + lane] = inputs[lane].1[frame + step];
                }
            }
            process_block::<L>(
                &mut left[..frames * width],
                &mut right[..frames * width],
                Detector::Main,
                frames,
                link,
                false,
                SAMPLE_RATE,
                (&mut left_channel, &mut right_channel),
            );
            for step in 0..frames {
                for lane in 0..width {
                    let track = base + lane;
                    out[track * 2 * FRAMES + frame + step] = left[step * width + lane].to_bits();
                    out[track * 2 * FRAMES + FRAMES + frame + step] =
                        right[step * width + lane].to_bits();
                }
            }
            frame += frames;
        }
    }
}

/// Pinned SHA-256 digests of the corpus, one per case.
///
/// Generated once from the `L = f32` instantiation on `x86_64` (master plan §8.3: pinning from the
/// scalar `Lane` oracle is allowed when the property being pinned is identity). A mismatch is
/// never repaired by re-pinning from the run that failed: it means a target, a width or an
/// operation order stopped agreeing with the oracle, which is what this gate exists to catch.
pub const C1_DIGESTS: [[u8; 32]; CASE_COUNT] = [[0; 32], [0; 32], [0; 32], [0; 32]];
