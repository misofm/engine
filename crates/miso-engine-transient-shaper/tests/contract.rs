//! Contract fixtures: descriptor, coefficients, resources, ramp law, resets and identity rules.
//!
//! Master plan §8: these are **contract** fixtures, not implementation bits. None of their expected
//! values moved in the #92 re-land; if one of them ever does, the job that moved it stops.

mod common;

use common::*;
use miso_engine_effect_contract::{
    BankWidth, EffectPrepareError, LatencySamples, LinkMode, NativeEffectFactory, ParameterChannel,
    PrepareEffectBankRequest, PreparedNativeEffect, ResetKind, StatePayloadError,
    StatePayloadInput, TailSamples, validate_descriptor_v1,
};
use miso_engine_effect_runtime::envelope::retention_coefficient;
use miso_engine_lane::Backend;
use miso_engine_transient_shaper::{
    TRANSIENT_SHAPER_COEFFICIENT_BITS_V1, TRANSIENT_SHAPER_DESCRIPTOR_V1,
    TRANSIENT_SHAPER_TIME_CONSTANTS_MS, TransientShaperFactory,
};

/// Red mutation: flip one bit of `TRANSIENT_SHAPER_COEFFICIENT_BITS_V1`.
#[test]
fn descriptor_coefficients_resources_and_transactional_caps_are_frozen() {
    validate_descriptor_v1(&TRANSIENT_SHAPER_DESCRIPTOR_V1).expect("descriptor");
    assert_eq!(
        TRANSIENT_SHAPER_DESCRIPTOR_V1.id.as_str(),
        "miso.transient-shaper"
    );
    assert_eq!(TRANSIENT_SHAPER_DESCRIPTOR_V1.parameters.len(), 3);
    assert_eq!(TRANSIENT_SHAPER_DESCRIPTOR_V1.qualities.len(), 4);
    assert_eq!(TRANSIENT_SHAPER_DESCRIPTOR_V1.state_layout_version, 1);
    for (quality, bits) in TRANSIENT_SHAPER_DESCRIPTOR_V1
        .qualities
        .iter()
        .zip(TRANSIENT_SHAPER_COEFFICIENT_BITS_V1)
    {
        assert_eq!(quality.latency, LatencySamples(0));
        assert_eq!(quality.tail, TailSamples::Finite(0));
        assert_eq!(quality.maximum_state.total(), Some(88));
        assert_eq!(quality.maximum_state.common_bytes, 0);
        assert_eq!(quality.maximum_state.left_bytes, LANE_STATE_BYTES as u32);
        assert_eq!(quality.maximum_state.right_bytes, LANE_STATE_BYTES as u32);
        // Re-accounting this row is #95's (audit finding F9); this job only fixed the stale doc.
        assert_eq!(quality.scratch_fixed_bytes, 24);
        assert_eq!(quality.scratch_bytes_per_frame, 0);
        assert_eq!(bits.len(), 4);
    }
    let values = initial_values();
    let mut too_small = request(&values);
    too_small.limits.maximum_total_state_bytes = 87;
    assert!(matches!(
        TransientShaperFactory.prepare(too_small),
        Err(EffectPrepareError {
            code: "effect.resource.limit"
        })
    ));
    let mut scratch_below = request(&values);
    scratch_below.limits.maximum_scratch_bytes = 23;
    assert!(matches!(
        TransientShaperFactory.prepare(scratch_below),
        Err(EffectPrepareError {
            code: "effect.resource.limit"
        })
    ));
    let mut negative_zero = values;
    negative_zero[0].value = -0.0;
    assert!(matches!(
        TransientShaperFactory.prepare(request(&negative_zero)),
        Err(EffectPrepareError {
            code: "effect.parameter.initial"
        })
    ));
}

