//! Fixed-four-phase true-peak safety limiter.
//!
//! The audible path remains at the host sample rate. The frozen Annex-2 FIR is detector-only.
#![allow(missing_docs)]

use miso_engine_effect_contract::{
    AutomationRate, AutomationSpanKind, EffectDescriptorV1, EffectPrepareError, EffectProcessBlock,
    EffectQuality, InitialParameterValue, LatencySamples, LinkMode, LinkModeSet,
    NativeEffectFactory, ParameterChannel, ParameterChannelPolicy, ParameterDescriptorV1,
    ParameterDomain, ParameterId, ParameterMapping, ParameterUnit, PortDescriptorV1, PortId,
    PortLayout, PortRole, PrepareEffectBankRequest, PrepareEffectRequest, PreparedAutomationSpan,
    PreparedNativeEffect, PreparedNativeEffectBank, ProcessReport, ResetKind, SmoothingRule,
    StatePayloadError, StatePayloadInput, StatePayloadOutput, StatePayloadSizes, TailSamples,
    expected_prepared_metadata, sanitize_sample,
};

const PARAMETER_COUNT: usize = 3;
const RAMP_COUNT: usize = 2;
const HISTORY_WORDS: usize = 12;
const STATE_HEADER_WORDS: usize = 23;
const FIR_ALIGNMENT_SAMPLES: u32 = 6;

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
        None => panic!("nonzero parameter identifier"),
    }
}

#[allow(clippy::too_many_arguments)]
const fn parameter(
    id: u32,
    name: &'static str,
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
        display_name: name,
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

/// Frozen descriptor rows. Descriptor position and stable numeric ID agree.
pub const TRUE_PEAK_LIMITER_PARAMETERS_V1: [ParameterDescriptorV1; PARAMETER_COUNT] = [
    parameter(
        1,
        "ceiling",
        "dBTP-est",
        ParameterUnit::Db,
        -24.0,
        0.0,
        -1.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        2,
        "release",
        "ms",
        ParameterUnit::Milliseconds,
        10.0,
        2000.0,
        100.0,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        3,
        "lookahead",
        "ms",
        ParameterUnit::Milliseconds,
        0.0,
        10.0,
        5.0,
        ParameterMapping::Linear,
        AutomationRate::None,
        SmoothingRule::None,
        0,
    ),
];

const PORTS: [PortDescriptorV1; 2] = [
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
];

const fn quality(rate: u32) -> miso_engine_effect_contract::QualityDescriptorV1 {
    let lookahead_maximum = rate / 100;
    let lane_words = 2 * lookahead_maximum + 31;
    let lane_bytes = lane_words * 4;
    miso_engine_effect_contract::QualityDescriptorV1 {
        quality: EffectQuality::Normal,
        sample_rate: rate,
        latency: LatencySamples((lookahead_maximum + 6) as u64),
        tail: TailSamples::Infinite,
        maximum_state: StatePayloadSizes {
            common_bytes: 0,
            left_bytes: lane_bytes,
            right_bytes: lane_bytes,
        },
        scratch_fixed_bytes: 24,
        scratch_bytes_per_frame: 0,
    }
}

const QUALITIES: [miso_engine_effect_contract::QualityDescriptorV1; 4] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
];

/// Immutable launch true-peak limiter descriptor.
pub const TRUE_PEAK_LIMITER_DESCRIPTOR_V1: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("miso.true-peak-limiter"),
    display_name: "True-Peak Limiter",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: match LinkModeSet::new(3) {
        Some(value) => value,
        None => panic!("frozen link bits"),
    },
    parameters: &TRUE_PEAK_LIMITER_PARAMETERS_V1,
    ports: &PORTS,
    qualities: &QUALITIES,
};

/// Factory for the fixed-latency scalar limiter.
#[derive(Clone, Copy, Debug, Default)]
pub struct TruePeakLimiterFactory;

