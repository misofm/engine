//! Issue #210 phase 3: the third drain, and what a record admitted through it means.
//!
//! The bank's own arithmetic is gated in `builtins`
//! (`tests/input_liveness.rs`, `tests/input_liveness_mono.rs`). This file gates the *record*: what
//! `TrackInputRecord` declares to the channel-symmetry witness, which is the fact the collapse
//! dispatch reads and the one thing no digest can see.
//!
//! # Why the record type is the right unit to test
//!
//! `ChannelSymmetryWitness::admit` is generic over `LiveControlRecord` and dispatches on the
//! record type's `SEAM` const and its `symmetry_event`. Nothing else decides what a drained record
//! did. So a test of `admit` over this record type is a test of the drain's whole contribution to
//! the witness, and it cannot go stale against the drain because the drain has no second opinion
//! to hold.

use builtins::{BuiltinLaneSelector, prepare_input_filter_pair};
use builtins_compiler::TrackInputRecord;
use effect_contract::{ChannelSymmetryWitness, LiveControlRecord, SymmetryEvent};

fn trim(lanes: BuiltinLaneSelector) -> TrackInputRecord {
    TrackInputRecord::TrimDb {
        lanes,
        db: -6.0,
        smoothing_samples: 64,
    }
}

fn polarity(lanes: BuiltinLaneSelector) -> TrackInputRecord {
    TrackInputRecord::PolarityInvert {
        lanes,
        inverted: true,
        smoothing_samples: 64,
    }
}

fn prepared_filter(lanes: BuiltinLaneSelector) -> TrackInputRecord {
    let prepared = prepare_input_filter_pair(48_000, 120.0, 8_000.0).expect("pair");
    TrackInputRecord::PreparedFilter {
        target: builtins::PreparedInputFilterTarget {
            lanes,
            ..prepared.targets[0]
        },
    }
}

