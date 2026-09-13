#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: the independent response oracle intentionally uses the
// platform's f64 methods; production evaluation uses `math`.

//! Acceptance gates for the native effect-owned response query.

mod support;

use bench_support::alloc as bench_alloc;
use dsp_reference::ReferenceSvfStateSpace;
use effect_contract::{EffectProcessBlock, NativeEffectFactory, ParameterChannel};
use parametric_eq::{
    EQ_SECTION_COUNT, EqBandKind, EqResponseConfiguration, EqResponseError, EqResponseMode,
    EqResponseOutput, EqResponseRequest, ParametricEqFactory, design_svf, query_response_into,
};
use support::{LAUNCH_RATES, request_at_rate, set_initial, single_section_values, values};

const FLOOR_DB: f64 = -120.0;
const TOLERANCE_DB: f64 = 0.005;

fn prepared(
    values: &[effect_contract::InitialParameterValue],
    bypass: bool,
    rate: u32,
) -> EqResponseConfiguration {
    EqResponseConfiguration::prepare(request_at_rate(values, bypass, rate))
        .expect("valid response configuration")
}

fn realized(words: parametric_eq::EqSvfWords) -> ReferenceSvfStateSpace {
    ReferenceSvfStateSpace::new(
        f64::from(words.c1),
        f64::from(words.a2),
        f64::from(words.a3),
        [
            f64::from(words.m0),
            f64::from(words.m1),
            f64::from(words.m2),
        ],
    )
}

fn oracle_db(
    kind: EqBandKind,
    rate: u32,
    frequency: f32,
    gain: f32,
    q: f32,
    slope: f32,
    probe: f32,
) -> f64 {
    let words = design_svf(kind, frequency, gain, q, slope, engine::SampleRateHz(rate))
        .expect("test configuration must design");
    realized(words)
        .magnitude_db(f64::from(rate), f64::from(probe))
        .expect("probe must be inside Nyquist")
}

fn configured_four_sections(rate: u32) -> Vec<effect_contract::InitialParameterValue> {
    let mut configured = values();
    let rows = [
        (EqBandKind::Bell, 120.0, 6.0, 0.7, 1.0),
        (EqBandKind::LowShelf, 500.0, -4.0, 1.0, 0.5),
        (EqBandKind::HighPass, 1_800.0, 0.0, 0.9, 1.0),
        (EqBandKind::Notch, 8_000.0, 0.0, 2.0, 1.0),
    ];
    assert!(
        rows.iter()
            .all(|(_, frequency, _, _, _)| { *frequency < rate as f32 * 0.5 })
    );
    for (section, (kind, frequency, gain, q, slope)) in rows.into_iter().enumerate() {
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            set_initial(&mut configured, section * 6, channel, 1.0);
            set_initial(
                &mut configured,
                section * 6 + 1,
                channel,
                kind as u32 as f32,
            );
            set_initial(&mut configured, section * 6 + 2, channel, frequency);
            set_initial(&mut configured, section * 6 + 3, channel, gain);
            set_initial(&mut configured, section * 6 + 4, channel, q);
            set_initial(&mut configured, section * 6 + 5, channel, slope);
        }
    }
    configured
}

