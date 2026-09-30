#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! Width and bank gates: the same body at `WIDTH` 1, 4 and 8, proven by `to_bits` identity.
//!
//! Decision D5 replaces the old tolerance-based cross-backend comparison with bit identity. That is
//! affordable here because there is exactly one realization: the scalar effect is
//! `Channel<f32, 1>` and a bank is `Channel<Simd4, 4>` or `Channel<Simd8, 8>` of the same generic
//! body, so a difference would be a `Lane` defect, not an effect defect.
//!
//! The digests fold every word through `dsp_reference::class_a`: class-A identity reads every NaN
//! as one value (issue #1065), because a NaN's sign and payload are the CPU's choice.

mod support;

use dsp_reference::class_a;
use effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectProcessBlock, NativeEffectFactory, ParameterChannel,
    PrepareEffectBankRequest, PreparedEffectTarget, PreparedNativeEffectBank, StatePayloadInput,
    StatePayloadOutput, TailSamples,
};
use lane::Backend;
use parametric_eq::{EqBandKind, PARAMETRIC_EQ_DESCRIPTOR, ParametricEqFactory, design_svf};
use sha2::{Digest, Sha256};
use support::{
    COMMON_BYTES, LANE_BYTES, Payload, apply_prepared_targets, apply_prepared_targets_lane,
    request, set_initial, snapshot, values,
};

/// The bank width and backend this build actually executes, or `None` on a scalar-only target.
fn native_bank() -> Option<(BankWidth, Backend)> {
    let backend = Backend::current();
    BankWidth::for_backend(backend).map(|width| (width, backend))
}

/// A backend this build cannot execute, for the declining path.
fn foreign_bank() -> (BankWidth, Backend) {
    match native_bank() {
        Some((BankWidth::Eight, _)) => (BankWidth::Four, Backend::Simd4),
        _ => (BankWidth::Eight, Backend::Simd8),
    }
}

fn hpf_target() -> PreparedEffectTarget {
    let words = design_svf(
        EqBandKind::HighPass,
        1_000.0,
        0.0,
        1.0,
        1.0,
        engine::SampleRateHz(48_000),
    )
    .expect("target design");
    let mut encoded = [0_u32; effect_contract::PREPARED_EFFECT_TARGET_WORDS];
    encoded[..6].copy_from_slice(&[
        1,
        EqBandKind::HighPass as u32,
        1_000.0_f32.to_bits(),
        0.0_f32.to_bits(),
        1.0_f32.to_bits(),
        1.0_f32.to_bits(),
    ]);
    for (destination, source) in encoded[6..].iter_mut().zip(words.to_array()) {
        *destination = source.to_bits();
    }
    PreparedEffectTarget {
        slot: 0,
        channel: ParameterChannel::Left,
        words: encoded,
    }
}

fn state_word(payload: &[u8], index: usize) -> u32 {
    u32::from_le_bytes(
        payload[index * 4..index * 4 + 4]
            .try_into()
            .expect("state word"),
    )
}

#[test]
fn bank_prepared_target_updates_only_the_selected_lane() {
    let Some((width, backend)) = native_bank() else {
        return;
    };
    let lanes = width.lanes() as usize;
    let values_by_track: Vec<_> = (0..lanes).map(|_| values()).collect();
    let requests: Vec<_> = values_by_track
        .iter()
        .map(|values| request(values, false))
        .collect();
    let factory = ParametricEqFactory;
    let mut bank = factory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
            active_mask: width.full_mask(),
        })
        .expect("bank request")
        .expect("native bank");
    let target = hpf_target();
    bank.apply_prepared_target_lane(0, &target)
        .expect("bank target");
    let (_, selected_left, _) = snapshot_bank(&*bank, 0);
    assert_eq!(state_word(&selected_left, 114), 1);
    if lanes > 1 {
        let (_, untouched_left, _) = snapshot_bank(&*bank, 1);
        assert_eq!(state_word(&untouched_left, 114), 0);
    }
}

/// A distinct four-band configuration per track, so no two lanes share coefficients.
fn configured_values(track: usize) -> Vec<effect_contract::InitialParameterValue> {
    let mut values = values();
    for band in 0..4 {
        let base = band * 6;
        set_initial(&mut values, base, ParameterChannel::Left, 1.0);
        set_initial(&mut values, base, ParameterChannel::Right, 1.0);
        set_initial(
            &mut values,
            base + 1,
            ParameterChannel::Left,
            (band % 6 + 1) as f32,
        );
        set_initial(
            &mut values,
            base + 1,
            ParameterChannel::Right,
            ((band + 3) % 6 + 1) as f32,
        );
        set_initial(
            &mut values,
            base + 2,
            ParameterChannel::Left,
            120.0 * (band + 1) as f32 + track as f32 * 37.0,
        );
        set_initial(
            &mut values,
            base + 2,
            ParameterChannel::Right,
            900.0 * (band + 1) as f32 + track as f32 * 53.0,
        );
        set_initial(
            &mut values,
            base + 3,
            ParameterChannel::Left,
            -9.0 + track as f32 + band as f32,
        );
        set_initial(
            &mut values,
            base + 4,
            ParameterChannel::Left,
            0.5 + track as f32 * 0.1 + band as f32 * 0.25,
        );
        set_initial(
            &mut values,
            base + 5,
            ParameterChannel::Left,
            0.2 + band as f32 * 0.2,
        );
    }
    values
}

fn snapshot_bank(bank: &dyn PreparedNativeEffectBank, track: u32) -> Payload {
    let mut common = [0_u8; COMMON_BYTES];
    let mut left = [0_u8; LANE_BYTES];
    let mut right = [0_u8; LANE_BYTES];
    let sizes = bank.metadata().program_key.state_sizes;
    bank.snapshot_track_state_payload(
        track,
        StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes)
            .expect("bank state output"),
    )
    .expect("bank snapshot");
    (common, left, right)
}

/// E7: every available width reproduces the scalar instantiation bit for bit, with ramps in flight.
#[test]
fn every_width_matches_the_scalar_instantiation() {
    let Some((width, backend)) = native_bank() else {
        return;
    };
    let lanes = width.lanes() as usize;
    let factory = ParametricEqFactory;
    let values_by_track: Vec<_> = (0..lanes).map(configured_values).collect();
    let requests: Vec<_> = values_by_track
        .iter()
        .map(|values| request(values, false))
        .collect();
    let mut bank = factory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
            active_mask: width.full_mask(),
        })
        .expect("valid bank request")
        .expect("the native width must bind");
    assert_eq!(bank.metadata().width, width);
    assert_eq!(bank.metadata().program_key.tail, TailSamples::Infinite);

    let mut scalar: Vec<_> = values_by_track
        .iter()
        .map(|values| {
            factory
                .prepare(request(values, false))
                .expect("scalar prepare")
        })
        .collect();

    // Blocks of 16, 128, 64 and 128 frames, with prepared edits in the first and third block for
    // different tracks, so ramps of different ages coexist inside one bank block.
    let mut position = 0_u64;
    for (index, frames) in [16_usize, 128, 64, 128].into_iter().enumerate() {
        for track in 0..lanes {
            if (index == 0 && track % 2 == 0) || (index == 2 && track % 2 == 1) {
                let mut target_values = values_by_track[track].clone();
                set_initial(
                    &mut target_values,
                    3,
                    ParameterChannel::Left,
                    -4.0 + track as f32 * 0.5,
                );
                set_initial(
                    &mut target_values,
                    4,
                    ParameterChannel::Right,
                    0.8 + track as f32 * 0.01,
                );
                let mut changed = vec![false; target_values.len()];
                changed[3 * 2] = true;
                changed[4 * 2 + 1] = true;
                apply_prepared_targets(scalar[track].as_mut(), &target_values, &changed);
                apply_prepared_targets_lane(&mut *bank, track, 48_000, &target_values, &changed);
            }
        }

        let mut bank_left = vec![0.0_f32; frames * lanes];
        let mut bank_right = vec![0.0_f32; frames * lanes];
        let empty_offsets = vec![0_u32; lanes + 1];
        let mut scalar_left = vec![vec![0.0_f32; frames]; lanes];
        let mut scalar_right = vec![vec![0.0_f32; frames]; lanes];
        for frame in 0..frames {
            for track in 0..lanes {
                let sample =
                    ((position as usize + frame) as f32 * 0.017 + track as f32 * 0.31).sin() * 0.6;
                bank_left[frame * lanes + track] = sample;
                bank_right[frame * lanes + track] = -sample * 0.75;
                scalar_left[track][frame] = sample;
                scalar_right[track][frame] = -sample * 0.75;
            }
        }

        let mut scalar_reports = Vec::with_capacity(lanes);
        for track in 0..lanes {
            scalar_reports.push(
                scalar[track].process(
                    EffectProcessBlock::new(
                        &mut scalar_left[track],
                        &mut scalar_right[track],
                        None,
                        position,
                        &[],
                        128,
                    )
                    .expect("scalar block"),
                ),
            );
        }
        let bank_report = bank.process_bank(
            EffectBankProcessBlock::new(
                &mut bank_left,
                &mut bank_right,
                None,
                frames as u32,
                width,
                position,
                &[],
                &empty_offsets,
                128,
            )
            .expect("bank block"),
        );

        for track in 0..lanes {
            assert_eq!(
                bank_report.reports[track], scalar_reports[track],
                "block {index} track {track} report"
            );
            for frame in 0..frames {
                let cell = frame * lanes + track;
                assert_eq!(
                    bank_left[cell].to_bits(),
                    scalar_left[track][frame].to_bits(),
                    "block {index} track {track} frame {frame} left"
                );
                assert_eq!(
                    bank_right[cell].to_bits(),
                    scalar_right[track][frame].to_bits(),
                    "block {index} track {track} frame {frame} right"
                );
            }
            assert_eq!(
                snapshot_bank(bank.as_ref(), track as u32),
                snapshot(scalar[track].as_ref()),
                "block {index} track {track} state"
            );
        }
        position += frames as u64;
    }

    let saved = snapshot_bank(bank.as_ref(), 0);
    let sizes = bank.metadata().program_key.state_sizes;
    bank.restore_track_state_payload(
        0,
        1,
        StatePayloadInput::new(&saved.0, &saved.1, &saved.2, sizes).expect("state input"),
    )
    .expect("state restore");
    assert_eq!(snapshot_bank(bank.as_ref(), 0), saved);
}

/// E8 for the bank: a bank block may be cut anywhere without moving a bit.
#[test]
fn bank_rendering_is_partition_invariant() {
    let Some((width, backend)) = native_bank() else {
        return;
    };
    let lanes = width.lanes() as usize;
    let factory = ParametricEqFactory;
    let values_by_track: Vec<_> = (0..lanes).map(configured_values).collect();
    let requests: Vec<_> = values_by_track
        .iter()
        .map(|values| request(values, false))
        .collect();
    let bind = || {
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &requests,
                active_mask: width.full_mask(),
            })
            .expect("valid bank request")
            .expect("the native width must bind")
    };
    let mut whole = bind();
    let mut split = bind();

    let frames = 128_usize;
    let source: Vec<f32> = (0..frames * lanes)
        .map(|index| ((index as f32) * 0.013).sin() * 0.5)
        .collect();
    let mut whole_left = source.clone();
    let mut whole_right: Vec<f32> = source.iter().map(|value| -value).collect();
    let mut split_left = whole_left.clone();
    let mut split_right = whole_right.clone();

    let empty_offsets = vec![0_u32; lanes + 1];
    for (track, values) in values_by_track.iter().enumerate().take(lanes) {
        let mut target_values = values.clone();
        set_initial(
            &mut target_values,
            3,
            ParameterChannel::Left,
            -4.0 + track as f32 * 0.5,
        );
        set_initial(
            &mut target_values,
            4,
            ParameterChannel::Right,
            0.8 + track as f32 * 0.01,
        );
        let mut changed = vec![false; target_values.len()];
        changed[3 * 2] = true;
        changed[4 * 2 + 1] = true;
        apply_prepared_targets_lane(&mut *whole, track, 48_000, &target_values, &changed);
        apply_prepared_targets_lane(&mut *split, track, 48_000, &target_values, &changed);
    }

    whole.process_bank(
        EffectBankProcessBlock::new(
            &mut whole_left,
            &mut whole_right,
            None,
            frames as u32,
            width,
            0,
            &[],
            &empty_offsets,
            128,
        )
        .expect("whole block"),
    );

    let mut first = 0_usize;
    for chunk in [1_usize, 7, 64, 56] {
        split.process_bank(
            EffectBankProcessBlock::new(
                &mut split_left[first * lanes..(first + chunk) * lanes],
                &mut split_right[first * lanes..(first + chunk) * lanes],
                None,
                chunk as u32,
                width,
                first as u64,
                &[],
                &empty_offsets,
                128,
            )
            .expect("split block"),
        );
        first += chunk;
    }
    assert_eq!(first, frames);

    assert_eq!(
        whole_left.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
        split_left.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
    );
    assert_eq!(
        whole_right.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
        split_right.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
    );
    for track in 0..lanes {
        assert_eq!(
            snapshot_bank(whole.as_ref(), track as u32),
            snapshot_bank(split.as_ref(), track as u32),
            "track {track}"
        );
    }
}

