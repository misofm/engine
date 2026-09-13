//! Explicit, stopped response previews owned by the native effect providers.
//!
//! This module is deliberately a small bridge.  It does not parse a session, inspect a graph or
//! retain a render plan: a caller supplies one native effect configuration (or one pair of input
//! filters), the owner prepares it through its existing response factory, and a query writes into
//! caller-owned buffers.  The browser adapter uses the same entry point, so native and Wasm
//! previews cannot grow separate numerical implementations.

use builtins::{BuiltinParameters, prepare_input_filter_response};
use effect_compiler::launch_native_effect_registry;
use effect_contract::{
    EffectQuality, InitialParameterValue, LinkMode, ParameterChannel, PreparedPorts,
    PreparedResponseAnalysis, ResponseAnalysisError, ResponseOutput, ResponsePrepareLimits,
    ResponseQuery, ResponseSummary, default_initial_values, normalize_zero,
};
use engine::realtime::{
    ResponseSnapshotAvailability, ResponseSnapshotCapture, ResponseSnapshotError,
    ResponseSnapshotOwnerInfo, ResponseSnapshotSection, ResponseSnapshotSink,
};
use math::{exp, log};

/// The explicit target supplied by a stopped preview caller.
#[derive(Clone, Copy, Debug)]
pub enum ResponsePreviewTarget<'a> {
    /// One native effect owner, identified by its generated effect ID.
    Effect {
        /// Generated native effect identity.
        effect_id: &'a str,
        /// Numeric parameter/channel overrides over descriptor defaults.
        overrides: &'a [ResponseParameterOverride],
        /// Native preparation quality.
        quality: EffectQuality,
        /// Effect-wide bypass.
        bypass: bool,
        /// Native channel-link mode.
        link_mode: LinkMode,
    },
    /// The builtin input HPF/LPF pair. Other builtin sections remain at their defaults.
    InputFilters {
        /// Left high-pass cutoff, in Hz.
        left_hpf_hz: f32,
        /// Left low-pass cutoff, in Hz.
        left_lpf_hz: f32,
        /// Right high-pass cutoff, in Hz.
        right_hpf_hz: f32,
        /// Right low-pass cutoff, in Hz.
        right_lpf_hz: f32,
    },
}

/// One numeric override in descriptor parameter ID/channel space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResponseParameterOverride {
    /// Native descriptor parameter ID.
    pub parameter_id: u32,
    /// Native parameter channel.
    pub channel: ParameterChannel,
    /// The descriptor-domain value, before owner validation.
    pub value: f32,
}

/// Bounded preparation resources for one response preview.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResponsePreviewLimits {
    /// Maximum concrete owner payload retained by preparation.
    pub maximum_prepared_bytes: usize,
    /// Maximum ordinary native effect state admitted while validating the request.
    pub maximum_total_state_bytes: u64,
    /// Maximum ordinary native effect scratch admitted while validating the request.
    pub maximum_scratch_bytes: u64,
    /// Maximum ordinary automation span capacity admitted while validating the request.
    pub maximum_automation_spans_per_block: u32,
}

impl Default for ResponsePreviewLimits {
    fn default() -> Self {
        Self {
            maximum_prepared_bytes: 1 << 20,
            maximum_total_state_bytes: 1 << 30,
            maximum_scratch_bytes: 1 << 30,
            maximum_automation_spans_per_block: 4096,
        }
    }
}

/// A generated response frequency axis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResponsePreviewGrid {
    /// Evenly spaced inclusive endpoints.
    Linear {
        /// Number of points, at least two.
        points: usize,
        /// Inclusive lower endpoint in Hz.
        minimum_hz: f32,
        /// Inclusive upper endpoint in Hz.
        maximum_hz: f32,
    },
    /// Logarithmically spaced inclusive positive endpoints.
    Logarithmic {
        /// Number of points, at least two.
        points: usize,
        /// Inclusive positive lower endpoint in Hz.
        minimum_hz: f32,
        /// Inclusive upper endpoint in Hz.
        maximum_hz: f32,
    },
}

impl ResponsePreviewGrid {
    /// Number of requested points.
    #[must_use]
    pub const fn points(self) -> usize {
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

/// A stopped response preparation request.
#[derive(Clone, Copy, Debug)]
pub struct ResponsePreviewRequest<'a> {
    /// Opaque caller correlation identity.
    pub configuration_id: u64,
    /// Explicit launch sample rate.
    pub sample_rate_hz: u32,
    /// Explicit render quantum used by native effect validation.
    pub quantum_frames: u32,
    /// One owner-local target.
    pub target: ResponsePreviewTarget<'a>,
    /// Preparation/resource caps.
    pub limits: ResponsePreviewLimits,
}

/// Caller-owned output buffers for one response query.
#[derive(Debug)]
pub struct ResponsePreviewOutput<'a> {
    /// Engine-generated frequency axis.
    pub frequencies_hz: &'a mut [f32],
    /// Left total curve.
    pub total_left_db: &'a mut [f32],
    /// Right total curve.
    pub total_right_db: &'a mut [f32],
    /// Optional section-major left curves.
    pub sections_left_db: Option<&'a mut [f32]>,
    /// Optional section-major right curves.
    pub sections_right_db: Option<&'a mut [f32]>,
}

