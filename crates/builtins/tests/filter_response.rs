#![allow(clippy::disallowed_methods)]
//! Acceptance gates for the owner-local native input HPF/LPF response query.

use bench_support::alloc as bench_alloc;
use builtins::test_support;
use builtins::{
    BuiltinChain, BuiltinLaneSelector, BuiltinParameterError, BuiltinParameters, ChannelParameters,
    DualMonoBlock, InputBuiltins, InputFilterResponseError, InputFilterResponseMode,
    InputFilterResponseOutput, InputFilterResponseRequest, InputFilterResponseSummary, Matrix2x2,
    builtin_filter_cutoff_maximum_hz, query_input_filter_response_into,
};
use dsp_reference::ReferenceSvfStateSpace;
use engine::LAUNCH_SAMPLE_RATES;

const FLOOR_DB: f64 = -120.0;
const TOLERANCE_DB: f64 = 0.005;

fn filters(left_hpf: f32, left_lpf: f32, right_hpf: f32, right_lpf: f32) -> BuiltinParameters {
    BuiltinParameters {
        left: ChannelParameters {
            hpf_hz: left_hpf,
            lpf_hz: left_lpf,
            ..ChannelParameters::default()
        },
        right: ChannelParameters {
            hpf_hz: right_hpf,
            lpf_hz: right_lpf,
            ..ChannelParameters::default()
        },
        ..BuiltinParameters::default()
    }
}

fn probe_grid(rate: u32, extras: &[f32]) -> Vec<f32> {
    let mut frequencies = vec![0.0, 10.0, 100.0, 1_000.0, 5_000.0, rate as f32 * 0.5];
    frequencies.extend_from_slice(extras);
    frequencies.sort_by(f32::total_cmp);
    frequencies.dedup_by(|left, right| *left == *right);
    frequencies
}

fn reference_section(rate: u32, cutoff: f32, high_pass: bool) -> ReferenceSvfStateSpace {
    let [c1, a2, a3, _, m0, m1, m2] = test_support::section_words(rate, cutoff, high_pass)
        .expect("reference words")
        .map(f32::from_bits);
    ReferenceSvfStateSpace::new(
        f64::from(c1),
        f64::from(a2),
        f64::from(a3),
        [f64::from(m0), f64::from(m1), f64::from(m2)],
    )
}

fn normalize_db(value: f64) -> f64 {
    assert!(!value.is_nan() && value != f64::INFINITY);
    if value == f64::NEG_INFINITY {
        FLOOR_DB
    } else {
        value.max(FLOOR_DB)
    }
}

fn section_db(rate: u32, cutoff: f32, high_pass: bool, frequency: f32) -> f64 {
    if cutoff == 0.0 {
        return 0.0;
    }
    normalize_db(
        reference_section(rate, cutoff, high_pass)
            .magnitude_db(f64::from(rate), f64::from(frequency))
            .expect("reference response"),
    )
}

fn total_db(rate: u32, hpf: f32, lpf: f32, frequency: f32) -> f64 {
    let mut product = dsp_reference::Complex64 { re: 1.0, im: 0.0 };
    for (cutoff, high_pass) in [(hpf, true), (lpf, false)] {
        if cutoff == 0.0 {
            continue;
        }
        let response = reference_section(rate, cutoff, high_pass)
            .response(f64::from(rate), f64::from(frequency))
            .expect("reference response");
        product = dsp_reference::Complex64 {
            re: product.re * response.re - product.im * response.im,
            im: product.re * response.im + product.im * response.re,
        };
    }
    normalize_db(20.0 * product.re.hypot(product.im).log10())
}

fn query_with_sections(
    rate: u32,
    configuration: BuiltinParameters,
    frequencies: &[f32],
    sections: bool,
) -> (
    InputFilterResponseSummary,
    Vec<f32>,
    Vec<f32>,
    Vec<f32>,
    Vec<f32>,
) {
    let mut total_left = vec![f32::NAN; frequencies.len()];
    let mut total_right = vec![f32::NAN; frequencies.len()];
    let mut sections_left = vec![f32::NAN; frequencies.len() * 2];
    let mut sections_right = vec![f32::NAN; frequencies.len() * 2];
    let summary = query_input_filter_response_into(
        InputFilterResponseRequest {
            configuration_id: 9_007_199_254_740_993,
            sample_rate_hz: rate,
            configuration,
            frequencies_hz: frequencies,
            maximum_points: frequencies.len(),
        },
        InputFilterResponseOutput {
            total_left_db: &mut total_left,
            total_right_db: &mut total_right,
            sections_left_db: sections.then_some(&mut sections_left),
            sections_right_db: sections.then_some(&mut sections_right),
        },
    )
    .expect("valid input-filter response query");
    (
        summary,
        total_left,
        total_right,
        sections_left,
        sections_right,
    )
}

