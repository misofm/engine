use std::sync::Arc;

use effect_contract::{
    EffectId, InitialParameterValue, NativeEffectFactory, PrepareEffectLimits, StatePayloadInput,
    StatePayloadOutput,
};
use effect_package::{
    BoundEffectDescriptorWire, BoundEffectStateMigrationEdge, EFFECT_STATE_UNAVAILABLE_INDEX,
    EFFECT_STATE_UNAVAILABLE_OFFSET, EffectStateDescriptorProvenance, EffectStateDiagnostic,
    EffectStateDiagnosticCode, EffectStateLimits, EffectStateMigrationEdgeError,
    EffectStateSelector, bind_effect_state_migration_edge, effect_state_bound_selector,
    effect_state_derived_resources, effect_state_descriptor_provenance,
    effect_state_expected_metadata, effect_state_replay_view_from_verified,
    effect_state_requirements, encode_effect_state, inspect_effect_state_selector,
    validate_effect_state_current_layout, validate_effect_state_replay_configuration,
    verify_effect_state,
};

use crate::prepare::admit_restored_state;
use crate::{
    EffectBankPreparation, EffectStateRestoreAdmission, RestoredScalarEffectState,
    UnpublishedEffectBankState, WireBoundNativeEffectFactory, restore_scalar_effect_state,
    restore_unpublished_effect_bank_track_state,
};

pub const EFFECT_STATE_MIGRATION_UNAVAILABLE_INDEX: u32 = u32::MAX;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum EffectStateMigrationDiagnosticCode {
    Ok = 0,
    Limit = 1,
    BufferTooSmall = 2,
    Registry = 3,
    Chain = 4,
    Step = 5,
    State = 6,
    Restore = 7,
    Overflow = 8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct EffectStateMigrationDiagnostic {
    pub code: EffectStateMigrationDiagnosticCode,
    pub detail: u32,
    pub item_index: u32,
    pub reserved: u32,
    pub required_bytes: u64,
    pub nested_state: EffectStateDiagnostic,
}

impl EffectStateMigrationDiagnostic {
    pub const fn ok() -> Self {
        Self {
            code: EffectStateMigrationDiagnosticCode::Ok,
            detail: 0,
            item_index: EFFECT_STATE_MIGRATION_UNAVAILABLE_INDEX,
            reserved: 0,
            required_bytes: 0,
            nested_state: canonical_nested_ok(),
        }
    }
}

const fn canonical_nested_ok() -> EffectStateDiagnostic {
    EffectStateDiagnostic::new(
        EffectStateDiagnosticCode::Ok,
        0,
        EFFECT_STATE_UNAVAILABLE_INDEX,
        EFFECT_STATE_UNAVAILABLE_OFFSET,
    )
}

fn migration_diagnostic(
    code: EffectStateMigrationDiagnosticCode,
    detail: u32,
    item_index: Option<usize>,
    required_bytes: u64,
) -> EffectStateMigrationDiagnostic {
    EffectStateMigrationDiagnostic {
        code,
        detail,
        item_index: item_index
            .and_then(|index| u32::try_from(index).ok())
            .unwrap_or(EFFECT_STATE_MIGRATION_UNAVAILABLE_INDEX),
        reserved: 0,
        required_bytes,
        nested_state: canonical_nested_ok(),
    }
}

fn state_diagnostic(
    detail: u32,
    item_index: Option<usize>,
    nested_state: EffectStateDiagnostic,
) -> EffectStateMigrationDiagnostic {
    EffectStateMigrationDiagnostic {
        code: EffectStateMigrationDiagnosticCode::State,
        detail,
        item_index: item_index
            .and_then(|index| u32::try_from(index).ok())
            .unwrap_or(EFFECT_STATE_MIGRATION_UNAVAILABLE_INDEX),
        reserved: 0,
        required_bytes: 0,
        nested_state,
    }
}

fn state_limit(byte_offset: u64, required_bytes: u64) -> EffectStateDiagnostic {
    let mut diagnostic = EffectStateDiagnostic::new(
        EffectStateDiagnosticCode::Limit,
        0,
        EFFECT_STATE_UNAVAILABLE_INDEX,
        byte_offset,
    );
    diagnostic.required_bytes = required_bytes;
    diagnostic
}

fn host_fit(value: u64) -> bool {
    usize::try_from(value)
        .ok()
        .is_some_and(|value| value <= isize::MAX as usize)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct EffectStateMigrationStepReport {
    pub common_bytes: u32,
    pub left_bytes: u32,
    pub right_bytes: u32,
    pub reserved: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum EffectStateMigrationStepFailure {
    Rejected = 1,
}

pub trait EffectStateMigrationStep: Send + Sync {
    fn scratch_bytes(&self) -> u64;

    fn migrate(
        &self,
        source_layout: u32,
        target_layout: u32,
        source: StatePayloadInput<'_>,
        target: StatePayloadOutput<'_>,
        algorithm_scratch: &mut [u8],
    ) -> Result<EffectStateMigrationStepReport, EffectStateMigrationStepFailure>;
}

pub struct EffectStateMigrationRegistration<'wire> {
    edge: Result<BoundEffectStateMigrationEdge<'wire>, EffectStateMigrationEdgeError>,
    step: Arc<dyn EffectStateMigrationStep>,
}

impl core::fmt::Debug for EffectStateMigrationRegistration<'_> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("EffectStateMigrationRegistration")
            .field("edge", &self.edge)
            .finish_non_exhaustive()
    }
}

