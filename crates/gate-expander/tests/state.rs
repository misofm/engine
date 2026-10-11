//! State continuation, resets, exact lengths, rollback, and lane-local recovery.

mod support;

use effect_contract::{
    EffectBankProcessBlock, LinkMode, PreparedAutomationSpan, PreparedNativeEffectBank, ResetKind,
    StatePayloadInput, StatePayloadOutput,
};
use gate_expander::STATE_LAYOUT_VERSION;
use support::{
    NATIVE_LANES, Values, active_values, assert_bits_eq, initial_values, native_width, noise,
    packed, prepare, prepare_bank_at_rate, prepare_bank_native, render_scalar_sidechain, request,
    retarget_spans, snapshot, snapshot_bank, track_of,
};

fn word(bytes: &[u8], index: usize) -> u32 {
    u32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().unwrap())
}

fn render_bank(
    bank: &mut dyn PreparedNativeEffectBank,
    left: &mut [f32],
    right: &mut [f32],
    block: usize,
) {
    let offsets = [0_u32; NATIVE_LANES + 1];
    render_bank_with_automation(bank, left, right, block, &[], &offsets);
}

fn render_bank_with_automation(
    bank: &mut dyn PreparedNativeEffectBank,
    left: &mut [f32],
    right: &mut [f32],
    block: usize,
    automation: &[PreparedAutomationSpan],
    automation_offsets: &[u32],
) {
    let frames = left.len() / NATIVE_LANES;
    let empty_offsets = [0_u32; NATIVE_LANES + 1];
    let mut start = 0;
    while start < frames {
        let end = (start + block).min(frames);
        let (block_automation, block_offsets) = if start == 0 {
            (automation, automation_offsets)
        } else {
            (&[][..], &empty_offsets[..])
        };
        bank.process_bank(
            EffectBankProcessBlock::new(
                &mut left[start * NATIVE_LANES..end * NATIVE_LANES],
                &mut right[start * NATIVE_LANES..end * NATIVE_LANES],
                None,
                (end - start) as u32,
                native_width(),
                start as u64,
                block_automation,
                block_offsets,
                128,
            )
            .unwrap(),
        );
        start = end;
    }
}

#[test]
fn reset_seeds_gain_open_hold_and_preserves_or_restores_parameters() {
    let mut values = active_values();
    support::set_parameter(&mut values, 2, 40.0, 40.0);
    let hold_samples = 3.0_f32;
    support::set_parameter(
        &mut values,
        5,
        hold_samples * 1000.0 / 48_000.0,
        hold_samples * 1000.0 / 48_000.0,
    );
    let mut effect = prepare(request(&values));
    let mut left = vec![0.01; 17];
    let mut right = vec![0.01; 17];
    render_scalar_sidechain(
        &mut effect,
        &mut left,
        &mut right,
        None,
        17,
        &retarget_spans(0),
        0,
    );
    let (_, before_left, before_right) = snapshot(&effect);
    assert_ne!(
        word(&before_left, 0),
        0,
        "the reset witness has active gain"
    );
    assert_eq!(word(&before_left, 6 + 3), 47.0_f32.to_bits());
    assert_eq!(word(&before_right, 6 + 3), 47.0_f32.to_bits());
    for (label, before) in [("left", &before_left), ("right", &before_right)] {
        for ramp in 0..4 {
            let slot = 6 + ramp * 4;
            assert_ne!(word(before, slot + 2), 0, "{label}: ramp {ramp} step");
            assert_eq!(
                word(before, slot + 3),
                47.0_f32.to_bits(),
                "{label}: ramp {ramp} remaining"
            );
        }
    }
    effect
        .processor
        .reset(ResetKind::DiscontinuityKeepParameters);
    let (_, discontinuity_left, discontinuity_right) = snapshot(&effect);
    for (label, before, after) in [
        ("left", &before_left, &discontinuity_left),
        ("right", &before_right, &discontinuity_right),
    ] {
        assert_eq!(word(after, 0), 0, "{label}: gain reset");
        assert_eq!(word(after, 1), 1.0_f32.to_bits(), "{label}: gate open");
        assert_eq!(
            word(after, 2),
            hold_samples.to_bits(),
            "{label}: hold reset to prepared K"
        );
        for index in 3..=5 {
            assert_eq!(
                word(after, index),
                word(before, index),
                "{label}: timing {index}"
            );
        }
        for ramp in 0..4 {
            let slot = 6 + ramp * 4;
            assert_eq!(
                word(after, slot),
                word(after, slot + 1),
                "{label}: ramp {ramp} target"
            );
            assert_eq!(word(after, slot + 2), 0, "{label}: ramp {ramp} step");
            assert_eq!(word(after, slot + 3), 0, "{label}: ramp {ramp} remaining");
        }
    }
    effect.processor.reset(ResetKind::FullToDefaults);
    let full = snapshot(&effect);
    let fresh = prepare(request(&values));
    assert_eq!(
        full,
        snapshot(&fresh),
        "full reset restores every common, left, and right payload value"
    );
}

