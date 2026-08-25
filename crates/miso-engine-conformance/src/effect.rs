//! Adversarial native-effect runtime conformance probes and the bounded reference mock.
#![allow(missing_docs)]

use std::{
    cell::Cell,
    panic::{AssertUnwindSafe, catch_unwind},
};

use crate::prng::SplitMix64;

use miso_engine_core::{
    EXTENDED_COMPATIBILITY_SAMPLE_RATES, LAUNCH_SAMPLE_RATES, SampleRateHz,
    is_extended_compatibility_sample_rate, is_launch_sample_rate, realtime::audit,
};
use miso_engine_effect_contract::{
    AutomationSpanKind, BankProcessReport, EffectDescriptorV1, EffectId, EffectPrepareError,
    EffectProcessBlock, EffectQuality, LatencySamples, LinkMode, LinkModeSet, NativeEffectFactory,
    ParameterChannel, ParameterChannelPolicy, ParameterDescriptorV1, ParameterDomain, ParameterId,
    ParameterMapping, ParameterUnit, PortDescriptorV1, PortId, PortLayout, PortRole,
    PrepareEffectBankRequest, PrepareEffectLimits, PrepareEffectRequest, PreparedAutomationSpan,
    PreparedBankMetadata, PreparedEffectMetadata, PreparedNativeEffect, PreparedNativeEffectBank,
    PreparedPortsV1, PreparedSidechainPort, ProcessReport, QualityDescriptorV1, ResetKind,
    SmoothingRule, StatePayloadError, StatePayloadInput, StatePayloadOutput, StatePayloadSizes,
    TailSamples, default_initial_values, expected_prepared_metadata, valid_runtime_span,
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
    // Issue #127: the reference mock declares the `(Linear, Linear)` class ladder, so the shared
    // harness exercises a nudgeable parameter rather than proving only the ladder-free path.
    nudge: miso_engine_effect_contract::default_nudge_ladder_v1(
        ParameterUnit::Linear,
        ParameterDomain::Continuous,
        ParameterMapping::Linear,
    ),
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
    observations: &[],
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
    /// Issue #105 F2: a real heap allocation inside the armed render scope. Unlike
    /// [`FaultKind::AllocationHook`] it calls nothing in `miso_engine_core::realtime::audit` --
    /// it is only detected when the consumer's test binary installs a counting global allocator.
    HeapAllocation,
    /// Issue #105 (master plan P1): output that depends on how the render was partitioned.
    PartitionDependent,
    /// Issue #105 E13: `ResetKind::FullToDefaults` that does not restore the prepared state.
    StickyReset,
    /// Issue #105 E12: a bypass path that emits the dry signal without the declared PDC delay.
    ///
    /// Distinct from [`FaultKind::LatencyChangingBypass`], which lies in `metadata()` and is
    /// caught by `metadata.exact` before any audio is rendered. This one reports the contractual
    /// latency and then fails to honour it, which only the bypass reference render can see.
    BypassDelayMismatch,
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
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
        if self.fault != FaultKind::None
            || !request.has_matching_backend_width()
            || request.requests.len() != request.width.lanes() as usize
        {
            return Ok(None);
        }
        let metadata = expected_prepared_metadata(self.descriptor(), request.requests[0])?;
        if request.requests.iter().any(|item| {
            expected_prepared_metadata(self.descriptor(), *item).map_or(true, |candidate| {
                candidate.program_key() != metadata.program_key()
            })
        }) {
            return Ok(None);
        }
        let mut lanes = core::array::from_fn(|_| DualAccumulatorDelay {
            metadata,
            initial_gain: [1.0; 2],
            gain: [1.0; 2],
            delay: [[0.0; 3]; 2],
            accumulator: [0.0; 2],
            active: [None; 2],
            delay_index: 0,
            metadata_calls: 0,
            snapshot_calls: Cell::new(0),
            fault: FaultKind::None,
        });
        for (index, item) in request.requests.iter().enumerate() {
            let left = item
                .initial_values
                .iter()
                .find(|value| value.channel == ParameterChannel::Left)
                .map_or(1.0, |value| value.value);
            let right = item
                .initial_values
                .iter()
                .find(|value| value.channel == ParameterChannel::Right)
                .map_or(1.0, |value| value.value);
            lanes[index].initial_gain = [left, right];
            lanes[index].gain = [left, right];
        }
        Ok(Some(Box::new(DualAccumulatorDelayBank {
            metadata: PreparedBankMetadata {
                width: request.width,
                program_key: metadata.program_key(),
            },
            lanes,
        })))
    }
}

