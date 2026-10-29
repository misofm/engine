//! Fixed two-band Linkwitz-Riley multiband compressor.
//!
//! One body, every width. The scalar product and the four- and eight-lane homogeneous banks are
//! the same generic [`Lane`] code instantiated at `WIDTH = 1`, 4 and 8, so agreement between them
//! is a property of the code and not of a tolerance (master plan for issue #83, D5).
//!
//! # Signal path
//!
//! Per channel, per track:
//!
//! ```text
//! (v1, lp1) = svf(x)      ap  = x - 2k*v1        <- one TPT state-variable stage, k = sqrt(2)
//! (_,  low) = svf(lp1)    high = ap - low        <- the second stage
//! ```
//!
//! `low + high` is `ap`, the second-order Butterworth all-pass, because `LP2^2 + HP2^2` is
//! `D(-s)/D(s)` exactly. That is why the crossover is **two** sections and not four: the audit of
//! this crate (#94 F4) found sections 0 and 2 of the old four-section form carrying bit-identical
//! state, and `tests/lr4_two_section_mapping_f64.rs` pins the identity in `f64` against the
//! independent four-section oracle and against the closed-form all-pass.
//!
//! Each band rides the current sample through a Giannoulis-Massberg-Reiss static curve with a
//! fixed 6 dB knee and a branching smoother, and the two gained bands are summed.
//!
//! # What this crate does *not* contain
//!
//! Parameter ramps, the state-payload codec, the gain computer, the dB conversions, the envelope
//! smoother, the detector link and the block boundary check all live in `effect-runtime`
//! and `lane`. The audit found every one of them copied here, and the copies had
//! already diverged (#94 F6). What is left is this effect's equations, its parameter table and its
//! state layout.
//!
//! # Realtime rules
//!
//! No allocation, no locks, no platform transcendentals and no per-value `is_finite` on any render
//! path. Denormals are handled by `flush` on the six recursive state words, and non-finite output
//! is caught once per block by `effect_runtime::bank` (D7).

use effect_contract::{
    AutomationRate, BankProcessReport, BankWidth, EffectBankProcessBlock, EffectDescriptor,
    EffectPrepareError, EffectProcessBlock, EffectQuality, InitialParameterValue, LatencySamples,
    LinkMode, LinkModeSet, NativeEffectFactory, ObservationCadence, ObservationChannels,
    ObservationCost, ObservationDescriptor, ObservationFold, ObservationKind, ObservationSample,
    ObservationTapId, ParameterChannel, ParameterChannelPolicy, ParameterDescriptor,
    ParameterDomain, ParameterId, ParameterMapping, ParameterUnit, PortDescriptor, PortId,
    PortLayout, PortRole, PrepareEffectBankRequest, PrepareEffectRequest, PreparedAutomationSpan,
    PreparedBankMetadata, PreparedEffect, PreparedEffectBank, PreparedEffectMetadata,
    PreparedNativeEffect, PreparedNativeEffectBank, ProcessReport, ResetKind, SmoothingRule,
    StatePayloadError, StatePayloadInput, StatePayloadOutput, StatePayloadSizes,
    expected_prepared_metadata,
};
use effect_runtime::bank::{self, NonFiniteReport, block_is_positive_zero, positive_zero_lanes};
use effect_runtime::dynamics::{GainComputerCoef, gain_delta_db, knee_coefficients};
use effect_runtime::envelope::retention_coefficient;
use effect_runtime::params::{ParameterSpec, normalize_zero, parameter_value_valid};
use effect_runtime::ramp::LinearRamp;
use effect_runtime::state_payload::{
    STATE_LENGTH_CODE, STATE_VERSION_CODE, ramp_path_inside, read_f32, read_ramp, read_u32,
    write_f32, write_ramp, write_u32,
};
use effect_runtime::svf::{RAMP_PATH_NORM_TOLERANCE, transition_norm};
use lane::kernels::{
    SvfState, ramp_toward, silence_advance, silence_skip_block, svf_state_held, svf_step_when,
};
use lane::{Lane, flush};
use lane::{silence_armable_holding, silence_step};
use math::fast_db::{fast_gain_from_db, fast_level_db};

pub mod corpus;
#[cfg(test)]
mod glide_tests;
#[cfg(test)]
mod padding_tests;
#[cfg(test)]
mod rest_tests;
mod shim;
#[cfg(test)]
mod split;
pub mod tail;

use shim::{
    GlideWeights, LINK_AVERAGE, LINK_DUAL_MONO, LINK_MAXIMUM, LINK_MIXED, LINK_WEIGHTS, LinkLanes,
    branching_smooth, glide_levels, link_levels,
};

/// Parameters in the frozen V1 order.
const PARAMETER_COUNT: usize = 11;

/// Parameter ramps: every parameter except the crossover, which ramps its three designed words
/// instead ([`CROSSOVER_WORDS`], issue #1338).
const RAMP_COUNT: usize = 10;

/// State-payload words each ramp occupies: current, target, step, remaining.
const RAMP_WORDS: usize = 4;

/// Fixed scalar words of one channel's state payload: 48 before the coefficient ramps, four
/// four-word coefficient ramps after them (issue #1069), three four-word crossover word ramps
/// after those (issue #1338), and the two four-word detector link glide weights last (issue
/// #1367 D4): 84. The lane's link mode is the weights' targets
/// (`effect_contract::link_glide_mode`), so it takes no word of its own.
const LANE_HEADER_WORDS: usize = LINK_WORD + LINK_WEIGHTS * RAMP_WORDS;

/// State layout version. This is the sole prelaunch layout identity; the payload shape incorporates
/// the audit's F1, F4 and D11 corrections.
const STATE_LAYOUT_VERSION: u32 = 1;

/// Index of the low band.
const LOW_BAND: usize = 0;

/// Index of the high band.
const HIGH_BAND: usize = 1;

/// Knee width of both bands, in dB. Fixed by the product (spec 018).
const KNEE_DB: f32 = 6.0;

/// The bands' knee words `(W/2, 1/(2 W))`, designed at compile time by the workspace's one knee
/// design, `effect_runtime::dynamics::knee_coefficients`, so this crate and the compressor's knee
/// parameter share its overflow rule (issue #994: a width whose `1/(2 W)` overflows is a hard
/// knee). For 6 dB they are the words this crate wrote inline before: `3.0` and `1/12` rounded
/// once, pinned by `knee_tests::the_fixed_knee_words_are_unchanged`.
const BAND_KNEE: (f32, f32) = knee_coefficients(KNEE_DB);

/// Samples a ramped parameter takes to reach its target (`SmoothingRule::Linear`, 64).
const SMOOTHING_SAMPLES: u32 = 64;

/// Coefficient ramps per channel and track: each band's attack and release retention coefficient,
/// in the order low attack, low release, high attack, high release (issue #1069).
///
/// They follow the compressor's coefficient ramps (`crates/compressor`, `rate_ramps`): a time
/// retarget designs its exact coefficient once, at the event ([`Instance::apply_automation`]), and
/// the render path walks the coefficient word linearly to it over the same 64 samples, so no
/// exponential runs per frame and the walk is a lane-wide `ramp_toward` like every other ramp.
const RATE_COUNT: usize = 4;

/// Crossover word ramps per channel and track: `c1`, `a2`, `a3` of the split (issue #1338).
///
/// A crossover span designs the target triple once, at the event ([`Instance::apply_automation`],
/// with [`design_lr4_words`]), and the three words walk linearly to it over the same 64 samples as
/// every other ramp. The kernel reads them frame by frame on a ramped segment, so where a segment
/// is cut cannot move them. Each point of the walk is a convex combination of two designs, which
/// keeps it within `effect_runtime::svf::RAMP_PATH_NORM_TOLERANCE` (#1366 D5).
const CROSSOVER_WORDS: usize = 3;

/// Ramp slots per channel and track: the ten parameter ramps, the four coefficient ramps, then
/// the three crossover word ramps.
const RAMP_SLOTS: usize = RAMP_COUNT + RATE_COUNT + CROSSOVER_WORDS;

/// The slot of coefficient ramp `rate` ([`RATE_COUNT`]).
const fn rate_slot(rate: usize) -> usize {
    RAMP_COUNT + rate
}

/// The slot of crossover word `word` ([`CROSSOVER_WORDS`]: 0 is `c1`, 1 `a2`, 2 `a3`).
const fn crossover_slot(word: usize) -> usize {
    RAMP_COUNT + RATE_COUNT + word
}

/// The parameter index whose time coefficient ramp `rate` walks: low attack 3, low release 4,
/// high attack 8, high release 9.
const fn rate_parameter(rate: usize) -> usize {
    (rate / 2) * 5 + 3 + rate % 2
}

/// The coefficient ramp that parameter ramp `index` (parameter `index + 1`) drives, if any.
const fn rate_of_ramp(index: usize) -> Option<usize> {
    match index {
        2 => Some(0),
        3 => Some(1),
        7 => Some(2),
        8 => Some(3),
        _ => None,
    }
}

/// Detector floor: the smallest amplitude a band level is measured at, `-160 dB`.
const DETECTOR_FLOOR: f32 = 1.0e-8;

/// Butterworth damping `k = 1 / Q = sqrt(2)`.
const BUTTERWORTH_K: f64 = core::f64::consts::SQRT_2;

/// `-2k`, the all-pass tap of the first stage. Rounded once; doubling and negating are exact.
const NEGATIVE_TWO_K: f32 = -2.0 * (core::f64::consts::SQRT_2 as f32);

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

