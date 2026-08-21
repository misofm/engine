//! Fixed two-second integer-time dual-mono and ping-pong delay.
//!
//! Rendering owns two prepared scalar rings. Reset and recovery invalidate them logically rather
//! than clearing them on the audio thread; snapshots canonicalize every logically invalid cell.
#![allow(missing_docs)]

use miso_engine_effect_contract::{
    AutomationRate, AutomationSpanKind, EffectDescriptorV1, EffectPrepareError, EffectProcessBlock,
    EffectQuality, InitialParameterValue, LatencySamples, LinkModeSet, NativeEffectFactory,
    ParameterChannel, ParameterChannelPolicy, ParameterDescriptorV1, ParameterDomain, ParameterId,
    ParameterMapping, ParameterUnit, PortDescriptorV1, PortId, PortLayout, PortRole,
    PrepareEffectBankRequest, PrepareEffectRequest, PreparedAutomationSpan, PreparedEffectMetadata,
    PreparedNativeEffect, ProcessReport, ResetKind, SmoothingRule, StatePayloadError,
    StatePayloadInput, StatePayloadOutput, StatePayloadSizes, TailSamples,
    expected_prepared_metadata, sanitize_sample,
};

const PER_LANE_PARAMETER_COUNT: usize = 4;
const ORDINARY_RAMP_COUNT: usize = 3;
const PARAMETER_COUNT: usize = 5;
const RAMP_SAMPLES: u32 = 64;
const TRANSITION_SAMPLES: u32 = 128;
const COMMON_BYTES: u32 = 16;
const FIXED_BYTES: u64 = 36;

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
    policy: ParameterChannelPolicy,
    minimum: f32,
    maximum: f32,
    default_value: f32,
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
        mapping: ParameterMapping::Linear,
        automation_rate: AutomationRate::Block,
        channel_policy: policy,
        smoothing: SmoothingRule::Linear,
        smoothing_samples,
        readable: true,
        automatable: true,
        enum_choices: &[],
    }
}

/// Frozen V1 delay parameters in descriptor and stable-ID order.
pub const DELAY_PARAMETERS_V1: [ParameterDescriptorV1; PARAMETER_COUNT] = [
    parameter(
        1,
        "delay time",
        "ms",
        ParameterUnit::Milliseconds,
        ParameterChannelPolicy::PerLane,
        1.0,
        2000.0,
        250.0,
        TRANSITION_SAMPLES,
    ),
    parameter(
        2,
        "feedback",
        "linear",
        ParameterUnit::Linear,
        ParameterChannelPolicy::PerLane,
        -0.95,
        0.95,
        0.35,
        RAMP_SAMPLES,
    ),
    parameter(
        3,
        "damping",
        "linear",
        ParameterUnit::Linear,
        ParameterChannelPolicy::PerLane,
        0.0,
        0.995,
        0.25,
        RAMP_SAMPLES,
    ),
    parameter(
        4,
        "mix",
        "linear",
        ParameterUnit::Linear,
        ParameterChannelPolicy::PerLane,
        0.0,
        1.0,
        0.35,
        RAMP_SAMPLES,
    ),
    parameter(
        5,
        "cross feedback",
        "linear",
        ParameterUnit::Linear,
        ParameterChannelPolicy::Shared,
        0.0,
        1.0,
        0.0,
        RAMP_SAMPLES,
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

const fn quality(sample_rate: u32) -> miso_engine_effect_contract::QualityDescriptorV1 {
    let ring_words = sample_rate * 2 + 3;
    let lane_bytes = (ring_words + 16) * 4;
    miso_engine_effect_contract::QualityDescriptorV1 {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(0),
        tail: TailSamples::Infinite,
        maximum_state: StatePayloadSizes {
            common_bytes: COMMON_BYTES,
            left_bytes: lane_bytes,
            right_bytes: lane_bytes,
        },
        scratch_fixed_bytes: FIXED_BYTES,
        scratch_bytes_per_frame: 0,
    }
}

const QUALITIES: [miso_engine_effect_contract::QualityDescriptorV1; 4] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
];

/// Immutable descriptor for the fixed integer-time delay V1 contract.
pub const DELAY_DESCRIPTOR_V1: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("miso.delay"),
    display_name: "Dual-Mono / Ping-Pong Delay",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::DUAL_MONO,
    parameters: &DELAY_PARAMETERS_V1,
    ports: &PORTS,
    qualities: &QUALITIES,
};

/// Factory for the launch scalar delay. There is intentionally no homogeneous bank kernel.
#[derive(Clone, Copy, Debug, Default)]
pub struct DelayFactory;

#[derive(Clone, Copy, Debug)]
struct Resources {
    max_delay: u32,
    ring_words: usize,
    lane_bytes: u32,
}