/// A per-lane retarget de-symmetrizes; a `Both` retarget preserves.
///
/// Red mutation: return `SymmetryEvent::Preserve` for every selector -> the `Left`/`Right` arms
/// fail. Red mutation: return `Desymmetrize` for every selector -> the `Both` arms fail, and a
/// symmetric ride would retire a track's collapse for the life of the plan.
#[test]
fn a_per_lane_record_desymmetrizes_and_a_both_record_preserves() {
    for build in [
        trim as fn(BuiltinLaneSelector) -> TrackInputRecord,
        polarity,
        prepared_filter,
    ] {
        for (lanes, expected) in [
            (BuiltinLaneSelector::Left, SymmetryEvent::Desymmetrize),
            (BuiltinLaneSelector::Right, SymmetryEvent::Desymmetrize),
            (BuiltinLaneSelector::Both, SymmetryEvent::Preserve),
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
    let mut witness = ChannelSymmetryWitness::SYMMETRIC;
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
    let mut witness = ChannelSymmetryWitness::SYMMETRIC;
    witness.admit(&trim(BuiltinLaneSelector::Left));
    assert!(!witness.holds(ChannelSymmetryWitness::LIVE));
    assert_eq!(
        witness.declined(),
        ChannelSymmetryWitness::LIVE,
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
/// comparison agrees again (`builtins/tests/input_liveness_mono.rs`:
/// `re_equalising_the_words_restores_the_designed_term_and_nothing_more`) and even though the
/// state proof would hold.
///
/// That is **stronger** than M3's rule, which is only that re-equal words must not *by themselves*
/// re-engage. It is the same law `EffectControlRecord` has carried since the witness existed,
/// and the phase deliberately does not change the M-series machinery to relax it.
///
/// Red mutation: make `ChannelSymmetryWitness::admit` set `LIVE` on a `Preserve` event instead
/// of leaving it -> a symmetric retarget after an asymmetric one silently re-arms the collapse
/// onto a right channel nothing proved, and this fails.
#[test]
fn re_equalising_the_words_does_not_re_arm_the_live_term() {
    let mut witness = ChannelSymmetryWitness::SYMMETRIC;
    witness.admit(&trim(BuiltinLaneSelector::Left));
    assert!(!witness.holds(ChannelSymmetryWitness::LIVE));

    // Put the other lane where the first one went, then ride both together for a while.
    witness.admit(&trim(BuiltinLaneSelector::Right));
    for _ in 0..16 {
        witness.admit(&trim(BuiltinLaneSelector::Both));
    }
    assert!(
        !witness.holds(ChannelSymmetryWitness::LIVE),
        "the `LIVE` term is a latch within a plan; only a rebind restores it"
    );
    assert!(
        !witness.eligible(),
        "so the lane does not re-engage its collapse on the strength of equal words"
    );

    // A rebind is the way back, and it is the *only* way back.
    let rebound = ChannelSymmetryWitness::SYMMETRIC;
    assert!(rebound.eligible());
}

/// A block applies the last value of each input cell, however many edits came before it.
///
/// Every other test in the tree writes one or two values per track, so a drain that stopped early,
/// applied the oldest value, or read a cell once per revision would pass them. Here each track
/// takes a long walk of immediate trims ending on -40 dB, rendered, then a second walk ending on
/// -20 dB, rendered again. Each walk is far longer than the 16-record queue the lane had before
/// #1346, and every edit is its own transaction.
///
/// * Every walk was **collapsed** to its last value: each block renders exactly the bits a twin
///   renders when it is sent only that block's last value, and a third, uncommanded twin shows the
///   comparison can see a trim at all.
/// * Nothing is **refused**: the walk is accepted in full, where the queue refused its 17th record.
///
/// The banked drain is held (`BuiltinBankProcessor::begin_block`, through the full cohort and
/// one-lane tail of the SIMD fixture, at the build's own width).
///
/// Replaces `a_block_drains_every_input_record_queued_at_its_entry` (#1027, from the #600
/// input-trim qualification), which filled the queue to capacity.
///
/// Red mutation: apply the oldest value of the walk instead of the newest (read the cell's
/// first-written slot) -> the first block renders the walk's first trim, not its last. Skip the
/// input drain in `begin_block` -> the filled run renders the uncommanded twin's bits.
#[cfg(feature = "test-support")]
#[test]
fn a_block_applies_the_last_of_many_input_edits() {
    use builtins_compiler::{PreparedBuiltinsGraphBound, test_only_prepared_unpaired_graph};
    use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};

    /// Far past the 16 records the input queue held.
    const WALK: usize = 40;

    fn trim(db: f32) -> TrackInputRecord {
        TrackInputRecord::TrimDb {
            lanes: BuiltinLaneSelector::Both,
            db,
            smoothing_samples: 0,
        }
    }

    fn render(bound: &mut PreparedBuiltinsGraphBound, block: u64) -> Vec<u32> {
        let mut output = [0.0_f32; 128];
        bound
            .plan
            .render(
                RenderIo {
                    output: PlanarBufferMut::try_new(&mut output, 2, 64, 64).expect("output"),
                },
                RenderTime {
                    absolute_sample: block * 64,
                },
            )
            .expect("render");
        output.iter().map(|value| value.to_bits()).collect()
    }

    /// Writes one transaction per edit of a walk of trims ending on `last` dB, on every track.
    fn walk(bound: &mut PreparedBuiltinsGraphBound, last: f32) {
        for index in 1..WALK {
            bound.test_only_write(|controls, stamp| {
                for control in controls {
                    control
                        .input
                        .as_mut()
                        .expect("input lane")
                        .write(stamp, trim(-(index as f32)));
                }
            });
        }
        bound.test_only_write(|controls, stamp| {
            for control in controls {
                control
                    .input
                    .as_mut()
                    .expect("input lane")
                    .write(stamp, trim(last));
            }
        });
    }

    type Build = fn() -> PreparedBuiltinsGraphBound;
    let arms: [(&str, Build); 1] = [("banked", test_only_prepared_unpaired_graph)];
    for (arm, build) in arms {
        let mut walked = build();
        let mut last_only = build();
        let mut quiet = build();
        for (block, last) in [(0_u64, -40.0_f32), (1, -20.0)] {
            walk(&mut walked, last);
            last_only.test_only_write(|controls, stamp| {
                for control in controls {
                    control
                        .input
                        .as_mut()
                        .expect("input lane")
                        .write(stamp, trim(last));
                }
            });
            let walked_bits = render(&mut walked, block);
            let last_only_bits = render(&mut last_only, block);
            assert_ne!(
                last_only_bits,
                render(&mut quiet, block),
                "{arm} block {block}: the trim moves no bit, so the comparison proves nothing"
            );
            assert_eq!(
                walked_bits, last_only_bits,
                "{arm} block {block}: the block did not apply the walk's last value"
            );
        }
    }
}
