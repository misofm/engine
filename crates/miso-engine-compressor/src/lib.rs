//! The launch feed-forward peak compressor.
//!
//! Descriptor, factory and contract glue. The signal path is `kernel`, one `Lane`-generic block
//! body instantiated at `f32`, `Simd4` and `Simd8`; the coefficient design is `design`, entirely
//! on the control plane; the payload codec is `state`.
//!
//! # What this crate no longer contains
//!
//! The audit of issue #88 found ten local copies of things the workspace now owns once. They are
//! gone, and with them the reasons they disagreed:
//!
//! | was here | is now |
//! |---|---|
//! | `Ramp` with a per-sample division | `effect_runtime::ramp::LinearRamp` (D11) |
//! | the five-way branching gain computer | `effect_runtime::dynamics::gain_delta_db` (GMR eq. 4) |
//! | `20 * log10` / `10^(x/20)` through libm | `dynamics::level_db` / `dynamics::gain_from_db` (D6) |
//! | `expf` per sample for the ballistics | `design::rate_coefficient`, at event rate, in `f64` |
//! | the two-rounding one-pole | `effect_runtime::envelope::rms_follow` + one `Lane::select` |
//! | `sanitize` / `recover` / `flushed` per value | `lane::flush` on `g`, one block-boundary check (D7) |
//! | `write_*` / `read_*` byte helpers | `effect_runtime::state_payload` |
//! | `parameter_value_valid` | `effect_runtime::params` |
//! | `PreparedCompressorGainMixKernelV1` | the `gain_mix_block` form inside the kernel (D10) |
//! | four integer modulos with a runtime divisor | integer compare-select wraps |
//!
//! # What is frozen
//!
//! The parameter table, the port list, the quality rows (latency `N = Fs/50`, `maximum_state`,
//! `scratch_fixed_bytes: 64`), the program key, the state payload layout and `state_layout_version
//! = 1`, the `L`/`D` derivation and the ring semantics, the link laws, and the four identities
//! (bypass, `mix == 0`, `G == 0 && makeup == +0`, `mix == 1`). Master plan §8.2: a contract fixture
//! does not move in a re-landing job. `scratch_fixed_bytes` is unused and stays anyway; #95 owns
//! the program key.
#![allow(missing_docs)]

mod design;
mod kernel;
mod state;

pub mod corpus;

use miso_engine_effect_contract::{
    AutomationRate, AutomationSpanKind, BankProcessReport, EffectBankProcessBlock,
    EffectDescriptorV1, EffectPrepareError, EffectProcessBlock, EffectQuality,
    InitialParameterValue, LatencySamples, LinkModeSet, NativeEffectFactory, ObservationCadenceV1,
    ObservationChannelsV1, ObservationCostV1, ObservationDescriptorV1, ObservationFoldV1,
    ObservationKindV1, ObservationSampleV1, ObservationTapId, ParameterChannel,
    ParameterChannelPolicy, ParameterDescriptorV1, ParameterDomain, ParameterId, ParameterMapping,
    ParameterUnit, PortDescriptorV1, PortId, PortLayout, PortRole, PrepareEffectBankRequest,
    PrepareEffectRequest, PreparedAutomationSpan, PreparedBankMetadata, PreparedEffectMetadata,
    PreparedNativeEffect, PreparedNativeEffectBank, ProcessReport, ResetKind, SmoothingRule,
    StatePayloadError, StatePayloadInput, StatePayloadOutput, StatePayloadSizes, TailSamples,
    expected_prepared_metadata,
};
use miso_engine_effect_runtime::bank::block_is_positive_zero;
use miso_engine_effect_runtime::params::{is_negative_zero, normalize_zero, parameter_value_valid};
use miso_engine_lane::{Backend, Lane, Simd4, Simd8};

use crate::design::{MAX_WIDTH, PARAMETER_COUNT, PARAMETER_SPECS, RAMP_COUNT, SMOOTHING_SAMPLES};
use crate::kernel::{Channel, Detector};

/// Fixed scalar words each channel section carries before its two ring arrays.
///
/// Public because it is part of the documented V1 payload layout (BRIEFS/013), not an internal
/// detail: a host that inspects a snapshot needs it to find the rings.
pub use crate::state::STATE_HEADER_WORDS;

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

