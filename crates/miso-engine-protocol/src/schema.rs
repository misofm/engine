//! Declarative message-field registry for shared typed protocol payloads.
//!
//! Session-edit payloads and transport payloads retain their specialized readers until their
//! respective migrations. This registry is metadata consumed by the existing bounded reader; it
//! is never a second encoder or decoder.

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

pub(crate) mod descriptor {
    use super::{FieldSpec, MessageSpec, Wire, enum_choice};

    pub(crate) const HANDLE: FieldSpec = FieldSpec::req(1, Wire::U32);
    pub(crate) const TRACK_ID: FieldSpec = FieldSpec::req(2, Wire::Utf8);
    pub(crate) const RACK: FieldSpec = FieldSpec::req(3, Wire::U8);
    pub(crate) const EFFECT_ID: FieldSpec = FieldSpec::req(4, Wire::Utf8);
    pub(crate) const PARAMETER_ID: FieldSpec = FieldSpec::req(5, Wire::U32);
    pub(crate) const CHANNEL: FieldSpec = FieldSpec::req(6, Wire::U8);
    pub(crate) const VALUE_KIND: FieldSpec = FieldSpec::req(7, Wire::U8);
    pub(crate) const UNIT: FieldSpec = FieldSpec::req(8, Wire::U8);
    pub(crate) const DOMAIN: FieldSpec = FieldSpec::req(9, Wire::U8);
    pub(crate) const MINIMUM: FieldSpec = FieldSpec::opt(10, Wire::F32);
    pub(crate) const MAXIMUM: FieldSpec = FieldSpec::opt(11, Wire::F32);
    pub(crate) const DEFAULT: FieldSpec = FieldSpec::req(12, Wire::F32);
    pub(crate) const MAPPING: FieldSpec = FieldSpec::req(13, Wire::U8);
    pub(crate) const AUTOMATION_RATE: FieldSpec = FieldSpec::req(14, Wire::U8);
    pub(crate) const SMOOTHING_SAMPLES: FieldSpec = FieldSpec::req(15, Wire::U32);
    pub(crate) const FLAGS: FieldSpec = FieldSpec::req(16, Wire::U32);
    pub(crate) const DISPLAY_NAME: FieldSpec = FieldSpec::opt(17, Wire::Utf8);
    pub(crate) const DISPLAY_UNIT: FieldSpec = FieldSpec::opt(18, Wire::Utf8);
    pub(crate) const ENUM_CHOICE: FieldSpec = FieldSpec::msg(19, false, true, &enum_choice::SPEC);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ParameterDescriptor",
        fields: &[
            HANDLE,
            TRACK_ID,
            RACK,
            EFFECT_ID,
            PARAMETER_ID,
            CHANNEL,
            VALUE_KIND,
            UNIT,
            DOMAIN,
            MINIMUM,
            MAXIMUM,
            DEFAULT,
            MAPPING,
            AUTOMATION_RATE,
            SMOOTHING_SAMPLES,
            FLAGS,
            DISPLAY_NAME,
            DISPLAY_UNIT,
            ENUM_CHOICE,
        ],
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
    pub(crate) const CANONICAL_TOML_CHUNK: FieldSpec = FieldSpec::req(3, Wire::Bytes);
    pub(crate) const EOF: FieldSpec = FieldSpec::req(4, Wire::Bool);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "SessionSnapshot",
        fields: &[TOTAL_BYTES, OFFSET, CANONICAL_TOML_CHUNK, EOF],
    };
}

pub(crate) mod transaction_applied {
    use super::*;
    pub(crate) const APPLIED_OPERATIONS: FieldSpec = FieldSpec::req(1, Wire::U32);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "TransactionApplied",
        fields: &[APPLIED_OPERATIONS],
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

pub(crate) mod metadata_request {
    use super::*;
    pub(crate) const AFTER_HANDLE: FieldSpec = FieldSpec::req(1, Wire::U32);
    pub(crate) const LIMIT: FieldSpec = FieldSpec::req(2, Wire::U16);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ParameterMetadataRequest",
        fields: &[AFTER_HANDLE, LIMIT],
    };
}