/// The exact Annex-2 four-phase detector table, indexed by history tap then phase.
const ANNEX2_FIR: [[f32; 4]; HISTORY_WORDS] = [
    [
        0.001_708_984_4,
        -0.029_174_805,
        -0.018_920_898,
        -0.008_300_781,
    ],
    [0.010_986_328, 0.029_296_875, 0.033_081_055, 0.014_892_578],
    [-0.019_653_32, -0.051_757_812, -0.058_227_54, -0.026_611_328],
    [0.033_203_125, 0.089_111_33, 0.101_562_5, 0.047_607_422],
    [-0.059_448_242, -0.166_503_9, -0.200_317_38, -0.102_294_92],
    [0.137_329_1, 0.465_087_9, 0.779_785_16, 0.972_167_97],
    [0.972_167_97, 0.779_785_16, 0.465_087_9, 0.137_329_1],
    [-0.102_294_92, -0.200_317_38, -0.166_503_9, -0.059_448_242],
    [0.047_607_422, 0.101_562_5, 0.089_111_33, 0.033_203_125],
    [-0.026_611_328, -0.058_227_54, -0.051_757_812, -0.019_653_32],
    [0.014_892_578, 0.033_081_055, 0.029_296_875, 0.010_986_328],
    [
        -0.008_300_781,
        -0.018_920_898,
        -0.029_174_805,
        0.001_708_984_4,
    ],
];

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

#[derive(Debug)]
struct Lane {
    main_cursor: u32,
    required_cursor: u32,
    lookahead_ms: f32,
    lookahead_samples: usize,
    gain: f32,
    hold_remaining: u32,
    ramps: [Ramp; RAMP_COUNT],
    history: [f32; HISTORY_WORDS],
    main_ring: Box<[f32]>,
    required_ring: Box<[f32]>,
}

impl Lane {
    fn new(defaults: &[f32; PARAMETER_COUNT], sample_rate: u32) -> Option<Self> {
        let (maximum, main_length, required_length) = dimensions(sample_rate)?;
        let lookahead_samples = lookahead_samples(defaults[2], sample_rate, maximum)?;
        Some(Self {
            main_cursor: 0,
            required_cursor: 0,
            lookahead_ms: defaults[2],
            lookahead_samples,
            gain: 1.0,
            hold_remaining: 0,
            ramps: [Ramp::fixed(defaults[0]), Ramp::fixed(defaults[1])],
            history: [0.0; HISTORY_WORDS],
            main_ring: vec![0.0; main_length].into_boxed_slice(),
            required_ring: vec![1.0; required_length].into_boxed_slice(),
        })
    }

    fn full_reset(&mut self, defaults: &[f32; PARAMETER_COUNT], sample_rate: u32) {
        self.main_cursor = 0;
        self.required_cursor = 0;
        self.lookahead_ms = defaults[2];
        self.lookahead_samples =
            lookahead_samples(defaults[2], sample_rate, self.required_ring.len() - 1)
                .expect("validated prepared lookahead");
        self.gain = 1.0;
        self.hold_remaining = 0;
        self.ramps = [Ramp::fixed(defaults[0]), Ramp::fixed(defaults[1])];
        self.history.fill(0.0);
        self.main_ring.fill(0.0);
        self.required_ring.fill(1.0);
    }

    fn discontinuity_reset(&mut self) {
        self.main_cursor = 0;
        self.required_cursor = 0;
        self.gain = 1.0;
        self.hold_remaining = 0;
        for ramp in &mut self.ramps {
            ramp.current = ramp.target;
            ramp.remaining = 0;
        }
        self.history.fill(0.0);
        self.main_ring.fill(0.0);
        self.required_ring.fill(1.0);
    }
}

/// A fixed-shape, allocation-free scalar limiter instance.
#[derive(Debug)]
pub struct PreparedTruePeakLimiter {
    metadata: miso_engine_effect_contract::PreparedEffectMetadata,
    left_defaults: [f32; PARAMETER_COUNT],
    right_defaults: [f32; PARAMETER_COUNT],
    left: Lane,
    right: Lane,
}

