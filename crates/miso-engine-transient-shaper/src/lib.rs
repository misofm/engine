//! Fixed causal dual-envelope transient shaper.
//!
//! This crate contains the scalar Issue-020 checkpoint only. Homogeneous banking and graph
//! registration are deliberately deferred to their separately bounded checkpoints.
#![allow(missing_docs)]

use miso_engine_effect_contract::{
    AutomationRate, AutomationSpanKind, EffectDescriptorV1, EffectPrepareError, EffectProcessBlock,
    EffectQuality, InitialParameterValue, LatencySamples, LinkMode, LinkModeSet,
    NativeEffectFactory, ParameterChannel, ParameterChannelPolicy, ParameterDescriptorV1,
    ParameterDomain, ParameterId, ParameterMapping, ParameterUnit, PortDescriptorV1, PortId,
    PortLayout, PortRole, PrepareEffectBankRequest, PrepareEffectRequest, PreparedAutomationSpan,
    PreparedEffectMetadata, PreparedNativeEffect, ProcessReport, ResetKind, SmoothingRule,
    StatePayloadError, StatePayloadInput, StatePayloadOutput, StatePayloadSizes, TailSamples,
    expected_prepared_metadata, sanitize_sample,
};

const PARAMETER_COUNT: usize = 3;
const RAMP_SAMPLES: u32 = 64;
const STATE_WORDS: usize = 11;
const LANE_STATE_BYTES: u32 = (STATE_WORDS * 4) as u32;

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

const fn parameter(
    id: u32,
    display_name: &'static str,
    display_unit: &'static str,
    minimum: f32,
    maximum: f32,
    default_value: f32,
) -> ParameterDescriptorV1 {
    ParameterDescriptorV1 {
        id: parameter_id(id),
        display_name,
        display_unit,
        unit: ParameterUnit::Linear,
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

/// Frozen V1 parameter rows in stable numeric-ID order.
pub const TRANSIENT_SHAPER_PARAMETERS_V1: [ParameterDescriptorV1; PARAMETER_COUNT] = [
    parameter(1, "attack amount", "%", -1.0, 1.0, 0.0),
    parameter(2, "sustain amount", "%", -1.0, 1.0, 0.0),
    parameter(3, "mix", "linear", 0.0, 1.0, 1.0),
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

const fn quality(sample_rate: u32) -> miso_engine_effect_contract::QualityDescriptorV1 {
    miso_engine_effect_contract::QualityDescriptorV1 {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(0),
        tail: TailSamples::Finite(0),
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

/// Immutable descriptor for the frozen causal transient-shaper V1 contract.
pub const TRANSIENT_SHAPER_DESCRIPTOR_V1: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("miso.transient-shaper"),
    display_name: "Transient Shaper",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::ALL,
    parameters: &TRANSIENT_SHAPER_PARAMETERS_V1,
    ports: &PORTS,
    qualities: &QUALITIES,
};

/// Fast-attack, fast-release, slow-attack, slow-release coefficient bits by launch-rate row.
pub const TRANSIENT_SHAPER_COEFFICIENT_BITS_V1: [[u32; 4]; 4] = [
    [0x3f74_a63c, 0x3f7f_b5bd, 0x3f7f_6b90, 0x3f7f_f124],
    [0x3f75_8d71, 0x3f7f_bbc5, 0x3f7f_779c, 0x3f7f_f259],
    [0x3f7a_42a5, 0x3f7f_dadc, 0x3f7f_b5bd, 0x3f7f_f892],
    [0x3f7a_b8ca, 0x3f7f_dde0, 0x3f7f_bbc5, 0x3f7f_f92c],
];

/// Scalar factory entry point for the transient shaper.
#[derive(Clone, Copy, Debug, Default)]
pub struct TransientShaperFactory;

#[derive(Clone, Copy, Debug)]
struct Coefficients {
    fast_attack: f32,
    fast_release: f32,
    slow_attack: f32,
    slow_release: f32,
}

fn coefficients(sample_rate: u32) -> Option<Coefficients> {
    let row = match sample_rate {
        44_100 => TRANSIENT_SHAPER_COEFFICIENT_BITS_V1[0],
        48_000 => TRANSIENT_SHAPER_COEFFICIENT_BITS_V1[1],
        88_200 => TRANSIENT_SHAPER_COEFFICIENT_BITS_V1[2],
        96_000 => TRANSIENT_SHAPER_COEFFICIENT_BITS_V1[3],
        _ => return None,
    };
    Some(Coefficients {
        fast_attack: f32::from_bits(row[0]),
        fast_release: f32::from_bits(row[1]),
        slow_attack: f32::from_bits(row[2]),
        slow_release: f32::from_bits(row[3]),
    })
}

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
            let delta = self.target - self.current;
            let step = delta / self.remaining as f32;
            self.current += step;
            self.remaining -= 1;
            if self.remaining == 0 {
                self.current = self.target;
            }
        }
    }

    fn reset_to_target(&mut self) {
        self.current = self.target;
        self.remaining = 0;
    }
}

#[derive(Clone, Copy, Debug)]
struct Lane {
    fast: f32,
    slow: f32,
    ramps: [Ramp; PARAMETER_COUNT],
}

impl Lane {
    fn new(defaults: &[f32; PARAMETER_COUNT]) -> Self {
        Self {
            fast: 0.0,
            slow: 0.0,
            ramps: core::array::from_fn(|index| Ramp::fixed(defaults[index])),
        }
    }

    fn full_reset(&mut self, defaults: &[f32; PARAMETER_COUNT]) {
        self.fast = 0.0;
        self.slow = 0.0;
        self.ramps = core::array::from_fn(|index| Ramp::fixed(defaults[index]));
    }

    fn discontinuity_reset(&mut self) {
        self.fast = 0.0;
        self.slow = 0.0;
        for ramp in &mut self.ramps {
            ramp.reset_to_target();
        }
    }
}

/// Fixed-shape allocation-free scalar transient shaper.
#[derive(Debug)]
pub struct PreparedTransientShaper {
    metadata: PreparedEffectMetadata,
    coefficients: Coefficients,
    left_defaults: [f32; PARAMETER_COUNT],
    right_defaults: [f32; PARAMETER_COUNT],
    left: Lane,
    right: Lane,
}

impl NativeEffectFactory for TransientShaperFactory {
    fn descriptor(&self) -> &'static EffectDescriptorV1 {
        &TRANSIENT_SHAPER_DESCRIPTOR_V1
    }

    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        let metadata = expected_prepared_metadata(self.descriptor(), request)?;
        let coefficients = coefficients(metadata.sample_rate).ok_or(EffectPrepareError {
            code: "effect.quality.unsupported",
        })?;
        let (left_defaults, right_defaults) = initial_defaults(request.initial_values)?;
        Ok(Box::new(PreparedTransientShaper {
            metadata,
            coefficients,
            left_defaults,
            right_defaults,
            left: Lane::new(&left_defaults),
            right: Lane::new(&right_defaults),
        }))
    }

    fn bind_homogeneous_bank(
        &self,
        _request: PrepareEffectBankRequest<'_>,
    ) -> Result<
        Option<Box<dyn miso_engine_effect_contract::PreparedNativeEffectBank>>,
        EffectPrepareError,
    > {
        // Issue 020 scalar checkpoint: the separate bank checkpoint owns binding validation and
        // the compressor gain/mix kernel use. Returning no cohort cannot make scalar preparation
        // unpublished or change its immutable program metadata.
        Ok(None)
    }
}

