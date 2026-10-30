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
//! | `expf` per sample for the ballistics | exact event-rate endpoints plus coefficient interpolation |
//! | the two-rounding one-pole | `effect_runtime::envelope::rms_follow` + one `Lane::select` |
//! | `sanitize` / `recover` / `flushed` per value | the rest snap on `g` (the D7 flush, raised to the gain plateau by #1784), one block-boundary check (D7) |
//! | `write_*` / `read_*` byte helpers | `effect_runtime::state_payload` |
//! | `parameter_value_valid` | `effect_runtime::params` |
//! | `PreparedCompressorGainMixKernel` | the `gain_mix_block` form inside the kernel (D10) |
//! | four integer modulos with a runtime divisor | integer compare-select wraps |
//!
//! # What is frozen
//!
//! The parameter table, the port list, the quality rows (zero processing latency and the
//! conservative state reservation), the program key, the state payload layout and
//! `state_layout_version = 1`, the link laws, and the four identities (bypass, `mix == 0`,
//! `G == 0 && makeup == +0`, `mix == 1`).
#![allow(missing_docs)]

mod design;
#[cfg(test)]
mod glide_tests;
mod kernel;
#[cfg(test)]
mod padding_tests;
#[cfg(test)]
mod rest_tests;
mod state;
#[cfg(test)]
mod swap_tests;

pub mod corpus;

use effect_contract::{
    AutomationRate, BankProcessReport, BankWidth, EffectBankProcessBlock, EffectDescriptor,
    EffectPrepareError, EffectProcessBlock, EffectQuality, InitialParameterValue, LatencySamples,
    LinkMode, LinkModeSet, NativeEffectFactory, ObservationCadence, ObservationChannels,
    ObservationCost, ObservationDescriptor, ObservationFold, ObservationKind, ObservationSample,
    ObservationTapId, ParameterAccessError, ParameterChannel, ParameterChannelPolicy,
    ParameterDescriptor, ParameterDomain, ParameterId, ParameterMapping, ParameterUnit,
    PortDescriptor, PortId, PortLayout, PortRole, PrepareEffectBankRequest, PrepareEffectRequest,
    PreparedAutomationSpan, PreparedBankMetadata, PreparedEffect, PreparedEffectBank,
    PreparedEffectMetadata, PreparedNativeEffect, PreparedNativeEffectBank, PreparedParameterState,
    ProcessReport, ResetKind, SmoothingRule, StatePayloadError, StatePayloadInput,
    StatePayloadOutput, StatePayloadSizes, TailSamples, expected_prepared_metadata,
};
use effect_runtime::bank::{block_is_positive_zero, check_block, nonfinite_lane_mask};
use effect_runtime::params::{is_negative_zero, normalize_zero, parameter_value_valid};
use lane::{Backend, Lane};

use crate::design::{COEF_COUNT, MAX_WIDTH, PARAMETER_COUNT, PARAMETER_SPECS, RAMP_COUNT};
use crate::kernel::{Channel, Detector};

#[cfg(test)]
thread_local! {
    /// Blocks the silent fast path skipped (issue #1090 gate 3), counted once per skipped block.
    static SILENT_ADMISSIONS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}

/// Counts one silent admission. Test builds only: production has no counter.
#[cfg(test)]
fn admitted_silent() {
    SILENT_ADMISSIONS.with(|blocks| blocks.set(blocks.get() + 1));
}

/// The D7 boundary check of one channel's block, applied **per lane** (issue #1090).
///
/// The clean block, the common case, costs what it always did: one [`check_block`] scan and one
/// `mask_any`. A rejected block zeroes and resets only the lanes that left the bound, and returns
/// their bitmask. At `L = f32` the one lane is the whole channel, so a scalar instance keeps the
/// policy it always had: its channel block is zeroed and its envelope cleared.
///
/// # Why per lane
///
/// A bank used to zero every lane of the channel and clear every lane's envelope when one lane
/// failed. That coupled bank-mates' bits (decision 12's coupling rule): one track fed a legal but
/// tripping value, or a bypassed track whose wet block the rack discards anyway, silenced every
/// other track of its bank. Per lane, each lane is recovered exactly as the same track rendered
/// per node is. The check is a per-word predicate, so a lane fails exactly when its own instance's
/// block would; and a rejection writes only that lane's words and that lane's envelope, which is
/// everything `Channel::clear_state` resets for it. A padded lane is recovered the same way, which
/// keeps it at rest; whether a failure is *reported* is the caller's choice.
///
/// `andnot` clears a rejected lane's bits outright, so a recovered word is exactly `+0.0`, as the
/// whole-block `fill(0.0)` wrote, and a kept lane's words are not touched by any arithmetic.
fn finish_lanes<L: Lane>(io: &mut [f32], channel: &mut Channel<L>) -> u32 {
    if check_block::<L>(io) {
        return 0;
    }
    let failed = nonfinite_lane_mask::<L>(io);
    let mut flags = [0.0_f32; MAX_WIDTH];
    for (lane, flag) in flags.iter_mut().enumerate().take(L::WIDTH) {
        if failed & (1 << lane) != 0 {
            *flag = 1.0;
        }
    }
    let rejected = L::load(&flags).gt(L::zero());
    for frame in io.chunks_exact_mut(L::WIDTH) {
        L::load(frame).andnot(rejected).store(frame);
    }
    channel.gain_reduction_db = channel.gain_reduction_db.andnot(rejected);
    failed
}

/// Every lane of an `L`-wide channel, as a [`Channel::rest_lanes`] mask.
const fn all_lanes<L: Lane>() -> u32 {
    (1 << L::WIDTH) - 1
}

/// Fixed scalar words each channel section carries in the current causal payload.
pub use crate::state::STATE_HEADER_WORDS;

