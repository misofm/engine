//! Decision D11: one division at event time, iterated additions, an exact snap on the last sample.

use effect_runtime::ramp::LinearRamp;
use lane::Lane;
use lane::kernels::{RampSegment, ramp_block};

/// The sample sequence `next_value()` produces, for `count` samples.
fn scalar_sequence(mut ramp: LinearRamp, count: usize) -> Vec<f32> {
    (0..count).map(|_| ramp.next_value()).collect()
}

/// The sample sequence the lane kernel produces when driven by `advance_block`, in blocks of
/// `frames`. The buffer holds `1.0` so the applied gain is the value itself.
fn block_sequence<L: Lane>(mut ramp: LinearRamp, count: usize, frames: usize) -> Vec<f32> {
    let mut out = Vec::with_capacity(count);
    let mut buffer = vec![1.0f32; frames.min(count) * L::WIDTH];
    let mut done = 0;
    while done < count {
        let this = frames.min(count - done);
        let segment: RampSegment<L> = ramp.advance_block::<L>(this);
        let block = &mut buffer[..this * L::WIDTH];
        block.fill(1.0);
        ramp_block::<L>(block, this, &segment);
        for frame in block.chunks_exact(L::WIDTH) {
            // Every lane carries the same scalar ramp, so lane 0 is the sequence.
            out.push(frame[0]);
            for value in frame {
                assert_eq!(value.to_bits(), frame[0].to_bits(), "lanes must agree");
            }
        }
        done += this;
    }
    out
}

fn assert_bit_equal(left: &[f32], right: &[f32], what: &str) {
    assert_eq!(left.len(), right.len(), "{what}: length");
    for (index, (a, b)) in left.iter().zip(right).enumerate() {
        assert_eq!(
            a.to_bits(),
            b.to_bits(),
            "{what}: sample {index}: {a} vs {b}"
        );
    }
}

/// `advance_block` plus the lane kernel reproduce `next_value()` exactly, at every block size.
///
/// This is the D11 partition property and the reason `ramp_frames` is `remaining - 1`: the snap
/// sample belongs to the target run, not the stepping run.
///
/// Red mutation: `ramp_frames = min(remaining, frames)` — the off-by-one — makes the sample at the
/// end of the ramp `current + step` instead of `target`.
#[test]
fn block_driving_matches_the_scalar_sequence() {
    for (target, samples) in [
        (1.0f32, 3u32),
        (0.1, 7),
        (-0.25, 64),
        (12.5, 1),
        (0.0, 128),
        (3.0, 500),
    ] {
        let mut ramp = LinearRamp::fixed(-0.5);
        ramp.set_target(target, samples);
        let expected = scalar_sequence(ramp, 512);
        for frames in [1usize, 7, 64, 128, 512] {
            lane::each_lane!(|L| assert_bit_equal(
                &block_sequence::<L>(ramp, 512, frames),
                &expected,
                &format!(
                    "{}, target {target}, {samples} samples, blocks of {frames}",
                    core::any::type_name::<L>()
                ),
            ));
        }
    }
}

/// The state a block leaves behind does not depend on how the block was partitioned.
#[test]
fn the_state_after_a_block_is_partition_invariant() {
    let mut reference = LinearRamp::fixed(2.0);
    reference.set_target(-1.0, 300);
    let mut one_shot = reference;
    let _: RampSegment<f32> = one_shot.advance_block::<f32>(512);
    for frames in [1usize, 7, 64, 128] {
        let mut split = reference;
        let mut done = 0;
        while done < 512 {
            let this = frames.min(512 - done);
            let _: RampSegment<f32> = split.advance_block::<f32>(this);
            done += this;
        }
        assert_eq!(
            split.current().to_bits(),
            one_shot.current().to_bits(),
            "blocks of {frames}: current"
        );
        assert_eq!(
            split.remaining(),
            one_shot.remaining(),
            "blocks of {frames}"
        );
        assert_eq!(split.target().to_bits(), one_shot.target().to_bits());
    }
}

/// `samples == 0` snaps, and `snap` and `fixed` agree with it.
#[test]
fn zero_length_ramps_snap() {
    let mut ramp = LinearRamp::fixed(1.0);
    ramp.set_target(-3.0, 0);
    assert_eq!(ramp.current().to_bits(), (-3.0f32).to_bits());
    assert_eq!(ramp.step().to_bits(), 0.0f32.to_bits());
    assert!(!ramp.is_ramping());

    let mut snapped = LinearRamp::fixed(1.0);
    snapped.set_target(-3.0, 64);
    snapped.snap();
    assert_eq!(snapped, ramp);
    assert_eq!(LinearRamp::default(), LinearRamp::fixed(0.0));
}

