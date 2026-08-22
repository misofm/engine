use miso_engine_effect_contract::{
    BankWidth, EffectDescriptorV1, EffectProgramKeyV1, EffectQuality, InitialParameterValue,
    LinkMode, NativeEffectFactory, NativeEffectRegistry, ParameterChannel, ParameterChannelPolicy,
    ParameterUnit, PrepareEffectBankRequest, PrepareEffectLimits, PrepareEffectRequest,
    PreparedBankMetadata, PreparedEffectMetadata, PreparedNativeEffect, PreparedNativeEffectBank,
    PreparedPortsV1, PreparedSidechainPort, RegistryError, StatePayloadInput, StatePayloadOutput,
    expected_prepared_metadata,
};
use miso_engine_effect_package::{
    BoundEffectDescriptorWireV1, EFFECT_STATE_V1_BUFFER_ENVELOPE_OUTPUT,
    EFFECT_STATE_V1_BUFFER_INITIAL_VALUE_SCRATCH, EFFECT_STATE_V1_BUFFER_PAYLOAD_SCRATCH,
    EFFECT_STATE_V1_UNAVAILABLE_INDEX, EFFECT_STATE_V1_UNAVAILABLE_OFFSET,
    EffectDescriptorBindingErrorKindV1, EffectStateDerivedResourcesV1, EffectStateDiagnosticCodeV1,
    EffectStateDiagnosticV1, EffectStateLimitsV1, EffectStateReplayViewV1,
    EffectStateRequirementsV1, VerifiedEffectStateV1, bind_effect_descriptor_wire_v1,
    effect_state_derived_resources_v1, effect_state_expected_metadata_v1,
    effect_state_v1_requirements, encode_effect_state_v1, validate_effect_state_current_layout_v1,
    validate_effect_state_metadata_v1, validate_effect_state_replay_v1, verify_effect_state_v1,
};
use miso_engine_session::{
    CompiledSession, EffectIdentity, LinkMode as SessionLinkMode,
    ParameterChannel as SessionChannel, ParameterUnit as SessionUnit, SidechainDeclaration,
};
use std::sync::Arc;

use miso_engine_core::KernelBackendV1;

use crate::{EffectDiagnostic, EffectDiagnosticSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectCompileCaps {
    pub maximum_total_state_bytes: u64,
    pub maximum_scratch_bytes: u64,
    pub maximum_automation_spans_per_block: u32,
}
pub struct EffectPreparedEntry {
    pub track_id: String,
    pub rack: EffectRack,
    pub effect_id: String,
    pub processor: Box<dyn PreparedNativeEffect>,
    pub metadata: PreparedEffectMetadata,
    /// Factory retained only for transactional off-render bank binding.
    pub factory: Arc<dyn miso_engine_effect_contract::NativeEffectFactory>,
    /// Exact owned request inputs used to prepare the scalar processor.
    pub bank_preparation: EffectBankPreparationV1,
}

/// Owned replayable portion of an accepted prepare request. It never crosses into render.
#[derive(Clone, Debug)]
pub struct EffectBankPreparationV1 {
    pub sample_rate: u32,
    pub quantum: u32,
    pub quality: EffectQuality,
    pub bypass: bool,
    pub link_mode: LinkMode,
    pub ports: PreparedPortsV1,
    pub initial_values: Box<[InitialParameterValue]>,
    pub limits: PrepareEffectLimits,
}

impl EffectBankPreparationV1 {
    #[must_use]
    pub fn request(&self) -> PrepareEffectRequest<'_> {
        PrepareEffectRequest {
            sample_rate: self.sample_rate,
            quantum: self.quantum,
            quality: self.quality,
            bypass: self.bypass,
            link_mode: self.link_mode,
            ports: self.ports,
            initial_values: &self.initial_values,
            limits: self.limits,
        }
    }

    #[must_use]
    pub fn state_replay(
        &self,
        effect_id: miso_engine_effect_contract::EffectId,
    ) -> EffectStateReplayViewV1<'_> {
        EffectStateReplayViewV1 {
            effect_id,
            request: self.request(),
        }
    }
}

pub struct WireBoundNativeEffectFactoryV1<'wire> {
    factory: Arc<dyn NativeEffectFactory>,
    descriptor: &'static EffectDescriptorV1,
    bound_descriptor: BoundEffectDescriptorWireV1<'wire>,
}

impl core::fmt::Debug for WireBoundNativeEffectFactoryV1<'_> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("WireBoundNativeEffectFactoryV1")
            .field("effect_id", &self.descriptor.id)
            .field("descriptor_identity", &self.bound_descriptor.identity())
            .finish_non_exhaustive()
    }
}

impl<'wire> WireBoundNativeEffectFactoryV1<'wire> {
    #[must_use]
    pub fn factory(&self) -> &Arc<dyn NativeEffectFactory> {
        &self.factory
    }

    #[must_use]
    pub const fn descriptor(&self) -> &'static EffectDescriptorV1 {
        self.descriptor
    }

    #[must_use]
    pub const fn bound_descriptor(&self) -> BoundEffectDescriptorWireV1<'wire> {
        self.bound_descriptor
    }
}