const fn effect_id(value: &'static str) -> effect_contract::EffectId {
    match effect_contract::EffectId::new(value) {
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
    name: &'static str,
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
) -> ParameterDescriptor {
    ParameterDescriptor {
        id: parameter_id(id),
        name,
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
        lattice: effect_contract::default_parameter_lattice(
            unit,
            ParameterDomain::Continuous,
            mapping,
        ),
        prepared_reason: None,
    }
}

/// Frozen V1 parameter descriptors. Parameter positions and stable IDs are identical.
pub const COMPRESSOR_PARAMETERS: [ParameterDescriptor; 7] = [
    parameter(
        1,
        "threshold_db",
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
        "knee_db",
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
        "attack_ms",
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
        "release_ms",
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
        "makeup_db",
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
];

const PORTS: [PortDescriptor; 3] = [
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
    PortDescriptor {
        id: port_id("sidechain-in"),
        role: PortRole::SidechainInput,
        required: false,
        layout: PortLayout::DualMonoPlanar,
    },
];

const fn quality(sample_rate: u32) -> effect_contract::QualityDescriptor {
    effect_contract::QualityDescriptor {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(0),
        maximum_state: StatePayloadSizes {
            common_bytes: 0,
            left_bytes: (state::STATE_HEADER_WORDS * 4) as u32,
            right_bytes: (state::STATE_HEADER_WORDS * 4) as u32,
        },
        scratch_fixed_bytes: 64,
        scratch_bytes_per_frame: 0,
    }
}

/// The fast tier's gain error, in dB: `fast_gain_from_db` is within this of `10^(g/20)` for every
/// `g` in `[-160, 24]` dB, the bound `crates/math/tests/f1_fast_db_bounds.rs` proves by an
/// exhaustive sweep (`GAIN_MAX_DB`). The applied gain's argument `smoothed + makeup` lies in
/// `[-124, 24]` dB.
const FAST_GAIN_ERROR_DB: f64 = 1.0e-5;

/// The largest distance of the gain-reduction word from its rest value `+0.0`, in dB:
/// `|GAIN_REDUCTION_MIN_DB|` plus one ulp.
///
/// The curve target is clamped to `[-100, 0]` dB and the word starts at `+0.0`. A release step
/// (`|t| <= |g|`) adds `fl(c fl(t - g))`, which has the sign of `t - g` and a magnitude at most
/// `|g|`, so `|g|` does not grow. An attack step (`|t| > |g|`) gives `|g + p| <= |t| + u |t - g|
/// <= 100 (1 + u)`, `u = 2^-24`, which rounds to at most `100` plus one ulp.
fn reduction_distance_max() -> f32 {
    f32::from_bits((-kernel::GAIN_REDUCTION_MIN_DB).to_bits() + 1)
}

/// The slowest release rate coefficient over the parameter domain at `sample_rate`: the design of
/// the longest release, one `f32` below it so that a non-monotone last bit of the `f64`
/// exponential cannot put a shorter release's coefficient under it. A coefficient ramp in flight
/// interpolates between two designed coefficients, so it never goes below either.
fn slowest_release_rate(sample_rate: u32) -> f32 {
    let longest = COMPRESSOR_PARAMETERS[4].maximum.unwrap_or(0.0);
    let c = design::rate_coefficient(longest, sample_rate);
    f32::from_bits(c.to_bits() - 1)
}

/// The exact-rest bound `R`, in samples, at `sample_rate` (#1375 D2), or `None` when it does not
/// exist (D3).
///
/// **Quantifier.** `R` holds when every input port is exactly zero from the first silent sample
/// `M` on: the main input **and** the sidechain (a connected sidechain without a buffer, or an
/// unconnected one, reads as zero). A sidechain that keeps playing keeps the detector moving, so it
/// states no rest; the output is still exactly zero (the tail), and the five composition values
/// hold for every sidechain input.
///
/// With every input zero the detector reads `+0.0`, the level clamps to -160 dB, under every legal knee start (`-80 - 12`
/// dB), so the curve target is `+0.0` from the first silent sample. The one signal-state word, the
/// gain-reduction word `g`, then steps `g' = snap_reduction(fma(c, 0 - g, g))` at the release rate
/// `c >= slowest_release_rate` (the attack arm needs `0 < g`, and `g <= 0` always), which is the
/// rate form [`effect_runtime::tail::rate_follower_rest_samples`] bounds. Its D3 condition holds:
/// the slowest coefficient, `1 / 480,000` at 96 kHz, is far above `2^-24`. The rest state `Z` is
/// the reset state (`g = +0.0`); parameter and ramp words are not signal state, and the latency is
/// zero, so `R` is the follower bound itself. It holds for any input peak: the word's distance is
/// bounded by the curve's clamp, not by the input.
fn rest_samples(sample_rate: u32) -> Option<u64> {
    effect_runtime::tail::rate_follower_rest_samples(
        slowest_release_rate(sample_rate),
        reduction_distance_max(),
        effect_runtime::tail::RestSnap::AtOrBelow(kernel::REDUCTION_REST_DB),
    )
}

/// `(1 + u)^count`, the relative growth of `count` rounded operations.
fn rounding_factor(count: u32) -> f64 {
    (0..count).fold(1.0, |factor, _| {
        factor * (1.0 + effect_runtime::tail::ROUNDOFF)
    })
}

/// `G_p = G_t` in millibels (#1375 D4): the output law is `y = fma(mix, x g - x, x)`, or `x g` on
/// the all-wet arm, or `x` on the identity arms, with `0 < g = fast_gain_from_db(smoothed +
/// makeup)`, `smoothed <= 0` and `mix` in `[0, 1]`. Through the four rounded operations,
/// `|y| <= max(1, g) |x| (1 + u)^4`, and `g <= 10^((max(makeup_max, 0) + FAST_GAIN_ERROR_DB) /
/// 20)`. A sidechain feeds only the detector, so the bound holds for every sidechain input; after
/// silence the output is the same law, so `G_t = G_p`.
fn peak_gain_millibels() -> i32 {
    let makeup_max = f64::from(COMPRESSOR_PARAMETERS[5].maximum.unwrap_or(0.0)).max(0.0);
    let gain = math::db_to_gain(makeup_max + FAST_GAIN_ERROR_DB);
    let rounding = rounding_factor(4);
    math::tail::ceil_millibels(gain * rounding, math::tail::Rounding::Computed)
}

/// `sigma_p = sigma_t` in millibels (#1375 D4, #1379 H3): the underflow allowance of the output's
/// chain, one [`math::tail::UNDERFLOW`] per rounded operation (four), each carried by at
/// most `(1 + u)^3` through the operations after it (the mix is at most `1`).
fn flush_stall_millibels() -> i32 {
    let rounding = rounding_factor(3);
    math::tail::ceil_millibels(
        4.0 * math::tail::UNDERFLOW * rounding,
        math::tail::Rounding::Computed,
    )
}

/// This effect's tail, tail over every peak, exact-rest bound and composition values, the one place
/// they are stated (decision 15 D15-4(b), #1377 D1, #1375).
///
/// * **Tail `Finite(0)`, decay `0`** (#1375 D1): every output arm is `x` times a finite gain, or
///   the mix of `x` and `x g`, of the *current* main input `x`: the compressor has no audio
///   history and zero latency. A sidechain feeds only the detector. So the output is exactly
///   `±0.0` from the first zero input sample, whatever the detector holds.
/// * **Rest** [`rest_samples`], for both peak cases; **tail over every peak** the same `R`, an
///   upper bound of `max(0, R(P*))`.
/// * **Composition** [`peak_gain_millibels`] and [`flush_stall_millibels`].
///
/// A rate the follower bound cannot cover states no bound.
fn tail_and_rest(sample_rate: u32, _quality: EffectQuality) -> effect_contract::NodeTailBound {
    let Some(rest) = rest_samples(sample_rate) else {
        return effect_contract::NodeTailBound::UNBOUNDED;
    };
    let gain = effect_contract::PeakGain::Millibels(peak_gain_millibels());
    let stall = effect_contract::FlushStall::Level(flush_stall_millibels());
    effect_contract::NodeTailBound {
        tail: TailSamples::Finite(0),
        tail_every_peak: TailSamples::Finite(rest),
        rest: effect_contract::RestBound::Bounded(effect_contract::RestSamples {
            peak_plus_24_dbfs: rest,
            any_sanitized_input: rest,
        }),
        composition: effect_contract::CompositionBound::Stated {
            decay: effect_contract::TailDecay(0),
            peak_gain: gain,
            tail_gain: gain,
            peak_stall: stall,
            tail_stall: stall,
        },
    }
}

const QUALITIES: [effect_contract::QualityDescriptor; 4] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
];

/// The one declared observation tap: the smoothed reduction the kernel already holds.
///
/// `Resident`, because `Channel::gain_reduction_db` is written by `process_block` every sample and
/// read back on the next one -- publishing it is a copy out of state the block wrote anyway, and
/// no lane kernel is touched. `PeakMagnitude` over a window is what makes the published number a
/// non-negative magnitude even though the smoother's own convention is negative-for-reduction.
pub const COMPRESSOR_OBSERVATIONS: [ObservationDescriptor; 1] = [ObservationDescriptor {
    id: ObservationTapId(1),
    display_name: "Gain Reduction",
    display_unit: "dB",
    kind: ObservationKind::GainReductionDb,
    unit: ParameterUnit::Db,
    cost: ObservationCost::Resident,
    cadence: ObservationCadence::PerBlock,
    fold: ObservationFold::PeakMagnitude,
    channels: ObservationChannels::PerLane,
    minimum: 0.0,
    maximum: 100.0,
    bypass_observation: Some(0.0),
}];

/// Immutable launch compressor descriptor.
pub const COMPRESSOR_DESCRIPTOR: EffectDescriptor = EffectDescriptor {
    id: effect_id("miso.compressor"),
    display_name: "Compressor",
    contract_major: 1,
    // Issue #143 P1: the observation menu is a declared, additive descriptor field, so declaring
    // the first tap is a `contract_minor` bump and a derived identity re-pin of exactly
    // `32 + len("Gain Reduction") + len("dB")` = 48 bytes. `state_layout_version` does not move:
    // no state byte changed, the tap is a read of state that was already there.
    contract_minor: 1,
    // #1278 D2a: the 37-word layout carries every ramp's step and the coefficient ramps, so a
    // mid-ramp restore continues bit for bit. It stays V1, the sole prelaunch identity
    // (`state::STATE_LAYOUT_VERSION`); the payload is not persisted (R6b).
    state_layout_version: state::STATE_LAYOUT_VERSION,
    supported_link_modes: LinkModeSet::ALL,
    lane_link: true,
    parameters: &COMPRESSOR_PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
    tail_and_rest,
    observations: &COMPRESSOR_OBSERVATIONS,
};

/// Factory entry point for the V1 compressor implementation.
#[derive(Clone, Copy, Debug, Default)]
pub struct CompressorFactory;

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
/// automation capacity, a span at exactly `first_sample`, inside the parameter's domain, addressed
/// to a single channel, in strictly increasing `(parameter, channel)` order, and not a duplicate.
/// Anything else is counted in `invalid_spans` and ignored — never applied partially.
fn apply_automation<L: Lane>(
    spans: &[PreparedAutomationSpan],
    first_sample: u64,
    lane: usize,
    instance: &mut Instance<L>,
    report: &mut ProcessReport,
) {
    let automation_capacity = instance.automation_capacity;
    let sample_rate = instance.sample_rate;
    let (left, right) = (&mut instance.left, &mut instance.right);
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
        let valid = span_index < automation_capacity as usize
            && parameter_index < RAMP_COUNT
            && span.sample == first_sample
            && parameter_value_valid(&PARAMETER_SPECS[parameter_index], span.value)
            && last_order.is_none_or(|previous| order > previous)
            && pending[channel_index][parameter_index].is_none();
        if !valid {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        }
        last_order = Some(order);
        pending[channel_index][parameter_index] = Some(normalize_zero(span.value));
    }
    for (parameter_index, (left_value, right_value)) in
        pending[0].iter().zip(pending[1].iter()).enumerate()
    {
        if let Some(value) = *left_value {
            left.set_parameter_target(parameter_index, lane, value, sample_rate);
        }
        if let Some(value) = *right_value {
            right.set_parameter_target(parameter_index, lane, value, sample_rate);
        }
    }
}

