//! Issue #143 P2: the arm/disarm seam and the window accumulator.
//!
//! What is gated here is everything about a subscription that does **not** need a plan: that an
//! `Observe` record rides the parameter queue and emits no span, that arming is a flag rather than
//! an allocation, that windows tile exactly, that the declared fold is what produces a non-negative
//! magnitude, and that an unarmed tap is never read. The binding to a running plan is P3's.

#![allow(missing_docs)]

use miso_engine_core::realtime::{
    ObservationReaderV1, QueueGeneration, bounded_spsc, observation_slot,
};
use miso_engine_effect_contract::{
    AutomationSpanKind, EffectControlLane, EffectControlRecordV1, ObservationCadenceV1,
    ObservationChannelsV1, ObservationCostV1, ObservationDescriptorV1, ObservationFoldV1,
    ObservationKindV1, ObservationLaneV1, ObservationSampleV1, ObservationTapId, ParameterChannel,
    ParameterUnit, PreparedAutomationSpan,
};

const fn tap(id: u32, fold: ObservationFoldV1) -> ObservationDescriptorV1 {
    ObservationDescriptorV1 {
        id: ObservationTapId(id),
        display_name: "Gain Reduction",
        display_unit: "dB",
        kind: ObservationKindV1::GainReductionDb,
        unit: ParameterUnit::Db,
        cost: ObservationCostV1::Resident,
        cadence: ObservationCadenceV1::PerBlock,
        fold,
        channels: ObservationChannelsV1::PerLane,
        minimum: 0.0,
        maximum: 100.0,
    }
}

static MENU: [ObservationDescriptorV1; 2] = [
    tap(1, ObservationFoldV1::PeakMagnitude),
    tap(2, ObservationFoldV1::Latest),
];

