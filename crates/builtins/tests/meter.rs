//! Meter taps: window boundaries, discontinuities, resets, drops, and the segment law.
//!
//! The meter is the one place in this crate that still runs a scalar sample loop, because its
//! energy accumulation is sequential `f64` and its held-peak decay is a counter state machine.
//! #85 changed the *shape* -- the observation is split at window boundaries, the configuration is
//! hoisted out of `MeterConfig` before the loop, and `sqrt` runs once per emitted window -- and
//! deliberately not the arithmetic: every value below is class A and its JSON fixtures are
//! byte-identical.

use core::hint::black_box;
use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};

use builtins::*;
use lane::{LaneF64, Simd4, Simd8, Widen};

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

// Issue #943, gate G2: the block-peak merge publishes the scalar meter's snapshots.

/// One event of a G2 stream, applied to every lane's meter in both arms.
#[derive(Clone, Copy)]
enum PeakEvent {
    /// A block of this many frames at the running sample time.
    Block(usize),
    /// Skip this many samples, so the next block is a discontinuity.
    Skip(u64),
}

/// Xorshift64\*, so both arms and every host see the same words.
struct PeakRng(u64);

impl PeakRng {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }
}

/// NaN, both infinities, both zeros, the extreme subnormals of both signs, `±MIN_POSITIVE`,
/// `±1.0`, `±f32::MAX` and `±2^k` for `k` in `-34..=5`.
fn peak_hostile_pool() -> Vec<f32> {
    let mut pool = vec![
        f32::NAN,
        f32::from_bits(0xFFC0_0714),
        f32::INFINITY,
        f32::NEG_INFINITY,
        0.0,
        -0.0,
        f32::from_bits(0x0000_0001),
        f32::from_bits(0x8000_0001),
        f32::from_bits(0x007F_FFFF),
        f32::from_bits(0x807F_FFFF),
        f32::MIN_POSITIVE,
        -f32::MIN_POSITIVE,
        1.0,
        -1.0,
        f32::MAX,
        -f32::MAX,
    ];
    for k in -34_i32..=5 {
        let power = f32::from_bits(((127 + k) as u32) << 23);
        pool.push(power);
        pool.push(-power);
    }
    pool
}

/// Words the meter sanitizes to `+0.0`: NaN, both infinities, both zeros, and the extreme
/// subnormals of both signs.
const PEAK_INVALID: [u32; 8] = [
    0x7FC0_0000,
    0xFFC0_0714,
    0x7F80_0000,
    0xFF80_0000,
    0x0000_0000,
    0x8000_0000,
    0x0000_0001,
    0x807F_FFFF,
];

/// Fills one strided 8-lane plane: hostile words from the pool, or a triangle tone whose
/// amplitude differs per lane.
///
/// On hostile input, lanes 1 and 5 carry only [`PEAK_INVALID`] words, so their windows' scalar
/// peak is exactly `+0.0`: a kernel that admitted a subnormal or an infinity would publish
/// something else there, where a lane that also carries normal words would hide it behind a
/// larger peak.
fn peak_fill(rng: &mut PeakRng, pool: &[f32], plane: &mut [f32], hostile: bool, phase: usize) {
    for (index, word) in plane.iter_mut().enumerate() {
        *word = if hostile && matches!(index % 8, 1 | 5) {
            f32::from_bits(PEAK_INVALID[rng.below(PEAK_INVALID.len())])
        } else if hostile {
            pool[rng.below(pool.len())]
        } else {
            let lane = index % 8;
            let step = ((index / 8 + phase) * (3 + lane)) % 256;
            let triangle = (step as f32 / 64.0 - 2.0).abs() - 1.0;
            (0.25 + 0.0625 * lane as f32) * triangle
        };
    }
}

fn peak_meters(metrics: MeterMetricSet, period: u32) -> Vec<PreparedMeter> {
    (0..8_u64)
        .map(|lane| {
            MeterAccumulator::prepare_selected(
                MeterHandle(NonZeroU64::new(lane + 1).expect("constant")),
                MeterConfig {
                    period_frames: NonZeroU32::new(period).expect("period"),
                    peak_hold_frames: 0,
                    peak_decay_db_per_second: 0.0,
                    queue_capacity: NonZeroUsize::new(256).expect("constant"),
                    reset_generation: 943,
                },
                48_000,
                metrics,
            )
            .expect("meter")
        })
        .collect()
}

