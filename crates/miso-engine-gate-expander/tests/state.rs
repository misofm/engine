//! Gates 7.5 and 7.6: the reset kinds, snapshot/restore continuation, and D7 lane-local recovery.

mod support;

use miso_engine_effect_contract::{
    BankWidth, EffectBankProcessBlock, LinkMode, PreparedNativeEffectBank, ProcessReport,
    ResetKind, StatePayloadInput,
};
use support::{
    Values, active_values, assert_bits_eq, initial_values, noise, packed_w8, prepare,
    prepare_bank_w8, render_scalar_sidechain, request, retarget_spans, set_parameter, snapshot,
    snapshot_bank, track_of,
};

const LATENCY: usize = 480;
const RAMP_WORD: usize = 7;

fn word(payload: &[u8], index: usize) -> u32 {
    u32::from_le_bytes(payload[index * 4..index * 4 + 4].try_into().expect("word"))
}

fn float(payload: &[u8], index: usize) -> f32 {
    f32::from_bits(word(payload, index))
}

/// Eight tracks with distinct parameters, as the identity gate uses.
fn track_values() -> [Values; 8] {
    let lookaheads = [0.0, 2.0, 5.0, 10.0, 10.0, 5.0, 2.0, 0.0];
    core::array::from_fn(|track| {
        let mut values = active_values();
        set_parameter(&mut values, 0, -20.0 - track as f32, -22.0 - track as f32);
        set_parameter(&mut values, 5, 2.0 + track as f32, 3.0 + track as f32);
        set_parameter(&mut values, 7, lookaheads[track], lookaheads[7 - track]);
        values
    })
}

/// Asserts the runtime words of a payload are the prepared resting state.
fn assert_cleared_runtime(payload: &[u8], hold_samples: f32, context: &str) {
    assert_eq!(float(payload, 0).to_bits(), 0, "{context}: G is +0");
    assert_eq!(float(payload, 1), 1.0, "{context}: the gate rests open");
    assert_eq!(float(payload, 2), hold_samples, "{context}: hold reloaded");
    for index in 23..payload.len() / 4 {
        assert_eq!(word(payload, index), 0, "{context}: ring word {index}");
    }
}

#[test]
fn reset_kinds_are_word_exact() {
    let values = active_values();
    // `active_values` sets hold to 0 ms, which is a hold of zero samples.
    let hold_samples = 0.0;
    let mut effect = prepare(request(&values));
    let mut left = noise(3, 640, 0.5);
    let mut right = noise(4, 640, 0.5);
    let spans = retarget_spans(0);
    render_scalar_sidechain(effect.as_mut(), &mut left, &mut right, None, 128, &spans, 0);
    let (_, before_left, _) = snapshot(effect.as_ref());

    effect.reset(ResetKind::DiscontinuityKeepParameters);
    let (_, after_left, _) = snapshot(effect.as_ref());
    assert_cleared_runtime(&after_left, hold_samples, "discontinuity");
    for index in 3..=6 {
        assert_eq!(
            word(&after_left, index),
            word(&before_left, index),
            "discontinuity keeps unsmoothed parameter word {index}"
        );
    }
    for ramp in 0..4 {
        let slot = RAMP_WORD + ramp * 4;
        assert_eq!(
            word(&after_left, slot),
            word(&before_left, slot + 1),
            "discontinuity snaps ramp {ramp} to its target"
        );
        assert_eq!(
            word(&after_left, slot + 1),
            word(&before_left, slot + 1),
            "discontinuity keeps ramp {ramp} target"
        );
        assert_eq!(word(&after_left, slot + 2), 0, "ramp {ramp} step");
        assert_eq!(word(&after_left, slot + 3), 0, "ramp {ramp} remaining");
    }

    effect.reset(ResetKind::FullToDefaults);
    let (_, full_left, _) = snapshot(effect.as_ref());
    assert_cleared_runtime(&full_left, hold_samples, "full");
    let prepared = [-20.0_f32, 20.0, 48.0, 6.0, 1.0, 0.0, 5.0, 10.0];
    assert_eq!(float(&full_left, 3), prepared[7], "lookahead");
    assert_eq!(float(&full_left, 4), prepared[4], "attack");
    assert_eq!(float(&full_left, 5), prepared[5], "hold ms");
    assert_eq!(float(&full_left, 6), prepared[6], "release");
    for (ramp, expected) in prepared[..4].iter().enumerate() {
        let slot = RAMP_WORD + ramp * 4;
        assert_eq!(float(&full_left, slot), *expected, "ramp {ramp}");
        assert_eq!(float(&full_left, slot + 1), *expected, "target {ramp}");
        assert_eq!(word(&full_left, slot + 2), 0, "step {ramp}");
        assert_eq!(word(&full_left, slot + 3), 0, "remaining {ramp}");
    }
}