#[test]
fn active_state_restore_continues_an_uninterrupted_donor() {
    let values = active_values();
    let source_left = noise(101, 1024, 0.4);
    let source_right = noise(202, 1024, 0.4);
    let spans = support::retarget_spans(0);
    let mut donor = prepare(request(&values));
    let mut warm_left = source_left[..17].to_vec();
    let mut warm_right = source_right[..17].to_vec();
    render_scalar_sidechain(
        &mut donor,
        &mut warm_left,
        &mut warm_right,
        None,
        17,
        &spans,
        0,
    );
    let payload = snapshot(&donor);
    assert_eq!(word(&payload.1, 6 + 3), 47.0_f32.to_bits());

    let mut uninterrupted = donor;
    let mut expected_left = source_left[17..].to_vec();
    let mut expected_right = source_right[17..].to_vec();
    render_scalar_sidechain(
        &mut uninterrupted,
        &mut expected_left,
        &mut expected_right,
        None,
        128,
        &[],
        17,
    );

    let mut restored = prepare(request(&values));
    let sizes = restored.metadata.state_sizes;
    restored
        .processor
        .restore_state_payload(
            STATE_LAYOUT_VERSION,
            StatePayloadInput::new(&payload.0, &payload.1, &payload.2, sizes).unwrap(),
        )
        .unwrap();
    let mut actual_left = source_left[17..].to_vec();
    let mut actual_right = source_right[17..].to_vec();
    render_scalar_sidechain(
        &mut restored,
        &mut actual_left,
        &mut actual_right,
        None,
        128,
        &[],
        17,
    );
    assert_bits_eq(&actual_left, &expected_left, "restored left");
    assert_bits_eq(&actual_right, &expected_right, "restored right");
    assert_eq!(snapshot(&restored), snapshot(&uninterrupted));
}

#[test]
fn old_lengths_and_one_byte_short_payloads_reject_scalar_and_bank() {
    for rate in [44_100, 48_000, 88_200, 96_000] {
        let values = initial_values();
        let mut scalar = prepare(support::request_at_rate(&values, rate));
        let sizes = scalar.metadata.state_sizes;
        let common = vec![0; sizes.common_bytes as usize];
        let old_bytes = match rate {
            44_100 => 3_620,
            48_000 => 3_932,
            88_200 => 7_148,
            96_000 => 7_772,
            _ => unreachable!(),
        };
        let old_lane = vec![0; old_bytes];
        assert!(
            scalar
                .processor
                .restore_state_payload(
                    STATE_LAYOUT_VERSION,
                    StatePayloadInput {
                        common: &common,
                        left: &old_lane,
                        right: &old_lane
                    },
                )
                .is_err()
        );
        let values: [Values; 8] = core::array::from_fn(|_| initial_values());
        if let Some(mut bank) = prepare_bank_at_rate(
            &values,
            LinkMode::DualMono,
            native_width(),
            lane::Backend::current(),
            128,
            rate,
        ) {
            let sizes = bank.metadata.program_key.state_sizes;
            assert!(
                bank.processor
                    .restore_track_state_payload(
                        0,
                        STATE_LAYOUT_VERSION,
                        StatePayloadInput {
                            common: &common,
                            left: &old_lane,
                            right: &old_lane
                        },
                    )
                    .is_err(),
                "old raw bank payload at {rate} Hz"
            );
            assert_eq!(sizes.left_bytes, 88);
        }
        let mut short_left = vec![0; sizes.left_bytes as usize - 1];
        let mut short_right = vec![0; sizes.right_bytes as usize];
        assert!(
            StatePayloadOutput::new(
                &mut vec![0; sizes.common_bytes as usize],
                &mut short_left,
                &mut short_right,
                sizes
            )
            .is_err()
        );
    }
}

