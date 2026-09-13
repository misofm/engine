//! Bounded, immutable response catalogs for a prepared native session.
//!
//! The catalog is a control-plane object. It owns only the selected stable target text, generated
//! frequency axes and the owner-provided prepared response values; it never retains a compiled
//! session clone, effect processor, request slice or response cache.

use core::alloc::Layout;
use core::mem::size_of;
use core::num::NonZeroU64;

use builtins::prepare_input_filter_response;
use builtins_compiler::requested_track_builtin_parameters;
use effect_compiler::{EffectPreparedEntry, EffectPreparedSession, EffectRack};
use effect_contract::{
    NativeEffectResponseFactory, PreparedResponseAnalysis, ResponseAnalysisDescriptor,
    ResponseAnalysisError, ResponseConfigurationView, ResponseOutput, ResponsePrepareLimits,
    ResponseQuery, ResponseSummary, validate_response_analysis_descriptor,
};
use engine::{SampleRateHz, is_launch_sample_rate};
use session::{EffectIdentity, StableId};

/// One named response target in a compiled session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResponseTarget {
    /// The fixed HPF/LPF input section of one track.
    InputFilters {
        /// Stable track identity.
        track_id: StableId,
    },
    /// One native effect slot in one of the three effect racks.
    Effect {
        /// Stable track identity.
        track_id: StableId,
        /// Rack containing the selected slot.
        rack: EffectRack,
        /// Stable session-local effect slot identity.
        effect_slot_id: StableId,
    },
}

/// Borrowed stable identity for an enumerated catalog binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseTargetView<'a> {
    /// The fixed HPF/LPF input section of one track.
    InputFilters {
        /// Borrowed stable track identity.
        track_id: &'a str,
    },
    /// One native effect slot in one of the three effect racks.
    Effect {
        /// Borrowed stable track identity.
        track_id: &'a str,
        /// Rack containing the selected slot.
        rack: EffectRack,
        /// Borrowed stable slot identity.
        effect_slot_id: &'a str,
    },
}

/// Frequency-axis definition retained by one response binding.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResponseFrequencyGrid {
    /// `points` linearly spaced points in the inclusive range.
    Linear {
        /// Number of axis points.
        points: usize,
        /// Inclusive lower frequency in hertz.
        minimum_hz: f32,
        /// Inclusive upper frequency in hertz.
        maximum_hz: f32,
    },
    /// `points` logarithmically spaced points in the inclusive positive range.
    Logarithmic {
        /// Number of axis points.
        points: usize,
        /// Inclusive positive lower frequency in hertz.
        minimum_hz: f32,
        /// Inclusive upper frequency in hertz.
        maximum_hz: f32,
    },
}

impl ResponseFrequencyGrid {
    fn points(self) -> usize {
        match self {
            Self::Linear { points, .. } | Self::Logarithmic { points, .. } => points,
        }
    }

    fn endpoints(self) -> (f32, f32) {
        match self {
            Self::Linear {
                minimum_hz,
                maximum_hz,
                ..
            }
            | Self::Logarithmic {
                minimum_hz,
                maximum_hz,
                ..
            } => (minimum_hz, maximum_hz),
        }
    }
}

/// One requested target, analysis and immutable frequency axis.
#[derive(Clone, Debug, PartialEq)]
pub struct SessionResponseSelection {
    /// Target selected from the accepted session.
    pub target: ResponseTarget,
    /// Owner-local response analysis identity.
    pub analysis_id: u32,
    /// Frequency axis requested for this binding.
    pub grid: ResponseFrequencyGrid,
}

/// Correlation identity retained by one prepared response snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequestedResponseIdentity {
    /// Caller-provided configuration correlation value.
    pub configuration_id: u64,
    /// Optional correlation with an intended plan candidate.
    pub plan_id: Option<u64>,
}

