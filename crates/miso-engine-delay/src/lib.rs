//! Fixed two-second integer-time dual-mono and ping-pong delay.
//!
//! # Shape
//!
//! One `W = 1` block kernel (`delay_chunk`) renders both lanes. `process` splits a block into
//! chunks whose boundaries are chosen so that every per-sample decision — which taps are readable,
//! whether a crossfade is running, whether a ramp snaps this sample, whether the ring write wraps —
//! is **constant inside the chunk**. The kernel therefore has no data-dependent branch and no
//! modulo, and the tap history each chunk reads is copied out with two contiguous slice copies
//! before the chunk writes anything (the non-overlap proof is on `PreparedDelay::chunk_frames`).
//!
//! Delays do not bank: a two-second gathered ring has no `W4`/`W8` kernel, so
//! `bind_homogeneous_bank` returns `Ok(None)` and every instance is a scalar dynamic-rack member
//! (master plan #83 §4.1: a scalar tail is a `W = 1` block through the same generic body).
//!
//! # Where the numerics live
//!
//! * Fusion — only `miso_engine_lane::Lane::fma` (D3). The standard library's fused form is
//!   forbidden here and `scripts/check-lane-policy.sh` greps for it.
//! * Denormals — `miso_engine_lane::flush` on the two recursive words per lane, the damping state
//!   and the ring write, once per sample (D7). Nothing else is classified per value.
//! * Finiteness — once per block per lane, over the output, the ring write window and the damping
//!   state (D7, master plan §4.4). A failing lane has its output zeroed and its history dropped.
//! * Ramps — [`miso_engine_effect_runtime::ramp::LinearRamp`]: one division at event time, iterated
//!   additions, an exact assignment of the target on the final sample (D11).
//! * Transcendentals — `miso_engine_math` at control rate only (D6). The render path has none.
//!
//! # Damping
//!
//! The frozen `[0, 0.995]` damping control is mapped to a **topology-preserving one-pole**
//! coefficient at prepare and at every automation point. The control
//! keeps the cutoff it had at 48 kHz at every sample rate, which the raw-coefficient form it
//! replaces did not (issue #93 finding F5). Issue 021's amendment records the mapping.
#![allow(missing_docs)]

use miso_engine_effect_contract::{
    AutomationRate, AutomationSpanKind, EffectDescriptorV1, EffectPrepareError, EffectProcessBlock,
    EffectQuality, InitialParameterValue, LatencySamples, LinkModeSet, NativeEffectFactory,
    ParameterChannel, ParameterChannelPolicy, ParameterDescriptorV1, ParameterDomain, ParameterId,
    ParameterMapping, ParameterUnit, PortDescriptorV1, PortId, PortLayout, PortRole,
    PrepareEffectBankRequest, PrepareEffectRequest, PreparedAutomationSpan, PreparedEffectMetadata,
    PreparedNativeEffect, ProcessReport, ResetKind, SmoothingRule, StatePayloadError,
    StatePayloadInput, StatePayloadOutput, StatePayloadSizes, TailSamples,
    expected_prepared_metadata,
};
use miso_engine_effect_runtime::bank::check_block;
use miso_engine_effect_runtime::params::{
    ParameterSpec, is_negative_zero, normalize_zero, parameter_value_valid,
};
use miso_engine_effect_runtime::ramp::LinearRamp;
use miso_engine_effect_runtime::state_payload::{read_f32, read_u32, write_f32, write_u32};
use miso_engine_lane::{Lane, flush};

pub mod corpus;

const PER_LANE_PARAMETER_COUNT: usize = 4;
const ORDINARY_RAMP_COUNT: usize = 3;
const PARAMETER_COUNT: usize = 5;
const RAMP_SAMPLES: u32 = 64;
const TRANSITION_SAMPLES: u32 = 128;
const COMMON_BYTES: u32 = 16;
const FIXED_BYTES: u64 = 36;

/// Words of the per-lane state header, before the ring.
const LANE_HEADER_WORDS: usize = 16;

/// Largest number of frames one chunk of [`PreparedDelay::process`] renders.
///
/// Bounds the four stack tap windows at 512 bytes each. Every other chunk bound is a property of
/// the state (master plan #83 D10: the kernel owns the frame loop, so the loop must be long enough
/// to pay for the setup and short enough to keep the windows on the stack).
const CHUNK_FRAMES: usize = 128;

/// Scalar mask of the `W = 1` lane instantiation.
type Mask = <f32 as Lane>::Mask;

#[inline(always)]
fn lane_eq(a: f32, b: f32) -> Mask {
    <f32 as Lane>::eq(a, b)
}

#[inline(always)]
fn lane_select(m: Mask, a: f32, b: f32) -> f32 {
    <f32 as Lane>::select(m, a, b)
}

#[inline(always)]
fn lane_mask_or(a: Mask, b: Mask) -> Mask {
    <f32 as Lane>::mask_or(a, b)
}

/// The mask a `bool` stands for, built from lane compares so that no lane-mask representation is
/// assumed here.
#[inline(always)]
fn lane_mask(condition: bool) -> Mask {
    lane_eq(0.0, if condition { 0.0 } else { 1.0 })
}

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

/// The runtime domain of one descriptor parameter.
///
/// Derived from [`DELAY_PARAMETERS_V1`] so the descriptor stays the single source of the frozen
/// domains; `descriptor_and_specs_agree` checks the two never drift.
const fn spec_of(parameter: &ParameterDescriptorV1) -> ParameterSpec {
    let (minimum, maximum) = match (parameter.minimum, parameter.maximum) {
        (Some(minimum), Some(maximum)) => (minimum, maximum),
        _ => panic!("every delay parameter is a bounded continuous domain"),
    };
    ParameterSpec::continuous(minimum, maximum, parameter.default_value)
}

/// Frozen parameter domains, in descriptor order.
const PARAMETER_SPECS: [ParameterSpec; PARAMETER_COUNT] = [
    spec_of(&DELAY_PARAMETERS_V1[0]),
    spec_of(&DELAY_PARAMETERS_V1[1]),
    spec_of(&DELAY_PARAMETERS_V1[2]),
    spec_of(&DELAY_PARAMETERS_V1[3]),
    spec_of(&DELAY_PARAMETERS_V1[4]),
];

/// Sample rate the frozen damping control keeps its meaning at.
const DAMPING_REFERENCE_RATE_HZ: f64 = 48_000.0;

/// `0.45 * 44_100`: strictly below Nyquist at every launch rate, so `tan(pi * fc / fs)` stays
/// finite and positive at 44.1 kHz, the lowest rate the contract admits.
const DAMPING_MAX_CUTOFF_HZ: f64 = 19_845.0;

/// Maps the frozen linear damping control `c` to the one-pole TPT coefficient at `sample_rate`.
///
/// `fc(c) = min(19_845, -ln(c) * 48_000 / (2 pi))` Hz, `G = tan(pi * fc / fs)`, `g = G / (1 + G)`.
///
/// The control keeps the meaning it had at 48 kHz: the first-order pole of the frozen
/// `y = (1 - c) * x + c * y` recurrence sits at `fc = -ln(c) * fs / (2 pi)`, so evaluating that
/// expression **at the reference rate** and re-designing the coefficient for the running rate
/// holds the damping cutoff in hertz at every sample rate (issue #93 finding F5: the raw
/// coefficient made the tone of the feedback tail a function of the sample rate). Reference
/// values: `c = 0.25` (the default) is 10_590.6 Hz, `c = 0.995` is 38.3 Hz, and every
/// `c <= 0.0745` clamps at 19_845 Hz. `c == 0` maps to exactly `0.0` and selects the identity
/// path, which is the frozen "damping off is the exact tap" behaviour.
///
/// Control plane only: one `log` and one `tan` per event, never per sample (D6).
fn damping_coefficient(c: f32, sample_rate: u32) -> f32 {
    if c == 0.0 {
        return 0.0;
    }
    let cutoff = (-miso_engine_math::log(f64::from(c)) * DAMPING_REFERENCE_RATE_HZ
        / (2.0 * core::f64::consts::PI))
        .min(DAMPING_MAX_CUTOFF_HZ);
    let big_g = miso_engine_math::tan(core::f64::consts::PI * cutoff / f64::from(sample_rate));
    (big_g / (1.0 + big_g)) as f32
}

/// Largest coefficient [`damping_coefficient`] can produce at `sample_rate`.
///
/// The restore domain of the damping ramp triple: the state words hold `g`, not `c`, so the
/// descriptor's `[0, 0.995]` is not the interval a restored word has to lie in.
fn damping_coefficient_max(sample_rate: u32) -> f32 {
    let big_g = miso_engine_math::tan(
        core::f64::consts::PI * DAMPING_MAX_CUTOFF_HZ / f64::from(sample_rate),
    );
    (big_g / (1.0 + big_g)) as f32
}

/// The per-lane values a fresh or fully reset lane starts from, computed once at prepare.
#[derive(Clone, Copy, Debug)]
struct LaneDefaults {
    /// Frozen default delay time, in milliseconds.
    delay_ms: f32,
    /// `delay_ms` in samples, validated at prepare so no reset needs to re-derive it.
    delay: u32,
    /// Feedback coefficient.
    feedback: f32,
    /// Damping **coefficient** `g`, already mapped through [`damping_coefficient`].
    damping_g: f32,
    /// Wet mix.
    mix: f32,
}

impl LaneDefaults {
    /// Validates the four per-lane defaults and maps the damping control.
    fn new(
        values: &[f32; PER_LANE_PARAMETER_COUNT],
        sample_rate: u32,
        max_delay: u32,
    ) -> Option<Self> {
        Some(Self {
            delay_ms: values[0],
            delay: delay_samples(values[0], sample_rate, max_delay)?,
            feedback: values[1],
            damping_g: damping_coefficient(values[2], sample_rate),
            mix: values[3],
        })
    }

    /// The three ordinary ramps of a lane that has just been prepared or fully reset.
    const fn ramps(&self) -> [LinearRamp; ORDINARY_RAMP_COUNT] {
        [
            LinearRamp::fixed(self.feedback),
            LinearRamp::fixed(self.damping_g),
            LinearRamp::fixed(self.mix),
        ]
    }
}

