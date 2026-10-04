//! Issue #1051: the soft clip's randomized bank differential.
//!
//! `conformance::run_effect_differential` renders every bank this build binds against its scalar
//! instances -- whole and chunked, dual and collapsed, continued and restored with hostile state
//! words -- and probes `bind_homogeneous_bank`'s three outcomes, all through the contract calls. A
//! failing seed prints the command that replays it.
//!
//! # Own snapshots with subnormal words
//!
//! The first run of this differential found that the soft clip refused its own snapshot when it
//! held subnormal words it was legally given (a subnormal mix inside `[0, 1]`, a subnormal input
//! sample in its dry history). #1071 made the restore accept every word the effect can hold, so
//! this test runs without a narrowing.

conformance::randomized_effect_test!(
    the_bank_renders_its_scalar_instances_under_random_state,
    soft_clip::SoftClipFactory,
    seeds: 8,
    blocks: 24,
    craft: None,
    known: &[],
    banks_natively: true,
    witness: false,
);

/// The D7 recovery's report against the contract, on fixed input (no seed): red until #1073
/// lands. This effect counts the frames of a failing block, on both channels and on every lane of the bank, where the contract counts blocks on the lane that failed.
#[test]
#[ignore = "#1073: the D7 recovery's report breaks the contract; see the test's documentation"]
fn the_d7_recovery_reports_one_block_on_the_failing_lane() {
    conformance::assert_d7_reports(&soft_clip::SoftClipFactory);
}
