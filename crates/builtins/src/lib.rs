//! The builtin track chain — polarity/trim, high-pass, low-pass, fader/mute, 2x2 matrix — and the
//! transparent meter taps around it (issues 007 and 085).
//!
//! # One body per pass
//!
//! Every sample loop in this crate is a `lane::kernels` block kernel, generic over
//! [`Lane`] and instantiated at `f32`, `Simd4` and `Simd8` from one source. A scalar track is
//! `InputStage<f32>` over planar slices; a bank is the same type at four or eight lanes over an
//! AoSoA block. There is no second arithmetic graph, so a track's bits do not depend on its cohort
//! membership or on the host (master plan #83 D5, §4).
//!
//! # Where the checks are (D7)
//!
//! Input is sanitised once per channel per block, here, because this crate *is* the input stage.
//! The two recursive state words of each filter section are flushed in-kernel. Output finiteness
//! is checked once per block, per lane, on the output of the recursive stage; a failing lane is
//! zeroed and its state reset, and no other lane's bits move. Fader, trim and matrix are
//! feed-forward with bounded coefficients, so finite in implies finite out and they carry no
//! checks at all.
#![allow(missing_docs)]

use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};

use engine::{
    SampleRateHz, is_extended_compatibility_sample_rate,
    realtime::{Consumer, Producer, QueueGeneration, bounded_spsc},
};
pub mod corpus;

use effect_contract::{BankWidth, ChannelSymmetryWitness};
use lane::{
    Backend, Lane, Simd4, Simd8,
    kernels::{
        SvfCoef,
        builtins::{
            GainMuteRamp, InputChainCoef, InputChainPlan, InputChainState, InputTrimRamp,
            Matrix2x2Coef, Matrix2x2Ramp, fader_matrix_block, gain_mute_block,
            gain_mute_ramp_block, input_chain_block_elided, input_chain_block_mono_elided,
            input_chain_plan, input_chain_ramp_block, input_chain_ramp_block_mono, lanes_below,
            mask_from_flags, matrix2x2_block, matrix2x2_ramp_block, no_lanes,
            plan_is_channel_symmetric, zero_lanes_block,
        },
    },
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

/// What one `process` call sanitised and recovered.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BuiltinProcessReport {
    /// Input samples replaced by positive zero at the input stage, over both channels.
    ///
    /// A sample is sanitised when its magnitude is not below `1e30`, which includes every NaN
    /// (D7). A subnormal input is **not** sanitised: it is a legal finite sample.
    pub sanitized_input: u64,
    /// Always zero. Retained for API stability: D7 replaced per-sample output sanitisation with
    /// the once-per-block boundary check the two counters below report.
    pub sanitized_output: u64,
    /// Left-channel lane-blocks whose output failed the boundary check.
    ///
    /// The check runs once per block, per lane, on the output of the recursive stage; a failing
    /// lane has its block zeroed and both of its sections reset. This counts lane-blocks, not
    /// samples, and never counts a padding lane.
    pub recovered_left_state: u64,
    /// Right-channel lane-blocks whose output failed the boundary check; see
    /// [`BuiltinProcessReport::recovered_left_state`].
    pub recovered_right_state: u64,
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
pub struct BuiltinParameterDescriptor {
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
    /// Exact-decimal persisted-value lattice and named step ladder.
    pub lattice: effect_contract::ParameterLattice,
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
    DisabledOrRateKeyedHertz { disabled: f32, minimum_hz: f32 },
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
            Self::DisabledOrRateKeyedHertz {
                disabled,
                minimum_hz,
            } => validate_builtin_filter_cutoff(value, sample_rate, disabled, minimum_hz).is_ok(),
        }
    }
}

/// The exact, inclusive cutoff maximum for one launch rate under the retained `f32` TPT state.
///
/// These are greatest contiguous shared HPF/LPF maxima.  The immediate successor of each value
/// is deliberately outside the public prepared domain.
#[must_use]
pub const fn builtin_filter_cutoff_maximum_hz(sample_rate: u32) -> Option<f32> {
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
pub fn validate_builtin_filter_cutoff(
    value: f32,
    sample_rate: u32,
    disabled: f32,
    minimum_hz: f32,
) -> Result<(), BuiltinParameterError> {
    let launch_maximum = builtin_filter_cutoff_maximum_hz(sample_rate);
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

/// Reserved persisted-wire index for a descriptor's out-of-range disabled sentinel.
pub const DISABLED_LATTICE_INDEX: u32 = u32::MAX;

/// One builtin lattice at a prepared sample rate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuiltinLatticePoints {
    /// Sorted in-domain values. Their `index` is the persisted step index.
    pub points: Vec<effect_contract::LatticePoint>,
    /// Canonical disabled sentinel carried by [`DISABLED_LATTICE_INDEX`], when declared.
    pub disabled: Option<String>,
}

/// Resolve a builtin descriptor into the shared lattice machinery.
///
/// The builtin vocabulary has a rate-keyed cutoff domain that effect descriptors do not. This
/// adapter supplies the selected rate's declared maximum, then delegates all arithmetic,
/// geometric rendering, intrinsic endpoints/defaults and index ordering to effect-contract's one
/// authority. The disabled sentinel stays outside the ordered domain under its reserved index.
pub fn builtin_parameter_lattice_points(
    descriptor: &BuiltinParameterDescriptor,
    sample_rate: u32,
) -> Result<BuiltinLatticePoints, effect_contract::LatticeError> {
    use effect_contract::{
        AutomationRate, ParameterChannelPolicy, ParameterDescriptor, ParameterDomain,
        ParameterMapping, ParameterUnit, SmoothingRule, canonical_descriptor_decimal,
        parameter_lattice_points_parts,
    };

    // `maximum_is_member` distinguishes a DECLARED bound, which #239 ruling
    // 5461507633 B2 makes a lattice member outright, from S1's rate-keyed
    // CLAMP, which is a physical ceiling the descriptor never declared and
    // whose top point is therefore the greatest generated value at or below it.
    let (domain, minimum, maximum, default_value, maximum_is_member) = match descriptor.domain {
        BuiltinParameterDomain::BooleanExact => (ParameterDomain::Boolean, None, None, 0.0, true),
        BuiltinParameterDomain::FiniteInclusive { minimum, maximum } => (
            ParameterDomain::Continuous,
            Some(minimum),
            Some(maximum),
            descriptor.default,
            true,
        ),
        BuiltinParameterDomain::DisabledOrRateKeyedHertz { minimum_hz, .. } => (
            ParameterDomain::Continuous,
            Some(minimum_hz),
            Some(
                builtin_filter_cutoff_maximum_hz(sample_rate)
                    .ok_or(effect_contract::LatticeError::Declaration)?,
            ),
            // The actual default is the disabled sentinel and stays outside this ordered set.
            minimum_hz,
            false,
        ),
    };
    let unit = match descriptor.mapping {
        BuiltinParameterMapping::DecibelAmplitude => ParameterUnit::Db,
        BuiltinParameterMapping::Hertz => ParameterUnit::Hz,
        BuiltinParameterMapping::Boolean | BuiltinParameterMapping::Linear => {
            if descriptor.name == "delay_samples" {
                ParameterUnit::Samples
            } else {
                ParameterUnit::Linear
            }
        }
    };
    let mapping = match descriptor.mapping {
        BuiltinParameterMapping::Boolean => ParameterMapping::Stepped,
        BuiltinParameterMapping::Hertz => ParameterMapping::Logarithmic,
        BuiltinParameterMapping::DecibelAmplitude | BuiltinParameterMapping::Linear => {
            ParameterMapping::Linear
        }
    };
    let parameter = ParameterDescriptor {
        id: effect_contract::ParameterId(descriptor.id),
        display_name: descriptor.name,
        display_unit: "builtin",
        unit,
        domain,
        minimum,
        maximum,
        default_value,
        mapping,
        automation_rate: AutomationRate::None,
        channel_policy: match descriptor.scope {
            BuiltinParameterScope::PerLane => ParameterChannelPolicy::PerLane,
            BuiltinParameterScope::MatrixShared => ParameterChannelPolicy::Shared,
        },
        smoothing: SmoothingRule::None,
        smoothing_samples: 0,
        readable: true,
        automatable: false,
        enum_choices: &[],
        lattice: descriptor.lattice,
    };
    let points = parameter_lattice_points_parts(
        parameter.unit,
        parameter.domain,
        parameter.mapping,
        parameter.minimum,
        parameter.maximum,
        parameter.default_value,
        &[],
        parameter.lattice,
        maximum_is_member,
    )?;
    let disabled = descriptor
        .disabled_value
        .map(|value| {
            canonical_descriptor_decimal(value, descriptor.lattice.precision)
                .ok_or(effect_contract::LatticeError::Declaration)
        })
        .transpose()?;
    Ok(BuiltinLatticePoints { points, disabled })
}

pub const BUILTIN_PARAMETER_DESCRIPTORS: [BuiltinParameterDescriptor; 12] = [
    BuiltinParameterDescriptor {
        id: 1,
        name: "polarity_invert",
        scope: BuiltinParameterScope::PerLane,
        mapping: BuiltinParameterMapping::Boolean,
        domain: BuiltinParameterDomain::BooleanExact,
        default: 0.0,
        // Live since #210 phase 3 (command kind 11). A flip is a retarget of the **trim**
        // coefficient to its own negation, so the declick is the trim ramp's and the row's
        // smoothing policy is the trim's: `LinearNUpdates`, the linear-N law, carrying the
        // coefficient through zero. The row is `BlockTarget` because a flip lands at a block
        // boundary like every other live builtin move.
        update_rate: BuiltinParameterUpdateRate::BlockTarget,
        smoothing: BuiltinSmoothingPolicy::LinearNUpdates,
        reset: BuiltinParameterReset::RestorePreparedValue,
        disabled_value: None,
        lattice: effect_contract::ParameterLattice::indices(),
    },
    BuiltinParameterDescriptor {
        id: 2,
        name: "trim_db",
        scope: BuiltinParameterScope::PerLane,
        mapping: BuiltinParameterMapping::DecibelAmplitude,
        domain: BuiltinParameterDomain::FiniteInclusive {
            minimum: -144.0,
            maximum: 24.0,
        },
        default: 0.0,
        // Live since #210 phase 3 (command kind 10): the input chain's trim coefficient steps per
        // sample under the linear-N law, in `input_chain_ramp_block`, exactly as `fader_db` does
        // in `gain_mute_ramp_block`. Gain-riding trim ahead of the compressor is the workflow the
        // D2 ruling adopted this for.
        update_rate: BuiltinParameterUpdateRate::BlockTarget,
        smoothing: BuiltinSmoothingPolicy::LinearNUpdates,
        reset: BuiltinParameterReset::RestorePreparedValue,
        disabled_value: None,
        lattice: effect_contract::ParameterLattice::arithmetic(0.1, 1),
    },
    BuiltinParameterDescriptor {
        id: 3,
        name: "hpf_hz",
        scope: BuiltinParameterScope::PerLane,
        mapping: BuiltinParameterMapping::Hertz,
        domain: BuiltinParameterDomain::DisabledOrRateKeyedHertz {
            disabled: 0.0,
            minimum_hz: 10.0,
        },
        default: 0.0,
        update_rate: BuiltinParameterUpdateRate::PreparedOnly,
        smoothing: BuiltinSmoothingPolicy::None,
        reset: BuiltinParameterReset::RestorePreparedValue,
        disabled_value: Some(0.0),
        lattice: effect_contract::ParameterLattice::cents(20.0, 3),
    },
    BuiltinParameterDescriptor {
        id: 4,
        name: "lpf_hz",
        scope: BuiltinParameterScope::PerLane,
        mapping: BuiltinParameterMapping::Hertz,
        domain: BuiltinParameterDomain::DisabledOrRateKeyedHertz {
            disabled: 0.0,
            minimum_hz: 10.0,
        },
        default: 0.0,
        update_rate: BuiltinParameterUpdateRate::PreparedOnly,
        smoothing: BuiltinSmoothingPolicy::None,
        reset: BuiltinParameterReset::RestorePreparedValue,
        disabled_value: Some(0.0),
        lattice: effect_contract::ParameterLattice::cents(20.0, 3),
    },
    // Issue #140 B: `fader_db` and `mute` become block targets with linear-N smoothing, because
    // the engine now has a post-preparation write path for them -- `FaderMuteRampBuiltins`,
    // bound by `ConsoleFaderProcessor` for a track a live console drives. This row states the
    // parameter's *capability*, exactly as `matrix_ll..rr` do: a session with no console has
    // nothing that writes either surface, and the prepared `FaderMuteBuiltins` it binds instead
    // is unchanged. `mute` is smoothed for the same reason it is a block target: a mute is a
    // retarget of the same gain to zero, over the same ramp window, not a discontinuity.
    BuiltinParameterDescriptor {
        id: 5,
        name: "fader_db",
        scope: BuiltinParameterScope::PerLane,
        mapping: BuiltinParameterMapping::DecibelAmplitude,
        domain: BuiltinParameterDomain::FiniteInclusive {
            minimum: -144.0,
            maximum: 24.0,
        },
        default: 0.0,
        update_rate: BuiltinParameterUpdateRate::BlockTarget,
        smoothing: BuiltinSmoothingPolicy::LinearNUpdates,
        reset: BuiltinParameterReset::RestorePreparedValue,
        disabled_value: None,
        lattice: effect_contract::ParameterLattice::arithmetic(0.1, 1)
            .with_ladder(effect_contract::FADER_STEP_LADDER),
    },
    BuiltinParameterDescriptor {
        id: 6,
        name: "mute",
        scope: BuiltinParameterScope::PerLane,
        mapping: BuiltinParameterMapping::Boolean,
        domain: BuiltinParameterDomain::BooleanExact,
        default: 0.0,
        update_rate: BuiltinParameterUpdateRate::BlockTarget,
        smoothing: BuiltinSmoothingPolicy::LinearNUpdates,
        reset: BuiltinParameterReset::RestorePreparedValue,
        disabled_value: None,
        lattice: effect_contract::ParameterLattice::indices(),
    },
    BuiltinParameterDescriptor {
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
        lattice: effect_contract::ParameterLattice::arithmetic(0.01, 2),
    },
    BuiltinParameterDescriptor {
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
        lattice: effect_contract::ParameterLattice::arithmetic(0.01, 2),
    },
    BuiltinParameterDescriptor {
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
        lattice: effect_contract::ParameterLattice::arithmetic(0.01, 2),
    },
    BuiltinParameterDescriptor {
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
        lattice: effect_contract::ParameterLattice::arithmetic(0.01, 2),
    },
    // Issue #210 phase 2. Appended rather than inserted: the contract's self-test compares whole
    // positional arrays, and appending shifts no existing row's index.
    //
    // `PreparedOnly` with `None` smoothing is the design's ruling, not an omission: changing a
    // delay length mid-render re-times the ring and glitches unavoidably, and the declicked
    // variant (a crossfaded dual read) is a recorded follow-up rather than speculative machinery.
    // The change path is the transactional session edit every other prepared-only builtin uses.
    BuiltinParameterDescriptor {
        id: 11,
        name: "delay_samples",
        scope: BuiltinParameterScope::PerLane,
        mapping: BuiltinParameterMapping::Linear,
        domain: BuiltinParameterDomain::FiniteInclusive {
            minimum: 0.0,
            maximum: 48_000.0,
        },
        default: 0.0,
        update_rate: BuiltinParameterUpdateRate::PreparedOnly,
        smoothing: BuiltinSmoothingPolicy::None,
        reset: BuiltinParameterReset::RestorePreparedValue,
        // Zero is "no delay", but `disabled_value` is the cutoff contract's escape hatch -- the
        // value that takes a parameter *out* of its own domain (`hpf_hz = 0.0` is not a cutoff at
        // all). Zero samples is an ordinary, in-domain point of a flat integer range, so this row
        // has no disabled value in that sense.
        disabled_value: None,
        lattice: effect_contract::ParameterLattice::arithmetic(1.0, 0),
    },
    // #239 ruling 5461507633 B4: pan remains persisted intent and therefore owns a descriptor
    // row rather than being canonicalized into matrix coefficients. One row is per-lane: session
    // `left` and `right` pan words address the same stable parameter ID on different lanes.
    BuiltinParameterDescriptor {
        id: 12,
        name: "pan",
        scope: BuiltinParameterScope::PerLane,
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
        lattice: effect_contract::ParameterLattice::arithmetic(0.01, 2),
    },
];

/// One prepared second-order TPT state-variable section, in the master-plan §4.2 A1 storage form.
///
/// The stored damping coefficient is `c1 = t / (1 + t)` with `t = g * (g + k)`, never
/// `a1 = 1 / (1 + t)`: at a low cutoff and a high Q, `a1` rounded to `f32` carries about 0.6 %
/// relative error in the pole damping while `c1` carries about 6e-8 (#87, amendment A1). The
/// output selection is the `(m0, m1, m2)` mix of the shared kernel — high-pass `(1, -k, -1)`,
/// low-pass `(0, 0, 1)` — so a bank does not need a per-lane high-pass mask, and a disabled
/// section is the arithmetic identity `(1, 0, 0)` with zero coefficients rather than a branch.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SvfSection {
    /// `t / (1 + t)`, the damping coefficient.
    pub c1: f32,
    /// `g * (1 - c1)`.
    pub a2: f32,
    /// `g * a2`.
    pub a3: f32,
    /// `1 / Q`; Butterworth throughout, so `sqrt(2)`.
    pub k: f32,
    /// Direct output mix.
    pub m0: f32,
    /// Band output mix.
    pub m1: f32,
    /// Low output mix.
    pub m2: f32,
    /// Whether a cutoff was designed; a disabled section is the arithmetic identity.
    pub enabled: bool,
}

impl SvfSection {
    /// The disabled section: zero coefficients and the direct mix, so `y = 1 * v0 + 0 + 0`.
    pub(crate) const IDENTITY: Self = Self {
        c1: 0.0,
        a2: 0.0,
        a3: 0.0,
        k: 0.0,
        m0: 1.0,
        m1: 0.0,
        m2: 0.0,
        enabled: false,
    };

