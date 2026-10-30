//! Fixed two-times cubic soft clipper, as one block kernel over the `Lane` foundation.
//!
//! The rendered graph is frozen by `.github/ISSUE_SPECS/BRIEFS/019`: per-lane drive, output and
//! mix; two-times oversampling through a 63-tap Blackman half-band used for both interpolation and
//! decimation; the cubic `c(u) = u - u^3/3` clamped to `±2/3`; a dry path delayed 31 samples;
//! latency 31 and a finite tail of 29. `tests/polyphase_identity.rs` compares the kernel with an
//! independent realization of the brief's ascending 63-tap graph. The implementation uses:
//!
//! * one generic [`kernel::soft_clip_block`] at widths 1, 4 and 8, with the prepared driver checked
//!   against scalar instances by the padded-bank and randomized differentials;
//! * the polyphase half-band form, which does the work of two 31-tap convolutions instead of four;
//! * one cursor per bank over a double-written power-of-two history, so every tap is a contiguous
//!   vector load at a constant offset — no per-lane cursor, no modulus, no gather;
//! * D7 instead of per-operation checking: `flush` on the two values that enter a history, and one
//!   boundary check per block per bank through `effect_runtime::bank`;
//! * D11 ramps, D6 decibel conversion and the shared state-payload codec, all from
//!   `effect-runtime` and `math`.
//!
//! # State layout version 1
//!
//! In-memory handoff carries each ramp's current, target, step and remaining samples, followed by
//! newest-first history ages. The cursor belongs to the cohort and is not serialized. A mismatched
//! layout version is rejected with `effect.state.version`.

use effect_contract::{
    AutomationRate, BankProcessReport, BankWidth, EffectBankProcessBlock, EffectDescriptor,
    EffectPrepareError, EffectProcessBlock, EffectQuality, InitialParameterValue, LatencySamples,
    LinkModeSet, NativeEffectFactory, ParameterChannel, ParameterChannelPolicy,
    ParameterDescriptor, ParameterDomain, ParameterId, ParameterMapping, ParameterUnit,
    PortDescriptor, PortId, PortLayout, PortRole, PrepareEffectBankRequest, PrepareEffectRequest,
    PreparedAutomationSpan, PreparedBankMetadata, PreparedEffect, PreparedEffectBank,
    PreparedNativeEffect, PreparedNativeEffectBank, ProcessReport, ResetKind, SmoothingRule,
    StatePayloadError, StatePayloadInput, StatePayloadOutput, StatePayloadSizes, TailSamples,
    expected_prepared_metadata,
};
use effect_runtime::bank::{
    NonFiniteReport, block_is_positive_zero, check_block, nonfinite_lane_mask,
    positive_zero_lanes_of,
};
use effect_runtime::params::{ParameterSpec, is_negative_zero, normalize_zero};
use effect_runtime::ramp::LinearRamp;
use effect_runtime::state_payload as payload;
use lane::kernels::halfband::{
    HALFBAND63_EVEN, HALFBAND63_EVEN_TAPS, HALFBAND63_LIVE_ROWS as LIVE_ROWS,
    HALFBAND63_POS_MASK as POS_MASK,
};
use lane::{Backend, Lane};
use math::db_to_gain_f32;

pub mod corpus;
pub mod kernel;

use kernel::{SoftClipCoef, SoftClipHistory, SoftClipState, soft_clip_block};

#[cfg(test)]
thread_local! {
    /// Blocks the bank-wide silent fast path skipped (issue #1746), counted once per skipped block.
    static SILENT_ADMISSIONS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
    /// While set, the silent fast path is refused and every block renders the whole kernel: the
    /// control arm of the fast-path differential and the rest probe's fork (issue #1746).
    static SILENT_FORBIDDEN: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}

/// `true` when a test has forbidden the silent fast path on this thread. Production: `false`.
#[cfg(test)]
fn silent_forbidden() -> bool {
    SILENT_FORBIDDEN.with(core::cell::Cell::get)
}

/// Counts one silent admission. Test builds only: production has no counter.
#[cfg(test)]
fn admitted_silent() {
    SILENT_ADMISSIONS.with(|blocks| blocks.set(blocks.get() + 1));
}

/// Every lane of an `L`-wide cohort, as a [`Channel::rest_lanes`] mask.
const fn all_lanes<L: Lane>() -> u32 {
    (1 << L::WIDTH) - 1
}

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
    version: 1,
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
    name: &'static str,
    display_name: &'static str,
    display_unit: &'static str,
    unit: ParameterUnit,
    minimum: f32,
    maximum: f32,
    default_value: f32,
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
        prepared_reason: None,
    }
}

/// Frozen scalar soft-clip parameter rows, in stable numeric-ID order.
pub const SOFT_CLIP_PARAMETERS: [ParameterDescriptor; PARAMETER_COUNT] = [
    parameter(
        1,
        "drive_db",
        "drive",
        "dB",
        ParameterUnit::Db,
        -24.0,
        36.0,
        0.0,
    ),
    parameter(
        2,
        "output_db",
        "output",
        "dB",
        ParameterUnit::Db,
        -24.0,
        24.0,
        0.0,
    ),
    parameter(
        3,
        "mix",
        "mix",
        "linear",
        ParameterUnit::Linear,
        0.0,
        1.0,
        1.0,
    ),
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
        maximum_state: StatePayloadSizes {
            common_bytes: STATE_SIZES.common as u32,
            left_bytes: STATE_SIZES.left as u32,
            right_bytes: STATE_SIZES.right as u32,
        },
        scratch_fixed_bytes: 24,
        scratch_bytes_per_frame: 0,
    }
}

/// `(1 + u)^count`, the relative growth of `count` rounded operations.
fn rounding_factor(count: u32) -> f64 {
    (0..count).fold(1.0, |factor, _| {
        factor * (1.0 + effect_runtime::tail::ROUNDOFF)
    })
}

/// The tail `T` beyond the latency, in samples (#1376 D1), from the half-band layout.
///
/// With the input zero from sample `N` on, the last non-zero `X` is `X[N-1]`. The interpolator
/// reads `X[n-1] .. X[n-30]` ([`HALFBAND63_EVEN_TAPS`] ages), so the last non-zero `e` is
/// `e[N-1+30]`; the decimator reads `e[n-1] .. e[n-30]`, so the last non-zero wet sample is at
/// `N - 1 + 60`. The odd phase reads `X[n-31]` and the dry path `x[n-31]` ([`kernel::DRY_DELAY`]),
/// both zero from `N + 31`. The output is therefore exactly `+-0.0` from
/// `N + max(2 * 30, 31) = N + 60`, which is `N + latency + 29`.
const fn tail_samples() -> u64 {
    let wet = 2 * HALFBAND63_EVEN_TAPS;
    let last = if wet > kernel::DRY_DELAY {
        wet
    } else {
        kernel::DRY_DELAY
    };
    (last - kernel::DRY_DELAY) as u64
}

/// The frames a lane renders from a reset before its output is the output a never-reset lane
/// renders on the same input (issue #2129 D3): `latency + T = 31 + 29 = 60`.
///
/// The wet output at `n` reads `e[n-1] .. e[n-30]`, and each `e[m]` reads `X[m-1] .. X[m-30]`
/// ([`tail_samples`]' layout), so the output reads input back to `n - 60`; the odd phase and the
/// dry path read `x[n-31]`. A lane reset at `s` holds `+0.0` for every input before `s`, so from
/// `s + 60` its output reads only post-reset input. The exact-rest bound ([`rest_samples`], 62) is
/// longer because it covers every stored row of each ring, its mirrors included, and an output
/// sample reads only the 30 ages per ring. A ramp holds no input-driven state.
///
/// `W - 1` cannot be made red by any output word: the one reset word the frame `s + 59` still reads
/// is `X[s - 1]`, weighted by the product of the two outermost half-band taps (about `1.7e-9`, below
/// half an `f32` ulp of the frame's other terms), so it rounds away; `W - 2` is red. The
/// derivation is the record.
const fn warmup_samples(declared_latency: u32) -> u32 {
    declared_latency + tail_samples() as u32
}