/// Why an explicit preview request was refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponsePreviewError {
    /// The requested launch rate/quantum or grid shape is invalid.
    InvalidShape,
    /// The requested native effect ID is absent.
    UnknownEffect,
    /// The effect exists but has no response provider.
    UnsupportedEffect,
    /// A parameter ID/channel row is not valid for this descriptor.
    InvalidParameter,
    /// A parameter ID/channel row was supplied more than once.
    DuplicateParameter,
    /// A shared parameter was addressed as a lane or a lane parameter as both.
    ConflictingChannel,
    /// The owner rejected the explicit preparation.
    Owner(ResponseAnalysisError),
    /// The generated axis is not finite, increasing and inside Nyquist.
    InvalidGrid,
    /// Caller buffers do not exactly match the requested shape.
    OutputShape,
}

/// One owner copied from an exclusive live-plan boundary.
#[derive(Clone, Debug)]
pub struct ResponseSnapshotOwner {
    /// Stable track identity repeated for convenient worker-side validation.
    pub track_id: Box<str>,
    /// Stable native owner identity in declared signal order.
    pub stable_id: Box<str>,
    /// Native rack/stage identity.
    pub rack: u8,
    /// Declared signal-chain slot, independent of bank execution order.
    pub slot: u32,
    /// Owner response kind (`1` EQ, `2` input filters).
    pub kind: u32,
    /// Bypass state captured at the same boundary as the words.
    pub bypassed: bool,
    /// Whether the owner supplied a linear response provider.
    pub availability: ResponseSnapshotAvailability,
    /// Rounded left target sections.
    pub left: Box<[ResponseSnapshotSection]>,
    /// Rounded right target sections.
    pub right: Box<[ResponseSnapshotSection]>,
}

/// Immutable, host-owned copy of one selected track's response owners.
#[derive(Clone, Debug)]
pub struct ResponseSnapshot {
    /// Validated launch sample rate used by every owner.
    pub sample_rate_hz: u32,
    /// Actual next absolute sample at the completed render boundary.
    pub captured_sample: u64,
    /// Owners in declared signal order, including explicit unavailable exclusions.
    pub owners: Box<[ResponseSnapshotOwner]>,
}

/// Preallocated collector for an exclusive engine snapshot callback.
///
/// Construct this storage off render, then capture without allocations on the plan-owning thread.
/// Finish the owned worker representation off render after the capture has completed.
pub struct ResponseSnapshotCollector {
    sample_rate_hz: u32,
    maximum_sections_per_owner: usize,
    next_owner: usize,
    owners: Vec<CollectorOwner>,
}

const COLLECTOR_SECTION_CAPACITY: usize = 4;

struct CollectorOwner {
    track_id: String,
    stable_id: String,
    rack: u8,
    slot: u32,
    kind: u32,
    bypassed: bool,
    availability: ResponseSnapshotAvailability,
    left: [ResponseSnapshotSection; COLLECTOR_SECTION_CAPACITY],
    right: [ResponseSnapshotSection; COLLECTOR_SECTION_CAPACITY],
    left_len: usize,
    right_len: usize,
}

impl ResponseSnapshotCollector {
    /// Allocate bounded worker-side storage for one capture.
    #[must_use]
    pub fn new(
        sample_rate_hz: u32,
        maximum_owners: usize,
        maximum_sections_per_owner: usize,
        maximum_identity_bytes: usize,
    ) -> Self {
        let empty = ResponseSnapshotSection {
            id: 0,
            kind: 0,
            enabled: false,
            word_count: 0,
            words: [0; effect_contract::RESPONSE_SNAPSHOT_WORDS],
        };
        let owners = (0..maximum_owners)
            .map(|_| CollectorOwner {
                track_id: String::with_capacity(maximum_identity_bytes),
                stable_id: String::with_capacity(maximum_identity_bytes),
                rack: 0,
                slot: 0,
                kind: 0,
                bypassed: false,
                availability: ResponseSnapshotAvailability::DeclaredUnavailable,
                left: [empty; COLLECTOR_SECTION_CAPACITY],
                right: [empty; COLLECTOR_SECTION_CAPACITY],
                left_len: 0,
                right_len: 0,
            })
            .collect();
        Self {
            sample_rate_hz,
            maximum_sections_per_owner: maximum_sections_per_owner.min(COLLECTOR_SECTION_CAPACITY),
            next_owner: 0,
            owners,
        }
    }

