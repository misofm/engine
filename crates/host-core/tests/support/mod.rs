use effect_contract::{PREPARED_EFFECT_TARGET_WORDS, ParameterChannel, PreparedEffectTarget};
use host_core::{EQ_TARGET_CAPACITY, EffectControlProducer};

/// Prepare on the test's control thread, then publish through the production owner endpoint.
pub(super) fn push_eq_parameter(
    producer: &mut EffectControlProducer,
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
        .expect("queue capacity");
    producer
        .publish_candidate_targets(revision, &targets[..count])
        .expect("publish targets");
    producer.commit_owner().expect("commit accepted EQ edit");
}