/// Every frozen coefficient bit is `exp(-1 / (tau * fs))` rounded once, from two independent
/// directions: the `f64` oracle in `dsp-reference`, and the runtime's own coefficient design.
///
/// The second half is the bit-compatibility check the re-land owes: this crate's frozen table and
/// `miso_engine_effect_runtime::envelope::retention_coefficient` must be the same number, or the
/// crate is carrying a private coefficient policy after the unification.
///
/// Red mutation: `retention_coefficient` returning `1 - exp(...)` (the rate rather than the pole),
/// or one flipped bit in the table.
#[test]
fn independent_coefficients_time_constants_layout_and_both_caps_are_exact() {
    const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
    let values = initial_values();
    for ((sample_rate, production_bits), quality) in RATES
        .into_iter()
        .zip(TRANSIENT_SHAPER_COEFFICIENT_BITS_V1)
        .zip(TRANSIENT_SHAPER_DESCRIPTOR_V1.qualities)
    {
        assert_eq!(quality.sample_rate, sample_rate);
        for ((time_ms, bits), index) in TRANSIENT_SHAPER_TIME_CONSTANTS_MS
            .into_iter()
            .zip(production_bits)
            .zip(0..TRANSIENT_SHAPER_TIME_CONSTANTS_MS.len())
        {
            let independent = miso_engine_dsp_reference::reference_transient_shaper_coefficient(
                f64::from(time_ms),
                f64::from(sample_rate),
            )
            .expect("independent coefficient");
            assert_eq!(
                (independent as f32).to_bits(),
                bits,
                "rate={sample_rate} coefficient={index}"
            );
            assert_eq!(
                retention_coefficient(time_ms, sample_rate).to_bits(),
                bits,
                "the runtime's coefficient design must reproduce the frozen bits: \
                 rate={sample_rate} coefficient={index}"
            );
            let retained = f64::from(f32::from_bits(bits));
            let recovered_time_ms = -1000.0 / (f64::from(sample_rate) * retained.ln());
            let timing_tolerance_ms =
                (1000.0 / f64::from(sample_rate)).max(f64::from(time_ms) * 0.02);
            assert!(
                (recovered_time_ms - f64::from(time_ms)).abs() <= timing_tolerance_ms,
                "rate={sample_rate} coefficient={index} recovered={recovered_time_ms}"
            );
        }
        let effect = prepare_with(&values, sample_rate, false, LinkMode::DualMono);
        let state = snapshot(effect.as_ref());
        let expected =
            expected_lane_bytes([0.0, 0.0], [(0.0, 0.0, 0), (0.0, 0.0, 0), (1.0, 1.0, 0)]);
        assert_eq!(state.0.len(), LANE_STATE_BYTES);
        assert_eq!(state.0, expected);
        assert_eq!(state.1, expected);
    }
}