/// The one declared observation tap: the smoothed reduction the kernel already holds.
///
/// `Resident`, because `Channel::gain_reduction_db` is written by `process_block` every sample and
/// read back on the next one -- publishing it is a copy out of state the block wrote anyway, and
/// no lane kernel is touched. `PeakMagnitude` over a window is what makes the published number a
/// non-negative magnitude even though the smoother's own convention is negative-for-reduction.
pub const COMPRESSOR_OBSERVATIONS_V1: [ObservationDescriptorV1; 1] = [ObservationDescriptorV1 {
    id: ObservationTapId(1),
    display_name: "Gain Reduction",
    display_unit: "dB",
    kind: ObservationKindV1::GainReductionDb,
    unit: ParameterUnit::Db,
    cost: ObservationCostV1::Resident,
    cadence: ObservationCadenceV1::PerBlock,
    fold: ObservationFoldV1::PeakMagnitude,
    channels: ObservationChannelsV1::PerLane,
    minimum: 0.0,
    maximum: 100.0,
}];

/// Immutable launch compressor descriptor.
pub const COMPRESSOR_DESCRIPTOR_V1: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("miso.compressor"),
    display_name: "Compressor",
    contract_major: 1,
    // Issue #143 P1: the observation menu is a declared, additive descriptor field, so declaring
    // the first tap is a `contract_minor` bump and a derived identity re-pin of exactly
    // `32 + len("Gain Reduction") + len("dB")` = 48 bytes. `state_layout_version` does not move:
    // no state byte changed, the tap is a read of state that was already there.
    contract_minor: 1,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::ALL,
    parameters: &COMPRESSOR_PARAMETERS_V1,
    ports: &PORTS,
    qualities: &QUALITIES,
    observations: &COMPRESSOR_OBSERVATIONS_V1,
};

/// Factory entry point for the V1 compressor implementation.
#[derive(Clone, Copy, Debug, Default)]
pub struct CompressorFactory;

/// Ring length `B = N + 1` for a prepared latency, or the resource-limit diagnostic.
fn ring_length(metadata: PreparedEffectMetadata) -> Result<usize, EffectPrepareError> {
    usize::try_from(metadata.latency.0)
        .ok()
        .and_then(|latency| latency.checked_add(1))
        .ok_or(EffectPrepareError {
            code: "effect.resource.limit",
        })
}

/// The preparation-time values of both channels of one request.
///
/// Rules unchanged from V1: exactly `2 * PARAMETER_COUNT` entries, in table order, alternating
/// left and right, every value inside its domain and not `-0.0`. The domain predicate is the
/// runtime's; the `-0.0` rejection is this effect's own preparation rule and stays.
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
    for index in 0..PARAMETER_COUNT {
        let left_value = values[index * 2];
        let right_value = values[index * 2 + 1];
        if left_value.parameter_index != index as u32
            || right_value.parameter_index != index as u32
            || left_value.channel != ParameterChannel::Left
            || right_value.channel != ParameterChannel::Right
            || !parameter_value_valid(&PARAMETER_SPECS[index], left_value.value)
            || !parameter_value_valid(&PARAMETER_SPECS[index], right_value.value)
            || is_negative_zero(left_value.value)
            || is_negative_zero(right_value.value)
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

/// Applies one lane's automation spans to its ramps.
///
/// Validation is unchanged from V1 and deliberately strict: a span must be inside the prepared
/// automation capacity, a `Point` at exactly `first_sample`, with `start_value` and `end_value`
/// bit-identical, inside the parameter's domain, addressed to a single channel, in strictly
/// increasing `(parameter, channel)` order, and not a duplicate. Anything else is counted in
/// `invalid_spans` and ignored — never applied partially.
fn apply_automation<L: Lane>(
    spans: &[PreparedAutomationSpan],
    metadata: PreparedEffectMetadata,
    first_sample: u64,
    lane: usize,
    left: &mut Channel<L>,
    right: &mut Channel<L>,
    report: &mut ProcessReport,
) {
    let mut pending = [[None; RAMP_COUNT]; 2];
    let mut last_order = None;
    for (span_index, span) in spans.iter().enumerate() {
        let channel_index = match span.channel {
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
            .and_then(|value| value.checked_add(channel_index as u32))
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
            && parameter_value_valid(&PARAMETER_SPECS[parameter_index], span.start_value)
            && last_order.is_none_or(|previous| order > previous)
            && pending[channel_index][parameter_index].is_none();
        if !valid {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        }
        last_order = Some(order);
        pending[channel_index][parameter_index] = Some(normalize_zero(span.start_value));
    }
    for (parameter_index, (left_ramp, right_ramp)) in left
        .ramps
        .iter_mut()
        .zip(right.ramps.iter_mut())
        .enumerate()
    {
        if let Some(value) = pending[0][parameter_index] {
            left_ramp[lane].set_target(value, SMOOTHING_SAMPLES);
        }
        if let Some(value) = pending[1][parameter_index] {
            right_ramp[lane].set_target(value, SMOOTHING_SAMPLES);
        }
    }
}

/// Which detector source a prepared configuration and a block imply.
fn detector_source<'a>(
    metadata: PreparedEffectMetadata,
    sidechain: Option<(&'a [f32], &'a [f32])>,
) -> Detector<'a> {
    if !matches!(
        metadata.ports.sidechain,
        miso_engine_effect_contract::PreparedSidechainPort::Connected { .. }
    ) {
        return Detector::Main;
    }
    match sidechain {
        Some((left, right)) => Detector::Sidechain(left, right),
        None => Detector::Silent,
    }
}

