//! Contract gates: descriptor, automation, state payload, bypass, resets.
//!
//! The descriptor, the parameter identifiers, the automation validation rules and the port and
//! latency contract are unchanged by issue #87. The state payload shape changed during prelaunch
//! development because the retained words are no longer four direct histories and four parameter ramps but two
//! integrators, six coefficient words, six increments and the four ramp target parameters.

mod support;

use effect_contract::{
    AutomationRate, EffectProcessBlock, NativeEffectFactory, ParameterChannel, ParameterDomain,
    ParameterMapping, ResetKind, SmoothingRule, StatePayloadError, StatePayloadInput,
    StatePayloadOutput, validate_descriptor,
};
use engine::SampleRateHz;
use parametric_eq::{
    EQ_BAND_DESCRIPTORS, EQ_SECTION_COUNT, EqBandKind, EqSvfWords, PARAMETRIC_EQ_DESCRIPTOR,
    ParametricEqFactory, design_svf,
};
use support::{
    COMMON_BYTES, LANE_BYTES, SECTIONS, WORDS_PER_BAND, band_word, point, process_zeros, request,
    set_initial, single_section_values, snapshot, values, word,
};

/// The frozen public surface: identifiers, domains, smoothing and the current state size.
#[test]
fn descriptor_is_frozen() {
    validate_descriptor(&PARAMETRIC_EQ_DESCRIPTOR).expect("descriptor");
    let parameters = PARAMETRIC_EQ_DESCRIPTOR.parameters;
    assert_eq!(parameters.len(), 24);
    assert_eq!(PARAMETRIC_EQ_DESCRIPTOR.state_layout_version, 1);
    for quality in PARAMETRIC_EQ_DESCRIPTOR.qualities {
        assert_eq!(quality.maximum_state.common_bytes, COMMON_BYTES as u32);
        assert_eq!(quality.maximum_state.left_bytes, LANE_BYTES as u32);
        assert_eq!(quality.maximum_state.right_bytes, LANE_BYTES as u32);
        assert_eq!(quality.latency.0, 0);
    }
    let names = ["enabled", "kind", "frequency", "gain", "q", "shelf-slope"];
    let domains = [
        ParameterDomain::Boolean,
        ParameterDomain::Enumeration,
        ParameterDomain::Continuous,
        ParameterDomain::Continuous,
        ParameterDomain::Continuous,
        ParameterDomain::Continuous,
    ];
    let mappings = [
        ParameterMapping::Stepped,
        ParameterMapping::Stepped,
        ParameterMapping::Logarithmic,
        ParameterMapping::Linear,
        ParameterMapping::Logarithmic,
        ParameterMapping::Linear,
    ];
    let frequencies = [80.0_f32, 400.0, 2_000.0, 10_000.0];
    for (band, descriptor) in EQ_BAND_DESCRIPTORS.iter().enumerate() {
        let base = band as u32 * 16 + 1;
        assert_eq!(descriptor.index, band as u8);
        assert_eq!(descriptor.cascade_order, band as u8);
        assert_eq!(
            [
                descriptor.enabled.0,
                descriptor.kind.0,
                descriptor.frequency_hz.0,
                descriptor.gain_db.0,
                descriptor.q.0,
                descriptor.shelf_slope.0,
            ],
            core::array::from_fn::<u32, 6, _>(|field| base + field as u32)
        );
        for field in 0..6 {
            let parameter = &parameters[band * 6 + field];
            assert_eq!(parameter.id.0, base + field as u32);
            assert_eq!(
                parameter.display_name,
                format!("band-{}-{}", band + 1, names[field])
            );
            assert_eq!(parameter.domain, domains[field]);
            assert_eq!(parameter.mapping, mappings[field]);
            assert_eq!(parameter.automatable, field >= 2);
            assert_eq!(
                parameter.automation_rate,
                if field >= 2 {
                    AutomationRate::Block
                } else {
                    AutomationRate::None
                }
            );
            assert_eq!(
                parameter.smoothing,
                if field >= 2 {
                    SmoothingRule::Linear
                } else {
                    SmoothingRule::None
                }
            );
            assert_eq!(parameter.smoothing_samples, if field >= 2 { 64 } else { 0 });
        }
        assert_eq!(parameters[band * 6 + 2].default_value, frequencies[band]);
        assert_eq!(parameters[band * 6 + 1].enum_choices.len(), 6);
    }
    assert_eq!(SECTIONS, EQ_SECTION_COUNT);
    assert_eq!(SECTIONS * WORDS_PER_BAND * 4, LANE_BYTES);
}

