//! Issue #1004: a both-channel write that arrives as two one-channel records keeps `LIVE`.
//!
//! These are the drain-level gates of the pairing rule in `EffectControlLane::stage`: which
//! drains keep the witness' `LIVE` term, and that the drain's *other* outputs -- the staged spans,
//! the target FIFO, `dropped`, `unbound`, `target_error` -- are exactly what they were before the
//! rule existed. The rendered consequences (collapsed output and state against a forced-dual
//! reference, on the real effects) are `tools/console-workload/tests/paired_spans.rs`.
//!
//! The oracle here is deliberately **not** the rule's own shape. The rule walks adjacent spans of
//! the sorted, deduplicated staging window; the model below replays the records into a map of
//! final values per `(parameter, channel)` and asks the question the collapse actually rests on --
//! does every parameter that was written on one channel end with the same bits on the other --
//! so an implementation that got adjacency, ordering or last-wins wrong disagrees with it.

use std::collections::BTreeMap;

use effect_contract::{
    AutomationSpanKind, ChannelSymmetryWitness, EffectControlLane, EffectControlRecord,
    PREPARED_EFFECT_TARGET_WORDS, ParameterChannel, PreparedAutomationSpan, PreparedEffectTarget,
    Staged,
};
use engine::realtime::{Producer, QueueGeneration, bounded_spsc};

const LIVE: u8 = ChannelSymmetryWitness::LIVE;
const UNBYPASSED: u8 = ChannelSymmetryWitness::UNBYPASSED;

fn depth(value: usize) -> core::num::NonZeroUsize {
    core::num::NonZeroUsize::new(value).expect("nonzero queue depth")
}

fn idle() -> PreparedAutomationSpan {
    PreparedAutomationSpan {
        kind: AutomationSpanKind::Point,
        channel: ParameterChannel::Both,
        parameter_index: u32::MAX,
        start_sample: 0,
        end_sample: 0,
        start_value: 0.0,
        end_value: 0.0,
    }
}

fn parameter(index: u32, channel: ParameterChannel, value: f32) -> EffectControlRecord {
    EffectControlRecord::Parameter {
        parameter_index: index,
        channel,
        value,
    }
}

fn left(index: u32, value: f32) -> EffectControlRecord {
    parameter(index, ParameterChannel::Left, value)
}

fn right(index: u32, value: f32) -> EffectControlRecord {
    parameter(index, ParameterChannel::Right, value)
}

fn both(index: u32, value: f32) -> EffectControlRecord {
    parameter(index, ParameterChannel::Both, value)
}

fn target(slot: u32, channel: ParameterChannel, marker: u32) -> EffectControlRecord {
    EffectControlRecord::PreparedTarget(PreparedEffectTarget {
        slot,
        channel,
        words: [marker; PREPARED_EFFECT_TARGET_WORDS],
    })
}

/// One lane under test: a queue of `capacity` records and a staging window of `window` spans.
struct Lane {
    producer: Producer<EffectControlRecord>,
    lane: EffectControlLane,
    staging: Vec<PreparedAutomationSpan>,
    block: u64,
}

impl Lane {
    fn new(capacity: usize, window: usize) -> Self {
        let (producer, consumer) =
            bounded_spsc::<EffectControlRecord>(depth(capacity), QueueGeneration(0))
                .expect("queue");
        Self {
            producer,
            lane: EffectControlLane::new(consumer, false),
            staging: vec![idle(); window],
            block: 0,
        }
    }

    fn with_targets(capacity: usize) -> Self {
        let (producer, consumer) =
            bounded_spsc::<EffectControlRecord>(depth(capacity), QueueGeneration(0))
                .expect("queue");
        Self {
            producer,
            lane: EffectControlLane::new_with_target_staging(consumer, false),
            staging: vec![idle(); capacity],
            block: 0,
        }
    }

    fn push(&mut self, records: &[EffectControlRecord]) {
        for record in records {
            self.producer.try_push(*record).expect("room in the queue");
        }
    }

    /// One drain at the top of the next block.
    fn drain(&mut self) -> Staged {
        let first_sample = self.block * 128;
        self.block += 1;
        self.lane.stage(&mut self.staging, first_sample, None)
    }

    fn live(&self) -> bool {
        self.lane.symmetry().holds(LIVE)
    }

    fn staged_spans(&self, staged: Staged) -> Vec<(u32, ParameterChannel, u32)> {
        self.staging[..staged.staged]
            .iter()
            .map(|span| {
                (
                    span.parameter_index,
                    span.channel,
                    span.start_value.to_bits(),
                )
            })
            .collect()
    }
}