/// The D11 ramp law, pinned sample by sample, and an exact mid-ramp restore.
///
/// The four pinned values are exact in `f32` under both the pre-audit per-sample division and the
/// D11 precomputed step, which is why this contract fixture did not move. The restore is where the
/// two laws could have diverged: `step` is not persisted, it is derived as
/// `(target - current) / remaining`, and for this row `(-1 - 0.96875) / 63` is exactly `-0.03125`,
/// the same increment the uninterrupted ramp is carrying.
///
/// Red mutation: `set_target` dividing by 63 instead of 64; or `read_lane` deriving `step` from
/// `RAMP_SAMPLES` instead of `remaining`, which makes the restored continuation differ.
#[test]
fn automation_updates_one_sixty_three_sixty_four_retargets_and_restores_exactly() {
    let values = initial_values();
    let mut effect = prepare(&values);
    let initial_right = snapshot(effect.as_ref()).1;

    let render = |effect: &mut Box<dyn PreparedNativeEffect>,
                  frames: usize,
                  first: u64,
                  spans: &[miso_engine_effect_contract::PreparedAutomationSpan]| {
        let mut left = vec![0.0_f32; frames];
        let mut right = vec![0.0_f32; frames];
        effect.process(
            miso_engine_effect_contract::EffectProcessBlock::new(
                &mut left, &mut right, None, first, spans, 128,
            )
            .expect("block"),
        );
    };

    render(
        &mut effect,
        1,
        0,
        &[point(ParameterChannel::Left, 0, 0, 1.0)],
    );
    let after_one = snapshot(effect.as_ref());
    assert_eq!(
        state_f32(&after_one.0, 2).to_bits(),
        (1.0_f32 / 64.0).to_bits()
    );
    assert_eq!(state_f32(&after_one.0, 3).to_bits(), 1.0_f32.to_bits());
    assert_eq!(state_u32(&after_one.0, 4), 63);
    assert_eq!(after_one.1, initial_right);

    render(&mut effect, 62, 1, &[]);
    let after_sixty_three = snapshot(effect.as_ref());
    assert_eq!(
        state_f32(&after_sixty_three.0, 2).to_bits(),
        (63.0_f32 / 64.0).to_bits()
    );
    assert_eq!(state_u32(&after_sixty_three.0, 4), 1);
    assert_eq!(after_sixty_three.1, initial_right);

    render(&mut effect, 1, 63, &[]);
    let after_sixty_four = snapshot(effect.as_ref());
    assert_eq!(
        state_f32(&after_sixty_four.0, 2).to_bits(),
        1.0_f32.to_bits()
    );
    assert_eq!(state_u32(&after_sixty_four.0, 4), 0);
    assert_eq!(after_sixty_four.1, initial_right);

    render(
        &mut effect,
        1,
        64,
        &[point(ParameterChannel::Left, 0, 64, -1.0)],
    );
    let active = snapshot(effect.as_ref());
    assert_eq!(state_f32(&active.0, 2).to_bits(), 0.96875_f32.to_bits());
    assert_eq!(state_f32(&active.0, 3).to_bits(), (-1.0_f32).to_bits());
    assert_eq!(state_u32(&active.0, 4), 63);
    assert_eq!(active.1, initial_right);

    let mut restored = prepare(&values);
    restored
        .restore_state_payload(
            1,
            StatePayloadInput::new(&[], &active.0, &active.1, restored.metadata().state_sizes)
                .expect("active state"),
        )
        .expect("active restore");
    let mut uninterrupted_left = [0.25_f32; 8];
    let mut uninterrupted_right = [-0.125_f32; 8];
    let mut restored_left = uninterrupted_left;
    let mut restored_right = uninterrupted_right;
    effect.process(
        miso_engine_effect_contract::EffectProcessBlock::new(
            &mut uninterrupted_left,
            &mut uninterrupted_right,
            None,
            65,
            &[],
            128,
        )
        .expect("uninterrupted"),
    );
    restored.process(
        miso_engine_effect_contract::EffectProcessBlock::new(
            &mut restored_left,
            &mut restored_right,
            None,
            65,
            &[],
            128,
        )
        .expect("restored"),
    );
    assert_eq!(
        uninterrupted_left.map(f32::to_bits),
        restored_left.map(f32::to_bits)
    );
    assert_eq!(
        uninterrupted_right.map(f32::to_bits),
        restored_right.map(f32::to_bits)
    );
    assert_eq!(snapshot(effect.as_ref()), snapshot(restored.as_ref()));
}

/// Red mutation: skip the envelope clear on the discontinuity reset.
#[test]
fn both_resets_have_word_exact_parameter_and_envelope_states() {
    let mut values = initial_values();
    for (index, value) in [0.25_f32, -0.25, 0.5, -0.5, 0.75, 0.5]
        .into_iter()
        .enumerate()
    {
        values[index].value = value;
    }
    let mut effect = prepare(&values);
    let mut left = [0.8_f32; 8];
    let mut right = [0.2_f32; 8];
    effect.process(
        miso_engine_effect_contract::EffectProcessBlock::new(
            &mut left,
            &mut right,
            None,
            0,
            &[point(ParameterChannel::Left, 0, 0, 1.0)],
            128,
        )
        .expect("active state"),
    );

    effect.reset(ResetKind::DiscontinuityKeepParameters);
    let discontinuity = snapshot(effect.as_ref());
    assert_eq!(
        discontinuity.0,
        expected_lane_bytes([0.0, 0.0], [(1.0, 1.0, 0), (0.5, 0.5, 0), (0.75, 0.75, 0)])
    );
    assert_eq!(
        discontinuity.1,
        expected_lane_bytes(
            [0.0, 0.0],
            [(-0.25, -0.25, 0), (-0.5, -0.5, 0), (0.5, 0.5, 0)]
        )
    );

    effect.reset(ResetKind::FullToDefaults);
    let full = snapshot(effect.as_ref());
    assert_eq!(
        full.0,
        expected_lane_bytes(
            [0.0, 0.0],
            [(0.25, 0.25, 0), (0.5, 0.5, 0), (0.75, 0.75, 0)]
        )
    );
    assert_eq!(
        full.1,
        expected_lane_bytes(
            [0.0, 0.0],
            [(-0.25, -0.25, 0), (-0.5, -0.5, 0), (0.5, 0.5, 0)]
        )
    );
}

