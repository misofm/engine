//! Launch hysteretic peak gate/downward-expander.
//!
//! One generic kernel body owns the whole per-sample graph ([`kernel`]); this module is its
//! control plane — the frozen descriptor, preparation, parameter smoothing, the state payload and
//! the once-per-block boundary policy. Everything scaffolding-shaped comes from
//! `effect-runtime`, everything transcendental from `math` and every
//! vector operation from `lane`; this crate keeps only the gate's own equations.
//!
//! # Shapes
//!
//! A prepared instance is `PreparedGate<L, CONNECTED>`. The scalar instance is `L = f32`, which is
//! literally the `WIDTH = 1` instantiation of the same kernel, so it is the oracle for the four-
//! and eight-lane banks rather than a second implementation of the same arithmetic. `CONNECTED`
//! selects the current connected sidechain source; it does not change timing or allocate history.
//!
//! # Realtime rules
//!
//! Preparation allocates the instance and fixed control state only. `process` and `process_bank`
//! touch no audio history, take no lock, make no syscall and call no platform libm.
//!
//! # Live attack, hold and release (#1336)
//!
//! All seven parameters are live block-rate values. Threshold, ratio, range and hysteresis ramp
//! their value over 64 samples. Attack and release commit their time at once and ramp the one-pole
//! coefficient instead: from the coefficient in effect `c0` to the design
//! `c1 = 1 - exp(-1000 / (t fs))` (`effect_runtime::envelope::attack_release_coefficient`, the
//! convention of Giannoulis, Massberg and Reiss, JAES 2012), `c[k] = ramp_toward(c[k-1],
//! (c1 - c0) / 64, c1)` and `c[64] = c1` exactly, as the compressor's coefficient ramps do. The
//! envelope `g[n] = flush(g[n-1] + c[n] (C[n] - g[n-1]))` is a one-pole with `0 < c < 2`, and every
//! `c[k]` lies between the two designs, inside `[5.0e-6, 0.203]` over the launch domains and rates,
//! so it is stable and never overshoots its target: a change makes no step a fixed design would not
//! make. Hold is an integer count that moves no sample: a change takes effect at once and keeps
//! the hold window's start (`PreparedGate::retime_hold`). A record costs at most two vendored
//! `exp` calls on the block's control side; a coefficient ramp in flight costs two lane selects and
//! adds per sample. The coefficient is a normal number, so its step is normal or zero, the envelope
//! keeps its flush, and a non-finite gain still trips the lane-local D7 recovery. The state payload
//! carries both coefficient ramps, so a ramp in flight crosses a plan swap bit-exactly.
#![allow(missing_docs)]

pub mod corpus;
pub mod kernel;

use effect_contract::{
    AutomationRate, BankProcessReport, BankWidth, EffectBankProcessBlock, EffectDescriptor,
    EffectPrepareError, EffectProcessBlock, EffectQuality, InitialParameterValue, LatencySamples,
    LinkMode, LinkModeSet, NativeEffectFactory, ObservationCadence, ObservationChannels,
    ObservationCost, ObservationDescriptor, ObservationFold, ObservationKind, ObservationSample,
    ObservationTapId, ParameterChannel, ParameterChannelPolicy, ParameterDescriptor,
    ParameterDomain, ParameterId, ParameterMapping, ParameterUnit, PortDescriptor, PortId,
    PortLayout, PortRole, PrepareEffectBankRequest, PrepareEffectRequest, PreparedAutomationSpan,
    PreparedBankMetadata, PreparedEffect, PreparedEffectBank, PreparedEffectMetadata,
    PreparedNativeEffect, PreparedNativeEffectBank, PreparedSidechainPort, ProcessReport,
    ResetKind, SmoothingRule, StatePayloadError, StatePayloadInput, StatePayloadOutput,
    StatePayloadSizes, TailSamples, expected_prepared_metadata,
};
use effect_runtime::bank::{block_is_positive_zero, check_block, nonfinite_lane_mask};
use effect_runtime::envelope::attack_release_coefficient;
use effect_runtime::params::{
    ParameterSpec, is_negative_zero, normalize_zero, parameter_value_valid,
};
use effect_runtime::ramp::LinearRamp;
use effect_runtime::state_payload as payload;
use lane::{Backend, Lane, Simd4};

use kernel::{
    GateArgs, GateCoef, GateState, LINK_COUNT, MAX_WIDTH, RAMP_COUNT, RATE_COUNT, gate_frames,
};

const PARAMETER_COUNT: usize = 7;

/// Effect-owned words at the front of each channel section of the state payload: gain, open flag,
/// hold countdown, the three times, four parameter ramp quadruples, two rate ramp quadruples
/// (#1336 D6) and the two link weight quadruples (#1370 D6). The lane's link mode is the weights'
/// targets (`effect_contract::link_glide_mode`), so it takes no word of its own.
const STATE_LANE_HEADER_WORDS: usize = 38;

/// Samples every smoothed parameter takes to reach a new target, from the descriptor.
const RAMP_SAMPLES: u32 = 64;

/// The lane word that encodes an open gate. A closed one is `+0.0`.
const OPEN_WORD: f32 = 1.0;

#[cfg(test)]
thread_local! {
    /// Blocks the silent fast path skipped (issue #1746), counted once per skipped block.
    static SILENT_ADMISSIONS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
    /// While set, the silent fast path is never taken: the control arm of the fast-path
    /// differential and the rest gate's fork render the whole kernel.
    static SILENT_FORBIDDEN: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}

/// Counts one silent admission. Test builds only: production has no counter.
#[cfg(test)]
fn admitted_silent() {
    SILENT_ADMISSIONS.with(|blocks| blocks.set(blocks.get() + 1));
}

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