/// Drain `records` into a fresh lane in one block and report whether `LIVE` survived.
fn keeps(records: &[EffectControlRecord]) -> bool {
    let mut lane = Lane::new(records.len().max(1), records.len().max(1));
    lane.push(records);
    let staged = lane.drain();
    assert_eq!(staged.dropped, 0);
    lane.live()
}

/// The independent oracle: replay one drain's parameter records into final values per
/// `(parameter, channel)` and ask whether every parameter written on one channel ends with the
/// same bits on the other. `Both` writes are not one-channel writes and need no twin.
fn model_pairs(records: &[EffectControlRecord]) -> bool {
    let mut last: BTreeMap<(u32, ParameterChannel), u32> = BTreeMap::new();
    for record in records {
        if let EffectControlRecord::Parameter {
            parameter_index,
            channel,
            value,
        } = *record
        {
            last.insert((parameter_index, channel), value.to_bits());
        }
    }
    last.iter().all(|((index, channel), bits)| match channel {
        ParameterChannel::Left => last.get(&(*index, ParameterChannel::Right)) == Some(bits),
        ParameterChannel::Right => last.get(&(*index, ParameterChannel::Left)) == Some(bits),
        ParameterChannel::Both => true,
    })
}

fn has_one_channel_parameter(records: &[EffectControlRecord]) -> bool {
    records.iter().any(|record| {
        matches!(record, EffectControlRecord::Parameter { channel, .. }
            if channel.writes_one_channel())
    })
}

/// The pre-#1004 drain's `LIVE` answer: any one-channel parameter record clears it.
fn base_keeps(records: &[EffectControlRecord]) -> bool {
    !has_one_channel_parameter(records)
}

#[test]
fn a_twin_pair_keeps_live_in_either_queue_order() {
    assert!(keeps(&[left(0, -12.0), right(0, -12.0)]));
    assert!(
        keeps(&[right(0, -12.0), left(0, -12.0)]),
        "the staging sorts the window, so queue order does not matter"
    );
    assert!(keeps(&[
        left(0, -12.0),
        right(0, -12.0),
        left(3, 40.0),
        right(3, 40.0)
    ]));
    assert!(
        keeps(&[
            left(3, 40.0),
            left(0, -12.0),
            right(3, 40.0),
            right(0, -12.0)
        ]),
        "two pairs interleaved in the queue are still two pairs"
    );
}

#[test]
fn a_lone_one_channel_write_clears_live() {
    assert!(!keeps(&[left(0, -12.0)]));
    assert!(!keeps(&[right(0, -12.0)]));
    assert!(!keeps(&[left(0, -12.0), right(0, -12.0), left(1, 4.0)]));
    assert!(!keeps(&[left(0, -12.0), right(0, -12.0), right(1, 4.0)]));
}

#[test]
fn near_pairs_clear_live() {
    let value = -12.0_f32;
    let ulp = f32::from_bits(value.to_bits() + 1);
    assert!(!keeps(&[left(0, value), right(0, ulp)]), "one ulp apart");
    assert!(!keeps(&[left(0, 0.0), right(0, -0.0)]), "+0.0 and -0.0");
    assert!(
        !keeps(&[
            left(0, f32::from_bits(0x7fc0_0000)),
            right(0, f32::from_bits(0x7fc0_0001))
        ]),
        "two NaN payloads"
    );
    assert!(
        !keeps(&[left(0, value), right(1, value)]),
        "the same value on two parameters is not a pair, although the spans are adjacent"
    );
    assert!(
        !keeps(&[
            left(0, value),
            right(0, value + 1.0),
            left(1, value),
            right(1, value + 1.0)
        ]),
        "two different-valued pairs"
    );
}

#[test]
fn last_wins_staging_decides_the_pair() {
    // [L v1, L v2, R v2]: the staged window is L v2, R v2.
    assert!(keeps(&[left(0, -6.0), left(0, -9.0), right(0, -9.0)]));
    // [L v1, R v2, L v2]: L is replaced by v2, which R already holds.
    assert!(keeps(&[left(0, -6.0), right(0, -9.0), left(0, -9.0)]));
    // [L v1, R v1, L v2]: the pair came apart again before the drain ended.
    assert!(!keeps(&[left(0, -6.0), right(0, -6.0), left(0, -9.0)]));
    // One channel written twice, the other once to the *first* value.
    assert!(!keeps(&[left(0, -6.0), left(0, -9.0), right(0, -6.0)]));
    // A restatement of the held value is an ordinary pair.
    assert!(keeps(&[
        left(0, -6.0),
        right(0, -6.0),
        left(0, -6.0),
        right(0, -6.0)
    ]));
}

