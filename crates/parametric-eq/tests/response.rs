#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: the independent response oracle intentionally uses the
// platform's f64 methods; production evaluation uses `math`.

//! Acceptance gates for the native effect-owned response query.

mod support;

use bench_support::alloc as bench_alloc;
use dsp_reference::ReferenceSvfStateSpace;
use effect_contract::{EffectProcessBlock, NativeEffectFactory, ParameterChannel};
use parametric_eq::{
    EQ_SECTION_COUNT, EqBandKind, EqResponseError, EqResponseMode, EqResponseOutput,
    EqResponseRequest, ParametricEqFactory, design_svf, query_response_into,
};
use support::{LAUNCH_RATES, request_at_rate, set_initial, single_section_values, values};

const FLOOR_DB: f64 = -120.0;
const TOLERANCE_DB: f64 = 0.005;

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
            let mut total_left = [f32::NAN; 4];
            let mut total_right = [f32::NAN; 4];
            let mut sections_left = [f32::NAN; EQ_SECTION_COUNT * 4];
            let mut sections_right = [f32::NAN; EQ_SECTION_COUNT * 4];
            let summary = query_response_into(
                EqResponseRequest {
                    configuration_id: 9_007_199_254_740_993,
                    configuration: request_at_rate(&configured, false, rate),
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
            assert_eq!(summary.enabled_left[0], true);
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
    let configured = configured_four_sections(rate);
    let frequencies = [0.0, 120.0, 1_000.0, 8_000.0, rate as f32 * 0.5];
    let mut total_left = [f32::NAN; 5];
    let mut total_right = [f32::NAN; 5];
    let mut sections_left = [f32::NAN; EQ_SECTION_COUNT * 5];
    let mut sections_right = [f32::NAN; EQ_SECTION_COUNT * 5];
    let summary = query_response_into(
        EqResponseRequest {
            configuration_id: u64::MAX,
            configuration: request_at_rate(&configured, false, rate),
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
    }

    let mut bypass_total_left = [f32::NAN; 5];
    let mut bypass_total_right = [f32::NAN; 5];
    let mut bypass_sections_left = [f32::NAN; EQ_SECTION_COUNT * 5];
    let bypass_summary = query_response_into(
        EqResponseRequest {
            configuration_id: 42,
            configuration: request_at_rate(&configured, true, rate),
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
}

#[test]
fn malformed_requests_preserve_output_sentinels_and_opaque_ids() {
    let configured = values();
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
                configuration: request_at_rate(&configured, false, 48_000),
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
            configuration: request_at_rate(&configured, false, 48_000),
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
            configuration: request_at_rate(&configured, false, 48_000),
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
    let mut left = [129.0_f32; 2];
    let mut right = [130.0_f32; 2];
    let result = query_response_into(
        EqResponseRequest {
            configuration_id: 0,
            configuration: request_at_rate(&bad_values, false, 176_400),
            frequencies_hz: &valid,
            maximum_points: 2,
        },
        EqResponseOutput {
            total_left_db: &mut left,
            total_right_db: &mut right,
            sections_left_db: None,
            sections_right_db: None,
        },
    );
    assert!(matches!(result, Err(EqResponseError::Configuration(_))));
    assert_eq!(left, [129.0_f32; 2]);
    assert_eq!(right, [130.0_f32; 2]);

    let mut left = [f32::NAN; 2];
    let mut right = [f32::NAN; 2];
    let summary = query_response_into(
        EqResponseRequest {
            configuration_id: u64::MAX,
            configuration: request_at_rate(&configured, false, 48_000),
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
    let frequencies = [0.0, 1_000.0, 12_000.0, 24_000.0];
    let mut total_left = [f32::NAN; 4];
    let mut total_right = [f32::NAN; 4];
    query_response_into(
        EqResponseRequest {
            configuration_id: 3,
            configuration: request_at_rate(&configured, false, 48_000),
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
    let frequencies = [0.0_f32, 120.0, 1_000.0, 8_000.0, 24_000.0];
    let mut total_left = [f32::NAN; 5];
    let mut total_right = [f32::NAN; 5];
    let mut sections_left = [f32::NAN; EQ_SECTION_COUNT * 5];
    let mut sections_right = [f32::NAN; EQ_SECTION_COUNT * 5];
    let request = EqResponseRequest {
        configuration_id: 1,
        configuration: request_at_rate(&configured, false, 48_000),
        frequencies_hz: &frequencies,
        maximum_points: frequencies.len(),
    };
    // Warm descriptor validation and any one-time dependency state outside the measured query.
    query_response_into(
        request,
        EqResponseOutput {
            total_left_db: &mut total_left,
            total_right_db: &mut total_right,
            sections_left_db: Some(&mut sections_left),
            sections_right_db: Some(&mut sections_right),
        },
    )
    .expect("warm response query");
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
