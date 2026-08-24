//! Fixed-work descriptive Issue-005 BTLV versus FlatBuffers comparison.
//!
//! This tool is intentionally outside the engine and protocol crates. It compares the frozen
//! logical corpus through the production BTLV codec with a small, schema-checked FlatBuffers
//! table (`protocol_benchmark.fbs`). FlatBuffers timing is a native host harness measurement;
//! it is not a browser or in-engine timing claim.

#![allow(unsafe_code)] // Isolated allocation instrumentation, identical in scope to protocol audit.
#![cfg_attr(target_arch = "wasm32", allow(dead_code))]

use miso_engine_bench_support::alloc as bench_alloc;
use miso_engine_bench_support::json::escape;
use std::{cell::Cell, env, fs, process::Command, time::Instant};

use miso_engine_protocol::{
    AutomationEnqueue, AutomationKind, AutomationRecord, Backpressure, BackpressureQueueKind,
    Capabilities, CapabilityFlags, CommandPayload, ConformanceDecoder, CounterId, CounterSnapshot,
    CounterValue, DecodeScratch, Diagnostic, DiagnosticSeverity, DiagnosticsPage, EventPayload,
    ExpectedRevision, MessageId, MeterBatch, MeterComponent, MeterRecord, NonOkResponse,
    ParameterAutomationRate, ParameterChannel, ParameterDescriptor, ParameterDomain,
    ParameterMapping, ParameterMetadataPage, ParameterRack, ParameterStatePage,
    ParameterStateRecord, ParameterUnit, ParameterValueKind, ProtocolCodec, ProtocolVersion,
    RequestId, SampleTime, SessionEditV1, SessionRevision, StatusCode, SuccessResponsePayload,
    TransactionApplied, TypedCommandFrame, TypedEventFrame, TypedNonOkResponseFrame,
    TypedSuccessResponseFrame,
};
use miso_engine_session::StableId;

/// Exact measured rounds. The executable rejects every other value.
const ROUNDS: u8 = 2;
const AUTOMATION_BATCHES: usize = 40;
const AUTOMATION_RECORDS: usize = 10_000;
const DESCRIPTORS: usize = 256;
const METERS: usize = 256;
const TRANSACTION_EDITS: usize = 64;
const MAX_FRAME_BYTES: usize = 65_536;
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
/// FNV-1a over each stable label and normalized logical record. Updated only with corpus changes.
pub(crate) const CORPUS_CHECKSUM: u64 = 0x9eee_4fcb_61be_3b9e;

// #104 F4: the arm/disarm pair reads the one audited allocator's totals instead of a private
// thread-local counter fed by a fourteenth copy of the `GlobalAlloc` wrapper. It counts the same
// events -- `alloc`, `alloc_zeroed` and `realloc`, with their requested byte counts.
std::thread_local! {
    static ALLOCATION_MARK: Cell<bench_alloc::Counters> = const {
        Cell::new(bench_alloc::Counters {
            allocations: 0,
            deallocations: 0,
            reallocations: 0,
            requested_bytes: 0,
        })
    };
}

fn arm_allocations() {
    ALLOCATION_MARK.with(|mark| mark.set(bench_alloc::counters()));
}

fn disarm_allocations() -> (u64, u64) {
    let moved = bench_alloc::delta_since(ALLOCATION_MARK.with(Cell::get));
    (moved.allocations, moved.requested_bytes)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Format {
    Btlv,
    FlatBuffers,
}

impl Format {
    const fn name(self) -> &'static str {
        match self {
            Self::Btlv => "btlv",
            Self::FlatBuffers => "flatbuffers",
        }
    }

    fn round_order(self, round: u8) -> u8 {
        match (round, self) {
            (1, Self::Btlv) | (2, Self::FlatBuffers) => 0,
            (1, Self::FlatBuffers) | (2, Self::Btlv) => 1,
            _ => unreachable!("rounds are fixed at two"),
        }
    }
}

#[derive(Clone, Copy)]
enum FrameDecoder {
    Command,
    Response,
    Event,
    Transaction,
}

impl From<ConformanceDecoder> for FrameDecoder {
    fn from(value: ConformanceDecoder) -> Self {
        match value {
            ConformanceDecoder::Command => Self::Command,
            ConformanceDecoder::Response => Self::Response,
            ConformanceDecoder::Event => Self::Event,
            ConformanceDecoder::Transaction => Self::Transaction,
        }
    }
}

