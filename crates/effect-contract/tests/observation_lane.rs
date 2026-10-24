//! Issue #143 P2: the arm/disarm seam and the window accumulator.
//!
//! What is gated here is everything about a subscription that does **not** need a plan: that an
//! `Observe` record rides the lane's observation queue, in order, beside the cells and emits no
//! span, that arming is a flag rather than an allocation, that windows tile exactly, that the
//! declared fold is what produces a non-negative magnitude, and that an unarmed tap is never
//! read. The binding to a running plan is P3's.

#![allow(missing_docs)]

#[path = "support/lane_rig.rs"]
mod lane_rig;

use effect_contract::{
    EffectLaneShape, ObservationCadence, ObservationChannels, ObservationCost,
    ObservationDescriptor, ObservationFold, ObservationKind, ObservationLane, ObservationSample,
    ObservationTapId, ObservePushError, ParameterChannelPolicy, ParameterUnit,
};
use engine::realtime::{ObservationReader, observation_slot};
use lane_rig::{Rig, both};

const fn tap(id: u32, fold: ObservationFold) -> ObservationDescriptor {
    ObservationDescriptor {
        id: ObservationTapId(id),
        display_name: "Gain Reduction",
        display_unit: "dB",
        kind: match fold {
            ObservationFold::PeakSigned => ObservationKind::GainChangeDb,
            _ => ObservationKind::GainReductionDb,
        },
        unit: ParameterUnit::Db,
        cost: ObservationCost::Resident,
        cadence: ObservationCadence::PerBlock,
        fold,
        channels: ObservationChannels::PerLane,
        minimum: 0.0,
        maximum: 100.0,
    }
}

static MENU: [ObservationDescriptor; 3] = [
    tap(1, ObservationFold::PeakMagnitude),
    tap(2, ObservationFold::Latest),
    tap(3, ObservationFold::PeakSigned),
];

fn lane(default_window_blocks: u32) -> (ObservationLane, Vec<ObservationReader>) {
    let mut publishers = Vec::new();
    let mut readers = Vec::new();
    for _ in 0..MENU.len() {
        let (publisher, reader) = observation_slot();
        publishers.push(publisher);
        readers.push(reader);
    }
    (
        ObservationLane::new(&MENU, publishers, default_window_blocks).expect("one per tap"),
        readers,
    )
}

fn sample(left: f32, right: f32) -> ObservationSample {
    ObservationSample { left, right }
}

/// A shape with one `Shared` parameter, so a block has one span to stage beside the observations.
fn one_shared_parameter() -> EffectLaneShape {
    EffectLaneShape::from_live_parameters([(0, ParameterChannelPolicy::Shared)], 0)
}

/// An `Observe` record emits no span and does not consume staging capacity.
///
/// Red mutation: make `drain_observations` stage a span for the subscription -> the lane's span
/// count moves, or its staging window overflows.
#[test]
fn an_observe_record_rides_the_observation_queue_and_emits_no_span() {
    let mut rig = Rig::new(one_shared_parameter(), 2);
    let (mut observation, _readers) = lane(4);

    rig.commit(&[both(0, 0.25)]).expect("a cell");
    rig.writer.try_push_observe(0, true, 3).expect("room");
    rig.writer.try_push_observe(1, true, 0).expect("room");

    // Exactly one span of capacity: the parameter's cell is the only span a block stages.
    let staged = rig.stage();
    assert_eq!(staged.staged, 1, "one parameter, no observation spans");
    assert_eq!(rig.staging[0].parameter_index, 0);
    assert!(
        !observation.any_armed(),
        "the queue drains apart from the cells"
    );
    let unbound = rig.lane.drain_observations(512, Some(&mut observation));
    assert_eq!(unbound, 0);
    assert!(observation.is_armed(0) && observation.is_armed(1));
    assert!(observation.any_armed());
    // `window_blocks == 0` takes the plan default.
    assert_eq!(observation.default_window_blocks(), 4);
}