pub(crate) mod metadata_page {
    use super::{descriptor, *};
    pub(crate) const LAST_HANDLE: FieldSpec = FieldSpec::req(1, Wire::U32);
    pub(crate) const EOF: FieldSpec = FieldSpec::req(2, Wire::Bool);
    pub(crate) const DESCRIPTOR: FieldSpec = FieldSpec::msg(3, true, true, &descriptor::SPEC);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ParameterMetadataPage",
        fields: &[LAST_HANDLE, EOF, DESCRIPTOR],
    };
}

pub(crate) mod state_request {
    use super::*;
    pub(crate) const HANDLES: FieldSpec = FieldSpec::req(1, Wire::PackedU32);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ParameterStateRequest",
        fields: &[HANDLES],
    };
}

pub(crate) mod state_page {
    use super::*;
    pub(crate) const OBSERVED_SAMPLE: FieldSpec = FieldSpec::req(1, Wire::U64);
    pub(crate) const COUNT: FieldSpec = FieldSpec::req(2, Wire::U16);
    pub(crate) const RECORD_BYTES: FieldSpec = FieldSpec::req(3, Wire::U16);
    pub(crate) const RECORDS: FieldSpec = FieldSpec::req(4, Wire::Bytes);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ParameterStatePage",
        fields: &[OBSERVED_SAMPLE, COUNT, RECORD_BYTES, RECORDS],
    };
}

pub(crate) mod automation_enqueued {
    use super::*;
    pub(crate) const ACCEPTED_RECORDS: FieldSpec = FieldSpec::req(1, Wire::U16);
    pub(crate) const OCCUPANCY: FieldSpec = FieldSpec::req(2, Wire::U64);
    pub(crate) const CAPACITY: FieldSpec = FieldSpec::req(3, Wire::U64);
    pub(crate) const GENERATION: FieldSpec = FieldSpec::req(4, Wire::U64);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "AutomationEnqueued",
        fields: &[ACCEPTED_RECORDS, OCCUPANCY, CAPACITY, GENERATION],
    };
}

pub(crate) mod automation_canceled {
    use super::*;
    pub(crate) const EVENT_SEQUENCE: FieldSpec = FieldSpec::req(1, Wire::U64);
    pub(crate) const ORIGIN_REQUEST_ID: FieldSpec = FieldSpec::req(2, Wire::U64);
    pub(crate) const CANCELED_RECORDS: FieldSpec = FieldSpec::req(3, Wire::U16);
    pub(crate) const REASON: FieldSpec = FieldSpec::req(4, Wire::U8);
    pub(crate) const QUEUE_GENERATION: FieldSpec = FieldSpec::req(5, Wire::U64);
    pub(crate) const EFFECTIVE_SAMPLE: FieldSpec = FieldSpec::opt(6, Wire::U64);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "AutomationCanceled",
        fields: &[
            EVENT_SEQUENCE,
            ORIGIN_REQUEST_ID,
            CANCELED_RECORDS,
            REASON,
            QUEUE_GENERATION,
            EFFECTIVE_SAMPLE,
        ],
    };
}

pub(crate) mod meter_batch {
    use super::*;
    pub(crate) const OBSERVED_SAMPLE: FieldSpec = FieldSpec::req(1, Wire::U64);
    pub(crate) const COUNT: FieldSpec = FieldSpec::req(2, Wire::U16);
    pub(crate) const RECORD_BYTES: FieldSpec = FieldSpec::req(3, Wire::U16);
    pub(crate) const RECORDS: FieldSpec = FieldSpec::req(4, Wire::Bytes);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "MeterBatch",
        fields: &[OBSERVED_SAMPLE, COUNT, RECORD_BYTES, RECORDS],
    };
}

