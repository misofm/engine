//! Launch feed-forward peak compressor descriptor and factory scaffold.
//!
//! Scalar processing follows in its own bounded implementation edit.
#![allow(missing_docs)]

use miso_engine_effect_contract::{
    AutomationRate, AutomationSpanKind, EffectDescriptorV1, EffectPrepareError, EffectProcessBlock,
    EffectQuality, InitialParameterValue, LatencySamples, LinkMode, LinkModeSet,
    NativeEffectFactory, ParameterChannel, ParameterChannelPolicy, ParameterDescriptorV1,
    ParameterDomain, ParameterId, ParameterMapping, ParameterUnit, PortDescriptorV1, PortId,
    PortLayout, PortRole, PrepareEffectBankRequest, PrepareEffectRequest, PreparedAutomationSpan,
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
    gain_reduction_db: f32,
    ramps: [Ramp; RAMP_COUNT],
    main_ring: Box<[f32]>,
    detector_ring: Box<[f32]>,
}

impl Lane {
    fn new(defaults: &[f32; PARAMETER_COUNT], ring_length: usize) -> Self {
        Self {
            cursor: 0,
            lookahead_ms: defaults[7],
            gain_reduction_db: 0.0,
            ramps: core::array::from_fn(|index| Ramp::fixed(defaults[index])),
            main_ring: vec![0.0; ring_length].into_boxed_slice(),
            detector_ring: vec![0.0; ring_length].into_boxed_slice(),
        }
    }

    fn full_reset(&mut self, defaults: &[f32; PARAMETER_COUNT]) {
        self.cursor = 0;
        self.lookahead_ms = defaults[7];
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

    fn detector_delay(&self, sample_rate: u32) -> usize {
        let latency = self.main_ring.len() - 1;
        let lookahead =
            ((self.lookahead_ms as f64 * sample_rate as f64 / 1000.0) + 0.5).floor() as usize;
        latency - lookahead.min(latency)
    }
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
            left: Lane::new(&left_defaults, ring_length),
            right: Lane::new(&right_defaults, ring_length),
        }))
    }

    fn bind_homogeneous_bank(
        &self,
        _request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
        Ok(None)
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
                self.left.full_reset(&self.left_defaults);
                self.right.full_reset(&self.right_defaults);
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
        let left = read_lane(input.left, ring_length)?;
        let right = read_lane(input.right, ring_length)?;
        self.left = left;
        self.right = right;
        Ok(())
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
    let ring_length = lane.main_ring.len();
    let cursor = lane.cursor as usize;
    lane.main_ring[cursor] = main;
    lane.detector_ring[cursor] = detector;
    let delayed = lane.main_ring[(cursor + 1) % ring_length];
    let detector_delay = lane.detector_delay(sample_rate);
    let detector = lane.detector_ring[(cursor + ring_length - detector_delay) % ring_length];
    lane.cursor = ((cursor + 1) % ring_length) as u32;

    let threshold = lane.ramps[0].current;
    let ratio = lane.ramps[1].current;
    let knee = lane.ramps[2].current;
    let attack_ms = lane.ramps[3].current;
    let release_ms = lane.ramps[4].current;
    let makeup_db = lane.ramps[5].current;
    let mix = lane.ramps[6].current;
    let Some(target) = gain_reduction_target(detector, threshold, ratio, knee) else {
        return recover(lane, delayed, recovered);
    };
    let Some(attack) = flushed((-1.0_f32 / (0.001_f32 * attack_ms * sample_rate as f32)).exp())
    else {
        return recover(lane, delayed, recovered);
    };
    let Some(release) = flushed((-1.0_f32 / (0.001_f32 * release_ms * sample_rate as f32)).exp())
    else {
        return recover(lane, delayed, recovered);
    };
    let coefficient = if target < lane.gain_reduction_db {
        attack
    } else {
        release
    };
    let p0 = coefficient * lane.gain_reduction_db;
    let p1 = (1.0_f32 - coefficient) * target;
    let Some(gain_reduction_db) = flushed(p0 + p1) else {
        return recover(lane, delayed, recovered);
    };
    lane.gain_reduction_db = gain_reduction_db;
    let Some(gain) = flushed(10.0_f32.powf((gain_reduction_db + makeup_db) * 0.05_f32)) else {
        return recover(lane, delayed, recovered);
    };
    if bypass
        || mix == 0.0
        || (gain_reduction_db == 0.0 && makeup_db.to_bits() == 0.0_f32.to_bits())
    {
        return delayed;
    }
    let Some(wet) = flushed(delayed * gain) else {
        return recover(lane, delayed, recovered);
    };
    if mix.to_bits() == 1.0_f32.to_bits() {
        return wet;
    }
    let delta = wet - delayed;
    match flushed(delayed + mix * delta) {
        Some(value) => value,
        None => recover(lane, delayed, recovered),
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

fn read_lane(bytes: &[u8], ring_length: usize) -> Result<Lane, StatePayloadError> {
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
    !negative_zero(value)
        && normal_or_zero(value)
        && parameter_value_valid(&COMPRESSOR_PARAMETERS_V1[index], value)
}

const fn state_error(code: &'static str) -> StatePayloadError {
    StatePayloadError { code }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_dsp_reference::{ReferenceCompressorParameters, ReferencePeakCompressor};
    use miso_engine_effect_contract::{
        EffectProcessBlock, PrepareEffectLimits, PreparedPortsV1, PreparedSidechainPort,
        StatePayloadInput, StatePayloadOutput, validate_descriptor_v1,
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

    #[test]
    fn descriptor_rows_and_resource_envelope_are_frozen() {
        validate_descriptor_v1(&COMPRESSOR_DESCRIPTOR_V1).expect("descriptor");
        assert_eq!(COMPRESSOR_DESCRIPTOR_V1.id.as_str(), "miso.compressor");
        assert_eq!(COMPRESSOR_PARAMETERS_V1.len(), 8);
        assert_eq!(QUALITIES[0].maximum_state.left_bytes, 7_160);
        assert_eq!(QUALITIES[1].latency, LatencySamples(960));
        assert_eq!(QUALITIES[1].maximum_state.left_bytes, 7_784);
        assert_eq!(QUALITIES[3].maximum_state.left_bytes, 15_464);
        assert_eq!(QUALITIES[3].maximum_state.total(), Some(30_928));
        assert_eq!(QUALITIES[1].scratch_fixed_bytes, 64);
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
        let input = (0..128)
            .map(|index| if index < 96 { 0.9_f32 } else { 0.1_f32 })
            .collect::<Vec<_>>();
        let expected = input
            .iter()
            .map(|sample| reference.process_sample(*sample as f64, *sample as f64) as f32)
            .collect::<Vec<_>>();
        let mut left = input.clone();
        let mut right = input;
        effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("block"),
        );
        for (actual, expected) in left.iter().zip(expected) {
            assert!(
                (actual - expected).abs() <= 2.0e-5,
                "{actual} != {expected}"
            );
        }
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
        let report = effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("block"),
        );
        assert_eq!(report.sanitized_main_samples, 0);
        let sizes = effect.metadata().state_sizes;
        let mut left_state = vec![0_u8; sizes.left_bytes as usize];
        let mut right_state = vec![0_u8; sizes.right_bytes as usize];
        effect
            .snapshot_state_payload(
                StatePayloadOutput::new(&mut [], &mut left_state, &mut right_state, sizes)
                    .expect("payload"),
            )
            .expect("snapshot");
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
    }
}
