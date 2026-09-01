//! Fixed two-times cubic soft clipper, as one block kernel over the `Lane` foundation.
//!
//! The rendered graph is frozen by `.github/ISSUE_SPECS/BRIEFS/019`: per-lane drive, output and
//! mix; two-times oversampling through a 63-tap Blackman half-band used for both interpolation and
//! decimation; the cubic `c(u) = u - u^3/3` clamped to `±2/3`; a dry path delayed 31 samples;
//! latency 31 and a finite tail of 29. None of that changed in the issue-#91 re-landing — the
//! output is bit-identical to the five hand-written copies it replaces
//! (`tests/polyphase_identity.rs`). What changed is everything around the arithmetic:
//!
//! * one generic [`kernel::soft_clip_block`] instead of an effect-crate scalar lane plus four
//!   `core/arch` kernels, so `WIDTH = 1`, 4 and 8 are the same code and lane identity is a property
//!   of the code (`tests/lane_identity.rs`);
//! * the polyphase half-band form, which does the work of two 31-tap convolutions instead of four;
//! * one cursor per bank over a double-written power-of-two history, so every tap is a contiguous
//!   vector load at a constant offset — no per-lane cursor, no modulus, no gather;
//! * D7 instead of per-operation checking: `flush` on the two values that enter a history, and one
//!   boundary check per block per bank through `effect_runtime::bank`;
//! * D11 ramps, D6 decibel conversion and the shared state-payload codec, all from
//!   `effect-runtime` and `math`.
//!
//! Measured on the delivery host, W8 bank, production `process_bank` shape: 246 ns per
//! track-channel-sample before, 3.0 ns after (`tests/descriptive_bench.rs`).
//!
//! # State layout version 2
//!
//! The shared cursor (D10) removed the per-lane cursor word and the D11 ramp added a `step` word,
//! so the payload changed shape and the descriptor's `state_layout_version` is 2. Layout-1
//! payloads are rejected with `effect.state.version`; a converting edge, if one is ever wanted, is
//! the migration registry of issue #080, not this crate.

use effect_contract::{
    AutomationRate, AutomationSpanKind, BankProcessReport, BankWidth, EffectBankProcessBlock,
    EffectDescriptor, EffectPrepareError, EffectProcessBlock, EffectQuality, InitialParameterValue,
    LatencySamples, LinkModeSet, NativeEffectFactory, ParameterChannel, ParameterChannelPolicy,
    ParameterDescriptor, ParameterDomain, ParameterId, ParameterMapping, ParameterUnit,
    PortDescriptor, PortId, PortLayout, PortRole, PrepareEffectBankRequest, PrepareEffectRequest,
    PreparedAutomationSpan, PreparedBankMetadata, PreparedEffectMetadata, PreparedNativeEffect,
    PreparedNativeEffectBank, ProcessReport, ResetKind, SmoothingRule, StatePayloadError,
    StatePayloadInput, StatePayloadOutput, StatePayloadSizes, TailSamples,
    expected_prepared_metadata,
};
use effect_runtime::bank::{NonFiniteReport, finish_block};
use effect_runtime::params::{ParameterSpec, is_negative_zero, normalize_zero};
use effect_runtime::ramp::LinearRamp;
use effect_runtime::state_payload as payload;
use lane::kernels::halfband::{HALFBAND63_LIVE_ROWS as LIVE_ROWS, HALFBAND63_POS_MASK as POS_MASK};
use lane::{Lane, Simd4, Simd8};
use math::db_to_gain_f32;

pub mod corpus;
pub mod kernel;

use kernel::{SoftClipCoef, SoftClipHistory, SoftClipState, soft_clip_block};

/// Parameters: drive, output, mix, in stable numeric-ID order.
const PARAMETER_COUNT: usize = 3;

/// Samples a parameter ramp takes to reach a new target (D11, one division per event).
const RAMP_SAMPLES: u32 = 64;

/// Effect-owned words in one channel's state payload; see [`STATE_LAYOUT`].
const LANE_STATE_WORDS: u32 = 104;

/// Words each ramp occupies in the payload: current, target, step, remaining.
const RAMP_WORDS: usize = 4;

/// First payload word of the interpolator input history.
const X_HISTORY_WORD: usize = PARAMETER_COUNT * RAMP_WORDS;

/// Ages of the interpolator input history that a restore must carry: `X[n] .. X[n-30]`.
const X_HISTORY_AGES: usize = 31;

/// First payload word of the shaped even-phase history.
const E_HISTORY_WORD: usize = X_HISTORY_WORD + X_HISTORY_AGES;

/// Ages of the shaped even-phase history that a restore must carry: `e[n] .. e[n-29]`.
const E_HISTORY_AGES: usize = 30;