fn assert_close(actual: f32, expected: f64, context: &str) {
    assert!(
        (f64::from(actual) - expected).abs() <= TOLERANCE_DB,
        "{context}: actual={actual} expected={expected}"
    );
}

#[test]
fn launch_rate_corpus_matches_independent_words_for_both_channels_and_sections() {
    for rate in LAUNCH_SAMPLE_RATES.map(|rate| rate.0) {
        let maximum = builtin_filter_cutoff_maximum_hz(rate).expect("launch maximum");
        let cases = [
            filters(0.0, 0.0, 0.0, 0.0),
            filters(100.0, 0.0, 1_000.0, 0.0),
            filters(0.0, 1_000.0, 0.0, maximum),
            filters(100.0, maximum, 1_000.0, 8_000.0),
        ];
        for configuration in cases {
            let frequencies = probe_grid(
                rate,
                &[
                    configuration.left.hpf_hz,
                    configuration.left.lpf_hz,
                    configuration.right.hpf_hz,
                    configuration.right.lpf_hz,
                ],
            );
            let (summary, total_left, total_right, sections_left, sections_right) =
                query_with_sections(rate, configuration, &frequencies, true);
            assert_eq!(summary.configuration_id, 9_007_199_254_740_993);
            assert_eq!(
                summary.mode,
                InputFilterResponseMode::RequestedConfiguration
            );
            assert_eq!(summary.sample_rate_hz, rate);
            assert_eq!(summary.points, frequencies.len());
            assert_eq!(
                summary.enabled_left,
                [
                    configuration.left.hpf_hz != 0.0,
                    configuration.left.lpf_hz != 0.0
                ]
            );
            assert_eq!(
                summary.enabled_right,
                [
                    configuration.right.hpf_hz != 0.0,
                    configuration.right.lpf_hz != 0.0
                ]
            );
            assert_eq!(summary.floor_db, FLOOR_DB as f32);

            for (point, frequency) in frequencies.iter().copied().enumerate() {
                assert_close(
                    total_left[point],
                    total_db(
                        rate,
                        configuration.left.hpf_hz,
                        configuration.left.lpf_hz,
                        frequency,
                    ),
                    "left total",
                );
                assert_close(
                    total_right[point],
                    total_db(
                        rate,
                        configuration.right.hpf_hz,
                        configuration.right.lpf_hz,
                        frequency,
                    ),
                    "right total",
                );
                assert_close(
                    sections_left[point],
                    section_db(rate, configuration.left.hpf_hz, true, frequency),
                    "left HPF",
                );
                assert_close(
                    sections_left[frequencies.len() + point],
                    section_db(rate, configuration.left.lpf_hz, false, frequency),
                    "left LPF",
                );
                assert_close(
                    sections_right[point],
                    section_db(rate, configuration.right.hpf_hz, true, frequency),
                    "right HPF",
                );
                assert_close(
                    sections_right[frequencies.len() + point],
                    section_db(rate, configuration.right.lpf_hz, false, frequency),
                    "right LPF",
                );
            }
            if configuration.left.hpf_hz == 0.0 && configuration.left.lpf_hz == 0.0 {
                assert!(total_left.iter().all(|value| value.to_bits() == 0));
                assert!(sections_left.iter().all(|value| value.to_bits() == 0));
            }
        }
    }
}

