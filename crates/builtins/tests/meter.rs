//! Meter taps: window boundaries, discontinuities, resets, drops, and the segment law.
//!
//! The meter is the one place in this crate that still runs a scalar sample loop, because its
//! energy accumulation is sequential `f64` and its held-peak decay is a counter state machine.
//! #85 changed the *shape* -- the observation is split at window boundaries, the configuration is
//! hoisted out of `MeterConfig` before the loop, and `sqrt` runs once per emitted window -- and
//! deliberately not the arithmetic: every value below is class A and its JSON fixtures are
//! byte-identical.

use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};

use builtins::*;

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

/// Issue #163 phase 4 item 3: the settled-silence early-out is bit-identical to the sample loop.
///
/// The skip is only sound while `held == 0.0` and `hold_remaining` is already at the configured
/// hold length. This drives both sides of that boundary in one accumulator and pins every field a
/// skipped segment must still produce: the window still tiles, the sequence still advances, the
/// retained peak from an earlier window is not disturbed, and both signed zeros are admitted.
///
/// Red mutation this holds against: skip the whole `while` body rather than just
/// `observe_segment` -> `self.frames` stops advancing, the period boundary is never reached and
/// no window is ever emitted, so every `try_pop` below returns `None`.
///
/// It deliberately does **not** claim the `held == 0.0` precondition: with decay disabled, as it
/// is here, a silent segment leaves a nonzero `held` alone anyway, so dropping that term changes
/// no bit in this configuration. `silence_against_a_nonzero_held_peak_still_decays` is the test
/// that makes that term load-bearing, and it is red without it.
#[test]
fn a_settled_silent_block_is_bit_identical_through_the_early_out() {
    let handle = MeterHandle(NonZeroU64::new(1).expect("constant"));
    let config = MeterConfig {
        period_frames: NonZeroU32::new(4).expect("constant"),
        peak_hold_frames: 0,
        peak_decay_db_per_second: 0.0,
        queue_capacity: NonZeroUsize::new(8).expect("constant"),
        reset_generation: 0,
    };
    let PreparedMeter {
        mut accumulator,
        mut consumer,
    } = MeterAccumulator::prepare(handle, config, 48_000).expect("meter");

    // Window 0: exact positive zeros. The lane starts settled, so this is the skipped path.
    accumulator
        .observe(&[0.0; 4], &[0.0; 4], 0)
        .expect("matched meter lanes");
    let snap = consumer
        .try_pop()
        .expect("a skipped block still closes its window");
    assert_eq!(
        snap.start_sample, 0,
        "the window still tiles from the start"
    );
    assert_eq!(
        snap.end_sample, 4,
        "a skipped block still advances `frames`"
    );
    assert_eq!(snap.left.sample_peak, 0.0);
    assert_eq!(snap.left.energy, 0.0);
    assert_eq!(snap.left.rms, 0.0);
    assert_eq!(snap.left.held_peak, 0.0);
    assert_eq!(snap.left.clipped_samples, 0);
    assert_eq!(snap.left.sanitized_samples, 0);

    // Window 1: negative zeros. `(-0.0).abs()` is `+0.0` and `(-0.0) * (-0.0)` is `+0.0`, so the
    // `== 0.0` admission test covers exactly the values the proof covers.
    accumulator
        .observe(&[-0.0; 4], &[-0.0; 4], 4)
        .expect("matched meter lanes");
    let snap = consumer.try_pop().expect("snapshot");
    assert_eq!(snap.start_sample, 4);
    assert_eq!(snap.end_sample, 8);
    assert_eq!(snap.left.sample_peak, 0.0, "a negative zero is not a peak");
    assert_eq!(
        snap.left.energy, 0.0,
        "a negative zero contributes no energy"
    );

    // Window 2: real signal. The early-out must not fire, and `all` short-circuits on frame 0.
    accumulator
        .observe(&[0.5, 0.0, 0.0, 0.0], &[0.0, 0.0, 0.0, 0.25], 8)
        .expect("matched meter lanes");
    let snap = consumer.try_pop().expect("snapshot");
    assert_eq!(snap.left.sample_peak, 0.5, "signal still meters");
    assert_eq!(snap.left.energy, 0.25);
    assert_eq!(snap.right.sample_peak, 0.25);

    // Window 3: silence again, but `held` is now `0.5`, so the lane is *not* settled and the
    // sample loop must run. With decay disabled the held peak is retained rather than decayed.
    accumulator
        .observe(&[0.0; 4], &[0.0; 4], 12)
        .expect("matched meter lanes");
    let snap = consumer.try_pop().expect("snapshot");
    assert_eq!(snap.start_sample, 12);
    assert_eq!(snap.end_sample, 16);
    assert_eq!(
        snap.left.held_peak, 0.5,
        "the retained held peak survives a silent window unchanged"
    );
    assert_eq!(
        snap.left.sample_peak, 0.0,
        "`sample_peak` is per window and `emit` cleared it; `held_peak` is the retained one"
    );
    assert_eq!(snap.left.energy, 0.0, "the silent window adds no energy");
}

