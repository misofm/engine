//! Fixed-two-times cubic soft clipper with a private scalar oversampling lane.
#![allow(missing_docs)]

use miso_engine_effect_contract::{
    AutomationRate, AutomationSpanKind, EffectDescriptorV1, EffectPrepareError, EffectProcessBlock,
    EffectQuality, InitialParameterValue, LatencySamples, LinkModeSet, NativeEffectFactory,
    ParameterChannel, ParameterChannelPolicy, ParameterDescriptorV1, ParameterDomain, ParameterId,
    ParameterMapping, ParameterUnit, PortDescriptorV1, PortId, PortLayout, PortRole,
    PrepareEffectBankRequest, PrepareEffectRequest, PreparedAutomationSpan, PreparedEffectMetadata,
    PreparedNativeEffect, PreparedNativeEffectBank, ProcessReport, ResetKind, SmoothingRule,
    StatePayloadError, StatePayloadInput, StatePayloadOutput, StatePayloadSizes, TailSamples,
    expected_prepared_metadata, sanitize_sample,
};

const PARAMETER_COUNT: usize = 3;
const STATE_WORDS: usize = 169;
const LANE_STATE_BYTES: u32 = (STATE_WORDS * 4) as u32;
const HISTORY: usize = 63;
const DRY_HISTORY: usize = 32;
const RAMP_SAMPLES: u32 = 64;
const TAPS: [usize; 31] = [
    2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 31, 32, 34, 36, 38, 40, 42, 44, 46, 48,
    50, 52, 54, 56, 58, 60,
];

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
        mapping: ParameterMapping::Linear,
        automation_rate: AutomationRate::Block,
        channel_policy: ParameterChannelPolicy::PerLane,
        smoothing: SmoothingRule::Linear,
        smoothing_samples: RAMP_SAMPLES,
        readable: true,
        automatable: true,
        enum_choices: &[],
    }
}