fn descriptor_binding_diagnostic(
    error: miso_engine_effect_package::EffectDescriptorBindingErrorV1,
) -> EffectStateDiagnosticV1 {
    let nested = error.diagnostic();
    let kind = match error.kind() {
        EffectDescriptorBindingErrorKindV1::ExternalWire => 1,
        EffectDescriptorBindingErrorKindV1::StaticDescriptorMismatch => 2,
    };
    let byte_offset = if nested.byte_offset
        == miso_engine_effect_package::EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE
    {
        EFFECT_STATE_V1_UNAVAILABLE_OFFSET
    } else {
        u64::from(nested.byte_offset)
    };
    let mut diagnostic = EffectStateDiagnosticV1::new(
        EffectStateDiagnosticCodeV1::Descriptor,
        (kind << 16) | nested.code as u32,
        nested.record_index,
        byte_offset,
    );
    diagnostic.required_bytes = u64::from(nested.required_bytes);
    diagnostic
}

pub fn bind_native_effect_factory_state_v1(
    factory: Arc<dyn NativeEffectFactory>,
    descriptor_wire: &[u8],
    maximum_descriptor_bytes: u32,
) -> Result<WireBoundNativeEffectFactoryV1<'_>, EffectStateDiagnosticV1> {
    let descriptor = factory.descriptor();
    let bound_descriptor =
        bind_effect_descriptor_wire_v1(descriptor, descriptor_wire, maximum_descriptor_bytes)
            .map_err(descriptor_binding_diagnostic)?;
    Ok(WireBoundNativeEffectFactoryV1 {
        factory,
        descriptor,
        bound_descriptor,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectStateRestoreAdmissionV1 {
    pub sample_rate: u32,
    pub quantum: u32,
    pub maximum_total_state_bytes: u64,
    pub maximum_scratch_bytes: u64,
    pub maximum_automation_spans_per_block: u32,
}

pub struct RestoredScalarEffectStateV1<'wire> {
    processor: Box<dyn PreparedNativeEffect>,
    metadata: PreparedEffectMetadata,
    bound_factory: WireBoundNativeEffectFactoryV1<'wire>,
    replay: EffectBankPreparationV1,
}

impl core::fmt::Debug for RestoredScalarEffectStateV1<'_> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("RestoredScalarEffectStateV1")
            .field("metadata", &self.metadata)
            .field("replay", &self.replay)
            .finish_non_exhaustive()
    }
}

impl RestoredScalarEffectStateV1<'_> {
    #[must_use]
    pub fn processor(&self) -> &dyn PreparedNativeEffect {
        self.processor.as_ref()
    }

    #[must_use]
    pub fn processor_mut(&mut self) -> &mut dyn PreparedNativeEffect {
        self.processor.as_mut()
    }

    #[must_use]
    pub const fn metadata(&self) -> PreparedEffectMetadata {
        self.metadata
    }

    #[must_use]
    pub fn bound_factory(&self) -> &WireBoundNativeEffectFactoryV1<'_> {
        &self.bound_factory
    }

    #[must_use]
    pub fn replay(&self) -> &EffectBankPreparationV1 {
        &self.replay
    }
}

pub struct UnpublishedEffectBankStateV1<'wire> {
    bank: Box<dyn PreparedNativeEffectBank>,
    metadata: PreparedBankMetadata,
    backend: KernelBackendV1,
    width: BankWidth,
    bound_factory: WireBoundNativeEffectFactoryV1<'wire>,
    replays: Box<[EffectBankPreparationV1]>,
}

impl core::fmt::Debug for UnpublishedEffectBankStateV1<'_> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("UnpublishedEffectBankStateV1")
            .field("metadata", &self.metadata)
            .field("backend", &self.backend)
            .field("width", &self.width)
            .field("replay_count", &self.replays.len())
            .finish_non_exhaustive()
    }
}

impl UnpublishedEffectBankStateV1<'_> {
    #[must_use]
    pub fn bank(&self) -> &dyn PreparedNativeEffectBank {
        self.bank.as_ref()
    }

    #[must_use]
    pub fn bank_mut(&mut self) -> &mut dyn PreparedNativeEffectBank {
        self.bank.as_mut()
    }

    #[must_use]
    pub fn metadata(&self) -> &PreparedBankMetadata {
        &self.metadata
    }

    #[must_use]
    pub const fn backend(&self) -> KernelBackendV1 {
        self.backend
    }

    #[must_use]
    pub const fn width(&self) -> BankWidth {
        self.width
    }

    #[must_use]
    pub fn bound_factory(&self) -> &WireBoundNativeEffectFactoryV1<'_> {
        &self.bound_factory
    }

    #[must_use]
    pub fn replays(&self) -> &[EffectBankPreparationV1] {
        &self.replays
    }
}

fn state_diagnostic(
    code: EffectStateDiagnosticCodeV1,
    detail: u32,
    byte_offset: u64,
) -> EffectStateDiagnosticV1 {
    EffectStateDiagnosticV1::new(code, detail, EFFECT_STATE_V1_UNAVAILABLE_INDEX, byte_offset)
}

fn state_unavailable(code: EffectStateDiagnosticCodeV1, detail: u32) -> EffectStateDiagnosticV1 {
    state_diagnostic(code, detail, EFFECT_STATE_V1_UNAVAILABLE_OFFSET)
}

fn bank_restore_error(detail: u32) -> EffectStateDiagnosticV1 {
    state_unavailable(EffectStateDiagnosticCodeV1::Restore, detail)
}

