//! Shared fixtures for the transient-shaper gates.
//!
//! Everything here drives the crate through its public factory, so the gates measure the
//! production shape and nothing is reachable that a host could not reach.

#![allow(dead_code)]

use miso_engine_effect_contract::{
    AutomationSpanKind, BankWidth, EffectPrepareError, EffectQuality, InitialParameterValue,
    LinkMode, NativeEffectFactory, ParameterChannel, PrepareEffectBankRequest, PrepareEffectLimits,
    PrepareEffectRequest, PreparedAutomationSpan, PreparedNativeEffect, PreparedNativeEffectBank,
    PreparedPorts, PreparedSidechainPort, StatePayloadOutput, StatePayloadSizes,
};
use miso_engine_effect_runtime::state_payload::{read_f32, read_u32, write_f32, write_u32};
use miso_engine_lane::Backend;
use miso_engine_transient_shaper::{TRANSIENT_SHAPER_PARAMETERS_V1, TransientShaperFactory};

/// Parameters per lane.
pub(crate) const PARAMETER_COUNT: usize = 3;

/// Words in one lane's state section, and its byte length.
pub(crate) const STATE_WORDS: usize = 11;

/// Byte length of one lane's state section.
pub(crate) const LANE_STATE_BYTES: usize = STATE_WORDS * 4;

/// Descriptor defaults, in the contract's declaration order.
pub(crate) fn initial_values() -> [InitialParameterValue; PARAMETER_COUNT * 2] {
    core::array::from_fn(|index| InitialParameterValue {
        parameter_index: (index / 2) as u32,
        channel: if index % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: TRANSIENT_SHAPER_PARAMETERS_V1[index / 2].default_value,
    })
}

/// Descriptor defaults with `attack`, `sustain` and `mix` overridden on both channels.
pub(crate) fn values_of(
    attack: f32,
    sustain: f32,
    mix: f32,
) -> [InitialParameterValue; PARAMETER_COUNT * 2] {
    let mut values = initial_values();
    for (index, value) in [attack, attack, sustain, sustain, mix, mix]
        .into_iter()
        .enumerate()
    {
        values[index].value = value;
    }
    values
}

/// A prepare request at 48 kHz, dual mono, not bypassed.
pub(crate) fn request<'a>(values: &'a [InitialParameterValue]) -> PrepareEffectRequest<'a> {
    request_with(values, 48_000, false, LinkMode::DualMono)
}

/// A prepare request with the rate, bypass flag and link mode spelled out.
pub(crate) fn request_with<'a>(
    values: &'a [InitialParameterValue],
    sample_rate: u32,
    bypass: bool,
    link_mode: LinkMode,
) -> PrepareEffectRequest<'a> {
    request_full(values, sample_rate, 128, bypass, link_mode)
}

/// A prepare request with the quantum spelled out as well.
pub(crate) fn request_full<'a>(
    values: &'a [InitialParameterValue],
    sample_rate: u32,
    quantum: u32,
    bypass: bool,
    link_mode: LinkMode,
) -> PrepareEffectRequest<'a> {
    PrepareEffectRequest {
        sample_rate,
        quantum,
        quality: EffectQuality::Normal,
        bypass,
        link_mode,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 88,
            maximum_scratch_bytes: 24,
            maximum_automation_spans_per_block: 16,
        },
    }
}

/// Prepares a scalar product, panicking on a rejected request.
pub(crate) fn prepare(values: &[InitialParameterValue]) -> Box<dyn PreparedNativeEffect> {
    TransientShaperFactory
        .prepare(request(values))
        .expect("prepare")
}

/// Prepares a scalar product with the rate, bypass flag and link mode spelled out.
pub(crate) fn prepare_with(
    values: &[InitialParameterValue],
    sample_rate: u32,
    bypass: bool,
    link_mode: LinkMode,
) -> Box<dyn PreparedNativeEffect> {
    TransientShaperFactory
        .prepare(request_with(values, sample_rate, bypass, link_mode))
        .expect("prepare")
}

/// The left and right state sections of a scalar product.
pub(crate) fn snapshot(effect: &dyn PreparedNativeEffect) -> (Vec<u8>, Vec<u8>) {
    let sizes = effect.metadata().state_sizes;
    let mut left = vec![0; sizes.left_bytes as usize];
    let mut right = vec![0; sizes.right_bytes as usize];
    effect
        .snapshot_state_payload(
            StatePayloadOutput::new(&mut [], &mut left, &mut right, sizes).expect("output"),
        )
        .expect("snapshot");
    (left, right)
}

