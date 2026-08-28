use crate::{
    BoundEffectDescriptorWire, EFFECT_STATE_UNAVAILABLE_INDEX, EFFECT_STATE_UNAVAILABLE_OFFSET,
    EffectDescriptorIdentity, EffectStateDiagnostic as Diagnostic,
    EffectStateDiagnosticCode as Code,
};
use miso_engine_core::{SampleRateHz, is_launch_sample_rate};
use miso_engine_effect_contract::{
    EffectDescriptor, EffectId, EffectQuality, InitialParameterValue, LinkMode, ParameterChannel,
    ParameterChannelPolicy, ParameterDomain, PrepareEffectRequest, PreparedEffectMetadata,
    PreparedPorts, PreparedSidechainPort, StatePayloadSizes, TailSamples,
};
use sha2::{Digest, Sha256};

const MAGIC: &[u8; 8] = b"MISOEFST";
const VERSION: u16 = 1;
const HEADER_BYTES: usize = 224;
const INITIAL_BYTES: usize = 16;
const DIGEST_DOMAIN: &[u8] = b"miso.engine.effect-state.current-layout.v1\0";

pub const EFFECT_STATE_BUFFER_ENVELOPE_OUTPUT: u32 = 1;
pub const EFFECT_STATE_BUFFER_PAYLOAD_SCRATCH: u32 = 2;
pub const EFFECT_STATE_BUFFER_INITIAL_VALUE_SCRATCH: u32 = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectStateSelector {
    descriptor_identity: EffectDescriptorIdentity,
    state_layout_version: u32,
}

impl EffectStateSelector {
    pub const fn descriptor_identity(self) -> EffectDescriptorIdentity {
        self.descriptor_identity
    }

    pub const fn state_layout_version(self) -> u32 {
        self.state_layout_version
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum EffectStateMigrationEdgeError {
    NonAdjacentLayout = 1,
    UnchangedIdentity = 2,
    EffectOrContractMismatch = 3,
    IncompatibleReplayDescriptor = 4,
}

#[derive(Clone, Copy, Debug)]
pub struct EffectStateDescriptorProvenance {
    descriptor: &'static EffectDescriptor,
    identity: EffectDescriptorIdentity,
}

impl PartialEq for EffectStateDescriptorProvenance {
    fn eq(&self, other: &Self) -> bool {
        core::ptr::eq(self.descriptor, other.descriptor) && self.identity == other.identity
    }
}

impl Eq for EffectStateDescriptorProvenance {}

/// An adjacent, replay-compatible historical-to-target descriptor edge.
///
/// Private fields ensure identities, layouts, and static descriptor provenance can only originate
/// from accepted descriptor-wire bindings.
#[derive(Clone, Copy, Debug)]
pub struct BoundEffectStateMigrationEdge<'wire> {
    source: BoundEffectDescriptorWire<'wire>,
    target: BoundEffectDescriptorWire<'wire>,
    source_selector: EffectStateSelector,
    target_selector: EffectStateSelector,
}

impl<'wire> BoundEffectStateMigrationEdge<'wire> {
    pub const fn source_bound(self) -> BoundEffectDescriptorWire<'wire> {
        self.source
    }

    pub const fn target_bound(self) -> BoundEffectDescriptorWire<'wire> {
        self.target
    }

    pub const fn source_selector(self) -> EffectStateSelector {
        self.source_selector
    }

    pub const fn target_selector(self) -> EffectStateSelector {
        self.target_selector
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectStateLimits {
    pub maximum_descriptor_bytes: u64,
    pub maximum_envelope_bytes: u64,
    pub maximum_payload_bytes: u64,
    pub maximum_initial_values: u32,
}

impl Default for EffectStateLimits {
    fn default() -> Self {
        Self {
            maximum_descriptor_bytes: 4_194_304,
            maximum_envelope_bytes: 268_435_456,
            maximum_payload_bytes: 134_217_728,
            maximum_initial_values: 4_096,
        }
    }
}

/// Complete borrowed control-plane replay of an accepted prepare request.
#[derive(Clone, Copy, Debug)]
pub struct EffectStateReplayView<'a> {
    pub effect_id: EffectId,
    pub request: PrepareEffectRequest<'a>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectStateRequirements {
    pub envelope_bytes: u64,
    pub payload_snapshot_scratch_bytes: u64,
    pub initial_value_scratch_slots: u32,
}

#[derive(Clone, Copy, Debug)]
struct Layout {
    total: usize,
    effect_id_end: usize,
    sidechain_id_start: usize,
    sidechain_id_end: usize,
    initial_start: usize,
    initial_bytes: u32,
    common_start: usize,
    left_start: usize,
    right_start: usize,
}

fn diagnostic(code: Code, byte_offset: usize, item_index: Option<usize>) -> Diagnostic {
    Diagnostic::new(
        code,
        0,
        item_index
            .and_then(|value| u32::try_from(value).ok())
            .unwrap_or(EFFECT_STATE_UNAVAILABLE_INDEX),
        u64::try_from(byte_offset).unwrap_or(EFFECT_STATE_UNAVAILABLE_OFFSET),
    )
}

fn unavailable(code: Code, detail: u32) -> Diagnostic {
    Diagnostic::new(
        code,
        detail,
        EFFECT_STATE_UNAVAILABLE_INDEX,
        EFFECT_STATE_UNAVAILABLE_OFFSET,
    )
}

fn limit(required_bytes: u64) -> Diagnostic {
    let mut value = unavailable(Code::Limit, 0);
    value.required_bytes = required_bytes;
    value
}

fn overflow(byte_offset: usize) -> Diagnostic {
    diagnostic(Code::Overflow, byte_offset, None)
}

fn checked_add(left: usize, right: usize, byte_offset: usize) -> Result<usize, Diagnostic> {
    left.checked_add(right).ok_or_else(|| overflow(byte_offset))
}

fn checked_mul(left: usize, right: usize, byte_offset: usize) -> Result<usize, Diagnostic> {
    left.checked_mul(right).ok_or_else(|| overflow(byte_offset))
}

fn checked_usize(value: u64, byte_offset: usize) -> Result<usize, Diagnostic> {
    let value = usize::try_from(value).map_err(|_| overflow(byte_offset))?;
    if value > isize::MAX as usize {
        return Err(overflow(byte_offset));
    }
    Ok(value)
}

fn validate_limits(limits: EffectStateLimits) -> Result<(), Diagnostic> {
    for value in [
        limits.maximum_descriptor_bytes,
        limits.maximum_envelope_bytes,
        limits.maximum_payload_bytes,
        u64::from(limits.maximum_initial_values),
    ] {
        if value == 0 {
            return Err(limit(1));
        }
    }
    checked_usize(limits.maximum_descriptor_bytes, 0)?;
    checked_usize(limits.maximum_envelope_bytes, 0)?;
    checked_usize(limits.maximum_payload_bytes, 0)?;
    Ok(())
}

fn metadata_mismatch(detail: u32) -> Diagnostic {
    unavailable(Code::Metadata, detail)
}

