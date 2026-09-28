//! Four-band, six-section dual-mono parametric EQ, realised as a cascade of TPT state-variable
//! sections. The dedicated HPF and LPF are live prepared-target controls around the original four
//! bands.
//!
//! The spec transfer is the RBJ Audio EQ Cookbook's, unchanged. The *realization* is Simper's
//! trapezoidal state-variable filter (decision D2 of the #83 master plan), designed in `f64` on the
//! control plane and rounded once into six `f32` words per section. Issue #87 replaced the shipped
//! "endpoint-conditioned delta" recurrence — a direct-form-I biquad with a re-labelled denominator,
//! four `f32` histories and a division per sample — because it did not realize the transfer its own
//! grid test certified (483 of 1,488 frozen rows failed, worst 12.4859 dB) and because its recovery
//! predicate counted a correctly decaying subnormal tail as a fault.
//!
//! # What is pinned
//!
//! * **Storage.** `c1 = t / (1 + t)` with `t = g * (g + k)`, never `a1 = 1 / (1 + t)`: at 10 Hz,
//!   Q = 18 and 88.2 kHz, `t` is about 4.7e-6, so `a1` rounded to `f32` carries about 0.6 % relative
//!   error in the pole damping while `c1` carries about 6e-8 (master plan §4.2 amendment A1).
//! * **The recurrence.** `lane::kernels::svf_block`, one generic body instantiated at
//!   `WIDTH` 1, 4 and 8, so lane identity and native↔wasm identity are properties of the code.
//! * **Smoothing.** Decision D11: a prepared target starts a 64-sample linear ramp of the six
//!   **words**, with the per-sample increment precomputed as a multiply by `2^-6` and an exact
//!   assignment of the target on the final sample. There is no per-sample redesign and no division
//!   anywhere on the render path.
//! * **Denormals and faults.** Decision D7: the two integrator words are flushed inside the kernel;
//!   output finiteness is checked once per block per channel through
//!   `effect_runtime::bank`. A subnormal sample is a legal signal value, not a fault.
//!
//! # State layout
//!
//! Version 1. Per physical section and lane, 19 little-endian 32-bit words (114 words), followed
//! by the two retained dedicated-cut enable words (116 words, 464 bytes per channel); the common
//! section is the shared codec's two-word header — the layout version and data word count — and
//! nothing else, because the two channels share no state. The header makes a payload
//! self-describing, so a stale or truncated restore is rejected on the payload's own evidence and
//! not only on the caller's out-of-band `state_layout_version`. A stale payload is rejected with
//! `effect.state.version`; there is no silent migration.

use effect_contract::{
    AutomationRate, BankProcessReport, BankWidth, EffectBankProcessBlock, EffectDescriptor,
    EffectPrepareError, EffectProcessBlock, EffectQuality as Quality, EffectTargetError,
    EnumChoice, InitialParameterValue, LatencySamples, LinkModeSet, NativeEffectFactory,
    NativeEffectResponseFactory, ParameterChannel, ParameterChannelPolicy, ParameterDescriptor,
    ParameterDomain, ParameterId, ParameterMapping, ParameterUnit, PortDescriptor, PortId,
    PortLayout, PortRole, PrepareEffectBankRequest, PrepareEffectRequest, PreparedBankMetadata,
    PreparedEffectMetadata, PreparedEffectTarget, PreparedNativeEffect, PreparedNativeEffectBank,
    ProcessReport, QualityDescriptor, ResetKind, ResponseAnalysisError, ResponseSnapshotKind,
    ResponseSnapshotRequest, ResponseSnapshotSection, ResponseSnapshotSummary, SmoothingRule,
    StatePayloadError, StatePayloadInput, StatePayloadOutput, StatePayloadSizes, TailSamples,
    expected_prepared_metadata,
};
use effect_runtime::bank::{BLOCK_LIMIT, block_is_positive_zero, check_block, nonfinite_lane_mask};
use effect_runtime::params::{
    ParameterSpec, normalize_zero, parameter_value_valid as domain_valid,
};
use effect_runtime::ramp::LinearRamp;
use effect_runtime::state_payload as payload;
use engine::{SampleRateHz, is_launch_sample_rate};
use lane::kernels::{
    SvfCoef, SvfCoefStep, SvfState, svf_block, svf_block_ramped, svf_block_ramped_with_dry_mask,
    svf_cascade_interleaved_bounded, svf_cascade_interleaved_with_dry_masks_bounded,
    svf_cascade_skewed, svf_cascade_skewed_with_dry_masks,
};
use lane::{Backend, Lane, Simd4, Simd8};

mod control;
mod response;

pub use response::{
    EqResponseConfiguration, EqResponseError, EqResponseMode, EqResponseOutput, EqResponseRequest,
    EqResponseSummary, query_response_into, query_snapshot_magnitudes_into,
};

/// Number of original, user-configurable general bands. Their parameter IDs and descriptor order
/// are stable and remain separate from the physical cascade section count.
pub const EQ_BAND_COUNT: usize = 4;

/// Number of physical sections in the prepared cascade: HPF, four original bands, LPF.
pub const EQ_SECTION_COUNT: usize = 6;

const HPF_SECTION: usize = 0;
const BAND_SECTION_OFFSET: usize = 1;
const LPF_SECTION: usize = 5;
const EFFECTIVE_CASCADE_DEPTH: usize = 2;

/// Coefficient words one SVF section carries, in the pinned `c1, a2, a3, m0, m1, m2` order.
///
/// This is the width of `EqSvfWords::to_array`, of `SvfCoef`, and of `SvfCoefStep`: exactly the
/// per-section surface the cascade kernel loads.
const EQ_COEFFICIENT_WORDS: usize = 6;

/// State payload layout version. This is the sole prelaunch layout identity.
const STATE_LAYOUT_VERSION: u32 = 1;
/// Words one band occupies in a lane section of the payload.
const STATE_WORDS_PER_BAND: usize = 19;
/// Effect-owned words in each channel section.
const STATE_LANE_WORDS: usize = EQ_SECTION_COUNT * STATE_WORDS_PER_BAND + 2;
const STATE_HPF_ENABLE_WORD: usize = EQ_SECTION_COUNT * STATE_WORDS_PER_BAND;
const STATE_LPF_ENABLE_WORD: usize = STATE_HPF_ENABLE_WORD + 1;
/// The payload shape, stamped into the common section by the shared codec.
///
/// W2-D2's rule for a crate that has to bump its layout anyway: adopt the runtime header **inside**
/// that bump, so the layout is never versioned twice. The header is two words — the version and the
/// data word count — which is what makes a payload self-describing: a stale or truncated restore is
/// rejected on the payload's own evidence rather than on the caller's word alone.
const STATE_LAYOUT: payload::StateLayout = payload::StateLayout {
    version: STATE_LAYOUT_VERSION,
    common_words: 0,
    lane_words: STATE_LANE_WORDS as u32,
};

/// Byte lengths the descriptor advertises, derived from the layout rather than written out.
const STATE_SIZES: payload::StatePayloadSizes = payload::expected_sizes(&STATE_LAYOUT);

/// Samples a prepared target takes to reach its destination (`SmoothingRule::Linear`, D11).
const RAMP_SAMPLES: u32 = 64;
/// `1 / RAMP_SAMPLES` as an exact power of two: the ramp multiplies, it never divides.
const RAMP_SCALE: f32 = 1.0 / RAMP_SAMPLES as f32;
/// Widest bank this crate binds; sizes the small fixed per-lane scratch arrays.
const MAX_LANES: usize = 8;

#[cfg(any(test, feature = "test-support"))]
std::thread_local! {
    static DESIGN_CALLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// Stationary depth-one tail passes that ran without a dry select (issue #976).
    static SELECT_FREE_TAIL_PASSES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// Stationary depth-two passes that ran the masked kernel (issue #977).
    static MASKED_PAIR_PASSES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(any(test, feature = "test-support"))]
fn reset_design_calls() {
    DESIGN_CALLS.with(|calls| calls.set(0));
}

#[cfg(any(test, feature = "test-support"))]
fn design_call_count() -> usize {
    DESIGN_CALLS.with(std::cell::Cell::get)
}

/// Resets the existing designer counter for host integration gates.
#[cfg(feature = "test-support")]
pub fn test_only_reset_design_calls() {
    reset_design_calls();
}

/// Reads the existing designer counter for host integration gates.
#[cfg(feature = "test-support")]
#[must_use]
pub fn test_only_design_call_count() -> usize {
    design_call_count()
}

/// Counts one select-free depth-one tail pass of the stationary cascade.
///
/// Test builds only: the one gate that sees a performance-only regression (issue #976 M3) needs to
/// know which kernel arm the odd last section took, and no rendered bit says so.
#[cfg(any(test, feature = "test-support"))]
fn count_select_free_tail_pass() {
    SELECT_FREE_TAIL_PASSES.with(|passes| passes.set(passes.get().saturating_add(1)));
}

#[cfg(any(test, feature = "test-support"))]
fn reset_select_free_tail_passes() {
    SELECT_FREE_TAIL_PASSES.with(|passes| passes.set(0));
}

#[cfg(any(test, feature = "test-support"))]
fn select_free_tail_pass_count() -> usize {
    SELECT_FREE_TAIL_PASSES.with(std::cell::Cell::get)
}

/// Resets this thread's count of select-free depth-one tail passes (issue #976).
#[cfg(feature = "test-support")]
pub fn test_only_reset_select_free_tail_passes() {
    reset_select_free_tail_passes();
}

/// This thread's count of stationary depth-one tail passes that ran without a dry select: the odd
/// last live section of an admitted block whose dry masks were empty on both channels.
#[cfg(feature = "test-support")]
#[must_use]
pub fn test_only_select_free_tail_passes() -> usize {
    select_free_tail_pass_count()
}

/// Counts one stationary depth-two pass that ran the masked kernel.
///
/// Test builds only: issue #977 runs every pair of an admitted plan select-free, which no rendered
/// bit can show -- a select whose every lane returns the wet word is invisible -- so this is the
/// gate on the schedule half of the change. The depth-one tail keeps its own counter
/// ([`count_select_free_tail_pass`]) and #976's rule.
#[cfg(any(test, feature = "test-support"))]
fn count_masked_pair_pass() {
    MASKED_PAIR_PASSES.with(|passes| passes.set(passes.get().saturating_add(1)));
}

#[cfg(any(test, feature = "test-support"))]
fn reset_masked_pair_passes() {
    MASKED_PAIR_PASSES.with(|passes| passes.set(0));
}

#[cfg(any(test, feature = "test-support"))]
fn masked_pair_pass_count() -> usize {
    MASKED_PAIR_PASSES.with(std::cell::Cell::get)
}

/// Resets this thread's count of masked stationary depth-two passes (issue #977).
#[cfg(feature = "test-support")]
pub fn test_only_reset_masked_pair_passes() {
    reset_masked_pair_passes();
}

/// This thread's count of stationary depth-two passes that ran the masked kernel: every pair of a
/// refused or all-live (six-section) plan, and none of an admitted one.
#[cfg(feature = "test-support")]
#[must_use]
pub fn test_only_masked_pair_passes() -> usize {
    masked_pair_pass_count()
}

#[cfg(any(test, feature = "test-support"))]
std::thread_local! {
    /// Ramping blocks whose section list dropped at least one section (issue #1005).
    static RAMPING_ELIDED_BLOCKS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Counts one ramping block that ran a shorter list than the full cascade.
///
/// Test builds only: issue #1005's differential has to show that the ramping list engaged, and no
/// rendered bit says so -- an elided identity section renders what the executed one renders.
#[cfg(any(test, feature = "test-support"))]
fn count_ramping_plan(length: usize) {
    if length < EQ_SECTION_COUNT {
        RAMPING_ELIDED_BLOCKS.with(|blocks| blocks.set(blocks.get().saturating_add(1)));
    }
}

#[cfg(any(test, feature = "test-support"))]
fn reset_ramping_elided_blocks() {
    RAMPING_ELIDED_BLOCKS.with(|blocks| blocks.set(0));
}

#[cfg(any(test, feature = "test-support"))]
fn ramping_elided_block_count() -> usize {
    RAMPING_ELIDED_BLOCKS.with(std::cell::Cell::get)
}

/// Resets this thread's count of ramping blocks that dropped a section (issue #1005).
#[cfg(feature = "test-support")]
pub fn test_only_reset_ramping_elided_blocks() {
    reset_ramping_elided_blocks();
}

/// This thread's count of ramping blocks (per channel pair, or per collapsed channel) whose
/// `ramping_sections` list was shorter than the full cascade.
#[cfg(feature = "test-support")]
#[must_use]
pub fn test_only_ramping_elided_blocks() -> usize {
    ramping_elided_block_count()
}

#[cfg(test)]
std::thread_local! {
    /// Unit tests only: whether a non-stationary block takes issue #1005's section list.
    ///
    /// `false`, the default, keeps it on the batch-head path, every section of each channel
    /// through [`Channel::process_block`]. The unit tests written before #1005 call
    /// `process_channels(.., false)` as their *full per-section* oracle for the stationary
    /// elision, and a list that elides too would turn "elided == full" into "elided == elided";
    /// so they keep testing exactly what they tested. Issue #1005's gates set it: the bank
    /// differential renders its candidate arm with it and its oracle arm without it. No feature
    /// build carries the switch, so every other binary -- the shipped module, the bench, the
    /// integration tests -- always takes the list.
    static RAMPING_LIST: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// `false` while this thread's unit tests render non-stationary blocks the batch-head way; see
/// [`RAMPING_LIST`].
#[cfg(test)]
fn ramping_list_enabled() -> bool {
    RAMPING_LIST.with(std::cell::Cell::get)
}

/// Frozen V1 section filter families.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EqBandKind {
    /// Peaking bell.
    Bell = 1,
    /// Low shelving.
    LowShelf = 2,
    /// High shelving.
    HighShelf = 3,
    /// Second-order low pass.
    LowPass = 4,
    /// Second-order high pass.
    HighPass = 5,
    /// Second-order notch.
    Notch = 6,
}

impl EqBandKind {
    /// Decodes the enumeration parameter's frozen numeric encoding.
    fn from_value(value: f32) -> Option<Self> {
        match value.to_bits() {
            bits if bits == 1.0_f32.to_bits() => Some(Self::Bell),
            bits if bits == 2.0_f32.to_bits() => Some(Self::LowShelf),
            bits if bits == 3.0_f32.to_bits() => Some(Self::HighShelf),
            bits if bits == 4.0_f32.to_bits() => Some(Self::LowPass),
            bits if bits == 5.0_f32.to_bits() => Some(Self::HighPass),
            bits if bits == 6.0_f32.to_bits() => Some(Self::Notch),
            _ => None,
        }
    }
}

/// Stable parameter IDs for one cascade position.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EqBandDescriptor {
    /// General-band index, `0..EQ_BAND_COUNT`.
    pub index: u8,
    /// Position in the physical cascade; one greater than `index` because HPF is prepended.
    pub cascade_order: u8,
    /// Boolean enable.
    pub enabled: ParameterId,
    /// Filter family, one of [`EqBandKind`].
    pub kind: ParameterId,
    /// Centre or corner frequency in Hz.
    pub frequency_hz: ParameterId,
    /// Bell or shelf gain in dB.
    pub gain_db: ParameterId,
    /// Quality factor.
    pub q: ParameterId,
    /// Shelf slope `S`.
    pub shelf_slope: ParameterId,
}

const fn parameter_id(value: u32) -> ParameterId {
    match ParameterId::new(value) {
        Some(value) => value,
        None => panic!("nonzero parameter id"),
    }
}

const fn effect_id(value: &'static str) -> effect_contract::EffectId {
    match effect_contract::EffectId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("valid effect id"),
    }
}

const fn port_id(value: &'static str) -> PortId {
    match PortId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("valid port id"),
    }
}

/// The first stable parameter ID of general band `band`; bands are spaced sixteen apart.
const fn band_base(band: usize) -> u32 {
    band as u32 * 16 + 1
}

/// Four static general-band descriptors in their stable parameter order. Their physical cascade
/// positions are one through four because the prepared cuts occupy positions zero and five.
pub const EQ_BAND_DESCRIPTORS: [EqBandDescriptor; EQ_BAND_COUNT] = {
    let mut bands = [EqBandDescriptor {
        index: 0,
        cascade_order: 0,
        enabled: parameter_id(1),
        kind: parameter_id(2),
        frequency_hz: parameter_id(3),
        gain_db: parameter_id(4),
        q: parameter_id(5),
        shelf_slope: parameter_id(6),
    }; EQ_BAND_COUNT];
    let mut band = 0;
    while band < EQ_BAND_COUNT {
        let base = band_base(band);
        bands[band] = EqBandDescriptor {
            index: band as u8,
            cascade_order: (band + BAND_SECTION_OFFSET) as u8,
            enabled: parameter_id(base),
            kind: parameter_id(base + 1),
            frequency_hz: parameter_id(base + 2),
            gain_db: parameter_id(base + 3),
            q: parameter_id(base + 4),
            shelf_slope: parameter_id(base + 5),
        };
        band += 1;
    }
    bands
};

const KIND_CHOICES: [EnumChoice; 6] = [
    EnumChoice {
        value: 1.0,
        label: "bell",
    },
    EnumChoice {
        value: 2.0,
        label: "low-shelf",
    },
    EnumChoice {
        value: 3.0,
        label: "high-shelf",
    },
    EnumChoice {
        value: 4.0,
        label: "low-pass",
    },
    EnumChoice {
        value: 5.0,
        label: "high-pass",
    },
    EnumChoice {
        value: 6.0,
        label: "notch",
    },
];

/// Display names, band major, in descriptor order. The table below is generated from them, so the
/// twenty-four descriptors cannot drift apart in a field no reader is comparing.
const PARAMETER_NAMES: [&str; EQ_BAND_COUNT * 6] = [
    "band-1-enabled",
    "band-1-kind",
    "band-1-frequency",
    "band-1-gain",
    "band-1-q",
    "band-1-shelf-slope",
    "band-2-enabled",
    "band-2-kind",
    "band-2-frequency",
    "band-2-gain",
    "band-2-q",
    "band-2-shelf-slope",
    "band-3-enabled",
    "band-3-kind",
    "band-3-frequency",
    "band-3-gain",
    "band-3-q",
    "band-3-shelf-slope",
    "band-4-enabled",
    "band-4-kind",
    "band-4-frequency",
    "band-4-gain",
    "band-4-q",
    "band-4-shelf-slope",
];

/// Default centre frequency of each band, in Hz.
const FREQUENCY_DEFAULTS: [f32; EQ_BAND_COUNT] = [80.0, 400.0, 2_000.0, 10_000.0];

/// Lowest and highest admitted frequency, gain, Q and shelf slope, in field order 2..6.
const NUMERIC_SPECS: [ParameterSpec; 4] = [
    ParameterSpec::logarithmic(10.0, 20_000.0, 80.0),
    ParameterSpec::continuous(-24.0, 24.0, 0.0),
    ParameterSpec::logarithmic(0.1, 18.0, core::f32::consts::FRAC_1_SQRT_2),
    ParameterSpec::continuous(0.1, 1.0, 1.0),
];

/// Per-field descriptor columns, in field order: enabled, kind, frequency, gain, Q, shelf slope.
/// Only the frequency default varies by band ([`FREQUENCY_DEFAULTS`]); everything else is shared,
/// so the twenty-four descriptors are generated from these six columns and cannot drift apart in a
/// field no reader is comparing.
const DISPLAY_UNITS: [&str; 6] = ["on/off", "type", "Hz", "dB", "Q", "S"];
const UNITS: [ParameterUnit; 6] = [
    ParameterUnit::Linear,
    ParameterUnit::Linear,
    ParameterUnit::Hz,
    ParameterUnit::Db,
    ParameterUnit::Ratio,
    ParameterUnit::Ratio,
];
const DOMAINS: [ParameterDomain; 6] = [
    ParameterDomain::Boolean,
    ParameterDomain::Enumeration,
    ParameterDomain::Continuous,
    ParameterDomain::Continuous,
    ParameterDomain::Continuous,
    ParameterDomain::Continuous,
];
const MINIMA: [Option<f32>; 6] = [None, None, Some(10.0), Some(-24.0), Some(0.1), Some(0.1)];
const MAXIMA: [Option<f32>; 6] = [
    None,
    None,
    Some(20_000.0),
    Some(24.0),
    Some(18.0),
    Some(1.0),
];
const DEFAULTS: [f32; 6] = [0.0, 1.0, 0.0, 0.0, core::f32::consts::FRAC_1_SQRT_2, 1.0];
const MAPPINGS: [ParameterMapping; 6] = [
    ParameterMapping::Stepped,
    ParameterMapping::Stepped,
    ParameterMapping::Logarithmic,
    ParameterMapping::Linear,
    ParameterMapping::Logarithmic,
    ParameterMapping::Linear,
];

/// One descriptor of one field of one band.
const fn parameter(band: usize, field: usize) -> ParameterDescriptor {
    let automatable = field >= 2;
    ParameterDescriptor {
        id: parameter_id(band_base(band) + field as u32),
        display_name: PARAMETER_NAMES[band * 6 + field],
        display_unit: DISPLAY_UNITS[field],
        unit: UNITS[field],
        domain: DOMAINS[field],
        minimum: MINIMA[field],
        maximum: MAXIMA[field],
        default_value: if field == 2 {
            FREQUENCY_DEFAULTS[band]
        } else {
            DEFAULTS[field]
        },
        mapping: MAPPINGS[field],
        automation_rate: if automatable {
            AutomationRate::Block
        } else {
            AutomationRate::None
        },
        channel_policy: ParameterChannelPolicy::PerLane,
        smoothing: if automatable {
            SmoothingRule::Linear
        } else {
            SmoothingRule::None
        },
        smoothing_samples: if automatable { RAMP_SAMPLES } else { 0 },
        readable: true,
        automatable,
        enum_choices: if field == 1 { &KIND_CHOICES } else { &[] },
        lattice: effect_contract::default_parameter_lattice(
            UNITS[field],
            DOMAINS[field],
            MAPPINGS[field],
        ),
    }
}

const fn cut_parameter(
    id: u32,
    display_name: &'static str,
    unit: ParameterUnit,
    domain: ParameterDomain,
    bounds: (Option<f32>, Option<f32>, f32),
    mapping: ParameterMapping,
) -> ParameterDescriptor {
    ParameterDescriptor {
        id: parameter_id(id),
        display_name,
        display_unit: match unit {
            ParameterUnit::Hz => "Hz",
            ParameterUnit::Ratio => "Q",
            ParameterUnit::Linear => "on/off",
            _ => "",
        },
        unit,
        domain,
        minimum: bounds.0,
        maximum: bounds.1,
        default_value: bounds.2,
        mapping,
        automation_rate: AutomationRate::Block,
        channel_policy: ParameterChannelPolicy::PerLane,
        smoothing: SmoothingRule::Linear,
        smoothing_samples: RAMP_SAMPLES,
        readable: true,
        automatable: true,
        enum_choices: &[],
        lattice: effect_contract::default_parameter_lattice(unit, domain, mapping),
    }
}

const EQ_PARAMETERS: [ParameterDescriptor; EQ_BAND_COUNT * 6 + 6] = {
    let mut table = [parameter(0, 0); EQ_BAND_COUNT * 6 + 6];
    let mut band = 0;
    while band < EQ_BAND_COUNT {
        let mut field = 0;
        while field < 6 {
            table[band * 6 + field] = parameter(band, field);
            field += 1;
        }
        band += 1;
    }
    table[EQ_BAND_COUNT * 6] = cut_parameter(
        65,
        "hpf-enabled",
        ParameterUnit::Linear,
        ParameterDomain::Boolean,
        (None, None, 0.0),
        ParameterMapping::Stepped,
    );
    table[EQ_BAND_COUNT * 6 + 1] = cut_parameter(
        66,
        "hpf-frequency",
        ParameterUnit::Hz,
        ParameterDomain::Continuous,
        (Some(10.0), Some(20_000.0), 80.0),
        ParameterMapping::Logarithmic,
    );
    table[EQ_BAND_COUNT * 6 + 2] = cut_parameter(
        67,
        "hpf-q",
        ParameterUnit::Ratio,
        ParameterDomain::Continuous,
        (Some(0.1), Some(18.0), core::f32::consts::FRAC_1_SQRT_2),
        ParameterMapping::Logarithmic,
    );
    table[EQ_BAND_COUNT * 6 + 3] = cut_parameter(
        81,
        "lpf-enabled",
        ParameterUnit::Linear,
        ParameterDomain::Boolean,
        (None, None, 0.0),
        ParameterMapping::Stepped,
    );
    table[EQ_BAND_COUNT * 6 + 4] = cut_parameter(
        82,
        "lpf-frequency",
        ParameterUnit::Hz,
        ParameterDomain::Continuous,
        (Some(10.0), Some(20_000.0), 18_000.0),
        ParameterMapping::Logarithmic,
    );
    table[EQ_BAND_COUNT * 6 + 5] = cut_parameter(
        83,
        "lpf-q",
        ParameterUnit::Ratio,
        ParameterDomain::Continuous,
        (Some(0.1), Some(18.0), core::f32::consts::FRAC_1_SQRT_2),
        ParameterMapping::Logarithmic,
    );
    table
};

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

const QUALITIES: [QualityDescriptor; 4] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
];

const fn quality(sample_rate: u32) -> QualityDescriptor {
    QualityDescriptor {
        quality: Quality::Normal,
        sample_rate,
        latency: LatencySamples(0),
        tail: TailSamples::Infinite,
        maximum_state: StatePayloadSizes {
            common_bytes: STATE_SIZES.common as u32,
            left_bytes: STATE_SIZES.left as u32,
            right_bytes: STATE_SIZES.right as u32,
        },
        scratch_fixed_bytes: 0,
        scratch_bytes_per_frame: 0,
    }
}

/// Authoritative static V1 effect metadata.
pub static PARAMETRIC_EQ_DESCRIPTOR: EffectDescriptor = EffectDescriptor {
    id: effect_id("miso.parametric-eq"),
    display_name: "Parametric EQ",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: STATE_LAYOUT_VERSION,
    supported_link_modes: LinkModeSet::DUAL_MONO,
    parameters: &EQ_PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
    observations: &[],
};

/// The six retained `f32` words of one TPT state-variable section.
///
/// Designed in `f64` and rounded exactly once, which is what makes the words a target-independent
/// function of the parameters (the `f64` design uses `math`, never the platform libm).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EqSvfWords {
    /// `t / (1 + t)` with `t = g * (g + k)` — the pole's distance from `z = 1` (amendment A1).
    pub c1: f32,
    /// `g * (1 - c1)`.
    pub a2: f32,
    /// `g * a2`.
    pub a3: f32,
    /// Direct output mix.
    pub m0: f32,
    /// Band output mix.
    pub m1: f32,
    /// Low output mix.
    pub m2: f32,
}

impl EqSvfWords {
    /// The exact identity section: `y = x` bit for bit, with no state growth and no mask.
    pub const IDENTITY: Self = Self {
        c1: 0.0,
        a2: 0.0,
        a3: 0.0,
        m0: 1.0,
        m1: 0.0,
        m2: 0.0,
    };

    /// The words in the pinned order `c1, a2, a3, m0, m1, m2`.
    #[must_use]
    pub const fn to_array(self) -> [f32; 6] {
        [self.c1, self.a2, self.a3, self.m0, self.m1, self.m2]
    }

    /// Rebuilds a set from [`EqSvfWords::to_array`] order.
    #[must_use]
    pub const fn from_array(words: [f32; 6]) -> Self {
        Self {
            c1: words[0],
            a2: words[1],
            a3: words[2],
            m0: words[3],
            m1: words[4],
            m2: words[5],
        }
    }
}

/// Why a section could not be designed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EqDesignError {
    /// A parameter, the sample rate, or their combination is outside the frozen domain.
    InvalidInput,
    /// The rounded words are not a stable, finite realization.
    Coefficients,
}

/// The `f64` mapping, before the single rounding. Frozen operation order.
///
/// Simper 2013: `g = tan(pi f0 / fs)` prewarped per kind, `k` the damping word, and the output mix
/// `(m0, m1, m2)` that selects the response. The shelf damping is RBJ's `1/Q_S`, which is what makes
/// `alpha_S` and `alpha_Q` the same quantity and the shelf transfer the cookbook's.
///
/// Returned in the pinned order `c1, a2, a3, m0, m1, m2`. Public because it is the quantity an
/// oracle compares against: [`design_svf`] is exactly this followed by one rounding.
#[must_use]
pub fn design_svf_words_f64(
    kind: EqBandKind,
    frequency_hz: f64,
    gain_db: f64,
    q: f64,
    shelf_slope: f64,
    sample_rate_hz: f64,
) -> [f64; 6] {
    let amplitude = math::pow(10.0, gain_db / 40.0);
    let warped = math::tan(core::f64::consts::PI * frequency_hz / sample_rate_hz);
    let shelf_k = ((amplitude + 1.0 / amplitude) * (1.0 / shelf_slope - 1.0) + 2.0).sqrt();
    let (g, k, m0, m1, m2) = match kind {
        EqBandKind::LowPass => (warped, 1.0 / q, 0.0, 0.0, 1.0),
        EqBandKind::HighPass => (warped, 1.0 / q, 1.0, -(1.0 / q), -1.0),
        EqBandKind::Notch => (warped, 1.0 / q, 1.0, -(1.0 / q), 0.0),
        EqBandKind::Bell => {
            let k = 1.0 / (q * amplitude);
            (warped, k, 1.0, k * (amplitude * amplitude - 1.0), 0.0)
        }
        EqBandKind::LowShelf => (
            warped / amplitude.sqrt(),
            shelf_k,
            1.0,
            shelf_k * (amplitude - 1.0),
            amplitude * amplitude - 1.0,
        ),
        EqBandKind::HighShelf => (
            warped * amplitude.sqrt(),
            shelf_k,
            amplitude * amplitude,
            shelf_k * (1.0 - amplitude) * amplitude,
            1.0 - amplitude * amplitude,
        ),
    };
    let t = g * (g + k);
    let c1 = t / (1.0 + t);
    let a1 = 1.0 - c1;
    let a2 = g * a1;
    let a3 = g * a2;
    [c1, a2, a3, m0, m1, m2]
}

/// Spectral norm of the zero-input state transition `M = [[1-2c1, -2a2], [2a2, 1-2a3]]`, in `f64`.
///
/// A linear ramp between two word triples is safe exactly when every point of it is contractive,
/// and `‖M‖₂` is convex in the words, so checking the endpoints checks the whole ramp. This is the
/// stability predicate that replaces the delta realization's Jury test: it is a statement about the
/// matrix the kernel actually iterates, not about a polynomial nothing evaluates.
#[must_use]
pub fn word_spectral_norm(words: EqSvfWords) -> f64 {
    let a00 = 1.0 - 2.0 * f64::from(words.c1);
    let a01 = -2.0 * f64::from(words.a2);
    let a10 = 2.0 * f64::from(words.a2);
    let a11 = 1.0 - 2.0 * f64::from(words.a3);
    let first = a00 * a00 + a10 * a10;
    let second = a01 * a01 + a11 * a11;
    let cross = a00 * a01 + a10 * a11;
    let difference = first - second;
    let largest = 0.5 * (first + second + (difference * difference + 4.0 * cross * cross).sqrt());
    largest.sqrt()
}

/// Largest spectral norm accepted from a rounded word set: one `f32` rounding above contractive.
const NORM_TOLERANCE: f64 = 1.0 + 1.0 / 4_194_304.0;

/// Conservative finite bound for the three output-mix words accepted at the prepared-target
/// boundary. It is derived from the descriptor's legal gain, Q and shelf-slope domains: with
/// `A = 10^(gain/40)`, `A < 4` and `1/A < 4`; with `Q >= 0.1` and `S >= 0.1`, the shelf damping
/// word is below `sqrt(74) < 9`. Therefore high-shelf `|m1| < 9*3*4 = 108`, low-shelf `|m1| <
/// 9*3 = 27`, bell `|m1| < 40`, and all remaining mix magnitudes are at most 16. The rounded
/// boundary freezes 128, leaving room for the single f32 rounding while rejecting arbitrary huge
/// finite payloads.
const OUTPUT_MIX_BOUND: f32 = 128.0;

/// Designs one section's six `f32` words from the frozen RBJ parameter domain.
///
/// # Errors
///
/// [`EqDesignError::InvalidInput`] if the sample rate is not a launch rate, a parameter is outside
/// its domain, or the centre frequency is at or above Nyquist. [`EqDesignError::Coefficients`] if
/// the rounded words are not finite or not contractive.
pub fn design_svf(
    kind: EqBandKind,
    frequency_hz: f32,
    gain_db: f32,
    q: f32,
    shelf_slope: f32,
    sample_rate: SampleRateHz,
) -> Result<EqSvfWords, EqDesignError> {
    #[cfg(any(test, feature = "test-support"))]
    DESIGN_CALLS.with(|calls| calls.set(calls.get().saturating_add(1)));
    if !is_launch_sample_rate(sample_rate)
        || !numeric_value_valid(0, frequency_hz)
        || !numeric_value_valid(1, gain_db)
        || !numeric_value_valid(2, q)
        || !numeric_value_valid(3, shelf_slope)
        || frequency_hz >= sample_rate.0 as f32 * 0.5
    {
        return Err(EqDesignError::InvalidInput);
    }
    let exact = design_svf_words_f64(
        kind,
        f64::from(frequency_hz),
        f64::from(gain_db),
        f64::from(q),
        f64::from(shelf_slope),
        f64::from(sample_rate.0),
    );
    // The single rounding. `-0.0` is normalised away so that adding a zero ramp increment to a word
    // is bit-preserving on every lane, which is what makes an idle lane of a ramping bank identical
    // to the same lane of a settled one.
    let words = EqSvfWords::from_array(exact.map(|value| {
        let rounded = value as f32;
        if rounded == 0.0 { 0.0 } else { rounded }
    }));
    validate_rounded_svf(words)?;
    Ok(words)
}

/// Validates the rounded words that the SVF kernel actually consumes.
///
/// This is shared by the trigonometric designer and the prepared-target decoder. The decoder
/// calls it without redesigning a section, so this predicate proves finiteness, canonical zeros,
/// the existing state-transition shape, the conservative output-mix bound, and the rounded
/// spectral-norm limit. It cannot prove that arbitrary stable coefficient words implement the
/// semantic cutoff or Q in a target header; the Rust preparer remains the coefficient authority.
fn validate_rounded_svf(words: EqSvfWords) -> Result<(), EqDesignError> {
    let values = words.to_array();
    if !values.iter().all(|value| value.is_finite())
        || values
            .iter()
            .any(|value| *value == 0.0 && value.to_bits() != 0)
        || !(0.0..1.0).contains(&words.c1)
        || words.a2 <= 0.0
        || words.a3 < 0.0
        || words.m0.abs() > OUTPUT_MIX_BOUND
        || words.m1.abs() > OUTPUT_MIX_BOUND
        || words.m2.abs() > OUTPUT_MIX_BOUND
        || word_spectral_norm(words) > NORM_TOLERANCE
    {
        return Err(EqDesignError::Coefficients);
    }
    Ok(())
}

/// `true` if `value` is inside the domain of numeric field `field` (0 frequency, 1 gain, 2 Q,
/// 3 shelf slope), using the shared descriptor validator rather than a fourth copy of the ranges.
fn numeric_value_valid(field: usize, value: f32) -> bool {
    domain_valid(&NUMERIC_SPECS[field], value)
}

/// The control-plane parameter state of one band of one track.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct BandTarget {
    enabled: bool,
    kind: EqBandKind,
    frequency: f32,
    gain: f32,
    q: f32,
    slope: f32,
}

impl BandTarget {
    /// The words this band settles at. A disabled band is the exact identity at every parameter.
    fn words(&self, sample_rate: SampleRateHz) -> Result<EqSvfWords, EqDesignError> {
        if !self.enabled {
            return Ok(EqSvfWords::IDENTITY);
        }
        design_svf(
            self.kind,
            self.frequency,
            self.gain,
            self.q,
            self.slope,
            sample_rate,
        )
    }

    /// `true` when `other` is this band bit for bit.
    ///
    /// Issue #144 item 6. Derived `PartialEq` would compare the four `f32` fields with `==`, which
    /// makes `-0.0` equal `0.0` and makes a `NaN` band unequal to itself. Neither can reach a
    /// stored `BandTarget` today -- values are normalised and domain-checked on the way in -- but
    /// the hoist this feeds decides whether an `f64` coefficient design is skipped, and a decision
    /// of that weight is made on bits rather than on a coincidence of the current admission rules.
    fn same_bits(&self, other: &Self) -> bool {
        self.enabled == other.enabled
            && self.kind == other.kind
            && self
                .numeric()
                .iter()
                .zip(other.numeric().iter())
                .all(|(left, right)| left.to_bits() == right.to_bits())
    }

    /// The four automatable fields, in payload order.
    const fn numeric(&self) -> [f32; 4] {
        [self.frequency, self.gain, self.q, self.slope]
    }

    /// Writes automatable field `field`.
    fn set_numeric(&mut self, field: usize, value: f32) {
        match field {
            0 => self.frequency = value,
            1 => self.gain = value,
            2 => self.q = value,
            _ => self.slope = value,
        }
    }
}

/// One cascade section of one channel, across `L::WIDTH` tracks.
#[derive(Clone, Copy)]
struct Section<L: Lane> {
    /// Words the most recently processed frame used.
    coef: SvfCoef<L>,
    /// Per-sample increment; exactly zero on every settled lane.
    step: SvfCoefStep<L>,
    /// Words the ramp is heading for, assigned exactly on its final sample.
    target: SvfCoef<L>,
    /// The two integrator words.
    state: SvfState<L>,
}

/// Reads word `index` of a coefficient set in the pinned order.
/// The words lane `track` of `section` is currently heading for.
///
/// These are exactly the words `BandTarget::words` last returned for this lane's stored band, so
/// reading them back is the same value the design would recompute -- see `Channel::target_words`.
fn coef_word<L: Lane>(coefficients: &SvfCoef<L>, index: usize) -> L {
    match index {
        0 => coefficients.c1,
        1 => coefficients.a2,
        2 => coefficients.a3,
        3 => coefficients.m0,
        4 => coefficients.m1,
        _ => coefficients.m2,
    }
}