/// One delayed lane: its ring, its tap-transition state and its three parameter ramps.
#[derive(Debug)]
struct DelayLane {
    /// Recursive damping state (the TPT integrator). D7-flushed once per sample.
    damping_state: f32,
    /// Last accepted delay time in milliseconds; `pending_delay` is its sample mapping.
    delay_target_ms: f32,
    /// Tap the lane reads when no crossfade is running.
    active_delay: u32,
    /// Tap the lane is crossfading towards; equals `active_delay` when idle.
    transition_delay: u32,
    /// Tap the next crossfade will start towards.
    pending_delay: u32,
    /// Updates left in the running crossfade; `0` when idle.
    transition_remaining: u32,
    /// Samples of ring history that are readable, saturating at the ring length.
    valid_history: u32,
    /// Feedback, damping coefficient `g`, wet mix.
    ramps: [LinearRamp; ORDINARY_RAMP_COUNT],
    /// `2 * sample_rate + 3` words. Allocated at prepare and never resized.
    ring: Box<[f32]>,
}

impl DelayLane {
    fn new(defaults: &LaneDefaults, ring_words: usize) -> Self {
        Self {
            damping_state: 0.0,
            delay_target_ms: defaults.delay_ms,
            active_delay: defaults.delay,
            transition_delay: defaults.delay,
            pending_delay: defaults.delay,
            transition_remaining: 0,
            valid_history: 0,
            ramps: defaults.ramps(),
            ring: vec![0.0; ring_words].into_boxed_slice(),
        }
    }

    /// Restores the lane to `delay`, `ramps` and an empty history, keeping the ring allocation.
    ///
    /// The one place the reset fields are listed. `FullToDefaults` passes the prepared defaults;
    /// `DiscontinuityKeepParameters` passes the pending tap and the ramps snapped to their targets.
    fn reset_to(
        &mut self,
        delay: u32,
        delay_target_ms: f32,
        ramps: [LinearRamp; ORDINARY_RAMP_COUNT],
    ) {
        self.damping_state = 0.0;
        self.delay_target_ms = delay_target_ms;
        self.active_delay = delay;
        self.transition_delay = delay;
        self.pending_delay = delay;
        self.transition_remaining = 0;
        self.valid_history = 0;
        self.ramps = ramps;
    }

    fn full_reset(&mut self, defaults: &LaneDefaults) {
        self.reset_to(defaults.delay, defaults.delay_ms, defaults.ramps());
    }

    fn discontinuity_reset(&mut self) {
        let mut ramps = self.ramps;
        for ramp in &mut ramps {
            ramp.snap();
        }
        // `pending_delay` is `delay_samples(delay_target_ms)` by invariant: automation writes the
        // two together and restore validates the pair, so no reset re-derives it.
        self.reset_to(self.pending_delay, self.delay_target_ms, ramps);
    }

    /// Starts the queued crossfade if one is queued and none is running.
    ///
    /// Called at a chunk boundary only. `pending_delay` changes at block start and `active_delay`
    /// at a chunk end, so a chunk boundary is the only sample index at which this can fire — which
    /// is what keeps the tap set constant inside a chunk.
    fn begin_transition(&mut self) {
        if self.transition_remaining == 0 && self.pending_delay != self.active_delay {
            self.transition_delay = self.pending_delay;
            self.transition_remaining = TRANSITION_SAMPLES;
        }
    }

    /// Copies `n` samples of tap history at delay `d` into `destination`.
    ///
    /// `start = cursor + r - d` lies in `[3, 2r)` because `cursor < r` and `1 <= d <= r - 3`, so
    /// one conditional subtract replaces the modulo the per-sample path performed on every tap
    /// read (issue #93 finding F2). The window is at most two contiguous runs.
    fn copy_window(&self, cursor: usize, d: usize, destination: &mut [f32]) {
        let r = self.ring.len();
        debug_assert!(cursor < r && (1..=r).contains(&d));
        let mut start = cursor + r - d;
        if start >= r {
            start -= r;
        }
        let n = destination.len();
        let first = n.min(r - start);
        destination[..first].copy_from_slice(&self.ring[start..start + first]);
        destination[first..].copy_from_slice(&self.ring[..n - first]);
    }

    /// `true` if a tap at `delay` reads real history for every sample of an `n`-frame chunk.
    ///
    /// Validity only grows, so a chunk is entirely valid or entirely invalid once
    /// [`DelayLane::history_bound`] has bounded it.
    const fn tap_is_valid(&self, delay: u32) -> bool {
        delay <= self.valid_history
    }
}

impl DelayLane {
    /// Fills the chunk's tap windows.
    ///
    /// `old` always receives the active tap; `new` receives the tap being faded towards and is
    /// left untouched when no crossfade is running, because the kernel does not read it then. A
    /// tap whose delay exceeds the valid history is written as zeros — the lazy-reset semantics of
    /// the per-sample path, resolved once per chunk instead of once per sample.
    fn fill_windows(&self, cursor: usize, old: &mut [f32], new: &mut [f32]) {
        self.fill_tap(cursor, self.active_delay, old);
        if self.transition_remaining > 0 {
            self.fill_tap(cursor, self.transition_delay, new);
        }
    }

    fn fill_tap(&self, cursor: usize, delay: u32, window: &mut [f32]) {
        if self.tap_is_valid(delay) {
            self.copy_window(cursor, delay as usize, window);
        } else {
            window.fill(0.0);
        }
    }

    /// Frames for which a tap at `delay` keeps the validity it has now.
    ///
    /// History only grows, so a valid tap stays valid; an invalid one becomes valid exactly
    /// `delay - valid_history` samples from now.
    const fn history_bound(&self, delay: u32) -> usize {
        if delay > self.valid_history {
            (delay - self.valid_history) as usize
        } else {
            usize::MAX
        }
    }
}

/// Prepared scalar delay state. The ring shape and metadata are immutable after preparation.
#[derive(Debug)]
pub struct PreparedDelay {
    metadata: PreparedEffectMetadata,
    resources: Resources,
    left_defaults: LaneDefaults,
    right_defaults: LaneDefaults,
    cross_default: f32,
    cursor: usize,
    cross: LinearRamp,
    left: DelayLane,
    right: DelayLane,
    /// Chunk length cap. Always [`CHUNK_FRAMES`] in production; the partition gate lowers it to 1
    /// to prove that the chunked kernel and a per-sample loop produce the same bits.
    #[cfg(test)]
    chunk_cap: usize,
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
    Ok(PreparedDelay {
        metadata,
        resources,
        left_defaults,
        right_defaults,
        cross_default,
        cursor: 0,
        cross: LinearRamp::fixed(cross_default),
        left: DelayLane::new(&left_defaults, resources.ring_words),
        right: DelayLane::new(&right_defaults, resources.ring_words),
        #[cfg(test)]
        chunk_cap: CHUNK_FRAMES,
    })
}

type ValidatedInputs = (
    PreparedEffectMetadata,
    Resources,
    LaneDefaults,
    LaneDefaults,
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
    let initial = EffectPrepareError {
        code: "effect.parameter.initial",
    };
    let left =
        LaneDefaults::new(&left, metadata.sample_rate, resources.max_delay).ok_or(initial)?;
    let right =
        LaneDefaults::new(&right, metadata.sample_rate, resources.max_delay).ok_or(initial)?;
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
    let initial = EffectPrepareError {
        code: "effect.parameter.initial",
    };
    if values.len() != 9 {
        return Err(initial);
    }
    let mut left = [0.0; PER_LANE_PARAMETER_COUNT];
    let mut right = [0.0; PER_LANE_PARAMETER_COUNT];
    for index in 0..PER_LANE_PARAMETER_COUNT {
        let left_value = values[index * 2];
        let right_value = values[index * 2 + 1];
        let spec = &PARAMETER_SPECS[index];
        if left_value.parameter_index != index as u32
            || right_value.parameter_index != index as u32
            || left_value.channel != ParameterChannel::Left
            || right_value.channel != ParameterChannel::Right
            || !parameter_value_valid(spec, left_value.value)
            || !parameter_value_valid(spec, right_value.value)
            || is_negative_zero(left_value.value)
            || is_negative_zero(right_value.value)
        {
            return Err(initial);
        }
        left[index] = normalize_zero(left_value.value);
        right[index] = normalize_zero(right_value.value);
    }
    let cross = values[8];
    if cross.parameter_index != 4
        || cross.channel != ParameterChannel::Both
        || !parameter_value_valid(&PARAMETER_SPECS[4], cross.value)
        || is_negative_zero(cross.value)
    {
        return Err(initial);
    }
    Ok((left, right, normalize_zero(cross.value)))
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

/// The four tap windows one chunk can need, on the stack of `process`.
///
/// Two per lane — the active tap and, while a crossfade runs, the tap being faded towards. 2 KiB,
/// written once per chunk by [`DelayLane::copy_window`] and never reallocated.
struct TapWindows {
    left_old: [f32; CHUNK_FRAMES],
    left_new: [f32; CHUNK_FRAMES],
    right_old: [f32; CHUNK_FRAMES],
    right_new: [f32; CHUNK_FRAMES],
}

impl TapWindows {
    const fn new() -> Self {
        Self {
            left_old: [0.0; CHUNK_FRAMES],
            left_new: [0.0; CHUNK_FRAMES],
            right_old: [0.0; CHUNK_FRAMES],
            right_new: [0.0; CHUNK_FRAMES],
        }
    }
}

/// Everything one lane contributes to one chunk, all of it constant across the chunk.
#[derive(Clone, Copy, Debug)]
struct LaneChunk<'a> {
    /// The active tap, `n` samples, zero-filled where the history is not yet valid.
    old: &'a [f32],
    /// The tap being faded towards; meaningful only when `fading`.
    new: &'a [f32],
    /// A crossfade is running for the whole chunk.
    fading: bool,
    /// This chunk consumes the final (128th) update of the crossfade.
    fade_last: bool,
    /// `(129 - transition_remaining) / 128 - 1/128`, so the first `+=` inside the kernel lands on
    /// update `j`'s exact weight `j / 128`.
    alpha: f32,
    /// `(first value, per-sample step)` of the damping coefficient ramp.
    damping: (f32, f32),
    /// `(first value, per-sample step)` of the feedback ramp.
    feedback: (f32, f32),
    /// `(first value, per-sample step)` of the wet-mix ramp.
    mix: (f32, f32),
}

/// The shared part of a chunk: the feedback matrix position and the bypass flag.
#[derive(Clone, Copy, Debug)]
struct CrossChunk {
    /// `(first value, per-sample step)` of the cross-feedback ramp.
    position: (f32, f32),
    /// Prepared bypass: the wet path still runs and the ring still fills, only the output is dry.
    bypass: bool,
}