fn sidechain_parts(ports: PreparedPorts) -> (u32, &'static str, bool) {
    match ports.sidechain {
        PreparedSidechainPort::None => (0, "", false),
        PreparedSidechainPort::Unconnected { id, required } => (1, id.as_str(), required),
        PreparedSidechainPort::Connected { id, required } => (2, id.as_str(), required),
    }
}

fn optional_float_bits_equal(left: Option<f32>, right: Option<f32>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn parameters_compatible(
    left: &'static EffectDescriptor,
    right: &'static EffectDescriptor,
) -> bool {
    left.parameters.len() == right.parameters.len()
        && left
            .parameters
            .iter()
            .zip(right.parameters)
            .all(|(left, right)| {
                left.id == right.id
                    && left.display_name == right.display_name
                    && left.display_unit == right.display_unit
                    && left.unit == right.unit
                    && left.domain == right.domain
                    && optional_float_bits_equal(left.minimum, right.minimum)
                    && optional_float_bits_equal(left.maximum, right.maximum)
                    && left.default_value.to_bits() == right.default_value.to_bits()
                    && left.mapping == right.mapping
                    && left.automation_rate == right.automation_rate
                    && left.channel_policy == right.channel_policy
                    && left.smoothing == right.smoothing
                    && left.smoothing_samples == right.smoothing_samples
                    && left.readable == right.readable
                    && left.automatable == right.automatable
                    && left.enum_choices.len() == right.enum_choices.len()
                    && left
                        .enum_choices
                        .iter()
                        .zip(right.enum_choices)
                        .all(|(left, right)| {
                            left.value.to_bits() == right.value.to_bits()
                                && left.label == right.label
                        })
            })
}

fn qualities_compatible(left: &'static EffectDescriptor, right: &'static EffectDescriptor) -> bool {
    left.qualities.len() == right.qualities.len()
        && left
            .qualities
            .iter()
            .zip(right.qualities)
            .all(|(left, right)| {
                left.quality == right.quality
                    && left.sample_rate == right.sample_rate
                    && left.latency == right.latency
                    && left.tail == right.tail
                    && left.scratch_fixed_bytes == right.scratch_fixed_bytes
                    && left.scratch_bytes_per_frame == right.scratch_bytes_per_frame
            })
}

pub fn effect_state_bound_selector(bound: BoundEffectDescriptorWire<'_>) -> EffectStateSelector {
    EffectStateSelector {
        descriptor_identity: bound.identity(),
        state_layout_version: bound.descriptor().state_layout_version,
    }
}

pub fn effect_state_descriptor_provenance(
    bound: BoundEffectDescriptorWire<'_>,
) -> EffectStateDescriptorProvenance {
    EffectStateDescriptorProvenance {
        descriptor: bound.descriptor(),
        identity: bound.identity(),
    }
}

pub fn bind_effect_state_migration_edge<'wire>(
    source: BoundEffectDescriptorWire<'wire>,
    target: BoundEffectDescriptorWire<'wire>,
) -> Result<BoundEffectStateMigrationEdge<'wire>, EffectStateMigrationEdgeError> {
    let source_descriptor = source.descriptor();
    let target_descriptor = target.descriptor();
    let adjacent = source_descriptor
        .state_layout_version
        .checked_add(1)
        .is_some_and(|version| version == target_descriptor.state_layout_version);
    if !adjacent {
        return Err(EffectStateMigrationEdgeError::NonAdjacentLayout);
    }
    if source.identity() == target.identity() {
        return Err(EffectStateMigrationEdgeError::UnchangedIdentity);
    }
    if source_descriptor.id != target_descriptor.id
        || source_descriptor.contract_major != target_descriptor.contract_major
        || source_descriptor.contract_minor != target_descriptor.contract_minor
    {
        return Err(EffectStateMigrationEdgeError::EffectOrContractMismatch);
    }
    if source_descriptor.display_name != target_descriptor.display_name
        || source_descriptor.supported_link_modes != target_descriptor.supported_link_modes
        || !parameters_compatible(source_descriptor, target_descriptor)
        || source_descriptor.ports != target_descriptor.ports
        || !qualities_compatible(source_descriptor, target_descriptor)
    {
        return Err(EffectStateMigrationEdgeError::IncompatibleReplayDescriptor);
    }
    Ok(BoundEffectStateMigrationEdge {
        source,
        target,
        source_selector: effect_state_bound_selector(source),
        target_selector: effect_state_bound_selector(target),
    })
}

fn parameter_value_valid(
    parameter: &miso_engine_effect_contract::ParameterDescriptor,
    value: f32,
) -> bool {
    if !value.is_finite() || value.to_bits() == (-0.0f32).to_bits() {
        return false;
    }
    match parameter.domain {
        ParameterDomain::Continuous => parameter
            .minimum
            .zip(parameter.maximum)
            .is_some_and(|(minimum, maximum)| value >= minimum && value <= maximum),
        ParameterDomain::Boolean => matches!(value.to_bits(), 0 | 0x3f80_0000),
        ParameterDomain::Enumeration => parameter
            .enum_choices
            .iter()
            .any(|choice| choice.value.to_bits() == value.to_bits()),
    }
}

fn validate_initial_values_no_alloc(
    descriptor: &'static EffectDescriptor,
    values: &[InitialParameterValue],
) -> Result<(), Diagnostic> {
    let mut cursor = 0usize;
    for (parameter_index, parameter) in descriptor.parameters.iter().enumerate() {
        let channels: &[ParameterChannel] = match parameter.channel_policy {
            ParameterChannelPolicy::Shared => &[ParameterChannel::Both],
            ParameterChannelPolicy::PerLane => &[ParameterChannel::Left, ParameterChannel::Right],
        };
        for channel in channels {
            let Some(value) = values.get(cursor) else {
                return Err(unavailable(Code::InitialValues, 0));
            };
            if value.parameter_index != parameter_index as u32
                || value.channel != *channel
                || !parameter_value_valid(parameter, value.value)
            {
                let mut diagnostic = unavailable(Code::InitialValues, 0);
                diagnostic.item_index = cursor as u32;
                return Err(diagnostic);
            }
            cursor += 1;
        }
    }
    if cursor != values.len() {
        return Err(unavailable(Code::InitialValues, 0));
    }
    Ok(())
}

