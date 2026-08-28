use crate::{
    EFFECT_DESCRIPTOR_WIRE_UNAVAILABLE, EffectDescriptorWireDiagnosticCode as Code,
    EffectDescriptorWireDiagnostic as Diagnostic,
};
use miso_engine_core::{
    LAUNCH_SAMPLE_RATES, SampleRateHz, is_extended_compatibility_sample_rate, is_launch_sample_rate,
};
use miso_engine_effect_contract::{
    AutomationRate, DescriptorDiagnosticCode, EffectDescriptor, EffectQuality, LinkMode,
    LinkModeSet, ObservationCadenceV1, ObservationChannelsV1, ObservationCostV1, ObservationFoldV1,
    ObservationKindV1, ParameterChannelPolicy, ParameterDomain, ParameterMapping, ParameterUnit,
    PortDescriptor, PortLayout, PortRole, SmoothingRule, TailSamples, validate_descriptor,
};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

const MAGIC: &[u8; 8] = b"MISOEFD1";
const VERSION: u16 = 1;
const HEADER_BYTES: usize = 96;
const PARAMETER_BYTES: usize = 80;
const PORT_BYTES: usize = 24;
const QUALITY_BYTES: usize = 64;
const ENUM_CHOICE_BYTES: usize = 16;
/// Issue #143: one observation record, in the frozen little-endian layout
///
/// | offset | width | field |
/// |---|---|---|
/// | 0 | u32 | `id`, the effect-local tap id |
/// | 4 | u8 | `kind` |
/// | 5 | u8 | `unit` |
/// | 6 | u8 | `cost` |
/// | 7 | u8 | `cadence` |
/// | 8 | u8 | `fold` |
/// | 9 | u8 | `channels` |
/// | 10 | u8 | `display_name` byte length |
/// | 11 | u8 | `display_unit` byte length |
/// | 12 | u32 | `minimum` bits |
/// | 16 | u32 | `maximum` bits |
/// | 20 | u32 | `display_name` offset into the string pool |
/// | 24 | u32 | `display_unit` offset into the string pool |
/// | 28 | u32 | required zero |
///
/// The six vocabularies are single bytes rather than the `u32`s the parameter record uses because
/// the record is 32 bytes and the fields do not fit otherwise; every string in this workspace is
/// capped at 255 bytes by `valid_text`, so the two lengths fit a byte for the same reason.
const OBSERVATION_BYTES: usize = 32;
const IDENTITY_DOMAIN: &[u8] = b"miso.engine.effect-descriptor.identity.v1\0";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectDescriptorIdentity([u8; 32]);

impl EffectDescriptorIdentity {
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub(crate) const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum EffectDescriptorBindingErrorKind {
    ExternalWire = 1,
    StaticDescriptorMismatch = 2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectDescriptorBindingError {
    kind: EffectDescriptorBindingErrorKind,
    diagnostic: Diagnostic,
}

impl EffectDescriptorBindingError {
    pub const fn kind(self) -> EffectDescriptorBindingErrorKind {
        self.kind
    }

    pub const fn diagnostic(self) -> Diagnostic {
        self.diagnostic
    }
}

/// Canonical Issue-082 wire proven to describe one exact static descriptor.
///
/// Private fields prevent raw wire or identity bytes from being treated as factory provenance.
#[derive(Clone, Copy, Debug)]
pub struct BoundEffectDescriptorWire<'a> {
    descriptor: &'static EffectDescriptor,
    wire: &'a [u8],
    identity: EffectDescriptorIdentity,
}

impl<'a> BoundEffectDescriptorWire<'a> {
    pub const fn wire(&self) -> &'a [u8] {
        self.wire
    }

    pub const fn identity(&self) -> EffectDescriptorIdentity {
        self.identity
    }

    pub(crate) const fn descriptor(&self) -> &'static EffectDescriptor {
        self.descriptor
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VerifiedEffectDescriptorWire<'a> {
    bytes: &'a [u8],
    parameter_count: u32,
    port_count: u32,
    quality_count: u32,
    enum_choice_count: u32,
    observation_count: u32,
    state_layout_version: u32,
    supported_link_mode_bits: u32,
}

impl<'a> VerifiedEffectDescriptorWire<'a> {
    pub const fn as_bytes(self) -> &'a [u8] {
        self.bytes
    }

    pub const fn parameter_count(self) -> u32 {
        self.parameter_count
    }

    pub const fn port_count(self) -> u32 {
        self.port_count
    }

    pub const fn quality_count(self) -> u32 {
        self.quality_count
    }

    pub const fn enum_choice_count(self) -> u32 {
        self.enum_choice_count
    }

    /// Number of declared observation taps (issue #143). Zero for every pre-#143 descriptor.
    pub const fn observation_count(self) -> u32 {
        self.observation_count
    }

    pub const fn state_layout_version(self) -> u32 {
        self.state_layout_version
    }

    pub const fn supported_link_mode_bits(self) -> u32 {
        self.supported_link_mode_bits
    }

    /// Identity of bytes this value already proved canonical; performs no second validation pass.
    pub fn identity(self) -> EffectDescriptorIdentity {
        descriptor_identity(self.bytes)
    }
}

#[derive(Clone, Copy)]
struct Layout {
    total: u32,
    parameters: u32,
    ports: u32,
    qualities: u32,
    choices: u32,
    observations: u32,
    parameter_offset: u32,
    port_offset: u32,
    quality_offset: u32,
    choice_offset: u32,
    observation_offset: u32,
    string_offset: u32,
    string_bytes: u32,
}

impl Layout {
    /// The value header byte 92 carries.
    ///
    /// **Zero when the descriptor declares no tap**, which is what keeps every pre-#143
    /// descriptor's bytes -- and therefore its identity -- exactly where they were: header bytes
    /// 88..96 stay the eight zeros the pre-#143 verifier demanded. A tap-bearing descriptor writes
    /// the real offset there, which is precisely the byte a stale reader refuses.
    const fn header_observation_offset(self) -> u32 {
        if self.observations == 0 {
            0
        } else {
            self.observation_offset
        }
    }
}

fn diagnostic(code: Code, byte_offset: usize, record_index: Option<usize>) -> Diagnostic {
    Diagnostic::new(
        code,
        u32::try_from(byte_offset).unwrap_or(EFFECT_DESCRIPTOR_WIRE_UNAVAILABLE),
        record_index
            .and_then(|index| u32::try_from(index).ok())
            .unwrap_or(EFFECT_DESCRIPTOR_WIRE_UNAVAILABLE),
    )
}

fn overflow(byte_offset: usize, record_index: Option<usize>) -> Diagnostic {
    diagnostic(Code::Overflow, byte_offset, record_index)
}

fn checked_add(left: u32, right: u32) -> Result<u32, Diagnostic> {
    checked_add_at(left, right, 16)
}

fn checked_mul(left: u32, right: u32) -> Result<u32, Diagnostic> {
    checked_mul_at(left, right, 16)
}

fn checked_add_at(left: u32, right: u32, byte_offset: usize) -> Result<u32, Diagnostic> {
    left.checked_add(right)
        .ok_or_else(|| overflow(byte_offset, None))
}

fn checked_mul_at(left: u32, right: u32, byte_offset: usize) -> Result<u32, Diagnostic> {
    left.checked_mul(right)
        .ok_or_else(|| overflow(byte_offset, None))
}

fn u32_len(length: usize) -> Result<u32, Diagnostic> {
    u32::try_from(length).map_err(|_| overflow(16, None))
}

fn add_text_size(total: &mut u32, value: &str) -> Result<(), Diagnostic> {
    *total = checked_add(*total, u32_len(value.len())?)?;
    Ok(())
}

fn port_key(port: &PortDescriptor) -> (u32, &[u8]) {
    (port.role as u32, port.id.as_str().as_bytes())
}

fn canonical_port_at(
    ports: &'static [PortDescriptor],
    index: usize,
) -> &'static PortDescriptor {
    ports
        .iter()
        .find(|candidate| {
            let key = port_key(candidate);
            ports.iter().filter(|port| port_key(port) < key).count() == index
        })
        .expect("validated descriptor has a unique canonical port at every index")
}

fn descriptor_layout(
    descriptor: &'static EffectDescriptor,
    maximum_descriptor_bytes: u32,
) -> Result<Layout, Diagnostic> {
    validate_descriptor(descriptor).map_err(|_| diagnostic(Code::Semantic, 0, None))?;
    if maximum_descriptor_bytes == 0 {
        return Err(diagnostic(Code::Limit, 16, None));
    }
    let parameters = u32_len(descriptor.parameters.len())?;
    let ports = u32_len(descriptor.ports.len())?;
    let qualities = u32_len(descriptor.qualities.len())?;
    let observations = u32_len(descriptor.observations.len())?;
    let mut choices = 0u32;
    let mut string_bytes = 0u32;
    add_text_size(&mut string_bytes, descriptor.id.as_str())?;
    add_text_size(&mut string_bytes, descriptor.display_name)?;
    for parameter in descriptor.parameters {
        choices = checked_add(choices, u32_len(parameter.enum_choices.len())?)?;
        add_text_size(&mut string_bytes, parameter.display_name)?;
        add_text_size(&mut string_bytes, parameter.display_unit)?;
        for choice in parameter.enum_choices {
            add_text_size(&mut string_bytes, choice.label)?;
        }
    }
    for index in 0..descriptor.ports.len() {
        add_text_size(
            &mut string_bytes,
            canonical_port_at(descriptor.ports, index).id.as_str(),
        )?;
    }
    // Issue #143: observation strings own the tail of the pool, after the ports, so a zero-tap
    // descriptor's pool is the pre-#143 pool byte for byte.
    for observation in descriptor.observations {
        add_text_size(&mut string_bytes, observation.display_name)?;
        add_text_size(&mut string_bytes, observation.display_unit)?;
    }
    let parameter_offset = HEADER_BYTES as u32;
    let port_offset = checked_add(
        parameter_offset,
        checked_mul(parameters, PARAMETER_BYTES as u32)?,
    )?;
    let quality_offset = checked_add(port_offset, checked_mul(ports, PORT_BYTES as u32)?)?;
    let choice_offset = checked_add(
        quality_offset,
        checked_mul(qualities, QUALITY_BYTES as u32)?,
    )?;
    let observation_offset = checked_add(
        choice_offset,
        checked_mul(choices, ENUM_CHOICE_BYTES as u32)?,
    )?;
    let string_offset = checked_add(
        observation_offset,
        checked_mul(observations, OBSERVATION_BYTES as u32)?,
    )?;
    let total = checked_add(string_offset, string_bytes)?;
    if total > maximum_descriptor_bytes
        || u64::from(total) > usize::MAX as u64
        || u64::from(total) > isize::MAX as u64
    {
        return Err(diagnostic(Code::Limit, 16, None));
    }
    Ok(Layout {
        total,
        parameters,
        ports,
        qualities,
        choices,
        observations,
        parameter_offset,
        port_offset,
        quality_offset,
        choice_offset,
        observation_offset,
        string_offset,
        string_bytes,
    })
}

