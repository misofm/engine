use core::num::NonZeroUsize;
use effect_contract::{
    BankWidth, ChannelSymmetryWitness, EffectControlLane, EffectControlRecord, EffectDescriptor,
    EffectProgramKey, EffectQuality, InitialParameterValue, LinkMode, NativeEffectFactory,
    NativeEffectRegistry, ObservationLane, ParameterChannel, ParameterChannelPolicy, ParameterUnit,
    PrepareEffectBankRequest, PrepareEffectLimits, PrepareEffectRequest, PreparedBankMetadata,
    PreparedEffectMetadata, PreparedNativeEffect, PreparedNativeEffectBank, PreparedPorts,
    PreparedSidechainPort, RegistryError, StatePayloadInput, StatePayloadOutput,
    expected_prepared_metadata, payload_sections_agree,
};
use effect_package::{
    BoundEffectDescriptorWire, EFFECT_STATE_BUFFER_ENVELOPE_OUTPUT,
    EFFECT_STATE_BUFFER_INITIAL_VALUE_SCRATCH, EFFECT_STATE_BUFFER_PAYLOAD_SCRATCH,
    EFFECT_STATE_UNAVAILABLE_INDEX, EFFECT_STATE_UNAVAILABLE_OFFSET,
    EffectDescriptorBindingErrorKind, EffectStateDerivedResources, EffectStateDiagnostic,
    EffectStateDiagnosticCode, EffectStateLimits, EffectStateReplayView, EffectStateRequirements,
    VerifiedEffectState, bind_effect_descriptor_wire, effect_state_derived_resources,
    effect_state_expected_metadata, effect_state_requirements, encode_effect_state,
    validate_effect_state_current_layout, validate_effect_state_metadata,
    validate_effect_state_replay, verify_effect_state,
};
use engine::realtime::{
    ObservationReader, Producer, QueueFull, QueueGeneration, bounded_spsc, observation_slot,
};
use session::{
    CompiledSession, EffectIdentity, LinkMode as SessionLinkMode,
    ParameterChannel as SessionChannel, ParameterUnit as SessionUnit, SidechainDeclaration,
};
use std::collections::BTreeMap;
use std::sync::Arc;

use crate::control::{EffectControlOwner, EffectControlOwnerError};
use crate::{EffectDiagnostic, EffectDiagnosticSet};
use lane::Backend;

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
    pub factory: Arc<dyn effect_contract::NativeEffectFactory>,
    /// Exact owned request inputs used to prepare the scalar processor.
    pub bank_preparation: EffectBankPreparation,
    /// The consumer half of this instance's live-console control channel (issue #140 A).
    ///
    /// `None` unless [`attach_effect_console`] was called, which is the only way one is ever
    /// created. It travels with the entry into `GraphPreparedEffect`, so the plan that renders the
    /// effect is the one that drains its queue, and a session with no console carries a `None`
    /// that the runtime turns back into the byte-identical console-free path.
    pub control: Option<Box<EffectControlLane>>,
    /// This instance's observation taps (issue #143 D3, level 1).
    ///
    /// `None` unless [`attach_effect_observation`] was called, which is the only way one is
    /// ever created. A session whose console request named no observation capacity carries `None`
    /// here, and the runtime turns that back into the byte-identical unobserved path: there is no
    /// lane, no slot and no vector anywhere in the compiled plan.
    pub observation: Option<Box<ObservationLane>>,
}

/// Owned replayable portion of an accepted prepare request. It never crosses into render.
#[derive(Clone, Debug)]
pub struct EffectBankPreparation {
    pub sample_rate: u32,
    pub quantum: u32,
    pub quality: EffectQuality,
    pub bypass: bool,
    pub link_mode: LinkMode,
    pub ports: PreparedPorts,
    pub initial_values: Box<[InitialParameterValue]>,
    pub limits: PrepareEffectLimits,
}

impl EffectBankPreparation {
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
    pub fn state_replay(&self, effect_id: effect_contract::EffectId) -> EffectStateReplayView<'_> {
        EffectStateReplayView {
            effect_id,
            request: self.request(),
        }
    }
}

pub struct WireBoundNativeEffectFactory<'wire> {
    factory: Arc<dyn NativeEffectFactory>,
    descriptor: &'static EffectDescriptor,
    bound_descriptor: BoundEffectDescriptorWire<'wire>,
}

impl core::fmt::Debug for WireBoundNativeEffectFactory<'_> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("WireBoundNativeEffectFactory")
            .field("effect_id", &self.descriptor.id)
            .field("descriptor_identity", &self.bound_descriptor.identity())
            .finish_non_exhaustive()
    }
}

impl<'wire> WireBoundNativeEffectFactory<'wire> {
    #[must_use]
    pub fn factory(&self) -> &Arc<dyn NativeEffectFactory> {
        &self.factory
    }

    #[must_use]
    pub const fn descriptor(&self) -> &'static EffectDescriptor {
        self.descriptor
    }

    #[must_use]
    pub const fn bound_descriptor(&self) -> BoundEffectDescriptorWire<'wire> {
        self.bound_descriptor
    }
}

fn descriptor_binding_diagnostic(
    error: effect_package::EffectDescriptorBindingError,
) -> EffectStateDiagnostic {
    let nested = error.diagnostic();
    let kind = match error.kind() {
        EffectDescriptorBindingErrorKind::ExternalWire => 1,
        EffectDescriptorBindingErrorKind::StaticDescriptorMismatch => 2,
    };
    let byte_offset = if nested.byte_offset == effect_package::EFFECT_DESCRIPTOR_WIRE_UNAVAILABLE {
        EFFECT_STATE_UNAVAILABLE_OFFSET
    } else {
        u64::from(nested.byte_offset)
    };
    let mut diagnostic = EffectStateDiagnostic::new(
        EffectStateDiagnosticCode::Descriptor,
        (kind << 16) | nested.code as u32,
        nested.record_index,
        byte_offset,
    );
    diagnostic.required_bytes = u64::from(nested.required_bytes);
    diagnostic
}

pub fn bind_native_effect_factory_state(
    factory: Arc<dyn NativeEffectFactory>,
    descriptor_wire: &[u8],
    maximum_descriptor_bytes: u32,
) -> Result<WireBoundNativeEffectFactory<'_>, EffectStateDiagnostic> {
    let descriptor = factory.descriptor();
    let bound_descriptor =
        bind_effect_descriptor_wire(descriptor, descriptor_wire, maximum_descriptor_bytes)
            .map_err(descriptor_binding_diagnostic)?;
    Ok(WireBoundNativeEffectFactory {
        factory,
        descriptor,
        bound_descriptor,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectStateRestoreAdmission {
    pub sample_rate: u32,
    pub quantum: u32,
    pub maximum_total_state_bytes: u64,
    pub maximum_scratch_bytes: u64,
    pub maximum_automation_spans_per_block: u32,
}

pub struct RestoredScalarEffectState<'wire> {
    processor: Box<dyn PreparedNativeEffect>,
    metadata: PreparedEffectMetadata,
    bound_factory: WireBoundNativeEffectFactory<'wire>,
    replay: EffectBankPreparation,
    /// This restore's channel-symmetry witness, decided from the envelope's own bytes.
    symmetry: ChannelSymmetryWitness,
}

impl core::fmt::Debug for RestoredScalarEffectState<'_> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("RestoredScalarEffectState")
            .field("metadata", &self.metadata)
            .field("replay", &self.replay)
            .finish_non_exhaustive()
    }
}

