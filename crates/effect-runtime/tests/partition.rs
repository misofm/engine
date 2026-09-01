#![allow(clippy::disallowed_methods)] // D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! Gate P1 for this crate: a block boundary is not observable.
//!
//! A ramp, an envelope follower and the gain computer are composed into one small dynamics
//! processor — the shape every effect in wave 2 will have — and 512 frames are rendered once as a
//! single block and again in blocks of 1, 7, 64, 128 and 512. Every rendered sample and the whole
//! of the processor's state must be bit-identical.
//!
//! Red mutation: carry the envelope in a local of `process_block` instead of in the state struct,
//! or return the ramp segment without advancing the scalar ramp state.

use effect_runtime::dynamics::{GainComputerCoef, gain_delta_db, gain_from_db, level_db};
use effect_runtime::envelope::{peak_follow, retention_coefficient};
use effect_runtime::ramp::LinearRamp;
use lane::kernels::RampSegment;
use lane::{Lane, Simd4, Simd8, flush};

/// The block sizes every block API in the workspace is gated on.
const PARTITIONS: [usize; 5] = [1, 7, 64, 128, 512];

/// Total frames rendered by each run.
const FRAMES: usize = 512;

struct Processor<L: Lane> {
    envelope: L,
    release: L,
    makeup: LinearRamp,
    curve: GainComputerCoef<L>,
}

impl<L: Lane> Processor<L> {
    fn new() -> Self {
        let mut makeup = LinearRamp::fixed(0.5);
        makeup.set_target(1.75, 300);
        Self {
            envelope: L::zero(),
            release: L::splat(retention_coefficient(30.0, 48_000)),
            makeup,
            curve: GainComputerCoef::new(-18.0, 4.0, 6.0),
        }
    }

    /// One block: ramp the makeup gain, follow the peak, ride the static curve.
    ///
    /// The makeup gain is applied with the same iterated addition and the same snap that
    /// `lane::kernels::ramp_block` uses, which is why the whole composition is
    /// partition-invariant and not merely each piece of it.
    fn process_block(&mut self, io: &mut [f32], frames: usize) {
        debug_assert_eq!(io.len(), frames * L::WIDTH);
        let segment: RampSegment<L> = self.makeup.advance_block::<L>(frames);
        let mut gain = segment.start;
        let mut envelope = self.envelope;
        for (index, frame) in io.chunks_exact_mut(L::WIDTH).enumerate() {
            let x = L::load(frame);
            envelope = flush(peak_follow(x.abs(), envelope, self.release));
            let reduction = gain_delta_db(level_db(envelope), &self.curve);
            let compressed = x.mul(gain_from_db(reduction));
            let makeup = if index < segment.ramp_frames {
                gain
            } else {
                segment.target
            };
            compressed.mul(makeup).store(frame);
            gain = gain.add(segment.step);
        }
        self.envelope = envelope;
    }
}

/// A deterministic signal with a loud burst in the middle, so the compressor actually works.
fn signal<L: Lane>() -> Vec<f32> {
    let mut out = vec![0.0f32; FRAMES * L::WIDTH];
    for (index, sample) in out.iter_mut().enumerate() {
        let frame = index / L::WIDTH;
        let lane = index % L::WIDTH;
        let phase = (frame as f32) * 0.07 + (lane as f32) * 0.31;
        let amplitude = if (100..300).contains(&frame) {
            0.9
        } else {
            0.02
        };
        *sample = amplitude * phase.sin();
    }
    out
}

fn render<L: Lane>(partition: usize) -> (Vec<f32>, Vec<u32>) {
    let mut buffer = signal::<L>();
    let mut processor = Processor::<L>::new();
    let mut done = 0;
    while done < FRAMES {
        let this = partition.min(FRAMES - done);
        let start = done * L::WIDTH;
        let end = (done + this) * L::WIDTH;
        processor.process_block(&mut buffer[start..end], this);
        done += this;
    }
    let mut state = vec![0u32; L::WIDTH];
    processor.envelope.store_bits(&mut state);
    state.push(processor.makeup.current.to_bits());
    state.push(processor.makeup.step.to_bits());
    state.push(processor.makeup.remaining);
    (buffer, state)
}

fn assert_partition_invariant<L: Lane>(width: &str) {
    let (reference, reference_state) = render::<L>(FRAMES);
    for partition in PARTITIONS {
        let (actual, state) = render::<L>(partition);
        assert_eq!(actual.len(), reference.len());
        for (index, (a, b)) in actual.iter().zip(&reference).enumerate() {
            assert_eq!(
                a.to_bits(),
                b.to_bits(),
                "{width}, blocks of {partition}: sample {index}: {a} vs {b}"
            );
        }
        assert_eq!(
            state, reference_state,
            "{width}, blocks of {partition}: state after the render"
        );
    }
}

#[test]
fn the_composition_is_partition_invariant_at_width_one() {
    assert_partition_invariant::<f32>("W=1");
}

#[test]
fn the_composition_is_partition_invariant_at_width_four() {
    assert_partition_invariant::<Simd4>("W=4");
}

#[test]
fn the_composition_is_partition_invariant_at_width_eight() {
    assert_partition_invariant::<Simd8>("W=8");
}

/// The whole composition is also width-independent: lane 0 of every width sees the same samples,
/// so it must produce the same bits.
#[test]
fn the_composition_is_width_independent() {
    let (scalar, _) = render::<f32>(64);
    let (four, _) = render::<Simd4>(64);
    let (eight, _) = render::<Simd8>(64);
    for frame in 0..FRAMES {
        assert_eq!(
            scalar[frame].to_bits(),
            four[frame * 4].to_bits(),
            "frame {frame}: W=1 vs W=4 lane 0"
        );
        assert_eq!(
            scalar[frame].to_bits(),
            eight[frame * 8].to_bits(),
            "frame {frame}: W=1 vs W=8 lane 0"
        );
    }
}