struct DualAccumulatorDelayBank {
    metadata: PreparedBankMetadata,
    lanes: [DualAccumulatorDelay; 8],
}
impl PreparedNativeEffectBank for DualAccumulatorDelayBank {
    fn metadata(&self) -> PreparedBankMetadata {
        self.metadata.clone()
    }
    fn reset(&mut self, kind: ResetKind) {
        for lane in &mut self.lanes[..self.metadata.width.lanes() as usize] {
            lane.reset(kind);
        }
    }
    fn process_bank(
        &mut self,
        block: miso_engine_effect_contract::EffectBankProcessBlock<'_>,
    ) -> BankProcessReport {
        let width = self.metadata.width;
        let mut report = BankProcessReport::empty(width);
        let lanes = width.lanes() as usize;
        for frame in 0..block.frames as usize {
            for lane in 0..lanes {
                let index = frame * lanes + lane;
                let sample = block.first_sample + frame as u64;
                let mut left = block.left[index];
                let mut right = block.right[index];
                if !canonical_finite(left) {
                    left = 0.0;
                    report.reports[lane].sanitized_main_samples = report.reports[lane]
                        .sanitized_main_samples
                        .saturating_add(1);
                }
                if !canonical_finite(right) {
                    right = 0.0;
                    report.reports[lane].sanitized_main_samples = report.reports[lane]
                        .sanitized_main_samples
                        .saturating_add(1);
                }
                let (left, recover_left) = self.lanes[lane].process_lane(0, left, sample);
                let (right, recover_right) = self.lanes[lane].process_lane(1, right, sample);
                if recover_left {
                    report.reports[lane].nonfinite_left_blocks =
                        report.reports[lane].nonfinite_left_blocks.saturating_add(1);
                }
                if recover_right {
                    report.reports[lane].nonfinite_right_blocks = report.reports[lane]
                        .nonfinite_right_blocks
                        .saturating_add(1);
                }
                block.left[index] = left;
                block.right[index] = right;
                self.lanes[lane].delay_index = (self.lanes[lane].delay_index + 1) % 3;
            }
        }
        report
    }
    fn snapshot_track_state_payload(
        &self,
        track: u32,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.lanes
            .get(track as usize)
            .ok_or(StatePayloadError {
                code: "effect.bank.track",
            })?
            .snapshot_state_payload(output)
    }
    fn restore_track_state_payload(
        &mut self,
        track: u32,
        version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.lanes
            .get_mut(track as usize)
            .ok_or(StatePayloadError {
                code: "effect.bank.track",
            })?
            .restore_state_payload(version, input)
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
            if self.fault == FaultKind::BypassDelayMismatch {
                input
            } else {
                delayed
            }
        } else {
            delayed * self.gain[lane]
        };
        self.accumulator[lane] += output.abs();
        if !canonical_finite(output) || !canonical_finite(self.accumulator[lane]) {
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
        if self.fault != FaultKind::StickyReset {
            self.delay_index = 0;
        }
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
            FaultKind::HeapAllocation => {
                // A real allocation and a real free inside the armed scope, with no call into the
                // audit hooks at all. Only a counting `GlobalAlloc` sees this (issue #105 F2).
                let scratch = vec![0.0_f32; 16];
                core::hint::black_box(&scratch);
            }
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
                if !canonical_finite(*value) {
                    *value = 0.0;
                    report.sanitized_main_samples = report.sanitized_main_samples.saturating_add(1);
                }
            }
            if let Some((left, right)) = block.sidechain.as_mut() {
                for value in [left[frame], right[frame]] {
                    if !canonical_finite(value) {
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
            if self.fault == FaultKind::PartitionDependent && frame == 0 {
                left += block.left.len() as f32 * 1e-7;
            }
            if recover_left {
                report.nonfinite_left_blocks = report.nonfinite_left_blocks.saturating_add(1);
            }
            if recover_right {
                report.nonfinite_right_blocks = report.nonfinite_right_blocks.saturating_add(1);
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
        .any(|v| !canonical_finite(*v))
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

/// Prove, before any effect is judged, that this test binary can actually observe a violation.
///
/// Issue #105 finding F2: the harness's "allocation detection" was the reference mock calling
/// `audit::forbidden` on itself, so a production effect that allocated in `process` passed. Real
/// detection needs two things from the *consumer*, neither of which this crate can provide for
/// itself: `miso-engine-core`'s `realtime-audit` feature (so `in_render_scope` arms the
/// thread-local depth guard) and a counting `#[global_allocator]` that reports to
/// `record_allocator_violation` (`miso_engine_bench_support::alloc::AuditedAllocator`, installed
/// in count-and-continue mode). Without either, `process.allocation` would be vacuously green for
/// every effect for ever. So the harness arms a scope and allocates in it on purpose: if the
/// counters do not move, the run stops with a named harness failure instead of reporting success.
///
/// `miso_engine_conformance::effect_conformance_test!` sets both up; see its documentation.
fn detection_is_real() -> Option<&'static str> {
    if !audit::in_render_scope(audit::is_render_scope_active) {
        // The consumer forgot `features = ["realtime-audit"]` on its conformance dev-dependency:
        // `in_render_scope` compiled to the inlined identity function and arms nothing.
        return Some("harness.audit_unarmed");
    }
    audit::reset();
    let observed = audit::in_render_scope(|| {
        let scratch = Vec::<u8>::with_capacity(64);
        core::hint::black_box(&scratch);
        audit::snapshot()
    });
    audit::reset();
    (observed.allocations == 0).then_some("harness.allocator_not_installed")
}

/// One `process` call inside an armed render scope, with the audit counters attributed to it.
///
/// Returns `None` when the call panicked. Every failure the counters name is pushed here, so the
/// probes above cannot forget to classify one: the single audited `"process.realtime_or_panic"`
/// string conflated a panic, an allocation, a lock and a syscall into one verdict.
fn armed_process(
    effect: &mut dyn PreparedNativeEffect,
    block: EffectProcessBlock<'_>,
    process_calls: &mut u64,
    failures: &mut Vec<&'static str>,
) -> Option<ProcessReport> {
    // `audit::reset` asserts the scope is not armed; it is called only from out here.
    audit::reset();
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        audit::in_render_scope(|| effect.process(block))
    }));
    *process_calls = process_calls.saturating_add(1);
    let observed = audit::snapshot();
    audit::reset();
    let panicked = outcome.is_err();
    // `audit::forbidden` panics, and the panic payload is itself a heap allocation raised *inside*
    // the armed scope, so on the panicking path the allocation counters are the harness's own
    // unwinding and are not attributable to the effect. The hook counters are incremented before
    // the unwind starts, so they are attributable on both paths.
    if !panicked && observed.allocations + observed.deallocations > 0 {
        failures.push("process.allocation");
    }
    if observed.locks > 0 {
        failures.push("process.lock");
    }
    if observed.logs > 0 {
        failures.push("process.log");
    }
    if observed.file_io + observed.network_io + observed.syscalls > 0 {
        failures.push("process.io");
    }
    if observed.feature_detection > 0 {
        failures.push("process.feature_detection");
    }
    if panicked {
        failures.push("process.panic");
    }
    outcome.ok()
}

/// The effect's own declared resource envelope, instead of a magic constant.
///
/// Issue #105: the harness hard-coded `1 << 20` bytes of state. The delay declares 1,411,368 bytes
/// at 88.2 kHz and 1,536,168 at 96 kHz, so `validate_prepare_request` rejected the harness's own
/// request and every launch gate above 48 kHz reported `prepare.request` for a conforming effect.
/// Deriving the limits from the quality row is both correct and strictly stronger than a constant:
/// it gates that the effect fits inside exactly what it advertises, not inside a megabyte.
fn declared_limits(quality: &QualityDescriptorV1, quantum: u32) -> PrepareEffectLimits {
    let scratch = quality
        .scratch_bytes_per_frame
        .saturating_mul(u64::from(quantum))
        .saturating_add(quality.scratch_fixed_bytes);
    PrepareEffectLimits {
        // A zero limit is rejected as `effect.prepare.capacity`, so an effect that declares no
        // state or no scratch still gets a legal (minimal) budget.
        maximum_total_state_bytes: quality.maximum_state.total().unwrap_or(u64::MAX).max(1),
        maximum_scratch_bytes: scratch.max(1),
        maximum_automation_spans_per_block: 8,
    }
}

/// The deterministic dual-lane render input for the latency, partition and determinism gates.
///
/// A frozen SplitMix64 stream rather than an impulse: passthrough of a single non-zero sample is a
/// far weaker statement about a bypass path than bit-exact passthrough of a full-band signal, and
/// partition invariance is only interesting where consecutive blocks carry different data.
fn reference_input(len: usize) -> (Vec<f32>, Vec<f32>) {
    let mut prng = SplitMix64::new(0x4D49_534F_5F31_3035);
    let mut left = Vec::with_capacity(len);
    let mut right = Vec::with_capacity(len);
    for _ in 0..len {
        left.push(prng.next_bipolar_f32() * 0.5);
        right.push(prng.next_bipolar_f32() * 0.5);
    }
    (left, right)
}

/// Render `compare` samples of the reference input through a fresh instance, `frames` at a time.
fn render_sequence(
    factory: &dyn NativeEffectFactory,
    request: PrepareEffectRequest<'_>,
    frames: u32,
    input: (&[f32], &[f32]),
    compare: usize,
    tier: &mut EffectConformanceTierReport,
) -> Option<(Vec<f32>, Vec<f32>)> {
    let Ok(mut effect) = factory.prepare(request) else {
        tier.failures.push("prepare.factory");
        return None;
    };
    let step = frames as usize;
    let total = compare.div_ceil(step) * step;
    let mut left = input.0[..total].to_vec();
    let mut right = input.1[..total].to_vec();
    for start in (0..total).step_by(step) {
        let block = EffectProcessBlock::new(
            &mut left[start..start + step],
            &mut right[start..start + step],
            None,
            start as u64,
            &[],
            request.quantum,
        )
        .ok()?;
        armed_process(
            effect.as_mut(),
            block,
            &mut tier.process_calls,
            &mut tier.failures,
        )?;
    }
    left.truncate(compare);
    right.truncate(compare);
    Some((left, right))
}

fn bits_equal(a: &[f32], b: &[f32]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.to_bits() == y.to_bits())
}