/// Issue #95's frozen split: a malformed shape is a typed error, a width this build cannot
/// execute is a legal non-bank, and neither is ever a silent fallback to a different width.
///
/// Before #95 this crate answered `Ok(None)` to all three, which was the half of the wave-2
/// divergence that hid planner bugs; every other effect answered `effect.bank.requests` to the
/// first two. Red mutation: replace `request.validate_shape()?` in `bind_homogeneous_bank` with
/// the old combined `return Ok(None)` and the two `Err` cases below fail.
#[test]
fn bank_binding_rejects_malformed_shapes_and_declines_a_foreign_width() {
    let factory = ParametricEqFactory;
    let values = values();
    let request = request(&values, false);

    // A backend and a width that disagree about the lane count, and a member count that does not
    // match the declared width: both contradict the request's own fields.
    for (width, backend, count) in [
        (BankWidth::Four, Backend::Simd8, 4),
        (BankWidth::Eight, Backend::Simd8, 4),
    ] {
        let requests = vec![request; count];
        assert_eq!(
            factory
                .bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend,
                    width,
                    requests: &requests,
                    active_mask: width.full_mask(),
                })
                .err()
                .map(|error| error.code),
            Some("effect.bank.requests"),
            "{width:?} {backend:?} is a malformed shape, not a capability gap"
        );
    }

    // A width this artifact was not built for is a capability gap: well formed, not bankable
    // here, and the tracks render as scalar instances.
    let (foreign_width, foreign_backend) = foreign_bank();
    let requests = vec![request; foreign_width.lanes() as usize];
    assert!(
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: foreign_backend,
                width: foreign_width,
                requests: &requests,
                active_mask: foreign_width.full_mask(),
            })
            .expect("a declined bank is not an error")
            .is_none(),
        "{foreign_width:?} {foreign_backend:?} must decline"
    );
}

/// A bank whose tracks do not share a program key is not a bank.
#[test]
fn bank_binding_declines_a_heterogeneous_cohort() {
    let Some((width, backend)) = native_bank() else {
        return;
    };
    let lanes = width.lanes() as usize;
    let values = values();
    let mut requests: Vec<_> = (0..lanes).map(|_| request(&values, false)).collect();
    requests[1].bypass = true;
    assert!(
        ParametricEqFactory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &requests,
                active_mask: width.full_mask(),
            })
            .expect("a declined bank is not an error")
            .is_none()
    );
}

/// Issue #1088 (console strip P2a), gate 3: the EQ has not opted into padding (P2b, #1089), so
/// it declines a padded bank request, and only after it has validated every member.
///
/// Red if the guard in `bind_homogeneous_bank` is removed -- a padded request binds, and the bank
/// runs its clone lanes as real tracks -- or if it moves above member validation, where a padded
/// request with a malformed member is declined instead of refused.
#[test]
fn a_padded_request_is_declined_until_the_eq_opts_in() {
    let Some((width, backend)) = native_bank() else {
        return;
    };
    let lanes = width.lanes() as usize;
    let values = values();
    let requests = vec![request(&values, false); lanes];
    let bind = |requests: &[effect_contract::PrepareEffectRequest<'_>], mask: &[bool]| {
        ParametricEqFactory.bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests,
            active_mask: mask,
        })
    };
    assert!(
        bind(&requests, width.full_mask())
            .expect("a full bank")
            .is_some(),
        "the control: the same members bind as a full bank"
    );
    for members in 1..lanes {
        let mask: Vec<bool> = (0..lanes).map(|lane| lane < members).collect();
        assert!(
            bind(&requests, &mask)
                .expect("a padded request is well formed")
                .is_none(),
            "{members} of {lanes} lanes active"
        );
    }
    let mut malformed = requests.clone();
    malformed[0].limits.maximum_total_state_bytes = 0;
    let refusal = ParametricEqFactory
        .prepare(malformed[0])
        .err()
        .expect("a malformed member")
        .code;
    let mask: Vec<bool> = (0..lanes).map(|lane| lane == 0).collect();
    assert_eq!(
        bind(&malformed, &mask).err().map(|error| error.code),
        Some(refusal),
        "a padded request still validates its members"
    );
}

/// A perturbation on one track's left lane stays there: no leak across lanes or channels.
#[test]
fn bank_lane_and_track_changes_do_not_leak() {
    let Some((width, backend)) = native_bank() else {
        return;
    };
    let lanes = width.lanes() as usize;
    let factory = ParametricEqFactory;
    let values_by_track: Vec<_> = (0..lanes).map(configured_values).collect();
    let requests: Vec<_> = values_by_track
        .iter()
        .map(|values| request(values, false))
        .collect();
    let bind = || {
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &requests,
                active_mask: width.full_mask(),
            })
            .expect("request")
            .expect("native width binds")
    };
    let mut baseline = bind();
    let mut changed = bind();
    let frames = 8;
    let mut baseline_left = vec![0.1_f32; frames * lanes];
    let mut baseline_right = vec![-0.1_f32; frames * lanes];
    let mut changed_left = baseline_left.clone();
    let mut changed_right = baseline_right.clone();
    changed_left[3] = 0.75;
    let offsets = vec![0_u32; lanes + 1];
    for (bank, left, right) in [
        (&mut baseline, &mut baseline_left, &mut baseline_right),
        (&mut changed, &mut changed_left, &mut changed_right),
    ] {
        bank.process_bank(
            EffectBankProcessBlock::new(
                left,
                right,
                None,
                frames as u32,
                width,
                0,
                &[],
                &offsets,
                128,
            )
            .expect("block"),
        );
    }
    for frame in 0..frames {
        for track in 0..lanes {
            let cell = frame * lanes + track;
            assert_eq!(
                baseline_right[cell].to_bits(),
                changed_right[cell].to_bits(),
                "left-only perturbation reached a right lane"
            );
            if track != 3 {
                assert_eq!(baseline_left[cell].to_bits(), changed_left[cell].to_bits());
            }
        }
    }
    for track in 0..lanes {
        if track != 3 {
            assert_eq!(
                snapshot_bank(baseline.as_ref(), track as u32),
                snapshot_bank(changed.as_ref(), track as u32)
            );
        }
    }
}

/// Tracks in the odd-live-count scenario: one eight-lane bank, or two four-lane banks.
const ODD_TRACKS: usize = 8;
/// Blocks in the odd-live-count scenario.
const ODD_BLOCKS: usize = 32;

/// The configurations of [`odd_live_counts_render_the_base_bits`], named by what the stationary
/// cascade keeps of the six physical sections (HPF, bands 1..4, LPF) on an admitted block.
#[derive(Clone, Copy, Debug)]
enum OddShape {
    /// One live section: the HPF on every lane of both channels, through prepared targets.
    HpfEverywhere,
    /// One live section: the HPF on some lanes of one channel only, so the section runs with dry
    /// lanes; the other channel's odd tracks join twelve blocks in.
    HpfSomeLanes,
    /// One live section: a general bell, prepared through `set_initial` (the console fixture).
    BellOnly,
    /// Three live sections: the HPF and a bell at prepare time, then an LPF on some lanes of each
    /// channel through prepared targets, so the last section runs with dry lanes.
    ThreeWithDryLpf,
    /// Three live general bands.
    ThreeGeneral,
    /// Five live sections, the dead one first (the HPF).
    FiveDeadFirst,
    /// Five live sections, the dead one in the middle (band 3).
    FiveDeadMiddle,
    /// Five live sections, the dead one last (the LPF).
    FiveDeadLast,
}

const ODD_SHAPES: [OddShape; 8] = [
    OddShape::HpfEverywhere,
    OddShape::HpfSomeLanes,
    OddShape::BellOnly,
    OddShape::ThreeWithDryLpf,
    OddShape::ThreeGeneral,
    OddShape::FiveDeadFirst,
    OddShape::FiveDeadMiddle,
    OddShape::FiveDeadLast,
];

/// Parameter index of the HPF's enable word; frequency and Q follow it, and the LPF's three follow
/// those (`physical_targets`' order).
const HPF_PARAMETERS: usize = 24;
const LPF_PARAMETERS: usize = 27;

/// Enables general band `band` on `channel` with a per-track design.
fn odd_band(
    values: &mut [effect_contract::InitialParameterValue],
    band: usize,
    kind: EqBandKind,
    channel: ParameterChannel,
    track: usize,
) {
    let right = matches!(channel, ParameterChannel::Right);
    let base = band * 6;
    let frequency = [150.0, 900.0, 3_100.0, 9_000.0][band] * (1.0 + track as f32 * 0.07);
    set_initial(values, base, channel, 1.0);
    set_initial(values, base + 1, channel, kind as u32 as f32);
    set_initial(
        values,
        base + 2,
        channel,
        if right { frequency * 1.13 } else { frequency },
    );
    set_initial(
        values,
        base + 3,
        channel,
        -12.0 + (track * 3 + band * 5) as f32 % 25.0,
    );
    set_initial(
        values,
        base + 4,
        channel,
        0.4 + (track + band) as f32 * 0.21,
    );
    set_initial(values, base + 5, channel, 0.5 + band as f32 * 0.15);
}

/// Enables the dedicated cut whose enable word is `first` on `channel` with a per-track design,
/// and returns the three parameter indices it wrote.
fn odd_cut(
    values: &mut [effect_contract::InitialParameterValue],
    first: usize,
    channel: ParameterChannel,
    track: usize,
) -> [usize; 3] {
    let right = matches!(channel, ParameterChannel::Right);
    let frequency = if first == HPF_PARAMETERS {
        40.0 + track as f32 * 17.0
    } else {
        6_500.0 + track as f32 * 900.0
    };
    set_initial(values, first, channel, 1.0);
    set_initial(
        values,
        first + 1,
        channel,
        if right { frequency * 1.21 } else { frequency },
    );
    set_initial(values, first + 2, channel, 0.55 + track as f32 * 0.09);
    [first, first + 1, first + 2]
}

/// One track's prepare-time values, and the prepared cut targets it receives before `block`:
/// `(block, final values, changed mask)`.
type OddEvent = (
    usize,
    Vec<effect_contract::InitialParameterValue>,
    Vec<bool>,
);

fn odd_configuration(
    shape: OddShape,
    track: usize,
) -> (Vec<effect_contract::InitialParameterValue>, Vec<OddEvent>) {
    use ParameterChannel::{Left, Right};
    let mut initial = values();
    let mut events = Vec::new();
    // A prepared cut on the given channels, applied before `block`.
    let mut cut_event = |initial: &[effect_contract::InitialParameterValue],
                         block: usize,
                         first: usize,
                         channels: &[ParameterChannel]| {
        if channels.is_empty() {
            return;
        }
        let mut target = initial.to_vec();
        let mut changed = vec![false; target.len()];
        for &channel in channels {
            let lane = usize::from(matches!(channel, Right));
            for parameter in odd_cut(&mut target, first, channel, track) {
                changed[parameter * 2 + lane] = true;
            }
        }
        events.push((block, target, changed));
    };
    match shape {
        OddShape::HpfEverywhere => {
            cut_event(&initial, 0, HPF_PARAMETERS, &[Left, Right]);
        }
        OddShape::HpfSomeLanes => {
            if track.is_multiple_of(2) {
                cut_event(&initial, 0, HPF_PARAMETERS, &[Left]);
            } else {
                cut_event(&initial, 12, HPF_PARAMETERS, &[Right]);
            }
        }
        OddShape::BellOnly => {
            for channel in [Left, Right] {
                odd_band(&mut initial, 0, EqBandKind::Bell, channel, track);
            }
        }
        OddShape::ThreeWithDryLpf => {
            for channel in [Left, Right] {
                odd_cut(&mut initial, HPF_PARAMETERS, channel, track);
                odd_band(&mut initial, 1, EqBandKind::Bell, channel, track);
            }
            let mut channels = Vec::new();
            if !track.is_multiple_of(3) {
                channels.push(Left);
            }
            if track.is_multiple_of(2) {
                channels.push(Right);
            }
            cut_event(&initial, 0, LPF_PARAMETERS, &channels);
        }
        OddShape::ThreeGeneral => {
            for channel in [Left, Right] {
                odd_band(&mut initial, 0, EqBandKind::LowShelf, channel, track);
                odd_band(&mut initial, 1, EqBandKind::Bell, channel, track);
                odd_band(&mut initial, 3, EqBandKind::HighShelf, channel, track);
            }
        }
        OddShape::FiveDeadFirst | OddShape::FiveDeadMiddle | OddShape::FiveDeadLast => {
            let kinds = [
                EqBandKind::LowShelf,
                EqBandKind::Bell,
                EqBandKind::Notch,
                EqBandKind::HighShelf,
            ];
            for channel in [Left, Right] {
                if !matches!(shape, OddShape::FiveDeadFirst) {
                    odd_cut(&mut initial, HPF_PARAMETERS, channel, track);
                }
                for (band, kind) in kinds.into_iter().enumerate() {
                    if !(matches!(shape, OddShape::FiveDeadMiddle) && band == 2) {
                        odd_band(&mut initial, band, kind, channel, track);
                    }
                }
                if !matches!(shape, OddShape::FiveDeadLast) {
                    odd_cut(&mut initial, LPF_PARAMETERS, channel, track);
                }
            }
        }
    }
    (initial, events)
}

/// Frames in block `block`: mostly one quantum, and a short ragged block every fifth.
fn odd_frames(block: usize) -> usize {
    if block % 5 == 4 { 37 } else { 128 }
}