impl RestoredScalarEffectState<'_> {
    /// This restored instance's channel-symmetry witness.
    ///
    /// `RESTORED` holds exactly when the envelope's left and right payload sections compared
    /// **byte-equal**, and `DESIGNED` is the restored processor's own comparison of the words its
    /// kernel reads. The other three terms are set: a restore says nothing about the track's
    /// source mapping and nothing about live console traffic.
    ///
    /// # Why the check is on the wire bytes and not on the restored object
    ///
    /// The payload is the whole of what a restore carries, and it carries *state* as well as
    /// designed words -- rings, cursors, integrators. Two channels whose designed words agree but
    /// whose restored rings differ are not doing identical work and must not be collapsed, and
    /// only the byte comparison sees that. It is also the cheapest possible form: one `memcmp` of
    /// two equal-length slices, off the render thread, once.
    #[must_use]
    pub const fn channel_symmetry(&self) -> ChannelSymmetryWitness {
        self.symmetry
    }

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
    pub fn bound_factory(&self) -> &WireBoundNativeEffectFactory<'_> {
        &self.bound_factory
    }

    #[must_use]
    pub fn replay(&self) -> &EffectBankPreparation {
        &self.replay
    }
}

pub struct UnpublishedEffectBankState<'wire> {
    bank: Box<dyn PreparedNativeEffectBank>,
    metadata: PreparedBankMetadata,
    backend: Backend,
    width: BankWidth,
    bound_factory: WireBoundNativeEffectFactory<'wire>,
    replays: Box<[EffectBankPreparation]>,
    /// Per-lane `RESTORED` term, one entry per lane of the bound width.
    ///
    /// A freshly bound bank has restored nothing, so every lane starts `true`: the term says "no
    /// restore has contradicted this lane", and a lane that was never restored has not been
    /// contradicted. Each `restore_unpublished_effect_bank_track_state` writes exactly its own
    /// lane's entry, which is what keeps the per-lane witnesses free of cross-lane coupling.
    restored: Box<[bool]>,
}

impl core::fmt::Debug for UnpublishedEffectBankState<'_> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("UnpublishedEffectBankState")
            .field("metadata", &self.metadata)
            .field("backend", &self.backend)
            .field("width", &self.width)
            .field("replay_count", &self.replays.len())
            .finish_non_exhaustive()
    }
}

impl UnpublishedEffectBankState<'_> {
    /// One lane's channel-symmetry witness for this bound bank.
    ///
    /// `RESTORED` is what that lane's restore decided (see
    /// [`RestoredScalarEffectState::channel_symmetry`] for the argument); `DESIGNED` is the
    /// bank's own comparison of the words its kernel reads for that lane. A lane index the bound
    /// width does not have declines outright.
    #[must_use]
    pub fn lane_channel_symmetry(&self, lane: usize) -> ChannelSymmetryWitness {
        let Some(restored) = self.restored.get(lane).copied() else {
            return ChannelSymmetryWitness::DECLINED;
        };
        let mut witness = ChannelSymmetryWitness::SYMMETRIC;
        witness.set(ChannelSymmetryWitness::RESTORED, restored);
        witness.set(
            ChannelSymmetryWitness::DESIGNED,
            self.bank.lane_channel_symmetry(lane),
        );
        witness
    }

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
    pub const fn backend(&self) -> Backend {
        self.backend
    }

    #[must_use]
    pub const fn width(&self) -> BankWidth {
        self.width
    }

    #[must_use]
    pub fn bound_factory(&self) -> &WireBoundNativeEffectFactory<'_> {
        &self.bound_factory
    }

    #[must_use]
    pub fn replays(&self) -> &[EffectBankPreparation] {
        &self.replays
    }
}

fn state_diagnostic(
    code: EffectStateDiagnosticCode,
    detail: u32,
    byte_offset: u64,
) -> EffectStateDiagnostic {
    EffectStateDiagnostic::new(code, detail, EFFECT_STATE_UNAVAILABLE_INDEX, byte_offset)
}

fn state_unavailable(code: EffectStateDiagnosticCode, detail: u32) -> EffectStateDiagnostic {
    state_diagnostic(code, detail, EFFECT_STATE_UNAVAILABLE_OFFSET)
}

fn bank_restore_error(detail: u32) -> EffectStateDiagnostic {
    state_unavailable(EffectStateDiagnosticCode::Restore, detail)
}

fn admit_bank_preparation(
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
    Ok(())
}

fn admit_bank_derived_resources(
    resources: EffectStateDerivedResources,
    admission: EffectStateRestoreAdmission,
) -> Result<(), EffectStateDiagnostic> {
    let payload_bytes = resources
        .state_sizes
        .total()
        .ok_or_else(|| state_unavailable(EffectStateDiagnosticCode::Overflow, 0))?;
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
    bound_factory: &WireBoundNativeEffectFactory<'_>,
    backend: Backend,
    width: BankWidth,
    replays: &[EffectBankPreparation],
    admission: EffectStateRestoreAdmission,
) -> Result<EffectProgramKey, EffectStateDiagnostic> {
    if !width.matches_backend(backend) || replays.len() != width.lanes() as usize {
        return Err(state_unavailable(EffectStateDiagnosticCode::Factory, 2));
    }
    let mut program_key = None;
    for (track_index, replay) in replays.iter().enumerate() {
        admit_bank_preparation(replay, admission).map_err(|mut diagnostic| {
            diagnostic.item_index = track_index as u32;
            diagnostic
        })?;
        let replay_view = replay.state_replay(bound_factory.descriptor.id);
        let resources =
            effect_state_derived_resources(bound_factory.bound_descriptor, replay.request())
                .map_err(|_| state_unavailable(EffectStateDiagnosticCode::Factory, 2))?;
        admit_bank_derived_resources(resources, admission).map_err(|mut diagnostic| {
            diagnostic.item_index = track_index as u32;
            diagnostic
        })?;
        let metadata = effect_state_expected_metadata(bound_factory.bound_descriptor, replay_view)
            .map_err(|_| state_unavailable(EffectStateDiagnosticCode::Factory, 2))?;
        let candidate = metadata.program_key();
        if program_key
            .as_ref()
            .is_some_and(|expected| expected != &candidate)
        {
            return Err(state_unavailable(EffectStateDiagnosticCode::Factory, 2));
        }
        program_key = Some(candidate);
    }
    program_key.ok_or_else(|| state_unavailable(EffectStateDiagnosticCode::Factory, 2))
}