/// Runs one stream through two arms of eight meters -- arm A through `observe_input`, arm B
/// through `observe_input_with_block_peak` with the lane kernel's seeded-zero block peaks -- and
/// asserts every published snapshot is the same on every field. Returns the snapshots compared.
fn peak_differential(
    metrics: MeterMetricSet,
    period: u32,
    events: &[PeakEvent],
    hostile: bool,
    seed: u64,
) -> usize {
    use lane::kernels::builtins::meter_sample_peak_block;
    use lane::{Lane, Simd8};
    let pool = peak_hostile_pool();
    let mut rng = PeakRng(seed);
    let mut scalar = peak_meters(metrics, period);
    let mut banked = peak_meters(metrics, period);
    let mut time = 1_000_u64;
    let mut compared = 0;
    for (index, event) in events.iter().enumerate() {
        match *event {
            PeakEvent::Block(frames) => {
                let mut left = vec![0.0_f32; frames * 8];
                let mut right = vec![0.0_f32; frames * 8];
                peak_fill(&mut rng, &pool, &mut left, hostile, index * 5);
                peak_fill(&mut rng, &pool, &mut right, hostile, index * 11 + 3);
                let mut peaks = [[0.0_f32; 8]; 2];
                meter_sample_peak_block::<Simd8>(&left, frames, Simd8::zero()).store(&mut peaks[0]);
                meter_sample_peak_block::<Simd8>(&right, frames, Simd8::zero())
                    .store(&mut peaks[1]);
                for lane in 0..8 {
                    let input = MeterInput::strided(&left, &right, frames, 8, lane).expect("view");
                    let a = scalar[lane].accumulator.observe_input(input, time);
                    let b = banked[lane].accumulator.observe_input_with_block_peak(
                        input,
                        time,
                        Some([peaks[0][lane], peaks[1][lane]]),
                    );
                    assert_eq!(a, b);
                }
                time += frames as u64;
            }
            PeakEvent::Skip(samples) => time += samples,
        }
        for lane in 0..8 {
            loop {
                match (
                    scalar[lane].consumer.try_pop(),
                    banked[lane].consumer.try_pop(),
                ) {
                    (Ok(a), Ok(b)) => {
                        assert_snapshot_bits(b, a);
                        if hostile && matches!(lane, 1 | 5) {
                            assert_eq!(
                                [a.left.sample_peak.to_bits(), a.right.sample_peak.to_bits()],
                                [0, 0],
                                "an invalid-only lane's window peak is +0.0"
                            );
                        }
                        compared += 1;
                    }
                    (Err(_), Err(_)) => break,
                    _ => panic!(
                        "metrics {} period {period} event {index} lane {lane}: publication \
                         count differs",
                        metrics.bits()
                    ),
                }
            }
        }
    }
    compared
}

/// The streams G2 runs: 64 plain 128-frame blocks, then the same with a skipped block, a
/// zero-frame block, and both together.
fn peak_streams() -> Vec<(&'static str, Vec<PeakEvent>)> {
    let plain: Vec<PeakEvent> = (0..64).map(|_| PeakEvent::Block(128)).collect();
    let with = |at: usize, event: PeakEvent| {
        let mut stream = plain.clone();
        stream.insert(at, event);
        stream
    };
    let mut all = plain.clone();
    all.insert(50, PeakEvent::Block(0));
    all.insert(20, PeakEvent::Skip(77));
    vec![
        ("plain", plain.clone()),
        ("skip", with(20, PeakEvent::Skip(77))),
        ("zero-frame", with(40, PeakEvent::Block(0))),
        ("all", all),
    ]
}

#[cfg(feature = "test-support")]
fn peak_merges() -> Option<u64> {
    Some(builtins::test_only_block_peak_merges())
}

