//! Exact rest after silence (`lanes_at_rest`, issue #1746) and the skipped block's clock advance
//! (`advance_clocks`, issue #1747).
//!
//! The descriptor states no tail bound yet (the SVF ring-out's certified bound is the parametric
//! EQ's #1372 derivation), so this is the rest claim the silent fast path and the per-lane skip
//! depend on: every lane, with four live bands at Q 16 (the longest ring), a band live-disabled
//! during the burst and a 200 ms release, reaches exact `+0.0` state within 8 s of silence,
//! after which a skipped block and a rendered one leave the same payload.

mod support;

use effect_contract::LinkMode;
use support::{
    ENABLED, GAIN, NATIVE_LANES, Q, RELEASE, Values, defaults, index, noise, prepare_bank,
    process_bank, set_band, snapshot_bank, spans_both,
};

const FRAMES: usize = 128;

#[test]
fn a_silent_bank_reaches_exact_rest_and_a_skip_equals_a_render() {
    let mut values: Vec<Values> = vec![defaults(); NATIVE_LANES];
    for set_values in &mut values {
        for band in 0..4 {
            set_band(set_values, band, Q, 16.0);
            set_band(set_values, band, GAIN, 12.0);
            set_band(set_values, band, RELEASE, 200.0);
        }
    }
    let links = vec![LinkMode::Maximum; NATIVE_LANES];
    let mut bank = prepare_bank(&values, &links);
    let mut twin = prepare_bank(&values, &links);
    // Band 2 is disabled on every lane mid-burst: its frozen integrators must not hold rest off.
    let disable: Vec<_> = (0..NATIVE_LANES)
        .flat_map(|_| spans_both(index(2, ENABLED) as u32, 0.0, (20 * FRAMES) as u64))
        .collect();
    let disable_offsets: Vec<u32> = (0..=NATIVE_LANES).map(|lane| 2 * lane as u32).collect();
    let offsets = vec![0_u32; NATIVE_LANES + 1];
    let burst = noise(7, FRAMES * NATIVE_LANES * 40, 0.9);
    let mut rested = None;
    // 8 s at 48 kHz.
    for step in 0..3_000_usize {
        let span = FRAMES * NATIVE_LANES;
        let input = if step < 40 {
            burst[step * span..(step + 1) * span].to_vec()
        } else {
            vec![0.0_f32; span]
        };
        let first = (step * FRAMES) as u64;
        for effect in [&mut bank, &mut twin] {
            let (mut left, mut right) = (input.clone(), input.clone());
            let (spans, offsets) = if step == 20 {
                (&disable[..], &disable_offsets[..])
            } else {
                (&[][..], &offsets[..])
            };
            process_bank(effect, &mut left, &mut right, first, spans, offsets, false);
        }
        if step >= 40 && bank.processor.lanes_at_rest() == (1 << NATIVE_LANES) - 1 {
            rested = Some(step);
            break;
        }
    }
    let rested = rested.expect("every lane rests within 8 s of silence");
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
