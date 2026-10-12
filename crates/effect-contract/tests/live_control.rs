//! Issue #140 A, rewritten for decision 15's latest-target cells (issue #1345): the live-control
//! staging seam.
//!
//! These are the gates for the two things the render path may never get wrong: the spans a drain
//! produces are already in the contract's canonical order (so every effect's own validator accepts
//! them rather than counting them as `invalid_spans`), and the bypass shunt reproduces the dry
//! signal delayed by exactly the effect's declared latency (so bypass preserves PDC).

#[path = "support/lane_rig.rs"]
mod lane_rig;

use effect_contract::{
    BypassShunt, EffectControlRecord, EffectLaneShape, EffectWriteRefusal, ParameterChannel,
    ParameterChannelPolicy,
};
use lane_rig::{Rig, both, left, right, target};

/// A shape with `Left`/`Right` cells for each `PerLane` index and one `Both` cell for each
/// `Shared` one.
fn shape(per_lane: &[u32], shared: &[u32], target_slots: usize) -> EffectLaneShape {
    let mut live: Vec<(u32, ParameterChannelPolicy)> = per_lane
        .iter()
        .map(|index| (*index, ParameterChannelPolicy::PerLane))
        .chain(
            shared
                .iter()
                .map(|index| (*index, ParameterChannelPolicy::Shared)),
        )
        .collect();
    live.sort_by_key(|(index, _)| *index);
    EffectLaneShape::from_live_parameters(live, target_slots)
}

#[test]
fn prepared_control_record_stays_within_the_frozen_sixty_four_byte_bound() {
    assert!(core::mem::size_of::<EffectControlRecord>() <= 64);
    assert_eq!(effect_contract::PREPARED_EFFECT_TARGET_WORDS, 12);
}

/// Prepared targets are cells: the last write to each `(slot, channel)` is the one applied, and a
/// block applies at most one target per cell. `[Left A, Both C, Left Z]` leaves the left cell at Z
/// and the right at C, and the replaced writes are counted.
///
/// Test value: a target lane that stays a FIFO (three targets staged), or one that applies the
/// first write of a cell.
#[test]
fn prepared_targets_are_cells_where_the_last_write_wins() {
    let mut rig = Rig::new(shape(&[], &[], 6), 0);
    rig.commit(&[
        target(3, ParameterChannel::Left, 0x11),
        target(3, ParameterChannel::Both, 0x22),
        target(3, ParameterChannel::Left, 0x33),
    ])
    .expect("cells");
    let staged = rig.stage();
    assert_eq!(staged.staged, 0);
    assert_eq!(staged.staged_targets, 2);
    let targets = rig.lane.prepared_targets();
    assert_eq!(
        targets
            .iter()
            .map(|target| (target.slot, target.channel, target.words[0]))
            .collect::<Vec<_>>(),
        vec![
            (3, ParameterChannel::Left, 0x33),
            (3, ParameterChannel::Right, 0x22),
        ],
        "the left cell holds the third write, the right cell the second"
    );
    assert_eq!(
        rig.superseded.get(),
        2,
        "the left cell replaced two writes, the right none"
    );
}

/// A lane without target cells refuses a target, writes nothing and stages nothing.
///
/// Test value: a target accepted by a lane that has nowhere to hold it (a dropped value that was
/// acknowledged).
#[test]
fn a_lane_without_target_slots_refuses_a_target_and_writes_nothing() {
    let mut rig = Rig::new(shape(&[0], &[], 0), 0);
    assert_eq!(
        rig.commit(&[target(1, ParameterChannel::Left, 0x44)]),
        Err(EffectWriteRefusal::Target)
    );
    assert_eq!(rig.pin.last_published(), 0, "a refusal uses no revision");
    let staged = rig.stage();
    assert_eq!((staged.staged, staged.staged_targets), (0, 0));
    assert!(rig.lane.prepared_targets().is_empty());
    // A slot past the owner's slots is refused the same way.
    let mut rig = Rig::new(shape(&[], &[], 2), 0);
    assert_eq!(
        rig.commit(&[target(2, ParameterChannel::Both, 1)]),
        Err(EffectWriteRefusal::Target)
    );
}

/// A prepared-target owner's lane has no parameter cells: its parameters ride its targets, so a
/// bare parameter record is refused rather than rendered through a second path.
///
/// Test value: a target owner that silently falls back to render-time semantic design.
#[test]
fn a_target_owners_lane_refuses_a_bare_parameter_and_resets_its_target_prefix() {
    let mut rig = Rig::new(shape(&[], &[], 6), 0);
    assert_eq!(
        rig.commit(&[left(7, 0.5)]),
        Err(EffectWriteRefusal::NotLive)
    );
    let staged = rig.stage();
    assert_eq!((staged.staged, staged.staged_targets), (0, 0));

    rig.commit(&[target(2, ParameterChannel::Both, 0x55)])
        .expect("a target");
    let staged = rig.stage();
    assert_eq!(staged.staged_targets, 1);
    assert_eq!(rig.lane.prepared_targets()[0].words[0], 0x55);

    // The next block has nothing new. The old prefix must not remain visible to a processor that
    // sees an empty block.
    let staged = rig.stage();
    assert_eq!((staged.staged, staged.staged_targets), (0, 0));
    assert!(rig.lane.prepared_targets().is_empty());
}