    /// Designs the Butterworth (`k = sqrt(2)`) section for a cutoff; `0.0` is the identity.
    ///
    /// The design is `f64` throughout in the frozen operation order below, with exactly one cast
    /// per stored word. Rejection is coefficient representability only: the public cutoff domain
    /// is the issue-036 table, enforced before preparation by
    /// [`validate_builtin_filter_cutoff`].
    fn design(rate: u32, cutoff: f32, high_pass: bool) -> Result<Self, BuiltinParameterError> {
        if cutoff == 0.0 {
            return Ok(Self::IDENTITY);
        }
        if rate == 0 {
            return Err(BuiltinParameterError::FilterCoefficients);
        }
        let g = math::tan(core::f64::consts::PI * f64::from(cutoff) / f64::from(rate));
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
        let (m0, m1, m2) = if high_pass {
            (1.0, -k, -1.0)
        } else {
            (0.0, 0.0, 1.0)
        };
        Ok(Self {
            c1,
            a2,
            a3,
            k,
            m0,
            m1,
            m2,
            enabled: true,
        })
    }

    /// The seven stored words, in the order the evidence and fixture tools read them.
    const fn words(self) -> [u32; 7] {
        [
            self.c1.to_bits(),
            self.a2.to_bits(),
            self.a3.to_bits(),
            self.k.to_bits(),
            self.m0.to_bits(),
            self.m1.to_bits(),
            self.m2.to_bits(),
        ]
    }
}

/// One prepared input channel: the folded trim and its two cascaded sections.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct InputLane {
    /// Trim with the polarity inversion folded in, so the render path has no per-sample branch.
    pub trim_signed: f32,
    /// High-pass section, applied first.
    pub hpf: SvfSection,
    /// Low-pass section, applied second.
    pub lpf: SvfSection,
}

/// The prepared input record of one track, both channels. Plain data: this is what a bank is
/// built from, and it is the only thing preparation produces for the input section.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PreparedInputTrack {
    /// Left channel.
    pub left: InputLane,
    /// Right channel.
    pub right: InputLane,
}

#[derive(Clone, Copy)]
struct FaderLane {
    gain: f32,
    muted: bool,
}

/// Broadcasts one `f32` per lane into a [`Lane`] value.
#[inline]
fn lane_words<L: Lane>(words: &[f32; MAX_BANK_LANES]) -> L {
    debug_assert!(L::WIDTH <= MAX_BANK_LANES);
    L::load(&words[..L::WIDTH])
}

/// Reads a [`Lane`] value back into one `f32` per lane.
#[inline]
fn lane_read<L: Lane>(value: L) -> [f32; MAX_BANK_LANES] {
    let mut words = [0.0_f32; MAX_BANK_LANES];
    value.store(&mut words[..L::WIDTH]);
    words
}

/// Widest bank this crate builds. `BankWidth` is four or eight (D4).
const MAX_BANK_LANES: usize = 8;

/// The damping of every builtin section: Butterworth, `k = 1 / Q = sqrt(2)`, rounded once.
const BUTTERWORTH_K: f32 = core::f64::consts::SQRT_2 as f32;

/// Builds the kernel coefficient set of one section from one [`SvfSection`] per lane.
fn svf_coef<L: Lane>(sections: &[SvfSection; MAX_BANK_LANES]) -> SvfCoef<L> {
    let pick = |select: fn(&SvfSection) -> f32| -> L {
        let mut words = [0.0_f32; MAX_BANK_LANES];
        for (word, section) in words.iter_mut().zip(sections.iter()) {
            *word = select(section);
        }
        lane_words::<L>(&words)
    };
    SvfCoef {
        c1: pick(|section| section.c1),
        a2: pick(|section| section.a2),
        a3: pick(|section| section.a3),
        m0: pick(|section| section.m0),
        m1: pick(|section| section.m1),
        m2: pick(|section| section.m2),
    }
}

/// The builtin input stage — sanitise, trim, high-pass, low-pass, boundary check — at one width.
///
/// There is exactly one body: a scalar track is `InputStage<f32>` over planar slices, and a bank
/// is the same type at `Simd4` or `Simd8` over AoSoA blocks (master plan §4.1). Lane identity is
/// therefore a property of the code.
///
/// Lanes at or above [`InputStage::members`] are **padding lanes**. They carry
/// [`SvfSection::IDENTITY`] coefficients and unit trim, so they are arithmetically inert; they run
/// through every pass, and they are excluded from every counter and from the boundary check by
/// [`InputStage::active`]. Their samples are never observed.
pub(crate) struct InputStage<L: Lane> {
    /// Populated lanes, `1..=L::WIDTH`.
    members: usize,
    /// Lanes below [`InputStage::members`].
    active: L::Mask,
    /// Folded trim and the four section coefficient sets; `[channel][section]`, section `0` is
    /// the high-pass.
    coef: InputChainCoef<L>,
    /// Retained integrator state, indexed like [`InputChainCoef::section`].
    state: InputChainState<L>,
    /// Which sections the render path may skip, decided from the words in [`InputStage::coef`]
    /// and [`InputStage::state`] and re-decided by every write to either.
    ///
    /// # Why a live trim cannot make this stale (issue #210 phase 3)
    ///
    /// The plan is a pure function of the **six SVF coefficient words per section and the two
    /// integrators per section** -- [`section_is_identity`] reads those eight and nothing else.
    /// `trim` is not among them. `hpf_hz` and `lpf_hz`, which are the only parameters that design
    /// those six words, remain `PreparedOnly` in `BUILTIN_PARAMETER_DESCRIPTORS`, so the
    /// coefficients are still written exactly once, here.
    ///
    /// `trim_db` and `polarity_invert` are live since phase 3 and write [`InputStage::coef`]'s
    /// `trim` word after preparation -- but a section's elidability does not depend on the value
    /// the chain multiplies its input by, only on whether the section is the arithmetic identity,
    /// so that write cannot flip a `true` to a `false` or the reverse. The decision is therefore
    /// still taken once, at the point the section words are written, and cannot go stale.
    ///
    /// **The proviso this leaves for a later phase**: if `hpf_hz` or `lpf_hz` ever become live,
    /// the premise above fails at exactly the word list this comment names, and that liveness
    /// requires a command-driven invalidation -- the retarget must recompute the plan the way
    /// [`InputStage::set_lane_state_words`] and [`InputStage::reset`] already do. See
    /// `docs/rulings/builtins-input-liveness-d2.md`.
    ///
    /// The state words are written by [`InputStage::reset`] and by the evidence-only
    /// [`InputStage::set_lane_state_words`], and both re-decide the plan.
    ///
    /// The render path writes `state` in exactly one place -- the boundary-check recovery in
    /// [`InputStage::process`], which is `state.andnot(bad)` -- and that write is *monotone*
    /// toward the identity: it either leaves a word alone or replaces it with the `+0.0` the
    /// identity pattern wants. So it can never invalidate a `true`, and it is deliberately left
    /// bit-for-bit as it was rather than made to re-decide the plan. A section it happens to make
    /// newly elidable simply stays unelided until the next reset, which costs correctness nothing.
    plan: InputChainPlan,
    /// The live trim ramp (#210 phase 3). `ramp.current[c]` is the same value as `coef.trim[c]`
    /// between events -- there is no second copy of the coefficient -- and `ramp.target[c]` is the
    /// per-lane target, whose sign **is** the lane's polarity and whose magnitude is its trim
    /// gain. Neither is stored a second time, for the reason [`InputStage::lane_track`] gives: the
    /// words are the only copy of the design.
    ramp: InputTrimRamp<L>,
    /// Per-lane frames left in the current trim ramp, `[channel][lane]`.
    ///
    /// The authoritative countdown, in the same relationship to the kernel's `f32` word that
    /// [`FaderRampStage::remaining`] has to [`GainMuteRamp::remaining`]: the kernel's word is
    /// recomputed from this at the top of every ramping block and never carried across one.
    remaining: [[u32; MAX_BANK_LANES]; 2],
    /// Whether any lane of either channel is mid-ramp.
    ///
    /// **This is the feature's off gate.** A session that has never had a trim or polarity command
    /// admitted for this bank leaves it `false` for the life of the plan, and
    /// [`InputStage::process`] then dispatches the untouched [`input_chain_block_elided`] over the
    /// prepared coefficients -- the same call, on the same words, in the same order, as before the
    /// feature existed. The cost of the feature to such a session is this one `bool` test per bank
    /// per block, and the `false` arm is byte-identical work.
    ramping: bool,
    /// One flag per lane: [`InputStage::compute_lane_channel_symmetry`]'s verdict, held rather
    /// than re-derived.
    ///
    /// # Why this is cached, and why the cache cannot go stale
    ///
    /// The comparison is thirty words per lane and every one of them is a `lane_read` -- a whole
    /// SIMD register spilled to the stack so one lane can be indexed out of it. The collapse
    /// dispatch pulls the witness once per lane per slot per block
    /// (`rack::BankChain::run`), so deriving it there costs more per block than the
    /// collapse it gates can save. Every effect bank already holds this same comparison from bind
    /// (`rack::EffectBankStage::designed`); this is the input bank's form of it, and
    /// the reason it needs maintenance where theirs does not is that #210 phase 3 made the trim
    /// and polarity words **live**.
    ///
    /// The words this compares move in exactly five places, and every one of them refreshes:
    ///
    /// * [`InputStage::new`], which seeds it;
    /// * [`InputStage::set_trim_signed`] -- the only writer a drained `TrimDb` or
    ///   `PolarityInvert` record reaches, whichever channel selector it carries;
    /// * [`InputStage::settle`], which republishes `coef.trim` and decrements the countdowns on
    ///   every ramping block;
    /// * [`InputStage::mirror_trim_ramp`], which duplicates the whole record onto channel `1` at
    ///   the bottom of every ramping *collapsed* block;
    /// * [`InputStage::reset`].
    ///
    /// The ramp kernel itself (`input_chain_ramp_block` and its mono twin) also advances
    /// `ramp.current` in place, and it is covered by the same refresh: `settle` republishes
    /// `coef.trim` from those words immediately after the kernel returns, and the refresh follows
    /// `settle`.
    ///
    /// The last three are reachable only through [`InputStage::process`] and
    /// [`InputStage::process_mono`], which refresh once at the bottom of their ramping arm rather
    /// than once per writer. Nothing else in this type writes a compared word:
    /// [`InputStage::desymmetrize`] copies integrators, [`InputStage::set_lane_state_words`] and
    /// the render path's boundary recovery write state, and `load_countdown` writes the kernel's
    /// `f32` countdown residue, which is not among the compared words -- the authoritative `u32`
    /// in [`InputStage::remaining`] is.
    ///
    /// So the cache is exact at **every** point a reader can observe it, which is what
    /// [`InputStage::lane_channel_symmetry`]'s debug assertion states: a stale flag is not a
    /// window that has to be argued closed, it is a failure every debug-built test in the tree
    /// re-proves absent on every block it renders.
    ///
    /// # Why a bitmask and not `[bool; MAX_BANK_LANES]`
    ///
    /// Because this type's `size_of` is sealed accounting -- it is a term in the builtin-compiler
    /// mutation-matrix transcript's `engine_owned_processor_payload_bytes`, and a change that
    /// moves no rendered bit must not move a sealed byte count. One byte beside `ramping` lands
    /// in padding the struct already had; eight would not. `MAX_BANK_LANES` is eight, so bit
    /// `lane` is lane `lane` and the mask needs no widening rule.
    symmetry: u8,
    /// Lifetime boundary-check recoveries per channel.
    lifetime_recovered: [u64; 2],
}

impl<L: Lane> InputStage<L> {
    /// Builds the stage from one prepared track per populated lane.
    ///
    /// `tracks.len()` must be in `1..=L::WIDTH`; the remaining lanes become padding lanes.
    fn new(tracks: &[PreparedInputTrack]) -> Self {
        debug_assert!(!tracks.is_empty() && tracks.len() <= L::WIDTH);
        let members = tracks.len().min(L::WIDTH).max(1);
        let mut trim = [[1.0_f32; MAX_BANK_LANES]; 2];
        let mut sections = [[SvfSection::IDENTITY; MAX_BANK_LANES]; 4];
        for (lane, track) in tracks.iter().enumerate().take(L::WIDTH) {
            for (channel, input) in [track.left, track.right].into_iter().enumerate() {
                trim[channel][lane] = input.trim_signed;
                sections[channel * 2][lane] = input.hpf;
                sections[channel * 2 + 1][lane] = input.lpf;
            }
        }
        let coef = InputChainCoef {
            trim: [lane_words::<L>(&trim[0]), lane_words::<L>(&trim[1])],
            section: [
                [svf_coef::<L>(&sections[0]), svf_coef::<L>(&sections[1])],
                [svf_coef::<L>(&sections[2]), svf_coef::<L>(&sections[3])],
            ],
        };
        let state = InputChainState::default();
        // `InputChainState::default()` is `+0.0` in every word, so a bank whose designs are all
        // disabled is decided elidable here and stays so: nothing after preparation writes an
        // elided section's coefficients or its state.
        let plan = input_chain_plan::<L>(&coef, &state);
        // The settled ramp: `current` and `target` are the prepared `trim_signed` words, the step
        // is zero and nothing is counting down. This is the initialisation the class-A OFF claim
        // rests on -- a lane that is never retargeted renders through `coef.trim`, which is these
        // words, which are `InputLane::trim_signed` unchanged.
        let ramp = InputTrimRamp {
            current: coef.trim,
            target: coef.trim,
            step: [L::zero(); 2],
            remaining: [L::zero(); 2],
        };
        let mut stage = Self {
            members,
            active: lanes_below::<L>(members),
            coef,
            state,
            plan,
            ramp,
            remaining: [[0; MAX_BANK_LANES]; 2],
            ramping: false,
            symmetry: 0,
            lifetime_recovered: [0; 2],
        };
        stage.refresh_channel_symmetry();
        stage
    }

    /// Retakes every lane's channel-symmetry comparison into [`InputStage::symmetry`].
    ///
    /// Called from each of the five writers of a compared word, never from the dispatch: this is
    /// the walk the cache exists to keep off the per-block path, so it runs on a retarget, on a
    /// ramping block, and on a reset, and on no other block at all.
    fn refresh_channel_symmetry(&mut self) {
        let mut symmetry = 0_u8;
        for lane in 0..MAX_BANK_LANES {
            if self.compute_lane_channel_symmetry(lane) {
                symmetry |= 1 << lane;
            }
        }
        self.symmetry = symmetry;
    }

    /// Largest ramp countdown that is exact in `f32`.
    ///
    /// The same clamp, for the same reason, as [`FADER_RAMP_COUNTDOWN_MAXIMUM`] and
    /// [`MATRIX_RAMP_COUNTDOWN_MAXIMUM`].
    const RAMP_COUNTDOWN_MAXIMUM: u32 = 1 << 24;

    /// Retargets one lane's trim on the addressed channels, over an explicit ramp window.
    ///
    /// `signed` is the whole coefficient: its magnitude is the trim gain and its sign is the
    /// polarity. A polarity flip is therefore this call with the same magnitude and the opposite
    /// sign, and the linear ramp carries the coefficient **through zero** -- which is the whole
    /// declick story for a live polarity invert, and why it needs no DSP of its own.
    ///
    /// D11: one division per channel per event, never per sample. A window of `0` is an immediate
    /// assignment, exactly as it is for the fader and the matrix.
    fn set_trim_signed(
        &mut self,
        lane: usize,
        channels: BuiltinLaneSelector,
        signed: impl Fn(f32) -> f32,
        smoothing_samples: u32,
    ) {
        debug_assert!(lane < L::WIDTH);
        for channel in 0..2 {
            if !channels.covers(channel) {
                continue;
            }
            let mut current = lane_read::<L>(self.ramp.current[channel]);
            let mut target = lane_read::<L>(self.ramp.target[channel]);
            let mut step = lane_read::<L>(self.ramp.step[channel]);
            let value = signed(target[lane]);
            target[lane] = value;
            step[lane] = if smoothing_samples == 0 {
                0.0
            } else {
                (value - current[lane]) / smoothing_samples as f32
            };
            if smoothing_samples == 0 {
                current[lane] = value;
            }
            self.ramp.target[channel] = lane_words::<L>(&target);
            self.ramp.step[channel] = lane_words::<L>(&step);
            self.ramp.current[channel] = lane_words::<L>(&current);
            self.coef.trim[channel] = self.ramp.current[channel];
            self.remaining[channel][lane] = smoothing_samples;
            if smoothing_samples > 0 {
                self.ramping = true;
            }
        }
        self.refresh_channel_symmetry();
    }

    /// Retargets one lane's trim in decibels, keeping its polarity.
    fn set_trim_db(
        &mut self,
        lane: usize,
        channels: BuiltinLaneSelector,
        gain: f32,
        smoothing_samples: u32,
    ) {
        self.set_trim_signed(
            lane,
            channels,
            |previous| {
                if previous.is_sign_negative() {
                    -gain
                } else {
                    gain
                }
            },
            smoothing_samples,
        );
    }

    /// Sets or clears one lane's polarity inversion, keeping its trim magnitude.
    fn set_polarity_invert(
        &mut self,
        lane: usize,
        channels: BuiltinLaneSelector,
        inverted: bool,
        smoothing_samples: u32,
    ) {
        self.set_trim_signed(
            lane,
            channels,
            move |previous| {
                let magnitude = previous.abs();
                if inverted { -magnitude } else { magnitude }
            },
            smoothing_samples,
        );
    }

    /// One lane and channel's current trim coefficient, for tests and control-plane readback.
    fn trim_signed(&self, lane: usize, channel: usize) -> f32 {
        lane_read::<L>(self.coef.trim[channel % 2])[lane]
    }

    /// One lane and channel's trim target, for tests and control-plane readback.
    fn trim_target(&self, lane: usize, channel: usize) -> f32 {
        lane_read::<L>(self.ramp.target[channel % 2])[lane]
    }

