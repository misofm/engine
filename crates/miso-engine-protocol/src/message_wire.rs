//! Schema-specific common BTLV payloads shared by typed command responses.
//!
//! This module deliberately exposes typed values only. It has no arbitrary-message, arbitrary
//! field, or opaque byte-payload encoder. Encoding is a checked sizing pass followed by one
//! caller-buffer write pass; neither pass allocates.

use crate::{
    DecodeError, EncodeError, ProtocolCodec,
    schema::{capabilities_request, descriptor, enum_choice},
    session_wire::{Message, Rule},
};

const WIRE_U8: u8 = 1;
const WIRE_U16: u8 = 2;
const WIRE_U32: u8 = 3;
const WIRE_U64: u8 = 4;
const WIRE_F32: u8 = 6;
const WIRE_UTF8: u8 = 9;
const WIRE_MESSAGE: u8 = 11;
const WIRE_PACKED_U16: u8 = 12;
const WIRE_PACKED_U32: u8 = 13;
const TLV_PREFIX_BYTES: usize = 8;
const NESTED_HEADER_BYTES: usize = 8;

/// Capability bits advertised by a typed BTLV endpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CapabilityFlags(pub u64);

impl CapabilityFlags {
    /// Canonical BTLV, caller-buffer, replay, revisioned-edit, and snapshot support.
    pub const B1B_BASE: Self = Self((1 << 5) - 1);
    /// B1b behavior plus fixed transient automation batches and parameter metadata.
    pub const B2B_BASE: Self = Self((1 << 8) - 1);
    /// B2b behavior plus typed transport provider support.
    pub const B3A_BASE: Self = Self((1 << 9) - 1);
    /// Complete issue-005 typed provider, telemetry, and reliable-event registry.
    pub const B4_BASE: Self = Self((1 << 14) - 1);
    /// Reliable session-committed event support.
    pub const SESSION_EVENT_STREAM: Self = Self(1 << 12);
    const KNOWN: u64 = (1 << 14) - 1;
}

/// All 27 fields of a successful `CAPABILITIES_GET` response.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)] // The 27 frozen field names mirror the normative registry verbatim.
pub struct Capabilities<'a> {
    /// Lowest supported protocol version.
    pub minimum_version: crate::ProtocolVersion,
    /// Highest supported protocol version.
    pub maximum_version: crate::ProtocolVersion,
    /// Effective protocol and endpoint resource bounds.
    pub maximum_frame_bytes: u64,
    /// Effective top-level/nested field bound.
    pub maximum_tlvs: u32,
    /// Effective UTF-8 field byte bound.
    pub maximum_string_bytes: u64,
    /// Effective nested MESSAGE depth.
    pub maximum_nesting: u8,
    /// Fixed v1 batch capacity, exactly 256.
    pub maximum_automation_records: u16,
    /// Effective queue/replay/response bounds, in frozen field order.
    pub control_command_slots: u64,
    pub control_command_bytes: u64,
    pub automation_batch_slots: u64,
    pub reliable_response_slots: u64,
    pub reliable_event_slots: u64,
    pub telemetry_slots: u64,
    pub replay_entries: u64,
    pub replay_bytes: u64,
    pub maximum_cached_response_bytes: u64,
    pub per_block_automation_density: u64,
    pub admission_quantum_frames: u64,
    pub maximum_parameter_page_items: u16,
    pub maximum_diagnostic_page_items: u16,
    pub maximum_telemetry_handles: u16,
    pub maximum_transaction_edits: u32,
    /// Strictly increasing frozen command IDs.
    pub supported_commands: &'a [u16],
    /// Strictly increasing frozen event IDs.
    pub supported_events: &'a [u16],
    /// Frozen behavior/provider/event flags.
    pub flags: CapabilityFlags,
}

/// Borrowed strict decode view of a capabilities response's packed ID fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)] // Borrowed counterpart of the same frozen 27-field registry.
pub struct DecodedCapabilities<'a> {
    pub minimum_version: crate::ProtocolVersion,
    pub maximum_version: crate::ProtocolVersion,
    pub maximum_frame_bytes: u64,
    pub maximum_tlvs: u32,
    pub maximum_string_bytes: u64,
    pub maximum_nesting: u8,
    pub maximum_automation_records: u16,
    pub control_command_slots: u64,
    pub control_command_bytes: u64,
    pub automation_batch_slots: u64,
    pub reliable_response_slots: u64,
    pub reliable_event_slots: u64,
    pub telemetry_slots: u64,
    pub replay_entries: u64,
    pub replay_bytes: u64,
    pub maximum_cached_response_bytes: u64,
    pub per_block_automation_density: u64,
    pub admission_quantum_frames: u64,
    pub maximum_parameter_page_items: u16,
    pub maximum_diagnostic_page_items: u16,
    pub maximum_telemetry_handles: u16,
    pub maximum_transaction_edits: u32,
    /// LE packed strict-increasing `u16` IDs.
    pub supported_commands: &'a [u8],
    /// LE packed strict-increasing `u16` IDs.
    pub supported_events: &'a [u8],
    pub flags: CapabilityFlags,
}

/// Typed `SESSION_SNAPSHOT_GET` request fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)] // Exact two-field wire schema.
pub struct SessionSnapshotRequest {
    pub offset: u64,
    pub maximum_bytes: u32,
}

/// Typed `SESSION_SNAPSHOT_GET` success fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)] // Exact four-field wire schema.
pub struct SessionSnapshot<'a> {
    pub total_bytes: u64,
    pub offset: u64,
    pub canonical_toml_chunk: &'a [u8],
    pub eof: bool,
}

/// Typed successful transaction response payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)] // Exact one-field wire schema.
pub struct TransactionApplied {
    pub applied_operations: u32,
}

/// Typed reliable `SESSION_COMMITTED` payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)] // Exact four-field wire schema.
pub struct SessionCommitted {
    pub event_sequence: u64,
    pub origin_request_id: crate::RequestId,
    pub previous_revision: crate::SessionRevision,
    pub applied_operations: u32,
}

/// Fixed B2a parameter domain registry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum ParameterDomain {
    Continuous = 1,
    Boolean = 2,
    Enumeration = 3,
}
/// Fixed B2a descriptor value kind; only finite `f32` values exist in v1.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum ParameterValueKind {
    F32 = 1,
}
/// Fixed B2a rack registry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum ParameterRack {
    Simd1 = 1,
    Dynamic = 2,
    Simd2 = 3,
}
/// Fixed B2a parameter channel registry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum ParameterChannel {
    Left = 1,
    Right = 2,
    Both = 3,
}
/// Fixed B2a unit registry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum ParameterUnit {
    Db = 1,
    Hz = 2,
    Milliseconds = 3,
    Samples = 4,
    Linear = 5,
    Ratio = 6,
}
/// Fixed B2a mapping registry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum ParameterMapping {
    Linear = 1,
    Logarithmic = 2,
    Exponential = 3,
    Stepped = 4,
}
/// Fixed B2a automation-rate registry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum ParameterAutomationRate {
    Sample = 1,
    Block = 2,
    None = 3,
}

/// One declared enumeration choice in a typed descriptor.
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct EnumChoice {
    pub value: f32,
    pub label: String,
}
/// Complete typed B2a parameter descriptor.
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ParameterDescriptor {
    pub handle: u32,
    pub track_id: String,
    pub rack: ParameterRack,
    pub effect_id: String,
    pub parameter_id: u32,
    pub channel: ParameterChannel,
    pub value_kind: ParameterValueKind,
    pub unit: ParameterUnit,
    pub domain: ParameterDomain,
    pub minimum: Option<f32>,
    pub maximum: Option<f32>,
    pub default: f32,
    pub mapping: ParameterMapping,
    pub automation_rate: ParameterAutomationRate,
    pub smoothing_samples: u32,
    pub flags: u32,
    pub display_name: Option<String>,
    pub display_unit: Option<String>,
    pub enum_choices: Vec<EnumChoice>,
}
/// Typed metadata query cursor and bounded page limit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct ParameterMetadataRequest {
    pub after_handle: u32,
    pub limit: u16,
}
/// Typed metadata page in strictly increasing handle order.
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ParameterMetadataPage {
    pub last_handle: u32,
    pub eof: bool,
    pub descriptors: Vec<ParameterDescriptor>,
}
/// One validated fixed-width parameter state record.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ParameterStateRecord {
    pub handle: u32,
    pub flags: u32,
    pub value: f32,
}
/// Typed state request handles in caller order.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct ParameterStateRequest {
    pub handles: Vec<u32>,
}
/// Typed state page preserving requested-handle order.
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ParameterStatePage {
    pub observed_sample: u64,
    pub records: Vec<ParameterStateRecord>,
}

/// Typed `AUTOMATION_ENQUEUE` command records. The enclosing frame supplies identity/revision.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AutomationEnqueue<'a> {
    /// Canonically ordered fixed records, one through 256 inclusive.
    pub records: &'a [crate::AutomationRecord],
}

/// Borrowed strict decode view of one `AUTOMATION_ENQUEUE` record array.
///
/// The records remain borrowed from the caller frame. Converting them into the fixed queue slot
/// is explicit so decoding itself cannot allocate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecodedAutomationEnqueue<'a> {
    /// Number of exact 32-byte records in `record_bytes`.
    pub count: u16,
    record_bytes: &'a [u8],
}

impl<'a> DecodedAutomationEnqueue<'a> {
    /// Borrow the exact canonical fixed-record bytes.
    #[must_use]
    pub const fn record_bytes(self) -> &'a [u8] {
        self.record_bytes
    }

    /// Decode one already bounds-checked fixed record without allocation.
    pub fn record(self, index: usize) -> Result<crate::AutomationRecord, DecodeError> {
        let offset = index
            .checked_mul(crate::AUTOMATION_RECORD_BYTES)
            .ok_or(DecodeError::LimitExceeded)?;
        let end = offset
            .checked_add(crate::AUTOMATION_RECORD_BYTES)
            .ok_or(DecodeError::LimitExceeded)?;
        let bytes: &[u8; crate::AUTOMATION_RECORD_BYTES] = self
            .record_bytes
            .get(offset..end)
            .ok_or(DecodeError::InvalidTlv)?
            .try_into()
            .map_err(|_| DecodeError::InvalidValueLength)?;
        crate::AutomationRecord::decode_le(bytes).map_err(|_| DecodeError::InvalidTlv)
    }

    /// Copy this borrowed input into the prepared fixed queue-slot representation.
    pub fn into_batch(
        self,
        revision: crate::SessionRevision,
        request_id: crate::RequestId,
    ) -> Result<crate::AutomationBatchSlot, DecodeError> {
        let mut records = [crate::AutomationRecord::EMPTY; crate::AUTOMATION_BATCH_RECORDS];
        for (index, record) in records[..usize::from(self.count)].iter_mut().enumerate() {
            *record = self.record(index)?;
        }
        crate::AutomationBatchSlot::new(revision, request_id, &records[..usize::from(self.count)])
            .map_err(|_| DecodeError::InvalidTlv)
    }
}

/// Typed successful `AUTOMATION_ENQUEUE` admission report.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AutomationEnqueued {
    /// Exact number of atomically accepted fixed records.
    pub accepted_records: u16,
    /// Automation queue occupancy after acceptance.
    pub occupancy: u64,
    /// Fixed automation queue capacity in batches.
    pub capacity: u64,
    /// Immutable queue generation.
    pub generation: u64,
}

/// Frozen absolute transport-state registry. V1 has no pause, toggle, rate, or loop state.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum TransportState {
    /// The endpoint transport is stopped.
    #[default]
    Stopped = 1,
    /// The endpoint transport is playing.
    Playing = 2,
}

/// Typed `TRANSPORT_SET` command. An absent position retains the provider's current position.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransportSetRequest {
    /// Absolute target state.
    pub state: TransportState,
    /// Absolute target position, when changing it.
    pub position: Option<crate::SampleTime>,
}

/// Typed `TRANSPORT_GET`/`TRANSPORT_SET` success payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransportSnapshot {
    /// Effective absolute state.
    pub state: TransportState,
    /// Effective absolute position.
    pub position: crate::SampleTime,
    /// Engine sample at which this observation/change is effective.
    pub effective_sample: crate::SampleTime,
}

/// Typed reliable `TRANSPORT_STATE` event payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransportStateEvent {
    /// Endpoint-monotonic reliable event sequence.
    pub event_sequence: u64,
    /// Effective absolute state.
    pub state: TransportState,
    /// Effective absolute position.
    pub position: crate::SampleTime,
    /// Effective engine sample.
    pub effective_sample: crate::SampleTime,
    /// Originating set request, when this state change came from a command.
    pub origin_request_id: Option<crate::RequestId>,
}

/// Frozen reason why an already accepted automation batch was canceled.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum AutomationCancellationReason {
    /// A structural session revision invalidated revision-scoped handles.
    RevisionChanged = 1,
    /// An absolute transport locate invalidated pending automation.
    TransportLocate = 2,
    /// The endpoint is shutting down.
    EndpointShutdown = 3,
    /// The typed provider became unavailable.
    ProviderUnavailable = 4,
    /// An explicit endpoint reconfiguration invalidated pending automation.
    ExplicitReconfiguration = 5,
}

/// Typed reliable `AUTOMATION_CANCELED` event payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AutomationCanceled {
    /// Endpoint-monotonic reliable event sequence.
    pub event_sequence: u64,
    /// Original nonzero automation command request ID.
    pub origin_request_id: crate::RequestId,
    /// Number of records canceled from the accepted batch.
    pub canceled_records: u16,
    /// Explicit cancellation cause.
    pub reason: AutomationCancellationReason,
    /// Immutable automation queue generation.
    pub queue_generation: u64,
    /// Effective endpoint sample, when known.
    pub effective_sample: Option<crate::SampleTime>,
}

/// Frozen meter component registry for one fixed telemetry record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum MeterComponent {
    /// Left channel metric for a revision-scoped meter handle.
    Left = 1,
    /// Right channel metric for a revision-scoped meter handle.
    Right = 2,
    /// Aggregate metric for a revision-scoped meter handle.
    Aggregate = 3,
}

/// One exact 16-byte meter telemetry record.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeterRecord {
    /// Nonzero revision-scoped meter handle.
    pub handle: u32,
    /// Explicit left/right/aggregate component.
    pub component: MeterComponent,
    /// Frozen VALID/CLIPPED/HELD bit flags.
    pub flags: u16,
    /// Metric value; it is finite whenever VALID is set.
    pub value: f32,
}

/// Borrowed typed `METER_BATCH` telemetry payload.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeterBatch<'a> {
    /// Absolute engine sample at which all records were observed.
    pub observed_sample: crate::SampleTime,
    /// One through 256 validated fixed meter records.
    pub records: &'a [MeterRecord],
}

/// Borrowed strict decoded fixed-record `METER_BATCH` payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DecodedMeterBatch<'a> {
    /// Absolute engine sample at which all records were observed.
    pub observed_sample: crate::SampleTime,
    /// Exact fixed record count.
    pub count: u16,
    record_bytes: &'a [u8],
}

impl<'a> DecodedMeterBatch<'a> {
    /// Borrow the already validated exact `count * 16` record byte range.
    #[must_use]
    pub const fn record_bytes(self) -> &'a [u8] {
        self.record_bytes
    }

    /// Decode one bounded fixed record without allocation.
    pub fn record(self, index: usize) -> Result<MeterRecord, DecodeError> {
        if index >= usize::from(self.count) {
            return Err(DecodeError::InvalidTlv);
        }
        let offset = index.checked_mul(16).ok_or(DecodeError::LimitExceeded)?;
        let bytes = self
            .record_bytes
            .get(offset..offset + 16)
            .ok_or(DecodeError::InvalidTlv)?;
        decode_meter_record(bytes)
    }
}

/// Typed reliable `DIAGNOSTIC` event payload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagnosticEvent {
    /// One provider diagnostic, whose provider sequence is always present here.
    pub diagnostic: Diagnostic,
}

/// Frozen registered non-resetting counter IDs.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u32)]
#[allow(missing_docs)]
pub enum CounterId {
    ControlCommandBackpressure = 1,
    AutomationBackpressure = 2,
    ReliableResponseBackpressure = 3,
    ReliableEventBackpressure = 4,
    TelemetryCoalesced = 5,
    TelemetryDropped = 6,
    MalformedFrames = 7,
    ReplayHits = 8,
    RequestIdReuse = 9,
    ReplayExpired = 10,
    LateAutomation = 11,
    CanceledAutomation = 12,
    AutomationTimePast = 13,
    AutomationOrderReject = 14,
    ValidationFailures = 15,
}

/// Complete endpoint-local telemetry configuration. It does not create meter production.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct TelemetryConfiguration {
    pub meter_handles: Vec<u32>,
    pub meter_period_blocks: u32,
    pub counter_ids: Vec<CounterId>,
    pub counter_period_blocks: u32,
    pub diagnostics_enabled: bool,
    pub minimum_diagnostic_severity: DiagnosticSeverity,
}

/// Typed `COUNTERS_GET` selector.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct CountersRequest {
    pub all: bool,
    /// Sorted unique numeric IDs; unknown values return typed provider `NOT_FOUND`.
    pub ids: Vec<u32>,
}

/// One ascending typed non-resetting counter value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct CounterValue {
    pub id: CounterId,
    pub value: u64,
}

/// Typed `COUNTERS_GET` success page.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct CounterSnapshot {
    pub observed_sample: crate::SampleTime,
    pub values: Vec<CounterValue>,
}

/// Borrowed counter snapshot used by caller-buffer event egress. It is schema-identical to
/// [`CounterSnapshot`] while avoiding an allocation merely to turn prepared counter slots into
/// one `COUNTER_SNAPSHOT` event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CounterSnapshotRef<'a> {
    /// Observation timestamp shared by every value in this batch.
    pub observed_sample: crate::SampleTime,
    /// Ascending bounded counter values.
    pub values: &'a [CounterValue],
}

/// Typed nondestructive `DIAGNOSTICS_GET` cursor request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct DiagnosticsRequest {
    pub after_sequence: u64,
    pub limit: u16,
    pub minimum_severity: DiagnosticSeverity,
}

/// Typed bounded retained diagnostics page.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct DiagnosticsPage {
    pub last_sequence: u64,
    pub eof: bool,
    pub diagnostics: Vec<Diagnostic>,
}

/// One structured diagnostic path segment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PathSegment {
    /// A typed/session field name.
    Field(String),
    /// A zero-based array position.
    Index(u64),
    /// A stable-ID selector.
    StableId(String),
}

/// Frozen diagnostic severity registry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum DiagnosticSeverity {
    /// Informational diagnostic.
    Info = 1,
    /// Warning diagnostic.
    Warning = 2,
    /// Error diagnostic.
    Error = 3,
}

impl DiagnosticSeverity {
    fn decode(value: u8) -> Result<Self, DecodeError> {
        match value {
            1 => Ok(Self::Info),
            2 => Ok(Self::Warning),
            3 => Ok(Self::Error),
            _ => Err(DecodeError::InvalidTlv),
        }
    }
}

/// One typed protocol diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    /// Stable dotted machine-readable code.
    pub code: String,
    /// Severity in the frozen registry.
    pub severity: DiagnosticSeverity,
    /// Ordered structured path; empty represents the document root.
    pub path: Vec<PathSegment>,
    /// Bounded non-stable explanatory detail, when available.
    pub detail: Option<String>,
    /// Zero-based session-edit operation index, when applicable.
    pub operation_index: Option<u32>,
    /// Relevant absolute sample time, when applicable.
    pub sample_time: Option<u64>,
    /// Provider diagnostic sequence, when applicable.
    pub provider_sequence: Option<u64>,
}

/// Exact queue-kind codes used by typed backpressure payloads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum BackpressureQueueKind {
    /// Reliable copied control command queue.
    ControlCommand = 1,
    /// Fixed automation batch queue.
    Automation = 2,
    /// Reliable response queue.
    ReliableResponse = 3,
    /// Reliable event queue.
    ReliableEvent = 4,
    /// Explicitly lossy telemetry queue.
    Telemetry = 5,
    /// Bounded exact-byte replay cache.
    ReplayCache = 6,
}

impl BackpressureQueueKind {
    fn decode(value: u8) -> Result<Self, DecodeError> {
        match value {
            1 => Ok(Self::ControlCommand),
            2 => Ok(Self::Automation),
            3 => Ok(Self::ReliableResponse),
            4 => Ok(Self::ReliableEvent),
            5 => Ok(Self::Telemetry),
            6 => Ok(Self::ReplayCache),
            _ => Err(DecodeError::InvalidTlv),
        }
    }

    const fn has_generation(self) -> bool {
        !matches!(self, Self::ReplayCache)
    }
}

/// Typed queue/replay saturation data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Backpressure {
    /// Saturated resource class.
    pub queue_kind: BackpressureQueueKind,
    /// Fixed capacity in logical items or entries.
    pub capacity: u64,
    /// Occupancy before the rejected atomic attempt.
    pub occupancy: u64,
    /// Atomic requested item count; exactly one in BTLV v1.
    pub requested_items: u16,
    /// Queue generation for SPSC queue kinds; absent for replay cache.
    pub generation: Option<u64>,
    /// Absolute retry boundary/sample hint, when known.
    pub retry_boundary: Option<u64>,
    /// Requested copied bytes when a byte budget rejected.
    pub requested_bytes: Option<u64>,
    /// Available copied bytes when a byte budget rejected.
    pub available_bytes: Option<u64>,
}

/// The only payload permitted on a non-OK BTLV response.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NonOkResponse {
    /// Leading deterministic diagnostics that fit the endpoint response bound.
    pub diagnostics: Vec<Diagnostic>,
    /// Count of deterministic diagnostics omitted after the retained prefix.
    pub omitted_diagnostics: u32,
    /// Typed saturation detail present if and only if status is `BACKPRESSURE`.
    pub backpressure: Option<Backpressure>,
}

impl NonOkResponse {
    /// Retain at most `maximum` leading diagnostics and record the exact omitted suffix count.
    #[must_use]
    pub fn bounded(diagnostics: &[Diagnostic], maximum: usize) -> Self {
        let retained = diagnostics.len().min(maximum);
        Self {
            diagnostics: diagnostics[..retained].to_vec(),
            omitted_diagnostics: u32::try_from(diagnostics.len().saturating_sub(retained))
                .unwrap_or(u32::MAX),
            backpressure: None,
        }
    }
}

impl ProtocolCodec {
    /// Return the exact direct-caller-buffer length for one common non-OK response payload.
    pub fn encoded_non_ok_payload_len(&self, value: &NonOkResponse) -> Result<usize, EncodeError> {
        let length = non_ok_len(self, value)?;
        if length > self.limits().max_frame_bytes {
            return Err(EncodeError::LimitExceeded);
        }
        Ok(length)
    }

    /// Encode one canonical common non-OK response payload into caller output.
    ///
    /// A short output buffer is never modified. This path has no heap allocation after the typed
    /// `NonOkResponse` input already exists.
    pub fn encode_non_ok_payload(
        &self,
        value: &NonOkResponse,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_non_ok_payload_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = PayloadWriter::new(output, self.limits().max_tlv_count);
        write_non_ok(self, &mut writer, value)?;
        debug_assert_eq!(writer.position, required);
        Ok(required)
    }

    /// Decode one common non-OK response payload after the outer frame has supplied its count.
    pub fn decode_non_ok_payload(
        &self,
        payload: &[u8],
        tlv_count: u32,
    ) -> Result<NonOkResponse, DecodeError> {
        if tlv_count > self.limits().max_tlv_count || payload.len() > self.limits().max_frame_bytes
        {
            return Err(DecodeError::LimitExceeded);
        }
        decode_non_ok(self, top_level_message(self, payload, tlv_count)?)
    }

    /// Return the exact canonical nested-message length for one typed diagnostic.
    pub fn encoded_diagnostic_message_len(&self, value: &Diagnostic) -> Result<usize, EncodeError> {
        diagnostic_message_len(self, value)
    }

    /// Decode one nested typed diagnostic payload.
    pub fn decode_diagnostic_message(&self, value: &[u8]) -> Result<Diagnostic, DecodeError> {
        decode_diagnostic(self, nested_message(self, value, 0)?)
    }

    /// Exact payload length for the 27-field successful capabilities response.
    pub fn encoded_capabilities_len(&self, value: &Capabilities<'_>) -> Result<usize, EncodeError> {
        capabilities_len(self, value)
    }

    /// Encode a successful capabilities payload directly into caller-owned output.
    pub fn encode_capabilities(
        &self,
        value: &Capabilities<'_>,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_capabilities_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = PayloadWriter::new(output, self.limits().max_tlv_count);
        write_capabilities(&mut writer, value)?;
        Ok(required)
    }