fn resources(sample_rate: u32) -> Option<Resources> {
    let max_delay = sample_rate.checked_mul(2)?;
    let ring_words_u32 = max_delay.checked_add(3)?;
    let lane_words = ring_words_u32.checked_add(16)?;
    let lane_bytes = lane_words.checked_mul(4)?;
    let ring_words = usize::try_from(ring_words_u32).ok()?;
    if usize::try_from(lane_bytes).is_err() || isize::try_from(lane_bytes).is_err() {
        return None;
    }
    Some(Resources {
        max_delay,
        ring_words,
        lane_bytes,
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

    fn snap_to_target(&mut self) {
        self.current = self.target;
        self.remaining = 0;
    }
}

#[derive(Debug)]
struct DelayLane {
    damping_state: f32,
    delay_target_ms: f32,
    active_delay: u32,
    transition_delay: u32,
    pending_delay: u32,
    transition_remaining: u32,
    valid_history: u32,
    ramps: [Ramp; ORDINARY_RAMP_COUNT],
    ring: Box<[f32]>,
}

impl DelayLane {
    fn new(
        defaults: &[f32; PER_LANE_PARAMETER_COUNT],
        resources: Resources,
        sample_rate: u32,
    ) -> Option<Self> {
        let delay = delay_samples(defaults[0], sample_rate, resources.max_delay)?;
        Some(Self {
            damping_state: 0.0,
            delay_target_ms: defaults[0],
            active_delay: delay,
            transition_delay: delay,
            pending_delay: delay,
            transition_remaining: 0,
            valid_history: 0,
            ramps: [
                Ramp::fixed(defaults[1]),
                Ramp::fixed(defaults[2]),
                Ramp::fixed(defaults[3]),
            ],
            ring: vec![0.0; resources.ring_words].into_boxed_slice(),
        })
    }

    fn full_reset(
        &mut self,
        defaults: &[f32; PER_LANE_PARAMETER_COUNT],
        sample_rate: u32,
        max_delay: u32,
    ) {
        let delay = delay_samples(defaults[0], sample_rate, max_delay)
            .expect("prepared delay default remains valid");
        self.damping_state = 0.0;
        self.delay_target_ms = defaults[0];
        self.active_delay = delay;
        self.transition_delay = delay;
        self.pending_delay = delay;
        self.transition_remaining = 0;
        self.valid_history = 0;
        self.ramps = [
            Ramp::fixed(defaults[1]),
            Ramp::fixed(defaults[2]),
            Ramp::fixed(defaults[3]),
        ];
    }

    fn discontinuity_reset(&mut self, sample_rate: u32, max_delay: u32) {
        let delay = delay_samples(self.delay_target_ms, sample_rate, max_delay)
            .expect("validated delay target remains valid");
        self.damping_state = 0.0;
        self.active_delay = delay;
        self.transition_delay = delay;
        self.pending_delay = delay;
        self.transition_remaining = 0;
        self.valid_history = 0;
        for ramp in &mut self.ramps {
            ramp.snap_to_target();
        }
    }

    fn begin_transition(&mut self) {
        if self.transition_remaining == 0 && self.pending_delay != self.active_delay {
            self.transition_delay = self.pending_delay;
            self.transition_remaining = TRANSITION_SAMPLES;
        }
    }

    fn tap(&self, cursor: usize) -> Result<f32, ()> {
        self.tap_at(cursor, self.active_delay)
    }

    fn tap_at(&self, cursor: usize, delay: u32) -> Result<f32, ()> {
        if delay > self.valid_history {
            return Ok(0.0);
        }
        let delay = usize::try_from(delay).map_err(|_| ())?;
        let index = (cursor + self.ring.len() - delay) % self.ring.len();
        flush(self.ring[index]).ok_or(())
    }

    fn read_transition(&mut self, cursor: usize) -> Result<f32, ()> {
        let remaining = self.transition_remaining;
        if remaining == 0 {
            return self.tap(cursor);
        }
        let new = self.tap_at(cursor, self.transition_delay)?;
        if remaining == 1 {
            self.active_delay = self.transition_delay;
            self.transition_remaining = 0;
            return Ok(new);
        }
        let old = self.tap_at(cursor, self.active_delay)?;
        self.transition_remaining -= 1;
        let update = 129_u32 - remaining;
        let alpha = update as f32 * (1.0_f32 / 128.0_f32);
        let delta = new - old;
        let scaled = alpha * delta;
        flush(old + scaled).ok_or(())
    }

    fn filter(&mut self, tap: f32) -> Result<f32, ()> {
        let damping = self.ramps[1].current;
        let value = if damping == 0.0 {
            tap
        } else {
            let complement = 1.0_f32 - damping;
            let first = complement * tap;
            let second = damping * self.damping_state;
            flush(first + second).ok_or(())?
        };
        let value = flush(value).ok_or(())?;
        self.damping_state = value;
        Ok(value)
    }

    fn write(&mut self, cursor: usize, dry: f32, feedback: f32) -> Result<(), ()> {
        let value = if feedback == 0.0 {
            dry
        } else {
            flush(dry + feedback).ok_or(())?
        };
        self.ring[cursor] = value;
        self.valid_history = self
            .valid_history
            .saturating_add(1)
            .min(u32::try_from(self.ring.len()).map_err(|_| ())?);
        Ok(())
    }

    fn recover(&mut self, recovered: &mut u64) {
        self.damping_state = 0.0;
        self.valid_history = 0;
        *recovered = recovered.saturating_add(1);
    }
}

/// Prepared scalar delay state. The ring shape and metadata are immutable after preparation.
#[derive(Debug)]
pub struct PreparedDelay {
    metadata: PreparedEffectMetadata,
    resources: Resources,
    left_defaults: [f32; PER_LANE_PARAMETER_COUNT],
    right_defaults: [f32; PER_LANE_PARAMETER_COUNT],
    cross_default: f32,
    cursor: usize,
    cross: Ramp,
    left: DelayLane,
    right: DelayLane,
}

impl NativeEffectFactory for DelayFactory {
    fn descriptor(&self) -> &'static EffectDescriptorV1 {
        &DELAY_DESCRIPTOR_V1
    }

    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        Ok(Box::new(prepare_delay(request)?))
    }

    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<
        Option<Box<dyn miso_engine_effect_contract::PreparedNativeEffectBank>>,
        EffectPrepareError,
    > {
        if !request.has_matching_backend_width()
            || request.requests.len() != request.width.lanes() as usize
        {
            return Err(EffectPrepareError {
                code: "effect.bank.requests",
            });
        }
        for member in request.requests.iter().copied() {
            let _ = validate_inputs(member)?;
        }
        // A variable gathered two-second ring has no accepted W4/W8 core kernel. Every validated
        // request therefore remains a legal, ordered scalar member.
        Ok(None)
    }
}

