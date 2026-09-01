#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! Gates 7.1, 7.2 and 7.4: lane identity, partition invariance and the signed-zero dry path.
//!
//! The oracle for lane identity is the scalar `Lane` instantiation (master plan #83 §1.7): the
//! eight-lane bank runs the *same* `gate_block` body at `WIDTH = 8`, so agreement is a property of
//! the code and any disagreement is a bug in the lane-crossing parts — which in this kernel are
//! exactly the per-lane detector gather and the coefficient packing.
use lane::Backend;

mod support;

use effect_contract::{
    BankWidth, EffectBankProcessBlock, LinkMode, PreparedAutomationSpan, PreparedNativeEffectBank,
    ProcessReport,
};
use support::{
    Values, active_values, add_report, assert_bits_eq, initial_values, noise, packed_w8, prepare,
    prepare_bank, prepare_bank_w8, render_scalar_sidechain, request, request_at, retarget_spans,
    set_parameter, snapshot, snapshot_bank, track_of,
};

/// Eight distinct but program-compatible parameter sets: the lookaheads cover 0, 2, 5 and 10 ms,
/// so every lane taps a different ring slot and a shared tap would fail immediately.
fn track_values() -> [Values; 8] {
    let lookaheads = [0.0, 2.0, 5.0, 10.0, 10.0, 5.0, 2.0, 0.0];
    core::array::from_fn(|track| {
        let mut values = active_values();
        set_parameter(&mut values, 0, -20.0 - track as f32, -22.0 - track as f32);
        set_parameter(&mut values, 1, 2.0 + track as f32, 3.0 + track as f32);
        set_parameter(&mut values, 2, 24.0 + track as f32, 30.0 + track as f32);
        set_parameter(&mut values, 3, 1.0 + track as f32, 2.0 + track as f32);
        set_parameter(
            &mut values,
            4,
            0.5 + track as f32 * 0.25,
            0.6 + track as f32 * 0.25,
        );
        set_parameter(&mut values, 5, track as f32, 1.0 + track as f32);
        set_parameter(&mut values, 6, 10.0 + track as f32, 12.0 + track as f32);
        set_parameter(&mut values, 7, lookaheads[track], lookaheads[7 - track]);
        values
    })
}

/// Seeded noise with 200-sample tone bursts on top, so the gate opens and closes repeatedly.
fn source(seed: u64, frames: usize) -> Vec<f32> {
    let mut signal = noise(seed, frames, 0.002);
    for (frame, sample) in signal.iter_mut().enumerate() {
        if (frame / 200) % 3 == 0 {
            *sample += 0.4 * ((frame as f32) * 0.05).sin();
        }
    }
    signal
}

