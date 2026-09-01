#![allow(clippy::disallowed_methods)] // D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! E3 / P1 — a block boundary is not observable.
//!
//! The same 4,096-frame stream, with automation events inside it, rendered in partitions of
//! {1, 7, 64, 128, 512} frames and in one shot, must produce the same bits and leave the same
//! state. This is what the segmented ramp driver and the shared history cursor have to survive:
//! the ramp's iterated additions are not `start + n * step`, and the history position is not
//! derived from a frame index.

mod support;

use effect_contract::NativeEffectFactory;
use effect_contract::{
    EffectProcessBlock, ParameterChannel, PreparedAutomationSpan, PreparedNativeEffect,
};
use soft_clip::SoftClipFactory;
use support::{bits, initial_values, values_from};

const FRAMES: usize = 4_096;
const PARTITIONS: [usize; 5] = [1, 7, 64, 128, 512];

fn signal(index: usize) -> f32 {
    let phase = index as f32 * 0.041;
    phase.sin() * 0.9 + (phase * 2.7).sin() * 0.2
}

/// Automation points, keyed by the sample they must start at.
///
/// Only sample 0 carries events. The contract accepts a point only when it starts at the block's
/// first sample, so an event anywhere else would be *seen* by some partitions and *rejected* by
/// others, and the partitions would then legitimately disagree. Sample 0 is the one instant every
/// partition begins a block at. The 64-sample ramps it starts still cross block boundaries in the
/// small partitions and end mid-block in the large ones, which is the property under test.
fn automation_at(first_sample: u64) -> Vec<PreparedAutomationSpan> {
    if first_sample != 0 {
        return Vec::new();
    }
    vec![
        support::point(0, ParameterChannel::Left, 18.0, 0),
        support::point(0, ParameterChannel::Right, -12.0, 0),
        support::point(1, ParameterChannel::Left, -6.0, 0),
        support::point(2, ParameterChannel::Right, 0.25, 0),
    ]
}

fn prepared() -> Box<dyn PreparedNativeEffect> {
    let values = values_from([(0.0, 0.0), (0.0, 0.0), (1.0, 1.0)]);
    let mut request = support::request(&values);
    request.quantum = FRAMES as u32;
    SoftClipFactory.prepare(request).expect("prepare")
}

/// A rendered stream and the state the instance was left in.
struct Rendered {
    left: Vec<f32>,
    right: Vec<f32>,
    state: (Vec<u8>, Vec<u8>, Vec<u8>),
}

/// Renders the stream in blocks of at most `partition` frames.
fn render(partition: usize) -> Rendered {
    let mut effect = prepared();
    let mut left: Vec<f32> = (0..FRAMES).map(signal).collect();
    let mut right: Vec<f32> = (0..FRAMES).map(|index| signal(index + 11)).collect();
    let mut done = 0;
    while done < FRAMES {
        let span = partition.min(FRAMES - done);
        let first_sample = done as u64;
        let automation = automation_at(first_sample);
        effect.process(
            EffectProcessBlock::new(
                &mut left[done..done + span],
                &mut right[done..done + span],
                None,
                first_sample,
                &automation,
                FRAMES as u32,
            )
            .expect("block"),
        );
        done += span;
    }
    let state = support::snapshot(effect.as_ref());
    Rendered { left, right, state }
}

#[test]
fn every_partition_renders_the_one_shot_bits_and_state() {
    let reference = render(1);
    for partition in PARTITIONS.into_iter().chain(core::iter::once(FRAMES)) {
        let actual = render(partition);
        assert_eq!(
            bits(&actual.left),
            bits(&reference.left),
            "left, partition {partition}"
        );
        assert_eq!(
            bits(&actual.right),
            bits(&reference.right),
            "right, partition {partition}"
        );
        assert_eq!(
            actual.state, reference.state,
            "state, partition {partition}"
        );
    }
    // A ramp that never moved would make the whole test vacuous.
    let plain = {
        let values = initial_values();
        let mut effect = support::prepare(&values);
        let mut left: Vec<f32> = (0..128).map(signal).collect();
        let mut right: Vec<f32> = (0..128).map(|index| signal(index + 11)).collect();
        support::process(effect.as_mut(), &mut left, &mut right, 0, &[]);
        left
    };
    assert_ne!(bits(&plain), bits(&reference.left[..128]));
}