/// Writes word `index` of a coefficient set in the pinned order.
fn coef_word_mut<L: Lane>(coefficients: &mut SvfCoef<L>, index: usize) -> &mut L {
    match index {
        0 => &mut coefficients.c1,
        1 => &mut coefficients.a2,
        2 => &mut coefficients.a3,
        3 => &mut coefficients.m0,
        4 => &mut coefficients.m1,
        _ => &mut coefficients.m2,
    }
}

/// Writes increment `index` of a step set in the pinned order.
fn step_word_mut<L: Lane>(step: &mut SvfCoefStep<L>, index: usize) -> &mut L {
    match index {
        0 => &mut step.c1,
        1 => &mut step.a2,
        2 => &mut step.a3,
        3 => &mut step.m0,
        4 => &mut step.m1,
        _ => &mut step.m2,
    }
}

/// Reads lane `lane` of a vector. Control plane only: it leaves the vector domain.
fn lane_get<L: Lane>(value: L, lane: usize) -> f32 {
    let mut words = [0.0_f32; MAX_LANES];
    value.store(&mut words[..L::WIDTH]);
    words[lane]
}

/// Writes lane `lane` of a vector. Control plane only.
fn lane_set<L: Lane>(value: &mut L, lane: usize, sample: f32) {
    let mut words = [0.0_f32; MAX_LANES];
    value.store(&mut words[..L::WIDTH]);
    words[lane] = sample;
    *value = L::load(&words[..L::WIDTH]);
}

/// The `-0.0` bit pattern.
///
/// It is the one input value an *elided* identity section would not reproduce, and the one state
/// word that can defeat the sign argument the elision gate rests on. See [`cascade_sections`].
const NEGATIVE_ZERO_BITS: u32 = 0x8000_0000;

/// Sign-clearing mask, so a magnitude comparison is one integer `and` and one integer compare.
const MAGNITUDE_MASK: u32 = 0x7fff_ffff;

/// Magnitude bits of [`BLOCK_LIMIT`], the ceiling the elision gate applies to its input planes.
///
/// `f32` magnitude bit patterns are monotone in magnitude, so `bits & MAGNITUDE_MASK > this` is
/// exactly `|x| > BLOCK_LIMIT` — and because every infinity and every NaN has magnitude bits at or
/// above `0x7f80_0000`, that single compare also refuses both. The gate needs all three refusals:
/// a non-finite value does not survive an identity section unchanged (`0.0 * inf` is `NaN`), and
/// the bound is what keeps a live section from overflowing into one mid-cascade.
const ELISION_MAGNITUDE_CEILING: u32 = BLOCK_LIMIT.to_bits();

/// The six [`EqSvfWords::IDENTITY`] words as raw bits, in the pinned order.
///
/// Bits rather than floats, for the same reason [`Channel::state_bits`] uses them: the flag this
/// feeds claims a section is the *exact* identity, and `-0.0 == 0.0` would let a section that is
/// not claim that it is.
const IDENTITY_WORD_BITS: [u32; 6] = {
    let words = EqSvfWords::IDENTITY.to_array();
    [
        words[0].to_bits(),
        words[1].to_bits(),
        words[2].to_bits(),
        words[3].to_bits(),
        words[4].to_bits(),
        words[5].to_bits(),
    ]
};

#[inline]
fn words_are_identity(words: EqSvfWords) -> bool {
    words
        .to_array()
        .into_iter()
        .zip(IDENTITY_WORD_BITS)
        .all(|(word, bits)| word.to_bits() == bits)
}

/// Reads the raw bits of one lane of a vector. Control plane only: it leaves the vector domain.
fn lane_bits<L: Lane>(value: L, lane: usize) -> u32 {
    debug_assert!(L::WIDTH <= MAX_LANES);
    let mut words = [0_u32; MAX_LANES];
    value.store_bits(&mut words[..L::WIDTH]);
    words[lane]
}

/// `true` when every lane of `value` holds exactly the bit pattern `bits`.
fn lane_bits_all<L: Lane>(value: L, bits: u32) -> bool {
    debug_assert!(L::WIDTH <= MAX_LANES);
    let mut words = [0_u32; MAX_LANES];
    value.store_bits(&mut words[..L::WIDTH]);
    words[..L::WIDTH].iter().all(|word| *word == bits)
}

/// Magnitude bits of `f32::INFINITY`: every word whose magnitude bits reach this is an infinity or a
/// NaN.
const NON_FINITE_MAGNITUDE: u32 = 0x7f80_0000;

/// Magnitude bits of [`lane::FLUSH_EPS`], the smallest magnitude `flush` keeps.
const INERT_MAGNITUDE_FLOOR: u32 = lane::FLUSH_EPS.to_bits();

/// `true` when every lane of `value` is *inert*: exactly `+0.0`, or finite with magnitude bits in
/// `[INERT_MAGNITUDE_FLOOR, ELISION_MAGNITUDE_CEILING]` (issue #979).
///
/// Bits, not float compares: `-0.0` (magnitude bits `0`) and every magnitude below `FLUSH_EPS`
/// fall under the floor, and every infinity and NaN above the ceiling, so all three refuse. The
/// lanes are folded with non-short-circuiting `&`/`|` into one branch-free reduction, like the
/// `+0.0` test it replaced, because it runs for every dead section on every stationary block.
fn lane_is_inert<L: Lane>(value: L) -> bool {
    debug_assert!(L::WIDTH <= MAX_LANES);
    let mut words = [0_u32; MAX_LANES];
    value.store_bits(&mut words[..L::WIDTH]);
    let mut inert = true;
    for word in &words[..L::WIDTH] {
        // `FLOOR <= m <= CEILING` as one unsigned compare: below the floor the subtraction wraps
        // above `CEILING - FLOOR`.
        let offset = (*word & MAGNITUDE_MASK).wrapping_sub(INERT_MAGNITUDE_FLOOR);
        inert &= (*word == 0) | (offset <= ELISION_MAGNITUDE_CEILING - INERT_MAGNITUDE_FLOOR);
    }
    inert
}

/// `true` when both integrator words of `section` are inert on every lane: an identity section
/// holding them leaves them exactly where they are and passes its input through (issue #979; the
/// argument is on [`cascade_sections`]).
fn section_state_is_inert<L: Lane>(section: &Section<L>) -> bool {
    lane_is_inert::<L>(section.state.ic1) && lane_is_inert::<L>(section.state.ic2)
}

/// `true` when no word of `io` is `-0.0` and every word is finite and inside [`BLOCK_LIMIT`].
///
/// One branchless integer scan of the block that carries two witnesses and decides once, at the
/// end (issue #980). Per word it does one `xor`, one unsigned `min`, one `and` and one unsigned
/// `max` -- four integer vector operations per vector of words, which LLVM emits as
/// `vpminud`/`vpmaxud` on x86 and `i32x4.min_u`/`i32x4.max_u` in `simd128` -- and nothing leaves
/// the integer domain until the block is finished. There is no float compare anywhere: a float
/// compare would call `-0.0` equal to `+0.0` and order a NaN away.
///
/// * `nearest` starts at `u32::MAX` and keeps the minimum of `bits ^ NEGATIVE_ZERO_BITS`. For any
///   `x` and `c`, `x ^ c == 0` exactly when `x == c`, so a word contributes `0` exactly when it is
///   the `-0.0` bit pattern, and every other word contributes something non-zero. The minimum is
///   therefore `0` exactly when some word is `-0.0`. (`+0.0` contributes `0x8000_0000`, so it is
///   an ordinary sample here, as it must be.)
/// * `largest` starts at `0` and keeps the maximum of `bits & MAGNITUDE_MASK`, the magnitude bits.
///   They are monotone in magnitude, and every infinity and every NaN has magnitude bits at or
///   above `0x7f80_0000`, above [`ELISION_MAGNITUDE_CEILING`] (see that constant). So
///   `largest <= ELISION_MAGNITUDE_CEILING` is exactly "every word is finite and
///   `|x| <= BLOCK_LIMIT`".
///
/// The block is admitted iff `nearest != 0 && largest <= ELISION_MAGNITUDE_CEILING`: the same
/// predicate, word for word, as the rejection accumulator it replaced (two compares and two `or`s
/// per word, seven vector operations per vector), and an empty block is admitted by both.
///
/// It is deliberately **not** chunked and short-circuiting the way
/// [`block_is_positive_zero`] is. That predicate's common answer is "no" on the first chunk; this
/// one's common answer is "yes", which is only knowable from the whole block.
#[inline]
#[must_use]
fn block_admits_elision(io: &[f32]) -> bool {
    let mut nearest = u32::MAX;
    let mut largest = 0_u32;
    for value in io {
        let bits = value.to_bits();
        nearest = nearest.min(bits ^ NEGATIVE_ZERO_BITS);
        largest = largest.max(bits & MAGNITUDE_MASK);
    }
    nearest != 0 && largest <= ELISION_MAGNITUDE_CEILING
}

/// The elision gate in the rejection-accumulator form it had before issue #980: the oracle the
/// min/max form is proven equal to.
#[cfg(test)]
fn block_admits_elision_oracle(io: &[f32]) -> bool {
    let mut rejected = 0_u32;
    for value in io {
        let bits = value.to_bits();
        rejected |= u32::from(bits == NEGATIVE_ZERO_BITS);
        rejected |= u32::from((bits & MAGNITUDE_MASK) > ELISION_MAGNITUDE_CEILING);
    }
    rejected == 0
}

/// One channel — left or right — of a cascade over `W = L::WIDTH` tracks.
///
/// This is the only realization in the crate: the scalar effect is the `L = f32`, `W = 1`
/// instantiation of the same body, because a planar block is already a one-lane AoSoA block. There
/// is no second copy of the recurrence, of the ramp law, or of the reset rules to keep in step.
struct Channel<L: Lane, const W: usize> {
    sections: [Section<L>; EQ_SECTION_COUNT],
    /// Samples still to be produced before each lane's ramp is at its target.
    remaining: [[u32; W]; EQ_SECTION_COUNT],
    /// Live parameter targets, per track.
    targets: [[BandTarget; EQ_SECTION_COUNT]; W],
    /// Per section: this section's coefficient words are [`EqSvfWords::IDENTITY`] to the bit on
    /// **every** lane of this channel.
    ///
    /// Maintained only where coefficients change -- [`settle`](Self::settle),
    /// [`start_ramp`](Self::start_ramp), [`snap`](Self::snap),
    /// [`restore_track`](Self::restore_track) -- so a rendered block pays nothing to keep it. A
    /// ramp moves `coef` per segment through the ramp kernel without passing through any of those,
    /// which would leave the flag stale; it cannot be read while that is true, because a lane with
    /// a ramp in flight makes the whole bank non-stationary and the elision this feeds is on the
    /// stationary path only. Every ramp ends in [`snap`](Self::snap), which refreshes it.
    ///
    /// [`identity_flags_agree`](Self::identity_flags_agree) re-derives the whole array from the
    /// words and is asserted in debug builds on every stationary block, so a coefficient-change
    /// site added later without a refresh is a test failure rather than a silent wrong render.
    identity: [bool; EQ_SECTION_COUNT],
}

impl<L: Lane, const W: usize> Channel<L, W> {
    /// Builds a settled channel from per-track band parameters.
    fn new(
        targets: [[BandTarget; EQ_SECTION_COUNT]; W],
        sample_rate: SampleRateHz,
    ) -> Result<Self, EqDesignError> {
        let words: [[Result<EqSvfWords, EqDesignError>; EQ_SECTION_COUNT]; W] =
            core::array::from_fn(|track| {
                core::array::from_fn(|section| targets[track][section].words(sample_rate))
            });
        let mut prepared = [[EqSvfWords::IDENTITY; EQ_SECTION_COUNT]; W];
        for track in 0..W {
            for section in 0..EQ_SECTION_COUNT {
                prepared[track][section] = words[track][section]?;
            }
        }
        Ok(Self::from_prepared(targets, prepared))
    }

    /// Builds a settled channel from already-designed words.
    ///
    /// The cache is prepared off render and is reused by full resets, so this constructor has no
    /// semantic-to-coefficient design path of its own.
    fn from_prepared(
        targets: [[BandTarget; EQ_SECTION_COUNT]; W],
        words: [[EqSvfWords; EQ_SECTION_COUNT]; W],
    ) -> Self {
        let identity = SvfCoef {
            c1: L::zero(),
            a2: L::zero(),
            a3: L::zero(),
            m0: L::splat(1.0),
            m1: L::zero(),
            m2: L::zero(),
        };
        let mut channel = Self {
            sections: [Section {
                coef: identity,
                step: SvfCoefStep::default(),
                target: identity,
                state: SvfState::default(),
            }; EQ_SECTION_COUNT],
            remaining: [[0; W]; EQ_SECTION_COUNT],
            targets,
            // Every `(section, track)` pair is settled below, and `settle` refreshes the flag.
            identity: [false; EQ_SECTION_COUNT],
        };
        for (track, sections) in words.iter().enumerate() {
            for (section, words) in sections.iter().enumerate() {
                channel.settle(section, track, *words);
            }
        }
        channel
    }

    /// Places lane `track` of `section` at `words` with no ramp in flight.
    fn settle(&mut self, section: usize, track: usize, words: EqSvfWords) {
        let slot = &mut self.sections[section];
        for (index, word) in words.to_array().into_iter().enumerate() {
            lane_set(coef_word_mut(&mut slot.coef, index), track, word);
            lane_set(coef_word_mut(&mut slot.target, index), track, word);
            lane_set(step_word_mut(&mut slot.step, index), track, 0.0);
        }
        self.remaining[section][track] = 0;
        self.refresh_identity(section);
    }

    /// Re-derives `identity[section]` from the section's coefficient words.
    ///
    /// Control plane only: it runs where a coefficient changes, never per block and never per
    /// frame. Six lane compares, and it reads the words rather than tracking which lane was
    /// written, so it cannot drift out of step with a partially updated section.
    fn refresh_identity(&mut self, section: usize) {
        let identity = {
            let coef = &self.sections[section].coef;
            (0..6)
                .all(|index| lane_bits_all::<L>(coef_word(coef, index), IDENTITY_WORD_BITS[index]))
        };
        self.identity[section] = identity;
    }

    /// `true` when every `identity` flag equals what the coefficient words say right now.
    ///
    /// Asserted in debug builds on every stationary block. It is the standing check that the list
    /// of coefficient-change sites is complete.
    fn identity_flags_agree(&self) -> bool {
        (0..EQ_SECTION_COUNT).all(|section| {
            let coef = &self.sections[section].coef;
            let observed = (0..6)
                .all(|index| lane_bits_all::<L>(coef_word(coef, index), IDENTITY_WORD_BITS[index]));
            observed == self.identity[section]
        })
    }

    /// Builds the per-lane dry-output mask for a dedicated cut at the current segment boundary.
    ///
    /// The decision is deliberately made from the coefficient *bits*, not `Lane::eq`: ordered
    /// floating equality treats `-0.0` as equal to `+0.0`, while the identity contract is exact.
    /// The fixed 0/1 decision vector is loaded and compared only to manufacture the backend's
    /// canonical mask representation. Original sections never select dry, even when disabled.
    #[inline(always)]
    fn dry_mask(&self, section: usize) -> L::Mask {
        let dedicated = section == HPF_SECTION || section == LPF_SECTION;
        let mut decisions = [0.0_f32; MAX_LANES];
        if dedicated {
            let slot = &self.sections[section];
            for (track, decision) in decisions.iter_mut().enumerate().take(W) {
                let exact_identity = (0..EQ_COEFFICIENT_WORDS).all(|index| {
                    lane_bits(coef_word(&slot.coef, index), track) == IDENTITY_WORD_BITS[index]
                });
                if exact_identity && self.remaining[section][track] == 0 {
                    *decision = 1.0;
                }
            }
        }
        L::load(&decisions[..L::WIDTH]).eq(L::splat(1.0))
    }

    /// Starts a [`RAMP_SAMPLES`]-sample word ramp on lane `track` of `section` (D11).
    ///
    /// The increment is one multiply by an exact power of two, computed once here; a ramp that is
    /// re-targeted mid-flight starts from the words in force now, not from the old target.
    ///
    /// # The stationary hoist (issue #144 item 6)
    ///
    /// When all six words this lane is being sent to are already the six words it holds, every
    /// increment is `(word - word) * RAMP_SCALE`, which is exactly `+0.0` for finite words. The
    /// ramp would then spend [`RAMP_SAMPLES`] samples adding zero to a lane that is already where
    /// it is being sent. That costs far more than the lane: `process_section` takes its ramping
    /// decision across **all** `W` lanes of the section, so one lane's no-op window drags the
    /// whole bank onto `svf_block_ramped` -- six vector additions and a negate per frame -- for
    /// sixty-four samples. A console that re-sends a band it did not move (an automation refresh,
    /// a touched-but-unmoved control) pays that on every refresh.
    ///
    /// [`LinearRamp::stationary_at`] decides it by bit compare. The lane is settled instead, which
    /// is bit-identical because a zero increment is bit-preserving on every word -- exactly the
    /// property `design_svf` normalises `-0.0` away to guarantee, and the same property that
    /// makes an idle lane of a ramping bank identical to the same lane of a settled one.
    /// `#[inline(never)]`: this is coefficient *design*, not render arithmetic. It runs only when
    /// an admitted automation span retargets a band, it is per lane and scalar by nature (six
    /// words, one subtract and one multiply each), and the shipped wasm artifact is gated on the
    /// EQ's render kernel carrying no scalar arithmetic budget it does not need
    /// (`check-web-audioworklet.sh`, `KERNEL_ROSTER`). Before #163 phase 3 the inliner kept it out
    /// of `process_bank` on its own; phase 3 made that function small enough that it stopped, so
    /// the shape is pinned here rather than left to a heuristic.
    #[inline(never)]
    fn start_ramp(&mut self, section: usize, track: usize, words: EqSvfWords) {
        if self.stationary_at(section, track, words) {
            self.settle(section, track, words);
            return;
        }
        let slot = &mut self.sections[section];
        for (index, word) in words.to_array().into_iter().enumerate() {
            let current = lane_get(coef_word(&slot.coef, index), track);
            lane_set(coef_word_mut(&mut slot.target, index), track, word);
            lane_set(
                step_word_mut(&mut slot.step, index),
                track,
                (word - current) * RAMP_SCALE,
            );
        }
        self.remaining[section][track] = RAMP_SAMPLES;
        // `coef` did not move here, so this cannot change the flag. It is refreshed anyway because
        // "every coefficient-change site refreshes" is a rule that is cheap to keep total and
        // expensive to keep partial.
        self.refresh_identity(section);
    }

    /// Applies a validated prepared target to one lane at a block boundary.
    ///
    /// All decoding and validation is completed by the caller before this method mutates the
    /// semantic target or coefficient ramp. The target words are supplied by the off-render
    /// preparer and are copied directly into the existing ramp state.
    fn apply_prepared_target(
        &mut self,
        track: usize,
        section: usize,
        target: BandTarget,
        words: EqSvfWords,
    ) {
        self.targets[track][section] = target;
        self.start_ramp(section, track, words);
    }

    /// The words lane `track` of `section` is heading for, read back out of the lane words.
    ///
    /// Issue #144 item 6, the re-preparation half. `BandTarget::words` is a pure function of the
    /// band and the sample rate, and `Section::target` holds exactly what it returned the last
    /// time this lane's band changed. So when an automation point restates a band it has already
    /// been given, the design does not have to be recomputed -- it can be read. That matters far
    /// more than the ramp arithmetic: the design is an `f64` `design_svf` per lane per event,
    /// and a console that refreshes its automation is paying it on every refresh for every band it
    /// did not move.
    ///
    /// This is bit-identical rather than approximately equal, by determinism: same band, same
    /// rate, same function, same words.
    fn target_words(&self, section: usize, track: usize) -> EqSvfWords {
        let slot = &self.sections[section];
        EqSvfWords::from_array(core::array::from_fn(|index| {
            lane_get(coef_word(&slot.target, index), track)
        }))
    }

    /// `true` when lane `track` of `section` already holds exactly `words`.
    ///
    /// All six words must agree bitwise, and each must pass [`LinearRamp::stationary_at`], which
    /// is where the `-0.0` and non-finite exclusions live. A partial match is not a hoist: five
    /// settled words and one moving word is a ramp.
    fn stationary_at(&self, section: usize, track: usize, words: EqSvfWords) -> bool {
        let slot = &self.sections[section];
        words
            .to_array()
            .into_iter()
            .enumerate()
            .all(|(index, word)| {
                LinearRamp::stationary_at(lane_get(coef_word(&slot.coef, index), track), word)
            })
    }

    /// Assigns the target exactly and stops lane `track`'s ramp — the D11 snap.
    ///
    /// **The caller refreshes** `identity[section]`. Every call site is a loop over the lanes of
    /// one section, and [`refresh_identity`](Self::refresh_identity) re-derives the whole section
    /// from its coefficient words, so running it inside this function re-derived the same six lane
    /// compares `W` times for one bank-wide ramp end. Hoisting it is exact rather than merely
    /// cheaper: the flag is read only on a *stationary* block ([`cascade_sections`]), no lane's
    /// ramp can start mid-block, and a section's last snap is the one whose value survives either
    /// way. `identity_flags_agree` is the standing oracle that no refresh site was lost.
    fn snap(&mut self, section: usize, track: usize) {
        let slot = &mut self.sections[section];
        for index in 0..6 {
            let target = lane_get(coef_word(&slot.target, index), track);
            lane_set(coef_word_mut(&mut slot.coef, index), track, target);
            lane_set(step_word_mut(&mut slot.step, index), track, 0.0);
        }
        self.remaining[section][track] = 0;
    }

    /// Snaps **every** lane of `section` at once: six vector copies and one zeroed step set.
    ///
    /// Bit-identical to `for track in 0..W { self.snap(section, track) }`, and it is the shape a
    /// bank-wide ramp end actually has -- the console moves a band on all `W` lanes of a bank
    /// together. The per-lane form pays a `lane_get`/`lane_set` pair per word per lane, and each of
    /// those is a full `store`/`load` round trip out of and back into the vector domain: `12 * W`
    /// of them per section, to write words the target already holds in exactly the right lanes.
    ///
    /// The caller refreshes `identity[section]`, as with [`snap`](Self::snap).
    fn snap_section(&mut self, section: usize) {
        let slot = &mut self.sections[section];
        slot.coef = slot.target;
        slot.step = SvfCoefStep::default();
        self.remaining[section] = [0; W];
    }

    /// Runs the six physical sections over one block in place.
    ///
    /// `#[inline(always)]`, and [`process_section`](Self::process_section) with it, so that the
    /// whole render path of a bank stays inside one wasm function. `check-web-audioworklet.sh`
    /// asserts that each shipped effect has **exactly one** arithmetic-carrying kernel in the
    /// artifact -- that is how a de-vectorisation is caught. #163 phase 3 made `process_bank`
    /// small enough that the inliner began outlining this body into a second one, and a second
    /// kernel reads to that gate as a kernel that moved, not as the ramp fallback it is.
    #[inline(always)]
    fn process_block(&mut self, io: &mut [f32], frames: usize) {
        for section in 0..EQ_SECTION_COUNT {
            self.process_section(section, io, frames);
        }
    }

    /// Runs one section over one block, splitting it where a lane's ramp ends.
    ///
    /// The block is cut at every distinct ramp end, so within a segment every ramping lane steps on
    /// every frame and every settled lane has a zero increment. The cut is control-plane work done
    /// once per section per block; the frames themselves never branch.
    #[inline(always)]
    fn process_section(&mut self, section: usize, io: &mut [f32], frames: usize) {
        let mut position = 0;
        let mut snapped = false;
        while position < frames {
            let mut length = frames - position;
            let mut ramping = false;
            let mut was_ramping = [false; W];
            for (track, was_ramping) in was_ramping.iter_mut().enumerate() {
                let remaining = self.remaining[section][track];
                if remaining > 0 {
                    ramping = true;
                    *was_ramping = true;
                    length = length.min(remaining as usize);
                }
            }
            debug_assert!(length > 0);
            let dry_mask = self.dry_mask(section);
            let slot = &mut self.sections[section];
            let block = &mut io[position * W..(position + length) * W];
            if ramping {
                if section == HPF_SECTION || section == LPF_SECTION {
                    svf_block_ramped_with_dry_mask::<L>(
                        block,
                        length,
                        &mut slot.coef,
                        &slot.step,
                        length,
                        &mut slot.state,
                        dry_mask,
                    );
                } else {
                    svf_block_ramped::<L>(
                        block,
                        length,
                        &mut slot.coef,
                        &slot.step,
                        length,
                        &mut slot.state,
                    );
                }
            } else {
                if section == HPF_SECTION || section == LPF_SECTION {
                    svf_block_ramped_with_dry_mask::<L>(
                        block,
                        length,
                        &mut slot.coef,
                        &slot.step,
                        0,
                        &mut slot.state,
                        dry_mask,
                    );
                } else {
                    svf_block::<L>(block, length, &slot.coef, &mut slot.state);
                }
            }
            for track in 0..W {
                let remaining = &mut self.remaining[section][track];
                *remaining = remaining.saturating_sub(length as u32);
            }
            // The kernel processes current coefficients, then advances them. Snap after a segment
            // consumes the final sample so the exact target is first used by sample A+64.
            for (track, &was_ramping) in was_ramping.iter().enumerate() {
                if was_ramping && self.remaining[section][track] == 0 {
                    self.snap(section, track);
                    snapped = true;
                }
            }
            position += length;
        }
        // Once for the section, not once per snapped lane per segment. Nothing between the snaps
        // above and here reads `identity`, and the value this leaves is the value the last snap
        // would have left: see the note on `snap`.
        if snapped {
            self.refresh_identity(section);
        }
    }

    /// `true` when no lane of any section has a ramp in flight.
    ///
    /// A ramp means the coefficients move within the block, so the "same coefficients" leg of the
    /// fixed-point induction does not hold and the fast path must not engage.
    fn no_ramp_in_flight(&self) -> bool {
        self.remaining
            .iter()
            .all(|section| section.iter().all(|remaining| *remaining == 0))
    }

    /// The raw bit patterns of every integrator word, for an exact before/after comparison.
    ///
    /// Bits rather than floats because the comparison has to distinguish `+0.0` from `-0.0`: an
    /// integrator that settles at `-0.0` is still settled, but a float compare would also call a
    /// pair that moved between the two zeros "unchanged", which is exactly the bit the fast path
    /// promises not to move.
    fn state_bits(&self, out: &mut [u32; EQ_SECTION_COUNT * 2 * MAX_LANES]) {
        for (index, section) in self.sections.iter().enumerate() {
            let base = index * 2 * L::WIDTH;
            section
                .state
                .ic1
                .store_bits(&mut out[base..base + L::WIDTH]);
            section
                .state
                .ic2
                .store_bits(&mut out[base + L::WIDTH..base + 2 * L::WIDTH]);
        }
    }

    /// Clears every integrator word, leaving coefficients and ramps alone.
    fn reset_states(&mut self) {
        for section in &mut self.sections {
            section.state = SvfState::default();
        }
    }

    /// Ends every ramp at its target and clears the integrators (a seek or a transport stop).
    fn discontinuity_reset(&mut self) {
        self.reset_states();
        for section in 0..EQ_SECTION_COUNT {
            // Every lane, so this is the whole-section snap by construction; one refresh per
            // section rather than one per lane.
            self.snap_section(section);
            self.refresh_identity(section);
        }
    }
}

/// Runs both channels' six-section cascades over one block.
///
/// Issue #163 phase 3. `stationary` means no lane of either channel has a ramp in flight, so
/// every section would run [`svf_block`] with fixed coefficients over the whole block --
/// exactly `EQ_SECTION_COUNT * 2` serial passes, each one a recurrence whose next frame
/// cannot start until this one's integrators are written. That shape is latency-bound: it
/// leaves the vector units idle for most of every frame, and it is why the EQ took the same
/// wall time at `Simd4` as at `Simd8`.
///
/// The interleaved form runs the two channels in one frame loop, two sections deep, so several
/// independent recurrences are always in flight. It is the *same* operation order per chain --
/// see [`svf_cascade_interleaved`] -- so this is a schedule change, not a numeric one, and the
/// fixture pins are untouched.
///
/// A ramping block falls back to the per-section path, which owns the block-splitting rule
/// that a moving coefficient needs. Ramps run for at most a smoothing window after a
/// parameter change; a console rendering audio is stationary on essentially every block.
///
/// It runs only the sections [`ramping_sections`] lists (issue #1005): every section that is live
/// or ramping, and a dead one only where the elision gate refuses or an unsafe ramp precedes it.
/// Each listed section runs [`Channel::process_section`] on the left channel and then on the
/// right, exactly as [`Channel::process_block`] runs it; the two channels share no state, so
/// taking them section by section rather than channel by channel moves no bit.
///
/// Returns the §4.4 verdict per channel when the cascade's last pass judged it as it stored
/// (issue #999; see [`interleave`]), and `None` when the caller must scan the planes: a ramped
/// block, a refused or all-live plan, and a plan with no live section.
///
/// [`svf_cascade_interleaved`]: lane::kernels::svf_cascade_interleaved
#[inline(always)]
fn process_channels<L: Lane, const W: usize>(
    channels: (&mut Channel<L, W>, &mut Channel<L, W>),
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    stationary: bool,
) -> Option<[bool; 2]> {
    if !stationary {
        #[cfg(test)]
        if !ramping_list_enabled() {
            channels.0.process_block(left, frames);
            channels.1.process_block(right, frames);
            return None;
        }
        let (list, length) = ramping_sections::<L, W>(channels.0, channels.1, left, right, frames);
        #[cfg(any(test, feature = "test-support"))]
        count_ramping_plan(length);
        for &section in &list[..length] {
            channels.0.process_section(section, left, frames);
            channels.1.process_section(section, right, frames);
        }
        return None;
    }
    debug_assert!(channels.0.identity_flags_agree());
    debug_assert!(channels.1.identity_flags_agree());
    let sections = cascade_sections::<L, W>(channels.0, channels.1, left, right, frames);
    // The kept (live) sections run in passes of the effective stationary depth of two on every
    // backend, and an odd count ends in one depth-one pass: #976 removed the identity padding
    // section that used to make the count even.
    interleave::<L, W, EFFECTIVE_CASCADE_DEPTH>(channels, left, right, frames, sections)
}

/// [`process_channels`] over one plane: the collapsed track's live channel.
///
/// The one-channel forms below are the dual bodies with the second channel's argument deleted and
/// every remaining line left where it was. Two of the dual predicates read both channels and are
/// therefore restated rather than dropped, and both restatements are *equalities* on a
/// collapse-eligible bank rather than weakenings:
///
/// * `stationary` is `no_ramp_in_flight` on both channels. A collapse-eligible bank's two channels
///   carry the same `remaining` array -- a `ParameterChannel::Both` retarget writes both, and a
///   one-channel retarget clears the witness' `LIVE` term -- so the left channel's answer is the
///   conjunction's;
/// * `cascade_sections`' `dead` is `identity` on both channels, and the identity flag is derived
///   from the coefficient words the `DESIGNED` term compares bit for bit. Same array, same answer.
/// * [`ramping_sections`]' `ramping` and safe-ramp terms read `remaining`, `coef.m0` and `step.m0`
///   on both channels (issue #1005). A `Both` retarget writes the same words and the same
///   countdown into both, the ramp advances both by the same steps, and the dual run's right
///   channel is the left channel bit for bit -- which is what the collapse renders on. Same words,
///   same answer.
///
/// What is **not** restated is the `-0.0` gate on the input planes: it is evaluated on the one
/// plane the chain gathered, which is the only plane the collapsed cascade reads.
///
/// The folded §4.4 verdict is [`process_channels`]', one plane of it.
#[inline(always)]
fn process_channels_mono<L: Lane, const W: usize>(
    channel: &mut Channel<L, W>,
    io: &mut [f32],
    frames: usize,
    stationary: bool,
) -> Option<bool> {
    if !stationary {
        #[cfg(test)]
        if !ramping_list_enabled() {
            channel.process_block(io, frames);
            return None;
        }
        let (list, length) = ramping_sections_mono::<L, W>(channel, io, frames);
        #[cfg(any(test, feature = "test-support"))]
        count_ramping_plan(length);
        for &section in &list[..length] {
            channel.process_section(section, io, frames);
        }
        return None;
    }
    debug_assert!(channel.identity_flags_agree());
    let sections = cascade_sections_mono::<L, W>(channel, io, frames);
    interleave_mono::<L, W, EFFECTIVE_CASCADE_DEPTH>(channel, io, frames, sections)
}

/// [`cascade_sections`] over one channel. Every leg is [`cascade_sections`]'s, gated on the one
/// live channel; the proof it rests on is that function's, unchanged.
#[inline(always)]
fn cascade_sections_mono<L: Lane, const W: usize>(
    channel: &Channel<L, W>,
    io: &[f32],
    frames: usize,
) -> ([usize; EQ_SECTION_COUNT], usize) {
    let all = (core::array::from_fn(|section| section), EQ_SECTION_COUNT);
    let dead = |section: usize| channel.identity[section];
    let live = (0..EQ_SECTION_COUNT)
        .filter(|section| !dead(*section))
        .count();
    if live >= EQ_SECTION_COUNT {
        return all;
    }
    let words = frames * W;
    if !block_admits_elision(&io[..words]) {
        return all;
    }
    for section in 0..EQ_SECTION_COUNT {
        let admissible = if dead(section) {
            section_state_is_inert(&channel.sections[section])
        } else {
            section_state_is_flush_shaped(&channel.sections[section])
        };
        if !admissible {
            return all;
        }
    }
    let mut list = [0_usize; EQ_SECTION_COUNT];
    let mut length = 0;
    for section in 0..EQ_SECTION_COUNT {
        if !dead(section) {
            list[length] = section;
            length += 1;
        }
    }
    debug_assert_eq!(length, live);
    (list, length)
}

/// [`interleave`] over one stream. Same kernels (the skewed pair, issue #978), same list, same
/// tail rule, same select-free pairs on an admitted plan (issue #977), same §4.4 verdict folded
/// into the depth-one pass (issue #999), one channel of it.
///
/// This is a second instantiation of [`svf_cascade_interleaved`] -- at `CHANNELS = 1` where the
/// dual path uses `2` -- and [`cascade_sections`] warns that a second arithmetic-carrying EQ kernel
/// in the shipped wasm artifact reads to `KERNEL_ROSTER` as a kernel that moved. It was checked
/// rather than assumed: at mono-collapse M2 the artifact carries one EQ symbol, because both
/// cascades inline into `process_bank_inner`. The compressor and the limiter did grow a second
/// symbol each, and the roster names them. If a later change splits this one out, the fix is a
/// roster row, not a looser pattern. The depth-one tail (issue #976) is two more instantiations,
/// `<L, 1, 1>` with and without the dry select (their bounded twins since issue #999), and the
/// pairs run [`svf_cascade_skewed`] and its dry-mask twin (issue #978), all inlined the same way.
///
/// [`svf_cascade_interleaved`]: lane::kernels::svf_cascade_interleaved
#[inline(always)]
fn interleave_mono<L: Lane, const W: usize, const DEPTH: usize>(
    channel: &mut Channel<L, W>,
    io: &mut [f32],
    frames: usize,
    sections: ([usize; EQ_SECTION_COUNT], usize),
) -> Option<bool> {
    debug_assert_eq!(EQ_SECTION_COUNT % DEPTH, 0);
    let (list, length) = sections;
    debug_assert!(
        length % DEPTH <= 1,
        "the tail rule is one section at depth one"
    );
    let admitted = length < EQ_SECTION_COUNT;
    let pairs = length / DEPTH;
    let tail = length % DEPTH == 1;
    let mut within = None;
    for pass in 0..pairs {
        let base = pass * DEPTH;
        let at: [usize; DEPTH] = core::array::from_fn(|k| list[base + k]);
        let coefficients: [[SvfCoef<L>; DEPTH]; 1] =
            [core::array::from_fn(|k| channel.sections[at[k]].coef)];
        let mut state: [[SvfState<L>; DEPTH]; 1] =
            [core::array::from_fn(|k| channel.sections[at[k]].state)];
        if admitted {
            svf_cascade_skewed::<L, 1, DEPTH>([&mut *io], frames, &coefficients, &mut state);
        } else {
            #[cfg(any(test, feature = "test-support"))]
            count_masked_pair_pass();
            let dry_masks: [[L::Mask; DEPTH]; 1] =
                [core::array::from_fn(|k| channel.dry_mask(at[k]))];
            svf_cascade_skewed_with_dry_masks::<L, 1, DEPTH>(
                [&mut *io],
                frames,
                &coefficients,
                &mut state,
                &dry_masks,
            );
        }
        let [only] = state;
        for (k, word) in only.into_iter().enumerate() {
            channel.sections[at[k]].state = word;
        }
    }
    if tail {
        let at = list[length - 1];
        let coefficients: [[SvfCoef<L>; 1]; 1] = [[channel.sections[at].coef]];
        let mut state: [[SvfState<L>; 1]; 1] = [[channel.sections[at].state]];
        // #976's rule, unchanged by #977: see `interleave`.
        let dry_masks: [[L::Mask; 1]; 1] = [[channel.dry_mask(at)]];
        let [verdict] = if L::mask_any(dry_masks[0][0]) {
            svf_cascade_interleaved_with_dry_masks_bounded::<L, 1, 1>(
                [&mut *io],
                frames,
                &coefficients,
                &mut state,
                &dry_masks,
                BLOCK_LIMIT,
            )
        } else {
            #[cfg(any(test, feature = "test-support"))]
            count_select_free_tail_pass();
            svf_cascade_interleaved_bounded::<L, 1, 1>(
                [&mut *io],
                frames,
                &coefficients,
                &mut state,
                BLOCK_LIMIT,
            )
        };
        channel.sections[at].state = state[0][0];
        within = Some(verdict);
    }
    within
}

