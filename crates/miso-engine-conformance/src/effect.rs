//! Adversarial native-effect runtime conformance probes and the bounded reference mock.
#![allow(missing_docs)]

use std::{
    cell::Cell,
    panic::{AssertUnwindSafe, catch_unwind},
};

use miso_engine_core::{
    EXTENDED_COMPATIBILITY_SAMPLE_RATES, LAUNCH_SAMPLE_RATES, SampleRateHz,
    is_extended_compatibility_sample_rate, is_launch_sample_rate, realtime::audit,
};
use miso_engine_effect_contract::{
    AutomationSpanKind, EffectDescriptorV1, EffectId, EffectPrepareError, EffectProcessBlock,
    EffectQuality, InitialParameterValue, LatencySamples, LinkMode, LinkModeSet,
    NativeEffectFactory, ParameterChannel, ParameterChannelPolicy, ParameterDescriptorV1,
    ParameterDomain, ParameterId, ParameterMapping, ParameterUnit, PortDescriptorV1, PortId,
    PortLayout, PortRole, PrepareEffectBankRequest, PrepareEffectLimits, PrepareEffectRequest,
    PreparedAutomationSpan, PreparedEffectMetadata, PreparedNativeEffect, PreparedNativeEffectBank,
    PreparedPortsV1, PreparedSidechainPort, ProcessReport, QualityDescriptorV1, ResetKind,
    SmoothingRule, StatePayloadError, StatePayloadInput, StatePayloadOutput, StatePayloadSizes,
    TailSamples, expected_prepared_metadata, sanitize_sample, valid_runtime_span,
    validate_descriptor_v1,
};

