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
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "SessionSnapshotRequest",
        fields: &[FieldSpec::req(1, Wire::U64), FieldSpec::req(2, Wire::U32)],
    };
}

pub(crate) mod snapshot {
    use super::*;
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "SessionSnapshot",
        fields: &[
            FieldSpec::req(1, Wire::U64),
            FieldSpec::req(2, Wire::U64),
            FieldSpec::req(3, Wire::Bytes),
            FieldSpec::req(4, Wire::Bool),
        ],
    };
}

pub(crate) mod transaction_applied {
    use super::*;
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "TransactionApplied",
        fields: &[FieldSpec::req(1, Wire::U32)],
    };
}

pub(crate) mod session_committed {
    use super::*;
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "SessionCommitted",
        fields: &[
            FieldSpec::req(1, Wire::U64),
            FieldSpec::req(2, Wire::U64),
            FieldSpec::req(3, Wire::U64),
            FieldSpec::req(4, Wire::U32),
        ],
    };
}

pub(crate) mod metadata_request {
    use super::*;
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ParameterMetadataRequest",
        fields: &[FieldSpec::req(1, Wire::U32), FieldSpec::req(2, Wire::U16)],
    };
}

pub(crate) mod metadata_page {
    use super::{descriptor, *};
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ParameterMetadataPage",
        fields: &[
            FieldSpec::req(1, Wire::U32),
            FieldSpec::req(2, Wire::Bool),
            FieldSpec::msg(3, true, true, &descriptor::SPEC),
        ],
    };
}

pub(crate) mod state_request {
    use super::*;
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ParameterStateRequest",
        fields: &[FieldSpec::req(1, Wire::PackedU32)],
    };
}

pub(crate) mod state_page {
    use super::*;
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "ParameterStatePage",
        fields: &[
            FieldSpec::req(1, Wire::U64),
            FieldSpec::req(2, Wire::U16),
            FieldSpec::req(3, Wire::U16),
            FieldSpec::req(4, Wire::Bytes),
        ],
    };
}

pub(crate) mod automation_enqueued {
    use super::*;
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "AutomationEnqueued",
        fields: &[
            FieldSpec::req(1, Wire::U16),
            FieldSpec::req(2, Wire::U64),
            FieldSpec::req(3, Wire::U64),
            FieldSpec::req(4, Wire::U64),
        ],
    };
}

pub(crate) mod automation_canceled {
    use super::*;
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "AutomationCanceled",
        fields: &[
            FieldSpec::req(1, Wire::U64),
            FieldSpec::req(2, Wire::U64),
            FieldSpec::req(3, Wire::U16),
            FieldSpec::req(4, Wire::U8),
            FieldSpec::req(5, Wire::U64),
            FieldSpec::opt(6, Wire::U64),
        ],
    };
}

pub(crate) mod meter_batch {
    use super::*;
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "MeterBatch",
        fields: &[
            FieldSpec::req(1, Wire::U64),
            FieldSpec::req(2, Wire::U16),
            FieldSpec::req(3, Wire::U16),
            FieldSpec::req(4, Wire::Bytes),
        ],
    };
}

pub(crate) mod telemetry_configuration {
    use super::*;
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "TelemetryConfiguration",
        fields: &[
            FieldSpec::req(1, Wire::PackedU32),
            FieldSpec::req(2, Wire::U32),
            FieldSpec::req(3, Wire::PackedU32),
            FieldSpec::req(4, Wire::U32),
            FieldSpec::req(5, Wire::Bool),
            FieldSpec::req(6, Wire::U8),
        ],
    };
}

pub(crate) mod counters_request {
    use super::*;
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "CountersRequest",
        fields: &[
            FieldSpec::req(1, Wire::Bool),
            FieldSpec::opt(2, Wire::PackedU32),
        ],
    };
}

pub(crate) mod counter_value {
    use super::*;
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "CounterValue",
        fields: &[FieldSpec::req(1, Wire::U32), FieldSpec::req(2, Wire::U64)],
    };
}

pub(crate) mod counter_snapshot {
    use super::{counter_value, *};
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "CounterSnapshot",
        fields: &[
            FieldSpec::req(1, Wire::U64),
            FieldSpec::msg(2, true, true, &counter_value::SPEC),
        ],
    };
}