/// One hostile input word for `(block, frame, track, channel)`.
///
/// Every word is one of: `+0.0` (one in sixteen), a subnormal of either sign (one in sixteen), or a
/// normal of either sign with magnitude in `2^-24..2^26`. On top of that, block `8k + 3` carries one
/// `-0.0` (tracks 1, 4, 7, 2, alternating planes, so the collapsed mono leg sees half of them) and
/// block `8k + 6` carries one non-finite word on tracks 0 and 4 of one plane, so every
/// four-lane group and every eight-lane group sees the same fault: a fault zeroes and resets a
/// whole bank plane, and placing it this way keeps the bank digest independent of the bank width.
/// Either word refuses elision for the bank (or scalar track) that carries it, which then renders
/// all six sections.
fn odd_word(block: usize, frame: usize, track: usize, channel: usize) -> f32 {
    if block % 8 == 3
        && frame == (block * 7) % odd_frames(block)
        && track == (block / 8 * 3 + 1) % ODD_TRACKS
        && channel == (block / 8) % 2
    {
        return -0.0;
    }
    if block % 8 == 6
        && frame == (block * 13) % odd_frames(block)
        && track.is_multiple_of(4)
        && channel == (block / 8) % 2
    {
        return [f32::INFINITY, f32::NEG_INFINITY, f32::NAN, f32::INFINITY][block / 8];
    }
    let mut state = ((block as u64) << 40)
        ^ ((frame as u64) << 20)
        ^ ((track as u64) << 4)
        ^ channel as u64
        ^ 0x0976_0976_0976_0976;
    let word = support::splitmix64(&mut state);
    let sign = ((word >> 63) as u32) << 31;
    match word & 15 {
        0 => 0.0,
        1 => f32::from_bits(sign | (((word >> 8) as u32 & 0x007f_ffff) | 1)),
        _ => {
            let exponent = ((word >> 8) % 50) as u32 + 127 - 24;
            let mantissa = (word >> 16) as u32 & 0x007f_ffff;
            f32::from_bits(sign | (exponent << 23) | mantissa)
        }
    }
}

fn fold_report(hasher: &mut Sha256, report: &effect_contract::ProcessReport) {
    for count in [
        report.sanitized_main_samples,
        report.sanitized_sidechain_samples,
        report.invalid_spans,
        report.nonfinite_left_blocks,
        report.nonfinite_right_blocks,
    ] {
        hasher.update(count.to_le_bytes());
    }
}

/// Folds rendered output words. Each must be finite, the §4.4 check's promise (a non-finite block
/// is zeroed), so the class-A fold can never hide a NaN in an output word (#1065).
fn fold_words(hasher: &mut Sha256, words: impl Iterator<Item = f32>) {
    for word in words {
        assert!(
            word.is_finite(),
            "an output word is not finite: {:#010x}",
            word.to_bits()
        );
        hasher.update(class_a::bits(word).to_le_bytes());
    }
}

fn fold_payload(hasher: &mut Sha256, payload: &Payload) {
    for section in [&payload.0[..], &payload.1[..], &payload.2[..]] {
        for word in class_a::le_words(section) {
            hasher.update(word);
        }
    }
}

/// Resets this thread's count of select-free depth-one tail passes (issue #976 M3); a no-op
/// without `test-support`, where the counter does not exist.
fn reset_select_free_tails() {
    #[cfg(feature = "test-support")]
    parametric_eq::test_only_reset_select_free_tail_passes();
}

/// This thread's count of select-free depth-one tail passes, or `None` without `test-support`.
fn select_free_tails() -> Option<usize> {
    #[cfg(feature = "test-support")]
    let count = Some(parametric_eq::test_only_select_free_tail_passes());
    #[cfg(not(feature = "test-support"))]
    let count = None;
    count
}

/// One leg's digest, and per shape (in [`ODD_SHAPES`] order) how many stationary depth-one tail
/// passes ran without the dry select.
type OddLeg = (String, Vec<Option<usize>>);

/// The scalar leg: one prepared effect per track, every block, every word.
fn odd_scalar_digest() -> OddLeg {
    let factory = ParametricEqFactory;
    let mut hasher = Sha256::new();
    let mut tails = Vec::new();
    for shape in ODD_SHAPES {
        reset_select_free_tails();
        let configurations: Vec<_> = (0..ODD_TRACKS)
            .map(|track| odd_configuration(shape, track))
            .collect();
        let mut effects: Vec<_> = configurations
            .iter()
            .map(|(initial, _)| {
                factory
                    .prepare(request(initial, false))
                    .expect("scalar prepare")
            })
            .collect();
        let mut position = 0_u64;
        for block in 0..ODD_BLOCKS {
            let frames = odd_frames(block);
            for (track, effect) in effects.iter_mut().enumerate() {
                for (at, target, changed) in &configurations[track].1 {
                    if *at == block {
                        apply_prepared_targets(effect.as_mut(), target, changed);
                    }
                }
                let mut left: Vec<f32> = (0..frames)
                    .map(|frame| odd_word(block, frame, track, 0))
                    .collect();
                let mut right: Vec<f32> = (0..frames)
                    .map(|frame| odd_word(block, frame, track, 1))
                    .collect();
                let report = effect.process(
                    EffectProcessBlock::new(&mut left, &mut right, None, position, &[], 128)
                        .expect("scalar block"),
                );
                fold_words(&mut hasher, left.into_iter().chain(right));
                fold_report(&mut hasher, &report);
                fold_payload(&mut hasher, &snapshot(effect.as_ref()));
            }
            position += frames as u64;
        }
        tails.push(select_free_tails());
    }
    (hex(&hasher.finalize()), tails)
}

/// The bank legs: `ODD_TRACKS / lanes` native banks, folded per track in track order so the
/// digest does not depend on the bank width. `mono` renders the collapsed body instead of the dual
/// one, over the left plane alone.
fn odd_bank_digest(width: BankWidth, backend: Backend, mono: bool) -> OddLeg {
    let lanes = width.lanes() as usize;
    assert_eq!(ODD_TRACKS % lanes, 0, "the scenario fills whole banks");
    let factory = ParametricEqFactory;
    let mut hasher = Sha256::new();
    let mut tails = Vec::new();
    for shape in ODD_SHAPES {
        reset_select_free_tails();
        let configurations: Vec<_> = (0..ODD_TRACKS)
            .map(|track| odd_configuration(shape, track))
            .collect();
        let mut banks: Vec<_> = configurations
            .chunks(lanes)
            .map(|group| {
                let requests: Vec<_> = group
                    .iter()
                    .map(|(initial, _)| request(initial, false))
                    .collect();
                factory
                    .bind_homogeneous_bank(PrepareEffectBankRequest {
                        backend,
                        width,
                        requests: &requests,
                        active_mask: width.full_mask(),
                    })
                    .expect("valid bank request")
                    .expect("the native width must bind")
            })
            .collect();
        let offsets = vec![0_u32; lanes + 1];
        let mut position = 0_u64;
        for block in 0..ODD_BLOCKS {
            let frames = odd_frames(block);
            for (group, bank) in banks.iter_mut().enumerate() {
                assert!(!mono || bank.supports_mono_collapse());
                for lane in 0..lanes {
                    for (at, target, changed) in &configurations[group * lanes + lane].1 {
                        if *at == block {
                            apply_prepared_targets_lane(
                                bank.as_mut(),
                                lane,
                                48_000,
                                target,
                                changed,
                            );
                        }
                    }
                }
                let plane = |channel: usize| -> Vec<f32> {
                    (0..frames * lanes)
                        .map(|cell| {
                            odd_word(block, cell / lanes, group * lanes + cell % lanes, channel)
                        })
                        .collect()
                };
                let mut left = plane(0);
                let mut right = if mono {
                    vec![f32::from_bits(0x7F7F_FFFF); frames * lanes]
                } else {
                    plane(1)
                };
                let process = EffectBankProcessBlock::new(
                    &mut left,
                    &mut right,
                    None,
                    frames as u32,
                    width,
                    position,
                    &[],
                    &offsets,
                    128,
                )
                .expect("bank block");
                let report = if mono {
                    bank.process_bank_mono(process)
                } else {
                    bank.process_bank(process)
                };
                for lane in 0..lanes {
                    let column = |plane: &[f32]| -> Vec<f32> {
                        (0..frames)
                            .map(|frame| plane[frame * lanes + lane])
                            .collect()
                    };
                    if mono {
                        fold_words(&mut hasher, column(&left).into_iter());
                    } else {
                        fold_words(&mut hasher, column(&left).into_iter().chain(column(&right)));
                    }
                    fold_report(&mut hasher, &report.reports[lane]);
                    fold_payload(&mut hasher, &snapshot_bank(bank.as_ref(), lane as u32));
                }
            }
            position += frames as u64;
        }
        tails.push(select_free_tails());
    }
    (hex(&hasher.finalize()), tails)
}

