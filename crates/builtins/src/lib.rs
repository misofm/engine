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

#[cfg(any(test, feature = "test-support"))]
use core::cell::Cell;
use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};

use engine::{
    SampleRateHz, is_launch_sample_rate,
    realtime::{Consumer, Producer, QueueGeneration, bounded_spsc},
};
pub mod corpus;
pub mod filter_control;
mod filter_response;
mod tail;
pub use filter_control::{
    INPUT_FILTER_RAMP_SAMPLES, InputFilterPair, PreparedInputFilterPair, PreparedInputFilterTarget,
    prepare_input_filter_pair, validate_input_filter_pair, validate_prepared_input_filter_target,
};
pub use filter_response::{
    InputFilterResponseError, InputFilterResponseMode, InputFilterResponseOutput,
    InputFilterResponseRequest, InputFilterResponseSummary, input_filter_response_descriptor,
    prepare_input_filter_response, query_input_filter_response_into,
    query_input_filter_snapshot_magnitudes_into,
};
use tail::CachedDesign;
pub use tail::{
    ChargedInputBound, INPUT_BOUND_BUDGET_FRAMES, INPUT_BOUND_CACHE_ENTRIES,
    INPUT_BOUND_DESIGN_CHARGE, INPUT_BOUND_SECTION_CHARGE, InputBoundCache, InputSectionBound,
    input_section_flush_law, input_section_live_bound, input_section_live_bound_table,
    input_section_live_cascade, input_section_live_envelope, input_section_worst_case_pair,
};

use effect_contract::{
    BankWidth, ChannelSymmetryWitness, EffectPrepareError, ResponseAnalysisError,
    ResponseSnapshotKind, ResponseSnapshotRequest, ResponseSnapshotSection,
    ResponseSnapshotSummary,
};
use lane::{
    Backend, Lane, Simd4,
    kernels::{
        SvfCoef,
        builtins::{
            GainMuteRamp, InputChainCoef, InputChainPlan, InputChainState, InputTrimRamp,
            Matrix2x2Coef, Matrix2x2Ramp, fader_matrix_block, fader_matrix_block_without_identity,
            gain_mute_block, gain_mute_ramp_block, input_chain_block_elided,
            input_chain_block_mono_elided, input_chain_plan, input_chain_ramp_block_elided,
            input_chain_ramp_block_filter, input_chain_ramp_block_filter_mono,
            input_chain_ramp_block_mono_elided, lanes_below, mask_from_flags, matrix2x2_block,
            matrix2x2_block_without_identity, matrix2x2_ramp_block, no_lanes,
            plan_is_channel_symmetric, section_is_identity, zero_lanes_block,
        },
    },
};

// The filter-ramp bodies find a ramp's leading updates from its countdown, exactly only for a
// countdown of at most `lane`'s ramp length (#1452 undo 1); a retarget writes this crate's.
const _: () = assert!(
    INPUT_FILTER_RAMP_SAMPLES == lane::kernels::builtins::INPUT_FILTER_RAMP_UPDATES,
    "the builtins' filter ramp length is the lane filter-ramp bodies' ramp length"
);

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