    /// Loads the kernel's `f32` countdown words from the authoritative `u32` counters.
    fn load_countdown(&mut self) {
        for channel in 0..2 {
            let mut words = [0.0_f32; MAX_BANK_LANES];
            for (lane, word) in words.iter_mut().enumerate() {
                *word = self.remaining[channel][lane].min(Self::RAMP_COUNTDOWN_MAXIMUM) as f32;
            }
            self.ramp.remaining[channel] = lane_words::<L>(&words);
        }
    }

    /// Advances the authoritative countdowns by `frames`, snaps the lanes that settled, and
    /// republishes `coef.trim` from the ramp's current words.
    ///
    /// The snap is an **assignment** to the exact target (D11), not the last accumulated sum: a
    /// lane whose countdown reached zero inside the block holds a value the kernel already
    /// assigned, and this restates it so that a lane whose countdown reached zero exactly at the
    /// block edge is assigned too.
    fn settle(&mut self, frames: usize, channels: core::ops::Range<usize>) {
        let frames = u32::try_from(frames).unwrap_or(u32::MAX);
        let mut ramping = false;
        for channel in channels {
            let mut current = lane_read::<L>(self.ramp.current[channel]);
            let target = lane_read::<L>(self.ramp.target[channel]);
            for lane in 0..L::WIDTH {
                let remaining = self.remaining[channel][lane].saturating_sub(frames);
                self.remaining[channel][lane] = remaining;
                if remaining == 0 {
                    // A **restatement**, not a correction: the kernel's step 3 already assigned
                    // this exact word on the frame the countdown reached zero, and the assertion
                    // below is what keeps the redundancy honest. It is kept because it is the
                    // line a future path that settles a lane *outside* the kernel -- a snap, a
                    // reset, a restore -- would otherwise have to remember to add, and because
                    // the authoritative countdown is this `u32` rather than the kernel's
                    // clamped `f32` word.
                    debug_assert_eq!(
                        current[lane].to_bits(),
                        target[lane].to_bits(),
                        "the ramp kernel assigns the exact target on the frame it settles"
                    );
                    current[lane] = target[lane];
                } else {
                    ramping = true;
                }
            }
            self.ramp.current[channel] = lane_words::<L>(&current);
            self.coef.trim[channel] = self.ramp.current[channel];
        }
        self.ramping = ramping;
    }

    /// Duplicates the left channel's whole trim-ramp record onto the right channel.
    ///
    /// The collapsed block's accounting rule, applied to the ramp: a collapsed track's right
    /// channel *is* its left channel, so the right channel's ramp advances exactly as the left
    /// one's did rather than freezing. See [`InputStage::process_mono`] for why the report is
    /// duplicated for the same reason.
    ///
    /// **This is the ramp's only restore path, and it runs per block rather than at the disengage
    /// boundary.** [`InputStage::desymmetrize`] carries the argument for why the boundary is the
    /// wrong place for it: the drain of the disengaging block sits between the two, and a copy
    /// there would clobber exactly the record that drain just wrote.
    fn mirror_trim_ramp(&mut self) {
        self.ramp.current[1] = self.ramp.current[0];
        self.ramp.target[1] = self.ramp.target[0];
        self.ramp.step[1] = self.ramp.step[0];
        self.ramp.remaining[1] = self.ramp.remaining[0];
        self.remaining[1] = self.remaining[0];
        self.coef.trim[1] = self.coef.trim[0];
    }

    /// Sums the populated lanes of an exact-integer lane word.
    fn members_sum(&self, value: L) -> u64 {
        let words = lane_read::<L>(value);
        words
            .iter()
            .take(self.members)
            .map(|word| *word as u64)
            .sum()
    }

    // REALTIME_POLICY_BEGIN
    /// Renders one block of both channels.
    ///
    /// `left` and `right` are AoSoA blocks of `frames * L::WIDTH` samples; at `L = f32` a planar
    /// slice is already such a block. One kernel call does the whole chain — sanitise, trim, both
    /// sections, both channels, and the boundary scan — in one frame loop, so the four
    /// independent recurrences overlap instead of serialising.
    fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: usize,
    ) -> BuiltinProcessReport {
        // The feature's off gate, and the whole of its steady-state cost: one `bool`. The `false`
        // arm is the call this function has always made, on the prepared coefficient words, with
        // the elision plan Job 1 decided -- byte-identical work.
        let report = if self.ramping {
            self.load_countdown();
            let report = input_chain_ramp_block::<L>(
                left,
                right,
                frames,
                &self.coef,
                &mut self.state,
                &mut self.ramp,
            );
            self.settle(frames, 0..2);
            self.refresh_channel_symmetry();
            report
        } else {
            input_chain_block_elided::<L>(
                left,
                right,
                frames,
                &self.coef,
                &mut self.state,
                &self.plan,
            )
        };
        let mut recovered = [0_u64; 2];
        for (channel, recovered) in recovered.iter_mut().enumerate() {
            let bad = L::mask_and(report.nonfinite[channel], self.active);
            if !L::mask_any(bad) {
                continue;
            }
            let io: &mut [f32] = if channel == 0 {
                &mut *left
            } else {
                &mut *right
            };
            zero_lanes_block::<L>(io, frames, bad);
            for section in &mut self.state.section[channel] {
                section.ic1 = section.ic1.andnot(bad);
                section.ic2 = section.ic2.andnot(bad);
            }
            *recovered = self.members_sum(L::select(bad, L::splat(1.0), L::zero()));
            self.lifetime_recovered[channel] =
                self.lifetime_recovered[channel].saturating_add(*recovered);
        }
        BuiltinProcessReport {
            sanitized_input: self
                .members_sum(report.sanitized[0])
                .saturating_add(self.members_sum(report.sanitized[1])),
            sanitized_output: 0,
            recovered_left_state: recovered[0],
            recovered_right_state: recovered[1],
        }
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN
    /// Renders one block of the **collapsed** track: one plane, one channel's coefficients and
    /// state, and the right channel's accounting duplicated from the left.
    ///
    /// The seam duplicates this plane into the right one before the fader reads it, so the right
    /// plane this call does not touch is not the block's right output -- it is scratch. The right
    /// channel's *state* is not touched either, and is restored by [`InputStage::desymmetrize`]
    /// before the first dual block after a disengage.
    ///
    /// The recovery arm is the dual body's channel-`0` arm with the channel index frozen: the same
    /// mask, the same `zero_lanes_block`, the same `andnot` over the same two sections. Its
    /// per-channel counters are duplicated rather than left at zero, because the right plane the
    /// seam is about to write carries exactly the left plane's recovered samples.
    fn process_mono(&mut self, left: &mut [f32], frames: usize) -> BuiltinProcessReport {
        debug_assert!(plan_is_channel_symmetric(&self.plan));
        // The collapse gate's own premise, restated where the one-plane body relies on it. A
        // collapsed block is dispatched only under `all_lanes_symmetric`, which compares the whole
        // trim-ramp record (`lane_channel_symmetry`), so the record this body is about to advance
        // for channel `0` and mirror onto channel `1` is a record the two channels already share.
        // It is asserted here rather than at the disengage boundary because *here* is where it
        // holds: by the time `desymmetrize` runs, this block's drain may legitimately have moved
        // one channel's words and not the other's.
        debug_assert!(self.trim_ramp_channels_agree());
        let report = if self.ramping {
            self.load_countdown();
            let report = input_chain_ramp_block_mono::<L>(
                left,
                frames,
                &self.coef,
                &mut self.state,
                &mut self.ramp,
            );
            // Channel `0` only: the right channel's ramp is not advanced by the one-plane kernel,
            // so it is settled from the left channel's countdown and then duplicated, exactly as
            // the report below is. A collapsed track's right channel *is* its left channel here
            // too, and freezing the right ramp instead would leave the disengage boundary with a
            // per-channel word to repair that no `desymmetrize` copy can reconstruct once the two
            // countdowns have diverged.
            self.settle(frames, 0..1);
            self.mirror_trim_ramp();
            self.refresh_channel_symmetry();
            report
        } else {
            input_chain_block_mono_elided::<L>(
                left,
                frames,
                &self.coef,
                &mut self.state,
                &self.plan,
            )
        };
        let mut recovered = 0_u64;
        let bad = L::mask_and(report.nonfinite[0], self.active);
        if L::mask_any(bad) {
            zero_lanes_block::<L>(left, frames, bad);
            for section in &mut self.state.section[0] {
                section.ic1 = section.ic1.andnot(bad);
                section.ic2 = section.ic2.andnot(bad);
            }
            recovered = self.members_sum(L::select(bad, L::splat(1.0), L::zero()));
            for channel in 0..2 {
                self.lifetime_recovered[channel] =
                    self.lifetime_recovered[channel].saturating_add(recovered);
            }
        }
        BuiltinProcessReport {
            sanitized_input: self
                .members_sum(report.sanitized[0])
                .saturating_add(self.members_sum(report.sanitized[1])),
            sanitized_output: 0,
            recovered_left_state: recovered,
            recovered_right_state: recovered,
        }
    }
    // REALTIME_POLICY_END

    /// Copies the left channel's **retained integrators** onto the right channel.
    ///
    /// The collapse's disengage boundary for this stage. The integrators are exactly what a
    /// collapsed run left behind: [`InputStage::process_mono`] advances channel `0`'s and freezes
    /// channel `1`'s, and by the induction the witness maintains the counterfactual dual run's
    /// right state *is* the left one, so copying them is not an approximation of that run -- it is
    /// that run's state. `plan`, `members`, `active` and the counters are cohort-wide, and
    /// `coef.section` is designed and compares bit-equal between the channels for every lane of a
    /// collapse-eligible bank (which is what `DESIGNED` *is*).
    ///
    /// # Why the trim ramp is deliberately **not** copied here
    ///
    /// It was, until the disengage-under-drain window was probed. The reasoning that put it here
    /// -- "the whole per-channel state is restored at the disengage boundary" -- reads well and is
    /// wrong for this one word set, because the trim ramp has a *second* maintainer that the
    /// integrators do not: `process_mono` mirrors the whole record onto the right channel at the
    /// bottom of every collapsed block. So the two channels' records are already equal at the
    /// **start** of every block, and the only thing that can make them differ before this is
    /// reached is the one event that must survive:
    ///
    /// 1. block `N-1` renders collapsed; `process_mono` mirrors the record;
    /// 2. block `N`'s `begin_block` drains a `Left`-only trim or polarity record and applies it,
    ///    which is what makes the two channels differ -- and is why the witness declines;
    /// 3. `BankChain::run` reads the declining witness and calls `disengage_collapse`, which calls
    ///    this.
    ///
    /// A copy at step 3 clones the *post-drain* left record onto the right channel, so a retarget
    /// the console addressed to one lane ramps both -- and, because `LIVE` is a latch, the chain
    /// never collapses again and the right channel never recovers. The integrators have no such
    /// window: nothing in the drain writes them.
    ///
    /// The rule this leaves is narrower than the one it replaces and is the true one: **a stage
    /// restores at the disengage boundary exactly the per-channel state its one-plane body
    /// froze.** `process_mono` froze the integrators. It did not freeze the ramp; it mirrored it.
    fn desymmetrize(&mut self) {
        self.state.section[1] = self.state.section[0];
    }

    /// The eight live trim-ramp words of one lane. Evidence only.
    ///
    /// # What words `6..8` are, and what a test may conclude from them
    ///
    /// The first six -- `current`, `target` and `step` per channel -- are the retained ramp and
    /// mean the same thing at every block boundary. The last two are the **kernel's** countdown
    /// words, and they are residue: `InputStage::load_countdown` overwrites them from the
    /// authoritative `[[u32; 8]; 2]` at the top of every ramping block, so what they hold between
    /// blocks is whatever the last kernel run counted down to, which is `-frames` for a settled
    /// lane and depends on how the caller partitioned its blocks.
    ///
    /// Two consequences, both relied on in the suites: comparing them across two arms is sound
    /// **only when the arms rendered the same block sizes** (which is what
    /// `a_symmetric_ride_through_a_collapse_renders_never_collapsed_bits` and its siblings do), and
    /// asserting they are exactly `+0.0` is exactly the assertion that no ramping block ran, which
    /// is `the_settled_arm_leaves_the_ramp_words_untouched`'s whole content. Neither is a claim
    /// about the ramp's *value*; for that, read `current` and `target`.
    fn trim_ramp_words(&self, lane: usize) -> [u32; 8] {
        let read = |value: L| lane_read::<L>(value)[lane].to_bits();
        [
            read(self.ramp.current[0]),
            read(self.ramp.current[1]),
            read(self.ramp.target[0]),
            read(self.ramp.target[1]),
            read(self.ramp.step[0]),
            read(self.ramp.step[1]),
            read(self.ramp.remaining[0]),
            read(self.ramp.remaining[1]),
        ]
    }

    /// Whether every per-channel trim-ramp word compares bit-equal between the two channels.
    fn trim_ramp_channels_agree(&self) -> bool {
        for words in [
            (self.ramp.current[0], self.ramp.current[1]),
            (self.ramp.target[0], self.ramp.target[1]),
            (self.ramp.step[0], self.ramp.step[1]),
            (self.coef.trim[0], self.coef.trim[1]),
        ] {
            let left = lane_read::<L>(words.0);
            let right = lane_read::<L>(words.1);
            for lane in 0..L::WIDTH {
                if left[lane].to_bits() != right[lane].to_bits() {
                    return false;
                }
            }
        }
        self.remaining[0][..L::WIDTH] == self.remaining[1][..L::WIDTH]
    }

    /// Whether this stage can **prove**, right now, that its two channels' state is bit-equal.
    ///
    /// The mono collapse's way back (M3). The proof is a walk over exactly the words
    /// [`InputStage::desymmetrize`] copies -- the four integrators per channel and the trim ramp
    /// record -- because those are the whole of this kernel's per-channel state, and a `true` that
    /// covered less would re-engage a collapse onto a right channel that is not the left one.
    ///
    /// It is asked only inside a recovery window (`rack::BankChain::run`), so a
    /// session that never drives its channels apart never pays for it.
    fn channels_agree(&self) -> bool {
        for section in 0..2 {
            let left = &self.state.section[0][section];
            let right = &self.state.section[1][section];
            for (left_word, right_word) in [(left.ic1, right.ic1), (left.ic2, right.ic2)] {
                let left_words = lane_read::<L>(left_word);
                let right_words = lane_read::<L>(right_word);
                for lane in 0..L::WIDTH {
                    if left_words[lane].to_bits() != right_words[lane].to_bits() {
                        return false;
                    }
                }
            }
        }
        self.trim_ramp_channels_agree()
    }

    /// Whether this chain may be collapsed at all: the two channels must elide the same sections.
    ///
    /// See `lane::kernels::builtins::plan_is_channel_symmetric` for why an elision
    /// disagreement is a `-0.0` divergence rather than a scheduling difference.
    const fn mono_collapse_gate(&self) -> bool {
        plan_is_channel_symmetric(&self.plan)
    }

    /// Clears every retained integrator word; prepared coefficients are untouched.
    ///
    /// The elision plan is re-decided, because it is a function of the state words as well as the
    /// coefficients. A reset can only ever make a section *more* elidable -- it writes `+0.0`
    /// everywhere -- so leaving the plan alone would be sound; it is recomputed anyway, because
    /// the cheap rule is the one worth keeping: every write to `state` outside the render path
    /// re-decides `plan`.
    fn reset(&mut self) {
        self.state = InputChainState::default();
        // Snap every lane to its target and cancel any ramp in flight, exactly as
        // `FaderRampStage::reset` and `MatrixStage::reset` do: a reset is a state reset, and a
        // half-finished ramp is state. The *target* is kept, because `trim_db` and
        // `polarity_invert` declare `BuiltinParameterReset::RestorePreparedValue` for the
        // **prepared** value and the live target is what the console last asked for -- the same
        // reading `fader_db` has had since #140 B.
        self.ramp.current = self.ramp.target;
        self.ramp.step = [L::zero(); 2];
        self.ramp.remaining = [L::zero(); 2];
        self.remaining = [[0; MAX_BANK_LANES]; 2];
        self.ramping = false;
        self.coef.trim = self.ramp.current;
        self.plan = input_chain_plan::<L>(&self.coef, &self.state);
        self.refresh_channel_symmetry();
    }

    /// Recovers the prepared record of one lane from the coefficient words.
    ///
    /// The words are the only copy of the design: a section is disabled exactly when its output
    /// mix is the identity `(1, 0, 0)`, it is a high-pass exactly when its mix is `(1, -k, -1)`,
    /// and `k` is Butterworth throughout. Keeping a second, structured copy next to the lane words
    /// would be the defect this crate exists to remove -- and it is what a bank is built from, so
    /// the two would have to agree forever.
    fn lane_track(&self, lane: usize) -> PreparedInputTrack {
        let trim = self.coef.trim.map(|trim| lane_read::<L>(trim)[lane]);
        let section = |channel: usize, index: usize| -> SvfSection {
            let coef = &self.coef.section[channel][index];
            let (m0, m1, m2) = (
                lane_read::<L>(coef.m0)[lane],
                lane_read::<L>(coef.m1)[lane],
                lane_read::<L>(coef.m2)[lane],
            );
            let enabled = !(m0 == 1.0 && m1 == 0.0 && m2 == 0.0);
            SvfSection {
                c1: lane_read::<L>(coef.c1)[lane],
                a2: lane_read::<L>(coef.a2)[lane],
                a3: lane_read::<L>(coef.a3)[lane],
                k: if enabled { BUTTERWORTH_K } else { 0.0 },
                m0,
                m1,
                m2,
                enabled,
            }
        };
        PreparedInputTrack {
            left: InputLane {
                trim_signed: trim[0],
                hpf: section(0, 0),
                lpf: section(0, 1),
            },
            right: InputLane {
                trim_signed: trim[1],
                hpf: section(1, 0),
                lpf: section(1, 1),
            },
        }
    }

    /// Whether every designed word the input chain's kernel reads for `lane` compares
    /// **bit-equal** between the two channels, as
    /// [`compute_lane_channel_symmetry`](Self::compute_lane_channel_symmetry) defines it.
    ///
    /// The held answer. See [`InputStage::symmetry`] for the five writers that maintain it.
    ///
    /// The debug assertion is the whole soundness argument, executable: it re-derives the walk and
    /// compares, so every debug-built test in the tree -- every drain, every ramping block, every
    /// collapse engage, disengage and re-engage, in whatever order a test interleaves them --
    /// checks the cache against its own definition on each block it renders. A stale flag is
    /// therefore a test failure and not a silent wrong collapse.
    fn lane_channel_symmetry(&self, lane: usize) -> bool {
        let cached = lane < MAX_BANK_LANES && self.symmetry & (1 << lane) != 0;
        debug_assert_eq!(
            cached,
            self.compute_lane_channel_symmetry(lane),
            "the cached channel-symmetry flag is stale for lane {lane}"
        );
        cached
    }

    /// Whether every designed word the input chain's kernel reads for `lane` compares
    /// **bit-equal** between the two channels.
    ///
    /// # The word list, and why it is exactly this
    ///
    /// `input_chain_block` (and both of its elided variants) reads one thing per channel:
    /// `InputChainCoef`. That is
    ///
    /// * `trim[channel]` -- one word, with the polarity inversion already folded in
    ///   (`InputLane::trim_signed`), so its sign bit *is* the polarity flag and comparing the word
    ///   compares `trim_db` and `polarity_invert` together. Since #210 phase 3 it is the **live**
    ///   word, not the prepared one: `coef.trim` is republished from the ramp's `current` after
    ///   every retarget and after every ramping block, so this comparison is on what the kernel
    ///   will load for the block being dispatched, which is the only reading that can gate a
    ///   collapse;
    /// * the rest of the trim ramp record -- `target`, `step` and the countdown -- three more
    ///   words per channel, read by `input_chain_ramp_block` exactly as `trim` is. They are here
    ///   for a reason a settled bank cannot show: at the block an **asymmetric** retarget is
    ///   admitted, `current` has not moved yet, so a witness that compared only `trim` would still
    ///   call the two channels equal and let that block collapse -- publishing the left channel's
    ///   new ramp on the right one. The `LIVE` term the drain clears is the primary guard against
    ///   that (`BuiltinBankProcessor`); this is the same fact restated in the words themselves, so
    ///   the two have to agree for a wrong collapse to happen;
    /// * `section[channel][s].{c1, a2, a3, m0, m1, m2}` -- six words per section, two sections
    ///   (`0` high-pass, `1` low-pass).
    ///
    /// Twenty-six words per lane. `k` (`1/Q`) is not among them: it lives only in the
    /// control-plane `SvfSection` and is folded into `m1` by `svf_coef`, so the kernel never sees
    /// it and neither does this.
    ///
    /// Deliberately excluded, each for its own reason:
    ///
    /// * `InputChainState` (`ic1`, `ic2`) -- running integrator state, not a designed word.
    /// * `InputChainPlan` -- the Job-1 elision decision. It is a pure boolean function of the very
    ///   words above and of the state, so it carries no information this comparison does not; and
    ///   it is decided over **every** lane of the bank (`every_lane_is`), so reading it would make
    ///   one lane's witness depend on its neighbours' parameters, which is exactly the cross-lane
    ///   coupling the witness must not have.
    /// * `delay_samples` (#210 phase 2) -- the track's input-side time alignment. It is a real
    ///   per-lane designed word and an asymmetric one really does decline the track's collapse,
    ///   but it is not a word *this kernel* reads: the delay is a graph node at
    ///   `TrackStage::Input`, upstream of this bank, and `input_chain_block` never sees it. Its
    ///   verdict is taken at prepare by `track_input_delay_symmetric`, which conjoins it into
    ///   the same `DESIGNED` term this function answers for; listing it here would be claiming a
    ///   load that does not happen.
    /// * `members`, `active`, `lifetime_recovered` -- cohort shape and counters, not track
    ///   parameters. A lane's witness must not change because the cohort grew.
    ///
    /// Taken only by [`InputStage::refresh_channel_symmetry`] and by the reader's assertion:
    /// it is the definition, not the per-block path.
    fn compute_lane_channel_symmetry(&self, lane: usize) -> bool {
        if lane >= self.members || lane >= L::WIDTH {
            return false;
        }
        for (left_word, right_word) in [
            (self.coef.trim[0], self.coef.trim[1]),
            (self.ramp.target[0], self.ramp.target[1]),
            (self.ramp.step[0], self.ramp.step[1]),
        ] {
            if lane_read::<L>(left_word)[lane].to_bits()
                != lane_read::<L>(right_word)[lane].to_bits()
            {
                return false;
            }
        }
        if self.remaining[0][lane] != self.remaining[1][lane] {
            return false;
        }
        for section in 0..2 {
            let left = &self.coef.section[0][section];
            let right = &self.coef.section[1][section];
            for (left_word, right_word) in [
                (left.c1, right.c1),
                (left.a2, right.a2),
                (left.a3, right.a3),
                (left.m0, right.m0),
                (left.m1, right.m1),
                (left.m2, right.m2),
            ] {
                if lane_read::<L>(left_word)[lane].to_bits()
                    != lane_read::<L>(right_word)[lane].to_bits()
                {
                    return false;
                }
            }
        }
        true
    }

    /// Which sections this stage elides, `[channel][section]`. Evidence only.
    fn elision_plan(&self) -> [[bool; 2]; 2] {
        self.plan.elided
    }

    /// The eight retained words of one lane: `[l_hpf_ic1, l_hpf_ic2, l_lpf_ic1, l_lpf_ic2, r..]`.
    fn lane_state_words(&self, lane: usize) -> [u32; 8] {
        let mut words = [0_u32; 8];
        for channel in 0..2 {
            for section in 0..2 {
                let state = &self.state.section[channel][section];
                words[channel * 4 + section * 2] = lane_read::<L>(state.ic1)[lane].to_bits();
                words[channel * 4 + section * 2 + 1] = lane_read::<L>(state.ic2)[lane].to_bits();
            }
        }
        words
    }

    /// Overwrites the eight retained words of one lane. Evidence and fault-injection only.
    ///
    /// This is the one post-preparation write to the retained state, so it is the one place the
    /// elision plan could go stale, and it re-decides it. The case that makes the hook
    /// load-bearing rather than defensive: identity coefficients over an injected `-0.0`
    /// integrator emit `-0.0` where the elided form emits `+0.0`, so a plan left standing here
    /// would move bits.
    fn set_lane_state_words(&mut self, lane: usize, words: [u32; 8]) {
        for channel in 0..2 {
            for section in 0..2 {
                let state = &mut self.state.section[channel][section];
                let mut ic1 = lane_read::<L>(state.ic1);
                let mut ic2 = lane_read::<L>(state.ic2);
                ic1[lane] = f32::from_bits(words[channel * 4 + section * 2]);
                ic2[lane] = f32::from_bits(words[channel * 4 + section * 2 + 1]);
                state.ic1 = lane_words::<L>(&ic1);
                state.ic2 = lane_words::<L>(&ic2);
            }
        }
        self.plan = input_chain_plan::<L>(&self.coef, &self.state);
    }
}