/// The cascade positions a **ramping** block will run, in cascade order (issue #1005).
///
/// All six, unless the stationary elision gate admits the block. Then a section is dropped when it
/// is *dead* -- the exact identity on every lane of both channels **and** ramping on no lane of
/// either -- and no *unsafe* ramping section precedes it. A ramping section is safe when its `m0`
/// word is exactly `1.0` with a `+0.0` increment on every lane of both channels
/// ([`ramp_keeps_unit_m0`]); any other ramping section keeps every dead section after it. The
/// argument is the ramping paragraph of [`cascade_sections`]' proof.
///
/// * **The gate is [`cascade_sections`]', leg for leg**: (a) neither input plane carries `-0.0`, a
///   non-finite word or a magnitude above the ceiling; (b) every dead section's state is inert on
///   both channels; (c) every other section's state is a word `flush` can leave -- `+0.0`, or
///   finite with a magnitude of at least `FLUSH_EPS` ([`section_state_is_flush_shaped`]). Any
///   refusal returns all six, and the block renders as it did before this function existed.
/// * **Freshness.** `identity[s]` is only read for a section with no lane in flight on either
///   channel, and that flag is fresh: every ramp ends in a snap followed by `refresh_identity`, and
///   `settle`, `start_ramp`, the resets and a restore refresh too. A ramping section's flag can be
///   stale -- its words move through the ramp kernel -- and the `ramping` term is what stops it
///   being read.
/// * **Conservative in time.** `remaining` is read at block start, so a lane that snaps mid-block
///   counts as ramping for the whole block, and no ramp can start mid-block.
#[inline(always)]
fn ramping_sections<L: Lane, const W: usize>(
    left_channel: &Channel<L, W>,
    right_channel: &Channel<L, W>,
    left: &[f32],
    right: &[f32],
    frames: usize,
) -> ([usize; EQ_SECTION_COUNT], usize) {
    let all = (core::array::from_fn(|section| section), EQ_SECTION_COUNT);
    let ramping = |section: usize| {
        section_is_ramping(&left_channel.remaining[section])
            || section_is_ramping(&right_channel.remaining[section])
    };
    let dead = |section: usize| {
        !ramping(section) && left_channel.identity[section] && right_channel.identity[section]
    };
    if !(0..EQ_SECTION_COUNT).any(dead) {
        return all;
    }
    // (a), as in `cascade_sections`.
    let words = frames * W;
    if !block_admits_elision(&left[..words]) || !block_admits_elision(&right[..words]) {
        return all;
    }
    for section in 0..EQ_SECTION_COUNT {
        let admissible = if dead(section) {
            // (b)
            section_state_is_inert(&left_channel.sections[section])
                && section_state_is_inert(&right_channel.sections[section])
        } else {
            // (c)
            section_state_is_flush_shaped(&left_channel.sections[section])
                && section_state_is_flush_shaped(&right_channel.sections[section])
        };
        if !admissible {
            return all;
        }
    }
    let mut list = [0_usize; EQ_SECTION_COUNT];
    let mut length = 0;
    let mut unsafe_before = false;
    for section in 0..EQ_SECTION_COUNT {
        if dead(section) && !unsafe_before {
            continue;
        }
        if ramping(section)
            && !(ramp_keeps_unit_m0(&left_channel.sections[section])
                && ramp_keeps_unit_m0(&right_channel.sections[section]))
        {
            unsafe_before = true;
        }
        list[length] = section;
        length += 1;
    }
    (list, length)
}

/// [`ramping_sections`] over one channel: the collapsed body's list. Every term is
/// [`ramping_sections`]', with the right channel's dropped as [`cascade_sections_mono`] drops it;
/// [`process_channels_mono`] says why each restatement is an equality on a collapse-eligible bank.
#[inline(always)]
fn ramping_sections_mono<L: Lane, const W: usize>(
    channel: &Channel<L, W>,
    io: &[f32],
    frames: usize,
) -> ([usize; EQ_SECTION_COUNT], usize) {
    let all = (core::array::from_fn(|section| section), EQ_SECTION_COUNT);
    let ramping = |section: usize| section_is_ramping(&channel.remaining[section]);
    let dead = |section: usize| !ramping(section) && channel.identity[section];
    if !(0..EQ_SECTION_COUNT).any(dead) {
        return all;
    }
    let words = frames * W;
    if !block_admits_elision(&io[..words]) {
        return all;
    }
    for section in 0..EQ_SECTION_COUNT {
        let admissible = if dead(section) {
            section_state_is_inert(&channel.sections[section])
        } else {
            section_state_is_flush_shaped(&channel.sections[section])
        };
        if !admissible {
            return all;
        }
    }
    let mut list = [0_usize; EQ_SECTION_COUNT];
    let mut length = 0;
    let mut unsafe_before = false;
    for section in 0..EQ_SECTION_COUNT {
        if dead(section) && !unsafe_before {
            continue;
        }
        if ramping(section) && !ramp_keeps_unit_m0(&channel.sections[section]) {
            unsafe_before = true;
        }
        list[length] = section;
        length += 1;
    }
    (list, length)
}

/// `true` when some lane of a section has a ramp in flight.
#[inline(always)]
fn section_is_ramping<const W: usize>(remaining: &[u32; W]) -> bool {
    remaining.iter().any(|samples| *samples != 0)
}

/// `true` when `section`'s `m0` word is exactly `1.0` with a `+0.0` increment on every lane: the
/// section multiplies its input by exactly one on every frame of a ramp, whatever its other words
/// do, which is the case of [`cascade_sections`]' proof that needs no designed word.
///
/// Both words are read by bit pattern: a `-0.0` increment would keep `m0` at `1.0` too, but the
/// claim is kept to the one pattern the ramp law writes for a word that does not move.
#[inline(always)]
fn ramp_keeps_unit_m0<L: Lane>(section: &Section<L>) -> bool {
    lane_bits_all::<L>(section.coef.m0, 1.0_f32.to_bits()) && lane_bits_all::<L>(section.step.m0, 0)
}

/// `true` when every lane of `value` is a word `flush` can leave behind: exactly `+0.0`, or finite
/// with a magnitude of at least [`lane::FLUSH_EPS`]. No `-0.0`, no subnormal, no tiny normal.
#[inline(always)]
fn lane_is_flush_shaped<L: Lane>(value: L) -> bool {
    debug_assert!(L::WIDTH <= MAX_LANES);
    let mut words = [0_u32; MAX_LANES];
    value.store_bits(&mut words[..L::WIDTH]);
    words[..L::WIDTH].iter().all(|word| {
        *word == 0
            || (INERT_MAGNITUDE_FLOOR..NON_FINITE_MAGNITUDE).contains(&(*word & MAGNITUDE_MASK))
    })
}

/// Leg (c) of [`cascade_sections`] and [`ramping_sections`]: both integrator words of a kept
/// section are words the kernel can have written ([`lane_is_flush_shaped`]) on every lane.
///
/// That is finite and free of `-0.0`, which is what the leg asked before issue #1015, and also
/// free of the tiny words only a restore can put there. The `-0.0` induction on
/// [`cascade_sections`] argues its high-shelf case from kernel-written states (`v1 = ic1 + d1` is
/// `+0.0` or at least `FLUSH_EPS * 2^-24`), and a restored payload was the one way around that: a
/// live high shelf whose `m0` and `m2` are both exactly `0.5` (gain `-6.0206` dB as an `f32`, the
/// one gain that designs them), holding `ic1 = ic2 = -2^-149`, turns an input of `-2^-149` into
/// `y = (-0.0) + ((-0.0) + (-0.0)) = -0.0` on its first frame, and an elided identity section after
/// it passed that `-0.0` on where the executed one writes `+0.0`. Refusing costs one block: the
/// refused block runs every section, the kernel flushes the words, and the next block engages.
/// Kernel-written states always pass, so a session that restores nothing tiny elides exactly as it
/// did.
#[inline(always)]
fn section_state_is_flush_shaped<L: Lane>(section: &Section<L>) -> bool {
    lane_is_flush_shaped::<L>(section.state.ic1) && lane_is_flush_shaped::<L>(section.state.ic2)
}

/// The cascade positions this stationary block will actually run, in cascade order.
///
/// All six, unless identity-section elision is admissible — in which case the sections that are
/// the exact identity on every lane of *both* channels are dropped from the list, and the cascade
/// runs shorter.
///
/// # Why a section can be dropped at all
///
/// `BandTarget::words` maps `enabled = false` to [`EqSvfWords::IDENTITY`] at every other
/// parameter, so a disabled band is not "nearly" a pass-through, it is `c1 = a2 = a3 = m1 = m2 =
/// +0.0, m0 = 1.0`. A console that ships six sections and uses two spends most of the cascade
/// there.
/// The list is exactly the live sections, in cascade order. [`interleave`] runs both channels
/// through [`svf_cascade_interleaved`] two sections per pass and an odd last section alone at depth
/// one, so every dropped section is one section's arithmetic that no longer runs.
///
/// The flag is per section **across both channels** because that is the granularity the kernel
/// has: one pass carries section `k` of the left channel and section `k` of the right, and a
/// section that is identity on the left but live on the right must still run.
///
/// # The proof obligation
///
/// The claim is exact bit identity of the rendered audio **and** of every section's integrator
/// words, elided sections included — not approximate agreement. Take an identity section whose
/// integrator words are each *inert* -- exactly `+0.0`, or finite with a magnitude in
/// `[FLUSH_EPS, BLOCK_LIMIT]` (issue #979) -- and one finite input word `v0`.
///
/// * `v3 = v0 - ic2`. For `ic2 = +0.0` that is `v0` bit for bit, including `v0 = -0.0` (IEEE-754
///   gives `(-0) - (+0) = -0` under round-to-nearest). Otherwise `|ic2| <= 1e30`, below `2^103`,
///   half an ulp of `f32::MAX`, so the exact difference is under `f32::MAX + 2^103` in magnitude
///   and rounds to a finite `v3` for every finite `v0`, whatever section feeds it.
/// * `d1 = nc1 * ic1 + a2 * v3`. `nc1 = -c1 = -0.0` and `a2 = +0.0`, and `ic1` and `v3` are
///   finite, so both products are zeros of some sign and `d1` is a zero. `v1 = ic1 + d1` is `ic1`
///   for a non-zero `ic1`, and `+0.0` for `ic1 = +0.0` (`(+0.0) + (±0.0) = +0.0`).
/// * `d2 = a3 * v3 + a2 * ic1` is a zero the same way, and `v2 = ic2 + d2` is `ic2` or `+0.0`.
/// * `ic1' = flush(ic1 + (d1 + d1)) = flush(ic1) = ic1`: a zero addend leaves a non-zero `ic1`
///   exactly, `flush` keeps every magnitude of at least `FLUSH_EPS`, and it maps a zero to `+0.0`.
///   Likewise `ic2' = ic2`. **The state does not move, by induction over the block.** That is what
///   happens to every identity section, whatever it held when it became the identity -- a
///   dedicated cut switched on and off again through prepared targets freezes a non-zero state
///   exactly this way (#807) -- provided the state was inert to begin with.
/// * `y = m2 * v2 + (m1 * v1 + m0 * v0) = (+0.0 * v2) + ((+0.0 * v1) + v0)`. `v1` and `v2` are
///   finite, so both products are zeros; `m0 = 1.0`, so `m0 * v0 = v0` exactly; and a zero plus
///   `v0` is `v0` for every `v0` **except** `v0 = -0.0`, where `(±0.0) + (-0.0)` can be `+0.0`.
///
/// So an identity section at an inert state is the exact identity on a finite input, with exactly
/// one exception: it rewrites `-0.0` to `+0.0`. Eliding it is therefore bit-exact **iff no `-0.0`
/// ever reaches it**, and the state it keeps is unchanged either way — which is what gate (b) below
/// asserts it already holds.
///
/// Each bound of "inert" is load-bearing. A magnitude below `FLUSH_EPS` (a restored subnormal, say)
/// is flushed to `+0.0` by the executed section and kept by the elided one, and a `-0.0` is flushed
/// to `+0.0` the same way. A magnitude above [`BLOCK_LIMIT`] can overflow `v3`: a disabled band
/// restored with `ic2 = -f32::MAX` behind a live +24 dB bell that turns an admitted `9e29` into
/// `1.4e31` computes `v3 = inf`, then `d1 = 0.0 * inf = NaN`, and the executed section writes `NaN`
/// where the elided one passes `v0` on (VERIFY-EQ, finding 1). `flush` keeps every state the kernel
/// writes at `+0.0` or at least `FLUSH_EPS` in magnitude, so the floor refuses only restored
/// payloads (admitted on finiteness alone); the cap refuses those too, and the rare huge state a
/// section can be left with when it is switched off after a spike.
///
/// The argument is per dead section and never counted how many were dropped, so it covers dropping
/// every one of them. Until issue #976 the list was padded back up to a whole number of depth-two
/// passes with dead sections, which the proof licensed to drop anyway; the padding only kept one
/// kernel instantiation. Pairing is not arithmetic: each `(stream, section)` chain runs the same
/// operations in the same order whichever pass carries it, and a value handed between passes goes
/// through an exact `f32` store and load.
///
/// That leaves: can a `-0.0` reach an elided section? Its input is either the block input or a
/// live section's output, so both have to be closed.
///
/// * **The block input.** Gate (a) refuses any block containing the `-0.0` bit pattern.
/// * **A live section's output.** `y = m2 * v2 + (m1 * v1 + m0 * v0)`, two `f32` additions.
///   An `f32` addition yields `-0.0` **only** when both addends are `-0.0`: a nonzero exact sum in
///   the subnormal range is representable, so addition never underflows to a zero of either sign,
///   and exact cancellation `a + (-a)` gives `+0.0` under round-to-nearest. So `y = -0.0` requires
///   all three of `m2 * v2`, `m1 * v1` and `m0 * v0` to be `-0.0`. `design_svf` normalises
///   `-0.0` out of every designed word, and the `m0` column of `design_svf_words_f64` is `+0.0`
///   for a low-pass and strictly positive for every other kind, so `m0 >= +0.0` always. Two cases:
///   - `m0 > 0`. Then `m0 * v0 = -0.0` needs either `v0 = -0.0` — excluded by induction, this
///     section's input carries no `-0.0` — or a multiplication that underflows to `-0.0`, which
///     needs `|m0 * v0| <= 2^-150`. For `m0 = 1.0` (high-pass, notch, bell, low shelf) that forces
///     `v0 = ±0.0` and contradicts. The one remaining `m0` is the high shelf's `A^2`, with
///     `A = 10^(gain/40)` and `gain` domain-limited to `[-24, 24]`, so `A^2 >= 0.063`; underflow
///     then needs `|v0| <= 2^-150 / A^2` **and** `A^2 <= 0.5`, hence `m2 = 1 - A^2 >= 0.5`, hence
///     `m2 * v2 = -0.0` needs `v2 = -2^-149` exactly. `v2 = ic2 + d2` and `v1 = ic1 + d1`; every
///     `a2` this design produces is `g / (1 + g * (g + k)) <= 1 / (2 + k) < 0.5` because `k > 0`
///     on every kind, so `a2 * v3` at `|v3| = 2^-149` underflows to a zero, `d1` is a zero, and
///     `v1 = ic1 + d1` is either `+0.0` (giving `m1 * v1 = +0.0`, since a cut high shelf has
///     `m1 = shelf_k * (1 - A) * A > 0`) or has `|v1| >= FLUSH_EPS * 2^-24`, far too large to
///     underflow. Either way `m1 * v1 != -0.0` and the conjunction fails. That last step needs
///     `ic1` to be `+0.0` or at least `FLUSH_EPS` in magnitude: every word the kernel writes is,
///     and gate (c) refuses a restored one that is not (issue #1015; a restored `-2^-149` there is
///     a counterexample, see [`section_state_is_flush_shaped`]).
///   - `m0 = +0.0`, i.e. a low pass, whose other words are `m1 = +0.0` and `m2 = 1.0`. Then
///     `m2 * v2 = v2`, so `y = -0.0` needs `v2 = -0.0`, which by the addition rule needs
///     `ic2 = -0.0`. `flush` maps every zero to `+0.0`, so no integrator word the kernel writes is
///     ever `-0.0`; the only way in is a restored state payload, and gate (c) refuses that.
///
///   So no live section emits `-0.0` when its own input carries none, and the induction closes.
///
/// **Finiteness is part of the claim, not an aside.** `0.0 * inf` is `NaN`, so an identity section
/// handed an infinity writes `NaN` where elision would pass the infinity through. Gate (a)'s
/// magnitude ceiling refuses non-finite input, and by refusing anything above [`BLOCK_LIMIT`] —
/// `1e30`, about `3.4e8` below `f32::MAX`, against a six-section cascade whose per-section output
/// mix is bounded by `|m0| + |m1| + |m2| < 2^6` — it also leaves the cascade unable to reach an
/// infinity from a block it admitted.
///
/// # A ramping block (issue #1005)
///
/// [`ramping_sections`] drops dead sections from a block on which some lane ramps, under the same
/// three legs. Everything above is per dead section,
/// and a dead section of a ramping block is an identity section with no lane in flight on either
/// channel, so it applies unchanged once one more input is closed: a dead section may now sit
/// downstream of a **ramping** live section, whose words are interpolated, not designed, and the
/// `-0.0` induction above was argued for designed words.
///
/// * **A safe ramping section.** Its `m0` word is exactly `1.0` with a `+0.0` increment on every
///   lane of both channels, so `m0` stays `1.0` on every frame the ramp runs (`1.0 + 0.0 = 1.0`),
///   and a snap that ends the ramp mid-block leaves the target's `m0`: `1.0` again, since the
///   increment was `(target - current) * 2^-6 = +0.0`. After a restore whose target disagrees,
///   the rest of the block runs the target's designed words at a kernel-written state, which is
///   the stationary case above. So on every ramping frame `m0 * v0 = v0` exactly, and
///   `y = m2 * v2 + (m1 * v1 + v0)` is `-0.0` only if `v0` is, whatever `m1` and `m2` are:
///   the addition rule above, with no designed word needed. This covers bell, low-shelf, notch
///   and high-pass rides, and every high-pass cut toggle (the identity and a high pass both have
///   `m0 = 1`).
/// * **Any other ramping section** -- a high shelf's gain, a low pass, a low-pass cut toggle, or a
///   section whose lanes mix families -- keeps every dead section after it in the list, where it
///   runs exactly as before, so nothing new has to be argued about its output. A dead section
///   ahead of it sees only the block input and the outputs of stationary live, safe ramping or
///   dead sections, which the induction covers.
/// * **Finiteness.** Interpolated words carry no gain bound of their own. It is not needed: an
///   inert identity section is the exact identity on *every* finite `v0` other than `-0.0` (see
///   above), so the elided and executed arms can differ on a lane only after a non-finite word
///   reaches a dead section. From there that lane's output is non-finite in both arms -- the
///   executed section writes `NaN` (`+0.0 * inf`), the elided one passes the word on, and no
///   later section turns a non-finite input into a finite output -- so the §4.4 check rejects the
///   channel in both, flags the same lanes, zeroes the plane and clears every integrator. Neither
///   arm touches a dead section's coefficients, so the two leave the block identical.
///
/// The kept sections run [`Channel::process_section`] as before, in cascade order, left then right
/// per section. The two channels share nothing, so interleaving them by section moves no bit.
///
/// # Why an admitted plan runs select-free (issue #977)
///
/// A dry lane is a lane of a dedicated cut that holds the exact identity words (`c1 = a2 = a3 =
/// m1 = m2 = +0.0`, `m0 = 1.0`) with no ramp in flight. The masked kernel returns the section input
/// `x` there and the wet output everywhere else. On an admitted plan that select is a no-op on
/// every lane, so [`interleave`] runs its depth-two passes without it (the depth-one tail keeps
/// #976's rule for a code-generation reason given there; the argument below covers it as well):
///
/// * The dry lane's input carries no `-0.0`. A dry lane exists only in a dedicated cut; the HPF is
///   physical section 0, so its input is the block input, which leg (a) clears of `-0.0` and bounds
///   by [`BLOCK_LIMIT`]; the LPF is the last section, so its input is a live or elided section's
///   output, which carries no `-0.0` by the induction above.
/// * Its state is finite. A section with a dry lane is live on some other lane, and leg (c) refuses
///   a live section with a non-finite integrator word on any lane.
/// * So, for the HPF, `v3 = x - ic2` is finite (`|x| <= 1e30`, below `2^103`, half an ulp of
///   `f32::MAX`, so no finite `ic2` can carry it past `f32::MAX`); `d1 = (-0.0 * ic1) + (+0.0 * v3)`
///   and `d2 = (+0.0 * v3) + (+0.0 * ic1)` are zeros of some sign; `v1 = ic1 + d1` and
///   `v2 = ic2 + d2` are finite; and the wet output `(+0.0 * v2) + ((+0.0 * v1) + 1.0 * x)` is `x`
///   bit for bit: a non-zero `x` absorbs the signed zeros, and `x = +0.0` gives `+0.0` whatever
///   their signs.
/// * The LPF's input is not bounded by leg (a). The same argument holds unless `x - ic2`
///   overflows, which needs `|x| >= 2^103`, or `x` is non-finite. Then the wet output is non-finite
///   where the select would return `x`, and `x` is itself non-finite or above [`BLOCK_LIMIT`]. The
///   LPF is the last section, so that word is the block output: both kernels fail the §4.4 check
///   on the same lane, and the plane is zeroed and its state reset the same way.
/// * The state update never reads the mask, so both kernels leave the same integrators.
///
/// Leg (c)'s finiteness term is load-bearing. The dry lane still runs the recurrence, and on a
/// *refused* block (which keeps the masks) a word of at least `2^103` against a restored
/// `ic2 = -f32::MAX` overflows `v3` and leaves the dry lane's state `NaN` while the select passes
/// the input on; a downstream LPF can keep that block inside the §4.4 bound, so nothing resets the
/// lane. A later admitted block would then turn the `NaN` into output (VERIFY-EQ, finding 2).
/// Refusing it keeps that bank on the masked kernel until a reset clears the state.
///
/// # Correctness never depends on the gate
///
/// Every leg is a refusal: any doubt returns all six sections and the block renders exactly as it
/// did before this function existed. The gate is a performance predicate, and the only thing a
/// wrong *engagement* rule could cost is speed.
///
/// [`svf_cascade_interleaved`]: lane::kernels::svf_cascade_interleaved
#[inline(always)]
fn cascade_sections<L: Lane, const W: usize>(
    left_channel: &Channel<L, W>,
    right_channel: &Channel<L, W>,
    left: &[f32],
    right: &[f32],
    frames: usize,
) -> ([usize; EQ_SECTION_COUNT], usize) {
    let all = (core::array::from_fn(|section| section), EQ_SECTION_COUNT);
    let dead = |section: usize| left_channel.identity[section] && right_channel.identity[section];
    let live = (0..EQ_SECTION_COUNT)
        .filter(|section| !dead(*section))
        .count();
    // The list is the live sections and nothing else: `interleave` runs an odd last one at depth
    // one rather than padding it with a dead section. The depth-one passes are further
    // instantiations of the interleaved kernel, `#[inline(always)]` into the one arithmetic-carrying
    // `process_bank` symbol per width that `KERNEL_ROSTER` counts, so they add no EQ symbol to the
    // wasm artifact. A cascade with nothing to drop never pays the scan below.
    if live >= EQ_SECTION_COUNT {
        return all;
    }
    // (a) Neither input plane carries `-0.0`, an infinity, a NaN, or a magnitude above the §4.4
    // bound. See the proof above; this is the leg that stops a `-0.0` entering the cascade.
    let words = frames * W;
    if !block_admits_elision(&left[..words]) || !block_admits_elision(&right[..words]) {
        return all;
    }
    for section in 0..EQ_SECTION_COUNT {
        let admissible = if dead(section) {
            // (b) An elided section's state must be inert (issue #979): `+0.0`, or finite with a
            // magnitude between `FLUSH_EPS` and the ceiling -- the states the proof's induction
            // starts from. Any other state would move in the full cascade and not here, and a
            // magnitude above the ceiling could overflow the executed section's `v3`.
            section_state_is_inert(&left_channel.sections[section])
                && section_state_is_inert(&right_channel.sections[section])
        } else {
            // (c) A live section's integrators must be words the kernel can write: `+0.0`, or
            // finite with a magnitude of at least `FLUSH_EPS` (issue #1015). Nothing the kernel
            // writes is ever `-0.0` or tiny, but a restored state payload is admitted on
            // finiteness alone: a low pass with `ic2 = -0.0` can emit `-0.0` into a later elided
            // section, and so can a high shelf at `m0 = m2 = 0.5` holding a restored subnormal
            // (`section_state_is_flush_shaped`). A non-finite word can be written: a dry lane
            // of a dedicated cut still runs the recurrence, and a refused block can overflow a
            // large restored integrator there to `NaN` while the select passes the dry input on
            // (issue #977). The admitted plan's pairs run select-free, so they need every dry
            // lane's state finite; see "Why an admitted plan runs select-free" above.
            section_state_is_flush_shaped(&left_channel.sections[section])
                && section_state_is_flush_shaped(&right_channel.sections[section])
        };
        if !admissible {
            return all;
        }
    }
    let mut list = [0_usize; EQ_SECTION_COUNT];
    let mut length = 0;
    for section in 0..EQ_SECTION_COUNT {
        if !dead(section) {
            list[length] = section;
            length += 1;
        }
    }
    debug_assert_eq!(length, live);
    (list, length)
}

/// One stationary block, both channels, `DEPTH` cascade sections fused per pass.
///
/// `sections` is the list [`cascade_sections`] chose and its length. Positions are read out of it
/// rather than counted from a base, so an elided cascade runs the same kernel over a shorter list —
/// the operation order per surviving chain is untouched, which is what keeps this a schedule
/// change.
///
/// The list runs as `length / DEPTH` whole passes and then, when the length is odd, one depth-one
/// pass over its last entry, which is the last live section in cascade order.
///
/// Each whole pass runs [`svf_cascade_skewed`] (or its dry-mask twin), the interleaved kernel
/// scheduled as a software pipeline: in iteration `i` section `k` runs frame `i - k`, so the two
/// sections of an iteration no longer wait on each other within the frame (issue #978). Every
/// `(stream, section)` chain runs the same operations on the same inputs in the same order as in
/// [`svf_cascade_interleaved`], so this is a schedule change and moves no bit. The depth-one tail
/// has nothing to pipeline and keeps [`svf_cascade_interleaved`], in its bounded twin (below).
///
/// # Select-free pairs on an admitted plan (issue #977)
///
/// A list shorter than [`EQ_SECTION_COUNT`] is exactly an *admitted* plan: [`cascade_sections`]
/// returns the full six both when it refuses and when every section is live, and a shorter list
/// only after all three of its legs passed. On an admitted plan every dry select is a no-op -- see
/// "Why an admitted plan runs select-free" on [`cascade_sections`] -- so every depth-two pass runs
/// [`svf_cascade_skewed`] and builds no [`Channel::dry_mask`]. A six-entry list keeps the
/// masked kernel: a refused block may carry `-0.0` or a non-finite word, and an all-live block was
/// never scanned for either, and there the select is not a no-op.
///
/// # The tail keeps #976's rule
///
/// The depth-one tail selects dry lanes when a lane of either channel is dry there, and runs
/// select-free otherwise, exactly as before #977, although the select is a no-op on an admitted
/// plan there too. That is a code-generation constraint, not an arithmetic one. The tail with no
/// dry lane is the standing console fixture's whole cascade (one live bell), and in the shipped
/// `simd128` artifact V8's register allocator keeps its four integrators in registers only when the
/// tail is written as it is here: with the tail made select-free on every admitted block -- one arm,
/// or the masked arm laid out first -- TurboFan kept one integrator in a stack slot across
/// iterations, and the one-band EQ rendered about 20 % slower through the render export (issue
/// #977, attempt 2). A tail with a dry lane (a dedicated cut that is the last live section and off
/// on some lanes) keeps the masked depth-one pass it had.
///
/// # The tail judges the §4.4 bound as it stores (issue #999)
///
/// When the list ends in the depth-one pass, that pass writes every output word of the block, so
/// it runs the bounded twin of its kernel ([`svf_cascade_interleaved_bounded`], or the dry-mask
/// one), which folds `|y| < BLOCK_LIMIT` per channel as it stores `y` -- `check_block`'s
/// conjunction over the same words -- and the verdict is returned for
/// [`PreparedParametricEq::render`] to use instead of re-reading both planes. The tail only runs
/// on an odd list, which is always admitted. Every other list returns `None` and `render` scans:
/// no live section (nothing is stored, and the block's own words meet the limit), a refused or
/// all-live list, and an even list, whose last pass is a pair. The pair keeps its kernel because
/// a bounded pair measured worse, not better: in the shipped `simd128` artifact V8 carried four
/// values of the bounded two-stream pair through stack slots across iterations, and natively the
/// per-node (`f32`) two-band row was about 5 % slower (#999, attempt 1). The bounded tail's fold
/// reduces each stored vector to a per-channel `bool` at once (attempt 2): folded into a vector mask
/// instead, the two accumulators went through stack slots across the loop's back edge in V8, which
/// the spill gate (issue #1000) refuses; as `bool`s they stay in general-purpose registers, and the
/// tail's integrators stay in vector registers (no carried stack slot).
///
/// [`svf_cascade_interleaved`]: lane::kernels::svf_cascade_interleaved
#[inline(always)]
fn interleave<L: Lane, const W: usize, const DEPTH: usize>(
    channels: (&mut Channel<L, W>, &mut Channel<L, W>),
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    sections: ([usize; EQ_SECTION_COUNT], usize),
) -> Option<[bool; 2]> {
    debug_assert_eq!(EQ_SECTION_COUNT % DEPTH, 0);
    let (list, length) = sections;
    debug_assert!(
        length % DEPTH <= 1,
        "the tail rule is one section at depth one"
    );
    let admitted = length < EQ_SECTION_COUNT;
    let pairs = length / DEPTH;
    let tail = length % DEPTH == 1;
    let mut within = None;
    for pass in 0..pairs {
        let base = pass * DEPTH;
        let at: [usize; DEPTH] = core::array::from_fn(|k| list[base + k]);
        let coefficients: [[SvfCoef<L>; DEPTH]; 2] = [
            core::array::from_fn(|k| channels.0.sections[at[k]].coef),
            core::array::from_fn(|k| channels.1.sections[at[k]].coef),
        ];
        let mut state: [[SvfState<L>; DEPTH]; 2] = [
            core::array::from_fn(|k| channels.0.sections[at[k]].state),
            core::array::from_fn(|k| channels.1.sections[at[k]].state),
        ];
        if admitted {
            svf_cascade_skewed::<L, 2, DEPTH>(
                [&mut *left, &mut *right],
                frames,
                &coefficients,
                &mut state,
            );
        } else {
            #[cfg(any(test, feature = "test-support"))]
            count_masked_pair_pass();
            let dry_masks: [[L::Mask; DEPTH]; 2] = [
                core::array::from_fn(|k| channels.0.dry_mask(at[k])),
                core::array::from_fn(|k| channels.1.dry_mask(at[k])),
            ];
            svf_cascade_skewed_with_dry_masks::<L, 2, DEPTH>(
                [&mut *left, &mut *right],
                frames,
                &coefficients,
                &mut state,
                &dry_masks,
            );
        }
        let [left_state, right_state] = state;
        for (k, word) in left_state.into_iter().enumerate() {
            channels.0.sections[at[k]].state = word;
        }
        for (k, word) in right_state.into_iter().enumerate() {
            channels.1.sections[at[k]].state = word;
        }
    }
    if tail {
        let at = list[length - 1];
        let coefficients: [[SvfCoef<L>; 1]; 2] = [
            [channels.0.sections[at].coef],
            [channels.1.sections[at].coef],
        ];
        let mut state: [[SvfState<L>; 1]; 2] = [
            [channels.0.sections[at].state],
            [channels.1.sections[at].state],
        ];
        // #976's rule, unchanged by #977: the tail selects dry lanes only when a lane of either
        // channel is dry there. See "The tail keeps #976's rule" above for why the admitted plan's
        // tail is not made select-free as well.
        let dry_masks: [[L::Mask; 1]; 2] = [[channels.0.dry_mask(at)], [channels.1.dry_mask(at)]];
        let verdict = if L::mask_any(dry_masks[0][0]) || L::mask_any(dry_masks[1][0]) {
            svf_cascade_interleaved_with_dry_masks_bounded::<L, 2, 1>(
                [&mut *left, &mut *right],
                frames,
                &coefficients,
                &mut state,
                &dry_masks,
                BLOCK_LIMIT,
            )
        } else {
            #[cfg(any(test, feature = "test-support"))]
            count_select_free_tail_pass();
            svf_cascade_interleaved_bounded::<L, 2, 1>(
                [&mut *left, &mut *right],
                frames,
                &coefficients,
                &mut state,
                BLOCK_LIMIT,
            )
        };
        let [[left_state], [right_state]] = state;
        channels.0.sections[at].state = left_state;
        channels.1.sections[at].state = right_state;
        within = Some(verdict);
    }
    within
}

/// Reads increment `index` of a step set in the pinned order.
fn step_word<L: Lane>(step: &SvfCoefStep<L>, index: usize) -> L {
    match index {
        0 => step.c1,
        1 => step.a2,
        2 => step.a3,
        3 => step.m0,
        4 => step.m1,
        _ => step.m2,
    }
}

/// One band's decoded payload words, held until every band of the lane has validated.
#[derive(Clone, Copy)]
struct RestoredBand {
    integrators: [f32; 2],
    coefficients: [f32; 6],
    step: [f32; 6],
    remaining: u32,
    target: BandTarget,
}

impl<L: Lane, const W: usize> Channel<L, W> {
    /// Writes lane `track`'s state words in the current order.
    fn snapshot_track(&self, track: usize, out: &mut [u32; STATE_LANE_WORDS]) {
        for section in 0..EQ_SECTION_COUNT {
            let base = section * STATE_WORDS_PER_BAND;
            let slot = &self.sections[section];
            out[base] = lane_get(slot.state.ic1, track).to_bits();
            out[base + 1] = lane_get(slot.state.ic2, track).to_bits();
            for index in 0..6 {
                out[base + 2 + index] = lane_get(coef_word(&slot.coef, index), track).to_bits();
                out[base + 8 + index] = lane_get(step_word(&slot.step, index), track).to_bits();
            }
            out[base + 14] = self.remaining[section][track];
            for (index, value) in self.targets[track][section]
                .numeric()
                .into_iter()
                .enumerate()
            {
                out[base + 15 + index] = value.to_bits();
            }
        }
        out[STATE_HPF_ENABLE_WORD] = u32::from(self.targets[track][HPF_SECTION].enabled);
        out[STATE_LPF_ENABLE_WORD] = u32::from(self.targets[track][LPF_SECTION].enabled);
    }

    /// Decodes the two retained dedicated-cut enable words, refusing malformed values before any
    /// state or semantic target is written.
    fn snapshot_cut_enables(
        words: &[u32; STATE_LANE_WORDS],
    ) -> Result<[bool; 2], StatePayloadError> {
        let invalid = StatePayloadError {
            code: "effect.state.payload",
        };
        let decode = |word| match word {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(invalid),
        };
        Ok([
            decode(words[STATE_HPF_ENABLE_WORD])?,
            decode(words[STATE_LPF_ENABLE_WORD])?,
        ])
    }

    /// Validates lane `track`'s state words and applies them all or none.
    ///
    /// `configuration` supplies the two parameters that are not automatable and therefore not in
    /// the payload — the band's enable and family. The ramp target words are **recomputed** from
    /// the stored parameters rather than stored, so a payload cannot carry words that disagree with
    /// the parameters it also carries.
    fn restore_track(
        &mut self,
        track: usize,
        words: &[u32; STATE_LANE_WORDS],
        configuration: &[BandTarget; EQ_SECTION_COUNT],
        sample_rate: SampleRateHz,
    ) -> Result<(), StatePayloadError> {
        let invalid = StatePayloadError {
            code: "effect.state.payload",
        };
        let mut decoded = [RestoredBand {
            integrators: [0.0; 2],
            coefficients: [0.0; 6],
            step: [0.0; 6],
            remaining: 0,
            target: configuration[0],
        }; EQ_SECTION_COUNT];
        for section in 0..EQ_SECTION_COUNT {
            let base = section * STATE_WORDS_PER_BAND;
            let read = |offset: usize| f32::from_bits(words[base + offset]);
            let band = RestoredBand {
                integrators: [read(0), read(1)],
                coefficients: core::array::from_fn(|index| read(2 + index)),
                step: core::array::from_fn(|index| read(8 + index)),
                remaining: words[base + 14],
                target: {
                    let mut target = configuration[section];
                    for index in 0..4 {
                        target.set_numeric(index, normalize_zero(read(15 + index)));
                    }
                    target
                },
            };
            let numeric = band.target.numeric();
            if !band.integrators.into_iter().all(f32::is_finite)
                || !band.coefficients.into_iter().all(f32::is_finite)
                || !band.step.into_iter().all(f32::is_finite)
                || band.remaining > RAMP_SAMPLES
                || !(0..4).all(|index| numeric_value_valid(index, numeric[index]))
            {
                return Err(invalid);
            }
            let target_words = band.target.words(sample_rate).map_err(|_| invalid)?;
            let current_words = EqSvfWords::from_array(band.coefficients);
            let step_words = EqSvfWords::from_array(band.step);
            if band
                .step
                .into_iter()
                .any(|value| value == 0.0 && value.to_bits() != 0)
            {
                return Err(invalid);
            }
            if band.remaining > 0 {
                // The current words are on a linear path between two validated designs. Validate
                // every bounded future step, so a finite but unstable forged ramp cannot enter
                // the render plane. Identity is the one valid settled endpoint with a2 == 0.
                let mut cursor = current_words;
                for _ in 0..band.remaining {
                    if !words_are_identity(cursor) {
                        validate_rounded_svf(cursor).map_err(|_| invalid)?;
                    }
                    cursor = EqSvfWords::from_array(core::array::from_fn(|index| {
                        cursor.to_array()[index] + step_words.to_array()[index]
                    }));
                }
            } else if band.step.iter().any(|value| value.to_bits() != 0) {
                // A settled lane still shares a section's bank-wide ramp kernel with its peers;
                // any nonzero step would make it drift whenever a neighbouring lane moves.
                return Err(invalid);
            }
            if band.remaining == 0
                && band
                    .coefficients
                    .iter()
                    .zip(target_words.to_array())
                    .any(|(stored, expected)| stored.to_bits() != expected.to_bits())
            {
                return Err(invalid);
            }
            decoded[section] = band;
        }
        for (section, band) in decoded.into_iter().enumerate() {
            let target_words = band
                .target
                .words(sample_rate)
                .unwrap_or(EqSvfWords::IDENTITY);
            let slot = &mut self.sections[section];
            lane_set(&mut slot.state.ic1, track, band.integrators[0]);
            lane_set(&mut slot.state.ic2, track, band.integrators[1]);
            for index in 0..6 {
                lane_set(
                    coef_word_mut(&mut slot.coef, index),
                    track,
                    band.coefficients[index],
                );
                lane_set(
                    step_word_mut(&mut slot.step, index),
                    track,
                    band.step[index],
                );
                lane_set(
                    coef_word_mut(&mut slot.target, index),
                    track,
                    target_words.to_array()[index],
                );
            }
            self.remaining[section][track] = band.remaining;
            self.targets[track][section] = band.target;
        }
        for section in 0..EQ_SECTION_COUNT {
            self.refresh_identity(section);
        }
        Ok(())
    }
}

