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
    use miso_engine_core::KernelBackendV1;
    use miso_engine_dsp_reference::{
        ReferenceGateExpanderParameters, ReferenceGatePhase,
        reference_gate_expander_gain_reduction_db,
    };
    use miso_engine_effect_contract::{
        EffectBankProcessBlock, EffectProcessBlock, PrepareEffectLimits, PreparedNativeEffectBank,
        PreparedPortsV1, PreparedSidechainPort, StatePayloadInput, StatePayloadOutput,
        validate_descriptor_v1,
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
        request_at_rate(values, 48_000)
    }

    fn request_at_rate<'a>(
        values: &'a [InitialParameterValue],
        sample_rate: u32,
    ) -> PrepareEffectRequest<'a> {
        let quality = QUALITIES
            .iter()
            .find(|quality| quality.sample_rate == sample_rate)
            .expect("launch rate");
        PrepareEffectRequest {
            sample_rate,
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
                maximum_total_state_bytes: quality.maximum_state.total().expect("state total"),
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

    fn snapshot_bank(effect: &dyn PreparedNativeEffectBank, track: u32) -> (Vec<u8>, Vec<u8>) {
        let sizes = effect.metadata().program_key.state_sizes;
        let mut left = vec![0; sizes.left_bytes as usize];
        let mut right = vec![0; sizes.right_bytes as usize];
        effect
            .snapshot_track_state_payload(
                track,
                StatePayloadOutput::new(&mut [], &mut left, &mut right, sizes).expect("sizes"),
            )
            .expect("snapshot");
        (left, right)
    }

    fn add_report(total: &mut ProcessReport, report: ProcessReport) {
        total.sanitized_main_samples = total
            .sanitized_main_samples
            .saturating_add(report.sanitized_main_samples);
        total.sanitized_sidechain_samples = total
            .sanitized_sidechain_samples
            .saturating_add(report.sanitized_sidechain_samples);
        total.invalid_spans = total.invalid_spans.saturating_add(report.invalid_spans);
        total.recovered_left_samples = total
            .recovered_left_samples
            .saturating_add(report.recovered_left_samples);
        total.recovered_right_samples = total
            .recovered_right_samples
            .saturating_add(report.recovered_right_samples);
    }

    fn set_parameter(
        values: &mut [InitialParameterValue; PARAMETER_COUNT * 2],
        parameter: usize,
        left: f32,
        right: f32,
    ) {
        values[parameter * 2].value = left;
        values[parameter * 2 + 1].value = right;
    }

    fn active_values() -> [InitialParameterValue; PARAMETER_COUNT * 2] {
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

    fn assert_bits_eq(actual: &[f32], expected: &[f32], context: &str) {
        assert_eq!(actual.len(), expected.len(), "{context} length");
        for (index, (&actual, &expected)) in actual.iter().zip(expected).enumerate() {
            assert_eq!(
                actual.to_bits(),
                expected.to_bits(),
                "{context} sample {index}"
            );
        }
    }

    fn assert_cleared_runtime(payload: &[u8], hold_samples: u32) {
        assert_eq!(read_u32(payload, 0), 0, "cursor");
        assert_eq!(read_f32(payload, 2).to_bits(), 0.0_f32.to_bits(), "gain");
        assert_eq!(read_u32(payload, 3), PHASE_OPEN, "phase");
        assert_eq!(read_u32(payload, 4), hold_samples, "hold");
        for word in STATE_HEADER_WORDS..payload.len() / 4 {
            assert_eq!(
                read_f32(payload, word).to_bits(),
                0.0_f32.to_bits(),
                "ring {word}"
            );
        }
    }

    fn assert_discontinuity_payload(
        before: &[u8],
        after: &[u8],
        prepared: &[f32; PARAMETER_COUNT],
        hold_samples: u32,
    ) {
        assert_cleared_runtime(after, hold_samples);
        assert_eq!(
            read_f32(after, 1).to_bits(),
            prepared[7].to_bits(),
            "lookahead"
        );
        assert_eq!(
            read_f32(after, 5).to_bits(),
            prepared[4].to_bits(),
            "attack"
        );
        assert_eq!(
            read_f32(after, 6).to_bits(),
            prepared[5].to_bits(),
            "hold ms"
        );
        assert_eq!(
            read_f32(after, 7).to_bits(),
            prepared[6].to_bits(),
            "release"
        );
        for index in 0..RAMP_COUNT {
            let word = 8 + index * 3;
            assert_eq!(
                read_f32(after, word).to_bits(),
                read_f32(before, word + 1).to_bits(),
                "ramp {index} current snaps to target"
            );
            assert_eq!(
                read_f32(after, word + 1).to_bits(),
                read_f32(before, word + 1).to_bits(),
                "ramp {index} target retained"
            );
            assert_eq!(read_u32(after, word + 2), 0, "ramp {index} remaining");
        }
    }

    fn assert_full_reset_payload(
        payload: &[u8],
        prepared: &[f32; PARAMETER_COUNT],
        hold_samples: u32,
    ) {
        assert_cleared_runtime(payload, hold_samples);
        assert_eq!(
            read_f32(payload, 1).to_bits(),
            prepared[7].to_bits(),
            "lookahead"
        );
        assert_eq!(
            read_f32(payload, 5).to_bits(),
            prepared[4].to_bits(),
            "attack"
        );
        assert_eq!(
            read_f32(payload, 6).to_bits(),
            prepared[5].to_bits(),
            "hold ms"
        );
        assert_eq!(
            read_f32(payload, 7).to_bits(),
            prepared[6].to_bits(),
            "release"
        );
        for (index, value) in prepared[..RAMP_COUNT].iter().enumerate() {
            let word = 8 + index * 3;
            assert_eq!(
                read_f32(payload, word).to_bits(),
                value.to_bits(),
                "ramp {index}"
            );
            assert_eq!(
                read_f32(payload, word + 1).to_bits(),
                value.to_bits(),
                "ramp {index} target"
            );
            assert_eq!(read_u32(payload, word + 2), 0, "ramp {index} remaining");
        }
    }

    fn scalar_prepared(values: &[InitialParameterValue]) -> PreparedGateExpander {
        let request = request(values);
        let metadata = expected_prepared_metadata(&GATE_EXPANDER_DESCRIPTOR_V1, request)
            .expect("prepared metadata");
        let (left_defaults, right_defaults) = initial_defaults(values).expect("initial values");
        let ring_length = metadata.latency.0 as usize + 1;
        PreparedGateExpander {
            metadata,
            left_defaults,
            right_defaults,
            left: Lane::new(&left_defaults, ring_length, metadata.sample_rate).expect("left lane"),
            right: Lane::new(&right_defaults, ring_length, metadata.sample_rate)
                .expect("right lane"),
        }
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn w8_prepared(
        values: &[[InitialParameterValue; PARAMETER_COUNT * 2]; 8],
    ) -> PreparedGateExpanderBank<8> {
        let first = request(&values[0]);
        let effect_metadata = expected_prepared_metadata(&GATE_EXPANDER_DESCRIPTOR_V1, first)
            .expect("prepared metadata");
        let left_defaults = core::array::from_fn(|track| {
            initial_defaults(&values[track]).expect("left defaults").0
        });
        let right_defaults = core::array::from_fn(|track| {
            initial_defaults(&values[track]).expect("right defaults").1
        });
        let ring_length = effect_metadata.latency.0 as usize + 1;
        let kernel = PreparedGateGainKernelV1::try_new(KernelBackendV1::X86Avx2)
            .expect("Issue 048 requires an executed available W8 backend");
        PreparedGateExpanderBank {
            metadata: PreparedBankMetadata {
                width: BankWidth::Eight,
                program_key: effect_metadata.program_key(),
            },
            effect_metadata,
            kernel,
            left_defaults,
            right_defaults,
            left: core::array::from_fn(|track| {
                Lane::new(
                    &left_defaults[track],
                    ring_length,
                    effect_metadata.sample_rate,
                )
                .expect("left lane")
            }),
            right: core::array::from_fn(|track| {
                Lane::new(
                    &right_defaults[track],
                    ring_length,
                    effect_metadata.sample_rate,
                )
                .expect("right lane")
            }),
        }
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn packed_w8(samples: &[Vec<f32>]) -> Vec<f32> {
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

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn assert_w8_track_matches_scalar(packed: &[f32], scalar: &[f32], track: usize, context: &str) {
        for (frame, expected) in scalar.iter().enumerate() {
            assert_eq!(
                packed[frame * 8 + track].to_bits(),
                expected.to_bits(),
                "{context} track {track}, frame {frame}"
            );
        }
    }

    fn retarget_spans(first_sample: u64) -> [PreparedAutomationSpan; 8] {
        let targets = [(-64.0, -60.0), (16.0, 12.0), (64.0, 48.0), (12.0, 8.0)];
        core::array::from_fn(|index| {
            let parameter_index = index / 2;
            let left = index % 2 == 0;
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
                start_value: if left {
                    targets[parameter_index].0
                } else {
                    targets[parameter_index].1
                },
                end_value: if left {
                    targets[parameter_index].0
                } else {
                    targets[parameter_index].1
                },
            }
        })
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
            assert_eq!(quality.maximum_state.common_bytes, 0);
            assert_eq!(quality.maximum_state.left_bytes, lane_bytes);
            assert_eq!(quality.maximum_state.right_bytes, lane_bytes);
            assert_eq!(quality.maximum_state.total(), Some(total));
            assert_eq!(quality.scratch_fixed_bytes, 64);
            assert_eq!(quality.scratch_bytes_per_frame, 0);
        }
    }

    #[test]
    fn all_rate_caps_lookahead_and_fixed_latency_are_exact() {
        let factory = GateExpanderFactory;
        for quality in QUALITIES {
            let rate = quality.sample_rate;
            let latency = quality.latency.0 as usize;
            for lookahead in [0.0, 2.0, 10.0] {
                let mut values = initial_values();
                values[14].value = lookahead;
                values[15].value = lookahead;
                for bypass in [false, true] {
                    let mut preparation = request_at_rate(&values, rate);
                    preparation.bypass = bypass;
                    let mut effect = factory.prepare(preparation).expect("exact cap prepares");
                    assert_eq!(effect.metadata().latency, quality.latency);
                    let mut left = vec![0.0; latency + 1];
                    let mut right = vec![0.0; latency + 1];
                    left[0] = -0.5;
                    right[0] = 0.25;
                    for offset in (0..left.len()).step_by(128) {
                        let end = (offset + 128).min(left.len());
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
                    assert!(left[..latency].iter().all(|sample| sample.to_bits() == 0));
                    assert!(right[..latency].iter().all(|sample| sample.to_bits() == 0));
                    assert_eq!(left[latency].to_bits(), (-0.5_f32).to_bits());
                    assert_eq!(right[latency].to_bits(), 0.25_f32.to_bits());
                }
                let defaults: [f32; PARAMETER_COUNT] =
                    core::array::from_fn(|index| values[index * 2].value);
                let lane = Lane::new(&defaults, latency + 1, rate).expect("lane");
                assert_eq!(
                    lane.detector_delay,
                    latency - rounded_samples(lookahead, rate).expect("lookahead")
                );
            }
            let values = initial_values();
            let mut below = request_at_rate(&values, rate);
            below.limits.maximum_total_state_bytes -= 1;
            assert_eq!(
                factory.prepare(below).err().expect("state cap").code,
                "effect.resource.limit"
            );
            let mut below_scratch = request_at_rate(&values, rate);
            below_scratch.limits.maximum_scratch_bytes -= 1;
            assert_eq!(
                factory
                    .prepare(below_scratch)
                    .err()
                    .expect("scratch cap")
                    .code,
                "effect.resource.limit"
            );
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
        assert_eq!(
            reference_gate_expander_gain_reduction_db(
                -80.0,
                ReferenceGateExpanderParameters {
                    ratio: 1.0,
                    ..parameters
                },
                ReferenceGatePhase::Closed
            ),
            Ok(0.0)
        );
        let mut defaults =
            core::array::from_fn(|index| GATE_EXPANDER_PARAMETERS_V1[index].default_value);
        defaults[5] = 3.0 * 1000.0 / 48_000.0;
        let mut lane = Lane::new(&defaults, 481, 48_000).expect("lane");
        assert_eq!(lane.hold_samples, 3);
        for expected in [2, 1, 0] {
            transition(&mut lane, -46.1);
            assert_eq!(lane.phase, Phase::Open);
            assert_eq!(lane.hold_remaining, expected);
        }
        transition(&mut lane, -46.1);
        assert_eq!(lane.phase, Phase::Closed);
        transition(&mut lane, -40.0);
        assert_eq!((lane.phase, lane.hold_remaining), (Phase::Open, 3));
        transition(&mut lane, -46.0);
        assert_eq!((lane.phase, lane.hold_remaining), (Phase::Open, 3));
        transition(&mut lane, -46.1);
        assert_eq!((lane.phase, lane.hold_remaining), (Phase::Open, 2));
        let attack_residual = lane.attack_coefficient.powi(48);
        let release_residual = lane.release_coefficient.powi(4_800);
        let one_over_e = (-1.0_f64).exp() as f32;
        assert!((attack_residual - one_over_e).abs() <= 0.02 * one_over_e);
        assert!((release_residual - one_over_e).abs() <= 0.02 * one_over_e);
        assert_eq!(linked_levels(LinkMode::DualMono, -0.25, 0.75), (0.25, 0.75));
        assert_eq!(linked_levels(LinkMode::Maximum, -0.25, 0.75), (0.75, 0.75));
        assert_eq!(linked_levels(LinkMode::Average, -0.25, 0.75), (0.5, 0.5));
    }

    #[test]
    fn active_sidechain_and_exact_automation_state_are_observable() {
        let values = initial_values();
        let factory = GateExpanderFactory;
        let mut effect = factory.prepare(request(&values)).expect("prepare");
        let span = PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel: ParameterChannel::Left,
            parameter_index: 0,
            start_sample: 0,
            end_sample: 0,
            start_value: -20.0,
            end_value: -20.0,
        };
        let mut left = [0.0; 32];
        let mut right = [0.0; 32];
        assert_eq!(
            effect.process(
                EffectProcessBlock::new(&mut left, &mut right, None, 0, &[span], 128)
                    .expect("block")
            ),
            ProcessReport::default()
        );
        let (left_payload, _) = snapshot(effect.as_ref());
        assert_eq!(read_f32(&left_payload, 8), -30.0);
        assert_eq!(read_u32(&left_payload, 10), 32);
        let malformed = PreparedAutomationSpan {
            channel: ParameterChannel::Both,
            start_sample: 32,
            end_sample: 32,
            start_value: -60.0,
            end_value: -60.0,
            ..span
        };
        let retarget = PreparedAutomationSpan {
            start_sample: 32,
            end_sample: 32,
            start_value: -60.0,
            end_value: -60.0,
            ..span
        };
        let mut left = [0.0; 64];
        let mut right = [0.0; 64];
        let report = effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 32, &[malformed, retarget], 128)
                .expect("block"),
        );
        assert_eq!(report.invalid_spans, 1);
        let (left_payload, _) = snapshot(effect.as_ref());
        assert_eq!(read_f32(&left_payload, 8).to_bits(), (-60.0_f32).to_bits());
        assert_eq!(read_u32(&left_payload, 10), 0);

        let mut connected_values = initial_values();
        connected_values[10].value = 0.0;
        connected_values[11].value = 0.0;
        connected_values[12].value = 5.0;
        connected_values[13].value = 5.0;
        connected_values[14].value = 10.0;
        connected_values[15].value = 10.0;
        let mut unconnected = factory
            .prepare(request(&connected_values))
            .expect("unconnected");
        let mut connected = request(&connected_values);
        connected.ports.sidechain = PreparedSidechainPort::Connected {
            id: port_id("sidechain-in"),
            required: false,
        };
        let mut connected_effect = factory.prepare(connected).expect("connected");
        let mut unconnected_left = vec![0.25; 481];
        let mut unconnected_right = vec![0.25; 481];
        let mut connected_left = unconnected_left.clone();
        let mut connected_right = unconnected_right.clone();
        let side_left = vec![0.0; 481];
        let side_right = vec![0.0; 481];
        for offset in (0..481).step_by(128) {
            let end = (offset + 128).min(481);
            unconnected.process(
                EffectProcessBlock::new(
                    &mut unconnected_left[offset..end],
                    &mut unconnected_right[offset..end],
                    None,
                    offset as u64,
                    &[],
                    128,
                )
                .expect("unconnected block"),
            );
            connected_effect.process(
                EffectProcessBlock::new(
                    &mut connected_left[offset..end],
                    &mut connected_right[offset..end],
                    Some((&side_left[offset..end], &side_right[offset..end])),
                    offset as u64,
                    &[],
                    128,
                )
                .expect("connected block"),
            );
        }
        assert_eq!(unconnected_left[480].to_bits(), 0.25_f32.to_bits());
        assert_eq!(unconnected_right[480].to_bits(), 0.25_f32.to_bits());
        assert!(connected_left[480] < unconnected_left[480]);
        assert!(connected_right[480] < unconnected_right[480]);
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

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn unconnected_w8_bank_matches_scalar_pcm_state_reports_and_continuation() {
        if PreparedGateGainKernelV1::try_new(KernelBackendV1::X86Avx2).is_err() {
            return;
        }
        const WIDTH: usize = 8;
        const FRAMES: usize = 128;
        let factory = GateExpanderFactory;
        let mut values = vec![initial_values(); WIDTH];
        for (track, value) in values.iter_mut().enumerate() {
            value[0].value = -60.0 + track as f32;
            value[1].value = -48.0 + 0.5 * track as f32;
            value[2].value = 2.0 + track as f32;
            value[3].value = 3.0 + track as f32;
            value[10].value = 0.0;
            value[11].value = 0.0;
            value[14].value = [0.0, 2.0, 5.0, 10.0][track % 4];
            value[15].value = [10.0, 5.0, 2.0, 0.0][track % 4];
        }
        let mut requests = values
            .iter()
            .map(|value| request(value))
            .collect::<Vec<_>>();
        for request in &mut requests {
            request.link_mode = LinkMode::Maximum;
        }
        let mut bank = factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: KernelBackendV1::X86Avx2,
                width: BankWidth::Eight,
                requests: &requests,
            })
            .expect("bank prepare")
            .expect("available W8 bank");
        let bank_metadata = bank.metadata();
        assert_eq!(bank_metadata.width, BankWidth::Eight);
        assert_eq!(bank_metadata.program_key.state_sizes.total(), Some(7_856));
        assert_eq!(
            (bank_metadata
                .program_key
                .state_sizes
                .total()
                .expect("state")
                + 64)
                * 8,
            63_360,
            "W8 retains eight exact dual-mono state payloads plus defaults"
        );
        let mut scalars = requests
            .iter()
            .copied()
            .map(|item| factory.prepare(item).expect("scalar"))
            .collect::<Vec<_>>();
        let automation = [
            PreparedAutomationSpan {
                kind: AutomationSpanKind::Point,
                channel: ParameterChannel::Left,
                parameter_index: 0,
                start_sample: 0,
                end_sample: 0,
                start_value: -72.0,
                end_value: -72.0,
            },
            PreparedAutomationSpan {
                kind: AutomationSpanKind::Point,
                channel: ParameterChannel::Right,
                parameter_index: 1,
                start_sample: 0,
                end_sample: 0,
                start_value: 12.0,
                end_value: 12.0,
            },
        ];
        let offsets = [0, 0, 0, 1, 1, 1, 2, 2, 2];
        let mut scalar_reports = [ProcessReport::default(); WIDTH];
        let mut saved_bank = Vec::new();
        let mut saved_scalar = Vec::new();
        for block_index in 0..2 {
            let first = (block_index * FRAMES) as u64;
            let mut bank_left = vec![0.0; FRAMES * WIDTH];
            let mut bank_right = vec![0.0; FRAMES * WIDTH];
            let mut scalar_left = vec![vec![0.0; FRAMES]; WIDTH];
            let mut scalar_right = vec![vec![0.0; FRAMES]; WIDTH];
            for frame in 0..FRAMES {
                for track in 0..WIDTH {
                    let left = ((frame * 17 + track * 11 + block_index * 3) % 31) as f32 * 0.03125
                        - 0.46875;
                    let right =
                        ((frame * 13 + track * 7 + block_index * 5) % 29) as f32 * 0.03125 - 0.4375;
                    bank_left[frame * WIDTH + track] = left;
                    bank_right[frame * WIDTH + track] = right;
                    scalar_left[track][frame] = left;
                    scalar_right[track][frame] = right;
                }
            }
            let bank_report = bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut bank_left,
                    &mut bank_right,
                    None,
                    FRAMES as u32,
                    BankWidth::Eight,
                    first,
                    if block_index == 0 { &automation } else { &[] },
                    if block_index == 0 {
                        &offsets
                    } else {
                        &[0; WIDTH + 1]
                    },
                    128,
                )
                .expect("bank block"),
            );
            for track in 0..WIDTH {
                let spans = if block_index == 0 && track == 2 {
                    &automation[0..1]
                } else if block_index == 0 && track == 5 {
                    &automation[1..2]
                } else {
                    &[]
                };
                let report = scalars[track].process(
                    EffectProcessBlock::new(
                        &mut scalar_left[track],
                        &mut scalar_right[track],
                        None,
                        first,
                        spans,
                        128,
                    )
                    .expect("scalar block"),
                );
                add_report(&mut scalar_reports[track], report);
                assert_eq!(bank_report.reports[track], report);
                for frame in 0..FRAMES {
                    assert_eq!(
                        bank_left[frame * WIDTH + track].to_bits(),
                        scalar_left[track][frame].to_bits(),
                        "left track {track}, frame {frame}, block {block_index}"
                    );
                    assert_eq!(
                        bank_right[frame * WIDTH + track].to_bits(),
                        scalar_right[track][frame].to_bits(),
                        "right track {track}, frame {frame}, block {block_index}"
                    );
                }
            }
        }
        for (track, scalar) in scalars.iter().enumerate() {
            let bank_state = snapshot_bank(bank.as_ref(), track as u32);
            let scalar_state = snapshot(scalar.as_ref());
            assert_eq!(bank_state, scalar_state, "state track {track}");
            saved_bank.push(bank_state);
            saved_scalar.push(scalar_state);
        }
        assert_ne!(saved_bank[0], saved_bank[7], "track state is not shared");
        assert_ne!(
            saved_bank[0].0, saved_bank[0].1,
            "left/right state is not shared"
        );

        let mut resumed_bank = factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: KernelBackendV1::X86Avx2,
                width: BankWidth::Eight,
                requests: &requests,
            })
            .expect("resumed bank prepare")
            .expect("available resumed W8 bank");
        let mut resumed_scalars = requests
            .iter()
            .copied()
            .map(|item| factory.prepare(item).expect("resumed scalar"))
            .collect::<Vec<_>>();
        for track in 0..WIDTH {
            let sizes = resumed_scalars[track].metadata().state_sizes;
            resumed_bank
                .restore_track_state_payload(
                    track as u32,
                    1,
                    StatePayloadInput::new(&[], &saved_bank[track].0, &saved_bank[track].1, sizes)
                        .expect("bank input"),
                )
                .expect("bank restore");
            resumed_scalars[track]
                .restore_state_payload(
                    1,
                    StatePayloadInput::new(
                        &[],
                        &saved_scalar[track].0,
                        &saved_scalar[track].1,
                        sizes,
                    )
                    .expect("scalar input"),
                )
                .expect("scalar restore");
        }
        for block_index in 2..4 {
            let first = (block_index * FRAMES) as u64;
            let mut bank_left = vec![0.0; FRAMES * WIDTH];
            let mut bank_right = vec![0.0; FRAMES * WIDTH];
            let mut scalar_left = vec![vec![0.0; FRAMES]; WIDTH];
            let mut scalar_right = vec![vec![0.0; FRAMES]; WIDTH];
            for frame in 0..FRAMES {
                for track in 0..WIDTH {
                    let left =
                        ((frame + track * 3 + block_index * 5) % 23) as f32 * 0.0625 - 0.6875;
                    let right =
                        ((frame * 3 + track * 5 + block_index) % 19) as f32 * 0.0625 - 0.5625;
                    bank_left[frame * WIDTH + track] = left;
                    bank_right[frame * WIDTH + track] = right;
                    scalar_left[track][frame] = left;
                    scalar_right[track][frame] = right;
                }
            }
            let bank_report = resumed_bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut bank_left,
                    &mut bank_right,
                    None,
                    FRAMES as u32,
                    BankWidth::Eight,
                    first,
                    &[],
                    &[0; WIDTH + 1],
                    128,
                )
                .expect("continuation bank block"),
            );
            for track in 0..WIDTH {
                let report = resumed_scalars[track].process(
                    EffectProcessBlock::new(
                        &mut scalar_left[track],
                        &mut scalar_right[track],
                        None,
                        first,
                        &[],
                        128,
                    )
                    .expect("continuation scalar block"),
                );
                assert_eq!(bank_report.reports[track], report);
                for frame in 0..FRAMES {
                    assert_eq!(
                        bank_left[frame * WIDTH + track].to_bits(),
                        scalar_left[track][frame].to_bits()
                    );
                    assert_eq!(
                        bank_right[frame * WIDTH + track].to_bits(),
                        scalar_right[track][frame].to_bits()
                    );
                }
            }
        }
        assert!(
            scalar_reports
                .iter()
                .all(|report| report.invalid_spans == 0)
        );
    }

    #[test]
    fn bank_validation_precedes_fallback_and_unavailable_w4_is_legal() {
        const WIDTH: usize = 4;
        let factory = GateExpanderFactory;
        let values = vec![initial_values(); WIDTH];
        let requests = values
            .iter()
            .map(|value| request(value))
            .collect::<Vec<_>>();
        let unavailable = factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: KernelBackendV1::WasmSimd128,
                width: BankWidth::Four,
                requests: &requests,
            })
            .expect("valid W4 request");
        if PreparedGateGainKernelV1::try_new(KernelBackendV1::WasmSimd128).is_err() {
            assert!(
                unavailable.is_none(),
                "unavailable backend is a legal scalar fallback"
            );
        }
        let error = match factory.bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: KernelBackendV1::Scalar,
            width: BankWidth::Four,
            requests: &requests,
        }) {
            Ok(_) => panic!("backend/width mismatch must reject"),
            Err(error) => error,
        };
        assert_eq!(error.code, "effect.bank.requests");

        let mut malformed_values = values.clone();
        malformed_values[WIDTH - 1][0].value = f32::NAN;
        let malformed_requests = malformed_values
            .iter()
            .map(|value| request(value))
            .collect::<Vec<_>>();
        let error = match factory.bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: KernelBackendV1::WasmSimd128,
            width: BankWidth::Four,
            requests: &malformed_requests,
        }) {
            Ok(_) => panic!("malformed member must reject before backend fallback"),
            Err(error) => error,
        };
        assert_eq!(error.code, "effect.parameter.initial");

        let mut connected_requests = values
            .iter()
            .map(|value| request(value))
            .collect::<Vec<_>>();
        for item in &mut connected_requests {
            item.ports.sidechain = PreparedSidechainPort::Connected {
                id: port_id("sidechain-in"),
                required: false,
            };
        }
        assert!(
            factory
                .bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend: KernelBackendV1::WasmSimd128,
                    width: BankWidth::Four,
                    requests: &connected_requests,
                })
                .expect("connected requests validate")
                .is_none()
        );
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn reset_kinds_are_word_exact_for_scalar_and_executed_w8() {
        let mut scalar_values = initial_values();
        set_parameter(&mut scalar_values, 4, 2.0, 3.0);
        set_parameter(&mut scalar_values, 5, 17.0, 19.0);
        set_parameter(&mut scalar_values, 6, 11.0, 13.0);
        set_parameter(&mut scalar_values, 7, 10.0, 5.0);
        let (scalar_left_defaults, scalar_right_defaults) =
            initial_defaults(&scalar_values).expect("defaults");
        let mut scalar = scalar_prepared(&scalar_values);
        let mut left = vec![0.25; 128];
        let mut right = vec![-0.5; 128];
        scalar.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &retarget_spans(0), 128)
                .expect("scalar seed block"),
        );
        let scalar_before = snapshot(&scalar);
        scalar.reset(ResetKind::DiscontinuityKeepParameters);
        let scalar_discontinuity = snapshot(&scalar);
        assert_discontinuity_payload(
            &scalar_before.0,
            &scalar_discontinuity.0,
            &scalar_left_defaults,
            rounded_samples(scalar_left_defaults[5], 48_000).expect("hold") as u32,
        );
        assert_discontinuity_payload(
            &scalar_before.1,
            &scalar_discontinuity.1,
            &scalar_right_defaults,
            rounded_samples(scalar_right_defaults[5], 48_000).expect("hold") as u32,
        );
        scalar.reset(ResetKind::FullToDefaults);
        let scalar_full = snapshot(&scalar);
        assert_full_reset_payload(
            &scalar_full.0,
            &scalar_left_defaults,
            rounded_samples(scalar_left_defaults[5], 48_000).expect("hold") as u32,
        );
        assert_full_reset_payload(
            &scalar_full.1,
            &scalar_right_defaults,
            rounded_samples(scalar_right_defaults[5], 48_000).expect("hold") as u32,
        );

        let bank_values = core::array::from_fn(|track| {
            let mut values = scalar_values;
            set_parameter(&mut values, 4, 1.0 + track as f32, 2.0 + track as f32);
            set_parameter(&mut values, 5, 10.0 + track as f32, 20.0 + track as f32);
            values
        });
        let defaults: [([f32; PARAMETER_COUNT], [f32; PARAMETER_COUNT]); 8] =
            core::array::from_fn(|track| {
                initial_defaults(&bank_values[track]).expect("bank defaults")
            });
        let mut bank = w8_prepared(&bank_values);
        let left = packed_w8(
            &(0..8)
                .map(|track| vec![0.125 + track as f32 * 0.03125; 128])
                .collect::<Vec<_>>(),
        );
        let right = packed_w8(
            &(0..8)
                .map(|track| vec![-0.25 - track as f32 * 0.03125; 128])
                .collect::<Vec<_>>(),
        );
        let mut left = left;
        let mut right = right;
        bank.process_bank(
            EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                128,
                BankWidth::Eight,
                0,
                &retarget_spans(0),
                &[0, 8, 8, 8, 8, 8, 8, 8, 8],
                128,
            )
            .expect("bank seed block"),
        );
        let before: [(Vec<u8>, Vec<u8>); 8] =
            core::array::from_fn(|track| snapshot_bank(&bank, track as u32));
        bank.reset(ResetKind::DiscontinuityKeepParameters);
        for (track, (before, defaults)) in before.iter().zip(&defaults).enumerate() {
            let state = snapshot_bank(&bank, track as u32);
            assert_discontinuity_payload(
                &before.0,
                &state.0,
                &defaults.0,
                rounded_samples(defaults.0[5], 48_000).expect("hold") as u32,
            );
            assert_discontinuity_payload(
                &before.1,
                &state.1,
                &defaults.1,
                rounded_samples(defaults.1[5], 48_000).expect("hold") as u32,
            );
        }
        bank.reset(ResetKind::FullToDefaults);
        for (track, defaults) in defaults.iter().enumerate() {
            let state = snapshot_bank(&bank, track as u32);
            assert_full_reset_payload(
                &state.0,
                &defaults.0,
                rounded_samples(defaults.0[5], 48_000).expect("hold") as u32,
            );
            assert_full_reset_payload(
                &state.1,
                &defaults.1,
                rounded_samples(defaults.1[5], 48_000).expect("hold") as u32,
            );
        }
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn active_snapshot_restore_continues_against_uninterrupted_scalar_and_w8() {
        let factory = GateExpanderFactory;
        let values = active_values();
        let mut uninterrupted = factory.prepare(request(&values)).expect("scalar prepare");
        let mut first_sample = 0_u64;
        for _ in 0..5 {
            let mut left = vec![0.001; 128];
            let mut right = vec![0.002; 128];
            uninterrupted.process(
                EffectProcessBlock::new(&mut left, &mut right, None, first_sample, &[], 128)
                    .expect("warm block"),
            );
            first_sample += 128;
        }
        let active_spans = [
            PreparedAutomationSpan {
                kind: AutomationSpanKind::Point,
                channel: ParameterChannel::Left,
                parameter_index: 0,
                start_sample: first_sample,
                end_sample: first_sample,
                start_value: -40.0,
                end_value: -40.0,
            },
            PreparedAutomationSpan {
                kind: AutomationSpanKind::Point,
                channel: ParameterChannel::Right,
                parameter_index: 1,
                start_sample: first_sample,
                end_sample: first_sample,
                start_value: 8.0,
                end_value: 8.0,
            },
        ];
        let mut left = vec![0.001; 17];
        let mut right = vec![0.002; 17];
        uninterrupted.process(
            EffectProcessBlock::new(
                &mut left,
                &mut right,
                None,
                first_sample,
                &active_spans,
                128,
            )
            .expect("active scalar block"),
        );
        first_sample += 17;
        let scalar_state = snapshot(uninterrupted.as_ref());
        assert_ne!(read_f32(&scalar_state.0, 2).to_bits(), 0.0_f32.to_bits());
        assert_eq!(read_u32(&scalar_state.0, 10), 47);
        assert_eq!(read_u32(&scalar_state.1, 13), 47);
        let mut restored = factory.prepare(request(&values)).expect("restored scalar");
        let sizes = restored.metadata().state_sizes;
        restored
            .restore_state_payload(
                1,
                StatePayloadInput::new(&[], &scalar_state.0, &scalar_state.1, sizes)
                    .expect("scalar state"),
            )
            .expect("scalar restore");
        for frames in [1_usize, 63, 64, 128] {
            let mut left = (0..frames)
                .map(|index| 0.001 + ((first_sample + index as u64) % 7) as f32 * 0.00001)
                .collect::<Vec<_>>();
            let mut right = (0..frames)
                .map(|index| 0.002 + ((first_sample + index as u64) % 5) as f32 * 0.00002)
                .collect::<Vec<_>>();
            let mut restored_left = left.clone();
            let mut restored_right = right.clone();
            let report = uninterrupted.process(
                EffectProcessBlock::new(&mut left, &mut right, None, first_sample, &[], 128)
                    .expect("uninterrupted scalar partition"),
            );
            let restored_report = restored.process(
                EffectProcessBlock::new(
                    &mut restored_left,
                    &mut restored_right,
                    None,
                    first_sample,
                    &[],
                    128,
                )
                .expect("restored scalar partition"),
            );
            assert_eq!(report, restored_report, "scalar report after {frames}");
            assert_bits_eq(&left, &restored_left, "scalar left continuation");
            assert_bits_eq(&right, &restored_right, "scalar right continuation");
            assert_eq!(
                snapshot(uninterrupted.as_ref()),
                snapshot(restored.as_ref())
            );
            first_sample += frames as u64;
        }

        let bank_values = core::array::from_fn(|track| {
            let mut values = active_values();
            set_parameter(&mut values, 0, -20.0 - track as f32, -22.0 - track as f32);
            values
        });
        let mut bank = w8_prepared(&bank_values);
        let mut scalar_peers = bank_values
            .iter()
            .map(|values| scalar_prepared(values))
            .collect::<Vec<_>>();
        let mut bank_first = 0_u64;
        for _ in 0..5 {
            let scalar_left = (0..8)
                .map(|track| vec![0.001 + track as f32 * 0.00001; 128])
                .collect::<Vec<_>>();
            let scalar_right = (0..8)
                .map(|track| vec![0.002 + track as f32 * 0.00001; 128])
                .collect::<Vec<_>>();
            let mut bank_left = packed_w8(&scalar_left);
            let mut bank_right = packed_w8(&scalar_right);
            bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut bank_left,
                    &mut bank_right,
                    None,
                    128,
                    BankWidth::Eight,
                    bank_first,
                    &[],
                    &[0; 9],
                    128,
                )
                .expect("W8 warm block"),
            );
            for track in 0..8 {
                scalar_peers[track].process(
                    EffectProcessBlock::new(
                        &mut scalar_left[track].clone(),
                        &mut scalar_right[track].clone(),
                        None,
                        bank_first,
                        &[],
                        128,
                    )
                    .expect("scalar peer warm block"),
                );
            }
            bank_first += 128;
        }
        let bank_spans = (0..8)
            .map(|track| PreparedAutomationSpan {
                kind: AutomationSpanKind::Point,
                channel: ParameterChannel::Left,
                parameter_index: 0,
                start_sample: bank_first,
                end_sample: bank_first,
                start_value: -40.0 - track as f32,
                end_value: -40.0 - track as f32,
            })
            .collect::<Vec<_>>();
        let bank_offsets = [0, 1, 2, 3, 4, 5, 6, 7, 8];
        let scalar_left = (0..8)
            .map(|track| vec![0.001 + track as f32 * 0.00001; 17])
            .collect::<Vec<_>>();
        let scalar_right = (0..8)
            .map(|track| vec![0.002 + track as f32 * 0.00001; 17])
            .collect::<Vec<_>>();
        let mut bank_left = packed_w8(&scalar_left);
        let mut bank_right = packed_w8(&scalar_right);
        let bank_report = bank.process_bank(
            EffectBankProcessBlock::new(
                &mut bank_left,
                &mut bank_right,
                None,
                17,
                BankWidth::Eight,
                bank_first,
                &bank_spans,
                &bank_offsets,
                128,
            )
            .expect("active W8 block"),
        );
        for track in 0..8 {
            let mut left = scalar_left[track].clone();
            let mut right = scalar_right[track].clone();
            let report = scalar_peers[track].process(
                EffectProcessBlock::new(
                    &mut left,
                    &mut right,
                    None,
                    bank_first,
                    &bank_spans[track..=track],
                    128,
                )
                .expect("active scalar peer"),
            );
            assert_eq!(
                bank_report.reports[track], report,
                "W8 active report {track}"
            );
        }
        bank_first += 17;
        let saved_bank: [(Vec<u8>, Vec<u8>); 8] =
            core::array::from_fn(|track| snapshot_bank(&bank, track as u32));
        let saved_scalars = scalar_peers
            .iter()
            .map(|effect| snapshot(effect))
            .collect::<Vec<_>>();
        assert!(saved_bank.iter().all(|state| read_u32(&state.0, 10) == 47));
        let mut restored_bank = w8_prepared(&bank_values);
        let mut restored_scalars = bank_values
            .iter()
            .map(|values| scalar_prepared(values))
            .collect::<Vec<_>>();
        for track in 0..8 {
            let sizes = restored_bank.effect_metadata.state_sizes;
            restored_bank
                .restore_track_state_payload(
                    track as u32,
                    1,
                    StatePayloadInput::new(&[], &saved_bank[track].0, &saved_bank[track].1, sizes)
                        .expect("W8 state"),
                )
                .expect("W8 restore");
            restored_scalars[track]
                .restore_state_payload(
                    1,
                    StatePayloadInput::new(
                        &[],
                        &saved_scalars[track].0,
                        &saved_scalars[track].1,
                        sizes,
                    )
                    .expect("scalar state"),
                )
                .expect("scalar restore");
        }
        for frames in [1_usize, 63, 64, 128] {
            let original_left = (0..8)
                .map(|track| {
                    (0..frames)
                        .map(|index| {
                            0.001
                                + track as f32 * 0.00001
                                + ((bank_first + index as u64) % 7) as f32 * 0.000001
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let original_right = (0..8)
                .map(|track| {
                    (0..frames)
                        .map(|index| {
                            0.002
                                + track as f32 * 0.00001
                                + ((bank_first + index as u64) % 5) as f32 * 0.000002
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let mut original_bank_left = packed_w8(&original_left);
            let mut original_bank_right = packed_w8(&original_right);
            let mut restored_bank_left = original_bank_left.clone();
            let mut restored_bank_right = original_bank_right.clone();
            let original_report = bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut original_bank_left,
                    &mut original_bank_right,
                    None,
                    frames as u32,
                    BankWidth::Eight,
                    bank_first,
                    &[],
                    &[0; 9],
                    128,
                )
                .expect("uninterrupted W8 partition"),
            );
            let restored_report = restored_bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut restored_bank_left,
                    &mut restored_bank_right,
                    None,
                    frames as u32,
                    BankWidth::Eight,
                    bank_first,
                    &[],
                    &[0; 9],
                    128,
                )
                .expect("restored W8 partition"),
            );
            assert_eq!(original_report, restored_report, "W8 report after {frames}");
            assert_bits_eq(
                &original_bank_left,
                &restored_bank_left,
                "W8 left continuation",
            );
            assert_bits_eq(
                &original_bank_right,
                &restored_bank_right,
                "W8 right continuation",
            );
            for track in 0..8 {
                let mut left = original_left[track].clone();
                let mut right = original_right[track].clone();
                let report = restored_scalars[track].process(
                    EffectProcessBlock::new(&mut left, &mut right, None, bank_first, &[], 128)
                        .expect("restored scalar continuation"),
                );
                assert_eq!(
                    original_report.reports[track], report,
                    "W8 scalar report {track}"
                );
                assert_w8_track_matches_scalar(&original_bank_left, &left, track, "W8 scalar left");
                assert_w8_track_matches_scalar(
                    &original_bank_right,
                    &right,
                    track,
                    "W8 scalar right",
                );
                assert_eq!(
                    snapshot_bank(&bank, track as u32),
                    snapshot_bank(&restored_bank, track as u32),
                    "W8 restored payload {track}"
                );
                assert_eq!(
                    snapshot_bank(&bank, track as u32),
                    snapshot(&restored_scalars[track]),
                    "W8 scalar payload {track}"
                );
            }
            bank_first += frames as u64;
        }
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn signed_zero_identity_is_bit_exact_for_scalar_and_executed_w8() {
        let factory = GateExpanderFactory;
        let mut values = initial_values();
        set_parameter(&mut values, 5, 0.0, 0.0);
        set_parameter(&mut values, 7, 10.0, 10.0);
        let mut connected_request = request(&values);
        connected_request.ports.sidechain = PreparedSidechainPort::Connected {
            id: port_id("sidechain-in"),
            required: false,
        };
        let mut scalar = factory
            .prepare(connected_request)
            .expect("connected scalar");
        let mut scalar_left = vec![0.0; 481];
        let mut scalar_right = vec![0.0; 481];
        scalar_left[0] = -0.0;
        let sidechain_left = vec![1.0; 481];
        let sidechain_right = vec![1.0; 481];
        let mut total = ProcessReport::default();
        for offset in (0..481).step_by(128) {
            let end = (offset + 128).min(481);
            let report = scalar.process(
                EffectProcessBlock::new(
                    &mut scalar_left[offset..end],
                    &mut scalar_right[offset..end],
                    Some((&sidechain_left[offset..end], &sidechain_right[offset..end])),
                    offset as u64,
                    &[],
                    128,
                )
                .expect("signed-zero scalar block"),
            );
            add_report(&mut total, report);
        }
        assert_eq!(total, ProcessReport::default());
        assert_eq!(scalar_left[480].to_bits(), (-0.0_f32).to_bits());
        assert_eq!(scalar_right[480].to_bits(), 0.0_f32.to_bits());
        let scalar_state = snapshot(scalar.as_ref());
        assert_eq!(read_f32(&scalar_state.0, 2).to_bits(), 0.0_f32.to_bits());
        assert_eq!(read_f32(&scalar_state.1, 2).to_bits(), 0.0_f32.to_bits());

        let bank_values = [initial_values(); 8];
        let mut bank = w8_prepared(&bank_values);
        let mut tracks_left = vec![vec![0.0; 481]; 8];
        let mut tracks_right = vec![vec![0.0; 481]; 8];
        for track in 0..8 {
            if track % 2 == 0 {
                tracks_left[track][0] = -0.0;
            } else {
                tracks_right[track][0] = -0.0;
            }
        }
        let mut bank_left = packed_w8(&tracks_left);
        let mut bank_right = packed_w8(&tracks_right);
        for offset in (0..481).step_by(128) {
            let end = (offset + 128).min(481);
            let report = bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut bank_left[offset * 8..end * 8],
                    &mut bank_right[offset * 8..end * 8],
                    None,
                    (end - offset) as u32,
                    BankWidth::Eight,
                    offset as u64,
                    &[],
                    &[0; 9],
                    128,
                )
                .expect("signed-zero W8 block"),
            );
            assert_eq!(report, BankProcessReport::empty(BankWidth::Eight));
        }
        for track in 0..8 {
            let expected_left = if track % 2 == 0 { -0.0_f32 } else { 0.0 };
            let expected_right = if track % 2 == 0 { 0.0 } else { -0.0_f32 };
            assert_eq!(
                bank_left[480 * 8 + track].to_bits(),
                expected_left.to_bits(),
                "W8 left zero sign {track}"
            );
            assert_eq!(
                bank_right[480 * 8 + track].to_bits(),
                expected_right.to_bits(),
                "W8 right zero sign {track}"
            );
            let state = snapshot_bank(&bank, track as u32);
            assert_eq!(read_f32(&state.0, 2).to_bits(), 0.0_f32.to_bits());
            assert_eq!(read_f32(&state.1, 2).to_bits(), 0.0_f32.to_bits());
        }
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn injected_lane_recovery_has_scalar_w8_output_state_and_report_parity() {
        const FAULT_TRACK: usize = 3;
        let bank_values = core::array::from_fn(|track| {
            let mut values = active_values();
            set_parameter(&mut values, 0, -20.0 - track as f32, -22.0 - track as f32);
            values
        });
        let mut bank = w8_prepared(&bank_values);
        let mut scalars = bank_values
            .iter()
            .map(|values| scalar_prepared(values))
            .collect::<Vec<_>>();
        let mut first_sample = 0_u64;
        for _ in 0..5 {
            let source_left = (0..8)
                .map(|track| vec![0.001 + track as f32 * 0.00001; 128])
                .collect::<Vec<_>>();
            let source_right = (0..8)
                .map(|track| vec![0.002 + track as f32 * 0.00001; 128])
                .collect::<Vec<_>>();
            let mut bank_left = packed_w8(&source_left);
            let mut bank_right = packed_w8(&source_right);
            let bank_report = bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut bank_left,
                    &mut bank_right,
                    None,
                    128,
                    BankWidth::Eight,
                    first_sample,
                    &[],
                    &[0; 9],
                    128,
                )
                .expect("recovery warm W8 block"),
            );
            assert_eq!(bank_report, BankProcessReport::empty(BankWidth::Eight));
            for track in 0..8 {
                let mut left = source_left[track].clone();
                let mut right = source_right[track].clone();
                let report = scalars[track].process(
                    EffectProcessBlock::new(&mut left, &mut right, None, first_sample, &[], 128)
                        .expect("recovery warm scalar block"),
                );
                assert_eq!(report, ProcessReport::default());
                assert_w8_track_matches_scalar(&bank_left, &left, track, "recovery warm left");
                assert_w8_track_matches_scalar(&bank_right, &right, track, "recovery warm right");
                assert_eq!(
                    snapshot_bank(&bank, track as u32),
                    snapshot(&scalars[track]),
                    "recovery warm state {track}"
                );
            }
            first_sample += 128;
        }

        let saved_fault = snapshot(&scalars[FAULT_TRACK]);
        let mut control = scalar_prepared(&bank_values[FAULT_TRACK]);
        let sizes = control.metadata.state_sizes;
        control
            .restore_state_payload(
                1,
                StatePayloadInput::new(&[], &saved_fault.0, &saved_fault.1, sizes)
                    .expect("control state"),
            )
            .expect("control restore");
        bank.left[FAULT_TRACK].gain_reduction_db = f32::NAN;
        scalars[FAULT_TRACK].left.gain_reduction_db = f32::NAN;

        let source_left = (0..8)
            .map(|track| vec![0.001 + track as f32 * 0.00001])
            .collect::<Vec<_>>();
        let source_right = (0..8)
            .map(|track| vec![0.002 + track as f32 * 0.00001])
            .collect::<Vec<_>>();
        let mut bank_left = packed_w8(&source_left);
        let mut bank_right = packed_w8(&source_right);
        let bank_report = bank.process_bank(
            EffectBankProcessBlock::new(
                &mut bank_left,
                &mut bank_right,
                None,
                1,
                BankWidth::Eight,
                first_sample,
                &[],
                &[0; 9],
                128,
            )
            .expect("injected W8 frame"),
        );
        for track in 0..8 {
            let mut left = source_left[track].clone();
            let mut right = source_right[track].clone();
            let report = scalars[track].process(
                EffectProcessBlock::new(&mut left, &mut right, None, first_sample, &[], 128)
                    .expect("injected scalar frame"),
            );
            assert_eq!(
                bank_report.reports[track], report,
                "recovery report {track}"
            );
            assert_w8_track_matches_scalar(&bank_left, &left, track, "recovery left");
            assert_w8_track_matches_scalar(&bank_right, &right, track, "recovery right");
            assert_eq!(
                snapshot_bank(&bank, track as u32),
                snapshot(&scalars[track]),
                "recovery state {track}"
            );
            if track == FAULT_TRACK {
                assert_eq!(report.recovered_left_samples, 1);
                assert_eq!(report.recovered_right_samples, 0);
            } else {
                assert_eq!(report, ProcessReport::default());
            }
        }
        assert_eq!(
            bank_left[FAULT_TRACK].to_bits(),
            source_left[FAULT_TRACK][0].to_bits(),
            "recovery emits delayed dry"
        );
        let recovered = snapshot_bank(&bank, FAULT_TRACK as u32);
        assert_eq!(read_f32(&recovered.0, 2).to_bits(), 0.0_f32.to_bits());
        assert_eq!(read_u32(&recovered.0, 3), PHASE_OPEN);
        assert_eq!(read_u32(&recovered.0, 4), 0);

        let mut control_left = source_left[FAULT_TRACK].clone();
        let mut control_right = source_right[FAULT_TRACK].clone();
        let control_report = control.process(
            EffectProcessBlock::new(
                &mut control_left,
                &mut control_right,
                None,
                first_sample,
                &[],
                128,
            )
            .expect("uninterrupted control frame"),
        );
        assert_eq!(control_report, ProcessReport::default());
        assert!(control_left[0].abs() < source_left[FAULT_TRACK][0].abs());
        assert_eq!(
            bank_right[FAULT_TRACK].to_bits(),
            control_right[0].to_bits()
        );
        assert_eq!(
            recovered.1,
            snapshot(&control).1,
            "right lane is unaffected"
        );
    }
}