impl PreparedDelay {
    /// Chunk length cap. Production is always [`CHUNK_FRAMES`].
    #[inline(always)]
    fn chunk_cap(&self) -> usize {
        #[cfg(test)]
        {
            self.chunk_cap
        }
        #[cfg(not(test))]
        {
            CHUNK_FRAMES
        }
    }

    /// The longest chunk starting at the current cursor over which every per-sample decision is
    /// constant, and inside which no tap read can observe a write of the same chunk.
    ///
    /// # Why chunking cannot change bits
    ///
    /// Sample `k` of a chunk reads ring cell `(cursor + k - D) mod R`; sample `j` of the same
    /// chunk writes cell `cursor + j`. An overlap needs `k - D === j (mod R)` with
    /// `0 <= j < k < n <= D <= R - 3`, that is `j = k - D < 0`, which is impossible. Every tap read
    /// of a chunk therefore sees only cells written *before* the chunk, so copying the whole tap
    /// window out before the chunk writes anything is the same read-before-write order the
    /// per-sample loop had. Every other per-sample quantity — which taps are readable, whether a
    /// crossfade is running, whether a ramp snaps, whether the write window wraps — is held
    /// constant by one of the bounds below, and every state word the kernel touches is written
    /// back at the chunk end. Gate `partition_invariance_over_1_7_64_128_512` confirms it.
    ///
    /// | bound | reason |
    /// |---|---|
    /// | `frames`, `chunk_cap` | block end; stack window size |
    /// | `R - cursor` | the write window stays one contiguous run |
    /// | `active_delay`, and `transition_delay` while fading | the non-overlap proof above |
    /// | `transition_remaining` while fading | a crossfade ends exactly at a chunk end |
    /// | `D - valid_history` when `D > valid_history` | the tap is all-zero or all-valid |
    /// | `remaining - 1` per running ramp (`1` at `remaining == 1`) | the D11 snap is its own frame |
    ///
    /// Every bound is at least 1, so the returned length is at least 1.
    fn chunk_frames(&self, remaining_in_block: usize, cap: usize) -> usize {
        let ring_words = self.left.ring.len();
        let mut frames = remaining_in_block.min(cap).min(ring_words - self.cursor);
        for lane in [&self.left, &self.right] {
            frames = frames
                .min(lane.active_delay as usize)
                .min(lane.history_bound(lane.active_delay));
            if lane.transition_remaining > 0 {
                frames = frames
                    .min(lane.transition_delay as usize)
                    .min(lane.history_bound(lane.transition_delay))
                    .min(lane.transition_remaining as usize);
            }
            for ramp in &lane.ramps {
                frames = frames.min(ramp_bound(ramp));
            }
        }
        frames = frames.min(ramp_bound(&self.cross));
        debug_assert!(frames >= 1);
        frames
    }

    /// Renders `io_left.len()` frames and advances every state word past them.
    fn process_chunk(
        &mut self,
        io_left: &mut [f32],
        io_right: &mut [f32],
        windows: &mut TapWindows,
        bypass: bool,
    ) {
        let frames = io_left.len();
        let cursor = self.cursor;
        let ring_words = self.left.ring.len();
        debug_assert!((1..=CHUNK_FRAMES).contains(&frames));
        debug_assert!(cursor + frames <= ring_words);

        self.left.fill_windows(
            cursor,
            &mut windows.left_old[..frames],
            &mut windows.left_new[..frames],
        );
        self.right.fill_windows(
            cursor,
            &mut windows.right_old[..frames],
            &mut windows.right_new[..frames],
        );

        let left_chunk = chunk_of(
            &mut self.left,
            &windows.left_old[..frames],
            &windows.left_new[..frames],
            frames,
        );
        let right_chunk = chunk_of(
            &mut self.right,
            &windows.right_old[..frames],
            &windows.right_new[..frames],
            frames,
        );
        let cross_segment = self.cross.advance_block::<f32>(frames);
        let cross_chunk = CrossChunk {
            position: (cross_segment.start, cross_segment.step),
            bypass,
        };

        {
            let left = &mut self.left;
            let right = &mut self.right;
            delay_chunk(
                io_left,
                io_right,
                &mut left.ring[cursor..cursor + frames],
                &mut right.ring[cursor..cursor + frames],
                left_chunk,
                right_chunk,
                cross_chunk,
                &mut left.damping_state,
                &mut right.damping_state,
            );
        }

        let advanced = frames as u32;
        let ring_limit = ring_words as u32;
        for lane in [&mut self.left, &mut self.right] {
            lane.valid_history = lane.valid_history.saturating_add(advanced).min(ring_limit);
            if lane.transition_remaining > 0 {
                lane.transition_remaining -= advanced;
                if lane.transition_remaining == 0 {
                    lane.active_delay = lane.transition_delay;
                }
            }
        }
        self.cursor += frames;
        if self.cursor == ring_words {
            self.cursor = 0;
        }
    }

    /// The once-per-block, once-per-lane finiteness check of D7 and master plan §4.4.
    ///
    /// Scans the lane's output, the ring cells this block wrote and the lane's damping state. A
    /// failing lane has its output zeroed, its damping state cleared and its history dropped; the
    /// shared cursor, the parameters and the crossfade state continue, which is the frozen
    /// recovery behaviour with its granularity moved from the sample to the block.
    fn finish_block(
        &mut self,
        io_left: &mut [f32],
        io_right: &mut [f32],
        cursor_at_block_start: usize,
        frames: usize,
        report: &mut ProcessReport,
    ) {
        let ring_words = self.left.ring.len();
        let written = frames.min(ring_words);
        let first = written.min(ring_words - cursor_at_block_start);
        let window = (cursor_at_block_start, first, written - first);
        recover_lane(
            &mut self.left,
            io_left,
            window,
            &mut report.nonfinite_left_blocks,
        );
        recover_lane(
            &mut self.right,
            io_right,
            window,
            &mut report.nonfinite_right_blocks,
        );
    }
}

/// Frames for which `ramp`'s per-sample value is `start + k * step` with no snap inside.
const fn ramp_bound(ramp: &LinearRamp) -> usize {
    match ramp.remaining {
        0 => usize::MAX,
        1 => 1,
        remaining => remaining as usize - 1,
    }
}

/// Checks one lane's block and recovers it if anything left the finite range.
fn recover_lane(
    lane: &mut DelayLane,
    output: &mut [f32],
    window: (usize, usize, usize),
    recovered: &mut u64,
) {
    let (start, first, second) = window;
    let clean = check_block::<f32>(output)
        && check_block::<f32>(&lane.ring[start..start + first])
        && check_block::<f32>(&lane.ring[..second])
        && check_block::<f32>(&[lane.damping_state]);
    if !clean {
        output.fill(0.0);
        lane.damping_state = 0.0;
        lane.valid_history = 0;
        *recovered = recovered.saturating_add(1);
    }
}

/// Builds one lane's chunk description and advances its three ramps past the chunk.
fn chunk_of<'a>(
    lane: &mut DelayLane,
    old: &'a [f32],
    new: &'a [f32],
    frames: usize,
) -> LaneChunk<'a> {
    /// Weight of one crossfade update; exact, and every `j / 128` for `j` in `1..=128` is exact,
    /// so the iterated addition below is the same value the closed form gives.
    const STEP_ALPHA: f32 = 1.0 / 128.0;

    let remaining = lane.transition_remaining;
    let feedback = lane.ramps[0].advance_block::<f32>(frames);
    let damping = lane.ramps[1].advance_block::<f32>(frames);
    let mix = lane.ramps[2].advance_block::<f32>(frames);
    LaneChunk {
        old,
        new,
        fading: remaining > 0,
        fade_last: remaining as usize == frames,
        alpha: (128 - remaining.min(128)) as f32 * STEP_ALPHA,
        damping: (damping.start, damping.step),
        feedback: (feedback.start, feedback.step),
        mix: (mix.start, mix.step),
    }
}