fn admit_bank_preparation(
    replay: &EffectBankPreparationV1,
    admission: EffectStateRestoreAdmissionV1,
) -> Result<(), EffectStateDiagnosticV1> {
    if !miso_engine_core::is_launch_sample_rate(miso_engine_core::SampleRateHz(
        admission.sample_rate,
    )) || replay.sample_rate != admission.sample_rate
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
    Ok(())
}

fn admit_bank_derived_resources(
    resources: EffectStateDerivedResourcesV1,
    admission: EffectStateRestoreAdmissionV1,
) -> Result<(), EffectStateDiagnosticV1> {
    let payload_bytes = resources
        .state_sizes
        .total()
        .ok_or_else(|| state_unavailable(EffectStateDiagnosticCodeV1::Overflow, 0))?;
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

fn expected_bank_program_key(
    bound_factory: &WireBoundNativeEffectFactoryV1<'_>,
    backend: KernelBackendV1,
    width: BankWidth,
    replays: &[EffectBankPreparationV1],
    admission: EffectStateRestoreAdmissionV1,
) -> Result<EffectProgramKeyV1, EffectStateDiagnosticV1> {
    if !width.matches_backend(backend) || replays.len() != width.lanes() as usize {
        return Err(state_unavailable(EffectStateDiagnosticCodeV1::Factory, 2));
    }
    let mut program_key = None;
    for (track_index, replay) in replays.iter().enumerate() {
        admit_bank_preparation(replay, admission).map_err(|mut diagnostic| {
            diagnostic.item_index = track_index as u32;
            diagnostic
        })?;
        let replay_view = replay.state_replay(bound_factory.descriptor.id);
        let resources =
            effect_state_derived_resources_v1(bound_factory.bound_descriptor, replay.request())
                .map_err(|_| state_unavailable(EffectStateDiagnosticCodeV1::Factory, 2))?;
        admit_bank_derived_resources(resources, admission).map_err(|mut diagnostic| {
            diagnostic.item_index = track_index as u32;
            diagnostic
        })?;
        let metadata =
            effect_state_expected_metadata_v1(bound_factory.bound_descriptor, replay_view)
                .map_err(|_| state_unavailable(EffectStateDiagnosticCodeV1::Factory, 2))?;
        let candidate = metadata.program_key();
        if program_key
            .as_ref()
            .is_some_and(|expected| expected != &candidate)
        {
            return Err(state_unavailable(EffectStateDiagnosticCodeV1::Factory, 2));
        }
        program_key = Some(candidate);
    }
    program_key.ok_or_else(|| state_unavailable(EffectStateDiagnosticCodeV1::Factory, 2))
}

pub fn prepare_unpublished_effect_bank_state_v1<'wire>(
    bound_factory: WireBoundNativeEffectFactoryV1<'wire>,
    backend: KernelBackendV1,
    width: BankWidth,
    replays: Box<[EffectBankPreparationV1]>,
    admission: EffectStateRestoreAdmissionV1,
) -> Result<UnpublishedEffectBankStateV1<'wire>, EffectStateDiagnosticV1> {
    let program_key =
        expected_bank_program_key(&bound_factory, backend, width, replays.as_ref(), admission)?;
    let requests: Vec<_> = replays
        .iter()
        .map(EffectBankPreparationV1::request)
        .collect();
    let bank = bound_factory
        .factory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
        })
        .map_err(|_| state_unavailable(EffectStateDiagnosticCodeV1::Factory, 3))?
        .ok_or_else(|| state_unavailable(EffectStateDiagnosticCodeV1::Factory, 3))?;
    let metadata = bank.metadata();
    if metadata.width != width || metadata.program_key != program_key {
        return Err(state_unavailable(EffectStateDiagnosticCodeV1::Factory, 4));
    }
    if !core::ptr::eq(bound_factory.factory.descriptor(), bound_factory.descriptor) {
        return Err(state_unavailable(EffectStateDiagnosticCodeV1::Factory, 4));
    }
    Ok(UnpublishedEffectBankStateV1 {
        bank,
        metadata,
        backend,
        width,
        bound_factory,
        replays,
    })
}

fn unpublished_bank_track_replay<'a>(
    capability: &'a UnpublishedEffectBankStateV1<'_>,
    track_index: u32,
) -> Result<&'a EffectBankPreparationV1, EffectStateDiagnosticV1> {
    let replay = capability
        .replays
        .get(track_index as usize)
        .ok_or_else(|| bank_restore_error(1))?;
    if !capability.width.matches_backend(capability.backend)
        || capability.replays.len() != capability.width.lanes() as usize
    {
        return Err(bank_restore_error(2));
    }
    if capability.metadata.width != capability.width {
        return Err(bank_restore_error(2));
    }
    Ok(replay)
}

fn validate_unpublished_bank_program_and_provenance(
    capability: &UnpublishedEffectBankStateV1<'_>,
    replay: &EffectBankPreparationV1,
) -> Result<(), EffectStateDiagnosticV1> {
    let metadata = capability.bank.metadata();
    if metadata.width != capability.width {
        return Err(bank_restore_error(2));
    }
    let expected = effect_state_expected_metadata_v1(
        capability.bound_factory.bound_descriptor,
        replay.state_replay(capability.bound_factory.descriptor.id),
    )
    .map_err(|_| bank_restore_error(2))?;
    if metadata.program_key != expected.program_key()
        || capability.metadata.program_key != expected.program_key()
    {
        return Err(bank_restore_error(3));
    }
    if !core::ptr::eq(
        capability.bound_factory.factory.descriptor(),
        capability.bound_factory.descriptor,
    ) {
        return Err(bank_restore_error(4));
    }
    Ok(())
}