/// Frozen scalar soft-clip parameter rows, in stable numeric-ID order.
pub const SOFT_CLIP_PARAMETERS_V1: [ParameterDescriptorV1; PARAMETER_COUNT] = [
    parameter(1, "drive", "dB", ParameterUnit::Db, -24.0, 36.0, 0.0),
    parameter(2, "output", "dB", ParameterUnit::Db, -24.0, 24.0, 0.0),
    parameter(3, "mix", "linear", ParameterUnit::Linear, 0.0, 1.0, 1.0),
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
    miso_engine_effect_contract::QualityDescriptorV1 {
        quality: EffectQuality::Normal,
        sample_rate: rate,
        latency: LatencySamples(31),
        tail: TailSamples::Finite(31),
        maximum_state: StatePayloadSizes {
            common_bytes: 0,
            left_bytes: LANE_STATE_BYTES,
            right_bytes: LANE_STATE_BYTES,
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

/// Immutable descriptor for the frozen cubic soft-clip contract.
pub const SOFT_CLIP_DESCRIPTOR_V1: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("miso.soft-clip"),
    display_name: "Cubic Soft Clip",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::DUAL_MONO,
    parameters: &SOFT_CLIP_PARAMETERS_V1,
    ports: &PORTS,
    qualities: &QUALITIES,
};

/// Factory for the fixed-latency scalar launch realization.
#[derive(Clone, Copy, Debug, Default)]
pub struct SoftClipFactory;

/// The frozen symmetric 63-tap f32 halfband table, held in direct index order.
const H: [f32; HISTORY] = [
    0.0,
    0.0,
    4.117_896_6e-5,
    0.0,
    -1.843_658_7e-4,
    0.0,
    4.762_265_3e-4,
    0.0,
    -9.890_399e-4,
    0.0,
    1.823_257_9e-3,
    0.0,
    -3.110_171_5e-3,
    0.0,
    5.017_224_7e-3,
    0.0,
    -7.761_148e-3,
    0.0,
    1.163_983_6e-2,
    0.0,
    -1.710_855_8e-2,
    0.0,
    2.496_97e-2,
    0.0,
    -3.690_095e-2,
    0.0,
    5.726_341e-2,
    0.0,
    -1.021_490_2e-1,
    0.0,
    3.169_724_3e-1,
    5.0e-1,
    3.169_724_3e-1,
    0.0,
    -1.021_490_2e-1,
    0.0,
    5.726_341e-2,
    0.0,
    -3.690_095e-2,
    0.0,
    2.496_97e-2,
    0.0,
    -1.710_855_8e-2,
    0.0,
    1.163_983_6e-2,
    0.0,
    -7.761_148e-3,
    0.0,
    5.017_224_7e-3,
    0.0,
    -3.110_171_5e-3,
    0.0,
    1.823_257_9e-3,
    0.0,
    -9.890_399e-4,
    0.0,
    4.762_265_3e-4,
    0.0,
    -1.843_658_7e-4,
    0.0,
    4.117_896_6e-5,
    0.0,
    0.0,
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

    fn advance(&mut self) -> Option<()> {
        if self.remaining != 0 {
            if self.remaining == 1 {
                self.current = self.target;
            } else {
                let delta = checked(self.target - self.current)?;
                let step = checked(delta / self.remaining as f32)?;
                self.current = checked(self.current + step)?;
            }
            self.remaining -= 1;
        }
        normal_or_zero(self.current).then_some(())
    }

    fn snap_to_target(&mut self) {
        self.current = self.target;
        self.remaining = 0;
    }
}

#[derive(Clone, Debug)]
struct Lane {
    high_cursor: u32,
    dry_cursor: u32,
    ramps: [Ramp; PARAMETER_COUNT],
    interp: [f32; HISTORY],
    decim: [f32; HISTORY],
    dry: [f32; DRY_HISTORY],
}

impl Lane {
    fn new(defaults: [f32; PARAMETER_COUNT]) -> Option<Self> {
        if !state_parameter_valid(0, defaults[0])
            || !state_parameter_valid(1, defaults[1])
            || !state_parameter_valid(2, defaults[2])
        {
            return None;
        }
        Some(Self {
            high_cursor: 0,
            dry_cursor: 0,
            ramps: defaults.map(Ramp::fixed),
            interp: [0.0; HISTORY],
            decim: [0.0; HISTORY],
            dry: [0.0; DRY_HISTORY],
        })
    }

    fn clear_histories(&mut self) {
        self.high_cursor = 0;
        self.dry_cursor = 0;
        self.interp.fill(0.0);
        self.decim.fill(0.0);
        self.dry.fill(0.0);
    }

    fn full_reset(&mut self, defaults: [f32; PARAMETER_COUNT]) {
        self.clear_histories();
        self.ramps = defaults.map(Ramp::fixed);
    }

    fn discontinuity_reset(&mut self) {
        self.clear_histories();
        self.ramps.iter_mut().for_each(Ramp::snap_to_target);
    }

    fn process(&mut self, input: f32, bypass: bool) -> Result<f32, f32> {
        let recovery_dry = delayed_dry(self);
        for ramp in &mut self.ramps {
            if ramp.advance().is_none() {
                return Err(recovery_dry);
            }
        }
        let dry_index = self.dry_cursor as usize;
        self.dry[dry_index] = input;
        let delayed = self.dry[(dry_index + 1) % DRY_HISTORY];
        self.dry_cursor = ((dry_index + 1) % DRY_HISTORY) as u32;
        let doubled = checked(2.0_f32 * self.ramps[0].current).ok_or(recovery_dry)?;
        let first = checked(doubled * input).ok_or(recovery_dry)?;
        let wet = self.stage(first).map_err(|()| recovery_dry)?;
        let _discarded = self.stage(0.0).map_err(|()| recovery_dry)?;
        let mix = self.ramps[2].current;
        let output = self.ramps[1].current;
        if bypass || (mix.to_bits() == 0.0_f32.to_bits() && output.to_bits() == 1.0_f32.to_bits()) {
            return Ok(delayed);
        }
        let a = checked(1.0_f32 - mix).ok_or(recovery_dry)?;
        let b = checked(a * delayed).ok_or(recovery_dry)?;
        let c = checked(mix * wet).ok_or(recovery_dry)?;
        let e = checked(b + c).ok_or(recovery_dry)?;
        checked(output * e).ok_or(recovery_dry)
    }

    fn stage(&mut self, input: f32) -> Result<f32, ()> {
        let cursor = self.high_cursor as usize;
        self.interp[cursor] = checked(input).ok_or(())?;
        let interpolated = convolve(&self.interp, cursor).ok_or(())?;
        let shaped = cubic(interpolated).ok_or(())?;
        self.decim[cursor] = shaped;
        let output = convolve(&self.decim, cursor).ok_or(())?;
        self.high_cursor = ((cursor + 1) % HISTORY) as u32;
        Ok(output)
    }

    fn recover(&mut self) {
        self.clear_histories();
        self.ramps.iter_mut().for_each(Ramp::snap_to_target);
    }
}

/// A prepared scalar soft-clip dual-mono instance.
#[derive(Debug)]
pub struct PreparedSoftClip {
    metadata: PreparedEffectMetadata,
    left_defaults: [f32; PARAMETER_COUNT],
    right_defaults: [f32; PARAMETER_COUNT],
    left: Lane,
    right: Lane,
}

impl NativeEffectFactory for SoftClipFactory {
    fn descriptor(&self) -> &'static EffectDescriptorV1 {
        &SOFT_CLIP_DESCRIPTOR_V1
    }

    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        let metadata = expected_prepared_metadata(self.descriptor(), request)?;
        let (left_defaults, right_defaults) = initial_defaults(request.initial_values)?;
        let left = Lane::new(left_defaults).ok_or(EffectPrepareError {
            code: "effect.parameter.initial",
        })?;
        let right = Lane::new(right_defaults).ok_or(EffectPrepareError {
            code: "effect.parameter.initial",
        })?;
        Ok(Box::new(PreparedSoftClip {
            metadata,
            left_defaults,
            right_defaults,
            left,
            right,
        }))
    }

    fn bind_homogeneous_bank(
        &self,
        _request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
        Ok(None)
    }
}

