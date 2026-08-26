//! Semantic, render-safe native effect runtime contract V1.
//!
//! This crate deliberately has no descriptor wire format, digest, package, CID, parser, or
//! persistence envelope. Those are interchange concerns owned by issue 029.
#![allow(missing_docs)]

mod live;
pub use live::{BypassShunt, EffectControlLane, EffectControlRecordV1, ObservationLaneV1, Staged};

use core::{fmt, hash::Hash};
use miso_engine_core::{
    LAUNCH_SAMPLE_RATES, SampleRateHz, is_extended_compatibility_sample_rate, is_launch_sample_rate,
};
use miso_engine_lane::{Backend, Simd4, Simd8};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StaticIdError {
    Empty,
    TooLong,
    FirstCharacter,
    Character,
}
fn valid_static_id(value: &str) -> Result<(), StaticIdError> {
    let bytes = value.as_bytes();
    if bytes.is_empty() {
        return Err(StaticIdError::Empty);
    }
    if bytes.len() > 127 {
        return Err(StaticIdError::TooLong);
    }
    if !bytes[0].is_ascii_lowercase() {
        return Err(StaticIdError::FirstCharacter);
    }
    if bytes[1..]
        .iter()
        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'.' | b'_' | b'-'))
    {
        Ok(())
    } else {
        Err(StaticIdError::Character)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EffectId(&'static str);
impl EffectId {
    pub fn parse(value: &'static str) -> Option<Self> {
        Self::new(value).ok()
    }
    pub const fn new(value: &'static str) -> Result<Self, StaticIdError> {
        let b = value.as_bytes();
        if b.is_empty() {
            return Err(StaticIdError::Empty);
        }
        if b.len() > 127 {
            return Err(StaticIdError::TooLong);
        }
        if !(b[0] >= b'a' && b[0] <= b'z') {
            return Err(StaticIdError::FirstCharacter);
        }
        let mut i = 1;
        while i < b.len() {
            let x = b[i];
            if !((x >= b'a' && x <= b'z')
                || (x >= b'0' && x <= b'9')
                || x == b'.'
                || x == b'_'
                || x == b'-')
            {
                return Err(StaticIdError::Character);
            }
            i += 1
        }
        Ok(Self(value))
    }
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}
impl fmt::Display for EffectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PortId(&'static str);
impl PortId {
    pub const fn new(value: &'static str) -> Result<Self, StaticIdError> {
        match EffectId::new(value) {
            Ok(id) => Ok(Self(id.0)),
            Err(e) => Err(e),
        }
    }
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}
impl fmt::Display for PortId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParameterId(pub u32);
impl ParameterId {
    pub const fn new(v: u32) -> Option<Self> {
        if v == 0 { None } else { Some(Self(v)) }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LatencySamples(pub u64);
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TailSamples {
    Finite(u64),
    Infinite,
}
macro_rules! scalar_enum { ($name:ident {$($v:ident=$n:expr),+$(,)?})=>{#[repr(u32)]#[derive(Clone,Copy,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]pub enum $name{$($v=$n),+}impl $name{pub const fn from_raw(v:u32)->Option<Self>{match v{$($n=>Some(Self::$v),)+_=>None}}}}; }
scalar_enum!(ParameterUnit {Db=1,Hz=2,Milliseconds=3,Samples=4,Linear=5,Ratio=6});
scalar_enum!(ParameterDomain {Continuous=1,Boolean=2,Enumeration=3});
scalar_enum!(ParameterMapping {Linear=1,Logarithmic=2,Exponential=3,Stepped=4});
scalar_enum!(AutomationRate {Sample=1,Block=2,None=3});
scalar_enum!(ParameterChannel {Left=1,Right=2,Both=3});
scalar_enum!(EffectQuality {Draft=1,Normal=2,High=3});
pub type Quality = EffectQuality;
scalar_enum!(LinkMode {DualMono=1,Maximum=2,Average=3});
scalar_enum!(ResetKind {FullToDefaults=1,DiscontinuityKeepParameters=2});
scalar_enum!(AutomationSpanKind {Point=1,Step=2,Linear=3,Exponential=4});
pub type AutomationKind = AutomationSpanKind;
scalar_enum!(ParameterChannelPolicy {Shared=1,PerLane=2});
// Issue #143 D1: the declared observation menu. Each vocabulary is a `scalar_enum!` for the same
// reason the parameter vocabularies are -- the descriptor wire, the C inspect surface and the
// browser metadata all carry the raw `u32`, and `from_raw` is the single place a foreign value is
// refused rather than silently reinterpreted.
// What an observation tap reports. One kind ships in V1.
scalar_enum!(ObservationKindV1 {GainReductionDb=1});
// What publishing an observation costs.
//
// `Resident` means the value already exists in kernel state when the block ends: publishing is a
// copy out of state that `process` wrote anyway, and no lane kernel changes. `Computed` means an
// analysis pass that would not otherwise run; V1 declares the class, validates it, and refuses to
// bind one (`ObservationUnbound`/`UnsupportedKind` on the control plane).
scalar_enum!(ObservationCostV1 {Resident=1,Computed=2});
// How often a tap produces a value.
scalar_enum!(ObservationCadenceV1 {PerBlock=1,PerWindow=2});
// How a window of per-block values folds into the one number a consumer reads.
//
// `PeakMagnitude` is `max(|x|)` over the window, which is what makes a gain-reduction tap publish
// a **non-negative magnitude** even though the contract's internal sign convention is
// negative-for-reduction ([`GainReductionV1`]). The published value is non-negative *by the
// declared fold*, not by a host convention nobody can check.
scalar_enum!(ObservationFoldV1 {Latest=1,PeakMagnitude=2});
// Whether a tap publishes one value per instance or one per dual-mono lane.
scalar_enum!(ObservationChannelsV1 {Shared=1,PerLane=2});
/// An effect-local observation tap identifier: nonzero, and ascending within a descriptor.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ObservationTapId(pub u32);
impl ObservationTapId {
    /// `None` for zero, which the addressing ABI reserves as "no tap".
    #[must_use]
    pub const fn new(v: u32) -> Option<Self> {
        if v == 0 { None } else { Some(Self(v)) }
    }
}
scalar_enum!(SmoothingRule {None=1,Linear=2,OnePole99=3});
scalar_enum!(PortRole {MainInput=1,MainOutput=2,SidechainInput=3});
pub type PortKind = PortRole;
scalar_enum!(PortLayout {DualMonoPlanar=1});
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BankWidth {
    Four,
    Eight,
}
impl BankWidth {
    pub const fn lanes(self) -> u32 {
        match self {
            Self::Four => 4,
            Self::Eight => 8,
        }
    }
    /// The bank width a backend executes, or `None` for the one-lane scalar path.
    ///
    /// This is the workspace's single backend-to-width law (#84 phase A): every crate that needs
    /// the width of a [`Backend`] calls this rather than re-deriving a table of its own.
    #[must_use]
    pub const fn for_backend(backend: Backend) -> Option<Self> {
        match backend {
            Backend::Scalar => None,
            Backend::Simd4 => Some(Self::Four),
            Backend::Simd8 => Some(Self::Eight),
        }
    }

    /// Whether this non-scalar bank width is legal for a selected backend.
    #[must_use]
    pub const fn matches_backend(self, backend: Backend) -> bool {
        matches!(
            (self, Self::for_backend(backend)),
            (Self::Four, Some(Self::Four)) | (Self::Eight, Some(Self::Eight))
        )
    }
}

/// Transpose one four-by-four tile of 32-bit words: `out[k][lane] == rows[lane][k]`.
///
/// A transpose is a **pure permutation of 32-bit words** -- no arithmetic, no rounding, no
/// canonicalisation -- so every NaN payload, every `-0.0` and every subnormal survives it bit for
/// bit. That is what lets the planar/AoSoA round trip of a full bank be one whole-tile vector
/// shuffle instead of a scalar move per lane-sample, with no effect on a rendered bit.
///
/// The vocabulary is the lane crate's neutral re-export ([`Simd4`]), whose `transpose` is `wide`'s
/// **safe** shuffle network: `_MM_TRANSPOSE4_PS` on SSE, a dedicated `simd128`/NEON unpack arm, and
/// a scalar element permutation everywhere else. The workspace denies `unsafe_code`; none is
/// needed here. This lives beside [`BankWidth`] because the tile is a fact about the AoSoA layout
/// the contract defines, and because the SIMD vocabulary is the contract's dependency, not the
/// rack's (`scripts/check-lane-policy.sh` D4, `scripts/check-rack-policy.sh`).
#[inline(always)]
#[must_use]
pub fn transpose_tile_4(rows: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    Simd4::transpose(rows.map(Simd4::new)).map(Simd4::to_array)
}

/// Transpose one eight-by-eight tile of 32-bit words: `out[k][lane] == rows[lane][k]`.
///
/// The eight-lane twin of [`transpose_tile_4`]; see it for why this is bit-exact. On `x86-64-v3`
/// [`Simd8`]'s `transpose` is the 24-shuffle AVX pattern (eight `unpack`, eight `shuffle`, eight
/// `permute2f128`); on a target where `wide` lowers `f32x8` to two 128-bit halves it is the scalar
/// element permutation, which is equally bit-exact and equally correct.
#[inline(always)]
#[must_use]
pub fn transpose_tile_8(rows: [[f32; 8]; 8]) -> [[f32; 8]; 8] {
    Simd8::transpose(rows.map(Simd8::new)).map(Simd8::to_array)
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LinkModeSet(u32);
impl LinkModeSet {
    pub const DUAL_MONO: Self = Self(1);
    pub const ALL: Self = Self(7);
    pub const fn new(v: u32) -> Option<Self> {
        if v & !7 != 0 || v & 1 == 0 {
            None
        } else {
            Some(Self(v))
        }
    }
    pub const fn bits(self) -> u32 {
        self.0
    }
    pub const fn contains(self, m: LinkMode) -> bool {
        self.0 & (1 << (m as u32 - 1)) != 0
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StatePayloadSizes {
    pub common_bytes: u32,
    pub left_bytes: u32,
    pub right_bytes: u32,
}
impl StatePayloadSizes {
    pub const fn total(self) -> Option<u64> {
        match (self.common_bytes as u64).checked_add(self.left_bytes as u64) {
            Some(v) => v.checked_add(self.right_bytes as u64),
            None => None,
        }
    }

    /// The one exact-length check for a three-section payload.
    ///
    /// The audit found this comparison written out four times in this crate alone and once more
    /// per effect crate, with four different diagnostic strings. It is one function now, and
    /// `effect.state.length` is its one code.
    ///
    /// # Errors
    ///
    /// `effect.state.length` if any section length differs from the prepared size.
    pub const fn check(
        self,
        common: usize,
        left: usize,
        right: usize,
    ) -> Result<(), StatePayloadError> {
        if common != self.common_bytes as usize
            || left != self.left_bytes as usize
            || right != self.right_bytes as usize
        {
            return Err(StatePayloadError {
                code: "effect.state.length",
            });
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnumChoiceV1 {
    pub value: f32,
    pub label: &'static str,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParameterDescriptorV1 {
    pub id: ParameterId,
    pub display_name: &'static str,
    pub display_unit: &'static str,
    pub unit: ParameterUnit,
    pub domain: ParameterDomain,
    pub minimum: Option<f32>,
    pub maximum: Option<f32>,
    pub default_value: f32,
    pub mapping: ParameterMapping,
    pub automation_rate: AutomationRate,
    pub channel_policy: ParameterChannelPolicy,
    pub smoothing: SmoothingRule,
    pub smoothing_samples: u32,
    pub readable: bool,
    pub automatable: bool,
    pub enum_choices: &'static [EnumChoiceV1],
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PortDescriptorV1 {
    pub id: PortId,
    pub role: PortRole,
    pub required: bool,
    pub layout: PortLayout,
}
/// One declared observation tap: what an effect will let a console watch, and what watching costs.
///
/// # Why a declared menu rather than a host-side table
///
/// The effect is the only thing that knows which of its internal values exist, what they mean and
/// whether reading one is a copy or a second computation. A host that kept its own table would be
/// a second source of truth that goes stale the moment a kernel changes. Every consumer -- the
/// subscribe path, the descriptor wire, the browser metadata -- reads this and nothing else.
///
/// `minimum`/`maximum` are the declared bounds of the **published** value, after the declared
/// [`fold`](Self::fold). A gain-reduction tap therefore declares `0 .. 100`, not `-100 .. 0`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ObservationDescriptorV1 {
    /// Effect-local tap id: nonzero, unique, and strictly ascending in declaration order.
    pub id: ObservationTapId,
    /// Human-facing name, e.g. `"Gain Reduction"`.
    pub display_name: &'static str,
    /// Human-facing unit suffix, e.g. `"dB"`.
    pub display_unit: &'static str,
    /// What the tap reports.
    pub kind: ObservationKindV1,
    /// The unit the effect publishes in. A `Linear` tap is converted once per window on the
    /// control plane, never on the render thread (issue #143 R4).
    pub unit: ParameterUnit,
    /// What publishing it costs.
    pub cost: ObservationCostV1,
    /// How often it produces a value.
    pub cadence: ObservationCadenceV1,
    /// How a window of values folds into the number a consumer reads.
    pub fold: ObservationFoldV1,
    /// One value per instance, or one per dual-mono lane.
    pub channels: ObservationChannelsV1,
    /// Declared inclusive lower bound of the published value.
    pub minimum: f32,
    /// Declared inclusive upper bound of the published value.
    pub maximum: f32,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QualityDescriptorV1 {
    pub quality: EffectQuality,
    pub sample_rate: u32,
    pub latency: LatencySamples,
    pub tail: TailSamples,
    pub maximum_state: StatePayloadSizes,
    pub scratch_fixed_bytes: u64,
    pub scratch_bytes_per_frame: u64,
}
#[derive(Clone, Copy, Debug)]
pub struct EffectDescriptorV1 {
    pub id: EffectId,
    pub display_name: &'static str,
    pub contract_major: u16,
    pub contract_minor: u16,
    pub state_layout_version: u32,
    pub supported_link_modes: LinkModeSet,
    pub parameters: &'static [ParameterDescriptorV1],
    pub ports: &'static [PortDescriptorV1],
    pub qualities: &'static [QualityDescriptorV1],
    /// The declared observation menu (issue #143 D1). Last, and empty for every effect that
    /// declares no tap, so a zero-tap descriptor encodes byte-identically to the pre-#143 wire.
    pub observations: &'static [ObservationDescriptorV1],
}
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DescriptorDiagnosticCode {
    ContractMajor,
    StateLayoutVersion,
    Text,
    LinkModes,
    ParameterId,
    ParameterOrder,
    Parameter,
    Port,
    Quality,
    QualityOrder,
    StateSizes,
    ObservationId,
    ObservationOrder,
    Observation,
}
impl DescriptorDiagnosticCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContractMajor => "effect.descriptor.contract_major",
            Self::StateLayoutVersion => "effect.descriptor.state_layout_version",
            Self::Text => "effect.descriptor.text",
            Self::LinkModes => "effect.descriptor.link_modes",
            Self::ParameterId => "effect.descriptor.parameter_id",
            Self::ParameterOrder => "effect.descriptor.parameter_order",
            Self::Parameter => "effect.descriptor.parameter",
            Self::Port => "effect.descriptor.port",
            Self::Quality => "effect.descriptor.quality",
            Self::QualityOrder => "effect.descriptor.quality_order",
            Self::StateSizes => "effect.descriptor.state_sizes",
            Self::ObservationId => "effect.descriptor.observation_id",
            Self::ObservationOrder => "effect.descriptor.observation_order",
            Self::Observation => "effect.descriptor.observation",
        }
    }
}
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DescriptorError {
    pub path: &'static str,
    pub code: DescriptorDiagnosticCode,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DescriptorErrorSet(pub Vec<DescriptorError>);
impl DescriptorErrorSet {
    pub fn errors(&self) -> &[DescriptorError] {
        &self.0
    }
}
fn valid_text(v: &str) -> bool {
    !v.is_empty() && v.len() <= 255 && !v.chars().any(|c| c == '\0' || c == '\r' || c.is_control())
}
/// `to_bits`, with both zeros canonicalised to `+0.0`.
///
/// Every identity comparison in this crate goes through it, so `-0.0` and `+0.0` are one value
/// wherever a descriptor, an enum choice or a span endpoint is compared.
#[must_use]
pub fn canonical_bits(v: f32) -> u32 {
    if v == 0.0 {
        0.0f32.to_bits()
    } else {
        v.to_bits()
    }
}
/// `+0.0` for either zero, the value unchanged otherwise.
#[must_use]
pub fn normalize_zero(v: f32) -> f32 {
    if v == 0.0 { 0.0 } else { v }
}
/// `true` if `v` is the negative-zero bit pattern.
#[must_use]
pub fn is_negative_zero(v: f32) -> bool {
    v.to_bits() == (-0.0_f32).to_bits()
}
/// `true` if `v` is inside the descriptor parameter's declared domain.
///
/// The descriptor-domain predicate. `miso_engine_effect_runtime::params::parameter_value_valid` is
/// the render-side crate's predicate over its own small `ParameterSpec`; the two state the same
/// law over two descriptions and cannot be one function while the contract is `std` and
/// `effect-runtime` is `no_std` (see `scripts/check-effect-runtime-policy.sh`).
#[must_use]
pub fn parameter_value_valid(p: &ParameterDescriptorV1, v: f32) -> bool {
    if !v.is_finite() {
        return false;
    }
    match p.domain {
        ParameterDomain::Continuous => p
            .minimum
            .zip(p.maximum)
            .is_some_and(|(a, b)| v >= a && v <= b),
        ParameterDomain::Boolean => {
            canonical_bits(v) == canonical_bits(0.0) || canonical_bits(v) == canonical_bits(1.0)
        }
        ParameterDomain::Enumeration => p
            .enum_choices
            .iter()
            .any(|c| canonical_bits(c.value) == canonical_bits(v)),
    }
}
fn parameter_valid(p: &ParameterDescriptorV1) -> bool {
    if !valid_text(p.display_name)
        || !valid_text(p.display_unit)
        || !p.default_value.is_finite()
        || is_negative_zero(p.default_value)
    {
        return false;
    }
    if p.automation_rate == AutomationRate::None {
        if p.automatable || p.smoothing != SmoothingRule::None {
            return false;
        }
    } else if !p.automatable {
        return false;
    }
    if (p.smoothing == SmoothingRule::None) != (p.smoothing_samples == 0) {
        return false;
    }
    match p.domain {
        ParameterDomain::Continuous => match (p.minimum, p.maximum) {
            (Some(a), Some(b)) => {
                a.is_finite()
                    && b.is_finite()
                    && !is_negative_zero(a)
                    && !is_negative_zero(b)
                    && a < b
                    && parameter_value_valid(p, p.default_value)
                    && p.enum_choices.is_empty()
                    && matches!(
                        p.mapping,
                        ParameterMapping::Linear
                            | ParameterMapping::Logarithmic
                            | ParameterMapping::Exponential
                    )
                    && (p.mapping != ParameterMapping::Logarithmic || a > 0.0)
            }
            _ => false,
        },
        ParameterDomain::Boolean => {
            p.minimum.is_none()
                && p.maximum.is_none()
                && p.enum_choices.is_empty()
                && p.mapping == ParameterMapping::Stepped
                && parameter_value_valid(p, p.default_value)
        }
        ParameterDomain::Enumeration => {
            let mut labels = BTreeSet::new();
            p.minimum.is_none()
                && p.maximum.is_none()
                && p.mapping == ParameterMapping::Stepped
                && p.enum_choices.len() >= 2
                && p.enum_choices.windows(2).all(|w| {
                    w[0].value.is_finite()
                        && w[1].value.is_finite()
                        && !is_negative_zero(w[0].value)
                        && !is_negative_zero(w[1].value)
                        && w[0].value < w[1].value
                        && valid_text(w[0].label)
                        && labels.insert(w[0].label)
                })
                && p.enum_choices.last().is_some_and(|c| {
                    c.value.is_finite()
                        && !is_negative_zero(c.value)
                        && valid_text(c.label)
                        && labels.insert(c.label)
                })
                && parameter_value_valid(p, p.default_value)
        }
    }
}
/// The three rules an [`ObservationDescriptorV1`] must satisfy beyond text and bounds.
///
/// * **Text and bounds.** Both display strings are non-empty printable text, and the declared
///   published range is finite, ordered, and free of `-0.0` -- the same canonicalisation rule
///   every other descriptor float obeys, so an identity comparison never depends on a zero's sign.
/// * **A `Computed` tap may not claim `PerBlock`.** `Computed` is the class whose value does not
///   already exist when the block ends; claiming per-block cadence for it would put an analysis
///   pass on the render thread, which is precisely what the cost split exists to prevent.
/// * **A `PerLane` tap requires per-lane state to read.** The observation is a read of kernel
///   state, so an effect whose qualities declare no per-lane state (`left_bytes == 0`) cannot
///   produce two independent lanes and must declare `Shared`.
///
/// The third rule is the in-descriptor form of issue #143's "PerLane requires bank width": bank
/// width is not a descriptor field (it is a *factory* capability, `bind_homogeneous_bank`), so the
/// checkable statement is the one that actually catches the error -- a per-lane tap on an effect
/// with no per-lane state.
fn observation_valid(d: &EffectDescriptorV1, o: &ObservationDescriptorV1) -> bool {
    valid_text(o.display_name)
        && valid_text(o.display_unit)
        && o.minimum.is_finite()
        && o.maximum.is_finite()
        && !is_negative_zero(o.minimum)
        && !is_negative_zero(o.maximum)
        && o.minimum < o.maximum
        && !(matches!(o.cost, ObservationCostV1::Computed)
            && matches!(o.cadence, ObservationCadenceV1::PerBlock))
        && (!matches!(o.channels, ObservationChannelsV1::PerLane)
            || d.qualities.iter().all(|q| q.maximum_state.left_bytes > 0))
}
pub fn validate_descriptor_v1(d: &'static EffectDescriptorV1) -> Result<(), DescriptorErrorSet> {
    let mut e = Vec::new();
    if d.contract_major != 1 {
        e.push(DescriptorError {
            path: "descriptor.contract_major",
            code: DescriptorDiagnosticCode::ContractMajor,
        })
    }
    if d.state_layout_version == 0 {
        e.push(DescriptorError {
            path: "descriptor.state_layout_version",
            code: DescriptorDiagnosticCode::StateLayoutVersion,
        })
    }
    if valid_static_id(d.id.as_str()).is_err() || !valid_text(d.display_name) {
        e.push(DescriptorError {
            path: "descriptor",
            code: DescriptorDiagnosticCode::Text,
        })
    }
    if !d.supported_link_modes.contains(LinkMode::DualMono) {
        e.push(DescriptorError {
            path: "descriptor.supported_link_modes",
            code: DescriptorDiagnosticCode::LinkModes,
        })
    }
    let (mut prior, mut ids) = (0, BTreeSet::new());
    for p in d.parameters {
        if p.id.0 == 0 || !ids.insert(p.id.0) {
            e.push(DescriptorError {
                path: "parameters",
                code: DescriptorDiagnosticCode::ParameterId,
            })
        }
        if p.id.0 <= prior {
            e.push(DescriptorError {
                path: "parameters",
                code: DescriptorDiagnosticCode::ParameterOrder,
            })
        }
        prior = p.id.0;
        if !parameter_valid(p) {
            e.push(DescriptorError {
                path: "parameters",
                code: DescriptorDiagnosticCode::Parameter,
            })
        }
    }
    let (mut ports, mut input, mut output, mut side) = (BTreeSet::new(), 0, 0, 0);
    for p in d.ports {
        if valid_static_id(p.id.as_str()).is_err() || !ports.insert(p.id) {
            e.push(DescriptorError {
                path: "ports",
                code: DescriptorDiagnosticCode::Port,
            })
        }
        match p.role {
            PortRole::MainInput
                if p.id.as_str() == "main-in"
                    && p.required
                    && p.layout == PortLayout::DualMonoPlanar =>
            {
                input += 1
            }
            PortRole::MainOutput
                if p.id.as_str() == "main-out"
                    && p.required
                    && p.layout == PortLayout::DualMonoPlanar =>
            {
                output += 1
            }
            PortRole::SidechainInput
                if p.id.as_str() != "main-in"
                    && p.id.as_str() != "main-out"
                    && p.layout == PortLayout::DualMonoPlanar =>
            {
                side += 1
            }
            _ => e.push(DescriptorError {
                path: "ports",
                code: DescriptorDiagnosticCode::Port,
            }),
        }
    }
    if input != 1 || output != 1 || side > 1 || d.ports.len() != 2 + side {
        e.push(DescriptorError {
            path: "ports",
            code: DescriptorDiagnosticCode::Port,
        })
    }
    let (mut qprior, mut qrates) = (None, BTreeMap::<EffectQuality, BTreeSet<u32>>::new());
    for q in d.qualities {
        let key = (q.quality, q.sample_rate);
        if qprior.is_some_and(|p| key <= p) {
            e.push(DescriptorError {
                path: "qualities",
                code: DescriptorDiagnosticCode::QualityOrder,
            })
        }
        qprior = Some(key);
        let rate = SampleRateHz(q.sample_rate);
        if !(is_launch_sample_rate(rate) || is_extended_compatibility_sample_rate(rate)) {
            e.push(DescriptorError {
                path: "qualities",
                code: DescriptorDiagnosticCode::Quality,
            })
        }
        if q.maximum_state.left_bytes != q.maximum_state.right_bytes {
            e.push(DescriptorError {
                path: "qualities",
                code: DescriptorDiagnosticCode::StateSizes,
            })
        }
        qrates.entry(q.quality).or_default().insert(q.sample_rate);
    }
    if !qrates.contains_key(&EffectQuality::Normal) {
        e.push(DescriptorError {
            path: "qualities",
            code: DescriptorDiagnosticCode::Quality,
        })
    }
    for r in qrates.values() {
        if LAUNCH_SAMPLE_RATES.iter().any(|rate| !r.contains(&rate.0)) {
            e.push(DescriptorError {
                path: "qualities",
                code: DescriptorDiagnosticCode::Quality,
            })
        }
    }
    let (mut oprior, mut oids) = (0, BTreeSet::new());
    for o in d.observations {
        if o.id.0 == 0 || !oids.insert(o.id.0) {
            e.push(DescriptorError {
                path: "observations",
                code: DescriptorDiagnosticCode::ObservationId,
            })
        }
        if o.id.0 <= oprior {
            e.push(DescriptorError {
                path: "observations",
                code: DescriptorDiagnosticCode::ObservationOrder,
            })
        }
        oprior = o.id.0;
        if !observation_valid(d, o) {
            e.push(DescriptorError {
                path: "observations",
                code: DescriptorDiagnosticCode::Observation,
            })
        }
    }
    e.sort();
    e.dedup();
    if e.is_empty() {
        Ok(())
    } else {
        Err(DescriptorErrorSet(e))
    }
}
pub fn map_normalized(m: ParameterMapping, a: f32, b: f32, x: f32) -> Option<f32> {
    if !(a.is_finite() && b.is_finite() && a < b && x.is_finite() && (0.0..=1.0).contains(&x)) {
        return None;
    }
    if x == 0.0 {
        return Some(a);
    }
    if x == 1.0 {
        return Some(b);
    }
    match m {
        ParameterMapping::Linear => Some(a + x * (b - a)),
        ParameterMapping::Logarithmic if a > 0.0 => Some(a * miso_engine_math::powf(b / a, x)),
        ParameterMapping::Exponential => Some(a + (b - a) * x * x),
        _ => None,
    }
}
pub fn inverse_map_normalized(m: ParameterMapping, a: f32, b: f32, v: f32) -> Option<f32> {
    if !(a.is_finite() && b.is_finite() && a < b && v.is_finite() && v >= a && v <= b) {
        return None;
    }
    if canonical_bits(v) == canonical_bits(a) {
        return Some(0.0);
    }
    if canonical_bits(v) == canonical_bits(b) {
        return Some(1.0);
    }
    match m {
        ParameterMapping::Linear => Some((v - a) / (b - a)),
        ParameterMapping::Logarithmic if a > 0.0 => {
            Some(miso_engine_math::logf(v / a) / miso_engine_math::logf(b / a))
        }
        ParameterMapping::Exponential => Some(((v - a) / (b - a)).sqrt()),
        _ => None,
    }
}

/// Select a legal stepped-domain value, with exact ties resolved toward the lower value.
pub fn map_stepped_normalized(choices: &[f32], x: f32) -> Option<f32> {
    if choices.len() < 2
        || !x.is_finite()
        || !(0.0..=1.0).contains(&x)
        || choices.iter().any(|v| !v.is_finite())
        || choices.windows(2).any(|pair| pair[0] >= pair[1])
    {
        return None;
    }
    let position = x * (choices.len() - 1) as f32;
    let lower = position.floor() as usize;
    let upper = position.ceil() as usize;
    if position - lower as f32 <= upper as f32 - position {
        Some(choices[lower])
    } else {
        Some(choices[upper])
    }
}

/// Return the normalized zero-based index of an exact legal stepped-domain value.
pub fn inverse_map_stepped_normalized(choices: &[f32], value: f32) -> Option<f32> {
    if choices.len() < 2 || !value.is_finite() {
        return None;
    }
    choices
        .iter()
        .position(|choice| canonical_bits(*choice) == canonical_bits(value))
        .map(|index| index as f32 / (choices.len() - 1) as f32)
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PreparedPortsV1 {
    pub sidechain: PreparedSidechainPort,
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PreparedSidechainPort {
    None,
    Unconnected { id: PortId, required: bool },
    Connected { id: PortId, required: bool },
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InitialParameterValue {
    pub parameter_index: u32,
    pub channel: ParameterChannel,
    pub value: f32,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrepareEffectLimits {
    pub maximum_total_state_bytes: u64,
    pub maximum_scratch_bytes: u64,
    pub maximum_automation_spans_per_block: u32,
}
#[derive(Clone, Copy, Debug)]
pub struct PrepareEffectRequest<'a> {
    pub sample_rate: u32,
    pub quantum: u32,
    pub quality: EffectQuality,
    pub bypass: bool,
    pub link_mode: LinkMode,
    pub ports: PreparedPortsV1,
    pub initial_values: &'a [InitialParameterValue],
    pub limits: PrepareEffectLimits,
}
#[derive(Clone, Copy, Debug)]
pub struct PrepareEffectBankRequest<'a> {
    /// Backend this build executes on. Factories validate it against `width`.
    pub backend: Backend,
    pub width: BankWidth,
    pub requests: &'a [PrepareEffectRequest<'a>],
}

impl PrepareEffectBankRequest<'_> {
    /// Reject a bank request whose stored backend and width do not describe the same lane count.
    #[must_use]
    pub const fn has_matching_backend_width(self) -> bool {
        self.width.matches_backend(self.backend)
    }

    /// The **contract-violation** half of the `bind_homogeneous_bank` rule (issue #95).
    ///
    /// A bank request is malformed — and therefore a typed `Err`, not a fallback — when its
    /// declared backend and width disagree about the lane count, or when it does not carry
    /// exactly one member request per lane. Neither can arise from a correct planner, so a caller
    /// that sees `effect.bank.requests` has a bug to fix, not a slower path to take.
    ///
    /// Every `bind_homogeneous_bank` implementation calls this **before** it inspects a member,
    /// so that a malformed request can never be hidden behind an absent capability.
    ///
    /// # Errors
    ///
    /// `effect.bank.requests` if the request's shape is not the one its own fields declare.
    pub const fn validate_shape(self) -> Result<(), EffectPrepareError> {
        if !self.has_matching_backend_width() || self.requests.len() != self.width.lanes() as usize
        {
            return Err(EffectPrepareError {
                code: "effect.bank.requests",
            });
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectPrepareError {
    pub code: &'static str,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StatePayloadError {
    pub code: &'static str,
}
#[derive(Clone, Copy, Debug)]
pub struct PreparedEffectMetadata {
    pub descriptor: &'static EffectDescriptorV1,
    pub sample_rate: u32,
    pub quantum: u32,
    pub quality: EffectQuality,
    pub bypass: bool,
    pub link_mode: LinkMode,
    pub ports: PreparedPortsV1,
    pub latency: LatencySamples,
    pub tail: TailSamples,
    pub state_sizes: StatePayloadSizes,
    pub scratch_bytes: u64,
    pub automation_capacity: u32,
}
/// The semantic cohort identity: two prepared effects may share one bank iff these fields agree.
///
/// # `bypass` is still here, and why (issue #95 finding F4)
///
/// It should not be. `bypass` is a per-instance *configuration*, not a program: a bypassed track
/// and an identical enabled track run the same kernel with the same coefficients, and keeping the
/// flag in the key means they can never share a bank — toggling bypass on one track of an
/// eight-track cohort splits it and forces a structural rebuild. The target design, which is
/// exact and preserves every current guarantee:
///
/// * **Identity coefficients.** The wet path still runs for a bypassed lane. Its state stays
///   continuous, so un-bypassing does not click, and the cohort does not split.
/// * **A per-lane bitwise select, never an arithmetic identity.** `out = select(bypass_mask,
///   dry_delayed, wet)` with `bypass_mask = [u32::from(b).wrapping_neg(); W]` built once at bind.
///   `fma(0, wet, dry)` is **not** equivalent: `-0.0 + 0.0` is `+0.0`, which would break
///   `executed_w8_bypass_preserves_lane_local_signed_zero_at_fixed_latency`.
/// * **Latency preserved exactly.** `dry_delayed` is the lane's input delayed by exactly
///   `PreparedEffectMetadata.latency` — the same integer the enabled path reports — so a bypassed
///   lane's impulse lands on the same sample as an enabled lane's.
/// * **PDC exact by construction.** `graph-compiler` derives route timings solely from
///   `PreparedEffectMetadata.latency`, and `bypass` stays in `PrepareEffectRequest` and
///   `PreparedEffectMetadata` (it is also byte 108 of the persisted state envelope, a contract
///   fixture). Removing it from the *key* therefore changes no timing at all: the existing
///   `bypass leaves route_timings unchanged` test stays green untouched.
///
/// What blocks it is not the contract. Every effect's bank today reads one `metadata.bypass` for
/// the whole bank and builds an all-or-nothing `L::Mask` from it — `parametric-eq` does not even
/// run the wet path when bypassed — so removing the field from the key would silently apply lane
/// 0's bypass to all eight lanes. Making it per lane is a DSP change inside all nine effect
/// crates plus the rack's bank driver, which is the seam #96 owns and which this contract
/// cleanup may not touch. It is handed over with the design above rather than half-taken: a
/// key that no longer separates bypassed lanes, on kernels that cannot separate them, is a
/// correctness bug, not a cleanup.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EffectProgramKeyV1 {
    pub effect_id: EffectId,
    pub contract_major: u16,
    pub state_layout_version: u32,
    pub sample_rate: u32,
    pub quantum: u32,
    pub quality: EffectQuality,
    pub bypass: bool,
    pub link_mode: LinkMode,
    pub ports: PreparedPortsV1,
    pub latency: LatencySamples,
    pub tail: TailSamples,
    pub state_sizes: StatePayloadSizes,
    pub scratch_bytes: u64,
    pub automation_capacity: u32,
}
impl PreparedEffectMetadata {
    pub fn program_key(self) -> EffectProgramKeyV1 {
        EffectProgramKeyV1 {
            effect_id: self.descriptor.id,
            contract_major: self.descriptor.contract_major,
            state_layout_version: self.descriptor.state_layout_version,
            sample_rate: self.sample_rate,
            quantum: self.quantum,
            quality: self.quality,
            bypass: self.bypass,
            link_mode: self.link_mode,
            ports: self.ports,
            latency: self.latency,
            tail: self.tail,
            state_sizes: self.state_sizes,
            scratch_bytes: self.scratch_bytes,
            automation_capacity: self.automation_capacity,
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedBankMetadata {
    pub width: BankWidth,
    pub program_key: EffectProgramKeyV1,
}
/// What one `process` call observed. Every counter here counts **blocks**, never samples
/// (decision D7): an effect classifies no individual sample, so a per-sample count would have no
/// definition. `nonfinite_left_blocks` / `nonfinite_right_blocks` are the D7 output boundary check
/// -- one vector compare per channel per block -- and were named `recovered_*_samples` until #96
/// renamed them to what wave 2 already stores in them.
///
/// `sanitized_main_samples` and `sanitized_sidechain_samples` have no production writer: input
/// sanitisation happens once per track per block at the track input stage. They are retained for
/// the conformance reference mock, which is deliberately the permissive end of the contract.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProcessReport {
    pub sanitized_main_samples: u64,
    pub sanitized_sidechain_samples: u64,
    pub invalid_spans: u64,
    pub nonfinite_left_blocks: u64,
    pub nonfinite_right_blocks: u64,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BankProcessReport {
    pub width: BankWidth,
    pub reports: [ProcessReport; 8],
}
impl BankProcessReport {
    pub const fn empty(width: BankWidth) -> Self {
        Self {
            width,
            reports: [ProcessReport {
                sanitized_main_samples: 0,
                sanitized_sidechain_samples: 0,
                invalid_spans: 0,
                nonfinite_left_blocks: 0,
                nonfinite_right_blocks: 0,
            }; 8],
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreparedAutomationSpan {
    pub kind: AutomationSpanKind,
    pub channel: ParameterChannel,
    pub parameter_index: u32,
    pub start_sample: u64,
    pub end_sample: u64,
    pub start_value: f32,
    pub end_value: f32,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessBlockError {
    ZeroFrames,
    Shape,
    Automation,
}
pub struct EffectProcessBlock<'a> {
    pub left: &'a mut [f32],
    pub right: &'a mut [f32],
    pub sidechain: Option<(&'a [f32], &'a [f32])>,
    pub first_sample: u64,
    pub automation: &'a [PreparedAutomationSpan],
}
impl<'a> EffectProcessBlock<'a> {
    pub fn new(
        left: &'a mut [f32],
        right: &'a mut [f32],
        sidechain: Option<(&'a [f32], &'a [f32])>,
        first_sample: u64,
        automation: &'a [PreparedAutomationSpan],
        quantum: u32,
    ) -> Result<Self, ProcessBlockError> {
        if left.is_empty() {
            return Err(ProcessBlockError::ZeroFrames);
        }
        if left.len() > quantum as usize
            || left.len() != right.len()
            || sidechain.is_some_and(|(a, b)| a.len() != left.len() || b.len() != left.len())
            || first_sample.checked_add(left.len() as u64).is_none()
        {
            return Err(ProcessBlockError::Shape);
        }
        Ok(Self {
            left,
            right,
            sidechain,
            first_sample,
            automation,
        })
    }
    pub fn frames(&self) -> usize {
        self.left.len()
    }
}
pub struct EffectBankProcessBlock<'a> {
    pub left: &'a mut [f32],
    pub right: &'a mut [f32],
    pub sidechain: Option<(&'a [f32], &'a [f32])>,
    pub frames: u32,
    pub width: BankWidth,
    pub first_sample: u64,
    pub automation: &'a [PreparedAutomationSpan],
    pub automation_offsets: &'a [u32],
}
impl<'a> EffectBankProcessBlock<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        left: &'a mut [f32],
        right: &'a mut [f32],
        sidechain: Option<(&'a [f32], &'a [f32])>,
        frames: u32,
        width: BankWidth,
        first_sample: u64,
        automation: &'a [PreparedAutomationSpan],
        offsets: &'a [u32],
        quantum: u32,
    ) -> Result<Self, ProcessBlockError> {
        let size = (frames as usize)
            .checked_mul(width.lanes() as usize)
            .ok_or(ProcessBlockError::Shape)?;
        if frames == 0
            || frames > quantum
            || left.len() != size
            || right.len() != size
            || sidechain.is_some_and(|(a, b)| a.len() != size || b.len() != size)
            || offsets.len() != width.lanes() as usize + 1
            || offsets.first() != Some(&0)
            || offsets.last().copied() != Some(automation.len() as u32)
            || offsets.windows(2).any(|w| w[0] > w[1])
        {
            return Err(ProcessBlockError::Shape);
        }
        Ok(Self {
            left,
            right,
            sidechain,
            frames,
            width,
            first_sample,
            automation,
            automation_offsets: offsets,
        })
    }
}
pub fn valid_runtime_span(
    s: &PreparedAutomationSpan,
    m: PreparedEffectMetadata,
    first: u64,
    frames: u32,
) -> bool {
    let Some(p) = m.descriptor.parameters.get(s.parameter_index as usize) else {
        return false;
    };
    if !s.start_value.is_finite()
        || !s.end_value.is_finite()
        || s.start_sample < first
        || s.start_sample >= first.saturating_add(frames as u64)
        || !parameter_value_valid(p, s.start_value)
        || !parameter_value_valid(p, s.end_value)
        || (p.channel_policy == ParameterChannelPolicy::Shared
            && s.channel != ParameterChannel::Both)
        || (p.channel_policy == ParameterChannelPolicy::PerLane
            && s.channel == ParameterChannel::Both)
    {
        return false;
    }
    match s.kind {
        AutomationSpanKind::Point => {
            s.start_sample == s.end_sample
                && canonical_bits(s.start_value) == canonical_bits(s.end_value)
        }
        AutomationSpanKind::Step | AutomationSpanKind::Linear => s.end_sample > s.start_sample,
        AutomationSpanKind::Exponential => {
            s.end_sample > s.start_sample
                && s.start_value != 0.0
                && s.end_value != 0.0
                && s.start_value.is_sign_positive() == s.end_value.is_sign_positive()
        }
    }
}

/// Validate the compiler-owned canonical ordering and non-overlap rules for one delivered block.
///
/// This states the contract's whole-block law: the first invalid span rejects the entire block.
/// It is exercised by `miso-engine-conformance`'s `effect_contract` suite rather than from a
/// render path, and that is deliberate -- an effect whose own policy is to drop and count an
/// invalid span while still applying its valid siblings (as `miso-engine-compressor` does, see
/// `apply_automation` there) cannot call this without changing its rendered output. Such effects
/// carry their own guard; this function remains the reference the conformance suite holds them
/// against where the whole-block rule does apply.
pub fn validate_automation_block(
    spans: &[PreparedAutomationSpan],
    metadata: PreparedEffectMetadata,
    first_sample: u64,
    frames: u32,
) -> Result<(), ProcessBlockError> {
    if frames == 0
        || frames > metadata.quantum
        || first_sample.checked_add(frames as u64).is_none()
        || spans.len() > metadata.automation_capacity as usize
    {
        return Err(ProcessBlockError::Automation);
    }
    let mut prior_sort_key = None;
    for (span_index, span) in spans.iter().enumerate() {
        if !valid_runtime_span(span, metadata, first_sample, frames) {
            return Err(ProcessBlockError::Automation);
        }
        let sort_key = (span.start_sample, span.parameter_index, span.channel);
        if prior_sort_key.is_some_and(|prior| sort_key < prior) {
            return Err(ProcessBlockError::Automation);
        }
        if let Some(previous) = spans[..span_index].iter().rev().find(|previous| {
            previous.parameter_index == span.parameter_index && previous.channel == span.channel
        }) && (span.start_sample < previous.end_sample
            || (span.start_sample == previous.end_sample
                && canonical_bits(span.start_value) != canonical_bits(previous.end_value)))
        {
            return Err(ProcessBlockError::Automation);
        }
        let parameter = &metadata.descriptor.parameters[span.parameter_index as usize];
        if parameter.automation_rate == AutomationRate::None
            || (parameter.automation_rate == AutomationRate::Block
                && (span.kind != AutomationSpanKind::Point || span.start_sample != first_sample))
        {
            return Err(ProcessBlockError::Automation);
        }
        prior_sort_key = Some(sort_key);
    }
    Ok(())
}

/// Allocation-free control-plane parameter smoother, in the decision-D11 form.
///
/// # This is the law, not the render-path implementation
///
/// `miso_engine_effect_runtime::ramp::LinearRamp` is the **one** implementation a render path
/// uses: it is lane-generic, it drives `miso_engine_lane::kernels::ramp_block`, and every effect
/// crate ramps through it. This type stays because the contract has to state what
/// [`SmoothingRule`] *means* without depending on the render-side crate — the contract is `std`
/// and control-plane, `effect-runtime` is `no_std` and lane-generic, and neither may depend on the
/// other. The two are proven bit-for-bit identical for [`SmoothingRule::Linear`] by
/// `crates/miso-engine-effect-runtime/tests/contract_ramp_identity.rs`; if that test ever goes
/// red, this type is wrong and `LinearRamp` wins.
///
/// # D11
///
/// One division, at the moment the target changes: `step = (target - current) / N`. Then
/// `current += step` per sample, and an exact assignment of `target` on update `N`. The audited
/// form divided by `remaining` on **every** sample (issue #95 finding F2); that is deleted here.
/// [`SmoothingRule::OnePole99`] likewise precomputes `a` and `1 - a` once, at construction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParameterSmoother {
    current: f32,
    target: f32,
    step: f32,
    remaining: u32,
    total: u32,
    rule: SmoothingRule,
    one_pole_a: f32,
    one_pole_k: f32,
}
impl ParameterSmoother {
    pub fn new(initial: f32, rule: SmoothingRule, samples: u32) -> Option<Self> {
        if !initial.is_finite() || (rule == SmoothingRule::None) != (samples == 0) {
            return None;
        }
        let one_pole_a = if rule == SmoothingRule::OnePole99 {
            miso_engine_math::expf(miso_engine_math::logf(0.01) / samples as f32)
        } else {
            0.0
        };
        let initial = normalize_zero(initial);
        Some(Self {
            current: initial,
            target: initial,
            step: 0.0,
            remaining: 0,
            total: samples,
            rule,
            one_pole_a,
            one_pole_k: 1.0 - one_pole_a,
        })
    }
    /// Points the smoother at `target`. **This is the only division** (D11).
    pub fn set_target(&mut self, target: f32) -> bool {
        if !target.is_finite() {
            return false;
        }
        self.target = normalize_zero(target);
        // Issue #144 item 6, the stationary hoist. A `Linear` retarget to the value already in
        // force has `target - current == +0.0` exactly, so every update of the window adds `+0.0`
        // to a value that is already the target: `total` updates that cannot change a bit. The
        // detection is a bit compare, never a tolerance, and it is confined to `Linear` on
        // purpose -- `OnePole99` computes `a * current + k * target`, two products and two
        // roundings, which does *not* return `current` unchanged even when the two are equal, so
        // hoisting it would be a numeric change rather than a skipped no-op.
        //
        // `normalize_zero` above means `-0.0` never reaches this state, so the sign-of-zero
        // exclusion `LinearRamp::stationary_at` carries has nothing to exclude here; the
        // finiteness half is already guaranteed by the early return above.
        let stationary =
            self.rule == SmoothingRule::Linear && self.target.to_bits() == self.current.to_bits();
        if self.rule == SmoothingRule::None || stationary {
            self.current = self.target;
            self.step = 0.0;
            self.remaining = 0;
        } else {
            // Frozen operation order, identical to `LinearRamp::set_target`.
            self.step = (self.target - self.current) / self.total as f32;
            self.remaining = self.total;
        }
        true
    }
    /// Produces the next update's value and advances the state.
    ///
    /// * `remaining == 0` — at rest: returns `current` unchanged.
    /// * `remaining == 1` — the final update: assigns `target` exactly (the D11 snap).
    /// * otherwise — `current += step` for [`SmoothingRule::Linear`], one precomputed-coefficient
    ///   product for [`SmoothingRule::OnePole99`]. No division on either path.
    pub fn next_value(&mut self) -> f32 {
        match self.remaining {
            0 => self.current,
            1 => {
                self.current = self.target;
                self.step = 0.0;
                self.remaining = 0;
                self.current
            }
            _ => {
                self.current = match self.rule {
                    SmoothingRule::None => self.target,
                    SmoothingRule::Linear => self.current + self.step,
                    SmoothingRule::OnePole99 => {
                        self.one_pole_a * self.current + self.one_pole_k * self.target
                    }
                };
                self.remaining -= 1;
                self.current
            }
        }
    }
    pub fn snap(&mut self) {
        self.current = self.target;
        self.step = 0.0;
        self.remaining = 0;
    }
    pub const fn current(self) -> f32 {
        self.current
    }
    /// Per-update increment in force, or `0.0` when the smoother is at rest.
    pub const fn step(self) -> f32 {
        self.step
    }
    /// Updates still to be produced before the smoother is at its target.
    pub const fn remaining(self) -> u32 {
        self.remaining
    }
}

/// Evaluate a prepared linear or exponential segment at one absolute sample.
pub fn automation_segment_value(span: PreparedAutomationSpan, sample: u64) -> Option<f32> {
    if sample < span.start_sample
        || sample > span.end_sample
        || span.end_sample <= span.start_sample
    {
        return None;
    }
    if sample == span.end_sample {
        return Some(span.end_value);
    }
    let x = (sample - span.start_sample) as f32 / (span.end_sample - span.start_sample) as f32;
    match span.kind {
        AutomationSpanKind::Linear => {
            Some(span.start_value + x * (span.end_value - span.start_value))
        }
        AutomationSpanKind::Exponential
            if span.start_value != 0.0
                && span.end_value != 0.0
                && span.start_value.is_sign_positive() == span.end_value.is_sign_positive() =>
        {
            Some(span.start_value * miso_engine_math::powf(span.end_value / span.start_value, x))
        }
        _ => None,
    }
}
pub struct StatePayloadOutput<'a> {
    pub common: &'a mut [u8],
    pub left: &'a mut [u8],
    pub right: &'a mut [u8],
}
pub struct StatePayloadInput<'a> {
    pub common: &'a [u8],
    pub left: &'a [u8],
    pub right: &'a [u8],
}
impl<'a> StatePayloadOutput<'a> {
    pub fn new(
        common: &'a mut [u8],
        left: &'a mut [u8],
        right: &'a mut [u8],
        s: StatePayloadSizes,
    ) -> Result<Self, StatePayloadError> {
        s.check(common.len(), left.len(), right.len())?;
        Ok(Self {
            common,
            left,
            right,
        })
    }
}
impl<'a> StatePayloadInput<'a> {
    pub fn new(
        common: &'a [u8],
        left: &'a [u8],
        right: &'a [u8],
        s: StatePayloadSizes,
    ) -> Result<Self, StatePayloadError> {
        s.check(common.len(), left.len(), right.len())?;
        Ok(Self {
            common,
            left,
            right,
        })
    }
}
pub trait NativeEffectFactory: Send + Sync {
    fn descriptor(&self) -> &'static EffectDescriptorV1;
    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError>;

    /// Binds `width` tracks into one homogeneous bank, or declines.
    ///
    /// # The three-outcome rule (issue #95; frozen for every implementation)
    ///
    /// Wave 2 left this method meaning two different things: eight crates declined a cohort whose
    /// members did not share a program key with `Ok(None)`, one rejected it with
    /// `Err("effect.bank.program")`, and one answered a malformed *shape* with `Ok(None)` where
    /// the others answered `Err("effect.bank.requests")`. One semantic now applies everywhere,
    /// decided from what the consumer actually does with each answer:
    ///
    /// | outcome | meaning | `graph-compiler` | `effect-compiler` restore |
    /// |---|---|---|---|
    /// | `Err(code)` | **the request violates this contract** | fails the whole graph compile with `code` | `effect.state.unavailable` |
    /// | `Ok(None)` | the request is well formed, but this artifact cannot bank it | skips the cohort; the tracks run as scalar instances | `effect.state.unavailable` |
    /// | `Ok(Some(bank))` | bound | uses the bank | uses the bank |
    ///
    /// `Err` is therefore reserved for what a correct planner cannot produce:
    ///
    /// * a shape that contradicts itself — [`PrepareEffectBankRequest::validate_shape`];
    /// * a **member** request that would fail [`NativeEffectFactory::prepare`] on its own, which
    ///   is the same diagnostic `prepare` would have returned (`effect.quality.unsupported`,
    ///   `effect.parameter.initial`, …).
    ///
    /// `Ok(None)` covers every remaining "no bank here", because each one is a legal session that
    /// must still render:
    ///
    /// * a width this build does not execute (decision D4 makes that a compile-time constant, so
    ///   it is a property of the artifact and not of the request);
    /// * a **heterogeneous cohort** — members that do not all share one `EffectProgramKeyV1`;
    /// * a port or link configuration this effect has no bank kernel for.
    ///
    /// The heterogeneous case is the one that moved. `graph-compiler` groups candidates by
    /// `metadata.program_key()` before it ever calls this method, so a mixed cohort is
    /// unreachable from the production planner; the choice is about what happens if a *future*
    /// planner produces one. `Err` would turn that into "the session does not compile at all",
    /// `Ok(None)` into "the session renders, one bank slower". A planner bug must not cost the
    /// user their session, so `Ok(None)` wins, and the cohort-grouping invariant is gated where
    /// it is decided — in the planner — rather than by making every effect crate a second,
    /// fatal check of it.
    ///
    /// # Errors
    ///
    /// See the table above. Every implementation validates **every** member before it decides the
    /// shape, so an absent capability can never hide a malformed member.
    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError>;
}
/// One observation reading, in the tap's declared [`unit`](ObservationDescriptorV1::unit).
///
/// Two lanes always, because a dual-mono effect has two of everything. A tap that declares
/// [`ObservationChannelsV1::Shared`] writes the same value into both, so a consumer never has to
/// ask which field is meaningful.
///
/// The **sign and unit are the effect's own**, not the consumer's: a compressor writes the
/// negative decibels its smoother holds, a true-peak limiter writes the linear reduction word its
/// kernel recurses on. Turning that into the non-negative magnitude a meter shows is the declared
/// [`fold`](ObservationDescriptorV1::fold) plus one control-plane unit conversion -- neither of
/// which happens inside an effect, and neither of which puts a `log` on a render thread.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ObservationSampleV1 {
    /// Left lane.
    pub left: f32,
    /// Right lane.
    pub right: f32,
}

pub trait PreparedNativeEffect: Send {
    fn metadata(&self) -> PreparedEffectMetadata;
    fn reset(&mut self, kind: ResetKind);
    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport;

    /// Read one declared [`ObservationCostV1::Resident`] tap into `out` (issue #143 D2).
    ///
    /// `tap_index` is the index into
    /// [`EffectDescriptorV1::observations`](EffectDescriptorV1::observations), never the wire
    /// `tap_id`: the translation is an admission-time lookup, off the render thread, exactly as
    /// `parameter_index` is for a live parameter.
    ///
    /// Returns `false` -- and leaves `out` untouched -- for a tap this effect does not implement.
    /// The default implementation returns `false` for every index, so adding the method breaks no
    /// implementation and changes no prepared byte: an effect that declares no tap costs one vtable
    /// slot.
    ///
    /// # `&self`, and why the type says so
    ///
    /// Resident means *the value already exists*. The method takes `&self` so an implementation
    /// physically cannot advance a smoother, run a release step, or otherwise make the reading a
    /// second opinion about what the block did; calling it twice returns identical bits because
    /// there is nothing it could have changed. That is the whole content of "resident" and it is
    /// enforced by the signature rather than asserted in prose (#143 E6).
    fn observe_resident(&self, tap_index: u32, out: &mut ObservationSampleV1) -> bool {
        let _ = (tap_index, out);
        false
    }
    fn snapshot_state_payload(
        &self,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError>;

    /// Restores a snapshot into this prepared instance.
    ///
    /// # The payload-header rule (issue #95; the uniform rule for every effect)
    ///
    /// `state_layout_version` is the caller's **claim** about the bytes it is handing over. It
    /// arrives out of band — from the descriptor the caller believes wrote them — and it is only
    /// trustworthy while the caller is the same build as the writer, which is exactly the
    /// situation a persisted session is not in.
    ///
    /// The frozen rule, adopted from the design #87 landed in `parametric-eq`:
    ///
    /// > **A version or length word inside the payload outranks the caller's claim.** Where a
    /// > payload carries a header, the restore compares the two and rejects on the payload's own
    /// > evidence; it never lets the argument override the bytes. Where a payload carries no
    /// > header, the argument is all there is, and the restore checks it against the descriptor's
    /// > `state_layout_version` and the prepared `StatePayloadSizes` — the caller may not name a
    /// > version this instance was not prepared for.
    ///
    /// The header itself is two little-endian words at the front of the common section — the
    /// layout version and the effect's data word count — implemented once in
    /// `miso_engine_effect_runtime::state_payload` (`HEADER_WORDS`, `read_header`, `snapshot`,
    /// `restore`). It is what makes a payload self-describing, so a stale or truncated restore is
    /// rejected by evidence rather than by trust.
    ///
    /// **Uniform adoption is not this issue's to take.** A header moves `maximum_state.common_bytes`
    /// from 0 to 8, which is a canonical descriptor byte, an effect CID and a
    /// `state_layout_version` bump (decision W2-D2). Crates that had to bump anyway adopted it
    /// inside that bump — `parametric-eq`, `gate-expander`, `soft-clip`, `true-peak-limiter`; the
    /// rest keep their current layout until a coordinated change carries the identity bump with
    /// #97. What is frozen **now** is the rule above, so that no effect adopts a header and then
    /// still lets the argument win, and no new effect ships without one.
    ///
    /// # Errors
    ///
    /// `effect.state.version` if the payload is not the layout this instance was prepared for,
    /// `effect.state.length` if a section is not its prepared size, and the effect's own code if
    /// a decoded value is outside its declared domain.
    fn restore_state_payload(
        &mut self,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError>;
}
pub trait PreparedNativeEffectBank: Send {
    fn metadata(&self) -> PreparedBankMetadata;
    fn reset(&mut self, kind: ResetKind);
    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport;

    /// Read one declared [`ObservationCostV1::Resident`] tap for **every lane at once**.
    ///
    /// One call per tap per block, not one per lane: a bank's state is vector state, so extracting
    /// it once and scattering into `out` is a single `store`, while a per-lane accessor would be
    /// `width` of them. `out` is exactly `PreparedBankMetadata::width.lanes()` long; an
    /// implementation writes every entry or returns `false` and writes none.
    ///
    /// Same `&self` rule, for the same reason, as
    /// [`PreparedNativeEffect::observe_resident`].
    fn observe_resident_bank(&self, tap_index: u32, out: &mut [ObservationSampleV1]) -> bool {
        let _ = (tap_index, out);
        false
    }
    fn snapshot_track_state_payload(
        &self,
        track_index: u32,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError>;
    fn restore_track_state_payload(
        &mut self,
        track_index: u32,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError>;
}
#[derive(Default)]
pub struct NativeEffectRegistry {
    factories: BTreeMap<EffectId, Arc<dyn NativeEffectFactory>>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistryError {
    pub code: &'static str,
    pub id: Option<EffectId>,
}
impl NativeEffectRegistry {
    pub fn new(
        f: impl IntoIterator<Item = Box<dyn NativeEffectFactory>>,
    ) -> Result<Self, RegistryError> {
        let mut m = BTreeMap::new();
        for x in f {
            let d = x.descriptor();
            if validate_descriptor_v1(d).is_err() {
                return Err(RegistryError {
                    code: "effect.descriptor.invalid",
                    id: Some(d.id),
                });
            }
            if m.insert(d.id, Arc::from(x)).is_some() {
                return Err(RegistryError {
                    code: "effect.registry.duplicate",
                    id: Some(d.id),
                });
            }
        }
        Ok(Self { factories: m })
    }
    pub fn get(&self, id: EffectId) -> Option<&dyn NativeEffectFactory> {
        self.factories.get(&id).map(Arc::as_ref)
    }
    pub fn get_ascii(&self, id: &str) -> Option<&dyn NativeEffectFactory> {
        self.factories
            .iter()
            .find_map(|(key, value)| (key.as_str() == id).then_some(value.as_ref()))
    }
    /// Clone the immutable factory handle for an off-render prepared plan.
    pub fn get_shared_ascii(&self, id: &str) -> Option<Arc<dyn NativeEffectFactory>> {
        self.factories
            .iter()
            .find_map(|(key, value)| (key.as_str() == id).then_some(Arc::clone(value)))
    }
    pub fn len(&self) -> usize {
        self.factories.len()
    }
    pub fn is_empty(&self) -> bool {
        self.factories.is_empty()
    }
    /// Every registered descriptor, in stable [`EffectId`] order (issue #137 D4).
    ///
    /// The parameter-metadata codegen reads the registry through this, so "an effect in the
    /// registry is missing from the emitted metadata" is not a rule anyone has to remember to
    /// check: there is no other list to fall out of step with.
    pub fn descriptors(&self) -> impl Iterator<Item = &'static EffectDescriptorV1> + '_ {
        self.factories.values().map(|factory| factory.descriptor())
    }
}
/// The exact `(parameter_index, channel)` sequence a prepare request must carry, in order.
///
/// `Shared` contributes one `Both` entry, `PerLane` contributes `Left` then `Right`. This is the
/// only statement of that order: [`validate_initial_values`] checks against it and
/// [`default_initial_values`] fills it with the descriptor's declared defaults, so a caller can
/// build a conforming request from the descriptor alone. Before issue #95 the order existed only
/// inside the validator as a freshly allocated `Vec`, which is why the conformance harness had to
/// hard-code one parameter's L/R pair and could therefore only ever run against its own mock.
pub fn initial_value_slots(
    d: &'static EffectDescriptorV1,
) -> impl Iterator<Item = (u32, ParameterChannel)> + 'static {
    d.parameters
        .iter()
        .enumerate()
        .flat_map(|(index, parameter)| {
            let index = index as u32;
            match parameter.channel_policy {
                ParameterChannelPolicy::Shared => [Some((index, ParameterChannel::Both)), None],
                ParameterChannelPolicy::PerLane => [
                    Some((index, ParameterChannel::Left)),
                    Some((index, ParameterChannel::Right)),
                ],
            }
        })
        .flatten()
}

/// A conforming `initial_values` slice built from the descriptor's declared defaults.
///
/// Every entry is `parameter_valid`-checked at descriptor validation, so the result always passes
/// [`validate_initial_values`]. `-0.0` is normalised: the validator rejects it, and a descriptor
/// that declares it is already invalid.
pub fn default_initial_values(
    d: &'static EffectDescriptorV1,
) -> impl Iterator<Item = InitialParameterValue> + 'static {
    initial_value_slots(d).map(|(parameter_index, channel)| InitialParameterValue {
        parameter_index,
        channel,
        value: normalize_zero(d.parameters[parameter_index as usize].default_value),
    })
}

pub fn validate_initial_values(
    d: &'static EffectDescriptorV1,
    v: &[InitialParameterValue],
) -> Result<(), EffectPrepareError> {
    if initial_value_slots(d).count() != v.len() {
        return Err(EffectPrepareError {
            code: "effect.parameter.initial",
        });
    }
    for (x, e) in v.iter().zip(initial_value_slots(d)) {
        let p = d
            .parameters
            .get(x.parameter_index as usize)
            .ok_or(EffectPrepareError {
                code: "effect.parameter.unknown",
            })?;
        if (x.parameter_index, x.channel) != e
            || !parameter_value_valid(p, x.value)
            || is_negative_zero(x.value)
        {
            return Err(EffectPrepareError {
                code: "effect.parameter.initial",
            });
        }
    }
    Ok(())
}
/// What a validated prepare request resolved to.
///
/// `scratch_bytes` is computed **once**, here. The audit found the same
/// `scratch_fixed_bytes + scratch_bytes_per_frame * quantum` accounting written out twice in this
/// file — in `validate_prepare_request` to compare against the caller's limit, and again in
/// `expected_prepared_metadata` to fill the metadata — so a change to one silently disagreed with
/// the other. One value, computed once, used for both.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidatedPrepare {
    /// The quality row the request resolved to.
    pub quality: QualityDescriptorV1,
    /// Scratch this preparation is entitled to: `scratch_fixed_bytes + per_frame * quantum`.
    pub scratch_bytes: u64,
}

/// `scratch_fixed_bytes + scratch_bytes_per_frame * quantum`, checked.
///
/// # What `scratch_fixed_bytes` means (issue #95, closing #88 F10 and #92 F9)
///
/// Wave 2 reported the field as "declared and unused": the compressor declares 64 and the
/// transient shaper 24, and neither touches a scratch byte. Both deferred a re-accounting here.
/// The re-accounting is a **definition**, not a number change, because the number is part of the
/// canonical descriptor bytes and therefore of the effect's CID (issue 082) — moving it would
/// break contract fixtures that master plan §8 rule 2 forbids this job to touch.
///
/// The definition: `scratch_fixed_bytes` is the **admission ceiling** an effect reserves, not a
/// measurement of what it uses. A host admits a preparation by proving it can supply this much
/// scratch; an effect that uses less is conforming, an effect that uses more is not. Under that
/// definition a declared 64 with an actual 0 is correct — conservative, not wrong — and the two
/// deferred items close with no descriptor byte moved and no CID re-pinned.
///
/// Tightening a declared ceiling toward its measured use remains legal, but it is a descriptor
/// identity change: it belongs to the owning effect's issue, coordinated with #97 (package/CID),
/// never to a contract cleanup.
///
/// # Errors
///
/// `effect.resource.limit` on overflow.
const fn scratch_for(
    quality: QualityDescriptorV1,
    quantum: u32,
) -> Result<u64, EffectPrepareError> {
    let limit = EffectPrepareError {
        code: "effect.resource.limit",
    };
    let Some(per_block) = quality.scratch_bytes_per_frame.checked_mul(quantum as u64) else {
        return Err(limit);
    };
    match quality.scratch_fixed_bytes.checked_add(per_block) {
        Some(total) => Ok(total),
        None => Err(limit),
    }
}

pub fn validate_prepare_request(
    d: &'static EffectDescriptorV1,
    r: PrepareEffectRequest<'_>,
) -> Result<ValidatedPrepare, EffectPrepareError> {
    validate_descriptor_v1(d).map_err(|_| EffectPrepareError {
        code: "effect.descriptor.invalid",
    })?;
    if r.sample_rate == 0
        || r.quantum == 0
        || r.limits.maximum_total_state_bytes == 0
        || r.limits.maximum_scratch_bytes == 0
        || r.limits.maximum_automation_spans_per_block == 0
    {
        return Err(EffectPrepareError {
            code: "effect.prepare.capacity",
        });
    }
    if !d.supported_link_modes.contains(r.link_mode) {
        return Err(EffectPrepareError {
            code: "effect.link_mode.unsupported",
        });
    }
    let q = d
        .qualities
        .iter()
        .find(|q| q.quality == r.quality && q.sample_rate == r.sample_rate)
        .copied()
        .ok_or(EffectPrepareError {
            code: "effect.quality.unsupported",
        })?;
    let state = q.maximum_state.total().ok_or(EffectPrepareError {
        code: "effect.resource.limit",
    })?;
    let scratch = scratch_for(q, r.quantum)?;
    if state > r.limits.maximum_total_state_bytes
        || scratch > r.limits.maximum_scratch_bytes
        || usize::try_from(state).is_err()
        || isize::try_from(state).is_err()
        || usize::try_from(scratch).is_err()
        || isize::try_from(scratch).is_err()
    {
        return Err(EffectPrepareError {
            code: "effect.resource.limit",
        });
    }
    validate_initial_values(d, r.initial_values)?;
    match r.ports.sidechain {
        PreparedSidechainPort::None => {
            if d.ports.iter().any(|p| p.role == PortRole::SidechainInput) {
                return Err(EffectPrepareError {
                    code: "effect.sidechain.missing",
                });
            }
        }
        PreparedSidechainPort::Unconnected { id, required } => {
            if required
                || !d.ports.iter().any(|p| {
                    p.role == PortRole::SidechainInput && p.id == id && p.required == required
                })
            {
                return Err(EffectPrepareError {
                    code: "effect.sidechain.missing",
                });
            }
        }
        PreparedSidechainPort::Connected { id, required } => {
            if !d
                .ports
                .iter()
                .any(|p| p.role == PortRole::SidechainInput && p.id == id && p.required == required)
            {
                return Err(EffectPrepareError {
                    code: "effect.sidechain.unknown_port",
                });
            }
        }
    }
    Ok(ValidatedPrepare {
        quality: q,
        scratch_bytes: scratch,
    })
}

/// Derive the sole conforming immutable metadata value for a validated prepare request.
pub fn expected_prepared_metadata(
    descriptor: &'static EffectDescriptorV1,
    request: PrepareEffectRequest<'_>,
) -> Result<PreparedEffectMetadata, EffectPrepareError> {
    let ValidatedPrepare {
        quality,
        scratch_bytes,
    } = validate_prepare_request(descriptor, request)?;
    Ok(PreparedEffectMetadata {
        descriptor,
        sample_rate: request.sample_rate,
        quantum: request.quantum,
        quality: request.quality,
        bypass: request.bypass,
        link_mode: request.link_mode,
        ports: request.ports,
        latency: quality.latency,
        tail: quality.tail,
        state_sizes: quality.maximum_state,
        scratch_bytes,
        automation_capacity: request.limits.maximum_automation_spans_per_block,
    })
}

#[cfg(test)]
mod bank_width_tests {
    use super::{Backend, BankWidth};

    /// #84 phase A, eval A-2: `for_backend` is the workspace's one backend-to-width law, it is
    /// total over `Backend`, and `matches_backend` agrees with it on every `(width, backend)` pair.
    #[test]
    fn bank_width_for_backend_is_total() {
        assert_eq!(BankWidth::for_backend(Backend::Scalar), None);
        assert_eq!(
            BankWidth::for_backend(Backend::Simd4),
            Some(BankWidth::Four)
        );
        assert_eq!(
            BankWidth::for_backend(Backend::Simd8),
            Some(BankWidth::Eight)
        );

        for width in [BankWidth::Four, BankWidth::Eight] {
            for backend in [Backend::Scalar, Backend::Simd4, Backend::Simd8] {
                assert_eq!(
                    width.matches_backend(backend),
                    BankWidth::for_backend(backend) == Some(width),
                    "{width:?} vs {backend:?}"
                );
                if width.matches_backend(backend) {
                    assert_eq!(
                        u32::try_from(backend.width()).expect("width"),
                        width.lanes()
                    );
                }
            }
        }
    }
}