/// Stateless native factory for prepared parametric EQs.
#[derive(Clone, Copy, Debug, Default)]
pub struct ParametricEqFactory;

/// A prepared EQ over `W = L::WIDTH` tracks: one body for the scalar effect and for every bank.
struct PreparedParametricEq<L: Lane, const W: usize> {
    metadata: PreparedEffectMetadata,
    bank: PreparedBankMetadata,
    initial: [[[BandTarget; EQ_SECTION_COUNT]; 2]; W],
    /// Designed words corresponding to `initial`, retained so a full reset never redesigns.
    initial_words: [[[EqSvfWords; EQ_SECTION_COUNT]; 2]; W],
    left: Channel<L, W>,
    right: Channel<L, W>,
    /// Issue #163 phase 4 item 1: the previous block proved this bank is at a silent fixed point.
    ///
    /// Set only by [`render`](Self::render), and only after it has *observed* -- not assumed --
    /// all three facts on a block it actually ran: the input was exactly `+0.0` everywhere, the
    /// output it produced was exactly `+0.0` everywhere, and every integrator word came out of the
    /// block bit-identical to the word it went in with. Cleared by anything that can move a
    /// coefficient or a state word out from under that observation.
    ///
    /// The induction it licenses: a bank kernel is a pure function of (input, state,
    /// coefficients). If those three are bit-identical to a block that provably produced all
    /// `+0.0` output and left its state unmoved, the next block produces the same output and
    /// leaves the state unmoved again -- so writing nothing is bit-identical to running it. This
    /// is why the flag is *earned* by observation rather than derived from a theory about where an
    /// SVF settles: the fixed point is not always `+0.0` (a negative coefficient drives an
    /// integrator to `-0.0` and it stays there), and a theory that assumed it was would either
    /// engage wrongly or never engage at all.
    silent_fixed_point: bool,
}

impl<L: Lane, const W: usize> PreparedParametricEq<L, W> {
    /// The channel's sample rate, as the design functions take it.
    fn sample_rate(&self) -> SampleRateHz {
        SampleRateHz(self.metadata.sample_rate)
    }

    /// Applies one validated prepared target to one prepared track.
    ///
    /// Decoding is bounded and allocation-free. The target's coefficient words are copied into
    /// the existing ramp state; no semantic redesign occurs on this path.
    fn apply_target_lane(
        &mut self,
        lane: usize,
        target: &PreparedEffectTarget,
    ) -> Result<(), EffectTargetError> {
        if lane >= W || lane >= L::WIDTH {
            return Err(EffectTargetError::Capacity);
        }
        let (section, channel, band, words) = control::decode_prepared_target(target)?;
        if section != HPF_SECTION && section != LPF_SECTION {
            let permitted = |prepared: BandTarget| {
                band.enabled == prepared.enabled && band.kind == prepared.kind
            };
            let left_permitted = permitted(self.initial[lane][0][section]);
            let right_permitted = permitted(self.initial[lane][1][section]);
            let allowed = match channel {
                ParameterChannel::Left => left_permitted,
                ParameterChannel::Right => right_permitted,
                ParameterChannel::Both => left_permitted && right_permitted,
            };
            if !allowed {
                return Err(EffectTargetError::Domain);
            }
        }
        self.silent_fixed_point = false;
        match channel {
            ParameterChannel::Left => self.left.apply_prepared_target(lane, section, band, words),
            ParameterChannel::Right => self.right.apply_prepared_target(lane, section, band, words),
            ParameterChannel::Both => {
                self.left.apply_prepared_target(lane, section, band, words);
                self.right.apply_prepared_target(lane, section, band, words);
            }
        }
        Ok(())
    }

    /// Runs both channels over one block and applies the master plan §4.4 boundary check.
    ///
    /// The check is `check_block`'s, one verdict per channel per block. When the stationary
    /// cascade ends in its depth-one pass on an admitted plan, that pass has already judged every
    /// word as it stored it, and its verdict is used (issue #999; see [`interleave`]); every other
    /// block -- ramped, refused, all-live, an even live count, nothing live -- scans the plane
    /// after the cascade. The verdict is the same either way, so the zeroing and the reset are too.
    /// A rejected block is zeroed and the channel's integrators are cleared; coefficients and ramps
    /// survive, because a non-finite block is a fault report, not an automation event. The two
    /// channels are judged independently: they carry independent state and independent counters,
    /// which is what dual-mono means here.
    ///
    /// `#[inline(always)]`, and [`render_mono`](Self::render_mono) with it: the stationary cascade
    /// is the EQ's arithmetic, and the shipped wasm artifact is gated on it living in the one
    /// `process_bank` (or `process_bank_mono`) symbol per width (`KERNEL_ROSTER` rule 1). The
    /// inliner decided that on its own until issue #978's skewed pairs -- a prologue, a steady
    /// state, an epilogue and a short-block fallback per pass kind -- made `render` large enough
    /// that it outlined it in the `simd128` build, which left `process_bank` carrying no
    /// arithmetic. Each width calls it from one place (the bank body, or the scalar `process`), so
    /// pinning the shape here duplicates nothing.
    #[inline(always)]
    fn render(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: usize,
    ) -> [[bool; MAX_LANES]; 2] {
        let mut failures = [[false; MAX_LANES]; 2];
        let words = frames * W;
        // Issue #163 phase 4 item 1. `quiet` is the whole precondition, evaluated whole-bank:
        // no ramp in flight on either channel (so the coefficients are the same words the
        // observed block used) and both input planes exactly `+0.0`. The block test short-circuits on
        // its first chunk, so a console rendering music pays 32 words per channel for this and
        // nothing else.
        // #163 phase 3 reuses this leg. "Stationary" -- no lane of either channel has a ramp in
        // flight -- is exactly the condition under which every section runs `svf_block` with fixed
        // coefficients over the whole block, which is the shape the interleaved cascade replaces.
        let stationary = self.left.no_ramp_in_flight() && self.right.no_ramp_in_flight();
        let quiet = stationary
            && block_is_positive_zero(&left[..words])
            && block_is_positive_zero(&right[..words]);
        if quiet && self.silent_fixed_point {
            // The buffers already hold exactly `+0.0`, which is exactly what the six sections
            // would have written; the integrators are at a fixed point the previous block
            // measured. Writing nothing is bit-identical, and an all-`+0.0` block is trivially
            // inside the §4.4 bound, so no lane failed.
            return failures;
        }
        let mut before_left = [0_u32; EQ_SECTION_COUNT * 2 * MAX_LANES];
        let mut before_right = [0_u32; EQ_SECTION_COUNT * 2 * MAX_LANES];
        if quiet {
            self.left.state_bits(&mut before_left);
            self.right.state_bits(&mut before_right);
        }
        let within = process_channels(
            (&mut self.left, &mut self.right),
            left,
            right,
            frames,
            stationary,
        );
        for (index, (channel, block)) in
            [(&mut self.left, &mut *left), (&mut self.right, &mut *right)]
                .into_iter()
                .enumerate()
        {
            // Issue #999: an admitted stationary block's last pass judged every word as it stored
            // it; every other block scans the plane.
            let passed = match within {
                Some(verdict) => verdict[index],
                None => check_block::<L>(block),
            };
            if passed {
                continue;
            }
            let mask = nonfinite_lane_mask::<L>(block);
            block.fill(0.0);
            channel.reset_states();
            for (lane, failed) in failures[index].iter_mut().enumerate().take(W) {
                *failed = mask & (1 << lane) != 0;
            }
        }
        // Earn (or lose) the flag from what this block actually did. All three legs are required:
        // the input was `+0.0` and the coefficients were still (`quiet`), the integrators came out
        // exactly as they went in, and the output the sections wrote was `+0.0` to the bit. Only
        // then is "write nothing" a faithful replay of "run the kernel".
        self.silent_fixed_point = quiet && {
            let mut after_left = [0_u32; EQ_SECTION_COUNT * 2 * MAX_LANES];
            let mut after_right = [0_u32; EQ_SECTION_COUNT * 2 * MAX_LANES];
            self.left.state_bits(&mut after_left);
            self.right.state_bits(&mut after_right);
            after_left == before_left
                && after_right == before_right
                && block_is_positive_zero(&left[..words])
                && block_is_positive_zero(&right[..words])
        };
        failures
    }

    /// [`render`](Self::render) over one plane: the collapsed track's live channel.
    ///
    /// `stationary` and `quiet` read the left channel alone, which on a collapse-eligible bank is
    /// the same predicate the dual body evaluates -- see [`process_channels_mono`] for why the
    /// two channels' `remaining` arrays and identity flags agree. `failures[1]` is copied from
    /// `failures[0]`, because the right plane the seam is about to write is this left plane and a
    /// dual run would have rejected it on the same words.
    #[inline(always)]
    fn render_mono(&mut self, left: &mut [f32], frames: usize) -> [[bool; MAX_LANES]; 2] {
        let mut failures = [[false; MAX_LANES]; 2];
        let words = frames * W;
        let stationary = self.left.no_ramp_in_flight();
        let quiet = stationary && block_is_positive_zero(&left[..words]);
        if quiet && self.silent_fixed_point {
            return failures;
        }
        let mut before_left = [0_u32; EQ_SECTION_COUNT * 2 * MAX_LANES];
        if quiet {
            self.left.state_bits(&mut before_left);
        }
        let within = process_channels_mono(&mut self.left, left, frames, stationary);
        if !within.unwrap_or_else(|| check_block::<L>(left)) {
            let mask = nonfinite_lane_mask::<L>(left);
            left.fill(0.0);
            self.left.reset_states();
            for (lane, failed) in failures[0].iter_mut().enumerate().take(W) {
                *failed = mask & (1 << lane) != 0;
            }
            failures[1] = failures[0];
        }
        self.silent_fixed_point = quiet && {
            let mut after_left = [0_u32; EQ_SECTION_COUNT * 2 * MAX_LANES];
            self.left.state_bits(&mut after_left);
            after_left == before_left && block_is_positive_zero(&left[..words])
        };
        failures
    }

    /// Copies the left channel's whole per-channel state onto the right (the disengage boundary).
    ///
    /// # The copy list, term by term
    ///
    /// | word | why it is here |
    /// |---|---|
    /// | `sections[s].state` | the two integrators per section -- the cascade's whole cross-frame state. |
    /// | `sections[s].coef` | the words the kernel loads. A ramp rewrites them per segment through the ramp kernel, and a collapsed block advances only the left channel's. |
    /// | `sections[s].step`, `sections[s].target` | the rest of the ramp: where the words are going and by how much per sample. |
    /// | `remaining[s][lane]` | the per-lane ramp countdown, which is what `no_ramp_in_flight` and the block-splitting rule read. |
    /// | `identity[s]` | the derived per-section identity flag the elision gate reads. It is a function of `coef`, so it must travel with it. |
    ///
    /// `targets` is deliberately **not** copied: it is the control-plane band table, no rendered
    /// block writes it, and the counterfactual dual run never moved it either.
    /// # Which entries are individually gated, and which are here by the rule
    ///
    /// `crates/parametric-eq/tests/mono_collapse.rs` fails if `sections` is dropped.
    /// `remaining` and `identity` are **not** individually gated, and the reason is specific: both
    /// are read only to choose a *schedule* -- `no_ramp_in_flight` selects the interleaved cascade
    /// over the per-section one, and `identity` selects the elided cascade over the full one --
    /// and this crate's own gates prove all three schedules render the same bits. So a stale copy
    /// of either leaves the two channels taking different schedules to the same words. They are
    /// copied because "the two channels are in the same state" is the invariant, not "the two
    /// channels happen to agree on their output".
    fn desymmetrize(&mut self) {
        self.right.sections = self.left.sections;
        self.right.remaining = self.left.remaining;
        self.right.identity = self.left.identity;
    }

    /// Restores every band of both channels to the parameters preparation was given.
    fn reset_to_defaults(&mut self) -> Result<(), EqDesignError> {
        let left = core::array::from_fn(|track| self.initial[track][0]);
        let right = core::array::from_fn(|track| self.initial[track][1]);
        let left_words = core::array::from_fn(|track| self.initial_words[track][0]);
        let right_words = core::array::from_fn(|track| self.initial_words[track][1]);
        self.left = Channel::from_prepared(left, left_words);
        self.right = Channel::from_prepared(right, right_words);
        Ok(())
    }
}

/// Reads one track's four general bands out of a validated initial-value list.
///
/// The contract validates that the list is exactly one value per parameter per channel, in
/// descriptor order, before this runs, so the index arithmetic needs no search and no allocation.
fn band_targets(
    values: &[InitialParameterValue],
    channel: usize,
    sample_rate: SampleRateHz,
) -> Result<[BandTarget; EQ_BAND_COUNT], EffectPrepareError> {
    let mut bands = [BandTarget {
        enabled: false,
        kind: EqBandKind::Bell,
        frequency: 0.0,
        gain: 0.0,
        q: 0.0,
        slope: 0.0,
    }; EQ_BAND_COUNT];
    for (section, band) in bands.iter_mut().enumerate() {
        let field = |index: usize| values[(section * 6 + index) * 2 + channel].value;
        *band = BandTarget {
            enabled: field(0).to_bits() == 1.0_f32.to_bits(),
            kind: EqBandKind::from_value(field(1)).ok_or(EffectPrepareError {
                code: "effect.parameter.initial",
            })?,
            frequency: field(2),
            gain: field(3),
            q: field(4),
            slope: field(5),
        };
        band.words(sample_rate).map_err(|_| EffectPrepareError {
            code: "effect.eq.coefficients",
        })?;
    }
    Ok(bands)
}

/// Reads one dedicated cut's three prepared-target values. The cut is represented by the same
/// section target used by the general bands, with fixed zero gain and unit shelf slope so the
/// existing coefficient authority and state machinery remain shared.
fn cut_target(
    values: &[InitialParameterValue],
    channel: usize,
    sample_rate: SampleRateHz,
    base: usize,
    kind: EqBandKind,
) -> Result<BandTarget, EffectPrepareError> {
    let field = |parameter: usize| values[parameter * 2 + channel].value;
    let target = BandTarget {
        enabled: field(base).to_bits() == 1.0_f32.to_bits(),
        kind,
        frequency: field(base + 1),
        gain: 0.0,
        q: field(base + 2),
        slope: 1.0,
    };
    target.words(sample_rate).map_err(|_| EffectPrepareError {
        code: "effect.eq.coefficients",
    })?;
    Ok(target)
}

/// Reads and lays out one track's six physical sections. Parameter order stays the original four
/// general bands followed by the six appended cut rows; DSP order is HPF, bands 1..4, LPF.
fn physical_targets(
    values: &[InitialParameterValue],
    channel: usize,
    sample_rate: SampleRateHz,
) -> Result<[BandTarget; EQ_SECTION_COUNT], EffectPrepareError> {
    let bands = band_targets(values, channel, sample_rate)?;
    let hpf = cut_target(
        values,
        channel,
        sample_rate,
        EQ_BAND_COUNT * 6,
        EqBandKind::HighPass,
    )?;
    let lpf = cut_target(
        values,
        channel,
        sample_rate,
        EQ_BAND_COUNT * 6 + 3,
        EqBandKind::LowPass,
    )?;
    let mut sections = [hpf; EQ_SECTION_COUNT];
    sections[HPF_SECTION] = hpf;
    sections[BAND_SECTION_OFFSET..BAND_SECTION_OFFSET + EQ_BAND_COUNT].copy_from_slice(&bands);
    sections[LPF_SECTION] = lpf;
    Ok(sections)
}

/// Builds a prepared EQ of width `W` from one request per track.
fn prepare_width<L: Lane, const W: usize>(
    metadata: PreparedEffectMetadata,
    width: BankWidth,
    requests: &[PrepareEffectRequest<'_>],
) -> Result<PreparedParametricEq<L, W>, EffectPrepareError> {
    debug_assert_eq!(L::WIDTH, W);
    let sample_rate = SampleRateHz(metadata.sample_rate);
    let mut initial = [[[BandTarget {
        enabled: false,
        kind: EqBandKind::Bell,
        frequency: 0.0,
        gain: 0.0,
        q: 0.0,
        slope: 0.0,
    }; EQ_SECTION_COUNT]; 2]; W];
    for (track, request) in requests.iter().enumerate() {
        initial[track][0] = physical_targets(request.initial_values, 0, sample_rate)?;
        initial[track][1] = physical_targets(request.initial_values, 1, sample_rate)?;
    }
    let mut initial_words = [[[EqSvfWords::IDENTITY; EQ_SECTION_COUNT]; 2]; W];
    for track in 0..W {
        for channel in 0..2 {
            for section in 0..EQ_SECTION_COUNT {
                initial_words[track][channel][section] = initial[track][channel][section]
                    .words(sample_rate)
                    .map_err(|_| EffectPrepareError {
                        code: "effect.eq.coefficients",
                    })?;
            }
        }
    }
    Ok(PreparedParametricEq {
        metadata,
        bank: PreparedBankMetadata {
            width,
            program_key: metadata.program_key(),
        },
        initial,
        initial_words,
        left: Channel::from_prepared(
            core::array::from_fn(|track| initial[track][0]),
            core::array::from_fn(|track| initial_words[track][0]),
        ),
        right: Channel::from_prepared(
            core::array::from_fn(|track| initial[track][1]),
            core::array::from_fn(|track| initial_words[track][1]),
        ),
        // Nothing has been observed yet, so nothing is claimed.
        silent_fixed_point: false,
    })
}

impl NativeEffectFactory for ParametricEqFactory {
    fn descriptor(&self) -> &'static EffectDescriptor {
        &PARAMETRIC_EQ_DESCRIPTOR
    }

    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        let metadata = expected_prepared_metadata(self.descriptor(), request)?;
        Ok(Box::new(prepare_width::<f32, 1>(
            metadata,
            BankWidth::Four,
            &[request],
        )?))
    }

    fn response_analysis(&self) -> Option<&dyn NativeEffectResponseFactory> {
        Some(self)
    }

    fn target_preparation(&self) -> Option<&dyn effect_contract::NativeEffectTargetPreparation> {
        Some(self)
    }

    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
        // Issue #95: a self-contradicting shape is a contract violation and a typed error; a
        // width this build does not execute is a capability gap and a legal `Ok(None)`. This
        // crate used to answer `Ok(None)` to both, which was the other half of the wave-2
        // divergence (`NativeEffectFactory::bind_homogeneous_bank` states the frozen rule).
        request.validate_shape()?;
        let lanes = request.width.lanes() as usize;
        if lanes != Backend::current().width() {
            return Ok(None);
        }
        let first = request
            .requests
            .first()
            .copied()
            .ok_or(EffectPrepareError {
                code: "effect.bank.requests",
            })?;
        let metadata = expected_prepared_metadata(self.descriptor(), first)?;
        for item in request.requests.iter().copied() {
            let candidate = expected_prepared_metadata(self.descriptor(), item)?;
            if candidate.program_key() != metadata.program_key() {
                return Ok(None);
            }
        }
        Ok(Some(match request.width {
            BankWidth::Four => Box::new(prepare_width::<Simd4, 4>(
                metadata,
                request.width,
                request.requests,
            )?) as Box<dyn PreparedNativeEffectBank>,
            BankWidth::Eight => Box::new(prepare_width::<Simd8, 8>(
                metadata,
                request.width,
                request.requests,
            )?) as Box<dyn PreparedNativeEffectBank>,
        }))
    }
}

/// Maps the shared codec's error onto the contract's.
fn runtime_state_error(error: payload::StatePayloadError) -> StatePayloadError {
    StatePayloadError { code: error.code }
}

/// Reads a whole payload — header, then both lane sections — rejecting before anything is decoded.
fn read_payload(
    input: StatePayloadInput<'_>,
) -> Result<([u32; STATE_LANE_WORDS], [u32; STATE_LANE_WORDS]), StatePayloadError> {
    let mut left = [0_u32; STATE_LANE_WORDS];
    let mut right = [0_u32; STATE_LANE_WORDS];
    payload::restore(
        &STATE_LAYOUT,
        &payload::StatePayloadInput {
            common: input.common,
            left: input.left,
            right: input.right,
        },
        &mut payload::StateWordsMut {
            common: &mut [],
            left: &mut left,
            right: &mut right,
        },
    )
    .map_err(runtime_state_error)?;
    Ok((left, right))
}

/// Writes the header and one lane pair into a payload.
fn write_payload(
    output: StatePayloadOutput<'_>,
    left: &[u32; STATE_LANE_WORDS],
    right: &[u32; STATE_LANE_WORDS],
) -> Result<(), StatePayloadError> {
    payload::snapshot(
        &STATE_LAYOUT,
        &payload::StateWords {
            common: &[],
            left,
            right,
        },
        &mut payload::StatePayloadOutput {
            common: output.common,
            left: output.left,
            right: output.right,
        },
    )
    .map_err(runtime_state_error)
}

impl<L: Lane, const W: usize> PreparedParametricEq<L, W> {
    /// Snapshots one track, shared by the scalar and bank surfaces.
    fn snapshot_track(
        &self,
        track: usize,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        let mut left = [0_u32; STATE_LANE_WORDS];
        let mut right = [0_u32; STATE_LANE_WORDS];
        self.left.snapshot_track(track, &mut left);
        self.right.snapshot_track(track, &mut right);
        write_payload(output, &left, &right)
    }

    /// Restores one track, all or none across both channels.
    fn restore_track(
        &mut self,
        track: usize,
        version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        if version != STATE_LAYOUT_VERSION {
            return Err(StatePayloadError {
                code: payload::STATE_VERSION_CODE,
            });
        }
        let (left, right) = read_payload(input)?;
        let left_enables = Channel::<L, W>::snapshot_cut_enables(&left)?;
        let right_enables = Channel::<L, W>::snapshot_cut_enables(&right)?;
        let mut left_configuration = self.initial[track][0];
        let mut right_configuration = self.initial[track][1];
        left_configuration[HPF_SECTION].enabled = left_enables[0];
        left_configuration[LPF_SECTION].enabled = left_enables[1];
        right_configuration[HPF_SECTION].enabled = right_enables[0];
        right_configuration[LPF_SECTION].enabled = right_enables[1];
        let sample_rate = self.sample_rate();
        let mut candidate_left = Channel::<L, W>::new(
            core::array::from_fn(|index| self.left.targets[index]),
            sample_rate,
        )
        .map_err(|_| StatePayloadError {
            code: "effect.state.payload",
        })?;
        candidate_left.restore_track(track, &left, &left_configuration, sample_rate)?;
        let mut candidate_right = Channel::<L, W>::new(
            core::array::from_fn(|index| self.right.targets[index]),
            sample_rate,
        )
        .map_err(|_| StatePayloadError {
            code: "effect.state.payload",
        })?;
        candidate_right.restore_track(track, &right, &right_configuration, sample_rate)?;
        self.left
            .restore_track(track, &left, &left_configuration, sample_rate)?;
        self.right
            .restore_track(track, &right, &right_configuration, sample_rate)?;
        Ok(())
    }
}

/// The `DESIGNED` term of the channel-symmetry witness, over the EQ's own kernel read surface.
///
/// # The word list, and why it is exactly this
///
/// `svf_cascade_interleaved` (`lane::kernels`) loads one thing per section per
/// channel: `SvfCoef` -- the six words `c1, a2, a3, m0, m1, m2`. The ramped body
/// (`svf_block_ramped`) additionally advances those words by `SvfCoefStep` and, at the sample the
/// ramp lands on, assigns `target` over `coef`; how many samples remain is `Channel::remaining`.
/// So the designed surface for lane `l` of one channel is
///
/// * `sections[s].coef`   -- 6 x 6 words, what this frame uses;
/// * `sections[s].step`   -- 6 x 6 words, the increment, exactly `+0.0` on a settled lane;
/// * `sections[s].target` -- 6 x 6 words, where the ramp lands;
/// * `remaining[s][l]`    -- 6 counters, when it lands.
///
/// Two channels that agree on all 114 of those words produce bit-identical output from
/// bit-identical input, and stay in agreement for as long as no per-channel write arrives:
/// the ramp kernel applies the same increment to the same word, and `snap` assigns the same
/// target on the same sample.
///
/// Deliberately excluded, each for its own reason:
///
/// * `sections[s].state` (`ic1`, `ic2`) -- running filter state, not a designed word. It is what
///   the *state* half of the collapse argument is about, and it converges by induction rather
///   than by comparison.
/// * `identity[s]` -- a per-**channel** flag over *every* lane of the bank, refreshed from the
///   same `coef` words this already compares. Reading it would make one lane's witness depend on
///   its neighbours' parameters, which is precisely the cross-lane coupling the witness must not
///   have; and it carries no information the coefficient comparison does not.
/// * `targets[l][s]` (`BandTarget`) -- the control-plane band description. The kernel never reads
///   it, and it is the design *input*, not the designed word: two different band descriptions
///   that design to the same six words are symmetric, and the words are what decides that.
/// * `silent_fixed_point`, `metadata.bypass` -- whole-instance, so they cannot be asymmetric.
impl<L: Lane, const W: usize> PreparedParametricEq<L, W> {
    fn designed_channel_symmetry(&self, lane: usize) -> bool {
        if lane >= W || lane >= L::WIDTH {
            return false;
        }
        for section in 0..EQ_SECTION_COUNT {
            let left = &self.left.sections[section];
            let right = &self.right.sections[section];
            for word in 0..EQ_COEFFICIENT_WORDS {
                if lane_bits(coef_word(&left.coef, word), lane)
                    != lane_bits(coef_word(&right.coef, word), lane)
                    || lane_bits(step_word(&left.step, word), lane)
                        != lane_bits(step_word(&right.step, word), lane)
                    || lane_bits(coef_word(&left.target, word), lane)
                        != lane_bits(coef_word(&right.target, word), lane)
                {
                    return false;
                }
            }
            if self.left.remaining[section][lane] != self.right.remaining[section][lane] {
                return false;
            }
        }
        true
    }
}

impl PreparedNativeEffect for PreparedParametricEq<f32, 1> {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.metadata
    }

    fn copy_response_snapshot(
        &self,
        request: ResponseSnapshotRequest<'_>,
    ) -> Result<ResponseSnapshotSummary, ResponseAnalysisError> {
        self.copy_response_snapshot_for_lane(0, request)
    }

    fn channel_symmetry(&self) -> bool {
        self.designed_channel_symmetry(0)
    }

    fn apply_prepared_target(
        &mut self,
        target: &PreparedEffectTarget,
    ) -> Result<(), EffectTargetError> {
        self.apply_target_lane(0, target)
    }

    fn reset(&mut self, kind: ResetKind) {
        // #163 phase 4 item 1: a reset moves state, and `FullToDefaults` moves coefficients too.
        // The flag is a claim about a block that has now been overwritten, so it is withdrawn.
        self.silent_fixed_point = false;
        match kind {
            ResetKind::FullToDefaults => {
                let _ = self.reset_to_defaults();
            }
            ResetKind::DiscontinuityKeepParameters => {
                self.left.discontinuity_reset();
                self.right.discontinuity_reset();
            }
        }
    }

    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        let mut report = ProcessReport::default();
        if !block.automation.is_empty() {
            self.silent_fixed_point = false;
        }
        // EQ semantic spans are control-plane input. A production owner must lower them to
        // prepared targets before this entry point; never design coefficients on the render
        // thread. Count every raw span while rendering the existing state unchanged.
        report.invalid_spans = block.automation.len() as u64;
        if self.metadata.bypass {
            return report;
        }
        let frames = block.left.len();
        let failures = self.render(block.left, block.right, frames);
        report.nonfinite_left_blocks = u64::from(failures[0][0]);
        report.nonfinite_right_blocks = u64::from(failures[1][0]);
        report
    }

    fn snapshot_state_payload(
        &self,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.snapshot_track(0, output)
    }

    fn restore_state_payload(
        &mut self,
        version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        // A refusal is a state-preserving no-op, including the fixed-point witness.
        let result = self.restore_track(0, version, input);
        if result.is_ok() {
            self.silent_fixed_point = false;
        }
        result
    }
}

impl<L: Lane, const W: usize> PreparedNativeEffectBank for PreparedParametricEq<L, W> {
    fn metadata(&self) -> PreparedBankMetadata {
        self.bank.clone()
    }

    fn copy_response_snapshot_lane(
        &self,
        lane: usize,
        request: ResponseSnapshotRequest<'_>,
    ) -> Result<ResponseSnapshotSummary, ResponseAnalysisError> {
        self.copy_response_snapshot_for_lane(lane, request)
    }

    fn lane_channel_symmetry(&self, lane: usize) -> bool {
        self.designed_channel_symmetry(lane)
    }

    fn apply_prepared_target_lane(
        &mut self,
        lane: usize,
        target: &PreparedEffectTarget,
    ) -> Result<(), EffectTargetError> {
        self.apply_target_lane(lane, target)
    }

    fn reset(&mut self, kind: ResetKind) {
        // #163 phase 4 item 1: a reset moves state, and `FullToDefaults` moves coefficients too.
        // The flag is a claim about a block that has now been overwritten, so it is withdrawn.
        self.silent_fixed_point = false;
        match kind {
            ResetKind::FullToDefaults => {
                let _ = self.reset_to_defaults();
            }
            ResetKind::DiscontinuityKeepParameters => {
                self.left.discontinuity_reset();
                self.right.discontinuity_reset();
            }
        }
    }

    fn supports_mono_collapse(&self) -> bool {
        true
    }

    fn process_bank_mono(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        self.process_bank_inner::<true>(block)
    }

    fn desymmetrize_channels(&mut self) {
        self.desymmetrize();
    }

    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        self.process_bank_inner::<false>(block)
    }

    fn snapshot_track_state_payload(
        &self,
        track_index: u32,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.snapshot_track(bank_track_index(track_index, W)?, output)
    }

    fn restore_track_state_payload(
        &mut self,
        track_index: u32,
        version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        // A refusal is a state-preserving no-op, including the fixed-point witness.
        let result = self.restore_track(bank_track_index(track_index, W)?, version, input);
        if result.is_ok() {
            self.silent_fixed_point = false;
        }
        result
    }
}

impl<L: Lane, const W: usize> PreparedParametricEq<L, W> {
    /// Copies one lane's retained target words into the live response transfer record.
    ///
    /// This reads the exact words held by each channel's `Section::target`; it never redesigns a
    /// band from parameter values. The render owner supplies the bypass bit for the same boundary.
    fn copy_response_snapshot_for_lane(
        &self,
        lane: usize,
        request: ResponseSnapshotRequest<'_>,
    ) -> Result<ResponseSnapshotSummary, ResponseAnalysisError> {
        if lane >= W || lane >= L::WIDTH {
            return Err(ResponseAnalysisError::Capacity);
        }
        if request.left.len() != EQ_SECTION_COUNT || request.right.len() != EQ_SECTION_COUNT {
            return Err(ResponseAnalysisError::OutputShape);
        }
        let bypassed = request.bypassed;
        let left = request.left;
        let right = request.right;
        for (public_section, physical_section) in
            response::RESPONSE_TO_PHYSICAL.into_iter().enumerate()
        {
            let left_target = self.left.targets[lane][physical_section];
            let right_target = self.right.targets[lane][physical_section];
            let left_words = self.left.target_words(physical_section, lane).to_array();
            let right_words = self.right.target_words(physical_section, lane).to_array();
            let mut left_bits = [0_u32; effect_contract::RESPONSE_SNAPSHOT_WORDS];
            let mut right_bits = [0_u32; effect_contract::RESPONSE_SNAPSHOT_WORDS];
            for (destination, source) in left_bits[..left_words.len()].iter_mut().zip(left_words) {
                *destination = source.to_bits();
            }
            for (destination, source) in right_bits[..right_words.len()].iter_mut().zip(right_words)
            {
                *destination = source.to_bits();
            }
            left[public_section] = ResponseSnapshotSection {
                id: u32::try_from(public_section + 1).expect("EQ section count fits u32"),
                kind: left_target.kind as u32,
                enabled: left_target.enabled,
                word_count: u8::try_from(left_words.len()).expect("EQ words fit u8"),
                words: left_bits,
            };
            right[public_section] = ResponseSnapshotSection {
                id: u32::try_from(public_section + 1).expect("EQ section count fits u32"),
                kind: right_target.kind as u32,
                enabled: right_target.enabled,
                word_count: u8::try_from(right_words.len()).expect("EQ words fit u8"),
                words: right_bits,
            };
        }
        Ok(ResponseSnapshotSummary {
            kind: ResponseSnapshotKind::ParametricEq,
            sample_rate_hz: self.metadata.sample_rate,
            bypassed,
            sections: EQ_SECTION_COUNT as u32,
        })
    }

    /// The one bank body, dual or collapsed: the shape guard, the automation drain, the bypass
    /// short circuit and the report are the same statements in the same order, and `MONO` chooses
    /// the render body and nothing else.
    ///
    /// # Why `MONO` is a const generic and not an argument
    ///
    /// It was an argument, and the measurement said no. One body carrying both cascades is one
    /// function for the inliner to weigh, and the shipped dual path got slower: the
    /// `sixty_four_track_eq_only` row moved 28% against its sealed number on a session that never
    /// collapses. A const generic monomorphises the two, so the dual instantiation is the code that
    /// shipped before the collapse existed and the collapsed one is its own function. "Adding a
    /// path must not move the path already there" is the same rule `ConsoleEffectBankStage` exists
    /// for, applied inside a kernel.
    ///
    /// `#[inline(always)]` for the reason on [`render`](Self::render): each instantiation has one
    /// caller, `process_bank` or `process_bank_mono`, and the roster finds the EQ's arithmetic in
    /// those two symbols. Issue #978's larger render body made the inliner outline the dual
    /// instantiation in the `simd128` build; the pin keeps the shape the roster was measured on.
    #[inline(always)]
    fn process_bank_inner<const MONO: bool>(
        &mut self,
        block: EffectBankProcessBlock<'_>,
    ) -> BankProcessReport {
        let mut report = BankProcessReport::empty(self.bank.width);
        if !bank_block_matches(&block, self.bank.width, self.metadata.quantum)
            || self.bank.width.lanes() as usize != W
        {
            return report;
        }
        if !block.automation.is_empty() {
            self.silent_fixed_point = false;
        }
        for track in 0..W {
            let start = block.automation_offsets[track] as usize;
            let end = block.automation_offsets[track + 1] as usize;
            // Raw semantic EQ spans are refused per addressed bank lane. The bank continues
            // rendering its already prepared state, so no render-thread designer is reachable.
            report.reports[track].invalid_spans = (end - start) as u64;
        }
        if self.metadata.bypass {
            return report;
        }
        let frames = block.frames as usize;
        let failures = if MONO {
            self.render_mono(block.left, frames)
        } else {
            self.render(block.left, block.right, frames)
        };
        for (track, entry) in report.reports.iter_mut().enumerate().take(W) {
            entry.nonfinite_left_blocks = u64::from(failures[0][track]);
            entry.nonfinite_right_blocks = u64::from(failures[1][track]);
        }
        report
    }
}

/// Rejects a track index outside the bank.
fn bank_track_index(track_index: u32, lanes: usize) -> Result<usize, StatePayloadError> {
    let track = track_index as usize;
    if track >= lanes {
        return Err(StatePayloadError {
            code: "effect.bank.track",
        });
    }
    Ok(track)
}

/// The once-per-block bank entry guard: shapes, offsets and sample arithmetic.
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
            .checked_add(u64::from(block.frames))
            .is_some()
        && block.automation_offsets.len() == lanes + 1
        && block.automation_offsets.first() == Some(&0)
        && block.automation_offsets.last().copied() == Some(block.automation.len() as u32)
        && !block
            .automation_offsets
            .windows(2)
            .any(|pair| pair[0] > pair[1])
}

pub mod corpus;

/// Issue #163 phase 3: the interleaved cascade is the per-section path, bit for bit.
///
/// The frozen E9 corpus (`corpus.rs`, `tests/determinism.rs`) drives [`Channel::process_block`]
/// directly, because its whole job is to describe *arithmetic* independently of layout -- so it
/// runs one channel at a time and never reaches [`process_channels`]. That leaves the phase-3 fast
/// path outside the pinned digests, which is exactly where a schedule change could hide. This
/// module closes that: it drives both arms of [`process_channels`] over the same corpus signals and
/// asserts equality of the rendered audio **and** of every integrator word left behind, at all
/// three widths.
///
/// The two arms are the *same entry point* with its stationary flag flipped, so the sequential arm
/// is literally the code the EQ ran before phase 3 rather than a re-transcription of it, and no
/// runtime tuning knob is added to reach it.
#[cfg(test)]
mod interleave_identity {
    use super::{
        BAND_SECTION_OFFSET, BandTarget, Channel, EFFECTIVE_CASCADE_DEPTH, EQ_BAND_COUNT,
        EQ_SECTION_COUNT, EqBandKind, HPF_SECTION, LPF_SECTION, MAX_LANES, PreparedParametricEq,
        RAMP_SAMPLES, STATE_SIZES, SampleRateHz, Section, cascade_sections, cascade_sections_mono,
        corpus, process_channels, process_channels_mono,
    };
    use lane::kernels::svf_block;
    use lane::{Lane, Simd4, Simd8};