/// A ramp at rest produces its target for a whole block, and the segment says so.
#[test]
fn a_resting_ramp_is_a_constant_segment() {
    let mut ramp = LinearRamp::fixed(0.75);
    let segment: RampSegment<f32> = ramp.advance_block::<f32>(128);
    assert_eq!(segment.ramp_frames, 0);
    assert_eq!(segment.step.to_bits(), 0.0f32.to_bits());
    assert_eq!(segment.target.to_bits(), 0.75f32.to_bits());
    assert_eq!(ramp, LinearRamp::fixed(0.75));
}

/// `remaining == 0` implies `current == target`, after every operation that can change either.
#[test]
fn the_rest_invariant_holds() {
    let mut ramp = LinearRamp::fixed(0.0);
    for (target, samples) in [(1.0f32, 5u32), (-2.0, 1), (0.5, 0), (7.0, 64)] {
        ramp.set_target(target, samples);
        for _ in 0..samples + 2 {
            ramp.next_value();
            if ramp.remaining() == 0 {
                assert_eq!(ramp.current().to_bits(), ramp.target().to_bits());
            }
        }
        assert_eq!(ramp.remaining(), 0);
        assert_eq!(ramp.current().to_bits(), target.to_bits());
    }
}

/// Issue #2368: words enter a ramp only in phase. `from_words` refuses exactly a settled ramp
/// that is not at its target bit for bit, or whose step is not `+0.0`, and keeps every other
/// ramp's words as given; `record_advance` stores a settled advance at the target with a `+0.0`
/// step whatever words it is handed, and an advance in flight verbatim.
///
/// Test value: `from_words` comparing with float `==` (which accepts a settled `-0.0`/`+0.0`
/// pair and refuses a settled NaN at its own bits), dropping its step clause, or
/// `record_advance` storing a settled `current` verbatim turns this red; no restore test reaches
/// the `-0.0`, NaN or `record_advance` cases.
#[test]
fn words_enter_a_ramp_only_in_phase() {
    let bits = |ramp: LinearRamp| {
        [
            ramp.current().to_bits(),
            ramp.target().to_bits(),
            ramp.step().to_bits(),
            ramp.remaining(),
        ]
    };
    let nan = f32::from_bits(0x7FC0_0001);
    for value in [0.0_f32, -0.0, 0.5, f32::from_bits(1), nan, f32::INFINITY] {
        let ramp = LinearRamp::from_words(value, value, 0.0, 0).expect("settled at its target");
        assert_eq!(bits(ramp), bits(LinearRamp::fixed(value)), "{value:e}");
    }
    for (case, current, target, step) in [
        ("-0.0 at +0.0", -0.0_f32, 0.0_f32, 0.0_f32),
        ("+0.0 at -0.0", 0.0, -0.0, 0.0),
        ("one ulp off", 0.5_f32.next_up(), 0.5, 0.0),
        ("a step of -0.0", 0.5, 0.5, -0.0),
        ("a step", 0.5, 0.5, 1e-3),
    ] {
        assert_eq!(
            LinearRamp::from_words(current, target, step, 0),
            None,
            "{case}"
        );
    }
    for (current, target, step, remaining) in [
        (-0.0_f32, 0.0_f32, 0.0_f32, 1_u32),
        (0.25, 0.75, -1e-3, 64),
        (0.5, 0.5, 0.0, 3),
        (0.5, 0.25, nan, u32::MAX),
    ] {
        let ramp = LinearRamp::from_words(current, target, step, remaining).expect("in flight");
        assert_eq!(
            bits(ramp),
            [
                current.to_bits(),
                target.to_bits(),
                step.to_bits(),
                remaining
            ]
        );
    }

    let mut ramp = LinearRamp::fixed(0.0);
    ramp.set_target(1.0, 8);
    ramp.record_advance(0.375, 0.125, 5);
    assert_eq!(
        bits(ramp),
        [
            0.375_f32.to_bits(),
            1.0_f32.to_bits(),
            0.125_f32.to_bits(),
            5
        ]
    );
    ramp.record_advance(0.875, 0.125, 0);
    assert_eq!(
        bits(ramp),
        bits(LinearRamp::fixed(1.0)),
        "a settled advance is at its target"
    );

    let mut held = LinearRamp::holding(0.5, 3);
    assert!(held.is_ramping());
    assert_eq!(
        [held.next_value(), held.next_value(), held.next_value()],
        [0.5; 3]
    );
    assert_eq!(bits(held), bits(LinearRamp::fixed(0.5)));
    assert_eq!(
        bits(LinearRamp::holding(0.5, 0)),
        bits(LinearRamp::fixed(0.5))
    );
}