fn hex(digest: &[u8]) -> String {
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// The digests [`odd_live_counts_render_the_base_bits`] pins, recorded on the unmodified base of
/// issue #976 (before the stationary cascade stopped padding odd live-section counts).
const ODD_LIVE_DIGESTS: [(&str, &str); 3] = [
    (
        "scalar",
        "81015a5c841e7fb53b6d7f4968c1706b46902db7055cdc687fa2d7c5366cb852",
    ),
    (
        "bank",
        "247bc0b6fd53e45fe85f15dd481dc65eb6110bc3ca8cd6a056e40a6d9c131d7c",
    ),
    (
        "bank-mono",
        "e904180499a49c1a203eef5da5f6a257b7e9a74c34a1e1ba22fdb7ebcb1e108a",
    ),
];

/// Issue #976 gate 1: odd live-section counts (1, 3 and 5) render the bits they rendered when the
/// stationary cascade still rounded the live count up to whole depth-2 passes with an identity
/// padding section.
///
/// The scenario covers, through the public API only: the HPF live on every lane (prepared targets),
/// the HPF live on some lanes only (the last kept section runs with dry lanes), general bands at
/// prepare time, an LPF on some lanes as the last of three, and five live sections with the dead
/// one first, in the middle and last. The input is hostile (subnormals, `+0.0`, magnitudes
/// `2^-24..2^26`, ragged blocks), and some blocks carry a `-0.0` or a non-finite word, which refuse
/// elision where they land, so that bank or track renders all six sections. Every output word, every report and every lane's state
/// payload after every block is folded into one SHA-256 per leg, by class-A words (no NaN reaches
/// an output word: a non-finite block is zeroed).
///
/// Under `--features test-support` it also asserts M3's performance half: the HPF-everywhere shape
/// runs its one live section, the odd tail, without the dry select on every admitted stationary
/// block, which no rendered bit can show.
#[test]
fn odd_live_counts_render_the_base_bits() {
    let mut legs = vec![("scalar", odd_scalar_digest())];
    if let Some((width, backend)) = native_bank() {
        legs.push(("bank", odd_bank_digest(width, backend, false)));
        legs.push(("bank-mono", odd_bank_digest(width, backend, true)));
    }
    for (leg, (digest, tails)) in &legs {
        println!("odd-live digest {leg} {digest}");
        println!("odd-live select-free tails {leg} {tails:?} (shapes {ODD_SHAPES:?})");
    }
    let hpf_everywhere = ODD_SHAPES
        .iter()
        .position(|shape| matches!(shape, OddShape::HpfEverywhere))
        .expect("the HPF-everywhere shape is in the scenario");
    for (leg, (_, tails)) in &legs {
        if let Some(count) = tails[hpf_everywhere] {
            assert!(
                count > 0,
                "#976 M3: the {leg} leg's HPF-everywhere tail must run select-free"
            );
        }
    }
    for (leg, (digest, _)) in &legs {
        let pinned = ODD_LIVE_DIGESTS
            .iter()
            .find(|(name, _)| name == leg)
            .map(|(_, pin)| *pin)
            .expect("every leg is pinned");
        assert_eq!(
            digest, pinned,
            "#976 gate 1: the {leg} leg moved a bit, a report or a state word"
        );
    }
}

/// Tracks in the admitted-select scenario: one eight-lane bank, or two four-lane banks.
const SELECT_TRACKS: usize = 8;
/// Blocks in the admitted-select scenario.
const SELECT_BLOCKS: usize = 32;
/// The block before which the dedicated cuts switch on, and the one before which they switch off.
const SELECT_ENABLE: usize = 0;
const SELECT_DISABLE: usize = 5;
/// The blocks whose input carries the `1.1e31` spike that poisons a restored dry lane.
const SELECT_SPIKES: [usize; 2] = [2, 18];

/// The configurations of [`admitted_blocks_render_the_base_bits_without_selects`].
#[derive(Clone, Copy, Debug, PartialEq)]
enum SelectShape {
    /// Two live general bands (a bell and a high shelf) and the HPF, switched on everywhere at
    /// block 0 and off at block 5 on the left of even tracks and the right of odd tracks: the pass
    /// that carries the HPF has dry lanes holding a frozen, non-zero state.
    DryHpfInPair,
    /// The same with the LPF on the opposite lanes: the depth-one tail carries the dry lanes.
    DryLpfInTail,
    /// Both cuts on everywhere at block 0 and off on both channels of even tracks at block 5: a
    /// pass of two dedicated cuts, dry together on those lanes.
    DryCutsTogether,
    /// VERIFY-EQ finding 2: the HPF live on the right, a 10 Hz LPF live on the left, and a
    /// restored `ic2 = -f32::MAX` in track 0's (dry) left HPF before blocks 0 and 16. A refused
    /// block carrying `1.1e31` there overflows the dry lane's `v3`, which leaves its state `NaN`
    /// while the dry select passes the spike on and the LPF keeps the output under the §4.4 bound.
    /// Every later block is refused on that `NaN` (leg (c)'s finiteness term) until the fault
    /// block that resets the plane.
    PoisonedDryHpf,
}

const SELECT_SHAPES: [SelectShape; 4] = [
    SelectShape::DryHpfInPair,
    SelectShape::DryLpfInTail,
    SelectShape::DryCutsTogether,
    SelectShape::PoisonedDryHpf,
];

/// Marks every parameter of a dedicated cut on `channel` as changed.
fn mark_cut(changed: &mut [bool], first: usize, channel: ParameterChannel) {
    let lane = usize::from(matches!(channel, ParameterChannel::Right));
    for parameter in first..first + 3 {
        changed[parameter * 2 + lane] = true;
    }
}

fn select_configuration(
    shape: SelectShape,
    track: usize,
) -> (Vec<effect_contract::InitialParameterValue>, Vec<OddEvent>) {
    use ParameterChannel::{Left, Right};
    let mut initial = values();
    let mut events = Vec::new();
    // `cuts` switch on at `SELECT_ENABLE` on both channels, and off at `SELECT_DISABLE` on
    // `off(track)`'s channels.
    let mut switch = |initial: &[effect_contract::InitialParameterValue],
                      cuts: &[usize],
                      off: &[ParameterChannel]| {
        let mut on = initial.to_vec();
        let mut changed = vec![false; on.len()];
        for &first in cuts {
            for channel in [Left, Right] {
                odd_cut(&mut on, first, channel, track);
                mark_cut(&mut changed, first, channel);
            }
        }
        events.push((SELECT_ENABLE, on.clone(), changed));
        let mut changed = vec![false; on.len()];
        for &first in cuts {
            for &channel in off {
                set_initial(&mut on, first, channel, 0.0);
                mark_cut(&mut changed, first, channel);
            }
        }
        events.push((SELECT_DISABLE, on, changed));
    };
    let alternate = if track.is_multiple_of(2) {
        [Left]
    } else {
        [Right]
    };
    let opposite = if track.is_multiple_of(2) {
        [Right]
    } else {
        [Left]
    };
    match shape {
        SelectShape::DryHpfInPair | SelectShape::DryLpfInTail => {
            for channel in [Left, Right] {
                odd_band(&mut initial, 0, EqBandKind::Bell, channel, track);
                odd_band(&mut initial, 2, EqBandKind::HighShelf, channel, track);
            }
            if shape == SelectShape::DryHpfInPair {
                switch(&initial, &[HPF_PARAMETERS], &alternate);
            } else {
                switch(&initial, &[LPF_PARAMETERS], &opposite);
            }
        }
        SelectShape::DryCutsTogether => {
            let off: &[ParameterChannel] = if track.is_multiple_of(2) {
                &[Left, Right]
            } else {
                &[]
            };
            switch(&initial, &[HPF_PARAMETERS, LPF_PARAMETERS], off);
        }
        SelectShape::PoisonedDryHpf => {
            odd_cut(&mut initial, HPF_PARAMETERS, Right, track);
            set_initial(&mut initial, LPF_PARAMETERS, Left, 1.0);
            set_initial(&mut initial, LPF_PARAMETERS + 1, Left, 10.0);
        }
    }
    (initial, events)
}

/// Frames in block `block`: mostly one quantum, and a short ragged block every fifth.
fn select_frames(block: usize) -> usize {
    odd_frames(block)
}

/// One hostile input word for `(block, frame, track, channel)` of shape `shape`.
///
/// The body has [`odd_word`]'s distribution under its own seed: `+0.0` (one in sixteen), a
/// subnormal of either sign (one in sixteen), or a normal with magnitude in `2^-24..2^26`. Block `8k + 3` carries one `-0.0` on
/// track `2k` (a lane that is dry in [`SelectShape::DryCutsTogether`]), plane `k mod 2`; block
/// `8k + 6` carries one non-finite word on tracks 0 and 4 of plane `k mod 2`, which faults, zeroes
/// and resets that plane of every bank the same way at either width. The poisoned shape adds the
/// `1.1e31` spike on track 0's left plane at [`SELECT_SPIKES`]. Each of those words refuses the
/// elision for the bank (or the scalar track) that carries it.
fn select_word(
    shape: SelectShape,
    block: usize,
    frame: usize,
    track: usize,
    channel: usize,
) -> f32 {
    let frames = select_frames(block);
    if shape == SelectShape::PoisonedDryHpf
        && SELECT_SPIKES.contains(&block)
        && frame == 10
        && track == 0
        && channel == 0
    {
        return 1.1e31;
    }
    let k = block / 8;
    if block % 8 == 3 && frame == (block * 7) % frames && track == 2 * k && channel == k % 2 {
        return -0.0;
    }
    if block % 8 == 6
        && frame == (block * 13) % frames
        && track.is_multiple_of(4)
        && channel == k % 2
    {
        return [f32::INFINITY, f32::NEG_INFINITY, f32::NAN, f32::INFINITY][k];
    }
    let mut state = ((block as u64) << 40)
        ^ ((frame as u64) << 20)
        ^ ((track as u64) << 4)
        ^ channel as u64
        ^ 0x0977_0977_0977_0977;
    let word = support::splitmix64(&mut state);
    let sign = ((word >> 63) as u32) << 31;
    match word & 15 {
        0 => 0.0,
        1 => f32::from_bits(sign | (((word >> 8) as u32 & 0x007f_ffff) | 1)),
        _ => {
            let exponent = ((word >> 8) % 50) as u32 + 127 - 24;
            let mantissa = (word >> 16) as u32 & 0x007f_ffff;
            f32::from_bits(sign | (exponent << 23) | mantissa)
        }
    }
}

/// `true` when the elision gate's input leg (a) admits `plane`: no `-0.0`, finite, inside the
/// §4.4 bound. The test's own statement of it, independent of the crate's.
fn plane_admits(plane: &[f32]) -> bool {
    plane
        .iter()
        .all(|word| word.to_bits() != 0x8000_0000 && word.is_finite() && word.abs() <= 1.0e30)
}

/// The blocks of a switching shape that run the stationary cascade: every block but the two
/// that start a ramp (a ramp is 64 samples, and both switches land on a 128-frame block).
fn select_stationary(block: usize) -> bool {
    block != SELECT_ENABLE && block != SELECT_DISABLE
}

/// Resets this thread's count of masked stationary depth-two passes (issue #977); a no-op
/// without `test-support`.
fn reset_masked_passes() {
    #[cfg(feature = "test-support")]
    parametric_eq::test_only_reset_masked_pair_passes();
}

/// This thread's count of masked stationary depth-two passes, or `None` without `test-support`.
fn masked_passes() -> Option<usize> {
    #[cfg(feature = "test-support")]
    let count = Some(parametric_eq::test_only_masked_pair_passes());
    #[cfg(not(feature = "test-support"))]
    let count = None;
    count
}

/// Masked depth-two passes a leg ran, per shape: `(on admitted blocks, on refused blocks)`, counted
/// only for the switching shapes (whose admission the test can state from the input alone).
type SelectCounts = Vec<Option<(usize, usize)>>;

/// One leg of the admitted-select scenario: its digest, its masked-pass counts, and whether the
/// poisoned shape poisoned as intended.
type SelectLeg = (String, SelectCounts, bool);

/// Sets `ic2` of section `section` of the left channel in a lane payload.
fn plant_left_ic2(payload: &mut Payload, section: usize, value: f32) {
    let word = section * support::WORDS_PER_BAND + 1;
    payload.1[word * 4..word * 4 + 4].copy_from_slice(&value.to_bits().to_le_bytes());
}

/// Non-vacuity of [`SelectShape::PoisonedDryHpf`]: `true` unless `block` is a spike block and
/// track 0's dry left HPF was not left holding `NaN` integrators, or the block did not pass the
/// §4.4 check (a fault would reset the lane). Collected rather than asserted in place, so a leg
/// that breaks it still prints its digest first.
fn poisoned_as_intended(
    block: usize,
    payload: &Payload,
    report: &effect_contract::ProcessReport,
) -> bool {
    !SELECT_SPIKES.contains(&block)
        || ((0..2).all(|integrator| f32::from_bits(support::word(&payload.1, integrator)).is_nan())
            && report.nonfinite_left_blocks == 0)
}

/// The scalar leg of the admitted-select scenario.
fn select_scalar_digest() -> SelectLeg {
    let factory = ParametricEqFactory;
    let mut hasher = Sha256::new();
    let mut counts = Vec::new();
    let mut poisoned = true;
    for shape in SELECT_SHAPES {
        let counted = shape != SelectShape::PoisonedDryHpf && shape != SelectShape::DryCutsTogether;
        let mut tally = (0_usize, 0_usize);
        let configurations: Vec<_> = (0..SELECT_TRACKS)
            .map(|track| select_configuration(shape, track))
            .collect();
        let mut effects: Vec<_> = configurations
            .iter()
            .map(|(initial, _)| {
                factory
                    .prepare(request(initial, false))
                    .expect("scalar prepare")
            })
            .collect();
        let mut position = 0_u64;
        for block in 0..SELECT_BLOCKS {
            let frames = select_frames(block);
            for (track, effect) in effects.iter_mut().enumerate() {
                for (at, target, changed) in &configurations[track].1 {
                    if *at == block {
                        apply_prepared_targets(effect.as_mut(), target, changed);
                    }
                }
                if shape == SelectShape::PoisonedDryHpf && track == 0 && block % 16 == 0 {
                    let mut payload = snapshot(effect.as_ref());
                    plant_left_ic2(&mut payload, 0, -f32::MAX);
                    effect
                        .restore_state_payload(
                            PARAMETRIC_EQ_DESCRIPTOR.state_layout_version,
                            StatePayloadInput::new(
                                &payload.0,
                                &payload.1,
                                &payload.2,
                                effect.metadata().state_sizes,
                            )
                            .expect("state input"),
                        )
                        .expect("a finite integrator restores");
                }
                let mut left: Vec<f32> = (0..frames)
                    .map(|frame| select_word(shape, block, frame, track, 0))
                    .collect();
                let mut right: Vec<f32> = (0..frames)
                    .map(|frame| select_word(shape, block, frame, track, 1))
                    .collect();
                let admitted =
                    select_stationary(block) && plane_admits(&left) && plane_admits(&right);
                let before = masked_passes();
                let report = effect.process(
                    EffectProcessBlock::new(&mut left, &mut right, None, position, &[], 128)
                        .expect("scalar block"),
                );
                if let (Some(before), Some(after)) = (before, masked_passes())
                    && select_stationary(block)
                {
                    if admitted {
                        tally.0 += after - before;
                    } else {
                        tally.1 += after - before;
                    }
                }
                if shape == SelectShape::PoisonedDryHpf && track == 0 {
                    poisoned &= poisoned_as_intended(block, &snapshot(effect.as_ref()), &report);
                }
                fold_words(&mut hasher, left.into_iter().chain(right));
                fold_report(&mut hasher, &report);
                fold_payload(&mut hasher, &snapshot(effect.as_ref()));
            }
            position += frames as u64;
        }
        counts.push((counted && masked_passes().is_some()).then_some(tally));
    }
    reset_masked_passes();
    (hex(&hasher.finalize()), counts, poisoned)
}

/// The bank legs of the admitted-select scenario, folded per track in track order so the digest
/// does not depend on the bank width. `mono` renders the collapsed body over the left plane.
fn select_bank_digest(width: BankWidth, backend: Backend, mono: bool) -> SelectLeg {
    let lanes = width.lanes() as usize;
    assert_eq!(SELECT_TRACKS % lanes, 0, "the scenario fills whole banks");
    let factory = ParametricEqFactory;
    let mut hasher = Sha256::new();
    let mut counts = Vec::new();
    let mut poisoned = true;
    for shape in SELECT_SHAPES {
        let counted = shape != SelectShape::PoisonedDryHpf && shape != SelectShape::DryCutsTogether;
        let mut tally = (0_usize, 0_usize);
        let configurations: Vec<_> = (0..SELECT_TRACKS)
            .map(|track| select_configuration(shape, track))
            .collect();
        let mut banks: Vec<_> = configurations
            .chunks(lanes)
            .map(|group| {
                let requests: Vec<_> = group
                    .iter()
                    .map(|(initial, _)| request(initial, false))
                    .collect();
                factory
                    .bind_homogeneous_bank(PrepareEffectBankRequest {
                        backend,
                        width,
                        requests: &requests,
                        active_mask: width.full_mask(),
                    })
                    .expect("valid bank request")
                    .expect("the native width must bind")
            })
            .collect();
        let offsets = vec![0_u32; lanes + 1];
        let mut position = 0_u64;
        for block in 0..SELECT_BLOCKS {
            let frames = select_frames(block);
            for (group, bank) in banks.iter_mut().enumerate() {
                for lane in 0..lanes {
                    for (at, target, changed) in &configurations[group * lanes + lane].1 {
                        if *at == block {
                            apply_prepared_targets_lane(
                                bank.as_mut(),
                                lane,
                                48_000,
                                target,
                                changed,
                            );
                        }
                    }
                }
                if shape == SelectShape::PoisonedDryHpf && group == 0 && block % 16 == 0 {
                    let mut payload = snapshot_bank(bank.as_ref(), 0);
                    plant_left_ic2(&mut payload, 0, -f32::MAX);
                    let sizes = bank.metadata().program_key.state_sizes;
                    bank.restore_track_state_payload(
                        0,
                        PARAMETRIC_EQ_DESCRIPTOR.state_layout_version,
                        StatePayloadInput::new(&payload.0, &payload.1, &payload.2, sizes)
                            .expect("state input"),
                    )
                    .expect("a finite integrator restores");
                }
                let plane = |channel: usize| -> Vec<f32> {
                    (0..frames * lanes)
                        .map(|cell| {
                            select_word(
                                shape,
                                block,
                                cell / lanes,
                                group * lanes + cell % lanes,
                                channel,
                            )
                        })
                        .collect()
                };
                let mut left = plane(0);
                let mut right = if mono {
                    vec![f32::from_bits(0x7F7F_FFFF); frames * lanes]
                } else {
                    plane(1)
                };
                let admitted = select_stationary(block)
                    && plane_admits(&left)
                    && (mono || plane_admits(&right));
                let before = masked_passes();
                let process = EffectBankProcessBlock::new(
                    &mut left,
                    &mut right,
                    None,
                    frames as u32,
                    width,
                    position,
                    &[],
                    &offsets,
                    128,
                )
                .expect("bank block");
                let report = if mono {
                    bank.process_bank_mono(process)
                } else {
                    bank.process_bank(process)
                };
                if let (Some(before), Some(after)) = (before, masked_passes())
                    && select_stationary(block)
                {
                    if admitted {
                        tally.0 += after - before;
                    } else {
                        tally.1 += after - before;
                    }
                }
                if shape == SelectShape::PoisonedDryHpf && group == 0 {
                    poisoned &= poisoned_as_intended(
                        block,
                        &snapshot_bank(bank.as_ref(), 0),
                        &report.reports[0],
                    );
                }
                for lane in 0..lanes {
                    let column = |plane: &[f32]| -> Vec<f32> {
                        (0..frames)
                            .map(|frame| plane[frame * lanes + lane])
                            .collect()
                    };
                    if mono {
                        fold_words(&mut hasher, column(&left).into_iter());
                    } else {
                        fold_words(&mut hasher, column(&left).into_iter().chain(column(&right)));
                    }
                    fold_report(&mut hasher, &report.reports[lane]);
                    fold_payload(&mut hasher, &snapshot_bank(bank.as_ref(), lane as u32));
                }
            }
            position += frames as u64;
        }
        counts.push((counted && masked_passes().is_some()).then_some(tally));
    }
    reset_masked_passes();
    (hex(&hasher.finalize()), counts, poisoned)
}

/// The digests [`admitted_blocks_render_the_base_bits_without_selects`] pins, recorded on the
/// unmodified base of issue #977 (every stationary pass still masked) as raw bits
/// (`9316456b…`, `d4a1dc9d…`, `f442a0d3…`), and re-pinned by #1065 with every NaN word folded to
/// one value. Only NaN words moved: the same render hashed raw still gives the #977 pins, and
/// its 48 NaN words (the poisoned dry lane's `NaN` integrators, 16 per leg, all `0xFFC0_0000` on
/// x86) are the only words the fold changes. The folded scalar digest is the raw digest the
/// AArch64 leg printed for #1017, whose arithmetic NaN is already `0x7FC0_0000`.
const SELECT_DIGESTS: [(&str, &str); 3] = [
    (
        "scalar",
        "3719d502178e4c1e65fd01d18b9664d4a50259a1fc61d733cd229cd1e7e9f6e3",
    ),
    (
        "bank",
        "d68a2494011d118d10eb295683957bd55c09092259d9161ae0bd0a0cfc6ac5b7",
    ),
    (
        "bank-mono",
        "e5db81b9acb69a451505fbd48add97d960c928f934b9fc20c6fd136a88b725a6",
    ),
];

/// Issue #977 gate 1: on an admitted block every select of the stationary cascade is a no-op,
/// including a dedicated cut's dry lanes, so running the depth-two passes select-free renders the
/// bits the masked kernel rendered (the depth-one tail keeps #976's rule).
///
/// Through the public API only: dry lanes holding a frozen non-zero state (a cut switched on and
/// then off again through prepared targets) in a depth-two pass, in the depth-one tail, and in a
/// pass of two dry cuts; and the poisoned dry lane of VERIFY-EQ finding 2, whose `NaN` state must
/// keep its bank on the masked kernel. Hostile input (`+0.0`, subnormals, magnitudes
/// `2^-24..2^26`, ragged blocks) with `-0.0`, non-finite and oversized words on some blocks, which
/// refuse the elision where they land. Every output word, every report and every lane's state
/// payload after every block is folded into one SHA-256 per leg, by class-A words: state words
/// included, and every `NaN` one folded to one word (#1065), because the CPU picks its sign.
///
/// Gate 2, under `--features test-support`: over the two switching shapes whose admission the
/// input alone decides, the masked depth-two pass counter reads zero on every admitted stationary
/// block and is non-zero on the refused ones.
#[test]
fn admitted_blocks_render_the_base_bits_without_selects() {
    let mut legs = vec![("scalar", select_scalar_digest())];
    if let Some((width, backend)) = native_bank() {
        legs.push(("bank", select_bank_digest(width, backend, false)));
        legs.push(("bank-mono", select_bank_digest(width, backend, true)));
    }
    for (leg, (digest, counts, _)) in &legs {
        println!("admitted-select digest {leg} {digest}");
        println!(
            "admitted-select masked passes {leg} (admitted, refused) {counts:?} (shapes {SELECT_SHAPES:?})"
        );
    }
    for (leg, (digest, _, _)) in &legs {
        let pinned = SELECT_DIGESTS
            .iter()
            .find(|(name, _)| name == leg)
            .map(|(_, pin)| *pin)
            .expect("every leg is pinned");
        assert_eq!(
            digest, pinned,
            "#977 gate 1: the {leg} leg moved a bit, a report or a state word"
        );
    }
    for (leg, (_, _, poisoned)) in &legs {
        assert!(
            poisoned,
            "the {leg} leg's spike blocks must leave the dry lane NaN and pass the §4.4 check"
        );
    }
    for (leg, (_, counts, _)) in &legs {
        for (shape, count) in SELECT_SHAPES.iter().zip(counts) {
            if let Some((admitted, refused)) = count {
                assert_eq!(
                    *admitted, 0,
                    "#977 gate 2: the {leg} leg ran a masked pair on an admitted block ({shape:?})"
                );
                assert!(
                    *refused > 0,
                    "#977 gate 2: the {leg} leg's refused blocks must keep the masks ({shape:?})"
                );
            }
        }
    }
}

/// Tracks in the skewed-pass scenario: one eight-lane bank, or two four-lane banks.
const SKEW_TRACKS: usize = 8;
/// Blocks in the skewed-pass scenario.
const SKEW_BLOCKS: usize = 32;

/// The configurations of [`two_and_four_live_sections_render_the_base_bits`], named by their live
/// physical sections; every one of them runs its stationary cascade in depth-two passes only.
#[derive(Clone, Copy, Debug, PartialEq)]
enum SkewShape {
    /// Two live general bands: one pair.
    TwoGeneral,
    /// The two dedicated cuts, each live on some lanes and dry on the others: one pair of cuts.
    TwoCutsDry,
    /// Four live general bands: two pairs.
    FourGeneral,
    /// The HPF and the LPF on some lanes, and two general bands: two pairs, dry lanes in both.
    FourWithDryCuts,
    /// All six sections live, the cuts on some lanes only: three masked pairs on every block.
    SixWithDryCuts,
    /// All six sections live on every lane: three masked pairs on every block.
    SixEverywhere,
}

const SKEW_SHAPES: [SkewShape; 6] = [
    SkewShape::TwoGeneral,
    SkewShape::TwoCutsDry,
    SkewShape::FourGeneral,
    SkewShape::FourWithDryCuts,
    SkewShape::SixWithDryCuts,
    SkewShape::SixEverywhere,
];

/// One track's prepare-time values for `shape`: every section is set at prepare time.
fn skew_configuration(
    shape: SkewShape,
    track: usize,
) -> Vec<effect_contract::InitialParameterValue> {
    use ParameterChannel::{Left, Right};
    let mut values = values();
    let kinds = [
        EqBandKind::LowShelf,
        EqBandKind::Bell,
        EqBandKind::Notch,
        EqBandKind::HighShelf,
    ];
    let bands: &[usize] = match shape {
        SkewShape::TwoGeneral => &[0, 2],
        SkewShape::TwoCutsDry => &[],
        SkewShape::FourWithDryCuts => &[1, 2],
        SkewShape::FourGeneral | SkewShape::SixWithDryCuts | SkewShape::SixEverywhere => {
            &[0, 1, 2, 3]
        }
    };
    for channel in [Left, Right] {
        for &band in bands {
            odd_band(&mut values, band, kinds[band], channel, track);
        }
    }
    // Which channels carry each cut: everywhere, or a per-track pattern that leaves every
    // four-lane group with both dry and live lanes on both channels.
    let (hpf, lpf): (&[ParameterChannel], &[ParameterChannel]) = match shape {
        SkewShape::TwoGeneral | SkewShape::FourGeneral => (&[], &[]),
        SkewShape::SixEverywhere => (&[Left, Right], &[Left, Right]),
        SkewShape::TwoCutsDry | SkewShape::FourWithDryCuts | SkewShape::SixWithDryCuts => {
            match track % 4 {
                0 => (&[Left], &[Right]),
                1 => (&[Right], &[Left, Right]),
                2 => (&[Left, Right], &[]),
                _ => (&[], &[Left]),
            }
        }
    };
    for &channel in hpf {
        odd_cut(&mut values, HPF_PARAMETERS, channel, track);
    }
    for &channel in lpf {
        odd_cut(&mut values, LPF_PARAMETERS, channel, track);
    }
    values
}

/// Frames in block `block`: blocks `8k + 1` run 1, 2, 3 and 37 frames (the skewed pass's
/// fallback, its shortest pipelines, and a ragged block), every fifth block 37, the rest 128.
fn skew_frames(block: usize) -> usize {
    if block % 8 == 1 {
        [1, 2, 3, 37][block / 8]
    } else if block % 5 == 4 {
        37
    } else {
        128
    }
}

/// One hostile input word for the skewed-pass scenario, [`select_word`]'s families under its own
/// seed: `+0.0`, subnormals of either sign and normals in `2^-24..2^26`, with one `-0.0` on track
/// `2k`, plane `k mod 2`, at block `8k + 3`, and one non-finite word on tracks 0 and 4 of plane
/// `k mod 2` at block `8k + 6`.
fn skew_word(block: usize, frame: usize, track: usize, channel: usize) -> f32 {
    let frames = skew_frames(block);
    let k = block / 8;
    if block % 8 == 3 && frame == (block * 7) % frames && track == 2 * k && channel == k % 2 {
        return -0.0;
    }
    if block % 8 == 6
        && frame == (block * 13) % frames
        && track.is_multiple_of(4)
        && channel == k % 2
    {
        return [f32::INFINITY, f32::NAN, f32::NEG_INFINITY, f32::INFINITY][k];
    }
    let mut state = ((block as u64) << 40)
        ^ ((frame as u64) << 20)
        ^ ((track as u64) << 4)
        ^ channel as u64
        ^ 0x0978_0978_0978_0978;
    let word = support::splitmix64(&mut state);
    let sign = ((word >> 63) as u32) << 31;
    match word & 15 {
        0 => 0.0,
        1 => f32::from_bits(sign | (((word >> 8) as u32 & 0x007f_ffff) | 1)),
        _ => {
            let exponent = ((word >> 8) % 50) as u32 + 127 - 24;
            let mantissa = (word >> 16) as u32 & 0x007f_ffff;
            f32::from_bits(sign | (exponent << 23) | mantissa)
        }
    }
}

/// The scalar leg of the skewed-pass scenario.
fn skew_scalar_digest() -> String {
    let factory = ParametricEqFactory;
    let mut hasher = Sha256::new();
    for shape in SKEW_SHAPES {
        let mut effects: Vec<_> = (0..SKEW_TRACKS)
            .map(|track| {
                factory
                    .prepare(request(&skew_configuration(shape, track), false))
                    .expect("scalar prepare")
            })
            .collect();
        let mut position = 0_u64;
        for block in 0..SKEW_BLOCKS {
            let frames = skew_frames(block);
            for (track, effect) in effects.iter_mut().enumerate() {
                let mut left: Vec<f32> = (0..frames)
                    .map(|frame| skew_word(block, frame, track, 0))
                    .collect();
                let mut right: Vec<f32> = (0..frames)
                    .map(|frame| skew_word(block, frame, track, 1))
                    .collect();
                let report = effect.process(
                    EffectProcessBlock::new(&mut left, &mut right, None, position, &[], 128)
                        .expect("scalar block"),
                );
                fold_words(&mut hasher, left.into_iter().chain(right));
                fold_report(&mut hasher, &report);
                fold_payload(&mut hasher, &snapshot(effect.as_ref()));
            }
            position += frames as u64;
        }
    }
    hex(&hasher.finalize())
}

/// The bank legs of the skewed-pass scenario, folded per track in track order so the digest does
/// not depend on the bank width. `mono` renders the collapsed body over the left plane.
fn skew_bank_digest(width: BankWidth, backend: Backend, mono: bool) -> String {
    let lanes = width.lanes() as usize;
    assert_eq!(SKEW_TRACKS % lanes, 0, "the scenario fills whole banks");
    let factory = ParametricEqFactory;
    let mut hasher = Sha256::new();
    for shape in SKEW_SHAPES {
        let configurations: Vec<_> = (0..SKEW_TRACKS)
            .map(|track| skew_configuration(shape, track))
            .collect();
        let mut banks: Vec<_> = configurations
            .chunks(lanes)
            .map(|group| {
                let requests: Vec<_> = group
                    .iter()
                    .map(|initial| request(initial, false))
                    .collect();
                factory
                    .bind_homogeneous_bank(PrepareEffectBankRequest {
                        backend,
                        width,
                        requests: &requests,
                        active_mask: width.full_mask(),
                    })
                    .expect("valid bank request")
                    .expect("the native width must bind")
            })
            .collect();
        let offsets = vec![0_u32; lanes + 1];
        let mut position = 0_u64;
        for block in 0..SKEW_BLOCKS {
            let frames = skew_frames(block);
            for (group, bank) in banks.iter_mut().enumerate() {
                let plane = |channel: usize| -> Vec<f32> {
                    (0..frames * lanes)
                        .map(|cell| {
                            skew_word(block, cell / lanes, group * lanes + cell % lanes, channel)
                        })
                        .collect()
                };
                let mut left = plane(0);
                let mut right = if mono {
                    vec![f32::from_bits(0x7F7F_FFFF); frames * lanes]
                } else {
                    plane(1)
                };
                let process = EffectBankProcessBlock::new(
                    &mut left,
                    &mut right,
                    None,
                    frames as u32,
                    width,
                    position,
                    &[],
                    &offsets,
                    128,
                )
                .expect("bank block");
                let report = if mono {
                    bank.process_bank_mono(process)
                } else {
                    bank.process_bank(process)
                };
                for lane in 0..lanes {
                    let column = |plane: &[f32]| -> Vec<f32> {
                        (0..frames)
                            .map(|frame| plane[frame * lanes + lane])
                            .collect()
                    };
                    if mono {
                        fold_words(&mut hasher, column(&left).into_iter());
                    } else {
                        fold_words(&mut hasher, column(&left).into_iter().chain(column(&right)));
                    }
                    fold_report(&mut hasher, &report.reports[lane]);
                    fold_payload(&mut hasher, &snapshot_bank(bank.as_ref(), lane as u32));
                }
            }
            position += frames as u64;
        }
    }
    hex(&hasher.finalize())
}

/// The digests [`two_and_four_live_sections_render_the_base_bits`] pins, recorded on the
/// unmodified base of issue #978 (every depth-two pass in the interleaved schedule).
const SKEW_DIGESTS: [(&str, &str); 3] = [
    (
        "scalar",
        "9fdeb65d468cd6c5b5aed91c853d0dcb3fb4781dc723ecba223cf67926214dda",
    ),
    (
        "bank",
        "aad039b4e61453d750e40869a0c8b7aa59df2a6299eecfc6430faab0beebcc6a",
    ),
    (
        "bank-mono",
        "602d2f39e13d8431c2db10bb94caebeeb3f64d767261ec6c773c648079106ea4",
    ),
];

/// Issue #978 gate 2: two, four and six live sections -- only depth-two passes, select-free on
/// admitted blocks and masked on refused and all-live ones, with and without dry lanes -- render
/// the bits they rendered before the passes were software-pipelined.
///
/// Through the public API only. The blocks run 1, 2, 3, 37 and 128 frames, so the skewed pass's
/// fallback (one frame), its shortest pipelines and its steady state are all reached, dual and
/// collapsed mono. The input is hostile (`+0.0`, subnormals, magnitudes `2^-24..2^26`) with `-0.0`
/// and non-finite words on some blocks. Every output word, every report and every lane's state
/// payload after every block is folded into one SHA-256 per leg, raw bits.
#[test]
fn two_and_four_live_sections_render_the_base_bits() {
    let mut legs = vec![("scalar", skew_scalar_digest())];
    if let Some((width, backend)) = native_bank() {
        legs.push(("bank", skew_bank_digest(width, backend, false)));
        legs.push(("bank-mono", skew_bank_digest(width, backend, true)));
    }
    for (leg, digest) in &legs {
        println!("skewed-pass digest {leg} {digest}");
    }
    for (leg, digest) in &legs {
        let pinned = SKEW_DIGESTS
            .iter()
            .find(|(name, _)| name == leg)
            .map(|(_, pin)| *pin)
            .expect("every leg is pinned");
        assert_eq!(
            digest, pinned,
            "#978 gate 2: the {leg} leg moved a bit, a report or a state word"
        );
    }
}

/// Tracks in the switched-off-cut scenario: one eight-lane bank, or two four-lane banks.
const CLIFF_TRACKS: usize = 8;
/// Blocks rendered with the cut on (the first starts its ramp), then with it off.
const CLIFF_ON_BLOCKS: usize = 4;
const CLIFF_OFF_BLOCKS: usize = 12;

/// The configurations of [`a_cut_switched_off_keeps_the_bank_eliding`].
#[derive(Clone, Copy, Debug, PartialEq)]
enum CliffShape {
    /// Only the HPF, switched on and off on every lane of both channels: once it is off, every
    /// section is dead and an admitted block runs none of them.
    HpfOnly,
    /// A live bell throughout, and the HPF switched on and off everywhere: once it is off, the
    /// admitted plan is the bell alone.
    BellAndHpf,
}

const CLIFF_SHAPES: [CliffShape; 2] = [CliffShape::HpfOnly, CliffShape::BellAndHpf];

/// One track's prepare-time values and its two prepared-target events: the HPF on at block 0,
/// off (`enabled = 0`) at block [`CLIFF_ON_BLOCKS`], both channels.
fn cliff_configuration(
    shape: CliffShape,
    track: usize,
) -> (Vec<effect_contract::InitialParameterValue>, Vec<OddEvent>) {
    use ParameterChannel::{Left, Right};
    let mut initial = values();
    if shape == CliffShape::BellAndHpf {
        for channel in [Left, Right] {
            odd_band(&mut initial, 1, EqBandKind::Bell, channel, track);
        }
    }
    let mut on = initial.clone();
    let mut changed = vec![false; on.len()];
    for channel in [Left, Right] {
        odd_cut(&mut on, HPF_PARAMETERS, channel, track);
        mark_cut(&mut changed, HPF_PARAMETERS, channel);
    }
    let mut off = on.clone();
    let mut changed_off = vec![false; off.len()];
    for channel in [Left, Right] {
        set_initial(&mut off, HPF_PARAMETERS, channel, 0.0);
        mark_cut(&mut changed_off, HPF_PARAMETERS, channel);
    }
    (
        initial,
        vec![(0, on, changed), (CLIFF_ON_BLOCKS, off, changed_off)],
    )
}

/// One hostile input word: `+0.0` (one in sixteen), a subnormal of either sign (one in sixteen), or
/// a normal with magnitude in `2^-24..2^26`, under its own seed; blocks 7 and 12 carry one `-0.0`
/// (tracks 3 and 6, planes 1 and 0), which refuses the elision for that bank or track.
fn cliff_word(block: usize, frame: usize, track: usize, channel: usize) -> f32 {
    if (block == 7 && frame == 41 && track == 3 && channel == 1)
        || (block == 12 && frame == 99 && track == 6 && channel == 0)
    {
        return -0.0;
    }
    let mut state = ((block as u64) << 40)
        ^ ((frame as u64) << 20)
        ^ ((track as u64) << 4)
        ^ channel as u64
        ^ 0x0979_0979_0979_0979;
    let word = support::splitmix64(&mut state);
    let sign = ((word >> 63) as u32) << 31;
    match word & 15 {
        0 => 0.0,
        1 => f32::from_bits(sign | (((word >> 8) as u32 & 0x007f_ffff) | 1)),
        _ => {
            let exponent = ((word >> 8) % 50) as u32 + 127 - 24;
            let mantissa = (word >> 16) as u32 & 0x007f_ffff;
            f32::from_bits(sign | (exponent << 23) | mantissa)
        }
    }
}

/// VERIFY-EQ finding 1's configuration: a +24 dB bell at 12 kHz (band 1), band 2 disabled, and a
/// 10 Hz LPF, on both channels.
fn overflow_configuration() -> Vec<effect_contract::InitialParameterValue> {
    let mut values = values();
    for channel in [ParameterChannel::Left, ParameterChannel::Right] {
        set_initial(&mut values, 0, channel, 1.0);
        set_initial(&mut values, 1, channel, EqBandKind::Bell as u32 as f32);
        set_initial(&mut values, 2, channel, 12_000.0);
        set_initial(&mut values, 3, channel, 24.0);
        set_initial(&mut values, 4, channel, core::f32::consts::FRAC_1_SQRT_2);
        set_initial(&mut values, LPF_PARAMETERS, channel, 1.0);
        set_initial(&mut values, LPF_PARAMETERS + 1, channel, 10.0);
    }
    values
}

/// A `9e29` sine at 12 kHz (a quarter of 48 kHz), with no `-0.0`: every block is admitted, and the
/// bell turns it into about `1.4e31` ahead of the disabled band.
fn overflow_word(frame: usize) -> f32 {
    let value = 9.0e29 * (core::f32::consts::FRAC_PI_2 * (frame % 4) as f32).sin();
    if value.to_bits() == 0x8000_0000 {
        0.0
    } else {
        value
    }
}

/// The scalar leg of the switched-off-cut scenario, then of the restored-overflow scenario; and how
/// many left-plane faults the overflow scenario reported.
fn cliff_scalar_digest() -> (String, u64) {
    let factory = ParametricEqFactory;
    let mut hasher = Sha256::new();
    for shape in CLIFF_SHAPES {
        let configurations: Vec<_> = (0..CLIFF_TRACKS)
            .map(|track| cliff_configuration(shape, track))
            .collect();
        let mut effects: Vec<_> = configurations
            .iter()
            .map(|(initial, _)| {
                factory
                    .prepare(request(initial, false))
                    .expect("scalar prepare")
            })
            .collect();
        for block in 0..CLIFF_ON_BLOCKS + CLIFF_OFF_BLOCKS {
            for (track, effect) in effects.iter_mut().enumerate() {
                for (at, target, changed) in &configurations[track].1 {
                    if *at == block {
                        apply_prepared_targets(effect.as_mut(), target, changed);
                    }
                }
                let mut left: Vec<f32> = (0..128)
                    .map(|frame| cliff_word(block, frame, track, 0))
                    .collect();
                let mut right: Vec<f32> = (0..128)
                    .map(|frame| cliff_word(block, frame, track, 1))
                    .collect();
                let report = effect.process(
                    EffectProcessBlock::new(
                        &mut left,
                        &mut right,
                        None,
                        (block * 128) as u64,
                        &[],
                        128,
                    )
                    .expect("scalar block"),
                );
                fold_words(&mut hasher, left.into_iter().chain(right));
                fold_report(&mut hasher, &report);
                fold_payload(&mut hasher, &snapshot(effect.as_ref()));
            }
        }
    }
    let values = overflow_configuration();
    let mut effect = factory
        .prepare(request(&values, false))
        .expect("scalar prepare");
    let mut payload = snapshot(effect.as_ref());
    plant_left_ic2(&mut payload, 2, -f32::MAX);
    effect
        .restore_state_payload(
            PARAMETRIC_EQ_DESCRIPTOR.state_layout_version,
            StatePayloadInput::new(
                &payload.0,
                &payload.1,
                &payload.2,
                effect.metadata().state_sizes,
            )
            .expect("state input"),
        )
        .expect("a finite integrator restores");
    let mut faults = 0;
    for block in 0..8 {
        let mut left: Vec<f32> = (0..128).map(overflow_word).collect();
        let mut right = left.clone();
        let report = effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, (block * 128) as u64, &[], 128)
                .expect("scalar block"),
        );
        faults += report.nonfinite_left_blocks;
        fold_words(&mut hasher, left.into_iter().chain(right));
        fold_report(&mut hasher, &report);
        fold_payload(&mut hasher, &snapshot(effect.as_ref()));
    }
    (hex(&hasher.finalize()), faults)
}

