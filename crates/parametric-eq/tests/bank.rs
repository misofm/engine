#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! Width and bank gates: the same body at `WIDTH` 1, 4 and 8, proven by `to_bits` identity.
//!
//! Decision D5 replaces the old tolerance-based cross-backend comparison with bit identity. That is
//! affordable here because there is exactly one realization: the scalar effect is
//! `Channel<f32, 1>` and a bank is `Channel<Simd4, 4>` or `Channel<Simd8, 8>` of the same generic
//! body, so a difference would be a `Lane` defect, not an effect defect.

mod support;

use effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectProcessBlock, NativeEffectFactory, ParameterChannel,
    PrepareEffectBankRequest, PreparedEffectTarget, PreparedNativeEffectBank, StatePayloadInput,
    StatePayloadOutput, TailSamples,
};
use lane::Backend;
use parametric_eq::{EqBandKind, ParametricEqFactory, design_svf};
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
            })
            .expect("a declined bank is not an error")
            .is_none()
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

/// Descriptive only (AGENTS.md: benchmarks are descriptive during feature development).
///
/// One warmup round and two measured rounds of the production shape: a `WIDTH`-wide bank, a
/// 128-frame block, four sections, both channels, no ramp in flight, nothing hashed or checked
/// inside the timed interval. Run with `--ignored --nocapture`.
#[test]
#[ignore = "descriptive measurement"]
fn descriptive_bank_throughput() {
    use std::time::Instant;
    let Some((width, backend)) = native_bank() else {
        return;
    };
    let lanes = width.lanes() as usize;
    let values_by_track: Vec<_> = (0..lanes).map(configured_values).collect();
    let requests: Vec<_> = values_by_track
        .iter()
        .map(|values| request(values, false))
        .collect();
    let mut bank = ParametricEqFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
        })
        .expect("request")
        .expect("native width binds");
    let frames = 128_usize;
    let blocks = 20_000_u64;
    let mut left: Vec<f32> = (0..frames * lanes)
        .map(|index| ((index as f32) * 0.011).sin() * 0.25)
        .collect();
    let mut right = left.clone();
    let offsets = vec![0_u32; lanes + 1];
    let mut round = |count: u64| {
        let start = Instant::now();
        for block in 0..count {
            bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut left,
                    &mut right,
                    None,
                    frames as u32,
                    width,
                    block * frames as u64,
                    &[],
                    &offsets,
                    128,
                )
                .expect("block"),
            );
        }
        start.elapsed().as_nanos() as f64 / (count as f64 * frames as f64)
    };
    round(blocks / 4);
    let first = round(blocks);
    let second = round(blocks);
    println!(
        "issue-087 bank throughput width={lanes} frames={frames} \
         ns_per_frame_round1={first:.3} ns_per_frame_round2={second:.3} \
         ns_per_frame_per_track_round2={:.4}",
        second / lanes as f64
    );
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

fn fold_words(hasher: &mut Sha256, words: impl Iterator<Item = f32>) {
    for word in words {
        hasher.update(word.to_bits().to_le_bytes());
    }
}

fn fold_payload(hasher: &mut Sha256, payload: &Payload) {
    hasher.update(payload.0);
    hasher.update(payload.1);
    hasher.update(payload.2);
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
/// payload after every block is folded into one SHA-256 per leg; state words are hashed as raw
/// bits (no NaN reaches an output word: a non-finite block is zeroed).
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