#[cfg(not(feature = "test-support"))]
fn peak_merges() -> Option<u64> {
    None
}

#[cfg(feature = "test-support")]
fn reset_peak_merges() {
    builtins::test_only_reset_block_peak_merges();
}

#[cfg(not(feature = "test-support"))]
fn reset_peak_merges() {}

/// Gate G2. Every snapshot of the merge arm equals the scalar arm's, `sample_peak` by bits, at
/// periods 512, 300, 128 and 64 on hostile and tone input, through discontinuities and a
/// zero-frame block. A selection other than exactly `SAMPLE_PEAK` declines the merge.
///
/// Built with `--features test-support`, it also pins the merge count: every block at period 512,
/// none at 64 (each 128-frame block crosses a window boundary), some at 300.
#[test]
fn a_block_peak_merge_publishes_the_scalar_meters_snapshots() {
    let peak = MeterMetricSet::SAMPLE_PEAK;
    let peak_and_counts =
        MeterMetricSet::from_bits_retain(peak.bits() | MeterMetricSet::COUNTS.bits());
    let mut seed = 0x0943_5A3B_1E00_0001_u64;
    let mut total = 0;
    for period in [512_u32, 300, 128, 64] {
        for hostile in [true, false] {
            for (name, stream) in peak_streams() {
                seed = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(1);
                reset_peak_merges();
                let compared = peak_differential(peak, period, &stream, hostile, seed);
                let merges = peak_merges();
                total += compared;
                assert!(
                    compared > 0,
                    "period {period} {name}: windows were published"
                );
                if name == "plain" {
                    eprintln!(
                        "G2 period {period} hostile {hostile}: {compared} snapshots, merges {merges:?}"
                    );
                    let expected_windows = 64 * 128 / period as usize * 8;
                    assert_eq!(compared, expected_windows, "period {period}");
                    if let Some(merges) = merges {
                        match period {
                            512 | 128 => assert_eq!(merges, 8 * 64, "period {period}"),
                            64 => assert_eq!(merges, 0, "period {period}"),
                            _ => assert!(
                                merges > 0 && merges < 8 * 64,
                                "period {period}: {merges} merges"
                            ),
                        }
                    }
                }
                for metrics in [MeterMetricSet::ALL, peak_and_counts] {
                    reset_peak_merges();
                    let declined = peak_differential(metrics, period, &stream, hostile, seed);
                    assert!(declined > 0);
                    total += declined;
                    if let Some(merges) = peak_merges() {
                        assert_eq!(merges, 0, "metrics {} never merge", metrics.bits());
                    }
                }
            }
        }
    }
    eprintln!("G2: {total} snapshots bit-identical across both arms");
}

// Issue #950, gate M2: the banked commit publishes the scalar meter's snapshots.

/// One event of an M2 stream, applied to every meter of both arms.
#[derive(Clone, Copy, Debug)]
enum BankedEvent {
    /// A block of this many frames at the running sample time.
    Block(usize),
    /// Skip this many samples, so the next block is a discontinuity.
    Skip(u64),
    /// `reset` with this kind.
    Reset(BuiltinResetKind),
}

/// How arm B hands each lane its banked block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BankedMode {
    /// The protocol: the first seed in binding order, the pass from those seeds, and the result.
    Seeded,
    /// No pass: every meter is handed `None`.
    Withheld,
    /// The safety net: every seed is moved by one ulp before the pass runs from it and is handed
    /// on, so no energy-keeping meter may commit.
    MovedSeed,
    /// The safety net for one channel: only the left seed is moved, so only the left half of the
    /// seed check can refuse the commit.
    MovedLeftSeed,
    /// The same for the right channel alone.
    MovedRightSeed,
}

/// One M2 configuration. `metrics` is one lane's meters, in binding order; every lane has the same.
#[derive(Clone, Debug)]
struct BankedRun {
    metrics: Vec<MeterMetricSet>,
    period: u32,
    hold: u32,
    decay: f32,
    hostile: bool,
    mode: BankedMode,
}

