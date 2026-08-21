//! Launch hysteretic peak gate/downward-expander.
//!
//! The prepared scalar instance owns all delay, detector and gain state. Its shape is fixed at
//! preparation; the render method performs no allocation and does not consult runtime features.
#![allow(missing_docs)]

use miso_engine_core::{GateGainKernelError, PreparedGateGainKernelV1};
use miso_engine_effect_contract::{
    AutomationRate, AutomationSpanKind, BankProcessReport, BankWidth, EffectBankProcessBlock,
    EffectDescriptorV1, EffectPrepareError, EffectProcessBlock, EffectQuality,
    InitialParameterValue, LatencySamples, LinkMode, LinkModeSet, NativeEffectFactory,
    ParameterChannel, ParameterChannelPolicy, ParameterDescriptorV1, ParameterDomain, ParameterId,
    ParameterMapping, ParameterUnit, PortDescriptorV1, PortId, PortLayout, PortRole,
    PrepareEffectBankRequest, PrepareEffectRequest, PreparedAutomationSpan, PreparedBankMetadata,
    PreparedEffectMetadata, PreparedNativeEffect, PreparedNativeEffectBank, PreparedSidechainPort,
    ProcessReport, ResetKind, SmoothingRule, StatePayloadError, StatePayloadInput,
    StatePayloadOutput, StatePayloadSizes, TailSamples, expected_prepared_metadata,
    sanitize_sample,
};

const PARAMETER_COUNT: usize = 8;
const RAMP_COUNT: usize = 4;
const STATE_HEADER_WORDS: usize = 20;
const PHASE_CLOSED: u32 = 0;
const PHASE_OPEN: u32 = 1;

const fn effect_id(value: &'static str) -> miso_engine_effect_contract::EffectId {
    match miso_engine_effect_contract::EffectId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("valid static effect identifier"),
    }
}

const fn port_id(value: &'static str) -> PortId {
    match PortId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("valid static port identifier"),
    }
}

const fn parameter_id(value: u32) -> ParameterId {
    match ParameterId::new(value) {
        Some(value) => value,
        None => panic!("nonzero static parameter identifier"),
    }
}

#[allow(clippy::too_many_arguments)]
const fn parameter(
    id: u32,
    display_name: &'static str,
    display_unit: &'static str,
    unit: ParameterUnit,
    minimum: f32,
    maximum: f32,
    default_value: f32,
    mapping: ParameterMapping,
    automation_rate: AutomationRate,
    smoothing: SmoothingRule,
    smoothing_samples: u32,
) -> ParameterDescriptorV1 {
    ParameterDescriptorV1 {
        id: parameter_id(id),
        display_name,
        display_unit,
        unit,
        domain: ParameterDomain::Continuous,
        minimum: Some(minimum),
        maximum: Some(maximum),
        default_value,
        mapping,
        automation_rate,
        channel_policy: ParameterChannelPolicy::PerLane,
        smoothing,
        smoothing_samples,
        readable: true,
        automatable: !matches!(automation_rate, AutomationRate::None),
        enum_choices: &[],
    }
}

/// Frozen V1 gate/expander parameters. Descriptor position and stable numeric ID agree.
pub const GATE_EXPANDER_PARAMETERS_V1: [ParameterDescriptorV1; PARAMETER_COUNT] = [
    parameter(
        1,
        "threshold",
        "dB",
        ParameterUnit::Db,
        -80.0,
        0.0,
        -40.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        2,
        "ratio",
        "ratio",
        ParameterUnit::Ratio,
        1.0,
        20.0,
        4.0,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        3,
        "range",
        "dB",
        ParameterUnit::Db,
        0.0,
        96.0,
        80.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        4,
        "hysteresis",
        "dB",
        ParameterUnit::Db,
        0.0,
        24.0,
        6.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        5,
        "attack",
        "ms",
        ParameterUnit::Milliseconds,
        0.1,
        50.0,
        1.0,
        ParameterMapping::Logarithmic,
        AutomationRate::None,
        SmoothingRule::None,
        0,
    ),
    parameter(
        6,
        "hold",
        "ms",
        ParameterUnit::Milliseconds,
        0.0,
        1000.0,
        100.0,
        ParameterMapping::Linear,
        AutomationRate::None,
        SmoothingRule::None,
        0,
    ),
    parameter(
        7,
        "release",
        "ms",
        ParameterUnit::Milliseconds,
        5.0,
        2000.0,
        100.0,
        ParameterMapping::Logarithmic,
        AutomationRate::None,
        SmoothingRule::None,
        0,
    ),
    parameter(
        8,
        "lookahead",
        "ms",
        ParameterUnit::Milliseconds,
        0.0,
        10.0,
        2.0,
        ParameterMapping::Linear,
        AutomationRate::None,
        SmoothingRule::None,
        0,
    ),
];