impl PreparedNativeEffect for PreparedTransientShaper {
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
        for index in 0..block.frames() {
            for ramp in &mut self.left.ramps {
                ramp.advance();
            }
            for ramp in &mut self.right.ramps {
                ramp.advance();
            }
            let left = sanitize(block.left[index], &mut report.sanitized_main_samples);
            let right = sanitize(block.right[index], &mut report.sanitized_main_samples);
            let (detector_left, detector_right) =
                linked_magnitudes(self.metadata.link_mode, left, right);
            block.left[index] = process_lane(
                left,
                detector_left,
                self.coefficients,
                self.metadata.bypass,
                &mut self.left,
                &mut report.recovered_left_samples,
            );
            block.right[index] = process_lane(
                right,
                detector_right,
                self.coefficients,
                self.metadata.bypass,
                &mut self.right,
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
        write_lane(output.left, self.left);
        write_lane(output.right, self.right);
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
    for (index, parameter) in TRANSIENT_SHAPER_PARAMETERS_V1.iter().enumerate() {
        let left_value = values[index * 2];
        let right_value = values[index * 2 + 1];
        if left_value.parameter_index != index as u32
            || right_value.parameter_index != index as u32
            || left_value.channel != ParameterChannel::Left
            || right_value.channel != ParameterChannel::Right
            || !parameter_value_valid(parameter, left_value.value)
            || !parameter_value_valid(parameter, right_value.value)
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

fn normalize_zero(value: f32) -> f32 {
    if value == 0.0 { 0.0 } else { value }
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

fn linked_magnitudes(link_mode: LinkMode, left: f32, right: f32) -> (f32, f32) {
    let left = left.abs();
    let right = right.abs();
    match link_mode {
        LinkMode::DualMono => (left, right),
        LinkMode::Maximum => {
            let value = left.max(right);
            (value, value)
        }
        LinkMode::Average => {
            let left_half = 0.5_f32 * left;
            let right_half = 0.5_f32 * right;
            let value = left_half + right_half;
            (value, value)
        }
    }
}

fn process_lane(
    input: f32,
    detector: f32,
    coefficients: Coefficients,
    bypass: bool,
    lane: &mut Lane,
    recovered: &mut u64,
) -> f32 {
    let fast_coefficient = if detector > lane.fast {
        coefficients.fast_attack
    } else {
        coefficients.fast_release
    };
    let slow_coefficient = if detector > lane.slow {
        coefficients.slow_attack
    } else {
        coefficients.slow_release
    };
    let fast0 = fast_coefficient * lane.fast;
    let fast1 = (1.0_f32 - fast_coefficient) * detector;
    let Some(fast) = flush(fast0 + fast1) else {
        return recover(lane, input, recovered);
    };
    let slow0 = slow_coefficient * lane.slow;
    let slow1 = (1.0_f32 - slow_coefficient) * detector;
    let Some(slow) = flush(slow0 + slow1) else {
        return recover(lane, input, recovered);
    };
    lane.fast = fast;
    lane.slow = slow;

    let fast_db = 20.0_f32 * fast.max(1.0e-8_f32).log10();
    let slow_db = 20.0_f32 * slow.max(1.0e-8_f32).log10();
    let Some(contrast_raw) = flush(fast_db - slow_db) else {
        return recover(lane, input, recovered);
    };
    let contrast = contrast_raw.clamp(-24.0_f32, 24.0_f32);
    let attack_term = contrast.max(0.0_f32);
    let sustain_term = (-contrast).max(0.0_f32);
    let shape0 = lane.ramps[0].current * attack_term;
    let shape1 = lane.ramps[1].current * sustain_term;
    let Some(shape_raw) = flush(shape0 + shape1) else {
        return recover(lane, input, recovered);
    };
    let shape = normalize_zero(shape_raw.clamp(-18.0_f32, 18.0_f32));
    let gain_exponent = shape * 0.05_f32;
    let Some(gain) = flush(10.0_f32.powf(gain_exponent)) else {
        return recover(lane, input, recovered);
    };
    let mix = lane.ramps[2].current;
    if bypass || mix == 0.0 || shape == 0.0 {
        return input;
    }
    let Some(wet) = flush(input * gain) else {
        return recover(lane, input, recovered);
    };
    if mix == 1.0 {
        return wet;
    }
    let delta = wet - input;
    let scaled = mix * delta;
    match flush(input + scaled) {
        Some(value) => value,
        None => recover(lane, input, recovered),
    }
}

fn flush(value: f32) -> Option<f32> {
    if !value.is_finite() {
        None
    } else if value.is_subnormal() {
        Some(0.0)
    } else {
        Some(value)
    }
}

fn recover(lane: &mut Lane, input: f32, recovered: &mut u64) -> f32 {
    lane.fast = 0.0;
    lane.slow = 0.0;
    *recovered = recovered.saturating_add(1);
    input
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
            && parameter_index < PARAMETER_COUNT
            && span.kind == AutomationSpanKind::Point
            && span.start_sample == first_sample
            && span.end_sample == first_sample
            && span.start_value.to_bits() == span.end_value.to_bits()
            && parameter_value_valid(
                &TRANSIENT_SHAPER_PARAMETERS_V1[parameter_index],
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
    for (parameter_index, (left_pending, right_pending)) in
        pending[0].iter().zip(pending[1].iter()).enumerate()
    {
        if let Some(value) = *left_pending {
            left.ramps[parameter_index].target = value;
            left.ramps[parameter_index].remaining = RAMP_SAMPLES;
        }
        if let Some(value) = *right_pending {
            right.ramps[parameter_index].target = value;
            right.ramps[parameter_index].remaining = RAMP_SAMPLES;
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

fn write_lane(bytes: &mut [u8], lane: Lane) {
    write_f32(bytes, 0, lane.fast);
    write_f32(bytes, 1, lane.slow);
    for (index, ramp) in lane.ramps.iter().enumerate() {
        let word = 2 + index * 3;
        write_f32(bytes, word, ramp.current);
        write_f32(bytes, word + 1, ramp.target);
        write_u32(bytes, word + 2, ramp.remaining);
    }
}

fn read_lane(bytes: &[u8]) -> Result<Lane, StatePayloadError> {
    let fast = read_f32(bytes, 0);
    let slow = read_f32(bytes, 1);
    if !valid_envelope(fast) || !valid_envelope(slow) {
        return Err(state_error("effect.state.envelope"));
    }
    let mut ramps = [Ramp::fixed(0.0); PARAMETER_COUNT];
    for (index, ramp) in ramps.iter_mut().enumerate() {
        let word = 2 + index * 3;
        let current = read_f32(bytes, word);
        let target = read_f32(bytes, word + 1);
        let remaining = read_u32(bytes, word + 2);
        let parameter = &TRANSIENT_SHAPER_PARAMETERS_V1[index];
        if !parameter_value_valid(parameter, current)
            || !parameter_value_valid(parameter, target)
            || remaining > RAMP_SAMPLES
        {
            return Err(state_error("effect.state.parameter"));
        }
        *ramp = Ramp {
            current: normalize_zero(current),
            target: normalize_zero(target),
            remaining,
        };
    }
    Ok(Lane { fast, slow, ramps })
}

fn valid_envelope(value: f32) -> bool {
    (value.is_normal() && value.is_sign_positive()) || value.to_bits() == 0.0_f32.to_bits()
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
            .expect("state length was validated"),
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
        ReferenceTransientShaper, ReferenceTransientShaperParameters,
        reference_transient_shaper_coefficient,
    };
    use miso_engine_effect_contract::{
        EffectProcessBlock, InitialParameterValue, PreparedNativeEffect, StatePayloadInput,
        StatePayloadOutput, validate_descriptor_v1,
    };

    fn initial_values() -> [InitialParameterValue; PARAMETER_COUNT * 2] {
        core::array::from_fn(|index| InitialParameterValue {
            parameter_index: (index / 2) as u32,
            channel: if index % 2 == 0 {
                ParameterChannel::Left
            } else {
                ParameterChannel::Right
            },
            value: TRANSIENT_SHAPER_PARAMETERS_V1[index / 2].default_value,
        })
    }

    fn request<'a>(values: &'a [InitialParameterValue]) -> PrepareEffectRequest<'a> {
        request_with(values, 48_000, false, LinkMode::DualMono)
    }

    fn request_with<'a>(
        values: &'a [InitialParameterValue],
        sample_rate: u32,
        bypass: bool,
        link_mode: LinkMode,
    ) -> PrepareEffectRequest<'a> {
        PrepareEffectRequest {
            sample_rate,
            quantum: 128,
            quality: EffectQuality::Normal,
            bypass,
            link_mode,
            ports: miso_engine_effect_contract::PreparedPortsV1 {
                sidechain: miso_engine_effect_contract::PreparedSidechainPort::None,
            },
            initial_values: values,
            limits: miso_engine_effect_contract::PrepareEffectLimits {
                maximum_total_state_bytes: 88,
                maximum_scratch_bytes: 24,
                maximum_automation_spans_per_block: 16,
            },
        }
    }

    fn prepare(values: &[InitialParameterValue]) -> Box<dyn PreparedNativeEffect> {
        TransientShaperFactory
            .prepare(request(values))
            .expect("prepare")
    }

    fn snapshot(effect: &dyn PreparedNativeEffect) -> (Vec<u8>, Vec<u8>) {
        let sizes = effect.metadata().state_sizes;
        let mut left = vec![0; sizes.left_bytes as usize];
        let mut right = vec![0; sizes.right_bytes as usize];
        effect
            .snapshot_state_payload(
                StatePayloadOutput::new(&mut [], &mut left, &mut right, sizes).expect("output"),
            )
            .expect("snapshot");
        (left, right)
    }

    fn prepare_concrete(
        values: &[InitialParameterValue],
        sample_rate: u32,
        bypass: bool,
        link_mode: LinkMode,
    ) -> PreparedTransientShaper {
        let request = request_with(values, sample_rate, bypass, link_mode);
        let metadata =
            expected_prepared_metadata(&TRANSIENT_SHAPER_DESCRIPTOR_V1, request).expect("metadata");
        let coefficients = coefficients(sample_rate).expect("coefficients");
        let (left_defaults, right_defaults) = initial_defaults(values).expect("defaults");
        PreparedTransientShaper {
            metadata,
            coefficients,
            left_defaults,
            right_defaults,
            left: Lane::new(&left_defaults),
            right: Lane::new(&right_defaults),
        }
    }

    fn state_f32(bytes: &[u8], word: usize) -> f32 {
        read_f32(bytes, word)
    }

    fn state_u32(bytes: &[u8], word: usize) -> u32 {
        read_u32(bytes, word)
    }

    fn expected_lane_bytes(
        envelopes: [f32; 2],
        ramps: [(f32, f32, u32); PARAMETER_COUNT],
    ) -> Vec<u8> {
        let mut bytes = vec![0_u8; LANE_STATE_BYTES as usize];
        write_f32(&mut bytes, 0, envelopes[0]);
        write_f32(&mut bytes, 1, envelopes[1]);
        for (index, ramp) in ramps.into_iter().enumerate() {
            let word = 2 + index * 3;
            write_f32(&mut bytes, word, ramp.0);
            write_f32(&mut bytes, word + 1, ramp.1);
            write_u32(&mut bytes, word + 2, ramp.2);
        }
        bytes
    }

    fn render_reference_row(
        attack_amount: f32,
        sustain_amount: f32,
        signal: &[f32],
        measured_from: usize,
    ) -> (f64, f64, f64) {
        let mut values = initial_values();
        values[0].value = attack_amount;
        values[1].value = attack_amount;
        values[2].value = sustain_amount;
        values[3].value = sustain_amount;
        let mut effect = prepare(&values);
        let mut reference = ReferenceTransientShaper::new(
            48_000.0,
            ReferenceTransientShaperParameters {
                attack_amount: attack_amount as f64,
                sustain_amount: sustain_amount as f64,
                mix: 1.0,
            },
        )
        .expect("reference");
        let mut maximum_error_db = 0.0_f64;
        let mut minimum_reference_gain_db = f64::INFINITY;
        let mut maximum_reference_gain_db = f64::NEG_INFINITY;
        for (index, input) in signal.iter().copied().enumerate() {
            let mut left = [input];
            let mut right = [input];
            effect.process(
                EffectProcessBlock::new(&mut left, &mut right, None, index as u64, &[], 128)
                    .expect("row sample"),
            );
            let expected = reference.process_sample(input as f64, input.abs() as f64);
            if index >= measured_from && input.abs() >= 1.0e-4 {
                let production_gain_db =
                    20.0_f64 * (left[0].abs() as f64 / input.abs() as f64).log10();
                let reference_gain_db = 20.0_f64 * (expected.abs() / input.abs() as f64).log10();
                maximum_error_db =
                    maximum_error_db.max((production_gain_db - reference_gain_db).abs());
                minimum_reference_gain_db = minimum_reference_gain_db.min(reference_gain_db);
                maximum_reference_gain_db = maximum_reference_gain_db.max(reference_gain_db);
            }
        }
        (
            maximum_error_db,
            minimum_reference_gain_db,
            maximum_reference_gain_db,
        )
    }

    #[test]
    fn descriptor_coefficients_resources_and_transactional_caps_are_frozen() {
        validate_descriptor_v1(&TRANSIENT_SHAPER_DESCRIPTOR_V1).expect("descriptor");
        assert_eq!(
            TRANSIENT_SHAPER_DESCRIPTOR_V1.id.as_str(),
            "miso.transient-shaper"
        );
        assert_eq!(TRANSIENT_SHAPER_DESCRIPTOR_V1.parameters.len(), 3);
        assert_eq!(TRANSIENT_SHAPER_DESCRIPTOR_V1.qualities.len(), 4);
        for (quality, bits) in TRANSIENT_SHAPER_DESCRIPTOR_V1
            .qualities
            .iter()
            .zip(TRANSIENT_SHAPER_COEFFICIENT_BITS_V1)
        {
            let coefficients = coefficients(quality.sample_rate).expect("launch coefficient row");
            assert_eq!(coefficients.fast_attack.to_bits(), bits[0]);
            assert_eq!(coefficients.fast_release.to_bits(), bits[1]);
            assert_eq!(coefficients.slow_attack.to_bits(), bits[2]);
            assert_eq!(coefficients.slow_release.to_bits(), bits[3]);
            assert_eq!(quality.latency, LatencySamples(0));
            assert_eq!(quality.tail, TailSamples::Finite(0));
            assert_eq!(quality.maximum_state.total(), Some(88));
            assert_eq!(quality.scratch_fixed_bytes, 24);
        }
        let values = initial_values();
        let mut too_small = request(&values);
        too_small.limits.maximum_total_state_bytes = 87;
        assert!(matches!(
            TransientShaperFactory.prepare(too_small),
            Err(EffectPrepareError {
                code: "effect.resource.limit"
            })
        ));
    }

    #[test]
    fn independent_coefficients_time_constants_layout_and_both_caps_are_exact() {
        const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
        const TIMES_MS: [f64; 4] = [0.5, 20.0, 10.0, 100.0];
        let values = initial_values();
        for ((sample_rate, production_bits), quality) in RATES
            .into_iter()
            .zip(TRANSIENT_SHAPER_COEFFICIENT_BITS_V1)
            .zip(TRANSIENT_SHAPER_DESCRIPTOR_V1.qualities)
        {
            assert_eq!(quality.sample_rate, sample_rate);
            assert_eq!(quality.maximum_state.common_bytes, 0);
            assert_eq!(quality.maximum_state.left_bytes, 44);
            assert_eq!(quality.maximum_state.right_bytes, 44);
            assert_eq!(quality.scratch_fixed_bytes, 24);
            assert_eq!(quality.scratch_bytes_per_frame, 0);
            for ((time_ms, bits), index) in TIMES_MS
                .into_iter()
                .zip(production_bits)
                .zip(0..TIMES_MS.len())
            {
                let independent =
                    reference_transient_shaper_coefficient(time_ms, sample_rate as f64)
                        .expect("independent coefficient");
                assert_eq!(
                    (independent as f32).to_bits(),
                    bits,
                    "rate={sample_rate} coefficient={index}"
                );
                let retained = f32::from_bits(bits) as f64;
                let recovered_time_ms = -1000.0 / (sample_rate as f64 * retained.ln());
                let timing_tolerance_ms = (1000.0 / sample_rate as f64).max(time_ms * 0.02);
                assert!(
                    (recovered_time_ms - time_ms).abs() <= timing_tolerance_ms,
                    "rate={sample_rate} coefficient={index} recovered={recovered_time_ms}"
                );
            }
            let effect = TransientShaperFactory
                .prepare(request_with(
                    &values,
                    sample_rate,
                    false,
                    LinkMode::DualMono,
                ))
                .expect("exact caps");
            let state = snapshot(effect.as_ref());
            let expected =
                expected_lane_bytes([0.0, 0.0], [(0.0, 0.0, 0), (0.0, 0.0, 0), (1.0, 1.0, 0)]);
            assert_eq!(state.0.len(), STATE_WORDS * 4);
            assert_eq!(state.0, expected);
            assert_eq!(state.1, expected);
        }

        let mut scratch_below = request(&values);
        scratch_below.limits.maximum_scratch_bytes = 23;
        assert!(matches!(
            TransientShaperFactory.prepare(scratch_below),
            Err(EffectPrepareError {
                code: "effect.resource.limit"
            })
        ));
        let mut negative_zero = values;
        negative_zero[0].value = -0.0;
        assert!(matches!(
            TransientShaperFactory.prepare(request(&negative_zero)),
            Err(EffectPrepareError {
                code: "effect.parameter.initial"
            })
        ));
    }

    #[test]
    fn scalar_matches_independent_f64_oracle_and_preserves_identity_bits() {
        let mut values = initial_values();
        values[0].value = 0.75;
        values[1].value = 0.75;
        values[2].value = -0.5;
        values[3].value = -0.5;
        let mut effect = prepare(&values);
        let mut reference = ReferenceTransientShaper::new(
            48_000.0,
            ReferenceTransientShaperParameters {
                attack_amount: 0.75,
                sustain_amount: -0.5,
                mix: 1.0,
            },
        )
        .expect("reference");
        let mut left = (0..96)
            .map(|index| ((index as f32 * 0.071).sin() * 0.7).max(-0.7))
            .collect::<Vec<_>>();
        let mut right = left.clone();
        let input = left.clone();
        effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("block"),
        );
        for (sample, original) in left.iter().zip(input) {
            let expected = reference.process_sample(original as f64, original.abs() as f64) as f32;
            assert!(
                (sample - expected).abs() < 2.0e-5,
                "sample={sample} expected={expected}"
            );
        }

        let defaults = initial_values();
        let mut identity = prepare(&defaults);
        let mut identity_left = [-0.0_f32, 0.25, -0.5, 0.0];
        let original = identity_left.map(f32::to_bits);
        let mut identity_right = identity_left;
        identity.process(
            EffectProcessBlock::new(&mut identity_left, &mut identity_right, None, 0, &[], 128)
                .expect("identity block"),
        );
        assert_eq!(identity_left.map(f32::to_bits), original);
        assert_eq!(identity_right.map(f32::to_bits), original);

        assert_eq!(
            linked_magnitudes(LinkMode::DualMono, -1.0, 0.25),
            (1.0, 0.25)
        );
        assert_eq!(linked_magnitudes(LinkMode::Maximum, -1.0, 0.25), (1.0, 1.0));
        assert_eq!(
            linked_magnitudes(LinkMode::Average, -1.0, 0.25),
            (0.625, 0.625)
        );
    }

    #[test]
    fn impulse_step_and_decay_cover_both_attack_and_sustain_signs() {
        let mut impulse = vec![0.0_f32; 32];
        impulse[0] = 1.0;
        let (attack_boost_error, _, attack_boost_maximum) =
            render_reference_row(1.0, 0.0, &impulse, 0);
        assert!(attack_boost_error <= 0.01, "error={attack_boost_error}");
        assert!(attack_boost_maximum > 0.25);

        let step = vec![1.0_f32; 64];
        let (attack_cut_error, attack_cut_minimum, _) = render_reference_row(-1.0, 0.0, &step, 0);
        assert!(attack_cut_error <= 0.01, "error={attack_cut_error}");
        assert!(attack_cut_minimum < -0.25);

        let mut decay = vec![1.0_f32; 4_800];
        decay.extend((0..512).map(|index| 0.9_f32 * 0.995_f32.powi(index)));
        let (sustain_boost_error, _, sustain_boost_maximum) =
            render_reference_row(0.0, 1.0, &decay, 4_800);
        assert!(sustain_boost_error <= 0.01, "error={sustain_boost_error}");
        assert!(sustain_boost_maximum > 0.25);
        let (sustain_cut_error, sustain_cut_minimum, _) =
            render_reference_row(0.0, -1.0, &decay, 4_800);
        assert!(sustain_cut_error <= 0.01, "error={sustain_cut_error}");
        assert!(sustain_cut_minimum < -0.25);
    }

    #[test]
    fn automation_updates_one_sixty_three_sixty_four_retargets_and_restores_exactly() {
        let values = initial_values();
        let mut effect = prepare(&values);
        let initial_right = snapshot(effect.as_ref()).1;
        let target_one = PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel: ParameterChannel::Left,
            parameter_index: 0,
            start_sample: 0,
            end_sample: 0,
            start_value: 1.0,
            end_value: 1.0,
        };
        let mut left = [0.0_f32];
        let mut right = [0.0_f32];
        effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[target_one], 128)
                .expect("first update"),
        );
        let after_one = snapshot(effect.as_ref());
        assert_eq!(
            state_f32(&after_one.0, 2).to_bits(),
            (1.0_f32 / 64.0).to_bits()
        );
        assert_eq!(state_f32(&after_one.0, 3).to_bits(), 1.0_f32.to_bits());
        assert_eq!(state_u32(&after_one.0, 4), 63);
        assert_eq!(after_one.1, initial_right);

        let mut left = [0.0_f32; 62];
        let mut right = [0.0_f32; 62];
        effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 1, &[], 128)
                .expect("updates two through sixty-three"),
        );
        let after_sixty_three = snapshot(effect.as_ref());
        assert_eq!(
            state_f32(&after_sixty_three.0, 2).to_bits(),
            (63.0_f32 / 64.0).to_bits()
        );
        assert_eq!(state_u32(&after_sixty_three.0, 4), 1);
        assert_eq!(after_sixty_three.1, initial_right);

        let mut left = [0.0_f32];
        let mut right = [0.0_f32];
        effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 63, &[], 128)
                .expect("update sixty-four"),
        );
        let after_sixty_four = snapshot(effect.as_ref());
        assert_eq!(
            state_f32(&after_sixty_four.0, 2).to_bits(),
            1.0_f32.to_bits()
        );
        assert_eq!(state_u32(&after_sixty_four.0, 4), 0);
        assert_eq!(after_sixty_four.1, initial_right);

        let retarget = PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel: ParameterChannel::Left,
            parameter_index: 0,
            start_sample: 64,
            end_sample: 64,
            start_value: -1.0,
            end_value: -1.0,
        };
        let mut left = [0.0_f32];
        let mut right = [0.0_f32];
        effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 64, &[retarget], 128)
                .expect("retarget update one"),
        );
        let active = snapshot(effect.as_ref());
        assert_eq!(state_f32(&active.0, 2).to_bits(), 0.96875_f32.to_bits());
        assert_eq!(state_f32(&active.0, 3).to_bits(), (-1.0_f32).to_bits());
        assert_eq!(state_u32(&active.0, 4), 63);
        assert_eq!(active.1, initial_right);

        let mut restored = prepare(&values);
        restored
            .restore_state_payload(
                1,
                StatePayloadInput::new(&[], &active.0, &active.1, restored.metadata().state_sizes)
                    .expect("active state"),
            )
            .expect("active restore");
        let mut uninterrupted_left = [0.25_f32; 8];
        let mut uninterrupted_right = [-0.125_f32; 8];
        let mut restored_left = uninterrupted_left;
        let mut restored_right = uninterrupted_right;
        effect.process(
            EffectProcessBlock::new(
                &mut uninterrupted_left,
                &mut uninterrupted_right,
                None,
                65,
                &[],
                128,
            )
            .expect("uninterrupted"),
        );
        restored.process(
            EffectProcessBlock::new(&mut restored_left, &mut restored_right, None, 65, &[], 128)
                .expect("restored"),
        );
        assert_eq!(
            uninterrupted_left.map(f32::to_bits),
            restored_left.map(f32::to_bits)
        );
        assert_eq!(
            uninterrupted_right.map(f32::to_bits),
            restored_right.map(f32::to_bits)
        );
        assert_eq!(snapshot(effect.as_ref()), snapshot(restored.as_ref()));
    }

    #[test]
    fn both_resets_have_word_exact_parameter_and_envelope_states() {
        let mut values = initial_values();
        values[0].value = 0.25;
        values[1].value = -0.25;
        values[2].value = 0.5;
        values[3].value = -0.5;
        values[4].value = 0.75;
        values[5].value = 0.5;
        let mut effect = prepare(&values);
        let span = PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel: ParameterChannel::Left,
            parameter_index: 0,
            start_sample: 0,
            end_sample: 0,
            start_value: 1.0,
            end_value: 1.0,
        };
        let mut left = [0.8_f32; 8];
        let mut right = [0.2_f32; 8];
        effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[span], 128)
                .expect("active state"),
        );

        effect.reset(ResetKind::DiscontinuityKeepParameters);
        let discontinuity = snapshot(effect.as_ref());
        assert_eq!(
            discontinuity.0,
            expected_lane_bytes([0.0, 0.0], [(1.0, 1.0, 0), (0.5, 0.5, 0), (0.75, 0.75, 0)])
        );
        assert_eq!(
            discontinuity.1,
            expected_lane_bytes(
                [0.0, 0.0],
                [(-0.25, -0.25, 0), (-0.5, -0.5, 0), (0.5, 0.5, 0)]
            )
        );

        effect.reset(ResetKind::FullToDefaults);
        let full = snapshot(effect.as_ref());
        assert_eq!(
            full.0,
            expected_lane_bytes(
                [0.0, 0.0],
                [(0.25, 0.25, 0), (0.5, 0.5, 0), (0.75, 0.75, 0)]
            )
        );
        assert_eq!(
            full.1,
            expected_lane_bytes(
                [0.0, 0.0],
                [(-0.25, -0.25, 0), (-0.5, -0.5, 0), (0.5, 0.5, 0)]
            )
        );
    }

    #[test]
    fn public_identity_sanitation_and_injected_recovery_are_exact_and_isolated() {
        let defaults = initial_values();
        let mut identity = prepare(&defaults);
        let mut identity_left = [-0.0_f32, 0.25, -0.5, 0.0];
        let mut identity_right = [0.0_f32, -0.125, 0.75, -0.0];
        let original_left = identity_left.map(f32::to_bits);
        let original_right = identity_right.map(f32::to_bits);
        identity.process(
            EffectProcessBlock::new(&mut identity_left, &mut identity_right, None, 0, &[], 128)
                .expect("default identity"),
        );
        assert_eq!(identity_left.map(f32::to_bits), original_left);
        assert_eq!(identity_right.map(f32::to_bits), original_right);
        let identity_state = snapshot(identity.as_ref());
        assert!(state_f32(&identity_state.0, 0) > 0.0);
        assert!(state_f32(&identity_state.0, 1) > 0.0);
        assert!(state_f32(&identity_state.1, 0) > 0.0);
        assert!(state_f32(&identity_state.1, 1) > 0.0);

        let mut active = initial_values();
        active[0].value = 1.0;
        active[1].value = 1.0;
        let mut bypass = prepare_concrete(&active, 48_000, true, LinkMode::DualMono);
        let mut bypass_left = [-0.0_f32, 0.8];
        let mut bypass_right = [0.0_f32, 0.4];
        let bypass_original_left = bypass_left.map(f32::to_bits);
        let bypass_original_right = bypass_right.map(f32::to_bits);
        bypass.process(
            EffectProcessBlock::new(&mut bypass_left, &mut bypass_right, None, 0, &[], 128)
                .expect("bypass identity"),
        );
        assert_eq!(bypass_left.map(f32::to_bits), bypass_original_left);
        assert_eq!(bypass_right.map(f32::to_bits), bypass_original_right);
        let bypass_state = snapshot(&bypass);
        assert!(state_f32(&bypass_state.0, 0) > 0.0);
        assert!(state_f32(&bypass_state.1, 0) > 0.0);

        active[4].value = 0.0;
        active[5].value = 0.0;
        let mut mix_zero = prepare(&active);
        let mut mix_left = [-0.0_f32, 0.8];
        let mut mix_right = [0.0_f32, 0.4];
        let mix_original_left = mix_left.map(f32::to_bits);
        let mix_original_right = mix_right.map(f32::to_bits);
        mix_zero.process(
            EffectProcessBlock::new(&mut mix_left, &mut mix_right, None, 0, &[], 128)
                .expect("mix-zero identity"),
        );
        assert_eq!(mix_left.map(f32::to_bits), mix_original_left);
        assert_eq!(mix_right.map(f32::to_bits), mix_original_right);
        let mix_state = snapshot(mix_zero.as_ref());
        assert!(state_f32(&mix_state.0, 0) > 0.0);
        assert!(state_f32(&mix_state.1, 0) > 0.0);

        let mut sanitized = prepare(&defaults);
        let mut sanitized_left = [f32::NAN];
        let mut sanitized_right = [f32::from_bits(1)];
        let sanitation_report = sanitized.process(
            EffectProcessBlock::new(&mut sanitized_left, &mut sanitized_right, None, 0, &[], 128)
                .expect("two sanitized lane samples"),
        );
        assert_eq!(sanitation_report.sanitized_main_samples, 2);
        assert_eq!(sanitized_left[0].to_bits(), 0.0_f32.to_bits());
        assert_eq!(sanitized_right[0].to_bits(), 0.0_f32.to_bits());

        active[4].value = 1.0;
        active[5].value = 1.0;
        let mut faulted = prepare_concrete(&active, 48_000, false, LinkMode::DualMono);
        let mut healthy = prepare_concrete(&active, 48_000, false, LinkMode::DualMono);
        faulted.left.fast = f32::INFINITY;
        let mut faulted_left = [-0.25_f32];
        let mut faulted_right = [0.5_f32];
        let mut healthy_left = faulted_left;
        let mut healthy_right = faulted_right;
        let faulted_report = faulted.process(
            EffectProcessBlock::new(&mut faulted_left, &mut faulted_right, None, 0, &[], 128)
                .expect("faulted public process"),
        );
        let healthy_report = healthy.process(
            EffectProcessBlock::new(&mut healthy_left, &mut healthy_right, None, 0, &[], 128)
                .expect("healthy public process"),
        );
        assert_eq!(faulted_left[0].to_bits(), (-0.25_f32).to_bits());
        assert_eq!(faulted_report.recovered_left_samples, 1);
        assert_eq!(faulted_report.recovered_right_samples, 0);
        assert_eq!(healthy_report.recovered_left_samples, 0);
        assert_eq!(healthy_report.recovered_right_samples, 0);
        assert_eq!(
            faulted_right.map(f32::to_bits),
            healthy_right.map(f32::to_bits)
        );
        let faulted_state = snapshot(&faulted);
        let healthy_state = snapshot(&healthy);
        assert_eq!(state_f32(&faulted_state.0, 0).to_bits(), 0.0_f32.to_bits());
        assert_eq!(state_f32(&faulted_state.0, 1).to_bits(), 0.0_f32.to_bits());
        assert_eq!(&faulted_state.0[8..], &healthy_state.0[8..]);
        assert_eq!(faulted_state.1, healthy_state.1);
    }

    #[test]
    fn links_automation_atomic_state_resets_sanitation_and_recovery_are_lane_local() {
        let mut values = initial_values();
        values[0].value = 1.0;
        values[1].value = 1.0;
        values[2].value = 1.0;
        values[3].value = 1.0;
        let mut effect = prepare(&values);
        let span = PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel: ParameterChannel::Left,
            parameter_index: 0,
            start_sample: 0,
            end_sample: 0,
            start_value: -1.0,
            end_value: -1.0,
        };
        let mut left = [0.8_f32; 64];
        let mut right = [0.1_f32; 64];
        let report = effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[span], 128).expect("block"),
        );
        assert_eq!(report.invalid_spans, 0);
        let active = snapshot(effect.as_ref());
        let mut restored = prepare(&values);
        restored
            .restore_state_payload(
                1,
                StatePayloadInput::new(&[], &active.0, &active.1, restored.metadata().state_sizes)
                    .expect("input"),
            )
            .expect("restore");
        let mut continuation_a_left = [0.35_f32; 16];
        let mut continuation_a_right = [-0.2_f32; 16];
        let mut continuation_b_left = continuation_a_left;
        let mut continuation_b_right = continuation_a_right;
        effect.process(
            EffectProcessBlock::new(
                &mut continuation_a_left,
                &mut continuation_a_right,
                None,
                64,
                &[],
                128,
            )
            .expect("continuation"),
        );
        restored.process(
            EffectProcessBlock::new(
                &mut continuation_b_left,
                &mut continuation_b_right,
                None,
                64,
                &[],
                128,
            )
            .expect("restored continuation"),
        );
        assert_eq!(
            continuation_a_left.map(f32::to_bits),
            continuation_b_left.map(f32::to_bits)
        );
        assert_eq!(
            continuation_a_right.map(f32::to_bits),
            continuation_b_right.map(f32::to_bits)
        );

        let before_invalid = snapshot(effect.as_ref());
        let mut invalid_left = before_invalid.0.clone();
        invalid_left[..4].copy_from_slice(&f32::NAN.to_bits().to_le_bytes());
        assert_eq!(
            effect.restore_state_payload(
                1,
                StatePayloadInput::new(
                    &[],
                    &invalid_left,
                    &before_invalid.1,
                    effect.metadata().state_sizes,
                )
                .expect("invalid input"),
            ),
            Err(StatePayloadError {
                code: "effect.state.envelope"
            })
        );
        assert_eq!(snapshot(effect.as_ref()), before_invalid);

        let mut lane = Lane::new(&[1.0, 0.0, 1.0]);
        lane.fast = f32::INFINITY;
        let mut recovered = 0;
        assert_eq!(
            process_lane(
                -0.25,
                0.25,
                coefficients(48_000).expect("coefficients"),
                false,
                &mut lane,
                &mut recovered,
            )
            .to_bits(),
            (-0.25_f32).to_bits()
        );
        assert_eq!(recovered, 1);
        assert_eq!(lane.fast.to_bits(), 0.0_f32.to_bits());
        assert_eq!(lane.slow.to_bits(), 0.0_f32.to_bits());

        effect.reset(ResetKind::DiscontinuityKeepParameters);
        let discontinuity = snapshot(effect.as_ref());
        effect.reset(ResetKind::FullToDefaults);
        let full = snapshot(effect.as_ref());
        assert_ne!(discontinuity, full);
        let mut sanitized_left = [f32::NAN];
        let mut sanitized_right = [0.0_f32];
        let report = effect.process(
            EffectProcessBlock::new(
                &mut sanitized_left,
                &mut sanitized_right,
                None,
                80,
                &[],
                128,
            )
            .expect("sanitize block"),
        );
        assert_eq!(report.sanitized_main_samples, 1);
        assert_eq!(sanitized_left[0].to_bits(), 0.0_f32.to_bits());
    }
}
