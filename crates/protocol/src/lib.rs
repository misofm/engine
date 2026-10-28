//! MISO Control BTLV v1, a bounded transport-neutral control-plane protocol.
//!
//! This crate deliberately owns no renderer, `PreparedRenderPlan`, PCM payload, transport
//! adapter, or exported C ABI. Its caller-buffer codec is usable by future IPC, shared-memory,
//! browser-message, and C ABI adapters without retaining any caller pointer.

mod btlv;
#[cfg(any(test, feature = "test-support"))]
mod control_rows;
mod controller;
mod frame_table;
mod message_wire;
mod model;
mod queue;
mod schema;
mod session_wire;
mod typed_frame;
mod wire;

pub use message_wire::{
    Backpressure, BackpressureQueueKind, CONTROL_PAGE_ITEMS, CONTROL_STATE_RECORD_BYTES,
    Capabilities, CapabilityFlags, ControlAddress, ControlBody, ControlClass, ControlEntry,
    ControlFamily, ControlKind, ControlListPage, ControlListRequest, ControlOwnerFilter,
    ControlOwnerHeader, ControlOwnerHeaderKind, ControlOwnerKind, ControlRampRow, ControlReason,
    ControlRouteEnds, ControlRow, ControlScope, ControlSection, ControlSectionHeader,
    ControlStatePage, ControlStateRecord, ControlTaper, CounterId, CounterSnapshot,
    CounterSnapshotRef, CounterValue, CountersRequest, DecodedCapabilities, DecodedMeterBatch,
    Diagnostic, DiagnosticEvent, DiagnosticSeverity, DiagnosticsPage, DiagnosticsRequest, EditPath,
    EnumChoice, LOUDNESS_LEVEL_FLOOR, LoudnessReply, LoudnessRequest, MAXIMUM_COUNTER_ENTRIES,
    MAXIMUM_METER_ENTRIES, MeterBatch, MeterComponent, MeterEntry, MeterMetric, MeterRecord,
    MeterTarget, NonOkResponse, OBSERVATION_READ_MAXIMUM_SELECTIONS, ObservationEnded,
    ObservationReadPage, ObservationReadRequest, ObservationReadRow, ObservationReadSelection,
    ObservationRowStatus, ObservationRowWindow, ObservationSection, ObservationSelectionRef,
    ObservationTapKey, ParameterChannel, ParameterRack, ParameterUnit, PathSegment,
    RESPONSE_BYPASS_CONTROL, ResponseConfiguration, ResponseConfigurationKind, ResponseCurves,
    ResponseGetRequest, ResponseGrid, ResponseMember, ResponseMemberKind, ResponseMemberSection,
    ResponseOverride, ResponseReady, ResponseReadyOutcome, ResponseResult, ResponseTarget,
    SESSION_REPLACE_DIGEST_BYTES, SPECTRUM_CAPTURE_BIN_COUNT, SPECTRUM_CAPTURE_WINDOW_FRAMES,
    SPECTRUM_HOP_FRAMES, SPECTRUM_SMOOTHING_MAXIMUM_MS, SPECTRUM_TARGET_ID_MAXIMUM_BYTES,
    SessionCommitted, SessionReplaceRequest, SessionSnapshot, SessionSnapshotRequest,
    SpectrumCaptureReply, SpectrumCaptureRequest, SpectrumCaptureStatus, SpectrumCaptureWindow,
    SpectrumConfigureRequest, SpectrumConfigured, SpectrumEntry, SpectrumForm, SpectrumStreamBins,
    SpectrumStreamReply, SpectrumStreamRequest, SpectrumStreamStatus, SpectrumStreamTarget,
    SpectrumTargetKind, TelemetryConfiguration, TrackResponseCapture, TransactionApplied,
    TransportSetRequest, TransportSnapshot, TransportState, TransportStateEvent,
    response_ready_encoded_bound, response_result_encoded_len,
};
/// The control core's address types, to name the address of a [`SessionEdit::SetControl`] and of a
/// [`SessionEdit::Observe`].
pub mod control_core {
    /// Name-to-address lookup and the edit lists of a whole fader, builtins or matrix: tests and
    /// the audit tool write values by row name; no production caller does (the control list
    /// carries each row's ID).
    #[cfg(any(test, feature = "test-support"))]
    pub use crate::control_rows::{builtins_edits, fader_edits, matrix_or_pan_edits, row_address};
    pub use controls::{
        BYPASS_CONTROL, ControlAddress, ControlError, LINK_MODE_CONTROL, Lane,
        MAXIMUM_OBSERVATION_WINDOW_BLOCKS, MonitorOverlay, MonitorRow, MonitorWrite,
        ObservationBindings, ObserveError, ObserveWrite, Owner, Section, TapKey, TapSection,
    };
}
pub use frame_table::{
    FrameCode, FrameCodeTable, FrameField, FrameMessage, ResponseFrameTable, response_frame_table,
};
pub use model::{
    ObserveContext, PreparedSessionTransaction, SessionCommit, SessionEdit, SessionEditError,
    SessionEditOpcode, SessionStore, SessionStoreError, apply_session_edit,
};
pub use queue::{
    ControlCommandSlot, CounterTelemetryRecord, ProtocolQueueConfig, ProtocolQueueError,
    ProtocolQueueResourceReport, ProtocolQueues, QueueKind, QueueReport, ReliableEnqueueError,
    ReliableEventReservation, ReliableEventReservations, ReliableHeader, ReliablePayload,
    ReliableSlot, TelemetryCounters, TelemetryKey, TelemetryRecord,
};
pub use session_wire::{DecodedSessionTransaction, SessionTransactionFrame};
pub use typed_frame::{
    CommandPayload, DecodedCommandPayload, DecodedEventPayload, DecodedSuccessResponsePayload,
    DecodedTypedCommandFrame, DecodedTypedEventFrame, DecodedTypedResponseFrame, EventPayload,
    SuccessResponsePayload, TypedCommandFrame, TypedEventFrame, TypedNonOkResponseFrame,
    TypedSuccessResponseFrame,
};
pub use wire::{
    CommandFrame, CommandHeader, DecodeError, DecodeScratch, DecodedFrame, EncodeError, EventFrame,
    EventHeader, ExpectedRevision, Frame, FrameHeader, FrameKind, MessageId, ProtocolCodec,
    ProtocolLimits, ProtocolVersion, RETIRED_MESSAGE_IDS, RequestId, ResponseHeader, SampleTime,
    SessionRevision, StatusCode,
};

