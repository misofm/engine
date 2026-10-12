//! Issue #1004, on decision 15's cells (issue #1345): a both-channel write that arrives as two
//! one-channel writes keeps `LIVE`.
//!
//! These are the drain-level gates of the pairing rule in `EffectControlLane::stage`: which drains
//! keep the witness' `LIVE` term, and that the drain's *other* outputs -- the staged spans and the
//! target prefix -- are exactly what a sorted, last-wins window holds. The pairing is structural
//! now: the witness folds from the cells' final contents, so two cells that were both applied in
//! one block with equal words are one both-channel write, however the writes were spread across
//! transactions. The rendered consequences (collapsed output and state against a forced-dual
//! reference, on the real effects) are `tools/console-workload/tests/paired_spans.rs`.
//!
//! The oracle here is deliberately **not** the lane's own shape. The lane reads cells in a fixed
//! order; the model below replays the writes into a map of final values per `(parameter,
//! channel)` and asks the question the collapse actually rests on -- does every parameter that was
//! written on one channel end with the same bits on the other -- so an implementation that got
//! pairing, ordering or last-wins wrong disagrees with it.

#[path = "support/lane_rig.rs"]
mod lane_rig;

use std::collections::BTreeMap;

use effect_contract::{
    EffectControlRecord, EffectLaneShape, EffectWriteRefusal, ParameterChannel,
    ParameterChannelPolicy,
};
use lane_rig::{Rig, UNBYPASSED, both, left, right, target};

/// Parameters 0, 1 and 3 are `PerLane` (a `Left` and a `Right` cell); 2 is `Shared` (one `Both`
/// cell).
fn shape(target_slots: usize) -> EffectLaneShape {
    EffectLaneShape::from_live_parameters(
        [
            (0, ParameterChannelPolicy::PerLane),
            (1, ParameterChannelPolicy::PerLane),
            (2, ParameterChannelPolicy::Shared),
            (3, ParameterChannelPolicy::PerLane),
        ],
        target_slots,
    )
}

/// Write `records` into a fresh lane, each as a transaction of its own, drain one block and report
/// whether `LIVE` survived.
fn keeps(records: &[EffectControlRecord]) -> bool {
    let mut rig = Rig::new(shape(0), 0);
    rig.commit_each(records);
    rig.stage();
    rig.live()
}