    /// Finish the collector off render after the engine owner reports a successful capture.
    #[must_use]
    pub fn finish(self, capture: ResponseSnapshotCapture) -> ResponseSnapshot {
        let owner_count = self.next_owner;
        let owners = self
            .owners
            .into_iter()
            .take(owner_count)
            .map(|owner| ResponseSnapshotOwner {
                track_id: owner.track_id.into_boxed_str(),
                stable_id: owner.stable_id.into_boxed_str(),
                rack: owner.rack,
                slot: owner.slot,
                kind: owner.kind,
                bypassed: owner.bypassed,
                availability: owner.availability,
                left: owner.left[..owner.left_len].to_vec().into_boxed_slice(),
                right: owner.right[..owner.right_len].to_vec().into_boxed_slice(),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        ResponseSnapshot {
            sample_rate_hz: self.sample_rate_hz,
            captured_sample: capture.captured_sample,
            owners,
        }
    }
}

impl ResponseSnapshotSink for ResponseSnapshotCollector {
    fn copy_owner(
        &mut self,
        owner: ResponseSnapshotOwnerInfo<'_>,
        left: &[ResponseSnapshotSection],
        right: &[ResponseSnapshotSection],
    ) -> Result<(), ResponseSnapshotError> {
        if self.next_owner >= self.owners.len()
            || left.len() > self.maximum_sections_per_owner
            || right.len() > self.maximum_sections_per_owner
            || left.len() > COLLECTOR_SECTION_CAPACITY
            || right.len() > COLLECTOR_SECTION_CAPACITY
            || owner.track_id.len() > self.owners[self.next_owner].track_id.capacity()
            || owner.stable_id.len() > self.owners[self.next_owner].stable_id.capacity()
        {
            return Err(ResponseSnapshotError::Capacity);
        }
        let destination = &mut self.owners[self.next_owner];
        destination.track_id.clear();
        destination.track_id.push_str(owner.track_id);
        destination.stable_id.clear();
        destination.stable_id.push_str(owner.stable_id);
        destination.rack = owner.rack;
        destination.slot = owner.slot;
        destination.kind = owner.kind;
        destination.bypassed = owner.bypassed;
        destination.availability = owner.availability;
        destination.left[..left.len()].copy_from_slice(left);
        destination.right[..right.len()].copy_from_slice(right);
        destination.left_len = left.len();
        destination.right_len = right.len();
        self.next_owner += 1;
        Ok(())
    }
}

/// Caller-owned output for a composed live track subtotal.
#[derive(Debug)]
pub struct ResponseSnapshotOutput<'a> {
    /// Engine-generated frequency axis.
    pub frequencies_hz: &'a mut [f32],
    /// Unfloored-composed left subtotal, published once at the end.
    pub total_left_db: &'a mut [f32],
    /// Unfloored-composed right subtotal, published once at the end.
    pub total_right_db: &'a mut [f32],
}

/// Why a copied live snapshot could not be composed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseSnapshotQueryError {
    /// The generated axis or sample rate is invalid.
    InvalidGrid,
    /// A point/owner budget or checked scratch allocation was exceeded.
    Capacity,
    /// Caller buffers or owner section shapes do not match their contract.
    OutputShape,
    /// The snapshot has no applicable linear provider after ordered exclusions.
    UnsupportedProvider,
    /// A provider's copied words are malformed.
    MalformedSnapshot,
    /// A provider or composed magnitude was non-finite/negative.
    Numerical,
}

/// Metadata returned by a successful composed target query.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResponseSnapshotQuerySummary {
    /// The immutable boundary sample associated with the copied target words.
    pub captured_sample: u64,
    /// Validated sample rate shared by every owner.
    pub sample_rate_hz: u32,
    /// Number of owners copied, including unavailable exclusions.
    pub owners: u32,
    /// Number of declared owners without a linear provider.
    pub excluded_owners: u32,
    /// The frozen live-query mode.
    pub mode: ResponseSnapshotMode,
}

/// Semantic mode for a live target snapshot query.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseSnapshotMode {
    /// The response of the target words held at the capture boundary.
    Target,
}

impl crate::prepare::PreparedHost {
    /// Copy one selected track's prepared response owners at the current exclusive boundary.
    ///
    /// The host must call this between render calls. The engine callback borrows prepared words
    /// only for the duration of each sink call; a caller that needs allocation-free capture can
    /// provide fixed storage through [`ResponseSnapshotSink`].
    pub fn copy_response_snapshot(
        &mut self,
        track_id: &str,
        sink: &mut dyn ResponseSnapshotSink,
    ) -> Result<ResponseSnapshotCapture, ResponseSnapshotError> {
        self.plan
            .copy_response_snapshot(engine::realtime::ResponseSnapshotRequest { track_id, sink })
    }
}

/// An immutable native response owner prepared for repeated queries.
pub struct PreparedResponsePreview {
    owner: Box<dyn PreparedResponseAnalysis>,
    configuration_id: u64,
}