/// Enables band one as a bell at `1 kHz`, `0 dB`, on the left channel only.
fn bell_values() -> Vec<effect_contract::InitialParameterValue> {
    let mut values = values();
    set_initial(&mut values, 0, ParameterChannel::Left, 1.0);
    set_initial(&mut values, 2, ParameterChannel::Left, 1_000.0);
    values
}

fn expected_words(gain: f32) -> EqSvfWords {
    design_svf(
        EqBandKind::Bell,
        1_000.0,
        gain,
        core::f32::consts::FRAC_1_SQRT_2,
        1.0,
        SampleRateHz(48_000),
    )
    .expect("legal design")
}

/// E6: an automation point starts a sixty-four sample linear ramp of the six words (D11).
#[test]
fn automation_starts_a_64_sample_word_ramp() {
    assert_eq!(
        PARAMETRIC_EQ_DESCRIPTOR.parameters[3].id.0, 4,
        "public stable ID is sparse identity"
    );
    let values = bell_values();
    let mut effect = ParametricEqFactory
        .prepare(request(&values, false))
        .expect("prepare");
    let start = expected_words(0.0);
    let target = expected_words(12.0);
    let step: [f32; 6] = core::array::from_fn(|index| {
        (target.to_array()[index] - start.to_array()[index]) * (1.0 / 64.0)
    });

    let (_, left, _) = snapshot(effect.as_ref());
    for index in 0..6 {
        assert_eq!(
            band_word(&left, 0, 2 + index),
            start.to_array()[index].to_bits(),
            "settled word {index}"
        );
    }

    let report = process_zeros(
        effect.as_mut(),
        0,
        1,
        &[point(3, ParameterChannel::Left, 0, 12.0)],
    );
    assert_eq!(report.invalid_spans, 0);
    let (_, left, _) = snapshot(effect.as_ref());
    assert_eq!(band_word(&left, 0, 14), 63, "one frame consumed");
    for (index, increment) in step.into_iter().enumerate() {
        assert_eq!(
            band_word(&left, 0, 2 + index),
            (start.to_array()[index] + increment).to_bits(),
            "first ramped word {index}"
        );
        assert_eq!(band_word(&left, 0, 8 + index), increment.to_bits());
    }
    assert_eq!(f32::from_bits(band_word(&left, 0, 16)), 12.0, "gain target");

    let report = process_zeros(effect.as_mut(), 1, 63, &[]);
    assert_eq!(report.invalid_spans, 0);
    let (_, left, _) = snapshot(effect.as_ref());
    assert_eq!(band_word(&left, 0, 14), 0, "ramp finished");
    for index in 0..6 {
        assert_eq!(
            band_word(&left, 0, 2 + index),
            target.to_array()[index].to_bits(),
            "snapped word {index}"
        );
        assert_eq!(band_word(&left, 0, 8 + index), 0.0_f32.to_bits());
    }
}

/// Every malformed span is counted once and none of them discards a valid point.
#[test]
fn malformed_automation_rejects_each_span_without_losing_valid_targets() {
    let values = bell_values();
    let mut effect = ParametricEqFactory
        .prepare(request(&values, false))
        .expect("prepare");
    let mut wrong_time = point(5, ParameterChannel::Left, 1, 0.5);
    wrong_time.end_sample = 1;
    let mut mismatched_point = point(2, ParameterChannel::Right, 0, 100.0);
    mismatched_point.end_value = 200.0;
    let automation = [
        point(3, ParameterChannel::Left, 0, 6.0),
        point(3, ParameterChannel::Left, 0, 8.0),
        point(4, ParameterChannel::Left, 0, 1.0),
        point(3, ParameterChannel::Right, 0, 7.0),
        point(0, ParameterChannel::Left, 0, 1.0),
        point(5, ParameterChannel::Both, 0, 0.5),
        wrong_time,
        mismatched_point,
    ];
    let report = process_zeros(effect.as_mut(), 0, 1, &automation);
    assert_eq!(report.invalid_spans, 6);
    let (_, left, right) = snapshot(effect.as_ref());
    assert_eq!(
        f32::from_bits(band_word(&left, 0, 16)),
        6.0,
        "first gain wins"
    );
    assert_eq!(f32::from_bits(band_word(&left, 0, 17)), 1.0, "Q accepted");
    assert_eq!(
        f32::from_bits(band_word(&right, 0, 16)),
        0.0,
        "right untouched"
    );
    assert_eq!(band_word(&right, 0, 14), 0, "right has no ramp in flight");
}