#[test]
fn lane_identity_scalar_w8() {
    const BLOCK: usize = 128;
    const BLOCKS: usize = 64;
    const FRAMES: usize = BLOCK * BLOCKS;
    for link in [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average] {
        let values = track_values();
        let Some(mut bank) = prepare_bank_w8(&values, link) else {
            eprintln!("no eight-lane backend on this build");
            return;
        };
        let mut scalars: Vec<_> = (0..8)
            .map(|track| {
                let mut request = request(&values[track]);
                request.link_mode = link;
                prepare(request)
            })
            .collect();

        let left_source: Vec<Vec<f32>> = (0..8).map(|t| source(11 + t as u64, FRAMES)).collect();
        let right_source: Vec<Vec<f32>> = (0..8).map(|t| source(91 + t as u64, FRAMES)).collect();
        let mut bank_left = packed_w8(&left_source);
        let mut bank_right = packed_w8(&right_source);
        let mut scalar_left = left_source.clone();
        let mut scalar_right = right_source.clone();
        let mut scalar_reports = [ProcessReport::default(); 8];
        let mut bank_reports = [ProcessReport::default(); 8];

        let mut start = 0;
        while start < FRAMES {
            // The last block is 17 frames, so a partial block is inside the identity claim.
            let block = if FRAMES - start < BLOCK + 17 {
                17
            } else {
                BLOCK
            };
            let end = (start + block).min(FRAMES);
            let spans: Vec<PreparedAutomationSpan> = if start % 1_024 == 0 && start != 0 {
                retarget_spans(start as u64).to_vec()
            } else {
                Vec::new()
            };
            // Every track receives the same automation batch, so the bank's per-track offsets
            // enumerate the same spans eight times.
            let mut automation = Vec::new();
            let mut offsets = [0_u32; 9];
            for track in 0..8 {
                automation.extend_from_slice(&spans);
                offsets[track + 1] = automation.len() as u32;
            }
            let report = bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut bank_left[start * 8..end * 8],
                    &mut bank_right[start * 8..end * 8],
                    None,
                    (end - start) as u32,
                    BankWidth::Eight,
                    start as u64,
                    &automation,
                    &offsets,
                    128,
                )
                .expect("bank block"),
            );
            for track in 0..8 {
                add_report(&mut bank_reports[track], report.reports[track]);
                let single = render_scalar_sidechain(
                    scalars[track].as_mut(),
                    &mut scalar_left[track][start..end],
                    &mut scalar_right[track][start..end],
                    None,
                    end - start,
                    &spans,
                    start as u64,
                );
                add_report(&mut scalar_reports[track], single);
            }
            start = end;
        }

        for track in 0..8 {
            assert_bits_eq(
                &track_of(&bank_left, track, 8),
                &scalar_left[track],
                &format!("{link:?} left track {track}"),
            );
            assert_bits_eq(
                &track_of(&bank_right, track, 8),
                &scalar_right[track],
                &format!("{link:?} right track {track}"),
            );
            assert_eq!(
                bank_reports[track], scalar_reports[track],
                "{link:?} reports track {track}"
            );
            assert_eq!(
                snapshot_bank(bank.as_ref(), track as u32),
                snapshot(scalars[track].as_ref()),
                "{link:?} payload track {track}"
            );
        }
        assert!(
            bank_left.iter().any(|sample| *sample != 0.0),
            "{link:?}: the identity comparison must not be vacuous"
        );
    }
}

/// Renders through a bank in `block`-sized chunks, with one automation batch at frame 0.
fn render_bank(
    bank: &mut dyn PreparedNativeEffectBank,
    left: &mut [f32],
    right: &mut [f32],
    block: usize,
    automation: &[PreparedAutomationSpan],
    quantum: u32,
) {
    let frames = left.len() / 8;
    let mut start = 0;
    while start < frames {
        let end = (start + block).min(frames);
        let mut spans = Vec::new();
        let mut offsets = [0_u32; 9];
        for track in 0..8 {
            if start == 0 {
                spans.extend_from_slice(automation);
            }
            offsets[track + 1] = spans.len() as u32;
        }
        bank.process_bank(
            EffectBankProcessBlock::new(
                &mut left[start * 8..end * 8],
                &mut right[start * 8..end * 8],
                None,
                (end - start) as u32,
                BankWidth::Eight,
                start as u64,
                &spans,
                &offsets,
                quantum,
            )
            .expect("bank block"),
        );
        start = end;
    }
}