fn derive_expected_metadata_without_revalidation(
    descriptor: &'static EffectDescriptor,
    request: PrepareEffectRequest<'_>,
) -> Result<PreparedEffectMetadata, Diagnostic> {
    if !is_launch_sample_rate(SampleRateHz(request.sample_rate)) {
        return Err(metadata_mismatch(4));
    }
    if request.quantum == 0 {
        return Err(metadata_mismatch(5));
    }
    let quality = descriptor
        .qualities
        .iter()
        .find(|quality| {
            quality.quality == request.quality && quality.sample_rate == request.sample_rate
        })
        .ok_or_else(|| metadata_mismatch(6))?;
    if !descriptor.supported_link_modes.contains(request.link_mode) {
        return Err(metadata_mismatch(8));
    }
    match request.ports.sidechain {
        PreparedSidechainPort::None => {
            if descriptor
                .ports
                .iter()
                .any(|port| port.role == miso_engine_effect_contract::PortRole::SidechainInput)
            {
                return Err(metadata_mismatch(9));
            }
        }
        PreparedSidechainPort::Unconnected { id, required } => {
            if required
                || !descriptor.ports.iter().any(|port| {
                    port.role == miso_engine_effect_contract::PortRole::SidechainInput
                        && port.id == id
                        && port.required == required
                })
            {
                return Err(metadata_mismatch(9));
            }
        }
        PreparedSidechainPort::Connected { id, required } => {
            if !descriptor.ports.iter().any(|port| {
                port.role == miso_engine_effect_contract::PortRole::SidechainInput
                    && port.id == id
                    && port.required == required
            }) {
                return Err(metadata_mismatch(9));
            }
        }
    }
    let state_bytes = quality.maximum_state.total().ok_or_else(|| overflow(216))?;
    let scratch_bytes = quality
        .scratch_bytes_per_frame
        .checked_mul(u64::from(request.quantum))
        .and_then(|bytes| quality.scratch_fixed_bytes.checked_add(bytes))
        .ok_or_else(|| overflow(176))?;
    checked_usize(state_bytes, 216)?;
    checked_usize(scratch_bytes, 176)?;
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

fn validate_request_limits(
    metadata: PreparedEffectMetadata,
    request: PrepareEffectRequest<'_>,
) -> Result<(), Diagnostic> {
    let state_bytes = metadata.state_sizes.total().ok_or_else(|| overflow(216))?;
    if request.limits.maximum_total_state_bytes == 0
        || request.limits.maximum_scratch_bytes == 0
        || request.limits.maximum_automation_spans_per_block == 0
        || state_bytes > request.limits.maximum_total_state_bytes
        || metadata.scratch_bytes > request.limits.maximum_scratch_bytes
    {
        return Err(metadata_mismatch(15));
    }
    Ok(())
}

fn validate_replay(
    bound: BoundEffectDescriptorWire<'_>,
    replay: EffectStateReplayView<'_>,
) -> Result<PreparedEffectMetadata, Diagnostic> {
    let descriptor = bound.descriptor();
    if replay.effect_id != descriptor.id {
        return Err(metadata_mismatch(1));
    }
    let expected = derive_expected_metadata_without_revalidation(descriptor, replay.request)?;
    validate_request_limits(expected, replay.request)?;
    validate_initial_values_no_alloc(descriptor, replay.request.initial_values)?;
    Ok(expected)
}

pub fn effect_state_expected_metadata(
    bound: BoundEffectDescriptorWire<'_>,
    replay: EffectStateReplayView<'_>,
) -> Result<PreparedEffectMetadata, Diagnostic> {
    validate_replay(bound, replay)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectStateDerivedResources {
    pub state_sizes: StatePayloadSizes,
    pub scratch_bytes: u64,
    pub automation_capacity: u32,
}

pub fn effect_state_derived_resources(
    bound: BoundEffectDescriptorWire<'_>,
    request: PrepareEffectRequest<'_>,
) -> Result<EffectStateDerivedResources, Diagnostic> {
    if !is_launch_sample_rate(SampleRateHz(request.sample_rate)) {
        return Err(metadata_mismatch(4));
    }
    if request.quantum == 0 {
        return Err(metadata_mismatch(5));
    }
    let quality = bound
        .descriptor()
        .qualities
        .iter()
        .find(|quality| {
            quality.quality == request.quality && quality.sample_rate == request.sample_rate
        })
        .ok_or_else(|| metadata_mismatch(6))?;
    let state_bytes = quality.maximum_state.total().ok_or_else(|| overflow(216))?;
    let scratch_bytes = quality
        .scratch_bytes_per_frame
        .checked_mul(u64::from(request.quantum))
        .and_then(|bytes| quality.scratch_fixed_bytes.checked_add(bytes))
        .ok_or_else(|| overflow(176))?;
    checked_usize(state_bytes, 216)?;
    checked_usize(scratch_bytes, 176)?;
    Ok(EffectStateDerivedResources {
        state_sizes: quality.maximum_state,
        scratch_bytes,
        automation_capacity: request.limits.maximum_automation_spans_per_block,
    })
}

pub fn validate_effect_state_metadata(
    bound: BoundEffectDescriptorWire<'_>,
    replay: EffectStateReplayView<'_>,
    actual: PreparedEffectMetadata,
) -> Result<(), Diagnostic> {
    let descriptor = bound.descriptor();
    if replay.effect_id != descriptor.id
        || !core::ptr::eq(actual.descriptor, descriptor)
        || actual.descriptor.id != descriptor.id
    {
        return Err(metadata_mismatch(1));
    }
    if actual.descriptor.contract_major != descriptor.contract_major
        || actual.descriptor.contract_minor != descriptor.contract_minor
    {
        return Err(metadata_mismatch(2));
    }
    if actual.descriptor.state_layout_version != descriptor.state_layout_version {
        return Err(metadata_mismatch(3));
    }
    let expected = derive_expected_metadata_without_revalidation(descriptor, replay.request)?;
    for (detail, equal) in [
        (4, actual.sample_rate == expected.sample_rate),
        (5, actual.quantum == expected.quantum),
        (6, actual.quality == expected.quality),
        (7, actual.bypass == expected.bypass),
        (8, actual.link_mode == expected.link_mode),
        (9, actual.ports == expected.ports),
        (10, actual.latency == expected.latency),
        (11, actual.tail == expected.tail),
        (12, actual.state_sizes == expected.state_sizes),
        (13, actual.scratch_bytes == expected.scratch_bytes),
        (
            14,
            actual.automation_capacity == expected.automation_capacity,
        ),
    ] {
        if !equal {
            return Err(metadata_mismatch(detail));
        }
    }
    validate_request_limits(expected, replay.request)?;
    validate_initial_values_no_alloc(descriptor, replay.request.initial_values)
}

fn authoring_layout(
    bound: BoundEffectDescriptorWire<'_>,
    replay: EffectStateReplayView<'_>,
    limits: EffectStateLimits,
) -> Result<(Layout, PreparedEffectMetadata), Diagnostic> {
    validate_limits(limits)?;
    let descriptor_bytes = u64::try_from(bound.wire().len()).map_err(|_| overflow(0))?;
    if descriptor_bytes > limits.maximum_descriptor_bytes {
        return Err(limit(descriptor_bytes));
    }
    let initial_count = replay.request.initial_values.len();
    if initial_count > usize::try_from(limits.maximum_initial_values).map_err(|_| overflow(132))? {
        return Err(limit(
            u64::try_from(initial_count).map_err(|_| overflow(132))?,
        ));
    }
    let metadata = validate_replay(bound, replay)?;
    let payload = metadata.state_sizes.total().ok_or_else(|| overflow(216))?;
    if payload > limits.maximum_payload_bytes {
        return Err(limit(payload));
    }
    let effect_id = replay.effect_id.as_str().len();
    let (_, sidechain_id, _) = sidechain_parts(replay.request.ports);
    let effect_id_end = checked_add(HEADER_BYTES, effect_id, 124)?;
    let sidechain_id_start = effect_id_end;
    let sidechain_id_end = checked_add(sidechain_id_start, sidechain_id.len(), 128)?;
    let initial_start = checked_add(sidechain_id_end, 7, 188)? & !7;
    let initial_bytes_usize = checked_mul(initial_count, INITIAL_BYTES, 188)?;
    let initial_bytes = u32::try_from(initial_bytes_usize).map_err(|_| overflow(188))?;
    let initial_end = checked_add(initial_start, initial_bytes_usize, 188)?;
    let common_start = initial_end;
    let left_start = checked_add(
        common_start,
        metadata.state_sizes.common_bytes as usize,
        216,
    )?;
    let right_start = checked_add(left_start, metadata.state_sizes.left_bytes as usize, 216)?;
    let total = checked_add(right_start, metadata.state_sizes.right_bytes as usize, 216)?;
    let total_u64 = u64::try_from(total).map_err(|_| overflow(16))?;
    if total > isize::MAX as usize || total_u64 > limits.maximum_envelope_bytes {
        return Err(limit(total_u64));
    }
    Ok((
        Layout {
            total,
            effect_id_end,
            sidechain_id_start,
            sidechain_id_end,
            initial_start,
            initial_bytes,
            common_start,
            left_start,
            right_start,
        },
        metadata,
    ))
}

pub fn effect_state_requirements(
    bound: BoundEffectDescriptorWire<'_>,
    replay: EffectStateReplayView<'_>,
    limits: EffectStateLimits,
) -> Result<EffectStateRequirements, Diagnostic> {
    let (layout, metadata) = authoring_layout(bound, replay, limits)?;
    Ok(EffectStateRequirements {
        envelope_bytes: u64::try_from(layout.total).map_err(|_| overflow(16))?,
        payload_snapshot_scratch_bytes: metadata
            .state_sizes
            .total()
            .ok_or_else(|| overflow(216))?,
        initial_value_scratch_slots: u32::try_from(replay.request.initial_values.len())
            .map_err(|_| overflow(132))?,
    })
}

fn write_u16(output: &mut [u8], offset: usize, value: u16) {
    output[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn write_u32(output: &mut [u8], offset: usize, value: u32) {
    output[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn write_u64(output: &mut [u8], offset: usize, value: u64) {
    output[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

fn state_digest(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(DIGEST_DOMAIN);
    hasher.update((bytes.len() as u64).to_le_bytes());
    hasher.update(&bytes[..56]);
    hasher.update([0; 32]);
    hasher.update(&bytes[88..]);
    hasher.finalize().into()
}

pub fn encode_effect_state(
    bound: BoundEffectDescriptorWire<'_>,
    replay: EffectStateReplayView<'_>,
    common: &[u8],
    left: &[u8],
    right: &[u8],
    limits: EffectStateLimits,
    output: &mut [u8],
) -> Result<u64, Diagnostic> {
    let (layout, metadata) = authoring_layout(bound, replay, limits)?;
    let sizes = metadata.state_sizes;
    if common.len() != sizes.common_bytes as usize
        || left.len() != sizes.left_bytes as usize
        || right.len() != sizes.right_bytes as usize
    {
        return Err(unavailable(Code::Payload, 0));
    }
    if output.len() < layout.total {
        return Err(Diagnostic::buffer_too_small(
            EFFECT_STATE_BUFFER_ENVELOPE_OUTPUT,
            layout.total as u64,
        ));
    }

    let output = &mut output[..layout.total];
    let total_bytes = u64::try_from(layout.total).map_err(|_| overflow(16))?;
    output.fill(0);
    output[..8].copy_from_slice(MAGIC);
    write_u16(output, 8, VERSION);
    write_u16(output, 10, HEADER_BYTES as u16);
    write_u64(output, 16, total_bytes);
    output[24..56].copy_from_slice(bound.identity().as_bytes());
    write_u16(output, 88, metadata.descriptor.contract_major);
    write_u16(output, 90, metadata.descriptor.contract_minor);
    write_u32(output, 92, metadata.descriptor.state_layout_version);
    write_u32(output, 96, replay.request.sample_rate);
    write_u32(output, 100, replay.request.quantum);
    write_u32(output, 104, replay.request.quality as u32);
    write_u32(output, 108, u32::from(replay.request.bypass));
    write_u32(output, 112, replay.request.link_mode as u32);
    let (sidechain_kind, sidechain_id, sidechain_required) = sidechain_parts(replay.request.ports);
    write_u32(output, 116, sidechain_kind);
    write_u32(output, 120, u32::from(sidechain_required));
    write_u32(output, 124, replay.effect_id.as_str().len() as u32);
    write_u32(output, 128, sidechain_id.len() as u32);
    write_u32(output, 132, replay.request.initial_values.len() as u32);
    write_u64(output, 136, metadata.latency.0);
    match metadata.tail {
        TailSamples::Finite(samples) => {
            write_u32(output, 144, 1);
            write_u64(output, 152, samples);
        }
        TailSamples::Infinite => write_u32(output, 144, 2),
    }
    write_u32(output, 160, sizes.common_bytes);
    write_u32(output, 164, sizes.left_bytes);
    write_u32(output, 168, sizes.right_bytes);
    write_u64(output, 176, metadata.scratch_bytes);
    write_u32(output, 184, metadata.automation_capacity);
    write_u32(output, 188, layout.initial_bytes);
    write_u64(output, 192, replay.request.limits.maximum_total_state_bytes);
    write_u64(output, 200, replay.request.limits.maximum_scratch_bytes);
    write_u32(
        output,
        208,
        replay.request.limits.maximum_automation_spans_per_block,
    );
    write_u64(
        output,
        216,
        sizes.total().expect("validated state-size sum"),
    );
    output[HEADER_BYTES..layout.effect_id_end]
        .copy_from_slice(replay.effect_id.as_str().as_bytes());
    output[layout.sidechain_id_start..layout.sidechain_id_end]
        .copy_from_slice(sidechain_id.as_bytes());
    for (index, value) in replay.request.initial_values.iter().enumerate() {
        let record = layout.initial_start + index * INITIAL_BYTES;
        write_u32(output, record, value.parameter_index);
        write_u32(output, record + 4, value.channel as u32);
        write_u32(output, record + 8, value.value.to_bits());
    }
    output[layout.common_start..layout.left_start].copy_from_slice(common);
    output[layout.left_start..layout.right_start].copy_from_slice(left);
    output[layout.right_start..layout.total].copy_from_slice(right);
    let digest = state_digest(output);
    output[56..88].copy_from_slice(&digest);
    Ok(total_bytes)
}

fn read_u16(bytes: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(bytes[offset..offset + 2].try_into().expect("two bytes"))
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().expect("four bytes"))
}

fn read_u64(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(bytes[offset..offset + 8].try_into().expect("eight bytes"))
}

fn valid_id(value: &str) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty()
        && bytes.len() <= 127
        && bytes[0].is_ascii_lowercase()
        && bytes[1..].iter().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
        })
}

#[derive(Clone, Copy, Debug)]
struct ParsedEffectState<'a> {
    bytes: &'a [u8],
    descriptor_identity: EffectDescriptorIdentity,
    effect_id: &'a str,
    sidechain_id: &'a str,
    contract_major: u16,
    contract_minor: u16,
    state_layout_version: u32,
    sample_rate: u32,
    quantum: u32,
    quality: EffectQuality,
    bypass: bool,
    link_mode: LinkMode,
    sidechain_kind: u32,
    sidechain_required: bool,
    latency_samples: u64,
    tail: TailSamples,
    state_sizes: StatePayloadSizes,
    scratch_bytes: u64,
    automation_capacity: u32,
    request_maximum_total_state_bytes: u64,
    request_maximum_scratch_bytes: u64,
    request_maximum_automation_spans_per_block: u32,
    initial_start: usize,
    initial_count: usize,
    common_start: usize,
    left_start: usize,
    right_start: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct VerifiedEffectState<'a> {
    bytes: &'a [u8],
    descriptor_identity: EffectDescriptorIdentity,
    effect_id: &'a str,
    sidechain_id: &'a str,
    contract_major: u16,
    contract_minor: u16,
    state_layout_version: u32,
    sample_rate: u32,
    quantum: u32,
    quality: EffectQuality,
    bypass: bool,
    link_mode: LinkMode,
    sidechain_kind: u32,
    sidechain_required: bool,
    latency_samples: u64,
    tail: TailSamples,
    state_sizes: StatePayloadSizes,
    scratch_bytes: u64,
    automation_capacity: u32,
    request_maximum_total_state_bytes: u64,
    request_maximum_scratch_bytes: u64,
    request_maximum_automation_spans_per_block: u32,
    initial_start: usize,
    initial_count: usize,
    common_start: usize,
    left_start: usize,
    right_start: usize,
    bound_descriptor: &'static EffectDescriptor,
}

impl<'a> VerifiedEffectState<'a> {
    pub const fn as_bytes(self) -> &'a [u8] {
        self.bytes
    }
    pub const fn descriptor_identity(self) -> EffectDescriptorIdentity {
        self.descriptor_identity
    }
    pub const fn effect_id(self) -> &'a str {
        self.effect_id
    }
    pub const fn contract_version(self) -> (u16, u16) {
        (self.contract_major, self.contract_minor)
    }
    pub const fn state_layout_version(self) -> u32 {
        self.state_layout_version
    }
    pub const fn sample_rate(self) -> u32 {
        self.sample_rate
    }
    pub const fn quantum(self) -> u32 {
        self.quantum
    }
    pub const fn quality(self) -> EffectQuality {
        self.quality
    }
    pub const fn bypass(self) -> bool {
        self.bypass
    }
    pub const fn link_mode(self) -> LinkMode {
        self.link_mode
    }
    pub const fn sidechain(self) -> (u32, &'a str, bool) {
        (
            self.sidechain_kind,
            self.sidechain_id,
            self.sidechain_required,
        )
    }
    pub const fn latency_samples(self) -> u64 {
        self.latency_samples
    }
    pub const fn tail(self) -> TailSamples {
        self.tail
    }
    pub const fn state_sizes(self) -> StatePayloadSizes {
        self.state_sizes
    }
    pub const fn scratch_bytes(self) -> u64 {
        self.scratch_bytes
    }
    pub const fn automation_capacity(self) -> u32 {
        self.automation_capacity
    }
    pub const fn request_limits(self) -> (u64, u64, u32) {
        (
            self.request_maximum_total_state_bytes,
            self.request_maximum_scratch_bytes,
            self.request_maximum_automation_spans_per_block,
        )
    }
    pub fn initial_values(self) -> EffectStateInitialValues<'a> {
        EffectStateInitialValues {
            bytes: self.bytes,
            cursor: 0,
            start: self.initial_start,
            count: self.initial_count,
        }
    }
    pub fn payloads(self) -> (&'a [u8], &'a [u8], &'a [u8]) {
        (
            &self.bytes[self.common_start..self.left_start],
            &self.bytes[self.left_start..self.right_start],
            &self.bytes[self.right_start..],
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EffectStateInitialValues<'a> {
    bytes: &'a [u8],
    start: usize,
    cursor: usize,
    count: usize,
}

impl Iterator for EffectStateInitialValues<'_> {
    type Item = InitialParameterValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor == self.count {
            return None;
        }
        let record = self.start + self.cursor * INITIAL_BYTES;
        self.cursor += 1;
        Some(InitialParameterValue {
            parameter_index: read_u32(self.bytes, record),
            channel: ParameterChannel::from_raw(read_u32(self.bytes, record + 4))
                .expect("verified channel"),
            value: f32::from_bits(read_u32(self.bytes, record + 8)),
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.count - self.cursor;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for EffectStateInitialValues<'_> {}

fn parse_effect_state_structure<'a>(
    bytes: &'a [u8],
    limits: EffectStateLimits,
) -> Result<ParsedEffectState<'a>, Diagnostic> {
    let byte_length = u64::try_from(bytes.len()).map_err(|_| overflow(16))?;
    if byte_length > limits.maximum_envelope_bytes {
        return Err(limit(byte_length));
    }
    if bytes.len() < HEADER_BYTES {
        return Err(diagnostic(Code::Header, bytes.len(), None));
    }
    if bytes[..8] != MAGIC[..] {
        let offset = bytes[..8]
            .iter()
            .zip(MAGIC)
            .position(|(a, b)| a != b)
            .unwrap_or(0);
        return Err(diagnostic(Code::Header, offset, None));
    }
    if read_u16(bytes, 8) != VERSION {
        return Err(diagnostic(Code::Header, 8, None));
    }
    if read_u16(bytes, 10) != HEADER_BYTES as u16 {
        return Err(diagnostic(Code::Header, 10, None));
    }
    if read_u32(bytes, 92) == 0 {
        return Err(diagnostic(Code::Header, 92, None));
    }
    for (offset, size) in [(12, 4), (148, 4), (172, 4), (212, 4)] {
        if let Some(index) = bytes[offset..offset + size]
            .iter()
            .position(|byte| *byte != 0)
        {
            return Err(diagnostic(Code::Reserved, offset + index, None));
        }
    }
    if read_u64(bytes, 16) != byte_length {
        return Err(diagnostic(Code::Length, 16, None));
    }

    let effect_len = checked_usize(u64::from(read_u32(bytes, 124)), 124)?;
    let sidechain_len = checked_usize(u64::from(read_u32(bytes, 128)), 128)?;
    let initial_count = checked_usize(u64::from(read_u32(bytes, 132)), 132)?;
    if initial_count > usize::try_from(limits.maximum_initial_values).map_err(|_| overflow(132))? {
        return Err(limit(
            u64::try_from(initial_count).map_err(|_| overflow(132))?,
        ));
    }
    let payload = u64::from(read_u32(bytes, 160))
        .checked_add(u64::from(read_u32(bytes, 164)))
        .and_then(|value| value.checked_add(u64::from(read_u32(bytes, 168))))
        .ok_or_else(|| overflow(216))?;
    if payload > limits.maximum_payload_bytes {
        return Err(limit(payload));
    }
    let initial_bytes = checked_mul(initial_count, INITIAL_BYTES, 188)?;
    if checked_usize(u64::from(read_u32(bytes, 188)), 188)? != initial_bytes {
        return Err(diagnostic(Code::Length, 188, None));
    }
    if payload != read_u64(bytes, 216) {
        return Err(diagnostic(Code::Length, 216, None));
    }
    let effect_id_end = checked_add(HEADER_BYTES, effect_len, 124)?;
    let sidechain_id_start = effect_id_end;
    let sidechain_id_end = checked_add(sidechain_id_start, sidechain_len, 128)?;
    let initial_start = checked_add(sidechain_id_end, 7, 188)? & !7;
    let initial_end = checked_add(initial_start, initial_bytes, 188)?;
    let common_start = initial_end;
    let left_start = checked_add(
        common_start,
        checked_usize(u64::from(read_u32(bytes, 160)), 160)?,
        160,
    )?;
    let right_start = checked_add(
        left_start,
        checked_usize(u64::from(read_u32(bytes, 164)), 164)?,
        164,
    )?;
    let total = checked_add(
        right_start,
        checked_usize(u64::from(read_u32(bytes, 168)), 168)?,
        168,
    )?;
    if total != bytes.len() {
        return Err(diagnostic(Code::Length, 16, None));
    }
    if bytes[sidechain_id_end..initial_start]
        .iter()
        .any(|byte| *byte != 0)
    {
        let index = bytes[sidechain_id_end..initial_start]
            .iter()
            .position(|byte| *byte != 0)
            .unwrap();
        return Err(diagnostic(Code::Length, sidechain_id_end + index, None));
    }
    for index in 0..initial_count {
        let record = initial_start + index * INITIAL_BYTES;
        if read_u32(bytes, record + 12) != 0 {
            return Err(diagnostic(Code::Reserved, record + 12, Some(index)));
        }
    }

    let quality = EffectQuality::from_raw(read_u32(bytes, 104))
        .ok_or_else(|| diagnostic(Code::Enum, 104, None))?;
    let bypass = match read_u32(bytes, 108) {
        0 => false,
        1 => true,
        _ => return Err(diagnostic(Code::Enum, 108, None)),
    };
    let link_mode = LinkMode::from_raw(read_u32(bytes, 112))
        .ok_or_else(|| diagnostic(Code::Enum, 112, None))?;
    let sidechain_kind = read_u32(bytes, 116);
    if sidechain_kind > 2 {
        return Err(diagnostic(Code::Enum, 116, None));
    }
    let sidechain_required = match read_u32(bytes, 120) {
        0 => false,
        1 => true,
        _ => return Err(diagnostic(Code::Enum, 120, None)),
    };
    match sidechain_kind {
        0 if sidechain_len != 0 || sidechain_required => {
            return Err(diagnostic(Code::Enum, 116, None));
        }
        1 if sidechain_len == 0 || sidechain_required => {
            return Err(diagnostic(Code::Enum, 116, None));
        }
        2 if sidechain_len == 0 => return Err(diagnostic(Code::Enum, 116, None)),
        _ => {}
    }
    let tail = match read_u32(bytes, 144) {
        1 => TailSamples::Finite(read_u64(bytes, 152)),
        2 if read_u64(bytes, 152) == 0 => TailSamples::Infinite,
        2 => return Err(diagnostic(Code::Enum, 152, None)),
        _ => return Err(diagnostic(Code::Enum, 144, None)),
    };
    let effect_id = core::str::from_utf8(&bytes[HEADER_BYTES..effect_id_end])
        .map_err(|_| diagnostic(Code::Text, 124, None))?;
    if !valid_id(effect_id) {
        return Err(diagnostic(Code::Text, 124, None));
    }
    let sidechain_id = core::str::from_utf8(&bytes[sidechain_id_start..sidechain_id_end])
        .map_err(|_| diagnostic(Code::Text, 128, None))?;
    if sidechain_kind != 0 && !valid_id(sidechain_id) {
        return Err(diagnostic(Code::Text, 128, None));
    }
    let mut prior: Option<(u32, u32)> = None;
    for index in 0..initial_count {
        let record = initial_start + index * INITIAL_BYTES;
        let parameter = read_u32(bytes, record);
        let channel = read_u32(bytes, record + 4);
        if ParameterChannel::from_raw(channel).is_none() {
            return Err(diagnostic(Code::Enum, record + 4, Some(index)));
        }
        let key = (parameter, channel);
        if prior.is_some_and(|value| value >= key) {
            return Err(diagnostic(Code::Order, record, Some(index)));
        }
        prior = Some(key);
        let value = f32::from_bits(read_u32(bytes, record + 8));
        if !value.is_finite() || value.to_bits() == (-0.0f32).to_bits() {
            return Err(diagnostic(Code::InitialValues, record + 8, Some(index)));
        }
    }
    if state_digest(bytes) != bytes[56..88] {
        return Err(diagnostic(Code::Digest, 56, None));
    }

    let mut identity = [0; 32];
    identity.copy_from_slice(&bytes[24..56]);
    let descriptor_identity = EffectDescriptorIdentity::from_bytes(identity);
    Ok(ParsedEffectState {
        bytes,
        descriptor_identity,
        effect_id,
        sidechain_id,
        contract_major: read_u16(bytes, 88),
        contract_minor: read_u16(bytes, 90),
        state_layout_version: read_u32(bytes, 92),
        sample_rate: read_u32(bytes, 96),
        quantum: read_u32(bytes, 100),
        quality,
        bypass,
        link_mode,
        sidechain_kind,
        sidechain_required,
        latency_samples: read_u64(bytes, 136),
        tail,
        state_sizes: StatePayloadSizes {
            common_bytes: read_u32(bytes, 160),
            left_bytes: read_u32(bytes, 164),
            right_bytes: read_u32(bytes, 168),
        },
        scratch_bytes: read_u64(bytes, 176),
        automation_capacity: read_u32(bytes, 184),
        request_maximum_total_state_bytes: read_u64(bytes, 192),
        request_maximum_scratch_bytes: read_u64(bytes, 200),
        request_maximum_automation_spans_per_block: read_u32(bytes, 208),
        initial_start,
        initial_count,
        common_start,
        left_start,
        right_start,
    })
}

fn bind_parsed_effect_state<'a>(
    parsed: ParsedEffectState<'a>,
    bound: BoundEffectDescriptorWire<'_>,
) -> Result<VerifiedEffectState<'a>, Diagnostic> {
    if parsed.descriptor_identity != bound.identity() {
        return Err(unavailable(Code::Descriptor, 3 << 16));
    }
    Ok(VerifiedEffectState {
        bytes: parsed.bytes,
        descriptor_identity: parsed.descriptor_identity,
        effect_id: parsed.effect_id,
        sidechain_id: parsed.sidechain_id,
        contract_major: parsed.contract_major,
        contract_minor: parsed.contract_minor,
        state_layout_version: parsed.state_layout_version,
        sample_rate: parsed.sample_rate,
        quantum: parsed.quantum,
        quality: parsed.quality,
        bypass: parsed.bypass,
        link_mode: parsed.link_mode,
        sidechain_kind: parsed.sidechain_kind,
        sidechain_required: parsed.sidechain_required,
        latency_samples: parsed.latency_samples,
        tail: parsed.tail,
        state_sizes: parsed.state_sizes,
        scratch_bytes: parsed.scratch_bytes,
        automation_capacity: parsed.automation_capacity,
        request_maximum_total_state_bytes: parsed.request_maximum_total_state_bytes,
        request_maximum_scratch_bytes: parsed.request_maximum_scratch_bytes,
        request_maximum_automation_spans_per_block: parsed
            .request_maximum_automation_spans_per_block,
        initial_start: parsed.initial_start,
        initial_count: parsed.initial_count,
        common_start: parsed.common_start,
        left_start: parsed.left_start,
        right_start: parsed.right_start,
        bound_descriptor: bound.descriptor(),
    })
}

