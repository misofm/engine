//! #1572: the response preview is one core function, `prepare_response_query`.
//!
//! * **Assembly.** For a parametric EQ with different left and right gains and one disabled band,
//!   and for input filters with different left and right cutoffs, at every launch rate and for
//!   every channel and section choice, `curves()` has the documented `Some`/`None` pattern, the
//!   documented lengths and enabled masks, and totals bit-equal to the stopped preview.
//! * **Bounds.** 4096 points, 256 overrides and the four launch rates pass; one more point or
//!   override, and a rate outside the launch set, are refused before any owner is prepared.

use bench_support::alloc as bench_alloc;
use effect_compiler::launch_native_effect_registry;
use effect_contract::{EffectQuality, LinkMode, ParameterChannel, ParameterChannelPolicy};
use host_core::{
    PreparedResponseQuery, RESPONSE_MAXIMUM_OVERRIDES, RESPONSE_MAXIMUM_POINTS, ResponseChannels,
    ResponseParameterOverride, ResponsePreviewError, ResponsePreviewGrid, ResponsePreviewLimits,
    ResponsePreviewOutput, ResponsePreviewRequest, ResponsePreviewTarget, ResponseQuery,
    ResponseQueryError, prepare_response_preview, prepare_response_query,
};

const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const EQ: &str = "miso.parametric-eq";
const POINTS: usize = 64;

fn grid(points: usize) -> ResponsePreviewGrid {
    ResponsePreviewGrid::Logarithmic {
        points,
        minimum_hz: 20.0,
        maximum_hz: 20_000.0,
    }
}

fn parameter_id(name: &str) -> (u32, ParameterChannelPolicy) {
    let registry = launch_native_effect_registry().expect("registry");
    let descriptor = registry.get_ascii(EQ).expect("EQ").descriptor();
    let parameter = descriptor
        .parameters
        .iter()
        .find(|parameter| parameter.name == name)
        .unwrap_or_else(|| panic!("EQ parameter {name}"));
    (parameter.id.0, parameter.channel_policy)
}

/// One override row per channel the parameter takes, or the one shared row.
fn rows(name: &str, left: f32, right: f32) -> Vec<ResponseParameterOverride> {
    let (parameter_id, policy) = parameter_id(name);
    match policy {
        ParameterChannelPolicy::Shared => vec![ResponseParameterOverride {
            parameter_id,
            channel: ParameterChannel::Both,
            value: left,
        }],
        ParameterChannelPolicy::PerLane => vec![
            ResponseParameterOverride {
                parameter_id,
                channel: ParameterChannel::Left,
                value: left,
            },
            ResponseParameterOverride {
                parameter_id,
                channel: ParameterChannel::Right,
                value: right,
            },
        ],
    }
}

fn eq_overrides() -> Vec<ResponseParameterOverride> {
    let mut rows_out = Vec::new();
    // Different left and right gains on two bands; band 3 is the disabled one.
    for (band, left, right) in [(1, 6.0, -4.0), (2, -3.0, 5.0), (3, 7.0, 7.0)] {
        rows_out.extend(rows(&format!("band_{band}_enabled"), 1.0, 1.0));
        rows_out.extend(rows(&format!("band_{band}_gain_db"), left, right));
    }
    // Disable band 3 on both lanes.
    let disabled = rows("band_3_enabled", 0.0, 0.0);
    rows_out.retain(|row| !disabled.iter().any(|d| d.parameter_id == row.parameter_id));
    rows_out.extend(disabled);
    rows_out
}

fn query<'a>(
    rate: u32,
    target: ResponsePreviewTarget<'a>,
    channels: ResponseChannels,
    sections: bool,
    points: usize,
) -> ResponseQuery<'a> {
    ResponseQuery {
        configuration_id: 0x5eed_1572,
        sample_rate_hz: rate,
        quantum_frames: 128,
        target,
        grid: grid(points),
        channels,
        sections,
        limits: ResponsePreviewLimits::default(),
    }
}

fn effect<'a>(overrides: &'a [ResponseParameterOverride]) -> ResponsePreviewTarget<'a> {
    ResponsePreviewTarget::Effect {
        effect_id: EQ,
        overrides,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
    }
}