/// What one M2 run saw: snapshots compared, and the banked commit count after every event when
/// the counter is built.
struct BankedOutcome {
    compared: usize,
    commits: Option<Vec<u64>>,
}

#[cfg(feature = "test-support")]
fn banked_commits() -> Option<u64> {
    Some(builtins::test_only_banked_meter_commits())
}

#[cfg(not(feature = "test-support"))]
fn banked_commits() -> Option<u64> {
    None
}

#[cfg(feature = "test-support")]
fn reset_banked_commits() {
    builtins::test_only_reset_banked_meter_commits();
}

#[cfg(not(feature = "test-support"))]
fn reset_banked_commits() {}

/// Every field of one snapshot as bits, so a float compares by its bits rather than `==`.
fn banked_snapshot_bits(snapshot: &MeterSnapshot) -> [u64; 23] {
    let lane = |lane: &MeterLaneSnapshot| {
        [
            u64::from(lane.sample_peak.to_bits()),
            lane.rms.to_bits(),
            lane.energy.to_bits(),
            u64::from(lane.held_peak.to_bits()),
            lane.clipped_samples,
            lane.sanitized_samples,
        ]
    };
    let [l0, l1, l2, l3, l4, l5] = lane(&snapshot.left);
    let [r0, r1, r2, r3, r4, r5] = lane(&snapshot.right);
    [
        snapshot.handle.0.get(),
        u64::from(snapshot.present_metrics.bits()),
        snapshot.reset_generation,
        snapshot.window_sequence,
        snapshot.start_sample,
        snapshot.end_sample,
        u64::from(snapshot.frames),
        snapshot.cumulative_clipped_samples,
        snapshot.cumulative_sanitized_samples,
        snapshot.cumulative_discontinuities,
        snapshot.cumulative_dropped_snapshots,
        l0,
        l1,
        l2,
        l3,
        l4,
        l5,
        r0,
        r1,
        r2,
        r3,
        r4,
        r5,
    ]
}

/// One arm's meters, `[lane][binding]`.
fn banked_meters(run: &BankedRun, width: usize) -> Vec<Vec<PreparedMeter>> {
    (0..width)
        .map(|lane| {
            run.metrics
                .iter()
                .enumerate()
                .map(|(binding, &metrics)| {
                    MeterAccumulator::prepare_selected(
                        MeterHandle(
                            NonZeroU64::new((lane * 8 + binding) as u64 + 1).expect("constant"),
                        ),
                        MeterConfig {
                            period_frames: NonZeroU32::new(run.period).expect("period"),
                            peak_hold_frames: run.hold,
                            peak_decay_db_per_second: run.decay,
                            queue_capacity: NonZeroUsize::new(1024).expect("constant"),
                            reset_generation: 950,
                        },
                        48_000,
                        metrics,
                    )
                    .expect("meter")
                })
                .collect()
        })
        .collect()
}

/// Fills one strided plane of `width` lanes: hostile words (lane 1, and lane 5 at eight lanes,
/// only words the meter sanitizes to `+0.0`), or a triangle tone whose amplitude differs per lane
/// and clips on some.
fn banked_fill(
    rng: &mut PeakRng,
    pool: &[f32],
    plane: &mut [f32],
    width: usize,
    hostile: bool,
    phase: usize,
) {
    for (index, word) in plane.iter_mut().enumerate() {
        let lane = index % width;
        *word = if hostile && matches!(lane, 1 | 5) {
            f32::from_bits(PEAK_INVALID[rng.below(PEAK_INVALID.len())])
        } else if hostile {
            pool[rng.below(pool.len())]
        } else {
            let step = ((index / width + phase) * (3 + lane)) % 256;
            let triangle = (step as f32 / 64.0 - 2.0).abs() - 1.0;
            (0.375 * (1 + (lane + phase) % 4) as f32) * triangle
        };
    }
}