/// The left and right state sections of one track of a bank.
pub(crate) fn bank_snapshot(
    bank: &dyn PreparedNativeEffectBank,
    track: u32,
    sizes: StatePayloadSizes,
) -> (Vec<u8>, Vec<u8>) {
    let mut left = vec![0; sizes.left_bytes as usize];
    let mut right = vec![0; sizes.right_bytes as usize];
    bank.snapshot_track_state_payload(
        track,
        StatePayloadOutput::new(&mut [], &mut left, &mut right, sizes).expect("output"),
    )
    .expect("bank snapshot");
    (left, right)
}

/// The error of a bank binding that must be rejected.
pub(crate) fn bank_error(
    result: Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError>,
) -> EffectPrepareError {
    match result {
        Err(error) => error,
        Ok(_) => panic!("bank request must reject"),
    }
}

/// The `f32` at word `word` of a state section.
pub(crate) fn state_f32(bytes: &[u8], word: usize) -> f32 {
    read_f32(bytes, word)
}

/// The `u32` at word `word` of a state section.
pub(crate) fn state_u32(bytes: &[u8], word: usize) -> u32 {
    read_u32(bytes, word)
}

/// The eleven bytes-worth of words one lane's state section must hold.
pub(crate) fn expected_lane_bytes(
    envelopes: [f32; 2],
    ramps: [(f32, f32, u32); PARAMETER_COUNT],
) -> Vec<u8> {
    let mut bytes = vec![0_u8; LANE_STATE_BYTES];
    write_f32(&mut bytes, 0, envelopes[0]);
    write_f32(&mut bytes, 1, envelopes[1]);
    for (index, ramp) in ramps.into_iter().enumerate() {
        let word = 2 + index * 3;
        write_f32(&mut bytes, word, ramp.0);
        write_f32(&mut bytes, word + 1, ramp.1);
        write_u32(&mut bytes, word + 2, ramp.2);
    }
    bytes
}

/// A block-rate `Point` automation span at `first_sample`.
pub(crate) fn point(
    channel: ParameterChannel,
    parameter_index: u32,
    first_sample: u64,
    value: f32,
) -> PreparedAutomationSpan {
    PreparedAutomationSpan {
        kind: AutomationSpanKind::Point,
        channel,
        parameter_index,
        start_sample: first_sample,
        end_sample: first_sample,
        start_value: value,
        end_value: value,
    }
}

/// The bank width this build renders, and the backend that names it.
///
/// There is no runtime SIMD dispatch (D4): the build has exactly one production width, and a plan
/// asking for another one is refused as unavailable. `BankWidth::for_backend` is the workspace's
/// one backend-to-width law (#84 phase A).
pub(crate) fn native_bank() -> Option<(Backend, BankWidth)> {
    let backend = Backend::current();
    BankWidth::for_backend(backend).map(|width| (backend, width))
}

/// A kernel backend and width this build does **not** render, for the unavailable-fallback gate.
pub(crate) fn foreign_bank() -> (Backend, BankWidth) {
    match Backend::current() {
        Backend::Simd8 => (Backend::Simd4, BankWidth::Four),
        _ => (Backend::Simd8, BankWidth::Eight),
    }
}

/// Binds a bank of `lanes` identical members at the native width, or `None` if this build has no
/// bank width.
pub(crate) fn bind_native_bank(
    values: &[[InitialParameterValue; PARAMETER_COUNT * 2]],
    link_mode: LinkMode,
) -> Option<Box<dyn PreparedNativeEffectBank>> {
    bind_native_bank_quantum(values, link_mode, 128)
}

/// Binds a bank at the native width with the quantum spelled out.
pub(crate) fn bind_native_bank_quantum(
    values: &[[InitialParameterValue; PARAMETER_COUNT * 2]],
    link_mode: LinkMode,
    quantum: u32,
) -> Option<Box<dyn PreparedNativeEffectBank>> {
    let (backend, width) = native_bank()?;
    assert_eq!(values.len(), width.lanes() as usize);
    let requests = values
        .iter()
        .map(|values| request_full(values, 48_000, quantum, false, link_mode))
        .collect::<Vec<_>>();
    Some(
        TransientShaperFactory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &requests,
            })
            .expect("bank binding")
            .expect("the native width is available"),
    )
}
