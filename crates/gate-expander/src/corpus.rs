//! The gate's frozen cross-target corpus.
//!
//! `tools/wasm-gate-corpus` delegates to this module the way it delegates to
//! `math`'s M3 corpus and `effect-runtime`'s D1 corpus: the pins live here,
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
//! per-lane thresholds, ratios, ranges, hysteresis bands and hold times, so a case exercises the
//! current-sample detector, transition, curve, both one-pole rates and identity select.
//! No case produces a NaN: master plan D5 excludes NaN payloads because wasm canonicalises them.
//!
//! The last case ([`CLAMPED_CASE`], issue #1459) is the only one in which issue #1409's ramp clamp
//! acts: every ramped word of every lane starts [`CLAMPED_OFFSET_ULPS`] ulps from its resting value,
//! the offset of #1409's scan moves, and ramps back to it over a window whose length differs per
//! lane, so the unclamped law (`current + step`) would pass the target before the snap. The
//! clamped prologue (`ramp_toward`) holds the word at the target instead, and the digest records
//! that, so the wasm `simd128` leg replays the clamp inside a real effect render path.

use crate::kernel::{GateArgs, GateCoef, GateRamp, GateState, MAX_WIDTH, RAMP_COUNT, gate_block};
use lane::Lane;

/// Independent single-lane signals in every case; a multiple of the widest backend.
pub const LANES: usize = 8;

/// Frames per signal: long enough for the hold to expire and the release to settle several times.
pub const FRAMES: usize = 1024;

/// Number of frozen cases.
pub const CASE_COUNT: usize = 7;

/// Case names, in pin order.
pub const CASE_NAMES: [&str; CASE_COUNT] = [
    "dual_mono/noise",
    "maximum/noise",
    "average/noise",
    "dual_mono/bursts",
    "dual_mono/subnormal",
    "dual_mono/ramping",
    "dual_mono/clamped_ramp",
];

/// The case in which issue #1409's ramp clamp acts (module doc).
pub const CLAMPED_CASE: usize = 6;

/// Frames the ramping cases render through the ramping prologue, as one block.
const RAMP_FRAMES: usize = 64;

/// How far, in ulps of the resting word, each word of [`CLAMPED_CASE`] starts from its target: the
/// offset of #1409's scan moves (for example the gate threshold from `0xc29fffdf` to `-80.0`).
pub const CLAMPED_OFFSET_ULPS: u32 = 0x21;

/// The ramp window, in samples, of one lane and channel of [`CLAMPED_CASE`]: a different frame per
/// lane and channel, every one inside the [`RAMP_FRAMES`]-frame ramping block. The words are
/// [`CLAMPED_OFFSET_ULPS`] ulps from their targets, so every window here is short enough that the
/// step is over half an ulp (each unclamped addition rounds to a whole ulp) and long enough that
/// the word reaches its target before the snap and would then pass it.
#[must_use]
pub const fn clamped_window(lane: usize, channel: usize) -> u32 {
    (40 + 3 * lane + channel) as u32
}

/// One ramped word of [`CLAMPED_CASE`]: `(start, target, step, samples)`, the step derived by the
/// control plane's own law ([`effect_runtime::ramp::LinearRamp::set_target`]).
#[must_use]
pub fn clamped_move(lane: usize, channel: usize, word: usize) -> (f32, f32, f32, u32) {
    let target = parameters(lane, channel)[word];
    // Adding to the bits moves away from zero, inside the target's binade for every resting word.
    let start = f32::from_bits(target.to_bits() + CLAMPED_OFFSET_ULPS);
    let samples = clamped_window(lane, channel);
    let mut ramp = effect_runtime::ramp::LinearRamp::fixed(start);
    ramp.set_target(target, samples);
    (start, ramp.target, ramp.step, ramp.remaining)
}

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
            // Quiet input, a power of two below each lane's re-arm level (`threshold -
            // hysteresis`), so the gate closes after its hold and the expansion curve, which reads
            // threshold, ratio and range, sets the gain while the clamp acts.
            6 => noise * quiet_peak(lane, channel),
            _ => noise * 0.25,
        };
    }
}

