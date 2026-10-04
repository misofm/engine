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
    BankWidth, EffectBankProcessBlock, EffectProcessBlock, InitialParameterValue,
    NativeEffectFactory, ParameterChannel, PrepareEffectBankRequest, PreparedNativeEffect,
    PreparedNativeEffectBank, StatePayloadInput, StatePayloadOutput,
};
use parametric_eq::{EqBandKind, ParametricEqFactory};
use support::{
    Payload, apply_prepared_targets, request, set_initial, single_section_values, snapshot,
};

/// Samples rendered before the ramp starts, so the integrators hold signal.
const WARM: usize = 37;
/// Samples compared after each restore: past the 64-sample ramp's end.
const TAIL: usize = 100;

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
    name: &'static str,
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
        Ramp {
            name: "bell moving in frequency, gain and Q",
            values: bell.clone(),
            retargets: vec![(0, first.clone(), first_changed.clone())],
        },
        Ramp {
            name: "10 kHz bell at Q 0.1 swinging -24 dB to +24 dB",
            values: wide,
            retargets: vec![(0, swing, swing_changed)],
        },
        Ramp {
            name: "bell retargeted 17 samples into its ramp",
            values: bell,
            retargets: vec![(0, first, first_changed), (17, second, second_changed)],
        },
    ]
}

/// The continuing lane after `WARM` samples and `elapsed` ramp samples, retargets applied.
fn continuing(ramp: &Ramp, elapsed: usize) -> Box<dyn PreparedNativeEffect> {
    let mut effect = ParametricEqFactory
        .prepare(request(&ramp.values, false))
        .expect("prepare");
    let (mut left, mut right) = (signal(WARM, 1), signal(WARM, 2));
    effect.process(
        EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("warm block"),
    );
    let (left, right) = (signal(elapsed, 3), signal(elapsed, 4));
    for sample in 0..elapsed.max(1) {
        for (at, values, changed) in &ramp.retargets {
            if *at == sample {
                assert!(apply_prepared_targets(effect.as_mut(), values, changed) > 0);
            }
        }
        if sample < elapsed {
            let (mut l, mut r) = ([left[sample]], [right[sample]]);
            let first = (WARM + sample) as u64;
            effect.process(
                EffectProcessBlock::new(&mut l, &mut r, None, first, &[], 128).expect("ramp"),
            );
        }
    }
    effect
}

/// Renders `TAIL` frames through the restored bank lane `lane`, the other lanes fed another signal.
fn bank_tail(
    ramp: &Ramp,
    width: BankWidth,
    lane: usize,
    payload: &Payload,
    first: u64,
) -> Option<(Vec<f32>, Vec<f32>)> {
    let lanes = width.lanes() as usize;
    let requests: Vec<_> = (0..lanes).map(|_| request(&ramp.values, false)).collect();
    let mut bank = ParametricEqFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: width.backend(),
            width,
            requests: &requests,
            active_mask: width.full_mask(),
        })
        .expect("bank request")?;
    let sizes = bank.metadata().program_key.state_sizes;
    bank.restore_track_state_payload(
        lane as u32,
        1,
        StatePayloadInput::new(&payload.0, &payload.1, &payload.2, sizes).expect("input"),
    )
    .unwrap_or_else(|error| panic!("{width:?}: the bank lane refused ({})", error.code));
    let (tail_left, tail_right) = (signal(TAIL, 5), signal(TAIL, 6));
    let (other_left, other_right) = (signal(TAIL, 7), signal(TAIL, 8));
    let mut left = vec![0.0_f32; TAIL * lanes];
    let mut right = vec![0.0_f32; TAIL * lanes];
    for frame in 0..TAIL {
        for track in 0..lanes {
            let (l, r) = if track == lane {
                (tail_left[frame], tail_right[frame])
            } else {
                (other_left[frame], other_right[frame])
            };
            left[frame * lanes + track] = l;
            right[frame * lanes + track] = r;
        }
    }
    let offsets = vec![0_u32; lanes + 1];
    bank.process_bank(
        EffectBankProcessBlock::new(
            &mut left,
            &mut right,
            None,
            TAIL as u32,
            width,
            first,
            &[],
            &offsets,
            128,
        )
        .expect("bank block"),
    );
    let gather = |plane: &[f32]| (0..TAIL).map(|frame| plane[frame * lanes + lane]).collect();
    Some((gather(&left), gather(&right)))
}