#[test]
fn partition_invariance() {
    const FRAMES: usize = 4_096;
    // The automation batch lands at frame 0, which is a block boundary in every partition, so the
    // event itself is identical and what is being compared is only where the block edges fall
    // relative to the 64-sample ramp.
    let spans = retarget_spans(0);
    let values = track_values();
    let left_source: Vec<Vec<f32>> = (0..8).map(|t| source(5 + t as u64, FRAMES)).collect();
    let right_source: Vec<Vec<f32>> = (0..8).map(|t| source(77 + t as u64, FRAMES)).collect();

    for &partition in &[1_usize, 7, 64, 128, 512] {
        // Scalar leg.
        for track in 0..8 {
            let mut reference = prepare(request_at(&values[track], 48_000, 512));
            let mut one_shot_left = left_source[track].clone();
            let mut one_shot_right = right_source[track].clone();
            let mut split_left = left_source[track].clone();
            let mut split_right = right_source[track].clone();
            render_scalar_sidechain(
                reference.as_mut(),
                &mut one_shot_left,
                &mut one_shot_right,
                None,
                512,
                &spans,
                0,
            );
            let mut split = prepare(request_at(&values[track], 48_000, 512));
            render_scalar_sidechain(
                split.as_mut(),
                &mut split_left,
                &mut split_right,
                None,
                partition,
                &spans,
                0,
            );
            assert_bits_eq(
                &split_left,
                &one_shot_left,
                &format!("scalar track {track} left at partition {partition}"),
            );
            assert_bits_eq(
                &split_right,
                &one_shot_right,
                &format!("scalar track {track} right at partition {partition}"),
            );
            assert_eq!(
                snapshot(split.as_ref()),
                snapshot(reference.as_ref()),
                "scalar track {track} state at partition {partition}"
            );
        }

        // Bank leg.
        let bank = |values: &[Values; 8]| {
            prepare_bank(
                values,
                LinkMode::DualMono,
                BankWidth::Eight,
                Backend::Simd8,
                512,
            )
        };
        let Some(mut reference) = bank(&values) else {
            continue;
        };
        let Some(mut split) = bank(&values) else {
            continue;
        };
        let mut one_shot_left = packed_w8(&left_source);
        let mut one_shot_right = packed_w8(&right_source);
        let mut split_left = one_shot_left.clone();
        let mut split_right = one_shot_right.clone();
        render_bank(
            reference.as_mut(),
            &mut one_shot_left,
            &mut one_shot_right,
            512,
            &spans,
            512,
        );
        render_bank(
            split.as_mut(),
            &mut split_left,
            &mut split_right,
            partition,
            &spans,
            512,
        );
        assert_bits_eq(
            &split_left,
            &one_shot_left,
            &format!("W8 left at partition {partition}"),
        );
        assert_bits_eq(
            &split_right,
            &one_shot_right,
            &format!("W8 right at partition {partition}"),
        );
        for track in 0..8 {
            assert_eq!(
                snapshot_bank(split.as_ref(), track as u32),
                snapshot_bank(reference.as_ref(), track as u32),
                "W8 state track {track} at partition {partition}"
            );
        }
    }
}

#[test]
fn signed_zero_identity_is_bit_exact() {
    // D7 flushes the one recursive state word, never the dry path. A `-0.0` input therefore has to
    // come back out of the latency ring as `-0.0`, bit for bit, while the gate is an identity.
    const FRAMES: usize = 512;
    let values = initial_values();
    let mut left = vec![0.0_f32; FRAMES];
    let mut right = vec![0.0_f32; FRAMES];
    left[0] = -0.0;
    right[0] = 0.0;

    let mut effect = prepare(request(&values));
    let report = render_scalar_sidechain(effect.as_mut(), &mut left, &mut right, None, 128, &[], 0);
    assert_eq!(
        left[480].to_bits(),
        0x8000_0000,
        "scalar: the negative zero survived the ring and the select"
    );
    assert_eq!(right[480].to_bits(), 0, "scalar: positive zero");
    assert_eq!(
        report,
        ProcessReport::default(),
        "scalar: no report movement"
    );
    // The stored gain word is the canonical `+0.0` even though the dry path kept its sign.
    let (_, payload_left, _) = snapshot(effect.as_ref());
    assert_eq!(
        u32::from_le_bytes(payload_left[0..4].try_into().expect("word")),
        0,
        "scalar: G is canonical +0"
    );

    let bank_values: [Values; 8] = core::array::from_fn(|_| initial_values());
    let Some(mut bank) = prepare_bank_w8(&bank_values, LinkMode::DualMono) else {
        return;
    };
    let mut bank_left = vec![0.0_f32; FRAMES * 8];
    let mut bank_right = vec![0.0_f32; FRAMES * 8];
    for track in 0..8 {
        bank_left[track] = -0.0;
        bank_right[track] = 0.0;
    }
    render_bank(
        bank.as_mut(),
        &mut bank_left,
        &mut bank_right,
        128,
        &[],
        128,
    );
    for track in 0..8 {
        assert_eq!(
            bank_left[480 * 8 + track].to_bits(),
            0x8000_0000,
            "W8 track {track}: negative zero"
        );
        assert_eq!(bank_right[480 * 8 + track].to_bits(), 0, "W8 track {track}");
    }
}