/// The current, frozen BTLV wire major version.
pub const PROTOCOL_MAJOR_V1: u16 = 1;
/// The current, frozen BTLV wire minor version.
pub const PROTOCOL_MINOR_V1: u16 = 0;
/// Exact bytes in every BTLV outer header.
pub const OUTER_HEADER_BYTES: usize = 48;
/// The maximum number of records in one meter or counter telemetry page.
pub const MAXIMUM_TELEMETRY_PAGE_RECORDS: usize = 256;
#[cfg(any(test, feature = "test-support"))]
pub use controller::MockProvider;
pub use controller::{
    AttachObservationEndedError, CommandFrameProcessError, CommittedCommandFrame, ControlCommand,
    ControlProvider, ControllerRequest, ControllerResourceAllocationError, ControllerResponse,
    ControllerRetainedCapacity, DocumentDiff, DocumentReplace, DocumentReplaceFn,
    DocumentReplaceRefusal, DocumentReplaceRequest, EventEgressError, MeterWindowSpan,
    ParameterProviderError, PreparedCommandCommitError, PreparedCommandFrame,
    PreparedImmediateCommandFrame, PreparedObservationRead, PreparedSessionCommand,
    PreparedStructuralCommand, ProtocolController, ProtocolControllerConfig, ProviderFeatures,
    ReplayCache, ReplayCacheConfig, ReplayCacheError, ReplayCacheResourceReport, ReplayDecision,
    ReplayHit, ResponseProviderError, ResponseReadyEnqueueError, SessionCommand,
    SessionCommandReply, TelemetryRefusal, TelemetryRefusalPath,
};