fn validate_verified_initial_values(
    state: VerifiedEffectState<'_>,
    descriptor: &'static EffectDescriptor,
) -> Result<(), Diagnostic> {
    let mut values = state.initial_values();
    let mut item_index = 0usize;
    for (parameter_index, parameter) in descriptor.parameters.iter().enumerate() {
        let channels: &[ParameterChannel] = match parameter.channel_policy {
            ParameterChannelPolicy::Shared => &[ParameterChannel::Both],
            ParameterChannelPolicy::PerLane => &[ParameterChannel::Left, ParameterChannel::Right],
        };
        for channel in channels {
            let Some(value) = values.next() else {
                let mut diagnostic = diagnostic(
                    Code::InitialValues,
                    state.initial_start + item_index * INITIAL_BYTES,
                    Some(item_index),
                );
                diagnostic.detail = 0;
                return Err(diagnostic);
            };
            let record = state.initial_start + item_index * INITIAL_BYTES;
            if value.parameter_index != parameter_index as u32 {
                return Err(diagnostic(Code::InitialValues, record, Some(item_index)));
            }
            if value.channel != *channel {
                return Err(diagnostic(
                    Code::InitialValues,
                    record + 4,
                    Some(item_index),
                ));
            }
            if !parameter_value_valid(parameter, value.value) {
                return Err(diagnostic(
                    Code::InitialValues,
                    record + 8,
                    Some(item_index),
                ));
            }
            item_index += 1;
        }
    }
    if values.next().is_some() {
        return Err(diagnostic(
            Code::InitialValues,
            state.initial_start + item_index * INITIAL_BYTES,
            Some(item_index),
        ));
    }
    Ok(())
}

