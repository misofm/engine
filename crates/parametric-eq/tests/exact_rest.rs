//! Issue #1328 gate 2: an EQ section that used to stick at a non-zero fixed point reaches rest.
//!
//! A low shelf at 10 Hz, +24 dB, slope 0.1, at 96 kHz, driven by a unit impulse, used to settle at
//! `ic1 = +0.0`, `ic2 = 6.01e-20` under the per-word flush: `2 * d2` is below half an ulp of `ic2`,
//! so the word never moves again and the output stays near -361 dBFS forever, which also keeps the
//! EQ's silent fast path from ever engaging. The joint SVF flush (`lane::flush_pair`) zeroes the
//! pair once both words are below `REST_EPS`, so the band's integrators reach `+0.0` and the output
//! reaches exactly `+0.0`.

mod support;

use std::f32::consts::FRAC_1_SQRT_2;

use effect_contract::{EffectProcessBlock, NativeEffectFactory, PreparedNativeEffect};
use parametric_eq::{EqBandKind, ParametricEqFactory};
use support::{band_word, request_at_rate, single_section_values, snapshot};

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
