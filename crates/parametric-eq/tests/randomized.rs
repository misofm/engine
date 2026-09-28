//! Issue #1051: the parametric EQ's randomized bank differential.
//!
//! `conformance::run_effect_differential` renders every bank this build binds against its scalar
//! instances -- whole and chunked, dual and collapsed, continued and restored with hostile state
//! words -- and probes `bind_homogeneous_bank`'s three outcomes, all through the contract calls. A
//! failing seed prints the command that replays it.
//!
//! # A known defect, narrowed
//!
//! The first run of this differential found that `ParametricEqFactory::bind_homogeneous_bank`
//! answers `Ok(None)` at a width this build does not execute, and on the first heterogeneous
//! member, before it has validated every member's initial values. The three-outcome rule on
//! `NativeEffectFactory::bind_homogeneous_bank` says an invalid member refuses first, so "an absent
//! capability can never hide a malformed member". Until its issue lands the per-PR test accepts
//! that decline (`Known::BindDeclinesBeforeValidating`); the ignored test below is the
//! reproducer.

conformance::randomized_effect_test!(
    the_bank_renders_its_scalar_instances_under_random_state,
    parametric_eq::ParametricEqFactory,
    seeds: 24,
    blocks: 24,
    craft: None,
    known: &[conformance::Known::BindDeclinesBeforeValidating],
);

/// The same differential without the narrowing: red until the known defect's issue lands.
#[test]
#[ignore = "reproduces a known defect found by #1051; see the module documentation"]
fn the_bank_renders_its_scalar_instances_including_the_known_defect() {
    bench_support::alloc::assert_installed();
    bench_support::alloc::set_mode(bench_support::alloc::Mode::Count);
    let coverage = conformance::run_effect_differential(&conformance::EffectDifferential {
        factory: &parametric_eq::ParametricEqFactory,
        test: "the_bank_renders_its_scalar_instances_including_the_known_defect",
        replay: "cargo test -p parametric-eq --test randomized -- --ignored --exact \
                 the_bank_renders_its_scalar_instances_including_the_known_defect",
        seeds: 24,
        blocks: 24,
        craft: None,
        known: &[],
    });
    conformance::assert_reached(&coverage);
}