/// Hard caller-owned limits for catalog preparation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionResponseLimits {
    /// Maximum selected bindings.
    pub maximum_bindings: usize,
    /// Maximum axis points in one binding.
    pub maximum_points_per_binding: usize,
    /// Maximum axis points retained across all bindings.
    pub maximum_total_grid_points: usize,
    /// Maximum retained catalog heap payload.
    pub maximum_retained_bytes: usize,
    /// Maximum one retained allocation payload.
    pub maximum_single_allocation_bytes: usize,
}

/// Retained resource facts for one prepared catalog.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionResponseResources {
    /// Number of selected response bindings.
    pub bindings: usize,
    /// Total selected axis points.
    pub grid_points: usize,
    /// Actual retained `f32` axis capacities in bytes.
    pub grid_bytes: usize,
    /// Actual binding-vector allocation in bytes.
    pub metadata_bytes: usize,
    /// Actual retained canonical stable-ID capacities in bytes.
    pub owned_id_bytes: usize,
    /// Owner-reported concrete response payload bytes.
    pub owner_prepared_bytes: usize,
    /// Checked sum of the retained payload categories above.
    pub total_retained_bytes: usize,
    /// Largest actual retained allocation payload.
    pub largest_allocation_bytes: usize,
}

/// Availability of a target's owner-provided response capability.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResponseAvailability {
    /// The descriptor is borrowed from the immutable owner declaration.
    Available {
        /// Owner-authoritative response descriptor.
        descriptor: &'static ResponseAnalysisDescriptor,
    },
    /// The target exists but has no native response provider.
    Unsupported,
}

/// Why a response target, catalog or query was refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionResponseError {
    /// The compiled session uses a non-launch sample rate.
    UnsupportedSampleRate,
    /// The requested track is absent.
    UnknownTrack,
    /// The requested effect slot is absent in the selected rack.
    UnknownEffectSlot,
    /// No prepared entry matches the selected native slot.
    PreparedEntryMissing,
    /// More than one prepared entry matches the selected native slot.
    PreparedEntryAmbiguous,
    /// A selected prepared entry no longer agrees with its session tuple or owner identity.
    PreparedEntryMismatch,
    /// A target exists but its owner has no response provider.
    UnsupportedResponse,
    /// The target's owner does not expose the requested analysis identity.
    UnknownAnalysis,
    /// A target appears more than once in one candidate catalog.
    DuplicateSelection,
    /// The generated axis is invalid or not strictly increasing as `f32`.
    InvalidFrequencyGrid,
    /// A point, shape or checked byte count exceeds a caller limit.
    Capacity,
    /// A fallible retained allocation could not be reserved.
    ReservationFailure,
    /// A retained allocation or owner payload exceeds a byte cap.
    ResourceLimit,
    /// The owner rejected a validated request; its stable code is preserved here.
    Owner(ResponseAnalysisError),
    /// The handle belongs to an older catalog generation.
    StaleHandle,
    /// The handle ordinal is not present in this provider.
    UnknownHandle,
    /// Replacing the catalog would wrap the checked generation counter.
    GenerationExhausted,
    /// An owner returned a descriptor or prepared object inconsistent with discovery.
    OwnerMismatch,
}

/// A private provider-local binding handle.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SessionResponseHandle {
    generation: NonZeroU64,
    ordinal: NonZeroU64,
}

/// Borrowed immutable catalog metadata for one binding.
#[derive(Clone, Copy, Debug)]
pub struct ResponseBindingView<'a> {
    /// Provider-local binding handle.
    pub handle: SessionResponseHandle,
    /// Stable target identity borrowed from catalog-owned text.
    pub target: ResponseTargetView<'a>,
    /// Owner-local analysis identity.
    pub analysis_id: u32,
    /// Owner-authoritative response descriptor.
    pub descriptor: &'static ResponseAnalysisDescriptor,
    /// Normalized axis definition.
    pub grid: ResponseFrequencyGrid,
    /// Immutable generated axis borrowed from the catalog.
    pub frequencies_hz: &'a [f32],
    /// Immutable owner configuration metadata.
    pub configuration: ResponseConfigurationView<'a>,
}