#[test]
fn malformed_final_right_word_leaves_both_channels_unchanged() {
    let values = active_values();
    let mut effect = prepare(request(&values));
    let mut warm_left = vec![0.01_f32; 17];
    let mut warm_right = warm_left.clone();
    render_scalar_sidechain(
        &mut effect,
        &mut warm_left,
        &mut warm_right,
        None,
        17,
        &[],
        0,
    );
    let before = snapshot(&effect);
    assert_ne!(word(&before.1, 0), 0, "rollback witness has live gain");
    let mut right = before.2.clone();
    right[87] = 0xff;
    let sizes = effect.metadata.state_sizes;
    let input = StatePayloadInput::new(&before.0, &before.1, &right, sizes).unwrap();
    assert!(
        effect
            .processor
            .restore_state_payload(STATE_LAYOUT_VERSION, input)
            .is_err()
    );
    assert_eq!(snapshot(&effect), before);

    let values: [Values; 8] = core::array::from_fn(|track| {
        let mut values = active_values();
        support::set_parameter(&mut values, 0, -20.0 - track as f32, -20.0 - track as f32);
        values
    });
    let mut control =
        prepare_bank_native(&values, LinkMode::DualMono).expect("the build's own width binds");
    let mut target =
        prepare_bank_native(&values, LinkMode::DualMono).expect("the build's own width binds");
    let prefix_left = packed(&[&[0.01_f32; 17]; NATIVE_LANES]);
    let prefix_right = prefix_left.clone();
    let mut control_prefix_left = prefix_left.clone();
    let mut control_prefix_right = prefix_right.clone();
    render_bank(
        control.processor.as_mut(),
        &mut control_prefix_left,
        &mut control_prefix_right,
        17,
    );
    let mut target_prefix_left = prefix_left;
    let mut target_prefix_right = prefix_right;
    render_bank(
        target.processor.as_mut(),
        &mut target_prefix_left,
        &mut target_prefix_right,
        17,
    );
    let state_before: Vec<_> = (0..NATIVE_LANES as u32)
        .map(|track| snapshot_bank(&target, track))
        .collect();
    assert!(state_before.iter().any(|payload| word(&payload.1, 0) != 0));
    let mut malformed_right = state_before[3].2.clone();
    malformed_right[87] = 0xff;
    let sizes = target.metadata.program_key.state_sizes;
    assert!(
        target
            .processor
            .restore_track_state_payload(
                3,
                STATE_LAYOUT_VERSION,
                StatePayloadInput::new(
                    &state_before[3].0,
                    &state_before[3].1,
                    &malformed_right,
                    sizes,
                )
                .expect("bank payload sizes"),
            )
            .is_err()
    );
    for track in 0..NATIVE_LANES as u32 {
        assert_eq!(
            snapshot_bank(&target, track),
            state_before[track as usize],
            "bank lane {track} survives malformed right restore"
        );
    }
    let continuation = packed(&[&[0.01_f32; 17]; NATIVE_LANES]);
    let mut control_left = continuation.clone();
    let mut control_right = continuation.clone();
    let mut target_left = continuation.clone();
    let mut target_right = continuation;
    render_bank(
        control.processor.as_mut(),
        &mut control_left,
        &mut control_right,
        17,
    );
    render_bank(
        target.processor.as_mut(),
        &mut target_left,
        &mut target_right,
        17,
    );
    for track in 0..NATIVE_LANES {
        assert_bits_eq(
            &track_of(&target_left, track, NATIVE_LANES),
            &track_of(&control_left, track, NATIVE_LANES),
            &format!("bank lane {track} left after rejected restore"),
        );
        assert_bits_eq(
            &track_of(&target_right, track, NATIVE_LANES),
            &track_of(&control_right, track, NATIVE_LANES),
            &format!("bank lane {track} right after rejected restore"),
        );
    }
}