pub fn snapshot_unpublished_effect_bank_track_state_v1(
    capability: &UnpublishedEffectBankStateV1<'_>,
    track_index: u32,
    limits: EffectStateLimitsV1,
    payload_scratch: &mut [u8],
    output: &mut [u8],
) -> Result<u64, EffectStateDiagnosticV1> {
    let replay = capability
        .replays
        .get(track_index as usize)
        .ok_or_else(|| bank_restore_error(1))?;
    let replay_view = replay.state_replay(capability.bound_factory.descriptor.id);
    let requirements = effect_state_v1_requirements(
        capability.bound_factory.bound_descriptor,
        replay_view,
        limits,
    )?;
    let output_bytes = usize::try_from(requirements.envelope_bytes)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCodeV1::Overflow, 0))?;
    if output.len() < output_bytes {
        return Err(EffectStateDiagnosticV1::buffer_too_small(
            EFFECT_STATE_V1_BUFFER_ENVELOPE_OUTPUT,
            requirements.envelope_bytes,
        ));
    }
    let scratch_bytes = usize::try_from(requirements.payload_snapshot_scratch_bytes)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCodeV1::Overflow, 0))?;
    if payload_scratch.len() < scratch_bytes {
        return Err(EffectStateDiagnosticV1::buffer_too_small(
            EFFECT_STATE_V1_BUFFER_PAYLOAD_SCRATCH,
            requirements.payload_snapshot_scratch_bytes,
        ));
    }
    let replay = unpublished_bank_track_replay(capability, track_index)?;
    validate_unpublished_bank_program_and_provenance(capability, replay)?;
    let sizes = capability.metadata.program_key.state_sizes;
    let (common, remainder) =
        payload_scratch[..scratch_bytes].split_at_mut(sizes.common_bytes as usize);
    let (left, right) = remainder.split_at_mut(sizes.left_bytes as usize);
    let payload_output = StatePayloadOutput::new(common, left, right, sizes)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCodeV1::Payload, 2))?;
    capability
        .bank
        .snapshot_track_state_payload(track_index, payload_output)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCodeV1::Payload, 2))?;
    encode_effect_state_v1(
        capability.bound_factory.bound_descriptor,
        replay.state_replay(capability.bound_factory.descriptor.id),
        common,
        left,
        right,
        limits,
        output,
    )
}

pub fn restore_unpublished_effect_bank_track_state_v1<'wire>(
    mut capability: UnpublishedEffectBankStateV1<'wire>,
    track_index: u32,
    envelope: &[u8],
    limits: EffectStateLimitsV1,
    admission: EffectStateRestoreAdmissionV1,
) -> Result<UnpublishedEffectBankStateV1<'wire>, EffectStateDiagnosticV1> {
    let state =
        verify_effect_state_v1(capability.bound_factory.bound_descriptor, envelope, limits)?;
    admit_restored_state(state, admission)?;
    let replay = unpublished_bank_track_replay(&capability, track_index)?;
    validate_effect_state_current_layout_v1(state)?;
    validate_effect_state_replay_v1(
        state,
        replay.state_replay(capability.bound_factory.descriptor.id),
    )
    .map_err(|_| bank_restore_error(2))?;
    validate_unpublished_bank_program_and_provenance(&capability, replay)?;
    let (common, left, right) = state.payloads();
    let payload_input = StatePayloadInput::new(
        common,
        left,
        right,
        capability.metadata.program_key.state_sizes,
    )
    .map_err(|_| state_unavailable(EffectStateDiagnosticCodeV1::Payload, 4))?;
    capability
        .bank
        .restore_track_state_payload(track_index, state.state_layout_version(), payload_input)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCodeV1::Payload, 4))?;
    Ok(capability)
}

fn state_limit(byte_offset: u64, required_bytes: u64) -> EffectStateDiagnosticV1 {
    let mut diagnostic = state_diagnostic(EffectStateDiagnosticCodeV1::Limit, 0, byte_offset);
    diagnostic.required_bytes = required_bytes;
    diagnostic
}

pub fn scalar_effect_state_v1_requirements(
    bound_factory: &WireBoundNativeEffectFactoryV1<'_>,
    replay: &EffectBankPreparationV1,
    limits: EffectStateLimitsV1,
) -> Result<EffectStateRequirementsV1, EffectStateDiagnosticV1> {
    effect_state_v1_requirements(
        bound_factory.bound_descriptor,
        replay.state_replay(bound_factory.descriptor.id),
        limits,
    )
}

