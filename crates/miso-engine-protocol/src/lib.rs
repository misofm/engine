//! MISO Control BTLV v1, a bounded transport-neutral control-plane protocol.
//!
//! This crate deliberately owns no renderer, `PreparedRenderPlan`, PCM payload, transport
//! adapter, or exported C ABI. Its caller-buffer codec is usable by future IPC, shared-memory,
//! browser-message, and C ABI adapters without retaining any caller pointer.

mod btlv;
mod conformance;
mod controller;
mod message_wire;
mod model;
mod queue;
mod schema;
mod session_wire;
mod typed_frame;
mod wire;

pub use message_wire::{
    AutomationCanceled, AutomationCancellationReason, AutomationEnqueue, AutomationEnqueued,
    Backpressure, BackpressureQueueKind, Capabilities, CapabilityFlags, CounterId, CounterSnapshot,
    CounterSnapshotRef, CounterValue, CountersRequest, DecodedAutomationEnqueue,
    DecodedCapabilities, DecodedMeterBatch, Diagnostic, DiagnosticEvent, DiagnosticSeverity,
    DiagnosticsPage, DiagnosticsRequest, EnumChoice, MeterBatch, MeterComponent, MeterRecord,
    NonOkResponse, ParameterAutomationRate, ParameterChannel, ParameterDescriptor, ParameterDomain,
    ParameterMapping, ParameterMetadataPage, ParameterMetadataRequest, ParameterRack,
    ParameterStatePage, ParameterStateRecord, ParameterStateRequest, ParameterUnit,
    ParameterValueKind, PathSegment, SessionCommitted, SessionSnapshot, SessionSnapshotRequest,
    TelemetryConfiguration, TransactionApplied, TransportSetRequest, TransportSnapshot,
    TransportState, TransportStateEvent,
};
pub use model::{
    PreparedSessionTransaction, SessionCommit, SessionEditError, SessionEditOpcode, SessionEdit,
    SessionStore, SessionStoreError, apply_session_edit,
};
pub use queue::{
    AutomationBatchError, AutomationBatchSlot, AutomationEnqueueError, AutomationKind,
    AutomationRecord, ControlCommandSlot, CounterTelemetryRecord, ParameterHandle,
    ProtocolQueueConfig, ProtocolQueueError, ProtocolQueueResourceReport, ProtocolQueues,
    QueueKind, QueueReport, ReliableEnqueueError, ReliableEventReservation,
    ReliableEventReservations, ReliableHeader, ReliablePayload, ReliableSlot, TelemetryCounters,
    TelemetryKey, TelemetryRecord,
};
pub use session_wire::{
    DecodedSessionTransaction, SessionTransactionFrame, complete_all_opcode_fixture,
};
pub use typed_frame::{
    CommandPayload, DecodedCommandPayload, DecodedEventPayload, DecodedSuccessResponsePayload,
    DecodedTypedCommandFrame, DecodedTypedEventFrame, DecodedTypedResponseFrame, EventPayload,
    SuccessResponsePayload, TypedCommandFrame, TypedEventFrame, TypedNonOkResponseFrame,
    TypedSuccessResponseFrame,
};
pub use wire::{
    CommandFrame, CommandHeader, DecodeError, DecodeScratch, DecodedFrame, EncodeError, EventFrame,
    EventHeader, ExpectedRevision, Frame, FrameHeader, FrameKind, MessageId, ProtocolCodec,
    ProtocolLimits, ProtocolVersion, RequestId, ResponseHeader, SampleTime, SessionRevision,
    StatusCode,
};

/// The current, frozen BTLV wire major version.
pub const PROTOCOL_MAJOR_V1: u16 = 1;
/// The current, frozen BTLV wire minor version.
pub const PROTOCOL_MINOR_V1: u16 = 0;
/// Exact bytes in every BTLV outer header.
pub const OUTER_HEADER_BYTES: usize = 48;
/// Exact bytes in every BTLV TLV prefix.
pub const TLV_PREFIX_BYTES: usize = 8;
/// The maximum number of records in one automation batch slot.
pub const AUTOMATION_BATCH_RECORDS: usize = 256;
/// Exact byte width of a transient automation record.
pub const AUTOMATION_RECORD_BYTES: usize = 32;
pub use conformance::{ConformanceDecoder, ConformanceFrame, complete_schema_corpus};
pub use controller::{
    CommandFrameProcessError, CommittedCommandFrame, ControlCommand, ControlProvider,
    ControllerRequest, ControllerResourceAllocationError, ControllerResponse,
    ControllerRetainedCapacity, EventEgressError, MockProvider, MockProviderConfig,
    ParameterProviderError, PreparedCommandCommitError, PreparedCommandFrame,
    PreparedImmediateCommandFrame, PreparedStructuralCommand, ProtocolController,
    ProtocolControllerConfig, ProviderFeatures, ReplayCache, ReplayCacheConfig, ReplayCacheError,
    ReplayCacheResourceReport, ReplayDecision, ReplayHit,
};
