//! Meter taps: window boundaries, discontinuities, resets, drops, and the segment law.
//!
//! The meter is the one place in this crate that still runs a scalar sample loop, because its
//! energy accumulation is sequential `f64` and its held-peak decay is a counter state machine.
//! #85 changed the *shape* -- the observation is split at window boundaries, the configuration is
//! hoisted out of `MeterConfig` before the loop, and `sqrt` runs once per emitted window -- and
//! deliberately not the arithmetic: every value below is class A and its JSON fixtures are
//! byte-identical.

use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};

use miso_engine_builtins::*;

#[test]
fn meter_windows_are_exact() {
    let handle = MeterHandle(NonZeroU64::new(1).expect("constant"));
    let config = MeterConfig {
        period_frames: NonZeroU32::new(2).expect("constant"),
        peak_hold_frames: 0,
        peak_decay_db_per_second: 0.0,
        queue_capacity: NonZeroUsize::new(2).expect("constant"),
        reset_generation: 7,
    };
    let PreparedMeter {
        mut accumulator,
        mut consumer,
    } = MeterAccumulator::prepare(handle, config, 48_000).expect("meter");
    accumulator
        .observe(&[1.0, 0.5], &[0.0, -1.0], 3)
        .expect("matched meter lanes");
    let snap = consumer.try_pop().expect("snapshot");
    assert_eq!(snap.start_sample, 3);
    assert_eq!(snap.end_sample, 5);
    assert_eq!(snap.left.clipped_samples, 1);
    assert_eq!(snap.right.clipped_samples, 1);
}

#[test]
fn meter_windows_discontinuities_resets_and_drops_are_exact() {
    let handle = MeterHandle(NonZeroU64::new(1).expect("constant"));
    let config = MeterConfig {
        period_frames: NonZeroU32::new(2).expect("constant"),
        peak_hold_frames: 1,
        peak_decay_db_per_second: 0.0,
        queue_capacity: NonZeroUsize::new(1).expect("constant"),
        reset_generation: 9,
    };
    let PreparedMeter {
        mut accumulator,
        mut consumer,
    } = MeterAccumulator::prepare(handle, config, 48_000).expect("meter");
    assert_eq!(
        accumulator.observe(&[0.5], &[f32::NAN, 0.0], 0),
        Err(MeterObservationError::LaneLength)
    );
    accumulator
        .observe(&[1.0, 0.0], &[0.25, -0.25], 4)
        .expect("first window");
    let first = consumer.try_pop().expect("first snapshot");
    assert_eq!(
        (first.start_sample, first.end_sample, first.frames),
        (4, 6, 2)
    );
    assert_eq!(first.left.energy, 1.0);
    assert!((first.left.rms - 1.0 / 2.0_f64.sqrt()).abs() <= f64::EPSILON);
    assert_eq!(first.left.held_peak, 1.0);
    accumulator
        .observe(&[0.0], &[0.0], 9)
        .expect("discontinuity");
    accumulator
        .observe(&[0.0], &[0.0], 10)
        .expect("second window");
    let second = consumer.try_pop().expect("second snapshot");
    assert_eq!((second.start_sample, second.end_sample), (9, 11));
    assert_eq!(second.cumulative_discontinuities, 1);
    accumulator
        .observe(&[0.0, 0.0], &[0.0, 0.0], 11)
        .expect("queued snapshot");
    accumulator
        .observe(&[0.0, 0.0], &[0.0, 0.0], 13)
        .expect("dropped snapshot");
    let queued = consumer.try_pop().expect("queued snapshot");
    assert_eq!(queued.cumulative_dropped_snapshots, 0);
    accumulator
        .observe(&[0.0, 0.0], &[0.0, 0.0], 15)
        .expect("post-drop snapshot");
    let post_drop = consumer.try_pop().expect("post-drop snapshot");
    assert_eq!(post_drop.cumulative_dropped_snapshots, 1);
    accumulator.reset(BuiltinResetKind::DiscontinuityKeepTargets);
    accumulator
        .observe(&[0.0, 0.0], &[0.0, 0.0], 17)
        .expect("reset window");
    let reset = consumer.try_pop().expect("reset snapshot");
    assert_eq!(reset.window_sequence, 5);
    assert_eq!(reset.cumulative_dropped_snapshots, 1);
    accumulator.reset(BuiltinResetKind::FullToPrepared);
    accumulator
        .observe(&[0.0, 0.0], &[0.0, 0.0], 19)
        .expect("full reset window");
    let full_reset = consumer.try_pop().expect("full reset snapshot");
    assert_eq!(full_reset.window_sequence, 0);
    assert_eq!(full_reset.cumulative_dropped_snapshots, 0);
    assert_eq!(full_reset.cumulative_discontinuities, 0);
}