impl<'wire> EffectStateMigrationRegistration<'wire> {
    pub fn new(
        edge: BoundEffectStateMigrationEdge<'wire>,
        step: Arc<dyn EffectStateMigrationStep>,
    ) -> Self {
        Self {
            edge: Ok(edge),
            step,
        }
    }

    pub fn from_bound_descriptors(
        source: BoundEffectDescriptorWire<'wire>,
        target: BoundEffectDescriptorWire<'wire>,
        step: Arc<dyn EffectStateMigrationStep>,
    ) -> Self {
        let edge = bind_effect_state_migration_edge(source, target);
        Self { edge, step }
    }

    pub fn step(&self) -> &Arc<dyn EffectStateMigrationStep> {
        &self.step
    }
}

fn registry_edge_error(
    error: EffectStateMigrationEdgeError,
    item_index: usize,
) -> EffectStateMigrationDiagnostic {
    migration_diagnostic(
        EffectStateMigrationDiagnosticCode::Registry,
        error as u32,
        Some(item_index),
        0,
    )
}

pub fn bind_effect_state_migration_registration<'wire>(
    source: BoundEffectDescriptorWire<'wire>,
    target: BoundEffectDescriptorWire<'wire>,
    step: Arc<dyn EffectStateMigrationStep>,
) -> EffectStateMigrationRegistration<'wire> {
    EffectStateMigrationRegistration::from_bound_descriptors(source, target, step)
}

struct RegisteredEffectStateMigration<'wire> {
    edge: BoundEffectStateMigrationEdge<'wire>,
    step: Arc<dyn EffectStateMigrationStep>,
    scratch_bytes: u64,
}

impl core::fmt::Debug for RegisteredEffectStateMigration<'_> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("RegisteredEffectStateMigration")
            .field("source", &self.edge.source_selector())
            .field("target", &self.edge.target_selector())
            .field("scratch_bytes", &self.scratch_bytes)
            .field("step", &Arc::as_ptr(&self.step))
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
pub struct StateMigrationRegistry<'wire> {
    maximum_entries: u32,
    registrations: Box<[RegisteredEffectStateMigration<'wire>]>,
}

impl<'wire> StateMigrationRegistry<'wire> {
    pub fn new(
        maximum_entries: u32,
        registrations: Box<[EffectStateMigrationRegistration<'wire>]>,
    ) -> Result<Self, EffectStateMigrationDiagnostic> {
        if !host_fit(u64::from(maximum_entries)) {
            return Err(migration_diagnostic(
                EffectStateMigrationDiagnosticCode::Overflow,
                1,
                None,
                u64::from(maximum_entries),
            ));
        }
        let maximum_entries_usize = usize::try_from(maximum_entries).map_err(|_| {
            migration_diagnostic(
                EffectStateMigrationDiagnosticCode::Overflow,
                1,
                None,
                u64::from(maximum_entries),
            )
        })?;
        if registrations.len() > maximum_entries_usize {
            let required_bytes = u64::try_from(registrations.len()).map_err(|_| {
                migration_diagnostic(
                    EffectStateMigrationDiagnosticCode::Overflow,
                    1,
                    None,
                    u64::MAX,
                )
            })?;
            return Err(migration_diagnostic(
                EffectStateMigrationDiagnosticCode::Limit,
                1,
                None,
                required_bytes,
            ));
        }
        let mut accepted = Vec::with_capacity(registrations.len());
        for (index, registration) in registrations.into_vec().into_iter().enumerate() {
            let edge = registration
                .edge
                .map_err(|error| registry_edge_error(error, index))?;
            let scratch_bytes = registration.step.scratch_bytes();
            if !host_fit(scratch_bytes) {
                return Err(migration_diagnostic(
                    EffectStateMigrationDiagnosticCode::Overflow,
                    1,
                    Some(index),
                    scratch_bytes,
                ));
            }
            if accepted
                .iter()
                .any(|prior: &RegisteredEffectStateMigration<'wire>| {
                    prior.edge.source_selector() == edge.source_selector()
                })
            {
                return Err(migration_diagnostic(
                    EffectStateMigrationDiagnosticCode::Registry,
                    5,
                    Some(index),
                    0,
                ));
            }
            accepted.push(RegisteredEffectStateMigration {
                edge,
                step: registration.step,
                scratch_bytes,
            });
        }
        Ok(Self {
            maximum_entries,
            registrations: accepted.into_boxed_slice(),
        })
    }

    pub const fn maximum_entries(&self) -> u32 {
        self.maximum_entries
    }

