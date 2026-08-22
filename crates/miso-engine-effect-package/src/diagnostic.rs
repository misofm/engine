#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PackageError {
    Header,
    Length,
    Limit,
    Canonical,
    Text,
    Hash,
    Unavailable,
    State,
}

pub const EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE: u32 = u32::MAX;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum EffectDescriptorWireDiagnosticCodeV1 {
    Ok = 0,
    Null = 1,
    Limit = 2,
    BufferTooSmall = 3,
    Header = 4,
    Length = 5,
    Reserved = 6,
    Enum = 7,
    Flags = 8,
    Order = 9,
    Offset = 10,
    Text = 11,
    Float = 12,
    Semantic = 13,
    Overflow = 14,
}

impl EffectDescriptorWireDiagnosticCodeV1 {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "effect.descriptor.wire.ok",
            Self::Null => "effect.descriptor.wire.null",
            Self::Limit => "effect.descriptor.wire.limit",
            Self::BufferTooSmall => "effect.descriptor.wire.buffer_too_small",
            Self::Header => "effect.descriptor.wire.header",
            Self::Length => "effect.descriptor.wire.length",
            Self::Reserved => "effect.descriptor.wire.reserved",
            Self::Enum => "effect.descriptor.wire.enum",
            Self::Flags => "effect.descriptor.wire.flags",
            Self::Order => "effect.descriptor.wire.order",
            Self::Offset => "effect.descriptor.wire.offset",
            Self::Text => "effect.descriptor.wire.text",
            Self::Float => "effect.descriptor.wire.float",
            Self::Semantic => "effect.descriptor.wire.semantic",
            Self::Overflow => "effect.descriptor.wire.overflow",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectDescriptorWireDiagnosticV1 {
    pub code: EffectDescriptorWireDiagnosticCodeV1,
    pub byte_offset: u32,
    pub record_index: u32,
    pub required_bytes: u32,
}

impl EffectDescriptorWireDiagnosticV1 {
    pub const fn new(
        code: EffectDescriptorWireDiagnosticCodeV1,
        byte_offset: u32,
        record_index: u32,
    ) -> Self {
        Self {
            code,
            byte_offset,
            record_index,
            required_bytes: 0,
        }
    }

    pub const fn buffer_too_small(required_bytes: u32) -> Self {
        Self {
            code: EffectDescriptorWireDiagnosticCodeV1::BufferTooSmall,
            byte_offset: EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE,
            record_index: EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE,
            required_bytes,
        }
    }
}