/// The fader and mute stage at one width: one multiply and one mask clear per sample.
pub(crate) struct FaderStage<L: Lane> {
    /// Prepared gain per channel, `[left, right]`.
    gain: [L; 2],
    /// Muted lanes per channel; a muted lane is exactly `+0.0`.
    mute: [L::Mask; 2],
}

impl<L: Lane> FaderStage<L> {
    /// Builds the stage from one prepared fader per populated lane and channel.
    fn new(lanes: &[(FaderLane, FaderLane)]) -> Self {
        let mut gain = [[1.0_f32; MAX_BANK_LANES]; 2];
        let mut mute = [[0.0_f32; MAX_BANK_LANES]; 2];
        for (lane, pair) in lanes.iter().enumerate().take(L::WIDTH) {
            for (channel, fader) in [pair.0, pair.1].into_iter().enumerate() {
                gain[channel][lane] = fader.gain;
                mute[channel][lane] = f32::from(u8::from(fader.muted));
            }
        }
        Self {
            gain: [lane_words::<L>(&gain[0]), lane_words::<L>(&gain[1])],
            mute: [
                mask_from_flags::<L>(&mute[0][..L::WIDTH]),
                mask_from_flags::<L>(&mute[1][..L::WIDTH]),
            ],
        }
    }

    /// Renders one block of both channels. Feed-forward, so it carries no counters and no checks.
    fn process(&mut self, left: &mut [f32], right: &mut [f32], frames: usize) {
        gain_mute_block::<L>(left, frames, self.gain[0], self.mute[0]);
        gain_mute_block::<L>(right, frames, self.gain[1], self.mute[1]);
    }
}

/// The ramped fader and mute stage at one width (D11 ramps, issue #212's banked strip).
///
/// # One body, so lane identity is a property of the code
///
/// This is the *only* ramped-fader implementation in the workspace. A live-console track is this
/// type at `L = f32` over planar slices ([`FaderMuteRampBuiltins`]); a banked strip slot is the
/// same type at `Simd4` or `Simd8` over an AoSoA block ([`BuiltinFaderBank`]). The banked form is
/// therefore op-order-identical to the per-track form by construction rather than by two
/// implementations being compared -- the same rule [`InputStage`] follows, and the reason the
/// per-track scalar path was rewritten onto this type instead of being left beside it.
///
/// The lane arrays are `[channel][lane]`: `channel` is the dual-mono side (`0` left, `1` right)
/// and `lane` is the bank member. A scalar track has `L::WIDTH == 1`, so `lane` is always `0` and
/// the two channels are the two `[GainMuteRamp; 2]` entries -- exactly the `[f32; 2]` pairs the
/// per-track type carried before.
///
/// # Ramp independence, and why partition invariance follows
///
/// Every lane owns its countdown, its step and its current gain, and
/// [`gain_mute_ramp_block`] advances all three in place per frame. A lane's ramp therefore evolves
/// by its own additions, in its own order, regardless of the block size, of where the block
/// boundaries fall, or of which other tracks share its bank. That is what makes a banked lane's
/// bits equal the same track's bits rendered alone, and it is the same argument
/// [`matrix2x2_ramp_block`] carries.
///
/// # Which kernel a block runs
///
/// A channel with **no** lane ramping dispatches [`gain_mute_block`] over the whole block: one
/// multiply and one mask clear per frame, the identical operation the prepared-only [`FaderStage`]
/// has always run. A settled lane's arithmetic is therefore unchanged by banking, muted or not --
/// a settled mute is the exact `+0.0` the `andnot` produces, never a multiply's signed zero.
///
/// A channel with any lane ramping runs the ramp kernel over `max(remaining)` frames -- capped at
/// the block -- and then, if the block outlives every ramp, [`gain_mute_block`] over the tail. A
/// lane whose own ramp ended earlier inside that window keeps multiplying by its exactly-assigned
/// target, which is what the scalar tail multiply did for it before.
pub(crate) struct FaderRampStage<L: Lane> {
    /// Ramp words per channel. `ramp[c].current` is the settled gain between events, and
    /// `ramp[c].mute` is the per-lane mute mask, so there is no second copy of either.
    ramp: [GainMuteRamp<L>; 2],
    /// Each lane's fader gain, independent of mute, `[channel][lane]`.
    fader_gain: [[f32; MAX_BANK_LANES]; 2],
    /// Each lane's mute flag, `[channel][lane]`.
    muted: [[bool; MAX_BANK_LANES]; 2],
    /// Frames left in each lane's ramp, `[channel][lane]`.
    remaining: [[u32; MAX_BANK_LANES]; 2],
}

/// Largest ramp countdown that is exact in `f32`.
///
/// The same clamp, for the same reason, as [`MATRIX_RAMP_COUNTDOWN_MAXIMUM`]: a window may be up to
/// `u32::MAX` updates but the in-kernel countdown is an `f32` integer. It is invisible because a
/// lane can only reach zero inside a block when its remaining count is at most the block length,
/// and because the authoritative countdown is the `u32` in [`FaderRampStage::remaining`] -- the
/// kernel's leftover word is recomputed from it at the top of every ramping block, never carried.
const FADER_RAMP_COUNTDOWN_MAXIMUM: u32 = 1 << 24;

impl<L: Lane> FaderRampStage<L> {
    /// Builds a settled stage from one prepared fader per populated lane and channel.
    ///
    /// Lanes at or above `lanes.len()` are padding: unit gain, unmuted, never ramping. A muted
    /// lane starts at gain `0.0` -- its mute is a fader endpoint, not a separate state -- which is
    /// what lets [`Self::set_mute`] unmute by retargeting back to `fader_gain`.
    fn new(lanes: &[(FaderLane, FaderLane)]) -> Self {
        let mut settled = [[1.0_f32; MAX_BANK_LANES]; 2];
        let mut fader_gain = [[1.0_f32; MAX_BANK_LANES]; 2];
        let mut muted = [[false; MAX_BANK_LANES]; 2];
        let mut flags = [[0.0_f32; MAX_BANK_LANES]; 2];
        for (lane, pair) in lanes.iter().enumerate().take(L::WIDTH) {
            for (channel, fader) in [pair.0, pair.1].into_iter().enumerate() {
                fader_gain[channel][lane] = fader.gain;
                muted[channel][lane] = fader.muted;
                settled[channel][lane] = if fader.muted { 0.0 } else { fader.gain };
                flags[channel][lane] = f32::from(u8::from(fader.muted));
            }
        }
        let ramp = core::array::from_fn(|channel| {
            let current = lane_words::<L>(&settled[channel]);
            GainMuteRamp {
                current,
                target: current,
                step: L::zero(),
                remaining: L::zero(),
                mute: mask_from_flags::<L>(&flags[channel][..L::WIDTH]),
            }
        });
        Self {
            ramp,
            fader_gain,
            muted,
            remaining: [[0; MAX_BANK_LANES]; 2],
        }
    }

    /// Recomputes one channel's mute mask from the per-lane flags.
    fn sync_mute(&mut self, channel: usize) {
        let mut flags = [0.0_f32; MAX_BANK_LANES];
        for (lane, flag) in flags.iter_mut().enumerate() {
            *flag = f32::from(u8::from(self.muted[channel][lane]));
        }
        self.ramp[channel].mute = mask_from_flags::<L>(&flags[..L::WIDTH]);
    }

    /// Retargets one lane of one channel. D11: one division per event, never per sample.
    fn retarget(&mut self, lane: usize, channel: usize, target: f32, smoothing_samples: u32) {
        let current = lane_read::<L>(self.ramp[channel].current);
        let mut targets = lane_read::<L>(self.ramp[channel].target);
        let mut steps = lane_read::<L>(self.ramp[channel].step);
        targets[lane] = target;
        steps[lane] = if smoothing_samples == 0 {
            0.0
        } else {
            // D11: one division, at the moment the target changes.
            (target - current[lane]) / smoothing_samples as f32
        };
        self.ramp[channel].target = lane_words::<L>(&targets);
        self.ramp[channel].step = lane_words::<L>(&steps);
        self.remaining[channel][lane] = smoothing_samples;
        if smoothing_samples == 0 {
            let mut current = current;
            current[lane] = target;
            self.ramp[channel].current = lane_words::<L>(&current);
        }
    }

    /// Retargets one lane's fader gain on the channels `channels` covers.
    ///
    /// A muted lane keeps its `0.0` target: the new gain is remembered and takes effect when the
    /// lane is unmuted, exactly as a physical console's fader does.
    fn set_fader_gain(
        &mut self,
        lane: usize,
        channels: BuiltinLaneSelector,
        gain: f32,
        smoothing_samples: u32,
    ) {
        for channel in 0..2 {
            if !channels.covers(channel) {
                continue;
            }
            self.fader_gain[channel][lane] = gain;
            let target = if self.muted[channel][lane] { 0.0 } else { gain };
            self.retarget(lane, channel, target, smoothing_samples);
        }
    }

    /// Sets or clears one lane's mute on the channels `channels` covers, as a retarget.
    fn set_mute(
        &mut self,
        lane: usize,
        channels: BuiltinLaneSelector,
        muted: bool,
        smoothing_samples: u32,
    ) {
        for channel in 0..2 {
            if !channels.covers(channel) {
                continue;
            }
            self.muted[channel][lane] = muted;
            let target = if muted {
                0.0
            } else {
                self.fader_gain[channel][lane]
            };
            self.retarget(lane, channel, target, smoothing_samples);
            self.sync_mute(channel);
        }
    }

    /// The settled gain of one lane and channel, for tests and control-plane readback.
    fn target_gain(&self, lane: usize, channel: usize) -> f32 {
        lane_read::<L>(self.ramp[channel].target)[lane]
    }

    /// Whether one lane and channel is muted.
    const fn is_muted(&self, lane: usize, channel: usize) -> bool {
        self.muted[channel][lane]
    }

    // REALTIME_POLICY_BEGIN
    /// Renders one block of both channels.
    fn process(&mut self, left: &mut [f32], right: &mut [f32], frames: usize) {
        self.process_plane(0, left, frames);
        self.process_plane(1, right, frames);
    }