/// Result of one provider query.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SessionResponseSummary {
    /// Complete provider-local handle used for the query.
    pub handle: SessionResponseHandle,
    /// Correlation identity retained by the queried catalog.
    pub identity: RequestedResponseIdentity,
    /// Common owner response summary.
    pub summary: ResponseSummary,
}

/// An independently prepared, immutable response catalog candidate.
pub struct PreparedSessionResponseCatalog {
    identity: RequestedResponseIdentity,
    bindings: Vec<ResponseBinding>,
    resources: SessionResponseResources,
}

impl PreparedSessionResponseCatalog {
    /// Prepare a bounded catalog from one accepted effect session.
    pub fn prepare(
        effects: &EffectPreparedSession,
        identity: RequestedResponseIdentity,
        selections: &[SessionResponseSelection],
        limits: SessionResponseLimits,
    ) -> Result<Self, SessionResponseError> {
        let sample_rate_hz = effects.session.sample_rate().0;
        if !is_launch_sample_rate(SampleRateHz(sample_rate_hz)) {
            return Err(SessionResponseError::UnsupportedSampleRate);
        }
        if selections.len() > limits.maximum_bindings {
            return Err(SessionResponseError::Capacity);
        }

        let mut total_points = 0usize;
        let mut lower_owned_id_bytes = 0usize;
        for (index, selection) in selections.iter().enumerate() {
            let resolved = resolve_target(effects, &selection.target)?;
            let descriptor = resolved.descriptor();
            if descriptor.id != selection.analysis_id {
                return Err(SessionResponseError::UnknownAnalysis);
            }
            let grid = validate_grid_shape(
                selection.grid,
                sample_rate_hz,
                limits.maximum_points_per_binding,
            )?;
            total_points = total_points
                .checked_add(grid.points())
                .ok_or(SessionResponseError::Capacity)?;
            Layout::array::<f32>(grid.points()).map_err(|_| SessionResponseError::Capacity)?;
            if total_points > limits.maximum_total_grid_points {
                return Err(SessionResponseError::Capacity);
            }
            lower_owned_id_bytes = lower_owned_id_bytes
                .checked_add(target_id_bytes(&selection.target)?)
                .ok_or(SessionResponseError::Capacity)?;
            if duplicate_selection(&selections[..index], &selection.target) {
                return Err(SessionResponseError::DuplicateSelection);
            }
        }

        let metadata_layout = Layout::array::<ResponseBinding>(selections.len())
            .map_err(|_| SessionResponseError::Capacity)?;
        let metadata_lower = metadata_layout.size();
        let grid_lower = total_points
            .checked_mul(size_of::<f32>())
            .ok_or(SessionResponseError::Capacity)?;
        let lower_total = metadata_lower
            .checked_add(grid_lower)
            .and_then(|value| value.checked_add(lower_owned_id_bytes))
            .ok_or(SessionResponseError::Capacity)?;
        if lower_total > limits.maximum_retained_bytes
            || metadata_lower > limits.maximum_single_allocation_bytes
        {
            return Err(SessionResponseError::ResourceLimit);
        }
        if selections.iter().any(|selection| {
            let id_bytes = target_id_bytes(&selection.target).unwrap_or(usize::MAX);
            let points_bytes = selection
                .grid
                .points()
                .checked_mul(size_of::<f32>())
                .unwrap_or(usize::MAX);
            id_bytes > limits.maximum_single_allocation_bytes
                || points_bytes > limits.maximum_single_allocation_bytes
        }) {
            return Err(SessionResponseError::ResourceLimit);
        }

        let mut bindings = Vec::new();
        bindings
            .try_reserve_exact(selections.len())
            .map_err(|_| SessionResponseError::ReservationFailure)?;
        let metadata_bytes = bindings
            .capacity()
            .checked_mul(size_of::<ResponseBinding>())
            .ok_or(SessionResponseError::Capacity)?;
        if metadata_bytes > limits.maximum_single_allocation_bytes {
            return Err(SessionResponseError::ResourceLimit);
        }

        let mut grid_bytes = 0usize;
        let mut owned_id_bytes = 0usize;
        let mut owner_prepared_bytes = 0usize;
        let mut largest_allocation_bytes = metadata_bytes;

        for selection in selections {
            let resolved = resolve_target(effects, &selection.target)?;
            let descriptor = resolved.descriptor();
            if descriptor.id != selection.analysis_id {
                return Err(SessionResponseError::UnknownAnalysis);
            }
            let normalized_grid = validate_grid_shape(
                selection.grid,
                sample_rate_hz,
                limits.maximum_points_per_binding,
            )?;
            let frequencies = generate_grid(normalized_grid)?;
            validate_generated_grid(&frequencies, normalized_grid)?;
            let generated_grid_bytes = frequencies
                .len()
                .checked_mul(size_of::<f32>())
                .ok_or(SessionResponseError::Capacity)?;
            if generated_grid_bytes > limits.maximum_single_allocation_bytes {
                return Err(SessionResponseError::ResourceLimit);
            }

            let target = own_target(&selection.target)?;
            let target_bytes = target.owned_bytes();
            if target_bytes > limits.maximum_single_allocation_bytes {
                return Err(SessionResponseError::ResourceLimit);
            }

            let known_before_owner = metadata_bytes
                .checked_add(grid_lower)
                .and_then(|value| value.checked_add(lower_owned_id_bytes))
                .and_then(|value| value.checked_add(owner_prepared_bytes))
                .ok_or(SessionResponseError::Capacity)?;
            let remaining = limits
                .maximum_retained_bytes
                .checked_sub(known_before_owner)
                .unwrap_or(0);
            let owner_limit = remaining.min(limits.maximum_single_allocation_bytes);
            let owner = prepare_owner(
                &resolved,
                sample_rate_hz,
                ResponsePrepareLimits {
                    maximum_prepared_bytes: owner_limit,
                },
            )?;
            if owner.analysis_descriptor().id != descriptor.id
                || owner.configuration().sample_rate_hz != sample_rate_hz
            {
                return Err(SessionResponseError::OwnerMismatch);
            }
            let owner_bytes = owner.retained_bytes();
            if owner_bytes > limits.maximum_single_allocation_bytes {
                return Err(SessionResponseError::ResourceLimit);
            }

            grid_bytes = grid_bytes
                .checked_add(generated_grid_bytes)
                .ok_or(SessionResponseError::Capacity)?;
            owned_id_bytes = owned_id_bytes
                .checked_add(target_bytes)
                .ok_or(SessionResponseError::Capacity)?;
            owner_prepared_bytes = owner_prepared_bytes
                .checked_add(owner_bytes)
                .ok_or(SessionResponseError::Capacity)?;
            let total = metadata_bytes
                .checked_add(grid_bytes)
                .and_then(|value| value.checked_add(owned_id_bytes))
                .and_then(|value| value.checked_add(owner_prepared_bytes))
                .ok_or(SessionResponseError::Capacity)?;
            if total > limits.maximum_retained_bytes {
                return Err(SessionResponseError::ResourceLimit);
            }
            largest_allocation_bytes = largest_allocation_bytes
                .max(generated_grid_bytes)
                .max(target_bytes)
                .max(owner_bytes);
            bindings.push(ResponseBinding {
                target,
                analysis_id: selection.analysis_id,
                descriptor,
                grid: normalized_grid,
                frequencies,
                owner,
            });
        }

        let total_retained_bytes = metadata_bytes
            .checked_add(grid_bytes)
            .and_then(|value| value.checked_add(owned_id_bytes))
            .and_then(|value| value.checked_add(owner_prepared_bytes))
            .ok_or(SessionResponseError::Capacity)?;
        Ok(Self {
            identity,
            bindings,
            resources: SessionResponseResources {
                bindings: selections.len(),
                grid_points: total_points,
                grid_bytes,
                metadata_bytes,
                owned_id_bytes,
                owner_prepared_bytes,
                total_retained_bytes,
                largest_allocation_bytes,
            },
        })
    }