pub(crate) mod telemetry_configuration {
    use super::*;
    pub(crate) const METER_HANDLES: FieldSpec = FieldSpec::req(1, Wire::PackedU32);
    pub(crate) const METER_PERIOD_BLOCKS: FieldSpec = FieldSpec::req(2, Wire::U32);
    pub(crate) const COUNTER_IDS: FieldSpec = FieldSpec::req(3, Wire::PackedU32);
    pub(crate) const COUNTER_PERIOD_BLOCKS: FieldSpec = FieldSpec::req(4, Wire::U32);
    pub(crate) const DIAGNOSTICS_ENABLED: FieldSpec = FieldSpec::req(5, Wire::Bool);
    pub(crate) const MINIMUM_DIAGNOSTIC_SEVERITY: FieldSpec = FieldSpec::req(6, Wire::U8);
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "TelemetryConfiguration",
        fields: &[
            METER_HANDLES,
            METER_PERIOD_BLOCKS,
            COUNTER_IDS,
            COUNTER_PERIOD_BLOCKS,
            DIAGNOSTICS_ENABLED,
            MINIMUM_DIAGNOSTIC_SEVERITY,
        ],
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
    pub(crate) const MAXIMUM_AUTOMATION_RECORDS: FieldSpec = FieldSpec::req(9, Wire::U16);
    pub(crate) const CONTROL_COMMAND_SLOTS: FieldSpec = FieldSpec::req(10, Wire::U64);
    pub(crate) const CONTROL_COMMAND_BYTES: FieldSpec = FieldSpec::req(11, Wire::U64);
    pub(crate) const AUTOMATION_BATCH_SLOTS: FieldSpec = FieldSpec::req(12, Wire::U64);
    pub(crate) const RELIABLE_RESPONSE_SLOTS: FieldSpec = FieldSpec::req(13, Wire::U64);
    pub(crate) const RELIABLE_EVENT_SLOTS: FieldSpec = FieldSpec::req(14, Wire::U64);
    pub(crate) const TELEMETRY_SLOTS: FieldSpec = FieldSpec::req(15, Wire::U64);
    pub(crate) const REPLAY_ENTRIES: FieldSpec = FieldSpec::req(16, Wire::U64);
    pub(crate) const REPLAY_BYTES: FieldSpec = FieldSpec::req(17, Wire::U64);
    pub(crate) const MAXIMUM_CACHED_RESPONSE_BYTES: FieldSpec = FieldSpec::req(18, Wire::U64);
    pub(crate) const PER_BLOCK_AUTOMATION_DENSITY: FieldSpec = FieldSpec::req(19, Wire::U64);
    pub(crate) const ADMISSION_QUANTUM_FRAMES: FieldSpec = FieldSpec::req(20, Wire::U64);
    pub(crate) const MAXIMUM_PARAMETER_PAGE_ITEMS: FieldSpec = FieldSpec::req(21, Wire::U16);
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
            MAXIMUM_AUTOMATION_RECORDS,
            CONTROL_COMMAND_SLOTS,
            CONTROL_COMMAND_BYTES,
            AUTOMATION_BATCH_SLOTS,
            RELIABLE_RESPONSE_SLOTS,
            RELIABLE_EVENT_SLOTS,
            TELEMETRY_SLOTS,
            REPLAY_ENTRIES,
            REPLAY_BYTES,
            MAXIMUM_CACHED_RESPONSE_BYTES,
            PER_BLOCK_AUTOMATION_DENSITY,
            ADMISSION_QUANTUM_FRAMES,
            MAXIMUM_PARAMETER_PAGE_ITEMS,
            MAXIMUM_DIAGNOSTIC_PAGE_ITEMS,
            MAXIMUM_TELEMETRY_HANDLES,
            MAXIMUM_TRANSACTION_EDITS,
            SUPPORTED_COMMANDS,
            SUPPORTED_EVENTS,
            FLAGS,
        ],
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrated_specs_are_sorted_unique_and_structurally_consistent() {
        let specs = [
            &capabilities_request::SPEC,
            &enum_choice::SPEC,
            &descriptor::SPEC,
            &snapshot_request::SPEC,
            &snapshot::SPEC,
            &transaction_applied::SPEC,
            &session_committed::SPEC,
            &metadata_request::SPEC,
            &metadata_page::SPEC,
            &state_request::SPEC,
            &state_page::SPEC,
            &automation_enqueued::SPEC,
            &automation_canceled::SPEC,
            &meter_batch::SPEC,
            &telemetry_configuration::SPEC,
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
        ];
        for spec in specs {
            assert!(!spec.name.is_empty());
            assert!(spec.fields.windows(2).all(|pair| pair[0].id < pair[1].id));
            for field in spec.fields {
                assert_ne!(field.id, 0);
                assert_eq!(field.wire == Wire::Message, field.nested.is_some());
                assert!(
                    !field.repeated
                        || field.wire == Wire::Message
                        || matches!(
                            field.wire,
                            Wire::PackedU16 | Wire::PackedU32 | Wire::PackedU64 | Wire::PackedF32
                        )
                );
            }
        }
        assert!(std::hint::black_box(descriptor::HANDLE).mandatory);
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
}