    fn process_plane(&mut self, channel: usize, plane: &mut [f32], frames: usize) {
        let maximum = self.remaining[channel]
            .iter()
            .take(L::WIDTH)
            .copied()
            .max()
            .unwrap_or(0);
        if maximum == 0 {
            gain_mute_block::<L>(
                plane,
                frames,
                self.ramp[channel].current,
                self.ramp[channel].mute,
            );
            return;
        }
        let ramp_frames = (maximum as usize).min(frames);
        let mut countdown = [0.0_f32; MAX_BANK_LANES];
        for (lane, word) in countdown.iter_mut().enumerate() {
            *word = self.remaining[channel][lane].min(FADER_RAMP_COUNTDOWN_MAXIMUM) as f32;
        }
        self.ramp[channel].remaining = lane_words::<L>(&countdown);
        let split = ramp_frames * L::WIDTH;
        gain_mute_ramp_block::<L>(&mut plane[..split], ramp_frames, &mut self.ramp[channel]);
        for remaining in self.remaining[channel].iter_mut().take(L::WIDTH) {
            *remaining = remaining.saturating_sub(ramp_frames as u32);
        }
        // The kernel already assigned the target exactly on the frame a lane settled on, so these
        // two writes are the bookkeeping the scalar path did after its loop and not a second
        // numeric event: `current` is re-assigned to the value it already holds, and the step is
        // dropped so a later block cannot walk past a finished ramp.
        let mut current = lane_read::<L>(self.ramp[channel].current);
        let target = lane_read::<L>(self.ramp[channel].target);
        let mut steps = lane_read::<L>(self.ramp[channel].step);
        for lane in 0..L::WIDTH {
            if self.remaining[channel][lane] == 0 {
                current[lane] = target[lane];
                steps[lane] = 0.0;
            }
        }
        self.ramp[channel].current = lane_words::<L>(&current);
        self.ramp[channel].step = lane_words::<L>(&steps);
        if ramp_frames < frames {
            // Reached only when every lane settled inside this block, because `ramp_frames` is the
            // largest remaining count: the settled kernel is therefore correct for all of them.
            gain_mute_block::<L>(
                &mut plane[split..],
                frames - ramp_frames,
                self.ramp[channel].current,
                self.ramp[channel].mute,
            );
        }
    }
    // REALTIME_POLICY_END

    /// Snaps every lane to its target and cancels any ramp in flight.
    fn reset(&mut self) {
        for channel in 0..2 {
            self.ramp[channel].current = self.ramp[channel].target;
            self.ramp[channel].step = L::zero();
            self.ramp[channel].remaining = L::zero();
        }
        self.remaining = [[0; MAX_BANK_LANES]; 2];
    }
}

/// The smoothed 2x2 channel matrix at one width (D11 ramps, master plan §4.2).
///
/// The lane words are authoritative for the coefficient values; the scalar arrays carry the
/// control-plane bookkeeping (target, window length, frames left) that decides which kernel a
/// block runs.
pub(crate) struct MatrixStage<L: Lane> {
    /// Settled coefficients and the per-lane identity mask.
    coef: Matrix2x2Coef<L>,
    /// Ramp words; `ramp.current` is the same value as [`MatrixStage::coef`] between events, and
    /// `ramp.target` is the per-lane target -- there is no scalar copy of it.
    ramp: Matrix2x2Ramp<L>,
    /// Per-lane smoothing window, in sample updates.
    smoothing_samples: [u32; MAX_BANK_LANES],
    /// Per-lane frames left in the current ramp.
    remaining: [u32; MAX_BANK_LANES],
}

/// Largest ramp countdown that is exact in `f32`.
///
/// A window may be up to `u32::MAX` updates. The in-kernel countdown is an `f32` integer, so it is
/// clamped here; the clamp is invisible, because a lane can only reach zero inside a block when
/// its remaining count is at most the block length.
const MATRIX_RAMP_COUNTDOWN_MAXIMUM: u32 = 1 << 24;

impl<L: Lane> MatrixStage<L> {
    /// Builds a settled stage from one prepared matrix and window per populated lane.
    fn new(lanes: &[(Matrix2x2, u32)]) -> Self {
        let mut target = [Matrix2x2::IDENTITY; MAX_BANK_LANES];
        let mut smoothing_samples = [0_u32; MAX_BANK_LANES];
        for (lane, (matrix, samples)) in lanes.iter().enumerate().take(L::WIDTH) {
            target[lane] = *matrix;
            smoothing_samples[lane] = *samples;
        }

        let mut stage = Self {
            coef: Matrix2x2Coef {
                ll: L::zero(),
                lr: L::zero(),
                rl: L::zero(),
                rr: L::zero(),
                identity: no_lanes::<L>(),
            },
            ramp: Matrix2x2Ramp {
                current: [L::zero(); 4],
                target: [L::zero(); 4],
                step: [L::zero(); 4],
                remaining: L::zero(),
            },
            smoothing_samples,
            remaining: [0; MAX_BANK_LANES],
        };
        stage.write_current(&target);
        stage.ramp.target = stage.ramp.current;
        stage.sync_settled();
        stage
    }

    /// Reads the per-lane targets back out of the ramp words.
    fn read_target(&self) -> [Matrix2x2; MAX_BANK_LANES] {
        let words = self.ramp.target.map(lane_read::<L>);
        let mut values = [Matrix2x2::IDENTITY; MAX_BANK_LANES];
        for (lane, matrix) in values.iter_mut().enumerate() {
            *matrix = Matrix2x2 {
                ll: words[0][lane],
                lr: words[1][lane],
                rl: words[2][lane],
                rr: words[3][lane],
            };
        }
        values
    }

    /// Writes one matrix per lane into the ramp's current words.
    fn write_current(&mut self, values: &[Matrix2x2; MAX_BANK_LANES]) {
        let mut words = [[0.0_f32; MAX_BANK_LANES]; 4];
        for (lane, matrix) in values.iter().enumerate() {
            words[0][lane] = matrix.ll;
            words[1][lane] = matrix.lr;
            words[2][lane] = matrix.rl;
            words[3][lane] = matrix.rr;
        }
        for (slot, word) in self.ramp.current.iter_mut().zip(words.iter()) {
            *slot = lane_words::<L>(word);
        }
    }

    /// Reads the ramp's current words back as one matrix per lane.
    fn read_current(&self) -> [Matrix2x2; MAX_BANK_LANES] {
        let words = self.ramp.current.map(lane_read::<L>);
        let mut values = [Matrix2x2::IDENTITY; MAX_BANK_LANES];
        for (lane, matrix) in values.iter_mut().enumerate() {
            *matrix = Matrix2x2 {
                ll: words[0][lane],
                lr: words[1][lane],
                rl: words[2][lane],
                rr: words[3][lane],
            };
        }
        values
    }

    /// Copies the current words into the settled coefficients and recomputes the identity mask.
    ///
    /// A lane is an identity lane only when it is settled: a ramping lane must keep running the
    /// ramp arithmetic even if it passes through the identity matrix on the way.
    fn sync_settled(&mut self) {
        self.coef.ll = self.ramp.current[0];
        self.coef.lr = self.ramp.current[1];
        self.coef.rl = self.ramp.current[2];
        self.coef.rr = self.ramp.current[3];
        let current = self.read_current();
        let mut flags = [0.0_f32; MAX_BANK_LANES];
        for (lane, flag) in flags.iter_mut().enumerate() {
            let settled = self.remaining[lane] == 0 && current[lane] == Matrix2x2::IDENTITY;
            *flag = f32::from(u8::from(settled));
        }
        self.coef.identity = mask_from_flags::<L>(&flags[..L::WIDTH]);
    }

    /// Retargets one lane. D11: one division per coefficient per event, never per sample.
    fn set_target(&mut self, lane: usize, target: Matrix2x2) -> Result<(), BuiltinParameterError> {
        let samples = if lane < L::WIDTH {
            self.smoothing_samples[lane]
        } else {
            0
        };
        self.set_target_over(lane, target, samples)
    }

    /// Retargets one lane over an explicit ramp window, and adopts that window as the lane's own.
    ///
    /// Issue #137 D1: a live console changes the pan window with the pan, so the retarget and the
    /// window are one event. `set_target` is exactly this call with the prepared window, so the
    /// two cannot drift.
    fn set_target_over(
        &mut self,
        lane: usize,
        target: Matrix2x2,
        samples: u32,
    ) -> Result<(), BuiltinParameterError> {
        let target = target.checked()?;
        if lane >= L::WIDTH {
            return Err(BuiltinParameterError::LaneLength);
        }
        self.smoothing_samples[lane] = samples;
        let current = self.read_current()[lane];
        let mut targets = self.ramp.target.map(lane_read::<L>);
        let mut steps = self.ramp.step.map(lane_read::<L>);
        let target_words = [target.ll, target.lr, target.rl, target.rr];
        let current_words = [current.ll, current.lr, current.rl, current.rr];
        for index in 0..4 {
            targets[index][lane] = target_words[index];
            steps[index][lane] = if samples == 0 {
                0.0
            } else {
                (target_words[index] - current_words[index]) / samples as f32
            };
        }
        for index in 0..4 {
            self.ramp.target[index] = lane_words::<L>(&targets[index]);
            self.ramp.step[index] = lane_words::<L>(&steps[index]);
        }
        self.remaining[lane] = samples;
        if samples == 0 {
            let mut current = self.read_current();
            current[lane] = target;
            self.write_current(&current);
        }
        self.sync_settled();
        Ok(())
    }

    // REALTIME_POLICY_BEGIN
    #[inline(always)]
    fn is_settled(&self) -> bool {
        self.remaining
            .iter()
            .take(L::WIDTH)
            .all(|&remaining| remaining == 0)
    }

    /// Renders one block of both channels.
    fn process(&mut self, left: &mut [f32], right: &mut [f32], frames: usize) {
        let maximum = self
            .remaining
            .iter()
            .take(L::WIDTH)
            .copied()
            .max()
            .unwrap_or(0);
        if maximum == 0 {
            matrix2x2_block::<L>(left, right, frames, &self.coef);
            return;
        }
        let ramp_frames = (maximum as usize).min(frames);
        let mut countdown = [0.0_f32; MAX_BANK_LANES];
        for (lane, word) in countdown.iter_mut().enumerate() {
            *word = self.remaining[lane].min(MATRIX_RAMP_COUNTDOWN_MAXIMUM) as f32;
        }
        self.ramp.remaining = lane_words::<L>(&countdown);
        let split = ramp_frames * L::WIDTH;
        matrix2x2_ramp_block::<L>(
            &mut left[..split],
            &mut right[..split],
            ramp_frames,
            &mut self.ramp,
        );
        for remaining in self.remaining.iter_mut().take(L::WIDTH) {
            *remaining = remaining.saturating_sub(ramp_frames as u32);
        }
        let mut current = self.read_current();
        let target = self.read_target();
        for (lane, current) in current.iter_mut().enumerate().take(L::WIDTH) {
            if self.remaining[lane] == 0 {
                *current = target[lane];
            }
        }
        self.write_current(&current);
        self.sync_settled();
        if ramp_frames < frames {
            matrix2x2_block::<L>(
                &mut left[split..],
                &mut right[split..],
                frames - ramp_frames,
                &self.coef,
            );
        }
    }
    // REALTIME_POLICY_END

    /// Snaps every lane to its target and cancels any ramp in flight.
    fn reset(&mut self) {
        let target = self.read_target();
        self.write_current(&target);
        self.remaining = [0; MAX_BANK_LANES];
        self.sync_settled();
    }
}

/// The scalar builtin input section of one track.
pub struct InputBuiltins {
    stage: InputStage<f32>,
}

/// The scalar fader and mute section of one track.
pub struct FaderMuteBuiltins {
    stage: FaderStage<f32>,
}

/// The scalar 2x2 channel matrix section of one track.
pub struct MatrixBuiltins {
    stage: MatrixStage<f32>,
}