fn same_bits(a: &[f32], b: &[f32]) -> bool {
    a.iter().zip(b).all(|(x, y)| x.to_bits() == y.to_bits())
}

#[test]
fn a_mid_ramp_restore_continues_bit_for_bit_at_every_sample() {
    let mut banks_bound = 0;
    let mut failures = Vec::new();
    for ramp in ramps() {
        for elapsed in 0..=64_usize {
            let mut lane = continuing(&ramp, elapsed);
            let payload = snapshot(lane.as_ref());
            let first = (WARM + elapsed) as u64;
            let mut restored = ParametricEqFactory
                .prepare(request(&ramp.values, false))
                .expect("prepare");
            let sizes = restored.metadata().state_sizes;
            if let Err(error) = restored.restore_state_payload(
                1,
                StatePayloadInput::new(&payload.0, &payload.1, &payload.2, sizes).expect("input"),
            ) {
                failures.push(format!(
                    "{}: sample {elapsed}: the scalar restore refused ({})",
                    ramp.name, error.code
                ));
                continue;
            }
            let (mut left, mut right) = (signal(TAIL, 5), signal(TAIL, 6));
            let (mut restored_left, mut restored_right) = (left.clone(), right.clone());
            lane.process(
                EffectProcessBlock::new(&mut left, &mut right, None, first, &[], 128)
                    .expect("tail"),
            );
            restored.process(
                EffectProcessBlock::new(
                    &mut restored_left,
                    &mut restored_right,
                    None,
                    first,
                    &[],
                    128,
                )
                .expect("tail"),
            );
            if !same_bits(&left, &restored_left) || !same_bits(&right, &restored_right) {
                failures.push(format!(
                    "{}: sample {elapsed}: the restored instance diverges from the continuing lane",
                    ramp.name
                ));
            }
            for &width in BankWidth::ALL {
                let target = width.lanes() as usize - 1;
                // A width this build does not execute declines by design (issue #1112).
                let Some((bank_left, bank_right)) =
                    bank_tail(&ramp, width, target, &payload, first)
                else {
                    continue;
                };
                banks_bound += 1;
                if !same_bits(&left, &bank_left) || !same_bits(&right, &bank_right) {
                    failures.push(format!(
                        "{}: sample {elapsed}: {width:?} bank lane {target} diverges from the \
                         continuing lane",
                        ramp.name
                    ));
                }
            }
        }
    }
    assert!(banks_bound > 0, "no width bound a bank");
    assert!(failures.is_empty(), "{failures:#?}");
}

/// A restore validates both channels before it commits either: a payload whose left section is a
/// mid-ramp snapshot and whose right section is refused leaves the receiver as it was, scalar and
/// bank lane alike.
#[test]
fn a_restore_refused_on_one_channel_moves_neither() {
    let ramp = &ramps()[0];
    let source = continuing(ramp, 9);
    let (common, left, mut right) = snapshot(source.as_ref());
    // Band one's `remaining` counter (section 1, word 14 of 19) past the 64-sample ramp.
    let at = 4 * (19 + 14);
    right[at..at + 4].copy_from_slice(&65_u32.to_le_bytes());
    let mut receiver = ParametricEqFactory
        .prepare(request(&ramp.values, false))
        .expect("prepare");
    let before = snapshot(receiver.as_ref());
    let sizes = receiver.metadata().state_sizes;
    let input = || StatePayloadInput::new(&common, &left, &right, sizes).expect("input");
    assert!(receiver.restore_state_payload(1, input()).is_err());
    assert_eq!(
        snapshot(receiver.as_ref()),
        before,
        "the scalar instance moved"
    );
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
        let before = bank_lane_snapshot(bank.as_ref(), lane);
        assert!(
            bank.restore_track_state_payload(lane as u32, 1, input())
                .is_err()
        );
        assert_eq!(
            bank_lane_snapshot(bank.as_ref(), lane),
            before,
            "{width:?}: bank lane {lane} moved"
        );
    }
}

fn bank_lane_snapshot(bank: &dyn PreparedNativeEffectBank, lane: usize) -> Payload {
    let mut out = (
        [0_u8; support::COMMON_BYTES],
        [0_u8; support::LANE_BYTES],
        [0_u8; support::LANE_BYTES],
    );
    let sizes = bank.metadata().program_key.state_sizes;
    bank.snapshot_track_state_payload(
        lane as u32,
        StatePayloadOutput::new(&mut out.0, &mut out.1, &mut out.2, sizes).expect("output"),
    )
    .expect("snapshot");
    out
}