#[test]
fn reset_kinds_are_word_exact_for_every_lane_of_a_bank() {
    let values = track_values();
    let Some(mut bank) = prepare_bank_w8(&values, LinkMode::DualMono) else {
        return;
    };
    let mut left = packed_w8(&(0..8).map(|t| noise(20 + t, 256, 0.5)).collect::<Vec<_>>());
    let mut right = packed_w8(&(0..8).map(|t| noise(60 + t, 256, 0.5)).collect::<Vec<_>>());
    render_bank(bank.as_mut(), &mut left, &mut right, 128);
    bank.reset(ResetKind::FullToDefaults);
    for track in 0..8 {
        let (_, payload, _) = snapshot_bank(bank.as_ref(), track as u32);
        let hold = ((2.0 + track as f32) * 48.0 + 0.5).floor();
        assert_cleared_runtime(&payload, hold, &format!("track {track}"));
    }
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
            .expect("bank block"),
        );
        start = end;
    }
}

#[test]
fn active_restore_continues_against_uninterrupted() {
    const FRAMES: usize = 1_024;
    let values = active_values();
    let source_left = noise(101, FRAMES, 0.4);
    let source_right = noise(202, FRAMES, 0.4);
    let spans = retarget_spans(0);

    // Snapshot mid-ramp: a 17-frame block after a retarget leaves `remaining == 47`.
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
    let (common, left_payload, right_payload) = snapshot(donor.as_ref());
    assert_eq!(
        float(&left_payload, RAMP_WORD + 3),
        47.0,
        "the snapshot is taken mid-ramp"
    );
    assert_ne!(word(&left_payload, 0), 0, "the snapshot has a live gain");

    for &partition in &[1_usize, 63, 64, 128] {
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
                2,
                StatePayloadInput::new(&common, &left_payload, &right_payload, sizes)
                    .expect("sizes"),
            )
            .expect("restore");
        let mut actual_left = source_left[17..].to_vec();
        let mut actual_right = source_right[17..].to_vec();
        render_scalar_sidechain(
            restored.as_mut(),
            &mut actual_left,
            &mut actual_right,
            None,
            partition,
            &[],
            17,
        );
        assert_bits_eq(
            &actual_left,
            &expected_left[17..],
            &format!("restored left at partition {partition}"),
        );
        assert_bits_eq(
            &actual_right,
            &expected_right[17..],
            &format!("restored right at partition {partition}"),
        );
        assert_eq!(
            snapshot(restored.as_ref()),
            snapshot(uninterrupted.as_ref()),
            "restored state at partition {partition}"
        );
    }
}