/// Runs one stream through two arms at `L`'s width -- arm A through `observe_input`, arm B through
/// the banked protocol of `run.mode` -- and asserts every published snapshot is the same on every
/// field by bits.
///
/// Arm B is what the graph and `MeterObserver` do together: per lane, the first `banked_seed` in
/// binding order; the pass only if some lane answered, from `+0.0` for a lane that did not; then
/// each meter, in binding order, commits through `observe_input_banked` if it banks and is not
/// exactly `SAMPLE_PEAK`, and otherwise takes issue #943's call with the pass's peak.
fn banked_differential<L: Widen>(
    run: &BankedRun,
    events: &[BankedEvent],
    seed: u64,
) -> BankedOutcome {
    use lane::kernels::builtins::meter_block;
    let width = L::WIDTH;
    let pool = peak_hostile_pool();
    let mut rng = PeakRng(seed);
    let mut scalar = banked_meters(run, width);
    let mut banked = banked_meters(run, width);
    let mut time = 1_000_u64;
    let mut compared = 0;
    let mut commits = banked_commits().map(|_| Vec::with_capacity(events.len()));
    reset_banked_commits();
    for (index, event) in events.iter().enumerate() {
        match *event {
            BankedEvent::Block(frames) => {
                let mut left = vec![0.0_f32; frames * width];
                let mut right = vec![0.0_f32; frames * width];
                banked_fill(&mut rng, &pool, &mut left, width, run.hostile, index * 5);
                banked_fill(
                    &mut rng,
                    &pool,
                    &mut right,
                    width,
                    run.hostile,
                    index * 11 + 3,
                );
                let mut seeds = [[0.0_f64; 8]; 2];
                let mut seeded = false;
                if run.mode != BankedMode::Withheld {
                    for (lane, meters) in banked.iter().enumerate() {
                        if let Some([l, r]) = meters
                            .iter()
                            .find_map(|meter| meter.accumulator.banked_seed(time, frames))
                        {
                            let moved = |x: f64, channel: usize| match (run.mode, channel) {
                                (BankedMode::MovedSeed, _)
                                | (BankedMode::MovedLeftSeed, 0)
                                | (BankedMode::MovedRightSeed, 1) => {
                                    f64::from_bits(x.to_bits() + 1)
                                }
                                _ => x,
                            };
                            seeds[0][lane] = moved(l, 0);
                            seeds[1][lane] = moved(r, 1);
                            seeded = true;
                        }
                    }
                }
                let blocks: Option<Vec<MeterBankedBlock>> = seeded.then(|| {
                    let mut peak = [[0.0_f32; 8]; 2];
                    let mut clipped = [[0.0_f32; 8]; 2];
                    let mut sanitized = [[0.0_f32; 8]; 2];
                    let mut energy = [[0.0_f64; 8]; 2];
                    for (plane, words) in [&left, &right].into_iter().enumerate() {
                        let result = black_box(meter_block::<L>(
                            words,
                            frames,
                            <L::F64 as LaneF64>::load(&seeds[plane][..width]),
                        ));
                        result.peak.store(&mut peak[plane][..width]);
                        result.clipped.store(&mut clipped[plane][..width]);
                        result.sanitized.store(&mut sanitized[plane][..width]);
                        result.energy.store(&mut energy[plane][..width]);
                    }
                    (0..width)
                        .map(|lane| MeterBankedBlock {
                            sample_peak: [peak[0][lane], peak[1][lane]],
                            clipped: [clipped[0][lane] as u64, clipped[1][lane] as u64],
                            sanitized: [sanitized[0][lane] as u64, sanitized[1][lane] as u64],
                            energy_seed: [seeds[0][lane], seeds[1][lane]],
                            energy: [energy[0][lane], energy[1][lane]],
                        })
                        .collect()
                });
                for lane in 0..width {
                    let input =
                        MeterInput::strided(&left, &right, frames, width, lane).expect("view");
                    let block = blocks.as_ref().map(|blocks| blocks[lane]);
                    for (a, b) in scalar[lane].iter_mut().zip(banked[lane].iter_mut()) {
                        let expected = a.accumulator.observe_input(input, time);
                        let meter = &mut b.accumulator;
                        let actual = match block {
                            Some(block)
                                if meter.banked_eligible()
                                    && meter.metrics() != MeterMetricSet::SAMPLE_PEAK =>
                            {
                                meter.observe_input_banked(input, time, Some(block))
                            }
                            _ if run.mode == BankedMode::Withheld => {
                                meter.observe_input_banked(input, time, None)
                            }
                            _ => meter.observe_input_with_block_peak(
                                input,
                                time,
                                block.map(|block| block.sample_peak),
                            ),
                        };
                        assert_eq!(expected, actual);
                    }
                }
                time += frames as u64;
            }
            BankedEvent::Skip(samples) => time += samples,
            BankedEvent::Reset(kind) => {
                for meter in scalar.iter_mut().chain(banked.iter_mut()).flatten() {
                    meter.accumulator.reset(kind);
                }
            }
        }
        if let Some(commits) = commits.as_mut() {
            commits.push(banked_commits().expect("built with the counter"));
        }
        for (lane, (a_meters, b_meters)) in scalar.iter_mut().zip(banked.iter_mut()).enumerate() {
            for (binding, (a, b)) in a_meters.iter_mut().zip(b_meters.iter_mut()).enumerate() {
                loop {
                    match (a.consumer.try_pop(), b.consumer.try_pop()) {
                        (Ok(expected), Ok(actual)) => {
                            let (expected_bits, actual_bits) = (
                                banked_snapshot_bits(&expected),
                                banked_snapshot_bits(&actual),
                            );
                            if actual_bits != expected_bits {
                                panic!(
                                    "{run:?} width {width} event {index} ({event:?}) lane {lane} \
                                     binding {binding}: window {} differs\n banked {actual:?}\n \
                                     scalar {expected:?}",
                                    expected.window_sequence
                                );
                            }
                            compared += 1;
                        }
                        (Err(_), Err(_)) => break,
                        _ => panic!(
                            "{run:?} width {width} event {index} lane {lane} binding {binding}: \
                             publication count differs"
                        ),
                    }
                }
            }
        }
    }
    BankedOutcome { compared, commits }
}