fn input_filters() -> ResponsePreviewTarget<'static> {
    ResponsePreviewTarget::InputFilters {
        left_hpf_hz: 80.0,
        left_lpf_hz: 12_000.0,
        right_hpf_hz: 160.0,
        right_lpf_hz: 6_000.0,
    }
}

fn bits(values: &[f32]) -> Vec<u32> {
    values.iter().map(|value| value.to_bits()).collect()
}

/// Test value: the core drops or swaps a side, folds the enabled sections into the wrong bits,
/// or returns the sections when they were not asked for, so a host would draw the wrong curve.
fn check_assembly(rate: u32, target: ResponsePreviewTarget<'_>, expect_disabled: bool) {
    let reference = prepare_response_preview(ResponsePreviewRequest {
        configuration_id: 0x5eed_1572,
        sample_rate_hz: rate,
        quantum_frames: 128,
        target,
        limits: ResponsePreviewLimits::default(),
    })
    .expect("reference");
    let section_count = reference.descriptor().sections.len();
    let mut frequencies = vec![0.0; POINTS];
    let mut left = vec![0.0; POINTS];
    let mut right = vec![0.0; POINTS];
    let mut sections_left = vec![0.0; section_count * POINTS];
    let mut sections_right = vec![0.0; section_count * POINTS];
    reference
        .query_into(
            grid(POINTS),
            ResponsePreviewOutput {
                frequencies_hz: &mut frequencies,
                total_left_db: &mut left,
                total_right_db: &mut right,
                sections_left_db: Some(&mut sections_left),
                sections_right_db: Some(&mut sections_right),
            },
        )
        .expect("reference query");
    assert_ne!(left, right, "the fixture must differ between sides");
    let configuration = reference.configuration();
    let expected_mask = |enabled: &[bool]| {
        enabled.iter().enumerate().fold(0_u32, |mask, (index, on)| {
            mask | (u32::from(*on) << index.min(31))
        })
    };
    let (mask_left, mask_right) = (
        expected_mask(configuration.enabled_left),
        expected_mask(configuration.enabled_right),
    );
    if expect_disabled {
        let all = (1_u32 << section_count.min(31)) - 1;
        assert_ne!(mask_left, all, "one band must be disabled");
        assert!(mask_left != 0, "an enabled band must remain");
    }

    for channels in [
        ResponseChannels::Left,
        ResponseChannels::Right,
        ResponseChannels::Both,
    ] {
        for sections in [false, true] {
            let prepared = prepare_response_query(&query(rate, target, channels, sections, POINTS))
                .expect("prepare");
            assert_eq!(prepared.points(), POINTS);
            assert_eq!(prepared.section_count(), section_count);
            let curves = prepared.curves().expect("curves");
            let want_left = channels != ResponseChannels::Right;
            let want_right = channels != ResponseChannels::Left;
            assert_eq!(curves.configuration_id, 0x5eed_1572);
            assert_eq!(curves.sample_rate_hz, rate);
            assert_eq!(curves.section_count as usize, section_count);
            assert_eq!(curves.enabled_left, mask_left);
            assert_eq!(curves.enabled_right, mask_right);
            assert_eq!(curves.bypass, configuration.bypass.unwrap_or(false));
            assert_eq!(bits(&curves.frequencies_hz), bits(&frequencies));
            assert_eq!(curves.left_db.is_some(), want_left);
            assert_eq!(curves.right_db.is_some(), want_right);
            assert_eq!(curves.sections_left_db.is_some(), sections && want_left);
            assert_eq!(curves.sections_right_db.is_some(), sections && want_right);
            if let Some(values) = &curves.left_db {
                assert_eq!(bits(values), bits(&left));
            }
            if let Some(values) = &curves.right_db {
                assert_eq!(bits(values), bits(&right));
            }
            if let Some(values) = &curves.sections_left_db {
                assert_eq!(values.len(), section_count * POINTS);
                assert_eq!(bits(values), bits(&sections_left));
            }
            if let Some(values) = &curves.sections_right_db {
                assert_eq!(values.len(), section_count * POINTS);
                assert_eq!(bits(values), bits(&sections_right));
            }
        }
    }
}