/// The decaying-hold half of the #163 phase 4 item 3 boundary: silence against a *nonzero* held
/// peak is not a no-op, and the early-out must not claim it.
///
/// With decay enabled, every silent sample runs `held = flush_subnormal(held * decay)`. That is
/// the one case where a lane is silent and settled-looking but its state still moves, so it is the
/// case that decides whether `held == 0.0` belongs in the precondition.
///
/// Red mutation: drop `self.left.held == 0.0 && self.right.held == 0.0` from `settled_silence` ->
/// the decay stops running the moment the signal does, and `held_peak` below stays at `1.0`
/// forever instead of decaying toward zero.
#[test]
fn silence_against_a_nonzero_held_peak_still_decays() {
    let handle = MeterHandle(NonZeroU64::new(1).expect("constant"));
    let config = MeterConfig {
        period_frames: NonZeroU32::new(4).expect("constant"),
        peak_hold_frames: 0,
        peak_decay_db_per_second: 120.0,
        queue_capacity: NonZeroUsize::new(8).expect("constant"),
        reset_generation: 0,
    };
    let PreparedMeter {
        mut accumulator,
        mut consumer,
    } = MeterAccumulator::prepare(handle, config, 48_000).expect("meter");

    accumulator
        .observe(&[1.0, 0.0, 0.0, 0.0], &[1.0, 0.0, 0.0, 0.0], 0)
        .expect("matched meter lanes");
    let first = consumer.try_pop().expect("snapshot");
    assert_eq!(first.left.sample_peak, 1.0);
    let held_after_signal = first.left.held_peak;
    assert!(
        held_after_signal < 1.0 && held_after_signal > 0.0,
        "the three silent samples of window 0 already decay the hold: {held_after_signal}"
    );

    // A fully silent window against a nonzero `held`. The sample loop must run.
    accumulator
        .observe(&[0.0; 4], &[0.0; 4], 4)
        .expect("matched meter lanes");
    let second = consumer.try_pop().expect("snapshot");
    assert!(
        second.left.held_peak < held_after_signal,
        "a silent window against a nonzero held peak keeps decaying it: \
         {} is not below {held_after_signal}",
        second.left.held_peak
    );
    assert_eq!(second.left.energy, 0.0, "and still adds no energy");
}

fn assert_snapshot_bits(actual: MeterSnapshot, expected: MeterSnapshot) {
    assert_eq!(actual, expected);
    for (a, b) in [(actual.left, expected.left), (actual.right, expected.right)] {
        assert_eq!(a.sample_peak.to_bits(), b.sample_peak.to_bits());
        assert_eq!(a.rms.to_bits(), b.rms.to_bits());
        assert_eq!(a.energy.to_bits(), b.energy.to_bits());
        assert_eq!(a.held_peak.to_bits(), b.held_peak.to_bits());
    }
}