/// Which detector source a prepared configuration and a block imply.
///
/// `sidechain_connected` is whether the prepared sidechain port is
/// [`effect_contract::PreparedSidechainPort::Connected`].
fn detector_source<'a>(
    sidechain_connected: bool,
    sidechain: Option<(&'a [f32], &'a [f32])>,
) -> Detector<'a> {
    if !sidechain_connected {
        return Detector::Main;
    }
    match sidechain {
        Some((left, right)) => Detector::Sidechain(left, right),
        None => Detector::Silent,
    }
}

/// The track index a bank call names, or the track diagnostic.
///
/// A lane past the width and a padded lane (a clear bit of `active`, issue #1090) are both not a
/// track.
fn checked_track(track_index: u32, width: usize, active: u32) -> Result<usize, StatePayloadError> {
    let track = usize::try_from(track_index).map_err(|_| StatePayloadError {
        code: "effect.state.track",
    })?;
    if track >= width || active & (1 << track) == 0 {
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
///
/// It keeps only the prepared values its own methods read (issue #1461): the whole
/// `PreparedEffectMetadata` stays on the control side, in the prepare result.
struct Instance<L: Lane> {
    /// The prepared sample rate: ramp lengths, reset and restore.
    sample_rate: u32,
    /// The prepared bypass flag, which selects the kernel's dry path.
    bypass: bool,
    /// Every lane's prepared detector link mode (issue #1368 D3). A lane's mode is one of its
    /// prepared defaults: a render-time lane swap moves it with the lane.
    link_modes: [LinkMode; MAX_WIDTH],
    /// The link law `link_modes` gives, which the kernel reads; rebuilt whenever a mode moves.
    links: kernel::Links<L>,
    /// The prepared automation capacity: a span at or past it is invalid.
    automation_capacity: u32,
    /// The prepared state payload sizes a snapshot or restore must match.
    state_sizes: StatePayloadSizes,
    left: Channel<L>,
    right: Channel<L>,
    /// Issue #163 phase 4 item 1: the instance is at a silent fixed point. Earned at the end of a
    /// quiet block in [`render`](Self::render) when every lane is at exact rest
    /// ([`lanes_at_rest`](Self::lanes_at_rest), issue #1745), never assumed, and withdrawn by
    /// every target, automation, reset and restore. See the matching field on `parametric-eq`
    /// for the induction it licenses.
    silent_fixed_point: bool,
    /// Whether a collapsed block ([`render_mono`](Self::render_mono)) has rendered since the last
    /// disengage copy ([`desymmetrize`](Self::desymmetrize)), which alone clears it (#1894). While
    /// it is set the right channel is frozen and stale, so [`lanes_at_rest`](Self::lanes_at_rest)
    /// reads the left channel alone: the effect contract's collapsed-bank rule. The compressor has
    /// no clocks, so nothing else reads it.
    collapsed: bool,
    /// The bypass flag in force when the claim above was earned. Bypass selects a different path
    /// through the kernel, so a claim earned on one side of it says nothing about the other.
    silent_bypass: bool,
}

impl<L: Lane> Instance<L> {
    /// Allocates both channels from per-lane preparation values. `links[lane]` is lane `lane`'s
    /// detector link mode; only `[..L::WIDTH]` is read.
    fn new(
        metadata: &PreparedEffectMetadata,
        links: &[LinkMode; MAX_WIDTH],
        left_defaults: &[[f32; PARAMETER_COUNT]; MAX_WIDTH],
        right_defaults: &[[f32; PARAMETER_COUNT]; MAX_WIDTH],
    ) -> Self {
        let mut instance = Self {
            sample_rate: metadata.sample_rate,
            bypass: metadata.bypass,
            link_modes: *links,
            links: kernel::Links::per_lane(links),
            automation_capacity: metadata.automation_capacity,
            state_sizes: metadata.state_sizes,
            left: Channel::new(left_defaults, metadata.sample_rate),
            right: Channel::new(right_defaults, metadata.sample_rate),
            silent_fixed_point: false,
            collapsed: false,
            silent_bypass: metadata.bypass,
        };
        instance.rest_links();
        instance
    }

    /// Seats every lane's link weights at rest at its mode, on both channels (issue #1370 D2).
    fn rest_links(&mut self) {
        for lane in 0..L::WIDTH {
            let mode = self.link_modes[lane];
            self.left.rest_link(lane, mode);
            self.right.rest_link(lane, mode);
        }
    }

    // REALTIME_POLICY_BEGIN: #1370, a link glide is retargeted at a block boundary.
    /// Glides lane `lane`'s detector link to `mode` over `samples` samples (issue #1370 D1-D3).
    ///
    /// The lane's mode becomes `mode` at once, and with it the masks the prepared law reads, so a
    /// glide that arrives lands on `mode`'s own law; the weights carry the transition. The mode is
    /// the one a full reset keeps and a lane swap moves. `false`, changing nothing, for a lane
    /// past the width, a mode the descriptor does not support or a glide longer than
    /// `LINK_GLIDE_MAX_SAMPLES`; the caller checks padding.
    fn retarget_link(&mut self, lane: usize, mode: LinkMode, samples: u32) -> bool {
        if lane >= L::WIDTH
            || !COMPRESSOR_DESCRIPTOR.supported_link_modes.contains(mode)
            || samples > effect_contract::LINK_GLIDE_MAX_SAMPLES
        {
            return false;
        }
        self.link_modes[lane] = mode;
        self.links = kernel::Links::per_lane(&self.link_modes);
        self.left.retarget_link(lane, mode, samples);
        self.right.retarget_link(lane, mode, samples);
        // A glide moves the detector, so a silent claim earned under the old law goes.
        self.silent_fixed_point = false;
        true
    }
    // REALTIME_POLICY_END

    fn reset(&mut self, kind: ResetKind) {
        // A reset changes the recursive word and ramps, so any silent fixed-point claim goes.
        self.silent_fixed_point = false;
        let rate = self.sample_rate;
        match kind {
            ResetKind::FullToDefaults => {
                self.left.full_reset(rate);
                self.right.full_reset(rate);
                self.rest_links();
            }
            ResetKind::DiscontinuityKeepParameters => {
                self.left.discontinuity_reset(rate);
                self.right.discontinuity_reset(rate);
            }
        }
    }

    // REALTIME_POLICY_BEGIN
    /// [`reset`](Self::reset) for lane `lane` alone (issue #2131): both channels of the lane take
    /// the words the whole reset writes there (`kernel::Channel::full_reset_lane`,
    /// `kernel::Channel::discontinuity_reset_lane`), and a full reset seats the lane's link weights
    /// at rest at its mode. Every other lane keeps every word. The compressor has no shared cursor
    /// or counter; the silent claim is withdrawn, as `reset` withdraws it. A lane past the width
    /// changes nothing.
    fn reset_lane(&mut self, lane: usize, kind: ResetKind) {
        if lane >= L::WIDTH {
            return;
        }
        self.silent_fixed_point = false;
        let rate = self.sample_rate;
        match kind {
            ResetKind::FullToDefaults => {
                self.left.full_reset_lane(lane, rate);
                self.right.full_reset_lane(lane, rate);
                let mode = self.link_modes[lane];
                self.left.rest_link(lane, mode);
                self.right.rest_link(lane, mode);
            }
            ResetKind::DiscontinuityKeepParameters => {
                self.left.discontinuity_reset_lane(lane, rate);
                self.right.discontinuity_reset_lane(lane, rate);
            }
        }
    }
    // REALTIME_POLICY_END

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
        // * both input planes are exactly `+0.0`, which is the only input for which a settled
        //   causal compressor can skip the kernel while preserving output and state bits.
        let quiet = self.left.max_remaining() == 0
            && self.right.max_remaining() == 0
            && matches!(detector, Detector::Main | Detector::Silent)
            && self.silent_bypass == self.bypass
            && block_is_positive_zero(&left[..words])
            && block_is_positive_zero(&right[..words]);
        if quiet && self.silent_fixed_point {
            // Every lane is at its exact rest and the buffers already hold the `+0.0` the
            // current-sample kernel would write, so the whole block is bit-identical to the slow
            // path. There is no history cursor to advance.
            #[cfg(test)]
            admitted_silent();
            return;
        }
        kernel::process_block::<L>(
            left,
            right,
            detector,
            frames,
            &self.links,
            self.bypass,
            self.sample_rate,
            (&mut self.left, &mut self.right),
        );
        self.silent_bypass = self.bypass;
        let left_mask = finish_lanes::<L>(left, &mut self.left);
        let right_mask = finish_lanes::<L>(right, &mut self.right);
        // Earn or lose the claim from the state the block left (issue #1745): every lane of both
        // channels at its exact rest.
        self.silent_fixed_point = quiet && self.lanes_at_rest() == all_lanes::<L>();
        if left_mask | right_mask == 0 {
            return;
        }
        for lane in 0..L::WIDTH {
            let bit = 1 << lane;
            record(lane, left_mask & bit != 0, right_mask & bit != 0);
        }
    }

    /// Renders one block of the **collapsed** track: the left plane only.
    ///
    /// Every leg reads the left channel, which is the dual body's predicate with the right
    /// channel's conjunct deleted. A collapsed bank's two channels hold the same ramps and
    /// recursive word by the induction the witness states, so the right-channel checks are the
    /// left checks restated. Reading them off a right plane the chain did not gather is what would
    /// be wrong.
    ///
    /// `record` is called with the **same** verdict for both channels, because the right plane the
    /// seam is about to write is this left plane.
    fn render_mono(
        &mut self,
        left: &mut [f32],
        detector: Detector<'_>,
        frames: usize,
        mut record: impl FnMut(usize, bool, bool),
    ) {
        // From here until the disengage copy the right channel is frozen and stale.
        self.collapsed = true;
        let words = frames * L::WIDTH;
        let quiet = self.left.max_remaining() == 0
            && matches!(detector, Detector::Main | Detector::Silent)
            && self.silent_bypass == self.bypass
            && block_is_positive_zero(&left[..words]);
        if quiet && self.silent_fixed_point {
            #[cfg(test)]
            admitted_silent();
            return;
        }
        kernel::process_block_mono::<L>(
            left,
            detector,
            frames,
            &self.links,
            self.bypass,
            self.sample_rate,
            &mut self.left,
        );
        self.silent_bypass = self.bypass;
        let left_mask = finish_lanes::<L>(left, &mut self.left);
        // The collapsed claim reads the left channel only: the right one is frozen at the
        // collapse, and the counterfactual right channel is the left one.
        self.silent_fixed_point = quiet && self.left.rest_lanes() == all_lanes::<L>();
        if left_mask == 0 {
            return;
        }
        for lane in 0..L::WIDTH {
            let bit = 1 << lane;
            record(lane, left_mask & bit != 0, left_mask & bit != 0);
        }
    }

    /// The lanes at exact rest on both channels (`PreparedNativeEffectBank::lanes_at_rest`), or
    /// on the left channel alone while [`collapsed`](Self::collapsed) (#1894): see
    /// `kernel::Channel::rest_lanes` for the words pinned and why the list is complete.
    fn lanes_at_rest(&self) -> u32 {
        if self.collapsed {
            self.left.rest_lanes()
        } else {
            self.left.rest_lanes() & self.right.rest_lanes()
        }
    }

    /// Copies the left channel's whole state onto the right (the collapse's disengage boundary).
    ///
    /// See `kernel::Channel::copy_state_from` for the word-by-word list and why each is on it.
    fn desymmetrize(&mut self) {
        self.right.copy_state_from(&self.left);
        self.collapsed = false;
    }

    // REALTIME_POLICY_BEGIN: #1278 D3, the payload calls run in the plan-swap block.
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
            self.state_sizes,
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
        if state_layout_version != COMPRESSOR_DESCRIPTOR.state_layout_version {
            return Err(StatePayloadError {
                code: "effect.state.version",
            });
        }
        state::validate_lengths(
            input.common.len(),
            input.left.len(),
            input.right.len(),
            self.state_sizes,
        )?;
        let link_left = state::validate_channel(input.left)?;
        let link_right = state::validate_channel(input.right)?;
        if link_left != link_right {
            return Err(StatePayloadError {
                code: "effect.state.link",
            });
        }
        // Issue #1370 D6: the weights land only under the mode they glide to. A payload taken
        // under another mode (the plan-swap carry into a successor prepared with a new mode)
        // leaves the lane's own mode at rest, as before the glide existed.
        let carried = link_left == self.link_modes[lane];
        let rate = self.sample_rate;
        state::commit_channel(input.left, &mut self.left, lane, rate, carried);
        state::commit_channel(input.right, &mut self.right, lane, rate, carried);
        if !carried {
            let mode = self.link_modes[lane];
            self.left.rest_link(lane, mode);
            self.right.rest_link(lane, mode);
        }
        // A restore writes the recursive word and coefficients from a payload this instance never
        // rendered, so any standing claim is void. Withdrawn only here, after the commit: a
        // refused restore returns above without writing a word, so the claim it leaves is as
        // true as it was (a write-free refusal, #1747).
        self.silent_fixed_point = false;
        Ok(())
    }

    /// Exchanges lane `lane`'s prepared defaults with `bytes` in place (issue #1747): the left
    /// channel's seven preparation values, then the right's, each `f32` little-endian, then the
    /// lane's link mode as one word (issue #1368, `effect_contract::exchange_lane_link_word`). The values are
    /// what `full_reset` re-seeds a lane from; the mode is the lane's link law, which is rebuilt
    /// from it. No state word changes. `false`, changing nothing, for a lane out of range or a
    /// wrong length.
    fn exchange_lane_defaults(&mut self, lane: usize, bytes: &mut [u8]) -> bool {
        if lane >= L::WIDTH || bytes.len() != LANE_DEFAULT_BYTES {
            return false;
        }
        let (values_bytes, link_bytes) = bytes.split_at_mut(2 * PARAMETER_COUNT * 4);
        self.link_modes[lane] =
            effect_contract::exchange_lane_link_word(self.link_modes[lane], link_bytes);
        self.links = kernel::Links::per_lane(&self.link_modes);
        // The weights rest at the incoming mode; the swap's payload restore then brings a glide
        // in flight with it (issue #1370 D6).
        let mode = self.link_modes[lane];
        self.left.rest_link(lane, mode);
        self.right.rest_link(lane, mode);
        let (left_bytes, right_bytes) = values_bytes.split_at_mut(PARAMETER_COUNT * 4);
        for (values, side) in [
            (&mut self.left.defaults[lane], left_bytes),
            (&mut self.right.defaults[lane], right_bytes),
        ] {
            for (value, word) in values
                .iter_mut()
                .zip(side.as_chunks_mut::<4>().0.iter_mut())
            {
                let incoming =
                    f32::from_bits(u32::from_le_bytes([word[0], word[1], word[2], word[3]]));
                word.copy_from_slice(&value.to_bits().to_le_bytes());
                *value = incoming;
            }
        }
        true
    }
    // REALTIME_POLICY_END
}