impl PreparedResponsePreview {
    /// The owner descriptor used by this preparation.
    #[must_use]
    pub fn descriptor(&self) -> &'static effect_contract::ResponseAnalysisDescriptor {
        self.owner.analysis_descriptor()
    }

    /// The owner's normalized immutable configuration view.
    #[must_use]
    pub fn configuration(&self) -> effect_contract::ResponseConfigurationView<'_> {
        self.owner.configuration()
    }

    /// Concrete owner payload bytes retained by this preview.
    #[must_use]
    pub fn retained_bytes(&self) -> usize {
        self.owner.retained_bytes()
    }

    /// Query a generated axis into caller-owned buffers.
    pub fn query_into(
        &self,
        grid: ResponsePreviewGrid,
        output: ResponsePreviewOutput<'_>,
    ) -> Result<ResponseSummary, ResponsePreviewError> {
        let points = grid.points();
        if points < 2 || output.frequencies_hz.len() != points {
            return Err(ResponsePreviewError::InvalidGrid);
        }
        let sample_rate_hz = self.configuration().sample_rate_hz;
        generate_response_grid(grid, sample_rate_hz, output.frequencies_hz)?;
        if output.total_left_db.len() != points || output.total_right_db.len() != points {
            return Err(ResponsePreviewError::OutputShape);
        }
        let sections = self.descriptor().sections.len();
        for values in [
            output.sections_left_db.as_deref(),
            output.sections_right_db.as_deref(),
        ]
        .into_iter()
        .flatten()
        {
            if values.len()
                != sections
                    .checked_mul(points)
                    .ok_or(ResponsePreviewError::OutputShape)?
            {
                return Err(ResponsePreviewError::OutputShape);
            }
        }
        self.owner
            .query_into(
                ResponseQuery {
                    configuration_id: self.configuration_id,
                    frequencies_hz: output.frequencies_hz,
                    maximum_points: points,
                },
                ResponseOutput {
                    total_left_db: output.total_left_db,
                    total_right_db: output.total_right_db,
                    sections_left_db: output.sections_left_db,
                    sections_right_db: output.sections_right_db,
                },
            )
            .map_err(ResponsePreviewError::Owner)
    }
}

/// Prepare one explicit owner response without compiling a session or instantiating an effect
/// processor.
pub fn prepare_response_preview(
    request: ResponsePreviewRequest<'_>,
) -> Result<PreparedResponsePreview, ResponsePreviewError> {
    if request.sample_rate_hz == 0 || request.quantum_frames == 0 {
        return Err(ResponsePreviewError::InvalidShape);
    }
    let limits = ResponsePrepareLimits {
        maximum_prepared_bytes: request.limits.maximum_prepared_bytes,
    };
    let owner = match request.target {
        ResponsePreviewTarget::InputFilters {
            left_hpf_hz,
            left_lpf_hz,
            right_hpf_hz,
            right_lpf_hz,
        } => {
            let mut configuration = BuiltinParameters::default();
            configuration.left.hpf_hz = left_hpf_hz;
            configuration.left.lpf_hz = left_lpf_hz;
            configuration.right.hpf_hz = right_hpf_hz;
            configuration.right.lpf_hz = right_lpf_hz;
            prepare_input_filter_response(request.sample_rate_hz, configuration, limits)
                .map_err(ResponsePreviewError::Owner)?
        }
        ResponsePreviewTarget::Effect {
            effect_id,
            overrides,
            quality,
            bypass,
            link_mode,
        } => {
            let registry =
                launch_native_effect_registry().map_err(|_| ResponsePreviewError::UnknownEffect)?;
            let factory = registry
                .get_ascii(effect_id)
                .ok_or(ResponsePreviewError::UnknownEffect)?;
            let response = factory
                .response_analysis()
                .ok_or(ResponsePreviewError::UnsupportedEffect)?;
            let descriptor = factory.descriptor();
            let mut initial_values: Vec<InitialParameterValue> =
                default_initial_values(descriptor).collect();
            apply_overrides(descriptor, &mut initial_values, overrides)?;
            response
                .prepare_response(
                    effect_contract::PrepareEffectRequest {
                        sample_rate: request.sample_rate_hz,
                        quantum: request.quantum_frames,
                        quality,
                        bypass,
                        link_mode,
                        ports: PreparedPorts {
                            sidechain: effect_contract::PreparedSidechainPort::None,
                        },
                        initial_values: &initial_values,
                        limits: effect_contract::PrepareEffectLimits {
                            maximum_total_state_bytes: request.limits.maximum_total_state_bytes,
                            maximum_scratch_bytes: request.limits.maximum_scratch_bytes,
                            maximum_automation_spans_per_block: request
                                .limits
                                .maximum_automation_spans_per_block,
                        },
                    },
                    limits,
                )
                .map_err(ResponsePreviewError::Owner)?
        }
    };
    Ok(PreparedResponsePreview {
        owner,
        configuration_id: request.configuration_id,
    })
}