    fn find(
        &self,
        selector: EffectStateSelector,
    ) -> Option<&RegisteredEffectStateMigration<'wire>> {
        self.registrations
            .iter()
            .find(|registration| registration.edge.source_selector() == selector)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectStateMigrationAdmission {
    pub maximum_chain_steps: u32,
    pub maximum_intermediate_envelope_bytes: u64,
    pub maximum_migration_scratch_bytes: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectStateMigrationWorkspaceRequirements {
    pub chain_step_count: u32,
    pub first_envelope_bytes: u64,
    pub second_envelope_bytes: u64,
    pub migration_scratch_bytes: u64,
    pub scalar_initial_value_scratch_slots: u32,
    pub scalar_initial_value_scratch_bytes: u64,
}

pub struct ResolvedEffectStateMigration<'registry, 'wire, 'factory_wire, 'state> {
    registrations: Box<[&'registry RegisteredEffectStateMigration<'wire>]>,
    source_envelope: &'state [u8],
    current_bound: BoundEffectDescriptorWire<'factory_wire>,
    current_effect_id: EffectId,
    current_selector: EffectStateSelector,
    current_provenance: EffectStateDescriptorProvenance,
    factory: Arc<dyn NativeEffectFactory>,
    replay: EffectBankPreparation,
    requirements: EffectStateMigrationWorkspaceRequirements,
    state_limits: EffectStateLimits,
    migration_admission: EffectStateMigrationAdmission,
    restore_admission: EffectStateRestoreAdmission,
}

impl core::fmt::Debug for ResolvedEffectStateMigration<'_, '_, '_, '_> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("ResolvedEffectStateMigration")
            .field("requirements", &self.requirements)
            .field("replay", &self.replay)
            .field("source_bytes", &self.source_envelope.len())
            .field("current_selector", &self.current_selector)
            .field("current_effect_id", &self.current_effect_id)
            .field(
                "current_bound_selector",
                &effect_state_bound_selector(self.current_bound),
            )
            .field("current_provenance", &self.current_provenance)
            .field("factory", &Arc::as_ptr(&self.factory))
            .field("state_limits", &self.state_limits)
            .field("migration_admission", &self.migration_admission)
            .field("restore_admission", &self.restore_admission)
            .finish_non_exhaustive()
    }
}

impl<'registry, 'wire, 'factory_wire, 'state>
    ResolvedEffectStateMigration<'registry, 'wire, 'factory_wire, 'state>
{
    pub const fn requirements(&self) -> EffectStateMigrationWorkspaceRequirements {
        self.requirements
    }

    pub fn replay(&self) -> &EffectBankPreparation {
        &self.replay
    }

    pub fn chain_step_count(&self) -> usize {
        self.registrations.len()
    }
}

fn replay_is_bit_exact(left: &EffectBankPreparation, right: &EffectBankPreparation) -> bool {
    left.sample_rate == right.sample_rate
        && left.quantum == right.quantum
        && left.quality == right.quality
        && left.bypass == right.bypass
        && left.link_mode == right.link_mode
        && left.ports == right.ports
        && left.limits == right.limits
        && left.initial_values.len() == right.initial_values.len()
        && left
            .initial_values
            .iter()
            .zip(right.initial_values.iter())
            .all(|(left, right)| {
                left.parameter_index == right.parameter_index
                    && left.channel == right.channel
                    && left.value.to_bits() == right.value.to_bits()
            })
}

fn buffer_diagnostic(detail: u32, required_bytes: u64) -> EffectStateMigrationDiagnostic {
    migration_diagnostic(
        EffectStateMigrationDiagnosticCode::BufferTooSmall,
        detail,
        None,
        required_bytes,
    )
}

fn step_diagnostic(detail: u32, index: usize) -> EffectStateMigrationDiagnostic {
    migration_diagnostic(
        EffectStateMigrationDiagnosticCode::Step,
        detail,
        Some(index),
        0,
    )
}

fn restore_diagnostic(
    detail: u32,
    nested_state: EffectStateDiagnostic,
) -> EffectStateMigrationDiagnostic {
    EffectStateMigrationDiagnostic {
        code: EffectStateMigrationDiagnosticCode::Restore,
        detail,
        item_index: EFFECT_STATE_MIGRATION_UNAVAILABLE_INDEX,
        reserved: 0,
        required_bytes: 0,
        nested_state,
    }
}

fn nested_restore(detail: u32) -> EffectStateDiagnostic {
    EffectStateDiagnostic::new(
        EffectStateDiagnosticCode::Restore,
        detail,
        EFFECT_STATE_UNAVAILABLE_INDEX,
        EFFECT_STATE_UNAVAILABLE_OFFSET,
    )
}

#[derive(Clone, Copy)]
enum FinalEnvelopeLocation {
    Source,
    First(usize),
    Second(usize),
}