pub fn validate_effect_state_current_layout(
    state: VerifiedEffectState<'_>,
) -> Result<(), Diagnostic> {
    let descriptor = state.bound_descriptor;
    if state.effect_id != descriptor.id.as_str() {
        return Err(metadata_mismatch(1));
    }
    if (state.contract_major, state.contract_minor)
        != (descriptor.contract_major, descriptor.contract_minor)
    {
        return Err(metadata_mismatch(2));
    }
    if state.state_layout_version != descriptor.state_layout_version {
        return Err(metadata_mismatch(3));
    }
    if !is_launch_sample_rate(SampleRateHz(state.sample_rate)) {
        return Err(metadata_mismatch(4));
    }
    if state.quantum == 0 {
        return Err(metadata_mismatch(5));
    }
    let quality = descriptor
        .qualities
        .iter()
        .find(|quality| {
            quality.quality == state.quality && quality.sample_rate == state.sample_rate
        })
        .ok_or_else(|| metadata_mismatch(6))?;
    if !descriptor.supported_link_modes.contains(state.link_mode) {
        return Err(metadata_mismatch(8));
    }
    let sidechain_matches = match state.sidechain_kind {
        0 => !descriptor
            .ports
            .iter()
            .any(|port| port.role == miso_engine_effect_contract::PortRole::SidechainInput),
        1 => {
            !state.sidechain_required
                && descriptor.ports.iter().any(|port| {
                    port.role == miso_engine_effect_contract::PortRole::SidechainInput
                        && port.id.as_str() == state.sidechain_id
                        && !port.required
                })
        }
        2 => descriptor.ports.iter().any(|port| {
            port.role == miso_engine_effect_contract::PortRole::SidechainInput
                && port.id.as_str() == state.sidechain_id
                && port.required == state.sidechain_required
        }),
        _ => false,
    };
    if !sidechain_matches {
        return Err(metadata_mismatch(9));
    }
    if state.latency_samples != quality.latency.0 {
        return Err(metadata_mismatch(10));
    }
    if state.tail != quality.tail {
        return Err(metadata_mismatch(11));
    }
    if state.state_sizes != quality.maximum_state {
        return Err(metadata_mismatch(12));
    }
    let expected_scratch = quality
        .scratch_bytes_per_frame
        .checked_mul(u64::from(state.quantum))
        .and_then(|bytes| quality.scratch_fixed_bytes.checked_add(bytes))
        .ok_or_else(|| overflow(176))?;
    if state.scratch_bytes != expected_scratch {
        return Err(metadata_mismatch(13));
    }
    if state.automation_capacity != state.request_maximum_automation_spans_per_block {
        return Err(metadata_mismatch(14));
    }
    let payload = state.state_sizes.total().ok_or_else(|| overflow(216))?;
    if state.request_maximum_total_state_bytes == 0
        || state.request_maximum_scratch_bytes == 0
        || state.request_maximum_automation_spans_per_block == 0
        || payload > state.request_maximum_total_state_bytes
        || state.scratch_bytes > state.request_maximum_scratch_bytes
    {
        return Err(metadata_mismatch(15));
    }
    validate_verified_initial_values(state, descriptor)
}

