//! Shared fixtures for the gate/expander's integration tests.
#![allow(dead_code, unreachable_pub)]
use miso_engine_lane::Backend;

use miso_engine_effect_contract::{
    AutomationSpanKind, BankWidth, EffectProcessBlock, EffectQuality, InitialParameterValue,
    LinkMode, NativeEffectFactory, ParameterChannel, PortId, PrepareEffectBankRequest,
    PrepareEffectLimits, PrepareEffectRequest, PreparedAutomationSpan, PreparedNativeEffect,
    PreparedNativeEffectBank, PreparedPortsV1, PreparedSidechainPort, ProcessReport,
    StatePayloadOutput,
};
use miso_engine_gate_expander::{
    GATE_EXPANDER_DESCRIPTOR_V1, GATE_EXPANDER_PARAMETERS_V1, GateExpanderFactory,
};

/// Number of frozen parameters.
pub const PARAMETER_COUNT: usize = 8;

/// One prepared parameter set.
pub type Values = [InitialParameterValue; PARAMETER_COUNT * 2];

/// The sidechain port identifier the descriptor declares.
pub fn sidechain_port() -> PortId {
    PortId::new("sidechain-in").expect("static port id")
}

/// Prepares an eight-lane bank from eight per-track parameter sets, or `None` when this build has
/// no eight-lane backend.
pub fn prepare_bank_w8(
    values: &[Values; 8],
    link_mode: LinkMode,
) -> Option<Box<dyn PreparedNativeEffectBank>> {
    prepare_bank(values, link_mode, BankWidth::Eight, Backend::Simd8, 128)
}

/// Prepares a bank of `width` lanes from the first `width` parameter sets.
pub fn prepare_bank(
    values: &[Values; 8],
    link_mode: LinkMode,
    width: BankWidth,
    backend: Backend,
    quantum: u32,
) -> Option<Box<dyn PreparedNativeEffectBank>> {
    let requests: Vec<PrepareEffectRequest<'_>> = values[..width.lanes() as usize]
        .iter()
        .map(|set| {
            let mut request = request_at(set, 48_000, quantum);
            request.link_mode = link_mode;
            request
        })
        .collect();
    GateExpanderFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
        })
        .expect("bank binding")
}

/// The descriptor defaults, in the interleaved left/right order `prepare` requires.
pub fn initial_values() -> Values {
    core::array::from_fn(|index| InitialParameterValue {
        parameter_index: (index / 2) as u32,
        channel: if index % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: GATE_EXPANDER_PARAMETERS_V1[index / 2].default_value,
    })
}

/// Overwrites one parameter's left and right initial values.
pub fn set_parameter(values: &mut Values, parameter: usize, left: f32, right: f32) {
    values[parameter * 2].value = left;
    values[parameter * 2 + 1].value = right;
}

/// A parameter set that keeps the gate audibly active.
pub fn active_values() -> Values {
    let mut values = initial_values();
    set_parameter(&mut values, 0, -20.0, -20.0);
    set_parameter(&mut values, 1, 20.0, 20.0);
    set_parameter(&mut values, 2, 48.0, 48.0);
    set_parameter(&mut values, 3, 6.0, 6.0);
    set_parameter(&mut values, 4, 1.0, 1.0);
    set_parameter(&mut values, 5, 0.0, 0.0);
    set_parameter(&mut values, 6, 5.0, 5.0);
    set_parameter(&mut values, 7, 10.0, 10.0);
    values
}

/// A prepare request at 48 kHz with an unconnected sidechain.
pub fn request(values: &[InitialParameterValue]) -> PrepareEffectRequest<'_> {
    request_at_rate(values, 48_000)
}

/// A prepare request at `sample_rate` with an unconnected sidechain.
pub fn request_at_rate(
    values: &[InitialParameterValue],
    sample_rate: u32,
) -> PrepareEffectRequest<'_> {
    request_at(values, sample_rate, 128)
}

/// A prepare request at `sample_rate` and `quantum` with an unconnected sidechain.
pub fn request_at(
    values: &[InitialParameterValue],
    sample_rate: u32,
    quantum: u32,
) -> PrepareEffectRequest<'_> {
    let quality = GATE_EXPANDER_DESCRIPTOR_V1
        .qualities
        .iter()
        .find(|quality| quality.sample_rate == sample_rate)
        .expect("launch rate");
    PrepareEffectRequest {
        sample_rate,
        quantum,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPortsV1 {
            sidechain: PreparedSidechainPort::Unconnected {
                id: sidechain_port(),
                required: false,
            },
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: quality.maximum_state.total().expect("state total"),
            maximum_scratch_bytes: 64,
            maximum_automation_spans_per_block: 16,
        },
    }
}