/// More spans than the block's capacity is one rejection per span, and no target moves.
#[test]
fn an_over_capacity_automation_block_is_rejected_whole() {
    let values = bell_values();
    let mut effect = ParametricEqFactory
        .prepare(request(&values, false))
        .expect("prepare");
    let spans: Vec<_> = (0..49)
        .map(|_| point(3, ParameterChannel::Left, 0, 6.0))
        .collect();
    let report = process_zeros(effect.as_mut(), 0, 1, &spans);
    assert_eq!(report.invalid_spans, 49);
    let (_, left, _) = snapshot(effect.as_ref());
    assert_eq!(band_word(&left, 0, 14), 0);
}

/// E8: the rendered block and the retained state do not depend on where a block is cut.
#[test]
fn automation_is_partition_invariant() {
    for partition in [
        vec![1_usize, 63, 64],
        vec![1, 7, 64, 56],
        vec![128],
        vec![64, 64],
    ] {
        let values = bell_values();
        let mut whole = ParametricEqFactory
            .prepare(request(&values, false))
            .expect("whole prepare");
        let mut split = ParametricEqFactory
            .prepare(request(&values, false))
            .expect("split prepare");
        let automation = [
            point(2, ParameterChannel::Left, 0, 3_000.0),
            point(3, ParameterChannel::Left, 0, -9.0),
        ];
        let mut whole_left: Vec<f32> = (0..192).map(|index| (index as f32).sin() * 0.5).collect();
        let mut whole_right: Vec<f32> = whole_left.iter().map(|value| -value).collect();
        let mut split_left = whole_left.clone();
        let mut split_right = whole_right.clone();

        whole.process(
            EffectProcessBlock::new(
                &mut whole_left[..128],
                &mut whole_right[..128],
                None,
                0,
                &automation,
                128,
            )
            .expect("whole block"),
        );
        whole.process(
            EffectProcessBlock::new(
                &mut whole_left[128..],
                &mut whole_right[128..],
                None,
                128,
                &[],
                128,
            )
            .expect("whole tail"),
        );

        let mut first = 0_usize;
        for (index, frames) in partition.iter().copied().enumerate() {
            let spans: &[_] = if index == 0 { &automation } else { &[] };
            split.process(
                EffectProcessBlock::new(
                    &mut split_left[first..first + frames],
                    &mut split_right[first..first + frames],
                    None,
                    first as u64,
                    spans,
                    128,
                )
                .expect("split block"),
            );
            first += frames;
        }
        split.process(
            EffectProcessBlock::new(
                &mut split_left[first..],
                &mut split_right[first..],
                None,
                first as u64,
                &[],
                128,
            )
            .expect("split tail"),
        );

        assert_eq!(
            whole_left.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            split_left.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            "partition {partition:?} left"
        );
        assert_eq!(
            whole_right.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            split_right.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            "partition {partition:?} right"
        );
        assert_eq!(
            snapshot(whole.as_ref()),
            snapshot(split.as_ref()),
            "partition {partition:?} state"
        );
    }
}

/// A restored mid-ramp payload continues the ramp bit for bit, because the increment is stored.
#[test]
fn state_restore_continues_active_ramp_bit_exactly() {
    let values = bell_values();
    let mut source = ParametricEqFactory
        .prepare(request(&values, false))
        .expect("source prepare");
    let automation = [point(3, ParameterChannel::Left, 0, -6.0)];
    let mut first_left = [0.25_f32; 17];
    let mut first_right = [0.125_f32; 17];
    source.process(
        EffectProcessBlock::new(&mut first_left, &mut first_right, None, 0, &automation, 128)
            .expect("first block"),
    );
    let (common, saved_left, saved_right) = snapshot(source.as_ref());
    let mut restored = ParametricEqFactory
        .prepare(request(&values, false))
        .expect("restore prepare");
    restored
        .restore_state_payload(
            1,
            StatePayloadInput::new(
                &common,
                &saved_left,
                &saved_right,
                restored.metadata().state_sizes,
            )
            .expect("state input"),
        )
        .expect("restore");

    let mut source_left = [0.5_f32; 64];
    let mut source_right = [-0.25_f32; 64];
    let mut restored_left = source_left;
    let mut restored_right = source_right;
    source.process(
        EffectProcessBlock::new(&mut source_left, &mut source_right, None, 17, &[], 128)
            .expect("source continuation"),
    );
    restored.process(
        EffectProcessBlock::new(&mut restored_left, &mut restored_right, None, 17, &[], 128)
            .expect("restored continuation"),
    );
    assert_eq!(
        source_left.map(f32::to_bits),
        restored_left.map(f32::to_bits)
    );
    assert_eq!(
        source_right.map(f32::to_bits),
        restored_right.map(f32::to_bits)
    );
    assert_eq!(snapshot(source.as_ref()), snapshot(restored.as_ref()));
}

