//! Exact rest after silence (`lanes_at_rest`, issue #1746) and the skipped block's clock advance
//! (`advance_clocks`, issue #1747).
//!
//! The descriptor states no tail bound yet (the SVF ring-out's certified bound is the parametric
//! EQ's #1372 derivation), so this is the rest claim the silent fast path and the per-lane skip
//! depend on: the joint flush of the SVF (armed by the input's silence clock, #1328) and the gain
//! word's flush bring every lane to exact `+0.0` state, after which a skipped block and a rendered
//! one leave the same payload.

mod support;

use effect_contract::LinkMode;
use support::{
    NATIVE_LANES, Values, defaults, noise, prepare_bank, process_bank, set, snapshot_bank,
};

const FRAMES: usize = 128;

#[test]
fn a_silent_bank_reaches_exact_rest_and_a_skip_equals_a_render() {
    let mut values: Vec<Values> = vec![defaults(); NATIVE_LANES];
    // Q 8 (the longest ring), the listen arm on lane 0.
    for set_values in &mut values {
        set(set_values, 1, 8.0);
    }
    set(&mut values[0], 9, 1.0);
    let links = vec![LinkMode::Maximum; NATIVE_LANES];
    let mut bank = prepare_bank(&values, &links);
    let mut twin = prepare_bank(&values, &links);
    let offsets = vec![0_u32; NATIVE_LANES + 1];
    let burst = noise(7, FRAMES * NATIVE_LANES * 40, 0.9);
    let mut rested = None;
    // 4 s at 48 kHz.
    for step in 0..1_500_usize {
        let span = FRAMES * NATIVE_LANES;
        let input = if step < 40 {
            burst[step * span..(step + 1) * span].to_vec()
        } else {
            vec![0.0_f32; span]
        };
        let first = (step * FRAMES) as u64;
        for effect in [&mut bank, &mut twin] {
            let (mut left, mut right) = (input.clone(), input.clone());
            process_bank(effect, &mut left, &mut right, first, &[], &offsets, false);
        }
        if step >= 40 && bank.processor.lanes_at_rest() == (1 << NATIVE_LANES) - 1 {
            rested = Some(step);
            break;
        }
    }
    let rested = rested.expect("every lane rests within 4 s of silence");
    // One more silent block: rendered on `bank`, skipped on `twin`.
    let mut left = vec![0.0_f32; FRAMES * NATIVE_LANES];
    let mut right = left.clone();
    let first = ((rested + 1) * FRAMES) as u64;
    process_bank(
        &mut bank,
        &mut left,
        &mut right,
        first,
        &[],
        &offsets,
        false,
    );
    assert!(
        left.iter().chain(&right).all(|word| word.to_bits() == 0),
        "+0.0 out"
    );
    twin.processor
        .advance_clocks((1 << NATIVE_LANES) - 1, FRAMES as u32);
    for track in 0..NATIVE_LANES as u32 {
        assert_eq!(
            snapshot_bank(&bank, track),
            snapshot_bank(&twin, track),
            "track {track}"
        );
    }
}
