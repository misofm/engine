pub const EFFECT_PACKAGE_UNAVAILABLE_INDEX: u32 = u32::MAX;
pub const EFFECT_PACKAGE_UNAVAILABLE_OFFSET: u64 = u64::MAX;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum EffectPackageDiagnosticCode {
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
pub struct EffectPackageDiagnostic {
    pub code: EffectPackageDiagnosticCode,
    pub detail: u32,
    pub artifact_index: u32,
    pub reserved: u32,
    pub byte_offset: u64,
    pub required_bytes: u64,
}

impl EffectPackageDiagnostic {
    pub const fn new(
        code: EffectPackageDiagnosticCode,
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
            code: EffectPackageDiagnosticCode::BufferTooSmall,
            detail: 0,
            artifact_index: EFFECT_PACKAGE_UNAVAILABLE_INDEX,
            reserved: 0,
            byte_offset: EFFECT_PACKAGE_UNAVAILABLE_OFFSET,
            required_bytes,
        }
    }
}

pub const EFFECT_STATE_UNAVAILABLE_INDEX: u32 = u32::MAX;
pub const EFFECT_STATE_UNAVAILABLE_OFFSET: u64 = u64::MAX;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum EffectStateDiagnosticCode {
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
pub struct EffectStateDiagnostic {
    pub code: EffectStateDiagnosticCode,
    pub detail: u32,
    pub item_index: u32,
    pub reserved: u32,
    pub byte_offset: u64,
    pub required_bytes: u64,
}

impl EffectStateDiagnostic {
    pub const fn new(
        code: EffectStateDiagnosticCode,
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
            code: EffectStateDiagnosticCode::BufferTooSmall,
            detail,
            item_index: EFFECT_STATE_UNAVAILABLE_INDEX,
            reserved: 0,
            byte_offset: EFFECT_STATE_UNAVAILABLE_OFFSET,
            required_bytes,
        }
    }
}

pub const EFFECT_DESCRIPTOR_WIRE_UNAVAILABLE: u32 = u32::MAX;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum EffectDescriptorWireDiagnosticCode {
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

impl EffectDescriptorWireDiagnosticCode {
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
pub struct EffectDescriptorWireDiagnostic {
    pub code: EffectDescriptorWireDiagnosticCode,
    pub byte_offset: u32,
    pub record_index: u32,
    pub required_bytes: u32,
}

impl EffectDescriptorWireDiagnostic {
    pub const fn new(
        code: EffectDescriptorWireDiagnosticCode,
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
            code: EffectDescriptorWireDiagnosticCode::BufferTooSmall,
            byte_offset: EFFECT_DESCRIPTOR_WIRE_UNAVAILABLE,
            record_index: EFFECT_DESCRIPTOR_WIRE_UNAVAILABLE,
            required_bytes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Issue 029 freezes the descriptor-wire diagnostic numbering and the matching
    /// `effect.descriptor.wire.<name>` strings; `as_str` is their only executable form.
    #[test]
    fn descriptor_wire_diagnostic_codes_and_strings_are_frozen() {
        for (code, value, text) in [
            (
                EffectDescriptorWireDiagnosticCode::Ok,
                0,
                "effect.descriptor.wire.ok",
            ),
            (
                EffectDescriptorWireDiagnosticCode::Null,
                1,
                "effect.descriptor.wire.null",
            ),
            (
                EffectDescriptorWireDiagnosticCode::Limit,
                2,
                "effect.descriptor.wire.limit",
            ),
            (
                EffectDescriptorWireDiagnosticCode::BufferTooSmall,
                3,
                "effect.descriptor.wire.buffer_too_small",
            ),
            (
                EffectDescriptorWireDiagnosticCode::Header,
                4,
                "effect.descriptor.wire.header",
            ),
            (
                EffectDescriptorWireDiagnosticCode::Length,
                5,
                "effect.descriptor.wire.length",
            ),
            (
                EffectDescriptorWireDiagnosticCode::Reserved,
                6,
                "effect.descriptor.wire.reserved",
            ),
            (
                EffectDescriptorWireDiagnosticCode::Enum,
                7,
                "effect.descriptor.wire.enum",
            ),
            (
                EffectDescriptorWireDiagnosticCode::Flags,
                8,
                "effect.descriptor.wire.flags",
            ),
            (
                EffectDescriptorWireDiagnosticCode::Order,
                9,
                "effect.descriptor.wire.order",
            ),
            (
                EffectDescriptorWireDiagnosticCode::Offset,
                10,
                "effect.descriptor.wire.offset",
            ),
            (
                EffectDescriptorWireDiagnosticCode::Text,
                11,
                "effect.descriptor.wire.text",
            ),
            (
                EffectDescriptorWireDiagnosticCode::Float,
                12,
                "effect.descriptor.wire.float",
            ),
            (
                EffectDescriptorWireDiagnosticCode::Semantic,
                13,
                "effect.descriptor.wire.semantic",
            ),
            (
                EffectDescriptorWireDiagnosticCode::Overflow,
                14,
                "effect.descriptor.wire.overflow",
            ),
        ] {
            assert_eq!(code as u32, value, "{text}");
            assert_eq!(code.as_str(), text);
        }
    }
}
