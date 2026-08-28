//! Issue #143 E6: the limiter's resident tap is a read, not a second recursion.
//!
//! The limiter is the interesting case for "resident means resident" because its reduction word is
//! the *only* recursive word in the crate: `d <- max(target, release * (target - d) + d)`. A read
//! that advanced it by one release step would be indistinguishable from a correct read on any
//! single call and would drift on every subsequent one. So the gate is not "the number looks
//! plausible" but three exact statements:
//!
//! 1. The reading equals the word the **state envelope** writes for the same lane. That envelope
//!    is a separate, already-gated route to the same kernel state, so agreeing with it is
//!    agreeing with the kernel rather than with itself.
//! 2. Two calls with no `process` between them are bit-identical, and so is the third.
//! 3. Observing between two blocks does not change what the second block renders.

#![allow(missing_docs)]

use miso_engine_effect_contract::{
    EffectProcessBlock, EffectQuality, InitialParameterValue, LinkMode, NativeEffectFactory,
    ObservationSample, ParameterChannel, PrepareEffectLimits, PrepareEffectRequest,
    PreparedNativeEffect, PreparedPorts, PreparedSidechainPort, StatePayloadOutput,
};
use miso_engine_true_peak_limiter::{
    TRUE_PEAK_LIMITER_DESCRIPTOR_V1, TRUE_PEAK_LIMITER_PARAMETERS_V1, TruePeakLimiterFactory,
};

/// `words::REDUCTION` in the crate's own per-lane payload layout.
const REDUCTION_WORD: usize = 3;

fn values() -> [InitialParameterValue; 6] {
    core::array::from_fn(|index| InitialParameterValue {
        parameter_index: (index / 2) as u32,
        channel: if index % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: TRUE_PEAK_LIMITER_PARAMETERS_V1[index / 2].default_value,
    })
}

fn quality() -> &'static miso_engine_effect_contract::QualityDescriptor {
    TRUE_PEAK_LIMITER_DESCRIPTOR_V1
        .qualities
        .iter()
        .find(|quality| quality.sample_rate == 48_000)
        .expect("launch rate")
}

fn request(values: &[InitialParameterValue]) -> PrepareEffectRequest<'_> {
    PrepareEffectRequest {
        sample_rate: 48_000,
        quantum: 128,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::Maximum,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: quality().maximum_state.total().expect("state total"),
            maximum_scratch_bytes: 24,
            maximum_automation_spans_per_block: 16,
        },
    }
}

fn observe(effect: &dyn PreparedNativeEffect) -> ObservationSample {
    let mut sample = ObservationSample::default();
    assert!(
        effect.observe_resident(0, &mut sample),
        "the limiter implements its one declared tap"
    );
    sample
}

/// The reduction word the persisted envelope reports for one lane, as a second route to the same
/// kernel state.
fn snapshot_reduction(effect: &dyn PreparedNativeEffect) -> (f32, f32) {
    let sizes = effect.metadata().state_sizes;
    let mut common = vec![0_u8; sizes.common_bytes as usize];
    let mut left = vec![0_u8; sizes.left_bytes as usize];
    let mut right = vec![0_u8; sizes.right_bytes as usize];
    effect
        .snapshot_state_payload(StatePayloadOutput {
            common: &mut common,
            left: &mut left,
            right: &mut right,
        })
        .expect("snapshot");
    let word = |bytes: &[u8], index: usize| {
        f32::from_bits(u32::from_le_bytes(
            bytes[index * 4..index * 4 + 4].try_into().unwrap(),
        ))
    };
    (word(&left, REDUCTION_WORD), word(&right, REDUCTION_WORD))
}

fn render(effect: &mut dyn PreparedNativeEffect, amplitude: f32, blocks: usize) {
    for block in 0..blocks {
        let mut left: Vec<f32> = (0..128)
            .map(|frame| {
                if frame % 2 == 0 {
                    amplitude
                } else {
                    -amplitude
                }
            })
            .collect();
        let mut right = left.clone();
        let first_sample = block as u64 * 128;
        let block =
            EffectProcessBlock::new(&mut left, &mut right, None, first_sample, &[], 128).unwrap();
        let _ = effect.process(block);
    }
}

/// Red mutation: multiply the read by `0.9` (one "release step" applied in the read) -> the
/// envelope comparison and the repeatability comparison both fail.
#[test]
fn the_limiter_reads_the_reduction_word_the_envelope_persists() {
    let values = values();
    let mut effect = TruePeakLimiterFactory.prepare(request(&values)).unwrap();
    render(effect.as_mut(), 0.98, 64);

    let observed = observe(&*effect);
    let (left, right) = snapshot_reduction(&*effect);
    assert_eq!(
        observed.left.to_bits(),
        left.to_bits(),
        "the tap and the state envelope report the same left word"
    );
    assert_eq!(
        observed.right.to_bits(),
        right.to_bits(),
        "the tap and the state envelope report the same right word"
    );
    assert!(
        observed.left > 0.0 && observed.left <= 1.0,
        "the case is not vacuous: the limiter is reducing ({})",
        observed.left
    );
}

/// Two calls, then a third, all bit-identical. `&self` makes this true by construction; the test
/// makes it true observably, which is what catches a read that recurses through interior state.
#[test]
fn a_resident_read_is_repeatable_to_the_bit() {
    let values = values();
    let mut effect = TruePeakLimiterFactory.prepare(request(&values)).unwrap();
    render(effect.as_mut(), 0.98, 64);
    let first = observe(&*effect);
    let second = observe(&*effect);
    let third = observe(&*effect);
    assert_eq!(first.left.to_bits(), second.left.to_bits());
    assert_eq!(second.left.to_bits(), third.left.to_bits());
    assert_eq!(first.right.to_bits(), third.right.to_bits());
    assert!(first.left > 0.0, "not vacuous: {}", first.left);
}

/// Observing between blocks changes nothing the next block renders. This is the property the whole
/// cost split rests on: an armed tap must not be able to alter the signal.
#[test]
fn observing_between_blocks_does_not_move_a_single_output_sample() {
    let values = values();
    let mut watched = TruePeakLimiterFactory.prepare(request(&values)).unwrap();
    let mut unwatched = TruePeakLimiterFactory.prepare(request(&values)).unwrap();
    let mut watched_out = Vec::new();
    let mut unwatched_out = Vec::new();
    for block in 0..32_u64 {
        for (effect, sink, watch) in [
            (watched.as_mut(), &mut watched_out, true),
            (unwatched.as_mut(), &mut unwatched_out, false),
        ] {
            let mut left: Vec<f32> = (0..128)
                .map(|frame| if frame % 2 == 0 { 0.98 } else { -0.98 })
                .collect();
            let mut right = left.clone();
            let process =
                EffectProcessBlock::new(&mut left, &mut right, None, block * 128, &[], 128)
                    .unwrap();
            let _ = effect.process(process);
            if watch {
                let _ = observe(&*effect);
                let _ = observe(&*effect);
            }
            sink.extend(left.iter().map(|value| value.to_bits()));
        }
    }
    assert_eq!(
        watched_out, unwatched_out,
        "observation is not a side effect"
    );
}
