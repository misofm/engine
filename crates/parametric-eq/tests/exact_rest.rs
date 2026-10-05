//! Issue #1328 gate 2: an EQ section that used to stick at a non-zero fixed point reaches rest.
//!
//! A low shelf at 10 Hz, +24 dB, slope 0.1, at 96 kHz, driven by a unit impulse, used to settle at
//! `ic1 = +0.0`, `ic2 = 6.01e-20` under the per-word flush: `2 * d2` is below half an ulp of `ic2`,
//! so the word never moves again and the output stays near -361 dBFS forever, which also keeps the
//! EQ's silent fast path from ever engaging. The joint SVF flush (`lane::flush_pair`) zeroes the
//! pair once both words are below `REST_EPS` on a silent input, so the band's integrators reach
//! `+0.0` and the output reaches exactly `+0.0`.
//!
//! Amendment A8 gate: the same rule must leave a tiny but non-zero input alone. Four boosting low
//! shelves fed a square far below any rest threshold must apply all four boosts, exactly as they
//! do to the same square at an ordinary level.

mod support;

use std::f32::consts::FRAC_1_SQRT_2;

use effect_contract::{
    EffectProcessBlock, InitialParameterValue, NativeEffectFactory, ParameterChannel,
    PreparedNativeEffect,
};
use parametric_eq::{EqBandKind, ParametricEqFactory};
use support::{band_word, request_at_rate, set_initial, single_section_values, snapshot, values};

const RATE: u32 = 96_000;
const FRAMES: usize = 128;
/// The bound the issue states: rest within 9,600,000 samples (100 s) of the impulse.
const REST_BOUND_SAMPLES: usize = 9_600_000;
/// Blocks rendered after rest that must stay at rest.
const HELD_BLOCKS: usize = 4;

fn at_rest(effect: &dyn PreparedNativeEffect, left: &[f32], right: &[f32]) -> bool {
    let (_, left_state, right_state) = snapshot(effect);
    [&left_state[..], &right_state[..]]
        .iter()
        .all(|payload| band_word(payload, 0, 0) == 0 && band_word(payload, 0, 1) == 0)
        && left.iter().chain(right).all(|sample| sample.to_bits() == 0)
}

#[test]
fn the_low_shelf_fixed_point_reaches_exact_rest() {
    let configured = single_section_values(EqBandKind::LowShelf, 10.0, 24.0, FRAC_1_SQRT_2, 0.1);
    let mut effect = ParametricEqFactory
        .prepare(request_at_rate(&configured, false, RATE))
        .expect("the shelf prepares");

    let mut left = [0.0_f32; FRAMES];
    let mut right = [0.0_f32; FRAMES];
    left[0] = 1.0;
    right[0] = 1.0;
    let mut rest_at = None;
    let mut block = 0_usize;
    while block * FRAMES < REST_BOUND_SAMPLES {
        let first_sample = (block * FRAMES) as u64;
        effect.process(
            EffectProcessBlock::new(
                &mut left,
                &mut right,
                None,
                first_sample,
                &[],
                FRAMES as u32,
            )
            .expect("block"),
        );
        if block > 0 && at_rest(effect.as_ref(), &left, &right) {
            rest_at = Some(block);
            break;
        }
        left.fill(0.0);
        right.fill(0.0);
        block += 1;
    }
    let rest_at = rest_at.unwrap_or_else(|| {
        let (_, left_state, _) = snapshot(effect.as_ref());
        panic!(
            "no exact rest within {REST_BOUND_SAMPLES} samples: band 0 left ic1 {:e}, ic2 {:e}",
            f32::from_bits(band_word(&left_state, 0, 0)),
            f32::from_bits(band_word(&left_state, 0, 1)),
        )
    });
    for held in 1..=HELD_BLOCKS {
        left.fill(0.0);
        right.fill(0.0);
        let first_sample = ((rest_at + held) * FRAMES) as u64;
        effect.process(
            EffectProcessBlock::new(
                &mut left,
                &mut right,
                None,
                first_sample,
                &[],
                FRAMES as u32,
            )
            .expect("block"),
        );
        assert!(
            at_rest(effect.as_ref(), &left, &right),
            "rest reached at block {rest_at} must hold, broke {held} blocks later"
        );
    }
}

/// Parameters per EQ band in the descriptor: enabled, kind, frequency, gain, Q, slope.
const BAND_PARAMETERS: usize = 6;
/// The four general bands, all used by the chain gate.
const BANDS: usize = 4;
/// Half a period of the chain gate's square: 12,488 samples, 3.84 Hz at 96 kHz (the attempt-3
/// verdict's measurement input).
const SQUARE_HALF_PERIOD: usize = 12_488;
/// Samples the chain gate renders: four half periods of the square, so every shelf has settled.
const CHAIN_SAMPLES: usize = 4 * SQUARE_HALF_PERIOD;
/// The ordinary input level the tiny runs are scaled from.
const REFERENCE_LEVEL: f32 = 0.03;

