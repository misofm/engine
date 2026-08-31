//! E2 — the vector widths render the same bits as `WIDTH = 1`, lane for lane.
//!
//! One body instantiated at three widths is the whole reason lane identity is claimed as a
//! property of the code (master plan §1.1); this test is what turns the claim into evidence. Each
//! lane of a bank is compared against an independent scalar instance prepared with that lane's
//! parameters and fed that lane's signal, including automation that starts in different blocks on
//! different lanes — which is what makes the segmented ramp driver's per-lane minimum load-bearing.
//!
//! A bank binds only at the width this artifact executes, so only one of the two widths below runs
//! on a given host. The *kernel* is compared at all three widths on every host by
//! `tests/determinism.rs`, which runs the frozen corpus at `f32`, `Simd4` and `Simd8` and asserts
//! one digest for the three; what this file adds is the driver around it — ramps, automation,
//! bank binding and the snapshot.

mod support;

use effect_contract::{BankWidth, ParameterChannel, PreparedAutomationSpan};
use support::{
    PARAMETERS, bank_available, bits, prepare, prepare_bank, process, process_bank, values_from,
};

const FRAMES: usize = 128;
const BLOCKS: usize = 64;

/// A deterministic per-lane signal.
fn sample(lane: usize, index: usize) -> f32 {
    let base = index as f32 * (0.031 + lane as f32 * 0.007);
    base.sin() * 0.7 + (base * 3.1).sin() * 0.25
}

/// Per-lane parameter values, all inside the frozen domains and all different.
fn lane_values(lane: usize) -> [(f32, f32); PARAMETERS] {
    [
        (-24.0 + lane as f32 * 7.0, -12.0 + lane as f32 * 5.0),
        (-6.0 + lane as f32 * 2.0, 3.0 - lane as f32 * 1.5),
        (
            (lane as f32 * 0.13).min(1.0),
            1.0 - (lane as f32 * 0.11).min(1.0),
        ),
    ]
}

/// The automation this lane receives at the start of block `block`, if any.
fn lane_automation(lane: usize, block: usize, first_sample: u64) -> Vec<PreparedAutomationSpan> {
    if block % 8 != lane % 8 {
        return Vec::new();
    }
    let parameter = (block / 8) % PARAMETERS;
    let value = match parameter {
        0 => 6.0 + lane as f32,
        1 => -3.0 + lane as f32 * 0.5,
        _ => (0.05 * lane as f32).min(1.0),
    };
    vec![
        support::point(
            parameter as u32,
            ParameterChannel::Left,
            value,
            first_sample,
        ),
        support::point(
            parameter as u32,
            ParameterChannel::Right,
            value,
            first_sample,
        ),
    ]
}

fn lane_identity_at(width: BankWidth) {
    if !bank_available(width) {
        return;
    }
    let lanes = width.lanes() as usize;
    let per_lane: Vec<Vec<_>> = (0..lanes)
        .map(|lane| values_from(lane_values(lane)).to_vec())
        .collect();
    let mut bank = prepare_bank(width, &per_lane).expect("bank binds on this host");
    let mut scalars: Vec<_> = per_lane.iter().map(|values| prepare(values)).collect();

    let mut first_sample = 0_u64;
    for block in 0..BLOCKS {
        let mut bank_left = vec![0.0_f32; FRAMES * lanes];
        let mut bank_right = vec![0.0_f32; FRAMES * lanes];
        for frame in 0..FRAMES {
            for lane in 0..lanes {
                bank_left[frame * lanes + lane] = sample(lane, block * FRAMES + frame);
                bank_right[frame * lanes + lane] = sample(lane + 3, block * FRAMES + frame);
            }
        }
        let mut automation = Vec::new();
        let mut offsets = vec![0_u32];
        for lane in 0..lanes {
            automation.extend(lane_automation(lane, block, first_sample));
            offsets.push(automation.len() as u32);
        }
        process_bank(
            bank.as_mut(),
            width,
            &mut bank_left,
            &mut bank_right,
            FRAMES,
            first_sample,
            &automation,
            &offsets,
        );

        for (lane, scalar) in scalars.iter_mut().enumerate() {
            let mut left: Vec<f32> = (0..FRAMES)
                .map(|frame| sample(lane, block * FRAMES + frame))
                .collect();
            let mut right: Vec<f32> = (0..FRAMES)
                .map(|frame| sample(lane + 3, block * FRAMES + frame))
                .collect();
            let spans = lane_automation(lane, block, first_sample);
            process(scalar.as_mut(), &mut left, &mut right, first_sample, &spans);
            let bank_lane_left: Vec<f32> = (0..FRAMES)
                .map(|frame| bank_left[frame * lanes + lane])
                .collect();
            let bank_lane_right: Vec<f32> = (0..FRAMES)
                .map(|frame| bank_right[frame * lanes + lane])
                .collect();
            assert_eq!(
                bits(&bank_lane_left),
                bits(&left),
                "left, width {lanes}, block {block}, lane {lane}"
            );
            assert_eq!(
                bits(&bank_lane_right),
                bits(&right),
                "right, width {lanes}, block {block}, lane {lane}"
            );
        }
        first_sample += FRAMES as u64;
    }

    // The state each width carries is the same too, or the next block would diverge.
    for (lane, scalar) in scalars.iter().enumerate() {
        assert_eq!(
            support::snapshot_bank(bank.as_ref(), lane as u32),
            support::snapshot(scalar.as_ref()),
            "state, width {lanes}, lane {lane}"
        );
    }
}

#[test]
fn the_hosts_bank_width_matches_the_scalar_instantiation() {
    lane_identity_at(BankWidth::Four);
    lane_identity_at(BankWidth::Eight);
    assert!(
        bank_available(BankWidth::Four) || bank_available(BankWidth::Eight),
        "no bank width is native to this artifact, so this test proved nothing"
    );
}
