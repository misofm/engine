//! Native, owner-provided response analysis contracts.
//!
//! This module is deliberately separate from [`EffectDescriptor`].  Response analyses are a
//! native control-plane capability and do not add bytes to the effect descriptor, its digest, or
//! any wire representation.  Providers own their immutable prepared configuration; callers own
//! the frequency and output buffers passed to a query.

use super::{EffectPrepareError, ObservationCost, ParameterUnit, PrepareEffectRequest};

/// The owner kinds that can currently provide a live target snapshot.
///
/// This is an internal transfer identity rather than a public response mode. The response
/// evaluator still decides how to interpret the copied words after the exclusive owner releases
/// them.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseSnapshotKind {
    /// A native parametric EQ cascade.
    ParametricEq,
    /// The prepared input HPF/LPF pair.
    BuiltinInputFilters,
}

/// Maximum number of words carried by one response section snapshot.
pub const RESPONSE_SNAPSHOT_WORDS: usize = 7;

/// One immutable owner section copied into caller-provided snapshot storage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResponseSnapshotSection {
    /// Stable section identity in the owner's descriptor order.
    pub id: u32,
    /// Owner-specific section family, such as an EQ band kind.
    pub kind: u32,
    /// Whether this section contributes to its owner's subtotal.
    pub enabled: bool,
    /// Number of meaningful words in [`Self::words`].
    pub word_count: u8,
    /// Rounded owner words in a fixed bounded transfer record.
    pub words: [u32; RESPONSE_SNAPSHOT_WORDS],
}

/// Caller-owned storage and boundary metadata for one owner snapshot copy.
#[derive(Debug)]
pub struct ResponseSnapshotRequest<'a> {
    /// The live effect bypass state at the same ownership boundary as the copied sections.
    pub bypassed: bool,
    /// Left-channel section storage.
    pub left: &'a mut [ResponseSnapshotSection],
    /// Right-channel section storage.
    pub right: &'a mut [ResponseSnapshotSection],
}

/// Metadata returned after a successful owner snapshot copy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResponseSnapshotSummary {
    /// The owner-specific snapshot kind.
    pub kind: ResponseSnapshotKind,
    /// The prepared owner's validated sample rate.
    pub sample_rate_hz: u32,
    /// The bypass state supplied for this exact boundary.
    pub bypassed: bool,
    /// Number of sections written in each channel.
    pub sections: u32,
}

/// The only response mode shipped by the native response providers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseAnalysisMode {
    /// Evaluate the configuration supplied to the provider's preparation call.
    RequestedConfiguration,
}

/// The query cadence declared by a response provider.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseQueryCadence {
    /// A value is produced only when a caller explicitly asks for one.
    ExplicitQuery,
}

/// Channel semantics for a response provider.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseChannelLayout {
    /// Left and right are evaluated independently.
    Independent,
}

/// Reference used by a response provider's dB output.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseAmplitudeReference {
    /// Zero dB denotes unit amplitude.
    Unity,
}

/// Shape of the output made available by a provider.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseSectionOutput {
    /// Total left/right curves are required and section curves are optional.
    TotalAndOptionalSections,
}

/// Meaning of the total curve.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseTotalScope {
    /// The total is the complete parametric EQ cascade.
    ParametricEqCascade,
    /// The total is the builtin input HPF/LPF subtotal.
    BuiltinInputFilterSubtotal,
}

/// Bypass behavior declared by a provider.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseBypassSemantics {
    /// EQ total output is identity while its configured section curves remain available.
    EffectWideIdentityWithSections,
    /// The provider has no effect-wide bypass field.
    NoEffectBypass,
}