/// The bank legs of both scenarios, folded per track in track order so the digest does not depend
/// on the bank width, and the left-plane faults the overflow scenario reported on lane 0. `mono`
/// renders the collapsed body over the left plane.
fn cliff_bank_digest(width: BankWidth, backend: Backend, mono: bool) -> (String, u64) {
    let lanes = width.lanes() as usize;
    assert_eq!(CLIFF_TRACKS % lanes, 0, "the scenario fills whole banks");
    let factory = ParametricEqFactory;
    let mut hasher = Sha256::new();
    let bind = |configurations: &[Vec<effect_contract::InitialParameterValue>]| {
        configurations
            .chunks(lanes)
            .map(|group| {
                let requests: Vec<_> = group
                    .iter()
                    .map(|initial| request(initial, false))
                    .collect();
                factory
                    .bind_homogeneous_bank(PrepareEffectBankRequest {
                        backend,
                        width,
                        requests: &requests,
                        active_mask: width.full_mask(),
                    })
                    .expect("valid bank request")
                    .expect("the native width must bind")
            })
            .collect::<Vec<_>>()
    };
    let offsets = vec![0_u32; lanes + 1];
    let render = |bank: &mut Box<dyn PreparedNativeEffectBank>,
                  hasher: &mut Sha256,
                  block: usize,
                  word: &dyn Fn(usize, usize, usize) -> f32|
     -> u64 {
        let plane = |channel: usize| -> Vec<f32> {
            (0..128 * lanes)
                .map(|cell| word(cell / lanes, cell % lanes, channel))
                .collect()
        };
        let mut left = plane(0);
        let mut right = if mono {
            vec![f32::from_bits(0x7F7F_FFFF); 128 * lanes]
        } else {
            plane(1)
        };
        let process = EffectBankProcessBlock::new(
            &mut left,
            &mut right,
            None,
            128,
            width,
            (block * 128) as u64,
            &[],
            &offsets,
            128,
        )
        .expect("bank block");
        let report = if mono {
            bank.process_bank_mono(process)
        } else {
            bank.process_bank(process)
        };
        for lane in 0..lanes {
            let column = |plane: &[f32]| -> Vec<f32> {
                (0..128).map(|frame| plane[frame * lanes + lane]).collect()
            };
            if mono {
                fold_words(hasher, column(&left).into_iter());
            } else {
                fold_words(hasher, column(&left).into_iter().chain(column(&right)));
            }
            fold_report(hasher, &report.reports[lane]);
            fold_payload(hasher, &snapshot_bank(bank.as_ref(), lane as u32));
        }
        report.reports[0].nonfinite_left_blocks
    };
    for shape in CLIFF_SHAPES {
        let configurations: Vec<_> = (0..CLIFF_TRACKS)
            .map(|track| cliff_configuration(shape, track))
            .collect();
        let initials: Vec<_> = configurations
            .iter()
            .map(|(initial, _)| initial.clone())
            .collect();
        let mut banks = bind(&initials);
        for block in 0..CLIFF_ON_BLOCKS + CLIFF_OFF_BLOCKS {
            for (group, bank) in banks.iter_mut().enumerate() {
                for lane in 0..lanes {
                    for (at, target, changed) in &configurations[group * lanes + lane].1 {
                        if *at == block {
                            apply_prepared_targets_lane(
                                bank.as_mut(),
                                lane,
                                48_000,
                                target,
                                changed,
                            );
                        }
                    }
                }
                let word = |frame: usize, lane: usize, channel: usize| {
                    cliff_word(block, frame, group * lanes + lane, channel)
                };
                render(bank, &mut hasher, block, &word);
            }
        }
    }
    let mut faults = 0;
    let values = overflow_configuration();
    let mut banks = bind(&vec![values; CLIFF_TRACKS]);
    // Tracks 0 and 4 carry the planted integrator: one of them lands in every four-lane bank, and
    // a fault zeroes and resets a whole bank plane, so a four-lane (AArch64 NEON) plan faults and
    // resets exactly the tracks the one eight-lane bank does and the digest does not depend on the
    // bank width (#1017).
    for (group, bank) in banks.iter_mut().enumerate() {
        for lane in 0..lanes {
            if !(group * lanes + lane).is_multiple_of(4) {
                continue;
            }
            let mut payload = snapshot_bank(bank.as_ref(), lane as u32);
            plant_left_ic2(&mut payload, 2, -f32::MAX);
            let sizes = bank.metadata().program_key.state_sizes;
            bank.restore_track_state_payload(
                lane as u32,
                PARAMETRIC_EQ_DESCRIPTOR.state_layout_version,
                StatePayloadInput::new(&payload.0, &payload.1, &payload.2, sizes)
                    .expect("state input"),
            )
            .expect("a finite integrator restores");
        }
    }
    for block in 0..8 {
        for bank in &mut banks {
            let word = |frame: usize, _: usize, _: usize| overflow_word(frame);
            faults += render(bank, &mut hasher, block, &word);
        }
    }
    (hex(&hasher.finalize()), faults)
}