#[test]
fn resident_meter_matches_planar_bits_for_every_selection_stride_and_state_transition() {
    let hostile = [
        0.0,
        -0.0,
        0.25,
        -0.75,
        1.0,
        -1.5,
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::from_bits(1),
        f32::from_bits(0x8000_0001),
        0.03125,
    ];
    for stride in [1, 4, 8] {
        for lane in 0..stride {
            for selection in 1..=MeterMetricSet::ALL.bits() {
                let config = MeterConfig {
                    period_frames: NonZeroU32::new(5).unwrap(),
                    peak_hold_frames: 2,
                    peak_decay_db_per_second: 120.0,
                    queue_capacity: NonZeroUsize::new(2).unwrap(),
                    reset_generation: 714,
                };
                let prepare = || {
                    MeterAccumulator::prepare_selected(
                        MeterHandle(NonZeroU64::new(1).unwrap()),
                        config,
                        48_000,
                        MeterMetricSet::from_bits_retain(selection),
                    )
                    .unwrap()
                };
                let mut planar = prepare();
                let mut resident = prepare();
                let mut time = 3_u64;
                for block in 0..18 {
                    if block == 7 {
                        time += 17;
                    }
                    if block == 10 || block == 13 {
                        let reset = if block == 10 {
                            BuiltinResetKind::DiscontinuityKeepTargets
                        } else {
                            BuiltinResetKind::FullToPrepared
                        };
                        planar.accumulator.reset(reset);
                        resident.accumulator.reset(reset);
                    }
                    let frames = [0, 1, 4, 13, 2, 5][block % 6];
                    let left: Vec<_> = (0..frames)
                        .map(|f| {
                            if block > 13 {
                                -0.0
                            } else {
                                hostile[(f + block + lane) % hostile.len()]
                            }
                        })
                        .collect();
                    let right: Vec<_> = (0..frames)
                        .map(|f| {
                            if block > 13 {
                                0.0
                            } else {
                                hostile[(f * 3 + lane + 1) % hostile.len()]
                            }
                        })
                        .collect();
                    // Unselected lanes and the unused capacity behind the validated view are poison.
                    let mut bank_left = vec![f32::from_bits(0x7fc0_0714); (frames + 3) * stride];
                    let mut bank_right = vec![f32::NEG_INFINITY; (frames + 3) * stride];
                    for f in 0..frames {
                        bank_left[f * stride + lane] = left[f];
                        bank_right[f * stride + lane] = right[f];
                    }
                    let input = MeterInput::strided(
                        &bank_left[..frames * stride],
                        &bank_right[..frames * stride],
                        frames,
                        stride,
                        lane,
                    )
                    .unwrap();
                    assert_eq!(
                        resident.accumulator.observe_input(input, time),
                        planar.accumulator.observe(&left, &right, time),
                    );
                    time += frames as u64;
                    // Delayed drains force queue overflow as well as partial/multiple windows.
                    if block % 3 == 2 || block == 17 {
                        loop {
                            match (resident.consumer.try_pop(), planar.consumer.try_pop()) {
                                (Ok(a), Ok(b)) => assert_snapshot_bits(a, b),
                                (Err(a), Err(b)) => {
                                    assert_eq!(a, b);
                                    break;
                                }
                                _ => panic!("publication count differs"),
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn resident_meter_shape_and_time_errors_precede_mutation_and_empty_input_matches_planar() {
    for (left, right, frames, stride, lane) in [
        (&[][..], &[][..], 0, 0, 0),
        (&[][..], &[][..], 0, 4, 4),
        (&[][..], &[][..], usize::MAX, 8, 0),
        (&[0.0][..], &[][..], 1, 1, 0),
        (&[0.0][..], &[0.0][..], 0, 1, 0),
        (&[0.0][..], &[0.0, 0.0][..], 1, 1, 0),
    ] {
        assert!(matches!(
            MeterInput::strided(left, right, frames, stride, lane),
            Err(MeterObservationError::LaneLength)
        ));
    }
    let config = MeterConfig {
        period_frames: NonZeroU32::new(2).unwrap(),
        peak_hold_frames: 0,
        peak_decay_db_per_second: 0.0,
        queue_capacity: NonZeroUsize::new(4).unwrap(),
        reset_generation: 1,
    };
    let prepare = || {
        MeterAccumulator::prepare(MeterHandle(NonZeroU64::new(1).unwrap()), config, 48_000).unwrap()
    };
    let mut a = prepare();
    let mut b = prepare();
    a.accumulator.observe(&[0.5], &[-0.25], 3).unwrap();
    b.accumulator.observe(&[0.5], &[-0.25], 3).unwrap();
    assert_eq!(
        a.accumulator.observe(&[1.0], &[], u64::MAX),
        Err(MeterObservationError::LaneLength)
    );
    let input = MeterInput::strided(&[0.0; 8], &[1.0; 8], 1, 8, 7).unwrap();
    assert_eq!(
        a.accumulator.observe_input(input, u64::MAX),
        Err(MeterObservationError::SampleTimeOverflow)
    );
    for meter in [&mut a, &mut b] {
        meter.accumulator.observe(&[0.125], &[-0.5], 4).unwrap();
    }
    assert_snapshot_bits(a.consumer.try_pop().unwrap(), b.consumer.try_pop().unwrap());
    a.accumulator
        .observe_input(MeterInput::strided(&[], &[], 0, 8, 7).unwrap(), u64::MAX)
        .unwrap();
    b.accumulator.observe(&[], &[], u64::MAX).unwrap();
    for meter in [&mut a, &mut b] {
        meter
            .accumulator
            .observe(&[0.0; 2], &[-0.0; 2], 20)
            .unwrap();
    }
    assert_snapshot_bits(a.consumer.try_pop().unwrap(), b.consumer.try_pop().unwrap());
}