/// Metadata owned by one native response provider.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResponseAnalysisDescriptor {
    /// Owner-local nonzero response capability identity.
    pub id: u32,
    /// Bounded static display name.
    pub name: &'static str,
    /// Sections in the exact order used by section-major output buffers.
    pub sections: &'static [ResponseSectionDescriptor],
    /// Frequency-axis unit. Providers currently declare [`ParameterUnit::Hz`].
    pub axis_unit: ParameterUnit,
    /// Output unit. Providers currently declare [`ParameterUnit::Db`].
    pub unit: ParameterUnit,
    /// Amplitude reference for the output unit.
    pub amplitude_reference: ResponseAmplitudeReference,
    /// Channel arrangement of the output.
    pub channels: ResponseChannelLayout,
    /// Supported query mode.
    pub mode: ResponseAnalysisMode,
    /// Query cadence.
    pub cadence: ResponseQueryCadence,
    /// Output shape declaration.
    pub section_output: ResponseSectionOutput,
    /// Declared cost class.
    pub cost: ObservationCost,
    /// Public dB floor. It is finite and strictly negative.
    pub floor_db: f32,
    /// Meaning of the total curve.
    pub total_scope: ResponseTotalScope,
    /// Bypass behavior.
    pub bypass: ResponseBypassSemantics,
}

/// One owner-local section in a response descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResponseSectionDescriptor {
    /// Nonzero owner-local stable section identity.
    pub id: u32,
    /// Bounded static section name.
    pub name: &'static str,
}

/// Resource admission bound for one prepared response provider.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResponsePrepareLimits {
    /// Maximum bytes of the provider's concrete retained payload.
    pub maximum_prepared_bytes: usize,
}

/// A caller-owned response query. The frequency slice is immutable for the duration of the call.
#[derive(Clone, Copy, Debug)]
pub struct ResponseQuery<'a> {
    /// Opaque correlation identity, echoed without narrowing.
    pub configuration_id: u64,
    /// Strictly increasing frequencies in Hz.
    pub frequencies_hz: &'a [f32],
    /// Maximum number of points admitted by the caller.
    pub maximum_points: usize,
}

/// Caller-owned output buffers for one response query.
#[derive(Debug)]
pub struct ResponseOutput<'a> {
    /// Required left-channel total curve.
    pub total_left_db: &'a mut [f32],
    /// Required right-channel total curve.
    pub total_right_db: &'a mut [f32],
    /// Optional section-major left curves.
    pub sections_left_db: Option<&'a mut [f32]>,
    /// Optional section-major right curves.
    pub sections_right_db: Option<&'a mut [f32]>,
}

/// Metadata returned after a successful query.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResponseSummary {
    /// The query's opaque correlation identity.
    pub configuration_id: u64,
    /// Mode used by the query.
    pub mode: ResponseAnalysisMode,
    /// Validated preparation sample rate.
    pub sample_rate_hz: u32,
    /// Number of points written to each total output.
    pub points: usize,
    /// Public magnitude floor in dB.
    pub floor_db: f32,
}

/// Borrowed immutable configuration metadata exposed by a prepared provider.
///
/// The slices are borrowed from the provider and remain valid only while that immutable provider
/// borrow is alive. They are not caller-owned preparation storage and cannot be mutated through
/// this view.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResponseConfigurationView<'a> {
    /// Validated preparation sample rate.
    pub sample_rate_hz: u32,
    /// Per-section left enable state in descriptor order.
    pub enabled_left: &'a [bool],
    /// Per-section right enable state in descriptor order.
    pub enabled_right: &'a [bool],
    /// EQ effect-wide bypass; `None` for builtin filters.
    pub bypass: Option<bool>,
}

/// Why a native response preparation or query was refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseAnalysisError {
    /// The requested response mode is not implemented by the provider.
    UnsupportedMode,
    /// The provider does not implement the requested capability.
    UnsupportedCapability,
    /// The immutable preparation configuration was refused by the owning effect.
    Configuration(EffectPrepareError),
    /// The frequency grid is empty, non-finite, out of range, duplicated, or descending.
    InvalidFrequencyGrid,
    /// The point budget or fixed section shape cannot be admitted.
    Capacity,
    /// A caller output buffer has the wrong exact shape.
    OutputShape,
    /// The owner evaluator encountered a non-finite or singular value.
    Numerical,
    /// The concrete provider payload exceeds the caller's admission bound.
    ResourceLimit,
}