#[test]
fn rounded_words_match_independent_oracle_for_all_families_and_launch_rates() {
    let kinds = [
        EqBandKind::Bell,
        EqBandKind::LowShelf,
        EqBandKind::HighShelf,
        EqBandKind::LowPass,
        EqBandKind::HighPass,
        EqBandKind::Notch,
    ];
    let mut worst = 0.0_f64;
    for rate in LAUNCH_RATES {
        let frequencies = [0.0, 37.0, 1_234.0, rate as f32 * 0.5];
        for kind in kinds {
            let (gain, q, slope) = match kind {
                EqBandKind::LowPass | EqBandKind::HighPass | EqBandKind::Notch => {
                    (0.0, 0.70710677, 1.0)
                }
                EqBandKind::LowShelf | EqBandKind::HighShelf => (6.0, 1.0, 0.5),
                EqBandKind::Bell => (6.0, 0.70710677, 1.0),
            };
            let configured = single_section_values(kind, 1_000.0, gain, q, slope);
            let configuration = prepared(&configured, false, rate);
            let mut total_left = [f32::NAN; 4];
            let mut total_right = [f32::NAN; 4];
            let mut sections_left = [f32::NAN; EQ_SECTION_COUNT * 4];
            let mut sections_right = [f32::NAN; EQ_SECTION_COUNT * 4];
            let summary = query_response_into(
                EqResponseRequest {
                    configuration_id: 9_007_199_254_740_993,
                    configuration: &configuration,
                    frequencies_hz: &frequencies,
                    maximum_points: frequencies.len(),
                },
                EqResponseOutput {
                    total_left_db: &mut total_left,
                    total_right_db: &mut total_right,
                    sections_left_db: Some(&mut sections_left),
                    sections_right_db: Some(&mut sections_right),
                },
            )
            .expect("valid response query");
            assert_eq!(summary.configuration_id, 9_007_199_254_740_993);
            assert_eq!(summary.mode, EqResponseMode::RequestedConfiguration);
            assert_eq!(summary.sample_rate_hz, rate);
            assert_eq!(summary.points, frequencies.len());
            assert!(summary.enabled_left[0]);
            assert!(summary.enabled_left[1..].iter().all(|enabled| !enabled));
            for (point, frequency) in frequencies.into_iter().enumerate() {
                let expected = oracle_db(kind, rate, 1_000.0, gain, q, slope, frequency);
                let expected = if expected.is_finite() {
                    expected.max(FLOOR_DB)
                } else {
                    FLOOR_DB
                };
                let error = (f64::from(total_left[point]) - expected).abs();
                worst = worst.max(error);
                assert!(
                    error <= TOLERANCE_DB,
                    "{kind:?} Fs={rate} f={frequency}: got={} expected={expected} error={error}",
                    total_left[point]
                );
                assert!((f64::from(sections_left[point]) - expected).abs() <= TOLERANCE_DB);
                for section in 1..EQ_SECTION_COUNT {
                    assert_eq!(sections_left[section * frequencies.len() + point], 0.0);
                }
            }
        }
    }
    eprintln!("issue-764 oracle all-families worst_error_db={worst:.6e}");
}

