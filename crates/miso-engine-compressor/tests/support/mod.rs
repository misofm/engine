//! Fixtures shared by the #88 evals.
//!
//! Everything here goes through the production entry points — `CompressorFactory::prepare`,
//! `bind_homogeneous_bank`, `process`, `process_bank`, the payload calls. No test reaches into the
//! crate's internals, so a test that passes is a statement about the shipped path.
#![allow(dead_code, unreachable_pub)]

use miso_engine_compressor::{COMPRESSOR_PARAMETERS, CompressorFactory};
use miso_engine_effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectProcessBlock, EffectQuality, InitialParameterValue,
    LinkMode, NativeEffectFactory, ParameterChannel, PortId, PrepareEffectBankRequest,
    PrepareEffectLimits, PrepareEffectRequest, PreparedAutomationSpan, PreparedNativeEffect,
    PreparedNativeEffectBank, PreparedPorts, PreparedSidechainPort, ProcessReport,
    StatePayloadInput, StatePayloadOutput,
};
use miso_engine_lane::Backend;

/// Parameters in the descriptor table.
pub const PARAMETER_COUNT: usize = 8;

/// Fixed scalar words before the rings, per channel section.
pub const STATE_HEADER_WORDS: usize = miso_engine_compressor::STATE_HEADER_WORDS;

/// The sidechain port identifier.
pub fn sidechain_port() -> PortId {
    PortId::new("sidechain-in").expect("port id")
}

/// The descriptor defaults, as an initial-value list in contract order.
pub fn initial_values() -> [InitialParameterValue; PARAMETER_COUNT * 2] {
    core::array::from_fn(|index| InitialParameterValue {
        parameter_index: (index / 2) as u32,
        channel: if index % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: COMPRESSOR_PARAMETERS[index / 2].default_value,
    })
}

/// The descriptor defaults with `values[parameter]` on **both** channels.
pub fn values_with(overrides: &[(usize, f32)]) -> [InitialParameterValue; PARAMETER_COUNT * 2] {
    let mut values = initial_values();
    for (parameter, value) in overrides.iter().copied() {
        values[parameter * 2].value = value;
        values[parameter * 2 + 1].value = value;
    }
    values
}

/// A preparation request at 48 kHz with the given quantum.
pub fn request_with_quantum<'a>(
    values: &'a [InitialParameterValue],
    quantum: u32,
) -> PrepareEffectRequest<'a> {
    PrepareEffectRequest {
        sample_rate: 48_000,
        quantum,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::Unconnected {
                id: sidechain_port(),
                required: false,
            },
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 15_568,
            maximum_scratch_bytes: 64,
            maximum_automation_spans_per_block: 16,
        },
    }
}

/// A preparation request at 48 kHz, quantum 128.
pub fn request<'a>(values: &'a [InitialParameterValue]) -> PrepareEffectRequest<'a> {
    request_with_quantum(values, 128)
}

/// Prepares a scalar instance.
pub fn prepare(request: PrepareEffectRequest<'_>) -> Box<dyn PreparedNativeEffect> {
    CompressorFactory.prepare(request).expect("prepare")
}

/// The backend this build was compiled for, as the plan selector sees it, and its bank width.
pub fn native_bank_width() -> Option<(Backend, BankWidth)> {
    let backend = Backend::current();
    BankWidth::for_backend(backend).map(|width| (backend, width))
}

/// Binds a bank of `requests` at this build's width, or `None` if there is no bank width here.
pub fn bind_bank(
    requests: &[PrepareEffectRequest<'_>],
) -> Option<Box<dyn PreparedNativeEffectBank>> {
    let (backend, width) = native_bank_width()?;
    assert_eq!(requests.len(), width.lanes() as usize);
    CompressorFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests,
        })
        .expect("bank bind")
}

/// Renders a scalar instance over `left`/`right` in blocks of `partition` frames.
pub fn render_scalar(
    effect: &mut dyn PreparedNativeEffect,
    left: &mut [f32],
    right: &mut [f32],
    partition: usize,
    quantum: u32,
    spans: &[(u64, PreparedAutomationSpan)],
) -> ProcessReport {
    let mut total = ProcessReport::default();
    let mut offset = 0;
    while offset < left.len() {
        let end = (offset + partition).min(left.len());
        let block_spans: Vec<PreparedAutomationSpan> = spans
            .iter()
            .filter(|(at, _)| *at >= offset as u64 && *at < end as u64)
            .map(|(_, span)| {
                let mut span = *span;
                span.start_sample = offset as u64;
                span.end_sample = offset as u64;
                span
            })
            .collect();
        let report = effect.process(
            EffectProcessBlock::new(
                &mut left[offset..end],
                &mut right[offset..end],
                None,
                offset as u64,
                &block_spans,
                quantum,
            )
            .expect("bounded block"),
        );
        accumulate(&mut total, report);
        offset = end;
    }
    total
}