pub fn snapshot_scalar_effect_state_v1(
    bound_factory: &WireBoundNativeEffectFactoryV1<'_>,
    replay: &EffectBankPreparationV1,
    processor: &dyn PreparedNativeEffect,
    limits: EffectStateLimitsV1,
    payload_scratch: &mut [u8],
    output: &mut [u8],
) -> Result<u64, EffectStateDiagnosticV1> {
    let replay_view = replay.state_replay(bound_factory.descriptor.id);
    let requirements =
        effect_state_v1_requirements(bound_factory.bound_descriptor, replay_view, limits)?;
    let output_bytes = usize::try_from(requirements.envelope_bytes)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCodeV1::Overflow, 0))?;
    if output.len() < output_bytes {
        return Err(EffectStateDiagnosticV1::buffer_too_small(
            EFFECT_STATE_V1_BUFFER_ENVELOPE_OUTPUT,
            requirements.envelope_bytes,
        ));
    }
    let scratch_bytes = usize::try_from(requirements.payload_snapshot_scratch_bytes)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCodeV1::Overflow, 0))?;
    if payload_scratch.len() < scratch_bytes {
        return Err(EffectStateDiagnosticV1::buffer_too_small(
            EFFECT_STATE_V1_BUFFER_PAYLOAD_SCRATCH,
            requirements.payload_snapshot_scratch_bytes,
        ));
    }
    let metadata = processor.metadata();
    validate_effect_state_metadata_v1(bound_factory.bound_descriptor, replay_view, metadata)?;
    let sizes = metadata.state_sizes;
    let (common, remainder) =
        payload_scratch[..scratch_bytes].split_at_mut(sizes.common_bytes as usize);
    let (left, right) = remainder.split_at_mut(sizes.left_bytes as usize);
    let payload_output = StatePayloadOutput::new(common, left, right, sizes)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCodeV1::Payload, 1))?;
    processor
        .snapshot_state_payload(payload_output)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCodeV1::Payload, 1))?;
    encode_effect_state_v1(
        bound_factory.bound_descriptor,
        replay_view,
        common,
        left,
        right,
        limits,
        output,
    )
}

pub(crate) fn admit_restored_state(
    state: VerifiedEffectStateV1<'_>,
    admission: EffectStateRestoreAdmissionV1,
) -> Result<(), EffectStateDiagnosticV1> {
    if !miso_engine_core::is_launch_sample_rate(miso_engine_core::SampleRateHz(
        admission.sample_rate,
    )) || state.sample_rate() != admission.sample_rate
    {
        return Err(state_limit(96, u64::from(state.sample_rate())));
    }
    if admission.quantum == 0 || state.quantum() != admission.quantum {
        return Err(state_limit(100, u64::from(state.quantum())));
    }
    let (saved_state_cap, saved_scratch_cap, saved_automation_cap) = state.request_limits();
    if admission.maximum_total_state_bytes == 0
        || saved_state_cap > admission.maximum_total_state_bytes
    {
        return Err(state_limit(192, saved_state_cap));
    }
    if admission.maximum_scratch_bytes == 0 || saved_scratch_cap > admission.maximum_scratch_bytes {
        return Err(state_limit(200, saved_scratch_cap));
    }
    if admission.maximum_automation_spans_per_block == 0
        || saved_automation_cap > admission.maximum_automation_spans_per_block
    {
        return Err(state_limit(208, u64::from(saved_automation_cap)));
    }
    let payload_bytes = state
        .state_sizes()
        .total()
        .ok_or_else(|| state_unavailable(EffectStateDiagnosticCodeV1::Overflow, 0))?;
    if payload_bytes > admission.maximum_total_state_bytes {
        return Err(state_limit(216, payload_bytes));
    }
    if state.scratch_bytes() > admission.maximum_scratch_bytes {
        return Err(state_limit(176, state.scratch_bytes()));
    }
    if state.automation_capacity() > admission.maximum_automation_spans_per_block {
        return Err(state_limit(184, u64::from(state.automation_capacity())));
    }
    Ok(())
}

fn restored_ports(
    descriptor: &'static EffectDescriptorV1,
    state: VerifiedEffectStateV1<'_>,
) -> Result<PreparedPortsV1, EffectStateDiagnosticV1> {
    let (kind, id, required) = state.sidechain();
    let sidechain = match kind {
        0 => PreparedSidechainPort::None,
        1 | 2 => {
            let port = descriptor
                .ports
                .iter()
                .find(|port| {
                    port.role == miso_engine_effect_contract::PortRole::SidechainInput
                        && port.id.as_str() == id
                        && port.required == required
                })
                .ok_or_else(|| state_unavailable(EffectStateDiagnosticCodeV1::Metadata, 9))?;
            if kind == 1 {
                PreparedSidechainPort::Unconnected {
                    id: port.id,
                    required,
                }
            } else {
                PreparedSidechainPort::Connected {
                    id: port.id,
                    required,
                }
            }
        }
        _ => return Err(state_unavailable(EffectStateDiagnosticCodeV1::Metadata, 9)),
    };
    Ok(PreparedPortsV1 { sidechain })
}