/// First payload word of the dry history.
const DRY_HISTORY_WORD: usize = E_HISTORY_WORD + E_HISTORY_AGES;

/// Ages of the dry history that a restore must carry: `x[n] .. x[n-30]`.
const DRY_HISTORY_AGES: usize = 31;

/// The payload shape, stamped into the common section by the shared codec.
const STATE_LAYOUT: payload::StateLayout = payload::StateLayout {
    version: 2,
    common_words: 0,
    lane_words: LANE_STATE_WORDS,
};

/// Byte lengths the descriptor advertises for one prepared instance.
const STATE_SIZES: payload::StatePayloadSizes = payload::expected_sizes(&STATE_LAYOUT);

/// External parameter domains, for validation and clamping through the shared helpers.
const PARAMETER_SPECS: [ParameterSpec; PARAMETER_COUNT] = [
    ParameterSpec::continuous(-24.0, 36.0, 0.0),
    ParameterSpec::continuous(-24.0, 24.0, 0.0),
    ParameterSpec::continuous(0.0, 1.0, 1.0),
];

const fn effect_id(value: &'static str) -> effect_contract::EffectId {
    match effect_contract::EffectId::new(value) {
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
) -> ParameterDescriptor {
    ParameterDescriptor {
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
        lattice: effect_contract::default_parameter_lattice(
            unit,
            ParameterDomain::Continuous,
            ParameterMapping::Linear,
        ),
    }
}

/// Frozen scalar soft-clip parameter rows, in stable numeric-ID order.
pub const SOFT_CLIP_PARAMETERS: [ParameterDescriptor; PARAMETER_COUNT] = [
    parameter(1, "drive", "dB", ParameterUnit::Db, -24.0, 36.0, 0.0),
    parameter(2, "output", "dB", ParameterUnit::Db, -24.0, 24.0, 0.0),
    parameter(3, "mix", "linear", ParameterUnit::Linear, 0.0, 1.0, 1.0),
];

const PORTS: [PortDescriptor; 2] = [
    PortDescriptor {
        id: port_id("main-in"),
        role: PortRole::MainInput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptor {
        id: port_id("main-out"),
        role: PortRole::MainOutput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
];

const fn quality(rate: u32) -> effect_contract::QualityDescriptor {
    effect_contract::QualityDescriptor {
        quality: EffectQuality::Normal,
        sample_rate: rate,
        latency: LatencySamples(31),
        tail: TailSamples::Finite(29),
        maximum_state: StatePayloadSizes {
            common_bytes: STATE_SIZES.common as u32,
            left_bytes: STATE_SIZES.left as u32,
            right_bytes: STATE_SIZES.right as u32,
        },
        scratch_fixed_bytes: 24,
        scratch_bytes_per_frame: 0,
    }
}

const QUALITIES: [effect_contract::QualityDescriptor; 4] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
];

/// Immutable descriptor for the frozen cubic soft-clip contract.
pub const SOFT_CLIP_DESCRIPTOR: EffectDescriptor = EffectDescriptor {
    id: effect_id("miso.soft-clip"),
    display_name: "Cubic Soft Clip",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: STATE_LAYOUT.version,
    supported_link_modes: LinkModeSet::DUAL_MONO,
    parameters: &SOFT_CLIP_PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
    observations: &[],
};

/// Factory for the fixed-latency soft-clip realization.
#[derive(Clone, Copy, Debug, Default)]
pub struct SoftClipFactory;

// ---------------------------------------------------------------------------------------------
// Parameter conversion
// ---------------------------------------------------------------------------------------------

/// `true` if `value` is inside the external domain of parameter `index`.
fn parameter_value_valid(index: usize, value: f32) -> bool {
    PARAMETER_SPECS
        .get(index)
        .is_some_and(|spec| effect_runtime::params::parameter_value_valid(spec, value))
}

/// The external value converted to what the kernel uses: a linear gain, or the mix itself.
fn convert_parameter(index: usize, value: f32) -> Option<f32> {
    if !parameter_value_valid(index, value) {
        return None;
    }
    let value = normalize_zero(value);
    match index {
        0 | 1 => {
            let gain = db_to_gain_f32(value);
            converted_value_valid(index, gain).then_some(gain)
        }
        2 => Some(value),
        _ => None,
    }
}

/// The converted (kernel-domain) range of parameter `index`.
fn converted_domain(index: usize) -> Option<(f32, f32)> {
    let spec = PARAMETER_SPECS.get(index)?;
    match index {
        0 | 1 => Some((db_to_gain_f32(spec.minimum), db_to_gain_f32(spec.maximum))),
        2 => Some((spec.minimum, spec.maximum)),
        _ => None,
    }
}

/// `true` if a converted value is finite, not `-0.0`, not subnormal, and inside its range.
///
/// This is control-plane validation of a restored or prepared coefficient, not a render-path
/// check: the render path has none (D7).
fn converted_value_valid(index: usize, value: f32) -> bool {
    if is_negative_zero(value) || !value.is_finite() || value.is_subnormal() {
        return false;
    }
    converted_domain(index).is_some_and(|(low, high)| value >= low && value <= high)
}

/// Converts the six validated initial values into the two per-channel coefficient sets.
///
/// The `-0.0` rule is **not** re-implemented here. `expected_prepared_metadata` has already run
/// the contract's `validate_initial_values`, which rejects a negative zero, and every caller of
/// this function goes through it first; the effect-runtime's rule is the lenient one (normalise,
/// do not reject) and #95 owns reconciling the two. One law, one home: whichever way that lands,
/// this crate follows it without an edit.
fn initial_defaults(
    values: &[InitialParameterValue],
) -> Result<([f32; PARAMETER_COUNT], [f32; PARAMETER_COUNT]), EffectPrepareError> {
    let invalid = EffectPrepareError {
        code: "effect.parameter.initial",
    };
    if values.len() != PARAMETER_COUNT * 2 {
        return Err(invalid);
    }
    let mut left = [0.0; PARAMETER_COUNT];
    let mut right = [0.0; PARAMETER_COUNT];
    for (index, value) in values.iter().enumerate() {
        let parameter = index / 2;
        let channel = if index.is_multiple_of(2) {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        };
        if value.parameter_index != parameter as u32
            || value.channel != channel
            || !parameter_value_valid(parameter, value.value)
        {
            return Err(invalid);
        }
        let converted = convert_parameter(parameter, value.value).ok_or(invalid)?;
        if index.is_multiple_of(2) {
            left[parameter] = converted;
        } else {
            right[parameter] = converted;
        }
    }
    Ok((left, right))
}

// ---------------------------------------------------------------------------------------------
// One channel of one cohort
// ---------------------------------------------------------------------------------------------

/// Per-lane values written into a lane vector, sized for the widest backend.
type LaneWords = [f32; 8];

/// Reads one lane out of a vector.
fn lane_of<L: Lane>(value: L, lane: usize) -> f32 {
    let mut words: LaneWords = [0.0; 8];
    value.store(&mut words[..L::WIDTH]);
    words[lane]
}

/// Writes one lane of a vector, leaving the others bit-identical.
fn set_lane<L: Lane>(value: &mut L, lane: usize, sample: f32) {
    let mut words: LaneWords = [0.0; 8];
    value.store(&mut words[..L::WIDTH]);
    words[lane] = sample;
    *value = L::load(&words[..L::WIDTH]);
}

/// One audio channel of a cohort: the kernel's state and histories, plus the control-plane ramps.
///
/// The ramp's *current* value of record lives in [`SoftClipState`], because it is the kernel's
/// iterated additions that produce it; `ramps[lane][p].current` is synchronised from there at every
/// block boundary, which is when the control plane next needs it (to divide once, at a new target).
struct Channel<L: Lane> {
    state: SoftClipState<L>,
    history: SoftClipHistory,
    ramps: Box<[[LinearRamp; PARAMETER_COUNT]]>,
}

impl<L: Lane> Channel<L> {
    /// Allocates a channel whose lanes rest at `defaults`. Control plane.
    fn new(defaults: &[[f32; PARAMETER_COUNT]]) -> Self {
        debug_assert_eq!(defaults.len(), L::WIDTH);
        let value = |parameter: usize| {
            let mut words: LaneWords = [0.0; 8];
            for (lane, slot) in words[..L::WIDTH].iter_mut().enumerate() {
                *slot = defaults[lane][parameter];
            }
            L::load(&words[..L::WIDTH])
        };
        Self {
            state: SoftClipState::from_lanes(value(0), value(1), value(2)),
            history: SoftClipHistory::new(L::WIDTH),
            ramps: defaults
                .iter()
                .map(|lane| lane.map(LinearRamp::fixed))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    /// Clears every history and returns every ramp to `defaults`, at rest.
    ///
    /// In place, never by rebuilding: a reset can reach this from a seek or a transport stop, and
    /// the render thread has no allocator.
    fn reset_full(&mut self, defaults: &[[f32; PARAMETER_COUNT]]) {
        debug_assert_eq!(defaults.len(), L::WIDTH);
        self.history.clear();
        for (lane, (values, ramps)) in defaults.iter().zip(self.ramps.iter_mut()).enumerate() {
            for (parameter, value) in values.iter().copied().enumerate() {
                ramps[parameter] = LinearRamp::fixed(value);
                set_lane(self.state.field_mut(parameter), lane, value);
            }
        }
    }

    /// Clears every history and snaps every ramp to its target, keeping parameters.
    fn reset_discontinuity(&mut self) {
        self.history.clear();
        for (lane, ramps) in self.ramps.iter_mut().enumerate() {
            for (parameter, ramp) in ramps.iter_mut().enumerate() {
                ramp.snap();
                set_lane(self.state.field_mut(parameter), lane, ramp.current);
            }
        }
    }

    /// Renders one AoSoA block, splitting it wherever a D11 ramp reaches its target.
    ///
    /// A segment is the longest run over which every lane's three increments are constant. The
    /// final sample of a ramp is an **assignment** of the target, not an addition, so a ramp with
    /// `remaining == 1` is snapped into the state before the segment that renders that sample and
    /// contributes a zero increment to it. That reproduces
    /// [`LinearRamp::next_value`](effect_runtime::ramp::LinearRamp::next_value) sample
    /// for sample, which `tests/ramp_law.rs` asserts by bits.
    ///
    /// Segments are bounded by `1 + 3 * WIDTH` per block and the loop allocates nothing.
    fn process(&mut self, io: &mut [f32], frames: usize, bypass: bool) {
        let width = L::WIDTH;
        debug_assert_eq!(io.len(), frames * width);
        let all = L::zero().eq(L::zero());
        let bypass_mask = if bypass { all } else { L::mask_not(all) };
        let mut done = 0;
        while done < frames {
            for lane in 0..width {
                for parameter in 0..PARAMETER_COUNT {
                    let ramp = &mut self.ramps[lane][parameter];
                    if ramp.remaining == 1 {
                        ramp.snap();
                        set_lane(self.state.field_mut(parameter), lane, ramp.current);
                    }
                }
            }
            let mut span = frames - done;
            for ramps in self.ramps.iter() {
                for ramp in ramps {
                    if ramp.remaining > 0 {
                        span = span.min(ramp.remaining as usize - 1);
                    }
                }
            }
            debug_assert!(span >= 1);
            let coefficients = SoftClipCoef {
                drive_step: self.step_vector(0),
                output_step: self.step_vector(1),
                mix_step: self.step_vector(2),
                bypass: bypass_mask,
            };
            soft_clip_block::<L>(
                &mut io[done * width..(done + span) * width],
                span,
                &coefficients,
                &mut self.state,
                &mut self.history,
            );
            for ramps in self.ramps.iter_mut() {
                for ramp in ramps {
                    if ramp.remaining > 0 {
                        ramp.remaining -= span as u32;
                    }
                }
            }
            done += span;
        }
        self.synchronize_currents();
    }

    /// The per-lane increment vector of one parameter; `+0.0` where a lane is not ramping.
    fn step_vector(&self, parameter: usize) -> L {
        let mut words: LaneWords = [0.0; 8];
        for (lane, slot) in words[..L::WIDTH].iter_mut().enumerate() {
            let ramp = &self.ramps[lane][parameter];
            *slot = if ramp.remaining > 0 { ramp.step } else { 0.0 };
        }
        L::load(&words[..L::WIDTH])
    }

    /// Copies the kernel's iterated values back into the control-plane ramps.
    fn synchronize_currents(&mut self) {
        for parameter in 0..PARAMETER_COUNT {
            let mut words: LaneWords = [0.0; 8];
            self.state.field(parameter).store(&mut words[..L::WIDTH]);
            for (lane, value) in words[..L::WIDTH].iter().enumerate() {
                self.ramps[lane][parameter].current = *value;
            }
        }
    }
}

impl<L: Lane> SoftClipState<L> {
    /// The parameter's lane vector, by stable index.
    fn field(&self, parameter: usize) -> L {
        match parameter {
            0 => self.drive,
            1 => self.output,
            _ => self.mix,
        }
    }

    /// The parameter's lane vector, by stable index, for a control-plane write.
    fn field_mut(&mut self, parameter: usize) -> &mut L {
        match parameter {
            0 => &mut self.drive,
            1 => &mut self.output,
            _ => &mut self.mix,
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Automation
// ---------------------------------------------------------------------------------------------

/// Applies one block's automation spans to a pair of channels' ramps.
///
/// The validation rules are the frozen ones: block-rate points only, at `first_sample`, in
/// ascending `(parameter, channel)` order, at most one per parameter and channel, inside the
/// prepared automation capacity. What changed is the tail: a target is now handed to
/// [`LinearRamp::set_target`], which divides **once** (D11), instead of being re-divided by the
/// remaining count on every sample.
fn apply_automation<L: Lane>(
    spans: &[PreparedAutomationSpan],
    metadata: PreparedEffectMetadata,
    first_sample: u64,
    lane: usize,
    left: &mut Channel<L>,
    right: &mut Channel<L>,
    report: &mut ProcessReport,
) {
    let mut pending = [[None; PARAMETER_COUNT]; 2];
    let mut prior = None;
    for (span_index, span) in spans.iter().enumerate() {
        let channel = match span.channel {
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
            .and_then(|value| value.checked_add(channel as u32))
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
            && prior.is_none_or(|previous| order > previous)
            && pending[channel][parameter].is_none();
        if !valid {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        }
        let Some(value) = convert_parameter(parameter, normalize_zero(span.start_value)) else {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        };
        prior = Some(order);
        pending[channel][parameter] = Some(value);
    }
    for (parameter, (left_target, right_target)) in
        pending[0].into_iter().zip(pending[1]).enumerate()
    {
        if let Some(target) = left_target {
            left.ramps[lane][parameter].set_target(target, RAMP_SAMPLES);
        }
        if let Some(target) = right_target {
            right.ramps[lane][parameter].set_target(target, RAMP_SAMPLES);
        }
    }
}

// ---------------------------------------------------------------------------------------------
// State payload, layout version 2
// ---------------------------------------------------------------------------------------------

/// Reads one lane of one channel into the 104 payload words of layout 2.
fn write_lane_words<L: Lane>(channel: &Channel<L>, lane: usize, words: &mut [u32]) {
    debug_assert_eq!(words.len(), LANE_STATE_WORDS as usize);
    for parameter in 0..PARAMETER_COUNT {
        let ramp = &channel.ramps[lane][parameter];
        let base = parameter * RAMP_WORDS;
        words[base] = lane_of(channel.state.field(parameter), lane).to_bits();
        words[base + 1] = ramp.target.to_bits();
        words[base + 2] = ramp.step.to_bits();
        words[base + 3] = ramp.remaining;
    }
    let width = L::WIDTH;
    let position = channel.history.pos as usize;
    for (offset, source, ages) in [
        (X_HISTORY_WORD, &channel.history.x, X_HISTORY_AGES),
        (E_HISTORY_WORD, &channel.history.e, E_HISTORY_AGES),
        (DRY_HISTORY_WORD, &channel.history.dry, DRY_HISTORY_AGES),
    ] {
        for age in 0..ages {
            let row = history_row_of_age(position, age);
            words[offset + age] = source[row * width + lane].to_bits();
        }
    }
}

/// The live row holding the value written `age + 1` frames before the next write position.
///
/// `pos` is where the *next* sample will be written, so age 0 is the sample just rendered.
fn history_row_of_age(position: usize, age: usize) -> usize {
    (position + LIVE_ROWS - 1 - age) & POS_MASK
}

/// Diagnostic codes this crate raises for payload contents the shared codec does not own.
const STATE_PARAMETER_CODE: &str = "effect.state.parameter";
const STATE_HISTORY_CODE: &str = "effect.state.history";

/// One lane's decoded, validated payload contents, before anything is written.
///
/// Decoding and applying are separate so that a rejected restore cannot leave an effect half
/// updated and so that neither step allocates: the contract allows a restore to fail, and a
/// half-restored bank member would be worse than a rejected one.
struct LaneRestore {
    ramps: [LinearRamp; PARAMETER_COUNT],
    x: [f32; X_HISTORY_AGES],
    e: [f32; E_HISTORY_AGES],
    dry: [f32; DRY_HISTORY_AGES],
}

/// Validates the 104 payload words of layout 2.
///
/// Ramp currents and targets must be inside the *converted* domain (a linear gain, not decibels),
/// must not be `-0.0` and must not be subnormal; `step` must be finite and normal-or-zero;
/// `remaining` must not exceed the smoothing window; every history word must be finite and either
/// zero or normal. There is no cursor word to validate any more — the cursor belongs to the bank.
fn decode_lane_words(words: &[u32]) -> Result<LaneRestore, StatePayloadError> {
    debug_assert_eq!(words.len(), LANE_STATE_WORDS as usize);
    let mut ramps = [LinearRamp::fixed(0.0); PARAMETER_COUNT];
    for (parameter, ramp) in ramps.iter_mut().enumerate() {
        let base = parameter * RAMP_WORDS;
        let current = f32::from_bits(words[base]);
        let target = f32::from_bits(words[base + 1]);
        let step = f32::from_bits(words[base + 2]);
        let remaining = words[base + 3];
        if !converted_value_valid(parameter, current)
            || !converted_value_valid(parameter, target)
            || !normal_or_zero(step)
            || remaining > RAMP_SAMPLES
        {
            return Err(StatePayloadError {
                code: STATE_PARAMETER_CODE,
            });
        }
        *ramp = LinearRamp {
            current,
            target,
            step,
            remaining,
        };
    }
    let mut restore = LaneRestore {
        ramps,
        x: [0.0; X_HISTORY_AGES],
        e: [0.0; E_HISTORY_AGES],
        dry: [0.0; DRY_HISTORY_AGES],
    };
    for (slot, offset) in [
        (&mut restore.x[..], X_HISTORY_WORD),
        (&mut restore.e[..], E_HISTORY_WORD),
        (&mut restore.dry[..], DRY_HISTORY_WORD),
    ] {
        for (age, value) in slot.iter_mut().enumerate() {
            let word = f32::from_bits(words[offset + age]);
            if !normal_or_zero(word) {
                return Err(StatePayloadError {
                    code: STATE_HISTORY_CODE,
                });
            }
            *value = word;
        }
    }
    Ok(restore)
}

/// `true` if `value` is finite and either a zero or a normal. Signed zeros are accepted.
fn normal_or_zero(value: f32) -> bool {
    value.is_finite() && (value == 0.0 || value.is_normal())
}

/// Writes one decoded lane into a channel, relative to that channel's shared cursor.
///
/// Ages are placed at `pos + 31 - age` and mirrored, so a restored track lines up with the bank it
/// joins whatever position the bank happens to be at — which is what makes one cursor per bank
/// correct (issue #91 F3). Rows the payload does not carry are zeroed, so a restore leaves no
/// residue of the state it replaced.
fn apply_lane_words<L: Lane>(channel: &mut Channel<L>, lane: usize, restore: &LaneRestore) {
    let width = L::WIDTH;
    let position = channel.history.pos as usize;
    for (values, target) in [
        (&restore.x[..], &mut channel.history.x),
        (&restore.e[..], &mut channel.history.e),
        (&restore.dry[..], &mut channel.history.dry),
    ] {
        for age in 0..LIVE_ROWS {
            let value = values.get(age).copied().unwrap_or(0.0);
            let row = history_row_of_age(position, age);
            target[row * width + lane] = value;
            target[(row + LIVE_ROWS) * width + lane] = value;
        }
    }
    for (parameter, ramp) in restore.ramps.into_iter().enumerate() {
        set_lane(channel.state.field_mut(parameter), lane, ramp.current);
        channel.ramps[lane][parameter] = ramp;
    }
}

/// The contract's payload sections, as the shared codec wants them.
fn snapshot_sections<L: Lane>(
    left: &Channel<L>,
    right: &Channel<L>,
    lane: usize,
    output: StatePayloadOutput<'_>,
) -> Result<(), StatePayloadError> {
    let mut left_words = [0_u32; LANE_STATE_WORDS as usize];
    let mut right_words = [0_u32; LANE_STATE_WORDS as usize];
    write_lane_words(left, lane, &mut left_words);
    write_lane_words(right, lane, &mut right_words);
    let mut out = payload::StatePayloadOutput {
        common: output.common,
        left: output.left,
        right: output.right,
    };
    payload::snapshot(
        &STATE_LAYOUT,
        &payload::StateWords {
            common: &[],
            left: &left_words,
            right: &right_words,
        },
        &mut out,
    )
    .map_err(runtime_state_error)
}

/// Restores one lane of a pair of channels from a payload, rejecting before writing anything.
fn restore_sections<L: Lane>(
    left: &mut Channel<L>,
    right: &mut Channel<L>,
    lane: usize,
    state_layout_version: u32,
    input: StatePayloadInput<'_>,
) -> Result<(), StatePayloadError> {
    if state_layout_version != STATE_LAYOUT.version {
        return Err(StatePayloadError {
            code: payload::STATE_VERSION_CODE,
        });
    }
    let mut left_words = [0_u32; LANE_STATE_WORDS as usize];
    let mut right_words = [0_u32; LANE_STATE_WORDS as usize];
    payload::restore(
        &STATE_LAYOUT,
        &payload::StatePayloadInput {
            common: input.common,
            left: input.left,
            right: input.right,
        },
        &mut payload::StateWordsMut {
            common: &mut [],
            left: &mut left_words,
            right: &mut right_words,
        },
    )
    .map_err(runtime_state_error)?;
    let left_restore = decode_lane_words(&left_words)?;
    let right_restore = decode_lane_words(&right_words)?;
    apply_lane_words(left, lane, &left_restore);
    apply_lane_words(right, lane, &right_restore);
    Ok(())
}

/// Maps the shared codec's error onto the contract's.
fn runtime_state_error(error: payload::StatePayloadError) -> StatePayloadError {
    StatePayloadError { code: error.code }
}

// ---------------------------------------------------------------------------------------------
// Prepared instances
// ---------------------------------------------------------------------------------------------

/// A prepared soft-clip cohort of `L::WIDTH` dual-mono tracks.
///
/// `WIDTH = 1` is the scalar instance the contract's `PreparedNativeEffect` uses, and 4 and 8 are
/// the banks; the type, the driver and the kernel are the same in all three cases.
struct SoftClip<L: Lane> {
    metadata: PreparedEffectMetadata,
    left_defaults: Box<[[f32; PARAMETER_COUNT]]>,
    right_defaults: Box<[[f32; PARAMETER_COUNT]]>,
    left: Channel<L>,
    right: Channel<L>,
    nonfinite: NonFiniteReport,
}

impl<L: Lane> SoftClip<L> {
    fn new(
        metadata: PreparedEffectMetadata,
        left_defaults: Box<[[f32; PARAMETER_COUNT]]>,
        right_defaults: Box<[[f32; PARAMETER_COUNT]]>,
    ) -> Self {
        let left = Channel::new(&left_defaults);
        let right = Channel::new(&right_defaults);
        Self {
            metadata,
            left_defaults,
            right_defaults,
            left,
            right,
            nonfinite: NonFiniteReport::new(),
        }
    }

    fn reset(&mut self, kind: ResetKind) {
        match kind {
            ResetKind::FullToDefaults => {
                self.left.reset_full(&self.left_defaults);
                self.right.reset_full(&self.right_defaults);
            }
            ResetKind::DiscontinuityKeepParameters => {
                self.left.reset_discontinuity();
                self.right.reset_discontinuity();
            }
        }
    }

    /// Renders one block of both channels and applies the master plan §4.4 boundary check.
    ///
    /// Returns `true` if the block was accepted. On rejection both channels are zeroed, every
    /// lane's histories are cleared and its ramps snapped, and the bank's `nonfinite_blocks`
    /// counter advances by one — a *block*, never a sample.
    fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: usize,
        bypass: bool,
    ) -> bool {
        self.left.process(left, frames, bypass);
        self.right.process(right, frames, bypass);
        let Self {
            left: left_channel,
            right: right_channel,
            nonfinite,
            ..
        } = self;
        finish_block::<L>(left, right, nonfinite, || {
            left_channel.reset_discontinuity();
            right_channel.reset_discontinuity();
        })
    }
}

/// A prepared scalar soft-clip dual-mono instance.
struct PreparedSoftClip {
    inner: SoftClip<f32>,
}

/// A prepared homogeneous soft-clip cohort.
struct PreparedSoftClipBank<L: Lane> {
    metadata: PreparedBankMetadata,
    inner: SoftClip<L>,
}

impl NativeEffectFactory for SoftClipFactory {
    fn descriptor(&self) -> &'static EffectDescriptor {
        &SOFT_CLIP_DESCRIPTOR
    }

    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        let metadata = expected_prepared_metadata(self.descriptor(), request)?;
        let (left, right) = initial_defaults(request.initial_values)?;
        Ok(Box::new(PreparedSoftClip {
            inner: SoftClip::new(
                metadata,
                vec![left].into_boxed_slice(),
                vec![right].into_boxed_slice(),
            ),
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
            BankWidth::Four => prepare_bank::<Simd4>(self, request),
            BankWidth::Eight => prepare_bank::<Simd8>(self, request),
        }
    }
}

/// `true` if this artifact executes `width` lanes natively.
///
/// D4 replaced runtime SIMD dispatch with a compile-time ISA pin plus a boot attestation, so this
/// is a `cfg` question and not a CPUID one. A width the artifact was not built for is declined
/// with `Ok(None)`, exactly as the deleted `PreparedSoftClipBankKernel::try_new` declined an
/// unavailable backend, and the caller falls back to scalar instances.
const fn width_is_native(width: BankWidth) -> bool {
    match width {
        BankWidth::Four => cfg!(any(
            target_arch = "aarch64",
            all(target_arch = "wasm32", target_feature = "simd128")
        )),
        BankWidth::Eight => cfg!(any(target_arch = "x86", target_arch = "x86_64")),
    }
}

fn prepare_bank<L: Lane>(
    factory: &SoftClipFactory,
    request: PrepareEffectBankRequest<'_>,
) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
    let first = request
        .requests
        .first()
        .copied()
        .ok_or(EffectPrepareError {
            code: "effect.bank.requests",
        })?;
    let metadata = expected_prepared_metadata(factory.descriptor(), first)?;
    let (first_left, first_right) = initial_defaults(first.initial_values)?;
    let mut left_defaults = vec![first_left; L::WIDTH];
    let mut right_defaults = vec![first_right; L::WIDTH];
    let mut same_program = true;
    for (track, member) in request.requests.iter().copied().enumerate() {
        let candidate = expected_prepared_metadata(factory.descriptor(), member)?;
        if candidate.program_key() != metadata.program_key() {
            same_program = false;
        }
        let (left, right) = initial_defaults(member.initial_values)?;
        left_defaults[track] = left;
        right_defaults[track] = right;
    }
    if !same_program || !width_is_native(request.width) {
        return Ok(None);
    }
    Ok(Some(Box::new(PreparedSoftClipBank::<L> {
        metadata: PreparedBankMetadata {
            width: request.width,
            program_key: metadata.program_key(),
        },
        inner: SoftClip::new(
            metadata,
            left_defaults.into_boxed_slice(),
            right_defaults.into_boxed_slice(),
        ),
    })))
}

impl PreparedNativeEffect for PreparedSoftClip {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.inner.metadata
    }

    fn reset(&mut self, kind: ResetKind) {
        self.inner.reset(kind);
    }

    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        let mut report = ProcessReport::default();
        let frames = block.frames();
        apply_automation(
            block.automation,
            self.inner.metadata,
            block.first_sample,
            0,
            &mut self.inner.left,
            &mut self.inner.right,
            &mut report,
        );
        let bypass = self.inner.metadata.bypass;
        if !self.inner.process(block.left, block.right, frames, bypass) {
            let count = frames as u64;
            report.nonfinite_left_blocks = report.nonfinite_left_blocks.saturating_add(count);
            report.nonfinite_right_blocks = report.nonfinite_right_blocks.saturating_add(count);
        }
        report
    }

    fn snapshot_state_payload(
        &self,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        snapshot_sections(&self.inner.left, &self.inner.right, 0, output)
    }

    fn restore_state_payload(
        &mut self,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        restore_sections(
            &mut self.inner.left,
            &mut self.inner.right,
            0,
            state_layout_version,
            input,
        )
    }
}

impl<L: Lane> PreparedNativeEffectBank for PreparedSoftClipBank<L> {
    fn metadata(&self) -> PreparedBankMetadata {
        self.metadata.clone()
    }

    fn reset(&mut self, kind: ResetKind) {
        self.inner.reset(kind);
    }

    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        let mut report = BankProcessReport::empty(self.metadata.width);
        if !bank_block_matches(&block, self.metadata.width, self.inner.metadata.quantum)
            || L::WIDTH != self.metadata.width.lanes() as usize
        {
            return report;
        }
        for lane in 0..L::WIDTH {
            let start = block.automation_offsets[lane] as usize;
            let end = block.automation_offsets[lane + 1] as usize;
            apply_automation(
                &block.automation[start..end],
                self.inner.metadata,
                block.first_sample,
                lane,
                &mut self.inner.left,
                &mut self.inner.right,
                &mut report.reports[lane],
            );
        }
        let frames = block.frames as usize;
        let bypass = self.inner.metadata.bypass;
        if !self.inner.process(block.left, block.right, frames, bypass) {
            let count = frames as u64;
            for lane in 0..L::WIDTH {
                report.reports[lane].nonfinite_left_blocks = report.reports[lane]
                    .nonfinite_left_blocks
                    .saturating_add(count);
                report.reports[lane].nonfinite_right_blocks = report.reports[lane]
                    .nonfinite_right_blocks
                    .saturating_add(count);
            }
        }
        report
    }

    fn snapshot_track_state_payload(
        &self,
        track_index: u32,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        let lane = bank_track_index::<L>(track_index)?;
        snapshot_sections(&self.inner.left, &self.inner.right, lane, output)
    }

    fn restore_track_state_payload(
        &mut self,
        track_index: u32,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        let lane = bank_track_index::<L>(track_index)?;
        restore_sections(
            &mut self.inner.left,
            &mut self.inner.right,
            lane,
            state_layout_version,
            input,
        )
    }
}

fn bank_track_index<L: Lane>(track_index: u32) -> Result<usize, StatePayloadError> {
    let track = usize::try_from(track_index).map_err(|_| StatePayloadError {
        code: "effect.bank.track",
    })?;
    if track >= L::WIDTH {
        return Err(StatePayloadError {
            code: "effect.bank.track",
        });
    }
    Ok(track)
}

fn bank_block_matches(block: &EffectBankProcessBlock<'_>, width: BankWidth, quantum: u32) -> bool {
    let lanes = width.lanes() as usize;
    let Some(length) = (block.frames as usize).checked_mul(lanes) else {
        return false;
    };
    block.width == width
        && block.frames != 0
        && block.frames <= quantum
        && block.left.len() == length
        && block.right.len() == length
        && block.sidechain.is_none()
        && block
            .first_sample
            .checked_add(block.frames as u64)
            .is_some()
        && block.automation_offsets.len() == lanes + 1
        && block.automation_offsets.first() == Some(&0)
        && block.automation_offsets.last().copied() == Some(block.automation.len() as u32)
        && !block
            .automation_offsets
            .windows(2)
            .any(|pair| pair[0] > pair[1])
}