/// A plan with no observation capacity refuses the record instead of applying it silently.
#[test]
fn an_observe_record_against_an_incapable_plan_is_reported_unbound() {
    let mut rig = Rig::new(one_shared_parameter(), 4);
    rig.writer.try_push_observe(0, true, 1).expect("room");
    assert_eq!(
        rig.lane.drain_observations(0, None),
        1,
        "no capacity is refused, never ignored"
    );

    // And so is a tap index the menu does not have.
    let (mut observation, _readers) = lane(2);
    rig.writer.try_push_observe(7, true, 1).expect("room");
    assert_eq!(rig.lane.drain_observations(0, Some(&mut observation)), 1);
    assert!(!observation.any_armed());
}

/// Subscriptions are a FIFO, not a level: a later one never replaces an earlier one, they apply in
/// order, and a full queue refuses the newest and returns it, dropping nothing.
///
/// Test value: observation records collapsed like cell values (arm, disarm, re-arm folding to one
/// arm with the first window), or a full queue that overwrites its oldest record.
#[test]
fn subscriptions_apply_in_order_and_a_full_queue_refuses_the_newest() {
    let mut rig = Rig::new(one_shared_parameter(), 3);
    let (mut observation, _readers) = lane(4);
    assert_eq!(rig.writer.observe_capacity(), 3);
    rig.writer.try_push_observe(0, true, 2).expect("room");
    rig.writer.try_push_observe(0, false, 0).expect("room");
    rig.writer.try_push_observe(0, true, 7).expect("room");
    assert_eq!(rig.writer.observe_available_capacity(), 0);
    match rig.writer.try_push_observe(1, true, 1) {
        Err(ObservePushError::Full(full)) => assert_eq!(
            full.value,
            effect_contract::EffectControlRecord::Observe {
                tap_index: 1,
                armed: true,
                window_blocks: 1,
            },
            "the refused record is handed back"
        ),
        other => panic!("expected a full queue, got {other:?}"),
    }
    assert_eq!(rig.writer.observe_full_count(), 1);
    assert_eq!(rig.lane.drain_observations(0, Some(&mut observation)), 0);
    assert!(observation.is_armed(0), "arm, disarm, arm leaves it armed");
    assert_eq!(
        observation.window_blocks(0),
        Some(7),
        "the last record's window is the one in force"
    );
    assert!(!observation.is_armed(1), "the refused record never applied");
    assert_eq!(
        rig.writer.observe_available_capacity(),
        3,
        "the drain emptied the queue"
    );

    // The same three records in another order end the other way: order is what decides.
    rig.writer.try_push_observe(0, true, 5).expect("room");
    rig.writer.try_push_observe(0, true, 0).expect("room");
    rig.writer.try_push_observe(0, false, 9).expect("room");
    assert_eq!(rig.lane.drain_observations(0, Some(&mut observation)), 0);
    assert!(
        !observation.is_armed(0),
        "arm, arm, disarm leaves it disarmed"
    );
    assert_eq!(
        observation.window_blocks(0),
        Some(9),
        "and the last record's window again, a disarm's included"
    );
}