/// The digests [`a_cut_switched_off_keeps_the_bank_eliding`] pins, recorded on the unmodified base
/// of issue #979 (an elided section's state had to be exactly `+0.0`).
///
/// The two bank rows were re-recorded by #1017, whose only change to the scenario is that the
/// overflow leg plants track 4 as well as track 0. Before it planted lane 0 of every bank, which is
/// one track on the eight-lane launch plan and two on a four-lane (AArch64 NEON) plan, so the bank
/// digest depended on the width; with tracks 0 and 4 planted the x86-64-v3 and AArch64 legs pin the
/// same digest. No render changed: the eight-lane leg with the old planting still renders the old
/// rows (`2e0845c6…`, `26a755c1…`), and the scalar row is untouched.
const CLIFF_DIGESTS: [(&str, &str); 3] = [
    (
        "scalar",
        "a34ce0342d321d08dae9a99fa1adc175a2511041035a27cd5b744cff89b1b5f4",
    ),
    (
        "bank",
        "9e6886cfbab5d3a7a95fa50323ec074c060560d6f44ba2134a88ebf9ce7f63cd",
    ),
    (
        "bank-mono",
        "171406a7198ccc9dc1d1288a9c2eb5519fc3d8d336907b982ab035169d1bdde6",
    ),
];