fn apply_overrides(
    descriptor: &'static effect_contract::EffectDescriptor,
    initial_values: &mut [InitialParameterValue],
    overrides: &[ResponseParameterOverride],
) -> Result<(), ResponsePreviewError> {
    for (index, override_value) in overrides.iter().enumerate() {
        let parameter = descriptor
            .parameters
            .iter()
            .find(|parameter| parameter.id.0 == override_value.parameter_id)
            .ok_or(ResponsePreviewError::InvalidParameter)?;
        let expected_channel = match parameter.channel_policy {
            effect_contract::ParameterChannelPolicy::Shared => {
                if override_value.channel != ParameterChannel::Both {
                    return Err(ResponsePreviewError::ConflictingChannel);
                }
                ParameterChannel::Both
            }
            effect_contract::ParameterChannelPolicy::PerLane => {
                if override_value.channel == ParameterChannel::Both {
                    return Err(ResponsePreviewError::ConflictingChannel);
                }
                override_value.channel
            }
        };
        let Some(slot) = initial_values.iter_mut().find(|value| {
            value.parameter_index == index_of_parameter(descriptor, parameter)
                && value.channel == expected_channel
        }) else {
            return Err(ResponsePreviewError::InvalidParameter);
        };
        if slot.value.to_bits() != parameter.default_value.to_bits()
            && slot.value.to_bits() != normalize_zero(parameter.default_value).to_bits()
        {
            return Err(ResponsePreviewError::DuplicateParameter);
        }
        // The exact slot is still marked below; a separate bitset is not needed because a
        // duplicate is detected by the sentinel's original default value for every descriptor.
        // Defaults can be repeated, so use the preceding rows to make duplicate detection exact.
        if overrides[..index].iter().any(|previous| {
            previous.parameter_id == override_value.parameter_id
                && previous.channel == override_value.channel
        }) {
            return Err(ResponsePreviewError::DuplicateParameter);
        }
        slot.value = override_value.value;
    }
    Ok(())
}

fn index_of_parameter(
    descriptor: &'static effect_contract::EffectDescriptor,
    parameter: &effect_contract::ParameterDescriptor,
) -> u32 {
    descriptor
        .parameters
        .iter()
        .position(|candidate| core::ptr::eq(candidate, parameter))
        .expect("parameter came from descriptor") as u32
}

/// Generate and validate one explicit frequency axis with portable f64 interpolation.
pub fn generate_response_grid(
    grid: ResponsePreviewGrid,
    sample_rate_hz: u32,
    output: &mut [f32],
) -> Result<(), ResponsePreviewError> {
    let points = grid.points();
    let (minimum_hz, maximum_hz) = grid.endpoints();
    if points < 2
        || output.len() != points
        || !minimum_hz.is_finite()
        || !maximum_hz.is_finite()
        || minimum_hz < 0.0
        || minimum_hz >= maximum_hz
        || maximum_hz > sample_rate_hz as f32 * 0.5
        || (matches!(grid, ResponsePreviewGrid::Logarithmic { .. }) && minimum_hz <= 0.0)
    {
        return Err(ResponsePreviewError::InvalidGrid);
    }
    let denominator = (points - 1) as f64;
    for index in 0..points {
        let t = index as f64 / denominator;
        let result = match grid {
            ResponsePreviewGrid::Linear { .. } => {
                let minimum = f64::from(minimum_hz);
                let maximum = f64::from(maximum_hz);
                minimum + (maximum - minimum) * t
            }
            ResponsePreviewGrid::Logarithmic { .. } => {
                let minimum = f64::from(minimum_hz);
                let maximum = f64::from(maximum_hz);
                exp(log(minimum) + (log(maximum) - log(minimum)) * t)
            }
        } as f32;
        if !result.is_finite() || (index > 0 && result <= output[index - 1]) {
            return Err(ResponsePreviewError::InvalidGrid);
        }
        output[index] = result;
    }
    output[0] = minimum_hz;
    output[points - 1] = maximum_hz;
    Ok(())
}