impl PreparedNativeEffect for PreparedSoftClip {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.metadata
    }

    fn reset(&mut self, kind: ResetKind) {
        match kind {
            ResetKind::FullToDefaults => {
                self.left.full_reset(self.left_defaults);
                self.right.full_reset(self.right_defaults);
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
            let left_input = sanitize(block.left[index], &mut report.sanitized_main_samples);
            let right_input = sanitize(block.right[index], &mut report.sanitized_main_samples);
            block.left[index] = match self.left.process(left_input, self.metadata.bypass) {
                Ok(value) => value,
                Err(delayed) => {
                    self.left.recover();
                    report.recovered_left_samples = report.recovered_left_samples.saturating_add(1);
                    delayed
                }
            };
            block.right[index] = match self.right.process(right_input, self.metadata.bypass) {
                Ok(value) => value,
                Err(delayed) => {
                    self.right.recover();
                    report.recovered_right_samples =
                        report.recovered_right_samples.saturating_add(1);
                    delayed
                }
            };
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
        let left = read_lane(input.left)?;
        let right = read_lane(input.right)?;
        self.left = left;
        self.right = right;
        Ok(())
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
    for (index, value) in values.iter().enumerate() {
        let parameter = index / 2;
        let channel = if index % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        };
        if value.parameter_index != parameter as u32
            || value.channel != channel
            || !parameter_value_valid(parameter, value.value)
            || negative_zero(value.value)
        {
            return Err(EffectPrepareError {
                code: "effect.parameter.initial",
            });
        }
        let converted = convert_parameter(parameter, value.value).ok_or(EffectPrepareError {
            code: "effect.parameter.initial",
        })?;
        if index % 2 == 0 {
            left[parameter] = converted;
        } else {
            right[parameter] = converted;
        }
    }
    Ok((left, right))
}

fn apply_automation(
    spans: &[PreparedAutomationSpan],
    metadata: PreparedEffectMetadata,
    first_sample: u64,
    left: &mut Lane,
    right: &mut Lane,
    report: &mut ProcessReport,
) {
    let mut pending = [[None; PARAMETER_COUNT]; 2];
    let mut prior = None;
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
            && parameter < PARAMETER_COUNT
            && span.kind == AutomationSpanKind::Point
            && span.start_sample == first_sample
            && span.end_sample == first_sample
            && span.start_value.to_bits() == span.end_value.to_bits()
            && parameter_value_valid(parameter, span.start_value)
            && !negative_zero(span.start_value)
            && prior.is_none_or(|previous| order > previous)
            && pending[lane][parameter].is_none();
        if !valid {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        }
        let Some(value) = convert_parameter(parameter, normalize_zero(span.start_value)) else {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        };
        prior = Some(order);
        pending[lane][parameter] = Some(value);
    }
    for (parameter, (left_target, right_target)) in
        pending[0].into_iter().zip(pending[1]).enumerate()
    {
        if let Some(target) = left_target {
            left.ramps[parameter].target = target;
            left.ramps[parameter].remaining = RAMP_SAMPLES;
        }
        if let Some(target) = right_target {
            right.ramps[parameter].target = target;
            right.ramps[parameter].remaining = RAMP_SAMPLES;
        }
    }
}

