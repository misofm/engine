//! Issue #1051: the transient shaper's randomized bank differential.
//!
//! `conformance::run_effect_differential` renders every bank this build binds against its scalar
//! instances -- whole and chunked, dual and collapsed, continued and restored with hostile state
//! words -- and probes `bind_homogeneous_bank`'s three outcomes, all through the contract calls. A
//! failing seed prints the command that replays it.

conformance::randomized_effect_test!(
    the_bank_renders_its_scalar_instances_under_random_state,
    transient_shaper::TransientShaperFactory,
    seeds: 24,
    blocks: 24,
    craft: None,
    known: &[],
    banks_natively: true,
    witness: false,
);

/// The D7 recovery's report against the contract, on fixed input (no seed): red until #1073
/// lands. This effect zeroes a failing block and resets, but its report never counts the block.
#[test]
#[ignore = "#1073: the D7 recovery's report breaks the contract; see the test's documentation"]
fn the_d7_recovery_reports_one_block_on_the_failing_lane() {
    conformance::assert_d7_reports(Box::new(transient_shaper::TransientShaperFactory));
}

/// #1278: the plan-swap carry restores every lane it carries, so a restore must accept every state
/// the effect itself reaches -- here each sampled state of a smoothed ramp to a domain edge, at
/// every launch rate.
/// Since #1409 every ramp word stays between its start and its target, so the strict restore
/// (#1411) admits each snapshot. Red when the render site whose word the snapshot holds leaves a
/// ramp word outside its endpoints at render (a missing or reverted #1409 clamp), or when the
/// restore refuses a valid in-range snapshot. The fix is that clamp or that validation, never a
/// restore slack.
#[test]
fn the_effects_own_edge_ramp_snapshots_restore() {
    conformance::EffectDifferential::assert_edge_ramps_restore(Box::new(
        transient_shaper::TransientShaperFactory,
    ));
}
