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
//! Both bands go through a `Fs/50` ring, which is the declared latency; the detector tap of each
//! band is read `lookahead` samples earlier in the same ring. Each band then rides a
//! Giannoulis-Massberg-Reiss static curve with a fixed 6 dB knee and a branching smoother, and the
//! two gained bands are summed.
//!
//! # What this crate does *not* contain
//!
//! Parameter ramps, the state-payload codec, the gain computer, the dB conversions, the envelope
//! smoother, the detector link and the block boundary check all live in `miso-engine-effect-runtime`
//! and `miso-engine-lane`. The audit found every one of them copied here, and the copies had
//! already diverged (#94 F6). What is left is this effect's equations, its parameter table and its
//! state layout.
//!
//! # Realtime rules
//!
//! No allocation, no locks, no platform transcendentals and no per-value `is_finite` on any render
//! path. Denormals are handled by `flush` on the six recursive state words, and non-finite output
//! is caught once per block by `miso_engine_effect_runtime::bank` (D7).

use miso_engine_effect_contract::{
    AutomationRate, AutomationSpanKind, BankProcessReport, BankWidth, EffectBankProcessBlock,
    EffectDescriptor, EffectPrepareError, EffectProcessBlock, EffectQuality,
    InitialParameterValue, LatencySamples, LinkMode, LinkModeSet, NativeEffectFactory,
    ObservationCadence, ObservationChannels, ObservationCost, ObservationDescriptor,
    ObservationFold, ObservationKind, ObservationSample, ObservationTapId, ParameterChannel,
    ParameterChannelPolicy, ParameterDescriptor, ParameterDomain, ParameterId, ParameterMapping,
    ParameterUnit, PortDescriptor, PortId, PortLayout, PortRole, PrepareEffectBankRequest,
    PrepareEffectRequest, PreparedAutomationSpan, PreparedBankMetadata, PreparedEffectMetadata,
    PreparedNativeEffect, PreparedNativeEffectBank, ProcessReport, ResetKind, SmoothingRule,
    StatePayloadError, StatePayloadInput, StatePayloadOutput, StatePayloadSizes, TailSamples,
    expected_prepared_metadata,
};
use miso_engine_effect_runtime::bank::{self, NonFiniteReport};
use miso_engine_effect_runtime::dynamics::{GainComputerCoef, gain_delta_db};
use miso_engine_effect_runtime::envelope::retention_coefficient;
use miso_engine_effect_runtime::params::{ParameterSpec, normalize_zero, parameter_value_valid};
use miso_engine_effect_runtime::ramp::LinearRamp;
use miso_engine_effect_runtime::state_payload::{
    STATE_LENGTH_CODE, STATE_VERSION_CODE, read_f32, read_u32, write_f32, write_u32,
};
use miso_engine_lane::kernels::{SvfState, svf_step};
use miso_engine_lane::{Lane, Simd4, Simd8, flush};
use miso_engine_math::fast_db::{fast_gain_from_db, fast_level_db};

pub mod corpus;
mod shim;
#[cfg(test)]
mod split;

use shim::{LINK_AVERAGE, LINK_DUAL_MONO, LINK_MAXIMUM, branching_smooth, link_levels};

/// Parameters in the frozen V1 order.
const PARAMETER_COUNT: usize = 12;

/// Ramped parameters: everything but the two preparation-time ones.
const RAMP_COUNT: usize = 10;

/// State-payload words each ramp occupies: current, target, step, remaining.
const RAMP_WORDS: usize = 4;

/// Fixed scalar words of one channel's state payload, before the two rings.
const LANE_HEADER_WORDS: usize = 48;

/// State layout version. Version 1 was the four-section, three-ring, three-word-ramp layout; the
/// audit's F1, F4 and D11 all change it, and pre-launch there is no persisted version-1 state.
const STATE_LAYOUT_VERSION: u32 = 2;

/// Index of the low band.
const LOW_BAND: usize = 0;

/// Index of the high band.
const HIGH_BAND: usize = 1;

/// Knee width of both bands, in dB. Fixed by the product (spec 018).
const KNEE_DB: f32 = 6.0;

/// Samples a ramped parameter takes to reach its target (`SmoothingRule::Linear`, 64).
const SMOOTHING_SAMPLES: u32 = 64;

/// Detector floor: the smallest amplitude a band level is measured at, `-160 dB`.
const DETECTOR_FLOOR: f32 = 1.0e-8;

/// Butterworth damping `k = 1 / Q = sqrt(2)`.
const BUTTERWORTH_K: f64 = core::f64::consts::SQRT_2;

/// `-2k`, the all-pass tap of the first stage. Rounded once; doubling and negating are exact.
const NEGATIVE_TWO_K: f32 = -2.0 * (core::f64::consts::SQRT_2 as f32);

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