    /// Return the retained resource report.
    #[must_use]
    pub const fn resources(&self) -> SessionResponseResources {
        self.resources
    }
}

/// Provider-local immutable response catalog handle and generation owner.
pub struct SessionResponseProvider {
    catalog: PreparedSessionResponseCatalog,
    generation: NonZeroU64,
}

impl SessionResponseProvider {
    /// Start a provider with generation one.
    #[must_use]
    pub fn new(catalog: PreparedSessionResponseCatalog) -> Self {
        Self {
            catalog,
            generation: NonZeroU64::new(1).expect("literal generation is nonzero"),
        }
    }

    /// Replace the complete catalog and invalidate every handle from the old generation.
    pub fn replace_catalog(
        &mut self,
        candidate: PreparedSessionResponseCatalog,
    ) -> Result<(), SessionResponseError> {
        let next = self
            .generation
            .get()
            .checked_add(1)
            .and_then(NonZeroU64::new)
            .ok_or(SessionResponseError::GenerationExhausted)?;
        self.catalog = candidate;
        self.generation = next;
        Ok(())
    }

    /// Enumerate bindings in the caller's selection order.
    pub fn bindings(&self) -> impl Iterator<Item = ResponseBindingView<'_>> {
        self.catalog
            .bindings
            .iter()
            .enumerate()
            .map(move |(index, binding)| binding.view(self.generation, index))
    }

    /// Return the active catalog resource report.
    #[must_use]
    pub const fn resources(&self) -> SessionResponseResources {
        self.catalog.resources
    }

    /// Query one immutable binding into caller-owned output buffers.
    pub fn query_into(
        &self,
        handle: SessionResponseHandle,
        output: ResponseOutput<'_>,
    ) -> Result<SessionResponseSummary, SessionResponseError> {
        if handle.generation != self.generation {
            return Err(SessionResponseError::StaleHandle);
        }
        let ordinal = usize::try_from(handle.ordinal.get())
            .map_err(|_| SessionResponseError::UnknownHandle)?;
        let Some(index) = ordinal.checked_sub(1) else {
            return Err(SessionResponseError::UnknownHandle);
        };
        let Some(binding) = self.catalog.bindings.get(index) else {
            return Err(SessionResponseError::UnknownHandle);
        };
        let summary = binding
            .owner
            .query_into(
                ResponseQuery {
                    configuration_id: self.catalog.identity.configuration_id,
                    frequencies_hz: &binding.frequencies,
                    maximum_points: binding.frequencies.len(),
                },
                output,
            )
            .map_err(SessionResponseError::Owner)?;
        Ok(SessionResponseSummary {
            handle,
            identity: self.catalog.identity,
            summary,
        })
    }
}