pub fn effect_descriptor_wire_required_size(
    descriptor: &'static EffectDescriptor,
    maximum_descriptor_bytes: u32,
) -> Result<u32, Diagnostic> {
    Ok(descriptor_layout(descriptor, maximum_descriptor_bytes)?.total)
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

fn write_text(output: &mut [u8], cursor: &mut u32, value: &str) -> (u32, u32) {
    let offset = *cursor;
    let length = value.len() as u32;
    let start = offset as usize;
    output[start..start + value.len()].copy_from_slice(value.as_bytes());
    *cursor += length;
    (offset, length)
}

pub fn encode_effect_descriptor_wire(
    descriptor: &'static EffectDescriptor,
    maximum_descriptor_bytes: u32,
    output: &mut [u8],
) -> Result<u32, Diagnostic> {
    let layout = descriptor_layout(descriptor, maximum_descriptor_bytes)?;
    if output.len() < layout.total as usize {
        return Err(Diagnostic::buffer_too_small(layout.total));
    }
    let output = &mut output[..layout.total as usize];
    output.fill(0);
    output[..8].copy_from_slice(MAGIC);
    write_u16(output, 8, VERSION);
    write_u16(output, 10, HEADER_BYTES as u16);
    write_u32(output, 16, layout.total);
    write_u16(output, 20, descriptor.contract_major);
    write_u16(output, 22, descriptor.contract_minor);
    write_u32(output, 24, descriptor.state_layout_version);
    write_u32(output, 28, descriptor.supported_link_modes.bits());
    write_u32(output, 48, layout.parameters);
    write_u32(output, 52, layout.parameter_offset);
    write_u32(output, 56, layout.ports);
    write_u32(output, 60, layout.port_offset);
    write_u32(output, 64, layout.qualities);
    write_u32(output, 68, layout.quality_offset);
    write_u32(output, 72, layout.choices);
    write_u32(output, 76, layout.choice_offset);
    write_u32(output, 80, layout.string_bytes);
    write_u32(output, 84, layout.string_offset);
    write_u32(output, 88, layout.observations);
    write_u32(output, 92, layout.header_observation_offset());

    let mut string_cursor = layout.string_offset;
    let (offset, length) = write_text(output, &mut string_cursor, descriptor.id.as_str());
    write_u32(output, 32, offset);
    write_u32(output, 36, length);
    let (offset, length) = write_text(output, &mut string_cursor, descriptor.display_name);
    write_u32(output, 40, offset);
    write_u32(output, 44, length);

    let mut choice_index = 0u32;
    for (index, parameter) in descriptor.parameters.iter().enumerate() {
        let record = layout.parameter_offset as usize + index * PARAMETER_BYTES;
        write_u32(output, record, parameter.id.0);
        write_u32(output, record + 4, parameter.unit as u32);
        write_u32(output, record + 8, parameter.domain as u32);
        write_u32(output, record + 12, parameter.mapping as u32);
        write_u32(output, record + 16, parameter.automation_rate as u32);
        write_u32(output, record + 20, parameter.channel_policy as u32);
        write_u32(output, record + 24, parameter.smoothing as u32);
        write_u32(output, record + 28, parameter.smoothing_samples);
        let flags = u32::from(parameter.readable)
            | (u32::from(parameter.automatable) << 1)
            | (u32::from(parameter.minimum.is_some()) << 2)
            | (u32::from(parameter.maximum.is_some()) << 3);
        write_u32(output, record + 32, flags);
        write_u32(
            output,
            record + 36,
            parameter.minimum.unwrap_or(0.0).to_bits(),
        );
        write_u32(
            output,
            record + 40,
            parameter.maximum.unwrap_or(0.0).to_bits(),
        );
        write_u32(output, record + 44, parameter.default_value.to_bits());
        write_u32(output, record + 48, choice_index);
        write_u32(output, record + 52, parameter.enum_choices.len() as u32);
        let (offset, length) = write_text(output, &mut string_cursor, parameter.display_name);
        write_u32(output, record + 56, offset);
        write_u32(output, record + 60, length);
        let (offset, length) = write_text(output, &mut string_cursor, parameter.display_unit);
        write_u32(output, record + 64, offset);
        write_u32(output, record + 68, length);
        for choice in parameter.enum_choices {
            let choice_record =
                layout.choice_offset as usize + choice_index as usize * ENUM_CHOICE_BYTES;
            write_u32(output, choice_record, choice.value.to_bits());
            let (offset, length) = write_text(output, &mut string_cursor, choice.label);
            write_u32(output, choice_record + 4, offset);
            write_u32(output, choice_record + 8, length);
            choice_index += 1;
        }
    }
    for index in 0..descriptor.ports.len() {
        let port = canonical_port_at(descriptor.ports, index);
        let record = layout.port_offset as usize + index * PORT_BYTES;
        let (offset, length) = write_text(output, &mut string_cursor, port.id.as_str());
        write_u32(output, record, offset);
        write_u32(output, record + 4, length);
        write_u32(output, record + 8, port.role as u32);
        write_u32(output, record + 12, u32::from(port.required));
        write_u32(output, record + 16, port.layout as u32);
    }
    for (index, observation) in descriptor.observations.iter().enumerate() {
        let record = layout.observation_offset as usize + index * OBSERVATION_BYTES;
        write_u32(output, record, observation.id.0);
        output[record + 4] = observation.kind as u8;
        output[record + 5] = observation.unit as u8;
        output[record + 6] = observation.cost as u8;
        output[record + 7] = observation.cadence as u8;
        output[record + 8] = observation.fold as u8;
        output[record + 9] = observation.channels as u8;
        output[record + 10] = observation.display_name.len() as u8;
        output[record + 11] = observation.display_unit.len() as u8;
        write_u32(output, record + 12, observation.minimum.to_bits());
        write_u32(output, record + 16, observation.maximum.to_bits());
        let (offset, _) = write_text(output, &mut string_cursor, observation.display_name);
        write_u32(output, record + 20, offset);
        let (offset, _) = write_text(output, &mut string_cursor, observation.display_unit);
        write_u32(output, record + 24, offset);
    }
    for (index, quality) in descriptor.qualities.iter().enumerate() {
        let record = layout.quality_offset as usize + index * QUALITY_BYTES;
        write_u32(output, record, quality.quality as u32);
        write_u32(output, record + 4, quality.sample_rate);
        write_u64(output, record + 8, quality.latency.0);
        match quality.tail {
            TailSamples::Finite(samples) => {
                write_u32(output, record + 16, 1);
                write_u64(output, record + 24, samples);
            }
            TailSamples::Infinite => write_u32(output, record + 16, 2),
        }
        write_u32(output, record + 32, quality.maximum_state.common_bytes);
        write_u32(output, record + 36, quality.maximum_state.left_bytes);
        write_u32(output, record + 40, quality.maximum_state.right_bytes);
        write_u64(output, record + 48, quality.scratch_fixed_bytes);
        write_u64(output, record + 56, quality.scratch_bytes_per_frame);
    }
    debug_assert_eq!(choice_index, layout.choices);
    debug_assert_eq!(string_cursor, layout.total);
    Ok(layout.total)
}

fn read_u16(bytes: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(bytes[offset..offset + 2].try_into().expect("checked field"))
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().expect("checked field"))
}

fn read_u64(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(bytes[offset..offset + 8].try_into().expect("checked field"))
}

#[derive(Clone, Copy)]
struct BorrowedEffectDescriptorView<'a> {
    bytes: &'a [u8],
    layout: Layout,
    effect_id: &'a str,
    display_name: &'a str,
    contract_major: u16,
    state_layout_version: u32,
    link_modes: LinkModeSet,
}

#[derive(Clone, Copy)]
struct BorrowedParameter<'a> {
    id: u32,
    domain: ParameterDomain,
    mapping: ParameterMapping,
    automation_rate: AutomationRate,
    smoothing: SmoothingRule,
    smoothing_samples: u32,
    automatable: bool,
    minimum: Option<f32>,
    maximum: Option<f32>,
    default_value: f32,
    choice_start: u32,
    choice_count: u32,
    display_name: &'a str,
    display_unit: &'a str,
}

#[derive(Clone, Copy)]
struct BorrowedPort<'a> {
    id: &'a str,
    role: PortRole,
    required: bool,
    layout: PortLayout,
}

#[derive(Clone, Copy)]
struct BorrowedObservation<'a> {
    id: u32,
    cost: ObservationCostV1,
    cadence: ObservationCadenceV1,
    channels: ObservationChannelsV1,
    minimum: f32,
    maximum: f32,
    display_name: &'a str,
    display_unit: &'a str,
}

#[derive(Clone, Copy)]
struct BorrowedQuality {
    quality: EffectQuality,
    sample_rate: u32,
    left_bytes: u32,
    right_bytes: u32,
}

impl<'a> BorrowedEffectDescriptorView<'a> {
    fn text(self, offset: usize, length: usize) -> &'a str {
        core::str::from_utf8(&self.bytes[offset..offset + length]).expect("text phase checked")
    }

    fn parameter(self, index: usize) -> BorrowedParameter<'a> {
        let record = self.layout.parameter_offset as usize + index * PARAMETER_BYTES;
        let flags = read_u32(self.bytes, record + 32);
        let name_offset = read_u32(self.bytes, record + 56) as usize;
        let name_length = read_u32(self.bytes, record + 60) as usize;
        let unit_offset = read_u32(self.bytes, record + 64) as usize;
        let unit_length = read_u32(self.bytes, record + 68) as usize;
        BorrowedParameter {
            id: read_u32(self.bytes, record),
            domain: ParameterDomain::from_raw(read_u32(self.bytes, record + 8)).unwrap(),
            mapping: ParameterMapping::from_raw(read_u32(self.bytes, record + 12)).unwrap(),
            automation_rate: AutomationRate::from_raw(read_u32(self.bytes, record + 16)).unwrap(),
            smoothing: SmoothingRule::from_raw(read_u32(self.bytes, record + 24)).unwrap(),
            smoothing_samples: read_u32(self.bytes, record + 28),
            automatable: flags & 2 != 0,
            minimum: (flags & 4 != 0).then(|| f32::from_bits(read_u32(self.bytes, record + 36))),
            maximum: (flags & 8 != 0).then(|| f32::from_bits(read_u32(self.bytes, record + 40))),
            default_value: f32::from_bits(read_u32(self.bytes, record + 44)),
            choice_start: read_u32(self.bytes, record + 48),
            choice_count: read_u32(self.bytes, record + 52),
            display_name: self.text(name_offset, name_length),
            display_unit: self.text(unit_offset, unit_length),
        }
    }

    fn choice(self, index: usize) -> (f32, &'a str) {
        let record = self.layout.choice_offset as usize + index * ENUM_CHOICE_BYTES;
        let offset = read_u32(self.bytes, record + 4) as usize;
        let length = read_u32(self.bytes, record + 8) as usize;
        (
            f32::from_bits(read_u32(self.bytes, record)),
            self.text(offset, length),
        )
    }

    fn port(self, index: usize) -> BorrowedPort<'a> {
        let record = self.layout.port_offset as usize + index * PORT_BYTES;
        let offset = read_u32(self.bytes, record) as usize;
        let length = read_u32(self.bytes, record + 4) as usize;
        BorrowedPort {
            id: self.text(offset, length),
            role: PortRole::from_raw(read_u32(self.bytes, record + 8)).unwrap(),
            required: read_u32(self.bytes, record + 12) == 1,
            layout: PortLayout::from_raw(read_u32(self.bytes, record + 16)).unwrap(),
        }
    }

    fn observation(self, index: usize) -> BorrowedObservation<'a> {
        let record = self.layout.observation_offset as usize + index * OBSERVATION_BYTES;
        let byte = |offset: usize| u32::from(self.bytes[record + offset]);
        BorrowedObservation {
            id: read_u32(self.bytes, record),
            cost: ObservationCostV1::from_raw(byte(6)).unwrap(),
            cadence: ObservationCadenceV1::from_raw(byte(7)).unwrap(),
            channels: ObservationChannelsV1::from_raw(byte(9)).unwrap(),
            minimum: f32::from_bits(read_u32(self.bytes, record + 12)),
            maximum: f32::from_bits(read_u32(self.bytes, record + 16)),
            display_name: self.text(
                read_u32(self.bytes, record + 20) as usize,
                usize::from(self.bytes[record + 10]),
            ),
            display_unit: self.text(
                read_u32(self.bytes, record + 24) as usize,
                usize::from(self.bytes[record + 11]),
            ),
        }
    }

    fn quality(self, index: usize) -> BorrowedQuality {
        let record = self.layout.quality_offset as usize + index * QUALITY_BYTES;
        BorrowedQuality {
            quality: EffectQuality::from_raw(read_u32(self.bytes, record)).unwrap(),
            sample_rate: read_u32(self.bytes, record + 4),
            left_bytes: read_u32(self.bytes, record + 36),
            right_bytes: read_u32(self.bytes, record + 40),
        }
    }
}

fn valid_text(value: &str) -> bool {
    !value.is_empty() && value.len() <= 255 && !value.chars().any(char::is_control)
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

fn canonical_float(value: f32) -> bool {
    value.is_finite() && value.to_bits() != (-0.0f32).to_bits()
}

fn take_text<'a>(
    bytes: &'a [u8],
    offset: u32,
    length: u32,
    cursor: &mut u32,
    field_offset: usize,
    record_index: Option<usize>,
) -> Result<&'a [u8], Diagnostic> {
    if offset != *cursor {
        return Err(diagnostic(Code::Offset, field_offset, record_index));
    }
    let end = offset
        .checked_add(length)
        .ok_or_else(|| overflow(field_offset, record_index))?;
    if end as usize > bytes.len() {
        return Err(diagnostic(Code::Length, field_offset, record_index));
    }
    *cursor = end;
    Ok(&bytes[offset as usize..end as usize])
}