#[test]
fn both_spans_need_no_twin_and_do_not_break_a_pair() {
    assert!(keeps(&[both(0, 1.0)]));
    assert!(keeps(&[both(1, 1.0), left(0, -6.0), right(0, -6.0)]));
    // A `Both` span on the same parameter sits after the pair (`Left < Right < Both`). A
    // `PerLane` effect refuses it on both channels and a `Shared` one refuses the pair on both
    // channels, so either way the two channels end equal.
    assert!(keeps(&[left(0, -6.0), both(0, -3.0), right(0, -6.0)]));
    assert!(!keeps(&[left(0, -6.0), both(0, -6.0)]));
}

#[test]
fn a_pair_split_across_two_drains_clears_live() {
    for (first, second) in [
        (left(0, -6.0), right(0, -6.0)),
        (right(0, -6.0), left(0, -6.0)),
    ] {
        let mut lane = Lane::new(4, 4);
        lane.push(&[first]);
        lane.drain();
        assert!(!lane.live(), "the first half alone is a one-channel write");
        lane.push(&[second]);
        lane.drain();
        assert!(!lane.live(), "and nothing re-earns the term");
    }
    // The other way round: a kept pair, then a lone write in the next drain.
    let mut lane = Lane::new(4, 4);
    lane.push(&[left(0, -6.0), right(0, -6.0)]);
    lane.drain();
    assert!(lane.live());
    lane.push(&[left(0, -9.0)]);
    lane.drain();
    assert!(!lane.live());
}

#[test]
fn a_pair_whose_second_half_never_arrives_clears_live() {
    let mut lane = Lane::new(4, 4);
    lane.push(&[left(2, 0.5)]);
    lane.drain();
    assert!(!lane.live());
    for _ in 0..4 {
        lane.drain();
        assert!(!lane.live(), "empty drains never restore the term");
    }
}

#[test]
fn one_channel_targets_fold_as_before_in_every_order() {
    // VERIFY-AUTOMATION F1: `[Left A, Both C, Right A]` leaves the channels at C and A although
    // the last target per channel agrees. Every order of those three, and every one-channel
    // target anywhere in a drain, clears the term: targets are never deferred.
    let atoms = [
        target(1, ParameterChannel::Left, 0xA),
        target(1, ParameterChannel::Both, 0xC),
        target(1, ParameterChannel::Right, 0xA),
    ];
    for order in permutations(&atoms) {
        let mut lane = Lane::with_targets(4);
        lane.push(&order);
        let staged = lane.drain();
        assert_eq!(staged.staged_targets, 3);
        assert!(!staged.target_error);
        assert!(!lane.live(), "{order:?} kept LIVE");
    }
    let mut lane = Lane::with_targets(4);
    lane.push(&[
        target(1, ParameterChannel::Left, 0xA),
        target(1, ParameterChannel::Right, 0xA),
    ]);
    lane.drain();
    assert!(
        !lane.live(),
        "a per-channel target pair with equal words still clears: the rule is spans only"
    );
    let mut lane = Lane::with_targets(4);
    lane.push(&[target(1, ParameterChannel::Both, 0xC)]);
    lane.drain();
    assert!(lane.live(), "a Both target preserves, as it always did");
}

#[test]
fn a_parameter_pair_refused_by_a_target_owner_still_errors_and_moves_nothing() {
    // A prepared-target owner refuses raw parameter records (`target_error`). The pair is never
    // staged, so it writes nothing on either channel; the drain's error is what the render path
    // surfaces.
    let mut lane = Lane::with_targets(4);
    lane.push(&[left(3, 6.0), right(3, 6.0)]);
    let staged = lane.drain();
    assert!(staged.target_error);
    assert_eq!(staged.staged, 0);
    assert_eq!(staged.staged_targets, 0);
}