/// `out[n] == in[n - latency]`, bit-exact, with zeros before `latency`.
fn is_delayed_copy(output: &[f32], input: &[f32], latency: usize) -> bool {
    if output.len() < latency {
        return false;
    }
    output[..latency].iter().all(|value| *value == 0.0)
        && output[latency..]
            .iter()
            .zip(input)
            .all(|(out, dry)| out.to_bits() == dry.to_bits())
}

/// The PDC/bypass contract, master-plan P1 partition invariance, and render determinism.
///
/// This replaces the audited "index of the first non-zero output sample equals the declared
/// latency" heuristic, which is not a property of a conforming effect at all: a linear-phase
/// halfband produces output from sample 0 and carries its declared latency as *group* delay, so
/// the heuristic failed the soft clip (31 samples of halfband delay) and would fail any lookahead
/// FIR. What is contractual is:
///
/// * `bypass = true`: the effect emits the dry input delayed by exactly its declared latency
///   ("prepared effect latency is fixed, and bypass must preserve it"), bit-exact on both lanes.
///   That gates the declared number directly, and it is a *stronger* statement than the heuristic.
/// * `bypass = false`: the rendered output does not depend on how the render was partitioned into
///   blocks (master plan P1), and two fresh instances render the same input identically.
fn latency_and_partition_probe(
    factory: &dyn NativeEffectFactory,
    request: PrepareEffectRequest<'_>,
    expected: PreparedEffectMetadata,
    config: ConformanceConfig,
    tier: &mut EffectConformanceTierReport,
) {
    let quantum = config.quantum as usize;
    let Ok(latency) = usize::try_from(expected.latency.0) else {
        tier.failures.push("latency.declared");
        return;
    };
    let Some(compare) = latency.checked_add(2 * quantum) else {
        tier.failures.push("latency.declared");
        return;
    };
    let (input_left, input_right) = reference_input(compare + quantum);
    let input = (input_left.as_slice(), input_right.as_slice());

    let partitions = [1_u32, config.quantum - 1, config.quantum];
    let mut rendered = Vec::with_capacity(partitions.len());
    for frames in partitions {
        let Some(pair) = render_sequence(factory, request, frames, input, compare, tier) else {
            return;
        };
        rendered.push(pair);
    }
    if request.bypass
        && rendered.iter().any(|(left, right)| {
            !is_delayed_copy(left, input.0, latency) || !is_delayed_copy(right, input.1, latency)
        })
    {
        tier.failures.push("latency.bypass_delay");
    }
    let (whole_left, whole_right) = &rendered[partitions.len() - 1];
    if rendered[..partitions.len() - 1]
        .iter()
        .any(|(left, right)| !bits_equal(left, whole_left) || !bits_equal(right, whole_right))
    {
        tier.failures.push("process.partition_invariance");
    }
    for _ in 0..100 {
        let Some((left, right)) =
            render_sequence(factory, request, config.quantum, input, compare, tier)
        else {
            return;
        };
        if !bits_equal(&left, whole_left) || !bits_equal(&right, whole_right) {
            tier.failures.push("process.determinism");
            break;
        }
    }
}