/// The exact-rest bound `R` beyond the latency, in samples (#1376 D2), from the ring layout.
///
/// The histories are FIR delay lines of [`LIVE_ROWS`] live rows (each mirrored), and the ramps
/// hold no input-driven state, so `Z` is the reset state: every row of every history `+0.0`. The
/// predicate is the one [`Channel::rest_lanes`] pins (`PreparedNativeEffectBank::lanes_at_rest`):
/// every row, mirrors included, not only the ages a payload carries. A row written at sample `n`
/// is overwritten by the push at `n + 32`. The last non-zero `X` and `dry` pushes are at `N - 1`,
/// so those rings are all `+0.0` from `N + 32`; the last non-zero `e` push is at `N - 1 + 30`, so
/// the `e` ring is all `+0.0` from `N + 30 + 32 = N + 62`, which is `N + latency + 31`. It is the
/// same for both peak cases: no word's reach depends on the input's size, only on its last
/// non-zero sample. (Counting the 30 ages the payload carries instead of the 32 rows would state
/// 29, two samples before the predicate can hold.)
const fn rest_samples() -> u64 {
    let e_ring = HALFBAND63_EVEN_TAPS + LIVE_ROWS;
    let last = if e_ring > LIVE_ROWS {
        e_ring
    } else {
        LIVE_ROWS
    };
    (last - kernel::DRY_DELAY) as u64
}

/// `l1e = sum |h[2k]|` over the 30 even taps, in exact `f64` (every tap is an `f32`, and their
/// binades span fewer than 29 bits, so the sum has no rounding).
fn even_tap_l1() -> f64 {
    HALFBAND63_EVEN.iter().map(|tap| f64::from(tap.abs())).sum()
}

/// Rounded operations on the longest path from an input sample to an output sample: the drive
/// product, the interpolator's 30 tap products and 30 sums, the odd phase's halving (exact unless
/// subnormal), the decimator's 31 products (30 taps
/// and the centre) and 31 sums, and the five output operations (`1 - mix`, the two mix products,
/// their sum and the output gain). Counting the sums that start from `+0.0` (which are exact) keeps
/// the count an upper bound.
const ROUNDED_OPERATIONS: u32 = 1 + 30 + 30 + 1 + 31 + 31 + 5;

/// `G_p = G_t` in millibels (#1376 D4): the output law is `y = g_out ((1 - mix) d + mix wet)`, or
/// the dry word `d = x[n-31]` on the identity arms, with `X = 2 g_drive x`, `e = cubic(u)`.
///
/// The cubic shrinks magnitudes, `|cubic(u)| <= |u|` (its Lipschitz constant is `1` and
/// `cubic(0) = 0`; the clamp `+-2/3` is below `|u|` wherever it acts, and the rounded polynomial
/// `fl(u - p2)` with `p2` of the sign of `u` and `|p2| <= |u| / 3` cannot exceed `|u|`), and the
/// flushes only shrink. So `|u| <= l1e max|X|`, `|e| <= |u|`, the odd phase is at most
/// `|0.5 X|`, and `|wet| <= (l1e^2 + 1/4) max|X| = 2 g_drive (l1e^2 + 1/4) max|x|`, with `l1e` the
/// even taps' `l1` norm ([`even_tap_l1`]) and the centre tap `1/2`. Mixed with the dry word,
/// `|y| <= g_out max(1, 2 g_drive (l1e^2 + 1/4)) max|x| (1 + u)^k`, `k` the
/// [`ROUNDED_OPERATIONS`] on any path, at the domain's largest drive and output words (every ramp
/// word stays inside its converted domain). The wet bound uses the cubic's Lipschitz constant, not
/// its clamp, so it is linear in the input: the soft clip's words are fixed designs, an input after
/// silence meets the same operator, and `G_t = G_p`. The `+-2/3` clamp would give an absolute
/// bound, which H1 cannot carry, so it is not used.
fn peak_gain_millibels() -> Option<i32> {
    let drive = f64::from(converted_domain(0)?.1);
    let output = f64::from(converted_domain(1)?.1);
    let l1e = even_tap_l1();
    let wet = 2.0 * drive * (l1e * l1e + 0.25);
    let gain = output * wet.max(1.0) * rounding_factor(ROUNDED_OPERATIONS);
    Some(math::tail::ceil_millibels(
        gain,
        math::tail::Rounding::Computed,
    ))
}

/// `sigma_p = sigma_t` in millibels (#1376 D4, #1379 H3): one [`math::tail::UNDERFLOW`] per
/// rounded operation ([`ROUNDED_OPERATIONS`]), each carried to the output by at most the gain of
/// the operations after it: two half-band stages of `l1` norm at most `l1e + 1/2` (the even taps
/// and the centre), the mix (at most `1`) and the output word, so at most
/// `g_out max(1, l1e + 1/2)^2 (1 + u)^k`. An error made before the drive product does not exist:
/// the drive product is the first rounded operation.
fn flush_stall_millibels() -> Option<i32> {
    let output = f64::from(converted_domain(1)?.1);
    let stage = (even_tap_l1() + 0.5).max(1.0);
    let carried = output * stage * stage * rounding_factor(ROUNDED_OPERATIONS);
    Some(math::tail::ceil_millibels(
        f64::from(ROUNDED_OPERATIONS) * math::tail::UNDERFLOW * carried,
        math::tail::Rounding::Computed,
    ))
}