/// The bytes of one lane's prepared defaults: both channels' preparation values (issue #1747) and
/// the lane's link mode (issue #1368).
const LANE_DEFAULT_BYTES: usize = 2 * PARAMETER_COUNT * 4 + 4;

/// The `DESIGNED` term of the channel-symmetry witness, over the compressor's own kernel read
/// surface.
///
/// # The word list, and why it is exactly this
///
/// Every designed per-lane word the compressor's kernel loads is a `Channel` field
/// (`compressor::kernel::Channel`), and the render path reads the coefficient words and ramps
/// listed below:
///
/// * `words[c][l]` -- the eight designed coefficients (`COEF_COUNT`), the documented "source of
///   truth; `Coef` is a load of these". `Coef::load` reads them and derives `wet_identity`,
///   `dry_mix_zero` and `makeup_zero` as pure functions of them, so those three masks carry no
///   information this comparison does not already have.
/// * `ramps[p][l]` -- one `LinearRamp` per smoothed parameter (`RAMP_COUNT`), all four fields:
///   `max_remaining` reads `remaining` to size the ramping prefix, and `advance_ramps` reads
///   `current`, `target` and `step` to move it. A ramp mid-flight is symmetric exactly when both
///   channels are the same distance from the same target by the same step.
///
/// Deliberately excluded, each for its own reason:
///
/// * `gain_reduction_db` -- running state, not designed words.
/// * `defaults[l][p]` -- the control-plane reset values. They are not read by the kernel; a
///   `FullToDefaults` reset that made the channels disagree would show up in `words` and `ramps`
///   immediately, which is where the witness sees it.
/// * `links`, `bypass`, `sample_rate`, `silent_fixed_point`,
///   `silent_bypass` -- whole-instance or per-channel-shared (a lane's link law is the same on
///   both channels), so they cannot be asymmetric. The link is the reason the seam sits where it does, not a thing that breaks it:
///   on identical planes the collapsed kernel computes the link on the one plane read twice, in
///   the original operation order.
impl<L: Lane> Instance<L> {
    fn designed_channel_symmetry(&self, lane: usize) -> bool {
        if lane >= L::WIDTH {
            return false;
        }
        let (left, right) = (&self.left, &self.right);
        for coefficient in 0..COEF_COUNT {
            if left.words[coefficient][lane].to_bits() != right.words[coefficient][lane].to_bits() {
                return false;
            }
        }
        for parameter in 0..RAMP_COUNT {
            let (l, r) = (&left.ramps[parameter][lane], &right.ramps[parameter][lane]);
            if l.current.to_bits() != r.current.to_bits()
                || l.target.to_bits() != r.target.to_bits()
                || l.step.to_bits() != r.step.to_bits()
                || l.remaining != r.remaining
            {
                return false;
            }
        }
        for (l, r) in left
            .rate_ramps
            .iter()
            .chain(&left.link_ramps)
            .zip(right.rate_ramps.iter().chain(&right.link_ramps))
        {
            let (l, r) = (&l[lane], &r[lane]);
            if l.current.to_bits() != r.current.to_bits()
                || l.target.to_bits() != r.target.to_bits()
                || l.step.to_bits() != r.step.to_bits()
                || l.remaining != r.remaining
            {
                return false;
            }
        }
        true
    }
}

