//! Declarative message-field registry for shared typed protocol payloads.
//!
//! Every typed payload family consumes this metadata through the shared bounded reader. The
//! registry is never a second encoder or decoder.

use crate::message_wire::{ParameterChannel, ParameterRack, ParameterUnit};

pub(crate) const fn parameter_rack_wire(value: ParameterRack) -> u8 {
    match value {
        ParameterRack::Inserts => 2,
        ParameterRack::Builtins => 4,
        ParameterRack::Console => 5,
    }
}

pub(crate) const fn parameter_rack_from_wire(
    value: u8,
) -> Result<ParameterRack, crate::DecodeError> {
    // `1` (`simd1`) and `3` (`simd2`) are retired by decision 12: refused like any unallocated
    // code, never reinterpreted.
    match value {
        2 => Ok(ParameterRack::Inserts),
        4 => Ok(ParameterRack::Builtins),
        5 => Ok(ParameterRack::Console),
        _ => Err(crate::DecodeError::InvalidTlv),
    }
}

pub(crate) const fn parameter_channel_wire(value: ParameterChannel) -> u8 {
    match value {
        ParameterChannel::Left => 1,
        ParameterChannel::Right => 2,
        ParameterChannel::Both => 3,
    }
}

pub(crate) const fn parameter_channel_from_wire(
    value: u8,
) -> Result<ParameterChannel, crate::DecodeError> {
    match value {
        1 => Ok(ParameterChannel::Left),
        2 => Ok(ParameterChannel::Right),
        3 => Ok(ParameterChannel::Both),
        _ => Err(crate::DecodeError::InvalidTlv),
    }
}

pub(crate) const fn parameter_unit_wire(value: ParameterUnit) -> u8 {
    match value {
        ParameterUnit::Db => 1,
        ParameterUnit::Hz => 2,
        ParameterUnit::Milliseconds => 3,
        ParameterUnit::Samples => 4,
        ParameterUnit::Linear => 5,
        ParameterUnit::Ratio => 6,
    }
}

pub(crate) const fn parameter_unit_from_wire(
    value: u8,
) -> Result<ParameterUnit, crate::DecodeError> {
    match value {
        1 => Ok(ParameterUnit::Db),
        2 => Ok(ParameterUnit::Hz),
        3 => Ok(ParameterUnit::Milliseconds),
        4 => Ok(ParameterUnit::Samples),
        5 => Ok(ParameterUnit::Linear),
        6 => Ok(ParameterUnit::Ratio),
        _ => Err(crate::DecodeError::InvalidTlv),
    }
}

pub(crate) const fn session_parameter_rack_wire(value: ::session::RackName) -> u8 {
    parameter_rack_wire(ParameterRack::from_session(value))
}

pub(crate) const fn session_parameter_rack_from_wire(
    value: u8,
) -> Result<::session::RackName, crate::DecodeError> {
    match parameter_rack_from_wire(value) {
        Ok(value) => Ok(value.into_session()),
        Err(error) => Err(error),
    }
}

pub(crate) const fn session_parameter_channel_wire(value: ::session::ParameterChannel) -> u8 {
    parameter_channel_wire(ParameterChannel::from_session(value))
}

pub(crate) const fn session_parameter_channel_from_wire(
    value: u8,
) -> Result<::session::ParameterChannel, crate::DecodeError> {
    match parameter_channel_from_wire(value) {
        Ok(value) => Ok(value.into_session()),
        Err(error) => Err(error),
    }
}

pub(crate) const fn session_parameter_unit_wire(value: ::session::ParameterUnit) -> u8 {
    parameter_unit_wire(ParameterUnit::from_session(value))
}

pub(crate) const fn session_parameter_unit_from_wire(
    value: u8,
) -> Result<::session::ParameterUnit, crate::DecodeError> {
    match parameter_unit_from_wire(value) {
        Ok(value) => Ok(value.into_session()),
        Err(error) => Err(error),
    }
}

/// Frozen BTLV wire type.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Wire {
    U8 = 1,
    U16 = 2,
    U32 = 3,
    U64 = 4,
    I64 = 5,
    F32 = 6,
    F64 = 7,
    Bool = 8,
    Utf8 = 9,
    Bytes = 10,
    Message = 11,
    PackedU16 = 12,
    PackedU32 = 13,
    PackedU64 = 14,
    PackedF32 = 15,
}

impl Wire {
    pub(crate) const fn raw(self) -> u8 {
        self as u8
    }

    /// The type's spelling in the frame table a client generates its codec from.
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::I64 => "i64",
            Self::F32 => "f32",
            Self::F64 => "f64",
            Self::Bool => "bool",
            Self::Utf8 => "utf8",
            Self::Bytes => "bytes",
            Self::Message => "message",
            Self::PackedU16 => "packedU16",
            Self::PackedU32 => "packedU32",
            Self::PackedU64 => "packedU64",
            Self::PackedF32 => "packedF32",
        }
    }

    pub(crate) const fn from_raw(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::U8),
            2 => Some(Self::U16),
            3 => Some(Self::U32),
            4 => Some(Self::U64),
            5 => Some(Self::I64),
            6 => Some(Self::F32),
            7 => Some(Self::F64),
            8 => Some(Self::Bool),
            9 => Some(Self::Utf8),
            10 => Some(Self::Bytes),
            11 => Some(Self::Message),
            12 => Some(Self::PackedU16),
            13 => Some(Self::PackedU32),
            14 => Some(Self::PackedU64),
            15 => Some(Self::PackedF32),
            _ => None,
        }
    }
}

/// One declarative field contract.
#[derive(Clone, Copy, Debug)]
pub(crate) struct FieldSpec {
    pub(crate) id: u16,
    pub(crate) wire: Wire,
    pub(crate) mandatory: bool,
    pub(crate) repeated: bool,
    pub(crate) nested: Option<&'static MessageSpec>,
}

impl FieldSpec {
    pub(crate) const fn req(id: u16, wire: Wire) -> Self {
        Self {
            id,
            wire,
            mandatory: true,
            repeated: false,
            nested: None,
        }
    }

    pub(crate) const fn opt(id: u16, wire: Wire) -> Self {
        Self {
            id,
            wire,
            mandatory: false,
            repeated: false,
            nested: None,
        }
    }

    pub(crate) const fn msg(
        id: u16,
        mandatory: bool,
        repeated: bool,
        nested: &'static MessageSpec,
    ) -> Self {
        Self {
            id,
            wire: Wire::Message,
            mandatory,
            repeated,
            nested: Some(nested),
        }
    }
}

/// Named message whose fields are strictly ascending.
#[derive(Debug)]
pub(crate) struct MessageSpec {
    pub(crate) name: &'static str,
    pub(crate) fields: &'static [FieldSpec],
}

impl MessageSpec {
    /// Derive the emitted top-level field count from this schema and the occurrences of its
    /// optional or repeated fields. Required singular fields contribute exactly one.
    pub(crate) fn field_count(
        &'static self,
        variable: &[(FieldSpec, usize)],
    ) -> Result<u32, crate::EncodeError> {
        for (candidate, _) in variable {
            let Some(field) = self.fields.iter().find(|field| field.id == candidate.id) else {
                return Err(crate::EncodeError::LimitExceeded);
            };
            if (!field.repeated && field.mandatory)
                || candidate.wire != field.wire
                || candidate.mandatory != field.mandatory
                || candidate.repeated != field.repeated
                || variable
                    .iter()
                    .filter(|(other, _)| other.id == candidate.id)
                    .count()
                    != 1
            {
                return Err(crate::EncodeError::LimitExceeded);
            }
        }
        let mut total = 0_usize;
        for field in self.fields {
            let occurrences = if field.repeated || !field.mandatory {
                variable
                    .iter()
                    .find(|(candidate, _)| candidate.id == field.id)
                    .map_or(0, |(_, count)| *count)
            } else {
                1
            };
            if !field.repeated && occurrences > 1 {
                return Err(crate::EncodeError::LimitExceeded);
            }
            total = total
                .checked_add(occurrences)
                .ok_or(crate::EncodeError::LimitExceeded)?;
        }
        u32::try_from(total).map_err(|_| crate::EncodeError::LimitExceeded)
    }
}

pub(crate) mod capabilities_request {
    use super::MessageSpec;

    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "CapabilitiesGetCommand",
        fields: &[],
    };
}

pub(crate) mod enum_choice {
    use super::{FieldSpec, MessageSpec, Wire};

    pub(crate) const VALUE: FieldSpec = FieldSpec::req(1, Wire::F32);
    pub(crate) const LABEL: FieldSpec = FieldSpec::req(2, Wire::Utf8);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "EnumChoice",
        fields: &[VALUE, LABEL],
    };
}

pub(crate) mod snapshot_request {
    use super::*;
    pub(crate) const OFFSET: FieldSpec = FieldSpec::req(1, Wire::U64);
    pub(crate) const MAXIMUM_BYTES: FieldSpec = FieldSpec::req(2, Wire::U32);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "SessionSnapshotRequest",
        fields: &[OFFSET, MAXIMUM_BYTES],
    };
}

pub(crate) mod snapshot {
    use super::*;
    pub(crate) const TOTAL_BYTES: FieldSpec = FieldSpec::req(1, Wire::U64);
    pub(crate) const OFFSET: FieldSpec = FieldSpec::req(2, Wire::U64);
    pub(crate) const CANONICAL_JSON_CHUNK: FieldSpec = FieldSpec::req(3, Wire::Bytes);
    pub(crate) const EOF: FieldSpec = FieldSpec::req(4, Wire::Bool);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "SessionSnapshot",
        fields: &[TOTAL_BYTES, OFFSET, CANONICAL_JSON_CHUNK, EOF],
    };
}

pub(crate) mod transaction_applied {
    use super::*;
    pub(crate) const APPLIED_OPERATIONS: FieldSpec = FieldSpec::req(1, Wire::U32);
    pub(crate) const EDIT_PATH: FieldSpec = FieldSpec::req(2, Wire::U8);
    /// #1585 D5: one arm tag per `Observe` edit, present exactly when the transaction holds one.
    pub(crate) const OBSERVE_ARM_TAGS: FieldSpec = FieldSpec::opt(3, Wire::PackedU64);
    /// #1585 D5: the tags of the subscriptions the commit ended, present exactly when it ended one.
    pub(crate) const REMOVED_ARM_TAGS: FieldSpec = FieldSpec::opt(4, Wire::PackedU64);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "TransactionApplied",
        fields: &[
            APPLIED_OPERATIONS,
            EDIT_PATH,
            OBSERVE_ARM_TAGS,
            REMOVED_ARM_TAGS,
        ],
    };
}

/// `SESSION_REPLACE` (#1386): the frame names the document the host passes beside it, by its exact
/// length and its SHA-256; it never carries the document. Its success reply is
/// [`transaction_applied`].
pub(crate) mod session_replace_request {
    use super::*;
    pub(crate) const DOCUMENT_BYTES: FieldSpec = FieldSpec::req(1, Wire::U32);
    /// Exactly 32 bytes.
    pub(crate) const DOCUMENT_SHA256: FieldSpec = FieldSpec::req(2, Wire::Bytes);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "SessionReplaceRequest",
        fields: &[DOCUMENT_BYTES, DOCUMENT_SHA256],
    };
}

pub(crate) mod session_committed {
    use super::*;
    pub(crate) const EVENT_SEQUENCE: FieldSpec = FieldSpec::req(1, Wire::U64);
    pub(crate) const ORIGIN_REQUEST_ID: FieldSpec = FieldSpec::req(2, Wire::U64);
    pub(crate) const PREVIOUS_REVISION: FieldSpec = FieldSpec::req(3, Wire::U64);
    pub(crate) const APPLIED_OPERATIONS: FieldSpec = FieldSpec::req(4, Wire::U32);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "SessionCommitted",
        fields: &[
            EVENT_SEQUENCE,
            ORIGIN_REQUEST_ID,
            PREVIOUS_REVISION,
            APPLIED_OPERATIONS,
        ],
    };
}

/// The control list (decision 17, C4): `CONTROL_LIST_GET` and `CONTROL_STATE_GET` share one
/// request, and the list page carries one `control_entry` per owner header, section header and
/// control row.
pub(crate) mod control_list_request {
    use super::*;
    pub(crate) const STRUCTURE_EPOCH: FieldSpec = FieldSpec::req(1, Wire::U64);
    pub(crate) const AFTER_HANDLE: FieldSpec = FieldSpec::req(2, Wire::U32);
    pub(crate) const LIMIT: FieldSpec = FieldSpec::req(3, Wire::U16);
    pub(crate) const OWNER_KIND: FieldSpec = FieldSpec::opt(4, Wire::U8);
    pub(crate) const OWNER_ID: FieldSpec = FieldSpec::opt(5, Wire::Utf8);
    /// Only the window's header rows (#2206); written only when true.
    pub(crate) const HEADERS_ONLY: FieldSpec = FieldSpec::opt(6, Wire::Bool);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ControlListRequest",
        fields: &[
            STRUCTURE_EPOCH,
            AFTER_HANDLE,
            LIMIT,
            OWNER_KIND,
            OWNER_ID,
            HEADERS_ONLY,
        ],
    };
}

pub(crate) mod control_address {
    use super::*;
    pub(crate) const OWNER_KIND: FieldSpec = FieldSpec::req(1, Wire::U8);
    pub(crate) const OWNER_ID: FieldSpec = FieldSpec::opt(2, Wire::Utf8);
    pub(crate) const SECTION: FieldSpec = FieldSpec::req(3, Wire::U8);
    pub(crate) const SLOT_ID: FieldSpec = FieldSpec::opt(4, Wire::Utf8);
    pub(crate) const CONTROL: FieldSpec = FieldSpec::opt(5, Wire::U32);
    pub(crate) const LANE: FieldSpec = FieldSpec::opt(6, Wire::U8);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ControlAddress",
        fields: &[OWNER_KIND, OWNER_ID, SECTION, SLOT_ID, CONTROL, LANE],
    };
}

