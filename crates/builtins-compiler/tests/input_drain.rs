//! Issue #210 phase 3: the third drain, and what a record admitted through it means.
//!
//! The bank's own arithmetic is gated in `builtins`
//! (`tests/input_liveness.rs`, `tests/input_liveness_mono.rs`). This file gates the *record*: what
//! `TrackInputRecord` declares to the channel-symmetry witness, which is the fact the collapse
//! dispatch reads and the one thing no digest can see.
//!
//! # Why the record type is the right unit to test
//!
//! `ChannelSymmetryWitness::admit` is generic over `LiveConsoleRecord` and dispatches on the
//! record type's `SEAM` const and its `symmetry_event`. Nothing else decides what a drained record
//! did. So a test of `admit` over this record type is a test of the drain's whole contribution to
//! the witness, and it cannot go stale against the drain because the drain has no second opinion
//! to hold.

use builtins::{BuiltinLaneSelector, prepare_input_filter_pair};
use builtins_compiler::TrackInputRecord;
use effect_contract::{ChannelSymmetryWitness, LiveConsoleRecord, SeamSide, SymmetryEvent};

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

#[test]
fn prepared_filter_record_stays_within_the_frozen_queue_slot_bound() {
    assert!(core::mem::size_of::<TrackInputRecord>() <= 64);
}

/// The input chain is upstream of the fader/matrix seam, so every record on this queue gates the
/// collapse.
///
/// Red mutation: set `TrackInputRecord::SEAM` to `SeamSide::SeamSide` -> `admit` compiles the
/// clearing arm away entirely, every assertion below about a declining witness fails, and a
/// one-lane trim ride would publish on both channels of a collapsed block.
#[test]
fn the_input_record_is_upstream_of_the_seam() {
    assert_eq!(
        TrackInputRecord::SEAM,
        SeamSide::UpstreamOfSeam,
        "the input chain runs once on a collapsed track, so its records gate the collapse"
    );
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

/// A block applies every input record queued at its entry, however full the queue is.
///
/// The drain reads the queue's length once at block entry (`available_at_entry`) and pops that
/// many. Every other test in the tree queues one or two records per track, so a drain that stopped
/// early -- a per-block cap, an off-by-one on the count, a `break` on the first retarget -- would
/// leave the rest pending and apply them blocks late without any of them noticing. Here each
/// track's queue is filled to capacity with a walk of immediate trims ending on -40 dB, rendered,
/// then filled again, ending on -20 dB, and rendered again. The second fill starts where the first
/// drain left the ring's cursors, so the second block's drain reads a wrapped count.
///
/// * Every record was **applied** in its block: each block renders exactly the bits a twin renders
///   when it is sent only that block's last record, and a third, uncommanded twin shows the
///   comparison can see a trim at all.
/// * Nothing is **pending**: after each block every queue takes another capacity's worth of
///   records.
///
/// Both drains are held: the banked one (`BuiltinBankProcessor::begin_block`, through the
/// eight-lane cohort and one-lane tail of the SIMD fixture) and the scalar one
/// (`ConsoleInputProcessor::process`, through the scalar-dispatch fixture).
///
/// Ported by #1027 from the #600 input-trim qualification (`tools/bench/src/input_symmetry.rs`,
/// `separate_capacity_sixteen_drain_witness_has_no_pending_records`), retired with its capture
/// tooling.
///
/// Red mutation (Sol, #1027 attempt 1): cap the banked drain at two records per block,
/// `for _ in 0..available.min(2)` -> the banked arm's first block renders the second record's trim
/// instead of the last one's; with that assertion removed, the refill finds records still pending.
/// The same cap on the scalar drain turns the scalar arm red. A consumer that reads a wrapped count
/// as empty turns the second block red.
#[cfg(feature = "test-support")]
#[test]
fn a_block_drains_every_input_record_queued_at_its_entry() {
    use builtins_compiler::{
        PreparedBuiltinsGraphBound, test_only_prepared_pair_graph,
        test_only_prepared_scalar_pair_graph,
    };
    use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};

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

    /// Fills every track's queue: a walk of trims ending on `last` dB. Returns the capacities.
    fn fill(bound: &mut PreparedBuiltinsGraphBound, arm: &str, last: f32) -> Vec<usize> {
        let mut capacities = Vec::new();
        for control in &mut bound.track_controls {
            let capacity = control.input.capacity();
            assert!(
                capacity > 2,
                "{arm}: a queue of {capacity} cannot tell a cap of two apart"
            );
            for index in 1..capacity {
                control
                    .input
                    .try_push(trim(-(index as f32)))
                    .unwrap_or_else(|_| {
                        panic!(
                            "{arm} {}: a record from before the block is still pending",
                            control.track_id
                        )
                    });
            }
            control.input.try_push(trim(last)).unwrap_or_else(|_| {
                panic!(
                    "{arm} {}: a record from before the block is still pending",
                    control.track_id
                )
            });
            assert!(
                control.input.try_push(trim(-6.0)).is_err(),
                "{arm}: the queue is full before the block"
            );
            capacities.push(capacity);
        }
        capacities
    }

    type Build = fn() -> PreparedBuiltinsGraphBound;
    let arms: [(&str, Build); 2] = [
        ("banked", || test_only_prepared_pair_graph(false)),
        ("scalar", || test_only_prepared_scalar_pair_graph(false)),
    ];
    for (arm, build) in arms {
        let mut filled = build();
        let mut last_only = build();
        let mut quiet = build();
        // Block 0: an empty ring filled to capacity. Block 1: refilled after the drain, so the
        // ring's cursors have wrapped, and the drain reads a wrapped count.
        for (block, last) in [(0_u64, -40.0_f32), (1, -20.0)] {
            fill(&mut filled, arm, last);
            for control in &mut last_only.track_controls {
                control.input.try_push(trim(last)).expect("queue room");
            }
            let filled_bits = render(&mut filled, block);
            let last_only_bits = render(&mut last_only, block);
            assert_ne!(
                last_only_bits,
                render(&mut quiet, block),
                "{arm} block {block}: the trim moves no bit, so the comparison proves nothing"
            );
            assert_eq!(
                filled_bits, last_only_bits,
                "{arm} block {block}: the block did not apply every queued record: the last one \
                 did not decide it"
            );
        }
        // Nothing from block 1 is pending either: every queue takes another capacity's worth.
        fill(&mut filled, arm, -6.0);
    }
}