/// A lane with no observation queue answers a subscription with a typed error.
#[test]
fn a_lane_without_an_observation_queue_refuses_a_subscription() {
    let mut rig = Rig::new(one_shared_parameter(), 0);
    assert!(matches!(
        rig.writer.try_push_observe(0, true, 1),
        Err(ObservePushError::Unbound)
    ));
    assert_eq!(rig.writer.observe_capacity(), 0);
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

/// Issue #1918: `PeakSigned` publishes the value of largest magnitude with its sign, so a boost
/// and a cut of one size stay distinct, and of two equal magnitudes the earlier one stays.
///
/// Test value: red if the fold drops the sign (9.5), keeps the latest block (+4), takes the
/// signed maximum (+4), or lets a later equal magnitude replace the earlier one.
#[test]
fn the_signed_peak_fold_keeps_the_sign_of_the_largest_magnitude() {
    let (mut observation, readers) = lane(3);
    observation.arm(2, true, 3, 0);
    let readings = [(2.0, -1.0), (-9.5, 8.25), (4.0, -3.0)];
    for (index, (left, right)) in readings.into_iter().enumerate() {
        observation.accumulate(2, sample(left, right), index as u64 * 64, 64);
    }
    let first = readers[2].read().expect("first window");
    assert_eq!((first.left, first.right), (-9.5, 8.25));

    // A tie in magnitude keeps the earlier value, whichever its sign.
    for (index, (left, right)) in [(1.5, -1.5), (-1.5, 1.5), (0.5, 0.5)]
        .into_iter()
        .enumerate()
    {
        observation.accumulate(2, sample(left, right), 192 + index as u64 * 64, 64);
    }
    let second = readers[2].read().expect("second window");
    assert_eq!((second.left, second.right), (1.5, -1.5));
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

/// One publisher per declared tap, or nothing at all.
#[test]
fn a_lane_refuses_a_publisher_count_that_does_not_match_the_menu() {
    let (publisher, _reader) = observation_slot();
    assert!(ObservationLane::new(&MENU, vec![publisher], 4).is_none());
    assert!(ObservationLane::new(&MENU, Vec::new(), 4).is_none());
}

/// A plan swap exchanges two lanes' whole tap state (#1327 D2): the open window, the arm state
/// and the publisher, so the reader the host holds for the old lane receives the windows the
/// carried tap goes on to publish. Each lane keeps its own default window length.
///
/// Test value: a swap that copies the window but keeps the successor's publisher leaves the host's
/// reader on a dead cell; one that restarts the window loses the blocks already folded.
#[test]
fn a_lane_swap_carries_the_open_window_and_the_publisher() {
    let (mut old, old_readers) = lane(4);
    let (mut fresh, fresh_readers) = lane(8);
    assert!(old.arm(0, true, 2, 0));
    old.accumulate(0, sample(-3.0, 1.0), 0, 128);
    assert!(fresh.swap_state(&mut old));
    assert!(fresh.is_armed(0) && fresh.any_armed());
    assert!(!old.is_armed(0) && !old.any_armed());
    assert_eq!(
        (old.default_window_blocks(), fresh.default_window_blocks()),
        (4, 8)
    );

    fresh.accumulate(0, sample(-5.0, 0.5), 128, 128);
    let window = old_readers[0]
        .read()
        .expect("the window arrives on the reader of the lane it left");
    assert_eq!(
        (
            window.first_sample,
            window.end_sample,
            window.sequence,
            window.blocks,
            window.left,
            window.right
        ),
        (0, 256, 1, 2, 5.0, 1.0)
    );
    assert!(
        fresh_readers[0].read().is_none(),
        "the other reader sees nothing"
    );
}

/// Two lanes of different menus exchange nothing.
#[test]
fn a_lane_swap_refuses_another_menu() {
    static ONE_TAP: [ObservationDescriptor; 1] = [tap(1, ObservationFold::PeakMagnitude)];
    static FOLDED: [ObservationDescriptor; 3] = [
        tap(1, ObservationFold::PeakMagnitude),
        tap(2, ObservationFold::PeakMagnitude),
        tap(3, ObservationFold::PeakSigned),
    ];
    let (mut two, _readers) = lane(4);
    let (publisher, _reader) = observation_slot();
    let mut one = ObservationLane::new(&ONE_TAP, vec![publisher], 4).expect("lane");
    let mut publishers = Vec::new();
    for _ in 0..FOLDED.len() {
        publishers.push(observation_slot().0);
    }
    let mut folded = ObservationLane::new(&FOLDED, publishers, 4).expect("lane");
    assert!(two.arm(0, true, 1, 0));
    assert!(!two.swap_state(&mut one), "another tap count");
    assert!(!two.swap_state(&mut folded), "another fold at tap 1");
    assert!(two.is_armed(0), "nothing was exchanged");
    assert!(!one.is_armed(0) && !folded.is_armed(0));
}