#[test]
fn ten_thousand_deterministic_meter_mutations_remain_bounded_and_finite() {
    let handle = MeterHandle(NonZeroU64::new(1).expect("constant"));
    let mut state = 0x4d45_5445_525f_3031_u64;
    for iteration in 0..10_000_u64 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let period = NonZeroU32::new(((state as u32) & 7) + 1).expect("nonzero");
        let capacity = NonZeroUsize::new((((state >> 8) as usize) & 3) + 1).expect("nonzero");
        let PreparedMeter {
            mut accumulator,
            mut consumer,
        } = MeterAccumulator::prepare(
            handle,
            MeterConfig {
                period_frames: period,
                peak_hold_frames: ((state >> 16) as u32) & 15,
                peak_decay_db_per_second: ((state >> 32) as f32 / u32::MAX as f32) * 120.0,
                queue_capacity: capacity,
                reset_generation: iteration,
            },
            48_000,
        )
        .expect("generated meter config");
        let frames = usize::try_from(period.get()).expect("small period") * 2;
        let mut left = [0.0_f32; 16];
        let mut right = [0.0_f32; 16];
        for index in 0..frames {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            left[index] = if state & 31 == 0 {
                f32::NAN
            } else {
                ((state as i32) as f32) / i32::MAX as f32
            };
            right[index] = if state & 63 == 0 {
                f32::INFINITY
            } else {
                (((state >> 32) as i32) as f32) / i32::MAX as f32
            };
        }
        accumulator
            .observe(
                &left[..frames],
                &right[..frames],
                iteration.saturating_mul(32),
            )
            .expect("matching meter lanes");
        while let Ok(snapshot) = consumer.try_pop() {
            assert_eq!(snapshot.frames, period.get());
            assert!(snapshot.left.energy.is_finite());
            assert!(snapshot.right.energy.is_finite());
            assert!(snapshot.left.rms.is_finite());
            assert!(snapshot.right.rms.is_finite());
            assert!(snapshot.left.sample_peak.is_finite());
            assert!(snapshot.right.sample_peak.is_finite());
        }
    }
}

