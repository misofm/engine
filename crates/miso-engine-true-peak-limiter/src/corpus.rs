//! The cross-target determinism corpus for the limiter kernel (master plan gate G5).
//!
//! Each case runs [`limiter_block`](crate::limiter_block) over [`LANES`] independent stereo tracks
//! for [`FRAMES`] frames and yields one `u32` result word per output sample, **lane major**: left
//! channel of lane 0, then lane 1, and so on, then the right channel the same way. At width `W`
//! the corpus is processed in `LANES / W` groups of an AoSoA block and read back lane major before
//! hashing, so a digest describes the arithmetic and not the layout, and `W = 1`, `4` and `8` must
//! produce the same 32 bytes on every target.
//!
//! `tools/miso-engine-wasm-gate-corpus` replays these cases under wasmtime, with and without
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
//! # No NaN
//!
//! The determinism claim excludes NaN payloads (wasm canonicalises them). Every case is driven with
//! finite input inside the ranges the boundary check accepts, and the crate's determinism test
//! asserts the outputs are finite rather than assuming it.

use miso_engine_lane::Lane;

use crate::{ChannelState, Cursors, LimiterCoef, PARAMETER_COUNT, Shape, limiter_block};

/// Independent tracks in every case; a multiple of the widest backend.
pub const LANES: usize = 8;

/// Frames per track: past the 486-sample latency at 48 kHz, so most of the digest is real audio.
pub const FRAMES: usize = 1024;

/// Result words per case: both channels of every lane.
pub const POINTS: usize = 2 * LANES * FRAMES;

/// The rate every case runs at.
const RATE: u32 = 48_000;

/// Number of corpus cases.
pub const CASE_COUNT: usize = 5;

/// Human-readable name of each case, indexed by case number.
pub const CASE_NAMES: [&str; CASE_COUNT] = [
    "limiter/noise_dual_mono",
    "limiter/noise_linked",
    "limiter/near_nyquist_w_min",
    "limiter/impulse_train_long_lookahead",
    "limiter/subnormal_release_flush",
];

/// The pinned digest of each case, generated from the scalar `Lane` instantiation.
///
/// Re-pinning is permitted only from that oracle and only with the deviation stated in the commit
/// message (master plan §8). A wasm mismatch is never fixed by re-pinning: it means a target
/// stopped agreeing with the scalar law, which is the whole reason the gate exists.
pub const D90_DIGESTS: [[u8; 32]; CASE_COUNT] = [
    // limiter/noise_dual_mono
    [
        0x50, 0x58, 0x50, 0x2b, 0x14, 0xa4, 0xad, 0x0b, 0x26, 0x84, 0xae, 0xec, 0xbe, 0x2f, 0xe5,
        0xb9, 0x73, 0x9d, 0xe2, 0x3e, 0xc3, 0x11, 0xa6, 0x6b, 0xd5, 0xf0, 0x4f, 0x7d, 0xd8, 0x41,
        0x9a, 0x12,
    ],
    // limiter/noise_linked
    [
        0x97, 0x83, 0xa0, 0x8e, 0x15, 0x78, 0x5a, 0x35, 0x96, 0x4a, 0xd1, 0xb5, 0x81, 0xcd, 0xc0,
        0xe4, 0x86, 0xb9, 0x63, 0x2e, 0x8c, 0x30, 0xfa, 0x74, 0xdc, 0x20, 0x31, 0xb4, 0x65, 0xda,
        0xc9, 0x62,
    ],
    // limiter/near_nyquist_w_min
    [
        0x68, 0x99, 0x38, 0xbc, 0x6e, 0x8e, 0x5f, 0xa4, 0xd0, 0xb4, 0x0f, 0x25, 0x9a, 0xe4, 0xbe,
        0xbf, 0xf4, 0xb0, 0x22, 0x1a, 0xf7, 0x0f, 0x71, 0xd2, 0xe0, 0x70, 0x74, 0x39, 0xb1, 0xb0,
        0x68, 0x72,
    ],
    // limiter/impulse_train_long_lookahead
    [
        0x95, 0x6c, 0xa8, 0xf3, 0x31, 0x6a, 0x53, 0x37, 0xfd, 0xd0, 0xd3, 0x70, 0x17, 0xcc, 0x47,
        0xae, 0x25, 0x90, 0x76, 0xf8, 0xc3, 0x09, 0x73, 0x89, 0xa4, 0xfb, 0xfd, 0x54, 0x11, 0xda,
        0xe6, 0x20,
    ],
    // limiter/subnormal_release_flush
    [
        0x54, 0x6b, 0x4a, 0x87, 0x98, 0x61, 0xe7, 0x8d, 0x96, 0x22, 0x62, 0x9b, 0x0a, 0x14, 0xb1,
        0x42, 0x93, 0xca, 0x05, 0x80, 0x2e, 0x25, 0x30, 0x46, 0xe2, 0x79, 0xb5, 0x58, 0xba, 0x6a,
        0xe0, 0x26,
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

    let shape = Shape::new(RATE).expect("launch rate");
    let coefficients = LimiterCoef::<L>::new(case == 1, false);
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
        let mut left_state = ChannelState::new(width, &shape, &defaults, RATE);
        let mut right_state = ChannelState::new(width, &shape, &defaults, RATE);
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