#[test]
fn bypass_and_observe_records_fold_in_place_around_a_pair() {
    let mut lane = Lane::new(8, 8);
    lane.push(&[
        EffectControlRecord::Bypass(true),
        left(0, -6.0),
        EffectControlRecord::Observe {
            tap_index: 0,
            armed: true,
            window_blocks: 4,
        },
        right(0, -6.0),
    ]);
    let staged = lane.drain();
    assert_eq!(
        staged.unbound, 1,
        "a plan with no observation lane refuses the subscription"
    );
    assert!(
        lane.live(),
        "the pair is still a pair across unrelated records"
    );
    assert!(
        !lane.lane.symmetry().holds(UNBYPASSED),
        "the bypass folded as ever"
    );
    lane.push(&[left(0, -9.0), EffectControlRecord::Bypass(false)]);
    lane.drain();
    assert!(lane.lane.symmetry().holds(UNBYPASSED));
    assert!(
        !lane.live(),
        "a lone write clears LIVE even when a record after it re-earns UNBYPASSED"
    );
}

#[test]
fn a_queue_at_capacity_drains_what_was_admitted_and_nothing_else() {
    // Capacity two: both halves fit, a third record is refused at the producer, and the drain
    // pairs what was admitted.
    let mut lane = Lane::new(2, 2);
    lane.push(&[left(0, -6.0), right(0, -6.0)]);
    assert!(
        lane.producer.try_push(left(1, 4.0)).is_err(),
        "the queue is full"
    );
    let staged = lane.drain();
    assert_eq!((staged.staged, staged.dropped), (2, 0));
    assert!(lane.live());

    // Capacity three: the second half of the second pair is refused at the producer, so the drain
    // sees a pair and a lone write, and clears. The refusal is the producer's, before any ack.
    let mut lane = Lane::new(3, 3);
    lane.push(&[left(0, -6.0), right(0, -6.0), left(1, 4.0)]);
    assert!(lane.producer.try_push(right(1, 4.0)).is_err());
    let staged = lane.drain();
    assert_eq!((staged.staged, staged.dropped), (3, 0));
    assert!(!lane.live());
}

#[test]
fn a_window_smaller_than_the_drain_drops_exactly_as_before_and_never_keeps_a_half() {
    // Unreachable in a prepared plan (preparation refuses a queue deeper than the automation
    // capacity), and pinned anyway: `dropped` counts exactly what the pre-#1004 drain counted, a
    // dropped half leaves its twin unpaired, and a pair dropped whole wrote nothing.
    let mut lane = Lane::new(4, 1);
    lane.push(&[left(0, -6.0), right(0, -6.0)]);
    let staged = lane.drain();
    assert_eq!((staged.staged, staged.dropped), (1, 1));
    assert!(
        !lane.live(),
        "the staged Left lost its twin to the full window"
    );

    let mut lane = Lane::new(4, 2);
    lane.push(&[left(0, -6.0), right(0, -6.0), left(1, 4.0), right(1, 4.0)]);
    let staged = lane.drain();
    assert_eq!((staged.staged, staged.dropped), (2, 2));
    assert_eq!(
        lane.staged_spans(staged),
        vec![
            (0, ParameterChannel::Left, (-6.0_f32).to_bits()),
            (0, ParameterChannel::Right, (-6.0_f32).to_bits())
        ]
    );
    assert!(
        lane.live(),
        "parameter 1 reached neither channel, so the channels still agree"
    );
}

/// Every order of a mixed drain: one-channel, `Both` and paired writes, one record at a time.
///
/// The drain's `LIVE` answer must equal the model's, and its staging output must equal the
/// staging a pre-#1004 drain produced -- which, since the rule touches only the witness, is the
/// same function of the records: a sorted, last-wins window.
#[test]
fn every_order_of_a_mixed_drain_agrees_with_the_model() {
    let atom_sets: [&[EffectControlRecord]; 6] = [
        // A pair, a lone Left on another parameter, and a Both.
        &[left(0, -6.0), right(0, -6.0), left(1, 4.0), both(2, 0.5)],
        // Two pairs, one of them one ulp apart.
        &[
            left(0, -6.0),
            right(0, -6.0),
            left(1, 4.0),
            right(1, f32::from_bits(4.0_f32.to_bits() + 1)),
        ],
        // Last-wins across a pair: L v1, L v2, R v2, Both.
        &[left(0, -6.0), left(0, -9.0), right(0, -9.0), both(1, 0.25)],
        // Two pairs and a restatement.
        &[
            left(0, -6.0),
            right(0, -6.0),
            left(1, 4.0),
            right(1, 4.0),
            left(0, -6.0),
        ],
        // The F1 shape, in spans: L a, Both c, R a on one parameter.
        &[left(0, -6.0), both(0, -3.0), right(0, -6.0)],
        // A pair and a Both on the same parameter, plus a lone Right elsewhere.
        &[left(0, -6.0), right(0, -6.0), both(0, -3.0), right(3, 1.0)],
    ];
    let mut kept = 0;
    let mut cleared = 0;
    for atoms in atom_sets {
        for order in permutations(atoms) {
            let expected = model_pairs(&order);
            let mut lane = Lane::new(order.len(), order.len());
            lane.push(&order);
            let staged = lane.drain();
            assert_eq!(staged.dropped, 0);
            assert_eq!(
                lane.staged_spans(staged),
                model_window(&order),
                "{order:?}: the staged window is the sorted last-wins window"
            );
            assert_eq!(lane.live(), expected, "{order:?}");
            if expected {
                kept += 1;
            } else {
                cleared += 1;
            }
        }
    }
    assert!(
        kept > 0 && cleared > 0,
        "the corpus must exercise both answers"
    );
}