fn preflight_workspace(
    requirements: EffectStateMigrationWorkspaceRequirements,
    first_envelope: &[u8],
    second_envelope: &[u8],
    migration_scratch: &[u8],
) -> Result<(usize, usize, usize), EffectStateMigrationDiagnostic> {
    let first = usize::try_from(requirements.first_envelope_bytes).map_err(|_| {
        migration_diagnostic(
            EffectStateMigrationDiagnosticCode::Overflow,
            2,
            None,
            requirements.first_envelope_bytes,
        )
    })?;
    let second = usize::try_from(requirements.second_envelope_bytes).map_err(|_| {
        migration_diagnostic(
            EffectStateMigrationDiagnosticCode::Overflow,
            2,
            None,
            requirements.second_envelope_bytes,
        )
    })?;
    let scratch = usize::try_from(requirements.migration_scratch_bytes).map_err(|_| {
        migration_diagnostic(
            EffectStateMigrationDiagnosticCode::Overflow,
            2,
            None,
            requirements.migration_scratch_bytes,
        )
    })?;
    if first_envelope.len() < first {
        return Err(buffer_diagnostic(1, requirements.first_envelope_bytes));
    }
    if second_envelope.len() < second {
        return Err(buffer_diagnostic(2, requirements.second_envelope_bytes));
    }
    if migration_scratch.len() < scratch {
        return Err(buffer_diagnostic(3, requirements.migration_scratch_bytes));
    }
    Ok((first, second, scratch))
}

fn validate_scalar_terminal(
    resolved: &ResolvedEffectStateMigration<'_, '_, '_, '_>,
    capability: &WireBoundNativeEffectFactory<'_>,
) -> Result<(), EffectStateMigrationDiagnostic> {
    let bound = capability.bound_descriptor();
    if effect_state_bound_selector(bound) != resolved.current_selector
        || effect_state_descriptor_provenance(bound) != resolved.current_provenance
        || !Arc::ptr_eq(capability.factory(), &resolved.factory)
        || !core::ptr::eq(capability.factory().descriptor(), capability.descriptor())
    {
        return Err(migration_diagnostic(
            EffectStateMigrationDiagnosticCode::Chain,
            3,
            None,
            0,
        ));
    }
    Ok(())
}

fn validate_bank_terminal(
    resolved: &ResolvedEffectStateMigration<'_, '_, '_, '_>,
    capability: &UnpublishedEffectBankState<'_>,
    track_index: u32,
) -> Result<(), EffectStateMigrationDiagnostic> {
    let Some(replay) = capability.replays().get(track_index as usize) else {
        return Err(restore_diagnostic(2, nested_restore(1)));
    };
    let live_metadata = capability.bank().metadata();
    if capability.replays().len() != capability.width().lanes() as usize
        || !capability.width().matches_backend(capability.backend())
        || !replay_is_bit_exact(replay, &resolved.replay)
        || capability.metadata().width != capability.width()
        || live_metadata.width != capability.width()
    {
        return Err(restore_diagnostic(2, nested_restore(2)));
    }
    let expected = effect_state_expected_metadata(
        resolved.current_bound,
        resolved.replay.state_replay(resolved.current_effect_id),
    )
    .map_err(|diagnostic| state_diagnostic(3, None, diagnostic))?;
    if capability.metadata().program_key != expected.program_key()
        || live_metadata.program_key != expected.program_key()
    {
        return Err(restore_diagnostic(2, nested_restore(3)));
    }
    let bound = capability.bound_factory().bound_descriptor();
    if effect_state_bound_selector(bound) != resolved.current_selector
        || effect_state_descriptor_provenance(bound) != resolved.current_provenance
        || !Arc::ptr_eq(capability.bound_factory().factory(), &resolved.factory)
        || !core::ptr::eq(
            capability.bound_factory().factory().descriptor(),
            capability.bound_factory().descriptor(),
        )
    {
        return Err(restore_diagnostic(2, nested_restore(4)));
    }
    Ok(())
}

