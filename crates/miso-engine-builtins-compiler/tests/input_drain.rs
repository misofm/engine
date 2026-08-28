//! Issue #210 phase 3: the third drain, and what a record admitted through it means.
//!
//! The bank's own arithmetic is gated in `miso-engine-builtins`
//! (`tests/input_liveness.rs`, `tests/input_liveness_mono.rs`). This file gates the *record*: what
//! `TrackInputRecordV1` declares to the channel-symmetry witness, which is the fact the collapse
//! dispatch reads and the one thing no digest can see.
//!
//! # Why the record type is the right unit to test
//!
//! `ChannelSymmetryWitnessV1::admit` is generic over `LiveConsoleRecordV1` and dispatches on the
//! record type's `SEAM` const and its `symmetry_event`. Nothing else decides what a drained record
//! did. So a test of `admit` over this record type is a test of the drain's whole contribution to
//! the witness, and it cannot go stale against the drain because the drain has no second opinion
//! to hold.

use miso_engine_builtins::BuiltinLaneSelector;
use miso_engine_builtins_compiler::TrackInputRecordV1;
use miso_engine_effect_contract::{
    ChannelSymmetryWitnessV1, LiveConsoleRecordV1, SeamSideV1, SymmetryEventV1,
};

fn trim(lanes: BuiltinLaneSelector) -> TrackInputRecordV1 {
    TrackInputRecordV1::TrimDb {
        lanes,
        db: -6.0,
        smoothing_samples: 64,
    }
}

fn polarity(lanes: BuiltinLaneSelector) -> TrackInputRecordV1 {
    TrackInputRecordV1::PolarityInvert {
        lanes,
        inverted: true,
        smoothing_samples: 64,
    }
}

/// The input chain is upstream of the fader/matrix seam, so every record on this queue gates the
/// collapse.
///
/// Red mutation: set `TrackInputRecordV1::SEAM` to `SeamSideV1::SeamSide` -> `admit` compiles the
/// clearing arm away entirely, every assertion below about a declining witness fails, and a
/// one-lane trim ride would publish on both channels of a collapsed block.
#[test]
fn the_input_record_is_upstream_of_the_seam() {
    assert_eq!(
        TrackInputRecordV1::SEAM,
        SeamSideV1::UpstreamOfSeam,
        "the input chain runs once on a collapsed track, so its records gate the collapse"
    );
}

/// A per-lane retarget de-symmetrizes; a `Both` retarget preserves.
///
/// Red mutation: return `SymmetryEventV1::Preserve` for every selector -> the `Left`/`Right` arms
/// fail. Red mutation: return `Desymmetrize` for every selector -> the `Both` arms fail, and a
/// symmetric ride would retire a track's collapse for the life of the plan.
#[test]
fn a_per_lane_record_desymmetrizes_and_a_both_record_preserves() {
    for build in [
        trim as fn(BuiltinLaneSelector) -> TrackInputRecordV1,
        polarity,
    ] {
        for (lanes, expected) in [
            (BuiltinLaneSelector::Left, SymmetryEventV1::Desymmetrize),
            (BuiltinLaneSelector::Right, SymmetryEventV1::Desymmetrize),
            (BuiltinLaneSelector::Both, SymmetryEventV1::Preserve),
        ] {
            assert_eq!(
                build(lanes).symmetry_event(),
                expected,
                "{lanes:?} is {expected:?}"
            );
        }
    }
}

