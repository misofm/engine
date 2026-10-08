//! Issue #1409 gate 2: every ramped word of this effect stays inside its endpoints.
//!
//! The render site: site 6 (`run_segment`): each band's threshold, ratio, attack, release and
//! makeup.
//! Each word has one in-domain move, found by #1409's scan, whose word the unclamped D11 law
//! (`current + step` before the snap) took past its target. The shared harness
//! (`conformance::ramp_endpoint`, issue #1458) renders each move in one-frame blocks and checks
//! that the effect's own state snapshot holds every ramp word between its value at rest and its
//! target, and that the same window rendered as one block gives the same ramp words, output and
//! final snapshot.
//! The multiband compressor refreshes its ratio, attack and release coefficients once per segment,
//! by its frozen design (`tests/identity.rs`,
//! `partition_control_trajectory_preserves_ramp_positions`). On the high band's ratio and attack
//! moves that refresh makes the one-frame and one-block outputs differ, so for those two moves
//! (`whole: false`) the partition half compares the ramp words only; every other move compares
//! everything.
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
const WORDS: [RampWord; 10] = [
    RampWord {
        name: "low threshold",
        parameter: 1,
        section: Section::Channels,
        word: 3,
    },
    RampWord {
        name: "low ratio",
        parameter: 2,
        section: Section::Channels,
        word: 7,
    },
    RampWord {
        name: "low attack",
        parameter: 3,
        section: Section::Channels,
        word: 11,
    },
    RampWord {
        name: "low release",
        parameter: 4,
        section: Section::Channels,
        word: 15,
    },
    RampWord {
        name: "low makeup",
        parameter: 5,
        section: Section::Channels,
        word: 19,
    },
    RampWord {
        name: "high threshold",
        parameter: 6,
        section: Section::Channels,
        word: 23,
    },
    RampWord {
        name: "high ratio",
        parameter: 7,
        section: Section::Channels,
        word: 27,
    },
    RampWord {
        name: "high attack",
        parameter: 8,
        section: Section::Channels,
        word: 31,
    },
    RampWord {
        name: "high release",
        parameter: 9,
        section: Section::Channels,
        word: 35,
    },
    RampWord {
        name: "high makeup",
        parameter: 10,
        section: Section::Channels,
        word: 39,
    },
];

/// One overshooting move per word (#1409 gate 2, recorded in the issue).
const MOVES: [Move; 10] = [
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
        start: f32::from_bits(0x3dccccee),
        target: 0.1,
        whole: true,
    },
    Move {
        word: WORDS[3],
        start: f32::from_bits(0x459c3fdf),
        target: 5000.0,
        whole: true,
    },
    Move {
        word: WORDS[4],
        start: f32::from_bits(0x41bfffdf),
        target: 24.0,
        whole: true,
    },
    Move {
        word: WORDS[5],
        start: f32::from_bits(0xc29fffdf),
        target: -80.0,
        whole: true,
    },
    Move {
        word: WORDS[6],
        start: f32::from_bits(0x3f800021),
        target: 1.0,
        whole: false,
    },
    Move {
        word: WORDS[7],
        start: f32::from_bits(0x3dccccee),
        target: 0.1,
        whole: false,
    },
    Move {
        word: WORDS[8],
        start: f32::from_bits(0x459c3fdf),
        target: 5000.0,
        whole: true,
    },
    Move {
        word: WORDS[9],
        start: f32::from_bits(0x41bfffdf),
        target: 24.0,
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
        Box::new(multiband_compressor::MultibandCompressorFactory),
        &ENDPOINTS,
        &[false],
    );
}

#[test]
fn every_ramped_word_of_a_bank_lane_follows_its_own_target() {
    check_every_bank_move(
        Box::new(multiband_compressor::MultibandCompressorFactory),
        &ENDPOINTS,
    );
}