#[test]
fn asymmetric_cascade_sections_are_independent_and_bypass_is_total_identity() {
    let rate = 48_000;
    let mut configured = configured_four_sections(rate);
    set_initial(&mut configured, 3, ParameterChannel::Right, -6.0);
    let configuration = prepared(&configured, false, rate);
    let frequencies = [0.0, 120.0, 1_000.0, 8_000.0, rate as f32 * 0.5];
    let mut total_left = [f32::NAN; 5];
    let mut total_right = [f32::NAN; 5];
    let mut sections_left = [f32::NAN; EQ_SECTION_COUNT * 5];
    let mut sections_right = [f32::NAN; EQ_SECTION_COUNT * 5];
    let summary = query_response_into(
        EqResponseRequest {
            configuration_id: u64::MAX,
            configuration: &configuration,
            frequencies_hz: &frequencies,
            maximum_points: 5,
        },
        EqResponseOutput {
            total_left_db: &mut total_left,
            total_right_db: &mut total_right,
            sections_left_db: Some(&mut sections_left),
            sections_right_db: Some(&mut sections_right),
        },
    )
    .expect("valid cascade query");
    assert!(summary.enabled_left.into_iter().all(|enabled| enabled));
    assert!(summary.enabled_right.into_iter().all(|enabled| enabled));
    for point in 0..frequencies.len() {
        let mut product = dsp_reference::Complex64 { re: 1.0, im: 0.0 };
        for section in 0..EQ_SECTION_COUNT {
            let value = sections_left[section * frequencies.len() + point];
            assert!(value.is_finite());
            let (kind, frequency, gain, q, slope) = [
                (EqBandKind::Bell, 120.0, 6.0, 0.7, 1.0),
                (EqBandKind::LowShelf, 500.0, -4.0, 1.0, 0.5),
                (EqBandKind::HighPass, 1_800.0, 0.0, 0.9, 1.0),
                (EqBandKind::Notch, 8_000.0, 0.0, 2.0, 1.0),
            ][section];
            let words = design_svf(kind, frequency, gain, q, slope, engine::SampleRateHz(rate))
                .expect("cascade design");
            let response = realized(words)
                .response(f64::from(rate), f64::from(frequencies[point]))
                .expect("cascade probe");
            let expected_db = (20.0 * response.re.hypot(response.im).log10()).max(FLOOR_DB);
            assert!((f64::from(value) - expected_db).abs() <= TOLERANCE_DB);
            product = dsp_reference::Complex64 {
                re: product.re * response.re - product.im * response.im,
                im: product.re * response.im + product.im * response.re,
            };
        }
        let expected_total = (20.0 * product.re.hypot(product.im).log10()).max(FLOOR_DB);
        assert!((f64::from(total_left[point]) - expected_total).abs() <= TOLERANCE_DB);

        let right_rows = [
            (EqBandKind::Bell, 120.0, -6.0, 0.7, 1.0),
            (EqBandKind::LowShelf, 500.0, -4.0, 1.0, 0.5),
            (EqBandKind::HighPass, 1_800.0, 0.0, 0.9, 1.0),
            (EqBandKind::Notch, 8_000.0, 0.0, 2.0, 1.0),
        ];
        let mut right_product = dsp_reference::Complex64 { re: 1.0, im: 0.0 };
        for (kind, frequency, gain, q, slope) in right_rows {
            let words = design_svf(kind, frequency, gain, q, slope, engine::SampleRateHz(rate))
                .expect("right cascade design");
            let response = realized(words)
                .response(f64::from(rate), f64::from(frequencies[point]))
                .expect("right cascade probe");
            right_product = dsp_reference::Complex64 {
                re: right_product.re * response.re - right_product.im * response.im,
                im: right_product.re * response.im + right_product.im * response.re,
            };
        }
        let expected_right =
            (20.0 * right_product.re.hypot(right_product.im).log10()).max(FLOOR_DB);
        assert!((f64::from(total_right[point]) - expected_right).abs() <= TOLERANCE_DB);
    }

    let mut bypass_total_left = [f32::NAN; 5];
    let mut bypass_total_right = [f32::NAN; 5];
    let mut bypass_sections_left = [f32::NAN; EQ_SECTION_COUNT * 5];
    let bypass_configuration = prepared(&configured, true, rate);
    let bypass_summary = query_response_into(
        EqResponseRequest {
            configuration_id: 42,
            configuration: &bypass_configuration,
            frequencies_hz: &frequencies,
            maximum_points: frequencies.len(),
        },
        EqResponseOutput {
            total_left_db: &mut bypass_total_left,
            total_right_db: &mut bypass_total_right,
            sections_left_db: Some(&mut bypass_sections_left),
            sections_right_db: None,
        },
    )
    .expect("valid bypass query");
    assert!(bypass_total_left.iter().all(|value| value.to_bits() == 0));
    assert!(bypass_total_right.iter().all(|value| value.to_bits() == 0));
    assert!(bypass_sections_left.iter().any(|value| *value != 0.0));
    assert!(bypass_summary.bypass);

    let mut total_only_left = [f32::NAN; 5];
    let mut total_only_right = [f32::NAN; 5];
    let total_only = query_response_into(
        EqResponseRequest {
            configuration_id: 43,
            configuration: &configuration,
            frequencies_hz: &frequencies,
            maximum_points: frequencies.len(),
        },
        EqResponseOutput {
            total_left_db: &mut total_only_left,
            total_right_db: &mut total_only_right,
            sections_left_db: None,
            sections_right_db: None,
        },
    )
    .expect("total-only response query");
    assert_eq!(total_only.points, frequencies.len());
    assert_eq!(total_only_left, total_left);
    assert_eq!(total_only_right, total_right);

    let mut right_only_sections = [f32::NAN; EQ_SECTION_COUNT * 5];
    let mut right_only_left = [f32::NAN; 5];
    let mut right_only_right = [f32::NAN; 5];
    query_response_into(
        EqResponseRequest {
            configuration_id: 44,
            configuration: &configuration,
            frequencies_hz: &frequencies,
            maximum_points: frequencies.len(),
        },
        EqResponseOutput {
            total_left_db: &mut right_only_left,
            total_right_db: &mut right_only_right,
            sections_left_db: None,
            sections_right_db: Some(&mut right_only_sections),
        },
    )
    .expect("right-only section query");
    assert_eq!(right_only_left, total_left);
    assert_eq!(right_only_right, total_right);
}