/// The track index a bank call names, or the track diagnostic.
fn checked_track(track_index: u32, width: usize) -> Result<usize, StatePayloadError> {
    let track = usize::try_from(track_index).map_err(|_| StatePayloadError {
        code: "effect.state.track",
    })?;
    if track >= width {
        return Err(StatePayloadError {
            code: "effect.state.track",
        });
    }
    Ok(track)
}

/// Everything a prepared compressor is, at one lane width.
///
/// The scalar instance and the bank are the same object at `L = f32` and `L = Simd4 | Simd8`, and
/// they share every method below. Only the two contract traits differ — one addresses lane 0 and
/// reports into a `ProcessReport`, the other addresses a track and reports into an array of them —
/// so the traits are thin wrappers and there is exactly one body per operation. A second copy of
/// `restore` is precisely the divergence the audit found in six other crates.
struct Instance<L: Lane> {
    metadata: PreparedEffectMetadata,
    left: Channel<L>,
    right: Channel<L>,
    /// Issue #163 phase 4 item 1: the previous block proved this instance is at a silent fixed
    /// point. Earned only by observation in [`render`](Self::render), never assumed. See the
    /// matching field on `miso-engine-parametric-eq` for the induction it licenses.
    silent_fixed_point: bool,
    /// The bypass flag in force when the claim above was earned. Bypass selects a different path
    /// through the kernel, so a claim earned on one side of it says nothing about the other.
    silent_bypass: bool,
}

impl<L: Lane> Instance<L> {
    /// Allocates both channels from per-lane preparation values.
    fn new(
        metadata: PreparedEffectMetadata,
        left_defaults: &[[f32; PARAMETER_COUNT]; MAX_WIDTH],
        right_defaults: &[[f32; PARAMETER_COUNT]; MAX_WIDTH],
        length: usize,
    ) -> Self {
        Self {
            metadata,
            left: Channel::new(left_defaults, length, metadata.sample_rate),
            right: Channel::new(right_defaults, length, metadata.sample_rate),
            silent_fixed_point: false,
            silent_bypass: metadata.bypass,
        }
    }

    fn reset(&mut self, kind: ResetKind) {
        // #163 phase 4 item 1: a reset moves the rings and the recursive word, so the claim goes.
        self.silent_fixed_point = false;
        let rate = self.metadata.sample_rate;
        match kind {
            ResetKind::FullToDefaults => {
                self.left.full_reset(rate);
                self.right.full_reset(rate);
            }
            ResetKind::DiscontinuityKeepParameters => {
                self.left.discontinuity_reset(rate);
                self.right.discontinuity_reset(rate);
            }
        }
    }

