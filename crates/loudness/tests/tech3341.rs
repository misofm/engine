//! EBU Tech 3341 (2023) Table 1, the loudness cases defined by recipe, at every launch rate.
//!
//! Each case runs the production meter and the `dsp-reference` oracle on the same `f32` samples
//! and holds both to the case's expectation (+-0.1 LU): a wrong oracle fails here as surely as a
//! wrong meter.
//!
//! Not here: case 6 is a 5.0-channel signal and the engine measures a stereo pair; cases 7 and 8
//! are authentic programme files, a file-based successor's work; cases 15-23 are the true-peak
//! cases (`tests/true_peak.rs`).

mod common;

use common::{assert_within, planes, run, trace};
use dsp_reference::loudness::reference_tone_sequence;
use loudness::{LoudnessAnalysis, SUPPORTED_RATES};

const TOLERANCE: f64 = 0.1;

type Segments = Vec<(f64, Option<f64>)>;

fn repeat(pattern: &[(f64, Option<f64>)], times: usize) -> Segments {
    pattern
        .iter()
        .copied()
        .cycle()
        .take(pattern.len() * times)
        .collect()
}

/// Cases 1-5: M, S and I (cases 1 and 2) or I alone at the end of the signal.
fn end_of_signal(rate: u32, case: u32, segments: &[(f64, Option<f64>)], all: bool) {
    let (plane, widened) = planes(&reference_tone_sequence(rate, segments));
    let mut analysis = LoudnessAnalysis::new();
    run(rate, &plane, &mut analysis, |_, _| {});
    let oracle = trace(rate, &widened);
    let end = oracle.segments();
    let name = |what: &str| format!("case {case} at {rate} Hz: {what}");
    // Cases 1 and 2 are one tone, read at its own level; cases 3-5 integrate to -23 LUFS.
    let expected = if all {
        segments[0].1.expect("tone")
    } else {
        -23.0
    };
    assert_within(&name("I"), analysis.integrated(), expected, TOLERANCE);
    assert_within(
        &name("oracle I"),
        oracle.integrated(0, end),
        expected,
        TOLERANCE,
    );
    if all {
        for (what, value) in [
            ("M", analysis.momentary()),
            ("S", analysis.short_term()),
            ("oracle M", oracle.momentary(end)),
            ("oracle S", oracle.short_term(end)),
        ] {
            assert_within(&name(what), value, expected, TOLERANCE);
        }
    }
}

/// Cases 9 and 12: every M (or S) reading from `settle` seconds on.
fn constant_after(rate: u32, case: u32, segments: &[(f64, Option<f64>)], short: bool, settle: f64) {
    let (plane, widened) = planes(&reference_tone_sequence(rate, segments));
    let oracle = trace(rate, &widened);
    let settle = (settle * f64::from(rate)).round() as u64;
    let hop = u64::from(rate / 100);
    let mut analysis = LoudnessAnalysis::new();
    let mut readings = 0;
    run(rate, &plane, &mut analysis, |record, analysis| {
        if record.end_sample < settle {
            return;
        }
        let end = (record.end_sample / hop) as usize;
        let (value, reference) = if short {
            (analysis.short_term(), oracle.short_term(end))
        } else {
            (analysis.momentary(), oracle.momentary(end))
        };
        let what = format!("case {case} at {rate} Hz, hop ending {}", record.end_sample);
        assert_within(&what, value, -23.0, TOLERANCE);
        assert_within(&format!("oracle {what}"), reference, -23.0, TOLERANCE);
        readings += 1;
    });
    assert!(readings > 0);
}

/// Cases 10 and 13 (file-based): reset at each segment's start, Max S (or M) at its end.
fn reset_per_segment(rate: u32, case: u32, segments: &[Segments], short: bool) {
    let all: Segments = segments.concat();
    let (plane, widened) = planes(&reference_tone_sequence(rate, &all));
    let oracle = trace(rate, &widened);
    let hop = u64::from(rate / 100);
    let window = if short { 300 } else { 40 };
    let bounds = spans(rate, segments);
    let mut analysis = LoudnessAnalysis::new();
    let mut segment = 0;
    run(rate, &plane, &mut analysis, |record, analysis| {
        let Some(&(start, end)) = bounds.get(segment) else {
            return;
        };
        if record.end_sample == start {
            analysis.reset(start);
        }
        if record.end_sample == end {
            let value = if short {
                analysis.short_term_max()
            } else {
                analysis.momentary_max()
            };
            let what = format!("case {case} at {rate} Hz, segment {segment}");
            assert_within(&what, value, -23.0, TOLERANCE);
            let first = (start / hop) as usize + window;
            let last = (end / hop) as usize;
            let reference = (first..=last)
                .map(|e| oracle.window_mean_square(e, window).expect("window"))
                .fold(0.0, f64::max);
            let reference = dsp_reference::loudness::reference_loudness_level(reference);
            assert_within(&format!("oracle {what}"), Some(reference), -23.0, TOLERANCE);
            segment += 1;
        }
    });
    assert_eq!(
        segment,
        segments.len(),
        "case {case} at {rate} Hz: segments read"
    );
}