/// This effect's tail, tail over every peak, exact-rest bound and composition values, the one place
/// they are stated (decision 15 D15-4(b), #1377 D1, #1376).
///
/// * **Tail** [`tail_samples`] (`29`), decay `0`: the output is exactly zero from there.
/// * **Rest** [`rest_samples`] (`31`), for both peak cases; **tail over every peak**
///   `max(T, R) = 31`.
/// * **Composition** [`peak_gain_millibels`] and [`flush_stall_millibels`].
fn tail_and_rest(_sample_rate: u32, _quality: EffectQuality) -> effect_contract::NodeTailBound {
    let (Some(gain), Some(stall)) = (peak_gain_millibels(), flush_stall_millibels()) else {
        return effect_contract::NodeTailBound::UNBOUNDED;
    };
    let tail = tail_samples();
    let rest = rest_samples();
    let gain = effect_contract::PeakGain::Millibels(gain);
    let stall = effect_contract::FlushStall::Level(stall);
    effect_contract::NodeTailBound {
        tail: TailSamples::Finite(tail),
        tail_every_peak: TailSamples::Finite(tail.max(rest)),
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

/// Immutable descriptor for the frozen cubic soft-clip contract.
pub const SOFT_CLIP_DESCRIPTOR: EffectDescriptor = EffectDescriptor {
    id: effect_id("miso.soft-clip"),
    display_name: "Cubic Soft Clip",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: STATE_LAYOUT.version,
    supported_link_modes: LinkModeSet::DUAL_MONO,
    lane_link: false,
    parameters: &SOFT_CLIP_PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
    tail_and_rest,
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

/// `true` if a converted value is finite, not `-0.0`, and inside its range.
///
/// A subnormal is accepted wherever the range holds one: only the mix's `[0, 1]` does (the two
/// gains' converted ranges start at `-24 dB`, a normal gain), and a subnormal mix is a legal value
/// the effect holds and renders with, so its own snapshot must restore it (#1071).
///
/// This is the rule for every ramp word: the target, and the current whether the ramp is at rest
/// or in flight. The clamped law keeps every word the effect's own ramps produce between the
/// ramp's start and its target (issue #1409 D2), both inside the range, so an in-flight current
/// outside it is one the effect never holds and is refused with no rounding margin (issue #1411
/// D1).
///
/// This is control-plane validation of a restored or prepared coefficient, not a render-path
/// check: the render path has none (D7).
fn converted_value_valid(index: usize, value: f32) -> bool {
    if is_negative_zero(value) || !value.is_finite() {
        return false;
    }
    converted_domain(index).is_some_and(|(low, high)| value >= low && value <= high)
}

/// `true` if `step` is an increment parameter `index`'s ramp can hold.
///
/// Only the mix can step by a subnormal: a ramp toward or from a subnormal mix divides a subnormal
/// difference. A gain step is `|Δ| / 64` with `|Δ|` at least an ulp of the `-24 dB` gain (about
/// `7.5e-9`), so it is zero or normal, and a subnormal gain step is never self-produced.
fn ramp_step_valid(index: usize, step: f32) -> bool {
    match index {
        2 => step.is_finite(),
        _ => normal_or_zero(step),
    }
}

/// Converts the six validated initial values into the two per-channel coefficient sets.
///
/// `expected_prepared_metadata` has already rejected negative zero in the initial values; every
/// caller goes through that validation first. Accepted runtime points normalize zero instead.
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
        for (lane, values) in defaults.iter().enumerate() {
            self.reset_lane_full(lane, values);
        }
        self.history.pos = 0;
    }

    /// Clears every history and snaps every ramp to its target, keeping parameters.
    fn reset_discontinuity(&mut self) {
        for lane in 0..L::WIDTH {
            self.reset_lane_discontinuity(lane);
        }
        self.history.pos = 0;
    }

    // REALTIME_POLICY_BEGIN
    /// Zeroes one lane's rows of every history, leaving every other lane bit-unchanged.
    ///
    /// The shared cursor is not moved. It need not be: every row of the lane is zero, so the ring
    /// reads the same at any rotation, which is the property a lane restore already relies on
    /// ([`apply_lane_words`]).
    fn clear_lane_rows(&mut self, lane: usize) {
        let width = L::WIDTH;
        for history in [
            &mut self.history.x,
            &mut self.history.e,
            &mut self.history.dry,
        ] {
            for word in history.iter_mut().skip(lane).step_by(width) {
                *word = 0.0;
            }
        }
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN
    /// [`reset_full`](Self::reset_full) for one lane (issue #2131): its rows of every history are
    /// zeroed and its ramps rest at `defaults`, and every other lane and the shared cursor are left
    /// bit-unchanged, as in [`reset_lane_discontinuity`](Self::reset_lane_discontinuity).
    fn reset_lane_full(&mut self, lane: usize, defaults: &[f32; PARAMETER_COUNT]) {
        self.clear_lane_rows(lane);
        for (parameter, value) in defaults.iter().copied().enumerate() {
            self.ramps[lane][parameter] = LinearRamp::fixed(value);
            set_lane(self.state.field_mut(parameter), lane, value);
        }
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN
    /// [`reset_discontinuity`](Self::reset_discontinuity) for one lane: its rows of every history
    /// are zeroed and its ramps snapped, and every other lane is left bit-unchanged (issue #1092).
    /// The shared cursor is not moved ([`clear_lane_rows`](Self::clear_lane_rows)), so this is
    /// bit-for-bit what a scalar instance's whole reset does to its one lane.
    fn reset_lane_discontinuity(&mut self, lane: usize) {
        self.clear_lane_rows(lane);
        for (parameter, ramp) in self.ramps[lane].iter_mut().enumerate() {
            ramp.snap();
            set_lane(self.state.field_mut(parameter), lane, ramp.current);
        }
    }
    // REALTIME_POLICY_END

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
    ///
    /// `#[inline(never)]` keeps this the one arithmetic kernel of the cohort, as it was before the
    /// silent fast path joined its caller (issue #1746): inlined into `SoftClip::process` it would
    /// move the wasm kernel roster's `Channel<f32x4>::process` row into another symbol.
    #[inline(never)]
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
                drive_target: self.target_vector(0),
                output_target: self.target_vector(1),
                mix_target: self.target_vector(2),
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

    /// The per-lane target vector of one parameter, which no iterated word passes (issue #1409
    /// D4). A lane that is not ramping has a zero step, which the clamp leaves unchanged whatever
    /// the target.
    fn target_vector(&self, parameter: usize) -> L {
        let mut words: LaneWords = [0.0; 8];
        for (lane, slot) in words[..L::WIDTH].iter_mut().enumerate() {
            *slot = self.ramps[lane][parameter].target;
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

    /// The lanes of this channel at their **exact rest** (issue #1746), as a bitmask over
    /// `L::WIDTH` lanes: from this state, one block of exact `+0.0` input leaves every word of the
    /// lane bit-identical and writes `+0.0`.
    ///
    /// The channel's words, per lane, are the three state words (`drive`, `output`, `mix`), the
    /// lane's rows of the three histories (`x`, `e`, `dry`, all 64 double-written rows each), and
    /// every field of its three ramps: the payload (`write_lane_words`) plus the mirror rows and
    /// `ramp.current`. Pinned:
    ///
    /// * every row of the lane in all three histories `+0.0` by raw bits, mirrors included;
    /// * every ramp of the lane settled, `remaining == 0`.
    ///
    /// Why that is enough. A settled lane's increment is `+0.0` (`step_vector`) and it takes no
    /// completion snap (only `remaining == 1` snaps), whichever body the block runs: the settled
    /// body holds the three words, and the ramping body (when another lane ramps) computes
    /// `ramp_toward(w, +0.0, target)`, which is `w`, because `w + 0.0 = w` for every finite, never
    /// `-0.0` ramp word and `w` lies in `[min(w, t), max(w, t)]`. `remaining` and `step` are not
    /// written. `ramp.current` is a copy of the state word at every block boundary: construction,
    /// both resets, the lane reset and a restore write the two together, every rendered block ends
    /// with [`synchronize_currents`](Self::synchronize_currents), and an automation point changes
    /// `current` only in the stationary hoist, to the same bits. So the synchronisation rewrites
    /// it with the bits it holds, and a skipped block need not run it.
    ///
    /// With `+0.0` input every pushed word is `+0.0`: the dry word is the raw `+0.0`; `X =
    /// flush((g + g) * +0.0)` is `+0.0` (`g > 0`, and `flush` clears a zero's sign anyway); each
    /// half-band sum starts at `+0.0` and adds products `h * +0.0`, which are `±0.0`, and `+0.0 +
    /// -0.0` is `+0.0`; `cubic(+0.0)` is `+0.0 - (+0.0 / 3)`, `+0.0`, and is not clamped; so `e`
    /// and `odd` are `+0.0` and the wet sum is `+0.0`. The block writes only `+0.0` into rows that
    /// already hold `+0.0`. The output is `output * ((1 - mix) * +0.0 + mix * +0.0)`: `1 - mix >=
    /// 0` and `mix >= +0.0` (the mix domain is `[0, 1]` and no ramp word is `-0.0`), so both
    /// products and their sum are `+0.0`, and `output > 0` keeps it; the identity and bypass select
    /// takes the `+0.0` dry word instead. The D7 check passes `+0.0`, so no lane recovery runs.
    ///
    /// A dry `-0.0` (a `-0.0` input) is a history word that is not `+0.0`, so the lane is refused:
    /// its identity path would write `-0.0`. Bypass changes nothing above.
    ///
    /// **Clock:** the shared cursor `history.pos`, one per channel of the cohort, which a block
    /// advances by `frames` modulo 32 whatever the signal. It is not a lane word; the bank's silent
    /// fast path advances it ([`advance_cursor`](Self::advance_cursor)).
    fn rest_lanes(&self) -> u32 {
        let mut mask = positive_zero_lanes_of::<L>(&self.history.x)
            & positive_zero_lanes_of::<L>(&self.history.e)
            & positive_zero_lanes_of::<L>(&self.history.dry);
        for (lane, ramps) in self.ramps.iter().enumerate() {
            if ramps.iter().any(|ramp| ramp.remaining != 0) {
                mask &= !(1 << lane);
            }
        }
        mask
    }

    /// Advances the shared cursor as a rendered block of `frames` does: `history_advance` once per
    /// frame, which is `(pos + frames) & POS_MASK`. The one clock a skipped block must move.
    fn advance_cursor(&mut self, frames: usize) {
        self.history.pos = ((self.history.pos as usize + frames) & POS_MASK) as u32;
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
/// prepared automation capacity. Accepted targets go to [`LinearRamp::set_target`], which divides
/// once per event (D11).
fn apply_automation<L: Lane>(
    spans: &[PreparedAutomationSpan],
    automation_capacity: u32,
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
        let valid = span_index < automation_capacity as usize
            && parameter < PARAMETER_COUNT
            && span.sample == first_sample
            && parameter_value_valid(parameter, span.value)
            && prior.is_none_or(|previous| order > previous)
            && pending[channel][parameter].is_none();
        if !valid {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        }
        let Some(value) = convert_parameter(parameter, normalize_zero(span.value)) else {
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
// State payload, layout version 1
// ---------------------------------------------------------------------------------------------

// REALTIME_POLICY_BEGIN: #1278 D3, the payload calls run in the plan-swap block.
/// Reads one lane of one channel into the 104 payload words of layout 1.
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

/// Validates the 104 payload words of layout 1.
///
/// The rule is "accept every word the effect itself can hold, refuse the rest" (#1071), so a
/// snapshot of a lane that only ever held its own words always survives its own restore and a
/// restored lane continues bit for bit:
///
/// * ramp targets and currents, at rest or in flight, must be inside the *converted* domain (a
///   linear gain, not decibels) and must not be `-0.0`; a subnormal is in domain only for the mix
///   ([`converted_value_valid`]; the clamped law keeps every in-flight current between its start
///   and its target, issue #1409 D2, so no rounding margin is admitted, issue #1411 D1);
/// * `step` must be finite, and for the gains zero or normal ([`ramp_step_valid`]);
/// * `remaining` must not exceed the smoothing window;
/// * each history admits every word the kernel can write into it (#1300):
///   * `X` admits every word except a subnormal ([`x_history_word_valid`]);
///   * `e` admits a zero, a normal or a NaN ([`e_history_word_valid`]);
///   * the dry history admits every word ([`dry_history_word_valid`]).
///
///   The `X` and `e` rules also admit some words the kernel never writes: a normal below the
///   flush threshold (`1e-20`), and in `X` a `-0.0`. `flush` writes neither. Tightening this is
///   out of scope (#1071 attempt-1 NIT-1).
///
///   A history word may be non-finite at a block boundary while the output is still finite, so
///   D7 (which checks only the output) has not cleared it. That is sound: soft-clip has no
///   recursive state, its three histories are FIR delay lines, and the kernel reads no word older
///   than 31 samples. A non-finite history word has one of three outcomes:
///   * it reaches the output, and D7 zeroes the block and resets the lane;
///   * `cubic` reads it and clamps it to a finite value (an `X` infinity gives `±2/3`);
///   * it feeds only a path that the identity select discards (an `e` NaN on the identity or
///     bypass path).
///
///   A word the effect writes itself lives at most 31 samples without D7. A crafted payload can
///   hold an `X` NaN beside a finite dry word, which the effect never produces. On the identity
///   path that `X` NaN can keep making `e` NaNs for up to about 59 samples without D7. If the mix
///   or output leaves identity in that time, D7 fires. Nothing is recursive, so no output leaves
///   the D7 bound: a crafted payload can cause no output beyond a D7 recovery.
///
/// Flushing on snapshot instead would change rendered bits (a subnormal dry sample is the output
/// at `mix == 0`) and break the exact continuation the plan-swap carry relies on (#1278 D2a).
/// The cursor belongs to the bank and is not carried in the payload.
fn decode_lane_words(words: &[u32]) -> Result<LaneRestore, StatePayloadError> {
    debug_assert_eq!(words.len(), LANE_STATE_WORDS as usize);
    let mut ramps = [LinearRamp::fixed(0.0); PARAMETER_COUNT];
    for (parameter, ramp) in ramps.iter_mut().enumerate() {
        let base = parameter * RAMP_WORDS;
        let current = f32::from_bits(words[base]);
        let target = f32::from_bits(words[base + 1]);
        let step = f32::from_bits(words[base + 2]);
        let remaining = words[base + 3];
        if remaining > RAMP_SAMPLES
            || !converted_value_valid(parameter, target)
            || !ramp_step_valid(parameter, step)
            || !converted_value_valid(parameter, current)
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
    let rules: [fn(f32) -> bool; 3] = [
        x_history_word_valid,
        e_history_word_valid,
        dry_history_word_valid,
    ];
    for ((slot, offset), valid) in [
        (&mut restore.x[..], X_HISTORY_WORD),
        (&mut restore.e[..], E_HISTORY_WORD),
        (&mut restore.dry[..], DRY_HISTORY_WORD),
    ]
    .into_iter()
    .zip(rules)
    {
        for (age, value) in slot.iter_mut().enumerate() {
            let word = f32::from_bits(words[offset + age]);
            if !valid(word) {
                return Err(StatePayloadError {
                    code: STATE_HISTORY_CODE,
                });
            }
            *value = word;
        }
    }
    Ok(restore)
}

/// `true` if `value` is a word the interpolator input history `X` can hold: every word except a
/// subnormal.
///
/// The kernel pushes `flush(2 * drive * xin)` (`kernel.rs:192`). `flush` zeroes every magnitude
/// below `1e-20` and leaves a NaN or an infinity unchanged, so `X` holds a zero, a normal, an
/// infinity (a finite input above about `2.7e36` at `+36 dB` overflows, or the input is infinite)
/// or a NaN (a NaN input). The drive gain is at least `-24 dB`, so the product is never `0 * inf`.
fn x_history_word_valid(value: f32) -> bool {
    !value.is_subnormal()
}

/// `true` if `value` is a word the decimation history `e` can hold: a zero, a normal or a NaN.
///
/// The kernel pushes `flush(cubic(u))` (`kernel.rs:196`), where `u` is interpolated from `X`.
/// `cubic` clamps `±inf` to `±2/3` and returns NaN for NaN, so `e` is never infinite; a NaN comes
/// from a NaN in `X`, or from two infinite `X` words in the interpolation window whose half-band
/// tap products have opposite signs (`inf - inf`). The even taps alternate in sign, so two
/// infinities of the same sign can also do this.
fn e_history_word_valid(value: f32) -> bool {
    normal_or_zero(value) || value.is_nan()
}

/// `true` for every word: the dry history holds the raw input `xin` unflushed (`kernel.rs:191`),
/// and a host may submit any `f32`, so the identity path can reproduce any input sample.
fn dry_history_word_valid(_value: f32) -> bool {
    true
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
// REALTIME_POLICY_END

// ---------------------------------------------------------------------------------------------
// Prepared instances
// ---------------------------------------------------------------------------------------------

/// A prepared soft-clip cohort of `L::WIDTH` dual-mono tracks.
///
/// `WIDTH = 1` is the scalar instance the contract's `PreparedNativeEffect` uses, and 4 and 8 are
/// the banks; the type, the driver and the kernel are the same in all three cases.
///
/// It keeps no copy of its `PreparedEffectMetadata` (issue #1461): the prepare result carries that
/// record on the control side, and the cohort keeps only the prepared values its own code reads.
struct SoftClip<L: Lane> {
    /// The session's bypass, read by `process` and `process_bank`.
    bypass: bool,
    /// The prepared automation span capacity, read by `apply_automation`.
    automation_capacity: u32,
    left_defaults: Box<[[f32; PARAMETER_COUNT]]>,
    right_defaults: Box<[[f32; PARAMETER_COUNT]]>,
    left: Channel<L>,
    right: Channel<L>,
    nonfinite: NonFiniteReport,
    /// Bit `l` is set when lane `l` carries a member (issue #1092). A padded lane's bit is clear:
    /// it takes no automation, and D7 recovers it without charging it (the padding contract on
    /// `PrepareEffectBankRequest`). Every lane of a scalar instance and of a full bank is active.
    active: u32,
    /// Whether a collapsed block (`process::<true>`) has rendered since the last disengage copy
    /// ([`desymmetrize`](Self::desymmetrize)), which alone clears it (#1894). While it is set the
    /// right channel is frozen and stale, so [`lanes_at_rest`](Self::lanes_at_rest) reads the left
    /// channel alone and a skipped block advances the left cursor alone, as the collapsed silent
    /// path does: the effect contract's collapsed-bank rule.
    collapsed: bool,
}

impl<L: Lane> SoftClip<L> {
    fn new(
        bypass: bool,
        automation_capacity: u32,
        left_defaults: Box<[[f32; PARAMETER_COUNT]]>,
        right_defaults: Box<[[f32; PARAMETER_COUNT]]>,
    ) -> Self {
        let left = Channel::new(&left_defaults);
        let right = Channel::new(&right_defaults);
        Self {
            bypass,
            automation_capacity,
            left_defaults,
            right_defaults,
            left,
            right,
            nonfinite: NonFiniteReport::new(),
            active: (1 << L::WIDTH) - 1,
            collapsed: false,
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

    // REALTIME_POLICY_BEGIN
    /// `PreparedNativeEffectBank::reset_lane` (issue #2131): lane `lane`'s part of
    /// [`reset`](Self::reset) on both channels, and nothing of any other lane. Each channel's
    /// shared cursor stays where it is, as the D7 recovery leaves it: the lane's rows are all zero,
    /// so they read the same at any rotation, and the payload is age-ordered from the cursor.
    /// `collapsed` stays as the whole-bank reset leaves it. A lane past the width changes nothing.
    /// Bounded by one lane's rows and words.
    fn reset_lane(&mut self, lane: usize, kind: ResetKind) {
        if lane >= L::WIDTH {
            return;
        }
        match kind {
            ResetKind::FullToDefaults => {
                self.left.reset_lane_full(lane, &self.left_defaults[lane]);
                self.right.reset_lane_full(lane, &self.right_defaults[lane]);
            }
            ResetKind::DiscontinuityKeepParameters => {
                self.left.reset_lane_discontinuity(lane);
                self.right.reset_lane_discontinuity(lane);
            }
        }
    }
    // REALTIME_POLICY_END

    /// Renders one block of both channels and applies the master plan §4.4 boundary check.
    ///
    /// Returns, per channel (`[left, right]`), the bitmask of the active lanes whose own output on
    /// that channel failed the D7 check, `[0, 0]` when the block was accepted. A failing lane is
    /// recovered on both channels, but each channel reports only its own failure, once per block
    /// (#1073). See [`recover_lanes`](Self::recover_lanes).
    ///
    /// # The silent fast path (issue #1746)
    ///
    /// Whole-cohort, never per lane, and decided afresh on every block from state, after this
    /// block's automation was applied: when both input planes are exact `+0.0` and every lane,
    /// padded lanes included, is at rest ([`lanes_at_rest`](Self::lanes_at_rest)), the kernel is
    /// skipped. The buffers already hold the `+0.0` it would write, no lane word would move, and the
    /// D7 check would pass, so the block reports `[0, 0]` as a rendered one does. The one word a
    /// rendered block would still move is each channel's cursor, which is advanced here. Bit tests
    /// and integer arithmetic only.
    ///
    /// # The collapsed body (issue #1872)
    ///
    /// `MONO` renders the left channel alone, on the left plane alone: `right` is never read or
    /// written (a collapsed block passes it empty), and the right channel's words are left exactly
    /// as they were, for [`desymmetrize`](Self::desymmetrize) to replace. The silent fast path
    /// reads the left plane and the left channel's rest only and advances the left cursor only,
    /// because a collapsed bank's counterfactual right channel is its left one. A lane that fails
    /// the D7 check on the left plane is recovered on **both** channels and reported on both, as
    /// the dual block on `L == R` fails both planes in the same lanes; the right recovery is
    /// overwritten by the disengage copy, and is kept so that the right words never hold a state
    /// no dual run could reach.
    fn process<const MONO: bool>(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: usize,
        bypass: bool,
    ) -> [u32; 2] {
        #[cfg(test)]
        let allowed = !silent_forbidden();
        #[cfg(not(test))]
        let allowed = true;
        if MONO {
            // From here until the disengage copy the right channel is frozen and stale.
            self.collapsed = true;
        }
        let silent = if MONO {
            block_is_positive_zero(left) && self.left.rest_lanes() == all_lanes::<L>()
        } else {
            block_is_positive_zero(left)
                && block_is_positive_zero(right)
                && self.lanes_at_rest() == all_lanes::<L>()
        };
        if allowed && silent {
            #[cfg(test)]
            admitted_silent();
            self.left.advance_cursor(frames);
            if !MONO {
                self.right.advance_cursor(frames);
            }
            return [0, 0];
        }
        self.left.process(left, frames, bypass);
        if MONO {
            if check_block::<L>(left) {
                return [0, 0];
            }
            let failed = nonfinite_lane_mask::<L>(left);
            let charged = self.recover_lanes(left, &mut [], failed);
            return [failed & charged; 2];
        }
        self.right.process(right, frames, bypass);
        if check_block::<L>(left) && check_block::<L>(right) {
            return [0, 0];
        }
        let failed = [
            nonfinite_lane_mask::<L>(left),
            nonfinite_lane_mask::<L>(right),
        ];
        let charged = self.recover_lanes(left, right, failed[0] | failed[1]);
        [failed[0] & charged, failed[1] & charged]
    }

    /// The lanes at exact rest on both channels (`PreparedNativeEffectBank::lanes_at_rest`), or
    /// on the left channel alone while [`collapsed`](Self::collapsed) (#1894): see
    /// [`Channel::rest_lanes`] for the words pinned, why the list is complete, and the one clock.
    fn lanes_at_rest(&self) -> u32 {
        if self.collapsed {
            self.left.rest_lanes()
        } else {
            self.left.rest_lanes() & self.right.rest_lanes()
        }
    }

    /// The D7 recovery, one lane at a time (issue #1092; decision 12's coupling rule).
    ///
    /// A lane that failed the boundary check on either channel has both of its channels zeroed,
    /// its rows of every history cleared and its ramps snapped, which is exactly what the scalar
    /// instance does to its one lane; every other lane is left bit-unchanged. The shared
    /// `effect_runtime::bank::finish_block` zeroed and reset the whole bank and this crate charged
    /// every lane, so one hot track -- a bypassed one whose wet path still runs (#1087) -- silenced
    /// and charged its bank-mates.
    ///
    /// A padded lane is recovered like any other, so it stays at rest, but it is never charged.
    /// A collapsed block passes `right` empty; both channels' words are still recovered.
    /// Returns the charged lanes; when there are any, the bank's `nonfinite_blocks` advances by one
    /// -- a *block*, never a sample -- and `nonfinite_lanes` records them.
    fn recover_lanes(&mut self, left: &mut [f32], right: &mut [f32], failed: u32) -> u32 {
        let width = L::WIDTH;
        for lane in 0..width {
            if failed & (1 << lane) == 0 {
                continue;
            }
            for word in left.iter_mut().skip(lane).step_by(width) {
                *word = 0.0;
            }
            for word in right.iter_mut().skip(lane).step_by(width) {
                *word = 0.0;
            }
            self.left.reset_lane_discontinuity(lane);
            self.right.reset_lane_discontinuity(lane);
        }
        let charged = failed & self.active;
        if charged != 0 {
            self.nonfinite.nonfinite_lanes = charged;
            self.nonfinite.nonfinite_blocks = self.nonfinite.nonfinite_blocks.saturating_add(1);
        }
        charged
    }

    /// The collapse's disengage copy (issue #1872): the left channel's whole render state onto
    /// the right, every lane. That is every word [`Channel::process`] reads per channel: the
    /// three [`SoftClipState`] words, the history cursor `pos`, the whole `x`, `e` and `dry`
    /// rings (all 64 rows: the rings are double-written and the taps read both copies), and the
    /// ramps, every field. In place, by `copy_from_slice` of equal lengths: no allocation.
    ///
    /// `left_defaults` and `right_defaults` are not copied: they are each channel's own prepared
    /// values, which the kernel never reads (a full reset, preparation-side, re-seeds from them).
    fn desymmetrize(&mut self) {
        let (left, right) = (&self.left, &mut self.right);
        right.state = left.state;
        right.history.pos = left.history.pos;
        right.history.x.copy_from_slice(&left.history.x);
        right.history.e.copy_from_slice(&left.history.e);
        right.history.dry.copy_from_slice(&left.history.dry);
        right.ramps.copy_from_slice(&left.ramps);
        self.collapsed = false;
    }

    /// `PreparedNativeEffectBank::channels_agree` (issue #1872): every word
    /// [`desymmetrize`](Self::desymmetrize) copies compares bit-equal between the channels, on
    /// every lane. The whole per-channel state is `3 * 64 + 3` lane words plus the cursor and the
    /// ramps (about 1.6k words at eight lanes), small enough to compare outright, as the
    /// contract's cost rule allows; it is asked only inside a recovery window.
    fn channels_bit_equal(&self) -> bool {
        let (left, right) = (&self.left, &self.right);
        let same_words = |a: &[f32], b: &[f32]| {
            a.len() == b.len() && a.iter().zip(b).all(|(a, b)| a.to_bits() == b.to_bits())
        };
        (0..L::WIDTH).all(|lane| same_state_lane(left, right, lane))
            && left.history.pos == right.history.pos
            && same_words(&left.history.x, &right.history.x)
            && same_words(&left.history.e, &right.history.e)
            && same_words(&left.history.dry, &right.history.dry)
            && left.ramps.len() == right.ramps.len()
            && left
                .ramps
                .iter()
                .flatten()
                .zip(right.ramps.iter().flatten())
                .all(|(a, b)| same_ramp_bits(a, b))
    }

    /// The `DESIGNED` term of the channel-symmetry witness for `lane` (issue #1872): whether every
    /// per-channel word [`Channel::process`] reads that is not running state compares bit-equal
    /// between the two channels.
    ///
    /// # The word list, and why it is exactly this
    ///
    /// Per channel the kernel reads the lane's three ramps (`ramps[lane]`, all four fields each:
    /// `remaining` decides whether a ramp moves, when it snaps and where a segment ends, and
    /// `current`, `target` and `step` are what it moves; [`SoftClipCoef`] is derived from them per
    /// segment) and the lane's three [`SoftClipState`] words (`drive`, `output`, `mix`), the ramps'
    /// current of record. Twelve ramp words and three state words.
    ///
    /// Deliberately excluded:
    ///
    /// * `history.pos` and the `x`, `e` and `dry` rings: running state, which the induction the
    ///   witness states carries.
    /// * `left_defaults` and `right_defaults`: prepared values the kernel never loads.
    /// * `bypass`, `automation_capacity`, `active`: whole-instance. There are no link words: the
    ///   descriptor offers `DualMono` only.
    ///
    /// Raw bits, so `+0.0` against `-0.0` declines. A lane past the width declines.
    fn designed_channel_symmetry(&self, lane: usize) -> bool {
        if lane >= L::WIDTH {
            return false;
        }
        let (left, right) = (&self.left, &self.right);
        left.ramps[lane]
            .iter()
            .zip(&right.ramps[lane])
            .all(|(a, b)| same_ramp_bits(a, b))
            && same_state_lane(left, right, lane)
    }
}

/// Whether two ramps hold the same four words, raw bits (issue #1872).
fn same_ramp_bits(a: &LinearRamp, b: &LinearRamp) -> bool {
    a.current.to_bits() == b.current.to_bits()
        && a.target.to_bits() == b.target.to_bits()
        && a.step.to_bits() == b.step.to_bits()
        && a.remaining == b.remaining
}

/// Whether lane `lane` of two channels holds the same three [`SoftClipState`] words, raw bits.
fn same_state_lane<L: Lane>(left: &Channel<L>, right: &Channel<L>, lane: usize) -> bool {
    (0..PARAMETER_COUNT).all(|parameter| {
        lane_of(left.state.field(parameter), lane).to_bits()
            == lane_of(right.state.field(parameter), lane).to_bits()
    })
}

impl<L: Lane> SoftClip<L> {
    /// The heap this cohort owns (#1074): both default tables, and per channel its ramp table and
    /// its three histories. Every other field is inline.
    fn heap_bytes(&self) -> u64 {
        use effect_contract::retained::slice;
        let channel = |channel: &Channel<L>| {
            let history = &channel.history;
            slice(&channel.ramps) + slice(&history.x) + slice(&history.e) + slice(&history.dry)
        };
        slice(&self.left_defaults)
            + slice(&self.right_defaults)
            + channel(&self.left)
            + channel(&self.right)
    }
}

/// A prepared scalar soft-clip dual-mono instance.
struct PreparedSoftClip {
    inner: SoftClip<f32>,
}

/// A prepared homogeneous soft-clip cohort.
///
/// Its `PreparedBankMetadata` is returned beside it (issue #1461); it keeps only the two bank
/// values `process_bank` reads.
struct PreparedSoftClipBank<L: Lane> {
    /// The bound bank width, read by `process_bank`'s block check and report.
    width: BankWidth,
    /// The prepared quantum, read by `process_bank`'s block check.
    quantum: u32,
    inner: SoftClip<L>,
}

/// The bank's one clock, advanced for a skipped block (issue #1747).
impl<L: Lane> PreparedSoftClipBank<L> {
    /// Advances the bank's shared history position `pos` on both channels by `frames`, exactly as
    /// one rendered block of `frames` frames of `+0.0` input would; on the left channel alone on
    /// a collapsed bank, as the collapsed block would (#1894).
    ///
    /// `pos` is the soft clip's only clock: the kernel advances it once per frame by
    /// `history_advance`, a step of one modulo the 32 live rows, whatever the signal. Nothing else
    /// a zero block changes is a clock. A lane at rest has every ramp settled (a ramp in flight
    /// clears its rest bit), so a zero block leaves its ramp words alone; and every row of its
    /// three histories already holds `+0.0`, which is exactly what a zero block would push
    /// (`xin = +0.0`, `X = flush(2g * +0.0) = +0.0` for a positive gain, and `e = flush(cubic(u))`
    /// with `u` an ascending sum from `+0.0` of zero products, so `+0.0`). A ring of `+0.0` reads
    /// the same at every rotation, so moving `pos` without writing the rows leaves every state word
    /// and every payload word bit-identical to the rendered block.
    ///
    /// The position is per bank, so it moves whatever `lanes_mask` holds: the caller skips a bank
    /// whole and passes every lane, and a lane in the mask that was not at rest is a caller defect
    /// (its rows would rotate under it). Two banks of one slot need not hold equal positions: a
    /// lane swap between them stays exact because a restore places each row by its age relative
    /// to the receiving bank's own position (`apply_lane_words`). The advance is the silent fast
    /// path's own ([`Channel::advance_cursor`]): constant time.
    fn advance_skipped_clocks(&mut self, lanes_mask: u32, frames: u32) {
        debug_assert_eq!(
            lanes_mask & all_lanes::<L>() & !self.inner.lanes_at_rest(),
            0,
            "a lane not at rest"
        );
        self.inner.left.advance_cursor(frames as usize);
        if !self.inner.collapsed {
            self.inner.right.advance_cursor(frames as usize);
        }
    }
}

impl NativeEffectFactory for SoftClipFactory {
    fn descriptor(&self) -> &'static EffectDescriptor {
        &SOFT_CLIP_DESCRIPTOR
    }

    fn prepared_inline_bytes(&self) -> u64 {
        size_of::<PreparedSoftClip>() as u64
    }

    fn prepared_undeclared_heap_bytes(&self, quality: &effect_contract::QualityDescriptor) -> u64 {
        // One lane per channel: its defaults row, its ramps and its three histories, each
        // `HALFBAND63_ROWS` long because the 31-row window is double-written; the payload
        // carries one copy of each window and the ramps' fields.
        let channel = PARAMETER_COUNT * (size_of::<f32>() + size_of::<LinearRamp>())
            + 3 * lane::kernels::halfband::HALFBAND63_ROWS * size_of::<f32>();
        (2 * channel as u64).saturating_sub(quality.maximum_state.total().unwrap_or(0))
    }

    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<PreparedEffect, EffectPrepareError> {
        let metadata = expected_prepared_metadata(self.descriptor(), request)?;
        let (left, right) = initial_defaults(request.initial_values)?;
        Ok(PreparedEffect {
            processor: Box::new(PreparedSoftClip {
                inner: SoftClip::new(
                    metadata.bypass,
                    metadata.automation_capacity,
                    vec![left].into_boxed_slice(),
                    vec![right].into_boxed_slice(),
                ),
            }),
            metadata,
        })
    }

    /// Issue #2199: preparation seats every ramp fixed at its default and every state word at its
    /// default (`Channel::new`), and those are all the words
    /// `SoftClip::designed_channel_symmetry` compares, so the two channels agree exactly when
    /// their defaults do, bit for bit. No instance is built.
    fn designed_channel_symmetry(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<bool, EffectPrepareError> {
        expected_prepared_metadata(self.descriptor(), request)?;
        let (left, right) = initial_defaults(request.initial_values)?;
        Ok(left
            .iter()
            .zip(&right)
            .all(|(left, right)| left.to_bits() == right.to_bits()))
    }

    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<PreparedEffectBank>, EffectPrepareError> {
        bind_bank::<true>(self, request)
    }
}

/// `bind_homogeneous_bank`, with the D4 width check as a switch: `NATIVE_ONLY` declines a width
/// this artifact does not execute, which is what production asks for. The unit tests pass `false`
/// to bind the other width's bank on this host through the same code.
fn bind_bank<const NATIVE_ONLY: bool>(
    factory: &SoftClipFactory,
    request: PrepareEffectBankRequest<'_>,
) -> Result<Option<PreparedEffectBank>, EffectPrepareError> {
    request.validate_shape()?;
    Ok(effect_contract::match_bank_width!(request.width, |L| {
        boxed::<L, NATIVE_ONLY>(prepare_bank::<_, NATIVE_ONLY>(factory, request)?)
    }))
}

/// Boxes a bank bound at `L::WIDTH` lanes.
///
/// The D4 check of [`prepare_bank`] is repeated here, where it is a compile-time constant ahead of
/// the only reference to the bank's vtable, so a production build never links the render code of
/// a width it does not execute. `prepare_bank` cannot promise that alone: it returns the bank by
/// value, and the eight-lane soft clip would otherwise stay in the four-lane browser artifact.
fn boxed<L: Lane, const NATIVE_ONLY: bool>(
    bank: Option<(PreparedSoftClipBank<L>, PreparedBankMetadata)>,
) -> Option<PreparedEffectBank>
where
    PreparedSoftClipBank<L>: PreparedNativeEffectBank,
{
    if NATIVE_ONLY && !executes(L::WIDTH) {
        return None;
    }
    bank.map(|(bank, metadata)| PreparedEffectBank {
        processor: Box::new(bank) as Box<dyn PreparedNativeEffectBank>,
        metadata,
    })
}

/// `true` if this artifact executes banks of `lanes` lanes natively.
///
/// D4 replaced runtime SIMD dispatch with a compile-time ISA pin plus a boot attestation, so this
/// is a compile-time constant and not a CPUID question. A width the artifact was not built for is
/// declined with `Ok(None)`, and the caller falls back to scalar instances.
const fn executes(lanes: usize) -> bool {
    lanes == Backend::current().width()
}

/// Binds one bank of `L::WIDTH` lanes, returned beside its `PreparedBankMetadata`.
///
/// # Padding (issue #1092; decision 12)
///
/// The soft clip accepts a padded request. Each lane, active or padded, is prepared from its own
/// request, which for a padded lane is the caller's clone of an active member's, so every lane is
/// validated. A padded lane takes no automation, and D7 recovers it without charging it
/// ([`SoftClip::recover_lanes`]). Fed `+0.0` it writes `+0.0`: every history holds zeros, so the
/// dry word is `+0.0`, and the wet sum is `+0.0` scaled by a positive output gain. The soft clip
/// never reads across lanes, and its one whole-bank decision, the ramp segmentation, is
/// partition-invariant (`tests/ramp_law.rs`), so an active lane's bits depend neither on the
/// padded lanes nor on which member they clone.
fn prepare_bank<L: Lane, const NATIVE_ONLY: bool>(
    factory: &SoftClipFactory,
    request: PrepareEffectBankRequest<'_>,
) -> Result<Option<(PreparedSoftClipBank<L>, PreparedBankMetadata)>, EffectPrepareError> {
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
    for (track, member) in request.requests.iter().copied().enumerate().skip(1) {
        let candidate = expected_prepared_metadata(factory.descriptor(), member)?;
        if candidate.program_key() != metadata.program_key() {
            same_program = false;
        }
        let (left, right) = initial_defaults(member.initial_values)?;
        left_defaults[track] = left;
        right_defaults[track] = right;
    }
    // `L::WIDTH` is a compile-time constant here, so the D4 check folds (see [`boxed`]).
    if !same_program || (NATIVE_ONLY && !executes(L::WIDTH)) {
        return Ok(None);
    }
    let mut inner = SoftClip::new(
        metadata.bypass,
        metadata.automation_capacity,
        left_defaults.into_boxed_slice(),
        right_defaults.into_boxed_slice(),
    );
    inner.active = request
        .active_mask
        .iter()
        .enumerate()
        .filter(|(_, active)| **active)
        .fold(0, |bits, (lane, _)| bits | (1 << lane));
    Ok(Some((
        PreparedSoftClipBank::<L> {
            width: request.width,
            quantum: metadata.quantum,
            inner,
        },
        PreparedBankMetadata {
            width: request.width,
            program_key: metadata.program_key(),
        },
    )))
}

impl PreparedNativeEffect for PreparedSoftClip {
    /// Issue #2129 D3: the latency plus the tail, [`warmup_samples`].
    fn warmup_samples(&self, declared_latency: u32) -> u32 {
        warmup_samples(declared_latency)
    }

    fn retained_bytes(&self) -> u64 {
        self.inner.heap_bytes()
    }

    // REALTIME_POLICY_BEGIN
    fn reset(&mut self, kind: ResetKind) {
        self.inner.reset(kind);
    }
    // REALTIME_POLICY_END

    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        let mut report = ProcessReport::default();
        let frames = block.frames();
        apply_automation(
            block.automation,
            self.inner.automation_capacity,
            block.first_sample,
            0,
            &mut self.inner.left,
            &mut self.inner.right,
            &mut report,
        );
        let bypass = self.inner.bypass;
        // One block on each channel whose output failed, never its frames (D7, #1073).
        let [left, right] = self
            .inner
            .process::<false>(block.left, block.right, frames, bypass);
        report.nonfinite_left_blocks = report
            .nonfinite_left_blocks
            .saturating_add(u64::from(left & 1));
        report.nonfinite_right_blocks = report
            .nonfinite_right_blocks
            .saturating_add(u64::from(right & 1));
        report
    }

    // REALTIME_POLICY_BEGIN: #1278 D3, the payload calls run in the plan-swap block.
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
    // REALTIME_POLICY_END

    /// Issue #1872: [`SoftClip::designed_channel_symmetry`] on lane 0.
    fn channel_symmetry(&self) -> bool {
        self.inner.designed_channel_symmetry(0)
    }
}

impl<L: Lane> PreparedNativeEffectBank for PreparedSoftClipBank<L> {
    /// Issue #2129 D3: the latency plus the tail, [`warmup_samples`].
    fn warmup_samples(&self, declared_latency: u32) -> u32 {
        warmup_samples(declared_latency)
    }

    // REALTIME_POLICY_BEGIN
    fn reset_lane(&mut self, lane: usize, kind: ResetKind) {
        self.inner.reset_lane(lane, kind);
    }
    // REALTIME_POLICY_END
    fn retained_bytes(&self) -> u64 {
        self.inner.heap_bytes()
    }

    fn reset(&mut self, kind: ResetKind) {
        self.inner.reset(kind);
    }

    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        self.process_bank_inner::<false>(block)
    }

    /// Issue #1872: [`SoftClip::designed_channel_symmetry`].
    fn lane_channel_symmetry(&self, lane: usize) -> bool {
        self.inner.designed_channel_symmetry(lane)
    }

    fn supports_mono_collapse(&self) -> bool {
        true
    }

    fn process_bank_mono(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        self.process_bank_inner::<true>(block)
    }

    fn desymmetrize_channels(&mut self) {
        self.inner.desymmetrize();
    }

    fn channels_agree(&self) -> bool {
        self.inner.channels_bit_equal()
    }

    /// Issue #1746: [`Channel::rest_lanes`] on both channels, padded lanes included; on the left
    /// channel alone on a collapsed bank (#1894).
    fn lanes_at_rest(&self) -> u32 {
        self.inner.lanes_at_rest()
    }

    // REALTIME_POLICY_BEGIN: #1278 D3, the payload calls run in the plan-swap block.
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
    // REALTIME_POLICY_END

    fn advance_clocks(&mut self, lanes_mask: u32, frames: u32) {
        self.advance_skipped_clocks(lanes_mask, frames);
    }

    fn lane_default_bytes(&self) -> Option<usize> {
        Some(LANE_DEFAULT_BYTES)
    }

    fn exchange_lane_defaults(&mut self, lane: usize, bytes: &mut [u8]) -> bool {
        self.exchange_defaults(lane, bytes)
    }
}

impl<L: Lane> PreparedSoftClipBank<L> {
    /// The one bank body, dual or collapsed (issue #1872).
    ///
    /// `MONO` chooses the render body and nothing else: the guards, the automation drain (both
    /// channels' spans, as a dual block takes them) and the report are the same statements in the
    /// same order. A collapsed block hands the kernel no right plane, so the right plane is never
    /// read or written. Inlined into `process_bank` and `process_bank_mono`; the arithmetic stays
    /// in the one `#[inline(never)]` [`Channel::process`].
    #[inline(always)]
    fn process_bank_inner<const MONO: bool>(
        &mut self,
        block: EffectBankProcessBlock<'_>,
    ) -> BankProcessReport {
        let mut report = BankProcessReport::empty(self.width);
        if !bank_block_matches(&block, self.width, self.quantum)
            || L::WIDTH != self.width.lanes() as usize
        {
            return report;
        }
        for lane in 0..L::WIDTH {
            // A padded lane carries no track, so no span is its to apply (issue #1092).
            if self.inner.active & (1 << lane) == 0 {
                continue;
            }
            let start = block.automation_offsets[lane] as usize;
            let end = block.automation_offsets[lane + 1] as usize;
            apply_automation(
                &block.automation[start..end],
                self.inner.automation_capacity,
                block.first_sample,
                lane,
                &mut self.inner.left,
                &mut self.inner.right,
                &mut report.reports[lane],
            );
        }
        let frames = block.frames as usize;
        let bypass = self.inner.bypass;
        // Only the lanes that failed are charged, never a bank-mate or a padded lane (issue
        // #1092). Each is charged as its scalar instance charges itself: one block on each
        // channel whose own output failed, never the block's frames (D7, #1073).
        let right_plane = if MONO { &mut [][..] } else { block.right };
        let [left, right] = self
            .inner
            .process::<MONO>(block.left, right_plane, frames, bypass);
        for (lane, lane_report) in report.reports.iter_mut().enumerate().take(L::WIDTH) {
            lane_report.nonfinite_left_blocks = lane_report
                .nonfinite_left_blocks
                .saturating_add(u64::from((left >> lane) & 1));
            lane_report.nonfinite_right_blocks = lane_report
                .nonfinite_right_blocks
                .saturating_add(u64::from((right >> lane) & 1));
        }
        report
    }
}

/// One lane's prepared defaults in bytes: every parameter of both channels, one `f32` each.
const LANE_DEFAULT_BYTES: usize = 2 * PARAMETER_COUNT * 4;

// REALTIME_POLICY_BEGIN: #1747, the prepared defaults move with a render-time lane swap.
impl<L: Lane> PreparedSoftClipBank<L> {
    /// [`PreparedNativeEffectBank::exchange_lane_defaults`]: lane `lane`'s `left_defaults` row
    /// then its `right_defaults` row, each parameter in descriptor order as little-endian `f32`
    /// bits. They are the soft clip's only prepared per-lane values: a full reset re-seeds the
    /// lane's ramps and state words from them. The D7 recovery snaps ramps and reads no default.
    fn exchange_defaults(&mut self, lane: usize, bytes: &mut [u8]) -> bool {
        if lane >= L::WIDTH || bytes.len() != LANE_DEFAULT_BYTES {
            return false;
        }
        let inner = &mut self.inner;
        let words = inner.left_defaults[lane]
            .iter_mut()
            .chain(inner.right_defaults[lane].iter_mut());
        for (value, chunk) in words.zip(bytes.as_chunks_mut::<4>().0.iter_mut()) {
            let mut incoming = [0_u8; 4];
            incoming.copy_from_slice(chunk);
            chunk.copy_from_slice(&value.to_bits().to_le_bytes());
            *value = f32::from_bits(u32::from_le_bytes(incoming));
        }
        true
    }
}
// REALTIME_POLICY_END

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

#[cfg(test)]
mod padding_tests;
#[cfg(test)]
mod rest_tests;