#[test]
fn malformed_requests_preserve_output_sentinels_and_opaque_ids() {
    let configured = values();
    let configuration = prepared(&configured, false, 48_000);
    let valid = [100.0_f32, 1_000.0];
    let invalid_grids: [&[f32]; 7] = [
        &[],
        &[f32::NAN],
        &[f32::INFINITY],
        &[-1.0],
        &[24_001.0],
        &[100.0, 100.0],
        &[1_000.0, 100.0],
    ];
    for grid in invalid_grids {
        let mut left = [81.0_f32; 2];
        let mut right = [82.0_f32; 2];
        let result = query_response_into(
            EqResponseRequest {
                configuration_id: u64::MAX,
                configuration: &configuration,
                frequencies_hz: grid,
                maximum_points: 2,
            },
            EqResponseOutput {
                total_left_db: &mut left[..grid.len()],
                total_right_db: &mut right[..grid.len()],
                sections_left_db: None,
                sections_right_db: None,
            },
        );
        assert_eq!(result, Err(EqResponseError::InvalidFrequencyGrid));
        assert_eq!(left, [81.0_f32; 2]);
        assert_eq!(right, [82.0_f32; 2]);
    }

    let mut left = [97.0_f32; 2];
    let mut right = [98.0_f32; 2];
    let result = query_response_into(
        EqResponseRequest {
            configuration_id: 0,
            configuration: &configuration,
            frequencies_hz: &valid,
            maximum_points: 1,
        },
        EqResponseOutput {
            total_left_db: &mut left,
            total_right_db: &mut right,
            sections_left_db: None,
            sections_right_db: None,
        },
    );
    assert_eq!(result, Err(EqResponseError::Capacity));
    assert_eq!(left, [97.0_f32; 2]);
    assert_eq!(right, [98.0_f32; 2]);

    let mut left = [113.0_f32; 2];
    let mut right = [114.0_f32; 2];
    let result = query_response_into(
        EqResponseRequest {
            configuration_id: 0,
            configuration: &configuration,
            frequencies_hz: &valid,
            maximum_points: 2,
        },
        EqResponseOutput {
            total_left_db: &mut left[..1],
            total_right_db: &mut right,
            sections_left_db: None,
            sections_right_db: None,
        },
    );
    assert_eq!(result, Err(EqResponseError::OutputShape));
    assert_eq!(left, [113.0_f32; 2]);
    assert_eq!(right, [114.0_f32; 2]);

    let mut bad_values = configured.clone();
    support::set_initial(&mut bad_values, 2, ParameterChannel::Left, f32::NAN);
    let result = EqResponseConfiguration::prepare(request_at_rate(&bad_values, false, 176_400));
    assert!(matches!(result, Err(EqResponseError::Configuration(_))));

    let mut left = [f32::NAN; 2];
    let mut right = [f32::NAN; 2];
    let summary = query_response_into(
        EqResponseRequest {
            configuration_id: u64::MAX,
            configuration: &configuration,
            frequencies_hz: &valid,
            maximum_points: 2,
        },
        EqResponseOutput {
            total_left_db: &mut left,
            total_right_db: &mut right,
            sections_left_db: None,
            sections_right_db: None,
        },
    )
    .expect("valid opaque id query");
    assert_eq!(summary.configuration_id, u64::MAX);
}