/// The random M2 stream: 150 events of blocks of the brief's sizes, zero-frame blocks, skips and
/// both reset kinds.
fn banked_random_stream(seed: u64) -> Vec<BankedEvent> {
    const SIZES: [usize; 11] = [1, 2, 3, 63, 64, 127, 128, 129, 256, 300, 511];
    let mut rng = PeakRng(seed);
    (0..150)
        .map(|_| match rng.below(19) {
            0 | 1 => BankedEvent::Skip(1 + rng.below(5_000) as u64),
            2 => BankedEvent::Reset(BuiltinResetKind::FullToPrepared),
            3 => BankedEvent::Reset(BuiltinResetKind::DiscontinuityKeepTargets),
            4 => BankedEvent::Block(0),
            _ => BankedEvent::Block(SIZES[rng.below(SIZES.len())]),
        })
        .collect()
}

fn banked_plain_stream(blocks: usize) -> Vec<BankedEvent> {
    (0..blocks).map(|_| BankedEvent::Block(128)).collect()
}

fn metric_set(bits: &[MeterMetricSet]) -> MeterMetricSet {
    MeterMetricSet::from_bits_retain(bits.iter().fold(0, |set, metric| set | metric.bits()))
}

/// M2's sweep at one period: nine metric selections, four hold/decay settings, hostile and tone
/// input, the fixed and a random stream, at both vector widths. Returns the snapshots compared and
/// the banked commits counted (zero without the counter).
fn banked_sweep(period: u32) -> (usize, u64) {
    use MeterMetricSet as M;
    let sets = [
        M::ALL,
        M::ENERGY_RMS,
        M::COUNTS,
        M::HELD_PEAK,
        M::SAMPLE_PEAK,
        metric_set(&[M::SAMPLE_PEAK, M::ENERGY_RMS]),
        metric_set(&[M::SAMPLE_PEAK, M::COUNTS]),
        metric_set(&[M::ENERGY_RMS, M::HELD_PEAK]),
        metric_set(&[M::COUNTS, M::HELD_PEAK]),
    ];
    let ballistics = [(0_u32, 0.0_f32), (7, 0.0), (0, 12.0), (100, 60.0)];
    let fixed: Vec<BankedEvent> = banked_plain_stream(48);
    let mut seed = 0x0950_5A3B_1E00_0001_u64 ^ u64::from(period) << 32;
    let mut total = 0;
    let mut committed = 0_u64;
    for metrics in sets {
        for (hold, decay) in ballistics {
            for hostile in [true, false] {
                seed = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(1);
                let random = banked_random_stream(seed);
                let run = BankedRun {
                    metrics: vec![metrics],
                    period,
                    hold,
                    decay,
                    hostile,
                    mode: BankedMode::Seeded,
                };
                for (fixed_stream, stream) in [(true, &fixed), (false, &random)] {
                    let eight = banked_differential::<Simd8>(&run, stream, seed);
                    let four = banked_differential::<Simd4>(&run, stream, seed ^ 0x4);
                    // 48 blocks of 128 frames outlast every period; a random stream's resets and
                    // gaps may leave a long window unfinished.
                    assert!(
                        !fixed_stream || (eight.compared > 0 && four.compared > 0),
                        "{run:?}: windows were published"
                    );
                    total += eight.compared + four.compared;
                    for outcome in [eight, four] {
                        let Some(commits) = outcome.commits else {
                            continue;
                        };
                        let last = commits.last().copied().unwrap_or(0);
                        committed += last;
                        if metrics.contains(M::HELD_PEAK) && (hold != 0 || decay != 0.0) {
                            assert_eq!(last, 0, "{run:?}: a held peak with ballistics never banks");
                        }
                        if !metrics.contains(M::ENERGY_RMS) {
                            assert_eq!(last, 0, "{run:?}: no energy, no seed, no pass");
                        }
                        if fixed_stream
                            && metrics.contains(M::ENERGY_RMS)
                            && (!metrics.contains(M::HELD_PEAK) || (hold == 0 && decay == 0.0))
                            && period.is_multiple_of(128)
                        {
                            assert!(last > 0, "{run:?}: a bankable meter commits");
                        }
                    }
                }
            }
        }
    }
    (total, committed)
}