const PORTS: [PortDescriptorV1; 3] = [
    PortDescriptorV1 {
        id: port_id("main-in"),
        role: PortRole::MainInput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptorV1 {
        id: port_id("main-out"),
        role: PortRole::MainOutput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptorV1 {
        id: port_id("sidechain-in"),
        role: PortRole::SidechainInput,
        required: false,
        layout: PortLayout::DualMonoPlanar,
    },
];

const fn quality(
    sample_rate: u32,
    latency: u64,
) -> miso_engine_effect_contract::QualityDescriptorV1 {
    let ring_length = latency as u32 + 1;
    let per_lane = (STATE_HEADER_WORDS as u32 + 2 * ring_length) * 4;
    miso_engine_effect_contract::QualityDescriptorV1 {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(latency),
        tail: TailSamples::Finite(0),
        maximum_state: StatePayloadSizes {
            common_bytes: 0,
            left_bytes: per_lane,
            right_bytes: per_lane,
        },
        scratch_fixed_bytes: 64,
        scratch_bytes_per_frame: 0,
    }
}

const QUALITIES: [miso_engine_effect_contract::QualityDescriptorV1; 4] = [
    quality(44_100, 441),
    quality(48_000, 480),
    quality(88_200, 882),
    quality(96_000, 960),
];

/// Immutable descriptor for the launch gate/expander contract.
pub const GATE_EXPANDER_DESCRIPTOR_V1: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("miso.gate-expander"),
    display_name: "Gate / Expander",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::ALL,
    parameters: &GATE_EXPANDER_PARAMETERS_V1,
    ports: &PORTS,
    qualities: &QUALITIES,
};

/// Factory for the fixed-latency scalar launch implementation.
#[derive(Clone, Copy, Debug, Default)]
pub struct GateExpanderFactory;

#[derive(Clone, Copy, Debug)]
struct Ramp {
    current: f32,
    target: f32,
    remaining: u32,
}

impl Ramp {
    const fn fixed(value: f32) -> Self {
        Self {
            current: value,
            target: value,
            remaining: 0,
        }
    }

    fn advance(&mut self) {
        if self.remaining != 0 {
            self.current += (self.target - self.current) / self.remaining as f32;
            self.remaining -= 1;
            if self.remaining == 0 {
                self.current = self.target;
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Phase {
    Closed,
    Open,
}

impl Phase {
    const fn word(self) -> u32 {
        match self {
            Self::Closed => PHASE_CLOSED,
            Self::Open => PHASE_OPEN,
        }
    }

    const fn from_word(value: u32) -> Option<Self> {
        match value {
            PHASE_CLOSED => Some(Self::Closed),
            PHASE_OPEN => Some(Self::Open),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Lane {
    cursor: u32,
    lookahead_ms: f32,
    gain_reduction_db: f32,
    phase: Phase,
    hold_remaining: u32,
    attack_ms: f32,
    hold_ms: f32,
    release_ms: f32,
    detector_delay: usize,
    hold_samples: u32,
    attack_coefficient: f32,
    release_coefficient: f32,
    ramps: [Ramp; RAMP_COUNT],
    main_ring: Box<[f32]>,
    detector_ring: Box<[f32]>,
}

impl Lane {
    fn new(
        defaults: &[f32; PARAMETER_COUNT],
        ring_length: usize,
        sample_rate: u32,
    ) -> Option<Self> {
        let mut lane = Self {
            cursor: 0,
            lookahead_ms: defaults[7],
            gain_reduction_db: 0.0,
            phase: Phase::Open,
            hold_remaining: 0,
            attack_ms: defaults[4],
            hold_ms: defaults[5],
            release_ms: defaults[6],
            detector_delay: 0,
            hold_samples: 0,
            attack_coefficient: 0.0,
            release_coefficient: 0.0,
            ramps: core::array::from_fn(|index| Ramp::fixed(defaults[index])),
            main_ring: vec![0.0; ring_length].into_boxed_slice(),
            detector_ring: vec![0.0; ring_length].into_boxed_slice(),
        };
        lane.rederive(sample_rate)?;
        lane.hold_remaining = lane.hold_samples;
        Some(lane)
    }

    fn rederive(&mut self, sample_rate: u32) -> Option<()> {
        let latency = self.main_ring.len().checked_sub(1)?;
        let lookahead = rounded_samples(self.lookahead_ms, sample_rate)?;
        self.detector_delay = latency.checked_sub(lookahead.min(latency))?;
        self.hold_samples = u32::try_from(rounded_samples(self.hold_ms, sample_rate)?).ok()?;
        self.attack_coefficient = time_coefficient(self.attack_ms, sample_rate)?;
        self.release_coefficient = time_coefficient(self.release_ms, sample_rate)?;
        Some(())
    }

    fn full_reset(&mut self, defaults: &[f32; PARAMETER_COUNT], sample_rate: u32) {
        self.cursor = 0;
        self.lookahead_ms = defaults[7];
        self.gain_reduction_db = 0.0;
        self.phase = Phase::Open;
        self.attack_ms = defaults[4];
        self.hold_ms = defaults[5];
        self.release_ms = defaults[6];
        self.ramps = core::array::from_fn(|index| Ramp::fixed(defaults[index]));
        self.main_ring.fill(0.0);
        self.detector_ring.fill(0.0);
        let _ = self.rederive(sample_rate);
        self.hold_remaining = self.hold_samples;
    }

    fn discontinuity_reset(&mut self) {
        self.cursor = 0;
        self.gain_reduction_db = 0.0;
        self.phase = Phase::Open;
        self.hold_remaining = self.hold_samples;
        self.ramps.iter_mut().for_each(|ramp| {
            ramp.current = ramp.target;
            ramp.remaining = 0;
        });
        self.main_ring.fill(0.0);
        self.detector_ring.fill(0.0);
    }
}

/// A fixed-shape allocation-free scalar gate/expander instance.
#[derive(Debug)]
pub struct PreparedGateExpander {
    metadata: PreparedEffectMetadata,
    left_defaults: [f32; PARAMETER_COUNT],
    right_defaults: [f32; PARAMETER_COUNT],
    left: Lane,
    right: Lane,
}

/// A fixed-width homogeneous unconnected-sidechain gate/expander cohort.
///
/// The `W`-specialized arrays retain exactly one independent scalar lane per track and per audio
/// channel. The core token only performs the frozen final multiply/identity selection.
struct PreparedGateExpanderBank<const W: usize> {
    metadata: PreparedBankMetadata,
    effect_metadata: PreparedEffectMetadata,
    kernel: PreparedGateGainKernelV1,
    left_defaults: [[f32; PARAMETER_COUNT]; W],
    right_defaults: [[f32; PARAMETER_COUNT]; W],
    left: [Lane; W],
    right: [Lane; W],
}

impl NativeEffectFactory for GateExpanderFactory {
    fn descriptor(&self) -> &'static EffectDescriptorV1 {
        &GATE_EXPANDER_DESCRIPTOR_V1
    }

    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        let metadata = expected_prepared_metadata(self.descriptor(), request)?;
        let (left_defaults, right_defaults) = initial_defaults(request.initial_values)?;
        let ring_length = usize::try_from(metadata.latency.0)
            .ok()
            .and_then(|latency| latency.checked_add(1))
            .ok_or(EffectPrepareError {
                code: "effect.resource.limit",
            })?;
        let left = Lane::new(&left_defaults, ring_length, metadata.sample_rate).ok_or(
            EffectPrepareError {
                code: "effect.parameter.initial",
            },
        )?;
        let right = Lane::new(&right_defaults, ring_length, metadata.sample_rate).ok_or(
            EffectPrepareError {
                code: "effect.parameter.initial",
            },
        )?;
        Ok(Box::new(PreparedGateExpander {
            metadata,
            left_defaults,
            right_defaults,
            left,
            right,
        }))
    }

    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
        if !request.has_matching_backend_width()
            || request.requests.len() != request.width.lanes() as usize
        {
            return Err(EffectPrepareError {
                code: "effect.bank.requests",
            });
        }
        match request.width {
            BankWidth::Four => prepare_homogeneous_bank::<4>(self, request),
            BankWidth::Eight => prepare_homogeneous_bank::<8>(self, request),
        }
    }
}

fn prepare_homogeneous_bank<const W: usize>(
    factory: &GateExpanderFactory,
    request: PrepareEffectBankRequest<'_>,
) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
    let first_request = request
        .requests
        .first()
        .copied()
        .ok_or(EffectPrepareError {
            code: "effect.bank.requests",
        })?;
    let metadata = expected_prepared_metadata(factory.descriptor(), first_request)?;
    let (first_left_defaults, first_right_defaults) =
        initial_defaults(first_request.initial_values)?;
    let mut left_defaults = [first_left_defaults; W];
    let mut right_defaults = [first_right_defaults; W];
    let mut same_program = true;
    for (track, item) in request.requests.iter().copied().enumerate() {
        let candidate = expected_prepared_metadata(factory.descriptor(), item)?;
        if candidate.program_key() != metadata.program_key() {
            same_program = false;
        }
        let (left, right) = initial_defaults(item.initial_values)?;
        left_defaults[track] = left;
        right_defaults[track] = right;
    }
    if !same_program
        || !matches!(
            metadata.ports.sidechain,
            PreparedSidechainPort::Unconnected {
                id,
                required: false,
            } if id == port_id("sidechain-in")
        )
    {
        return Ok(None);
    }
    let kernel = match PreparedGateGainKernelV1::try_new(request.backend) {
        Ok(kernel) => kernel,
        Err(GateGainKernelError::BackendUnavailable) => return Ok(None),
        Err(_) => {
            return Err(EffectPrepareError {
                code: "effect.bank.backend",
            });
        }
    };
    let ring_length = usize::try_from(metadata.latency.0)
        .ok()
        .and_then(|latency| latency.checked_add(1))
        .ok_or(EffectPrepareError {
            code: "effect.resource.limit",
        })?;
    // `initial_defaults` accepted every static parameter above, and the descriptor fixes the
    // supported rate/ring pair. `Lane::new` can therefore not fail here; fixed-width construction
    // avoids an allocation and preserves width-specialized retained state.
    let left = core::array::from_fn(|track| {
        Lane::new(&left_defaults[track], ring_length, metadata.sample_rate)
            .expect("validated gate preparation values")
    });
    let right = core::array::from_fn(|track| {
        Lane::new(&right_defaults[track], ring_length, metadata.sample_rate)
            .expect("validated gate preparation values")
    });
    Ok(Some(Box::new(PreparedGateExpanderBank::<W> {
        metadata: PreparedBankMetadata {
            width: request.width,
            program_key: metadata.program_key(),
        },
        effect_metadata: metadata,
        kernel,
        left_defaults,
        right_defaults,
        left,
        right,
    })))
}

impl PreparedNativeEffect for PreparedGateExpander {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.metadata
    }

    fn reset(&mut self, kind: ResetKind) {
        match kind {
            ResetKind::FullToDefaults => {
                self.left
                    .full_reset(&self.left_defaults, self.metadata.sample_rate);
                self.right
                    .full_reset(&self.right_defaults, self.metadata.sample_rate);
            }
            ResetKind::DiscontinuityKeepParameters => {
                self.left.discontinuity_reset();
                self.right.discontinuity_reset();
            }
        }
    }

    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        let mut report = ProcessReport::default();
        apply_automation(
            block.automation,
            self.metadata,
            block.first_sample,
            &mut self.left,
            &mut self.right,
            &mut report,
        );
        let connected = matches!(
            self.metadata.ports.sidechain,
            miso_engine_effect_contract::PreparedSidechainPort::Connected { .. }
        );
        for index in 0..block.frames() {
            self.left.ramps.iter_mut().for_each(Ramp::advance);
            self.right.ramps.iter_mut().for_each(Ramp::advance);
            let main_left = sanitize(block.left[index], &mut report.sanitized_main_samples);
            let main_right = sanitize(block.right[index], &mut report.sanitized_main_samples);
            let (source_left, source_right) = if connected {
                let (left, right) = block
                    .sidechain
                    .map_or((0.0, 0.0), |(left, right)| (left[index], right[index]));
                (
                    sanitize(left, &mut report.sanitized_sidechain_samples),
                    sanitize(right, &mut report.sanitized_sidechain_samples),
                )
            } else {
                (main_left, main_right)
            };
            let (detector_left, detector_right) =
                linked_levels(self.metadata.link_mode, source_left, source_right);
            block.left[index] = process_lane(
                main_left,
                detector_left,
                &mut self.left,
                self.metadata.bypass,
                &mut report.recovered_left_samples,
            );
            block.right[index] = process_lane(
                main_right,
                detector_right,
                &mut self.right,
                self.metadata.bypass,
                &mut report.recovered_right_samples,
            );
        }
        report
    }

    fn snapshot_state_payload(
        &self,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        validate_state_lengths(
            output.common.len(),
            output.left.len(),
            output.right.len(),
            self.metadata.state_sizes,
        )?;
        write_lane(output.left, &self.left);
        write_lane(output.right, &self.right);
        Ok(())
    }

    fn restore_state_payload(
        &mut self,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        if state_layout_version != 1 {
            return Err(state_error("effect.state.version"));
        }
        validate_state_lengths(
            input.common.len(),
            input.left.len(),
            input.right.len(),
            self.metadata.state_sizes,
        )?;
        let ring_length = self.left.main_ring.len();
        let left = read_lane(input.left, ring_length, self.metadata.sample_rate)?;
        let right = read_lane(input.right, ring_length, self.metadata.sample_rate)?;
        self.left = left;
        self.right = right;
        Ok(())
    }
}

impl<const W: usize> PreparedNativeEffectBank for PreparedGateExpanderBank<W> {
    fn metadata(&self) -> PreparedBankMetadata {
        self.metadata.clone()
    }

    fn reset(&mut self, kind: ResetKind) {
        for track in 0..W {
            match kind {
                ResetKind::FullToDefaults => {
                    self.left[track]
                        .full_reset(&self.left_defaults[track], self.effect_metadata.sample_rate);
                    self.right[track].full_reset(
                        &self.right_defaults[track],
                        self.effect_metadata.sample_rate,
                    );
                }
                ResetKind::DiscontinuityKeepParameters => {
                    self.left[track].discontinuity_reset();
                    self.right[track].discontinuity_reset();
                }
            }
        }
    }

    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        let mut report = BankProcessReport::empty(self.metadata.width);
        if block.width != self.metadata.width
            || block.frames > self.effect_metadata.quantum
            || block.sidechain.is_some()
            || W != self.metadata.width.lanes() as usize
        {
            return report;
        }
        for track in 0..W {
            let start = block.automation_offsets[track] as usize;
            let end = block.automation_offsets[track + 1] as usize;
            apply_automation(
                &block.automation[start..end],
                self.effect_metadata,
                block.first_sample,
                &mut self.left[track],
                &mut self.right[track],
                &mut report.reports[track],
            );
        }
        for frame in 0..block.frames as usize {
            let start = frame * W;
            process_bank_frame(
                &mut self.left,
                &mut self.right,
                self.kernel,
                self.effect_metadata,
                &mut report,
                &mut block.left[start..start + W],
                &mut block.right[start..start + W],
            );
        }
        report
    }

    fn snapshot_track_state_payload(
        &self,
        track_index: u32,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        let track = checked_track(track_index, W)?;
        validate_state_lengths(
            output.common.len(),
            output.left.len(),
            output.right.len(),
            self.effect_metadata.state_sizes,
        )?;
        write_lane(output.left, &self.left[track]);
        write_lane(output.right, &self.right[track]);
        Ok(())
    }

    fn restore_track_state_payload(
        &mut self,
        track_index: u32,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        let track = checked_track(track_index, W)?;
        if state_layout_version != 1 {
            return Err(state_error("effect.state.version"));
        }
        validate_state_lengths(
            input.common.len(),
            input.left.len(),
            input.right.len(),
            self.effect_metadata.state_sizes,
        )?;
        let ring_length = self.left[track].main_ring.len();
        let left = read_lane(input.left, ring_length, self.effect_metadata.sample_rate)?;
        let right = read_lane(input.right, ring_length, self.effect_metadata.sample_rate)?;
        self.left[track] = left;
        self.right[track] = right;
        Ok(())
    }
}

fn checked_track(track_index: u32, width: usize) -> Result<usize, StatePayloadError> {
    let track = usize::try_from(track_index).map_err(|_| state_error("effect.state.track"))?;
    if track >= width {
        return Err(state_error("effect.state.track"));
    }
    Ok(track)
}

#[allow(clippy::too_many_arguments)]
fn process_bank_frame<const W: usize>(
    left_lanes: &mut [Lane; W],
    right_lanes: &mut [Lane; W],
    kernel: PreparedGateGainKernelV1,
    metadata: PreparedEffectMetadata,
    report: &mut BankProcessReport,
    left_samples: &mut [f32],
    right_samples: &mut [f32],
) {
    let mut left_gains = [1.0; W];
    let mut right_gains = [1.0; W];
    let mut left_identity = [0_u32; W];
    let mut right_identity = [0_u32; W];
    let mut left_delayed = [0.0; W];
    let mut right_delayed = [0.0; W];
    for track in 0..W {
        left_lanes[track].ramps.iter_mut().for_each(Ramp::advance);
        right_lanes[track].ramps.iter_mut().for_each(Ramp::advance);
        let main_left = sanitize(
            left_samples[track],
            &mut report.reports[track].sanitized_main_samples,
        );
        let main_right = sanitize(
            right_samples[track],
            &mut report.reports[track].sanitized_main_samples,
        );
        let (detector_left, detector_right) =
            linked_levels(metadata.link_mode, main_left, main_right);
        let left = prepare_lane_gain(
            main_left,
            detector_left,
            &mut left_lanes[track],
            metadata.bypass,
            &mut report.reports[track].recovered_left_samples,
        );
        let right = prepare_lane_gain(
            main_right,
            detector_right,
            &mut right_lanes[track],
            metadata.bypass,
            &mut report.reports[track].recovered_right_samples,
        );
        left_samples[track] = left.delayed;
        right_samples[track] = right.delayed;
        left_gains[track] = left.gain;
        right_gains[track] = right.gain;
        left_identity[track] = u32::from(left.identity).wrapping_neg();
        right_identity[track] = u32::from(right.identity).wrapping_neg();
        left_delayed[track] = left.delayed;
        right_delayed[track] = right.delayed;
    }
    if kernel
        .process_gain(left_samples, &left_gains, &left_identity)
        .is_err()
        || kernel
            .process_gain(right_samples, &right_gains, &right_identity)
            .is_err()
    {
        left_samples.copy_from_slice(&left_delayed);
        right_samples.copy_from_slice(&right_delayed);
        return;
    }
    for track in 0..W {
        left_samples[track] = finish_bank_output(
            left_samples[track],
            left_delayed[track],
            &mut left_lanes[track],
            &mut report.reports[track].recovered_left_samples,
        );
        right_samples[track] = finish_bank_output(
            right_samples[track],
            right_delayed[track],
            &mut right_lanes[track],
            &mut report.reports[track].recovered_right_samples,
        );
    }
}

fn finish_bank_output(value: f32, delayed: f32, lane: &mut Lane, recovered: &mut u64) -> f32 {
    match finite_or_zero(value) {
        Some(value) => value,
        None => recover(lane, delayed, recovered),
    }
}

fn initial_defaults(
    values: &[InitialParameterValue],
) -> Result<([f32; PARAMETER_COUNT], [f32; PARAMETER_COUNT]), EffectPrepareError> {
    if values.len() != PARAMETER_COUNT * 2 {
        return Err(EffectPrepareError {
            code: "effect.parameter.initial",
        });
    }
    let mut left = [0.0; PARAMETER_COUNT];
    let mut right = [0.0; PARAMETER_COUNT];
    for (index, parameter) in GATE_EXPANDER_PARAMETERS_V1.iter().enumerate() {
        let left_value = values[index * 2];
        let right_value = values[index * 2 + 1];
        if left_value.parameter_index != index as u32
            || right_value.parameter_index != index as u32
            || left_value.channel != ParameterChannel::Left
            || right_value.channel != ParameterChannel::Right
            || !parameter_value_valid(parameter, left_value.value)
            || !parameter_value_valid(parameter, right_value.value)
            || negative_zero(left_value.value)
            || negative_zero(right_value.value)
        {
            return Err(EffectPrepareError {
                code: "effect.parameter.initial",
            });
        }
        left[index] = normalize_zero(left_value.value);
        right[index] = normalize_zero(right_value.value);
    }
    Ok((left, right))
}

#[derive(Clone, Copy)]
struct GainFrame {
    delayed: f32,
    gain: f32,
    identity: bool,
}

fn process_lane(
    main: f32,
    detector: f32,
    lane: &mut Lane,
    bypass: bool,
    recovered: &mut u64,
) -> f32 {
    let frame = prepare_lane_gain(main, detector, lane, bypass, recovered);
    if frame.identity {
        return frame.delayed;
    }
    match finite_or_zero(frame.delayed * frame.gain) {
        Some(output) => output,
        None => recover(lane, frame.delayed, recovered),
    }
}

fn prepare_lane_gain(
    main: f32,
    detector: f32,
    lane: &mut Lane,
    bypass: bool,
    recovered: &mut u64,
) -> GainFrame {
    let ring_length = lane.main_ring.len();
    let cursor = lane.cursor as usize;
    lane.main_ring[cursor] = main;
    lane.detector_ring[cursor] = detector;
    let delayed = lane.main_ring[(cursor + 1) % ring_length];
    let level = lane.detector_ring[(cursor + ring_length - lane.detector_delay) % ring_length];
    lane.cursor = ((cursor + 1) % ring_length) as u32;

    let Some(level_db) = finite_or_zero((20.0_f32 * level.max(1.0e-8).log10()).clamp(-160.0, 24.0))
    else {
        return recovered_gain_frame(lane, delayed, recovered);
    };
    transition(lane, level_db);
    let target = match lane.phase {
        Phase::Open => 0.0,
        Phase::Closed => ((lane.ramps[1].current - 1.0) * (level_db - lane.ramps[0].current))
            .clamp(-lane.ramps[2].current, 0.0),
    };
    let coefficient = if target > lane.gain_reduction_db {
        lane.attack_coefficient
    } else {
        lane.release_coefficient
    };
    let Some(gain_reduction_db) =
        finite_or_zero(coefficient * lane.gain_reduction_db + (1.0 - coefficient) * target)
    else {
        return recovered_gain_frame(lane, delayed, recovered);
    };
    lane.gain_reduction_db = gain_reduction_db;
    let Some(gain) = finite_or_zero(10.0_f32.powf(0.05 * gain_reduction_db)) else {
        return recovered_gain_frame(lane, delayed, recovered);
    };
    GainFrame {
        delayed,
        gain,
        identity: bypass || gain_reduction_db == 0.0,
    }
}

fn recovered_gain_frame(lane: &mut Lane, delayed: f32, recovered: &mut u64) -> GainFrame {
    let _ = recover(lane, delayed, recovered);
    GainFrame {
        delayed,
        gain: 1.0,
        identity: true,
    }
}

fn transition(lane: &mut Lane, level_db: f32) {
    let threshold = lane.ramps[0].current;
    let hysteresis = lane.ramps[3].current;
    match lane.phase {
        Phase::Closed if level_db >= threshold => {
            lane.phase = Phase::Open;
            lane.hold_remaining = lane.hold_samples;
        }
        Phase::Closed => {}
        Phase::Open if level_db >= threshold - hysteresis => {
            lane.hold_remaining = lane.hold_samples
        }
        Phase::Open if lane.hold_remaining != 0 => lane.hold_remaining -= 1,
        Phase::Open => lane.phase = Phase::Closed,
    }
}

fn apply_automation(
    spans: &[PreparedAutomationSpan],
    metadata: PreparedEffectMetadata,
    first_sample: u64,
    left: &mut Lane,
    right: &mut Lane,
    report: &mut ProcessReport,
) {
    let mut pending = [[None; RAMP_COUNT]; 2];
    let mut last_order = None;
    for (span_index, span) in spans.iter().enumerate() {
        let lane_index = match span.channel {
            ParameterChannel::Left => 0,
            ParameterChannel::Right => 1,
            ParameterChannel::Both => {
                report.invalid_spans = report.invalid_spans.saturating_add(1);
                continue;
            }
        };
        let parameter_index = span.parameter_index as usize;
        let Some(order) = span
            .parameter_index
            .checked_mul(2)
            .and_then(|value| value.checked_add(lane_index as u32))
        else {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        };
        let valid = span_index < metadata.automation_capacity as usize
            && parameter_index < RAMP_COUNT
            && span.kind == AutomationSpanKind::Point
            && span.start_sample == first_sample
            && span.end_sample == first_sample
            && span.start_value.to_bits() == span.end_value.to_bits()
            && parameter_value_valid(
                &GATE_EXPANDER_PARAMETERS_V1[parameter_index],
                span.start_value,
            )
            && last_order.is_none_or(|previous| order > previous)
            && pending[lane_index][parameter_index].is_none();
        if !valid {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        }
        last_order = Some(order);
        pending[lane_index][parameter_index] = Some(normalize_zero(span.start_value));
    }
    for (index, (left_ramp, right_ramp)) in left
        .ramps
        .iter_mut()
        .zip(right.ramps.iter_mut())
        .enumerate()
    {
        if let Some(value) = pending[0][index] {
            left_ramp.target = value;
            left_ramp.remaining = 64;
        }
        if let Some(value) = pending[1][index] {
            right_ramp.target = value;
            right_ramp.remaining = 64;
        }
    }
}

fn linked_levels(link_mode: LinkMode, left: f32, right: f32) -> (f32, f32) {
    let left = left.abs();
    let right = right.abs();
    match link_mode {
        LinkMode::DualMono => (left, right),
        LinkMode::Maximum => {
            let value = left.max(right);
            (value, value)
        }
        LinkMode::Average => {
            let value = 0.5 * left + 0.5 * right;
            (value, value)
        }
    }
}

fn sanitize(value: f32, counter: &mut u64) -> f32 {
    match sanitize_sample(value) {
        Some(value) => value,
        None => {
            *counter = counter.saturating_add(1);
            0.0
        }
    }
}

fn finite_or_zero(value: f32) -> Option<f32> {
    if !value.is_finite() {
        None
    } else if value.is_subnormal() {
        Some(0.0)
    } else {
        Some(value)
    }
}

fn recover(lane: &mut Lane, delayed: f32, recovered: &mut u64) -> f32 {
    lane.phase = Phase::Open;
    lane.hold_remaining = lane.hold_samples;
    lane.gain_reduction_db = 0.0;
    *recovered = recovered.saturating_add(1);
    delayed
}

fn rounded_samples(milliseconds: f32, sample_rate: u32) -> Option<usize> {
    let samples = (milliseconds as f64 * sample_rate as f64 / 1000.0 + 0.5).floor();
    if !samples.is_finite() || samples < 0.0 {
        return None;
    }
    usize::try_from(samples as u64).ok()
}

fn time_coefficient(milliseconds: f32, sample_rate: u32) -> Option<f32> {
    finite_or_zero((-1.0_f32 / (0.001 * milliseconds * sample_rate as f32)).exp())
}

fn parameter_value_valid(parameter: &ParameterDescriptorV1, value: f32) -> bool {
    value.is_finite()
        && parameter
            .minimum
            .zip(parameter.maximum)
            .is_some_and(|(minimum, maximum)| value >= minimum && value <= maximum)
}

fn negative_zero(value: f32) -> bool {
    value.to_bits() == (-0.0_f32).to_bits()
}
fn normalize_zero(value: f32) -> f32 {
    if value == 0.0 { 0.0 } else { value }
}
fn normal_or_zero(value: f32) -> bool {
    value.is_finite() && (value == 0.0 || value.is_normal())
}

fn validate_state_lengths(
    common_bytes: usize,
    left_bytes: usize,
    right_bytes: usize,
    sizes: StatePayloadSizes,
) -> Result<(), StatePayloadError> {
    if common_bytes != sizes.common_bytes as usize
        || left_bytes != sizes.left_bytes as usize
        || right_bytes != sizes.right_bytes as usize
    {
        return Err(state_error("effect.state.length"));
    }
    Ok(())
}

fn write_lane(bytes: &mut [u8], lane: &Lane) {
    write_u32(bytes, 0, lane.cursor);
    write_f32(bytes, 1, lane.lookahead_ms);
    write_f32(bytes, 2, lane.gain_reduction_db);
    write_u32(bytes, 3, lane.phase.word());
    write_u32(bytes, 4, lane.hold_remaining);
    write_f32(bytes, 5, lane.attack_ms);
    write_f32(bytes, 6, lane.hold_ms);
    write_f32(bytes, 7, lane.release_ms);
    for (index, ramp) in lane.ramps.iter().enumerate() {
        let word = 8 + index * 3;
        write_f32(bytes, word, ramp.current);
        write_f32(bytes, word + 1, ramp.target);
        write_u32(bytes, word + 2, ramp.remaining);
    }
    for (index, value) in lane.main_ring.iter().enumerate() {
        write_f32(bytes, STATE_HEADER_WORDS + index, *value);
    }
    for (index, value) in lane.detector_ring.iter().enumerate() {
        write_f32(
            bytes,
            STATE_HEADER_WORDS + lane.main_ring.len() + index,
            *value,
        );
    }
}

fn read_lane(
    bytes: &[u8],
    ring_length: usize,
    sample_rate: u32,
) -> Result<Lane, StatePayloadError> {
    let expected_length = (STATE_HEADER_WORDS + 2 * ring_length)
        .checked_mul(4)
        .ok_or(state_error("effect.state.length"))?;
    if bytes.len() != expected_length {
        return Err(state_error("effect.state.length"));
    }
    let cursor = read_u32(bytes, 0);
    if cursor as usize >= ring_length {
        return Err(state_error("effect.state.cursor"));
    }
    let lookahead_ms = read_f32(bytes, 1);
    let gain_reduction_db = read_f32(bytes, 2);
    let phase = Phase::from_word(read_u32(bytes, 3)).ok_or(state_error("effect.state.phase"))?;
    let hold_remaining = read_u32(bytes, 4);
    let attack_ms = read_f32(bytes, 5);
    let hold_ms = read_f32(bytes, 6);
    let release_ms = read_f32(bytes, 7);
    if !parameter_state_valid(7, lookahead_ms)
        || !parameter_state_valid(4, attack_ms)
        || !parameter_state_valid(5, hold_ms)
        || !parameter_state_valid(6, release_ms)
    {
        return Err(state_error("effect.state.parameter"));
    }
    if !normal_or_zero(gain_reduction_db) || !(-96.0..=0.0).contains(&gain_reduction_db) {
        return Err(state_error("effect.state.gain"));
    }
    let mut defaults = [0.0; PARAMETER_COUNT];
    defaults[4] = attack_ms;
    defaults[5] = hold_ms;
    defaults[6] = release_ms;
    defaults[7] = lookahead_ms;
    let mut ramps = [Ramp::fixed(0.0); RAMP_COUNT];
    for (index, ramp) in ramps.iter_mut().enumerate() {
        let word = 8 + index * 3;
        let current = read_f32(bytes, word);
        let target = read_f32(bytes, word + 1);
        let remaining = read_u32(bytes, word + 2);
        if !parameter_state_valid(index, current)
            || !parameter_state_valid(index, target)
            || remaining > 64
        {
            return Err(state_error("effect.state.parameter"));
        }
        defaults[index] = current;
        *ramp = Ramp {
            current,
            target,
            remaining,
        };
    }
    let mut lane = Lane::new(&defaults, ring_length, sample_rate)
        .ok_or(state_error("effect.state.parameter"))?;
    if hold_remaining > lane.hold_samples {
        return Err(state_error("effect.state.hold"));
    }
    lane.cursor = cursor;
    lane.gain_reduction_db = normalize_zero(gain_reduction_db);
    lane.phase = phase;
    lane.hold_remaining = hold_remaining;
    lane.ramps = ramps;
    for (index, value) in lane.main_ring.iter_mut().enumerate() {
        *value = read_f32(bytes, STATE_HEADER_WORDS + index);
        if !normal_or_zero(*value) {
            return Err(state_error("effect.state.ring"));
        }
    }
    for (index, value) in lane.detector_ring.iter_mut().enumerate() {
        *value = read_f32(bytes, STATE_HEADER_WORDS + ring_length + index);
        if !normal_or_zero(*value) {
            return Err(state_error("effect.state.ring"));
        }
    }
    Ok(lane)
}

fn parameter_state_valid(index: usize, value: f32) -> bool {
    !negative_zero(value) && parameter_value_valid(&GATE_EXPANDER_PARAMETERS_V1[index], value)
}
fn write_u32(bytes: &mut [u8], word: usize, value: u32) {
    let offset = word * 4;
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}
fn write_f32(bytes: &mut [u8], word: usize, value: f32) {
    write_u32(bytes, word, value.to_bits());
}
fn read_u32(bytes: &[u8], word: usize) -> u32 {
    let offset = word * 4;
    u32::from_le_bytes(
        bytes[offset..offset + 4]
            .try_into()
            .expect("state length was checked"),
    )
}
fn read_f32(bytes: &[u8], word: usize) -> f32 {
    f32::from_bits(read_u32(bytes, word))
}
const fn state_error(code: &'static str) -> StatePayloadError {
    StatePayloadError { code }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_dsp_reference::{
        ReferenceGateExpanderParameters, ReferenceGatePhase,
        reference_gate_expander_gain_reduction_db,
    };
    use miso_engine_effect_contract::{
        EffectProcessBlock, PrepareEffectLimits, PreparedPortsV1, PreparedSidechainPort,
        StatePayloadInput, StatePayloadOutput, validate_descriptor_v1,
    };

    fn initial_values() -> [InitialParameterValue; 16] {
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

    fn request<'a>(values: &'a [InitialParameterValue]) -> PrepareEffectRequest<'a> {
        PrepareEffectRequest {
            sample_rate: 48_000,
            quantum: 128,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: PreparedPortsV1 {
                sidechain: PreparedSidechainPort::Unconnected {
                    id: port_id("sidechain-in"),
                    required: false,
                },
            },
            initial_values: values,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: 7_856,
                maximum_scratch_bytes: 64,
                maximum_automation_spans_per_block: 16,
            },
        }
    }

    fn snapshot(effect: &dyn PreparedNativeEffect) -> (Vec<u8>, Vec<u8>) {
        let sizes = effect.metadata().state_sizes;
        let mut left = vec![0; sizes.left_bytes as usize];
        let mut right = vec![0; sizes.right_bytes as usize];
        effect
            .snapshot_state_payload(
                StatePayloadOutput::new(&mut [], &mut left, &mut right, sizes).expect("sizes"),
            )
            .expect("snapshot");
        (left, right)
    }

    #[test]
    fn descriptor_and_exact_resources_are_frozen() {
        validate_descriptor_v1(&GATE_EXPANDER_DESCRIPTOR_V1).expect("descriptor");
        assert_eq!(
            GATE_EXPANDER_DESCRIPTOR_V1.id.as_str(),
            "miso.gate-expander"
        );
        for (quality, (rate, latency, lane_bytes, total)) in QUALITIES.iter().zip([
            (44_100, 441, 3_616, 7_232),
            (48_000, 480, 3_928, 7_856),
            (88_200, 882, 7_144, 14_288),
            (96_000, 960, 7_768, 15_536),
        ]) {
            assert_eq!(quality.sample_rate, rate);
            assert_eq!(quality.latency, LatencySamples(latency));
            assert_eq!(quality.tail, TailSamples::Finite(0));
            assert_eq!(quality.maximum_state.left_bytes, lane_bytes);
            assert_eq!(quality.maximum_state.total(), Some(total));
            assert_eq!(quality.scratch_fixed_bytes, 64);
        }
    }

    #[test]
    fn preparation_caps_and_lookahead_rows_are_transactional() {
        let values = initial_values();
        let factory = GateExpanderFactory;
        let effect = factory.prepare(request(&values)).expect("prepare");
        assert_eq!(effect.metadata().latency, LatencySamples(480));
        let mut below = request(&values);
        below.limits.maximum_total_state_bytes -= 1;
        let error = match factory.prepare(below) {
            Ok(_) => panic!("one byte below state must reject"),
            Err(error) => error,
        };
        assert_eq!(error.code, "effect.resource.limit");
        let mut below_scratch = request(&values);
        below_scratch.limits.maximum_scratch_bytes -= 1;
        let error = match factory.prepare(below_scratch) {
            Ok(_) => panic!("one byte below scratch must reject"),
            Err(error) => error,
        };
        assert_eq!(error.code, "effect.resource.limit");
        for (lookahead, expected_delay) in [(0.0, 480), (5.0, 240), (10.0, 0)] {
            let mut defaults =
                core::array::from_fn(|index| GATE_EXPANDER_PARAMETERS_V1[index].default_value);
            defaults[7] = lookahead;
            let lane = Lane::new(&defaults, 481, 48_000).expect("lane");
            assert_eq!(lane.detector_delay, expected_delay);
        }
    }

    #[test]
    fn independent_curve_and_exact_hold_transitions_agree() {
        let parameters = ReferenceGateExpanderParameters {
            threshold_db: -40.0,
            ratio: 4.0,
            range_db: 12.0,
        };
        assert_eq!(
            reference_gate_expander_gain_reduction_db(
                -40.0,
                parameters,
                ReferenceGatePhase::Closed
            ),
            Ok(0.0)
        );
        assert_eq!(
            reference_gate_expander_gain_reduction_db(
                -80.0,
                parameters,
                ReferenceGatePhase::Closed
            ),
            Ok(-12.0)
        );
        assert_eq!(
            reference_gate_expander_gain_reduction_db(-80.0, parameters, ReferenceGatePhase::Open),
            Ok(0.0)
        );
        let mut defaults =
            core::array::from_fn(|index| GATE_EXPANDER_PARAMETERS_V1[index].default_value);
        defaults[5] = 0.0;
        let mut lane = Lane::new(&defaults, 481, 48_000).expect("lane");
        transition(&mut lane, -80.0);
        assert_eq!(lane.phase, Phase::Closed);
        transition(&mut lane, -40.0);
        assert_eq!(lane.phase, Phase::Open);
        transition(&mut lane, -46.0);
        assert_eq!(lane.phase, Phase::Open);
        transition(&mut lane, -46.1);
        assert_eq!(lane.phase, Phase::Closed);
    }

    #[test]
    fn fixed_latency_bypass_sidechain_and_automation_state_are_observable() {
        let mut values = initial_values();
        values[10].value = 0.0;
        values[11].value = 0.0;
        values[15].value = 10.0;
        let factory = GateExpanderFactory;
        let mut bypass_request = request(&values);
        bypass_request.bypass = true;
        let mut effect = factory.prepare(bypass_request).expect("prepare");
        let mut left = vec![0.0; 481];
        let mut right = vec![0.0; 481];
        left[0] = 1.0;
        for offset in (0..481).step_by(128) {
            let end = (offset + 128).min(481);
            effect.process(
                EffectProcessBlock::new(
                    &mut left[offset..end],
                    &mut right[offset..end],
                    None,
                    offset as u64,
                    &[],
                    128,
                )
                .expect("block"),
            );
        }
        assert_eq!(left[480].to_bits(), 1.0_f32.to_bits());
        let span = PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel: ParameterChannel::Left,
            parameter_index: 0,
            start_sample: 481,
            end_sample: 481,
            start_value: -20.0,
            end_value: -20.0,
        };
        let mut one_left = [0.0];
        let mut one_right = [0.0];
        effect.process(
            EffectProcessBlock::new(&mut one_left, &mut one_right, None, 481, &[span], 128)
                .expect("block"),
        );
        let (left_payload, _) = snapshot(effect.as_ref());
        assert_eq!(read_f32(&left_payload, 8), -39.6875);
        assert_eq!(read_u32(&left_payload, 10), 63);

        let mut connected_values = initial_values();
        connected_values[10].value = 0.0;
        connected_values[11].value = 0.0;
        let mut connected = request(&connected_values);
        connected.ports.sidechain = PreparedSidechainPort::Connected {
            id: port_id("sidechain-in"),
            required: false,
        };
        let mut connected_effect = factory.prepare(connected).expect("connected");
        let mut high_left = [1.0];
        let mut high_right = [1.0];
        let side_left = [0.0];
        let side_right = [0.0];
        connected_effect.process(
            EffectProcessBlock::new(
                &mut high_left,
                &mut high_right,
                Some((&side_left, &side_right)),
                0,
                &[],
                128,
            )
            .expect("block"),
        );
        let (connected_payload, _) = snapshot(connected_effect.as_ref());
        assert_eq!(read_u32(&connected_payload, 3), PHASE_CLOSED);
    }

    #[test]
    fn restore_is_all_or_none_and_sanitation_is_lane_local() {
        let values = initial_values();
        let factory = GateExpanderFactory;
        let mut effect = factory.prepare(request(&values)).expect("prepare");
        let (before_left, before_right) = snapshot(effect.as_ref());
        let mut malformed = before_right.clone();
        write_u32(&mut malformed, 3, 99);
        let sizes = effect.metadata().state_sizes;
        assert_eq!(
            effect.restore_state_payload(
                1,
                StatePayloadInput::new(&[], &before_left, &malformed, sizes).expect("sizes")
            ),
            Err(StatePayloadError {
                code: "effect.state.phase"
            })
        );
        assert_eq!(snapshot(effect.as_ref()), (before_left, before_right));
        let mut left = [f32::NAN];
        let mut right = [0.0];
        let report = effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("block"),
        );
        assert_eq!(report.sanitized_main_samples, 1);
        assert_eq!(left[0].to_bits(), 0.0_f32.to_bits());
    }
}