/// A prepared, allocation-free scalar compressor instance: the `L = f32` instantiation.
pub struct PreparedCompressor {
    instance: Instance<f32>,
    /// Whether the prepared sidechain port is connected, which picks the detector source.
    sidechain_connected: bool,
}

impl PreparedCompressor {
    /// The scalar instance a prepared metadata and its per-channel defaults describe.
    fn new(
        metadata: &PreparedEffectMetadata,
        left_defaults: [f32; PARAMETER_COUNT],
        right_defaults: [f32; PARAMETER_COUNT],
    ) -> Self {
        Self {
            instance: Instance::new(
                metadata,
                &[metadata.link_mode; MAX_WIDTH],
                &[left_defaults; MAX_WIDTH],
                &[right_defaults; MAX_WIDTH],
            ),
            sidechain_connected: matches!(
                metadata.ports.sidechain,
                effect_contract::PreparedSidechainPort::Connected { .. }
            ),
        }
    }
}

/// A prepared homogeneous bank: `L::WIDTH` lanes as one vector, same kernel body.
///
/// # Padded lanes (issue #1090)
///
/// A bank may carry fewer members than lanes (the padding contract on
/// [`PrepareEffectBankRequest`]). A padded lane is prepared from its request, which the caller
/// cloned from an active member's, and runs the same kernel as every other lane: the compressor
/// has no whole-bank decision that could move an active lane's bits, so nothing in the kernel
/// knows which lanes are padded. The bank's own bookkeeping does:
///
/// * **D7.** Recovery is per lane ([`finish_lanes`]) for every lane, and only an active lane's
///   failure is reported. A padded lane is fed `+0.0` and its output is discarded, so its failure
///   is nobody's; recovering it anyway keeps it at rest.
/// * **Automation.** A span addressed to a padded lane is neither applied nor counted, so its
///   ramps stay where preparation put them and its report stays empty.
/// * **State payloads.** A padded lane is not a track: a snapshot or restore that names one is
///   `effect.state.track`, as a lane past the width is. A restore could otherwise move it off rest.
///
/// On `+0.0` input a padded lane stays at rest and writes `+0.0`, whatever member it cloned: every
/// legal threshold and knee put the silent detector level, `-160` dB, below the knee, so the curve
/// target is `+0.0`, the envelope stays `+0.0`, and every output word is `+0.0`: the dry word,
/// `+0.0 * gain`, or their mix, `mix * (+0.0 - +0.0) + +0.0`. That is the contract's `+0.0`-out
/// clause, and it is why a padded lane never keeps an otherwise silent bank off the silent fast
/// path.
///
/// # What stays whole-bank, and why no bit moves
///
/// Four decisions are still taken once for all lanes. Each couples cost, never bits:
///
/// * the **silent fast path** skips a block only when every lane is at its fixed point on `+0.0`
///   input, where the slow path would write the same `+0.0` and leave the same state;
/// * the **ramping prefix** is as long as the longest ramp of any lane, and a lane with no ramp in
///   flight renders the same bits through the ramping body as through the settled one;
/// * the **all-wet arm** is taken when every lane has `mix == 1`, and differs from the general law
///   only in a quieted signalling NaN, which fails that lane's own D7 check either way;
/// * the **link** combines the two channels of one lane, lane-wise, never two lanes. Each lane
///   takes its own member's mode (issue #1368); the all-DualMono `abs` arm is chosen once per
///   bank from those modes and has the general law's bits on a DualMono lane.
struct PreparedCompressorBank<L: Lane> {
    /// The bound width, which the block shape guard checks.
    width: BankWidth,
    /// The prepared render quantum, which bounds a block's frames.
    quantum: u32,
    instance: Instance<L>,
    /// Bit `l` is set when lane `l` carries a member; clear for a padded lane. All `L::WIDTH` bits
    /// are set for a full bank.
    active: u32,
}

/// A bank request whose shape and every member request have been validated: what a bank of either
/// width is built from.
struct BankParts {
    width: BankWidth,
    metadata: PreparedEffectMetadata,
    left_defaults: [[f32; PARAMETER_COUNT]; MAX_WIDTH],
    right_defaults: [[f32; PARAMETER_COUNT]; MAX_WIDTH],
    /// Each lane's detector link mode (issue #1368): a member's own, and for a padded lane the
    /// first member's (D5), so padding never takes a bank off the all-DualMono arm.
    links: [LinkMode; MAX_WIDTH],
    /// Every lane's request, padded ones included, prepares the same program key.
    same_program: bool,
    /// [`PreparedCompressorBank::active`].
    active: u32,
}