pub fn validate_effect_state_replay(
    state: VerifiedEffectState<'_>,
    replay: EffectStateReplayView<'_>,
) -> Result<(), Diagnostic> {
    let descriptor = state.bound_descriptor;
    if replay.effect_id != descriptor.id || state.effect_id != descriptor.id.as_str() {
        return Err(metadata_mismatch(1));
    }
    if (state.contract_major, state.contract_minor)
        != (descriptor.contract_major, descriptor.contract_minor)
    {
        return Err(metadata_mismatch(2));
    }
    if state.state_layout_version != descriptor.state_layout_version {
        return Err(metadata_mismatch(3));
    }
    if state.sample_rate != replay.request.sample_rate {
        return Err(metadata_mismatch(4));
    }
    if state.quantum != replay.request.quantum {
        return Err(metadata_mismatch(5));
    }
    if state.quality != replay.request.quality {
        return Err(metadata_mismatch(6));
    }
    if state.bypass != replay.request.bypass {
        return Err(metadata_mismatch(7));
    }
    if state.link_mode != replay.request.link_mode {
        return Err(metadata_mismatch(8));
    }
    let expected_sidechain = sidechain_parts(replay.request.ports);
    if state.sidechain() != expected_sidechain {
        return Err(metadata_mismatch(9));
    }
    let expected = derive_expected_metadata_without_revalidation(descriptor, replay.request)?;
    if state.latency_samples != expected.latency.0 {
        return Err(metadata_mismatch(10));
    }
    if state.tail != expected.tail {
        return Err(metadata_mismatch(11));
    }
    if state.state_sizes != expected.state_sizes {
        return Err(metadata_mismatch(12));
    }
    if state.scratch_bytes != expected.scratch_bytes {
        return Err(metadata_mismatch(13));
    }
    if state.automation_capacity != expected.automation_capacity {
        return Err(metadata_mismatch(14));
    }
    if state.request_limits()
        != (
            replay.request.limits.maximum_total_state_bytes,
            replay.request.limits.maximum_scratch_bytes,
            replay.request.limits.maximum_automation_spans_per_block,
        )
    {
        return Err(metadata_mismatch(15));
    }
    validate_request_limits(expected, replay.request)?;
    let mut saved = state.initial_values();
    for (index, expected) in replay.request.initial_values.iter().enumerate() {
        let record = state.initial_start + index * INITIAL_BYTES;
        let Some(actual) = saved.next() else {
            return Err(diagnostic(Code::InitialValues, record, Some(index)));
        };
        if actual.parameter_index != expected.parameter_index {
            return Err(diagnostic(Code::InitialValues, record, Some(index)));
        }
        if actual.channel != expected.channel {
            return Err(diagnostic(Code::InitialValues, record + 4, Some(index)));
        }
        if actual.value.to_bits() != expected.value.to_bits() {
            return Err(diagnostic(Code::InitialValues, record + 8, Some(index)));
        }
    }
    if saved.next().is_some() {
        return Err(unavailable(Code::InitialValues, 0));
    }
    validate_verified_initial_values(state, descriptor)
}

