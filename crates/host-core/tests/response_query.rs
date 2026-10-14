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