    /// `true` when both integrator words of `section` are exactly `+0.0` on every lane.
    ///
    /// Before issue #979 this was elision leg (b). The leg now accepts any inert state, so the
    /// exact "this section holds no state" question lives here, with the one test that asks it.
    fn section_state_is_positive_zero<L: Lane>(section: &Section<L>) -> bool {
        effect_runtime::bank::lane_is_positive_zero::<L>(section.state.ic1)
            && effect_runtime::bank::lane_is_positive_zero::<L>(section.state.ic2)
    }

    /// Frames per case: the corpus length, several blocks' worth of settling.
    const FRAMES: usize = corpus::FRAMES;

    /// One channel of `W` tracks from the corpus band table, offset so the two channels of a case
    /// never carry the same configuration.
    fn channel<L: Lane, const W: usize>(offset: usize) -> Channel<L, W> {
        let targets: [[BandTarget; EQ_SECTION_COUNT]; W] =
            core::array::from_fn(|lane| corpus::sections((offset + lane) % corpus::LANES));
        Channel::new(targets, corpus::CORPUS_RATE).expect("every corpus row is a legal design")
    }

    /// One AoSoA block of corpus audio for `W` tracks.
    fn block<const W: usize>(case: usize, offset: usize) -> Vec<f32> {
        let mut lanes = vec![[0.0_f32; FRAMES]; W];
        for (index, lane) in lanes.iter_mut().enumerate() {
            corpus::fill(case, (offset + index) % corpus::LANES, lane);
        }
        let mut out = vec![0.0_f32; FRAMES * W];
        for frame in 0..FRAMES {
            for (index, lane) in lanes.iter().enumerate() {
                out[frame * W + index] = lane[frame];
            }
        }
        out
    }

    /// Every integrator word of a channel pair, as raw bits.
    fn integrators<L: Lane, const W: usize>(
        left: &Channel<L, W>,
        right: &Channel<L, W>,
    ) -> Vec<u32> {
        let mut words = [0_u32; EQ_SECTION_COUNT * 2 * MAX_LANES];
        let mut out = Vec::new();
        left.state_bits(&mut words);
        out.extend_from_slice(&words);
        right.state_bits(&mut words);
        out.extend_from_slice(&words);
        out
    }

    /// Raw bits of a block.
    fn bits(samples: &[f32]) -> Vec<u32> {
        samples.iter().map(|sample| sample.to_bits()).collect()
    }

    /// The four-section oracle that represents the pre-cut prepared cascade.
    ///
    /// This intentionally owns only the four original sections and calls the existing
    /// per-section [`svf_block`] kernel. It is therefore an independent schedule/shape oracle for
    /// the compatibility claim; a second six-section path would only prove that two new paths
    /// agree. The physical six-section channel supplies the same settled coefficient/state words
    /// for sections 1..4, so no filter arithmetic is reimplemented here.
    fn original_four<L: Lane, const W: usize>(offset: usize) -> [Section<L>; EQ_BAND_COUNT] {
        let six = channel::<L, W>(offset);
        core::array::from_fn(|band| six.sections[BAND_SECTION_OFFSET + band])
    }

    fn process_original_four<L: Lane>(
        sections: &mut [Section<L>; EQ_BAND_COUNT],
        io: &mut [f32],
        frames: usize,
    ) {
        for section in sections {
            svf_block::<L>(io, frames, &section.coef, &mut section.state);
        }
    }

    /// Four-band ramp oracle for the one synchronized ramp used below. It mirrors the production
    /// driver's current-then-advance order, final snap, and fixed tail without reimplementing
    /// coefficient design.
    fn process_original_four_ramped<L: Lane, const W: usize>(
        sections: &mut [Section<L>; EQ_BAND_COUNT],
        io: &mut [f32],
        frames: usize,
    ) {
        for (index, section) in sections.iter_mut().enumerate() {
            if index == 1 {
                let ramp = RAMP_SAMPLES as usize;
                lane::kernels::svf_block_ramped::<L>(
                    &mut io[..ramp * W],
                    ramp,
                    &mut section.coef,
                    &section.step,
                    ramp,
                    &mut section.state,
                );
                section.coef = section.target;
                section.step = Default::default();
                lane::kernels::svf_block::<L>(
                    &mut io[ramp * W..frames * W],
                    frames - ramp,
                    &section.coef,
                    &mut section.state,
                );
            } else {
                lane::kernels::svf_block::<L>(io, frames, &section.coef, &mut section.state);
            }
        }
    }

    /// The exact signed-zero boundary that a disabled cut must preserve for the old four-band
    /// schedule. This retained regression records the historical signed-zero compatibility failure
    /// that the prepared path now fixes with per-lane dry selection.
    fn all_high_pass_channel<L: Lane, const W: usize>() -> Channel<L, W> {
        let disabled = BandTarget {
            enabled: false,
            kind: EqBandKind::HighPass,
            frequency: 1_000.0,
            gain: 0.0,
            q: 1.0,
            slope: 1.0,
        };
        let live = BandTarget {
            enabled: true,
            ..disabled
        };
        let mut targets = [disabled; EQ_SECTION_COUNT];
        for target in targets
            .iter_mut()
            .skip(BAND_SECTION_OFFSET)
            .take(EQ_BAND_COUNT)
        {
            *target = live;
        }
        Channel::new([targets; W], SampleRateHz(44_100)).expect("legal all-HighPass design")
    }

    fn original_state_bits<L: Lane>(sections: &[Section<L>; EQ_BAND_COUNT]) -> Vec<u32> {
        let mut words = [0_u32; EQ_BAND_COUNT * 2 * MAX_LANES];
        for (index, section) in sections.iter().enumerate() {
            let base = index * 2 * L::WIDTH;
            section
                .state
                .ic1
                .store_bits(&mut words[base..base + L::WIDTH]);
            section
                .state
                .ic2
                .store_bits(&mut words[base + L::WIDTH..base + 2 * L::WIDTH]);
        }
        words.to_vec()
    }

    /// Compares the six-section prepared renderer with the direct pre-cut four-section oracle.
    ///
    /// With both dedicated cuts disabled and an admissible finite input, the elision list contains
    /// exactly the original four physical sections. The output and original-band integrator words
    /// must remain bit-identical for dual and collapsed mono execution at every launch width,
    /// including a non-cold state.
    fn compare_disabled_cuts_to_original_four<L: Lane, const W: usize>(
        width: &str,
        case: usize,
        seeded: bool,
        mono: bool,
        refusal_state: bool,
        negative_zero: bool,
    ) {
        let mut six_left = channel::<L, W>(0);
        let mut six_right = channel::<L, W>(3);
        let mut four_left = original_four::<L, W>(0);
        let mut four_right = original_four::<L, W>(3);
        if seeded {
            for section in 0..EQ_BAND_COUNT {
                six_left.sections[BAND_SECTION_OFFSET + section].state.ic1 = L::splat(1.0e-40);
                six_left.sections[BAND_SECTION_OFFSET + section].state.ic2 = L::splat(-1.0e-41);
                four_left[section].state.ic1 = L::splat(1.0e-40);
                four_left[section].state.ic2 = L::splat(-1.0e-41);
                six_right.sections[BAND_SECTION_OFFSET + section].state.ic1 = L::splat(-3.5e-7);
                six_right.sections[BAND_SECTION_OFFSET + section].state.ic2 = L::splat(9.0e-8);
                four_right[section].state.ic1 = L::splat(-3.5e-7);
                four_right[section].state.ic2 = L::splat(9.0e-8);
            }
        }
        if refusal_state {
            // A restored negative-zero integrator in a live original band refuses elision. The
            // direct four-section oracle carries the same state, so this still checks old-band
            // output/state bits while exercising the full six-section fallback.
            six_left.sections[BAND_SECTION_OFFSET].state.ic1 = L::splat(-0.0);
            four_left[0].state.ic1 = L::splat(-0.0);
        }
        let mut six_left_io = block::<W>(case, 0);
        let mut six_right_io = block::<W>(case, 3);
        let mut four_left_io = six_left_io.clone();
        let mut four_right_io = six_right_io.clone();
        if negative_zero {
            six_left_io[0] = -0.0;
            four_left_io[0] = -0.0;
        }
        let kept = if mono {
            cascade_sections_mono(&six_left, &six_left_io, FRAMES).1
        } else {
            cascade_sections(&six_left, &six_right, &six_left_io, &six_right_io, FRAMES).1
        };
        if negative_zero || refusal_state {
            assert_eq!(
                kept,
                EQ_SECTION_COUNT,
                "{width} {mode}: compatibility oracle refusal must retain all six sections",
                mode = if mono { "mono" } else { "dual" }
            );
        }
        if mono {
            process_channels_mono(&mut six_left, &mut six_left_io, FRAMES, true);
            process_original_four(&mut four_left, &mut four_left_io, FRAMES);
            assert_eq!(
                bits(&six_left_io),
                bits(&four_left_io),
                "{width} mono output differs from the direct four-band oracle, case={case}, seeded={seeded}"
            );
            assert_eq!(
                original_state_bits(&core::array::from_fn(|band| {
                    six_left.sections[BAND_SECTION_OFFSET + band]
                })),
                original_state_bits(&four_left),
                "{width} mono original-band state differs from the direct four-band oracle, case={case}, seeded={seeded}"
            );
        } else {
            process_channels(
                (&mut six_left, &mut six_right),
                &mut six_left_io,
                &mut six_right_io,
                FRAMES,
                true,
            );
            process_original_four(&mut four_left, &mut four_left_io, FRAMES);
            process_original_four(&mut four_right, &mut four_right_io, FRAMES);
            assert_eq!(
                bits(&six_left_io),
                bits(&four_left_io),
                "{width} dual left output differs from the direct four-band oracle, case={case}, seeded={seeded}"
            );
            assert_eq!(
                bits(&six_right_io),
                bits(&four_right_io),
                "{width} dual right output differs from the direct four-band oracle, case={case}, seeded={seeded}"
            );
            assert_eq!(
                original_state_bits(&core::array::from_fn(|band| {
                    six_left.sections[BAND_SECTION_OFFSET + band]
                })),
                original_state_bits(&four_left),
                "{width} dual left original-band state differs from the direct four-band oracle, case={case}, seeded={seeded}"
            );
            assert_eq!(
                original_state_bits(&core::array::from_fn(|band| {
                    six_right.sections[BAND_SECTION_OFFSET + band]
                })),
                original_state_bits(&four_right),
                "{width} dual right original-band state differs from the direct four-band oracle, case={case}, seeded={seeded}"
            );
        }
    }

    /// A signed-zero input refuses elision and therefore follows the established full six-section
    /// per-section path. The direct four-section oracle remains the compatibility authority for
    /// settled disabled cuts; this six-versus-six comparison checks only that refusal selects the
    /// corrected schedule consistently. Compare the stationary refusal with the nonstationary
    /// reference at every width and for both dual and collapsed mono entry points.
    fn compare_signed_zero_refusal<L: Lane, const W: usize>(width: &str, mono: bool, plane: usize) {
        let mut left = block::<W>(0, 0);
        let mut right = block::<W>(0, 3);
        if plane == 0 {
            left[0] = -0.0;
        } else {
            right[0] = -0.0;
        }
        let kept = if mono {
            let channel = channel::<L, W>(0);
            cascade_sections_mono(&channel, &left, FRAMES).1
        } else {
            let left_channel = channel::<L, W>(0);
            let right_channel = channel::<L, W>(3);
            cascade_sections(&left_channel, &right_channel, &left, &right, FRAMES).1
        };
        assert_eq!(
            kept,
            EQ_SECTION_COUNT,
            "{width} {mode} plane={plane}: -0.0 must refuse the shortened cascade",
            mode = if mono { "mono" } else { "dual" }
        );

        let mut stationary_left_channel = channel::<L, W>(0);
        let mut stationary_right_channel = channel::<L, W>(3);
        let mut reference_left_channel = channel::<L, W>(0);
        let mut reference_right_channel = channel::<L, W>(3);
        let mut stationary_left = left.clone();
        let mut stationary_right = right.clone();
        let mut reference_left = left;
        let mut reference_right = right;
        if mono {
            process_channels_mono(
                &mut stationary_left_channel,
                &mut stationary_left,
                FRAMES,
                true,
            );
            process_channels_mono(
                &mut reference_left_channel,
                &mut reference_left,
                FRAMES,
                false,
            );
            assert_eq!(
                bits(&stationary_left),
                bits(&reference_left),
                "{width} mono signed-zero refusal changed audio"
            );
            assert_eq!(
                integrators(&stationary_left_channel, &stationary_right_channel),
                integrators(&reference_left_channel, &reference_right_channel),
                "{width} mono signed-zero refusal changed state"
            );
        } else {
            process_channels(
                (&mut stationary_left_channel, &mut stationary_right_channel),
                &mut stationary_left,
                &mut stationary_right,
                FRAMES,
                true,
            );
            process_channels(
                (&mut reference_left_channel, &mut reference_right_channel),
                &mut reference_left,
                &mut reference_right,
                FRAMES,
                false,
            );
            assert_eq!(
                bits(&stationary_left),
                bits(&reference_left),
                "{width} dual left signed-zero refusal changed audio"
            );
            assert_eq!(
                bits(&stationary_right),
                bits(&reference_right),
                "{width} dual right signed-zero refusal changed audio"
            );
            assert_eq!(
                integrators(&stationary_left_channel, &stationary_right_channel),
                integrators(&reference_left_channel, &reference_right_channel),
                "{width} dual signed-zero refusal changed state"
            );
        }
    }

    /// Runs one corpus case at one width down both arms and asserts they are the same bits.
    ///
    /// `seed_state` puts a non-zero, subnormal-adjacent state into every section first, so the
    /// comparison covers the D7 flush inside the interleaved body and not only a cold start.
    fn compare<L: Lane, const W: usize>(width: &str, case: usize, seed_state: bool) {
        let mut arms = Vec::new();
        for stationary in [true, false] {
            let mut left_channel = channel::<L, W>(0);
            let mut right_channel = channel::<L, W>(3);
            if seed_state {
                for section in 0..EQ_SECTION_COUNT {
                    left_channel.sections[section].state.ic1 = L::splat(1.0e-40);
                    left_channel.sections[section].state.ic2 = L::splat(-1.0e-41);
                    right_channel.sections[section].state.ic1 = L::splat(-3.5e-7);
                    right_channel.sections[section].state.ic2 = L::splat(9.0e-8);
                }
            }
            let mut left = block::<W>(case, 0);
            let mut right = block::<W>(case, 3);
            process_channels(
                (&mut left_channel, &mut right_channel),
                &mut left,
                &mut right,
                FRAMES,
                stationary,
            );
            arms.push((left, right, integrators(&left_channel, &right_channel)));
        }
        let (interleaved, sequential) = (&arms[0], &arms[1]);
        assert_eq!(
            bits(&interleaved.0),
            bits(&sequential.0),
            "#163 phase 3: interleaved left channel differs at {width}, case {case}, \
             seeded={seed_state}"
        );
        assert_eq!(
            bits(&interleaved.1),
            bits(&sequential.1),
            "#163 phase 3: interleaved right channel differs at {width}, case {case}, \
             seeded={seed_state}"
        );
        assert_eq!(
            interleaved.2, sequential.2,
            "#163 phase 3: interleaved integrators differ at {width}, case {case}, \
             seeded={seed_state}"
        );
        // Non-vacuity: the arms must have filtered something, the channels must differ from each
        // other, and nothing may have left the finite range.
        assert_ne!(
            bits(&interleaved.0),
            bits(&block::<W>(case, 0)),
            "#163 phase 3: the case must actually filter at {width}"
        );
        assert_ne!(
            bits(&interleaved.0),
            bits(&interleaved.1),
            "#163 phase 3: the two channels must carry different audio at {width}"
        );
        assert!(
            interleaved.0.iter().all(|sample| sample.is_finite()),
            "#163 phase 3: a corpus case must stay finite at {width}"
        );
    }

    #[test]
    fn the_interleaved_cascade_renders_the_per_section_path_bit_for_bit() {
        // Case 1 of the corpus is the ramped one, and a ramp never reaches the interleaved arm --
        // its caller passes `stationary = false` -- so the stationary cases are 0 and 2.
        for case in [0_usize, 2] {
            for seeded in [false, true] {
                compare::<f32, 1>("Scalar", case, seeded);
                compare::<Simd4, 4>("Simd4", case, seeded);
                compare::<Simd8, 8>("Simd8", case, seeded);
            }
        }
    }

    #[test]
    fn prepared_hpf_and_last_lpf_are_reached_at_every_backend_width() {
        fn run<L: Lane, const W: usize>() {
            for (section, kind, label) in [
                (HPF_SECTION, EqBandKind::HighPass, "HPF"),
                (LPF_SECTION, EqBandKind::LowPass, "LPF"),
            ] {
                let mut targets = corpus::sections(0);
                for target in &mut targets {
                    target.enabled = false;
                }
                targets[section] = BandTarget {
                    enabled: true,
                    kind,
                    frequency: 1_000.0,
                    gain: 0.0,
                    q: 1.0,
                    slope: 1.0,
                };
                let targets = [targets; W];
                let mut left_channel =
                    Channel::<L, W>::new(targets, corpus::CORPUS_RATE).expect("cut design");
                let mut right_channel =
                    Channel::<L, W>::new(targets, corpus::CORPUS_RATE).expect("cut design");
                let mut left = vec![0.0_f32; corpus::FRAMES * W];
                let mut right = vec![0.0_f32; corpus::FRAMES * W];
                for lane in 0..W {
                    left[lane] = 1.0;
                    right[lane] = 1.0;
                }
                let source = left.clone();
                process_channels(
                    (&mut left_channel, &mut right_channel),
                    &mut left,
                    &mut right,
                    corpus::FRAMES,
                    true,
                );
                assert!(
                    left.iter()
                        .zip(source)
                        .any(|(actual, input)| { actual.to_bits() != input.to_bits() }),
                    "the prepared {label} must affect stationary output at width {W}"
                );
                assert!(
                    !section_state_is_positive_zero(&left_channel.sections[section]),
                    "the prepared {label} must retain state at width {W}"
                );
            }
        }

        run::<f32, 1>();
        run::<Simd4, 4>();
        run::<Simd8, 8>();
    }

    /// Builds a cut-only matrix. The four original bands are disabled so each scalar lane can
    /// independently exercise neither, HPF-only, LPF-only, or both cuts; the two channels use
    /// different masks and cutoff/Q words. The enabled LPF is deliberately nontrivial.
    fn mixed_cut_channel<L: Lane, const W: usize>(
        statuses: [u8; W],
        offset: usize,
        right: bool,
    ) -> Channel<L, W> {
        let targets: [[BandTarget; EQ_SECTION_COUNT]; W] = core::array::from_fn(|lane| {
            let track = (offset + lane) % corpus::LANES;
            let mut sections = corpus::sections(track);
            for section in &mut sections[BAND_SECTION_OFFSET..BAND_SECTION_OFFSET + EQ_BAND_COUNT] {
                section.enabled = false;
            }
            let status = statuses[lane];
            sections[HPF_SECTION] = BandTarget {
                enabled: status & 1 != 0,
                kind: EqBandKind::HighPass,
                frequency: 180.0 + track as f32 * 97.0 + if right { 31.0 } else { 0.0 },
                gain: 0.0,
                q: 0.35 + track as f32 * 0.17,
                slope: 1.0,
            };
            sections[LPF_SECTION] = BandTarget {
                enabled: status & 2 != 0,
                kind: EqBandKind::LowPass,
                frequency: 6_000.0 + track as f32 * 1_100.0 + if right { 47.0 } else { 0.0 },
                gain: 0.0,
                q: 0.55 + track as f32 * 0.11,
                slope: 1.0,
            };
            sections
        });
        Channel::new(targets, corpus::CORPUS_RATE).expect("mixed cut design")
    }

    fn channel_lane_state_bits<L: Lane, const W: usize>(
        channel: &Channel<L, W>,
        lane: usize,
    ) -> Vec<u32> {
        let mut words = [0_u32; EQ_SECTION_COUNT * 2 * MAX_LANES];
        channel.state_bits(&mut words);
        let mut out = Vec::with_capacity(EQ_SECTION_COUNT * 2);
        for section in 0..EQ_SECTION_COUNT {
            let base = section * 2 * W;
            out.push(words[base + lane]);
            out.push(words[base + W + lane]);
        }
        out
    }

    fn mixed_cut_scalar_parity<L: Lane, const W: usize>(width: &str) {
        let left_statuses = core::array::from_fn(|lane| [0_u8, 1, 2, 3, 3, 2, 1, 0][lane]);
        let right_statuses = core::array::from_fn(|lane| [3_u8, 2, 1, 0, 1, 3, 0, 2][lane]);
        let mut bank_left = mixed_cut_channel::<L, W>(left_statuses, 0, false);
        let mut bank_right = mixed_cut_channel::<L, W>(right_statuses, 3, true);
        let mut scalar_left: [Channel<f32, 1>; W] = core::array::from_fn(|lane| {
            mixed_cut_channel::<f32, 1>([left_statuses[lane]], lane, false)
        });
        let mut scalar_right: [Channel<f32, 1>; W] = core::array::from_fn(|lane| {
            mixed_cut_channel::<f32, 1>([right_statuses[lane]], lane + 3, true)
        });
        const FRAMES: usize = 11;
        let mut left = vec![0.0_f32; FRAMES * W];
        let mut right = vec![0.0_f32; FRAMES * W];
        for frame in 0..FRAMES {
            for lane in 0..W {
                let sample = 0.000_001 * (1.0 + frame as f32 * 0.25 + lane as f32 * 0.5);
                left[frame * W + lane] = sample;
                right[frame * W + lane] = -sample * 0.7;
            }
        }
        let source = left.clone();
        let right_source = right.clone();
        process_channels(
            (&mut bank_left, &mut bank_right),
            &mut left,
            &mut right,
            FRAMES,
            true,
        );

        for lane in 0..W {
            let mut scalar_left_io = (0..FRAMES)
                .map(|frame| source[frame * W + lane])
                .collect::<Vec<_>>();
            let mut scalar_right_io = (0..FRAMES)
                .map(|frame| right_source[frame * W + lane])
                .collect::<Vec<_>>();
            process_channels(
                (&mut scalar_left[lane], &mut scalar_right[lane]),
                &mut scalar_left_io,
                &mut scalar_right_io,
                FRAMES,
                true,
            );
            for frame in 0..FRAMES {
                assert_eq!(
                    left[frame * W + lane].to_bits(),
                    scalar_left_io[frame].to_bits(),
                    "{width} mixed cut left lane {lane} frame {frame}; bank={:#010x} scalar={:#010x}",
                    left[frame * W + lane].to_bits(),
                    scalar_left_io[frame].to_bits()
                );
                assert_eq!(
                    right[frame * W + lane].to_bits(),
                    scalar_right_io[frame].to_bits(),
                    "{width} mixed cut right lane {lane} frame {frame}"
                );
            }
            assert_eq!(
                channel_lane_state_bits(&bank_left, lane),
                channel_lane_state_bits(&scalar_left[lane], 0),
                "{width} mixed cut left state lane {lane}"
            );
            assert_eq!(
                channel_lane_state_bits(&bank_right, lane),
                channel_lane_state_bits(&scalar_right[lane], 0),
                "{width} mixed cut right state lane {lane}"
            );
        }
        // At least one enabled final LPF must have changed the signal; this catches a dropped
        // remainder section even when all scalar lanes happen to agree with their bank lane.
        assert!(
            (0..W).any(|lane| right_statuses[lane] & 2 != 0
                && (0..FRAMES).any(|frame| {
                    right[frame * W + lane].to_bits() != right_source[frame * W + lane].to_bits()
                })),
            "{width} mixed cut matrix must execute its nontrivial final LPF"
        );
        // One scalar track after a complete bank is the corresponding tail shape.
        let mut tail_channel = mixed_cut_channel::<f32, 1>([3], W, false);
        let mut tail = (0..FRAMES)
            .map(|frame| 0.000_001 * (1.0 + frame as f32 * 0.25 + W as f32 * 0.5))
            .collect::<Vec<_>>();
        let tail_source = tail.clone();
        process_channels_mono(&mut tail_channel, &mut tail, FRAMES, true);
        assert!(
            tail.iter()
                .zip(tail_source)
                .any(|(output, input)| output.to_bits() != input.to_bits()),
            "{width} scalar tail must execute both dedicated cuts"
        );
    }

    #[test]
    fn mixed_dedicated_cut_masks_match_scalar_lanes_bit_for_bit() {
        mixed_cut_scalar_parity::<Simd4, 4>("Simd4");
        mixed_cut_scalar_parity::<Simd8, 8>("Simd8");
    }

    /// A general-band ramp must keep the old four-band oracle in lockstep while dedicated cuts stay
    /// disabled. The six-section consumer is explicitly forced onto the fallback by `remaining`.
    fn disabled_cuts_general_ramp_parity<L: Lane, const W: usize>(width: &str) {
        const FRAMES: usize = 80;
        const RAMP_SECTION: usize = BAND_SECTION_OFFSET + 1;
        let mut six_left = channel::<L, W>(0);
        let mut six_right = channel::<L, W>(3);
        let mut four_left = original_four::<L, W>(0);
        let mut four_right = original_four::<L, W>(3);
        for lane in 0..W {
            let left_target = corpus::bands((lane + 3) % corpus::LANES)[1]
                .words(corpus::CORPUS_RATE)
                .expect("ramp target");
            let right_target = corpus::bands((lane + 6) % corpus::LANES)[1]
                .words(corpus::CORPUS_RATE)
                .expect("ramp target");
            six_left.start_ramp(RAMP_SECTION, lane, left_target);
            six_right.start_ramp(RAMP_SECTION, lane, right_target);
        }
        four_left[1] = six_left.sections[RAMP_SECTION];
        four_right[1] = six_right.sections[RAMP_SECTION];
        let mut six_left_io = vec![0.0_f32; FRAMES * W];
        let mut six_right_io = vec![0.0_f32; FRAMES * W];
        for frame in 0..FRAMES {
            for lane in 0..W {
                let sample = 0.02 + frame as f32 * 0.0003 + lane as f32 * 0.004;
                six_left_io[frame * W + lane] = sample;
                six_right_io[frame * W + lane] = -sample * 0.8;
            }
        }
        let mut four_left_io = six_left_io.clone();
        let mut four_right_io = six_right_io.clone();
        assert!(!six_left.no_ramp_in_flight() || !six_right.no_ramp_in_flight());
        process_channels(
            (&mut six_left, &mut six_right),
            &mut six_left_io,
            &mut six_right_io,
            FRAMES,
            false,
        );
        process_original_four_ramped::<L, W>(&mut four_left, &mut four_left_io, FRAMES);
        process_original_four_ramped::<L, W>(&mut four_right, &mut four_right_io, FRAMES);
        assert_eq!(
            bits(&six_left_io),
            bits(&four_left_io),
            "{width} ramped old-four left output"
        );
        assert_eq!(
            bits(&six_right_io),
            bits(&four_right_io),
            "{width} ramped old-four right output"
        );
        assert_eq!(
            original_state_bits(&core::array::from_fn(|band| {
                six_left.sections[BAND_SECTION_OFFSET + band]
            })),
            original_state_bits(&four_left),
            "{width} ramped old-four left original state"
        );
        assert_eq!(
            original_state_bits(&core::array::from_fn(|band| {
                six_right.sections[BAND_SECTION_OFFSET + band]
            })),
            original_state_bits(&four_right),
            "{width} ramped old-four right original state"
        );
    }

    #[test]
    fn disabled_cuts_with_an_original_band_ramp_match_the_four_section_oracle() {
        disabled_cuts_general_ramp_parity::<f32, 1>("Scalar");
        disabled_cuts_general_ramp_parity::<Simd4, 4>("Simd4");
        disabled_cuts_general_ramp_parity::<Simd8, 8>("Simd8");
    }

    #[test]
    fn resident_sizes_are_measured_separately_from_serialized_state() {
        println!(
            "resident_size Channel width=1 bytes={}",
            core::mem::size_of::<Channel<f32, 1>>()
        );
        println!(
            "resident_size Channel width=4 bytes={}",
            core::mem::size_of::<Channel<Simd4, 4>>()
        );
        println!(
            "resident_size Channel width=8 bytes={}",
            core::mem::size_of::<Channel<Simd8, 8>>()
        );
        println!(
            "resident_size PreparedParametricEq width=1 bytes={}",
            core::mem::size_of::<PreparedParametricEq<f32, 1>>()
        );
        println!(
            "resident_size PreparedParametricEq width=4 bytes={}",
            core::mem::size_of::<PreparedParametricEq<Simd4, 4>>()
        );
        println!(
            "resident_size PreparedParametricEq width=8 bytes={}",
            core::mem::size_of::<PreparedParametricEq<Simd8, 8>>()
        );
        println!("serialized_state_bytes total={}", STATE_SIZES.total());
    }

    #[test]
    fn a_tiny_restored_disabled_cut_state_refuses_elision_but_preserves_old_bands() {
        for width in [1_usize, 4, 8] {
            match width {
                1 => tiny_disabled_cut_state::<f32, 1>("Scalar"),
                4 => tiny_disabled_cut_state::<Simd4, 4>("Simd4"),
                _ => tiny_disabled_cut_state::<Simd8, 8>("Simd8"),
            }
        }
    }

    fn tiny_disabled_cut_state<L: Lane, const W: usize>(width: &str) {
        let mut six_left = channel::<L, W>(0);
        let mut six_right = channel::<L, W>(3);
        let mut four_left = original_four::<L, W>(0);
        six_left.sections[HPF_SECTION].state.ic1 = L::splat(1.0e-30);
        let mut left = vec![0.000_001_f32; 9 * W];
        let mut right = vec![-0.000_002_f32; 9 * W];
        let mut old_left = left.clone();
        assert_eq!(
            cascade_sections(&six_left, &six_right, &left, &right, 9).1,
            EQ_SECTION_COUNT,
            "{width} restored tiny disabled HPF state must refuse elision"
        );
        process_channels(
            (&mut six_left, &mut six_right),
            &mut left,
            &mut right,
            9,
            true,
        );
        process_original_four(&mut four_left, &mut old_left, 9);
        assert_eq!(
            bits(&left),
            bits(&old_left),
            "{width} tiny HPF state left output"
        );
        assert_eq!(
            original_state_bits(&core::array::from_fn(|band| {
                six_left.sections[BAND_SECTION_OFFSET + band]
            })),
            original_state_bits(&four_left),
            "{width} tiny HPF state left original state"
        );
        let hpf_state = channel_lane_state_bits(&six_left, 0);
        assert_eq!(
            hpf_state[0], 0,
            "{width} executed disabled HPF flushes its restored tiny state"
        );
    }

    #[test]
    fn disabled_cuts_match_the_full_six_section_reference() {
        for width in [1_usize, 4, 8] {
            match width {
                1 => compare::<f32, 1>("Scalar-disabled-cuts", 0, false),
                4 => compare::<Simd4, 4>("Simd4-disabled-cuts", 0, false),
                _ => compare::<Simd8, 8>("Simd8-disabled-cuts", 0, false),
            }
        }
    }