fn lane(default_window_blocks: u32) -> (ObservationLaneV1, Vec<ObservationReaderV1>) {
    let mut publishers = Vec::new();
    let mut readers = Vec::new();
    for _ in 0..MENU.len() {
        let (publisher, reader) = observation_slot();
        publishers.push(publisher);
        readers.push(reader);
    }
    (
        ObservationLaneV1::new(&MENU, publishers, default_window_blocks).expect("one per tap"),
        readers,
    )
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

fn depth(value: usize) -> core::num::NonZeroUsize {
    core::num::NonZeroUsize::new(value).expect("nonzero")
}

fn sample(left: f32, right: f32) -> ObservationSampleV1 {
    ObservationSampleV1 { left, right }
}

/// An `Observe` record emits no span and does not consume staging capacity.
///
/// Red mutation: make the `Observe` arm fall through to the span builder -> `staged` becomes 3 and
/// a parameter is displaced from a full window.
#[test]
fn an_observe_record_rides_the_parameter_queue_and_emits_no_span() {
    let (mut producer, consumer) =
        bounded_spsc::<EffectControlRecordV1>(depth(8), QueueGeneration(0)).expect("queue");
    let mut control = EffectControlLane::new(consumer, false);
    let (mut observation, _readers) = lane(4);

    producer
        .try_push(EffectControlRecordV1::Parameter {
            parameter_index: 0,
            channel: ParameterChannel::Both,
            value: 0.25,
        })
        .expect("room");
    producer
        .try_push(EffectControlRecordV1::Observe {
            tap_index: 0,
            armed: true,
            window_blocks: 3,
        })
        .expect("room");
    producer
        .try_push(EffectControlRecordV1::Observe {
            tap_index: 1,
            armed: true,
            window_blocks: 0,
        })
        .expect("room");

    // Exactly one span of capacity: an `Observe` that consumed staging would overflow it.
    let mut staging = [idle(); 1];
    let staged = control.stage(&mut staging, 512, Some(&mut observation));
    assert_eq!(staged.staged, 1, "one parameter, no observation spans");
    assert_eq!(staged.dropped, 0, "the observations did not crowd it out");
    assert_eq!(staged.unbound, 0);
    assert_eq!(staging[0].parameter_index, 0);
    assert!(observation.is_armed(0) && observation.is_armed(1));
    assert!(observation.any_armed());
    // `window_blocks == 0` takes the plan default.
    assert_eq!(observation.default_window_blocks(), 4);
}

/// A plan with no observation capacity refuses the record instead of applying it silently.
#[test]
fn an_observe_record_against_an_incapable_plan_is_reported_unbound() {
    let (mut producer, consumer) =
        bounded_spsc::<EffectControlRecordV1>(depth(4), QueueGeneration(0)).expect("queue");
    let mut control = EffectControlLane::new(consumer, false);
    producer
        .try_push(EffectControlRecordV1::Observe {
            tap_index: 0,
            armed: true,
            window_blocks: 1,
        })
        .expect("room");
    let mut staging = [idle(); 4];
    let staged = control.stage(&mut staging, 0, None);
    assert_eq!(staged.staged, 0);
    assert_eq!(staged.unbound, 1, "no capacity is refused, never ignored");

    // And so is a tap index the menu does not have.
    let (mut observation, _readers) = lane(2);
    producer
        .try_push(EffectControlRecordV1::Observe {
            tap_index: 7,
            armed: true,
            window_blocks: 1,
        })
        .expect("room");
    let staged = control.stage(&mut staging, 0, Some(&mut observation));
    assert_eq!(staged.unbound, 1);
    assert!(!observation.any_armed());
}

/// An unarmed tap is never read: `wants` is the whole of level-2 zero.
#[test]
fn an_unarmed_tap_is_never_read_and_publishes_nothing() {
    let (mut observation, readers) = lane(2);
    assert!(!observation.wants(0));
    assert!(!observation.wants(1));
    assert!(!observation.wants(9), "an absent tap is not wanted either");
    // Even if a caller ignores `wants` and folds anyway, an unarmed tap stores nothing.
    observation.accumulate(0, sample(-6.0, -6.0), 0, 128);
    observation.accumulate(0, sample(-6.0, -6.0), 128, 128);
    assert_eq!(readers[0].read(), None, "nothing was published");
}

/// Windows tile with no gap, close at exactly `window_blocks`, and fold by the declared rule.
///
/// Red mutation: reset `first_sample` to `0` instead of to `end_sample` on close -> the second
/// window no longer begins where the first ended.
#[test]
fn windows_tile_exactly_and_fold_by_the_declared_rule() {
    let (mut observation, readers) = lane(2);
    observation.arm(0, true, 3, 1_024);

    // Three blocks of 128 frames: the deepest magnitude wins, and the sign is folded away.
    let readings = [(-2.0, -1.0), (-9.5, -3.0), (-4.0, -8.25)];
    for (index, (left, right)) in readings.into_iter().enumerate() {
        assert!(observation.wants(0));
        let first_sample = 1_024 + index as u64 * 128;
        observation.accumulate(0, sample(left, right), first_sample, 128);
        if index < 2 {
            assert_eq!(readers[0].read(), None, "the window has not closed yet");
        }
    }
    let first = readers[0].read().expect("first window");
    assert_eq!(first.sequence, 1);
    assert_eq!(first.blocks, 3);
    assert_eq!(first.first_sample, 1_024);
    assert_eq!(first.end_sample, 1_024 + 384);
    assert_eq!(first.left, 9.5, "max(|x|) over the window, non-negative");
    assert_eq!(first.right, 8.25);

    // The next window starts exactly where the last one ended, and the accumulator was cleared.
    for index in 3..6 {
        let first_sample = 1_024 + index as u64 * 128;
        observation.accumulate(0, sample(-1.5, -0.5), first_sample, 128);
    }
    let second = readers[0].read().expect("second window");
    assert_eq!(second.sequence, 2);
    assert_eq!(second.first_sample, first.end_sample, "windows tile");
    assert_eq!(second.end_sample, 1_024 + 768);
    assert_eq!(second.left, 1.5, "the peak did not carry over");
}

/// `Latest` overwrites rather than folding, and keeps the effect's own sign.
#[test]
fn the_latest_fold_publishes_the_last_block_verbatim() {
    let (mut observation, readers) = lane(1);
    observation.arm(1, true, 2, 0);
    observation.accumulate(1, sample(-9.0, 9.0), 0, 64);
    observation.accumulate(1, sample(-1.0, 1.0), 64, 64);
    let window = readers[1].read().expect("window");
    assert_eq!(window.left, -1.0, "latest, not the peak");
    assert_eq!(window.right, 1.0);
    assert_eq!(window.blocks, 2);
}

/// Re-arming is idempotent except that the newer window length wins and the window restarts.
#[test]
fn a_second_subscribe_takes_the_newer_window_length() {
    let (mut observation, readers) = lane(8);
    observation.arm(0, true, 4, 0);
    observation.accumulate(0, sample(-3.0, -3.0), 0, 128);
    // Re-subscribe at a shorter window: the half-filled window is abandoned, not published.
    observation.arm(0, true, 2, 128);
    assert_eq!(readers[0].read(), None);
    observation.accumulate(0, sample(-1.0, -1.0), 128, 128);
    observation.accumulate(0, sample(-2.0, -2.0), 256, 128);
    let window = readers[0].read().expect("window");
    assert_eq!(window.blocks, 2, "the newer length won");
    assert_eq!(window.first_sample, 128, "and the window restarted");
    assert_eq!(window.left, 2.0, "the abandoned block did not leak in");
}

/// Issue #143 D7: dropping every subscription is one operation with one meaning.
#[test]
fn disarming_all_stops_every_tap_without_disturbing_published_windows() {
    let (mut observation, readers) = lane(1);
    observation.arm(0, true, 1, 0);
    observation.accumulate(0, sample(-5.0, -5.0), 0, 128);
    let published = readers[0].read().expect("window");
    assert_eq!(published.left, 5.0);

    observation.disarm_all();
    assert!(!observation.any_armed());
    assert!(!observation.wants(0));
    observation.accumulate(0, sample(-50.0, -50.0), 128, 128);
    let after = readers[0]
        .read()
        .expect("the last window is still readable");
    assert_eq!(after, published, "a disarm publishes nothing new");
}

/// Retained bytes are a formula over the declared menu, and arming does not change them.
#[test]
fn retained_bytes_are_a_formula_over_the_declared_menu() {
    let (mut observation, _readers) = lane(4);
    let before = observation.retained_bytes();
    assert_eq!(observation.len(), MENU.len());
    assert!(!observation.is_empty());
    assert!(before > 0);
    observation.arm(0, true, 2, 0);
    observation.arm(1, true, 2, 0);
    assert_eq!(
        observation.retained_bytes(),
        before,
        "subscribe allocates nothing: the slots came from the declared menu"
    );
    observation.disarm_all();
    assert_eq!(observation.retained_bytes(), before);
}

/// One publisher per declared tap, or nothing at all.
#[test]
fn a_lane_refuses_a_publisher_count_that_does_not_match_the_menu() {
    let (publisher, _reader) = observation_slot();
    assert!(ObservationLaneV1::new(&MENU, vec![publisher], 4).is_none());
    assert!(ObservationLaneV1::new(&MENU, Vec::new(), 4).is_none());
}