/// Defaults, bypass and `mix = 0` return the input bits exactly, including `-0.0`, while the
/// followers still warm.
///
/// `exp2_lane(0)` is exactly `1`, so `shape == 0` already gives `gain == 1`; the identity *select*
/// is what preserves a signed zero, because `fma(mix, x * 1 - x, x)` maps `-0.0` to `+0.0`.
///
/// Red mutation: drop the `shape == 0` term from the identity mask — the `-0.0` rows go red.
#[test]
fn identity_rules_are_bit_exact_and_the_followers_still_warm() {
    let cases: [(&str, Box<dyn PreparedNativeEffect>); 3] = [
        ("defaults", prepare(&initial_values())),
        (
            "bypass",
            prepare_with(&values_of(1.0, 1.0, 1.0), 48_000, true, LinkMode::DualMono),
        ),
        ("mix-zero", prepare(&values_of(1.0, 1.0, 0.0))),
    ];
    for (name, mut effect) in cases {
        let mut left = [-0.0_f32, 0.25, -0.5, 0.0, 0.8];
        let mut right = [0.0_f32, -0.125, 0.75, -0.0, 0.4];
        let original_left = left.map(f32::to_bits);
        let original_right = right.map(f32::to_bits);
        effect.process(
            miso_engine_effect_contract::EffectProcessBlock::new(
                &mut left,
                &mut right,
                None,
                0,
                &[],
                128,
            )
            .expect("identity block"),
        );
        assert_eq!(left.map(f32::to_bits), original_left, "{name} left");
        assert_eq!(right.map(f32::to_bits), original_right, "{name} right");
        let state = snapshot(effect.as_ref());
        assert!(state_f32(&state.0, 0) > 0.0, "{name}: fast must warm");
        assert!(state_f32(&state.0, 1) > 0.0, "{name}: slow must warm");
        assert!(state_f32(&state.1, 0) > 0.0, "{name}: fast must warm");
        assert!(state_f32(&state.1, 1) > 0.0, "{name}: slow must warm");
    }
}

/// The three link modes, read straight off the envelope state after one frame.
///
/// From a zeroed envelope the first rising sample gives `e = attack * 0 + (1 - attack) * u`, which
/// is exactly `(1 - attack) * u` in `f32`, so the detector each channel saw is readable from the
/// state word to the last bit. That pins the linking law itself — including `Average`'s frozen
/// `0.5 * |l| + 0.5 * |r|` operation order — instead of a downstream consequence of it.
///
/// Red mutation: swap the `LINK_MAXIMUM` and `LINK_AVERAGE` constants; or write `Average` as
/// `0.5 * (|l| + |r|)`, one rounding fewer.
#[test]
fn link_modes_drive_the_detector_as_specified() {
    let values = values_of(1.0, 0.0, 1.0);
    let one_minus_attack = 1.0_f32 - f32::from_bits(TRANSIENT_SHAPER_COEFFICIENT_BITS_V1[1][0]);
    let half = 0.5_f32;
    let expected = [
        (LinkMode::DualMono, 1.0_f32, 0.25_f32),
        (LinkMode::Maximum, 1.0, 1.0),
        (
            LinkMode::Average,
            half * 1.0 + half * 0.25,
            half * 1.0 + half * 0.25,
        ),
    ];
    for (link_mode, left_detector, right_detector) in expected {
        let mut effect = prepare_with(&values, 48_000, false, link_mode);
        let mut left = [-1.0_f32];
        let mut right = [0.25_f32];
        effect.process(
            miso_engine_effect_contract::EffectProcessBlock::new(
                &mut left,
                &mut right,
                None,
                0,
                &[],
                128,
            )
            .expect("link block"),
        );
        let state = snapshot(effect.as_ref());
        assert_eq!(
            state_f32(&state.0, 0).to_bits(),
            (one_minus_attack * left_detector).to_bits(),
            "{link_mode:?} left detector"
        );
        assert_eq!(
            state_f32(&state.1, 0).to_bits(),
            (one_minus_attack * right_detector).to_bits(),
            "{link_mode:?} right detector"
        );
    }
}

