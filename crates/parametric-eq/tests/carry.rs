//! Issue #1278: a lane's snapshot, restored at any sample of a prepared-target ramp into a fresh
//! scalar instance or a fresh bank lane, continues exactly as the lane it was taken from.
//!
//! The plan-swap carry (#1269) moves a lane's state into a freshly prepared plan at a block
//! boundary, and the EQ is the console-slot effect on most tracks, so its mid-ramp restore is the
//! carry's most frequent one. The conformance differential reaches it at random boundaries; this
//! test walks every sample of three fixed ramps:
//!
//! * a bell moving in frequency, gain and Q (the case a restore that re-derives a moving band's
//!   step as `(target - current) / remaining` diverges on from sample 5 on);
//! * a 10 kHz bell at Q 0.1 swinging -24 dB to +24 dB, whose `f32` walk sits a few ulps above the
//!   design's norm limit: the restore refused it at every sample until #1278 attempt 3 gave the
//!   remaining path its own bound (`RAMP_PATH_NORM_TOLERANCE`);
//! * the first ramp retargeted after 17 samples, so the restored ramp starts from a point no
//!   design produced.
//!
//! Each restore must be accepted, and the restored instance and the restored bank lane must render
//! every following word bit for bit as the continuing lane does, at every width this build binds.

mod support;

use effect_contract::{
    BankWidth, EffectProcessBlock, InitialParameterValue, NativeEffectFactory, ParameterChannel,
    PrepareEffectBankRequest, PreparedEffect, PreparedEffectBank, StatePayloadInput,
    StatePayloadOutput,
};
use parametric_eq::{EqBandKind, ParametricEqFactory};
use support::{
    Payload, apply_prepared_targets, request, set_initial, single_section_values, snapshot,
};

/// Samples rendered before the ramp starts, so the integrators hold signal.
const WARM: usize = 37;

/// A deterministic test signal with no exact zero.
fn signal(frames: usize, seed: u32) -> Vec<f32> {
    let mut state = seed.wrapping_mul(2_654_435_761).wrapping_add(1);
    (0..frames)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            (state as f32 / u32::MAX as f32) * 1.6 - 0.8 + 1.0e-3
        })
        .collect()
}

/// Band one's frequency, gain and Q moved on both channels; the touched mask covers them.
fn moved(
    values: &[InitialParameterValue],
    frequency: f32,
    gain: f32,
    q: f32,
) -> (Vec<InitialParameterValue>, Vec<bool>) {
    let mut target = values.to_vec();
    let mut changed = vec![false; target.len()];
    for channel in [ParameterChannel::Left, ParameterChannel::Right] {
        set_initial(&mut target, 2, channel, frequency);
        set_initial(&mut target, 3, channel, gain);
        set_initial(&mut target, 4, channel, q);
    }
    // Parameters 2, 3 and 4 (frequency, gain, Q), left and right entries.
    for touched in &mut changed[4..10] {
        *touched = true;
    }
    (target, changed)
}

/// One ramp: the prepared configuration, and the targets applied after so many ramp samples.
struct Ramp {
    values: Vec<InitialParameterValue>,
    retargets: Vec<(usize, Vec<InitialParameterValue>, Vec<bool>)>,
}

fn ramps() -> Vec<Ramp> {
    let bell = single_section_values(EqBandKind::Bell, 1_000.0, 6.0, 1.0, 1.0);
    let wide = single_section_values(EqBandKind::Bell, 10_000.0, -24.0, 0.1, 1.0);
    let (first, first_changed) = moved(&bell, 3_100.0, -9.5, 2.3);
    let (second, second_changed) = moved(&first, 450.0, 11.0, 0.6);
    let (swing, swing_changed) = moved(&wide, 10_000.0, 24.0, 0.1);
    vec![
        // bell moving in frequency, gain and Q.
        Ramp {
            values: bell.clone(),
            retargets: vec![(0, first.clone(), first_changed.clone())],
        },
        // 10 kHz bell at Q 0.1 swinging -24 dB to +24 dB.
        Ramp {
            values: wide,
            retargets: vec![(0, swing, swing_changed)],
        },
        // bell retargeted 17 samples into its ramp.
        Ramp {
            values: bell,
            retargets: vec![(0, first, first_changed), (17, second, second_changed)],
        },
    ]
}