/// The address message of a `SET_CONTROL` edit (#1678): the control list's address fields, and a
/// seventh, the native effect the edit was written for. The list's own rows and an automation target
/// use [`control_address`], which has no seventh field.
pub(crate) mod set_control_address {
    use super::*;
    pub(crate) const OWNER_KIND: FieldSpec = control_address::OWNER_KIND;
    pub(crate) const OWNER_ID: FieldSpec = control_address::OWNER_ID;
    pub(crate) const SECTION: FieldSpec = control_address::SECTION;
    pub(crate) const SLOT_ID: FieldSpec = control_address::SLOT_ID;
    pub(crate) const CONTROL: FieldSpec = control_address::CONTROL;
    pub(crate) const LANE: FieldSpec = control_address::LANE;
    /// The native effect ID the edit was written for; present only in an `inserts` or `console`
    /// section. The core refuses the edit when that insert or slot runs another effect.
    pub(crate) const EXPECTED_EFFECT: FieldSpec = FieldSpec::opt(7, Wire::Utf8);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "SetControlAddress",
        fields: &[
            OWNER_KIND,
            OWNER_ID,
            SECTION,
            SLOT_ID,
            CONTROL,
            LANE,
            EXPECTED_EFFECT,
        ],
    };
}

pub(crate) mod control_entry {
    use super::{FieldSpec, MessageSpec, Wire, control_address, enum_choice};

    pub(crate) const HANDLE: FieldSpec = FieldSpec::req(1, Wire::U32);
    pub(crate) const ROW_KIND: FieldSpec = FieldSpec::req(2, Wire::U8);
    pub(crate) const ADDRESS: FieldSpec = FieldSpec::msg(3, true, false, &control_address::SPEC);
    // A header row.
    pub(crate) const HEADER_KIND: FieldSpec = FieldSpec::opt(4, Wire::U8);
    pub(crate) const HEADER_NAME: FieldSpec = FieldSpec::opt(5, Wire::Utf8);
    pub(crate) const EFFECT: FieldSpec = FieldSpec::opt(6, Wire::Utf8);
    pub(crate) const SOURCE: FieldSpec = FieldSpec::opt(7, Wire::Utf8);
    pub(crate) const DESTINATION: FieldSpec = FieldSpec::opt(8, Wire::Utf8);
    pub(crate) const TAP: FieldSpec = FieldSpec::opt(9, Wire::U8);
    // A control row.
    pub(crate) const NAME: FieldSpec = FieldSpec::opt(10, Wire::Utf8);
    pub(crate) const DISPLAY_NAME: FieldSpec = FieldSpec::opt(11, Wire::Utf8);
    pub(crate) const DISPLAY_UNIT: FieldSpec = FieldSpec::opt(12, Wire::Utf8);
    pub(crate) const UNIT: FieldSpec = FieldSpec::opt(13, Wire::U8);
    pub(crate) const KIND: FieldSpec = FieldSpec::opt(14, Wire::U8);
    pub(crate) const MINIMUM: FieldSpec = FieldSpec::opt(15, Wire::F32);
    pub(crate) const MAXIMUM: FieldSpec = FieldSpec::opt(16, Wire::F32);
    pub(crate) const DISABLED_VALUE: FieldSpec = FieldSpec::opt(17, Wire::F32);
    pub(crate) const DEFAULT: FieldSpec = FieldSpec::opt(18, Wire::F32);
    pub(crate) const TAPER: FieldSpec = FieldSpec::opt(19, Wire::U8);
    pub(crate) const STEP: FieldSpec = FieldSpec::opt(20, Wire::F32);
    pub(crate) const PRECISION: FieldSpec = FieldSpec::opt(21, Wire::U8);
    pub(crate) const LADDER: FieldSpec = FieldSpec::opt(22, Wire::PackedF32);
    pub(crate) const CHOICE: FieldSpec = FieldSpec::msg(23, false, true, &enum_choice::SPEC);
    pub(crate) const RAMP_ROW: FieldSpec = FieldSpec::opt(24, Wire::U8);
    pub(crate) const DEFAULT_RAMP_SAMPLES: FieldSpec = FieldSpec::opt(25, Wire::U32);
    pub(crate) const CLASS: FieldSpec = FieldSpec::opt(26, Wire::U8);
    pub(crate) const REASON: FieldSpec = FieldSpec::opt(27, Wire::U8);
    pub(crate) const REASON_ISSUE: FieldSpec = FieldSpec::opt(28, Wire::U32);
    pub(crate) const REOPEN: FieldSpec = FieldSpec::opt(29, Wire::Utf8);
    pub(crate) const SCOPE: FieldSpec = FieldSpec::opt(30, Wire::U8);
    pub(crate) const AUTOMATABLE: FieldSpec = FieldSpec::opt(31, Wire::Bool);
    pub(crate) const FAMILY: FieldSpec = FieldSpec::opt(32, Wire::U8);
    pub(crate) const ACTIVE: FieldSpec = FieldSpec::opt(33, Wire::Bool);
    pub(crate) const VALUE: FieldSpec = FieldSpec::opt(34, Wire::F32);
    // #1650 D4: a rendered stored automation drives the row's lane.
    pub(crate) const AUTOMATED: FieldSpec = FieldSpec::opt(35, Wire::Bool);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ControlEntry",
        fields: &[
            HANDLE,
            ROW_KIND,
            ADDRESS,
            HEADER_KIND,
            HEADER_NAME,
            EFFECT,
            SOURCE,
            DESTINATION,
            TAP,
            NAME,
            DISPLAY_NAME,
            DISPLAY_UNIT,
            UNIT,
            KIND,
            MINIMUM,
            MAXIMUM,
            DISABLED_VALUE,
            DEFAULT,
            TAPER,
            STEP,
            PRECISION,
            LADDER,
            CHOICE,
            RAMP_ROW,
            DEFAULT_RAMP_SAMPLES,
            CLASS,
            REASON,
            REASON_ISSUE,
            REOPEN,
            SCOPE,
            AUTOMATABLE,
            FAMILY,
            ACTIVE,
            VALUE,
            AUTOMATED,
        ],
    };
}

pub(crate) mod control_list_page {
    use super::*;
    pub(crate) const REVISION: FieldSpec = FieldSpec::req(1, Wire::U64);
    pub(crate) const STRUCTURE_EPOCH: FieldSpec = FieldSpec::req(2, Wire::U64);
    pub(crate) const LAST_HANDLE: FieldSpec = FieldSpec::req(3, Wire::U32);
    pub(crate) const EOF: FieldSpec = FieldSpec::req(4, Wire::Bool);
    pub(crate) const ENTRY: FieldSpec = FieldSpec::msg(5, true, true, &control_entry::SPEC);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ControlListPage",
        fields: &[REVISION, STRUCTURE_EPOCH, LAST_HANDLE, EOF, ENTRY],
    };
}

pub(crate) mod control_state_page {
    use super::*;
    pub(crate) const REVISION: FieldSpec = FieldSpec::req(1, Wire::U64);
    pub(crate) const STRUCTURE_EPOCH: FieldSpec = FieldSpec::req(2, Wire::U64);
    pub(crate) const LAST_HANDLE: FieldSpec = FieldSpec::req(3, Wire::U32);
    pub(crate) const EOF: FieldSpec = FieldSpec::req(4, Wire::Bool);
    pub(crate) const COUNT: FieldSpec = FieldSpec::req(5, Wire::U16);
    pub(crate) const RECORD_BYTES: FieldSpec = FieldSpec::req(6, Wire::U16);
    pub(crate) const RECORDS: FieldSpec = FieldSpec::req(7, Wire::Bytes);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ControlStatePage",
        fields: &[
            REVISION,
            STRUCTURE_EPOCH,
            LAST_HANDLE,
            EOF,
            COUNT,
            RECORD_BYTES,
            RECORDS,
        ],
    };
}

/// `RESPONSE_GET` (decision 17, C13): the request names exactly one of a configuration, a control
/// address or a strip, then the axis and the output shape.
pub(crate) mod response_configuration {
    use super::*;
    pub(crate) const KIND: FieldSpec = FieldSpec::req(1, Wire::U8);
    pub(crate) const EFFECT: FieldSpec = FieldSpec::opt(2, Wire::Utf8);
    pub(crate) const QUALITY: FieldSpec = FieldSpec::opt(3, Wire::U8);
    pub(crate) const LINK_MODE: FieldSpec = FieldSpec::opt(4, Wire::U8);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ResponseConfiguration",
        fields: &[KIND, EFFECT, QUALITY, LINK_MODE],
    };
}

pub(crate) mod response_override {
    use super::*;
    pub(crate) const CONTROL: FieldSpec = FieldSpec::req(1, Wire::U32);
    pub(crate) const LANE: FieldSpec = FieldSpec::req(2, Wire::U8);
    pub(crate) const VALUE: FieldSpec = FieldSpec::req(3, Wire::F32);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ResponseOverride",
        fields: &[CONTROL, LANE, VALUE],
    };
}

pub(crate) mod response_request {
    use super::*;
    pub(crate) const CONFIGURATION: FieldSpec =
        FieldSpec::msg(1, false, false, &response_configuration::SPEC);
    pub(crate) const ADDRESS: FieldSpec = FieldSpec::msg(2, false, false, &control_address::SPEC);
    pub(crate) const STRIP_ID: FieldSpec = FieldSpec::opt(3, Wire::Utf8);
    pub(crate) const GRID: FieldSpec = FieldSpec::req(4, Wire::U8);
    pub(crate) const POINTS: FieldSpec = FieldSpec::req(5, Wire::U32);
    pub(crate) const MINIMUM_HZ: FieldSpec = FieldSpec::req(6, Wire::F32);
    pub(crate) const MAXIMUM_HZ: FieldSpec = FieldSpec::req(7, Wire::F32);
    pub(crate) const CHANNELS: FieldSpec = FieldSpec::req(8, Wire::U8);
    pub(crate) const SECTIONS: FieldSpec = FieldSpec::req(9, Wire::Bool);
    pub(crate) const OVERRIDE: FieldSpec =
        FieldSpec::msg(10, false, true, &response_override::SPEC);
    pub(crate) const CONFIGURATION_ID: FieldSpec = FieldSpec::opt(11, Wire::U64);
    pub(crate) const SAMPLE_RATE_HZ: FieldSpec = FieldSpec::opt(12, Wire::U32);
    pub(crate) const QUANTUM_FRAMES: FieldSpec = FieldSpec::opt(13, Wire::U32);
    pub(crate) const MAXIMUM_PREPARED_BYTES: FieldSpec = FieldSpec::opt(14, Wire::U64);
    pub(crate) const MAXIMUM_STATE_BYTES: FieldSpec = FieldSpec::opt(15, Wire::U64);
    pub(crate) const MAXIMUM_SCRATCH_BYTES: FieldSpec = FieldSpec::opt(16, Wire::U64);
    pub(crate) const MAXIMUM_AUTOMATION_SPANS: FieldSpec = FieldSpec::opt(17, Wire::U32);
    pub(crate) const MAXIMUM_REPLY_BYTES: FieldSpec = FieldSpec::opt(18, Wire::U32);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ResponseRequest",
        fields: &[
            CONFIGURATION,
            ADDRESS,
            STRIP_ID,
            GRID,
            POINTS,
            MINIMUM_HZ,
            MAXIMUM_HZ,
            CHANNELS,
            SECTIONS,
            OVERRIDE,
            CONFIGURATION_ID,
            SAMPLE_RATE_HZ,
            QUANTUM_FRAMES,
            MAXIMUM_PREPARED_BYTES,
            MAXIMUM_STATE_BYTES,
            MAXIMUM_SCRATCH_BYTES,
            MAXIMUM_AUTOMATION_SPANS,
            MAXIMUM_REPLY_BYTES,
        ],
    };
}

pub(crate) mod response_result {
    use super::*;
    pub(crate) const REQUEST_TAG: FieldSpec = FieldSpec::opt(1, Wire::U64);
    pub(crate) const SAMPLE_RATE_HZ: FieldSpec = FieldSpec::opt(2, Wire::U32);
    pub(crate) const CONFIGURATION_ID: FieldSpec = FieldSpec::opt(3, Wire::U64);
    pub(crate) const FLOOR_DB: FieldSpec = FieldSpec::opt(4, Wire::F32);
    pub(crate) const BYPASS: FieldSpec = FieldSpec::opt(5, Wire::Bool);
    pub(crate) const SECTION_COUNT: FieldSpec = FieldSpec::opt(6, Wire::U32);
    pub(crate) const ENABLED_LEFT: FieldSpec = FieldSpec::opt(7, Wire::U32);
    pub(crate) const ENABLED_RIGHT: FieldSpec = FieldSpec::opt(8, Wire::U32);
    pub(crate) const RETAINED_BYTES: FieldSpec = FieldSpec::opt(9, Wire::U64);
    pub(crate) const FREQUENCIES_HZ: FieldSpec = FieldSpec::opt(10, Wire::PackedF32);
    pub(crate) const LEFT_DB: FieldSpec = FieldSpec::opt(11, Wire::PackedF32);
    pub(crate) const RIGHT_DB: FieldSpec = FieldSpec::opt(12, Wire::PackedF32);
    pub(crate) const SECTIONS_LEFT_DB: FieldSpec = FieldSpec::opt(13, Wire::PackedF32);
    pub(crate) const SECTIONS_RIGHT_DB: FieldSpec = FieldSpec::opt(14, Wire::PackedF32);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ResponseResult",
        fields: &[
            REQUEST_TAG,
            SAMPLE_RATE_HZ,
            CONFIGURATION_ID,
            FLOOR_DB,
            BYPASS,
            SECTION_COUNT,
            ENABLED_LEFT,
            ENABLED_RIGHT,
            RETAINED_BYTES,
            FREQUENCIES_HZ,
            LEFT_DB,
            RIGHT_DB,
            SECTIONS_LEFT_DB,
            SECTIONS_RIGHT_DB,
        ],
    };
}

/// One owner a `RESPONSE_READY` event names (#1578 D2): its section (builtins 4, console 5,
/// inserts 2), its slot ID (present exactly for console and inserts), its native effect ID, its
/// provider kind (parametric EQ 1, input filters 2, unavailable 3), its bypass, its two enabled
/// masks and the number of valid mask bits.
pub(crate) mod response_member {
    use super::*;
    pub(crate) const SECTION: FieldSpec = FieldSpec::req(1, Wire::U8);
    pub(crate) const SLOT_ID: FieldSpec = FieldSpec::opt(2, Wire::Utf8);
    pub(crate) const EFFECT: FieldSpec = FieldSpec::req(3, Wire::Utf8);
    pub(crate) const KIND: FieldSpec = FieldSpec::req(4, Wire::U8);
    pub(crate) const BYPASSED: FieldSpec = FieldSpec::req(5, Wire::Bool);
    pub(crate) const ENABLED_LEFT: FieldSpec = FieldSpec::req(6, Wire::U32);
    pub(crate) const ENABLED_RIGHT: FieldSpec = FieldSpec::req(7, Wire::U32);
    pub(crate) const SECTION_COUNT: FieldSpec = FieldSpec::req(8, Wire::U8);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ResponseMember",
        fields: &[
            SECTION,
            SLOT_ID,
            EFFECT,
            KIND,
            BYPASSED,
            ENABLED_LEFT,
            ENABLED_RIGHT,
            SECTION_COUNT,
        ],
    };
}