fn convolve(history: &[f32; HISTORY], cursor: usize) -> Option<f32> {
    let mut accumulator = 0.0_f32;
    for tap in TAPS {
        let sample = history[(cursor + HISTORY - tap) % HISTORY];
        let product = checked(H[tap] * sample)?;
        accumulator = checked(accumulator + product)?;
    }
    Some(accumulator)
}

fn cubic(value: f32) -> Option<f32> {
    if value <= -1.0 {
        Some(-2.0_f32 / 3.0_f32)
    } else if value >= 1.0 {
        Some(2.0_f32 / 3.0_f32)
    } else {
        let p0 = checked(value * value)?;
        let p1 = checked(p0 * value)?;
        let p2 = checked(p1 / 3.0_f32)?;
        checked(value - p2)
    }
}

fn delayed_dry(lane: &Lane) -> f32 {
    lane.dry[(lane.dry_cursor as usize + 1) % DRY_HISTORY]
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

fn checked(value: f32) -> Option<f32> {
    if !value.is_finite() {
        None
    } else if value.is_subnormal() {
        Some(0.0)
    } else {
        Some(value)
    }
}

fn normal_or_zero(value: f32) -> bool {
    value.is_finite() && (value == 0.0 || value.is_normal())
}

fn parameter_value_valid(index: usize, value: f32) -> bool {
    value.is_finite()
        && SOFT_CLIP_PARAMETERS_V1
            .get(index)
            .and_then(|parameter| parameter.minimum.zip(parameter.maximum))
            .is_some_and(|(minimum, maximum)| value >= minimum && value <= maximum)
}

fn converted_domain(index: usize, value: f32) -> bool {
    match index {
        0 => (db_gain(-24.0)..=db_gain(36.0)).contains(&value),
        1 => (db_gain(-24.0)..=db_gain(24.0)).contains(&value),
        2 => (0.0..=1.0).contains(&value),
        _ => false,
    }
}

fn state_parameter_valid(index: usize, value: f32) -> bool {
    !negative_zero(value) && normal_or_zero(value) && converted_domain(index, value)
}

fn convert_parameter(index: usize, value: f32) -> Option<f32> {
    if !parameter_value_valid(index, value) {
        return None;
    }
    let value = normalize_zero(value);
    match index {
        0 | 1 => checked(db_gain(value)),
        2 => Some(value),
        _ => None,
    }
}

fn db_gain(value: f32) -> f32 {
    10.0_f32.powf(value * 0.05_f32)
}

fn negative_zero(value: f32) -> bool {
    value.to_bits() == (-0.0_f32).to_bits()
}

fn normalize_zero(value: f32) -> f32 {
    if value == 0.0 { 0.0 } else { value }
}

fn validate_state_lengths(
    common: usize,
    left: usize,
    right: usize,
    sizes: StatePayloadSizes,
) -> Result<(), StatePayloadError> {
    if common != sizes.common_bytes as usize
        || left != sizes.left_bytes as usize
        || right != sizes.right_bytes as usize
    {
        return Err(state_error("effect.state.length"));
    }
    Ok(())
}

fn write_lane(bytes: &mut [u8], lane: &Lane) {
    write_u32(bytes, 0, lane.high_cursor);
    write_u32(bytes, 1, lane.dry_cursor);
    for (index, ramp) in lane.ramps.iter().enumerate() {
        let word = 2 + index * 3;
        write_f32(bytes, word, ramp.current);
        write_f32(bytes, word + 1, ramp.target);
        write_u32(bytes, word + 2, ramp.remaining);
    }
    for (index, value) in lane.interp.iter().enumerate() {
        write_f32(bytes, 11 + index, *value);
    }
    for (index, value) in lane.decim.iter().enumerate() {
        write_f32(bytes, 74 + index, *value);
    }
    for (index, value) in lane.dry.iter().enumerate() {
        write_f32(bytes, 137 + index, *value);
    }
}

fn read_lane(bytes: &[u8]) -> Result<Lane, StatePayloadError> {
    if bytes.len() != LANE_STATE_BYTES as usize {
        return Err(state_error("effect.state.length"));
    }
    let high_cursor = read_u32(bytes, 0);
    let dry_cursor = read_u32(bytes, 1);
    if high_cursor as usize >= HISTORY || dry_cursor as usize >= DRY_HISTORY {
        return Err(state_error("effect.state.cursor"));
    }
    let mut ramps = [Ramp::fixed(0.0); PARAMETER_COUNT];
    for (index, ramp) in ramps.iter_mut().enumerate() {
        let word = 2 + index * 3;
        let current = read_f32(bytes, word);
        let target = read_f32(bytes, word + 1);
        let remaining = read_u32(bytes, word + 2);
        if !state_parameter_valid(index, current)
            || !state_parameter_valid(index, target)
            || remaining > RAMP_SAMPLES
        {
            return Err(state_error("effect.state.parameter"));
        }
        *ramp = Ramp {
            current,
            target,
            remaining,
        };
    }
    let mut interp = [0.0; HISTORY];
    let mut decim = [0.0; HISTORY];
    let mut dry = [0.0; DRY_HISTORY];
    for (index, value) in interp.iter_mut().enumerate() {
        *value = read_f32(bytes, 11 + index);
        if !normal_or_zero(*value) {
            return Err(state_error("effect.state.history"));
        }
    }
    for (index, value) in decim.iter_mut().enumerate() {
        *value = read_f32(bytes, 74 + index);
        if !normal_or_zero(*value) {
            return Err(state_error("effect.state.history"));
        }
    }
    for (index, value) in dry.iter_mut().enumerate() {
        *value = read_f32(bytes, 137 + index);
        if !normal_or_zero(*value) {
            return Err(state_error("effect.state.history"));
        }
    }
    Ok(Lane {
        high_cursor,
        dry_cursor,
        ramps,
        interp,
        decim,
        dry,
    })
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
    use miso_engine_dsp_reference::{ReferenceSoftClip, reference_halfband_63};
    use miso_engine_effect_contract::{
        EffectProcessBlock, LinkMode, PrepareEffectLimits, PreparedNativeEffect, PreparedPortsV1,
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
            value: SOFT_CLIP_PARAMETERS_V1[index / 2].default_value,
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
                sidechain: miso_engine_effect_contract::PreparedSidechainPort::None,
            },
            initial_values: values,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: 1_352,
                maximum_scratch_bytes: 24,
                maximum_automation_spans_per_block: 16,
            },
        }
    }

    fn prepare(values: &[InitialParameterValue]) -> Box<dyn PreparedNativeEffect> {
        SoftClipFactory.prepare(request(values)).expect("prepare")
    }

    fn process(
        effect: &mut dyn PreparedNativeEffect,
        left: &mut [f32],
        right: &mut [f32],
        first: u64,
        automation: &[PreparedAutomationSpan],
    ) -> ProcessReport {
        effect.process(
            EffectProcessBlock::new(left, right, None, first, automation, 128).expect("block"),
        )
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
    fn descriptor_resources_and_independent_fir_design_are_frozen() {
        validate_descriptor_v1(&SOFT_CLIP_DESCRIPTOR_V1).expect("descriptor");
        assert_eq!(
            SOFT_CLIP_DESCRIPTOR_V1.supported_link_modes,
            LinkModeSet::DUAL_MONO
        );
        assert_eq!(SOFT_CLIP_DESCRIPTOR_V1.parameters.len(), 3);
        for quality in QUALITIES {
            assert_eq!(quality.latency, LatencySamples(31));
            assert_eq!(quality.tail, TailSamples::Finite(31));
            assert_eq!(quality.maximum_state.left_bytes, 676);
            assert_eq!(quality.maximum_state.right_bytes, 676);
            assert_eq!(quality.scratch_fixed_bytes, 24);
        }
        let reference = reference_halfband_63();
        for (actual, expected) in H.into_iter().zip(reference) {
            assert!((actual as f64 - expected).abs() < 1.0e-7);
        }
        let values = initial_values();
        let mut too_small = request(&values);
        too_small.limits.maximum_total_state_bytes = 1_351;
        assert!(matches!(
            SoftClipFactory.prepare(too_small),
            Err(EffectPrepareError {
                code: "effect.resource.limit"
            })
        ));
    }

    #[test]
    fn scalar_matches_independent_oracle_after_warmup() {
        let mut values = initial_values();
        values[0].value = 18.0;
        values[1].value = 18.0;
        values[2].value = 0.0;
        values[3].value = 0.0;
        values[4].value = 1.0;
        values[5].value = 1.0;
        let mut effect = prepare(&values);
        let mut oracle = ReferenceSoftClip::new(18.0, 0.0, 1.0).expect("oracle");
        let mut input = (0..128)
            .map(|index| (index as f32 * 0.073).sin() * 0.8)
            .collect::<Vec<_>>();
        let expected = input
            .iter()
            .map(|value| oracle.process(*value as f64) as f32)
            .collect::<Vec<_>>();
        let mut right = input.clone();
        process(effect.as_mut(), &mut input, &mut right, 0, &[]);
        for (actual, expected) in input.into_iter().zip(expected).skip(64) {
            assert!(
                (actual - expected).abs() <= 3.0e-6,
                "actual={actual:?}, expected={expected:?}"
            );
        }
    }

    #[test]
    fn delayed_identity_automation_reset_restore_and_recovery_are_lane_local() {
        let mut values = initial_values();
        values[4].value = 0.0;
        values[5].value = 0.0;
        let mut effect = prepare(&values);
        let mut left = vec![-0.0; 64];
        let mut right = vec![0.0; 64];
        left[31] = 0.25;
        right[31] = -0.5;
        process(effect.as_mut(), &mut left, &mut right, 0, &[]);
        assert_eq!(left[62].to_bits(), 0.25_f32.to_bits());
        assert_eq!(right[62].to_bits(), (-0.5_f32).to_bits());
        assert_eq!(left[31].to_bits(), (-0.0_f32).to_bits());

        let span = PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel: ParameterChannel::Left,
            parameter_index: 0,
            start_sample: 64,
            end_sample: 64,
            start_value: 12.0,
            end_value: 12.0,
        };
        let mut active_left = vec![0.2; 64];
        let mut active_right = vec![0.1; 64];
        process(
            effect.as_mut(),
            &mut active_left,
            &mut active_right,
            64,
            &[span],
        );
        let state = snapshot(effect.as_ref());
        let mut continuation_left = vec![0.3; 32];
        let mut continuation_right = vec![-0.2; 32];
        let mut expected_left = continuation_left.clone();
        let mut expected_right = continuation_right.clone();
        process(
            effect.as_mut(),
            &mut expected_left,
            &mut expected_right,
            128,
            &[],
        );
        effect
            .restore_state_payload(
                1,
                StatePayloadInput::new(&[], &state.0, &state.1, effect.metadata().state_sizes)
                    .expect("state"),
            )
            .expect("restore");
        process(
            effect.as_mut(),
            &mut continuation_left,
            &mut continuation_right,
            128,
            &[],
        );
        assert_eq!(continuation_left, expected_left);
        assert_eq!(continuation_right, expected_right);

        let mut invalid_left = vec![f32::NAN; 1];
        let mut invalid_right = vec![0.1; 1];
        let report = process(
            effect.as_mut(),
            &mut invalid_left,
            &mut invalid_right,
            160,
            &[],
        );
        assert_eq!(report.sanitized_main_samples, 1);
        let mut concrete = PreparedSoftClip {
            metadata: effect.metadata(),
            left_defaults: [1.0, 1.0, 1.0],
            right_defaults: [1.0, 1.0, 1.0],
            left: Lane::new([1.0, 1.0, 1.0]).expect("lane"),
            right: Lane::new([1.0, 1.0, 1.0]).expect("lane"),
        };
        concrete.left.interp[61] = f32::NAN;
        concrete.left.dry[1] = -0.25;
        let mut bad_left = [0.0];
        let mut good_right = [0.0];
        let report = concrete.process(
            EffectProcessBlock::new(&mut bad_left, &mut good_right, None, 0, &[], 128)
                .expect("block"),
        );
        assert_eq!(report.recovered_left_samples, 1);
        assert_eq!(report.recovered_right_samples, 0);
        assert_eq!(bad_left[0].to_bits(), (-0.25_f32).to_bits());
        concrete.reset(ResetKind::DiscontinuityKeepParameters);
        assert_eq!(concrete.left.high_cursor, 0);
        concrete.reset(ResetKind::FullToDefaults);
        assert_eq!(concrete.left.ramps[0].current, 1.0);
    }
}