/// Cases 11 and 14 (live): one run; the largest S (or M) reading in each segment's span.
fn live_per_segment(rate: u32, case: u32, segments: &[Segments], short: bool) {
    let all: Segments = segments.concat();
    let (plane, widened) = planes(&reference_tone_sequence(rate, &all));
    let oracle = trace(rate, &widened);
    let hop = u64::from(rate / 100);
    let bounds = spans(rate, segments);
    let mut analysis = LoudnessAnalysis::new();
    let mut maxima = vec![(f64::NEG_INFINITY, f64::NEG_INFINITY); segments.len()];
    run(rate, &plane, &mut analysis, |record, analysis| {
        let Some(segment) = bounds
            .iter()
            .position(|(start, end)| record.end_sample > *start && record.end_sample <= *end)
        else {
            return;
        };
        let end = (record.end_sample / hop) as usize;
        let (value, reference) = if short {
            (analysis.short_term(), oracle.short_term(end))
        } else {
            (analysis.momentary(), oracle.momentary(end))
        };
        let slot = &mut maxima[segment];
        slot.0 = slot.0.max(value.unwrap_or(f64::NEG_INFINITY));
        slot.1 = slot.1.max(reference.unwrap_or(f64::NEG_INFINITY));
    });
    for (i, (value, reference)) in maxima.into_iter().enumerate() {
        let expected = -38.0 + i as f64;
        let what = format!("case {case} at {rate} Hz, segment {i}");
        assert_within(&what, Some(value), expected, TOLERANCE);
        assert_within(
            &format!("oracle {what}"),
            Some(reference),
            expected,
            TOLERANCE,
        );
    }
}

/// `[start, end)` samples of each segment.
fn spans(rate: u32, segments: &[Segments]) -> Vec<(u64, u64)> {
    let frames = |seconds: f64| (seconds * f64::from(rate)).round() as u64;
    let mut start = 0;
    segments
        .iter()
        .map(|segment| {
            let length: u64 = segment.iter().map(|(seconds, _)| frames(*seconds)).sum();
            let span = (start, start + length);
            start += length;
            span
        })
        .collect()
}

#[test]
fn tech_3341_recipe_cases_at_every_launch_rate() {
    let s = Some;
    for rate in SUPPORTED_RATES {
        end_of_signal(rate, 1, &[(20.0, s(-23.0))], true);
        end_of_signal(rate, 2, &[(20.0, s(-33.0))], true);
        end_of_signal(
            rate,
            3,
            &[(10.0, s(-36.0)), (60.0, s(-23.0)), (10.0, s(-36.0))],
            false,
        );
        end_of_signal(
            rate,
            4,
            &[
                (10.0, s(-72.0)),
                (10.0, s(-36.0)),
                (60.0, s(-23.0)),
                (10.0, s(-36.0)),
                (10.0, s(-72.0)),
            ],
            false,
        );
        end_of_signal(
            rate,
            5,
            &[(20.0, s(-26.0)), (20.1, s(-20.0)), (20.0, s(-26.0))],
            false,
        );
        constant_after(
            rate,
            9,
            &repeat(&[(1.34, s(-20.0)), (1.66, s(-30.0))], 5),
            true,
            3.0,
        );
        reset_per_segment(
            rate,
            10,
            &(0..20)
                .map(|i| vec![(0.15 * f64::from(i), None), (3.0, s(-23.0)), (1.0, None)])
                .collect::<Vec<_>>(),
            true,
        );
        live_per_segment(
            rate,
            11,
            &(0..20)
                .map(|i| {
                    let i = f64::from(i);
                    vec![
                        (0.15 * i, None),
                        (3.0, s(-38.0 + i)),
                        (3.0 - 0.15 * i, None),
                    ]
                })
                .collect::<Vec<_>>(),
            true,
        );
        constant_after(
            rate,
            12,
            &repeat(&[(0.18, s(-20.0)), (0.22, s(-30.0))], 25),
            false,
            1.0,
        );
        reset_per_segment(
            rate,
            13,
            &(0..20)
                .map(|i| vec![(0.02 * f64::from(i), None), (0.4, s(-23.0)), (1.0, None)])
                .collect::<Vec<_>>(),
            false,
        );
        live_per_segment(
            rate,
            14,
            &(0..20)
                .map(|i| {
                    let i = f64::from(i);
                    vec![
                        (0.02 * i, None),
                        (0.4, s(-38.0 + i)),
                        (0.4 - 0.02 * i, None),
                    ]
                })
                .collect::<Vec<_>>(),
            false,
        );
    }
}
