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
//! capability can never hide a malformed member". Until #1070 lands the per-PR test accepts
//! that decline (`Known::BindDeclinesBeforeValidating`); the ignored test below is the
//! reproducer.
//!
//! # Ramps
//!
//! The EQ refuses raw automation spans (`invalid_spans`; #807): its automation enters as prepared
//! targets, which the differential designs through `target_preparation` and applies to the scalar
//! instance, the bank lane and the lane's restored twin at a block boundary. So every EQ ramp the
//! restored-against-continued oracle sees comes from a target, and [`ramp_in_flight`] reads each
//! continuation's snapshot so the in-flight coverage counts the ramps that are actually moving.

/// Words per band in a lane section, and the band's `remaining` ramp counter within them
/// (`STATE_WORDS_PER_BAND` and the snapshot's word 14 in `parametric_eq`).
const WORDS_PER_BAND: usize = 19;
const REMAINING_WORD: usize = 14;

/// Whether any of the six sections of either channel holds a ramp with samples remaining.
fn ramp_in_flight(payload: &conformance::Payload) -> bool {
    [&payload.left, &payload.right].into_iter().any(|section| {
        (0..parametric_eq::EQ_SECTION_COUNT).any(|band| {
            let at = 4 * (band * WORDS_PER_BAND + REMAINING_WORD);
            u32::from_le_bytes(section[at..at + 4].try_into().expect("a word")) != 0
        })
    })
}

conformance::randomized_effect_test!(
    the_bank_renders_its_scalar_instances_under_random_state,
    parametric_eq::ParametricEqFactory,
    seeds: 24,
    blocks: 24,
    craft: None,
    known: &[conformance::Known::BindDeclinesBeforeValidating],
    banks_natively: true,
    witness: true,
    in_flight: Some(ramp_in_flight),
);

/// The same differential without the narrowing: red until the known defect's issue lands.
#[test]
#[ignore = "#1070: bind declines before validating its members; see the module documentation"]
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
        banks_natively: true,
        witness: true,
        in_flight: Some(ramp_in_flight),
    });
    conformance::assert_reached(&coverage);
}
