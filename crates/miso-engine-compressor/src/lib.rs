//! Launch feed-forward peak compressor descriptor and factory scaffold.
//!
//! Scalar processing follows in its own bounded implementation edit.
#![allow(missing_docs)]

use miso_engine_core::{CompressorGainMixKernelError, PreparedCompressorGainMixKernelV1};
use miso_engine_effect_contract::{
    AutomationRate, AutomationSpanKind, BankProcessReport, BankWidth, EffectBankProcessBlock,
    EffectDescriptorV1, EffectPrepareError, EffectProcessBlock, EffectQuality,
    InitialParameterValue, LatencySamples, LinkMode, LinkModeSet, NativeEffectFactory,
    ParameterChannel, ParameterChannelPolicy, ParameterDescriptorV1, ParameterDomain, ParameterId,
    ParameterMapping, ParameterUnit, PortDescriptorV1, PortId, PortLayout, PortRole,
    PrepareEffectBankRequest, PrepareEffectRequest, PreparedAutomationSpan, PreparedBankMetadata,
    PreparedEffectMetadata, PreparedNativeEffect, PreparedNativeEffectBank, ProcessReport,
    ResetKind, SmoothingRule, StatePayloadError, StatePayloadInput, StatePayloadOutput,
    StatePayloadSizes, TailSamples, expected_prepared_metadata, sanitize_sample,
};

const fn effect_id(value: &'static str) -> miso_engine_effect_contract::EffectId {
    match miso_engine_effect_contract::EffectId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("valid static effect id"),
    }
}

const fn port_id(value: &'static str) -> PortId {
    match PortId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("valid static port id"),
    }
}