struct WorkFrame {
    label: String,
    logical_kind: u32,
    request_id: u64,
    revision: u64,
    status: u32,
    automation_records: usize,
    decoder: FrameDecoder,
    btlv: BtlvSource,
    logical_values: Vec<LogicalValue>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
enum LogicalValueKind {
    Unsigned = 1,
    FloatBits = 2,
    Text = 3,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LogicalValue {
    field: u16,
    index: u32,
    subfield: u16,
    kind: LogicalValueKind,
    unsigned_value: u64,
    float_bits: u32,
    text_value: Option<String>,
}

fn logical_unsigned(field: u16, index: usize, subfield: u16, value: u64) -> LogicalValue {
    LogicalValue {
        field,
        index: u32::try_from(index).expect("bounded logical corpus index"),
        subfield,
        kind: LogicalValueKind::Unsigned,
        unsigned_value: value,
        float_bits: 0,
        text_value: None,
    }
}

fn logical_float(field: u16, index: usize, subfield: u16, value: f32) -> LogicalValue {
    LogicalValue {
        field,
        index: u32::try_from(index).expect("bounded logical corpus index"),
        subfield,
        kind: LogicalValueKind::FloatBits,
        unsigned_value: 0,
        float_bits: value.to_bits(),
        text_value: None,
    }
}

fn logical_text(field: u16, index: usize, subfield: u16, value: &str) -> LogicalValue {
    LogicalValue {
        field,
        index: u32::try_from(index).expect("bounded logical corpus index"),
        subfield,
        kind: LogicalValueKind::Text,
        unsigned_value: 0,
        float_bits: 0,
        text_value: Some(value.to_owned()),
    }
}

enum BtlvSource {
    CapabilitiesQuery,
    CapabilitiesResponse,
    Transaction(Vec<SessionEditV1>),
    Metadata(ParameterMetadataPage),
    State(ParameterStatePage),
    Automation(Vec<AutomationRecord>),
    Meters(Vec<MeterRecord>),
    Counters(CounterSnapshot),
    Diagnostics(DiagnosticsPage),
    Success,
    Error(StatusCode, NonOkResponse),
}

impl BtlvSource {
    fn prepare(&self) -> PreparedBtlv<'_> {
        let request = request_id(41);
        let revision = SessionRevision(9);
        match self {
            Self::CapabilitiesQuery => PreparedBtlv::Command(TypedCommandFrame {
                request_id: request,
                expected_revision: ExpectedRevision::Any,
                payload: CommandPayload::CapabilitiesGet,
            }),
            Self::CapabilitiesResponse => PreparedBtlv::Success(TypedSuccessResponseFrame {
                request_id: request,
                revision,
                payload: SuccessResponsePayload::Capabilities(capabilities()),
            }),
            Self::Transaction(edits) => PreparedBtlv::Command(TypedCommandFrame {
                request_id: request,
                expected_revision: ExpectedRevision::Exact(revision),
                payload: CommandPayload::SessionTransactionApply(edits),
            }),
            Self::Metadata(page) => PreparedBtlv::Success(TypedSuccessResponseFrame {
                request_id: request,
                revision,
                payload: SuccessResponsePayload::ParameterMetadata(page.clone()),
            }),
            Self::State(page) => PreparedBtlv::Success(TypedSuccessResponseFrame {
                request_id: request,
                revision,
                payload: SuccessResponsePayload::ParameterState(page.clone()),
            }),
            Self::Automation(records) => PreparedBtlv::Command(TypedCommandFrame {
                request_id: request,
                expected_revision: ExpectedRevision::Exact(revision),
                payload: CommandPayload::AutomationEnqueue(AutomationEnqueue { records }),
            }),
            Self::Meters(records) => PreparedBtlv::Event(TypedEventFrame {
                revision,
                payload: EventPayload::MeterBatch(MeterBatch {
                    observed_sample: SampleTime(960),
                    records,
                }),
            }),
            Self::Counters(page) => PreparedBtlv::Success(TypedSuccessResponseFrame {
                request_id: request,
                revision,
                payload: SuccessResponsePayload::CounterSnapshot(page.clone()),
            }),
            Self::Diagnostics(page) => PreparedBtlv::Success(TypedSuccessResponseFrame {
                request_id: request,
                revision,
                payload: SuccessResponsePayload::DiagnosticsPage(page.clone()),
            }),
            Self::Success => PreparedBtlv::Success(TypedSuccessResponseFrame {
                request_id: request,
                revision,
                payload: SuccessResponsePayload::SessionTransactionApplied(TransactionApplied {
                    applied_operations: TRANSACTION_EDITS as u32,
                }),
            }),
            Self::Error(status, payload) => PreparedBtlv::Error(TypedNonOkResponseFrame {
                request_id: request,
                revision,
                message_id: MessageId::SessionTransactionApply,
                status: *status,
                payload,
            }),
        }
    }
}

enum PreparedBtlv<'a> {
    Command(TypedCommandFrame<'a>),
    Success(TypedSuccessResponseFrame<'a>),
    Error(TypedNonOkResponseFrame<'a>),
    Event(TypedEventFrame<'a>),
}

impl PreparedBtlv<'_> {
    fn encode_into(&self, codec: &ProtocolCodec, output: &mut [u8]) -> usize {
        match self {
            Self::Command(frame) => codec
                .encode_command_frame_into(frame, output)
                .expect("frozen command"),
            Self::Success(frame) => codec
                .encode_success_response_frame_into(frame, output)
                .expect("frozen success response"),
            Self::Error(frame) => codec
                .encode_non_ok_response_frame_into(frame, output)
                .expect("frozen error response"),
            Self::Event(frame) => codec
                .encode_event_frame_into(frame, output)
                .expect("frozen event"),
        }
    }
}

fn request_id(value: u64) -> RequestId {
    RequestId::new(value).expect("benchmark request ID is nonzero")
}

fn capabilities() -> Capabilities<'static> {
    const COMMANDS: &[u16] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    const EVENTS: &[u16] = &[0x8001, 0x8002, 0x8010, 0x8020, 0x8021, 0x8030];
    Capabilities {
        minimum_version: ProtocolVersion::V1,
        maximum_version: ProtocolVersion::V1,
        maximum_frame_bytes: MAX_FRAME_BYTES as u64,
        maximum_tlvs: 1024,
        maximum_string_bytes: 1024,
        maximum_nesting: 4,
        maximum_automation_records: 256,
        control_command_slots: 64,
        control_command_bytes: MAX_FRAME_BYTES as u64,
        automation_batch_slots: 64,
        reliable_response_slots: 64,
        reliable_event_slots: 64,
        telemetry_slots: 64,
        replay_entries: 64,
        replay_bytes: MAX_FRAME_BYTES as u64,
        maximum_cached_response_bytes: MAX_FRAME_BYTES as u64,
        per_block_automation_density: 256,
        admission_quantum_frames: 128,
        maximum_parameter_page_items: 256,
        maximum_diagnostic_page_items: 256,
        maximum_telemetry_handles: 256,
        maximum_transaction_edits: TRANSACTION_EDITS as u32,
        supported_commands: COMMANDS,
        supported_events: EVENTS,
        flags: CapabilityFlags::B4_BASE,
    }
}

fn corpus() -> Vec<WorkFrame> {
    let mut frames = Vec::with_capacity(54);
    push_frame(
        &mut frames,
        "capabilities.query",
        1,
        41,
        0,
        0,
        0,
        FrameDecoder::Command,
        BtlvSource::CapabilitiesQuery,
    );
    push_frame(
        &mut frames,
        "capabilities.response",
        2,
        41,
        9,
        0,
        0,
        FrameDecoder::Response,
        BtlvSource::CapabilitiesResponse,
    );

    let edits = (0..TRANSACTION_EDITS)
        .map(|index| SessionEditV1::SetSessionId {
            session_id: StableId::parse(&format!("benchmark-edit-{index:02}"))
                .expect("generated stable ID"),
        })
        .collect();
    push_frame(
        &mut frames,
        "transaction.64_operations",
        3,
        41,
        9,
        0,
        0,
        FrameDecoder::Transaction,
        BtlvSource::Transaction(edits),
    );

    let mut descriptors = (0..DESCRIPTORS)
        .map(|index| ParameterDescriptor {
            handle: (index + 1) as u32,
            track_id: format!("track-{index:03}"),
            rack: ParameterRack::Dynamic,
            effect_id: "compressor".to_owned(),
            parameter_id: index as u32,
            channel: ParameterChannel::Left,
            value_kind: ParameterValueKind::F32,
            unit: ParameterUnit::Db,
            domain: ParameterDomain::Continuous,
            minimum: Some(-60.0),
            maximum: Some(12.0),
            default: 0.0,
            mapping: ParameterMapping::Linear,
            automation_rate: ParameterAutomationRate::Sample,
            smoothing_samples: 64,
            flags: 0,
            display_name: Some(format!("Parameter {index:03}")),
            display_unit: Some("dB".to_owned()),
            enum_choices: Vec::new(),
        })
        .collect::<Vec<_>>();
    let second_descriptor_page = descriptors.split_off(128);
    push_frame(
        &mut frames,
        "parameters.descriptors.page.1",
        4,
        41,
        9,
        0,
        0,
        FrameDecoder::Response,
        BtlvSource::Metadata(ParameterMetadataPage {
            last_handle: 128,
            eof: false,
            descriptors,
        }),
    );
    push_frame(
        &mut frames,
        "parameters.descriptors.page.2",
        4,
        41,
        9,
        0,
        0,
        FrameDecoder::Response,
        BtlvSource::Metadata(ParameterMetadataPage {
            last_handle: DESCRIPTORS as u32,
            eof: true,
            descriptors: second_descriptor_page,
        }),
    );
    let state_records = (0..DESCRIPTORS)
        .map(|index| ParameterStateRecord {
            handle: (index + 1) as u32,
            flags: 1,
            value: index as f32 / DESCRIPTORS as f32,
        })
        .collect::<Vec<_>>();
    push_frame(
        &mut frames,
        "parameters.state.page.1",
        5,
        41,
        9,
        0,
        0,
        FrameDecoder::Response,
        BtlvSource::State(ParameterStatePage {
            observed_sample: 960,
            records: state_records,
        }),
    );
    let second_state_page = match frames.last_mut() {
        Some(WorkFrame {
            btlv: BtlvSource::State(page),
            ..
        }) => page.records.split_off(128),
        _ => unreachable!("first state page was just inserted"),
    };
    push_frame(
        &mut frames,
        "parameters.state.page.2",
        5,
        41,
        9,
        0,
        0,
        FrameDecoder::Response,
        BtlvSource::State(ParameterStatePage {
            observed_sample: 960,
            records: second_state_page,
        }),
    );

    let mut sample = 0_u64;
    for batch in 0..AUTOMATION_BATCHES {
        let count = if batch + 1 == AUTOMATION_BATCHES {
            16
        } else {
            256
        };
        let records = (0..count)
            .map(|_| {
                let record = AutomationRecord {
                    kind: AutomationKind::Point,
                    handle: miso_engine_protocol::ParameterHandle(1),
                    start: SampleTime(sample),
                    end: SampleTime(sample),
                    start_value: sample as f32 * 0.001,
                    end_value: sample as f32 * 0.001,
                };
                sample += 1;
                record
            })
            .collect();
        let label = format!("automation.batch.{batch:02}");
        push_frame(
            &mut frames,
            label,
            6,
            41,
            9,
            0,
            count,
            FrameDecoder::Command,
            BtlvSource::Automation(records),
        );
    }
    assert_eq!(
        sample as usize, AUTOMATION_RECORDS,
        "exact automation corpus"
    );

    let meters = (0..METERS)
        .map(|index| MeterRecord {
            handle: (index + 1) as u32,
            component: MeterComponent::Left,
            flags: 1,
            value: -0.1 * index as f32,
        })
        .collect();
    push_frame(
        &mut frames,
        "meters.256",
        7,
        0,
        9,
        0,
        0,
        FrameDecoder::Event,
        BtlvSource::Meters(meters),
    );
    let counters = (1..=15)
        .map(|raw| CounterValue {
            id: counter_id(raw),
            value: raw as u64 * 100,
        })
        .collect();
    push_frame(
        &mut frames,
        "counters.page",
        8,
        41,
        9,
        0,
        0,
        FrameDecoder::Response,
        BtlvSource::Counters(CounterSnapshot {
            observed_sample: SampleTime(960),
            values: counters,
        }),
    );
    let diagnostics = (0..16)
        .map(|index| Diagnostic {
            code: "protocol.benchmark".to_owned(),
            severity: DiagnosticSeverity::Warning,
            path: Vec::new(),
            detail: Some(format!("diagnostic-{index}")),
            operation_index: Some(index),
            sample_time: Some(960 + u64::from(index)),
            provider_sequence: Some(u64::from(index) + 1),
        })
        .collect();
    push_frame(
        &mut frames,
        "diagnostics.page",
        9,
        41,
        9,
        0,
        0,
        FrameDecoder::Response,
        BtlvSource::Diagnostics(DiagnosticsPage {
            last_sequence: 16,
            eof: true,
            diagnostics,
        }),
    );
    push_frame(
        &mut frames,
        "status.success",
        10,
        41,
        9,
        StatusCode::Ok as u32,
        0,
        FrameDecoder::Response,
        BtlvSource::Success,
    );
    for (label, status) in [
        ("status.conflict", StatusCode::RevisionConflict),
        ("status.validation", StatusCode::ValidationFailed),
        ("status.backpressure", StatusCode::Backpressure),
    ] {
        let backpressure = (status == StatusCode::Backpressure).then_some(Backpressure {
            queue_kind: BackpressureQueueKind::ControlCommand,
            capacity: 64,
            occupancy: 64,
            requested_items: 1,
            generation: Some(1),
            retry_boundary: Some(1_088),
            requested_bytes: Some(128),
            available_bytes: Some(0),
        });
        push_frame(
            &mut frames,
            label,
            11,
            41,
            9,
            status as u32,
            0,
            FrameDecoder::Response,
            BtlvSource::Error(
                status,
                NonOkResponse {
                    diagnostics: Vec::new(),
                    omitted_diagnostics: 0,
                    backpressure,
                },
            ),
        );
    }
    assert_eq!(frames.len(), 54, "frozen frame count");
    frames
}

fn counter_id(raw: u32) -> CounterId {
    match raw {
        1 => CounterId::ControlCommandBackpressure,
        2 => CounterId::AutomationBackpressure,
        3 => CounterId::ReliableResponseBackpressure,
        4 => CounterId::ReliableEventBackpressure,
        5 => CounterId::TelemetryCoalesced,
        6 => CounterId::TelemetryDropped,
        7 => CounterId::MalformedFrames,
        8 => CounterId::ReplayHits,
        9 => CounterId::RequestIdReuse,
        10 => CounterId::ReplayExpired,
        11 => CounterId::LateAutomation,
        12 => CounterId::CanceledAutomation,
        13 => CounterId::AutomationTimePast,
        14 => CounterId::AutomationOrderReject,
        15 => CounterId::ValidationFailures,
        _ => unreachable!("frozen counter range"),
    }
}

#[allow(clippy::too_many_arguments)]
fn push_frame(
    frames: &mut Vec<WorkFrame>,
    label: impl Into<String>,
    logical_kind: u32,
    request_id: u64,
    revision: u64,
    status: u32,
    automation_records: usize,
    decoder: FrameDecoder,
    btlv: BtlvSource,
) {
    let label = label.into();
    let logical_values = logical_values(&btlv);
    frames.push(WorkFrame {
        label,
        logical_kind,
        request_id,
        revision,
        status,
        automation_records,
        decoder,
        btlv,
        logical_values,
    });
}

fn logical_values(source: &BtlvSource) -> Vec<LogicalValue> {
    let mut values = Vec::new();
    match source {
        BtlvSource::CapabilitiesQuery => {}
        BtlvSource::CapabilitiesResponse => {
            let value = capabilities();
            values.extend([
                logical_unsigned(1, 0, 0, u64::from(value.minimum_version.major)),
                logical_unsigned(2, 0, 0, u64::from(value.minimum_version.minor)),
                logical_unsigned(3, 0, 0, u64::from(value.maximum_version.major)),
                logical_unsigned(4, 0, 0, u64::from(value.maximum_version.minor)),
                logical_unsigned(5, 0, 0, value.maximum_frame_bytes),
                logical_unsigned(6, 0, 0, u64::from(value.maximum_tlvs)),
                logical_unsigned(7, 0, 0, value.maximum_string_bytes),
                logical_unsigned(8, 0, 0, u64::from(value.maximum_nesting)),
                logical_unsigned(9, 0, 0, u64::from(value.maximum_automation_records)),
                logical_unsigned(10, 0, 0, value.control_command_slots),
                logical_unsigned(11, 0, 0, value.control_command_bytes),
                logical_unsigned(12, 0, 0, value.automation_batch_slots),
                logical_unsigned(13, 0, 0, value.reliable_response_slots),
                logical_unsigned(14, 0, 0, value.reliable_event_slots),
                logical_unsigned(15, 0, 0, value.telemetry_slots),
                logical_unsigned(16, 0, 0, value.replay_entries),
                logical_unsigned(17, 0, 0, value.replay_bytes),
                logical_unsigned(18, 0, 0, value.maximum_cached_response_bytes),
                logical_unsigned(19, 0, 0, value.per_block_automation_density),
                logical_unsigned(20, 0, 0, value.admission_quantum_frames),
                logical_unsigned(21, 0, 0, u64::from(value.maximum_parameter_page_items)),
                logical_unsigned(22, 0, 0, u64::from(value.maximum_diagnostic_page_items)),
                logical_unsigned(23, 0, 0, u64::from(value.maximum_telemetry_handles)),
                logical_unsigned(24, 0, 0, u64::from(value.maximum_transaction_edits)),
                logical_unsigned(27, 0, 0, value.flags.0),
            ]);
            for (index, command) in value.supported_commands.iter().copied().enumerate() {
                values.push(logical_unsigned(25, index, 0, u64::from(command)));
            }
            for (index, event) in value.supported_events.iter().copied().enumerate() {
                values.push(logical_unsigned(26, index, 0, u64::from(event)));
            }
        }
        BtlvSource::Transaction(edits) => {
            for (index, edit) in edits.iter().enumerate() {
                values.push(logical_unsigned(
                    1,
                    index,
                    1,
                    u64::from(edit.opcode().raw()),
                ));
                let SessionEditV1::SetSessionId { session_id } = edit else {
                    panic!("frozen comparison transaction uses SetSessionId operations");
                };
                values.push(logical_text(1, index, 2, session_id.as_str()));
            }
        }
        BtlvSource::Metadata(page) => {
            values.push(logical_unsigned(1, 0, 0, u64::from(page.last_handle)));
            values.push(logical_unsigned(2, 0, 0, u64::from(page.eof)));
            for (index, descriptor) in page.descriptors.iter().enumerate() {
                values.extend([
                    logical_unsigned(3, index, 1, u64::from(descriptor.handle)),
                    logical_text(3, index, 2, &descriptor.track_id),
                    logical_unsigned(3, index, 3, descriptor.rack as u64),
                    logical_text(3, index, 4, &descriptor.effect_id),
                    logical_unsigned(3, index, 5, u64::from(descriptor.parameter_id)),
                    logical_unsigned(3, index, 6, descriptor.channel as u64),
                    logical_unsigned(3, index, 7, descriptor.value_kind as u64),
                    logical_unsigned(3, index, 8, descriptor.unit as u64),
                    logical_unsigned(3, index, 9, descriptor.domain as u64),
                    logical_float(3, index, 12, descriptor.default),
                    logical_unsigned(3, index, 13, descriptor.mapping as u64),
                    logical_unsigned(3, index, 14, descriptor.automation_rate as u64),
                    logical_unsigned(3, index, 15, u64::from(descriptor.smoothing_samples)),
                    logical_unsigned(3, index, 16, u64::from(descriptor.flags)),
                ]);
                if let Some(minimum) = descriptor.minimum {
                    values.push(logical_float(3, index, 10, minimum));
                }
                if let Some(maximum) = descriptor.maximum {
                    values.push(logical_float(3, index, 11, maximum));
                }
                if let Some(display_name) = &descriptor.display_name {
                    values.push(logical_text(3, index, 17, display_name));
                }
                if let Some(display_unit) = &descriptor.display_unit {
                    values.push(logical_text(3, index, 18, display_unit));
                }
                for (choice, enum_choice) in descriptor.enum_choices.iter().enumerate() {
                    values.push(logical_float(3, index, 19, enum_choice.value));
                    values.push(logical_text(
                        3,
                        index,
                        u16::try_from(20 + choice).expect("bounded enum choices"),
                        &enum_choice.label,
                    ));
                }
            }
        }
        BtlvSource::State(page) => {
            values.push(logical_unsigned(1, 0, 0, page.observed_sample));
            for (index, record) in page.records.iter().enumerate() {
                values.extend([
                    logical_unsigned(2, index, 1, u64::from(record.handle)),
                    logical_unsigned(2, index, 2, u64::from(record.flags)),
                    logical_float(2, index, 3, record.value),
                ]);
            }
        }
        BtlvSource::Automation(records) => {
            for (index, record) in records.iter().enumerate() {
                values.extend([
                    logical_unsigned(1, index, 1, record.kind as u64),
                    logical_unsigned(1, index, 2, u64::from(record.handle.0)),
                    logical_unsigned(1, index, 3, record.start.0),
                    logical_unsigned(1, index, 4, record.end.0),
                    logical_float(1, index, 5, record.start_value),
                    logical_float(1, index, 6, record.end_value),
                ]);
            }
        }
        BtlvSource::Meters(records) => {
            values.push(logical_unsigned(1, 0, 0, 960));
            for (index, record) in records.iter().enumerate() {
                values.extend([
                    logical_unsigned(2, index, 1, u64::from(record.handle)),
                    logical_unsigned(2, index, 2, record.component as u64),
                    logical_unsigned(2, index, 3, u64::from(record.flags)),
                    logical_float(2, index, 4, record.value),
                ]);
            }
        }
        BtlvSource::Counters(page) => {
            values.push(logical_unsigned(1, 0, 0, page.observed_sample.0));
            for (index, counter) in page.values.iter().enumerate() {
                values.push(logical_unsigned(2, index, 1, counter.id as u64));
                values.push(logical_unsigned(2, index, 2, counter.value));
            }
        }
        BtlvSource::Diagnostics(page) => {
            values.push(logical_unsigned(1, 0, 0, page.last_sequence));
            values.push(logical_unsigned(2, 0, 0, u64::from(page.eof)));
            for (index, diagnostic) in page.diagnostics.iter().enumerate() {
                assert!(
                    diagnostic.path.is_empty(),
                    "frozen diagnostic paths are root"
                );
                values.push(logical_text(3, index, 1, &diagnostic.code));
                values.push(logical_unsigned(3, index, 2, diagnostic.severity as u64));
                if let Some(detail) = &diagnostic.detail {
                    values.push(logical_text(3, index, 4, detail));
                }
                if let Some(operation_index) = diagnostic.operation_index {
                    values.push(logical_unsigned(3, index, 5, u64::from(operation_index)));
                }
                if let Some(sample_time) = diagnostic.sample_time {
                    values.push(logical_unsigned(3, index, 6, sample_time));
                }
                if let Some(provider_sequence) = diagnostic.provider_sequence {
                    values.push(logical_unsigned(3, index, 7, provider_sequence));
                }
            }
        }
        BtlvSource::Success => {
            values.push(logical_unsigned(1, 0, 0, TRANSACTION_EDITS as u64));
        }
        BtlvSource::Error(_, payload) => {
            for (index, diagnostic) in payload.diagnostics.iter().enumerate() {
                values.push(logical_text(1, index, 1, &diagnostic.code));
                values.push(logical_unsigned(1, index, 2, diagnostic.severity as u64));
            }
            values.push(logical_unsigned(
                2,
                0,
                0,
                u64::from(payload.omitted_diagnostics),
            ));
            if let Some(backpressure) = payload.backpressure {
                values.extend([
                    logical_unsigned(3, 0, 1, backpressure.queue_kind as u64),
                    logical_unsigned(3, 0, 2, backpressure.capacity),
                    logical_unsigned(3, 0, 3, backpressure.occupancy),
                    logical_unsigned(3, 0, 4, u64::from(backpressure.requested_items)),
                ]);
                for (subfield, optional) in [
                    (5, backpressure.generation),
                    (6, backpressure.retry_boundary),
                    (7, backpressure.requested_bytes),
                    (8, backpressure.available_bytes),
                ] {
                    if let Some(value) = optional {
                        values.push(logical_unsigned(3, 0, subfield, value));
                    }
                }
            }
        }
    }
    values
}

/// Return the deterministic checksum consumed by native tests and scalar/SIMD Wasm golden builds.
#[must_use]
pub(crate) fn corpus_checksum() -> u64 {
    let mut hash = FNV_OFFSET;
    for frame in corpus() {
        hash = hash_bytes(hash, frame.label.as_bytes());
        hash = hash_bytes(hash, &frame.logical_kind.to_le_bytes());
        hash = hash_bytes(hash, &frame.request_id.to_le_bytes());
        hash = hash_bytes(hash, &frame.revision.to_le_bytes());
        hash = hash_bytes(hash, &frame.status.to_le_bytes());
        hash = hash_bytes(
            hash,
            &u32::try_from(frame.automation_records)
                .expect("bounded automation count")
                .to_le_bytes(),
        );
        for value in &frame.logical_values {
            hash = hash_bytes(hash, &value.field.to_le_bytes());
            hash = hash_bytes(hash, &value.index.to_le_bytes());
            hash = hash_bytes(hash, &value.subfield.to_le_bytes());
            hash = hash_bytes(hash, &[value.kind as u8]);
            hash = hash_bytes(hash, &value.unsigned_value.to_le_bytes());
            hash = hash_bytes(hash, &value.float_bits.to_le_bytes());
            if let Some(text) = &value.text_value {
                hash = hash_bytes(hash, text.as_bytes());
            }
        }
    }
    hash
}

fn hash_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

// Generated-like bindings for `protocol_benchmark.fbs`, intentionally kept local to the
// comparison tool. The builder and verifier are the official `flatbuffers` 25.12.19 runtime.
const VT_KIND: flatbuffers::VOffsetT = 4;
const VT_REQUEST_ID: flatbuffers::VOffsetT = 6;
const VT_REVISION: flatbuffers::VOffsetT = 8;
const VT_STATUS: flatbuffers::VOffsetT = 10;
const VT_AUTOMATION_RECORDS: flatbuffers::VOffsetT = 12;
const VT_UNSIGNED_KEYS: flatbuffers::VOffsetT = 14;
const VT_UNSIGNED_VALUES: flatbuffers::VOffsetT = 16;
const VT_FLOAT_KEYS: flatbuffers::VOffsetT = 18;
const VT_FLOAT_BITS: flatbuffers::VOffsetT = 20;
const VT_TEXT_KEYS: flatbuffers::VOffsetT = 22;
const VT_TEXT_OFFSETS: flatbuffers::VOffsetT = 24;
const VT_TEXT_LENGTHS: flatbuffers::VOffsetT = 26;
const VT_TEXT_UTF8: flatbuffers::VOffsetT = 28;

struct FlatbufferScratch {
    unsigned_keys: Vec<u64>,
    unsigned_values: Vec<u64>,
    float_keys: Vec<u64>,
    float_bits: Vec<u32>,
    text_keys: Vec<u64>,
    text_offsets: Vec<u32>,
    text_lengths: Vec<u32>,
    text_utf8: Vec<u8>,
}

impl FlatbufferScratch {
    fn prepare(maximum_values: usize, maximum_text_bytes: usize) -> Self {
        Self {
            unsigned_keys: Vec::with_capacity(maximum_values),
            unsigned_values: Vec::with_capacity(maximum_values),
            float_keys: Vec::with_capacity(maximum_values),
            float_bits: Vec::with_capacity(maximum_values),
            text_keys: Vec::with_capacity(maximum_values),
            text_offsets: Vec::with_capacity(maximum_values),
            text_lengths: Vec::with_capacity(maximum_values),
            text_utf8: Vec::with_capacity(maximum_text_bytes),
        }
    }

    fn load(&mut self, frame: &WorkFrame) {
        self.unsigned_keys.clear();
        self.unsigned_values.clear();
        self.float_keys.clear();
        self.float_bits.clear();
        self.text_keys.clear();
        self.text_offsets.clear();
        self.text_lengths.clear();
        self.text_utf8.clear();
        for value in &frame.logical_values {
            let key = logical_key(value);
            match value.kind {
                LogicalValueKind::Unsigned => {
                    self.unsigned_keys.push(key);
                    self.unsigned_values.push(value.unsigned_value);
                }
                LogicalValueKind::FloatBits => {
                    self.float_keys.push(key);
                    self.float_bits.push(value.float_bits);
                }
                LogicalValueKind::Text => {
                    let text = value
                        .text_value
                        .as_deref()
                        .expect("typed text logical value");
                    self.text_keys.push(key);
                    self.text_offsets.push(
                        u32::try_from(self.text_utf8.len()).expect("bounded FlatBuffers text pool"),
                    );
                    self.text_lengths.push(
                        u32::try_from(text.len()).expect("bounded FlatBuffers logical string"),
                    );
                    self.text_utf8.extend_from_slice(text.as_bytes());
                }
            }
        }
    }

    fn prepared_bytes(&self) -> usize {
        self.unsigned_keys.capacity() * std::mem::size_of::<u64>()
            + self.unsigned_values.capacity() * std::mem::size_of::<u64>()
            + self.float_keys.capacity() * std::mem::size_of::<u64>()
            + self.float_bits.capacity() * std::mem::size_of::<u32>()
            + self.text_keys.capacity() * std::mem::size_of::<u64>()
            + self.text_offsets.capacity() * std::mem::size_of::<u32>()
            + self.text_lengths.capacity() * std::mem::size_of::<u32>()
            + self.text_utf8.capacity()
    }
}

fn logical_key(value: &LogicalValue) -> u64 {
    u64::from(value.field) | (u64::from(value.subfield) << 16) | (u64::from(value.index) << 32)
}

#[derive(Clone, Copy)]
struct WireFrame<'a> {
    table: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for WireFrame<'a> {
    type Inner = Self;

    unsafe fn follow(buffer: &'a [u8], location: usize) -> Self::Inner {
        // SAFETY: `flatbuffers::root_with_opts` invokes this only after this type's verifier.
        // SAFETY: the trait's safety contract guarantees a valid table at this location.
        let table = unsafe { flatbuffers::Table::new(buffer, location) };
        Self { table }
    }
}

impl flatbuffers::Verifiable for WireFrame<'_> {
    fn run_verifier(
        verifier: &mut flatbuffers::Verifier,
        position: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        verifier
            .visit_table(position)?
            .visit_field::<u32>("kind", VT_KIND, true)?
            .visit_field::<u64>("request_id", VT_REQUEST_ID, true)?
            .visit_field::<u64>("revision", VT_REVISION, true)?
            .visit_field::<u32>("status", VT_STATUS, true)?
            .visit_field::<u32>("automation_records", VT_AUTOMATION_RECORDS, true)?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u64>>>(
                "unsigned_keys",
                VT_UNSIGNED_KEYS,
                true,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u64>>>(
                "unsigned_values",
                VT_UNSIGNED_VALUES,
                true,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u64>>>(
                "float_keys",
                VT_FLOAT_KEYS,
                true,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u32>>>(
                "float_bits",
                VT_FLOAT_BITS,
                true,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u64>>>(
                "text_keys",
                VT_TEXT_KEYS,
                true,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u32>>>(
                "text_offsets",
                VT_TEXT_OFFSETS,
                true,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u32>>>(
                "text_lengths",
                VT_TEXT_LENGTHS,
                true,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                "text_utf8",
                VT_TEXT_UTF8,
                true,
            )?
            .finish();
        Ok(())
    }
}

fn flatbuffer_encode(
    frame: &WorkFrame,
    builder: &mut flatbuffers::FlatBufferBuilder<'_>,
    scratch: &mut FlatbufferScratch,
) -> usize {
    builder.reset();
    scratch.load(frame);
    let unsigned_keys = builder.create_vector(&scratch.unsigned_keys);
    let unsigned_values = builder.create_vector(&scratch.unsigned_values);
    let float_keys = builder.create_vector(&scratch.float_keys);
    let float_bits = builder.create_vector(&scratch.float_bits);
    let text_keys = builder.create_vector(&scratch.text_keys);
    let text_offsets = builder.create_vector(&scratch.text_offsets);
    let text_lengths = builder.create_vector(&scratch.text_lengths);
    let text_utf8 = builder.create_vector(&scratch.text_utf8);
    let table = builder.start_table();
    builder.push_slot_always(VT_KIND, frame.logical_kind);
    builder.push_slot_always(VT_REQUEST_ID, frame.request_id);
    builder.push_slot_always(VT_REVISION, frame.revision);
    builder.push_slot_always(VT_STATUS, frame.status);
    builder.push_slot_always(
        VT_AUTOMATION_RECORDS,
        u32::try_from(frame.automation_records).expect("bounded automation count"),
    );
    builder.push_slot_always(VT_UNSIGNED_KEYS, unsigned_keys);
    builder.push_slot_always(VT_UNSIGNED_VALUES, unsigned_values);
    builder.push_slot_always(VT_FLOAT_KEYS, float_keys);
    builder.push_slot_always(VT_FLOAT_BITS, float_bits);
    builder.push_slot_always(VT_TEXT_KEYS, text_keys);
    builder.push_slot_always(VT_TEXT_OFFSETS, text_offsets);
    builder.push_slot_always(VT_TEXT_LENGTHS, text_lengths);
    builder.push_slot_always(VT_TEXT_UTF8, text_utf8);
    let table = builder.end_table(table);
    builder.finish(table, Some("MPCB"));
    builder.finished_data().len()
}

fn flatbuffer_verify(frame: &WorkFrame, input: &[u8]) -> Result<(), &'static str> {
    if input.len() > MAX_FRAME_BYTES
        || input.len() < 8
        || !flatbuffers::buffer_has_identifier(input, "MPCB", false)
    {
        return Err("invalid FlatBuffers envelope");
    }
    let options = flatbuffers::VerifierOptions {
        max_depth: 2,
        max_tables: 1,
        max_apparent_size: MAX_FRAME_BYTES,
        ignore_missing_null_terminator: false,
    };
    let root = flatbuffers::root_with_opts::<WireFrame<'_>>(&options, input)
        .map_err(|_| "FlatBuffers verifier rejection")?;
    // SAFETY: every access below follows the exact slot types verified by `root_with_opts`.
    let kind = unsafe { root.table.get::<u32>(VT_KIND, Some(0)) }.unwrap_or(0);
    // SAFETY: exact verified schema slot.
    let request_id = unsafe { root.table.get::<u64>(VT_REQUEST_ID, Some(0)) }.unwrap_or(0);
    // SAFETY: exact verified schema slot.
    let revision = unsafe { root.table.get::<u64>(VT_REVISION, Some(0)) }.unwrap_or(0);
    // SAFETY: exact verified schema slot.
    let status = unsafe { root.table.get::<u32>(VT_STATUS, Some(0)) }.unwrap_or(0);
    // SAFETY: exact verified schema slot.
    let automation_records =
        unsafe { root.table.get::<u32>(VT_AUTOMATION_RECORDS, Some(0)) }.unwrap_or(0);
    if kind != frame.logical_kind
        || request_id != frame.request_id
        || revision != frame.revision
        || status != frame.status
        || automation_records
            != u32::try_from(frame.automation_records).map_err(|_| "automation count overflow")?
    {
        return Err("FlatBuffers logical header mismatch");
    }
    macro_rules! vector {
        ($type:ty, $slot:expr, $missing:literal) => {{
            // SAFETY: the exact vector element type and slot were verified above.
            unsafe {
                root.table
                    .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, $type>>>(
                        $slot, None,
                    )
            }
            .ok_or($missing)?
        }};
    }
    let unsigned_keys = vector!(u64, VT_UNSIGNED_KEYS, "missing unsigned keys");
    let unsigned_values = vector!(u64, VT_UNSIGNED_VALUES, "missing unsigned values");
    let float_keys = vector!(u64, VT_FLOAT_KEYS, "missing float keys");
    let float_bits = vector!(u32, VT_FLOAT_BITS, "missing float bits");
    let text_keys = vector!(u64, VT_TEXT_KEYS, "missing text keys");
    let text_offsets = vector!(u32, VT_TEXT_OFFSETS, "missing text offsets");
    let text_lengths = vector!(u32, VT_TEXT_LENGTHS, "missing text lengths");
    let text_utf8 = vector!(u8, VT_TEXT_UTF8, "missing text UTF-8 pool");
    if unsigned_keys.len() != unsigned_values.len()
        || float_keys.len() != float_bits.len()
        || text_keys.len() != text_offsets.len()
        || text_keys.len() != text_lengths.len()
    {
        return Err("FlatBuffers parallel semantic vector mismatch");
    }
    let (mut unsigned_index, mut float_index, mut text_index) = (0_usize, 0_usize, 0_usize);
    for expected in &frame.logical_values {
        let key = logical_key(expected);
        match expected.kind {
            LogicalValueKind::Unsigned => {
                if unsigned_keys.get(unsigned_index) != key
                    || unsigned_values.get(unsigned_index) != expected.unsigned_value
                {
                    return Err("FlatBuffers unsigned logical value mismatch");
                }
                unsigned_index += 1;
            }
            LogicalValueKind::FloatBits => {
                if float_keys.get(float_index) != key
                    || float_bits.get(float_index) != expected.float_bits
                {
                    return Err("FlatBuffers float logical value mismatch");
                }
                float_index += 1;
            }
            LogicalValueKind::Text => {
                let offset = usize::try_from(text_offsets.get(text_index))
                    .map_err(|_| "FlatBuffers text offset overflow")?;
                let length = usize::try_from(text_lengths.get(text_index))
                    .map_err(|_| "FlatBuffers text length overflow")?;
                let end = offset
                    .checked_add(length)
                    .ok_or("FlatBuffers text range overflow")?;
                let encoded = text_utf8
                    .bytes()
                    .get(offset..end)
                    .ok_or("FlatBuffers text range invalid")?;
                if text_keys.get(text_index) != key
                    || std::str::from_utf8(encoded).map_err(|_| "FlatBuffers text is not UTF-8")?
                        != expected
                            .text_value
                            .as_deref()
                            .expect("typed text logical value")
                {
                    return Err("FlatBuffers text logical value mismatch");
                }
                text_index += 1;
            }
        }
    }
    if unsigned_index != unsigned_keys.len()
        || float_index != float_keys.len()
        || text_index != text_keys.len()
    {
        return Err("FlatBuffers semantic vector had trailing values");
    }
    Ok(())
}

fn decode_btlv(
    codec: &ProtocolCodec,
    decoder: FrameDecoder,
    input: &[u8],
    scratch: &mut [u16; 1024],
) {
    let decoded = match decoder {
        FrameDecoder::Command => codec
            .decode_typed_command(input, &mut DecodeScratch::new(scratch))
            .map(|_| ()),
        FrameDecoder::Response => codec
            .decode_typed_response(input, &mut DecodeScratch::new(scratch))
            .map(|_| ()),
        FrameDecoder::Event => codec
            .decode_typed_event(input, &mut DecodeScratch::new(scratch))
            .map(|_| ()),
        FrameDecoder::Transaction => codec
            .decode_session_transaction(input, &mut DecodeScratch::new(scratch))
            .map(|_| ()),
    };
    decoded.expect("frozen BTLV frame verifies");
}

struct Measurement {
    encoded_bytes: usize,
    encode_ns: u128,
    decode_ns: u128,
    allocation_count: u64,
    allocation_bytes: u64,
    peak_scratch_bytes: usize,
    prepared_linear_memory_bytes: usize,
    malformed_rejection_ns: u128,
}

fn measure_frame(
    format: Format,
    frame: &WorkFrame,
    codec: &ProtocolCodec,
    output: &mut [u8],
    flatbuffer_builder: &mut flatbuffers::FlatBufferBuilder<'_>,
    flatbuffer_scratch: &mut FlatbufferScratch,
    scratch: &mut [u16; 1024],
) -> Measurement {
    // Owned response-page wrappers are prepared before the measured interval. Both encoders then
    // receive the same immutable logical object graph without benchmark-adapter clone allocation.
    let prepared_btlv = match format {
        Format::Btlv => Some(frame.btlv.prepare()),
        Format::FlatBuffers => None,
    };
    arm_allocations();
    let encode_start = Instant::now();
    let encoded_bytes = match format {
        Format::Btlv => prepared_btlv
            .as_ref()
            .expect("BTLV source prepared before allocation interval")
            .encode_into(codec, output),
        Format::FlatBuffers => flatbuffer_encode(frame, flatbuffer_builder, flatbuffer_scratch),
    };
    let encode_ns = encode_start.elapsed().as_nanos();
    let encoded = match format {
        Format::Btlv => &output[..encoded_bytes],
        Format::FlatBuffers => flatbuffer_builder.finished_data(),
    };
    let decode_start = Instant::now();
    match format {
        Format::Btlv => decode_btlv(codec, frame.decoder, encoded, scratch),
        Format::FlatBuffers => {
            flatbuffer_verify(frame, encoded).expect("frozen FlatBuffer semantic object verifies");
        }
    }
    let decode_ns = decode_start.elapsed().as_nanos();
    let (allocation_count, allocation_bytes) = disarm_allocations();
    let malformed_start = Instant::now();
    let mut malformed = encoded.to_vec();
    malformed[0] ^= 0xff;
    match format {
        Format::Btlv => {
            let _ = codec.decode(&malformed, &mut DecodeScratch::new(scratch));
        }
        Format::FlatBuffers => {
            let _ = flatbuffer_verify(frame, &malformed);
        }
    }
    Measurement {
        encoded_bytes,
        encode_ns,
        decode_ns,
        allocation_count,
        allocation_bytes,
        peak_scratch_bytes: match format {
            Format::Btlv => std::mem::size_of_val(scratch),
            Format::FlatBuffers => flatbuffer_scratch.prepared_bytes(),
        },
        prepared_linear_memory_bytes: MAX_FRAME_BYTES,
        malformed_rejection_ns: malformed_start.elapsed().as_nanos(),
    }
}

fn warmup(
    format: Format,
    frames: &[WorkFrame],
    codec: &ProtocolCodec,
    output: &mut [u8],
    flatbuffer_builder: &mut flatbuffers::FlatBufferBuilder<'_>,
    flatbuffer_scratch: &mut FlatbufferScratch,
    scratch: &mut [u16; 1024],
) {
    for frame in frames {
        let _ = measure_frame(
            format,
            frame,
            codec,
            output,
            flatbuffer_builder,
            flatbuffer_scratch,
            scratch,
        );
    }
}

fn main() {
    // #104 F4: prove the shared audited allocator is the one serving this process. A global
    // allocator registered by a dependency that is never named may not be linked at all, and a
    // silently absent audit reports success for every gate below it.
    bench_alloc::assert_installed();
    #[cfg(target_arch = "wasm32")]
    {
        assert_eq!(corpus_checksum(), CORPUS_CHECKSUM);
        return;
    }
    #[cfg(not(target_arch = "wasm32"))]
    native_main();
}

#[cfg(not(target_arch = "wasm32"))]
fn native_main() {
    let rounds = parse_rounds();
    assert_eq!(rounds, ROUNDS, "the benchmark is exactly two rounds");
    let frames = corpus();
    assert_eq!(corpus_checksum(), CORPUS_CHECKSUM, "frozen corpus checksum");
    let metadata = Metadata::gather();
    let codec = ProtocolCodec::default();
    let mut output = vec![0_u8; MAX_FRAME_BYTES];
    let mut flatbuffer_builder = flatbuffers::FlatBufferBuilder::with_capacity(MAX_FRAME_BYTES);
    let maximum_logical_values = frames
        .iter()
        .map(|frame| frame.logical_values.len())
        .max()
        .expect("nonempty benchmark corpus");
    let maximum_text_bytes = maximum_text_bytes(&frames);
    let mut flatbuffer_scratch =
        FlatbufferScratch::prepare(maximum_logical_values, maximum_text_bytes);
    let mut scratch = [0_u16; 1024];
    for format in [Format::Btlv, Format::FlatBuffers] {
        warmup(
            format,
            &frames,
            &codec,
            &mut output,
            &mut flatbuffer_builder,
            &mut flatbuffer_scratch,
            &mut scratch,
        );
    }
    for round in 1..=ROUNDS {
        let order = if round == 1 {
            [Format::Btlv, Format::FlatBuffers]
        } else {
            [Format::FlatBuffers, Format::Btlv]
        };
        for format in order {
            for frame in &frames {
                let measurement = measure_frame(
                    format,
                    frame,
                    &codec,
                    &mut output,
                    &mut flatbuffer_builder,
                    &mut flatbuffer_scratch,
                    &mut scratch,
                );
                println!(
                    "{}",
                    json_record(round, format, frame, &measurement, &metadata)
                );
            }
        }
    }
}

fn parse_rounds() -> u8 {
    let mut arguments = env::args().skip(1);
    match (arguments.next().as_deref(), arguments.next()) {
        (Some("--rounds"), Some(value)) if arguments.next().is_none() => value
            .parse()
            .ok()
            .filter(|rounds| *rounds == ROUNDS)
            .unwrap_or_else(|| panic!("usage: miso_engine_protocol_bench --rounds 2")),
        _ => panic!("usage: miso_engine_protocol_bench --rounds 2"),
    }
}

fn maximum_text_bytes(frames: &[WorkFrame]) -> usize {
    frames
        .iter()
        .map(|frame| {
            frame
                .logical_values
                .iter()
                .filter_map(|value| value.text_value.as_ref())
                .map(String::len)
                .sum()
        })
        .max()
        .unwrap_or(0)
}

struct Metadata {
    cpu: String,
    governor: String,
    rustc: String,
    target_cpu: String,
    target_features: String,
    wasm_host: String,
    wasm_version: String,
    wasm_scalar_bytes: u64,
    wasm_simd_bytes: u64,
}

impl Metadata {
    fn gather() -> Self {
        Self {
            cpu: fs::read_to_string("/proc/cpuinfo")
                .ok()
                .and_then(|contents| {
                    contents
                        .lines()
                        .find_map(|line| line.strip_prefix("model name\t: ").map(str::to_owned))
                })
                .unwrap_or_else(|| "unknown".to_owned()),
            governor: fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor")
                .map(|value| value.trim().to_owned())
                .unwrap_or_else(|_| "unknown".to_owned()),
            rustc: command(&["rustc", "-V"]),
            target_cpu: variable("MISO_ENGINE_BENCH_TARGET_CPU"),
            target_features: variable("MISO_ENGINE_BENCH_TARGET_FEATURES"),
            wasm_host: variable("MISO_ENGINE_BENCH_WASM_HOST"),
            wasm_version: variable("MISO_ENGINE_BENCH_WASM_HOST_VERSION"),
            wasm_scalar_bytes: variable("MISO_ENGINE_BENCH_WASM_SCALAR_BYTES")
                .parse()
                .unwrap_or(0),
            wasm_simd_bytes: variable("MISO_ENGINE_BENCH_WASM_SIMD_BYTES")
                .parse()
                .unwrap_or(0),
        }
    }
}

fn json_record(
    round: u8,
    format: Format,
    frame: &WorkFrame,
    measurement: &Measurement,
    metadata: &Metadata,
) -> String {
    let automation = frame.automation_records as u128;
    let encode_per_automation = measurement
        .encode_ns
        .checked_div(automation)
        .map_or_else(|| "null".to_owned(), |value| value.to_string());
    let decode_per_automation = measurement
        .decode_ns
        .checked_div(automation)
        .map_or_else(|| "null".to_owned(), |value| value.to_string());
    format!(
        concat!(
            "{{\"schema_version\":1,\"benchmark_id\":\"issue005_btlv_flatbuffers\",\"format\":\"{}\",\"round\":{},\"rounds\":2,\"order_index\":{},",
            "\"seed\":\"0x4953535545303035\",\"corpus_checksum\":\"{:016x}\",\"counts\":{{\"structural_operations\":64,\"parameter_descriptors\":256,\"parameter_state_records\":256,\"automation_records\":10000,\"automation_batches\":40,\"meters\":256}},",
            "\"frame_label\":\"{}\",\"automation_records_in_frame\":{},\"encoded_bytes\":{},\"encode_wall_ns_per_frame\":{},\"decode_wall_ns_per_frame\":{},\"encode_wall_ns_per_automation_record\":{},\"decode_wall_ns_per_automation_record\":{},",
            "\"allocation_count_after_preparation\":{},\"allocation_bytes_after_preparation\":{},\"peak_scratch_bytes\":{},\"prepared_linear_memory_bytes\":{},\"malformed_rejection_wall_ns\":{},",
            "\"toolchain\":\"{}\",\"target_cpu\":\"{}\",\"target_features\":\"{}\",\"cpu\":\"{}\",\"governor\":\"{}\",\"wasm_host\":\"{}\",\"wasm_host_version\":\"{}\",\"timing_scope\":\"native-host-harness\",\"wasm_timing_scope\":\"not-measured-corpus-parity-only\",",
            "\"wasm_scalar_bytes\":{},\"wasm_simd128_bytes\":{},\"wasm_simd128_delta_bytes\":{},\"descriptive_only\":true,\"threshold\":null}}"
        ),
        format.name(),
        round,
        format.round_order(round),
        CORPUS_CHECKSUM,
        escape(&frame.label),
        frame.automation_records,
        measurement.encoded_bytes,
        measurement.encode_ns,
        measurement.decode_ns,
        encode_per_automation,
        decode_per_automation,
        measurement.allocation_count,
        measurement.allocation_bytes,
        measurement.peak_scratch_bytes,
        measurement.prepared_linear_memory_bytes,
        measurement.malformed_rejection_ns,
        escape(&metadata.rustc),
        escape(&metadata.target_cpu),
        escape(&metadata.target_features),
        escape(&metadata.cpu),
        escape(&metadata.governor),
        escape(&metadata.wasm_host),
        escape(&metadata.wasm_version),
        metadata.wasm_scalar_bytes,
        metadata.wasm_simd_bytes,
        i128::from(metadata.wasm_simd_bytes) - i128::from(metadata.wasm_scalar_bytes)
    )
}

fn variable(name: &str) -> String {
    env::var(name)
        .ok()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "unknown".to_owned())
}
fn command(args: &[&str]) -> String {
    let Some((program, rest)) = args.split_first() else {
        return "unknown".to_owned();
    };
    Command::new(program)
        .args(rest)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|output| output.trim().to_owned())
        .filter(|output| !output.is_empty())
        .unwrap_or_else(|| "unknown".to_owned())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frozen_corpus_has_required_cardinality_and_checksum() {
        let frames = corpus();
        assert_eq!(frames.len(), 54);
        assert_eq!(
            frames
                .iter()
                .map(|frame| frame.automation_records)
                .sum::<usize>(),
            AUTOMATION_RECORDS
        );
        assert_eq!(corpus_checksum(), CORPUS_CHECKSUM);
        assert!(include_str!("../CORPUS_MANIFEST.md").contains("9eee4fcb61be3b9e"));
    }

    #[test]
    fn actual_flatbuffer_builder_and_bounded_verifier_agree() {
        let frames = corpus();
        let maximum = frames
            .iter()
            .map(|frame| frame.logical_values.len())
            .max()
            .expect("nonempty corpus");
        let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(MAX_FRAME_BYTES);
        let mut scratch = FlatbufferScratch::prepare(maximum, maximum_text_bytes(&frames));
        for frame in &frames {
            let length = flatbuffer_encode(frame, &mut builder, &mut scratch);
            assert_eq!(builder.finished_data().len(), length);
            assert_eq!(
                flatbuffer_verify(frame, builder.finished_data()),
                Ok(()),
                "semantic FlatBuffer for {}",
                frame.label
            );
            let mut malformed = builder.finished_data().to_vec();
            malformed[0] ^= 0xff;
            assert!(flatbuffer_verify(frame, &malformed).is_err());
        }
        let schema = include_str!("../protocol_benchmark.fbs");
        assert!(schema.contains("unsigned_values:[ulong]"));
        assert!(schema.contains("float_bits:[uint]"));
        assert!(schema.contains("text_offsets:[uint]"));
        assert!(!schema.contains("payload:[ubyte]"));
    }

    #[test]
    fn btlv_sources_encode_and_decode_without_schema_escapes() {
        let codec = ProtocolCodec::default();
        let mut output = vec![0; MAX_FRAME_BYTES];
        let mut scratch = [0; 1024];
        for frame in corpus() {
            let length = frame.btlv.prepare().encode_into(&codec, &mut output);
            decode_btlv(&codec, frame.decoder, &output[..length], &mut scratch);
        }
    }

    #[test]
    fn jsonl_schema_record_carries_frozen_corpus_and_environment_fields() {
        let frame = corpus().into_iter().next().expect("frozen frame");
        let measurement = Measurement {
            encoded_bytes: 64,
            encode_ns: 12,
            decode_ns: 8,
            allocation_count: 0,
            allocation_bytes: 0,
            peak_scratch_bytes: 2048,
            prepared_linear_memory_bytes: MAX_FRAME_BYTES,
            malformed_rejection_ns: 3,
        };
        let metadata = Metadata {
            cpu: "cpu".to_owned(),
            governor: "governor".to_owned(),
            rustc: "rustc".to_owned(),
            target_cpu: "baseline".to_owned(),
            target_features: "none".to_owned(),
            wasm_host: "wasm-interp".to_owned(),
            wasm_version: "version".to_owned(),
            wasm_scalar_bytes: 100,
            wasm_simd_bytes: 120,
        };
        let record = json_record(1, Format::Btlv, &frame, &measurement, &metadata);
        for required in [
            "\"schema_version\":1",
            "\"format\":\"btlv\"",
            "\"round\":1",
            "\"rounds\":2",
            "\"order_index\":0",
            "\"seed\":\"0x4953535545303035\"",
            "\"corpus_checksum\":\"9eee4fcb61be3b9e\"",
            "\"structural_operations\":64",
            "\"parameter_descriptors\":256",
            "\"parameter_state_records\":256",
            "\"automation_records\":10000",
            "\"automation_batches\":40",
            "\"meters\":256",
            "\"encoded_bytes\":64",
            "\"encode_wall_ns_per_frame\":12",
            "\"decode_wall_ns_per_frame\":8",
            "\"allocation_count_after_preparation\":0",
            "\"allocation_bytes_after_preparation\":0",
            "\"peak_scratch_bytes\":2048",
            "\"prepared_linear_memory_bytes\":65536",
            "\"malformed_rejection_wall_ns\":3",
            "\"toolchain\":\"rustc\"",
            "\"target_cpu\":\"baseline\"",
            "\"target_features\":\"none\"",
            "\"cpu\":\"cpu\"",
            "\"governor\":\"governor\"",
            "\"wasm_host\":\"wasm-interp\"",
            "\"wasm_host_version\":\"version\"",
            "\"timing_scope\":\"native-host-harness\"",
            "\"wasm_timing_scope\":\"not-measured-corpus-parity-only\"",
            "\"wasm_scalar_bytes\":100",
            "\"wasm_simd128_bytes\":120",
            "\"wasm_simd128_delta_bytes\":20",
            "\"descriptive_only\":true",
            "\"threshold\":null",
        ] {
            assert!(record.contains(required), "missing schema field {required}");
        }
    }
}