fn prepare_delay(request: PrepareEffectRequest<'_>) -> Result<PreparedDelay, EffectPrepareError> {
    let (metadata, resources, left_defaults, right_defaults, cross_default) =
        validate_inputs(request)?;
    let left = DelayLane::new(&left_defaults, resources, metadata.sample_rate).ok_or(
        EffectPrepareError {
            code: "effect.parameter.initial",
        },
    )?;
    let right = DelayLane::new(&right_defaults, resources, metadata.sample_rate).ok_or(
        EffectPrepareError {
            code: "effect.parameter.initial",
        },
    )?;
    Ok(PreparedDelay {
        metadata,
        resources,
        left_defaults,
        right_defaults,
        cross_default,
        cursor: 0,
        cross: Ramp::fixed(cross_default),
        left,
        right,
    })
}

type ValidatedInputs = (
    PreparedEffectMetadata,
    Resources,
    [f32; PER_LANE_PARAMETER_COUNT],
    [f32; PER_LANE_PARAMETER_COUNT],
    f32,
);

fn validate_inputs(
    request: PrepareEffectRequest<'_>,
) -> Result<ValidatedInputs, EffectPrepareError> {
    let metadata = expected_prepared_metadata(&DELAY_DESCRIPTOR_V1, request)?;
    let resources = resources(metadata.sample_rate).ok_or(EffectPrepareError {
        code: "effect.resource.limit",
    })?;
    if metadata.state_sizes.common_bytes != COMMON_BYTES
        || metadata.state_sizes.left_bytes != resources.lane_bytes
        || metadata.state_sizes.right_bytes != resources.lane_bytes
        || metadata.scratch_bytes != FIXED_BYTES
    {
        return Err(EffectPrepareError {
            code: "effect.resource.limit",
        });
    }
    let (left, right, cross) = initial_defaults(request.initial_values)?;
    Ok((metadata, resources, left, right, cross))
}

fn initial_defaults(
    values: &[InitialParameterValue],
) -> Result<
    (
        [f32; PER_LANE_PARAMETER_COUNT],
        [f32; PER_LANE_PARAMETER_COUNT],
        f32,
    ),
    EffectPrepareError,