/// One effect crate's whole conformance test: run the shared harness against its factory.
///
/// Issue #105 F1. The harness had validated nothing but its own reference mock, and a contract
/// whose only conforming implementation is its own mock is not evidence. Every production
/// `NativeEffectFactory` now carries a `tests/conformance.rs` that is this macro and nothing else;
/// `scripts/check-effect-contract.sh` fails if a crate that implements the trait does not have one.
///
/// The consumer's `[dev-dependencies]` must be exactly:
///
/// ```toml
/// miso-engine-conformance = { workspace = true, features = ["realtime-audit"] }
/// miso-engine-bench-support.workspace = true
/// ```
///
/// The feature arms `in_render_scope`; `miso-engine-bench-support` is where the workspace's one
/// audited `#[global_allocator]` lives (#104 phase B), and it is what makes `process.allocation` a
/// real measurement rather than a mock calling the audit hooks on itself. The harness refuses to
/// run without either (`harness.audit_unarmed`, `harness.allocator_not_installed`), so a consumer
/// cannot silently lose the gate by dropping a manifest line.
///
/// The macro must appear only in a `tests/*.rs` binary that has no other `#[global_allocator]`.
#[macro_export]
macro_rules! effect_conformance_test {
    ($factory:expr) => {
        #[test]
        fn passes_effect_contract_conformance() {
            // A `#[global_allocator]` registered by an rlib that no symbol names may not be linked
            // at all, and a silently absent audit reports success for every gate below it (#104
            // F4). Count-and-continue mode is what lets the harness *report* a violation instead
            // of the process aborting inside `GlobalAlloc::alloc`.
            ::miso_engine_bench_support::alloc::assert_installed();
            ::miso_engine_bench_support::alloc::set_mode(
                ::miso_engine_bench_support::alloc::Mode::Count,
            );
            let report = $crate::run_effect_conformance(
                &$factory,
                $crate::ConformanceConfig {
                    quantum: 128,
                    blocks: 1,
                },
            );
            assert!(
                report.passed(),
                "launch gate failures: {:?}",
                report.launch_gates.failures
            );
            assert!(
                report.launch_gates.prepared_configurations > 0,
                "the harness must actually prepare something"
            );
        }
    };
}

