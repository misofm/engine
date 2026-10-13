#![allow(dead_code)]

use effect_contract::{
    EffectControlRecord, PREPARED_EFFECT_TARGET_WORDS, ParameterChannel, PreparedEffectTarget,
};
use engine::realtime::PlanRevisionGate;
use host_core::{EQ_TARGET_CAPACITY, EffectControlProducer};

/// One live transaction on `producer`, as a host makes it: stamp the next revision on the plan's
/// gate, write `record` to its cell, publish the revision. A subscription goes to the
/// producer's observation queue instead.
pub(super) fn write_effect(
    producer: &mut EffectControlProducer,
    gate: &mut PlanRevisionGate,
    record: EffectControlRecord,
) {
    if let EffectControlRecord::Observe {
        tap_index,
        armed,
        window_blocks,
    } = record
    {
        producer
            .try_push_observe(tap_index, armed, window_blocks)
            .expect("room in the observation queue");
        return;
    }
    producer.preflight(record).expect("a cell for the record");
    let revision = gate.last_published() + 1;
    let stamp = gate.stamp(revision).expect("a legal revision");
    producer.write(stamp, record).expect("preflighted");
    gate.publish(revision);
}

/// Prepare on the test's control thread, then publish through the production owner endpoint.
pub(super) fn push_eq_parameter(
    producer: &mut EffectControlProducer,
    gate: &mut PlanRevisionGate,
    parameter_index: u32,
    channel: ParameterChannel,
    value: f32,
) {
    let revision = producer
        .owner()
        .expect("EQ control owner")
        .committed_revision();
    producer.begin_owner(revision).expect("begin EQ edit");
    producer
        .edit_owner(parameter_index, channel, value)
        .expect("valid EQ edit");
    let mut targets = [PreparedEffectTarget {
        slot: 0,
        channel: ParameterChannel::Both,
        words: [0; PREPARED_EFFECT_TARGET_WORDS],
    }; EQ_TARGET_CAPACITY];
    let count = producer
        .owner()
        .unwrap()
        .prepare_targets_into(&mut targets)
        .expect("off-render EQ preparation");
    producer
        .preflight_candidate_targets(revision, &targets[..count])
        .expect("a valid target set");
    let published = gate.last_published() + 1;
    let stamp = gate.stamp(published).expect("a legal revision");
    producer
        .publish_candidate_targets(stamp, revision, &targets[..count])
        .expect("publish targets");
    gate.publish(published);
    producer.commit_owner().expect("commit accepted EQ edit");
}