pub fn validate_effect_state_replay_configuration(
    state: VerifiedEffectState<'_>,
    replay: EffectStateReplayView<'_>,
) -> Result<(), Diagnostic> {
    let descriptor = state.bound_descriptor;
    if replay.effect_id != descriptor.id || state.effect_id != descriptor.id.as_str() {
        return Err(metadata_mismatch(1));
    }
    if (state.contract_major, state.contract_minor)
        != (descriptor.contract_major, descriptor.contract_minor)
    {
        return Err(metadata_mismatch(2));
    }
    if state.sample_rate != replay.request.sample_rate {
        return Err(metadata_mismatch(4));
    }
    if state.quantum != replay.request.quantum {
        return Err(metadata_mismatch(5));
    }
    if state.quality != replay.request.quality {
        return Err(metadata_mismatch(6));
    }
    if state.bypass != replay.request.bypass {
        return Err(metadata_mismatch(7));
    }
    if state.link_mode != replay.request.link_mode {
        return Err(metadata_mismatch(8));
    }
    if state.sidechain() != sidechain_parts(replay.request.ports) {
        return Err(metadata_mismatch(9));
    }
    if state.request_limits()
        != (
            replay.request.limits.maximum_total_state_bytes,
            replay.request.limits.maximum_scratch_bytes,
            replay.request.limits.maximum_automation_spans_per_block,
        )
    {
        return Err(metadata_mismatch(15));
    }
    let mut saved = state.initial_values();
    for (index, expected) in replay.request.initial_values.iter().enumerate() {
        let record = state.initial_start + index * INITIAL_BYTES;
        let Some(actual) = saved.next() else {
            return Err(diagnostic(Code::InitialValues, record, Some(index)));
        };
        if actual.parameter_index != expected.parameter_index {
            return Err(diagnostic(Code::InitialValues, record, Some(index)));
        }
        if actual.channel != expected.channel {
            return Err(diagnostic(Code::InitialValues, record + 4, Some(index)));
        }
        if actual.value.to_bits() != expected.value.to_bits() {
            return Err(diagnostic(Code::InitialValues, record + 8, Some(index)));
        }
    }
    if saved.next().is_some() {
        return Err(unavailable(Code::InitialValues, 0));
    }
    Ok(())
}

