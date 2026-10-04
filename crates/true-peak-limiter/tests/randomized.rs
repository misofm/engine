//! The true-peak limiter's randomized bank differential (#1051), reinstated by #1278.
//!
//! `conformance::run_effect_differential` renders every bank this build binds against its scalar
//! instances -- whole and chunked, dual and collapsed, restored against restored and restored
//! against continued -- and requires that no `process` call and no state-payload call allocates.
//! #1051 dropped this binary as cost without a new catch; #1278 reinstates it because the plan-swap
//! carry restores limiter lanes in the swap block, and only this harness audits the payload calls
//! and compares a restored lane with the continued one. A failing seed prints the command that
//! replays it.

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

/// #1278: the plan-swap carry restores every lane it carries, so a restore must accept every state
/// the effect itself reaches -- including a smoothed ramp to a domain edge whose iterated
/// `current + step` has rounded past the edge, at every launch rate. Red on a restore that holds a
/// moving ramp's `current` (or a subnormal step) to the strict domain.
#[test]
fn the_effects_own_edge_ramp_snapshots_restore() {
    conformance::EffectDifferential::assert_edge_ramps_restore(
        &true_peak_limiter::TruePeakLimiterFactory,
    );
}
