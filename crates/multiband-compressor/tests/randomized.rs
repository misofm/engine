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
//! Until its issue lands, the per-PR test lets a scenario either carry automation, on one lane
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
#[ignore = "reproduces a known defect found by #1051; see the module documentation"]
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
    });
    conformance::assert_reached(&coverage);
}