#[test]
fn scalar_and_bank_recovery_is_channel_and_lane_local() {
    let mut values = active_values();
    support::set_parameter(&mut values, 2, 40.0, 40.0);
    let mut scalar = prepare(request(&values));
    let mut scalar_control = prepare(request(&values));
    let mut warm_left = vec![0.01; 17];
    let mut warm_right = warm_left.clone();
    let mut control_warm_left = warm_left.clone();
    let mut control_warm_right = warm_right.clone();
    render_scalar_sidechain(
        &mut scalar,
        &mut warm_left,
        &mut warm_right,
        None,
        17,
        &retarget_spans(0),
        0,
    );
    render_scalar_sidechain(
        &mut scalar_control,
        &mut control_warm_left,
        &mut control_warm_right,
        None,
        17,
        &retarget_spans(0),
        0,
    );
    let before = snapshot(&scalar);
    assert_ne!(word(&before.1, 0), 0, "scalar fault witness has live gain");
    assert_ne!(word(&before.2, 0), 0, "scalar peer witness has live gain");
    for (label, payload) in [("left", &before.1), ("right", &before.2)] {
        for ramp in 0..4 {
            let slot = 6 + ramp * 4;
            assert_ne!(word(payload, slot + 2), 0, "{label}: ramp {ramp} step");
            assert_eq!(
                word(payload, slot + 3),
                47.0_f32.to_bits(),
                "{label}: ramp {ramp} remaining"
            );
        }
    }
    let mut left = vec![0.2; 128];
    let mut right = vec![0.2; 128];
    left[0] = f32::NAN;
    let report = render_scalar_sidechain(&mut scalar, &mut left, &mut right, None, 128, &[], 17);
    assert_eq!(report.nonfinite_left_blocks, 128);
    assert!(left.iter().all(|sample| sample.to_bits() == 0));
    assert!(right.iter().all(|sample| sample.is_finite()));
    let mut control_left = vec![0.2; 128];
    let mut control_right = vec![0.2; 128];
    render_scalar_sidechain(
        &mut scalar_control,
        &mut control_left,
        &mut control_right,
        None,
        128,
        &[],
        17,
    );
    let fresh = prepare(request(&values));
    let after = snapshot(&scalar);
    let control = snapshot(&scalar_control);
    let fresh_state = snapshot(&fresh);
    assert_eq!(after.1, fresh_state.1, "faulted scalar channel full-resets");
    assert_eq!(
        after.2, control.2,
        "the unaffected scalar right channel preserves its serialized state"
    );
    assert_ne!(
        after.1, control.1,
        "the faulted scalar left channel differs from no-fault control"
    );

    let values: [Values; 8] = core::array::from_fn(|track| {
        let mut values = active_values();
        support::set_parameter(&mut values, 0, -20.0 - track as f32, -20.0 - track as f32);
        support::set_parameter(&mut values, 2, 40.0, 40.0);
        values
    });
    let mut control =
        prepare_bank_native(&values, LinkMode::DualMono).expect("the build's own width binds");
    let mut bank =
        prepare_bank_native(&values, LinkMode::DualMono).expect("the build's own width binds");
    let warm_left = packed(&[&[0.01; 17]; NATIVE_LANES]);
    let warm_right = warm_left.clone();
    let mut control_warm_left = warm_left.clone();
    let mut control_warm_right = warm_right.clone();
    let mut bank_warm_left = warm_left;
    let mut bank_warm_right = warm_right;
    let bank_automation_one = retarget_spans(0);
    let bank_automation: [PreparedAutomationSpan; 8 * NATIVE_LANES] =
        core::array::from_fn(|index| bank_automation_one[index % bank_automation_one.len()]);
    let bank_automation_offsets: [u32; NATIVE_LANES + 1] =
        core::array::from_fn(|index| (index * bank_automation_one.len()) as u32);
    render_bank_with_automation(
        control.processor.as_mut(),
        &mut control_warm_left,
        &mut control_warm_right,
        17,
        &bank_automation,
        &bank_automation_offsets,
    );
    render_bank_with_automation(
        bank.processor.as_mut(),
        &mut bank_warm_left,
        &mut bank_warm_right,
        17,
        &bank_automation,
        &bank_automation_offsets,
    );
    let before_bank: Vec<_> = (0..NATIVE_LANES as u32)
        .map(|track| snapshot_bank(&bank, track))
        .collect();
    assert!(before_bank.iter().all(|payload| word(&payload.1, 0) != 0));
    for (track, payload) in before_bank.iter().enumerate() {
        for ramp in 0..4 {
            let slot = 6 + ramp * 4;
            assert_ne!(
                word(&payload.1, slot + 2),
                0,
                "track {track}: left ramp {ramp} step"
            );
            assert_ne!(
                word(&payload.2, slot + 2),
                0,
                "track {track}: right ramp {ramp} step"
            );
            assert_eq!(
                word(&payload.1, slot + 3),
                47.0_f32.to_bits(),
                "track {track}: left ramp {ramp} remaining"
            );
            assert_eq!(
                word(&payload.2, slot + 3),
                47.0_f32.to_bits(),
                "track {track}: right ramp {ramp} remaining"
            );
        }
    }
    let mut packed_left = packed(&[&[0.2; 128]; NATIVE_LANES]);
    let mut packed_right = packed_left.clone();
    packed_left[3] = f32::INFINITY;
    let mut control_left = packed(&[&[0.2; 128]; NATIVE_LANES]);
    let mut control_right = control_left.clone();
    let offsets = [0_u32; NATIVE_LANES + 1];
    let control_report = control.processor.process_bank(
        EffectBankProcessBlock::new(
            &mut control_left,
            &mut control_right,
            None,
            128,
            native_width(),
            17,
            &[],
            &offsets,
            128,
        )
        .unwrap(),
    );
    let report = bank.processor.process_bank(
        EffectBankProcessBlock::new(
            &mut packed_left,
            &mut packed_right,
            None,
            128,
            native_width(),
            17,
            &[],
            &offsets,
            128,
        )
        .unwrap(),
    );
    assert_eq!(report.reports[3].nonfinite_left_blocks, 128);
    for track in 0..NATIVE_LANES {
        if track != 3 {
            assert_bits_eq(
                &track_of(&packed_left, track, NATIVE_LANES),
                &track_of(&control_left, track, NATIVE_LANES),
                &format!("peer track {track} left PCM"),
            );
            assert_eq!(
                snapshot_bank(&bank, track as u32),
                snapshot_bank(&control, track as u32),
                "peer track {track} state"
            );
        }
    }
    assert_eq!(report.reports[3].nonfinite_right_blocks, 0);
    assert_eq!(control_report.reports[3].nonfinite_right_blocks, 0);
    assert_eq!(
        snapshot_bank(&bank, 3).2,
        snapshot_bank(&control, 3).2,
        "fault lane right serialized state remains equal to no-fault control"
    );
    assert_bits_eq(
        &track_of(&packed_right, 3, NATIVE_LANES),
        &track_of(&control_right, 3, NATIVE_LANES),
        "fault lane right PCM remains live",
    );
    let fresh_bank =
        prepare_bank_native(&values, LinkMode::DualMono).expect("the build's own width binds");
    assert_eq!(
        snapshot_bank(&bank, 3).1,
        snapshot_bank(&fresh_bank, 3).1,
        "fault lane left full-resets to prepared defaults"
    );
}

