#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: the independent response oracle intentionally uses the
// platform's f64 methods; production evaluation uses `math`.

//! Acceptance gates for the native effect-owned response query.

mod support;

use bench_support::alloc as bench_alloc;
use dsp_reference::ReferenceSvfStateSpace;
use effect_contract::{
    EffectProcessBlock, NativeEffectFactory, ParameterChannel, PreparedResponseAnalysis,
    ResponseAnalysisError, ResponseAnalysisMode, ResponseBypassSemantics, ResponseOutput,
    ResponsePrepareLimits, ResponseQuery, ResponseSnapshotKind, ResponseSnapshotRequest,
    ResponseSnapshotSection, ResponseTotalScope,
};
use parametric_eq::{
    EQ_SECTION_COUNT, EqBandKind, EqResponseConfiguration, EqResponseError, EqResponseMode,
    EqResponseOutput, EqResponseRequest, EqSvfWords, ParametricEqFactory, design_svf,
    query_response_into, query_snapshot_magnitudes_into,
};
use support::{
    FROZEN_FREQUENCIES, GridRow, LAUNCH_RATES, frozen_grid, request_at_rate, set_initial,
    single_section_values, values,
};

const FLOOR_DB: f64 = -120.0;
const TOLERANCE_DB: f64 = 0.005;

fn snapshot_sentinel() -> ResponseSnapshotSection {
    ResponseSnapshotSection {
        id: 99,
        kind: 99,
        enabled: true,
        word_count: 7,
        words: [0xDEAD_BEEF; effect_contract::RESPONSE_SNAPSHOT_WORDS],
    }
}

fn prepared(
    values: &[effect_contract::InitialParameterValue],
    bypass: bool,
    rate: u32,
) -> EqResponseConfiguration {
    EqResponseConfiguration::prepare(request_at_rate(values, bypass, rate))
        .expect("valid response configuration")
}