/// Renders a bank over interleaved-by-lane buffers in blocks of `partition` frames.
#[allow(clippy::too_many_arguments)]
pub fn render_bank(
    bank: &mut dyn PreparedNativeEffectBank,
    left: &mut [f32],
    right: &mut [f32],
    lanes: usize,
    width: BankWidth,
    partition: usize,
    quantum: u32,
    spans: &[(u64, usize, PreparedAutomationSpan)],
) {
    let frames = left.len() / lanes;
    let mut offset = 0;
    while offset < frames {
        let end = (offset + partition).min(frames);
        let count = end - offset;
        let mut flat = Vec::new();
        let mut offsets = vec![0_u32; lanes + 1];
        for track in 0..lanes {
            for (at, lane, span) in spans {
                if *lane == track && *at >= offset as u64 && *at < end as u64 {
                    let mut span = *span;
                    span.start_sample = offset as u64;
                    span.end_sample = offset as u64;
                    flat.push(span);
                }
            }
            offsets[track + 1] = flat.len() as u32;
        }
        bank.process_bank(
            EffectBankProcessBlock::new(
                &mut left[offset * lanes..end * lanes],
                &mut right[offset * lanes..end * lanes],
                None,
                count as u32,
                width,
                offset as u64,
                &flat,
                &offsets,
                quantum,
            )
            .expect("bank block"),
        );
        offset = end;
    }
}

/// Adds one report into a running total.
pub fn accumulate(total: &mut ProcessReport, report: ProcessReport) {
    total.sanitized_main_samples = total
        .sanitized_main_samples
        .saturating_add(report.sanitized_main_samples);
    total.sanitized_sidechain_samples = total
        .sanitized_sidechain_samples
        .saturating_add(report.sanitized_sidechain_samples);
    total.invalid_spans = total.invalid_spans.saturating_add(report.invalid_spans);
    total.nonfinite_left_blocks = total
        .nonfinite_left_blocks
        .saturating_add(report.nonfinite_left_blocks);
    total.nonfinite_right_blocks = total
        .nonfinite_right_blocks
        .saturating_add(report.nonfinite_right_blocks);
}

/// The two channel sections of a scalar instance's payload.
pub fn snapshot(effect: &dyn PreparedNativeEffect) -> (Vec<u8>, Vec<u8>) {
    let sizes = effect.metadata().state_sizes;
    let mut left = vec![0_u8; sizes.left_bytes as usize];
    let mut right = vec![0_u8; sizes.right_bytes as usize];
    effect
        .snapshot_state_payload(
            StatePayloadOutput::new(&mut [], &mut left, &mut right, sizes).expect("payload"),
        )
        .expect("snapshot");
    (left, right)
}

/// The two channel sections of one track of a bank's payload.
pub fn snapshot_track(
    bank: &dyn PreparedNativeEffectBank,
    track: u32,
    sizes_from: &dyn PreparedNativeEffect,
) -> (Vec<u8>, Vec<u8>) {
    let sizes = sizes_from.metadata().state_sizes;
    let mut left = vec![0_u8; sizes.left_bytes as usize];
    let mut right = vec![0_u8; sizes.right_bytes as usize];
    bank.snapshot_track_state_payload(
        track,
        StatePayloadOutput::new(&mut [], &mut left, &mut right, sizes).expect("payload"),
    )
    .expect("snapshot");
    (left, right)
}

/// Restores a scalar instance from two channel sections.
pub fn restore(
    effect: &mut dyn PreparedNativeEffect,
    version: u32,
    left: &[u8],
    right: &[u8],
) -> Result<(), miso_engine_effect_contract::StatePayloadError> {
    let sizes = effect.metadata().state_sizes;
    effect.restore_state_payload(
        version,
        StatePayloadInput::new(&[], left, right, sizes).expect("payload"),
    )
}

/// Deterministic bipolar noise, scaled.
pub fn noise(samples: usize, seed: u64, scale: f32) -> Vec<f32> {
    let mut generator = miso_engine_conformance::SplitMix64::new(seed);
    (0..samples)
        .map(|_| generator.next_bipolar_f32() * scale)
        .collect()
}