pub fn restore_scalar_effect_state_v1<'wire>(
    bound_factory: WireBoundNativeEffectFactoryV1<'wire>,
    envelope: &[u8],
    limits: EffectStateLimitsV1,
    admission: EffectStateRestoreAdmissionV1,
    initial_value_scratch: &mut [InitialParameterValue],
) -> Result<RestoredScalarEffectStateV1<'wire>, EffectStateDiagnosticV1> {
    let state = verify_effect_state_v1(bound_factory.bound_descriptor, envelope, limits)?;
    admit_restored_state(state, admission)?;
    validate_effect_state_current_layout_v1(state)?;
    let initial_count = state.initial_values().len();
    if initial_value_scratch.len() < initial_count {
        let required_bytes = initial_count
            .checked_mul(core::mem::size_of::<InitialParameterValue>())
            .and_then(|bytes| u64::try_from(bytes).ok())
            .ok_or_else(|| state_unavailable(EffectStateDiagnosticCodeV1::Overflow, 0))?;
        return Err(EffectStateDiagnosticV1::buffer_too_small(
            EFFECT_STATE_V1_BUFFER_INITIAL_VALUE_SCRATCH,
            required_bytes,
        ));
    }
    for (destination, value) in initial_value_scratch[..initial_count]
        .iter_mut()
        .zip(state.initial_values())
    {
        *destination = value;
    }
    let ports = restored_ports(bound_factory.descriptor, state)?;
    let (maximum_total_state_bytes, maximum_scratch_bytes, maximum_automation_spans_per_block) =
        state.request_limits();
    let replay = EffectBankPreparationV1 {
        sample_rate: state.sample_rate(),
        quantum: state.quantum(),
        quality: state.quality(),
        bypass: state.bypass(),
        link_mode: state.link_mode(),
        ports,
        initial_values: initial_value_scratch[..initial_count]
            .to_vec()
            .into_boxed_slice(),
        limits: PrepareEffectLimits {
            maximum_total_state_bytes,
            maximum_scratch_bytes,
            maximum_automation_spans_per_block,
        },
    };
    let replay_view = replay.state_replay(bound_factory.descriptor.id);
    validate_effect_state_replay_v1(state, replay_view)?;
    let mut processor = bound_factory
        .factory
        .prepare(replay.request())
        .map_err(|_| state_unavailable(EffectStateDiagnosticCodeV1::Factory, 3))?;
    let metadata = processor.metadata();
    validate_effect_state_metadata_v1(bound_factory.bound_descriptor, replay_view, metadata)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCodeV1::Factory, 4))?;
    let (common, left, right) = state.payloads();
    let payload_input = StatePayloadInput::new(common, left, right, metadata.state_sizes)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCodeV1::Payload, 3))?;
    processor
        .restore_state_payload(state.state_layout_version(), payload_input)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCodeV1::Payload, 3))?;
    Ok(RestoredScalarEffectStateV1 {
        processor,
        metadata,
        bound_factory,
        replay,
    })
}
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EffectRack {
    Simd1,
    Dynamic,
    Simd2,
}
pub struct EffectPreparedSession {
    pub session: CompiledSession,
    pub entries: Vec<EffectPreparedEntry>,
}

/// Construct the caller-injected native registry for the V1 launch effect set.
///
/// Registry construction is control-plane work. Callers retain and inject the immutable registry
/// into [`prepare_native_session_effects`]; there is no render-reachable global catalog.
pub fn launch_native_effect_registry_v1() -> Result<NativeEffectRegistry, RegistryError> {
    NativeEffectRegistry::new([
        Box::new(miso_engine_parametric_eq::ParametricEqFactory) as Box<dyn NativeEffectFactory>,
        Box::new(miso_engine_compressor::CompressorFactory) as Box<dyn NativeEffectFactory>,
        Box::new(miso_engine_gate_expander::GateExpanderFactory) as Box<dyn NativeEffectFactory>,
        Box::new(miso_engine_multiband_compressor::MultibandCompressorFactory)
            as Box<dyn NativeEffectFactory>,
        Box::new(miso_engine_true_peak_limiter::TruePeakLimiterFactory)
            as Box<dyn NativeEffectFactory>,
        Box::new(miso_engine_soft_clip::SoftClipFactory) as Box<dyn NativeEffectFactory>,
        Box::new(miso_engine_transient_shaper::TransientShaperFactory)
            as Box<dyn NativeEffectFactory>,
        Box::new(miso_engine_delay::DelayFactory) as Box<dyn NativeEffectFactory>,
    ])
}