fn execute_migration_steps(
    resolved: &ResolvedEffectStateMigration<'_, '_, '_, '_>,
    first_envelope: &mut [u8],
    second_envelope: &mut [u8],
    migration_scratch: &mut [u8],
) -> Result<FinalEnvelopeLocation, EffectStateMigrationDiagnostic> {
    let mut location = FinalEnvelopeLocation::Source;
    for (index, registration) in resolved.registrations.iter().enumerate() {
        let source_bytes: &[u8] = match location {
            FinalEnvelopeLocation::Source => resolved.source_envelope,
            FinalEnvelopeLocation::First(bytes) => &first_envelope[..bytes],
            FinalEnvelopeLocation::Second(bytes) => &second_envelope[..bytes],
        };
        let source = verify_effect_state(
            registration.edge.source_bound(),
            source_bytes,
            resolved.state_limits,
        )
        .map_err(|diagnostic| state_diagnostic(1, Some(index), diagnostic))?;
        validate_effect_state_current_layout(source)
            .map_err(|diagnostic| state_diagnostic(1, Some(index), diagnostic))?;
        validate_effect_state_replay_configuration(
            source,
            resolved.replay.state_replay(resolved.current_effect_id),
        )
        .map_err(|diagnostic| state_diagnostic(1, Some(index), diagnostic))?;

        let target_bound = registration.edge.target_bound();
        let replay = resolved.replay.state_replay(resolved.current_effect_id);
        let requirements = effect_state_requirements(target_bound, replay, resolved.state_limits)
            .map_err(|diagnostic| state_diagnostic(2, Some(index), diagnostic))?;
        let metadata = effect_state_expected_metadata(target_bound, replay)
            .map_err(|diagnostic| state_diagnostic(2, Some(index), diagnostic))?;
        let payload_bytes = usize::try_from(
            metadata
                .state_sizes
                .total()
                .ok_or_else(|| state_diagnostic(2, Some(index), state_limit(216, u64::MAX)))?,
        )
        .map_err(|_| state_diagnostic(2, Some(index), state_limit(216, u64::MAX)))?;
        let algorithm_bytes = usize::try_from(registration.scratch_bytes).map_err(|_| {
            migration_diagnostic(
                EffectStateMigrationDiagnosticCode::Overflow,
                2,
                Some(index),
                registration.scratch_bytes,
            )
        })?;
        let used_scratch = payload_bytes.checked_add(algorithm_bytes).ok_or_else(|| {
            migration_diagnostic(
                EffectStateMigrationDiagnosticCode::Overflow,
                2,
                Some(index),
                u64::MAX,
            )
        })?;
        let (payload, algorithm) = migration_scratch[..used_scratch].split_at_mut(payload_bytes);
        payload.fill(0xa5);
        let common_end = metadata.state_sizes.common_bytes as usize;
        let left_end = common_end + metadata.state_sizes.left_bytes as usize;
        let (common, remainder) = payload.split_at_mut(common_end);
        let (left, right) = remainder.split_at_mut(left_end - common_end);
        let target = StatePayloadOutput::new(common, left, right, metadata.state_sizes)
            .map_err(|_| step_diagnostic(3, index))?;
        let (source_common, source_left, source_right) = source.payloads();
        let source_payload = StatePayloadInput::new(
            source_common,
            source_left,
            source_right,
            source.state_sizes(),
        )
        .map_err(|_| {
            state_diagnostic(
                1,
                Some(index),
                EffectStateDiagnostic::new(
                    EffectStateDiagnosticCode::Payload,
                    3,
                    EFFECT_STATE_UNAVAILABLE_INDEX,
                    EFFECT_STATE_UNAVAILABLE_OFFSET,
                ),
            )
        })?;
        let result = registration.step.migrate(
            registration.edge.source_selector().state_layout_version(),
            registration.edge.target_selector().state_layout_version(),
            source_payload,
            target,
            algorithm,
        );
        let report = match result {
            Ok(report) => report,
            Err(_) => {
                let changed = payload.iter().any(|byte| *byte != 0xa5);
                return Err(step_diagnostic(if changed { 2 } else { 1 }, index));
            }
        };
        if report.reserved != 0
            || report.common_bytes != metadata.state_sizes.common_bytes
            || report.left_bytes != metadata.state_sizes.left_bytes
            || report.right_bytes != metadata.state_sizes.right_bytes
        {
            return Err(step_diagnostic(3, index));
        }
        let target_bytes = usize::try_from(requirements.envelope_bytes)
            .map_err(|_| state_diagnostic(2, Some(index), state_limit(16, u64::MAX)))?;
        let output = if index % 2 == 0 {
            &mut first_envelope[..target_bytes]
        } else {
            &mut second_envelope[..target_bytes]
        };
        encode_effect_state(
            target_bound,
            replay,
            common,
            left,
            right,
            resolved.state_limits,
            output,
        )
        .map_err(|diagnostic| state_diagnostic(2, Some(index), diagnostic))?;
        let target_state = verify_effect_state(target_bound, output, resolved.state_limits)
            .map_err(|diagnostic| state_diagnostic(2, Some(index), diagnostic))?;
        validate_effect_state_current_layout(target_state)
            .map_err(|diagnostic| state_diagnostic(2, Some(index), diagnostic))?;
        validate_effect_state_replay_configuration(target_state, replay)
            .map_err(|diagnostic| state_diagnostic(2, Some(index), diagnostic))?;
        location = if index % 2 == 0 {
            FinalEnvelopeLocation::First(target_bytes)
        } else {
            FinalEnvelopeLocation::Second(target_bytes)
        };
    }
    Ok(location)
}

fn final_envelope<'a>(
    location: FinalEnvelopeLocation,
    source: &'a [u8],
    first: &'a [u8],
    second: &'a [u8],
) -> &'a [u8] {
    match location {
        FinalEnvelopeLocation::Source => source,
        FinalEnvelopeLocation::First(bytes) => &first[..bytes],
        FinalEnvelopeLocation::Second(bytes) => &second[..bytes],
    }
}

fn validate_final_envelope(
    resolved: &ResolvedEffectStateMigration<'_, '_, '_, '_>,
    bound: BoundEffectDescriptorWire<'_>,
    envelope: &[u8],
) -> Result<(), EffectStateMigrationDiagnostic> {
    let state = verify_effect_state(bound, envelope, resolved.state_limits)
        .map_err(|diagnostic| state_diagnostic(3, None, diagnostic))?;
    validate_effect_state_current_layout(state)
        .map_err(|diagnostic| state_diagnostic(3, None, diagnostic))?;
    validate_effect_state_replay_configuration(
        state,
        resolved.replay.state_replay(resolved.current_effect_id),
    )
    .map_err(|diagnostic| state_diagnostic(3, None, diagnostic))
}