/// A left-only impulse must not reach the right lane's state -- in dual-mono only.
///
/// Against a **control render** rather than against the initial state (issue #95): the audited
/// probe compared the right section after the impulse with the right section before it, which
/// conflates "the left input leaked into the right lane" with "time passed". A lookahead ring's
/// write index, a smoother's countdown and a hold timer all advance in both lanes on every block
/// regardless of input, so the audited probe failed every real effect that has any of them. The
/// control instance is prepared identically and rendered for the same number of blocks with
/// silence, so anything that differs is attributable to the impulse alone.
///
/// It is a requirement *in dual-mono only*: `LinkMode::Maximum` and `LinkMode::Average` declare a
/// linked detector, which the contract explicitly permits to be shared, so a left-only impulse
/// legitimately reaches the right lane there.
///
/// Issue #105 gives it its own pair of instances: the main render now drives *both* lanes (F7), so
/// the right lane's state legitimately differs from a silent control's there.
fn lane_isolation_probe(
    factory: &dyn NativeEffectFactory,
    request: PrepareEffectRequest<'_>,
    expected: PreparedEffectMetadata,
    config: ConformanceConfig,
    tier: &mut EffectConformanceTierReport,
) {
    let (Ok(mut impulsed), Ok(mut silent)) = (factory.prepare(request), factory.prepare(request))
    else {
        tier.failures.push("prepare.factory");
        return;
    };
    let frames = config.quantum as usize;
    let blocks = expected.latency.0 as usize / frames + 1;
    for block_index in 0..blocks {
        for (effect, impulse) in [(&mut impulsed, true), (&mut silent, false)] {
            let mut left = vec![0.0; frames];
            let mut right = vec![0.0; frames];
            if impulse && block_index == 0 {
                left[0] = 1.0;
            }
            let Ok(block) = EffectProcessBlock::new(
                &mut left,
                &mut right,
                None,
                (block_index * frames) as u64,
                &[],
                config.quantum,
            ) else {
                tier.failures.push("state.lane_isolation");
                return;
            };
            if armed_process(
                effect.as_mut(),
                block,
                &mut tier.process_calls,
                &mut tier.failures,
            )
            .is_none()
            {
                return;
            }
        }
    }
    match (
        snapshot_payload(impulsed.as_ref(), expected),
        snapshot_payload(silent.as_ref(), expected),
    ) {
        (Some((_, _, impulsed_right)), Some((_, _, silent_right))) => {
            if impulsed_right != silent_right {
                tier.failures.push("state.lane_isolation");
            }
        }
        _ => tier.failures.push("state.snapshot"),
    }
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
    if let Some(failure) = detection_is_real() {
        report.launch_gates.failures.push(failure);
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
                // Issue #95: built from the descriptor, not hard-coded to the mock's single
                // parameter. This is what lets the harness run against a real effect (eval E6).
                let initial: Vec<_> = default_initial_values(descriptor).collect();
                let request = PrepareEffectRequest {
                    sample_rate: quality.sample_rate,
                    quantum: config.quantum,
                    quality: quality.quality,
                    bypass,
                    link_mode,
                    ports: unconnected_ports(descriptor),
                    initial_values: &initial,
                    limits: declared_limits(quality, config.quantum),
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
                let frames = config.quantum as usize;
                let mut left = vec![0.0; frames];
                let mut right = vec![0.0; frames];

                // Issue #95: render as many blocks as the declared latency needs, not one.
                // The audited probe rendered a single `quantum`-frame block and asserted the
                // impulse landed inside it, which is only true for an effect whose latency is
                // shorter than one quantum — the reference mock's three samples. The compressor
                // declares 882 (20 ms of lookahead at 44.1 kHz), so the probe could never have
                // passed for a real effect. Latency is a *sample count*, so the gate is the
                // absolute index of the first non-zero output over the whole render.
                let blocks_for_latency = expected.latency.0 as usize / frames + 1;
                let mut within_bounds = true;
                let mut panicked = false;
                for block_index in 0..blocks_for_latency {
                    left.fill(0.0);
                    right.fill(0.0);
                    if block_index == 0 {
                        // Issue #105 F7: both lanes carry the impulse. A left-only impulse never
                        // drove the right lane's own recurrence at all, so half of every
                        // dual-mono effect was rendered on silence for the whole probe. The two
                        // amplitudes differ so that a lane swap cannot pass either.
                        left[0] = 1.0;
                        right[0] = 0.875;
                    }
                    let first_sample = (block_index * frames) as u64;
                    let block = EffectProcessBlock::new(
                        &mut left,
                        &mut right,
                        None,
                        first_sample,
                        &[],
                        config.quantum,
                    )
                    .expect("valid block");
                    if armed_process(
                        effect.as_mut(),
                        block,
                        &mut tier.process_calls,
                        &mut tier.failures,
                    )
                    .is_none()
                    {
                        panicked = true;
                        break;
                    }
                    within_bounds &=
                        block_is_within_bounds(&left) && block_is_within_bounds(&right);
                }
                if panicked {
                    continue;
                }
                if !within_bounds {
                    tier.failures.push("process.block_bounds");
                }
                if effect.metadata().program_key() != expected.program_key() {
                    tier.failures.push("metadata.changed");
                }
                snapshot_checks(effect.as_mut(), expected, &mut tier.failures);
                if link_mode == LinkMode::DualMono {
                    lane_isolation_probe(factory, request, expected, config, tier);
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
                latency_and_partition_probe(factory, request, expected, config, tier);
                if !sanitization_probe(factory, request, tier) {
                    tier.failures.push("process.input_sanitization");
                }
                if !sidechain_probe(factory, request, tier) {
                    tier.failures.push("process.sidechain_sanitization");
                }
                reset_probe(factory, request, expected, tier);
                if !continuation_probe(factory, request, expected, tier) {
                    tier.failures.push("state.continuation");
                }
            }
        }
    }
    report.launch_gates.finish();
    report.extended_compatibility_probes.finish();
    report
}