/// Issue #979 gate 1: a dedicated cut switched on and then off again through prepared targets
/// leaves its section at the identity with a frozen, non-zero state, and the wider elision leg (b)
/// that now lets such a bank elide renders the bits the shipped rule rendered.
///
/// Through the public API only: the HPF on every lane of both channels at block 0, four blocks,
/// then a target with `enabled = 0`, twelve more blocks of hostile input (subnormals, `+0.0`,
/// magnitudes `2^-24..2^26`; blocks 7 and 12 carry a `-0.0`), alone and beside a live bell. Then
/// VERIFY-EQ finding 1: a +24 dB bell ahead of a disabled band restored with `ic2 = -f32::MAX` (on
/// tracks 0 and 4 of the bank legs) and a 10 Hz LPF behind it, on a `9e29` sine. The executed band overflows `v3`, the block is zeroed,
/// reset and reported; the capped rule keeps refusing that band, so this leg stays on its pin. Every
/// output word, every report and every lane's state payload after every block, one SHA-256 per leg.
#[test]
fn a_cut_switched_off_keeps_the_bank_eliding() {
    let mut legs = vec![("scalar", cliff_scalar_digest())];
    if let Some((width, backend)) = native_bank() {
        legs.push(("bank", cliff_bank_digest(width, backend, false)));
        legs.push(("bank-mono", cliff_bank_digest(width, backend, true)));
    }
    for (leg, (digest, faults)) in &legs {
        println!("switched-off-cut digest {leg} {digest} (overflow faults {faults})");
    }
    for (leg, (digest, _)) in &legs {
        let pinned = CLIFF_DIGESTS
            .iter()
            .find(|(name, _)| name == leg)
            .map(|(_, pin)| *pin)
            .expect("every leg is pinned");
        assert_eq!(
            digest, pinned,
            "#979 gate 1: the {leg} leg moved a bit, a report or a state word"
        );
    }
    for (leg, (_, faults)) in &legs {
        assert!(
            *faults > 0,
            "non-vacuity: the {leg} leg's executed band must overflow and fault its first block"
        );
    }
}

/// Tracks in the block-limit scenario. Track `t` and track `t + 4` share one voice (`t % 4`): the
/// same configuration and the same input. A fault zeroes and resets a whole bank plane, so on a
/// four-lane host both banks fault on exactly the blocks and planes the one eight-lane bank does,
/// and the per-track digest does not depend on the bank width.
const LIMIT_TRACKS: usize = 8;
/// Blocks in the block-limit scenario.
const LIMIT_BLOCKS: usize = 24;
/// Voices in the block-limit scenario (see [`LIMIT_TRACKS`]).
const LIMIT_VOICES: usize = 4;

/// The configurations of [`admitted_blocks_over_the_block_limit_render_the_base_bits`], named by
/// the stationary cascade's shape on an admitted block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LimitShape {
    /// One live bell boosting 1 kHz by 18 dB: the cascade is one depth-one pass without selects.
    HotBell,
    /// The HPF, the boosting bell and a +12 dB high shelf: one depth-two pass, then the shelf alone.
    HotThree,
    /// The HPF and the boosting bell, then an LPF on some lanes of each channel through prepared
    /// targets: the depth-one pass is the LPF, with dry lanes.
    HotDryLpf,
    /// Two boosting bells: one depth-two pass and no depth-one pass.
    HotPair,
    /// Nothing live: an admitted block runs no section, and its own words meet the limit.
    NothingLive,
    /// One live +6 dB bell whose integrators are restored huge but finite (`MAX`, `-MAX`), which
    /// turns admitted input into infinities and `NaN` inside the cascade.
    RestoredHuge,
}

const LIMIT_SHAPES: [LimitShape; 6] = [
    LimitShape::HotBell,
    LimitShape::HotThree,
    LimitShape::HotDryLpf,
    LimitShape::HotPair,
    LimitShape::NothingLive,
    LimitShape::RestoredHuge,
];

/// Enables general band `band` on both channels; the right channel is detuned, and so is each voice.
fn limit_band(
    values: &mut [effect_contract::InitialParameterValue],
    band: usize,
    kind: EqBandKind,
    frequency: f32,
    gain: f32,
    voice: usize,
) {
    for channel in [ParameterChannel::Left, ParameterChannel::Right] {
        let detune = if matches!(channel, ParameterChannel::Right) {
            1.1
        } else {
            1.0
        };
        let base = band * 6;
        set_initial(values, base, channel, 1.0);
        set_initial(values, base + 1, channel, kind as u32 as f32);
        set_initial(
            values,
            base + 2,
            channel,
            frequency * detune * (1.0 + voice as f32 * 0.05),
        );
        set_initial(values, base + 3, channel, gain);
        set_initial(values, base + 4, channel, 0.8 + voice as f32 * 0.1);
        set_initial(values, base + 5, channel, 1.0);
    }
}

/// One voice's prepare-time values and its prepared cut targets: `(block, final values, changed)`.
fn limit_configuration(
    shape: LimitShape,
    voice: usize,
) -> (Vec<effect_contract::InitialParameterValue>, Vec<OddEvent>) {
    use ParameterChannel::{Left, Right};
    let mut initial = values();
    let mut events = Vec::new();
    match shape {
        LimitShape::HotBell => {
            limit_band(&mut initial, 0, EqBandKind::Bell, 1_000.0, 18.0, voice);
        }
        LimitShape::HotThree => {
            for channel in [Left, Right] {
                odd_cut(&mut initial, HPF_PARAMETERS, channel, voice);
            }
            limit_band(&mut initial, 0, EqBandKind::Bell, 1_000.0, 18.0, voice);
            limit_band(&mut initial, 3, EqBandKind::HighShelf, 6_000.0, 12.0, voice);
        }
        LimitShape::HotDryLpf => {
            for channel in [Left, Right] {
                odd_cut(&mut initial, HPF_PARAMETERS, channel, voice);
            }
            limit_band(&mut initial, 1, EqBandKind::Bell, 1_000.0, 18.0, voice);
            let mut channels = Vec::new();
            if voice < 2 {
                channels.push(Left);
            }
            if voice == 1 || voice == 2 {
                channels.push(Right);
            }
            if !channels.is_empty() {
                let mut target = initial.clone();
                let mut changed = vec![false; target.len()];
                for &channel in &channels {
                    let lane = usize::from(matches!(channel, Right));
                    for parameter in odd_cut(&mut target, LPF_PARAMETERS, channel, voice) {
                        changed[parameter * 2 + lane] = true;
                    }
                }
                events.push((0, target, changed));
            }
        }
        LimitShape::HotPair => {
            limit_band(&mut initial, 0, EqBandKind::Bell, 1_000.0, 18.0, voice);
            limit_band(&mut initial, 2, EqBandKind::Bell, 3_000.0, 12.0, voice);
        }
        LimitShape::NothingLive => {}
        LimitShape::RestoredHuge => {
            limit_band(&mut initial, 0, EqBandKind::Bell, 1_000.0, 6.0, voice);
        }
    }
    (initial, events)
}

/// Frames in block `block`: mostly one quantum, a ragged block every fifth, and one single frame.
fn limit_frames(block: usize) -> usize {
    match block {
        9 => 1,
        _ if block % 5 == 4 => 37,
        _ => 128,
    }
}

/// How loud plane `channel` of block `block` is: 0 cold (hostile small words), 1 warm (a `1e28`
/// sine, inside the limit after every shape's gain), 2 hot (a `9e29` sine, outside it after a
/// boost), 3 edge (the limit itself and its neighbours). The two planes differ, so a block can
/// fault on the left only, on the right only, on both or on neither.
fn limit_heat(block: usize, channel: usize) -> usize {
    let left = [0, 2, 1, 2, 3, 0, 2, 3];
    let right = [0, 0, 2, 2, 3, 1, 3, 1];
    [left, right][channel][block % 8]
}

/// Admitted words at the limit and around it.
const LIMIT_EDGE_WORDS: [f32; 6] = [1.0e30, -1.0e30, 9.999_999e29, -9.999_999e29, 1.0, 5.0e29];