fn parse_borrowed_wire(
    bytes: &[u8],
    maximum_descriptor_bytes: u32,
) -> Result<BorrowedEffectDescriptorView<'_>, Diagnostic> {
    // Phase 2: explicit limit and host-fit checks. Rust references already cover argument validity.
    if maximum_descriptor_bytes == 0
        || bytes.len() > maximum_descriptor_bytes as usize
        || bytes.len() > u32::MAX as usize
        || bytes.len() > isize::MAX as usize
    {
        return Err(diagnostic(Code::Limit, 16, None));
    }
    // Phase 3: magic and versioned header shape.
    if bytes.len() < HEADER_BYTES {
        return Err(diagnostic(Code::Header, bytes.len(), None));
    }
    if &bytes[..8] != MAGIC {
        let offset = bytes[..8]
            .iter()
            .zip(MAGIC)
            .position(|(actual, expected)| actual != expected)
            .unwrap_or(0);
        return Err(diagnostic(Code::Header, offset, None));
    }
    if read_u16(bytes, 8) != VERSION {
        return Err(diagnostic(Code::Header, 8, None));
    }
    if read_u16(bytes, 10) != HEADER_BYTES as u16 {
        return Err(diagnostic(Code::Header, 10, None));
    }
    // Phase 4: total and section lengths, using the frozen contiguous physical layout.
    let total = read_u32(bytes, 16);
    if total as usize != bytes.len() {
        return Err(diagnostic(Code::Length, 16, None));
    }
    let parameters = read_u32(bytes, 48);
    let ports = read_u32(bytes, 56);
    let qualities = read_u32(bytes, 64);
    let choices = read_u32(bytes, 72);
    let string_bytes = read_u32(bytes, 80);
    let observations = read_u32(bytes, 88);
    let parameter_offset = HEADER_BYTES as u32;
    let port_offset = checked_add_at(
        parameter_offset,
        checked_mul_at(parameters, PARAMETER_BYTES as u32, 48)?,
        48,
    )?;
    let quality_offset = checked_add_at(
        port_offset,
        checked_mul_at(ports, PORT_BYTES as u32, 56)?,
        56,
    )?;
    let choice_offset = checked_add_at(
        quality_offset,
        checked_mul_at(qualities, QUALITY_BYTES as u32, 64)?,
        64,
    )?;
    let observation_offset = checked_add_at(
        choice_offset,
        checked_mul_at(choices, ENUM_CHOICE_BYTES as u32, 72)?,
        72,
    )?;
    let string_offset = checked_add_at(
        observation_offset,
        checked_mul_at(observations, OBSERVATION_BYTES as u32, 88)?,
        88,
    )?;
    if checked_add_at(string_offset, string_bytes, 80)? != total {
        return Err(diagnostic(Code::Length, 80, None));
    }
    let layout = Layout {
        total,
        parameters,
        ports,
        qualities,
        choices,
        observations,
        parameter_offset,
        port_offset,
        quality_offset,
        choice_offset,
        observation_offset,
        string_offset,
        string_bytes,
    };

    // Phase 5: reserved words and flags, in table traversal order.
    if read_u32(bytes, 12) != 0 {
        return Err(diagnostic(Code::Reserved, 12, None));
    }
    // Issue #143: header bytes 88..96 are the observation section's count and offset. A
    // descriptor that declares no tap keeps them all zero -- byte for byte the reserved-zero
    // window the pre-#143 verifier enforced, which is what makes every zero-tap identity unmoved
    // and what makes a stale reader refuse a tap-bearing descriptor rather than ignore its menu.
    if observations == 0
        && let Some(index) = bytes[88..96].iter().position(|byte| *byte != 0)
    {
        return Err(diagnostic(Code::Reserved, 88 + index, None));
    }
    for index in 0..parameters as usize {
        let record = parameter_offset as usize + index * PARAMETER_BYTES;
        let flags = read_u32(bytes, record + 32);
        if flags & !15 != 0 {
            return Err(diagnostic(Code::Flags, record + 32, Some(index)));
        }
        if flags & 4 == 0 && read_u32(bytes, record + 36) != 0 {
            return Err(diagnostic(Code::Flags, record + 36, Some(index)));
        }
        if flags & 8 == 0 && read_u32(bytes, record + 40) != 0 {
            return Err(diagnostic(Code::Flags, record + 40, Some(index)));
        }
        for field in [72, 76] {
            if read_u32(bytes, record + field) != 0 {
                return Err(diagnostic(Code::Reserved, record + field, Some(index)));
            }
        }
    }
    for index in 0..ports as usize {
        let record = port_offset as usize + index * PORT_BYTES;
        if read_u32(bytes, record + 20) != 0 {
            return Err(diagnostic(Code::Reserved, record + 20, Some(index)));
        }
    }
    for index in 0..qualities as usize {
        let record = quality_offset as usize + index * QUALITY_BYTES;
        for field in [20, 44] {
            if read_u32(bytes, record + field) != 0 {
                return Err(diagnostic(Code::Reserved, record + field, Some(index)));
            }
        }
    }
    for index in 0..choices as usize {
        let record = choice_offset as usize + index * ENUM_CHOICE_BYTES;
        if read_u32(bytes, record + 12) != 0 {
            return Err(diagnostic(Code::Reserved, record + 12, Some(index)));
        }
    }
    for index in 0..observations as usize {
        let record = observation_offset as usize + index * OBSERVATION_BYTES;
        if read_u32(bytes, record + 28) != 0 {
            return Err(diagnostic(Code::Reserved, record + 28, Some(index)));
        }
    }

    // Phase 6: exact offsets, table order, and first-use string ownership.
    let mut string_cursor = string_offset;
    let effect_id_bytes = take_text(
        bytes,
        read_u32(bytes, 32),
        read_u32(bytes, 36),
        &mut string_cursor,
        32,
        None,
    )?;
    let display_name_bytes = take_text(
        bytes,
        read_u32(bytes, 40),
        read_u32(bytes, 44),
        &mut string_cursor,
        40,
        None,
    )?;
    for (field, expected) in [
        (52, parameter_offset),
        (60, port_offset),
        (68, quality_offset),
        (76, choice_offset),
        (84, string_offset),
        (92, layout.header_observation_offset()),
    ] {
        if read_u32(bytes, field) != expected {
            return Err(diagnostic(Code::Offset, field, None));
        }
    }
    let mut choice_cursor = 0u32;
    let mut prior_parameter = None;
    for index in 0..parameters as usize {
        let record = parameter_offset as usize + index * PARAMETER_BYTES;
        let id = read_u32(bytes, record);
        if prior_parameter.is_some_and(|prior| id <= prior) {
            return Err(diagnostic(Code::Order, record, Some(index)));
        }
        prior_parameter = Some(id);
        if read_u32(bytes, record + 48) != choice_cursor {
            return Err(diagnostic(Code::Offset, record + 48, Some(index)));
        }
        let count = read_u32(bytes, record + 52);
        choice_cursor = choice_cursor
            .checked_add(count)
            .ok_or_else(|| overflow(record + 52, Some(index)))?;
        if choice_cursor > choices {
            return Err(diagnostic(Code::Length, record + 52, Some(index)));
        }
        take_text(
            bytes,
            read_u32(bytes, record + 56),
            read_u32(bytes, record + 60),
            &mut string_cursor,
            record + 56,
            Some(index),
        )?;
        take_text(
            bytes,
            read_u32(bytes, record + 64),
            read_u32(bytes, record + 68),
            &mut string_cursor,
            record + 64,
            Some(index),
        )?;
        let start = read_u32(bytes, record + 48) as usize;
        let mut prior_choice = None;
        for choice_index in start..start + count as usize {
            let choice_record = choice_offset as usize + choice_index * ENUM_CHOICE_BYTES;
            let value = f32::from_bits(read_u32(bytes, choice_record));
            if prior_choice.is_some_and(|prior| value.is_finite() && value <= prior) {
                return Err(diagnostic(Code::Order, choice_record, Some(choice_index)));
            }
            if value.is_finite() {
                prior_choice = Some(value);
            }
            take_text(
                bytes,
                read_u32(bytes, choice_record + 4),
                read_u32(bytes, choice_record + 8),
                &mut string_cursor,
                choice_record + 4,
                Some(choice_index),
            )?;
        }
    }
    if choice_cursor != choices {
        return Err(diagnostic(Code::Offset, 72, None));
    }
    let mut prior_port: Option<(u32, &[u8])> = None;
    for index in 0..ports as usize {
        let record = port_offset as usize + index * PORT_BYTES;
        let id = take_text(
            bytes,
            read_u32(bytes, record),
            read_u32(bytes, record + 4),
            &mut string_cursor,
            record,
            Some(index),
        )?;
        let key = (read_u32(bytes, record + 8), id);
        if prior_port.is_some_and(|prior| prior >= key) {
            return Err(diagnostic(Code::Order, record + 8, Some(index)));
        }
        prior_port = Some(key);
    }
    let mut prior_observation = None;
    for index in 0..observations as usize {
        let record = observation_offset as usize + index * OBSERVATION_BYTES;
        let id = read_u32(bytes, record);
        if prior_observation.is_some_and(|prior| id <= prior) {
            return Err(diagnostic(Code::Order, record, Some(index)));
        }
        prior_observation = Some(id);
        take_text(
            bytes,
            read_u32(bytes, record + 20),
            u32::from(bytes[record + 10]),
            &mut string_cursor,
            record + 20,
            Some(index),
        )?;
        take_text(
            bytes,
            read_u32(bytes, record + 24),
            u32::from(bytes[record + 11]),
            &mut string_cursor,
            record + 24,
            Some(index),
        )?;
    }
    let mut prior_quality = None;
    for index in 0..qualities as usize {
        let record = quality_offset as usize + index * QUALITY_BYTES;
        let key = (read_u32(bytes, record), read_u32(bytes, record + 4));
        if prior_quality.is_some_and(|prior| prior >= key) {
            return Err(diagnostic(Code::Order, record, Some(index)));
        }
        prior_quality = Some(key);
    }
    if string_cursor != total {
        return Err(diagnostic(Code::Offset, 80, None));
    }

    // Phase 7: all scalar enum representations.
    let link_modes =
        LinkModeSet::new(read_u32(bytes, 28)).ok_or_else(|| diagnostic(Code::Enum, 28, None))?;
    for index in 0..parameters as usize {
        let record = parameter_offset as usize + index * PARAMETER_BYTES;
        let fields = [
            (
                4,
                ParameterUnit::from_raw(read_u32(bytes, record + 4)).is_some(),
            ),
            (
                8,
                ParameterDomain::from_raw(read_u32(bytes, record + 8)).is_some(),
            ),
            (
                12,
                ParameterMapping::from_raw(read_u32(bytes, record + 12)).is_some(),
            ),
            (
                16,
                AutomationRate::from_raw(read_u32(bytes, record + 16)).is_some(),
            ),
            (
                20,
                ParameterChannelPolicy::from_raw(read_u32(bytes, record + 20)).is_some(),
            ),
            (
                24,
                SmoothingRule::from_raw(read_u32(bytes, record + 24)).is_some(),
            ),
        ];
        if let Some((field, _)) = fields.into_iter().find(|(_, valid)| !valid) {
            return Err(diagnostic(Code::Enum, record + field, Some(index)));
        }
    }
    for index in 0..ports as usize {
        let record = port_offset as usize + index * PORT_BYTES;
        if PortRole::from_raw(read_u32(bytes, record + 8)).is_none() {
            return Err(diagnostic(Code::Enum, record + 8, Some(index)));
        }
        if read_u32(bytes, record + 12) > 1 {
            return Err(diagnostic(Code::Enum, record + 12, Some(index)));
        }
        if PortLayout::from_raw(read_u32(bytes, record + 16)).is_none() {
            return Err(diagnostic(Code::Enum, record + 16, Some(index)));
        }
    }
    for index in 0..qualities as usize {
        let record = quality_offset as usize + index * QUALITY_BYTES;
        if EffectQuality::from_raw(read_u32(bytes, record)).is_none() {
            return Err(diagnostic(Code::Enum, record, Some(index)));
        }
        match read_u32(bytes, record + 16) {
            1 => {}
            2 if read_u64(bytes, record + 24) == 0 => {}
            2 => return Err(diagnostic(Code::Enum, record + 24, Some(index))),
            _ => return Err(diagnostic(Code::Enum, record + 16, Some(index))),
        }
    }

    for index in 0..observations as usize {
        let record = observation_offset as usize + index * OBSERVATION_BYTES;
        let fields = [
            (
                4,
                ObservationKindV1::from_raw(u32::from(bytes[record + 4])).is_some(),
            ),
            (
                5,
                ParameterUnit::from_raw(u32::from(bytes[record + 5])).is_some(),
            ),
            (
                6,
                ObservationCostV1::from_raw(u32::from(bytes[record + 6])).is_some(),
            ),
            (
                7,
                ObservationCadenceV1::from_raw(u32::from(bytes[record + 7])).is_some(),
            ),
            (
                8,
                ObservationFoldV1::from_raw(u32::from(bytes[record + 8])).is_some(),
            ),
            (
                9,
                ObservationChannelsV1::from_raw(u32::from(bytes[record + 9])).is_some(),
            ),
        ];
        if let Some((field, _)) = fields.into_iter().find(|(_, valid)| !valid) {
            return Err(diagnostic(Code::Enum, record + field, Some(index)));
        }
    }

    // Phase 8: UTF-8, control scalars, lengths, and constructor-equivalent ID grammar.
    let effect_id =
        core::str::from_utf8(effect_id_bytes).map_err(|_| diagnostic(Code::Text, 32, None))?;
    let display_name =
        core::str::from_utf8(display_name_bytes).map_err(|_| diagnostic(Code::Text, 40, None))?;
    if !valid_id(effect_id) {
        return Err(diagnostic(Code::Text, 32, None));
    }
    if !valid_text(display_name) {
        return Err(diagnostic(Code::Text, 40, None));
    }
    for index in 0..parameters as usize {
        let record = parameter_offset as usize + index * PARAMETER_BYTES;
        for field in [56, 64] {
            let offset = read_u32(bytes, record + field) as usize;
            let length = read_u32(bytes, record + field + 4) as usize;
            let value = core::str::from_utf8(&bytes[offset..offset + length])
                .map_err(|_| diagnostic(Code::Text, record + field, Some(index)))?;
            if !valid_text(value) {
                return Err(diagnostic(Code::Text, record + field, Some(index)));
            }
        }
    }
    for index in 0..ports as usize {
        let record = port_offset as usize + index * PORT_BYTES;
        let offset = read_u32(bytes, record) as usize;
        let length = read_u32(bytes, record + 4) as usize;
        let value = core::str::from_utf8(&bytes[offset..offset + length])
            .map_err(|_| diagnostic(Code::Text, record, Some(index)))?;
        if !valid_id(value) {
            return Err(diagnostic(Code::Text, record, Some(index)));
        }
    }
    for index in 0..parameters as usize {
        let record = parameter_offset as usize + index * PARAMETER_BYTES;
        let start = read_u32(bytes, record + 48) as usize;
        let count = read_u32(bytes, record + 52) as usize;
        for choice_index in start..start + count {
            let choice_record = choice_offset as usize + choice_index * ENUM_CHOICE_BYTES;
            let offset = read_u32(bytes, choice_record + 4) as usize;
            let length = read_u32(bytes, choice_record + 8) as usize;
            let value = core::str::from_utf8(&bytes[offset..offset + length])
                .map_err(|_| diagnostic(Code::Text, choice_record + 4, Some(choice_index)))?;
            if !valid_text(value) {
                return Err(diagnostic(
                    Code::Text,
                    choice_record + 4,
                    Some(choice_index),
                ));
            }
        }
    }

    for index in 0..observations as usize {
        let record = observation_offset as usize + index * OBSERVATION_BYTES;
        for (field, length_field) in [(20, 10), (24, 11)] {
            let offset = read_u32(bytes, record + field) as usize;
            let length = usize::from(bytes[record + length_field]);
            let value = core::str::from_utf8(&bytes[offset..offset + length])
                .map_err(|_| diagnostic(Code::Text, record + field, Some(index)))?;
            if !valid_text(value) {
                return Err(diagnostic(Code::Text, record + field, Some(index)));
            }
        }
    }

    // Phase 9: canonical finite f32 bit patterns.
    for index in 0..parameters as usize {
        let record = parameter_offset as usize + index * PARAMETER_BYTES;
        let flags = read_u32(bytes, record + 32);
        for (field, present) in [(36, flags & 4 != 0), (40, flags & 8 != 0), (44, true)] {
            if present && !canonical_float(f32::from_bits(read_u32(bytes, record + field))) {
                return Err(diagnostic(Code::Float, record + field, Some(index)));
            }
        }
    }
    for index in 0..choices as usize {
        let record = choice_offset as usize + index * ENUM_CHOICE_BYTES;
        if !canonical_float(f32::from_bits(read_u32(bytes, record))) {
            return Err(diagnostic(Code::Float, record, Some(index)));
        }
    }

    for index in 0..observations as usize {
        let record = observation_offset as usize + index * OBSERVATION_BYTES;
        for field in [12, 16] {
            if !canonical_float(f32::from_bits(read_u32(bytes, record + field))) {
                return Err(diagnostic(Code::Float, record + field, Some(index)));
            }
        }
    }

    Ok(BorrowedEffectDescriptorView {
        bytes,
        layout,
        effect_id,
        display_name,
        contract_major: read_u16(bytes, 20),
        state_layout_version: read_u32(bytes, 24),
        link_modes,
    })
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct BorrowedSemanticError {
    path: &'static str,
    code: DescriptorDiagnosticCode,
    byte_offset: usize,
    record_index: Option<usize>,
}

fn parameter_value_valid(
    view: BorrowedEffectDescriptorView<'_>,
    parameter: BorrowedParameter<'_>,
    value: f32,
) -> bool {
    if !value.is_finite() {
        return false;
    }
    match parameter.domain {
        ParameterDomain::Continuous => parameter
            .minimum
            .zip(parameter.maximum)
            .is_some_and(|(minimum, maximum)| value >= minimum && value <= maximum),
        ParameterDomain::Boolean => matches!(value.to_bits(), 0 | 0x3f80_0000),
        ParameterDomain::Enumeration => (parameter.choice_start
            ..parameter.choice_start + parameter.choice_count)
            .any(|index| view.choice(index as usize).0.to_bits() == value.to_bits()),
    }
}