/// Prepares one scalar instance.
pub fn prepare(request: PrepareEffectRequest<'_>) -> Box<dyn PreparedNativeEffect> {
    GateExpanderFactory.prepare(request).expect("prepared gate")
}

/// Renders `left`/`right` in place through a scalar instance in `block`-sized chunks.
pub fn render_scalar(
    effect: &mut dyn PreparedNativeEffect,
    left: &mut [f32],
    right: &mut [f32],
    block: usize,
) -> ProcessReport {
    render_scalar_sidechain(effect, left, right, None, block, &[], 0)
}

/// As [`render_scalar`], with an optional sidechain and an automation batch delivered in the block
/// that starts at `first_sample` — the timeline position the whole render begins at.
pub fn render_scalar_sidechain(
    effect: &mut dyn PreparedNativeEffect,
    left: &mut [f32],
    right: &mut [f32],
    sidechain: Option<(&[f32], &[f32])>,
    block: usize,
    automation: &[PreparedAutomationSpan],
    first_sample: u64,
) -> ProcessReport {
    let frames = left.len();
    let quantum = effect.metadata().quantum;
    let mut total = ProcessReport::default();
    let mut start = 0;
    while start < frames {
        let end = (start + block).min(frames);
        let spans: &[PreparedAutomationSpan] = if start == 0 { automation } else { &[] };
        let report = {
            let side = sidechain.map(|(l, r)| (&l[start..end], &r[start..end]));
            effect.process(
                EffectProcessBlock::new(
                    &mut left[start..end],
                    &mut right[start..end],
                    side,
                    first_sample + start as u64,
                    spans,
                    quantum,
                )
                .expect("block"),
            )
        };
        add_report(&mut total, report);
        start = end;
    }
    total
}

/// Accumulates one block's report into a running total.
pub fn add_report(total: &mut ProcessReport, report: ProcessReport) {
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

/// Snapshots one scalar instance's state payload.
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

/// Snapshots one track of a bank.
pub fn snapshot_bank(
    effect: &dyn PreparedNativeEffectBank,
    track: u32,
) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let sizes = effect.metadata().program_key.state_sizes;
    let mut common = vec![0; sizes.common_bytes as usize];
    let mut left = vec![0; sizes.left_bytes as usize];
    let mut right = vec![0; sizes.right_bytes as usize];
    effect
        .snapshot_track_state_payload(
            track,
            StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("sizes"),
        )
        .expect("snapshot");
    (common, left, right)
}

/// Asserts two `f32` slices are equal bit for bit.
pub fn assert_bits_eq(actual: &[f32], expected: &[f32], context: &str) {
    assert_eq!(actual.len(), expected.len(), "{context} length");
    for (index, (&actual, &expected)) in actual.iter().zip(expected).enumerate() {
        assert_eq!(
            actual.to_bits(),
            expected.to_bits(),
            "{context} sample {index}"
        );
    }
}

/// Interleaves eight per-track signals into one AoSoA block.
pub fn packed_w8(samples: &[Vec<f32>]) -> Vec<f32> {
    assert_eq!(samples.len(), 8, "W8 tracks");
    let frames = samples[0].len();
    assert!(samples.iter().all(|track| track.len() == frames));
    let mut packed = vec![0.0; frames * 8];
    for frame in 0..frames {
        for track in 0..8 {
            packed[frame * 8 + track] = samples[track][frame];
        }
    }
    packed
}

/// Extracts one track from an AoSoA block.
pub fn track_of(packed: &[f32], track: usize, width: usize) -> Vec<f32> {
    packed
        .chunks_exact(width)
        .map(|frame| frame[track])
        .collect()
}

/// A deterministic bipolar noise signal.
pub fn noise(seed: u64, frames: usize, amplitude: f32) -> Vec<f32> {
    let mut state = seed | 1;
    (0..frames)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let word = (state >> 32) as u32;
            (f32::from((word >> 16) as u16) * (2.0 / 65_536.0) - 1.0) * amplitude
        })
        .collect()
}

/// One automation point per smoothed parameter and channel at `first_sample`.
pub fn retarget_spans(first_sample: u64) -> [PreparedAutomationSpan; 8] {
    let targets = [
        (-64.0_f32, -60.0_f32),
        (16.0, 12.0),
        (64.0, 48.0),
        (12.0, 8.0),
    ];
    core::array::from_fn(|index| {
        let parameter_index = index / 2;
        let left = index % 2 == 0;
        let value = if left {
            targets[parameter_index].0
        } else {
            targets[parameter_index].1
        };
        PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel: if left {
                ParameterChannel::Left
            } else {
                ParameterChannel::Right
            },
            parameter_index: parameter_index as u32,
            start_sample: first_sample,
            end_sample: first_sample,
            start_value: value,
            end_value: value,
        }
    })
}