#[test]
fn parametric_eq_curves_match_the_stopped_preview_at_every_rate() {
    let overrides = eq_overrides();
    for rate in RATES {
        check_assembly(rate, effect(&overrides), true);
    }
}

#[test]
fn input_filter_curves_match_the_stopped_preview_at_every_rate() {
    for rate in RATES {
        check_assembly(rate, input_filters(), false);
    }
}

/// Test value: a bound that is missing, off by one, or checked after the owner is prepared.
#[test]
fn bounds_pass_at_the_limit_and_refuse_one_past_it() {
    let overrides = eq_overrides();
    let target = effect(&overrides);
    let both = ResponseChannels::Both;
    for rate in RATES {
        prepare_response_query(&query(rate, target, both, false, RESPONSE_MAXIMUM_POINTS))
            .expect("4096 points")
            .curves()
            .expect("4096 point curves");
    }
    assert_eq!(RESPONSE_MAXIMUM_POINTS, 4096);
    assert_eq!(RESPONSE_MAXIMUM_OVERRIDES, 256);

    // 256 distinct-or-not rows: the cap is on the count, and the duplicate rule is the preview's.
    let at_cap: Vec<_> = overrides
        .iter()
        .copied()
        .cycle()
        .take(RESPONSE_MAXIMUM_OVERRIDES)
        .collect();
    let past_cap: Vec<_> = overrides
        .iter()
        .copied()
        .cycle()
        .take(RESPONSE_MAXIMUM_OVERRIDES + 1)
        .collect();
    assert!(matches!(
        prepare_response_query(&query(48_000, effect(&at_cap), both, false, POINTS)).err(),
        Some(ResponseQueryError::Preview(
            ResponsePreviewError::DuplicateParameter
        ))
    ));

    // Warm every process static the refused calls could touch, then count.
    bench_alloc::assert_installed();
    let _ = prepare_response_query(&query(48_000, target, both, false, POINTS)).expect("warm");
    let refused = |query: ResponseQuery<'_>| -> Option<ResponseQueryError> {
        let mark = bench_alloc::current_thread_counters();
        let result = prepare_response_query(&query);
        let delta = bench_alloc::current_thread_delta_since(mark);
        assert_eq!(
            (delta.allocations, delta.deallocations),
            (0, 0),
            "a refused query prepared an owner"
        );
        result.err()
    };
    let too_many = refused(query(48_000, effect(&past_cap), both, false, POINTS));
    assert_eq!(too_many, Some(ResponseQueryError::Overrides));
    assert_eq!(
        refused(query(
            48_000,
            target,
            both,
            false,
            RESPONSE_MAXIMUM_POINTS + 1
        )),
        Some(ResponseQueryError::Points)
    );
    assert_eq!(
        refused(query(48_000, target, both, false, 1)),
        Some(ResponseQueryError::Points)
    );
    for rate in [192_000, 22_050, 0] {
        assert_eq!(
            refused(query(rate, target, both, false, POINTS)),
            Some(ResponseQueryError::SampleRate),
            "rate {rate}"
        );
    }
    // The input-filter target carries no overrides, so the override cap does not apply to it.
    let _: PreparedResponseQuery =
        prepare_response_query(&query(48_000, input_filters(), both, true, POINTS)).expect("ok");
}

fn curves_of(overrides: &[ResponseParameterOverride]) -> host_core::ResponseCurves {
    prepare_response_query(&query(
        48_000,
        effect(overrides),
        ResponseChannels::Both,
        true,
        POINTS,
    ))
    .expect("prepared")
    .curves()
    .expect("curves")
}

fn curve_bits(curves: &host_core::ResponseCurves) -> Vec<Vec<u32>> {
    [
        Some(&curves.frequencies_hz),
        curves.left_db.as_ref(),
        curves.right_db.as_ref(),
        curves.sections_left_db.as_ref(),
        curves.sections_right_db.as_ref(),
    ]
    .into_iter()
    .flatten()
    .map(|values| bits(values))
    .collect()
}