/// A restore is atomic and every word is validated.
///
/// Red mutation: accept `remaining > RAMP_SAMPLES`, or drop the `valid_envelope` check.
#[test]
fn state_restore_validates_version_length_envelope_and_parameters() {
    let values = initial_values();
    let mut effect = prepare(&values);
    let sizes = effect.metadata().state_sizes;
    let good = snapshot(effect.as_ref());

    assert_eq!(
        effect.restore_state_payload(
            2,
            StatePayloadInput::new(&[], &good.0, &good.1, sizes).expect("input"),
        ),
        Err(StatePayloadError {
            code: "effect.state.version"
        })
    );

    for (word, bits, code) in [
        (0_usize, f32::NAN.to_bits(), "effect.state.envelope"),
        (1, (-1.0_f32).to_bits(), "effect.state.envelope"),
        (2, 2.0_f32.to_bits(), "effect.state.parameter"),
        (4, 65_u32, "effect.state.parameter"),
    ] {
        let mut bad = good.0.clone();
        bad[word * 4..word * 4 + 4].copy_from_slice(&bits.to_le_bytes());
        assert_eq!(
            effect.restore_state_payload(
                1,
                StatePayloadInput::new(&[], &bad, &good.1, sizes).expect("input"),
            ),
            Err(StatePayloadError { code }),
            "word {word}"
        );
        assert_eq!(snapshot(effect.as_ref()), good, "rejection must be atomic");
    }
}

/// Resource validation and the program-key check precede the legal "backend unavailable" fallback,
/// and a width this build does not render is unavailable rather than an error.
///
/// Red mutation: return `Err("effect.bank.program")` on a program mismatch (the divergence the 83c
/// write-up found in `true-peak-limiter`) — the heterogeneous row goes red.
#[test]
fn bank_resources_and_validation_precede_legal_unavailable_fallback() {
    let factory = TransientShaperFactory;
    let (backend, width) = foreign_bank();
    let lanes = width.lanes() as usize;
    let values = vec![initial_values(); lanes];
    let requests = values
        .iter()
        .map(|values| request(values))
        .collect::<Vec<_>>();
    assert!(
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &requests,
            })
            .expect("legal unavailable fallback")
            .is_none()
    );

    let mut malformed_values = values.clone();
    malformed_values[lanes - 1][0].value = f32::NAN;
    let malformed_requests = malformed_values
        .iter()
        .map(|values| request(values))
        .collect::<Vec<_>>();
    assert_eq!(
        bank_error(factory.bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &malformed_requests,
        }))
        .code,
        "effect.parameter.initial"
    );

    for limit in ["state", "scratch"] {
        let mut below = requests.clone();
        if limit == "state" {
            below[lanes - 1].limits.maximum_total_state_bytes = 87;
        } else {
            below[lanes - 1].limits.maximum_scratch_bytes = 23;
        }
        assert_eq!(
            bank_error(factory.bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &below,
            }))
            .code,
            "effect.resource.limit"
        );
    }

    let mut heterogeneous = requests.clone();
    heterogeneous[lanes - 1].bypass = true;
    assert!(
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &heterogeneous,
            })
            .expect("valid heterogeneous fallback")
            .is_none()
    );

    // A backend and a width that do not describe the same lane count is a malformed request.
    assert_eq!(
        bank_error(factory.bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: Backend::Simd8,
            width: BankWidth::Four,
            requests: &requests,
        }))
        .code,
        "effect.bank.requests"
    );
}