/// Discover one target's owner capability without preparing a provider or retaining a catalog.
pub fn describe_session_response_target(
    effects: &EffectPreparedSession,
    target: &ResponseTarget,
) -> Result<ResponseAvailability, SessionResponseError> {
    Ok(match resolve_target(effects, target)? {
        ResolvedTarget::InputFilters { descriptor, .. }
        | ResolvedTarget::Effect { descriptor, .. } => {
            ResponseAvailability::Available { descriptor }
        }
    })
}

enum ResolvedTarget<'a> {
    InputFilters {
        track: &'a session::Track,
        descriptor: &'static ResponseAnalysisDescriptor,
    },
    Effect {
        entry: &'a EffectPreparedEntry,
        response: &'a dyn NativeEffectResponseFactory,
        descriptor: &'static ResponseAnalysisDescriptor,
    },
}

impl ResolvedTarget<'_> {
    fn descriptor(&self) -> &'static ResponseAnalysisDescriptor {
        match self {
            Self::InputFilters { descriptor, .. } | Self::Effect { descriptor, .. } => descriptor,
        }
    }
}

fn resolve_target<'a>(
    effects: &'a EffectPreparedSession,
    target: &ResponseTarget,
) -> Result<ResolvedTarget<'a>, SessionResponseError> {
    let model = effects.session.normalized_model();
    match target {
        ResponseTarget::InputFilters { track_id } => {
            let track = model
                .tracks
                .iter()
                .find(|track| track.id == *track_id)
                .ok_or(SessionResponseError::UnknownTrack)?;
            let descriptor = builtins::input_filter_response_descriptor();
            validate_response_analysis_descriptor(descriptor)
                .map_err(|_| SessionResponseError::OwnerMismatch)?;
            Ok(ResolvedTarget::InputFilters { track, descriptor })
        }
        ResponseTarget::Effect {
            track_id,
            rack,
            effect_slot_id,
        } => {
            let track = model
                .tracks
                .iter()
                .find(|track| track.id == *track_id)
                .ok_or(SessionResponseError::UnknownTrack)?;
            let rack_declaration = match rack {
                EffectRack::Simd1 => &track.simd1,
                EffectRack::Dynamic => &track.dynamic,
                EffectRack::Simd2 => &track.simd2,
            };
            let effect = rack_declaration
                .effects
                .iter()
                .find(|effect| effect.id == *effect_slot_id)
                .ok_or(SessionResponseError::UnknownEffectSlot)?;
            if !matches!(effect.identity, EffectIdentity::Native { .. }) {
                return Err(SessionResponseError::UnsupportedResponse);
            }
            let mut matching = None;
            let mut count = 0usize;
            for entry in &effects.entries {
                if entry.track_id == track_id.as_str()
                    && entry.rack == *rack
                    && entry.effect_id == effect_slot_id.as_str()
                {
                    matching = Some(entry);
                    count = count.saturating_add(1);
                }
            }
            let entry = match count {
                0 => return Err(SessionResponseError::PreparedEntryMissing),
                1 => matching.expect("one matching entry is retained"),
                _ => return Err(SessionResponseError::PreparedEntryAmbiguous),
            };
            if entry.factory.descriptor().id.as_str() != effect_identity(effect)
                || entry.metadata.descriptor.id != entry.factory.descriptor().id
                || entry.bank_preparation.sample_rate != effects.session.sample_rate().0
                || entry.bank_preparation.quantum != effects.session.quantum().0
                || entry.metadata.sample_rate != effects.session.sample_rate().0
                || entry.metadata.quantum != effects.session.quantum().0
            {
                return Err(SessionResponseError::PreparedEntryMismatch);
            }
            let response = entry
                .factory
                .response_analysis()
                .ok_or(SessionResponseError::UnsupportedResponse)?;
            let descriptor = response.analysis_descriptor();
            validate_response_analysis_descriptor(descriptor)
                .map_err(|_| SessionResponseError::OwnerMismatch)?;
            Ok(ResolvedTarget::Effect {
                entry,
                response,
                descriptor,
            })
        }
    }
}