    /// Renders one block and applies the section 4.4 boundary policy to each channel.
    ///
    /// `record(lane, left_failed, right_failed)` is called once per lane, and only when a block
    /// was rejected, so the caller maps the failure onto whichever report shape it owns.
    fn render(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        detector: Detector<'_>,
        frames: usize,
        mut record: impl FnMut(usize, bool, bool),
    ) {
        let words = frames * L::WIDTH;
        // Issue #163 phase 4 item 1. Whole-bank, never per lane. Four legs, all cheap:
        //
        // * no ramp in flight on either channel, so the coefficient words are the ones the
        //   observed block used;
        // * the detector is the main input rather than a connected sidechain, so there is no
        //   second buffer whose contents could differ from the block that was observed;
        // * the bypass flag is the one that was in force when the claim was earned, since it
        //   selects a different path through the kernel;
        // * both input planes are exactly `+0.0`, which short-circuits on the first chunk for a
        //   block carrying signal.
        let quiet = self.left.max_remaining() == 0
            && self.right.max_remaining() == 0
            && matches!(detector, Detector::Main | Detector::Silent)
            && self.silent_bypass == self.metadata.bypass
            && block_is_positive_zero(&left[..words])
            && block_is_positive_zero(&right[..words]);
        if quiet && self.silent_fixed_point {
            // Both rings are known all-`+0.0` (that is one of the legs the claim was earned on),
            // the one recursive word is at its fixed point, and the buffers already hold the
            // `+0.0` the kernel would have written. The cursor is the only state that must still
            // move: advancing it by `frames` is exactly what `frames` per-sample increments do,
            // and it keeps the state bit-identical to the slow path rather than merely
            // equivalent.
            self.left.advance_cursor(frames as u32);
            self.right.advance_cursor(frames as u32);
            return;
        }
        let before = quiet.then(|| (self.left.recursive_bits(), self.right.recursive_bits()));
        kernel::process_block::<L>(
            left,
            right,
            detector,
            frames,
            self.metadata.link_mode,
            self.metadata.bypass,
            self.metadata.sample_rate,
            (&mut self.left, &mut self.right),
        );
        // Earn or lose the claim from what this block actually did: the recursive gain-reduction
        // word came out as it went in, both delay rings are entirely `+0.0` (so a later cursor
        // position reads the same silence a slow path would), and the output is `+0.0` to the bit.
        self.silent_fixed_point = match before {
            Some((left_before, right_before)) => {
                left_before == self.left.recursive_bits()
                    && right_before == self.right.recursive_bits()
                    && self.left.rings_are_positive_zero()
                    && self.right.rings_are_positive_zero()
                    && block_is_positive_zero(&left[..words])
                    && block_is_positive_zero(&right[..words])
            }
            None => false,
        };
        self.silent_bypass = self.metadata.bypass;
        let left_mask = kernel::finish_channel::<L>(left, &mut self.left);
        let right_mask = kernel::finish_channel::<L>(right, &mut self.right);
        if left_mask | right_mask == 0 {
            return;
        }
        for lane in 0..L::WIDTH {
            let bit = 1 << lane;
            record(lane, left_mask & bit != 0, right_mask & bit != 0);
        }
    }

    /// Writes one lane's payload.
    fn snapshot(
        &self,
        mut output: StatePayloadOutput<'_>,
        lane: usize,
    ) -> Result<(), StatePayloadError> {
        state::validate_lengths(
            output.common.len(),
            output.left.len(),
            output.right.len(),
            self.metadata.state_sizes,
        )?;
        state::snapshot_lane(&mut output, &self.left, &self.right, lane);
        Ok(())
    }

    /// Restores one lane's payload, transactionally across both channels.
    fn restore(
        &mut self,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
        lane: usize,
    ) -> Result<(), StatePayloadError> {
        // #163 phase 4 item 1: a restore writes rings, the recursive word and the coefficients
        // from a payload this instance never rendered, so any standing claim is void. Withdrawn
        // before the version check so a rejected restore cannot leave a half-trusted claim either.
        self.silent_fixed_point = false;
        if state_layout_version != COMPRESSOR_DESCRIPTOR_V1.state_layout_version {
            return Err(StatePayloadError {
                code: "effect.state.version",
            });
        }
        state::validate_lengths(
            input.common.len(),
            input.left.len(),
            input.right.len(),
            self.metadata.state_sizes,
        )?;
        let length = self.left.ring_length as usize;
        state::validate_channel(input.left, length)?;
        state::validate_channel(input.right, length)?;
        let rate = self.metadata.sample_rate;
        state::commit_channel(input.left, &mut self.left, lane, rate);
        state::commit_channel(input.right, &mut self.right, lane, rate);
        Ok(())
    }
}

/// A prepared, allocation-free scalar compressor instance: the `L = f32` instantiation.
pub struct PreparedCompressor {
    instance: Instance<f32>,
}