const fn parameter_id(value: u32) -> ParameterId {
    match ParameterId::new(value) {
        Some(value) => value,
        None => panic!("nonzero parameter id"),
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

/// Frozen V1 parameter descriptors. Parameter positions and stable IDs are identical.
pub const COMPRESSOR_PARAMETERS_V1: [ParameterDescriptorV1; 8] = [
    parameter(
        1,
        "threshold",
        "dB",
        ParameterUnit::Db,
        -80.0,
        0.0,
        -18.0,
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
        "knee",
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
        4,
        "attack",
        "ms",
        ParameterUnit::Milliseconds,
        0.1,
        200.0,
        10.0,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        5,
        "release",
        "ms",
        ParameterUnit::Milliseconds,
        5.0,
        5000.0,
        100.0,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        6,
        "makeup",
        "dB",
        ParameterUnit::Db,
        -24.0,
        24.0,
        0.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        7,
        "mix",
        "linear",
        ParameterUnit::Linear,
        0.0,
        1.0,
        1.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        8,
        "lookahead",
        "ms",
        ParameterUnit::Milliseconds,
        0.0,
        20.0,
        5.0,
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
    let per_lane = (24 + 2 * ring_length) * 4;
    miso_engine_effect_contract::QualityDescriptorV1 {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(latency),
        tail: TailSamples::Infinite,
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
    quality(44_100, 882),
    quality(48_000, 960),
    quality(88_200, 1764),
    quality(96_000, 1920),
];

/// Immutable launch compressor descriptor.
pub const COMPRESSOR_DESCRIPTOR_V1: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("miso.compressor"),
    display_name: "Compressor",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::ALL,
    parameters: &COMPRESSOR_PARAMETERS_V1,
    ports: &PORTS,
    qualities: &QUALITIES,
};

/// Factory entry point for the V1 compressor implementation.
#[derive(Clone, Copy, Debug, Default)]
pub struct CompressorFactory;

const PARAMETER_COUNT: usize = 8;
const RAMP_COUNT: usize = 7;
const STATE_HEADER_WORDS: usize = 24;

#[derive(Clone, Copy, Debug)]
struct Ramp {
    current: f32,
    target: f32,
    remaining: u32,
}

impl Ramp {
    fn fixed(value: f32) -> Self {
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

#[derive(Debug)]
struct Lane {
    cursor: u32,
    lookahead_ms: f32,
    detector_delay: usize,
    gain_reduction_db: f32,
    ramps: [Ramp; RAMP_COUNT],
    main_ring: Box<[f32]>,
    detector_ring: Box<[f32]>,
}

#[derive(Clone, Copy)]
struct GainMixFrame {
    dry: f32,
    gain: f32,
    mix: f32,
    dry_identity: bool,
    wet_identity: bool,
}

impl Lane {
    fn new(defaults: &[f32; PARAMETER_COUNT], ring_length: usize, sample_rate: u32) -> Self {
        Self {
            cursor: 0,
            lookahead_ms: defaults[7],
            detector_delay: detector_delay(defaults[7], sample_rate, ring_length),
            gain_reduction_db: 0.0,
            ramps: core::array::from_fn(|index| Ramp::fixed(defaults[index])),
            main_ring: vec![0.0; ring_length].into_boxed_slice(),
            detector_ring: vec![0.0; ring_length].into_boxed_slice(),
        }
    }

    fn full_reset(&mut self, defaults: &[f32; PARAMETER_COUNT], sample_rate: u32) {
        self.cursor = 0;
        self.lookahead_ms = defaults[7];
        self.detector_delay = detector_delay(defaults[7], sample_rate, self.main_ring.len());
        self.gain_reduction_db = 0.0;
        self.ramps = core::array::from_fn(|index| Ramp::fixed(defaults[index]));
        self.main_ring.fill(0.0);
        self.detector_ring.fill(0.0);
    }

    fn discontinuity_reset(&mut self) {
        self.cursor = 0;
        self.gain_reduction_db = 0.0;
        for ramp in &mut self.ramps {
            ramp.current = ramp.target;
            ramp.remaining = 0;
        }
        self.main_ring.fill(0.0);
        self.detector_ring.fill(0.0);
    }
}

fn detector_delay(lookahead_ms: f32, sample_rate: u32, ring_length: usize) -> usize {
    let latency = ring_length - 1;
    let lookahead = ((lookahead_ms as f64 * sample_rate as f64 / 1000.0) + 0.5).floor() as usize;
    latency - lookahead.min(latency)
}

/// A prepared allocation-free scalar compressor instance.
#[derive(Debug)]
pub struct PreparedCompressor {
    metadata: PreparedEffectMetadata,
    left_defaults: [f32; PARAMETER_COUNT],
    right_defaults: [f32; PARAMETER_COUNT],
    left: Lane,
    right: Lane,
}

struct PreparedCompressorBank<const W: usize> {
    metadata: PreparedBankMetadata,
    effect_metadata: PreparedEffectMetadata,
    kernel: PreparedCompressorGainMixKernelV1,
    left_defaults: [[f32; PARAMETER_COUNT]; W],
    right_defaults: [[f32; PARAMETER_COUNT]; W],
    left: [Lane; W],
    right: [Lane; W],
}

impl NativeEffectFactory for CompressorFactory {
    fn descriptor(&self) -> &'static EffectDescriptorV1 {
        &COMPRESSOR_DESCRIPTOR_V1
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
        Ok(Box::new(PreparedCompressor {
            metadata,
            left_defaults,
            right_defaults,
            left: Lane::new(&left_defaults, ring_length, metadata.sample_rate),
            right: Lane::new(&right_defaults, ring_length, metadata.sample_rate),
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
    factory: &CompressorFactory,
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
    if !same_program {
        return Ok(None);
    }
    if !matches!(
        metadata.ports.sidechain,
        miso_engine_effect_contract::PreparedSidechainPort::Unconnected {
            id,
            required: false,
        } if id == port_id("sidechain-in")
    ) {
        return Ok(None);
    }
    let kernel = match PreparedCompressorGainMixKernelV1::try_new(request.backend) {
        Ok(kernel) => kernel,
        Err(CompressorGainMixKernelError::BackendUnavailable) => return Ok(None),
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
    Ok(Some(Box::new(PreparedCompressorBank::<W> {
        metadata: PreparedBankMetadata {
            width: request.width,
            program_key: metadata.program_key(),
        },
        effect_metadata: metadata,
        kernel,
        left_defaults,
        right_defaults,
        left: core::array::from_fn(|track| {
            Lane::new(&left_defaults[track], ring_length, metadata.sample_rate)
        }),
        right: core::array::from_fn(|track| {
            Lane::new(&right_defaults[track], ring_length, metadata.sample_rate)
        }),
    })))
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
    for (index, parameter) in COMPRESSOR_PARAMETERS_V1.iter().enumerate() {
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

impl PreparedNativeEffect for PreparedCompressor {
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
        for index in 0..block.left.len() {
            advance_ramps(&mut self.left);
            advance_ramps(&mut self.right);
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
            let (level_left, level_right) =
                linked_levels(self.metadata.link_mode, source_left, source_right);
            block.left[index] = process_lane(
                main_left,
                level_left,
                &mut self.left,
                self.metadata.sample_rate,
                self.metadata.bypass,
                &mut report.recovered_left_samples,
            );
            block.right[index] = process_lane(
                main_right,
                level_right,
                &mut self.right,
                self.metadata.sample_rate,
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
            return Err(StatePayloadError {
                code: "effect.state.version",
            });
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

impl<const W: usize> PreparedNativeEffectBank for PreparedCompressorBank<W> {
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
            process_bank_frame(
                &mut self.left,
                &mut self.right,
                self.kernel,
                self.effect_metadata,
                &mut report,
                &mut block.left[frame * W..(frame + 1) * W],
                &mut block.right[frame * W..(frame + 1) * W],
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
    kernel: PreparedCompressorGainMixKernelV1,
    metadata: PreparedEffectMetadata,
    report: &mut BankProcessReport,
    left_samples: &mut [f32],
    right_samples: &mut [f32],
) {
    let mut left_dry = [0.0; W];
    let mut right_dry = [0.0; W];
    let mut detector_left = [0.0; W];
    let mut detector_right = [0.0; W];
    for track in 0..W {
        advance_ramps(&mut left_lanes[track]);
        advance_ramps(&mut right_lanes[track]);
        left_dry[track] = sanitize(
            left_samples[track],
            &mut report.reports[track].sanitized_main_samples,
        );
        right_dry[track] = sanitize(
            right_samples[track],
            &mut report.reports[track].sanitized_main_samples,
        );
    }
    for track in 0..W {
        let (left, right) = linked_levels(metadata.link_mode, left_dry[track], right_dry[track]);
        detector_left[track] = left;
        detector_right[track] = right;
    }
    let mut left_values = [0.0; W];
    let mut right_values = [0.0; W];
    let mut left_gain = [0.0; W];
    let mut right_gain = [0.0; W];
    let mut left_mix = [0.0; W];
    let mut right_mix = [0.0; W];
    let mut left_dry_mask = [0_u32; W];
    let mut right_dry_mask = [0_u32; W];
    let mut left_wet_mask = [0_u32; W];
    let mut right_wet_mask = [0_u32; W];
    for track in 0..W {
        let left = prepare_lane_gain(
            left_dry[track],
            detector_left[track],
            &mut left_lanes[track],
            metadata.sample_rate,
            metadata.bypass,
            &mut report.reports[track].recovered_left_samples,
        );
        let right = prepare_lane_gain(
            right_dry[track],
            detector_right[track],
            &mut right_lanes[track],
            metadata.sample_rate,
            metadata.bypass,
            &mut report.reports[track].recovered_right_samples,
        );
        left_values[track] = left.dry;
        right_values[track] = right.dry;
        left_gain[track] = left.gain;
        right_gain[track] = right.gain;
        left_mix[track] = left.mix;
        right_mix[track] = right.mix;
        left_dry_mask[track] = u32::from(left.dry_identity).wrapping_neg();
        right_dry_mask[track] = u32::from(right.dry_identity).wrapping_neg();
        left_wet_mask[track] = u32::from(!left.dry_identity && left.wet_identity).wrapping_neg();
        right_wet_mask[track] = u32::from(!right.dry_identity && right.wet_identity).wrapping_neg();
    }
    if kernel
        .process_gain_mix(
            &mut left_values,
            &left_gain,
            &left_mix,
            &left_dry_mask,
            &left_wet_mask,
        )
        .is_err()
        || kernel
            .process_gain_mix(
                &mut right_values,
                &right_gain,
                &right_mix,
                &right_dry_mask,
                &right_wet_mask,
            )
            .is_err()
    {
        return;
    }
    for track in 0..W {
        left_samples[track] = finish_bank_output(
            left_values[track],
            left_lanes[track].main_ring
                [(left_lanes[track].cursor as usize) % left_lanes[track].main_ring.len()],
            &mut left_lanes[track],
            &mut report.reports[track].recovered_left_samples,
        );
        right_samples[track] = finish_bank_output(
            right_values[track],
            right_lanes[track].main_ring
                [(right_lanes[track].cursor as usize) % right_lanes[track].main_ring.len()],
            &mut right_lanes[track],
            &mut report.reports[track].recovered_right_samples,
        );
    }
}

fn finish_bank_output(value: f32, delayed: f32, lane: &mut Lane, recovered: &mut u64) -> f32 {
    match flushed(value) {
        Some(value) => value,
        None => recover(lane, delayed, recovered),
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

fn advance_ramps(lane: &mut Lane) {
    for ramp in &mut lane.ramps {
        ramp.advance();
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
            let value = 0.5_f32 * left + 0.5_f32 * right;
            (value, value)
        }
    }
}

fn process_lane(
    main: f32,
    detector: f32,
    lane: &mut Lane,
    sample_rate: u32,
    bypass: bool,
    recovered: &mut u64,
) -> f32 {
    let frame = prepare_lane_gain(main, detector, lane, sample_rate, bypass, recovered);
    if frame.dry_identity {
        return frame.dry;
    }
    let Some(wet) = flushed(frame.dry * frame.gain) else {
        return recover(lane, frame.dry, recovered);
    };
    if frame.wet_identity {
        return wet;
    }
    let delta = wet - frame.dry;
    match flushed(frame.dry + frame.mix * delta) {
        Some(value) => value,
        None => recover(lane, frame.dry, recovered),
    }
}

fn prepare_lane_gain(
    main: f32,
    detector: f32,
    lane: &mut Lane,
    sample_rate: u32,
    bypass: bool,
    recovered: &mut u64,
) -> GainMixFrame {
    let ring_length = lane.main_ring.len();
    let cursor = lane.cursor as usize;
    lane.main_ring[cursor] = main;
    lane.detector_ring[cursor] = detector;
    let delayed = lane.main_ring[(cursor + 1) % ring_length];
    let detector = lane.detector_ring[(cursor + ring_length - lane.detector_delay) % ring_length];
    lane.cursor = ((cursor + 1) % ring_length) as u32;

    let threshold = lane.ramps[0].current;
    let ratio = lane.ramps[1].current;
    let knee = lane.ramps[2].current;
    let attack_ms = lane.ramps[3].current;
    let release_ms = lane.ramps[4].current;
    let makeup_db = lane.ramps[5].current;
    let mix = lane.ramps[6].current;
    let Some(target) = gain_reduction_target(detector, threshold, ratio, knee) else {
        return recovery_frame(lane, delayed, recovered);
    };
    let Some(attack) = flushed((-1.0_f32 / (0.001_f32 * attack_ms * sample_rate as f32)).exp())
    else {
        return recovery_frame(lane, delayed, recovered);
    };
    let Some(release) = flushed((-1.0_f32 / (0.001_f32 * release_ms * sample_rate as f32)).exp())
    else {
        return recovery_frame(lane, delayed, recovered);
    };
    let coefficient = if target < lane.gain_reduction_db {
        attack
    } else {
        release
    };
    let p0 = coefficient * lane.gain_reduction_db;
    let p1 = (1.0_f32 - coefficient) * target;
    let Some(gain_reduction_db) = flushed(p0 + p1) else {
        return recovery_frame(lane, delayed, recovered);
    };
    lane.gain_reduction_db = gain_reduction_db;
    let Some(gain) = flushed(10.0_f32.powf((gain_reduction_db + makeup_db) * 0.05_f32)) else {
        return recovery_frame(lane, delayed, recovered);
    };
    GainMixFrame {
        dry: delayed,
        gain,
        mix,
        dry_identity: bypass
            || mix == 0.0
            || (gain_reduction_db == 0.0 && makeup_db.to_bits() == 0.0_f32.to_bits()),
        wet_identity: mix.to_bits() == 1.0_f32.to_bits(),
    }
}

fn recovery_frame(lane: &mut Lane, delayed: f32, recovered: &mut u64) -> GainMixFrame {
    let _ = recover(lane, delayed, recovered);
    GainMixFrame {
        dry: delayed,
        gain: 1.0,
        mix: 0.0,
        dry_identity: true,
        wet_identity: false,
    }
}

fn gain_reduction_target(level: f32, threshold: f32, ratio: f32, knee: f32) -> Option<f32> {
    let u0 = level.max(1.0e-8_f32);
    let x = (20.0_f32 * u0.log10()).clamp(-160.0_f32, 24.0_f32);
    let h = 0.5_f32 * knee;
    let lo = threshold - h;
    let hi = threshold + h;
    let q = 1.0_f32 / ratio;
    let y = if knee == 0.0 && x <= threshold {
        x
    } else if knee == 0.0 && x > threshold {
        threshold + (x - threshold) * q
    } else if knee > 0.0 && x < lo {
        x
    } else if knee > 0.0 && x > hi {
        threshold + (x - threshold) * q
    } else {
        let v = (x - threshold) + h;
        let p0 = v * v;
        let p1 = (q - 1.0_f32) * p0;
        let p2 = 2.0_f32 * knee;
        x + p1 / p2
    };
    flushed((y - x).clamp(-100.0_f32, 0.0_f32))
}

fn flushed(value: f32) -> Option<f32> {
    if !value.is_finite() {
        None
    } else if value.is_subnormal() {
        Some(0.0)
    } else {
        Some(value)
    }
}

fn recover(lane: &mut Lane, delayed: f32, recovered: &mut u64) -> f32 {
    lane.gain_reduction_db = 0.0;
    *recovered = recovered.saturating_add(1);
    delayed
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
            && parameter_value_valid(&COMPRESSOR_PARAMETERS_V1[parameter_index], span.start_value)
            && last_order.is_none_or(|previous| order > previous)
            && pending[lane_index][parameter_index].is_none();
        if !valid {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        }
        last_order = Some(order);
        pending[lane_index][parameter_index] = Some(normalize_zero(span.start_value));
    }
    for (parameter_index, (left_ramp, right_ramp)) in left
        .ramps
        .iter_mut()
        .zip(right.ramps.iter_mut())
        .enumerate()
    {
        if let Some(value) = pending[0][parameter_index] {
            left_ramp.target = value;
            left_ramp.remaining = 64;
        }
        if let Some(value) = pending[1][parameter_index] {
            right_ramp.target = value;
            right_ramp.remaining = 64;
        }
    }
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
    for (index, ramp) in lane.ramps.iter().enumerate() {
        let word = 3 + index * 3;
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

fn write_u32(bytes: &mut [u8], word: usize, value: u32) {
    let offset = word * 4;
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn write_f32(bytes: &mut [u8], word: usize, value: f32) {
    write_u32(bytes, word, value.to_bits());
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
    if !parameter_state_valid(7, lookahead_ms) {
        return Err(state_error("effect.state.parameter"));
    }
    let gain_reduction_db = read_f32(bytes, 2);
    if !(normal_or_zero(gain_reduction_db) && (-100.0..=0.0).contains(&gain_reduction_db)) {
        return Err(state_error("effect.state.gain"));
    }
    let mut ramps = [Ramp::fixed(0.0); RAMP_COUNT];
    for (index, ramp) in ramps.iter_mut().enumerate() {
        let word = 3 + index * 3;
        let current = read_f32(bytes, word);
        let target = read_f32(bytes, word + 1);
        let remaining = read_u32(bytes, word + 2);
        if !parameter_state_valid(index, current)
            || !parameter_state_valid(index, target)
            || remaining > 64
        {
            return Err(state_error("effect.state.parameter"));
        }
        *ramp = Ramp {
            current,
            target,
            remaining,
        };
    }
    let mut lane = Lane::new(
        &[0.0, 1.0, 0.0, 0.1, 5.0, 0.0, 0.0, lookahead_ms],
        ring_length,
        sample_rate,
    );
    lane.cursor = cursor;
    lane.lookahead_ms = lookahead_ms;
    lane.gain_reduction_db = normalize_zero(gain_reduction_db);
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

fn read_u32(bytes: &[u8], word: usize) -> u32 {
    let offset = word * 4;
    u32::from_le_bytes(
        bytes[offset..offset + 4]
            .try_into()
            .expect("state section length is validated"),
    )
}

fn read_f32(bytes: &[u8], word: usize) -> f32 {
    f32::from_bits(read_u32(bytes, word))
}

fn normal_or_zero(value: f32) -> bool {
    value.is_finite() && (value == 0.0 || value.is_normal())
}

fn parameter_state_valid(index: usize, value: f32) -> bool {
    !negative_zero(value) && parameter_value_valid(&COMPRESSOR_PARAMETERS_V1[index], value)
}

const fn state_error(code: &'static str) -> StatePayloadError {
    StatePayloadError { code }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_core::KernelBackendV1;
    use miso_engine_dsp_reference::{ReferenceCompressorParameters, ReferencePeakCompressor};
    use miso_engine_effect_contract::{
        EffectBankProcessBlock, EffectProcessBlock, PrepareEffectLimits, PreparedPortsV1,
        PreparedSidechainPort, StatePayloadInput, StatePayloadOutput, validate_descriptor_v1,
    };

    fn initial_values() -> [InitialParameterValue; 16] {
        core::array::from_fn(|index| {
            let parameter = index / 2;
            InitialParameterValue {
                parameter_index: parameter as u32,
                channel: if index % 2 == 0 {
                    ParameterChannel::Left
                } else {
                    ParameterChannel::Right
                },
                value: COMPRESSOR_PARAMETERS_V1[parameter].default_value,
            }
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
                maximum_total_state_bytes: 15_568,
                maximum_scratch_bytes: 64,
                maximum_automation_spans_per_block: 16,
            },
        }
    }

    fn lane_defaults() -> [f32; PARAMETER_COUNT] {
        core::array::from_fn(|index| COMPRESSOR_PARAMETERS_V1[index].default_value)
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

    fn process_blocks(
        effect: &mut dyn PreparedNativeEffect,
        left: &mut [f32],
        right: &mut [f32],
        sidechain: Option<(&[f32], &[f32])>,
    ) -> ProcessReport {
        let mut total = ProcessReport::default();
        let mut offset = 0;
        while offset < left.len() {
            let end = (offset + 128).min(left.len());
            let sidechain =
                sidechain.map(|(left, right)| (&left[offset..end], &right[offset..end]));
            let report = effect.process(
                EffectProcessBlock::new(
                    &mut left[offset..end],
                    &mut right[offset..end],
                    sidechain,
                    offset as u64,
                    &[],
                    128,
                )
                .expect("bounded block"),
            );
            add_report(&mut total, report);
            offset = end;
        }
        total
    }

    fn snapshot(effect: &dyn PreparedNativeEffect) -> (Vec<u8>, Vec<u8>) {
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

    #[test]
    fn descriptor_rows_and_resource_envelope_are_frozen() {
        validate_descriptor_v1(&COMPRESSOR_DESCRIPTOR_V1).expect("descriptor");
        assert_eq!(COMPRESSOR_DESCRIPTOR_V1.id.as_str(), "miso.compressor");
        assert_eq!(COMPRESSOR_PARAMETERS_V1.len(), 8);
        for (quality, (rate, latency, lane_bytes, total_bytes)) in QUALITIES.iter().zip([
            (44_100, 882, 7_160, 14_320),
            (48_000, 960, 7_784, 15_568),
            (88_200, 1_764, 14_216, 28_432),
            (96_000, 1_920, 15_464, 30_928),
        ]) {
            let ring_length = latency as usize + 1;
            assert_eq!(quality.sample_rate, rate);
            assert_eq!(quality.latency, LatencySamples(latency));
            assert_eq!(quality.maximum_state.left_bytes, lane_bytes);
            assert_eq!(quality.maximum_state.right_bytes, lane_bytes);
            assert_eq!(quality.maximum_state.total(), Some(total_bytes));
            assert_eq!(quality.scratch_fixed_bytes, 64);
            assert_eq!(quality.scratch_bytes_per_frame, 0);
            assert_eq!(
                lane_bytes as usize,
                (STATE_HEADER_WORDS + 2 * ring_length) * 4
            );
            assert_eq!(
                total_bytes * 4 + 64 * 4,
                (u64::from(lane_bytes) * 2 + 64) * 4
            );
            assert_eq!(
                total_bytes * 8 + 64 * 8,
                (u64::from(lane_bytes) * 2 + 64) * 8
            );
        }
    }

    #[test]
    fn preparation_has_expected_metadata_and_one_byte_below_rejects() {
        let values = initial_values();
        let factory = CompressorFactory;
        let effect = factory.prepare(request(&values)).expect("prepare");
        assert_eq!(
            effect.metadata().latency,
            expected_prepared_metadata(&COMPRESSOR_DESCRIPTOR_V1, request(&values))
                .expect("metadata")
                .latency
        );
        let mut below = request(&values);
        below.limits.maximum_total_state_bytes -= 1;
        let error = match factory.prepare(below) {
            Ok(_) => panic!("one byte below must reject"),
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
    }

    #[test]
    fn lookahead_taps_are_derived_only_at_prepare_restore_and_full_reset() {
        let mut defaults = lane_defaults();
        for (lookahead_ms, expected_delay) in [(0.0, 960), (5.0, 720), (20.0, 0)] {
            defaults[7] = lookahead_ms;
            let mut lane = Lane::new(&defaults, 961, 48_000);
            assert_eq!(lane.detector_delay, expected_delay);
            let mut reset_defaults = defaults;
            reset_defaults[7] = 20.0 - lookahead_ms;
            lane.full_reset(&reset_defaults, 48_000);
            assert_eq!(
                lane.detector_delay,
                detector_delay(reset_defaults[7], 48_000, 961)
            );
        }
    }

    #[test]
    fn bank_fallback_never_hides_malformed_or_incompatible_requests() {
        let factory = CompressorFactory;
        let (backend, width) = if cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
            (KernelBackendV1::Aarch64Neon, BankWidth::Four)
        } else {
            (KernelBackendV1::X86Avx2, BankWidth::Eight)
        };
        let lanes = width.lanes() as usize;

        let mut malformed_values = vec![initial_values(); lanes];
        malformed_values[lanes - 1][0].value = f32::NAN;
        let malformed_requests = malformed_values
            .iter()
            .map(|values| request(values))
            .collect::<Vec<_>>();
        let error = match factory.bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &malformed_requests,
        }) {
            Ok(_) => panic!("unavailable backend must not hide malformed values"),
            Err(error) => error,
        };
        assert_eq!(error.code, "effect.parameter.initial");

        let connected_values = vec![initial_values(); lanes];
        let mut connected_requests = connected_values
            .iter()
            .map(|values| request(values))
            .collect::<Vec<_>>();
        for request in &mut connected_requests {
            request.ports.sidechain = PreparedSidechainPort::Connected {
                id: port_id("sidechain-in"),
                required: false,
            };
        }
        assert!(
            factory
                .bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend,
                    width,
                    requests: &connected_requests,
                })
                .expect("valid connected fallback")
                .is_none()
        );
        connected_requests[lanes - 1]
            .limits
            .maximum_total_state_bytes -= 1;
        let error = match factory.bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &connected_requests,
        }) {
            Ok(_) => panic!("connected fallback must validate every request"),
            Err(error) => error,
        };
        assert_eq!(error.code, "effect.resource.limit");

        let values = vec![initial_values(); lanes];
        let mut requests = values
            .iter()
            .map(|values| request(values))
            .collect::<Vec<_>>();
        requests[lanes - 1].bypass = true;
        assert!(
            factory
                .bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend,
                    width,
                    requests: &requests,
                })
                .expect("valid heterogeneous fallback")
                .is_none()
        );
    }

    #[test]
    fn bypass_preserves_exact_dry_bits_at_fixed_20ms_latency() {
        let values = initial_values();
        let mut preparation = request(&values);
        preparation.bypass = true;
        let mut effect = CompressorFactory.prepare(preparation).expect("prepare");
        let mut left = vec![0.0; 128];
        let mut right = vec![0.0; 128];
        left[0] = -0.75;
        right[0] = 0.25;
        let mut rendered_left = Vec::new();
        let mut rendered_right = Vec::new();
        for block_index in 0..8 {
            effect.process(
                EffectProcessBlock::new(
                    &mut left,
                    &mut right,
                    None,
                    (block_index * 128) as u64,
                    &[],
                    128,
                )
                .expect("block"),
            );
            rendered_left.extend_from_slice(&left);
            rendered_right.extend_from_slice(&right);
            left.fill(0.0);
            right.fill(0.0);
        }
        assert!(
            rendered_left[..960]
                .iter()
                .all(|sample| sample.to_bits() == 0)
        );
        assert!(
            rendered_right[..960]
                .iter()
                .all(|sample| sample.to_bits() == 0)
        );
        assert_eq!(rendered_left[960].to_bits(), (-0.75_f32).to_bits());
        assert_eq!(rendered_right[960].to_bits(), 0.25_f32.to_bits());
    }

    #[test]
    fn scalar_peak_processing_matches_the_independent_f64_oracle() {
        let mut values = initial_values();
        for lane in 0..2 {
            values[lane].value = -24.0;
            values[2 + lane].value = 8.0;
            values[4 + lane].value = 0.0;
            values[6 + lane].value = 1.0;
            values[8 + lane].value = 20.0;
            values[10 + lane].value = 0.0;
            values[12 + lane].value = 1.0;
            values[14 + lane].value = 0.0;
        }
        let mut effect = CompressorFactory
            .prepare(request(&values))
            .expect("prepare");
        let parameters = ReferenceCompressorParameters {
            threshold_db: -24.0,
            ratio: 8.0,
            knee_db: 0.0,
            attack_ms: 1.0,
            release_ms: 20.0,
            makeup_db: 0.0,
            mix: 1.0,
            lookahead_ms: 0.0,
        };
        let mut reference = ReferencePeakCompressor::new(48_000.0, parameters).expect("oracle");
        let input = (0..2_048)
            .map(|index| if index < 1_280 { 0.9_f32 } else { 0.1_f32 })
            .collect::<Vec<_>>();
        let expected = input
            .iter()
            .map(|sample| reference.process_sample(*sample as f64, *sample as f64) as f32)
            .collect::<Vec<_>>();
        let mut left = input.clone();
        let mut right = input;
        for (block_index, (left, right)) in
            left.chunks_mut(128).zip(right.chunks_mut(128)).enumerate()
        {
            effect.process(
                EffectProcessBlock::new(left, right, None, (block_index * 128) as u64, &[], 128)
                    .expect("block"),
            );
        }
        assert!(left[960..].iter().any(|sample| *sample != 0.0));
        assert!(
            left[1_200] < 0.9,
            "oracle test must exercise gain reduction"
        );
        for (actual, expected) in left.iter().zip(expected) {
            assert!(
                (actual - expected).abs() <= 2.0e-5,
                "{actual} != {expected}"
            );
        }
    }

    #[test]
    fn links_are_exact_and_connected_sidechain_is_distinct_from_main_detection() {
        assert_eq!(linked_levels(LinkMode::DualMono, -0.25, 0.75), (0.25, 0.75));
        assert_eq!(linked_levels(LinkMode::Maximum, -0.25, 0.75), (0.75, 0.75));
        assert_eq!(linked_levels(LinkMode::Average, -0.25, 0.75), (0.5, 0.5));

        let mut values = initial_values();
        for lane in 0..2 {
            values[lane].value = -40.0;
            values[2 + lane].value = 20.0;
            values[4 + lane].value = 0.0;
            values[6 + lane].value = 0.1;
            values[12 + lane].value = 1.0;
            values[14 + lane].value = 20.0;
        }
        let factory = CompressorFactory;
        let mut unconnected = factory.prepare(request(&values)).expect("unconnected");
        let mut connected_request = request(&values);
        connected_request.ports.sidechain = PreparedSidechainPort::Connected {
            id: port_id("sidechain-in"),
            required: false,
        };
        let mut connected = factory.prepare(connected_request).expect("connected");
        let mut unconnected_left = vec![0.25; 1_024];
        let mut unconnected_right = vec![0.25; 1_024];
        let mut connected_left = unconnected_left.clone();
        let mut connected_right = unconnected_right.clone();
        let sidechain_left = vec![0.0; 1_024];
        let sidechain_right = vec![0.0; 1_024];
        process_blocks(
            unconnected.as_mut(),
            &mut unconnected_left,
            &mut unconnected_right,
            None,
        );
        process_blocks(
            connected.as_mut(),
            &mut connected_left,
            &mut connected_right,
            Some((&sidechain_left, &sidechain_right)),
        );
        assert_eq!(connected_left[960].to_bits(), 0.25_f32.to_bits());
        assert_eq!(connected_right[960].to_bits(), 0.25_f32.to_bits());
        assert!(unconnected_left[960] < connected_left[960]);
        assert!(unconnected_right[960] < connected_right[960]);
    }

    #[test]
    fn block_point_reaches_target_on_its_sixty_fourth_update() {
        let values = initial_values();
        let mut effect = CompressorFactory
            .prepare(request(&values))
            .expect("prepare");
        let span = PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel: ParameterChannel::Left,
            parameter_index: 0,
            start_sample: 0,
            end_sample: 0,
            start_value: -80.0,
            end_value: -80.0,
        };
        let mut left = vec![0.0; 64];
        let mut right = vec![0.0; 64];
        effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[span], 128).expect("block"),
        );
        let sizes = effect.metadata().state_sizes;
        let mut left_state = vec![0_u8; sizes.left_bytes as usize];
        let mut right_state = vec![0_u8; sizes.right_bytes as usize];
        effect
            .snapshot_state_payload(
                StatePayloadOutput::new(&mut [], &mut left_state, &mut right_state, sizes)
                    .expect("payload"),
            )
            .expect("snapshot");
        assert_eq!(read_f32(&left_state, 3).to_bits(), (-80.0_f32).to_bits());
    }

    #[test]
    fn state_restore_is_transactional_and_sanitation_is_lane_local() {
        let values = initial_values();
        let mut effect = CompressorFactory
            .prepare(request(&values))
            .expect("prepare");
        let mut left = vec![0.5; 128];
        let mut right = vec![-0.25; 128];
        right[0] = f32::NAN;
        let report = effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("block"),
        );
        assert_eq!(report.sanitized_main_samples, 1);
        let sizes = effect.metadata().state_sizes;
        let mut left_state = vec![0_u8; sizes.left_bytes as usize];
        let mut right_state = vec![0_u8; sizes.right_bytes as usize];
        effect
            .snapshot_state_payload(
                StatePayloadOutput::new(&mut [], &mut left_state, &mut right_state, sizes)
                    .expect("payload"),
            )
            .expect("snapshot");
        assert_eq!(
            read_f32(&left_state, STATE_HEADER_WORDS).to_bits(),
            0.5_f32.to_bits()
        );
        assert_eq!(
            read_f32(&right_state, STATE_HEADER_WORDS).to_bits(),
            0.0_f32.to_bits()
        );
        let saved_left = left_state.clone();
        let saved_right = right_state.clone();
        let mut malformed_right = right_state.clone();
        malformed_right[..4].copy_from_slice(&u32::MAX.to_le_bytes());
        assert!(
            effect
                .restore_state_payload(
                    1,
                    StatePayloadInput::new(&[], &left_state, &malformed_right, sizes)
                        .expect("payload"),
                )
                .is_err()
        );
        effect
            .snapshot_state_payload(
                StatePayloadOutput::new(&mut [], &mut left_state, &mut right_state, sizes)
                    .expect("payload"),
            )
            .expect("snapshot");
        assert_eq!(left_state, saved_left);
        assert_eq!(right_state, saved_right);

        let positive_subnormal = f32::from_bits(1);
        write_f32(&mut left_state, 1, positive_subnormal);
        write_f32(&mut left_state, 9, positive_subnormal);
        write_f32(&mut left_state, 10, positive_subnormal);
        effect
            .restore_state_payload(
                1,
                StatePayloadInput::new(&[], &left_state, &right_state, sizes).expect("payload"),
            )
            .expect("every preparation-legal finite parameter state restores");
        let (restored_left, _) = snapshot(effect.as_ref());
        assert_eq!(
            read_f32(&restored_left, 1).to_bits(),
            positive_subnormal.to_bits()
        );
        assert_eq!(
            read_f32(&restored_left, 9).to_bits(),
            positive_subnormal.to_bits()
        );
        assert_eq!(
            read_f32(&restored_left, 10).to_bits(),
            positive_subnormal.to_bits()
        );

        effect.reset(ResetKind::DiscontinuityKeepParameters);
        let (discontinuity_left, _) = snapshot(effect.as_ref());
        assert_eq!(read_u32(&discontinuity_left, 0), 0);
        assert_eq!(
            read_f32(&discontinuity_left, 2).to_bits(),
            0.0_f32.to_bits()
        );
        assert_eq!(read_u32(&discontinuity_left, 11), 0);
        assert!(
            discontinuity_left[STATE_HEADER_WORDS * 4..]
                .chunks_exact(4)
                .all(|word| word == 0.0_f32.to_le_bytes())
        );

        effect.reset(ResetKind::FullToDefaults);
        let (default_left, _) = snapshot(effect.as_ref());
        assert_eq!(read_f32(&default_left, 1).to_bits(), 5.0_f32.to_bits());
        assert_eq!(read_f32(&default_left, 3).to_bits(), (-18.0_f32).to_bits());
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn unconnected_w8_bank_matches_scalar_and_preserves_per_track_state() {
        if PreparedCompressorGainMixKernelV1::try_new(KernelBackendV1::X86Avx2).is_err() {
            return;
        }
        let factory = CompressorFactory;
        let values = [initial_values(); 8];
        let requests = values
            .iter()
            .map(|values| request(values))
            .collect::<Vec<_>>();
        let mut bank = match factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: KernelBackendV1::X86Avx2,
                width: BankWidth::Eight,
                requests: &requests,
            })
            .expect("binding")
        {
            Some(bank) => bank,
            None => return,
        };
        let mut scalar = values
            .iter()
            .map(|values| factory.prepare(request(values)).expect("scalar"))
            .collect::<Vec<_>>();
        let offsets = [0_u32; 9];
        for block_index in 0..8 {
            let mut bank_left = vec![0.0_f32; 128 * 8];
            let mut bank_right = vec![0.0_f32; 128 * 8];
            for frame in 0..128 {
                for track in 0..8 {
                    let index = frame * 8 + track;
                    bank_left[index] = 0.9 - track as f32 * 0.025 + frame as f32 * 0.0001;
                    bank_right[index] = -0.6 + track as f32 * 0.015 - frame as f32 * 0.0001;
                }
            }
            let mut scalar_left: [Vec<f32>; 8] = core::array::from_fn(|track| {
                (0..128)
                    .map(|frame| bank_left[frame * 8 + track])
                    .collect::<Vec<_>>()
            });
            let mut scalar_right: [Vec<f32>; 8] = core::array::from_fn(|track| {
                (0..128)
                    .map(|frame| bank_right[frame * 8 + track])
                    .collect::<Vec<_>>()
            });
            for track in 0..8 {
                scalar[track].process(
                    EffectProcessBlock::new(
                        &mut scalar_left[track],
                        &mut scalar_right[track],
                        None,
                        (block_index * 128) as u64,
                        &[],
                        128,
                    )
                    .expect("scalar block"),
                );
            }
            let report = bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut bank_left,
                    &mut bank_right,
                    None,
                    128,
                    BankWidth::Eight,
                    (block_index * 128) as u64,
                    &[],
                    &offsets,
                    128,
                )
                .expect("bank block"),
            );
            assert_eq!(report, BankProcessReport::empty(BankWidth::Eight));
            for frame in 0..128 {
                for track in 0..8 {
                    assert_eq!(
                        bank_left[frame * 8 + track].to_bits(),
                        scalar_left[track][frame].to_bits(),
                        "left block={block_index} frame={frame} track={track}"
                    );
                    assert_eq!(
                        bank_right[frame * 8 + track].to_bits(),
                        scalar_right[track][frame].to_bits(),
                        "right block={block_index} frame={frame} track={track}"
                    );
                }
            }
        }
        let sizes = scalar[3].metadata().state_sizes;
        let mut scalar_left = vec![0_u8; sizes.left_bytes as usize];
        let mut scalar_right = vec![0_u8; sizes.right_bytes as usize];
        let mut bank_left = vec![0_u8; sizes.left_bytes as usize];
        let mut bank_right = vec![0_u8; sizes.right_bytes as usize];
        scalar[3]
            .snapshot_state_payload(
                StatePayloadOutput::new(&mut [], &mut scalar_left, &mut scalar_right, sizes)
                    .expect("scalar payload"),
            )
            .expect("scalar snapshot");
        bank.snapshot_track_state_payload(
            3,
            StatePayloadOutput::new(&mut [], &mut bank_left, &mut bank_right, sizes)
                .expect("bank payload"),
        )
        .expect("bank snapshot");
        assert_eq!(bank_left, scalar_left);
        assert_eq!(bank_right, scalar_right);
    }
}