fn effect_identity(effect: &session::Effect) -> &str {
    match &effect.identity {
        EffectIdentity::Native { effect_id } => effect_id.as_str(),
        EffectIdentity::ThirdPartyCid { .. } => "",
    }
}

fn prepare_owner(
    resolved: &ResolvedTarget<'_>,
    sample_rate_hz: u32,
    limits: ResponsePrepareLimits,
) -> Result<Box<dyn PreparedResponseAnalysis>, SessionResponseError> {
    match resolved {
        ResolvedTarget::InputFilters { track, .. } => {
            let parameters = requested_track_builtin_parameters(track, u32::MAX)
                .map_err(|error| SessionResponseError::Owner(ResponseAnalysisError::Configuration(
                    effect_contract::EffectPrepareError {
                        code: builtin_error_code(error),
                    },
                )))?;
            prepare_input_filter_response(sample_rate_hz, parameters, limits)
                .map_err(SessionResponseError::Owner)
        }
        ResolvedTarget::Effect { entry, response, .. } => response
            .prepare_response(entry.bank_preparation.request(), limits)
            .map_err(SessionResponseError::Owner),
    }
}

fn builtin_error_code(error: builtins::BuiltinParameterError) -> &'static str {
    match error {
        builtins::BuiltinParameterError::GainDomain => "builtin.gain.domain",
        builtins::BuiltinParameterError::FilterCutoff => "builtin.filter.cutoff",
        builtins::BuiltinParameterError::FilterOrder => "builtin.filter.order",
        builtins::BuiltinParameterError::FilterCoefficients => "builtin.filter.coefficients",
        builtins::BuiltinParameterError::MatrixCoefficient => "builtin.matrix.coefficient",
        builtins::BuiltinParameterError::MatrixSmoothing => "builtin.matrix.smoothing",
        builtins::BuiltinParameterError::EmptyBlock
        | builtins::BuiltinParameterError::LaneLength
        | builtins::BuiltinParameterError::SampleTimeOverflow => {
            "builtin.resource.arithmetic_overflow"
        }
    }
}

