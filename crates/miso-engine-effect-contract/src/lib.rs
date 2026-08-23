//! Semantic, render-safe native effect runtime contract V1.
//!
//! This crate deliberately has no descriptor wire format, digest, package, CID, parser, or
//! persistence envelope. Those are interchange concerns owned by issue 029.
#![allow(missing_docs)]

use core::{fmt, hash::Hash};
use miso_engine_core::{
    KernelBackendV1, LAUNCH_SAMPLE_RATES, SampleRateHz, is_extended_compatibility_sample_rate,
    is_launch_sample_rate,
};
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
    /// Whether this non-scalar bank width is legal for a selected backend.
    #[must_use]
    pub const fn matches_backend(self, backend: KernelBackendV1) -> bool {
        matches!(
            (self, backend),
            (
                Self::Four,
                KernelBackendV1::WasmSimd128 | KernelBackendV1::Aarch64Neon
            ) | (
                Self::Eight,
                KernelBackendV1::X86Avx2 | KernelBackendV1::X86Avx2Fma
            )
        )
    }
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
    /// Backend selected before plan preparation. Factories validate it against `width`.
    pub backend: KernelBackendV1,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProcessReport {
    pub sanitized_main_samples: u64,
    pub sanitized_sidechain_samples: u64,
    pub invalid_spans: u64,
    pub recovered_left_samples: u64,
    pub recovered_right_samples: u64,
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
                recovered_left_samples: 0,
                recovered_right_samples: 0,
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
        if self.rule == SmoothingRule::None {
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
        if common.len() != s.common_bytes as usize
            || left.len() != s.left_bytes as usize
            || right.len() != s.right_bytes as usize
        {
            Err(StatePayloadError {
                code: "effect.state.length",
            })
        } else {
            Ok(Self {
                common,
                left,
                right,
            })
        }
    }
}
impl<'a> StatePayloadInput<'a> {
    pub fn new(
        common: &'a [u8],
        left: &'a [u8],
        right: &'a [u8],
        s: StatePayloadSizes,
    ) -> Result<Self, StatePayloadError> {
        if common.len() != s.common_bytes as usize
            || left.len() != s.left_bytes as usize
            || right.len() != s.right_bytes as usize
        {
            Err(StatePayloadError {
                code: "effect.state.length",
            })
        } else {
            Ok(Self {
                common,
                left,
                right,
            })
        }
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
pub trait PreparedNativeEffect: Send {
    fn metadata(&self) -> PreparedEffectMetadata;
    fn reset(&mut self, kind: ResetKind);
    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport;
    fn snapshot_state_payload(
        &self,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError>;
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
}
pub fn validate_initial_values(
    d: &'static EffectDescriptorV1,
    v: &[InitialParameterValue],
) -> Result<(), EffectPrepareError> {
    let mut expected = Vec::new();
    for (i, p) in d.parameters.iter().enumerate() {
        match p.channel_policy {
            ParameterChannelPolicy::Shared => expected.push((i as u32, ParameterChannel::Both)),
            ParameterChannelPolicy::PerLane => {
                expected.push((i as u32, ParameterChannel::Left));
                expected.push((i as u32, ParameterChannel::Right));
            }
        }
    }
    if expected.len() != v.len() {
        return Err(EffectPrepareError {
            code: "effect.parameter.initial",
        });
    }
    for (x, e) in v.iter().zip(expected) {
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
pub fn validate_prepare_request(
    d: &'static EffectDescriptorV1,
    r: PrepareEffectRequest<'_>,
) -> Result<QualityDescriptorV1, EffectPrepareError> {
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
    let scratch = q
        .scratch_fixed_bytes
        .checked_add(
            q.scratch_bytes_per_frame
                .checked_mul(r.quantum as u64)
                .ok_or(EffectPrepareError {
                    code: "effect.resource.limit",
                })?,
        )
        .ok_or(EffectPrepareError {
            code: "effect.resource.limit",
        })?;
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
    Ok(q)
}

/// Derive the sole conforming immutable metadata value for a validated prepare request.
pub fn expected_prepared_metadata(
    descriptor: &'static EffectDescriptorV1,
    request: PrepareEffectRequest<'_>,
) -> Result<PreparedEffectMetadata, EffectPrepareError> {
    let quality = validate_prepare_request(descriptor, request)?;
    let scratch_bytes = quality
        .scratch_fixed_bytes
        .checked_add(
            quality
                .scratch_bytes_per_frame
                .checked_mul(request.quantum as u64)
                .ok_or(EffectPrepareError {
                    code: "effect.resource.limit",
                })?,
        )
        .ok_or(EffectPrepareError {
            code: "effect.resource.limit",
        })?;
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