pub fn prepare_native_session_effects(
    session: &CompiledSession,
    registry: &NativeEffectRegistry,
    caps: EffectCompileCaps,
) -> Result<EffectPreparedSession, EffectDiagnosticSet> {
    let mut diagnostics = Vec::new();
    let mut entries = Vec::new();
    if caps.maximum_total_state_bytes == 0
        || caps.maximum_scratch_bytes == 0
        || caps.maximum_automation_spans_per_block == 0
    {
        return Err(EffectDiagnosticSet::sorted(vec![EffectDiagnostic {
            code: "effect.resource.limit",
            path: "$.effect_compile_caps".to_owned(),
        }]));
    }
    for track in &session.normalized_model().tracks {
        for (rack, effects) in [
            (EffectRack::Simd1, &track.simd1.effects),
            (EffectRack::Dynamic, &track.dynamic.effects),
            (EffectRack::Simd2, &track.simd2.effects),
        ] {
            for effect in effects {
                let path = format!("$.tracks[id={}].effects[id={}]", track.id, effect.id);
                let EffectIdentity::Native { effect_id } = &effect.identity else {
                    diagnostics.push(EffectDiagnostic {
                        code: "effect.third_party.unavailable_at_launch",
                        path,
                    });
                    continue;
                };
                let Some(factory) = registry.get_shared_ascii(effect_id.as_str()) else {
                    diagnostics.push(EffectDiagnostic {
                        code: "effect.native.unavailable",
                        path,
                    });
                    continue;
                };
                let descriptor = factory.descriptor();
                let quality = match effect.quality {
                    miso_engine_session::EffectQuality::Draft => EffectQuality::Draft,
                    miso_engine_session::EffectQuality::Normal => EffectQuality::Normal,
                    miso_engine_session::EffectQuality::High => EffectQuality::High,
                };
                let link_mode = match effect.link_mode {
                    SessionLinkMode::DualMono => LinkMode::DualMono,
                    SessionLinkMode::Maximum => LinkMode::Maximum,
                    SessionLinkMode::Average => LinkMode::Average,
                };
                if !descriptor.supported_link_modes.contains(link_mode) {
                    diagnostics.push(EffectDiagnostic {
                        code: "effect.link_mode.unsupported",
                        path,
                    });
                    continue;
                }
                if !descriptor.qualities.iter().any(|item| {
                    item.quality == quality && item.sample_rate == session.sample_rate().0
                }) {
                    diagnostics.push(EffectDiagnostic {
                        code: "effect.quality.unsupported",
                        path,
                    });
                    continue;
                }
                let mut initial = Vec::new();
                let mut invalid = false;
                for (index, parameter) in descriptor.parameters.iter().enumerate() {
                    let matching: Vec<_> = effect
                        .params
                        .iter()
                        .filter(|item| item.parameter_id == parameter.id.0)
                        .collect();
                    if matching
                        .iter()
                        .any(|item| !same_unit(item.unit, parameter.unit))
                    {
                        diagnostics.push(EffectDiagnostic {
                            code: "effect.parameter.unit_mismatch",
                            path: path.clone(),
                        });
                        invalid = true;
                        break;
                    }
                    match parameter.channel_policy {
                        ParameterChannelPolicy::Shared => {
                            let values: Vec<_> = matching
                                .iter()
                                .filter(|item| item.channel == SessionChannel::Both)
                                .collect();
                            if matching.len() != values.len() || values.len() > 1 {
                                diagnostics.push(EffectDiagnostic {
                                    code: "effect.parameter.channel",
                                    path: path.clone(),
                                });
                                invalid = true;
                                break;
                            }
                            initial.push(InitialParameterValue {
                                parameter_index: index as u32,
                                channel: ParameterChannel::Both,
                                value: normalize_zero(
                                    values
                                        .first()
                                        .map_or(parameter.default_value, |item| item.value),
                                ),
                            });
                        }
                        ParameterChannelPolicy::PerLane => {
                            let both_count = matching
                                .iter()
                                .filter(|item| item.channel == SessionChannel::Both)
                                .count();
                            let left_count = matching
                                .iter()
                                .filter(|item| item.channel == SessionChannel::Left)
                                .count();
                            let right_count = matching
                                .iter()
                                .filter(|item| item.channel == SessionChannel::Right)
                                .count();
                            if both_count > 1
                                || left_count > 1
                                || right_count > 1
                                || (both_count == 1 && (left_count != 0 || right_count != 0))
                            {
                                diagnostics.push(EffectDiagnostic {
                                    code: "effect.parameter.duplicate_channel",
                                    path: path.clone(),
                                });
                                invalid = true;
                                break;
                            }
                            let both = matching
                                .iter()
                                .find(|item| item.channel == SessionChannel::Both)
                                .map(|item| item.value);
                            for (channel, requested) in [
                                (
                                    ParameterChannel::Left,
                                    matching
                                        .iter()
                                        .find(|item| item.channel == SessionChannel::Left)
                                        .map(|item| item.value),
                                ),
                                (
                                    ParameterChannel::Right,
                                    matching
                                        .iter()
                                        .find(|item| item.channel == SessionChannel::Right)
                                        .map(|item| item.value),
                                ),
                            ] {
                                initial.push(InitialParameterValue {
                                    parameter_index: index as u32,
                                    channel,
                                    value: normalize_zero(
                                        requested.or(both).unwrap_or(parameter.default_value),
                                    ),
                                });
                            }
                        }
                    }
                }
                if invalid {
                    continue;
                }
                if let Some(item) = initial.iter().find(|item| {
                    !parameter_value_is_valid(
                        &descriptor.parameters[item.parameter_index as usize],
                        item.value,
                    )
                }) {
                    let _ = item;
                    diagnostics.push(EffectDiagnostic {
                        code: "effect.parameter.domain",
                        path,
                    });
                    continue;
                }
                if effect.params.iter().any(|item| {
                    !descriptor
                        .parameters
                        .iter()
                        .any(|parameter| parameter.id.0 == item.parameter_id)
                }) {
                    diagnostics.push(EffectDiagnostic {
                        code: "effect.parameter.unknown",
                        path,
                    });
                    continue;
                }
                let declared_sidechain = descriptor.ports.iter().find(|port| {
                    port.role == miso_engine_effect_contract::PortRole::SidechainInput
                });
                let ports = match (&effect.sidechain, declared_sidechain) {
                    (SidechainDeclaration::None, None) => PreparedPortsV1 {
                        sidechain: PreparedSidechainPort::None,
                    },
                    (SidechainDeclaration::None, Some(port)) if !port.required => PreparedPortsV1 {
                        sidechain: PreparedSidechainPort::Unconnected {
                            id: port.id,
                            required: false,
                        },
                    },
                    (SidechainDeclaration::None, Some(_)) => {
                        diagnostics.push(EffectDiagnostic {
                            code: "effect.sidechain.missing",
                            path,
                        });
                        continue;
                    }
                    (SidechainDeclaration::Routed(_), None) => {
                        diagnostics.push(EffectDiagnostic {
                            code: "effect.sidechain.unexpected",
                            path,
                        });
                        continue;
                    }
                    (SidechainDeclaration::Routed(sidechain), Some(_)) => {
                        match descriptor.ports.iter().find(|port| {
                            port.role == miso_engine_effect_contract::PortRole::SidechainInput
                                && port.id.as_str() == sidechain.port_id.as_str()
                        }) {
                            Some(port) => PreparedPortsV1 {
                                sidechain: PreparedSidechainPort::Connected {
                                    id: port.id,
                                    required: port.required,
                                },
                            },
                            None => {
                                diagnostics.push(EffectDiagnostic {
                                    code: "effect.sidechain.unknown_port",
                                    path,
                                });
                                continue;
                            }
                        }
                    }
                };
                let bank_preparation = EffectBankPreparationV1 {
                    sample_rate: session.sample_rate().0,
                    quantum: session.quantum().0,
                    quality,
                    bypass: effect.bypass,
                    link_mode,
                    ports,
                    initial_values: initial.into_boxed_slice(),
                    limits: PrepareEffectLimits {
                        maximum_total_state_bytes: caps.maximum_total_state_bytes,
                        maximum_scratch_bytes: caps.maximum_scratch_bytes,
                        maximum_automation_spans_per_block: caps.maximum_automation_spans_per_block,
                    },
                };
                let request = bank_preparation.request();
                let expected = match expected_prepared_metadata(descriptor, request) {
                    Ok(metadata) => metadata,
                    Err(error) => {
                        diagnostics.push(EffectDiagnostic {
                            code: error.code,
                            path,
                        });
                        continue;
                    }
                };
                let processor = match factory.prepare(request) {
                    Ok(value) => value,
                    Err(error) => {
                        diagnostics.push(EffectDiagnostic {
                            code: error.code,
                            path,
                        });
                        continue;
                    }
                };
                let metadata = processor.metadata();
                if metadata.descriptor.id != expected.descriptor.id
                    || metadata.descriptor.contract_major != expected.descriptor.contract_major
                    || metadata.descriptor.state_layout_version
                        != expected.descriptor.state_layout_version
                    || metadata.sample_rate != expected.sample_rate
                    || metadata.quantum != expected.quantum
                    || metadata.quality != expected.quality
                    || metadata.bypass != expected.bypass
                    || metadata.link_mode != expected.link_mode
                    || metadata.ports != expected.ports
                    || metadata.latency != expected.latency
                    || metadata.tail != expected.tail
                    || metadata.state_sizes != expected.state_sizes
                    || metadata.scratch_bytes != expected.scratch_bytes
                    || metadata.automation_capacity != expected.automation_capacity
                {
                    diagnostics.push(EffectDiagnostic {
                        code: "effect.metadata.mismatch",
                        path,
                    });
                    continue;
                }
                entries.push(EffectPreparedEntry {
                    track_id: track.id.as_str().to_owned(),
                    rack,
                    effect_id: effect.id.as_str().to_owned(),
                    processor,
                    metadata,
                    factory,
                    bank_preparation,
                });
            }
        }
    }
    if diagnostics.is_empty() {
        entries.sort_by(|a, b| {
            (&a.track_id, a.rack, &a.effect_id).cmp(&(&b.track_id, b.rack, &b.effect_id))
        });
        Ok(EffectPreparedSession {
            session: session.clone(),
            entries,
        })
    } else {
        Err(EffectDiagnosticSet::sorted(diagnostics))
    }
}