fn duplicate_selection(previous: &[SessionResponseSelection], target: &ResponseTarget) -> bool {
    previous
        .iter()
        .any(|selection| selection.target == *target)
}

fn target_id_bytes(target: &ResponseTarget) -> Result<usize, SessionResponseError> {
    let lower = match target {
        ResponseTarget::InputFilters { track_id } => track_id.as_str().len(),
        ResponseTarget::Effect {
            track_id,
            effect_slot_id,
            ..
        } => track_id
            .as_str()
            .len()
            .checked_add(effect_slot_id.as_str().len())
            .ok_or(SessionResponseError::Capacity)?,
    };
    Layout::array::<u8>(lower)
        .map(|layout| layout.size())
        .map_err(|_| SessionResponseError::Capacity)
}

fn validate_grid_shape(
    grid: ResponseFrequencyGrid,
    sample_rate_hz: u32,
    maximum_points: usize,
) -> Result<ResponseFrequencyGrid, SessionResponseError> {
    let (mut minimum_hz, maximum_hz) = grid.endpoints();
    let points = grid.points();
    if points < 2 || points > maximum_points {
        return Err(SessionResponseError::Capacity);
    }
    if !minimum_hz.is_finite()
        || !maximum_hz.is_finite()
        || minimum_hz < 0.0
        || minimum_hz >= maximum_hz
        || maximum_hz > sample_rate_hz as f32 * 0.5
    {
        return Err(SessionResponseError::InvalidFrequencyGrid);
    }
    if minimum_hz == 0.0 {
        minimum_hz = 0.0;
    }
    if matches!(grid, ResponseFrequencyGrid::Logarithmic { .. }) && minimum_hz <= 0.0 {
        return Err(SessionResponseError::InvalidFrequencyGrid);
    }
    let normalized = match grid {
        ResponseFrequencyGrid::Linear { .. } => ResponseFrequencyGrid::Linear {
            points,
            minimum_hz,
            maximum_hz,
        },
        ResponseFrequencyGrid::Logarithmic { .. } => ResponseFrequencyGrid::Logarithmic {
            points,
            minimum_hz,
            maximum_hz,
        },
    };
    Ok(normalized)
}

fn validate_generated_grid(
    frequencies: &[f32],
    grid: ResponseFrequencyGrid,
) -> Result<(), SessionResponseError> {
    let (minimum_hz, maximum_hz) = grid.endpoints();
    if frequencies.len() != grid.points()
        || frequencies.first().copied() != Some(minimum_hz)
        || frequencies.last().copied() != Some(maximum_hz)
        || frequencies.iter().any(|frequency| {
            !frequency.is_finite() || *frequency < minimum_hz || *frequency > maximum_hz
        })
        || frequencies.windows(2).any(|pair| pair[1] <= pair[0])
    {
        return Err(SessionResponseError::InvalidFrequencyGrid);
    }
    Ok(())
}