fn parameter_semantics_valid(
    view: BorrowedEffectDescriptorView<'_>,
    parameter: BorrowedParameter<'_>,
) -> bool {
    if !valid_text(parameter.display_name)
        || !valid_text(parameter.display_unit)
        || !canonical_float(parameter.default_value)
    {
        return false;
    }
    if parameter.automation_rate == AutomationRate::None {
        if parameter.automatable || parameter.smoothing != SmoothingRule::None {
            return false;
        }
    } else if !parameter.automatable {
        return false;
    }
    if (parameter.smoothing == SmoothingRule::None) != (parameter.smoothing_samples == 0) {
        return false;
    }
    match parameter.domain {
        ParameterDomain::Continuous => match (parameter.minimum, parameter.maximum) {
            (Some(minimum), Some(maximum)) => {
                canonical_float(minimum)
                    && canonical_float(maximum)
                    && minimum < maximum
                    && parameter_value_valid(view, parameter, parameter.default_value)
                    && parameter.choice_count == 0
                    && matches!(
                        parameter.mapping,
                        ParameterMapping::Linear
                            | ParameterMapping::Logarithmic
                            | ParameterMapping::Exponential
                    )
                    && (parameter.mapping != ParameterMapping::Logarithmic || minimum > 0.0)
            }
            _ => false,
        },
        ParameterDomain::Boolean => {
            parameter.minimum.is_none()
                && parameter.maximum.is_none()
                && parameter.choice_count == 0
                && parameter.mapping == ParameterMapping::Stepped
                && parameter_value_valid(view, parameter, parameter.default_value)
        }
        ParameterDomain::Enumeration => {
            let mut labels = BTreeSet::new();
            let choices: Vec<_> = (parameter.choice_start
                ..parameter.choice_start + parameter.choice_count)
                .map(|index| view.choice(index as usize))
                .collect();
            parameter.minimum.is_none()
                && parameter.maximum.is_none()
                && parameter.mapping == ParameterMapping::Stepped
                && choices.len() >= 2
                && choices.windows(2).all(|window| {
                    canonical_float(window[0].0)
                        && canonical_float(window[1].0)
                        && window[0].0 < window[1].0
                        && valid_text(window[0].1)
                        && labels.insert(window[0].1)
                })
                && choices.last().is_some_and(|choice| {
                    canonical_float(choice.0) && valid_text(choice.1) && labels.insert(choice.1)
                })
                && parameter_value_valid(view, parameter, parameter.default_value)
        }
    }
}

fn borrowed_semantic_errors(
    view: BorrowedEffectDescriptorView<'_>,
) -> Vec<BorrowedSemanticError> {
    let mut errors = Vec::new();
    let mut push = |path, code, byte_offset, record_index| {
        errors.push(BorrowedSemanticError {
            path,
            code,
            byte_offset,
            record_index,
        });
    };
    if view.contract_major != 1 {
        push(
            "descriptor.contract_major",
            DescriptorDiagnosticCode::ContractMajor,
            20,
            None,
        );
    }
    if view.state_layout_version == 0 {
        push(
            "descriptor.state_layout_version",
            DescriptorDiagnosticCode::StateLayoutVersion,
            24,
            None,
        );
    }
    if !valid_id(view.effect_id) || !valid_text(view.display_name) {
        push("descriptor", DescriptorDiagnosticCode::Text, 32, None);
    }
    if !view.link_modes.contains(LinkMode::DualMono) {
        push(
            "descriptor.supported_link_modes",
            DescriptorDiagnosticCode::LinkModes,
            28,
            None,
        );
    }

    let mut prior_parameter = 0;
    let mut parameter_ids = BTreeSet::new();
    for index in 0..view.layout.parameters as usize {
        let parameter = view.parameter(index);
        let record = view.layout.parameter_offset as usize + index * PARAMETER_BYTES;
        if parameter.id == 0 || !parameter_ids.insert(parameter.id) {
            push(
                "parameters",
                DescriptorDiagnosticCode::ParameterId,
                record,
                Some(index),
            );
        }
        if parameter.id <= prior_parameter {
            push(
                "parameters",
                DescriptorDiagnosticCode::ParameterOrder,
                record,
                Some(index),
            );
        }
        prior_parameter = parameter.id;
        if !parameter_semantics_valid(view, parameter) {
            push(
                "parameters",
                DescriptorDiagnosticCode::Parameter,
                record + 4,
                Some(index),
            );
        }
    }

    let mut port_ids = BTreeSet::new();
    let (mut main_inputs, mut main_outputs, mut sidechains) = (0, 0, 0);
    for index in 0..view.layout.ports as usize {
        let port = view.port(index);
        let record = view.layout.port_offset as usize + index * PORT_BYTES;
        if !valid_id(port.id) || !port_ids.insert(port.id) {
            push("ports", DescriptorDiagnosticCode::Port, record, Some(index));
        }
        match port.role {
            PortRole::MainInput
                if port.id == "main-in"
                    && port.required
                    && port.layout == PortLayout::DualMonoPlanar =>
            {
                main_inputs += 1;
            }
            PortRole::MainOutput
                if port.id == "main-out"
                    && port.required
                    && port.layout == PortLayout::DualMonoPlanar =>
            {
                main_outputs += 1;
            }
            PortRole::SidechainInput
                if port.id != "main-in"
                    && port.id != "main-out"
                    && port.layout == PortLayout::DualMonoPlanar =>
            {
                sidechains += 1;
            }
            _ => push(
                "ports",
                DescriptorDiagnosticCode::Port,
                record + 8,
                Some(index),
            ),
        }
    }
    if main_inputs != 1
        || main_outputs != 1
        || sidechains > 1
        || view.layout.ports as usize != 2 + sidechains
    {
        push(
            "ports",
            DescriptorDiagnosticCode::Port,
            view.layout.port_offset as usize,
            None,
        );
    }

    let mut prior_quality = None;
    let mut quality_rates = BTreeMap::<EffectQuality, BTreeSet<u32>>::new();
    for index in 0..view.layout.qualities as usize {
        let quality = view.quality(index);
        let record = view.layout.quality_offset as usize + index * QUALITY_BYTES;
        let key = (quality.quality, quality.sample_rate);
        if prior_quality.is_some_and(|prior| key <= prior) {
            push(
                "qualities",
                DescriptorDiagnosticCode::QualityOrder,
                record,
                Some(index),
            );
        }
        prior_quality = Some(key);
        let rate = SampleRateHz(quality.sample_rate);
        if !(is_launch_sample_rate(rate) || is_extended_compatibility_sample_rate(rate)) {
            push(
                "qualities",
                DescriptorDiagnosticCode::Quality,
                record + 4,
                Some(index),
            );
        }
        if quality.left_bytes != quality.right_bytes {
            push(
                "qualities",
                DescriptorDiagnosticCode::StateSizes,
                record + 36,
                Some(index),
            );
        }
        quality_rates
            .entry(quality.quality)
            .or_default()
            .insert(quality.sample_rate);
    }
    if !quality_rates.contains_key(&EffectQuality::Normal) {
        push(
            "qualities",
            DescriptorDiagnosticCode::Quality,
            view.layout.quality_offset as usize,
            None,
        );
    }
    for rates in quality_rates.values() {
        if LAUNCH_SAMPLE_RATES
            .iter()
            .any(|sample_rate| !rates.contains(&sample_rate.0))
        {
            push(
                "qualities",
                DescriptorDiagnosticCode::Quality,
                view.layout.quality_offset as usize,
                None,
            );
        }
    }
    // Issue #143: the borrowed mirror of `validate_descriptor`'s observation rules. The
    // per-lane rule reads the *decoded* qualities rather than the static descriptor, so external
    // wire is judged by its own bytes and never by a descriptor it claims to be.
    let per_lane_state =
        (0..view.layout.qualities as usize).all(|index| view.quality(index).left_bytes > 0);
    let mut prior_observation = 0;
    let mut observation_ids = BTreeSet::new();
    for index in 0..view.layout.observations as usize {
        let observation = view.observation(index);
        let record = view.layout.observation_offset as usize + index * OBSERVATION_BYTES;
        if observation.id == 0 || !observation_ids.insert(observation.id) {
            push(
                "observations",
                DescriptorDiagnosticCode::ObservationId,
                record,
                Some(index),
            );
        }
        if observation.id <= prior_observation {
            push(
                "observations",
                DescriptorDiagnosticCode::ObservationOrder,
                record,
                Some(index),
            );
        }
        prior_observation = observation.id;
        let valid = valid_text(observation.display_name)
            && valid_text(observation.display_unit)
            && canonical_float(observation.minimum)
            && canonical_float(observation.maximum)
            && observation.minimum < observation.maximum
            && !(matches!(observation.cost, ObservationCostV1::Computed)
                && matches!(observation.cadence, ObservationCadenceV1::PerBlock))
            && (!matches!(observation.channels, ObservationChannelsV1::PerLane) || per_lane_state);
        if !valid {
            push(
                "observations",
                DescriptorDiagnosticCode::Observation,
                record + 4,
                Some(index),
            );
        }
    }
    errors.sort();
    errors.dedup_by(|left, right| left.path == right.path && left.code == right.code);
    errors
}

fn semantic_mismatch(byte_offset: usize, record_index: Option<usize>) -> Diagnostic {
    diagnostic(Code::Semantic, byte_offset, record_index)
}

fn compare_static_descriptor(
    view: BorrowedEffectDescriptorView<'_>,
    descriptor: &'static EffectDescriptor,
) -> Result<(), Diagnostic> {
    if read_u16(view.bytes, 20) != descriptor.contract_major {
        return Err(semantic_mismatch(20, None));
    }
    if read_u16(view.bytes, 22) != descriptor.contract_minor {
        return Err(semantic_mismatch(22, None));
    }
    if read_u32(view.bytes, 24) != descriptor.state_layout_version {
        return Err(semantic_mismatch(24, None));
    }
    if read_u32(view.bytes, 28) != descriptor.supported_link_modes.bits() {
        return Err(semantic_mismatch(28, None));
    }
    if view.effect_id != descriptor.id.as_str() {
        return Err(semantic_mismatch(32, None));
    }
    if view.display_name != descriptor.display_name {
        return Err(semantic_mismatch(40, None));
    }
    if view.layout.parameters as usize != descriptor.parameters.len() {
        return Err(semantic_mismatch(48, None));
    }
    if view.layout.ports as usize != descriptor.ports.len() {
        return Err(semantic_mismatch(56, None));
    }
    if view.layout.qualities as usize != descriptor.qualities.len() {
        return Err(semantic_mismatch(64, None));
    }
    let descriptor_choices = descriptor
        .parameters
        .iter()
        .try_fold(0usize, |total, parameter| {
            total.checked_add(parameter.enum_choices.len())
        })
        .ok_or_else(|| semantic_mismatch(72, None))?;
    if view.layout.choices as usize != descriptor_choices {
        return Err(semantic_mismatch(72, None));
    }
    if view.layout.observations as usize != descriptor.observations.len() {
        return Err(semantic_mismatch(88, None));
    }

    let mut choice_index = 0usize;
    for (index, parameter) in descriptor.parameters.iter().enumerate() {
        let record = view.layout.parameter_offset as usize + index * PARAMETER_BYTES;
        let flags = u32::from(parameter.readable)
            | (u32::from(parameter.automatable) << 1)
            | (u32::from(parameter.minimum.is_some()) << 2)
            | (u32::from(parameter.maximum.is_some()) << 3);
        let scalar_fields = [
            (0, parameter.id.0),
            (4, parameter.unit as u32),
            (8, parameter.domain as u32),
            (12, parameter.mapping as u32),
            (16, parameter.automation_rate as u32),
            (20, parameter.channel_policy as u32),
            (24, parameter.smoothing as u32),
            (28, parameter.smoothing_samples),
            (32, flags),
            (36, parameter.minimum.unwrap_or(0.0).to_bits()),
            (40, parameter.maximum.unwrap_or(0.0).to_bits()),
            (44, parameter.default_value.to_bits()),
            (48, choice_index as u32),
            (52, parameter.enum_choices.len() as u32),
        ];
        if let Some((field, _)) = scalar_fields
            .into_iter()
            .find(|(field, expected)| read_u32(view.bytes, record + field) != *expected)
        {
            return Err(semantic_mismatch(record + field, Some(index)));
        }
        let borrowed = view.parameter(index);
        if borrowed.display_name != parameter.display_name {
            return Err(semantic_mismatch(record + 56, Some(index)));
        }
        if borrowed.display_unit != parameter.display_unit {
            return Err(semantic_mismatch(record + 64, Some(index)));
        }
        choice_index += parameter.enum_choices.len();
    }

    for index in 0..descriptor.ports.len() {
        let expected = canonical_port_at(descriptor.ports, index);
        let actual = view.port(index);
        let record = view.layout.port_offset as usize + index * PORT_BYTES;
        if actual.id != expected.id.as_str() {
            return Err(semantic_mismatch(record, Some(index)));
        }
        if actual.role != expected.role {
            return Err(semantic_mismatch(record + 8, Some(index)));
        }
        if actual.required != expected.required {
            return Err(semantic_mismatch(record + 12, Some(index)));
        }
        if actual.layout != expected.layout {
            return Err(semantic_mismatch(record + 16, Some(index)));
        }
    }

    for (index, quality) in descriptor.qualities.iter().enumerate() {
        let record = view.layout.quality_offset as usize + index * QUALITY_BYTES;
        let (tail_kind, tail_samples) = match quality.tail {
            TailSamples::Finite(samples) => (1, samples),
            TailSamples::Infinite => (2, 0),
        };
        let u32_fields = [
            (0, quality.quality as u32),
            (4, quality.sample_rate),
            (16, tail_kind),
            (32, quality.maximum_state.common_bytes),
            (36, quality.maximum_state.left_bytes),
            (40, quality.maximum_state.right_bytes),
        ];
        if let Some((field, _)) = u32_fields
            .into_iter()
            .find(|(field, expected)| read_u32(view.bytes, record + field) != *expected)
        {
            return Err(semantic_mismatch(record + field, Some(index)));
        }
        let u64_fields = [
            (8, quality.latency.0),
            (24, tail_samples),
            (48, quality.scratch_fixed_bytes),
            (56, quality.scratch_bytes_per_frame),
        ];
        if let Some((field, _)) = u64_fields
            .into_iter()
            .find(|(field, expected)| read_u64(view.bytes, record + field) != *expected)
        {
            return Err(semantic_mismatch(record + field, Some(index)));
        }
    }

    for (index, observation) in descriptor.observations.iter().enumerate() {
        let record = view.layout.observation_offset as usize + index * OBSERVATION_BYTES;
        if read_u32(view.bytes, record) != observation.id.0 {
            return Err(semantic_mismatch(record, Some(index)));
        }
        let scalar_fields = [
            (4, observation.kind as u8),
            (5, observation.unit as u8),
            (6, observation.cost as u8),
            (7, observation.cadence as u8),
            (8, observation.fold as u8),
            (9, observation.channels as u8),
            (10, observation.display_name.len() as u8),
            (11, observation.display_unit.len() as u8),
        ];
        if let Some((field, _)) = scalar_fields
            .into_iter()
            .find(|(field, expected)| view.bytes[record + field] != *expected)
        {
            return Err(semantic_mismatch(record + field, Some(index)));
        }
        if read_u32(view.bytes, record + 12) != observation.minimum.to_bits() {
            return Err(semantic_mismatch(record + 12, Some(index)));
        }
        if read_u32(view.bytes, record + 16) != observation.maximum.to_bits() {
            return Err(semantic_mismatch(record + 16, Some(index)));
        }
        let borrowed = view.observation(index);
        if borrowed.display_name != observation.display_name {
            return Err(semantic_mismatch(record + 20, Some(index)));
        }
        if borrowed.display_unit != observation.display_unit {
            return Err(semantic_mismatch(record + 24, Some(index)));
        }
    }

    let mut choice_index = 0usize;
    for parameter in descriptor.parameters {
        for choice in parameter.enum_choices {
            let choice_record =
                view.layout.choice_offset as usize + choice_index * ENUM_CHOICE_BYTES;
            let borrowed = view.choice(choice_index);
            if borrowed.0.to_bits() != choice.value.to_bits() {
                return Err(semantic_mismatch(choice_record, Some(choice_index)));
            }
            if borrowed.1 != choice.label {
                return Err(semantic_mismatch(choice_record + 4, Some(choice_index)));
            }
            choice_index += 1;
        }
    }
    Ok(())
}