#[test]
fn a_track_restores_into_a_bank_whose_cursor_is_elsewhere() {
    // The payload is cursor-normalised, so a track snapshotted from a scalar instance at one
    // position in its ring must continue identically inside a bank whose shared cursor is at
    // another. This is the property the layout-2 bump exists for.
    const FRAMES: usize = 1_024;
    let values = track_values();
    let Some(mut bank) = prepare_bank_w8(&values, LinkMode::DualMono) else {
        return;
    };
    // Advance the bank's shared cursor by an amount that is not a multiple of the ring length.
    let mut warm_left = vec![0.0_f32; 37 * 8];
    let mut warm_right = vec![0.0_f32; 37 * 8];
    render_bank(bank.as_mut(), &mut warm_left, &mut warm_right, 37);

    let source_left = noise(7, FRAMES, 0.4);
    let source_right = noise(8, FRAMES, 0.4);
    let mut donor = prepare(request(&values[3]));
    let mut donor_left = source_left[..300].to_vec();
    let mut donor_right = source_right[..300].to_vec();
    render_scalar_sidechain(
        donor.as_mut(),
        &mut donor_left,
        &mut donor_right,
        None,
        128,
        &[],
        0,
    );
    let (common, left_payload, right_payload) = snapshot(donor.as_ref());
    let sizes = bank.metadata().program_key.state_sizes;
    bank.restore_track_state_payload(
        3,
        2,
        StatePayloadInput::new(&common, &left_payload, &right_payload, sizes).expect("sizes"),
    )
    .expect("restore into a bank");

    let mut expected_left = source_left[300..].to_vec();
    let mut expected_right = source_right[300..].to_vec();
    render_scalar_sidechain(
        donor.as_mut(),
        &mut expected_left,
        &mut expected_right,
        None,
        128,
        &[],
        300,
    );
    let mut bank_left = packed_w8(&vec![source_left[300..].to_vec(); 8]);
    let mut bank_right = packed_w8(&vec![source_right[300..].to_vec(); 8]);
    render_bank(bank.as_mut(), &mut bank_left, &mut bank_right, 128);
    assert_bits_eq(
        &track_of(&bank_left, 3, 8),
        &expected_left,
        "restored bank track 3 left",
    );
    assert_bits_eq(
        &track_of(&bank_right, 3, 8),
        &expected_right,
        "restored bank track 3 right",
    );
}

#[test]
fn a_malformed_phase_word_rejects_and_leaves_both_lanes_untouched() {
    let values = active_values();
    let mut effect = prepare(request(&values));
    let mut left = noise(9, 256, 0.4);
    let mut right = noise(10, 256, 0.4);
    render_scalar_sidechain(effect.as_mut(), &mut left, &mut right, None, 128, &[], 0);
    let before = snapshot(effect.as_ref());

    let (common, mut left_payload, right_payload) = before.clone();
    left_payload[4..8].copy_from_slice(&0x3F80_0001_u32.to_le_bytes());
    let sizes = effect.metadata().state_sizes;
    let error = effect
        .restore_state_payload(
            2,
            StatePayloadInput::new(&common, &left_payload, &right_payload, sizes).expect("sizes"),
        )
        .expect_err("a phase word that is neither +0 nor 1.0 is rejected");
    assert_eq!(error.code, "effect.state.phase");
    assert_eq!(snapshot(effect.as_ref()), before, "restore is all-or-none");

    // The out-of-band version argument is checked before anything is read.
    let stale = effect
        .restore_state_payload(
            1,
            StatePayloadInput::new(&common, &before.1, &before.2, sizes).expect("sizes"),
        )
        .expect_err("layout 1 no longer restores");
    assert_eq!(stale.code, "effect.state.version");
}