/// Frozen parameter order and stable numeric IDs for the V1 two-band product.
pub const MULTIBAND_COMPRESSOR_PARAMETERS_V1: [ParameterDescriptor; PARAMETER_COUNT] = [
    parameter(
        1,
        "crossover",
        "Hz",
        ParameterUnit::Hz,
        80.0,
        8_000.0,
        1_000.0,
        ParameterMapping::Logarithmic,
        AutomationRate::None,
        SmoothingRule::None,
        0,
    ),
    parameter(
        2,
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
    parameter(
        3,
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

/// Bytes of one channel's state payload at `sample_rate`.
///
/// `48` fixed words — crossover, lookahead, two smoother words, ten four-word ramps (D11 adds the
/// precomputed step) and four filter words — followed by the low and high rings of `Fs/50 + 1`
/// samples each. Version 1 carried three three-word ramps' worth less, eight filter words and a
/// third ring for the dry signal, which #94 F1 and F4 removed.
const fn lane_bytes(sample_rate: u32) -> u32 {
    let ring = sample_rate / 50 + 1;
    (LANE_HEADER_WORDS as u32 + 2 * ring) * 4
}

const fn quality(sample_rate: u32) -> miso_engine_effect_contract::QualityDescriptor {
    let bytes = lane_bytes(sample_rate);
    miso_engine_effect_contract::QualityDescriptor {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples((sample_rate / 50) as u64),
        tail: TailSamples::Infinite,
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

const QUALITIES: [miso_engine_effect_contract::QualityDescriptor; 4] = [
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
pub const MULTIBAND_COMPRESSOR_OBSERVATIONS_V1: [ObservationDescriptor; 1] =
    [ObservationDescriptor {
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
    }];

/// Immutable descriptor for the launch two-band multiband compressor.
pub const MULTIBAND_COMPRESSOR_DESCRIPTOR_V1: EffectDescriptor = EffectDescriptor {
    id: effect_id("miso.multiband-compressor"),
    display_name: "Multiband Compressor",
    contract_major: 1,
    // Issue #143 P1: declaring the first tap is a `contract_minor` bump and a derived identity
    // re-pin of exactly `32 + len("Gain Reduction") + len("dB")` = 48 bytes.
    // `state_layout_version` does not move: the tap reads state that was already there.
    contract_minor: 1,
    state_layout_version: STATE_LAYOUT_VERSION,
    supported_link_modes: LinkModeSet::ALL,
    parameters: &MULTIBAND_COMPRESSOR_PARAMETERS_V1,
    ports: &PORTS,
    qualities: &QUALITIES,
    observations: &MULTIBAND_COMPRESSOR_OBSERVATIONS_V1,
};

/// The domain of one parameter, in the shared runtime's vocabulary.
///
/// Derived from the frozen descriptor rather than written a second time: the mapping is a
/// control-surface concern and is not used here, so every spec is built as continuous over the
/// descriptor's own bounds.
const fn spec(index: usize) -> ParameterSpec {
    let descriptor = &MULTIBAND_COMPRESSOR_PARAMETERS_V1[index];
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

/// Domains of the twelve parameters, in descriptor order.
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
    spec(11),
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
/// 1. `(v1, lp1) = svf_step(x)` — the first stage, both taps of one state
/// 2. `ap = fma(-2k, v1, x)` — unfused (#163 phase 2); this is `svf_block`'s all-pass mix
///    `(1, -2k, 0)`
/// 3. `(_, low) = svf_step(lp1)` — the second stage
/// 4. `high = ap - low`
///
/// Step 4 is what makes the band sum the all-pass by construction rather than by accident: the
/// only rounding between `ap` and `low + high` is the one subtraction.
#[inline(always)]
pub fn lr4_step<L: Lane>(x: L, c: &Lr4Coef<L>, s: &mut Lr4State<L>) -> (L, L) {
    let (v1, lp1) = svf_step(x, c.nc1, c.a2, c.a3, &mut s.a);
    let ap = c.nk2.fma(v1, x);
    let (_, low) = svf_step(lp1, c.nc1, c.a2, c.a3, &mut s.b);
    (low, ap.sub(low))
}

/// Designs `(c1, a2, a3)` in `f64` for one crossover frequency, rounded once to `f32`.
///
/// `g = tan(pi fc / fs)`, `k = sqrt(2)`, `t = g (g + k)`, `c1 = t / (1 + t)`, `a2 = g (1 - c1)`,
/// `a3 = g a2`. The tangent comes from `miso-engine-math`, never the platform libm (D6).
///
/// The rounded triple is then checked *back*: `g` is recovered from `c1` alone, the analytic
/// second-order low-pass magnitude at the requested crossover is evaluated from it, and it must be
/// the half-power point to 0.005 dB. That is a test of the `f32` rounding of the damping
/// coefficient — the thing amendment A1 exists for — and not a restatement of the design.
fn design_lr4(sample_rate: u32, crossover_hz: f32) -> Option<[f32; 3]> {
    if sample_rate == 0 || !parameter_value_valid(&SPECS[0], crossover_hz) {
        return None;
    }
    let g = miso_engine_math::tan(
        core::f64::consts::PI * f64::from(crossover_hz) / f64::from(sample_rate),
    );
    let t = g * (g + BUTTERWORTH_K);
    let c1 = t / (1.0 + t);
    let a1 = 1.0 - c1;
    let designed = [c1 as f32, (g * a1) as f32, (g * (g * a1)) as f32];
    if !designed.into_iter().all(normal_or_zero)
        || !(0.0..1.0).contains(&designed[0])
        || designed[1] <= 0.0
        || designed[2] <= 0.0
    {
        return None;
    }
    // `t = c1 / (1 - c1)` and `t = g (g + k)`, so `g` is the positive root of `g^2 + kg - t`.
    let realized_t = f64::from(designed[0]) / (1.0 - f64::from(designed[0]));
    let realized_g = 0.5
        * (miso_engine_math::sqrt(BUTTERWORTH_K * BUTTERWORTH_K + 4.0 * realized_t)
            - BUTTERWORTH_K);
    if realized_g <= 0.0 {
        return None;
    }
    // `a3 / a2` is `g` too; a triple whose two routes to `g` disagree is not this filter.
    let ratio_g = f64::from(designed[2]) / f64::from(designed[1]);
    if (ratio_g - realized_g).abs() > 1.0e-4 * realized_g {
        return None;
    }
    // Analytic |LP2| at the crossover: `1 / |D(j*t_probe)|` with `D(s) = s^2 + k s + 1`.
    let probe = miso_engine_math::tan(
        core::f64::consts::PI * f64::from(crossover_hz) / f64::from(sample_rate),
    ) / realized_g;
    let real = 1.0 - probe * probe;
    let imaginary = BUTTERWORTH_K * probe;
    let magnitude = miso_engine_math::sqrt(real * real + imaginary * imaginary);
    if magnitude <= 0.0 || !magnitude.is_finite() {
        return None;
    }
    let magnitude_db = -20.0 * miso_engine_math::log10(magnitude);
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

/// The detector tap's offset from the write cursor, in ring slots.
///
/// The output tap is one slot ahead of the cursor, which is `ring_len - 1 = Fs/50` samples of
/// delay: the declared latency. The detector tap is `lookahead` samples earlier still, so its
/// offset is `1 + lookahead` and lies in `[1, ring_len]` — which is what makes the single
/// compare-and-subtract wrap in [`Instance::detector`] correct.
fn detector_offset(lookahead_ms: f32, sample_rate: u32, ring_len: usize) -> Option<usize> {
    if !parameter_value_valid(&SPECS[1], lookahead_ms) || sample_rate == 0 || ring_len < 2 {
        return None;
    }
    let samples =
        miso_engine_math::floor(f64::from(lookahead_ms) * f64::from(sample_rate) / 1_000.0 + 0.5);
    if !samples.is_finite() || samples < 0.0 {
        return None;
    }
    let latency = ring_len - 1;
    Some(1 + (samples as usize).min(latency))
}

// ---------------------------------------------------------------------------------------------
// Instance
// ---------------------------------------------------------------------------------------------

/// Control-rate values derived from one band's ratio, attack and release.
///
/// Keyed by the raw bits of the three ramp values they came from, so a segment recomputes them
/// only when one of the three has actually moved. `-0.0` never reaches a ramp, so the key is a
/// faithful identity.
#[derive(Clone, Copy)]
struct BandCache {
    key: [u32; 3],
    inv_ratio_minus_one: f32,
    attack: f32,
    release: f32,
}

impl BandCache {
    /// A cache that will always miss on its first use.
    const fn empty() -> Self {
        Self {
            key: [0; 3],
            inv_ratio_minus_one: 0.0,
            attack: 0.0,
            release: 0.0,
        }
    }

    /// Refreshes the cache from the three ramps' current values if any of them has changed.
    fn refresh(&mut self, ratio: f32, attack_ms: f32, release_ms: f32, sample_rate: u32) {
        let key = [ratio.to_bits(), attack_ms.to_bits(), release_ms.to_bits()];
        if key == self.key {
            return;
        }
        self.key = key;
        self.inv_ratio_minus_one = 1.0 / ratio - 1.0;
        self.attack = retention_coefficient(attack_ms, sample_rate);
        self.release = retention_coefficient(release_ms, sample_rate);
    }
}

/// One band's control-rate lane coefficients for one segment.
#[derive(Clone, Copy)]
struct BandCoef<L: Lane> {
    inv_ratio_minus_one: L,
    attack: L,
    release: L,
}

/// The ten ramps of one channel, as lanes, for the duration of one segment.
#[derive(Clone, Copy)]
struct Segment<L: Lane> {
    current: [L; RAMP_COUNT],
    step: [L; RAMP_COUNT],
}

/// How far the next segment reaches, and whether anything is ramping over it.
///
/// The two answers are one decision because they come from the same scan of the same counters, so
/// they are produced together rather than scanned for twice ([`Instance::plan_segment`]).
///
/// `ramping` is a **whole-bank** predicate: every track of both channels, all ten parameters. The
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
    /// `true` while any ramp in the bank still has samples to produce.
    ramping: bool,
}

/// Everything one channel of one bank owns.
struct Side<L: Lane, const W: usize> {
    coefficients: Lr4Coef<L>,
    filter: Lr4State<L>,
    /// The branching smoother's state, in dB, per band.
    gain_db: [L; 2],
    ramps: [[LinearRamp; RAMP_COUNT]; W],
    cache: [[BandCache; 2]; W],
    /// `(c1, a2, a3)` per track, kept so a reset does not have to redesign (#94 F9).
    designed: [[f32; 3]; W],
    crossover_hz: [f32; W],
    lookahead_ms: [f32; W],
    detector_offset: [usize; W],
    /// `ring_len * W` samples, slot-major: slot `s` of lane `l` is at `s * W + l`.
    low_ring: Box<[f32]>,
    high_ring: Box<[f32]>,
    defaults: [[f32; PARAMETER_COUNT]; W],
}

impl<L: Lane, const W: usize> Side<L, W> {
    fn new(
        defaults: [[f32; PARAMETER_COUNT]; W],
        sample_rate: u32,
        ring_len: usize,
    ) -> Option<Self> {
        let mut designed = [[0.0; 3]; W];
        let mut crossover_hz = [0.0; W];
        let mut lookahead_ms = [0.0; W];
        let mut offsets = [0usize; W];
        let mut ramps = [[LinearRamp::fixed(0.0); RAMP_COUNT]; W];
        for track in 0..W {
            designed[track] = design_lr4(sample_rate, defaults[track][0])?;
            crossover_hz[track] = defaults[track][0];
            lookahead_ms[track] = defaults[track][1];
            offsets[track] = detector_offset(defaults[track][1], sample_rate, ring_len)?;
            for index in 0..RAMP_COUNT {
                ramps[track][index] = LinearRamp::fixed(defaults[track][index + 2]);
            }
        }
        Some(Self {
            coefficients: lane_coefficients::<L, W>(&designed),
            filter: Lr4State::default(),
            gain_db: [L::zero(); 2],
            ramps,
            cache: [[BandCache::empty(); 2]; W],
            designed,
            crossover_hz,
            lookahead_ms,
            detector_offset: offsets,
            low_ring: alloc_ring(ring_len * W),
            high_ring: alloc_ring(ring_len * W),
            defaults,
        })
    }

    /// Clears history. Coefficients and parameters are deliberately untouched.
    fn discontinuity_reset(&mut self) {
        self.filter = Lr4State::default();
        self.gain_db = [L::zero(); 2];
        self.low_ring.fill(0.0);
        self.high_ring.fill(0.0);
        for track in 0..W {
            for ramp in &mut self.ramps[track] {
                ramp.snap();
            }
        }
    }

    /// Returns to the prepared defaults without allocating or redesigning (#94 F9).
    fn full_reset(&mut self, sample_rate: u32, ring_len: usize) {
        self.discontinuity_reset();
        self.coefficients = lane_coefficients::<L, W>(&self.designed);
        for track in 0..W {
            self.crossover_hz[track] = self.defaults[track][0];
            self.lookahead_ms[track] = self.defaults[track][1];
            self.detector_offset[track] =
                detector_offset(self.defaults[track][1], sample_rate, ring_len)
                    .unwrap_or(self.detector_offset[track]);
            for index in 0..RAMP_COUNT {
                self.ramps[track][index] = LinearRamp::fixed(self.defaults[track][index + 2]);
            }
            self.cache[track] = [BandCache::empty(); 2];
        }
    }
}

/// Allocates one zeroed ring. The only allocation in the crate, and it happens at prepare.
fn alloc_ring(samples: usize) -> Box<[f32]> {
    vec![0.0; samples].into_boxed_slice()
}

/// Splats the per-track designs into one lane coefficient set.
fn lane_coefficients<L: Lane, const W: usize>(designed: &[[f32; 3]; W]) -> Lr4Coef<L> {
    let mut c1 = [0.0f32; 8];
    let mut a2 = [0.0f32; 8];
    let mut a3 = [0.0f32; 8];
    for track in 0..W {
        c1[track] = designed[track][0];
        a2[track] = designed[track][1];
        a3[track] = designed[track][2];
    }
    Lr4Coef {
        nc1: L::load(&c1[..W]).neg(),
        a2: L::load(&a2[..W]),
        a3: L::load(&a3[..W]),
        nk2: L::splat(NEGATIVE_TWO_K),
    }
}

/// One prepared bank of `W` tracks: the whole effect, at one width.
struct Instance<L: Lane, const W: usize> {
    sample_rate: u32,
    bypass: bool,
    link: LinkMode,
    ring_len: usize,
    cursor: usize,
    nonfinite: NonFiniteReport,
    /// Index 0 is the left channel, index 1 the right.
    sides: [Side<L, W>; 2],
}

impl<L: Lane, const W: usize> Instance<L, W> {
    fn new(
        left: [[f32; PARAMETER_COUNT]; W],
        right: [[f32; PARAMETER_COUNT]; W],
        metadata: PreparedEffectMetadata,
    ) -> Option<Self> {
        debug_assert_eq!(L::WIDTH, W);
        let ring_len = usize::try_from(metadata.sample_rate / 50)
            .ok()?
            .checked_add(1)?;
        if ring_len < 2 {
            return None;
        }
        Some(Self {
            sample_rate: metadata.sample_rate,
            bypass: metadata.bypass,
            link: metadata.link_mode,
            ring_len,
            cursor: 0,
            nonfinite: NonFiniteReport::new(),
            sides: [
                Side::new(left, metadata.sample_rate, ring_len)?,
                Side::new(right, metadata.sample_rate, ring_len)?,
            ],
        })
    }

    fn reset(&mut self, kind: ResetKind) {
        self.cursor = 0;
        for side in &mut self.sides {
            match kind {
                ResetKind::FullToDefaults => side.full_reset(self.sample_rate, self.ring_len),
                ResetKind::DiscontinuityKeepParameters => side.discontinuity_reset(),
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Render path
// ---------------------------------------------------------------------------------------------

/// `index` reduced into `[0, modulus)`, given `index < 2 * modulus`.
///
/// One compare and one subtract. The version-1 code used three integer modulos per lane per sample
/// on a ring length that is never a power of two, each of them a hardware divide (#94 F8).
#[inline(always)]
fn wrap(index: usize, modulus: usize) -> usize {
    debug_assert!(index < 2 * modulus);
    if index >= modulus {
        index - modulus
    } else {
        index
    }
}

/// The per-track detector tap of one ring, gathered into one lane.
///
/// The only non-contiguous access on the render path, and it is loads only: lookahead is a
/// per-track parameter, so each track reads a different slot of the shared ring.
#[inline(always)]
fn detector_tap<L: Lane, const W: usize>(
    ring: &[f32],
    cursor: usize,
    offsets: &[usize; W],
    ring_len: usize,
) -> L {
    let mut values = [0.0f32; 8];
    for track in 0..W {
        values[track] = ring[wrap(cursor + offsets[track], ring_len) * W + track];
    }
    L::load(&values[..W])
}

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
    let curve = GainComputerCoef {
        threshold_db: threshold,
        inv_ratio_minus_one: coefficients.inv_ratio_minus_one,
        half_knee_db: L::splat(0.5 * KNEE_DB),
        inv_two_knee: L::splat(1.0 / (2.0 * KNEE_DB)),
    };
    let target = gain_delta_db(level, &curve)
        .max(L::splat(-100.0))
        .min(L::zero());
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

/// One segment: `frames` frames over which no ramp arrives at its target.
///
/// The whole render path is here. `LINK` and `BYPASS` are compile-time, because both are fixed
/// when the effect is prepared; a bypassed instance is a pure `Fs/50` delay through the low ring
/// and runs neither the crossover nor the dynamics, and it still advances its ramps so that its
/// parameter state does not depend on whether it was bypassed.
///
/// `RAMPING` is the third compile-time switch, and it is the one that varies *within* a block.
/// It says whether any ramp of any track of either channel still has samples to produce over this
/// segment; [`Instance::plan_segment`] decides it from the ramp counters at the same moment it
/// decides the segment's length. At `RAMPING == false` the twenty per-frame lane additions below
/// are not merely predicted away, they are not emitted: a settled bank pays no smoothing
/// arithmetic at all.
///
/// **Why that is bit-identical, not merely close.** `LinearRamp` keeps `remaining == 0` implying
/// `step == +0.0`, and this crate admits parameter words through exactly four doors — prepared
/// defaults, `apply_automation`, `snap`, and a restored payload — every one of which runs
/// `normalize_zero` over a `parameter_value_valid` word. So on the skipped path every lane of
/// every `step` is `+0.0` and every lane of every `current` is finite and is not `-0.0`, which
/// makes `current.add(step)` the identity on all of them. The two exclusions matter and are the
/// same two `LinearRamp::stationary_at` names: `-0.0 + 0.0` is `+0.0`, and a NaN is quieted by an
/// addition. `Instance::flat_path_is_identity` asserts the precondition in debug builds rather
/// than leaving it as a comment.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn run_segment<L: Lane, const W: usize, const LINK: u8, const BYPASS: bool, const RAMPING: bool>(
    sides: &mut [Side<L, W>; 2],
    cursor: &mut usize,
    ring_len: usize,
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    segments: &mut [Segment<L>; 2],
    coefficients: &[[BandCoef<L>; 2]; 2],
) {
    let (head, tail) = sides.split_at_mut(1);
    let near = &mut head[0];
    let far = &mut tail[0];
    let mut filter_near = near.filter;
    let mut filter_far = far.filter;
    let mut gain_near = near.gain_db;
    let mut gain_far = far.gain_db;
    let mut position = *cursor;
    for frame in 0..frames {
        if RAMPING {
            for index in 0..RAMP_COUNT {
                segments[0].current[index] =
                    segments[0].current[index].add(segments[0].step[index]);
                segments[1].current[index] =
                    segments[1].current[index].add(segments[1].step[index]);
            }
        }
        let slot = position * W;
        let delayed = wrap(position + 1, ring_len) * W;
        let input_near = L::load(&left[frame * W..]);
        let input_far = L::load(&right[frame * W..]);
        if BYPASS {
            input_near.store(&mut near.low_ring[slot..]);
            input_far.store(&mut far.low_ring[slot..]);
            L::load(&near.low_ring[delayed..]).store(&mut left[frame * W..]);
            L::load(&far.low_ring[delayed..]).store(&mut right[frame * W..]);
            position = wrap(position + 1, ring_len);
            continue;
        }
        let (low_near, high_near) = lr4_step(input_near, &near.coefficients, &mut filter_near);
        let (low_far, high_far) = lr4_step(input_far, &far.coefficients, &mut filter_far);
        low_near.store(&mut near.low_ring[slot..]);
        high_near.store(&mut near.high_ring[slot..]);
        low_far.store(&mut far.low_ring[slot..]);
        high_far.store(&mut far.high_ring[slot..]);

        let detector_near_low =
            detector_tap::<L, W>(&near.low_ring, position, &near.detector_offset, ring_len);
        let detector_near_high =
            detector_tap::<L, W>(&near.high_ring, position, &near.detector_offset, ring_len);
        let detector_far_low =
            detector_tap::<L, W>(&far.low_ring, position, &far.detector_offset, ring_len);
        let detector_far_high =
            detector_tap::<L, W>(&far.high_ring, position, &far.detector_offset, ring_len);
        let (linked_near_low, linked_far_low) =
            link_levels::<L, LINK>(detector_near_low, detector_far_low);
        let (linked_near_high, linked_far_high) =
            link_levels::<L, LINK>(detector_near_high, detector_far_high);

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

        let output_near = L::load(&near.low_ring[delayed..])
            .mul(amplitude_near_low)
            .add(L::load(&near.high_ring[delayed..]).mul(amplitude_near_high));
        let output_far = L::load(&far.low_ring[delayed..])
            .mul(amplitude_far_low)
            .add(L::load(&far.high_ring[delayed..]).mul(amplitude_far_high));
        output_near.store(&mut left[frame * W..]);
        output_far.store(&mut right[frame * W..]);
        position = wrap(position + 1, ring_len);
    }
    near.filter = filter_near;
    far.filter = filter_far;
    near.gain_db = gain_near;
    far.gain_db = gain_far;
    *cursor = position;
}

/// Splits the block at ramp arrivals and runs each segment, ramped or flat.
///
/// `FORCE_RAMPING` is the split's A/B switch. It is `false` on every production path; the crate's
/// own bit-identity test sets it to reproduce exactly what this function did before the split
/// existed, so "split on versus split off" is a real comparison of two rendered buffers and two
/// state snapshots rather than an argument.
#[inline(always)]
fn process_block<
    L: Lane,
    const W: usize,
    const LINK: u8,
    const BYPASS: bool,
    const FORCE_RAMPING: bool,
>(
    instance: &mut Instance<L, W>,
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
) {
    let sample_rate = instance.sample_rate;
    let ring_len = instance.ring_len;
    let mut position = 0;
    while position < frames {
        let plan = instance.plan_segment(frames - position);
        let length = plan.frames;
        let mut segments = [instance.sides[0].segment(), instance.sides[1].segment()];
        let coefficients = [
            instance.sides[0].band_coefficients(sample_rate),
            instance.sides[1].band_coefficients(sample_rate),
        ];
        if plan.ramping || FORCE_RAMPING {
            run_segment::<L, W, LINK, BYPASS, true>(
                &mut instance.sides,
                &mut instance.cursor,
                ring_len,
                &mut left[position * W..(position + length) * W],
                &mut right[position * W..(position + length) * W],
                length,
                &mut segments,
                &coefficients,
            );
            let advanced = length as u32;
            instance.sides[0].store_segment(&segments[0], advanced);
            instance.sides[1].store_segment(&segments[1], advanced);
        } else {
            #[cfg(debug_assertions)]
            assert!(instance.flat_path_is_identity());
            run_segment::<L, W, LINK, BYPASS, false>(
                &mut instance.sides,
                &mut instance.cursor,
                ring_len,
                &mut left[position * W..(position + length) * W],
                &mut right[position * W..(position + length) * W],
                length,
                &mut segments,
                &coefficients,
            );
            // The write-back is skipped rather than performed and discarded, and that is sound
            // for the same reason the additions are: `store_segment` would write `current` back
            // unchanged, because no lane of it moved, and would apply `saturating_sub` to a
            // `remaining` that is already zero.
        }
        position += length;
    }
}

/// Dispatches on the two values that are fixed at preparation, once per block.
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
    match (instance.link, instance.bypass) {
        (LinkMode::DualMono, false) => {
            process_block::<L, W, LINK_DUAL_MONO, false, FORCE_RAMPING>(
                instance, left, right, frames,
            );
        }
        (LinkMode::Maximum, false) => {
            process_block::<L, W, LINK_MAXIMUM, false, FORCE_RAMPING>(
                instance, left, right, frames,
            );
        }
        (LinkMode::Average, false) => {
            process_block::<L, W, LINK_AVERAGE, false, FORCE_RAMPING>(
                instance, left, right, frames,
            );
        }
        (_, true) => {
            process_block::<L, W, LINK_DUAL_MONO, true, FORCE_RAMPING>(
                instance, left, right, frames,
            );
        }
    }
    instance.finish(left, right, frames, reports);
}

impl<L: Lane, const W: usize> Side<L, W> {
    /// The ten ramps of every track, as lanes, for one segment.
    fn segment(&self) -> Segment<L> {
        let mut current = [L::zero(); RAMP_COUNT];
        let mut step = [L::zero(); RAMP_COUNT];
        for index in 0..RAMP_COUNT {
            let mut values = [0.0f32; 8];
            let mut steps = [0.0f32; 8];
            for track in 0..W {
                values[track] = self.ramps[track][index].current;
                steps[track] = self.ramps[track][index].step;
            }
            current[index] = L::load(&values[..W]);
            step[index] = L::load(&steps[..W]);
        }
        Segment { current, step }
    }

    /// Writes a finished segment's lane values back into the scalar ramps.
    ///
    /// The lane accumulation is `current + step` iterated once per frame, lane by lane, which is
    /// exactly what `LinearRamp::next_value` does at `remaining >= 2`; the segment split
    /// guarantees no ramp reaches `remaining == 1` inside a segment, so no snap can be missed and
    /// the state can be written back instead of replayed.
    fn store_segment(&mut self, segment: &Segment<L>, advanced: u32) {
        for index in 0..RAMP_COUNT {
            let mut values = [0.0f32; 8];
            segment.current[index].store(&mut values[..W]);
            for (track, value) in values.iter().enumerate().take(W) {
                let ramp = &mut self.ramps[track][index];
                ramp.current = *value;
                ramp.remaining = ramp.remaining.saturating_sub(advanced);
            }
        }
    }

    /// Control-rate coefficients of both bands, refreshed only where a ramp value moved.
    fn band_coefficients(&mut self, sample_rate: u32) -> [BandCoef<L>; 2] {
        let mut bands = [BandCoef {
            inv_ratio_minus_one: L::zero(),
            attack: L::zero(),
            release: L::zero(),
        }; 2];
        for (band, coefficients) in bands.iter_mut().enumerate() {
            let base = band * 5;
            let mut ratios = [0.0f32; 8];
            let mut attacks = [0.0f32; 8];
            let mut releases = [0.0f32; 8];
            for track in 0..W {
                let ratio = self.ramps[track][base + 1].current;
                let attack_ms = self.ramps[track][base + 2].current;
                let release_ms = self.ramps[track][base + 3].current;
                let cache = &mut self.cache[track][band];
                cache.refresh(ratio, attack_ms, release_ms, sample_rate);
                ratios[track] = cache.inv_ratio_minus_one;
                attacks[track] = cache.attack;
                releases[track] = cache.release;
            }
            *coefficients = BandCoef {
                inv_ratio_minus_one: L::load(&ratios[..W]),
                attack: L::load(&attacks[..W]),
                release: L::load(&releases[..W]),
            };
        }
        bands
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
    /// the block is then one flat segment. Without the split that segment still ran the twenty
    /// additions per frame with every step at `+0.0`, which is the cost this change removes.
    ///
    /// The lengths this returns are exactly the lengths the pre-split form returned — the scan is
    /// the same scan, `ramping` is a second answer read out of it — so segment boundaries, and
    /// with them the sample at which each ramp arrives, do not move.
    fn plan_segment(&mut self, budget: usize) -> SegmentPlan {
        for side in &mut self.sides {
            for track in 0..W {
                for ramp in &mut side.ramps[track] {
                    if ramp.remaining == 1 {
                        ramp.snap();
                    }
                }
            }
        }
        let mut length = budget;
        let mut ramping = false;
        for side in &self.sides {
            for track in 0..W {
                for ramp in &side.ramps[track] {
                    if ramp.remaining > 0 {
                        ramping = true;
                        length = length.min(ramp.remaining as usize - 1);
                    }
                }
            }
        }
        SegmentPlan {
            frames: length.max(1),
            ramping,
        }
    }

    /// The precondition that makes the flat path bit-identical to the ramped one.
    ///
    /// Debug-only, and asserted at the point of use rather than argued in a comment: on a segment
    /// the split sends down the flat path, dropping `current.add(step)` must be the identity on
    /// every lane of every parameter of both channels. It is, when each ramp is at rest with a
    /// `+0.0` step and holds a value that `x + 0.0 == x` preserves bit for bit — which excludes
    /// `-0.0` (it would become `+0.0`) and the non-finite values (a NaN is quieted by the
    /// addition). Those are the same two exclusions `LinearRamp::stationary_at` carries, for the
    /// same reason.
    #[cfg(any(debug_assertions, test))]
    fn flat_path_is_identity(&self) -> bool {
        const NEGATIVE_ZERO: u32 = 0x8000_0000;
        self.sides.iter().all(|side| {
            side.ramps[..W].iter().flatten().all(|ramp| {
                ramp.remaining == 0
                    && ramp.step.to_bits() == 0
                    && ramp.current.is_finite()
                    && ramp.current.to_bits() != NEGATIVE_ZERO
            })
        })
    }

    /// The once-per-block output boundary check of master plan §4.4.
    ///
    /// The scan, the policy and the counters are `miso-engine-effect-runtime`'s; the only thing
    /// added here is attributing a rejected block to the contract's per-track reports. A bank's
    /// two channels share a reset, so the lane mask is the union over both and both counters move
    /// for a failing track.
    fn finish(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: usize,
        reports: &mut [ProcessReport],
    ) {
        let sides = &mut self.sides;
        let cursor = &mut self.cursor;
        let accepted = bank::finish_block::<L>(left, right, &mut self.nonfinite, || {
            *cursor = 0;
            for side in sides.iter_mut() {
                side.discontinuity_reset();
            }
        });
        if accepted {
            return;
        }
        let mask = self.nonfinite.nonfinite_lanes;
        let samples = frames as u64;
        for (track, report) in reports.iter_mut().enumerate().take(W) {
            if mask & (1 << track) != 0 {
                report.nonfinite_left_blocks = report.nonfinite_left_blocks.saturating_add(samples);
                report.nonfinite_right_blocks =
                    report.nonfinite_right_blocks.saturating_add(samples);
            }
        }
    }

    /// Points one track's ramps at the values a block's automation spans carry.
    ///
    /// The spans are `AutomationSpanKind::Point` events landing on the block's first sample, one
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
        let mut pending = [[None; RAMP_COUNT]; 2];
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
            let ramp = parameter
                .checked_sub(2)
                .filter(|value| *value < RAMP_COUNT && parameter < PARAMETER_COUNT);
            let valid = ramp.is_some()
                && index < capacity as usize
                && span.kind == AutomationSpanKind::Point
                && span.start_sample == first_sample
                && span.end_sample == first_sample
                && span.start_value.to_bits() == span.end_value.to_bits()
                && parameter_value_valid(&SPECS[parameter], span.start_value)
                && order.is_some_and(|value| previous.is_none_or(|earlier| value > earlier))
                && ramp.is_some_and(|value| pending[side][value].is_none());
            let Some(ramp) = ramp else {
                report.invalid_spans = report.invalid_spans.saturating_add(1);
                continue;
            };
            if !valid {
                report.invalid_spans = report.invalid_spans.saturating_add(1);
                continue;
            }
            previous = order;
            pending[side][ramp] = Some(normalize_zero(span.start_value));
        }
        for (side, targets) in pending.iter().enumerate() {
            for (index, target) in targets.iter().enumerate() {
                if let Some(value) = *target {
                    self.sides[side].ramps[track][index].set_target(value, SMOOTHING_SAMPLES);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------
// State payload, version 2
// ---------------------------------------------------------------------------------------------
//
// Per channel: crossover, lookahead, the two smoother words, ten four-word ramps, the four filter
// words, then the low and high rings written **oldest first**. Writing the rings in time order
// rather than in cursor order is what makes a scalar snapshot and a bank-track snapshot of the
// same history byte-identical, and it is why there is no cursor word: a track restored into a bank
// whose cursor is elsewhere is rotated into place by the restore itself.

/// Word offset of the first filter word.
const FILTER_WORD: usize = 4 + RAMP_COUNT * RAMP_WORDS;

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
    lookahead_ms: f32,
    designed: [f32; 3],
    detector_offset: usize,
    gains: [f32; 2],
    ramps: [LinearRamp; RAMP_COUNT],
    filter: [f32; 4],
}

fn write_side<L: Lane, const W: usize>(
    bytes: &mut [u8],
    side: &Side<L, W>,
    track: usize,
    cursor: usize,
    ring_len: usize,
) {
    write_f32(bytes, 0, side.crossover_hz[track]);
    write_f32(bytes, 1, side.lookahead_ms[track]);
    write_f32(bytes, 2, lane_value(side.gain_db[LOW_BAND], track));
    write_f32(bytes, 3, lane_value(side.gain_db[HIGH_BAND], track));
    for index in 0..RAMP_COUNT {
        let ramp = side.ramps[track][index];
        let word = 4 + index * RAMP_WORDS;
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
    for index in 0..ring_len {
        let slot = wrap(cursor + 1 + index, ring_len) * W + track;
        write_f32(bytes, LANE_HEADER_WORDS + index, side.low_ring[slot]);
        write_f32(
            bytes,
            LANE_HEADER_WORDS + ring_len + index,
            side.high_ring[slot],
        );
    }
}

/// Validates one channel's fixed words. Ring words are validated separately, in place.
fn stage_side(
    bytes: &[u8],
    sample_rate: u32,
    ring_len: usize,
) -> Result<StagedSide, StatePayloadError> {
    let crossover_hz = read_f32(bytes, 0);
    let lookahead_ms = read_f32(bytes, 1);
    if !parameter_state_valid(0, crossover_hz) || !parameter_state_valid(1, lookahead_ms) {
        return Err(state_error("effect.state.parameter"));
    }
    let designed =
        design_lr4(sample_rate, crossover_hz).ok_or(state_error("effect.state.coefficient"))?;
    let detector_offset = detector_offset(lookahead_ms, sample_rate, ring_len)
        .ok_or(state_error("effect.state.parameter"))?;
    let gains = [read_f32(bytes, 2), read_f32(bytes, 3)];
    if gains
        .into_iter()
        .any(|value| !normal_or_zero(value) || !(-100.0..=0.0).contains(&value))
    {
        return Err(state_error("effect.state.gain"));
    }
    let mut ramps = [LinearRamp::fixed(0.0); RAMP_COUNT];
    for (index, ramp) in ramps.iter_mut().enumerate() {
        let word = 4 + index * RAMP_WORDS;
        let current = read_f32(bytes, word);
        let target = read_f32(bytes, word + 1);
        let step = read_f32(bytes, word + 2);
        let remaining = read_u32(bytes, word + 3);
        // `LinearRamp`'s invariant, enforced rather than assumed: a ramp at rest is at its target
        // and has no increment. A payload that says otherwise would have the segment driver add a
        // stale step to a resting parameter for ever.
        if !parameter_state_valid(index + 2, current)
            || !parameter_state_valid(index + 2, target)
            || !normal_or_zero(step)
            || remaining > SMOOTHING_SAMPLES
            || (remaining == 0 && (step != 0.0 || current.to_bits() != target.to_bits()))
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
    for index in 0..2 * ring_len {
        if !normal_or_zero(read_f32(bytes, LANE_HEADER_WORDS + index)) {
            return Err(state_error("effect.state.ring"));
        }
    }
    Ok(StagedSide {
        crossover_hz,
        lookahead_ms,
        designed,
        detector_offset,
        gains: [normalize_zero(gains[0]), normalize_zero(gains[1])],
        ramps,
        filter,
    })
}

/// Applies a staged channel. Never allocates: the rings are written in place (#94 F9).
fn commit_side<L: Lane, const W: usize>(
    side: &mut Side<L, W>,
    staged: &StagedSide,
    bytes: &[u8],
    track: usize,
    cursor: usize,
    ring_len: usize,
) {
    side.crossover_hz[track] = staged.crossover_hz;
    side.lookahead_ms[track] = staged.lookahead_ms;
    side.designed[track] = staged.designed;
    side.detector_offset[track] = staged.detector_offset;
    side.coefficients = lane_coefficients::<L, W>(&side.designed);
    side.gain_db[LOW_BAND] = set_lane_value(side.gain_db[LOW_BAND], track, staged.gains[LOW_BAND]);
    side.gain_db[HIGH_BAND] =
        set_lane_value(side.gain_db[HIGH_BAND], track, staged.gains[HIGH_BAND]);
    side.ramps[track] = staged.ramps;
    side.cache[track] = [BandCache::empty(); 2];
    side.filter.a.ic1 = set_lane_value(side.filter.a.ic1, track, staged.filter[0]);
    side.filter.a.ic2 = set_lane_value(side.filter.a.ic2, track, staged.filter[1]);
    side.filter.b.ic1 = set_lane_value(side.filter.b.ic1, track, staged.filter[2]);
    side.filter.b.ic2 = set_lane_value(side.filter.b.ic2, track, staged.filter[3]);
    for index in 0..ring_len {
        let slot = wrap(cursor + 1 + index, ring_len) * W + track;
        side.low_ring[slot] = read_f32(bytes, LANE_HEADER_WORDS + index);
        side.high_ring[slot] = read_f32(bytes, LANE_HEADER_WORDS + ring_len + index);
    }
}

fn state_error(code: &'static str) -> StatePayloadError {
    StatePayloadError { code }
}

fn parameter_state_valid(index: usize, value: f32) -> bool {
    parameter_value_valid(&SPECS[index], value)
}

/// Bytes in one payload word. The shared codec's `WORD_BYTES`, named once here.
const fn state_payload_word_bytes() -> usize {
    miso_engine_effect_runtime::state_payload::WORD_BYTES
}

impl<L: Lane, const W: usize> Instance<L, W> {
    /// The three section lengths a payload of this instance must have.
    ///
    /// The shared codec's `validate_lengths` is not used: it derives its sizes from
    /// `expected_sizes`, which unconditionally reserves the two versioned header words this crate
    /// does not carry (W2-D2). The word codec itself is the shared one.
    fn validate_lengths(&self, sections: (usize, usize, usize)) -> Result<(), StatePayloadError> {
        let lane = (LANE_HEADER_WORDS + 2 * self.ring_len) * state_payload_word_bytes();
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
        write_side(
            output.left,
            &self.sides[0],
            track,
            self.cursor,
            self.ring_len,
        );
        write_side(
            output.right,
            &self.sides[1],
            track,
            self.cursor,
            self.ring_len,
        );
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
        let left = stage_side(input.left, self.sample_rate, self.ring_len)?;
        let right = stage_side(input.right, self.sample_rate, self.ring_len)?;
        commit_side(
            &mut self.sides[0],
            &left,
            input.left,
            track,
            self.cursor,
            self.ring_len,
        );
        commit_side(
            &mut self.sides[1],
            &right,
            input.right,
            track,
            self.cursor,
            self.ring_len,
        );
        Ok(())
    }
}

// ---------------------------------------------------------------------------------------------
// Contract surface
// ---------------------------------------------------------------------------------------------

/// Factory for the fixed two-band multiband compressor.
#[derive(Clone, Copy, Debug, Default)]
pub struct MultibandCompressorFactory;

/// A prepared scalar multiband compressor: the `WIDTH = 1` instantiation of the one body.
pub struct PreparedMultibandCompressor {
    metadata: PreparedEffectMetadata,
    instance: Instance<f32, 1>,
}

/// A prepared homogeneous bank of `W` tracks.
struct PreparedMultibandCompressorBank<L: Lane, const W: usize> {
    metadata: PreparedBankMetadata,
    effect_metadata: PreparedEffectMetadata,
    instance: Instance<L, W>,
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

fn prepare_bank<L: Lane, const W: usize>(
    factory: &MultibandCompressorFactory,
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
    let mut left = [first_left; W];
    let mut right = [first_right; W];
    let mut same_program = true;
    // Every request is validated before any fallback: a malformed bank request is an error, not a
    // reason to fall back to scalar.
    for (track, item) in request.requests.iter().copied().enumerate() {
        let candidate = expected_prepared_metadata(factory.descriptor(), item)?;
        if candidate.program_key() != metadata.program_key() {
            same_program = false;
        }
        let (track_left, track_right) = initial_defaults(item.initial_values)?;
        left[track] = track_left;
        right[track] = track_right;
    }
    if !same_program {
        return Ok(None);
    }
    let instance = Instance::<L, W>::new(left, right, metadata).ok_or(PREPARE_FAILED)?;
    Ok(Some(Box::new(PreparedMultibandCompressorBank::<L, W> {
        metadata: PreparedBankMetadata {
            width: request.width,
            program_key: metadata.program_key(),
        },
        effect_metadata: metadata,
        instance,
    })))
}

impl NativeEffectFactory for MultibandCompressorFactory {
    fn descriptor(&self) -> &'static EffectDescriptor {
        &MULTIBAND_COMPRESSOR_DESCRIPTOR_V1
    }

    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        let metadata = expected_prepared_metadata(self.descriptor(), request)?;
        let (left, right) = initial_defaults(request.initial_values)?;
        let instance = Instance::<f32, 1>::new([left], [right], metadata).ok_or(PREPARE_FAILED)?;
        Ok(Box::new(PreparedMultibandCompressor { metadata, instance }))
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
            BankWidth::Four => prepare_bank::<Simd4, 4>(self, request),
            BankWidth::Eight => prepare_bank::<Simd8, 8>(self, request),
        }
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
    fn metadata(&self) -> PreparedEffectMetadata {
        self.metadata
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

    fn reset(&mut self, kind: ResetKind) {
        self.instance.reset(kind);
    }

    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        let mut report = ProcessReport::default();
        self.instance.apply_automation(
            0,
            block.automation,
            self.metadata.automation_capacity,
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

    fn snapshot_state_payload(
        &self,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.instance.snapshot(0, output, self.metadata.state_sizes)
    }

    fn restore_state_payload(
        &mut self,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.instance.restore(0, state_layout_version, input)
    }
}

impl<L: Lane, const W: usize> PreparedNativeEffectBank for PreparedMultibandCompressorBank<L, W> {
    fn metadata(&self) -> PreparedBankMetadata {
        self.metadata.clone()
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
        self.instance.reset(kind);
    }

    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        let mut report = BankProcessReport::empty(self.metadata.width);
        // `EffectBankProcessBlock::new` has already validated the block's shape, its automation
        // offsets and its frame count against the quantum (#94 F11); what is left is the two
        // facts that belong to this instance rather than to the block.
        if block.width != self.metadata.width || block.sidechain.is_some() {
            return report;
        }
        for track in 0..W {
            let start = block.automation_offsets[track] as usize;
            let end = block.automation_offsets[track + 1] as usize;
            self.instance.apply_automation(
                track,
                &block.automation[start..end],
                self.effect_metadata.automation_capacity,
                block.first_sample,
                &mut report.reports[track],
            );
        }
        render::<L, W, false>(
            &mut self.instance,
            block.left,
            block.right,
            block.frames as usize,
            &mut report.reports,
        );
        report
    }

    fn snapshot_track_state_payload(
        &self,
        track_index: u32,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        let track = checked_track(track_index, W)?;
        self.instance
            .snapshot(track, output, self.effect_metadata.state_sizes)
    }

    fn restore_track_state_payload(
        &mut self,
        track_index: u32,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        let track = checked_track(track_index, W)?;
        self.instance.restore(track, state_layout_version, input)
    }
}

fn checked_track(track_index: u32, width: usize) -> Result<usize, StatePayloadError> {
    let track = usize::try_from(track_index).map_err(|_| state_error("effect.state.track"))?;
    if track >= width {
        return Err(state_error("effect.state.track"));
    }
    Ok(track)
}