/// Decision 17's lane rule: `both` on a per-lane parameter sets both lanes, so it gives the same
/// curves as a `left` row and a `right` row of the same value, and a `both` beside a `left` or a
/// `right` of the same parameter (in either order), or a lane given twice, is a duplicate.
///
/// Red if `both` on a per-lane parameter is refused (the lane rule before decision 17), sets one
/// lane only, or lets two rows write the same lane.
#[test]
fn both_on_a_per_lane_parameter_is_a_left_and_a_right_row() {
    let (gain, policy) = parameter_id("band_1_gain_db");
    assert_eq!(policy, ParameterChannelPolicy::PerLane);
    let (enabled, _) = parameter_id("band_1_enabled");
    let row = |parameter_id, channel, value| ResponseParameterOverride {
        parameter_id,
        channel,
        value,
    };
    let both = [
        row(enabled, ParameterChannel::Both, 1.0),
        row(gain, ParameterChannel::Both, 9.0),
    ];
    let pair = [
        row(enabled, ParameterChannel::Left, 1.0),
        row(enabled, ParameterChannel::Right, 1.0),
        row(gain, ParameterChannel::Left, 9.0),
        row(gain, ParameterChannel::Right, 9.0),
    ];
    let flat = curves_of(&[]);
    let from_both = curves_of(&both);
    assert_eq!(curve_bits(&from_both), curve_bits(&curves_of(&pair)));
    assert_ne!(
        curve_bits(&from_both),
        curve_bits(&flat),
        "the override took effect"
    );
    // One lane only changes one side.
    let left_only = curves_of(&[
        row(enabled, ParameterChannel::Both, 1.0),
        row(gain, ParameterChannel::Left, 9.0),
    ]);
    assert_eq!(
        bits(left_only.left_db.as_ref().unwrap()),
        bits(from_both.left_db.as_ref().unwrap())
    );
    assert_ne!(
        bits(left_only.right_db.as_ref().unwrap()),
        bits(from_both.right_db.as_ref().unwrap())
    );

    let refused = |overrides: &[ResponseParameterOverride]| {
        prepare_response_query(&query(
            48_000,
            effect(overrides),
            ResponseChannels::Both,
            false,
            POINTS,
        ))
        .err()
    };
    for overrides in [
        vec![
            row(gain, ParameterChannel::Both, 1.0),
            row(gain, ParameterChannel::Left, 2.0),
        ],
        vec![
            row(gain, ParameterChannel::Right, 1.0),
            row(gain, ParameterChannel::Both, 2.0),
        ],
        vec![
            row(gain, ParameterChannel::Left, 1.0),
            row(gain, ParameterChannel::Left, 2.0),
        ],
        vec![
            row(gain, ParameterChannel::Both, 1.0),
            row(gain, ParameterChannel::Both, 2.0),
        ],
    ] {
        assert_eq!(
            refused(&overrides),
            Some(ResponseQueryError::Preview(
                ResponsePreviewError::DuplicateParameter
            )),
            "{overrides:?}"
        );
    }
    assert_eq!(
        refused(&[row(u32::MAX, ParameterChannel::Both, 1.0)]),
        Some(ResponseQueryError::Preview(
            ResponsePreviewError::InvalidParameter
        ))
    );
}

