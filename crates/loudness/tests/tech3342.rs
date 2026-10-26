//! EBU Tech 3342 (2023) Table 1, the loudness-range cases defined by recipe, at every launch rate.
//!
//! The production meter and the `dsp-reference` oracle (Tech 3342's MATLAB function, exactly) run
//! on the same `f32` samples and are both held to the case's expectation, +-1 LU. Cases 5 and 6
//! are authentic programme files: a file-based successor's work.

mod common;

use common::{assert_within, planes, run, trace};
use dsp_reference::loudness::reference_tone_sequence;
use loudness::{LoudnessAnalysis, SUPPORTED_RATES};

const TOLERANCE: f64 = 1.0;

#[test]
fn tech_3342_recipe_cases_at_every_launch_rate() {
    let cases: [(u32, &[f64], f64); 4] = [
        (1, &[-20.0, -30.0], 10.0),
        (2, &[-20.0, -15.0], 5.0),
        (3, &[-40.0, -20.0], 20.0),
        (4, &[-50.0, -35.0, -20.0, -35.0, -50.0], 15.0),
    ];
    for rate in SUPPORTED_RATES {
        for (case, levels, expected) in cases {
            let segments: Vec<_> = levels.iter().map(|db| (20.0, Some(*db))).collect();
            let (plane, widened) = planes(&reference_tone_sequence(rate, &segments));
            let mut analysis = LoudnessAnalysis::new();
            run(rate, &plane, &mut analysis, |_, _| {});
            let oracle = trace(rate, &widened);
            let what = format!("case {case} at {rate} Hz: LRA");
            assert_within(&what, analysis.loudness_range(), expected, TOLERANCE);
            assert_within(
                &format!("oracle {what}"),
                oracle.loudness_range(0, oracle.segments()),
                expected,
                TOLERANCE,
            );
        }
    }
}