/// Frozen V1 gate/expander parameters. Descriptor position and stable numeric ID agree.
pub const GATE_EXPANDER_PARAMETERS: [ParameterDescriptor; PARAMETER_COUNT] = [
    parameter(
        1,
        "threshold_db",
        "threshold",
        "dB",
        ParameterUnit::Db,
        -80.0,
        0.0,
        -40.0,
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
        "range_db",
        "range",
        "dB",
        ParameterUnit::Db,
        0.0,
        96.0,
        80.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        4,
        "hysteresis_db",
        "hysteresis",
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
        5,
        "attack_ms",
        "attack",
        "ms",
        ParameterUnit::Milliseconds,
        0.1,
        50.0,
        1.0,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        6,
        "hold_ms",
        "hold",
        "ms",
        ParameterUnit::Milliseconds,
        0.0,
        1000.0,
        100.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::None,
        0,
    ),
    parameter(
        7,
        "release_ms",
        "release",
        "ms",
        ParameterUnit::Milliseconds,
        5.0,
        2000.0,
        100.0,
        ParameterMapping::Logarithmic,
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

/// Causal quality resources: 38 words per channel, the common two-word codec header and no delay.
const fn quality(sample_rate: u32) -> effect_contract::QualityDescriptor {
    let per_lane = (STATE_LANE_HEADER_WORDS as u32) * 4;
    let common = payload::HEADER_WORDS * payload::WORD_BYTES as u32;
    effect_contract::QualityDescriptor {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(0),
        maximum_state: StatePayloadSizes {
            common_bytes: common,
            left_bytes: per_lane,
            right_bytes: per_lane,
        },
        scratch_fixed_bytes: 64,
        scratch_bytes_per_frame: 0,
    }
}

/// The fast tier's gain error, in dB: `fast_gain_from_db` is within this of `10^(g/20)` for every
/// `g` in `[-160, 24]` dB, the bound `crates/math/tests/f1_fast_db_bounds.rs` proves by an
/// exhaustive sweep (at most `7.431e-6` dB on `[-160, 0]`). The applied gain's argument, the gain
/// word, lies in `[-96, 0]` dB. The same value as the compressor's, declared here because
/// `math::fast_db` is sealed (#1376).
const FAST_GAIN_ERROR_DB: f64 = 1.0e-5;

/// The smallest release rate coefficient over the parameter domain at `sample_rate`: the design of
/// the longest release, less `2^-24`. The design is `1 - expf(-1 / tau)` with `expf` in `[1/2, 1)`,
/// so every release's coefficient is a multiple of `2^-24`, and `expf` is within one ulp of
/// `e^x`: a shorter release can come out at most one `2^-24` step below the longest's, never two.
fn slowest_release_rate(sample_rate: u32) -> f32 {
    let longest = GATE_SPECS[6].maximum;
    let c = attack_release_coefficient(longest, sample_rate);
    c - 1.0 / 16_777_216.0
}

/// The smallest attack rate coefficient over the parameter domain, by the same rule as
/// [`slowest_release_rate`]: the 50 ms design less `2^-24`.
fn slowest_attack_rate(sample_rate: u32) -> f32 {
    let longest = GATE_SPECS[4].maximum;
    let c = attack_release_coefficient(longest, sample_rate);
    c - 1.0 / 16_777_216.0
}

/// The exact-rest bound `R` at `sample_rate` (#1376 D2), a [`effect_contract::RestBound::FixedPoint`] bound (root
/// option (a)), or `None` when it does not exist.
///
/// **Quantifier.** `R` holds when every input port is exactly zero from the first silent sample
/// `M` on: the main input and the sidechain. With a zero detector the level clamps to
/// [`kernel::LEVEL_MIN_DB`] (`-160` dB), below the opening threshold (at least `-80` dB) and below
/// the re-arm threshold (at least `-80 - 24` dB), so no reload happens: an open gate holds for at
/// most its hold count, at most `rounded_samples(1000 ms)`, and then closes with `hold = +0.0`;
/// a closed gate always has `hold = +0.0` (closing needs it, and only an opening reloads it). A
/// ramp in flight at `M` settles within 64 samples, under every rate's longest hold; that includes
/// an attack or release coefficient ramp (#1336), whose every word lies between two designs, both
/// at or above the slowest rate's, so the walk bounds below still hold. From then on
/// the target `t = min(max((ratio - 1)(-160 - threshold), -range), 0)` is one fixed word.
///
/// Closed, the gain word walks toward `t` and stops on a word whose step rounds away. Two cases:
///
/// * **From above** (`t < e <= 0`, at the release rate): it stops at the **stall word** `w*`,
///   within [`effect_runtime::tail::rate_follower_stall_samples`] steps, and `w*` is the same for
///   every start in `[w*, 0]`. Every history with fixed parameters is in this case and ends on
///   `w*`: the targets any input produces are at or above `t` (the level is at least `-160` dB and
///   the curve is monotone), the walk never overshoots, and a walk toward a shallower target stops
///   at or above that target's own stall word. A reset (open, hold loaded) is too, so this `w*` is
///   the fresh instance's `Z = (w*, open = +0.0, hold = +0.0)`.
/// * **From below** (`e < t`, at the attack rate): a parameter edit before `M` (a range, ratio or
///   threshold that makes the target shallower, a ramp in flight included) can leave the word
///   deeper than `t`. It then rises and stops on a fixed point on the far side of `t`, within
///   [`effect_runtime::tail::rate_follower_approach_samples`] steps from a distance of at most
///   96 dB (the word lies in `[-96, 0]`). A word an edit leaves between `t` and `w*` is already at,
///   or one step from, a fixed point, inside either bound.
///
/// So from `M + R`, `R = max(H_max, 64) + max(walk from above, walk from below)`, every signal-state
/// word is at a zero-input fixed point. It equals `Z` for every history whose parameters were
/// settled before the gate closed, but not for an edited one, which is why the bound is stated as a
/// fixed point. A snap of the word to its target would make `Z` unique for every history and is a
/// rendered-bit change, an owner question.
fn rest_samples(sample_rate: u32) -> Option<u64> {
    let hold = u64::from(rounded_samples(GATE_SPECS[5].maximum, sample_rate)?);
    let above =
        effect_runtime::tail::rate_follower_stall_samples(slowest_release_rate(sample_rate))?;
    let below = effect_runtime::tail::rate_follower_approach_samples(
        slowest_attack_rate(sample_rate),
        GATE_SPECS[2].maximum,
        lane::FLUSH_EPS,
    )?;
    hold.max(u64::from(RAMP_SAMPLES))
        .checked_add(above.max(below))
}

/// `G_p = G_t` in millibels (#1376 D4): the output is the input `x`, or `x g` with
/// `g = fast_gain_from_db(gain)`, `gain <= 0`, so `g <= 10^(FAST_GAIN_ERROR_DB / 20)`, through one
/// rounded product. A sidechain feeds only the detector, so the bound holds for every sidechain
/// input; after silence the output is the same law, so `G_t = G_p`.
fn peak_gain_millibels() -> i32 {
    let gain = math::db_to_gain(FAST_GAIN_ERROR_DB) * (1.0 + effect_runtime::tail::ROUNDOFF);
    math::tail::ceil_millibels(gain, math::tail::Rounding::Computed)
}

/// `sigma_p = sigma_t` in millibels (#1376 D4, #1379 H3): the underflow allowance of the one
/// rounded output product.
fn flush_stall_millibels() -> i32 {
    math::tail::ceil_millibels(math::tail::UNDERFLOW, math::tail::Rounding::Exact)
}

/// This effect's tail, tail over every peak, exact-rest bound and composition values, the one place
/// they are stated (decision 15 D15-4(b), #1377 D1, #1376).
///
/// * **Tail `Finite(0)`, decay `0`** (#1376 D1): the output is `select(identity, x, x g)` of the
///   current main input `x`: no audio history, zero latency. A sidechain feeds only the detector.
/// * **Rest** [`rest_samples`], a fixed-point bound (the rest state depends on the history of
///   parameter edits), the same for both peak cases (the walk's start does not depend on the
///   input); **tail over every peak** the same `R`.
/// * **Composition** [`peak_gain_millibels`] and [`flush_stall_millibels`].
fn tail_and_rest(sample_rate: u32, _quality: EffectQuality) -> effect_contract::NodeTailBound {
    let Some(rest) = rest_samples(sample_rate) else {
        return effect_contract::NodeTailBound::UNBOUNDED;
    };
    let gain = effect_contract::PeakGain::Millibels(peak_gain_millibels());
    let stall = effect_contract::FlushStall::Level(flush_stall_millibels());
    effect_contract::NodeTailBound {
        tail: TailSamples::Finite(0),
        tail_every_peak: TailSamples::Finite(rest),
        rest: effect_contract::RestBound::FixedPoint(effect_contract::RestSamples {
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

/// State payload layout version.
///
/// State layout identity retained by the amended prelaunch descriptor.
pub const STATE_LAYOUT_VERSION: u32 = 1;

/// The one declared observation tap: the branching smoother's own gain word.
///
/// `GateState::gain_db` is what `gate_block` writes every sample and reads back on the next one,
/// in decibels and negative for reduction -- a closed gate holds a large negative number and an
/// open one holds `+0.0`. Publishing it is a copy out of state the block wrote anyway.
pub const GATE_EXPANDER_OBSERVATIONS: [ObservationDescriptor; 1] = [ObservationDescriptor {
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

/// Immutable descriptor for the launch gate/expander contract.
pub const GATE_EXPANDER_DESCRIPTOR: EffectDescriptor = EffectDescriptor {
    id: effect_id("miso.gate-expander"),
    display_name: "Gate / Expander",
    contract_major: 1,
    // Issue #143 P1: declaring the first tap is a `contract_minor` bump and a derived identity
    // re-pin of exactly `32 + len("Gain Reduction") + len("dB")` = 48 bytes.
    // `state_layout_version` does not move: the tap reads state that was already there.
    contract_minor: 1,
    state_layout_version: STATE_LAYOUT_VERSION,
    supported_link_modes: LinkModeSet::ALL,
    lane_link: true,
    parameters: &GATE_EXPANDER_PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
    tail_and_rest,
    observations: &GATE_EXPANDER_OBSERVATIONS,
};

/// The parameter domains, in the runtime's vocabulary.
///
/// The descriptor stays the contract; this is the same table in the shape
/// `effect_runtime::params` validates against, so the gate has no domain check of its
/// own. `the_runtime_parameter_specs_agree_with_the_frozen_descriptor` asserts the two cannot drift.
const GATE_SPECS: [ParameterSpec; PARAMETER_COUNT] = [
    ParameterSpec::continuous(-80.0, 0.0, -40.0),
    ParameterSpec::logarithmic(1.0, 20.0, 4.0),
    ParameterSpec::continuous(0.0, 96.0, 80.0),
    ParameterSpec::continuous(0.0, 24.0, 6.0),
    ParameterSpec::logarithmic(0.1, 50.0, 1.0),
    ParameterSpec::continuous(0.0, 1000.0, 100.0),
    ParameterSpec::logarithmic(5.0, 2000.0, 100.0),
];

/// Factory for the launch gate/expander.
#[derive(Clone, Copy, Debug, Default)]
pub struct GateExpanderFactory;

/// The three unsmoothed times of one lane and channel, kept for state restore and coefficient
/// re-derivation.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct LaneTiming {
    attack_ms: f32,
    hold_ms: f32,
    release_ms: f32,
}

/// A prepared gate at one lane width.
///
/// `CONNECTED` selects whether the current detector words come from the sidechain.
///
/// It keeps only the prepared values its own methods read (issue #1461): the whole
/// `PreparedEffectMetadata` stays on the control side, in the prepare result. The prepared bypass
/// and link mode are already folded into `coef`. It keeps no quantum: it has no buffer sized by
/// one, and `EffectBankProcessBlock::new` already refuses a block longer than the quantum its
/// caller states (issue #1461, m1).
struct PreparedGate<L: Lane, const CONNECTED: bool> {
    /// The prepared sample rate: hold counts and one-pole coefficients, at seeding and restore.
    sample_rate: u32,
    /// The prepared automation capacity: a span at or past it is invalid.
    automation_capacity: u32,
    bank_width: Option<BankWidth>,
    defaults: [[[f32; PARAMETER_COUNT]; 2]; MAX_WIDTH],
    coef: [GateCoef<L>; 2],
    state: [GateState<L>; 2],
    timing: [[LaneTiming; 2]; MAX_WIDTH],
    ramp_frames_left: u32,
    /// Whether a collapsed block (`run_block::<true>`) has rendered since the last disengage copy
    /// (`desymmetrize`), which alone clears it (#1894). While it is set the right channel is
    /// frozen and stale, so [`rest_lanes`](Self::rest_lanes) reads the left channel alone: the
    /// effect contract's collapsed-bank rule. The gate's one clock is bank-level, so nothing else
    /// reads it.
    collapsed: bool,
    /// Bit `l` is set when lane `l` carries a member (issue #1092). A padded lane's bit is clear:
    /// it takes no automation, and D7 recovers it without reporting it (the padding contract on
    /// `PrepareEffectBankRequest`). Every lane of a scalar instance and of a full bank is active.
    active: u32,
}

/// Reads one lane out of a lane word.
fn lane_get<L: Lane>(value: L, lane: usize) -> f32 {
    let mut words = [0.0_f32; MAX_WIDTH];
    value.store(&mut words[..L::WIDTH]);
    words[lane]
}

/// Writes one lane of a lane word, leaving the others alone. Control plane only.
fn lane_set<L: Lane>(value: &mut L, lane: usize, sample: f32) {
    let mut words = [0.0_f32; MAX_WIDTH];
    value.store(&mut words[..L::WIDTH]);
    words[lane] = sample;
    *value = L::load(&words[..L::WIDTH]);
}

/// `floor(ms * fs / 1000 + 0.5)`, the frozen sample-count rounding of brief 014.
fn rounded_samples(milliseconds: f32, sample_rate: u32) -> Option<u32> {
    let samples = (f64::from(milliseconds) * f64::from(sample_rate) / 1000.0 + 0.5).floor();
    if !samples.is_finite() || samples < 0.0 || samples > f64::from(u32::MAX) {
        return None;
    }
    Some(samples as u32)
}

/// `true` for a finite value that is either zero or normal.
fn normal_or_zero(value: f32) -> bool {
    value.is_finite() && (value == 0.0 || value.is_normal())
}

/// `true` for a finite value that is a whole number in `[0, limit]`.
fn integral_within(value: f32, limit: f32) -> bool {
    value.is_finite() && value >= 0.0 && value <= limit && value == value.floor()
}

const fn state_error(code: &'static str) -> StatePayloadError {
    StatePayloadError { code }
}

/// Maps the runtime codec's error onto the contract's, preserving the code.
fn codec_error(error: payload::StatePayloadError) -> StatePayloadError {
    StatePayloadError { code: error.code }
}

impl<L: Lane, const CONNECTED: bool> PreparedGate<L, CONNECTED> {
    /// The payload layout of one prepared shape.
    fn layout(&self) -> payload::StateLayout {
        payload::StateLayout {
            version: STATE_LAYOUT_VERSION,
            common_words: 0,
            lane_words: STATE_LANE_HEADER_WORDS as u32,
        }
    }

    /// Builds one prepared shape. `defaults[track][channel]` holds the validated initial values;
    /// only `[..WIDTH]` is read.
    fn new(
        metadata: &PreparedEffectMetadata,
        bank_width: Option<BankWidth>,
        defaults: [[[f32; PARAMETER_COUNT]; 2]; MAX_WIDTH],
    ) -> Option<Self> {
        let width = L::WIDTH;
        let connected = matches!(
            metadata.ports.sidechain,
            PreparedSidechainPort::Connected { .. }
        );
        if connected != CONNECTED {
            return None;
        }
        let mut gate = Self {
            sample_rate: metadata.sample_rate,
            automation_capacity: metadata.automation_capacity,
            bank_width,
            defaults,
            coef: [GateCoef {
                hold_samples: L::zero(),
                bypass: L::splat(if metadata.bypass { 1.0 } else { 0.0 }),
                link_max: L::splat(f32::from(u8::from(metadata.link_mode == LinkMode::Maximum))),
                link_avg: L::splat(f32::from(u8::from(metadata.link_mode == LinkMode::Average))),
            }; 2],
            state: [GateState::default(); 2],
            timing: [[LaneTiming::default(); 2]; MAX_WIDTH],
            ramp_frames_left: 0,
            collapsed: false,
            active: all_lanes(width),
        };
        for channel in 0..2 {
            for lane in 0..width {
                gate.seed_lane(channel, lane)?;
            }
        }
        for lane in 0..width {
            gate.rest_link(lane);
        }
        Some(gate)
    }

    /// Sets each lane's detector link words from that lane's own prepared mode (issue #1368 D3):
    /// `links[lane]` is the mode of the request bound to `lane`. The kernel selects the link per
    /// lane, so a lane renders its own mode's bits whatever its bank-mates' modes are.
    fn with_lane_links(mut self, links: &[LinkMode]) -> Self {
        for (lane, &link) in links.iter().enumerate().take(L::WIDTH) {
            self.set_lane_link(lane, link);
            self.rest_link(lane);
        }
        self
    }

    /// Lane `lane`'s link words on both channels, from `link`.
    fn set_lane_link(&mut self, lane: usize, link: LinkMode) {
        for coef in &mut self.coef {
            let maximum = f32::from(u8::from(link == LinkMode::Maximum));
            let average = f32::from(u8::from(link == LinkMode::Average));
            lane_set(&mut coef.link_max, lane, maximum);
            lane_set(&mut coef.link_avg, lane, average);
        }
    }

    /// The mode lane `lane`'s link words select (the left channel's; both hold the same).
    fn lane_link(&self, lane: usize) -> LinkMode {
        if lane_get(self.coef[0].link_max, lane) > 0.0 {
            LinkMode::Maximum
        } else if lane_get(self.coef[0].link_avg, lane) > 0.0 {
            LinkMode::Average
        } else {
            LinkMode::DualMono
        }
    }

    /// Marks the lanes of `active` as the bank's members and every other lane as padded.
    fn with_active_lanes(mut self, active: u32) -> Self {
        self.active = active & all_lanes(L::WIDTH);
        self
    }

    /// Whether `lane` carries a member rather than padding.
    const fn is_active(&self, lane: usize) -> bool {
        self.active & (1 << lane) != 0
    }

    /// Derives one lane's timing, coefficients and resting ramps from its prepared defaults.
    fn seed_lane(&mut self, channel: usize, lane: usize) -> Option<()> {
        let values = self.defaults[lane][channel];
        self.timing[lane][channel] = LaneTiming {
            attack_ms: values[4],
            hold_ms: values[5],
            release_ms: values[6],
        };
        self.rederive_lane(channel, lane)?;
        let hold = lane_get(self.coef[channel].hold_samples, lane);
        lane_set(&mut self.state[channel].gain_db, lane, 0.0);
        lane_set(&mut self.state[channel].hysteresis.open, lane, OPEN_WORD);
        lane_set(&mut self.state[channel].hysteresis.hold, lane, hold);
        let rates = [
            attack_release_coefficient(values[4], self.sample_rate),
            attack_release_coefficient(values[6], self.sample_rate),
        ];
        let state = &mut self.state[channel];
        let resting = values[..RAMP_COUNT].iter().chain(&rates);
        for (ramp, &value) in state.ramps.iter_mut().chain(&mut state.rates).zip(resting) {
            lane_set(&mut ramp.current, lane, value);
            lane_set(&mut ramp.target, lane, value);
            lane_set(&mut ramp.step, lane, 0.0);
            lane_set(&mut ramp.remaining, lane, 0.0);
        }
        Some(())
    }

    /// Seats lane `lane`'s link weights at rest at its mode on both channels (issue #1370 D2):
    /// preparation, a full reset and a lane whose mode a lane swap replaced. The D7 recovery
    /// (`reset_lane_full` of one channel) leaves a glide in flight alone, as the compressor's does.
    fn rest_link(&mut self, lane: usize) {
        let weights = effect_contract::link_glide_weights(self.lane_link(lane));
        for state in &mut self.state {
            for (ramp, &value) in state.links.iter_mut().zip(&weights) {
                lane_set(&mut ramp.current, lane, value);
                lane_set(&mut ramp.target, lane, value);
                lane_set(&mut ramp.step, lane, 0.0);
                lane_set(&mut ramp.remaining, lane, 0.0);
            }
        }
    }

    // REALTIME_POLICY_BEGIN: #1370, a link glide is retargeted at a block boundary.
    /// Glides lane `lane`'s detector link to `mode` over `samples` samples (issue #1370 D1-D3).
    ///
    /// The lane's link words become `mode`'s at once, so a glide that arrives lands on `mode`'s
    /// own law; the weights, retargeted from the words in force, carry the transition, and the
    /// bank's ramp clock covers it. The mode is the one a full reset keeps and a lane swap moves.
    /// `false`, changing nothing, for a lane past the width or padded, a mode the descriptor does
    /// not support or a glide longer than `LINK_GLIDE_MAX_SAMPLES`.
    fn retarget_link(&mut self, lane: usize, mode: LinkMode, samples: u32) -> bool {
        if lane >= L::WIDTH
            || !self.is_active(lane)
            || !GATE_EXPANDER_DESCRIPTOR.supported_link_modes.contains(mode)
            || samples > effect_contract::LINK_GLIDE_MAX_SAMPLES
        {
            return false;
        }
        self.set_lane_link(lane, mode);
        let weights = effect_contract::link_glide_weights(mode);
        for state in &mut self.state {
            for (slot, &weight) in state.links.iter_mut().zip(&weights) {
                let mut ramp = LinearRamp {
                    current: lane_get(slot.current, lane),
                    target: lane_get(slot.target, lane),
                    step: lane_get(slot.step, lane),
                    remaining: lane_get(slot.remaining, lane) as u32,
                };
                ramp.set_target(weight, samples);
                lane_set(&mut slot.current, lane, ramp.current);
                lane_set(&mut slot.target, lane, ramp.target);
                lane_set(&mut slot.step, lane, ramp.step);
                lane_set(&mut slot.remaining, lane, ramp.remaining as f32);
                self.ramp_frames_left = self.ramp_frames_left.max(ramp.remaining);
            }
        }
        true
    }

    /// Whether a link glide is in flight on any lane of the channels the block renders: the
    /// left alone on a collapsed block, whose right state is frozen.
    fn gliding<const MONO: bool>(&self) -> bool {
        let channels = if MONO {
            &self.state[..1]
        } else {
            &self.state[..]
        };
        channels.iter().any(|state| {
            state.links.iter().any(|ramp| {
                let mut remaining = [0.0_f32; MAX_WIDTH];
                ramp.remaining.store(&mut remaining[..L::WIDTH]);
                remaining[..L::WIDTH].iter().any(|word| *word > 0.0)
            })
        })
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN: #1278 D3, a restore commit re-derives the hold count.
    /// Recomputes one lane's hold count from its timing. The attack and release coefficients are
    /// state (`GateState::rates`, #1336 D2): seeding designs them and a restore commits them as
    /// written.
    fn rederive_lane(&mut self, channel: usize, lane: usize) -> Option<()> {
        let hold = rounded_samples(self.timing[lane][channel].hold_ms, self.sample_rate)?;
        lane_set(&mut self.coef[channel].hold_samples, lane, hold as f32);
        Some(())
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN
    /// Clears one lane of one channel back to its prepared defaults: gain, phase, hold and resting
    /// ramps. This is `ResetKind::FullToDefaults` for one lane, and it is also the D7 recovery
    /// action for a lane whose block failed the boundary check.
    fn reset_lane_full(&mut self, channel: usize, lane: usize) {
        let _ = self.seed_lane(channel, lane);
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN
    /// `ResetKind::DiscontinuityKeepParameters` for one lane: recursive state clears, smoothed
    /// values snap to their targets and unsmoothed times are kept.
    fn reset_lane_discontinuity(&mut self, channel: usize, lane: usize) {
        let hold = lane_get(self.coef[channel].hold_samples, lane);
        lane_set(&mut self.state[channel].gain_db, lane, 0.0);
        lane_set(&mut self.state[channel].hysteresis.open, lane, OPEN_WORD);
        lane_set(&mut self.state[channel].hysteresis.hold, lane, hold);
        let state = &mut self.state[channel];
        for ramp in state
            .ramps
            .iter_mut()
            .chain(&mut state.rates)
            .chain(&mut state.links)
        {
            let target = lane_get(ramp.target, lane);
            lane_set(&mut ramp.current, lane, target);
            lane_set(&mut ramp.step, lane, 0.0);
            lane_set(&mut ramp.remaining, lane, 0.0);
        }
    }
    // REALTIME_POLICY_END

    fn reset(&mut self, kind: ResetKind) {
        for lane in 0..L::WIDTH {
            self.reset_lane(lane, kind);
        }
        self.ramp_frames_left = 0;
    }

    // REALTIME_POLICY_BEGIN
    /// `PreparedNativeEffectBank::reset_lane` (issue #2131): lane `lane`'s part of
    /// [`reset`](Self::reset) on both channels, and nothing of any other lane. The bank-level
    /// `ramp_frames_left` stays: it only chooses where `run_block` splits a block, no output bit
    /// reads it, and its invariant (`>=` every lane's `remaining`) still holds once this lane's
    /// ramps rest. `collapsed` stays as the whole-bank reset leaves it. A lane past the width
    /// changes nothing. Bounded by one lane's words.
    fn reset_lane(&mut self, lane: usize, kind: ResetKind) {
        if lane >= L::WIDTH {
            return;
        }
        for channel in 0..2 {
            match kind {
                ResetKind::FullToDefaults => self.reset_lane_full(channel, lane),
                ResetKind::DiscontinuityKeepParameters => {
                    self.reset_lane_discontinuity(channel, lane);
                }
            }
        }
        if kind == ResetKind::FullToDefaults {
            self.rest_link(lane);
        }
    }
    // REALTIME_POLICY_END

    /// Validates one block's automation spans for one lane and points its ramps at the accepted
    /// values. The division that D11 permits happens exactly here, in
    /// `LinearRamp::set_target`. A time span (#1336) commits the time and retargets its
    /// coefficient ramp ([`retime_rate`](Self::retime_rate)) or applies the hold rule
    /// ([`retime_hold`](Self::retime_hold)).
    fn apply_automation(
        &mut self,
        spans: &[PreparedAutomationSpan],
        first_sample: u64,
        lane: usize,
        report: &mut ProcessReport,
    ) {
        let mut pending = [[None; PARAMETER_COUNT]; 2];
        let mut last_order = None;
        for (span_index, span) in spans.iter().enumerate() {
            let channel = match span.channel {
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
                .and_then(|value| value.checked_add(channel as u32))
            else {
                report.invalid_spans = report.invalid_spans.saturating_add(1);
                continue;
            };
            let valid = span_index < self.automation_capacity as usize
                && parameter_index < PARAMETER_COUNT
                && span.sample == first_sample
                && parameter_value_valid(&GATE_SPECS[parameter_index], span.value)
                && last_order.is_none_or(|previous| order > previous)
                && pending[channel][parameter_index].is_none();
            if !valid {
                report.invalid_spans = report.invalid_spans.saturating_add(1);
                continue;
            }
            last_order = Some(order);
            pending[channel][parameter_index] = Some(normalize_zero(span.value));
        }
        for (channel, targets) in pending.iter().enumerate() {
            for (index, target) in targets.iter().enumerate() {
                let Some(value) = *target else { continue };
                match index {
                    4 => self.retime_rate(channel, lane, 0, value),
                    5 => self.retime_hold(channel, lane, value),
                    6 => self.retime_rate(channel, lane, 1, value),
                    _ => {}
                }
                if index >= RAMP_COUNT {
                    continue;
                }
                let slot = &mut self.state[channel].ramps[index];
                let mut ramp = LinearRamp {
                    current: lane_get(slot.current, lane),
                    target: lane_get(slot.target, lane),
                    step: lane_get(slot.step, lane),
                    remaining: lane_get(slot.remaining, lane) as u32,
                };
                ramp.set_target(value, RAMP_SAMPLES);
                lane_set(&mut slot.current, lane, ramp.current);
                lane_set(&mut slot.target, lane, ramp.target);
                lane_set(&mut slot.step, lane, ramp.step);
                lane_set(&mut slot.remaining, lane, ramp.remaining as f32);
                self.ramp_frames_left = self.ramp_frames_left.max(RAMP_SAMPLES);
            }
        }
    }

    /// Commits an attack (`slot` 0) or release (`slot` 1) time for one lane and channel and
    /// retargets that lane's coefficient ramp (#1336 D2, D3).
    ///
    /// The time does not ramp: `timing` holds the committed time, and the coefficient moves from
    /// the value in effect, `c0` (mid-ramp included), to the design `c1 = 1 - exp(-1000 / (t fs))`
    /// in [`RAMP_SAMPLES`] samples, `c[k] = ramp_toward(c[k-1], (c1 - c0) / 64, c1)` and
    /// `c[64] = c1` exactly. Every `c[k]` lies in `[min(c0, c1), max(c0, c1)]` (the #1409 clamp),
    /// inside the launch interval `[RATE_MIN, RATE_MAX]`, so the one-pole stays stable and the
    /// envelope never overshoots. Two vendored `math` exps per record, on the block's control
    /// side, as the compressor pays (D9). Out of line so the scalar `exp` stays out of
    /// `process_bank`.
    #[inline(never)]
    fn retime_rate(&mut self, channel: usize, lane: usize, slot: usize, milliseconds: f32) {
        let timing = &mut self.timing[lane][channel];
        if slot == 0 {
            timing.attack_ms = milliseconds;
        } else {
            timing.release_ms = milliseconds;
        }
        let design = attack_release_coefficient(milliseconds, self.sample_rate);
        let rate = &mut self.state[channel].rates[slot];
        let mut ramp = LinearRamp {
            current: lane_get(rate.current, lane),
            target: lane_get(rate.target, lane),
            step: lane_get(rate.step, lane),
            remaining: lane_get(rate.remaining, lane) as u32,
        };
        ramp.set_target(design, RAMP_SAMPLES);
        lane_set(&mut rate.current, lane, ramp.current);
        lane_set(&mut rate.target, lane, ramp.target);
        lane_set(&mut rate.step, lane, ramp.step);
        lane_set(&mut rate.remaining, lane, ramp.remaining as f32);
        self.ramp_frames_left = self.ramp_frames_left.max(RAMP_SAMPLES);
    }

    /// Commits a hold time for one lane and channel (#1336 D4): the window keeps its start.
    ///
    /// With `H0` the hold count in effect, `H1 = rounded_samples(h, fs)` and `r` the countdown, an
    /// open lane takes `r' = max(0, r + H1 - H0)`, the countdown its last reload would have left
    /// had it used `H1` (so `r' <= H1`, as `r <= H0`), and a closed lane `r' = min(r, H1)`. The next
    /// reload uses `H1`. Every value is an integer below `2^24`, so the `f32` words are exact. Hold
    /// moves no sample value itself, so it takes effect at once, with no ramp: the release one-pole
    /// already smooths the close it times.
    fn retime_hold(&mut self, channel: usize, lane: usize, milliseconds: f32) {
        // In the validated domain at a launch rate the count always exists.
        let Some(next) = rounded_samples(milliseconds, self.sample_rate) else {
            return;
        };
        self.timing[lane][channel].hold_ms = milliseconds;
        let next = next as f32;
        let previous = lane_get(self.coef[channel].hold_samples, lane);
        let state = &mut self.state[channel];
        let countdown = lane_get(state.hysteresis.hold, lane);
        let countdown = if lane_get(state.hysteresis.open, lane) > 0.0 {
            (countdown + next - previous).max(0.0)
        } else {
            countdown.min(next)
        };
        lane_set(&mut state.hysteresis.hold, lane, countdown);
        lane_set(&mut self.coef[channel].hold_samples, lane, next);
    }

    /// Runs one block through the kernel and applies the D7 boundary policy.
    ///
    /// The block is split at the frame the last ramp finishes, so the smoothing prologue runs as a
    /// const-generic variant rather than as a per-sample branch. Splitting anywhere else, or not
    /// at all, would produce the same samples — the `RAMPING = false` variant omits updates that
    /// are exact no-ops once every `remaining` is zero — which is what makes the render
    /// partition-invariant.
    ///
    /// `MONO` (issue #973) renders the collapsed track: `left` holds both channels, `right` is the
    /// ungathered scratch and is never read or written, and only the left channel's state moves.
    /// The silent test, the kernel and the D7 check then read the left plane and the left state
    /// only, which is the dual predicate with the right conjunct restated as the left one.
    fn run_block<const MONO: bool>(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        sidechain: Option<(&[f32], &[f32])>,
        frames: usize,
        reports: &mut [ProcessReport; MAX_WIDTH],
    ) {
        let width = L::WIDTH;
        if MONO {
            // From here until the disengage copy the right channel is frozen and stale.
            self.collapsed = true;
        }
        let smoothed = (self.ramp_frames_left as usize).min(frames);
        if self.silent_block::<MONO>(left, right, sidechain) {
            // Every lane of both channels is at its exact rest and every input word is `+0.0`,
            // so the kernel would leave every state word as it is and write the `+0.0` the
            // buffers already hold, and `finish_block` would recover and report nothing. The
            // one word the rendered block moves regardless is the bank's `ramp_frames_left`
            // clock, which it advances here exactly as below.
            #[cfg(test)]
            admitted_silent();
            self.ramp_frames_left -= smoothed as u32;
            return;
        }
        if smoothed > 0 {
            let split = smoothed * width;
            let side = sidechain.map(|(l, r)| (&l[..split], &r[..split]));
            let right = if MONO {
                &mut [][..]
            } else {
                &mut right[..split]
            };
            if self.gliding::<MONO>() {
                self.run_segment::<true, MONO, true>(&mut left[..split], right, side, smoothed);
            } else {
                self.run_segment::<true, MONO, false>(&mut left[..split], right, side, smoothed);
            }
        }
        if smoothed < frames {
            let split = smoothed * width;
            let side = sidechain.map(|(l, r)| (&l[split..], &r[split..]));
            let right = if MONO {
                &mut [][..]
            } else {
                &mut right[split..]
            };
            let rest = frames - smoothed;
            self.run_segment::<false, MONO, false>(&mut left[split..], right, side, rest);
        }
        self.ramp_frames_left -= smoothed as u32;
        if MONO {
            self.finish_block_mono(left, frames, reports);
        } else {
            self.finish_block(left, right, frames, reports);
        }
    }

    /// Whether this block may skip the kernel (issue #1746): every input plane is exact `+0.0`
    /// (the connected sidechain's too, so the scalar instance with a sidechain takes the path
    /// only when its detector is silent as well) and every lane is at rest
    /// ([`rest_lanes`](Self::rest_lanes)), padded lanes included. Decided on demand from state
    /// after this block's automation was applied, never cached. The cheap input scan runs first,
    /// so a block with signal never evaluates the predicate.
    ///
    /// `#[inline(never)]`: it carries no vector arithmetic, and out of line it cannot change how
    /// the kernel inlines into `process_bank` and `process_bank_mono`, the `f32x4` functions the
    /// wasm kernel roster names for this effect.
    ///
    /// `MONO` reads the left plane and the left channel's rest only (issue #973): the right plane
    /// is not gathered, and a collapsed bank's counterfactual right channel is its left one.
    #[inline(never)]
    fn silent_block<const MONO: bool>(
        &self,
        left: &[f32],
        right: &[f32],
        sidechain: Option<(&[f32], &[f32])>,
    ) -> bool {
        #[cfg(test)]
        if SILENT_FORBIDDEN.with(core::cell::Cell::get) {
            return false;
        }
        let all = all_lanes(L::WIDTH);
        if MONO {
            return block_is_positive_zero(left) && self.channel_rest_lanes(0) == all;
        }
        block_is_positive_zero(left)
            && block_is_positive_zero(right)
            && sidechain.is_none_or(|(side_left, side_right)| {
                block_is_positive_zero(side_left) && block_is_positive_zero(side_right)
            })
            && self.rest_lanes() == all
    }

    /// The lanes at exact rest on both channels (`PreparedNativeEffectBank::lanes_at_rest`, issue
    /// #1746): bit `l` is set when [`kernel::channel_at_rest`] holds for lane `l` of both
    /// channels, so one block of exact `+0.0` on every input of the lane leaves every one of its
    /// state words bit-identical and writes `+0.0` on both output planes. See that function for
    /// the words pinned and why the list is complete. A padded lane is answered by the same rule.
    /// While [`collapsed`](Self::collapsed) the left channel answers alone (#1894).
    ///
    /// The gate has one clock, and it is bank-level: `ramp_frames_left`, the countdown that
    /// selects the ramping segment, advances by the block length alone once every ramp is
    /// settled. It has no per-lane clock. From state only, no allocation, bounded by the width.
    fn rest_lanes(&self) -> u32 {
        if self.collapsed {
            self.channel_rest_lanes(0)
        } else {
            self.channel_rest_lanes(0) & self.channel_rest_lanes(1)
        }
    }

    /// The lanes of one channel at exact rest: [`kernel::channel_at_rest`] per lane.
    fn channel_rest_lanes(&self, channel: usize) -> u32 {
        let width = L::WIDTH;
        let mut mask = all_lanes(width);
        let coef = self.coef[channel].lanes();
        let state = self.state[channel].lanes();
        for lane in 0..width {
            if !kernel::channel_at_rest(&coef[lane], &state[lane]) {
                mask &= !(1 << lane);
            }
        }
        mask
    }

    fn run_segment<const RAMPING: bool, const MONO: bool, const GLIDE: bool>(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        sidechain: Option<(&[f32], &[f32])>,
        frames: usize,
    ) {
        let (state_left, state_right) = self.state.split_at_mut(1);
        gate_frames::<L, CONNECTED, RAMPING, MONO, GLIDE>(GateArgs {
            left,
            right,
            sidechain,
            frames,
            coef: (&self.coef[0], &self.coef[1]),
            state: (&mut state_left[0], &mut state_right[0]),
        });
    }

    /// The D7 boundary check, once per block per channel, and the lane-local recovery.
    ///
    /// The scan and the lane attribution are `effect_runtime::bank`'s §4.4 forms. The
    /// gain words are scanned as a one-frame block of their own, because a NaN `G` produces a
    /// perfectly finite output: `fast_gain_from_db` clamps its argument with the D8 `max`/`min`, which
    /// swallow NaN, so the output block alone would never show it.
    ///
    /// Recovery is *lane-local and channel-local*, which is where this departs from
    /// `bank::finish_block`: that zeroes both channels and resets the whole bank, and issue #48
    /// froze the gate's recovery as one track's one channel, with the other lanes bit-unchanged.
    ///
    /// A padded lane (issue #1092) is recovered like any other, so it stays finite and at rest, but
    /// it is never reported: its entry in the bank report stays empty.
    fn finish_block(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: usize,
        reports: &mut [ProcessReport; MAX_WIDTH],
    ) {
        let width = L::WIDTH;
        let mut gains = [0.0_f32; MAX_WIDTH];
        for (channel, block) in [left, right].into_iter().enumerate() {
            self.state[channel].gain_db.store(&mut gains[..width]);
            if check_block::<L>(block) && check_block::<L>(&gains[..width]) {
                continue;
            }
            let failed =
                nonfinite_lane_mask::<L>(block) | nonfinite_lane_mask::<L>(&gains[..width]);
            for lane in 0..width {
                if failed & (1 << lane) == 0 {
                    continue;
                }
                for frame in 0..frames {
                    block[frame * width + lane] = 0.0;
                }
                self.reset_lane_full(channel, lane);
                if !self.is_active(lane) {
                    continue;
                }
                let report = &mut reports[lane];
                let counter = if channel == 0 {
                    &mut report.nonfinite_left_blocks
                } else {
                    &mut report.nonfinite_right_blocks
                };
                *counter = counter.saturating_add(1);
            }
        }
    }

    /// [`finish_block`](Self::finish_block) for a collapsed block (issue #973): the left plane and
    /// the left gain words only decide. A failed lane is zeroed and recovered on **both**
    /// channels, as the dual block on `L == R` fails and recovers both: the right channel's
    /// `timing` is its own committed values, which the disengage copy does not carry, so the dual
    /// run's re-seed from the right defaults must happen here (its right state words the copy then
    /// replaces). An active lane's failure is counted on both channels, because the right plane
    /// the seam writes is this left plane.
    fn finish_block_mono(
        &mut self,
        left: &mut [f32],
        frames: usize,
        reports: &mut [ProcessReport; MAX_WIDTH],
    ) {
        let width = L::WIDTH;
        let mut gains = [0.0_f32; MAX_WIDTH];
        self.state[0].gain_db.store(&mut gains[..width]);
        if check_block::<L>(left) && check_block::<L>(&gains[..width]) {
            return;
        }
        let failed = nonfinite_lane_mask::<L>(left) | nonfinite_lane_mask::<L>(&gains[..width]);
        for lane in 0..width {
            if failed & (1 << lane) == 0 {
                continue;
            }
            for frame in 0..frames {
                left[frame * width + lane] = 0.0;
            }
            self.reset_lane_full(0, lane);
            self.reset_lane_full(1, lane);
            if !self.is_active(lane) {
                continue;
            }
            let report = &mut reports[lane];
            report.nonfinite_left_blocks = report.nonfinite_left_blocks.saturating_add(1);
            report.nonfinite_right_blocks = report.nonfinite_right_blocks.saturating_add(1);
        }
    }

    /// The `DESIGNED` term of the channel-symmetry witness for `lane` (issue #973): whether every
    /// per-channel word the kernel reads that is not running state compares bit-equal between
    /// the two channels.
    ///
    /// # The word list, and why it is exactly this
    ///
    /// [`gate_frames`] reads per channel a [`GateCoef`] and a [`GateState`], and nothing else:
    ///
    /// * `GateCoef`: `hold_samples` (the reload value), `bypass`, `link_max` and `link_avg`. The
    ///   bypass word is prepared from a whole-instance value and the link words from the lane's
    ///   own mode (issue #1368), the same on both channels, so they cannot differ today; they
    ///   are compared because the kernel loads them per channel.
    /// * the four parameter ramps (`GateState::ramps`), the two rate ramps
    ///   (`GateState::rates`) and the two link weights (`GateState::links`, #1370), all four
    ///   fields each: `remaining` decides whether a ramp moves and
    ///   when it snaps, and `current`, `target` and `step` are what it moves.
    ///
    /// Deliberately excluded:
    ///
    /// * `gain_db` and `hysteresis` (`open`, `hold`): running state, which the induction the
    ///   witness states carries, not designed words.
    /// * `defaults` and `timing`: control-side values the kernel never loads. A reset or a hold
    ///   retime that made the channels disagree shows in `hold_samples` and the ramps.
    /// * `sample_rate`, `automation_capacity`, `ramp_frames_left`, `active`: whole-instance.
    ///
    /// Raw bits, so `+0.0` against `-0.0` declines. A lane past the width declines.
    fn designed_channel_symmetry(&self, lane: usize) -> bool {
        if lane >= L::WIDTH {
            return false;
        }
        let same = |a: L, b: L| lane_get(a, lane).to_bits() == lane_get(b, lane).to_bits();
        let (coef_left, coef_right) = (&self.coef[0], &self.coef[1]);
        let (left, right) = (&self.state[0], &self.state[1]);
        same(coef_left.hold_samples, coef_right.hold_samples)
            && same(coef_left.bypass, coef_right.bypass)
            && same(coef_left.link_max, coef_right.link_max)
            && same(coef_left.link_avg, coef_right.link_avg)
            && left
                .ramps
                .iter()
                .chain(&left.rates)
                .chain(&left.links)
                .zip(right.ramps.iter().chain(&right.rates).chain(&right.links))
                .all(|(l, r)| {
                    same(l.current, r.current)
                        && same(l.target, r.target)
                        && same(l.step, r.step)
                        && same(l.remaining, r.remaining)
                })
    }

    // REALTIME_POLICY_BEGIN: #1278 D3, the payload codec runs in the plan-swap block.
    /// Writes one lane's channel section of the state payload.
    fn write_lane(&self, channel: usize, lane: usize, bytes: &mut [u8]) {
        let state = &self.state[channel];
        payload::write_f32(bytes, 0, lane_get(state.gain_db, lane));
        payload::write_f32(bytes, 1, lane_get(state.hysteresis.open, lane));
        payload::write_f32(bytes, 2, lane_get(state.hysteresis.hold, lane));
        let timing = self.timing[lane][channel];
        payload::write_f32(bytes, 3, timing.attack_ms);
        payload::write_f32(bytes, 4, timing.hold_ms);
        payload::write_f32(bytes, 5, timing.release_ms);
        for (index, ramp) in state.ramps.iter().enumerate() {
            let word = STATE_RAMP_WORD + index * 4;
            payload::write_f32(bytes, word, lane_get(ramp.current, lane));
            payload::write_f32(bytes, word + 1, lane_get(ramp.target, lane));
            payload::write_f32(bytes, word + 2, lane_get(ramp.step, lane));
            payload::write_f32(bytes, word + 3, lane_get(ramp.remaining, lane));
        }
        for (index, rate) in state.rates.iter().chain(&state.links).enumerate() {
            let word = STATE_RATE_WORD + index * 4;
            payload::write_f32(bytes, word, lane_get(rate.current, lane));
            payload::write_f32(bytes, word + 1, lane_get(rate.target, lane));
            payload::write_f32(bytes, word + 2, lane_get(rate.step, lane));
            payload::write_f32(bytes, word + 3, lane_get(rate.remaining, lane));
        }
    }

    fn snapshot_lane(
        &self,
        lane: usize,
        output: &mut StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        let layout = self.layout();
        payload::validate_lengths(
            &layout,
            (output.common.len(), output.left.len(), output.right.len()),
        )
        .map_err(codec_error)?;
        payload::write_header(&layout, output.common);
        self.write_lane(0, lane, output.left);
        self.write_lane(1, lane, output.right);
        Ok(())
    }

    /// Parses and validates one channel section without touching any state.
    ///
    /// Every value is checked here so that a restore is all-or-none: two calls of this, both
    /// accepted, before the first word of either channel is written.
    fn parse_lane(&self, bytes: &[u8]) -> Result<LaneRestore, StatePayloadError> {
        let gain_db = payload::read_f32(bytes, 0);
        let open = payload::read_f32(bytes, 1);
        let hold = payload::read_f32(bytes, 2);
        let timing = LaneTiming {
            attack_ms: payload::read_f32(bytes, 3),
            hold_ms: payload::read_f32(bytes, 4),
            release_ms: payload::read_f32(bytes, 5),
        };
        for (index, value) in [
            (4_usize, timing.attack_ms),
            (5, timing.hold_ms),
            (6, timing.release_ms),
        ] {
            if is_negative_zero(value) || !parameter_value_valid(&GATE_SPECS[index], value) {
                return Err(state_error("effect.state.parameter"));
            }
        }
        if !normal_or_zero(gain_db) || !(-96.0..=0.0).contains(&gain_db) {
            return Err(state_error("effect.state.gain"));
        }
        if open.to_bits() != 0 && open.to_bits() != OPEN_WORD.to_bits() {
            return Err(state_error("effect.state.phase"));
        }
        let hold_samples = rounded_samples(timing.hold_ms, self.sample_rate)
            .ok_or(state_error("effect.state.parameter"))?;
        if !integral_within(hold, hold_samples as f32) {
            return Err(state_error("effect.state.hold"));
        }
        let mut ramps = [[0.0_f32; 4]; RAMP_COUNT];
        for (index, ramp) in ramps.iter_mut().enumerate() {
            let word = STATE_RAMP_WORD + index * 4;
            let current = payload::read_f32(bytes, word);
            let target = payload::read_f32(bytes, word + 1);
            let step = payload::read_f32(bytes, word + 2);
            let remaining = payload::read_f32(bytes, word + 3);
            // #1278: the effect's own mid-ramp snapshot must restore. Its step is
            // `(target - current) / 64`, which is subnormal for a ramp a few ulps long near zero,
            // so the step is held only to be finite. The target and the `current`, moving or
            // resting, are held to the strict domain: the clamped law keeps every word between the
            // ramp's start and its target (issue #1409 D2), so a word past the domain is one the
            // engine never holds (issue #1411 D1).
            let spec = &GATE_SPECS[index];
            let path = integral_within(remaining, RAMP_SAMPLES as f32)
                && payload::ramp_path_inside(
                    LinearRamp {
                        current,
                        target,
                        step,
                        remaining: remaining as u32,
                    },
                    (spec.minimum, spec.maximum),
                    RAMP_SAMPLES,
                );
            if is_negative_zero(current)
                || is_negative_zero(target)
                || !parameter_value_valid(spec, current)
                || !parameter_value_valid(spec, target)
                || !path
            {
                return Err(state_error("effect.state.parameter"));
            }
            *ramp = [current, target, step, remaining];
        }
        // #1336 D6: each coefficient ramp is aimed at the design of its committed time, bit for
        // bit, its path stays inside the launch interval, and a resting one is at its target.
        let mut rates = [[0.0_f32; 4]; RATE_COUNT];
        for (index, (rate, milliseconds)) in rates
            .iter_mut()
            .zip([timing.attack_ms, timing.release_ms])
            .enumerate()
        {
            let word = STATE_RATE_WORD + index * 4;
            let current = payload::read_f32(bytes, word);
            let target = payload::read_f32(bytes, word + 1);
            let step = payload::read_f32(bytes, word + 2);
            let remaining = payload::read_f32(bytes, word + 3);
            let design = attack_release_coefficient(milliseconds, self.sample_rate);
            let valid = target.to_bits() == design.to_bits()
                && integral_within(remaining, RAMP_SAMPLES as f32)
                && (remaining != 0.0 || current.to_bits() == target.to_bits())
                && payload::ramp_path_inside(
                    LinearRamp {
                        current,
                        target,
                        step,
                        remaining: remaining as u32,
                    },
                    (RATE_MIN, RATE_MAX),
                    RAMP_SAMPLES,
                );
            if !valid {
                return Err(state_error("effect.state.parameter"));
            }
            *rate = [current, target, step, remaining];
        }
        // Issue #1370 D6: each weight lies in `[0, 1]` on its whole path, the engine never holds
        // `-0.0` there, a glide in flight moves (its step is `(t - c) / n` with `t` in `{0, 1}`,
        // `c` a word of an earlier glide and `n <= 2^24`, never `0`), and the two targets are
        // exactly one supported mode's rest weights.
        let mut links = [[0.0_f32; 4]; LINK_COUNT];
        for (index, link) in links.iter_mut().enumerate() {
            let word = STATE_LINK_WORD + index * 4;
            let current = payload::read_f32(bytes, word);
            let target = payload::read_f32(bytes, word + 1);
            let step = payload::read_f32(bytes, word + 2);
            let remaining = payload::read_f32(bytes, word + 3);
            let maximum = effect_contract::LINK_GLIDE_MAX_SAMPLES;
            let valid = integral_within(remaining, maximum as f32)
                && !is_negative_zero(current)
                && (remaining == 0.0 || step != 0.0)
                && payload::ramp_path_inside(
                    LinearRamp {
                        current,
                        target,
                        step,
                        remaining: remaining as u32,
                    },
                    (0.0, 1.0),
                    maximum,
                );
            if !valid {
                return Err(state_error("effect.state.link"));
            }
            *link = [current, target, step, remaining];
        }
        let mode = effect_contract::link_glide_mode([links[0][1], links[1][1]])
            .filter(|mode| {
                GATE_EXPANDER_DESCRIPTOR
                    .supported_link_modes
                    .contains(*mode)
            })
            .ok_or(state_error("effect.state.link"))?;
        Ok(LaneRestore {
            gain_db: normalize_zero(gain_db),
            open,
            hold,
            timing,
            ramps,
            rates,
            links,
            mode,
        })
    }

    /// Commits one validated channel section into one lane.
    fn commit_lane(
        &mut self,
        channel: usize,
        lane: usize,
        parsed: &LaneRestore,
    ) -> Result<(), StatePayloadError> {
        self.timing[lane][channel] = parsed.timing;
        self.rederive_lane(channel, lane)
            .ok_or(state_error("effect.state.parameter"))?;
        lane_set(&mut self.state[channel].gain_db, lane, parsed.gain_db);
        lane_set(&mut self.state[channel].hysteresis.open, lane, parsed.open);
        lane_set(&mut self.state[channel].hysteresis.hold, lane, parsed.hold);
        for (index, ramp) in self.state[channel].ramps.iter_mut().enumerate() {
            lane_set(&mut ramp.current, lane, parsed.ramps[index][0]);
            lane_set(&mut ramp.target, lane, parsed.ramps[index][1]);
            lane_set(&mut ramp.step, lane, parsed.ramps[index][2]);
            lane_set(&mut ramp.remaining, lane, parsed.ramps[index][3]);
        }
        for (index, rate) in self.state[channel].rates.iter_mut().enumerate() {
            lane_set(&mut rate.current, lane, parsed.rates[index][0]);
            lane_set(&mut rate.target, lane, parsed.rates[index][1]);
            lane_set(&mut rate.step, lane, parsed.rates[index][2]);
            lane_set(&mut rate.remaining, lane, parsed.rates[index][3]);
        }
        if parsed
            .ramps
            .iter()
            .chain(&parsed.rates)
            .any(|ramp| ramp[3] != 0.0)
        {
            self.ramp_frames_left = self.ramp_frames_left.max(RAMP_SAMPLES);
        }
        // Issue #1370 D6: the weights land only under the mode they glide to. A payload taken
        // under another mode (the plan-swap carry into a successor prepared with a new mode)
        // leaves the lane's own mode at rest, as before the glide existed.
        if parsed.mode == self.lane_link(lane) {
            for (index, link) in self.state[channel].links.iter_mut().enumerate() {
                lane_set(&mut link.current, lane, parsed.links[index][0]);
                lane_set(&mut link.target, lane, parsed.links[index][1]);
                lane_set(&mut link.step, lane, parsed.links[index][2]);
                lane_set(&mut link.remaining, lane, parsed.links[index][3]);
                self.ramp_frames_left = self.ramp_frames_left.max(parsed.links[index][3] as u32);
            }
        }
        Ok(())
    }

    fn restore_lane(
        &mut self,
        lane: usize,
        state_layout_version: u32,
        input: &StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        if state_layout_version != STATE_LAYOUT_VERSION {
            return Err(state_error("effect.state.version"));
        }
        let layout = self.layout();
        payload::validate_lengths(
            &layout,
            (input.common.len(), input.left.len(), input.right.len()),
        )
        .map_err(codec_error)?;
        payload::read_header(&layout, input.common).map_err(codec_error)?;
        let left = self.parse_lane(input.left)?;
        let right = self.parse_lane(input.right)?;
        if left.mode != right.mode {
            return Err(state_error("effect.state.link"));
        }
        if left.mode != self.lane_link(lane) {
            self.rest_link(lane);
        }
        self.commit_lane(0, lane, &left)?;
        self.commit_lane(1, lane, &right)
    }
    // REALTIME_POLICY_END
}

/// Word index of the first ramp quadruple in a channel section.
const STATE_RAMP_WORD: usize = 6;

/// Word index of the first coefficient ramp quadruple (attack, then release) in a channel section
/// (#1336 D6).
const STATE_RATE_WORD: usize = STATE_RAMP_WORD + RAMP_COUNT * 4;

/// Word index of the first link weight quadruple (`w_link`, then `w_avg`) in a channel section
/// (#1370 D6). The rate quadruples run straight into it.
const STATE_LINK_WORD: usize = STATE_RATE_WORD + RATE_COUNT * 4;

/// The launch interval of a one-pole rate coefficient (#1336 D3): every design over the attack
/// and release domains at every launch rate lies inside it, and so does every word of a ramp
/// between two designs. The extremes, measured over the domains: the 2000 ms release at 96 kHz,
/// whose `f32` design `1 - expf(-1 / tau)` is `87 * 2^-24 = 5.1856e-6` (the exact value
/// `5.2083e-6` rounds down onto the `2^-24` grid of `expf` in `[1/2, 1)`), and the 0.1 ms attack
/// at 44.1 kHz, `0.2028858`.
const RATE_MIN: f32 = 5.0e-6;
const RATE_MAX: f32 = 0.203;

/// One validated channel section, held on the stack until both channels have been accepted.
struct LaneRestore {
    gain_db: f32,
    open: f32,
    hold: f32,
    timing: LaneTiming,
    ramps: [[f32; 4]; RAMP_COUNT],
    rates: [[f32; 4]; RATE_COUNT],
    links: [[f32; 4]; LINK_COUNT],
    /// The mode the weights glide to, decoded from their targets.
    mode: LinkMode,
}

impl<const CONNECTED: bool> PreparedNativeEffect for PreparedGate<f32, CONNECTED> {
    /// Every field is inline: this instance owns no heap (#1074).
    fn retained_bytes(&self) -> u64 {
        0
    }

    // REALTIME_POLICY_BEGIN
    fn reset(&mut self, kind: ResetKind) {
        PreparedGate::reset(self, kind);
    }
    // REALTIME_POLICY_END

    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        let mut reports = [ProcessReport::default(); MAX_WIDTH];
        self.apply_automation(block.automation, block.first_sample, 0, &mut reports[0]);
        let frames = block.frames();
        let sidechain = if CONNECTED { block.sidechain } else { None };
        self.run_block::<false>(block.left, block.right, sidechain, frames, &mut reports);
        reports[0]
    }

    /// Issue #973: [`PreparedGate::designed_channel_symmetry`] on lane 0. The term speaks for
    /// designed words only, connected or not: a sidechain's two channels are signal, which the
    /// witness's other terms and the compile's routing own, as for the compressor.
    fn channel_symmetry(&self) -> bool {
        self.designed_channel_symmetry(0)
    }

    fn retarget_link(&mut self, track: usize, mode: LinkMode, samples: u32) -> bool {
        track == 0 && PreparedGate::retarget_link(self, 0, mode, samples)
    }

    /// Issue #143 D2: the branching smoother's own gain word, read for lane 0.
    fn observe_resident(&self, tap_index: u32, out: &mut ObservationSample) -> bool {
        if tap_index != 0 {
            return false;
        }
        out.left = lane_get(self.state[0].gain_db, 0);
        out.right = lane_get(self.state[1].gain_db, 0);
        true
    }

    // REALTIME_POLICY_BEGIN: #1278 D3, the payload calls run in the plan-swap block.
    fn snapshot_state_payload(
        &self,
        mut output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.snapshot_lane(0, &mut output)
    }

    fn restore_state_payload(
        &mut self,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.restore_lane(0, state_layout_version, &input)
    }
    // REALTIME_POLICY_END
}

/// Whether this artifact executes banks of `lanes` lanes: a compile-time constant (D4).
const fn executes(lanes: usize) -> bool {
    lanes == Backend::current().width()
}

/// The lane bitmask with every lane of a `width`-lane shape set.
const fn all_lanes(width: usize) -> u32 {
    (1 << width) - 1
}

/// The lane bitmask of a validated active mask: bit `l` set where lane `l` carries a member.
fn active_bits(mask: &[bool]) -> u32 {
    mask.iter()
        .enumerate()
        .filter(|(_, active)| **active)
        .fold(0, |bits, (lane, _)| bits | (1 << lane))
}

/// Rejects a track index the bank does not have.
fn checked_track(track_index: u32, width: usize) -> Result<usize, StatePayloadError> {
    let track = usize::try_from(track_index).map_err(|_| state_error("effect.state.track"))?;
    if track >= width {
        return Err(state_error("effect.state.track"));
    }
    Ok(track)
}

macro_rules! bank_impl {
    ($lane:ty) => {
        impl PreparedNativeEffectBank for PreparedGate<$lane, false> {
            fn reset_lane(&mut self, lane: usize, kind: ResetKind) {
                PreparedGate::reset_lane(self, lane, kind);
            }
            /// Every field is inline: this instance owns no heap (#1074).
            fn retained_bytes(&self) -> u64 {
                0
            }

            fn observe_resident_bank(&self, tap_index: u32, out: &mut [ObservationSample]) -> bool {
                let lanes = <$lane as Lane>::WIDTH;
                if tap_index != 0 || out.len() != lanes {
                    return false;
                }
                let mut left = [0.0_f32; MAX_WIDTH];
                let mut right = [0.0_f32; MAX_WIDTH];
                self.state[0].gain_db.store(&mut left[..lanes]);
                self.state[1].gain_db.store(&mut right[..lanes]);
                for (lane, sample) in out.iter_mut().enumerate() {
                    sample.left = left[lane];
                    sample.right = right[lane];
                }
                true
            }

            fn reset(&mut self, kind: ResetKind) {
                PreparedGate::reset(self, kind);
            }

            /// Issue #1746: `PreparedGate::rest_lanes`, both channels, padded lanes included; the
            /// left channel alone on a collapsed bank (#1894).
            fn lanes_at_rest(&self) -> u32 {
                self.rest_lanes()
            }

            fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
                self.process_bank_inner::<false>(block)
            }

            /// Issue #973: [`PreparedGate::designed_channel_symmetry`].
            fn lane_channel_symmetry(&self, lane: usize) -> bool {
                self.designed_channel_symmetry(lane)
            }

            /// Issue #1370: `PreparedGate::retarget_link`; a padded lane takes no glide.
            fn retarget_link(&mut self, track: usize, mode: LinkMode, samples: u32) -> bool {
                PreparedGate::retarget_link(self, track, mode, samples)
            }

            fn supports_mono_collapse(&self) -> bool {
                true
            }

            fn process_bank_mono(
                &mut self,
                block: EffectBankProcessBlock<'_>,
            ) -> BankProcessReport {
                self.process_bank_inner::<true>(block)
            }

            fn desymmetrize_channels(&mut self) {
                self.desymmetrize();
            }

            fn channels_agree(&self) -> bool {
                self.channels_bit_equal()
            }

            // REALTIME_POLICY_BEGIN: #1278 D3, the payload calls run in the plan-swap block.
            fn snapshot_track_state_payload(
                &self,
                track_index: u32,
                mut output: StatePayloadOutput<'_>,
            ) -> Result<(), StatePayloadError> {
                let track = checked_track(track_index, <$lane>::WIDTH)?;
                self.snapshot_lane(track, &mut output)
            }

            fn restore_track_state_payload(
                &mut self,
                track_index: u32,
                state_layout_version: u32,
                input: StatePayloadInput<'_>,
            ) -> Result<(), StatePayloadError> {
                let track = checked_track(track_index, <$lane>::WIDTH)?;
                self.restore_lane(track, state_layout_version, &input)
            }
            // REALTIME_POLICY_END

            fn advance_clocks(&mut self, lanes_mask: u32, frames: u32) {
                let lanes = u32::MAX >> (32 - <$lane>::WIDTH);
                debug_assert_eq!(
                    lanes_mask & lanes & !self.lanes_at_rest(),
                    0,
                    "a lane not at rest"
                );
                self.advance_skipped_clocks(lanes_mask, frames);
            }

            fn lane_default_bytes(&self) -> Option<usize> {
                Some(LANE_DEFAULT_BYTES)
            }

            fn exchange_lane_defaults(&mut self, lane: usize, bytes: &mut [u8]) -> bool {
                self.exchange_defaults(lane, bytes)
            }
        }
    };
}

impl<L: Lane> PreparedGate<L, false> {
    /// The one bank body, dual or collapsed (issue #973).
    ///
    /// `MONO` chooses the render body and nothing else: the guard, the automation drain (both
    /// channels' spans, as a dual block takes them) and the report are the same statements in the
    /// same order. Inlined into each caller so the dual and the collapsed body stay two symbols,
    /// `process_bank` and `process_bank_mono`, each with the kernel inlined.
    #[inline(always)]
    fn process_bank_inner<const MONO: bool>(
        &mut self,
        block: EffectBankProcessBlock<'_>,
    ) -> BankProcessReport {
        let width = self.bank_width.expect("a bank is prepared with a width");
        let mut report = BankProcessReport::empty(width);
        // The block constructor validated every shape (finding F11); what it cannot know is which
        // bank the block is handed to, so the width and the absent sidechain -- a bank is only
        // bound for an unconnected sidechain port -- stay real guards.
        if block.width != width || block.sidechain.is_some() {
            return report;
        }
        debug_assert_eq!(block.left.len(), block.frames as usize * L::WIDTH);
        let mut reports = [ProcessReport::default(); MAX_WIDTH];
        for (track, lane_report) in reports.iter_mut().enumerate().take(L::WIDTH) {
            // A padded lane carries no track, so no span is its to apply (issue #1092).
            if !self.is_active(track) {
                continue;
            }
            let start = block.automation_offsets[track] as usize;
            let end = block.automation_offsets[track + 1] as usize;
            self.apply_automation(
                &block.automation[start..end],
                block.first_sample,
                track,
                lane_report,
            );
        }
        let frames = block.frames as usize;
        self.run_block::<MONO>(block.left, block.right, None, frames, &mut reports);
        report.reports[..L::WIDTH].copy_from_slice(&reports[..L::WIDTH]);
        report
    }

    /// The collapse's disengage copy (issue #973): the left channel's whole per-channel render
    /// state onto the right, every lane. That is every word [`gate_frames`] reads per channel:
    /// the [`GateState`] (`gain_db`, `open`, `hold`, the parameter and rate ramps) and the
    /// [`GateCoef`]. A hold retime taken on the right during the collapse read a frozen
    /// countdown; this copy replaces it with the left one, which is the dual run's.
    ///
    /// `defaults` and `timing` are not copied: they are each channel's own committed values,
    /// which a collapsed block's automation updates on both channels as a dual block would.
    fn desymmetrize(&mut self) {
        self.state[1] = self.state[0];
        self.coef[1] = self.coef[0];
        self.collapsed = false;
    }

    /// [`PreparedNativeEffectBank::channels_agree`] (issue #973): every word
    /// [`desymmetrize`](Self::desymmetrize) copies compares bit-equal between the channels, on
    /// every lane. The whole per-channel state is 31 lane words, so it is compared outright; no
    /// fixed point is needed.
    fn channels_bit_equal(&self) -> bool {
        let words = |coef: &GateCoef<L>, state: &GateState<L>| {
            let ramp_words =
                |ramp: &kernel::GateRamp<L>| [ramp.current, ramp.target, ramp.step, ramp.remaining];
            let mut all = [L::zero(); 4 + 3 + 4 * (RAMP_COUNT + RATE_COUNT + LINK_COUNT)];
            all[..7].copy_from_slice(&[
                coef.hold_samples,
                coef.bypass,
                coef.link_max,
                coef.link_avg,
                state.gain_db,
                state.hysteresis.open,
                state.hysteresis.hold,
            ]);
            for (chunk, ramp) in all[7..]
                .as_chunks_mut::<4>()
                .0
                .iter_mut()
                .zip(state.ramps.iter().chain(&state.rates).chain(&state.links))
            {
                chunk.copy_from_slice(&ramp_words(ramp));
            }
            all
        };
        let left = words(&self.coef[0], &self.state[0]);
        let right = words(&self.coef[1], &self.state[1]);
        left.iter().zip(&right).all(|(l, r)| {
            (0..L::WIDTH).all(|lane| lane_get(*l, lane).to_bits() == lane_get(*r, lane).to_bits())
        })
    }
}

// REALTIME_POLICY_BEGIN: #1747, the clock advance of a skipped block.
impl<L: Lane> PreparedGate<L, false> {
    /// [`PreparedNativeEffectBank::advance_clocks`]: the gate's one clock is the bank-wide
    /// `ramp_frames_left`, a schedule word. It only chooses where `run_block` splits a block, and
    /// no output bit reads it: the `RAMPING = true` body run over settled ramps selects their
    /// words back unchanged, so a split anywhere renders the same samples. It keeps the invariant
    /// `ramp_frames_left >= every lane's remaining`, and a block of any input lowers it by its
    /// length, down to zero; the skip lowers it as a rendered block would, which keeps that
    /// invariant tight. The gate has no per-lane clock: the `hold` countdown is detector state
    /// carried in the payload, not a clock.
    fn advance_skipped_clocks(&mut self, lanes_mask: u32, frames: u32) {
        let _ = lanes_mask;
        self.ramp_frames_left = self.ramp_frames_left.saturating_sub(frames);
    }
}
// REALTIME_POLICY_END

/// One lane's prepared defaults in bytes: every parameter of both channels, one `f32` each, then
/// the lane's link mode (issue #1368).
const LANE_DEFAULT_BYTES: usize = 2 * PARAMETER_COUNT * 4 + 4;

// REALTIME_POLICY_BEGIN: #1747, the prepared defaults move with a render-time lane swap.
impl<L: Lane> PreparedGate<L, false> {
    /// [`PreparedNativeEffectBank::exchange_lane_defaults`]: lane `lane`'s `defaults` row, left
    /// then right, each parameter in descriptor order as little-endian `f32` bits, then the lane's
    /// link mode as one word (issue #1368, `effect_contract::exchange_lane_link_word`). The row
    /// and the mode are the gate's prepared per-lane values: `reset_lane_full` (a full reset and
    /// the D7 recovery) re-seeds the lane from the row, the link words are set from the mode, and
    /// every other per-lane word is payload or derived from the row.
    fn exchange_defaults(&mut self, lane: usize, bytes: &mut [u8]) -> bool {
        if lane >= L::WIDTH || bytes.len() != LANE_DEFAULT_BYTES {
            return false;
        }
        let (values_bytes, link_bytes) = bytes.split_at_mut(2 * PARAMETER_COUNT * 4);
        let link = effect_contract::exchange_lane_link_word(self.lane_link(lane), link_bytes);
        self.set_lane_link(lane, link);
        // The weights rest at the incoming mode; the swap's payload restore then brings a glide
        // in flight with it (issue #1370 D6).
        self.rest_link(lane);
        let words = self.defaults[lane].iter_mut().flatten();
        for (value, chunk) in words.zip(values_bytes.as_chunks_mut::<4>().0.iter_mut()) {
            let mut incoming = [0_u8; 4];
            incoming.copy_from_slice(chunk);
            chunk.copy_from_slice(&value.to_bits().to_le_bytes());
            *value = f32::from_bits(u32::from_le_bytes(incoming));
        }
        true
    }
}
// REALTIME_POLICY_END

bank_impl!(Simd4);
// Issues #1110 and #1112: an eight-lane type only where `avx2` is enabled (`BankWidth`).
#[cfg(target_feature = "avx2")]
bank_impl!(lane::Simd8);

/// Validates one prepare request's initial values against the frozen domains.
fn initial_defaults(
    values: &[InitialParameterValue],
) -> Result<[[f32; PARAMETER_COUNT]; 2], EffectPrepareError> {
    if values.len() != PARAMETER_COUNT * 2 {
        return Err(EffectPrepareError {
            code: "effect.parameter.initial",
        });
    }
    let mut defaults = [[0.0; PARAMETER_COUNT]; 2];
    for (index, spec) in GATE_SPECS.iter().enumerate() {
        let left = values[index * 2];
        let right = values[index * 2 + 1];
        if left.parameter_index != index as u32
            || right.parameter_index != index as u32
            || left.channel != ParameterChannel::Left
            || right.channel != ParameterChannel::Right
            || !parameter_value_valid(spec, left.value)
            || !parameter_value_valid(spec, right.value)
            || is_negative_zero(left.value)
            || is_negative_zero(right.value)
        {
            return Err(EffectPrepareError {
                code: "effect.parameter.initial",
            });
        }
        defaults[0][index] = normalize_zero(left.value);
        defaults[1][index] = normalize_zero(right.value);
    }
    Ok(defaults)
}

impl NativeEffectFactory for GateExpanderFactory {
    fn descriptor(&self) -> &'static EffectDescriptor {
        &GATE_EXPANDER_DESCRIPTOR
    }

    fn prepared_inline_bytes(&self) -> u64 {
        (size_of::<PreparedGate<f32, true>>()).max(size_of::<PreparedGate<f32, false>>()) as u64
    }

    fn prepared_undeclared_heap_bytes(&self, _: &effect_contract::QualityDescriptor) -> u64 {
        0
    }

    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<PreparedEffect, EffectPrepareError> {
        let metadata = expected_prepared_metadata(self.descriptor(), request)?;
        let values = initial_defaults(request.initial_values)?;
        // Every row of the lane table holds validated values, never zeros: only row 0 is read at
        // `WIDTH = 1`, and a zero row would lie outside the ratio, attack and release domains.
        let defaults = [values; MAX_WIDTH];
        let connected = matches!(
            metadata.ports.sidechain,
            PreparedSidechainPort::Connected { .. }
        );
        let invalid = EffectPrepareError {
            code: "effect.parameter.initial",
        };
        let processor: Box<dyn PreparedNativeEffect> = if connected {
            Box::new(PreparedGate::<f32, true>::new(&metadata, None, defaults).ok_or(invalid)?)
        } else {
            Box::new(PreparedGate::<f32, false>::new(&metadata, None, defaults).ok_or(invalid)?)
        };
        Ok(PreparedEffect {
            processor,
            metadata,
        })
    }

    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<PreparedEffectBank>, EffectPrepareError> {
        bind_bank::<true>(request)
    }
}

/// `bind_homogeneous_bank`, with the D4 width check as a switch: `NATIVE_ONLY` declines a width
/// this artifact does not execute, which is what production asks for. The unit tests pass `false`
/// to bind the other width's bank on this host through the same code.
///
/// # Padding (issue #1092; decision 12)
///
/// The gate accepts a padded request. Each lane, active or padded, is prepared from its own
/// request, which for a padded lane is the caller's clone of an active member's, so every lane is
/// validated and inside its declared domains; no lane is ever filled with zeros. The rows of the
/// lane table past this width start as a clone of the first member's (lane 0's) values for the
/// same reason.
/// A padded lane takes no automation and is recovered by D7 without being reported
/// ([`PreparedGate::with_active_lanes`]). Fed `+0.0` it writes `+0.0` (`dry * gain`, or the dry
/// word), and the gate never reads across lanes, so an active lane's bits depend neither on the
/// padded lanes nor on which member they clone.
fn bind_bank<const NATIVE_ONLY: bool>(
    request: PrepareEffectBankRequest<'_>,
) -> Result<Option<PreparedEffectBank>, EffectPrepareError> {
    request.validate_shape()?;
    // Members come first (`validate_shape`), so lane 0 carries one.
    let first = request.requests[0];
    let metadata = expected_prepared_metadata(&GATE_EXPANDER_DESCRIPTOR, first)?;
    let mut defaults = [initial_defaults(first.initial_values)?; MAX_WIDTH];
    // Issue #1368: the key carries no link mode (`lane_link`), so each lane takes its own
    // member's mode, and a padded lane takes the first member's (D5).
    let mut links = [metadata.link_mode; MAX_WIDTH];
    let mut same_program = true;
    for (track, item) in request.requests.iter().copied().enumerate().skip(1) {
        let candidate = expected_prepared_metadata(&GATE_EXPANDER_DESCRIPTOR, item)?;
        if candidate.program_key() != metadata.program_key() {
            same_program = false;
        }
        defaults[track] = initial_defaults(item.initial_values)?;
        if request.active_mask[track] {
            links[track] = candidate.link_mode;
        }
    }
    // Every request is validated before the shape is decided, so a malformed member is an
    // error and an unbankable-but-valid cohort is a legal `Ok(None)` fallback.
    if !same_program
        || !matches!(
            metadata.ports.sidechain,
            PreparedSidechainPort::Unconnected {
                id,
                required: false,
            } if id == port_id("sidechain-in")
        )
    {
        return Ok(None);
    }
    let invalid = EffectPrepareError {
        code: "effect.parameter.initial",
    };
    let width = Some(request.width);
    let active = active_bits(request.active_mask);
    // There is no runtime SIMD dispatch (master plan #83 D4, revision 4): `wide` picks its
    // instruction set at compile time and the workspace pins `x86-64-v3`, so an unavailable width
    // is a scalar fallback. Each arm's check is a compile-time constant, so a production build
    // never links the bank of a width it does not execute.
    let processor = effect_contract::match_bank_width!(request.width, |L| {
        if NATIVE_ONLY && !executes(L::WIDTH) {
            return Ok(None);
        }
        Box::new(
            PreparedGate::<L, false>::new(&metadata, width, defaults)
                .ok_or(invalid)?
                .with_lane_links(&links[..request.requests.len()])
                .with_active_lanes(active),
        ) as Box<dyn PreparedNativeEffectBank>
    });
    Ok(Some(PreparedEffectBank {
        processor,
        metadata: PreparedBankMetadata {
            width: request.width,
            program_key: metadata.program_key(),
        },
    }))
}

#[cfg(test)]
mod glide_tests;
#[cfg(test)]
mod padding_tests;
#[cfg(test)]
mod rest_tests;
#[cfg(test)]
mod swap_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use effect_contract::{PrepareEffectLimits, PreparedPorts, validate_descriptor};
    use effect_runtime::params::ParameterKind;

    impl<L: Lane, const CONNECTED: bool> PreparedGate<L, CONNECTED> {
        /// Test-only fault injection: makes one lane's smoothed gain a NaN.
        ///
        /// Issue #48 forbids a public injection API, so this exists only under `cfg(test)` and is
        /// the only way to reach the case where the *state* is non-finite while the output block
        /// is not — which is exactly the case `fast_gain_from_db`'s NaN-swallowing clamp creates.
        pub(crate) fn inject_nonfinite_gain(&mut self, lane: usize, channel: usize) {
            lane_set(&mut self.state[channel].gain_db, lane, f32::NAN);
        }
    }

    fn values(threshold: f32) -> [InitialParameterValue; PARAMETER_COUNT * 2] {
        let defaults = [threshold, 20.0, 48.0, 6.0, 1.0, 0.0, 5.0];
        core::array::from_fn(|index| InitialParameterValue {
            parameter_index: (index / 2) as u32,
            channel: if index % 2 == 0 {
                ParameterChannel::Left
            } else {
                ParameterChannel::Right
            },
            value: defaults[index / 2],
        })
    }

    fn metadata(values: &[InitialParameterValue]) -> PreparedEffectMetadata {
        let quality = GATE_EXPANDER_DESCRIPTOR
            .qualities
            .iter()
            .find(|quality| quality.sample_rate == 48_000)
            .expect("launch rate");
        expected_prepared_metadata(
            &GATE_EXPANDER_DESCRIPTOR,
            PrepareEffectRequest {
                sample_rate: 48_000,
                quantum: 128,
                quality: EffectQuality::Normal,
                bypass: false,
                link_mode: LinkMode::DualMono,
                ports: PreparedPorts {
                    sidechain: PreparedSidechainPort::Unconnected {
                        id: port_id("sidechain-in"),
                        required: false,
                    },
                },
                initial_values: values,
                limits: PrepareEffectLimits {
                    maximum_total_state_bytes: quality.maximum_state.total().expect("total"),
                    maximum_scratch_bytes: 64,
                    maximum_automation_spans_per_block: 16,
                },
                tail_bound: conformance::tail_bound_of(
                    Box::new(crate::GateExpanderFactory),
                    48_000,
                    EffectQuality::Normal,
                ),
            },
        )
        .expect("prepared metadata")
    }

    fn noise(seed: u64, frames: usize) -> Vec<f32> {
        let mut state = seed | 1;
        (0..frames)
            .map(|_| {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                let word = (state >> 32) as u32;
                (f32::from((word >> 16) as u16) * (2.0 / 65_536.0) - 1.0) * 0.3
            })
            .collect()
    }

    #[test]
    fn the_runtime_parameter_specs_agree_with_the_frozen_descriptor() {
        validate_descriptor(&GATE_EXPANDER_DESCRIPTOR).expect("descriptor");
        for (index, descriptor) in GATE_EXPANDER_PARAMETERS.iter().enumerate() {
            let spec = GATE_SPECS[index];
            assert_eq!(spec.kind, ParameterKind::Continuous, "parameter {index}");
            assert_eq!(Some(spec.minimum), descriptor.minimum, "min {index}");
            assert_eq!(Some(spec.maximum), descriptor.maximum, "max {index}");
            assert_eq!(spec.default, descriptor.default_value, "default {index}");
            let expected = match descriptor.mapping {
                ParameterMapping::Linear => effect_runtime::params::ParameterMapping::Linear,
                ParameterMapping::Logarithmic => {
                    effect_runtime::params::ParameterMapping::Logarithmic
                }
                other => panic!("parameter {index} has an unexpected mapping {other:?}"),
            };
            assert_eq!(spec.mapping, expected, "mapping {index}");
        }
        for (index, descriptor) in GATE_EXPANDER_PARAMETERS.iter().enumerate() {
            assert_eq!(
                descriptor.smoothing_samples,
                if index == 5 { 0 } else { RAMP_SAMPLES },
                "smoothing {index}"
            );
        }
    }

    /// A NaN injected into one lane's gain recovers that lane alone, at this build's native bank
    /// width, `lane::Native`: eight lanes in the 8-lane (AVX2) build, four in a 4-lane
    /// (NEON/simd128) build (#1017, #1112). The bank is compared with a per-lane scalar peer and
    /// with an uninjected control bank of the same width.
    #[test]
    fn injected_nonfinite_gain_has_scalar_parity_at_the_native_width() {
        let width = BankWidth::for_backend(Backend::current())
            .expect("no product target renders scalar banks");
        injected_nonfinite_gain_parity::<lane::Native>(width);
    }

    fn injected_nonfinite_gain_parity<L: Lane>(width: BankWidth) {
        const FRAMES: usize = 128;
        const WARM: usize = 5;
        let lanes = L::WIDTH;
        assert_eq!(
            lanes,
            width.lanes() as usize,
            "{width:?} is not {lanes} lanes"
        );
        let track_values: [_; MAX_WIDTH] =
            core::array::from_fn(|track| values(-20.0 - track as f32));
        let mut defaults = [[[0.0; PARAMETER_COUNT]; 2]; MAX_WIDTH];
        for (track, set) in track_values.iter().enumerate() {
            defaults[track] = initial_defaults(set).expect("initial values");
        }
        let shared = metadata(&track_values[0]);
        let mut bank = PreparedGate::<L, false>::new(&shared, Some(width), defaults).expect("bank");
        let mut control =
            PreparedGate::<L, false>::new(&shared, Some(width), defaults).expect("control bank");
        let mut peers: Vec<PreparedGate<f32, false>> = (0..lanes)
            .map(|track| {
                let mut lane_defaults = [[[0.0; PARAMETER_COUNT]; 2]; MAX_WIDTH];
                lane_defaults[0] = defaults[track];
                PreparedGate::<f32, false>::new(&shared, None, lane_defaults).expect("peer")
            })
            .collect();

        let sources: Vec<Vec<f32>> = (0..lanes)
            .map(|track| noise(17 + track as u64, FRAMES * (WARM + 1)))
            .collect();
        let mut bank_reports = [ProcessReport::default(); MAX_WIDTH];
        let mut peer_reports = [ProcessReport::default(); MAX_WIDTH];
        let mut bank_left = vec![0.0_f32; FRAMES * lanes];
        let mut bank_right = vec![0.0_f32; FRAMES * lanes];
        let mut control_left = bank_left.clone();
        let mut control_right = bank_right.clone();
        let mut peer_left = vec![vec![0.0_f32; FRAMES]; lanes];
        let mut peer_right = vec![vec![0.0_f32; FRAMES]; lanes];

        for block in 0..=WARM {
            if block == WARM {
                bank.inject_nonfinite_gain(3, 0);
                peers[3].inject_nonfinite_gain(0, 0);
            }
            for track in 0..lanes {
                for frame in 0..FRAMES {
                    let sample = sources[track][block * FRAMES + frame];
                    bank_left[frame * lanes + track] = sample;
                    bank_right[frame * lanes + track] = sample;
                    control_left[frame * lanes + track] = sample;
                    control_right[frame * lanes + track] = sample;
                    peer_left[track][frame] = sample;
                    peer_right[track][frame] = sample;
                }
            }
            bank_reports = [ProcessReport::default(); MAX_WIDTH];
            peer_reports = [ProcessReport::default(); MAX_WIDTH];
            bank.run_block::<false>(
                &mut bank_left,
                &mut bank_right,
                None,
                FRAMES,
                &mut bank_reports,
            );
            control.run_block::<false>(
                &mut control_left,
                &mut control_right,
                None,
                FRAMES,
                &mut [ProcessReport::default(); MAX_WIDTH],
            );
            for track in 0..lanes {
                let mut single = [ProcessReport::default(); MAX_WIDTH];
                peers[track].run_block::<false>(
                    &mut peer_left[track],
                    &mut peer_right[track],
                    None,
                    FRAMES,
                    &mut single,
                );
                peer_reports[track] = single[0];
            }
        }

        // The NaN never reaches the output: `fast_gain_from_db`'s D8 clamp swallows it, so `A` is finite
        // and `z * A` is finite. Only the block-end scan of the gain words sees it.
        for track in 0..lanes {
            for frame in 0..FRAMES {
                assert_eq!(
                    bank_left[frame * lanes + track].to_bits(),
                    peer_left[track][frame].to_bits(),
                    "W{lanes} left track {track} frame {frame}"
                );
                assert_eq!(
                    bank_right[frame * lanes + track].to_bits(),
                    peer_right[track][frame].to_bits(),
                    "W{lanes} right track {track} frame {frame}"
                );
            }
            assert_eq!(
                bank_reports[track], peer_reports[track],
                "W{lanes} report {track}"
            );
            if track == 3 {
                assert_eq!(bank_reports[track].nonfinite_left_blocks, 1);
                assert_eq!(bank_reports[track].nonfinite_right_blocks, 0);
                for frame in 0..FRAMES {
                    assert_eq!(bank_left[frame * lanes + track].to_bits(), 0);
                }
                assert_eq!(lane_get(bank.state[0].gain_db, 3).to_bits(), 0, "G is +0");
                assert_eq!(lane_get(bank.state[0].hysteresis.open, 3), OPEN_WORD);
                assert_eq!(
                    lane_get(bank.state[0].hysteresis.hold, 3),
                    lane_get(bank.coef[0].hold_samples, 3)
                );
            } else {
                assert_eq!(bank_reports[track], ProcessReport::default());
                for frame in 0..FRAMES {
                    assert_eq!(
                        bank_left[frame * lanes + track].to_bits(),
                        control_left[frame * lanes + track].to_bits(),
                        "W{lanes} track {track} is untouched by track 3's recovery"
                    );
                }
            }
            assert_eq!(
                bank_right[3].to_bits(),
                control_right[3].to_bits(),
                "the right channel of track 3 is untouched"
            );
        }
    }

    #[test]
    fn internal_w4_pcm_and_serialized_continuation_are_bit_exact() {
        const WIDTH: usize = 4;
        const PREFIX: usize = 17;
        const FRAMES: usize = 129;
        let track_sets: [_; MAX_WIDTH] = core::array::from_fn(|track| values(-20.0 - track as f32));
        let mut defaults = [[[0.0; PARAMETER_COUNT]; 2]; MAX_WIDTH];
        for track in 0..WIDTH {
            defaults[track] = initial_defaults(&track_sets[track]).expect("defaults");
        }
        let shared = metadata(&track_sets[0]);
        let mut donor = PreparedGate::<Simd4, false>::new(&shared, Some(BankWidth::Four), defaults)
            .expect("internal W4 donor");
        let mut restored =
            PreparedGate::<Simd4, false>::new(&shared, Some(BankWidth::Four), defaults)
                .expect("internal W4 restore target");

        let mut prefix_left = vec![0.01_f32; PREFIX * WIDTH];
        let mut prefix_right = prefix_left.clone();
        let mut reports = [ProcessReport::default(); MAX_WIDTH];
        donor.run_block::<false>(
            &mut prefix_left,
            &mut prefix_right,
            None,
            PREFIX,
            &mut reports,
        );
        assert!(lane_get(donor.state[0].gain_db, 2) < 0.0);

        let sizes = shared.state_sizes;
        let mut payloads = Vec::new();
        for lane in 0..WIDTH {
            let mut common = vec![0; sizes.common_bytes as usize];
            let mut left_payload = vec![0; sizes.left_bytes as usize];
            let mut right_payload = vec![0; sizes.right_bytes as usize];
            donor
                .snapshot_track_state_payload(
                    lane as u32,
                    StatePayloadOutput::new(
                        &mut common,
                        &mut left_payload,
                        &mut right_payload,
                        sizes,
                    )
                    .expect("W4 payload sizes"),
                )
                .expect("W4 snapshot");
            payloads.push((common, left_payload, right_payload));
        }
        for (lane, (common, left_payload, right_payload)) in payloads.iter().enumerate() {
            restored
                .restore_track_state_payload(
                    lane as u32,
                    STATE_LAYOUT_VERSION,
                    StatePayloadInput::new(common, left_payload, right_payload, sizes)
                        .expect("W4 payload"),
                )
                .expect("W4 restore");
        }

        let continuation_frames = FRAMES - PREFIX;
        let mut donor_left = vec![0.0_f32; continuation_frames * WIDTH];
        let mut donor_right = vec![0.0_f32; continuation_frames * WIDTH];
        let mut restored_left = donor_left.clone();
        let mut restored_right = donor_right.clone();
        for lane in 0..WIDTH {
            for (frame, sample) in noise(101 + lane as u64, continuation_frames)
                .into_iter()
                .enumerate()
            {
                donor_left[frame * WIDTH + lane] = sample;
                donor_right[frame * WIDTH + lane] = -sample;
                restored_left[frame * WIDTH + lane] = sample;
                restored_right[frame * WIDTH + lane] = -sample;
            }
        }
        donor.run_block::<false>(
            &mut donor_left,
            &mut donor_right,
            None,
            continuation_frames,
            &mut reports,
        );
        restored.run_block::<false>(
            &mut restored_left,
            &mut restored_right,
            None,
            continuation_frames,
            &mut reports,
        );
        assert_eq!(donor_left, restored_left, "W4 continuation left PCM");
        assert_eq!(donor_right, restored_right, "W4 continuation right PCM");
        let mut donor_common = vec![0; sizes.common_bytes as usize];
        let mut donor_left_payload = vec![0; sizes.left_bytes as usize];
        let mut donor_right_payload = vec![0; sizes.right_bytes as usize];
        donor
            .snapshot_track_state_payload(
                2,
                StatePayloadOutput::new(
                    &mut donor_common,
                    &mut donor_left_payload,
                    &mut donor_right_payload,
                    sizes,
                )
                .expect("W4 output sizes"),
            )
            .expect("W4 output snapshot");
        let mut restored_common = vec![0; sizes.common_bytes as usize];
        let mut restored_left_payload = vec![0; sizes.left_bytes as usize];
        let mut restored_right_payload = vec![0; sizes.right_bytes as usize];
        restored
            .snapshot_track_state_payload(
                2,
                StatePayloadOutput::new(
                    &mut restored_common,
                    &mut restored_left_payload,
                    &mut restored_right_payload,
                    sizes,
                )
                .expect("W4 restored output sizes"),
            )
            .expect("W4 restored output snapshot");
        assert_eq!(donor_common, restored_common);
        assert_eq!(donor_left_payload, restored_left_payload);
        assert_eq!(donor_right_payload, restored_right_payload);
    }
}