pub fn prepare_unpublished_effect_bank_state<'wire>(
    bound_factory: WireBoundNativeEffectFactory<'wire>,
    backend: Backend,
    width: BankWidth,
    replays: Box<[EffectBankPreparation]>,
    admission: EffectStateRestoreAdmission,
) -> Result<UnpublishedEffectBankState<'wire>, EffectStateDiagnostic> {
    let program_key =
        expected_bank_program_key(&bound_factory, backend, width, replays.as_ref(), admission)?;
    let requests: Vec<_> = replays.iter().map(EffectBankPreparation::request).collect();
    let bank = bound_factory
        .factory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
        })
        .map_err(|_| state_unavailable(EffectStateDiagnosticCode::Factory, 3))?
        .ok_or_else(|| state_unavailable(EffectStateDiagnosticCode::Factory, 3))?;
    let metadata = bank.metadata();
    if metadata.width != width || metadata.program_key != program_key {
        return Err(state_unavailable(EffectStateDiagnosticCode::Factory, 4));
    }
    if !core::ptr::eq(bound_factory.factory.descriptor(), bound_factory.descriptor) {
        return Err(state_unavailable(EffectStateDiagnosticCode::Factory, 4));
    }
    let restored = vec![true; width.lanes() as usize].into_boxed_slice();
    Ok(UnpublishedEffectBankState {
        bank,
        metadata,
        backend,
        width,
        bound_factory,
        replays,
        restored,
    })
}

fn unpublished_bank_track_replay<'a>(
    capability: &'a UnpublishedEffectBankState<'_>,
    track_index: u32,
) -> Result<&'a EffectBankPreparation, EffectStateDiagnostic> {
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
    capability: &UnpublishedEffectBankState<'_>,
    replay: &EffectBankPreparation,
) -> Result<(), EffectStateDiagnostic> {
    let metadata = capability.bank.metadata();
    if metadata.width != capability.width {
        return Err(bank_restore_error(2));
    }
    let expected = effect_state_expected_metadata(
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

pub fn snapshot_unpublished_effect_bank_track_state(
    capability: &UnpublishedEffectBankState<'_>,
    track_index: u32,
    limits: EffectStateLimits,
    payload_scratch: &mut [u8],
    output: &mut [u8],
) -> Result<u64, EffectStateDiagnostic> {
    let replay = capability
        .replays
        .get(track_index as usize)
        .ok_or_else(|| bank_restore_error(1))?;
    let replay_view = replay.state_replay(capability.bound_factory.descriptor.id);
    let requirements = effect_state_requirements(
        capability.bound_factory.bound_descriptor,
        replay_view,
        limits,
    )?;
    let output_bytes = usize::try_from(requirements.envelope_bytes)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCode::Overflow, 0))?;
    if output.len() < output_bytes {
        return Err(EffectStateDiagnostic::buffer_too_small(
            EFFECT_STATE_BUFFER_ENVELOPE_OUTPUT,
            requirements.envelope_bytes,
        ));
    }
    let scratch_bytes = usize::try_from(requirements.payload_snapshot_scratch_bytes)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCode::Overflow, 0))?;
    if payload_scratch.len() < scratch_bytes {
        return Err(EffectStateDiagnostic::buffer_too_small(
            EFFECT_STATE_BUFFER_PAYLOAD_SCRATCH,
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
        .map_err(|_| state_unavailable(EffectStateDiagnosticCode::Payload, 2))?;
    capability
        .bank
        .snapshot_track_state_payload(track_index, payload_output)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCode::Payload, 2))?;
    encode_effect_state(
        capability.bound_factory.bound_descriptor,
        replay.state_replay(capability.bound_factory.descriptor.id),
        common,
        left,
        right,
        limits,
        output,
    )
}

pub fn restore_unpublished_effect_bank_track_state<'wire>(
    mut capability: UnpublishedEffectBankState<'wire>,
    track_index: u32,
    envelope: &[u8],
    limits: EffectStateLimits,
    admission: EffectStateRestoreAdmission,
) -> Result<UnpublishedEffectBankState<'wire>, EffectStateDiagnostic> {
    let state = verify_effect_state(capability.bound_factory.bound_descriptor, envelope, limits)?;
    admit_restored_state(state, admission)?;
    let replay = unpublished_bank_track_replay(&capability, track_index)?;
    validate_effect_state_current_layout(state)?;
    validate_effect_state_replay(
        state,
        replay.state_replay(capability.bound_factory.descriptor.id),
    )
    .map_err(|_| bank_restore_error(2))?;
    validate_unpublished_bank_program_and_provenance(&capability, replay)?;
    let (common, left, right) = state.payloads();
    // The restore hook. Taken here, before a byte enters the bank, on the two equal-length
    // sections of the verified envelope: byte equality of the sections is bitwise equality of the
    // two channels' words, `-0.0` included, because every payload word is little-endian raw bits.
    // An unequal restore declines this lane until a reset, and cannot be re-earned by the
    // parameter words agreeing afterwards -- equal words do not imply equal state.
    let sections_agree = payload_sections_agree(left, right);
    let payload_input = StatePayloadInput::new(
        common,
        left,
        right,
        capability.metadata.program_key.state_sizes,
    )
    .map_err(|_| state_unavailable(EffectStateDiagnosticCode::Payload, 4))?;
    capability
        .bank
        .restore_track_state_payload(track_index, state.state_layout_version(), payload_input)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCode::Payload, 4))?;
    if let Some(lane) = capability.restored.get_mut(track_index as usize) {
        *lane = *lane && sections_agree;
    }
    Ok(capability)
}

fn state_limit(byte_offset: u64, required_bytes: u64) -> EffectStateDiagnostic {
    let mut diagnostic = state_diagnostic(EffectStateDiagnosticCode::Limit, 0, byte_offset);
    diagnostic.required_bytes = required_bytes;
    diagnostic
}

pub fn scalar_effect_state_requirements(
    bound_factory: &WireBoundNativeEffectFactory<'_>,
    replay: &EffectBankPreparation,
    limits: EffectStateLimits,
) -> Result<EffectStateRequirements, EffectStateDiagnostic> {
    effect_state_requirements(
        bound_factory.bound_descriptor,
        replay.state_replay(bound_factory.descriptor.id),
        limits,
    )
}