#[test]
fn preparation_validates_full_request_and_owns_source_words() {
    let mut source = single_section_values(EqBandKind::Bell, 1_000.0, 6.0, 0.7, 1.0);
    let configuration = prepared(&source, false, 48_000);
    let frequencies = [0.0_f32, 1_000.0, 12_000.0];
    let mut before_left = [f32::NAN; 3];
    let mut before_right = [f32::NAN; 3];
    query_response_into(
        EqResponseRequest {
            configuration_id: 1,
            configuration: &configuration,
            frequencies_hz: &frequencies,
            maximum_points: frequencies.len(),
        },
        EqResponseOutput {
            total_left_db: &mut before_left,
            total_right_db: &mut before_right,
            sections_left_db: None,
            sections_right_db: None,
        },
    )
    .expect("source-owned baseline query");
    set_initial(&mut source, 3, ParameterChannel::Left, -24.0);
    set_initial(&mut source, 3, ParameterChannel::Right, 24.0);
    let mut after_left = [f32::NAN; 3];
    let mut after_right = [f32::NAN; 3];
    query_response_into(
        EqResponseRequest {
            configuration_id: 2,
            configuration: &configuration,
            frequencies_hz: &frequencies,
            maximum_points: frequencies.len(),
        },
        EqResponseOutput {
            total_left_db: &mut after_left,
            total_right_db: &mut after_right,
            sections_left_db: None,
            sections_right_db: None,
        },
    )
    .expect("source-owned query after mutation");
    assert_eq!(before_left, after_left);
    assert_eq!(before_right, after_right);

    let short = &source[..source.len() - 1];
    assert!(matches!(
        EqResponseConfiguration::prepare(request_at_rate(short, false, 48_000)),
        Err(EqResponseError::Configuration(_))
    ));

    let mut reordered = source.clone();
    reordered.swap(0, 1);
    assert!(matches!(
        EqResponseConfiguration::prepare(request_at_rate(&reordered, false, 48_000)),
        Err(EqResponseError::Configuration(_))
    ));

    let mut wrong_channel = source.clone();
    wrong_channel[0].channel = ParameterChannel::Both;
    assert!(matches!(
        EqResponseConfiguration::prepare(request_at_rate(&wrong_channel, false, 48_000)),
        Err(EqResponseError::Configuration(_))
    ));

    let mut zero_capacity = request_at_rate(&source, false, 48_000);
    zero_capacity.limits.maximum_total_state_bytes = 0;
    assert!(matches!(
        EqResponseConfiguration::prepare(zero_capacity),
        Err(EqResponseError::Configuration(_))
    ));

    let mut disabled_nan = values();
    set_initial(&mut disabled_nan, 2, ParameterChannel::Left, f32::NAN);
    assert!(matches!(
        EqResponseConfiguration::prepare(request_at_rate(&disabled_nan, false, 48_000)),
        Err(EqResponseError::Configuration(_))
    ));

    for rate in [44_100, 48_000, 88_200, 96_000] {
        assert!(EqResponseConfiguration::prepare(request_at_rate(&source, false, rate)).is_ok());
    }
    assert!(matches!(
        EqResponseConfiguration::prepare(request_at_rate(&source, false, 176_400)),
        Err(EqResponseError::Configuration(_))
    ));
}