/// Every sequence of up to four records over a small alphabet, drained in one block and split
/// across two blocks at every point, against the model.
///
/// The alphabet has two parameters, all three channels and two values, so it reaches every
/// adjacency the sorted window can form: twins, near twins, a Left next to another parameter's
/// Right, lone halves at either end, and `Both` spans between and after.
#[test]
fn exhaustive_short_sequences_agree_with_the_model_in_one_drain_and_split_across_two() {
    let mut alphabet = Vec::new();
    for index in 0..2 {
        for channel in [
            ParameterChannel::Left,
            ParameterChannel::Right,
            ParameterChannel::Both,
        ] {
            for value in [-6.0_f32, -9.0] {
                alphabet.push(parameter(index, channel, value));
            }
        }
    }
    let mut sequences: Vec<Vec<EffectControlRecord>> = vec![Vec::new()];
    let mut frontier: Vec<Vec<EffectControlRecord>> = vec![Vec::new()];
    for _ in 0..4 {
        let mut next = Vec::new();
        for prefix in &frontier {
            for record in &alphabet {
                let mut sequence = prefix.clone();
                sequence.push(*record);
                next.push(sequence);
            }
        }
        sequences.extend(next.iter().cloned());
        frontier = next;
    }
    let mut counts = [0_u64; 3];
    for sequence in &sequences {
        let one_drain = model_pairs(sequence);
        assert_eq!(keeps(sequence), one_drain, "{sequence:?}");
        assert!(
            !base_keeps(sequence) || one_drain,
            "{sequence:?}: the rule may only keep more than the base drain, never less"
        );
        counts[usize::from(one_drain)] += 1;
        for split in 1..sequence.len() {
            let (first, second) = sequence.split_at(split);
            let mut lane = Lane::new(4, 4);
            lane.push(first);
            lane.drain();
            let after_first = lane.live();
            lane.push(second);
            lane.drain();
            assert_eq!(after_first, model_pairs(first), "{first:?} | {second:?}");
            assert_eq!(
                lane.live(),
                model_pairs(first) && model_pairs(second),
                "{first:?} | {second:?}: each drain is judged alone and LIVE never returns"
            );
            counts[2] += 1;
        }
    }
    assert!(
        counts[0] > 0 && counts[1] > 0 && counts[2] > 0,
        "{counts:?}"
    );
}

/// The sorted, last-wins window a drain stages, computed independently of the drain.
fn model_window(records: &[EffectControlRecord]) -> Vec<(u32, ParameterChannel, u32)> {
    let mut last: BTreeMap<(u32, u32), (ParameterChannel, u32)> = BTreeMap::new();
    for record in records {
        if let EffectControlRecord::Parameter {
            parameter_index,
            channel,
            value,
        } = *record
        {
            last.insert(
                (parameter_index, channel as u32),
                (channel, value.to_bits()),
            );
        }
    }
    last.into_iter()
        .map(|((index, _), (channel, bits))| (index, channel, bits))
        .collect()
}

fn permutations(atoms: &[EffectControlRecord]) -> Vec<Vec<EffectControlRecord>> {
    if atoms.len() <= 1 {
        return vec![atoms.to_vec()];
    }
    let mut out = Vec::new();
    for (index, atom) in atoms.iter().enumerate() {
        let mut rest = atoms.to_vec();
        rest.remove(index);
        for mut tail in permutations(&rest) {
            tail.insert(0, *atom);
            out.push(tail);
        }
    }
    out
}