/// Compose one copied track snapshot through the existing native owner evaluators.
///
/// Each owner writes positive unfloored magnitudes into bounded scratch arrays.  The host sums
/// `log10` magnitudes in `f64`, remembers exact zeros separately, and applies the public `-120 dB`
/// floor only once after every owner has been validated.  No render state or caller parameters
/// participate in this operation.
pub fn query_response_snapshot_into(
    snapshot: &ResponseSnapshot,
    grid: ResponsePreviewGrid,
    output: ResponseSnapshotOutput<'_>,
) -> Result<ResponseSnapshotQuerySummary, ResponseSnapshotQueryError> {
    let points = grid.points();
    if points < 2
        || output.frequencies_hz.len() != points
        || output.total_left_db.len() != points
        || output.total_right_db.len() != points
    {
        return Err(ResponseSnapshotQueryError::OutputShape);
    }
    let mut frequencies = Vec::new();
    frequencies
        .try_reserve_exact(points)
        .map_err(|_| ResponseSnapshotQueryError::Capacity)?;
    frequencies.resize(points, 0.0);
    generate_response_grid(grid, snapshot.sample_rate_hz, &mut frequencies)
        .map_err(|_| ResponseSnapshotQueryError::InvalidGrid)?;

    let mut left_sum = Vec::new();
    let mut right_sum = Vec::new();
    let mut left_magnitudes = Vec::new();
    let mut right_magnitudes = Vec::new();
    for values in [
        &mut left_sum,
        &mut right_sum,
        &mut left_magnitudes,
        &mut right_magnitudes,
    ] {
        values
            .try_reserve_exact(points)
            .map_err(|_| ResponseSnapshotQueryError::Capacity)?;
        values.resize(points, 0.0);
    }
    let mut left_zero = vec![false; points];
    let mut right_zero = vec![false; points];
    let mut provided = 0_u32;
    let mut excluded = 0_u32;

    for owner in snapshot.owners.iter() {
        if owner.availability == ResponseSnapshotAvailability::DeclaredUnavailable {
            excluded = excluded
                .checked_add(1)
                .ok_or(ResponseSnapshotQueryError::Capacity)?;
            continue;
        }
        if owner.left.is_empty() || owner.right.is_empty() {
            return Err(ResponseSnapshotQueryError::MalformedSnapshot);
        }
        match owner.kind {
            1 => effect_compiler::query_parametric_eq_snapshot_magnitudes_into(
                snapshot.sample_rate_hz,
                owner.bypassed,
                &owner.left,
                &owner.right,
                &frequencies,
                points,
                &mut left_magnitudes,
                &mut right_magnitudes,
            )
            .map_err(map_eq_snapshot_error)?,
            2 => builtins::query_input_filter_snapshot_magnitudes_into(
                snapshot.sample_rate_hz,
                owner.bypassed,
                &owner.left,
                &owner.right,
                &frequencies,
                points,
                &mut left_magnitudes,
                &mut right_magnitudes,
            )
            .map_err(map_builtin_snapshot_error)?,
            _ => return Err(ResponseSnapshotQueryError::MalformedSnapshot),
        }
        provided = provided
            .checked_add(1)
            .ok_or(ResponseSnapshotQueryError::Capacity)?;
        for index in 0..points {
            let left = left_magnitudes[index];
            let right = right_magnitudes[index];
            if !left.is_finite() || !right.is_finite() || left < 0.0 || right < 0.0 {
                return Err(ResponseSnapshotQueryError::Numerical);
            }
            if left == 0.0 {
                left_zero[index] = true;
            } else if !left_zero[index] {
                left_sum[index] += math::log10(left);
            }
            if right == 0.0 {
                right_zero[index] = true;
            } else if !right_zero[index] {
                right_sum[index] += math::log10(right);
            }
        }
    }
    if provided == 0 {
        return Err(ResponseSnapshotQueryError::UnsupportedProvider);
    }
    for index in 0..points {
        let left_db = if left_zero[index] {
            -120.0
        } else {
            20.0 * left_sum[index]
        };
        let right_db = if right_zero[index] {
            -120.0
        } else {
            20.0 * right_sum[index]
        };
        if !left_db.is_finite() || !right_db.is_finite() {
            return Err(ResponseSnapshotQueryError::Numerical);
        }
        output.total_left_db[index] = (left_db as f32).max(-120.0);
        output.total_right_db[index] = (right_db as f32).max(-120.0);
    }
    output.frequencies_hz.copy_from_slice(&frequencies);
    Ok(ResponseSnapshotQuerySummary {
        captured_sample: snapshot.captured_sample,
        sample_rate_hz: snapshot.sample_rate_hz,
        owners: u32::try_from(snapshot.owners.len())
            .map_err(|_| ResponseSnapshotQueryError::Capacity)?,
        excluded_owners: excluded,
        mode: ResponseSnapshotMode::Target,
    })
}

fn map_eq_snapshot_error(error: effect_compiler::EqResponseError) -> ResponseSnapshotQueryError {
    match error {
        effect_compiler::EqResponseError::InvalidFrequencyGrid => {
            ResponseSnapshotQueryError::InvalidGrid
        }
        effect_compiler::EqResponseError::Capacity => ResponseSnapshotQueryError::Capacity,
        effect_compiler::EqResponseError::OutputShape => {
            ResponseSnapshotQueryError::MalformedSnapshot
        }
        effect_compiler::EqResponseError::Numerical => ResponseSnapshotQueryError::Numerical,
        effect_compiler::EqResponseError::Configuration(_) => {
            ResponseSnapshotQueryError::MalformedSnapshot
        }
    }
}

