//! Issue #1051: the soft clip's randomized bank differential.
//!
//! `conformance::run_effect_differential` renders every bank this build binds against its scalar
//! instances -- whole and chunked, dual and collapsed, continued and restored with hostile state
//! words -- and probes `bind_homogeneous_bank`'s three outcomes, all through the contract calls. A
//! failing seed prints the command that replays it.
//!
//! # A known defect, narrowed
//!
//! The first run of this differential found that the soft clip holds subnormal words it was
//! legally given -- a parameter value of `f32::from_bits(1)` inside its declared domain in a ramp,
//! a subnormal input sample in its history rows -- and its own `restore_state_payload` then
//! refuses the snapshot that carries them (`effect.state.parameter`, `effect.state.history`: those
//! words must be zero or normal). So a snapshot does not survive its own restore. Until its issue
//! lands the per-PR test accepts that refusal so long as the scalar instance and every bank lane
//! refuse alike (`Known::SubnormalStateRefusedOnRestore`); the ignored test below is the
//! reproducer.

conformance::randomized_effect_test!(
    the_bank_renders_its_scalar_instances_under_random_state,
    soft_clip::SoftClipFactory,
    seeds: 8,
    blocks: 24,
    craft: None,
    known: &[conformance::Known::SubnormalStateRefusedOnRestore],
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
        factory: &soft_clip::SoftClipFactory,
        test: "the_bank_renders_its_scalar_instances_including_the_known_defect",
        replay: "cargo test -p soft-clip --test randomized -- --ignored --exact \
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