/// `RESPONSE_READY` (`0x8040`, #1578 D2): the result of one strip `RESPONSE_GET`, by its tag. The
/// diagnostic is present exactly when `outcome` is not `OK`; the capture's fields (6 to 9, 12, the
/// curves and the members) only when it is.
pub(crate) mod response_ready {
    use super::*;
    pub(crate) const SEQUENCE: FieldSpec = FieldSpec::req(1, Wire::U64);
    pub(crate) const REQUEST_TAG: FieldSpec = FieldSpec::req(2, Wire::U64);
    pub(crate) const STRIP_ID: FieldSpec = FieldSpec::req(3, Wire::Utf8);
    pub(crate) const OUTCOME: FieldSpec = FieldSpec::req(4, Wire::U16);
    pub(crate) const DIAGNOSTIC: FieldSpec = FieldSpec::opt(5, Wire::Utf8);
    pub(crate) const CAPTURED_SAMPLE: FieldSpec = FieldSpec::opt(6, Wire::U64);
    pub(crate) const SAMPLE_RATE_HZ: FieldSpec = FieldSpec::opt(7, Wire::U32);
    pub(crate) const FLOOR_DB: FieldSpec = FieldSpec::opt(8, Wire::F32);
    pub(crate) const FREQUENCIES_HZ: FieldSpec = FieldSpec::opt(9, Wire::PackedF32);
    pub(crate) const LEFT_DB: FieldSpec = FieldSpec::opt(10, Wire::PackedF32);
    pub(crate) const RIGHT_DB: FieldSpec = FieldSpec::opt(11, Wire::PackedF32);
    pub(crate) const EXCLUDED_COUNT: FieldSpec = FieldSpec::opt(12, Wire::U32);
    pub(crate) const MEMBER: FieldSpec = FieldSpec::msg(13, false, true, &response_member::SPEC);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ResponseReady",
        fields: &[
            SEQUENCE,
            REQUEST_TAG,
            STRIP_ID,
            OUTCOME,
            DIAGNOSTIC,
            CAPTURED_SAMPLE,
            SAMPLE_RATE_HZ,
            FLOOR_DB,
            FREQUENCIES_HZ,
            LEFT_DB,
            RIGHT_DB,
            EXCLUDED_COUNT,
            MEMBER,
        ],
    };
}

/// One selection of an `OBSERVATION_READ` request (#1586 D1): a tap by its stable address (the
/// `Observe` edit's address shape: a strip owner, a console or inserts section with its slot ID,
/// no `control` and no `lane`), its effect-local tap ID and the lanes to return.
pub(crate) mod observation_selection {
    use super::*;
    pub(crate) const ADDRESS: FieldSpec = FieldSpec::msg(1, true, false, &control_address::SPEC);
    pub(crate) const TAP_ID: FieldSpec = FieldSpec::req(2, Wire::U32);
    /// Left `1`, right `2`, both `3`.
    pub(crate) const CHANNELS: FieldSpec = FieldSpec::req(3, Wire::U8);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ObservationSelection",
        fields: &[ADDRESS, TAP_ID, CHANNELS],
    };
}

/// `OBSERVATION_READ` (`0x000f`, #1586 D1): 1 to 256 selections.
pub(crate) mod observation_read_request {
    use super::*;
    pub(crate) const SELECTION: FieldSpec =
        FieldSpec::msg(1, true, true, &observation_selection::SPEC);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ObservationReadRequest",
        fields: &[SELECTION],
    };
}

/// One row of an `OBSERVATION_READ` reply (#1586 D1): the status, the arm state the status was
/// taken from, the window exactly when ready, each lane exactly when ready and asked for, and the
/// tap's native unit.
pub(crate) mod observation_row {
    use super::*;
    /// Pending `1`, unarmed `2`, ready `3`.
    pub(crate) const STATUS: FieldSpec = FieldSpec::req(1, Wire::U8);
    pub(crate) const ARMED: FieldSpec = FieldSpec::req(2, Wire::Bool);
    pub(crate) const ARM_SAMPLE: FieldSpec = FieldSpec::req(3, Wire::U64);
    pub(crate) const ARM_TAG: FieldSpec = FieldSpec::req(4, Wire::U64);
    pub(crate) const FIRST_SAMPLE: FieldSpec = FieldSpec::opt(5, Wire::U64);
    pub(crate) const END_SAMPLE: FieldSpec = FieldSpec::opt(6, Wire::U64);
    pub(crate) const SEQUENCE: FieldSpec = FieldSpec::opt(7, Wire::U64);
    pub(crate) const BLOCKS: FieldSpec = FieldSpec::opt(8, Wire::U32);
    pub(crate) const LEFT: FieldSpec = FieldSpec::opt(9, Wire::F32);
    pub(crate) const RIGHT: FieldSpec = FieldSpec::opt(10, Wire::F32);
    /// The tap's native `ParameterUnit` code.
    pub(crate) const UNIT: FieldSpec = FieldSpec::req(11, Wire::U8);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ObservationRow",
        fields: &[
            STATUS,
            ARMED,
            ARM_SAMPLE,
            ARM_TAG,
            FIRST_SAMPLE,
            END_SAMPLE,
            SEQUENCE,
            BLOCKS,
            LEFT,
            RIGHT,
            UNIT,
        ],
    };
}

/// The `OBSERVATION_READ` reply (#1586 D1): the render sample at the read and one row per
/// selection, in selection order.
pub(crate) mod observation_read_page {
    use super::*;
    pub(crate) const OBSERVED_SAMPLE: FieldSpec = FieldSpec::req(1, Wire::U64);
    pub(crate) const ROW: FieldSpec = FieldSpec::msg(2, true, true, &observation_row::SPEC);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ObservationReadPage",
        fields: &[OBSERVED_SAMPLE, ROW],
    };
}

/// One tap of a `SPECTRUM_CONFIGURE` request (#1594 D2).
pub(crate) mod spectrum_entry {
    use super::*;
    /// Track post-input `1`, track post-pan `2`, output `3`.
    pub(crate) const TARGET_KIND: FieldSpec = FieldSpec::req(1, Wire::U8);
    pub(crate) const TARGET_ID: FieldSpec = FieldSpec::req(2, Wire::Utf8);
    /// Left `1`, right `2`, both `3`.
    pub(crate) const CHANNELS: FieldSpec = FieldSpec::req(3, Wire::U8);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "SpectrumEntry",
        fields: &[TARGET_KIND, TARGET_ID, CHANNELS],
    };
}

/// `SPECTRUM_CONFIGURE` (`0x0010`, #1594 D2): the form, the taps, the budget and the hop.
pub(crate) mod spectrum_configure_request {
    use super::*;
    /// Single `1`, collection `2`.
    pub(crate) const FORM: FieldSpec = FieldSpec::req(1, Wire::U8);
    pub(crate) const ENTRY: FieldSpec = FieldSpec::msg(2, false, true, &spectrum_entry::SPEC);
    pub(crate) const MAXIMUM_CAPTURE_BYTES: FieldSpec = FieldSpec::req(3, Wire::U64);
    pub(crate) const HOP_FRAMES: FieldSpec = FieldSpec::opt(4, Wire::U32);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "SpectrumConfigureRequest",
        fields: &[FORM, ENTRY, MAXIMUM_CAPTURE_BYTES, HOP_FRAMES],
    };
}

/// The `SPECTRUM_CONFIGURE` reply (#1594 D3).
pub(crate) mod spectrum_configured {
    use super::*;
    pub(crate) const REVISION: FieldSpec = FieldSpec::req(1, Wire::U64);
    pub(crate) const CONFIGURED_EPOCH: FieldSpec = FieldSpec::req(2, Wire::U64);
    pub(crate) const ADOPTED_EPOCH: FieldSpec = FieldSpec::req(3, Wire::U64);
    pub(crate) const REBUILD: FieldSpec = FieldSpec::req(4, Wire::Bool);
    pub(crate) const PLAN_EPOCH: FieldSpec = FieldSpec::opt(5, Wire::U64);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "SpectrumConfigured",
        fields: &[
            REVISION,
            CONFIGURED_EPOCH,
            ADOPTED_EPOCH,
            REBUILD,
            PLAN_EPOCH,
        ],
    };
}

/// `SPECTRUM_CAPTURE` (`0x0011`, #1595 D2): one operation on the one-shot window.
pub(crate) mod spectrum_capture_request {
    use super::*;
    /// Arm `1`, read `2`, cancel `3`, select `4`.
    pub(crate) const OP: FieldSpec = FieldSpec::req(1, Wire::U8);
    /// Read and select only: left `1`, right `2`, both `3`.
    pub(crate) const CHANNELS: FieldSpec = FieldSpec::opt(2, Wire::U8);
    /// Select only: #1594 D2's target kind codes.
    pub(crate) const TARGET_KIND: FieldSpec = FieldSpec::opt(3, Wire::U8);
    /// Select only.
    pub(crate) const TARGET_ID: FieldSpec = FieldSpec::opt(4, Wire::Utf8);
    /// Read only; absent is false.
    pub(crate) const INCLUDE_WINDOW: FieldSpec = FieldSpec::opt(5, Wire::Bool);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "SpectrumCaptureRequest",
        fields: &[OP, CHANNELS, TARGET_KIND, TARGET_ID, INCLUDE_WINDOW],
    };
}

/// The `SPECTRUM_CAPTURE` reply (#1595 D2): the epochs and the status, and with `ready` the
/// analyzed window.
pub(crate) mod spectrum_capture_reply {
    use super::*;
    pub(crate) const ADOPTED_EPOCH: FieldSpec = FieldSpec::req(1, Wire::U64);
    pub(crate) const SELECTION_EPOCH: FieldSpec = FieldSpec::req(2, Wire::U64);
    /// Done `1`, pending `2`, ready `3`.
    pub(crate) const STATUS: FieldSpec = FieldSpec::req(3, Wire::U8);
    pub(crate) const TARGET_KIND: FieldSpec = FieldSpec::opt(4, Wire::U8);
    pub(crate) const TARGET_ID: FieldSpec = FieldSpec::opt(5, Wire::Utf8);
    pub(crate) const CHANNELS: FieldSpec = FieldSpec::opt(6, Wire::U8);
    pub(crate) const SAMPLE_RATE_HZ: FieldSpec = FieldSpec::opt(7, Wire::U32);
    pub(crate) const WINDOW_FRAMES: FieldSpec = FieldSpec::opt(8, Wire::U32);
    pub(crate) const BIN_COUNT: FieldSpec = FieldSpec::opt(9, Wire::U32);
    pub(crate) const FLOOR_DB: FieldSpec = FieldSpec::opt(10, Wire::F32);
    pub(crate) const CAPTURED_SAMPLE: FieldSpec = FieldSpec::opt(11, Wire::U64);
    pub(crate) const END_SAMPLE: FieldSpec = FieldSpec::opt(12, Wire::U64);
    pub(crate) const SNAPSHOT_TOKEN: FieldSpec = FieldSpec::opt(13, Wire::U64);
    pub(crate) const SOURCE_UNDERRUN: FieldSpec = FieldSpec::opt(14, Wire::Bool);
    pub(crate) const FREQUENCIES_HZ: FieldSpec = FieldSpec::opt(15, Wire::PackedF32);
    pub(crate) const LEFT_DB: FieldSpec = FieldSpec::opt(16, Wire::PackedF32);
    pub(crate) const RIGHT_DB: FieldSpec = FieldSpec::opt(17, Wire::PackedF32);
    pub(crate) const LEFT_WINDOW: FieldSpec = FieldSpec::opt(18, Wire::PackedF32);
    pub(crate) const RIGHT_WINDOW: FieldSpec = FieldSpec::opt(19, Wire::PackedF32);
    /// Every field a ready reply adds, in tag order.
    pub(crate) const READY: [FieldSpec; 16] = [
        TARGET_KIND,
        TARGET_ID,
        CHANNELS,
        SAMPLE_RATE_HZ,
        WINDOW_FRAMES,
        BIN_COUNT,
        FLOOR_DB,
        CAPTURED_SAMPLE,
        END_SAMPLE,
        SNAPSHOT_TOKEN,
        SOURCE_UNDERRUN,
        FREQUENCIES_HZ,
        LEFT_DB,
        RIGHT_DB,
        LEFT_WINDOW,
        RIGHT_WINDOW,
    ];
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "SpectrumCaptureReply",
        fields: &[
            ADOPTED_EPOCH,
            SELECTION_EPOCH,
            STATUS,
            TARGET_KIND,
            TARGET_ID,
            CHANNELS,
            SAMPLE_RATE_HZ,
            WINDOW_FRAMES,
            BIN_COUNT,
            FLOOR_DB,
            CAPTURED_SAMPLE,
            END_SAMPLE,
            SNAPSHOT_TOKEN,
            SOURCE_UNDERRUN,
            FREQUENCIES_HZ,
            LEFT_DB,
            RIGHT_DB,
            LEFT_WINDOW,
            RIGHT_WINDOW,
        ],
    };
}

/// `SPECTRUM_STREAM` (`0x0012`, #1595 D3): one operation on the continuous stream.
pub(crate) mod spectrum_stream_request {
    use super::*;
    /// Start `1`, select `2`, read `3`, reset `4`, stop `5`.
    pub(crate) const OP: FieldSpec = FieldSpec::req(1, Wire::U8);
    /// Start (required) and select (optional; absent keeps the smoothing) only.
    pub(crate) const SMOOTHING_MS: FieldSpec = FieldSpec::opt(2, Wire::F64);
    /// Select only: #1594 D2's target kind codes.
    pub(crate) const TARGET_KIND: FieldSpec = FieldSpec::opt(3, Wire::U8);
    /// Select only.
    pub(crate) const TARGET_ID: FieldSpec = FieldSpec::opt(4, Wire::Utf8);
    /// Select only: left `1`, right `2`, both `3`.
    pub(crate) const CHANNELS: FieldSpec = FieldSpec::opt(5, Wire::U8);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "SpectrumStreamRequest",
        fields: &[OP, SMOOTHING_MS, TARGET_KIND, TARGET_ID, CHANNELS],
    };
}

