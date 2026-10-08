//! Issue #1409 gate 2: every ramped word of this effect stays inside its endpoints.
//!
//! The render site: sites 4 (`RampVec::advance_where`, the main detector's bank path) and 2
//! (`LinearRamp::next_value` and `advance_block`, through the connected sidechain's
//! `Channel::advance_ramps`); every parameter word and the attack and release coefficient words.
//! Each word has one in-domain move, found by #1409's scan, whose word the unclamped D11 law
//! (`current + step` before the snap) took past its target. The shared harness
//! (`conformance::ramp_endpoint`, issue #1458) renders each move in one-frame blocks and checks
//! that the effect's own state snapshot holds every ramp word between its value at rest and its
//! target, and that the same window rendered as one block gives the same ramp words, output and
//! final snapshot.
//! The moves run with the sidechain unconnected (the main detector, site 4) and connected (the
//! sidechain prefix, site 2).
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
const WORDS: [RampWord; 9] = [
    RampWord {
        name: "threshold",
        parameter: 0,
        section: Section::Channels,
        word: 1,
    },
    RampWord {
        name: "ratio",
        parameter: 1,
        section: Section::Channels,
        word: 5,
    },
    RampWord {
        name: "knee",
        parameter: 2,
        section: Section::Channels,
        word: 9,
    },
    RampWord {
        name: "attack",
        parameter: 3,
        section: Section::Channels,
        word: 13,
    },
    RampWord {
        name: "release",
        parameter: 4,
        section: Section::Channels,
        word: 17,
    },
    RampWord {
        name: "makeup",
        parameter: 5,
        section: Section::Channels,
        word: 21,
    },
    RampWord {
        name: "mix",
        parameter: 6,
        section: Section::Channels,
        word: 25,
    },
    RampWord {
        name: "attack coefficient",
        parameter: 3,
        section: Section::Channels,
        word: 29,
    },
    RampWord {
        name: "release coefficient",
        parameter: 4,
        section: Section::Channels,
        word: 33,
    },
];

/// One overshooting move per word (#1409 gate 2, recorded in the issue).
const MOVES: [Move; 9] = [
    Move {
        word: WORDS[0],
        start: f32::from_bits(0xc29fffdf),
        target: -80.0,
        whole: true,
    },
    Move {
        word: WORDS[1],
        start: f32::from_bits(0x3f800021),
        target: 1.0,
        whole: true,
    },
    Move {
        word: WORDS[2],
        start: f32::from_bits(0x41bfffdf),
        target: 24.0,
        whole: true,
    },
    Move {
        word: WORDS[3],
        start: f32::from_bits(0x3dccccee),
        target: 0.1,
        whole: true,
    },
    Move {
        word: WORDS[4],
        start: f32::from_bits(0x459c3fdf),
        target: 5000.0,
        whole: true,
    },
    Move {
        word: WORDS[5],
        start: f32::from_bits(0x41bfffdf),
        target: 24.0,
        whole: true,
    },
    Move {
        word: WORDS[6],
        start: f32::from_bits(0x3f7fffa0),
        target: 1.0,
        whole: true,
    },
    Move {
        word: WORDS[7],
        start: f32::from_bits(0x3dccccf4),
        target: 0.1,
        whole: true,
    },
    Move {
        word: WORDS[8],
        start: f32::from_bits(0x40a00027),
        target: 5.0,
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
        Box::new(compressor::CompressorFactory),
        &ENDPOINTS,
        &[false, true],
    );
}

#[test]
fn every_ramped_word_of_a_bank_lane_follows_its_own_target() {
    check_every_bank_move(Box::new(compressor::CompressorFactory), &ENDPOINTS);
}