#[test]
fn scalar_and_bank_state_payloads_interchange_without_changing_audio() {
    let values = active_values();
    let source_left = noise(41, 160, 0.3);
    let source_right = noise(42, 160, 0.3);

    // Scalar -> bank: restore one scalar track after a partial render and continue it in a bank of
    // the build's own width (W8 in the 8-lane (AVX2) build, W4 in a 4-lane one).
    let mut scalar = prepare(request(&values));
    let mut scalar_prefix_left = source_left[..17].to_vec();
    let mut scalar_prefix_right = source_right[..17].to_vec();
    render_scalar_sidechain(
        &mut scalar,
        &mut scalar_prefix_left,
        &mut scalar_prefix_right,
        None,
        17,
        &[],
        0,
    );
    let scalar_payload = snapshot(&scalar);
    let mut scalar_expected_left = source_left[17..].to_vec();
    let mut scalar_expected_right = source_right[17..].to_vec();
    render_scalar_sidechain(
        &mut scalar,
        &mut scalar_expected_left,
        &mut scalar_expected_right,
        None,
        128,
        &[],
        17,
    );

    let bank_values = [values; 8];
    let mut scalar_to_bank =
        prepare_bank_native(&bank_values, LinkMode::DualMono).expect("the build's own width binds");
    let sizes = scalar_to_bank.metadata.program_key.state_sizes;
    scalar_to_bank
        .processor
        .restore_track_state_payload(
            3,
            STATE_LAYOUT_VERSION,
            StatePayloadInput::new(
                &scalar_payload.0,
                &scalar_payload.1,
                &scalar_payload.2,
                sizes,
            )
            .unwrap(),
        )
        .unwrap();
    let mut bank_left = packed(&[&source_left[17..]; NATIVE_LANES]);
    let mut bank_right = packed(&[&source_right[17..]; NATIVE_LANES]);
    render_bank(
        scalar_to_bank.processor.as_mut(),
        &mut bank_left,
        &mut bank_right,
        128,
    );
    assert_bits_eq(
        &track_of(&bank_left, 3, NATIVE_LANES),
        &scalar_expected_left,
        "scalar to bank left",
    );
    assert_bits_eq(
        &track_of(&bank_right, 3, NATIVE_LANES),
        &scalar_expected_right,
        "scalar to bank right",
    );

    // Bank -> scalar: snapshot the same track after a partial bank render and continue it in W1.
    let mut bank =
        prepare_bank_native(&bank_values, LinkMode::DualMono).expect("the build's own width binds");
    let mut bank_prefix_left = packed(&[&source_left[..17]; NATIVE_LANES]);
    let mut bank_prefix_right = packed(&[&source_right[..17]; NATIVE_LANES]);
    render_bank(
        bank.processor.as_mut(),
        &mut bank_prefix_left,
        &mut bank_prefix_right,
        17,
    );
    let bank_payload = snapshot_bank(&bank, 3);
    let mut bank_to_scalar = prepare(request(&values));
    let sizes = bank_to_scalar.metadata.state_sizes;
    bank_to_scalar
        .processor
        .restore_state_payload(
            STATE_LAYOUT_VERSION,
            StatePayloadInput::new(&bank_payload.0, &bank_payload.1, &bank_payload.2, sizes)
                .unwrap(),
        )
        .unwrap();
    let mut scalar_left = source_left[17..].to_vec();
    let mut scalar_right = source_right[17..].to_vec();
    render_scalar_sidechain(
        &mut bank_to_scalar,
        &mut scalar_left,
        &mut scalar_right,
        None,
        128,
        &[],
        17,
    );
    let mut bank_continuation_left = packed(&[&source_left[17..]; NATIVE_LANES]);
    let mut bank_continuation_right = packed(&[&source_right[17..]; NATIVE_LANES]);
    render_bank(
        bank.processor.as_mut(),
        &mut bank_continuation_left,
        &mut bank_continuation_right,
        128,
    );
    assert_bits_eq(
        &scalar_left,
        &track_of(&bank_continuation_left, 3, NATIVE_LANES),
        "bank to scalar left",
    );
    assert_bits_eq(
        &scalar_right,
        &track_of(&bank_continuation_right, 3, NATIVE_LANES),
        "bank to scalar right",
    );
}