#[test]
fn optional_buffers_and_filter_subtotal_ignore_excluded_fields() {
    let rate = 48_000;
    let base = filters(100.0, 4_000.0, 1_000.0, 8_000.0);
    let frequencies = [0.0, 100.0, 1_000.0, 4_000.0, 8_000.0, 24_000.0];
    let (summary, total_left, total_right, sections_left, sections_right) =
        query_with_sections(rate, base, &frequencies, true);
    let (_, total_only_left, total_only_right, _, _) =
        query_with_sections(rate, base, &frequencies, false);
    assert_eq!(total_left, total_only_left);
    assert_eq!(total_right, total_only_right);

    let mut left_only = vec![f32::NAN; frequencies.len() * 2];
    let mut right_only = vec![f32::NAN; frequencies.len() * 2];
    let mut left_total = vec![f32::NAN; frequencies.len()];
    let mut right_total = vec![f32::NAN; frequencies.len()];
    query_input_filter_response_into(
        InputFilterResponseRequest {
            configuration_id: u64::MAX,
            sample_rate_hz: rate,
            configuration: base,
            frequencies_hz: &frequencies,
            maximum_points: frequencies.len(),
        },
        InputFilterResponseOutput {
            total_left_db: &mut left_total,
            total_right_db: &mut right_total,
            sections_left_db: Some(&mut left_only),
            sections_right_db: None,
        },
    )
    .expect("left-only section query");
    assert_eq!(left_only, sections_left);
    query_input_filter_response_into(
        InputFilterResponseRequest {
            configuration_id: u64::MAX,
            sample_rate_hz: rate,
            configuration: base,
            frequencies_hz: &frequencies,
            maximum_points: frequencies.len(),
        },
        InputFilterResponseOutput {
            total_left_db: &mut left_total,
            total_right_db: &mut right_total,
            sections_left_db: None,
            sections_right_db: Some(&mut right_only),
        },
    )
    .expect("right-only section query");
    assert_eq!(right_only, sections_right);

    let mut variant = base;
    variant.left.polarity_invert = true;
    variant.left.trim_db = -12.0;
    variant.left.fader_db = 6.0;
    variant.left.muted = true;
    variant.right.trim_db = 18.0;
    variant.right.fader_db = -24.0;
    variant.right.muted = true;
    variant.matrix = Matrix2x2 {
        ll: 0.75,
        lr: -0.25,
        rl: 0.5,
        rr: 0.5,
    };
    variant.smoothing_samples = 64;
    let (_, variant_left, variant_right, _, _) =
        query_with_sections(rate, variant, &frequencies, false);
    assert_eq!(total_left, variant_left);
    assert_eq!(total_right, variant_right);
    assert_eq!(summary.enabled_left, [true, true]);
    assert_eq!(summary.enabled_right, [true, true]);
}

fn impulse_db(samples: &[f32], rate: u32, frequency: f32) -> f64 {
    let phase = -core::f64::consts::TAU * f64::from(frequency) / f64::from(rate);
    let (step_re, step_im) = (phase.cos(), phase.sin());
    let (mut unit_re, mut unit_im) = (1.0_f64, 0.0_f64);
    let (mut re, mut im) = (0.0_f64, 0.0_f64);
    for sample in samples {
        let sample = f64::from(*sample);
        re += sample * unit_re;
        im += sample * unit_im;
        (unit_re, unit_im) = (
            unit_re * step_re - unit_im * step_im,
            unit_re * step_im + unit_im * step_re,
        );
    }
    20.0 * re.hypot(im).log10()
}