/// The continuing lane after `WARM` samples and `elapsed` ramp samples, retargets applied.
fn continuing(ramp: &Ramp, elapsed: usize) -> PreparedEffect {
    let mut effect = ParametricEqFactory
        .prepare(request(&ramp.values, false))
        .expect("prepare");
    let (mut left, mut right) = (signal(WARM, 1), signal(WARM, 2));
    effect.processor.process(
        EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("warm block"),
    );
    let (left, right) = (signal(elapsed, 3), signal(elapsed, 4));
    for sample in 0..elapsed.max(1) {
        for (at, values, changed) in &ramp.retargets {
            if *at == sample {
                assert!(apply_prepared_targets(&mut effect, values, changed) > 0);
            }
        }
        if sample < elapsed {
            let (mut l, mut r) = ([left[sample]], [right[sample]]);
            let first = (WARM + sample) as u64;
            effect.processor.process(
                EffectProcessBlock::new(&mut l, &mut r, None, first, &[], 128).expect("ramp"),
            );
        }
    }
    effect
}

/// A restore validates both channels before it commits either: a payload whose left section is a
/// mid-ramp snapshot and whose right section is refused leaves the receiver as it was, scalar and
/// bank lane alike.
#[test]
fn a_restore_refused_on_one_channel_moves_neither() {
    let ramp = &ramps()[0];
    let source = continuing(ramp, 9);
    let (common, left, mut right) = snapshot(&source);
    // Band one's `remaining` counter (section 1, word 14 of 19) past the 64-sample ramp.
    let at = 4 * (19 + 14);
    right[at..at + 4].copy_from_slice(&65_u32.to_le_bytes());
    let mut receiver = ParametricEqFactory
        .prepare(request(&ramp.values, false))
        .expect("prepare");
    let before = snapshot(&receiver);
    let sizes = receiver.metadata.state_sizes;
    let input = || StatePayloadInput::new(&common, &left, &right, sizes).expect("input");
    assert!(
        receiver
            .processor
            .restore_state_payload(1, input())
            .is_err()
    );
    assert_eq!(snapshot(&receiver), before, "the scalar instance moved");
    for &width in BankWidth::ALL {
        let lanes = width.lanes() as usize;
        let requests: Vec<_> = (0..lanes).map(|_| request(&ramp.values, false)).collect();
        let Some(mut bank) = ParametricEqFactory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: width.backend(),
                width,
                requests: &requests,
                active_mask: width.full_mask(),
            })
            .expect("bank request")
        else {
            continue;
        };
        let lane = lanes - 1;
        let before = bank_lane_snapshot(&bank, lane);
        assert!(
            bank.processor
                .restore_track_state_payload(lane as u32, 1, input())
                .is_err()
        );
        assert_eq!(
            bank_lane_snapshot(&bank, lane),
            before,
            "{width:?}: bank lane {lane} moved"
        );
    }
}

fn bank_lane_snapshot(bank: &PreparedEffectBank, lane: usize) -> Payload {
    let mut out = (
        [0_u8; support::COMMON_BYTES],
        [0_u8; support::LANE_BYTES],
        [0_u8; support::LANE_BYTES],
    );
    let sizes = bank.metadata.program_key.state_sizes;
    bank.processor
        .snapshot_track_state_payload(
            lane as u32,
            StatePayloadOutput::new(&mut out.0, &mut out.1, &mut out.2, sizes).expect("output"),
        )
        .expect("snapshot");
    out
}

