use crate::{
    EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE, EffectDescriptorWireDiagnosticCodeV1 as Code,
    EffectDescriptorWireDiagnosticV1 as Diagnostic,
};
use miso_engine_core::{
    LAUNCH_SAMPLE_RATES, SampleRateHz, is_extended_compatibility_sample_rate, is_launch_sample_rate,
};
use miso_engine_effect_contract::{
    AutomationRate, DescriptorDiagnosticCode, EffectDescriptorV1, EffectQuality, LinkMode,
    LinkModeSet, ParameterChannelPolicy, ParameterDomain, ParameterMapping, ParameterUnit,
    PortDescriptorV1, PortLayout, PortRole, SmoothingRule, TailSamples, validate_descriptor_v1,
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
const IDENTITY_DOMAIN: &[u8] = b"miso.engine.effect-descriptor.identity.v1\0";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectDescriptorIdentityV1([u8; 32]);

impl EffectDescriptorIdentityV1 {
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub(crate) const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum EffectDescriptorBindingErrorKindV1 {
    ExternalWire = 1,
    StaticDescriptorMismatch = 2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectDescriptorBindingErrorV1 {
    kind: EffectDescriptorBindingErrorKindV1,
    diagnostic: Diagnostic,
}

impl EffectDescriptorBindingErrorV1 {
    pub const fn kind(self) -> EffectDescriptorBindingErrorKindV1 {
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
pub struct BoundEffectDescriptorWireV1<'a> {
    descriptor: &'static EffectDescriptorV1,
    wire: &'a [u8],
    identity: EffectDescriptorIdentityV1,
}

impl<'a> BoundEffectDescriptorWireV1<'a> {
    pub const fn wire(&self) -> &'a [u8] {
        self.wire
    }

    pub const fn identity(&self) -> EffectDescriptorIdentityV1 {
        self.identity
    }

    pub(crate) const fn descriptor(&self) -> &'static EffectDescriptorV1 {
        self.descriptor
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VerifiedEffectDescriptorWireV1<'a> {
    bytes: &'a [u8],
    parameter_count: u32,
    port_count: u32,
    quality_count: u32,
    enum_choice_count: u32,
    state_layout_version: u32,
    supported_link_mode_bits: u32,
}

impl<'a> VerifiedEffectDescriptorWireV1<'a> {
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

    pub const fn state_layout_version(self) -> u32 {
        self.state_layout_version
    }

    pub const fn supported_link_mode_bits(self) -> u32 {
        self.supported_link_mode_bits
    }

    /// Identity of bytes this value already proved canonical; performs no second validation pass.
    pub fn identity(self) -> EffectDescriptorIdentityV1 {
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
    parameter_offset: u32,
    port_offset: u32,
    quality_offset: u32,
    choice_offset: u32,
    string_offset: u32,
    string_bytes: u32,
}

fn diagnostic(code: Code, byte_offset: usize, record_index: Option<usize>) -> Diagnostic {
    Diagnostic::new(
        code,
        u32::try_from(byte_offset).unwrap_or(EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE),
        record_index
            .and_then(|index| u32::try_from(index).ok())
            .unwrap_or(EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE),
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

fn port_key(port: &PortDescriptorV1) -> (u32, &[u8]) {
    (port.role as u32, port.id.as_str().as_bytes())
}

fn canonical_port_at(
    ports: &'static [PortDescriptorV1],
    index: usize,
) -> &'static PortDescriptorV1 {
    ports
        .iter()
        .find(|candidate| {
            let key = port_key(candidate);
            ports.iter().filter(|port| port_key(port) < key).count() == index
        })
        .expect("validated descriptor has a unique canonical port at every index")
}

fn descriptor_layout(
    descriptor: &'static EffectDescriptorV1,
    maximum_descriptor_bytes: u32,
) -> Result<Layout, Diagnostic> {
    validate_descriptor_v1(descriptor).map_err(|_| diagnostic(Code::Semantic, 0, None))?;
    if maximum_descriptor_bytes == 0 {
        return Err(diagnostic(Code::Limit, 16, None));
    }
    let parameters = u32_len(descriptor.parameters.len())?;
    let ports = u32_len(descriptor.ports.len())?;
    let qualities = u32_len(descriptor.qualities.len())?;
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
    let string_offset = checked_add(
        choice_offset,
        checked_mul(choices, ENUM_CHOICE_BYTES as u32)?,
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
        parameter_offset,
        port_offset,
        quality_offset,
        choice_offset,
        string_offset,
        string_bytes,
    })
}

pub fn effect_descriptor_wire_v1_required_size(
    descriptor: &'static EffectDescriptorV1,
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

pub fn encode_effect_descriptor_wire_v1(
    descriptor: &'static EffectDescriptorV1,
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
struct BorrowedEffectDescriptorViewV1<'a> {
    bytes: &'a [u8],
    layout: Layout,
    effect_id: &'a str,
    display_name: &'a str,
    contract_major: u16,
    state_layout_version: u32,
    link_modes: LinkModeSet,
}

#[derive(Clone, Copy)]
struct BorrowedParameterV1<'a> {
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
struct BorrowedPortV1<'a> {
    id: &'a str,
    role: PortRole,
    required: bool,
    layout: PortLayout,
}

#[derive(Clone, Copy)]
struct BorrowedQualityV1 {
    quality: EffectQuality,
    sample_rate: u32,
    left_bytes: u32,
    right_bytes: u32,
}

impl<'a> BorrowedEffectDescriptorViewV1<'a> {
    fn text(self, offset: usize, length: usize) -> &'a str {
        core::str::from_utf8(&self.bytes[offset..offset + length]).expect("text phase checked")
    }

    fn parameter(self, index: usize) -> BorrowedParameterV1<'a> {
        let record = self.layout.parameter_offset as usize + index * PARAMETER_BYTES;
        let flags = read_u32(self.bytes, record + 32);
        let name_offset = read_u32(self.bytes, record + 56) as usize;
        let name_length = read_u32(self.bytes, record + 60) as usize;
        let unit_offset = read_u32(self.bytes, record + 64) as usize;
        let unit_length = read_u32(self.bytes, record + 68) as usize;
        BorrowedParameterV1 {
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

    fn port(self, index: usize) -> BorrowedPortV1<'a> {
        let record = self.layout.port_offset as usize + index * PORT_BYTES;
        let offset = read_u32(self.bytes, record) as usize;
        let length = read_u32(self.bytes, record + 4) as usize;
        BorrowedPortV1 {
            id: self.text(offset, length),
            role: PortRole::from_raw(read_u32(self.bytes, record + 8)).unwrap(),
            required: read_u32(self.bytes, record + 12) == 1,
            layout: PortLayout::from_raw(read_u32(self.bytes, record + 16)).unwrap(),
        }
    }

    fn quality(self, index: usize) -> BorrowedQualityV1 {
        let record = self.layout.quality_offset as usize + index * QUALITY_BYTES;
        BorrowedQualityV1 {
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
) -> Result<BorrowedEffectDescriptorViewV1<'_>, Diagnostic> {
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
    let string_offset = checked_add_at(
        choice_offset,
        checked_mul_at(choices, ENUM_CHOICE_BYTES as u32, 72)?,
        72,
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
        parameter_offset,
        port_offset,
        quality_offset,
        choice_offset,
        string_offset,
        string_bytes,
    };

    // Phase 5: reserved words and flags, in table traversal order.
    if read_u32(bytes, 12) != 0 {
        return Err(diagnostic(Code::Reserved, 12, None));
    }
    if let Some(index) = bytes[88..96].iter().position(|byte| *byte != 0) {
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

    Ok(BorrowedEffectDescriptorViewV1 {
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
    view: BorrowedEffectDescriptorViewV1<'_>,
    parameter: BorrowedParameterV1<'_>,
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
    view: BorrowedEffectDescriptorViewV1<'_>,
    parameter: BorrowedParameterV1<'_>,
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
    view: BorrowedEffectDescriptorViewV1<'_>,
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
    errors.sort();
    errors.dedup_by(|left, right| left.path == right.path && left.code == right.code);
    errors
}

fn semantic_mismatch(byte_offset: usize, record_index: Option<usize>) -> Diagnostic {
    diagnostic(Code::Semantic, byte_offset, record_index)
}

fn compare_static_descriptor(
    view: BorrowedEffectDescriptorViewV1<'_>,
    descriptor: &'static EffectDescriptorV1,
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

fn descriptor_identity(bytes: &[u8]) -> EffectDescriptorIdentityV1 {
    let mut hasher = Sha256::new();
    hasher.update(IDENTITY_DOMAIN);
    hasher.update((bytes.len() as u64).to_le_bytes());
    hasher.update(bytes);
    EffectDescriptorIdentityV1(hasher.finalize().into())
}

pub fn bind_effect_descriptor_wire_v1<'a>(
    descriptor: &'static EffectDescriptorV1,
    wire: &'a [u8],
    maximum_descriptor_bytes: u32,
) -> Result<BoundEffectDescriptorWireV1<'a>, EffectDescriptorBindingErrorV1> {
    validate_descriptor_v1(descriptor).map_err(|_| EffectDescriptorBindingErrorV1 {
        kind: EffectDescriptorBindingErrorKindV1::StaticDescriptorMismatch,
        diagnostic: semantic_mismatch(0, None),
    })?;
    let view = parse_borrowed_wire(wire, maximum_descriptor_bytes).map_err(|diagnostic| {
        EffectDescriptorBindingErrorV1 {
            kind: EffectDescriptorBindingErrorKindV1::ExternalWire,
            diagnostic,
        }
    })?;
    if let Some(error) = borrowed_semantic_errors(view)
        .into_iter()
        .min_by_key(|error| (error.byte_offset, error.record_index))
    {
        return Err(EffectDescriptorBindingErrorV1 {
            kind: EffectDescriptorBindingErrorKindV1::ExternalWire,
            diagnostic: diagnostic(Code::Semantic, error.byte_offset, error.record_index),
        });
    }
    compare_static_descriptor(view, descriptor).map_err(|diagnostic| {
        EffectDescriptorBindingErrorV1 {
            kind: EffectDescriptorBindingErrorKindV1::StaticDescriptorMismatch,
            diagnostic,
        }
    })?;
    Ok(BoundEffectDescriptorWireV1 {
        descriptor,
        wire,
        identity: descriptor_identity(wire),
    })
}

pub fn verify_effect_descriptor_wire_v1(
    bytes: &[u8],
    maximum_descriptor_bytes: u32,
) -> Result<VerifiedEffectDescriptorWireV1<'_>, Diagnostic> {
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
    Ok(VerifiedEffectDescriptorWireV1 {
        bytes,
        parameter_count: view.layout.parameters,
        port_count: view.layout.ports,
        quality_count: view.layout.qualities,
        enum_choice_count: view.layout.choices,
        state_layout_version: view.state_layout_version,
        supported_link_mode_bits: view.link_modes.bits(),
    })
}

pub fn effect_descriptor_identity_v1(
    bytes: &[u8],
    maximum_descriptor_bytes: u32,
) -> Result<EffectDescriptorIdentityV1, Diagnostic> {
    Ok(verify_effect_descriptor_wire_v1(bytes, maximum_descriptor_bytes)?.identity())
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_effect_contract::{
        DescriptorError, EffectId, EnumChoiceV1, LatencySamples, ParameterDescriptorV1,
        ParameterId, PortDescriptorV1, PortId, QualityDescriptorV1, StatePayloadSizes,
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

    static CHOICES: [EnumChoiceV1; 2] = [
        EnumChoiceV1 {
            value: 0.0,
            label: "No",
        },
        EnumChoiceV1 {
            value: 1.0,
            label: "Up",
        },
    ];
    static DUPLICATE_LABEL_CHOICES: [EnumChoiceV1; 2] = [
        EnumChoiceV1 {
            value: 0.0,
            label: "No",
        },
        EnumChoiceV1 {
            value: 1.0,
            label: "No",
        },
    ];
    static OUT_OF_ORDER_CHOICES: [EnumChoiceV1; 2] = [CHOICES[1], CHOICES[0]];
    static NONFINITE_CHOICES: [EnumChoiceV1; 2] = [
        EnumChoiceV1 {
            value: f32::NAN,
            label: "No",
        },
        CHOICES[1],
    ];
    static CONTROL_LABEL_CHOICES: [EnumChoiceV1; 2] = [
        EnumChoiceV1 {
            label: "\n",
            ..CHOICES[0]
        },
        CHOICES[1],
    ];
    static PARAMETERS: [ParameterDescriptorV1; 3] = [
        ParameterDescriptorV1 {
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
            nudge_ladder: None,
            enum_choices: &[],
        },
        ParameterDescriptorV1 {
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
            nudge_ladder: None,
            enum_choices: &[],
        },
        ParameterDescriptorV1 {
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
            nudge_ladder: None,
            enum_choices: &CHOICES,
        },
    ];
    static ZERO_ID_PARAMETERS: [ParameterDescriptorV1; 3] = [
        ParameterDescriptorV1 {
            id: ParameterId(0),
            ..PARAMETERS[0]
        },
        PARAMETERS[1],
        PARAMETERS[2],
    ];
    static BAD_DEFAULT_PARAMETERS: [ParameterDescriptorV1; 3] = [
        ParameterDescriptorV1 {
            default_value: 25.0,
            ..PARAMETERS[0]
        },
        PARAMETERS[1],
        PARAMETERS[2],
    ];
    static BAD_BOOLEAN_PARAMETERS: [ParameterDescriptorV1; 3] = [
        PARAMETERS[0],
        ParameterDescriptorV1 {
            minimum: Some(0.0),
            ..PARAMETERS[1]
        },
        PARAMETERS[2],
    ];
    static BAD_SMOOTHING_PARAMETERS: [ParameterDescriptorV1; 3] = [
        ParameterDescriptorV1 {
            smoothing_samples: 0,
            ..PARAMETERS[0]
        },
        PARAMETERS[1],
        PARAMETERS[2],
    ];
    static BAD_LABEL_PARAMETERS: [ParameterDescriptorV1; 3] = [
        PARAMETERS[0],
        PARAMETERS[1],
        ParameterDescriptorV1 {
            enum_choices: &DUPLICATE_LABEL_CHOICES,
            ..PARAMETERS[2]
        },
    ];
    static DUPLICATE_ID_PARAMETERS: [ParameterDescriptorV1; 3] = [
        PARAMETERS[0],
        ParameterDescriptorV1 {
            id: ParameterId(1),
            ..PARAMETERS[1]
        },
        PARAMETERS[2],
    ];
    static OUT_OF_ORDER_PARAMETERS: [ParameterDescriptorV1; 3] =
        [PARAMETERS[1], PARAMETERS[0], PARAMETERS[2]];
    static MISSING_MINIMUM_PARAMETERS: [ParameterDescriptorV1; 3] = [
        ParameterDescriptorV1 {
            minimum: None,
            ..PARAMETERS[0]
        },
        PARAMETERS[1],
        PARAMETERS[2],
    ];
    static BAD_LOG_PARAMETERS: [ParameterDescriptorV1; 3] = [
        ParameterDescriptorV1 {
            mapping: ParameterMapping::Logarithmic,
            ..PARAMETERS[0]
        },
        PARAMETERS[1],
        PARAMETERS[2],
    ];
    static BAD_BOOLEAN_DEFAULT_PARAMETERS: [ParameterDescriptorV1; 3] = [
        PARAMETERS[0],
        ParameterDescriptorV1 {
            default_value: 0.5,
            ..PARAMETERS[1]
        },
        PARAMETERS[2],
    ];
    static SHORT_ENUM_PARAMETERS: [ParameterDescriptorV1; 3] = [
        PARAMETERS[0],
        PARAMETERS[1],
        ParameterDescriptorV1 {
            enum_choices: &[CHOICES[0]],
            ..PARAMETERS[2]
        },
    ];
    static BAD_ENUM_DEFAULT_PARAMETERS: [ParameterDescriptorV1; 3] = [
        PARAMETERS[0],
        PARAMETERS[1],
        ParameterDescriptorV1 {
            default_value: 2.0,
            ..PARAMETERS[2]
        },
    ];
    static BAD_AUTOMATION_PARAMETERS: [ParameterDescriptorV1; 3] = [
        ParameterDescriptorV1 {
            automatable: false,
            ..PARAMETERS[0]
        },
        PARAMETERS[1],
        ParameterDescriptorV1 {
            automatable: true,
            ..PARAMETERS[2]
        },
    ];
    static NONFINITE_PARAMETERS: [ParameterDescriptorV1; 3] = [
        ParameterDescriptorV1 {
            minimum: Some(f32::NAN),
            ..PARAMETERS[0]
        },
        PARAMETERS[1],
        PARAMETERS[2],
    ];
    static NEGATIVE_ZERO_PARAMETERS: [ParameterDescriptorV1; 3] = [
        ParameterDescriptorV1 {
            default_value: -0.0,
            ..PARAMETERS[0]
        },
        PARAMETERS[1],
        PARAMETERS[2],
    ];
    static REVERSED_BOUND_PARAMETERS: [ParameterDescriptorV1; 3] = [
        ParameterDescriptorV1 {
            minimum: Some(24.0),
            maximum: Some(-24.0),
            ..PARAMETERS[0]
        },
        PARAMETERS[1],
        PARAMETERS[2],
    ];
    static CONTINUOUS_CHOICES_PARAMETERS: [ParameterDescriptorV1; 3] = [
        ParameterDescriptorV1 {
            enum_choices: &CHOICES,
            ..PARAMETERS[0]
        },
        PARAMETERS[1],
        PARAMETERS[2],
    ];
    static BOOLEAN_MAPPING_PARAMETERS: [ParameterDescriptorV1; 3] = [
        PARAMETERS[0],
        ParameterDescriptorV1 {
            mapping: ParameterMapping::Linear,
            ..PARAMETERS[1]
        },
        PARAMETERS[2],
    ];
    static OUT_OF_ORDER_ENUM_PARAMETERS: [ParameterDescriptorV1; 3] = [
        PARAMETERS[0],
        PARAMETERS[1],
        ParameterDescriptorV1 {
            enum_choices: &OUT_OF_ORDER_CHOICES,
            ..PARAMETERS[2]
        },
    ];
    static NONFINITE_ENUM_PARAMETERS: [ParameterDescriptorV1; 3] = [
        PARAMETERS[0],
        PARAMETERS[1],
        ParameterDescriptorV1 {
            enum_choices: &NONFINITE_CHOICES,
            ..PARAMETERS[2]
        },
    ];
    static CONTROL_LABEL_PARAMETERS: [ParameterDescriptorV1; 3] = [
        PARAMETERS[0],
        PARAMETERS[1],
        ParameterDescriptorV1 {
            enum_choices: &CONTROL_LABEL_CHOICES,
            ..PARAMETERS[2]
        },
    ];

    static PORTS_UNSORTED: [PortDescriptorV1; 3] = [
        PortDescriptorV1 {
            id: port_id("sidechain"),
            role: PortRole::SidechainInput,
            required: false,
            layout: PortLayout::DualMonoPlanar,
        },
        PortDescriptorV1 {
            id: port_id("main-out"),
            role: PortRole::MainOutput,
            required: true,
            layout: PortLayout::DualMonoPlanar,
        },
        PortDescriptorV1 {
            id: port_id("main-in"),
            role: PortRole::MainInput,
            required: true,
            layout: PortLayout::DualMonoPlanar,
        },
    ];
    static PORTS_SORTED: [PortDescriptorV1; 3] =
        [PORTS_UNSORTED[2], PORTS_UNSORTED[1], PORTS_UNSORTED[0]];
    static BAD_PORTS: [PortDescriptorV1; 3] = [
        PortDescriptorV1 {
            required: false,
            ..PORTS_SORTED[0]
        },
        PORTS_SORTED[1],
        PORTS_SORTED[2],
    ];
    static DUPLICATE_PORTS: [PortDescriptorV1; 3] = [
        PORTS_SORTED[0],
        PortDescriptorV1 {
            id: PORTS_SORTED[0].id,
            ..PORTS_SORTED[1]
        },
        PORTS_SORTED[2],
    ];
    static MISSING_OUTPUT_PORTS: [PortDescriptorV1; 1] = [PORTS_SORTED[0]];
    static TWO_SIDECHAIN_PORTS: [PortDescriptorV1; 4] = [
        PORTS_SORTED[0],
        PORTS_SORTED[1],
        PORTS_SORTED[2],
        PortDescriptorV1 {
            id: port_id("key-input"),
            ..PORTS_SORTED[2]
        },
    ];

    const fn quality(sample_rate: u32) -> QualityDescriptorV1 {
        QualityDescriptorV1 {
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

    static QUALITIES: [QualityDescriptorV1; 8] = [
        quality(44_100),
        quality(48_000),
        quality(88_200),
        quality(96_000),
        quality(176_400),
        quality(192_000),
        quality(352_800),
        quality(384_000),
    ];
    static BAD_RATE_QUALITIES: [QualityDescriptorV1; 8] = [
        quality(12_345),
        QUALITIES[1],
        QUALITIES[2],
        QUALITIES[3],
        QUALITIES[4],
        QUALITIES[5],
        QUALITIES[6],
        QUALITIES[7],
    ];
    static BAD_STATE_QUALITIES: [QualityDescriptorV1; 8] = [
        QualityDescriptorV1 {
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
    static OUT_OF_ORDER_QUALITIES: [QualityDescriptorV1; 8] = [
        QUALITIES[1],
        QUALITIES[0],
        QUALITIES[2],
        QUALITIES[3],
        QUALITIES[4],
        QUALITIES[5],
        QUALITIES[6],
        QUALITIES[7],
    ];
    static MISSING_RATE_QUALITIES: [QualityDescriptorV1; 7] = [
        QUALITIES[0],
        QUALITIES[1],
        QUALITIES[2],
        QUALITIES[4],
        QUALITIES[5],
        QUALITIES[6],
        QUALITIES[7],
    ];
    static DRAFT_ONLY_QUALITIES: [QualityDescriptorV1; 8] = [
        QualityDescriptorV1 {
            quality: EffectQuality::Draft,
            ..QUALITIES[0]
        },
        QualityDescriptorV1 {
            quality: EffectQuality::Draft,
            ..QUALITIES[1]
        },
        QualityDescriptorV1 {
            quality: EffectQuality::Draft,
            ..QUALITIES[2]
        },
        QualityDescriptorV1 {
            quality: EffectQuality::Draft,
            ..QUALITIES[3]
        },
        QualityDescriptorV1 {
            quality: EffectQuality::Draft,
            ..QUALITIES[4]
        },
        QualityDescriptorV1 {
            quality: EffectQuality::Draft,
            ..QUALITIES[5]
        },
        QualityDescriptorV1 {
            quality: EffectQuality::Draft,
            ..QUALITIES[6]
        },
        QualityDescriptorV1 {
            quality: EffectQuality::Draft,
            ..QUALITIES[7]
        },
    ];

    static DESCRIPTOR: EffectDescriptorV1 = EffectDescriptorV1 {
        id: effect_id("test.effect"),
        display_name: "Test Effect",
        contract_major: 1,
        contract_minor: u16::MAX,
        state_layout_version: 7,
        supported_link_modes: LinkModeSet::ALL,
        parameters: &PARAMETERS,
        ports: &PORTS_UNSORTED,
        qualities: &QUALITIES,
    };
    static SORTED_PORT_DESCRIPTOR: EffectDescriptorV1 = EffectDescriptorV1 {
        ports: &PORTS_SORTED,
        ..DESCRIPTOR
    };
    static EMPTY_PARAMETER_DESCRIPTOR: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &[],
        ..DESCRIPTOR
    };
    static NO_SIDECHAIN_PORTS: [PortDescriptorV1; 2] = [PORTS_SORTED[0], PORTS_SORTED[1]];
    static NO_SIDECHAIN_DESCRIPTOR: EffectDescriptorV1 = EffectDescriptorV1 {
        ports: &NO_SIDECHAIN_PORTS,
        ..DESCRIPTOR
    };
    static LAUNCH_QUALITIES: [QualityDescriptorV1; 4] =
        [QUALITIES[0], QUALITIES[1], QUALITIES[2], QUALITIES[3]];
    static LAUNCH_QUALITY_DESCRIPTOR: EffectDescriptorV1 = EffectDescriptorV1 {
        qualities: &LAUNCH_QUALITIES,
        ..DESCRIPTOR
    };
    static NO_CHOICE_PARAMETERS: [ParameterDescriptorV1; 3] = [
        PARAMETERS[0],
        PARAMETERS[1],
        ParameterDescriptorV1 {
            domain: ParameterDomain::Boolean,
            default_value: 1.0,
            enum_choices: &[],
            ..PARAMETERS[2]
        },
    ];
    static NO_CHOICE_DESCRIPTOR: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &NO_CHOICE_PARAMETERS,
        ..DESCRIPTOR
    };
    static BAD_CONTRACT: EffectDescriptorV1 = EffectDescriptorV1 {
        contract_major: 2,
        ..DESCRIPTOR
    };
    static BAD_STATE_VERSION: EffectDescriptorV1 = EffectDescriptorV1 {
        state_layout_version: 0,
        ..DESCRIPTOR
    };
    static BAD_DISPLAY: EffectDescriptorV1 = EffectDescriptorV1 {
        display_name: "Test\nEffect",
        ..DESCRIPTOR
    };
    static BAD_ZERO_ID: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &ZERO_ID_PARAMETERS,
        ..DESCRIPTOR
    };
    static BAD_DEFAULT: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &BAD_DEFAULT_PARAMETERS,
        ..DESCRIPTOR
    };
    static BAD_BOOLEAN: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &BAD_BOOLEAN_PARAMETERS,
        ..DESCRIPTOR
    };
    static BAD_SMOOTHING: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &BAD_SMOOTHING_PARAMETERS,
        ..DESCRIPTOR
    };
    static BAD_LABEL: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &BAD_LABEL_PARAMETERS,
        ..DESCRIPTOR
    };
    static BAD_PORT: EffectDescriptorV1 = EffectDescriptorV1 {
        ports: &BAD_PORTS,
        ..DESCRIPTOR
    };
    static BAD_RATE: EffectDescriptorV1 = EffectDescriptorV1 {
        qualities: &BAD_RATE_QUALITIES,
        ..DESCRIPTOR
    };
    static BAD_STATE: EffectDescriptorV1 = EffectDescriptorV1 {
        qualities: &BAD_STATE_QUALITIES,
        ..DESCRIPTOR
    };
    static DUPLICATE_ID: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &DUPLICATE_ID_PARAMETERS,
        ..DESCRIPTOR
    };
    static OUT_OF_ORDER_PARAMETER: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &OUT_OF_ORDER_PARAMETERS,
        ..DESCRIPTOR
    };
    static MISSING_MINIMUM: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &MISSING_MINIMUM_PARAMETERS,
        ..DESCRIPTOR
    };
    static BAD_LOG: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &BAD_LOG_PARAMETERS,
        ..DESCRIPTOR
    };
    static BAD_BOOLEAN_DEFAULT: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &BAD_BOOLEAN_DEFAULT_PARAMETERS,
        ..DESCRIPTOR
    };
    static SHORT_ENUM: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &SHORT_ENUM_PARAMETERS,
        ..DESCRIPTOR
    };
    static BAD_ENUM_DEFAULT: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &BAD_ENUM_DEFAULT_PARAMETERS,
        ..DESCRIPTOR
    };
    static BAD_AUTOMATION: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &BAD_AUTOMATION_PARAMETERS,
        ..DESCRIPTOR
    };
    static NONFINITE: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &NONFINITE_PARAMETERS,
        ..DESCRIPTOR
    };
    static NEGATIVE_ZERO: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &NEGATIVE_ZERO_PARAMETERS,
        ..DESCRIPTOR
    };
    static REVERSED_BOUND: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &REVERSED_BOUND_PARAMETERS,
        ..DESCRIPTOR
    };
    static CONTINUOUS_CHOICES: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &CONTINUOUS_CHOICES_PARAMETERS,
        ..DESCRIPTOR
    };
    static BOOLEAN_MAPPING: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &BOOLEAN_MAPPING_PARAMETERS,
        ..DESCRIPTOR
    };
    static OUT_OF_ORDER_ENUM: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &OUT_OF_ORDER_ENUM_PARAMETERS,
        ..DESCRIPTOR
    };
    static NONFINITE_ENUM: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &NONFINITE_ENUM_PARAMETERS,
        ..DESCRIPTOR
    };
    static CONTROL_LABEL: EffectDescriptorV1 = EffectDescriptorV1 {
        parameters: &CONTROL_LABEL_PARAMETERS,
        ..DESCRIPTOR
    };
    static DUPLICATE_PORT: EffectDescriptorV1 = EffectDescriptorV1 {
        ports: &DUPLICATE_PORTS,
        ..DESCRIPTOR
    };
    static MISSING_OUTPUT: EffectDescriptorV1 = EffectDescriptorV1 {
        ports: &MISSING_OUTPUT_PORTS,
        ..DESCRIPTOR
    };
    static TWO_SIDECHAINS: EffectDescriptorV1 = EffectDescriptorV1 {
        ports: &TWO_SIDECHAIN_PORTS,
        ..DESCRIPTOR
    };
    static OUT_OF_ORDER_QUALITY: EffectDescriptorV1 = EffectDescriptorV1 {
        qualities: &OUT_OF_ORDER_QUALITIES,
        ..DESCRIPTOR
    };
    static MISSING_RATE: EffectDescriptorV1 = EffectDescriptorV1 {
        qualities: &MISSING_RATE_QUALITIES,
        ..DESCRIPTOR
    };
    static MISSING_NORMAL: EffectDescriptorV1 = EffectDescriptorV1 {
        qualities: &DRAFT_ONLY_QUALITIES,
        ..DESCRIPTOR
    };
    static MULTI_ERROR: EffectDescriptorV1 = EffectDescriptorV1 {
        contract_major: 2,
        state_layout_version: 0,
        parameters: &DUPLICATE_ID_PARAMETERS,
        ports: &BAD_PORTS,
        qualities: &BAD_STATE_QUALITIES,
        ..DESCRIPTOR
    };

    fn encode(descriptor: &'static EffectDescriptorV1) -> Vec<u8> {
        let required = effect_descriptor_wire_v1_required_size(descriptor, 1 << 20).unwrap();
        let mut bytes = vec![0xa5; required as usize];
        assert_eq!(
            encode_effect_descriptor_wire_v1(descriptor, 1 << 20, &mut bytes),
            Ok(required)
        );
        bytes
    }

    fn semantic_test_view(bytes: &[u8]) -> BorrowedEffectDescriptorViewV1<'_> {
        let layout = Layout {
            total: read_u32(bytes, 16),
            parameters: read_u32(bytes, 48),
            ports: read_u32(bytes, 56),
            qualities: read_u32(bytes, 64),
            choices: read_u32(bytes, 72),
            parameter_offset: read_u32(bytes, 52),
            port_offset: read_u32(bytes, 60),
            quality_offset: read_u32(bytes, 68),
            choice_offset: read_u32(bytes, 76),
            string_bytes: read_u32(bytes, 80),
            string_offset: read_u32(bytes, 84),
        };
        let id_offset = read_u32(bytes, 32) as usize;
        let id_length = read_u32(bytes, 36) as usize;
        let display_offset = read_u32(bytes, 40) as usize;
        let display_length = read_u32(bytes, 44) as usize;
        BorrowedEffectDescriptorViewV1 {
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

    fn accepted_errors(descriptor: &'static EffectDescriptorV1) -> Vec<DescriptorError> {
        validate_descriptor_v1(descriptor).unwrap_err().0
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

    fn assert_parity(descriptor: &'static EffectDescriptorV1, mutate: impl FnOnce(&mut [u8])) {
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
        let verified = verify_effect_descriptor_wire_v1(&bytes, 1 << 20).unwrap();
        assert_eq!(verified.as_bytes(), bytes);
        assert_eq!(verified.parameter_count(), 3);
        assert_eq!(verified.port_count(), 3);
        assert_eq!(verified.quality_count(), 8);
        assert_eq!(verified.enum_choice_count(), 2);
        assert_eq!(verified.state_layout_version(), 7);
        assert_eq!(verified.supported_link_mode_bits(), 7);
        let identity = effect_descriptor_identity_v1(&bytes, 1 << 20).unwrap();
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
        let required = effect_descriptor_wire_v1_required_size(&DESCRIPTOR, 1 << 20).unwrap();
        let mut exact = vec![0x5a; required as usize + 16];
        assert_eq!(
            encode_effect_descriptor_wire_v1(&DESCRIPTOR, 1 << 20, &mut exact),
            Ok(required)
        );
        assert_eq!(&exact[required as usize..], &[0x5a; 16]);
        let mut short = vec![0x6b; required as usize - 1];
        let before = short.clone();
        assert_eq!(
            encode_effect_descriptor_wire_v1(&DESCRIPTOR, 1 << 20, &mut short),
            Err(Diagnostic::buffer_too_small(required))
        );
        assert_eq!(short, before);
        assert_eq!(
            effect_descriptor_wire_v1_required_size(&DESCRIPTOR, required - 1)
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
            effect_descriptor_identity_v1(&unsorted, 1 << 20),
            effect_descriptor_identity_v1(&sorted, 1 << 20)
        );
        let mut changed = unsorted.clone();
        write_u16(&mut changed, 22, 9);
        assert!(verify_effect_descriptor_wire_v1(&changed, 1 << 20).is_ok());
        assert_ne!(
            effect_descriptor_identity_v1(&unsorted, 1 << 20),
            effect_descriptor_identity_v1(&changed, 1 << 20)
        );
    }

    #[test]
    fn diagnostic_phase_order_and_wire_only_rejections_are_frozen() {
        let original = encode(&DESCRIPTOR);
        let mut bytes = original.clone();
        bytes[0] ^= 1;
        bytes[88] = 1;
        assert_eq!(
            verify_effect_descriptor_wire_v1(&bytes, 1 << 20)
                .unwrap_err()
                .code,
            Code::Header
        );
        let mut bytes = original.clone();
        bytes[88] = 1;
        write_u32(&mut bytes, 28, 0);
        assert_eq!(
            verify_effect_descriptor_wire_v1(&bytes, 1 << 20)
                .unwrap_err()
                .code,
            Code::Reserved
        );
        let mut bytes = original.clone();
        write_u32(&mut bytes, 52, 104);
        write_u32(&mut bytes, 28, 0);
        assert_eq!(
            verify_effect_descriptor_wire_v1(&bytes, 1 << 20)
                .unwrap_err()
                .code,
            Code::Offset
        );
        let mut trailing = original.clone();
        trailing.push(0);
        assert_eq!(
            verify_effect_descriptor_wire_v1(&trailing, 1 << 20)
                .unwrap_err()
                .code,
            Code::Length
        );
        let mut negative_zero = original;
        write_u32(&mut negative_zero, HEADER_BYTES + 44, (-0.0f32).to_bits());
        let error = verify_effect_descriptor_wire_v1(&negative_zero, 1 << 20).unwrap_err();
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
            let error = verify_effect_descriptor_wire_v1(&bytes, u32::MAX).unwrap_err();
            assert_eq!(
                (error.code, error.byte_offset, error.record_index),
                (
                    Code::Overflow,
                    expected_offset,
                    EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE
                )
            );
        }

        let original = encode(&DESCRIPTOR);
        let mut flags_before_reserved = original.clone();
        write_u32(&mut flags_before_reserved, HEADER_BYTES + 32, 16);
        write_u32(&mut flags_before_reserved, HEADER_BYTES + 72, 1);
        let error = verify_effect_descriptor_wire_v1(&flags_before_reserved, 1 << 20).unwrap_err();
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
        let error = verify_effect_descriptor_wire_v1(&header_text_before_table_offset, 1 << 20)
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
            verify_effect_descriptor_wire_v1(&port_text_before_choice_text, 1 << 20).unwrap_err();
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
            let error = verify_effect_descriptor_wire_v1(&bytes, 1 << 20).unwrap_err();
            assert_eq!(
                (error.code, error.byte_offset, error.record_index),
                (Code::Enum, 28, EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE)
            );
        }
        let mut effect_id = encode(&DESCRIPTOR);
        let offset = read_u32(&effect_id, 32) as usize;
        effect_id[offset] = b'A';
        assert_eq!(
            verify_effect_descriptor_wire_v1(&effect_id, 1 << 20)
                .unwrap_err()
                .code,
            Code::Text
        );
        let mut effect_id_rest = encode(&DESCRIPTOR);
        let offset = read_u32(&effect_id_rest, 32) as usize;
        effect_id_rest[offset + 4] = b'/';
        let error = verify_effect_descriptor_wire_v1(&effect_id_rest, 1 << 20).unwrap_err();
        assert_eq!(
            (error.code, error.byte_offset, error.record_index),
            (Code::Text, 32, EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE)
        );
        let mut port_id = encode(&DESCRIPTOR);
        let port = read_u32(&port_id, 60) as usize;
        let offset = read_u32(&port_id, port) as usize;
        port_id[offset] = b'A';
        assert_eq!(
            verify_effect_descriptor_wire_v1(&port_id, 1 << 20)
                .unwrap_err()
                .code,
            Code::Text
        );
        let mut port_id_rest = encode(&DESCRIPTOR);
        let port = read_u32(&port_id_rest, 60) as usize;
        let offset = read_u32(&port_id_rest, port) as usize;
        port_id_rest[offset + 4] = b'/';
        let error = verify_effect_descriptor_wire_v1(&port_id_rest, 1 << 20).unwrap_err();
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
        assert!(validate_descriptor_v1(&EMPTY_PARAMETER_DESCRIPTOR).is_ok());
        let bytes = encode(&EMPTY_PARAMETER_DESCRIPTOR);
        assert!(verify_effect_descriptor_wire_v1(&bytes, 1 << 20).is_ok());
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
            let error = bind_effect_descriptor_wire_v1(&DESCRIPTOR, bytes, 1 << 20).unwrap_err();
            assert_eq!(
                error.kind(),
                EffectDescriptorBindingErrorKindV1::StaticDescriptorMismatch,
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
            assert_mismatch(&bytes, offset as u32, EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE);
        }

        for field in [32, 40] {
            let mut bytes = encode(&DESCRIPTOR);
            let text = read_u32(&bytes, field) as usize;
            bytes[text] = if field == 32 { b'u' } else { b'R' };
            assert_mismatch(&bytes, field as u32, EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE);
        }

        assert_mismatch(
            &encode(&EMPTY_PARAMETER_DESCRIPTOR),
            48,
            EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE,
        );
        assert_mismatch(
            &encode(&NO_SIDECHAIN_DESCRIPTOR),
            56,
            EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE,
        );
        assert_mismatch(
            &encode(&LAUNCH_QUALITY_DESCRIPTOR),
            64,
            EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE,
        );
        assert_mismatch(
            &encode(&NO_CHOICE_DESCRIPTOR),
            72,
            EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE,
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