/// A payload claiming an invalid prelaunch version is rejected, never silently migrated.
#[test]
fn an_invalid_version_payload_is_rejected() {
    let values = values();
    let mut effect = ParametricEqFactory
        .prepare(request(&values, false))
        .expect("prepare");
    let (common, left, right) = snapshot(effect.as_ref());
    assert_eq!(
        effect.restore_state_payload(
            0,
            StatePayloadInput::new(&common, &left, &right, effect.metadata().state_sizes)
                .expect("input"),
        ),
        Err(StatePayloadError {
            code: "effect.state.version"
        })
    );
}

/// A payload of the wrong length is rejected before a word is read.
///
/// Exactly, in both directions: a payload longer than the layout is as wrong as a short one,
/// because the surplus is either another layout's data or uninitialised memory.
#[test]
fn a_payload_of_the_wrong_length_is_rejected() {
    let values = values();
    let mut effect = ParametricEqFactory
        .prepare(request(&values, false))
        .expect("prepare");
    let (common, left, right) = snapshot(effect.as_ref());
    for (common_len, left_len, right_len) in [
        (COMMON_BYTES - 4, LANE_BYTES, LANE_BYTES),
        (COMMON_BYTES + 4, LANE_BYTES, LANE_BYTES),
        (COMMON_BYTES, LANE_BYTES - 4, LANE_BYTES),
        (COMMON_BYTES, LANE_BYTES, LANE_BYTES + 4),
    ] {
        let resize = |source: &[u8], length: usize| {
            let mut bytes = vec![0_u8; length];
            let take = source.len().min(length);
            bytes[..take].copy_from_slice(&source[..take]);
            bytes
        };
        assert_eq!(
            effect.restore_state_payload(
                1,
                StatePayloadInput {
                    common: &resize(&common, common_len),
                    left: &resize(&left, left_len),
                    right: &resize(&right, right_len),
                },
            ),
            Err(StatePayloadError {
                code: "effect.state.length"
            }),
            "sections {common_len}/{left_len}/{right_len}"
        );
    }
}

/// The header makes a payload self-describing, so a stale one rejects itself.
///
/// The out-of-band `state_layout_version` argument is the caller's claim; word 0 is the payload's
/// own. Both are checked, and the version is checked before the word count so that a payload from
/// an older layout reports the version it actually is rather than the length that version implies.
#[test]
fn a_payload_with_a_stale_header_is_rejected_on_its_own_evidence() {
    let values = values();
    let mut effect = ParametricEqFactory
        .prepare(request(&values, false))
        .expect("prepare");
    let (common, left, right) = snapshot(effect.as_ref());
    assert_eq!(word(&common, 0), 1, "the layout version is stamped");
    assert_eq!(
        word(&common, 1),
        (SECTIONS * WORDS_PER_BAND * 2) as u32,
        "the data word count is stamped"
    );

    let mut stale = common;
    stale[0] = 0;
    assert_eq!(
        effect.restore_state_payload(
            1,
            StatePayloadInput::new(&stale, &left, &right, effect.metadata().state_sizes)
                .expect("input"),
        ),
        Err(StatePayloadError {
            code: "effect.state.version"
        }),
        "an invalid payload version cannot pass even when the caller claims the current version"
    );

    let mut miscounted = common;
    miscounted[4] = 0xff;
    assert_eq!(
        effect.restore_state_payload(
            1,
            StatePayloadInput::new(&miscounted, &left, &right, effect.metadata().state_sizes)
                .expect("input"),
        ),
        Err(StatePayloadError {
            code: "effect.state.length"
        }),
        "a payload whose header disagrees with its own body is not a payload"
    );
}