const MOCK_ID: EffectId = match EffectId::new("conformance.delay") {
    Ok(value) => value,
    Err(_) => panic!("valid static effect ID"),
};
const MAIN_IN: PortId = match PortId::new("main-in") {
    Ok(value) => value,
    Err(_) => panic!("valid static port ID"),
};
const MAIN_OUT: PortId = match PortId::new("main-out") {
    Ok(value) => value,
    Err(_) => panic!("valid static port ID"),
};
const SIDECHAIN_IN: PortId = match PortId::new("sidechain-in") {
    Ok(value) => value,
    Err(_) => panic!("valid static port ID"),
};
const PARAMETERS: [ParameterDescriptorV1; 1] = [ParameterDescriptorV1 {
    id: ParameterId(1),
    display_name: "Gain",
    display_unit: "linear",
    unit: ParameterUnit::Linear,
    domain: ParameterDomain::Continuous,
    minimum: Some(0.0),
    maximum: Some(2.0),
    default_value: 1.0,
    mapping: ParameterMapping::Linear,
    automation_rate: miso_engine_effect_contract::AutomationRate::Sample,
    channel_policy: ParameterChannelPolicy::PerLane,
    smoothing: SmoothingRule::None,
    smoothing_samples: 0,
    readable: true,
    automatable: true,
    enum_choices: &[],
}];
const PORTS: [PortDescriptorV1; 3] = [
    PortDescriptorV1 {
        id: MAIN_IN,
        role: PortRole::MainInput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptorV1 {
        id: SIDECHAIN_IN,
        role: PortRole::SidechainInput,
        required: false,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptorV1 {
        id: MAIN_OUT,
        role: PortRole::MainOutput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
];
const STATE_SIZES: StatePayloadSizes = StatePayloadSizes {
    common_bytes: 4,
    left_bytes: 56,
    right_bytes: 56,
};
const fn quality(sample_rate: u32) -> QualityDescriptorV1 {
    QualityDescriptorV1 {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(3),
        tail: TailSamples::Finite(3),
        maximum_state: STATE_SIZES,
        scratch_fixed_bytes: 0,
        scratch_bytes_per_frame: 0,
    }
}
const QUALITIES: [QualityDescriptorV1; 8] = [
    quality(LAUNCH_SAMPLE_RATES[0].0),
    quality(LAUNCH_SAMPLE_RATES[1].0),
    quality(LAUNCH_SAMPLE_RATES[2].0),
    quality(LAUNCH_SAMPLE_RATES[3].0),
    quality(EXTENDED_COMPATIBILITY_SAMPLE_RATES[0].0),
    quality(EXTENDED_COMPATIBILITY_SAMPLE_RATES[1].0),
    quality(EXTENDED_COMPATIBILITY_SAMPLE_RATES[2].0),
    quality(EXTENDED_COMPATIBILITY_SAMPLE_RATES[3].0),
];
pub static DUAL_ACCUMULATOR_DELAY_DESCRIPTOR: EffectDescriptorV1 = EffectDescriptorV1 {
    id: MOCK_ID,
    display_name: "Conformance dual accumulator delay",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::DUAL_MONO,
    parameters: &PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FaultKind {
    None,
    AllocationHook,
    DeallocationHook,
    LockHook,
    IoHook,
    NetworkHook,
    LogHook,
    SyscallHook,
    SharedLaneState,
    ChangingMetadata,
    ChangingTail,
    LatencyChangingBypass,
    BadResources,
    MalformedSpanAcceptance,
    NonfinitePropagation,
    NondeterministicSnapshot,
    PartialSnapshot,
    BadRestore,
    ExtendedRatePreparation,
    Panic,
}

pub struct DualAccumulatorDelayFactory {
    fault: FaultKind,
}
impl DualAccumulatorDelayFactory {
    pub const fn correct() -> Self {
        Self {
            fault: FaultKind::None,
        }
    }
    pub const fn faulty(fault: FaultKind) -> Self {
        Self { fault }
    }
}
impl NativeEffectFactory for DualAccumulatorDelayFactory {
    fn descriptor(&self) -> &'static EffectDescriptorV1 {
        &DUAL_ACCUMULATOR_DELAY_DESCRIPTOR
    }
    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        if self.fault == FaultKind::ExtendedRatePreparation
            && is_extended_compatibility_sample_rate(SampleRateHz(request.sample_rate))
        {
            return Err(EffectPrepareError {
                code: "effect.conformance.extended_rate_probe",
            });
        }
        let metadata = expected_prepared_metadata(self.descriptor(), request)?;
        let left = request
            .initial_values
            .iter()
            .find(|v| v.channel == ParameterChannel::Left)
            .map_or(1.0, |v| v.value);
        let right = request
            .initial_values
            .iter()
            .find(|v| v.channel == ParameterChannel::Right)
            .map_or(1.0, |v| v.value);
        Ok(Box::new(DualAccumulatorDelay {
            metadata,
            initial_gain: [left, right],
            gain: [left, right],
            delay: [[0.0; 3]; 2],
            accumulator: [0.0; 2],
            active: [None; 2],
            delay_index: 0,
            metadata_calls: 0,
            snapshot_calls: Cell::new(0),
            fault: self.fault,
        }))
    }
    fn bind_homogeneous_bank(
        &self,
        _request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
        Ok(None)
    }
}

struct DualAccumulatorDelay {
    metadata: PreparedEffectMetadata,
    initial_gain: [f32; 2],
    gain: [f32; 2],
    delay: [[f32; 3]; 2],
    accumulator: [f32; 2],
    active: [Option<PreparedAutomationSpan>; 2],
    delay_index: usize,
    metadata_calls: u64,
    snapshot_calls: Cell<u64>,
    fault: FaultKind,
}
impl DualAccumulatorDelay {
    fn process_lane(&mut self, lane: usize, input: f32, sample: u64) -> (f32, bool) {
        if let Some(span) = self.active[lane] {
            match span.kind {
                AutomationSpanKind::Step => {
                    if sample == span.start_sample {
                        self.gain[lane] = span.start_value;
                    }
                    if sample == span.end_sample {
                        self.gain[lane] = span.end_value;
                        self.active[lane] = None;
                    }
                }
                AutomationSpanKind::Linear | AutomationSpanKind::Exponential => {
                    if let Some(value) =
                        miso_engine_effect_contract::automation_segment_value(span, sample)
                    {
                        self.gain[lane] = value;
                    }
                    if sample == span.end_sample {
                        self.active[lane] = None;
                    }
                }
                AutomationSpanKind::Point => {}
            }
        }
        let delayed = self.delay[lane][self.delay_index];
        self.delay[lane][self.delay_index] = input;
        let output = if self.metadata.bypass {
            delayed
        } else {
            delayed * self.gain[lane]
        };
        self.accumulator[lane] += output.abs();
        if sanitize_sample(output).is_none() || sanitize_sample(self.accumulator[lane]).is_none() {
            self.accumulator[lane] = 0.0;
            (0.0, true)
        } else {
            (output, false)
        }
    }
}
impl PreparedNativeEffect for DualAccumulatorDelay {
    fn metadata(&self) -> PreparedEffectMetadata {
        let mut metadata = self.metadata;
        if self.fault == FaultKind::ChangingMetadata && self.metadata_calls != 0 {
            metadata.latency = LatencySamples(metadata.latency.0 + 1);
        }
        if self.fault == FaultKind::ChangingTail && self.metadata_calls != 0 {
            metadata.tail = TailSamples::Infinite;
        }
        if self.fault == FaultKind::LatencyChangingBypass && metadata.bypass {
            metadata.latency = LatencySamples(0);
        }
        if self.fault == FaultKind::BadResources {
            metadata.scratch_bytes = metadata.scratch_bytes.saturating_add(1);
        }
        metadata
    }
    fn reset(&mut self, kind: ResetKind) {
        self.delay = [[0.0; 3]; 2];
        self.accumulator = [0.0; 2];
        self.active = [None; 2];
        self.delay_index = 0;
        if kind == ResetKind::FullToDefaults {
            self.gain = self.initial_gain;
        }
    }
    fn process(&mut self, mut block: EffectProcessBlock<'_>) -> ProcessReport {
        self.metadata_calls = self.metadata_calls.saturating_add(1);
        use miso_engine_core::realtime::audit::ForbiddenOperation;
        match self.fault {
            FaultKind::AllocationHook => audit::forbidden(ForbiddenOperation::Allocation),
            FaultKind::DeallocationHook => audit::forbidden(ForbiddenOperation::Deallocation),
            FaultKind::LockHook => audit::forbidden(ForbiddenOperation::Lock),
            FaultKind::IoHook => audit::forbidden(ForbiddenOperation::FileIo),
            FaultKind::NetworkHook => audit::forbidden(ForbiddenOperation::NetworkIo),
            FaultKind::LogHook => audit::forbidden(ForbiddenOperation::Log),
            FaultKind::SyscallHook => audit::forbidden(ForbiddenOperation::Syscall),
            FaultKind::Panic => panic!("intentional conformance fault"),
            _ => {}
        }
        let mut report = ProcessReport::default();
        for span in block.automation {
            let valid = valid_runtime_span(
                span,
                self.metadata,
                block.first_sample,
                block.left.len() as u32,
            );
            if !valid && self.fault != FaultKind::MalformedSpanAcceptance {
                report.invalid_spans = report.invalid_spans.saturating_add(1);
                continue;
            }
            let lanes: &[usize] = match span.channel {
                ParameterChannel::Left => &[0],
                ParameterChannel::Right => &[1],
                ParameterChannel::Both => &[0, 1],
            };
            for &lane in lanes {
                match span.kind {
                    AutomationSpanKind::Point => self.gain[lane] = span.start_value,
                    _ => self.active[lane] = Some(*span),
                }
            }
        }
        for frame in 0..block.left.len() {
            let sample = block.first_sample + frame as u64;
            let mut input = [block.left[frame], block.right[frame]];
            for value in &mut input {
                if sanitize_sample(*value).is_none() {
                    *value = 0.0;
                    report.sanitized_main_samples = report.sanitized_main_samples.saturating_add(1);
                }
            }
            if let Some((left, right)) = block.sidechain.as_mut() {
                for value in [left[frame], right[frame]] {
                    if sanitize_sample(value).is_none() {
                        report.sanitized_sidechain_samples =
                            report.sanitized_sidechain_samples.saturating_add(1);
                    }
                }
            }
            let (mut left, recover_left) = self.process_lane(0, input[0], sample);
            let (right, recover_right) = self.process_lane(1, input[1], sample);
            if self.fault == FaultKind::SharedLaneState {
                self.accumulator[1] = self.accumulator[0];
            }
            if self.fault == FaultKind::NonfinitePropagation && frame == 0 {
                left = f32::NAN;
            }
            if recover_left {
                report.recovered_left_samples = report.recovered_left_samples.saturating_add(1);
            }
            if recover_right {
                report.recovered_right_samples = report.recovered_right_samples.saturating_add(1);
            }
            block.left[frame] = left;
            block.right[frame] = right;
            self.delay_index = (self.delay_index + 1) % 3;
        }
        report
    }
    fn snapshot_state_payload(
        &self,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        if self.fault == FaultKind::PartialSnapshot {
            output.common[0] = 1;
            return Err(StatePayloadError {
                code: "effect.state.partial",
            });
        }
        output
            .common
            .copy_from_slice(&(self.delay_index as u32).to_le_bytes());
        if self.fault == FaultKind::NondeterministicSnapshot {
            let calls = self.snapshot_calls.get().saturating_add(1);
            self.snapshot_calls.set(calls);
            output.common[0] ^= calls as u8;
        }
        encode_lane(
            output.left,
            self.delay[0],
            self.gain[0],
            self.accumulator[0],
            self.active[0],
        );
        encode_lane(
            output.right,
            self.delay[1],
            self.gain[1],
            self.accumulator[1],
            self.active[1],
        );
        Ok(())
    }
    fn restore_state_payload(
        &mut self,
        version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        if version != 1 || self.fault == FaultKind::BadRestore {
            return Err(StatePayloadError {
                code: "effect.state.version",
            });
        }
        let index = u32::from_le_bytes(input.common.try_into().map_err(|_| StatePayloadError {
            code: "effect.state.length",
        })?) as usize;
        let left = decode_lane(input.left)?;
        let right = decode_lane(input.right)?;
        if index >= 3 {
            return Err(StatePayloadError {
                code: "effect.state.invalid",
            });
        }
        self.delay_index = index;
        self.delay = [left.0, right.0];
        self.gain = [left.1, right.1];
        self.accumulator = [left.2, right.2];
        self.active = [left.3, right.3];
        Ok(())
    }
}

fn encode_lane(
    output: &mut [u8],
    delay: [f32; 3],
    gain: f32,
    accumulator: f32,
    active: Option<PreparedAutomationSpan>,
) {
    output.fill(0);
    for (index, value) in delay.into_iter().enumerate() {
        output[index * 4..index * 4 + 4].copy_from_slice(&value.to_bits().to_le_bytes());
    }
    output[12..16].copy_from_slice(&gain.to_bits().to_le_bytes());
    output[16..20].copy_from_slice(&accumulator.to_bits().to_le_bytes());
    if let Some(span) = active {
        output[20] = 1;
        output[24..28].copy_from_slice(&(span.kind as u32).to_le_bytes());
        output[28..36].copy_from_slice(&span.start_sample.to_le_bytes());
        output[36..44].copy_from_slice(&span.end_sample.to_le_bytes());
        output[44..48].copy_from_slice(&span.start_value.to_bits().to_le_bytes());
        output[48..52].copy_from_slice(&span.end_value.to_bits().to_le_bytes());
    }
}
type LaneState = ([f32; 3], f32, f32, Option<PreparedAutomationSpan>);
fn decode_lane(input: &[u8]) -> Result<LaneState, StatePayloadError> {
    if input.len() != 56 || input[20] > 1 || input[21..24] != [0; 3] || input[52..56] != [0; 4] {
        return Err(StatePayloadError {
            code: "effect.state.invalid",
        });
    }
    let read = |offset: usize| {
        f32::from_bits(u32::from_le_bytes(
            input[offset..offset + 4]
                .try_into()
                .expect("bounded lane field"),
        ))
    };
    let delay = [read(0), read(4), read(8)];
    let gain = read(12);
    let accumulator = read(16);
    if delay
        .iter()
        .chain([&gain, &accumulator])
        .any(|v| sanitize_sample(*v).is_none())
    {
        return Err(StatePayloadError {
            code: "effect.state.invalid",
        });
    }
    let active = if input[20] == 0 {
        None
    } else {
        let kind = AutomationSpanKind::from_raw(u32::from_le_bytes(
            input[24..28].try_into().expect("bounded lane field"),
        ))
        .ok_or(StatePayloadError {
            code: "effect.state.invalid",
        })?;
        let start_sample =
            u64::from_le_bytes(input[28..36].try_into().expect("bounded lane field"));
        let end_sample = u64::from_le_bytes(input[36..44].try_into().expect("bounded lane field"));
        let start_value = read(44);
        let end_value = read(48);
        if !start_value.is_finite() || !end_value.is_finite() || end_sample <= start_sample {
            return Err(StatePayloadError {
                code: "effect.state.invalid",
            });
        }
        Some(PreparedAutomationSpan {
            kind,
            channel: ParameterChannel::Left,
            parameter_index: 0,
            start_sample,
            end_sample,
            start_value,
            end_value,
        })
    };
    Ok((delay, gain, accumulator, active))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectConformanceTierReport {
    pub failures: Vec<&'static str>,
    pub prepared_configurations: u64,
    pub process_calls: u64,
}

impl EffectConformanceTierReport {
    fn new() -> Self {
        Self {
            failures: Vec::new(),
            prepared_configurations: 0,
            process_calls: 0,
        }
    }

    fn finish(&mut self) {
        self.failures.sort_unstable();
        self.failures.dedup();
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectConformanceReport {
    pub launch_gates: EffectConformanceTierReport,
    pub extended_compatibility_probes: EffectConformanceTierReport,
}
impl EffectConformanceReport {
    pub fn passed(&self) -> bool {
        self.launch_gates.failures.is_empty()
    }
}
#[derive(Clone, Copy, Debug)]
pub struct ConformanceConfig {
    pub quantum: u32,
    pub blocks: u32,
}

pub fn run_effect_conformance(
    factory: &dyn NativeEffectFactory,
    config: ConformanceConfig,
) -> EffectConformanceReport {
    let mut report = EffectConformanceReport {
        launch_gates: EffectConformanceTierReport::new(),
        extended_compatibility_probes: EffectConformanceTierReport::new(),
    };
    let descriptor = factory.descriptor();
    if validate_descriptor_v1(descriptor).is_err() {
        report.launch_gates.failures.push("descriptor.validation");
        return report;
    }
    if config.quantum < 4 || config.blocks == 0 {
        report.launch_gates.failures.push("configuration");
        return report;
    }
    for quality in descriptor.qualities {
        let tier = if is_launch_sample_rate(SampleRateHz(quality.sample_rate)) {
            &mut report.launch_gates
        } else {
            debug_assert!(is_extended_compatibility_sample_rate(SampleRateHz(
                quality.sample_rate
            )));
            &mut report.extended_compatibility_probes
        };
        for link_mode in [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average] {
            if !descriptor.supported_link_modes.contains(link_mode) {
                continue;
            }
            for bypass in [false, true] {
                let initial = [
                    InitialParameterValue {
                        parameter_index: 0,
                        channel: ParameterChannel::Left,
                        value: 1.0,
                    },
                    InitialParameterValue {
                        parameter_index: 0,
                        channel: ParameterChannel::Right,
                        value: 1.0,
                    },
                ];
                let request = PrepareEffectRequest {
                    sample_rate: quality.sample_rate,
                    quantum: config.quantum,
                    quality: quality.quality,
                    bypass,
                    link_mode,
                    ports: PreparedPortsV1 {
                        sidechain: PreparedSidechainPort::Unconnected {
                            id: SIDECHAIN_IN,
                            required: false,
                        },
                    },
                    initial_values: &initial,
                    limits: PrepareEffectLimits {
                        maximum_total_state_bytes: 1 << 20,
                        maximum_scratch_bytes: 1 << 20,
                        maximum_automation_spans_per_block: 8,
                    },
                };
                let expected = match expected_prepared_metadata(descriptor, request) {
                    Ok(v) => v,
                    Err(_) => {
                        tier.failures.push("prepare.request");
                        continue;
                    }
                };
                let mut effect = match factory.prepare(request) {
                    Ok(v) => v,
                    Err(_) => {
                        tier.failures.push("prepare.factory");
                        continue;
                    }
                };
                tier.prepared_configurations += 1;
                if effect.metadata().program_key() != expected.program_key() {
                    tier.failures.push("metadata.exact");
                }
                let initial_right = snapshot_payload(effect.as_ref(), expected)
                    .map(|(_, _, right)| right)
                    .unwrap_or_default();
                let frames = config.quantum as usize;
                let mut left = vec![0.0; frames];
                let mut right = vec![0.0; frames];
                left[0] = 1.0;
                let block =
                    EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], config.quantum)
                        .expect("valid block");
                let process = catch_unwind(AssertUnwindSafe(|| {
                    audit::in_render_scope(|| effect.process(block))
                }));
                tier.process_calls += 1;
                if process.is_err() {
                    tier.failures.push("process.realtime_or_panic");
                    continue;
                }
                if left
                    .iter()
                    .chain(&right)
                    .any(|v| sanitize_sample(*v).is_none())
                {
                    tier.failures.push("process.sanitization");
                }
                if left.iter().position(|v| *v != 0.0) != usize::try_from(expected.latency.0).ok() {
                    tier.failures.push("latency.impulse");
                }
                if effect.metadata().program_key() != expected.program_key() {
                    tier.failures.push("metadata.changed");
                }
                if let Some((_, _, right_after)) =
                    snapshot_checks(effect.as_mut(), expected, &mut tier.failures)
                    && initial_right != right_after
                {
                    tier.failures.push("state.lane_isolation");
                }
                let malformed = [PreparedAutomationSpan {
                    kind: AutomationSpanKind::Point,
                    channel: ParameterChannel::Left,
                    parameter_index: u32::MAX,
                    start_sample: config.quantum as u64,
                    end_sample: config.quantum as u64,
                    start_value: 1.0,
                    end_value: 1.0,
                }];
                let mut malformed_left = [0.0];
                let mut malformed_right = [0.0];
                let block = EffectProcessBlock::new(
                    &mut malformed_left,
                    &mut malformed_right,
                    None,
                    config.quantum as u64,
                    &malformed,
                    config.quantum,
                )
                .expect("shape-valid malformed-span block");
                let malformed_report = effect.process(block);
                tier.process_calls += 1;
                if malformed_report.invalid_spans != 1 {
                    tier.failures.push("automation.malformed");
                }
                for frames in [1, config.quantum - 1, config.quantum] {
                    if !impulse_sequence(
                        factory,
                        request,
                        expected,
                        frames,
                        &mut tier.process_calls,
                    ) {
                        tier.failures.push("latency.frame_boundaries");
                    }
                }
                for _ in 0..100 {
                    if !impulse_sequence(
                        factory,
                        request,
                        expected,
                        config.quantum,
                        &mut tier.process_calls,
                    ) {
                        tier.failures.push("latency.repetition");
                        break;
                    }
                }
                if !sanitization_probe(factory, request, &mut tier.process_calls) {
                    tier.failures.push("process.input_sanitization");
                }
                if !sidechain_probe(factory, request, &mut tier.process_calls) {
                    tier.failures.push("process.sidechain_sanitization");
                }
                if !reset_probe(factory, request, expected, &mut tier.process_calls) {
                    tier.failures.push("reset.semantics");
                }
                if !continuation_probe(factory, request, expected, &mut tier.process_calls) {
                    tier.failures.push("state.continuation");
                }
            }
        }
    }
    report.launch_gates.finish();
    report.extended_compatibility_probes.finish();
    report
}

fn impulse_sequence(
    factory: &dyn NativeEffectFactory,
    request: PrepareEffectRequest<'_>,
    expected: PreparedEffectMetadata,
    frames: u32,
    process_calls: &mut u64,
) -> bool {
    let Ok(mut effect) = factory.prepare(request) else {
        return false;
    };
    let mut absolute = 0_u64;
    let mut observed = None;
    while absolute <= expected.latency.0.saturating_add(frames as u64) {
        let mut left = vec![0.0; frames as usize];
        let mut right = vec![0.0; frames as usize];
        if absolute == 0 {
            left[0] = 1.0;
        }
        let Ok(block) =
            EffectProcessBlock::new(&mut left, &mut right, None, absolute, &[], request.quantum)
        else {
            return false;
        };
        let process = catch_unwind(AssertUnwindSafe(|| {
            audit::in_render_scope(|| effect.process(block))
        }));
        *process_calls = process_calls.saturating_add(1);
        if process.is_err() {
            return false;
        }
        if let Some(index) = left.iter().position(|value| *value != 0.0) {
            observed = Some(absolute + index as u64);
            break;
        }
        absolute = match absolute.checked_add(frames as u64) {
            Some(value) => value,
            None => return false,
        };
    }
    observed == Some(expected.latency.0)
        && effect.metadata().program_key() == expected.program_key()
}

fn sanitization_probe(
    factory: &dyn NativeEffectFactory,
    request: PrepareEffectRequest<'_>,
    process_calls: &mut u64,
) -> bool {
    let Ok(mut effect) = factory.prepare(request) else {
        return false;
    };
    let mut left = [f32::NAN, f32::INFINITY, f32::from_bits(1), -0.0];
    let mut right = [0.0; 4];
    let Ok(block) = EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], request.quantum)
    else {
        return false;
    };
    let Ok(report) = catch_unwind(AssertUnwindSafe(|| {
        audit::in_render_scope(|| effect.process(block))
    })) else {
        return false;
    };
    *process_calls = process_calls.saturating_add(1);
    report.sanitized_main_samples == 3
        && left
            .iter()
            .chain(&right)
            .all(|value| sanitize_sample(*value).is_some())
}

fn sidechain_probe(
    factory: &dyn NativeEffectFactory,
    mut request: PrepareEffectRequest<'_>,
    process_calls: &mut u64,
) -> bool {
    request.ports = PreparedPortsV1 {
        sidechain: PreparedSidechainPort::Connected {
            id: SIDECHAIN_IN,
            required: false,
        },
    };
    let Ok(mut effect) = factory.prepare(request) else {
        return false;
    };
    let mut left = [0.0; 4];
    let mut right = [0.0; 4];
    let side_left = [f32::NAN, 1.0, 0.0, -1.0];
    let side_right = [f32::from_bits(1), -0.0, 0.5, -0.5];
    let Ok(block) = EffectProcessBlock::new(
        &mut left,
        &mut right,
        Some((&side_left, &side_right)),
        0,
        &[],
        request.quantum,
    ) else {
        return false;
    };
    let Ok(report) = catch_unwind(AssertUnwindSafe(|| {
        audit::in_render_scope(|| effect.process(block))
    })) else {
        return false;
    };
    *process_calls = process_calls.saturating_add(1);
    report.sanitized_sidechain_samples == 2
}

fn reset_probe(
    factory: &dyn NativeEffectFactory,
    request: PrepareEffectRequest<'_>,
    expected: PreparedEffectMetadata,
    process_calls: &mut u64,
) -> bool {
    let Ok(mut effect) = factory.prepare(request) else {
        return false;
    };
    let mut left = [1.0, 0.0, 0.0, 0.0];
    let mut right = [0.0; 4];
    let Ok(block) = EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], request.quantum)
    else {
        return false;
    };
    if catch_unwind(AssertUnwindSafe(|| {
        audit::in_render_scope(|| effect.process(block))
    }))
    .is_err()
    {
        return false;
    }
    *process_calls = process_calls.saturating_add(1);
    effect.reset(ResetKind::DiscontinuityKeepParameters);
    let mut left = [0.0; 4];
    let mut right = [0.0; 4];
    let Ok(block) = EffectProcessBlock::new(&mut left, &mut right, None, 4, &[], request.quantum)
    else {
        return false;
    };
    if catch_unwind(AssertUnwindSafe(|| {
        audit::in_render_scope(|| effect.process(block))
    }))
    .is_err()
    {
        return false;
    }
    *process_calls = process_calls.saturating_add(1);
    effect.reset(ResetKind::FullToDefaults);
    left == [0.0; 4]
        && right == [0.0; 4]
        && effect.metadata().program_key() == expected.program_key()
}

