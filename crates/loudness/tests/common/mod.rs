//! Shared drivers for the loudness conformance tests: production and oracle on the same samples.

#![allow(dead_code, unreachable_pub)]

use dsp_reference::loudness::ReferenceLoudnessTrace;
use loudness::{HopRecord, LoudnessAnalysis, LoudnessObserver};

/// Frames per rendered block the tests feed the observer.
pub const BLOCK: usize = 512;

/// The `f32` plane the engine would render, and the same samples widened back for the oracle.
pub fn planes(signal: &[f64]) -> (Vec<f32>, Vec<f64>) {
    let plane: Vec<f32> = signal.iter().map(|x| *x as f32).collect();
    let widened = plane.iter().map(|x| f64::from(*x)).collect();
    (plane, widened)
}

/// Runs `plane` on both channels through a fresh observer at `rate` from sample 0, in blocks of
/// [`BLOCK`], ingesting every record into `analysis` and then calling `each` with it.
pub fn run(
    rate: u32,
    plane: &[f32],
    analysis: &mut LoudnessAnalysis,
    mut each: impl FnMut(&HopRecord, &mut LoudnessAnalysis),
) {
    let mut observer = LoudnessObserver::new(rate).expect("launch rate");
    let mut first = 0_u64;
    for block in plane.chunks(BLOCK) {
        observer.observe(block, block, first, &mut |record| {
            analysis.ingest(&record);
            each(&record, analysis);
        });
        first += block.len() as u64;
    }
}

/// The oracle's trace of `widened` on both channels.
pub fn trace(rate: u32, widened: &[f64]) -> ReferenceLoudnessTrace {
    ReferenceLoudnessTrace::measure(widened, widened, rate).expect("oracle")
}

/// The rate at which the recipe tests also hold the oracle to each case's expectation.
pub const ORACLE_RATE: u32 = 48_000;

/// The oracle's trace at [`ORACLE_RATE`] only, `None` at any other rate.
// The oracle's other-rate K design is already cross-checked by the histogram test at 44.1 and
// 96 kHz, where it must agree with production to 0.05/0.1 LU.
pub fn oracle_at(rate: u32, widened: &[f64]) -> Option<ReferenceLoudnessTrace> {
    (rate == ORACLE_RATE).then(|| trace(rate, widened))
}

/// Asserts `value` is within `tolerance` of `expected`, naming the reading.
pub fn assert_within(what: &str, value: Option<f64>, expected: f64, tolerance: f64) {
    let value = value.unwrap_or_else(|| panic!("{what}: undefined, expected {expected}"));
    assert!(
        (value - expected).abs() <= tolerance,
        "{what}: {value:.4}, expected {expected} +- {tolerance}"
    );
}