/// The [`CLAMPED_CASE`] input peak of one lane: the power of two at or below `-64 + 2.5 * bias`
/// dB, which is 3 dB under the lane's re-arm level. Exact on every target: one IEEE division, one
/// floor and an exponent built from bits, no transcendental function.
fn quiet_peak(lane: usize, channel: usize) -> f32 {
    let bias = lane as f32 + channel as f32 * 0.5;
    let peak_db = -64.0 + bias * 2.5;
    let exponent = (peak_db / 6.020_6).floor() as i32;
    f32::from_bits(((127 + exponent) as u32) << 23)
}

/// The per-lane parameters of a case, chosen so no two lanes share a decision boundary.
fn parameters(lane: usize, channel: usize) -> [f32; 7] {
    let bias = lane as f32 + channel as f32 * 0.5;
    [
        -60.0 + bias * 3.0, // threshold
        1.5 + bias * 0.75,  // ratio
        12.0 + bias * 4.0,  // range
        1.0 + bias * 0.5,   // hysteresis
        0.5 + bias * 0.25,  // attack ms
        0.1 + bias * 0.05,  // hold ms
        8.0 + bias * 2.0,   // release ms
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
        attack[offset] = effect_runtime::envelope::attack_release_coefficient(values[4], 48_000);
        release[offset] = effect_runtime::envelope::attack_release_coefficient(values[6], 48_000);
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
        hysteresis: effect_runtime::envelope::HysteresisState {
            open: L::splat(1.0),
            hold: coef.hold_samples,
        },
        ramps: [GateRamp::fixed(L::zero()); RAMP_COUNT],
    };
    for (index, ramp) in state.ramps.iter_mut().enumerate() {
        let current = L::load(&resting[index][..width]);
        *ramp = GateRamp::fixed(current);
        if case == CLAMPED_CASE {
            let mut words = [[0.0_f32; MAX_WIDTH]; 4];
            for offset in 0..width {
                let (start, target, step, samples) =
                    clamped_move(group * width + offset, channel, index);
                for (slot, value) in words.iter_mut().zip([start, target, step, samples as f32]) {
                    slot[offset] = value;
                }
            }
            *ramp = GateRamp {
                current: L::load(&words[0][..width]),
                target: L::load(&words[1][..width]),
                step: L::load(&words[2][..width]),
                remaining: L::load(&words[3][..width]),
            };
        }
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
        let ramping = case == 5 || case == CLAMPED_CASE;
        if ramping {
            gate_block::<L, false, true>(GateArgs {
                left: &mut left[..RAMP_FRAMES * width],
                right: &mut right[..RAMP_FRAMES * width],
                sidechain: None,
                frames: RAMP_FRAMES,
                coef: (&coef_left, &coef_right),
                state: (&mut state_left, &mut state_right),
            });
        }
        let done = if ramping { RAMP_FRAMES } else { 0 };
        gate_block::<L, false, false>(GateArgs {
            left: &mut left[done * width..],
            right: &mut right[done * width..],
            sidechain: None,
            frames: FRAMES - done,
            coef: (&coef_left, &coef_right),
            state: (&mut state_left, &mut state_right),
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Issue #1459 gate 1: in [`CLAMPED_CASE`], every ramped word of every lane and channel is a
    /// move whose unclamped law passes its target before the snap and inside the ramping block, so
    /// #1409's clamp acts in the rendered case at every lane of every width.
    ///
    /// Test value: a case edit that leaves some lane's move unclamped (a window too long for the
    /// offset, so the step rounds to nothing, or too short for the word to reach its target before
    /// the snap, or a window past the ramping block) turns this red; the digest alone cannot,
    /// since it would be re-pinned with the case.
    #[test]
    fn the_clamped_case_passes_every_target_under_the_unclamped_law() {
        for lane in 0..LANES {
            for channel in 0..2 {
                for word in 0..RAMP_COUNT {
                    let (start, target, step, samples) = clamped_move(lane, channel, word);
                    assert!(
                        samples as usize <= RAMP_FRAMES,
                        "lane {lane} channel {channel} word {word}: window {samples} ends past \
                         the ramping block"
                    );
                    let (low, high) = (start.min(target), start.max(target));
                    let mut unclamped = start;
                    let first_out = (1..samples).find(|_| {
                        unclamped += step;
                        !(low..=high).contains(&unclamped)
                    });
                    assert!(
                        first_out.is_some(),
                        "lane {lane} channel {channel} word {word}: the unclamped law stays \
                         inside [{low}, {high}] over its {samples}-sample window, so the clamp \
                         never acts"
                    );
                }
            }
        }
    }
}