/// The `W = 1` delay kernel: one body, both lanes, one chunk.
///
/// # Frozen operation order
///
/// Per frame, per lane: tap (`fma` blend while crossfading), damping (`sub`, `mul`, `fma`, `add`,
/// `flush`), feedback gain (`mul`), the matrix (`sub`, two `mul`, two `fma`, four `select`), the
/// ring write (`add`, `flush`) and the wet mix (`sub`, `fma`, two compares, two `select`). Seven
/// coefficient additions carry the D11 ramps. No division, no modulo, no branch on a sample value
/// and no classification of a sample: the two `flush` calls are the entire denormal policy and the
/// finiteness check happens once per block in [`PreparedDelay::finish_block`].
///
/// Every identity the frozen contract promises — damping off is the exact tap, mix 0 is the exact
/// dry sample, mix 1 the exact tap, cross 0 the exact dual-mono pair and cross 1 the exact swap —
/// is a per-sample `select`, not a branch, so a ramp that lands on `0.0` or `1.0` between two
/// events is treated exactly as the per-sample path treated it. The `select` is bitwise, so a
/// non-finite value in the arm that is not chosen cannot leak into the chosen one: at cross 0 the
/// two lanes stay independent even when one of them has gone non-finite.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn delay_chunk(
    io_left: &mut [f32],
    io_right: &mut [f32],
    write_left: &mut [f32],
    write_right: &mut [f32],
    left: LaneChunk<'_>,
    right: LaneChunk<'_>,
    cross: CrossChunk,
    state_left: &mut f32,
    state_right: &mut f32,
) {
    /// Weight of one crossfade update. See [`chunk_of`].
    const STEP_ALPHA: f32 = 1.0 / 128.0;

    let frames = io_left.len();
    debug_assert!(frames >= 1);
    debug_assert_eq!(io_right.len(), frames);
    debug_assert_eq!(write_left.len(), frames);
    debug_assert_eq!(write_right.len(), frames);
    debug_assert_eq!(left.old.len(), frames);
    debug_assert_eq!(right.old.len(), frames);
    // Re-sliced to the chunk length so the bounds of all six streams are one fact the compiler
    // already has; the indexing below then carries no check.
    let io_left = &mut io_left[..frames];
    let io_right = &mut io_right[..frames];
    let write_left = &mut write_left[..frames];
    let write_right = &mut write_right[..frames];

    // Three specialisations of the body below, each *provably* the same computation rather than a
    // different one: a ramp whose first value is exactly `0.0` or `1.0` and whose step is exactly
    // zero holds that value for every sample of the chunk, so the corresponding per-sample mask is
    // constant and the `select` collapses to the arm it always chooses. They are read once, so
    // the loop is unswitched rather than branched per sample.
    let matrix_through = cross.position.0 == 0.0 && cross.position.1 == 0.0;
    let matrix_swap = cross.position.0 == 1.0 && cross.position.1 == 0.0;
    let damping_off = left.damping.0 == 0.0
        && left.damping.1 == 0.0
        && right.damping.0 == 0.0
        && right.damping.1 == 0.0;

    let last = frames - 1;
    let (mut gain_left, mut feedback_left, mut mix_left, mut alpha_left) =
        (left.damping.0, left.feedback.0, left.mix.0, left.alpha);
    let (mut gain_right, mut feedback_right, mut mix_right, mut alpha_right) =
        (right.damping.0, right.feedback.0, right.mix.0, right.alpha);
    let mut position = cross.position.0;
    let (mut state_l, mut state_r) = (*state_left, *state_right);
    let bypass = lane_mask(cross.bypass);

    for frame in 0..frames {
        let tap_left = tap_sample(&left, frame, last, &mut alpha_left, STEP_ALPHA);
        let tap_right = tap_sample(&right, frame, last, &mut alpha_right, STEP_ALPHA);
        let (damped_left, damped_right) = if damping_off {
            // Exactly `damp_sample`'s identity arm: the state takes the flushed tap, the output the
            // tap itself.
            state_l = flush(tap_left);
            state_r = flush(tap_right);
            (tap_left, tap_right)
        } else {
            (
                damp_sample(tap_left, gain_left, &mut state_l),
                damp_sample(tap_right, gain_right, &mut state_r),
            )
        };
        let sent_left = feedback_left * damped_left;
        let sent_right = feedback_right * damped_right;

        let (fed_left, fed_right) = if matrix_through {
            (sent_left, sent_right)
        } else if matrix_swap {
            (sent_right, sent_left)
        } else {
            let opposite = 1.0 - position;
            let through = lane_eq(position, 0.0);
            let swapped = lane_eq(position, 1.0);
            let mixed_left = opposite.fma(sent_left, position * sent_right);
            let mixed_right = position.fma(sent_left, opposite * sent_right);
            (
                lane_select(
                    through,
                    sent_left,
                    lane_select(swapped, sent_right, mixed_left),
                ),
                lane_select(
                    through,
                    sent_right,
                    lane_select(swapped, sent_left, mixed_right),
                ),
            )
        };

        let dry_left = io_left[frame];
        let dry_right = io_right[frame];
        write_left[frame] = flush(dry_left + fed_left);
        write_right[frame] = flush(dry_right + fed_right);
        io_left[frame] = mix_sample(dry_left, tap_left, mix_left, bypass);
        io_right[frame] = mix_sample(dry_right, tap_right, mix_right, bypass);

        gain_left += left.damping.1;
        feedback_left += left.feedback.1;
        mix_left += left.mix.1;
        gain_right += right.damping.1;
        feedback_right += right.feedback.1;
        mix_right += right.mix.1;
        position += cross.position.1;
    }

    *state_left = state_l;
    *state_right = state_r;
}

/// One tap sample: the active tap, or the crossfade blend `old + alpha * (new - old)`.
///
/// The final update of a crossfade delivers the new tap's bits exactly, which is the frozen
/// behaviour and is why the chunk that carries update 128 ends on it.
#[inline(always)]
fn tap_sample(lane: &LaneChunk<'_>, frame: usize, last: usize, alpha: &mut f32, step: f32) -> f32 {
    let old = lane.old[frame];
    if !lane.fading {
        return old;
    }
    let new = lane.new[frame];
    *alpha += step;
    if lane.fade_last && frame == last {
        return new;
    }
    alpha.fma(new - old, old)
}

/// One damping sample: a topology-preserving one-pole low pass, `g` already designed.
///
/// `v = g * (x - s)`, `y = s + v` with one rounding, `s' = y + v`. `g == 0` is the exact tap
/// identity the frozen contract promises, selected per sample. `s` is a recursive word, so it is
/// the D7 flush site.
#[inline(always)]
fn damp_sample(tap: f32, gain: f32, state: &mut f32) -> f32 {
    let previous = *state;
    let identity = lane_eq(gain, 0.0);
    let difference = tap - previous;
    let half = gain * difference;
    let output = gain.fma(difference, previous);
    *state = flush(lane_select(identity, tap, output + half));
    lane_select(identity, tap, output)
}

/// One output sample: `dry + mix * (wet - dry)` with one rounding, with the frozen dry and wet
/// identities and prepared bypass as bitwise selects.
#[inline(always)]
fn mix_sample(dry: f32, wet: f32, mix: f32, bypass: Mask) -> f32 {
    let blended = mix.fma(wet - dry, dry);
    let wet_or_blend = lane_select(lane_eq(mix, 1.0), wet, blended);
    lane_select(lane_mask_or(lane_eq(mix, 0.0), bypass), dry, wet_or_blend)
}

impl PreparedNativeEffect for PreparedDelay {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.metadata
    }

    fn reset(&mut self, kind: ResetKind) {
        self.cursor = 0;
        match kind {
            ResetKind::FullToDefaults => {
                self.cross = LinearRamp::fixed(self.cross_default);
                self.left.full_reset(&self.left_defaults);
                self.right.full_reset(&self.right_defaults);
            }
            ResetKind::DiscontinuityKeepParameters => {
                self.cross.snap();
                self.left.discontinuity_reset();
                self.right.discontinuity_reset();
            }
        }
    }

    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        let mut report = ProcessReport::default();
        let max_delay = self.resources.max_delay;
        let bypass = self.metadata.bypass;
        apply_automation(
            block.automation,
            self.metadata,
            block.first_sample,
            max_delay,
            &mut self.left,
            &mut self.right,
            &mut self.cross,
            &mut report,
        );

        let ring_words = self.left.ring.len();
        debug_assert_eq!(self.right.ring.len(), ring_words);
        let cursor_at_block_start = self.cursor;
        let cap = self.chunk_cap();
        let mut windows = TapWindows::new();
        let EffectProcessBlock {
            left: io_left,
            right: io_right,
            ..
        } = block;
        let frames = io_left.len();

        let mut offset = 0;
        while offset < frames {
            // The only place a queued crossfade may start: `pending_delay` moves at block start and
            // `active_delay` at a chunk end, so a chunk boundary is the only sample index at which
            // the tap set can change.
            self.left.begin_transition();
            self.right.begin_transition();
            let chunk = self.chunk_frames(frames - offset, cap);
            self.process_chunk(
                &mut io_left[offset..offset + chunk],
                &mut io_right[offset..offset + chunk],
                &mut windows,
                bypass,
            );
            offset += chunk;
        }

        self.finish_block(
            io_left,
            io_right,
            cursor_at_block_start,
            frames,
            &mut report,
        );

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
        write_u32(output.common, 0, self.cursor as u32);
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
        if state_layout_version != DELAY_DESCRIPTOR_V1.state_layout_version {
            return Err(state_error("effect.state.version"));
        }
        validate_state_lengths(
            input.common.len(),
            input.left.len(),
            input.right.len(),
            self.metadata.state_sizes,
        )?;
        let sample_rate = self.metadata.sample_rate;
        let cursor = read_u32(input.common, 0) as usize;
        if cursor >= self.resources.ring_words {
            return Err(state_error("effect.state.cursor"));
        }
        let cross = read_ramp(input.common, 1, &PARAMETER_SPECS[4])?;
        // Everything is validated as a pure function of the bytes before anything is written, so a
        // rejected restore cannot leave the effect half updated -- and validating the rings in
        // place is what removes the two ring-sized allocations the old path made on every restore
        // (issue #93 finding F9).
        let left = read_lane_header(input.left, self.resources, sample_rate)?;
        let right = read_lane_header(input.right, self.resources, sample_rate)?;
        validate_ring(
            input.left,
            cursor,
            left.valid_history,
            self.resources.ring_words,
        )?;
        validate_ring(
            input.right,
            cursor,
            right.valid_history,
            self.resources.ring_words,
        )?;

        self.cursor = cursor;
        self.cross = cross;
        left.apply(&mut self.left, input.left);
        right.apply(&mut self.right, input.right);
        Ok(())
    }
}

/// A validated lane header, held until both lanes and both rings have been accepted.
#[derive(Clone, Copy, Debug)]
struct LaneHeader {
    damping_state: f32,
    delay_target_ms: f32,
    active_delay: u32,
    transition_delay: u32,
    pending_delay: u32,
    transition_remaining: u32,
    valid_history: u32,
    ramps: [LinearRamp; ORDINARY_RAMP_COUNT],
}

impl LaneHeader {
    /// Commits the header and the ring words. Called only once every section has been validated.
    fn apply(&self, lane: &mut DelayLane, bytes: &[u8]) {
        lane.damping_state = self.damping_state;
        lane.delay_target_ms = self.delay_target_ms;
        lane.active_delay = self.active_delay;
        lane.transition_delay = self.transition_delay;
        lane.pending_delay = self.pending_delay;
        lane.transition_remaining = self.transition_remaining;
        lane.valid_history = self.valid_history;
        lane.ramps = self.ramps;
        for (index, value) in lane.ring.iter_mut().enumerate() {
            *value = read_f32(bytes, LANE_HEADER_WORDS + index);
        }
    }
}

