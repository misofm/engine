//! State continuation, resets, exact lengths, rollback, and lane-local recovery.

mod support;

use effect_contract::{
    BankWidth, EffectBankProcessBlock, LinkMode, PreparedNativeEffectBank, ResetKind,
    StatePayloadInput, StatePayloadOutput,
};
use gate_expander::STATE_LAYOUT_VERSION;
use support::{
    Values, active_values, assert_bits_eq, initial_values, noise, packed_w8, prepare,
    prepare_bank_w8, render_scalar_sidechain, request, snapshot, snapshot_bank, track_of,
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
    let frames = left.len() / 8;
    let offsets = [0_u32; 9];
    let mut start = 0;
    while start < frames {
        let end = (start + block).min(frames);
        bank.process_bank(
            EffectBankProcessBlock::new(
                &mut left[start * 8..end * 8],
                &mut right[start * 8..end * 8],
                None,
                (end - start) as u32,
                BankWidth::Eight,
                start as u64,
                &[],
                &offsets,
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
    values[10].value = 1.0;
    values[11].value = 1.0;
    let mut effect = prepare(request(&values));
    let mut left = noise(3, 128, 0.5);
    let mut right = noise(4, 128, 0.5);
    render_scalar_sidechain(effect.as_mut(), &mut left, &mut right, None, 128, &[], 0);
    let (_, before, _) = snapshot(effect.as_ref());
    effect.reset(ResetKind::DiscontinuityKeepParameters);
    let (_, discontinuity, _) = snapshot(effect.as_ref());
    assert_eq!(word(&discontinuity, 0), 0);
    assert_eq!(word(&discontinuity, 1), 1.0_f32.to_bits());
    assert_eq!(word(&discontinuity, 2), 48.0_f32.to_bits());
    assert_eq!(word(&discontinuity, 3), word(&before, 3));
    assert_eq!(word(&discontinuity, 4), word(&before, 4));
    assert_eq!(word(&discontinuity, 5), word(&before, 5));
    effect.reset(ResetKind::FullToDefaults);
    let (_, full, _) = snapshot(effect.as_ref());
    assert_eq!(word(&full, 0), 0);
    assert_eq!(word(&full, 1), 1.0_f32.to_bits());
    assert_eq!(word(&full, 2), 48.0_f32.to_bits());
    assert_eq!(word(&full, 3), 1.0_f32.to_bits());
    assert_eq!(word(&full, 4), 1.0_f32.to_bits());
    assert_eq!(word(&full, 5), 5.0_f32.to_bits());
}

#[test]
fn active_state_restore_continues_bit_exactly_across_partitions() {
    let values = active_values();
    let source_left = noise(101, 1024, 0.4);
    let source_right = noise(202, 1024, 0.4);
    let spans = support::retarget_spans(0);
    let mut donor = prepare(request(&values));
    let mut warm_left = source_left[..17].to_vec();
    let mut warm_right = source_right[..17].to_vec();
    render_scalar_sidechain(
        donor.as_mut(),
        &mut warm_left,
        &mut warm_right,
        None,
        17,
        &spans,
        0,
    );
    let payload = snapshot(donor.as_ref());
    assert_eq!(word(&payload.1, 6 + 3), 47.0_f32.to_bits());

    let mut uninterrupted = prepare(request(&values));
    let mut expected_left = source_left.clone();
    let mut expected_right = source_right.clone();
    render_scalar_sidechain(
        uninterrupted.as_mut(),
        &mut expected_left[..17],
        &mut expected_right[..17],
        None,
        17,
        &spans,
        0,
    );
    render_scalar_sidechain(
        uninterrupted.as_mut(),
        &mut expected_left[17..],
        &mut expected_right[17..],
        None,
        128,
        &[],
        17,
    );

    let mut restored = prepare(request(&values));
    let sizes = restored.metadata().state_sizes;
    restored
        .restore_state_payload(
            STATE_LAYOUT_VERSION,
            StatePayloadInput::new(&payload.0, &payload.1, &payload.2, sizes).unwrap(),
        )
        .unwrap();
    let mut actual_left = source_left[17..].to_vec();
    let mut actual_right = source_right[17..].to_vec();
    render_scalar_sidechain(
        restored.as_mut(),
        &mut actual_left,
        &mut actual_right,
        None,
        128,
        &[],
        17,
    );
    assert_bits_eq(&actual_left, &expected_left[17..], "restored left");
    assert_bits_eq(&actual_right, &expected_right[17..], "restored right");
    assert_eq!(
        snapshot(restored.as_ref()),
        snapshot(uninterrupted.as_ref())
    );
}

#[test]
fn old_lengths_and_one_byte_short_payloads_reject_scalar_and_bank() {
    for rate in [44_100, 48_000, 88_200, 96_000] {
        let values = initial_values();
        let mut scalar = prepare(support::request_at_rate(&values, rate));
        let sizes = scalar.metadata().state_sizes;
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
        if rate == 48_000 {
            let values: [Values; 8] = core::array::from_fn(|_| initial_values());
            if let Some(mut bank) = prepare_bank_w8(&values, LinkMode::DualMono) {
                let sizes = bank.metadata().program_key.state_sizes;
                assert!(
                    bank.restore_track_state_payload(
                        0,
                        STATE_LAYOUT_VERSION,
                        StatePayloadInput {
                            common: &common,
                            left: &old_lane,
                            right: &old_lane
                        },
                    )
                    .is_err()
                );
                assert_eq!(sizes.left_bytes, 88);
            }
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
    let before = snapshot(effect.as_ref());
    let mut right = before.2.clone();
    right[87] = 0xff;
    let sizes = effect.metadata().state_sizes;
    let input = StatePayloadInput::new(&before.0, &before.1, &right, sizes).unwrap();
    assert!(
        effect
            .restore_state_payload(STATE_LAYOUT_VERSION, input)
            .is_err()
    );
    assert_eq!(snapshot(effect.as_ref()), before);
}

#[test]
fn scalar_and_bank_recovery_is_channel_and_lane_local() {
    let values = initial_values();
    let mut scalar = prepare(request(&values));
    let mut left = vec![0.2; 128];
    let mut right = vec![0.2; 128];
    left[0] = f32::NAN;
    let report = render_scalar_sidechain(scalar.as_mut(), &mut left, &mut right, None, 128, &[], 0);
    assert_eq!(report.nonfinite_left_blocks, 128);
    assert!(left.iter().all(|sample| sample.to_bits() == 0));
    assert!(right.iter().all(|sample| sample.is_finite()));

    let values: [Values; 8] = core::array::from_fn(|_| initial_values());
    let Some(mut bank) = prepare_bank_w8(&values, LinkMode::DualMono) else {
        return;
    };
    let mut packed_left = packed_w8(&vec![vec![0.2; 128]; 8]);
    let mut packed_right = packed_left.clone();
    packed_left[3] = f32::INFINITY;
    let offsets = [0_u32; 9];
    let report = bank.process_bank(
        EffectBankProcessBlock::new(
            &mut packed_left,
            &mut packed_right,
            None,
            128,
            BankWidth::Eight,
            0,
            &[],
            &offsets,
            128,
        )
        .unwrap(),
    );
    assert_eq!(report.reports[3].nonfinite_left_blocks, 128);
    for track in 0..8 {
        if track != 3 {
            assert!(
                track_of(&packed_left, track, 8)
                    .iter()
                    .all(|sample| sample.is_finite())
            );
        }
    }
}

#[test]
fn scalar_and_bank_state_payloads_interchange_without_changing_audio() {
    let values = active_values();
    let source_left = noise(41, 160, 0.3);
    let source_right = noise(42, 160, 0.3);

    // Scalar -> bank: restore one scalar track after a partial render and continue it in W8.
    let mut scalar = prepare(request(&values));
    let mut scalar_prefix_left = source_left[..17].to_vec();
    let mut scalar_prefix_right = source_right[..17].to_vec();
    render_scalar_sidechain(
        scalar.as_mut(),
        &mut scalar_prefix_left,
        &mut scalar_prefix_right,
        None,
        17,
        &[],
        0,
    );
    let scalar_payload = snapshot(scalar.as_ref());
    let mut scalar_expected_left = source_left[17..].to_vec();
    let mut scalar_expected_right = source_right[17..].to_vec();
    render_scalar_sidechain(
        scalar.as_mut(),
        &mut scalar_expected_left,
        &mut scalar_expected_right,
        None,
        128,
        &[],
        17,
    );

    let bank_values = [values; 8];
    let Some(mut scalar_to_bank) = prepare_bank_w8(&bank_values, LinkMode::DualMono) else {
        return;
    };
    let sizes = scalar_to_bank.metadata().program_key.state_sizes;
    scalar_to_bank
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
    let mut bank_left = packed_w8(&vec![source_left[17..].to_vec(); 8]);
    let mut bank_right = packed_w8(&vec![source_right[17..].to_vec(); 8]);
    render_bank(&mut *scalar_to_bank, &mut bank_left, &mut bank_right, 128);
    assert_bits_eq(
        &track_of(&bank_left, 3, 8),
        &scalar_expected_left,
        "scalar to bank left",
    );
    assert_bits_eq(
        &track_of(&bank_right, 3, 8),
        &scalar_expected_right,
        "scalar to bank right",
    );

    // Bank -> scalar: snapshot the same track after a partial W8 render and continue it in W1.
    let Some(mut bank) = prepare_bank_w8(&bank_values, LinkMode::DualMono) else {
        return;
    };
    let mut bank_prefix_left = packed_w8(&vec![source_left[..17].to_vec(); 8]);
    let mut bank_prefix_right = packed_w8(&vec![source_right[..17].to_vec(); 8]);
    render_bank(
        &mut *bank,
        &mut bank_prefix_left,
        &mut bank_prefix_right,
        17,
    );
    let bank_payload = snapshot_bank(&*bank, 3);
    let mut bank_to_scalar = prepare(request(&values));
    let sizes = bank_to_scalar.metadata().state_sizes;
    bank_to_scalar
        .restore_state_payload(
            STATE_LAYOUT_VERSION,
            StatePayloadInput::new(&bank_payload.0, &bank_payload.1, &bank_payload.2, sizes)
                .unwrap(),
        )
        .unwrap();
    let mut scalar_left = source_left[17..].to_vec();
    let mut scalar_right = source_right[17..].to_vec();
    render_scalar_sidechain(
        bank_to_scalar.as_mut(),
        &mut scalar_left,
        &mut scalar_right,
        None,
        128,
        &[],
        17,
    );
    let mut bank_continuation_left = packed_w8(&vec![source_left[17..].to_vec(); 8]);
    let mut bank_continuation_right = packed_w8(&vec![source_right[17..].to_vec(); 8]);
    render_bank(
        &mut *bank,
        &mut bank_continuation_left,
        &mut bank_continuation_right,
        128,
    );
    assert_bits_eq(
        &scalar_left,
        &track_of(&bank_continuation_left, 3, 8),
        "bank to scalar left",
    );
    assert_bits_eq(
        &scalar_right,
        &track_of(&bank_continuation_right, 3, 8),
        "bank to scalar right",
    );
}

#[test]
fn bank_restore_of_one_track_does_not_mutate_peers() {
    let values: [Values; 8] = core::array::from_fn(|_| initial_values());
    let Some(mut donor_bank) = prepare_bank_w8(&values, LinkMode::DualMono) else {
        return;
    };
    let Some(mut target_bank) = prepare_bank_w8(&values, LinkMode::DualMono) else {
        return;
    };
    let mut left = packed_w8(&vec![vec![0.1; 128]; 8]);
    let mut right = left.clone();
    render_bank(&mut *donor_bank, &mut left, &mut right, 128);
    let donor = snapshot_bank(&*donor_bank, 3);
    let peer_before = snapshot_bank(&*target_bank, 4);
    let sizes = target_bank.metadata().program_key.state_sizes;
    target_bank
        .restore_track_state_payload(
            3,
            STATE_LAYOUT_VERSION,
            StatePayloadInput::new(&donor.0, &donor.1, &donor.2, sizes).unwrap(),
        )
        .unwrap();
    assert_eq!(snapshot_bank(&*target_bank, 4), peer_before);
}