#[test]
fn deep_cut_plus_gain_composes_before_public_flooring() {
    let rate = 48_000;
    let mut configured = values();
    for channel in [ParameterChannel::Left, ParameterChannel::Right] {
        set_initial(&mut configured, 0, channel, 1.0);
        set_initial(&mut configured, 1, channel, EqBandKind::Notch as u32 as f32);
        set_initial(&mut configured, 2, channel, 1_000.0);
        set_initial(&mut configured, 4, channel, 18.0);
        set_initial(&mut configured, 5, channel, 1.0);
        set_initial(&mut configured, 6, channel, 1.0);
        set_initial(&mut configured, 7, channel, EqBandKind::Bell as u32 as f32);
        set_initial(&mut configured, 8, channel, 1_000.0);
        set_initial(&mut configured, 9, channel, 24.0);
        set_initial(&mut configured, 10, channel, 0.7);
        set_initial(&mut configured, 11, channel, 1.0);
    }
    let notch_words = design_svf(
        EqBandKind::Notch,
        1_000.0,
        0.0,
        18.0,
        1.0,
        engine::SampleRateHz(rate),
    )
    .expect("notch design");
    let gain_words = design_svf(
        EqBandKind::Bell,
        1_000.0,
        24.0,
        0.7,
        1.0,
        engine::SampleRateHz(rate),
    )
    .expect("gain design");
    let notch = realized(notch_words);
    let gain = realized(gain_words);
    let mut selected = None;
    for step in -10_000..=10_000 {
        let frequency = 1_000.0 + step as f32 * 0.001;
        if !(0.0..=rate as f32 * 0.5).contains(&frequency) {
            continue;
        }
        let notch_response = notch
            .response(f64::from(rate), f64::from(frequency))
            .expect("notch probe");
        let gain_response = gain
            .response(f64::from(rate), f64::from(frequency))
            .expect("gain probe");
        let notch_db = 20.0 * notch_response.re.hypot(notch_response.im).log10();
        let gain_db = 20.0 * gain_response.re.hypot(gain_response.im).log10();
        let product = dsp_reference::Complex64 {
            re: notch_response.re * gain_response.re - notch_response.im * gain_response.im,
            im: notch_response.re * gain_response.im + notch_response.im * gain_response.re,
        };
        let product_db = 20.0 * product.re.hypot(product.im).log10();
        let floored_sum = notch_db.max(FLOOR_DB) + gain_db.max(FLOOR_DB);
        if notch_db < FLOOR_DB && product_db > FLOOR_DB && (product_db - floored_sum).abs() > 1.0 {
            selected = Some((frequency, product_db, floored_sum));
            break;
        }
    }
    let (frequency, expected_total, incorrectly_summed) =
        selected.expect("the notch/gain search must find a deep but recoverable null");
    let configuration = prepared(&configured, false, rate);
    let frequencies = [frequency];
    let mut total_left = [f32::NAN; 1];
    let mut total_right = [f32::NAN; 1];
    let mut sections_left = [f32::NAN; EQ_SECTION_COUNT];
    let summary = query_response_into(
        EqResponseRequest {
            configuration_id: 55,
            configuration: &configuration,
            frequencies_hz: &frequencies,
            maximum_points: 1,
        },
        EqResponseOutput {
            total_left_db: &mut total_left,
            total_right_db: &mut total_right,
            sections_left_db: Some(&mut sections_left),
            sections_right_db: None,
        },
    )
    .expect("deep null query");
    assert_eq!(summary.points, 1);
    assert_eq!(sections_left[0], -120.0);
    assert!(
        (f64::from(sections_left[1])
            - 20.0
                * gain
                    .response(f64::from(rate), f64::from(frequency))
                    .unwrap()
                    .re
                    .hypot(
                        gain.response(f64::from(rate), f64::from(frequency))
                            .unwrap()
                            .im,
                    )
                    .log10())
        .abs()
            <= TOLERANCE_DB
    );
    assert!((f64::from(total_left[0]) - expected_total.max(FLOOR_DB)).abs() <= TOLERANCE_DB);
    assert!((f64::from(total_left[0]) - incorrectly_summed).abs() > 1.0);
}

#[test]
fn settled_pcm_impulse_matches_query_at_all_launch_rates() {
    let frequencies = [120.0_f32, 1_000.0, 2_400.0];
    for rate in LAUNCH_RATES {
        let mut configured = configured_four_sections(rate);
        set_initial(&mut configured, 3, ParameterChannel::Right, -6.0);
        let configuration = prepared(&configured, false, rate);
        let mut queried_left = [f32::NAN; 3];
        let mut queried_right = [f32::NAN; 3];
        query_response_into(
            EqResponseRequest {
                configuration_id: u64::from(rate),
                configuration: &configuration,
                frequencies_hz: &frequencies,
                maximum_points: frequencies.len(),
            },
            EqResponseOutput {
                total_left_db: &mut queried_left,
                total_right_db: &mut queried_right,
                sections_left_db: None,
                sections_right_db: None,
            },
        )
        .expect("PCM comparison query");

        let mut effect = ParametricEqFactory
            .prepare(request_at_rate(&configured, false, rate))
            .expect("PCM comparison effect");
        let mut left = vec![0.0_f32; rate as usize];
        let mut right = vec![0.0_f32; rate as usize];
        left[0] = 1.0;
        right[0] = 1.0;
        for first in (0..left.len()).step_by(128) {
            let end = (first + 128).min(left.len());
            let report = effect.process(
                EffectProcessBlock::new(
                    &mut left[first..end],
                    &mut right[first..end],
                    None,
                    first as u64,
                    &[],
                    128,
                )
                .expect("PCM comparison block"),
            );
            assert_eq!(report.nonfinite_left_blocks, 0);
            assert_eq!(report.nonfinite_right_blocks, 0);
        }
        for (index, frequency) in frequencies.into_iter().enumerate() {
            let measured_left = support::impulse_dft_db(&left, rate, f64::from(frequency));
            let measured_right = support::impulse_dft_db(&right, rate, f64::from(frequency));
            assert!(measured_left.is_finite() && measured_right.is_finite());
            assert!(
                (measured_left - f64::from(queried_left[index])).abs() <= 0.05,
                "Fs={rate} f={frequency} left measured={measured_left} query={}",
                queried_left[index]
            );
            assert!(
                (measured_right - f64::from(queried_right[index])).abs() <= 0.05,
                "Fs={rate} f={frequency} right measured={measured_right} query={}",
                queried_right[index]
            );
        }
    }
}

