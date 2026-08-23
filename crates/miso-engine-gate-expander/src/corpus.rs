//! The gate's frozen cross-target corpus.
//!
//! `tools/miso-engine-wasm-gate-corpus` delegates to this module the way it delegates to
//! `miso-engine-math`'s M3 corpus and `miso-engine-effect-runtime`'s D1 corpus: the pins live here,
//! next to the code they describe, and the cross-target gate replays them rather than carrying a
//! transcription that could drift from the gate it is meant to replay.
//!
//! # Why a digest is width independent
//!
//! A case is [`LANES`] *independent* single-lane signals of [`FRAMES`] frames. At width `W` the
//! corpus is run as `LANES / W` groups of an AoSoA block and read back lane-major before hashing,
//! so the digest describes the arithmetic and not the layout. The gate's recurrences run along the
//! frame axis inside one lane and never across lanes, which is what makes one pin serve every
//! width.
//!
//! # What the cases cover
//!
//! Each case drives the production [`gate_block`] through a whole signal at one link mode, with
//! per-lane thresholds, ratios, ranges, hysteresis bands, hold times and lookaheads, so a case
//! exercises the gather, the transition, the curve, both one-pole rates and the identity select.
//! No case produces a NaN: master plan D5 excludes NaN payloads because wasm canonicalises them.

use crate::kernel::{
    GateArgs, GateCoef, GateRamp, GateRing, GateState, MAX_WIDTH, RAMP_COUNT, gate_block,
};
use miso_engine_lane::Lane;

/// Independent single-lane signals in every case; a multiple of the widest backend.
pub const LANES: usize = 8;

/// Frames per signal: long enough for the hold to expire and the release to settle several times.
pub const FRAMES: usize = 1024;

/// Fixed latency of the corpus, in samples. A short ring keeps the wasm leg quick while still
/// exercising the wrap of a power-of-two slot count.
pub const DELAY: u32 = 48;

/// Ring slots, `(DELAY + 1).next_power_of_two()`.
const SLOTS: usize = 64;

/// Number of frozen cases.
pub const CASE_COUNT: usize = 6;

/// Case names, in pin order.
pub const CASE_NAMES: [&str; CASE_COUNT] = [
    "dual_mono/noise",
    "maximum/noise",
    "average/noise",
    "dual_mono/bursts",
    "dual_mono/subnormal",
    "dual_mono/ramping",
];

/// Result words per case: two channels of [`LANES`] signals.
pub const POINTS: usize = 2 * LANES * FRAMES;

/// A tiny xorshift, so the corpus needs no random-number dependency.
struct Xorshift(u64);

impl Xorshift {
    fn new(seed: u64) -> Self {
        Self(seed | 1)
    }

    fn next_u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        (x >> 32) as u32
    }
}

/// Fills one lane's input signal for a case.
fn signal(case: usize, lane: usize, channel: usize, out: &mut [f32]) {
    let mut random =
        Xorshift::new(0x51ED_2701 ^ ((case as u64) << 32) ^ ((lane * 2 + channel) as u64));
    for (frame, sample) in out.iter_mut().enumerate() {
        let noise = f32::from((random.next_u32() >> 16) as u16) * (2.0 / 65_536.0) - 1.0;
        *sample = match case {
            // Bursts: 64 frames of signal, 64 of near-silence, so the hold and both one-pole
            // rates are exercised in every lane.
            3 => {
                if (frame / 64) % 2 == 0 {
                    noise * 0.5
                } else {
                    noise * 1.0e-4
                }
            }
            // Subnormal input: the flush band has to remove the same bits on every target.
            4 => f32::from_bits((random.next_u32() & 0x007F_FFFF) | 1),
            _ => noise * 0.25,
        };
    }
}

/// The per-lane parameters of a case, chosen so no two lanes share a decision boundary.
fn parameters(lane: usize, channel: usize) -> [f32; 8] {
    let bias = lane as f32 + channel as f32 * 0.5;
    [
        -60.0 + bias * 3.0, // threshold
        1.5 + bias * 0.75,  // ratio
        12.0 + bias * 4.0,  // range
        1.0 + bias * 0.5,   // hysteresis
        0.5 + bias * 0.25,  // attack ms
        0.1 + bias * 0.05,  // hold ms
        8.0 + bias * 2.0,   // release ms
        0.0,                // lookahead ms, converted to a tap below
    ]
}