/// The full builtin chain of one track: input, fader/mute, matrix.
pub struct BuiltinChain {
    input: InputBuiltins,
    fader_mute: FaderMuteBuiltins,
    matrix: MatrixBuiltins,
    #[cfg(test)]
    fused_dispatches: u32,
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
            #[cfg(test)]
            fused_dispatches: 0,
        })
    }
    pub fn process_input(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport {
        self.input.process(block)
    }
    pub fn process_fader_mute(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport {
        self.fader_mute.process(block)
    }
    pub fn process_matrix(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport {
        self.matrix.process(block)
    }
    /// Runs the whole chain over one already-validated block.
    ///
    /// The block was validated by [`DualMonoBlock::new`]; nothing revalidates it here (F8).
    pub fn process_dual_mono(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport {
        let DualMonoBlock {
            left,
            right,
            first_sample,
        } = block;
        let frames = left.len();
        let mut report = self.input.stage.process(left, right, frames);
        if self.matrix.stage.is_settled() {
            #[cfg(test)]
            {
                self.fused_dispatches += 1;
            }
            fader_matrix_block::<f32>(
                left,
                right,
                frames,
                self.fader_mute.stage.gain[0],
                self.fader_mute.stage.mute[0],
                self.fader_mute.stage.gain[1],
                self.fader_mute.stage.mute[1],
                &self.matrix.stage.coef,
            );
        } else {
            self.fader_mute.stage.process(left, right, frames);
            self.matrix.stage.process(left, right, frames);
        }
        let _ = first_sample;
        report.sanitized_output = 0;
        report
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
    /// Consume the chain and retain only its bankable post-input section.
    pub fn into_input_builtins(self) -> InputBuiltins {
        self.input
    }
}

fn prepare_sections(
    sample_rate: u32,
    parameters: BuiltinParameters,
) -> Result<(InputBuiltins, FaderMuteBuiltins, MatrixBuiltins), BuiltinParameterError> {
    if sample_rate == 0 {
        return Err(BuiltinParameterError::FilterCutoff);
    }
    let matrix = parameters.matrix.checked()?;
    for lane in [parameters.left, parameters.right] {
        if !lane.trim_db.is_finite()
            || !(-144.0..=24.0).contains(&lane.trim_db)
            || !lane.fader_db.is_finite()
            || !(-144.0..=24.0).contains(&lane.fader_db)
        {
            return Err(BuiltinParameterError::GainDomain);
        }
        validate_builtin_filter_cutoff(lane.hpf_hz, sample_rate, 0.0, 10.0)?;
        validate_builtin_filter_cutoff(lane.lpf_hz, sample_rate, 0.0, 10.0)?;
        if lane.hpf_hz > 0.0 && lane.lpf_hz > 0.0 && lane.hpf_hz >= lane.lpf_hz {
            return Err(BuiltinParameterError::FilterOrder);
        }
    }
    let lane = |params: ChannelParameters| -> Result<InputLane, BuiltinParameterError> {
        let trim = db_gain(params.trim_db)?;
        Ok(InputLane {
            trim_signed: if params.polarity_invert { -trim } else { trim },
            hpf: SvfSection::design(sample_rate, zero(params.hpf_hz), true)?,
            lpf: SvfSection::design(sample_rate, zero(params.lpf_hz), false)?,
        })
    };
    let fader = |params: ChannelParameters| -> Result<FaderLane, BuiltinParameterError> {
        Ok(FaderLane {
            gain: db_gain(params.fader_db)?,
            muted: params.muted,
        })
    };
    let track = PreparedInputTrack {
        left: lane(parameters.left)?,
        right: lane(parameters.right)?,
    };
    let faders = [(fader(parameters.left)?, fader(parameters.right)?)];
    Ok((
        InputBuiltins {
            stage: InputStage::<f32>::new(&[track]),
        },
        FaderMuteBuiltins {
            stage: FaderStage::<f32>::new(&faders),
        },
        MatrixBuiltins {
            stage: MatrixStage::<f32>::new(&[(matrix, parameters.smoothing_samples)]),
        },
    ))
}

impl InputBuiltins {
    /// This track's channel-symmetry witness, as far as the input builtins can speak to it.
    ///
    /// `DESIGNED` is the bitwise comparison of the twenty-six words
    /// `InputStage::lane_channel_symmetry` documents. Every other term stays set, and two of them
    /// deliberately so:
    ///
    /// * `SOURCE` is the track's **source mapping**, which this crate never sees; it is decided on
    ///   the control plane by `builtins_compiler::track_mono_source` and conjoined
    ///   there. It is not stamped into this object because the prepared size of this type is a
    ///   sealed fixture-ABI accounting (the builtin-compiler mutation-matrix transcript), and a phase that changes no
    ///   behaviour must not move a sealed byte count to carry a bit nothing rendered reads.
    /// * The two live terms stay set because this object has no queue: a per-node scalar input
    ///   section is reached by the console through `ConsoleInputProcessor`, which owns the
    ///   consumer, folds `ChannelSymmetryWitness::admit` per record and conjoins the result with
    ///   this value -- exactly as `BuiltinBankProcessor` does for the banked form. The seam the
    ///   builtins liveness work was to land on is closed (#210 phase 3): `TrackInputRecord`
    ///   implements `LiveConsoleRecord` with `SEAM = UpstreamOfSeam`, so an asymmetric
    ///   `trim_db` or `polarity_invert` retarget clears `LIVE` at the drain, before the collapse
    ///   dispatch reads the witness. `hpf_hz` and `lpf_hz` remain `PreparedOnly` and have no
    ///   write path at all.
    #[must_use]
    pub fn channel_symmetry(&self) -> ChannelSymmetryWitness {
        let mut witness = ChannelSymmetryWitness::SYMMETRIC;
        witness.set(
            ChannelSymmetryWitness::DESIGNED,
            self.stage.lane_channel_symmetry(0),
        );
        witness
    }

    /// Renders one already-validated block. Infallible: the block shape was checked once (F9).
    pub fn process(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport {
        let frames = block.left.len();
        self.stage.process(block.left, block.right, frames)
    }
    /// Retargets this track's `trim_db` on the addressed channels, over an explicit window.
    ///
    /// The scalar sibling of [`BuiltinInputBank::set_trim_db`]: one body, one width, so a
    /// per-node console track and a bank lane cannot drift.
    ///
    /// # Errors
    ///
    /// [`BuiltinParameterError::GainDomain`] when `db` is outside `trim_db`'s declared domain.
    pub fn set_trim_db(
        &mut self,
        channels: BuiltinLaneSelector,
        db: f32,
        smoothing_samples: u32,
    ) -> Result<(), BuiltinParameterError> {
        let gain = checked_trim_gain(db)?;
        self.stage.set_trim_db(0, channels, gain, smoothing_samples);
        Ok(())
    }

    /// Sets or clears this track's polarity inversion on the addressed channels.
    pub fn set_polarity_invert(
        &mut self,
        channels: BuiltinLaneSelector,
        inverted: bool,
        smoothing_samples: u32,
    ) {
        self.stage
            .set_polarity_invert(0, channels, inverted, smoothing_samples);
    }

    /// The trim coefficient one channel applies to the next frame. Readback only.
    #[must_use]
    pub fn trim_signed(&self, channel: usize) -> f32 {
        self.stage.trim_signed(0, channel)
    }

    /// The trim coefficient one channel is ramping toward. Readback only.
    #[must_use]
    pub fn trim_target(&self, channel: usize) -> f32 {
        self.stage.trim_target(0, channel)
    }
    pub fn reset(&mut self) {
        self.stage.reset();
    }
    pub fn tail(&self) -> BuiltinTail {
        let track = self.stage.lane_track(0);
        if track.left.hpf.enabled
            || track.left.lpf.enabled
            || track.right.hpf.enabled
            || track.right.lpf.enabled
        {
            BuiltinTail::Infinite
        } else {
            BuiltinTail::FiniteZero
        }
    }
    pub fn lifetime_recovered_state(&self) -> (u64, u64) {
        (
            self.stage.lifetime_recovered[0],
            self.stage.lifetime_recovered[1],
        )
    }
    pub fn reset_lifetime_recovered_state(&mut self) {
        self.stage.lifetime_recovered = [0; 2];
    }
}

/// The input stage of a bank at the width its backend selected.
///
/// The two variants differ in size because their lane words do: an eight-lane coefficient set is
/// twice a four-lane one. Boxing the larger one -- which is what `large_enum_variant` suggests --
/// would put every coefficient the render loop loads behind a pointer it has to chase once per
/// bank per block, to save about 560 bytes on a structure there is one of per cohort and which is
/// allocated once at preparation. The space is not worth the indirection.
#[allow(clippy::large_enum_variant)]
enum InputStageKernel {
    /// Four lanes: AArch64 NEON and wasm `simd128`.
    Simd4(InputStage<Simd4>),
    /// Eight lanes: `x86-64-v3`.
    Simd8(InputStage<Simd8>),
}

/// A homogeneous input-builtins bank over one AoSoA cohort.
///
/// # Lane semantics (owned by this crate; consumed by #86)
///
/// `inputs.len()` is in `1..=width.lanes()`. Lanes at or above that count are **padding lanes**:
/// they carry identity coefficients and unit trim, they are sanitised like any other lane so no
/// bit pattern left in the scratch buffer can poison the recurrence, they are excluded from every
/// report counter and from the block boundary check, and their samples are never observed. The
/// caller assigns lanes in sorted member order and is responsible for never gathering into or
/// scattering from a padding lane; there is no `&[bool]` argument and no stored mask copy.
pub struct BuiltinInputBank {
    backend: Backend,
    width: BankWidth,
    members: usize,
    stage: InputStageKernel,
}

impl BuiltinInputBank {
    /// Lane `lane`'s track's channel-symmetry witness, as far as the input builtins speak to it.
    ///
    /// The banked form of [`InputBuiltins::channel_symmetry`]; a padding lane, which no track
    /// owns, declines.
    #[must_use]
    pub fn lane_symmetry(&self, lane: usize) -> ChannelSymmetryWitness {
        let designed = match &self.stage {
            InputStageKernel::Simd4(stage) => stage.lane_channel_symmetry(lane),
            InputStageKernel::Simd8(stage) => stage.lane_channel_symmetry(lane),
        };
        let mut witness = ChannelSymmetryWitness::SYMMETRIC;
        witness.set(ChannelSymmetryWitness::DESIGNED, designed);
        witness
    }

    /// Builds a bank from one to `width.lanes()` independently prepared tracks.
    ///
    /// # Errors
    ///
    /// [`BuiltinParameterError::LaneLength`] if `backend` has no bank width, if `width` is not the
    /// width that backend selects, or if `inputs.len()` is outside `1..=width.lanes()`.
    pub fn new(
        backend: Backend,
        width: BankWidth,
        inputs: Vec<InputBuiltins>,
    ) -> Result<Self, BuiltinParameterError> {
        if BankWidth::for_backend(backend) != Some(width)
            || inputs.is_empty()
            || inputs.len() > width.lanes() as usize
        {
            return Err(BuiltinParameterError::LaneLength);
        }
        let members = inputs.len();
        let tracks: Vec<PreparedInputTrack> = inputs
            .iter()
            .map(|input| input.stage.lane_track(0))
            .collect();
        let stage = match width {
            BankWidth::Four => InputStageKernel::Simd4(InputStage::<Simd4>::new(&tracks)),
            BankWidth::Eight => InputStageKernel::Simd8(InputStage::<Simd8>::new(&tracks)),
        };
        Ok(Self {
            backend,
            width,
            members,
            stage,
        })
    }

    #[must_use]
    pub const fn backend(&self) -> Backend {
        self.backend
    }
    #[must_use]
    pub const fn width(&self) -> BankWidth {
        self.width
    }
    /// Populated lanes; lanes at or above this index are padding lanes.
    #[must_use]
    pub const fn active_lanes(&self) -> usize {
        self.members
    }

    /// Renders one AoSoA block of `frames * width.lanes()` samples per channel.
    ///
    /// The shape is fixed by the prepared plan and validated there, so it is a `debug_assert`
    /// here and never a render-path branch (master plan §4.3).
    pub fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: u32,
    ) -> BuiltinProcessReport {
        let frames = frames as usize;
        debug_assert_eq!(left.len(), frames * self.width.lanes() as usize);
        debug_assert_eq!(right.len(), frames * self.width.lanes() as usize);
        match &mut self.stage {
            InputStageKernel::Simd4(stage) => stage.process(left, right, frames),
            InputStageKernel::Simd8(stage) => stage.process(left, right, frames),
        }
    }

    /// Renders one AoSoA block of the **collapsed** cohort: the left plane only.
    ///
    /// `right` is deliberately not a parameter. A collapsed chain gathers one plane and duplicates
    /// it at the seam, so there is no right block here to be wrong about.
    pub fn process_mono(&mut self, left: &mut [f32], frames: u32) -> BuiltinProcessReport {
        let frames = frames as usize;
        debug_assert_eq!(left.len(), frames * self.width.lanes() as usize);
        match &mut self.stage {
            InputStageKernel::Simd4(stage) => stage.process_mono(left, frames),
            InputStageKernel::Simd8(stage) => stage.process_mono(left, frames),
        }
    }

    /// Whether this bank may run [`BuiltinInputBank::process_mono`] at all.
    ///
    /// Fixed by preparation: the elision plan is decided once, from the coefficient words and a
    /// `+0.0` state, and nothing on the render path re-decides it.
    #[must_use]
    pub const fn supports_mono_collapse(&self) -> bool {
        match &self.stage {
            InputStageKernel::Simd4(stage) => stage.mono_collapse_gate(),
            InputStageKernel::Simd8(stage) => stage.mono_collapse_gate(),
        }
    }

    /// Copies every lane's left-channel per-channel state onto the right channel (the disengage
    /// copy): the integrators and the trim ramp record.
    pub fn desymmetrize(&mut self) {
        match &mut self.stage {
            InputStageKernel::Simd4(stage) => stage.desymmetrize(),
            InputStageKernel::Simd8(stage) => stage.desymmetrize(),
        }
    }

    /// Whether this bank can prove, right now, that its two channels' state is bit-equal (M3).
    #[must_use]
    pub fn channels_agree(&self) -> bool {
        match &self.stage {
            InputStageKernel::Simd4(stage) => stage.channels_agree(),
            InputStageKernel::Simd8(stage) => stage.channels_agree(),
        }
    }

    /// Retargets one member lane's `trim_db` on the addressed channels, over an explicit window.
    ///
    /// The lane's polarity is preserved: the magnitude changes and the sign does not, because
    /// `trim_db` and `polarity_invert` are two parameters that share one coefficient, not one
    /// parameter with two spellings.
    ///
    /// # Errors
    ///
    /// [`BuiltinParameterError::LaneLength`] when `lane` is not a populated member, and
    /// [`BuiltinParameterError::GainDomain`] when `db` is outside the declared `[-144, 24]`
    /// domain of `trim_db`.
    pub fn set_trim_db(
        &mut self,
        lane: usize,
        channels: BuiltinLaneSelector,
        db: f32,
        smoothing_samples: u32,
    ) -> Result<(), BuiltinParameterError> {
        if lane >= self.members {
            return Err(BuiltinParameterError::LaneLength);
        }
        let gain = checked_trim_gain(db)?;
        match &mut self.stage {
            InputStageKernel::Simd4(stage) => {
                stage.set_trim_db(lane, channels, gain, smoothing_samples);
            }
            InputStageKernel::Simd8(stage) => {
                stage.set_trim_db(lane, channels, gain, smoothing_samples);
            }
        }
        Ok(())
    }

    /// Sets or clears one member lane's polarity inversion on the addressed channels.
    ///
    /// A retarget of the **same** coefficient to `-trim_signed`, so the declick is the trim ramp's:
    /// the linear ramp carries the coefficient through zero over the requested window. There is no
    /// second DSP path and no crossfade.
    ///
    /// # Errors
    ///
    /// [`BuiltinParameterError::LaneLength`] when `lane` is not a populated member.
    pub fn set_polarity_invert(
        &mut self,
        lane: usize,
        channels: BuiltinLaneSelector,
        inverted: bool,
        smoothing_samples: u32,
    ) -> Result<(), BuiltinParameterError> {
        if lane >= self.members {
            return Err(BuiltinParameterError::LaneLength);
        }
        match &mut self.stage {
            InputStageKernel::Simd4(stage) => {
                stage.set_polarity_invert(lane, channels, inverted, smoothing_samples);
            }
            InputStageKernel::Simd8(stage) => {
                stage.set_polarity_invert(lane, channels, inverted, smoothing_samples);
            }
        }
        Ok(())
    }

    /// The trim coefficient one lane and channel applies to the next frame. Readback only.
    #[must_use]
    pub fn trim_signed(&self, lane: usize, channel: usize) -> f32 {
        match &self.stage {
            InputStageKernel::Simd4(stage) => stage.trim_signed(lane, channel),
            InputStageKernel::Simd8(stage) => stage.trim_signed(lane, channel),
        }
    }

    /// The trim coefficient one lane and channel is ramping toward. Readback only.
    #[must_use]
    pub fn trim_target(&self, lane: usize, channel: usize) -> f32 {
        match &self.stage {
            InputStageKernel::Simd4(stage) => stage.trim_target(lane, channel),
            InputStageKernel::Simd8(stage) => stage.trim_target(lane, channel),
        }
    }

    /// Resets only the per-lane filter state; prepared coefficients remain unchanged.
    pub fn reset(&mut self) {
        match &mut self.stage {
            InputStageKernel::Simd4(stage) => stage.reset(),
            InputStageKernel::Simd8(stage) => stage.reset(),
        }
    }
}

/// The dispatched fader-ramp stage of a bank, at the width the selected backend chose.
///
/// Both variants are held inline for the reason [`InputStageKernel`] states: boxing the larger one
/// would put the coefficients the render loop loads behind a pointer it chases once per bank per
/// block, to save a few hundred bytes on a structure there is one of per cohort. The space is not
/// worth the indirection.
#[allow(clippy::large_enum_variant)]
enum FaderStageKernel {
    /// Four lanes: AArch64 NEON and wasm `simd128`.
    Simd4(FaderRampStage<Simd4>),
    /// Eight lanes: `x86-64-v3`.
    Simd8(FaderRampStage<Simd8>),
}

/// A homogeneous fader/mute bank over one AoSoA cohort (issue #212, the banked strip).
///
/// # What banking does and does not change
///
/// Nothing numeric. The bank is `FaderRampStage` at `Simd4` or `Simd8`, and a per-track fader is
/// the same type at `f32`, so a member lane's output bits are the bits that track produced as its
/// own dispatched op -- settled or mid-ramp, muted or not. What banking removes is one graph op,
/// one arena buffer and one `dyn` dispatch per track per block, and -- because the fader now sits
/// in the cohort's chain rather than between two of them -- one planar/AoSoA round-trip.
///
/// # Lane semantics (owned by this crate)
///
/// `faders.len()` is in `1..=width.lanes()`. Lanes at or above that count are **padding lanes**:
/// unit gain, unmuted, never ramping, so they are arithmetically inert. They run through the
/// kernel like any other lane and their samples are never observed. The caller assigns lanes in
/// sorted member order; there is no stored mask and no `&[bool]` argument.
///
/// # The drain contract lives one level up
///
/// This type exposes the retargets ([`Self::set_fader_db`], [`Self::set_mute`]) and knows nothing
/// about queues. The bank's owner drains its members' per-track command queues at the top of the
/// block and calls these, which is what keeps `TrackFaderRecord`'s single-consumer SPSC
/// contract intact while the consumer moves from the per-track node to the bank.
pub struct BuiltinFaderBank {
    backend: Backend,
    width: BankWidth,
    members: usize,
    stage: FaderStageKernel,
}

impl BuiltinFaderBank {
    /// Builds a bank from one to `width.lanes()` independently prepared tracks.
    ///
    /// # Errors
    ///
    /// [`BuiltinParameterError::LaneLength`] if `backend` has no bank width, if `width` is not the
    /// width that backend selects, or if `faders.len()` is outside `1..=width.lanes()`.
    /// [`BuiltinParameterError::GainDomain`] if a declared `fader_db` is outside `[-144, 24]`.
    pub fn new(
        backend: Backend,
        width: BankWidth,
        faders: Vec<BuiltinParameters>,
    ) -> Result<Self, BuiltinParameterError> {
        if BankWidth::for_backend(backend) != Some(width)
            || faders.is_empty()
            || faders.len() > width.lanes() as usize
        {
            return Err(BuiltinParameterError::LaneLength);
        }
        let members = faders.len();
        let lanes = faders
            .into_iter()
            .map(fader_lanes)
            .collect::<Result<Vec<_>, _>>()?;
        let stage = match width {
            BankWidth::Four => FaderStageKernel::Simd4(FaderRampStage::<Simd4>::new(&lanes)),
            BankWidth::Eight => FaderStageKernel::Simd8(FaderRampStage::<Simd8>::new(&lanes)),
        };
        Ok(Self {
            backend,
            width,
            members,
            stage,
        })
    }

    #[must_use]
    pub const fn backend(&self) -> Backend {
        self.backend
    }
    #[must_use]
    pub const fn width(&self) -> BankWidth {
        self.width
    }
    /// Populated lanes; lanes at or above this index are padding lanes.
    #[must_use]
    pub const fn active_lanes(&self) -> usize {
        self.members
    }

    /// Retargets one member lane's fader gain in decibels over an explicit ramp window.
    ///
    /// # Errors
    ///
    /// [`BuiltinParameterError::LaneLength`] when `lane` is not a populated member, and
    /// [`BuiltinParameterError::GainDomain`] when `db` is outside the declared `[-144, 24]`
    /// domain of `fader_db`.
    pub fn set_fader_db(
        &mut self,
        lane: usize,
        channels: BuiltinLaneSelector,
        db: f32,
        smoothing_samples: u32,
    ) -> Result<(), BuiltinParameterError> {
        if lane >= self.members {
            return Err(BuiltinParameterError::LaneLength);
        }
        let gain = checked_fader_gain(db)?;
        match &mut self.stage {
            FaderStageKernel::Simd4(stage) => {
                stage.set_fader_gain(lane, channels, gain, smoothing_samples);
            }
            FaderStageKernel::Simd8(stage) => {
                stage.set_fader_gain(lane, channels, gain, smoothing_samples);
            }
        }
        Ok(())
    }

    /// Sets or clears one member lane's mute, as a retarget of the same gain.
    ///
    /// # Errors
    ///
    /// [`BuiltinParameterError::LaneLength`] when `lane` is not a populated member.
    pub fn set_mute(
        &mut self,
        lane: usize,
        channels: BuiltinLaneSelector,
        muted: bool,
        smoothing_samples: u32,
    ) -> Result<(), BuiltinParameterError> {
        if lane >= self.members {
            return Err(BuiltinParameterError::LaneLength);
        }
        match &mut self.stage {
            FaderStageKernel::Simd4(stage) => {
                stage.set_mute(lane, channels, muted, smoothing_samples)
            }
            FaderStageKernel::Simd8(stage) => {
                stage.set_mute(lane, channels, muted, smoothing_samples)
            }
        }
        Ok(())
    }

    /// The settled gain of one lane and channel, for tests and control-plane readback.
    #[must_use]
    pub fn target_gain(&self, lane: usize, channel: usize) -> f32 {
        match &self.stage {
            FaderStageKernel::Simd4(stage) => stage.target_gain(lane, channel % 2),
            FaderStageKernel::Simd8(stage) => stage.target_gain(lane, channel % 2),
        }
    }

    /// Whether one lane and channel is muted.
    #[must_use]
    pub const fn is_muted(&self, lane: usize, channel: usize) -> bool {
        match &self.stage {
            FaderStageKernel::Simd4(stage) => stage.is_muted(lane, channel % 2),
            FaderStageKernel::Simd8(stage) => stage.is_muted(lane, channel % 2),
        }
    }

    /// Renders one AoSoA block of `frames * width.lanes()` samples per channel.
    ///
    /// The shape is fixed by the prepared plan and validated there, so it is a `debug_assert`
    /// here and never a render-path branch (master plan §4.3).
    pub fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: u32,
    ) -> BuiltinProcessReport {
        let frames = frames as usize;
        debug_assert_eq!(left.len(), frames * self.width.lanes() as usize);
        debug_assert_eq!(right.len(), frames * self.width.lanes() as usize);
        match &mut self.stage {
            FaderStageKernel::Simd4(stage) => stage.process(left, right, frames),
            FaderStageKernel::Simd8(stage) => stage.process(left, right, frames),
        }
        BuiltinProcessReport::default()
    }

    /// Runs the settled fader and matrix stages in one traversal when their shapes and ramps
    /// agree. Returns false without touching either stage when a ramp is still active.
    pub fn try_process_settled_with_matrix(
        &mut self,
        matrix: &mut BuiltinMatrixBank,
        left: &mut [f32],
        right: &mut [f32],
        frames: u32,
    ) -> bool {
        if self.backend != matrix.backend || self.width != matrix.width || self.members != matrix.members
            || self.remaining_nonzero() || matrix.remaining_nonzero()
        {
            return false;
        }
        match (&self.stage, &matrix.stage) {
            (FaderStageKernel::Simd4(fader), MatrixStageKernel::Simd4(matrix)) => {
                fader_matrix_block::<Simd4>(left, right, frames as usize, fader.ramp[0].current, fader.ramp[0].mute, fader.ramp[1].current, fader.ramp[1].mute, &matrix.coef);
            }
            (FaderStageKernel::Simd8(fader), MatrixStageKernel::Simd8(matrix)) => {
                fader_matrix_block::<Simd8>(left, right, frames as usize, fader.ramp[0].current, fader.ramp[0].mute, fader.ramp[1].current, fader.ramp[1].mute, &matrix.coef);
            }
            _ => return false,
        }
        true
    }

    fn remaining_nonzero(&self) -> bool {
        match &self.stage {
            FaderStageKernel::Simd4(stage) => stage.remaining.iter().flatten().any(|v| *v != 0),
            FaderStageKernel::Simd8(stage) => stage.remaining.iter().flatten().any(|v| *v != 0),
        }
    }

    /// Snaps every lane to its target and cancels any ramp in flight.
    pub fn reset(&mut self) {
        match &mut self.stage {
            FaderStageKernel::Simd4(stage) => stage.reset(),
            FaderStageKernel::Simd8(stage) => stage.reset(),
        }
    }
}

/// The dispatched matrix stage of a bank, at the width the selected backend chose.
///
/// Both variants are held inline for the reason [`InputStageKernel`] states: boxing the larger one
/// would put the coefficients the render loop loads behind a pointer it chases once per bank per
/// block, to save a few hundred bytes on a structure there is one of per cohort. The space is not
/// worth the indirection.
#[allow(clippy::large_enum_variant)]
enum MatrixStageKernel {
    /// Four lanes: AArch64 NEON and wasm `simd128`.
    Simd4(MatrixStage<Simd4>),
    /// Eight lanes: `x86-64-v3`.
    Simd8(MatrixStage<Simd8>),
}

/// A homogeneous 2x2 pan/matrix bank over one AoSoA cohort (issue #212, the banked strip).
///
/// `MatrixStage` has been per-lane and width-generic since it was written -- a per-track matrix
/// is that type at `f32` -- so this bank introduces no arithmetic at all. It is the same
/// settled/ramping kernel choice, made per bank instead of per track, over the same per-lane ramp
/// state. Padding lanes carry [`Matrix2x2::IDENTITY`] with a zero window, so they settle
/// immediately into the stage's identity mask and pass their samples through untouched.
///
/// As with [`BuiltinFaderBank`], the queue lives one level up: this type exposes the retarget
/// and the owner drains `TrackControlRecord` for each member at the top of the block.
pub struct BuiltinMatrixBank {
    backend: Backend,
    width: BankWidth,
    members: usize,
    stage: MatrixStageKernel,
}

impl BuiltinMatrixBank {
    /// Builds a bank from one to `width.lanes()` prepared `(matrix, window)` pairs.
    ///
    /// # Errors
    ///
    /// [`BuiltinParameterError::LaneLength`] if `backend` has no bank width, if `width` is not the
    /// width that backend selects, or if `lanes.len()` is outside `1..=width.lanes()`.
    /// [`BuiltinParameterError::MatrixCoefficient`] if a coefficient is outside `[-1, 1]`.
    pub fn new(
        backend: Backend,
        width: BankWidth,
        lanes: Vec<(Matrix2x2, u32)>,
    ) -> Result<Self, BuiltinParameterError> {
        if BankWidth::for_backend(backend) != Some(width)
            || lanes.is_empty()
            || lanes.len() > width.lanes() as usize
        {
            return Err(BuiltinParameterError::LaneLength);
        }
        let members = lanes.len();
        let lanes = lanes
            .into_iter()
            .map(|(matrix, samples)| Ok((matrix.checked()?, samples)))
            .collect::<Result<Vec<_>, BuiltinParameterError>>()?;
        let stage = match width {
            BankWidth::Four => MatrixStageKernel::Simd4(MatrixStage::<Simd4>::new(&lanes)),
            BankWidth::Eight => MatrixStageKernel::Simd8(MatrixStage::<Simd8>::new(&lanes)),
        };
        Ok(Self {
            backend,
            width,
            members,
            stage,
        })
    }

    #[must_use]
    pub const fn backend(&self) -> Backend {
        self.backend
    }
    #[must_use]
    pub const fn width(&self) -> BankWidth {
        self.width
    }
    /// Populated lanes; lanes at or above this index are padding lanes.
    #[must_use]
    pub const fn active_lanes(&self) -> usize {
        self.members
    }

    /// Retargets one member lane's 2x2 matrix over an explicit ramp window.
    ///
    /// # Errors
    ///
    /// [`BuiltinParameterError::LaneLength`] when `lane` is not a populated member, and
    /// [`BuiltinParameterError::MatrixCoefficient`] when a coefficient is outside `[-1, 1]` or is
    /// not finite.
    pub fn set_target_smoothed(
        &mut self,
        lane: usize,
        target: Matrix2x2,
        smoothing_samples: u32,
    ) -> Result<(), BuiltinParameterError> {
        if lane >= self.members {
            return Err(BuiltinParameterError::LaneLength);
        }
        match &mut self.stage {
            MatrixStageKernel::Simd4(stage) => {
                stage.set_target_over(lane, target, smoothing_samples)
            }
            MatrixStageKernel::Simd8(stage) => {
                stage.set_target_over(lane, target, smoothing_samples)
            }
        }
    }

    /// Renders one AoSoA block of `frames * width.lanes()` samples per channel.
    pub fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: u32,
    ) -> BuiltinProcessReport {
        let frames = frames as usize;
        debug_assert_eq!(left.len(), frames * self.width.lanes() as usize);
        debug_assert_eq!(right.len(), frames * self.width.lanes() as usize);
        match &mut self.stage {
            MatrixStageKernel::Simd4(stage) => stage.process(left, right, frames),
            MatrixStageKernel::Simd8(stage) => stage.process(left, right, frames),
        }
        BuiltinProcessReport::default()
    }

    fn remaining_nonzero(&self) -> bool {
        match &self.stage {
            MatrixStageKernel::Simd4(stage) => stage.remaining.iter().any(|v| *v != 0),
            MatrixStageKernel::Simd8(stage) => stage.remaining.iter().any(|v| *v != 0),
        }
    }

    /// Snaps every lane to its target and cancels any ramp in flight.
    pub fn reset(&mut self) {
        match &mut self.stage {
            MatrixStageKernel::Simd4(stage) => stage.reset(),
            MatrixStageKernel::Simd8(stage) => stage.reset(),
        }
    }
}

impl FaderMuteBuiltins {
    /// Renders one already-validated block.
    ///
    /// Feed-forward with `|gain| <= 15.85`, so finite in implies finite out: no sanitisation, no
    /// boundary check and no counters (D7). The report is always the default.
    pub fn process(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport {
        let frames = block.left.len();
        self.stage.process(block.left, block.right, frames);
        BuiltinProcessReport::default()
    }
    fn reset(&mut self) {}
}

/// Which lane of a dual-mono track a live fader or mute command addresses (issue #140 B).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltinLaneSelector {
    /// The left lane only.
    Left,
    /// The right lane only.
    Right,
    /// Both lanes, with one shared ramp window.
    Both,
}

impl BuiltinLaneSelector {
    const fn covers(self, lane: usize) -> bool {
        matches!(
            (self, lane),
            (Self::Left, 0) | (Self::Right, 1) | (Self::Both, _)
        )
    }
}

/// The live-console fader and mute section of one track (issue #140 B).
///
/// # The ramped-fader decision, and why it is a separate type
///
/// [`FaderMuteBuiltins`] is the *prepared-only* fader: one multiply and one `andnot` per frame,
/// with no ramp state at all. (This paragraph read "`fader_db` and `mute` declare
/// `BuiltinParameterUpdateRate::PreparedOnly`" when it was written; those rows are `BlockTarget`
/// since #140 B flipped them, and the ABI table is the authority. What the sentence was about is
/// the *type*, and that is unchanged.) Making that type ramp would change the fixed
/// input/fader/matrix section layout, the builtin resource report, and the frozen
/// builtins-compiler transcript for **every** session, console or not.
///
/// This type is the ramped fader instead. It exists only for a track a live console drives, it is
/// bound only by `ConsoleFaderProcessor`, and [`FaderMuteBuiltins`] is byte-for-byte the type it
/// always was. No builtins fixture digest, no frozen transcript and no corpus digest moves,
/// because for a command-free session none of this code is reachable.
///
/// # Mute is a fader endpoint, not a discontinuity
///
/// A mute is a retarget of the same gain to `0`, over the same window a fader move uses, so
/// muting a live signal fades it rather than clipping it off. Unmuting retargets back to the
/// lane's current `fader_db`. A **settled** mute is still the exact `+0.0` the prepared path
/// produces -- `andnot`, not a multiply -- so a muted lane's output bits are identical to a
/// session that declared the mute in its JSON. During the ramp itself the lane is a plain
/// multiply, which is what makes the fade a fade; the final ramp sample multiplies by exactly
/// `+0.0` and can therefore carry the input's sign, and every sample after it is exactly `+0.0`.
///
/// # D11, once per retarget
///
/// `step = (target - current) / N` at the moment the target changes, then `current += step` per
/// sample and an exact assignment of `target` on update `N` (master plan D11). There is no
/// division per sample and no allocation anywhere on this path.
pub struct FaderMuteRampBuiltins {
    /// The one ramped-fader body, at width one. `lane` is always `0`; the two dual-mono sides are
    /// the stage's two channels.
    stage: FaderRampStage<f32>,
}

impl FaderMuteRampBuiltins {
    /// Builds the ramped fader from the same prepared parameters [`FaderMuteBuiltins`] uses.
    ///
    /// # Errors
    ///
    /// [`BuiltinParameterError::GainDomain`] when a declared `fader_db` is outside `[-144, 24]`
    /// or maps to a coefficient that is not representable.
    pub fn new(parameters: BuiltinParameters) -> Result<Self, BuiltinParameterError> {
        Ok(Self {
            stage: FaderRampStage::<f32>::new(&[fader_lanes(parameters)?]),
        })
    }

    /// Retarget one or both lanes' fader gain in decibels over an explicit ramp window.
    ///
    /// A muted lane keeps its `0.0` target: the new gain is remembered and takes effect when the
    /// lane is unmuted, exactly as a physical console's fader does.
    ///
    /// # Errors
    ///
    /// [`BuiltinParameterError::GainDomain`] when `db` is outside the declared `[-144, 24]`
    /// domain of `fader_db`.
    pub fn set_fader_db(
        &mut self,
        lanes: BuiltinLaneSelector,
        db: f32,
        smoothing_samples: u32,
    ) -> Result<(), BuiltinParameterError> {
        let gain = checked_fader_gain(db)?;
        self.stage.set_fader_gain(0, lanes, gain, smoothing_samples);
        Ok(())
    }

    /// Set or clear one or both lanes' mute, as a retarget of the same gain.
    pub fn set_mute(&mut self, lanes: BuiltinLaneSelector, muted: bool, smoothing_samples: u32) {
        self.stage.set_mute(0, lanes, muted, smoothing_samples);
    }

    /// The settled gain of one lane, for tests and control-plane readback.
    #[must_use]
    pub fn target_gain(&self, lane: usize) -> f32 {
        self.stage.target_gain(0, lane % 2)
    }

    /// Whether one lane is muted.
    #[must_use]
    pub const fn is_muted(&self, lane: usize) -> bool {
        self.stage.is_muted(0, lane % 2)
    }

    /// Renders one already-validated block.
    pub fn process(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport {
        let frames = block.left.len();
        self.stage.process(block.left, block.right, frames);
        BuiltinProcessReport::default()
    }

    /// Snaps both lanes to their targets and cancels any ramp in flight.
    pub fn reset(&mut self) {
        self.stage.reset();
    }
}

/// Validates one live `fader_db` against the declared domain and converts it to a coefficient.
///
/// The domain is `fader_db`'s own, so a live move is admitted on exactly the terms a declared one
/// is; sharing this with preparation is what keeps the two from drifting.
fn checked_fader_gain(db: f32) -> Result<f32, BuiltinParameterError> {
    if !db.is_finite() || !(-144.0..=24.0).contains(&db) {
        return Err(BuiltinParameterError::GainDomain);
    }
    db_gain(db)
}

/// Validates one live `trim_db` against the declared domain and converts it to a coefficient.
///
/// The domain is `trim_db`'s own in `BUILTIN_PARAMETER_DESCRIPTORS` -- the same `[-144, 24]`
/// `fader_db` carries, and the same range `prepare_sections` checks a declared value against -- so
/// a live move is admitted on exactly the terms a declared one is. Sharing this with preparation
/// is what keeps the two from drifting, which is the argument [`checked_fader_gain`] makes for the
/// fader.
///
/// The result is a **magnitude**: the polarity sign is applied by the caller, because
/// `polarity_invert` is its own parameter with its own command kind and a trim move must not
/// silently clear it.
fn checked_trim_gain(db: f32) -> Result<f32, BuiltinParameterError> {
    if !db.is_finite() || !(-144.0..=24.0).contains(&db) {
        return Err(BuiltinParameterError::GainDomain);
    }
    db_gain(db)
}

/// The prepared fader pair of one track, validated on the same terms as a live move.
fn fader_lanes(
    parameters: BuiltinParameters,
) -> Result<(FaderLane, FaderLane), BuiltinParameterError> {
    let lane = |params: ChannelParameters| -> Result<FaderLane, BuiltinParameterError> {
        Ok(FaderLane {
            gain: checked_fader_gain(params.fader_db)?,
            muted: params.muted,
        })
    };
    Ok((lane(parameters.left)?, lane(parameters.right)?))
}

impl MatrixBuiltins {
    pub fn set_target(&mut self, target: Matrix2x2) -> Result<(), BuiltinParameterError> {
        self.stage.set_target(0, target)
    }
    /// Retarget the 2x2 matrix over an explicit ramp window (issue #137 D1).
    ///
    /// The window becomes this stage's smoothing window, so a subsequent [`Self::set_target`]
    /// uses it too. `matrix_ll/lr/rl/rr` are the only builtin parameters whose declared update
    /// rate is `BuiltinParameterUpdateRate::BlockTarget`, which is why this is the one live
    /// builtin setter the ABI admits.
    ///
    /// # Errors
    ///
    /// [`BuiltinParameterError::MatrixCoefficient`] when a coefficient is outside `[-1, 1]` or is not
    /// finite.
    pub fn set_target_smoothed(
        &mut self,
        target: Matrix2x2,
        smoothing_samples: u32,
    ) -> Result<(), BuiltinParameterError> {
        self.stage.set_target_over(0, target, smoothing_samples)
    }
    /// Renders one already-validated block. Feed-forward with `|m| <= 1`: no checks, no counters.
    pub fn process(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport {
        let frames = block.left.len();
        self.stage.process(block.left, block.right, frames);
        BuiltinProcessReport::default()
    }
    pub fn reset(&mut self) {
        self.stage.reset();
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
        (math::cos(theta) as f32, math::sin(theta) as f32)
    };
    let (ll, rl) = gains(left);
    let (lr, rr) = gains(right);
    Matrix2x2 { ll, lr, rl, rr }.checked()
}

pub fn balance_matrix(balance: f32) -> Result<Matrix2x2, BuiltinParameterError> {
    if !balance.is_finite() || !(-1.0..=1.0).contains(&balance) {
        return Err(BuiltinParameterError::MatrixCoefficient);
    }
    let gain = math::cos(f64::from(balance.abs()) * core::f64::consts::FRAC_PI_2) as f32;
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
        let decay = math::pow(
            10.0,
            -f64::from(config.peak_decay_db_per_second) / (20.0 * f64::from(sample_rate)),
        ) as f32;
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

    // REALTIME_POLICY_BEGIN
    /// Observes one block, split at the window boundaries it crosses.
    ///
    /// The split is computed once per segment instead of testing the period after every sample,
    /// and the whole per-sample configuration — hold length, decay multiplier, whether decay is
    /// enabled at all — is hoisted into locals before the loop; [`MeterConfig`] is never passed by
    /// value per sample (F7). `sqrt` runs once per emitted window, when a snapshot is built.
    ///
    /// # Errors
    ///
    /// [`MeterObservationError::LaneLength`] if the channels differ in length, and
    /// [`MeterObservationError::SampleTimeOverflow`] if the block would run past `u64::MAX`.
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
        let period = self.config.period_frames.get();
        let window = MeterWindow {
            hold_frames: self.config.peak_hold_frames,
            decay: self.decay,
            decay_enabled: self.config.peak_decay_db_per_second != 0.0,
        };
        // Issue #163 phase 4 item 3: a silent block against a settled lane is a provable no-op,
        // and the loop below is otherwise a third full read of the same audio (after the kernel's
        // own store and after D7's boundary scan), with a serial `f64` energy dependency and four
        // branches per sample per channel.
        //
        // The proof, sample by sample, for a lane whose `held` is `+0.0` and whose
        // `hold_remaining` already equals the configured hold length:
        //
        // * `normal_or_zero(±0.0)` is true, so `sanitized` does not move and the sample is not
        //   replaced;
        // * `absolute` is `+0.0`, so `absolute > peak` is false for every reachable `peak` (peak
        //   is a magnitude and never negative) and `peak` does not move;
        // * `energy += 0.0 * 0.0` adds exactly `+0.0`, which is the identity on every
        //   non-negative `f64` -- and `energy` only ever grows from `+0.0`;
        // * `absolute >= 1.0` is false, so `clipped` does not move;
        // * `absolute >= held` is true exactly when `held == 0.0`, which re-arms `hold_remaining`
        //   to `hold_frames` -- a no-op only once it is already there, which is why that is a
        //   precondition rather than a consequence.
        //
        // Both signed zeros qualify: `(-0.0).abs()` is `+0.0` and `(-0.0) * (-0.0)` is `+0.0`, so
        // the `== 0.0` test below (which matches both) admits exactly the values the proof covers.
        // Neither `self.frames`, `self.start`, `self.sequence` nor the window split is touched
        // here, so a skipped block still advances the window and still emits on the period
        // boundary -- the early-out is inside the segment, not around it.
        //
        // Cost on the active path is one compare: `all` short-circuits on the first nonzero
        // sample, so a block carrying signal pays for the first frame and nothing more.
        let settled_silence = self.left.held == 0.0
            && self.right.held == 0.0
            && self.left.hold_remaining == window.hold_frames
            && self.right.hold_remaining == window.hold_frames
            && left[..len].iter().all(|sample| *sample == 0.0)
            && right[..len].iter().all(|sample| *sample == 0.0);
        let mut offset = 0;
        while offset < len {
            let take = ((period - self.frames) as usize).min(len - offset);
            let end = offset + take;
            if settled_silence {
                self.frames = self.frames.saturating_add(take as u32);
                offset = end;
                if self.frames == period {
                    self.emit();
                }
                continue;
            }
            observe_segment(
                &mut self.left,
                &left[offset..end],
                window,
                &mut self.cumulative_clipped,
                &mut self.cumulative_sanitized,
            );
            observe_segment(
                &mut self.right,
                &right[offset..end],
                window,
                &mut self.cumulative_clipped,
                &mut self.cumulative_sanitized,
            );
            self.frames = self.frames.saturating_add(take as u32);
            offset = end;
            if self.frames == period {
                self.emit();
            }
        }
        Ok(())
    }
    // REALTIME_POLICY_END
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

/// The per-sample meter configuration, hoisted out of [`MeterConfig`] once per block.
#[derive(Clone, Copy)]
struct MeterWindow {
    /// Frames a new peak is held for before it may decay.
    hold_frames: u32,
    /// Precomputed per-sample decay multiplier.
    decay: f32,
    /// Whether decay is enabled at all.
    decay_enabled: bool,
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

/// Accumulates one lane over a segment that lies entirely inside one meter window.
///
/// Branch-free per sample except for the held-peak state machine, which is a three-way scalar
/// choice on counters rather than on sample values. `peak` is the D8 select form
/// `select(a > p, a, p)`, never `f32::max`: the two disagree on `+/-0.0` ordering, and `f32::max`
/// is forbidden on any path whose bits are pinned.
fn observe_segment(
    lane: &mut MeterLane,
    samples: &[f32],
    window: MeterWindow,
    cumulative_clipped: &mut u64,
    cumulative_sanitized: &mut u64,
) {
    let mut peak = lane.peak;
    let mut energy = lane.energy;
    let mut held = lane.held;
    let mut hold_remaining = lane.hold_remaining;
    let mut clipped = 0_u64;
    let mut sanitized = 0_u64;
    for sample in samples.iter().copied() {
        let invalid = !normal_or_zero(sample);
        let sample = if invalid { 0.0 } else { sample };
        sanitized += u64::from(invalid);
        let absolute = sample.abs();
        peak = if absolute > peak { absolute } else { peak };
        energy += f64::from(sample) * f64::from(sample);
        clipped += u64::from(absolute >= 1.0);
        if absolute >= held {
            held = absolute;
            hold_remaining = window.hold_frames;
        } else if hold_remaining > 0 {
            hold_remaining -= 1;
        } else if window.decay_enabled {
            held = flush_subnormal(held * window.decay);
        }
    }
    lane.peak = peak;
    lane.energy = energy;
    lane.held = held;
    lane.hold_remaining = hold_remaining;
    lane.clipped = lane.clipped.saturating_add(clipped);
    lane.sanitized = lane.sanitized.saturating_add(sanitized);
    *cumulative_clipped = cumulative_clipped.saturating_add(clipped);
    *cumulative_sanitized = cumulative_sanitized.saturating_add(sanitized);
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

/// `10^(dB / 20)` designed in `f64` through [`math`] and rounded once.
fn db_gain(db: f32) -> Result<f32, BuiltinParameterError> {
    let value = math::pow(10.0, f64::from(db) / 20.0) as f32;
    if normal_or_zero(value) {
        Ok(zero(value))
    } else {
        Err(BuiltinParameterError::GainDomain)
    }
}

/// Preparation-time coefficient representability: finite and not subnormal.
///
/// This is control-plane classification, not a render-path check: D7 replaced every per-value
/// render check with the in-kernel flush and the once-per-block boundary scan.
fn normal_or_zero(value: f32) -> bool {
    value.is_finite() && !value.is_subnormal()
}

/// Maps a finite subnormal or non-finite value to `+0.0`; used by the meter's held-peak decay.
fn flush_subnormal(value: f32) -> f32 {
    if normal_or_zero(value) { value } else { 0.0 }
}

/// Normalises `-0.0` to `+0.0` in a prepared control value.
fn zero(value: f32) -> f32 {
    if value == 0.0 { 0.0 } else { value }
}

/// Evidence and fixture access to prepared words and retained state.
///
/// Not part of the supported surface: it exists so that gates and the fixture generator can read
/// and inject the exact words a render path uses without going through production output, which
/// master plan §8 forbids as a source of pins.
#[doc(hidden)]
pub mod test_support {
    use super::{
        BuiltinChain, BuiltinInputBank, BuiltinParameterError, InputBuiltins, InputStageKernel,
        Matrix2x2, MatrixBuiltins, SvfSection,
    };

    /// The seven words `[c1, a2, a3, k, m0, m1, m2]` of one designed section.
    ///
    /// # Errors
    ///
    /// Propagates [`BuiltinParameterError::FilterCoefficients`] when a cast word is not
    /// representable.
    pub fn section_words(
        rate: u32,
        cutoff: f32,
        high_pass: bool,
    ) -> Result<[u32; 7], BuiltinParameterError> {
        SvfSection::design(rate, cutoff, high_pass).map(SvfSection::words)
    }

    /// The four prepared sections of one input chain, `[l_hpf, l_lpf, r_hpf, r_lpf]`.
    #[must_use]
    pub fn input_section_words(input: &InputBuiltins) -> [[u32; 7]; 4] {
        let track = input.stage.lane_track(0);
        [
            track.left.hpf.words(),
            track.left.lpf.words(),
            track.right.hpf.words(),
            track.right.lpf.words(),
        ]
    }

    /// The folded trim words of one input chain, `[left, right]`.
    #[must_use]
    pub fn input_trim_words(input: &InputBuiltins) -> [u32; 2] {
        let track = input.stage.lane_track(0);
        [
            track.left.trim_signed.to_bits(),
            track.right.trim_signed.to_bits(),
        ]
    }

    /// Retained state words `[l_hpf_ic1, l_hpf_ic2, l_lpf_ic1, l_lpf_ic2, r_hpf_ic1, ..]`.
    #[must_use]
    pub fn input_state_words(input: &InputBuiltins) -> [u32; 8] {
        input.stage.lane_state_words(0)
    }

    /// The live trim ramp's eight words, `[current_l, current_r, target_l, target_r, step_l,
    /// step_r, countdown_l, countdown_r]` (#210 phase 3).
    ///
    /// The **countdown** words are what make this more than a readback: they are written only by
    /// the ramping kernel, so a settled block that leaves them at `+0.0` is a settled block that
    /// took the settled arm. That is the one observable the class-A OFF dispatch has -- the two
    /// arms are bit-identical in the *plane*, by the elision proof, so a digest cannot tell them
    /// apart and only the ramp state can.
    #[must_use]
    pub fn input_trim_ramp_words(input: &InputBuiltins) -> [u32; 8] {
        input.stage.trim_ramp_words(0)
    }

    /// [`input_trim_ramp_words`] for one lane of a bank.
    #[must_use]
    pub fn bank_trim_ramp_words(bank: &BuiltinInputBank, lane: usize) -> [u32; 8] {
        match &bank.stage {
            InputStageKernel::Simd4(stage) => stage.trim_ramp_words(lane),
            InputStageKernel::Simd8(stage) => stage.trim_ramp_words(lane),
        }
    }

    /// Overwrites the retained state words of one input chain.
    pub fn set_input_state_words(input: &mut InputBuiltins, words: [u32; 8]) {
        input.stage.set_lane_state_words(0, words);
    }

    /// Which sections of a chain the render path elides, `[channel][section]`, section `0` first.
    #[must_use]
    pub fn input_elision_plan(input: &InputBuiltins) -> [[bool; 2]; 2] {
        input.stage.elision_plan()
    }

    /// Which sections of a bank the render path elides, in the [`input_elision_plan`] order.
    #[must_use]
    pub fn bank_elision_plan(bank: &BuiltinInputBank) -> [[bool; 2]; 2] {
        match &bank.stage {
            InputStageKernel::Simd4(stage) => stage.elision_plan(),
            InputStageKernel::Simd8(stage) => stage.elision_plan(),
        }
    }

    /// Retained state words of one bank lane, in the [`input_state_words`] order.
    #[must_use]
    pub fn bank_lane_state_words(bank: &BuiltinInputBank, lane: usize) -> [u32; 8] {
        match &bank.stage {
            InputStageKernel::Simd4(stage) => stage.lane_state_words(lane),
            InputStageKernel::Simd8(stage) => stage.lane_state_words(lane),
        }
    }

    /// Cumulative per-channel recovered-lane counts of a bank, `[left, right]`.
    ///
    /// The one piece of a block's accounting that survives the call: `BuiltinProcessReport` is
    /// dropped by the graph adapter, so a collapsed body that fed only the left counter would be
    /// invisible everywhere else. `mono_collapse::the_collapsed_body_publishes_the_dual_bodys_report`
    /// is the gate.
    #[must_use]
    pub fn bank_lifetime_recovered(bank: &BuiltinInputBank) -> [u64; 2] {
        match &bank.stage {
            InputStageKernel::Simd4(stage) => stage.lifetime_recovered,
            InputStageKernel::Simd8(stage) => stage.lifetime_recovered,
        }
    }

    /// Overwrites the retained state words of one bank lane.
    pub fn set_bank_lane_state_words(bank: &mut BuiltinInputBank, lane: usize, words: [u32; 8]) {
        match &mut bank.stage {
            InputStageKernel::Simd4(stage) => stage.set_lane_state_words(lane, words),
            InputStageKernel::Simd8(stage) => stage.set_lane_state_words(lane, words),
        }
    }

    /// The current (applied) matrix of a scalar matrix section.
    #[must_use]
    pub fn matrix_current(matrix: &MatrixBuiltins) -> Matrix2x2 {
        matrix.stage.read_current()[0]
    }

    /// The input section of a chain, for state injection.
    pub fn chain_input_mut(chain: &mut BuiltinChain) -> &mut InputBuiltins {
        &mut chain.input
    }

    /// The input section of a chain.
    #[must_use]
    pub fn chain_input(chain: &BuiltinChain) -> &InputBuiltins {
        &chain.input
    }

    /// The matrix section of a chain.
    #[must_use]
    pub fn chain_matrix(chain: &BuiltinChain) -> &MatrixBuiltins {
        &chain.matrix
    }

    /// The matrix section of a chain, mutably.
    pub fn chain_matrix_mut(chain: &mut BuiltinChain) -> &mut MatrixBuiltins {
        &mut chain.matrix
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BuiltinChain, BuiltinLaneSelector, BuiltinParameters, BuiltinProcessReport,
        BuiltinResetKind, ChannelParameters, DualMonoBlock, Matrix2x2, test_support,
    };

    fn process_reference(
        chain: &mut BuiltinChain,
        left: &mut [f32],
        right: &mut [f32],
        first_sample: u64,
    ) -> BuiltinProcessReport {
        let report = chain.process_input(DualMonoBlock::new(left, right, first_sample).unwrap());
        chain.process_fader_mute(DualMonoBlock::new(left, right, first_sample).unwrap());
        chain.process_matrix(DualMonoBlock::new(left, right, first_sample).unwrap());
        report
    }

    fn assert_pair(
        dut: &mut BuiltinChain,
        reference: &mut BuiltinChain,
        mut left: Vec<f32>,
        mut right: Vec<f32>,
        first_sample: u64,
        fused: bool,
    ) {
        let mut old_left = left.clone();
        let mut old_right = right.clone();
        let before = dut.fused_dispatches;
        let report =
            dut.process_dual_mono(DualMonoBlock::new(&mut left, &mut right, first_sample).unwrap());
        let old_report = process_reference(reference, &mut old_left, &mut old_right, first_sample);
        assert_eq!(
            left.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            old_left.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
        assert_eq!(
            right.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            old_right.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
        assert_eq!(report, old_report);
        assert_eq!(
            test_support::input_state_words(test_support::chain_input(dut)),
            test_support::input_state_words(test_support::chain_input(reference)),
        );
        assert_eq!(
            test_support::matrix_current(test_support::chain_matrix(dut)),
            test_support::matrix_current(test_support::chain_matrix(reference)),
        );
        assert_eq!(
            dut.fused_dispatches - before,
            u32::from(fused),
            "selected path witness"
        );
    }

    fn set_matrix(
        dut: &mut BuiltinChain,
        reference: &mut BuiltinChain,
        target: Matrix2x2,
        samples: u32,
    ) {
        dut.matrix.set_target_smoothed(target, samples).unwrap();
        reference
            .matrix
            .set_target_smoothed(target, samples)
            .unwrap();
    }

    #[test]
    fn full_public_chain_matches_the_three_section_reference() {
        let enabled = BuiltinParameters {
            left: ChannelParameters {
                polarity_invert: true,
                trim_db: -3.0,
                hpf_hz: 80.0,
                lpf_hz: 16_000.0,
                fader_db: -4.0,
                muted: false,
            },
            right: ChannelParameters {
                polarity_invert: false,
                trim_db: 2.0,
                hpf_hz: 140.0,
                lpf_hz: 12_000.0,
                fader_db: -9.0,
                muted: true,
            },
            matrix: Matrix2x2 {
                ll: 0.75,
                lr: -0.2,
                rl: 0.35,
                rr: 0.9,
            },
            smoothing_samples: 0,
        };
        let mut dut = BuiltinChain::new(48_000, enabled).unwrap();
        let mut reference = BuiltinChain::new(48_000, enabled).unwrap();
        assert_pair(
            &mut dut,
            &mut reference,
            vec![
                0.25,
                -0.5,
                f32::NAN,
                0.75,
                f32::INFINITY,
                -0.0,
                0.125,
                -0.25,
            ],
            vec![-0.75, 0.5, 0.25, f32::NEG_INFINITY, 1.0, 0.0, -0.125, 0.375],
            0,
            true,
        );

        let bad = [f32::INFINITY.to_bits(); 8];
        test_support::set_input_state_words(test_support::chain_input_mut(&mut dut), bad);
        test_support::set_input_state_words(test_support::chain_input_mut(&mut reference), bad);
        assert_pair(
            &mut dut,
            &mut reference,
            vec![0.1, -0.2, 0.3, -0.4],
            vec![0.4, -0.3, 0.2, -0.1],
            8,
            true,
        );

        let disabled = BuiltinParameters {
            left: ChannelParameters {
                fader_db: -6.0,
                ..ChannelParameters::default()
            },
            right: ChannelParameters {
                polarity_invert: true,
                trim_db: 1.0,
                fader_db: 3.0,
                ..ChannelParameters::default()
            },
            matrix: Matrix2x2 {
                ll: 0.6,
                lr: 0.25,
                rl: -0.4,
                rr: 0.8,
            },
            smoothing_samples: 0,
        };
        let mut dut = BuiltinChain::new(96_000, disabled).unwrap();
        let mut reference = BuiltinChain::new(96_000, disabled).unwrap();
        assert_pair(
            &mut dut,
            &mut reference,
            vec![-0.0, 0.0, 0.5, -0.25, 0.75],
            vec![0.0, -0.0, -0.5, 0.25, -0.75],
            100,
            true,
        );
    }

    #[test]
    fn eligibility_sequence_uses_whole_call_fallback_then_fuses_the_next_call() {
        let parameters = BuiltinParameters::default();
        let mut dut = BuiltinChain::new(48_000, parameters).unwrap();
        let mut reference = BuiltinChain::new(48_000, parameters).unwrap();
        let a = Matrix2x2 {
            ll: 0.8,
            lr: 0.2,
            rl: -0.1,
            rr: 0.9,
        };
        let b = Matrix2x2 {
            ll: 0.5,
            lr: -0.3,
            rl: 0.4,
            rr: 0.7,
        };

        set_matrix(&mut dut, &mut reference, a, 0);
        assert_pair(
            &mut dut,
            &mut reference,
            vec![0.2; 3],
            vec![-0.4; 3],
            0,
            true,
        );

        set_matrix(&mut dut, &mut reference, b, 6);
        assert_pair(
            &mut dut,
            &mut reference,
            vec![0.3; 2],
            vec![-0.2; 2],
            3,
            false,
        );
        set_matrix(&mut dut, &mut reference, a, 5);
        assert_pair(
            &mut dut,
            &mut reference,
            vec![0.4; 2],
            vec![0.1; 2],
            5,
            false,
        );
        assert_pair(
            &mut dut,
            &mut reference,
            vec![-0.25; 7],
            vec![0.5; 7],
            7,
            false,
        );
        assert_pair(
            &mut dut,
            &mut reference,
            vec![0.75; 3],
            vec![-0.5; 3],
            14,
            true,
        );

        dut.input
            .set_trim_db(BuiltinLaneSelector::Left, -3.0, 4)
            .unwrap();
        reference
            .input
            .set_trim_db(BuiltinLaneSelector::Left, -3.0, 4)
            .unwrap();
        assert_pair(
            &mut dut,
            &mut reference,
            vec![0.1; 5],
            vec![0.2; 5],
            17,
            true,
        );

        set_matrix(&mut dut, &mut reference, b, 9);
        dut.reset(BuiltinResetKind::DiscontinuityKeepTargets);
        reference.reset(BuiltinResetKind::DiscontinuityKeepTargets);
        assert_pair(
            &mut dut,
            &mut reference,
            vec![0.6; 4],
            vec![-0.3; 4],
            22,
            true,
        );
    }
}