/// The descriptor's own sidechain port, unconnected — or `None` when it declares no sidechain.
///
/// Issue #95: the harness used to name the reference mock's `sidechain-in` port unconditionally,
/// so `validate_prepare_request` rejected every effect that declares no sidechain at all (the
/// parametric EQ) or names its port differently. `PreparedSidechainPort::None` and
/// `Unconnected { id, required }` are not interchangeable — the contract checks each against the
/// descriptor — so the request has to be built from the descriptor.
fn unconnected_ports(descriptor: &'static EffectDescriptorV1) -> PreparedPortsV1 {
    let sidechain = descriptor
        .ports
        .iter()
        .find(|port| port.role == PortRole::SidechainInput)
        .map_or(PreparedSidechainPort::None, |port| {
            PreparedSidechainPort::Unconnected {
                id: port.id,
                required: port.required,
            }
        });
    PreparedPortsV1 { sidechain }
}

/// The mock's own per-value canonical-finite predicate.
///
/// Issue #95 deleted `miso_engine_effect_contract::sanitize_sample`: decision D7 says no
/// production effect classifies an individual sample, so the contract no longer offers a helper
/// for doing so. The reference mock still does it, deliberately — it is the *permissive* end of
/// the contract, and the faulty-mock corpus needs a mock that reacts to a poisoned sample. What is
/// gated for a real effect is the D7 property below (`block_is_within_bounds`), not this.
fn canonical_finite(value: f32) -> bool {
    value.is_finite() && (value == 0.0 || value.is_normal())
}