fn map_builtin_snapshot_error(
    error: builtins::InputFilterResponseError,
) -> ResponseSnapshotQueryError {
    match error {
        builtins::InputFilterResponseError::UnsupportedSampleRate
        | builtins::InputFilterResponseError::InvalidFrequencyGrid => {
            ResponseSnapshotQueryError::InvalidGrid
        }
        builtins::InputFilterResponseError::Capacity => ResponseSnapshotQueryError::Capacity,
        builtins::InputFilterResponseError::OutputShape
        | builtins::InputFilterResponseError::Configuration(_) => {
            ResponseSnapshotQueryError::MalformedSnapshot
        }
        builtins::InputFilterResponseError::Numerical => ResponseSnapshotQueryError::Numerical,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        HostPrepareCaps, HostShapePolicy, compile_host_model, parse_host_session,
        prepare_host_runtime,
    };
    use effect_contract::{EffectQuality, LinkMode, ParameterChannel};

    #[test]
    fn explicit_eq_and_input_filter_queries_use_owner_outputs_at_launch_rates() {
        for sample_rate_hz in [44_100, 48_000, 88_200, 96_000] {
            let eq = prepare_response_preview(ResponsePreviewRequest {
                configuration_id: 9_007_199_254_740_993,
                sample_rate_hz,
                quantum_frames: 128,
                target: ResponsePreviewTarget::Effect {
                    effect_id: "miso.parametric-eq",
                    overrides: &[],
                    quality: EffectQuality::Normal,
                    bypass: false,
                    link_mode: LinkMode::DualMono,
                },
                limits: ResponsePreviewLimits::default(),
            })
            .expect("EQ response owner");
            let mut frequencies = [0.0; 4];
            let mut left = [0.0; 4];
            let mut right = [0.0; 4];
            let mut sections_left = [0.0; 16];
            let mut sections_right = [0.0; 16];
            let summary = eq
                .query_into(
                    ResponsePreviewGrid::Linear {
                        points: 4,
                        minimum_hz: 0.0,
                        maximum_hz: sample_rate_hz as f32 * 0.5,
                    },
                    ResponsePreviewOutput {
                        frequencies_hz: &mut frequencies,
                        total_left_db: &mut left,
                        total_right_db: &mut right,
                        sections_left_db: Some(&mut sections_left),
                        sections_right_db: Some(&mut sections_right),
                    },
                )
                .expect("EQ query");
            assert_eq!(summary.configuration_id, 9_007_199_254_740_993);
            assert!(
                left.iter()
                    .chain(right.iter())
                    .all(|value| value.is_finite())
            );
            assert_eq!(frequencies[0], 0.0);
            assert_eq!(frequencies[3], sample_rate_hz as f32 * 0.5);

            let filters = prepare_response_preview(ResponsePreviewRequest {
                configuration_id: 7,
                sample_rate_hz,
                quantum_frames: 128,
                target: ResponsePreviewTarget::InputFilters {
                    left_hpf_hz: 80.0,
                    left_lpf_hz: 18_000.0_f32.min(sample_rate_hz as f32 * 0.45),
                    right_hpf_hz: 120.0,
                    right_lpf_hz: 16_000.0_f32.min(sample_rate_hz as f32 * 0.45),
                },
                limits: ResponsePreviewLimits::default(),
            })
            .expect("input-filter response owner");
            let mut frequencies = [0.0; 3];
            let mut left = [0.0; 3];
            let mut right = [0.0; 3];
            let mut sections_left = [0.0; 6];
            let mut sections_right = [0.0; 6];
            filters
                .query_into(
                    ResponsePreviewGrid::Logarithmic {
                        points: 3,
                        minimum_hz: 20.0,
                        maximum_hz: sample_rate_hz as f32 * 0.45,
                    },
                    ResponsePreviewOutput {
                        frequencies_hz: &mut frequencies,
                        total_left_db: &mut left,
                        total_right_db: &mut right,
                        sections_left_db: Some(&mut sections_left),
                        sections_right_db: Some(&mut sections_right),
                    },
                )
                .expect("input-filter query");
            assert!(
                left.iter()
                    .chain(right.iter())
                    .all(|value| value.is_finite())
            );
            assert!(frequencies.windows(2).all(|pair| pair[1] > pair[0]));
        }
    }

    #[test]
    fn effect_overrides_reject_unknown_duplicate_and_conflicting_rows() {
        let registry = launch_native_effect_registry().expect("registry");
        let descriptor = registry
            .get_ascii("miso.parametric-eq")
            .expect("EQ")
            .descriptor();
        let id = descriptor.parameters[0].id.0;
        fn request<'a>(overrides: &'a [ResponseParameterOverride]) -> ResponsePreviewRequest<'a> {
            ResponsePreviewRequest {
                configuration_id: 1,
                sample_rate_hz: 48_000,
                quantum_frames: 128,
                target: ResponsePreviewTarget::Effect {
                    effect_id: "miso.parametric-eq",
                    overrides,
                    quality: EffectQuality::Normal,
                    bypass: false,
                    link_mode: LinkMode::DualMono,
                },
                limits: ResponsePreviewLimits::default(),
            }
        }
        assert!(matches!(
            prepare_response_preview(request(&[ResponseParameterOverride {
                parameter_id: u32::MAX,
                channel: ParameterChannel::Left,
                value: 1.0,
            }]))
            .err(),
            Some(ResponsePreviewError::InvalidParameter)
        ));
        let duplicate = [
            ResponseParameterOverride {
                parameter_id: id,
                channel: ParameterChannel::Left,
                value: 1.0,
            },
            ResponseParameterOverride {
                parameter_id: id,
                channel: ParameterChannel::Left,
                value: 0.0,
            },
        ];
        assert!(matches!(
            prepare_response_preview(request(&duplicate)).err(),
            Some(ResponsePreviewError::DuplicateParameter)
        ));
        assert!(matches!(
            prepare_response_preview(request(&[ResponseParameterOverride {
                parameter_id: id,
                channel: ParameterChannel::Both,
                value: 1.0,
            }]))
            .err(),
            Some(ResponsePreviewError::ConflictingChannel)
        ));
    }

    #[test]
    fn mixed_prepared_track_composes_real_filter_eq_and_excludes_compressor() {
        let caps = HostPrepareCaps {
            shape: HostShapePolicy::AnyLaunchRate,
            source_ring_frames: 256,
            maximum_source_channels: None,
            maximum_automation_spans_per_block: 4096,
            maximum_tracks: u64::MAX,
            maximum_sources: u64::MAX,
            maximum_routes: u64::MAX,
            maximum_effects: u64::MAX,
            maximum_graph_session_plus_plan_bytes: u64::MAX,
            maximum_source_total_bytes: u64::MAX,
            maximum_source_overhead_bytes: u64::MAX,
            maximum_effect_state_bytes: u64::MAX,
            maximum_effect_scratch_bytes: u64::MAX,
            maximum_builtin_retained_bytes: u64::MAX,
            maximum_named_allocation_bytes: u64::MAX,
            maximum_meter_streams: u64::MAX,
            maximum_meter_items: u64::MAX,
            maximum_meter_bytes: u64::MAX,
        };
        let mut model = parse_host_session(include_str!(
            "../../../fixtures/session/v1/parametric-eq-nine-track.json"
        ))
        .expect("EQ fixture");
        let compressor = parse_host_session(include_str!(
            "../../../fixtures/session/v1/compressor-dynamic-observation.json"
        ))
        .expect("compressor fixture");
        model.tracks[0].dynamic = compressor.tracks[0].dynamic.clone();
        let compiled = compile_host_model(
            &model,
            caps.compile_caps(model.sources.len())
                .expect("compile caps"),
        )
        .expect("mixed prepared session");
        let mut host = prepare_host_runtime(&compiled, &caps).expect("prepared host");
        let mut collector = ResponseSnapshotCollector::new(48_000, 8, 4, 32);
        let before_capture = bench_support::alloc::current_thread_counters();
        let capture = host
            .copy_response_snapshot("eq0", &mut collector)
            .expect("mixed track capture");
        let capture_delta = bench_support::alloc::current_thread_delta_since(before_capture);
        assert_eq!(capture_delta.allocations, 0, "snapshot capture allocated");
        assert_eq!(
            capture_delta.reallocations, 0,
            "snapshot capture reallocated"
        );
        assert_eq!(capture_delta.deallocations, 0, "snapshot capture freed");
        assert_eq!(capture.captured_sample, 0);
        let snapshot = collector.finish(capture);
        assert_eq!(snapshot.owners.len(), 3);
        assert_eq!(
            snapshot
                .owners
                .iter()
                .map(|owner| owner.stable_id.as_ref())
                .collect::<Vec<_>>(),
            vec!["input-filters", "eq", "comp"]
        );
        assert_eq!(
            snapshot.owners[2].availability,
            ResponseSnapshotAvailability::DeclaredUnavailable
        );
        assert!(snapshot.owners[2].left.is_empty());

        let mut frequencies = [0.0; 5];
        let mut left = [0.0; 5];
        let mut right = [0.0; 5];
        let summary = query_response_snapshot_into(
            &snapshot,
            ResponsePreviewGrid::Linear {
                points: 5,
                minimum_hz: 20.0,
                maximum_hz: 20_000.0,
            },
            ResponseSnapshotOutput {
                frequencies_hz: &mut frequencies,
                total_left_db: &mut left,
                total_right_db: &mut right,
            },
        )
        .expect("composed mixed response");
        assert_eq!(summary.owners, 3);
        assert_eq!(summary.excluded_owners, 1);
        assert!(
            left.iter()
                .chain(right.iter())
                .all(|value| value.is_finite())
        );
        assert!(left.iter().any(|value| value.abs() > 0.01));
    }
}