pub fn snapshot_scalar_effect_state(
    bound_factory: &WireBoundNativeEffectFactory<'_>,
    replay: &EffectBankPreparation,
    processor: &dyn PreparedNativeEffect,
    limits: EffectStateLimits,
    payload_scratch: &mut [u8],
    output: &mut [u8],
) -> Result<u64, EffectStateDiagnostic> {
    let replay_view = replay.state_replay(bound_factory.descriptor.id);
    let requirements =
        effect_state_requirements(bound_factory.bound_descriptor, replay_view, limits)?;
    let output_bytes = usize::try_from(requirements.envelope_bytes)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCode::Overflow, 0))?;
    if output.len() < output_bytes {
        return Err(EffectStateDiagnostic::buffer_too_small(
            EFFECT_STATE_BUFFER_ENVELOPE_OUTPUT,
            requirements.envelope_bytes,
        ));
    }
    let scratch_bytes = usize::try_from(requirements.payload_snapshot_scratch_bytes)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCode::Overflow, 0))?;
    if payload_scratch.len() < scratch_bytes {
        return Err(EffectStateDiagnostic::buffer_too_small(
            EFFECT_STATE_BUFFER_PAYLOAD_SCRATCH,
            requirements.payload_snapshot_scratch_bytes,
        ));
    }
    let metadata = processor.metadata();
    validate_effect_state_metadata(bound_factory.bound_descriptor, replay_view, metadata)?;
    let sizes = metadata.state_sizes;
    let (common, remainder) =
        payload_scratch[..scratch_bytes].split_at_mut(sizes.common_bytes as usize);
    let (left, right) = remainder.split_at_mut(sizes.left_bytes as usize);
    let payload_output = StatePayloadOutput::new(common, left, right, sizes)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCode::Payload, 1))?;
    processor
        .snapshot_state_payload(payload_output)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCode::Payload, 1))?;
    encode_effect_state(
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
    state: VerifiedEffectState<'_>,
    admission: EffectStateRestoreAdmission,
) -> Result<(), EffectStateDiagnostic> {
    if !engine::is_launch_sample_rate(engine::SampleRateHz(admission.sample_rate))
        || state.sample_rate() != admission.sample_rate
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
        .ok_or_else(|| state_unavailable(EffectStateDiagnosticCode::Overflow, 0))?;
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
    descriptor: &'static EffectDescriptor,
    state: VerifiedEffectState<'_>,
) -> Result<PreparedPorts, EffectStateDiagnostic> {
    let (kind, id, required) = state.sidechain();
    let sidechain = match kind {
        0 => PreparedSidechainPort::None,
        1 | 2 => {
            let port = descriptor
                .ports
                .iter()
                .find(|port| {
                    port.role == effect_contract::PortRole::SidechainInput
                        && port.id.as_str() == id
                        && port.required == required
                })
                .ok_or_else(|| state_unavailable(EffectStateDiagnosticCode::Metadata, 9))?;
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
        _ => return Err(state_unavailable(EffectStateDiagnosticCode::Metadata, 9)),
    };
    Ok(PreparedPorts { sidechain })
}

pub fn restore_scalar_effect_state<'wire>(
    bound_factory: WireBoundNativeEffectFactory<'wire>,
    envelope: &[u8],
    limits: EffectStateLimits,
    admission: EffectStateRestoreAdmission,
    initial_value_scratch: &mut [InitialParameterValue],
) -> Result<RestoredScalarEffectState<'wire>, EffectStateDiagnostic> {
    let state = verify_effect_state(bound_factory.bound_descriptor, envelope, limits)?;
    admit_restored_state(state, admission)?;
    validate_effect_state_current_layout(state)?;
    let initial_count = state.initial_values().len();
    if initial_value_scratch.len() < initial_count {
        let required_bytes = initial_count
            .checked_mul(core::mem::size_of::<InitialParameterValue>())
            .and_then(|bytes| u64::try_from(bytes).ok())
            .ok_or_else(|| state_unavailable(EffectStateDiagnosticCode::Overflow, 0))?;
        return Err(EffectStateDiagnostic::buffer_too_small(
            EFFECT_STATE_BUFFER_INITIAL_VALUE_SCRATCH,
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
    let replay = EffectBankPreparation {
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
    validate_effect_state_replay(state, replay_view)?;
    let mut processor = bound_factory
        .factory
        .prepare(replay.request())
        .map_err(|_| state_unavailable(EffectStateDiagnosticCode::Factory, 3))?;
    let metadata = processor.metadata();
    validate_effect_state_metadata(bound_factory.bound_descriptor, replay_view, metadata)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCode::Factory, 4))?;
    let (common, left, right) = state.payloads();
    // The restore hook; see `restore_unpublished_effect_bank_track_state` for the argument.
    let sections_agree = payload_sections_agree(left, right);
    let payload_input = StatePayloadInput::new(common, left, right, metadata.state_sizes)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCode::Payload, 3))?;
    processor
        .restore_state_payload(state.state_layout_version(), payload_input)
        .map_err(|_| state_unavailable(EffectStateDiagnosticCode::Payload, 3))?;
    let mut symmetry = ChannelSymmetryWitness::SYMMETRIC;
    symmetry.set(ChannelSymmetryWitness::RESTORED, sections_agree);
    symmetry.set(
        ChannelSymmetryWitness::DESIGNED,
        processor.channel_symmetry(),
    );
    Ok(RestoredScalarEffectState {
        processor,
        metadata,
        bound_factory,
        replay,
        symmetry,
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
pub fn launch_native_effect_registry() -> Result<NativeEffectRegistry, RegistryError> {
    NativeEffectRegistry::new([
        Box::new(parametric_eq::ParametricEqFactory) as Box<dyn NativeEffectFactory>,
        Box::new(compressor::CompressorFactory) as Box<dyn NativeEffectFactory>,
        Box::new(gate_expander::GateExpanderFactory) as Box<dyn NativeEffectFactory>,
        Box::new(multiband_compressor::MultibandCompressorFactory) as Box<dyn NativeEffectFactory>,
        Box::new(true_peak_limiter::TruePeakLimiterFactory) as Box<dyn NativeEffectFactory>,
        Box::new(soft_clip::SoftClipFactory) as Box<dyn NativeEffectFactory>,
        Box::new(transient_shaper::TransientShaperFactory) as Box<dyn NativeEffectFactory>,
        Box::new(delay::DelayFactory) as Box<dyn NativeEffectFactory>,
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
                    session::EffectQuality::Draft => EffectQuality::Draft,
                    session::EffectQuality::Normal => EffectQuality::Normal,
                    session::EffectQuality::High => EffectQuality::High,
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
                                value: effect_contract::normalize_zero(
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
                                    value: effect_contract::normalize_zero(
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
                    !effect_contract::parameter_value_valid(
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
                let declared_sidechain = descriptor
                    .ports
                    .iter()
                    .find(|port| port.role == effect_contract::PortRole::SidechainInput);
                let ports = match (&effect.sidechain, declared_sidechain) {
                    (SidechainDeclaration::None, None) => PreparedPorts {
                        sidechain: PreparedSidechainPort::None,
                    },
                    (SidechainDeclaration::None, Some(port)) if !port.required => PreparedPorts {
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
                            port.role == effect_contract::PortRole::SidechainInput
                                && port.id.as_str() == sidechain.port_id.as_str()
                        }) {
                            Some(port) => PreparedPorts {
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
                let bank_preparation = EffectBankPreparation {
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
                    control: None,
                    observation: None,
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

/// One prepared live-console control channel for one effect instance (issue #140 A).
///
/// The producer half stays on the control plane; the consumer half rode into the plan inside the
/// entry. A producer must be dropped before the plan that owns its consumer, which is why
/// `PreparedHost`'s field order puts the plan first.
pub struct EffectControlProducer {
    /// Session-stable track identity this channel addresses.
    pub track_id: Box<str>,
    /// Which rack the effect sits in.
    pub rack: EffectRack,
    /// Zero-based position of the effect **within its rack, in session declaration order**.
    ///
    /// This is the `effect_index` the `miso.command.v1` wire addresses, and it is derived from the
    /// normalized session model here rather than from `EffectPreparedSession::entries`, which is
    /// sorted by effect id.
    pub effect_index: u32,
    /// Session-stable effect instance identity.
    pub effect_id: Box<str>,
    /// The effect's declared parameter table, so an admitting host can map a wire `parameter_id`
    /// to the `parameter_index` the render side stages, and check the value's domain, without a
    /// second copy of the registry.
    pub descriptor: &'static EffectDescriptor,
    /// Checked bounded producer endpoint; unsupported prepared targets are refused before queue
    /// mutation, while ordinary records retain the queue's full-result retry semantics.
    producer: EffectControlProducerHandle,
    /// Optional off-audio-thread owner for a factory's prepared-target capability.
    ///
    /// The box is private so callers must use the checked transaction methods below; no raw
    /// producer or reusable "validated" flag can bypass candidate/revision admission.
    owner: Option<Box<EffectControlOwner>>,
}

/// Why an effect-control publication failed.
#[derive(Debug)]
pub enum EffectControlPushError {
    /// This owner has no prepared-target capability at this checkpoint. The record was not
    /// published and is returned for the caller's refusal report.
    Unsupported { record: EffectControlRecord },
    /// The bounded queue was full. The underlying queue retains its generation and full counter.
    Full(QueueFull<EffectControlRecord>),
}

/// Checked control-plane endpoint for one effect queue.
///
/// The underlying producer is intentionally private: callers cannot bypass target capability
/// checks or manufacture a production prepared-target route by setting a flag. Component tests
/// that need to exercise the target consumer construct an exclusively owned low-level queue and
/// lane directly.
pub struct EffectControlProducerHandle {
    inner: Producer<EffectControlRecord>,
    requires_prepared_targets: bool,
}

impl EffectControlProducerHandle {
    fn new(inner: Producer<EffectControlRecord>, requires_prepared_targets: bool) -> Self {
        Self {
            inner,
            requires_prepared_targets,
        }
    }

    /// Exact usable queue capacity.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Producer-side capacity snapshot used for complete target-prefix admission.
    #[must_use]
    pub fn available_capacity(&self) -> usize {
        self.inner.available_capacity()
    }

    /// Producer-local successful publication count.
    #[must_use]
    pub const fn success_count(&self) -> u64 {
        self.inner.success_count()
    }

    /// Producer-local full/overflow count.
    #[must_use]
    pub const fn full_count(&self) -> u64 {
        self.inner.full_count()
    }

    /// Check delivery capability without touching the queue, counters, or record ownership.
    ///
    /// A prepared-capable owner will eventually require all EQ parameter records to arrive with
    /// their companion targets. Assignment4 keeps production owners unregistered, so this path
    /// remains fail-closed until the owner transaction is available.
    pub fn preflight(&self, record: EffectControlRecord) -> Result<(), EffectControlPushError> {
        if matches!(record, EffectControlRecord::PreparedTarget(_))
            || (self.requires_prepared_targets
                && matches!(record, EffectControlRecord::Parameter { .. }))
        {
            return Err(EffectControlPushError::Unsupported { record });
        }
        Ok(())
    }

    /// Publish one record after checking whether this owner can deliver prepared targets.
    pub fn try_push(&mut self, record: EffectControlRecord) -> Result<(), EffectControlPushError> {
        self.preflight(record)?;
        self.inner
            .try_push(record)
            .map_err(EffectControlPushError::Full)
    }

    pub(crate) fn try_push_prepared(
        &mut self,
        target: effect_contract::PreparedEffectTarget,
    ) -> Result<(), EffectControlPushError> {
        self.inner
            .try_push(EffectControlRecord::PreparedTarget(target))
            .map_err(EffectControlPushError::Full)
    }
}

impl EffectControlProducer {
    /// Read-only checked endpoint for capacity and accounting inspection.
    #[must_use]
    pub fn producer(&self) -> &EffectControlProducerHandle {
        &self.producer
    }

    /// Publishes one ordinary semantic/observation record through the checked endpoint.
    pub fn try_push(&mut self, record: EffectControlRecord) -> Result<(), EffectControlPushError> {
        self.producer.try_push(record)
    }

    /// Checks one ordinary record without queue mutation.
    pub fn preflight(&self, record: EffectControlRecord) -> Result<(), EffectControlPushError> {
        self.producer.preflight(record)
    }

    #[must_use]
    pub fn capacity(&self) -> usize {
        self.producer.capacity()
    }

    #[must_use]
    pub const fn success_count(&self) -> u64 {
        self.producer.success_count()
    }

    #[must_use]
    pub const fn full_count(&self) -> u64 {
        self.producer.full_count()
    }

    /// Whether this effect has an opted-in prepared-target owner.
    #[must_use]
    pub fn has_owner(&self) -> bool {
        self.owner.is_some()
    }

    /// Borrows the checked owner for inspection of its committed state.
    #[must_use]
    pub fn owner(&self) -> Option<&EffectControlOwner> {
        self.owner.as_deref()
    }

    /// Starts the owner transaction at a checked committed revision.
    pub fn begin_owner(&mut self, base_revision: u64) -> Result<(), EffectControlOwnerError> {
        self.owner
            .as_deref_mut()
            .ok_or(EffectControlOwnerError::Unsupported)?
            .begin(base_revision)
    }

    /// Applies one checked semantic edit to the owner candidate.
    pub fn edit_owner(
        &mut self,
        parameter_index: u32,
        channel: ParameterChannel,
        value: f32,
    ) -> Result<(), EffectControlOwnerError> {
        self.owner
            .as_deref_mut()
            .ok_or(EffectControlOwnerError::Unsupported)?
            .edit(parameter_index, channel, value)
    }

    /// Publishes a validated target prefix and marks the owner ready for its one commit.
    pub fn publish_candidate_targets(
        &mut self,
        base_revision: u64,
        targets: &[effect_contract::PreparedEffectTarget],
    ) -> Result<(), EffectControlOwnerError> {
        let owner = self
            .owner
            .as_deref_mut()
            .ok_or(EffectControlOwnerError::Unsupported)?;
        owner.publish(&mut self.producer, base_revision, targets)
    }

    /// Preflights this owner's exact target prefix, including revision and complete queue room.
    /// The queue, candidate and owner phase remain unchanged.
    pub fn preflight_candidate_targets(
        &self,
        base_revision: u64,
        targets: &[effect_contract::PreparedEffectTarget],
    ) -> Result<(), EffectControlOwnerError> {
        let owner = self
            .owner
            .as_deref()
            .ok_or(EffectControlOwnerError::Unsupported)?;
        owner.preflight_publication(&self.producer, base_revision, targets)
    }

    /// Commits the candidate after successful complete publication.
    pub fn commit_owner(&mut self) -> Result<u64, EffectControlOwnerError> {
        self.owner
            .as_deref_mut()
            .ok_or(EffectControlOwnerError::Unsupported)?
            .commit()
    }

    /// Discards an open or poisoned candidate.
    pub fn discard_owner(&mut self) -> Result<(), EffectControlOwnerError> {
        self.owner
            .as_deref_mut()
            .ok_or(EffectControlOwnerError::Unsupported)?
            .discard()
    }
}

#[cfg(test)]
mod control_producer_tests {
    use super::{EffectControlProducerHandle, EffectControlPushError};
    use core::num::NonZeroUsize;
    use effect_contract::{EffectControlRecord, ParameterChannel, PreparedEffectTarget};
    use engine::realtime::{QueueGeneration, bounded_spsc};

    fn target(slot: u32) -> EffectControlRecord {
        EffectControlRecord::PreparedTarget(PreparedEffectTarget {
            slot,
            channel: ParameterChannel::Left,
            words: [slot; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
        })
    }

    fn parameter(value: f32) -> EffectControlRecord {
        EffectControlRecord::Parameter {
            parameter_index: 3,
            channel: ParameterChannel::Left,
            value,
        }
    }

    #[test]
    fn unsupported_target_is_returned_without_touching_queue_or_full_counter() {
        let (producer, consumer) = bounded_spsc::<EffectControlRecord>(
            NonZeroUsize::new(2).expect("capacity"),
            QueueGeneration(7),
        )
        .expect("queue");
        let mut producer = EffectControlProducerHandle::new(producer, false);
        producer
            .try_push(parameter(0.25))
            .expect("first queue slot");
        producer
            .try_push(parameter(0.5))
            .expect("second queue slot");
        let record = target(11);

        assert_eq!(producer.success_count(), 2);
        assert_eq!(producer.full_count(), 0);
        let refusal = producer.try_push(record).expect_err("unsupported target");
        match refusal {
            EffectControlPushError::Unsupported { record: returned } => {
                assert_eq!(returned, record);
            }
            EffectControlPushError::Full(_) => panic!("unsupported is distinct from full"),
        }
        assert_eq!(producer.success_count(), 2);
        assert_eq!(producer.full_count(), 0);
        assert_eq!(consumer.available_at_entry(), 2);
    }

    #[test]
    fn ordinary_semantic_record_publishes_and_full_refusal_preserves_original_record() {
        let (producer, mut consumer) = bounded_spsc::<EffectControlRecord>(
            NonZeroUsize::new(1).expect("capacity"),
            QueueGeneration(9),
        )
        .expect("queue");
        let mut producer = EffectControlProducerHandle::new(producer, false);
        let first = parameter(0.25);
        assert!(producer.try_push(first).is_ok());
        assert_eq!(producer.success_count(), 1);
        assert_eq!(producer.full_count(), 0);
        assert_eq!(consumer.try_pop().expect("semantic record"), first);

        let second = parameter(0.5);
        let third = parameter(0.75);
        producer.try_push(second).expect("room after pop");
        let refusal = producer.try_push(third).expect_err("full queue");
        match refusal {
            EffectControlPushError::Full(full) => {
                assert_eq!(full.value, third);
                assert_eq!(full.generation, QueueGeneration(9));
                assert_eq!(full.full_count, 1);
            }
            EffectControlPushError::Unsupported { .. } => panic!("ordinary record is supported"),
        }
        assert_eq!(producer.success_count(), 2);
        assert_eq!(producer.full_count(), 1);
        assert_eq!(consumer.available_at_entry(), 1);
        assert_eq!(
            consumer.try_pop().expect("retained semantic record"),
            second
        );
    }
}

#[cfg(test)]
mod owner_tests {
    use super::*;
    use crate::EffectControlOwnerPhase;
    use core::num::NonZeroUsize;
    use effect_contract::{
        EffectControlRecord, EffectPrepareError, NativeEffectFactory, ParameterChannel,
        PrepareEffectBankRequest, PreparedEffectTarget, PreparedNativeEffect,
        default_initial_values,
    };
    use engine::realtime::{QueueGeneration, bounded_spsc};
    use parametric_eq::{PARAMETRIC_EQ_DESCRIPTOR, ParametricEqFactory};
    use std::sync::Arc;

    /// Explicit test-only opt-in wrapper. Production ParametricEqFactory keeps its capability
    /// getter at None until cutover assignment 10.
    struct OptInEqFactory;

    impl NativeEffectFactory for OptInEqFactory {
        fn descriptor(&self) -> &'static EffectDescriptor {
            &PARAMETRIC_EQ_DESCRIPTOR
        }

        fn prepare(
            &self,
            request: PrepareEffectRequest<'_>,
        ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
            ParametricEqFactory.prepare(request)
        }

        fn target_preparation(
            &self,
        ) -> Option<&dyn effect_contract::NativeEffectTargetPreparation> {
            Some(&ParametricEqFactory)
        }

        fn bind_homogeneous_bank(
            &self,
            request: PrepareEffectBankRequest<'_>,
        ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
            ParametricEqFactory.bind_homogeneous_bank(request)
        }
    }

    fn preparation() -> EffectBankPreparation {
        EffectBankPreparation {
            sample_rate: 48_000,
            quantum: 128,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: PreparedPorts {
                sidechain: PreparedSidechainPort::None,
            },
            initial_values: default_initial_values(&PARAMETRIC_EQ_DESCRIPTOR).collect(),
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: u64::MAX,
                maximum_scratch_bytes: u64::MAX,
                maximum_automation_spans_per_block: 64,
            },
        }
    }

    fn owner_and_queue() -> (
        EffectControlOwner,
        EffectControlProducerHandle,
        engine::realtime::Consumer<EffectControlRecord>,
    ) {
        let preparation = preparation();
        let factory: Arc<dyn NativeEffectFactory> = Arc::new(OptInEqFactory);
        let owner = EffectControlOwner::new(factory, &preparation).expect("valid EQ owner");
        let (producer, consumer) = bounded_spsc(
            NonZeroUsize::new(12).expect("capacity"),
            QueueGeneration(31),
        )
        .expect("queue");
        (
            owner,
            EffectControlProducerHandle::new(producer, true),
            consumer,
        )
    }

    #[test]
    fn opted_in_eq_owner_prepares_both_and_commits_once() {
        let (mut owner, mut producer, mut consumer) = owner_and_queue();
        let initial = owner.committed().to_vec();
        owner.begin(0).expect("base revision");
        owner
            .edit(2, ParameterChannel::Both, 100.0)
            .expect("numeric Both edit");
        let mut targets = [PreparedEffectTarget {
            slot: 0,
            channel: ParameterChannel::Left,
            words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
        }; 12];
        let count = owner
            .prepare_targets_into(&mut targets)
            .expect("off-audio preparation");
        assert_eq!(count, 1, "equal Both lanes coalesce");
        assert_eq!(targets[0].channel, ParameterChannel::Both);
        assert_eq!(
            owner.committed(),
            initial,
            "preparation leaves committed rows unchanged"
        );
        let final_rows = owner.candidate().to_vec();
        let changed: Vec<_> = final_rows
            .iter()
            .filter(|row| row.parameter_index == 2)
            .collect();
        assert_eq!(changed.len(), 2);
        assert!(changed.iter().all(|row| row.value == 100.0));
        owner
            .publish(&mut producer, 0, &targets[..count])
            .expect("complete prefix publication");
        assert_eq!(
            owner.committed(),
            initial,
            "publication precedes shadow commit"
        );
        assert!(owner.edit(2, ParameterChannel::Both, 200.0).is_err());
        assert!(owner.discard().is_err());
        assert_eq!(owner.commit().expect("one commit"), 1);
        assert_eq!(owner.committed(), final_rows);
        assert!(owner.commit().is_err(), "repeat commit refused");
        assert_eq!(
            consumer.try_pop(),
            Ok(EffectControlRecord::PreparedTarget(targets[0]))
        );
    }

    #[test]
    fn invalid_edit_poison_survives_valid_overwrite_until_discard() {
        let (mut owner, _producer, _consumer) = owner_and_queue();
        owner.begin(0).expect("base revision");
        assert!(owner.edit(2, ParameterChannel::Left, f32::NAN).is_err());
        assert!(owner.edit(2, ParameterChannel::Left, 100.0).is_err());
        assert!(owner.prepare_targets_into(&mut []).is_err());
        owner.discard().expect("discard poisoned candidate");
        owner.begin(0).expect("restart at unchanged revision");
    }

    #[test]
    fn full_prefix_refusal_preserves_queue_and_candidate() {
        let (mut owner, mut producer, mut consumer) = owner_and_queue();
        for slot in 0..11 {
            producer
                .try_push(EffectControlRecord::Bypass(slot % 2 == 0))
                .expect("fill queue");
        }
        let initial = owner.committed().to_vec();
        owner.begin(0).expect("base revision");
        owner
            .edit(2, ParameterChannel::Both, 100.0)
            .expect("numeric edit");
        owner
            .edit(2, ParameterChannel::Right, 200.0)
            .expect("asymmetric target");
        let candidate = owner.candidate().to_vec();
        let mut targets = [PreparedEffectTarget {
            slot: 0,
            channel: ParameterChannel::Left,
            words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
        }; 12];
        let count = owner.prepare_targets_into(&mut targets).expect("prepare");
        assert_eq!(count, 2);
        assert_eq!(producer.available_capacity(), 1);
        let successes = producer.success_count();
        let full = producer.full_count();
        assert_eq!(
            owner.publish(&mut producer, 0, &targets[..count]),
            Err(EffectControlOwnerError::Capacity)
        );
        assert_eq!(owner.committed(), initial);
        assert_eq!(owner.candidate(), candidate);
        assert_eq!(owner.committed_revision(), 0);
        assert_eq!(consumer.available_at_entry(), 11);
        assert_eq!(
            (producer.success_count(), producer.full_count()),
            (successes, full)
        );
        assert_eq!(owner.phase(), EffectControlOwnerPhase::Open);
        for slot in 0..11 {
            assert_eq!(
                consumer.try_pop(),
                Ok(EffectControlRecord::Bypass(slot % 2 == 0))
            );
        }
        assert_eq!(consumer.available_at_entry(), 0);
    }

    #[test]
    fn stale_base_and_commit_before_publish_are_refused() {
        let (mut owner, mut producer, _consumer) = owner_and_queue();
        assert!(owner.begin(1).is_err(), "stale base");
        owner.begin(0).expect("current base");
        assert!(owner.commit().is_err(), "commit before publication");
        owner.edit(2, ParameterChannel::Both, 100.0).expect("edit");
        let mut targets = [PreparedEffectTarget {
            slot: 0,
            channel: ParameterChannel::Left,
            words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
        }; 12];
        let count = owner.prepare_targets_into(&mut targets).expect("prepare");
        owner
            .publish(&mut producer, 1, &targets[..count])
            .expect_err("stale publication");
        assert_eq!(owner.phase(), EffectControlOwnerPhase::Open);
    }
}

/// Attach one bounded live-console control channel to every prepared effect of the session.
///
/// # The capacity rule that makes the render-side drain exact
///
/// A channel is prepared at `min(depth, automation_capacity)` records. Each drained
/// [`EffectControlRecord::Parameter`] produces at most one span and duplicates collapse, so a
/// full drain can never stage more spans than the effect's own
/// [`PreparedEffectMetadata::automation_capacity`](effect_contract::PreparedEffectMetadata).
/// "The staging window cannot overflow" is therefore an invariant of preparation, not a runtime
/// check on the render thread.
///
/// # Errors
///
/// `effect.control.prepare` if a bounded queue cannot be built, and
/// `effect.control.capacity` if an effect declares a zero automation capacity, which no launch
/// effect does and which would leave the channel unable to deliver anything.
pub fn attach_effect_console(
    prepared: &mut EffectPreparedSession,
    depth: NonZeroUsize,
) -> Result<Vec<EffectControlProducer>, EffectDiagnosticSet> {
    let declared = declared_effect_indices(&prepared.session);
    let mut producers = Vec::with_capacity(prepared.entries.len());
    let mut diagnostics = Vec::new();
    for entry in &mut prepared.entries {
        let path = format!(
            "$.tracks[id={}].effects[id={}]",
            entry.track_id, entry.effect_id
        );
        let Some(capacity) = NonZeroUsize::new(entry.metadata.automation_capacity as usize) else {
            diagnostics.push(EffectDiagnostic {
                code: "effect.control.capacity",
                path,
            });
            continue;
        };
        let Some(&effect_index) =
            declared.get(&(entry.track_id.clone(), entry.rack, entry.effect_id.clone()))
        else {
            diagnostics.push(EffectDiagnostic {
                code: "effect.control.prepare",
                path,
            });
            continue;
        };
        let Ok((producer, consumer)) =
            bounded_spsc::<EffectControlRecord>(depth.min(capacity), QueueGeneration(0))
        else {
            diagnostics.push(EffectDiagnostic {
                code: "effect.control.prepare",
                path,
            });
            continue;
        };
        let target_capable = entry.factory.target_preparation().is_some();
        let owner = if target_capable {
            match EffectControlOwner::new(Arc::clone(&entry.factory), &entry.bank_preparation) {
                Ok(owner) => Some(Box::new(owner)),
                Err(_) => {
                    diagnostics.push(EffectDiagnostic {
                        code: "effect.control.owner",
                        path: format!(
                            "$.tracks[id={}].effects[id={}]",
                            entry.track_id, entry.effect_id
                        ),
                    });
                    continue;
                }
            }
        } else {
            None
        };
        producers.push(EffectControlProducer {
            track_id: entry.track_id.as_str().into(),
            rack: entry.rack,
            effect_index,
            effect_id: entry.effect_id.as_str().into(),
            descriptor: entry.factory.descriptor(),
            producer: EffectControlProducerHandle::new(producer, owner.is_some()),
            owner,
        });
        entry.control = Some(Box::new(if target_capable {
            EffectControlLane::new_with_target_staging(consumer, entry.bank_preparation.bypass)
        } else {
            EffectControlLane::new(consumer, entry.bank_preparation.bypass)
        }));
    }
    if diagnostics.is_empty() {
        Ok(producers)
    } else {
        Err(EffectDiagnosticSet::sorted(diagnostics))
    }
}

/// The control-side reader half of one prepared effect instance's observation taps (issue #143).
///
/// Addressed exactly as an [`EffectControlProducer`] is -- by `(track_id, rack, effect_index)` --
/// because a subscription and the parameter commands it correlates with address the same instance
/// through the same numbers. `readers[i]` belongs to `descriptor.observations[i]`.
pub struct EffectObservationHandle {
    /// Normalized track identity this instance belongs to.
    pub track_id: Box<str>,
    /// Which rack of that track.
    pub rack: EffectRack,
    /// Declared position within the rack, in session declaration order.
    pub effect_index: u32,
    /// The instance's session-declared identifier.
    pub effect_id: Box<str>,
    /// The effect's declared menu, so an admitting host maps a wire `tap_id` to a `tap_index` and
    /// checks the cost class without a second copy of the registry.
    pub descriptor: &'static EffectDescriptor,
    /// One reader per declared tap, in declaration order.
    pub readers: Box<[ObservationReader]>,
}

/// Attach observation capacity to every prepared effect that declares at least one tap.
///
/// # Level 1 of the two-level zero (issue #143 D3)
///
/// This function is the **only** thing that creates an [`ObservationLane`]. A session whose
/// console request named no observation capacity never calls it, so its compiled plan contains no
/// lane, no accumulator and no conflating cell -- not a disabled one, none. That is what makes
/// "observation off costs nothing" an identity rather than a claim, and it is what
/// `observation_retained_bytes == 0` reports.
///
/// An effect that declares no tap gets no lane either, for the same reason: `miso.delay` has
/// nothing to observe, so it carries nothing.
///
/// `window_blocks` is the plan's default window length in render blocks. It is the *meter* window,
/// derived by the host from the same `console_meter_blocks` the peak meters use, so a gain-reduction
/// value and the peak beside it in one `miso.meter.v1` frame describe the same span of samples.
///
/// # Errors
///
/// `effect.observation.prepare` if an instance cannot be located in the declared order, and
/// `effect.observation.taps` if an effect declares more taps than the request's cap allows.
pub fn attach_effect_observation(
    prepared: &mut EffectPreparedSession,
    maximum_taps: u32,
    window_blocks: u32,
) -> Result<Vec<EffectObservationHandle>, EffectDiagnosticSet> {
    let declared = declared_effect_indices(&prepared.session);
    let mut handles = Vec::new();
    let mut diagnostics = Vec::new();
    for entry in &mut prepared.entries {
        let descriptor = entry.factory.descriptor();
        if descriptor.observations.is_empty() {
            continue;
        }
        let path = format!(
            "$.tracks[id={}].effects[id={}]",
            entry.track_id, entry.effect_id
        );
        if descriptor.observations.len() > maximum_taps as usize {
            diagnostics.push(EffectDiagnostic {
                code: "effect.observation.taps",
                path,
            });
            continue;
        }
        let Some(&effect_index) =
            declared.get(&(entry.track_id.clone(), entry.rack, entry.effect_id.clone()))
        else {
            diagnostics.push(EffectDiagnostic {
                code: "effect.observation.prepare",
                path,
            });
            continue;
        };
        let mut publishers = Vec::with_capacity(descriptor.observations.len());
        let mut readers = Vec::with_capacity(descriptor.observations.len());
        for _ in descriptor.observations {
            let (publisher, reader) = observation_slot();
            publishers.push(publisher);
            readers.push(reader);
        }
        let Some(lane) = ObservationLane::new(descriptor.observations, publishers, window_blocks)
        else {
            diagnostics.push(EffectDiagnostic {
                code: "effect.observation.prepare",
                path,
            });
            continue;
        };
        handles.push(EffectObservationHandle {
            track_id: entry.track_id.as_str().into(),
            rack: entry.rack,
            effect_index,
            effect_id: entry.effect_id.as_str().into(),
            descriptor,
            readers: readers.into_boxed_slice(),
        });
        entry.observation = Some(Box::new(lane));
    }
    if diagnostics.is_empty() {
        Ok(handles)
    } else {
        Err(EffectDiagnosticSet::sorted(diagnostics))
    }
}

/// Declared position within each `(track, rack)`, from the normalized model.
///
/// The same order the `miso.command.v1` `effect_index` names and the same order the browser host
/// counts, extracted once so the console attach and the observation attach cannot disagree about
/// what "effect 2 of the dynamic rack" means.
fn declared_effect_indices(
    session: &CompiledSession,
) -> BTreeMap<(String, EffectRack, String), u32> {
    let mut declared: BTreeMap<(String, EffectRack, String), u32> = BTreeMap::new();
    for track in &session.normalized_model().tracks {
        for (rack, effects) in [
            (EffectRack::Simd1, &track.simd1.effects),
            (EffectRack::Dynamic, &track.dynamic.effects),
            (EffectRack::Simd2, &track.simd2.effects),
        ] {
            for (index, effect) in effects.iter().enumerate() {
                declared.insert(
                    (track.id.to_string(), rack, effect.id.to_string()),
                    index as u32,
                );
            }
        }
    }
    declared
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