impl BankParts {
    /// Validates the shape and every lane's request, in lane order, before any fallback is taken:
    /// an unavailable backend, a connected sidechain or a heterogeneous cohort must never hide a
    /// malformed request (E13).
    ///
    /// A padded lane's request is validated like a member's. It is a clone of one, so it passes
    /// exactly when that member does; the preparation values it carries are the ones its lane runs.
    fn validate(request: PrepareEffectBankRequest<'_>) -> Result<Self, EffectPrepareError> {
        request.validate_shape()?;
        let first = request.requests[0];
        let metadata = expected_prepared_metadata(&COMPRESSOR_DESCRIPTOR, first)?;
        let (first_left, first_right) = initial_defaults(first.initial_values)?;
        let mut parts = Self {
            width: request.width,
            metadata,
            left_defaults: [first_left; MAX_WIDTH],
            right_defaults: [first_right; MAX_WIDTH],
            links: [metadata.link_mode; MAX_WIDTH],
            same_program: true,
            active: 0,
        };
        for (lane, item) in request.requests.iter().copied().enumerate() {
            let candidate = expected_prepared_metadata(&COMPRESSOR_DESCRIPTOR, item)?;
            if candidate.program_key() != metadata.program_key() {
                parts.same_program = false;
            }
            let (left, right) = initial_defaults(item.initial_values)?;
            parts.left_defaults[lane] = left;
            parts.right_defaults[lane] = right;
            if request.active_mask[lane] {
                parts.active |= 1 << lane;
                parts.links[lane] = candidate.link_mode;
            }
        }
        Ok(parts)
    }

    /// The bank at `L`, which must be the request's width.
    fn bank<L: Lane>(&self) -> PreparedCompressorBank<L> {
        debug_assert_eq!(L::WIDTH, self.width.lanes() as usize);
        PreparedCompressorBank {
            width: self.width,
            quantum: self.metadata.quantum,
            instance: Instance::new(
                &self.metadata,
                &self.links,
                &self.left_defaults,
                &self.right_defaults,
            ),
            active: self.active,
        }
    }

    /// The metadata the binding derived, which stays on the control side (issue #1461).
    fn bank_metadata(&self) -> PreparedBankMetadata {
        PreparedBankMetadata {
            width: self.width,
            program_key: self.metadata.program_key(),
        }
    }
}

/// The scalar instance [`CompressorFactory::prepare`] boxes, and its metadata.
fn prepare_scalar(
    request: PrepareEffectRequest<'_>,
) -> Result<(PreparedCompressor, PreparedEffectMetadata), EffectPrepareError> {
    let metadata = expected_prepared_metadata(&COMPRESSOR_DESCRIPTOR, request)?;
    let (left_defaults, right_defaults) = initial_defaults(request.initial_values)?;
    Ok((
        PreparedCompressor::new(&metadata, left_defaults, right_defaults),
        metadata,
    ))
}

impl NativeEffectFactory for CompressorFactory {
    fn descriptor(&self) -> &'static EffectDescriptor {
        &COMPRESSOR_DESCRIPTOR
    }

    fn prepared_inline_bytes(&self) -> u64 {
        size_of::<PreparedCompressor>() as u64
    }

    fn prepared_undeclared_heap_bytes(&self, _: &effect_contract::QualityDescriptor) -> u64 {
        0
    }

    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<PreparedEffect, EffectPrepareError> {
        let (processor, metadata) = prepare_scalar(request)?;
        Ok(PreparedEffect {
            processor: Box::new(processor),
            metadata,
        })
    }

    /// Issue #2199: the scalar instance `prepare` boxes, built inline and asked. It holds no heap
    /// (#1074), so the answer allocates nothing.
    fn designed_channel_symmetry(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<bool, EffectPrepareError> {
        prepare_scalar(request).map(|(processor, _)| processor.channel_symmetry())
    }

    /// Binds a full or a padded bank (issue #1090: the compressor accepts the padding contract on
    /// [`PrepareEffectBankRequest`]; the private bank type's documentation says how it keeps it).
    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<PreparedEffectBank>, EffectPrepareError> {
        let parts = BankParts::validate(request)?;
        if !parts.same_program {
            return Ok(None);
        }
        // A console slot never carries a sidechain (decision 12), and a keyed insert has no bank
        // kernel here.
        if !matches!(
            parts.metadata.ports.sidechain,
            effect_contract::PreparedSidechainPort::Unconnected {
                id,
                required: false,
            } if id == port_id("sidechain-in")
        ) {
            return Ok(None);
        }
        // D4, revision 4: there is no runtime SIMD dispatch. This build has exactly one lane
        // width, decided at compile time and attested at boot, so a bank of any other width is a
        // backend that is not available here — the `Ok(None)` scalar fallback, never an error.
        if Backend::current().width() != request.width.lanes() as usize {
            return Ok(None);
        }
        // The build's own width, as a constant, so only its bank is instantiated: a runtime match on
        // `request.width` would compile the other width's bank into every artifact. By width, not
        // by `Backend` variant, because `Backend::Scalar` is test-only (#1059).
        const NATIVE: Option<BankWidth> = BankWidth::for_backend(Backend::current());
        Ok(Some(match NATIVE {
            Some(width) => PreparedEffectBank {
                processor: effect_contract::match_bank_width!(width, |L| {
                    Box::new(parts.bank::<L>()) as Box<dyn PreparedNativeEffectBank>
                }),
                metadata: parts.bank_metadata(),
            },
            None => return Ok(None),
        }))
    }
}

impl PreparedNativeEffect for PreparedCompressor {
    /// Every field is inline: this instance owns no heap (#1074).
    fn retained_bytes(&self) -> u64 {
        0
    }

    fn channel_symmetry(&self) -> bool {
        self.instance.designed_channel_symmetry(0)
    }

    fn retarget_link(&mut self, track: usize, mode: LinkMode, samples: u32) -> bool {
        track == 0 && self.instance.retarget_link(0, mode, samples)
    }

    /// Applies a compressor target by zero-based descriptor index.
    ///
    /// Indices 0 through 6 are the threshold dB, ratio, knee dB, attack milliseconds, release
    /// milliseconds, makeup dB, and unitless wet mix. Left and Right are independent; Both is rejected. Validation
    /// precedence is index, channel, automatable, then finite in-domain value, and every rejection
    /// leaves the complete prepared state unchanged.
    ///
    /// The accepted target begins the compressor's 64-sample smoothing transition without
    /// processing a sample. A direct native caller must keep this call, any no-sample catch-up,
    /// and the following [`Self::process`] call within the canonical environment established by
    /// [`lane::CanonicalFpEnv`], as render entry does.
    fn apply_parameter_point(
        &mut self,
        parameter_index: u32,
        channel: ParameterChannel,
        value: f32,
    ) -> Result<(), ParameterAccessError> {
        let index = usize::try_from(parameter_index)
            .ok()
            .filter(|&index| index < PARAMETER_COUNT)
            .ok_or(ParameterAccessError::InvalidParameterIndex)?;
        let lane = match channel {
            ParameterChannel::Left => 0,
            ParameterChannel::Right => 1,
            ParameterChannel::Both => return Err(ParameterAccessError::InvalidChannel),
        };
        if !COMPRESSOR_PARAMETERS[index].automatable {
            return Err(ParameterAccessError::NotAutomatable);
        }
        if !parameter_value_valid(&PARAMETER_SPECS[index], value) {
            return Err(ParameterAccessError::InvalidValue);
        }
        let value = normalize_zero(value);
        self.instance.silent_fixed_point = false;
        let channel = if lane == 0 {
            &mut self.instance.left
        } else {
            &mut self.instance.right
        };
        channel.set_parameter_target(index, 0, value, self.instance.sample_rate);
        Ok(())
    }

    /// Reads the resident current and target values in the units listed above.
    ///
    /// The same index and channel validation applies. Reading never changes smoother or envelope
    /// state. Direct
    /// native scheduling shares the [`lane::CanonicalFpEnv`] precondition documented on
    /// [`Self::apply_parameter_point`].
    fn parameter_state(
        &self,
        parameter_index: u32,
        channel: ParameterChannel,
    ) -> Result<PreparedParameterState, ParameterAccessError> {
        let index = usize::try_from(parameter_index)
            .ok()
            .filter(|&index| index < PARAMETER_COUNT)
            .ok_or(ParameterAccessError::InvalidParameterIndex)?;
        let lane = match channel {
            ParameterChannel::Left => 0,
            ParameterChannel::Right => 1,
            ParameterChannel::Both => return Err(ParameterAccessError::InvalidChannel),
        };
        let channel = if lane == 0 {
            &self.instance.left
        } else {
            &self.instance.right
        };
        let ramp = channel.ramps[index][0];
        Ok(PreparedParameterState {
            current_value: ramp.current,
            target_value: ramp.target,
        })
    }