#[test]
fn bank_restore_of_one_track_does_not_mutate_peers() {
    let values: [Values; 8] = core::array::from_fn(|_| initial_values());
    let mut donor_bank =
        prepare_bank_native(&values, LinkMode::DualMono).expect("the build's own width binds");
    let mut target_bank =
        prepare_bank_native(&values, LinkMode::DualMono).expect("the build's own width binds");
    let mut left = packed(&[&[0.1; 128]; NATIVE_LANES]);
    let mut right = left.clone();
    render_bank(donor_bank.processor.as_mut(), &mut left, &mut right, 128);
    let donor = snapshot_bank(&donor_bank, 3);
    // Track 4 in the 8-lane (AVX2) build, track 0 in a 4-lane (NEON/simd128) one (#1112).
    let peer = 4 % NATIVE_LANES as u32;
    let peer_before = snapshot_bank(&target_bank, peer);
    let sizes = target_bank.metadata.program_key.state_sizes;
    target_bank
        .processor
        .restore_track_state_payload(
            3,
            STATE_LAYOUT_VERSION,
            StatePayloadInput::new(&donor.0, &donor.1, &donor.2, sizes).unwrap(),
        )
        .unwrap();
    assert_eq!(snapshot_bank(&target_bank, peer), peer_before);
}