pub fn restore_scalar_effect_state_with_migration<'wire>(
    resolved: ResolvedEffectStateMigration<'_, '_, '_, '_>,
    capability: WireBoundNativeEffectFactory<'wire>,
    first_envelope: &mut [u8],
    second_envelope: &mut [u8],
    migration_scratch: &mut [u8],
    initial_value_scratch: &mut [InitialParameterValue],
) -> Result<RestoredScalarEffectState<'wire>, EffectStateMigrationDiagnostic> {
    let requirements = resolved.requirements;
    preflight_workspace(
        requirements,
        first_envelope,
        second_envelope,
        migration_scratch,
    )?;
    let initial_slots =
        usize::try_from(requirements.scalar_initial_value_scratch_slots).map_err(|_| {
            migration_diagnostic(
                EffectStateMigrationDiagnosticCode::Overflow,
                3,
                None,
                requirements.scalar_initial_value_scratch_bytes,
            )
        })?;
    if initial_value_scratch.len() < initial_slots {
        return Err(buffer_diagnostic(
            4,
            requirements.scalar_initial_value_scratch_bytes,
        ));
    }
    validate_scalar_terminal(&resolved, &capability)?;
    let location = execute_migration_steps(
        &resolved,
        first_envelope,
        second_envelope,
        migration_scratch,
    )?;
    let envelope = final_envelope(
        location,
        resolved.source_envelope,
        first_envelope,
        second_envelope,
    );
    validate_final_envelope(&resolved, capability.bound_descriptor(), envelope)?;
    restore_scalar_effect_state(
        capability,
        envelope,
        resolved.state_limits,
        resolved.restore_admission,
        &mut initial_value_scratch[..initial_slots],
    )
    .map_err(|diagnostic| restore_diagnostic(1, diagnostic))
}

pub fn restore_unpublished_effect_bank_track_state_with_migration<'wire>(
    resolved: ResolvedEffectStateMigration<'_, '_, '_, '_>,
    capability: UnpublishedEffectBankState<'wire>,
    track_index: u32,
    first_envelope: &mut [u8],
    second_envelope: &mut [u8],
    migration_scratch: &mut [u8],
) -> Result<UnpublishedEffectBankState<'wire>, EffectStateMigrationDiagnostic> {
    preflight_workspace(
        resolved.requirements,
        first_envelope,
        second_envelope,
        migration_scratch,
    )?;
    validate_bank_terminal(&resolved, &capability, track_index)?;
    let location = execute_migration_steps(
        &resolved,
        first_envelope,
        second_envelope,
        migration_scratch,
    )?;
    let envelope = final_envelope(
        location,
        resolved.source_envelope,
        first_envelope,
        second_envelope,
    );
    let bound = capability.bound_factory().bound_descriptor();
    validate_final_envelope(&resolved, bound, envelope)?;
    restore_unpublished_effect_bank_track_state(
        capability,
        track_index,
        envelope,
        resolved.state_limits,
        resolved.restore_admission,
    )
    .map_err(|diagnostic| restore_diagnostic(2, diagnostic))
}

fn owned_replay_from_state(
    state: effect_package::VerifiedEffectState<'_>,
) -> Result<EffectBankPreparation, EffectStateDiagnostic> {
    let initial_values: Box<[InitialParameterValue]> = state.initial_values().collect();
    let (
        sample_rate,
        quantum,
        quality,
        bypass,
        link_mode,
        ports,
        maximum_total_state_bytes,
        maximum_scratch_bytes,
        maximum_automation_spans_per_block,
    ) = {
        let replay = effect_state_replay_view_from_verified(state, &initial_values)?;
        validate_effect_state_replay_configuration(state, replay)?;
        (
            replay.request.sample_rate,
            replay.request.quantum,
            replay.request.quality,
            replay.request.bypass,
            replay.request.link_mode,
            replay.request.ports,
            replay.request.limits.maximum_total_state_bytes,
            replay.request.limits.maximum_scratch_bytes,
            replay.request.limits.maximum_automation_spans_per_block,
        )
    };
    Ok(EffectBankPreparation {
        sample_rate,
        quantum,
        quality,
        bypass,
        link_mode,
        ports,
        initial_values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes,
            maximum_scratch_bytes,
            maximum_automation_spans_per_block,
        },
    })
}

fn admit_current_replay(
    bound: BoundEffectDescriptorWire<'_>,
    replay: &EffectBankPreparation,
    admission: EffectStateRestoreAdmission,
) -> Result<(), EffectStateDiagnostic> {
    if !engine::is_launch_sample_rate(engine::SampleRateHz(admission.sample_rate))
        || replay.sample_rate != admission.sample_rate
    {
        return Err(state_limit(96, u64::from(replay.sample_rate)));
    }
    if admission.quantum == 0 || replay.quantum != admission.quantum {
        return Err(state_limit(100, u64::from(replay.quantum)));
    }
    if admission.maximum_total_state_bytes == 0
        || replay.limits.maximum_total_state_bytes > admission.maximum_total_state_bytes
    {
        return Err(state_limit(192, replay.limits.maximum_total_state_bytes));
    }
    if admission.maximum_scratch_bytes == 0
        || replay.limits.maximum_scratch_bytes > admission.maximum_scratch_bytes
    {
        return Err(state_limit(200, replay.limits.maximum_scratch_bytes));
    }
    if admission.maximum_automation_spans_per_block == 0
        || replay.limits.maximum_automation_spans_per_block
            > admission.maximum_automation_spans_per_block
    {
        return Err(state_limit(
            208,
            u64::from(replay.limits.maximum_automation_spans_per_block),
        ));
    }
    let resources = effect_state_derived_resources(bound, replay.request())?;
    let payload_bytes = resources.state_sizes.total().ok_or_else(|| {
        EffectStateDiagnostic::new(
            EffectStateDiagnosticCode::Overflow,
            0,
            EFFECT_STATE_UNAVAILABLE_INDEX,
            216,
        )
    })?;
    if payload_bytes > admission.maximum_total_state_bytes {
        return Err(state_limit(216, payload_bytes));
    }
    if resources.scratch_bytes > admission.maximum_scratch_bytes {
        return Err(state_limit(176, resources.scratch_bytes));
    }
    if resources.automation_capacity > admission.maximum_automation_spans_per_block {
        return Err(state_limit(184, u64::from(resources.automation_capacity)));
    }
    Ok(())
}