/// Applies this block's automation points.
///
/// One `match` maps `(parameter, channel)` to a slot in `0..9`; the slot is the ordering key the
/// contract requires spans to be sorted by, the duplicate detector, and the index the values are
/// applied from. Every rejection increments `invalid_spans`; nothing here can panic, so a
/// malformed span costs a counter and not a process abort.
#[allow(clippy::too_many_arguments)]
fn apply_automation(
    spans: &[PreparedAutomationSpan],
    metadata: PreparedEffectMetadata,
    first_sample: u64,
    maximum_delay: u32,
    left: &mut DelayLane,
    right: &mut DelayLane,
    cross: &mut LinearRamp,
    report: &mut ProcessReport,
) {
    let sample_rate = metadata.sample_rate;
    let mut pending: [Option<f32>; 9] = [None; 9];
    let mut last_slot: Option<usize> = None;
    for (span_index, span) in spans.iter().enumerate() {
        let parameter_index = span.parameter_index as usize;
        let slot = match (parameter_index, span.channel) {
            (0..=3, ParameterChannel::Left) => Some(parameter_index * 2),
            (0..=3, ParameterChannel::Right) => Some(parameter_index * 2 + 1),
            (4, ParameterChannel::Both) => Some(8),
            _ => None,
        };
        let accepted = match slot {
            Some(slot) => {
                span_index < metadata.automation_capacity as usize
                    && span.kind == AutomationSpanKind::Point
                    && span.start_sample == first_sample
                    && span.end_sample == first_sample
                    && span.start_value.to_bits() == span.end_value.to_bits()
                    && parameter_value_valid(&PARAMETER_SPECS[parameter_index], span.start_value)
                    && last_slot.is_none_or(|previous| slot > previous)
                    && pending[slot].is_none()
                    // The delay time is the one parameter whose domain is not the whole story: it
                    // has to land on a legal integer tap as well.
                    && (parameter_index != 0
                        || delay_samples(span.start_value, sample_rate, maximum_delay).is_some())
            }
            None => false,
        };
        match slot.filter(|_| accepted) {
            Some(slot) => {
                last_slot = Some(slot);
                pending[slot] = Some(normalize_zero(span.start_value));
            }
            None => report.invalid_spans = report.invalid_spans.saturating_add(1),
        }
    }

    for (lane_index, lane) in [left, right].into_iter().enumerate() {
        if let Some(value) = pending[lane_index]
            && let Some(delay) = delay_samples(value, sample_rate, maximum_delay)
        {
            lane.delay_target_ms = value;
            lane.pending_delay = delay;
        }
        for ramp_index in 0..ORDINARY_RAMP_COUNT {
            if let Some(value) = pending[(ramp_index + 1) * 2 + lane_index] {
                // The damping ramp lives in the coefficient domain: the control is mapped here,
                // once per event, and never on the render path.
                let target = if ramp_index == 1 {
                    damping_coefficient(value, sample_rate)
                } else {
                    value
                };
                lane.ramps[ramp_index].set_target(target, RAMP_SAMPLES);
            }
        }
    }
    if let Some(value) = pending[8] {
        cross.set_target(value, RAMP_SAMPLES);
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
    for (index, ramp) in lane.ramps.iter().enumerate() {
        write_ramp(bytes, 7 + index * 3, *ramp);
    }
    for (index, value) in lane.ring.iter().copied().enumerate() {
        write_f32(
            bytes,
            LANE_HEADER_WORDS + index,
            if valid_ring_cell(cursor, lane.valid_history, lane.ring.len(), index) {
                value
            } else {
                0.0
            },
        );
    }
}

/// Validates one lane's header words without touching the effect.
fn read_lane_header(
    bytes: &[u8],
    resources: Resources,
    sample_rate: u32,
) -> Result<LaneHeader, StatePayloadError> {
    let damping_state = read_f32(bytes, 0);
    let delay_target_ms = read_f32(bytes, 1);
    let active_delay = read_u32(bytes, 2);
    let transition_delay = read_u32(bytes, 3);
    let pending_delay = read_u32(bytes, 4);
    let transition_remaining = read_u32(bytes, 5);
    let valid_history = read_u32(bytes, 6);
    if !normal_or_zero(damping_state)
        || !parameter_value_valid(&PARAMETER_SPECS[0], delay_target_ms)
        || is_negative_zero(delay_target_ms)
        || !valid_delay(active_delay, resources.max_delay)
        || !valid_delay(transition_delay, resources.max_delay)
        || !valid_delay(pending_delay, resources.max_delay)
        || transition_remaining > TRANSITION_SAMPLES
        || valid_history > resources.ring_words as u32
        || (transition_remaining == 0 && active_delay != transition_delay)
        || (transition_remaining != 0 && active_delay == transition_delay)
        || delay_samples(delay_target_ms, sample_rate, resources.max_delay) != Some(pending_delay)
    {
        return Err(state_error("effect.state.lane"));
    }
    // The damping triple holds the mapped coefficient, so its domain is the coefficient's, not the
    // control's; the other two hold their descriptor values.
    let damping_domain = ParameterSpec::continuous(0.0, damping_coefficient_max(sample_rate), 0.0);
    let specs = [&PARAMETER_SPECS[1], &damping_domain, &PARAMETER_SPECS[3]];
    let mut ramps = [LinearRamp::fixed(0.0); ORDINARY_RAMP_COUNT];
    for (index, ramp) in ramps.iter_mut().enumerate() {
        *ramp = read_ramp(bytes, 7 + index * 3, specs[index])?;
    }
    Ok(LaneHeader {
        damping_state,
        delay_target_ms,
        active_delay,
        transition_delay,
        pending_delay,
        transition_remaining,
        valid_history,
        ramps,
    })
}

/// Validates every ring word of one lane without writing any of them.
fn validate_ring(
    bytes: &[u8],
    cursor: usize,
    valid_history: u32,
    ring_words: usize,
) -> Result<(), StatePayloadError> {
    for index in 0..ring_words {
        let value = read_f32(bytes, LANE_HEADER_WORDS + index);
        if !normal_or_zero(value)
            || (!valid_ring_cell(cursor, valid_history, ring_words, index)
                && value.to_bits() != 0.0_f32.to_bits())
        {
            return Err(state_error("effect.state.history"));
        }
    }
    Ok(())
}

const fn valid_delay(value: u32, maximum: u32) -> bool {
    value >= 1 && value <= maximum
}

/// `true` if ring cell `index` holds history the effect may read back.
///
/// `age` is the distance behind the cursor, computed with one conditional instead of a modulo.
fn valid_ring_cell(cursor: usize, valid_history: u32, ring_words: usize, index: usize) -> bool {
    if valid_history as usize >= ring_words {
        return true;
    }
    let age = if index < cursor {
        cursor - index
    } else {
        cursor + ring_words - index
    };
    age != 0 && age <= valid_history as usize
}

fn normal_or_zero(value: f32) -> bool {
    value.is_normal() || value == 0.0
}

fn write_ramp(bytes: &mut [u8], word: usize, ramp: LinearRamp) {
    write_f32(bytes, word, ramp.current);
    write_f32(bytes, word + 1, ramp.target);
    write_u32(bytes, word + 2, ramp.remaining);
}

/// Reads a ramp triple and re-derives its D11 step.
///
/// The payload stores `(current, target, remaining)`; the step is a function of the three, so it is
/// recomputed here with the one division `LinearRamp::set_target` performs at event time rather
/// than stored and trusted. `remaining == 0` must come with `current == target`, which is
/// `LinearRamp`'s invariant and which every snapshot this effect writes satisfies.
fn read_ramp(
    bytes: &[u8],
    word: usize,
    spec: &ParameterSpec,
) -> Result<LinearRamp, StatePayloadError> {
    let current = read_f32(bytes, word);
    let target = read_f32(bytes, word + 1);
    let remaining = read_u32(bytes, word + 2);
    if !parameter_value_valid(spec, current)
        || !parameter_value_valid(spec, target)
        || remaining > RAMP_SAMPLES
        || (remaining == 0 && current != target)
    {
        return Err(state_error("effect.state.parameter"));
    }
    let mut ramp = LinearRamp::fixed(normalize_zero(current));
    ramp.set_target(normalize_zero(target), remaining);
    Ok(ramp)
}

const fn state_error(code: &'static str) -> StatePayloadError {
    StatePayloadError { code }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_core::KernelBackendV1;
    use miso_engine_dsp_reference::{ReferenceDelayPair, ReferenceDelayParameters};
    use miso_engine_effect_contract::{
        BankWidth, EffectProcessBlock, InitialParameterValue, LinkMode, PrepareEffectBankRequest,
        PreparedNativeEffect, StatePayloadInput, StatePayloadOutput, validate_descriptor_v1,
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

    fn request_with_quantum<'a>(
        values: &'a [InitialParameterValue],
        sample_rate: u32,
        quantum: u32,
    ) -> PrepareEffectRequest<'a> {
        let resources = resources(sample_rate).expect("launch resources");
        PrepareEffectRequest {
            sample_rate,
            quantum,
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

    fn request<'a>(
        values: &'a [InitialParameterValue],
        sample_rate: u32,
    ) -> PrepareEffectRequest<'a> {
        request_with_quantum(values, sample_rate, 128)
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

    fn point(
        parameter_index: u32,
        channel: ParameterChannel,
        first_sample: u64,
        value: f32,
    ) -> PreparedAutomationSpan {
        PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel,
            parameter_index,
            start_sample: first_sample,
            end_sample: first_sample,
            start_value: value,
            end_value: value,
        }
    }

    fn process_zeros(
        effect: &mut PreparedDelay,
        frames: usize,
        first_sample: u64,
        automation: &[PreparedAutomationSpan],
    ) -> ProcessReport {
        let mut left = vec![0.0_f32; frames];
        let mut right = vec![0.0_f32; frames];
        effect.process(
            EffectProcessBlock::new(
                &mut left,
                &mut right,
                None,
                first_sample,
                automation,
                effect.metadata.quantum,
            )
            .expect("zero block"),
        )
    }

    fn process_chunked(effect: &mut PreparedDelay, left: &mut [f32], right: &mut [f32]) {
        for offset in (0..left.len()).step_by(effect.metadata.quantum as usize) {
            let end = (offset + effect.metadata.quantum as usize).min(left.len());
            effect.process(
                EffectProcessBlock::new(
                    &mut left[offset..end],
                    &mut right[offset..end],
                    None,
                    offset as u64,
                    &[],
                    effect.metadata.quantum,
                )
                .expect("chunk"),
            );
        }
    }

    /// E1 — the frozen contract surface: resource table, latency, tail, caps, tap mapping.
    #[test]
    fn descriptor_exact_resources_caps_and_integer_mapping_are_frozen() {
        validate_descriptor_v1(&DELAY_DESCRIPTOR_V1).expect("descriptor");
        assert_eq!(DELAY_DESCRIPTOR_V1.id.as_str(), "miso.delay");
        assert_eq!(DELAY_DESCRIPTOR_V1.state_layout_version, 1);
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

    /// The runtime domains are the descriptor's domains, so validation cannot drift from metadata.
    #[test]
    fn descriptor_and_specs_agree() {
        for (spec, parameter) in PARAMETER_SPECS.iter().zip(&DELAY_PARAMETERS_V1) {
            assert_eq!(spec.minimum, parameter.minimum.expect("bounded"));
            assert_eq!(spec.maximum, parameter.maximum.expect("bounded"));
            assert_eq!(spec.default, parameter.default_value);
            assert_eq!(parameter.domain, ParameterDomain::Continuous);
        }
    }

    /// E2 — tap timing is sample exact and the frozen identities are bit exact.
    #[test]
    fn tap_timing_is_sample_exact() {
        let mut values = initial_values();
        for index in [0, 1] {
            values[index].value = 1.0;
        }
        values[2].value = 0.0;
        values[3].value = 0.0;
        values[4].value = 0.0;
        values[5].value = 0.0;
        values[6].value = 1.0;
        values[7].value = 1.0;
        values[8].value = 0.0;
        let mut unit = prepare(&values);
        let mut left = vec![0.0_f32; 100];
        let mut right = vec![0.0_f32; 100];
        left[0] = 1.0;
        unit.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("unit block"),
        );
        assert_eq!(left[48].to_bits(), 1.0_f32.to_bits());
        assert!(left[..48].iter().all(|sample| sample.to_bits() == 0));
        assert!(left[49..].iter().all(|sample| sample.to_bits() == 0));
        assert!(right.iter().all(|sample| sample.to_bits() == 0));

        let default_values = initial_values();
        let mut default = prepare(&default_values);
        let mut default_left = vec![0.0_f32; 12_001];
        let mut default_right = vec![0.0_f32; 12_001];
        default_left[0] = 1.0;
        process_chunked(&mut default, &mut default_left, &mut default_right);
        assert_eq!(default_left[12_000].to_bits(), 0.35_f32.to_bits());

        let mut ping_pong_values = values;
        ping_pong_values[2].value = 0.5;
        ping_pong_values[3].value = 0.5;
        ping_pong_values[8].value = 1.0;
        let mut ping_pong = prepare(&ping_pong_values);
        let mut ping_left = vec![0.0_f32; 100];
        let mut ping_right = vec![0.0_f32; 100];
        ping_left[0] = 1.0;
        ping_pong.process(
            EffectProcessBlock::new(&mut ping_left, &mut ping_right, None, 0, &[], 128)
                .expect("ping-pong block"),
        );
        assert_eq!(ping_right[96].to_bits(), 0.5_f32.to_bits());
        assert_eq!(ping_left[48].to_bits(), 1.0_f32.to_bits());
    }

    /// E3 — the damped, cross-fed tail against the independent `f64` oracle, at two sample rates.
    #[test]
    fn damped_matrix_tail_matches_reference_oracle() {
        for sample_rate in [48_000_u32, 96_000] {
            let mut values = initial_values();
            for index in [0, 1] {
                values[index].value = 1.0;
            }
            values[2].value = -0.5;
            values[3].value = 0.25;
            values[4].value = 0.375;
            values[5].value = 0.375;
            values[6].value = 1.0;
            values[7].value = 1.0;
            values[8].value = 0.5;
            let mut effect =
                prepare_delay(request(&values, sample_rate)).expect("prepare at launch rate");
            let mut reference = ReferenceDelayPair::new(
                f64::from(sample_rate),
                ReferenceDelayParameters {
                    left_delay_ms: 1.0,
                    right_delay_ms: 1.0,
                    left_feedback: -0.5,
                    right_feedback: 0.25,
                    left_damping: 0.375,
                    right_damping: 0.375,
                    left_mix: 1.0,
                    right_mix: 1.0,
                    cross_feedback: 0.5,
                },
            )
            .expect("oracle");
            let mut left = vec![0.0_f32; 160];
            let mut right = vec![0.0_f32; 160];
            left[0] = 1.0;
            right[0] = -0.25;
            effect.process(
                EffectProcessBlock::new(&mut left[..128], &mut right[..128], None, 0, &[], 128)
                    .expect("first block"),
            );
            effect.process(
                EffectProcessBlock::new(&mut left[128..], &mut right[128..], None, 128, &[], 128)
                    .expect("second block"),
            );
            let mut worst = 0.0_f32;
            for index in 0..160 {
                let (expected_left, expected_right) = reference.process_sample(
                    if index == 0 { 1.0 } else { 0.0 },
                    if index == 0 { -0.25 } else { 0.0 },
                );
                worst = worst
                    .max((left[index] - expected_left as f32).abs())
                    .max((right[index] - expected_right as f32).abs());
            }
            assert!(worst < 4.0e-6, "{sample_rate} Hz worst deviation {worst}");
        }
    }

    /// E4 — the damping control keeps its cutoff at every rate, and the mapping is monotone.
    #[test]
    fn damping_mapping_is_rate_invariant_and_monotone() {
        // `g = G / (1 + G)` inverts to `G = g / (1 - g)`, and `fc = atan(G) * fs / pi`.
        fn cutoff_of(g: f32, sample_rate: u32) -> f64 {
            let big_g = f64::from(g) / (1.0 - f64::from(g));
            miso_engine_math::atan(big_g) * f64::from(sample_rate) / core::f64::consts::PI
        }

        assert_eq!(
            damping_coefficient(0.0, 48_000).to_bits(),
            0.0_f32.to_bits()
        );
        for sample_rate in [44_100_u32, 48_000, 88_200, 96_000] {
            let cutoff = cutoff_of(damping_coefficient(0.25, sample_rate), sample_rate);
            assert!(
                (cutoff - 10_590.6).abs() < 0.1,
                "{sample_rate} Hz default cutoff {cutoff}"
            );
            let low = cutoff_of(damping_coefficient(0.995, sample_rate), sample_rate);
            assert!(
                (low - 38.29).abs() < 0.1,
                "{sample_rate} Hz low cutoff {low}"
            );
            // Everything below the clamp point shares one cutoff.
            let clamped = cutoff_of(damping_coefficient(0.01, sample_rate), sample_rate);
            assert!(
                (clamped - DAMPING_MAX_CUTOFF_HZ).abs() < 0.1,
                "{sample_rate} Hz clamped cutoff {clamped}"
            );
            let maximum = damping_coefficient_max(sample_rate);
            let mut previous = f32::INFINITY;
            for step in 0..1_000 {
                let control = 0.08 + (0.995 - 0.08) * step as f32 / 999.0;
                let g = damping_coefficient(control, sample_rate);
                assert!(g > 0.0 && g <= maximum, "{control} -> {g} > {maximum}");
                assert!(g < previous, "{control} is not below {previous}");
                previous = g;
            }
        }
    }

    /// E5 — D11 ramp updates, retargeting, and the driver's chunking of a ramp.
    #[test]
    fn ramp_updates_retarget_and_partition_are_exact() {
        let mut values = initial_values();
        values[6].value = 0.0;
        let span = point(3, ParameterChannel::Left, 0, 1.0);
        let mut effect = prepare(&values);
        process_zeros(&mut effect, 1, 0, &[span]);
        assert_eq!(
            effect.left.ramps[2].current.to_bits(),
            (1.0_f32 / 64.0).to_bits()
        );
        assert_eq!(effect.left.ramps[2].remaining, 63);
        assert_eq!(effect.right.ramps[2].current.to_bits(), 0.35_f32.to_bits());
        process_zeros(&mut effect, 62, 1, &[]);
        assert_eq!(
            effect.left.ramps[2].current.to_bits(),
            (63.0_f32 / 64.0).to_bits()
        );
        assert_eq!(effect.left.ramps[2].remaining, 1);
        process_zeros(&mut effect, 1, 63, &[]);
        assert_eq!(effect.left.ramps[2].current.to_bits(), 1.0_f32.to_bits());
        assert_eq!(effect.left.ramps[2].remaining, 0);
        let retarget = point(3, ParameterChannel::Left, 64, 0.0);
        process_zeros(&mut effect, 1, 64, &[retarget]);
        assert_eq!(
            effect.left.ramps[2].current.to_bits(),
            (63.0_f32 / 64.0).to_bits()
        );
        assert_eq!(effect.left.ramps[2].remaining, 63);

        let mut whole = prepare(&values);
        let mut partitioned = prepare(&values);
        process_zeros(&mut whole, 64, 0, &[span]);
        process_zeros(&mut partitioned, 1, 0, &[span]);
        process_zeros(&mut partitioned, 63, 1, &[]);
        assert_eq!(snapshot(&whole), snapshot(&partitioned));

        // The coefficient the kernel rides is the `LinearRamp` per-sample sequence, for a step that
        // is not representable, under the chunk lengths `chunk_frames` can hand it.
        let mut per_sample = LinearRamp::fixed(0.35);
        per_sample.set_target(0.8, RAMP_SAMPLES);
        let expected: Vec<u32> = (0..RAMP_SAMPLES)
            .map(|_| per_sample.next_value().to_bits())
            .collect();
        let mut chunked = LinearRamp::fixed(0.35);
        chunked.set_target(0.8, RAMP_SAMPLES);
        let mut produced: Vec<u32> = Vec::new();
        let wanted = [7_usize, 1, 40, 16, 64];
        let mut index = 0;
        while produced.len() < RAMP_SAMPLES as usize {
            let frames = wanted[index % wanted.len()]
                .min(RAMP_SAMPLES as usize - produced.len())
                .min(ramp_bound(&chunked));
            index += 1;
            let segment = chunked.advance_block::<f32>(frames);
            let mut value = segment.start;
            for _ in 0..frames {
                produced.push(value.to_bits());
                value += segment.step;
            }
        }
        assert_eq!(produced, expected);
        assert_eq!(chunked.current.to_bits(), 0.8_f32.to_bits());
    }

    /// E6 — the frozen 128-update crossfade law, its final update, and a queued retarget.
    #[test]
    fn crossfade_updates_are_exact_and_queued_retarget_completes() {
        let mut values = initial_values();
        values[0].value = 1.0;
        values[1].value = 1.0;
        values[2].value = 0.0;
        values[3].value = 0.0;
        values[4].value = 0.0;
        values[5].value = 0.0;
        values[6].value = 1.0;
        values[7].value = 1.0;
        values[8].value = 0.0;
        let mut effect = prepare(&values);
        let frames = 192;
        let mut left = vec![0.0_f32; frames];
        let mut right = vec![0.0_f32; frames];
        left[0] = 1.0;
        left[31] = 1.0e-8;
        left[32] = 0.5;
        left[79] = 1.0;
        let dry = left.clone();
        let to_two = [point(0, ParameterChannel::Left, 0, 2.0)];
        let back_to_one = [point(0, ParameterChannel::Left, 64, 1.0)];
        for offset in (0..frames).step_by(64) {
            let automation: &[PreparedAutomationSpan] = match offset {
                0 => &to_two,
                64 => &back_to_one,
                _ => &[],
            };
            effect.process(
                EffectProcessBlock::new(
                    &mut left[offset..offset + 64],
                    &mut right[offset..offset + 64],
                    None,
                    offset as u64,
                    automation,
                    128,
                )
                .expect("crossfade block"),
            );
        }

        // Update 49: the 1 ms tap is `dry[0]`, the 2 ms tap is still outside the valid history.
        assert_eq!(left[48].to_bits(), (79.0_f32 / 128.0).to_bits());
        // Update 97: the 2 ms tap has become valid and carries `dry[0]`.
        assert_eq!(left[96].to_bits(), (97.0_f32 / 128.0).to_bits());
        // Update 128 delivers the new tap's bits, not `old + 1 * (new - old)`, which for this pair
        // rounds to `+0.0`.
        assert_eq!(left[127].to_bits(), dry[31].to_bits());
        assert_eq!((dry[79] + (dry[31] - dry[79])).to_bits(), 0.0_f32.to_bits());
        assert_eq!(effect.left.active_delay, 96);
        // 128 updates of the queued fade, less the 64 frames of the last block.
        assert_eq!(effect.left.transition_remaining, 64);
        // The queued retarget starts on the sample after the fade completed, at weight 1/128.
        let queued = 0.5_f32 + (1.0_f32 / 128.0) * (0.0_f32 - 0.5_f32);
        assert_eq!(left[128].to_bits(), queued.to_bits());
        assert_eq!(queued.to_bits(), (127.0_f32 / 256.0).to_bits());
        assert_eq!(effect.left.transition_delay, 48);
    }

    /// Deterministic noise plus impulses, from an integer generator so every partition sees the
    /// same input bits.
    fn partition_signal(frames: usize) -> (Vec<f32>, Vec<f32>) {
        let mut state = 0x2545_f491_4f6c_dd1d_u64;
        let mut next = || {
            state ^= state >> 12;
            state ^= state << 25;
            state ^= state >> 27;
            state.wrapping_mul(0x2545_f491_4f6c_dd1d) >> 32
        };
        let mut left = vec![0.0_f32; frames];
        let mut right = vec![0.0_f32; frames];
        for index in 0..frames {
            left[index] = f32::from((next() >> 16) as u16) * (2.0 / 65_536.0) - 1.0;
            right[index] = f32::from((next() >> 16) as u16) * (2.0 / 65_536.0) - 1.0;
            if index % 9_973 == 0 {
                left[index] = 1.0;
                right[index] = -1.0;
            }
        }
        (left, right)
    }

    /// The automation event that lands on every multiple of `PARTITION_EVENT`.
    fn partition_spans(sample: u64) -> [PreparedAutomationSpan; 9] {
        let step = (sample / PARTITION_EVENT as u64) as usize;
        let delay = [1.0_f32, 4.0, 13.0, 7.0, 2.0];
        let feedback = [-0.9_f32, 0.0, 0.5, 0.95];
        let damping = [0.0_f32, 0.25, 0.9, 0.995];
        let mix = [0.0_f32, 1.0, 0.5, 0.35];
        let cross = [0.0_f32, 0.5, 1.0, 0.25];
        [
            point(0, ParameterChannel::Left, sample, delay[step % 5]),
            point(0, ParameterChannel::Right, sample, delay[(step + 2) % 5]),
            point(1, ParameterChannel::Left, sample, feedback[step % 4]),
            point(1, ParameterChannel::Right, sample, feedback[(step + 1) % 4]),
            point(2, ParameterChannel::Left, sample, damping[step % 4]),
            point(2, ParameterChannel::Right, sample, damping[(step + 3) % 4]),
            point(3, ParameterChannel::Left, sample, mix[step % 4]),
            point(3, ParameterChannel::Right, sample, mix[(step + 1) % 4]),
            point(4, ParameterChannel::Both, sample, cross[step % 4]),
        ]
    }

    /// `lcm(7, 512)`, and a multiple of 1, 64 and 128: every partition starts a block here.
    const PARTITION_EVENT: usize = 3_584;

    /// One partition run: the left output bits, the right output bits and the final snapshot.
    type PartitionRun = (Vec<u32>, Vec<u32>, (Vec<u8>, Vec<u8>, Vec<u8>));

    fn render_partition(frames: usize, block_frames: usize, chunk_cap: usize) -> PartitionRun {
        let mut values = initial_values();
        // 13 ms is 624 samples, so the initial tap stops being all-zero 624 samples in -- inside a
        // block at every partition and inside a chunk at the 128-frame cap. That is what makes the
        // `D - valid_history` chunk bound load bearing.
        values[0].value = 13.0;
        values[1].value = 13.0;
        let mut effect =
            prepare_delay(request_with_quantum(&values, 48_000, 512)).expect("prepare at q512");
        effect.chunk_cap = chunk_cap;
        let (mut left, mut right) = partition_signal(frames);
        let mut offset = 0;
        while offset < frames {
            let end = (offset + block_frames).min(frames);
            let spans = partition_spans(offset as u64);
            let automation: &[PreparedAutomationSpan] =
                if offset > 0 && offset % PARTITION_EVENT == 0 {
                    &spans
                } else {
                    &[]
                };
            let report = effect.process(
                EffectProcessBlock::new(
                    &mut left[offset..end],
                    &mut right[offset..end],
                    None,
                    offset as u64,
                    automation,
                    512,
                )
                .expect("partition block"),
            );
            assert_eq!(report, ProcessReport::default());
            offset = end;
        }
        (
            left.iter().map(|value| value.to_bits()).collect(),
            right.iter().map(|value| value.to_bits()).collect(),
            snapshot(&effect),
        )
    }

    /// P1 — the critical gate: chunking must not change one bit.
    #[test]
    fn partition_invariance_over_1_7_64_128_512() {
        const FRAMES: usize = 96_000;
        let reference = render_partition(FRAMES, 512, CHUNK_FRAMES);
        for block_frames in [1_usize, 7, 64, 128, 512] {
            let produced = render_partition(FRAMES, block_frames, CHUNK_FRAMES);
            assert_eq!(produced.0, reference.0, "left at block {block_frames}");
            assert_eq!(produced.1, reference.1, "right at block {block_frames}");
            assert_eq!(produced.2, reference.2, "state at block {block_frames}");
        }
        // And against the per-sample kernel: with a one-frame cap every chunk bound collapses and
        // the kernel is the straight per-sample loop the frozen contract describes.
        let per_sample = render_partition(FRAMES, 512, 1);
        assert_eq!(per_sample.0, reference.0, "left at chunk cap 1");
        assert_eq!(per_sample.1, reference.1, "right at chunk cap 1");
        assert_eq!(per_sample.2, reference.2, "state at chunk cap 1");
    }

    /// Parameters for the recovery tests: a 1 ms tap, wet output, real feedback.
    fn recovery_values(cross: f32) -> [InitialParameterValue; 9] {
        let mut values = initial_values();
        values[0].value = 1.0;
        values[1].value = 1.0;
        values[2].value = 0.5;
        values[3].value = 0.5;
        values[6].value = 1.0;
        values[7].value = 1.0;
        values[8].value = cross;
        values
    }

    /// E7 — D7: finiteness is a once-per-block, once-per-lane decision, and at cross 0 the two
    /// lanes are independent.
    #[test]
    fn nonfinite_state_recovers_per_block_lane_locally_at_p_zero() {
        let values = recovery_values(0.0);
        let mut injected = prepare(&values);
        let mut clean = prepare(&values);
        let (warm_left, warm_right) = partition_signal(128);
        for effect in [&mut injected, &mut clean] {
            let mut left = warm_left.clone();
            let mut right = warm_right.clone();
            let report = effect.process(
                EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("warm"),
            );
            assert_eq!(report, ProcessReport::default());
        }

        // The cell the next sample's 1 ms tap reads, which is inside the valid history.
        let ring_words = injected.left.ring.len();
        let poisoned = injected.cursor + ring_words - 48;
        injected.left.ring[poisoned % ring_words] = f32::INFINITY;

        let (next_left, next_right) = partition_signal(64);
        let mut injected_left = next_left.clone();
        let mut injected_right = next_right.clone();
        let report = injected.process(
            EffectProcessBlock::new(&mut injected_left, &mut injected_right, None, 128, &[], 128)
                .expect("faulted block"),
        );
        let mut clean_left = next_left;
        let mut clean_right = next_right;
        clean.process(
            EffectProcessBlock::new(&mut clean_left, &mut clean_right, None, 128, &[], 128)
                .expect("clean block"),
        );

        assert_eq!(report.nonfinite_left_blocks, 1);
        assert_eq!(report.nonfinite_right_blocks, 0);
        assert_eq!(report.sanitized_main_samples, 0);
        assert!(injected_left.iter().all(|sample| sample.to_bits() == 0));
        assert_eq!(injected.left.valid_history, 0);
        assert_eq!(injected.left.damping_state.to_bits(), 0.0_f32.to_bits());
        assert_eq!(
            injected_right
                .iter()
                .map(|s| s.to_bits())
                .collect::<Vec<_>>(),
            clean_right.iter().map(|s| s.to_bits()).collect::<Vec<_>>()
        );
        assert_eq!(injected.right.valid_history, clean.right.valid_history);

        // With the matrix engaged the fault crosses to the other lane inside the same block, and
        // both lanes recover.
        let crossed_values = recovery_values(0.5);
        let mut crossed = prepare(&crossed_values);
        let (warm_left, warm_right) = partition_signal(128);
        let mut left = warm_left;
        let mut right = warm_right;
        crossed.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("warm"),
        );
        let ring_words = crossed.left.ring.len();
        let poisoned = (crossed.cursor + ring_words - 48) % ring_words;
        crossed.left.ring[poisoned] = f32::INFINITY;
        let mut left = vec![0.25_f32; 64];
        let mut right = vec![-0.125_f32; 64];
        let report = crossed.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 128, &[], 128).expect("crossed"),
        );
        assert_eq!(report.nonfinite_left_blocks, 1);
        assert_eq!(report.nonfinite_right_blocks, 1);
        assert!(
            left.iter()
                .chain(&right)
                .all(|sample| sample.to_bits() == 0)
        );

        // A non-finite *input* is not sanitised here -- the input stage upstream owns that -- so it
        // is caught by the same block check and counted as a recovery.
        let mut from_input = prepare(&values);
        let mut left = [f32::NAN, f32::from_bits(1)];
        let mut right = [0.25_f32; 2];
        let report = from_input.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("nan input"),
        );
        assert_eq!(report.sanitized_main_samples, 0);
        assert_eq!(report.nonfinite_left_blocks, 1);
        assert_eq!(report.nonfinite_right_blocks, 0);
        assert_eq!(left.map(f32::to_bits), [0, 0]);
    }

    /// E8 — a rejected restore changes nothing, and both resets are word exact.
    #[test]
    fn invalid_restore_is_atomic_and_both_resets_are_word_exact() {
        let values = initial_values();
        let mut effect = prepare(&values);
        let spans = [
            point(0, ParameterChannel::Left, 0, 2.0),
            point(1, ParameterChannel::Left, 0, -0.5),
            point(4, ParameterChannel::Both, 0, 1.0),
        ];
        let mut left = [0.25_f32; 8];
        let mut right = [-0.125_f32; 8];
        effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &spans, 128)
                .expect("dirty block"),
        );
        let before = snapshot(&effect);
        // The effect is moved on, so a payload that is *partly* committed is observable: the
        // rejected restore must leave `after`, not a mixture of `after` and `before`.
        let mut left = [-0.5_f32; 24];
        let mut right = [0.25_f32; 24];
        effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 8, &[], 128)
                .expect("second block"),
        );
        let after = snapshot(&effect);
        assert_ne!(after, before);
        for (section, word, value) in [(2_usize, 0_usize, f32::NAN), (1, 0, f32::INFINITY)] {
            let mut invalid = before.clone();
            let bytes = match section {
                1 => &mut invalid.1,
                _ => &mut invalid.2,
            };
            write_f32(bytes, word, value);
            assert!(
                effect
                    .restore_state_payload(
                        1,
                        StatePayloadInput::new(
                            &invalid.0,
                            &invalid.1,
                            &invalid.2,
                            effect.metadata().state_sizes,
                        )
                        .expect("invalid payload shape"),
                    )
                    .is_err()
            );
            assert_eq!(snapshot(&effect), after);
        }
        // A stale ring word outside the valid history is rejected after the header, and still
        // leaves nothing written.
        let mut stale = before.clone();
        write_f32(&mut stale.1, LANE_HEADER_WORDS + 4_000, 0.5);
        assert!(
            effect
                .restore_state_payload(
                    1,
                    StatePayloadInput::new(
                        &stale.0,
                        &stale.1,
                        &stale.2,
                        effect.metadata().state_sizes,
                    )
                    .expect("stale payload shape"),
                )
                .is_err()
        );
        assert_eq!(snapshot(&effect), after);
        assert!(
            effect
                .restore_state_payload(
                    2,
                    StatePayloadInput::new(
                        &before.0,
                        &before.1,
                        &before.2,
                        effect.metadata().state_sizes,
                    )
                    .expect("payload shape"),
                )
                .is_err()
        );

        // A restored effect continues bit for bit.
        let mut restored = prepare(&values);
        restored
            .restore_state_payload(
                1,
                StatePayloadInput::new(
                    &before.0,
                    &before.1,
                    &before.2,
                    restored.metadata().state_sizes,
                )
                .expect("state input"),
            )
            .expect("restore");
        let mut next_left = [0.1_f32; 16];
        let mut next_right = [-0.2_f32; 16];
        let mut restored_left = next_left;
        let mut restored_right = next_right;
        effect
            .restore_state_payload(
                1,
                StatePayloadInput::new(
                    &before.0,
                    &before.1,
                    &before.2,
                    effect.metadata().state_sizes,
                )
                .expect("state input"),
            )
            .expect("restore the reference state");
        effect.process(
            EffectProcessBlock::new(&mut next_left, &mut next_right, None, 8, &[], 128)
                .expect("continuation"),
        );
        restored.process(
            EffectProcessBlock::new(&mut restored_left, &mut restored_right, None, 8, &[], 128)
                .expect("restored continuation"),
        );
        assert_eq!(next_left.map(f32::to_bits), restored_left.map(f32::to_bits));
        assert_eq!(
            next_right.map(f32::to_bits),
            restored_right.map(f32::to_bits)
        );

        effect.reset(ResetKind::DiscontinuityKeepParameters);
        let mut retained_values = values;
        retained_values[0].value = 2.0;
        retained_values[2].value = -0.5;
        retained_values[8].value = 1.0;
        let retained = prepare(&retained_values);
        assert_eq!(snapshot(&effect), snapshot(&retained));

        effect.reset(ResetKind::FullToDefaults);
        let fresh = prepare(&values);
        assert_eq!(snapshot(&effect), snapshot(&fresh));
        assert_eq!(effect.cursor, 0);
        assert_eq!(effect.left.valid_history, 0);
    }

    /// E9 — the dry identities keep the input's bits, including its sign of zero, while the ring
    /// word the same sample writes is canonical `+0.0` under D7.
    #[test]
    fn dry_identities_warm_histories_with_canonical_zero_state() {
        let mut mix_values = initial_values();
        mix_values[6].value = 0.0;
        mix_values[7].value = 0.0;
        let mut mix_zero = prepare(&mix_values);
        let mut left = [-0.0_f32];
        let mut right = [0.0_f32];
        mix_zero.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128)
                .expect("mix-zero block"),
        );
        assert_eq!(left[0].to_bits(), (-0.0_f32).to_bits());
        assert_eq!(mix_zero.left.valid_history, 1);
        // Re-pinned: the ring write is a recursive word, so it is flushed and `-0.0` becomes
        // `+0.0` (D7, issue #93 finding F10 -- the old pin was the software flush disagreeing with
        // hardware FTZ).
        assert_eq!(mix_zero.left.ring[0].to_bits(), 0.0_f32.to_bits());

        let values = initial_values();
        let mut bypass_request = request(&values, 48_000);
        bypass_request.bypass = true;
        let mut bypass = prepare_delay(bypass_request).expect("bypass prepare");
        let mut left = [-0.0_f32];
        let mut right = [0.0_f32];
        bypass.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128)
                .expect("bypass block"),
        );
        assert_eq!(left[0].to_bits(), (-0.0_f32).to_bits());
        assert_eq!(bypass.left.valid_history, 1);
        assert_eq!(bypass.left.ring[0].to_bits(), 0.0_f32.to_bits());

        // The D7 flush is what canonicalises the ring word, not the addition: a value inside the
        // flush band and the sum of two negative zeros both become exactly `+0.0`.
        let mut flush_values = initial_values();
        flush_values[0].value = 1.0;
        flush_values[1].value = 1.0;
        flush_values[2].value = 0.0;
        flush_values[3].value = 0.0;
        flush_values[4].value = 0.0;
        flush_values[5].value = 0.0;
        flush_values[6].value = 0.0;
        flush_values[7].value = 0.0;
        let mut flushing = prepare(&flush_values);
        let mut left = vec![0.0_f32; 64];
        let mut right = vec![0.0_f32; 64];
        left[0] = -0.5;
        left[1] = 1.0e-30;
        left[48] = -0.0;
        flushing.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("flush block"),
        );
        assert_eq!(flushing.left.ring[1].to_bits(), 0.0_f32.to_bits());
        assert_eq!(flushing.left.ring[48].to_bits(), 0.0_f32.to_bits());
        assert_eq!(left[1].to_bits(), 1.0e-30_f32.to_bits());
        assert_eq!(left[48].to_bits(), (-0.0_f32).to_bits());

        // Damping off is the exact tap, and the damping state follows it.
        let mut wet_values = initial_values();
        wet_values[0].value = 1.0;
        wet_values[1].value = 1.0;
        wet_values[4].value = 0.0;
        wet_values[5].value = 0.0;
        wet_values[6].value = 1.0;
        wet_values[7].value = 1.0;
        let mut wet = prepare(&wet_values);
        let mut left = vec![0.0_f32; 64];
        let mut right = vec![0.0_f32; 64];
        left[0] = 0.125;
        wet.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("wet block"),
        );
        assert_eq!(wet.left.ramps[1].current.to_bits(), 0.0_f32.to_bits());
        assert_eq!(left[48].to_bits(), 0.125_f32.to_bits());
    }

    /// E10 — the delay never binds a bank, and every member is validated before the fallback.
    #[test]
    fn bank_fallback_validates_every_member() {
        let factory = DelayFactory;
        let bank_values = [initial_values(); 4];
        let requests: [PrepareEffectRequest<'_>; 4] =
            core::array::from_fn(|index| request(&bank_values[index], 48_000));
        assert!(
            factory
                .bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend: KernelBackendV1::Aarch64Neon,
                    width: BankWidth::Four,
                    requests: &requests,
                })
                .expect("legal scalar fallback")
                .is_none()
        );

        let mut malformed_values = bank_values;
        malformed_values[3][0].value = f32::NAN;
        let malformed_requests: [PrepareEffectRequest<'_>; 4] =
            core::array::from_fn(|index| request(&malformed_values[index], 48_000));
        let malformed = match factory.bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: KernelBackendV1::Aarch64Neon,
            width: BankWidth::Four,
            requests: &malformed_requests,
        }) {
            Err(error) => error,
            Ok(_) => panic!("malformed member must precede fallback"),
        };
        assert_eq!(malformed.code, "effect.parameter.initial");
        let mut below_cap = requests;
        below_cap[3].limits.maximum_total_state_bytes -= 1;
        let under_cap = match factory.bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: KernelBackendV1::Aarch64Neon,
            width: BankWidth::Four,
            requests: &below_cap,
        }) {
            Err(error) => error,
            Ok(_) => panic!("under-cap member must precede fallback"),
        };
        assert_eq!(under_cap.code, "effect.resource.limit");
    }

    /// Automation that is out of shape is counted, never applied, and never panics.
    #[test]
    fn malformed_automation_is_counted_and_never_applied() {
        let values = initial_values();
        let mut effect = prepare(&values);
        let spans = [
            // Out of order: parameter 3 before parameter 1.
            point(3, ParameterChannel::Left, 0, 0.5),
            point(1, ParameterChannel::Left, 0, 0.5),
            // Wrong channel for a per-lane parameter.
            point(1, ParameterChannel::Both, 0, 0.5),
            // Unknown parameter.
            point(9, ParameterChannel::Left, 0, 0.5),
            // Outside the domain.
            point(4, ParameterChannel::Both, 0, 2.0),
            // Not a point.
            PreparedAutomationSpan {
                kind: AutomationSpanKind::Linear,
                channel: ParameterChannel::Both,
                parameter_index: 4,
                start_sample: 0,
                end_sample: 0,
                start_value: 0.5,
                end_value: 0.5,
            },
        ];
        let report = process_zeros(&mut effect, 4, 0, &spans);
        assert_eq!(report.invalid_spans, 5);
        assert_eq!(effect.left.ramps[2].target.to_bits(), 0.5_f32.to_bits());
        assert_eq!(effect.left.ramps[0].current.to_bits(), 0.35_f32.to_bits());
        assert_eq!(effect.cross.target.to_bits(), 0.0_f32.to_bits());
    }
}