    // REALTIME_POLICY_BEGIN
    fn reset(&mut self, kind: ResetKind) {
        self.instance.reset(kind);
    }
    // REALTIME_POLICY_END

    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        // #163 phase 4 item 1, as in `process_bank`.
        if !block.automation.is_empty() {
            self.instance.silent_fixed_point = false;
        }
        let mut report = ProcessReport::default();
        apply_automation(
            block.automation,
            block.first_sample,
            0,
            &mut self.instance,
            &mut report,
        );
        let frames = block.left.len();
        self.instance.render(
            block.left,
            block.right,
            detector_source(self.sidechain_connected, block.sidechain),
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
    fn observe_resident(&self, tap_index: u32, out: &mut ObservationSample) -> bool {
        if tap_index != 0 {
            return false;
        }
        out.left = self.instance.left.gain_reduction_db;
        out.right = self.instance.right.gain_reduction_db;
        true
    }

    // REALTIME_POLICY_BEGIN: #1278 D3, the payload calls run in the plan-swap block.
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
    // REALTIME_POLICY_END
}

impl<L: Lane> PreparedNativeEffectBank for PreparedCompressorBank<L> {
    // REALTIME_POLICY_BEGIN
    /// Issue #2131: see `Instance::reset_lane`.
    fn reset_lane(&mut self, lane: usize, kind: ResetKind) {
        self.instance.reset_lane(lane, kind);
    }
    // REALTIME_POLICY_END
    /// Every field is inline: this instance owns no heap (#1074).
    fn retained_bytes(&self) -> u64 {
        0
    }

    fn lane_channel_symmetry(&self, lane: usize) -> bool {
        self.instance.designed_channel_symmetry(lane)
    }

    /// A padded lane carries no track, so it takes no glide.
    fn retarget_link(&mut self, track: usize, mode: LinkMode, samples: u32) -> bool {
        track < L::WIDTH
            && self.active & (1 << track) != 0
            && self.instance.retarget_link(track, mode, samples)
    }

    /// Issue #1745: `kernel::Channel::rest_lanes` on both channels, padded lanes included; on the
    /// left channel alone on a collapsed bank (#1894).
    fn lanes_at_rest(&self) -> u32 {
        self.instance.lanes_at_rest()
    }

    /// The compressor has no clocks (issue #1747; see `kernel::Channel::rest_lanes`), so a
    /// skipped block moves no state word or payload word. It overrides the default only for the
    /// bank flag `silent_fixed_point`: a rendered `+0.0` block after which every lane, padded
    /// lanes included, is at rest earns it (the bank always detects on the main input, and the
    /// prepared bypass is the one the claim records), so a mask that covers the whole bank sets
    /// it too, as the EQ's does. The contract's caller skips a bank whole; a partial mask leaves
    /// the flag to the render of the other lanes. No output bit and no payload word reads the
    /// flag: it only lets a later `+0.0` block take the fast path, which renders the same bits as
    /// the kernel would, so no test reads it either.
    fn advance_clocks(&mut self, lanes_mask: u32, frames: u32) {
        let bank = all_lanes::<L>();
        debug_assert_eq!(lanes_mask & !bank, 0, "a lane past the bank");
        debug_assert!(frames <= self.quantum, "a block longer than the quantum");
        debug_assert_eq!(
            lanes_mask & !self.instance.lanes_at_rest(),
            0,
            "a lane not at rest"
        );
        if lanes_mask & bank == bank {
            self.instance.silent_fixed_point = true;
        }
    }

    /// One `store` per channel fills every lane: the bank's reduction is one vector.
    fn observe_resident_bank(&self, tap_index: u32, out: &mut [ObservationSample]) -> bool {
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

    fn supports_mono_collapse(&self) -> bool {
        true
    }

    fn process_bank_mono(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        self.process_bank_inner::<true>(block)
    }

    fn desymmetrize_channels(&mut self) {
        self.instance.desymmetrize();
    }

    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        self.process_bank_inner::<false>(block)
    }

    // REALTIME_POLICY_BEGIN: #1278 D3, the payload calls run in the plan-swap block.
    fn snapshot_track_state_payload(
        &self,
        track_index: u32,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        let track = checked_track(track_index, L::WIDTH, self.active)?;
        self.instance.snapshot(output, track)
    }

    fn restore_track_state_payload(
        &mut self,
        track_index: u32,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        let track = checked_track(track_index, L::WIDTH, self.active)?;
        self.instance.restore(state_layout_version, input, track)
    }

    fn lane_default_bytes(&self) -> Option<usize> {
        Some(LANE_DEFAULT_BYTES)
    }

