//! Issue #1051: the multiband compressor's randomized bank differential.
//!
//! `conformance::run_effect_differential` renders every bank this build binds against its scalar
//! instances -- whole and chunked, dual and collapsed, continued and restored with hostile state
//! words -- and probes `bind_homogeneous_bank`'s three outcomes, all through the contract calls. A
//! failing seed prints the command that replays it.
//!
//! # A known defect, narrowed
//!
//! The first run of this differential found that a multiband lane's rendered bits depend on where
//! its in-flight parameter ramps are cut:
//!
//! * by a neighbour: a block-rate retarget on one bank lane while another lane's ramp is still in
//!   flight moves the first lane's output a few ulp away from its scalar instance's (first found on
//!   `high_ratio`), which breaks "banking regroups lanes and never changes per-lane arithmetic"
//!   (AGENTS.md);
//! * by a block boundary: the scalar instance renders a block with a ramp in flight differently
//!   whole and in two pieces, which breaks partition invariance (master plan P1).
//!
//! The cause is visible in `Instance::plan_segment`: a segment ends at the nearest ramp arrival of
//! *any* lane and at every block boundary, and each segment starts from coefficients refreshed at
//! its first sample, so where a neighbour's ramp or the host's block ends decides when this lane's
//! coefficients move.
//!
//! Until #1069 lands, the per-PR test lets a scenario either carry automation, on one lane
//! only, or render chunked blocks, never both, and restores only a lane's own untouched snapshot
//! (`Known::RampCutsMoveBits`); everything else runs at full strength. The ignored test below is
//! the reproducer and prints the seed that replays it.

conformance::randomized_effect_test!(
    the_bank_renders_its_scalar_instances_under_random_state,
    multiband_compressor::MultibandCompressorFactory,
    seeds: 12,
    blocks: 24,
    craft: None,
    known: &[conformance::Known::RampCutsMoveBits],
    banks_natively: true,
    witness: false,
);

/// The same differential without the narrowing: red until the known defect's issue lands.
#[test]
#[ignore = "#1069: a ramp's cut moves a lane's bits; see the module documentation"]
fn the_bank_renders_its_scalar_instances_including_the_known_defect() {
    bench_support::alloc::assert_installed();
    bench_support::alloc::set_mode(bench_support::alloc::Mode::Count);
    let coverage = conformance::run_effect_differential(&conformance::EffectDifferential {
        factory: &multiband_compressor::MultibandCompressorFactory,
        test: "the_bank_renders_its_scalar_instances_including_the_known_defect",
        replay: "cargo test -p multiband-compressor --test randomized -- --ignored --exact \
                 the_bank_renders_its_scalar_instances_including_the_known_defect",
        seeds: 24,
        blocks: 24,
        craft: None,
        known: &[],
        banks_natively: true,
        witness: false,
        in_flight: None,
    });
    conformance::assert_reached(&coverage);
}

/// The D7 recovery's report against the contract, on fixed input (no seed): red until #1073
/// lands. This effect counts the frames of a failing block on both channels, where the contract counts blocks.
#[test]
#[ignore = "#1073: the D7 recovery's report breaks the contract; see the test's documentation"]
fn the_d7_recovery_reports_one_block_on_the_failing_lane() {
    conformance::assert_d7_reports(&multiband_compressor::MultibandCompressorFactory);
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
    conformance::EffectDifferential::assert_edge_ramps_restore(
        &multiband_compressor::MultibandCompressorFactory,
    );
}
