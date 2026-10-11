//! Issue #1409 gate 2: every ramped word of this effect stays inside its endpoints.
//!
//! The render site: site 7 (`delay_chunk`, fed by `LaneChunk` and `CrossChunk`): each lane's
//! feedback, damping coefficient `g` and mix, and the shared cross feedback.
//! Each word has an in-domain move, found by #1409's scan, whose word the unclamped D11 law
//! (`current + step` before the snap) took past its target; the mix has a second, found by
//! #1458's scan, whose crossing frame changes an output bit (five moves, four words). The shared
//! harness (`conformance::ramp_endpoint`, issue #1458) renders each move in one-frame blocks and
//! checks that the effect's own state snapshot holds every ramp word between its value at rest and
//! its target, and that the same window rendered as one block gives the same ramp words, output
//! and final snapshot.
//! Every move runs at the 1 ms delay time ([`DELAY_TIME_MS`]), so the taps carry signal inside the
//! ramp window: feedback, damping `g` and cross feedback reach the ring (and the damping state),
//! and mix reaches the output. A second mix move reaches site 2's block-start word
//! (`LinearRamp::advance_block`). The delay renders per node, so it has no bank half.
//!
//! Test value: a render site left on (or reverted to) the unclamped update passes its target on its
//! move; the partition half catches a site clamped in one block shape only. Mutation evidence is in
//! the attempt records of #1409 and #1458.

use conformance::ramp_endpoint::{Move, RampEndpoints, RampWord, Section, check_every_move};

/// The delay time (parameter 0) every move starts at: its 1 ms minimum, 48 samples at 48 kHz. The
/// taps then carry signal from frame 48, inside the 64-sample ramp, so feedback, damping `g` and
/// cross feedback reach the ring while they move (their writes are read back 48 frames later,
/// after the window, so a defect in them shows in the final snapshot, not the window's output).
/// At the 250 ms default the taps stay silent for the whole window, and only the mix word, which
/// reaches the output, would be exercised.
const DELAY_TIME_MS: f32 = 1.0;

/// Every ramp word of this effect's payload.
const WORDS: [RampWord; 4] = [
    RampWord {
        name: "feedback",
        parameter: 1,
        section: Section::Channels,
        word: 7,
    },
    RampWord {
        name: "damping coefficient",
        parameter: 2,
        section: Section::Channels,
        word: 11,
    },
    RampWord {
        name: "mix",
        parameter: 3,
        section: Section::Channels,
        word: 15,
    },
    RampWord {
        name: "cross feedback",
        parameter: 4,
        section: Section::Common,
        word: 1,
    },
];

/// One overshooting move per word (#1409 gate 2), and a second mix move whose unclamped block-start
/// word changes an output bit (#1458 fold-in); both recorded in the issues.
const MOVES: [Move; 5] = [
    Move {
        word: WORDS[0],
        start: f32::from_bits(0x3f733312),
        target: 0.95,
        whole: true,
    },
    Move {
        word: WORDS[1],
        start: f32::from_bits(0x3e800021),
        target: 0.25,
        whole: true,
    },
    Move {
        word: WORDS[2],
        start: f32::from_bits(0x3f7fffa0),
        target: 1.0,
        whole: true,
    },
    Move {
        word: WORDS[3],
        start: f32::from_bits(0x3f7fffa0),
        target: 1.0,
        whole: true,
    },
    // Issue #1458: a second mix move, one ulp below the first. In the one-frame render every frame
    // is a block start, so the first word of `LinearRamp::advance_block` (site 2) is the word
    // that renders; here the unclamped `current + step` it would take passes the target and
    // changes an output bit, which the first mix move's does not. Found by a scan of candidate
    // delay moves against that site's unclamped mutant (the issue's attempt record).
    Move {
        word: WORDS[2],
        start: f32::from_bits(0x3f7fff9f),
        target: 1.0,
        whole: true,
    },
];

/// This effect's ramp words and moves, for the shared harness.
const ENDPOINTS: RampEndpoints<'static> = RampEndpoints {
    words: &WORDS,
    moves: &MOVES,
    rest: &[(0, DELAY_TIME_MS)],
};

#[test]
fn every_ramped_word_stays_inside_its_endpoints() {
    check_every_move(Box::new(delay::DelayFactory), &ENDPOINTS, &[false]);
}
