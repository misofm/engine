//! Width and bank gates: the same body at `WIDTH` 1, 4 and 8, proven by `to_bits` identity.
//!
//! Decision D5 replaces the old tolerance-based cross-backend comparison with bit identity. That is
//! affordable here because there is exactly one realization: the scalar effect is
//! `Channel<f32, 1>` and a bank is `Channel<Simd4, 4>` or `Channel<Simd8, 8>` of the same generic
//! body, so a difference would be a `Lane` defect, not an effect defect.

mod support;

use miso_engine_core::KernelBackendV1;
use miso_engine_effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectProcessBlock, NativeEffectFactory, ParameterChannel,
    PrepareEffectBankRequest, PreparedAutomationSpan, PreparedNativeEffectBank, StatePayloadInput,
    StatePayloadOutput, TailSamples,
};
use miso_engine_parametric_eq::ParametricEqFactory;
use support::{COMMON_BYTES, LANE_BYTES, Payload, point, request, set_initial, snapshot, values};

/// The bank width and backend this build actually executes, or `None` on a scalar-only target.
fn native_bank() -> Option<(BankWidth, KernelBackendV1)> {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        return Some((BankWidth::Eight, KernelBackendV1::X86Avx2Fma));
    }
    #[cfg(target_arch = "aarch64")]
    {
        return Some((BankWidth::Four, KernelBackendV1::Aarch64Neon));
    }
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    {
        return Some((BankWidth::Four, KernelBackendV1::WasmSimd128));
    }
    #[allow(unreachable_code)]
    None
}

/// A backend this build cannot execute, for the declining path.
fn foreign_bank() -> (BankWidth, KernelBackendV1) {
    match native_bank() {
        Some((BankWidth::Eight, _)) => (BankWidth::Four, KernelBackendV1::WasmSimd128),
        _ => (BankWidth::Eight, KernelBackendV1::X86Avx2Fma),
    }
}

/// A distinct four-band configuration per track, so no two lanes share coefficients.
fn configured_values(track: usize) -> Vec<miso_engine_effect_contract::InitialParameterValue> {
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

/// Automation for one track: a gain point on the left and a Q point on the right.
fn track_automation(track: usize, sample: u64) -> [PreparedAutomationSpan; 2] {
    [
        point(3, ParameterChannel::Left, sample, -4.0 + track as f32 * 0.5),
        point(
            4,
            ParameterChannel::Right,
            sample,
            0.8 + track as f32 * 0.01,
        ),
    ]
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

    // Blocks of 16, 128, 64 and 128 frames, with an automation event in the first and third block
    // for different tracks, so ramps of different ages coexist inside one bank block.
    let mut position = 0_u64;
    for (index, frames) in [16_usize, 128, 64, 128].into_iter().enumerate() {
        let automation_by_track: Vec<Vec<PreparedAutomationSpan>> = (0..lanes)
            .map(|track| {
                if (index == 0 && track % 2 == 0) || (index == 2 && track % 2 == 1) {
                    track_automation(track, position).to_vec()
                } else {
                    Vec::new()
                }
            })
            .collect();
        let mut automation = Vec::new();
        let mut offsets = vec![0_u32];
        for spans in &automation_by_track {
            automation.extend_from_slice(spans);
            offsets.push(automation.len() as u32);
        }

        let mut bank_left = vec![0.0_f32; frames * lanes];
        let mut bank_right = vec![0.0_f32; frames * lanes];
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
                        &automation_by_track[track],
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
                &automation,
                &offsets,
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
        2,
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

    let automation: Vec<PreparedAutomationSpan> = (0..lanes)
        .flat_map(|track| track_automation(track, 0))
        .collect();
    let offsets: Vec<u32> = (0..=lanes).map(|track| (track * 2) as u32).collect();
    let empty_offsets = vec![0_u32; lanes + 1];

    whole.process_bank(
        EffectBankProcessBlock::new(
            &mut whole_left,
            &mut whole_right,
            None,
            frames as u32,
            width,
            0,
            &automation,
            &offsets,
            128,
        )
        .expect("whole block"),
    );

    let mut first = 0_usize;
    for (index, chunk) in [1_usize, 7, 64, 56].into_iter().enumerate() {
        let spans: &[PreparedAutomationSpan] = if index == 0 { &automation } else { &[] };
        let chunk_offsets: &[u32] = if index == 0 { &offsets } else { &empty_offsets };
        split.process_bank(
            EffectBankProcessBlock::new(
                &mut split_left[first * lanes..(first + chunk) * lanes],
                &mut split_right[first * lanes..(first + chunk) * lanes],
                None,
                chunk as u32,
                width,
                first as u64,
                spans,
                chunk_offsets,
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

/// A malformed shape and a width this build cannot execute are both a legal non-bank, never an
/// error and never a silent fallback to a different width.
#[test]
fn bank_binding_rejects_malformed_shapes_and_declines_a_foreign_width() {
    let factory = ParametricEqFactory;
    let values = values();
    let request = request(&values, false);
    let (foreign_width, foreign_backend) = foreign_bank();
    for (width, backend, count) in [
        (BankWidth::Four, KernelBackendV1::X86Avx2Fma, 4),
        (BankWidth::Eight, KernelBackendV1::X86Avx2Fma, 4),
        (
            foreign_width,
            foreign_backend,
            foreign_width.lanes() as usize,
        ),
    ] {
        let requests = vec![request; count];
        assert!(
            factory
                .bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend,
                    width,
                    requests: &requests,
                })
                .expect("a declined bank is not an error")
                .is_none(),
            "{width:?} {backend:?} must decline"
        );
    }
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