/// The silence word is admitted only as the counter writes it: `+0.0`, or an integer in
/// `[1, 2^24]`. Each malformed word is refused and changes nothing (issue #1328, amendment A9).
#[test]
fn a_malformed_silence_word_is_refused() {
    let values = support::values();
    let source = ParametricEqFactory
        .prepare(request(&values, false))
        .expect("prepare");
    let good = snapshot(&source);
    for bad in [
        (-0.0_f32).to_bits(),
        0.5_f32.to_bits(),
        1_023.5_f32.to_bits(),
        (-1.0_f32).to_bits(),
        16_777_218.0_f32.to_bits(),
        f32::INFINITY.to_bits(),
        f32::NAN.to_bits(),
        1,
    ] {
        for channel in 0..2 {
            let mut payload = good;
            let section = if channel == 0 {
                &mut payload.1
            } else {
                &mut payload.2
            };
            section[support::SILENCE_WORD * 4..support::SILENCE_WORD * 4 + 4]
                .copy_from_slice(&bad.to_le_bytes());
            let mut target = ParametricEqFactory
                .prepare(request(&values, false))
                .expect("prepare");
            let before = snapshot(&target);
            let sizes = target.metadata.state_sizes;
            assert!(
                target
                    .processor
                    .restore_state_payload(
                        1,
                        StatePayloadInput::new(&payload.0, &payload.1, &payload.2, sizes)
                            .expect("input"),
                    )
                    .is_err(),
                "silence word {bad:#010x} on channel {channel} must be refused"
            );
            assert!(snapshot(&target) == before, "a refusal changes nothing");
        }
    }
    for counted in [0.0_f32, 1.0, 4_096.0, 16_777_216.0] {
        let mut payload = good;
        payload.1[support::SILENCE_WORD * 4..support::SILENCE_WORD * 4 + 4]
            .copy_from_slice(&counted.to_bits().to_le_bytes());
        let mut target = ParametricEqFactory
            .prepare(request(&values, false))
            .expect("prepare");
        let sizes = target.metadata.state_sizes;
        target
            .processor
            .restore_state_payload(
                1,
                StatePayloadInput::new(&payload.0, &payload.1, &payload.2, sizes).expect("input"),
            )
            .expect("a counted word restores");
        assert_eq!(
            support::word(&snapshot(&target).1, support::SILENCE_WORD),
            counted.to_bits()
        );
    }
}

/// Gate 7 (#1337, D5): the payload carries each general band's enable and family. A lane taken
/// mid-ramp of a live bell-to-notch change, restored into an instance prepared as the bell,
/// continues bit for bit and settles on the notch; an enable word other than `0` or `1`, or a
/// family word outside `1..=6`, is refused on either channel and moves nothing.
#[test]
fn a_restore_takes_enable_and_family_from_the_payload() {
    let bell = single_section_values(EqBandKind::Bell, 1_000.0, 6.0, 1.0, 1.0);
    let mut notch = bell.clone();
    let mut changed = vec![false; notch.len()];
    for (offset, channel) in [(0, ParameterChannel::Left), (1, ParameterChannel::Right)] {
        set_initial(&mut notch, 1, channel, EqBandKind::Notch as u32 as f32);
        changed[2 + offset] = true;
    }
    let ramp = Ramp {
        values: bell.clone(),
        retargets: vec![(0, notch, changed)],
    };
    let mut source = continuing(&ramp, 23);
    let payload = snapshot(&source);
    let mut restored = ParametricEqFactory
        .prepare(request(&bell, false))
        .expect("prepare");
    let sizes = restored.metadata.state_sizes;
    restored
        .processor
        .restore_state_payload(
            1,
            StatePayloadInput::new(&payload.0, &payload.1, &payload.2, sizes).expect("input"),
        )
        .expect("a mid-ramp family change restores");
    let rendered = [&mut source, &mut restored].map(|effect| {
        let (mut left, mut right) = (signal(96, 5), signal(96, 6));
        effect.processor.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 60, &[], 128).expect("tail"),
        );
        left.iter()
            .chain(&right)
            .map(|s| s.to_bits())
            .collect::<Vec<_>>()
    });
    assert_eq!(
        rendered[0], rendered[1],
        "the restored lane continues bit for bit"
    );
    let (continued, carried) = (snapshot(&source), snapshot(&restored));
    assert!(continued == carried, "and holds the same state");
    assert_eq!(
        support::word(&carried.1, support::BAND_CONFIG_WORD + 1),
        EqBandKind::Notch as u32,
        "the restored lane settled on the notch"
    );

    let good = snapshot(&source);
    for (offset, bad) in [(0, 2_u32), (1, 0), (1, 7)] {
        for channel in 0..2 {
            let mut payload = good;
            let section = if channel == 0 {
                &mut payload.1
            } else {
                &mut payload.2
            };
            let at = (support::BAND_CONFIG_WORD + 2 * 3 + offset) * 4;
            section[at..at + 4].copy_from_slice(&bad.to_le_bytes());
            let mut target = ParametricEqFactory
                .prepare(request(&bell, false))
                .expect("prepare");
            let before = snapshot(&target);
            assert_eq!(
                target.processor.restore_state_payload(
                    1,
                    StatePayloadInput::new(&payload.0, &payload.1, &payload.2, sizes)
                        .expect("input"),
                ),
                Err(effect_contract::StatePayloadError {
                    code: "effect.state.payload"
                }),
                "word {offset} = {bad} on channel {channel} must be refused"
            );
            assert!(snapshot(&target) == before, "a refusal changes nothing");
        }
    }
}