    #[test]
    fn disabled_cuts_preserve_original_band_bits_against_a_direct_four_section_oracle() {
        for seeded in [false, true] {
            for refusal_state in [false, true] {
                for negative_zero in [false, true] {
                    for case in [0_usize, 2] {
                        for mono in [false, true] {
                            compare_disabled_cuts_to_original_four::<f32, 1>(
                                "Scalar",
                                case,
                                seeded,
                                mono,
                                refusal_state,
                                negative_zero,
                            );
                            compare_disabled_cuts_to_original_four::<Simd4, 4>(
                                "Simd4",
                                case,
                                seeded,
                                mono,
                                refusal_state,
                                negative_zero,
                            );
                            compare_disabled_cuts_to_original_four::<Simd8, 8>(
                                "Simd8",
                                case,
                                seeded,
                                mono,
                                refusal_state,
                                negative_zero,
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn disabled_cuts_preserve_all_high_pass_signed_zero_against_four_section_oracle() {
        let mut prepared = all_high_pass_channel::<f32, 1>();
        let mut oracle = core::array::from_fn(|band| prepared.sections[BAND_SECTION_OFFSET + band]);
        let mut prepared_io = vec![-0.0_f32];
        let mut oracle_io = prepared_io.clone();

        assert_eq!(
            cascade_sections_mono(&prepared, &prepared_io, 1).1,
            EQ_SECTION_COUNT,
            "signed-zero input must retain the full prepared schedule while testing the oracle"
        );
        process_channels_mono(&mut prepared, &mut prepared_io, 1, true);
        process_original_four(&mut oracle, &mut oracle_io, 1);

        assert_eq!(
            prepared_io[0].to_bits(),
            oracle_io[0].to_bits(),
            "all-HighPass signed-zero compatibility mismatch: prepared={:#010x}, four={:#010x}",
            prepared_io[0].to_bits(),
            oracle_io[0].to_bits()
        );
    }

    #[test]
    fn signed_zero_refusal_preserves_the_full_six_section_bits() {
        for mono in [false, true] {
            for plane in [0_usize, 1] {
                if mono && plane == 1 {
                    continue;
                }
                compare_signed_zero_refusal::<f32, 1>("Scalar", mono, plane);
                compare_signed_zero_refusal::<Simd4, 4>("Simd4", mono, plane);
                compare_signed_zero_refusal::<Simd8, 8>("Simd8", mono, plane);
            }
        }
    }

    /// Six physical sections use the same effective stationary depth-two pass on every backend.
    #[test]
    fn the_tuned_depth_is_per_backend_and_divides_the_cascade() {
        assert_eq!(EQ_SECTION_COUNT % EFFECTIVE_CASCADE_DEPTH, 0);
        assert_eq!(EFFECTIVE_CASCADE_DEPTH, 2);
    }
}

/// Identity-section elision: the shortened cascade is the full cascade, bit for bit.
///
/// The oracle is the one [`interleave_identity`] already uses and for the same reason: the
/// `stationary = false` arm of [`process_channels`] is [`Channel::process_block`], which runs all
/// six sections through the per-section path and knows nothing about elision. So an equality
/// against it is an equality against the code the EQ ran before this existed, not against a
/// re-transcription of it -- and no runtime knob is added to reach either arm.
///
/// What is covered: every live/dead subset of the six sections, both channels agreeing and
/// disagreeing, at all three widths, cold and with seeded subnormal-adjacent state; the three
/// refusal legs of the gate (`-0.0` input, a non-`+0.0` state in an elided section, a `-0.0`
/// integrator in a live one) and the magnitude ceiling; per-lane disagreement inside one section;
/// and enable/disable transitions arriving mid-session through the ramp path.
#[cfg(test)]
mod elision {
    use super::{
        BandTarget, Channel, ELISION_MAGNITUDE_CEILING, EQ_SECTION_COUNT, EqBandKind, EqSvfWords,
        HPF_SECTION, INERT_MAGNITUDE_FLOOR, LPF_SECTION, MAX_LANES, RAMP_SAMPLES, STATE_LANE_WORDS,
        STATE_WORDS_PER_BAND, block_admits_elision, block_admits_elision_oracle, cascade_sections,
        cascade_sections_mono, corpus, masked_pair_pass_count, process_channels,
        process_channels_mono, reset_masked_pair_passes, reset_select_free_tail_passes,
        select_free_tail_pass_count,
    };
    use lane::{Lane, Simd4, Simd8};

    const FRAMES: usize = corpus::FRAMES;
    /// Every subset of the six cascade positions, as a bitmask of *live* sections.
    const MASKS: core::ops::Range<u8> = 0..(1 << EQ_SECTION_COUNT);

    /// A channel whose section `s` is a real corpus band when `live` has bit `s` set, and the
    /// exact identity when it does not.
    ///
    /// Disabling is expressed through `BandTarget::enabled`, which is the production route: it is
    /// what `BandTarget::words` maps to [`EqSvfWords::IDENTITY`], so the test is exercising the
    /// same words a session with a disabled band prepares.
    fn channel<L: Lane, const W: usize>(offset: usize, live: u8) -> Channel<L, W> {
        let targets: [[BandTarget; EQ_SECTION_COUNT]; W] = core::array::from_fn(|lane| {
            let mut bands = corpus::sections((offset + lane) % corpus::LANES);
            for (section, band) in bands.iter_mut().enumerate() {
                band.enabled = live & (1 << section) != 0;
            }
            bands
        });
        Channel::new(targets, corpus::CORPUS_RATE).expect("every corpus row is a legal design")
    }

    fn block<const W: usize>(case: usize, offset: usize) -> Vec<f32> {
        let mut lanes = vec![[0.0_f32; FRAMES]; W];
        for (index, lane) in lanes.iter_mut().enumerate() {
            corpus::fill(case, (offset + index) % corpus::LANES, lane);
        }
        let mut out = vec![0.0_f32; FRAMES * W];
        for frame in 0..FRAMES {
            for (index, lane) in lanes.iter().enumerate() {
                out[frame * W + index] = lane[frame];
            }
        }
        out
    }

    fn integrators<L: Lane, const W: usize>(
        left: &Channel<L, W>,
        right: &Channel<L, W>,
    ) -> Vec<u32> {
        let mut words = [0_u32; EQ_SECTION_COUNT * 2 * MAX_LANES];
        let mut out = Vec::new();
        left.state_bits(&mut words);
        out.extend_from_slice(&words);
        right.state_bits(&mut words);
        out.extend_from_slice(&words);
        out
    }

    fn bits(samples: &[f32]) -> Vec<u32> {
        samples.iter().map(|sample| sample.to_bits()).collect()
    }

    /// How many sections the elision gate would run for this pair over this block.
    fn kept<L: Lane, const W: usize>(
        left_channel: &Channel<L, W>,
        right_channel: &Channel<L, W>,
        left: &[f32],
        right: &[f32],
    ) -> usize {
        cascade_sections::<L, W>(left_channel, right_channel, left, right, FRAMES).1
    }

    /// Runs one case down both arms with the given live masks and asserts bit equality of the
    /// rendered audio and of every integrator word, elided sections included.
    ///
    /// Returns the number of sections the elided arm actually ran, so a caller can assert that a
    /// configuration engaged rather than quietly falling back.
    fn compare<L: Lane, const W: usize>(
        width: &str,
        case: usize,
        left_live: u8,
        right_live: u8,
        seed_state: bool,
    ) -> usize {
        let mut arms = Vec::new();
        let mut ran = EQ_SECTION_COUNT;
        for stationary in [true, false] {
            let mut left_channel = channel::<L, W>(0, left_live);
            let mut right_channel = channel::<L, W>(3, right_live);
            if seed_state {
                for section in 0..EQ_SECTION_COUNT {
                    // Only *live* sections are seeded: a non-inert state in a dead section is a
                    // refusal leg with its own test, and seeding it here would silently disable
                    // the very engagement this function is asserting. The seeds are words the
                    // kernel can write, just above `FLUSH_EPS`: since issue #1015 a restored
                    // subnormal in a live section refuses too (`stationary_subnormal`).
                    if left_live & (1 << section) != 0 {
                        left_channel.sections[section].state.ic1 = L::splat(1.5e-20);
                        left_channel.sections[section].state.ic2 = L::splat(-1.25e-20);
                    }
                    if right_live & (1 << section) != 0 {
                        right_channel.sections[section].state.ic1 = L::splat(-3.5e-7);
                        right_channel.sections[section].state.ic2 = L::splat(9.0e-8);
                    }
                }
            }
            let mut left = block::<W>(case, 0);
            let mut right = block::<W>(case, 3);
            if stationary {
                ran = kept(&left_channel, &right_channel, &left, &right);
            }
            process_channels(
                (&mut left_channel, &mut right_channel),
                &mut left,
                &mut right,
                FRAMES,
                stationary,
            );
            arms.push((left, right, integrators(&left_channel, &right_channel)));
        }
        let (elided, full) = (&arms[0], &arms[1]);
        let label = format!(
            "{width}, case {case}, live {left_live:04b}/{right_live:04b}, seeded={seed_state}"
        );
        assert_eq!(
            bits(&elided.0),
            bits(&full.0),
            "elided left channel differs from the full cascade at {label}"
        );
        assert_eq!(
            bits(&elided.1),
            bits(&full.1),
            "elided right channel differs from the full cascade at {label}"
        );
        assert_eq!(
            elided.2, full.2,
            "elided integrators differ from the full cascade at {label}"
        );
        assert!(
            elided
                .0
                .iter()
                .chain(elided.1.iter())
                .all(|s| s.is_finite()),
            "an elided case must stay finite at {label}"
        );
        ran
    }

    /// [`compare`] for the collapsed body: one channel through [`process_channels_mono`], against
    /// the same channel's per-section path.
    fn compare_mono<L: Lane, const W: usize>(
        width: &str,
        case: usize,
        live: u8,
        seed_state: bool,
    ) -> usize {
        let mut arms = Vec::new();
        let mut ran = EQ_SECTION_COUNT;
        for stationary in [true, false] {
            let mut channel = channel::<L, W>(0, live);
            if seed_state {
                for section in 0..EQ_SECTION_COUNT {
                    if live & (1 << section) != 0 {
                        channel.sections[section].state.ic1 = L::splat(1.5e-20);
                        channel.sections[section].state.ic2 = L::splat(-1.25e-20);
                    }
                }
            }
            let mut io = block::<W>(case, 0);
            if stationary {
                ran = cascade_sections_mono::<L, W>(&channel, &io, FRAMES).1;
            }
            process_channels_mono(&mut channel, &mut io, FRAMES, stationary);
            let mut words = [0_u32; EQ_SECTION_COUNT * 2 * MAX_LANES];
            channel.state_bits(&mut words);
            arms.push((io, words));
        }
        let (elided, full) = (&arms[0], &arms[1]);
        let label = format!("{width} mono, case {case}, live {live:06b}, seeded={seed_state}");
        assert_eq!(
            bits(&elided.0),
            bits(&full.0),
            "elided mono output differs from the full cascade at {label}"
        );
        assert_eq!(
            elided.1, full.1,
            "elided mono integrators differ from the full cascade at {label}"
        );
        assert!(
            elided.0.iter().all(|s| s.is_finite()),
            "an elided mono case must stay finite at {label}"
        );
        ran
    }

    /// Every live/dead subset, both channels, all three widths, cold and seeded; dual and
    /// collapsed mono.
    ///
    /// Issue #976: the elided list is exactly the live sections -- no identity section is kept as
    /// padding -- so an odd live count runs its last section in a depth-one pass. Both corpus
    /// cases are admissible (no `-0.0`, nothing above the ceiling), so every mask elides down to
    /// its live count.
    #[test]
    fn an_elided_cascade_is_the_full_cascade_bit_for_bit() {
        for case in [0_usize, 2] {
            for seeded in [false, true] {
                for live in MASKS {
                    let count = live.count_ones() as usize;
                    let ran = [
                        compare::<f32, 1>("Scalar", case, live, live, seeded),
                        compare::<Simd4, 4>("Simd4", case, live, live, seeded),
                        compare::<Simd8, 8>("Simd8", case, live, live, seeded),
                    ];
                    assert_eq!(
                        ran, [count; 3],
                        "the dual cascade keeps exactly the live sections ({live:06b})"
                    );
                    let ran = [
                        compare_mono::<f32, 1>("Scalar", case, live, seeded),
                        compare_mono::<Simd4, 4>("Simd4", case, live, seeded),
                        compare_mono::<Simd8, 8>("Simd8", case, live, seeded),
                    ];
                    assert_eq!(
                        ran, [count; 3],
                        "the mono cascade keeps exactly the live sections ({live:06b})"
                    );
                }
            }
        }
    }

    /// An odd last live section with no dry lane runs the depth-one pass without the dry select.
    ///
    /// Issue #976 M3: rendered bits cannot see which arm the tail took -- a select whose mask is
    /// empty returns the wet word -- so this counter is the only gate on the performance half of
    /// the tail rule. A general band is never dry, so one live band is an odd tail with empty masks.
    #[test]
    fn an_odd_tail_without_dry_lanes_skips_the_select() {
        fn run<L: Lane, const W: usize>(width: &str) {
            // One live general band (physical section 2), and three: sections 1, 2 and 4.
            for (live, pairs) in [(0b00_0100_u8, 0_usize), (0b01_0110, 1)] {
                reset_select_free_tail_passes();
                let mut left_channel = channel::<L, W>(0, live);
                let mut right_channel = channel::<L, W>(3, live);
                let mut left = block::<W>(0, 0);
                let mut right = block::<W>(0, 3);
                process_channels(
                    (&mut left_channel, &mut right_channel),
                    &mut left,
                    &mut right,
                    FRAMES,
                    true,
                );
                assert_eq!(
                    select_free_tail_pass_count(),
                    1,
                    "{width} dual {live:06b}: {pairs} pair(s), then one select-free tail pass"
                );
                process_channels_mono(&mut left_channel, &mut left, FRAMES, true);
                assert_eq!(
                    select_free_tail_pass_count(),
                    2,
                    "{width} mono {live:06b}: {pairs} pair(s), then one select-free tail pass"
                );
            }
        }
        run::<f32, 1>("Scalar");
        run::<Simd4, 4>("Simd4");
        run::<Simd8, 8>("Simd8");
    }

    /// The two channels are allowed to disagree about which sections are live, and a section is
    /// only elidable when it is identity on **both**.
    #[test]
    fn the_two_channels_are_judged_together() {
        for left_live in MASKS {
            for right_live in MASKS {
                let ran = compare::<Simd4, 4>("Simd4", 0, left_live, right_live, false);
                let live = (left_live | right_live).count_ones() as usize;
                assert_eq!(
                    ran, live,
                    "a section is elidable only when it is identity on both channels \
                     ({left_live:04b}/{right_live:04b})"
                );
            }
        }
    }

    /// Non-vacuity: the shapes this optimisation exists for really do shorten the cascade.
    ///
    /// One live band of four is the shipped console fixture's shape (see the intended
    /// sixty-four-track session), and it is the row the standing measurement moves.
    #[test]
    fn the_shipped_shape_actually_elides() {
        for (live, expected) in [(0b0001_u8, 1_usize), (0b0011, 2), (0b0000, 0)] {
            let left_channel = channel::<Simd8, 8>(0, live);
            let right_channel = channel::<Simd8, 8>(3, live);
            let left = block::<8>(0, 0);
            let right = block::<8>(0, 3);
            assert_eq!(
                kept(&left_channel, &right_channel, &left, &right),
                expected,
                "a bank with live mask {live:04b} should run {expected} of \
                 {EQ_SECTION_COUNT} sections"
            );
        }
        // Three live sections run three: one depth-two pass and a depth-one tail, with no identity
        // section kept as padding (issue #976).
        let left_channel = channel::<Simd8, 8>(0, 0b0111);
        let right_channel = channel::<Simd8, 8>(3, 0b0111);
        assert_eq!(
            kept(
                &left_channel,
                &right_channel,
                &block::<8>(0, 0),
                &block::<8>(0, 3)
            ),
            3,
            "three live sections run three, not four"
        );
    }

    /// A section that is identity on some lanes and live on others is not elidable.
    #[test]
    fn a_section_live_on_one_lane_is_not_elided() {
        let mut targets: [[BandTarget; EQ_SECTION_COUNT]; 8] =
            core::array::from_fn(corpus::sections);
        for bands in &mut targets {
            for band in bands.iter_mut() {
                band.enabled = false;
            }
        }
        // Lane 5 alone keeps section 2.
        targets[5][2].enabled = true;
        let left_channel =
            Channel::<Simd8, 8>::new(targets, corpus::CORPUS_RATE).expect("legal design");
        let right_channel = channel::<Simd8, 8>(3, 0b0000);
        let left = block::<8>(0, 0);
        let right = block::<8>(0, 3);
        assert_eq!(
            kept(&left_channel, &right_channel, &left, &right),
            1,
            "one live lane keeps its whole section, and only that section"
        );
        // And it renders the same bits as the full cascade.
        let ran = compare::<Simd8, 8>("Simd8", 0, 0b0100, 0b0000, false);
        assert_eq!(ran, 1, "the mixed-lane case must still engage");
    }

    /// A `-0.0` anywhere in either input plane refuses the elision.
    ///
    /// This is the leg the proof rests on: an elided identity section would rewrite that `-0.0` to
    /// `+0.0`, which is a moved bit.
    #[test]
    fn a_negative_zero_input_refuses_elision() {
        for plane in 0..2 {
            for position in [0_usize, 1, FRAMES * 8 - 1] {
                let left_channel = channel::<Simd8, 8>(0, 0b0001);
                let right_channel = channel::<Simd8, 8>(3, 0b0001);
                let mut left = block::<8>(0, 0);
                let mut right = block::<8>(0, 3);
                if plane == 0 {
                    left[position] = -0.0;
                } else {
                    right[position] = -0.0;
                }
                assert_eq!(
                    kept(&left_channel, &right_channel, &left, &right),
                    EQ_SECTION_COUNT,
                    "a -0.0 at word {position} of plane {plane} must refuse elision"
                );
                // A `+0.0` in the same place must not.
                let mut left = block::<8>(0, 0);
                let mut right = block::<8>(0, 3);
                if plane == 0 {
                    left[position] = 0.0;
                } else {
                    right[position] = 0.0;
                }
                assert_eq!(
                    kept(&left_channel, &right_channel, &left, &right),
                    1,
                    "a +0.0 at word {position} of plane {plane} is an ordinary sample"
                );
            }
        }
    }

    /// Non-finite input, and input above the §4.4 magnitude bound, refuse the elision.
    #[test]
    fn a_non_finite_or_oversized_input_refuses_elision() {
        for sample in [
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::NAN,
            1.0e31,
            -1.0e31,
            f32::MAX,
        ] {
            let left_channel = channel::<Simd8, 8>(0, 0b0001);
            let right_channel = channel::<Simd8, 8>(3, 0b0001);
            let mut left = block::<8>(0, 0);
            let right = block::<8>(0, 3);
            left[17] = sample;
            assert_eq!(
                kept(&left_channel, &right_channel, &left, &right),
                EQ_SECTION_COUNT,
                "{sample} must refuse elision"
            );
        }
        // The bound is inclusive on the legal side: 1e29 is an ordinary, if absurd, sample.
        let left_channel = channel::<Simd8, 8>(0, 0b0001);
        let right_channel = channel::<Simd8, 8>(3, 0b0001);
        let mut left = block::<8>(0, 0);
        let right = block::<8>(0, 3);
        left[17] = 1.0e29;
        assert_eq!(
            kept(&left_channel, &right_channel, &left, &right),
            1,
            "a large but admissible sample does not refuse elision"
        );
    }

    /// Issue #979 gate 3: a restored state in a section that *would* be elided refuses the
    /// elision unless it is inert, and either way the stationary path renders the per-section
    /// path's bits and integrators.
    ///
    /// The words go in through [`Channel::restore_track`], on one lane of dead section 3, in either
    /// integrator. Refused: `-0.0`, magnitudes below `FLUSH_EPS` (`1e-30`, the word under the floor),
    /// magnitudes above the ceiling (the word over it, `f32::MAX`). Admitted: `+-1.0`,
    /// `+-FLUSH_EPS` and `+-1e30`, the two ends of the inert band. Before issue #979 this test was
    /// `a_non_zero_state_in_a_dead_section_refuses_elision` and refused `1.0` too.
    #[test]
    fn a_non_inert_state_in_a_dead_section_refuses_elision() {
        let floor = f32::from_bits(INERT_MAGNITUDE_FLOOR);
        let ceiling = f32::from_bits(ELISION_MAGNITUDE_CEILING);
        let refused = [
            -0.0_f32,
            1.0e-30,
            -1.0e-30,
            f32::from_bits(INERT_MAGNITUDE_FLOOR - 1),
            f32::from_bits(ELISION_MAGNITUDE_CEILING + 1),
            -f32::from_bits(ELISION_MAGNITUDE_CEILING + 1),
            f32::MAX,
            -f32::MAX,
        ];
        let admitted = [1.0_f32, -1.0, floor, -floor, ceiling, -ceiling];
        for (word, admit) in refused
            .into_iter()
            .map(|word| (word, false))
            .chain(admitted.into_iter().map(|word| (word, true)))
        {
            for integrator in [0_usize, 1] {
                let label = format!("dead section 3 holding {word:e} in integrator {integrator}");
                let mut arms = Vec::new();
                let mut ran = EQ_SECTION_COUNT;
                for stationary in [true, false] {
                    let mut left_channel = channel::<Simd8, 8>(0, 0b0000_0010);
                    let mut right_channel = channel::<Simd8, 8>(3, 0b0000_0010);
                    let mut words = [0_u32; STATE_LANE_WORDS];
                    left_channel.snapshot_track(5, &mut words);
                    words[3 * STATE_WORDS_PER_BAND + integrator] = word.to_bits();
                    let configuration = left_channel.targets[5];
                    left_channel
                        .restore_track(5, &words, &configuration, corpus::CORPUS_RATE)
                        .expect("a finite integrator restores");
                    let mut left = block::<8>(0, 0);
                    let mut right = block::<8>(0, 3);
                    if stationary {
                        ran = kept(&left_channel, &right_channel, &left, &right);
                    }
                    process_channels(
                        (&mut left_channel, &mut right_channel),
                        &mut left,
                        &mut right,
                        FRAMES,
                        stationary,
                    );
                    arms.push((
                        bits(&left),
                        bits(&right),
                        integrators(&left_channel, &right_channel),
                    ));
                }
                // The bits first: under a mutation that admits a word the executed section would
                // move, the integrators are what differ.
                assert!(
                    arms[0] == arms[1],
                    "{label}: the stationary path must render the per-section path's bits and integrators"
                );
                if admit {
                    assert_eq!(ran, 1, "{label}: an inert state is admitted");
                } else {
                    assert_eq!(ran, EQ_SECTION_COUNT, "{label}: must refuse elision");
                }
            }
        }
    }

    /// Issue #979 gate 2: a band switched on and off again on every lane of both channels leaves
    /// its section at the identity with a frozen, non-zero state, and the bank keeps eliding.
    ///
    /// The transition goes through [`Channel::start_ramp`], the automation route: on at step 2,
    /// off at step 5, sixteen blocks, beside one live general band. Before issue #979 leg (b)
    /// refused every block after the disable (0 of the 9 counted below elided, VERIFY-EQ); the
    /// inert rule elides every stationary block from the first one after the ramp. Every block
    /// renders the per-section path's bits and integrators. Once for a general band (section 3)
    /// and once for the HPF (section 0), at every width.
    #[test]
    fn a_band_switched_off_keeps_the_bank_eliding() {
        fn run<L: Lane, const W: usize>(width: &str) {
            const BLOCKS: usize = 16;
            const ON: usize = 2;
            const OFF: usize = 5;
            for section in [3_usize, HPF_SECTION] {
                let words = |lane: usize, offset: usize| {
                    let track = (lane + offset) % corpus::LANES;
                    let target = if section == HPF_SECTION {
                        BandTarget {
                            enabled: true,
                            kind: EqBandKind::HighPass,
                            frequency: 40.0 + track as f32 * 23.0,
                            gain: 0.0,
                            q: 0.5 + track as f32 * 0.1,
                            slope: 1.0,
                        }
                    } else {
                        let mut band = corpus::sections(track)[section];
                        band.enabled = true;
                        band
                    };
                    target.words(corpus::CORPUS_RATE).expect("legal design")
                };
                let mut arms = Vec::new();
                let mut eliding_after = 0_usize;
                let mut first_after_elided = false;
                for elide in [true, false] {
                    let mut left_channel = channel::<L, W>(0, 0b0000_0010);
                    let mut right_channel = channel::<L, W>(3, 0b0000_0010);
                    let mut rendered: Vec<u32> = Vec::new();
                    for step in 0..BLOCKS {
                        for lane in 0..W {
                            if step == ON {
                                left_channel.start_ramp(section, lane, words(lane, 0));
                                right_channel.start_ramp(section, lane, words(lane, 3));
                            }
                            if step == OFF {
                                left_channel.start_ramp(section, lane, EqSvfWords::IDENTITY);
                                right_channel.start_ramp(section, lane, EqSvfWords::IDENTITY);
                            }
                        }
                        let mut left = block::<W>(0, step % corpus::LANES);
                        let mut right = block::<W>(0, (step + 3) % corpus::LANES);
                        let stationary =
                            left_channel.no_ramp_in_flight() && right_channel.no_ramp_in_flight();
                        if elide && step > OFF {
                            assert!(stationary, "{width} section {section}: the ramp has ended");
                            let elided = kept(&left_channel, &right_channel, &left, &right)
                                < EQ_SECTION_COUNT;
                            first_after_elided |= elided && step == OFF + 1;
                            eliding_after += usize::from(elided && step > OFF + 1);
                        }
                        process_channels(
                            (&mut left_channel, &mut right_channel),
                            &mut left,
                            &mut right,
                            FRAMES,
                            stationary && elide,
                        );
                        rendered.extend(bits(&left));
                        rendered.extend(bits(&right));
                    }
                    if elide {
                        assert!(
                            (0..EQ_SECTION_COUNT).all(|index| index == 1
                                || (left_channel.identity[index] && right_channel.identity[index])),
                            "{width} section {section}: switched off, it is the identity again"
                        );
                        let frozen = left_channel.sections[section].state;
                        assert!(
                            !(effect_runtime::bank::lane_is_positive_zero::<L>(frozen.ic1)
                                && effect_runtime::bank::lane_is_positive_zero::<L>(frozen.ic2)),
                            "{width} section {section}: non-vacuity, the section holds a frozen \
                             non-zero state"
                        );
                    }
                    rendered.extend(integrators(&left_channel, &right_channel));
                    arms.push(rendered);
                }
                println!(
                    "#979 cliff {width} section {section}: {eliding_after} of {} later blocks elide \
                     (first stationary block after the ramp: {first_after_elided})",
                    BLOCKS - OFF - 2
                );
                assert_eq!(
                    (eliding_after, first_after_elided),
                    (BLOCKS - OFF - 2, true),
                    "{width} section {section}: every stationary block after the switch-off elides"
                );
                assert!(
                    arms[0] == arms[1],
                    "{width} section {section}: the eliding arm must render the per-section path's \
                     bits and integrators"
                );
            }
        }
        run::<f32, 1>("Scalar");
        run::<Simd4, 4>("Simd4");
        run::<Simd8, 8>("Simd8");
    }

    /// A `-0.0` integrator in a *live* section refuses the elision.
    ///
    /// Nothing the kernel writes is ever `-0.0` -- `flush` maps every zero to `+0.0` -- but a
    /// restored state payload is admitted on finiteness alone, and a low-pass section carrying
    /// `ic2 = -0.0` is the one shape that can emit `-0.0` into a later elided section.
    #[test]
    fn a_negative_zero_state_in_a_live_section_refuses_elision() {
        for word in [0_usize, 1] {
            let mut left_channel = channel::<Simd8, 8>(0, 0b0001);
            let right_channel = channel::<Simd8, 8>(3, 0b0001);
            let mut lanes = [0.0_f32; 8];
            lanes[6] = -0.0;
            let seeded = Simd8::load(&lanes);
            if word == 0 {
                left_channel.sections[0].state.ic1 = seeded;
            } else {
                left_channel.sections[0].state.ic2 = seeded;
            }
            assert_eq!(
                kept(
                    &left_channel,
                    &right_channel,
                    &block::<8>(0, 0),
                    &block::<8>(0, 3)
                ),
                EQ_SECTION_COUNT,
                "a live section holding -0.0 in integrator {word} must refuse elision"
            );
        }
    }

    /// The word patterns of issue #980 gate 1: both zeros, the smallest subnormals, the ceiling and
    /// the word above it at either sign, both infinities, a quiet and a payload NaN, `+-1.0`, the
    /// largest magnitude pattern and the all-ones word.
    const GATE_EDGES: [u32; 16] = [
        0,
        0x8000_0000,
        1,
        0x8000_0001,
        ELISION_MAGNITUDE_CEILING,
        ELISION_MAGNITUDE_CEILING + 1,
        0x7f80_0000,
        0xff80_0000,
        0x7fc0_0000,
        0xffc0_1234,
        0x3f80_0000,
        0xbf80_0000,
        ELISION_MAGNITUDE_CEILING | 0x8000_0000,
        (ELISION_MAGNITUDE_CEILING + 1) | 0x8000_0000,
        0x7fff_ffff,
        0xffff_ffff,
    ];

    /// An ordinary admissible word of either sign for position `index`: finite, non-zero, well
    /// inside the ceiling, never `-0.0`.
    fn ordinary_word(index: usize) -> f32 {
        let magnitude = 0.001 + (index % 97) as f32 * 0.37;
        if index % 3 == 1 {
            -magnitude
        } else {
            magnitude
        }
    }

    /// Issue #980 gate 1: the min/max gate gives the rejection accumulator's verdict on every
    /// ordered pair of edge patterns, planted at two positions of a block of ordinary words, at
    /// every block length that exercises an empty block, a lone word, the scalar remainder and the
    /// vector body of both widths.
    #[test]
    fn the_min_max_gate_equals_the_rejection_oracle() {
        let mut verdicts = [0_usize; 2];
        for length in [0_usize, 1, 7, 8, 9, 1024] {
            let placements: &[(usize, usize)] = if length == 0 {
                &[(0, 0)]
            } else {
                &[(length / 3, length - 1), (0, length / 2)]
            };
            for &first in &GATE_EDGES {
                for &second in &GATE_EDGES {
                    for &(at_first, at_second) in placements {
                        let mut block: Vec<f32> = (0..length).map(ordinary_word).collect();
                        if length > 0 {
                            block[at_first] = f32::from_bits(first);
                            block[at_second] = f32::from_bits(second);
                        }
                        let oracle = block_admits_elision_oracle(&block);
                        assert_eq!(
                            block_admits_elision(&block),
                            oracle,
                            "length {length}: {first:#010x} at {at_first}, {second:#010x} at \
                             {at_second}"
                        );
                        verdicts[usize::from(oracle)] += 1;
                    }
                }
            }
        }
        assert!(
            verdicts[0] > 0 && verdicts[1] > 0,
            "the edge set must reach both verdicts: {verdicts:?}"
        );
    }

    /// Issue #980: the min/max gate and the rejection accumulator agree on every one of the
    /// `2^32` words, alone and in every run of 64 consecutive words.
    ///
    /// Both predicates are a conjunction of the same per-word test, so the lone-word verdict is the
    /// whole claim; the 64-word runs put every word through the vectorised loop body as well.
    /// Ignored by default (about a minute in release, several in a dev build): run it with
    /// `cargo test -p parametric-eq --release --lib every_word -- --ignored`.
    #[test]
    #[ignore = "exhaustive over 2^32 words; run explicitly"]
    fn every_word_gets_the_rejection_oracles_verdict() {
        use std::hint::black_box;
        const RUN: u64 = 64;
        let threads = std::thread::available_parallelism()
            .map_or(1, core::num::NonZeroUsize::get)
            .clamp(1, 8) as u64;
        let per_thread = (1_u64 << 32) / threads / RUN * RUN;
        let admitted: u64 = std::thread::scope(|scope| {
            let workers: Vec<_> = (0..threads)
                .map(|thread| {
                    scope.spawn(move || {
                        let start = thread * per_thread;
                        let end = if thread + 1 == threads {
                            1_u64 << 32
                        } else {
                            start + per_thread
                        };
                        let mut admitted = 0_u64;
                        let mut run = [0.0_f32; RUN as usize];
                        let mut first = start;
                        while first < end {
                            for (offset, word) in run.iter_mut().enumerate() {
                                let bits = first + offset as u64;
                                *word = f32::from_bits(bits as u32);
                                // `black_box` keeps the optimiser from proving the two
                                // bodies equal on a known word and deleting the comparison:
                                // both run, on a word they cannot see through.
                                let lone = [*word];
                                let verdict = block_admits_elision(black_box(&lone));
                                assert_eq!(
                                    verdict,
                                    block_admits_elision_oracle(black_box(&lone)),
                                    "word {bits:#010x}"
                                );
                                admitted += u64::from(verdict);
                            }
                            assert_eq!(
                                block_admits_elision(black_box(&run)),
                                block_admits_elision_oracle(black_box(&run)),
                                "the run of {RUN} words from {first:#010x}"
                            );
                            first += RUN;
                        }
                        admitted
                    })
                })
                .collect();
            workers
                .into_iter()
                .map(|worker| worker.join().expect("worker"))
                .sum()
        });
        // Every magnitude pattern from `0` to the ceiling is admitted at either sign, except the
        // one `-0.0` pattern.
        let expected = 2 * (u64::from(ELISION_MAGNITUDE_CEILING) + 1) - 1;
        assert_eq!(admitted, expected, "admitted words");
    }

    /// A channel with the two general bands at sections 1 and 3 live on every lane, and each
    /// dedicated cut live only on the lanes whose bit is set in its mask: dry on the others.
    fn dry_cut_channel<L: Lane, const W: usize>(
        offset: usize,
        hpf_lanes: u8,
        lpf_lanes: u8,
    ) -> Channel<L, W> {
        let targets: [[BandTarget; EQ_SECTION_COUNT]; W] = core::array::from_fn(|lane| {
            let track = (offset + lane) % corpus::LANES;
            let mut sections = corpus::sections(track);
            for (section, band) in sections.iter_mut().enumerate() {
                band.enabled = section == 1 || section == 3;
            }
            sections[HPF_SECTION] = BandTarget {
                enabled: hpf_lanes >> lane & 1 != 0,
                kind: EqBandKind::HighPass,
                frequency: 40.0 + track as f32 * 23.0,
                gain: 0.0,
                q: 0.5 + track as f32 * 0.1,
                slope: 1.0,
            };
            sections[LPF_SECTION] = BandTarget {
                enabled: lpf_lanes >> lane & 1 != 0,
                kind: EqBandKind::LowPass,
                frequency: 7_000.0 + track as f32 * 900.0,
                gain: 0.0,
                q: 0.6 + track as f32 * 0.07,
                slope: 1.0,
            };
            sections
        });
        Channel::new(targets, corpus::CORPUS_RATE).expect("dry cut design")
    }

    /// Issue #977: an admitted plan runs every depth-two pass select-free, dry lanes included, and
    /// renders what the per-section path renders; the depth-one tail keeps #976's rule
    /// (select-free unless a lane of either channel is dry there); a refused block keeps the masks
    /// on all six sections.
    ///
    /// The cut masks put the dry lanes in two pairs (both cuts live on some lanes), in the
    /// depth-one tail (the LPF after the pair of general bands: a masked tail), in a pair (the HPF
    /// with band 1, ahead of a general-band tail: a select-free tail), and nowhere (both cuts live
    /// everywhere); the widths run them at one, four and eight lanes, dual and mono. At one lane
    /// the masks reduce to lane 0, so the LPF-tail case has no LPF and no tail there.
    #[test]
    fn an_admitted_plan_runs_every_pair_select_free() {
        fn run<L: Lane, const W: usize>(width: &str) {
            for (hpf_lanes, lpf_lanes, select_free_tails) in [
                (0b0101_0101_u8, 0b0000_1001_u8, 0_usize),
                (0b0000_0000, 0b0110_0110, 0),
                (0b0011_0011, 0b0000_0000, 1),
                (0b1111_1111, 0b1111_1111, 0),
            ] {
                let label = format!("{width} hpf {hpf_lanes:08b} lpf {lpf_lanes:08b}");
                let mut arms = Vec::new();
                for stationary in [true, false] {
                    let mut left_channel = dry_cut_channel::<L, W>(0, hpf_lanes, lpf_lanes);
                    let mut right_channel = dry_cut_channel::<L, W>(3, hpf_lanes, lpf_lanes);
                    let mut left = block::<W>(0, 0);
                    let mut right = block::<W>(0, 3);
                    let kept = kept(&left_channel, &right_channel, &left, &right);
                    reset_masked_pair_passes();
                    reset_select_free_tail_passes();
                    process_channels(
                        (&mut left_channel, &mut right_channel),
                        &mut left,
                        &mut right,
                        FRAMES,
                        stationary,
                    );
                    if stationary {
                        assert!(kept < EQ_SECTION_COUNT, "{label}: the block is admitted");
                        assert_eq!(masked_pair_pass_count(), 0, "{label}: no masked pair");
                        assert_eq!(
                            select_free_tail_pass_count(),
                            select_free_tails,
                            "{label}: the tail keeps #976's rule"
                        );
                    }
                    let mut mono = block::<W>(0, 5);
                    process_channels_mono(&mut left_channel, &mut mono, FRAMES, stationary);
                    if stationary {
                        assert_eq!(masked_pair_pass_count(), 0, "{label}: no masked mono pair");
                        assert_eq!(
                            select_free_tail_pass_count(),
                            2 * select_free_tails,
                            "{label}: the mono tail keeps #976's rule"
                        );
                    }
                    arms.push((
                        bits(&left),
                        bits(&right),
                        bits(&mono),
                        integrators(&left_channel, &right_channel),
                    ));
                }
                assert!(
                    arms[0] == arms[1],
                    "{label}: the select-free plan must render the per-section path's bits"
                );
                // A `-0.0` refuses the block: all six sections, three masked passes, both bodies.
                let mut left_channel = dry_cut_channel::<L, W>(0, hpf_lanes, lpf_lanes);
                let mut right_channel = dry_cut_channel::<L, W>(3, hpf_lanes, lpf_lanes);
                let mut left = block::<W>(0, 0);
                let mut right = block::<W>(0, 3);
                left[W] = -0.0;
                reset_masked_pair_passes();
                process_channels(
                    (&mut left_channel, &mut right_channel),
                    &mut left,
                    &mut right,
                    FRAMES,
                    true,
                );
                assert_eq!(
                    masked_pair_pass_count(),
                    3,
                    "{label}: a refused block is masked"
                );
                let mut mono = block::<W>(0, 5);
                mono[W] = -0.0;
                process_channels_mono(&mut left_channel, &mut mono, FRAMES, true);
                assert_eq!(
                    masked_pair_pass_count(),
                    6,
                    "{label}: and so is its mono body"
                );
            }
        }
        run::<f32, 1>("Scalar");
        run::<Simd4, 4>("Simd4");
        run::<Simd8, 8>("Simd8");
    }

    /// A non-finite integrator in a *live* section refuses the elision (issue #977, leg (c)).
    ///
    /// The kernel can write one: a dry lane still runs the recurrence, and a refused block can
    /// overflow a large restored integrator there to `NaN` behind the dry select. An admitted plan
    /// runs select-free, which would turn that state into output, so leg (c) keeps the bank on the
    /// masked kernel instead.
    #[test]
    fn a_non_finite_state_in_a_live_section_refuses_elision() {
        for word in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, -f32::NAN] {
            for integrator in [0_usize, 1] {
                // The HPF is live on lane 0 only, so lane 6 is a dry lane of a live section.
                let mut left_channel = dry_cut_channel::<Simd8, 8>(0, 0b0000_0001, 0);
                let right_channel = dry_cut_channel::<Simd8, 8>(3, 0, 0);
                let mut lanes = [0.25_f32; 8];
                lanes[6] = word;
                let seeded = Simd8::load(&lanes);
                if integrator == 0 {
                    left_channel.sections[HPF_SECTION].state.ic1 = seeded;
                } else {
                    left_channel.sections[HPF_SECTION].state.ic2 = seeded;
                }
                let (left, right) = (block::<8>(0, 0), block::<8>(0, 3));
                assert_eq!(
                    kept(&left_channel, &right_channel, &left, &right),
                    EQ_SECTION_COUNT,
                    "a live section holding {word} in integrator {integrator} must refuse elision"
                );
                assert_eq!(
                    cascade_sections_mono::<Simd8, 8>(&left_channel, &left, FRAMES).1,
                    EQ_SECTION_COUNT,
                    "and so must its mono body"
                );
            }
        }
        // A finite state there is admitted: the term refuses non-finite words, not large ones.
        let mut left_channel = dry_cut_channel::<Simd8, 8>(0, 0b0000_0001, 0);
        let right_channel = dry_cut_channel::<Simd8, 8>(3, 0, 0);
        left_channel.sections[HPF_SECTION].state.ic2 = Simd8::splat(-f32::MAX);
        let (left, right) = (block::<8>(0, 0), block::<8>(0, 3));
        assert_eq!(kept(&left_channel, &right_channel, &left, &right), 3);
    }

    /// The `-0.0` refusal is load-bearing: forced past it, the bits really do move.
    ///
    /// Without this the gate could be decorative -- a refusal that never mattered would leave
    /// every other test in this module green. Here the elided cascade is run *directly*, with the
    /// section list the gate would have produced had it not refused, and the result is asserted to
    /// **differ** from the full cascade. That difference is exactly the `-0.0` an identity section
    /// rewrites to `+0.0`.
    #[test]
    fn the_negative_zero_refusal_is_load_bearing() {
        let mut moved = 0_usize;
        for stationary in [true, false] {
            let mut left_channel = channel::<Simd8, 8>(0, 0b0000);
            let mut right_channel = channel::<Simd8, 8>(3, 0b0000);
            let mut left = vec![-0.0_f32; FRAMES * 8];
            let mut right = vec![-0.0_f32; FRAMES * 8];
            if stationary {
                // The list the gate refuses to hand out: an all-identity cascade elides to zero
                // sections, so the elided arm writes nothing at all.
                super::interleave::<Simd8, 8, 2>(
                    (&mut left_channel, &mut right_channel),
                    &mut left,
                    &mut right,
                    FRAMES,
                    ([0, 1, 2, 3, 4, 5], 0),
                );
            } else {
                process_channels(
                    (&mut left_channel, &mut right_channel),
                    &mut left,
                    &mut right,
                    FRAMES,
                    false,
                );
            }
            moved += usize::from(left[0].to_bits() == 0x8000_0000);
        }
        assert_eq!(
            moved, 1,
            "the elided arm must keep the -0.0 the full cascade rewrites to +0.0; if both arms \
             agree the gate is not protecting anything"
        );
        // And the gate does refuse this block.
        let left_channel = channel::<Simd8, 8>(0, 0b0000);
        let right_channel = channel::<Simd8, 8>(3, 0b0000);
        let negative = vec![-0.0_f32; FRAMES * 8];
        assert_eq!(
            kept(&left_channel, &right_channel, &negative, &negative),
            EQ_SECTION_COUNT,
            "an all -0.0 block is refused"
        );
    }

    /// Enabling and disabling a band mid-session keeps the flag honest, and every block renders
    /// exactly what the full cascade renders.
    ///
    /// The transition arrives the way automation delivers it -- through `start_ramp`, which ramps
    /// the six words over the smoothing window -- so the sequence covers the non-stationary blocks
    /// during the ramp, the snap that ends it, and the stationary blocks on either side.
    #[test]
    fn a_mid_session_enable_or_disable_stays_bit_exact() {
        const BLOCKS: usize = 8;
        let mut engaged = 0_usize;
        let mut arms = Vec::new();
        for elide in [true, false] {
            let mut left_channel = channel::<Simd8, 8>(0, 0b0001);
            let mut right_channel = channel::<Simd8, 8>(3, 0b0001);
            let mut rendered: Vec<u32> = Vec::new();
            for step in 0..BLOCKS {
                if step == 2 {
                    // Band 3 comes on, on every lane of both channels.
                    for lane in 0..8 {
                        let mut band = corpus::bands(lane % corpus::LANES)[2];
                        band.enabled = true;
                        let words = band.words(corpus::CORPUS_RATE).expect("legal design");
                        left_channel.start_ramp(2, lane, words);
                        right_channel.start_ramp(2, lane, words);
                    }
                }
                if step == 5 {
                    // And goes off again.
                    for lane in 0..8 {
                        left_channel.start_ramp(2, lane, EqSvfWords::IDENTITY);
                        right_channel.start_ramp(2, lane, EqSvfWords::IDENTITY);
                    }
                }
                let mut left = block::<8>(0, step % corpus::LANES);
                let mut right = block::<8>(0, (step + 3) % corpus::LANES);
                let stationary =
                    left_channel.no_ramp_in_flight() && right_channel.no_ramp_in_flight();
                // The two arms differ only in whether the stationary path may shorten the
                // cascade; `elide = false` forces the per-section path, which never does.
                let stationary = stationary && elide;
                if stationary
                    && kept(&left_channel, &right_channel, &left, &right) < EQ_SECTION_COUNT
                {
                    engaged += 1;
                }
                process_channels(
                    (&mut left_channel, &mut right_channel),
                    &mut left,
                    &mut right,
                    FRAMES,
                    stationary,
                );
                rendered.extend(bits(&left));
                rendered.extend(bits(&right));
            }
            rendered.extend(integrators(&left_channel, &right_channel));
            arms.push(rendered);
        }
        assert_eq!(
            arms[0], arms[1],
            "an enable/disable transition must render the full cascade's bits at every block"
        );
        assert!(
            engaged >= 4,
            "the transition sequence must actually engage the elision on its stationary blocks, \
             engaged on {engaged} of {BLOCKS}"
        );
    }

    /// A `DiscontinuityKeepParameters` reset taken mid-ramp must refresh `identity`.
    ///
    /// This is the one coefficient-change site the rest of the suite never reaches.
    /// [`start_ramp`](Channel::start_ramp) refreshes the flag while `coef` is still the identity it
    /// is leaving, so the flag reads `true`; the ramp then walks `coef` away through
    /// the ramp kernel, which refreshes nothing. That staleness is licensed only because a ramp in
    /// flight makes the bank non-stationary, and every ramp that ends *inside*
    /// [`process_section`](Channel::process_section) refreshes on the way out.
    ///
    /// A discontinuity reset ends the ramp from outside the render path: it snaps `coef` to a live
    /// target and clears `remaining`, so the very next block is stationary and reads the flag.
    /// Without the refresh at that site the flag still says "identity" for a section that is now a
    /// real filter, and `cascade_sections` elides it -- the whole cascade here, so the block comes
    /// back unfiltered.
    ///
    /// The conformance reset scenario does not reach this: it resets with no ramp in flight, where
    /// the snap is a no-op and the flag was already right.
    fn reset_mid_ramp_case<L: Lane, const W: usize>(width: &str) {
        // Every section identity, so the ramped section is the only live one after the reset and a
        // stale flag elides the entire cascade rather than one position of it.
        let mut left_channel = channel::<L, W>(0, 0b0000);
        let mut right_channel = channel::<L, W>(3, 0b0000);
        assert!(
            left_channel.identity[0] && right_channel.identity[0],
            "{width}: section 0 must start at the identity the ramp leaves"
        );

        // Ramp section 0 of every lane from that identity to a live corpus design.
        for (offset, target_channel) in [(0_usize, &mut left_channel), (3, &mut right_channel)] {
            for track in 0..W {
                let words = corpus::bands((offset + track) % corpus::LANES)[0]
                    .words(corpus::CORPUS_RATE)
                    .expect("every corpus row is a legal design");
                target_channel.start_ramp(0, track, words);
            }
        }

        // Walk part of the way, which moves `coef` off identity without ending the ramp.
        const MID_RAMP: usize = 16;
        assert!(MID_RAMP < RAMP_SAMPLES as usize, "the walk must not settle");
        let mut left = block::<W>(0, 0);
        let mut right = block::<W>(0, 3);
        process_channels(
            (&mut left_channel, &mut right_channel),
            &mut left,
            &mut right,
            MID_RAMP,
            false,
        );
        assert!(
            (0..W).all(|track| left_channel.remaining[0][track] > 1),
            "{width}: the ramp must still be in flight when the reset lands"
        );
        assert!(
            !left_channel.identity_flags_agree(),
            "{width}: non-vacuity -- the reset must land inside the licensed stale window, where \
             the words have left identity and the flag has not been refreshed"
        );

        // The `ResetKind::DiscontinuityKeepParameters` route.
        left_channel.discontinuity_reset();
        right_channel.discontinuity_reset();
        assert!(
            left_channel.identity_flags_agree() && right_channel.identity_flags_agree(),
            "{width}: a discontinuity reset must leave `identity` agreeing with the words it \
             snapped to"
        );

        // And the next stationary block must render the live section rather than elide it. The
        // reference reached the same coefficients and the same cleared integrators without ever
        // ramping, so the two renders are bit-identical when the flag is right and diverge by a
        // whole cascade when it is not.
        let mut reference_left = channel::<L, W>(0, 0b0001);
        let mut reference_right = channel::<L, W>(3, 0b0001);
        let mut left = block::<W>(0, 0);
        let mut right = block::<W>(0, 3);
        let mut reference_left_block = block::<W>(0, 0);
        let mut reference_right_block = block::<W>(0, 3);
        assert_eq!(
            kept(&left_channel, &right_channel, &left, &right),
            kept(
                &reference_left,
                &reference_right,
                &reference_left_block,
                &reference_right_block,
            ),
            "{width}: the reset channel must run the same cascade positions as the settled one"
        );
        process_channels(
            (&mut left_channel, &mut right_channel),
            &mut left,
            &mut right,
            FRAMES,
            true,
        );
        process_channels(
            (&mut reference_left, &mut reference_right),
            &mut reference_left_block,
            &mut reference_right_block,
            FRAMES,
            true,
        );
        assert_eq!(
            bits(&left),
            bits(&reference_left_block),
            "{width}: the block after a mid-ramp reset must render the live section"
        );
        assert_eq!(
            bits(&right),
            bits(&reference_right_block),
            "{width}: the block after a mid-ramp reset must render the live section"
        );
        assert_eq!(
            integrators(&left_channel, &right_channel),
            integrators(&reference_left, &reference_right),
            "{width}: and must leave the same integrators"
        );
    }

    /// The mid-ramp discontinuity reset, at all three widths.
    #[test]
    fn a_discontinuity_reset_mid_ramp_refreshes_the_identity_flag() {
        reset_mid_ramp_case::<f32, 1>("Scalar");
        reset_mid_ramp_case::<Simd4, 4>("Simd4");
        reset_mid_ramp_case::<Simd8, 8>("Simd8");
    }
}

/// Issue #999 gate 1: the §4.4 verdict the stationary cascade folds into its depth-one pass is
/// [`check_block`] of the planes that pass wrote, and only a block that runs that pass on an
/// admitted plan takes it.
///
/// [`interleave`] returns the tail's verdict for an admitted list of odd length (1, 3 or 5
/// sections) and `None` for everything else -- an even list (0, 2 or 4), a refused or all-live list
/// (6), and a ramped block -- where [`PreparedParametricEq::render`] keeps the scan. Each block
/// below is rendered through [`process_channels`] and [`process_channels_mono`] exactly as `render`
/// calls them, and the returned verdict is compared with `check_block` of the rendered planes.
///
/// The words that reach the tail's store are steered to the limit with *scale* sections -- live
/// sections holding `c1 = a2 = a3 = m1 = m2 = +0.0` and a chosen `m0`, whose output is `m0 * x`
/// rounded once for every finite state and every `x != -0.0` -- so the stored words include the
/// limit itself (`m0 = 1.0` on an admitted `1e30`), its neighbours, both infinities and `NaN` (a
/// *clash* section, whose `m2 * v2` is `-inf` against a restored `ic2 = 1e30`). Every input word
/// is admitted: finite, at most `1e30` in magnitude, never `-0.0`. A second family renders the
/// corpus bands over admitted input scaled to `1e30` with huge restored integrators, the states
/// elision leg (c) admits in a live section. At `f32`, `Simd4` and `Simd8`, dual and collapsed
/// mono; every category of stored word and every combination of the two channels' verdicts
/// occurs at every width.
#[cfg(test)]
mod boundary_fold {
    use super::{
        BLOCK_LIMIT, Channel, EQ_SECTION_COUNT, EqSvfWords, cascade_sections,
        cascade_sections_mono, check_block, corpus, lane_set, process_channels,
        process_channels_mono,
    };
    use lane::{Lane, Simd4, Simd8};

    /// `2^104`: times `1e30` it overflows, times `1.0` it does not.
    const HUGE: f32 = f32::from_bits(0x7380_0000);
    /// The largest `f32` below the limit, and the smallest above it.
    const BELOW: f32 = f32::from_bits(BLOCK_LIMIT.to_bits() - 1);
    const ABOVE: f32 = f32::from_bits(BLOCK_LIMIT.to_bits() + 1);

    /// A live section whose output is `m0 * x` (see the module comment). `m0 = 1.0` is the
    /// identity words: that lane stores its input unchanged, and is dry in a dedicated cut.
    fn scale(m0: f32) -> EqSvfWords {
        EqSvfWords {
            c1: 0.0,
            a2: 0.0,
            a3: 0.0,
            m0,
            m1: 0.0,
            m2: 0.0,
        }
    }

    /// A live section whose mix meets itself at infinity once `ic2 = 1e30`: `m2 * v2 = -inf`, so
    /// `x = 1e30` gives `-inf + inf = NaN` and a small `x` gives `-inf`.
    fn clash() -> EqSvfWords {
        EqSvfWords {
            c1: 0.0,
            a2: 0.0,
            a3: 0.0,
            m0: HUGE,
            m1: 0.0,
            m2: -HUGE,
        }
    }

    /// Words that never amplify: ahead of the tail, and on a channel meant to pass.
    fn tame(index: usize) -> EqSvfWords {
        [
            scale(1.0),
            scale(0.5),
            scale(f32::from_bits(0x3f7f_ffff)),
            scale(1.0),
        ][index % 4]
    }

    /// Words that push the stored word past the limit, to infinity, or to `NaN`.
    fn hot(index: usize) -> EqSvfWords {
        [
            scale(1.0),
            scale(f32::from_bits(0x3f80_0001)),
            scale(HUGE),
            clash(),
            scale(1.0),
            scale(0.5),
        ][index % 6]
    }

    /// Admitted input words at the limit and around it.
    const LOUD: [f32; 10] = [
        1.0e30, BELOW, 1.0, -1.0e30, 5.0e29, 0.0, 1.0e-39, -BELOW, 7.0, -3.0e-5,
    ];
    /// Admitted input words that no tame section carries past the limit.
    const QUIET: [f32; 5] = [1.0, 0.5, 0.0, 2.0, -1.0];

    /// What the folded blocks stored, and how their verdicts fell.
    #[derive(Default, Debug)]
    struct Seen {
        folded: usize,
        scanned: usize,
        /// Dual verdicts `(left, right)`: pass-pass, fail-pass, pass-fail, fail-fail.
        dual: [usize; 4],
        /// Mono verdicts: pass, fail.
        mono: [usize; 2],
        at: usize,
        below: usize,
        above: usize,
        positive_infinity: usize,
        negative_infinity: usize,
        nan: usize,
        /// Folded dual blocks of the corpus family where a channel failed, and where both passed.
        designed: [usize; 2],
    }

    impl Seen {
        fn stored(&mut self, words: &[f32]) {
            for &word in words {
                match word {
                    _ if word.is_nan() => self.nan += 1,
                    f32::INFINITY => self.positive_infinity += 1,
                    f32::NEG_INFINITY => self.negative_infinity += 1,
                    _ if word.abs() == BLOCK_LIMIT => self.at += 1,
                    _ if word.abs() == BELOW => self.below += 1,
                    _ if word.abs() == ABOVE => self.above += 1,
                    _ => {}
                }
            }
        }
    }

    /// One AoSoA plane of `frames` frames from `word(frame, lane)`.
    fn plane<const W: usize>(frames: usize, word: impl Fn(usize, usize) -> f32) -> Vec<f32> {
        (0..frames * W)
            .map(|cell| word(cell / W, cell % W))
            .collect()
    }

    /// Renders one block both ways `render` can: the dual body over `(left, right)` and the
    /// collapsed body over `left`; asserts the verdict rule and records what was stored.
    #[allow(clippy::too_many_arguments)]
    fn judge<L: Lane, const W: usize>(
        label: &str,
        build: &dyn Fn(usize) -> Channel<L, W>,
        left: &[f32],
        right: &[f32],
        frames: usize,
        stationary: bool,
        designed: bool,
        seen: &mut Seen,
    ) {
        let (mut left_channel, mut right_channel) = (build(0), build(1));
        let (mut left_plane, mut right_plane) = (left.to_vec(), right.to_vec());
        let kept = cascade_sections::<L, W>(
            &left_channel,
            &right_channel,
            &left_plane,
            &right_plane,
            frames,
        )
        .1;
        let within = process_channels(
            (&mut left_channel, &mut right_channel),
            &mut left_plane,
            &mut right_plane,
            frames,
            stationary,
        );
        let scan = [
            check_block::<L>(&left_plane),
            check_block::<L>(&right_plane),
        ];
        assert_eq!(
            within.is_some(),
            stationary && kept % 2 == 1,
            "{label} dual: kept {kept}, stationary {stationary}: only an admitted list ending in \
             the depth-one pass folds the verdict"
        );
        if let Some(verdict) = within {
            assert_eq!(
                verdict, scan,
                "{label} dual: kept {kept}: the folded verdict is not the scan"
            );
            seen.folded += 1;
            seen.dual[usize::from(!verdict[0]) + 2 * usize::from(!verdict[1])] += 1;
            seen.stored(&left_plane);
            seen.stored(&right_plane);
            if designed {
                seen.designed[usize::from(verdict == [true; 2])] += 1;
            }
        } else {
            seen.scanned += 1;
        }

        let mut channel = build(0);
        let mut io = left.to_vec();
        let kept = cascade_sections_mono::<L, W>(&channel, &io, frames).1;
        let within = process_channels_mono(&mut channel, &mut io, frames, stationary);
        let scan = check_block::<L>(&io);
        assert_eq!(
            within.is_some(),
            stationary && kept % 2 == 1,
            "{label} mono: kept {kept}, stationary {stationary}: only an admitted list ending in \
             the depth-one pass folds the verdict"
        );
        if let Some(verdict) = within {
            assert_eq!(
                verdict, scan,
                "{label} mono: kept {kept}: the folded verdict is not the scan"
            );
            seen.folded += 1;
            seen.mono[usize::from(!verdict)] += 1;
            seen.stored(&io);
        } else {
            seen.scanned += 1;
        }
    }

    /// A channel whose sections in `live` are scale or clash sections and the rest the identity.
    /// The last live section takes its words from `tail`, the others from [`tame`]; a lane holding
    /// [`clash`] starts from `ic2 = 1e30`.
    fn steered<L: Lane, const W: usize>(
        live: u8,
        tail: fn(usize) -> EqSvfWords,
        variant: usize,
        side: usize,
    ) -> Channel<L, W> {
        let mut disabled = corpus::sections(0);
        for band in &mut disabled {
            band.enabled = false;
        }
        let mut channel = Channel::<L, W>::new([disabled; W], corpus::CORPUS_RATE)
            .expect("a disabled band is a legal design");
        let last = (0..EQ_SECTION_COUNT).rev().find(|s| live & (1 << s) != 0);
        for section in (0..EQ_SECTION_COUNT).filter(|s| live & (1 << s) != 0) {
            for lane in 0..W {
                let index = lane + 2 * section + side + variant;
                let words = if Some(section) == last {
                    tail(index)
                } else {
                    tame(index)
                };
                channel.settle(section, lane, words);
                if words.m2 != 0.0 {
                    lane_set(&mut channel.sections[section].state.ic2, lane, 1.0e30);
                }
            }
        }
        channel
    }

    /// Masks of live sections: nothing, one (a cut, a band, the other cut), two, three, four, five
    /// (the LPF last, and the HPF first), all six.
    const LIVE: [u8; 11] = [
        0b00_0000, 0b00_0001, 0b00_0100, 0b10_0000, 0b10_0001, 0b01_0110, 0b10_0110, 0b01_1110,
        0b11_1101, 0b01_1111, 0b11_1111,
    ];

    fn sweep<L: Lane, const W: usize>(width: &str) -> Seen {
        let mut seen = Seen::default();
        type Table = fn(usize) -> EqSvfWords;
        let pairs: [(Table, Table, bool); 5] = [
            (tame, tame, true),
            (tame, tame, false),
            (tame, hot, true),
            (hot, tame, true),
            (hot, hot, true),
        ];
        for live in LIVE {
            for (pair, &(left_tail, right_tail, loud)) in pairs.iter().enumerate() {
                for variant in 0..3 {
                    for frames in [1_usize, 16] {
                        let tails = [left_tail, right_tail];
                        let build = |side: usize| steered::<L, W>(live, tails[side], variant, side);
                        let word = |side: usize| {
                            move |frame: usize, lane: usize| {
                                let index = frame + 3 * lane + 5 * side + variant;
                                if loud {
                                    LOUD[index % LOUD.len()]
                                } else {
                                    QUIET[index % QUIET.len()]
                                }
                            }
                        };
                        let left = plane::<W>(frames, word(0));
                        let right = plane::<W>(frames, word(1));
                        let label = format!(
                            "{width} live {live:06b} pair {pair} variant {variant} frames {frames}"
                        );
                        for stationary in [true, false] {
                            judge::<L, W>(
                                &label, &build, &left, &right, frames, stationary, false, &mut seen,
                            );
                        }
                        // Refused: one `-0.0`, then one `NaN`, on the right plane.
                        for poison in [-0.0_f32, f32::NAN] {
                            let mut refused = right.clone();
                            refused[frames * W - 1] = poison;
                            judge::<L, W>(
                                &format!("{label} poisoned {poison}"),
                                &build,
                                &left,
                                &refused,
                                frames,
                                true,
                                false,
                                &mut seen,
                            );
                        }
                    }
                }
            }
        }
        // The corpus bands (odd live counts of general bands) over admitted input scaled to the
        // limit, some lanes holding huge restored integrators in a live band; and, quiet, over the
        // same input scaled to `1e20` with no restored state.
        for live in [0b00_0100_u8, 0b01_0110, 0b01_1110 | 0b00_0001] {
            for seed in 0..6 {
                let loud = seed < 4;
                let build = |side: usize| {
                    let targets = core::array::from_fn(|lane| {
                        let mut bands = corpus::sections((lane + 3 * side) % corpus::LANES);
                        for (section, band) in bands.iter_mut().enumerate() {
                            band.enabled = live & (1 << section) != 0 && section != 0;
                        }
                        bands
                    });
                    let mut channel = Channel::<L, W>::new(targets, corpus::CORPUS_RATE)
                        .expect("every corpus row is a legal design");
                    for lane in (0..W).filter(|lane| loud && (lane + side + seed).is_multiple_of(3))
                    {
                        let section = 2 + (lane + seed) % 2;
                        if live & (1 << section) != 0 {
                            let state = &mut channel.sections[section].state;
                            lane_set(&mut state.ic1, lane, f32::MAX);
                            lane_set(&mut state.ic2, lane, -f32::MAX);
                        }
                    }
                    channel
                };
                let frames = 64;
                let word = |side: usize| {
                    move |frame: usize, lane: usize| {
                        let mut samples = [0.0_f32; corpus::FRAMES];
                        corpus::fill(0, (lane + side + seed) % corpus::LANES, &mut samples);
                        samples[frame] * if loud { 1.0e30 } else { 1.0e20 }
                    }
                };
                let left = plane::<W>(frames, word(0));
                let right = plane::<W>(frames, word(1));
                let label = format!("{width} corpus live {live:06b} seed {seed}");
                judge::<L, W>(&label, &build, &left, &right, frames, true, true, &mut seen);
            }
        }
        seen
    }

    fn check<L: Lane, const W: usize>(width: &str) {
        let seen = sweep::<L, W>(width);
        println!("boundary fold {width}: {seen:?}");
        assert!(seen.folded > 0 && seen.scanned > 0, "{width}: {seen:?}");
        assert!(
            seen.dual.iter().all(|&count| count > 0) && seen.mono.iter().all(|&count| count > 0),
            "{width}: every combination of verdicts must occur: {seen:?}"
        );
        assert!(
            [
                seen.at,
                seen.below,
                seen.above,
                seen.positive_infinity,
                seen.negative_infinity,
                seen.nan,
            ]
            .iter()
            .all(|&count| count > 0),
            "{width}: the folded blocks must store words at, just below and just above the limit, \
             both infinities and NaN: {seen:?}"
        );
        assert!(
            seen.designed.iter().all(|&count| count > 0),
            "{width}: the corpus family must fold failing and passing blocks: {seen:?}"
        );
    }

    #[test]
    fn the_folded_verdict_is_the_boundary_scan() {
        check::<f32, 1>("Scalar");
        check::<Simd4, 4>("Simd4");
        check::<Simd8, 8>("Simd8");
    }
}

#[cfg(test)]
mod target_application {
    use super::*;
    use effect_contract::{
        AutomationSpanKind, EffectTargetRequest, NativeEffectTargetPreparation,
        PreparedAutomationSpan,
    };

    fn request<'a>(values: &'a [InitialParameterValue]) -> PrepareEffectRequest<'a> {
        PrepareEffectRequest {
            sample_rate: 48_000,
            quantum: 128,
            quality: Quality::Normal,
            bypass: false,
            link_mode: effect_contract::LinkMode::DualMono,
            ports: effect_contract::PreparedPorts {
                sidechain: effect_contract::PreparedSidechainPort::None,
            },
            initial_values: values,
            limits: effect_contract::PrepareEffectLimits {
                maximum_total_state_bytes: 1_024,
                maximum_scratch_bytes: 1,
                maximum_automation_spans_per_block: 48,
            },
        }
    }

    #[test]
    fn prepared_apply_and_resets_do_not_design_on_render_or_reset_paths() {
        let values: Vec<_> =
            effect_contract::default_initial_values(&PARAMETRIC_EQ_DESCRIPTOR).collect();
        let mut target_values = values.clone();
        target_values[48].value = 1.0;
        target_values[50].value = 1_000.0;
        target_values[52].value = 1.0;
        let mut changed = vec![false; target_values.len()];
        changed[48] = true;
        changed[50] = true;
        changed[52] = true;
        let mut targets = [PreparedEffectTarget {
            slot: 0,
            channel: ParameterChannel::Left,
            words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
        }; EQ_SECTION_COUNT * 2];
        let count = <ParametricEqFactory as NativeEffectTargetPreparation>::prepare_targets(
            &ParametricEqFactory,
            EffectTargetRequest {
                sample_rate: 48_000,
                values: &target_values,
                changed: &changed,
            },
            &mut targets,
        )
        .expect("target preparation");
        assert_eq!(count, 1);

        let mut effect = ParametricEqFactory
            .prepare(request(&values))
            .expect("effect preparation");
        reset_design_calls();
        effect
            .apply_prepared_target(&targets[0])
            .expect("prepared apply");
        let mut left = [1.0_f32; 64];
        let mut right = [0.0_f32; 64];
        effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("render"),
        );
        assert_eq!(
            design_call_count(),
            0,
            "prepared apply and render must not redesign"
        );
        effect.reset(ResetKind::FullToDefaults);
        effect.reset(ResetKind::DiscontinuityKeepParameters);
        assert_eq!(design_call_count(), 0, "cached resets must not redesign");

        let mut raw = ParametricEqFactory
            .prepare(request(&target_values))
            .expect("legacy effect preparation");
        reset_design_calls();
        let mut left = [1.0_f32; 1];
        let mut right = [0.0_f32; 1];
        let report = raw.process(
            EffectProcessBlock::new(
                &mut left,
                &mut right,
                None,
                0,
                &[PreparedAutomationSpan {
                    kind: AutomationSpanKind::Point,
                    channel: ParameterChannel::Left,
                    parameter_index: 2,
                    start_sample: 0,
                    end_sample: 0,
                    start_value: 2_000.0,
                    end_value: 2_000.0,
                }],
                128,
            )
            .expect("legacy render"),
        );
        assert_eq!(report.invalid_spans, 1);
        assert_eq!(design_call_count(), 0);
    }
}

/// Issue #1005: a ramping block runs only live or ramping sections, under the elision gate.
///
/// Two gates, at every width (`f32`, `Simd4`, `Simd8`), dual and collapsed:
///
/// * **The list, structurally** (gate 2). The unsafe-ramp rule cannot be seen by a random
///   differential -- its counterexample needs a constructed subnormal underflow -- so the list
///   [`ramping_sections`] returns is asserted directly for the shapes the rule decides.
/// * **The bank differential** (gate 1). Two prepared EQs per scenario, one rendering ramping
///   blocks the batch-head way ([`Channel::process_block`], the unit-test default of the
///   [`RAMPING_LIST`] switch) and one through the list, driven through the contract calls
///   with production target preparation, resets, restores and hostile input. Every output word,
///   every report, every lane payload and every internal word is compared by bits after every
///   block.
#[cfg(test)]
mod ramping_elision {
    use super::*;
    use effect_contract::{
        EffectBankProcessBlock, EffectTargetRequest, NativeEffectTargetPreparation,
    };

    const RATE: SampleRateHz = SampleRateHz(48_000);
    const QUANTUM: usize = 128;
    const BAND: usize = BAND_SECTION_OFFSET;

    // ---------------------------------------------------------------------------------------
    // Gate 2: the list, structurally.
    // ---------------------------------------------------------------------------------------

    fn target(enabled: bool, kind: EqBandKind, frequency: f32, gain: f32, q: f32) -> BandTarget {
        BandTarget {
            enabled,
            kind,
            frequency,
            gain,
            q,
            slope: 1.0,
        }
    }

    fn off() -> BandTarget {
        target(false, EqBandKind::Bell, 1_000.0, 0.0, 1.0)
    }

    fn design(kind: EqBandKind, frequency: f32, gain: f32, q: f32) -> EqSvfWords {
        design_svf(kind, frequency, gain, q, 1.0, RATE).expect("a legal design")
    }

    /// The physical layout the gate-2 shapes share: HPF off, band 1 a live bell, bands 2-4 as
    /// given, LPF off.
    fn layout(band2: BandTarget) -> [BandTarget; EQ_SECTION_COUNT] {
        let mut sections = [off(); EQ_SECTION_COUNT];
        sections[HPF_SECTION] = target(false, EqBandKind::HighPass, 80.0, 0.0, 0.7);
        sections[BAND] = target(true, EqBandKind::Bell, 400.0, -3.0, 1.0);
        sections[BAND + 1] = band2;
        sections[LPF_SECTION] = target(false, EqBandKind::LowPass, 12_000.0, 0.0, 0.7);
        sections
    }

    fn channel<L: Lane, const W: usize>(sections: [BandTarget; EQ_SECTION_COUNT]) -> Channel<L, W> {
        Channel::new([sections; W], RATE).expect("a legal channel")
    }

    fn admitted<const W: usize>() -> Vec<f32> {
        (0..QUANTUM * W)
            .map(|index| 0.25 - 0.001 * (index % 97) as f32)
            .collect()
    }

    /// The list at this width, dual (the pair) and collapsed (the left channel alone).
    fn lists<L: Lane, const W: usize>(
        left: &Channel<L, W>,
        right: &Channel<L, W>,
        io: &[f32],
    ) -> (Vec<usize>, Vec<usize>) {
        let (dual, dual_length) = ramping_sections::<L, W>(left, right, io, io, QUANTUM);
        let (mono, mono_length) = ramping_sections_mono::<L, W>(left, io, QUANTUM);
        (dual[..dual_length].to_vec(), mono[..mono_length].to_vec())
    }

    /// Both channels built from `sections`, then `ramp` applied to each.
    fn pair<L: Lane, const W: usize>(
        sections: [BandTarget; EQ_SECTION_COUNT],
        ramp: impl Fn(&mut Channel<L, W>),
    ) -> (Channel<L, W>, Channel<L, W>) {
        let mut left = channel::<L, W>(sections);
        let mut right = channel::<L, W>(sections);
        ramp(&mut left);
        ramp(&mut right);
        (left, right)
    }

    fn expect_list<L: Lane, const W: usize>(
        width: &str,
        shape: &str,
        channels: &(Channel<L, W>, Channel<L, W>),
        expected: &[usize],
    ) {
        let io = admitted::<W>();
        let (dual, mono) = lists::<L, W>(&channels.0, &channels.1, &io);
        assert_eq!(dual, expected, "{width}: {shape}: the dual list");
        assert_eq!(mono, expected, "{width}: {shape}: the collapsed list");
    }

    #[allow(clippy::too_many_lines)]
    fn unsafe_ramp_rule<L: Lane, const W: usize>(width: &str) {
        let shelf = target(true, EqBandKind::HighShelf, 3_000.0, 6.0, 0.7);
        let bell = target(true, EqBandKind::Bell, 2_000.0, 4.0, 1.2);
        let shelf_ride = design(EqBandKind::HighShelf, 3_000.0, 7.5, 0.7);
        let bell_ride = design(EqBandKind::Bell, 2_000.0, 5.5, 1.2);

        // A ramping high shelf in band 2, bands 3 and 4 dead: they are kept, and so is the LPF,
        // because the shelf's `m0` word moves. The HPF ahead of it still goes.
        let shape = pair::<L, W>(layout(shelf), |c| c.start_ramp(BAND + 1, 0, shelf_ride));
        expect_list(width, "ramping high shelf", &shape, &[1, 2, 3, 4, 5]);

        // The same with a ramping bell: `m0` stays exactly one, so they go.
        let shape = pair::<L, W>(layout(bell), |c| c.start_ramp(BAND + 1, 0, bell_ride));
        expect_list(width, "ramping bell", &shape, &[1, 2]);

        // An LPF-cut toggle: an unsafe ramp (its `m0` falls from one to zero), and nothing
        // follows it, so the list keeps nothing extra.
        let mut sections = layout(off());
        sections[LPF_SECTION].enabled = true;
        let lpf = design(EqBandKind::LowPass, 12_000.0, 0.0, 0.7);
        let shape = pair::<L, W>(layout(off()), |c| {
            c.targets[0][LPF_SECTION] = sections[LPF_SECTION];
            c.start_ramp(LPF_SECTION, 0, lpf);
        });
        expect_list(width, "LPF toggle", &shape, &[1, 5]);

        // An HPF-cut toggle: the identity and a high pass both have `m0 = 1`, so the dead bands
        // after it go. The HPF itself is ramping from the identity and runs.
        let hpf = design(EqBandKind::HighPass, 80.0, 0.0, 0.7);
        let shape = pair::<L, W>(layout(off()), |c| c.start_ramp(HPF_SECTION, 0, hpf));
        expect_list(width, "HPF toggle", &shape, &[0, 1]);

        // VERIFY-AUTOMATION A2: an identity -> high-shelf ramp in band 2. `coef.m0` starts at
        // exactly one, but its increment does not, so bands 3 and 4 are kept (mutation M7).
        let shape = pair::<L, W>(layout(off()), |c| c.start_ramp(BAND + 1, 0, shelf_ride));
        assert!(
            lane_bits_all::<L>(shape.0.sections[BAND + 1].coef.m0, 1.0_f32.to_bits()),
            "{width}: the enable ramp starts at m0 = 1"
        );
        expect_list(width, "identity to high shelf", &shape, &[1, 2, 3, 4, 5]);

        // The production twin of that shape: a high shelf at 0 dB designs `m0 = A^2 = 1` exactly,
        // and a ride to +6 dB moves it.
        let flat = target(true, EqBandKind::HighShelf, 3_000.0, 0.0, 0.7);
        let shape = pair::<L, W>(layout(flat), |c| {
            c.start_ramp(
                BAND + 1,
                0,
                design(EqBandKind::HighShelf, 3_000.0, 6.0, 0.7),
            );
        });
        assert!(
            lane_bits_all::<L>(shape.0.sections[BAND + 1].coef.m0, 1.0_f32.to_bits()),
            "{width}: a 0 dB high shelf has m0 = 1"
        );
        expect_list(width, "0 dB shelf ride", &shape, &[1, 2, 3, 4, 5]);

        // A lane-mixed section: a bell ramping on the left channel, a high shelf ramping on the
        // right. The dual list keeps the dead sections after it; each channel's own list is the
        // shape it carries.
        let left = {
            let mut c = channel::<L, W>(layout(bell));
            c.start_ramp(BAND + 1, 0, bell_ride);
            c
        };
        let right = {
            let mut c = channel::<L, W>(layout(shelf));
            c.start_ramp(BAND + 1, 0, shelf_ride);
            c
        };
        let io = admitted::<W>();
        let (dual, _) = lists::<L, W>(&left, &right, &io);
        assert_eq!(dual, [1, 2, 3, 4, 5], "{width}: channel-mixed section");

        // Mixed across lanes within one channel: a bell on lane 0 and a high shelf on the last
        // lane, both ramping.
        if W > 1 {
            let mut per_lane = [layout(bell); W];
            per_lane[W - 1] = layout(shelf);
            let build = || {
                let mut c = Channel::<L, W>::new(per_lane, RATE).expect("a legal channel");
                c.start_ramp(BAND + 1, 0, bell_ride);
                c.start_ramp(BAND + 1, W - 1, shelf_ride);
                c
            };
            let shape = (build(), build());
            expect_list(width, "lane-mixed section", &shape, &[1, 2, 3, 4, 5]);
            // Only the bell lane ramps: the settled shelf lane's `m0` is still not one.
            let build = || {
                let mut c = Channel::<L, W>::new(per_lane, RATE).expect("a legal channel");
                c.start_ramp(BAND + 1, 0, bell_ride);
                c
            };
            let shape = (build(), build());
            expect_list(
                width,
                "bell ramp beside a shelf lane",
                &shape,
                &[1, 2, 3, 4, 5],
            );
        }

        // The gate refuses into all six, as the stationary gate does.
        let shape = pair::<L, W>(layout(bell), |c| c.start_ramp(BAND + 1, 0, bell_ride));
        let mut io = admitted::<W>();
        io[3] = -0.0;
        let (dual, mono) = lists::<L, W>(&shape.0, &shape.1, &io);
        assert_eq!(
            (dual.len(), mono.len()),
            (6, 6),
            "{width}: a -0.0 input refuses"
        );
        let mut refused = pair::<L, W>(layout(bell), |c| c.start_ramp(BAND + 1, 0, bell_ride));
        lane_set(&mut refused.0.sections[BAND + 2].state.ic2, 0, -0.0);
        lane_set(&mut refused.1.sections[BAND + 2].state.ic2, 0, -0.0);
        let io = admitted::<W>();
        let (dual, mono) = lists::<L, W>(&refused.0, &refused.1, &io);
        assert_eq!(
            (dual.len(), mono.len()),
            (6, 6),
            "{width}: a -0.0 dead state refuses"
        );
        let mut refused = pair::<L, W>(layout(bell), |c| c.start_ramp(BAND + 1, 0, bell_ride));
        lane_set(&mut refused.0.sections[BAND].state.ic1, 0, -0.0);
        lane_set(&mut refused.1.sections[BAND].state.ic1, 0, -0.0);
        let (dual, mono) = lists::<L, W>(&refused.0, &refused.1, &io);
        assert_eq!(
            (dual.len(), mono.len()),
            (6, 6),
            "{width}: a -0.0 live state refuses"
        );

        // Nothing dead: all six, unscanned.
        let mut every = [target(true, EqBandKind::Bell, 900.0, 2.0, 1.0); EQ_SECTION_COUNT];
        every[HPF_SECTION] = target(true, EqBandKind::HighPass, 40.0, 0.0, 0.7);
        every[LPF_SECTION] = target(true, EqBandKind::LowPass, 15_000.0, 0.0, 0.7);
        let shape = pair::<L, W>(every, |c| c.start_ramp(BAND + 1, 0, bell_ride));
        expect_list(width, "all live", &shape, &[0, 1, 2, 3, 4, 5]);
    }

    /// Gate 2 at every width, dual and collapsed.
    #[test]
    fn the_unsafe_ramp_rule_keeps_every_dead_section_after_it() {
        unsafe_ramp_rule::<f32, 1>("Scalar");
        unsafe_ramp_rule::<Simd4, 4>("Simd4");
        unsafe_ramp_rule::<Simd8, 8>("Simd8");
    }

    /// A ramping identity section is not dead, on either list: the identity flag of a section
    /// in flight is stale by design, and `ramping` is what keeps it from being read (M2, M4, M6).
    fn ramping_identity_runs<L: Lane, const W: usize>(width: &str) {
        let hpf = design(EqBandKind::HighPass, 80.0, 0.0, 0.7);
        // One channel only: the other is dead there, so the section is ramping on one side.
        let mut left = channel::<L, W>(layout(off()));
        let right = channel::<L, W>(layout(off()));
        left.start_ramp(HPF_SECTION, W - 1, hpf);
        assert!(
            left.identity[HPF_SECTION],
            "{width}: the flag still says identity"
        );
        let io = admitted::<W>();
        let (dual, _) = ramping_sections::<L, W>(&left, &right, &io, &io, QUANTUM);
        assert_eq!(
            dual[..2],
            [0, 1],
            "{width}: the left ramp keeps the HPF (dual)"
        );
        let (dual, _) = ramping_sections::<L, W>(&right, &left, &io, &io, QUANTUM);
        assert_eq!(
            dual[..2],
            [0, 1],
            "{width}: the right ramp keeps the HPF (dual)"
        );
        let (mono, length) = ramping_sections_mono::<L, W>(&left, &io, QUANTUM);
        assert_eq!(
            mono[..length],
            [0, 1],
            "{width}: the ramp keeps the HPF (collapsed)"
        );
    }

    #[test]
    fn a_ramping_identity_section_is_never_dead() {
        ramping_identity_runs::<f32, 1>("Scalar");
        ramping_identity_runs::<Simd4, 4>("Simd4");
        ramping_identity_runs::<Simd8, 8>("Simd8");
    }

    /// The one restored state leg (c) as the stationary gate writes it lets past: a live high shelf
    /// at `m0 = m2 = 0.5` holding `ic1 = ic2 = -2^-149`, fed `-2^-149`, emits `-0.0` on its first
    /// frame (see [`section_state_is_flush_shaped`]). Behind a ramping HPF toggle, with every
    /// section after the shelf dead, the tightened leg refuses the list, and the block renders
    /// the batch-head bits. Red with leg (c) as the stationary gate has it (M8): the dead band
    /// after the shelf is elided and passes the `-0.0` the executed one rewrites to `+0.0`.
    fn restored_subnormal_live_state<L: Lane, const W: usize>(width: &str) {
        let magic = target(true, EqBandKind::HighShelf, 3_000.0, -6.020_6, 0.7);
        let words = magic.words(RATE).expect("a legal shelf");
        assert_eq!(
            (words.m0.to_bits(), words.m2.to_bits()),
            (0.5_f32.to_bits(), 0.5_f32.to_bits()),
            "{width}: the shelf designs m0 = m2 = 0.5"
        );
        let mut sections = layout(off());
        sections[BAND] = magic;
        let hpf = design(EqBandKind::HighPass, 80.0, 0.0, 0.7);
        let tiny = f32::from_bits(0x8000_0001);
        let build = || {
            let mut c = channel::<L, W>(sections);
            c.start_ramp(HPF_SECTION, 0, hpf);
            lane_set(&mut c.sections[BAND].state.ic1, 0, tiny);
            lane_set(&mut c.sections[BAND].state.ic2, 0, tiny);
            c
        };
        let mut io = admitted::<W>();
        io[0] = tiny;
        for mono in [false, true] {
            let mut rendered = Vec::new();
            for list in [false, true] {
                let (mut left, mut right) = (build(), build());
                let (mut l, mut r) = (io.clone(), io.clone());
                RAMPING_LIST.with(|switch| switch.set(list));
                if mono {
                    process_channels_mono::<L, W>(&mut left, &mut l, QUANTUM, false);
                } else {
                    process_channels::<L, W>(
                        (&mut left, &mut right),
                        &mut l,
                        &mut r,
                        QUANTUM,
                        false,
                    );
                }
                RAMPING_LIST.with(|switch| switch.set(false));
                let mut state = [0_u32; EQ_SECTION_COUNT * 2 * MAX_LANES];
                left.state_bits(&mut state);
                let bits: Vec<u32> = l.iter().chain(&r).map(|word| word.to_bits()).collect();
                rendered.push((bits, state));
            }
            assert!(
                rendered[0] == rendered[1],
                "{width} mono {mono}: the list moved a bit behind a restored subnormal"
            );
        }
        let (dual, mono) = lists::<L, W>(&build(), &build(), &io);
        assert_eq!(
            (dual.len(), mono.len()),
            (6, 6),
            "{width}: a restored subnormal in a live section refuses the list"
        );
        // Once the kernel has flushed the words, the list engages again.
        let mut flushed = (build(), build());
        for channel in [&mut flushed.0, &mut flushed.1] {
            lane_set(&mut channel.sections[BAND].state.ic1, 0, 0.0);
            lane_set(&mut channel.sections[BAND].state.ic2, 0, 0.0);
        }
        expect_list(width, "flushed shelf state", &flushed, &[0, 1]);
    }

    #[test]
    fn a_restored_subnormal_live_state_refuses_the_list() {
        restored_subnormal_live_state::<f32, 1>("Scalar");
        restored_subnormal_live_state::<Simd4, 4>("Simd4");
        restored_subnormal_live_state::<Simd8, 8>("Simd8");
    }

    // ---------------------------------------------------------------------------------------
    // Gate 1: the bank differential.
    // ---------------------------------------------------------------------------------------

    struct Rng(u64);

    impl Rng {
        fn next(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x >> 12;
            x ^= x << 25;
            x ^= x >> 27;
            self.0 = x;
            x.wrapping_mul(0x2545_f491_4f6c_dd1d)
        }
        fn unit(&mut self) -> f32 {
            (self.next() >> 40) as f32 / (1_u64 << 24) as f32
        }
        fn below(&mut self, n: usize) -> usize {
            (self.next() % n as u64) as usize
        }
        fn chance(&mut self, p: f32) -> bool {
            self.unit() < p
        }
        fn log(&mut self, low: f32, high: f32) -> f32 {
            math::expf(math::logf(low) + self.unit() * (math::logf(high) - math::logf(low)))
                .clamp(low, high)
        }
    }

    /// One lane's value for descriptor index `index`, from the EQ's domains, with the domain ends
    /// (VERIFY-AUTOMATION A1's boundary targets) drawn often.
    fn draw(rng: &mut Rng, index: usize) -> f32 {
        let edge = rng.chance(0.2);
        if index < 24 {
            match index % 6 {
                0 => f32::from(u8::from(rng.chance(0.6))),
                1 => (1 + rng.below(6)) as f32,
                2 if edge => [10.0, 20.0, 20_000.0][rng.below(3)],
                2 => rng.log(20.0, 18_000.0),
                3 if edge => [-24.0, 24.0, 0.0][rng.below(3)],
                3 => -24.0 + 48.0 * rng.unit(),
                4 if edge => [0.1, 18.0][rng.below(2)],
                4 => rng.log(0.2, 12.0),
                _ if edge => [0.1, 1.0][rng.below(2)],
                _ => 0.1 + 0.9 * rng.unit(),
            }
        } else {
            match index - 24 {
                0 | 3 => f32::from(u8::from(rng.chance(0.5))),
                1 | 4 if edge => [10.0, 20.0, 20_000.0][rng.below(3)],
                1 | 4 => rng.log(20.0, 18_000.0),
                _ if edge => [0.1, 18.0][rng.below(2)],
                _ => rng.log(0.2, 12.0),
            }
        }
    }

    /// Automatable descriptor indices: band frequency, gain, Q and slope, and every cut parameter.
    fn automatable(index: usize) -> bool {
        index >= 24 || index % 6 >= 2
    }

    fn request(values: &[InitialParameterValue], rate: u32) -> PrepareEffectRequest<'_> {
        PrepareEffectRequest {
            sample_rate: rate,
            quantum: QUANTUM as u32,
            quality: Quality::Normal,
            bypass: false,
            link_mode: effect_contract::LinkMode::DualMono,
            ports: effect_contract::PreparedPorts {
                sidechain: effect_contract::PreparedSidechainPort::None,
            },
            initial_values: values,
            limits: effect_contract::PrepareEffectLimits {
                maximum_total_state_bytes: 1 << 16,
                maximum_scratch_bytes: 1 << 16,
                maximum_automation_spans_per_block: 48,
            },
        }
    }

    fn hostile(rng: &mut Rng, profile: usize) -> f32 {
        match profile {
            0 => 2.0 * rng.unit() - 1.0,
            1 => 0.0,
            2 => {
                if rng.chance(0.3) {
                    -0.0
                } else {
                    2.0 * rng.unit() - 1.0
                }
            }
            3 => f32::from_bits((rng.next() as u32) & 0x807f_ffff),
            4 => {
                if rng.chance(0.05) {
                    [f32::INFINITY, f32::NEG_INFINITY, f32::NAN, 1.0e31, -3.0e38][rng.below(5)]
                } else {
                    2.0 * rng.unit() - 1.0
                }
            }
            5 => {
                if rng.chance(0.02) {
                    -0.0
                } else {
                    1.0e-3 * (2.0 * rng.unit() - 1.0)
                }
            }
            _ => {
                // Near the ceiling, inside and just outside it.
                if rng.chance(0.1) {
                    [9.0e29, -1.0e30, 1.0e-39, -1.0e-45][rng.below(4)]
                } else {
                    2.0 * rng.unit() - 1.0
                }
            }
        }
    }

    /// Every word the EQ holds, as bits: coefficients, increments, targets, integrators, the
    /// countdowns, the identity flags, the semantic targets and the fixed-point witness.
    fn fingerprint<L: Lane, const W: usize>(eq: &PreparedParametricEq<L, W>) -> Vec<u32> {
        let mut out = Vec::new();
        let mut lanes = [0_u32; MAX_LANES];
        let mut push = |value: L, out: &mut Vec<u32>| {
            value.store_bits(&mut lanes[..L::WIDTH]);
            out.extend_from_slice(&lanes[..L::WIDTH]);
        };
        for channel in [&eq.left, &eq.right] {
            for section in &channel.sections {
                for index in 0..6 {
                    push(coef_word(&section.coef, index), &mut out);
                    push(step_word(&section.step, index), &mut out);
                    push(coef_word(&section.target, index), &mut out);
                }
                push(section.state.ic1, &mut out);
                push(section.state.ic2, &mut out);
            }
            for section in &channel.remaining {
                out.extend_from_slice(section);
            }
            out.extend(channel.identity.iter().map(|flag| u32::from(*flag)));
            for track in &channel.targets {
                for band in track {
                    out.push(u32::from(band.enabled));
                    out.push(band.kind as u32);
                    out.extend(band.numeric().iter().map(|value| value.to_bits()));
                    out.push(band.slope.to_bits());
                }
            }
        }
        out.push(u32::from(eq.silent_fixed_point));
        out
    }

    /// The contract payload of every lane.
    fn payloads<L: Lane, const W: usize>(eq: &PreparedParametricEq<L, W>) -> Vec<u8> {
        let mut out = Vec::new();
        for lane in 0..W {
            let mut common = vec![0_u8; STATE_SIZES.common];
            let mut left = vec![0_u8; STATE_SIZES.left];
            let mut right = vec![0_u8; STATE_SIZES.right];
            eq.snapshot_track(
                lane,
                StatePayloadOutput {
                    common: &mut common,
                    left: &mut left,
                    right: &mut right,
                },
            )
            .expect("snapshot");
            out.extend_from_slice(&common);
            out.extend_from_slice(&left);
            out.extend_from_slice(&right);
        }
        out
    }

    /// One arm of the differential: a prepared EQ of this width driven through the contract.
    struct Arm<L: Lane, const W: usize> {
        eq: PreparedParametricEq<L, W>,
    }

    impl<L: Lane, const W: usize> Arm<L, W> {
        fn prepare(requests: &[PrepareEffectRequest<'_>]) -> Self {
            let metadata = expected_prepared_metadata(&PARAMETRIC_EQ_DESCRIPTOR, requests[0])
                .expect("metadata");
            let width = if W == 8 {
                BankWidth::Eight
            } else {
                BankWidth::Four
            };
            Self {
                eq: prepare_width::<L, W>(metadata, width, requests).expect("preparation"),
            }
        }

        /// Renders one block through the entry point this width ships, and returns the report
        /// as words. A bank goes through `process_bank` or `process_bank_mono`; the scalar
        /// instance's `process` is its bypass check, [`PreparedParametricEq::render`] and the
        /// report, so the one-lane arm calls the body it wraps (and `render_mono`, which only
        /// banks reach through the contract).
        fn render(
            &mut self,
            left: &mut [f32],
            right: &mut [f32],
            mono: bool,
            first: u64,
        ) -> Vec<u64> {
            let frames = left.len() / W;
            if W == 1 {
                let failures = if mono {
                    self.eq.render_mono(left, frames)
                } else {
                    self.eq.render(left, right, frames)
                };
                return vec![u64::from(failures[0][0]), u64::from(failures[1][0])];
            }
            let offsets = [0_u32; MAX_LANES + 1];
            let block = EffectBankProcessBlock::new(
                left,
                right,
                None,
                frames as u32,
                self.eq.bank.width,
                first,
                &[],
                &offsets[..=W],
                QUANTUM as u32,
            )
            .expect("bank block");
            let report = if mono {
                PreparedNativeEffectBank::process_bank_mono(&mut self.eq, block)
            } else {
                PreparedNativeEffectBank::process_bank(&mut self.eq, block)
            };
            report
                .reports
                .iter()
                .take(W)
                .flat_map(|entry| {
                    [
                        entry.invalid_spans,
                        entry.nonfinite_left_blocks,
                        entry.nonfinite_right_blocks,
                    ]
                })
                .collect()
        }

        fn apply(
            &mut self,
            lane: usize,
            target: &PreparedEffectTarget,
        ) -> Result<(), EffectTargetError> {
            PreparedNativeEffectBank::apply_prepared_target_lane(&mut self.eq, lane, target)
        }

        fn reset(&mut self, kind: ResetKind) {
            PreparedNativeEffectBank::reset(&mut self.eq, kind);
        }

        fn restore(
            &mut self,
            lane: usize,
            payload: &[Vec<u8>; 3],
        ) -> Result<(), StatePayloadError> {
            PreparedNativeEffectBank::restore_track_state_payload(
                &mut self.eq,
                lane as u32,
                STATE_LAYOUT_VERSION,
                StatePayloadInput {
                    common: &payload[0],
                    left: &payload[1],
                    right: &payload[2],
                },
            )
        }

        /// Whether this block is a ramping block: on either channel dual, on the one channel
        /// the collapsed body renders (its right channel is not advanced until a disengage copies
        /// the left over it).
        fn ramping(&self, mono: bool) -> bool {
            !(self.eq.left.no_ramp_in_flight() && (mono || self.eq.right.no_ramp_in_flight()))
        }
    }

    /// Encodes lane words into a contract payload.
    fn encode(left: &[u32; STATE_LANE_WORDS], right: &[u32; STATE_LANE_WORDS]) -> [Vec<u8>; 3] {
        let mut common = vec![0_u8; STATE_SIZES.common];
        let mut left_bytes = vec![0_u8; STATE_SIZES.left];
        let mut right_bytes = vec![0_u8; STATE_SIZES.right];
        write_payload(
            StatePayloadOutput {
                common: &mut common,
                left: &mut left_bytes,
                right: &mut right_bytes,
            },
            left,
            right,
        )
        .expect("encode");
        [common, left_bytes, right_bytes]
    }

    /// A restore payload for lane `lane`: the oracle's own words with one hostile edit per
    /// channel (VERIFY-AUTOMATION A1: `-0.0`, a large finite `ic2`, `NaN`, in a dead section and in
    /// a live one; and ramps into and out of the identity, which a prepared target cannot start on
    /// a general band because its enable is not automatable).
    fn hostile_payload<L: Lane, const W: usize>(
        rng: &mut Rng,
        eq: &PreparedParametricEq<L, W>,
        lane: usize,
        mono: bool,
        rate: SampleRateHz,
    ) -> ([Vec<u8>; 3], usize) {
        let mut words = [[0_u32; STATE_LANE_WORDS]; 2];
        eq.left.snapshot_track(lane, &mut words[0]);
        eq.right.snapshot_track(lane, &mut words[1]);
        let section = rng.below(EQ_SECTION_COUNT);
        let base = section * STATE_WORDS_PER_BAND;
        let edit = rng.below(10);
        let integrator = [
            -0.0_f32,
            1.0e31,
            -3.0e38,
            f32::NAN,
            1.0e-40,
            2.0e-20,
            -1.0e-20,
            f32::INFINITY,
        ][rng.below(8)];
        let which = rng.below(2);
        for (side, lane_words) in words.iter_mut().enumerate() {
            if mono && side == 1 {
                break;
            }
            match edit {
                0..=4 => lane_words[base + which] = integrator.to_bits(),
                5..=7 => {
                    // A ramp into or out of the identity: from the identity to the section's
                    // designed words when its target is live, from a high shelf to the identity
                    // when it is not.
                    let channel = if side == 0 { &eq.left } else { &eq.right };
                    let designed = channel.target_words(section, lane);
                    let (from, to) = if words_are_identity(designed) {
                        let band = channel.targets[lane][section];
                        let shelf = design_svf(
                            EqBandKind::HighShelf,
                            band.frequency.clamp(20.0, 18_000.0),
                            [24.0, -24.0, 6.0][rng.below(3)],
                            band.q.clamp(0.1, 18.0),
                            1.0,
                            rate,
                        )
                        .unwrap_or(EqSvfWords::IDENTITY);
                        (shelf, EqSvfWords::IDENTITY)
                    } else {
                        (EqSvfWords::IDENTITY, designed)
                    };
                    let samples = 1 + rng.below(RAMP_SAMPLES as usize) as u32;
                    for index in 0..6 {
                        let start = from.to_array()[index];
                        let step = (to.to_array()[index] - start) * RAMP_SCALE;
                        lane_words[base + 2 + index] = start.to_bits();
                        lane_words[base + 8 + index] = step.to_bits();
                    }
                    lane_words[base + 14] = samples;
                }
                _ => {
                    // A cut toggled through the payload: its enable word flipped, and a ramp from
                    // the words it holds to the words the flipped enable designs.
                    let (cut, kind, word) = if rng.chance(0.5) {
                        (HPF_SECTION, EqBandKind::HighPass, STATE_HPF_ENABLE_WORD)
                    } else {
                        (LPF_SECTION, EqBandKind::LowPass, STATE_HPF_ENABLE_WORD + 1)
                    };
                    let channel = if side == 0 { &eq.left } else { &eq.right };
                    let band = channel.targets[lane][cut];
                    let to = if band.enabled {
                        EqSvfWords::IDENTITY
                    } else {
                        design_svf(kind, band.frequency, 0.0, band.q, 1.0, rate)
                            .unwrap_or(EqSvfWords::IDENTITY)
                    };
                    lane_words[word] ^= 1;
                    let cut_base = cut * STATE_WORDS_PER_BAND;
                    let samples = 1 + rng.below(RAMP_SAMPLES as usize) as u32;
                    for index in 0..6 {
                        let start = f32::from_bits(lane_words[cut_base + 2 + index]);
                        let step = (to.to_array()[index] - start) * RAMP_SCALE;
                        lane_words[cut_base + 8 + index] = step.to_bits();
                    }
                    lane_words[cut_base + 14] = samples;
                }
            }
        }
        if mono {
            words[1] = words[0];
        }
        let kind = match edit {
            0..=4 => 0,
            5..=7 => 1,
            _ => 2,
        };
        (encode(&words[0], &words[1]), kind)
    }

    /// Totals over a width's scenarios.
    #[derive(Default)]
    struct Tally {
        scenarios: u64,
        blocks: u64,
        ramping: u64,
        elided: u64,
        retargets: u64,
        both_targets: u64,
        refused_targets: u64,
        restores: u64,
        refused_restores: u64,
        /// Accepted restores by edit: hostile integrator, identity ramp, cut toggle.
        restored: [u64; 3],
        resets: u64,
        rejected_blocks: u64,
    }

    #[allow(clippy::too_many_lines)]
    fn scenario<L: Lane, const W: usize>(seed: u64, mono: bool, tally: &mut Tally) {
        let mut rng = Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
        let rate = [44_100_u32, 48_000, 48_000, 96_000, 88_200][rng.below(5)];
        let parameters = PARAMETRIC_EQ_DESCRIPTOR.parameters.len();
        // Per-scenario shape: some bands off on every lane (so the list has dead sections to
        // drop), a shared kind per band most of the time, and cuts usually off.
        let band_off: [bool; EQ_BAND_COUNT] = core::array::from_fn(|_| rng.chance(0.55));
        let band_kind: [f32; EQ_BAND_COUNT] = core::array::from_fn(|_| (1 + rng.below(6)) as f32);
        let shared_kind = rng.chance(0.7);
        let cut_on: [bool; 2] = core::array::from_fn(|_| rng.chance(0.3));
        let shape = |rng: &mut Rng, index: usize| -> f32 {
            if index < 24 {
                let band = index / 6;
                match index % 6 {
                    0 => f32::from(u8::from(!band_off[band] && rng.chance(0.85))),
                    1 if shared_kind => band_kind[band],
                    _ => draw(rng, index),
                }
            } else {
                match index - 24 {
                    0 => f32::from(u8::from(cut_on[0] && rng.chance(0.8))),
                    3 => f32::from(u8::from(cut_on[1] && rng.chance(0.8))),
                    _ => draw(rng, index),
                }
            }
        };
        // Per lane: descriptor-major, left then right.
        let initial: Vec<Vec<InitialParameterValue>> = (0..W)
            .map(|_| {
                let symmetric = mono || rng.chance(0.5);
                let mut lane = Vec::with_capacity(parameters * 2);
                for index in 0..parameters {
                    let left = shape(&mut rng, index);
                    let right = if symmetric || (index < 24 && index % 6 < 2) {
                        left
                    } else {
                        shape(&mut rng, index)
                    };
                    for (channel, value) in [
                        (ParameterChannel::Left, left),
                        (ParameterChannel::Right, right),
                    ] {
                        lane.push(InitialParameterValue {
                            parameter_index: index as u32,
                            channel,
                            value,
                        });
                    }
                }
                lane
            })
            .collect();
        let requests: Vec<PrepareEffectRequest<'_>> =
            initial.iter().map(|values| request(values, rate)).collect();
        // A shape whose designs are not all legal (a frequency past the rate's range) is not a
        // scenario; the draw moves on.
        let metadata = expected_prepared_metadata(&PARAMETRIC_EQ_DESCRIPTOR, requests[0]);
        let width = if W == 8 {
            BankWidth::Eight
        } else {
            BankWidth::Four
        };
        if metadata.is_err()
            || prepare_width::<L, W>(metadata.expect("checked"), width, &requests).is_err()
        {
            return;
        }
        let mut oracle = Arm::<L, W>::prepare(&requests);
        let mut candidate = Arm::<L, W>::prepare(&requests);
        drop(requests);
        let mut values = initial.clone();
        tally.scenarios += 1;
        let factory = ParametricEqFactory;
        let mut riders: Vec<usize> = Vec::new();
        for block in 0..BLOCKS_PER_SCENARIO {
            let context = format!("W{W} seed {seed} block {block} mono {mono}");
            // A band ridden on every lane at once, now and then: bank-wide and lane-mixed ramps.
            if rng.chance(0.08) {
                riders = (0..W).collect();
            } else if rng.chance(0.2) {
                riders.clear();
            }
            if rng.chance(0.55) || !riders.is_empty() {
                let count = 1 + rng.below(3);
                let mut edits: Vec<(usize, usize)> = (0..count)
                    .map(|_| {
                        let mut index = rng.below(parameters);
                        while !automatable(index) {
                            index = rng.below(parameters);
                        }
                        (rng.below(W), index)
                    })
                    .collect();
                if !riders.is_empty() {
                    let index = 6 * rng.below(EQ_BAND_COUNT) + 3;
                    edits.extend(riders.iter().map(|&lane| (lane, index)));
                }
                for (lane, index) in edits {
                    let value = draw(&mut rng, index);
                    // Both rows in one preparation (a `Both` target when the sections agree, the
                    // SDK's shape), or one row.
                    let sides: &[usize] = if mono || rng.chance(0.5) {
                        &[0, 1]
                    } else if rng.chance(0.5) {
                        &[0]
                    } else {
                        &[1]
                    };
                    let mut changed = vec![false; parameters * 2];
                    let previous: Vec<f32> = sides
                        .iter()
                        .map(|&side| values[lane][index * 2 + side].value)
                        .collect();
                    for &side in sides {
                        values[lane][index * 2 + side].value = value;
                        changed[index * 2 + side] = true;
                    }
                    let mut targets = [PreparedEffectTarget {
                        slot: 0,
                        channel: ParameterChannel::Left,
                        words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
                    }; EQ_SECTION_COUNT * 2];
                    match factory.prepare_targets(
                        EffectTargetRequest {
                            sample_rate: rate,
                            values: &values[lane],
                            changed: &changed,
                        },
                        &mut targets,
                    ) {
                        Ok(count) => {
                            for target in &targets[..count] {
                                let a = oracle.apply(lane, target);
                                let b = candidate.apply(lane, target);
                                assert_eq!(a, b, "{context}: target application");
                                tally.retargets += 1;
                                tally.both_targets +=
                                    u64::from(target.channel == ParameterChannel::Both);
                            }
                        }
                        Err(_) => {
                            for (&side, value) in sides.iter().zip(previous) {
                                values[lane][index * 2 + side].value = value;
                            }
                        }
                    }
                }
                // A refusal word: a target whose band family is not the prepared one.
                if rng.chance(0.05) {
                    let lane = rng.below(W);
                    let band = rng.below(EQ_BAND_COUNT);
                    let mut refused = values[lane].clone();
                    let mut changed = vec![false; parameters * 2];
                    for side in 0..2 {
                        let slot = (band * 6 + 1) * 2 + side;
                        refused[slot].value = if refused[slot].value == 3.0 { 1.0 } else { 3.0 };
                        changed[slot] = true;
                    }
                    let mut targets = [PreparedEffectTarget {
                        slot: 0,
                        channel: ParameterChannel::Left,
                        words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
                    }; EQ_SECTION_COUNT * 2];
                    if let Ok(count) = factory.prepare_targets(
                        EffectTargetRequest {
                            sample_rate: rate,
                            values: &refused,
                            changed: &changed,
                        },
                        &mut targets,
                    ) {
                        for target in &targets[..count] {
                            let a = oracle.apply(lane, target);
                            let b = candidate.apply(lane, target);
                            assert_eq!(a, b, "{context}: refused target");
                            tally.refused_targets += u64::from(a.is_err());
                        }
                    }
                }
            }
            // Resets mid-ramp (VERIFY-AUTOMATION A1), about 2 % of blocks.
            if rng.chance(0.02) {
                let kind = if rng.chance(0.5) {
                    ResetKind::DiscontinuityKeepParameters
                } else {
                    values = initial.clone();
                    ResetKind::FullToDefaults
                };
                oracle.reset(kind);
                candidate.reset(kind);
                tally.resets += 1;
            }
            // Restores of hostile integrators and forged identity ramps.
            if rng.chance(0.05) {
                let lane = rng.below(W);
                let (payload, kind) =
                    hostile_payload(&mut rng, &oracle.eq, lane, mono, SampleRateHz(rate));
                let a = oracle.restore(lane, &payload);
                let b = candidate.restore(lane, &payload);
                assert_eq!(a, b, "{context}: restore");
                tally.restores += 1;
                tally.refused_restores += u64::from(a.is_err());
                tally.restored[kind] += u64::from(a.is_ok());
            }
            let frames = if rng.chance(0.6) {
                QUANTUM
            } else {
                1 + rng.below(QUANTUM)
            };
            let words = frames * W;
            let profile = if rng.chance(0.6) {
                [0, 1, 5][rng.below(3)]
            } else {
                [2, 3, 4, 6][rng.below(4)]
            };
            let mut left = vec![0.0_f32; words];
            let mut right = vec![0.0_f32; words];
            for sample in &mut left {
                *sample = hostile(&mut rng, profile);
            }
            if mono {
                right.copy_from_slice(&left);
            } else {
                for sample in &mut right {
                    *sample = hostile(&mut rng, profile);
                }
            }
            assert_eq!(
                oracle.ramping(mono),
                candidate.ramping(mono),
                "{context}: ramping"
            );
            let ramping = candidate.ramping(mono);
            let (mut oracle_left, mut oracle_right) = (left.clone(), right.clone());
            let (mut candidate_left, mut candidate_right) = (left, right);
            let first = block * QUANTUM as u64;
            RAMPING_LIST.with(|list| list.set(false));
            let oracle_report = oracle.render(&mut oracle_left, &mut oracle_right, mono, first);
            RAMPING_LIST.with(|list| list.set(true));
            let elided_before = ramping_elided_block_count();
            let candidate_report =
                candidate.render(&mut candidate_left, &mut candidate_right, mono, first);
            let elided = ramping_elided_block_count() - elided_before;
            RAMPING_LIST.with(|list| list.set(false));
            assert_eq!(oracle_report, candidate_report, "{context}: report");
            let same =
                |a: &[f32], b: &[f32]| a.iter().zip(b).all(|(x, y)| x.to_bits() == y.to_bits());
            assert!(
                same(&oracle_left, &candidate_left),
                "{context}: left output"
            );
            if !mono {
                assert!(
                    same(&oracle_right, &candidate_right),
                    "{context}: right output"
                );
            }
            assert!(
                fingerprint(&oracle.eq) == fingerprint(&candidate.eq),
                "{context}: state"
            );
            assert!(
                payloads(&oracle.eq) == payloads(&candidate.eq),
                "{context}: payload"
            );
            tally.blocks += 1;
            tally.ramping += u64::from(ramping);
            tally.elided += elided as u64;
            tally.rejected_blocks += u64::from(oracle_report.iter().any(|word| *word != 0));
        }
    }

    const BLOCKS_PER_SCENARIO: u64 = 96;

    /// Scenarios per width and body: 300 in release, 40 in a dev build (gate 1's minimums).
    fn scenarios() -> u64 {
        if cfg!(debug_assertions) { 40 } else { 300 }
    }

    fn differential<L: Lane, const W: usize>(width: &str) {
        for mono in [false, true] {
            reset_ramping_elided_blocks();
            let mut tally = Tally::default();
            let mut seed = 0;
            while tally.scenarios < scenarios() {
                scenario::<L, W>(seed + if mono { 1 << 32 } else { 0 }, mono, &mut tally);
                seed += 1;
            }
            println!(
                "#1005 differential {width} {}: {} scenarios, {} blocks bit-identical, {} ramping, \
                 {} ramping blocks elided a section; {} targets ({} Both, {} refused), {} restores \
                 ({} refused; accepted: {} hostile integrators, {} identity ramps, {} cut toggles), \
                 {} resets, {} blocks with a rejected lane",
                if mono { "collapsed" } else { "dual" },
                tally.scenarios,
                tally.blocks,
                tally.ramping,
                tally.elided,
                tally.retargets,
                tally.both_targets,
                tally.refused_targets,
                tally.restores,
                tally.refused_restores,
                tally.restored[0],
                tally.restored[1],
                tally.restored[2],
                tally.resets,
                tally.rejected_blocks,
            );
            assert!(
                tally.ramping > 0 && tally.elided > 0,
                "{width}: the list never engaged"
            );
            assert!(
                tally.both_targets > 0,
                "{width}: no Both target was applied"
            );
            assert!(
                tally.refused_restores > 0,
                "{width}: no restore was refused"
            );
            assert!(
                tally.restored.iter().all(|accepted| *accepted > 0),
                "{width}: a restore edit was never accepted"
            );
        }
    }

    #[test]
    fn a_ramping_block_renders_the_batch_head_bits_scalar() {
        differential::<f32, 1>("Scalar");
    }

    #[test]
    fn a_ramping_block_renders_the_batch_head_bits_simd4() {
        differential::<Simd4, 4>("Simd4");
    }

    #[test]
    fn a_ramping_block_renders_the_batch_head_bits_simd8() {
        differential::<Simd8, 8>("Simd8");
    }
}

/// Issue #1015: a restored subnormal integrator in a live section refuses the stationary elision.
///
/// The shape: one live high shelf at `m0 = m2 = 0.5` (gain `-6.0206` dB as an `f32`), every other
/// section dead, a restored `ic1 = ic2 = -2^-149` on the last lane, and an input word of `-2^-149`
/// there. The shelf emits `-0.0` on its first frame; an elided identity after it passes that `-0.0`
/// on, and the executed one writes `+0.0`. Before the fix the stationary cascade rendered
/// `0x80000000` where the full per-section cascade rendered `0x00000000`.
#[cfg(test)]
mod stationary_subnormal {
    use super::*;

    const RATE: SampleRateHz = SampleRateHz(48_000);
    const FRAMES: usize = 16;
    const TINY: u32 = 0x8000_0001;

    fn sections() -> [BandTarget; EQ_SECTION_COUNT] {
        let off = BandTarget {
            enabled: false,
            kind: EqBandKind::Bell,
            frequency: 1_000.0,
            gain: 0.0,
            q: 1.0,
            slope: 1.0,
        };
        let mut sections = [off; EQ_SECTION_COUNT];
        sections[BAND_SECTION_OFFSET] = BandTarget {
            enabled: true,
            kind: EqBandKind::HighShelf,
            frequency: 3_000.0,
            gain: -6.020_6,
            q: 0.7,
            slope: 1.0,
        };
        sections
    }

    /// The input: ordinary words, and `-2^-149` on the last lane's first frame.
    fn input<const W: usize>() -> Vec<f32> {
        let mut io = vec![0.25_f32; FRAMES * W];
        io[W - 1] = f32::from_bits(TINY);
        io
    }

    fn bits(words: &[f32]) -> Vec<u32> {
        words.iter().map(|word| word.to_bits()).collect()
    }

    fn state<L: Lane, const W: usize>(channel: &Channel<L, W>) -> Vec<u32> {
        let mut words = [0_u32; EQ_SECTION_COUNT * 2 * MAX_LANES];
        channel.state_bits(&mut words);
        words.to_vec()
    }

    /// The channel level: the stationary cascade against the full per-section one (the
    /// unit-test default of `process_channels(.., false)`), dual and collapsed.
    fn channel_level<L: Lane, const W: usize>(width: &str, mismatches: &mut Vec<String>) {
        let words = sections()[BAND_SECTION_OFFSET]
            .words(RATE)
            .expect("a legal shelf");
        assert_eq!(
            (words.m0.to_bits(), words.m2.to_bits()),
            (0.5_f32.to_bits(), 0.5_f32.to_bits()),
            "{width}: the shelf designs m0 = m2 = 0.5"
        );
        let build = || {
            let mut channel = Channel::<L, W>::new([sections(); W], RATE).expect("legal");
            let slot = &mut channel.sections[BAND_SECTION_OFFSET];
            lane_set(&mut slot.state.ic1, W - 1, f32::from_bits(TINY));
            lane_set(&mut slot.state.ic2, W - 1, f32::from_bits(TINY));
            channel
        };
        for mono in [false, true] {
            let mut arms = Vec::new();
            for stationary in [true, false] {
                let (mut left, mut right) = (build(), build());
                let (mut l, mut r) = (input::<W>(), input::<W>());
                if mono {
                    process_channels_mono::<L, W>(&mut left, &mut l, FRAMES, stationary);
                } else {
                    process_channels::<L, W>(
                        (&mut left, &mut right),
                        &mut l,
                        &mut r,
                        FRAMES,
                        stationary,
                    );
                }
                arms.push((bits(&l), bits(&r), state(&left), state(&right)));
            }
            let (elided, full) = (&arms[0], &arms[1]);
            if elided != full {
                mismatches.push(format!(
                    "{width} mono {mono}: the stationary cascade rendered {:08x} where the full \
                     cascade rendered {:08x}",
                    elided.0[W - 1],
                    full.0[W - 1]
                ));
            }
        }
    }

    /// The contract level: the payload restores (the shape is reachable), and the bank renders
    /// the full cascade's bits through `process_bank` / `process_bank_mono` (or, for the scalar
    /// instance, the `render` that `process` wraps).
    fn contract_level<L: Lane, const W: usize>(width: &str, mismatches: &mut Vec<String>) {
        let mut values: Vec<InitialParameterValue> =
            effect_contract::default_initial_values(&PARAMETRIC_EQ_DESCRIPTOR).collect();
        for (field, value) in [(0, 1.0), (1, 3.0), (2, 3_000.0), (3, -6.020_6), (4, 0.7)] {
            for side in 0..2 {
                values[field * 2 + side].value = value;
            }
        }
        let request = PrepareEffectRequest {
            sample_rate: RATE.0,
            quantum: 128,
            quality: Quality::Normal,
            bypass: false,
            link_mode: effect_contract::LinkMode::DualMono,
            ports: effect_contract::PreparedPorts {
                sidechain: effect_contract::PreparedSidechainPort::None,
            },
            initial_values: &values,
            limits: effect_contract::PrepareEffectLimits {
                maximum_total_state_bytes: 1 << 16,
                maximum_scratch_bytes: 1 << 16,
                maximum_automation_spans_per_block: 48,
            },
        };
        let requests = vec![request; W];
        let metadata =
            expected_prepared_metadata(&PARAMETRIC_EQ_DESCRIPTOR, request).expect("metadata");
        let width_tag = if W == 8 {
            BankWidth::Eight
        } else {
            BankWidth::Four
        };
        let prepare = || prepare_width::<L, W>(metadata, width_tag, &requests).expect("prepared");
        let lane = W - 1;
        let payload = {
            let eq = prepare();
            let mut words = [[0_u32; STATE_LANE_WORDS]; 2];
            eq.left.snapshot_track(lane, &mut words[0]);
            eq.right.snapshot_track(lane, &mut words[1]);
            for channel in &mut words {
                let base = BAND_SECTION_OFFSET * STATE_WORDS_PER_BAND;
                channel[base] = TINY;
                channel[base + 1] = TINY;
            }
            let mut common = vec![0_u8; STATE_SIZES.common];
            let mut left = vec![0_u8; STATE_SIZES.left];
            let mut right = vec![0_u8; STATE_SIZES.right];
            write_payload(
                StatePayloadOutput {
                    common: &mut common,
                    left: &mut left,
                    right: &mut right,
                },
                &words[0],
                &words[1],
            )
            .expect("encode");
            [common, left, right]
        };
        let restored = || {
            let mut eq = prepare();
            PreparedNativeEffectBank::restore_track_state_payload(
                &mut eq,
                lane as u32,
                STATE_LAYOUT_VERSION,
                StatePayloadInput {
                    common: &payload[0],
                    left: &payload[1],
                    right: &payload[2],
                },
            )
            .expect("a finite subnormal integrator restores");
            eq
        };
        for mono in [false, true] {
            let mut shipped = restored();
            let mut full = restored();
            let (mut l, mut r) = (input::<W>(), input::<W>());
            if W == 1 {
                if mono {
                    shipped.render_mono(&mut l, FRAMES);
                } else {
                    shipped.render(&mut l, &mut r, FRAMES);
                }
            } else {
                let offsets = [0_u32; MAX_LANES + 1];
                let block = EffectBankProcessBlock::new(
                    &mut l,
                    &mut r,
                    None,
                    FRAMES as u32,
                    width_tag,
                    0,
                    &[],
                    &offsets[..=W],
                    128,
                )
                .expect("block");
                if mono {
                    PreparedNativeEffectBank::process_bank_mono(&mut shipped, block);
                } else {
                    PreparedNativeEffectBank::process_bank(&mut shipped, block);
                }
            }
            let (mut fl, mut fr) = (input::<W>(), input::<W>());
            if mono {
                process_channels_mono::<L, W>(&mut full.left, &mut fl, FRAMES, false);
            } else {
                process_channels::<L, W>(
                    (&mut full.left, &mut full.right),
                    &mut fl,
                    &mut fr,
                    FRAMES,
                    false,
                );
            }
            if !(bits(&l) == bits(&fl)
                && (mono || bits(&r) == bits(&fr))
                && state(&shipped.left) == state(&full.left)
                && (mono || state(&shipped.right) == state(&full.right)))
            {
                mismatches.push(format!(
                    "{width} mono {mono}: the bank rendered {:08x} where the full cascade \
                     rendered {:08x}",
                    l[W - 1].to_bits(),
                    fl[W - 1].to_bits()
                ));
            }
        }
    }

    #[test]
    fn a_restored_subnormal_live_state_renders_the_full_cascade_bits() {
        let mut mismatches = Vec::new();
        channel_level::<f32, 1>("Scalar", &mut mismatches);
        channel_level::<Simd4, 4>("Simd4", &mut mismatches);
        channel_level::<Simd8, 8>("Simd8", &mut mismatches);
        assert!(mismatches.is_empty(), "{mismatches:#?}");
    }

    #[test]
    fn a_restored_subnormal_live_state_renders_the_full_cascade_bits_through_the_contract() {
        let mut mismatches = Vec::new();
        contract_level::<f32, 1>("Scalar", &mut mismatches);
        contract_level::<Simd4, 4>("Simd4", &mut mismatches);
        contract_level::<Simd8, 8>("Simd8", &mut mismatches);
        assert!(mismatches.is_empty(), "{mismatches:#?}");
    }
}