> {
    if values.len() != 9 {
        return Err(EffectPrepareError {
            code: "effect.parameter.initial",
        });
    }
    let mut left = [0.0; PER_LANE_PARAMETER_COUNT];
    let mut right = [0.0; PER_LANE_PARAMETER_COUNT];
    for index in 0..PER_LANE_PARAMETER_COUNT {
        let left_value = values[index * 2];
        let right_value = values[index * 2 + 1];
        let parameter = &DELAY_PARAMETERS_V1[index];
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
    let cross = values[8];
    if cross.parameter_index != 4
        || cross.channel != ParameterChannel::Both
        || !parameter_value_valid(&DELAY_PARAMETERS_V1[4], cross.value)
        || negative_zero(cross.value)
    {
        return Err(EffectPrepareError {
            code: "effect.parameter.initial",
        });
    }
    Ok((left, right, normalize_zero(cross.value)))
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

fn delay_samples(milliseconds: f32, sample_rate: u32, maximum: u32) -> Option<u32> {
    if !milliseconds.is_finite() || !(1.0..=2000.0).contains(&milliseconds) {
        return None;
    }
    let rounded = (milliseconds as f64 * sample_rate as f64 / 1000.0 + 0.5).floor();
    if !rounded.is_finite() || rounded < 1.0 || rounded > maximum as f64 {
        return None;
    }
    let value = u32::try_from(rounded as u64).ok()?;
    (1..=maximum).contains(&value).then_some(value)
}

impl PreparedNativeEffect for PreparedDelay {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.metadata
    }

    fn reset(&mut self, kind: ResetKind) {
        match kind {
            ResetKind::FullToDefaults => {
                self.cursor = 0;
                self.cross = Ramp::fixed(self.cross_default);
                self.left.full_reset(
                    &self.left_defaults,
                    self.metadata.sample_rate,
                    self.resources.max_delay,
                );
                self.right.full_reset(
                    &self.right_defaults,
                    self.metadata.sample_rate,
                    self.resources.max_delay,
                );
            }
            ResetKind::DiscontinuityKeepParameters => {
                self.cursor = 0;
                self.cross.snap_to_target();
                self.left
                    .discontinuity_reset(self.metadata.sample_rate, self.resources.max_delay);
                self.right
                    .discontinuity_reset(self.metadata.sample_rate, self.resources.max_delay);
            }
        }
    }

    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        let mut report = ProcessReport::default();
        apply_automation(
            block.automation,
            self.metadata,
            block.first_sample,
            self.metadata.sample_rate,
            self.resources.max_delay,
            &mut self.left,
            &mut self.right,
            &mut self.cross,
            &mut report,
        );
        for index in 0..block.frames() {
            self.left.ramps.iter_mut().for_each(Ramp::advance);
            self.right.ramps.iter_mut().for_each(Ramp::advance);
            self.cross.advance();
            self.left.begin_transition();
            self.right.begin_transition();
            let left_dry = sanitize(block.left[index], &mut report.sanitized_main_samples);
            let right_dry = sanitize(block.right[index], &mut report.sanitized_main_samples);
            let (left_tap, left_filtered, mut left_healthy) =
                match self.left.read_transition(self.cursor) {
                    Ok(tap) => match self.left.filter(tap) {
                        Ok(filtered) => (tap, filtered, true),
                        Err(()) => (0.0, 0.0, false),
                    },
                    Err(()) => (0.0, 0.0, false),
                };
            let (right_tap, right_filtered, mut right_healthy) =
                match self.right.read_transition(self.cursor) {
                    Ok(tap) => match self.right.filter(tap) {
                        Ok(filtered) => (tap, filtered, true),
                        Err(()) => (0.0, 0.0, false),
                    },
                    Err(()) => (0.0, 0.0, false),
                };
            if !left_healthy {
                self.left.recover(&mut report.recovered_left_samples);
            }
            if !right_healthy {
                self.right.recover(&mut report.recovered_right_samples);
            }
            let left_gain = if left_healthy {
                match flush(self.left.ramps[0].current * left_filtered) {
                    Some(value) => value,
                    None => {
                        self.left.recover(&mut report.recovered_left_samples);
                        left_healthy = false;
                        0.0
                    }
                }
            } else {
                0.0
            };
            let right_gain = if right_healthy {
                match flush(self.right.ramps[0].current * right_filtered) {
                    Some(value) => value,
                    None => {
                        self.right.recover(&mut report.recovered_right_samples);
                        right_healthy = false;
                        0.0
                    }
                }
            } else {
                0.0
            };
            let (mut left_feedback, mut right_feedback) =
                match feedback_matrix(self.cross.current, left_gain, right_gain) {
                    Some(values) => values,
                    None => {
                        if left_healthy {
                            self.left.recover(&mut report.recovered_left_samples);
                            left_healthy = false;
                        }
                        if right_healthy {
                            self.right.recover(&mut report.recovered_right_samples);
                            right_healthy = false;
                        }
                        (0.0, 0.0)
                    }
                };
            if !left_healthy {
                left_feedback = 0.0;
            }
            if !right_healthy {
                right_feedback = 0.0;
            }
            if left_healthy
                && self
                    .left
                    .write(self.cursor, left_dry, left_feedback)
                    .is_err()
            {
                self.left.recover(&mut report.recovered_left_samples);
                left_healthy = false;
            }
            if right_healthy
                && self
                    .right
                    .write(self.cursor, right_dry, right_feedback)
                    .is_err()
            {
                self.right.recover(&mut report.recovered_right_samples);
                right_healthy = false;
            }
            block.left[index] = if left_healthy {
                mix_output(
                    left_dry,
                    left_tap,
                    self.left.ramps[2].current,
                    self.metadata.bypass,
                    &mut self.left,
                    &mut report.recovered_left_samples,
                )
            } else {
                left_dry
            };
            block.right[index] = if right_healthy {
                mix_output(
                    right_dry,
                    right_tap,
                    self.right.ramps[2].current,
                    self.metadata.bypass,
                    &mut self.right,
                    &mut report.recovered_right_samples,
                )
            } else {
                right_dry
            };
            self.cursor += 1;
            if self.cursor == self.resources.ring_words {
                self.cursor = 0;
            }
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
        write_u32(
            output.common,
            0,
            u32::try_from(self.cursor).map_err(|_| state_error("effect.state.cursor"))?,
        );
        write_ramp(output.common, 1, self.cross);
        write_lane(output.left, &self.left, self.cursor);
        write_lane(output.right, &self.right, self.cursor);
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
        let cursor = usize::try_from(read_u32(input.common, 0))
            .map_err(|_| state_error("effect.state.cursor"))?;
        if cursor >= self.resources.ring_words {
            return Err(state_error("effect.state.cursor"));
        }
        let cross = read_ramp(input.common, 1, &DELAY_PARAMETERS_V1[4])?;
        let left = read_lane(
            input.left,
            cursor,
            self.resources,
            self.metadata.sample_rate,
        )?;
        let right = read_lane(
            input.right,
            cursor,
            self.resources,
            self.metadata.sample_rate,
        )?;
        self.cursor = cursor;
        self.cross = cross;
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

fn flush(value: f32) -> Option<f32> {
    if !value.is_finite() {
        None
    } else if value.is_subnormal() {
        Some(0.0)
    } else {
        Some(value)
    }
}

fn feedback_matrix(cross: f32, left: f32, right: f32) -> Option<(f32, f32)> {
    if cross == 0.0 {
        return Some((left, right));
    }
    if cross == 1.0 {
        return Some((right, left));
    }
    let opposite = flush(1.0_f32 - cross)?;
    let left_cross = flush(cross * left)?;
    let right_cross = flush(cross * right)?;
    let left_direct = flush(opposite * left)?;
    let right_direct = flush(opposite * right)?;
    let left_feedback = flush(left_direct + right_cross)?;
    let right_feedback = flush(left_cross + right_direct)?;
    Some((left_feedback, right_feedback))
}

fn mix_output(
    dry: f32,
    wet: f32,
    mix: f32,
    bypass: bool,
    lane: &mut DelayLane,
    recovered: &mut u64,
) -> f32 {
    if bypass || mix == 0.0 {
        return dry;
    }
    if mix == 1.0 {
        return wet;
    }
    let delta = wet - dry;
    let scaled = mix * delta;
    match flush(dry + scaled) {
        Some(value) => value,
        None => {
            lane.recover(recovered);
            dry
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn apply_automation(
    spans: &[PreparedAutomationSpan],
    metadata: PreparedEffectMetadata,
    first_sample: u64,
    sample_rate: u32,
    maximum_delay: u32,
    left: &mut DelayLane,
    right: &mut DelayLane,
    cross: &mut Ramp,
    report: &mut ProcessReport,
) {
    let mut delay_pending = [None; 2];
    let mut ordinary_pending = [[None; ORDINARY_RAMP_COUNT]; 2];
    let mut cross_pending = None;
    let mut last_order = None;
    for (span_index, span) in spans.iter().enumerate() {
        let parameter_index = span.parameter_index as usize;
        let (order, lane_index) = match (parameter_index, span.channel) {
            (0..=3, ParameterChannel::Left) => (span.parameter_index * 2, Some(0)),
            (0..=3, ParameterChannel::Right) => (span.parameter_index * 2 + 1, Some(1)),
            (4, ParameterChannel::Both) => (8, None),
            _ => {
                report.invalid_spans = report.invalid_spans.saturating_add(1);
                continue;
            }
        };
        let Some(parameter) = DELAY_PARAMETERS_V1.get(parameter_index) else {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        };
        let valid = span_index < metadata.automation_capacity as usize
            && span.kind == AutomationSpanKind::Point
            && span.start_sample == first_sample
            && span.end_sample == first_sample
            && span.start_value.to_bits() == span.end_value.to_bits()
            && parameter_value_valid(parameter, span.start_value)
            && last_order.is_none_or(|previous| order > previous)
            && match (parameter_index, lane_index) {
                (0, Some(lane)) => delay_pending[lane].is_none(),
                (1..=3, Some(lane)) => ordinary_pending[lane][parameter_index - 1].is_none(),
                (4, None) => cross_pending.is_none(),
                _ => false,
            };
        if !valid {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        }
        last_order = Some(order);
        let value = normalize_zero(span.start_value);
        match (parameter_index, lane_index) {
            (0, Some(lane)) => delay_pending[lane] = Some(value),
            (1..=3, Some(lane)) => ordinary_pending[lane][parameter_index - 1] = Some(value),
            (4, None) => cross_pending = Some(value),
            _ => unreachable!("validated delay automation shape"),
        }
    }
    for (lane_index, lane) in [left, right].into_iter().enumerate() {
        if let Some(value) = delay_pending[lane_index] {
            let delay = delay_samples(value, sample_rate, maximum_delay)
                .expect("validated delay automation maps inside prepared range");
            lane.delay_target_ms = value;
            lane.pending_delay = delay;
        }
        for (ramp_index, value) in ordinary_pending[lane_index].iter().enumerate() {
            if let Some(value) = *value {
                lane.ramps[ramp_index].target = value;
                lane.ramps[ramp_index].remaining = RAMP_SAMPLES;
            }
        }
    }
    if let Some(value) = cross_pending {
        cross.target = value;
        cross.remaining = RAMP_SAMPLES;
    }
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

fn write_lane(bytes: &mut [u8], lane: &DelayLane, cursor: usize) {
    write_f32(bytes, 0, lane.damping_state);
    write_f32(bytes, 1, lane.delay_target_ms);
    write_u32(bytes, 2, lane.active_delay);
    write_u32(bytes, 3, lane.transition_delay);
    write_u32(bytes, 4, lane.pending_delay);
    write_u32(bytes, 5, lane.transition_remaining);
    write_u32(bytes, 6, lane.valid_history);
    for (index, ramp) in lane.ramps.iter().copied().enumerate() {
        write_ramp(bytes, 7 + index * 3, ramp);
    }
    for (index, value) in lane.ring.iter().copied().enumerate() {
        write_f32(
            bytes,
            16 + index,
            if valid_ring_cell(cursor, lane.valid_history, lane.ring.len(), index) {
                value
            } else {
                0.0
            },
        );
    }
}

fn read_lane(
    bytes: &[u8],
    cursor: usize,
    resources: Resources,
    sample_rate: u32,
) -> Result<DelayLane, StatePayloadError> {
    let damping_state = read_f32(bytes, 0);
    let delay_target_ms = read_f32(bytes, 1);
    let active_delay = read_u32(bytes, 2);
    let transition_delay = read_u32(bytes, 3);
    let pending_delay = read_u32(bytes, 4);
    let transition_remaining = read_u32(bytes, 5);
    let valid_history = read_u32(bytes, 6);
    if !normal_or_zero(damping_state)
        || !parameter_value_valid(&DELAY_PARAMETERS_V1[0], delay_target_ms)
        || negative_zero(delay_target_ms)
        || !valid_delay(active_delay, resources.max_delay)
        || !valid_delay(transition_delay, resources.max_delay)
        || !valid_delay(pending_delay, resources.max_delay)
        || transition_remaining > TRANSITION_SAMPLES
        || valid_history
            > u32::try_from(resources.ring_words)
                .map_err(|_| state_error("effect.state.history"))?
        || (transition_remaining == 0 && active_delay != transition_delay)
        || (transition_remaining != 0 && active_delay == transition_delay)
        || delay_samples(delay_target_ms, sample_rate, resources.max_delay) != Some(pending_delay)
    {
        return Err(state_error("effect.state.lane"));
    }
    let mut ramps = [Ramp::fixed(0.0); ORDINARY_RAMP_COUNT];
    for (index, ramp) in ramps.iter_mut().enumerate() {
        *ramp = read_ramp(bytes, 7 + index * 3, &DELAY_PARAMETERS_V1[index + 1])?;
    }
    let mut ring = vec![0.0; resources.ring_words].into_boxed_slice();
    for (index, value) in ring.iter_mut().enumerate() {
        let state_value = read_f32(bytes, 16 + index);
        if !normal_or_zero(state_value)
            || (!valid_ring_cell(cursor, valid_history, resources.ring_words, index)
                && state_value.to_bits() != 0.0_f32.to_bits())
        {
            return Err(state_error("effect.state.history"));
        }
        *value = state_value;
    }
    Ok(DelayLane {
        damping_state,
        delay_target_ms,
        active_delay,
        transition_delay,
        pending_delay,
        transition_remaining,
        valid_history,
        ramps,
        ring,
    })
}

fn valid_delay(value: u32, maximum: u32) -> bool {
    (1..=maximum).contains(&value)
}

fn valid_ring_cell(cursor: usize, valid_history: u32, ring_words: usize, index: usize) -> bool {
    if valid_history as usize >= ring_words {
        return true;
    }
    let age = (cursor + ring_words - index) % ring_words;
    age != 0 && age <= valid_history as usize
}

fn normal_or_zero(value: f32) -> bool {
    value.is_normal() || value == 0.0
}

fn write_ramp(bytes: &mut [u8], word: usize, ramp: Ramp) {
    write_f32(bytes, word, ramp.current);
    write_f32(bytes, word + 1, ramp.target);
    write_u32(bytes, word + 2, ramp.remaining);
}

fn read_ramp(
    bytes: &[u8],
    word: usize,
    parameter: &ParameterDescriptorV1,
) -> Result<Ramp, StatePayloadError> {
    let current = read_f32(bytes, word);
    let target = read_f32(bytes, word + 1);
    let remaining = read_u32(bytes, word + 2);
    if !parameter_value_valid(parameter, current)
        || !parameter_value_valid(parameter, target)
        || remaining > RAMP_SAMPLES
    {
        return Err(state_error("effect.state.parameter"));
    }
    Ok(Ramp {
        current: normalize_zero(current),
        target: normalize_zero(target),
        remaining,
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
    use miso_engine_dsp_reference::{ReferenceDelayPair, ReferenceDelayParameters};
    use miso_engine_effect_contract::{
        EffectProcessBlock, InitialParameterValue, LinkMode, PreparedNativeEffect,
        StatePayloadInput, StatePayloadOutput, validate_descriptor_v1,
    };

    fn initial_values() -> [InitialParameterValue; 9] {
        let mut values = core::array::from_fn(|index| InitialParameterValue {
            parameter_index: (index / 2) as u32,
            channel: if index % 2 == 0 {
                ParameterChannel::Left
            } else {
                ParameterChannel::Right
            },
            value: DELAY_PARAMETERS_V1[index / 2].default_value,
        });
        values[8] = InitialParameterValue {
            parameter_index: 4,
            channel: ParameterChannel::Both,
            value: DELAY_PARAMETERS_V1[4].default_value,
        };
        values
    }

    fn request<'a>(
        values: &'a [InitialParameterValue],
        sample_rate: u32,
    ) -> PrepareEffectRequest<'a> {
        let resources = resources(sample_rate).expect("launch resources");
        PrepareEffectRequest {
            sample_rate,
            quantum: 128,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: miso_engine_effect_contract::PreparedPortsV1 {
                sidechain: miso_engine_effect_contract::PreparedSidechainPort::None,
            },
            initial_values: values,
            limits: miso_engine_effect_contract::PrepareEffectLimits {
                maximum_total_state_bytes: u64::from(COMMON_BYTES)
                    + 2 * u64::from(resources.lane_bytes),
                maximum_scratch_bytes: FIXED_BYTES,
                maximum_automation_spans_per_block: 16,
            },
        }
    }

    fn prepare(values: &[InitialParameterValue]) -> PreparedDelay {
        prepare_delay(request(values, 48_000)).expect("prepare")
    }

    fn snapshot(effect: &dyn PreparedNativeEffect) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        let sizes = effect.metadata().state_sizes;
        let mut common = vec![0; sizes.common_bytes as usize];
        let mut left = vec![0; sizes.left_bytes as usize];
        let mut right = vec![0; sizes.right_bytes as usize];
        effect
            .snapshot_state_payload(
                StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("output"),
            )
            .expect("snapshot");
        (common, left, right)
    }

    #[test]
    fn descriptor_exact_resources_caps_and_integer_mapping_are_frozen() {
        validate_descriptor_v1(&DELAY_DESCRIPTOR_V1).expect("descriptor");
        assert_eq!(DELAY_DESCRIPTOR_V1.id.as_str(), "miso.delay");
        assert_eq!(
            DELAY_DESCRIPTOR_V1.supported_link_modes,
            LinkModeSet::DUAL_MONO
        );
        let expected = [
            (44_100, 88_203, 352_876, 705_768),
            (48_000, 96_003, 384_076, 768_168),
            (88_200, 176_403, 705_676, 1_411_368),
            (96_000, 192_003, 768_076, 1_536_168),
        ];
        let values = initial_values();
        for (sample_rate, ring_words, lane_bytes, total_state) in expected {
            let resource = resources(sample_rate).expect("resources");
            assert_eq!(resource.ring_words, ring_words);
            assert_eq!(resource.lane_bytes, lane_bytes);
            let prepared = prepare_delay(request(&values, sample_rate)).expect("prepare");
            assert_eq!(prepared.metadata.state_sizes.total(), Some(total_state));
            assert_eq!(prepared.metadata.latency, LatencySamples(0));
            assert_eq!(prepared.metadata.tail, TailSamples::Infinite);
            assert_eq!(
                delay_samples(2000.0, sample_rate, resource.max_delay),
                Some(resource.max_delay)
            );
        }
        let mut too_small = request(&values, 48_000);
        too_small.limits.maximum_total_state_bytes -= 1;
        assert!(matches!(
            prepare_delay(too_small),
            Err(EffectPrepareError {
                code: "effect.resource.limit"
            })
        ));
        let mut too_small_fixed = request(&values, 48_000);
        too_small_fixed.limits.maximum_scratch_bytes -= 1;
        assert!(matches!(
            prepare_delay(too_small_fixed),
            Err(EffectPrepareError {
                code: "effect.resource.limit"
            })
        ));
    }

    #[test]
    fn integer_impulse_default_tail_and_matrix_match_independent_oracle() {
        let mut values = initial_values();
        for index in [0, 1] {
            values[index].value = 1.0;
        }
        values[2].value = 0.5;
        values[3].value = 0.5;
        values[4].value = 0.0;
        values[5].value = 0.0;
        values[6].value = 1.0;
        values[7].value = 1.0;
        values[8].value = 1.0;
        let mut effect = prepare(&values);
        let mut reference = ReferenceDelayPair::new(
            48_000.0,
            ReferenceDelayParameters {
                left_delay_ms: 1.0,
                right_delay_ms: 1.0,
                left_feedback: 0.5,
                right_feedback: 0.5,
                left_damping: 0.0,
                right_damping: 0.0,
                left_mix: 1.0,
                right_mix: 1.0,
                cross_feedback: 1.0,
            },
        )
        .expect("oracle");
        let mut left = vec![0.0_f32; 100];
        let mut right = vec![0.0_f32; 100];
        left[0] = 1.0;
        effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("block"),
        );
        for index in 0..100 {
            let (expected_left, expected_right) =
                reference.process_sample(if index == 0 { 1.0 } else { 0.0 }, 0.0);
            assert!((left[index] - expected_left as f32).abs() < 1.0e-6);
            assert!((right[index] - expected_right as f32).abs() < 1.0e-6);
        }
        assert_eq!(left[48].to_bits(), 1.0_f32.to_bits());
        assert_eq!(right[96].to_bits(), 0.5_f32.to_bits());
    }

    #[test]
    fn crossfade_queue_automation_atomic_state_lazy_reset_and_recovery_are_local() {
        let mut values = initial_values();
        values[0].value = 1.0;
        values[1].value = 1.0;
        values[6].value = 1.0;
        values[7].value = 1.0;
        let mut effect = prepare(&values);
        effect.left.valid_history = u32::try_from(effect.left.ring.len()).expect("ring words");
        effect.left.active_delay = 1;
        effect.left.transition_delay = 1;
        effect.left.pending_delay = 2;
        effect.left.ring[effect.left.ring.len() - 1] = 0.0;
        effect.left.ring[effect.left.ring.len() - 2] = 1.0;
        effect.left.begin_transition();
        let first = effect.left.read_transition(0).expect("first");
        for _ in 2..64 {
            let _ = effect.left.read_transition(0).expect("transition");
        }
        let sixty_fourth = effect.left.read_transition(0).expect("sixty fourth");
        for _ in 65..=128 {
            let _ = effect.left.read_transition(0).expect("transition");
        }
        assert_eq!(first.to_bits(), (1.0_f32 / 128.0).to_bits());
        assert_eq!(sixty_fourth.to_bits(), 0.5_f32.to_bits());
        assert_eq!(effect.left.active_delay, 2);
        assert_eq!(effect.left.transition_remaining, 0);
        effect.left.pending_delay = 3;
        effect.left.ring[effect.left.ring.len() - 3] = 0.25;
        effect.left.begin_transition();
        let alpha = 1.0_f32 * (1.0_f32 / 128.0_f32);
        let queued_expected = 1.0_f32 + alpha * (0.25_f32 - 1.0_f32);
        assert_eq!(
            effect.left.read_transition(0).expect("queued").to_bits(),
            queued_expected.to_bits()
        );

        let delay_span = PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel: ParameterChannel::Left,
            parameter_index: 0,
            start_sample: 0,
            end_sample: 0,
            start_value: 2.0,
            end_value: 2.0,
        };
        let mut left = [0.2_f32; 64];
        let mut right = [-0.1_f32; 64];
        effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[delay_span], 128)
                .expect("automation block"),
        );
        let active = snapshot(&effect);
        let mut restored = prepare(&values);
        restored
            .restore_state_payload(
                1,
                StatePayloadInput::new(
                    &active.0,
                    &active.1,
                    &active.2,
                    restored.metadata().state_sizes,
                )
                .expect("state input"),
            )
            .expect("restore");
        let mut next_left = [0.1_f32; 16];
        let mut next_right = [-0.2_f32; 16];
        let mut restored_left = next_left;
        let mut restored_right = next_right;
        effect.process(
            EffectProcessBlock::new(&mut next_left, &mut next_right, None, 64, &[], 128)
                .expect("continuation"),
        );
        restored.process(
            EffectProcessBlock::new(&mut restored_left, &mut restored_right, None, 64, &[], 128)
                .expect("restored continuation"),
        );
        assert_eq!(next_left.map(f32::to_bits), restored_left.map(f32::to_bits));
        assert_eq!(
            next_right.map(f32::to_bits),
            restored_right.map(f32::to_bits)
        );

        effect.left.valid_history = u32::try_from(effect.left.ring.len()).expect("ring words");
        effect.left.active_delay = 1;
        effect.left.transition_delay = 1;
        effect.left.pending_delay = 1;
        effect.left.ring[(effect.cursor + effect.left.ring.len() - 1) % effect.left.ring.len()] =
            f32::INFINITY;
        let mut bad_left = [-0.25_f32];
        let mut good_right = [0.2_f32];
        let report = effect.process(
            EffectProcessBlock::new(&mut bad_left, &mut good_right, None, 80, &[], 128)
                .expect("fault block"),
        );
        assert_eq!(report.recovered_left_samples, 1);
        assert_eq!(report.recovered_right_samples, 0);
        assert_eq!(bad_left[0].to_bits(), (-0.25_f32).to_bits());
        assert_eq!(effect.left.valid_history, 0);
        effect.reset(ResetKind::DiscontinuityKeepParameters);
        let reset = snapshot(&effect);
        assert!(reset.1[16 * 4..].iter().all(|value| *value == 0));
        effect.reset(ResetKind::FullToDefaults);
        assert_eq!(effect.cursor, 0);
        assert_eq!(effect.left.valid_history, 0);
    }
}
