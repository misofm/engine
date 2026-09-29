//! Issue #1051: the true-peak limiter's randomized bank differential.
//!
//! `conformance::run_effect_differential` renders every bank this build binds against its scalar
//! instances -- whole and chunked, dual and collapsed, continued and restored with hostile state
//! words -- and probes `bind_homogeneous_bank`'s three outcomes, all through the contract calls. A
//! failing seed prints the command that replays it.

conformance::randomized_effect_test!(
    the_bank_renders_its_scalar_instances_under_random_state,
    true_peak_limiter::TruePeakLimiterFactory,
    seeds: 8,
    blocks: 24,
    craft: None,
    known: &[],
    banks_natively: true,
    witness: true,
);

/// The D7 recovery's report against the contract, on fixed input (no seed): red until #1073
/// lands. This effect zeroes a failing block and resets, but its report never counts the block.
#[test]
#[ignore = "#1073: the D7 recovery's report breaks the contract; see the test's documentation"]
fn the_d7_recovery_reports_one_block_on_the_failing_lane() {
    conformance::assert_d7_reports(&true_peak_limiter::TruePeakLimiterFactory);
}