fn assert_bits_equal(actual: &[f32], expected: &[f32]) {
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.iter().zip(expected) {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
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

fn frozen_probe_grid(row: GridRow) -> ([f32; 5], usize) {
    let nyquist = row.rate as f32 * 0.5;
    let candidates = [
        0.0,
        row.frequency * 0.5,
        row.frequency,
        row.frequency * 1.5,
        nyquist,
    ];
    let mut sorted = candidates;
    sorted.sort_by(|left, right| left.partial_cmp(right).expect("finite probe"));
    let mut probes = [0.0_f32; 5];
    let mut points = 0;
    for probe in sorted {
        if probe <= nyquist && (points == 0 || probe > probes[points - 1]) {
            probes[points] = probe;
            points += 1;
        }
    }
    (probes, points)
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

fn values_for_frozen_row(row: GridRow) -> Vec<effect_contract::InitialParameterValue> {
    let mut configured = values();
    for channel in [ParameterChannel::Left, ParameterChannel::Right] {
        set_initial(&mut configured, 0, channel, 1.0);
        set_initial(&mut configured, 1, channel, row.kind as u32 as f32);
        set_initial(&mut configured, 2, channel, row.frequency);
        set_initial(&mut configured, 3, channel, row.gain);
        set_initial(&mut configured, 4, channel, row.q);
        set_initial(&mut configured, 5, channel, row.slope);
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
fn production_query_covers_the_complete_frozen_1488_row_corpus() {
    let rows = frozen_grid();
    assert_eq!(rows.len(), 1_488);
    assert!(FROZEN_FREQUENCIES.iter().all(|frequency| {
        rows.iter()
            .any(|row| row.frequency.to_bits() == frequency.to_bits())
    }));
    let mut worst_error = 0.0_f64;
    let mut probes = 0_u64;
    for row in rows {
        let configured = values_for_frozen_row(row);
        let configuration = prepared(&configured, false, row.rate);
        let (frequencies, points) = frozen_probe_grid(row);
        probes += points as u64;
        let mut total_left = [f32::NAN; 5];
        let mut total_right = [f32::NAN; 5];
        let mut sections_left = [f32::NAN; EQ_SECTION_COUNT * 5];
        let mut sections_right = [f32::NAN; EQ_SECTION_COUNT * 5];
        query_response_into(
            EqResponseRequest {
                configuration_id: u64::from(row.rate),
                configuration: &configuration,
                frequencies_hz: &frequencies[..points],
                maximum_points: points,
            },
            EqResponseOutput {
                total_left_db: &mut total_left[..points],
                total_right_db: &mut total_right[..points],
                sections_left_db: Some(&mut sections_left[..EQ_SECTION_COUNT * points]),
                sections_right_db: Some(&mut sections_right[..EQ_SECTION_COUNT * points]),
            },
        )
        .expect("every frozen row is a valid production query");
        for point in 0..points {
            let reference = oracle_db(
                row.kind,
                row.rate,
                row.frequency,
                row.gain,
                row.q,
                row.slope,
                frequencies[point],
            );
            assert!(
                !reference.is_nan() && reference != f64::INFINITY,
                "invalid independent oracle result for {row:?} f={}",
                frequencies[point]
            );
            let expected = if reference == f64::NEG_INFINITY {
                FLOOR_DB
            } else {
                reference.max(FLOOR_DB)
            };
            for (channel, actual) in [total_left[point], total_right[point]]
                .into_iter()
                .enumerate()
            {
                let error = (f64::from(actual) - expected).abs();
                worst_error = worst_error.max(error);
                assert!(
                    error <= TOLERANCE_DB,
                    "row={row:?} f={} channel={channel}: got={actual} expected={expected} error={error}",
                    frequencies[point]
                );
            }
            for section in 0..EQ_SECTION_COUNT {
                let left = sections_left[section * points + point];
                let right = sections_right[section * points + point];
                let expected = if section == 0 { expected } else { 0.0 };
                assert!((f64::from(left) - expected).abs() <= TOLERANCE_DB);
                assert!((f64::from(right) - expected).abs() <= TOLERANCE_DB);
            }
        }
    }
    assert!(probes >= 1_488 * 4);
    eprintln!(
        "issue-764 production frozen rows=1488 probes={probes} worst_error_db={worst_error:.6e}"
    );
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
        for (section, (kind, frequency, gain, q, slope)) in right_rows.into_iter().enumerate() {
            let words = design_svf(kind, frequency, gain, q, slope, engine::SampleRateHz(rate))
                .expect("right cascade design");
            let response = realized(words)
                .response(f64::from(rate), f64::from(frequencies[point]))
                .expect("right cascade probe");
            let expected_section = (20.0 * response.re.hypot(response.im).log10()).max(FLOOR_DB);
            let actual_section = sections_right[section * frequencies.len() + point];
            assert!(actual_section.is_finite());
            assert!(
                (f64::from(actual_section) - expected_section).abs() <= TOLERANCE_DB,
                "right section={section} f={}: got={actual_section} expected={expected_section}",
                frequencies[point]
            );
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
    assert_eq!(bypass_sections_left, sections_left);
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
    assert_eq!(right_only_sections, sections_right);
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
fn query_does_not_change_an_unfinished_ramp_or_following_audio() {
    let configured = single_section_values(EqBandKind::Bell, 1_000.0, 6.0, 0.7, 1.0);
    let request = request_at_rate(&configured, false, 48_000);
    let mut queried = ParametricEqFactory
        .prepare(request)
        .expect("prepared effect");
    let mut twin = ParametricEqFactory.prepare(request).expect("prepared twin");
    let automation = [support::point(3, ParameterChannel::Left, 0, -6.0)];
    let mut prefix_left = [0.125_f32; 16];
    let mut prefix_right = [-0.25_f32; 16];
    let mut twin_prefix_left = prefix_left;
    let mut twin_prefix_right = prefix_right;
    let queried_prefix_report = queried.process(
        EffectProcessBlock::new(
            &mut prefix_left,
            &mut prefix_right,
            None,
            0,
            &automation,
            128,
        )
        .expect("automation prefix"),
    );
    assert_eq!(queried_prefix_report.nonfinite_left_blocks, 0);
    assert_eq!(queried_prefix_report.nonfinite_right_blocks, 0);
    let twin_prefix_report = twin.process(
        EffectProcessBlock::new(
            &mut twin_prefix_left,
            &mut twin_prefix_right,
            None,
            0,
            &automation,
            128,
        )
        .expect("twin automation prefix"),
    );
    assert_eq!(twin_prefix_report.nonfinite_left_blocks, 0);
    assert_eq!(twin_prefix_report.nonfinite_right_blocks, 0);
    let before = support::snapshot(queried.as_ref());
    assert_eq!(before, support::snapshot(twin.as_ref()));
    assert!(
        support::band_word(&before.1, 0, 14) > 0,
        "the prefix must leave the gain ramp in flight"
    );

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
    assert_eq!(before, support::snapshot(queried.as_ref()));

    let mut continuation_left = [0.375_f32; 128];
    let mut continuation_right = [-0.5_f32; 128];
    let mut twin_continuation_left = continuation_left;
    let mut twin_continuation_right = continuation_right;
    let queried_continuation_report = queried.process(
        EffectProcessBlock::new(
            &mut continuation_left,
            &mut continuation_right,
            None,
            16,
            &[],
            128,
        )
        .expect("continued audio"),
    );
    assert_eq!(queried_continuation_report.nonfinite_left_blocks, 0);
    assert_eq!(queried_continuation_report.nonfinite_right_blocks, 0);
    let twin_continuation_report = twin.process(
        EffectProcessBlock::new(
            &mut twin_continuation_left,
            &mut twin_continuation_right,
            None,
            16,
            &[],
            128,
        )
        .expect("twin continued audio"),
    );
    assert_eq!(twin_continuation_report.nonfinite_left_blocks, 0);
    assert_eq!(twin_continuation_report.nonfinite_right_blocks, 0);
    assert!(continuation_left.iter().any(|sample| *sample != 0.0));
    assert!(continuation_right.iter().any(|sample| *sample != 0.0));
    for (actual, expected) in continuation_left.iter().zip(twin_continuation_left) {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
    for (actual, expected) in continuation_right.iter().zip(twin_continuation_right) {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
    assert_eq!(
        support::snapshot(queried.as_ref()),
        support::snapshot(twin.as_ref()),
        "query changed the continued effect state"
    );
}

#[test]
fn malformed_buffers_and_budgets_refuse_without_writes_or_allocations() {
    #[derive(Clone, Copy, Debug)]
    enum Refusal {
        InvalidGrid,
        ZeroBudget,
        BudgetTooSmall,
        LeftTotalShort,
        RightTotalShort,
        LeftTotalLong,
        RightTotalLong,
        LeftSectionsShort,
        RightSectionsShort,
        LeftSectionsLong,
        RightSectionsLong,
    }

    let configured = values();
    let configuration = prepared(&configured, false, 48_000);
    let valid = [100.0_f32, 1_000.0];
    let cases = [
        Refusal::InvalidGrid,
        Refusal::ZeroBudget,
        Refusal::BudgetTooSmall,
        Refusal::LeftTotalShort,
        Refusal::RightTotalShort,
        Refusal::LeftTotalLong,
        Refusal::RightTotalLong,
        Refusal::LeftSectionsShort,
        Refusal::RightSectionsShort,
        Refusal::LeftSectionsLong,
        Refusal::RightSectionsLong,
    ];
    bench_alloc::set_mode(bench_alloc::Mode::Count);
    for case in cases {
        let frequencies = if matches!(case, Refusal::InvalidGrid) {
            [100.0_f32, 100.0]
        } else {
            valid
        };
        let maximum_points = match case {
            Refusal::ZeroBudget => 0,
            Refusal::BudgetTooSmall => 1,
            _ => frequencies.len(),
        };
        let mut total_left = [11.0_f32; 3];
        let mut total_right = [12.0_f32; 3];
        let mut sections_left = [13.0_f32; EQ_SECTION_COUNT * 2 + 1];
        let mut sections_right = [14.0_f32; EQ_SECTION_COUNT * 2 + 1];
        let expected = match case {
            Refusal::InvalidGrid => EqResponseError::InvalidFrequencyGrid,
            Refusal::ZeroBudget | Refusal::BudgetTooSmall => EqResponseError::Capacity,
            _ => EqResponseError::OutputShape,
        };
        let mark = bench_alloc::current_thread_counters();
        let result = query_response_into(
            EqResponseRequest {
                configuration_id: 99,
                configuration: &configuration,
                frequencies_hz: &frequencies,
                maximum_points,
            },
            EqResponseOutput {
                total_left_db: match case {
                    Refusal::LeftTotalShort => &mut total_left[..1],
                    Refusal::LeftTotalLong => &mut total_left,
                    _ => &mut total_left[..frequencies.len()],
                },
                total_right_db: match case {
                    Refusal::RightTotalShort => &mut total_right[..1],
                    Refusal::RightTotalLong => &mut total_right,
                    _ => &mut total_right[..frequencies.len()],
                },
                sections_left_db: match case {
                    Refusal::LeftSectionsShort => Some(&mut sections_left[..7]),
                    Refusal::LeftSectionsLong => Some(&mut sections_left),
                    Refusal::RightSectionsShort | Refusal::RightSectionsLong => None,
                    _ => None,
                },
                sections_right_db: match case {
                    Refusal::RightSectionsShort => Some(&mut sections_right[..7]),
                    Refusal::RightSectionsLong => Some(&mut sections_right),
                    Refusal::LeftSectionsShort | Refusal::LeftSectionsLong => None,
                    _ => None,
                },
            },
        );
        let delta = bench_alloc::current_thread_delta_since(mark);
        assert_eq!(result, Err(expected), "unexpected result for {case:?}");
        assert_eq!(
            delta.allocations, 0,
            "refusal allocated for {case:?}: {delta:?}"
        );
        assert_eq!(
            delta.deallocations, 0,
            "refusal freed for {case:?}: {delta:?}"
        );
        assert_eq!(
            delta.reallocations, 0,
            "refusal reallocated for {case:?}: {delta:?}"
        );
        assert_eq!(total_left, [11.0_f32; 3], "left total mutated for {case:?}");
        assert_eq!(
            total_right, [12.0_f32; 3],
            "right total mutated for {case:?}"
        );
        assert_eq!(
            sections_left,
            [13.0_f32; EQ_SECTION_COUNT * 2 + 1],
            "left sections mutated for {case:?}"
        );
        assert_eq!(
            sections_right,
            [14.0_f32; EQ_SECTION_COUNT * 2 + 1],
            "right sections mutated for {case:?}"
        );
    }
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

fn common_prepared(
    values: &[effect_contract::InitialParameterValue],
    bypass: bool,
    rate: u32,
    maximum_prepared_bytes: usize,
) -> Result<Box<dyn PreparedResponseAnalysis>, ResponseAnalysisError> {
    let factory = ParametricEqFactory;
    factory
        .response_analysis()
        .expect("EQ response owner")
        .prepare_response(
            support::request_at_rate(values, bypass, rate),
            ResponsePrepareLimits {
                maximum_prepared_bytes,
            },
        )
}

#[test]
fn common_eq_adapter_is_bit_identical_for_asymmetric_sections_and_all_launch_rates() {
    for rate in LAUNCH_RATES {
        let mut configured = configured_four_sections(rate);
        set_initial(&mut configured, 3, ParameterChannel::Right, -6.0);
        set_initial(&mut configured, 6, ParameterChannel::Left, 0.0);
        set_initial(&mut configured, 12, ParameterChannel::Right, 0.0);
        set_initial(&mut configured, 18, ParameterChannel::Left, 0.0);
        let direct_configuration = prepared(&configured, false, rate);
        let provider = common_prepared(&configured, false, rate, usize::MAX).expect("provider");
        let descriptor = provider.analysis_descriptor();
        assert_eq!(descriptor.sections.len(), EQ_SECTION_COUNT);
        assert_eq!(descriptor.floor_db, -120.0);
        assert_eq!(
            descriptor.total_scope,
            ResponseTotalScope::ParametricEqCascade
        );
        assert_eq!(
            descriptor.bypass,
            ResponseBypassSemantics::EffectWideIdentityWithSections
        );
        assert_eq!(provider.configuration().sample_rate_hz, rate);
        assert_eq!(provider.configuration().bypass, Some(false));
        assert_eq!(
            provider.configuration().enabled_left,
            &[true, false, true, false]
        );
        assert_eq!(
            provider.configuration().enabled_right,
            &[true, true, false, true]
        );

        let frequencies = [0.0_f32, 120.0, 1_000.0, 8_000.0, rate as f32 * 0.5];
        let mut direct_left = [f32::NAN; 5];
        let mut direct_right = [f32::NAN; 5];
        let mut direct_sections_left = [f32::NAN; EQ_SECTION_COUNT * 5];
        let mut direct_sections_right = [f32::NAN; EQ_SECTION_COUNT * 5];
        query_response_into(
            EqResponseRequest {
                configuration_id: u64::MAX,
                configuration: &direct_configuration,
                frequencies_hz: &frequencies,
                maximum_points: frequencies.len(),
            },
            EqResponseOutput {
                total_left_db: &mut direct_left,
                total_right_db: &mut direct_right,
                sections_left_db: Some(&mut direct_sections_left),
                sections_right_db: Some(&mut direct_sections_right),
            },
        )
        .expect("direct query");

        let mut common_left = [f32::NAN; 5];
        let mut common_right = [f32::NAN; 5];
        let mut common_sections_left = [f32::NAN; EQ_SECTION_COUNT * 5];
        let mut common_sections_right = [f32::NAN; EQ_SECTION_COUNT * 5];
        let summary = provider
            .query_into(
                ResponseQuery {
                    configuration_id: u64::MAX,
                    frequencies_hz: &frequencies,
                    maximum_points: frequencies.len(),
                },
                ResponseOutput {
                    total_left_db: &mut common_left,
                    total_right_db: &mut common_right,
                    sections_left_db: Some(&mut common_sections_left),
                    sections_right_db: Some(&mut common_sections_right),
                },
            )
            .expect("common query");
        assert_eq!(summary.mode, ResponseAnalysisMode::RequestedConfiguration);
        assert_eq!(summary.configuration_id, u64::MAX);
        assert_bits_equal(&common_left, &direct_left);
        assert_bits_equal(&common_right, &direct_right);
        assert_bits_equal(&common_sections_left, &direct_sections_left);
        assert_bits_equal(&common_sections_right, &direct_sections_right);

        let mut total_only_left = [f32::NAN; 5];
        let mut total_only_right = [f32::NAN; 5];
        provider
            .query_into(
                ResponseQuery {
                    configuration_id: 1,
                    frequencies_hz: &frequencies,
                    maximum_points: frequencies.len(),
                },
                ResponseOutput {
                    total_left_db: &mut total_only_left,
                    total_right_db: &mut total_only_right,
                    sections_left_db: None,
                    sections_right_db: None,
                },
            )
            .expect("total-only common query");
        assert_bits_equal(&total_only_left, &direct_left);
        assert_bits_equal(&total_only_right, &direct_right);

        let mut left_only = [f32::NAN; EQ_SECTION_COUNT * 5];
        let mut left_only_left = [f32::NAN; 5];
        let mut left_only_right = [f32::NAN; 5];
        provider
            .query_into(
                ResponseQuery {
                    configuration_id: 2,
                    frequencies_hz: &frequencies,
                    maximum_points: frequencies.len(),
                },
                ResponseOutput {
                    total_left_db: &mut left_only_left,
                    total_right_db: &mut left_only_right,
                    sections_left_db: Some(&mut left_only),
                    sections_right_db: None,
                },
            )
            .expect("left-only common query");
        assert_bits_equal(&left_only_left, &direct_left);
        assert_bits_equal(&left_only_right, &direct_right);
        assert_bits_equal(&left_only, &direct_sections_left);

        let mut right_only = [f32::NAN; EQ_SECTION_COUNT * 5];
        let mut right_only_left = [f32::NAN; 5];
        let mut right_only_right = [f32::NAN; 5];
        provider
            .query_into(
                ResponseQuery {
                    configuration_id: 2,
                    frequencies_hz: &frequencies,
                    maximum_points: frequencies.len(),
                },
                ResponseOutput {
                    total_left_db: &mut right_only_left,
                    total_right_db: &mut right_only_right,
                    sections_left_db: None,
                    sections_right_db: Some(&mut right_only),
                },
            )
            .expect("right-only common query");
        assert_bits_equal(&right_only_left, &direct_left);
        assert_bits_equal(&right_only_right, &direct_right);
        assert_bits_equal(&right_only, &direct_sections_right);

        let bypass = common_prepared(&configured, true, rate, usize::MAX).expect("bypass provider");
        let mut bypass_left = [f32::NAN; 5];
        let mut bypass_right = [f32::NAN; 5];
        let mut bypass_sections = [f32::NAN; EQ_SECTION_COUNT * 5];
        bypass
            .query_into(
                ResponseQuery {
                    configuration_id: 3,
                    frequencies_hz: &frequencies,
                    maximum_points: frequencies.len(),
                },
                ResponseOutput {
                    total_left_db: &mut bypass_left,
                    total_right_db: &mut bypass_right,
                    sections_left_db: Some(&mut bypass_sections),
                    sections_right_db: None,
                },
            )
            .expect("bypass common query");
        assert!(bypass_left.iter().all(|value| value.to_bits() == 0));
        assert!(bypass_right.iter().all(|value| value.to_bits() == 0));
        assert_bits_equal(&bypass_sections, &direct_sections_left);
    }
}

#[test]
fn common_eq_adapter_preserves_input_ownership_capacity_and_refusal_sentinels() {
    let mut values = configured_four_sections(48_000);
    let provider = common_prepared(&values, false, 48_000, usize::MAX).expect("provider");
    let retained = provider.retained_bytes();
    assert!(retained > 0);
    assert!(matches!(
        common_prepared(&values, false, 48_000, retained - 1),
        Err(ResponseAnalysisError::ResourceLimit)
    ));
    assert!(common_prepared(&values, false, 48_000, retained).is_ok());
    let frequencies = [100.0_f32, 1_000.0];
    let enabled_left_before = provider.configuration().enabled_left.to_owned();
    let enabled_right_before = provider.configuration().enabled_right.to_owned();
    let mut before_left = [f32::NAN; 2];
    let mut before_right = [f32::NAN; 2];
    provider
        .query_into(
            ResponseQuery {
                configuration_id: 1,
                frequencies_hz: &frequencies,
                maximum_points: frequencies.len(),
            },
            ResponseOutput {
                total_left_db: &mut before_left,
                total_right_db: &mut before_right,
                sections_left_db: None,
                sections_right_db: None,
            },
        )
        .expect("pre-mutation common query");
    values[2].value = 20_000.0;
    values[3].value = -12.0;
    let mut after_left = [f32::NAN; 2];
    let mut after_right = [f32::NAN; 2];
    provider
        .query_into(
            ResponseQuery {
                configuration_id: 2,
                frequencies_hz: &frequencies,
                maximum_points: frequencies.len(),
            },
            ResponseOutput {
                total_left_db: &mut after_left,
                total_right_db: &mut after_right,
                sections_left_db: None,
                sections_right_db: None,
            },
        )
        .expect("post-mutation common query");
    assert_bits_equal(&after_left, &before_left);
    assert_bits_equal(&after_right, &before_right);
    assert_eq!(provider.configuration().enabled_left, &enabled_left_before);
    assert_eq!(
        provider.configuration().enabled_right,
        &enabled_right_before
    );

    let mut invalid_left = [47.0_f32; 1];
    let mut invalid_right = [48.0_f32; 1];
    let mut invalid_sections_left = [49.0_f32; EQ_SECTION_COUNT];
    let mut invalid_sections_right = [50.0_f32; EQ_SECTION_COUNT];
    let invalid = provider.query_into(
        ResponseQuery {
            configuration_id: u64::MAX,
            frequencies_hz: &[f32::NAN],
            maximum_points: 1,
        },
        ResponseOutput {
            total_left_db: &mut invalid_left,
            total_right_db: &mut invalid_right,
            sections_left_db: Some(&mut invalid_sections_left),
            sections_right_db: Some(&mut invalid_sections_right),
        },
    );
    assert_eq!(invalid, Err(ResponseAnalysisError::InvalidFrequencyGrid));
    assert_eq!(invalid_left, [47.0]);
    assert_eq!(invalid_right, [48.0]);
    assert_eq!(invalid_sections_left, [49.0; EQ_SECTION_COUNT]);
    assert_eq!(invalid_sections_right, [50.0; EQ_SECTION_COUNT]);

    let mut left = [41.0_f32; 2];
    let mut right = [42.0_f32; 2];
    let mut sections_left = [43.0_f32; EQ_SECTION_COUNT * 2];
    let mut sections_right = [44.0_f32; EQ_SECTION_COUNT * 2];
    let result = provider.query_into(
        ResponseQuery {
            configuration_id: u64::MAX,
            frequencies_hz: &[1_000.0, 100.0],
            maximum_points: 2,
        },
        ResponseOutput {
            total_left_db: &mut left,
            total_right_db: &mut right,
            sections_left_db: Some(&mut sections_left),
            sections_right_db: Some(&mut sections_right),
        },
    );
    assert_eq!(result, Err(ResponseAnalysisError::InvalidFrequencyGrid));
    assert_eq!(left, [41.0; 2]);
    assert_eq!(right, [42.0; 2]);
    assert_eq!(sections_left, [43.0; EQ_SECTION_COUNT * 2]);
    assert_eq!(sections_right, [44.0; EQ_SECTION_COUNT * 2]);

    let mut left = [43.0_f32; 2];
    let mut right = [44.0_f32; 2];
    let mut sections_left = [45.0_f32; EQ_SECTION_COUNT * 2];
    let mut sections_right = [46.0_f32; EQ_SECTION_COUNT * 2];
    let result = provider.query_into(
        ResponseQuery {
            configuration_id: u64::MAX,
            frequencies_hz: &frequencies,
            maximum_points: 1,
        },
        ResponseOutput {
            total_left_db: &mut left,
            total_right_db: &mut right,
            sections_left_db: Some(&mut sections_left),
            sections_right_db: Some(&mut sections_right),
        },
    );
    assert_eq!(result, Err(ResponseAnalysisError::Capacity));
    assert_eq!(left, [43.0; 2]);
    assert_eq!(right, [44.0; 2]);
    assert_eq!(sections_left, [45.0; EQ_SECTION_COUNT * 2]);
    assert_eq!(sections_right, [46.0; EQ_SECTION_COUNT * 2]);

    let mut left = [45.0_f32; 2];
    let mut right = [46.0_f32; 2];
    let result = provider.query_into(
        ResponseQuery {
            configuration_id: u64::MAX,
            frequencies_hz: &frequencies,
            maximum_points: 2,
        },
        ResponseOutput {
            total_left_db: &mut left[..1],
            total_right_db: &mut right,
            sections_left_db: None,
            sections_right_db: None,
        },
    );
    assert_eq!(result, Err(ResponseAnalysisError::OutputShape));
    assert_eq!(left, [45.0; 2]);
    assert_eq!(right, [46.0; 2]);

    let mut left = [49.0_f32; 2];
    let mut right = [50.0_f32; 2];
    let mut sections = [51.0_f32; 8];
    let result = provider.query_into(
        ResponseQuery {
            configuration_id: u64::MAX,
            frequencies_hz: &frequencies,
            maximum_points: frequencies.len(),
        },
        ResponseOutput {
            total_left_db: &mut left,
            total_right_db: &mut right,
            sections_left_db: Some(&mut sections[..7]),
            sections_right_db: None,
        },
    );
    assert_eq!(result, Err(ResponseAnalysisError::OutputShape));
    assert_eq!(left, [49.0; 2]);
    assert_eq!(right, [50.0; 2]);
    assert_eq!(sections, [51.0; 8]);

    let mut left = [52.0_f32; 2];
    let mut right = [53.0_f32; 2];
    let mut sections = [54.0_f32; 9];
    let result = provider.query_into(
        ResponseQuery {
            configuration_id: u64::MAX,
            frequencies_hz: &frequencies,
            maximum_points: frequencies.len(),
        },
        ResponseOutput {
            total_left_db: &mut left,
            total_right_db: &mut right,
            sections_left_db: Some(&mut sections),
            sections_right_db: None,
        },
    );
    assert_eq!(result, Err(ResponseAnalysisError::OutputShape));
    assert_eq!(left, [52.0; 2]);
    assert_eq!(right, [53.0; 2]);
    assert_eq!(sections, [54.0; 9]);

    let mut left = [55.0_f32; 2];
    let mut right = [56.0_f32; 2];
    let mut right_sections = [57.0_f32; 9];
    let result = provider.query_into(
        ResponseQuery {
            configuration_id: u64::MAX,
            frequencies_hz: &frequencies,
            maximum_points: frequencies.len(),
        },
        ResponseOutput {
            total_left_db: &mut left,
            total_right_db: &mut right,
            sections_left_db: None,
            sections_right_db: Some(&mut right_sections[..7]),
        },
    );
    assert_eq!(result, Err(ResponseAnalysisError::OutputShape));
    assert_eq!(left, [55.0; 2]);
    assert_eq!(right, [56.0; 2]);
    assert_eq!(right_sections, [57.0; 9]);

    let mut invalid = values.clone();
    set_initial(&mut invalid, 2, ParameterChannel::Left, f32::NAN);
    assert!(matches!(
        common_prepared(&invalid, false, 48_000, usize::MAX),
        Err(ResponseAnalysisError::Configuration(
            effect_contract::EffectPrepareError {
                code: "effect.parameter.initial"
            }
        ))
    ));
}

#[test]
fn common_eq_trait_object_query_is_allocation_free() {
    bench_alloc::assert_installed();
    let configured = configured_four_sections(48_000);
    let provider = common_prepared(&configured, false, 48_000, usize::MAX).expect("provider");
    let frequencies = [0.0_f32, 120.0, 1_000.0, 8_000.0, 24_000.0];
    let mut left = [f32::NAN; 5];
    let mut right = [f32::NAN; 5];
    bench_alloc::set_mode(bench_alloc::Mode::Count);
    let trait_provider = &*provider as &dyn PreparedResponseAnalysis;
    let mark = bench_alloc::current_thread_counters();
    assert_eq!(trait_provider.analysis_descriptor().floor_db, -120.0);
    let view = trait_provider.configuration();
    assert_eq!(view.sample_rate_hz, 48_000);
    assert_eq!(view.enabled_left.len(), EQ_SECTION_COUNT);
    assert_eq!(trait_provider.retained_bytes(), provider.retained_bytes());
    let delta = bench_alloc::current_thread_delta_since(mark);
    assert_eq!(delta.allocations, 0, "trait metadata allocated: {delta:?}");
    assert_eq!(delta.deallocations, 0, "trait metadata freed: {delta:?}");
    assert_eq!(
        delta.reallocations, 0,
        "trait metadata reallocated: {delta:?}"
    );

    let mut refused_left = [31.0_f32; 5];
    let mut refused_right = [32.0_f32; 5];
    let mut refused_sections_left = [33.0_f32; EQ_SECTION_COUNT * 5];
    let mut refused_sections_right = [34.0_f32; EQ_SECTION_COUNT * 5];
    let mark = bench_alloc::current_thread_counters();
    let refusal = trait_provider.query_into(
        ResponseQuery {
            configuration_id: u64::MAX,
            frequencies_hz: &[1_000.0, 100.0],
            maximum_points: 2,
        },
        ResponseOutput {
            total_left_db: &mut refused_left[..2],
            total_right_db: &mut refused_right[..2],
            sections_left_db: Some(&mut refused_sections_left[..EQ_SECTION_COUNT * 2]),
            sections_right_db: Some(&mut refused_sections_right[..EQ_SECTION_COUNT * 2]),
        },
    );
    let delta = bench_alloc::current_thread_delta_since(mark);
    assert_eq!(refusal, Err(ResponseAnalysisError::InvalidFrequencyGrid));
    assert_eq!(delta.allocations, 0, "trait refusal allocated: {delta:?}");
    assert_eq!(delta.deallocations, 0, "trait refusal freed: {delta:?}");
    assert_eq!(
        delta.reallocations, 0,
        "trait refusal reallocated: {delta:?}"
    );
    assert_eq!(refused_left, [31.0; 5]);
    assert_eq!(refused_right, [32.0; 5]);
    assert_eq!(refused_sections_left, [33.0; EQ_SECTION_COUNT * 5]);
    assert_eq!(refused_sections_right, [34.0; EQ_SECTION_COUNT * 5]);

    let mark = bench_alloc::current_thread_counters();
    let summary = trait_provider
        .query_into(
            ResponseQuery {
                configuration_id: 9_007_199_254_740_993,
                frequencies_hz: &frequencies,
                maximum_points: frequencies.len(),
            },
            ResponseOutput {
                total_left_db: &mut left,
                total_right_db: &mut right,
                sections_left_db: None,
                sections_right_db: None,
            },
        )
        .expect("trait-object query");
    let delta = bench_alloc::current_thread_delta_since(mark);
    assert_eq!(summary.configuration_id, 9_007_199_254_740_993);
    assert_eq!(delta.allocations, 0, "trait query allocated: {delta:?}");
    assert_eq!(delta.deallocations, 0, "trait query freed: {delta:?}");
    assert_eq!(delta.reallocations, 0, "trait query reallocated: {delta:?}");
}

#[test]
fn prepared_owner_snapshot_copies_asymmetric_target_words() {
    let mut configured = values();
    for channel in [ParameterChannel::Left, ParameterChannel::Right] {
        set_initial(&mut configured, 0, channel, 1.0);
        set_initial(&mut configured, 1, channel, EqBandKind::Bell as u32 as f32);
        set_initial(&mut configured, 2, channel, 1_000.0);
        set_initial(
            &mut configured,
            3,
            channel,
            if channel == ParameterChannel::Left {
                6.0
            } else {
                -6.0
            },
        );
        set_initial(&mut configured, 4, channel, 0.7);
        set_initial(&mut configured, 5, channel, 1.0);
    }
    let effect = ParametricEqFactory
        .prepare(request_at_rate(&configured, false, 48_000))
        .expect("prepared effect");
    let sentinel = snapshot_sentinel();
    let mut left = [sentinel; EQ_SECTION_COUNT];
    let mut right = [sentinel; EQ_SECTION_COUNT];
    let summary = effect
        .copy_response_snapshot(ResponseSnapshotRequest {
            bypassed: false,
            left: &mut left,
            right: &mut right,
        })
        .expect("owner snapshot");

    assert_eq!(summary.kind, ResponseSnapshotKind::ParametricEq);
    assert_eq!(summary.sample_rate_hz, 48_000);
    assert!(!summary.bypassed);
    assert_eq!(summary.sections, EQ_SECTION_COUNT as u32);
    assert_eq!(left[0].kind, EqBandKind::Bell as u32);
    assert!(left[0].enabled && right[0].enabled);
    assert_eq!(left[0].word_count, 6);
    assert_ne!(left[0].words[4], right[0].words[4]);
    assert_eq!(
        left[1].words[..6],
        EqSvfWords::IDENTITY.to_array().map(f32::to_bits)
    );
    assert!(!left[1].enabled);
    assert_eq!(right[3], left[3]);
}

#[test]
fn prepared_owner_snapshot_rejects_wrong_shape_without_writing() {
    let configured = single_section_values(EqBandKind::Bell, 1_000.0, 6.0, 0.7, 1.0);
    let effect = ParametricEqFactory
        .prepare(request_at_rate(&configured, false, 48_000))
        .expect("prepared effect");
    let sentinel = snapshot_sentinel();
    let mut left = [sentinel; EQ_SECTION_COUNT - 1];
    let mut right = [sentinel; EQ_SECTION_COUNT];
    let error = effect
        .copy_response_snapshot(ResponseSnapshotRequest {
            bypassed: false,
            left: &mut left,
            right: &mut right,
        })
        .expect_err("wrong shape must refuse");
    assert_eq!(error, ResponseAnalysisError::OutputShape);
    assert!(left.iter().all(|section| *section == sentinel));
    assert!(right.iter().all(|section| *section == sentinel));
}

#[test]
fn snapshot_magnitude_query_validates_both_lanes_before_publishing() {
    let identity = ResponseSnapshotSection {
        id: 1,
        kind: 1,
        enabled: false,
        word_count: 6,
        words: [0; effect_contract::RESPONSE_SNAPSHOT_WORDS],
    };
    let singular = ResponseSnapshotSection {
        id: 1,
        kind: 1,
        enabled: true,
        word_count: 6,
        words: [0; effect_contract::RESPONSE_SNAPSHOT_WORDS],
    };
    let left_sections = [identity; EQ_SECTION_COUNT];
    let right_sections = [singular; EQ_SECTION_COUNT];
    let mut left = [31.0_f64];
    let mut right = [37.0_f64];
    assert_eq!(
        query_snapshot_magnitudes_into(
            48_000,
            false,
            &left_sections,
            &right_sections,
            &[0.0],
            1,
            &mut left,
            &mut right,
        ),
        Err(EqResponseError::Numerical)
    );
    assert_eq!(left, [31.0]);
    assert_eq!(right, [37.0]);
}