fn generate_grid(grid: ResponseFrequencyGrid) -> Result<Box<[f32]>, SessionResponseError> {
    let points = grid.points();
    let layout = Layout::array::<f32>(points).map_err(|_| SessionResponseError::Capacity)?;
    let mut frequencies = Vec::new();
    frequencies
        .try_reserve_exact(points)
        .map_err(|_| SessionResponseError::ReservationFailure)?;
    let (minimum_hz, maximum_hz) = grid.endpoints();
    let denominator = (points - 1) as f64;
    for index in 0..points {
        let t = index as f64 / denominator;
        let value = match grid {
            ResponseFrequencyGrid::Linear { .. } => {
                f64::from(minimum_hz) + f64::from(maximum_hz - minimum_hz) * t
            }
            ResponseFrequencyGrid::Logarithmic { .. } => {
                (f64::from(minimum_hz).ln()
                    + (f64::from(maximum_hz).ln() - f64::from(minimum_hz).ln()) * t)
                .exp()
            }
        } as f32;
        frequencies.push(value);
    }
    if let Some(first) = frequencies.first_mut() {
        *first = minimum_hz;
    }
    if let Some(last) = frequencies.last_mut() {
        *last = maximum_hz;
    }
    debug_assert_eq!(layout.size(), frequencies.capacity() * size_of::<f32>());
    Ok(frequencies.into_boxed_slice())
}

fn own_target(target: &ResponseTarget) -> Result<OwnedTarget, SessionResponseError> {
    match target {
        ResponseTarget::InputFilters { track_id } => Ok(OwnedTarget::InputFilters {
            track_id: own_id(track_id.as_str())?,
        }),
        ResponseTarget::Effect {
            track_id,
            rack,
            effect_slot_id,
        } => Ok(OwnedTarget::Effect {
            track_id: own_id(track_id.as_str())?,
            rack: *rack,
            effect_slot_id: own_id(effect_slot_id.as_str())?,
        }),
    }
}

fn own_id(value: &str) -> Result<String, SessionResponseError> {
    let mut owned = String::new();
    owned
        .try_reserve_exact(value.len())
        .map_err(|_| SessionResponseError::ReservationFailure)?;
    owned.push_str(value);
    Ok(owned)
}

enum OwnedTarget {
    InputFilters { track_id: String },
    Effect {
        track_id: String,
        rack: EffectRack,
        effect_slot_id: String,
    },
}

impl OwnedTarget {
    fn owned_bytes(&self) -> usize {
        match self {
            Self::InputFilters { track_id } => track_id.capacity(),
            Self::Effect {
                track_id,
                effect_slot_id,
                ..
            } => track_id
                .capacity()
                .saturating_add(effect_slot_id.capacity()),
        }
    }

    fn view(&self) -> ResponseTargetView<'_> {
        match self {
            Self::InputFilters { track_id } => ResponseTargetView::InputFilters {
                track_id: track_id.as_str(),
            },
            Self::Effect {
                track_id,
                rack,
                effect_slot_id,
            } => ResponseTargetView::Effect {
                track_id: track_id.as_str(),
                rack: *rack,
                effect_slot_id: effect_slot_id.as_str(),
            },
        }
    }
}

struct ResponseBinding {
    target: OwnedTarget,
    analysis_id: u32,
    descriptor: &'static ResponseAnalysisDescriptor,
    grid: ResponseFrequencyGrid,
    frequencies: Box<[f32]>,
    owner: Box<dyn PreparedResponseAnalysis>,
}

impl ResponseBinding {
    fn view(&self, generation: NonZeroU64, index: usize) -> ResponseBindingView<'_> {
        let ordinal = u64::try_from(index)
            .ok()
            .and_then(|value| value.checked_add(1))
            .and_then(NonZeroU64::new)
            .expect("a retained binding ordinal fits in u64");
        ResponseBindingView {
            handle: SessionResponseHandle {
                generation,
                ordinal,
            },
            target: self.target.view(),
            analysis_id: self.analysis_id,
            descriptor: self.descriptor,
            grid: self.grid,
            frequencies_hz: &self.frequencies,
            configuration: self.owner.configuration(),
        }
    }
}