    /// Strictly decode the exact 27-field successful capabilities payload.
    pub fn decode_capabilities<'a>(
        &self,
        payload: &'a [u8],
        count: u32,
    ) -> Result<DecodedCapabilities<'a>, DecodeError> {
        decode_capabilities(self, top_level_message(self, payload, count)?)
    }

    /// Validate the empty capabilities command payload, allowing only skippable future fields.
    pub fn decode_capabilities_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<(), DecodeError> {
        top_level_message(self, payload, count)?.schema_spec(&capabilities_request::SPEC)
    }

    /// Exact caller-output length for a snapshot request.
    pub const fn encoded_snapshot_request_len(&self, _value: SessionSnapshotRequest) -> usize {
        32
    }

    /// Encode the typed two-field snapshot request.
    pub fn encode_snapshot_request(
        &self,
        value: SessionSnapshotRequest,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        if value.maximum_bytes == 0 {
            return Err(EncodeError::LimitExceeded);
        }
        let required = self.encoded_snapshot_request_len(value);
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = PayloadWriter::new(output, self.limits().max_tlv_count);
        writer.field(1, WIRE_U64, true, &value.offset.to_le_bytes())?;
        writer.field(2, WIRE_U32, true, &value.maximum_bytes.to_le_bytes())?;
        Ok(required)
    }

    /// Strictly decode the typed two-field snapshot request.
    pub fn decode_snapshot_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<SessionSnapshotRequest, DecodeError> {
        let message = top_level_message(self, payload, count)?;
        message.schema(&[Rule::one(1), Rule::one(2)])?;
        let result = SessionSnapshotRequest {
            offset: read_u64(message.one(1, WIRE_U64)?)?,
            maximum_bytes: read_u32(message.one(2, WIRE_U32)?)?,
        };
        if result.maximum_bytes == 0 {
            return Err(DecodeError::InvalidTlv);
        }
        Ok(result)
    }

    /// Exact caller-output length for a snapshot success payload.
    pub fn encoded_snapshot_len(&self, value: SessionSnapshot<'_>) -> Result<usize, EncodeError> {
        if value.offset > value.total_bytes
            || u64::try_from(value.canonical_toml_chunk.len())
                .map_err(|_| EncodeError::LimitExceeded)?
                > value.total_bytes.saturating_sub(value.offset)
            || value.eof
                != (value
                    .offset
                    .saturating_add(value.canonical_toml_chunk.len() as u64)
                    == value.total_bytes)
        {
            return Err(EncodeError::LimitExceeded);
        }
        let len = tlv_len(8)?
            .checked_add(tlv_len(8)?)
            .and_then(|v| v.checked_add(tlv_len(value.canonical_toml_chunk.len()).ok()?))
            .and_then(|v| v.checked_add(tlv_len(1).ok()?))
            .ok_or(EncodeError::LimitExceeded)?;
        if len > self.limits().max_frame_bytes {
            return Err(EncodeError::LimitExceeded);
        }
        Ok(len)
    }

    /// Encode a typed canonical-TOML snapshot chunk.
    pub fn encode_snapshot(
        &self,
        value: SessionSnapshot<'_>,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_snapshot_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = PayloadWriter::new(output, self.limits().max_tlv_count);
        writer.field(1, WIRE_U64, true, &value.total_bytes.to_le_bytes())?;
        writer.field(2, WIRE_U64, true, &value.offset.to_le_bytes())?;
        writer.field(3, 10, true, value.canonical_toml_chunk)?;
        writer.field(4, 8, true, &[u8::from(value.eof)])?;
        Ok(required)
    }

    /// Strictly decode a typed snapshot success payload.
    pub fn decode_snapshot<'a>(
        &self,
        payload: &'a [u8],
        count: u32,
    ) -> Result<SessionSnapshot<'a>, DecodeError> {
        let message = top_level_message(self, payload, count)?;
        message.schema(&[Rule::one(1), Rule::one(2), Rule::one(3), Rule::one(4)])?;
        let result = SessionSnapshot {
            total_bytes: read_u64(message.one(1, WIRE_U64)?)?,
            offset: read_u64(message.one(2, WIRE_U64)?)?,
            canonical_toml_chunk: message.one(3, 10)?,
            eof: read_bool(message.one(4, 8)?)?,
        };
        if result.offset > result.total_bytes
            || u64::try_from(result.canonical_toml_chunk.len())
                .map_err(|_| DecodeError::LimitExceeded)?
                > result.total_bytes.saturating_sub(result.offset)
            || result.eof
                != (result
                    .offset
                    .saturating_add(result.canonical_toml_chunk.len() as u64)
                    == result.total_bytes)
        {
            return Err(DecodeError::InvalidTlv);
        }
        Ok(result)
    }

    /// Encode/decode helpers for the one-field transaction success payload.
    pub fn encode_transaction_applied(
        &self,
        value: TransactionApplied,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        const LEN: usize = 16;
        if output.len() < LEN {
            return Err(EncodeError::OutputTooSmall { required: LEN });
        }
        PayloadWriter::new(output, self.limits().max_tlv_count).field(
            1,
            WIRE_U32,
            true,
            &value.applied_operations.to_le_bytes(),
        )?;
        Ok(LEN)
    }

    /// Strictly decode the one-field transaction success payload.
    pub fn decode_transaction_applied(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<TransactionApplied, DecodeError> {
        let message = top_level_message(self, payload, count)?;
        message.schema(&[Rule::one(1)])?;
        Ok(TransactionApplied {
            applied_operations: read_u32(message.one(1, WIRE_U32)?)?,
        })
    }

    /// Encode/decode helpers for the four-field reliable session-committed event.
    pub fn encode_session_committed(
        &self,
        value: SessionCommitted,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        const LEN: usize = 64;
        if output.len() < LEN {
            return Err(EncodeError::OutputTooSmall { required: LEN });
        }
        let mut writer = PayloadWriter::new(output, self.limits().max_tlv_count);
        writer.field(1, WIRE_U64, true, &value.event_sequence.to_le_bytes())?;
        writer.field(
            2,
            WIRE_U64,
            true,
            &value.origin_request_id.get().to_le_bytes(),
        )?;
        writer.field(3, WIRE_U64, true, &value.previous_revision.0.to_le_bytes())?;
        writer.field(4, WIRE_U32, true, &value.applied_operations.to_le_bytes())?;
        Ok(LEN)
    }

    /// Strictly decode a reliable session-committed event payload.
    pub fn decode_session_committed(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<SessionCommitted, DecodeError> {
        let message = top_level_message(self, payload, count)?;
        message.schema(&[Rule::one(1), Rule::one(2), Rule::one(3), Rule::one(4)])?;
        Ok(SessionCommitted {
            event_sequence: read_u64(message.one(1, WIRE_U64)?)?,
            origin_request_id: crate::RequestId::new(read_u64(message.one(2, WIRE_U64)?)?)
                .ok_or(DecodeError::InvalidTlv)?,
            previous_revision: crate::SessionRevision(read_u64(message.one(3, WIRE_U64)?)?),
            applied_operations: read_u32(message.one(4, WIRE_U32)?)?,
        })
    }

    /// Encode/decode the exact two-field metadata cursor request.
    pub fn encode_parameter_metadata_request(
        &self,
        value: ParameterMetadataRequest,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        if value.limit == 0 || value.limit > 256 {
            return Err(EncodeError::LimitExceeded);
        }
        if output.len() < 32 {
            return Err(EncodeError::OutputTooSmall { required: 32 });
        }
        let mut writer = PayloadWriter::new(output, self.limits().max_tlv_count);
        writer.field(1, WIRE_U32, true, &value.after_handle.to_le_bytes())?;
        writer.field(2, WIRE_U16, true, &value.limit.to_le_bytes())?;
        Ok(32)
    }
    /// Strictly decode the exact two-field metadata cursor request.
    pub fn decode_parameter_metadata_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<ParameterMetadataRequest, DecodeError> {
        let message = top_level_message(self, payload, count)?;
        message.schema(&[Rule::one(1), Rule::one(2)])?;
        let value = ParameterMetadataRequest {
            after_handle: read_u32(message.one(1, WIRE_U32)?)?,
            limit: read_u16(message.one(2, WIRE_U16)?)?,
        };
        if value.limit == 0 || value.limit > 256 {
            return Err(DecodeError::InvalidTlv);
        }
        Ok(value)
    }
    /// Return the exact direct caller-buffer size for a metadata page.
    pub fn encoded_parameter_metadata_page_len(
        &self,
        value: &ParameterMetadataPage,
    ) -> Result<usize, EncodeError> {
        metadata_page_len(self, value)
    }
    /// Encode a canonical typed metadata page without allocation.
    pub fn encode_parameter_metadata_page(
        &self,
        value: &ParameterMetadataPage,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_parameter_metadata_page_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = PayloadWriter::new(output, self.limits().max_tlv_count);
        write_metadata_page(self, &mut writer, value)?;
        Ok(required)
    }
    /// Strictly decode a typed metadata page into bounded typed descriptors.
    pub fn decode_parameter_metadata_page(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<ParameterMetadataPage, DecodeError> {
        decode_metadata_page(self, top_level_message(self, payload, count)?)
    }
    /// Encode a bounded sorted unique nonzero state-handle request without allocation.
    pub fn encoded_parameter_state_request_len(
        &self,
        value: &ParameterStateRequest,
    ) -> Result<usize, EncodeError> {
        check_handles(&value.handles).map_err(|_| EncodeError::LimitExceeded)?;
        tlv_len(
            value
                .handles
                .len()
                .checked_mul(4)
                .ok_or(EncodeError::LimitExceeded)?,
        )
    }

    /// Encode a bounded sorted unique nonzero state-handle request without allocation.
    pub fn encode_parameter_state_request(
        &self,
        value: &ParameterStateRequest,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_parameter_state_request_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = PayloadWriter::new(output, self.limits().max_tlv_count);
        writer.packed_u32(1, &value.handles)?;
        Ok(required)
    }
    /// Strictly decode a bounded sorted unique nonzero state-handle request.
    pub fn decode_parameter_state_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<ParameterStateRequest, DecodeError> {
        let message = top_level_message(self, payload, count)?;
        message.schema(&[Rule::one(1)])?;
        let bytes = message.one(1, WIRE_PACKED_U32)?;
        if !bytes.len().is_multiple_of(4) {
            return Err(DecodeError::InvalidValueLength);
        }
        let handles = bytes
            .chunks_exact(4)
            .map(read_u32)
            .collect::<Result<Vec<_>, _>>()?;
        check_handles(&handles)?;
        Ok(ParameterStateRequest { handles })
    }
    /// Return exact caller-buffer bytes for the validated fixed-record state page.
    pub fn encoded_parameter_state_page_len(
        &self,
        value: &ParameterStatePage,
    ) -> Result<usize, EncodeError> {
        state_page_len(self, value)
    }
    /// Encode a validated fixed-16-byte-record state page without allocation.
    pub fn encode_parameter_state_page(
        &self,
        value: &ParameterStatePage,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_parameter_state_page_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = PayloadWriter::new(output, self.limits().max_tlv_count);
        writer.field(1, WIRE_U64, true, &value.observed_sample.to_le_bytes())?;
        writer.field(
            2,
            WIRE_U16,
            true,
            &(value.records.len() as u16).to_le_bytes(),
        )?;
        writer.field(3, WIRE_U16, true, &16_u16.to_le_bytes())?;
        write_state_record_bytes(&mut writer, &value.records)?;
        Ok(required)
    }
    /// Strictly decode fixed 16-byte state records.
    pub fn decode_parameter_state_page(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<ParameterStatePage, DecodeError> {
        decode_state_page(self, top_level_message(self, payload, count)?)
    }

    /// Return the exact direct caller-buffer length for one typed automation command payload.
    pub fn encoded_automation_enqueue_len(
        &self,
        value: AutomationEnqueue<'_>,
    ) -> Result<usize, EncodeError> {
        validate_automation_records(value.records).map_err(|_| EncodeError::LimitExceeded)?;
        let bytes = value
            .records
            .len()
            .checked_mul(crate::AUTOMATION_RECORD_BYTES)
            .ok_or(EncodeError::LimitExceeded)?;
        let len = checked_add(checked_add(tlv_len(2)?, tlv_len(2)?)?, tlv_len(bytes)?)?;
        if len > self.limits().max_frame_bytes || 3 > self.limits().max_tlv_count {
            return Err(EncodeError::LimitExceeded);
        }
        Ok(len)
    }

    /// Encode a fixed-record automation command directly into caller-owned output.
    pub fn encode_automation_enqueue(
        &self,
        value: AutomationEnqueue<'_>,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_automation_enqueue_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = PayloadWriter::new(output, self.limits().max_tlv_count);
        let count = u16::try_from(value.records.len()).map_err(|_| EncodeError::LimitExceeded)?;
        writer.field(1, WIRE_U16, true, &count.to_le_bytes())?;
        writer.field(
            2,
            WIRE_U16,
            true,
            &(crate::AUTOMATION_RECORD_BYTES as u16).to_le_bytes(),
        )?;
        write_automation_record_bytes(&mut writer, value.records)?;
        debug_assert_eq!(writer.position, required);
        Ok(required)
    }

    /// Strictly decode a borrowed fixed-record automation command without allocation.
    pub fn decode_automation_enqueue<'a>(
        &self,
        payload: &'a [u8],
        count: u32,
    ) -> Result<DecodedAutomationEnqueue<'a>, DecodeError> {
        decode_automation_enqueue(self, payload, count)
    }

    /// Encode the exact four-field successful automation admission payload.
    pub fn encode_automation_enqueued(
        &self,
        value: AutomationEnqueued,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        const LEN: usize = 64;
        if value.accepted_records == 0 || value.capacity == 0 || value.occupancy > value.capacity {
            return Err(EncodeError::LimitExceeded);
        }
        if output.len() < LEN {
            return Err(EncodeError::OutputTooSmall { required: LEN });
        }
        let mut writer = PayloadWriter::new(output, self.limits().max_tlv_count);
        writer.field(1, WIRE_U16, true, &value.accepted_records.to_le_bytes())?;
        writer.field(2, WIRE_U64, true, &value.occupancy.to_le_bytes())?;
        writer.field(3, WIRE_U64, true, &value.capacity.to_le_bytes())?;
        writer.field(4, WIRE_U64, true, &value.generation.to_le_bytes())?;
        Ok(LEN)
    }

    /// Strictly decode the exact four-field successful automation admission payload.
    pub fn decode_automation_enqueued(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<AutomationEnqueued, DecodeError> {
        let message = top_level_message(self, payload, count)?;
        message.schema(&[Rule::one(1), Rule::one(2), Rule::one(3), Rule::one(4)])?;
        let result = AutomationEnqueued {
            accepted_records: read_u16(message.one(1, WIRE_U16)?)?,
            occupancy: read_u64(message.one(2, WIRE_U64)?)?,
            capacity: read_u64(message.one(3, WIRE_U64)?)?,
            generation: read_u64(message.one(4, WIRE_U64)?)?,
        };
        if result.accepted_records == 0
            || result.capacity == 0
            || result.occupancy > result.capacity
        {
            return Err(DecodeError::InvalidTlv);
        }
        Ok(result)
    }

    /// Encode the canonical empty `TRANSPORT_GET` command payload.
    pub const fn encode_transport_get_request(
        &self,
        _output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        Ok(0)
    }

    /// Strictly decode an empty `TRANSPORT_GET` request, skipping only unknown optional fields.
    pub fn decode_transport_get_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<(), DecodeError> {
        let fields = scan_transport_fields(self, payload, count, &[])?;
        transport_schema(&fields, &[])
    }

    /// Return the exact caller-output size for a typed absolute transport set request.
    pub const fn encoded_transport_set_request_len(&self, value: TransportSetRequest) -> usize {
        if value.position.is_some() { 32 } else { 16 }
    }

    /// Encode a typed absolute transport-state set request without allocation.
    pub fn encode_transport_set_request(
        &self,
        value: TransportSetRequest,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_transport_set_request_len(value);
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = PayloadWriter::new(output, self.limits().max_tlv_count);
        writer.field(1, WIRE_U8, true, &[value.state as u8])?;
        if let Some(position) = value.position {
            writer.field(2, WIRE_U64, false, &position.0.to_le_bytes())?;
        }
        Ok(required)
    }

    /// Strictly decode a typed absolute transport-state set request without allocation.
    pub fn decode_transport_set_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<TransportSetRequest, DecodeError> {
        let fields = scan_transport_fields(self, payload, count, &[1, 2])?;
        transport_schema(&fields, &[(1, WIRE_U8, true), (2, WIRE_U64, false)])?;
        Ok(TransportSetRequest {
            state: parse_transport_state(read_u8(transport_required(&fields, 1)?)?)?,
            position: transport_optional(&fields, 2)
                .map(read_u64)
                .transpose()?
                .map(crate::SampleTime),
        })
    }

    /// Encode a typed three-field transport snapshot directly into caller output.
    pub fn encode_transport_snapshot(
        &self,
        value: TransportSnapshot,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        const LEN: usize = 48;
        if output.len() < LEN {
            return Err(EncodeError::OutputTooSmall { required: LEN });
        }
        let mut writer = PayloadWriter::new(output, self.limits().max_tlv_count);
        writer.field(1, WIRE_U8, true, &[value.state as u8])?;
        writer.field(2, WIRE_U64, true, &value.position.0.to_le_bytes())?;
        writer.field(3, WIRE_U64, true, &value.effective_sample.0.to_le_bytes())?;
        Ok(LEN)
    }

    /// Strictly decode a typed three-field transport snapshot without allocation.
    pub fn decode_transport_snapshot(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<TransportSnapshot, DecodeError> {
        let fields = scan_transport_fields(self, payload, count, &[1, 2, 3])?;
        transport_schema(
            &fields,
            &[(1, WIRE_U8, true), (2, WIRE_U64, true), (3, WIRE_U64, true)],
        )?;
        Ok(TransportSnapshot {
            state: parse_transport_state(read_u8(transport_required(&fields, 1)?)?)?,
            position: crate::SampleTime(read_u64(transport_required(&fields, 2)?)?),
            effective_sample: crate::SampleTime(read_u64(transport_required(&fields, 3)?)?),
        })
    }

    /// Return the exact caller-output size for a reliable typed transport-state event.
    pub const fn encoded_transport_state_event_len(&self, value: TransportStateEvent) -> usize {
        if value.origin_request_id.is_some() {
            80
        } else {
            64
        }
    }

    /// Encode a reliable typed transport-state event payload without allocation.
    pub fn encode_transport_state_event(
        &self,
        value: TransportStateEvent,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_transport_state_event_len(value);
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = PayloadWriter::new(output, self.limits().max_tlv_count);
        writer.field(1, WIRE_U64, true, &value.event_sequence.to_le_bytes())?;
        writer.field(2, WIRE_U8, true, &[value.state as u8])?;
        writer.field(3, WIRE_U64, true, &value.position.0.to_le_bytes())?;
        writer.field(4, WIRE_U64, true, &value.effective_sample.0.to_le_bytes())?;
        if let Some(origin) = value.origin_request_id {
            writer.field(5, WIRE_U64, false, &origin.get().to_le_bytes())?;
        }
        Ok(required)
    }

    /// Strictly decode a reliable typed transport-state event payload without allocation.
    pub fn decode_transport_state_event(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<TransportStateEvent, DecodeError> {
        let fields = scan_transport_fields(self, payload, count, &[1, 2, 3, 4, 5])?;
        transport_schema(
            &fields,
            &[
                (1, WIRE_U64, true),
                (2, WIRE_U8, true),
                (3, WIRE_U64, true),
                (4, WIRE_U64, true),
                (5, WIRE_U64, false),
            ],
        )?;
        Ok(TransportStateEvent {
            event_sequence: read_u64(transport_required(&fields, 1)?)?,
            state: parse_transport_state(read_u8(transport_required(&fields, 2)?)?)?,
            position: crate::SampleTime(read_u64(transport_required(&fields, 3)?)?),
            effective_sample: crate::SampleTime(read_u64(transport_required(&fields, 4)?)?),
            origin_request_id: match transport_optional(&fields, 5).map(read_u64).transpose()? {
                Some(value) => Some(crate::RequestId::new(value).ok_or(DecodeError::InvalidTlv)?),
                None => None,
            },
        })
    }

    /// Return the exact direct caller-buffer size for a reliable automation cancellation event.
    pub const fn encoded_automation_canceled_len(&self, value: AutomationCanceled) -> usize {
        if value.effective_sample.is_some() {
            96
        } else {
            80
        }
    }

    /// Encode a reliable typed `AUTOMATION_CANCELED` payload without allocation.
    pub fn encode_automation_canceled(
        &self,
        value: AutomationCanceled,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        if value.canceled_records == 0 {
            return Err(EncodeError::LimitExceeded);
        }
        let required = self.encoded_automation_canceled_len(value);
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = PayloadWriter::new(output, self.limits().max_tlv_count);
        writer.field(1, WIRE_U64, true, &value.event_sequence.to_le_bytes())?;
        writer.field(
            2,
            WIRE_U64,
            true,
            &value.origin_request_id.get().to_le_bytes(),
        )?;
        writer.field(3, WIRE_U16, true, &value.canceled_records.to_le_bytes())?;
        writer.field(4, WIRE_U8, true, &[value.reason as u8])?;
        writer.field(5, WIRE_U64, true, &value.queue_generation.to_le_bytes())?;
        if let Some(sample) = value.effective_sample {
            writer.field(6, WIRE_U64, false, &sample.0.to_le_bytes())?;
        }
        Ok(required)
    }

    /// Strictly decode a reliable typed `AUTOMATION_CANCELED` payload.
    pub fn decode_automation_canceled(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<AutomationCanceled, DecodeError> {
        let message = top_level_message(self, payload, count)?;
        message.schema(&[
            Rule::one(1),
            Rule::one(2),
            Rule::one(3),
            Rule::one(4),
            Rule::one(5),
            Rule::optional(6),
        ])?;
        let value = AutomationCanceled {
            event_sequence: read_u64(message.one(1, WIRE_U64)?)?,
            origin_request_id: crate::RequestId::new(read_u64(message.one(2, WIRE_U64)?)?)
                .ok_or(DecodeError::InvalidTlv)?,
            canceled_records: read_u16(message.one(3, WIRE_U16)?)?,
            reason: parse_automation_cancellation_reason(read_u8(message.one(4, WIRE_U8)?)?)?,
            queue_generation: read_u64(message.one(5, WIRE_U64)?)?,
            effective_sample: message
                .optional_one(6, WIRE_U64)?
                .map(read_u64)
                .transpose()?
                .map(crate::SampleTime),
        };
        if value.canceled_records == 0 {
            return Err(DecodeError::InvalidTlv);
        }
        Ok(value)
    }

    /// Return the exact caller-buffer bytes required for one fixed-record meter batch.
    pub fn encoded_meter_batch_len(&self, value: MeterBatch<'_>) -> Result<usize, EncodeError> {
        validate_meter_records(value.records)?;
        let records = value
            .records
            .len()
            .checked_mul(16)
            .ok_or(EncodeError::LimitExceeded)?;
        checked_add(48, tlv_len(records)?)
    }

    /// Encode a canonical fixed-record lossy `METER_BATCH` payload without allocation.
    pub fn encode_meter_batch(
        &self,
        value: MeterBatch<'_>,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_meter_batch_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let count = u16::try_from(value.records.len()).map_err(|_| EncodeError::LimitExceeded)?;
        let record_bytes = value
            .records
            .len()
            .checked_mul(16)
            .ok_or(EncodeError::LimitExceeded)?;
        let mut writer = PayloadWriter::new(output, self.limits().max_tlv_count);
        writer.field(1, WIRE_U64, true, &value.observed_sample.0.to_le_bytes())?;
        writer.field(2, WIRE_U16, true, &count.to_le_bytes())?;
        writer.field(3, WIRE_U16, true, &16_u16.to_le_bytes())?;
        let start = writer.position;
        writer.field(4, 10, true, &[])?;
        let end = start
            .checked_add(tlv_len(record_bytes)?)
            .ok_or(EncodeError::LimitExceeded)?;
        let bytes = writer
            .output
            .get_mut(start..end)
            .ok_or(EncodeError::LimitExceeded)?;
        bytes[4..8].copy_from_slice(
            &u32::try_from(record_bytes)
                .map_err(|_| EncodeError::LimitExceeded)?
                .to_le_bytes(),
        );
        for (index, record) in value.records.iter().enumerate() {
            encode_meter_record(record, &mut bytes[8 + index * 16..8 + (index + 1) * 16]);
        }
        bytes[8 + record_bytes..].fill(0);
        writer.position = end;
        debug_assert_eq!(writer.position, required);
        Ok(required)
    }

    /// Strictly decode a bounded borrowed fixed-record lossy `METER_BATCH` payload.
    pub fn decode_meter_batch<'a>(
        &self,
        payload: &'a [u8],
        count: u32,
    ) -> Result<DecodedMeterBatch<'a>, DecodeError> {
        let message = top_level_message(self, payload, count)?;
        message.schema(&[Rule::one(1), Rule::one(2), Rule::one(3), Rule::one(4)])?;
        let result = DecodedMeterBatch {
            observed_sample: crate::SampleTime(read_u64(message.one(1, WIRE_U64)?)?),
            count: read_u16(message.one(2, WIRE_U16)?)?,
            record_bytes: message.one(4, 10)?,
        };
        if result.count == 0
            || result.count > 256
            || read_u16(message.one(3, WIRE_U16)?)? != 16
            || result.record_bytes.len()
                != usize::from(result.count)
                    .checked_mul(16)
                    .ok_or(DecodeError::LimitExceeded)?
        {
            return Err(DecodeError::InvalidTlv);
        }
        for index in 0..usize::from(result.count) {
            let _ = result.record(index)?;
        }
        Ok(result)
    }

    /// Encode the lossy `COUNTER_SNAPSHOT` event payload using its exact shared counter schema.
    pub fn encode_counter_snapshot_event(
        &self,
        value: &CounterSnapshot,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        self.encode_counter_snapshot(value, output)
    }

    /// Strictly decode the lossy `COUNTER_SNAPSHOT` event payload.
    pub fn decode_counter_snapshot_event(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<CounterSnapshot, DecodeError> {
        self.decode_counter_snapshot(payload, count)
    }

    /// Return the exact caller-buffer length for one reliable typed diagnostic event.
    pub fn encoded_diagnostic_event_len(
        &self,
        value: &DiagnosticEvent,
    ) -> Result<usize, EncodeError> {
        self.encoded_diagnostic_ref_len(&value.diagnostic)
    }

    /// Return the exact caller-buffer size for one borrowed reliable diagnostic event.
    pub fn encoded_diagnostic_ref_len(
        &self,
        diagnostic: &Diagnostic,
    ) -> Result<usize, EncodeError> {
        if diagnostic.provider_sequence.is_none() {
            return Err(EncodeError::LimitExceeded);
        }
        let required_nesting = if diagnostic.path.is_empty() { 1 } else { 2 };
        if self.limits().max_nesting < required_nesting {
            return Err(EncodeError::LimitExceeded);
        }
        tlv_len(self.encoded_diagnostic_message_len(diagnostic)?)
    }

    /// Encode one reliable typed diagnostic event directly into caller output without allocation.
    pub fn encode_diagnostic_event(
        &self,
        value: &DiagnosticEvent,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        self.encode_diagnostic_ref(&value.diagnostic, output)
    }

    /// Encode one borrowed reliable diagnostic event directly into caller output.
    pub fn encode_diagnostic_ref(
        &self,
        diagnostic: &Diagnostic,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_diagnostic_ref_len(diagnostic)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = PayloadWriter::new(output, self.limits().max_tlv_count);
        write_diagnostic_field(self, &mut writer, 1, true, diagnostic)?;
        Ok(required)
    }

    /// Strictly decode one reliable typed diagnostic event requiring provider sequence field 7.
    pub fn decode_diagnostic_event(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<DiagnosticEvent, DecodeError> {
        let message = top_level_message(self, payload, count)?;
        message.schema(&[Rule::one(1)])?;
        let diagnostic = decode_diagnostic(
            self,
            nested_message(self, message.one(1, WIRE_MESSAGE)?, 1)?,
        )?;
        if diagnostic.provider_sequence.is_none() {
            return Err(DecodeError::InvalidTlv);
        }
        Ok(DiagnosticEvent { diagnostic })
    }

    /// Return the direct caller-buffer size for a complete telemetry configuration echo.
    pub fn encoded_telemetry_configuration_len(
        &self,
        value: &TelemetryConfiguration,
    ) -> Result<usize, EncodeError> {
        validate_telemetry_configuration(value).map_err(|_| EncodeError::LimitExceeded)?;
        let meters = value
            .meter_handles
            .len()
            .checked_mul(4)
            .ok_or(EncodeError::LimitExceeded)?;
        let counters = value
            .counter_ids
            .len()
            .checked_mul(4)
            .ok_or(EncodeError::LimitExceeded)?;
        let length = [meters, 4, counters, 4, 1, 1]
            .into_iter()
            .try_fold(0_usize, |sum, field| checked_add(sum, tlv_len(field)?))?;
        if length > self.limits().max_frame_bytes || 6 > self.limits().max_tlv_count {
            return Err(EncodeError::LimitExceeded);
        }
        Ok(length)
    }

    /// Encode a canonical telemetry configuration or success echo without allocation.
    pub fn encode_telemetry_configuration(
        &self,
        value: &TelemetryConfiguration,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_telemetry_configuration_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = PayloadWriter::new(output, self.limits().max_tlv_count);
        writer.packed_u32(1, &value.meter_handles)?;
        writer.field(2, WIRE_U32, true, &value.meter_period_blocks.to_le_bytes())?;
        write_counter_ids(&mut writer, 3, true, &value.counter_ids)?;
        writer.field(
            4,
            WIRE_U32,
            true,
            &value.counter_period_blocks.to_le_bytes(),
        )?;
        writer.field(5, 8, true, &[u8::from(value.diagnostics_enabled)])?;
        writer.field(6, WIRE_U8, true, &[value.minimum_diagnostic_severity as u8])?;
        Ok(required)
    }

    /// Strictly decode a telemetry configuration command or canonical success echo.
    pub fn decode_telemetry_configuration(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<TelemetryConfiguration, DecodeError> {
        let message = top_level_message(self, payload, count)?;
        message.schema(&[
            Rule::one(1),
            Rule::one(2),
            Rule::one(3),
            Rule::one(4),
            Rule::one(5),
            Rule::one(6),
        ])?;
        let result = TelemetryConfiguration {
            meter_handles: decode_nonzero_u32s(message.one(1, WIRE_PACKED_U32)?, true)?,
            meter_period_blocks: read_u32(message.one(2, WIRE_U32)?)?,
            counter_ids: decode_counter_ids(message.one(3, WIRE_PACKED_U32)?)?,
            counter_period_blocks: read_u32(message.one(4, WIRE_U32)?)?,
            diagnostics_enabled: read_bool(message.one(5, 8)?)?,
            minimum_diagnostic_severity: DiagnosticSeverity::decode(read_u8(
                message.one(6, WIRE_U8)?,
            )?)?,
        };
        validate_telemetry_configuration(&result).map_err(|_| DecodeError::InvalidTlv)?;
        Ok(result)
    }

    /// Return exact caller-buffer bytes for one typed counters selector.
    pub fn encoded_counters_request_len(
        &self,
        value: &CountersRequest,
    ) -> Result<usize, EncodeError> {
        validate_counters_request(value).map_err(|_| EncodeError::LimitExceeded)?;
        let mut result = tlv_len(1)?;
        if !value.all {
            result = checked_add(result, tlv_len(value.ids.len() * 4)?)?;
        }
        Ok(result)
    }

    /// Encode a typed counters selector without allocation.
    pub fn encode_counters_request(
        &self,
        value: &CountersRequest,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_counters_request_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = PayloadWriter::new(output, self.limits().max_tlv_count);
        writer.field(1, 8, true, &[u8::from(value.all)])?;
        if !value.all {
            write_u32s(&mut writer, 2, false, &value.ids)?;
        }
        Ok(required)
    }

    /// Strictly decode an all-or-explicit-ID counters selector.
    pub fn decode_counters_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<CountersRequest, DecodeError> {
        let message = top_level_message(self, payload, count)?;
        message.schema(&[Rule::one(1), Rule::optional(2)])?;
        let all = read_bool(message.one(1, 8)?)?;
        let ids = message
            .optional_one(2, WIRE_PACKED_U32)?
            .map(|bytes| decode_nonzero_u32s(bytes, true))
            .transpose()?
            .unwrap_or_default();
        let result = CountersRequest { all, ids };
        validate_counters_request(&result).map_err(|_| DecodeError::InvalidTlv)?;
        Ok(result)
    }

    /// Return exact caller-buffer bytes for one typed nondestructive counter snapshot.
    pub fn encoded_counter_snapshot_len(
        &self,
        value: &CounterSnapshot,
    ) -> Result<usize, EncodeError> {
        self.encoded_counter_snapshot_ref_len(CounterSnapshotRef {
            observed_sample: value.observed_sample,
            values: &value.values,
        })
    }

    /// Return exact caller-buffer bytes for a borrowed fixed/prepared counter snapshot.
    pub fn encoded_counter_snapshot_ref_len(
        &self,
        value: CounterSnapshotRef<'_>,
    ) -> Result<usize, EncodeError> {
        validate_counter_snapshot_ref(value).map_err(|_| EncodeError::LimitExceeded)?;
        let mut result = tlv_len(8)?;
        for _ in value.values {
            result = checked_add(result, tlv_len(NESTED_HEADER_BYTES + 32)?)?;
        }
        Ok(result)
    }

    /// Encode a typed ascending nondestructive counter snapshot without allocation.
    pub fn encode_counter_snapshot(
        &self,
        value: &CounterSnapshot,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        self.encode_counter_snapshot_ref(
            CounterSnapshotRef {
                observed_sample: value.observed_sample,
                values: &value.values,
            },
            output,
        )
    }

    /// Encode a borrowed ascending counter snapshot without allocation.
    pub fn encode_counter_snapshot_ref(
        &self,
        value: CounterSnapshotRef<'_>,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_counter_snapshot_ref_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = PayloadWriter::new(output, self.limits().max_tlv_count);
        writer.field(1, WIRE_U64, true, &value.observed_sample.0.to_le_bytes())?;
        for counter in value.values {
            writer.nested_start(2, true, 32, 2)?;
            writer.field(1, WIRE_U32, true, &(counter.id as u32).to_le_bytes())?;
            writer.field(2, WIRE_U64, true, &counter.value.to_le_bytes())?;
            writer.finish_nested(32)?;
        }
        Ok(required)
    }

    /// Strictly decode a typed ascending nondestructive counter snapshot.
    pub fn decode_counter_snapshot(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<CounterSnapshot, DecodeError> {
        let message = top_level_message(self, payload, count)?;
        message.schema(&[Rule::one(1), Rule::repeated(2)])?;
        let mut values = Vec::with_capacity(message.values(2, WIRE_MESSAGE)?.count());
        for raw in message.values(2, WIRE_MESSAGE)? {
            let counter = nested_message(self, raw, 1)?;
            counter.schema(&[Rule::one(1), Rule::one(2)])?;
            values.push(CounterValue {
                id: parse_counter_id(read_u32(counter.one(1, WIRE_U32)?)?)?,
                value: read_u64(counter.one(2, WIRE_U64)?)?,
            });
        }
        let result = CounterSnapshot {
            observed_sample: crate::SampleTime(read_u64(message.one(1, WIRE_U64)?)?),
            values,
        };
        validate_counter_snapshot(&result).map_err(|_| DecodeError::InvalidTlv)?;
        Ok(result)
    }

    /// Encode the exact three-field diagnostics cursor request without allocation.
    pub fn encode_diagnostics_request(
        &self,
        value: DiagnosticsRequest,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        if value.limit == 0 || value.limit > 256 {
            return Err(EncodeError::LimitExceeded);
        }
        if output.len() < 48 {
            return Err(EncodeError::OutputTooSmall { required: 48 });
        }
        let mut writer = PayloadWriter::new(output, self.limits().max_tlv_count);
        writer.field(1, WIRE_U64, true, &value.after_sequence.to_le_bytes())?;
        writer.field(2, WIRE_U16, true, &value.limit.to_le_bytes())?;
        writer.field(3, WIRE_U8, true, &[value.minimum_severity as u8])?;
        Ok(48)
    }

    /// Strictly decode a typed diagnostics cursor request.
    pub fn decode_diagnostics_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<DiagnosticsRequest, DecodeError> {
        let message = top_level_message(self, payload, count)?;
        message.schema(&[Rule::one(1), Rule::one(2), Rule::one(3)])?;
        let value = DiagnosticsRequest {
            after_sequence: read_u64(message.one(1, WIRE_U64)?)?,
            limit: read_u16(message.one(2, WIRE_U16)?)?,
            minimum_severity: DiagnosticSeverity::decode(read_u8(message.one(3, WIRE_U8)?)?)?,
        };
        if value.limit == 0 || value.limit > 256 {
            return Err(DecodeError::InvalidTlv);
        }
        Ok(value)
    }

    /// Return exact caller-buffer bytes for a typed diagnostics page.
    pub fn encoded_diagnostics_page_len(
        &self,
        value: &DiagnosticsPage,
    ) -> Result<usize, EncodeError> {
        validate_diagnostics_page(value)?;
        check_field_count(
            self,
            2_u32
                .checked_add(
                    u32::try_from(value.diagnostics.len())
                        .map_err(|_| EncodeError::LimitExceeded)?,
                )
                .ok_or(EncodeError::LimitExceeded)?,
        )?;
        let nested_path = value
            .diagnostics
            .iter()
            .any(|diagnostic| !diagnostic.path.is_empty());
        let required_nesting = if nested_path { 2 } else { 1 };
        if self.limits().max_nesting < required_nesting {
            return Err(EncodeError::LimitExceeded);
        }
        let mut result = checked_add(tlv_len(8)?, tlv_len(1)?)?;
        for diagnostic in &value.diagnostics {
            result = checked_add(
                result,
                tlv_len(self.encoded_diagnostic_message_len(diagnostic)?)?,
            )?;
        }
        Ok(result)
    }

    /// Encode a typed diagnostics page directly into caller output without allocation.
    pub fn encode_diagnostics_page(
        &self,
        value: &DiagnosticsPage,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_diagnostics_page_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = PayloadWriter::new(output, self.limits().max_tlv_count);
        writer.field(1, WIRE_U64, true, &value.last_sequence.to_le_bytes())?;
        writer.field(2, 8, true, &[u8::from(value.eof)])?;
        for diagnostic in &value.diagnostics {
            write_diagnostic_field(self, &mut writer, 3, true, diagnostic)?;
        }
        Ok(required)
    }

    /// Strictly decode a typed bounded diagnostics page.
    pub fn decode_diagnostics_page(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<DiagnosticsPage, DecodeError> {
        let message = top_level_message(self, payload, count)?;
        message.schema(&[Rule::one(1), Rule::one(2), Rule::repeated(3)])?;
        let diagnostics = message
            .values(3, WIRE_MESSAGE)?
            .map(|raw| decode_diagnostic(self, nested_message(self, raw, 1)?))
            .collect::<Result<Vec<_>, _>>()?;
        let value = DiagnosticsPage {
            last_sequence: read_u64(message.one(1, WIRE_U64)?)?,
            eof: read_bool(message.one(2, 8)?)?,
            diagnostics,
        };
        validate_diagnostics_page(&value).map_err(|_| DecodeError::InvalidTlv)?;
        Ok(value)
    }
}

#[derive(Clone, Copy)]
struct TransportField<'a> {
    wire: u8,
    mandatory: bool,
    value: &'a [u8],
}

struct TransportFields<'a> {
    values: [Option<TransportField<'a>>; 5],
}

fn scan_transport_fields<'a>(
    codec: &ProtocolCodec,
    payload: &'a [u8],
    count: u32,
    known: &[u16],
) -> Result<TransportFields<'a>, DecodeError> {
    if count > codec.limits().max_tlv_count || payload.len() > codec.limits().max_frame_bytes {
        return Err(DecodeError::LimitExceeded);
    }
    let mut fields = TransportFields { values: [None; 5] };
    let mut cursor = 0_usize;
    let mut prior = 0_u16;
    for index in 0..count {
        let prefix_end = cursor
            .checked_add(TLV_PREFIX_BYTES)
            .ok_or(DecodeError::LimitExceeded)?;
        let prefix = payload
            .get(cursor..prefix_end)
            .ok_or(DecodeError::Truncated)?;
        let id = u16::from_le_bytes(prefix[..2].try_into().map_err(|_| DecodeError::Truncated)?);
        let wire = prefix[2];
        let mandatory = prefix[3];
        if id == 0 || !(1..=15).contains(&wire) || mandatory & !1 != 0 || (index != 0 && id < prior)
        {
            return Err(DecodeError::InvalidTlv);
        }
        prior = id;
        let length = usize::try_from(u32::from_le_bytes(
            prefix[4..8]
                .try_into()
                .map_err(|_| DecodeError::Truncated)?,
        ))
        .map_err(|_| DecodeError::LimitExceeded)?;
        let value_start = prefix_end;
        let value_end = value_start
            .checked_add(length)
            .ok_or(DecodeError::LimitExceeded)?;
        let value = payload
            .get(value_start..value_end)
            .ok_or(DecodeError::Truncated)?;
        let padded_end = value_end
            .checked_add(padding(length))
            .ok_or(DecodeError::LimitExceeded)?;
        if payload
            .get(value_end..padded_end)
            .ok_or(DecodeError::Truncated)?
            .iter()
            .any(|byte| *byte != 0)
        {
            return Err(DecodeError::InvalidTlv);
        }
        if known.contains(&id) {
            let slot = fields
                .values
                .get_mut(usize::from(id.saturating_sub(1)))
                .ok_or(DecodeError::InvalidTlv)?;
            if slot
                .replace(TransportField {
                    wire,
                    mandatory: mandatory == 1,
                    value,
                })
                .is_some()
            {
                return Err(DecodeError::InvalidTlv);
            }
        } else if mandatory == 1 {
            return Err(DecodeError::UnknownRequiredField);
        }
        cursor = padded_end;
    }
    if cursor != payload.len() {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(fields)
}

fn transport_schema(
    fields: &TransportFields<'_>,
    rules: &[(u16, u8, bool)],
) -> Result<(), DecodeError> {
    for &(id, wire, mandatory) in rules {
        let Some(field) = fields
            .values
            .get(usize::from(id.saturating_sub(1)))
            .and_then(|value| *value)
        else {
            if mandatory {
                return Err(DecodeError::InvalidTlv);
            }
            continue;
        };
        if field.wire != wire || field.mandatory != mandatory {
            return Err(DecodeError::InvalidTlv);
        }
    }
    Ok(())
}

fn transport_required<'a>(
    fields: &'a TransportFields<'a>,
    id: u16,
) -> Result<&'a [u8], DecodeError> {
    fields
        .values
        .get(usize::from(id.saturating_sub(1)))
        .and_then(|value| *value)
        .map(|field| field.value)
        .ok_or(DecodeError::InvalidTlv)
}

fn transport_optional<'a>(fields: &'a TransportFields<'a>, id: u16) -> Option<&'a [u8]> {
    fields
        .values
        .get(usize::from(id.saturating_sub(1)))
        .and_then(|value| *value)
        .map(|field| field.value)
}

fn parse_transport_state(value: u8) -> Result<TransportState, DecodeError> {
    match value {
        1 => Ok(TransportState::Stopped),
        2 => Ok(TransportState::Playing),
        _ => Err(DecodeError::InvalidTlv),
    }
}

fn parse_automation_cancellation_reason(
    value: u8,
) -> Result<AutomationCancellationReason, DecodeError> {
    match value {
        1 => Ok(AutomationCancellationReason::RevisionChanged),
        2 => Ok(AutomationCancellationReason::TransportLocate),
        3 => Ok(AutomationCancellationReason::EndpointShutdown),
        4 => Ok(AutomationCancellationReason::ProviderUnavailable),
        5 => Ok(AutomationCancellationReason::ExplicitReconfiguration),
        _ => Err(DecodeError::InvalidTlv),
    }
}

fn parse_meter_component(value: u16) -> Result<MeterComponent, DecodeError> {
    match value {
        1 => Ok(MeterComponent::Left),
        2 => Ok(MeterComponent::Right),
        3 => Ok(MeterComponent::Aggregate),
        _ => Err(DecodeError::InvalidTlv),
    }
}

fn validate_meter_records(records: &[MeterRecord]) -> Result<(), EncodeError> {
    if records.is_empty() || records.len() > 256 {
        return Err(EncodeError::LimitExceeded);
    }
    for record in records {
        if record.handle == 0
            || record.flags & !0b111 != 0
            || (record.flags & 1 != 0 && !record.value.is_finite())
        {
            return Err(EncodeError::LimitExceeded);
        }
    }
    Ok(())
}

fn encode_meter_record(value: &MeterRecord, output: &mut [u8]) {
    debug_assert_eq!(output.len(), 16);
    output[..4].copy_from_slice(&value.handle.to_le_bytes());
    output[4..6].copy_from_slice(&(value.component as u16).to_le_bytes());
    output[6..8].copy_from_slice(&value.flags.to_le_bytes());
    output[8..12].copy_from_slice(&value.value.to_le_bytes());
    output[12..16].fill(0);
}

fn decode_meter_record(value: &[u8]) -> Result<MeterRecord, DecodeError> {
    if value.len() != 16 || value[12..].iter().any(|byte| *byte != 0) {
        return Err(DecodeError::InvalidTlv);
    }
    let record = MeterRecord {
        handle: u32::from_le_bytes(value[..4].try_into().map_err(|_| DecodeError::Truncated)?),
        component: parse_meter_component(u16::from_le_bytes(
            value[4..6].try_into().map_err(|_| DecodeError::Truncated)?,
        ))?,
        flags: u16::from_le_bytes(value[6..8].try_into().map_err(|_| DecodeError::Truncated)?),
        value: f32::from_le_bytes(
            value[8..12]
                .try_into()
                .map_err(|_| DecodeError::Truncated)?,
        ),
    };
    if record.handle == 0
        || record.flags & !0b111 != 0
        || (record.flags & 1 != 0 && !record.value.is_finite())
    {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(record)
}

fn parse_counter_id(value: u32) -> Result<CounterId, DecodeError> {
    match value {
        1 => Ok(CounterId::ControlCommandBackpressure),
        2 => Ok(CounterId::AutomationBackpressure),
        3 => Ok(CounterId::ReliableResponseBackpressure),
        4 => Ok(CounterId::ReliableEventBackpressure),
        5 => Ok(CounterId::TelemetryCoalesced),
        6 => Ok(CounterId::TelemetryDropped),
        7 => Ok(CounterId::MalformedFrames),
        8 => Ok(CounterId::ReplayHits),
        9 => Ok(CounterId::RequestIdReuse),
        10 => Ok(CounterId::ReplayExpired),
        11 => Ok(CounterId::LateAutomation),
        12 => Ok(CounterId::CanceledAutomation),
        13 => Ok(CounterId::AutomationTimePast),
        14 => Ok(CounterId::AutomationOrderReject),
        15 => Ok(CounterId::ValidationFailures),
        _ => Err(DecodeError::InvalidTlv),
    }
}

fn decode_nonzero_u32s(bytes: &[u8], allow_empty: bool) -> Result<Vec<u32>, DecodeError> {
    if !bytes.len().is_multiple_of(4) {
        return Err(DecodeError::InvalidValueLength);
    }
    let result = bytes
        .chunks_exact(4)
        .map(read_u32)
        .collect::<Result<Vec<_>, _>>()?;
    if (!allow_empty && result.is_empty())
        || result.len() > 256
        || result.contains(&0)
        || result.windows(2).any(|pair| pair[0] >= pair[1])
    {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(result)
}

fn decode_counter_ids(bytes: &[u8]) -> Result<Vec<CounterId>, DecodeError> {
    let values = decode_nonzero_u32s(bytes, true)?;
    values.into_iter().map(parse_counter_id).collect()
}

fn check_counter_ids(ids: &[CounterId], allow_empty: bool) -> Result<(), DecodeError> {
    if (!allow_empty && ids.is_empty())
        || ids.len() > 256
        || ids.windows(2).any(|pair| pair[0] >= pair[1])
    {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(())
}

fn validate_telemetry_configuration(value: &TelemetryConfiguration) -> Result<(), DecodeError> {
    if value.meter_handles.len() > 256
        || value.meter_handles.contains(&0)
        || value
            .meter_handles
            .windows(2)
            .any(|pair| pair[0] >= pair[1])
        || (!value.meter_handles.is_empty() && value.meter_period_blocks == 0)
        || (!value.counter_ids.is_empty() && value.counter_period_blocks == 0)
    {
        return Err(DecodeError::InvalidTlv);
    }
    check_counter_ids(&value.counter_ids, true)
}

fn validate_counters_request(value: &CountersRequest) -> Result<(), DecodeError> {
    if value.all != value.ids.is_empty() {
        return Err(DecodeError::InvalidTlv);
    }
    if value.ids.len() > 256
        || value.ids.contains(&0)
        || value.ids.windows(2).any(|pair| pair[0] >= pair[1])
    {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(())
}

fn validate_counter_snapshot(value: &CounterSnapshot) -> Result<(), DecodeError> {
    validate_counter_snapshot_ref(CounterSnapshotRef {
        observed_sample: value.observed_sample,
        values: &value.values,
    })
}

fn validate_counter_snapshot_ref(value: CounterSnapshotRef<'_>) -> Result<(), DecodeError> {
    if value.values.len() > 256 || value.values.windows(2).any(|pair| pair[0].id >= pair[1].id) {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(())
}

fn validate_diagnostics_page(value: &DiagnosticsPage) -> Result<(), EncodeError> {
    if value.diagnostics.len() > 256
        || value
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.provider_sequence.is_none())
        || value
            .diagnostics
            .windows(2)
            .any(|pair| pair[0].provider_sequence >= pair[1].provider_sequence)
        || value
            .diagnostics
            .last()
            .is_some_and(|diagnostic| diagnostic.provider_sequence != Some(value.last_sequence))
    {
        return Err(EncodeError::LimitExceeded);
    }
    Ok(())
}

fn write_counter_ids(
    writer: &mut PayloadWriter<'_>,
    id: u16,
    mandatory: bool,
    values: &[CounterId],
) -> Result<(), EncodeError> {
    let len = values
        .len()
        .checked_mul(4)
        .ok_or(EncodeError::LimitExceeded)?;
    let start = writer.position;
    writer.field(id, WIRE_PACKED_U32, mandatory, &[])?;
    let end = start
        .checked_add(tlv_len(len)?)
        .ok_or(EncodeError::LimitExceeded)?;
    let bytes = writer
        .output
        .get_mut(start..end)
        .ok_or(EncodeError::LimitExceeded)?;
    bytes[4..8].copy_from_slice(&(len as u32).to_le_bytes());
    for (index, value) in values.iter().enumerate() {
        let offset = TLV_PREFIX_BYTES + index * 4;
        bytes[offset..offset + 4].copy_from_slice(&(*value as u32).to_le_bytes());
    }
    bytes[TLV_PREFIX_BYTES + len..].fill(0);
    writer.position = end;
    Ok(())
}

fn write_u32s(
    writer: &mut PayloadWriter<'_>,
    id: u16,
    mandatory: bool,
    values: &[u32],
) -> Result<(), EncodeError> {
    let len = values
        .len()
        .checked_mul(4)
        .ok_or(EncodeError::LimitExceeded)?;
    let start = writer.position;
    writer.field(id, WIRE_PACKED_U32, mandatory, &[])?;
    let end = start
        .checked_add(tlv_len(len)?)
        .ok_or(EncodeError::LimitExceeded)?;
    let bytes = writer
        .output
        .get_mut(start..end)
        .ok_or(EncodeError::LimitExceeded)?;
    bytes[4..8].copy_from_slice(&(len as u32).to_le_bytes());
    for (index, value) in values.iter().enumerate() {
        let offset = TLV_PREFIX_BYTES + index * 4;
        bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    bytes[TLV_PREFIX_BYTES + len..].fill(0);
    writer.position = end;
    Ok(())
}

fn validate_automation_records(
    records: &[crate::AutomationRecord],
) -> Result<(), crate::AutomationBatchError> {
    crate::AutomationBatchSlot::new(
        crate::SessionRevision(0),
        crate::RequestId::new(1).expect("constant nonzero request ID"),
        records,
    )
    .map(|_| ())
}

fn write_automation_record_bytes(
    writer: &mut PayloadWriter<'_>,
    records: &[crate::AutomationRecord],
) -> Result<(), EncodeError> {
    let len = records
        .len()
        .checked_mul(crate::AUTOMATION_RECORD_BYTES)
        .ok_or(EncodeError::LimitExceeded)?;
    let start = writer.position;
    writer.field(3, 10, true, &[])?;
    let end = start
        .checked_add(tlv_len(len)?)
        .ok_or(EncodeError::LimitExceeded)?;
    let bytes = writer
        .output
        .get_mut(start..end)
        .ok_or(EncodeError::LimitExceeded)?;
    bytes[4..8].copy_from_slice(
        &u32::try_from(len)
            .map_err(|_| EncodeError::LimitExceeded)?
            .to_le_bytes(),
    );
    for (index, record) in records.iter().enumerate() {
        let offset = TLV_PREFIX_BYTES + index * crate::AUTOMATION_RECORD_BYTES;
        let destination: &mut [u8; crate::AUTOMATION_RECORD_BYTES] = bytes
            .get_mut(offset..offset + crate::AUTOMATION_RECORD_BYTES)
            .ok_or(EncodeError::LimitExceeded)?
            .try_into()
            .map_err(|_| EncodeError::LimitExceeded)?;
        record
            .encode_le(destination)
            .map_err(|_| EncodeError::LimitExceeded)?;
    }
    bytes[TLV_PREFIX_BYTES + len..].fill(0);
    writer.position = end;
    Ok(())
}

/// This parser is deliberately schema-specific: automation must not allocate a generic field
/// table merely to borrow its fixed record array.
fn decode_automation_enqueue<'a>(
    codec: &ProtocolCodec,
    payload: &'a [u8],
    count: u32,
) -> Result<DecodedAutomationEnqueue<'a>, DecodeError> {
    if count > codec.limits().max_tlv_count || payload.len() > codec.limits().max_frame_bytes {
        return Err(DecodeError::LimitExceeded);
    }
    let mut cursor = 0_usize;
    let mut prior = 0_u16;
    let mut record_count = None;
    let mut stride = None;
    let mut records = None;
    for index in 0..count {
        let prefix = payload
            .get(
                cursor
                    ..cursor
                        .checked_add(TLV_PREFIX_BYTES)
                        .ok_or(DecodeError::LimitExceeded)?,
            )
            .ok_or(DecodeError::Truncated)?;
        let id = u16::from_le_bytes(prefix[..2].try_into().map_err(|_| DecodeError::Truncated)?);
        let wire = prefix[2];
        let mandatory = prefix[3];
        if id == 0 || !(1..=15).contains(&wire) || mandatory & !1 != 0 || (index != 0 && id < prior)
        {
            return Err(DecodeError::InvalidTlv);
        }
        prior = id;
        let length = usize::try_from(u32::from_le_bytes(
            prefix[4..8]
                .try_into()
                .map_err(|_| DecodeError::Truncated)?,
        ))
        .map_err(|_| DecodeError::LimitExceeded)?;
        let value_start = cursor
            .checked_add(TLV_PREFIX_BYTES)
            .ok_or(DecodeError::LimitExceeded)?;
        let value_end = value_start
            .checked_add(length)
            .ok_or(DecodeError::LimitExceeded)?;
        let value = payload
            .get(value_start..value_end)
            .ok_or(DecodeError::Truncated)?;
        let padded_end = value_end
            .checked_add(padding(length))
            .ok_or(DecodeError::LimitExceeded)?;
        if payload
            .get(value_end..padded_end)
            .ok_or(DecodeError::Truncated)?
            .iter()
            .any(|byte| *byte != 0)
        {
            return Err(DecodeError::InvalidTlv);
        }
        match id {
            1 => {
                if mandatory != 1
                    || wire != WIRE_U16
                    || record_count.replace(read_u16(value)?).is_some()
                {
                    return Err(DecodeError::InvalidTlv);
                }
            }
            2 => {
                if mandatory != 1 || wire != WIRE_U16 || stride.replace(read_u16(value)?).is_some()
                {
                    return Err(DecodeError::InvalidTlv);
                }
            }
            3 => {
                if mandatory != 1 || wire != 10 || records.replace(value).is_some() {
                    return Err(DecodeError::InvalidTlv);
                }
            }
            _ if mandatory == 1 => return Err(DecodeError::UnknownRequiredField),
            _ => {}
        }
        cursor = padded_end;
    }
    if cursor != payload.len() {
        return Err(DecodeError::InvalidTlv);
    }
    let count = record_count.ok_or(DecodeError::InvalidTlv)?;
    let records = records.ok_or(DecodeError::InvalidTlv)?;
    if count == 0
        || usize::from(count) > crate::AUTOMATION_BATCH_RECORDS
        || stride != Some(crate::AUTOMATION_RECORD_BYTES as u16)
        || records.len()
            != usize::from(count)
                .checked_mul(crate::AUTOMATION_RECORD_BYTES)
                .ok_or(DecodeError::LimitExceeded)?
    {
        return Err(DecodeError::InvalidTlv);
    }
    let result = DecodedAutomationEnqueue {
        count,
        record_bytes: records,
    };
    let mut previous = None;
    for index in 0..usize::from(count) {
        let record = result.record(index)?;
        if previous.is_some_and(|prior: crate::AutomationRecord| {
            (record.start, record.handle) < (prior.start, prior.handle)
        }) {
            return Err(DecodeError::InvalidTlv);
        }
        previous = Some(record);
    }
    for left_index in 0..usize::from(count) {
        let left = result.record(left_index)?;
        let left_end = if left.kind == crate::AutomationKind::Point {
            left.start
        } else {
            left.end
        };
        for right_index in left_index + 1..usize::from(count) {
            let right = result.record(right_index)?;
            if left.handle != right.handle {
                continue;
            }
            let right_end = if right.kind == crate::AutomationKind::Point {
                right.start
            } else {
                right.end
            };
            if left.start == right.start || (left.start < right_end && right.start < left_end) {
                return Err(DecodeError::InvalidTlv);
            }
        }
    }
    Ok(result)
}

struct PayloadWriter<'a> {
    output: &'a mut [u8],
    position: usize,
}

impl<'a> PayloadWriter<'a> {
    fn new(output: &'a mut [u8], _maximum_fields: u32) -> Self {
        Self {
            output,
            position: 0,
        }
    }

    fn field(
        &mut self,
        id: u16,
        wire: u8,
        mandatory: bool,
        value: &[u8],
    ) -> Result<(), EncodeError> {
        let end = self
            .position
            .checked_add(tlv_len(value.len())?)
            .ok_or(EncodeError::LimitExceeded)?;
        let bytes = self
            .output
            .get_mut(self.position..end)
            .ok_or(EncodeError::LimitExceeded)?;
        bytes[..2].copy_from_slice(&id.to_le_bytes());
        bytes[2] = wire;
        bytes[3] = u8::from(mandatory);
        bytes[4..8].copy_from_slice(
            &u32::try_from(value.len())
                .map_err(|_| EncodeError::LimitExceeded)?
                .to_le_bytes(),
        );
        bytes[TLV_PREFIX_BYTES..TLV_PREFIX_BYTES + value.len()].copy_from_slice(value);
        bytes[TLV_PREFIX_BYTES + value.len()..].fill(0);
        self.position = end;
        Ok(())
    }

    fn packed_u16(&mut self, id: u16, values: &[u16]) -> Result<(), EncodeError> {
        let value_len = values
            .len()
            .checked_mul(2)
            .ok_or(EncodeError::LimitExceeded)?;
        let start = self.position;
        self.field(id, WIRE_PACKED_U16, true, &[])?;
        let end = start
            .checked_add(tlv_len(value_len)?)
            .ok_or(EncodeError::LimitExceeded)?;
        let bytes = self
            .output
            .get_mut(start..end)
            .ok_or(EncodeError::LimitExceeded)?;
        bytes[4..8].copy_from_slice(
            &u32::try_from(value_len)
                .map_err(|_| EncodeError::LimitExceeded)?
                .to_le_bytes(),
        );
        for (index, value) in values.iter().enumerate() {
            let offset = TLV_PREFIX_BYTES + index * 2;
            bytes[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
        }
        bytes[TLV_PREFIX_BYTES + value_len..].fill(0);
        self.position = end;
        Ok(())
    }

    fn packed_u32(&mut self, id: u16, values: &[u32]) -> Result<(), EncodeError> {
        let value_len = values
            .len()
            .checked_mul(4)
            .ok_or(EncodeError::LimitExceeded)?;
        let start = self.position;
        self.field(id, WIRE_PACKED_U32, true, &[])?;
        let end = start
            .checked_add(tlv_len(value_len)?)
            .ok_or(EncodeError::LimitExceeded)?;
        let bytes = self
            .output
            .get_mut(start..end)
            .ok_or(EncodeError::LimitExceeded)?;
        bytes[4..8].copy_from_slice(
            &u32::try_from(value_len)
                .map_err(|_| EncodeError::LimitExceeded)?
                .to_le_bytes(),
        );
        for (index, value) in values.iter().enumerate() {
            bytes[TLV_PREFIX_BYTES + index * 4..TLV_PREFIX_BYTES + index * 4 + 4]
                .copy_from_slice(&value.to_le_bytes());
        }
        bytes[TLV_PREFIX_BYTES + value_len..].fill(0);
        self.position = end;
        Ok(())
    }

    fn nested_start(
        &mut self,
        id: u16,
        mandatory: bool,
        body_len: usize,
        field_count: u32,
    ) -> Result<(), EncodeError> {
        let value_len = NESTED_HEADER_BYTES
            .checked_add(body_len)
            .ok_or(EncodeError::LimitExceeded)?;
        let start = self.position;
        self.field(id, WIRE_MESSAGE, mandatory, &[])?;
        let end = start
            .checked_add(tlv_len(value_len)?)
            .ok_or(EncodeError::LimitExceeded)?;
        let bytes = self
            .output
            .get_mut(start..end)
            .ok_or(EncodeError::LimitExceeded)?;
        bytes[4..8].copy_from_slice(
            &u32::try_from(value_len)
                .map_err(|_| EncodeError::LimitExceeded)?
                .to_le_bytes(),
        );
        bytes[8..12].copy_from_slice(&field_count.to_le_bytes());
        bytes[12..16].fill(0);
        self.position = start + TLV_PREFIX_BYTES + NESTED_HEADER_BYTES;
        Ok(())
    }

    fn finish_nested(&mut self, body_len: usize) -> Result<(), EncodeError> {
        let padding = padding(NESTED_HEADER_BYTES + body_len);
        let end = self
            .position
            .checked_add(padding)
            .ok_or(EncodeError::LimitExceeded)?;
        self.output
            .get_mut(self.position..end)
            .ok_or(EncodeError::LimitExceeded)?
            .fill(0);
        self.position = end;
        Ok(())
    }
}

fn write_non_ok(
    codec: &ProtocolCodec,
    writer: &mut PayloadWriter<'_>,
    value: &NonOkResponse,
) -> Result<(), EncodeError> {
    for diagnostic in &value.diagnostics {
        write_diagnostic_field(codec, writer, 1, true, diagnostic)?;
    }
    writer.field(2, WIRE_U32, true, &value.omitted_diagnostics.to_le_bytes())?;
    if let Some(backpressure) = value.backpressure {
        write_backpressure_field(codec, writer, 3, false, backpressure)?;
    }
    Ok(())
}

fn check_handles(handles: &[u32]) -> Result<(), DecodeError> {
    if handles.is_empty()
        || handles.len() > 256
        || handles.contains(&0)
        || handles.windows(2).any(|pair| pair[0] >= pair[1])
    {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(())
}
fn enum_choice_len(codec: &ProtocolCodec, choice: &EnumChoice) -> Result<usize, EncodeError> {
    if !choice.value.is_finite() {
        return Err(EncodeError::LimitExceeded);
    }
    check_string(codec, &choice.label)?;
    checked_add(
        NESTED_HEADER_BYTES,
        checked_add(tlv_len(4)?, tlv_len(choice.label.len())?)?,
    )
}
fn descriptor_len(
    codec: &ProtocolCodec,
    value: &ParameterDescriptor,
) -> Result<usize, EncodeError> {
    validate_descriptor(codec, value)?;
    let mut len = tlv_len(4)?;
    for fixed in [
        value.track_id.len(),
        1,
        value.effect_id.len(),
        4,
        1,
        1,
        1,
        1,
    ] {
        len = checked_add(len, tlv_len(fixed)?)?;
    }
    if value.minimum.is_some() {
        len = checked_add(len, tlv_len(4)?)?;
    }
    if value.maximum.is_some() {
        len = checked_add(len, tlv_len(4)?)?;
    }
    for fixed in [4, 1, 1, 4, 4] {
        len = checked_add(len, tlv_len(fixed)?)?;
    }
    if let Some(v) = &value.display_name {
        len = checked_add(len, tlv_len(v.len())?)?;
    }
    if let Some(v) = &value.display_unit {
        len = checked_add(len, tlv_len(v.len())?)?;
    }
    for choice in &value.enum_choices {
        len = checked_add(len, tlv_len(enum_choice_len(codec, choice)?)?)?;
    }
    checked_add(NESTED_HEADER_BYTES, len)
}
fn metadata_page_len(
    codec: &ProtocolCodec,
    value: &ParameterMetadataPage,
) -> Result<usize, EncodeError> {
    if value.descriptors.len() > 256
        || value
            .descriptors
            .windows(2)
            .any(|pair| pair[0].handle >= pair[1].handle)
        || value
            .descriptors
            .last()
            .is_some_and(|v| v.handle != value.last_handle)
        || value.descriptors.first().is_some_and(|v| v.handle == 0)
    {
        return Err(EncodeError::LimitExceeded);
    }
    let mut len = checked_add(tlv_len(4)?, tlv_len(1)?)?;
    for descriptor in &value.descriptors {
        len = checked_add(len, tlv_len(descriptor_len(codec, descriptor)?)?)?;
    }
    if len > codec.limits().max_frame_bytes
        || (2 + value.descriptors.len() as u32) > codec.limits().max_tlv_count
    {
        return Err(EncodeError::LimitExceeded);
    }
    Ok(len)
}
fn write_metadata_page(
    codec: &ProtocolCodec,
    writer: &mut PayloadWriter<'_>,
    value: &ParameterMetadataPage,
) -> Result<(), EncodeError> {
    writer.field(1, WIRE_U32, true, &value.last_handle.to_le_bytes())?;
    writer.field(2, 8, true, &[u8::from(value.eof)])?;
    for descriptor in &value.descriptors {
        write_descriptor(codec, writer, 3, descriptor)?;
    }
    Ok(())
}
fn write_descriptor(
    codec: &ProtocolCodec,
    writer: &mut PayloadWriter<'_>,
    id: u16,
    value: &ParameterDescriptor,
) -> Result<(), EncodeError> {
    let total = descriptor_len(codec, value)?;
    let body = total - NESTED_HEADER_BYTES;
    let fields = 14
        + u32::from(value.minimum.is_some())
        + u32::from(value.maximum.is_some())
        + u32::from(value.display_name.is_some())
        + u32::from(value.display_unit.is_some())
        + value.enum_choices.len() as u32;
    macro_rules! field {
        ($spec:expr, $bytes:expr) => {
            writer.field($spec.id, $spec.wire.raw(), $spec.mandatory, $bytes)
        };
    }
    writer.nested_start(id, true, body, fields)?;
    field!(descriptor::HANDLE, &value.handle.to_le_bytes())?;
    field!(descriptor::TRACK_ID, value.track_id.as_bytes())?;
    field!(descriptor::RACK, &[value.rack as u8])?;
    field!(descriptor::EFFECT_ID, value.effect_id.as_bytes())?;
    field!(descriptor::PARAMETER_ID, &value.parameter_id.to_le_bytes())?;
    field!(descriptor::CHANNEL, &[value.channel as u8])?;
    field!(descriptor::VALUE_KIND, &[value.value_kind as u8])?;
    field!(descriptor::UNIT, &[value.unit as u8])?;
    field!(descriptor::DOMAIN, &[value.domain as u8])?;
    if let Some(v) = value.minimum {
        field!(descriptor::MINIMUM, &v.to_le_bytes())?;
    }
    if let Some(v) = value.maximum {
        field!(descriptor::MAXIMUM, &v.to_le_bytes())?;
    }
    field!(descriptor::DEFAULT, &value.default.to_le_bytes())?;
    field!(descriptor::MAPPING, &[value.mapping as u8])?;
    field!(descriptor::AUTOMATION_RATE, &[value.automation_rate as u8])?;
    field!(
        descriptor::SMOOTHING_SAMPLES,
        &value.smoothing_samples.to_le_bytes()
    )?;
    field!(descriptor::FLAGS, &value.flags.to_le_bytes())?;
    if let Some(v) = &value.display_name {
        field!(descriptor::DISPLAY_NAME, v.as_bytes())?;
    }
    if let Some(v) = &value.display_unit {
        field!(descriptor::DISPLAY_UNIT, v.as_bytes())?;
    }
    for choice in &value.enum_choices {
        let total = enum_choice_len(codec, choice)?;
        writer.nested_start(
            descriptor::ENUM_CHOICE.id,
            descriptor::ENUM_CHOICE.mandatory,
            total - NESTED_HEADER_BYTES,
            2,
        )?;
        field!(enum_choice::VALUE, &choice.value.to_le_bytes())?;
        field!(enum_choice::LABEL, choice.label.as_bytes())?;
        writer.finish_nested(total - NESTED_HEADER_BYTES)?;
    }
    writer.finish_nested(body)
}
fn decode_metadata_page(
    codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<ParameterMetadataPage, DecodeError> {
    message.schema(&[Rule::one(1), Rule::one(2), Rule::repeated(3)])?;
    let descriptors = message
        .values(3, WIRE_MESSAGE)?
        .map(|v| decode_descriptor(codec, nested_message(codec, v, 1)?))
        .collect::<Result<Vec<_>, _>>()?;
    let page = ParameterMetadataPage {
        last_handle: read_u32(message.one(1, WIRE_U32)?)?,
        eof: read_bool(message.one(2, 8)?)?,
        descriptors,
    };
    if page.descriptors.len() > 256
        || page
            .descriptors
            .windows(2)
            .any(|p| p[0].handle >= p[1].handle)
        || page
            .descriptors
            .last()
            .is_some_and(|v| v.handle != page.last_handle)
    {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(page)
}
fn decode_descriptor(
    codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<ParameterDescriptor, DecodeError> {
    message.schema_spec(&descriptor::SPEC)?;
    let choices = message
        .values(19, WIRE_MESSAGE)?
        .map(|v| decode_choice(codec, nested_message(codec, v, 2)?))
        .collect::<Result<Vec<_>, _>>()?;
    let value = ParameterDescriptor {
        handle: read_u32(message.one(1, WIRE_U32)?)?,
        track_id: read_string(codec, message.one(2, WIRE_UTF8)?)?.to_owned(),
        rack: parse_rack(read_u8(message.one(3, WIRE_U8)?)?)?,
        effect_id: read_string(codec, message.one(4, WIRE_UTF8)?)?.to_owned(),
        parameter_id: read_u32(message.one(5, WIRE_U32)?)?,
        channel: parse_channel(read_u8(message.one(6, WIRE_U8)?)?)?,
        value_kind: parse_value_kind(read_u8(message.one(7, WIRE_U8)?)?)?,
        unit: parse_unit(read_u8(message.one(8, WIRE_U8)?)?)?,
        domain: parse_domain(read_u8(message.one(9, WIRE_U8)?)?)?,
        minimum: message
            .optional_one(10, WIRE_F32)?
            .map(read_f32)
            .transpose()?,
        maximum: message
            .optional_one(11, WIRE_F32)?
            .map(read_f32)
            .transpose()?,
        default: read_f32(message.one(12, WIRE_F32)?)?,
        mapping: parse_mapping(read_u8(message.one(13, WIRE_U8)?)?)?,
        automation_rate: parse_rate(read_u8(message.one(14, WIRE_U8)?)?)?,
        smoothing_samples: read_u32(message.one(15, WIRE_U32)?)?,
        flags: read_u32(message.one(16, WIRE_U32)?)?,
        display_name: message
            .optional_one(17, WIRE_UTF8)?
            .map(|v| read_string(codec, v).map(str::to_owned))
            .transpose()?,
        display_unit: message
            .optional_one(18, WIRE_UTF8)?
            .map(|v| read_string(codec, v).map(str::to_owned))
            .transpose()?,
        enum_choices: choices,
    };
    validate_descriptor_decode(codec, &value)?;
    Ok(value)
}
fn decode_choice(codec: &ProtocolCodec, message: Message<'_>) -> Result<EnumChoice, DecodeError> {
    message.schema_spec(&enum_choice::SPEC)?;
    let value = read_f32(message.one(1, WIRE_F32)?)?;
    if !value.is_finite() {
        return Err(DecodeError::InvalidTlv);
    };
    Ok(EnumChoice {
        value,
        label: read_string(codec, message.one(2, WIRE_UTF8)?)?.to_owned(),
    })
}
fn validate_descriptor(
    codec: &ProtocolCodec,
    value: &ParameterDescriptor,
) -> Result<(), EncodeError> {
    if value.handle == 0 || value.flags & !7 != 0 || !value.default.is_finite() {
        return Err(EncodeError::LimitExceeded);
    }
    check_string(codec, &value.track_id)?;
    check_string(codec, &value.effect_id)?;
    if !valid_stable_id(&value.track_id) || !valid_stable_id(&value.effect_id) {
        return Err(EncodeError::LimitExceeded);
    }
    if let Some(v) = &value.display_name {
        check_string(codec, v)?
    }
    if let Some(v) = &value.display_unit {
        check_string(codec, v)?
    }
    match value.domain {
        ParameterDomain::Continuous => match (value.minimum, value.maximum) {
            (Some(min), Some(max))
                if min.is_finite()
                    && max.is_finite()
                    && min <= value.default
                    && value.default <= max
                    && value.enum_choices.is_empty() => {}
            _ => return Err(EncodeError::LimitExceeded),
        },
        ParameterDomain::Boolean
            if value.minimum.is_none()
                && value.maximum.is_none()
                && value.enum_choices.is_empty()
                && (value.default == 0.0 || value.default == 1.0) => {}
        ParameterDomain::Enumeration
            if value.minimum.is_none()
                && value.maximum.is_none()
                && !value.enum_choices.is_empty()
                && value.enum_choices.iter().all(|c| c.value.is_finite())
                && value.enum_choices.iter().any(|c| c.value == value.default)
                && value
                    .enum_choices
                    .iter()
                    .enumerate()
                    .all(|(i, c)| value.enum_choices[..i].iter().all(|p| p.value != c.value)) => {}
        _ => return Err(EncodeError::LimitExceeded),
    }
    Ok(())
}
fn validate_descriptor_decode(
    codec: &ProtocolCodec,
    value: &ParameterDescriptor,
) -> Result<(), DecodeError> {
    validate_descriptor(codec, value).map_err(|_| DecodeError::InvalidTlv)
}
fn parse_domain(v: u8) -> Result<ParameterDomain, DecodeError> {
    match v {
        1 => Ok(ParameterDomain::Continuous),
        2 => Ok(ParameterDomain::Boolean),
        3 => Ok(ParameterDomain::Enumeration),
        _ => Err(DecodeError::InvalidTlv),
    }
}
fn parse_value_kind(v: u8) -> Result<ParameterValueKind, DecodeError> {
    if v == 1 {
        Ok(ParameterValueKind::F32)
    } else {
        Err(DecodeError::InvalidTlv)
    }
}
fn parse_rack(v: u8) -> Result<ParameterRack, DecodeError> {
    match v {
        1 => Ok(ParameterRack::Simd1),
        2 => Ok(ParameterRack::Dynamic),
        3 => Ok(ParameterRack::Simd2),
        _ => Err(DecodeError::InvalidTlv),
    }
}
fn parse_channel(v: u8) -> Result<ParameterChannel, DecodeError> {
    match v {
        1 => Ok(ParameterChannel::Left),
        2 => Ok(ParameterChannel::Right),
        3 => Ok(ParameterChannel::Both),
        _ => Err(DecodeError::InvalidTlv),
    }
}
fn parse_unit(v: u8) -> Result<ParameterUnit, DecodeError> {
    match v {
        1 => Ok(ParameterUnit::Db),
        2 => Ok(ParameterUnit::Hz),
        3 => Ok(ParameterUnit::Milliseconds),
        4 => Ok(ParameterUnit::Samples),
        5 => Ok(ParameterUnit::Linear),
        6 => Ok(ParameterUnit::Ratio),
        _ => Err(DecodeError::InvalidTlv),
    }
}
fn parse_mapping(v: u8) -> Result<ParameterMapping, DecodeError> {
    match v {
        1 => Ok(ParameterMapping::Linear),
        2 => Ok(ParameterMapping::Logarithmic),
        3 => Ok(ParameterMapping::Exponential),
        4 => Ok(ParameterMapping::Stepped),
        _ => Err(DecodeError::InvalidTlv),
    }
}
fn parse_rate(v: u8) -> Result<ParameterAutomationRate, DecodeError> {
    match v {
        1 => Ok(ParameterAutomationRate::Sample),
        2 => Ok(ParameterAutomationRate::Block),
        3 => Ok(ParameterAutomationRate::None),
        _ => Err(DecodeError::InvalidTlv),
    }
}
fn state_page_len(codec: &ProtocolCodec, value: &ParameterStatePage) -> Result<usize, EncodeError> {
    if value.records.len() > 256 {
        return Err(EncodeError::LimitExceeded);
    }
    for record in &value.records {
        validate_state_record(record).map_err(|_| EncodeError::LimitExceeded)?;
    }
    let bytes = value
        .records
        .len()
        .checked_mul(16)
        .ok_or(EncodeError::LimitExceeded)?;
    let len = checked_add(
        checked_add(tlv_len(8)?, tlv_len(2)?)?,
        checked_add(tlv_len(2)?, tlv_len(bytes)?)?,
    )?;
    if len > codec.limits().max_frame_bytes {
        return Err(EncodeError::LimitExceeded);
    }
    Ok(len)
}
fn write_state_record_bytes(
    writer: &mut PayloadWriter<'_>,
    records: &[ParameterStateRecord],
) -> Result<(), EncodeError> {
    let len = records
        .len()
        .checked_mul(16)
        .ok_or(EncodeError::LimitExceeded)?;
    let start = writer.position;
    writer.field(4, 10, true, &[])?;
    let end = start
        .checked_add(tlv_len(len)?)
        .ok_or(EncodeError::LimitExceeded)?;
    let bytes = writer
        .output
        .get_mut(start..end)
        .ok_or(EncodeError::LimitExceeded)?;
    bytes[4..8].copy_from_slice(&(len as u32).to_le_bytes());
    for (i, r) in records.iter().enumerate() {
        let o = 8 + i * 16;
        bytes[o..o + 4].copy_from_slice(&r.handle.to_le_bytes());
        bytes[o + 4..o + 8].copy_from_slice(&r.flags.to_le_bytes());
        bytes[o + 8..o + 12].copy_from_slice(&r.value.to_le_bytes());
        bytes[o + 12..o + 16].fill(0);
    }
    bytes[8 + len..].fill(0);
    writer.position = end;
    Ok(())
}
fn decode_state_page(
    _codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<ParameterStatePage, DecodeError> {
    message.schema(&[Rule::one(1), Rule::one(2), Rule::one(3), Rule::one(4)])?;
    let count = read_u16(message.one(2, WIRE_U16)?)? as usize;
    if count > 256 || read_u16(message.one(3, WIRE_U16)?)? != 16 {
        return Err(DecodeError::InvalidTlv);
    }
    let bytes = message.one(4, 10)?;
    if bytes.len() != count.checked_mul(16).ok_or(DecodeError::LimitExceeded)? {
        return Err(DecodeError::InvalidValueLength);
    }
    let mut records = Vec::with_capacity(count);
    for raw in bytes.chunks_exact(16) {
        if raw[12..].iter().any(|v| *v != 0) {
            return Err(DecodeError::NonzeroReserved);
        }
        let record = ParameterStateRecord {
            handle: read_u32(&raw[..4])?,
            flags: read_u32(&raw[4..8])?,
            value: read_f32(&raw[8..12])?,
        };
        validate_state_record(&record)?;
        records.push(record)
    }
    Ok(ParameterStatePage {
        observed_sample: read_u64(message.one(1, WIRE_U64)?)?,
        records,
    })
}
fn validate_state_record(value: &ParameterStateRecord) -> Result<(), DecodeError> {
    if value.handle == 0 || value.flags & !3 != 0 {
        return Err(DecodeError::InvalidTlv);
    }
    let valid = value.flags & 1 != 0;
    let active = value.flags & 2 != 0;
    if (!valid && (!value.value.is_sign_positive() || value.value != 0.0 || active))
        || (valid && !value.value.is_finite())
    {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(())
}

fn capabilities_len(codec: &ProtocolCodec, value: &Capabilities<'_>) -> Result<usize, EncodeError> {
    check_capabilities(value)?;
    let command_bytes = value
        .supported_commands
        .len()
        .checked_mul(2)
        .ok_or(EncodeError::LimitExceeded)?;
    let event_bytes = value
        .supported_events
        .len()
        .checked_mul(2)
        .ok_or(EncodeError::LimitExceeded)?;
    let lengths = [
        2_usize,
        2,
        2,
        2,
        8,
        4,
        8,
        1,
        2,
        8,
        8,
        8,
        8,
        8,
        8,
        8,
        8,
        8,
        8,
        8,
        2,
        2,
        2,
        4,
        command_bytes,
        event_bytes,
        8,
    ];
    let mut result = 0_usize;
    for length in lengths {
        result = checked_add(result, tlv_len(length)?)?;
    }
    if result > codec.limits().max_frame_bytes || 27 > codec.limits().max_tlv_count {
        return Err(EncodeError::LimitExceeded);
    }
    Ok(result)
}

fn write_capabilities(
    writer: &mut PayloadWriter<'_>,
    value: &Capabilities<'_>,
) -> Result<(), EncodeError> {
    writer.field(
        1,
        WIRE_U16,
        true,
        &value.minimum_version.major.to_le_bytes(),
    )?;
    writer.field(
        2,
        WIRE_U16,
        true,
        &value.minimum_version.minor.to_le_bytes(),
    )?;
    writer.field(
        3,
        WIRE_U16,
        true,
        &value.maximum_version.major.to_le_bytes(),
    )?;
    writer.field(
        4,
        WIRE_U16,
        true,
        &value.maximum_version.minor.to_le_bytes(),
    )?;
    writer.field(5, WIRE_U64, true, &value.maximum_frame_bytes.to_le_bytes())?;
    writer.field(6, WIRE_U32, true, &value.maximum_tlvs.to_le_bytes())?;
    writer.field(7, WIRE_U64, true, &value.maximum_string_bytes.to_le_bytes())?;
    writer.field(8, WIRE_U8, true, &[value.maximum_nesting])?;
    writer.field(
        9,
        WIRE_U16,
        true,
        &value.maximum_automation_records.to_le_bytes(),
    )?;
    for (id, field) in [
        (10, value.control_command_slots),
        (11, value.control_command_bytes),
        (12, value.automation_batch_slots),
        (13, value.reliable_response_slots),
        (14, value.reliable_event_slots),
        (15, value.telemetry_slots),
        (16, value.replay_entries),
        (17, value.replay_bytes),
        (18, value.maximum_cached_response_bytes),
        (19, value.per_block_automation_density),
        (20, value.admission_quantum_frames),
    ] {
        writer.field(id, WIRE_U64, true, &field.to_le_bytes())?;
    }
    writer.field(
        21,
        WIRE_U16,
        true,
        &value.maximum_parameter_page_items.to_le_bytes(),
    )?;
    writer.field(
        22,
        WIRE_U16,
        true,
        &value.maximum_diagnostic_page_items.to_le_bytes(),
    )?;
    writer.field(
        23,
        WIRE_U16,
        true,
        &value.maximum_telemetry_handles.to_le_bytes(),
    )?;
    writer.field(
        24,
        WIRE_U32,
        true,
        &value.maximum_transaction_edits.to_le_bytes(),
    )?;
    writer.packed_u16(25, value.supported_commands)?;
    writer.packed_u16(26, value.supported_events)?;
    writer.field(27, WIRE_U64, true, &value.flags.0.to_le_bytes())?;
    Ok(())
}

fn decode_capabilities<'a>(
    codec: &ProtocolCodec,
    message: Message<'a>,
) -> Result<DecodedCapabilities<'a>, DecodeError> {
    message.schema(&[
        Rule::one(1),
        Rule::one(2),
        Rule::one(3),
        Rule::one(4),
        Rule::one(5),
        Rule::one(6),
        Rule::one(7),
        Rule::one(8),
        Rule::one(9),
        Rule::one(10),
        Rule::one(11),
        Rule::one(12),
        Rule::one(13),
        Rule::one(14),
        Rule::one(15),
        Rule::one(16),
        Rule::one(17),
        Rule::one(18),
        Rule::one(19),
        Rule::one(20),
        Rule::one(21),
        Rule::one(22),
        Rule::one(23),
        Rule::one(24),
        Rule::one(25),
        Rule::one(26),
        Rule::one(27),
    ])?;
    let value = DecodedCapabilities {
        minimum_version: crate::ProtocolVersion {
            major: read_u16(message.one(1, WIRE_U16)?)?,
            minor: read_u16(message.one(2, WIRE_U16)?)?,
        },
        maximum_version: crate::ProtocolVersion {
            major: read_u16(message.one(3, WIRE_U16)?)?,
            minor: read_u16(message.one(4, WIRE_U16)?)?,
        },
        maximum_frame_bytes: read_u64(message.one(5, WIRE_U64)?)?,
        maximum_tlvs: read_u32(message.one(6, WIRE_U32)?)?,
        maximum_string_bytes: read_u64(message.one(7, WIRE_U64)?)?,
        maximum_nesting: read_u8(message.one(8, WIRE_U8)?)?,
        maximum_automation_records: read_u16(message.one(9, WIRE_U16)?)?,
        control_command_slots: read_u64(message.one(10, WIRE_U64)?)?,
        control_command_bytes: read_u64(message.one(11, WIRE_U64)?)?,
        automation_batch_slots: read_u64(message.one(12, WIRE_U64)?)?,
        reliable_response_slots: read_u64(message.one(13, WIRE_U64)?)?,
        reliable_event_slots: read_u64(message.one(14, WIRE_U64)?)?,
        telemetry_slots: read_u64(message.one(15, WIRE_U64)?)?,
        replay_entries: read_u64(message.one(16, WIRE_U64)?)?,
        replay_bytes: read_u64(message.one(17, WIRE_U64)?)?,
        maximum_cached_response_bytes: read_u64(message.one(18, WIRE_U64)?)?,
        per_block_automation_density: read_u64(message.one(19, WIRE_U64)?)?,
        admission_quantum_frames: read_u64(message.one(20, WIRE_U64)?)?,
        maximum_parameter_page_items: read_u16(message.one(21, WIRE_U16)?)?,
        maximum_diagnostic_page_items: read_u16(message.one(22, WIRE_U16)?)?,
        maximum_telemetry_handles: read_u16(message.one(23, WIRE_U16)?)?,
        maximum_transaction_edits: read_u32(message.one(24, WIRE_U32)?)?,
        supported_commands: message.one(25, WIRE_PACKED_U16)?,
        supported_events: message.one(26, WIRE_PACKED_U16)?,
        flags: CapabilityFlags(read_u64(message.one(27, WIRE_U64)?)?),
    };
    if !value.supported_commands.len().is_multiple_of(2)
        || !value.supported_events.len().is_multiple_of(2)
    {
        return Err(DecodeError::InvalidValueLength);
    }
    check_capabilities_decode(&value)?;
    let _ = codec;
    Ok(value)
}

fn check_capabilities(value: &Capabilities<'_>) -> Result<(), EncodeError> {
    check_capabilities_common(value).map_err(|_| EncodeError::LimitExceeded)
}
fn check_capabilities_decode(value: &DecodedCapabilities<'_>) -> Result<(), DecodeError> {
    let has_command = |wanted| {
        value
            .supported_commands
            .chunks_exact(2)
            .any(|id| u16::from_le_bytes([id[0], id[1]]) == wanted)
    };
    let has_event = |wanted| {
        value
            .supported_events
            .chunks_exact(2)
            .any(|id| u16::from_le_bytes([id[0], id[1]]) == wanted)
    };
    let session_family = has_command(0x0003) && has_event(0x8001) && has_event(0x8002);
    if value.minimum_version.major != 1
        || value.maximum_version.major != 1
        || value.minimum_version.minor > value.maximum_version.minor
        || value.maximum_frame_bytes == 0
        || value.maximum_tlvs < 27
        || value.maximum_nesting == 0
        || value.maximum_automation_records != 256
        || value.maximum_parameter_page_items > 256
        || value.maximum_diagnostic_page_items > 256
        || ((value.maximum_transaction_edits != 0) != session_family)
        || value.flags.0 & !CapabilityFlags::KNOWN != 0
        || !strict_ids(value.supported_commands, false)
        || !strict_ids(value.supported_events, true)
        || ((value.flags.0 & (1 << 3) != 0) != session_family)
        || ((value.flags.0 & (1 << 4) != 0) != has_command(0x0002))
        || ((value.flags.0 & (1 << 5) != 0) != has_command(0x0006))
        || ((value.flags.0 & (1 << 6) != 0) != has_command(0x0009))
        || ((value.flags.0 & (1 << 7) != 0) != (has_command(0x0004) && has_command(0x0005)))
        || ((value.flags.0 & (1 << 8) != 0) != has_command(0x0007))
        || ((value.flags.0 & (1 << 9) != 0) != has_event(0x8020))
        || ((value.flags.0 & (1 << 10) != 0) != (has_command(0x000a) && has_event(0x8021)))
        || ((value.flags.0 & (1 << 11) != 0) != (has_command(0x000b) && has_event(0x8030)))
        || ((value.flags.0 & CapabilityFlags::SESSION_EVENT_STREAM.0 != 0) != session_family)
        || ((value.flags.0 & (1 << 13) != 0) != (has_command(0x0008) && has_event(0x8010)))
        || (has_command(0x0003) != has_event(0x8001))
        || (has_event(0x8001) != has_event(0x8002))
        || (has_command(0x0004) != has_command(0x0005))
        || (has_command(0x0008) != has_event(0x8010))
        || (has_command(0x000a) != has_event(0x8021))
        || (has_command(0x000b) != has_event(0x8030))
    {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(())
}
fn check_capabilities_common(value: &Capabilities<'_>) -> Result<(), DecodeError> {
    let has_command = |wanted| value.supported_commands.contains(&wanted);
    let has_event = |wanted| value.supported_events.contains(&wanted);
    let session_family = has_command(0x0003) && has_event(0x8001) && has_event(0x8002);
    if value.minimum_version.major != 1
        || value.maximum_version.major != 1
        || value.minimum_version.minor > value.maximum_version.minor
        || value.maximum_frame_bytes == 0
        || value.maximum_tlvs < 27
        || value.maximum_nesting == 0
        || value.maximum_automation_records != 256
        || value.maximum_parameter_page_items > 256
        || value.maximum_diagnostic_page_items > 256
        || ((value.maximum_transaction_edits != 0) != session_family)
        || value.flags.0 & !CapabilityFlags::KNOWN != 0
        || !strict_u16_ids(value.supported_commands, false)
        || !strict_u16_ids(value.supported_events, true)
        || ((value.flags.0 & (1 << 3) != 0) != session_family)
        || ((value.flags.0 & (1 << 4) != 0) != has_command(0x0002))
        || ((value.flags.0 & (1 << 5) != 0) != has_command(0x0006))
        || ((value.flags.0 & (1 << 6) != 0) != has_command(0x0009))
        || ((value.flags.0 & (1 << 7) != 0) != (has_command(0x0004) && has_command(0x0005)))
        || ((value.flags.0 & (1 << 8) != 0) != has_command(0x0007))
        || ((value.flags.0 & (1 << 9) != 0) != has_event(0x8020))
        || ((value.flags.0 & (1 << 10) != 0) != (has_command(0x000a) && has_event(0x8021)))
        || ((value.flags.0 & (1 << 11) != 0) != (has_command(0x000b) && has_event(0x8030)))
        || ((value.flags.0 & CapabilityFlags::SESSION_EVENT_STREAM.0 != 0) != session_family)
        || ((value.flags.0 & (1 << 13) != 0) != (has_command(0x0008) && has_event(0x8010)))
        || (has_command(0x0003) != has_event(0x8001))
        || (has_event(0x8001) != has_event(0x8002))
        || (has_command(0x0004) != has_command(0x0005))
        || (has_command(0x0008) != has_event(0x8010))
        || (has_command(0x000a) != has_event(0x8021))
        || (has_command(0x000b) != has_event(0x8030))
    {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(())
}
fn strict_ids(bytes: &[u8], events: bool) -> bool {
    let mut prior = None;
    for item in bytes.chunks_exact(2) {
        let id = u16::from_le_bytes([item[0], item[1]]);
        if !allocated_id(id, events) || prior.is_some_and(|previous| id <= previous) {
            return false;
        }
        prior = Some(id);
    }
    true
}
fn strict_u16_ids(ids: &[u16], events: bool) -> bool {
    ids.iter()
        .copied()
        .enumerate()
        .all(|(index, id)| allocated_id(id, events) && (index == 0 || id > ids[index - 1]))
}
fn allocated_id(id: u16, event: bool) -> bool {
    if event {
        matches!(id, 0x8001 | 0x8002 | 0x8010 | 0x8020 | 0x8021 | 0x8030)
    } else {
        matches!(id, 1..=11)
    }
}

fn write_diagnostic_field(
    codec: &ProtocolCodec,
    writer: &mut PayloadWriter<'_>,
    id: u16,
    mandatory: bool,
    value: &Diagnostic,
) -> Result<(), EncodeError> {
    let body_len = diagnostic_body_len(codec, value)?;
    writer.nested_start(id, mandatory, body_len, diagnostic_field_count(value)?)?;
    write_diagnostic_body(codec, writer, value)?;
    writer.finish_nested(body_len)
}

fn write_diagnostic_body(
    codec: &ProtocolCodec,
    writer: &mut PayloadWriter<'_>,
    value: &Diagnostic,
) -> Result<(), EncodeError> {
    check_diagnostic(codec, value)?;
    writer.field(1, WIRE_UTF8, true, value.code.as_bytes())?;
    writer.field(2, WIRE_U8, true, &[value.severity as u8])?;
    for segment in &value.path {
        write_path_segment_field(codec, writer, 3, true, segment)?;
    }
    if let Some(detail) = &value.detail {
        writer.field(4, WIRE_UTF8, false, detail.as_bytes())?;
    }
    if let Some(index) = value.operation_index {
        writer.field(5, WIRE_U32, false, &index.to_le_bytes())?;
    }
    if let Some(sample) = value.sample_time {
        writer.field(6, WIRE_U64, false, &sample.to_le_bytes())?;
    }
    if let Some(sequence) = value.provider_sequence {
        writer.field(7, WIRE_U64, false, &sequence.to_le_bytes())?;
    }
    Ok(())
}

fn write_path_segment_field(
    codec: &ProtocolCodec,
    writer: &mut PayloadWriter<'_>,
    id: u16,
    mandatory: bool,
    value: &PathSegment,
) -> Result<(), EncodeError> {
    let body_len = path_segment_body_len(codec, value)?;
    writer.nested_start(id, mandatory, body_len, 2)?;
    writer.field(1, WIRE_U8, true, &[path_segment_tag(value)])?;
    match value {
        PathSegment::Field(field) => writer.field(2, WIRE_UTF8, false, field.as_bytes())?,
        PathSegment::Index(index) => writer.field(3, WIRE_U64, false, &index.to_le_bytes())?,
        PathSegment::StableId(id) => writer.field(4, WIRE_UTF8, false, id.as_bytes())?,
    }
    writer.finish_nested(body_len)
}

fn write_backpressure_field(
    codec: &ProtocolCodec,
    writer: &mut PayloadWriter<'_>,
    id: u16,
    mandatory: bool,
    value: Backpressure,
) -> Result<(), EncodeError> {
    let body_len = backpressure_body_len(codec, value)?;
    writer.nested_start(id, mandatory, body_len, backpressure_field_count(value)?)?;
    writer.field(1, WIRE_U8, true, &[value.queue_kind as u8])?;
    writer.field(2, WIRE_U64, true, &value.capacity.to_le_bytes())?;
    writer.field(3, WIRE_U64, true, &value.occupancy.to_le_bytes())?;
    writer.field(4, WIRE_U16, true, &value.requested_items.to_le_bytes())?;
    if let Some(generation) = value.generation {
        writer.field(5, WIRE_U64, false, &generation.to_le_bytes())?;
    }
    if let Some(boundary) = value.retry_boundary {
        writer.field(6, WIRE_U64, false, &boundary.to_le_bytes())?;
    }
    if let Some(bytes) = value.requested_bytes {
        writer.field(7, WIRE_U64, false, &bytes.to_le_bytes())?;
    }
    if let Some(bytes) = value.available_bytes {
        writer.field(8, WIRE_U64, false, &bytes.to_le_bytes())?;
    }
    writer.finish_nested(body_len)
}

fn non_ok_len(codec: &ProtocolCodec, value: &NonOkResponse) -> Result<usize, EncodeError> {
    let count = non_ok_field_count(value)?;
    check_field_count(codec, count)?;
    check_non_ok_nesting(codec, value)?;
    let mut result = tlv_len(4)?;
    for diagnostic in &value.diagnostics {
        result = checked_add(result, tlv_len(diagnostic_message_len(codec, diagnostic)?)?)?;
    }
    if let Some(backpressure) = value.backpressure {
        result = checked_add(
            result,
            tlv_len(backpressure_message_len(codec, backpressure)?)?,
        )?;
    }
    Ok(result)
}

fn diagnostic_message_len(codec: &ProtocolCodec, value: &Diagnostic) -> Result<usize, EncodeError> {
    checked_add(NESTED_HEADER_BYTES, diagnostic_body_len(codec, value)?)
}

fn diagnostic_body_len(codec: &ProtocolCodec, value: &Diagnostic) -> Result<usize, EncodeError> {
    check_diagnostic(codec, value)?;
    check_field_count(codec, diagnostic_field_count(value)?)?;
    let mut result = checked_add(tlv_len(value.code.len())?, tlv_len(1)?)?;
    for segment in &value.path {
        result = checked_add(result, tlv_len(path_segment_message_len(codec, segment)?)?)?;
    }
    if let Some(detail) = &value.detail {
        result = checked_add(result, tlv_len(detail.len())?)?;
    }
    if value.operation_index.is_some() {
        result = checked_add(result, tlv_len(4)?)?;
    }
    if value.sample_time.is_some() {
        result = checked_add(result, tlv_len(8)?)?;
    }
    if value.provider_sequence.is_some() {
        result = checked_add(result, tlv_len(8)?)?;
    }
    Ok(result)
}

fn path_segment_message_len(
    codec: &ProtocolCodec,
    value: &PathSegment,
) -> Result<usize, EncodeError> {
    checked_add(NESTED_HEADER_BYTES, path_segment_body_len(codec, value)?)
}

fn path_segment_body_len(codec: &ProtocolCodec, value: &PathSegment) -> Result<usize, EncodeError> {
    let value_len = match value {
        PathSegment::Field(field) => {
            check_string(codec, field)?;
            field.len()
        }
        PathSegment::Index(_) => 8,
        PathSegment::StableId(id) => {
            check_string(codec, id)?;
            if !valid_stable_id(id) {
                return Err(EncodeError::LimitExceeded);
            }
            id.len()
        }
    };
    checked_add(tlv_len(1)?, tlv_len(value_len)?)
}

fn backpressure_message_len(
    codec: &ProtocolCodec,
    value: Backpressure,
) -> Result<usize, EncodeError> {
    checked_add(NESTED_HEADER_BYTES, backpressure_body_len(codec, value)?)
}

fn backpressure_body_len(codec: &ProtocolCodec, value: Backpressure) -> Result<usize, EncodeError> {
    check_backpressure(value)?;
    check_field_count(codec, backpressure_field_count(value)?)?;
    let mut result = checked_add(
        tlv_len(1)?,
        checked_add(tlv_len(8)?, checked_add(tlv_len(8)?, tlv_len(2)?)?)?,
    )?;
    for present in [
        value.generation.is_some(),
        value.retry_boundary.is_some(),
        value.requested_bytes.is_some(),
        value.available_bytes.is_some(),
    ] {
        if present {
            result = checked_add(result, tlv_len(8)?)?;
        }
    }
    Ok(result)
}

fn non_ok_field_count(value: &NonOkResponse) -> Result<u32, EncodeError> {
    u32::try_from(value.diagnostics.len())
        .map_err(|_| EncodeError::LimitExceeded)?
        .checked_add(1)
        .and_then(|count| count.checked_add(u32::from(value.backpressure.is_some())))
        .ok_or(EncodeError::LimitExceeded)
}

fn diagnostic_field_count(value: &Diagnostic) -> Result<u32, EncodeError> {
    u32::try_from(value.path.len())
        .map_err(|_| EncodeError::LimitExceeded)?
        .checked_add(2)
        .and_then(|count| count.checked_add(u32::from(value.detail.is_some())))
        .and_then(|count| count.checked_add(u32::from(value.operation_index.is_some())))
        .and_then(|count| count.checked_add(u32::from(value.sample_time.is_some())))
        .and_then(|count| count.checked_add(u32::from(value.provider_sequence.is_some())))
        .ok_or(EncodeError::LimitExceeded)
}

fn backpressure_field_count(value: Backpressure) -> Result<u32, EncodeError> {
    4_u32
        .checked_add(u32::from(value.generation.is_some()))
        .and_then(|count| count.checked_add(u32::from(value.retry_boundary.is_some())))
        .and_then(|count| count.checked_add(u32::from(value.requested_bytes.is_some())))
        .and_then(|count| count.checked_add(u32::from(value.available_bytes.is_some())))
        .ok_or(EncodeError::LimitExceeded)
}

fn check_diagnostic(codec: &ProtocolCodec, value: &Diagnostic) -> Result<(), EncodeError> {
    check_string(codec, &value.code)?;
    if !valid_dotted_code(&value.code) {
        return Err(EncodeError::LimitExceeded);
    }
    if let Some(detail) = &value.detail {
        check_string(codec, detail)?;
    }
    for segment in &value.path {
        let _ = path_segment_body_len(codec, segment)?;
    }
    Ok(())
}

fn check_non_ok_nesting(codec: &ProtocolCodec, value: &NonOkResponse) -> Result<(), EncodeError> {
    let nested_diagnostic = !value.diagnostics.is_empty();
    let nested_backpressure = value.backpressure.is_some();
    let nested_path = value
        .diagnostics
        .iter()
        .any(|diagnostic| !diagnostic.path.is_empty());
    let required_depth = if nested_path {
        2
    } else if nested_diagnostic || nested_backpressure {
        1
    } else {
        0
    };
    if codec.limits().max_nesting < required_depth {
        return Err(EncodeError::LimitExceeded);
    }
    Ok(())
}

fn check_backpressure(value: Backpressure) -> Result<(), EncodeError> {
    if value.capacity == 0
        || value.occupancy > value.capacity
        || value.requested_items != 1
        || value.queue_kind.has_generation() != value.generation.is_some()
        || value.requested_bytes.is_some() != value.available_bytes.is_some()
    {
        return Err(EncodeError::LimitExceeded);
    }
    Ok(())
}

fn check_string(codec: &ProtocolCodec, value: &str) -> Result<(), EncodeError> {
    if value.len() > codec.limits().max_string_bytes {
        return Err(EncodeError::LimitExceeded);
    }
    Ok(())
}

fn check_field_count(codec: &ProtocolCodec, count: u32) -> Result<(), EncodeError> {
    if count > codec.limits().max_tlv_count {
        return Err(EncodeError::LimitExceeded);
    }
    Ok(())
}

fn path_segment_tag(value: &PathSegment) -> u8 {
    match value {
        PathSegment::Field(_) => 1,
        PathSegment::Index(_) => 2,
        PathSegment::StableId(_) => 3,
    }
}

fn decode_non_ok(
    codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<NonOkResponse, DecodeError> {
    message.schema(&[Rule::repeated(1), Rule::one(2), Rule::optional(3)])?;
    let diagnostics = message
        .values(1, WIRE_MESSAGE)?
        .map(|value| decode_diagnostic(codec, nested_message(codec, value, 1)?))
        .collect::<Result<Vec<_>, _>>()?;
    let omitted_diagnostics = read_u32(message.one(2, WIRE_U32)?)?;
    let backpressure = message
        .optional_one(3, WIRE_MESSAGE)?
        .map(|value| decode_backpressure(codec, nested_message(codec, value, 1)?))
        .transpose()?;
    Ok(NonOkResponse {
        diagnostics,
        omitted_diagnostics,
        backpressure,
    })
}

fn decode_diagnostic(
    codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<Diagnostic, DecodeError> {
    message.schema(&[
        Rule::one(1),
        Rule::one(2),
        Rule::repeated(3),
        Rule::optional(4),
        Rule::optional(5),
        Rule::optional(6),
        Rule::optional(7),
    ])?;
    let code = read_string(codec, message.one(1, WIRE_UTF8)?)?.to_owned();
    if !valid_dotted_code(&code) {
        return Err(DecodeError::InvalidTlv);
    }
    let severity = DiagnosticSeverity::decode(read_u8(message.one(2, WIRE_U8)?)?)?;
    let path = message
        .values(3, WIRE_MESSAGE)?
        .map(|value| decode_path_segment(codec, nested_message(codec, value, 2)?))
        .collect::<Result<Vec<_>, _>>()?;
    let detail = message
        .optional_one(4, WIRE_UTF8)?
        .map(|value| read_string(codec, value).map(str::to_owned))
        .transpose()?;
    let operation_index = message
        .optional_one(5, WIRE_U32)?
        .map(read_u32)
        .transpose()?;
    let sample_time = message
        .optional_one(6, WIRE_U64)?
        .map(read_u64)
        .transpose()?;
    let provider_sequence = message
        .optional_one(7, WIRE_U64)?
        .map(read_u64)
        .transpose()?;
    Ok(Diagnostic {
        code,
        severity,
        path,
        detail,
        operation_index,
        sample_time,
        provider_sequence,
    })
}

fn decode_path_segment(
    codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<PathSegment, DecodeError> {
    message.schema(&[
        Rule::one(1),
        Rule::optional(2),
        Rule::optional(3),
        Rule::optional(4),
    ])?;
    let tag = read_u8(message.one(1, WIRE_U8)?)?;
    let field = message.optional_one(2, WIRE_UTF8)?;
    let index = message.optional_one(3, WIRE_U64)?;
    let stable_id = message.optional_one(4, WIRE_UTF8)?;
    match tag {
        1 if field.is_some() && index.is_none() && stable_id.is_none() => Ok(PathSegment::Field(
            read_string(codec, field.expect("present"))?.to_owned(),
        )),
        2 if field.is_none() && index.is_some() && stable_id.is_none() => {
            Ok(PathSegment::Index(read_u64(index.expect("present"))?))
        }
        3 if field.is_none() && index.is_none() && stable_id.is_some() => {
            let stable_id = read_string(codec, stable_id.expect("present"))?;
            if !valid_stable_id(stable_id) {
                return Err(DecodeError::InvalidTlv);
            }
            Ok(PathSegment::StableId(stable_id.to_owned()))
        }
        _ => Err(DecodeError::InvalidTlv),
    }
}

fn decode_backpressure(
    _codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<Backpressure, DecodeError> {
    message.schema(&[
        Rule::one(1),
        Rule::one(2),
        Rule::one(3),
        Rule::one(4),
        Rule::optional(5),
        Rule::optional(6),
        Rule::optional(7),
        Rule::optional(8),
    ])?;
    let queue_kind = BackpressureQueueKind::decode(read_u8(message.one(1, WIRE_U8)?)?)?;
    let value = Backpressure {
        queue_kind,
        capacity: read_u64(message.one(2, WIRE_U64)?)?,
        occupancy: read_u64(message.one(3, WIRE_U64)?)?,
        requested_items: read_u16(message.one(4, WIRE_U16)?)?,
        generation: message
            .optional_one(5, WIRE_U64)?
            .map(read_u64)
            .transpose()?,
        retry_boundary: message
            .optional_one(6, WIRE_U64)?
            .map(read_u64)
            .transpose()?,
        requested_bytes: message
            .optional_one(7, WIRE_U64)?
            .map(read_u64)
            .transpose()?,
        available_bytes: message
            .optional_one(8, WIRE_U64)?
            .map(read_u64)
            .transpose()?,
    };
    if value.capacity == 0
        || value.occupancy > value.capacity
        || value.requested_items != 1
        || value.queue_kind.has_generation() != value.generation.is_some()
        || value.requested_bytes.is_some() != value.available_bytes.is_some()
    {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(value)
}

fn read_string<'a>(codec: &ProtocolCodec, value: &'a [u8]) -> Result<&'a str, DecodeError> {
    if value.len() > codec.limits().max_string_bytes {
        return Err(DecodeError::LimitExceeded);
    }
    core::str::from_utf8(value).map_err(|_| DecodeError::InvalidUtf8)
}

fn read_u8(value: &[u8]) -> Result<u8, DecodeError> {
    value
        .first()
        .copied()
        .filter(|_| value.len() == 1)
        .ok_or(DecodeError::InvalidValueLength)
}

fn read_bool(value: &[u8]) -> Result<bool, DecodeError> {
    match read_u8(value)? {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(DecodeError::InvalidValueLength),
    }
}

fn read_u16(value: &[u8]) -> Result<u16, DecodeError> {
    let value: [u8; 2] = value
        .try_into()
        .map_err(|_| DecodeError::InvalidValueLength)?;
    Ok(u16::from_le_bytes(value))
}

fn read_u32(value: &[u8]) -> Result<u32, DecodeError> {
    let value: [u8; 4] = value
        .try_into()
        .map_err(|_| DecodeError::InvalidValueLength)?;
    Ok(u32::from_le_bytes(value))
}

fn read_f32(value: &[u8]) -> Result<f32, DecodeError> {
    let value: [u8; 4] = value
        .try_into()
        .map_err(|_| DecodeError::InvalidValueLength)?;
    Ok(f32::from_le_bytes(value))
}

fn read_u64(value: &[u8]) -> Result<u64, DecodeError> {
    let value: [u8; 8] = value
        .try_into()
        .map_err(|_| DecodeError::InvalidValueLength)?;
    Ok(u64::from_le_bytes(value))
}

/// Validate the direct decoder's nested BTLV tree before the typed parsing below allocates its
/// bounded owned result. The frame decoder performs the same work for complete frames, but these
/// public payload helpers must be independently safe when called by a response dispatcher.
fn top_level_message<'a>(
    codec: &ProtocolCodec,
    bytes: &'a [u8],
    count: u32,
) -> Result<Message<'a>, DecodeError> {
    validate_message_tree(codec, bytes, count, 0)?;
    Message::tlvs(bytes, count)
}

fn nested_message<'a>(
    codec: &ProtocolCodec,
    bytes: &'a [u8],
    depth: u8,
) -> Result<Message<'a>, DecodeError> {
    let header = bytes
        .get(..NESTED_HEADER_BYTES)
        .ok_or(DecodeError::Truncated)?;
    let count = u32::from_le_bytes(header[..4].try_into().map_err(|_| DecodeError::Truncated)?);
    if header[4..].iter().any(|byte| *byte != 0) {
        return Err(DecodeError::NonzeroReserved);
    }
    validate_message_tree(codec, &bytes[NESTED_HEADER_BYTES..], count, depth)?;
    Message::nested(bytes)
}

fn validate_message_tree(
    codec: &ProtocolCodec,
    bytes: &[u8],
    count: u32,
    depth: u8,
) -> Result<(), DecodeError> {
    if count > codec.limits().max_tlv_count || bytes.len() > codec.limits().max_frame_bytes {
        return Err(DecodeError::LimitExceeded);
    }
    let mut cursor = 0_usize;
    let mut previous_id = 0_u16;
    for index in 0..count {
        let prefix_end = cursor
            .checked_add(TLV_PREFIX_BYTES)
            .ok_or(DecodeError::LimitExceeded)?;
        let prefix = bytes
            .get(cursor..prefix_end)
            .ok_or(DecodeError::Truncated)?;
        let id = u16::from_le_bytes(prefix[..2].try_into().map_err(|_| DecodeError::Truncated)?);
        let wire = prefix[2];
        if id == 0
            || !(1..=15).contains(&wire)
            || prefix[3] & !1 != 0
            || (index != 0 && id < previous_id)
        {
            return Err(DecodeError::InvalidTlv);
        }
        previous_id = id;
        let value_len = usize::try_from(u32::from_le_bytes(
            prefix[4..8]
                .try_into()
                .map_err(|_| DecodeError::Truncated)?,
        ))
        .map_err(|_| DecodeError::LimitExceeded)?;
        let value_start = prefix_end;
        let value_end = value_start
            .checked_add(value_len)
            .ok_or(DecodeError::LimitExceeded)?;
        let value = bytes
            .get(value_start..value_end)
            .ok_or(DecodeError::Truncated)?;
        validate_wire_value(codec, wire, value, depth)?;
        let padded_end = value_end
            .checked_add(padding(value_len))
            .ok_or(DecodeError::LimitExceeded)?;
        if bytes
            .get(value_end..padded_end)
            .ok_or(DecodeError::Truncated)?
            .iter()
            .any(|byte| *byte != 0)
        {
            return Err(DecodeError::InvalidTlv);
        }
        cursor = padded_end;
    }
    if cursor != bytes.len() {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(())
}

fn validate_wire_value(
    codec: &ProtocolCodec,
    wire: u8,
    value: &[u8],
    depth: u8,
) -> Result<(), DecodeError> {
    let exact_length = match wire {
        WIRE_U8 => Some(1),
        WIRE_U16 => Some(2),
        WIRE_U32 => Some(4),
        WIRE_U64 => Some(8),
        WIRE_UTF8 | WIRE_MESSAGE => None,
        5 | 7 => Some(8),
        6 => Some(4),
        8 => Some(1),
        10 | 12..=15 => None,
        _ => return Err(DecodeError::InvalidTlv),
    };
    if let Some(exact_length) = exact_length {
        if value.len() != exact_length {
            return Err(DecodeError::InvalidValueLength);
        }
        if wire == 8 && !matches!(value[0], 0 | 1) {
            return Err(DecodeError::InvalidValueLength);
        }
    }
    match wire {
        WIRE_UTF8 => {
            if value.len() > codec.limits().max_string_bytes {
                return Err(DecodeError::LimitExceeded);
            }
            if core::str::from_utf8(value).is_err() {
                return Err(DecodeError::InvalidUtf8);
            }
        }
        WIRE_MESSAGE => {
            if depth >= codec.limits().max_nesting {
                return Err(DecodeError::LimitExceeded);
            }
            let header = value
                .get(..NESTED_HEADER_BYTES)
                .ok_or(DecodeError::Truncated)?;
            let count =
                u32::from_le_bytes(header[..4].try_into().map_err(|_| DecodeError::Truncated)?);
            if header[4..].iter().any(|byte| *byte != 0) {
                return Err(DecodeError::NonzeroReserved);
            }
            validate_message_tree(codec, &value[NESTED_HEADER_BYTES..], count, depth + 1)?;
        }
        12 if !value.len().is_multiple_of(2) => return Err(DecodeError::InvalidValueLength),
        13 | 15 if !value.len().is_multiple_of(4) => return Err(DecodeError::InvalidValueLength),
        14 if !value.len().is_multiple_of(8) => return Err(DecodeError::InvalidValueLength),
        _ => {}
    }
    Ok(())
}

fn valid_dotted_code(value: &str) -> bool {
    !value.is_empty()
        && value.split('.').all(|segment| {
            !segment.is_empty()
                && segment
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        })
}

fn valid_stable_id(value: &str) -> bool {
    let bytes = value.as_bytes();
    (1..=127).contains(&bytes.len())
        && bytes[0].is_ascii_lowercase()
        && bytes[1..].iter().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
        })
}

fn checked_add(left: usize, right: usize) -> Result<usize, EncodeError> {
    left.checked_add(right).ok_or(EncodeError::LimitExceeded)
}

fn tlv_len(value_len: usize) -> Result<usize, EncodeError> {
    checked_add(
        TLV_PREFIX_BYTES,
        checked_add(value_len, padding(value_len))?,
    )
}

const fn padding(value_len: usize) -> usize {
    (8 - (value_len & 7)) & 7
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ProtocolLimits;

    fn codec() -> ProtocolCodec {
        ProtocolCodec::new(ProtocolLimits {
            max_frame_bytes: 4096,
            max_tlv_count: 64,
            max_string_bytes: 64,
            max_nesting: 4,
        })
    }

    fn diagnostic() -> Diagnostic {
        Diagnostic {
            code: "schema.unknown_field".to_owned(),
            severity: DiagnosticSeverity::Error,
            path: vec![
                PathSegment::Field("tracks".to_owned()),
                PathSegment::Index(2),
                PathSegment::StableId("vocal".to_owned()),
            ],
            detail: Some("unknown key".to_owned()),
            operation_index: Some(7),
            sample_time: Some(48),
            provider_sequence: None,
        }
    }

    fn response() -> NonOkResponse {
        NonOkResponse {
            diagnostics: vec![diagnostic()],
            omitted_diagnostics: 2,
            backpressure: Some(Backpressure {
                queue_kind: BackpressureQueueKind::ReliableEvent,
                capacity: 8,
                occupancy: 8,
                requested_items: 1,
                generation: Some(4),
                retry_boundary: Some(256),
                requested_bytes: Some(32),
                available_bytes: Some(0),
            }),
        }
    }

    #[test]
    fn common_error_payload_is_canonical_and_caller_owned() {
        let codec = codec();
        let value = response();
        let required = codec.encoded_non_ok_payload_len(&value).expect("size");
        let mut output = vec![0_u8; required];
        assert_eq!(
            codec.encode_non_ok_payload(&value, &mut output),
            Ok(required)
        );
        assert_eq!(
            hex(&output),
            concat!(
                "01000b010001000008000000000000000100090114000000",
                "736368656d612e756e6b6e6f776e5f6669656c6400000000",
                "0200010101000000030000000000000003000b0128000000",
                "020000000000000001000101010000000100000000000000",
                "0200090006000000747261636b73000003000b0128000000",
                "020000000000000001000101010000000200000000000000",
                "0300040008000000020000000000000003000b0128000000",
                "020000000000000001000101010000000300000000000000",
                "0400090005000000766f63616c000000040009000b000000",
                "756e6b6e6f776e206b657900000000000500030004000000",
                "070000000000000006000400080000003000000000000000",
                "0200030104000000020000000000000003000b0088000000",
                "080000000000000001000101010000000400000000000000",
                "020004010800000008000000000000000300040108000000",
                "080000000000000004000201020000000100000000000000",
                "050004000800000004000000000000000600040008000000",
                "000100000000000007000400080000002000000000000000",
                "08000400080000000000000000000000"
            )
        );
        assert_eq!(
            codec.decode_non_ok_payload(&output, 3).expect("decode"),
            value
        );
        let mut short = vec![0xaa; required - 1];
        assert_eq!(
            codec.encode_non_ok_payload(&value, &mut short),
            Err(EncodeError::OutputTooSmall { required })
        );
        assert!(short.iter().all(|byte| *byte == 0xaa));
    }

    #[test]
    fn unknown_optional_skips_but_required_and_wrong_variant_reject() {
        let codec = codec();
        let value = response();
        let required = codec.encoded_non_ok_payload_len(&value).expect("size");
        let mut output = vec![0_u8; required];
        codec
            .encode_non_ok_payload(&value, &mut output)
            .expect("encode");
        let mut optional = output.clone();
        optional.extend_from_slice(&[99, 0, WIRE_U8, 0, 1, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(
            codec
                .decode_non_ok_payload(&optional, 4)
                .expect("optional skips"),
            value
        );
        optional[required + 3] = 1;
        assert_eq!(
            codec.decode_non_ok_payload(&optional, 4),
            Err(DecodeError::UnknownRequiredField)
        );
        let mut bad_path = output;
        // The first path begins after the diagnostic's code and severity TLVs. Change the
        // `FIELD` variant's inner value field from id 2 to id 3 while its tag remains FIELD.
        bad_path[96..98].copy_from_slice(&3_u16.to_le_bytes());
        assert!(matches!(
            codec.decode_non_ok_payload(&bad_path, 3),
            Err(DecodeError::InvalidTlv | DecodeError::Truncated)
        ));
    }

    #[test]
    fn every_common_golden_truncation_rejects_and_bounded_count_is_exact() {
        let codec = codec();
        let value = response();
        let required = codec.encoded_non_ok_payload_len(&value).expect("size");
        let mut output = vec![0_u8; required];
        codec
            .encode_non_ok_payload(&value, &mut output)
            .expect("encode");
        for end in 0..output.len() {
            assert!(codec.decode_non_ok_payload(&output[..end], 3).is_err());
        }
        let bounded = NonOkResponse::bounded(&[diagnostic(), diagnostic(), diagnostic()], 2);
        assert_eq!(bounded.diagnostics.len(), 2);
        assert_eq!(bounded.omitted_diagnostics, 1);
    }

    #[test]
    fn path_tags_and_registered_mandatory_flags_are_exact() {
        let codec = codec();
        for (segment, expected_tag) in [
            (PathSegment::Field("field".to_owned()), 1),
            (PathSegment::Index(9), 2),
            (PathSegment::StableId("track_1".to_owned()), 3),
        ] {
            let value = NonOkResponse {
                diagnostics: vec![Diagnostic {
                    code: "a.b".to_owned(),
                    severity: DiagnosticSeverity::Error,
                    path: vec![segment],
                    detail: None,
                    operation_index: None,
                    sample_time: None,
                    provider_sequence: None,
                }],
                omitted_diagnostics: 0,
                backpressure: None,
            };
            let required = codec.encoded_non_ok_payload_len(&value).expect("length");
            let mut output = vec![0_u8; required];
            codec
                .encode_non_ok_payload(&value, &mut output)
                .expect("encode");
            // Top-level diagnostic value begins at byte 8. Its nested header, code TLV, and
            // severity TLV are fixed at 8, 16, and 16 bytes for this three-byte code.
            assert_eq!(output[72], expected_tag);
            assert_eq!(codec.decode_non_ok_payload(&output, 2), Ok(value));

            let mut missing_repeated_mandatory = output.clone();
            missing_repeated_mandatory[3] = 0;
            assert_eq!(
                codec.decode_non_ok_payload(&missing_repeated_mandatory, 2),
                Err(DecodeError::InvalidTlv)
            );
            let mut missing_omitted_count_mandatory = output.clone();
            let count_offset = output.len() - 16;
            missing_omitted_count_mandatory[count_offset + 3] = 0;
            assert_eq!(
                codec.decode_non_ok_payload(&missing_omitted_count_mandatory, 2),
                Err(DecodeError::InvalidTlv)
            );
            let mut wrong_tagged_variant = output;
            let incompatible_variant_field: u16 = match expected_tag {
                1 => 3,
                2 => 4,
                3 => 2,
                _ => unreachable!("frozen path tag"),
            };
            wrong_tagged_variant[80..82].copy_from_slice(&incompatible_variant_field.to_le_bytes());
            assert_eq!(
                codec.decode_non_ok_payload(&wrong_tagged_variant, 2),
                Err(DecodeError::InvalidTlv)
            );
        }
    }

    #[test]
    fn direct_common_decoder_enforces_counts_strings_and_nesting() {
        let value = response();
        let normal = codec();
        let required = normal.encoded_non_ok_payload_len(&value).expect("length");
        let mut output = vec![0_u8; required];
        normal
            .encode_non_ok_payload(&value, &mut output)
            .expect("encode");

        let field_limited = ProtocolCodec::new(ProtocolLimits {
            max_tlv_count: 2,
            ..codec().limits()
        });
        assert_eq!(
            field_limited.encoded_non_ok_payload_len(&value),
            Err(EncodeError::LimitExceeded)
        );
        assert_eq!(
            field_limited.decode_non_ok_payload(&output, 3),
            Err(DecodeError::LimitExceeded)
        );

        let string_limited = ProtocolCodec::new(ProtocolLimits {
            max_string_bytes: 3,
            ..codec().limits()
        });
        assert_eq!(
            string_limited.encoded_non_ok_payload_len(&value),
            Err(EncodeError::LimitExceeded)
        );
        assert_eq!(
            string_limited.decode_non_ok_payload(&output, 3),
            Err(DecodeError::LimitExceeded)
        );

        let nesting_limited = ProtocolCodec::new(ProtocolLimits {
            max_nesting: 1,
            ..codec().limits()
        });
        assert_eq!(
            nesting_limited.encoded_non_ok_payload_len(&value),
            Err(EncodeError::LimitExceeded)
        );
        assert_eq!(
            nesting_limited.decode_non_ok_payload(&output, 3),
            Err(DecodeError::LimitExceeded)
        );
    }

    #[test]
    fn direct_common_encoder_is_byte_stable_in_caller_storage() {
        let codec = codec();
        let value = response();
        let required = codec.encoded_non_ok_payload_len(&value).expect("length");
        let mut output = vec![0_u8; required];
        for _ in 0..16 {
            output.fill(0);
            assert_eq!(
                codec.encode_non_ok_payload(&value, &mut output),
                Ok(required)
            );
            assert_eq!(codec.decode_non_ok_payload(&output, 3), Ok(value.clone()));
        }
    }

    #[test]
    fn b1b_success_schemas_round_trip_and_truncate() {
        let codec = codec();
        let commands = [1_u16, 2, 3];
        let events = [0x8001_u16, 0x8002];
        let capabilities = Capabilities {
            minimum_version: crate::ProtocolVersion::V1,
            maximum_version: crate::ProtocolVersion::V1,
            maximum_frame_bytes: 4096,
            maximum_tlvs: 64,
            maximum_string_bytes: 64,
            maximum_nesting: 4,
            maximum_automation_records: 256,
            control_command_slots: 1,
            control_command_bytes: 64,
            automation_batch_slots: 1,
            reliable_response_slots: 1,
            reliable_event_slots: 1,
            telemetry_slots: 1,
            replay_entries: 1,
            replay_bytes: 1024,
            maximum_cached_response_bytes: 512,
            per_block_automation_density: 256,
            admission_quantum_frames: 64,
            maximum_parameter_page_items: 256,
            maximum_diagnostic_page_items: 256,
            maximum_telemetry_handles: 256,
            maximum_transaction_edits: 64,
            supported_commands: &commands,
            supported_events: &events,
            flags: CapabilityFlags(
                CapabilityFlags::B1B_BASE.0 | CapabilityFlags::SESSION_EVENT_STREAM.0,
            ),
        };
        let mut capability_bytes = vec![
            0;
            codec
                .encoded_capabilities_len(&capabilities)
                .expect("length")
        ];
        codec
            .encode_capabilities(&capabilities, &mut capability_bytes)
            .expect("encode");
        let decoded = codec
            .decode_capabilities(&capability_bytes, 27)
            .expect("decode");
        assert_eq!(decoded.maximum_frame_bytes, 4096);
        assert_eq!(decoded.supported_commands, &[1, 0, 2, 0, 3, 0]);
        for end in 0..capability_bytes.len() {
            assert!(
                codec
                    .decode_capabilities(&capability_bytes[..end], 27)
                    .is_err()
            );
        }
        let request = SessionSnapshotRequest {
            offset: 2,
            maximum_bytes: 3,
        };
        let mut request_bytes = [0_u8; 32];
        codec
            .encode_snapshot_request(request, &mut request_bytes)
            .expect("request");
        assert_eq!(
            codec.decode_snapshot_request(&request_bytes, 2),
            Ok(request)
        );
        let snapshot = SessionSnapshot {
            total_bytes: 5,
            offset: 2,
            canonical_toml_chunk: b"cde",
            eof: true,
        };
        let mut snapshot_bytes = vec![0; codec.encoded_snapshot_len(snapshot).expect("length")];
        codec
            .encode_snapshot(snapshot, &mut snapshot_bytes)
            .expect("snapshot");
        assert_eq!(codec.decode_snapshot(&snapshot_bytes, 4), Ok(snapshot));
        let applied = TransactionApplied {
            applied_operations: 3,
        };
        let mut applied_bytes = [0_u8; 16];
        codec
            .encode_transaction_applied(applied, &mut applied_bytes)
            .expect("applied");
        assert_eq!(
            codec.decode_transaction_applied(&applied_bytes, 1),
            Ok(applied)
        );
        let event = SessionCommitted {
            event_sequence: 1,
            origin_request_id: crate::RequestId::new(2).expect("id"),
            previous_revision: crate::SessionRevision(7),
            applied_operations: 3,
        };
        let mut event_bytes = [0_u8; 64];
        codec
            .encode_session_committed(event, &mut event_bytes)
            .expect("event");
        assert_eq!(codec.decode_session_committed(&event_bytes, 4), Ok(event));
    }

    #[test]
    fn b1b_fixed_goldens_and_every_byte_truncation() {
        let codec = codec();
        let request = SessionSnapshotRequest {
            offset: 2,
            maximum_bytes: 3,
        };
        let mut request_bytes = [0_u8; 32];
        codec
            .encode_snapshot_request(request, &mut request_bytes)
            .expect("request");
        assert_eq!(
            hex(&request_bytes),
            "0100040108000000020000000000000002000301040000000300000000000000"
        );
        for end in 0..request_bytes.len() {
            assert!(
                codec
                    .decode_snapshot_request(&request_bytes[..end], 2)
                    .is_err()
            );
        }

        // The two-byte chunk is intentionally an incomplete UTF-8 sequence; snapshot pages are
        // canonical byte ranges and may split a code point.
        let snapshot = SessionSnapshot {
            total_bytes: 5,
            offset: 2,
            canonical_toml_chunk: &[b'a', 0xc3],
            eof: false,
        };
        let mut snapshot_bytes = [0_u8; 64];
        codec
            .encode_snapshot(snapshot, &mut snapshot_bytes)
            .expect("snapshot");
        assert_eq!(
            hex(&snapshot_bytes),
            "010004010800000005000000000000000200040108000000020000000000000003000a010200000061c300000000000004000801010000000000000000000000"
        );
        for end in 0..snapshot_bytes.len() {
            assert!(codec.decode_snapshot(&snapshot_bytes[..end], 4).is_err());
        }

        let applied = TransactionApplied {
            applied_operations: 3,
        };
        let mut applied_bytes = [0_u8; 16];
        codec
            .encode_transaction_applied(applied, &mut applied_bytes)
            .expect("applied");
        assert_eq!(hex(&applied_bytes), "01000301040000000300000000000000");
        for end in 0..applied_bytes.len() {
            assert!(
                codec
                    .decode_transaction_applied(&applied_bytes[..end], 1)
                    .is_err()
            );
        }

        let event = SessionCommitted {
            event_sequence: 1,
            origin_request_id: crate::RequestId::new(2).expect("id"),
            previous_revision: crate::SessionRevision(7),
            applied_operations: 3,
        };
        let mut event_payload = [0_u8; 64];
        codec
            .encode_session_committed(event, &mut event_payload)
            .expect("event");
        assert_eq!(
            hex(&event_payload),
            "01000401080000000100000000000000020004010800000002000000000000000300040108000000070000000000000004000301040000000300000000000000"
        );
        for end in 0..event_payload.len() {
            assert!(
                codec
                    .decode_session_committed(&event_payload[..end], 4)
                    .is_err()
            );
        }
        let mut frame = [0_u8; crate::OUTER_HEADER_BYTES + 64];
        codec
            .write_outer_header(
                &mut frame,
                crate::FrameKind::Event,
                crate::MessageId::SessionCommitted,
                crate::StatusCode::Ok,
                0,
                8,
                0,
                64,
                4,
            )
            .expect("header");
        frame[crate::OUTER_HEADER_BYTES..].copy_from_slice(&event_payload);
        assert_eq!(
            hex(&frame),
            concat!(
                "4d49534f43544c0001000000300003000180000040000000000000000000000008000000000000000400000000000000",
                "01000401080000000100000000000000020004010800000002000000000000000300040108000000070000000000000004000301040000000300000000000000"
            )
        );
        for end in 0..frame.len() {
            assert!(
                codec
                    .decode(
                        &frame[..end],
                        &mut crate::DecodeScratch::new(&mut [0_u16; 4])
                    )
                    .is_err()
            );
        }
    }

    #[test]
    fn b2a_metadata_and_state_are_typed_bounded_and_strict() {
        let codec = codec();
        let descriptor = ParameterDescriptor {
            handle: 1,
            track_id: "vocal".to_owned(),
            rack: ParameterRack::Dynamic,
            effect_id: "comp".to_owned(),
            parameter_id: 7,
            channel: ParameterChannel::Left,
            value_kind: ParameterValueKind::F32,
            unit: ParameterUnit::Db,
            domain: ParameterDomain::Continuous,
            minimum: Some(-24.0),
            maximum: Some(24.0),
            default: 0.0,
            mapping: ParameterMapping::Linear,
            automation_rate: ParameterAutomationRate::Sample,
            smoothing_samples: 12,
            flags: 3,
            display_name: Some("Threshold".to_owned()),
            display_unit: Some("dB".to_owned()),
            enum_choices: Vec::new(),
        };
        let page = ParameterMetadataPage {
            last_handle: 1,
            eof: true,
            descriptors: vec![descriptor],
        };
        let mut metadata = vec![
            0;
            codec
                .encoded_parameter_metadata_page_len(&page)
                .expect("len")
        ];
        codec
            .encode_parameter_metadata_page(&page, &mut metadata)
            .expect("encode");
        assert_eq!(metadata[51], 1, "descriptor handle stays mandatory");
        assert_eq!(codec.decode_parameter_metadata_page(&metadata, 3), Ok(page));
        for end in 0..metadata.len() {
            assert!(
                codec
                    .decode_parameter_metadata_page(&metadata[..end], 3)
                    .is_err()
            );
        }
        let request = ParameterStateRequest {
            handles: vec![1, 2],
        };
        let mut request_bytes = [0_u8; 16];
        codec
            .encode_parameter_state_request(&request, &mut request_bytes)
            .expect("request");
        assert_eq!(
            codec.decode_parameter_state_request(&request_bytes, 1),
            Ok(request)
        );
        let state = ParameterStatePage {
            observed_sample: 44,
            records: vec![
                ParameterStateRecord {
                    handle: 1,
                    flags: 3,
                    value: -2.0,
                },
                ParameterStateRecord {
                    handle: 2,
                    flags: 0,
                    value: 0.0,
                },
            ],
        };
        let mut state_bytes = vec![0; codec.encoded_parameter_state_page_len(&state).expect("len")];
        codec
            .encode_parameter_state_page(&state, &mut state_bytes)
            .expect("encode");
        assert_eq!(
            codec.decode_parameter_state_page(&state_bytes, 4),
            Ok(state)
        );
        let invalid = ParameterStatePage {
            observed_sample: 0,
            records: vec![ParameterStateRecord {
                handle: 1,
                flags: 2,
                value: 0.0,
            }],
        };
        assert_eq!(
            codec.encoded_parameter_state_page_len(&invalid),
            Err(EncodeError::LimitExceeded)
        );
    }

    fn b2_descriptor(handle: u32, domain: ParameterDomain) -> ParameterDescriptor {
        let (minimum, maximum, default, choices) = match domain {
            ParameterDomain::Continuous => (Some(-1.0), Some(1.0), 0.0, Vec::new()),
            ParameterDomain::Boolean => (None, None, 1.0, Vec::new()),
            ParameterDomain::Enumeration => (
                None,
                None,
                2.0,
                vec![
                    EnumChoice {
                        value: 1.0,
                        label: "one".to_owned(),
                    },
                    EnumChoice {
                        value: 2.0,
                        label: "two".to_owned(),
                    },
                ],
            ),
        };
        ParameterDescriptor {
            handle,
            track_id: "vocal".to_owned(),
            rack: ParameterRack::Dynamic,
            effect_id: "comp".to_owned(),
            parameter_id: handle,
            channel: ParameterChannel::Left,
            value_kind: ParameterValueKind::F32,
            unit: ParameterUnit::Db,
            domain,
            minimum,
            maximum,
            default,
            mapping: ParameterMapping::Linear,
            automation_rate: ParameterAutomationRate::Sample,
            smoothing_samples: 12,
            flags: 3,
            display_name: None,
            display_unit: None,
            enum_choices: choices,
        }
    }

    #[test]
    fn b2a_goldens_truncations_malformed_matrix_and_encoder_audit() {
        let codec = ProtocolCodec::default();
        let metadata_request = ParameterMetadataRequest {
            after_handle: 4,
            limit: 3,
        };
        let mut metadata_request_bytes = [0_u8; 32];
        codec
            .encode_parameter_metadata_request(metadata_request, &mut metadata_request_bytes)
            .expect("metadata request");
        assert_eq!(
            hex(&metadata_request_bytes),
            "0100030104000000040000000000000002000201020000000300000000000000"
        );
        for end in 0..metadata_request_bytes.len() {
            assert!(
                codec
                    .decode_parameter_metadata_request(&metadata_request_bytes[..end], 2)
                    .is_err()
            );
        }
        let page = ParameterMetadataPage {
            last_handle: 3,
            eof: true,
            descriptors: vec![
                b2_descriptor(1, ParameterDomain::Continuous),
                b2_descriptor(2, ParameterDomain::Boolean),
                b2_descriptor(3, ParameterDomain::Enumeration),
            ],
        };
        let mut metadata = vec![
            0;
            codec
                .encoded_parameter_metadata_page_len(&page)
                .expect("metadata len")
        ];
        codec
            .encode_parameter_metadata_page(&page, &mut metadata)
            .expect("metadata encode");
        assert_eq!(
            hex(&metadata),
            concat!(
                "010003010400000003000000000000000200080101000000010000000000000003000b01080100001000000000000000",
                "010003010400000001000000000000000200090105000000766f63616c00000003000101010000000200000000000000",
                "0400090104000000636f6d70000000000500030104000000010000000000000006000101010000000100000000000000",
                "070001010100000001000000000000000800010101000000010000000000000009000101010000000100000000000000",
                "0a00060004000000000080bf000000000b000600040000000000803f000000000c000601040000000000000000000000",
                "0d0001010100000001000000000000000e0001010100000001000000000000000f000301040000000c00000000000000",
                "1000030104000000030000000000000003000b01e80000000e0000000000000001000301040000000200000000000000",
                "0200090105000000766f63616c000000030001010100000002000000000000000400090104000000636f6d7000000000",
                "050003010400000002000000000000000600010101000000010000000000000007000101010000000100000000000000",
                "08000101010000000100000000000000090001010100000002000000000000000c000601040000000000803f00000000",
                "0d0001010100000001000000000000000e0001010100000001000000000000000f000301040000000c00000000000000",
                "1000030104000000030000000000000003000b0148010000100000000000000001000301040000000300000000000000",
                "0200090105000000766f63616c000000030001010100000002000000000000000400090104000000636f6d7000000000",
                "050003010400000003000000000000000600010101000000010000000000000007000101010000000100000000000000",
                "08000101010000000100000000000000090001010100000003000000000000000c000601040000000000004000000000",
                "0d0001010100000001000000000000000e0001010100000001000000000000000f000301040000000c00000000000000",
                "1000030104000000030000000000000013000b0028000000020000000000000001000601040000000000803f00000000",
                "02000901030000006f6e65000000000013000b0028000000020000000000000001000601040000000000004000000000",
                "020009010300000074776f0000000000"
            )
        );
        for end in 0..metadata.len() {
            assert!(
                codec
                    .decode_parameter_metadata_page(&metadata[..end], 5)
                    .is_err()
            );
        }
        let state_request = ParameterStateRequest {
            handles: vec![1, 2],
        };
        let mut state_request_bytes = [0_u8; 16];
        codec
            .encode_parameter_state_request(&state_request, &mut state_request_bytes)
            .expect("state request");
        assert_eq!(
            hex(&state_request_bytes),
            "01000d01080000000100000002000000"
        );
        for end in 0..state_request_bytes.len() {
            assert!(
                codec
                    .decode_parameter_state_request(&state_request_bytes[..end], 1)
                    .is_err()
            );
        }
        let state = ParameterStatePage {
            observed_sample: 8,
            records: vec![
                ParameterStateRecord {
                    handle: 1,
                    flags: 1,
                    value: 0.5,
                },
                ParameterStateRecord {
                    handle: 2,
                    flags: 0,
                    value: 0.0,
                },
            ],
        };
        let mut state_bytes = vec![
            0;
            codec
                .encoded_parameter_state_page_len(&state)
                .expect("state len")
        ];
        codec
            .encode_parameter_state_page(&state, &mut state_bytes)
            .expect("state encode");
        assert_eq!(
            hex(&state_bytes),
            "01000401080000000800000000000000020002010200000002000000000000000300020102000000100000000000000004000a012000000001000000010000000000003f0000000002000000000000000000000000000000"
        );
        for end in 0..state_bytes.len() {
            assert!(
                codec
                    .decode_parameter_state_page(&state_bytes[..end], 4)
                    .is_err()
            );
        }
        assert!(
            codec
                .decode_parameter_metadata_request(
                    &[
                        1, 0, 3, 1, 4, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 1, 2, 0, 0, 0, 0, 0, 0, 0
                    ],
                    2
                )
                .is_err()
        );
        for request in [
            ParameterStateRequest { handles: vec![] },
            ParameterStateRequest { handles: vec![0] },
            ParameterStateRequest {
                handles: vec![2, 1],
            },
            ParameterStateRequest {
                handles: vec![1, 1],
            },
        ] {
            assert!(
                codec
                    .encode_parameter_state_request(&request, &mut [0_u8; 16])
                    .is_err()
            );
        }
        let mut bad = b2_descriptor(1, ParameterDomain::Continuous);
        bad.default = 2.0;
        assert!(
            codec
                .encoded_parameter_metadata_page_len(&ParameterMetadataPage {
                    last_handle: 1,
                    eof: true,
                    descriptors: vec![bad]
                })
                .is_err()
        );
        let mut bad = b2_descriptor(1, ParameterDomain::Boolean);
        bad.default = 0.5;
        assert!(
            codec
                .encoded_parameter_metadata_page_len(&ParameterMetadataPage {
                    last_handle: 1,
                    eof: true,
                    descriptors: vec![bad]
                })
                .is_err()
        );
        let mut bad = b2_descriptor(1, ParameterDomain::Enumeration);
        bad.enum_choices.push(EnumChoice {
            value: 2.0,
            label: "duplicate".to_owned(),
        });
        assert!(
            codec
                .encoded_parameter_metadata_page_len(&ParameterMetadataPage {
                    last_handle: 1,
                    eof: true,
                    descriptors: vec![bad]
                })
                .is_err()
        );
        let mut bad_state = state_bytes.clone();
        bad_state[60] = 4;
        assert!(codec.decode_parameter_state_page(&bad_state, 4).is_err());
        bad_state = state_bytes.clone();
        bad_state[68] = 1;
        assert!(codec.decode_parameter_state_page(&bad_state, 4).is_err());
        let mut many = (1..=256)
            .map(|handle| b2_descriptor(handle, ParameterDomain::Continuous))
            .collect::<Vec<_>>();
        let full = ParameterMetadataPage {
            last_handle: 256,
            eof: true,
            descriptors: many.clone(),
        };
        let required = codec
            .encoded_parameter_metadata_page_len(&full)
            .expect("full len");
        let mut output = vec![0; required];
        for _ in 0..4 {
            output.fill(0);
            assert_eq!(
                codec.encode_parameter_metadata_page(&full, &mut output),
                Ok(required)
            );
            assert_eq!(
                codec.decode_parameter_metadata_page(&output, 258),
                Ok(full.clone())
            );
        }
        many.clear();
    }

    fn automation_record(sample: u64, handle: u32, value: f32) -> crate::AutomationRecord {
        crate::AutomationRecord {
            kind: crate::AutomationKind::Point,
            handle: crate::ParameterHandle(handle),
            start: crate::SampleTime(sample),
            end: crate::SampleTime(sample),
            start_value: value,
            end_value: value,
        }
    }

    #[test]
    fn b2b_automation_goldens_truncation_and_malformed_cases_are_functional() {
        let codec = codec();
        let records = [automation_record(2, 1, 1.0)];
        let request = AutomationEnqueue { records: &records };
        let required = codec
            .encoded_automation_enqueue_len(request)
            .expect("request length");
        assert_eq!(required, 72);
        let mut bytes = [0_u8; 72];
        assert_eq!(
            codec.encode_automation_enqueue(request, &mut bytes),
            Ok(required)
        );
        assert_eq!(
            hex(&bytes),
            concat!(
                "01000201020000000100000000000000",
                "02000201020000002000000000000000",
                "03000a01200000000100000001000000",
                "02000000000000000200000000000000",
                "0000803f0000803f"
            )
        );
        let decoded = codec
            .decode_automation_enqueue(&bytes, 3)
            .expect("strict decode");
        assert_eq!(decoded.count, 1);
        assert_eq!(decoded.record(0), Ok(records[0]));
        assert_eq!(
            decoded
                .into_batch(
                    crate::SessionRevision(7),
                    crate::RequestId::new(9).expect("id")
                )
                .expect("slot")
                .as_slice(),
            &records
        );
        for end in 0..bytes.len() {
            assert!(codec.decode_automation_enqueue(&bytes[..end], 3).is_err());
        }
        let success = AutomationEnqueued {
            accepted_records: 1,
            occupancy: 1,
            capacity: 2,
            generation: 2,
        };
        let mut success_bytes = [0_u8; 64];
        codec
            .encode_automation_enqueued(success, &mut success_bytes)
            .expect("success encode");
        assert_eq!(
            hex(&success_bytes),
            concat!(
                "01000201020000000100000000000000",
                "02000401080000000100000000000000",
                "03000401080000000200000000000000",
                "04000401080000000200000000000000"
            )
        );
        assert_eq!(
            codec.decode_automation_enqueued(&success_bytes, 4),
            Ok(success)
        );
        for end in 0..success_bytes.len() {
            assert!(
                codec
                    .decode_automation_enqueued(&success_bytes[..end], 4)
                    .is_err()
            );
        }

        let mut invalid = bytes;
        invalid[8] = 0; // count zero
        assert!(codec.decode_automation_enqueue(&invalid, 3).is_err());
        invalid = bytes;
        invalid[24] = 31; // stride must be 32
        assert!(codec.decode_automation_enqueue(&invalid, 3).is_err());
        invalid = bytes;
        invalid[36] = 31; // exact count * stride byte length
        assert!(codec.decode_automation_enqueue(&invalid, 3).is_err());
        invalid = bytes;
        invalid[44..48].fill(0); // zero public handle
        assert!(codec.decode_automation_enqueue(&invalid, 3).is_err());
        invalid = bytes;
        invalid[35] = 0; // known field with optional flag
        assert!(codec.decode_automation_enqueue(&invalid, 3).is_err());
        invalid = bytes;
        invalid[34] = WIRE_U32; // known field wire type
        assert!(codec.decode_automation_enqueue(&invalid, 3).is_err());
        let mut extension = bytes.to_vec();
        extension.extend_from_slice(&[4, 0, WIRE_U8, 0, 1, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0]);
        assert!(codec.decode_automation_enqueue(&extension, 4).is_ok());
        extension[72 + 3] = 1;
        assert_eq!(
            codec.decode_automation_enqueue(&extension, 4),
            Err(DecodeError::UnknownRequiredField)
        );
        let ordered = [automation_record(3, 1, 1.0), automation_record(4, 2, 1.0)];
        let mut ordered_bytes = [0_u8; 104];
        codec
            .encode_automation_enqueue(AutomationEnqueue { records: &ordered }, &mut ordered_bytes)
            .expect("ordered encode");
        ordered_bytes[80..88].copy_from_slice(&2_u64.to_le_bytes());
        ordered_bytes[88..96].copy_from_slice(&2_u64.to_le_bytes());
        assert!(codec.decode_automation_enqueue(&ordered_bytes, 3).is_err());
        let adjacent = [
            crate::AutomationRecord {
                kind: crate::AutomationKind::Linear,
                handle: crate::ParameterHandle(1),
                start: crate::SampleTime(3),
                end: crate::SampleTime(5),
                start_value: 0.0,
                end_value: 1.0,
            },
            crate::AutomationRecord {
                kind: crate::AutomationKind::Linear,
                handle: crate::ParameterHandle(1),
                start: crate::SampleTime(5),
                end: crate::SampleTime(7),
                start_value: 1.0,
                end_value: 0.0,
            },
        ];
        let mut adjacent_bytes = [0_u8; 104];
        codec
            .encode_automation_enqueue(
                AutomationEnqueue { records: &adjacent },
                &mut adjacent_bytes,
            )
            .expect("adjacent encode");
        adjacent_bytes[80..88].copy_from_slice(&4_u64.to_le_bytes());
        assert!(codec.decode_automation_enqueue(&adjacent_bytes, 3).is_err());

        let queue_config = crate::ProtocolQueueConfig {
            control_command_slots: core::num::NonZeroUsize::new(1).expect("one"),
            control_command_bytes: core::num::NonZeroUsize::new(1).expect("one"),
            automation_batch_slots: core::num::NonZeroUsize::new(1).expect("one"),
            reliable_response_slots: core::num::NonZeroUsize::new(1).expect("one"),
            reliable_event_slots: core::num::NonZeroUsize::new(1).expect("one"),
            telemetry_slots: core::num::NonZeroUsize::new(1).expect("one"),
            per_block_automation_density: core::num::NonZeroUsize::new(256).expect("density"),
            quantum_frames: core::num::NonZeroUsize::new(1).expect("quantum"),
        };
        let mut queues = crate::ProtocolQueues::prepare(queue_config).expect("prepared");
        let mut output = vec![0_u8; 16 + 16 + 8 + crate::AUTOMATION_BATCH_RECORDS * 32];
        let audit_codec = ProtocolCodec::new(ProtocolLimits {
            max_frame_bytes: 16 * 1024,
            ..codec.limits()
        });
        let mut next = 0_u64;
        for batch_index in 0..40_u64 {
            let count = if batch_index == 39 { 16 } else { 256 };
            let mut batch = [crate::AutomationRecord::EMPTY; crate::AUTOMATION_BATCH_RECORDS];
            for record in &mut batch[..count] {
                *record = automation_record(next, 1, 1.0);
                next += 1;
            }
            let value = AutomationEnqueue {
                records: &batch[..count],
            };
            output.fill(0xa5);
            let len = audit_codec
                .encode_automation_enqueue(value, &mut output)
                .expect("encode");
            assert!(output[len..].iter().all(|byte| *byte == 0xa5));
            let decoded = audit_codec
                .decode_automation_enqueue(&output[..len], 3)
                .expect("decode");
            let slot = decoded
                .into_batch(
                    crate::SessionRevision(7),
                    crate::RequestId::new(batch_index + 1).expect("id"),
                )
                .expect("slot");
            queues
                .try_enqueue_automation(crate::SampleTime(0), slot)
                .expect("enqueue");
            assert_eq!(
                queues.try_dequeue_automation().expect("consume").len as usize,
                count
            );
        }
        assert_eq!(next, 10_000);
    }

    #[test]
    fn b3a_transport_goldens_truncation_and_direct_codec_are_strict() {
        let codec = codec();
        let mut empty = [0xaa; 1];
        assert_eq!(codec.encode_transport_get_request(&mut empty), Ok(0));
        assert_eq!(empty, [0xaa]);
        assert_eq!(codec.decode_transport_get_request(&[], 0), Ok(()));

        let set = TransportSetRequest {
            state: TransportState::Playing,
            position: Some(crate::SampleTime(2)),
        };
        let mut set_bytes = [0_u8; 32];
        codec
            .encode_transport_set_request(set, &mut set_bytes)
            .expect("set encode");
        assert_eq!(
            hex(&set_bytes),
            concat!(
                "01000101010000000200000000000000",
                "02000400080000000200000000000000"
            )
        );
        assert_eq!(codec.decode_transport_set_request(&set_bytes, 2), Ok(set));
        let mut short = [0xaa; 31];
        assert_eq!(
            codec.encode_transport_set_request(set, &mut short),
            Err(EncodeError::OutputTooSmall { required: 32 })
        );
        assert!(short.iter().all(|byte| *byte == 0xaa));
        for end in 0..set_bytes.len() {
            assert!(
                codec
                    .decode_transport_set_request(&set_bytes[..end], 2)
                    .is_err()
            );
        }
        let mut bad = set_bytes;
        bad[8] = 3;
        assert!(codec.decode_transport_set_request(&bad, 2).is_err());
        bad = set_bytes;
        bad[3] = 0;
        assert!(codec.decode_transport_set_request(&bad, 2).is_err());
        let mut extension = set_bytes.to_vec();
        extension.extend_from_slice(&[3, 0, WIRE_U8, 0, 1, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(
            codec.decode_transport_set_request(&extension, 3),
            Ok(set),
            "unknown optional fields are skippable"
        );
        extension[32 + 3] = 1;
        assert_eq!(
            codec.decode_transport_set_request(&extension, 3),
            Err(DecodeError::UnknownRequiredField)
        );

        let snapshot = TransportSnapshot {
            state: TransportState::Stopped,
            position: crate::SampleTime(2),
            effective_sample: crate::SampleTime(3),
        };
        let mut snapshot_bytes = [0_u8; 48];
        codec
            .encode_transport_snapshot(snapshot, &mut snapshot_bytes)
            .expect("snapshot encode");
        assert_eq!(
            hex(&snapshot_bytes),
            concat!(
                "01000101010000000100000000000000",
                "02000401080000000200000000000000",
                "03000401080000000300000000000000"
            )
        );
        assert_eq!(
            codec.decode_transport_snapshot(&snapshot_bytes, 3),
            Ok(snapshot)
        );
        for end in 0..snapshot_bytes.len() {
            assert!(
                codec
                    .decode_transport_snapshot(&snapshot_bytes[..end], 3)
                    .is_err()
            );
        }

        let event = TransportStateEvent {
            event_sequence: 1,
            state: TransportState::Playing,
            position: crate::SampleTime(2),
            effective_sample: crate::SampleTime(3),
            origin_request_id: crate::RequestId::new(9),
        };
        let mut event_bytes = [0_u8; 80];
        codec
            .encode_transport_state_event(event, &mut event_bytes)
            .expect("event encode");
        assert_eq!(
            hex(&event_bytes),
            concat!(
                "01000401080000000100000000000000",
                "02000101010000000200000000000000",
                "03000401080000000200000000000000",
                "04000401080000000300000000000000",
                "05000400080000000900000000000000"
            )
        );
        assert_eq!(
            codec.decode_transport_state_event(&event_bytes, 5),
            Ok(event)
        );
        for end in 0..event_bytes.len() {
            assert!(
                codec
                    .decode_transport_state_event(&event_bytes[..end], 5)
                    .is_err()
            );
        }
        let no_origin = TransportStateEvent {
            origin_request_id: None,
            ..event
        };
        let mut no_origin_bytes = [0_u8; 64];
        codec
            .encode_transport_state_event(no_origin, &mut no_origin_bytes)
            .expect("no origin encode");
        assert_eq!(
            codec.decode_transport_state_event(&no_origin_bytes, 4),
            Ok(no_origin)
        );
        for _ in 0..64 {
            set_bytes.fill(0);
            snapshot_bytes.fill(0);
            event_bytes.fill(0);
            assert_eq!(
                codec.encode_transport_set_request(set, &mut set_bytes),
                Ok(32)
            );
            assert_eq!(codec.decode_transport_set_request(&set_bytes, 2), Ok(set));
            assert_eq!(
                codec.encode_transport_snapshot(snapshot, &mut snapshot_bytes),
                Ok(48)
            );
            assert_eq!(
                codec.decode_transport_snapshot(&snapshot_bytes, 3),
                Ok(snapshot)
            );
            assert_eq!(
                codec.encode_transport_state_event(event, &mut event_bytes),
                Ok(80)
            );
            assert_eq!(
                codec.decode_transport_state_event(&event_bytes, 5),
                Ok(event)
            );
        }
    }

    #[test]
    fn b3b1_telemetry_and_counters_are_typed_canonical_and_bounded() {
        let codec = codec();
        let configuration = TelemetryConfiguration {
            meter_handles: vec![1, 2],
            meter_period_blocks: 4,
            counter_ids: vec![
                CounterId::ControlCommandBackpressure,
                CounterId::TelemetryCoalesced,
            ],
            counter_period_blocks: 8,
            diagnostics_enabled: true,
            minimum_diagnostic_severity: DiagnosticSeverity::Error,
        };
        let mut config_bytes = [0_u8; 96];
        codec
            .encode_telemetry_configuration(&configuration, &mut config_bytes)
            .expect("config");
        assert_eq!(
            hex(&config_bytes),
            concat!(
                "01000d01080000000100000002000000",
                "02000301040000000400000000000000",
                "03000d01080000000100000005000000",
                "04000301040000000800000000000000",
                "05000801010000000100000000000000",
                "06000101010000000300000000000000"
            )
        );
        assert_eq!(
            codec.decode_telemetry_configuration(&config_bytes, 6),
            Ok(configuration.clone())
        );
        for end in 0..config_bytes.len() {
            assert!(
                codec
                    .decode_telemetry_configuration(&config_bytes[..end], 6)
                    .is_err()
            );
        }
        let bad_config = TelemetryConfiguration {
            meter_handles: vec![2, 1],
            ..configuration.clone()
        };
        assert!(
            codec
                .encoded_telemetry_configuration_len(&bad_config)
                .is_err()
        );
        let bad_coupling = TelemetryConfiguration {
            meter_period_blocks: 0,
            ..configuration.clone()
        };
        assert!(
            codec
                .encoded_telemetry_configuration_len(&bad_coupling)
                .is_err()
        );

        let request = CountersRequest {
            all: false,
            ids: vec![1, 5],
        };
        let mut request_bytes = [0_u8; 32];
        codec
            .encode_counters_request(&request, &mut request_bytes)
            .expect("request");
        assert_eq!(
            hex(&request_bytes),
            concat!(
                "01000801010000000000000000000000",
                "02000d00080000000100000005000000"
            )
        );
        assert_eq!(
            codec.decode_counters_request(&request_bytes, 2),
            Ok(request.clone())
        );
        for end in 0..request_bytes.len() {
            assert!(
                codec
                    .decode_counters_request(&request_bytes[..end], 2)
                    .is_err()
            );
        }
        assert!(
            codec
                .encoded_counters_request_len(&CountersRequest {
                    all: true,
                    ids: vec![8]
                })
                .is_err()
        );
        assert!(
            codec
                .encoded_counters_request_len(&CountersRequest {
                    all: false,
                    ids: vec![]
                })
                .is_err()
        );

        let snapshot = CounterSnapshot {
            observed_sample: crate::SampleTime(9),
            values: vec![
                CounterValue {
                    id: CounterId::ControlCommandBackpressure,
                    value: 2,
                },
                CounterValue {
                    id: CounterId::TelemetryCoalesced,
                    value: 3,
                },
            ],
        };
        let mut snapshot_bytes =
            vec![0; codec.encoded_counter_snapshot_len(&snapshot).expect("size")];
        codec
            .encode_counter_snapshot(&snapshot, &mut snapshot_bytes)
            .expect("snapshot");
        assert_eq!(
            codec.decode_counter_snapshot(&snapshot_bytes, 3),
            Ok(snapshot.clone())
        );
        for end in 0..snapshot_bytes.len() {
            assert!(
                codec
                    .decode_counter_snapshot(&snapshot_bytes[..end], 3)
                    .is_err()
            );
        }
        let mut unknown = request_bytes;
        unknown[28] = 99;
        assert_eq!(
            codec.decode_counters_request(&unknown, 2),
            Ok(CountersRequest {
                all: false,
                ids: vec![1, 99]
            })
        );
        for _ in 0..32 {
            config_bytes.fill(0);
            request_bytes.fill(0);
            snapshot_bytes.fill(0);
            assert_eq!(
                codec.encode_telemetry_configuration(&configuration, &mut config_bytes),
                Ok(96)
            );
            assert_eq!(
                codec.encode_counters_request(&request, &mut request_bytes),
                Ok(32)
            );
            assert_eq!(
                codec.encode_counter_snapshot(&snapshot, &mut snapshot_bytes),
                Ok(snapshot_bytes.len())
            );
            assert_eq!(
                codec.decode_counter_snapshot(&snapshot_bytes, 3),
                Ok(snapshot.clone())
            );
        }
    }

    #[test]
    fn b3b2_diagnostics_pages_are_canonical_bounded_and_strict() {
        let codec = codec();
        let request = DiagnosticsRequest {
            after_sequence: 2,
            limit: 3,
            minimum_severity: DiagnosticSeverity::Warning,
        };
        let mut request_bytes = [0_u8; 48];
        codec
            .encode_diagnostics_request(request, &mut request_bytes)
            .expect("request");
        assert_eq!(
            hex(&request_bytes),
            concat!(
                "01000401080000000200000000000000",
                "02000201020000000300000000000000",
                "03000101010000000200000000000000"
            )
        );
        assert_eq!(
            codec.decode_diagnostics_request(&request_bytes, 3),
            Ok(request)
        );
        for end in 0..request_bytes.len() {
            assert!(
                codec
                    .decode_diagnostics_request(&request_bytes[..end], 3)
                    .is_err()
            );
        }
        let mut short = [0xaa; 47];
        assert_eq!(
            codec.encode_diagnostics_request(request, &mut short),
            Err(EncodeError::OutputTooSmall { required: 48 })
        );
        assert!(short.iter().all(|byte| *byte == 0xaa));

        let page_diagnostic = Diagnostic {
            code: "a.b".to_owned(),
            severity: DiagnosticSeverity::Error,
            path: Vec::new(),
            detail: None,
            operation_index: None,
            sample_time: None,
            provider_sequence: Some(3),
        };
        let page = DiagnosticsPage {
            last_sequence: 3,
            eof: true,
            diagnostics: vec![page_diagnostic.clone()],
        };
        let mut page_bytes = vec![
            0_u8;
            codec
                .encoded_diagnostics_page_len(&page)
                .expect("page length")
        ];
        codec
            .encode_diagnostics_page(&page, &mut page_bytes)
            .expect("page");
        assert_eq!(
            hex(&page_bytes),
            concat!(
                "01000401080000000300000000000000",
                "02000801010000000100000000000000",
                "03000b013800000003000000000000000100090103000000",
                "612e62000000000002000101010000000300000000000000",
                "07000400080000000300000000000000"
            )
        );
        assert_eq!(
            codec.decode_diagnostics_page(&page_bytes, 3),
            Ok(page.clone())
        );
        for end in 0..page_bytes.len() {
            assert!(
                codec
                    .decode_diagnostics_page(&page_bytes[..end], 3)
                    .is_err()
            );
        }

        let mut bad_request = request_bytes;
        bad_request[24..26].fill(0);
        assert!(codec.decode_diagnostics_request(&bad_request, 3).is_err());
        bad_request = request_bytes;
        bad_request[24..26].copy_from_slice(&257_u16.to_le_bytes());
        assert!(codec.decode_diagnostics_request(&bad_request, 3).is_err());
        bad_request = request_bytes;
        bad_request[40] = 4;
        assert!(codec.decode_diagnostics_request(&bad_request, 3).is_err());
        bad_request = request_bytes;
        bad_request[19] = 0;
        assert!(codec.decode_diagnostics_request(&bad_request, 3).is_err());
        let mut request_extension = request_bytes.to_vec();
        request_extension
            .extend_from_slice(&[4, 0, WIRE_U8, 0, 1, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(
            codec.decode_diagnostics_request(&request_extension, 4),
            Ok(request),
            "unknown optional request extension is skippable"
        );
        request_extension[48 + 3] = 1;
        assert_eq!(
            codec.decode_diagnostics_request(&request_extension, 4),
            Err(DecodeError::UnknownRequiredField)
        );

        let mut missing_sequence = page_bytes.clone();
        missing_sequence[80..82].copy_from_slice(&8_u16.to_le_bytes());
        assert!(codec.decode_diagnostics_page(&missing_sequence, 3).is_err());
        let mut optional_diagnostic = page_bytes.clone();
        optional_diagnostic[35] = 0;
        assert!(
            codec
                .decode_diagnostics_page(&optional_diagnostic, 3)
                .is_err()
        );
        let mismatched_last = DiagnosticsPage {
            last_sequence: 4,
            ..page.clone()
        };
        assert!(
            codec
                .encoded_diagnostics_page_len(&mismatched_last)
                .is_err()
        );
        let too_many = DiagnosticsPage {
            last_sequence: 257,
            eof: true,
            diagnostics: (1..=257)
                .map(|sequence| Diagnostic {
                    provider_sequence: Some(sequence),
                    ..page_diagnostic.clone()
                })
                .collect(),
        };
        assert!(codec.encoded_diagnostics_page_len(&too_many).is_err());
        let reversed = DiagnosticsPage {
            last_sequence: 3,
            eof: false,
            diagnostics: vec![
                Diagnostic {
                    provider_sequence: Some(4),
                    ..page_diagnostic.clone()
                },
                page_diagnostic.clone(),
            ],
        };
        assert!(codec.encoded_diagnostics_page_len(&reversed).is_err());

        for _ in 0..64 {
            page_bytes.fill(0);
            assert_eq!(
                codec.encode_diagnostics_page(&page, &mut page_bytes),
                Ok(page_bytes.len())
            );
            assert_eq!(
                codec.decode_diagnostics_page(&page_bytes, 3),
                Ok(page.clone())
            );
            request_bytes.fill(0);
            assert_eq!(
                codec.encode_diagnostics_request(request, &mut request_bytes),
                Ok(48)
            );
            assert_eq!(
                codec.decode_diagnostics_request(&request_bytes, 3),
                Ok(request)
            );
        }
    }

    #[test]
    fn b4_event_payloads_are_typed_canonical_and_truncation_safe() {
        let codec = codec();
        let canceled = AutomationCanceled {
            event_sequence: 1,
            origin_request_id: crate::RequestId::new(2).expect("id"),
            canceled_records: 3,
            reason: AutomationCancellationReason::RevisionChanged,
            queue_generation: 2,
            effective_sample: Some(crate::SampleTime(9)),
        };
        let mut canceled_bytes = [0_u8; 96];
        codec
            .encode_automation_canceled(canceled, &mut canceled_bytes)
            .expect("cancel");
        assert_eq!(
            hex(&canceled_bytes),
            concat!(
                "01000401080000000100000000000000",
                "02000401080000000200000000000000",
                "03000201020000000300000000000000",
                "04000101010000000100000000000000",
                "05000401080000000200000000000000",
                "06000400080000000900000000000000"
            )
        );
        assert_eq!(
            codec.decode_automation_canceled(&canceled_bytes, 6),
            Ok(canceled)
        );
        for end in 0..canceled_bytes.len() {
            assert!(
                codec
                    .decode_automation_canceled(&canceled_bytes[..end], 6)
                    .is_err()
            );
        }
        let mut bad_canceled = canceled_bytes;
        bad_canceled[40..42].fill(0);
        assert!(codec.decode_automation_canceled(&bad_canceled, 6).is_err());
        bad_canceled = canceled_bytes;
        bad_canceled[56] = 9;
        assert!(codec.decode_automation_canceled(&bad_canceled, 6).is_err());

        let records = [MeterRecord {
            handle: 1,
            component: MeterComponent::Right,
            flags: 3,
            value: 1.5,
        }];
        let batch = MeterBatch {
            observed_sample: crate::SampleTime(9),
            records: &records,
        };
        let mut meter_bytes = [0_u8; 72];
        codec
            .encode_meter_batch(batch, &mut meter_bytes)
            .expect("meter");
        assert_eq!(
            hex(&meter_bytes),
            concat!(
                "01000401080000000900000000000000",
                "02000201020000000100000000000000",
                "03000201020000001000000000000000",
                "04000a011000000001000000020003000000c03f00000000"
            )
        );
        let decoded = codec
            .decode_meter_batch(&meter_bytes, 4)
            .expect("decoded meter");
        assert_eq!(decoded.observed_sample, crate::SampleTime(9));
        assert_eq!(decoded.record(0), Ok(records[0]));
        for end in 0..meter_bytes.len() {
            assert!(codec.decode_meter_batch(&meter_bytes[..end], 4).is_err());
        }
        let mut bad_meter = meter_bytes;
        bad_meter[40..42].copy_from_slice(&15_u16.to_le_bytes());
        assert!(codec.decode_meter_batch(&bad_meter, 4).is_err());
        bad_meter = meter_bytes;
        bad_meter[64..68].copy_from_slice(&f32::NAN.to_le_bytes());
        assert!(codec.decode_meter_batch(&bad_meter, 4).is_err());
        bad_meter = meter_bytes;
        bad_meter[68] = 1;
        assert!(codec.decode_meter_batch(&bad_meter, 4).is_err());

        let counters = CounterSnapshot {
            observed_sample: crate::SampleTime(9),
            values: vec![CounterValue {
                id: CounterId::TelemetryCoalesced,
                value: 2,
            }],
        };
        let mut counter_bytes = vec![
            0;
            codec
                .encoded_counter_snapshot_len(&counters)
                .expect("counter len")
        ];
        codec
            .encode_counter_snapshot_event(&counters, &mut counter_bytes)
            .expect("counter event");
        assert_eq!(
            codec.decode_counter_snapshot_event(&counter_bytes, 2),
            Ok(counters.clone())
        );
        for end in 0..counter_bytes.len() {
            assert!(
                codec
                    .decode_counter_snapshot_event(&counter_bytes[..end], 2)
                    .is_err()
            );
        }

        let diagnostic = DiagnosticEvent {
            diagnostic: Diagnostic {
                code: "a.b".to_owned(),
                severity: DiagnosticSeverity::Error,
                path: Vec::new(),
                detail: None,
                operation_index: None,
                sample_time: None,
                provider_sequence: Some(3),
            },
        };
        let mut diagnostic_bytes = [0_u8; 64];
        codec
            .encode_diagnostic_event(&diagnostic, &mut diagnostic_bytes)
            .expect("diagnostic event");
        assert_eq!(
            hex(&diagnostic_bytes),
            concat!(
                "01000b01380000000300000000000000",
                "0100090103000000612e620000000000",
                "02000101010000000300000000000000",
                "07000400080000000300000000000000"
            )
        );
        assert_eq!(
            codec.decode_diagnostic_event(&diagnostic_bytes, 1),
            Ok(diagnostic.clone())
        );
        for end in 0..diagnostic_bytes.len() {
            assert!(
                codec
                    .decode_diagnostic_event(&diagnostic_bytes[..end], 1)
                    .is_err()
            );
        }
        let no_sequence = DiagnosticEvent {
            diagnostic: Diagnostic {
                provider_sequence: None,
                ..diagnostic.diagnostic.clone()
            },
        };
        assert!(codec.encoded_diagnostic_event_len(&no_sequence).is_err());

        let assert_event_frame =
            |message_id: crate::MessageId, payload: &[u8], tlv_count: u32, expected: &str| {
                let mut frame = vec![0_u8; crate::OUTER_HEADER_BYTES + payload.len()];
                codec
                    .write_outer_header(
                        &mut frame,
                        crate::FrameKind::Event,
                        message_id,
                        crate::StatusCode::Ok,
                        0,
                        7,
                        0,
                        payload.len() as u32,
                        tlv_count,
                    )
                    .expect("event header");
                frame[crate::OUTER_HEADER_BYTES..].copy_from_slice(payload);
                assert_eq!(hex(&frame), expected);
                assert_eq!(&frame[crate::OUTER_HEADER_BYTES..], payload);
                for end in 0..frame.len() {
                    assert!(
                        codec
                            .decode(
                                &frame[..end],
                                &mut crate::DecodeScratch::new(&mut [0_u16; 8]),
                            )
                            .is_err()
                    );
                }
            };
        assert_event_frame(
            crate::MessageId::AutomationCanceled,
            &canceled_bytes,
            6,
            concat!(
                "4d49534f43544c00010000003000030002800000600000000000000000000000",
                "07000000000000000600000000000000",
                "0100040108000000010000000000000002000401080000000200000000000000",
                "0300020102000000030000000000000004000101010000000100000000000000",
                "0500040108000000020000000000000006000400080000000900000000000000"
            ),
        );
        assert_event_frame(
            crate::MessageId::MeterBatch,
            &meter_bytes,
            4,
            concat!(
                "4d49534f43544c00010000003000030020800000480000000000000000000000",
                "07000000000000000400000000000000",
                "0100040108000000090000000000000002000201020000000100000000000000",
                "0300020102000000100000000000000004000a01100000000100000002000300",
                "0000c03f00000000"
            ),
        );
        assert_event_frame(
            crate::MessageId::CounterSnapshot,
            &counter_bytes,
            2,
            concat!(
                "4d49534f43544c00010000003000030021800000400000000000000000000000",
                "07000000000000000200000000000000",
                "0100040108000000090000000000000002000b01280000000200000000000000",
                "0100030104000000050000000000000002000401080000000200000000000000"
            ),
        );
        assert_event_frame(
            crate::MessageId::Diagnostic,
            &diagnostic_bytes,
            1,
            concat!(
                "4d49534f43544c00010000003000030030800000400000000000000000000000",
                "07000000000000000100000000000000",
                "01000b013800000003000000000000000100090103000000612e620000000000",
                "0200010101000000030000000000000007000400080000000300000000000000"
            ),
        );
        let mut forbidden = [0_u8; crate::OUTER_HEADER_BYTES];
        codec
            .write_outer_header(
                &mut forbidden,
                crate::FrameKind::Event,
                crate::MessageId::MeterBatch,
                crate::StatusCode::Ok,
                0,
                7,
                0,
                0,
                0,
            )
            .expect("only a typed event message ID can be encoded");
        forbidden[16..18].copy_from_slice(&0x6000_u16.to_le_bytes());
        assert!(
            matches!(
                codec.decode(&forbidden, &mut crate::DecodeScratch::new(&mut [0_u16; 0]),),
                Err(DecodeError::PcmForbidden)
            ),
            "a reserved PCM ID cannot be represented by a typed event encoder"
        );

        for _ in 0..32 {
            canceled_bytes.fill(0);
            meter_bytes.fill(0);
            diagnostic_bytes.fill(0);
            counter_bytes.fill(0);
            assert_eq!(
                codec.encode_automation_canceled(canceled, &mut canceled_bytes),
                Ok(96)
            );
            assert_eq!(codec.encode_meter_batch(batch, &mut meter_bytes), Ok(72));
            assert_eq!(
                codec.encode_diagnostic_event(&diagnostic, &mut diagnostic_bytes),
                Ok(64)
            );
            assert_eq!(
                codec.encode_counter_snapshot_event(&counters, &mut counter_bytes),
                Ok(counter_bytes.len())
            );
        }
    }

    fn hex(bytes: &[u8]) -> String {
        let mut output = String::with_capacity(bytes.len() * 2);
        for byte in bytes {
            use core::fmt::Write as _;
            write!(&mut output, "{byte:02x}").expect("string write");
        }
        output
    }
}