/// Decision D7 / master plan §4.4: what every effect's output block must satisfy.
///
/// `x == x` rejects NaN, `|x| < 1e30` rejects an infinity and a diverging recurrence. This is the
/// block-granular property the bank driver checks with one vector compare; the harness asserts it
/// on the values the effect actually produced.
fn block_is_within_bounds(values: &[f32]) -> bool {
    // `!(|x| < 1e30)` is exactly "NaN or out of range": an ordered compare against NaN is false,
    // so the NaN case needs no separate term. This is the scalar spelling of the one vector
    // compare `miso_engine_effect_runtime::bank::check_block` performs per block.
    values.iter().all(|value| value.abs() < 1.0e30)
}

/// A poisoned input block must leave the *processing* path bounded and unpoisoned (D7).
///
/// Forced to the enabled configuration (issue #95). A bypassed effect is contractually required
/// to emit the dry input delayed by its declared latency — poison in, poison out is the correct
/// answer there, not a fault — and under D7 an effect never sanitises its input at all: input
/// sanitisation happens once per track per block at the track input stage, and what this probe
/// gates is that the effect's own output stays within `x == x && |x| < 1e30`.
fn sanitization_probe(
    factory: &dyn NativeEffectFactory,
    mut request: PrepareEffectRequest<'_>,
    tier: &mut EffectConformanceTierReport,
) -> bool {
    request.bypass = false;
    let Ok(mut effect) = factory.prepare(request) else {
        return false;
    };
    let mut left = [f32::NAN, f32::INFINITY, f32::from_bits(1), -0.0];
    let mut right = [0.0; 4];
    let Ok(block) = EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], request.quantum)
    else {
        return false;
    };
    let Some(report) = armed_process(
        effect.as_mut(),
        block,
        &mut tier.process_calls,
        &mut tier.failures,
    ) else {
        return false;
    };
    // D7: the gate is that a block containing NaN, an infinity and a subnormal leaves the effect
    // within bounds and does not poison it — not that the effect counted three individual
    // samples. A production effect classifies nothing per value (issue #95 F1), so the audited
    // `report.sanitized_main_samples == 3` assertion would have failed every real effect.
    let _ = report;
    block_is_within_bounds(&left) && block_is_within_bounds(&right)
}