fn descriptor_identity(bytes: &[u8]) -> EffectDescriptorIdentity {
    let mut hasher = Sha256::new();
    hasher.update(IDENTITY_DOMAIN);
    hasher.update((bytes.len() as u64).to_le_bytes());
    hasher.update(bytes);
    EffectDescriptorIdentity(hasher.finalize().into())
}

pub fn bind_effect_descriptor_wire<'a>(
    descriptor: &'static EffectDescriptor,
    wire: &'a [u8],
    maximum_descriptor_bytes: u32,
) -> Result<BoundEffectDescriptorWire<'a>, EffectDescriptorBindingError> {
    validate_descriptor(descriptor).map_err(|_| EffectDescriptorBindingError {
        kind: EffectDescriptorBindingErrorKind::StaticDescriptorMismatch,
        diagnostic: semantic_mismatch(0, None),
    })?;
    let view = parse_borrowed_wire(wire, maximum_descriptor_bytes).map_err(|diagnostic| {
        EffectDescriptorBindingError {
            kind: EffectDescriptorBindingErrorKind::ExternalWire,
            diagnostic,
        }
    })?;
    if let Some(error) = borrowed_semantic_errors(view)
        .into_iter()
        .min_by_key(|error| (error.byte_offset, error.record_index))
    {
        return Err(EffectDescriptorBindingError {
            kind: EffectDescriptorBindingErrorKind::ExternalWire,
            diagnostic: diagnostic(Code::Semantic, error.byte_offset, error.record_index),
        });
    }
    compare_static_descriptor(view, descriptor).map_err(|diagnostic| {
        EffectDescriptorBindingError {
            kind: EffectDescriptorBindingErrorKind::StaticDescriptorMismatch,
            diagnostic,
        }
    })?;
    Ok(BoundEffectDescriptorWire {
        descriptor,
        wire,
        identity: descriptor_identity(wire),
    })
}

pub fn verify_effect_descriptor_wire(
    bytes: &[u8],
    maximum_descriptor_bytes: u32,
) -> Result<VerifiedEffectDescriptorWire<'_>, Diagnostic> {
    let view = parse_borrowed_wire(bytes, maximum_descriptor_bytes)?;
    if let Some(error) = borrowed_semantic_errors(view)
        .into_iter()
        .min_by_key(|error| (error.byte_offset, error.record_index))
    {
        return Err(diagnostic(
            Code::Semantic,
            error.byte_offset,
            error.record_index,
        ));
    }
    Ok(VerifiedEffectDescriptorWire {
        bytes,
        parameter_count: view.layout.parameters,
        port_count: view.layout.ports,
        quality_count: view.layout.qualities,
        enum_choice_count: view.layout.choices,
        observation_count: view.layout.observations,
        state_layout_version: view.state_layout_version,
        supported_link_mode_bits: view.link_modes.bits(),
    })
}

