//! Shared fixtures for the soft-clip integration tests.
//!
//! Everything here builds a *prepared* effect through the public contract surface, so the tests
//! exercise what the engine calls and never reach into the crate's private types.

#![allow(dead_code)]
#![allow(unreachable_pub)]

use miso_engine_effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectProcessBlock, EffectQuality, InitialParameterValue,
    LinkMode, NativeEffectFactory, ParameterChannel, PrepareEffectBankRequest, PrepareEffectLimits,
    PrepareEffectRequest, PreparedAutomationSpan, PreparedNativeEffect, PreparedNativeEffectBank,
    PreparedPortsV1, PreparedSidechainPort, ProcessReport, StatePayloadInput, StatePayloadOutput,
};
use miso_engine_lane::Backend;
use miso_engine_soft_clip::{SOFT_CLIP_DESCRIPTOR_V1, SOFT_CLIP_PARAMETERS_V1, SoftClipFactory};

/// Parameters: drive, output, mix.
pub const PARAMETERS: usize = 3;

/// Quantum every fixture prepares at.
pub const QUANTUM: u32 = 128;

/// Total payload bytes of one prepared instance under state layout 2.
pub fn total_state_bytes() -> u64 {
    SOFT_CLIP_DESCRIPTOR_V1.qualities[1]
        .maximum_state
        .total()
        .expect("state sizes")
}

/// The six initial values, in the contract's interleaved `(parameter, channel)` order.
pub fn initial_values() -> [InitialParameterValue; PARAMETERS * 2] {
    core::array::from_fn(|index| InitialParameterValue {
        parameter_index: (index / 2) as u32,
        channel: if index.is_multiple_of(2) {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: SOFT_CLIP_PARAMETERS_V1[index / 2].default_value,
    })
}

/// The six initial values with every parameter set from `(left, right)` pairs.
pub fn values_from(pairs: [(f32, f32); PARAMETERS]) -> [InitialParameterValue; PARAMETERS * 2] {
    let mut values = initial_values();
    for (parameter, (left, right)) in pairs.into_iter().enumerate() {
        values[parameter * 2].value = left;
        values[parameter * 2 + 1].value = right;
    }
    values
}

/// A prepare request at 48 kHz, Normal quality, with caps that admit exactly this effect.
pub fn request<'a>(values: &'a [InitialParameterValue]) -> PrepareEffectRequest<'a> {
    PrepareEffectRequest {
        sample_rate: 48_000,
        quantum: QUANTUM,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPortsV1 {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: total_state_bytes(),
            maximum_scratch_bytes: 24,
            maximum_automation_spans_per_block: 16,
        },
    }
}

/// A prepared scalar instance.
pub fn prepare(values: &[InitialParameterValue]) -> Box<dyn PreparedNativeEffect> {
    SoftClipFactory.prepare(request(values)).expect("prepare")
}

/// Renders one planar block through a scalar instance.
pub fn process(
    effect: &mut dyn PreparedNativeEffect,
    left: &mut [f32],
    right: &mut [f32],
    first_sample: u64,
    automation: &[PreparedAutomationSpan],
) -> ProcessReport {
    effect.process(
        EffectProcessBlock::new(left, right, None, first_sample, automation, QUANTUM)
            .expect("block"),
    )
}

/// The backend token a bank of `width` lanes is requested with on this host.
pub fn backend(width: BankWidth) -> Backend {
    match width {
        BankWidth::Four => Backend::Simd4,
        BankWidth::Eight => Backend::Simd8,
    }
}

/// `true` if this artifact runs banks of `width` lanes natively.
pub fn bank_available(width: BankWidth) -> bool {
    match width {
        BankWidth::Four => cfg!(any(
            target_arch = "aarch64",
            all(target_arch = "wasm32", target_feature = "simd128")
        )),
        BankWidth::Eight => cfg!(any(target_arch = "x86", target_arch = "x86_64")),
    }
}

/// A prepared bank whose lanes all carry `values`, or `None` where the host has no such width.
pub fn prepare_bank(
    width: BankWidth,
    per_lane: &[Vec<InitialParameterValue>],
) -> Option<Box<dyn PreparedNativeEffectBank>> {
    let requests: Vec<PrepareEffectRequest<'_>> =
        per_lane.iter().map(|values| request(values)).collect();
    SoftClipFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: backend(width),
            width,
            requests: &requests,
        })
        .expect("bind")
}

/// Renders one AoSoA block through a bank.
#[allow(clippy::too_many_arguments)]
pub fn process_bank(
    bank: &mut dyn PreparedNativeEffectBank,
    width: BankWidth,
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    first_sample: u64,
    automation: &[PreparedAutomationSpan],
    offsets: &[u32],
) -> miso_engine_effect_contract::BankProcessReport {
    bank.process_bank(
        EffectBankProcessBlock::new(
            left,
            right,
            None,
            frames as u32,
            width,
            first_sample,
            automation,
            offsets,
            QUANTUM,
        )
        .expect("bank block"),
    )
}

/// The three payload sections of a scalar snapshot.
pub fn snapshot(effect: &dyn PreparedNativeEffect) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let sizes = effect.metadata().state_sizes;
    let mut common = vec![0; sizes.common_bytes as usize];
    let mut left = vec![0; sizes.left_bytes as usize];
    let mut right = vec![0; sizes.right_bytes as usize];
    effect
        .snapshot_state_payload(
            StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("sizes"),
        )
        .expect("snapshot");
    (common, left, right)
}

/// The three payload sections of one bank track's snapshot.
pub fn snapshot_bank(
    bank: &dyn PreparedNativeEffectBank,
    track: u32,
) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let sizes = bank.metadata().program_key.state_sizes;
    let mut common = vec![0; sizes.common_bytes as usize];
    let mut left = vec![0; sizes.left_bytes as usize];
    let mut right = vec![0; sizes.right_bytes as usize];
    bank.snapshot_track_state_payload(
        track,
        StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("sizes"),
    )
    .expect("snapshot");
    (common, left, right)
}

/// Borrows a snapshot triple as a payload input.
pub fn as_input(sections: &(Vec<u8>, Vec<u8>, Vec<u8>)) -> StatePayloadInput<'_> {
    StatePayloadInput {
        common: &sections.0,
        left: &sections.1,
        right: &sections.2,
    }
}

/// Reads payload word `word` of a section.
pub fn word(bytes: &[u8], index: usize) -> u32 {
    u32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().expect("word"))
}

/// Reads payload word `word` of a section as an `f32`.
pub fn word_f32(bytes: &[u8], index: usize) -> f32 {
    f32::from_bits(word(bytes, index))
}

/// A canonical block-rate automation point.
pub fn point(
    parameter: u32,
    channel: ParameterChannel,
    value: f32,
    first_sample: u64,
) -> PreparedAutomationSpan {
    PreparedAutomationSpan {
        parameter_index: parameter,
        channel,
        kind: miso_engine_effect_contract::AutomationSpanKind::Point,
        start_sample: first_sample,
        end_sample: first_sample,
        start_value: value,
        end_value: value,
    }
}

/// Bits of a slice, for exact comparisons.
pub fn bits(values: &[f32]) -> Vec<u32> {
    values.iter().map(|value| value.to_bits()).collect()
}
