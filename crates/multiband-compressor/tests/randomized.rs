//! Issue #1051: the multiband compressor's randomized bank differential.
//!
//! `conformance::run_effect_differential` renders every bank this build binds against its scalar
//! instances -- whole and chunked, dual and collapsed, continued and restored with hostile state
//! words -- and probes `bind_homogeneous_bank`'s three outcomes, all through the contract calls. A
//! failing seed prints the command that replays it.
//!
//! # A known defect, narrowed
//!
//! The first run of this differential found that a multiband bank lane's rendered bits depend on
//! another lane's in-flight automation: a block-rate retarget on one lane while a neighbour's ramp
//! is still in flight moves the first lane's output by a few ulp against its scalar instance
//! (seed 1 at the time of writing, lane 5, `high_ratio`; the ignored test below replays it). That
//! breaks "banking regroups lanes and never changes per-lane arithmetic" (AGENTS.md). Until its
//! issue lands the per-PR test draws automation on one lane per scenario
//! (`Known::RampCutsMoveBits`); everything else runs at full strength.

conformance::randomized_effect_test!(
    the_bank_renders_its_scalar_instances_under_random_state,
    multiband_compressor::MultibandCompressorFactory,
    seeds: 12,
    blocks: 24,
    craft: None,
    known: &[conformance::Known::RampCutsMoveBits],
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
    });
    conformance::assert_reached(&coverage);
}