#[test]
fn nonfinite_input_recovers_lane_locally_at_the_block_boundary() {
    const BLOCK: usize = 128;
    const BLOCKS: usize = 8;
    const FRAMES: usize = BLOCK * BLOCKS;
    let values = initial_values();
    let clean_left = noise(31, FRAMES, 0.3);
    let clean_right = noise(32, FRAMES, 0.3);

    let mut control = prepare(request(&values));
    let mut control_left = clean_left.clone();
    let mut control_right = clean_right.clone();
    render_scalar_sidechain(
        control.as_mut(),
        &mut control_left,
        &mut control_right,
        None,
        BLOCK,
        &[],
        0,
    );

    let mut effect = prepare(request(&values));
    let mut left = clean_left.clone();
    let mut right = clean_right.clone();
    left[0] = f32::NAN;
    let mut reports = Vec::new();
    for block in 0..BLOCKS {
        let range = block * BLOCK..(block + 1) * BLOCK;
        reports.push(render_scalar_sidechain(
            effect.as_mut(),
            &mut left[range.clone()],
            &mut right[range],
            None,
            BLOCK,
            &[],
            (block * BLOCK) as u64,
        ));
    }

    // The NaN is written into the main ring and only reaches the output N samples later, so the
    // three blocks before that are bit-identical to the control.
    let hit = LATENCY / BLOCK;
    for (block, report) in reports.iter().enumerate().take(hit) {
        let range = block * BLOCK..(block + 1) * BLOCK;
        assert_bits_eq(
            &left[range.clone()],
            &control_left[range],
            &format!("block {block} left before the NaN surfaces"),
        );
        assert_eq!(*report, ProcessReport::default(), "block {block}");
    }
    // The block the NaN exits in is zeroed on the left only, and reported once per frame.
    for sample in &left[hit * BLOCK..(hit + 1) * BLOCK] {
        assert_eq!(sample.to_bits(), 0, "the recovered block is all +0");
    }
    assert_eq!(
        reports[hit].recovered_left_samples, BLOCK as u64,
        "one report per frame of the failing block"
    );
    assert_eq!(
        reports[hit].recovered_right_samples, 0,
        "right is untouched"
    );
    assert_bits_eq(
        &right[hit * BLOCK..(hit + 1) * BLOCK],
        &control_right[hit * BLOCK..(hit + 1) * BLOCK],
        "the right channel of the failing block",
    );

    // After the reset the left lane behaves exactly like a fresh instance fed the same input.
    let mut fresh = prepare(request(&values));
    let mut fresh_left = clean_left[(hit + 1) * BLOCK..].to_vec();
    let mut fresh_right = clean_right[(hit + 1) * BLOCK..].to_vec();
    render_scalar_sidechain(
        fresh.as_mut(),
        &mut fresh_left,
        &mut fresh_right,
        None,
        BLOCK,
        &[],
        0,
    );
    assert_bits_eq(
        &left[(hit + 1) * BLOCK..],
        &fresh_left,
        "the left lane restarted from a cleared ring",
    );
}

#[test]
fn nonfinite_input_recovers_one_lane_of_a_bank() {
    const BLOCK: usize = 128;
    const BLOCKS: usize = 8;
    const FRAMES: usize = BLOCK * BLOCKS;
    let values: [Values; 8] = core::array::from_fn(|_| initial_values());
    let Some(mut control) = prepare_bank_w8(&values, LinkMode::DualMono) else {
        return;
    };
    let Some(mut bank) = prepare_bank_w8(&values, LinkMode::DualMono) else {
        return;
    };
    let sources: Vec<Vec<f32>> = (0..8).map(|t| noise(41 + t, FRAMES, 0.3)).collect();
    let mut control_left = packed_w8(&sources);
    let mut control_right = packed_w8(&sources);
    render_bank(
        control.as_mut(),
        &mut control_left,
        &mut control_right,
        BLOCK,
    );

    let mut left = packed_w8(&sources);
    let mut right = packed_w8(&sources);
    left[3] = f32::INFINITY;
    let offsets = [0_u32; 9];
    let hit = LATENCY / BLOCK;
    for block in 0..BLOCKS {
        let range = block * BLOCK * 8..(block + 1) * BLOCK * 8;
        let report = bank.process_bank(
            EffectBankProcessBlock::new(
                &mut left[range.clone()],
                &mut right[range],
                None,
                BLOCK as u32,
                BankWidth::Eight,
                (block * BLOCK) as u64,
                &[],
                &offsets,
                128,
            )
            .expect("bank block"),
        );
        for track in 0..8 {
            let expected = if block == hit && track == 3 {
                BLOCK as u64
            } else {
                0
            };
            assert_eq!(
                report.reports[track].recovered_left_samples, expected,
                "block {block} track {track}"
            );
            assert_eq!(report.reports[track].recovered_right_samples, 0);
        }
    }
    for track in 0..8 {
        if track == 3 {
            continue;
        }
        assert_bits_eq(
            &track_of(&left, track, 8),
            &track_of(&control_left, track, 8),
            &format!("track {track} is untouched by track 3's recovery"),
        );
    }
    assert_bits_eq(
        &track_of(&right, 3, 8),
        &track_of(&control_right, 3, 8),
        "track 3's right channel is untouched",
    );
}
