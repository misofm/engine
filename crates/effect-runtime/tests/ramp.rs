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
    let mut done = 0;
    while done < count {
        let this = frames.min(count - done);
        let segment: RampSegment<L> = ramp.advance_block::<L>(this);
        let mut buffer = vec![1.0f32; this * L::WIDTH];
        ramp_block::<L>(&mut buffer, this, &segment);
        for frame in buffer.chunks_exact(L::WIDTH) {
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

/// The published D11 example: three samples from `0.0` to `1.0` are `1/3`, `1/3 + 1/3`, `1.0`.
///
/// The old law — re-deriving the step from the remaining distance every sample — gives `0.5` at
/// the second sample. That is the discriminating value.
///
/// Red mutation: `next_value()` recomputes `(target - current) / remaining as f32` per call.
#[test]
fn the_step_is_computed_once() {
    let mut ramp = LinearRamp::fixed(0.0);
    ramp.set_target(1.0, 3);
    let third = 1.0f32 / 3.0;
    assert_eq!(ramp.next_value().to_bits(), third.to_bits());
    assert_eq!(ramp.next_value().to_bits(), (third + third).to_bits());
    assert_ne!(
        (third + third).to_bits(),
        0.5f32.to_bits(),
        "must discriminate"
    );
    assert_eq!(ramp.next_value().to_bits(), 1.0f32.to_bits());
    assert_eq!(
        ramp.next_value().to_bits(),
        1.0f32.to_bits(),
        "at rest afterwards"
    );
    assert!(!ramp.is_ramping());
}

/// The final sample is an exact assignment, not an accumulation.
///
/// Red mutation: drop the `remaining == 1` arm of `next_value()`, so the last sample is
/// `current + step`. The target here is chosen so that the accumulated value misses it.
#[test]
fn the_last_sample_is_exactly_the_target() {
    let mut ramp = LinearRamp::fixed(0.0);
    ramp.set_target(0.1, 5);
    let sequence = scalar_sequence(ramp, 5);
    assert_eq!(sequence[4].to_bits(), 0.1f32.to_bits());
    let accumulated: f32 = (0..5).fold(0.0f32, |acc, _| acc + 0.1f32 / 5.0);
    assert_ne!(
        accumulated.to_bits(),
        0.1f32.to_bits(),
        "accumulation must miss, or the test proves nothing"
    );
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
            assert_bit_equal(
                &block_sequence::<f32>(ramp, 512, frames),
                &expected,
                &format!("scalar, target {target}, {samples} samples, blocks of {frames}"),
            );
            assert_bit_equal(
                &block_sequence::<lane::Simd4>(ramp, 512, frames),
                &expected,
                &format!("Simd4, target {target}, {samples} samples, blocks of {frames}"),
            );
            assert_bit_equal(
                &block_sequence::<lane::Simd8>(ramp, 512, frames),
                &expected,
                &format!("Simd8, target {target}, {samples} samples, blocks of {frames}"),
            );
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
            split.current.to_bits(),
            one_shot.current.to_bits(),
            "blocks of {frames}: current"
        );
        assert_eq!(split.remaining, one_shot.remaining, "blocks of {frames}");
        assert_eq!(split.target.to_bits(), one_shot.target.to_bits());
    }
}

/// `samples == 0` snaps, and `snap` and `fixed` agree with it.
#[test]
fn zero_length_ramps_snap() {
    let mut ramp = LinearRamp::fixed(1.0);
    ramp.set_target(-3.0, 0);
    assert_eq!(ramp.current.to_bits(), (-3.0f32).to_bits());
    assert_eq!(ramp.step.to_bits(), 0.0f32.to_bits());
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
            if ramp.remaining == 0 {
                assert_eq!(ramp.current.to_bits(), ramp.target.to_bits());
            }
        }
        assert_eq!(ramp.remaining, 0);
        assert_eq!(ramp.current.to_bits(), target.to_bits());
    }
}