/// Why a companion response declaration is not admissible in a native registry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseDescriptorError {
    /// The capability or section identity is zero.
    ZeroId,
    /// A static name is empty, too long, or contains a control character.
    InvalidName,
    /// There are no declared sections or the declaration is too large.
    SectionCount,
    /// Section IDs are not strictly increasing and unique.
    SectionOrder,
    /// The declared floor is not finite and negative.
    Floor,
    /// A typed semantic field does not describe the supported native contract.
    Semantics,
}

/// Validate a companion native response declaration without allocating.
pub fn validate_response_analysis_descriptor(
    descriptor: &ResponseAnalysisDescriptor,
) -> Result<(), ResponseDescriptorError> {
    if descriptor.id == 0 {
        return Err(ResponseDescriptorError::ZeroId);
    }
    if descriptor.name.is_empty()
        || descriptor.name.len() > 127
        || !super::valid_text(descriptor.name)
    {
        return Err(ResponseDescriptorError::InvalidName);
    }
    if descriptor.sections.is_empty() || descriptor.sections.len() > 64 {
        return Err(ResponseDescriptorError::SectionCount);
    }
    let mut previous = 0;
    for section in descriptor.sections {
        if section.id == 0 || section.id <= previous {
            return Err(ResponseDescriptorError::SectionOrder);
        }
        if section.name.is_empty() || section.name.len() > 127 || !super::valid_text(section.name) {
            return Err(ResponseDescriptorError::InvalidName);
        }
        previous = section.id;
    }
    if !descriptor.floor_db.is_finite() || descriptor.floor_db >= 0.0 {
        return Err(ResponseDescriptorError::Floor);
    }
    if descriptor.axis_unit != ParameterUnit::Hz
        || descriptor.unit != ParameterUnit::Db
        || descriptor.amplitude_reference != ResponseAmplitudeReference::Unity
        || descriptor.channels != ResponseChannelLayout::Independent
        || descriptor.mode != ResponseAnalysisMode::RequestedConfiguration
        || descriptor.cadence != ResponseQueryCadence::ExplicitQuery
        || descriptor.section_output != ResponseSectionOutput::TotalAndOptionalSections
        || descriptor.cost != ObservationCost::Computed
    {
        return Err(ResponseDescriptorError::Semantics);
    }
    let supported_scope = matches!(
        (descriptor.total_scope, descriptor.bypass),
        (
            ResponseTotalScope::ParametricEqCascade,
            ResponseBypassSemantics::EffectWideIdentityWithSections
        ) | (
            ResponseTotalScope::BuiltinInputFilterSubtotal,
            ResponseBypassSemantics::NoEffectBypass
        )
    );
    if !supported_scope {
        return Err(ResponseDescriptorError::Semantics);
    }
    Ok(())
}

/// Native response capability supplied by an effect owner.
pub trait NativeEffectResponseFactory: Send + Sync {
    /// Authoritative descriptor for this capability.
    fn analysis_descriptor(&self) -> &'static ResponseAnalysisDescriptor;
    /// Validate and retain a fixed response configuration off the render thread.
    fn prepare_response(
        &self,
        request: PrepareEffectRequest<'_>,
        limits: ResponsePrepareLimits,
    ) -> Result<Box<dyn PreparedResponseAnalysis>, ResponseAnalysisError>;
}

/// Prepared immutable response capability.
pub trait PreparedResponseAnalysis: Send + Sync {
    /// Authoritative descriptor for this prepared capability.
    fn analysis_descriptor(&self) -> &'static ResponseAnalysisDescriptor;
    /// Borrow immutable configuration metadata for the provider's lifetime.
    fn configuration(&self) -> ResponseConfigurationView<'_>;
    /// Concrete retained payload bytes, excluding the Box/vtable and registry catalog.
    fn retained_bytes(&self) -> usize;
    /// Query into caller-owned buffers without allocation or mutation of provider state.
    fn query_into(
        &self,
        query: ResponseQuery<'_>,
        output: ResponseOutput<'_>,
    ) -> Result<ResponseSummary, ResponseAnalysisError>;
}