fn normalize_zero(value: f32) -> f32 {
    if value == 0.0 { 0.0 } else { value }
}

fn parameter_value_is_valid(
    descriptor: &miso_engine_effect_contract::ParameterDescriptorV1,
    value: f32,
) -> bool {
    if !value.is_finite() {
        return false;
    }
    match descriptor.domain {
        miso_engine_effect_contract::ParameterDomain::Continuous => descriptor
            .minimum
            .zip(descriptor.maximum)
            .is_some_and(|(minimum, maximum)| value >= minimum && value <= maximum),
        miso_engine_effect_contract::ParameterDomain::Boolean => value == 0.0 || value == 1.0,
        miso_engine_effect_contract::ParameterDomain::Enumeration => {
            descriptor.enum_choices.iter().any(|choice| {
                normalize_zero(choice.value).to_bits() == normalize_zero(value).to_bits()
            })
        }
    }
}
fn same_unit(session: SessionUnit, contract: ParameterUnit) -> bool {
    matches!(
        (session, contract),
        (SessionUnit::Db, ParameterUnit::Db)
            | (SessionUnit::Hz, ParameterUnit::Hz)
            | (SessionUnit::Milliseconds, ParameterUnit::Milliseconds)
            | (SessionUnit::Samples, ParameterUnit::Samples)
            | (SessionUnit::Linear, ParameterUnit::Linear)
            | (SessionUnit::Ratio, ParameterUnit::Ratio)
    )
}