/// Every general band a low shelf at `frequency`, +24 dB, slope 1, on both channels.
fn four_boosting_shelves(frequency: f32) -> Vec<InitialParameterValue> {
    let mut configured = values();
    for band in 0..BANDS {
        let base = band * BAND_PARAMETERS;
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            set_initial(&mut configured, base, channel, 1.0);
            set_initial(
                &mut configured,
                base + 1,
                channel,
                EqBandKind::LowShelf as u32 as f32,
            );
            set_initial(&mut configured, base + 2, channel, frequency);
            set_initial(&mut configured, base + 3, channel, 24.0);
            set_initial(&mut configured, base + 4, channel, FRAC_1_SQRT_2);
            set_initial(&mut configured, base + 5, channel, 1.0);
        }
    }
    configured
}

/// `2^exponent`, exactly, for a normal-range exponent: built from its bits.
fn power_of_two(exponent: i32) -> f32 {
    f32::from_bits(u32::try_from(127 + exponent).expect("a normal exponent") << 23)
}

/// Renders the square at `level` through a fresh EQ and returns the left output.
fn render_square(configured: &[InitialParameterValue], level: f32) -> Vec<f32> {
    let mut effect = ParametricEqFactory
        .prepare(request_at_rate(configured, false, RATE))
        .expect("the shelves prepare");
    let mut output = Vec::with_capacity(CHAIN_SAMPLES);
    for first in (0..CHAIN_SAMPLES).step_by(FRAMES) {
        let mut left = [0.0_f32; FRAMES];
        for (offset, sample) in left.iter_mut().enumerate() {
            let polarity = if ((first + offset) / SQUARE_HALF_PERIOD).is_multiple_of(2) {
                1.0
            } else {
                -1.0
            };
            *sample = polarity * level;
        }
        let mut right = left;
        effect.process(
            EffectProcessBlock::new(
                &mut left,
                &mut right,
                None,
                first as u64,
                &[],
                FRAMES as u32,
            )
            .expect("block"),
        );
        output.extend_from_slice(&left);
    }
    output
}

/// Amendment A8: a tiny non-zero input through four +24 dB low shelves gets its full boost.
///
/// The joint flush ends a decay only on a sample whose input is exactly zero. A rule that ignored
/// the input (attempt 3's) also zeroed a section's state whenever both words sat below `REST_EPS`
/// while a tiny signal drove it: such a section passed only its direct term, so each following
/// section saw the unboosted signal and stayed at rest too, and four +24 dB shelves at 10 Hz lost
/// about 96 dB of response below about `3e-11` (the attempt-3 verdict measured `1.5e-6` of change at
/// the EQ's output, -116 dBFS).
///
/// Every input here is the same 3.84 Hz square at `REFERENCE_LEVEL * 2^-k`, at 10 Hz and 100 Hz:
/// `k = 30` and `34` sit just under one section's old rest limit at those frequencies (`2.8e-11`
/// and `1.7e-12`), `k = 36` (`4.4e-13`) under both. Scaling an input by a power of two scales every
/// product and sum of the recurrence exactly while no word reaches the subnormal range or
/// `FLUSH_EPS`, so the tiny run, scaled back up by `2^k`, must be the ordinary run. The comparison
/// allows `1e-4` of the ordinary run's peak, because a state word that crosses zero can land inside
/// the per-word law's `FLUSH_EPS` band (measured up to `1.3e-6` of the peak at these levels; the
/// per-word law's own limit, `3e-17` at 10 Hz, is far below them). A lost boost misses by nearly
/// the whole peak.
#[test]
fn a_tiny_input_through_four_boosting_shelves_gets_every_boost() {
    for frequency in [10.0_f32, 100.0] {
        let configured = four_boosting_shelves(frequency);
        let ordinary = render_square(&configured, REFERENCE_LEVEL);
        let peak = ordinary.iter().fold(0.0_f32, |peak, y| peak.max(y.abs()));
        // About +96 dB of boost at 3.84 Hz, so the ordinary run peaks far above its input.
        assert!(
            peak > 1_000.0 * REFERENCE_LEVEL,
            "{frequency} Hz: four +24 dB shelves must boost the ordinary square (peak {peak:e})"
        );
        for k in [30_i32, 34, 36] {
            let level = REFERENCE_LEVEL * power_of_two(-k);
            let tiny = render_square(&configured, level);
            let scale = power_of_two(k);
            let (worst, at) = tiny
                .iter()
                .zip(&ordinary)
                .enumerate()
                .map(|(index, (small, big))| ((small * scale - big).abs(), index))
                .fold(
                    (0.0_f32, 0),
                    |best, next| if next.0 > best.0 { next } else { best },
                );
            assert!(
                worst <= 1.0e-4 * peak,
                "{frequency} Hz, input {level:e}: the tiny run, scaled by 2^{k}, misses the \
                 ordinary run by {worst:e} at sample {at} (ordinary peak {peak:e}); \
                 the shelves must apply every boost to a tiny non-zero input"
            );
        }
    }
}