#[test]
fn settled_pcm_impulse_matches_query_at_all_launch_rates() {
    let frequencies = [250.0_f32, 1_000.0, 3_000.0];
    for rate in LAUNCH_SAMPLE_RATES.map(|rate| rate.0) {
        let configuration = filters(100.0, 4_000.0, 1_000.0, 8_000.0);
        let (_, queried_left, queried_right, _, _) =
            query_with_sections(rate, configuration, &frequencies, false);
        let mut input = BuiltinChain::new(rate, configuration)
            .expect("PCM chain")
            .into_input_builtins();
        let frames = 8_192;
        let mut left = vec![0.0_f32; frames];
        let mut right = vec![0.0_f32; frames];
        left[0] = 1.0;
        right[0] = 1.0;
        for first in (0..frames).step_by(128) {
            let report = input.process(
                DualMonoBlock::new(
                    &mut left[first..first + 128],
                    &mut right[first..first + 128],
                    first as u64,
                )
                .expect("PCM block"),
            );
            assert_eq!(report.recovered_left_state, 0);
            assert_eq!(report.recovered_right_state, 0);
        }
        for (index, frequency) in frequencies.iter().copied().enumerate() {
            let measured_left = impulse_db(&left, rate, frequency);
            let measured_right = impulse_db(&right, rate, frequency);
            assert!(measured_left.is_finite() && measured_right.is_finite());
            assert!(
                (measured_left - f64::from(queried_left[index])).abs() <= 0.05,
                "rate={rate} frequency={frequency} left measured={measured_left} query={}",
                queried_left[index]
            );
            assert!(
                (measured_right - f64::from(queried_right[index])).abs() <= 0.05,
                "rate={rate} frequency={frequency} right measured={measured_right} query={}",
                queried_right[index]
            );
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Refusal {
    InvalidGrid,
    ZeroBudget,
    OverBudget,
    LeftTotalShort,
    RightTotalShort,
    LeftTotalLong,
    RightTotalLong,
    LeftSectionsShort,
    RightSectionsShort,
    LeftSectionsLong,
    RightSectionsLong,
}

#[test]
fn malformed_grids_shapes_and_budgets_are_atomic_and_allocation_free() {
    let configuration = filters(100.0, 4_000.0, 1_000.0, 8_000.0);
    let valid = [100.0_f32, 1_000.0];
    let cases = [
        Refusal::InvalidGrid,
        Refusal::ZeroBudget,
        Refusal::OverBudget,
        Refusal::LeftTotalShort,
        Refusal::RightTotalShort,
        Refusal::LeftTotalLong,
        Refusal::RightTotalLong,
        Refusal::LeftSectionsShort,
        Refusal::RightSectionsShort,
        Refusal::LeftSectionsLong,
        Refusal::RightSectionsLong,
    ];
    bench_alloc::assert_installed();
    bench_alloc::set_mode(bench_alloc::Mode::Count);
    for case in cases {
        let frequencies = if matches!(case, Refusal::InvalidGrid) {
            [100.0_f32, 100.0]
        } else {
            valid
        };
        let maximum_points = match case {
            Refusal::ZeroBudget => 0,
            Refusal::OverBudget => 1,
            _ => frequencies.len(),
        };
        let mut total_left = [11.0_f32; 3];
        let mut total_right = [12.0_f32; 3];
        let mut sections_left = [13.0_f32; 5];
        let mut sections_right = [14.0_f32; 5];
        let expected = match case {
            Refusal::InvalidGrid => InputFilterResponseError::InvalidFrequencyGrid,
            Refusal::ZeroBudget | Refusal::OverBudget => InputFilterResponseError::Capacity,
            _ => InputFilterResponseError::OutputShape,
        };
        let mark = bench_alloc::current_thread_counters();
        let result = query_input_filter_response_into(
            InputFilterResponseRequest {
                configuration_id: u64::MAX,
                sample_rate_hz: 48_000,
                configuration,
                frequencies_hz: &frequencies,
                maximum_points,
            },
            InputFilterResponseOutput {
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
                    Refusal::LeftSectionsShort => Some(&mut sections_left[..3]),
                    Refusal::LeftSectionsLong => Some(&mut sections_left),
                    _ => None,
                },
                sections_right_db: match case {
                    Refusal::RightSectionsShort => Some(&mut sections_right[..3]),
                    Refusal::RightSectionsLong => Some(&mut sections_right),
                    _ => None,
                },
            },
        );
        let delta = bench_alloc::current_thread_delta_since(mark);
        assert_eq!(result, Err(expected), "case={case:?}");
        assert_eq!(delta.allocations, 0, "case={case:?} allocations={delta:?}");
        assert_eq!(delta.deallocations, 0, "case={case:?} frees={delta:?}");
        assert_eq!(
            delta.reallocations, 0,
            "case={case:?} reallocations={delta:?}"
        );
        assert_eq!(total_left, [11.0_f32; 3]);
        assert_eq!(total_right, [12.0_f32; 3]);
        assert_eq!(sections_left, [13.0_f32; 5]);
        assert_eq!(sections_right, [14.0_f32; 5]);
    }
}

#[test]
fn valid_query_is_allocation_free_after_setup() {
    let configuration = filters(100.0, 4_000.0, 1_000.0, 8_000.0);
    let frequencies = [0.0_f32, 100.0, 1_000.0, 4_000.0, 24_000.0];
    let mut total_left = [f32::NAN; 5];
    let mut total_right = [f32::NAN; 5];
    let mut sections_left = [f32::NAN; 10];
    let mut sections_right = [f32::NAN; 10];
    bench_alloc::set_mode(bench_alloc::Mode::Count);
    let mark = bench_alloc::current_thread_counters();
    query_input_filter_response_into(
        InputFilterResponseRequest {
            configuration_id: 1,
            sample_rate_hz: 48_000,
            configuration,
            frequencies_hz: &frequencies,
            maximum_points: frequencies.len(),
        },
        InputFilterResponseOutput {
            total_left_db: &mut total_left,
            total_right_db: &mut total_right,
            sections_left_db: Some(&mut sections_left),
            sections_right_db: Some(&mut sections_right),
        },
    )
    .expect("valid query");
    let delta = bench_alloc::current_thread_delta_since(mark);
    assert_eq!(delta.allocations, 0, "allocation delta={delta:?}");
    assert_eq!(delta.deallocations, 0, "free delta={delta:?}");
    assert_eq!(delta.reallocations, 0, "reallocation delta={delta:?}");
}

#[test]
fn preparation_rejects_invalid_boundaries_rates_and_excluded_fields() {
    let maximum = builtin_filter_cutoff_maximum_hz(48_000).expect("maximum");
    let successor = f32::from_bits(maximum.to_bits() + 1);
    let mut exact = filters(maximum, 0.0, 0.0, 0.0);
    let frequencies = [0.0_f32, 1_000.0, 24_000.0];
    query_with_sections(48_000, exact, &frequencies, false);

    exact.left.hpf_hz = successor;
    let mut total_left = [71.0_f32; 3];
    let mut total_right = [72.0_f32; 3];
    let result = query_input_filter_response_into(
        InputFilterResponseRequest {
            configuration_id: 1,
            sample_rate_hz: 48_000,
            configuration: exact,
            frequencies_hz: &frequencies,
            maximum_points: frequencies.len(),
        },
        InputFilterResponseOutput {
            total_left_db: &mut total_left,
            total_right_db: &mut total_right,
            sections_left_db: None,
            sections_right_db: None,
        },
    );
    assert_eq!(
        result,
        Err(InputFilterResponseError::Configuration(
            BuiltinParameterError::FilterCutoff
        ))
    );
    assert_eq!(total_left, [71.0; 3]);
    assert_eq!(total_right, [72.0; 3]);

    let mut cases = Vec::new();
    let mut negative_zero = BuiltinParameters::default();
    negative_zero.left.hpf_hz = -0.0;
    cases.push((negative_zero, BuiltinParameterError::FilterCutoff));
    let mut subminimum = BuiltinParameters::default();
    subminimum.left.hpf_hz = f32::from_bits(10.0_f32.to_bits() - 1);
    cases.push((subminimum, BuiltinParameterError::FilterCutoff));
    let mut nan = BuiltinParameters::default();
    nan.left.lpf_hz = f32::NAN;
    cases.push((nan, BuiltinParameterError::FilterCutoff));
    let mut infinity = BuiltinParameters::default();
    infinity.right.hpf_hz = f32::INFINITY;
    cases.push((infinity, BuiltinParameterError::FilterCutoff));
    let mut negative = BuiltinParameters::default();
    negative.right.lpf_hz = -1.0;
    cases.push((negative, BuiltinParameterError::FilterCutoff));
    let mut order = filters(2_000.0, 1_000.0, 0.0, 0.0);
    cases.push((order, BuiltinParameterError::FilterOrder));
    let mut gain = BuiltinParameters::default();
    gain.left.trim_db = f32::NAN;
    cases.push((gain, BuiltinParameterError::GainDomain));
    let mut matrix = BuiltinParameters::default();
    matrix.matrix.ll = f32::NAN;
    cases.push((matrix, BuiltinParameterError::MatrixCoefficient));
    for (configuration, error) in cases {
        let result = query_input_filter_response_into(
            InputFilterResponseRequest {
                configuration_id: 2,
                sample_rate_hz: 48_000,
                configuration,
                frequencies_hz: &frequencies,
                maximum_points: frequencies.len(),
            },
            InputFilterResponseOutput {
                total_left_db: &mut [1.0; 3],
                total_right_db: &mut [2.0; 3],
                sections_left_db: None,
                sections_right_db: None,
            },
        );
        assert_eq!(result, Err(InputFilterResponseError::Configuration(error)));
    }
    order.left.hpf_hz = 1_000.0;
    order.left.lpf_hz = 2_000.0;
    assert!(
        query_with_sections(48_000, order, &frequencies, false)
            .0
            .points
            == 3
    );

    for rate in [0, 44_101, 176_400, 192_000, 352_800, 384_000] {
        let result = query_input_filter_response_into(
            InputFilterResponseRequest {
                configuration_id: 3,
                sample_rate_hz: rate,
                configuration: BuiltinParameters::default(),
                frequencies_hz: &frequencies,
                maximum_points: frequencies.len(),
            },
            InputFilterResponseOutput {
                total_left_db: &mut [3.0; 3],
                total_right_db: &mut [4.0; 3],
                sections_left_db: None,
                sections_right_db: None,
            },
        );
        assert_eq!(result, Err(InputFilterResponseError::UnsupportedSampleRate));
    }
}

type InputSnapshot = ([[u32; 7]; 4], [u32; 2], [u32; 8], [u32; 8], [[bool; 2]; 2]);

fn input_snapshot(input: &InputBuiltins) -> InputSnapshot {
    (
        test_support::input_section_words(input),
        test_support::input_trim_words(input),
        test_support::input_state_words(input),
        test_support::input_trim_ramp_words(input),
        test_support::input_elision_plan(input),
    )
}

fn bits(values: &[f32]) -> Vec<u32> {
    values.iter().map(|value| value.to_bits()).collect()
}

#[test]
fn query_does_not_change_seeded_state_or_an_inflight_trim_polarity_ramp() {
    let configuration = filters(100.0, 4_000.0, 1_000.0, 8_000.0);
    let mut queried = BuiltinChain::new(48_000, configuration)
        .expect("queried chain")
        .into_input_builtins();
    let mut twin = BuiltinChain::new(48_000, configuration)
        .expect("twin chain")
        .into_input_builtins();
    let mut prefix_left = [0.25_f32; 32];
    let mut prefix_right = [-0.375_f32; 32];
    let mut twin_prefix_left = prefix_left;
    let mut twin_prefix_right = prefix_right;
    queried.process(DualMonoBlock::new(&mut prefix_left, &mut prefix_right, 0).expect("prefix"));
    twin.process(
        DualMonoBlock::new(&mut twin_prefix_left, &mut twin_prefix_right, 0).expect("twin prefix"),
    );
    queried
        .set_trim_db(BuiltinLaneSelector::Both, -9.0, 64)
        .expect("trim ramp");
    queried.set_polarity_invert(BuiltinLaneSelector::Both, true, 64);
    twin.set_trim_db(BuiltinLaneSelector::Both, -9.0, 64)
        .expect("twin trim ramp");
    twin.set_polarity_invert(BuiltinLaneSelector::Both, true, 64);
    let mut ramp_left = [0.125_f32; 16];
    let mut ramp_right = [-0.25_f32; 16];
    let mut twin_ramp_left = ramp_left;
    let mut twin_ramp_right = ramp_right;
    queried.process(
        DualMonoBlock::new(&mut ramp_left, &mut ramp_right, 32).expect("queried ramp block"),
    );
    twin.process(
        DualMonoBlock::new(&mut twin_ramp_left, &mut twin_ramp_right, 32).expect("twin ramp block"),
    );
    let before = input_snapshot(&queried);
    assert!(before.3[6] > 0 && before.3[7] > 0);
    assert_eq!(before, input_snapshot(&twin));

    let frequencies = [0.0_f32, 100.0, 1_000.0, 4_000.0, 24_000.0];
    let _ = query_with_sections(48_000, configuration, &frequencies, true);
    assert_eq!(before, input_snapshot(&queried));

    let mut left = [0.375_f32; 128];
    let mut right = [-0.5_f32; 128];
    let mut twin_left = left;
    let mut twin_right = right;
    let queried_report = queried
        .process(DualMonoBlock::new(&mut left, &mut right, 48).expect("continued queried block"));
    let twin_report = twin.process(
        DualMonoBlock::new(&mut twin_left, &mut twin_right, 48).expect("continued twin block"),
    );
    assert_eq!(queried_report, twin_report);
    assert_eq!(bits(&left), bits(&twin_left));
    assert_eq!(bits(&right), bits(&twin_right));
    assert_eq!(input_snapshot(&queried), input_snapshot(&twin));
}
