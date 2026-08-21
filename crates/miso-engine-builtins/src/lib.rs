//! Scalar fixed track processors and transparent meter state for issue 007.
#![allow(missing_docs)]

use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};

use miso_engine_core::{
    SampleRateHz, is_extended_compatibility_sample_rate,
    realtime::{Consumer, Producer, QueueGeneration, bounded_spsc},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChannelLinkMode {
    DualMono,
    ExplicitMatrix2x2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltinResetKind {
    FullToPrepared,
    DiscontinuityKeepTargets,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix2x2 {
    pub ll: f32,
    pub lr: f32,
    pub rl: f32,
    pub rr: f32,
}

impl Matrix2x2 {
    pub const IDENTITY: Self = Self {
        ll: 1.0,
        lr: 0.0,
        rl: 0.0,
        rr: 1.0,
    };

    pub fn checked(self) -> Result<Self, BuiltinParameterError> {
        if [self.ll, self.lr, self.rl, self.rr]
            .into_iter()
            .all(|v| v.is_finite() && (-1.0..=1.0).contains(&v))
        {
            Ok(Self {
                ll: zero(self.ll),
                lr: zero(self.lr),
                rl: zero(self.rl),
                rr: zero(self.rr),
            })
        } else {
            Err(BuiltinParameterError::MatrixCoefficient)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltinParameterError {
    EmptyBlock,
    LaneLength,
    SampleTimeOverflow,
    GainDomain,
    FilterCutoff,
    FilterOrder,
    FilterCoefficients,
    MatrixCoefficient,
    MatrixSmoothing,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BuiltinProcessReport {
    pub sanitized_input: u64,
    pub sanitized_output: u64,
    pub recovered_left_state: u64,
    pub recovered_right_state: u64,
}

impl BuiltinProcessReport {
    fn add(&mut self, other: Self) {
        self.sanitized_input = self.sanitized_input.saturating_add(other.sanitized_input);
        self.sanitized_output = self.sanitized_output.saturating_add(other.sanitized_output);
        self.recovered_left_state = self
            .recovered_left_state
            .saturating_add(other.recovered_left_state);
        self.recovered_right_state = self
            .recovered_right_state
            .saturating_add(other.recovered_right_state);
    }
}

/// A shape-validated dual-mono render block.
///
/// The constructor validates the two lanes and sample-time range before any processor can mutate
/// audio. Each public processor repeats the inexpensive validation as an internal invariant.
pub struct DualMonoBlock<'a> {
    left: &'a mut [f32],
    right: &'a mut [f32],
    first_sample: u64,
}

impl<'a> DualMonoBlock<'a> {
    pub fn new(
        left: &'a mut [f32],
        right: &'a mut [f32],
        first_sample: u64,
    ) -> Result<Self, BuiltinParameterError> {
        let block = DualMonoBlock {
            left,
            right,
            first_sample,
        };
        block.checked_len()?;
        Ok(block)
    }

    fn checked_len(&self) -> Result<usize, BuiltinParameterError> {
        if self.left.is_empty() {
            return Err(BuiltinParameterError::EmptyBlock);
        }
        if self.left.len() != self.right.len() {
            return Err(BuiltinParameterError::LaneLength);
        }
        let len = u64::try_from(self.left.len())
            .map_err(|_| BuiltinParameterError::SampleTimeOverflow)?;
        self.first_sample
            .checked_add(len)
            .ok_or(BuiltinParameterError::SampleTimeOverflow)?;
        Ok(self.left.len())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChannelParameters {
    pub polarity_invert: bool,
    pub trim_db: f32,
    pub hpf_hz: f32,
    pub lpf_hz: f32,
    pub fader_db: f32,
    pub muted: bool,
}

impl Default for ChannelParameters {
    fn default() -> Self {
        Self {
            polarity_invert: false,
            trim_db: 0.0,
            hpf_hz: 0.0,
            lpf_hz: 0.0,
            fader_db: 0.0,
            muted: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BuiltinParameters {
    pub left: ChannelParameters,
    pub right: ChannelParameters,
    pub matrix: Matrix2x2,
    pub smoothing_samples: u32,
}

impl Default for BuiltinParameters {
    fn default() -> Self {
        Self {
            left: ChannelParameters::default(),
            right: ChannelParameters::default(),
            matrix: Matrix2x2::IDENTITY,
            smoothing_samples: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltinTail {
    FiniteZero,
    Infinite,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BuiltinParameterDescriptorV1 {
    pub id: u32,
    pub name: &'static str,
    /// The state ownership boundary for this stable parameter ID.
    pub scope: BuiltinParameterScope,
    /// The semantic mapping used to interpret the stored `f32` value.
    pub mapping: BuiltinParameterMapping,
    /// The finite, rate-aware domain accepted during preparation.
    pub domain: BuiltinParameterDomain,
    pub default: f32,
    pub update_rate: BuiltinParameterUpdateRate,
    pub smoothing: BuiltinSmoothingPolicy,
    pub reset: BuiltinParameterReset,
    pub disabled_value: Option<f32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltinParameterScope {
    PerLane,
    MatrixShared,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltinParameterMapping {
    /// Exact numeric encodings `0.0` for false and `1.0` for true.
    Boolean,
    /// Amplitude gain mapping `10^(dB / 20)`.
    DecibelAmplitude,
    Hertz,
    Linear,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BuiltinParameterDomain {
    /// Only the exact false and true encodings are accepted.
    BooleanExact,
    /// A finite inclusive numeric range.
    FiniteInclusive { minimum: f32, maximum: f32 },
    /// Version 1 cutoff contract: exact zero disables; enabled values are bounded by the
    /// representable maximum recorded for the prepared launch sample rate.
    DisabledOrRateKeyedHertzV1 { disabled: f32, minimum_hz: f32 },
}

impl BuiltinParameterDomain {
    /// Validate a value against this descriptor's prepared sample-rate contract.
    #[must_use]
    pub fn contains(self, value: f32, sample_rate: u32) -> bool {
        match self {
            Self::BooleanExact => {
                value.to_bits() == 0.0_f32.to_bits() || value.to_bits() == 1.0_f32.to_bits()
            }
            Self::FiniteInclusive { minimum, maximum } => {
                value.is_finite() && value >= minimum && value <= maximum
            }
            Self::DisabledOrRateKeyedHertzV1 {
                disabled,
                minimum_hz,
            } => {
                validate_builtin_filter_cutoff_v1(value, sample_rate, disabled, minimum_hz).is_ok()
            }
        }
    }
}

/// The exact, inclusive cutoff maximum for one launch rate under the retained `f32` TPT state.
///
/// These are greatest contiguous shared HPF/LPF maxima.  The immediate successor of each value
/// is deliberately outside the public prepared domain.
#[must_use]
pub const fn builtin_filter_cutoff_maximum_hz_v1(sample_rate: u32) -> Option<f32> {
    match sample_rate {
        44_100 => Some(f32::from_bits(0x46ac_42f7)),
        48_000 => Some(f32::from_bits(0x46bb_7ede)),
        88_200 => Some(f32::from_bits(0x472c_42f7)),
        96_000 => Some(f32::from_bits(0x473b_7ede)),
        _ => None,
    }
}

/// Validate the V1 public/preparation cutoff contract without entering coefficient preparation.
///
/// Session compilation rejects unsupported rates before builtins preparation. For the retained
/// direct TPT compatibility checks at extended research rates, the helper keeps their previous
/// finite open-Nyquist mathematical domain; it does not expand the launch descriptor contract.
pub fn validate_builtin_filter_cutoff_v1(
    value: f32,
    sample_rate: u32,
    disabled: f32,
    minimum_hz: f32,
) -> Result<(), BuiltinParameterError> {
    let launch_maximum = builtin_filter_cutoff_maximum_hz_v1(sample_rate);
    let is_extended_compatibility =
        is_extended_compatibility_sample_rate(SampleRateHz(sample_rate));
    if launch_maximum.is_none() && !is_extended_compatibility {
        return Err(BuiltinParameterError::FilterCutoff);
    }
    if value.to_bits() == disabled.to_bits() {
        return Ok(());
    }
    match launch_maximum {
        Some(maximum_hz) if value.is_finite() && value >= minimum_hz && value <= maximum_hz => {
            Ok(())
        }
        None if value.is_finite()
            && value >= minimum_hz
            && f64::from(value) < f64::from(sample_rate) * 0.5 =>
        {
            Ok(())
        }
        _ => Err(BuiltinParameterError::FilterCutoff),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltinParameterUpdateRate {
    PreparedOnly,
    BlockTarget,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltinSmoothingPolicy {
    None,
    /// Exact linear interpolation over the requested number of sample updates.
    LinearNUpdates,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltinParameterReset {
    RestorePreparedValue,
    KeepTargetResetCurrent,
}

pub const BUILTIN_PARAMETER_DESCRIPTORS_V1: [BuiltinParameterDescriptorV1; 10] = [
    BuiltinParameterDescriptorV1 {
        id: 1,
        name: "polarity_invert",
        scope: BuiltinParameterScope::PerLane,
        mapping: BuiltinParameterMapping::Boolean,
        domain: BuiltinParameterDomain::BooleanExact,
        default: 0.0,
        update_rate: BuiltinParameterUpdateRate::PreparedOnly,
        smoothing: BuiltinSmoothingPolicy::None,
        reset: BuiltinParameterReset::RestorePreparedValue,
        disabled_value: None,
    },
    BuiltinParameterDescriptorV1 {
        id: 2,
        name: "trim_db",
        scope: BuiltinParameterScope::PerLane,
        mapping: BuiltinParameterMapping::DecibelAmplitude,
        domain: BuiltinParameterDomain::FiniteInclusive {
            minimum: -144.0,
            maximum: 24.0,
        },
        default: 0.0,
        update_rate: BuiltinParameterUpdateRate::PreparedOnly,
        smoothing: BuiltinSmoothingPolicy::None,
        reset: BuiltinParameterReset::RestorePreparedValue,
        disabled_value: None,
    },
    BuiltinParameterDescriptorV1 {
        id: 3,
        name: "hpf_hz",
        scope: BuiltinParameterScope::PerLane,
        mapping: BuiltinParameterMapping::Hertz,
        domain: BuiltinParameterDomain::DisabledOrRateKeyedHertzV1 {
            disabled: 0.0,
            minimum_hz: 10.0,
        },
        default: 0.0,
        update_rate: BuiltinParameterUpdateRate::PreparedOnly,
        smoothing: BuiltinSmoothingPolicy::None,
        reset: BuiltinParameterReset::RestorePreparedValue,
        disabled_value: Some(0.0),
    },
    BuiltinParameterDescriptorV1 {
        id: 4,
        name: "lpf_hz",
        scope: BuiltinParameterScope::PerLane,
        mapping: BuiltinParameterMapping::Hertz,
        domain: BuiltinParameterDomain::DisabledOrRateKeyedHertzV1 {
            disabled: 0.0,
            minimum_hz: 10.0,
        },
        default: 0.0,
        update_rate: BuiltinParameterUpdateRate::PreparedOnly,
        smoothing: BuiltinSmoothingPolicy::None,
        reset: BuiltinParameterReset::RestorePreparedValue,
        disabled_value: Some(0.0),
    },
    BuiltinParameterDescriptorV1 {
        id: 5,
        name: "fader_db",
        scope: BuiltinParameterScope::PerLane,
        mapping: BuiltinParameterMapping::DecibelAmplitude,
        domain: BuiltinParameterDomain::FiniteInclusive {
            minimum: -144.0,
            maximum: 24.0,
        },
        default: 0.0,
        update_rate: BuiltinParameterUpdateRate::PreparedOnly,
        smoothing: BuiltinSmoothingPolicy::None,
        reset: BuiltinParameterReset::RestorePreparedValue,
        disabled_value: None,
    },
    BuiltinParameterDescriptorV1 {
        id: 6,
        name: "mute",
        scope: BuiltinParameterScope::PerLane,
        mapping: BuiltinParameterMapping::Boolean,
        domain: BuiltinParameterDomain::BooleanExact,
        default: 0.0,
        update_rate: BuiltinParameterUpdateRate::PreparedOnly,
        smoothing: BuiltinSmoothingPolicy::None,
        reset: BuiltinParameterReset::RestorePreparedValue,
        disabled_value: None,
    },
    BuiltinParameterDescriptorV1 {
        id: 7,
        name: "matrix_ll",
        scope: BuiltinParameterScope::MatrixShared,
        mapping: BuiltinParameterMapping::Linear,
        domain: BuiltinParameterDomain::FiniteInclusive {
            minimum: -1.0,
            maximum: 1.0,
        },
        default: 1.0,
        update_rate: BuiltinParameterUpdateRate::BlockTarget,
        smoothing: BuiltinSmoothingPolicy::LinearNUpdates,
        reset: BuiltinParameterReset::KeepTargetResetCurrent,
        disabled_value: None,
    },
    BuiltinParameterDescriptorV1 {
        id: 8,
        name: "matrix_lr",
        scope: BuiltinParameterScope::MatrixShared,
        mapping: BuiltinParameterMapping::Linear,
        domain: BuiltinParameterDomain::FiniteInclusive {
            minimum: -1.0,
            maximum: 1.0,
        },
        default: 0.0,
        update_rate: BuiltinParameterUpdateRate::BlockTarget,
        smoothing: BuiltinSmoothingPolicy::LinearNUpdates,
        reset: BuiltinParameterReset::KeepTargetResetCurrent,
        disabled_value: None,
    },
    BuiltinParameterDescriptorV1 {
        id: 9,
        name: "matrix_rl",
        scope: BuiltinParameterScope::MatrixShared,
        mapping: BuiltinParameterMapping::Linear,
        domain: BuiltinParameterDomain::FiniteInclusive {
            minimum: -1.0,
            maximum: 1.0,
        },
        default: 0.0,
        update_rate: BuiltinParameterUpdateRate::BlockTarget,
        smoothing: BuiltinSmoothingPolicy::LinearNUpdates,
        reset: BuiltinParameterReset::KeepTargetResetCurrent,
        disabled_value: None,
    },
    BuiltinParameterDescriptorV1 {
        id: 10,
        name: "matrix_rr",
        scope: BuiltinParameterScope::MatrixShared,
        mapping: BuiltinParameterMapping::Linear,
        domain: BuiltinParameterDomain::FiniteInclusive {
            minimum: -1.0,
            maximum: 1.0,
        },
        default: 1.0,
        update_rate: BuiltinParameterUpdateRate::BlockTarget,
        smoothing: BuiltinSmoothingPolicy::LinearNUpdates,
        reset: BuiltinParameterReset::KeepTargetResetCurrent,
        disabled_value: None,
    },
];

#[derive(Clone, Copy)]
struct TptSvf {
    c1: f32,
    a2: f32,
    a3: f32,
    k: f32,
    s1: f32,
    s2: f32,
    high_pass: bool,
    enabled: bool,
}

impl TptSvf {
    fn identity() -> Self {
        Self {
            c1: 0.0,
            a2: 0.0,
            a3: 0.0,
            k: 0.0,
            s1: 0.0,
            s2: 0.0,
            high_pass: false,
            enabled: false,
        }
    }
    fn design(rate: u32, cutoff: f32, high_pass: bool) -> Result<Self, BuiltinParameterError> {
        if cutoff == 0.0 {
            return Ok(Self::identity());
        }
        let g = (core::f64::consts::PI * f64::from(cutoff) / f64::from(rate)).tan();
        let k64 = core::f64::consts::SQRT_2;
        let t0 = g + k64;
        let t1 = g * t0;
        let denominator = 1.0 + t1;
        let c1 = t1 / denominator;
        let a2 = g / denominator;
        let t2 = g * g;
        let a3 = t2 / denominator;
        let values = [c1, a2, a3, k64].map(|value| value as f32);
        if !values.into_iter().all(normal_or_zero) {
            return Err(BuiltinParameterError::FilterCoefficients);
        }
        let [c1, a2, a3, k] = values;
        let transition_00 = 1.0 - 2.0_f64 * f64::from(c1);
        let transition_01 = -2.0_f64 * f64::from(a2);
        let transition_10 = 2.0_f64 * f64::from(a2);
        let transition_11 = 1.0 - 2.0_f64 * f64::from(a3);
        let trace = transition_00 + transition_11;
        let determinant = transition_00 * transition_11 - transition_01 * transition_10;
        let denominator_a1 = -trace;
        let denominator_a2 = determinant;
        if denominator_a2.abs() >= 1.0
            || 1.0 + denominator_a1 + denominator_a2 <= 0.0
            || 1.0 - denominator_a1 + denominator_a2 <= 0.0
        {
            return Err(BuiltinParameterError::FilterCoefficients);
        }
        let cutoff_db = cast_tpt_magnitude_db(rate, cutoff, c1, a2, a3, k, high_pass)
            .ok_or(BuiltinParameterError::FilterCoefficients)?;
        if (cutoff_db + 3.010_299_956_6).abs() > 0.005 {
            return Err(BuiltinParameterError::FilterCoefficients);
        }
        Ok(Self {
            c1,
            a2,
            a3,
            k,
            s1: 0.0,
            s2: 0.0,
            high_pass,
            enabled: true,
        })
    }
    fn reset(&mut self) {
        self.s1 = 0.0;
        self.s2 = 0.0;
    }
    fn process(
        &mut self,
        input: f32,
        recovered: &mut u64,
        report: &mut BuiltinProcessReport,
    ) -> f32 {
        if !self.enabled {
            return input;
        }
        if !normal_or_zero(self.s1) || !normal_or_zero(self.s2) {
            self.reset();
            *recovered = recovered.saturating_add(1);
        }
        let v3 = input - self.s2;
        let p1 = self.a2 * v3;
        let p2 = self.c1 * self.s1;
        let d1 = p1 - p2;
        let v1 = self.s1 + d1;
        let p3 = self.a2 * self.s1;
        let p4 = self.a3 * v3;
        let d2 = p3 + p4;
        let v2 = self.s2 + d2;
        let q1 = d1 + d1;
        let n1 = self.s1 + q1;
        let q2 = d2 + d2;
        let n2 = self.s2 + q2;
        let low = v2;
        let kh = self.k * v1;
        let th = input - kh;
        let high = th - v2;
        if !normal_or_zero(n1) || !normal_or_zero(n2) {
            self.reset();
            *recovered = recovered.saturating_add(1);
            return 0.0;
        }
        self.s1 = n1;
        self.s2 = n2;
        sanitize(
            if self.high_pass { high } else { low },
            &mut report.sanitized_output,
        )
    }
}

/// Evaluate the exact-real state-space response implied by the stored `f32` coefficient bits.
///
/// This runs only during preparation.  It deliberately does not call the test/reference crate:
/// the production compiler must reject a cast coefficient set that misses the frozen cutoff gate
/// even when conformance tests are not linked into the host.
#[allow(clippy::too_many_arguments)]
fn cast_tpt_magnitude_db(
    rate: u32,
    frequency: f32,
    c1: f32,
    a2: f32,
    a3: f32,
    k: f32,
    high_pass: bool,
) -> Option<f64> {
    let (c1, a2, a3, k) = (f64::from(c1), f64::from(a2), f64::from(a3), f64::from(k));
    let a00 = 1.0 - 2.0 * c1;
    let a01 = -2.0 * a2;
    let a10 = 2.0 * a2;
    let a11 = 1.0 - 2.0 * a3;
    let b0 = 2.0 * a2;
    let b1 = 2.0 * a3;
    let (output_c0, output_c1, direct) = if high_pass {
        (-k * (1.0 - c1) - a2, k * a2 - (1.0 - a3), 1.0 - k * a2 - a3)
    } else {
        (a2, 1.0 - a3, a3)
    };

    let phase = core::f64::consts::TAU * f64::from(frequency) / f64::from(rate);
    let (z_real, z_imaginary) = (phase.cos(), phase.sin());
    let m00_real = z_real - a00;
    let m11_real = z_real - a11;
    let m01_real = -a01;
    let m10_real = -a10;
    let determinant_real = m00_real * m11_real - z_imaginary * z_imaginary - m01_real * m10_real;
    let determinant_imaginary = z_imaginary * (m00_real + m11_real);
    let determinant_norm =
        determinant_real * determinant_real + determinant_imaginary * determinant_imaginary;
    if determinant_norm == 0.0 || !determinant_norm.is_finite() {
        return None;
    }

    let state0_numerator_real = m11_real * b0 - m01_real * b1;
    let state0_numerator_imaginary = z_imaginary * b0;
    let state1_numerator_real = -m10_real * b0 + m00_real * b1;
    let state1_numerator_imaginary = z_imaginary * b1;
    let divide = |numerator_real: f64, numerator_imaginary: f64| {
        (
            (numerator_real * determinant_real + numerator_imaginary * determinant_imaginary)
                / determinant_norm,
            (numerator_imaginary * determinant_real - numerator_real * determinant_imaginary)
                / determinant_norm,
        )
    };
    let (state0_real, state0_imaginary) = divide(state0_numerator_real, state0_numerator_imaginary);
    let (state1_real, state1_imaginary) = divide(state1_numerator_real, state1_numerator_imaginary);
    let response_real = direct + output_c0 * state0_real + output_c1 * state1_real;
    let response_imaginary = output_c0 * state0_imaginary + output_c1 * state1_imaginary;
    let magnitude = response_real.hypot(response_imaginary);
    if magnitude.is_finite() && magnitude > 0.0 {
        Some(20.0 * magnitude.log10())
    } else {
        None
    }
}

#[derive(Clone, Copy)]
struct InputLane {
    polarity: bool,
    trim: f32,
    hpf: TptSvf,
    lpf: TptSvf,
}
#[derive(Clone, Copy)]
struct FaderLane {
    gain: f32,
    muted: bool,
}

pub struct InputBuiltins {
    left: InputLane,
    right: InputLane,
    lifetime_recovered_left: u64,
    lifetime_recovered_right: u64,
}
pub struct FaderMuteBuiltins {
    left: FaderLane,
    right: FaderLane,
}
pub struct MatrixBuiltins {
    current: Matrix2x2,
    target: Matrix2x2,
    smoothing_samples: u32,
    remaining_updates: u32,
}

pub struct BuiltinChain {
    input: InputBuiltins,
    fader_mute: FaderMuteBuiltins,
    matrix: MatrixBuiltins,
}

impl BuiltinChain {
    pub fn new(
        sample_rate: u32,
        parameters: BuiltinParameters,
    ) -> Result<Self, BuiltinParameterError> {
        let (input, fader_mute, matrix) = prepare_sections(sample_rate, parameters)?;
        Ok(Self {
            input,
            fader_mute,
            matrix,
        })
    }
    pub fn process_input(
        &mut self,
        block: DualMonoBlock<'_>,
    ) -> Result<BuiltinProcessReport, BuiltinParameterError> {
        self.input.process(block)
    }
    pub fn process_fader_mute(
        &mut self,
        block: DualMonoBlock<'_>,
    ) -> Result<BuiltinProcessReport, BuiltinParameterError> {
        self.fader_mute.process(block)
    }
    pub fn process_matrix(
        &mut self,
        block: DualMonoBlock<'_>,
    ) -> Result<BuiltinProcessReport, BuiltinParameterError> {
        self.matrix.process(block)
    }
    pub fn process_dual_mono(
        &mut self,
        block: DualMonoBlock<'_>,
    ) -> Result<BuiltinProcessReport, BuiltinParameterError> {
        block.checked_len()?;
        let DualMonoBlock {
            left,
            right,
            first_sample,
        } = block;
        let mut report = self.input.process(DualMonoBlock {
            left,
            right,
            first_sample,
        })?;
        report.add(self.fader_mute.process(DualMonoBlock {
            left,
            right,
            first_sample,
        })?);
        report.add(self.matrix.process(DualMonoBlock {
            left,
            right,
            first_sample,
        })?);
        Ok(report)
    }
    pub fn set_matrix_target(&mut self, target: Matrix2x2) -> Result<(), BuiltinParameterError> {
        self.matrix.set_target(target)
    }
    pub fn reset(&mut self, kind: BuiltinResetKind) {
        self.input.reset();
        self.matrix.reset();
        if matches!(kind, BuiltinResetKind::FullToPrepared) {
            self.fader_mute.reset();
        }
    }
    pub fn link_mode(&self) -> ChannelLinkMode {
        ChannelLinkMode::ExplicitMatrix2x2
    }
    pub fn tail(&self) -> BuiltinTail {
        self.input.tail()
    }
    pub fn into_sections(self) -> (InputBuiltins, FaderMuteBuiltins, MatrixBuiltins) {
        (self.input, self.fader_mute, self.matrix)
    }
}

fn prepare_sections(
    sample_rate: u32,
    parameters: BuiltinParameters,
) -> Result<(InputBuiltins, FaderMuteBuiltins, MatrixBuiltins), BuiltinParameterError> {
    if sample_rate == 0 {
        return Err(BuiltinParameterError::FilterCutoff);
    }
    parameters.matrix.checked()?;
    for lane in [parameters.left, parameters.right] {
        if !lane.trim_db.is_finite()
            || !(-144.0..=24.0).contains(&lane.trim_db)
            || !lane.fader_db.is_finite()
            || !(-144.0..=24.0).contains(&lane.fader_db)
        {
            return Err(BuiltinParameterError::GainDomain);
        }
        validate_builtin_filter_cutoff_v1(lane.hpf_hz, sample_rate, 0.0, 10.0)?;
        validate_builtin_filter_cutoff_v1(lane.lpf_hz, sample_rate, 0.0, 10.0)?;
        if lane.hpf_hz > 0.0 && lane.lpf_hz > 0.0 && lane.hpf_hz >= lane.lpf_hz {
            return Err(BuiltinParameterError::FilterOrder);
        }
    }
    let lane = |params: ChannelParameters| -> Result<InputLane, BuiltinParameterError> {
        Ok(InputLane {
            polarity: params.polarity_invert,
            trim: db_gain(params.trim_db)?,
            hpf: TptSvf::design(sample_rate, zero(params.hpf_hz), true)?,
            lpf: TptSvf::design(sample_rate, zero(params.lpf_hz), false)?,
        })
    };
    let fader = |params: ChannelParameters| -> Result<FaderLane, BuiltinParameterError> {
        Ok(FaderLane {
            gain: db_gain(params.fader_db)?,
            muted: params.muted,
        })
    };
    Ok((
        InputBuiltins {
            left: lane(parameters.left)?,
            right: lane(parameters.right)?,
            lifetime_recovered_left: 0,
            lifetime_recovered_right: 0,
        },
        FaderMuteBuiltins {
            left: fader(parameters.left)?,
            right: fader(parameters.right)?,
        },
        MatrixBuiltins {
            current: parameters.matrix,
            target: parameters.matrix,
            smoothing_samples: parameters.smoothing_samples,
            remaining_updates: 0,
        },
    ))
}

impl InputBuiltins {
    pub fn process(
        &mut self,
        block: DualMonoBlock<'_>,
    ) -> Result<BuiltinProcessReport, BuiltinParameterError> {
        let mut report = BuiltinProcessReport::default();
        block.checked_len()?;
        let mut recovered_left = 0;
        let mut recovered_right = 0;
        for (left, right) in block.left.iter_mut().zip(block.right.iter_mut()) {
            *left = process_input_lane(&mut self.left, *left, &mut recovered_left, &mut report);
            *right = process_input_lane(&mut self.right, *right, &mut recovered_right, &mut report);
        }
        self.lifetime_recovered_left = self.lifetime_recovered_left.saturating_add(recovered_left);
        self.lifetime_recovered_right = self
            .lifetime_recovered_right
            .saturating_add(recovered_right);
        report.recovered_left_state = recovered_left;
        report.recovered_right_state = recovered_right;
        Ok(report)
    }
    pub fn reset(&mut self) {
        self.left.hpf.reset();
        self.left.lpf.reset();
        self.right.hpf.reset();
        self.right.lpf.reset();
    }
    pub fn tail(&self) -> BuiltinTail {
        if self.left.hpf.enabled
            || self.left.lpf.enabled
            || self.right.hpf.enabled
            || self.right.lpf.enabled
        {
            BuiltinTail::Infinite
        } else {
            BuiltinTail::FiniteZero
        }
    }
    pub fn lifetime_recovered_state(&self) -> (u64, u64) {
        (self.lifetime_recovered_left, self.lifetime_recovered_right)
    }
    pub fn reset_lifetime_recovered_state(&mut self) {
        self.lifetime_recovered_left = 0;
        self.lifetime_recovered_right = 0;
    }
}

fn process_input_lane(
    lane: &mut InputLane,
    sample: f32,
    recovered: &mut u64,
    report: &mut BuiltinProcessReport,
) -> f32 {
    let sample = sanitize_input(sample, &mut report.sanitized_input);
    let signed = if lane.polarity { -sample } else { sample };
    let trimmed = sanitize(signed * lane.trim, &mut report.sanitized_output);
    let high = lane.hpf.process(trimmed, recovered, report);
    lane.lpf.process(high, recovered, report)
}

impl FaderMuteBuiltins {
    pub fn process(
        &mut self,
        block: DualMonoBlock<'_>,
    ) -> Result<BuiltinProcessReport, BuiltinParameterError> {
        let mut report = BuiltinProcessReport::default();
        block.checked_len()?;
        for (left, right) in block.left.iter_mut().zip(block.right.iter_mut()) {
            let left_input = sanitize_input(*left, &mut report.sanitized_input);
            let right_input = sanitize_input(*right, &mut report.sanitized_input);
            *left = if self.left.muted {
                0.0
            } else {
                sanitize(left_input * self.left.gain, &mut report.sanitized_output)
            };
            *right = if self.right.muted {
                0.0
            } else {
                sanitize(right_input * self.right.gain, &mut report.sanitized_output)
            };
        }
        Ok(report)
    }
    fn reset(&mut self) {}
}

impl MatrixBuiltins {
    pub fn set_target(&mut self, target: Matrix2x2) -> Result<(), BuiltinParameterError> {
        self.target = target.checked()?;
        self.remaining_updates = self.smoothing_samples;
        if self.remaining_updates == 0 {
            self.current = self.target;
        }
        Ok(())
    }
    pub fn process(
        &mut self,
        block: DualMonoBlock<'_>,
    ) -> Result<BuiltinProcessReport, BuiltinParameterError> {
        let mut report = BuiltinProcessReport::default();
        block.checked_len()?;
        for (left, right) in block.left.iter_mut().zip(block.right.iter_mut()) {
            self.advance();
            let in_left = sanitize_input(*left, &mut report.sanitized_input);
            let in_right = sanitize_input(*right, &mut report.sanitized_input);
            if self.current == Matrix2x2::IDENTITY {
                *left = in_left;
                *right = in_right;
            } else {
                *left = sanitize(
                    self.current.ll * in_left + self.current.lr * in_right,
                    &mut report.sanitized_output,
                );
                *right = sanitize(
                    self.current.rl * in_left + self.current.rr * in_right,
                    &mut report.sanitized_output,
                );
            }
        }
        Ok(report)
    }
    fn advance(&mut self) {
        if self.remaining_updates == 0 {
            return;
        }
        let remaining = self.remaining_updates as f32;
        self.current.ll += (self.target.ll - self.current.ll) / remaining;
        self.current.lr += (self.target.lr - self.current.lr) / remaining;
        self.current.rl += (self.target.rl - self.current.rl) / remaining;
        self.current.rr += (self.target.rr - self.current.rr) / remaining;
        self.remaining_updates -= 1;
        if self.remaining_updates == 0 {
            self.current = self.target;
        }
    }
    pub fn reset(&mut self) {
        self.current = self.target;
        self.remaining_updates = 0;
    }
}

pub fn pan_matrix(left: f32, right: f32) -> Result<Matrix2x2, BuiltinParameterError> {
    if !left.is_finite()
        || !right.is_finite()
        || !(-1.0..=1.0).contains(&left)
        || !(-1.0..=1.0).contains(&right)
    {
        return Err(BuiltinParameterError::MatrixCoefficient);
    }
    let gains = |position: f32| {
        let theta = (f64::from(position) + 1.0) * core::f64::consts::FRAC_PI_4;
        (theta.cos() as f32, theta.sin() as f32)
    };
    let (ll, rl) = gains(left);
    let (lr, rr) = gains(right);
    Matrix2x2 { ll, lr, rl, rr }.checked()
}

pub fn balance_matrix(balance: f32) -> Result<Matrix2x2, BuiltinParameterError> {
    if !balance.is_finite() || !(-1.0..=1.0).contains(&balance) {
        return Err(BuiltinParameterError::MatrixCoefficient);
    }
    let gain = (f64::from(balance.abs()) * core::f64::consts::FRAC_PI_2).cos() as f32;
    if balance >= 0.0 {
        Ok(Matrix2x2 {
            ll: gain,
            lr: 0.0,
            rl: 0.0,
            rr: 1.0,
        })
    } else {
        Ok(Matrix2x2 {
            ll: 1.0,
            lr: 0.0,
            rl: 0.0,
            rr: gain,
        })
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MeterTap {
    Input = 1,
    PostInputBuiltins = 2,
    PostSimd1 = 3,
    PostDynamic = 4,
    PostSimd2PreFader = 5,
    PostFader = 6,
    PostMatrix = 7,
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MeterHandle(pub NonZeroU64);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeterConfig {
    pub period_frames: NonZeroU32,
    pub peak_hold_frames: u32,
    pub peak_decay_db_per_second: f32,
    pub queue_capacity: NonZeroUsize,
    pub reset_generation: u64,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeterConfigError {
    DecayDomain,
    Queue,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeterObservationError {
    LaneLength,
    SampleTimeOverflow,
}
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MeterLaneSnapshot {
    pub sample_peak: f32,
    pub rms: f64,
    pub energy: f64,
    pub held_peak: f32,
    pub clipped_samples: u64,
    pub sanitized_samples: u64,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeterSnapshot {
    pub handle: MeterHandle,
    pub reset_generation: u64,
    pub window_sequence: u64,
    pub start_sample: u64,
    pub end_sample: u64,
    pub frames: u32,
    pub left: MeterLaneSnapshot,
    pub right: MeterLaneSnapshot,
    pub cumulative_clipped_samples: u64,
    pub cumulative_sanitized_samples: u64,
    pub cumulative_discontinuities: u64,
    pub cumulative_dropped_snapshots: u64,
}

struct MeterLane {
    peak: f32,
    energy: f64,
    clipped: u64,
    sanitized: u64,
    held: f32,
    hold_remaining: u32,
}
pub struct MeterAccumulator {
    handle: MeterHandle,
    config: MeterConfig,
    decay: f32,
    start: Option<u64>,
    frames: u32,
    sequence: u64,
    left: MeterLane,
    right: MeterLane,
    cumulative_clipped: u64,
    cumulative_sanitized: u64,
    discontinuities: u64,
    dropped: u64,
    producer: Producer<MeterSnapshot>,
}

pub struct PreparedMeter {
    pub accumulator: MeterAccumulator,
    pub consumer: Consumer<MeterSnapshot>,
}

impl MeterAccumulator {
    pub fn prepare(
        handle: MeterHandle,
        config: MeterConfig,
        sample_rate: u32,
    ) -> Result<PreparedMeter, MeterConfigError> {
        if !config.peak_decay_db_per_second.is_finite()
            || !(0.0..=120.0).contains(&config.peak_decay_db_per_second)
            || sample_rate == 0
        {
            return Err(MeterConfigError::DecayDomain);
        }
        let (producer, consumer) = bounded_spsc(
            config.queue_capacity,
            QueueGeneration(config.reset_generation),
        )
        .map_err(|_| MeterConfigError::Queue)?;
        let decay = 10.0_f64
            .powf(-f64::from(config.peak_decay_db_per_second) / (20.0 * f64::from(sample_rate)))
            as f32;
        Ok(PreparedMeter {
            accumulator: Self {
                handle,
                config,
                decay: if normal_or_zero(decay) { decay } else { 0.0 },
                start: None,
                frames: 0,
                sequence: 0,
                left: meter_lane(),
                right: meter_lane(),
                cumulative_clipped: 0,
                cumulative_sanitized: 0,
                discontinuities: 0,
                dropped: 0,
                producer,
            },
            consumer,
        })
    }
    pub fn observe(
        &mut self,
        left: &[f32],
        right: &[f32],
        first_sample: u64,
    ) -> Result<(), MeterObservationError> {
        if left.len() != right.len() {
            return Err(MeterObservationError::LaneLength);
        }
        let len = match u64::try_from(left.len())
            .ok()
            .and_then(|len| first_sample.checked_add(len))
        {
            Some(_) => left.len(),
            None => return Err(MeterObservationError::SampleTimeOverflow),
        };
        if self
            .start
            .is_some_and(|start| first_sample != start.saturating_add(u64::from(self.frames)))
        {
            self.discontinuity(first_sample);
        }
        if self.start.is_none() {
            self.start = Some(first_sample);
        }
        for index in 0..len {
            observe_lane(
                &mut self.left,
                left[index],
                self.config,
                self.decay,
                &mut self.cumulative_clipped,
                &mut self.cumulative_sanitized,
            );
            observe_lane(
                &mut self.right,
                right[index],
                self.config,
                self.decay,
                &mut self.cumulative_clipped,
                &mut self.cumulative_sanitized,
            );
            self.frames = self.frames.saturating_add(1);
            if self.frames == self.config.period_frames.get() {
                self.emit();
            }
        }
        Ok(())
    }
    pub fn reset(&mut self, kind: BuiltinResetKind) {
        self.start = None;
        self.frames = 0;
        self.left = meter_lane();
        self.right = meter_lane();
        if matches!(kind, BuiltinResetKind::FullToPrepared) {
            self.sequence = 0;
            self.cumulative_clipped = 0;
            self.cumulative_sanitized = 0;
            self.discontinuities = 0;
            self.dropped = 0;
        }
    }
    #[must_use]
    pub const fn dropped_snapshots(&self) -> u64 {
        self.dropped
    }
    fn discontinuity(&mut self, first_sample: u64) {
        self.start = Some(first_sample);
        self.frames = 0;
        self.left = meter_lane();
        self.right = meter_lane();
        self.discontinuities = self.discontinuities.saturating_add(1);
    }
    fn emit(&mut self) {
        let Some(start) = self.start else {
            // This can only follow internal corruption.  Preserve the render no-panic contract,
            // discard the incomplete interval, and surface it in the next snapshot counter.
            self.discontinuity(0);
            return;
        };
        let end = match start.checked_add(u64::from(self.frames)) {
            Some(value) => value,
            None => {
                self.discontinuity(start);
                return;
            }
        };
        let snapshot = MeterSnapshot {
            handle: self.handle,
            reset_generation: self.config.reset_generation,
            window_sequence: self.sequence,
            start_sample: start,
            end_sample: end,
            frames: self.frames,
            left: lane_snapshot(&self.left, self.frames),
            right: lane_snapshot(&self.right, self.frames),
            cumulative_clipped_samples: self.cumulative_clipped,
            cumulative_sanitized_samples: self.cumulative_sanitized,
            cumulative_discontinuities: self.discontinuities,
            cumulative_dropped_snapshots: self.dropped,
        };
        if self.producer.try_push(snapshot).is_err() {
            self.dropped = self.dropped.saturating_add(1);
        }
        self.sequence = self.sequence.saturating_add(1);
        self.start = Some(end);
        self.frames = 0;
        clear_interval(&mut self.left);
        clear_interval(&mut self.right);
    }
}

fn meter_lane() -> MeterLane {
    MeterLane {
        peak: 0.0,
        energy: 0.0,
        clipped: 0,
        sanitized: 0,
        held: 0.0,
        hold_remaining: 0,
    }
}
fn clear_interval(lane: &mut MeterLane) {
    lane.peak = 0.0;
    lane.energy = 0.0;
    lane.clipped = 0;
    lane.sanitized = 0;
}
fn observe_lane(
    lane: &mut MeterLane,
    sample: f32,
    config: MeterConfig,
    decay: f32,
    cumulative_clipped: &mut u64,
    cumulative_sanitized: &mut u64,
) {
    let sanitized = !normal_or_zero(sample);
    let sample = if sanitized { 0.0 } else { sample };
    if sanitized {
        lane.sanitized = lane.sanitized.saturating_add(1);
        *cumulative_sanitized = cumulative_sanitized.saturating_add(1);
    }
    let absolute = sample.abs();
    lane.peak = lane.peak.max(absolute);
    lane.energy += f64::from(sample) * f64::from(sample);
    if absolute >= 1.0 {
        lane.clipped = lane.clipped.saturating_add(1);
        *cumulative_clipped = cumulative_clipped.saturating_add(1);
    }
    if absolute >= lane.held {
        lane.held = absolute;
        lane.hold_remaining = config.peak_hold_frames;
    } else if lane.hold_remaining > 0 {
        lane.hold_remaining -= 1;
    } else if config.peak_decay_db_per_second != 0.0 {
        lane.held = sanitize(lane.held * decay, &mut 0);
    }
}
fn lane_snapshot(lane: &MeterLane, frames: u32) -> MeterLaneSnapshot {
    MeterLaneSnapshot {
        sample_peak: lane.peak,
        rms: (lane.energy / f64::from(frames)).sqrt(),
        energy: lane.energy,
        held_peak: lane.held,
        clipped_samples: lane.clipped,
        sanitized_samples: lane.sanitized,
    }
}

fn db_gain(db: f32) -> Result<f32, BuiltinParameterError> {
    let value = 10.0_f64.powf(f64::from(db) / 20.0) as f32;
    if normal_or_zero(value) {
        Ok(zero(value))
    } else {
        Err(BuiltinParameterError::GainDomain)
    }
}
fn normal_or_zero(value: f32) -> bool {
    value.is_finite() && !value.is_subnormal()
}
fn zero(value: f32) -> f32 {
    if value == 0.0 { 0.0 } else { value }
}
fn sanitize_input(value: f32, count: &mut u64) -> f32 {
    if normal_or_zero(value) {
        value
    } else {
        *count = count.saturating_add(1);
        0.0
    }
}
fn sanitize(value: f32, count: &mut u64) -> f32 {
    sanitize_input(value, count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_core::{EXTENDED_COMPATIBILITY_SAMPLE_RATES, LAUNCH_SAMPLE_RATES};
    use miso_engine_dsp_reference::{
        ReferenceBiquad, ReferenceFilterKind, ReferenceTptOutput, ReferenceTptStateSpace,
        rbj_butterworth_magnitude_db,
    };

    // Issue 032: the first tier is launch-gated; the second remains informational compatibility
    // evidence from issue 007 and is not an engine session or host support claim.
    fn launch_and_extended_compatibility_rates() -> impl Iterator<Item = u32> {
        LAUNCH_SAMPLE_RATES
            .into_iter()
            .chain(EXTENDED_COMPATIBILITY_SAMPLE_RATES)
            .map(|rate| rate.0)
    }

    #[test]
    fn polarity_trim_fader_and_matrix_are_exact() {
        let mut chain = BuiltinChain::new(
            48_000,
            BuiltinParameters {
                left: ChannelParameters {
                    polarity_invert: true,
                    trim_db: 6.0206,
                    fader_db: 0.0,
                    ..ChannelParameters::default()
                },
                right: ChannelParameters::default(),
                matrix: Matrix2x2::IDENTITY,
                smoothing_samples: 0,
            },
        )
        .expect("prepare");
        let mut left = [0.5_f32];
        let mut right = [0.0_f32];
        chain
            .process_dual_mono(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"))
            .expect("valid block");
        assert!((left[0] + 1.0).abs() < 2e-5);
        assert_eq!(right, [0.0]);
    }
    #[test]
    fn matrix_ramp_reaches_target() {
        let mut chain = BuiltinChain::new(
            48_000,
            BuiltinParameters {
                smoothing_samples: 2,
                ..BuiltinParameters::default()
            },
        )
        .expect("prepare");
        chain
            .set_matrix_target(Matrix2x2 {
                ll: 0.0,
                lr: 0.0,
                rl: 0.0,
                rr: 0.0,
            })
            .expect("target");
        let mut left = [1.0, 1.0];
        let mut right = [0.0, 0.0];
        chain
            .process_matrix(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"))
            .expect("valid block");
        assert_eq!(left, [0.5, 0.0]);
    }
    #[test]
    fn meter_windows_are_exact() {
        let handle = MeterHandle(NonZeroU64::new(1).expect("constant"));
        let config = MeterConfig {
            period_frames: NonZeroU32::new(2).expect("constant"),
            peak_hold_frames: 0,
            peak_decay_db_per_second: 0.0,
            queue_capacity: NonZeroUsize::new(2).expect("constant"),
            reset_generation: 7,
        };
        let PreparedMeter {
            mut accumulator,
            mut consumer,
        } = MeterAccumulator::prepare(handle, config, 48_000).expect("meter");
        accumulator
            .observe(&[1.0, 0.5], &[0.0, -1.0], 3)
            .expect("matched meter lanes");
        let snap = consumer.try_pop().expect("snapshot");
        assert_eq!(snap.start_sample, 3);
        assert_eq!(snap.end_sample, 5);
        assert_eq!(snap.left.clipped_samples, 1);
        assert_eq!(snap.right.clipped_samples, 1);
    }
    #[test]
    fn parameter_descriptors_have_complete_stable_contracts() {
        let descriptors = BUILTIN_PARAMETER_DESCRIPTORS_V1;
        assert_eq!(
            descriptors.map(|descriptor| descriptor.id),
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        );
        assert_eq!(
            descriptors.map(|descriptor| descriptor.name),
            [
                "polarity_invert",
                "trim_db",
                "hpf_hz",
                "lpf_hz",
                "fader_db",
                "mute",
                "matrix_ll",
                "matrix_lr",
                "matrix_rl",
                "matrix_rr",
            ]
        );
        assert_eq!(
            descriptors.map(|descriptor| descriptor.scope),
            [
                BuiltinParameterScope::PerLane,
                BuiltinParameterScope::PerLane,
                BuiltinParameterScope::PerLane,
                BuiltinParameterScope::PerLane,
                BuiltinParameterScope::PerLane,
                BuiltinParameterScope::PerLane,
                BuiltinParameterScope::MatrixShared,
                BuiltinParameterScope::MatrixShared,
                BuiltinParameterScope::MatrixShared,
                BuiltinParameterScope::MatrixShared,
            ]
        );
        assert_eq!(
            descriptors.map(|descriptor| descriptor.mapping),
            [
                BuiltinParameterMapping::Boolean,
                BuiltinParameterMapping::DecibelAmplitude,
                BuiltinParameterMapping::Hertz,
                BuiltinParameterMapping::Hertz,
                BuiltinParameterMapping::DecibelAmplitude,
                BuiltinParameterMapping::Boolean,
                BuiltinParameterMapping::Linear,
                BuiltinParameterMapping::Linear,
                BuiltinParameterMapping::Linear,
                BuiltinParameterMapping::Linear,
            ]
        );
        assert_eq!(
            descriptors.map(|descriptor| descriptor.default.to_bits()),
            [0, 0, 0, 0, 0, 0, 1.0_f32.to_bits(), 0, 0, 1.0_f32.to_bits()]
        );
        assert_eq!(
            descriptors.map(|descriptor| descriptor.update_rate),
            [
                BuiltinParameterUpdateRate::PreparedOnly,
                BuiltinParameterUpdateRate::PreparedOnly,
                BuiltinParameterUpdateRate::PreparedOnly,
                BuiltinParameterUpdateRate::PreparedOnly,
                BuiltinParameterUpdateRate::PreparedOnly,
                BuiltinParameterUpdateRate::PreparedOnly,
                BuiltinParameterUpdateRate::BlockTarget,
                BuiltinParameterUpdateRate::BlockTarget,
                BuiltinParameterUpdateRate::BlockTarget,
                BuiltinParameterUpdateRate::BlockTarget,
            ]
        );
        assert_eq!(
            descriptors.map(|descriptor| descriptor.smoothing),
            [
                BuiltinSmoothingPolicy::None,
                BuiltinSmoothingPolicy::None,
                BuiltinSmoothingPolicy::None,
                BuiltinSmoothingPolicy::None,
                BuiltinSmoothingPolicy::None,
                BuiltinSmoothingPolicy::None,
                BuiltinSmoothingPolicy::LinearNUpdates,
                BuiltinSmoothingPolicy::LinearNUpdates,
                BuiltinSmoothingPolicy::LinearNUpdates,
                BuiltinSmoothingPolicy::LinearNUpdates,
            ]
        );
        assert_eq!(
            descriptors.map(|descriptor| descriptor.reset),
            [
                BuiltinParameterReset::RestorePreparedValue,
                BuiltinParameterReset::RestorePreparedValue,
                BuiltinParameterReset::RestorePreparedValue,
                BuiltinParameterReset::RestorePreparedValue,
                BuiltinParameterReset::RestorePreparedValue,
                BuiltinParameterReset::RestorePreparedValue,
                BuiltinParameterReset::KeepTargetResetCurrent,
                BuiltinParameterReset::KeepTargetResetCurrent,
                BuiltinParameterReset::KeepTargetResetCurrent,
                BuiltinParameterReset::KeepTargetResetCurrent,
            ]
        );
        assert_eq!(
            descriptors.map(|descriptor| descriptor.disabled_value),
            [
                None,
                None,
                Some(0.0),
                Some(0.0),
                None,
                None,
                None,
                None,
                None,
                None,
            ]
        );
        assert_eq!(
            descriptors.map(|descriptor| descriptor.domain),
            [
                BuiltinParameterDomain::BooleanExact,
                BuiltinParameterDomain::FiniteInclusive {
                    minimum: -144.0,
                    maximum: 24.0,
                },
                BuiltinParameterDomain::DisabledOrRateKeyedHertzV1 {
                    disabled: 0.0,
                    minimum_hz: 10.0,
                },
                BuiltinParameterDomain::DisabledOrRateKeyedHertzV1 {
                    disabled: 0.0,
                    minimum_hz: 10.0,
                },
                BuiltinParameterDomain::FiniteInclusive {
                    minimum: -144.0,
                    maximum: 24.0,
                },
                BuiltinParameterDomain::BooleanExact,
                BuiltinParameterDomain::FiniteInclusive {
                    minimum: -1.0,
                    maximum: 1.0,
                },
                BuiltinParameterDomain::FiniteInclusive {
                    minimum: -1.0,
                    maximum: 1.0,
                },
                BuiltinParameterDomain::FiniteInclusive {
                    minimum: -1.0,
                    maximum: 1.0,
                },
                BuiltinParameterDomain::FiniteInclusive {
                    minimum: -1.0,
                    maximum: 1.0,
                },
            ]
        );
    }

    #[test]
    fn descriptor_domains_are_exhaustive_at_launch_rates() {
        for rate in [44_100, 48_000, 88_200, 96_000] {
            for descriptor in BUILTIN_PARAMETER_DESCRIPTORS_V1 {
                assert!(descriptor.domain.contains(descriptor.default, rate));
                assert!(!descriptor.domain.contains(f32::NAN, rate));
                assert!(!descriptor.domain.contains(f32::INFINITY, rate));
                assert!(!descriptor.domain.contains(f32::NEG_INFINITY, rate));
            }
            for descriptor in [
                BUILTIN_PARAMETER_DESCRIPTORS_V1[2],
                BUILTIN_PARAMETER_DESCRIPTORS_V1[3],
            ] {
                let maximum = builtin_filter_cutoff_maximum_hz_v1(rate)
                    .expect("launch rate has an exact cutoff maximum");
                let successor = f32::from_bits(maximum.to_bits() + 1);
                let nyquist = rate as f32 / 2.0;
                let just_below_maximum = f32::from_bits(maximum.to_bits() - 1);
                assert!(descriptor.domain.contains(0.0, rate));
                assert!(!descriptor.domain.contains(-0.0, rate));
                assert!(!descriptor.domain.contains(9.999, rate));
                assert!(descriptor.domain.contains(10.0, rate));
                assert!(descriptor.domain.contains(just_below_maximum, rate));
                assert!(descriptor.domain.contains(maximum, rate));
                assert!(!descriptor.domain.contains(successor, rate));
                assert!(!descriptor.domain.contains(nyquist, rate));
            }
        }
        for boolean in [
            BUILTIN_PARAMETER_DESCRIPTORS_V1[0],
            BUILTIN_PARAMETER_DESCRIPTORS_V1[5],
        ] {
            assert!(boolean.domain.contains(0.0, 48_000));
            assert!(boolean.domain.contains(1.0, 48_000));
            assert!(!boolean.domain.contains(-0.0, 48_000));
            assert!(!boolean.domain.contains(0.5, 48_000));
        }
        for decibels in [
            BUILTIN_PARAMETER_DESCRIPTORS_V1[1],
            BUILTIN_PARAMETER_DESCRIPTORS_V1[4],
        ] {
            assert!(decibels.domain.contains(-144.0, 48_000));
            assert!(decibels.domain.contains(24.0, 48_000));
            assert!(!decibels.domain.contains(-144.001, 48_000));
            assert!(!decibels.domain.contains(24.001, 48_000));
        }
        for matrix in &BUILTIN_PARAMETER_DESCRIPTORS_V1[6..] {
            assert!(matrix.domain.contains(-1.0, 48_000));
            assert!(matrix.domain.contains(1.0, 48_000));
            assert!(!matrix.domain.contains(-1.001, 48_000));
            assert!(!matrix.domain.contains(1.001, 48_000));
        }
    }

    #[test]
    fn compatibility_fallback_is_limited_to_the_exact_extended_rate_tier() {
        for rate in EXTENDED_COMPATIBILITY_SAMPLE_RATES.map(|rate| rate.0) {
            assert_eq!(builtin_filter_cutoff_maximum_hz_v1(rate), None);
            for descriptor in [
                BUILTIN_PARAMETER_DESCRIPTORS_V1[2],
                BUILTIN_PARAMETER_DESCRIPTORS_V1[3],
            ] {
                assert!(descriptor.domain.contains(0.0, rate));
                assert!(descriptor.domain.contains(10.0, rate));
                assert!(descriptor.domain.contains(0.45 * rate as f32, rate));
            }
            assert!(BuiltinChain::new(rate, BuiltinParameters::default()).is_ok());
        }
        for rate in [0, 32_000, 192_001] {
            assert_eq!(builtin_filter_cutoff_maximum_hz_v1(rate), None);
            for descriptor in [
                BUILTIN_PARAMETER_DESCRIPTORS_V1[2],
                BUILTIN_PARAMETER_DESCRIPTORS_V1[3],
            ] {
                assert!(!descriptor.domain.contains(0.0, rate));
                assert!(!descriptor.domain.contains(10.0, rate));
            }
            assert!(matches!(
                BuiltinChain::new(rate, BuiltinParameters::default()),
                Err(BuiltinParameterError::FilterCutoff)
            ));
        }
    }

    fn parameters_with_cutoff(cutoff: f32, high_pass: bool) -> BuiltinParameters {
        let mut parameters = BuiltinParameters::default();
        if high_pass {
            parameters.left.hpf_hz = cutoff;
        } else {
            parameters.left.lpf_hz = cutoff;
        }
        parameters
    }

    #[test]
    fn representable_cutoff_domain_is_shared_by_descriptors_and_preparation() {
        for (rate, maximum_bits) in [
            (44_100, 0x46ac_42f7),
            (48_000, 0x46bb_7ede),
            (88_200, 0x472c_42f7),
            (96_000, 0x473b_7ede),
        ] {
            let maximum =
                builtin_filter_cutoff_maximum_hz_v1(rate).expect("launch rate has maximum");
            assert_eq!(maximum.to_bits(), maximum_bits, "rate={rate}");
            let successor = f32::from_bits(maximum_bits + 1);
            let nyquist = rate as f32 * 0.5;
            let nyquist_predecessor = f32::from_bits(nyquist.to_bits() - 1);
            for (descriptor, high_pass) in [
                (BUILTIN_PARAMETER_DESCRIPTORS_V1[2], true),
                (BUILTIN_PARAMETER_DESCRIPTORS_V1[3], false),
            ] {
                for (cutoff, expected) in [
                    (0.0, true),
                    (10.0, true),
                    (f32::from_bits(maximum_bits - 1), true),
                    (maximum, true),
                    (successor, false),
                    (nyquist_predecessor, false),
                    (nyquist, false),
                    (9.999, false),
                    (f32::NAN, false),
                    (f32::INFINITY, false),
                    (f32::NEG_INFINITY, false),
                ] {
                    assert_eq!(
                        descriptor.domain.contains(cutoff, rate),
                        expected,
                        "descriptor rate={rate}, high_pass={high_pass}, cutoff={:08x}",
                        cutoff.to_bits()
                    );
                    assert_eq!(
                        BuiltinChain::new(rate, parameters_with_cutoff(cutoff, high_pass)).is_ok(),
                        expected,
                        "preparation rate={rate}, high_pass={high_pass}, cutoff={:08x}",
                        cutoff.to_bits()
                    );
                }
            }
        }
    }

    #[test]
    fn representable_cutoff_seam_is_contiguous_for_both_tpt_sections() {
        for (rate, maximum_bits) in [
            (44_100, 0x46ac_42f7),
            (48_000, 0x46bb_7ede),
            (88_200, 0x472c_42f7),
            (96_000, 0x473b_7ede),
        ] {
            let start_bits = (0.45_f32 * rate as f32).to_bits();
            for high_pass in [true, false] {
                for bits in start_bits..=maximum_bits {
                    let cutoff = f32::from_bits(bits);
                    assert!(
                        validate_builtin_filter_cutoff_v1(cutoff, rate, 0.0, 10.0).is_ok(),
                        "domain rate={rate}, high_pass={high_pass}, cutoff={bits:08x}"
                    );
                    TptSvf::design(rate, cutoff, high_pass).unwrap_or_else(|error| {
                        panic!(
                            "TPT rate={rate}, high_pass={high_pass}, cutoff={bits:08x}, error={error:?}"
                        )
                    });
                }
                let successor = f32::from_bits(maximum_bits + 1);
                assert_eq!(
                    validate_builtin_filter_cutoff_v1(successor, rate, 0.0, 10.0),
                    Err(BuiltinParameterError::FilterCutoff),
                    "successor rate={rate}, high_pass={high_pass}"
                );
                assert!(
                    matches!(
                        BuiltinChain::new(rate, parameters_with_cutoff(successor, high_pass)),
                        Err(BuiltinParameterError::FilterCutoff)
                    ),
                    "successor preparation rate={rate}, high_pass={high_pass}"
                );
            }
            assert!(
                matches!(
                    TptSvf::design(rate, f32::from_bits(maximum_bits + 1), true),
                    Err(BuiltinParameterError::FilterCoefficients)
                ),
                "the published successor must be the first underlying HPF coefficient failure: rate={rate}"
            );
        }
    }
    #[test]
    fn blocks_reject_before_processing_and_reports_are_per_call() {
        let mut left = [1.0_f32];
        let mut right = [1.0_f32, 1.0];
        assert!(matches!(
            DualMonoBlock::new(&mut left, &mut right, 0),
            Err(BuiltinParameterError::LaneLength)
        ));
        let mut empty_left = [];
        let mut empty_right = [];
        assert!(matches!(
            DualMonoBlock::new(&mut empty_left, &mut empty_right, 0),
            Err(BuiltinParameterError::EmptyBlock)
        ));
        let mut overflow_left = [1.0_f32];
        let mut overflow_right = [1.0_f32];
        assert!(matches!(
            DualMonoBlock::new(&mut overflow_left, &mut overflow_right, u64::MAX),
            Err(BuiltinParameterError::SampleTimeOverflow)
        ));

        let mut chain = BuiltinChain::new(
            48_000,
            BuiltinParameters {
                left: ChannelParameters {
                    hpf_hz: 100.0,
                    ..ChannelParameters::default()
                },
                ..BuiltinParameters::default()
            },
        )
        .expect("prepare");
        chain.input.left.hpf.s1 = f32::NAN;
        let mut first_left = [0.5_f32];
        let mut first_right = [0.0_f32];
        let first = chain
            .process_input(DualMonoBlock::new(&mut first_left, &mut first_right, 0).expect("block"))
            .expect("process");
        assert_eq!(first.recovered_left_state, 1);
        assert_eq!(chain.input.lifetime_recovered_state(), (1, 0));
        let mut second_left = [0.5_f32];
        let mut second_right = [0.0_f32];
        let second = chain
            .process_input(
                DualMonoBlock::new(&mut second_left, &mut second_right, 1).expect("block"),
            )
            .expect("process");
        assert_eq!(second.recovered_left_state, 0);
        assert_eq!(chain.input.lifetime_recovered_state(), (1, 0));
    }
    #[test]
    fn fader_and_identity_matrix_sanitize_and_preserve_signed_zero() {
        let mut chain = BuiltinChain::new(48_000, BuiltinParameters::default()).expect("prepare");
        let mut left = [-0.0_f32, f32::NAN];
        let mut right = [0.0_f32, f32::INFINITY];
        let report = chain
            .process_fader_mute(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"))
            .expect("process");
        assert_eq!(left[0].to_bits(), (-0.0_f32).to_bits());
        assert_eq!(right[0].to_bits(), 0.0_f32.to_bits());
        assert_eq!(left[1], 0.0);
        assert_eq!(right[1], 0.0);
        assert_eq!(report.sanitized_input, 2);
        let report = chain
            .process_matrix(DualMonoBlock::new(&mut left, &mut right, 2).expect("block"))
            .expect("process");
        assert_eq!(left[0].to_bits(), (-0.0_f32).to_bits());
        assert_eq!(right[0].to_bits(), 0.0_f32.to_bits());
        assert_eq!(report.sanitized_input, 0);
    }
    #[test]
    fn meter_windows_discontinuities_resets_and_drops_are_exact() {
        let handle = MeterHandle(NonZeroU64::new(1).expect("constant"));
        let config = MeterConfig {
            period_frames: NonZeroU32::new(2).expect("constant"),
            peak_hold_frames: 1,
            peak_decay_db_per_second: 0.0,
            queue_capacity: NonZeroUsize::new(1).expect("constant"),
            reset_generation: 9,
        };
        let PreparedMeter {
            mut accumulator,
            mut consumer,
        } = MeterAccumulator::prepare(handle, config, 48_000).expect("meter");
        assert_eq!(
            accumulator.observe(&[0.5], &[f32::NAN, 0.0], 0),
            Err(MeterObservationError::LaneLength)
        );
        accumulator
            .observe(&[1.0, 0.0], &[0.25, -0.25], 4)
            .expect("first window");
        let first = consumer.try_pop().expect("first snapshot");
        assert_eq!(
            (first.start_sample, first.end_sample, first.frames),
            (4, 6, 2)
        );
        assert_eq!(first.left.energy, 1.0);
        assert!((first.left.rms - 1.0 / 2.0_f64.sqrt()).abs() <= f64::EPSILON);
        assert_eq!(first.left.held_peak, 1.0);
        accumulator
            .observe(&[0.0], &[0.0], 9)
            .expect("discontinuity");
        accumulator
            .observe(&[0.0], &[0.0], 10)
            .expect("second window");
        let second = consumer.try_pop().expect("second snapshot");
        assert_eq!((second.start_sample, second.end_sample), (9, 11));
        assert_eq!(second.cumulative_discontinuities, 1);
        accumulator
            .observe(&[0.0, 0.0], &[0.0, 0.0], 11)
            .expect("queued snapshot");
        accumulator
            .observe(&[0.0, 0.0], &[0.0, 0.0], 13)
            .expect("dropped snapshot");
        let queued = consumer.try_pop().expect("queued snapshot");
        assert_eq!(queued.cumulative_dropped_snapshots, 0);
        accumulator
            .observe(&[0.0, 0.0], &[0.0, 0.0], 15)
            .expect("post-drop snapshot");
        let post_drop = consumer.try_pop().expect("post-drop snapshot");
        assert_eq!(post_drop.cumulative_dropped_snapshots, 1);
        accumulator.reset(BuiltinResetKind::DiscontinuityKeepTargets);
        accumulator
            .observe(&[0.0, 0.0], &[0.0, 0.0], 17)
            .expect("reset window");
        let reset = consumer.try_pop().expect("reset snapshot");
        assert_eq!(reset.window_sequence, 5);
        assert_eq!(reset.cumulative_dropped_snapshots, 1);
        accumulator.reset(BuiltinResetKind::FullToPrepared);
        accumulator
            .observe(&[0.0, 0.0], &[0.0, 0.0], 19)
            .expect("full reset window");
        let full_reset = consumer.try_pop().expect("full reset snapshot");
        assert_eq!(full_reset.window_sequence, 0);
        assert_eq!(full_reset.cumulative_dropped_snapshots, 0);
        assert_eq!(full_reset.cumulative_discontinuities, 0);
    }
    #[test]
    fn ten_thousand_deterministic_meter_mutations_remain_bounded_and_finite() {
        let handle = MeterHandle(NonZeroU64::new(1).expect("constant"));
        let mut state = 0x4d45_5445_525f_3031_u64;
        for iteration in 0..10_000_u64 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let period = NonZeroU32::new(((state as u32) & 7) + 1).expect("nonzero");
            let capacity = NonZeroUsize::new((((state >> 8) as usize) & 3) + 1).expect("nonzero");
            let PreparedMeter {
                mut accumulator,
                mut consumer,
            } = MeterAccumulator::prepare(
                handle,
                MeterConfig {
                    period_frames: period,
                    peak_hold_frames: ((state >> 16) as u32) & 15,
                    peak_decay_db_per_second: ((state >> 32) as f32 / u32::MAX as f32) * 120.0,
                    queue_capacity: capacity,
                    reset_generation: iteration,
                },
                48_000,
            )
            .expect("generated meter config");
            let frames = usize::try_from(period.get()).expect("small period") * 2;
            let mut left = [0.0_f32; 16];
            let mut right = [0.0_f32; 16];
            for index in 0..frames {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                left[index] = if state & 31 == 0 {
                    f32::NAN
                } else {
                    ((state as i32) as f32) / i32::MAX as f32
                };
                right[index] = if state & 63 == 0 {
                    f32::INFINITY
                } else {
                    (((state >> 32) as i32) as f32) / i32::MAX as f32
                };
            }
            accumulator
                .observe(
                    &left[..frames],
                    &right[..frames],
                    iteration.saturating_mul(32),
                )
                .expect("matching meter lanes");
            while let Ok(snapshot) = consumer.try_pop() {
                assert_eq!(snapshot.frames, period.get());
                assert!(snapshot.left.energy.is_finite());
                assert!(snapshot.right.energy.is_finite());
                assert!(snapshot.left.rms.is_finite());
                assert!(snapshot.right.rms.is_finite());
                assert!(snapshot.left.sample_peak.is_finite());
                assert!(snapshot.right.sample_peak.is_finite());
            }
        }
    }
    #[test]
    fn launch_and_extended_compatibility_rates_match_the_independent_f64_rbj_oracle() {
        for rate in launch_and_extended_compatibility_rates() {
            let parameters = BuiltinParameters {
                left: ChannelParameters {
                    hpf_hz: 100.0,
                    lpf_hz: 1_000.0,
                    ..ChannelParameters::default()
                },
                ..BuiltinParameters::default()
            };
            let mut chain = BuiltinChain::new(rate, parameters).expect("prepare");
            let mut left = [0.0_f32; 256];
            let mut right = [0.0_f32; 256];
            left[0] = 1.0;
            let mut high = ReferenceBiquad::rbj_butterworth(
                f64::from(rate),
                100.0,
                ReferenceFilterKind::HighPass,
            )
            .expect("reference high pass");
            let mut low = ReferenceBiquad::rbj_butterworth(
                f64::from(rate),
                1_000.0,
                ReferenceFilterKind::LowPass,
            )
            .expect("reference low pass");
            let expected: Vec<_> = (0..left.len())
                .map(|index| low.process(high.process(if index == 0 { 1.0 } else { 0.0 })))
                .collect();
            let _ = chain
                .process_input(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"))
                .expect("valid block");
            for (actual, reference) in left.iter().zip(expected) {
                assert!(
                    (f64::from(*actual) - reference).abs() <= 2e-5,
                    "rate={rate}, actual={actual}, reference={reference}"
                );
            }
            assert_eq!(right, [0.0; 256]);
        }
    }
    #[test]
    fn ten_thousand_bounded_parameter_and_block_mutations_stay_finite() {
        let mut state = 0x5EED_CAFE_1234_5678_u64;
        for iteration in 0..10_000_u64 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let fraction = |shift| ((state >> shift) as u32) as f32 / u32::MAX as f32;
            let db = |shift| -144.0 + fraction(shift) * 168.0;
            let matrix = Matrix2x2 {
                ll: fraction(0) * 2.0 - 1.0,
                lr: fraction(8) * 2.0 - 1.0,
                rl: fraction(16) * 2.0 - 1.0,
                rr: fraction(24) * 2.0 - 1.0,
            };
            let rate = LAUNCH_SAMPLE_RATES[(state as usize) & 3].0;
            let mut chain = BuiltinChain::new(
                rate,
                BuiltinParameters {
                    left: ChannelParameters {
                        polarity_invert: state & 1 != 0,
                        trim_db: db(0),
                        hpf_hz: 100.0,
                        lpf_hz: 1_000.0,
                        fader_db: db(32),
                        muted: state & 2 != 0,
                    },
                    right: ChannelParameters {
                        polarity_invert: state & 4 != 0,
                        trim_db: db(8),
                        hpf_hz: 0.0,
                        lpf_hz: 0.0,
                        fader_db: db(40),
                        muted: state & 8 != 0,
                    },
                    matrix,
                    smoothing_samples: (state as u32) & 127,
                },
            )
            .expect("generated parameters are in the prepared domain");
            chain
                .set_matrix_target(Matrix2x2::IDENTITY)
                .expect("identity");
            let mut left = [0.25_f32; 127];
            let mut right = [-0.5_f32; 127];
            let _ = chain
                .process_dual_mono(
                    DualMonoBlock::new(&mut left, &mut right, iteration.saturating_mul(127))
                        .expect("block"),
                )
                .expect("valid block");
            assert!(left.iter().chain(&right).all(|sample| sample.is_finite()));
        }
    }
    #[test]
    fn launch_and_extended_compatibility_rate_sweeps_match_f64_magnitude() {
        for rate in launch_and_extended_compatibility_rates() {
            for frequency in [100.0, 1_000.0, f64::from(rate) * 0.2] {
                let frames = 4_096;
                let mut left: Vec<f32> = (0..frames)
                    .map(|index| {
                        (core::f64::consts::TAU * frequency * index as f64 / f64::from(rate)).sin()
                            as f32
                    })
                    .collect();
                let mut right = vec![0.0_f32; frames];
                let parameters = BuiltinParameters {
                    left: ChannelParameters {
                        hpf_hz: 100.0,
                        lpf_hz: 1_000.0,
                        ..ChannelParameters::default()
                    },
                    ..BuiltinParameters::default()
                };
                let mut chain = BuiltinChain::new(rate, parameters).expect("prepare");
                let mut offset = 0;
                for quantum in [1, 127, 128, 255, 1_024].into_iter().cycle() {
                    if offset == frames {
                        break;
                    }
                    let end = (offset + quantum).min(frames);
                    let _ = chain
                        .process_input(
                            DualMonoBlock::new(
                                &mut left[offset..end],
                                &mut right[offset..end],
                                offset as u64,
                            )
                            .expect("block"),
                        )
                        .expect("valid block");
                    offset = end;
                }
                let mut high = ReferenceBiquad::rbj_butterworth(
                    f64::from(rate),
                    100.0,
                    ReferenceFilterKind::HighPass,
                )
                .expect("reference high pass");
                let mut low = ReferenceBiquad::rbj_butterworth(
                    f64::from(rate),
                    1_000.0,
                    ReferenceFilterKind::LowPass,
                )
                .expect("reference low pass");
                let mut actual_energy = 0.0_f64;
                let mut reference_energy = 0.0_f64;
                for (index, actual) in left.iter().copied().enumerate() {
                    let input =
                        (core::f64::consts::TAU * frequency * index as f64 / f64::from(rate)).sin();
                    let reference = low.process(high.process(input));
                    if index >= frames / 2 {
                        actual_energy += f64::from(actual) * f64::from(actual);
                        reference_energy += reference * reference;
                    }
                }
                let actual_db = 10.0 * actual_energy.log10();
                let reference_db = 10.0 * reference_energy.log10();
                if reference_db >= -120.0 {
                    assert!(
                        (actual_db - reference_db).abs() <= 0.05,
                        "rate={rate}, frequency={frequency}, actual={actual_db}, reference={reference_db}"
                    );
                }
            }
        }
    }
    #[test]
    fn cast_tpt_state_space_matches_independent_rbj_transfer_at_compatibility_rates() {
        for rate in launch_and_extended_compatibility_rates() {
            let mut cutoffs = vec![
                10.0,
                20.0,
                100.0,
                1_000.0,
                (20_000.0_f64).min(0.1 * f64::from(rate)),
                0.45 * f64::from(rate),
            ];
            cutoffs.sort_by(f64::total_cmp);
            cutoffs.dedup_by(|left, right| *left == *right);
            for (high_pass, kind, output) in [
                (
                    true,
                    ReferenceFilterKind::HighPass,
                    ReferenceTptOutput::HighPass,
                ),
                (
                    false,
                    ReferenceFilterKind::LowPass,
                    ReferenceTptOutput::LowPass,
                ),
            ] {
                for cutoff in &cutoffs {
                    let filter = TptSvf::design(rate, *cutoff as f32, high_pass).expect("valid");
                    let state = ReferenceTptStateSpace::from_cast_coefficients(
                        filter.c1, filter.a2, filter.a3, filter.k, output,
                    );
                    assert_tpt_limits_and_monotonic(state, rate, high_pass, *cutoff);
                    let mut probes = coherent_probes(rate, *cutoff);
                    probes.extend([*cutoff, 0.49 * f64::from(rate)]);
                    probes.sort_by(f64::total_cmp);
                    probes.dedup_by(|left, right| *left == *right);
                    for frequency in probes {
                        let reference =
                            rbj_butterworth_magnitude_db(f64::from(rate), *cutoff, kind, frequency)
                                .expect("reference");
                        let actual = state
                            .magnitude_db(f64::from(rate), frequency)
                            .expect("state");
                        if reference >= -120.0 {
                            assert!(
                                (actual - reference).abs() <= 0.005,
                                "rate={rate}, cutoff={cutoff}, frequency={frequency}, actual={actual}, reference={reference}"
                            );
                        }
                    }
                    let cutoff_db = state
                        .magnitude_db(f64::from(rate), *cutoff)
                        .expect("cutoff state");
                    assert!(
                        (cutoff_db + 3.010_299_956_6).abs() <= 0.005,
                        "rate={rate}, cutoff={cutoff}, db={cutoff_db}"
                    );
                }
            }
        }
    }
    #[test]
    fn one_second_impulse_dfts_match_rbj_at_launch_and_extended_compatibility_rates() {
        for rate in launch_and_extended_compatibility_rates() {
            let mut cutoffs = vec![
                10.0,
                20.0,
                100.0,
                1_000.0,
                (20_000.0_f64).min(0.1 * f64::from(rate)),
                0.45 * f64::from(rate),
            ];
            cutoffs.sort_by(f64::total_cmp);
            cutoffs.dedup_by(|left, right| *left == *right);
            for (high_pass, kind) in [
                (true, ReferenceFilterKind::HighPass),
                (false, ReferenceFilterKind::LowPass),
            ] {
                for cutoff in &cutoffs {
                    let mut partition_reference: Option<Vec<f32>> = None;
                    for quantum in [1, 127, 128, 255, 1_024] {
                        let mut filter = TptSvf::design(rate, *cutoff as f32, high_pass)
                            .expect("valid matrix cutoff");
                        let mut report = BuiltinProcessReport::default();
                        let mut recovered = 0;
                        let mut impulse = vec![0.0_f32; rate as usize];
                        for block_start in (0..impulse.len()).step_by(quantum) {
                            let block_end = (block_start + quantum).min(impulse.len());
                            for (index, sample) in
                                impulse[block_start..block_end].iter_mut().enumerate()
                            {
                                *sample = filter.process(
                                    if block_start + index == 0 { 1.0 } else { 0.0 },
                                    &mut recovered,
                                    &mut report,
                                );
                            }
                        }
                        assert!(impulse.iter().all(|sample| sample.is_finite()));
                        assert!(
                            recovered <= 1,
                            "rate={rate}, cutoff={cutoff}, recoveries={recovered}"
                        );
                        let tail_energy = impulse[impulse.len().saturating_sub(4_096)..]
                            .iter()
                            .map(|sample| f64::from(*sample) * f64::from(*sample))
                            .sum::<f64>();
                        assert!(
                            tail_energy.is_finite() && tail_energy <= 1e-8,
                            "rate={rate}, cutoff={cutoff}, quantum={quantum}, tail_energy={tail_energy}"
                        );
                        if let Some(reference) = &partition_reference {
                            assert_eq!(
                                &impulse, reference,
                                "block partition changed bits: rate={rate}, cutoff={cutoff}, quantum={quantum}"
                            );
                        } else {
                            partition_reference = Some(impulse.clone());
                        }
                        for frequency in coherent_probes(rate, *cutoff) {
                            let reference = rbj_butterworth_magnitude_db(
                                f64::from(rate),
                                *cutoff,
                                kind,
                                frequency,
                            )
                            .expect("reference");
                            let actual =
                                impulse_dft_magnitude_db(&impulse, f64::from(rate), frequency);
                            if reference >= -120.0 {
                                assert!(
                                    (actual - reference).abs() <= 0.05,
                                    "rate={rate}, cutoff={cutoff}, quantum={quantum}, frequency={frequency}, actual={actual}, reference={reference}"
                                );
                            } else {
                                assert!(
                                    actual <= -115.0,
                                    "rate={rate}, cutoff={cutoff}, quantum={quantum}, frequency={frequency}, actual={actual}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn coherent_sustained_sines_cover_launch_and_extended_compatibility_rates() {
        for rate in launch_and_extended_compatibility_rates() {
            let mut cutoffs = vec![
                10.0,
                20.0,
                100.0,
                1_000.0,
                (20_000.0_f64).min(0.1 * f64::from(rate)),
                0.45 * f64::from(rate),
            ];
            cutoffs.sort_by(f64::total_cmp);
            cutoffs.dedup_by(|left, right| *left == *right);
            for (high_pass, kind) in [
                (true, ReferenceFilterKind::HighPass),
                (false, ReferenceFilterKind::LowPass),
            ] {
                for cutoff in &cutoffs {
                    for frequency in coherent_probes(rate, *cutoff) {
                        let mut production = TptSvf::design(rate, *cutoff as f32, high_pass)
                            .expect("valid matrix cutoff");
                        let mut reference =
                            ReferenceBiquad::rbj_butterworth(f64::from(rate), *cutoff, kind)
                                .expect("reference");
                        let measurement =
                            sustained_measurement(&mut production, &mut reference, rate, frequency);
                        if measurement.reference_gain_db >= -90.0 {
                            assert!(
                                (measurement.production_gain_db - measurement.reference_gain_db)
                                    .abs()
                                    <= 0.05,
                                "rate={rate}, cutoff={cutoff}, frequency={frequency}, production={}, reference={}",
                                measurement.production_gain_db,
                                measurement.reference_gain_db
                            );
                            assert!(
                                measurement.residual_db <= -100.0,
                                "rate={rate}, cutoff={cutoff}, frequency={frequency}, residual={}",
                                measurement.residual_db
                            );
                        } else {
                            assert!(
                                measurement.total_output_db <= -88.0,
                                "rate={rate}, cutoff={cutoff}, frequency={frequency}, output={}",
                                measurement.total_output_db
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn production_order_hpf_lpf_cascade_meets_all_launch_response_gates() {
        for rate in LAUNCH_SAMPLE_RATES.map(|rate| rate.0) {
            let probes = cascade_probes(rate);
            let high_state = TptSvf::design(rate, 100.0, true).expect("high pass");
            let low_state = TptSvf::design(rate, 1_000.0, false).expect("low pass");
            let high_space = ReferenceTptStateSpace::from_cast_coefficients(
                high_state.c1,
                high_state.a2,
                high_state.a3,
                high_state.k,
                ReferenceTptOutput::HighPass,
            );
            let low_space = ReferenceTptStateSpace::from_cast_coefficients(
                low_state.c1,
                low_state.a2,
                low_state.a3,
                low_state.k,
                ReferenceTptOutput::LowPass,
            );
            for frequency in probes
                .iter()
                .copied()
                .chain([100.0, 1_000.0, 0.49 * f64::from(rate)])
            {
                let reference = rbj_butterworth_magnitude_db(
                    f64::from(rate),
                    100.0,
                    ReferenceFilterKind::HighPass,
                    frequency,
                )
                .expect("reference high")
                    + rbj_butterworth_magnitude_db(
                        f64::from(rate),
                        1_000.0,
                        ReferenceFilterKind::LowPass,
                        frequency,
                    )
                    .expect("reference low");
                let actual = high_space
                    .magnitude_db(f64::from(rate), frequency)
                    .expect("state high")
                    + low_space
                        .magnitude_db(f64::from(rate), frequency)
                        .expect("state low");
                if reference >= -120.0 {
                    assert!(
                        (actual - reference).abs() <= 0.005,
                        "analytic rate={rate}, frequency={frequency}, actual={actual}, reference={reference}"
                    );
                }
            }
            for quantum in [1, 127, 128, 255, 1_024] {
                let mut high = TptSvf::design(rate, 100.0, true).expect("high pass");
                let mut low = TptSvf::design(rate, 1_000.0, false).expect("low pass");
                let mut report = BuiltinProcessReport::default();
                let mut recovered = 0;
                let mut impulse = vec![0.0_f32; rate as usize];
                for start in (0..impulse.len()).step_by(quantum) {
                    let end = (start + quantum).min(impulse.len());
                    for (offset, sample) in impulse[start..end].iter_mut().enumerate() {
                        let input = if start + offset == 0 { 1.0 } else { 0.0 };
                        let high_output = high.process(input, &mut recovered, &mut report);
                        *sample = low.process(high_output, &mut recovered, &mut report);
                    }
                }
                assert!(impulse.iter().all(|sample| sample.is_finite()));
                for frequency in &probes {
                    let reference = rbj_butterworth_magnitude_db(
                        f64::from(rate),
                        100.0,
                        ReferenceFilterKind::HighPass,
                        *frequency,
                    )
                    .expect("reference high")
                        + rbj_butterworth_magnitude_db(
                            f64::from(rate),
                            1_000.0,
                            ReferenceFilterKind::LowPass,
                            *frequency,
                        )
                        .expect("reference low");
                    let actual = impulse_dft_magnitude_db(&impulse, f64::from(rate), *frequency);
                    if reference >= -120.0 {
                        assert!(
                            (actual - reference).abs() <= 0.05,
                            "impulse rate={rate}, quantum={quantum}, frequency={frequency}, actual={actual}, reference={reference}"
                        );
                    } else {
                        assert!(
                            actual <= -115.0,
                            "impulse rate={rate}, quantum={quantum}, frequency={frequency}, actual={actual}"
                        );
                    }
                }
            }
            for frequency in probes {
                let measurement = sustained_cascade_measurement(rate, frequency);
                if measurement.reference_gain_db >= -90.0 {
                    assert!(
                        (measurement.production_gain_db - measurement.reference_gain_db).abs()
                            <= 0.05,
                        "sustained rate={rate}, frequency={frequency}, production={}, reference={}",
                        measurement.production_gain_db,
                        measurement.reference_gain_db
                    );
                    assert!(
                        measurement.residual_db <= -100.0,
                        "sustained rate={rate}, frequency={frequency}, residual={}",
                        measurement.residual_db
                    );
                } else {
                    assert!(
                        measurement.total_output_db <= -88.0,
                        "sustained rate={rate}, frequency={frequency}, output={}",
                        measurement.total_output_db
                    );
                }
            }
        }
    }

    struct SustainedMeasurement {
        production_gain_db: f64,
        reference_gain_db: f64,
        residual_db: f64,
        total_output_db: f64,
    }

    fn sustained_measurement(
        production: &mut TptSvf,
        reference: &mut ReferenceBiquad,
        rate: u32,
        frequency: f64,
    ) -> SustainedMeasurement {
        let settle = rate as usize / 2;
        let frames = rate as usize / 4;
        let mut report = BuiltinProcessReport::default();
        let mut recovered = 0;
        let mut input_energy = 0.0_f64;
        let mut output_energy = 0.0_f64;
        let mut measured_outputs = Vec::with_capacity(frames);
        let mut reference_outputs = Vec::with_capacity(frames);
        let rate_f64 = f64::from(rate);
        for index in 0..settle + frames {
            let phase = core::f64::consts::TAU * frequency * index as f64 / rate_f64;
            let input = (0.5 * phase.sin()) as f32;
            let output = production.process(input, &mut recovered, &mut report);
            let reference_output = reference.process(f64::from(input));
            if index >= settle {
                let output = f64::from(output);
                measured_outputs.push(output);
                reference_outputs.push(reference_output);
                input_energy += f64::from(input) * f64::from(input);
                output_energy += output * output;
            }
        }
        let frames_f64 = frames as f64;
        let input_rms = (input_energy / frames_f64).sqrt();
        let [
            production_dc,
            production_sine_coefficient,
            production_cosine_coefficient,
        ] = fit_dc_sine_cosine(&measured_outputs, settle, rate_f64, frequency);
        let [_, reference_sine_coefficient, reference_cosine_coefficient] =
            fit_dc_sine_cosine(&reference_outputs, settle, rate_f64, frequency);
        let production_amplitude = production_sine_coefficient.hypot(production_cosine_coefficient);
        let reference_amplitude = reference_sine_coefficient.hypot(reference_cosine_coefficient);
        let mut residual_energy = 0.0_f64;
        for (offset, output) in measured_outputs.iter().copied().enumerate() {
            let index = settle + offset;
            let phase = core::f64::consts::TAU * frequency * index as f64 / rate_f64;
            let fitted = production_dc
                + production_sine_coefficient * phase.sin()
                + production_cosine_coefficient * phase.cos();
            residual_energy += (output - fitted).powi(2);
        }
        let residual_rms = (residual_energy / frames_f64).sqrt();
        let output_rms = (output_energy / frames_f64).sqrt();
        SustainedMeasurement {
            production_gain_db: 20.0 * (production_amplitude / 0.5).log10(),
            reference_gain_db: 20.0 * (reference_amplitude / 0.5).log10(),
            residual_db: 20.0 * (residual_rms / input_rms).log10(),
            total_output_db: 20.0 * (output_rms / input_rms).log10(),
        }
    }

    fn sustained_cascade_measurement(rate: u32, frequency: f64) -> SustainedMeasurement {
        let mut high = TptSvf::design(rate, 100.0, true).expect("high pass");
        let mut low = TptSvf::design(rate, 1_000.0, false).expect("low pass");
        let mut high_reference =
            ReferenceBiquad::rbj_butterworth(f64::from(rate), 100.0, ReferenceFilterKind::HighPass)
                .expect("reference high");
        let mut low_reference = ReferenceBiquad::rbj_butterworth(
            f64::from(rate),
            1_000.0,
            ReferenceFilterKind::LowPass,
        )
        .expect("reference low");
        let settle = rate as usize / 2;
        let frames = rate as usize / 4;
        let rate_f64 = f64::from(rate);
        let mut report = BuiltinProcessReport::default();
        let mut recovered = 0;
        let mut input_energy = 0.0_f64;
        let mut output_energy = 0.0_f64;
        let mut measured_outputs = Vec::with_capacity(frames);
        let mut reference_outputs = Vec::with_capacity(frames);
        for index in 0..settle + frames {
            let phase = core::f64::consts::TAU * frequency * index as f64 / rate_f64;
            let input = (0.5 * phase.sin()) as f32;
            let high_output = high.process(input, &mut recovered, &mut report);
            let output = low.process(high_output, &mut recovered, &mut report);
            let reference = low_reference.process(high_reference.process(f64::from(input)));
            if index >= settle {
                let output = f64::from(output);
                measured_outputs.push(output);
                reference_outputs.push(reference);
                input_energy += f64::from(input) * f64::from(input);
                output_energy += output * output;
            }
        }
        let frames_f64 = frames as f64;
        let input_rms = (input_energy / frames_f64).sqrt();
        let [dc, sine, cosine] = fit_dc_sine_cosine(&measured_outputs, settle, rate_f64, frequency);
        let [_, reference_sine, reference_cosine] =
            fit_dc_sine_cosine(&reference_outputs, settle, rate_f64, frequency);
        let production_amplitude = sine.hypot(cosine);
        let reference_amplitude = reference_sine.hypot(reference_cosine);
        let residual_energy = measured_outputs
            .iter()
            .copied()
            .enumerate()
            .map(|(offset, output)| {
                let phase =
                    core::f64::consts::TAU * frequency * (settle + offset) as f64 / rate_f64;
                (output - (dc + sine * phase.sin() + cosine * phase.cos())).powi(2)
            })
            .sum::<f64>();
        SustainedMeasurement {
            production_gain_db: 20.0 * (production_amplitude / 0.5).log10(),
            reference_gain_db: 20.0 * (reference_amplitude / 0.5).log10(),
            residual_db: 20.0 * ((residual_energy / frames_f64).sqrt() / input_rms).log10(),
            total_output_db: 20.0 * ((output_energy / frames_f64).sqrt() / input_rms).log10(),
        }
    }

    fn fit_dc_sine_cosine(
        samples: &[f64],
        first_index: usize,
        rate: f64,
        frequency: f64,
    ) -> [f64; 3] {
        let mut normal = [[0.0_f64; 3]; 3];
        let mut right = [0.0_f64; 3];
        for (offset, sample) in samples.iter().copied().enumerate() {
            let phase = core::f64::consts::TAU * frequency * (first_index + offset) as f64 / rate;
            let basis = [1.0, phase.sin(), phase.cos()];
            for row in 0..3 {
                right[row] += basis[row] * sample;
                for column in 0..3 {
                    normal[row][column] += basis[row] * basis[column];
                }
            }
        }
        solve_three_by_three([
            [normal[0][0], normal[0][1], normal[0][2], right[0]],
            [normal[1][0], normal[1][1], normal[1][2], right[1]],
            [normal[2][0], normal[2][1], normal[2][2], right[2]],
        ])
    }

    fn solve_three_by_three(mut augmented: [[f64; 4]; 3]) -> [f64; 3] {
        for column in 0..3 {
            let mut pivot = column;
            for row in column + 1..3 {
                if augmented[row][column].abs() > augmented[pivot][column].abs() {
                    pivot = row;
                }
            }
            augmented.swap(column, pivot);
            let divisor = augmented[column][column];
            assert!(divisor.is_finite() && divisor.abs() > f64::EPSILON);
            for value in &mut augmented[column][column..] {
                *value /= divisor;
            }
            let pivot_row = augmented[column];
            for (row_index, row) in augmented.iter_mut().enumerate() {
                if row_index == column {
                    continue;
                }
                let factor = row[column];
                for (value, pivot_value) in row[column..].iter_mut().zip(&pivot_row[column..]) {
                    *value -= factor * pivot_value;
                }
            }
        }
        [augmented[0][3], augmented[1][3], augmented[2][3]]
    }

    fn assert_tpt_limits_and_monotonic(
        state: ReferenceTptStateSpace,
        rate: u32,
        high_pass: bool,
        cutoff: f64,
    ) {
        let nyquist = 0.5 * f64::from(rate);
        let magnitude = |frequency| {
            let (real, imaginary) = state
                .response(f64::from(rate), frequency)
                .expect("finite state-space response");
            real.hypot(imaginary)
        };
        let (dc, at_nyquist) = (magnitude(0.0), magnitude(nyquist));
        if high_pass {
            assert!(
                dc <= 1e-6,
                "HPF DC limit: rate={rate}, cutoff={cutoff}, value={dc}"
            );
            assert!(
                (at_nyquist - 1.0).abs() <= 1e-6,
                "HPF Nyquist limit: rate={rate}, cutoff={cutoff}, value={at_nyquist}"
            );
        } else {
            assert!(
                (dc - 1.0).abs() <= 1e-6,
                "LPF DC limit: rate={rate}, cutoff={cutoff}, value={dc}"
            );
            assert!(
                at_nyquist <= 1e-6,
                "LPF Nyquist limit: rate={rate}, cutoff={cutoff}, value={at_nyquist}"
            );
        }
        let mut previous = magnitude(0.0);
        for index in 1..=4_096 {
            let current = magnitude(nyquist * f64::from(index) / 4_096.0);
            if high_pass {
                assert!(
                    current + 2e-6 >= previous,
                    "HPF monotonicity: rate={rate}, cutoff={cutoff}, index={index}, previous={previous}, current={current}"
                );
            } else {
                assert!(
                    current <= previous + 2e-6,
                    "LPF monotonicity: rate={rate}, cutoff={cutoff}, index={index}, previous={previous}, current={current}"
                );
            }
            previous = current;
        }
    }

    fn coherent_probes(rate: u32, cutoff: f64) -> Vec<f64> {
        let nyquist = 0.5 * f64::from(rate);
        let mut probes = [
            0.25 * cutoff,
            cutoff,
            4.0 * cutoff,
            0.2 * f64::from(rate),
            0.45 * f64::from(rate),
        ]
        .into_iter()
        .map(|probe| probe.clamp(4.0, nyquist - 4.0))
        .map(|probe| (probe / 4.0).round() * 4.0)
        .collect::<Vec<_>>();
        probes.sort_by(f64::total_cmp);
        probes.dedup_by(|left, right| *left == *right);
        probes
    }

    fn cascade_probes(rate: u32) -> Vec<f64> {
        let mut probes = coherent_probes(rate, 100.0);
        probes.extend(coherent_probes(rate, 1_000.0));
        probes.sort_by(f64::total_cmp);
        probes.dedup_by(|left, right| *left == *right);
        probes
    }

    fn impulse_dft_magnitude_db(samples: &[f32], rate: f64, frequency: f64) -> f64 {
        let phase = -core::f64::consts::TAU * frequency / rate;
        let (step_real, step_imaginary) = (phase.cos(), phase.sin());
        let (mut unit_real, mut unit_imaginary) = (1.0_f64, 0.0_f64);
        let (mut real, mut imaginary) = (0.0_f64, 0.0_f64);
        for sample in samples {
            let sample = f64::from(*sample);
            real += sample * unit_real;
            imaginary += sample * unit_imaginary;
            (unit_real, unit_imaginary) = (
                unit_real * step_real - unit_imaginary * step_imaginary,
                unit_real * step_imaginary + unit_imaginary * step_real,
            );
        }
        let magnitude = real.hypot(imaginary);
        if magnitude == 0.0 {
            f64::NEG_INFINITY
        } else {
            20.0 * magnitude.log10()
        }
    }
}