/// The fold a drain performs, over the whole record vocabulary: what the `LIVE` term ends up as.
///
/// This is the composition, not the component: `admit` is what the drain calls, and the terms it
/// leaves are what `BuiltinBankProcessor::lane_symmetry` conjoins with the bank's designed
/// comparison.
#[test]
fn the_live_term_survives_a_symmetric_ride_and_not_a_one_lane_one() {
    // A symmetric ride of any length leaves every term standing.
    let mut witness = ChannelSymmetryWitnessV1::SYMMETRIC;
    for _ in 0..64 {
        witness.admit(&trim(BuiltinLaneSelector::Both));
        witness.admit(&polarity(BuiltinLaneSelector::Both));
    }
    assert!(
        witness.eligible(),
        "a `Both` ride is symmetry-preserving however long it runs"
    );
    assert!(
        witness.preserves_channel_agreement(),
        "and it preserves the M3 agreement invariant too"
    );

    // One per-lane record clears `LIVE`, and only `LIVE`.
    let mut witness = ChannelSymmetryWitnessV1::SYMMETRIC;
    witness.admit(&trim(BuiltinLaneSelector::Left));
    assert!(!witness.holds(ChannelSymmetryWitnessV1::LIVE));
    assert_eq!(
        witness.declined(),
        ChannelSymmetryWitnessV1::LIVE,
        "an upstream one-channel write clears exactly one term"
    );
    assert!(
        !witness.preserves_channel_agreement(),
        "and `LIVE` is one of the four agreement terms, so the M3 invariant is cleared with it -- \
         which is what makes the collapse *and* the way back both refuse"
    );
}

/// **The re-engage rule.** Re-equalising the parameter words does not bring the collapse back.
///
/// `LIVE` is a latch: `admit` only ever clears it. So a track ridden asymmetrically and then put
/// back symmetric stays declined for the life of the plan, even though the bank's designed-word
/// comparison agrees again (`miso-engine-builtins/tests/input_liveness_mono.rs`:
/// `re_equalising_the_words_restores_the_designed_term_and_nothing_more`) and even though the
/// state proof would hold.
///
/// That is **stronger** than M3's rule, which is only that re-equal words must not *by themselves*
/// re-engage. It is the same law `EffectControlRecordV1` has carried since the witness existed,
/// and the phase deliberately does not change the M-series machinery to relax it.
///
/// Red mutation: make `ChannelSymmetryWitnessV1::admit` set `LIVE` on a `Preserve` event instead
/// of leaving it -> a symmetric retarget after an asymmetric one silently re-arms the collapse
/// onto a right channel nothing proved, and this fails.
#[test]
fn re_equalising_the_words_does_not_re_arm_the_live_term() {
    let mut witness = ChannelSymmetryWitnessV1::SYMMETRIC;
    witness.admit(&trim(BuiltinLaneSelector::Left));
    assert!(!witness.holds(ChannelSymmetryWitnessV1::LIVE));

    // Put the other lane where the first one went, then ride both together for a while.
    witness.admit(&trim(BuiltinLaneSelector::Right));
    for _ in 0..16 {
        witness.admit(&trim(BuiltinLaneSelector::Both));
    }
    assert!(
        !witness.holds(ChannelSymmetryWitnessV1::LIVE),
        "the `LIVE` term is a latch within a plan; only a rebind restores it"
    );
    assert!(
        !witness.eligible(),
        "so the lane does not re-engage its collapse on the strength of equal words"
    );

    // A rebind is the way back, and it is the *only* way back.
    let rebound = ChannelSymmetryWitnessV1::SYMMETRIC;
    assert!(rebound.eligible());
}

/// The record's variants are exhaustive at the witness hook: a third variant is a compile error
/// rather than a silent `Preserve`.
///
/// This is asserted by construction -- `symmetry_event`'s match has no wildcard arm -- and stated
/// here so the property has a name. The test itself checks the weaker observable: both shipped
/// variants answer, and they answer the same way for the same selector, because the two parameters
/// share one coefficient and a ride on either de-symmetrizes the same word.
#[test]
fn both_variants_answer_the_hook_identically_for_the_same_selector() {
    for lanes in [
        BuiltinLaneSelector::Left,
        BuiltinLaneSelector::Right,
        BuiltinLaneSelector::Both,
    ] {
        assert_eq!(
            trim(lanes).symmetry_event(),
            polarity(lanes).symmetry_event(),
            "{lanes:?}: trim and polarity write the same coefficient, so they gate the collapse \
             the same way"
        );
    }
}