    fn exchange_lane_defaults(&mut self, lane: usize, bytes: &mut [u8]) -> bool {
        self.instance.exchange_lane_defaults(lane, bytes)
    }
    // REALTIME_POLICY_END
}

impl<L: Lane> PreparedCompressorBank<L> {
    /// The one bank body, dual or collapsed.
    ///
    /// `MONO` chooses which render body the block takes and nothing else: the shape guard, the
    /// automation drain and the report are the same statements in the same order, because a
    /// collapsed block's automation is the dual block's automation and its per-lane accounting is
    /// the dual block's accounting with the right column duplicated from the left.
    ///
    /// A const generic rather than an argument, so the two monomorphise and the dual instantiation
    /// is the code that shipped before the collapse existed. See the parametric EQ's copy of this
    /// method for the measurement that settled it.
    fn process_bank_inner<const MONO: bool>(
        &mut self,
        block: EffectBankProcessBlock<'_>,
    ) -> BankProcessReport {
        let mut report = BankProcessReport::empty(self.width);
        let lanes = L::WIDTH;
        let frames = block.frames as usize;
        // The pre-audit guard was four inline conditions that indexed the automation offsets
        // before checking them and accepted `frames == 0`. This is the strengthened form: it
        // rejects rather than indexes.
        //
        // #95 landed and `effect_contract::validate_automation_block` now exists,
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
        if block.width != self.width
            || lanes != self.width.lanes() as usize
            || frames == 0
            || block.frames > self.quantum
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
        let active = self.active;
        for track in 0..lanes {
            // A padded lane carries no track, so nothing is automated on it and nothing is
            // counted against it (issue #1090).
            if active & (1 << track) == 0 {
                continue;
            }
            let start = block.automation_offsets[track] as usize;
            let end = block.automation_offsets[track + 1] as usize;
            apply_automation(
                &block.automation[start..end],
                block.first_sample,
                track,
                &mut self.instance,
                &mut report.reports[track],
            );
        }
        {
            let report = &mut report;
            let record = move |lane: usize, left_failed: bool, right_failed: bool| {
                // D7 reports attribute active lanes only: a padded lane's failure, recovered like
                // any lane's, is charged to nobody (issue #1090).
                if active & (1 << lane) == 0 {
                    return;
                }
                if left_failed {
                    report.reports[lane].nonfinite_left_blocks =
                        report.reports[lane].nonfinite_left_blocks.saturating_add(1);
                }
                if right_failed {
                    report.reports[lane].nonfinite_right_blocks = report.reports[lane]
                        .nonfinite_right_blocks
                        .saturating_add(1);
                }
            };
            if MONO {
                self.instance
                    .render_mono(block.left, Detector::Main, frames, record);
            } else {
                self.instance
                    .render(block.left, block.right, Detector::Main, frames, record);
            }
        }
        report
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

#[cfg(test)]
mod width_state_tests {
    use super::{Channel, Detector, Lane, kernel, state};
    use crate::design::{MAX_WIDTH, PARAMETER_COUNT, PARAMETER_SPECS};
    use effect_contract::LinkMode;

    const FRAMES: usize = 128;
    const SAMPLE_RATE: u32 = 48_000;

    fn defaults() -> [[f32; PARAMETER_COUNT]; MAX_WIDTH] {
        let values = core::array::from_fn(|index| PARAMETER_SPECS[index].default);
        [values; MAX_WIDTH]
    }

    fn serialized<L: Lane>(channel: &Channel<L>, lane: usize) -> Vec<u8> {
        let mut bytes = vec![0_u8; state::STATE_HEADER_WORDS * 4];
        state::write_channel(&mut bytes, channel, lane);
        bytes
    }

    fn state_matches_scalar<L: Lane>() {
        let defaults = defaults();
        let mut wide_left = Channel::<L>::new(&defaults, SAMPLE_RATE);
        let mut wide_right = Channel::<L>::new(&defaults, SAMPLE_RATE);
        let mut input_left = vec![0.0_f32; FRAMES * L::WIDTH];
        let mut input_right = vec![0.0_f32; FRAMES * L::WIDTH];
        for frame in 0..FRAMES {
            for lane in 0..L::WIDTH {
                let slot = frame * L::WIDTH + lane;
                input_left[slot] = ((frame * 3 + lane * 5) % 23) as f32 / 23.0;
                input_right[slot] = -(((frame * 7 + lane * 2) % 19) as f32) / 19.0;
            }
        }
        let mut left = input_left.clone();
        let mut right = input_right.clone();
        kernel::process_block::<L>(
            &mut left,
            &mut right,
            Detector::Main,
            FRAMES,
            &kernel::Links::uniform(LinkMode::DualMono),
            false,
            SAMPLE_RATE,
            (&mut wide_left, &mut wide_right),
        );

        for lane in 0..L::WIDTH {
            let mut scalar_left = Channel::<f32>::new(&defaults, SAMPLE_RATE);
            let mut scalar_right = Channel::<f32>::new(&defaults, SAMPLE_RATE);
            let mut scalar_input_left = vec![0.0_f32; FRAMES];
            let mut scalar_input_right = vec![0.0_f32; FRAMES];
            for frame in 0..FRAMES {
                scalar_input_left[frame] = input_left[frame * L::WIDTH + lane];
                scalar_input_right[frame] = input_right[frame * L::WIDTH + lane];
            }
            kernel::process_block::<f32>(
                &mut scalar_input_left,
                &mut scalar_input_right,
                Detector::Main,
                FRAMES,
                &kernel::Links::uniform(LinkMode::DualMono),
                false,
                SAMPLE_RATE,
                (&mut scalar_left, &mut scalar_right),
            );
            assert_eq!(
                serialized(&wide_left, lane),
                serialized(&scalar_left, 0),
                "left serialized state differs at lane {lane} for width {}",
                L::WIDTH
            );
            assert_eq!(
                serialized(&wide_right, lane),
                serialized(&scalar_right, 0),
                "right serialized state differs at lane {lane} for width {}",
                L::WIDTH
            );
        }
    }

    #[test]
    fn serialized_state_is_width_invariant_at_w1_w4_w8() {
        lane::each_lane!(|L| state_matches_scalar::<L>());
    }
}

/// Issue #1051: the `DESIGNED` witness against its own definition, one kernel word at a time.
///
/// `tests/randomized.rs` restores lanes whose parameter ramps differ in exactly one field, which
/// is what a payload can carry. It cannot reach a lane whose *coefficient* ramps alone differ: a
/// restore derives them from the parameter ramps, and a retarget moves the coefficient word with
/// them. The witness is specified over every word the kernel loads regardless
/// ([`Instance::designed_channel_symmetry`]), and a wrong `true` is wrong audio, so this draws the
/// words directly: random in-flight state, symmetric on both channels, then exactly one word or
/// ramp field of one lane moved on the right channel. The witness must decline that lane and only
/// that lane.
#[cfg(test)]
mod witness_tests {
    use super::{COEF_COUNT, COMPRESSOR_DESCRIPTOR, Instance, Lane, MAX_WIDTH, RAMP_COUNT};
    use crate::design::{PARAMETER_COUNT, PARAMETER_SPECS};
    use dsp_reference::randomized::Draw;
    use effect_contract::{
        EffectQuality, LinkMode, PrepareEffectLimits, PrepareEffectRequest, PreparedPorts,
        PreparedSidechainPort, default_initial_values, expected_prepared_metadata,
    };
    use effect_runtime::ramp::LinearRamp;

    fn instance<L: Lane>() -> Instance<L> {
        let values: Vec<_> = default_initial_values(&COMPRESSOR_DESCRIPTOR).collect();
        let request = PrepareEffectRequest {
            sample_rate: 48_000,
            quantum: 128,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: PreparedPorts {
                sidechain: PreparedSidechainPort::Unconnected {
                    id: COMPRESSOR_DESCRIPTOR.ports[2].id,
                    required: false,
                },
            },
            initial_values: &values,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: 360,
                maximum_scratch_bytes: 64,
                maximum_automation_spans_per_block: 16,
            },
            tail_bound: conformance::tail_bound_of(
                Box::new(crate::CompressorFactory),
                48_000,
                EffectQuality::Normal,
            ),
        };
        let metadata =
            expected_prepared_metadata(&COMPRESSOR_DESCRIPTOR, request).expect("a legal request");
        let defaults: [[f32; PARAMETER_COUNT]; MAX_WIDTH] =
            [core::array::from_fn(|index| PARAMETER_SPECS[index].default); MAX_WIDTH];
        Instance::new(
            &metadata,
            &[metadata.link_mode; MAX_WIDTH],
            &defaults,
            &defaults,
        )
    }

    fn ramp(draw: &mut Draw) -> LinearRamp {
        LinearRamp {
            current: draw.noise(4.0),
            target: draw.noise(4.0),
            step: draw.noise(0.1),
            remaining: draw.below(65) as u32,
        }
    }

    /// Moves one field to a value with other bits.
    fn perturb(value: &mut f32, draw: &mut Draw) {
        *value = if draw.chance(1, 2) {
            value.next_up()
        } else {
            -*value - 1.0
        };
    }

    fn witness<L: Lane>(seed: u64) {
        let mut draw = Draw::new(seed);
        let mut instance = instance::<L>();
        for lane in 0..L::WIDTH {
            for coefficient in 0..COEF_COUNT {
                instance.left.words[coefficient][lane] = draw.noise(2.0);
            }
            for parameter in 0..RAMP_COUNT {
                instance.left.ramps[parameter][lane] = ramp(&mut draw);
            }
            for slot in 0..2 {
                instance.left.rate_ramps[slot][lane] = ramp(&mut draw);
            }
        }
        instance.right.words = instance.left.words;
        instance.right.ramps = instance.left.ramps;
        instance.right.rate_ramps = instance.left.rate_ramps;
        let moved = draw.below(L::WIDTH);
        for lane in 0..L::WIDTH {
            assert!(
                instance.designed_channel_symmetry(lane),
                "seed {seed} W{}: symmetric",
                L::WIDTH
            );
        }
        // One word or one ramp field of lane `moved`, on the right channel only.
        let right = &mut instance.right;
        let group = draw.below(3);
        let field = draw.below(4);
        let ramp = match group {
            0 => {
                perturb(&mut right.words[draw.below(COEF_COUNT)][moved], &mut draw);
                None
            }
            1 => Some(&mut right.ramps[draw.below(RAMP_COUNT)][moved]),
            _ => Some(&mut right.rate_ramps[draw.below(2)][moved]),
        };
        if let Some(ramp) = ramp {
            match field {
                0 => perturb(&mut ramp.current, &mut draw),
                1 => perturb(&mut ramp.target, &mut draw),
                2 => perturb(&mut ramp.step, &mut draw),
                _ => ramp.remaining += 1,
            }
        }
        for lane in 0..L::WIDTH {
            assert_eq!(
                instance.designed_channel_symmetry(lane),
                lane != moved,
                "seed {seed} W{}: group {group} field {field} moved on lane {moved}, asked of lane \
                 {lane}",
                L::WIDTH
            );
        }
        assert!(
            !instance.designed_channel_symmetry(L::WIDTH),
            "a lane past the width"
        );
    }

    #[test]
    fn the_witness_declines_exactly_the_lane_whose_one_word_moved() {
        dsp_reference::randomized::run_seeds(
            "the_witness_declines_exactly_the_lane_whose_one_word_moved",
            "cargo test -p compressor --lib -- --exact \
             witness_tests::the_witness_declines_exactly_the_lane_whose_one_word_moved",
            256,
            |seed| {
                lane::each_lane!(|L| witness::<L>(seed));
            },
        );
    }
}