#[test]
fn query_does_not_change_a_prepared_effect_state() {
    let mut configured = single_section_values(EqBandKind::Bell, 1_000.0, 6.0, 0.7, 1.0);
    let mut effect = ParametricEqFactory
        .prepare(request_at_rate(&configured, false, 48_000))
        .expect("prepared effect");
    let automation = [support::point(3, ParameterChannel::Left, 0, -6.0)];
    let mut left = [0.0_f32; 128];
    let mut right = [0.0_f32; 128];
    effect.process(
        EffectProcessBlock::new(&mut left, &mut right, None, 0, &automation, 128)
            .expect("automation block"),
    );
    let before = support::snapshot(effect.as_ref());
    set_initial(&mut configured, 3, ParameterChannel::Left, -6.0);
    let configuration = prepared(&configured, false, 48_000);
    let frequencies = [0.0, 1_000.0, 12_000.0, 24_000.0];
    let mut total_left = [f32::NAN; 4];
    let mut total_right = [f32::NAN; 4];
    query_response_into(
        EqResponseRequest {
            configuration_id: 3,
            configuration: &configuration,
            frequencies_hz: &frequencies,
            maximum_points: 4,
        },
        EqResponseOutput {
            total_left_db: &mut total_left,
            total_right_db: &mut total_right,
            sections_left_db: None,
            sections_right_db: None,
        },
    )
    .expect("response query");
    let after = support::snapshot(effect.as_ref());
    assert_eq!(before, after, "response query touched prepared state");
}

#[test]
fn response_query_allocates_nothing_after_caller_setup() {
    bench_alloc::set_mode(bench_alloc::Mode::Count);
    bench_alloc::assert_installed();
    let configured = configured_four_sections(48_000);
    let configuration = prepared(&configured, false, 48_000);
    let frequencies = [0.0_f32, 120.0, 1_000.0, 8_000.0, 24_000.0];
    let mut total_left = [f32::NAN; 5];
    let mut total_right = [f32::NAN; 5];
    let mut sections_left = [f32::NAN; EQ_SECTION_COUNT * 5];
    let mut sections_right = [f32::NAN; EQ_SECTION_COUNT * 5];
    let request = EqResponseRequest {
        configuration_id: 1,
        configuration: &configuration,
        frequencies_hz: &frequencies,
        maximum_points: frequencies.len(),
    };
    let mark = bench_alloc::current_thread_counters();
    query_response_into(
        request,
        EqResponseOutput {
            total_left_db: &mut total_left,
            total_right_db: &mut total_right,
            sections_left_db: Some(&mut sections_left),
            sections_right_db: Some(&mut sections_right),
        },
    )
    .expect("measured response query");
    let delta = bench_alloc::current_thread_delta_since(mark);
    assert_eq!(delta.allocations, 0, "response query allocated: {delta:?}");
    assert_eq!(delta.deallocations, 0, "response query freed: {delta:?}");
    assert_eq!(
        delta.reallocations, 0,
        "response query reallocated: {delta:?}"
    );
}