/// Gate M2. Every snapshot of the banked arm equals the scalar arm's on every field by bits, at
/// both vector widths, over nine periods, nine metric selections, four hold/decay settings, hostile
/// and tone input, a fixed stream of 48 blocks of 128 frames and a random stream of 150 events.
///
/// Built with `--features test-support`, it also pins the commit counter: every block of a plain
/// 64-block stream at period 512 and 1536, none at 64, some at 300; none on a held peak with a hold
/// or a decay; none with the block withheld; none with both seeds, the left alone or the right
/// alone moved by one ulp; and a block after a skipped gap commits.
#[test]
fn a_banked_block_commit_publishes_the_scalar_meters_snapshots() {
    use MeterMetricSet as M;
    // One thread per period: the sweep is 2,592 runs, and the counters are thread-local.
    let periods = [1_u32, 64, 127, 128, 129, 300, 512, 1536, 4096];
    let sweeps: Vec<(usize, u64)> = std::thread::scope(|scope| {
        let handles: Vec<_> = periods
            .iter()
            .map(|&period| scope.spawn(move || banked_sweep(period)))
            .collect();
        handles
            .into_iter()
            .map(|handle| {
                handle
                    .join()
                    .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
            })
            .collect()
    });
    let total: usize = sweeps.iter().map(|sweep| sweep.0).sum();
    let committed: u64 = sweeps.iter().map(|sweep| sweep.1).sum();
    eprintln!(
        "M2 sweep: {total} snapshots bit-identical across both arms, {committed} banked commits"
    );
    assert!(total > 0);

    // The commit counter on a plain stream of 64 blocks of 128 frames, eight ALL meters.
    let plain = banked_plain_stream(64);
    let all = |period, hold, decay, mode| BankedRun {
        metrics: vec![M::ALL],
        period,
        hold,
        decay,
        hostile: false,
        mode,
    };
    let last = |outcome: BankedOutcome| outcome.commits.map(|commits| commits[commits.len() - 1]);
    for (period, expected) in [(512_u32, Some(8 * 64)), (64, Some(0)), (1536, Some(8 * 64))] {
        let commits = last(banked_differential::<Simd8>(
            &all(period, 0, 0.0, BankedMode::Seeded),
            &plain,
            7,
        ));
        if commits.is_some() {
            assert_eq!(commits, expected, "ALL at period {period}");
        }
    }
    if let Some(commits) = last(banked_differential::<Simd8>(
        &all(300, 0, 0.0, BankedMode::Seeded),
        &plain,
        7,
    )) {
        assert!(
            commits > 0 && commits < 8 * 64,
            "ALL at period 300: {commits}"
        );
    }
    for (hold, decay) in [(7, 0.0), (0, 12.0), (100, 60.0)] {
        if let Some(commits) = last(banked_differential::<Simd8>(
            &all(512, hold, decay, BankedMode::Seeded),
            &plain,
            7,
        )) {
            assert_eq!(commits, 0, "hold {hold} decay {decay}");
        }
    }
    // The safety net moves both seeds, then each one alone, so each channel's half of the seed
    // check is the only thing refusing a commit in one of the rows.
    for mode in [
        BankedMode::Withheld,
        BankedMode::MovedSeed,
        BankedMode::MovedLeftSeed,
        BankedMode::MovedRightSeed,
    ] {
        let outcome = banked_differential::<Simd8>(&all(512, 0, 0.0, mode), &plain, 7);
        assert_eq!(outcome.compared, 8 * 64 * 128 / 512, "{mode:?}");
        if let Some(commits) = outcome.commits {
            assert_eq!(commits[commits.len() - 1], 0, "{mode:?} commits nothing");
        }
        let outcome = banked_differential::<Simd4>(&all(512, 0, 0.0, mode), &plain, 7);
        assert_eq!(outcome.compared, 4 * 64 * 128 / 512, "{mode:?} at Simd4");
        if let Some(commits) = outcome.commits {
            assert_eq!(
                commits[commits.len() - 1],
                0,
                "{mode:?} commits nothing at Simd4"
            );
        }
    }

    // A discontinuity: the block after the gap commits too.
    let gap = [
        BankedEvent::Block(128),
        BankedEvent::Skip(7),
        BankedEvent::Block(128),
    ];
    if let Some(commits) =
        banked_differential::<Simd8>(&all(512, 0, 0.0, BankedMode::Seeded), &gap, 7).commits
    {
        assert_eq!(commits, [8, 8, 16], "the block after the skip commits");
    }
}