fn continuation_probe(
    factory: &dyn NativeEffectFactory,
    request: PrepareEffectRequest<'_>,
    expected: PreparedEffectMetadata,
    process_calls: &mut u64,
) -> bool {
    let (Ok(mut source), Ok(mut restored)) = (factory.prepare(request), factory.prepare(request))
    else {
        return false;
    };
    let mut left = [0.25, -0.5];
    let mut right = [-0.75, 0.125];
    let Ok(block) = EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], request.quantum)
    else {
        return false;
    };
    if catch_unwind(AssertUnwindSafe(|| {
        audit::in_render_scope(|| source.process(block))
    }))
    .is_err()
    {
        return false;
    }
    *process_calls = process_calls.saturating_add(1);
    let Some((common, lane_left, lane_right)) = snapshot_payload(source.as_ref(), expected) else {
        return false;
    };
    let Ok(input) = StatePayloadInput::new(&common, &lane_left, &lane_right, expected.state_sizes)
    else {
        return false;
    };
    if restored
        .restore_state_payload(expected.descriptor.state_layout_version, input)
        .is_err()
    {
        return false;
    }
    let mut source_left = [0.5, 0.75, -0.25, 0.0];
    let mut source_right = [0.125, -0.5, 1.0, 0.0];
    let mut restored_left = source_left;
    let mut restored_right = source_right;
    let Ok(source_block) = EffectProcessBlock::new(
        &mut source_left,
        &mut source_right,
        None,
        2,
        &[],
        request.quantum,
    ) else {
        return false;
    };
    let Ok(restored_block) = EffectProcessBlock::new(
        &mut restored_left,
        &mut restored_right,
        None,
        2,
        &[],
        request.quantum,
    ) else {
        return false;
    };
    if catch_unwind(AssertUnwindSafe(|| {
        audit::in_render_scope(|| source.process(source_block))
    }))
    .is_err()
        || catch_unwind(AssertUnwindSafe(|| {
            audit::in_render_scope(|| restored.process(restored_block))
        }))
        .is_err()
    {
        return false;
    }
    *process_calls = process_calls.saturating_add(2);
    source_left
        .iter()
        .zip(restored_left)
        .all(|(a, b)| a.to_bits() == b.to_bits())
        && source_right
            .iter()
            .zip(restored_right)
            .all(|(a, b)| a.to_bits() == b.to_bits())
}
fn snapshot_checks(
    effect: &mut dyn PreparedNativeEffect,
    metadata: PreparedEffectMetadata,
    failures: &mut Vec<&'static str>,
) -> Option<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    let s = metadata.state_sizes;
    let mut ca = vec![0xaa; s.common_bytes as usize];
    let mut la = vec![0xaa; s.left_bytes as usize];
    let mut ra = vec![0xaa; s.right_bytes as usize];
    let output = StatePayloadOutput::new(&mut ca, &mut la, &mut ra, s).expect("sizes");
    if effect.snapshot_state_payload(output).is_err() {
        failures.push("state.snapshot");
        return None;
    }
    let mut cb = vec![0x55; s.common_bytes as usize];
    let mut lb = vec![0x55; s.left_bytes as usize];
    let mut rb = vec![0x55; s.right_bytes as usize];
    let output = StatePayloadOutput::new(&mut cb, &mut lb, &mut rb, s).expect("sizes");
    if effect.snapshot_state_payload(output).is_err() || ca != cb || la != lb || ra != rb {
        failures.push("state.deterministic");
        return None;
    }
    let input = StatePayloadInput::new(&ca, &la, &ra, s).expect("sizes");
    if effect
        .restore_state_payload(metadata.descriptor.state_layout_version, input)
        .is_err()
    {
        failures.push("state.restore");
        return None;
    }
    Some((ca, la, ra))
}

fn snapshot_payload(
    effect: &dyn PreparedNativeEffect,
    metadata: PreparedEffectMetadata,
) -> Option<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    let sizes = metadata.state_sizes;
    let mut common = vec![0; sizes.common_bytes as usize];
    let mut left = vec![0; sizes.left_bytes as usize];
    let mut right = vec![0; sizes.right_bytes as usize];
    let output = StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).ok()?;
    effect.snapshot_state_payload(output).ok()?;
    Some((common, left, right))
}