/// The `SPECTRUM_STREAM` reply (#1595 D3): the epochs, the stream's metadata and, with `ready`,
/// the smoothed bins.
pub(crate) mod spectrum_stream_reply {
    use super::*;
    pub(crate) const ADOPTED_EPOCH: FieldSpec = FieldSpec::req(1, Wire::U64);
    pub(crate) const SELECTION_EPOCH: FieldSpec = FieldSpec::req(2, Wire::U64);
    /// #1592 D1's codes: inactive `0` through ready `6`.
    pub(crate) const STATUS: FieldSpec = FieldSpec::req(3, Wire::U8);
    /// `0` without a selection.
    pub(crate) const TARGET_KIND: FieldSpec = FieldSpec::req(4, Wire::U8);
    /// Present exactly with a selection.
    pub(crate) const TARGET_ID: FieldSpec = FieldSpec::opt(5, Wire::Utf8);
    /// `0` without a selection.
    pub(crate) const CHANNELS: FieldSpec = FieldSpec::req(6, Wire::U8);
    pub(crate) const SAMPLE_RATE_HZ: FieldSpec = FieldSpec::req(7, Wire::U32);
    pub(crate) const QUANTUM_FRAMES: FieldSpec = FieldSpec::req(8, Wire::U32);
    pub(crate) const HOP_FRAMES: FieldSpec = FieldSpec::req(9, Wire::U32);
    pub(crate) const CAPTURE_EPOCH: FieldSpec = FieldSpec::req(10, Wire::U64);
    pub(crate) const SEQUENCE: FieldSpec = FieldSpec::req(11, Wire::U64);
    pub(crate) const DROPPED_CAPTURES: FieldSpec = FieldSpec::req(12, Wire::U64);
    pub(crate) const WINDOWS: FieldSpec = FieldSpec::req(13, Wire::U64);
    pub(crate) const CAPTURED_SAMPLE: FieldSpec = FieldSpec::req(14, Wire::U64);
    pub(crate) const END_SAMPLE: FieldSpec = FieldSpec::req(15, Wire::U64);
    pub(crate) const ANALYSIS_EPOCH: FieldSpec = FieldSpec::req(16, Wire::U64);
    pub(crate) const HISTORY_START_SAMPLE: FieldSpec = FieldSpec::req(17, Wire::U64);
    pub(crate) const SMOOTHING_MS: FieldSpec = FieldSpec::req(18, Wire::F64);
    pub(crate) const SOURCE_UNDERRUN: FieldSpec = FieldSpec::req(19, Wire::Bool);
    /// The ready reply's planes, as `SPECTRUM_CAPTURE`'s.
    pub(crate) const FREQUENCIES_HZ: FieldSpec = FieldSpec::opt(20, Wire::PackedF32);
    pub(crate) const LEFT_DB: FieldSpec = FieldSpec::opt(21, Wire::PackedF32);
    pub(crate) const RIGHT_DB: FieldSpec = FieldSpec::opt(22, Wire::PackedF32);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "SpectrumStreamReply",
        fields: &[
            ADOPTED_EPOCH,
            SELECTION_EPOCH,
            STATUS,
            TARGET_KIND,
            TARGET_ID,
            CHANNELS,
            SAMPLE_RATE_HZ,
            QUANTUM_FRAMES,
            HOP_FRAMES,
            CAPTURE_EPOCH,
            SEQUENCE,
            DROPPED_CAPTURES,
            WINDOWS,
            CAPTURED_SAMPLE,
            END_SAMPLE,
            ANALYSIS_EPOCH,
            HISTORY_START_SAMPLE,
            SMOOTHING_MS,
            SOURCE_UNDERRUN,
            FREQUENCIES_HZ,
            LEFT_DB,
            RIGHT_DB,
        ],
    };
}

/// `LOUDNESS` (`0x0014`, #2115): one operation on the session's output loudness meter.
pub(crate) mod loudness_request {
    use super::*;
    /// Start `1`, read `2`, reset `3`, stop `4`.
    pub(crate) const OP: FieldSpec = FieldSpec::req(1, Wire::U8);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "LoudnessRequest",
        fields: &[OP],
    };
}

/// The `LOUDNESS` reply (#2115): the meter's state after the operation. Every level is optional,
/// absent until it is defined, and finite at or above the -120 floor.
pub(crate) mod loudness_reply {
    use super::*;
    pub(crate) const OBSERVED_SAMPLE: FieldSpec = FieldSpec::req(1, Wire::U64);
    pub(crate) const ARMED: FieldSpec = FieldSpec::req(2, Wire::Bool);
    pub(crate) const INTEGRATING_SINCE: FieldSpec = FieldSpec::opt(3, Wire::U64);
    pub(crate) const HOPS_DROPPED: FieldSpec = FieldSpec::req(4, Wire::U64);
    pub(crate) const SANITIZED_SAMPLES: FieldSpec = FieldSpec::req(5, Wire::U64);
    pub(crate) const GATED_BLOCK_COUNT: FieldSpec = FieldSpec::req(6, Wire::U64);
    pub(crate) const MOMENTARY_LUFS: FieldSpec = FieldSpec::opt(7, Wire::F32);
    pub(crate) const SHORT_TERM_LUFS: FieldSpec = FieldSpec::opt(8, Wire::F32);
    pub(crate) const INTEGRATED_LUFS: FieldSpec = FieldSpec::opt(9, Wire::F32);
    pub(crate) const LOUDNESS_RANGE_LU: FieldSpec = FieldSpec::opt(10, Wire::F32);
    pub(crate) const MOMENTARY_MAX_LUFS: FieldSpec = FieldSpec::opt(11, Wire::F32);
    pub(crate) const SHORT_TERM_MAX_LUFS: FieldSpec = FieldSpec::opt(12, Wire::F32);
    pub(crate) const TRUE_PEAK_LEFT_DBTP: FieldSpec = FieldSpec::opt(13, Wire::F32);
    pub(crate) const TRUE_PEAK_RIGHT_DBTP: FieldSpec = FieldSpec::opt(14, Wire::F32);
    pub(crate) const HOP_TRUE_PEAK_LEFT_DBTP: FieldSpec = FieldSpec::opt(15, Wire::F32);
    pub(crate) const HOP_TRUE_PEAK_RIGHT_DBTP: FieldSpec = FieldSpec::opt(16, Wire::F32);
    /// The optional levels, in tag order.
    pub(crate) const LEVELS: [FieldSpec; 10] = [
        MOMENTARY_LUFS,
        SHORT_TERM_LUFS,
        INTEGRATED_LUFS,
        LOUDNESS_RANGE_LU,
        MOMENTARY_MAX_LUFS,
        SHORT_TERM_MAX_LUFS,
        TRUE_PEAK_LEFT_DBTP,
        TRUE_PEAK_RIGHT_DBTP,
        HOP_TRUE_PEAK_LEFT_DBTP,
        HOP_TRUE_PEAK_RIGHT_DBTP,
    ];
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "LoudnessReply",
        fields: &[
            OBSERVED_SAMPLE,
            ARMED,
            INTEGRATING_SINCE,
            HOPS_DROPPED,
            SANITIZED_SAMPLES,
            GATED_BLOCK_COUNT,
            MOMENTARY_LUFS,
            SHORT_TERM_LUFS,
            INTEGRATED_LUFS,
            LOUDNESS_RANGE_LU,
            MOMENTARY_MAX_LUFS,
            SHORT_TERM_MAX_LUFS,
            TRUE_PEAK_LEFT_DBTP,
            TRUE_PEAK_RIGHT_DBTP,
            HOP_TRUE_PEAK_LEFT_DBTP,
            HOP_TRUE_PEAK_RIGHT_DBTP,
        ],
    };
}

pub(crate) mod transport_get {
    use super::MessageSpec;
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "TransportGet",
        fields: &[],
    };
}

pub(crate) mod transport_set {
    use super::*;
    pub(crate) const STATE: FieldSpec = FieldSpec::req(1, Wire::U8);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "TransportSet",
        fields: &[STATE],
    };
    /// The retired position (#1635): field 2 is refused in either flag form, never skipped as an
    /// unknown optional field and never reallocated.
    pub(crate) static RETIRED: MessageSpec = MessageSpec {
        name: "TransportSetRetiredFields",
        fields: &[FieldSpec::opt(2, Wire::U64)],
    };
}

pub(crate) mod transport_snapshot {
    use super::*;
    pub(crate) const STATE: FieldSpec = FieldSpec::req(1, Wire::U8);
    pub(crate) const EFFECTIVE_SAMPLE: FieldSpec = FieldSpec::req(3, Wire::U64);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "TransportSnapshot",
        fields: &[STATE, EFFECTIVE_SAMPLE],
    };
    /// The retired position (#1635): field 2.
    pub(crate) static RETIRED: MessageSpec = MessageSpec {
        name: "TransportSnapshotRetiredFields",
        fields: &[FieldSpec::opt(2, Wire::U64)],
    };
}

pub(crate) mod transport_state_event {
    use super::*;
    pub(crate) const EVENT_SEQUENCE: FieldSpec = FieldSpec::req(1, Wire::U64);
    pub(crate) const STATE: FieldSpec = FieldSpec::req(2, Wire::U8);
    pub(crate) const EFFECTIVE_SAMPLE: FieldSpec = FieldSpec::req(4, Wire::U64);
    pub(crate) const ORIGIN_REQUEST_ID: FieldSpec = FieldSpec::opt(5, Wire::U64);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "TransportStateEvent",
        fields: &[EVENT_SEQUENCE, STATE, EFFECTIVE_SAMPLE, ORIGIN_REQUEST_ID],
    };
    /// The retired position (#1635): field 3.
    pub(crate) static RETIRED: MessageSpec = MessageSpec {
        name: "TransportStateEventRetiredFields",
        fields: &[FieldSpec::opt(3, Wire::U64)],
    };
}

/// One ended tap's stable key inside `OBSERVATION_ENDED` (#1584 D6a fold).
pub(crate) mod observation_tap_key {
    use super::*;
    pub(crate) const STRIP_ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
    /// The section: inserts `2` or console `5`, the `RackName` codes.
    pub(crate) const SECTION: FieldSpec = FieldSpec::req(2, Wire::U8);
    /// The insert's effect ID or the console slot ID.
    pub(crate) const EFFECT_ID: FieldSpec = FieldSpec::req(3, Wire::Utf8);
    pub(crate) const TAP_ID: FieldSpec = FieldSpec::req(4, Wire::U32);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ObservationTapKey",
        fields: &[STRIP_ID, SECTION, EFFECT_ID, TAP_ID],
    };
}

/// `OBSERVATION_ENDED` (#1584): the stable keys of every tap whose subscription one commit ended,
/// one repeated `key` message each, at least one, in strictly ascending key order. The revision
/// that removed the taps' effects is the event header's.
pub(crate) mod observation_ended {
    use super::*;
    pub(crate) const EVENT_SEQUENCE: FieldSpec = FieldSpec::req(1, Wire::U64);
    pub(crate) const KEY: FieldSpec = FieldSpec::msg(2, true, true, &observation_tap_key::SPEC);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ObservationEnded",
        fields: &[EVENT_SEQUENCE, KEY],
    };
}

pub(crate) mod meter_batch {
    use super::*;
    pub(crate) const OBSERVED_SAMPLE: FieldSpec = FieldSpec::req(1, Wire::U64);
    pub(crate) const COUNT: FieldSpec = FieldSpec::req(2, Wire::U16);
    pub(crate) const RECORD_BYTES: FieldSpec = FieldSpec::req(3, Wire::U16);
    pub(crate) const RECORDS: FieldSpec = FieldSpec::req(4, Wire::Bytes);
    /// The first sample of the closed meter window; `observed_sample` is its exclusive end
    /// (#1583 D2).
    pub(crate) const FIRST_SAMPLE: FieldSpec = FieldSpec::req(5, Wire::U64);
    /// The window's sequence since the accepted configuration, from `1` (#1583 D2).
    pub(crate) const WINDOW_SEQUENCE: FieldSpec = FieldSpec::req(6, Wire::U64);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "MeterBatch",
        fields: &[
            OBSERVED_SAMPLE,
            COUNT,
            RECORD_BYTES,
            RECORDS,
            FIRST_SAMPLE,
            WINDOW_SEQUENCE,
        ],
    };
}

/// One meter table entry of `TELEMETRY_CONFIGURE` (#1583 D1): its 1-based position is its handle.
pub(crate) mod meter_entry {
    use super::*;
    /// Output `1`, strip `2`, master `3` (#1690 D3).
    pub(crate) const TARGET: FieldSpec = FieldSpec::req(1, Wire::U8);
    /// The strip's stable ID: present exactly when `target` is a strip.
    pub(crate) const STRIP_ID: FieldSpec = FieldSpec::opt(2, Wire::Utf8);
    /// Peak `1`, gain reduction `2`.
    pub(crate) const METRIC: FieldSpec = FieldSpec::req(3, Wire::U8);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "MeterEntry",
        fields: &[TARGET, STRIP_ID, METRIC],
    };
}

pub(crate) mod telemetry_configuration {
    use super::*;
    // Field 1 (`meter_handles`) is retired by #1583 D2 and never reallocated; see `RETIRED`.
    pub(crate) const METER_PERIOD_BLOCKS: FieldSpec = FieldSpec::req(2, Wire::U32);
    pub(crate) const COUNTER_IDS: FieldSpec = FieldSpec::req(3, Wire::PackedU32);
    pub(crate) const COUNTER_PERIOD_BLOCKS: FieldSpec = FieldSpec::req(4, Wire::U32);
    pub(crate) const DIAGNOSTICS_ENABLED: FieldSpec = FieldSpec::req(5, Wire::Bool);
    pub(crate) const MINIMUM_DIAGNOSTIC_SEVERITY: FieldSpec = FieldSpec::req(6, Wire::U8);
    /// The meter table, 0 to 256 entries, in handle order (#1583 D1).
    pub(crate) const METER: FieldSpec = FieldSpec::msg(7, false, true, &meter_entry::SPEC);
    /// The meter window in blocks: refused in a command, present in a success echo exactly when
    /// the provider has a meter delivery (#1583 D4).
    pub(crate) const METER_WINDOW_BLOCKS: FieldSpec = FieldSpec::opt(8, Wire::U32);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "TelemetryConfiguration",
        fields: &[
            METER_PERIOD_BLOCKS,
            COUNTER_IDS,
            COUNTER_PERIOD_BLOCKS,
            DIAGNOSTICS_ENABLED,
            MINIMUM_DIAGNOSTIC_SEVERITY,
            METER,
            METER_WINDOW_BLOCKS,
        ],
    };
    /// The retired position (#1583 D2): field 1, the positional meter handles, is refused in
    /// either flag form, never skipped as an unknown optional field and never reallocated.
    pub(crate) static RETIRED: MessageSpec = MessageSpec {
        name: "TelemetryConfigurationRetiredFields",
        fields: &[FieldSpec::opt(1, Wire::PackedU32)],
    };
}