fn validate_migration_admission(
    admission: EffectStateMigrationAdmission,
) -> Result<(), EffectStateMigrationDiagnostic> {
    for value in [
        u64::from(admission.maximum_chain_steps),
        admission.maximum_intermediate_envelope_bytes,
        admission.maximum_migration_scratch_bytes,
    ] {
        if !host_fit(value) {
            return Err(migration_diagnostic(
                EffectStateMigrationDiagnosticCode::Overflow,
                2,
                None,
                value,
            ));
        }
    }
    Ok(())
}

pub fn resolve_effect_state_migration<'registry, 'wire, 'factory_wire, 'state>(
    registry: &'registry StateMigrationRegistry<'wire>,
    current_factory: &WireBoundNativeEffectFactory<'factory_wire>,
    envelope: &'state [u8],
    state_limits: EffectStateLimits,
    migration_admission: EffectStateMigrationAdmission,
    restore_admission: EffectStateRestoreAdmission,
) -> Result<
    ResolvedEffectStateMigration<'registry, 'wire, 'factory_wire, 'state>,
    EffectStateMigrationDiagnostic,
> {
    validate_migration_admission(migration_admission)?;
    let source_selector = inspect_effect_state_selector(envelope, state_limits)
        .map_err(|diagnostic| state_diagnostic(1, None, diagnostic))?;
    let current_bound = current_factory.bound_descriptor();
    let current_effect_id = current_factory.descriptor().id;
    let current_selector = effect_state_bound_selector(current_bound);
    if source_selector.state_layout_version() > current_selector.state_layout_version()
        || (source_selector.state_layout_version() == current_selector.state_layout_version()
            && source_selector != current_selector)
    {
        return Err(migration_diagnostic(
            EffectStateMigrationDiagnosticCode::Chain,
            2,
            None,
            u64::from(source_selector.state_layout_version()),
        ));
    }

    let zero_step = source_selector == current_selector;
    let first_registration = if zero_step {
        None
    } else {
        Some(registry.find(source_selector).ok_or_else(|| {
            migration_diagnostic(EffectStateMigrationDiagnosticCode::Chain, 1, Some(0), 0)
        })?)
    };
    let source_bound = first_registration
        .map(|registration| registration.edge.source_bound())
        .unwrap_or(current_bound);
    let source_index = if zero_step { None } else { Some(0) };
    let source_state = verify_effect_state(source_bound, envelope, state_limits)
        .map_err(|diagnostic| state_diagnostic(1, source_index, diagnostic))?;
    if zero_step {
        admit_restored_state(source_state, restore_admission)
            .map_err(|diagnostic| state_diagnostic(3, None, diagnostic))?;
    }
    let source_detail = if zero_step { 3 } else { 1 };
    validate_effect_state_current_layout(source_state)
        .map_err(|diagnostic| state_diagnostic(source_detail, source_index, diagnostic))?;
    let replay = owned_replay_from_state(source_state)
        .map_err(|diagnostic| state_diagnostic(source_detail, source_index, diagnostic))?;

    let maximum_chain_steps =
        usize::try_from(migration_admission.maximum_chain_steps).map_err(|_| {
            migration_diagnostic(
                EffectStateMigrationDiagnosticCode::Overflow,
                2,
                None,
                u64::from(migration_admission.maximum_chain_steps),
            )
        })?;
    let mut registrations =
        Vec::with_capacity(maximum_chain_steps.min(registry.registrations.len()));
    let mut cursor = source_selector;
    let mut first_envelope_bytes = 0u64;
    let mut second_envelope_bytes = 0u64;
    let mut migration_scratch_bytes = 0u64;
    let mut final_requirements = if zero_step {
        Some(
            effect_state_requirements(
                current_bound,
                replay.state_replay(current_factory.descriptor().id),
                state_limits,
            )
            .map_err(|diagnostic| state_diagnostic(3, None, diagnostic))?,
        )
    } else {
        None
    };

    while cursor != current_selector {
        let index = registrations.len();
        if index >= maximum_chain_steps {
            let required_bytes = u64::try_from(index)
                .ok()
                .and_then(|value| value.checked_add(1))
                .ok_or_else(|| {
                    migration_diagnostic(
                        EffectStateMigrationDiagnosticCode::Overflow,
                        2,
                        Some(index),
                        u64::MAX,
                    )
                })?;
            return Err(migration_diagnostic(
                EffectStateMigrationDiagnosticCode::Limit,
                2,
                Some(index),
                required_bytes,
            ));
        }
        let registration = if index == 0 {
            first_registration.expect("nonzero chain has first registration")
        } else {
            registry.find(cursor).ok_or_else(|| {
                migration_diagnostic(EffectStateMigrationDiagnosticCode::Chain, 1, Some(index), 0)
            })?
        };
        let requirements = effect_state_requirements(
            registration.edge.target_bound(),
            replay.state_replay(current_factory.descriptor().id),
            state_limits,
        )
        .map_err(|diagnostic| state_diagnostic(2, Some(index), diagnostic))?;
        if requirements.envelope_bytes > migration_admission.maximum_intermediate_envelope_bytes {
            return Err(migration_diagnostic(
                EffectStateMigrationDiagnosticCode::Limit,
                3,
                Some(index),
                requirements.envelope_bytes,
            ));
        }
        let scratch = requirements
            .payload_snapshot_scratch_bytes
            .checked_add(registration.scratch_bytes)
            .ok_or_else(|| {
                migration_diagnostic(
                    EffectStateMigrationDiagnosticCode::Overflow,
                    2,
                    Some(index),
                    u64::MAX,
                )
            })?;
        if !host_fit(scratch) {
            return Err(migration_diagnostic(
                EffectStateMigrationDiagnosticCode::Overflow,
                2,
                Some(index),
                scratch,
            ));
        }
        if scratch > migration_admission.maximum_migration_scratch_bytes {
            return Err(migration_diagnostic(
                EffectStateMigrationDiagnosticCode::Limit,
                4,
                Some(index),
                scratch,
            ));
        }
        if index % 2 == 0 {
            first_envelope_bytes = first_envelope_bytes.max(requirements.envelope_bytes);
        } else {
            second_envelope_bytes = second_envelope_bytes.max(requirements.envelope_bytes);
        }
        migration_scratch_bytes = migration_scratch_bytes.max(scratch);
        registrations.push(registration);
        cursor = registration.edge.target_selector();
        final_requirements = Some(requirements);
        if cursor.state_layout_version() == current_selector.state_layout_version()
            && cursor != current_selector
        {
            return Err(migration_diagnostic(
                EffectStateMigrationDiagnosticCode::Chain,
                3,
                Some(index),
                0,
            ));
        }
        if cursor.state_layout_version() > current_selector.state_layout_version() {
            return Err(migration_diagnostic(
                EffectStateMigrationDiagnosticCode::Chain,
                2,
                Some(index),
                u64::from(cursor.state_layout_version()),
            ));
        }
    }

    if let Some(last) = registrations.last()
        && effect_state_descriptor_provenance(last.edge.target_bound())
            != effect_state_descriptor_provenance(current_bound)
    {
        return Err(migration_diagnostic(
            EffectStateMigrationDiagnosticCode::Chain,
            3,
            Some(registrations.len() - 1),
            0,
        ));
    }
    if !zero_step {
        admit_current_replay(current_bound, &replay, restore_admission)
            .map_err(|diagnostic| state_diagnostic(3, None, diagnostic))?;
    }
    let final_requirements = final_requirements.expect("current requirements exist");
    let initial_value_bytes = u64::try_from(core::mem::size_of::<InitialParameterValue>())
        .map_err(|_| {
            migration_diagnostic(
                EffectStateMigrationDiagnosticCode::Overflow,
                3,
                None,
                u64::MAX,
            )
        })?;
    let scalar_initial_value_scratch_bytes =
        u64::from(final_requirements.initial_value_scratch_slots)
            .checked_mul(initial_value_bytes)
            .ok_or_else(|| {
                migration_diagnostic(
                    EffectStateMigrationDiagnosticCode::Overflow,
                    3,
                    None,
                    u64::MAX,
                )
            })?;
    if !host_fit(scalar_initial_value_scratch_bytes) {
        return Err(migration_diagnostic(
            EffectStateMigrationDiagnosticCode::Overflow,
            3,
            None,
            scalar_initial_value_scratch_bytes,
        ));
    }
    let requirements = EffectStateMigrationWorkspaceRequirements {
        chain_step_count: u32::try_from(registrations.len()).map_err(|_| {
            migration_diagnostic(
                EffectStateMigrationDiagnosticCode::Overflow,
                2,
                None,
                u64::try_from(registrations.len()).unwrap_or(u64::MAX),
            )
        })?,
        first_envelope_bytes,
        second_envelope_bytes,
        migration_scratch_bytes,
        scalar_initial_value_scratch_slots: final_requirements.initial_value_scratch_slots,
        scalar_initial_value_scratch_bytes,
    };
    Ok(ResolvedEffectStateMigration {
        registrations: registrations.into_boxed_slice(),
        source_envelope: envelope,
        current_bound,
        current_effect_id,
        current_selector,
        current_provenance: effect_state_descriptor_provenance(current_bound),
        factory: Arc::clone(current_factory.factory()),
        replay,
        requirements,
        state_limits,
        migration_admission,
        restore_admission,
    })
}