pub fn effect_descriptor_identity(
    bytes: &[u8],
    maximum_descriptor_bytes: u32,
) -> Result<EffectDescriptorIdentity, Diagnostic> {
    Ok(verify_effect_descriptor_wire(bytes, maximum_descriptor_bytes)?.identity())
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_effect_contract::{
        DescriptorError, EffectId, EnumChoice, LatencySamples, ParameterDescriptor,
        ParameterId, PortDescriptor, PortId, QualityDescriptor, StatePayloadSizes,
    };

    const fn effect_id(value: &'static str) -> EffectId {
        match EffectId::new(value) {
            Ok(value) => value,
            Err(_) => panic!("valid static effect ID"),
        }
    }

    const fn port_id(value: &'static str) -> PortId {
        match PortId::new(value) {
            Ok(value) => value,
            Err(_) => panic!("valid static port ID"),
        }
    }

    static CHOICES: [EnumChoice; 2] = [
        EnumChoice {
            value: 0.0,
            label: "No",
        },
        EnumChoice {
            value: 1.0,
            label: "Up",
        },
    ];
    static DUPLICATE_LABEL_CHOICES: [EnumChoice; 2] = [
        EnumChoice {
            value: 0.0,
            label: "No",
        },
        EnumChoice {
            value: 1.0,
            label: "No",
        },
    ];
    static OUT_OF_ORDER_CHOICES: [EnumChoice; 2] = [CHOICES[1], CHOICES[0]];
    static NONFINITE_CHOICES: [EnumChoice; 2] = [
        EnumChoice {
            value: f32::NAN,
            label: "No",
        },
        CHOICES[1],
    ];
    static CONTROL_LABEL_CHOICES: [EnumChoice; 2] = [
        EnumChoice {
            label: "\n",
            ..CHOICES[0]
        },
        CHOICES[1],
    ];
    static PARAMETERS: [ParameterDescriptor; 3] = [
        ParameterDescriptor {
            id: ParameterId(1),
            display_name: "Gain",
            display_unit: "dB",
            unit: ParameterUnit::Db,
            domain: ParameterDomain::Continuous,
            minimum: Some(-24.0),
            maximum: Some(24.0),
            default_value: 0.0,
            mapping: ParameterMapping::Linear,
            automation_rate: AutomationRate::Sample,
            channel_policy: ParameterChannelPolicy::PerLane,
            smoothing: SmoothingRule::Linear,
            smoothing_samples: 64,
            readable: false,
            automatable: true,
            enum_choices: &[],
        },
        ParameterDescriptor {
            id: ParameterId(2),
            display_name: "Bypass",
            display_unit: "state",
            unit: ParameterUnit::Samples,
            domain: ParameterDomain::Boolean,
            minimum: None,
            maximum: None,
            default_value: 0.0,
            mapping: ParameterMapping::Stepped,
            automation_rate: AutomationRate::Block,
            channel_policy: ParameterChannelPolicy::Shared,
            smoothing: SmoothingRule::None,
            smoothing_samples: 0,
            readable: true,
            automatable: true,
            enum_choices: &[],
        },
        ParameterDescriptor {
            id: ParameterId(3),
            display_name: "Mode",
            display_unit: "choice",
            unit: ParameterUnit::Linear,
            domain: ParameterDomain::Enumeration,
            minimum: None,
            maximum: None,
            default_value: 1.0,
            mapping: ParameterMapping::Stepped,
            automation_rate: AutomationRate::None,
            channel_policy: ParameterChannelPolicy::Shared,
            smoothing: SmoothingRule::None,
            smoothing_samples: 0,
            readable: true,
            automatable: false,
            enum_choices: &CHOICES,
        },
    ];
    static ZERO_ID_PARAMETERS: [ParameterDescriptor; 3] = [
        ParameterDescriptor {
            id: ParameterId(0),
            ..PARAMETERS[0]
        },
        PARAMETERS[1],
        PARAMETERS[2],
    ];
    static BAD_DEFAULT_PARAMETERS: [ParameterDescriptor; 3] = [
        ParameterDescriptor {
            default_value: 25.0,
            ..PARAMETERS[0]
        },
        PARAMETERS[1],
        PARAMETERS[2],
    ];
    static BAD_BOOLEAN_PARAMETERS: [ParameterDescriptor; 3] = [
        PARAMETERS[0],
        ParameterDescriptor {
            minimum: Some(0.0),
            ..PARAMETERS[1]
        },
        PARAMETERS[2],
    ];
    static BAD_SMOOTHING_PARAMETERS: [ParameterDescriptor; 3] = [
        ParameterDescriptor {
            smoothing_samples: 0,
            ..PARAMETERS[0]
        },
        PARAMETERS[1],
        PARAMETERS[2],
    ];
    static BAD_LABEL_PARAMETERS: [ParameterDescriptor; 3] = [
        PARAMETERS[0],
        PARAMETERS[1],
        ParameterDescriptor {
            enum_choices: &DUPLICATE_LABEL_CHOICES,
            ..PARAMETERS[2]
        },
    ];
    static DUPLICATE_ID_PARAMETERS: [ParameterDescriptor; 3] = [
        PARAMETERS[0],
        ParameterDescriptor {
            id: ParameterId(1),
            ..PARAMETERS[1]
        },
        PARAMETERS[2],
    ];
    static OUT_OF_ORDER_PARAMETERS: [ParameterDescriptor; 3] =
        [PARAMETERS[1], PARAMETERS[0], PARAMETERS[2]];
    static MISSING_MINIMUM_PARAMETERS: [ParameterDescriptor; 3] = [
        ParameterDescriptor {
            minimum: None,
            ..PARAMETERS[0]
        },
        PARAMETERS[1],
        PARAMETERS[2],
    ];
    static BAD_LOG_PARAMETERS: [ParameterDescriptor; 3] = [
        ParameterDescriptor {
            mapping: ParameterMapping::Logarithmic,
            ..PARAMETERS[0]
        },
        PARAMETERS[1],
        PARAMETERS[2],
    ];
    static BAD_BOOLEAN_DEFAULT_PARAMETERS: [ParameterDescriptor; 3] = [
        PARAMETERS[0],
        ParameterDescriptor {
            default_value: 0.5,
            ..PARAMETERS[1]
        },
        PARAMETERS[2],
    ];
    static SHORT_ENUM_PARAMETERS: [ParameterDescriptor; 3] = [
        PARAMETERS[0],
        PARAMETERS[1],
        ParameterDescriptor {
            enum_choices: &[CHOICES[0]],
            ..PARAMETERS[2]
        },
    ];
    static BAD_ENUM_DEFAULT_PARAMETERS: [ParameterDescriptor; 3] = [
        PARAMETERS[0],
        PARAMETERS[1],
        ParameterDescriptor {
            default_value: 2.0,
            ..PARAMETERS[2]
        },
    ];
    static BAD_AUTOMATION_PARAMETERS: [ParameterDescriptor; 3] = [
        ParameterDescriptor {
            automatable: false,
            ..PARAMETERS[0]
        },
        PARAMETERS[1],
        ParameterDescriptor {
            automatable: true,
            ..PARAMETERS[2]
        },
    ];
    static NONFINITE_PARAMETERS: [ParameterDescriptor; 3] = [
        ParameterDescriptor {
            minimum: Some(f32::NAN),
            ..PARAMETERS[0]
        },
        PARAMETERS[1],
        PARAMETERS[2],
    ];
    static NEGATIVE_ZERO_PARAMETERS: [ParameterDescriptor; 3] = [
        ParameterDescriptor {
            default_value: -0.0,
            ..PARAMETERS[0]
        },
        PARAMETERS[1],
        PARAMETERS[2],
    ];
    static REVERSED_BOUND_PARAMETERS: [ParameterDescriptor; 3] = [
        ParameterDescriptor {
            minimum: Some(24.0),
            maximum: Some(-24.0),
            ..PARAMETERS[0]
        },
        PARAMETERS[1],
        PARAMETERS[2],
    ];
    static CONTINUOUS_CHOICES_PARAMETERS: [ParameterDescriptor; 3] = [
        ParameterDescriptor {
            enum_choices: &CHOICES,
            ..PARAMETERS[0]
        },
        PARAMETERS[1],
        PARAMETERS[2],
    ];
    static BOOLEAN_MAPPING_PARAMETERS: [ParameterDescriptor; 3] = [
        PARAMETERS[0],
        ParameterDescriptor {
            mapping: ParameterMapping::Linear,
            ..PARAMETERS[1]
        },
        PARAMETERS[2],
    ];
    static OUT_OF_ORDER_ENUM_PARAMETERS: [ParameterDescriptor; 3] = [
        PARAMETERS[0],
        PARAMETERS[1],
        ParameterDescriptor {
            enum_choices: &OUT_OF_ORDER_CHOICES,
            ..PARAMETERS[2]
        },
    ];
    static NONFINITE_ENUM_PARAMETERS: [ParameterDescriptor; 3] = [
        PARAMETERS[0],
        PARAMETERS[1],
        ParameterDescriptor {
            enum_choices: &NONFINITE_CHOICES,
            ..PARAMETERS[2]
        },
    ];
    static CONTROL_LABEL_PARAMETERS: [ParameterDescriptor; 3] = [
        PARAMETERS[0],
        PARAMETERS[1],
        ParameterDescriptor {
            enum_choices: &CONTROL_LABEL_CHOICES,
            ..PARAMETERS[2]
        },
    ];

    static PORTS_UNSORTED: [PortDescriptor; 3] = [
        PortDescriptor {
            id: port_id("sidechain"),
            role: PortRole::SidechainInput,
            required: false,
            layout: PortLayout::DualMonoPlanar,
        },
        PortDescriptor {
            id: port_id("main-out"),
            role: PortRole::MainOutput,
            required: true,
            layout: PortLayout::DualMonoPlanar,
        },
        PortDescriptor {
            id: port_id("main-in"),
            role: PortRole::MainInput,
            required: true,
            layout: PortLayout::DualMonoPlanar,
        },
    ];
    static PORTS_SORTED: [PortDescriptor; 3] =
        [PORTS_UNSORTED[2], PORTS_UNSORTED[1], PORTS_UNSORTED[0]];
    static BAD_PORTS: [PortDescriptor; 3] = [
        PortDescriptor {
            required: false,
            ..PORTS_SORTED[0]
        },
        PORTS_SORTED[1],
        PORTS_SORTED[2],
    ];
    static DUPLICATE_PORTS: [PortDescriptor; 3] = [
        PORTS_SORTED[0],
        PortDescriptor {
            id: PORTS_SORTED[0].id,
            ..PORTS_SORTED[1]
        },
        PORTS_SORTED[2],
    ];
    static MISSING_OUTPUT_PORTS: [PortDescriptor; 1] = [PORTS_SORTED[0]];
    static TWO_SIDECHAIN_PORTS: [PortDescriptor; 4] = [
        PORTS_SORTED[0],
        PORTS_SORTED[1],
        PORTS_SORTED[2],
        PortDescriptor {
            id: port_id("key-input"),
            ..PORTS_SORTED[2]
        },
    ];

    const fn quality(sample_rate: u32) -> QualityDescriptor {
        QualityDescriptor {
            quality: EffectQuality::Normal,
            sample_rate,
            latency: LatencySamples(4),
            tail: if sample_rate > 96_000 {
                TailSamples::Infinite
            } else {
                TailSamples::Finite(8)
            },
            maximum_state: StatePayloadSizes {
                common_bytes: u32::MAX,
                left_bytes: 16,
                right_bytes: 16,
            },
            scratch_fixed_bytes: u64::MAX,
            scratch_bytes_per_frame: u64::MAX,
        }
    }

    static QUALITIES: [QualityDescriptor; 8] = [
        quality(44_100),
        quality(48_000),
        quality(88_200),
        quality(96_000),
        quality(176_400),
        quality(192_000),
        quality(352_800),
        quality(384_000),
    ];
    static BAD_RATE_QUALITIES: [QualityDescriptor; 8] = [
        quality(12_345),
        QUALITIES[1],
        QUALITIES[2],
        QUALITIES[3],
        QUALITIES[4],
        QUALITIES[5],
        QUALITIES[6],
        QUALITIES[7],
    ];
    static BAD_STATE_QUALITIES: [QualityDescriptor; 8] = [
        QualityDescriptor {
            maximum_state: StatePayloadSizes {
                common_bytes: u32::MAX,
                left_bytes: 16,
                right_bytes: 17,
            },
            ..QUALITIES[0]
        },
        QUALITIES[1],
        QUALITIES[2],
        QUALITIES[3],
        QUALITIES[4],
        QUALITIES[5],
        QUALITIES[6],
        QUALITIES[7],
    ];
    static OUT_OF_ORDER_QUALITIES: [QualityDescriptor; 8] = [
        QUALITIES[1],
        QUALITIES[0],
        QUALITIES[2],
        QUALITIES[3],
        QUALITIES[4],
        QUALITIES[5],
        QUALITIES[6],
        QUALITIES[7],
    ];
    static MISSING_RATE_QUALITIES: [QualityDescriptor; 7] = [
        QUALITIES[0],
        QUALITIES[1],
        QUALITIES[2],
        QUALITIES[4],
        QUALITIES[5],
        QUALITIES[6],
        QUALITIES[7],
    ];
    static DRAFT_ONLY_QUALITIES: [QualityDescriptor; 8] = [
        QualityDescriptor {
            quality: EffectQuality::Draft,
            ..QUALITIES[0]
        },
        QualityDescriptor {
            quality: EffectQuality::Draft,
            ..QUALITIES[1]
        },
        QualityDescriptor {
            quality: EffectQuality::Draft,
            ..QUALITIES[2]
        },
        QualityDescriptor {
            quality: EffectQuality::Draft,
            ..QUALITIES[3]
        },
        QualityDescriptor {
            quality: EffectQuality::Draft,
            ..QUALITIES[4]
        },
        QualityDescriptor {
            quality: EffectQuality::Draft,
            ..QUALITIES[5]
        },
        QualityDescriptor {
            quality: EffectQuality::Draft,
            ..QUALITIES[6]
        },
        QualityDescriptor {
            quality: EffectQuality::Draft,
            ..QUALITIES[7]
        },
    ];

    static DESCRIPTOR: EffectDescriptor = EffectDescriptor {
        id: effect_id("test.effect"),
        display_name: "Test Effect",
        contract_major: 1,
        contract_minor: u16::MAX,
        state_layout_version: 7,
        supported_link_modes: LinkModeSet::ALL,
        parameters: &PARAMETERS,
        ports: &PORTS_UNSORTED,
        qualities: &QUALITIES,
        observations: &[],
    };
    static SORTED_PORT_DESCRIPTOR: EffectDescriptor = EffectDescriptor {
        ports: &PORTS_SORTED,
        ..DESCRIPTOR
    };
    static EMPTY_PARAMETER_DESCRIPTOR: EffectDescriptor = EffectDescriptor {
        parameters: &[],
        ..DESCRIPTOR
    };
    static NO_SIDECHAIN_PORTS: [PortDescriptor; 2] = [PORTS_SORTED[0], PORTS_SORTED[1]];
    static NO_SIDECHAIN_DESCRIPTOR: EffectDescriptor = EffectDescriptor {
        ports: &NO_SIDECHAIN_PORTS,
        ..DESCRIPTOR
    };
    static LAUNCH_QUALITIES: [QualityDescriptor; 4] =
        [QUALITIES[0], QUALITIES[1], QUALITIES[2], QUALITIES[3]];
    static LAUNCH_QUALITY_DESCRIPTOR: EffectDescriptor = EffectDescriptor {
        qualities: &LAUNCH_QUALITIES,
        ..DESCRIPTOR
    };
    static NO_CHOICE_PARAMETERS: [ParameterDescriptor; 3] = [
        PARAMETERS[0],
        PARAMETERS[1],
        ParameterDescriptor {
            domain: ParameterDomain::Boolean,
            default_value: 1.0,
            enum_choices: &[],
            ..PARAMETERS[2]
        },
    ];
    static NO_CHOICE_DESCRIPTOR: EffectDescriptor = EffectDescriptor {
        parameters: &NO_CHOICE_PARAMETERS,
        ..DESCRIPTOR
    };
    static BAD_CONTRACT: EffectDescriptor = EffectDescriptor {
        contract_major: 2,
        ..DESCRIPTOR
    };
    static BAD_STATE_VERSION: EffectDescriptor = EffectDescriptor {
        state_layout_version: 0,
        ..DESCRIPTOR
    };
    static BAD_DISPLAY: EffectDescriptor = EffectDescriptor {
        display_name: "Test\nEffect",
        ..DESCRIPTOR
    };
    static BAD_ZERO_ID: EffectDescriptor = EffectDescriptor {
        parameters: &ZERO_ID_PARAMETERS,
        ..DESCRIPTOR
    };
    static BAD_DEFAULT: EffectDescriptor = EffectDescriptor {
        parameters: &BAD_DEFAULT_PARAMETERS,
        ..DESCRIPTOR
    };
    static BAD_BOOLEAN: EffectDescriptor = EffectDescriptor {
        parameters: &BAD_BOOLEAN_PARAMETERS,
        ..DESCRIPTOR
    };
    static BAD_SMOOTHING: EffectDescriptor = EffectDescriptor {
        parameters: &BAD_SMOOTHING_PARAMETERS,
        ..DESCRIPTOR
    };
    static BAD_LABEL: EffectDescriptor = EffectDescriptor {
        parameters: &BAD_LABEL_PARAMETERS,
        ..DESCRIPTOR
    };
    static BAD_PORT: EffectDescriptor = EffectDescriptor {
        ports: &BAD_PORTS,
        ..DESCRIPTOR
    };
    static BAD_RATE: EffectDescriptor = EffectDescriptor {
        qualities: &BAD_RATE_QUALITIES,
        ..DESCRIPTOR
    };
    static BAD_STATE: EffectDescriptor = EffectDescriptor {
        qualities: &BAD_STATE_QUALITIES,
        ..DESCRIPTOR
    };
    static DUPLICATE_ID: EffectDescriptor = EffectDescriptor {
        parameters: &DUPLICATE_ID_PARAMETERS,
        ..DESCRIPTOR
    };
    static OUT_OF_ORDER_PARAMETER: EffectDescriptor = EffectDescriptor {
        parameters: &OUT_OF_ORDER_PARAMETERS,
        ..DESCRIPTOR
    };
    static MISSING_MINIMUM: EffectDescriptor = EffectDescriptor {
        parameters: &MISSING_MINIMUM_PARAMETERS,
        ..DESCRIPTOR
    };
    static BAD_LOG: EffectDescriptor = EffectDescriptor {
        parameters: &BAD_LOG_PARAMETERS,
        ..DESCRIPTOR
    };
    static BAD_BOOLEAN_DEFAULT: EffectDescriptor = EffectDescriptor {
        parameters: &BAD_BOOLEAN_DEFAULT_PARAMETERS,
        ..DESCRIPTOR
    };
    static SHORT_ENUM: EffectDescriptor = EffectDescriptor {
        parameters: &SHORT_ENUM_PARAMETERS,
        ..DESCRIPTOR
    };
    static BAD_ENUM_DEFAULT: EffectDescriptor = EffectDescriptor {
        parameters: &BAD_ENUM_DEFAULT_PARAMETERS,
        ..DESCRIPTOR
    };
    static BAD_AUTOMATION: EffectDescriptor = EffectDescriptor {
        parameters: &BAD_AUTOMATION_PARAMETERS,
        ..DESCRIPTOR
    };
    static NONFINITE: EffectDescriptor = EffectDescriptor {
        parameters: &NONFINITE_PARAMETERS,
        ..DESCRIPTOR
    };
    static NEGATIVE_ZERO: EffectDescriptor = EffectDescriptor {
        parameters: &NEGATIVE_ZERO_PARAMETERS,
        ..DESCRIPTOR
    };
    static REVERSED_BOUND: EffectDescriptor = EffectDescriptor {
        parameters: &REVERSED_BOUND_PARAMETERS,
        ..DESCRIPTOR
    };
    static CONTINUOUS_CHOICES: EffectDescriptor = EffectDescriptor {
        parameters: &CONTINUOUS_CHOICES_PARAMETERS,
        ..DESCRIPTOR
    };
    static BOOLEAN_MAPPING: EffectDescriptor = EffectDescriptor {
        parameters: &BOOLEAN_MAPPING_PARAMETERS,
        ..DESCRIPTOR
    };
    static OUT_OF_ORDER_ENUM: EffectDescriptor = EffectDescriptor {
        parameters: &OUT_OF_ORDER_ENUM_PARAMETERS,
        ..DESCRIPTOR
    };
    static NONFINITE_ENUM: EffectDescriptor = EffectDescriptor {
        parameters: &NONFINITE_ENUM_PARAMETERS,
        ..DESCRIPTOR
    };
    static CONTROL_LABEL: EffectDescriptor = EffectDescriptor {
        parameters: &CONTROL_LABEL_PARAMETERS,
        ..DESCRIPTOR
    };
    static DUPLICATE_PORT: EffectDescriptor = EffectDescriptor {
        ports: &DUPLICATE_PORTS,
        ..DESCRIPTOR
    };
    static MISSING_OUTPUT: EffectDescriptor = EffectDescriptor {
        ports: &MISSING_OUTPUT_PORTS,
        ..DESCRIPTOR
    };
    static TWO_SIDECHAINS: EffectDescriptor = EffectDescriptor {
        ports: &TWO_SIDECHAIN_PORTS,
        ..DESCRIPTOR
    };
    static OUT_OF_ORDER_QUALITY: EffectDescriptor = EffectDescriptor {
        qualities: &OUT_OF_ORDER_QUALITIES,
        ..DESCRIPTOR
    };
    static MISSING_RATE: EffectDescriptor = EffectDescriptor {
        qualities: &MISSING_RATE_QUALITIES,
        ..DESCRIPTOR
    };
    static MISSING_NORMAL: EffectDescriptor = EffectDescriptor {
        qualities: &DRAFT_ONLY_QUALITIES,
        ..DESCRIPTOR
    };
    static MULTI_ERROR: EffectDescriptor = EffectDescriptor {
        contract_major: 2,
        state_layout_version: 0,
        parameters: &DUPLICATE_ID_PARAMETERS,
        ports: &BAD_PORTS,
        qualities: &BAD_STATE_QUALITIES,
        ..DESCRIPTOR
    };

    fn encode(descriptor: &'static EffectDescriptor) -> Vec<u8> {
        let required = effect_descriptor_wire_required_size(descriptor, 1 << 20).unwrap();
        let mut bytes = vec![0xa5; required as usize];
        assert_eq!(
            encode_effect_descriptor_wire(descriptor, 1 << 20, &mut bytes),
            Ok(required)
        );
        bytes
    }

    fn semantic_test_view(bytes: &[u8]) -> BorrowedEffectDescriptorView<'_> {
        let layout = Layout {
            total: read_u32(bytes, 16),
            parameters: read_u32(bytes, 48),
            ports: read_u32(bytes, 56),
            qualities: read_u32(bytes, 64),
            choices: read_u32(bytes, 72),
            observations: read_u32(bytes, 88),
            parameter_offset: read_u32(bytes, 52),
            port_offset: read_u32(bytes, 60),
            quality_offset: read_u32(bytes, 68),
            choice_offset: read_u32(bytes, 76),
            observation_offset: read_u32(bytes, 76)
                + read_u32(bytes, 72) * ENUM_CHOICE_BYTES as u32,
            string_bytes: read_u32(bytes, 80),
            string_offset: read_u32(bytes, 84),
        };
        let id_offset = read_u32(bytes, 32) as usize;
        let id_length = read_u32(bytes, 36) as usize;
        let display_offset = read_u32(bytes, 40) as usize;
        let display_length = read_u32(bytes, 44) as usize;
        BorrowedEffectDescriptorView {
            bytes,
            layout,
            effect_id: core::str::from_utf8(&bytes[id_offset..id_offset + id_length]).unwrap(),
            display_name: core::str::from_utf8(
                &bytes[display_offset..display_offset + display_length],
            )
            .unwrap(),
            contract_major: read_u16(bytes, 20),
            state_layout_version: read_u32(bytes, 24),
            link_modes: LinkModeSet::new(read_u32(bytes, 28)).unwrap(),
        }
    }

    fn accepted_errors(descriptor: &'static EffectDescriptor) -> Vec<DescriptorError> {
        validate_descriptor(descriptor).unwrap_err().0
    }

    fn borrowed_errors(bytes: &[u8]) -> Vec<DescriptorError> {
        borrowed_semantic_errors(semantic_test_view(bytes))
            .into_iter()
            .map(|error| DescriptorError {
                path: error.path,
                code: error.code,
            })
            .collect()
    }

    fn assert_parity(descriptor: &'static EffectDescriptor, mutate: impl FnOnce(&mut [u8])) {
        let mut bytes = encode(&DESCRIPTOR);
        mutate(&mut bytes);
        assert_eq!(borrowed_errors(&bytes), accepted_errors(descriptor));
    }

    #[test]
    fn representative_layout_roundtrip_and_identity_are_exact() {
        let bytes = encode(&DESCRIPTOR);
        assert_eq!(&bytes[..8], MAGIC);
        assert_eq!(read_u16(&bytes, 10), 96);
        assert_eq!(read_u32(&bytes, 52), 96);
        assert_eq!(read_u32(&bytes, 60), 96 + 3 * 80);
        assert_eq!(read_u32(&bytes, 68), 96 + 3 * 80 + 3 * 24);
        assert_eq!(read_u32(&bytes, 76), 96 + 3 * 80 + 3 * 24 + 8 * 64);
        let verified = verify_effect_descriptor_wire(&bytes, 1 << 20).unwrap();
        assert_eq!(verified.as_bytes(), bytes);
        assert_eq!(verified.parameter_count(), 3);
        assert_eq!(verified.port_count(), 3);
        assert_eq!(verified.quality_count(), 8);
        assert_eq!(verified.enum_choice_count(), 2);
        assert_eq!(verified.state_layout_version(), 7);
        assert_eq!(verified.supported_link_mode_bits(), 7);
        let identity = effect_descriptor_identity(&bytes, 1 << 20).unwrap();
        let mut independent = Sha256::new();
        independent.update(IDENTITY_DOMAIN);
        independent.update((bytes.len() as u64).to_le_bytes());
        independent.update(&bytes);
        assert_eq!(
            identity.as_bytes(),
            &<[u8; 32]>::from(independent.finalize())
        );
    }

    #[test]
    fn exact_size_and_one_short_have_complete_canaries() {
        let required = effect_descriptor_wire_required_size(&DESCRIPTOR, 1 << 20).unwrap();
        let mut exact = vec![0x5a; required as usize + 16];
        assert_eq!(
            encode_effect_descriptor_wire(&DESCRIPTOR, 1 << 20, &mut exact),
            Ok(required)
        );
        assert_eq!(&exact[required as usize..], &[0x5a; 16]);
        let mut short = vec![0x6b; required as usize - 1];
        let before = short.clone();
        assert_eq!(
            encode_effect_descriptor_wire(&DESCRIPTOR, 1 << 20, &mut short),
            Err(Diagnostic::buffer_too_small(required))
        );
        assert_eq!(short, before);
        assert_eq!(
            effect_descriptor_wire_required_size(&DESCRIPTOR, required - 1)
                .unwrap_err()
                .code,
            Code::Limit
        );
    }

    #[test]
    fn port_order_is_canonical_and_legal_fields_change_identity() {
        let unsorted = encode(&DESCRIPTOR);
        let sorted = encode(&SORTED_PORT_DESCRIPTOR);
        assert_eq!(unsorted, sorted);
        assert_eq!(
            effect_descriptor_identity(&unsorted, 1 << 20),
            effect_descriptor_identity(&sorted, 1 << 20)
        );
        let mut changed = unsorted.clone();
        write_u16(&mut changed, 22, 9);
        assert!(verify_effect_descriptor_wire(&changed, 1 << 20).is_ok());
        assert_ne!(
            effect_descriptor_identity(&unsorted, 1 << 20),
            effect_descriptor_identity(&changed, 1 << 20)
        );
    }

    #[test]
    fn diagnostic_phase_order_and_wire_only_rejections_are_frozen() {
        let original = encode(&DESCRIPTOR);
        let mut bytes = original.clone();
        bytes[0] ^= 1;
        bytes[88] = 1;
        assert_eq!(
            verify_effect_descriptor_wire(&bytes, 1 << 20)
                .unwrap_err()
                .code,
            Code::Header
        );
        // Issue #143 moved byte 88 from "reserved zero" to `observation_count`, so the
        // reserved-before-enum ordering is proven on byte 92, which a zero-tap descriptor still
        // requires to be zero. Claiming a tap the wire does not carry is refused by length first,
        // which is the earlier phase and the more specific answer.
        let mut bytes = original.clone();
        bytes[92] = 1;
        write_u32(&mut bytes, 28, 0);
        assert_eq!(
            verify_effect_descriptor_wire(&bytes, 1 << 20)
                .unwrap_err()
                .code,
            Code::Reserved
        );
        let mut bytes = original.clone();
        bytes[88] = 1;
        assert_eq!(
            verify_effect_descriptor_wire(&bytes, 1 << 20)
                .unwrap_err()
                .code,
            Code::Length
        );
        let mut bytes = original.clone();
        write_u32(&mut bytes, 52, 104);
        write_u32(&mut bytes, 28, 0);
        assert_eq!(
            verify_effect_descriptor_wire(&bytes, 1 << 20)
                .unwrap_err()
                .code,
            Code::Offset
        );
        let mut trailing = original.clone();
        trailing.push(0);
        assert_eq!(
            verify_effect_descriptor_wire(&trailing, 1 << 20)
                .unwrap_err()
                .code,
            Code::Length
        );
        let mut negative_zero = original;
        write_u32(&mut negative_zero, HEADER_BYTES + 44, (-0.0f32).to_bits());
        let error = verify_effect_descriptor_wire(&negative_zero, 1 << 20).unwrap_err();
        assert_eq!((error.code, error.byte_offset), (Code::Float, 96 + 44));
    }

    #[test]
    fn field_overflow_offsets_and_within_phase_tie_breaks_are_exact() {
        for (field, expected_offset) in [(48, 48), (56, 56), (64, 64), (72, 72), (80, 80)] {
            let mut bytes = vec![0; HEADER_BYTES];
            bytes[..8].copy_from_slice(MAGIC);
            write_u16(&mut bytes, 8, VERSION);
            write_u16(&mut bytes, 10, HEADER_BYTES as u16);
            write_u32(&mut bytes, 16, HEADER_BYTES as u32);
            write_u32(&mut bytes, 52, HEADER_BYTES as u32);
            write_u32(&mut bytes, 60, HEADER_BYTES as u32);
            write_u32(&mut bytes, 68, HEADER_BYTES as u32);
            write_u32(&mut bytes, 76, HEADER_BYTES as u32);
            write_u32(&mut bytes, 84, HEADER_BYTES as u32);
            write_u32(&mut bytes, field, u32::MAX);
            let error = verify_effect_descriptor_wire(&bytes, u32::MAX).unwrap_err();
            assert_eq!(
                (error.code, error.byte_offset, error.record_index),
                (
                    Code::Overflow,
                    expected_offset,
                    EFFECT_DESCRIPTOR_WIRE_UNAVAILABLE
                )
            );
        }

        let original = encode(&DESCRIPTOR);
        let mut flags_before_reserved = original.clone();
        write_u32(&mut flags_before_reserved, HEADER_BYTES + 32, 16);
        write_u32(&mut flags_before_reserved, HEADER_BYTES + 72, 1);
        let error = verify_effect_descriptor_wire(&flags_before_reserved, 1 << 20).unwrap_err();
        assert_eq!(
            (error.code, error.byte_offset),
            (Code::Flags, (HEADER_BYTES + 32) as u32)
        );

        let mut header_text_before_table_offset = original.clone();
        write_u32(
            &mut header_text_before_table_offset,
            32,
            read_u32(&original, 32) + 1,
        );
        write_u32(
            &mut header_text_before_table_offset,
            52,
            HEADER_BYTES as u32 + 4,
        );
        let error = verify_effect_descriptor_wire(&header_text_before_table_offset, 1 << 20)
            .unwrap_err();
        assert_eq!((error.code, error.byte_offset), (Code::Offset, 32));

        let mut port_text_before_choice_text = original;
        let first_port = read_u32(&port_text_before_choice_text, 60) as usize;
        let port_text = read_u32(&port_text_before_choice_text, first_port) as usize;
        port_text_before_choice_text[port_text] = b'A';
        let first_choice = read_u32(&port_text_before_choice_text, 76) as usize;
        let choice_text = read_u32(&port_text_before_choice_text, first_choice + 4) as usize;
        port_text_before_choice_text[choice_text] = b'\n';
        let error =
            verify_effect_descriptor_wire(&port_text_before_choice_text, 1 << 20).unwrap_err();
        assert_eq!(
            (error.code, error.byte_offset, error.record_index),
            (Code::Text, first_port as u32, 0)
        );
    }

    #[test]
    fn safe_constructor_semantic_differential_parity_is_exact() {
        assert_parity(&BAD_CONTRACT, |bytes| write_u16(bytes, 20, 2));
        assert_parity(&BAD_STATE_VERSION, |bytes| write_u32(bytes, 24, 0));
        assert_parity(&BAD_DISPLAY, |bytes| {
            let offset = read_u32(bytes, 40) as usize;
            bytes[offset + 4] = b'\n';
        });
        assert_parity(&BAD_ZERO_ID, |bytes| write_u32(bytes, 96, 0));
        assert_parity(&BAD_DEFAULT, |bytes| {
            write_u32(bytes, 96 + 44, 25.0f32.to_bits());
        });
        assert_parity(&BAD_BOOLEAN, |bytes| {
            let record = 96 + PARAMETER_BYTES;
            write_u32(bytes, record + 32, read_u32(bytes, record + 32) | 4);
        });
        assert_parity(&BAD_SMOOTHING, |bytes| write_u32(bytes, 96 + 28, 0));
        assert_parity(&BAD_LABEL, |bytes| {
            let choice = read_u32(bytes, 76) as usize + ENUM_CHOICE_BYTES;
            let label = read_u32(bytes, choice + 4) as usize;
            bytes[label..label + 2].copy_from_slice(b"No");
        });
        assert_parity(&BAD_PORT, |bytes| {
            let port = read_u32(bytes, 60) as usize;
            write_u32(bytes, port + 12, 0);
        });
        assert_parity(&BAD_RATE, |bytes| {
            let quality = read_u32(bytes, 68) as usize;
            write_u32(bytes, quality + 4, 12_345);
        });
        assert_parity(&BAD_STATE, |bytes| {
            let quality = read_u32(bytes, 68) as usize;
            write_u32(bytes, quality + 40, 17);
        });
        assert_parity(&DUPLICATE_ID, |bytes| {
            write_u32(bytes, HEADER_BYTES + PARAMETER_BYTES, 1);
        });
        assert_parity(&OUT_OF_ORDER_PARAMETER, |bytes| {
            write_u32(bytes, HEADER_BYTES, 2);
            write_u32(bytes, HEADER_BYTES + PARAMETER_BYTES, 1);
        });
        assert_parity(&MISSING_MINIMUM, |bytes| {
            let flags = read_u32(bytes, HEADER_BYTES + 32) & !4;
            write_u32(bytes, HEADER_BYTES + 32, flags);
            write_u32(bytes, HEADER_BYTES + 36, 0);
        });
        assert_parity(&BAD_LOG, |bytes| {
            write_u32(
                bytes,
                HEADER_BYTES + 12,
                ParameterMapping::Logarithmic as u32,
            );
        });
        assert_parity(&BAD_BOOLEAN_DEFAULT, |bytes| {
            write_u32(bytes, HEADER_BYTES + PARAMETER_BYTES + 44, 0.5f32.to_bits());
        });
        assert_parity(&SHORT_ENUM, |bytes| {
            write_u32(bytes, HEADER_BYTES + 2 * PARAMETER_BYTES + 52, 1);
        });
        assert_parity(&BAD_ENUM_DEFAULT, |bytes| {
            write_u32(
                bytes,
                HEADER_BYTES + 2 * PARAMETER_BYTES + 44,
                2.0f32.to_bits(),
            );
        });
        assert_parity(&BAD_AUTOMATION, |bytes| {
            let first_flags = read_u32(bytes, HEADER_BYTES + 32) & !2;
            write_u32(bytes, HEADER_BYTES + 32, first_flags);
            let third = HEADER_BYTES + 2 * PARAMETER_BYTES;
            let third_flags = read_u32(bytes, third + 32) | 2;
            write_u32(bytes, third + 32, third_flags);
        });
        assert_parity(&NONFINITE, |bytes| {
            write_u32(bytes, HEADER_BYTES + 36, f32::NAN.to_bits());
        });
        assert_parity(&NEGATIVE_ZERO, |bytes| {
            write_u32(bytes, HEADER_BYTES + 44, (-0.0f32).to_bits());
        });
        assert_parity(&REVERSED_BOUND, |bytes| {
            write_u32(bytes, HEADER_BYTES + 36, 24.0f32.to_bits());
            write_u32(bytes, HEADER_BYTES + 40, (-24.0f32).to_bits());
        });
        assert_parity(&CONTINUOUS_CHOICES, |bytes| {
            write_u32(bytes, HEADER_BYTES + 52, 2);
        });
        assert_parity(&BOOLEAN_MAPPING, |bytes| {
            write_u32(
                bytes,
                HEADER_BYTES + PARAMETER_BYTES + 12,
                ParameterMapping::Linear as u32,
            );
        });
        assert_parity(&OUT_OF_ORDER_ENUM, |bytes| {
            let choice = read_u32(bytes, 76) as usize;
            write_u32(bytes, choice, 1.0f32.to_bits());
            write_u32(bytes, choice + ENUM_CHOICE_BYTES, 0.0f32.to_bits());
        });
        assert_parity(&NONFINITE_ENUM, |bytes| {
            let choice = read_u32(bytes, 76) as usize;
            write_u32(bytes, choice, f32::NAN.to_bits());
        });
        assert_parity(&CONTROL_LABEL, |bytes| {
            let choice = read_u32(bytes, 76) as usize;
            let label = read_u32(bytes, choice + 4) as usize;
            bytes[label] = b'\n';
        });
        assert_parity(&DUPLICATE_PORT, |bytes| {
            let port = read_u32(bytes, 60) as usize;
            write_u32(bytes, port + PORT_BYTES, read_u32(bytes, port));
            write_u32(bytes, port + PORT_BYTES + 4, read_u32(bytes, port + 4));
        });
        assert_parity(&MISSING_OUTPUT, |bytes| write_u32(bytes, 56, 1));
        let mut two_sidechains = encode(&DESCRIPTOR);
        let quality = read_u32(&two_sidechains, 68) as usize;
        let old_choice = read_u32(&two_sidechains, 76) as usize;
        let old_string = read_u32(&two_sidechains, 84) as usize;
        two_sidechains.splice(quality..quality, [0; PORT_BYTES]);
        let new_length = two_sidechains.len() as u32;
        write_u32(&mut two_sidechains, 16, new_length);
        write_u32(&mut two_sidechains, 56, 4);
        write_u32(&mut two_sidechains, 68, (quality + PORT_BYTES) as u32);
        write_u32(&mut two_sidechains, 76, (old_choice + PORT_BYTES) as u32);
        write_u32(&mut two_sidechains, 84, (old_string + PORT_BYTES) as u32);
        for field in [32, 40] {
            let offset = read_u32(&two_sidechains, field);
            write_u32(&mut two_sidechains, field, offset + PORT_BYTES as u32);
        }
        for index in 0..3 {
            let record = HEADER_BYTES + index * PARAMETER_BYTES;
            for field in [56, 64] {
                let offset = read_u32(&two_sidechains, record + field);
                write_u32(
                    &mut two_sidechains,
                    record + field,
                    offset + PORT_BYTES as u32,
                );
            }
        }
        let port = read_u32(&two_sidechains, 60) as usize;
        for index in 0..3 {
            let record = port + index * PORT_BYTES;
            let offset = read_u32(&two_sidechains, record);
            write_u32(&mut two_sidechains, record, offset + PORT_BYTES as u32);
        }
        let extra_port = quality;
        let effect_id_offset = read_u32(&two_sidechains, 32);
        write_u32(&mut two_sidechains, extra_port, effect_id_offset);
        write_u32(&mut two_sidechains, extra_port + 4, 11);
        write_u32(
            &mut two_sidechains,
            extra_port + 8,
            PortRole::SidechainInput as u32,
        );
        write_u32(
            &mut two_sidechains,
            extra_port + 16,
            PortLayout::DualMonoPlanar as u32,
        );
        let choice = old_choice + PORT_BYTES;
        for index in 0..2 {
            let record = choice + index * ENUM_CHOICE_BYTES;
            let offset = read_u32(&two_sidechains, record + 4);
            write_u32(&mut two_sidechains, record + 4, offset + PORT_BYTES as u32);
        }
        assert_eq!(
            borrowed_errors(&two_sidechains),
            accepted_errors(&TWO_SIDECHAINS)
        );
        assert_parity(&OUT_OF_ORDER_QUALITY, |bytes| {
            let quality = read_u32(bytes, 68) as usize;
            write_u32(bytes, quality + 4, 48_000);
            write_u32(bytes, quality + QUALITY_BYTES + 4, 44_100);
        });
        assert_parity(&MISSING_RATE, |bytes| {
            let quality = read_u32(bytes, 68) as usize;
            write_u32(bytes, quality + 3 * QUALITY_BYTES + 4, 100_000);
        });
        assert_parity(&MISSING_NORMAL, |bytes| {
            let quality = read_u32(bytes, 68) as usize;
            for index in 0..8 {
                write_u32(
                    bytes,
                    quality + index * QUALITY_BYTES,
                    EffectQuality::Draft as u32,
                );
            }
        });
        assert_parity(&MULTI_ERROR, |bytes| {
            write_u16(bytes, 20, 2);
            write_u32(bytes, 24, 0);
            write_u32(bytes, HEADER_BYTES + PARAMETER_BYTES, 1);
            let port = read_u32(bytes, 60) as usize;
            write_u32(bytes, port + 12, 0);
            let quality = read_u32(bytes, 68) as usize;
            write_u32(bytes, quality + 40, 17);
        });
    }

    #[test]
    fn constructor_sealed_id_and_link_mutations_have_raw_wire_diagnostics() {
        const VALID_127: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        const INVALID_128: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        assert_eq!(VALID_127.len(), 127);
        assert_eq!(INVALID_128.len(), 128);
        for valid in ["a", "z0._-a", VALID_127] {
            assert!(EffectId::new(valid).is_ok());
            assert!(PortId::new(valid).is_ok());
        }
        for invalid in [
            "",
            "0a",
            ".a",
            "_a",
            "-a",
            "Aa",
            "éa",
            "aA",
            "a/b",
            "a b",
            "a\n",
            "aé",
            INVALID_128,
        ] {
            assert!(EffectId::new(invalid).is_err());
            assert!(PortId::new(invalid).is_err());
        }
        let link_values = (0u32..256)
            .chain((8..32).map(|bit| 1u32 << bit))
            .chain((8..32).map(|bit| (1u32 << bit) | 1));
        for value in link_values {
            let valid = value & !7 == 0 && value & 1 != 0;
            assert_eq!(LinkModeSet::new(value).is_some(), valid);
            if valid {
                continue;
            }
            let mut bytes = encode(&DESCRIPTOR);
            write_u32(&mut bytes, 28, value);
            let error = verify_effect_descriptor_wire(&bytes, 1 << 20).unwrap_err();
            assert_eq!(
                (error.code, error.byte_offset, error.record_index),
                (Code::Enum, 28, EFFECT_DESCRIPTOR_WIRE_UNAVAILABLE)
            );
        }
        let mut effect_id = encode(&DESCRIPTOR);
        let offset = read_u32(&effect_id, 32) as usize;
        effect_id[offset] = b'A';
        assert_eq!(
            verify_effect_descriptor_wire(&effect_id, 1 << 20)
                .unwrap_err()
                .code,
            Code::Text
        );
        let mut effect_id_rest = encode(&DESCRIPTOR);
        let offset = read_u32(&effect_id_rest, 32) as usize;
        effect_id_rest[offset + 4] = b'/';
        let error = verify_effect_descriptor_wire(&effect_id_rest, 1 << 20).unwrap_err();
        assert_eq!(
            (error.code, error.byte_offset, error.record_index),
            (Code::Text, 32, EFFECT_DESCRIPTOR_WIRE_UNAVAILABLE)
        );
        let mut port_id = encode(&DESCRIPTOR);
        let port = read_u32(&port_id, 60) as usize;
        let offset = read_u32(&port_id, port) as usize;
        port_id[offset] = b'A';
        assert_eq!(
            verify_effect_descriptor_wire(&port_id, 1 << 20)
                .unwrap_err()
                .code,
            Code::Text
        );
        let mut port_id_rest = encode(&DESCRIPTOR);
        let port = read_u32(&port_id_rest, 60) as usize;
        let offset = read_u32(&port_id_rest, port) as usize;
        port_id_rest[offset + 4] = b'/';
        let error = verify_effect_descriptor_wire(&port_id_rest, 1 << 20).unwrap_err();
        assert_eq!(
            (error.code, error.byte_offset, error.record_index),
            (Code::Text, port as u32, 0)
        );
    }

    #[test]
    fn issue011_absent_rules_rates_and_control_scalars_are_not_strengthened() {
        assert!(valid_text("line\u{2028}separator"));
        assert!(!valid_text("line\nfeed"));
        assert!(!valid_text("c1\u{85}"));
        assert_eq!(DESCRIPTOR.contract_minor, u16::MAX);
        assert!(!PARAMETERS[0].readable);
        assert_eq!(QUALITIES[0].maximum_state.common_bytes, u32::MAX);
        assert_eq!(QUALITIES[0].scratch_fixed_bytes, u64::MAX);
        assert!(validate_descriptor(&EMPTY_PARAMETER_DESCRIPTOR).is_ok());
        let bytes = encode(&EMPTY_PARAMETER_DESCRIPTOR);
        assert!(verify_effect_descriptor_wire(&bytes, 1 << 20).is_ok());
        assert_eq!(
            QUALITIES.map(|quality| quality.sample_rate),
            [
                44_100, 48_000, 88_200, 96_000, 176_400, 192_000, 352_800, 384_000
            ]
        );
    }

    #[test]
    fn bound_descriptor_comparison_reports_earliest_semantic_wire_field() {
        fn assert_mismatch(bytes: &[u8], offset: u32, record_index: u32) {
            let error = bind_effect_descriptor_wire(&DESCRIPTOR, bytes, 1 << 20).unwrap_err();
            assert_eq!(
                error.kind(),
                EffectDescriptorBindingErrorKind::StaticDescriptorMismatch,
                "expected static mismatch at wire offset {offset}; nested={:?}",
                error.diagnostic()
            );
            assert_eq!(
                (
                    error.diagnostic().code,
                    error.diagnostic().byte_offset,
                    error.diagnostic().record_index,
                ),
                (Code::Semantic, offset, record_index)
            );
        }

        for (offset, value) in [(22, 1), (24, 8), (28, 1)] {
            let mut bytes = encode(&DESCRIPTOR);
            if offset == 22 {
                write_u16(&mut bytes, offset, value as u16);
            } else {
                write_u32(&mut bytes, offset, value);
            }
            assert_mismatch(&bytes, offset as u32, EFFECT_DESCRIPTOR_WIRE_UNAVAILABLE);
        }

        for field in [32, 40] {
            let mut bytes = encode(&DESCRIPTOR);
            let text = read_u32(&bytes, field) as usize;
            bytes[text] = if field == 32 { b'u' } else { b'R' };
            assert_mismatch(&bytes, field as u32, EFFECT_DESCRIPTOR_WIRE_UNAVAILABLE);
        }

        assert_mismatch(
            &encode(&EMPTY_PARAMETER_DESCRIPTOR),
            48,
            EFFECT_DESCRIPTOR_WIRE_UNAVAILABLE,
        );
        assert_mismatch(
            &encode(&NO_SIDECHAIN_DESCRIPTOR),
            56,
            EFFECT_DESCRIPTOR_WIRE_UNAVAILABLE,
        );
        assert_mismatch(
            &encode(&LAUNCH_QUALITY_DESCRIPTOR),
            64,
            EFFECT_DESCRIPTOR_WIRE_UNAVAILABLE,
        );
        assert_mismatch(
            &encode(&NO_CHOICE_DESCRIPTOR),
            72,
            EFFECT_DESCRIPTOR_WIRE_UNAVAILABLE,
        );

        let parameter = HEADER_BYTES;
        for (field, value) in [
            (4, ParameterUnit::Hz as u32),
            (28, 65),
            (32, 15),
            (36, (-23.0f32).to_bits()),
            (40, 23.0f32.to_bits()),
            (44, 1.0f32.to_bits()),
        ] {
            let mut bytes = encode(&DESCRIPTOR);
            write_u32(&mut bytes, parameter + field, value);
            assert_mismatch(&bytes, (parameter + field) as u32, 0);
        }
        for field in [56, 64] {
            let mut bytes = encode(&DESCRIPTOR);
            let text = read_u32(&bytes, parameter + field) as usize;
            bytes[text] = b'R';
            assert_mismatch(&bytes, (parameter + field) as u32, 0);
        }

        let port = read_u32(&encode(&DESCRIPTOR), 60) as usize;
        let mut bytes = encode(&DESCRIPTOR);
        let sidechain = port + 2 * PORT_BYTES;
        let port_text = read_u32(&bytes, sidechain) as usize;
        bytes[port_text] = b't';
        assert_mismatch(&bytes, sidechain as u32, 2);
        let mut bytes = encode(&DESCRIPTOR);
        write_u32(&mut bytes, sidechain + 12, 1);
        assert_mismatch(&bytes, (sidechain + 12) as u32, 2);

        let quality = read_u32(&encode(&DESCRIPTOR), 68) as usize;
        for (field, value) in [(8, 5_u64), (24, 9), (48, u64::MAX - 1), (56, u64::MAX - 1)] {
            let mut bytes = encode(&DESCRIPTOR);
            write_u64(&mut bytes, quality + field, value);
            assert_mismatch(&bytes, (quality + field) as u32, 0);
        }
        let mut bytes = encode(&DESCRIPTOR);
        write_u32(&mut bytes, quality + 32, u32::MAX - 1);
        assert_mismatch(&bytes, (quality + 32) as u32, 0);

        let choice = read_u32(&encode(&DESCRIPTOR), 76) as usize;
        let mut bytes = encode(&DESCRIPTOR);
        write_u32(&mut bytes, choice, (-1.0f32).to_bits());
        assert_mismatch(&bytes, choice as u32, 0);
        let mut bytes = encode(&DESCRIPTOR);
        let choice_text = read_u32(&bytes, choice + 4) as usize;
        bytes[choice_text] = b'G';
        assert_mismatch(&bytes, (choice + 4) as u32, 0);
    }

    #[test]
    fn accepted_enum_numbers_are_the_wire_numbers() {
        assert_eq!(
            [
                ParameterUnit::Db as u32,
                ParameterUnit::Hz as u32,
                ParameterUnit::Milliseconds as u32,
                ParameterUnit::Samples as u32,
                ParameterUnit::Linear as u32,
                ParameterUnit::Ratio as u32,
            ],
            [1, 2, 3, 4, 5, 6]
        );
        assert_eq!(
            [
                EffectQuality::Draft as u32,
                EffectQuality::Normal as u32,
                EffectQuality::High as u32
            ],
            [1, 2, 3]
        );
        assert_eq!(
            [
                PortRole::MainInput as u32,
                PortRole::MainOutput as u32,
                PortRole::SidechainInput as u32
            ],
            [1, 2, 3]
        );
    }
}