/// The independent oracle: replay one block's parameter writes into final values per
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
fn a_twin_pair_keeps_live_in_either_write_order() {
    assert!(keeps(&[left(0, -12.0), right(0, -12.0)]));
    assert!(
        keeps(&[right(0, -12.0), left(0, -12.0)]),
        "the cells are read in a fixed order, so write order does not matter"
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
        "two pairs interleaved in the writes are still two pairs"
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
        "the same value on two parameters is not a pair"
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
fn last_wins_decides_the_pair() {
    // [L v1, L v2, R v2]: the cells end at L v2, R v2.
    assert!(keeps(&[left(0, -6.0), left(0, -9.0), right(0, -9.0)]));
    // [L v1, R v2, L v2]: L is replaced by v2, which R already holds.
    assert!(keeps(&[left(0, -6.0), right(0, -9.0), left(0, -9.0)]));
    // [L v1, R v1, L v2]: the pair came apart again before the block.
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
fn a_shared_parameters_both_cell_needs_no_twin_and_does_not_break_a_pair() {
    assert!(keeps(&[both(2, 1.0)]));
    assert!(keeps(&[both(2, 1.0), left(0, -6.0), right(0, -6.0)]));
    assert!(!keeps(&[both(2, 1.0), left(0, -6.0)]));
}

/// A `Both` write to a `PerLane` parameter has no cell: the channel policy decides the cells, and
/// a `Shared` parameter takes `Both` alone. The host lowers a both-channel command on a `PerLane`
/// parameter to one write per lane before it reaches a cell.
#[test]
fn a_channel_the_policy_does_not_take_has_no_cell() {
    let mut rig = Rig::new(shape(0), 0);
    assert_eq!(
        rig.commit(&[both(0, 1.0)]),
        Err(EffectWriteRefusal::Channel)
    );
    assert_eq!(
        rig.commit(&[left(2, 1.0)]),
        Err(EffectWriteRefusal::Channel)
    );
    assert_eq!(
        rig.commit(&[right(2, 1.0)]),
        Err(EffectWriteRefusal::Channel)
    );
    assert_eq!(rig.stage().staged, 0);
}

#[test]
fn a_pair_split_across_two_blocks_clears_live() {
    for (first, second) in [
        (left(0, -6.0), right(0, -6.0)),
        (right(0, -6.0), left(0, -6.0)),
    ] {
        let mut rig = Rig::new(shape(0), 0);
        rig.commit_each(&[first]);
        rig.stage();
        assert!(!rig.live(), "the first half alone is a one-channel write");
        rig.commit_each(&[second]);
        rig.stage();
        assert!(!rig.live(), "and nothing re-earns the term");
    }
    // The other way round: a kept pair, then a lone write in the next block.
    let mut rig = Rig::new(shape(0), 0);
    rig.commit_each(&[left(0, -6.0), right(0, -6.0)]);
    rig.stage();
    assert!(rig.live());
    rig.commit_each(&[left(0, -9.0)]);
    rig.stage();
    assert!(!rig.live());
}

#[test]
fn a_pair_whose_second_half_never_arrives_clears_live() {
    let mut rig = Rig::new(shape(0), 0);
    rig.commit_each(&[left(3, 0.5)]);
    rig.stage();
    assert!(!rig.live());
    for _ in 0..4 {
        rig.stage();
        assert!(!rig.live(), "empty blocks never restore the term");
    }
}

/// Targets: equal words on a slot's two channel cells are one `Both` target (the structural form
/// of a symmetric both-channel edit), and any other applied target is a lone channel write.
///
/// Test value: the witness folded per write instead of from the cells' final contents, which
/// retires a mono collapse on a both-channel target edit (the F1 shape `[Left A, Both C, Right A]`
/// ends with the channels at C and A, and a lone write clears the term).
#[test]
fn targets_fold_from_the_final_cell_contents() {
    // `[Left A, Both C, Right A]`: the left cell ends at C and the right at A.
    let mut rig = Rig::new(shape(2), 0);
    rig.commit_each(&[
        target(1, ParameterChannel::Left, 0xA),
        target(1, ParameterChannel::Both, 0xC),
        target(1, ParameterChannel::Right, 0xA),
    ]);
    let staged = rig.stage();
    assert_eq!(staged.staged_targets, 2, "two cells, two lone targets");
    assert!(
        !rig.live(),
        "C on the left and A on the right: not symmetric"
    );

    // Equal final words on the two cells are one `Both` target, and keep the term.
    let mut rig = Rig::new(shape(2), 0);
    rig.commit_each(&[
        target(1, ParameterChannel::Left, 0xA),
        target(1, ParameterChannel::Right, 0xA),
    ]);
    let staged = rig.stage();
    assert_eq!(staged.staged_targets, 1);
    assert_eq!(
        rig.lane.prepared_targets()[0].channel,
        ParameterChannel::Both
    );
    assert!(
        rig.live(),
        "a per-channel pair with equal words is a both-channel write"
    );

    // A `Both` target preserves, as it always did.
    let mut rig = Rig::new(shape(2), 0);
    rig.commit_each(&[target(1, ParameterChannel::Both, 0xC)]);
    assert_eq!(rig.stage().staged_targets, 1);
    assert!(rig.live());

    // A one-channel target clears.
    let mut rig = Rig::new(shape(2), 0);
    rig.commit_each(&[target(1, ParameterChannel::Left, 0xA)]);
    assert_eq!(rig.stage().staged_targets, 1);
    assert!(!rig.live());

    // Two slots: one pair and one lone write clear the term; the pair applies once.
    let mut rig = Rig::new(shape(2), 0);
    rig.commit_each(&[
        target(0, ParameterChannel::Both, 0x1),
        target(1, ParameterChannel::Right, 0x2),
    ]);
    assert_eq!(rig.stage().staged_targets, 2);
    assert!(!rig.live());
}

#[test]
fn bypass_and_observe_records_fold_in_place_around_a_pair() {
    let mut rig = Rig::new(shape(0), 4);
    rig.commit_each(&[
        EffectControlRecord::Bypass(true),
        left(0, -6.0),
        right(0, -6.0),
    ]);
    rig.writer.try_push_observe(0, true, 4).expect("room");
    rig.stage();
    assert_eq!(
        rig.lane.drain_observations(0, None),
        1,
        "a plan with no observation lane refuses the subscription"
    );
    assert!(
        rig.live(),
        "the pair is still a pair across unrelated records"
    );
    assert!(
        !rig.lane.symmetry().holds(UNBYPASSED),
        "the bypass folded as ever"
    );
    rig.commit_each(&[left(0, -9.0), EffectControlRecord::Bypass(false)]);
    rig.stage();
    assert!(rig.lane.symmetry().holds(UNBYPASSED));
    assert!(
        !rig.live(),
        "a lone write clears LIVE even when a record after it re-earns UNBYPASSED"
    );
}

/// Every order of a mixed set of writes: the lane's `LIVE` answer equals the model's, and its
/// staged window equals the sorted last-wins window.
#[test]
fn every_order_of_a_mixed_block_agrees_with_the_model() {
    let atom_sets: [&[EffectControlRecord]; 5] = [
        // A pair, a lone Left on another parameter, and a Shared `Both`.
        &[left(0, -6.0), right(0, -6.0), left(1, 4.0), both(2, 0.5)],
        // Two pairs, one of them one ulp apart.
        &[
            left(0, -6.0),
            right(0, -6.0),
            left(1, 4.0),
            right(1, f32::from_bits(4.0_f32.to_bits() + 1)),
        ],
        // Last-wins across a pair: L v1, L v2, R v2, a `Both`.
        &[left(0, -6.0), left(0, -9.0), right(0, -9.0), both(2, 0.25)],
        // Two pairs and a restatement.
        &[
            left(0, -6.0),
            right(0, -6.0),
            left(1, 4.0),
            right(1, 4.0),
            left(0, -6.0),
        ],
        // A pair and a lone Right elsewhere.
        &[left(0, -6.0), right(0, -6.0), both(2, -3.0), right(3, 1.0)],
    ];
    let mut kept = 0;
    let mut cleared = 0;
    for atoms in atom_sets {
        for order in permutations(atoms) {
            let expected = model_pairs(&order);
            let mut rig = Rig::new(shape(0), 0);
            rig.commit_each(&order);
            let staged = rig.stage();
            assert_eq!(
                rig.staged_spans(staged),
                model_window(&order),
                "{order:?}: the staged window is the sorted last-wins window"
            );
            assert_eq!(rig.live(), expected, "{order:?}");
            let writes = order.len() as u64;
            assert_eq!(
                rig.superseded.get(),
                writes - staged.staged as u64,
                "{order:?}: each write a later one replaced is counted once"
            );
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

/// Every sequence of up to four writes over a small alphabet, drained in one block and split
/// across two blocks at every point, against the model.
///
/// The alphabet has two `PerLane` parameters and the `Shared` one, two values each, so it reaches
/// every adjacency the cells can form: twins, near twins, a Left next to another parameter's
/// Right, and lone halves at either end.
#[test]
fn exhaustive_short_sequences_agree_with_the_model_in_one_block_and_split_across_two() {
    let mut alphabet = Vec::new();
    for index in 0..2 {
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            for value in [-6.0_f32, -9.0] {
                alphabet.push(lane_rig::parameter(index, channel, value));
            }
        }
    }
    for value in [-6.0_f32, -9.0] {
        alphabet.push(both(2, value));
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
        let one_block = model_pairs(sequence);
        assert_eq!(keeps(sequence), one_block, "{sequence:?}");
        assert!(
            !base_keeps(sequence) || one_block,
            "{sequence:?}: the rule may only keep more than the base drain, never less"
        );
        counts[usize::from(one_block)] += 1;
        for split in 1..sequence.len() {
            let (first, second) = sequence.split_at(split);
            let mut rig = Rig::new(shape(0), 0);
            rig.commit_each(first);
            rig.stage();
            let after_first = rig.live();
            rig.commit_each(second);
            rig.stage();
            assert_eq!(after_first, model_pairs(first), "{first:?} | {second:?}");
            assert_eq!(
                rig.live(),
                model_pairs(first) && model_pairs(second),
                "{first:?} | {second:?}: each block is judged alone and LIVE never returns"
            );
            counts[2] += 1;
        }
    }
    assert!(
        counts[0] > 0 && counts[1] > 0 && counts[2] > 0,
        "{counts:?}"
    );
}

/// The sorted, last-wins window a block stages, computed independently of the lane.
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