/// A prepared homogeneous bank: `L::WIDTH` tracks as one vector, same kernel body.
struct PreparedCompressorBank<L: Lane> {
    metadata: PreparedBankMetadata,
    instance: Instance<L>,
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
        let length = ring_length(metadata)?;
        Ok(Box::new(PreparedCompressor {
            instance: Instance::new(
                metadata,
                &[left_defaults; MAX_WIDTH],
                &[right_defaults; MAX_WIDTH],
                length,
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
        let lanes = request.width.lanes() as usize;
        let first = request.requests[0];
        let metadata = expected_prepared_metadata(self.descriptor(), first)?;
        let (first_left, first_right) = initial_defaults(first.initial_values)?;
        let mut left_defaults = [first_left; MAX_WIDTH];
        let mut right_defaults = [first_right; MAX_WIDTH];
        let mut same_program = true;
        // Every request is validated before any fallback is taken: an unavailable backend or a
        // connected sidechain must never hide a malformed request (E13).
        for (track, item) in request.requests.iter().copied().enumerate() {
            let candidate = expected_prepared_metadata(self.descriptor(), item)?;
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
        // D4, revision 4: there is no runtime SIMD dispatch. This build has exactly one lane
        // width, decided at compile time and attested at boot, so a bank of any other width is a
        // backend that is not available here — the `Ok(None)` scalar fallback, never an error.
        if Backend::current().width() != lanes {
            return Ok(None);
        }
        let length = ring_length(metadata)?;
        let bank_metadata = PreparedBankMetadata {
            width: request.width,
            program_key: metadata.program_key(),
        };
        Ok(Some(match Backend::current() {
            Backend::Simd4 => Box::new(PreparedCompressorBank::<Simd4> {
                metadata: bank_metadata,
                instance: Instance::new(metadata, &left_defaults, &right_defaults, length),
            }) as Box<dyn PreparedNativeEffectBank>,
            Backend::Simd8 => Box::new(PreparedCompressorBank::<Simd8> {
                metadata: bank_metadata,
                instance: Instance::new(metadata, &left_defaults, &right_defaults, length),
            }) as Box<dyn PreparedNativeEffectBank>,
            Backend::Scalar => return Ok(None),
        }))
    }
}

impl PreparedNativeEffect for PreparedCompressor {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.instance.metadata
    }

    fn reset(&mut self, kind: ResetKind) {
        self.instance.reset(kind);
    }

    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        // #163 phase 4 item 1, as in `process_bank`.
        if !block.automation.is_empty() {
            self.instance.silent_fixed_point = false;
        }
        let mut report = ProcessReport::default();
        let metadata = self.instance.metadata;
        apply_automation(
            block.automation,
            metadata,
            block.first_sample,
            0,
            &mut self.instance.left,
            &mut self.instance.right,
            &mut report,
        );
        let frames = block.left.len();
        self.instance.render(
            block.left,
            block.right,
            detector_source(metadata, block.sidechain),
            frames,
            |_, left_failed, right_failed| {
                if left_failed {
                    report.nonfinite_left_blocks = report.nonfinite_left_blocks.saturating_add(1);
                }
                if right_failed {
                    report.nonfinite_right_blocks = report.nonfinite_right_blocks.saturating_add(1);
                }
            },
        );
        report
    }

    /// Issue #143 D2: the smoothed reduction the kernel already tracks, read for lane 0.
    ///
    /// `Channel::gain_reduction_db` is the compressor's own smoother state -- the value
    /// `process_block` writes every sample and reads back on the next one -- so this is a read,
    /// never a second opinion about what the kernel did. It is `<= 0` by the kernel's own clamp
    /// (`gain_delta_db` is clamped into `[-100, 0]` before it is smoothed), and it is the value at
    /// the end of the last processed block. At `L = f32` the channel's word *is* the lane's word.
    fn observe_resident(&self, tap_index: u32, out: &mut ObservationSampleV1) -> bool {
        if tap_index != 0 {
            return false;
        }
        out.left = self.instance.left.gain_reduction_db;
        out.right = self.instance.right.gain_reduction_db;
        true
    }

    fn snapshot_state_payload(
        &self,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.instance.snapshot(output, 0)
    }

    fn restore_state_payload(
        &mut self,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.instance.restore(state_layout_version, input, 0)
    }
}

impl<L: Lane> PreparedNativeEffectBank for PreparedCompressorBank<L> {
    fn metadata(&self) -> PreparedBankMetadata {
        self.metadata.clone()
    }

    /// One `store` per channel fills every lane: the bank's reduction is one vector.
    fn observe_resident_bank(&self, tap_index: u32, out: &mut [ObservationSampleV1]) -> bool {
        let lanes = L::WIDTH;
        if tap_index != 0 || out.len() != lanes {
            return false;
        }
        let mut left = [0.0_f32; MAX_WIDTH];
        let mut right = [0.0_f32; MAX_WIDTH];
        self.instance
            .left
            .gain_reduction_db
            .store(&mut left[..lanes]);
        self.instance
            .right
            .gain_reduction_db
            .store(&mut right[..lanes]);
        for (lane, sample) in out.iter_mut().enumerate() {
            sample.left = left[lane];
            sample.right = right[lane];
        }
        true
    }

    fn reset(&mut self, kind: ResetKind) {
        self.instance.reset(kind);
    }

    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        let mut report = BankProcessReport::empty(self.metadata.width);
        let lanes = L::WIDTH;
        let frames = block.frames as usize;
        // The pre-audit guard was four inline conditions that indexed the automation offsets
        // before checking them and accepted `frames == 0`. This is the strengthened form: it
        // rejects rather than indexes.
        //
        // #95 landed and `miso_engine_effect_contract::validate_automation_block` now exists,
        // but it is deliberately NOT adopted here, for two independent reasons.
        //
        // 1. Different subject. This guard checks the *bank block shape* -- width, lane count,
        //    buffer lengths and the per-lane `automation_offsets` array -- none of which the
        //    shared validator knows about. Only `frames == 0` and `frames > quantum` overlap.
        //    Span *content* is validated per lane by `apply_automation`, below.
        // 2. Opposite failure semantics. `apply_automation` drops an invalid span, counts it in
        //    `ProcessReport::invalid_spans`, and still applies the valid spans of the same block;
        //    `validate_automation_block` rejects the whole block on the first invalid span. The
        //    two also disagree on what "valid" is -- this effect accepts only a `Point` at
        //    exactly `first_sample` and rejects `ParameterChannel::Both` outright, while the
        //    shared validator admits Step/Linear/Exponential and *requires* `Both` for
        //    `ParameterChannelPolicy::Shared`. Adopting it would change both rendered PCM and
        //    the reported `invalid_spans` for any block mixing valid and invalid spans.
        if block.width != self.metadata.width
            || lanes != self.metadata.width.lanes() as usize
            || frames == 0
            || block.frames > self.instance.metadata.quantum
            || block.sidechain.is_some()
            || block.left.len() != frames * lanes
            || block.right.len() != frames * lanes
            || block.automation_offsets.len() != lanes + 1
            || !offsets_are_ordered(block.automation_offsets, block.automation.len())
        {
            return report;
        }
        // #163 phase 4 item 1: an admitted span can redesign coefficients, and a span whose
        // smoothing resolves to zero samples snaps them outright while leaving `max_remaining` at
        // zero -- so the ramp test alone would not notice it. Withdraw the claim whenever this
        // block carries automation at all; the next settled silent block re-earns it.
        if !block.automation.is_empty() {
            self.instance.silent_fixed_point = false;
        }
        let metadata = self.instance.metadata;
        for track in 0..lanes {
            let start = block.automation_offsets[track] as usize;
            let end = block.automation_offsets[track + 1] as usize;
            apply_automation(
                &block.automation[start..end],
                metadata,
                block.first_sample,
                track,
                &mut self.instance.left,
                &mut self.instance.right,
                &mut report.reports[track],
            );
        }
        self.instance.render(
            block.left,
            block.right,
            Detector::Main,
            frames,
            |lane, left_failed, right_failed| {
                if left_failed {
                    report.reports[lane].nonfinite_left_blocks =
                        report.reports[lane].nonfinite_left_blocks.saturating_add(1);
                }
                if right_failed {
                    report.reports[lane].nonfinite_right_blocks = report.reports[lane]
                        .nonfinite_right_blocks
                        .saturating_add(1);
                }
            },
        );
        report
    }

    fn snapshot_track_state_payload(
        &self,
        track_index: u32,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        let track = checked_track(track_index, L::WIDTH)?;
        self.instance.snapshot(output, track)
    }

    fn restore_track_state_payload(
        &mut self,
        track_index: u32,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        let track = checked_track(track_index, L::WIDTH)?;
        self.instance.restore(state_layout_version, input, track)
    }
}

/// Automation offsets must be non-decreasing and inside the span slice before they are indexed.
fn offsets_are_ordered(offsets: &[u32], spans: usize) -> bool {
    let mut previous = 0;
    for offset in offsets {
        if *offset < previous || *offset as usize > spans {
            return false;
        }
        previous = *offset;
    }
    true
}