/// Return the schema unit for one builtin parameter.
///
/// This is the single authority shared by the persisted lattice adapter and the metadata
/// generator. The `delay_samples` name exception is intentional: its linear mapping stores a
/// sample count, while matrix/pan and the other linear builtin values use the unitless linear
/// vocabulary.
#[must_use]
pub fn builtin_parameter_unit(
    descriptor: &BuiltinParameterDescriptor,
) -> effect_contract::ParameterUnit {
    match descriptor.mapping {
        BuiltinParameterMapping::DecibelAmplitude => effect_contract::ParameterUnit::Db,
        BuiltinParameterMapping::Hertz => effect_contract::ParameterUnit::Hz,
        BuiltinParameterMapping::Boolean | BuiltinParameterMapping::Linear => {
            if descriptor.name == "delay_samples" {
                effect_contract::ParameterUnit::Samples
            } else {
                effect_contract::ParameterUnit::Linear
            }
        }
    }
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
/// Session compilation rejects unsupported rates before builtins preparation; this helper refuses
/// them too, since only a launch rate has a cutoff domain (owner ruling R5, #1036).
pub fn validate_builtin_filter_cutoff(
    value: f32,
    sample_rate: u32,
    disabled: f32,
    minimum_hz: f32,
) -> Result<(), BuiltinParameterError> {
    let Some(maximum_hz) = builtin_filter_cutoff_maximum_hz(sample_rate) else {
        return Err(BuiltinParameterError::FilterCutoff);
    };
    if value.to_bits() == disabled.to_bits()
        || (value.is_finite() && value >= minimum_hz && value <= maximum_hz)
    {
        Ok(())
    } else {
        Err(BuiltinParameterError::FilterCutoff)
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
    /// Fixed 64-sample linear updates of the prepared SVF coefficient words.
    Linear64CoefficientUpdates,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltinParameterReset {
    RestorePreparedValue,
    KeepTargetResetCurrent,
}

/// One builtin lattice at a prepared sample rate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuiltinLatticePoints {
    /// Sorted in-domain values. Their `index` is the persisted step index.
    pub points: Vec<effect_contract::LatticePoint>,
    /// Canonical disabled sentinel, when declared. It sits outside `points`.
    pub disabled: Option<String>,
}

/// Resolve a builtin descriptor into the shared lattice machinery.
///
/// The builtin vocabulary has a rate-keyed cutoff domain that effect descriptors do not. This
/// adapter supplies the selected rate's declared maximum, then delegates all arithmetic,
/// geometric rendering, intrinsic endpoints/defaults and index ordering to effect-contract's one
/// authority. The disabled sentinel stays outside the ordered domain.
pub fn builtin_parameter_lattice_points(
    descriptor: &BuiltinParameterDescriptor,
    sample_rate: u32,
) -> Result<BuiltinLatticePoints, effect_contract::LatticeError> {
    use effect_contract::{
        ParameterDomain, ParameterMapping, canonical_descriptor_decimal,
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
    let unit = builtin_parameter_unit(descriptor);
    let mapping = match descriptor.mapping {
        BuiltinParameterMapping::Boolean => ParameterMapping::Stepped,
        BuiltinParameterMapping::Hertz => ParameterMapping::Logarithmic,
        BuiltinParameterMapping::DecibelAmplitude | BuiltinParameterMapping::Linear => {
            ParameterMapping::Linear
        }
    };
    let points = parameter_lattice_points_parts(
        unit,
        domain,
        mapping,
        minimum,
        maximum,
        default_value,
        &[],
        descriptor.lattice,
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
        // Live through the prepared input-filter target path. The semantic cutoff remains a
        // target association value; render receives only the fixed 64-update coefficient ramp.
        update_rate: BuiltinParameterUpdateRate::BlockTarget,
        smoothing: BuiltinSmoothingPolicy::Linear64CoefficientUpdates,
        reset: BuiltinParameterReset::KeepTargetResetCurrent,
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
        update_rate: BuiltinParameterUpdateRate::BlockTarget,
        smoothing: BuiltinSmoothingPolicy::Linear64CoefficientUpdates,
        reset: BuiltinParameterReset::KeepTargetResetCurrent,
        disabled_value: Some(0.0),
        lattice: effect_contract::ParameterLattice::cents(20.0, 3),
    },
    // Issue #140 B: `fader_db` and `mute` become block targets with linear-N smoothing, because
    // the engine now has a post-preparation write path for them -- `FaderMuteRampBuiltins`,
    // bound by `LiveControlFaderProcessor` for a track live controls drive. This row states the
    // parameter's *capability*, exactly as `matrix_ll..rr` do: a session with no live controls has
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
        #[cfg(test)]
        FILTER_DESIGN_CALLS.with(|calls| calls.set(calls.get() + 1));
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
    /// `N_SILENCE` at the track's rate, `lane::silence_frames` (issue #1328, amendment A9): the
    /// input's run of zero frames that arms the joint flush. One rate per bank, so one value.
    pub silence_frames: u32,
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
    #[cfg(test)]
    CHANNEL_SYMMETRY_LANE_READS.with(|reads| {
        if reads.get() != usize::MAX {
            reads.set(reads.get() + 1);
        }
    });
    let mut words = [0.0_f32; MAX_BANK_LANES];
    value.store(&mut words[..L::WIDTH]);
    words
}

/// Widest bank this crate builds. `BankWidth` is four or eight (D4).
const MAX_BANK_LANES: usize = 8;

#[cfg(test)]
thread_local! {
    static FILTER_DESIGN_CALLS: Cell<usize> = const { Cell::new(0) };
    static CHANNEL_SYMMETRY_PREDICATE_CALLS: Cell<usize> = const { Cell::new(0) };
    static CHANNEL_SYMMETRY_LANE_READS: Cell<usize> = const { Cell::new(usize::MAX) };
    static CHANNEL_SYMMETRY_OBSERVE_POST_RAMP: Cell<bool> = const { Cell::new(false) };
    static CHANNEL_SYMMETRY_LAST_POST_RAMP_READS: Cell<usize> = const { Cell::new(usize::MAX) };
    static FILTER_PREFIX_KERNEL_FRAMES: Cell<usize> = const { Cell::new(usize::MAX) };
    /// Settled matrix blocks that took the select-free arm (issue #944), counted per call.
    static MATRIX_SELECT_FREE_BLOCKS: Cell<usize> = const { Cell::new(0) };
    /// Settled fused fader/matrix blocks that took the select-free arm (issue #954), counted per
    /// call.
    static FUSED_SELECT_FREE_BLOCKS: Cell<usize> = const { Cell::new(0) };
}

#[cfg(test)]
fn begin_post_ramp_observation() {
    CHANNEL_SYMMETRY_OBSERVE_POST_RAMP.with(|observe| {
        if observe.get() {
            CHANNEL_SYMMETRY_LANE_READS.with(|reads| reads.set(0));
        }
    });
}

#[cfg(test)]
fn end_post_ramp_observation() {
    CHANNEL_SYMMETRY_OBSERVE_POST_RAMP.with(|observe| {
        if observe.get() {
            observe.set(false);
            CHANNEL_SYMMETRY_LANE_READS.with(|reads| {
                CHANNEL_SYMMETRY_LAST_POST_RAMP_READS.with(|last| last.set(reads.get()));
                reads.set(usize::MAX);
            });
        }
    });
}

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

#[inline]
fn zero_svf_coef<L: Lane>() -> SvfCoef<L> {
    SvfCoef {
        c1: L::zero(),
        a2: L::zero(),
        a3: L::zero(),
        m0: L::zero(),
        m1: L::zero(),
        m2: L::zero(),
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
    /// Accepted current-independent filter targets and per-sample coefficient steps.
    filter_target: [[SvfCoef<L>; 2]; 2],
    filter_step: [[SvfCoef<L>; 2]; 2],
    /// The prepared endpoint used by an explicit full reset.
    filter_initial: [[SvfCoef<L>; 2]; 2],
    /// Authoritative per-lane fixed-64 countdowns, indexed `[channel][section][lane]`.
    filter_remaining: [[[u32; MAX_BANK_LANES]; 2]; 2],
    filter_ramping: bool,
    /// Retained integrator state, indexed like [`InputChainCoef::section`].
    state: InputChainState<L>,
    /// Cached permission to skip each section across the whole bank.
    ///
    /// `section_is_identity` reads the six coefficient words and two integrators of every lane.
    /// Trim is not part of that predicate, so trim/polarity retargets cannot invalidate it.
    /// Filter targets arrive as prepared words; the callback never invokes the designer.
    ///
    /// `refresh_filter_plan` recomputes the predicate and forces every in-flight section false,
    /// including an enable whose current words are still identity. Retarget, ramp completion,
    /// disabled endpoint clearing, reset and explicit state restoration use this authority.
    /// A settled elided section keeps its integrators untouched; boundary recovery can only move
    /// them toward +0, so it cannot invalidate an elidable section. The required all-identity
    /// trim-ramp path preserves sanitization, trim timing and signed-zero normalization.
    /// See `docs/rulings/builtins-input-liveness-d2.md`.
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
    /// Whether the last block this stage rendered was collapsed ([`InputStage::process_mono`])
    /// and no disengage ([`InputStage::desymmetrize`]) has run since.
    ///
    /// # The collapsed-stage invariant (#1407)
    ///
    /// While a stage is collapsed, channel `0` is the only live state: the one-plane body advances
    /// channel `0`'s integrators and leaves channel `1`'s frozen at the values they held when the
    /// collapse engaged, and only the disengage copy repairs them. So no decision may read
    /// channel `1`'s state while this is set, and every `Both` record applies channel `0`'s
    /// decision to both channels. The reads that would otherwise see the frozen words go through
    /// [`InputStage::live_state_channel`]: the live filter retarget's rule-3 predicate and the
    /// elision plan. The lane export and `channels_agree` also read channel `1`'s integrators, and
    /// neither is reached collapsed: a carry disengages first (`disengage_for_carry`), and the M3
    /// proof is asked only of a chain rendering dual. A dual block after a collapsed one without
    /// the disengage copy is a debug assertion in [`InputStage::process`]. The coefficient,
    /// target, step and countdown records need no such redirection, because `process_mono`
    /// mirrors them onto channel `1` at the bottom of every collapsed block, so at a drain they
    /// are what the dual run would hold.
    ///
    /// One byte beside `ramping` and `symmetry`, in padding the struct already had: no sealed
    /// size moves.
    collapsed: bool,
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
        debug_assert!(
            tracks
                .iter()
                .all(|track| track.silence_frames == tracks[0].silence_frames),
            "a bank runs at one rate"
        );
        let coef = InputChainCoef {
            trim: [lane_words::<L>(&trim[0]), lane_words::<L>(&trim[1])],
            section: [
                [svf_coef::<L>(&sections[0]), svf_coef::<L>(&sections[1])],
                [svf_coef::<L>(&sections[2]), svf_coef::<L>(&sections[3])],
            ],
            silence: L::splat(tracks[0].silence_frames as f32),
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
            filter_target: coef.section,
            filter_step: [[zero_svf_coef::<L>(); 2]; 2],
            filter_initial: coef.section,
            filter_remaining: [[[0; MAX_BANK_LANES]; 2]; 2],
            filter_ramping: false,
            state,
            plan,
            ramp,
            remaining: [[0; MAX_BANK_LANES]; 2],
            ramping: false,
            collapsed: false,
            symmetry: 0,
            lifetime_recovered: [0; 2],
        };
        stage.refresh_channel_symmetry();
        stage
    }

    /// Retakes every lane's channel-symmetry comparison into [`InputStage::symmetry`].
    ///
    /// Called from preparation, both ramping process arms, and reset. The live retarget writer
    /// updates only its addressed lane below; neither path runs from dispatch.
    fn refresh_channel_symmetry(&mut self) {
        let mut symmetry = 0_u8;
        for lane in 0..MAX_BANK_LANES {
            if self.compute_lane_channel_symmetry(lane) {
                symmetry |= 1 << lane;
            }
        }
        self.symmetry = symmetry;
    }

    /// Refreshes the post-ramp channel mask by extracting each compared SIMD word once.
    ///
    /// The per-lane predicate remains the definition and the debug oracle. This path only
    /// changes the traversal used after a ramp block: it keeps a candidate mask while walking the
    /// fifteen compared word pairs, so a full W8 bank materializes thirty lane words rather than
    /// materializing the same words once for every lane.
    fn refresh_channel_symmetry_post_ramp(&mut self) {
        let member_mask = if self.members == u8::BITS as usize {
            u8::MAX
        } else {
            (1_u8 << self.members) - 1
        };
        let width_mask = if L::WIDTH == u8::BITS as usize {
            u8::MAX
        } else {
            (1_u8 << L::WIDTH) - 1
        };
        let mut candidate = member_mask & width_mask;
        for (left_word, right_word) in [
            (self.coef.trim[0], self.coef.trim[1]),
            (self.ramp.target[0], self.ramp.target[1]),
            (self.ramp.step[0], self.ramp.step[1]),
        ] {
            if candidate == 0 {
                break;
            }
            let left = lane_read::<L>(left_word);
            let right = lane_read::<L>(right_word);
            for lane in 0..L::WIDTH {
                if candidate & (1 << lane) != 0 && left[lane].to_bits() != right[lane].to_bits() {
                    candidate &= !(1 << lane);
                }
            }
        }
        if candidate != 0 {
            for lane in 0..L::WIDTH {
                if candidate & (1 << lane) != 0
                    && self.remaining[0][lane] != self.remaining[1][lane]
                {
                    candidate &= !(1 << lane);
                }
            }
        }
        for section in 0..2 {
            for (left_word, right_word) in [
                (
                    self.coef.section[0][section].c1,
                    self.coef.section[1][section].c1,
                ),
                (
                    self.coef.section[0][section].a2,
                    self.coef.section[1][section].a2,
                ),
                (
                    self.coef.section[0][section].a3,
                    self.coef.section[1][section].a3,
                ),
                (
                    self.coef.section[0][section].m0,
                    self.coef.section[1][section].m0,
                ),
                (
                    self.coef.section[0][section].m1,
                    self.coef.section[1][section].m1,
                ),
                (
                    self.coef.section[0][section].m2,
                    self.coef.section[1][section].m2,
                ),
            ] {
                if candidate == 0 {
                    break;
                }
                let left = lane_read::<L>(left_word);
                let right = lane_read::<L>(right_word);
                for lane in 0..L::WIDTH {
                    if candidate & (1 << lane) != 0 && left[lane].to_bits() != right[lane].to_bits()
                    {
                        candidate &= !(1 << lane);
                    }
                }
            }
        }
        self.symmetry = candidate;
    }

    /// Filter-aware post-ramp refresh. This retains the trim words and adds every live filter
    /// target/step/countdown word, so both trim-only and filter-prefix paths publish one complete
    /// channel-symmetry witness.
    fn refresh_filter_channel_symmetry_post_ramp(&mut self) {
        self.refresh_channel_symmetry_post_ramp();
        let mut candidate = self.symmetry;
        for section in 0..2 {
            for (left_word, right_word) in [
                (
                    self.filter_target[0][section].c1,
                    self.filter_target[1][section].c1,
                ),
                (
                    self.filter_target[0][section].a2,
                    self.filter_target[1][section].a2,
                ),
                (
                    self.filter_target[0][section].a3,
                    self.filter_target[1][section].a3,
                ),
                (
                    self.filter_target[0][section].m0,
                    self.filter_target[1][section].m0,
                ),
                (
                    self.filter_target[0][section].m1,
                    self.filter_target[1][section].m1,
                ),
                (
                    self.filter_target[0][section].m2,
                    self.filter_target[1][section].m2,
                ),
                (
                    self.filter_step[0][section].c1,
                    self.filter_step[1][section].c1,
                ),
                (
                    self.filter_step[0][section].a2,
                    self.filter_step[1][section].a2,
                ),
                (
                    self.filter_step[0][section].a3,
                    self.filter_step[1][section].a3,
                ),
                (
                    self.filter_step[0][section].m0,
                    self.filter_step[1][section].m0,
                ),
                (
                    self.filter_step[0][section].m1,
                    self.filter_step[1][section].m1,
                ),
                (
                    self.filter_step[0][section].m2,
                    self.filter_step[1][section].m2,
                ),
            ] {
                let left = lane_read::<L>(left_word);
                let right = lane_read::<L>(right_word);
                for lane in 0..L::WIDTH {
                    if candidate & (1 << lane) != 0 && left[lane].to_bits() != right[lane].to_bits()
                    {
                        candidate &= !(1 << lane);
                    }
                }
            }
            for lane in 0..L::WIDTH {
                if candidate & (1 << lane) != 0
                    && self.filter_remaining[0][section][lane]
                        != self.filter_remaining[1][section][lane]
                {
                    candidate &= !(1 << lane);
                }
            }
        }
        self.symmetry = candidate;
    }

    /// Updates only the addressed lane after a live trim/polarity retarget.
    fn refresh_channel_symmetry_lane(&mut self, lane: usize) {
        let bit = 1_u8 << lane;
        if self.compute_lane_channel_symmetry(lane) {
            self.symmetry |= bit;
        } else {
            self.symmetry &= !bit;
        }
    }

    /// Recompute the elision plan while a filter target is in flight.  An in-flight section is
    /// conservative even when its current words still happen to be identity words: the target
    /// is already accepted and the ramp body must execute it.
    ///
    /// While the stage is collapsed, channel `1`'s sections are decided over channel `0`'s
    /// integrators (the collapsed-stage invariant, [`InputStage::collapsed`]): a disable that
    /// completes collapsed clears channel `0`'s integrators only, and deciding channel `1` over
    /// its frozen words would split the plan between the channels.
    fn refresh_filter_plan(&mut self) {
        let mut plan = input_chain_plan::<L>(&self.coef, &self.state);
        if self.collapsed {
            for section in 0..2 {
                plan.elided[1][section] = section_is_identity::<L>(
                    &self.coef.section[1][section],
                    &self.state.section[0][section],
                );
            }
        }
        for channel in 0..2 {
            for section in 0..2 {
                if self.filter_remaining[channel][section]
                    .iter()
                    .take(self.members)
                    .any(|remaining| *remaining != 0)
                {
                    plan.elided[channel][section] = false;
                }
            }
        }
        self.plan = plan;
    }

    /// The channel whose integrators stand for `channel`'s: channel `0` while the stage is
    /// collapsed, `channel` itself otherwise. See [`InputStage::collapsed`].
    const fn live_state_channel(&self, channel: usize) -> usize {
        if self.collapsed { 0 } else { channel }
    }

    fn load_filter_countdown(&self) -> [[L; 2]; 2] {
        let mut words = [[[0.0_f32; MAX_BANK_LANES]; 2]; 2];
        for (channel, channel_words) in words.iter_mut().enumerate() {
            for (section, section_words) in channel_words.iter_mut().enumerate() {
                // A retarget writes `INPUT_FILTER_RAMP_SAMPLES` or zero, a settle only lowers the
                // countdown and a lane import takes what an export produced, so no countdown
                // exceeds the ramp length, which the bodies' leading window relies on (#1452).
                debug_assert!(
                    self.filter_remaining[channel][section]
                        .iter()
                        .all(|&remaining| remaining <= INPUT_FILTER_RAMP_SAMPLES),
                    "a filter countdown exceeds the ramp length"
                );
                for (lane, word) in section_words.iter_mut().enumerate() {
                    *word = self.filter_remaining[channel][section][lane]
                        .min(Self::RAMP_COUNTDOWN_MAXIMUM) as f32;
                }
            }
        }
        core::array::from_fn(|channel| {
            core::array::from_fn(|section| lane_words::<L>(&words[channel][section]))
        })
    }

    /// Bound the combined filter body to the countdown that still exists in its processed lanes.
    /// A settled section must enter the elided suffix immediately; the fixed policy's 64-sample
    /// ceiling is only a bound for an active countdown, not work every prefix must perform.
    fn filter_prefix_frames(&self, frames: usize, channels: core::ops::Range<usize>) -> usize {
        let mut remaining = 0_u32;
        for channel in channels {
            for section in 0..2 {
                for lane in 0..self.members {
                    remaining = remaining.max(self.filter_remaining[channel][section][lane]);
                }
            }
        }
        let remaining = remaining.min(INPUT_FILTER_RAMP_SAMPLES);
        frames.min(usize::try_from(remaining).unwrap_or(usize::MAX))
    }

    fn settle_filter(&mut self, frames: usize, channels: core::ops::Range<usize>) {
        let frames = u32::try_from(frames).unwrap_or(u32::MAX);
        for channel in channels {
            for section in 0..2 {
                let mut current = self.coef.section[channel][section];
                let target = self.filter_target[channel][section];
                let mut remaining = self.filter_remaining[channel][section];
                remaining = remaining.map(|value| value.saturating_sub(frames));
                let settle_words = |value: L, target: L, remaining: &[u32; MAX_BANK_LANES]| {
                    let mut current_words = lane_read::<L>(value);
                    let target_words = lane_read::<L>(target);
                    for lane in 0..L::WIDTH {
                        if remaining[lane] == 0 {
                            current_words[lane] = target_words[lane];
                        }
                    }
                    lane_words::<L>(&current_words)
                };
                current.c1 = settle_words(current.c1, target.c1, &remaining);
                current.a2 = settle_words(current.a2, target.a2, &remaining);
                current.a3 = settle_words(current.a3, target.a3, &remaining);
                current.m0 = settle_words(current.m0, target.m0, &remaining);
                current.m1 = settle_words(current.m1, target.m1, &remaining);
                current.m2 = settle_words(current.m2, target.m2, &remaining);
                self.filter_remaining[channel][section] = remaining;
                self.coef.section[channel][section] = current;
            }
        }
        self.filter_ramping = (0..2).any(|channel| {
            (0..2).any(|section| {
                self.filter_remaining[channel][section]
                    .iter()
                    .take(self.members)
                    .any(|remaining| *remaining != 0)
            })
        });
    }

    /// Apply one already-validated fixed-size target to one section and one or both channels.
    ///
    /// The live retarget law (#1407, decision 15 D15-4(b)) keeps every recursion word
    /// `[c1, a2, a3]` the kernel can load a designed filter, the disabled identity at rest, or a
    /// linear mixture of designs. Each covered channel decides its rule from its own words, so a
    /// lane whose channels hold equal words keeps equal words, steps included:
    ///
    /// 1. **In-flight re-send.** A target whose six words equal, bit for bit, the channel's
    ///    in-flight target on a lane whose countdown is non-zero leaves the lane untouched:
    ///    current, target, step and countdown keep their bits. A settled lane (countdown zero)
    ///    never takes this rule.
    /// 2. **Disable.** The identity target `[0, 0, 0, 1, 0, 0]` on a lane whose current words are
    ///    not already the identity freezes `c1`, `a2`, `a3` (step `+0.0`), ramps only `m0`, `m1`,
    ///    `m2` toward the identity mix and starts the 64-update countdown: a crossfade from the
    ///    filtered output, the filter at its own words, to the dry input. The kernel's completion
    ///    snap then writes the identity recursion and clears the integrators in the same step.
    /// 3. **Enable from rest.** A design target on a *settled disabled* lane -- countdown zero,
    ///    all six current words bitwise the identity and both integrators `+0.0` -- first writes
    ///    the target's `c1`, `a2`, `a3` into the current words and then ramps only the mix: the
    ///    reverse crossfade. An identity section holding restored non-zero integrators takes
    ///    rule 4, so that state is never released through a jumped recursion.
    /// 4. Every other retarget ramps all six words from the current words to the target over 64
    ///    updates, as before.
    ///
    /// Under any re-send history, the first sample uses the current words and sample A+64 uses
    /// the exact target. See `docs/rulings/builtins-input-liveness-d2.md`.
    fn apply_prepared_filter(&mut self, lane: usize, target: PreparedInputFilterTarget) {
        const IDENTITY: [f32; 6] = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0];
        debug_assert!(target.section < 2);
        debug_assert!(lane < self.members);
        let section = target.section as usize;
        let target_words = target.coefficients;
        let bit_equal = |left: &[f32; 6], right: &[f32; 6]| {
            left.iter()
                .zip(right)
                .all(|(left, right)| left.to_bits() == right.to_bits())
        };
        let target_is_identity = bit_equal(&target_words, &IDENTITY);
        for channel in 0..2 {
            if !target.lanes.covers(channel) {
                continue;
            }
            let mut current = self.coef.section[channel][section];
            let mut target_values = [
                lane_read::<L>(self.filter_target[channel][section].c1),
                lane_read::<L>(self.filter_target[channel][section].a2),
                lane_read::<L>(self.filter_target[channel][section].a3),
                lane_read::<L>(self.filter_target[channel][section].m0),
                lane_read::<L>(self.filter_target[channel][section].m1),
                lane_read::<L>(self.filter_target[channel][section].m2),
            ];
            let mut remaining = self.filter_remaining[channel][section];
            let in_flight_target: [f32; 6] =
                core::array::from_fn(|index| target_values[index][lane]);
            // Rule 1: an identical re-send never restarts a ramp in flight.
            if remaining[lane] != 0 && bit_equal(&in_flight_target, &target_words) {
                continue;
            }
            let mut current_values = [
                lane_read::<L>(current.c1),
                lane_read::<L>(current.a2),
                lane_read::<L>(current.a3),
                lane_read::<L>(current.m0),
                lane_read::<L>(current.m1),
                lane_read::<L>(current.m2),
            ];
            let current_words: [f32; 6] = core::array::from_fn(|index| current_values[index][lane]);
            let current_is_identity = bit_equal(&current_words, &IDENTITY);
            // The collapsed-stage invariant: while collapsed, channel `1`'s integrators are frozen
            // and channel `0`'s stand for both, so both channels decide over channel `0`'s.
            let state = self.state.section[self.live_state_channel(channel)][section];
            let settled_disabled = remaining[lane] == 0
                && current_is_identity
                && lane_read::<L>(state.ic1)[lane].to_bits() == 0
                && lane_read::<L>(state.ic2)[lane].to_bits() == 0;
            // Rule 3: an enable from rest jumps the recursion; only the mix ramps.
            if !target_is_identity && settled_disabled {
                for index in 0..3 {
                    current_values[index][lane] = target_words[index];
                }
                current.c1 = lane_words::<L>(&current_values[0]);
                current.a2 = lane_words::<L>(&current_values[1]);
                current.a3 = lane_words::<L>(&current_values[2]);
                self.coef.section[channel][section] = current;
            }
            // Rule 2: a disable freezes the recursion; only the mix ramps.
            let freeze_recursion = target_is_identity && !current_is_identity;
            let mut step_values = [
                lane_read::<L>(self.filter_step[channel][section].c1),
                lane_read::<L>(self.filter_step[channel][section].a2),
                lane_read::<L>(self.filter_step[channel][section].a3),
                lane_read::<L>(self.filter_step[channel][section].m0),
                lane_read::<L>(self.filter_step[channel][section].m1),
                lane_read::<L>(self.filter_step[channel][section].m2),
            ];
            let mut changed = freeze_recursion;
            for index in 0..6 {
                let current_word = current_values[index][lane];
                target_values[index][lane] = target_words[index];
                if index < 3 && freeze_recursion {
                    step_values[index][lane] = 0.0;
                } else if current_word.to_bits() != target_words[index].to_bits() {
                    step_values[index][lane] = (target_words[index] - current_word) * (1.0 / 64.0);
                    changed = true;
                } else {
                    step_values[index][lane] = 0.0;
                }
            }
            remaining[lane] = if changed {
                INPUT_FILTER_RAMP_SAMPLES
            } else {
                0
            };
            self.filter_target[channel][section] = SvfCoef {
                c1: lane_words::<L>(&target_values[0]),
                a2: lane_words::<L>(&target_values[1]),
                a3: lane_words::<L>(&target_values[2]),
                m0: lane_words::<L>(&target_values[3]),
                m1: lane_words::<L>(&target_values[4]),
                m2: lane_words::<L>(&target_values[5]),
            };
            self.filter_step[channel][section] = SvfCoef {
                c1: lane_words::<L>(&step_values[0]),
                a2: lane_words::<L>(&step_values[1]),
                a3: lane_words::<L>(&step_values[2]),
                m0: lane_words::<L>(&step_values[3]),
                m1: lane_words::<L>(&step_values[4]),
                m2: lane_words::<L>(&step_values[5]),
            };
            self.filter_remaining[channel][section] = remaining;
        }
        self.filter_ramping = (0..2).any(|channel| {
            (0..2).any(|section| {
                self.filter_remaining[channel][section]
                    .iter()
                    .take(self.members)
                    .any(|remaining| *remaining != 0)
            })
        });
        self.refresh_filter_plan();
        self.refresh_channel_symmetry();
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
        self.refresh_channel_symmetry_lane(lane);
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

    fn mirror_filter_ramp(&mut self) {
        self.coef.section[1] = self.coef.section[0];
        self.filter_target[1] = self.filter_target[0];
        self.filter_step[1] = self.filter_step[0];
        self.filter_remaining[1] = self.filter_remaining[0];
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
        debug_assert!(
            !self.collapsed,
            "a dual block after a collapsed one needs the disengage copy first (desymmetrize)"
        );
        let report = if self.filter_ramping {
            let prefix = self.filter_prefix_frames(frames, 0..2);
            if self.ramping {
                self.load_countdown();
            }
            let mut filter_remaining = self.load_filter_countdown();
            #[cfg(test)]
            FILTER_PREFIX_KERNEL_FRAMES.with(|observed| observed.set(prefix));
            let mut report = input_chain_ramp_block_filter::<L>(
                &mut left[..prefix * L::WIDTH],
                &mut right[..prefix * L::WIDTH],
                prefix,
                &mut self.coef,
                &mut self.state,
                &mut self.ramp,
                self.ramping,
                &self.filter_target,
                &self.filter_step,
                &mut filter_remaining,
            );
            if self.ramping {
                self.settle(prefix, 0..2);
            }
            self.settle_filter(prefix, 0..2);
            self.refresh_filter_plan();
            if prefix < frames {
                let suffix_frames = frames - prefix;
                let suffix = if self.ramping {
                    self.load_countdown();
                    let suffix = input_chain_ramp_block_elided::<L>(
                        &mut left[prefix * L::WIDTH..],
                        &mut right[prefix * L::WIDTH..],
                        suffix_frames,
                        &self.coef,
                        &mut self.state,
                        &mut self.ramp,
                        &self.plan,
                    );
                    self.settle(suffix_frames, 0..2);
                    suffix
                } else {
                    input_chain_block_elided::<L>(
                        &mut left[prefix * L::WIDTH..],
                        &mut right[prefix * L::WIDTH..],
                        suffix_frames,
                        &self.coef,
                        &mut self.state,
                        &self.plan,
                    )
                };
                report.sanitized[0] = report.sanitized[0].add(suffix.sanitized[0]);
                report.sanitized[1] = report.sanitized[1].add(suffix.sanitized[1]);
                report.nonfinite[0] = L::mask_or(report.nonfinite[0], suffix.nonfinite[0]);
                report.nonfinite[1] = L::mask_or(report.nonfinite[1], suffix.nonfinite[1]);
            }
            #[cfg(test)]
            begin_post_ramp_observation();
            self.refresh_filter_channel_symmetry_post_ramp();
            #[cfg(test)]
            end_post_ramp_observation();
            report
        } else if self.ramping {
            self.load_countdown();
            let report = input_chain_ramp_block_elided::<L>(
                left,
                right,
                frames,
                &self.coef,
                &mut self.state,
                &mut self.ramp,
                &self.plan,
            );
            self.settle(frames, 0..2);
            #[cfg(test)]
            begin_post_ramp_observation();
            self.refresh_filter_channel_symmetry_post_ramp();
            #[cfg(test)]
            end_post_ramp_observation();
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
        // From here until the disengage copy, channel `0` is the only live state.
        self.collapsed = true;
        let report = if self.filter_ramping {
            let prefix = self.filter_prefix_frames(frames, 0..1);
            if self.ramping {
                self.load_countdown();
            }
            let mut filter_remaining = self.load_filter_countdown();
            #[cfg(test)]
            FILTER_PREFIX_KERNEL_FRAMES.with(|observed| observed.set(prefix));
            let mut report = input_chain_ramp_block_filter_mono::<L>(
                &mut left[..prefix * L::WIDTH],
                prefix,
                &mut self.coef,
                &mut self.state,
                &mut self.ramp,
                self.ramping,
                &self.filter_target,
                &self.filter_step,
                &mut filter_remaining,
            );
            if self.ramping {
                self.settle(prefix, 0..1);
            }
            self.settle_filter(prefix, 0..1);
            self.mirror_filter_ramp();
            self.refresh_filter_plan();
            if prefix < frames {
                let suffix_frames = frames - prefix;
                let suffix = if self.ramping {
                    self.load_countdown();
                    let suffix = input_chain_ramp_block_mono_elided::<L>(
                        &mut left[prefix * L::WIDTH..],
                        suffix_frames,
                        &self.coef,
                        &mut self.state,
                        &mut self.ramp,
                        &self.plan,
                    );
                    self.settle(suffix_frames, 0..1);
                    suffix
                } else {
                    input_chain_block_mono_elided::<L>(
                        &mut left[prefix * L::WIDTH..],
                        suffix_frames,
                        &self.coef,
                        &mut self.state,
                        &self.plan,
                    )
                };
                report.sanitized[0] = report.sanitized[0].add(suffix.sanitized[0]);
                report.sanitized[1] = report.sanitized[1].add(suffix.sanitized[1]);
                report.nonfinite[0] = L::mask_or(report.nonfinite[0], suffix.nonfinite[0]);
                report.nonfinite[1] = L::mask_or(report.nonfinite[1], suffix.nonfinite[1]);
            }
            self.mirror_filter_ramp();
            self.mirror_trim_ramp();
            #[cfg(test)]
            begin_post_ramp_observation();
            self.refresh_filter_channel_symmetry_post_ramp();
            #[cfg(test)]
            end_post_ramp_observation();
            report
        } else if self.ramping {
            self.load_countdown();
            let report = input_chain_ramp_block_mono_elided::<L>(
                left,
                frames,
                &self.coef,
                &mut self.state,
                &mut self.ramp,
                &self.plan,
            );
            // Channel `0` only: the right channel's ramp is not advanced by the one-plane kernel,
            // so it is settled from the left channel's countdown and then duplicated, exactly as
            // the report below is. A collapsed track's right channel *is* its left channel here
            // too, and freezing the right ramp instead would leave the disengage boundary with a
            // per-channel word to repair that no `desymmetrize` copy can reconstruct once the two
            // countdowns have diverged.
            self.settle(frames, 0..1);
            self.mirror_trim_ramp();
            #[cfg(test)]
            begin_post_ramp_observation();
            self.refresh_filter_channel_symmetry_post_ramp();
            #[cfg(test)]
            end_post_ramp_observation();
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
    /// live controls addressed to one lane ramps both -- and, because `LIVE` is a latch, the chain
    /// never collapses again and the right channel never recovers. The integrators have no such
    /// window: nothing in the drain writes them.
    ///
    /// The rule this leaves is narrower than the one it replaces and is the true one: **a stage
    /// restores at the disengage boundary exactly the per-channel state its one-plane body
    /// froze.** `process_mono` froze the integrators. It did not freeze the ramp; it mirrored it.
    fn desymmetrize(&mut self) {
        self.state.section[1] = self.state.section[0];
        // The input's silence counter is per-channel state the one-plane body froze with the
        // integrators (issue #1328, amendment A9): channel `0`'s is the live one.
        self.state.silence[1] = self.state.silence[0];
        self.collapsed = false;
        self.refresh_filter_plan();
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
    /// [`InputStage::desymmetrize`] copies -- the four integrators per channel, the input's silence
    /// counter (issue #1328, amendment A9) and the trim ramp record -- because those are the whole
    /// of this kernel's per-channel state, and a `true` that
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
        if !self.trim_ramp_channels_agree() {
            return false;
        }
        // The input's silence counters (issue #1328, amendment A9): a collapsed block advances
        // channel `0`'s for both, so the two must already be equal.
        let (left_run, right_run) = (
            lane_read::<L>(self.state.silence[0]),
            lane_read::<L>(self.state.silence[1]),
        );
        for lane in 0..L::WIDTH {
            if left_run[lane].to_bits() != right_run[lane].to_bits() {
                return false;
            }
        }
        for section in 0..2 {
            for (left_word, right_word) in [
                (
                    self.coef.section[0][section].c1,
                    self.coef.section[1][section].c1,
                ),
                (
                    self.coef.section[0][section].a2,
                    self.coef.section[1][section].a2,
                ),
                (
                    self.coef.section[0][section].a3,
                    self.coef.section[1][section].a3,
                ),
                (
                    self.coef.section[0][section].m0,
                    self.coef.section[1][section].m0,
                ),
                (
                    self.coef.section[0][section].m1,
                    self.coef.section[1][section].m1,
                ),
                (
                    self.coef.section[0][section].m2,
                    self.coef.section[1][section].m2,
                ),
                (
                    self.filter_target[0][section].c1,
                    self.filter_target[1][section].c1,
                ),
                (
                    self.filter_target[0][section].a2,
                    self.filter_target[1][section].a2,
                ),
                (
                    self.filter_target[0][section].a3,
                    self.filter_target[1][section].a3,
                ),
                (
                    self.filter_target[0][section].m0,
                    self.filter_target[1][section].m0,
                ),
                (
                    self.filter_target[0][section].m1,
                    self.filter_target[1][section].m1,
                ),
                (
                    self.filter_target[0][section].m2,
                    self.filter_target[1][section].m2,
                ),
                (
                    self.filter_step[0][section].c1,
                    self.filter_step[1][section].c1,
                ),
                (
                    self.filter_step[0][section].a2,
                    self.filter_step[1][section].a2,
                ),
                (
                    self.filter_step[0][section].a3,
                    self.filter_step[1][section].a3,
                ),
                (
                    self.filter_step[0][section].m0,
                    self.filter_step[1][section].m0,
                ),
                (
                    self.filter_step[0][section].m1,
                    self.filter_step[1][section].m1,
                ),
                (
                    self.filter_step[0][section].m2,
                    self.filter_step[1][section].m2,
                ),
            ] {
                let left = lane_read::<L>(left_word);
                let right = lane_read::<L>(right_word);
                for lane in 0..self.members {
                    if left[lane].to_bits() != right[lane].to_bits() {
                        return false;
                    }
                }
            }
            if self.filter_remaining[0][section][..self.members]
                != self.filter_remaining[1][section][..self.members]
            {
                return false;
            }
        }
        true
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
        self.reset_with_kind(BuiltinResetKind::DiscontinuityKeepTargets);
    }

    fn reset_with_kind(&mut self, kind: BuiltinResetKind) {
        self.state = InputChainState::default();
        // Snap every lane to its target and cancel any ramp in flight, exactly as
        // `FaderRampStage::reset` and `MatrixStage::reset` do: a reset is a state reset, and a
        // half-finished ramp is state. The *target* is kept, because `trim_db` and
        // `polarity_invert` declare `BuiltinParameterReset::RestorePreparedValue` for the
        // **prepared** value and the live target is what live controls last asked for -- the same
        // reading `fader_db` has had since #140 B.
        self.ramp.current = self.ramp.target;
        self.ramp.step = [L::zero(); 2];
        self.ramp.remaining = [L::zero(); 2];
        self.remaining = [[0; MAX_BANK_LANES]; 2];
        self.ramping = false;
        self.coef.trim = self.ramp.current;
        if matches!(kind, BuiltinResetKind::FullToPrepared) {
            self.filter_target = self.filter_initial;
        }
        self.coef.section = if matches!(kind, BuiltinResetKind::FullToPrepared) {
            self.filter_initial
        } else {
            self.filter_target
        };
        self.filter_step = [[zero_svf_coef::<L>(); 2]; 2];
        self.filter_remaining = [[[0; MAX_BANK_LANES]; 2]; 2];
        self.filter_ramping = false;
        self.refresh_filter_plan();
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
        self.lane_track_from_sections(lane, &self.coef.section)
    }

    fn target_lane_track(&self, lane: usize) -> PreparedInputTrack {
        self.lane_track_from_sections(lane, &self.filter_target)
    }

    fn lane_track_from_sections(
        &self,
        lane: usize,
        sections: &[[SvfCoef<L>; 2]; 2],
    ) -> PreparedInputTrack {
        let trim = self.coef.trim.map(|trim| lane_read::<L>(trim)[lane]);
        let section = |channel: usize, index: usize| -> SvfSection {
            let coef = &sections[channel][index];
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
            silence_frames: lane_read::<L>(self.coef.silence)[lane] as u32,
        }
    }

    /// Copies one lane's retained HPF/LPF words into the live response transfer record.
    ///
    /// The sections are recovered from the coefficient words the render kernel loads. Trim and
    /// polarity are deliberately absent because they do not belong to this response subtotal.
    fn copy_response_snapshot_lane(
        &self,
        lane: usize,
        sample_rate_hz: u32,
        request: ResponseSnapshotRequest<'_>,
    ) -> Result<ResponseSnapshotSummary, ResponseAnalysisError> {
        if lane >= self.members || lane >= L::WIDTH {
            return Err(ResponseAnalysisError::Capacity);
        }
        if request.left.len() != 2 || request.right.len() != 2 {
            return Err(ResponseAnalysisError::OutputShape);
        }
        if !is_launch_sample_rate(SampleRateHz(sample_rate_hz)) {
            return Err(ResponseAnalysisError::Configuration(EffectPrepareError {
                code: "effect.quality.unsupported",
            }));
        }
        let bypassed = request.bypassed;
        let left = request.left;
        let right = request.right;
        let track = self.target_lane_track(lane);
        let left_sections = [track.left.hpf, track.left.lpf];
        let right_sections = [track.right.hpf, track.right.lpf];
        for (index, (left_section, right_section)) in
            left_sections.into_iter().zip(right_sections).enumerate()
        {
            left[index] = ResponseSnapshotSection {
                id: u32::try_from(index + 1).expect("builtin section count fits u32"),
                kind: u32::try_from(index + 1).expect("builtin section kind fits u32"),
                enabled: left_section.enabled,
                word_count: 7,
                words: left_section.words(),
            };
            right[index] = ResponseSnapshotSection {
                id: u32::try_from(index + 1).expect("builtin section count fits u32"),
                kind: u32::try_from(index + 1).expect("builtin section kind fits u32"),
                enabled: right_section.enabled,
                word_count: 7,
                words: right_section.words(),
            };
        }
        Ok(ResponseSnapshotSummary {
            kind: ResponseSnapshotKind::BuiltinInputFilters,
            sample_rate_hz,
            bypassed,
            sections: 2,
        })
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
    /// Taken by the full refresh, the addressed-lane retarget update, and the reader's assertion:
    /// it is the definition, never the per-block path.
    fn compute_lane_channel_symmetry(&self, lane: usize) -> bool {
        #[cfg(test)]
        CHANNEL_SYMMETRY_PREDICATE_CALLS.with(|calls| calls.set(calls.get() + 1));
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
            let left_target = &self.filter_target[0][section];
            let right_target = &self.filter_target[1][section];
            let left_step = &self.filter_step[0][section];
            let right_step = &self.filter_step[1][section];
            for (left_word, right_word) in [
                (left_target.c1, right_target.c1),
                (left_target.a2, right_target.a2),
                (left_target.a3, right_target.a3),
                (left_target.m0, right_target.m0),
                (left_target.m1, right_target.m1),
                (left_target.m2, right_target.m2),
                (left_step.c1, right_step.c1),
                (left_step.a2, right_step.a2),
                (left_step.a3, right_step.a3),
                (left_step.m0, right_step.m0),
                (left_step.m1, right_step.m1),
                (left_step.m2, right_step.m2),
            ] {
                if lane_read::<L>(left_word)[lane].to_bits()
                    != lane_read::<L>(right_word)[lane].to_bits()
                {
                    return false;
                }
            }
            if self.filter_remaining[0][section][lane] != self.filter_remaining[1][section][lane] {
                return false;
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
        self.refresh_filter_plan();
    }

    // REALTIME_POLICY_BEGIN
    /// One populated lane's whole per-lane state ([`InputLaneState`], issue #1276 D2).
    fn export_lane(&self, lane: usize) -> InputLaneState {
        debug_assert!(lane < self.members && lane < L::WIDTH);
        let read = |value: L| lane_read::<L>(value)[lane];
        let words = |coef: &SvfCoef<L>| {
            [
                read(coef.c1),
                read(coef.a2),
                read(coef.a3),
                read(coef.m0),
                read(coef.m1),
                read(coef.m2),
            ]
        };
        InputLaneState {
            channels: core::array::from_fn(|channel| InputChannelState {
                trim: read(self.coef.trim[channel]),
                ramp: [
                    read(self.ramp.current[channel]),
                    read(self.ramp.target[channel]),
                    read(self.ramp.step[channel]),
                    read(self.ramp.remaining[channel]),
                ],
                remaining: self.remaining[channel][lane],
                silence: read(self.state.silence[channel]),
                sections: core::array::from_fn(|section| InputSectionState {
                    coef: words(&self.coef.section[channel][section]),
                    target: words(&self.filter_target[channel][section]),
                    step: words(&self.filter_step[channel][section]),
                    remaining: self.filter_remaining[channel][section][lane],
                    integrators: [
                        read(self.state.section[channel][section].ic1),
                        read(self.state.section[channel][section].ic2),
                    ],
                }),
            }),
        }
    }

    /// Overwrite one populated lane with `state`, word for word, and re-derive only the caches
    /// those words feed: the two ramping flags, the elision plan and the lane's channel-symmetry
    /// bit. The lane then renders exactly what the lane it was exported from would have.
    ///
    /// The prepared endpoint (`filter_initial`) is this stage's own and stays: a full reset
    /// returns to what *this* plan prepared.
    fn import_lane(&mut self, lane: usize, state: &InputLaneState) {
        debug_assert!(lane < self.members && lane < L::WIDTH);
        fn put<L: Lane>(value: &mut L, lane: usize, word: f32) {
            let mut words = lane_read::<L>(*value);
            words[lane] = word;
            *value = lane_words::<L>(&words);
        }
        fn put_coef<L: Lane>(coef: &mut SvfCoef<L>, lane: usize, words: &[f32; 6]) {
            put(&mut coef.c1, lane, words[0]);
            put(&mut coef.a2, lane, words[1]);
            put(&mut coef.a3, lane, words[2]);
            put(&mut coef.m0, lane, words[3]);
            put(&mut coef.m1, lane, words[4]);
            put(&mut coef.m2, lane, words[5]);
        }
        for (channel, carried) in state.channels.iter().enumerate() {
            put(&mut self.coef.trim[channel], lane, carried.trim);
            put(&mut self.ramp.current[channel], lane, carried.ramp[0]);
            put(&mut self.ramp.target[channel], lane, carried.ramp[1]);
            put(&mut self.ramp.step[channel], lane, carried.ramp[2]);
            put(&mut self.ramp.remaining[channel], lane, carried.ramp[3]);
            self.remaining[channel][lane] = carried.remaining;
            put(&mut self.state.silence[channel], lane, carried.silence);
            for (section, words) in carried.sections.iter().enumerate() {
                put_coef(&mut self.coef.section[channel][section], lane, &words.coef);
                put_coef(
                    &mut self.filter_target[channel][section],
                    lane,
                    &words.target,
                );
                put_coef(&mut self.filter_step[channel][section], lane, &words.step);
                self.filter_remaining[channel][section][lane] = words.remaining;
                let integrators = &mut self.state.section[channel][section];
                put(&mut integrators.ic1, lane, words.integrators[0]);
                put(&mut integrators.ic2, lane, words.integrators[1]);
            }
        }
        // The flags `settle` and `settle_filter` would publish at a block boundary.
        self.ramping = self
            .remaining
            .iter()
            .any(|channel| channel[..L::WIDTH].iter().any(|remaining| *remaining != 0));
        self.filter_ramping = self.filter_remaining.iter().any(|channel| {
            channel.iter().any(|section| {
                section
                    .iter()
                    .take(self.members)
                    .any(|remaining| *remaining != 0)
            })
        });
        // Both refreshes are defence rather than load-bearing today: a settled carried lane
        // holds the coefficients its successor prepared, a ramping lane's body ignores the
        // elision plan, and `lane_channel_symmetry`'s debug assertion backs the symmetry bit, so
        // no test turns red without them. They are kept so the caches always describe the words
        // just written, whatever a later carry imports.
        self.refresh_filter_plan();
        self.refresh_channel_symmetry_lane(lane);
    }
    // REALTIME_POLICY_END
}

/// One input bank lane's whole state, both channels: what a plan swap hands from the
/// predecessor's lane to the successor's (issue #1276 D2).
///
/// Plain data, fixed size, no heap: the filter integrators, the coefficients in use, any
/// coefficient ramp in flight (target, per-sample step and countdown), the trim ramp
/// (current, target, step, the kernel's countdown word and the authoritative countdown), and the
/// input's silence counter (issue #1328, amendment A9). The
/// polarity is the trim's sign. In-memory state handed across a plan replacement, never persisted
/// (AGENTS.md, R6b).
#[derive(Clone, Copy, Debug)]
pub struct InputLaneState {
    channels: [InputChannelState; 2],
}

#[derive(Clone, Copy, Debug)]
struct InputChannelState {
    /// `coef.trim`, the live trim word the kernel loads.
    trim: f32,
    /// `ramp.{current, target, step, remaining}`.
    ramp: [f32; 4],
    /// The authoritative trim countdown.
    remaining: u32,
    /// `state.silence`: the input's run of exactly-zero frames (issue #1328, amendment A9), so the
    /// successor arms the joint flush on the frame the predecessor would have.
    silence: f32,
    /// High-pass, then low-pass.
    sections: [InputSectionState; 2],
}

#[derive(Clone, Copy, Debug)]
struct InputSectionState {
    /// `[c1, a2, a3, m0, m1, m2]` in use.
    coef: [f32; 6],
    /// The accepted target, same order.
    target: [f32; 6],
    /// The per-sample coefficient step, same order.
    step: [f32; 6],
    /// The authoritative coefficient-ramp countdown.
    remaining: u32,
    /// `[ic1, ic2]`.
    integrators: [f32; 2],
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
/// This is the *only* ramped-fader implementation in the workspace. A live-control track is this
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

    #[inline(always)]
    fn is_settled(&self) -> bool {
        self.remaining.iter().all(|channel| {
            channel
                .iter()
                .take(L::WIDTH)
                .all(|&remaining| remaining == 0)
        })
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
    /// Issue #137 D1: live controls change the pan window with the pan, so the retarget and the
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

    /// Renders a settled segment on the current coefficients.
    ///
    /// With no identity lane over all `L::WIDTH` lanes -- padding lanes included, so a partial
    /// bank, whose padding lanes are the identity, always keeps the select -- the per-lane select
    /// of [`matrix2x2_block`] returns its second arm on every word, and
    /// [`matrix2x2_block_without_identity`] is that arm without the select LLVM folds into an x86
    /// masked store (issue #944). Class A. The mask is tested here, once per call; it is never
    /// cached, because `sync_settled` rewrites it on every event and settle.
    #[inline(always)]
    fn settled_block(&self, left: &mut [f32], right: &mut [f32], frames: usize) {
        if L::mask_any(self.coef.identity) {
            matrix2x2_block::<L>(left, right, frames, &self.coef);
        } else {
            #[cfg(test)]
            MATRIX_SELECT_FREE_BLOCKS.with(|blocks| blocks.set(blocks.get() + 1));
            matrix2x2_block_without_identity::<L>(left, right, frames, &self.coef);
        }
    }

    /// Renders one settled block of the fused fader and matrix on the current coefficients.
    ///
    /// The fused twin of [`Self::settled_block`] (issue #954). With no identity lane over all
    /// `L::WIDTH` lanes -- padding lanes included, so a partial bank always keeps the select --
    /// the per-lane select of [`fader_matrix_block`] returns its second arm on every word, and
    /// [`fader_matrix_block_without_identity`] is that arm with no select. Class A.
    ///
    /// Every caller runs this only after its own settled check, which runs after the block's
    /// controls have drained, so `coef.identity` is the mask `sync_settled` wrote for exactly these
    /// coefficients, including a same-block instant retarget to the identity. It is tested here,
    /// once per call, and never cached.
    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    fn fused_settled_block(
        &self,
        left: &mut [f32],
        right: &mut [f32],
        frames: usize,
        gain_left: L,
        mute_left: L::Mask,
        gain_right: L,
        mute_right: L::Mask,
    ) {
        if L::mask_any(self.coef.identity) {
            fader_matrix_block::<L>(
                left, right, frames, gain_left, mute_left, gain_right, mute_right, &self.coef,
            );
        } else {
            #[cfg(test)]
            FUSED_SELECT_FREE_BLOCKS.with(|blocks| blocks.set(blocks.get() + 1));
            fader_matrix_block_without_identity::<L>(
                left, right, frames, gain_left, mute_left, gain_right, mute_right, &self.coef,
            );
        }
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
            self.settled_block(left, right, frames);
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
            self.settled_block(
                &mut left[split..],
                &mut right[split..],
                frames - ramp_frames,
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
            self.matrix.stage.fused_settled_block(
                left,
                right,
                frames,
                self.fader_mute.stage.gain[0],
                self.fader_mute.stage.mute[0],
                self.fader_mute.stage.gain[1],
                self.fader_mute.stage.mute[1],
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
        self.input.reset_with_kind(kind);
        self.matrix.reset();
        if matches!(kind, BuiltinResetKind::FullToPrepared) {
            self.fader_mute.reset();
        }
    }
    pub fn into_sections(self) -> (InputBuiltins, FaderMuteBuiltins, MatrixBuiltins) {
        (self.input, self.fader_mute, self.matrix)
    }
    /// Consume the chain and retain only its bankable post-input section.
    pub fn into_input_builtins(self) -> InputBuiltins {
        self.input
    }
}

/// Validates one strip's parameters and designs its input section (trim, polarity, HPF, LPF).
fn prepare_input_track(
    sample_rate: u32,
    parameters: &BuiltinParameters,
) -> Result<PreparedInputTrack, BuiltinParameterError> {
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
    Ok(PreparedInputTrack {
        left: lane(parameters.left)?,
        right: lane(parameters.right)?,
        silence_frames: lane::silence_frames(sample_rate),
    })
}

/// The certified tail and exact-rest bounds (#1329 D1, D2, D4) of the input section one strip's
/// parameters prepare: its trim, polarity, HPF and LPF as designed, with no live input lane (a
/// strip whose input lane is live reports [`input_section_live_bound`] instead, D5). The fader
/// and the matrix are gain-only and add nothing (D6).
///
/// Control plane only: the computation allocates and runs in `f64`, at preparation, and nothing
/// render owns stores its result (#1329 Amendment 4, R5). `builtins-compiler` keeps it per strip
/// in its prepared session, beside the strip's tail.
///
/// # Errors
///
/// The same as [`BuiltinChain::new`] for the same parameters.
pub fn input_section_bound(
    sample_rate: u32,
    parameters: BuiltinParameters,
) -> Result<InputSectionBound, BuiltinParameterError> {
    Ok(input_section_bound_charged(sample_rate, parameters)?.bound)
}

/// [`input_section_bound`] with its charge: the frames its computation walks and the
/// frame-equivalents it charges a preparation's budget (#1457 D1, D3, Amendment 3).
///
/// # Errors
///
/// The same as [`input_section_bound`].
pub fn input_section_bound_charged(
    sample_rate: u32,
    parameters: BuiltinParameters,
) -> Result<ChargedInputBound, BuiltinParameterError> {
    let track = prepare_input_track(sample_rate, &parameters)?;
    Ok(
        tail::fixed_input_bound(sample_rate, [&track.left, &track.right], u64::MAX)
            .expect("no walk reaches u64::MAX frames"),
    )
}

/// The input-section bound every strip of a session reports, in order, computing each distinct
/// design's bound at most once (#1329 Amendment 4, R7) and within the preparation budget
/// [`INPUT_BOUND_BUDGET_FRAMES`] (#1457 D1, D3).
///
/// Each distinct design, at its first strip in strip order, charges the budget its charge
/// ([`input_section_bound_charged`]); a later strip with the same design reports the same value
/// and charges nothing. While the remaining budget covers a design's charge, the design reports its
/// own bound. A design whose charge passes the remaining budget reports the rate's live bound
/// ([`input_section_live_bound_table`]), which is certified for every history of trim, polarity and
/// filter targets and so for every fixed design, and the remaining budget is then spent: every
/// later design with an enabled section reports the live bound without a walk. A memoryless design
/// (both filters disabled on both channels) reports the zero bound and charges nothing. The result
/// is a function of the session only.
///
/// `cache` (#1457 D2) keeps bounds across calls. A design it holds charges its stored charge
/// without computing anything, so the result is the same with and without it, cold or warm.
///
/// # Errors
///
/// The first strip's error, as [`input_section_bound`] reports it.
pub fn input_section_bounds(
    sample_rate: u32,
    strips: impl IntoIterator<Item = BuiltinParameters>,
    cache: Option<&mut InputBoundCache>,
) -> Result<Vec<InputSectionBound>, BuiltinParameterError> {
    input_section_bounds_within(sample_rate, strips, INPUT_BOUND_BUDGET_FRAMES, cache)
}

/// [`input_section_bounds`] under a budget of `budget` frame-equivalents. Only the gates choose
/// another budget, through `test_support::input_section_bounds_within` (#1457 ruling (i)).
fn input_section_bounds_within(
    sample_rate: u32,
    strips: impl IntoIterator<Item = BuiltinParameters>,
    budget: u64,
    mut cache: Option<&mut InputBoundCache>,
) -> Result<Vec<InputSectionBound>, BuiltinParameterError> {
    let fallback =
        input_section_live_bound_table(sample_rate).unwrap_or(InputSectionBound::UNBOUNDED);
    let mut remaining = budget;
    let mut designs = std::collections::BTreeMap::new();
    strips
        .into_iter()
        .map(|parameters| {
            let track = prepare_input_track(sample_rate, &parameters)?;
            let lanes = [&track.left, &track.right];
            let key = tail::input_bound_key(sample_rate, lanes);
            if let Some(bound) = designs.get(&key) {
                return Ok(*bound);
            }
            // A memoryless design (both filters disabled on both channels) has the zero bound and
            // walks nothing, so it is neither computed, charged nor cached.
            if lanes
                .iter()
                .all(|lane| !lane.hpf.enabled && !lane.lpf.enabled)
            {
                designs.insert(key, InputSectionBound::ZERO);
                return Ok(InputSectionBound::ZERO);
            }
            // A spent budget walks nothing more: every other design reports the live bound
            // without a walk.
            let cached = if remaining == 0 {
                Some(CachedDesign::ChargeAbove(0))
            } else {
                cache.as_deref().and_then(|cache| cache.get(&key))
            };
            let charged = match cached {
                Some(CachedDesign::Bound(hit)) => (hit.charge <= remaining).then_some(hit),
                // The walk passed `above` frames: under no more budget than that it stops again.
                Some(CachedDesign::ChargeAbove(above)) if remaining <= above => None,
                Some(CachedDesign::ChargeAbove(_)) | None => {
                    let computed = tail::fixed_input_bound(sample_rate, lanes, remaining);
                    if let Some(cache) = cache.as_deref_mut() {
                        match computed {
                            Some(bound) => cache.insert(key, CachedDesign::Bound(bound)),
                            None if remaining > 0 => {
                                cache.insert(key, CachedDesign::ChargeAbove(remaining));
                            }
                            None => {}
                        }
                    }
                    computed
                }
            };
            let bound = if let Some(charged) = charged {
                remaining -= charged.charge;
                charged.bound
            } else {
                remaining = 0;
                fallback
            };
            designs.insert(key, bound);
            Ok(bound)
        })
        .collect()
}

fn prepare_sections(
    sample_rate: u32,
    parameters: BuiltinParameters,
) -> Result<(InputBuiltins, FaderMuteBuiltins, MatrixBuiltins), BuiltinParameterError> {
    let track = prepare_input_track(sample_rate, &parameters)?;
    let matrix = parameters.matrix.checked()?;
    let fader = |params: ChannelParameters| -> Result<FaderLane, BuiltinParameterError> {
        Ok(FaderLane {
            gain: db_gain(params.fader_db)?,
            muted: params.muted,
        })
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
    ///   section is reached by live controls through `LiveControlInputProcessor`, which owns the
    ///   consumer, folds `ChannelSymmetryWitness::admit` per record and conjoins the result with
    ///   this value -- exactly as `BuiltinBankProcessor` does for the banked form. The seam the
    ///   builtins liveness work was to land on is closed (#210 phase 3): `TrackInputRecord`
    ///   implements `LiveControlRecord` with `SEAM = UpstreamOfSeam`, so an asymmetric
    ///   `trim_db` or `polarity_invert` retarget clears `LIVE` at the drain, before the collapse
    ///   dispatch reads the witness. Filter targets use the same upstream queue and clear the
    ///   witness according to their addressed lanes.
    #[must_use]
    pub fn channel_symmetry(&self) -> ChannelSymmetryWitness {
        let mut witness = ChannelSymmetryWitness::SYMMETRIC;
        witness.set(
            ChannelSymmetryWitness::DESIGNED,
            self.stage.lane_channel_symmetry(0),
        );
        witness
    }

    /// Copies this track's retained HPF/LPF words into caller-owned response storage.
    pub fn copy_response_snapshot(
        &self,
        sample_rate_hz: u32,
        request: ResponseSnapshotRequest<'_>,
    ) -> Result<ResponseSnapshotSummary, ResponseAnalysisError> {
        self.stage
            .copy_response_snapshot_lane(0, sample_rate_hz, request)
    }

    /// Applies one trusted, precomputed HPF/LPF section target. The fixed 64-update ramp is
    /// intentionally independent of the prepared-control smoothing field in this tranche.
    pub fn apply_prepared_filter(
        &mut self,
        target: PreparedInputFilterTarget,
    ) -> Result<(), BuiltinParameterError> {
        validate_prepared_input_filter_target(&target)?;
        self.stage.apply_prepared_filter(0, target);
        Ok(())
    }

    /// Renders one already-validated block. Infallible: the block shape was checked once (F9).
    pub fn process(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport {
        let frames = block.left.len();
        self.stage.process(block.left, block.right, frames)
    }
    /// Retargets this track's `trim_db` on the addressed channels, over an explicit window.
    ///
    /// The scalar sibling of [`BuiltinInputBank::set_trim_db`]: one body, one width, so a
    /// per-node live-control track and a bank lane cannot drift.
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

    pub fn reset_with_kind(&mut self, kind: BuiltinResetKind) {
        self.stage.reset_with_kind(kind);
    }
    pub fn lifetime_recovered_state(&self) -> (u64, u64) {
        (
            self.stage.lifetime_recovered[0],
            self.stage.lifetime_recovered[1],
        )
    }
}

/// This crate's one dispatch over its per-width bank kernels ([`InputStageKernel`],
/// [`FaderStageKernel`], [`MatrixStageKernel`]), and the one place outside those enums where their
/// eight-lane variant is written (issues #1110 and #1112).
///
/// The `Simd8` variants, like `lane::Simd8` and `effect_contract::BankWidth::Eight`, exist only
/// where `avx2` is enabled: a 4-lane (NEON/simd128) build runs four lanes only. So each rule's
/// eight-lane arm carries `#[cfg(target_feature = "avx2")]`, and every other line of the crate is
/// written once for both widths:
///
/// * `per_width!(Kernel at width, |L| stage)` builds the `Kernel` variant of `width` around
///   `stage`, with `L` naming that width's lane type in it;
/// * `per_width!(Kernel(stage) in kernel => body)` evaluates `body` with `stage` bound to the stage
///   inside `kernel`, whichever width it holds;
/// * `per_width!((A(a), B(b)) in (x, y) => body, else mismatch)` evaluates `body` on the stages
///   inside two kernels of the same width, and `mismatch` when their widths differ. That arm, too,
///   exists only where there is a second width to differ by.
///
/// Each form expands to a `match`, so `return` and `?` in a body act on the caller.
macro_rules! per_width {
    ($kernel:ident at $width:expr, |$lane:ident| $stage:expr) => {
        match $width {
            BankWidth::Four => {
                type $lane = Simd4;
                $kernel::Simd4($stage)
            }
            #[cfg(target_feature = "avx2")]
            BankWidth::Eight => {
                type $lane = lane::Simd8;
                $kernel::Simd8($stage)
            }
        }
    };
    ($kernel:ident($binding:ident) in $value:expr => $body:expr) => {
        match $value {
            $kernel::Simd4($binding) => $body,
            #[cfg(target_feature = "avx2")]
            $kernel::Simd8($binding) => $body,
        }
    };
    (
        ($a:ident($x:ident), $b:ident($y:ident)) in $value:expr => $body:expr,
        else $mismatch:expr
    ) => {
        match $value {
            ($a::Simd4($x), $b::Simd4($y)) => $body,
            #[cfg(target_feature = "avx2")]
            ($a::Simd8($x), $b::Simd8($y)) => $body,
            #[cfg(target_feature = "avx2")]
            _ => $mismatch,
        }
    };
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
    /// Four lanes: the 4-lane (NEON/simd128) width.
    Simd4(InputStage<Simd4>),
    /// Eight lanes: the 8-lane (AVX2) width, absent elsewhere ([`per_width!`]).
    #[cfg(target_feature = "avx2")]
    Simd8(InputStage<lane::Simd8>),
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
        let designed =
            per_width!(InputStageKernel(stage) in &self.stage => stage.lane_channel_symmetry(lane));
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
        let stage = per_width!(InputStageKernel at width, |L| InputStage::<L>::new(&tracks));
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

    /// Copies one populated lane's retained HPF/LPF words into caller-owned response storage.
    pub fn copy_response_snapshot_lane(
        &self,
        lane: usize,
        sample_rate_hz: u32,
        request: ResponseSnapshotRequest<'_>,
    ) -> Result<ResponseSnapshotSummary, ResponseAnalysisError> {
        per_width!(InputStageKernel(stage) in &self.stage => {
            stage.copy_response_snapshot_lane(lane, sample_rate_hz, request)
        })
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
        per_width!(InputStageKernel(stage) in &mut self.stage => stage.process(left, right, frames))
    }

    /// Renders one AoSoA block of the **collapsed** cohort: the left plane only.
    ///
    /// `right` is deliberately not a parameter. A collapsed chain gathers one plane and duplicates
    /// it at the seam, so there is no right block here to be wrong about.
    pub fn process_mono(&mut self, left: &mut [f32], frames: u32) -> BuiltinProcessReport {
        let frames = frames as usize;
        debug_assert_eq!(left.len(), frames * self.width.lanes() as usize);
        per_width!(InputStageKernel(stage) in &mut self.stage => stage.process_mono(left, frames))
    }

    /// Whether this bank may run [`BuiltinInputBank::process_mono`] at all.
    ///
    /// Fixed by preparation: the elision plan is decided once, from the coefficient words and a
    /// `+0.0` state, and nothing on the render path re-decides it.
    #[must_use]
    pub const fn supports_mono_collapse(&self) -> bool {
        per_width!(InputStageKernel(stage) in &self.stage => stage.mono_collapse_gate())
    }

    /// Copies every lane's left-channel per-channel state onto the right channel (the disengage
    /// copy): the integrators, the input's silence counter and the trim ramp record.
    pub fn desymmetrize(&mut self) {
        per_width!(InputStageKernel(stage) in &mut self.stage => stage.desymmetrize())
    }

    /// Whether this bank can prove, right now, that its two channels' state is bit-equal (M3).
    #[must_use]
    pub fn channels_agree(&self) -> bool {
        per_width!(InputStageKernel(stage) in &self.stage => stage.channels_agree())
    }

    /// Applies one trusted precomputed input-filter target to one populated lane in this bank.
    pub fn apply_prepared_filter(
        &mut self,
        lane: usize,
        target: PreparedInputFilterTarget,
    ) -> Result<(), BuiltinParameterError> {
        if lane >= self.members {
            return Err(BuiltinParameterError::LaneLength);
        }
        validate_prepared_input_filter_target(&target)?;
        per_width!(InputStageKernel(stage) in &mut self.stage => {
            stage.apply_prepared_filter(lane, target);
        });
        Ok(())
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
        per_width!(InputStageKernel(stage) in &mut self.stage => {
            stage.set_trim_db(lane, channels, gain, smoothing_samples);
        });
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
        per_width!(InputStageKernel(stage) in &mut self.stage => {
            stage.set_polarity_invert(lane, channels, inverted, smoothing_samples);
        });
        Ok(())
    }

    /// The trim coefficient one lane and channel applies to the next frame. Readback only.
    #[must_use]
    pub fn trim_signed(&self, lane: usize, channel: usize) -> f32 {
        per_width!(InputStageKernel(stage) in &self.stage => stage.trim_signed(lane, channel))
    }

    /// Resets only the per-lane filter state; prepared coefficients remain unchanged.
    pub fn reset(&mut self) {
        per_width!(InputStageKernel(stage) in &mut self.stage => stage.reset())
    }

    // REALTIME_POLICY_BEGIN
    /// Populated lane `lane`'s whole state (issue #1276 D2), or `None` for a padding lane or one
    /// past the bank. Allocation-free; bounded by the lane's fixed word count.
    #[must_use]
    pub fn export_lane(&self, lane: usize) -> Option<InputLaneState> {
        if lane >= self.members {
            return None;
        }
        Some(per_width!(InputStageKernel(stage) in &self.stage => stage.export_lane(lane)))
    }

    /// Overwrite populated lane `lane` with `state` (issue #1276 D2): every word verbatim, then the
    /// caches re-derived, so the lane renders bit-exactly what the exported lane would have.
    /// Refused, changing nothing, for a padding lane or one past the bank. Allocation-free.
    ///
    /// # Errors
    ///
    /// [`BuiltinParameterError::LaneLength`] when `lane` is not a populated member.
    pub fn import_lane(
        &mut self,
        lane: usize,
        state: &InputLaneState,
    ) -> Result<(), BuiltinParameterError> {
        if lane >= self.members {
            return Err(BuiltinParameterError::LaneLength);
        }
        per_width!(InputStageKernel(stage) in &mut self.stage => stage.import_lane(lane, state));
        Ok(())
    }
    // REALTIME_POLICY_END
}

/// The dispatched fader-ramp stage of a bank, at the width the selected backend chose.
///
/// Both variants are held inline for the reason [`InputStageKernel`] states: boxing the larger one
/// would put the coefficients the render loop loads behind a pointer it chases once per bank per
/// block, to save a few hundred bytes on a structure there is one of per cohort. The space is not
/// worth the indirection.
#[allow(clippy::large_enum_variant)]
enum FaderStageKernel {
    /// Four lanes: the 4-lane (NEON/simd128) width.
    Simd4(FaderRampStage<Simd4>),
    /// Eight lanes: the 8-lane (AVX2) width, absent elsewhere ([`per_width!`]).
    #[cfg(target_feature = "avx2")]
    Simd8(FaderRampStage<lane::Simd8>),
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
        let stage = per_width!(FaderStageKernel at width, |L| FaderRampStage::<L>::new(&lanes));
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
        per_width!(FaderStageKernel(stage) in &mut self.stage => {
            stage.set_fader_gain(lane, channels, gain, smoothing_samples);
        });
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
        per_width!(FaderStageKernel(stage) in &mut self.stage => {
            stage.set_mute(lane, channels, muted, smoothing_samples)
        });
        Ok(())
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
        per_width!(FaderStageKernel(stage) in &mut self.stage => {
            stage.process(left, right, frames);
        });
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
        if self.backend != matrix.backend
            || self.width != matrix.width
            || self.members != matrix.members
            || self.remaining_nonzero()
            || matrix.remaining_nonzero()
        {
            return false;
        }
        let stages = (&self.stage, &matrix.stage);
        per_width!((FaderStageKernel(fader), MatrixStageKernel(matrix)) in stages => {
            matrix.fused_settled_block(
                left,
                right,
                frames as usize,
                fader.ramp[0].current,
                fader.ramp[0].mute,
                fader.ramp[1].current,
                fader.ramp[1].mute,
            );
        }, else return false);
        true
    }

    fn remaining_nonzero(&self) -> bool {
        per_width!(FaderStageKernel(stage) in &self.stage => {
            stage.remaining.iter().flatten().any(|v| *v != 0)
        })
    }

    /// Snaps every lane to its target and cancels any ramp in flight.
    pub fn reset(&mut self) {
        per_width!(FaderStageKernel(stage) in &mut self.stage => stage.reset())
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
    /// Four lanes: the 4-lane (NEON/simd128) width.
    Simd4(MatrixStage<Simd4>),
    /// Eight lanes: the 8-lane (AVX2) width, absent elsewhere ([`per_width!`]).
    #[cfg(target_feature = "avx2")]
    Simd8(MatrixStage<lane::Simd8>),
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
        let stage = per_width!(MatrixStageKernel at width, |L| MatrixStage::<L>::new(&lanes));
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
        per_width!(MatrixStageKernel(stage) in &mut self.stage => {
            stage.set_target_over(lane, target, smoothing_samples)
        })
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
        per_width!(MatrixStageKernel(stage) in &mut self.stage => {
            stage.process(left, right, frames);
        });
        BuiltinProcessReport::default()
    }

    fn remaining_nonzero(&self) -> bool {
        per_width!(MatrixStageKernel(stage) in &self.stage => {
            stage.remaining.iter().any(|v| *v != 0)
        })
    }

    /// Snaps every lane to its target and cancels any ramp in flight.
    pub fn reset(&mut self) {
        per_width!(MatrixStageKernel(stage) in &mut self.stage => stage.reset())
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

/// The live-control fader and mute section of one track (issue #140 B).
///
/// # The ramped-fader decision, and why it is a separate type
///
/// [`FaderMuteBuiltins`] is the *prepared-only* fader: one multiply and one `andnot` per frame,
/// with no ramp state at all. (This paragraph read "`fader_db` and `mute` declare
/// `BuiltinParameterUpdateRate::PreparedOnly`" when it was written; those rows are `BlockTarget`
/// since #140 B flipped them, and the ABI table is the authority. What the sentence was about is
/// the *type*, and that is unchanged.) Making that type ramp would change the fixed
/// input/fader/matrix section layout, the builtin resource report, and the frozen
/// builtins-compiler transcript for **every** session, with live controls or without.
///
/// This type is the ramped fader instead. It exists only for a track live controls drive, it is
/// bound only by `LiveControlFaderProcessor`, and [`FaderMuteBuiltins`] is byte-for-byte the type
/// it always was. No builtins fixture digest, no frozen transcript and no corpus digest moves,
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
/// `step = (target - current) / N` at the moment the target changes, then
/// `current = ramp_toward(current, step, target)` per sample (`current + step` held inside its
/// endpoints, #1408) and an exact assignment of `target` on update `N` (master plan D11). There is
/// no division per sample and no allocation anywhere on this path.
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

    /// Completes a settled fader over a graph block whose shape was validated by preparation.
    ///
    /// This narrow raw-slice entry point is used only by the graph split-owner completion path;
    /// it avoids a fallible render-time wrapper while retaining the same fader kernel and state.
    pub fn process_pending(&mut self, left: &mut [f32], right: &mut [f32]) {
        let frames = left.len();
        self.stage.process(left, right, frames);
    }

    /// Completes a pending fader using an already shape-validated builtin block.
    pub fn process_pending_block(&mut self, block: &mut DualMonoBlock<'_>) {
        let frames = block.left.len();
        self.stage.process(block.left, block.right, frames);
    }

    /// Whether both dual-mono lanes are settled and can use the stateless fader arithmetic.
    #[must_use]
    pub fn is_settled(&self) -> bool {
        self.stage.is_settled()
    }

    /// Runs the settled fader and matrix stages with the established fused scalar kernel.
    /// Returns `false` when either stage is ramping; callers must then run the original two-stage
    /// arithmetic for the whole block.
    pub fn process_fader_matrix(
        &mut self,
        matrix: &mut MatrixBuiltins,
        block: &mut DualMonoBlock<'_>,
    ) -> bool {
        if !self.stage.is_settled() || !matrix.stage.is_settled() {
            return false;
        }
        let DualMonoBlock { left, right, .. } = block;
        let frames = left.len();
        matrix.stage.fused_settled_block(
            left,
            right,
            frames,
            self.stage.ramp[0].current,
            self.stage.ramp[0].mute,
            self.stage.ramp[1].current,
            self.stage.ramp[1].mute,
        );
        true
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
///
/// Issue #1255 D3: preparation's `fader_lanes`, the render-side setter
/// ([`BuiltinFaderBank::set_fader_db`]) and host-core's live-delta classifier share this one
/// function, so those three cannot disagree on the domain. Two other checks still spell the
/// `[-144, 24]` dB range themselves (`prepare_sections` here and builtins-compiler's `gain_path`);
/// they agree with it today.
///
/// # Errors
///
/// [`BuiltinParameterError::GainDomain`] when `db` is not finite or is outside `[-144, 24]`.
pub fn checked_fader_gain(db: f32) -> Result<f32, BuiltinParameterError> {
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
/// Stable selection and presence bits for fixed-size meter snapshots.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MeterMetricSet(u8);
impl MeterMetricSet {
    pub const SAMPLE_PEAK: Self = Self(1 << 0);
    pub const ENERGY_RMS: Self = Self(1 << 1);
    pub const COUNTS: Self = Self(1 << 2);
    pub const HELD_PEAK: Self = Self(1 << 3);
    pub const ALL: Self =
        Self(Self::SAMPLE_PEAK.0 | Self::ENERGY_RMS.0 | Self::COUNTS.0 | Self::HELD_PEAK.0);

    #[must_use]
    pub const fn from_bits_retain(bits: u8) -> Self {
        Self(bits)
    }
    #[must_use]
    pub const fn bits(self) -> u8 {
        self.0
    }
    #[must_use]
    pub const fn contains(self, metric: Self) -> bool {
        self.0 & metric.0 == metric.0
    }
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.0 != 0 && self.0 & !Self::ALL.0 == 0
    }
}
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
    Metrics,
    Queue,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeterObservationError {
    LaneLength,
    SampleTimeOverflow,
}

/// Checked, borrowed left/right input for a single meter lane.
#[derive(Clone, Copy)]
pub struct MeterInput<'a> {
    left: &'a [f32],
    right: &'a [f32],
    frames: usize,
    stride: usize,
    lane: usize,
}

impl<'a> MeterInput<'a> {
    // REALTIME_POLICY_BEGIN
    /// Validate both exact plane lengths before any meter state can change.
    /// Empty planes accept zero frames with a valid stride/lane.
    ///
    /// # Errors
    /// [`MeterObservationError::LaneLength`] for any invalid or overflowing shape.
    pub fn strided(
        left: &'a [f32],
        right: &'a [f32],
        frames: usize,
        stride: usize,
        lane: usize,
    ) -> Result<Self, MeterObservationError> {
        if stride == 0
            || lane >= stride
            || frames.checked_mul(stride) != Some(left.len())
            || right.len() != left.len()
        {
            return Err(MeterObservationError::LaneLength);
        }
        Ok(Self {
            left,
            right,
            frames,
            stride,
            lane,
        })
    }

    fn samples(
        &self,
        words: &'a [f32],
        start: usize,
        end: usize,
    ) -> impl Iterator<Item = f32> + Clone {
        let lane = self.lane;
        words[start * self.stride..end * self.stride]
            .chunks_exact(self.stride)
            .map(move |frame| frame[lane])
    }
    // REALTIME_POLICY_END
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
    pub present_metrics: MeterMetricSet,
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

/// One block's full meter partials for one meter's `[left, right]` lanes, computed by the bank
/// that produced the block (issue #950): what [`MeterAccumulator::observe_input_banked`] commits
/// instead of reading the samples.
///
/// Every field is what the meter's own scalar loop would compute over the same block, and the
/// commit relies on it: `lane::kernels::builtins::meter_block` over the block's final words,
/// seeded with `energy_seed`, gives exactly this.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeterBankedBlock {
    /// The maximum of `+0.0` and every sanitized magnitude of the block, per channel.
    pub sample_peak: [f32; 2],
    /// How many sanitized magnitudes of the block are `>= 1.0`, per channel.
    pub clipped: [u64; 2],
    /// How many of the block's words the sanitization replaced (NaN, `±inf` and nonzero
    /// subnormals), per channel.
    pub sanitized: [u64; 2],
    /// The energy the pass started from, per channel: the meter commits only if these are its own
    /// post-preamble energies, bit for bit.
    pub energy_seed: [f64; 2],
    /// `energy_seed` plus the square of every sanitized sample, added one sample at a time in
    /// frame order, per channel. It replaces the meter's energy; it is never added to it.
    pub energy: [f64; 2],
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
    metrics: MeterMetricSet,
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

#[cfg(test)]
mod meter_work_probe {
    use core::sync::atomic::AtomicUsize;
    pub(super) static ENERGY: AtomicUsize = AtomicUsize::new(0);
    pub(super) static COUNTS: AtomicUsize = AtomicUsize::new(0);
    pub(super) static HELD: AtomicUsize = AtomicUsize::new(0);
    pub(super) static SQRT: AtomicUsize = AtomicUsize::new(0);
}

#[cfg(any(test, feature = "test-support"))]
std::thread_local! {
    static METER_PEAK_SAMPLES: Cell<u64> = const { Cell::new(0) };
}

#[cfg(any(test, feature = "test-support"))]
std::thread_local! {
    static METER_BLOCK_PEAK_MERGES: Cell<u64> = const { Cell::new(0) };
}

#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn test_only_reset_peak_samples() {
    METER_PEAK_SAMPLES.with(|samples| samples.set(0));
}

/// Reset the count of block peaks merged by [`MeterAccumulator::observe_input_with_block_peak`]'s
/// fast path (issue #943).
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn test_only_reset_block_peak_merges() {
    METER_BLOCK_PEAK_MERGES.with(|merges| merges.set(0));
}

/// Block peaks merged by the fast path since the last reset: one per meter per block that took it.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[must_use]
pub fn test_only_block_peak_merges() -> u64 {
    METER_BLOCK_PEAK_MERGES.with(Cell::get)
}

#[cfg(any(test, feature = "test-support"))]
std::thread_local! {
    static METER_BANKED_COMMITS: Cell<u64> = const { Cell::new(0) };
}

/// Reset the count of banked blocks committed by [`MeterAccumulator::observe_input_banked`]
/// (issue #950).
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn test_only_reset_banked_meter_commits() {
    METER_BANKED_COMMITS.with(|commits| commits.set(0));
}

/// Banked blocks committed since the last reset: one per meter per block that took the commit.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[must_use]
pub fn test_only_banked_meter_commits() -> u64 {
    METER_BANKED_COMMITS.with(Cell::get)
}

#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[must_use]
pub fn test_only_peak_samples() -> u64 {
    METER_PEAK_SAMPLES.with(Cell::get)
}

impl MeterAccumulator {
    pub fn prepare(
        handle: MeterHandle,
        config: MeterConfig,
        sample_rate: u32,
    ) -> Result<PreparedMeter, MeterConfigError> {
        Self::prepare_selected(handle, config, sample_rate, MeterMetricSet::ALL)
    }

    pub fn prepare_selected(
        handle: MeterHandle,
        config: MeterConfig,
        sample_rate: u32,
        metrics: MeterMetricSet,
    ) -> Result<PreparedMeter, MeterConfigError> {
        if !metrics.is_valid() {
            return Err(MeterConfigError::Metrics);
        }
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
                metrics,
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
        let input = MeterInput::strided(left, right, left.len(), 1, 0)?;
        self.observe_input(input, first_sample)
    }

    /// Observe checked planar or strided words with the same scalar window state machine.
    ///
    /// # Errors
    /// [`MeterObservationError::SampleTimeOverflow`] before any state mutation if the
    /// block would run past `u64::MAX`.
    pub fn observe_input(
        &mut self,
        input: MeterInput<'_>,
        first_sample: u64,
    ) -> Result<(), MeterObservationError> {
        self.observe_input_with_block_peak(input, first_sample, None)
    }

    /// [`Self::observe_input`], with this block's `[left, right]` sample peak already computed by
    /// the bank that produced it (issue #943).
    ///
    /// `block_peak` must be the maximum of `+0.0` and every **sanitized** magnitude of this
    /// block's two channels -- `lane::kernels::builtins::meter_sample_peak_block` seeded with
    /// `+0.0` -- and nothing else. It is used only when the whole block lies inside the current
    /// window and the meter selects exactly [`MeterMetricSet::SAMPLE_PEAK`]; then it is merged into
    /// the window's peak with the same select form the sample loop uses, which on the sanitized
    /// domain is an exact reassociation of that loop, and the samples are not read. Any other
    /// block -- one that crosses a window boundary, an empty one, or any other metric selection --
    /// ignores `block_peak` and takes the sample loop, so every published word is the one
    /// [`Self::observe_input`] would publish.
    ///
    /// # Errors
    /// [`MeterObservationError::SampleTimeOverflow`] before any state mutation if the
    /// block would run past `u64::MAX`.
    pub fn observe_input_with_block_peak(
        &mut self,
        input: MeterInput<'_>,
        first_sample: u64,
        block_peak: Option<[f32; 2]>,
    ) -> Result<(), MeterObservationError> {
        self.observe_block(input, first_sample, block_peak, None)
    }

    /// Whether this meter can ever commit a banked block (issue #950): every metric it keeps is an
    /// order-free block partial or the seeded energy sum.
    ///
    /// Only a held peak with a hold or a decay is not. Its per-sample state machine counts frames
    /// down and multiplies by the decay, so it has no block partial; with neither, it is the plain
    /// select-form maximum `held = a >= held ? a : held`, which merges exactly. Fixed at
    /// preparation, since nothing changes the configuration or the selection afterwards.
    #[must_use]
    pub fn banked_eligible(&self) -> bool {
        !self.metrics.contains(MeterMetricSet::HELD_PEAK)
            || (self.config.peak_hold_frames == 0 && self.config.peak_decay_db_per_second == 0.0)
    }

    /// The `[left, right]` energy this meter's scalar loop would start a `frames`-frame block at
    /// `first_sample` from, or `None` if [`Self::observe_input_banked`] could not commit that block
    /// (issue #950).
    ///
    /// A pure read of this meter's own state, bounded and allocation-free. It mirrors
    /// [`Self::observe_input`]'s preamble without performing it: a block that does not continue the
    /// window is a discontinuity, which will zero both the window's frame count and its energy, and
    /// a meter with no window yet already has both at zero. `None` unless the meter can bank at all
    /// ([`Self::banked_eligible`]), keeps [`MeterMetricSet::ENERGY_RMS`] -- a meter without an
    /// energy has no seed, and answering one would stop a bank's seed search at a meter whose answer
    /// means nothing -- and the whole non-empty block lies inside the window.
    #[must_use]
    pub fn banked_seed(&self, first_sample: u64, frames: usize) -> Option<[f64; 2]> {
        if !self.banked_eligible()
            || !self.metrics.contains(MeterMetricSet::ENERGY_RMS)
            || frames == 0
        {
            return None;
        }
        let discontinuous = self
            .start
            .is_some_and(|start| first_sample != start.saturating_add(u64::from(self.frames)));
        let (now, seed) = if discontinuous {
            (0, [0.0, 0.0])
        } else {
            (self.frames, [self.left.energy, self.right.energy])
        };
        if frames > self.config.period_frames.get().saturating_sub(now) as usize {
            return None;
        }
        Some(seed)
    }

    /// [`Self::observe_input`], with this block's full meter partials already computed by the bank
    /// that produced it (issue #950).
    ///
    /// `banked` must be what the meter's own loop would compute over this block: the lane pass
    /// `lane::kernels::builtins::meter_block` over the block's words, seeded with
    /// `banked.energy_seed`. The preamble runs unchanged first -- the time check, the discontinuity
    /// and the window start -- and then the block is committed without reading a sample only when
    /// all of these hold:
    ///
    /// * `banked` is `Some`, and this meter is [`Self::banked_eligible`];
    /// * the block is non-empty and lies inside the current window;
    /// * if the meter keeps [`MeterMetricSet::ENERGY_RMS`], `banked.energy_seed` is its own
    ///   post-preamble energy, bit for bit, on both channels.
    ///
    /// The commit, per channel, left then right: the peak and the held peak merge with the loop's
    /// select forms (`q > peak`, `q >= held`); the energy is **replaced** by `banked.energy`, the
    /// seeded sample-serial sum, and never has a partial added to it; the counts `saturating_add`
    /// into the window's and then the lifetime counters. Then the window advances and emits at its
    /// period, exactly as the loop would. Every other block ignores `banked` and takes the loop, so
    /// every published word is the one [`Self::observe_input`] would publish.
    ///
    /// The seed check is what keeps the protocol safe: whoever computed `banked` read the seed from
    /// this meter's own state through [`Self::banked_seed`], and this meter accepts the result
    /// only for that state. A wrong seed can only make the block fall back to the loop.
    ///
    /// # Errors
    /// [`MeterObservationError::SampleTimeOverflow`] before any state mutation if the
    /// block would run past `u64::MAX`.
    pub fn observe_input_banked(
        &mut self,
        input: MeterInput<'_>,
        first_sample: u64,
        banked: Option<MeterBankedBlock>,
    ) -> Result<(), MeterObservationError> {
        self.observe_block(input, first_sample, None, banked)
    }

    /// The one body behind [`Self::observe_input`], [`Self::observe_input_with_block_peak`] and
    /// [`Self::observe_input_banked`]: the preamble, then a banked commit, a block-peak merge or
    /// the scalar loop.
    fn observe_block(
        &mut self,
        input: MeterInput<'_>,
        first_sample: u64,
        block_peak: Option<[f32; 2]>,
        banked: Option<MeterBankedBlock>,
    ) -> Result<(), MeterObservationError> {
        let len = match u64::try_from(input.frames)
            .ok()
            .and_then(|len| first_sample.checked_add(len))
        {
            Some(_) => input.frames,
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
        // Issue #950: the bank computed this block's partials and this meter's seeded energy, and
        // the commit checks that the block and the seed are this meter's own.
        if let Some(banked) = banked
            && self.commit_banked(banked, len, period)
        {
            return Ok(());
        }
        // Issue #943: the block's peak arrives precomputed and the whole block lies inside this
        // window, so the window's peak is one select-form merge per channel and the frame count.
        // Nothing else a `SAMPLE_PEAK` window keeps reads a sample.
        if let Some([left, right]) = block_peak
            && self.metrics == MeterMetricSet::SAMPLE_PEAK
            && len > 0
            && len <= (period - self.frames) as usize
        {
            #[cfg(any(test, feature = "test-support"))]
            METER_BLOCK_PEAK_MERGES.with(|merges| merges.set(merges.get().saturating_add(1)));
            self.left.peak = if left > self.left.peak {
                left
            } else {
                self.left.peak
            };
            self.right.peak = if right > self.right.peak {
                right
            } else {
                self.right.peak
            };
            self.frames = self.frames.saturating_add(len as u32);
            if self.frames == period {
                self.emit();
            }
            return Ok(());
        }
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
        let settled_silence = self.metrics == MeterMetricSet::ALL
            && self.left.held == 0.0
            && self.right.held == 0.0
            && self.left.hold_remaining == window.hold_frames
            && self.right.hold_remaining == window.hold_frames
            && input
                .samples(input.left, 0, len)
                .all(|sample| sample == 0.0)
            && input
                .samples(input.right, 0, len)
                .all(|sample| sample == 0.0);
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
            if self.metrics == MeterMetricSet::ALL {
                observe_segment(
                    &mut self.left,
                    input.samples(input.left, offset, end),
                    window,
                    &mut self.cumulative_clipped,
                    &mut self.cumulative_sanitized,
                );
                observe_segment(
                    &mut self.right,
                    input.samples(input.right, offset, end),
                    window,
                    &mut self.cumulative_clipped,
                    &mut self.cumulative_sanitized,
                );
            } else {
                observe_selected_segment(
                    &mut self.left,
                    input.samples(input.left, offset, end),
                    window,
                    self.metrics,
                    &mut self.cumulative_clipped,
                    &mut self.cumulative_sanitized,
                );
                observe_selected_segment(
                    &mut self.right,
                    input.samples(input.right, offset, end),
                    window,
                    self.metrics,
                    &mut self.cumulative_clipped,
                    &mut self.cumulative_sanitized,
                );
            }
            self.frames = self.frames.saturating_add(take as u32);
            offset = end;
            if self.frames == period {
                self.emit();
            }
        }
        Ok(())
    }

    /// Commit one banked block after the preamble, or return `false` and leave the meter untouched
    /// (issue #950). See [`Self::observe_input_banked`] for the conditions and the commit.
    fn commit_banked(&mut self, banked: MeterBankedBlock, len: usize, period: u32) -> bool {
        let seeded = !self.metrics.contains(MeterMetricSet::ENERGY_RMS)
            || (banked.energy_seed[0].to_bits() == self.left.energy.to_bits()
                && banked.energy_seed[1].to_bits() == self.right.energy.to_bits());
        if !self.banked_eligible()
            || len == 0
            || len > period.saturating_sub(self.frames) as usize
            || !seeded
        {
            return false;
        }
        #[cfg(any(test, feature = "test-support"))]
        METER_BANKED_COMMITS.with(|commits| commits.set(commits.get().saturating_add(1)));
        commit_banked_lane(
            &mut self.left,
            self.metrics,
            banked.sample_peak[0],
            banked.clipped[0],
            banked.sanitized[0],
            banked.energy[0],
            &mut self.cumulative_clipped,
            &mut self.cumulative_sanitized,
        );
        commit_banked_lane(
            &mut self.right,
            self.metrics,
            banked.sample_peak[1],
            banked.clipped[1],
            banked.sanitized[1],
            banked.energy[1],
            &mut self.cumulative_clipped,
            &mut self.cumulative_sanitized,
        );
        self.frames = self.frames.saturating_add(len as u32);
        if self.frames == period {
            self.emit();
        }
        true
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

    /// The metric selection this meter was prepared with.
    #[must_use]
    pub const fn metrics(&self) -> MeterMetricSet {
        self.metrics
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
            present_metrics: self.metrics,
            reset_generation: self.config.reset_generation,
            window_sequence: self.sequence,
            start_sample: start,
            end_sample: end,
            frames: self.frames,
            left: lane_snapshot(&self.left, self.frames, self.metrics),
            right: lane_snapshot(&self.right, self.frames, self.metrics),
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

// REALTIME_POLICY_BEGIN
/// One channel of a banked commit (issue #950): what [`observe_segment`] and
/// [`observe_selected_segment`] would leave in `lane` and the lifetime counters after one segment,
/// from that segment's partials.
///
/// * `SAMPLE_PEAK`: `peak` merges `q` with the loop's `if a > p` form. On the sanitized domain
///   `{+0.0} ∪ [MIN_POSITIVE, MAX]` that select form is associative and commutative by bits, so
///   merging the block's maximum equals the loop.
/// * `HELD_PEAK`, reached only without a hold or a decay: the loop's state machine is then
///   `held = a >= held ? a : held` with `hold_remaining` fixed at `0`, the same maximum merged with
///   its own `>=` form; `hold_remaining` is not touched.
/// * `ENERGY_RMS`: `energy` is replaced by the seeded, sample-serial sum. Replaced, never added
///   to: adding a zero-seeded partial rounds differently.
/// * `COUNTS`: the window's counts, then the lifetime counts, `saturating_add` the block's, as the
///   loop does.
#[expect(
    clippy::too_many_arguments,
    reason = "one channel's four partials and the two lifetime counters it shares with its sibling"
)]
fn commit_banked_lane(
    lane: &mut MeterLane,
    metrics: MeterMetricSet,
    q: f32,
    clipped: u64,
    sanitized: u64,
    energy: f64,
    cumulative_clipped: &mut u64,
    cumulative_sanitized: &mut u64,
) {
    if metrics.contains(MeterMetricSet::SAMPLE_PEAK) {
        lane.peak = if q > lane.peak { q } else { lane.peak };
    }
    if metrics.contains(MeterMetricSet::ENERGY_RMS) {
        lane.energy = energy;
    }
    if metrics.contains(MeterMetricSet::COUNTS) {
        lane.clipped = lane.clipped.saturating_add(clipped);
        lane.sanitized = lane.sanitized.saturating_add(sanitized);
        *cumulative_clipped = cumulative_clipped.saturating_add(clipped);
        *cumulative_sanitized = cumulative_sanitized.saturating_add(sanitized);
    }
    if metrics.contains(MeterMetricSet::HELD_PEAK) {
        lane.held = if q >= lane.held { q } else { lane.held };
    }
}
// REALTIME_POLICY_END

/// Accumulates one lane over a segment that lies entirely inside one meter window.
///
/// Branch-free per sample except for the held-peak state machine, which is a three-way scalar
/// choice on counters rather than on sample values. `peak` is the D8 select form
/// `select(a > p, a, p)`, never `f32::max`: the two disagree on `+/-0.0` ordering, and `f32::max`
/// is forbidden on any path whose bits are pinned.
fn observe_segment(
    lane: &mut MeterLane,
    samples: impl Iterator<Item = f32> + Clone,
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
    for sample in samples.clone() {
        let invalid = !normal_or_zero(sample);
        let sample = if invalid { 0.0 } else { sample };
        sanitized += u64::from(invalid);
        let absolute = sample.abs();
        peak = if absolute > peak { absolute } else { peak };
        #[cfg(test)]
        meter_work_probe::ENERGY.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        energy += f64::from(sample) * f64::from(sample);
        #[cfg(test)]
        meter_work_probe::COUNTS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        clipped += u64::from(absolute >= 1.0);
        #[cfg(test)]
        meter_work_probe::HELD.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
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

/// Partial selections use independent straight-line passes selected once per segment. The full
/// selection stays on [`observe_segment`] so its arithmetic order and published bits do not move.
fn observe_selected_segment(
    lane: &mut MeterLane,
    samples: impl Iterator<Item = f32> + Clone,
    window: MeterWindow,
    metrics: MeterMetricSet,
    cumulative_clipped: &mut u64,
    cumulative_sanitized: &mut u64,
) {
    if metrics.contains(MeterMetricSet::SAMPLE_PEAK) {
        let mut peak = lane.peak;
        for sample in samples.clone() {
            #[cfg(any(test, feature = "test-support"))]
            METER_PEAK_SAMPLES.with(|samples| samples.set(samples.get().saturating_add(1)));
            let sample = if normal_or_zero(sample) { sample } else { 0.0 };
            let absolute = sample.abs();
            peak = if absolute > peak { absolute } else { peak };
        }
        lane.peak = peak;
    }
    if metrics.contains(MeterMetricSet::ENERGY_RMS) {
        let mut energy = lane.energy;
        for sample in samples.clone() {
            let sample = if normal_or_zero(sample) { sample } else { 0.0 };
            #[cfg(test)]
            meter_work_probe::ENERGY.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
            energy += f64::from(sample) * f64::from(sample);
        }
        lane.energy = energy;
    }
    if metrics.contains(MeterMetricSet::COUNTS) {
        let mut clipped = 0_u64;
        let mut sanitized = 0_u64;
        for sample in samples.clone() {
            #[cfg(test)]
            meter_work_probe::COUNTS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
            let invalid = !normal_or_zero(sample);
            let sample = if invalid { 0.0 } else { sample };
            sanitized += u64::from(invalid);
            clipped += u64::from(sample.abs() >= 1.0);
        }
        lane.clipped = lane.clipped.saturating_add(clipped);
        lane.sanitized = lane.sanitized.saturating_add(sanitized);
        *cumulative_clipped = cumulative_clipped.saturating_add(clipped);
        *cumulative_sanitized = cumulative_sanitized.saturating_add(sanitized);
    }
    if metrics.contains(MeterMetricSet::HELD_PEAK) {
        let mut held = lane.held;
        let mut hold_remaining = lane.hold_remaining;
        for sample in samples.clone() {
            #[cfg(test)]
            meter_work_probe::HELD.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
            let sample = if normal_or_zero(sample) { sample } else { 0.0 };
            let absolute = sample.abs();
            if absolute >= held {
                held = absolute;
                hold_remaining = window.hold_frames;
            } else if hold_remaining > 0 {
                hold_remaining -= 1;
            } else if window.decay_enabled {
                held = flush_subnormal(held * window.decay);
            }
        }
        lane.held = held;
        lane.hold_remaining = hold_remaining;
    }
}

fn lane_snapshot(lane: &MeterLane, frames: u32, metrics: MeterMetricSet) -> MeterLaneSnapshot {
    if metrics == MeterMetricSet::ALL {
        #[cfg(test)]
        meter_work_probe::SQRT.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        return MeterLaneSnapshot {
            sample_peak: lane.peak,
            rms: (lane.energy / f64::from(frames)).sqrt(),
            energy: lane.energy,
            held_peak: lane.held,
            clipped_samples: lane.clipped,
            sanitized_samples: lane.sanitized,
        };
    }
    MeterLaneSnapshot {
        sample_peak: if metrics.contains(MeterMetricSet::SAMPLE_PEAK) {
            lane.peak
        } else {
            0.0
        },
        rms: if metrics.contains(MeterMetricSet::ENERGY_RMS) {
            #[cfg(test)]
            meter_work_probe::SQRT.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
            (lane.energy / f64::from(frames)).sqrt()
        } else {
            0.0
        },
        energy: if metrics.contains(MeterMetricSet::ENERGY_RMS) {
            lane.energy
        } else {
            0.0
        },
        held_peak: if metrics.contains(MeterMetricSet::HELD_PEAK) {
            lane.held
        } else {
            0.0
        },
        clipped_samples: if metrics.contains(MeterMetricSet::COUNTS) {
            lane.clipped
        } else {
            0
        },
        sanitized_samples: if metrics.contains(MeterMetricSet::COUNTS) {
            lane.sanitized
        } else {
            0
        },
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
        BuiltinChain, BuiltinFaderBank, BuiltinInputBank, BuiltinMatrixBank, BuiltinParameterError,
        FaderMuteRampBuiltins, FaderStageKernel, InputBuiltins, InputStageKernel, Matrix2x2,
        MatrixBuiltins, MatrixStageKernel, SvfSection, lane_read,
    };

    /// How many design bounds (#1329 D4) preparation has computed on the calling thread: one
    /// per distinct design it bounded (#1329 Amendment 5, MJ1). Thread-scoped, so a test reads
    /// the difference across its own preparation.
    #[cfg(any(test, feature = "test-support"))]
    #[must_use]
    pub fn fixed_input_bounds_computed() -> u64 {
        crate::tail::FIXED_INPUT_BOUNDS.with(core::cell::Cell::get)
    }

    /// How many frames this thread's design-bound walks have taken, stopped walks included, as
    /// counted frame by frame by the walk itself (#1457 D1, attempt 1 MJ2). Read the difference
    /// across one preparation.
    #[cfg(any(test, feature = "test-support"))]
    #[must_use]
    pub fn fixed_input_frames_walked() -> u64 {
        crate::tail::FIXED_INPUT_FRAMES.with(core::cell::Cell::get)
    }

    /// The charges, in frame-equivalents, of the design bounds this thread computed (#1457
    /// Amendment 3): what a preparation that computes every design it bounds spends of its budget.
    /// Read the difference across one preparation.
    #[cfg(any(test, feature = "test-support"))]
    #[must_use]
    pub fn fixed_input_charged() -> u64 {
        crate::tail::FIXED_INPUT_CHARGED.with(core::cell::Cell::get)
    }

    /// [`crate::input_section_bounds`] under a budget of `budget` frame-equivalents instead of
    /// [`crate::INPUT_BOUND_BUDGET_FRAMES`]: for the gates only (#1457 ruling (i): production has
    /// no entry point that takes a budget).
    ///
    /// # Errors
    ///
    /// The first strip's error, as [`crate::input_section_bound`] reports it.
    #[cfg(any(test, feature = "test-support"))]
    pub fn input_section_bounds_within(
        sample_rate: u32,
        strips: impl IntoIterator<Item = crate::BuiltinParameters>,
        budget: u64,
        cache: Option<&mut crate::InputBoundCache>,
    ) -> Result<Vec<crate::InputSectionBound>, BuiltinParameterError> {
        super::input_section_bounds_within(sample_rate, strips, budget, cache)
    }

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

    /// [`input_section_words`] for one lane of a bank.
    #[must_use]
    pub fn bank_section_words(bank: &BuiltinInputBank, lane: usize) -> [[u32; 7]; 4] {
        let track = per_width!(InputStageKernel(stage) in &bank.stage => stage.lane_track(lane));
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
        per_width!(InputStageKernel(stage) in &bank.stage => stage.trim_ramp_words(lane))
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
        per_width!(InputStageKernel(stage) in &bank.stage => stage.elision_plan())
    }

    /// Retained state words of one bank lane, in the [`input_state_words`] order.
    #[must_use]
    pub fn bank_lane_state_words(bank: &BuiltinInputBank, lane: usize) -> [u32; 8] {
        per_width!(InputStageKernel(stage) in &bank.stage => stage.lane_state_words(lane))
    }

    /// Cumulative per-channel recovered-lane counts of a bank, `[left, right]`.
    ///
    /// The one piece of a block's accounting that survives the call: `BuiltinProcessReport` is
    /// dropped by the graph adapter, so a collapsed body that fed only the left counter would be
    /// invisible everywhere else. `mono_collapse::the_collapsed_body_publishes_the_dual_bodys_report`
    /// is the gate.
    #[must_use]
    pub fn bank_lifetime_recovered(bank: &BuiltinInputBank) -> [u64; 2] {
        per_width!(InputStageKernel(stage) in &bank.stage => stage.lifetime_recovered)
    }

    /// The input's silence counter of one bank lane, `[left, right]`, as `f32` bits (issue #1328,
    /// amendment A9).
    #[must_use]
    pub fn bank_lane_silence_words(bank: &BuiltinInputBank, lane: usize) -> [u32; 2] {
        per_width!(InputStageKernel(stage) in &bank.stage => {
            stage
                .state
                .silence
                .map(|run| lane_read(run)[lane].to_bits())
        })
    }

    /// Overwrites the retained state words of one bank lane.
    pub fn set_bank_lane_state_words(bank: &mut BuiltinInputBank, lane: usize, words: [u32; 8]) {
        per_width!(InputStageKernel(stage) in &mut bank.stage => {
            stage.set_lane_state_words(lane, words)
        })
    }

    /// The current (applied) matrix of a scalar matrix section.
    #[must_use]
    pub fn matrix_current(matrix: &MatrixBuiltins) -> Matrix2x2 {
        matrix.stage.read_current()[0]
    }

    /// Exact retained words for the live scalar fader owner.
    #[must_use]
    pub fn scalar_fader_words(fader: &FaderMuteRampBuiltins) -> [u32; 14] {
        let stage = &fader.stage;
        let mut out = [0; 14];
        for channel in 0..2 {
            let base = channel * 6;
            out[base] = stage.ramp[channel].current.to_bits();
            out[base + 1] = stage.ramp[channel].target.to_bits();
            out[base + 2] = stage.ramp[channel].step.to_bits();
            out[base + 3] = stage.ramp[channel].remaining.to_bits();
            out[base + 4] = stage.remaining[channel][0];
            out[base + 5] = stage.fader_gain[channel][0].to_bits();
        }
        out[12] = u32::from(stage.muted[0][0]);
        out[13] = u32::from(stage.muted[1][0]);
        out
    }

    /// Exact retained words for the live scalar matrix owner.
    #[must_use]
    pub fn scalar_matrix_words(matrix: &MatrixBuiltins) -> [u32; 15] {
        let stage = &matrix.stage;
        let current = stage.read_current()[0];
        let target = stage.read_target()[0];
        let mut out = [0; 15];
        for (coefficient, (current, target)) in [
            (current.ll, target.ll),
            (current.lr, target.lr),
            (current.rl, target.rl),
            (current.rr, target.rr),
        ]
        .into_iter()
        .enumerate()
        {
            out[coefficient] = current.to_bits();
            out[4 + coefficient] = target.to_bits();
            out[8 + coefficient] = stage.ramp.step[coefficient].to_bits();
        }
        out[12] = stage.ramp.remaining.to_bits();
        out[13] = stage.remaining[0];
        out[14] = stage.smoothing_samples[0];
        out
    }

    /// Exact retained words for one fader-bank lane: per channel current/target/step/ramp word,
    /// authoritative countdown and remembered gain, followed by both mute flags.
    #[must_use]
    pub fn fader_bank_lane_words(bank: &BuiltinFaderBank, lane: usize) -> [u32; 14] {
        fn words<L: crate::Lane>(stage: &crate::FaderRampStage<L>, lane: usize) -> [u32; 14] {
            let mut out = [0; 14];
            for channel in 0..2 {
                let base = channel * 6;
                out[base] = crate::lane_read::<L>(stage.ramp[channel].current)[lane].to_bits();
                out[base + 1] = crate::lane_read::<L>(stage.ramp[channel].target)[lane].to_bits();
                out[base + 2] = crate::lane_read::<L>(stage.ramp[channel].step)[lane].to_bits();
                out[base + 3] =
                    crate::lane_read::<L>(stage.ramp[channel].remaining)[lane].to_bits();
                out[base + 4] = stage.remaining[channel][lane];
                out[base + 5] = stage.fader_gain[channel][lane].to_bits();
            }
            out[12] = u32::from(stage.muted[0][lane]);
            out[13] = u32::from(stage.muted[1][lane]);
            out
        }
        per_width!(FaderStageKernel(stage) in &bank.stage => words(stage, lane))
    }

    /// Exact current/target/step/ramp/countdown words for one matrix-bank lane.
    #[must_use]
    pub fn matrix_bank_lane_words(bank: &BuiltinMatrixBank, lane: usize) -> [u32; 15] {
        fn words<L: crate::Lane>(stage: &crate::MatrixStage<L>, lane: usize) -> [u32; 15] {
            let current = stage.read_current();
            let target = stage.read_target();
            let step = stage.ramp.step.map(crate::lane_read::<L>);
            let remaining = crate::lane_read::<L>(stage.ramp.remaining);
            let mut out = [0; 15];
            for (coefficient, (current, target)) in [
                (current[lane].ll, target[lane].ll),
                (current[lane].lr, target[lane].lr),
                (current[lane].rl, target[lane].rl),
                (current[lane].rr, target[lane].rr),
            ]
            .into_iter()
            .enumerate()
            {
                out[coefficient] = current.to_bits();
                out[4 + coefficient] = target.to_bits();
                out[8 + coefficient] = step[coefficient][lane].to_bits();
            }
            out[12] = remaining[lane].to_bits();
            out[13] = stage.remaining[lane];
            out[14] = stage.smoothing_samples[lane];
            out
        }
        per_width!(MatrixStageKernel(stage) in &bank.stage => words(stage, lane))
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
mod tests;
