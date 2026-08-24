//! Issue #140 A: the live-control staging seam.
//!
//! These are the gates for the two things the render path may never get wrong: the spans a drain
//! produces are already in the contract's canonical order (so every effect's own validator accepts
//! them rather than counting them as `invalid_spans`), and the bypass shunt reproduces the dry
//! signal delayed by exactly the effect's declared latency (so bypass preserves PDC).

use miso_engine_core::realtime::{QueueGeneration, bounded_spsc};
use miso_engine_effect_contract::{
    AutomationSpanKind, BypassShunt, EffectControlLane, EffectControlRecordV1, ParameterChannel,
    PreparedAutomationSpan,
};

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

fn parameter(index: u32, channel: ParameterChannel, value: f32) -> EffectControlRecordV1 {
    EffectControlRecordV1::Parameter {
        parameter_index: index,
        channel,
        value,
    }
}

/// Red mutation: delete the `existing > key` insertion leg in `EffectControlLane::stage` so
/// records are appended in arrival order -> the emitted keys are `[(3,Left), (1,Right), (1,Left)]`
/// and the ordering assertion fails on the first pair.
#[test]
fn a_drain_emits_the_contract_canonical_span_order() {
    let (mut producer, consumer) =
        bounded_spsc::<EffectControlRecordV1>(depth(8), QueueGeneration(0)).expect("queue");
    let mut lane = EffectControlLane::new(consumer, false);
    // Deliberately reverse-ordered arrival.
    for record in [
        parameter(3, ParameterChannel::Left, 0.5),
        parameter(1, ParameterChannel::Right, 0.25),
        parameter(1, ParameterChannel::Left, 0.125),
    ] {
        producer.try_push(record).expect("room");
    }
    let mut staging = [idle(); 8];
    let staged = lane.stage(&mut staging, 4_096);
    assert_eq!(staged.staged, 3);
    assert_eq!(staged.dropped, 0);
    let keys: Vec<(u32, ParameterChannel)> = staging[..3]
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
    for span in &staging[..3] {
        assert_eq!(span.kind, AutomationSpanKind::Point);
        assert_eq!(span.start_sample, 4_096);
        assert_eq!(span.end_sample, 4_096);
        assert_eq!(
            span.start_value.to_bits(),
            span.end_value.to_bits(),
            "a Point span's endpoints are bit-identical, which is what every effect checks"
        );
    }
}

/// Red mutation: drop the `replace` branch so a repeated target inserts a second span -> the
/// staged count is 3 instead of 2 and the duplicate breaks every effect's "not a duplicate" rule.
#[test]
fn a_repeated_target_collapses_last_wins() {
    let (mut producer, consumer) =
        bounded_spsc::<EffectControlRecordV1>(depth(8), QueueGeneration(0)).expect("queue");
    let mut lane = EffectControlLane::new(consumer, false);
    for record in [
        parameter(2, ParameterChannel::Left, 1.0),
        parameter(0, ParameterChannel::Right, -1.0),
        parameter(2, ParameterChannel::Left, 7.0),
    ] {
        producer.try_push(record).expect("room");
    }
    let mut staging = [idle(); 4];
    let staged = lane.stage(&mut staging, 0);
    assert_eq!(staged.staged, 2);
    assert_eq!(staging[0].parameter_index, 0);
    assert_eq!(staging[1].parameter_index, 2);
    assert_eq!(staging[1].start_value, 7.0, "the last record wins");
}

/// A drain leaves the queue empty and the next block stages nothing, which is what makes the
/// browser host's in-flight count an exact free-slot count rather than an estimate.
#[test]
fn a_drained_queue_stages_nothing_on_the_next_block() {
    let (mut producer, consumer) =
        bounded_spsc::<EffectControlRecordV1>(depth(4), QueueGeneration(0)).expect("queue");
    let mut lane = EffectControlLane::new(consumer, false);
    producer
        .try_push(parameter(1, ParameterChannel::Left, 0.5))
        .expect("room");
    let mut staging = [idle(); 4];
    assert_eq!(lane.stage(&mut staging, 0).staged, 1);
    assert_eq!(lane.stage(&mut staging, 128).staged, 0);
}

/// Bypass is not a span: it is retained state that survives blocks with no traffic.
#[test]
fn bypass_is_retained_state_and_produces_no_span() {
    let (mut producer, consumer) =
        bounded_spsc::<EffectControlRecordV1>(depth(4), QueueGeneration(0)).expect("queue");
    let mut lane = EffectControlLane::new(consumer, false);
    assert!(!lane.bypassed());
    producer
        .try_push(EffectControlRecordV1::Bypass(true))
        .expect("room");
    let mut staging = [idle(); 4];
    assert_eq!(
        lane.stage(&mut staging, 0).staged,
        0,
        "bypass is not a span"
    );
    assert!(lane.bypassed());
    assert_eq!(lane.stage(&mut staging, 128).staged, 0);
    assert!(lane.bypassed(), "bypass survives a block with no traffic");
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

/// The staging window cannot be overrun: preparation caps the queue at the automation capacity,
/// and a violated cap is counted rather than written past the end.
#[test]
fn a_full_window_counts_the_overflow_instead_of_overrunning() {
    let (mut producer, consumer) =
        bounded_spsc::<EffectControlRecordV1>(depth(8), QueueGeneration(0)).expect("queue");
    let mut lane = EffectControlLane::new(consumer, false);
    for index in 0..5_u32 {
        producer
            .try_push(parameter(index, ParameterChannel::Left, index as f32))
            .expect("room");
    }
    let mut staging = [idle(); 2];
    let staged = lane.stage(&mut staging, 0);
    assert_eq!(staged.staged, 2);
    assert_eq!(staged.dropped, 3);
}
