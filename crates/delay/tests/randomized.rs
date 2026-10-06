//! Issue #1051: the delay's randomized bank differential.
//!
//! `conformance::run_effect_differential` renders every bank this build binds against its scalar
//! instances -- whole and chunked, dual and collapsed, continued and restored with hostile state
//! words -- and probes `bind_homogeneous_bank`'s three outcomes, all through the contract calls. A
//! failing seed prints the command that replays it.

conformance::randomized_effect_test!(
    the_bank_renders_its_scalar_instances_under_random_state,
    delay::DelayFactory,
    seeds: 6,
    blocks: 24,
    craft: None,
    known: &[],
    banks_natively: false,
    witness: true,
);

/// #1278 D2a: the plan-swap carry restores every lane it carries, so a restore must accept every
/// state the effect itself reaches -- here each sampled state of a feedback, mix or cross-feedback
/// ramp to a domain edge, at every launch rate.
/// Since #1409 every ramp word stays between its start and its target, so the strict restore
/// (#1411) admits each snapshot. Red when a render site leaves a ramp word outside its endpoints at
/// render (a missing or reverted #1409 clamp), or when the restore refuses a valid in-range
/// snapshot. The fix is that clamp or that validation, never a restore slack.
#[test]
fn the_effects_own_edge_ramp_snapshots_restore() {
    conformance::EffectDifferential::assert_edge_ramps_restore(&delay::DelayFactory);
}