/// A payload whose words are out of domain or inconsistent with its parameters is rejected whole.
#[test]
fn a_malformed_payload_is_rejected_without_touching_either_channel() {
    let values = bell_values();
    let mut effect = ParametricEqFactory
        .prepare(request(&values, false))
        .expect("prepare");
    process_zeros(effect.as_mut(), 0, 8, &[]);
    let before = snapshot(effect.as_ref());
    for (word, bits) in [
        (0_usize, f32::NAN.to_bits()),
        (2, f32::NAN.to_bits()),
        (8, f32::INFINITY.to_bits()),
        (14, 65_u32),
        (15, 5.0_f32.to_bits()),
        (17, 0.0_f32.to_bits()),
    ] {
        let (common, mut left, right) = before;
        left[word * 4..word * 4 + 4].copy_from_slice(&bits.to_le_bytes());
        assert_eq!(
            effect.restore_state_payload(
                1,
                StatePayloadInput::new(&common, &left, &right, effect.metadata().state_sizes)
                    .expect("input"),
            ),
            Err(StatePayloadError {
                code: "effect.state.payload"
            }),
            "word {word}"
        );
        assert_eq!(
            snapshot(effect.as_ref()),
            before,
            "word {word} left a trace"
        );
    }
    // A settled band whose stored words disagree with its stored parameters is a forgery.
    let (common, mut left, right) = before;
    let poisoned = f32::from_bits(band_word(&left, 0, 2)) + 1.0e-3;
    left[8..12].copy_from_slice(&poisoned.to_bits().to_le_bytes());
    assert_eq!(
        effect.restore_state_payload(
            1,
            StatePayloadInput::new(&common, &left, &right, effect.metadata().state_sizes)
                .expect("input"),
        ),
        Err(StatePayloadError {
            code: "effect.state.payload"
        })
    );
}

/// A short or over-long snapshot buffer is refused before a byte is written.
#[test]
fn snapshot_rejects_bad_output_without_touching_either_lane() {
    let values = values();
    let effect = ParametricEqFactory
        .prepare(request(&values, false))
        .expect("prepare");
    let mut common = [0xC3_u8; COMMON_BYTES];
    let mut left = [0xA5_u8; LANE_BYTES];
    let mut right = [0x5A_u8; LANE_BYTES - 1];
    assert_eq!(
        effect.snapshot_state_payload(StatePayloadOutput {
            common: &mut common,
            left: &mut left,
            right: &mut right,
        }),
        Err(StatePayloadError {
            code: "effect.state.length"
        })
    );
    assert_eq!(common, [0xC3; COMMON_BYTES]);
    assert_eq!(left, [0xA5; LANE_BYTES]);
    assert_eq!(right, [0x5A; LANE_BYTES - 1]);
}

/// E10: a disabled band and a 0 dB band both return the dry signal, and only one of them stays cold.
///
/// The identity is exact and needs no mask: `(c1, a2, a3) = 0` and `(m0, m1, m2) = (1, 0, 0)` make
/// `y = fma(0, v2, fma(0, v1, 1 * x))`, which is `x` bit for bit. A 0 dB bell is *not* special
/// cased — the mapping gives `m1 = k * (A^2 - 1) = 0` at `A = 1` — so it is dry at the output while
/// its integrators stay charged, which is what makes a gain sweep through zero continuous.
#[test]
fn disabled_and_zero_db_sections_return_dry_bits_with_zero_state_growth() {
    let samples = [0.25_f32, -0.5, 1.0, -0.0, 0.0, 1.0e-30, -0.75];
    for (values, warm) in [(values(), false), (bell_values(), true)] {
        let mut effect = ParametricEqFactory
            .prepare(request(&values, false))
            .expect("prepare");
        let mut left = samples;
        let mut right = samples;
        effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("block"),
        );
        for (index, sample) in samples.into_iter().enumerate() {
            let expected = if sample == 0.0 { 0.0_f32 } else { sample };
            assert_eq!(
                left[index].to_bits(),
                expected.to_bits(),
                "warm={warm} sample {sample}"
            );
        }
        let (_, state, _) = snapshot(effect.as_ref());
        let charged = (0..SECTIONS)
            .flat_map(|band| (0..2).map(move |word| band_word(&state, band, word)))
            .any(|bits| bits != 0);
        assert_eq!(charged, warm, "state charge");
    }
}