/// One input word for `(block, frame, voice, channel)`. Every word is admitted by the elision gate
/// (finite, at most `1e30` in magnitude, never `-0.0`) except on block 13, which carries one `-0.0`,
/// and block 21, which carries one `NaN`, on voice 0 of one plane: those refuse the bank's elision.
fn limit_word(block: usize, frame: usize, voice: usize, channel: usize) -> f32 {
    if voice == 0 && frame == 3 % limit_frames(block) {
        if block == 13 && channel == 1 {
            return -0.0;
        }
        if block == 21 && channel == 0 {
            return f32::NAN;
        }
    }
    let position = block * 128 + frame;
    let phase = voice as f64 * 0.7 + channel as f64 * 1.9;
    let sine = |amplitude: f64| -> f32 {
        let value = (amplitude
            * (core::f64::consts::TAU * 1_000.0 * position as f64 / 48_000.0 + phase).sin())
            as f32;
        // `+ 0.0` turns a `-0.0` into `+0.0`.
        value + 0.0
    };
    match limit_heat(block, channel) {
        1 => sine(1.0e28),
        2 => sine(9.0e29),
        3 => LIMIT_EDGE_WORDS[(frame + voice + channel) % LIMIT_EDGE_WORDS.len()],
        _ => {
            let mut state = ((block as u64) << 40)
                ^ ((frame as u64) << 20)
                ^ ((voice as u64) << 4)
                ^ channel as u64
                ^ 0x0999_0999_0999_0999;
            let word = support::splitmix64(&mut state);
            let sign = ((word >> 63) as u32) << 31;
            match word & 15 {
                0 => 0.0,
                1 => f32::from_bits(sign | (((word >> 8) as u32 & 0x007f_ffff) | 1)),
                _ => {
                    let exponent = ((word >> 8) % 50) as u32 + 127 - 24;
                    let mantissa = (word >> 16) as u32 & 0x007f_ffff;
                    f32::from_bits(sign | (exponent << 23) | mantissa)
                }
            }
        }
    }
}

/// Sets both integrators of physical section `section` of one channel in a lane payload.
fn plant_integrators(payload: &mut Payload, right: bool, section: usize, ic1: f32, ic2: f32) {
    let lane = if right {
        &mut payload.2
    } else {
        &mut payload.1
    };
    for (offset, value) in [(0, ic1), (1, ic2)] {
        let word = section * support::WORDS_PER_BAND + offset;
        lane[word * 4..word * 4 + 4].copy_from_slice(&value.to_bits().to_le_bytes());
    }
}

/// `Some(right)` when voice 0 of [`LimitShape::RestoredHuge`] receives huge integrators in its
/// live bell (physical section 1) before `block`: the left channel's, then the right one's.
fn limit_restore(shape: LimitShape, block: usize) -> Option<bool> {
    if shape != LimitShape::RestoredHuge {
        return None;
    }
    // Each lands on a block whose plane is otherwise cold, so its fault is the restore's.
    match block {
        5 | 16 => Some(false),
        8 | 17 => Some(true),
        _ => None,
    }
}

/// Fault counts of one shape: blocks that faulted the left plane only, the right plane only, and
/// both, summed over tracks.
type LimitFaults = [u64; 3];

fn limit_tally(faults: &mut LimitFaults, left: u64, right: u64) {
    match (left != 0, right != 0) {
        (true, false) => faults[0] += 1,
        (false, true) => faults[1] += 1,
        (true, true) => faults[2] += 1,
        (false, false) => {}
    }
}

/// One leg of the block-limit scenario: its digest and its fault counts per shape.
type LimitLeg = (String, Vec<LimitFaults>);

/// The scalar leg of the block-limit scenario.
fn limit_scalar_digest() -> LimitLeg {
    let factory = ParametricEqFactory;
    let mut hasher = Sha256::new();
    let mut tallies = Vec::new();
    for shape in LIMIT_SHAPES {
        let mut faults = [0_u64; 3];
        let configurations: Vec<_> = (0..LIMIT_TRACKS)
            .map(|track| limit_configuration(shape, track % LIMIT_VOICES))
            .collect();
        let mut effects: Vec<_> = configurations
            .iter()
            .map(|(initial, _)| {
                factory
                    .prepare(request(initial, false))
                    .expect("scalar prepare")
            })
            .collect();
        let mut position = 0_u64;
        for block in 0..LIMIT_BLOCKS {
            let frames = limit_frames(block);
            for (track, effect) in effects.iter_mut().enumerate() {
                let voice = track % LIMIT_VOICES;
                for (at, target, changed) in &configurations[track].1 {
                    if *at == block {
                        apply_prepared_targets(effect.as_mut(), target, changed);
                    }
                }
                if let Some(right) = limit_restore(shape, block)
                    && voice == 0
                {
                    let mut payload = snapshot(effect.as_ref());
                    plant_integrators(&mut payload, right, 1, f32::MAX, -f32::MAX);
                    effect
                        .restore_state_payload(
                            PARAMETRIC_EQ_DESCRIPTOR.state_layout_version,
                            StatePayloadInput::new(
                                &payload.0,
                                &payload.1,
                                &payload.2,
                                effect.metadata().state_sizes,
                            )
                            .expect("state input"),
                        )
                        .expect("finite integrators restore");
                }
                let mut left: Vec<f32> = (0..frames)
                    .map(|frame| limit_word(block, frame, voice, 0))
                    .collect();
                let mut right: Vec<f32> = (0..frames)
                    .map(|frame| limit_word(block, frame, voice, 1))
                    .collect();
                let report = effect.process(
                    EffectProcessBlock::new(&mut left, &mut right, None, position, &[], 128)
                        .expect("scalar block"),
                );
                limit_tally(
                    &mut faults,
                    report.nonfinite_left_blocks,
                    report.nonfinite_right_blocks,
                );
                fold_words(&mut hasher, left.into_iter().chain(right));
                fold_report(&mut hasher, &report);
                fold_payload(&mut hasher, &snapshot(effect.as_ref()));
            }
            position += frames as u64;
        }
        tallies.push(faults);
    }
    (hex(&hasher.finalize()), tallies)
}

/// The bank legs of the block-limit scenario, folded per track in track order. `mono` renders the
/// collapsed body over the left plane.
fn limit_bank_digest(width: BankWidth, backend: Backend, mono: bool) -> LimitLeg {
    let lanes = width.lanes() as usize;
    assert_eq!(LIMIT_TRACKS % lanes, 0, "the scenario fills whole banks");
    let factory = ParametricEqFactory;
    let mut hasher = Sha256::new();
    let mut tallies = Vec::new();
    for shape in LIMIT_SHAPES {
        let mut faults = [0_u64; 3];
        let configurations: Vec<_> = (0..LIMIT_TRACKS)
            .map(|track| limit_configuration(shape, track % LIMIT_VOICES))
            .collect();
        let mut banks: Vec<_> = configurations
            .chunks(lanes)
            .map(|group| {
                let requests: Vec<_> = group
                    .iter()
                    .map(|(initial, _)| request(initial, false))
                    .collect();
                factory
                    .bind_homogeneous_bank(PrepareEffectBankRequest {
                        backend,
                        width,
                        requests: &requests,
                        active_mask: width.full_mask(),
                    })
                    .expect("valid bank request")
                    .expect("the native width must bind")
            })
            .collect();
        let offsets = vec![0_u32; lanes + 1];
        let mut position = 0_u64;
        for block in 0..LIMIT_BLOCKS {
            let frames = limit_frames(block);
            for (group, bank) in banks.iter_mut().enumerate() {
                assert!(!mono || bank.supports_mono_collapse());
                for lane in 0..lanes {
                    let track = group * lanes + lane;
                    for (at, target, changed) in &configurations[track].1 {
                        if *at == block {
                            apply_prepared_targets_lane(
                                bank.as_mut(),
                                lane,
                                48_000,
                                target,
                                changed,
                            );
                        }
                    }
                    if let Some(right) = limit_restore(shape, block)
                        && track.is_multiple_of(LIMIT_VOICES)
                    {
                        let mut payload = snapshot_bank(bank.as_ref(), lane as u32);
                        plant_integrators(&mut payload, right, 1, f32::MAX, -f32::MAX);
                        let sizes = bank.metadata().program_key.state_sizes;
                        bank.restore_track_state_payload(
                            lane as u32,
                            PARAMETRIC_EQ_DESCRIPTOR.state_layout_version,
                            StatePayloadInput::new(&payload.0, &payload.1, &payload.2, sizes)
                                .expect("state input"),
                        )
                        .expect("finite integrators restore");
                    }
                }
                let plane = |channel: usize| -> Vec<f32> {
                    (0..frames * lanes)
                        .map(|cell| {
                            let track = group * lanes + cell % lanes;
                            limit_word(block, cell / lanes, track % LIMIT_VOICES, channel)
                        })
                        .collect()
                };
                let mut left = plane(0);
                let mut right = if mono {
                    vec![f32::from_bits(0x7F7F_FFFF); frames * lanes]
                } else {
                    plane(1)
                };
                let process = EffectBankProcessBlock::new(
                    &mut left,
                    &mut right,
                    None,
                    frames as u32,
                    width,
                    position,
                    &[],
                    &offsets,
                    128,
                )
                .expect("bank block");
                let report = if mono {
                    bank.process_bank_mono(process)
                } else {
                    bank.process_bank(process)
                };
                for lane in 0..lanes {
                    let column = |plane: &[f32]| -> Vec<f32> {
                        (0..frames)
                            .map(|frame| plane[frame * lanes + lane])
                            .collect()
                    };
                    let entry = &report.reports[lane];
                    if mono {
                        limit_tally(&mut faults, entry.nonfinite_left_blocks, 0);
                        fold_words(&mut hasher, column(&left).into_iter());
                    } else {
                        limit_tally(
                            &mut faults,
                            entry.nonfinite_left_blocks,
                            entry.nonfinite_right_blocks,
                        );
                        fold_words(&mut hasher, column(&left).into_iter().chain(column(&right)));
                    }
                    fold_report(&mut hasher, entry);
                    fold_payload(&mut hasher, &snapshot_bank(bank.as_ref(), lane as u32));
                }
            }
            position += frames as u64;
        }
        tallies.push(faults);
    }
    (hex(&hasher.finalize()), tallies)
}

/// The digests [`admitted_blocks_over_the_block_limit_render_the_base_bits`] pins, recorded on the
/// unmodified base of issue #999 (every block's §4.4 verdict still a separate scan of the planes).
const LIMIT_DIGESTS: [(&str, &str); 3] = [
    (
        "scalar",
        "69929ee05f9192faebe174ef7a6d48a5de4abd584e18e1a4834ec0913b4a7c95",
    ),
    (
        "bank",
        "033bb41c2daae4fcc72b4cc61e62ce74e298cbb0234479be41fbf4f40518ff0c",
    ),
    (
        "bank-mono",
        "0da773b7d5ee458d4175f31b1523a035673dc39c6dc8c16eb5f425335864e289",
    ),
];

/// Issue #999 gate 2: admitted stationary blocks whose output crosses the §4.4 block limit render
/// the bits, reports and states they rendered when every verdict was a separate scan of the planes.
///
/// Through the public API only, 24 blocks of input that every elision leg admits but whose output
/// the boost carries past `1e30` on some planes and not others (left only, right only, both,
/// neither), plus the limit itself and its neighbours: one live bell (a depth-one pass without
/// selects), three live sections (a pair, then the depth-one pass), an LPF with dry lanes as the
/// depth-one pass, two live bells (a pair and nothing after it), nothing live (the words themselves
/// meet the limit), and a bell restored with huge finite integrators, which turns admitted input
/// into infinities and `NaN`. Ramped blocks (the LPF's prepared target), a `-0.0` and a `NaN` refuse
/// the elision where they land. Every output word, every report and every lane's state payload
/// after every block, one SHA-256 per leg (scalar, bank, bank-mono).
///
/// Non-vacuity: every shape but nothing-live faults a plane alone on the left and alone on the
/// right; nothing-live and the restored bell fault.
#[test]
fn admitted_blocks_over_the_block_limit_render_the_base_bits() {
    let mut legs = vec![("scalar", limit_scalar_digest())];
    if let Some((width, backend)) = native_bank() {
        legs.push(("bank", limit_bank_digest(width, backend, false)));
        legs.push(("bank-mono", limit_bank_digest(width, backend, true)));
    }
    for (leg, (digest, faults)) in &legs {
        println!("block-limit digest {leg} {digest}");
        println!(
            "block-limit faults {leg} (left only, right only, both) {faults:?} (shapes {LIMIT_SHAPES:?})"
        );
    }
    for (leg, (_, faults)) in &legs {
        for (shape, [left, right, both]) in LIMIT_SHAPES.iter().zip(faults) {
            assert!(
                left + right + both > 0,
                "non-vacuity: the {leg} leg's {shape:?} shape must fault a plane"
            );
            if *leg != "bank-mono"
                && !matches!(shape, LimitShape::NothingLive | LimitShape::RestoredHuge)
            {
                assert!(
                    *left > 0 && *right > 0,
                    "non-vacuity: the {leg} leg's {shape:?} shape must fault each plane alone"
                );
            }
        }
    }
    for (leg, (digest, _)) in &legs {
        let pinned = LIMIT_DIGESTS
            .iter()
            .find(|(name, _)| name == leg)
            .map(|(_, pin)| *pin)
            .expect("every leg is pinned");
        assert_eq!(
            digest, pinned,
            "#999 gate 2: the {leg} leg moved a bit, a report or a state word"
        );
    }
}
