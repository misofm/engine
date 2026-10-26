//! Shared helpers: prepare `miso.reverb` through its factory and render impulse responses.
#![allow(dead_code)]

use effect_contract::{
    EffectProcessBlock, EffectQuality, InitialParameterValue, LinkMode, NativeEffectFactory,
    ParameterChannel, PrepareEffectRequest, PreparedNativeEffect, ProcessReport,
};
use reverb::{REVERB_DESCRIPTOR, REVERB_PARAMETERS, ReverbFactory};

/// Block length of every render here.
pub(crate) const QUANTUM: u32 = 512;

/// Indices of the eight parameters, in descriptor order.
pub(crate) const PRE_DELAY: usize = 0;
pub(crate) const DECAY: usize = 1;
pub(crate) const SIZE: usize = 2;
pub(crate) const DAMPING: usize = 3;
pub(crate) const DIFFUSION: usize = 5;
pub(crate) const WIDTH: usize = 6;

/// The descriptor defaults.
pub(crate) fn defaults() -> [f32; 8] {
    core::array::from_fn(|index| REVERB_PARAMETERS[index].default_value)
}

/// A production instance (the `lane::Native` kernel) at `values`.
pub(crate) fn prepare(values: &[f32; 8], sample_rate: u32) -> Box<dyn PreparedNativeEffect> {
    let initial: [InitialParameterValue; 8] = core::array::from_fn(|index| InitialParameterValue {
        parameter_index: index as u32,
        channel: ParameterChannel::Both,
        value: values[index],
    });
    let quality = REVERB_DESCRIPTOR
        .qualities
        .iter()
        .find(|q| q.sample_rate == sample_rate)
        .expect("launch rate");
    let request = PrepareEffectRequest {
        sample_rate,
        quantum: QUANTUM,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: effect_contract::PreparedPorts {
            sidechain: effect_contract::PreparedSidechainPort::None,
        },
        initial_values: &initial,
        limits: effect_contract::PrepareEffectLimits {
            maximum_total_state_bytes: quality.maximum_state.total().expect("state total"),
            maximum_scratch_bytes: 1,
            maximum_automation_spans_per_block: 16,
        },
        tail_bound: conformance::tail_bound_of(
            Box::new(ReverbFactory),
            sample_rate,
            EffectQuality::Normal,
        ),
    };
    ReverbFactory.prepare(request).expect("prepare").processor
}

/// The response of a fresh instance at `values` to the impulse `(left, right)` at `n = 0`, over
/// `frames` frames: `(output left, output right)`.
pub(crate) fn impulse_response(
    values: &[f32; 8],
    sample_rate: u32,
    impulse: (f32, f32),
    frames: usize,
) -> (Vec<f32>, Vec<f32>) {
    let mut effect = prepare(values, sample_rate);
    let mut left = vec![0.0_f32; frames];
    let mut right = vec![0.0_f32; frames];
    left[0] = impulse.0;
    right[0] = impulse.1;
    for offset in (0..frames).step_by(QUANTUM as usize) {
        let end = (offset + QUANTUM as usize).min(frames);
        let report = effect.process(
            EffectProcessBlock::new(
                &mut left[offset..end],
                &mut right[offset..end],
                None,
                offset as u64,
                &[],
                QUANTUM,
            )
            .expect("block"),
        );
        assert_eq!(report, ProcessReport::default());
    }
    (left, right)
}