/// Frozen parameter order and stable numeric IDs for the V1 two-band product.
pub const MULTIBAND_COMPRESSOR_PARAMETERS: [ParameterDescriptor; PARAMETER_COUNT] = [
    // Issue #1338: the crossover is live. A span designs the target words once and the three
    // word ramps walk to them over the same 64 samples as every other row.
    parameter(
        1,
        "crossover_hz",
        "crossover",
        "Hz",
        ParameterUnit::Hz,
        80.0,
        8_000.0,
        1_000.0,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        3,
        "low_threshold_db",
        "low_threshold",
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
        4,
        "low_ratio",
        "low_ratio",
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
        5,
        "low_attack_ms",
        "low_attack",
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
        6,
        "low_release_ms",
        "low_release",
        "ms",
        ParameterUnit::Milliseconds,
        5.0,
        5_000.0,
        100.0,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        7,
        "low_makeup_db",
        "low_makeup",
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
        8,
        "high_threshold_db",
        "high_threshold",
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
        9,
        "high_ratio",
        "high_ratio",
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
        10,
        "high_attack_ms",
        "high_attack",
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
        11,
        "high_release_ms",
        "high_release",
        "ms",
        ParameterUnit::Milliseconds,
        5.0,
        5_000.0,
        100.0,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        12,
        "high_makeup_db",
        "high_makeup",
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

/// Bytes of one channel's state payload.
const fn lane_bytes() -> u32 {
    LANE_HEADER_WORDS as u32 * 4
}

const fn quality(sample_rate: u32) -> effect_contract::QualityDescriptor {
    let bytes = lane_bytes();
    effect_contract::QualityDescriptor {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(0),
        maximum_state: StatePayloadSizes {
            // No common section. The shared codec's two-word versioned header moves `common_bytes`
            // and therefore descriptor identity, which is a coordinated change: wave-2 decision
            // W2-D2 on #83 keeps this crate's common section empty and leaves uniform adoption of
            // the header to #95. The version still arrives out of band, as the contract's
            // `state_layout_version` argument, and is checked against `STATE_LAYOUT_VERSION`.
            common_bytes: 0,
            left_bytes: bytes,
            right_bytes: bytes,
        },
        // Nothing in this crate touches scratch; the 136 bytes version 1 reserved were never used
        // (#94 F12).
        scratch_fixed_bytes: 0,
        scratch_bytes_per_frame: 0,
    }
}

/// This effect's tail, tail over every peak, exact-rest bound and composition values, the one place
/// they are stated (decision 15 D15-4(b), #1377 D1, #1373): the pinned per-rate bound of
/// [`tail::multiband_live_bound_table`], over every crossover, curve, makeup and ramp history.
/// The quality is `Normal` alone. Off the launch rates nothing is stated.
fn tail_and_rest(sample_rate: u32, _quality: EffectQuality) -> effect_contract::NodeTailBound {
    tail::multiband_live_bound_table(sample_rate)
        .unwrap_or(effect_contract::NodeTailBound::UNBOUNDED)
}

const QUALITIES: [effect_contract::QualityDescriptor; 4] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
];

/// The one declared observation tap: the **most reduced** of the two bands (issue #143 R3).
///
/// `Side::gain_db` is one smoother word per band, in decibels and negative for reduction, so the
/// minimum of the two is the largest reduction the channel is applying. One aggregate tap ships in
/// V1 because the meter frame carries one slot per track; per-band taps are an additive follow-up
/// once per-tap frame slots exist, and they need no wire, contract or transport change to arrive.
pub const MULTIBAND_COMPRESSOR_OBSERVATIONS: [ObservationDescriptor; 1] = [ObservationDescriptor {
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

/// Immutable descriptor for the launch two-band multiband compressor.
pub const MULTIBAND_COMPRESSOR_DESCRIPTOR: EffectDescriptor = EffectDescriptor {
    id: effect_id("miso.multiband-compressor"),
    display_name: "Multiband Compressor",
    contract_major: 1,
    // Issue #143 P1: declaring the first tap is a `contract_minor` bump and a derived identity
    // re-pin of exactly `32 + len("Gain Reduction") + len("dB")` = 48 bytes.
    // `state_layout_version` does not move: the tap reads state that was already there.
    contract_minor: 1,
    state_layout_version: STATE_LAYOUT_VERSION,
    supported_link_modes: LinkModeSet::ALL,
    // Issue #1367 D1: the bank holds each lane's link mode (`Instance::link_lanes`), so members
    // that differ only by link mode share one bank.
    lane_link: true,
    parameters: &MULTIBAND_COMPRESSOR_PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
    tail_and_rest,
    observations: &MULTIBAND_COMPRESSOR_OBSERVATIONS,
};

/// The domain of one parameter, in the shared runtime's vocabulary.
///
/// Derived from the frozen descriptor rather than written a second time: the mapping is a
/// control-surface concern and is not used here, so every spec is built as continuous over the
/// descriptor's own bounds.
const fn spec(index: usize) -> ParameterSpec {
    let descriptor = &MULTIBAND_COMPRESSOR_PARAMETERS[index];
    let minimum = match descriptor.minimum {
        Some(value) => value,
        None => 0.0,
    };
    let maximum = match descriptor.maximum {
        Some(value) => value,
        None => 0.0,
    };
    ParameterSpec::continuous(minimum, maximum, descriptor.default_value)
}

/// Domains of the eleven parameters, in descriptor order.
const SPECS: [ParameterSpec; PARAMETER_COUNT] = [
    spec(0),
    spec(1),
    spec(2),
    spec(3),
    spec(4),
    spec(5),
    spec(6),
    spec(7),
    spec(8),
    spec(9),
    spec(10),
];

/// `true` if `value` is finite and either zero or normal.
fn normal_or_zero(value: f32) -> bool {
    value.is_finite() && (value == 0.0 || value.is_normal())
}

// ---------------------------------------------------------------------------------------------
// Crossover
// ---------------------------------------------------------------------------------------------

/// Coefficients of the two-stage Linkwitz-Riley split, one set per lane.
///
/// Both stages are the same Butterworth-Q low-pass design, so one coefficient set serves both
/// (master plan §4.2 amendment A1 storage: `c1 = t / (1 + t)`). Public so that the two-section
/// claim of audit #94 F4 can be checked from outside the crate against an independent oracle;
/// build one with [`lr4_coefficients`].
#[derive(Clone, Copy)]
pub struct Lr4Coef<L: Lane> {
    /// `-c1`, hoisted: a sign-bit flip is exact.
    pub nc1: L,
    /// `g * (1 - c1)`.
    pub a2: L,
    /// `g * a2`.
    pub a3: L,
    /// `-2k`, the all-pass tap. Exact: doubling and negating do not round.
    pub nk2: L,
}

/// The four recursive words of the two-stage split, one set per lane.
///
/// Public for the same reason as [`Lr4Coef`].
#[derive(Clone, Copy)]
pub struct Lr4State<L: Lane> {
    /// First stage.
    pub a: SvfState<L>,
    /// Second stage, fed by the first stage's low-pass tap.
    pub b: SvfState<L>,
}

impl<L: Lane> Default for Lr4State<L> {
    fn default() -> Self {
        Self {
            a: SvfState::default(),
            b: SvfState::default(),
        }
    }
}

/// One frame of the split: returns `(low, high)`.
///
/// Frozen operation order:
/// 1. `(v1, lp1) = svf_step(x, rest)` — the first stage, both taps of one state
/// 2. `ap = fma(-2k, v1, x)` — unfused (#163 phase 2); this is `svf_block`'s all-pass mix
///    `(1, -2k, 0)`
/// 3. `(_, low) = svf_step(lp1, rest)` — the second stage
/// 4. `high = ap - low`
///
/// Step 4 is what makes the band sum the all-pass by construction rather than by accident: the
/// only rounding between `ap` and `low + high` is the one subtraction.
///
/// `rest` is the frame's rest threshold from the compressor's own input,
/// `lane::silence_step(x, run)` (issue #1328, amendment A9): both stages' joint flush is armed by
/// the effect input `x`, so the second stage, whose input is the first stage's low-pass tap, is
/// never armed by that tap.
#[inline(always)]
pub fn lr4_step<L: Lane>(x: L, rest: L, c: &Lr4Coef<L>, s: &mut Lr4State<L>) -> (L, L) {
    lr4_step_when(true, x, rest, c, s)
}

/// [`lr4_step`] for a caller that knows, per segment, whether either stage's joint flush can act
/// (issue #1455): `armable = true` is [`lr4_step`]; `armable = false` runs both stages'
/// per-word flush ([`svf_step_when`]`(false, ..)`) and does not read `rest`, which is the same
/// bits on a segment for which [`Side::arms`] is `false`.
#[inline(always)]
fn lr4_step_when<L: Lane>(
    armable: bool,
    x: L,
    rest: L,
    c: &Lr4Coef<L>,
    s: &mut Lr4State<L>,
) -> (L, L) {
    let (v1, lp1) = svf_step_when(armable, x, c.nc1, c.a2, c.a3, rest, &mut s.a);
    let ap = c.nk2.fma(v1, x);
    let (_, low) = svf_step_when(armable, lp1, c.nc1, c.a2, c.a3, rest, &mut s.b);
    (low, ap.sub(low))
}

/// Designs `(c1, a2, a3)` in `f64` for one crossover frequency, rounded once to `f32`.
///
/// `g = tan(pi fc / fs)`, `k = sqrt(2)`, `t = g (g + k)`, `c1 = t / (1 + t)`, `a2 = g (1 - c1)`,
/// `a3 = g a2`. The tangent comes from `math`, never the platform libm (D6).
///
/// This is the computation alone: no domain test, no check-back, no failure. It is infallible by
/// proof rather than by branch: `tests/designer_total.rs` shows that [`design_lr4`] accepts every
/// `f32` crossover in the legal domain at every launch rate, returns exactly these words, and that
/// each triple passes [`effect_runtime::svf::NORM_TOLERANCE`] (#1366 D4). Outside that domain the
/// words are meaningless; callers that cannot guarantee the domain call [`design_lr4`].
#[must_use]
pub fn design_lr4_words(sample_rate: u32, crossover_hz: f32) -> [f32; 3] {
    let g = math::tan(core::f64::consts::PI * f64::from(crossover_hz) / f64::from(sample_rate));
    let t = g * (g + BUTTERWORTH_K);
    let c1 = t / (1.0 + t);
    let a1 = 1.0 - c1;
    [c1 as f32, (g * a1) as f32, (g * (g * a1)) as f32]
}

/// Tests the domain, designs with [`design_lr4_words`], and checks the rounded words back.
///
/// The rounded triple is checked *back*: `g` is recovered from `c1` alone, the analytic
/// second-order low-pass magnitude at the requested crossover is evaluated from it, and it must be
/// the half-power point to 0.005 dB. That is a test of the `f32` rounding of the damping
/// coefficient — the thing amendment A1 exists for — and not a restatement of the design.
#[must_use]
pub fn design_lr4(sample_rate: u32, crossover_hz: f32) -> Option<[f32; 3]> {
    if sample_rate == 0 || !parameter_value_valid(&SPECS[0], crossover_hz) {
        return None;
    }
    let designed = design_lr4_words(sample_rate, crossover_hz);
    if !designed.into_iter().all(normal_or_zero)
        || !(0.0..1.0).contains(&designed[0])
        || designed[1] <= 0.0
        || designed[2] <= 0.0
    {
        return None;
    }
    // `t = c1 / (1 - c1)` and `t = g (g + k)`, so `g` is the positive root of `g^2 + kg - t`.
    let realized_t = f64::from(designed[0]) / (1.0 - f64::from(designed[0]));
    let realized_g =
        0.5 * (math::sqrt(BUTTERWORTH_K * BUTTERWORTH_K + 4.0 * realized_t) - BUTTERWORTH_K);
    if realized_g <= 0.0 {
        return None;
    }
    // `a3 / a2` is `g` too; a triple whose two routes to `g` disagree is not this filter.
    let ratio_g = f64::from(designed[2]) / f64::from(designed[1]);
    if (ratio_g - realized_g).abs() > 1.0e-4 * realized_g {
        return None;
    }
    // Analytic |LP2| at the crossover: `1 / |D(j*t_probe)|` with `D(s) = s^2 + k s + 1`.
    let probe = math::tan(core::f64::consts::PI * f64::from(crossover_hz) / f64::from(sample_rate))
        / realized_g;
    let real = 1.0 - probe * probe;
    let imaginary = BUTTERWORTH_K * probe;
    let magnitude = math::sqrt(real * real + imaginary * imaginary);
    if magnitude <= 0.0 || !magnitude.is_finite() {
        return None;
    }
    let magnitude_db = -20.0 * math::log10(magnitude);
    if (magnitude_db + 3.010_299_956_6).abs() > 0.005 {
        return None;
    }
    Some(designed)
}

/// Designs one lane-wide coefficient set for a crossover at `crossover_hz`.
///
/// Every lane gets the same design. Returns `None` outside the frozen 80 Hz to 8 kHz domain, at a
/// zero sample rate, or if the `f32` rounding of the damping coefficient fails the design's own
/// half-power self-check.
#[must_use]
pub fn lr4_coefficients<L: Lane>(sample_rate: u32, crossover_hz: f32) -> Option<Lr4Coef<L>> {
    let designed = design_lr4(sample_rate, crossover_hz)?;
    Some(Lr4Coef {
        nc1: L::splat(designed[0]).neg(),
        a2: L::splat(designed[1]),
        a3: L::splat(designed[2]),
        nk2: L::splat(NEGATIVE_TWO_K),
    })
}

// ---------------------------------------------------------------------------------------------
// Instance
// ---------------------------------------------------------------------------------------------

/// One band's lane coefficients for one frame: `1/R - 1` and the two retention coefficients.
///
/// [`frame_coefficients`] derives them from the ramp words of the same frame, so a lane's
/// coefficients depend on its own ramps alone, never on where a segment or a block begins
/// (issue #1069).
#[derive(Clone, Copy)]
struct BandCoef<L: Lane> {
    inv_ratio_minus_one: L,
    attack: L,
    release: L,
}

/// The seventeen ramps of one channel ([`RAMP_SLOTS`]), as lanes, for the duration of one segment.
///
/// `target` rides beside `step` so each frame's update holds the word inside its endpoints
/// (issue #1409 D4); it is transient, never state.
#[derive(Clone, Copy)]
struct Segment<L: Lane> {
    current: [L; RAMP_SLOTS],
    step: [L; RAMP_SLOTS],
    target: [L; RAMP_SLOTS],
}

/// Both bands' coefficients from one frame's ramp words: `1/R - 1` lane-wide from the ratio word
/// (the operations `GainComputerCoef::new` runs, as the compressor's `design_curve` does), and the
/// attack and release coefficient words as they stand.
#[inline(always)]
fn frame_coefficients<L: Lane>(segment: &Segment<L>) -> [BandCoef<L>; 2] {
    let one = L::splat(1.0);
    core::array::from_fn(|band| BandCoef {
        inv_ratio_minus_one: one.div(segment.current[band * 5 + 1]).sub(one),
        attack: segment.current[rate_slot(band * 2)],
        release: segment.current[rate_slot(band * 2 + 1)],
    })
}

/// The split's lane coefficients from one frame's crossover words (issue #1338): `-c1`, `a2` and
/// `a3` as the three word ramps stand, and the constant all-pass tap.
///
/// On a flat segment the words are the settled designs, so this is the splat of the per-track
/// designs bit for bit (the negation is exact); on a ramped one it is derived again every frame,
/// after the frame's step, exactly as [`frame_coefficients`] is.
#[inline(always)]
fn crossover_coefficients<L: Lane>(segment: &Segment<L>) -> Lr4Coef<L> {
    Lr4Coef {
        nc1: segment.current[crossover_slot(0)].neg(),
        a2: segment.current[crossover_slot(1)],
        a3: segment.current[crossover_slot(2)],
        nk2: L::splat(NEGATIVE_TWO_K),
    }
}

/// How far the next segment reaches, and whether anything is ramping over it.
///
/// The two answers are one decision because they come from the same scan of the same counters, so
/// they are produced together rather than scanned for twice ([`Instance::plan_segment`]).
///
/// `ramping` is a **whole-bank** predicate: every track of both channels, all seventeen ramps. The
/// segment kernel is lane-wide and a per-lane branch inside its frame loop is precisely what the
/// bank contract forbids, so one moving track puts the whole bank on the ramped path for that
/// segment. That is the granularity the rest of the library already settled on — the compressor's
/// `max_remaining` is taken across both channels and every lane, the true-peak limiter's
/// stationary predicate is an `all` over both channels' ramp arrays, and the 2x2 matrix's early
/// return is on the maximum over the bank.
#[derive(Clone, Copy)]
struct SegmentPlan {
    /// Frames the segment covers. Always at least one.
    frames: usize,
    /// `true` while any ramp in the bank still has samples to produce, a link glide weight
    /// included (issue #1367).
    ramping: bool,
    /// `true` while a link glide weight of some lane is in flight over the segment (issue #1367
    /// D3): the segment takes [`run_segment`]'s `GLIDE` body.
    gliding: bool,
}

/// Everything one channel of one bank owns.
#[cfg_attr(test, derive(Clone))]
struct Side<L: Lane, const W: usize> {
    filter: Lr4State<L>,
    /// The run of exactly-zero input frames per lane: this channel's silence counter, which arms
    /// both crossover stages' joint flush (issue #1328, amendment A9). Kept where a frame-by-frame
    /// count over the crossover's frames would leave it: frame by frame on a segment where a side
    /// can arm, once per segment on any other (issue #1455, [`silence_skip_block`]); a bypassed
    /// instance runs no crossover and freezes it with the filter.
    silence: L,
    /// The branching smoother's state, in dB, per band.
    gain_db: [L; 2],
    /// Per track: the ten parameter ramps, the four coefficient ramps, then the three crossover
    /// word ramps ([`RAMP_SLOTS`]). The crossover ramps' targets are the design of `crossover_hz`.
    ramps: [[LinearRamp; RAMP_SLOTS]; W],
    /// The coefficient words a full reset restores, designed once at preparation.
    prepared_rates: [[f32; RATE_COUNT]; W],
    /// The prepared `(c1, a2, a3)` per track, kept so a reset does not have to redesign (#94 F9).
    prepared_designed: [[f32; CROSSOVER_WORDS]; W],
    /// The crossover the word ramps walk to, per track: the target of the last accepted span.
    crossover_hz: [f32; W],
    defaults: [[f32; PARAMETER_COUNT]; W],
    /// Per track, the detector link glide weights `w_link` then `w_avg` (issue #1367 D3, #1370
    /// D2): at rest each is exactly `effect_contract::link_glide_weights` of the lane's mode
    /// ([`Instance::links`]). Only a segment with a glide in flight reads them
    /// ([`run_segment`]'s `GLIDE`), which advances them frame by frame; the segment split
    /// ([`Instance::plan_segment`]) snaps and scans them with the other ramps.
    links: [[LinearRamp; LINK_WEIGHTS]; W],
}

impl<L: Lane, const W: usize> Side<L, W> {
    fn new(defaults: [[f32; PARAMETER_COUNT]; W], sample_rate: u32) -> Option<Self> {
        let mut designed = [[0.0; CROSSOVER_WORDS]; W];
        let mut crossover_hz = [0.0; W];
        let mut ramps = [[LinearRamp::fixed(0.0); RAMP_SLOTS]; W];
        let mut prepared_rates = [[0.0; RATE_COUNT]; W];
        for track in 0..W {
            designed[track] = design_lr4(sample_rate, defaults[track][0])?;
            crossover_hz[track] = defaults[track][0];
            for index in 0..RAMP_COUNT {
                ramps[track][index] = LinearRamp::fixed(defaults[track][index + 1]);
            }
            for rate in 0..RATE_COUNT {
                prepared_rates[track][rate] =
                    retention_coefficient(defaults[track][rate_parameter(rate)], sample_rate);
                ramps[track][rate_slot(rate)] = LinearRamp::fixed(prepared_rates[track][rate]);
            }
            for (word, value) in designed[track].into_iter().enumerate() {
                ramps[track][crossover_slot(word)] = LinearRamp::fixed(value);
            }
        }
        Some(Self {
            filter: Lr4State::default(),
            silence: L::zero(),
            gain_db: [L::zero(); 2],
            ramps,
            prepared_rates,
            prepared_designed: designed,
            crossover_hz,
            defaults,
            links: [[LinearRamp::fixed(0.0); LINK_WEIGHTS]; W],
        })
    }

    /// `true` when this side's crossover needs its armed form over a segment of `frames`
    /// (issue #1455): some lane whose silence counter can reach `armed_after` within the segment
    /// holds a recursive word other than `±0.0` in either stage, or the segment is longer than
    /// `armed_after` (`lane::silence_armable_holding`, the builtin input chain's rule over the same
    /// two-section cascade). `false` proves the unarmed form gives the same bits: a lane at rest
    /// in both stages that sees only zero input keeps both stages at `±0.0`, because the second
    /// stage's input is the first stage's low-pass tap.
    #[inline(always)]
    fn arms(&self, frames: usize, armed_after: L) -> bool {
        silence_armable_holding(
            self.silence,
            frames,
            armed_after,
            svf_state_held([self.filter.a, self.filter.b]),
        )
    }

    /// The lanes of this channel at exact rest (issue #893, the contract's
    /// `PreparedNativeEffectBank::lanes_at_rest`): the four crossover words and both bands'
    /// smoother words exactly `+0.0`, and every ramp of the lane's track settled.
    ///
    /// The list is every word a block carries forward but the silence counter, which is a clock:
    /// on `+0.0` input it advances to `min(run + frames, 2^24)` whatever the signal was
    /// ([`silence_step`] and [`silence_skip_block`] agree there). Why each pinned word is a fixed
    /// point of one `+0.0` block that writes `+0.0`:
    ///
    /// * **Crossover.** Both stages' `ic1`/`ic2` at `+0.0` with `+0.0` input give `v1 = +0.0` and
    ///   `+0.0` taps; the all-pass is `fma(-2k, +0.0, +0.0) = -0.0 + +0.0 = +0.0`, and
    ///   `high = +0.0 - +0.0 = +0.0`. Armed or not, the flush keeps `+0.0`.
    /// * **Smoother.** Each band's detector floors at -160 dB, below every threshold, so the
    ///   target is `+0.0`; the branching smoother from `+0.0` to `+0.0` stays `+0.0`, and
    ///   `flush(+0.0) = +0.0`. The applied gain is finite, so each band writes `+0.0 * g = +0.0`.
    /// * **Ramps.** `remaining == 0` with a `+0.0` step is the flat path, which moves no ramp word
    ///   ([`Instance::flat_sides_are_identity`]); a ramp in flight clears its track's bit. The
    ///   link glide weights count as ramps (issue #1367): a glide in flight moves its detector
    ///   every frame, so its lane is never at rest and a silent bank never skips it.
    ///
    /// A plain `flush` brings each smoother word to `+0.0` (the compressor's rest snap is not
    /// adopted here: that would move rendered bits). The crossover and coefficient words are ramps
    /// (issue #1338), so the settled-ramp rule covers them.
    /// One vector pass per word answers every lane.
    fn rest_lanes(&self) -> u32 {
        let mut mask = positive_zero_lanes(self.filter.a.ic1)
            & positive_zero_lanes(self.filter.a.ic2)
            & positive_zero_lanes(self.filter.b.ic1)
            & positive_zero_lanes(self.filter.b.ic2)
            & positive_zero_lanes(self.gain_db[LOW_BAND])
            & positive_zero_lanes(self.gain_db[HIGH_BAND]);
        for (track, (ramps, links)) in self.ramps.iter().zip(&self.links).enumerate() {
            if ramps
                .iter()
                .chain(links)
                .any(|ramp| ramp.remaining != 0 || ramp.step.to_bits() != 0)
            {
                mask &= !(1 << track);
            }
        }
        mask
    }

    /// `true` when no ramp of any track of this channel has samples left to produce, the link
    /// glide weights included (issue #1367).
    fn no_ramp_in_flight(&self) -> bool {
        self.ramps
            .iter()
            .flatten()
            .chain(self.links.iter().flatten())
            .all(|ramp| ramp.remaining == 0)
    }

    /// `true` while a link glide weight of any track of this channel is in flight (issue #1367).
    fn gliding(&self) -> bool {
        self.links.iter().flatten().any(LinearRamp::is_ramping)
    }

    /// The link glide weights of every track, as lanes, for one segment (issue #1367 D3).
    fn glide_weights(&self) -> GlideWeights<L> {
        let mut weights = GlideWeights::idle();
        let mut gliding = 0u32;
        for weight in 0..LINK_WEIGHTS {
            let mut values = [0.0f32; 8];
            let mut steps = [0.0f32; 8];
            let mut targets = [0.0f32; 8];
            for track in 0..W {
                let ramp = self.links[track][weight];
                values[track] = ramp.current;
                steps[track] = ramp.step;
                targets[track] = ramp.target;
                if ramp.remaining != 0 {
                    gliding |= 1 << track;
                }
            }
            weights.current[weight] = L::load(&values[..W]);
            weights.step[weight] = L::load(&steps[..W]);
            weights.target[weight] = L::load(&targets[..W]);
        }
        weights.gliding = lanes_mask::<L>(gliding);
        weights
    }

    /// Writes a finished segment's link glide weights back, as
    /// [`store_segment`](Self::store_segment) does the other ramps: no weight arrives inside a
    /// segment, so `remaining` stays positive on a gliding lane and zero on a resting one.
    fn store_glide(&mut self, weights: &GlideWeights<L>, advanced: u32) {
        for weight in 0..LINK_WEIGHTS {
            let mut values = [0.0f32; 8];
            weights.current[weight].store(&mut values[..W]);
            for (track, value) in values.iter().enumerate().take(W) {
                let ramp = &mut self.links[track][weight];
                ramp.current = *value;
                ramp.remaining = ramp.remaining.saturating_sub(advanced);
            }
        }
    }

    // REALTIME_POLICY_BEGIN
    /// [`discontinuity_reset`](Self::discontinuity_reset) on lane `track` alone (issue #1340, D1):
    /// that lane's four crossover words, both smoother words and its silence clock go to `+0.0`
    /// and its track's ramps snap, which leaves every ramp's step `+0.0`, so the lane can rest
    /// ([`rest_lanes`](Self::rest_lanes)). Every other lane's words are unchanged. This is the D7
    /// recovery, and it leaves a link glide in flight alone (issue #1367, #1370's rule).
    fn discontinuity_reset_lane(&mut self, track: usize) {
        for stage in [&mut self.filter.a, &mut self.filter.b] {
            stage.ic1 = set_lane_value(stage.ic1, track, 0.0);
            stage.ic2 = set_lane_value(stage.ic2, track, 0.0);
        }
        self.silence = set_lane_value(self.silence, track, 0.0);
        for gain in &mut self.gain_db {
            *gain = set_lane_value(*gain, track, 0.0);
        }
        for ramp in &mut self.ramps[track] {
            ramp.snap();
        }
    }
    // REALTIME_POLICY_END

    /// Clears history. Coefficients and parameters are deliberately untouched. A link glide in
    /// flight snaps to its target, the lane's mode (issue #1367, #1370's rule).
    fn discontinuity_reset(&mut self) {
        self.filter = Lr4State::default();
        self.silence = L::zero();
        self.gain_db = [L::zero(); 2];
        for track in 0..W {
            for ramp in self.ramps[track].iter_mut().chain(&mut self.links[track]) {
                ramp.snap();
            }
        }
    }

    /// Returns to the prepared defaults without allocating or redesigning. The link weights are
    /// the instance's to re-seat at each lane's mode ([`Instance::reset`]).
    fn full_reset(&mut self) {
        self.discontinuity_reset();
        for track in 0..W {
            self.seed_lane_defaults(track);
        }
    }

    /// Lane `track`'s crossover target and every ramp of its track at the prepared values: the
    /// per-lane half of [`full_reset`](Self::full_reset), which [`reset_lane`](Self::reset_lane)
    /// shares (issue #2131).
    fn seed_lane_defaults(&mut self, track: usize) {
        self.crossover_hz[track] = self.defaults[track][0];
        for index in 0..RAMP_COUNT {
            self.ramps[track][index] = LinearRamp::fixed(self.defaults[track][index + 1]);
        }
        for rate in 0..RATE_COUNT {
            self.ramps[track][rate_slot(rate)] =
                LinearRamp::fixed(self.prepared_rates[track][rate]);
        }
        for word in 0..CROSSOVER_WORDS {
            self.ramps[track][crossover_slot(word)] =
                LinearRamp::fixed(self.prepared_designed[track][word]);
        }
    }

    // REALTIME_POLICY_BEGIN
    /// The whole-side reset of `kind` on lane `track` alone (issue #2131): the D7 recovery's
    /// words ([`discontinuity_reset_lane`](Self::discontinuity_reset_lane)), then the lane's link
    /// glide snapped to its target as [`discontinuity_reset`](Self::discontinuity_reset) snaps
    /// it, and for `FullToDefaults` the lane's prepared values
    /// ([`seed_lane_defaults`](Self::seed_lane_defaults)). The link weights at rest are the
    /// instance's to re-seat, as for the whole reset. Every other lane's words are unchanged.
    fn reset_lane(&mut self, track: usize, kind: ResetKind) {
        self.discontinuity_reset_lane(track);
        for ramp in &mut self.links[track] {
            ramp.snap();
        }
        if kind == ResetKind::FullToDefaults {
            self.seed_lane_defaults(track);
        }
    }
    // REALTIME_POLICY_END
}

/// One prepared bank of `W` tracks: the whole effect, at one width.
#[cfg_attr(test, derive(Clone))]
struct Instance<L: Lane, const W: usize> {
    sample_rate: u32,
    bypass: bool,
    /// Each lane's detector link mode (issue #1367): the prepared one, then the target of each
    /// accepted [`retarget_link`](Self::retarget_link). A full reset keeps it.
    links: [LinkMode; W],
    /// [`links`](Self::links) as masks and the arm they pick (issue #1367 D1), read once per block
    /// by [`render_body`].
    link_lanes: LinkLanes<L>,
    /// The active lanes (issue #1893): bit `l` is set when lane `l` carries a member. A padded
    /// lane (a clear bit) renders like any lane, a clone fed `+0.0`, but is never charged or
    /// reported (decision 12's padding contract) and takes no automation.
    active: u32,
    nonfinite: NonFiniteReport,
    /// Index 0 is the left channel, index 1 the right.
    sides: [Side<L, W>; 2],
    /// Whether a collapsed block (`process_bank_mono`, past its shape guards) has run since the
    /// last disengage copy ([`desymmetrize`](Self::desymmetrize)), which alone clears it (#1894).
    /// While it is set the right side is frozen and stale, so
    /// [`lanes_at_rest`](Self::lanes_at_rest) reads the left side alone and a skipped block
    /// advances the left silence counter alone: the effect contract's collapsed-bank rule. A reset
    /// leaves it alone.
    collapsed: bool,
}

impl<L: Lane, const W: usize> Instance<L, W> {
    fn new(
        left: [[f32; PARAMETER_COUNT]; W],
        right: [[f32; PARAMETER_COUNT]; W],
        metadata: PreparedEffectMetadata,
    ) -> Option<Self> {
        debug_assert_eq!(L::WIDTH, W);
        let mut instance = Self {
            sample_rate: metadata.sample_rate,
            bypass: metadata.bypass,
            links: [metadata.link_mode; W],
            link_lanes: LinkLanes::new(&[metadata.link_mode; W]),
            active: all_lanes::<W>(),
            nonfinite: NonFiniteReport::new(),
            sides: [
                Side::new(left, metadata.sample_rate)?,
                Side::new(right, metadata.sample_rate)?,
            ],
            collapsed: false,
        };
        instance.seat_links(&[metadata.link_mode; W]);
        Some(instance)
    }

    /// Sets every lane's link mode, its masks and the arm, and seats each lane's weights at rest
    /// at its mode (issue #1367): preparation and bind.
    fn seat_links(&mut self, links: &[LinkMode; W]) {
        self.links = *links;
        self.link_lanes = LinkLanes::new(links);
        for lane in 0..W {
            self.rest_links(lane);
        }
    }

    /// Seats lane `lane`'s link weights at rest at its mode on both channels (issue #1367, #1370
    /// D2): preparation, a full reset, a padded lane that follows a new first member and a
    /// restore whose glide is not toward the lane's mode. The D7 recovery leaves a glide alone.
    fn rest_links(&mut self, lane: usize) {
        let weights = effect_contract::link_glide_weights(self.links[lane]);
        for side in &mut self.sides {
            side.links[lane] = weights.map(LinearRamp::fixed);
        }
    }

    // REALTIME_POLICY_BEGIN: #1367, a link glide is retargeted at a block boundary.
    /// Glides lane `lane`'s detector link to `mode` over `samples` samples (issue #1367 D3,
    /// #1370 D1-D3).
    ///
    /// The lane's mode, and with it its masks and the bank's arm, becomes `mode` at once, so a
    /// glide that arrives lands on `mode`'s own law; the weights of both channels, retargeted
    /// from the words in force (a glide in flight continues from where it is), carry the
    /// transition, and `samples == 0` steps them. The mode is the one a full reset keeps.
    /// `false`, changing nothing, for a lane past the width or padded, a mode the descriptor does
    /// not support or a glide longer than `LINK_GLIDE_MAX_SAMPLES`. Allocation-free and bounded
    /// by the lane count.
    fn retarget_link(&mut self, lane: usize, mode: LinkMode, samples: u32) -> bool {
        if lane >= W
            || self.active & (1 << lane) == 0
            || !MULTIBAND_COMPRESSOR_DESCRIPTOR.lane_link
            || !MULTIBAND_COMPRESSOR_DESCRIPTOR
                .supported_link_modes
                .contains(mode)
            || samples > effect_contract::LINK_GLIDE_MAX_SAMPLES
        {
            return false;
        }
        self.set_lane_link(lane, mode);
        let weights = effect_contract::link_glide_weights(mode);
        for side in &mut self.sides {
            for (ramp, &weight) in side.links[lane].iter_mut().zip(&weights) {
                ramp.set_target(weight, samples);
            }
        }
        true
    }

    /// Sets lane `lane`'s link mode and re-derives the masks and the arm (issue #1367). Every
    /// padded lane re-takes the first active lane's mode at rest, as bind gives it, so a bank
    /// whose members became uniform leaves the mixed arm; a padded lane's output is discarded,
    /// so this moves no member's bit.
    fn set_lane_link(&mut self, lane: usize, mode: LinkMode) {
        self.links[lane] = mode;
        let first = self.active.trailing_zeros() as usize;
        if first < W {
            let mode = self.links[first];
            for padded in 0..W {
                if self.active & (1 << padded) == 0 && self.links[padded] != mode {
                    self.links[padded] = mode;
                    self.rest_links(padded);
                }
            }
        }
        self.link_lanes = LinkLanes::new(&self.links);
    }

    /// Whether a link glide is in flight on any lane of the sides a block renders: the left alone
    /// on a collapsed block, whose right side is frozen.
    fn gliding<const MONO: bool>(&self) -> bool {
        self.sides[0].gliding() || (!MONO && self.sides[1].gliding())
    }
    // REALTIME_POLICY_END

    fn reset(&mut self, kind: ResetKind) {
        for side in &mut self.sides {
            match kind {
                ResetKind::FullToDefaults => side.full_reset(),
                ResetKind::DiscontinuityKeepParameters => side.discontinuity_reset(),
            }
        }
        if kind == ResetKind::FullToDefaults {
            // Issue #1367: the lane keeps its mode, its weights at rest. An explicit invariant:
            // `discontinuity_reset`, which `full_reset` runs, already snaps them there.
            for lane in 0..W {
                self.rest_links(lane);
            }
        }
    }

    // REALTIME_POLICY_BEGIN
    /// [`reset`](Self::reset) for lane `lane` alone (issue #2131): both sides' words of that lane
    /// ([`Side::reset_lane`]), and for `FullToDefaults` its link weights at rest at its mode. A
    /// lane past the width changes nothing. The silence counters are per lane; the bank's silent
    /// claim is the caller's to withdraw.
    fn reset_lane(&mut self, lane: usize, kind: ResetKind) {
        if lane >= W {
            return;
        }
        for side in &mut self.sides {
            side.reset_lane(lane, kind);
        }
        if kind == ResetKind::FullToDefaults {
            self.rest_links(lane);
        }
    }
    // REALTIME_POLICY_END
}

// ---------------------------------------------------------------------------------------------
// Render path
// ---------------------------------------------------------------------------------------------

/// One band's amplitude for one frame: detector level, static curve, smoother, makeup.
///
/// Frozen operation order:
/// 1. `level = clamp(level_db(max(detector, 1e-8)), -160, 24)` — the `-160 dB` floor is the
///    version-1 detector floor, kept so the curve sees the same silence
/// 2. `target = clamp(gain_delta_db(level), -100, 0)` — Giannoulis et al. equation 4, 6 dB knee
/// 3. `state = flush(branching_smooth(state, target, attack, release))` — D7 applies: this is a
///    recurrence
/// 4. `gain_from_db(state + makeup)`
///
/// Every clamp is the D8 select form, so a NaN detector floors at `-160 dB` instead of
/// propagating into the smoother; a NaN that matters comes from the filter state and reaches the
/// output, where the once-per-block boundary check catches it.
#[inline(always)]
// FAST-DB-CROSSING X5 and X6's `#[expect]`: the crossing registry entries this replaces lived in
// `scripts/check-fast-db-seal.sh`, retired in favour of `clippy.toml`'s `disallowed-methods`.
#[expect(
    clippy::disallowed_methods,
    reason = "FAST-DB-CROSSING X5/X6: one band's detector level and applied gain, never pinned coefficients"
)]
fn band_amplitude<L: Lane>(
    detector: L,
    threshold: L,
    makeup: L,
    coefficients: &BandCoef<L>,
    state: &mut L,
) -> L {
    // FAST-DB-CROSSING X5: one band's detector level. Same law as the wideband compressor's X1,
    // run twice per frame per channel because there are two bands.
    let level = fast_level_db(detector.max(L::splat(DETECTOR_FLOOR)))
        .max(L::splat(-160.0))
        .min(L::splat(24.0));
    let target = band_target(
        level,
        threshold,
        coefficients.inv_ratio_minus_one,
        BAND_KNEE,
    );
    let smoothed = flush(branching_smooth(
        *state,
        target,
        coefficients.attack,
        coefficients.release,
    ));
    *state = smoothed;
    // FAST-DB-CROSSING X6: one band's applied gain. Same law as the wideband compressor's X2.
    fast_gain_from_db(smoothed.add(makeup))
}

/// Step 2 of [`band_amplitude`]: one band's target, `clamp(gain_delta_db(level), -100, 0)`.
///
/// The knee words are an argument so that the knee rule of issue #994 can be exercised at widths
/// the product does not ship; the render path passes [`BAND_KNEE`], a constant.
#[inline(always)]
fn band_target<L: Lane>(level: L, threshold: L, inv_ratio_minus_one: L, knee: (f32, f32)) -> L {
    let curve = GainComputerCoef {
        threshold_db: threshold,
        inv_ratio_minus_one,
        half_knee_db: L::splat(knee.0),
        inv_two_knee: L::splat(knee.1),
    };
    gain_delta_db(level, &curve)
        .max(L::splat(-100.0))
        .min(L::zero())
}

/// One segment: `frames` frames over which no ramp arrives at its target.
///
/// The whole render path is here. `LINK` and `BYPASS` are compile-time, because both are fixed
/// when the effect is prepared (`LINK` is the arm the lanes' modes pick, [`LINK_MIXED`] when they
/// differ, issue #1367 D2); a bypassed instance is a pure dry path and runs neither the
/// crossover nor the dynamics, and it still advances its ramps so that its parameter state does
/// not depend on whether it was bypassed.
///
/// `RAMPING` is the third compile-time switch, and it is the one that varies *within* a block. It
/// says whether any ramp of any track of either channel still has samples to produce over this
/// segment; [`Instance::plan_segment`] decides it from the ramp counters at the same moment it
/// decides the segment's length. At `RAMPING == false` the thirty-four per-frame lane additions
/// below, and the per-frame coefficient derivation ([`frame_coefficients`], one division per band
/// and channel, and [`crossover_coefficients`]), are not merely predicted away, they are not
/// emitted: a settled bank derives its coefficients once per segment and pays no smoothing
/// arithmetic at all.
///
/// **Why that is bit-identical, not merely close.** `LinearRamp` keeps `remaining == 0` implying
/// `step == +0.0`, and this crate admits parameter words through exactly four doors — prepared
/// defaults, `apply_automation`, `snap`, and a restored payload — every one of which runs
/// `normalize_zero` over a `parameter_value_valid` word; a coefficient word enters through the same
/// four, as `retention_coefficient` of a legal time or a restored word, both in `(0, 1]`. So on the
/// skipped path every lane of every `step` is `+0.0` and every lane of every `current` is finite
/// and is not `-0.0`, which makes `ramp_toward(current, step, target)` (whose add is the identity
/// there, and whose clamp keeps an in-range word's bits) the identity on all of them. The two
/// exclusions matter and are the same two `LinearRamp::stationary_at` names: `-0.0 + 0.0` is
/// `+0.0`, and a NaN is quieted by an addition. `Instance::flat_path_is_identity` asserts the
/// precondition in debug builds rather than leaving it as a comment.
///
/// `ARMABLE` is the fourth (issue #1455), chosen per segment by [`process_block`]. At `true` the
/// crossover is the armed form: each side's silence counter advances frame by frame
/// ([`silence_step`]) and both stages run the joint flush on its rest threshold. At `false` the
/// frame loop runs no counter, and both stages run the per-word flush ([`lr4_step_when`]); the
/// caller has already advanced each counter over the segment with [`silence_skip_block`], which
/// leaves it where the frame loop would. The two forms give the same bits on a segment for which
/// neither side [`Side::arms`]: there every lane's threshold is `+0.0`, or the lane holds no state
/// for the joint term to zero (`lane::silence_armable_holding`), and `flush_pair(n1, n2, +0.0)` is
/// `(flush(n1), flush(n2))`. A bypassed segment runs neither; [`process_block`] gives it
/// `ARMABLE = true` as a compile-time constant, so its unarmed arm is dead code.
///
/// `MONO` (issue #1871) renders a collapsed track: `left` holds both channels and `right` is the
/// ungathered scratch, never read or written. Only the near side's state and `segments[0]` move;
/// the far side and `segments[1]` are left as they are (the automation drain still retargets the
/// far side's ramps from their stale `current` while collapsed, and the disengage copy overwrites
/// them, so no bit moves). Every cross-channel term is the dual one
/// with the near word read twice: `link_levels::<LINK>(own, own)` is `|p|`, `max(|p|, |p|)` or
/// `0.5|p| + 0.5|p|` in the dual order, so the near output and state are the dual block's bits on
/// `L == R`, in every link mode; the mixed arm selects among those same words per lane, so a
/// collapsed lane's link partner is its own `L == R` pair at its own mode. A const parameter and
/// never a runtime flag, so the dual body keeps its own symbol (the roster's M2 lesson).
///
/// `GLIDE` (issue #1367 D3) runs a detector link glide in flight: each frame advances both
/// channels' weights (`glide`, the near side's alone when `MONO`) beside the other ramps, and the
/// band levels are [`glide_levels`]'. Only a `RAMPING` segment over which some weight is in flight
/// takes it, and [`process_block`] gives it [`LINK_MIXED`] as `LINK`; every other segment is the
/// body it was before.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn run_segment<
    L: Lane,
    const W: usize,
    const LINK: u8,
    const BYPASS: bool,
    const RAMPING: bool,
    const ARMABLE: bool,
    const MONO: bool,
    const GLIDE: bool,
>(
    sides: &mut [Side<L, W>; 2],
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    segments: &mut [Segment<L>; 2],
    glide: &mut [GlideWeights<L>; 2],
    armed_after: L,
    link_lanes: &LinkLanes<L>,
) {
    const { assert!(!GLIDE || RAMPING, "a glide rides the ramped path") };
    let (head, tail) = sides.split_at_mut(1);
    let near = &mut head[0];
    let far = &mut tail[0];
    let mut filter_near = near.filter;
    let mut filter_far = far.filter;
    let mut silence_near = near.silence;
    let mut silence_far = far.silence;
    let mut gain_near = near.gain_db;
    let mut gain_far = far.gain_db;
    // On a flat segment no word moves, so the coefficients are derived once, here; on a ramped
    // one they are derived again every frame from the frame's own advanced words below.
    // A collapsed segment derives the near side's words only; the far copies are never read.
    let sides_moving = if MONO { 1 } else { 2 };
    let mut coefficients = [frame_coefficients(&segments[0]); 2];
    let mut split = [crossover_coefficients(&segments[0]); 2];
    if !MONO {
        coefficients[1] = frame_coefficients(&segments[1]);
        split[1] = crossover_coefficients(&segments[1]);
    }
    for frame in 0..frames {
        if RAMPING {
            for index in 0..RAMP_SLOTS {
                // Issue #1409: the step added, held inside `[min(current, target),
                // max(current, target)]`, so no word passes its target.
                for segment in segments.iter_mut().take(sides_moving) {
                    segment.current[index] = ramp_toward(
                        segment.current[index],
                        segment.step[index],
                        segment.target[index],
                    );
                }
            }
            // Issue #1069: the ratio and the two time coefficients take the value of this frame,
            // after its step, the same convention as threshold and makeup.
            coefficients[0] = frame_coefficients(&segments[0]);
            // Issue #1338: the split's words likewise, never cached at the segment's start.
            split[0] = crossover_coefficients(&segments[0]);
            if !MONO {
                coefficients[1] = frame_coefficients(&segments[1]);
                split[1] = crossover_coefficients(&segments[1]);
            }
        }
        if GLIDE {
            // Issue #1367 D3: the weights take the value of this frame, after its step, like
            // every other ramped word.
            for weights in glide.iter_mut().take(sides_moving) {
                weights.advance();
            }
        }
        let input_near = L::load(&left[frame * W..]);
        if BYPASS {
            input_near.store(&mut left[frame * W..]);
            if !MONO {
                L::load(&right[frame * W..]).store(&mut right[frame * W..]);
            }
            continue;
        }
        let rest_near = if ARMABLE {
            silence_step(input_near, &mut silence_near, armed_after)
        } else {
            L::zero()
        };
        let (low_near, high_near) =
            lr4_step_when(ARMABLE, input_near, rest_near, &split[0], &mut filter_near);
        let (low_far, high_far) = if MONO {
            (low_near, high_near)
        } else {
            let input_far = L::load(&right[frame * W..]);
            let rest_far = if ARMABLE {
                silence_step(input_far, &mut silence_far, armed_after)
            } else {
                L::zero()
            };
            lr4_step_when(ARMABLE, input_far, rest_far, &split[1], &mut filter_far)
        };
        let ((linked_near_low, linked_far_low), (linked_near_high, linked_far_high)) = if GLIDE {
            // A collapsed lane's far weights are its near ones, as its far band words are.
            let far = &glide[usize::from(!MONO)];
            (
                glide_levels(low_near, low_far, link_lanes, &glide[0], far),
                glide_levels(high_near, high_far, link_lanes, &glide[0], far),
            )
        } else {
            (
                link_levels::<L, LINK>(low_near, low_far, link_lanes),
                link_levels::<L, LINK>(high_near, high_far, link_lanes),
            )
        };

        let amplitude_near_low = band_amplitude(
            linked_near_low,
            segments[0].current[0],
            segments[0].current[4],
            &coefficients[0][LOW_BAND],
            &mut gain_near[LOW_BAND],
        );
        let amplitude_near_high = band_amplitude(
            linked_near_high,
            segments[0].current[5],
            segments[0].current[9],
            &coefficients[0][HIGH_BAND],
            &mut gain_near[HIGH_BAND],
        );
        let output_near = low_near
            .mul(amplitude_near_low)
            .add(high_near.mul(amplitude_near_high));
        output_near.store(&mut left[frame * W..]);
        if MONO {
            continue;
        }
        let amplitude_far_low = band_amplitude(
            linked_far_low,
            segments[1].current[0],
            segments[1].current[4],
            &coefficients[1][LOW_BAND],
            &mut gain_far[LOW_BAND],
        );
        let amplitude_far_high = band_amplitude(
            linked_far_high,
            segments[1].current[5],
            segments[1].current[9],
            &coefficients[1][HIGH_BAND],
            &mut gain_far[HIGH_BAND],
        );

        let output_far = low_far
            .mul(amplitude_far_low)
            .add(high_far.mul(amplitude_far_high));
        output_far.store(&mut right[frame * W..]);
    }
    near.filter = filter_near;
    near.silence = silence_near;
    near.gain_db = gain_near;
    if !MONO {
        far.filter = filter_far;
        far.silence = silence_far;
        far.gain_db = gain_far;
    }
}

/// Splits the block at ramp arrivals and runs each segment, ramped or flat.
///
/// `FORCE_RAMPING` is the split's A/B switch. It is `false` on every production path; the crate's
/// own bit-identity test sets it to reproduce exactly what this function did before the split
/// existed, so "split on versus split off" is a real comparison of two rendered buffers and two
/// state snapshots rather than an argument.
///
/// `MONO` (issue #1871) plans, arms, counts silence and writes ramps back on the left side only:
/// on a collapsed track the right side is the left one by the witness's induction, so the left
/// side's plan is the dual plan, and the right side stays exactly as the collapse found it but for
/// the automation drain, which still retargets its ramps from their stale `current`; the
/// disengage copy overwrites them, so no bit moves.
///
/// `GLIDE` (issue #1367 D3) is set on a block that starts with a link glide in flight
/// ([`render_body`] then passes [`LINK_MIXED`], or the bypass arm): each ramped segment over which
/// a weight is in flight runs [`run_segment`]'s `GLIDE` body, and every other segment the body it
/// runs without one. At `false` no segment glides, because no weight is in flight.
#[inline(always)]
fn process_block<
    L: Lane,
    const W: usize,
    const LINK: u8,
    const BYPASS: bool,
    const FORCE_RAMPING: bool,
    const MONO: bool,
    const GLIDE: bool,
>(
    instance: &mut Instance<L, W>,
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
) {
    let sample_rate = instance.sample_rate;
    // `N_SILENCE` at the instance's rate (issue #1328, amendment A9).
    let armed_after = L::splat(lane::silence_frames(sample_rate) as f32);
    let link_lanes = instance.link_lanes;
    let mut position = 0;
    while position < frames {
        let plan = instance.plan_segment::<MONO>(frames - position);
        let length = plan.frames;
        let near = instance.sides[0].segment();
        let mut segments = [
            near,
            if MONO {
                near
            } else {
                instance.sides[1].segment()
            },
        ];
        // A bypassed instance runs no crossover and freezes the silence counters with the filter
        // (issue #1328, amendment A9). Otherwise the joint flush's arithmetic runs only on a
        // segment it can act on (issue #1455, [`Side::arms`]); on every other segment each side's
        // counter advances once over the segment's input, before the frame loop overwrites it,
        // and the frame loop runs the unarmed form.
        let armable = BYPASS
            || instance.sides[0].arms(length, armed_after)
            || (!MONO && instance.sides[1].arms(length, armed_after));
        if !armable {
            silence_skip_block(
                &left[position * W..(position + length) * W],
                length,
                &mut instance.sides[0].silence,
            );
            if !MONO {
                silence_skip_block(
                    &right[position * W..(position + length) * W],
                    length,
                    &mut instance.sides[1].silence,
                );
            }
        }
        debug_assert!(GLIDE || !plan.gliding, "a glide outside a glide block");
        // Issue #1367 D3: a glide block renders every segment through the one glide body, the
        // segments after an arrival included: there the weights rest (no lane gliding, every
        // step `+0.0`) and the other ramps take the ramped path, both the identity on words at
        // rest (`flat_sides_are_identity`), so its bits are the flat body's on the mixed arm. One
        // body per block shape keeps the glide from multiplying the segment bodies.
        let mut glide = if GLIDE {
            let near = instance.sides[0].glide_weights();
            [
                near,
                if MONO {
                    near
                } else {
                    instance.sides[1].glide_weights()
                },
            ]
        } else {
            [GlideWeights::idle(); 2]
        };
        macro_rules! segment {
            ($ramping:literal, $armable:literal, $glide:literal) => {
                run_segment::<L, W, LINK, BYPASS, $ramping, $armable, MONO, $glide>(
                    &mut instance.sides,
                    &mut left[position * W..(position + length) * W],
                    if MONO {
                        &mut [][..]
                    } else {
                        &mut right[position * W..(position + length) * W]
                    },
                    length,
                    &mut segments,
                    &mut glide,
                    armed_after,
                    &link_lanes,
                )
            };
        }
        if GLIDE || plan.ramping || FORCE_RAMPING {
            match (armable, GLIDE) {
                (true, true) => segment!(true, true, true),
                (false, true) => segment!(true, false, true),
                (true, false) => segment!(true, true, false),
                (false, false) => segment!(true, false, false),
            }
            let advanced = length as u32;
            if GLIDE {
                instance.sides[0].store_glide(&glide[0], advanced);
                if !MONO {
                    instance.sides[1].store_glide(&glide[1], advanced);
                }
            }
            instance.sides[0].store_segment(&segments[0], advanced);
            if !MONO {
                instance.sides[1].store_segment(&segments[1], advanced);
            }
        } else {
            #[cfg(debug_assertions)]
            assert!(instance.flat_sides_are_identity(if MONO { 1 } else { 2 }));
            if armable {
                segment!(false, true, false);
            } else {
                segment!(false, false, false);
            }
            // The write-back is skipped rather than performed and discarded, and that is sound
            // for the same reason the additions are: `store_segment` would write `current` back
            // unchanged, because no lane of it moved, and would apply `saturating_sub` to a
            // `remaining` that is already zero.
        }
        position += length;
    }
}

/// Dispatches on the two values that are fixed at preparation, once per block: the link arm the
/// lanes' modes pick ([`LinkLanes`], issue #1367 D2) and the prepared bypass.
///
/// `FORCE_RAMPING` is threaded through from [`process_block`] and is `false` on every production
/// call; only the crate's split-identity test instantiates the other arm.
#[inline(always)]
fn render<L: Lane, const W: usize, const FORCE_RAMPING: bool>(
    instance: &mut Instance<L, W>,
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    reports: &mut [ProcessReport],
) {
    render_body::<L, W, FORCE_RAMPING, false>(instance, left, right, frames, reports);
}

/// [`render`] with the collapse's `MONO` switch (issue #1871): at `true` the block is the
/// collapsed one, `right` is never read or written, and the D7 check is
/// [`Instance::finish_mono`].
#[inline(always)]
fn render_body<L: Lane, const W: usize, const FORCE_RAMPING: bool, const MONO: bool>(
    instance: &mut Instance<L, W>,
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    reports: &mut [ProcessReport],
) {
    // Issue #1367 D3: a block that starts with a link glide in flight takes the glide-capable
    // block, whose resting lanes take their own modes' laws from the mixed arm's masks; every
    // other block takes the arm it took before.
    match (
        instance.gliding::<MONO>(),
        instance.link_lanes.arm,
        instance.bypass,
    ) {
        (true, _, false) => {
            process_block::<L, W, LINK_MIXED, false, FORCE_RAMPING, MONO, true>(
                instance, left, right, frames,
            );
        }
        (true, _, true) => {
            process_block::<L, W, LINK_DUAL_MONO, true, FORCE_RAMPING, MONO, true>(
                instance, left, right, frames,
            );
        }
        (false, LINK_DUAL_MONO, false) => {
            process_block::<L, W, LINK_DUAL_MONO, false, FORCE_RAMPING, MONO, false>(
                instance, left, right, frames,
            );
        }
        (false, LINK_MAXIMUM, false) => {
            process_block::<L, W, LINK_MAXIMUM, false, FORCE_RAMPING, MONO, false>(
                instance, left, right, frames,
            );
        }
        (false, LINK_AVERAGE, false) => {
            process_block::<L, W, LINK_AVERAGE, false, FORCE_RAMPING, MONO, false>(
                instance, left, right, frames,
            );
        }
        (false, _, false) => {
            process_block::<L, W, LINK_MIXED, false, FORCE_RAMPING, MONO, false>(
                instance, left, right, frames,
            );
        }
        (false, _, true) => {
            process_block::<L, W, LINK_DUAL_MONO, true, FORCE_RAMPING, MONO, false>(
                instance, left, right, frames,
            );
        }
    }
    if MONO {
        instance.finish_mono(left, reports);
    } else {
        instance.finish(left, right, reports);
    }
}

impl<L: Lane, const W: usize> Side<L, W> {
    /// The seventeen ramps of every track, as lanes, for one segment.
    fn segment(&self) -> Segment<L> {
        let mut current = [L::zero(); RAMP_SLOTS];
        let mut step = [L::zero(); RAMP_SLOTS];
        let mut target = [L::zero(); RAMP_SLOTS];
        for index in 0..RAMP_SLOTS {
            let mut values = [0.0f32; 8];
            let mut steps = [0.0f32; 8];
            let mut targets = [0.0f32; 8];
            for track in 0..W {
                values[track] = self.ramps[track][index].current;
                steps[track] = self.ramps[track][index].step;
                targets[track] = self.ramps[track][index].target;
            }
            current[index] = L::load(&values[..W]);
            step[index] = L::load(&steps[..W]);
            target[index] = L::load(&targets[..W]);
        }
        Segment {
            current,
            step,
            target,
        }
    }

    /// Writes a finished segment's lane values back into the scalar ramps.
    ///
    /// The lane accumulation is `ramp_toward(current, step, target)` iterated once per frame, lane
    /// by lane, which is exactly what `LinearRamp::next_value` does at `remaining >= 2`; the
    /// segment split guarantees no ramp reaches `remaining == 1` inside a segment, so no snap can
    /// be missed and the state can be written back instead of replayed.
    fn store_segment(&mut self, segment: &Segment<L>, advanced: u32) {
        for index in 0..RAMP_SLOTS {
            let mut values = [0.0f32; 8];
            segment.current[index].store(&mut values[..W]);
            for (track, value) in values.iter().enumerate().take(W) {
                let ramp = &mut self.ramps[track][index];
                ramp.current = *value;
                ramp.remaining = ramp.remaining.saturating_sub(advanced);
            }
        }
    }
}

impl<L: Lane, const W: usize> Instance<L, W> {
    /// Frames until the next ramp arrival, at most `budget`, and whether anything ramps over them.
    ///
    /// D11's snap is an assignment on the final sample, not an addition, so it cannot happen
    /// inside a vectorised run. Every ramp that is one sample from its target is therefore snapped
    /// here, at a zero-length segment boundary, and the segment that follows is bounded by the
    /// nearest remaining arrival. Boundaries depend only on absolute ramp positions, which is what
    /// makes the result independent of how the caller partitions the block.
    ///
    /// The snap pass runs **before** the ramping scan, and that ordering is what makes a settled
    /// ramp stop costing anything. A ramp arriving on this segment's first sample is snapped here,
    /// leaves `remaining == 0` behind it, and so does not report itself as in flight; the rest of
    /// the block is then one flat segment. Without the split that segment still ran the
    /// thirty-four additions per frame with every step at `+0.0`, which is the cost the split
    /// removes.
    ///
    /// The scan covers all seventeen slots of every track on both channels: the ten parameter
    /// ramps, the four coefficient ramps and the three crossover word ramps (issue #1338). A
    /// coefficient ramp therefore snaps to its exact designed word here, and one still in flight
    /// keeps the segment on the ramped path even when its parameter ramp has already settled (a
    /// retarget to the value in force, which the hoist in `LinearRamp::set_target` settles at once
    /// while the coefficient returns from mid-walk). That is the compressor's "a Point that cancels
    /// a time ramp" rule, kept by the scan rather than by re-arming the parameter ramp.
    ///
    /// Where a segment ends does not move a lane's bits (issue #1069): every word the kernel reads
    /// on a ramped segment, coefficients included, is advanced and read frame by frame, so a
    /// boundary set by another lane's arrival or by the host's block end only splits a run of
    /// identical per-frame arithmetic.
    ///
    /// `MONO` (issue #1871) snaps and scans the left side alone: a collapsed block leaves the
    /// right side as the collapse found it, and on the dual run it is the left one.
    fn plan_segment<const MONO: bool>(&mut self, budget: usize) -> SegmentPlan {
        let sides = if MONO { 1 } else { 2 };
        for side in self.sides.iter_mut().take(sides) {
            for track in 0..W {
                for ramp in side.ramps[track].iter_mut().chain(&mut side.links[track]) {
                    if ramp.remaining == 1 {
                        ramp.snap();
                    }
                }
            }
        }
        let mut length = budget;
        let mut ramping = false;
        let mut gliding = false;
        for side in self.sides.iter().take(sides) {
            for track in 0..W {
                for ramp in &side.ramps[track] {
                    if ramp.remaining > 0 {
                        ramping = true;
                        length = length.min(ramp.remaining as usize - 1);
                    }
                }
                // Issue #1367 D3: a link glide weight splits and keeps the segment ramped as the
                // other ramps do, so its arrival is a boundary and never inside a segment.
                for ramp in &side.links[track] {
                    if ramp.remaining > 0 {
                        gliding = true;
                        length = length.min(ramp.remaining as usize - 1);
                    }
                }
            }
        }
        SegmentPlan {
            frames: length.max(1),
            ramping: ramping || gliding,
            gliding,
        }
    }

    /// The precondition that makes the flat path bit-identical to the ramped one.
    ///
    /// Debug-only, and asserted at the point of use rather than argued in a comment: on a segment
    /// the split sends down the flat path, dropping `ramp_toward(current, step, target)` must be
    /// the identity on every lane of every parameter of both channels. It is, when each ramp is at
    /// rest with a `+0.0` step and holds a value that `x + 0.0 == x` preserves bit for bit — which
    /// excludes `-0.0` (it would become `+0.0`) and the non-finite values (a NaN is quieted by the
    /// addition). Those are the same two exclusions `LinearRamp::stationary_at` carries, for the
    /// same reason.
    #[cfg(test)]
    fn flat_path_is_identity(&self) -> bool {
        self.flat_sides_are_identity(2)
    }

    /// The flat-path identity precondition on the first `sides` sides: a collapsed block (issue
    /// #1871) runs the left side's ramps alone; `sides = 2` is the dual block's.
    #[cfg(any(debug_assertions, test))]
    fn flat_sides_are_identity(&self, sides: usize) -> bool {
        const NEGATIVE_ZERO: u32 = 0x8000_0000;
        self.sides.iter().take(sides).all(|side| {
            side.ramps[..W]
                .iter()
                .flatten()
                .chain(side.links.iter().flatten())
                .all(|ramp| {
                    ramp.remaining == 0
                        && ramp.step.to_bits() == 0
                        && ramp.current.is_finite()
                        && ramp.current.to_bits() != NEGATIVE_ZERO
                })
        })
    }

    /// The once-per-block output boundary check of master plan §4.4, recovered one lane at a time
    /// (issue #1340, D1; decision 12's coupling rule).
    ///
    /// The scan and the counters are `effect-runtime`'s. A lane that failed on either channel has
    /// both of its channels zeroed for the whole block and its history cleared on both
    /// [`Side`]s ([`Side::discontinuity_reset_lane`]), which is exactly what the scalar instance
    /// does to its one lane; every other lane is left bit-unchanged. The shared
    /// `bank::finish_block` zeroes and resets the whole bank instead, so one hot track -- a
    /// bypassed one whose wet path still runs under a lowered bypass -- would silence its
    /// bank-mates. The report counts **one block** on each channel whose own output failed (D7,
    /// #1073): a channel that was only zeroed alongside it is not a nonfinite block.
    fn finish(&mut self, left: &mut [f32], right: &mut [f32], reports: &mut [ProcessReport]) {
        if bank::check_block::<L>(left) && bank::check_block::<L>(right) {
            return;
        }
        // The rejected path only: which lanes failed on which channel, read before the zeroing.
        let failed = [
            bank::nonfinite_lane_mask::<L>(left),
            bank::nonfinite_lane_mask::<L>(right),
        ];
        let lanes = failed[0] | failed[1];
        // Every failed lane recovers, a padded one too, so it stays finite and at rest; only an
        // active lane is charged (issue #1893).
        for track in 0..W {
            if lanes & (1 << track) == 0 {
                continue;
            }
            for word in left.iter_mut().skip(track).step_by(W) {
                *word = 0.0;
            }
            for word in right.iter_mut().skip(track).step_by(W) {
                *word = 0.0;
            }
            for side in &mut self.sides {
                side.discontinuity_reset_lane(track);
            }
        }
        self.charge(failed[0], failed[1], reports);
    }

    /// [`finish`](Self::finish) for a collapsed block (issue #1871): the left plane alone
    /// decides, as the dual block on `L == R` fails the same lanes on both channels. A failed
    /// lane's left words are zeroed and its history is cleared on **both** sides, which is what
    /// the dual block does; the right side's words are then the copy's to replace. Each failed
    /// lane counts one block on each channel, because the right plane the seam writes is this
    /// left plane.
    fn finish_mono(&mut self, left: &mut [f32], reports: &mut [ProcessReport]) {
        if bank::check_block::<L>(left) {
            return;
        }
        let lanes = bank::nonfinite_lane_mask::<L>(left);
        for track in 0..W {
            if lanes & (1 << track) == 0 {
                continue;
            }
            for word in left.iter_mut().skip(track).step_by(W) {
                *word = 0.0;
            }
            for side in &mut self.sides {
                side.discontinuity_reset_lane(track);
            }
        }
        self.charge(lanes, lanes, reports);
    }

    /// Charges a rejected block's failed lanes, the active ones only (issue #1893): a padded
    /// lane's failure is recovered by the caller but never counted or reported. A block whose
    /// failures are all on padded lanes leaves the counters as they were.
    fn charge(&mut self, left: u32, right: u32, reports: &mut [ProcessReport]) {
        let (left, right) = (left & self.active, right & self.active);
        let lanes = left | right;
        if lanes == 0 {
            return;
        }
        self.nonfinite.nonfinite_lanes = lanes;
        self.nonfinite.nonfinite_blocks = self.nonfinite.nonfinite_blocks.saturating_add(1);
        for (track, report) in reports.iter_mut().enumerate().take(W) {
            report.nonfinite_left_blocks = report
                .nonfinite_left_blocks
                .saturating_add(u64::from((left >> track) & 1));
            report.nonfinite_right_blocks = report
                .nonfinite_right_blocks
                .saturating_add(u64::from((right >> track) & 1));
        }
    }

    /// Points one track's ramps at the values a block's automation spans carry.
    ///
    /// The spans are events landing on the block's first sample, one
    /// per parameter per channel, in `parameter_index * 2 + channel` order. Anything else is
    /// counted as an invalid span and ignored; nothing partially applies.
    fn apply_automation(
        &mut self,
        track: usize,
        spans: &[PreparedAutomationSpan],
        capacity: u32,
        first_sample: u64,
        report: &mut ProcessReport,
    ) {
        let mut pending = [[None; PARAMETER_COUNT]; 2];
        let mut previous: Option<u32> = None;
        for (index, span) in spans.iter().enumerate() {
            let side = match span.channel {
                ParameterChannel::Left => 0usize,
                ParameterChannel::Right => 1,
                ParameterChannel::Both => {
                    report.invalid_spans = report.invalid_spans.saturating_add(1);
                    continue;
                }
            };
            let parameter = span.parameter_index as usize;
            let order = span
                .parameter_index
                .checked_mul(2)
                .and_then(|value| value.checked_add(side as u32));
            let valid = parameter < PARAMETER_COUNT
                && index < capacity as usize
                && span.sample == first_sample
                && parameter_value_valid(&SPECS[parameter], span.value)
                && order.is_some_and(|value| previous.is_none_or(|earlier| value > earlier))
                && pending[side][parameter].is_none();
            if !valid {
                report.invalid_spans = report.invalid_spans.saturating_add(1);
                continue;
            }
            previous = order;
            pending[side][parameter] = Some(normalize_zero(span.value));
        }
        let sample_rate = self.sample_rate;
        for (side, targets) in pending.iter().enumerate() {
            // Issue #1338 D2: a crossover span designs its target words once, here, with the
            // infallible designer (#1366 D3: total over the domain `parameter_value_valid` has
            // just checked), and the three word ramps walk to them over the same window.
            if let Some(value) = targets[0] {
                let channel = &mut self.sides[side];
                channel.crossover_hz[track] = value;
                let words = design_lr4_words(sample_rate, value);
                for (word, target) in words.into_iter().enumerate() {
                    channel.ramps[track][crossover_slot(word)]
                        .set_target(target, SMOOTHING_SAMPLES);
                }
            }
            for (index, target) in targets[1..].iter().enumerate() {
                if let Some(value) = *target {
                    let ramps = &mut self.sides[side].ramps[track];
                    ramps[index].set_target(value, SMOOTHING_SAMPLES);
                    // Issue #1069: a time retarget designs its exact coefficient once, here, and
                    // the coefficient walks to it over the same window (the compressor's
                    // `set_parameter_target`). A retarget to the time in force settles the
                    // parameter ramp at once, and still returns a coefficient caught mid-walk to
                    // that time's exact design over the window: `plan_segment` scans this slot.
                    if let Some(rate) = rate_of_ramp(index) {
                        ramps[rate_slot(rate)].set_target(
                            retention_coefficient(value, sample_rate),
                            SMOOTHING_SAMPLES,
                        );
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------
// State payload, current layout
// ---------------------------------------------------------------------------------------------
//
// Per channel: crossover, the two smoother words, ten four-word parameter ramps, the four filter
// words, the silence counter, the four four-word coefficient ramps (issue #1069), the three
// four-word crossover word ramps (issue #1338), and the two four-word link glide weights (issue
// #1367 D4).

/// Word offset of the first filter word.
const FILTER_WORD: usize = 3 + RAMP_COUNT * RAMP_WORDS;
/// The input's silence counter, after the four crossover words (issue #1328, amendment A9).
const SILENCE_WORD: usize = FILTER_WORD + 4;
/// Word offset of the first coefficient ramp, after the silence counter (issue #1069).
const RATE_WORD: usize = SILENCE_WORD + 1;
/// Word offset of the first crossover word ramp, after the coefficient ramps (issue #1338).
const CROSSOVER_WORD: usize = RATE_WORD + RATE_COUNT * RAMP_WORDS;
/// Word offset of the first link glide weight (`w_link`, then `w_avg`), after the crossover word
/// ramps (issue #1367 D4).
const LINK_WORD: usize = CROSSOVER_WORD + CROSSOVER_WORDS * RAMP_WORDS;
/// The silence counter's saturation, `2^24`.
const SILENCE_SATURATION: f32 = 16_777_216.0;

// REALTIME_POLICY_BEGIN: #1278 D3, the payload codec runs in the plan-swap block.
/// One lane of a lane-wide value.
#[inline]
fn lane_value<L: Lane>(value: L, track: usize) -> f32 {
    let mut words = [0.0f32; 8];
    value.store(&mut words[..L::WIDTH]);
    words[track]
}

/// `value` with lane `track` replaced.
#[inline]
fn set_lane_value<L: Lane>(value: L, track: usize, replacement: f32) -> L {
    let mut words = [0.0f32; 8];
    value.store(&mut words[..L::WIDTH]);
    words[track] = replacement;
    L::load(&words[..L::WIDTH])
}

/// One channel's validated state, staged so that a rejected restore changes nothing.
struct StagedSide {
    crossover_hz: f32,
    gains: [f32; 2],
    ramps: [LinearRamp; RAMP_SLOTS],
    filter: [f32; 4],
    silence: f32,
    /// The two link glide weights (issue #1367 D4).
    links: [LinearRamp; LINK_WEIGHTS],
    /// The mode the weights glide to, decoded from their targets.
    mode: LinkMode,
}

fn write_side<L: Lane, const W: usize>(bytes: &mut [u8], side: &Side<L, W>, track: usize) {
    write_f32(bytes, 0, side.crossover_hz[track]);
    write_f32(bytes, 1, lane_value(side.gain_db[LOW_BAND], track));
    write_f32(bytes, 2, lane_value(side.gain_db[HIGH_BAND], track));
    for index in 0..RAMP_COUNT {
        let ramp = side.ramps[track][index];
        let word = 3 + index * RAMP_WORDS;
        write_f32(bytes, word, ramp.current);
        write_f32(bytes, word + 1, ramp.target);
        write_f32(bytes, word + 2, ramp.step);
        write_u32(bytes, word + 3, ramp.remaining);
    }
    let filter = [
        side.filter.a.ic1,
        side.filter.a.ic2,
        side.filter.b.ic1,
        side.filter.b.ic2,
    ];
    for (index, value) in filter.into_iter().enumerate() {
        write_f32(bytes, FILTER_WORD + index, lane_value(value, track));
    }
    write_f32(bytes, SILENCE_WORD, lane_value(side.silence, track));
    for rate in 0..RATE_COUNT {
        write_ramp(
            bytes,
            RATE_WORD + rate * RAMP_WORDS,
            side.ramps[track][rate_slot(rate)],
        );
    }
    for word in 0..CROSSOVER_WORDS {
        write_ramp(
            bytes,
            CROSSOVER_WORD + word * RAMP_WORDS,
            side.ramps[track][crossover_slot(word)],
        );
    }
    for (weight, ramp) in side.links[track].iter().enumerate() {
        write_ramp(bytes, LINK_WORD + weight * RAMP_WORDS, *ramp);
    }
}

/// Validates one channel's fixed words.
fn stage_side(bytes: &[u8], sample_rate: u32) -> Result<StagedSide, StatePayloadError> {
    let crossover_hz = read_f32(bytes, 0);
    if !parameter_state_valid(0, crossover_hz) {
        return Err(state_error("effect.state.parameter"));
    }
    let designed =
        design_lr4(sample_rate, crossover_hz).ok_or(state_error("effect.state.coefficient"))?;
    let gains = [read_f32(bytes, 1), read_f32(bytes, 2)];
    if gains
        .into_iter()
        .any(|value| !normal_or_zero(value) || !(-100.0..=0.0).contains(&value))
    {
        return Err(state_error("effect.state.gain"));
    }
    let mut ramps = [LinearRamp::fixed(0.0); RAMP_SLOTS];
    for (index, ramp) in ramps[..RAMP_COUNT].iter_mut().enumerate() {
        let word = 3 + index * RAMP_WORDS;
        let current = read_f32(bytes, word);
        let target = read_f32(bytes, word + 1);
        let step = read_f32(bytes, word + 2);
        let remaining = read_u32(bytes, word + 3);
        // `LinearRamp`'s invariant, enforced rather than assumed: a ramp at rest is at its target
        // and has no increment. A payload that says otherwise would have the segment driver add a
        // stale step to a resting parameter for ever.
        //
        // #1278: the effect's own mid-ramp snapshot must restore. The step of a ramp a few ulps
        // long near zero is subnormal, so the step is held only to be finite. The target and the
        // `current`, moving or settled, are held to the strict domain: the clamped law keeps every
        // word between the ramp's start and its target (issue #1409 D2), so a word past the
        // domain is one the engine never holds (issue #1411 D1).
        let spec = &SPECS[index + 1];
        let read = LinearRamp {
            current,
            target,
            step,
            remaining,
        };
        if !parameter_state_valid(index + 1, current)
            || !parameter_state_valid(index + 1, target)
            || !ramp_path_inside(read, (spec.minimum, spec.maximum), SMOOTHING_SAMPLES)
            || (remaining == 0 && current.to_bits() != target.to_bits())
        {
            return Err(state_error("effect.state.parameter"));
        }
        *ramp = LinearRamp {
            current: normalize_zero(current),
            target: normalize_zero(target),
            step: normalize_zero(step),
            remaining,
        };
    }
    let mut filter = [0.0f32; 4];
    for (index, word) in filter.iter_mut().enumerate() {
        *word = read_f32(bytes, FILTER_WORD + index);
        if !normal_or_zero(*word) {
            return Err(state_error("effect.state.filter"));
        }
    }
    // The silence counter is `+0.0` or an `f32` integer in `[1, 2^24]`: exactly the words
    // `lane::silence_step` writes (issue #1328, amendment A9).
    let silence = read_f32(bytes, SILENCE_WORD);
    let counted =
        (1.0..=SILENCE_SATURATION).contains(&silence) && (silence as u32) as f32 == silence;
    if silence.to_bits() != 0 && !counted {
        return Err(state_error("effect.state.filter"));
    }
    // Issue #1069, the compressor's rule: `retention_coefficient` designs every legal time in
    // `(0, 1]` (exactly `0.0` only for a time the domains exclude), so a target is held there, and
    // a `current`, moving or settled, lies between two designed words under the clamped law
    // (issue #1409 D2) and is held to the same range with no rounding budget (issue #1411 D1). A
    // settled coefficient ramp is at its target, as a settled parameter ramp is.
    for rate in 0..RATE_COUNT {
        let read = read_ramp(bytes, RATE_WORD + rate * RAMP_WORDS);
        let designed = |value: f32| value > 0.0 && value <= 1.0;
        if !designed(read.current)
            || !designed(read.target)
            || !ramp_path_inside(read, (0.0, 1.0), SMOOTHING_SAMPLES)
            || (read.remaining == 0 && read.current.to_bits() != read.target.to_bits())
        {
            return Err(state_error("effect.state.parameter"));
        }
        ramps[rate_slot(rate)] = LinearRamp {
            step: normalize_zero(read.step),
            ..read
        };
    }
    // Issue #1338 D6: each crossover word ramp walks between two designs. Its target triple is the
    // design of `crossover_hz` bit for bit, so a triple the engine would not design is refused and
    // the target's stability is #1366's proof, not a runtime check. Every word is a positive normal
    // below 1, a settled ramp is at its target, and the current triple, a point of a walk between
    // two designs, is held within `RAMP_PATH_NORM_TOLERANCE` at every point of its remaining walk.
    for (word, design) in designed.into_iter().enumerate() {
        let read = read_ramp(bytes, CROSSOVER_WORD + word * RAMP_WORDS);
        let positive = |value: f32| value.is_normal() && value > 0.0 && value < 1.0;
        if !positive(read.current)
            || read.target.to_bits() != design.to_bits()
            || !ramp_path_inside(read, (0.0, 1.0), SMOOTHING_SAMPLES)
            || (read.remaining == 0 && read.current.to_bits() != read.target.to_bits())
        {
            return Err(state_error("effect.state.coefficient"));
        }
        ramps[crossover_slot(word)] = LinearRamp {
            step: normalize_zero(read.step),
            ..read
        };
    }
    // The walk the render takes, point by point (`LinearRamp::next_value` is the kernel's clamped
    // law and its snap): a forged step keeps each word inside its endpoints but can leave the
    // straight line between the designs, so the current point alone proves nothing about the path
    // (the parametric EQ's restore walks its words for the same reason). At most 64 points.
    let mut walk: [LinearRamp; CROSSOVER_WORDS] =
        core::array::from_fn(|word| ramps[crossover_slot(word)]);
    let mut point = walk.map(|ramp| ramp.current);
    loop {
        if transition_norm(point[0], point[1], point[2]) > RAMP_PATH_NORM_TOLERANCE {
            return Err(state_error("effect.state.coefficient"));
        }
        if walk.iter().all(|ramp| ramp.remaining == 0) {
            break;
        }
        point = walk.each_mut().map(LinearRamp::next_value);
    }
    // Issue #1367 D4 (#1370 D6): each weight lies in `[0, 1]` on its whole path, the engine never
    // holds `-0.0` there, a glide in flight moves (its step is `(t - c) / n` with `t` in `{0, 1}`,
    // `c` a word of an earlier glide and `n <= 2^24`, never `0`), a settled weight holds its
    // target, and the two targets are exactly one supported mode's rest weights.
    let mut links = [LinearRamp::fixed(0.0); LINK_WEIGHTS];
    for (weight, link) in links.iter_mut().enumerate() {
        let read = read_ramp(bytes, LINK_WORD + weight * RAMP_WORDS);
        let valid = read.current.to_bits() != (-0.0_f32).to_bits()
            && (read.remaining == 0 || read.step != 0.0)
            && (read.remaining != 0 || read.current.to_bits() == read.target.to_bits())
            && ramp_path_inside(read, (0.0, 1.0), effect_contract::LINK_GLIDE_MAX_SAMPLES);
        if !valid {
            return Err(state_error("effect.state.link"));
        }
        *link = read;
    }
    let mode = effect_contract::link_glide_mode([links[0].target, links[1].target])
        .filter(|mode| {
            MULTIBAND_COMPRESSOR_DESCRIPTOR
                .supported_link_modes
                .contains(*mode)
        })
        .ok_or(state_error("effect.state.link"))?;
    Ok(StagedSide {
        crossover_hz,
        gains: [normalize_zero(gains[0]), normalize_zero(gains[1])],
        ramps,
        filter,
        silence,
        links,
        mode,
    })
}

/// Applies a staged channel. Never allocates.
fn commit_side<L: Lane, const W: usize>(side: &mut Side<L, W>, staged: &StagedSide, track: usize) {
    side.crossover_hz[track] = staged.crossover_hz;
    side.gain_db[LOW_BAND] = set_lane_value(side.gain_db[LOW_BAND], track, staged.gains[LOW_BAND]);
    side.gain_db[HIGH_BAND] =
        set_lane_value(side.gain_db[HIGH_BAND], track, staged.gains[HIGH_BAND]);
    side.ramps[track] = staged.ramps;
    side.filter.a.ic1 = set_lane_value(side.filter.a.ic1, track, staged.filter[0]);
    side.filter.a.ic2 = set_lane_value(side.filter.a.ic2, track, staged.filter[1]);
    side.filter.b.ic1 = set_lane_value(side.filter.b.ic1, track, staged.filter[2]);
    side.filter.b.ic2 = set_lane_value(side.filter.b.ic2, track, staged.filter[3]);
    side.silence = set_lane_value(side.silence, track, staged.silence);
}

fn state_error(code: &'static str) -> StatePayloadError {
    StatePayloadError { code }
}

fn parameter_state_valid(index: usize, value: f32) -> bool {
    parameter_value_valid(&SPECS[index], value)
}

/// Bytes in one payload word. The shared codec's `WORD_BYTES`, named once here.
const fn state_payload_word_bytes() -> usize {
    effect_runtime::state_payload::WORD_BYTES
}

impl<L: Lane, const W: usize> Instance<L, W> {
    /// The three section lengths a payload of this instance must have.
    ///
    /// The shared codec's `validate_lengths` is not used: it derives its sizes from
    /// `expected_sizes`, which unconditionally reserves the two versioned header words this crate
    /// does not carry (W2-D2). The word codec itself is the shared one.
    fn validate_lengths(&self, sections: (usize, usize, usize)) -> Result<(), StatePayloadError> {
        let lane = LANE_HEADER_WORDS * state_payload_word_bytes();
        if sections.0 != 0 || sections.1 != lane || sections.2 != lane {
            return Err(state_error(STATE_LENGTH_CODE));
        }
        Ok(())
    }

    fn snapshot(
        &self,
        track: usize,
        output: StatePayloadOutput<'_>,
        sizes: StatePayloadSizes,
    ) -> Result<(), StatePayloadError> {
        self.validate_lengths((output.common.len(), output.left.len(), output.right.len()))?;
        debug_assert_eq!(output.left.len(), sizes.left_bytes as usize);
        write_side(output.left, &self.sides[0], track);
        write_side(output.right, &self.sides[1], track);
        Ok(())
    }

    fn restore(
        &mut self,
        track: usize,
        version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        if version != STATE_LAYOUT_VERSION {
            return Err(state_error(STATE_VERSION_CODE));
        }
        self.validate_lengths((input.common.len(), input.left.len(), input.right.len()))?;
        let left = stage_side(input.left, self.sample_rate)?;
        let right = stage_side(input.right, self.sample_rate)?;
        if left.mode != right.mode {
            return Err(state_error("effect.state.link"));
        }
        commit_side(&mut self.sides[0], &left, track);
        commit_side(&mut self.sides[1], &right, track);
        // Issue #1367 D4 (#1370 D6): the weights land only under the mode they glide to. A
        // payload taken under another mode leaves the lane's own mode at rest.
        if left.mode == self.links[track] {
            self.sides[0].links[track] = left.links;
            self.sides[1].links[track] = right.links;
        } else {
            self.rest_links(track);
        }
        Ok(())
    }
}
// REALTIME_POLICY_END

// ---------------------------------------------------------------------------------------------
// Contract surface
// ---------------------------------------------------------------------------------------------

/// Factory for the fixed two-band multiband compressor.
#[derive(Clone, Copy, Debug, Default)]
pub struct MultibandCompressorFactory;

/// A prepared scalar multiband compressor: the `WIDTH = 1` instantiation of the one body.
///
/// It keeps no copy of its prepared metadata (issue #1461): the record goes back to the control
/// side in [`PreparedEffect::metadata`], and the processor keeps only the two values its own
/// render-owned methods read.
pub struct PreparedMultibandCompressor {
    /// The prepared automation capacity, read by `process`.
    automation_capacity: u32,
    /// The prepared state payload sizes, read by `snapshot_state_payload`.
    state_sizes: StatePayloadSizes,
    instance: Instance<f32, 1>,
}

/// A prepared homogeneous bank of `W` tracks.
///
/// Like the scalar form it keeps no copy of its prepared or bank metadata (issue #1461), only the
/// values its own methods read.
struct PreparedMultibandCompressorBank<L: Lane, const W: usize> {
    /// The bound bank width, read by `process_bank`.
    width: BankWidth,
    /// The prepared automation capacity, read by `process_bank`.
    automation_capacity: u32,
    /// The prepared state payload sizes, read by `snapshot_track_state_payload`.
    state_sizes: StatePayloadSizes,
    instance: Instance<L, W>,
    /// Issue #893: the bank is at a silent fixed point. Earned at the end of a rendered block
    /// whose input was exact `+0.0` with no ramp in flight, when every lane is at rest
    /// ([`Side::rest_lanes`]) on both channels, or by [`advance_clocks`] over the whole bank;
    /// withdrawn by every automation block, reset and restore. While it stands, the next quiet
    /// block is the same block again: every lane stays at rest and writes `+0.0`, so
    /// `process_bank` advances the clocks and returns. The prepared bypass is fixed at preparation
    /// (only a direct factory request sets it; a session bypass is the rack's shunt and never
    /// reaches this kernel, #1340), so the claim never spans two kernel paths.
    ///
    /// [`advance_clocks`]: PreparedNativeEffectBank::advance_clocks
    silent_fixed_point: bool,
}

/// The mask with every lane of a `W`-lane bank set.
const fn all_lanes<const W: usize>() -> u32 {
    if W >= 32 { u32::MAX } else { (1 << W) - 1 }
}

/// The lane mask whose lane `l` is set when bit `l` of `bits` is.
fn lanes_mask<L: Lane>(bits: u32) -> L::Mask {
    let mut flags = [0.0_f32; 8];
    for (lane, flag) in flags.iter_mut().enumerate().take(L::WIDTH) {
        if bits & (1 << lane) != 0 {
            *flag = -1.0;
        }
    }
    L::load(&flags[..L::WIDTH]).lt(L::zero())
}

impl<L: Lane, const W: usize> Instance<L, W> {
    /// Both channels' [`Side::rest_lanes`], or the left side's alone while
    /// [`collapsed`](Self::collapsed) (#1894).
    fn lanes_at_rest(&self) -> u32 {
        if self.collapsed {
            self.sides[0].rest_lanes()
        } else {
            self.sides[0].rest_lanes() & self.sides[1].rest_lanes()
        }
    }

    /// The sides a rendered `+0.0` block advances: the left alone while
    /// [`collapsed`](Self::collapsed), both otherwise (#1894).
    fn live_sides(&self) -> usize {
        if self.collapsed { 1 } else { 2 }
    }

    /// Advances the silence counters of the lanes in `bits` over `frames` frames of `+0.0`, as a
    /// rendered block would: `min(run + frames, 2^24)`. A prepared bypass (a direct factory
    /// request's; a session bypass is the rack's shunt, #1340) freezes the counters with the
    /// filter, so it advances nothing. A collapsed bank advances the left side alone, as its
    /// rendered block would (#1894).
    fn advance_silence(&mut self, bits: u32, frames: usize) {
        self.advance_silence_sides(bits, frames, self.live_sides());
    }

    /// [`advance_silence`](Self::advance_silence) on the first `sides` sides: a collapsed block's
    /// fixed-point skip (issue #1871) advances the left side alone, as its rendered block would.
    fn advance_silence_sides(&mut self, bits: u32, frames: usize, sides: usize) {
        if self.bypass || bits == 0 {
            return;
        }
        let mask = lanes_mask::<L>(bits);
        for side in self.sides.iter_mut().take(sides) {
            let mut advanced = side.silence;
            silence_advance(&mut advanced, frames);
            side.silence = L::select(mask, advanced, side.silence);
        }
    }
}

impl<L: Lane, const W: usize> Instance<L, W> {
    /// The `DESIGNED` term of the channel-symmetry witness for `lane` (issue #1871): whether every
    /// per-channel word the kernel reads that is not running state compares bit-equal between the
    /// two channels.
    ///
    /// # The word list, and why it is exactly this
    ///
    /// [`run_segment`] and [`process_block`] read per channel a [`Side`]'s `filter`, `silence`,
    /// `gain_db` and `ramps`, and nothing else. The first three are running state, which the
    /// induction the witness states carries. The designed words are the seventeen ramps of the
    /// lane's track ([`RAMP_SLOTS`]: ten parameter, four rate and three crossover word ramps), all
    /// four fields each: `remaining` decides whether a ramp moves and when it snaps (and so where
    /// a segment ends), and `current`, `step` and `target` are what it moves and where it stops.
    ///
    /// The lane's two link glide weights (issue #1367) are designed words too, all four fields:
    /// a glide in flight blends the lane's detector with them.
    ///
    /// Deliberately excluded: `prepared_rates`, `prepared_designed`, `crossover_hz` and
    /// `defaults`, which the kernel never loads (a reset or a span that made the channels disagree
    /// shows in the ramps); `sample_rate` and `bypass`, which are whole-instance; and `links` and
    /// `link_lanes`, each lane's mode, which is one value for both channels of the lane.
    ///
    /// Raw bits, so `+0.0` against `-0.0` declines. A lane past the width declines.
    fn designed_channel_symmetry(&self, lane: usize) -> bool {
        if lane >= W {
            return false;
        }
        let [left, right] = &self.sides;
        left.ramps[lane]
            .iter()
            .chain(&left.links[lane])
            .zip(right.ramps[lane].iter().chain(&right.links[lane]))
            .all(|(l, r)| ramp_bits(l) == ramp_bits(r))
    }

    /// The collapse's disengage copy (issue #1871): the left side's whole per-channel render state
    /// onto the right, every lane. That is every per-channel word [`run_segment`] and
    /// [`process_block`] read: the crossover state, the silence counter (so the right's joint
    /// flush arms on the dual run's frame), both smoother words and the seventeen ramps of every
    /// track, which a collapsed block advanced on the left alone.
    ///
    /// `prepared_rates`, `prepared_designed`, `crossover_hz` and `defaults` are not copied: they
    /// are each channel's own control-side values, which a collapsed block's automation updates
    /// on both channels as a dual block would, and which no render reads.
    fn desymmetrize(&mut self) {
        let [left, right] = &mut self.sides;
        right.filter = left.filter;
        right.silence = left.silence;
        right.gain_db = left.gain_db;
        right.ramps = left.ramps;
        // Issue #1367: the link glide weights are per-channel render state too.
        right.links = left.links;
        self.collapsed = false;
    }

    /// [`PreparedNativeEffectBank::channels_agree`] (issue #1871): every word
    /// [`desymmetrize`](Self::desymmetrize) copies compares bit-equal between the channels, on
    /// every lane: seven lane words (four crossover, the silence counter, two smoother words)
    /// and the seventeen ramps' and two link glide weights' (issue #1367) four words per track.
    /// Small enough to compare outright; no fixed point is needed.
    fn channels_bit_equal(&self) -> bool {
        let words = |side: &Side<L, W>| {
            [
                side.filter.a.ic1,
                side.filter.a.ic2,
                side.filter.b.ic1,
                side.filter.b.ic2,
                side.silence,
                side.gain_db[LOW_BAND],
                side.gain_db[HIGH_BAND],
            ]
        };
        let [left, right] = &self.sides;
        words(left).iter().zip(&words(right)).all(|(l, r)| {
            (0..W).all(|lane| lane_value(*l, lane).to_bits() == lane_value(*r, lane).to_bits())
        }) && left
            .ramps
            .iter()
            .flatten()
            .chain(left.links.iter().flatten())
            .zip(
                right
                    .ramps
                    .iter()
                    .flatten()
                    .chain(right.links.iter().flatten()),
            )
            .all(|(l, r)| ramp_bits(l) == ramp_bits(r))
    }
}

/// A ramp's four words as raw bits, for the witness and the agreement test.
fn ramp_bits(ramp: &LinearRamp) -> [u32; 4] {
    [
        ramp.current.to_bits(),
        ramp.target.to_bits(),
        ramp.step.to_bits(),
        ramp.remaining,
    ]
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
    for index in 0..PARAMETER_COUNT {
        let low = values[index * 2];
        let high = values[index * 2 + 1];
        if low.parameter_index != index as u32
            || high.parameter_index != index as u32
            || low.channel != ParameterChannel::Left
            || high.channel != ParameterChannel::Right
            || !parameter_value_valid(&SPECS[index], low.value)
            || !parameter_value_valid(&SPECS[index], high.value)
        {
            return Err(EffectPrepareError {
                code: "effect.parameter.initial",
            });
        }
        left[index] = normalize_zero(low.value);
        right[index] = normalize_zero(high.value);
    }
    Ok((left, right))
}

const PREPARE_FAILED: EffectPrepareError = EffectPrepareError {
    code: "effect.prepare.failed",
};

/// The bound instance of `request` and its first member's metadata, or `None` when the members
/// are not one program: [`prepare_bank`] without the box.
fn bind_instance<L: Lane, const W: usize>(
    factory: &MultibandCompressorFactory,
    request: PrepareEffectBankRequest<'_>,
) -> Result<Option<(Instance<L, W>, PreparedEffectMetadata)>, EffectPrepareError> {
    let first = request
        .requests
        .first()
        .copied()
        .ok_or(EffectPrepareError {
            code: "effect.bank.requests",
        })?;
    let metadata = expected_prepared_metadata(factory.descriptor(), first)?;
    let (first_left, first_right) = initial_defaults(first.initial_values)?;
    let mut left = [first_left; W];
    let mut right = [first_right; W];
    // Issue #1367 D1: the key carries no link mode (`lane_link`), so each active lane takes its
    // own member's mode, and a padded lane the first member's, so padding never forces the mixed
    // arm (members come first, so the first request is a member's).
    let mut links: [LinkMode; W] = [metadata.link_mode; W];
    let mut same_program = true;
    // Every request is validated before any fallback: a malformed bank request is an error, not a
    // reason to fall back to scalar.
    for (track, item) in request.requests.iter().copied().enumerate().skip(1) {
        let candidate = expected_prepared_metadata(factory.descriptor(), item)?;
        if candidate.program_key() != metadata.program_key() {
            same_program = false;
        }
        if request.active_mask[track] {
            links[track] = candidate.link_mode;
        }
        let (track_left, track_right) = initial_defaults(item.initial_values)?;
        left[track] = track_left;
        right[track] = track_right;
    }
    if !same_program {
        return Ok(None);
    }
    // Issue #1893: a padded request binds (decision 12). Its padded lanes are the contract's clones
    // fed `+0.0`; they run the same kernel, and no bank-wide decision reads them in a way that can
    // move an active lane's bits, because a clone fed `+0.0` rests like any lane. `active` keeps
    // them out of every charge, report, automation and payload.
    let mut instance = Instance::<L, W>::new(left, right, metadata).ok_or(PREPARE_FAILED)?;
    instance.seat_links(&links);
    instance.active = request
        .active_mask
        .iter()
        .enumerate()
        .filter(|(_, active)| **active)
        .fold(0, |mask, (lane, _)| mask | (1 << lane));
    Ok(Some((instance, metadata)))
}

fn prepare_bank<L: Lane, const W: usize>(
    factory: &MultibandCompressorFactory,
    request: PrepareEffectBankRequest<'_>,
) -> Result<Option<PreparedEffectBank>, EffectPrepareError> {
    let Some((instance, metadata)) = bind_instance::<L, W>(factory, request)? else {
        return Ok(None);
    };
    Ok(Some(PreparedEffectBank {
        processor: Box::new(PreparedMultibandCompressorBank::<L, W> {
            width: request.width,
            automation_capacity: metadata.automation_capacity,
            state_sizes: metadata.state_sizes,
            instance,
            silent_fixed_point: false,
        }),
        metadata: PreparedBankMetadata {
            width: request.width,
            program_key: metadata.program_key(),
        },
    }))
}

impl NativeEffectFactory for MultibandCompressorFactory {
    fn descriptor(&self) -> &'static EffectDescriptor {
        &MULTIBAND_COMPRESSOR_DESCRIPTOR
    }

    fn prepared_inline_bytes(&self) -> u64 {
        size_of::<PreparedMultibandCompressor>() as u64
    }

    fn prepared_undeclared_heap_bytes(&self, _: &effect_contract::QualityDescriptor) -> u64 {
        0
    }

    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<PreparedEffect, EffectPrepareError> {
        let metadata = expected_prepared_metadata(self.descriptor(), request)?;
        let (left, right) = initial_defaults(request.initial_values)?;
        let instance = Instance::<f32, 1>::new([left], [right], metadata).ok_or(PREPARE_FAILED)?;
        Ok(PreparedEffect {
            processor: Box::new(PreparedMultibandCompressor {
                automation_capacity: metadata.automation_capacity,
                state_sizes: metadata.state_sizes,
                instance,
            }),
            metadata,
        })
    }

    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<PreparedEffectBank>, EffectPrepareError> {
        request.validate_shape()?;
        effect_contract::match_bank_width!(request.width, |L, N| prepare_bank::<L, N>(
            self, request
        ))
    }
}

/// The aggregate band reading for one lane of one channel: the more negative of the two bands.
///
/// `min` rather than a sum or an average, because the two bands are applied to *disjoint* parts of
/// the spectrum: the channel's worst-case reduction is the deeper of the two, and adding them
/// would report a reduction the signal never received.
fn band_aggregate_db<L: Lane, const W: usize>(side: &Side<L, W>, lane: usize) -> f32 {
    let low = lane_value(side.gain_db[LOW_BAND], lane);
    let high = lane_value(side.gain_db[HIGH_BAND], lane);
    if low < high { low } else { high }
}

impl PreparedNativeEffect for PreparedMultibandCompressor {
    /// Every field is inline: this instance owns no heap (#1074).
    fn retained_bytes(&self) -> u64 {
        0
    }

    /// Issue #143 D2 / R3: the deeper of the two bands' smoother words, read for lane 0.
    fn observe_resident(&self, tap_index: u32, out: &mut ObservationSample) -> bool {
        if tap_index != 0 {
            return false;
        }
        out.left = band_aggregate_db(&self.instance.sides[0], 0);
        out.right = band_aggregate_db(&self.instance.sides[1], 0);
        true
    }

    // REALTIME_POLICY_BEGIN
    fn reset(&mut self, kind: ResetKind) {
        self.instance.reset(kind);
    }
    // REALTIME_POLICY_END

    /// Issue #1871: `Instance::designed_channel_symmetry` on lane 0.
    fn channel_symmetry(&self) -> bool {
        self.instance.designed_channel_symmetry(0)
    }

    /// Issue #1367 D3: `Instance::retarget_link` on track 0.
    fn retarget_link(&mut self, track: usize, mode: LinkMode, samples: u32) -> bool {
        track == 0 && self.instance.retarget_link(0, mode, samples)
    }

    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        let mut report = ProcessReport::default();
        self.instance.apply_automation(
            0,
            block.automation,
            self.automation_capacity,
            block.first_sample,
            &mut report,
        );
        let frames = block.left.len();
        if frames == 0 || block.right.len() != frames {
            return report;
        }
        let mut reports = [report];
        render::<f32, 1, false>(
            &mut self.instance,
            block.left,
            block.right,
            frames,
            &mut reports,
        );
        reports[0]
    }

    // REALTIME_POLICY_BEGIN: #1278 D3, the payload calls run in the plan-swap block.
    fn snapshot_state_payload(
        &self,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.instance.snapshot(0, output, self.state_sizes)
    }

    fn restore_state_payload(
        &mut self,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.instance.restore(0, state_layout_version, input)
    }
    // REALTIME_POLICY_END
}

impl<L: Lane, const W: usize> PreparedNativeEffectBank for PreparedMultibandCompressorBank<L, W> {
    // REALTIME_POLICY_BEGIN
    /// Issue #2131: [`Instance::reset_lane`], withdrawing the silent claim as
    /// [`reset`](PreparedNativeEffectBank::reset) withdraws it.
    fn reset_lane(&mut self, lane: usize, kind: ResetKind) {
        if lane >= W {
            return;
        }
        self.silent_fixed_point = false;
        self.instance.reset_lane(lane, kind);
    }
    // REALTIME_POLICY_END
    /// Every field is inline: this instance owns no heap (#1074).
    fn retained_bytes(&self) -> u64 {
        0
    }

    fn observe_resident_bank(&self, tap_index: u32, out: &mut [ObservationSample]) -> bool {
        if tap_index != 0 || out.len() != W || W != L::WIDTH {
            return false;
        }
        for (lane, sample) in out.iter_mut().enumerate() {
            sample.left = band_aggregate_db(&self.instance.sides[0], lane);
            sample.right = band_aggregate_db(&self.instance.sides[1], lane);
        }
        true
    }

    fn reset(&mut self, kind: ResetKind) {
        self.silent_fixed_point = false;
        self.instance.reset(kind);
    }

    /// Issue #893: [`Side::rest_lanes`] on both channels; the left side alone on a collapsed bank
    /// (#1894).
    fn lanes_at_rest(&self) -> u32 {
        self.instance.lanes_at_rest()
    }

    /// Issue #893: the clocks are the two channels' silence counters and nothing else
    /// ([`Side::rest_lanes`]). A masked lane takes `min(run + frames, 2^24)` on both channels (on
    /// the left alone on a collapsed bank, #1894), which is what a rendered `+0.0` block leaves; a
    /// prepared bypass freezes the counters, so it advances nothing. A rendered quiet block after
    /// which every lane is at rest earns `silent_fixed_point`, so a mask that covers the whole bank
    /// earns it too.
    fn advance_clocks(&mut self, lanes_mask: u32, frames: u32) {
        let bank = all_lanes::<W>();
        let bits = lanes_mask & bank;
        debug_assert_eq!(bits, lanes_mask, "a lane past the bank");
        debug_assert_eq!(
            self.instance.lanes_at_rest() & bits,
            bits,
            "a lane not at rest"
        );
        self.instance.advance_silence(bits, frames as usize);
        if bits == bank {
            self.silent_fixed_point = true;
        }
    }

    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        self.process_bank_inner::<false>(block)
    }

    /// Issue #1871: `Instance::designed_channel_symmetry`.
    fn lane_channel_symmetry(&self, lane: usize) -> bool {
        self.instance.designed_channel_symmetry(lane)
    }

    /// Issue #1367 D3: `Instance::retarget_link`; a padded lane takes no glide. An accepted glide
    /// withdraws the silent fixed-point claim (the next quiet block that ends at rest re-earns
    /// it).
    fn retarget_link(&mut self, track: usize, mode: LinkMode, samples: u32) -> bool {
        let accepted = self.instance.retarget_link(track, mode, samples);
        if accepted {
            self.silent_fixed_point = false;
        }
        accepted
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

    fn channels_agree(&self) -> bool {
        self.instance.channels_bit_equal()
    }

    // REALTIME_POLICY_BEGIN: #1278 D3, the payload calls run in the plan-swap block.
    fn snapshot_track_state_payload(
        &self,
        track_index: u32,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        let track = checked_track(track_index, W, self.instance.active)?;
        self.instance.snapshot(track, output, self.state_sizes)
    }

    fn restore_track_state_payload(
        &mut self,
        track_index: u32,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        let track = checked_track(track_index, W, self.instance.active)?;
        // A restore can write any state word, so the claim goes whatever the result.
        self.silent_fixed_point = false;
        self.instance.restore(track, state_layout_version, input)
    }
    // REALTIME_POLICY_END
}

impl<L: Lane, const W: usize> PreparedMultibandCompressorBank<L, W> {
    /// The one bank body, dual or collapsed (issue #1871).
    ///
    /// `MONO` chooses the render body and the sides the silent path reads, nothing else: the
    /// guards, the automation drain (both channels' spans, as a dual block takes them) and the
    /// reports are the same statements in the same order. A collapsed block's quiet test, its
    /// fixed-point skip and the claim it re-earns read the left plane and the left side, which is
    /// the dual predicate on `L == R` with the right conjunct restated as the left one. A claim
    /// earned collapsed holds on a dual block too, because a disengage copies the resting left
    /// side onto the right before the first one. Inlined into each caller so the dual and the
    /// collapsed body stay two symbols, `process_bank` and `process_bank_mono`.
    #[inline(always)]
    fn process_bank_inner<const MONO: bool>(
        &mut self,
        block: EffectBankProcessBlock<'_>,
    ) -> BankProcessReport {
        let mut report = BankProcessReport::empty(self.width);
        // `EffectBankProcessBlock::new` has already validated the block's shape, its automation
        // offsets and its frame count against the quantum (#94 F11); what is left is the two
        // facts that belong to this instance rather than to the block.
        if block.width != self.width || block.sidechain.is_some() {
            return report;
        }
        if MONO {
            // From here until the disengage copy the right side is frozen and stale.
            self.instance.collapsed = true;
        }
        // An automation span can start a ramp or move a word; the next settled quiet block
        // re-earns the claim.
        if !block.automation.is_empty() {
            self.silent_fixed_point = false;
        }
        for track in 0..W {
            // The planner gives a padded lane no span; one that arrives is not its to apply.
            if self.instance.active & (1 << track) == 0 {
                continue;
            }
            let start = block.automation_offsets[track] as usize;
            let end = block.automation_offsets[track + 1] as usize;
            self.instance.apply_automation(
                track,
                &block.automation[start..end],
                self.automation_capacity,
                block.first_sample,
                &mut report.reports[track],
            );
        }
        let frames = block.frames as usize;
        let words = frames * W;
        // Issue #893, whole-bank: no ramp in flight on either channel (no sidechain, and the
        // prepared bypass is fixed at preparation -- a session bypass is the rack's shunt, #1340
        // -- so neither can differ from the block that earned the claim) and
        // both input planes exactly `+0.0`. `-0.0` and subnormal words are not quiet.
        let quiet = self.instance.sides[0].no_ramp_in_flight()
            && (MONO || self.instance.sides[1].no_ramp_in_flight())
            && block_is_positive_zero(&block.left[..words])
            && (MONO || block_is_positive_zero(&block.right[..words]));
        let sides = if MONO { 1 } else { 2 };
        if quiet && self.silent_fixed_point {
            // Every lane is at rest and the planes already hold the `+0.0` the kernel would
            // write; the kernel would move no word but the silence counters, and no lane fails
            // the boundary check on `+0.0`.
            self.instance
                .advance_silence_sides(all_lanes::<W>(), frames, sides);
            return report;
        }
        render_body::<L, W, false, MONO>(
            &mut self.instance,
            block.left,
            block.right,
            frames,
            &mut report.reports,
        );
        let at_rest = if MONO {
            self.instance.sides[0].rest_lanes()
        } else {
            self.instance.lanes_at_rest()
        };
        self.silent_fixed_point = quiet && at_rest == all_lanes::<W>();
        report
    }
}

/// A lane past the width and a padded lane (a clear bit of `active`, issue #1893) are both not a
/// track: neither has a payload to snapshot or restore.
fn checked_track(track_index: u32, width: usize, active: u32) -> Result<usize, StatePayloadError> {
    let track = usize::try_from(track_index).map_err(|_| state_error("effect.state.track"))?;
    if track >= width || active & (1 << track) == 0 {
        return Err(state_error("effect.state.track"));
    }
    Ok(track)
}

#[cfg(test)]
mod reset_tests {
    use super::*;

    fn defaults() -> [InitialParameterValue; PARAMETER_COUNT * 2] {
        core::array::from_fn(|index| InitialParameterValue {
            parameter_index: (index / 2) as u32,
            channel: if index.is_multiple_of(2) {
                ParameterChannel::Left
            } else {
                ParameterChannel::Right
            },
            value: MULTIBAND_COMPRESSOR_PARAMETERS[index / 2].default_value,
        })
    }

    fn metadata(values: &[InitialParameterValue; PARAMETER_COUNT * 2]) -> PreparedEffectMetadata {
        expected_prepared_metadata(
            &MULTIBAND_COMPRESSOR_DESCRIPTOR,
            PrepareEffectRequest {
                sample_rate: 48_000,
                quantum: 128,
                quality: EffectQuality::Normal,
                bypass: false,
                link_mode: LinkMode::DualMono,
                ports: effect_contract::PreparedPorts {
                    sidechain: effect_contract::PreparedSidechainPort::None,
                },
                initial_values: values,
                limits: effect_contract::PrepareEffectLimits {
                    maximum_total_state_bytes: u64::MAX,
                    maximum_scratch_bytes: u64::MAX,
                    maximum_automation_spans_per_block: 32,
                },
                tail_bound: conformance::tail_bound_of(
                    Box::new(crate::MultibandCompressorFactory),
                    48_000,
                    EffectQuality::Normal,
                ),
            },
        )
        .expect("test metadata")
    }

    fn snapshot(
        instance: &Instance<f32, 1>,
        sizes: StatePayloadSizes,
    ) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        let mut common = vec![0; sizes.common_bytes as usize];
        let mut left = vec![0; sizes.left_bytes as usize];
        let mut right = vec![0; sizes.right_bytes as usize];
        instance
            .snapshot(
                0,
                StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes)
                    .expect("snapshot sizes"),
                sizes,
            )
            .expect("snapshot");
        (common, left, right)
    }

    #[test]
    fn full_reset_restores_prepared_crossover_coefficients() {
        let values = defaults();
        let prepared_metadata = metadata(&values);
        let sizes = prepared_metadata.state_sizes;
        let (left, right) = initial_defaults(&values).expect("defaults");
        let mut receiver =
            Instance::<f32, 1>::new([left], [right], prepared_metadata).expect("receiver instance");
        let (common, mut restored_left, mut restored_right) = snapshot(&receiver, sizes);
        // A settled 2 kHz crossover: the target and its designed words (issue #1338).
        for bytes in [&mut restored_left, &mut restored_right] {
            write_f32(bytes, 0, 2_000.0);
            for (word, value) in design_lr4_words(48_000, 2_000.0).into_iter().enumerate() {
                write_ramp(
                    bytes,
                    CROSSOVER_WORD + word * RAMP_WORDS,
                    LinearRamp::fixed(value),
                );
            }
        }
        receiver
            .restore(
                0,
                STATE_LAYOUT_VERSION,
                StatePayloadInput::new(&common, &restored_left, &restored_right, sizes)
                    .expect("restore sizes"),
            )
            .expect("different-crossover restore");
        receiver.reset(ResetKind::FullToDefaults);

        let fresh_metadata = metadata(&values);
        let (fresh_left_defaults, fresh_right_defaults) =
            initial_defaults(&values).expect("fresh defaults");
        let mut fresh = Instance::<f32, 1>::new(
            [fresh_left_defaults],
            [fresh_right_defaults],
            fresh_metadata,
        )
        .expect("fresh instance");
        let mut receiver_left = vec![0.25f32; 64];
        let mut receiver_right = vec![-0.125f32; 64];
        let mut fresh_left = receiver_left.clone();
        let mut fresh_right = receiver_right.clone();
        let mut receiver_reports = [ProcessReport::default()];
        let mut fresh_reports = [ProcessReport::default()];
        render::<f32, 1, false>(
            &mut receiver,
            &mut receiver_left,
            &mut receiver_right,
            64,
            &mut receiver_reports,
        );
        render::<f32, 1, false>(
            &mut fresh,
            &mut fresh_left,
            &mut fresh_right,
            64,
            &mut fresh_reports,
        );
        assert_eq!(receiver_left, fresh_left, "full reset left PCM");
        assert_eq!(receiver_right, fresh_right, "full reset right PCM");
        assert_eq!(
            snapshot(&receiver, sizes),
            snapshot(&fresh, sizes),
            "full reset state payload"
        );
    }
}

#[cfg(test)]
mod knee_tests {
    //! Issue #994: the bands' static curve under the workspace's one knee design.
    //!
    //! The product knee is a fixed 6 dB, so no session, control message or automation can give a
    //! band the overflowing widths the compressor's knee parameter admitted. What these tests pin
    //! is that the band curve takes its knee words from `knee_coefficients` — so the fixed knee is
    //! unchanged and a future knee parameter inherits the overflow rule — and that the band's own
    //! target computation, `band_target`, gives a finite, hard-knee target at exactly the
    //! threshold for the verification's 2.8e-45 and for every width around the bound.

    use super::*;
    use effect_runtime::dynamics::MIN_SOFT_KNEE_DB;

    /// The compile-time knee words are the ones this crate wrote inline before #994, bit for bit.
    ///
    /// Red mutation (MUTATIONS.md row 994-M2): `BAND_KNEE = knee_coefficients(2.0 * KNEE_DB)`.
    #[test]
    fn the_fixed_knee_words_are_unchanged() {
        let knee = core::hint::black_box(KNEE_DB);
        assert_eq!(BAND_KNEE.0.to_bits(), (0.5 * knee).to_bits());
        assert_eq!(BAND_KNEE.1.to_bits(), (1.0 / (2.0 * knee)).to_bits());
        const {
            assert!(
                KNEE_DB >= MIN_SOFT_KNEE_DB,
                "the product knee is a soft knee"
            );
        }
    }

    fn target_bits<L: Lane>(level: f32, threshold: f32, ratio: f32, knee: (f32, f32)) -> u32 {
        let mut words = [0_u32; 8];
        band_target::<L>(
            L::splat(level),
            L::splat(threshold),
            L::splat(1.0 / ratio - 1.0),
            knee,
        )
        .store_bits(&mut words[..L::WIDTH]);
        for word in &words[1..L::WIDTH] {
            assert_eq!(*word, words[0], "lanes of a splatted input disagree");
        }
        words[0]
    }

    /// At the old failing knee (2.8e-45) and at every width around the overflow bound, a band
    /// level exactly at the threshold — and a few subnormal steps and ulps either side of it —
    /// gets a finite target in `[-100, 0]`, identical at every lane width, and never the
    /// `-100 dB` the reduction clamp made of the NaN. An overflowing width gets the hard knee's
    /// target bit for bit; at exactly the threshold every one of these widths changes nothing.
    ///
    /// Red mutation (MUTATIONS.md row 994-M1): drop the `is_finite` test in
    /// `effect_runtime::dynamics::knee_coefficients`.
    #[test]
    fn a_band_level_at_the_threshold_never_takes_a_nan_target() {
        let widest_overflowing = f32::from_bits(0x0010_0000);
        for knee_bits in [
            0x0000_0001_u32,
            0x0000_0002,
            0x000F_FFFF,
            0x0010_0000,
            0x0010_0001,
            0x0010_0002,
        ] {
            let knee = f32::from_bits(knee_bits);
            let words = knee_coefficients(knee);
            for threshold in [0.0_f32, -0.0, -1.0e-45, -18.0, -80.0] {
                for ratio in [1.0_f32, 1.5, 4.0, 20.0] {
                    let mut levels = vec![threshold];
                    for step in 1..=3_u32 {
                        levels.push(threshold + f32::from_bits(step));
                        levels.push(threshold - f32::from_bits(step));
                    }
                    levels.push(threshold.next_up());
                    levels.push(threshold.next_down());
                    for level in levels {
                        let scalar = target_bits::<f32>(level, threshold, ratio, words);
                        let target = f32::from_bits(scalar);
                        assert!(
                            target.is_finite() && (-100.0..=0.0).contains(&target),
                            "W {knee_bits:#010x} T {threshold:e} R {ratio} x {level:e}: {target}"
                        );
                        assert!(
                            target > -1.0e-3,
                            "W {knee_bits:#010x} T {threshold:e} R {ratio} x {level:e}: {target} \
                             is a duck at the threshold"
                        );
                        lane::each_vector_lane!(|L| assert_eq!(
                            scalar,
                            target_bits::<L>(level, threshold, ratio, words)
                        ));
                        if knee <= widest_overflowing {
                            assert_eq!(
                                scalar,
                                target_bits::<f32>(level, threshold, ratio, (0.0, 0.0)),
                                "W {knee_bits:#010x} T {threshold:e} R {ratio} x {level:e}: \
                                 an overflowing width is the hard knee"
                            );
                        }
                    }
                    let at = f32::from_bits(target_bits::<f32>(threshold, threshold, ratio, words));
                    assert!(
                        at == 0.0,
                        "W {knee_bits:#010x} T {threshold:e} R {ratio}: {at} at the threshold"
                    );
                }
            }
        }
    }
}