/// Builds one channel's coefficients and initial state for a case.
fn prepare<L: Lane>(case: usize, group: usize, channel: usize) -> (GateCoef<L>, GateState<L>) {
    let width = L::WIDTH;
    let mut attack = [0.0_f32; MAX_WIDTH];
    let mut release = [0.0_f32; MAX_WIDTH];
    let mut hold = [0.0_f32; MAX_WIDTH];
    let mut resting = [[0.0_f32; MAX_WIDTH]; RAMP_COUNT];
    for offset in 0..width {
        let lane = group * width + offset;
        let values = parameters(lane, channel);
        attack[offset] =
            miso_engine_effect_runtime::envelope::attack_release_coefficient(values[4], 48_000);
        release[offset] =
            miso_engine_effect_runtime::envelope::attack_release_coefficient(values[6], 48_000);
        hold[offset] = (values[5] * 48.0 + 0.5).floor();
        for (index, slot) in resting.iter_mut().enumerate() {
            slot[offset] = values[index];
        }
    }
    let link = |mode: usize| L::splat(f32::from(u8::from(case == mode)));
    let coef = GateCoef {
        attack: L::load(&attack[..width]),
        release: L::load(&release[..width]),
        hold_samples: L::load(&hold[..width]),
        bypass: L::zero(),
        link_max: link(1),
        link_avg: link(2),
    };
    let mut state = GateState {
        gain_db: L::zero(),
        hysteresis: miso_engine_effect_runtime::envelope::HysteresisState {
            open: L::splat(1.0),
            hold: coef.hold_samples,
        },
        ramps: [GateRamp::fixed(L::zero()); RAMP_COUNT],
    };
    for (index, ramp) in state.ramps.iter_mut().enumerate() {
        let current = L::load(&resting[index][..width]);
        *ramp = GateRamp::fixed(current);
        if case == 5 {
            // The ramping case retargets every parameter by a fixed offset over 64 samples,
            // through the same precomputed-step law the control plane uses.
            ramp.target = current.add(L::splat(1.0));
            ramp.step = L::splat(1.0).div(L::splat(64.0));
            ramp.remaining = L::splat(64.0);
        }
    }
    (coef, state)
}

/// Runs one case at width `L::WIDTH` and writes the result bits, lane-major.
///
/// # Panics
///
/// Panics if `case >= CASE_COUNT` or if `out` is shorter than [`POINTS`].
pub fn run_case<L: Lane>(case: usize, out: &mut [u32]) {
    assert!(case < CASE_COUNT, "corpus case index out of range");
    assert!(out.len() >= POINTS, "corpus output too short");
    let width = L::WIDTH;
    let groups = LANES / width;
    for group in 0..groups {
        let mut left = vec![0.0_f32; FRAMES * width];
        let mut right = vec![0.0_f32; FRAMES * width];
        for offset in 0..width {
            let lane = group * width + offset;
            let mut own = vec![0.0_f32; FRAMES];
            signal(case, lane, 0, &mut own);
            for (frame, sample) in own.iter().enumerate() {
                left[frame * width + offset] = *sample;
            }
            signal(case, lane, 1, &mut own);
            for (frame, sample) in own.iter().enumerate() {
                right[frame * width + offset] = *sample;
            }
        }
        let (coef_left, mut state_left) = prepare::<L>(case, group, 0);
        let (coef_right, mut state_right) = prepare::<L>(case, group, 1);
        let mut main_left = vec![0.0_f32; SLOTS * width];
        let mut main_right = vec![0.0_f32; SLOTS * width];
        let mut empty_left: Vec<f32> = Vec::new();
        let mut empty_right: Vec<f32> = Vec::new();
        let mut tap_left = [0_u32; MAX_WIDTH];
        let mut tap_right = [0_u32; MAX_WIDTH];
        for offset in 0..width {
            let lane = group * width + offset;
            // Lookahead 0, 6, 12, ... samples, so every lane taps a different slot.
            tap_left[offset] = DELAY - (lane as u32 * 6).min(DELAY);
            tap_right[offset] = DELAY - ((lane as u32 * 6 + 3).min(DELAY));
        }
        let mut cursor = 0_u32;
        if case == 5 {
            gate_block::<L, false, true>(GateArgs {
                left: &mut left[..64 * width],
                right: &mut right[..64 * width],
                sidechain: None,
                frames: 64,
                coef: (&coef_left, &coef_right),
                state: (&mut state_left, &mut state_right),
                rings: (
                    GateRing {
                        main: &mut main_left,
                        detector: &mut empty_left,
                        tap: &tap_left,
                    },
                    GateRing {
                        main: &mut main_right,
                        detector: &mut empty_right,
                        tap: &tap_right,
                    },
                ),
                cursor: &mut cursor,
                slot_mask: (SLOTS - 1) as u32,
                delay: DELAY,
            });
        }
        let done = if case == 5 { 64 } else { 0 };
        gate_block::<L, false, false>(GateArgs {
            left: &mut left[done * width..],
            right: &mut right[done * width..],
            sidechain: None,
            frames: FRAMES - done,
            coef: (&coef_left, &coef_right),
            state: (&mut state_left, &mut state_right),
            rings: (
                GateRing {
                    main: &mut main_left,
                    detector: &mut empty_left,
                    tap: &tap_left,
                },
                GateRing {
                    main: &mut main_right,
                    detector: &mut empty_right,
                    tap: &tap_right,
                },
            ),
            cursor: &mut cursor,
            slot_mask: (SLOTS - 1) as u32,
            delay: DELAY,
        });
        for offset in 0..width {
            let lane = group * width + offset;
            for frame in 0..FRAMES {
                out[lane * FRAMES + frame] = left[frame * width + offset].to_bits();
                out[(LANES + lane) * FRAMES + frame] = right[frame * width + offset].to_bits();
            }
        }
    }
}

/// SHA-256 of every case, generated once from the scalar `Lane` oracle and frozen.
///
/// Master plan #83 §8: a pin comes from the oracle, never from copying production output. A
/// mismatch is never fixed by re-pinning — it means either the corpus changed, or a target stopped
/// agreeing with the scalar instantiation, and the second is what the gate exists to catch.
pub const GATE_DIGESTS: [[u8; 32]; CASE_COUNT] = include!("gate_digests.in");