/// T10: the observation is split at window boundaries, and where it is split changes nothing.
///
/// The pre-#85 loop tested the period after every sample; the new one takes whole segments. This
/// renders three full windows through every partition and compares the emitted snapshots word for
/// word -- peak, energy, RMS, held peak, clipped and sanitised counts, and the cumulative
/// counters, which is everything a snapshot carries.
#[test]
fn meter_segment_law_is_exact() {
    const PERIOD: u32 = 96;
    const FRAMES: usize = PERIOD as usize * 3;

    let signal = |channel: usize| -> Vec<f32> {
        let mut state = 0x51ED_0000_u64 ^ channel as u64;
        (0..FRAMES)
            .map(|index| {
                state ^= state >> 12;
                state ^= state << 25;
                state ^= state >> 27;
                let word = (state.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 32) as u32;
                match index % 23 {
                    0 => f32::NAN,
                    5 => f32::from_bits(1),
                    9 => 1.5,
                    13 => -1.0,
                    _ => (word as i32 as f32) / 1_073_741_824.0,
                }
            })
            .collect()
    };
    let left = signal(0);
    let right = signal(1);

    let observe = |quanta: &[usize]| -> Vec<MeterSnapshot> {
        let config = MeterConfig {
            period_frames: NonZeroU32::new(PERIOD).expect("constant"),
            peak_hold_frames: 17,
            peak_decay_db_per_second: 24.0,
            queue_capacity: NonZeroUsize::new(8).expect("constant"),
            reset_generation: 3,
        };
        let PreparedMeter {
            mut accumulator,
            mut consumer,
        } = MeterAccumulator::prepare(
            MeterHandle(NonZeroU64::new(4).expect("constant")),
            config,
            48_000,
        )
        .expect("meter");
        let mut start = 0;
        let mut index = 0;
        while start < FRAMES {
            let end = (start + quanta[index % quanta.len()]).min(FRAMES);
            accumulator
                .observe(&left[start..end], &right[start..end], start as u64)
                .expect("observation");
            start = end;
            index += 1;
        }
        let mut snapshots = Vec::new();
        while let Ok(snapshot) = consumer.try_pop() {
            snapshots.push(snapshot);
        }
        snapshots
    };

    let oracle = observe(&[FRAMES]);
    assert_eq!(oracle.len(), 3, "three whole windows");
    for quanta in [
        vec![1_usize],
        vec![7],
        vec![64],
        vec![PERIOD as usize],
        vec![1, 7, 64, 128],
        vec![95, 1, 200],
    ] {
        let actual = observe(&quanta);
        assert_eq!(actual.len(), oracle.len(), "quanta={quanta:?}");
        for (actual, expected) in actual.iter().zip(&oracle) {
            assert_eq!(actual, expected, "quanta={quanta:?}");
            for (actual, expected) in [(actual.left, expected.left), (actual.right, expected.right)]
            {
                assert_eq!(actual.sample_peak.to_bits(), expected.sample_peak.to_bits());
                assert_eq!(actual.energy.to_bits(), expected.energy.to_bits());
                assert_eq!(actual.rms.to_bits(), expected.rms.to_bits());
                assert_eq!(actual.held_peak.to_bits(), expected.held_peak.to_bits());
            }
        }
    }
    assert!(
        oracle.iter().any(
            |snapshot| snapshot.left.clipped_samples > 0 && snapshot.left.sanitized_samples > 0
        ),
        "the corpus must exercise both counters"
    );
}

/// T10: the peak is the D8 select form, which pins the `+/-0.0` case `f32::max` leaves open.
///
/// `f32::max(-0.0, +0.0)` may return either sign; `select(a > p, a, p)` returns the running peak
/// unless the new magnitude is strictly greater, and a magnitude is never negative, so the peak of
/// an all-zero window is exactly `+0.0` on every target.
#[test]
fn meter_peak_of_signed_zeros_is_positive_zero() {
    let config = MeterConfig {
        period_frames: NonZeroU32::new(4).expect("constant"),
        peak_hold_frames: 0,
        peak_decay_db_per_second: 0.0,
        queue_capacity: NonZeroUsize::new(2).expect("constant"),
        reset_generation: 0,
    };
    let PreparedMeter {
        mut accumulator,
        mut consumer,
    } = MeterAccumulator::prepare(
        MeterHandle(NonZeroU64::new(1).expect("constant")),
        config,
        48_000,
    )
    .expect("meter");
    accumulator
        .observe(&[-0.0, 0.0, -0.0, 0.0], &[0.0, -0.0, 0.0, -0.0], 0)
        .expect("observation");
    let snapshot = consumer.try_pop().expect("snapshot");
    assert_eq!(snapshot.left.sample_peak.to_bits(), 0.0_f32.to_bits());
    assert_eq!(snapshot.right.sample_peak.to_bits(), 0.0_f32.to_bits());
    assert_eq!(snapshot.left.held_peak.to_bits(), 0.0_f32.to_bits());
}