/// The checked write refuses a parameter the lane has no cell for, and a channel its policy does
/// not take, and in either case writes nothing.
///
/// Test value: a `Left` write to a `Shared` parameter reaching a cell (every launch effect counts
/// such a span as invalid, so the value would be lost after an ack).
#[test]
fn a_record_without_a_cell_is_refused_and_writes_nothing() {
    let mut rig = Rig::new(shape(&[1], &[2], 0), 1);
    assert_eq!(
        rig.commit(&[both(1, 1.0)]),
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
    assert_eq!(
        rig.commit(&[left(0, 1.0)]),
        Err(EffectWriteRefusal::NotLive)
    );
    assert_eq!(
        rig.commit(&[left(9, 1.0)]),
        Err(EffectWriteRefusal::NotLive)
    );
    assert_eq!(
        rig.commit(&[EffectControlRecord::Observe {
            tap_index: 0,
            armed: true,
            window_blocks: 0,
        }]),
        Err(EffectWriteRefusal::Observe),
        "a subscription is a queue record, not a cell"
    );
    // A batch with one bad record writes none of its good ones.
    assert_eq!(
        rig.commit(&[left(1, 0.5), left(0, 1.0)]),
        Err(EffectWriteRefusal::NotLive)
    );
    assert_eq!(rig.pin.last_published(), 0);
    assert_eq!(rig.stage().staged, 0);
}

/// Red mutation: stage the parameter groups in write order instead of the shape's order -> the
/// emitted keys are `[(3,Left), (1,Right), (1,Left)]` and the ordering assertion fails on the
/// first pair.
#[test]
fn a_drain_emits_the_contract_canonical_span_order() {
    let mut rig = Rig::new(shape(&[1, 3], &[], 0), 0);
    // Deliberately reverse-ordered writes.
    rig.commit_each(&[left(3, 0.5), right(1, 0.25), left(1, 0.125)]);
    let staged = rig.stage();
    assert_eq!(staged.staged, 3);
    let keys: Vec<(u32, ParameterChannel)> = rig.staging[..3]
        .iter()
        .map(|span| (span.parameter_index, span.channel))
        .collect();
    assert_eq!(
        keys,
        vec![
            (1, ParameterChannel::Left),
            (1, ParameterChannel::Right),
            (3, ParameterChannel::Left),
        ],
        "spans must leave the drain in (parameter_index, channel) order"
    );
    for span in &rig.staging[..3] {
        assert_eq!(span.kind, effect_contract::AutomationSpanKind::Point);
        assert_eq!(span.start_sample, 0);
        assert_eq!(span.end_sample, 0);
        assert_eq!(
            span.start_value.to_bits(),
            span.end_value.to_bits(),
            "a Point span's endpoints are bit-identical, which is what every effect checks"
        );
    }
    let staged = rig.stage();
    assert_eq!(staged.staged, 0);
    rig.commit(&[left(3, 0.75)]).expect("a cell");
    rig.stage();
    assert_eq!(
        rig.staging[0].start_sample,
        128 * 2,
        "the second block's sample"
    );
}

/// A repeated write collapses last-wins: one span per cell, the newest value, and each replaced
/// write counted once.
///
/// Test value: a duplicate staged span (breaks every effect's "not a duplicate" rule), or the
/// oldest value applied.
#[test]
fn a_repeated_write_collapses_last_wins_and_is_counted() {
    let mut rig = Rig::new(shape(&[0, 2], &[], 0), 0);
    rig.commit_each(&[left(2, 1.0), right(0, -1.0), left(2, 7.0)]);
    let staged = rig.stage();
    assert_eq!(staged.staged, 2);
    assert_eq!(rig.staging[0].parameter_index, 0);
    assert_eq!(rig.staging[1].parameter_index, 2);
    assert_eq!(rig.staging[1].start_value, 7.0, "the last write wins");
    assert_eq!(rig.superseded.get(), 1);
}

/// A drain leaves the cells read: the next block stages nothing.
#[test]
fn a_drained_cell_stages_nothing_on_the_next_block() {
    let mut rig = Rig::new(shape(&[1], &[], 0), 0);
    rig.commit(&[left(1, 0.5)]).expect("a cell");
    assert_eq!(rig.stage().staged, 1);
    assert_eq!(rig.stage().staged, 0);
}

/// Bypass is not a span: it is retained state that survives blocks with no traffic.
#[test]
fn bypass_is_retained_state_and_produces_no_span() {
    let mut rig = Rig::new(shape(&[], &[], 0), 0);
    assert!(!rig.lane.bypassed());
    rig.commit(&[EffectControlRecord::Bypass(true)])
        .expect("a cell");
    assert_eq!(rig.stage().staged, 0, "bypass is not a span");
    assert!(rig.lane.bypassed());
    assert_eq!(rig.stage().staged, 0);
    assert!(
        rig.lane.bypassed(),
        "bypass survives a block with no traffic"
    );
}

/// A transaction lands in one block: values stamped above the block's snapshot wait, and every
/// cell of the revision (bypass, parameter, target) applies in the block whose snapshot covers it.
///
/// Test value: an effect lane read without the snapshot (a revision's bypass applied a block
/// before its parameters, or a value of an unpublished revision applied at all).
#[test]
fn a_transaction_lands_in_one_block_under_the_blocks_snapshot() {
    let mut rig = Rig::new(shape(&[1], &[], 2), 0);
    let revision = rig.pin.last_published() + 1;
    {
        let stamp = rig.pin.stamp(revision).expect("a legal revision");
        for record in [
            EffectControlRecord::Bypass(true),
            left(1, 0.5),
            right(1, 0.5),
            target(0, ParameterChannel::Both, 0x77),
        ] {
            rig.writer.write(stamp, record).expect("cells");
        }
    }
    // Written, not published: a block that snapshots now does not see any of it.
    let staged = rig.stage();
    assert_eq!((staged.staged, staged.staged_targets), (0, 0));
    assert!(!rig.lane.bypassed());
    rig.pin.publish(&rig.gate, revision);
    let staged = rig.stage();
    assert_eq!((staged.staged, staged.staged_targets), (2, 1));
    assert!(rig.lane.bypassed(), "the bypass lands with the parameters");
    assert_eq!(rig.superseded.get(), 0);
}

/// Red mutation: make `BypassShunt::capture` skip the `pdc_delay_block` exchange -> the dry block
/// is the *current* input rather than the input `latency` samples ago, so a bypassed effect's
/// impulse lands `latency` samples early and every compiled PDC route timing is wrong.
#[test]
fn the_shunt_reproduces_the_dry_signal_at_the_declared_latency() {
    for latency in [0_usize, 1, 3, 8, 17] {
        let frames = 8_usize;
        let mut shunt = BypassShunt::new(frames, latency);
        let mut produced: Vec<f32> = Vec::new();
        for block in 0..6_usize {
            let mut left: Vec<f32> = (0..frames)
                .map(|frame| (block * frames + frame) as f32)
                .collect();
            let mut right: Vec<f32> = left.iter().map(|value| -*value).collect();
            shunt.capture(&left, &right);
            // The effect would have written its wet output here; bypass replaces it wholesale.
            left.fill(f32::NAN);
            right.fill(f32::NAN);
            shunt.apply(&mut left, &mut right);
            for (index, value) in left.iter().enumerate() {
                assert_eq!(
                    *value, -right[index],
                    "latency={latency}: both planes are delayed together"
                );
            }
            produced.extend_from_slice(&left);
        }
        for (index, value) in produced.iter().enumerate() {
            let expected = if index < latency {
                0.0
            } else {
                (index - latency) as f32
            };
            assert_eq!(
                *value, expected,
                "latency={latency}: sample {index} must be the input delayed by exactly {latency}"
            );
        }
    }
}

/// `-0.0` survives the shunt: the restore is a copy, never an arithmetic blend.
#[test]
fn the_shunt_preserves_signed_zero() {
    let mut shunt = BypassShunt::new(4, 0);
    let left = [-0.0_f32, 0.0, -0.0, 0.0];
    let right = [0.0_f32, -0.0, 0.0, -0.0];
    shunt.capture(&left, &right);
    let mut out_left = [1.0_f32; 4];
    let mut out_right = [1.0_f32; 4];
    shunt.apply(&mut out_left, &mut out_right);
    assert_eq!(
        out_left.map(f32::to_bits),
        left.map(f32::to_bits),
        "a bypassed block is a bitwise copy of the delayed dry block"
    );
    assert_eq!(out_right.map(f32::to_bits), right.map(f32::to_bits));
}

/// The deferred-mask path: a value written above the snapshot stays pending and is read by the
/// first block whose snapshot covers it, exactly once.
#[test]
fn a_value_above_the_snapshot_waits_and_is_applied_once() {
    let mut rig = Rig::new(shape(&[], &[4], 0), 0);
    rig.commit(&[both(4, 1.0)]).expect("a cell");
    let covered = rig.gate.snapshot();
    let revision = rig.pin.last_published() + 1;
    {
        let stamp = rig.pin.stamp(revision).expect("a legal revision");
        rig.writer.write(stamp, both(4, 2.0)).expect("a cell");
    }
    // The first block reads under the older snapshot: the first value, and the newer one waits.
    let staged = rig.stage_under(covered);
    assert_eq!(staged.staged, 1);
    assert_eq!(rig.staging[0].start_value, 1.0);
    rig.pin.publish(&rig.gate, revision);
    let staged = rig.stage();
    assert_eq!(staged.staged, 1);
    assert_eq!(rig.staging[0].start_value, 2.0);
    assert_eq!(rig.stage().staged, 0, "applied once");
    assert_eq!(
        rig.superseded.get(),
        0,
        "nothing was replaced before a snapshot covered it"
    );
}
