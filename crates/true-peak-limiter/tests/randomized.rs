//! Issue #1051: the true-peak limiter's D7 report reproducer.
//!
//! The limiter's randomized bank differential (`conformance::randomized_effect_test!`) was built
//! and measured under #1051 and then dropped: every mutant it caught that the limiter's own suite
//! misses is caught by another crate's CI suite (effect-compiler, graph-compiler, host-core), and
//! it found no defect, so per pull request it was cost without a new catch. Its nightly share went
//! with it; the in-crate `randomized_scenarios_render_exactly_the_unmodified_kernel` still runs.

/// The D7 recovery's report against the contract, on fixed input (no seed): red until #1073
/// lands. This effect zeroes a failing block and resets, but its report never counts the block.
#[test]
#[ignore = "#1073: the D7 recovery's report breaks the contract; see the test's documentation"]
fn the_d7_recovery_reports_one_block_on_the_failing_lane() {
    conformance::assert_d7_reports(&true_peak_limiter::TruePeakLimiterFactory);
}
