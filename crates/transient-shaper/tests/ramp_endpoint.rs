//! Issue #1409 gate 2: every ramped word of this effect stays inside its endpoints.
//!
//! The render site: site 2 (`LinearRamp::next_value`, as the shaper's `advance` reaches it): attack
//! amount, sustain amount and mix.
//! Each word has one in-domain move, found by #1409's scan, whose word the unclamped D11 law
//! (`current + step` before the snap) took past its target. The shared harness
//! (`conformance::ramp_endpoint`, issue #1458) renders each move in one-frame blocks and checks
//! that the effect's own state snapshot holds every ramp word between its value at rest and its
//! target, and that the same window rendered as one block gives the same ramp words, output and
//! final snapshot.
//! A bank of the native width, with the move on its last lane and every other lane resting, must
//! render the moving lane and hold its ramp words bit for bit as the scalar instance does (D4: each
//! lane clamps toward its own target).
//!
//! Test value: a render site left on (or reverted to) the unclamped update passes its target on its
//! move; the partition half catches a site clamped in one block shape only, and the bank half a
//! site that clamps toward another lane's target. Mutation evidence is in the attempt records of
//! #1409 and #1458.

use conformance::ramp_endpoint::{
    Move, RampEndpoints, RampWord, Section, check_every_bank_move, check_every_move,
};

/// Every ramp word of this effect's payload.
const WORDS: [RampWord; 3] = [
    RampWord {
        name: "attack amount",
        parameter: 0,
        section: Section::Channels,
        word: 2,
    },
    RampWord {
        name: "sustain amount",
        parameter: 1,
        section: Section::Channels,
        word: 6,
    },
    RampWord {
        name: "mix",
        parameter: 2,
        section: Section::Channels,
        word: 10,
    },
];

/// One overshooting move per word (#1409 gate 2, recorded in the issue).
const MOVES: [Move; 3] = [
    Move {
        word: WORDS[0],
        start: f32::from_bits(0x3f7fffa0),
        target: 1.0,
        whole: true,
    },
    Move {
        word: WORDS[1],
        start: f32::from_bits(0xbf7fffa0),
        target: -1.0,
        whole: true,
    },
    Move {
        word: WORDS[2],
        start: f32::from_bits(0x3f7fffa0),
        target: 1.0,
        whole: true,
    },
];

/// This effect's ramp words and moves, for the shared harness.
const ENDPOINTS: RampEndpoints<'static> = RampEndpoints {
    words: &WORDS,
    moves: &MOVES,
    rest: &[],
};

#[test]
fn every_ramped_word_stays_inside_its_endpoints() {
    check_every_move(
        Box::new(transient_shaper::TransientShaperFactory),
        &ENDPOINTS,
        &[false],
    );
}

#[test]
fn every_ramped_word_of_a_bank_lane_follows_its_own_target() {
    check_every_bank_move(
        Box::new(transient_shaper::TransientShaperFactory),
        &ENDPOINTS,
    );
}