pub(crate) mod counters_request {
    use super::*;
    pub(crate) const ALL: FieldSpec = FieldSpec::req(1, Wire::Bool);
    pub(crate) const IDS: FieldSpec = FieldSpec::opt(2, Wire::PackedU32);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "CountersRequest",
        fields: &[ALL, IDS],
    };
}

pub(crate) mod counter_value {
    use super::*;
    pub(crate) const ID: FieldSpec = FieldSpec::req(1, Wire::U32);
    pub(crate) const VALUE: FieldSpec = FieldSpec::req(2, Wire::U64);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "CounterValue",
        fields: &[ID, VALUE],
    };
}

pub(crate) mod counter_snapshot {
    use super::{counter_value, *};
    pub(crate) const OBSERVED_SAMPLE: FieldSpec = FieldSpec::req(1, Wire::U64);
    pub(crate) const VALUE: FieldSpec = FieldSpec::msg(2, true, true, &counter_value::SPEC);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "CounterSnapshot",
        fields: &[OBSERVED_SAMPLE, VALUE],
    };
}

pub(crate) mod diagnostics_request {
    use super::*;
    pub(crate) const AFTER_SEQUENCE: FieldSpec = FieldSpec::req(1, Wire::U64);
    pub(crate) const LIMIT: FieldSpec = FieldSpec::req(2, Wire::U16);
    pub(crate) const MINIMUM_SEVERITY: FieldSpec = FieldSpec::req(3, Wire::U8);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "DiagnosticsRequest",
        fields: &[AFTER_SEQUENCE, LIMIT, MINIMUM_SEVERITY],
    };
}

pub(crate) mod path_segment {
    use super::*;
    pub(crate) const TAG: FieldSpec = FieldSpec::req(1, Wire::U8);
    pub(crate) const FIELD: FieldSpec = FieldSpec::opt(2, Wire::Utf8);
    pub(crate) const INDEX: FieldSpec = FieldSpec::opt(3, Wire::U64);
    pub(crate) const STABLE_ID: FieldSpec = FieldSpec::opt(4, Wire::Utf8);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "PathSegment",
        fields: &[TAG, FIELD, INDEX, STABLE_ID],
    };
}

pub(crate) mod diagnostic {
    use super::{path_segment, *};
    pub(crate) const CODE: FieldSpec = FieldSpec::req(1, Wire::Utf8);
    pub(crate) const SEVERITY: FieldSpec = FieldSpec::req(2, Wire::U8);
    pub(crate) const PATH: FieldSpec = FieldSpec::msg(3, true, true, &path_segment::SPEC);
    pub(crate) const DETAIL: FieldSpec = FieldSpec::opt(4, Wire::Utf8);
    pub(crate) const OPERATION_INDEX: FieldSpec = FieldSpec::opt(5, Wire::U32);
    pub(crate) const SAMPLE_TIME: FieldSpec = FieldSpec::opt(6, Wire::U64);
    pub(crate) const PROVIDER_SEQUENCE: FieldSpec = FieldSpec::opt(7, Wire::U64);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "Diagnostic",
        fields: &[
            CODE,
            SEVERITY,
            PATH,
            DETAIL,
            OPERATION_INDEX,
            SAMPLE_TIME,
            PROVIDER_SEQUENCE,
        ],
    };
}

pub(crate) mod backpressure {
    use super::*;
    pub(crate) const QUEUE_KIND: FieldSpec = FieldSpec::req(1, Wire::U8);
    pub(crate) const CAPACITY: FieldSpec = FieldSpec::req(2, Wire::U64);
    pub(crate) const OCCUPANCY: FieldSpec = FieldSpec::req(3, Wire::U64);
    pub(crate) const REQUESTED_ITEMS: FieldSpec = FieldSpec::req(4, Wire::U16);
    pub(crate) const GENERATION: FieldSpec = FieldSpec::opt(5, Wire::U64);
    pub(crate) const RETRY_BOUNDARY: FieldSpec = FieldSpec::opt(6, Wire::U64);
    pub(crate) const REQUESTED_BYTES: FieldSpec = FieldSpec::opt(7, Wire::U64);
    pub(crate) const AVAILABLE_BYTES: FieldSpec = FieldSpec::opt(8, Wire::U64);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "Backpressure",
        fields: &[
            QUEUE_KIND,
            CAPACITY,
            OCCUPANCY,
            REQUESTED_ITEMS,
            GENERATION,
            RETRY_BOUNDARY,
            REQUESTED_BYTES,
            AVAILABLE_BYTES,
        ],
    };
}

pub(crate) mod non_ok {
    use super::{backpressure, diagnostic, *};
    pub(crate) const DIAGNOSTIC: FieldSpec = FieldSpec::msg(1, true, true, &diagnostic::SPEC);
    pub(crate) const OMITTED_DIAGNOSTICS: FieldSpec = FieldSpec::req(2, Wire::U32);
    pub(crate) const BACKPRESSURE: FieldSpec = FieldSpec::msg(3, false, false, &backpressure::SPEC);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "NonOkResponse",
        fields: &[DIAGNOSTIC, OMITTED_DIAGNOSTICS, BACKPRESSURE],
    };
}

pub(crate) mod diagnostics_page {
    use super::{diagnostic, *};
    pub(crate) const LAST_SEQUENCE: FieldSpec = FieldSpec::req(1, Wire::U64);
    pub(crate) const EOF: FieldSpec = FieldSpec::req(2, Wire::Bool);
    pub(crate) const DIAGNOSTIC: FieldSpec = FieldSpec::msg(3, true, true, &diagnostic::SPEC);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "DiagnosticsPage",
        fields: &[LAST_SEQUENCE, EOF, DIAGNOSTIC],
    };
}

pub(crate) mod diagnostic_event {
    use super::{diagnostic, *};
    pub(crate) const DIAGNOSTIC: FieldSpec = FieldSpec::msg(1, true, false, &diagnostic::SPEC);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "DiagnosticEvent",
        fields: &[DIAGNOSTIC],
    };
}

pub(crate) mod capabilities {
    use super::*;
    pub(crate) const MINIMUM_VERSION_MAJOR: FieldSpec = FieldSpec::req(1, Wire::U16);
    pub(crate) const MINIMUM_VERSION_MINOR: FieldSpec = FieldSpec::req(2, Wire::U16);
    pub(crate) const MAXIMUM_VERSION_MAJOR: FieldSpec = FieldSpec::req(3, Wire::U16);
    pub(crate) const MAXIMUM_VERSION_MINOR: FieldSpec = FieldSpec::req(4, Wire::U16);
    pub(crate) const MAXIMUM_FRAME_BYTES: FieldSpec = FieldSpec::req(5, Wire::U64);
    pub(crate) const MAXIMUM_TLVS: FieldSpec = FieldSpec::req(6, Wire::U32);
    pub(crate) const MAXIMUM_STRING_BYTES: FieldSpec = FieldSpec::req(7, Wire::U64);
    pub(crate) const MAXIMUM_NESTING: FieldSpec = FieldSpec::req(8, Wire::U8);
    // Fields 9 (`maximum_automation_records`), 12 (`automation_batch_slots`), 19
    // (`per_block_automation_density`) and 20 (`admission_quantum_frames`) are retired with the
    // `AUTOMATION_ENQUEUE` command (#1627) and never reallocated; see `RETIRED`.
    pub(crate) const CONTROL_COMMAND_SLOTS: FieldSpec = FieldSpec::req(10, Wire::U64);
    pub(crate) const CONTROL_COMMAND_BYTES: FieldSpec = FieldSpec::req(11, Wire::U64);
    pub(crate) const RELIABLE_RESPONSE_SLOTS: FieldSpec = FieldSpec::req(13, Wire::U64);
    pub(crate) const RELIABLE_EVENT_SLOTS: FieldSpec = FieldSpec::req(14, Wire::U64);
    pub(crate) const TELEMETRY_SLOTS: FieldSpec = FieldSpec::req(15, Wire::U64);
    pub(crate) const REPLAY_ENTRIES: FieldSpec = FieldSpec::req(16, Wire::U64);
    pub(crate) const REPLAY_BYTES: FieldSpec = FieldSpec::req(17, Wire::U64);
    pub(crate) const MAXIMUM_CACHED_RESPONSE_BYTES: FieldSpec = FieldSpec::req(18, Wire::U64);
    pub(crate) const MAXIMUM_CONTROL_PAGE_ITEMS: FieldSpec = FieldSpec::req(21, Wire::U16);
    pub(crate) const MAXIMUM_DIAGNOSTIC_PAGE_ITEMS: FieldSpec = FieldSpec::req(22, Wire::U16);
    pub(crate) const MAXIMUM_TELEMETRY_HANDLES: FieldSpec = FieldSpec::req(23, Wire::U16);
    pub(crate) const MAXIMUM_TRANSACTION_EDITS: FieldSpec = FieldSpec::req(24, Wire::U32);
    pub(crate) const SUPPORTED_COMMANDS: FieldSpec = FieldSpec::req(25, Wire::PackedU16);
    pub(crate) const SUPPORTED_EVENTS: FieldSpec = FieldSpec::req(26, Wire::PackedU16);
    pub(crate) const FLAGS: FieldSpec = FieldSpec::req(27, Wire::U64);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "Capabilities",
        fields: &[
            MINIMUM_VERSION_MAJOR,
            MINIMUM_VERSION_MINOR,
            MAXIMUM_VERSION_MAJOR,
            MAXIMUM_VERSION_MINOR,
            MAXIMUM_FRAME_BYTES,
            MAXIMUM_TLVS,
            MAXIMUM_STRING_BYTES,
            MAXIMUM_NESTING,
            CONTROL_COMMAND_SLOTS,
            CONTROL_COMMAND_BYTES,
            RELIABLE_RESPONSE_SLOTS,
            RELIABLE_EVENT_SLOTS,
            TELEMETRY_SLOTS,
            REPLAY_ENTRIES,
            REPLAY_BYTES,
            MAXIMUM_CACHED_RESPONSE_BYTES,
            MAXIMUM_CONTROL_PAGE_ITEMS,
            MAXIMUM_DIAGNOSTIC_PAGE_ITEMS,
            MAXIMUM_TELEMETRY_HANDLES,
            MAXIMUM_TRANSACTION_EDITS,
            SUPPORTED_COMMANDS,
            SUPPORTED_EVENTS,
            FLAGS,
        ],
    };
    /// The retired capability fields (#1627): a peer that sends one is refused in either flag
    /// form, never skipped as an unknown optional field.
    pub(crate) static RETIRED: MessageSpec = MessageSpec {
        name: "CapabilitiesRetiredFields",
        fields: &[
            FieldSpec::opt(9, Wire::U16),
            FieldSpec::opt(12, Wire::U64),
            FieldSpec::opt(19, Wire::U64),
            FieldSpec::opt(20, Wire::U64),
        ],
    };
}

pub(crate) mod session {
    use super::{FieldSpec, MessageSpec, Wire};