/// The same D7 property with the poison arriving on a **connected sidechain**.
///
/// Skipped for an effect that declares no sidechain input, and it names that effect's own port
/// rather than the reference mock's (issue #95). Enabled configuration only, for the reason given
/// on [`sanitization_probe`].
fn sidechain_probe(
    factory: &dyn NativeEffectFactory,
    mut request: PrepareEffectRequest<'_>,
    tier: &mut EffectConformanceTierReport,
) -> bool {
    let Some(port) = factory
        .descriptor()
        .ports
        .iter()
        .find(|port| port.role == PortRole::SidechainInput)
    else {
        return true;
    };
    request.bypass = false;
    request.ports = PreparedPortsV1 {
        sidechain: PreparedSidechainPort::Connected {
            id: port.id,
            required: port.required,
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
    let Some(report) = armed_process(
        effect.as_mut(),
        block,
        &mut tier.process_calls,
        &mut tier.failures,
    ) else {
        return false;
    };
    // D7, as in `sanitization_probe`: a poisoned sidechain must leave the main output within
    // bounds. Counting sidechain samples was the deleted per-value contract.
    let _ = report;
    block_is_within_bounds(&left) && block_is_within_bounds(&right)
}

/// `DiscontinuityKeepParameters` clears history; `FullToDefaults` restores the prepared state.
///
/// Issue #105 E13: the audited probe only checked that the render *after* the discontinuity reset
/// was silent, which says nothing about `FullToDefaults`. The contractual statement is that a
/// full reset is indistinguishable from a fresh `prepare` of the same request, so the gate is
/// byte equality of the two state payloads.
fn reset_probe(
    factory: &dyn NativeEffectFactory,
    request: PrepareEffectRequest<'_>,
    expected: PreparedEffectMetadata,
    tier: &mut EffectConformanceTierReport,
) {
    let (Ok(mut effect), Ok(fresh)) = (factory.prepare(request), factory.prepare(request)) else {
        tier.failures.push("reset.semantics");
        return;
    };
    let mut left = [1.0, 0.0, 0.0, 0.0];
    let mut right = [0.875, 0.0, 0.0, 0.0];
    let Ok(block) = EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], request.quantum)
    else {
        tier.failures.push("reset.semantics");
        return;
    };
    if armed_process(
        effect.as_mut(),
        block,
        &mut tier.process_calls,
        &mut tier.failures,
    )
    .is_none()
    {
        return;
    }
    effect.reset(ResetKind::DiscontinuityKeepParameters);
    let mut left = [0.0; 4];
    let mut right = [0.0; 4];
    let Ok(block) = EffectProcessBlock::new(&mut left, &mut right, None, 4, &[], request.quantum)
    else {
        tier.failures.push("reset.semantics");
        return;
    };
    if armed_process(
        effect.as_mut(),
        block,
        &mut tier.process_calls,
        &mut tier.failures,
    )
    .is_none()
    {
        return;
    }
    if left != [0.0; 4]
        || right != [0.0; 4]
        || effect.metadata().program_key() != expected.program_key()
    {
        tier.failures.push("reset.semantics");
    }
    effect.reset(ResetKind::FullToDefaults);
    if snapshot_payload(effect.as_ref(), expected) != snapshot_payload(fresh.as_ref(), expected) {
        tier.failures.push("reset.snapshot_differs");
    }
}

fn continuation_probe(
    factory: &dyn NativeEffectFactory,
    request: PrepareEffectRequest<'_>,
    expected: PreparedEffectMetadata,
    tier: &mut EffectConformanceTierReport,
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
    if armed_process(
        source.as_mut(),
        block,
        &mut tier.process_calls,
        &mut tier.failures,
    )
    .is_none()
    {
        return false;
    }
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
    if armed_process(
        source.as_mut(),
        source_block,
        &mut tier.process_calls,
        &mut tier.failures,
    )
    .is_none()
        || armed_process(
            restored.as_mut(),
            restored_block,
            &mut tier.process_calls,
            &mut tier.failures,
        )
        .is_none()
    {
        return false;
    }
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
