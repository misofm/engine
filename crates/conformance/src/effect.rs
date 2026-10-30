//! Adversarial native-effect runtime conformance probes and the bounded reference mock.
#![allow(missing_docs)]

use std::{
    cell::Cell,
    panic::{AssertUnwindSafe, catch_unwind},
};

use crate::prng::SplitMix64;

use effect_contract::{
    AutomationRate, BankProcessReport, BankWidth, CompositionBound, EffectDescriptor, EffectId,
    EffectPrepareError, EffectProcessBlock, EffectQuality, EffectTargetRequest,
    InitialParameterValue, LatencySamples, LinkMode, LinkModeSet, NativeEffectFactory,
    NativeEffectRegistry, NativeEffectTargetPreparation, NodeTailBound,
    PREPARED_EFFECT_TARGET_WORDS, ParameterChannel, ParameterChannelPolicy, ParameterDescriptor,
    ParameterDomain, ParameterId, ParameterMapping, ParameterUnit, PortDescriptor, PortId,
    PortLayout, PortRole, PrepareEffectBankRequest, PrepareEffectLimits, PrepareEffectRequest,
    PreparedAutomationSpan, PreparedBankMetadata, PreparedEffect, PreparedEffectBank,
    PreparedEffectMetadata, PreparedEffectTarget, PreparedNativeEffect, PreparedNativeEffectBank,
    PreparedPorts, PreparedSidechainPort, ProcessReport, QualityDescriptor, ResetKind, RestBound,
    SmoothingRule, StatePayloadError, StatePayloadInput, StatePayloadOutput, StatePayloadSizes,
    TailSamples, default_initial_values, expected_prepared_metadata, valid_runtime_span,
};
use engine::{LAUNCH_SAMPLE_RATES, realtime::audit};

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
const PARAMETERS: [ParameterDescriptor; 1] = [ParameterDescriptor {
    id: ParameterId(1),
    display_name: "Gain",
    name: "gain",
    display_unit: "linear",
    unit: ParameterUnit::Linear,
    domain: ParameterDomain::Continuous,
    minimum: Some(0.0),
    maximum: Some(2.0),
    default_value: 1.0,
    mapping: ParameterMapping::Linear,
    automation_rate: effect_contract::AutomationRate::Block,
    channel_policy: ParameterChannelPolicy::PerLane,
    smoothing: SmoothingRule::None,
    smoothing_samples: 0,
    readable: true,
    automatable: true,
    enum_choices: &[],
    lattice: effect_contract::ParameterLattice::arithmetic(0.01, 2),
    prepared_reason: None,
}];
const PORTS: [PortDescriptor; 3] = [
    PortDescriptor {
        id: MAIN_IN,
        role: PortRole::MainInput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptor {
        id: SIDECHAIN_IN,
        role: PortRole::SidechainInput,
        required: false,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptor {
        id: MAIN_OUT,
        role: PortRole::MainOutput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
];
const STATE_SIZES: StatePayloadSizes = StatePayloadSizes {
    common_bytes: 4,
    left_bytes: 20,
    right_bytes: 20,
};
const fn quality(sample_rate: u32) -> QualityDescriptor {
    QualityDescriptor {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(3),
        maximum_state: STATE_SIZES,
        scratch_fixed_bytes: 0,
        scratch_bytes_per_frame: 0,
    }
}
const QUALITIES: [QualityDescriptor; 4] = [
    quality(LAUNCH_SAMPLE_RATES[0].0),
    quality(LAUNCH_SAMPLE_RATES[1].0),
    quality(LAUNCH_SAMPLE_RATES[2].0),
    quality(LAUNCH_SAMPLE_RATES[3].0),
];
/// The mock's tail, tail over every peak and exact-rest bound (#1377 D1, D4): its declared
/// three-sample tail, with no exact-rest bound stated, because its output accumulator never rests.
fn tail_and_rest(_: u32, _: EffectQuality) -> NodeTailBound {
    NodeTailBound {
        tail: TailSamples::Finite(3),
        tail_every_peak: TailSamples::Infinite,
        rest: RestBound::Unstated,
        composition: CompositionBound::Unstated,
    }
}
pub static DUAL_ACCUMULATOR_DELAY_DESCRIPTOR: EffectDescriptor = EffectDescriptor {
    id: MOCK_ID,
    display_name: "Conformance dual accumulator delay",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::DUAL_MONO,
    lane_link: false,
    parameters: &PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
    tail_and_rest,
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
    LatencyChangingBypass,
    BadResources,
    MalformedSpanAcceptance,
    NonfinitePropagation,
    NondeterministicSnapshot,
    PartialSnapshot,
    BadRestore,
    Panic,
    /// Issue #105 F2: a real heap allocation inside the armed render scope. Unlike
    /// [`FaultKind::AllocationHook`] it calls nothing in `engine::realtime::audit` --
    /// it is only detected when the consumer's test binary installs a counting global allocator.
    HeapAllocation,
    /// Issue #105 (master plan P1): output that depends on how the render was partitioned.
    PartitionDependent,
    /// Issue #105 E13: `ResetKind::FullToDefaults` that does not restore the prepared state.
    StickyReset,
    /// Issue #105 E12: a bypass path that emits the dry signal without the declared PDC delay.
    ///
    /// Distinct from [`FaultKind::LatencyChangingBypass`], which lies in the metadata its prepare
    /// result carries and is caught by `metadata.exact` before any audio is rendered. This one
    /// reports the contractual latency and then fails to honour it, which only the bypass
    /// reference render can see.
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
    fn descriptor(&self) -> &'static EffectDescriptor {
        &DUAL_ACCUMULATOR_DELAY_DESCRIPTOR
    }
    fn prepared_inline_bytes(&self) -> u64 {
        size_of::<DualAccumulatorDelay>() as u64
    }
    fn prepared_undeclared_heap_bytes(&self, _: &effect_contract::QualityDescriptor) -> u64 {
        0
    }
    fn designed_channel_symmetry(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<bool, EffectPrepareError> {
        self.prepare(request)
            .map(|prepared| prepared.processor.channel_symmetry())
    }

    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<PreparedEffect, EffectPrepareError> {
        let mut metadata = expected_prepared_metadata(self.descriptor(), request)?;
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
        let processor = Box::new(DualAccumulatorDelay {
            bypass: metadata.bypass,
            initial_gain: [left, right],
            gain: [left, right],
            delay: [[0.0; 3]; 2],
            accumulator: [0.0; 2],
            delay_index: 0,
            snapshot_calls: Cell::new(0),
            fault: self.fault,
        });
        // The two metadata faults lie in the prepare result, the only place a reader finds the
        // metadata (issue #1461).
        if self.fault == FaultKind::LatencyChangingBypass && metadata.bypass {
            metadata.latency = LatencySamples(0);
        }
        if self.fault == FaultKind::BadResources {
            metadata.scratch_bytes = metadata.scratch_bytes.saturating_add(1);
        }
        Ok(PreparedEffect {
            processor,
            metadata,
        })
    }
    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<PreparedEffectBank>, EffectPrepareError> {
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
            bypass: metadata.bypass,
            initial_gain: [1.0; 2],
            gain: [1.0; 2],
            delay: [[0.0; 3]; 2],
            accumulator: [0.0; 2],
            delay_index: 0,
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
        Ok(Some(PreparedEffectBank {
            processor: Box::new(DualAccumulatorDelayBank {
                width: request.width,
                lanes,
            }),
            metadata: PreparedBankMetadata {
                width: request.width,
                program_key: metadata.program_key(),
            },
        }))
    }
}

struct DualAccumulatorDelayBank {
    width: BankWidth,
    lanes: [DualAccumulatorDelay; 8],
}
impl PreparedNativeEffectBank for DualAccumulatorDelayBank {
    fn retained_bytes(&self) -> u64 {
        0
    }

    fn reset(&mut self, kind: ResetKind) {
        for lane in &mut self.lanes[..self.width.lanes() as usize] {
            lane.reset(kind);
        }
    }
    fn reset_lane(&mut self, lane: usize, kind: ResetKind) {
        if let Some(state) = self.lanes[..self.width.lanes() as usize].get_mut(lane) {
            state.reset(kind);
        }
    }
    fn process_bank(
        &mut self,
        block: effect_contract::EffectBankProcessBlock<'_>,
    ) -> BankProcessReport {
        let width = self.width;
        let mut report = BankProcessReport::empty(width);
        let lanes = width.lanes() as usize;
        for frame in 0..block.frames as usize {
            for lane in 0..lanes {
                let index = frame * lanes + lane;
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
                let (left, recover_left) = self.lanes[lane].process_lane(0, left);
                let (right, recover_right) = self.lanes[lane].process_lane(1, right);
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

/// The mock keeps, as a plain field, the one prepared value it reads (issue #1461).
struct DualAccumulatorDelay {
    bypass: bool,
    initial_gain: [f32; 2],
    gain: [f32; 2],
    delay: [[f32; 3]; 2],
    accumulator: [f32; 2],
    delay_index: usize,
    snapshot_calls: Cell<u64>,
    fault: FaultKind,
}
impl DualAccumulatorDelay {
    fn process_lane(&mut self, lane: usize, input: f32) -> (f32, bool) {
        let delayed = self.delay[lane][self.delay_index];
        self.delay[lane][self.delay_index] = input;
        let output = if self.bypass {
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
    fn retained_bytes(&self) -> u64 {
        0
    }

    fn reset(&mut self, kind: ResetKind) {
        self.delay = [[0.0; 3]; 2];
        self.accumulator = [0.0; 2];
        if self.fault != FaultKind::StickyReset {
            self.delay_index = 0;
        }
        if kind == ResetKind::FullToDefaults {
            self.gain = self.initial_gain;
        }
    }
    fn process(&mut self, mut block: EffectProcessBlock<'_>) -> ProcessReport {
        use engine::realtime::audit::ForbiddenOperation;
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
                &DUAL_ACCUMULATOR_DELAY_DESCRIPTOR,
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
                self.gain[lane] = span.value;
            }
        }
        for frame in 0..block.left.len() {
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
            let (mut left, recover_left) = self.process_lane(0, input[0]);
            let (right, recover_right) = self.process_lane(1, input[1]);
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
        );
        encode_lane(
            output.right,
            self.delay[1],
            self.gain[1],
            self.accumulator[1],
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
        Ok(())
    }
}

fn encode_lane(output: &mut [u8], delay: [f32; 3], gain: f32, accumulator: f32) {
    output.fill(0);
    for (index, value) in delay.into_iter().enumerate() {
        output[index * 4..index * 4 + 4].copy_from_slice(&value.to_bits().to_le_bytes());
    }
    output[12..16].copy_from_slice(&gain.to_bits().to_le_bytes());
    output[16..20].copy_from_slice(&accumulator.to_bits().to_le_bytes());
}
type LaneState = ([f32; 3], f32, f32);
fn decode_lane(input: &[u8]) -> Result<LaneState, StatePayloadError> {
    if input.len() != 20 {
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
    Ok((delay, gain, accumulator))
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
/// itself: `engine`'s `realtime-audit` feature (so `in_render_scope` arms the
/// thread-local depth guard) and a counting `#[global_allocator]` that reports to
/// `record_allocator_violation` (`bench_support::alloc::AuditedAllocator`, installed
/// in count-and-continue mode). Without either, `process.allocation` would be vacuously green for
/// every effect for ever. So the harness arms a scope and allocates in it on purpose: if the
/// counters do not move, the run stops with a named harness failure instead of reporting success.
///
/// `conformance::effect_conformance_test!` sets both up; see its documentation.
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
fn declared_limits(quality: &QualityDescriptor, quantum: u32) -> PrepareEffectLimits {
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
    let Ok(mut effect) = prepare_processor(factory, request) else {
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

/// The samples of the Point schedule (issue #1661 D1): all but `128` are mid-quantum at `q = 128`,
/// so a mid-quantum Point is compared with a quantum-aligned one; they are
/// they are spaced closer than the launch effects' smoothing, so each Point cuts the ramp before it
/// in flight.
const POINT_SAMPLES: [usize; 9] = [1, 37, 63, 64, 100, 127, 128, 129, 191];

/// The two legal values a scheduled cell alternates between, the first one away from the default.
fn point_values(parameter: &ParameterDescriptor) -> Option<[f32; 2]> {
    let default = parameter.default_value;
    let [a, b] = match parameter.domain {
        ParameterDomain::Continuous => {
            let (low, high) = parameter.minimum.zip(parameter.maximum)?;
            if default - low >= high - default {
                [low, high]
            } else {
                [high, low]
            }
        }
        ParameterDomain::Boolean => [1.0, 0.0],
        ParameterDomain::Enumeration => {
            let [first, second, ..] = parameter.enum_choices else {
                return None;
            };
            [first.value, second.value]
        }
    };
    // Start with the value that moves the cell; both orders alternate the same pair.
    let pair = if a.to_bits() == default.to_bits() {
        [b, a]
    } else {
        [a, b]
    };
    Some(pair.map(effect_contract::normalize_zero))
}

/// The block starts of one partition of `total` samples: `0` and every cut below `total`, sorted.
fn block_starts(total: usize, cuts: impl IntoIterator<Item = usize>) -> Vec<usize> {
    let mut starts: Vec<usize> = cuts.into_iter().filter(|&cut| cut < total).collect();
    starts.push(0);
    starts.sort_unstable();
    starts.dedup();
    starts
}

/// One partitioned render of the Point schedule: the output planes and the final state payload.
struct PointRender {
    left: Vec<f32>,
    right: Vec<f32>,
    payload: Option<(Vec<u8>, Vec<u8>, Vec<u8>)>,
}

/// How the schedule reaches the effect: as `Point` spans, or as prepared targets.
enum PointSchedule {
    /// Every span of the schedule, in the contract's canonical order.
    Spans(Vec<PreparedAutomationSpan>),
    /// Per scheduled sample, the targets the effect's own `prepare_targets` designed for it.
    Targets(Vec<(usize, Vec<PreparedEffectTarget>)>),
}

/// Render the reference input in blocks that start at `starts`, delivering the schedule at its
/// samples: a span in the block of its sample (every scheduled sample is a block start, so every
/// span is at its block's first sample), a target immediately before the block that starts at its
/// sample.
fn render_points(
    factory: &dyn NativeEffectFactory,
    request: PrepareEffectRequest<'_>,
    starts: &[usize],
    schedule: &PointSchedule,
    input: (&[f32], &[f32]),
    tier: &mut EffectConformanceTierReport,
) -> Option<PointRender> {
    let Ok(PreparedEffect {
        processor: mut effect,
        metadata,
    }) = factory.prepare(request)
    else {
        tier.failures.push("prepare.factory");
        return None;
    };
    let total = input.0.len();
    let mut left = input.0.to_vec();
    let mut right = input.1.to_vec();
    for (index, &start) in starts.iter().enumerate() {
        let end = starts.get(index + 1).copied().unwrap_or(total);
        let spans: &[PreparedAutomationSpan] = match schedule {
            PointSchedule::Spans(spans) => {
                let from = spans.partition_point(|span| span.sample < start as u64);
                let to = spans.partition_point(|span| span.sample < end as u64);
                &spans[from..to]
            }
            PointSchedule::Targets(events) => {
                for (_, targets) in events.iter().filter(|(sample, _)| *sample == start) {
                    for target in targets {
                        if effect.apply_prepared_target(target).is_err() {
                            tier.failures.push("process.partition_invariance_points");
                            return None;
                        }
                    }
                }
                &[]
            }
        };
        let Ok(block) = EffectProcessBlock::new(
            &mut left[start..end],
            &mut right[start..end],
            None,
            start as u64,
            spans,
            request.quantum,
        ) else {
            tier.failures.push("process.partition_invariance_points");
            return None;
        };
        let report = armed_process(
            effect.as_mut(),
            block,
            &mut tier.process_calls,
            &mut tier.failures,
        )?;
        if report.invalid_spans != 0 {
            tier.failures.push("process.partition_invariance_points");
        }
    }
    let payload = snapshot_payload(effect.as_ref(), metadata);
    Some(PointRender {
        left,
        right,
        payload,
    })
}

/// The schedule's prepared targets: per scheduled sample, the targets `prepare_targets` designs
/// for the candidate whose `Block` cells (Boolean and Enumeration ones included, #1337) all take
/// that event's value. `None` when a moved cell has no two legal values or a legal candidate does
/// not prepare.
fn point_targets(
    preparation: &dyn NativeEffectTargetPreparation,
    descriptor: &'static EffectDescriptor,
    sample_rate: u32,
    initial: &[InitialParameterValue],
    samples: &[usize],
) -> Option<PointSchedule> {
    let moved: Vec<bool> = initial
        .iter()
        .map(|value| {
            descriptor.parameters[value.parameter_index as usize].automation_rate
                != AutomationRate::None
        })
        .collect();
    let mut candidate = initial.to_vec();
    let mut out = vec![
        PreparedEffectTarget {
            slot: 0,
            channel: ParameterChannel::Left,
            words: [0; PREPARED_EFFECT_TARGET_WORDS],
        };
        preparation.maximum_targets()
    ];
    let mut events = Vec::with_capacity(samples.len());
    for (event, &sample) in samples.iter().enumerate() {
        for (value, &moves) in candidate.iter_mut().zip(&moved) {
            let parameter = &descriptor.parameters[value.parameter_index as usize];
            if moves {
                value.value = point_values(parameter)?[event % 2];
            }
        }
        let count = preparation
            .prepare_targets(
                EffectTargetRequest {
                    sample_rate,
                    values: &candidate,
                    changed: &moved,
                },
                &mut out,
            )
            .ok()?;
        events.push((sample, out[..count].to_vec()));
    }
    Some(PointSchedule::Targets(events))
}

/// The schedule's spans: at every scheduled sample, one `Point` per channel of every `Block` cell.
/// `None` when a `Block` cell has no two legal values to alternate between.
fn point_spans(descriptor: &'static EffectDescriptor, samples: &[usize]) -> Option<PointSchedule> {
    let mut spans = Vec::new();
    for (event, &sample) in samples.iter().enumerate() {
        for (index, parameter) in descriptor.parameters.iter().enumerate() {
            if parameter.automation_rate == AutomationRate::None {
                continue;
            }
            let pair = point_values(parameter)?;
            let channels: &[ParameterChannel] = match parameter.channel_policy {
                ParameterChannelPolicy::Shared => &[ParameterChannel::Both],
                ParameterChannelPolicy::PerLane => {
                    &[ParameterChannel::Left, ParameterChannel::Right]
                }
            };
            for &channel in channels {
                spans.push(PreparedAutomationSpan {
                    channel,
                    parameter_index: index as u32,
                    sample: sample as u64,
                    value: pair[event % 2],
                });
            }
        }
    }
    spans.sort_by_key(|span| (span.sample, span.parameter_index, span.channel));
    Some(PointSchedule::Spans(spans))
}

/// Partition invariance with `Point`s at mid-quantum samples (issue #1661 D1).
///
/// A fixed schedule moves every `Block` cell of the descriptor at [`POINT_SAMPLES`], alternating
/// between two legal values of its domain ([`point_values`]); a per-lane cell takes a `Left` and a
/// `Right` twin, a shared one `Both`. Three partitions render it: (a) one-frame blocks, (b) blocks
/// cut exactly at the scheduled samples and quantum blocks otherwise, and (c) (b) with a further
/// cut at every multiple of `q - 1`. The three outputs must equal bit for bit and the three final
/// state payloads byte for byte, and no report may count an invalid span. Stored automation
/// renders an effect node in pieces that start at event samples (#1662); this is what makes the
/// pieces inaudible.
///
/// An effect that takes prepared targets (the parametric EQ refuses every raw span) takes the
/// schedule as targets from its own `prepare_targets`, applied immediately before the block that
/// starts at the scheduled sample. Every `Block` cell moves there, its Booleans and Enumerations
/// included (the EQ's band enables and families are live since #1337), and the schedule runs twice:
/// once with every Boolean prepared on, so the sections whose values move start in the signal
/// path, and once from the defaults, so sections prepared off are switched on and off by it.
fn partition_with_points_probe(
    factory: &dyn NativeEffectFactory,
    request: PrepareEffectRequest<'_>,
    expected: PreparedEffectMetadata,
    config: ConformanceConfig,
    tier: &mut EffectConformanceTierReport,
) {
    let descriptor = factory.descriptor();
    let quantum = config.quantum as usize;
    let Some(total) = usize::try_from(expected.latency.0)
        .ok()
        .and_then(|latency| latency.checked_add(3 * quantum))
    else {
        tier.failures.push("latency.declared");
        return;
    };
    let samples: Vec<usize> = POINT_SAMPLES
        .into_iter()
        .filter(|&sample| sample < total)
        .collect();
    let defaults: Vec<InitialParameterValue> = default_initial_values(descriptor).collect();
    if let Some(preparation) = factory.target_preparation() {
        let mut on = defaults.clone();
        for value in &mut on {
            if descriptor.parameters[value.parameter_index as usize].domain
                == ParameterDomain::Boolean
            {
                value.value = 1.0;
            }
        }
        for initial in [on, defaults] {
            let schedule = point_targets(
                preparation,
                descriptor,
                request.sample_rate,
                &initial,
                &samples,
            );
            check_point_schedule(
                factory, request, &initial, schedule, &samples, total, quantum, tier,
            );
        }
    } else {
        let schedule = point_spans(descriptor, &samples);
        check_point_schedule(
            factory, request, &defaults, schedule, &samples, total, quantum, tier,
        );
    }
}

/// Renders one Point schedule prepared from `initial` in the three partitions of
/// [`partition_with_points_probe`] and compares them.
#[allow(clippy::too_many_arguments)]
fn check_point_schedule(
    factory: &dyn NativeEffectFactory,
    request: PrepareEffectRequest<'_>,
    initial: &[InitialParameterValue],
    schedule: Option<PointSchedule>,
    samples: &[usize],
    total: usize,
    quantum: usize,
    tier: &mut EffectConformanceTierReport,
) {
    let Some(schedule) = schedule else {
        tier.failures.push("process.partition_invariance_points");
        return;
    };
    let mut request = PrepareEffectRequest {
        initial_values: initial,
        ..request
    };
    if let PointSchedule::Spans(spans) = &schedule {
        let per_sample = spans.len().div_ceil(samples.len().max(1));
        request.limits.maximum_automation_spans_per_block = request
            .limits
            .maximum_automation_spans_per_block
            .max(u32::try_from(per_sample).unwrap_or(u32::MAX));
    }

    let (input_left, input_right) = reference_input(total);
    let input = (input_left.as_slice(), input_right.as_slice());
    let cut_at_points = block_starts(
        total,
        (0..total).step_by(quantum).chain(samples.iter().copied()),
    );
    let forms = [
        block_starts(total, 0..total),
        cut_at_points.clone(),
        block_starts(
            total,
            cut_at_points
                .iter()
                .copied()
                .chain((0..total).step_by(quantum - 1)),
        ),
    ];
    let mut rendered = Vec::with_capacity(forms.len());
    for starts in &forms {
        let Some(render) = render_points(factory, request, starts, &schedule, input, tier) else {
            return;
        };
        rendered.push(render);
    }
    let reference = &rendered[0];
    if reference.payload.is_none()
        || rendered[1..].iter().any(|render| {
            !bits_equal(&render.left, &reference.left)
                || !bits_equal(&render.right, &reference.right)
                || render.payload != reference.payload
        })
    {
        tier.failures.push("process.partition_invariance_points");
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
/// conformance = { workspace = true, features = ["realtime-audit"] }
/// bench-support.workspace = true
/// ```
///
/// The feature arms `in_render_scope`; `bench-support` is where the workspace's one
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
            ::bench_support::alloc::assert_installed();
            ::bench_support::alloc::set_mode(::bench_support::alloc::Mode::Count);
            let report = $crate::run_effect_conformance(
                ::std::boxed::Box::new($factory),
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
    let (Ok(mut impulsed), Ok(mut silent)) = (
        prepare_processor(factory, request),
        prepare_processor(factory, request),
    ) else {
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

/// Runs every launch gate against `factory`.
///
/// The harness admits the factory through a [`NativeEffectRegistry`] of its own, as a host does,
/// so every request carries the registry's tail-bound entry (issue #1462 D1): a descriptor the
/// registry refuses fails `descriptor.validation`, or names the registry's own code (for example
/// `effect.tail_bound.inconsistent`).
pub fn run_effect_conformance(
    factory: Box<dyn NativeEffectFactory>,
    config: ConformanceConfig,
) -> EffectConformanceReport {
    let mut report = EffectConformanceReport {
        launch_gates: EffectConformanceTierReport::new(),
    };
    let descriptor = factory.descriptor();
    let registry = match NativeEffectRegistry::new([factory]) {
        Ok(registry) => registry,
        Err(error) => {
            report.launch_gates.failures.push(match error.code {
                "effect.descriptor.invalid" => "descriptor.validation",
                code => code,
            });
            return report;
        }
    };
    let Some(factory) = registry.get(descriptor.id) else {
        report.launch_gates.failures.push("descriptor.validation");
        return report;
    };
    if config.quantum < 4 || config.blocks == 0 {
        report.launch_gates.failures.push("configuration");
        return report;
    }
    if let Some(failure) = detection_is_real() {
        report.launch_gates.failures.push(failure);
        return report;
    }
    // `validate_descriptor` admits launch-rate rows only (owner ruling R5, #1036), so every
    // declared quality row is a launch gate.
    for quality in descriptor.qualities {
        let tier = &mut report.launch_gates;
        for link_mode in [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average] {
            if !descriptor.supported_link_modes.contains(link_mode) {
                continue;
            }
            for bypass in [false, true] {
                // Issue #95: built from the descriptor, not hard-coded to the mock's single
                // parameter. This is what lets the harness run against a real effect (eval E6).
                let initial: Vec<_> = default_initial_values(descriptor).collect();
                let Ok(tail_bound) =
                    registry.tail_bound(descriptor.id, quality.sample_rate, quality.quality)
                else {
                    tier.failures.push("prepare.request");
                    continue;
                };
                let request = PrepareEffectRequest {
                    sample_rate: quality.sample_rate,
                    quantum: config.quantum,
                    quality: quality.quality,
                    bypass,
                    link_mode,
                    ports: unconnected_ports(descriptor),
                    initial_values: &initial,
                    limits: declared_limits(quality, config.quantum),
                    tail_bound,
                };
                let expected = match expected_prepared_metadata(descriptor, request) {
                    Ok(v) => v,
                    Err(_) => {
                        tier.failures.push("prepare.request");
                        continue;
                    }
                };
                // The prepare result carries the metadata beside the processor, which holds no
                // copy of it (issue #1461), so this is the one place there is a metadata to check.
                let PreparedEffect {
                    processor: mut effect,
                    metadata,
                } = match factory.prepare(request) {
                    Ok(v) => v,
                    Err(_) => {
                        tier.failures.push("prepare.factory");
                        continue;
                    }
                };
                tier.prepared_configurations += 1;
                if metadata.program_key() != expected.program_key() {
                    tier.failures.push("metadata.exact");
                }
                let frames = config.quantum as usize;
                let mut left = vec![0.0; frames];
                let mut right = vec![0.0; frames];

                // Drive both channels through the declared latency and check every output block
                // against D7. The separate latency_and_partition_probe checks exact delayed dry
                // output under bypass and enabled partition invariance; an enabled effect's first
                // nonzero sample need not coincide with its declared group delay.
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
                snapshot_checks(effect.as_mut(), expected, &mut tier.failures);
                if link_mode == LinkMode::DualMono {
                    lane_isolation_probe(factory, request, expected, config, tier);
                }
                let malformed = [PreparedAutomationSpan {
                    channel: ParameterChannel::Left,
                    parameter_index: u32::MAX,
                    sample: config.quantum as u64,
                    value: 1.0,
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
                if !bypass {
                    partition_with_points_probe(factory, request, expected, config, tier);
                }
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
    report
}

/// The descriptor's own sidechain port, unconnected — or `None` when it declares no sidechain.
///
/// Issue #95: the harness used to name the reference mock's `sidechain-in` port unconditionally,
/// so `validate_prepare_request` rejected every effect that declares no sidechain at all (the
/// parametric EQ) or names its port differently. `PreparedSidechainPort::None` and
/// `Unconnected { id, required }` are not interchangeable — the contract checks each against the
/// descriptor — so the request has to be built from the descriptor.
fn unconnected_ports(descriptor: &'static EffectDescriptor) -> PreparedPorts {
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
    PreparedPorts { sidechain }
}

/// The mock's own per-value canonical-finite predicate.
///
/// Issue #95 deleted `effect_contract::sanitize_sample`: decision D7 says no
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
    // compare `effect_runtime::bank::check_block` performs per block.
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
    let Ok(mut effect) = prepare_processor(factory, request) else {
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
    request.ports = PreparedPorts {
        sidechain: PreparedSidechainPort::Connected {
            id: port.id,
            required: port.required,
        },
    };
    let Ok(mut effect) = prepare_processor(factory, request) else {
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
    let (Ok(mut effect), Ok(fresh)) = (
        prepare_processor(factory, request),
        prepare_processor(factory, request),
    ) else {
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
    if left != [0.0; 4] || right != [0.0; 4] {
        tier.failures.push("reset.semantics");
    }
    effect.reset(ResetKind::FullToDefaults);
    if snapshot_payload(effect.as_ref(), expected) != snapshot_payload(fresh.as_ref(), expected) {
        tier.failures.push("reset.snapshot_differs");
    }
}

/// Prepares one instance for a probe that needs only the processor; the probe holds the expected
/// metadata already.
fn prepare_processor(
    factory: &dyn NativeEffectFactory,
    request: PrepareEffectRequest<'_>,
) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
    factory.prepare(request).map(|prepared| prepared.processor)
}

fn continuation_probe(
    factory: &dyn NativeEffectFactory,
    request: PrepareEffectRequest<'_>,
    expected: PreparedEffectMetadata,
    tier: &mut EffectConformanceTierReport,
) -> bool {
    let (Ok(mut source), Ok(mut restored)) = (
        prepare_processor(factory, request),
        prepare_processor(factory, request),
    ) else {
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