    pub(crate) mod edit_payload {
        use super::*;
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "SessionEditPayload",
            fields: &[],
        };
    }
    pub(crate) mod edit {
        use super::*;
        pub(crate) const OPCODE: FieldSpec = FieldSpec::req(1, Wire::U16);
        pub(crate) const PAYLOAD: FieldSpec = FieldSpec::msg(2, true, false, &edit_payload::SPEC);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "SessionEdit",
            fields: &[OPCODE, PAYLOAD],
        };
    }
    pub(crate) mod transaction {
        use super::*;
        pub(crate) const EDIT: FieldSpec = FieldSpec::msg(1, true, true, &edit::SPEC);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "SessionTransaction",
            fields: &[EDIT],
        };
    }
    pub(crate) mod render_profile {
        use super::*;
        pub(crate) const ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const MODE: FieldSpec = FieldSpec::req(2, Wire::U8);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "RenderProfile",
            fields: &[ID, MODE],
        };
    }
    pub(crate) mod output_profile {
        use super::*;
        pub(crate) const ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const CHANNELS: FieldSpec = FieldSpec::req(2, Wire::U8);
        pub(crate) const LAYOUT: FieldSpec = FieldSpec::req(3, Wire::U8);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "OutputProfile",
            fields: &[ID, CHANNELS, LAYOUT],
        };
    }
    pub(crate) mod source {
        use super::*;
        pub(crate) const ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const CONTENT: FieldSpec = FieldSpec::req(2, Wire::Utf8);
        pub(crate) const CHANNELS: FieldSpec = FieldSpec::req(3, Wire::U8);
        pub(crate) const BIT_DEPTH: FieldSpec = FieldSpec::req(4, Wire::U8);
        pub(crate) const FRAMES: FieldSpec = FieldSpec::req(5, Wire::U64);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "Source",
            fields: &[ID, CONTENT, CHANNELS, BIT_DEPTH, FRAMES],
        };
    }
    pub(crate) mod channel_builtins {
        use super::*;
        pub(crate) const POLARITY_INVERT: FieldSpec = FieldSpec::req(1, Wire::Bool);
        pub(crate) const TRIM_DB: FieldSpec = FieldSpec::req(2, Wire::F32);
        pub(crate) const HPF_HZ: FieldSpec = FieldSpec::req(3, Wire::F32);
        pub(crate) const LPF_HZ: FieldSpec = FieldSpec::req(4, Wire::F32);
        pub(crate) const DELAY_SAMPLES: FieldSpec = FieldSpec::req(5, Wire::U32);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "ChannelBuiltins",
            fields: &[POLARITY_INVERT, TRIM_DB, HPF_HZ, LPF_HZ, DELAY_SAMPLES],
        };
    }
    pub(crate) mod builtins {
        use super::*;
        pub(crate) const LEFT: FieldSpec = FieldSpec::msg(1, true, false, &channel_builtins::SPEC);
        pub(crate) const RIGHT: FieldSpec = FieldSpec::msg(2, true, false, &channel_builtins::SPEC);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "DualMonoBuiltins",
            fields: &[LEFT, RIGHT],
        };
    }

    pub(crate) mod effect_identity {
        use super::*;
        pub(crate) const TAG: FieldSpec = FieldSpec::req(1, Wire::U8);
        pub(crate) const VALUE: FieldSpec = FieldSpec::req(2, Wire::Utf8);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "EffectIdentity",
            fields: &[TAG, VALUE],
        };
    }
    pub(crate) mod route_source {
        use super::*;
        pub(crate) const TAG: FieldSpec = FieldSpec::req(1, Wire::U8);
        pub(crate) const ID: FieldSpec = FieldSpec::req(2, Wire::Utf8);
        pub(crate) const TAP: FieldSpec = FieldSpec::req(3, Wire::U8);
        pub(crate) static TRACK: MessageSpec = MessageSpec {
            name: "RouteSourceTrack",
            fields: &[TAG, ID, TAP],
        };
        pub(crate) static SUBMIX: MessageSpec = MessageSpec {
            name: "RouteSourceSubmix",
            fields: &[TAG, ID, TAP],
        };
        pub(crate) static KNOWN: MessageSpec = MessageSpec {
            name: "RouteSourceKnownFields",
            fields: &[TAG, ID, TAP],
        };
    }
    pub(crate) mod route_destination {
        use super::*;
        pub(crate) const TAG: FieldSpec = FieldSpec::req(1, Wire::U8);
        pub(crate) const ID: FieldSpec = FieldSpec::req(2, Wire::Utf8);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "RouteDestination",
            fields: &[TAG, ID],
        };
    }
    pub(crate) mod sidechain {
        use super::*;
        pub(crate) const TAG: FieldSpec = FieldSpec::req(1, Wire::U8);
        pub(crate) const SOURCE: FieldSpec = FieldSpec::msg(2, true, false, &route_source::KNOWN);
        pub(crate) const PORT_ID: FieldSpec = FieldSpec::req(3, Wire::Utf8);
        pub(crate) static NONE: MessageSpec = MessageSpec {
            name: "SidechainNone",
            fields: &[TAG],
        };
        pub(crate) static ROUTED: MessageSpec = MessageSpec {
            name: "SidechainRouted",
            fields: &[TAG, SOURCE, PORT_ID],
        };
        pub(crate) static KNOWN: MessageSpec = MessageSpec {
            name: "SidechainKnownFields",
            fields: &[TAG, SOURCE, PORT_ID],
        };
    }
    pub(crate) mod param {
        use super::*;
        pub(crate) const PARAMETER_ID: FieldSpec = FieldSpec::req(1, Wire::U32);
        pub(crate) const CHANNEL: FieldSpec = FieldSpec::req(2, Wire::U8);
        pub(crate) const UNIT: FieldSpec = FieldSpec::req(3, Wire::U8);
        pub(crate) const VALUE: FieldSpec = FieldSpec::req(4, Wire::F32);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "EffectParam",
            fields: &[PARAMETER_ID, CHANNEL, UNIT, VALUE],
        };
    }
    pub(crate) mod console_entry {
        use super::*;
        pub(crate) const SLOT: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const BYPASS: FieldSpec = FieldSpec::req(2, Wire::Bool);
        pub(crate) const PARAM: FieldSpec = FieldSpec::msg(3, true, true, &param::SPEC);
        // #1369 D4: appended in place (decision 12/13 precedent); `slot` is code 1, then the
        // three link tokens in `LinkMode` order.
        pub(crate) const LINK_MODE: FieldSpec = FieldSpec::req(4, Wire::U8);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "ConsoleEntry",
            fields: &[SLOT, BYPASS, PARAM, LINK_MODE],
        };
    }
    // Decision 12 (#1094): the session console declaration, with the session model's own field
    // registry (`session::keys::console` and `console_slot`). A slot has no sidechain field.
    pub(crate) mod console_slot {
        use super::*;
        pub(crate) const SLOT: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const IDENTITY: FieldSpec =
            FieldSpec::msg(2, true, false, &effect_identity::SPEC);
        pub(crate) const QUALITY: FieldSpec = FieldSpec::req(3, Wire::U8);
        pub(crate) const LINK_MODE: FieldSpec = FieldSpec::req(4, Wire::U8);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "ConsoleSlot",
            fields: &[SLOT, IDENTITY, QUALITY, LINK_MODE],
        };
    }
    pub(crate) mod console {
        use super::*;
        pub(crate) const PRE_INSERT: FieldSpec = FieldSpec::msg(1, true, true, &console_slot::SPEC);
        pub(crate) const POST_INSERT: FieldSpec =
            FieldSpec::msg(2, true, true, &console_slot::SPEC);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "Console",
            fields: &[PRE_INSERT, POST_INSERT],
        };
    }
    pub(crate) mod rack {
        use super::*;
        pub(crate) const EFFECT: FieldSpec = FieldSpec::msg(1, true, true, &effect::SPEC);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "Rack",
            fields: &[EFFECT],
        };
    }
    pub(crate) mod effect {
        use super::*;
        pub(crate) const ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const IDENTITY: FieldSpec =
            FieldSpec::msg(2, true, false, &effect_identity::SPEC);
        pub(crate) const QUALITY: FieldSpec = FieldSpec::req(3, Wire::U8);
        pub(crate) const BYPASS: FieldSpec = FieldSpec::req(4, Wire::Bool);
        pub(crate) const LINK_MODE: FieldSpec = FieldSpec::req(5, Wire::U8);
        pub(crate) const PARAM: FieldSpec = FieldSpec::msg(6, true, true, &param::SPEC);
        pub(crate) const SIDECHAIN: FieldSpec = FieldSpec::msg(7, true, false, &sidechain::KNOWN);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "Effect",
            fields: &[ID, IDENTITY, QUALITY, BYPASS, LINK_MODE, PARAM, SIDECHAIN],
        };
    }
    pub(crate) mod fader {
        use super::*;
        pub(crate) const LEFT_DB: FieldSpec = FieldSpec::req(1, Wire::F32);
        pub(crate) const RIGHT_DB: FieldSpec = FieldSpec::req(2, Wire::F32);
        pub(crate) const LEFT_MUTE: FieldSpec = FieldSpec::req(3, Wire::Bool);
        pub(crate) const RIGHT_MUTE: FieldSpec = FieldSpec::req(4, Wire::Bool);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "DualMonoFader",
            fields: &[LEFT_DB, RIGHT_DB, LEFT_MUTE, RIGHT_MUTE],
        };
    }
    pub(crate) mod matrix_or_pan {
        use super::*;
        pub(crate) const TAG: FieldSpec = FieldSpec::req(1, Wire::U8);
        pub(crate) const A: FieldSpec = FieldSpec::req(2, Wire::F32);
        pub(crate) const B: FieldSpec = FieldSpec::req(3, Wire::F32);
        pub(crate) const C_OR_SMOOTHING: FieldSpec = FieldSpec::req(4, Wire::F32);
        pub(crate) const D: FieldSpec = FieldSpec::req(5, Wire::F32);
        pub(crate) const SMOOTHING: FieldSpec = FieldSpec::req(6, Wire::U32);
        pub(crate) const PAN_SMOOTHING: FieldSpec = FieldSpec::req(4, Wire::U32);
        pub(crate) static PAN: MessageSpec = MessageSpec {
            name: "Pan",
            fields: &[TAG, A, B, PAN_SMOOTHING],
        };
        pub(crate) static MATRIX: MessageSpec = MessageSpec {
            name: "Matrix",
            fields: &[TAG, A, B, C_OR_SMOOTHING, D, SMOOTHING],
        };
        pub(crate) static KNOWN: MessageSpec = MessageSpec {
            name: "MatrixOrPanKnownFields",
            fields: &[TAG, A, B, C_OR_SMOOTHING, D, SMOOTHING],
        };
    }
    pub(crate) mod track {
        use super::*;
        pub(crate) const ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const SOURCE_ID: FieldSpec = FieldSpec::req(2, Wire::Utf8);
        pub(crate) const LEFT_SOURCE_CHANNEL: FieldSpec = FieldSpec::req(3, Wire::U8);
        pub(crate) const RIGHT_SOURCE_CHANNEL: FieldSpec = FieldSpec::req(4, Wire::U8);
        pub(crate) const BUILTINS: FieldSpec = FieldSpec::msg(5, true, false, &builtins::SPEC);
        // Decision 12: fields 6 (`simd1`) and 8 (`simd2`) are retired and never reallocated;
        // `inserts` keeps the retired `dynamic` rack's field 7, and `console` is appended as 11.
        pub(crate) const INSERTS: FieldSpec = FieldSpec::msg(7, true, false, &rack::SPEC);
        pub(crate) const FADER: FieldSpec = FieldSpec::msg(9, true, false, &fader::SPEC);
        pub(crate) const MATRIX_OR_PAN: FieldSpec =
            FieldSpec::msg(10, true, false, &matrix_or_pan::KNOWN);
        pub(crate) const CONSOLE: FieldSpec = FieldSpec::msg(11, true, true, &console_entry::SPEC);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "Track",
            fields: &[
                ID,
                SOURCE_ID,
                LEFT_SOURCE_CHANNEL,
                RIGHT_SOURCE_CHANNEL,
                BUILTINS,
                INSERTS,
                FADER,
                MATRIX_OR_PAN,
                CONSOLE,
            ],
        };
        /// The retired track fields (#1094): a peer that sends one is refused in either flag
        /// form. A mandatory one is an unknown required field; an optional one is *not* skipped
        /// as an unknown optional field would be, because a retired ID is known, not new.
        pub(crate) static RETIRED: MessageSpec = MessageSpec {
            name: "TrackRetiredFields",
            fields: &[
                FieldSpec::opt(6, Wire::Message),
                FieldSpec::opt(8, Wire::Message),
            ],
        };
    }
    pub(crate) mod submix {
        use super::*;
        pub(crate) const ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        // #1199: the submix strip reuses the track's field specs. Field 3 is `console`, the
        // repeated console entry of the track's field 11 (#1202 D1); pan and matrix share the
        // one tagged field 6, as the track's 10.
        pub(crate) const BUILTINS: FieldSpec = FieldSpec::msg(2, true, false, &builtins::SPEC);
        pub(crate) const CONSOLE: FieldSpec = FieldSpec::msg(3, true, true, &console_entry::SPEC);
        pub(crate) const INSERTS: FieldSpec = FieldSpec::msg(4, true, false, &rack::SPEC);
        pub(crate) const FADER: FieldSpec = FieldSpec::msg(5, true, false, &fader::SPEC);
        pub(crate) const MATRIX_OR_PAN: FieldSpec =
            FieldSpec::msg(6, true, false, &matrix_or_pan::KNOWN);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "Submix",
            fields: &[ID, BUILTINS, CONSOLE, INSERTS, FADER, MATRIX_OR_PAN],
        };
    }
    pub(crate) mod vca {
        use super::*;
        // #1241 D1: a VCA's `members` is a repeated UTF-8 stable ID, built exactly as
        // `set_track_effect_order::EFFECT_ID`; an empty list is zero occurrences.
        pub(crate) const ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const FADER: FieldSpec = FieldSpec::msg(2, true, false, &fader::SPEC);
        pub(crate) const MEMBERS: FieldSpec = FieldSpec {
            id: 3,
            wire: Wire::Utf8,
            mandatory: true,
            repeated: true,
            nested: None,
        };
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "Vca",
            fields: &[ID, FADER, MEMBERS],
        };
    }
    pub(crate) mod output {
        use super::*;
        pub(crate) const ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "Output",
            fields: &[ID],
        };
    }
    pub(crate) mod channel_matrix {
        use super::*;
        pub(crate) const LL: FieldSpec = FieldSpec::req(1, Wire::F32);
        pub(crate) const LR: FieldSpec = FieldSpec::req(2, Wire::F32);
        pub(crate) const RL: FieldSpec = FieldSpec::req(3, Wire::F32);
        pub(crate) const RR: FieldSpec = FieldSpec::req(4, Wire::F32);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "ChannelMatrix",
            fields: &[LL, LR, RL, RR],
        };
    }
    pub(crate) mod route {
        use super::*;
        pub(crate) const ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const SOURCE: FieldSpec = FieldSpec::msg(2, true, false, &route_source::KNOWN);
        pub(crate) const DESTINATION: FieldSpec =
            FieldSpec::msg(3, true, false, &route_destination::SPEC);
        pub(crate) const CHANNEL_MATRIX: FieldSpec =
            FieldSpec::msg(4, true, false, &channel_matrix::SPEC);
        pub(crate) const GAIN_DB: FieldSpec = FieldSpec::req(5, Wire::F32);
        pub(crate) const MUTE: FieldSpec = FieldSpec::req(6, Wire::Bool);
        pub(crate) const FOLLOWS_MUTE: FieldSpec = FieldSpec::req(7, Wire::Bool);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "Route",
            fields: &[
                ID,
                SOURCE,
                DESTINATION,
                CHANNEL_MATRIX,
                GAIN_DB,
                MUTE,
                FOLLOWS_MUTE,
            ],
        };
    }
    pub(crate) mod automation_segment {
        use super::*;
        pub(crate) const SHAPE: FieldSpec = FieldSpec::req(1, Wire::U8);
        pub(crate) const START_SAMPLE: FieldSpec = FieldSpec::req(2, Wire::U64);
        pub(crate) const END_SAMPLE: FieldSpec = FieldSpec::req(3, Wire::U64);
        pub(crate) const START_VALUE: FieldSpec = FieldSpec::req(4, Wire::F32);
        pub(crate) const END_VALUE: FieldSpec = FieldSpec::req(5, Wire::F32);
        pub(crate) const UNIT: FieldSpec = FieldSpec::req(6, Wire::U8);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "AutomationSegment",
            fields: &[
                SHAPE,
                START_SAMPLE,
                END_SAMPLE,
                START_VALUE,
                END_VALUE,
                UNIT,
            ],
        };
    }
    pub(crate) mod automation_offset {
        use super::*;
        pub(crate) const LEFT_DB: FieldSpec = FieldSpec::req(1, Wire::F32);
        pub(crate) const RIGHT_DB: FieldSpec = FieldSpec::req(2, Wire::F32);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "AutomationOffset",
            fields: &[LEFT_DB, RIGHT_DB],
        };
    }
    pub(crate) mod automation {
        use super::*;
        pub(crate) const ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        // The target is the address `SET_CONTROL` and the control list carry (#1566 D4).
        pub(crate) const TARGET: FieldSpec =
            FieldSpec::msg(2, true, false, &crate::schema::control_address::SPEC);
        pub(crate) const SEGMENT: FieldSpec =
            FieldSpec::msg(3, true, true, &automation_segment::SPEC);
        // The stored hand-move offset (#1650 D1): required, appended after the segments.
        pub(crate) const OFFSET: FieldSpec =
            FieldSpec::msg(4, true, false, &automation_offset::SPEC);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "Automation",
            fields: &[ID, TARGET, SEGMENT, OFFSET],
        };
    }

    pub(crate) mod set_session_id {
        use super::*;
        pub(crate) const SESSION_ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "SetSessionId",
            fields: &[SESSION_ID],
        };
    }
    pub(crate) mod set_sample_rate {
        use super::*;
        pub(crate) const SAMPLE_RATE_HZ: FieldSpec = FieldSpec::req(1, Wire::U32);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "SetSampleRateHz",
            fields: &[SAMPLE_RATE_HZ],
        };
    }
    pub(crate) mod set_quantum {
        use super::*;
        pub(crate) const QUANTUM_FRAMES: FieldSpec = FieldSpec::req(1, Wire::U32);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "SetQuantumFrames",
            fields: &[QUANTUM_FRAMES],
        };
    }
    pub(crate) mod set_render_profile {
        use super::*;
        pub(crate) const VALUE: FieldSpec = FieldSpec::msg(1, true, false, &render_profile::SPEC);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "SetRenderProfile",
            fields: &[VALUE],
        };
    }
    pub(crate) mod set_output_profile {
        use super::*;
        pub(crate) const VALUE: FieldSpec = FieldSpec::msg(1, true, false, &output_profile::SPEC);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "SetOutputProfile",
            fields: &[VALUE],
        };
    }
    pub(crate) mod set_console {
        use super::*;
        pub(crate) const VALUE: FieldSpec = FieldSpec::msg(1, true, false, &console::SPEC);
        /// #1394: this edit's own ramp length in samples, absent for the session default.
        pub(crate) const RAMP_SAMPLES: FieldSpec = FieldSpec::opt(2, Wire::U32);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "SetConsole",
            fields: &[VALUE, RAMP_SAMPLES],
        };
    }
    pub(crate) mod upsert_source {
        use super::*;
        pub(crate) const VALUE: FieldSpec = FieldSpec::msg(1, true, false, &source::SPEC);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "UpsertSource",
            fields: &[VALUE],
        };
    }
    pub(crate) mod remove_source {
        use super::*;
        pub(crate) const SOURCE_ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "RemoveSource",
            fields: &[SOURCE_ID],
        };
    }
    pub(crate) mod set_source_content {
        use super::*;
        pub(crate) const SOURCE_ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const CONTENT: FieldSpec = FieldSpec::req(2, Wire::Utf8);
        pub(crate) const CHANNELS: FieldSpec = FieldSpec::req(3, Wire::U8);
        pub(crate) const BIT_DEPTH: FieldSpec = FieldSpec::req(4, Wire::U8);
        pub(crate) const FRAMES: FieldSpec = FieldSpec::req(5, Wire::U64);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "SetSourceContent",
            fields: &[SOURCE_ID, CONTENT, CHANNELS, BIT_DEPTH, FRAMES],
        };
    }
    pub(crate) mod upsert_track {
        use super::*;
        pub(crate) const VALUE: FieldSpec = FieldSpec::msg(1, true, false, &track::SPEC);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "UpsertTrack",
            fields: &[VALUE],
        };
    }
    pub(crate) mod remove_track {
        use super::*;
        pub(crate) const TRACK_ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "RemoveTrack",
            fields: &[TRACK_ID],
        };
    }
    pub(crate) mod set_track_source {
        use super::*;
        pub(crate) const TRACK_ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const SOURCE_ID: FieldSpec = FieldSpec::req(2, Wire::Utf8);
        pub(crate) const LEFT_CHANNEL: FieldSpec = FieldSpec::req(3, Wire::U8);
        pub(crate) const RIGHT_CHANNEL: FieldSpec = FieldSpec::req(4, Wire::U8);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "SetTrackSourceAssignment",
            fields: &[TRACK_ID, SOURCE_ID, LEFT_CHANNEL, RIGHT_CHANNEL],
        };
    }
    pub(crate) mod set_track_rack {
        use super::*;
        pub(crate) const TRACK_ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const RACK_NAME: FieldSpec = FieldSpec::req(2, Wire::U8);
        pub(crate) const RACK: FieldSpec = FieldSpec::msg(3, true, false, &rack::SPEC);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "SetTrackRack",
            fields: &[TRACK_ID, RACK_NAME, RACK],
        };
    }
    pub(crate) mod put_track_effect {
        use super::*;
        pub(crate) const TRACK_ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const RACK_NAME: FieldSpec = FieldSpec::req(2, Wire::U8);
        pub(crate) const FINAL_POSITION: FieldSpec = FieldSpec::req(3, Wire::U32);
        pub(crate) const EFFECT: FieldSpec = FieldSpec::msg(4, true, false, &effect::SPEC);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "PutTrackEffect",
            fields: &[TRACK_ID, RACK_NAME, FINAL_POSITION, EFFECT],
        };
    }
    pub(crate) mod remove_track_effect {
        use super::*;
        pub(crate) const TRACK_ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const RACK_NAME: FieldSpec = FieldSpec::req(2, Wire::U8);
        pub(crate) const EFFECT_ID: FieldSpec = FieldSpec::req(3, Wire::Utf8);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "RemoveTrackEffect",
            fields: &[TRACK_ID, RACK_NAME, EFFECT_ID],
        };
    }
    pub(crate) mod set_track_effect_order {
        use super::*;
        pub(crate) const TRACK_ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const RACK_NAME: FieldSpec = FieldSpec::req(2, Wire::U8);
        pub(crate) const EFFECT_ID: FieldSpec = FieldSpec {
            id: 3,
            wire: Wire::Utf8,
            mandatory: true,
            repeated: true,
            nested: None,
        };
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "SetTrackEffectOrder",
            fields: &[TRACK_ID, RACK_NAME, EFFECT_ID],
        };
    }
    pub(crate) mod set_effect_identity {
        use super::*;
        pub(crate) const TRACK_ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const RACK_NAME: FieldSpec = FieldSpec::req(2, Wire::U8);
        pub(crate) const EFFECT_ID: FieldSpec = FieldSpec::req(3, Wire::Utf8);
        pub(crate) const VALUE: FieldSpec = FieldSpec::msg(4, true, false, &effect_identity::SPEC);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "SetEffectIdentity",
            fields: &[TRACK_ID, RACK_NAME, EFFECT_ID, VALUE],
        };
    }
    pub(crate) mod set_effect_quality {
        use super::*;
        pub(crate) const TRACK_ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const RACK_NAME: FieldSpec = FieldSpec::req(2, Wire::U8);
        pub(crate) const EFFECT_ID: FieldSpec = FieldSpec::req(3, Wire::Utf8);
        pub(crate) const VALUE: FieldSpec = FieldSpec::req(4, Wire::U8);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "SetEffectQuality",
            fields: &[TRACK_ID, RACK_NAME, EFFECT_ID, VALUE],
        };
    }
    pub(crate) mod set_effect_sidechain {
        use super::*;
        pub(crate) const TRACK_ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const RACK_NAME: FieldSpec = FieldSpec::req(2, Wire::U8);
        pub(crate) const EFFECT_ID: FieldSpec = FieldSpec::req(3, Wire::Utf8);
        pub(crate) const VALUE: FieldSpec = FieldSpec::msg(4, true, false, &sidechain::KNOWN);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "SetEffectSidechain",
            fields: &[TRACK_ID, RACK_NAME, EFFECT_ID, VALUE],
        };
    }
    pub(crate) mod set_track_console {
        use super::*;
        pub(crate) const TRACK_ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const ENTRY: FieldSpec = FieldSpec::msg(2, true, true, &console_entry::SPEC);
        /// #1394: this edit's own ramp length in samples, absent for the session default.
        pub(crate) const RAMP_SAMPLES: FieldSpec = FieldSpec::opt(3, Wire::U32);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "SetTrackConsole",
            fields: &[TRACK_ID, ENTRY, RAMP_SAMPLES],
        };
    }
    pub(crate) mod upsert_submix {
        use super::*;
        pub(crate) const VALUE: FieldSpec = FieldSpec::msg(1, true, false, &submix::SPEC);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "UpsertSubmix",
            fields: &[VALUE],
        };
    }
    pub(crate) mod remove_submix {
        use super::*;
        pub(crate) const ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "RemoveSubmix",
            fields: &[ID],
        };
    }
    pub(crate) mod upsert_output {
        use super::*;
        pub(crate) const VALUE: FieldSpec = FieldSpec::msg(1, true, false, &output::SPEC);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "UpsertOutput",
            fields: &[VALUE],
        };
    }
    pub(crate) mod remove_output {
        use super::*;
        pub(crate) const ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "RemoveOutput",
            fields: &[ID],
        };
    }
    pub(crate) mod upsert_route {
        use super::*;
        pub(crate) const VALUE: FieldSpec = FieldSpec::msg(1, true, false, &route::SPEC);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "UpsertRoute",
            fields: &[VALUE],
        };
    }
    pub(crate) mod remove_route {
        use super::*;
        pub(crate) const ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "RemoveRoute",
            fields: &[ID],
        };
    }
    pub(crate) mod set_route_source {
        use super::*;
        pub(crate) const ROUTE_ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const VALUE: FieldSpec = FieldSpec::msg(2, true, false, &route_source::KNOWN);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "SetRouteSource",
            fields: &[ROUTE_ID, VALUE],
        };
    }
    pub(crate) mod set_route_destination {
        use super::*;
        pub(crate) const ROUTE_ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const VALUE: FieldSpec =
            FieldSpec::msg(2, true, false, &route_destination::SPEC);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "SetRouteDestination",
            fields: &[ROUTE_ID, VALUE],
        };
    }
    pub(crate) mod upsert_automation {
        use super::*;
        pub(crate) const VALUE: FieldSpec = FieldSpec::msg(1, true, false, &automation::SPEC);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "UpsertAutomation",
            fields: &[VALUE],
        };
    }
    pub(crate) mod remove_automation {
        use super::*;
        pub(crate) const ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "RemoveAutomation",
            fields: &[ID],
        };
    }
    pub(crate) mod set_automation_target {
        use super::*;
        pub(crate) const ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const VALUE: FieldSpec =
            FieldSpec::msg(2, true, false, &crate::schema::control_address::SPEC);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "SetAutomationTarget",
            fields: &[ID, VALUE],
        };
    }
    pub(crate) mod set_automation_segments {
        use super::*;
        pub(crate) const ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) const SEGMENT: FieldSpec =
            FieldSpec::msg(2, true, true, &automation_segment::SPEC);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "SetAutomationSegments",
            fields: &[ID, SEGMENT],
        };
    }
    pub(crate) mod upsert_vca {
        use super::*;
        pub(crate) const VALUE: FieldSpec = FieldSpec::msg(1, true, false, &vca::SPEC);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "UpsertVca",
            fields: &[VALUE],
        };
    }
    pub(crate) mod remove_vca {
        use super::*;
        pub(crate) const ID: FieldSpec = FieldSpec::req(1, Wire::Utf8);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "RemoveVca",
            fields: &[ID],
        };
    }

    /// `SET_CONTROL` (decision 17, C5): any control by its stable address (#1553 D2). The address is
    /// the control address message of the control list, with an optional seventh field, the native
    /// effect the edit was written for (#1678); a set requires its `control` and `lane`.
    pub(crate) mod set_control {
        use super::*;
        pub(crate) const ADDRESS: FieldSpec =
            FieldSpec::msg(1, true, false, &crate::schema::set_control_address::SPEC);
        pub(crate) const VALUE: FieldSpec = FieldSpec::req(2, Wire::F32);
        /// This edit's own ramp length in samples, absent for the row's default.
        pub(crate) const RAMP_SAMPLES: FieldSpec = FieldSpec::opt(3, Wire::U32);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "SetControl",
            fields: &[ADDRESS, VALUE, RAMP_SAMPLES],
        };
    }

    /// `OBSERVE` (decision 17, C9, #1584 D1): arm or disarm an effect's observation tap. The address
    /// is the control list's address message with a strip owner, a console or inserts section, its
    /// slot ID, and no `control` or `lane`; `window_blocks` `0` is the plan's meter window.
    pub(crate) mod observe {
        use super::*;
        pub(crate) const ADDRESS: FieldSpec =
            FieldSpec::msg(1, true, false, &crate::schema::control_address::SPEC);
        pub(crate) const TAP_ID: FieldSpec = FieldSpec::req(2, Wire::U32);
        pub(crate) const ARMED: FieldSpec = FieldSpec::req(3, Wire::Bool);
        pub(crate) const WINDOW_BLOCKS: FieldSpec = FieldSpec::req(4, Wire::U32);
        /// #2087: the tag the disarmed arm's reply gave, present exactly on a disarm.
        pub(crate) const ARM_TAG: FieldSpec = FieldSpec::opt(5, Wire::U64);
        pub(crate) static SPEC: MessageSpec = MessageSpec {
            name: "Observe",
            fields: &[ADDRESS, TAP_ID, ARMED, WINDOW_BLOCKS, ARM_TAG],
        };
    }

    /// The optional `ramp_samples` field of a live-value edit's payload (#1394 D1), or `None` for
    /// an opcode that does not carry one.
    pub(crate) const fn ramp_samples_field(opcode: crate::SessionEditOpcode) -> Option<FieldSpec> {
        use crate::SessionEditOpcode::*;
        match opcode {
            SetConsole => Some(set_console::RAMP_SAMPLES),
            SetTrackConsole => Some(set_track_console::RAMP_SAMPLES),
            SetControl => Some(set_control::RAMP_SAMPLES),
            // Listed one by one on purpose, with no wildcard: a new opcode must be classified here,
            // as in `SessionEdit::ramp_samples`, or the build fails.
            SetSessionId
            | SetSampleRateHz
            | SetQuantumFrames
            | SetRenderProfile
            | SetOutputProfile
            | UpsertSource
            | RemoveSource
            | SetSourceContent
            | UpsertTrack
            | RemoveTrack
            | SetTrackSourceAssignment
            | SetTrackRack
            | PutTrackEffect
            | RemoveTrackEffect
            | SetTrackEffectOrder
            | SetEffectIdentity
            | SetEffectQuality
            | SetEffectSidechain
            | UpsertSubmix
            | RemoveSubmix
            | UpsertOutput
            | RemoveOutput
            | UpsertRoute
            | RemoveRoute
            | SetRouteSource
            | SetRouteDestination
            | UpsertAutomation
            | RemoveAutomation
            | SetAutomationTarget
            | SetAutomationSegments
            | UpsertVca
            | RemoveVca
            | Observe => None,
        }
    }

    pub(crate) fn payload_spec(opcode: crate::SessionEditOpcode) -> &'static MessageSpec {
        use crate::SessionEditOpcode::*;
        match opcode {
            SetSessionId => &set_session_id::SPEC,
            SetSampleRateHz => &set_sample_rate::SPEC,
            SetQuantumFrames => &set_quantum::SPEC,
            SetRenderProfile => &set_render_profile::SPEC,
            SetOutputProfile => &set_output_profile::SPEC,
            SetConsole => &set_console::SPEC,
            UpsertSource => &upsert_source::SPEC,
            RemoveSource => &remove_source::SPEC,
            SetSourceContent => &set_source_content::SPEC,
            UpsertTrack => &upsert_track::SPEC,
            RemoveTrack => &remove_track::SPEC,
            SetTrackSourceAssignment => &set_track_source::SPEC,
            SetTrackRack => &set_track_rack::SPEC,
            PutTrackEffect => &put_track_effect::SPEC,
            RemoveTrackEffect => &remove_track_effect::SPEC,
            SetTrackEffectOrder => &set_track_effect_order::SPEC,
            SetEffectIdentity => &set_effect_identity::SPEC,
            SetEffectQuality => &set_effect_quality::SPEC,
            SetEffectSidechain => &set_effect_sidechain::SPEC,
            SetTrackConsole => &set_track_console::SPEC,
            UpsertSubmix => &upsert_submix::SPEC,
            RemoveSubmix => &remove_submix::SPEC,
            UpsertOutput => &upsert_output::SPEC,
            RemoveOutput => &remove_output::SPEC,
            UpsertRoute => &upsert_route::SPEC,
            RemoveRoute => &remove_route::SPEC,
            SetRouteSource => &set_route_source::SPEC,
            SetRouteDestination => &set_route_destination::SPEC,
            UpsertAutomation => &upsert_automation::SPEC,
            RemoveAutomation => &remove_automation::SPEC,
            SetAutomationTarget => &set_automation_target::SPEC,
            SetAutomationSegments => &set_automation_segments::SPEC,
            UpsertVca => &upsert_vca::SPEC,
            RemoveVca => &remove_vca::SPEC,
            SetControl => &set_control::SPEC,
            Observe => &observe::SPEC,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The session JSON keys and the session wire fields are one ID space (#1568).
    #[test]
    fn session_keys_and_wire_fields_have_equal_ids() {
        use std::collections::BTreeSet;

        use super::session::{
            automation, automation_offset, automation_segment, builtins, channel_builtins,
            channel_matrix, console, console_entry, console_slot, effect, effect_identity, fader,
            matrix_or_pan, output, output_profile, param, rack, render_profile, route,
            route_destination, route_source, sidechain, source, submix, track, vca,
        };

        // (keys module, its wire message, IDs the wire carries with no key). `None` is a keys
        // module with no wire message of its own: `control_smoothing` is written only by the
        // `SET_CONTROL` row of the session control table and never framed as a session record.
        // The `matrix_or_pan` tag (field 1) is wire-only: the walk emits it as `wire_tag`.
        let table: [(&str, Option<&MessageSpec>, &[u16]); 28] = [
            ("console", Some(&console::SPEC), &[]),
            ("console_slot", Some(&console_slot::SPEC), &[]),
            ("console_entry", Some(&console_entry::SPEC), &[]),
            ("render_profile", Some(&render_profile::SPEC), &[]),
            ("output_profile", Some(&output_profile::SPEC), &[]),
            ("source", Some(&source::SPEC), &[]),
            ("track", Some(&track::SPEC), &[]),
            ("builtins", Some(&builtins::SPEC), &[]),
            ("channel_builtins", Some(&channel_builtins::SPEC), &[]),
            ("rack", Some(&rack::SPEC), &[]),
            ("effect", Some(&effect::SPEC), &[]),
            ("identity", Some(&effect_identity::SPEC), &[]),
            ("param", Some(&param::SPEC), &[]),
            ("sidechain", Some(&sidechain::KNOWN), &[]),
            ("fader", Some(&fader::SPEC), &[]),
            ("matrix_or_pan", Some(&matrix_or_pan::KNOWN), &[1]),
            ("submix", Some(&submix::SPEC), &[]),
            ("vca", Some(&vca::SPEC), &[]),
            ("control_smoothing", None, &[]),
            ("output", Some(&output::SPEC), &[]),
            ("route", Some(&route::SPEC), &[]),
            ("route_source", Some(&route_source::KNOWN), &[]),
            ("route_destination", Some(&route_destination::SPEC), &[]),
            ("channel_matrix", Some(&channel_matrix::SPEC), &[]),
            ("automation", Some(&automation::SPEC), &[]),
            ("automation_offset", Some(&automation_offset::SPEC), &[]),
            // An automation target is a control address (#1566): the keys are its field IDs.
            ("target", Some(&control_address::SPEC), &[]),
            ("segment", Some(&automation_segment::SPEC), &[]),
        ];

        let modules = ::session::keys::MODULES;
        for (name, spec, wire_only) in &table {
            let (_, keys) = modules
                .iter()
                .find(|(module, _)| module == name)
                .unwrap_or_else(|| panic!("table row `{name}` names no keys module"));
            let Some(spec) = spec else { continue };
            let from_keys: BTreeSet<u16> = keys
                .iter()
                .map(|key| key.id)
                .chain(wire_only.iter().copied())
                .collect();
            let from_wire: BTreeSet<u16> = spec.fields.iter().map(|field| field.id).collect();
            assert_eq!(
                from_keys, from_wire,
                "keys::{name} field IDs differ from wire message {}",
                spec.name
            );
        }
        // A retired track field ID is never given a key (#1094).
        let (_, track_keys) = modules.iter().find(|(m, _)| *m == "track").unwrap();
        for retired in track::RETIRED.fields {
            assert!(track_keys.iter().all(|key| key.id != retired.id));
        }
        // The root `session` has no wire message: it is never sent whole.
        for (module, _) in modules {
            assert!(
                *module == "session" || table.iter().any(|(name, ..)| name == module),
                "keys::{module} has no row in the pairing table"
            );
        }
    }

    #[test]
    fn migrated_specs_are_sorted_unique_and_structurally_consistent() {
        let specs = [
            &capabilities_request::SPEC,
            &enum_choice::SPEC,
            &snapshot_request::SPEC,
            &snapshot::SPEC,
            &transaction_applied::SPEC,
            &session_replace_request::SPEC,
            &session_committed::SPEC,
            &control_list_request::SPEC,
            &control_address::SPEC,
            &set_control_address::SPEC,
            &control_entry::SPEC,
            &control_list_page::SPEC,
            &control_state_page::SPEC,
            &response_configuration::SPEC,
            &response_override::SPEC,
            &response_request::SPEC,
            &response_result::SPEC,
            &response_member::SPEC,
            &response_ready::SPEC,
            &observation_selection::SPEC,
            &observation_read_request::SPEC,
            &observation_row::SPEC,
            &observation_read_page::SPEC,
            &spectrum_entry::SPEC,
            &spectrum_configure_request::SPEC,
            &spectrum_configured::SPEC,
            &spectrum_capture_request::SPEC,
            &spectrum_capture_reply::SPEC,
            &spectrum_stream_request::SPEC,
            &spectrum_stream_reply::SPEC,
            &loudness_request::SPEC,
            &loudness_reply::SPEC,
            &transport_get::SPEC,
            &transport_set::SPEC,
            &transport_set::RETIRED,
            &transport_snapshot::SPEC,
            &transport_snapshot::RETIRED,
            &transport_state_event::SPEC,
            &transport_state_event::RETIRED,
            &observation_tap_key::SPEC,
            &observation_ended::SPEC,
            &meter_batch::SPEC,
            &meter_entry::SPEC,
            &telemetry_configuration::SPEC,
            &telemetry_configuration::RETIRED,
            &counters_request::SPEC,
            &counter_value::SPEC,
            &counter_snapshot::SPEC,
            &diagnostics_request::SPEC,
            &path_segment::SPEC,
            &diagnostic::SPEC,
            &backpressure::SPEC,
            &non_ok::SPEC,
            &diagnostics_page::SPEC,
            &diagnostic_event::SPEC,
            &capabilities::SPEC,
            &capabilities::RETIRED,
        ];
        fn validate(spec: &'static MessageSpec) {
            assert!(!spec.name.is_empty());
            assert!(spec.fields.windows(2).all(|pair| pair[0].id < pair[1].id));
            for field in spec.fields {
                assert_ne!(field.id, 0);
                assert_eq!(field.wire == Wire::Message, field.nested.is_some());
                if let Some(nested) = field.nested {
                    validate(nested);
                }
            }
        }
        for spec in specs {
            validate(spec);
        }
        validate(&session::transaction::SPEC);
        validate(&session::edit::SPEC);
        for edit in crate::session_wire::complete_all_opcode_fixture() {
            validate(session::payload_spec(edit.opcode()));
        }
        assert!(std::hint::black_box(control_entry::HANDLE).mandatory);
    }

    #[test]
    fn wire_registry_covers_every_frozen_code() {
        let wires = [
            Wire::U8,
            Wire::U16,
            Wire::U32,
            Wire::U64,
            Wire::I64,
            Wire::F32,
            Wire::F64,
            Wire::Bool,
            Wire::Utf8,
            Wire::Bytes,
            Wire::Message,
            Wire::PackedU16,
            Wire::PackedU32,
            Wire::PackedU64,
            Wire::PackedF32,
        ];
        assert!(wires.into_iter().enumerate().all(|(index, wire)| {
            wire.raw() == u8::try_from(index + 1).expect("wire registry fits u8")
        }));
    }

    #[test]
    fn parameter_enum_wire_mappings_are_exhaustive_and_roundtrip() {
        for (value, wire) in [
            // Decision 12: `inserts` kept the retired `dynamic` rack's `2`.
            (ParameterRack::Inserts, 2),
            // #178, ruled by #210's D2: the strip's own builtin section, unmoved at `4`.
            (ParameterRack::Builtins, 4),
            // Decision 12: a session console slot, appended.
            (ParameterRack::Console, 5),
        ] {
            assert_eq!(value as u64, u64::from(wire));
            assert_eq!(parameter_rack_wire(value), wire);
            assert_eq!(parameter_rack_from_wire(wire), Ok(value));
            let session = ::session::RackName::from(value);
            assert_eq!(ParameterRack::from(session), value);
            assert_eq!(session_parameter_rack_wire(session), wire);
            assert_eq!(session_parameter_rack_from_wire(wire), Ok(session));
            // The session enum spells the same explicit codes: an index-derived `RackName::wire()`
            // would make `inserts` `1` and `builtins` `2`.
            assert_eq!(session.wire(), wire);
            assert_eq!(::session::RackName::from_wire(wire), Some(session));
        }
        // The retired `simd1` and `simd2` codes are refused on both tables, never reinterpreted.
        for retired in [1, 3] {
            assert_eq!(
                parameter_rack_from_wire(retired),
                Err(crate::DecodeError::InvalidTlv)
            );
            assert_eq!(::session::RackName::from_wire(retired), None);
        }
        for (value, wire) in [
            (ParameterChannel::Left, 1),
            (ParameterChannel::Right, 2),
            (ParameterChannel::Both, 3),
        ] {
            assert_eq!(value as u64, u64::from(wire));
            assert_eq!(parameter_channel_wire(value), wire);
            assert_eq!(parameter_channel_from_wire(wire), Ok(value));
            let session = ::session::ParameterChannel::from(value);
            assert_eq!(ParameterChannel::from(session), value);
            assert_eq!(session_parameter_channel_wire(session), wire);
            assert_eq!(session_parameter_channel_from_wire(wire), Ok(session));
        }
        for (value, wire) in [
            (ParameterUnit::Db, 1),
            (ParameterUnit::Hz, 2),
            (ParameterUnit::Milliseconds, 3),
            (ParameterUnit::Samples, 4),
            (ParameterUnit::Linear, 5),
            (ParameterUnit::Ratio, 6),
        ] {
            assert_eq!(value as u64, u64::from(wire));
            assert_eq!(parameter_unit_wire(value), wire);
            assert_eq!(parameter_unit_from_wire(wire), Ok(value));
            let session = ::session::ParameterUnit::from(value);
            assert_eq!(ParameterUnit::from(session), value);
            assert_eq!(session_parameter_unit_wire(session), wire);
            assert_eq!(session_parameter_unit_from_wire(wire), Ok(session));
        }
        assert_eq!(
            parameter_rack_from_wire(0),
            Err(crate::DecodeError::InvalidTlv)
        );
        assert_eq!(
            parameter_channel_from_wire(4),
            Err(crate::DecodeError::InvalidTlv)
        );
        assert_eq!(
            parameter_unit_from_wire(7),
            Err(crate::DecodeError::InvalidTlv)
        );
    }

    #[test]
    fn top_level_counts_derive_required_optional_and_repeated_occurrences() {
        assert_eq!(
            capabilities::SPEC.field_count(&[]),
            u32::try_from(capabilities::SPEC.fields.len())
                .map_err(|_| crate::EncodeError::LimitExceeded)
        );
        assert_eq!(transport_set::SPEC.field_count(&[]), Ok(1));
        assert_eq!(
            transport_state_event::SPEC.field_count(&[]),
            Ok(3),
            "the event's origin is optional, so three required fields count"
        );
        assert_eq!(
            transport_state_event::SPEC
                .field_count(&[(transport_state_event::ORIGIN_REQUEST_ID, 1)]),
            Ok(4)
        );
        assert_eq!(
            counter_snapshot::SPEC.field_count(&[(counter_snapshot::VALUE, 3)]),
            Ok(4)
        );
        assert_eq!(
            transport_set::SPEC.field_count(&[(transport_set::STATE, 1)]),
            Err(crate::EncodeError::LimitExceeded)
        );
        assert_eq!(
            transport_state_event::SPEC.field_count(&[
                (transport_state_event::ORIGIN_REQUEST_ID, 1),
                (transport_state_event::ORIGIN_REQUEST_ID, 1),
            ]),
            Err(crate::EncodeError::LimitExceeded)
        );
    }
}
