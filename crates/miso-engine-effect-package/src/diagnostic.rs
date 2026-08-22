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

pub const EFFECT_PACKAGE_V1_UNAVAILABLE_INDEX: u32 = u32::MAX;
pub const EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET: u64 = u64::MAX;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum EffectPackageDiagnosticCodeV1 {
    Ok = 0,
    Limit = 1,
    BufferTooSmall = 2,
    Header = 3,
    Length = 4,
    Reserved = 5,
    Enum = 6,
    Offset = 7,
    Order = 8,
    Path = 9,
    Target = 10,
    Features = 11,
    Descriptor = 12,
    Hash = 13,
    Unavailable = 14,
    Cid = 15,
    Overflow = 16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct EffectPackageDiagnosticV1 {
    pub code: EffectPackageDiagnosticCodeV1,
    pub detail: u32,
    pub artifact_index: u32,
    pub reserved: u32,
    pub byte_offset: u64,
    pub required_bytes: u64,
}

impl EffectPackageDiagnosticV1 {
    pub const fn new(
        code: EffectPackageDiagnosticCodeV1,
        detail: u32,
        artifact_index: u32,
        byte_offset: u64,
    ) -> Self {
        Self {
            code,
            detail,
            artifact_index,
            reserved: 0,
            byte_offset,
            required_bytes: 0,
        }
    }

    pub const fn buffer_too_small(required_bytes: u64) -> Self {
        Self {
            code: EffectPackageDiagnosticCodeV1::BufferTooSmall,
            detail: 0,
            artifact_index: EFFECT_PACKAGE_V1_UNAVAILABLE_INDEX,
            reserved: 0,
            byte_offset: EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET,
            required_bytes,
        }
    }
}

pub const EFFECT_STATE_V1_UNAVAILABLE_INDEX: u32 = u32::MAX;
pub const EFFECT_STATE_V1_UNAVAILABLE_OFFSET: u64 = u64::MAX;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum EffectStateDiagnosticCodeV1 {
    Ok = 0,
    Limit = 1,
    BufferTooSmall = 2,
    Header = 3,
    Length = 4,
    Reserved = 5,
    Enum = 6,
    Order = 7,
    Text = 8,
    Descriptor = 9,
    Digest = 10,
    Metadata = 11,
    InitialValues = 12,
    Payload = 13,
    Factory = 14,
    Restore = 15,
    Overflow = 16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct EffectStateDiagnosticV1 {
    pub code: EffectStateDiagnosticCodeV1,
    pub detail: u32,
    pub item_index: u32,
    pub reserved: u32,
    pub byte_offset: u64,
    pub required_bytes: u64,
}

impl EffectStateDiagnosticV1 {
    pub const fn new(
        code: EffectStateDiagnosticCodeV1,
        detail: u32,
        item_index: u32,
        byte_offset: u64,
    ) -> Self {
        Self {
            code,
            detail,
            item_index,
            reserved: 0,
            byte_offset,
            required_bytes: 0,
        }
    }

    pub const fn buffer_too_small(detail: u32, required_bytes: u64) -> Self {
        Self {
            code: EffectStateDiagnosticCodeV1::BufferTooSmall,
            detail,
            item_index: EFFECT_STATE_V1_UNAVAILABLE_INDEX,
            reserved: 0,
            byte_offset: EFFECT_STATE_V1_UNAVAILABLE_OFFSET,
            required_bytes,
        }
    }
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
#[repr(C)]
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