impl NativeEffectFactory for TruePeakLimiterFactory {
    fn descriptor(&self) -> &'static EffectDescriptorV1 {
        &TRUE_PEAK_LIMITER_DESCRIPTOR_V1
    }

    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        let metadata = expected_prepared_metadata(self.descriptor(), request)?;
        let (left_defaults, right_defaults) = initial_defaults(request.initial_values)?;
        let left = Lane::new(&left_defaults, metadata.sample_rate).ok_or(EffectPrepareError {
            code: "effect.parameter.initial",
        })?;
        let right = Lane::new(&right_defaults, metadata.sample_rate).ok_or(EffectPrepareError {
            code: "effect.parameter.initial",
        })?;
        Ok(Box::new(PreparedTruePeakLimiter {
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
        for member in request.requests.iter().copied() {
            expected_prepared_metadata(self.descriptor(), member)?;
            initial_defaults(member.initial_values)?;
        }
        Ok(None)
    }
}

impl PreparedNativeEffect for PreparedTruePeakLimiter {
    fn metadata(&self) -> miso_engine_effect_contract::PreparedEffectMetadata {
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
        for index in 0..block.frames() {
            self.left.ramps.iter_mut().for_each(Ramp::advance);
            self.right.ramps.iter_mut().for_each(Ramp::advance);
            let left = sanitize(block.left[index], &mut report.sanitized_main_samples);
            let right = sanitize(block.right[index], &mut report.sanitized_main_samples);
            let left_peak = detector_peak(&mut self.left, left);
            let right_peak = detector_peak(&mut self.right, right);
            let (left_peak, right_peak) = match self.metadata.link_mode {
                LinkMode::DualMono => (left_peak, right_peak),
                LinkMode::Maximum => match (left_peak, right_peak) {
                    (Some(left_peak), Some(right_peak)) => {
                        let peak = canonical_max(left_peak, right_peak);
                        (Some(peak), Some(peak))
                    }
                    _ => (left_peak, right_peak),
                },
                LinkMode::Average => (left_peak, right_peak),
            };
            block.left[index] = process_lane(
                left,
                left_peak,
                &mut self.left,
                self.metadata.sample_rate,
                self.metadata.bypass,
                &mut report.recovered_left_samples,
            );
            block.right[index] = process_lane(
                right,
                right_peak,
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
            return Err(state_error("effect.state.version"));
        }
        validate_state_lengths(
            input.common.len(),
            input.left.len(),
            input.right.len(),
            self.metadata.state_sizes,
        )?;
        let left = read_lane(input.left, self.metadata.sample_rate)?;
        let right = read_lane(input.right, self.metadata.sample_rate)?;
        self.left = left;
        self.right = right;
        Ok(())
    }
}

fn dimensions(sample_rate: u32) -> Option<(usize, usize, usize)> {
    let maximum = usize::try_from(sample_rate / 100).ok()?;
    let main_length = maximum.checked_add(7)?;
    let required_length = maximum.checked_add(1)?;
    Some((maximum, main_length, required_length))
}

fn lookahead_samples(value: f32, sample_rate: u32, maximum: usize) -> Option<usize> {
    let samples = (value as f64 * sample_rate as f64 / 1000.0 + 0.5).floor();
    if !samples.is_finite() || samples < 0.0 {
        return None;
    }
    usize::try_from(samples as u64)
        .ok()
        .map(|value| value.min(maximum))
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
    for (index, parameter) in TRUE_PEAK_LIMITER_PARAMETERS_V1.iter().enumerate() {
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

fn canonical_max(left: f32, right: f32) -> f32 {
    if right > left { right } else { left }
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

#[allow(clippy::assign_op_pattern, clippy::needless_range_loop)]
fn detector_peak(lane: &mut Lane, input: f32) -> Option<f32> {
    lane.history.copy_within(0..HISTORY_WORDS - 1, 1);
    lane.history[0] = input;
    let mut peak = input.abs();
    for phase in 0..4 {
        let mut value = 0.0_f32;
        for (tap, sample) in lane.history.iter().enumerate() {
            value = value + ANNEX2_FIR[tap][phase] * *sample;
        }
        let value = finite_or_zero(value)?;
        peak = canonical_max(peak, value.abs());
    }
    finite_or_zero(peak)
}

fn process_lane(
    input: f32,
    peak: Option<f32>,
    lane: &mut Lane,
    sample_rate: u32,
    bypass: bool,
    recovered: &mut u64,
) -> f32 {
    let required = peak.and_then(|peak| required_gain(peak, lane.ramps[0].current));
    let gain = required.and_then(|required| advance_gain(lane, required, sample_rate));
    let delayed = delay_main(lane, input);
    let Some(gain) = gain else {
        recover(lane, recovered);
        return if bypass { delayed } else { 0.0 };
    };
    if bypass || gain == 1.0 {
        return delayed;
    }
    match finite_or_zero(delayed * gain) {
        Some(output) => output,
        None => {
            recover(lane, recovered);
            0.0
        }
    }
}

fn required_gain(peak: f32, ceiling_db: f32) -> Option<f32> {
    let limit = finite_or_zero(10.0_f32.powf((ceiling_db - 1.0) * 0.05))?;
    let required = if peak == 0.0 || peak <= limit {
        1.0
    } else {
        (limit / peak).clamp(0.0, 1.0)
    };
    finite_or_zero(required)
}

fn advance_gain(lane: &mut Lane, required: f32, sample_rate: u32) -> Option<f32> {
    let length = lane.required_ring.len();
    let cursor = lane.required_cursor as usize;
    lane.required_ring[cursor] = required;
    let read = (cursor + 1 + lane.lookahead_samples) % length;
    let delayed_required = lane.required_ring[read];
    lane.required_cursor = ((cursor + 1) % length) as u32;
    let release =
        finite_or_zero((-1.0_f32 / (0.001 * lane.ramps[1].current * sample_rate as f32)).exp())?;
    let hold_samples = u32::try_from(lane.lookahead_samples)
        .ok()?
        .checked_add(FIR_ALIGNMENT_SAMPLES)?;
    let gain = if delayed_required < lane.gain
        || (delayed_required == lane.gain && delayed_required < 1.0)
    {
        lane.hold_remaining = hold_samples;
        delayed_required
    } else if lane.hold_remaining != 0 {
        lane.hold_remaining -= 1;
        lane.gain
    } else {
        release * lane.gain + (1.0 - release) * delayed_required
    };
    let gain = finite_or_zero(gain)?.clamp(0.0, 1.0);
    lane.gain = gain;
    Some(gain)
}

fn delay_main(lane: &mut Lane, input: f32) -> f32 {
    let cursor = lane.main_cursor as usize;
    lane.main_ring[cursor] = input;
    let delayed = lane.main_ring[(cursor + 1) % lane.main_ring.len()];
    lane.main_cursor = ((cursor + 1) % lane.main_ring.len()) as u32;
    delayed
}

fn recover(lane: &mut Lane, recovered: &mut u64) {
    lane.gain = 0.0;
    lane.hold_remaining = 0;
    lane.required_ring.fill(0.0);
    *recovered = recovered.saturating_add(1);
}

fn apply_automation(
    spans: &[PreparedAutomationSpan],
    metadata: miso_engine_effect_contract::PreparedEffectMetadata,
    first_sample: u64,
    left: &mut Lane,
    right: &mut Lane,
    report: &mut ProcessReport,
) {
    let mut pending = [[None; RAMP_COUNT]; 2];
    let mut last_order = None;
    for (span_index, span) in spans.iter().enumerate() {
        let lane = match span.channel {
            ParameterChannel::Left => 0,
            ParameterChannel::Right => 1,
            ParameterChannel::Both => {
                report.invalid_spans = report.invalid_spans.saturating_add(1);
                continue;
            }
        };
        let parameter = span.parameter_index as usize;
        let Some(order) = span
            .parameter_index
            .checked_mul(2)
            .and_then(|value| value.checked_add(lane as u32))
        else {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        };
        let valid = span_index < metadata.automation_capacity as usize
            && parameter < RAMP_COUNT
            && span.kind == AutomationSpanKind::Point
            && span.start_sample == first_sample
            && span.end_sample == first_sample
            && span.start_value.to_bits() == span.end_value.to_bits()
            && parameter_value_valid(
                &TRUE_PEAK_LIMITER_PARAMETERS_V1[parameter],
                span.start_value,
            )
            && !negative_zero(span.start_value)
            && last_order.is_none_or(|previous| order > previous)
            && pending[lane][parameter].is_none();
        if !valid {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        }
        last_order = Some(order);
        pending[lane][parameter] = Some(normalize_zero(span.start_value));
    }
    for (parameter, (left_ramp, right_ramp)) in left
        .ramps
        .iter_mut()
        .zip(right.ramps.iter_mut())
        .enumerate()
    {
        if let Some(value) = pending[0][parameter] {
            left_ramp.target = value;
            left_ramp.remaining = 64;
        }
        if let Some(value) = pending[1][parameter] {
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
    write_u32(bytes, 0, lane.main_cursor);
    write_u32(bytes, 1, lane.required_cursor);
    write_f32(bytes, 2, lane.lookahead_ms);
    write_f32(bytes, 3, lane.gain);
    write_u32(bytes, 4, lane.hold_remaining);
    for (index, ramp) in lane.ramps.iter().enumerate() {
        let word = 5 + index * 3;
        write_f32(bytes, word, ramp.current);
        write_f32(bytes, word + 1, ramp.target);
        write_u32(bytes, word + 2, ramp.remaining);
    }
    for (index, value) in lane.history.iter().enumerate() {
        write_f32(bytes, 11 + index, *value);
    }
    for (index, value) in lane.main_ring.iter().enumerate() {
        write_f32(bytes, STATE_HEADER_WORDS + index, *value);
    }
    for (index, value) in lane.required_ring.iter().enumerate() {
        write_f32(
            bytes,
            STATE_HEADER_WORDS + lane.main_ring.len() + index,
            *value,
        );
    }
}

fn read_lane(bytes: &[u8], sample_rate: u32) -> Result<Lane, StatePayloadError> {
    let (maximum, main_length, required_length) =
        dimensions(sample_rate).ok_or(state_error("effect.state.length"))?;
    let expected = (STATE_HEADER_WORDS + main_length + required_length)
        .checked_mul(4)
        .ok_or(state_error("effect.state.length"))?;
    if bytes.len() != expected {
        return Err(state_error("effect.state.length"));
    }
    let main_cursor = read_u32(bytes, 0);
    let required_cursor = read_u32(bytes, 1);
    if main_cursor as usize >= main_length || required_cursor as usize >= required_length {
        return Err(state_error("effect.state.cursor"));
    }
    let lookahead_ms = read_f32(bytes, 2);
    let gain = read_f32(bytes, 3);
    let hold_remaining = read_u32(bytes, 4);
    if !parameter_state_valid(2, lookahead_ms)
        || !normal_or_zero(gain)
        || !(0.0..=1.0).contains(&gain)
    {
        return Err(state_error("effect.state.parameter"));
    }
    let mut defaults = [0.0; PARAMETER_COUNT];
    defaults[2] = lookahead_ms;
    let mut ramps = [Ramp::fixed(0.0); RAMP_COUNT];
    for (index, ramp) in ramps.iter_mut().enumerate() {
        let word = 5 + index * 3;
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
    let mut lane =
        Lane::new(&defaults, sample_rate).ok_or(state_error("effect.state.parameter"))?;
    if lane.lookahead_samples > maximum {
        return Err(state_error("effect.state.parameter"));
    }
    let maximum_hold = u32::try_from(lane.lookahead_samples)
        .ok()
        .and_then(|lookahead| lookahead.checked_add(FIR_ALIGNMENT_SAMPLES))
        .ok_or(state_error("effect.state.parameter"))?;
    if hold_remaining > maximum_hold {
        return Err(state_error("effect.state.parameter"));
    }
    lane.main_cursor = main_cursor;
    lane.required_cursor = required_cursor;
    lane.gain = normalize_zero(gain);
    lane.hold_remaining = hold_remaining;
    lane.ramps = ramps;
    for (index, value) in lane.history.iter_mut().enumerate() {
        *value = read_f32(bytes, 11 + index);
        if !normal_or_zero(*value) {
            return Err(state_error("effect.state.history"));
        }
    }
    for (index, value) in lane.main_ring.iter_mut().enumerate() {
        *value = read_f32(bytes, STATE_HEADER_WORDS + index);
        if !normal_or_zero(*value) {
            return Err(state_error("effect.state.ring"));
        }
    }
    for (index, value) in lane.required_ring.iter_mut().enumerate() {
        *value = read_f32(bytes, STATE_HEADER_WORDS + main_length + index);
        if !normal_or_zero(*value) || !(0.0..=1.0).contains(value) {
            return Err(state_error("effect.state.gain"));
        }
    }
    Ok(lane)
}

fn normal_or_zero(value: f32) -> bool {
    value.is_finite() && (value == 0.0 || value.is_normal())
}

fn parameter_state_valid(index: usize, value: f32) -> bool {
    !negative_zero(value) && parameter_value_valid(&TRUE_PEAK_LIMITER_PARAMETERS_V1[index], value)
}

const fn state_error(code: &'static str) -> StatePayloadError {
    StatePayloadError { code }
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

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_effect_contract::{
        EffectProcessBlock, PrepareEffectLimits, PreparedPortsV1, PreparedSidechainPort,
        StatePayloadInput, StatePayloadOutput, validate_descriptor_v1,
    };

    fn initial_values() -> [InitialParameterValue; PARAMETER_COUNT * 2] {
        core::array::from_fn(|index| InitialParameterValue {
            parameter_index: (index / 2) as u32,
            channel: if index % 2 == 0 {
                ParameterChannel::Left
            } else {
                ParameterChannel::Right
            },
            value: TRUE_PEAK_LIMITER_PARAMETERS_V1[index / 2].default_value,
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
                sidechain: PreparedSidechainPort::None,
            },
            initial_values: values,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: quality.maximum_state.total().expect("state total"),
                maximum_scratch_bytes: 24,
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

    fn render_blocks(
        effect: &mut dyn PreparedNativeEffect,
        left: &mut [f32],
        right: &mut [f32],
    ) -> ProcessReport {
        let mut report = ProcessReport::default();
        for (block, (left, right)) in left.chunks_mut(128).zip(right.chunks_mut(128)).enumerate() {
            let next = effect.process(
                EffectProcessBlock::new(left, right, None, (block * 128) as u64, &[], 128)
                    .expect("block"),
            );
            report.sanitized_main_samples = report
                .sanitized_main_samples
                .saturating_add(next.sanitized_main_samples);
            report.recovered_left_samples = report
                .recovered_left_samples
                .saturating_add(next.recovered_left_samples);
            report.recovered_right_samples = report
                .recovered_right_samples
                .saturating_add(next.recovered_right_samples);
        }
        report
    }

    #[test]
    fn descriptor_metadata_and_exact_resource_rows_are_frozen() {
        validate_descriptor_v1(&TRUE_PEAK_LIMITER_DESCRIPTOR_V1).expect("descriptor");
        assert_eq!(
            TRUE_PEAK_LIMITER_DESCRIPTOR_V1.id.as_str(),
            "miso.true-peak-limiter"
        );
        assert_eq!(
            TRUE_PEAK_LIMITER_DESCRIPTOR_V1.supported_link_modes.bits(),
            3
        );
        for (quality, expected) in QUALITIES.iter().zip([
            (44_100, 447, 3_652, 7_304),
            (48_000, 486, 3_964, 7_928),
            (88_200, 888, 7_180, 14_360),
            (96_000, 966, 7_804, 15_608),
        ]) {
            assert_eq!(quality.sample_rate, expected.0);
            assert_eq!(quality.latency, LatencySamples(expected.1));
            assert_eq!(quality.maximum_state.left_bytes, expected.2);
            assert_eq!(quality.maximum_state.total(), Some(expected.3));
            assert_eq!(quality.scratch_fixed_bytes, 24);
        }
    }

    #[test]
    fn phase_table_and_causal_history_are_exact() {
        let defaults = [-1.0, 100.0, 5.0];
        let mut lane = Lane::new(&defaults, 48_000).expect("lane");
        let peak = detector_peak(&mut lane, 1.0).expect("peak");
        assert_eq!(lane.history[0].to_bits(), 1.0_f32.to_bits());
        assert!(lane.history[1..].iter().all(|value| value.to_bits() == 0));
        for phase_value in ANNEX2_FIR[0] {
            assert_eq!(finite_or_zero(phase_value), Some(phase_value));
        }
        assert_eq!(peak.to_bits(), 1.0_f32.to_bits());
        let _ = detector_peak(&mut lane, -0.5).expect("second peak");
        assert_eq!(lane.history[0].to_bits(), (-0.5_f32).to_bits());
        assert_eq!(lane.history[1].to_bits(), 1.0_f32.to_bits());
    }

    #[test]
    fn fixed_latency_guarded_ceiling_and_bypass_bits_hold() {
        let guard_limit = 10.0_f32.powf((-6.0 - 1.0) * 0.05);
        for lookahead in [0.0, 5.0, 10.0] {
            let mut values = initial_values();
            values[0].value = -6.0;
            values[1].value = -6.0;
            values[4].value = lookahead;
            values[5].value = lookahead;
            let mut effect = TruePeakLimiterFactory
                .prepare(request(&values))
                .expect("prepare");
            let mut left = vec![0.0; 487];
            let mut right = vec![0.0; 487];
            left[0] = 1.0;
            right[0] = 0.5;
            assert_eq!(
                render_blocks(effect.as_mut(), &mut left, &mut right),
                ProcessReport::default()
            );
            assert!(left[..486].iter().all(|sample| sample.to_bits() == 0));
            assert!(left[486].abs() <= guard_limit, "left lookahead {lookahead}");
            assert!(
                right[486].abs() <= guard_limit,
                "right lookahead {lookahead}"
            );
            let state = snapshot(effect.as_ref());
            assert_eq!(read_u32(&state.0, 4), 0, "left hold {lookahead}");
            assert_eq!(read_u32(&state.1, 4), 0, "right hold {lookahead}");
        }

        let mut values = initial_values();
        values[0].value = -6.0;
        values[1].value = -6.0;
        values[4].value = 10.0;
        values[5].value = 10.0;
        let mut bypass_request = request(&values);
        bypass_request.bypass = true;
        let mut bypass = TruePeakLimiterFactory
            .prepare(bypass_request)
            .expect("bypass prepare");
        let mut left = vec![0.0; 487];
        let mut right = vec![0.0; 487];
        left[0] = -0.0;
        right[0] = 0.25;
        assert_eq!(
            render_blocks(bypass.as_mut(), &mut left, &mut right),
            ProcessReport::default()
        );
        assert_eq!(left[486].to_bits(), (-0.0_f32).to_bits());
        assert_eq!(right[486].to_bits(), 0.25_f32.to_bits());
    }

    #[test]
    fn required_gain_hold_defers_release_for_exact_lookahead_plus_alignment() {
        const EVENT_GAIN: f32 = 0.25;
        let (_, main_length, required_length) = dimensions(48_000).expect("dimensions");
        for lookahead_ms in [0.0, 5.0, 10.0] {
            let defaults = [-6.0, 10.0, lookahead_ms];
            let mut lane = Lane::new(&defaults, 48_000).expect("lane");
            let delay = 480 - lane.lookahead_samples;
            for index in 0..=delay {
                let raw = if index == 0 { EVENT_GAIN } else { 1.0 };
                let gain = advance_gain(&mut lane, raw, 48_000).expect("advance to event");
                if index < delay {
                    assert_eq!(gain.to_bits(), 1.0_f32.to_bits());
                }
            }
            let hold =
                u32::try_from(lane.lookahead_samples).expect("lookahead") + FIR_ALIGNMENT_SAMPLES;
            assert_eq!(lane.gain.to_bits(), EVENT_GAIN.to_bits());
            assert_eq!(lane.hold_remaining, hold);

            let mut payload = vec![0; (STATE_HEADER_WORDS + main_length + required_length) * 4];
            write_lane(&mut payload, &lane);
            let mut restored = read_lane(&payload, 48_000).expect("active hold restore");
            assert_eq!(restored.hold_remaining, hold);
            let mut corrupt = payload.clone();
            write_u32(&mut corrupt, 4, hold + 1);
            assert!(read_lane(&corrupt, 48_000).is_err());

            for expected_remaining in (0..hold).rev() {
                let gain = advance_gain(&mut restored, 1.0, 48_000).expect("held gain");
                assert_eq!(gain.to_bits(), EVENT_GAIN.to_bits());
                assert_eq!(restored.hold_remaining, expected_remaining);
            }
            let released = advance_gain(&mut restored, 1.0, 48_000).expect("first release");
            assert!(released > EVENT_GAIN);
            assert_eq!(restored.hold_remaining, 0);

            lane.discontinuity_reset();
            assert_eq!(lane.hold_remaining, 0);
        }
    }

    #[test]
    fn automation_state_restore_and_sanitation_are_transactional() {
        let values = initial_values();
        let mut effect = TruePeakLimiterFactory
            .prepare(request(&values))
            .expect("prepare");
        let span = PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel: ParameterChannel::Left,
            parameter_index: 0,
            start_sample: 0,
            end_sample: 0,
            start_value: -12.0,
            end_value: -12.0,
        };
        let mut left = vec![0.0; 64];
        let mut right = vec![0.0; 64];
        right[0] = f32::NAN;
        let report = effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[span], 128).expect("block"),
        );
        assert_eq!(report.sanitized_main_samples, 1);
        let saved = snapshot(effect.as_ref());
        assert_eq!(read_f32(&saved.0, 5).to_bits(), (-12.0_f32).to_bits());
        assert_eq!(read_u32(&saved.0, 7), 0);
        let mut peer = TruePeakLimiterFactory
            .prepare(request(&values))
            .expect("peer");
        let sizes = peer.metadata().state_sizes;
        peer.restore_state_payload(
            1,
            StatePayloadInput::new(&[], &saved.0, &saved.1, sizes).expect("state"),
        )
        .expect("restore");
        let mut corrupt = saved.1.clone();
        write_u32(&mut corrupt, 0, u32::MAX);
        assert!(
            peer.restore_state_payload(
                1,
                StatePayloadInput::new(&[], &saved.0, &corrupt, sizes).expect("corrupt state"),
            )
            .is_err()
        );
        assert_eq!(snapshot(peer.as_ref()), saved);
        peer.reset(ResetKind::DiscontinuityKeepParameters);
        let discontinuity = snapshot(peer.as_ref());
        assert_eq!(read_u32(&discontinuity.0, 0), 0);
        assert_eq!(read_u32(&discontinuity.0, 1), 0);
        assert_eq!(read_f32(&discontinuity.0, 3).to_bits(), 1.0_f32.to_bits());
        assert_eq!(read_u32(&discontinuity.0, 4), 0);
        assert_eq!(
            read_f32(&discontinuity.0, 5).to_bits(),
            (-12.0_f32).to_bits()
        );
        assert_eq!(
            read_f32(&discontinuity.0, 6).to_bits(),
            (-12.0_f32).to_bits()
        );
    }
}