pub fn effect_state_replay_view_from_verified<'initial>(
    state: VerifiedEffectState<'_>,
    initial_values: &'initial [InitialParameterValue],
) -> Result<EffectStateReplayView<'initial>, Diagnostic> {
    let descriptor = state.bound_descriptor;
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
                .ok_or_else(|| metadata_mismatch(9))?;
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
        _ => return Err(metadata_mismatch(9)),
    };
    let (maximum_total_state_bytes, maximum_scratch_bytes, maximum_automation_spans_per_block) =
        state.request_limits();
    let replay = EffectStateReplayView {
        effect_id: descriptor.id,
        request: PrepareEffectRequest {
            sample_rate: state.sample_rate,
            quantum: state.quantum,
            quality: state.quality,
            bypass: state.bypass,
            link_mode: state.link_mode,
            ports: PreparedPorts { sidechain },
            initial_values,
            limits: miso_engine_effect_contract::PrepareEffectLimits {
                maximum_total_state_bytes,
                maximum_scratch_bytes,
                maximum_automation_spans_per_block,
            },
        },
    };
    validate_effect_state_replay_configuration(state, replay)?;
    Ok(replay)
}

pub fn inspect_effect_state_selector(
    bytes: &[u8],
    limits: EffectStateLimits,
) -> Result<EffectStateSelector, Diagnostic> {
    validate_limits(limits)?;
    let parsed = parse_effect_state_structure(bytes, limits)?;
    Ok(EffectStateSelector {
        descriptor_identity: parsed.descriptor_identity,
        state_layout_version: parsed.state_layout_version,
    })
}

pub fn verify_effect_state<'a>(
    bound: BoundEffectDescriptorWire<'_>,
    bytes: &'a [u8],
    limits: EffectStateLimits,
) -> Result<VerifiedEffectState<'a>, Diagnostic> {
    validate_limits(limits)?;
    let descriptor_bytes = u64::try_from(bound.wire().len()).map_err(|_| overflow(0))?;
    if descriptor_bytes > limits.maximum_descriptor_bytes {
        return Err(limit(descriptor_bytes));
    }
    bind_parsed_effect_state(parse_effect_state_structure(bytes, limits)?, bound)
}