/// Issue #1411 D1: a ramp word one ulp outside its domain is refused even while the ramp moves.
///
/// From the effect's own snapshot with all four ramps in flight (threshold, ratio, range and
/// hysteresis on both channels), each left ramp's `current` in turn is written one ulp outside
/// each edge of its parameter's domain. The restore refuses it with `effect.state.parameter` and
/// leaves the scalar instance and a bank track unchanged; the same payload with `current` on the
/// edge restores. Red when `parse_lane` exempts a moving `current` from the domain or keeps a
/// rounding budget for it.
#[test]
fn a_moving_ramp_word_past_its_domain_is_refused() {
    const RAMPS: usize = 4;
    let values = active_values();
    let mut effect = prepare(request(&values));
    let mut left = noise(301, 9, 0.4);
    let mut right = noise(302, 9, 0.4);
    render_scalar_sidechain(
        &mut effect,
        &mut left,
        &mut right,
        None,
        9,
        &retarget_spans(0),
        0,
    );
    let saved = snapshot(&effect);
    let sizes = effect.metadata.state_sizes;
    let ramp_word = |index: usize| 6 + index * 4;
    for index in 0..RAMPS {
        assert_ne!(
            f32::from_bits(word(&saved.1, ramp_word(index) + 3)),
            0.0,
            "ramp {index} is in flight"
        );
    }
    let with_current = |index: usize, current: f32| {
        let mut section = saved.1.clone();
        let at = ramp_word(index) * 4;
        section[at..at + 4].copy_from_slice(&current.to_le_bytes());
        section
    };

    fn input<'a>(saved: &'a (Vec<u8>, Vec<u8>, Vec<u8>), left: &'a [u8]) -> StatePayloadInput<'a> {
        StatePayloadInput {
            common: &saved.0,
            left,
            right: &saved.2,
        }
    }
    let bank_values: [Values; 8] = core::array::from_fn(|_| active_values());
    let mut bank = prepare_bank_native(&bank_values, LinkMode::DualMono);
    if let Some(bank) = bank.as_mut() {
        bank.processor
            .restore_track_state_payload(
                0,
                STATE_LAYOUT_VERSION,
                StatePayloadInput::new(&saved.0, &saved.1, &saved.2, sizes).expect("sizes"),
            )
            .expect("the scalar snapshot restores into a bank track");
    }
    let bank_saved = bank.as_ref().map(|bank| snapshot_bank(bank, 0));

    for index in 0..RAMPS {
        let row = &gate_expander::GATE_EXPANDER_PARAMETERS[index];
        let (low, high) = (row.minimum.expect("min"), row.maximum.expect("max"));
        for (outside, edge) in [(low.next_down(), low), (high.next_up(), high)] {
            let case = format!(
                "ramp {index}: current {outside:e} ({:#010x})",
                outside.to_bits()
            );
            let section = with_current(index, outside);
            assert_eq!(
                effect
                    .processor
                    .restore_state_payload(STATE_LAYOUT_VERSION, input(&saved, &section))
                    .expect_err(&case)
                    .code,
                "effect.state.parameter",
                "{case}"
            );
            assert_eq!(snapshot(&effect), saved, "{case}");
            if let (Some(bank), Some(bank_saved)) = (bank.as_mut(), bank_saved.as_ref()) {
                assert_eq!(
                    bank.processor
                        .restore_track_state_payload(
                            0,
                            STATE_LAYOUT_VERSION,
                            input(&saved, &section)
                        )
                        .expect_err(&case)
                        .code,
                    "effect.state.parameter",
                    "bank {case}"
                );
                assert_eq!(&snapshot_bank(bank, 0), bank_saved, "bank {case}");
            }
            let section = with_current(index, edge);
            effect
                .processor
                .restore_state_payload(STATE_LAYOUT_VERSION, input(&saved, &section))
                .unwrap_or_else(|error| panic!("ramp {index}: current {edge:e}: {}", error.code));
            effect
                .processor
                .restore_state_payload(STATE_LAYOUT_VERSION, input(&saved, &saved.1))
                .expect("own snapshot");
            if let Some(bank) = bank.as_mut() {
                bank.processor
                    .restore_track_state_payload(0, STATE_LAYOUT_VERSION, input(&saved, &section))
                    .unwrap_or_else(|error| {
                        panic!("bank ramp {index}: current {edge:e}: {}", error.code)
                    });
                bank.processor
                    .restore_track_state_payload(0, STATE_LAYOUT_VERSION, input(&saved, &saved.1))
                    .expect("own snapshot");
            }
        }
    }
}