pub(crate) mod diagnostics_request {
    use super::*;
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "DiagnosticsRequest",
        fields: &[
            FieldSpec::req(1, Wire::U64),
            FieldSpec::req(2, Wire::U16),
            FieldSpec::req(3, Wire::U8),
        ],
    };
}

pub(crate) mod path_segment {
    use super::*;
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "PathSegment",
        fields: &[
            FieldSpec::req(1, Wire::U8),
            FieldSpec::opt(2, Wire::Utf8),
            FieldSpec::opt(3, Wire::U64),
            FieldSpec::opt(4, Wire::Utf8),
        ],
    };
}

pub(crate) mod diagnostic {
    use super::{path_segment, *};
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "Diagnostic",
        fields: &[
            FieldSpec::req(1, Wire::Utf8),
            FieldSpec::req(2, Wire::U8),
            FieldSpec::msg(3, true, true, &path_segment::SPEC),
            FieldSpec::opt(4, Wire::Utf8),
            FieldSpec::opt(5, Wire::U32),
            FieldSpec::opt(6, Wire::U64),
            FieldSpec::opt(7, Wire::U64),
        ],
    };
}

pub(crate) mod backpressure {
    use super::*;
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "Backpressure",
        fields: &[
            FieldSpec::req(1, Wire::U8),
            FieldSpec::req(2, Wire::U64),
            FieldSpec::req(3, Wire::U64),
            FieldSpec::req(4, Wire::U16),
            FieldSpec::opt(5, Wire::U64),
            FieldSpec::opt(6, Wire::U64),
            FieldSpec::opt(7, Wire::U64),
            FieldSpec::opt(8, Wire::U64),
        ],
    };
}

pub(crate) mod non_ok {
    use super::{backpressure, diagnostic, *};
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "NonOkResponse",
        fields: &[
            FieldSpec::msg(1, true, true, &diagnostic::SPEC),
            FieldSpec::req(2, Wire::U32),
            FieldSpec::msg(3, false, false, &backpressure::SPEC),
        ],
    };
}

pub(crate) mod diagnostics_page {
    use super::{diagnostic, *};
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "DiagnosticsPage",
        fields: &[
            FieldSpec::req(1, Wire::U64),
            FieldSpec::req(2, Wire::Bool),
            FieldSpec::msg(3, true, true, &diagnostic::SPEC),
        ],
    };
}

pub(crate) mod diagnostic_event {
    use super::{diagnostic, *};
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "DiagnosticEvent",
        fields: &[FieldSpec::msg(1, true, false, &diagnostic::SPEC)],
    };
}

pub(crate) mod capabilities {
    use super::*;
    pub(crate) static SPEC: MessageSpec = MessageSpec {
        name: "Capabilities",
        fields: &[
            FieldSpec::req(1, Wire::U16),
            FieldSpec::req(2, Wire::U16),
            FieldSpec::req(3, Wire::U16),
            FieldSpec::req(4, Wire::U16),
            FieldSpec::req(5, Wire::U64),
            FieldSpec::req(6, Wire::U32),
            FieldSpec::req(7, Wire::U64),
            FieldSpec::req(8, Wire::U8),
            FieldSpec::req(9, Wire::U16),
            FieldSpec::req(10, Wire::U64),
            FieldSpec::req(11, Wire::U64),
            FieldSpec::req(12, Wire::U64),
            FieldSpec::req(13, Wire::U64),
            FieldSpec::req(14, Wire::U64),
            FieldSpec::req(15, Wire::U64),
            FieldSpec::req(16, Wire::U64),
            FieldSpec::req(17, Wire::U64),
            FieldSpec::req(18, Wire::U64),
            FieldSpec::req(19, Wire::U64),
            FieldSpec::req(20, Wire::U64),
            FieldSpec::req(21, Wire::U16),
            FieldSpec::req(22, Wire::U16),
            FieldSpec::req(23, Wire::U16),
            FieldSpec::req(24, Wire::U32),
            FieldSpec::req(25, Wire::PackedU16),
            FieldSpec::req(26, Wire::PackedU16),
            FieldSpec::req(27, Wire::U64),
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