/// Gate M2's two-meter row (issue #950, amendment 2): with a `SAMPLE_PEAK` meter bound before an
/// `ALL` meter on every lane, the peak meter answers no seed -- it keeps no energy -- so the bank's
/// first-answer search reaches the `ALL` meter, which commits every block of a plain stream at
/// period 512. Had the peak meter answered its unused `+0.0`, the `ALL` meter would commit only
/// the blocks that start a window. Both meters' snapshots equal the scalar arm's by bits.
#[test]
fn a_peak_meter_bound_first_leaves_the_seed_to_the_all_meter_behind_it() {
    use MeterMetricSet as M;
    let plain = banked_plain_stream(64);
    for (hostile, seed) in [(false, 7), (true, 8)] {
        let pair = BankedRun {
            metrics: vec![M::SAMPLE_PEAK, M::ALL],
            period: 512,
            hold: 0,
            decay: 0.0,
            hostile,
            mode: BankedMode::Seeded,
        };
        let outcome = banked_differential::<Simd8>(&pair, &plain, seed);
        assert_eq!(outcome.compared, 2 * 8 * 64 * 128 / 512);
        if let Some(commits) = outcome.commits {
            assert_eq!(
                commits[commits.len() - 1],
                8 * 64,
                "the ALL meter commits every block"
            );
        }
        let outcome = banked_differential::<Simd4>(&pair, &plain, seed);
        if let Some(commits) = outcome.commits {
            assert_eq!(commits[commits.len() - 1], 4 * 64);
        }
    }
}