/// The input-filter rows are the builtin `hpf_hz` and `lpf_hz`: an absent lane keeps the row's
/// default, `both` sets both lanes, a `left` and a `right` row are different cutoffs, and any
/// other builtin parameter, or a repeat, is refused.
///
/// Red if a lane is dropped or swapped, an absent lane stops taking the default, or a parameter
/// that is not an input filter is admitted.
#[test]
fn input_filter_rows_fold_into_four_cutoffs_by_the_lane_rule() {
    let row = |parameter_id, channel, value| ResponseParameterOverride {
        parameter_id,
        channel,
        value,
    };
    let target = host_core::input_filter_target(&[
        row(3, ParameterChannel::Both, 80.0),
        row(4, ParameterChannel::Left, 12_000.0),
    ])
    .expect("target");
    let ResponsePreviewTarget::InputFilters {
        left_hpf_hz,
        left_lpf_hz,
        right_hpf_hz,
        right_lpf_hz,
    } = target
    else {
        panic!("input filters");
    };
    assert_eq!(
        (left_hpf_hz, left_lpf_hz, right_hpf_hz, right_lpf_hz),
        (80.0, 12_000.0, 80.0, 0.0)
    );
    for (rows, expected) in [
        (
            vec![row(2, ParameterChannel::Both, 0.0)],
            ResponsePreviewError::InvalidParameter,
        ),
        (
            vec![row(0, ParameterChannel::Both, 0.0)],
            ResponsePreviewError::InvalidParameter,
        ),
        (
            vec![
                row(3, ParameterChannel::Left, 80.0),
                row(3, ParameterChannel::Both, 90.0),
            ],
            ResponsePreviewError::DuplicateParameter,
        ),
    ] {
        assert!(
            matches!(host_core::input_filter_target(&rows), Err(error) if error == expected),
            "{rows:?}"
        );
    }
}

/// The one refusal table: every error maps to its class and code, and every code is a dotted code
/// a diagnostic can carry.
///
/// Red if an error maps to another class or code than decision 17 rules, or a code is not
/// encodable as a diagnostic.
#[test]
fn every_query_error_has_its_class_and_code() {
    use ResponsePreviewError as Preview;
    use effect_contract::{EffectPrepareError, ResponseAnalysisError as Owner};
    use host_core::ResponseRefusal::{
        Internal, InvalidField, LimitExceeded, NotFound, Unavailable, ValidationFailed,
    };
    let preview = ResponseQueryError::Preview;
    let table = [
        (ResponseQueryError::Points, InvalidField, "response.points"),
        (
            ResponseQueryError::Overrides,
            LimitExceeded,
            "response.override.count",
        ),
        (
            ResponseQueryError::SampleRate,
            InvalidField,
            "response.rate",
        ),
        (
            preview(Preview::InvalidShape),
            InvalidField,
            "response.rate",
        ),
        (preview(Preview::InvalidGrid), InvalidField, "response.grid"),
        (
            preview(Preview::Owner(Owner::InvalidFrequencyGrid)),
            InvalidField,
            "response.grid",
        ),
        (
            preview(Preview::UnknownEffect),
            NotFound,
            "response.effect.unknown",
        ),
        (
            preview(Preview::UnsupportedEffect),
            Unavailable,
            "response.effect.unsupported",
        ),
        (
            preview(Preview::Owner(Owner::UnsupportedMode)),
            Unavailable,
            "response.effect.unsupported",
        ),
        (
            preview(Preview::Owner(Owner::UnsupportedCapability)),
            Unavailable,
            "response.effect.unsupported",
        ),
        (
            preview(Preview::InvalidParameter),
            InvalidField,
            "response.override.control",
        ),
        (
            preview(Preview::ConflictingChannel),
            InvalidField,
            "response.override.lane",
        ),
        (
            preview(Preview::DuplicateParameter),
            InvalidField,
            "response.override.duplicate",
        ),
        (
            preview(Preview::Owner(Owner::Configuration(EffectPrepareError {
                code: "effect.quality.unsupported",
            }))),
            ValidationFailed,
            "effect.quality.unsupported",
        ),
        (
            preview(Preview::Owner(Owner::Capacity)),
            LimitExceeded,
            "response.limit",
        ),
        (
            preview(Preview::Owner(Owner::ResourceLimit)),
            LimitExceeded,
            "response.limit",
        ),
        (
            preview(Preview::Owner(Owner::Numerical)),
            Internal,
            "response.numerical",
        ),
        (preview(Preview::OutputShape), Internal, "response.internal"),
        (
            preview(Preview::Owner(Owner::OutputShape)),
            Internal,
            "response.internal",
        ),
    ];
    for (error, class, code) in table {
        assert_eq!(error.refusal(), (class, code), "{error:?}");
        assert!(
            code.split('.').all(|part| !part.is_empty()
                && part.bytes().all(|byte| byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || byte == b'_')),
            "{code}"
        );
    }
}
