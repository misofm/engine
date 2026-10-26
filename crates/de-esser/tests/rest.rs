//! Exact rest after silence (`lanes_at_rest`, issue #1746) and the skipped block's clock advance
//! (`advance_clocks`, issue #1747).
//!
//! The descriptor states no tail bound yet (the SVF ring-out's certified bound is the parametric
//! EQ's #1372 derivation), so this is the rest claim the silent fast path and the per-lane skip
//! depend on: every lane reaches exact `+0.0` state, after which a skipped block and a rendered one
//! leave the same payload. In this fixture the gain word's release (to its rest snap) is the last
//! word to settle; the SVF integrators reach `+0.0` earlier through their per-word flush, so the
//! joint flush (#1328) is not what this test exercises: #1372's tail and rest-bound gate owns it.

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

/// The shelf SVF (#1984) is rest state: on a split, high-pass bank at Q 0.5 and 1 kHz after a
/// 0.003 DC input, the critically damped band SVF flushes to `+0.0` before the Butterworth shelf
/// does, so at every 32-frame block where `lanes_at_rest` claims rest, a skipped twin
/// (`advance_clocks`) must leave the payload the rendered block leaves.
#[test]
fn the_shelf_state_is_part_of_rest() {
    const BLOCK: usize = 32;
    let mut values = defaults();
    set(&mut values, 0, 1_000.0);
    set(&mut values, 1, 0.5);
    set(&mut values, 7, 2.0);
    set(&mut values, 8, 2.0);
    let values = vec![values; NATIVE_LANES];
    let links = vec![LinkMode::DualMono; NATIVE_LANES];
    let mut bank = prepare_bank(&values, &links);
    let mut twin = prepare_bank(&values, &links);
    let offsets = vec![0_u32; NATIVE_LANES + 1];
    let all = (1 << NATIVE_LANES) - 1;
    let mut claims = 0;
    for step in 0..3_000_usize {
        let level = if step < 20 { 0.003 } else { 0.0 };
        let input = vec![level; BLOCK * NATIVE_LANES];
        let first = (step * BLOCK) as u64;
        // A skip is the contract only for a `+0.0` block over lanes at rest.
        let resting = level == 0.0 && bank.processor.lanes_at_rest() == all;
        let (mut left, mut right) = (input.clone(), input.clone());
        process_bank(
            &mut bank,
            &mut left,
            &mut right,
            first,
            &[],
            &offsets,
            false,
        );
        if resting {
            claims += 1;
            twin.processor.advance_clocks(all, BLOCK as u32);
            for track in 0..NATIVE_LANES as u32 {
                assert_eq!(
                    snapshot_bank(&bank, track),
                    snapshot_bank(&twin, track),
                    "block {step} track {track}: a skip is not a render"
                );
            }
        } else {
            let (mut left, mut right) = (input.clone(), input);
            process_bank(
                &mut twin,
                &mut left,
                &mut right,
                first,
                &[],
                &offsets,
                false,
            );
        }
    }
    assert!(claims > 0, "the bank rests");
}
