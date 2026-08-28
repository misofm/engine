//! Shared scaffolding for the multiband compressor's black-box gates.
//!
//! Everything here goes through the public contract surface: a factory, a prepared effect or bank,
//! and the state payload. No test in this crate reaches into the render path, which is what makes
//! the lane-identity and partition gates meaningful.

#![allow(dead_code, unreachable_pub)]
use miso_engine_lane::Backend;

use miso_engine_effect_contract::{
    AutomationSpanKind, BankWidth, EffectProcessBlock, EffectQuality, InitialParameterValue,
    LinkMode, NativeEffectFactory, ParameterChannel, PrepareEffectBankRequest, PrepareEffectLimits,
    PrepareEffectRequest, PreparedAutomationSpan, PreparedNativeEffect, PreparedNativeEffectBank,
    PreparedPorts, PreparedSidechainPort, StatePayloadInput, StatePayloadOutput, StatePayloadSizes,
};
use miso_engine_multiband_compressor::{
    MULTIBAND_COMPRESSOR_DESCRIPTOR, MultibandCompressorFactory,
};

/// Parameters in the frozen order.
pub const PARAMETER_COUNT: usize = 12;

/// The descriptor's own defaults, as an initial-value table.
pub fn values() -> [InitialParameterValue; PARAMETER_COUNT * 2] {
    core::array::from_fn(|index| InitialParameterValue {
        parameter_index: (index / 2) as u32,
        channel: if index % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: MULTIBAND_COMPRESSOR_DESCRIPTOR.parameters[index / 2].default_value,
    })
}

/// Eight deliberately different parameter sets, so no two bank lanes share a program.
///
/// Track 0 gets zero lookahead, track 1 five milliseconds and track 2 twenty, which is what makes
/// the per-track detector gather load-bearing; the ratios and thresholds put each track on a
/// different arm of the static curve, and track 7 sits at unity so the identity path is covered.
pub fn varied_values(track: usize) -> [InitialParameterValue; PARAMETER_COUNT * 2] {
    let mut prepared = values();
    for lane in 0..2 {
        prepared[2 + lane].value = [0.0, 5.0, 20.0][track % 3];
        match track % 4 {
            0 => {
                prepared[6 + lane].value = 1.0;
                prepared[12 + lane].value = 0.25;
                prepared[16 + lane].value = 1.0;
                prepared[22 + lane].value = 0.25;
            }
            1 => {
                prepared[4 + lane].value = -45.0;
                prepared[6 + lane].value = 20.0;
                prepared[8 + lane].value = 0.1;
                prepared[10 + lane].value = 5.0;
                prepared[16 + lane].value = 1.0;
            }
            2 => {
                prepared[6 + lane].value = 1.0;
                prepared[14 + lane].value = -45.0;
                prepared[16 + lane].value = 20.0;
                prepared[18 + lane].value = 0.1;
                prepared[20 + lane].value = 5.0;
            }
            _ => {
                prepared[4 + lane].value = -42.0;
                prepared[6 + lane].value = 12.0;
                prepared[8 + lane].value = 0.2;
                prepared[14 + lane].value = -36.0;
                prepared[16 + lane].value = 8.0;
                prepared[18 + lane].value = 0.3;
            }
        }
    }
    if track == 7 {
        for lane in 0..2 {
            prepared[6 + lane].value = 1.0;
            prepared[12 + lane].value = 0.0;
            prepared[16 + lane].value = 1.0;
            prepared[22 + lane].value = 0.0;
        }
    }
    prepared
}

/// A 48 kHz, 128-frame prepare request over `values`.
pub fn request(values: &[InitialParameterValue]) -> PrepareEffectRequest<'_> {
    request_with(values, LinkMode::DualMono, 128, false)
}

/// A prepare request with an explicit link mode, quantum and bypass flag.
pub fn request_with(
    values: &[InitialParameterValue],
    link_mode: LinkMode,
    quantum: u32,
    bypass: bool,
) -> PrepareEffectRequest<'_> {
    PrepareEffectRequest {
        sample_rate: 48_000,
        quantum,
        quality: EffectQuality::Normal,
        bypass,
        link_mode,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: u64::MAX,
            maximum_scratch_bytes: u64::MAX,
            maximum_automation_spans_per_block: 32,
        },
    }
}

/// The backend token a bank of `width` lanes must carry under the contract's own rule.
///
/// The token is the contract's, not this crate's: since D4 there is no runtime SIMD dispatch and
/// the crate picks `Simd4` or `Simd8` from the width alone. Tests still have to fill the field in,
/// and `BankWidth::matches_backend` decides which value is legal for which width.
pub const fn backend_for(width: BankWidth) -> Backend {
    match width {
        BankWidth::Four => Backend::Simd4,
        BankWidth::Eight => Backend::Simd8,
    }
}

/// Prepares a bank of `width` lanes over `requests`.
pub fn bank(
    width: BankWidth,
    requests: &[PrepareEffectRequest<'_>],
) -> Box<dyn PreparedNativeEffectBank> {
    MultibandCompressorFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: backend_for(width),
            width,
            requests,
        })
        .expect("legal bank request")
        .expect("bank")
}

/// One automation point on `parameter_index` of `channel`, landing on `first_sample`.
pub fn point(
    parameter_index: u32,
    channel: ParameterChannel,
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

/// Runs one scalar block.
pub fn process(
    effect: &mut dyn PreparedNativeEffect,
    left: &mut [f32],
    right: &mut [f32],
    first_sample: u64,
    spans: &[PreparedAutomationSpan],
    quantum: u32,
) {
    effect.process(
        EffectProcessBlock::new(left, right, None, first_sample, spans, quantum).expect("block"),
    );
}

/// A snapshot of one prepared scalar effect, as its three sections.
pub fn snapshot(effect: &dyn PreparedNativeEffect) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let sizes = effect.metadata().state_sizes;
    let mut sections = new_sections(sizes);
    effect
        .snapshot_state_payload(
            StatePayloadOutput::new(&mut sections.0, &mut sections.1, &mut sections.2, sizes)
                .expect("payload"),
        )
        .expect("snapshot");
    sections
}

/// A snapshot of one bank track.
pub fn snapshot_track(
    bank: &dyn PreparedNativeEffectBank,
    track: u32,
    sizes: StatePayloadSizes,
) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut sections = new_sections(sizes);
    bank.snapshot_track_state_payload(
        track,
        StatePayloadOutput::new(&mut sections.0, &mut sections.1, &mut sections.2, sizes)
            .expect("payload"),
    )
    .expect("snapshot");
    sections
}

/// Restores one prepared scalar effect from three sections.
pub fn restore(
    effect: &mut dyn PreparedNativeEffect,
    version: u32,
    sections: &(Vec<u8>, Vec<u8>, Vec<u8>),
    sizes: StatePayloadSizes,
) -> Result<(), miso_engine_effect_contract::StatePayloadError> {
    effect.restore_state_payload(
        version,
        StatePayloadInput::new(&sections.0, &sections.1, &sections.2, sizes).expect("payload"),
    )
}

/// Zeroed sections of the right sizes.
pub fn new_sections(sizes: StatePayloadSizes) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    (
        vec![0u8; sizes.common_bytes as usize],
        vec![0u8; sizes.left_bytes as usize],
        vec![0u8; sizes.right_bytes as usize],
    )
}

/// A deterministic bipolar test signal, seeded per track.
pub fn signal(frames: usize, seed: u64) -> Vec<f32> {
    let mut state = seed | 1;
    (0..frames)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            ((state >> 40) as f32 / 16_777_216.0) * 1.6 - 0.8
        })
        .collect()
}