/// Bypass copies the dry signal, including NaN, and never touches the state.
#[test]
fn bypass_copies_dry_bits_and_leaves_the_state_alone() {
    let values = bell_values();
    let mut effect = ParametricEqFactory
        .prepare(request(&values, true))
        .expect("bypass prepare");
    let before = snapshot(effect.as_ref());
    let mut left = [0.0_f32, f32::NAN, 0.25];
    let mut right = [-0.0_f32, 0.5, 0.0];
    let report = effect
        .process(EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("block"));
    assert!(left[1].is_nan(), "bypass is a copy, not a sanitiser");
    assert_eq!(right[0].to_bits(), (-0.0_f32).to_bits());
    assert_eq!(left[2].to_bits(), 0.25_f32.to_bits());
    assert_eq!(report.sanitized_main_samples, 0);
    assert_eq!(report.nonfinite_left_blocks, 0);
    assert_eq!(snapshot(effect.as_ref()), before);
}

/// The two resets mean two different things and both are exact.
#[test]
fn resets_restore_defaults_or_only_clear_history() {
    let values = single_section_values(EqBandKind::HighPass, 1_000.0, 0.0, 1.0, 1.0);
    let mut effect = ParametricEqFactory
        .prepare(request(&values, false))
        .expect("prepare");
    let fresh = snapshot(effect.as_ref());
    process_zeros(
        effect.as_mut(),
        0,
        8,
        &[point(2, ParameterChannel::Left, 0, 5_000.0)],
    );
    let mut left = [0.5_f32; 8];
    let mut right = [0.5_f32; 8];
    effect
        .process(EffectProcessBlock::new(&mut left, &mut right, None, 8, &[], 128).expect("block"));
    let (_, moved, _) = snapshot(effect.as_ref());
    assert_ne!(band_word(&moved, 0, 0), 0, "the integrators charged");
    assert_ne!(band_word(&moved, 0, 14), 0, "a ramp is in flight");

    effect.reset(ResetKind::DiscontinuityKeepParameters);
    let (_, kept, _) = snapshot(effect.as_ref());
    assert_eq!(band_word(&kept, 0, 0), 0, "history cleared");
    assert_eq!(band_word(&kept, 0, 14), 0, "the ramp snapped");
    assert_eq!(
        f32::from_bits(band_word(&kept, 0, 15)),
        5_000.0,
        "the automated parameter survived"
    );
    let settled = design_svf(
        EqBandKind::HighPass,
        5_000.0,
        0.0,
        1.0,
        1.0,
        SampleRateHz(48_000),
    )
    .expect("legal design");
    for index in 0..6 {
        assert_eq!(
            band_word(&kept, 0, 2 + index),
            settled.to_array()[index].to_bits()
        );
    }

    effect.reset(ResetKind::FullToDefaults);
    assert_eq!(snapshot(effect.as_ref()), fresh);
}

/// A band whose enable or family is not an admitted value is refused at prepare.
#[test]
fn prepare_refuses_an_unknown_family() {
    let mut values = values();
    set_initial(&mut values, 1, ParameterChannel::Left, 7.0);
    assert!(
        ParametricEqFactory
            .prepare(request(&values, false))
            .is_err()
    );
}

/// `-0.0` is a way of writing zero, and is normalised on the way in (83c decision 3).
///
/// Five of the eight effect crates rejected it outright before wave 2. The lenient rule wins: a
/// control message that writes zero the other way is not an error, while a `-0.0` reaching a
/// coefficient design or a payload is a value nothing downstream expects.
#[test]
fn a_negative_zero_automation_value_is_accepted_as_zero() {
    let mut values = bell_values();
    set_initial(&mut values, 3, ParameterChannel::Left, 6.0);
    let mut effect = ParametricEqFactory
        .prepare(request(&values, false))
        .expect("prepare");
    let report = process_zeros(
        effect.as_mut(),
        0,
        1,
        &[point(3, ParameterChannel::Left, 0, -0.0)],
    );
    assert_eq!(report.invalid_spans, 0);
    let (_, left, _) = snapshot(effect.as_ref());
    assert_eq!(band_word(&left, 0, 16), 0.0_f32.to_bits(), "target is +0.0");
    assert_eq!(band_word(&left, 0, 14), 64 - 1, "a ramp started");

    // And a payload that carries `-0.0` restores as `+0.0` rather than being rejected.
    let (common, mut stored, right) = snapshot(effect.as_ref());
    stored[16 * 4..16 * 4 + 4].copy_from_slice(&(-0.0_f32).to_bits().to_le_bytes());
    effect
        .restore_state_payload(
            1,
            StatePayloadInput::new(&common, &stored, &right, effect.metadata().state_sizes)
                .expect("input"),
        )
        .expect("a negative zero parameter is a legal payload");
    let (_, left, _) = snapshot(effect.as_ref());
    assert_eq!(band_word(&left, 0, 16), 0.0_f32.to_bits());
}
