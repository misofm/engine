//! Issue #1051: the parametric EQ's randomized bank differential.
//!
//! `conformance::run_effect_differential` renders every bank this build binds against its scalar
//! instances -- whole and chunked, dual and collapsed, continued and restored with hostile state
//! words -- and probes `bind_homogeneous_bank`'s three outcomes, all through the contract calls. A
//! failing seed prints the command that replays it.
//!
//! The bind probe runs at full strength: an invalid member refuses with `prepare`'s own
//! diagnostic at every width, the width this build does not execute included (#1070).
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
    banks_natively: true,
    witness: true,
    in_flight: Some(ramp_in_flight),
);
