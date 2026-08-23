//! Schema-specific common BTLV payloads shared by typed command responses.
//!
//! This module deliberately exposes typed values only. It has no arbitrary-message, arbitrary
//! field, or opaque byte-payload encoder. Encoding is a checked sizing pass followed by one
//! caller-buffer write pass; neither pass allocates.

use crate::{
    DecodeError, EncodeError, ProtocolCodec,
    btlv::{
        CountSink, Fields as Message, Sink, SliceSink, read_f32, read_u8, read_u16, read_u32,
        read_u64,
    },
    schema::{self, descriptor, enum_choice},
};

#[cfg(test)]
const WIRE_U8: u8 = 1;
#[cfg(test)]
const WIRE_U32: u8 = 3;

macro_rules! write_spec {
    ($writer:expr, $spec:expr, $bytes:expr $(,)?) => {{
        let spec = $spec;
        $writer.field_spec(spec, $bytes)
    }};
}

macro_rules! one_spec {
    ($message:expr, $spec:expr $(,)?) => {{
        let spec = $spec;
        $message.one(spec.id, spec.wire.raw())
    }};
}

macro_rules! optional_spec {
    ($message:expr, $spec:expr $(,)?) => {{
        let spec = $spec;
        $message.optional_one(spec.id, spec.wire.raw())
    }};
}

macro_rules! values_spec {
    ($message:expr, $spec:expr $(,)?) => {{
        let spec = $spec;
        $message.values(spec.id, spec.wire.raw())
    }};
}

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
///
/// This remains distinct from [`Capabilities`] for workspace source compatibility: existing
/// consumers construct `Capabilities` with native `u16` slice literals, while decode must borrow
/// allocation-free packed little-endian bytes from the input frame.
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

impl ParameterRack {
    pub(crate) const fn from_session(value: miso_engine_session::RackName) -> Self {
        match value {
            miso_engine_session::RackName::Simd1 => Self::Simd1,
            miso_engine_session::RackName::Dynamic => Self::Dynamic,
            miso_engine_session::RackName::Simd2 => Self::Simd2,
        }
    }

    pub(crate) const fn into_session(self) -> miso_engine_session::RackName {
        match self {
            Self::Simd1 => miso_engine_session::RackName::Simd1,
            Self::Dynamic => miso_engine_session::RackName::Dynamic,
            Self::Simd2 => miso_engine_session::RackName::Simd2,
        }
    }
}

impl From<miso_engine_session::RackName> for ParameterRack {
    fn from(value: miso_engine_session::RackName) -> Self {
        Self::from_session(value)
    }
}

impl From<ParameterRack> for miso_engine_session::RackName {
    fn from(value: ParameterRack) -> Self {
        value.into_session()
    }
}

impl ParameterChannel {
    pub(crate) const fn from_session(value: miso_engine_session::ParameterChannel) -> Self {
        match value {
            miso_engine_session::ParameterChannel::Left => Self::Left,
            miso_engine_session::ParameterChannel::Right => Self::Right,
            miso_engine_session::ParameterChannel::Both => Self::Both,
        }
    }

    pub(crate) const fn into_session(self) -> miso_engine_session::ParameterChannel {
        match self {
            Self::Left => miso_engine_session::ParameterChannel::Left,
            Self::Right => miso_engine_session::ParameterChannel::Right,
            Self::Both => miso_engine_session::ParameterChannel::Both,
        }
    }
}

impl From<miso_engine_session::ParameterChannel> for ParameterChannel {
    fn from(value: miso_engine_session::ParameterChannel) -> Self {
        Self::from_session(value)
    }
}

impl From<ParameterChannel> for miso_engine_session::ParameterChannel {
    fn from(value: ParameterChannel) -> Self {
        value.into_session()
    }
}

impl ParameterUnit {
    pub(crate) const fn from_session(value: miso_engine_session::ParameterUnit) -> Self {
        match value {
            miso_engine_session::ParameterUnit::Db => Self::Db,
            miso_engine_session::ParameterUnit::Hz => Self::Hz,
            miso_engine_session::ParameterUnit::Milliseconds => Self::Milliseconds,
            miso_engine_session::ParameterUnit::Samples => Self::Samples,
            miso_engine_session::ParameterUnit::Linear => Self::Linear,
            miso_engine_session::ParameterUnit::Ratio => Self::Ratio,
        }
    }

    pub(crate) const fn into_session(self) -> miso_engine_session::ParameterUnit {
        match self {
            Self::Db => miso_engine_session::ParameterUnit::Db,
            Self::Hz => miso_engine_session::ParameterUnit::Hz,
            Self::Milliseconds => miso_engine_session::ParameterUnit::Milliseconds,
            Self::Samples => miso_engine_session::ParameterUnit::Samples,
            Self::Linear => miso_engine_session::ParameterUnit::Linear,
            Self::Ratio => miso_engine_session::ParameterUnit::Ratio,
        }
    }
}

impl From<miso_engine_session::ParameterUnit> for ParameterUnit {
    fn from(value: miso_engine_session::ParameterUnit) -> Self {
        Self::from_session(value)
    }
}

impl From<ParameterUnit> for miso_engine_session::ParameterUnit {
    fn from(value: ParameterUnit) -> Self {
        value.into_session()
    }
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
        let mut sink = CountSink::new(self.limits());
        write_non_ok(self, &mut sink, value)?;
        checked_sink_len(self, &mut sink)
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
        let mut writer = SliceSink::new(output, self.limits());
        write_non_ok(self, &mut writer, value)?;
        checked_writer_len(&writer, required)
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
        decode_non_ok(self, Message::top_level(payload, tlv_count, self.limits())?)
    }

    /// Return the exact canonical nested-message length for one typed diagnostic.
    pub fn encoded_diagnostic_message_len(&self, value: &Diagnostic) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_diagnostic_message(self, &mut sink, value)?;
        checked_sink_len(self, &mut sink)
    }

    /// Decode one nested typed diagnostic payload.
    pub fn decode_diagnostic_message(&self, value: &[u8]) -> Result<Diagnostic, DecodeError> {
        decode_diagnostic(self, Message::nested_at_depth(value, self.limits(), 0)?)
    }

    /// Exact payload length for the 27-field successful capabilities response.
    pub fn encoded_capabilities_len(&self, value: &Capabilities<'_>) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_capabilities(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
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
        let mut writer = SliceSink::new(output, self.limits());
        write_capabilities(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode the exact 27-field successful capabilities payload.
    pub fn decode_capabilities<'a>(
        &self,
        payload: &'a [u8],
        count: u32,
    ) -> Result<DecodedCapabilities<'a>, DecodeError> {
        decode_capabilities(self, Message::top_level(payload, count, self.limits())?)
    }

    /// Validate the empty capabilities command payload, allowing only skippable future fields.
    pub fn decode_capabilities_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<(), DecodeError> {
        Message::top_level(payload, count, self.limits())?
            .schema_spec(&schema::capabilities_request::SPEC)
            .map(|_| ())
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
        let mut writer = SliceSink::new(output, self.limits());
        write_snapshot_request(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode the typed two-field snapshot request.
    pub fn decode_snapshot_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<SessionSnapshotRequest, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::snapshot_request::SPEC)?;
        let result = SessionSnapshotRequest {
            offset: read_u64(one_spec!(message, schema::snapshot_request::OFFSET)?)?,
            maximum_bytes: read_u32(one_spec!(message, schema::snapshot_request::MAXIMUM_BYTES)?)?,
        };
        if result.maximum_bytes == 0 {
            return Err(DecodeError::InvalidTlv);
        }
        Ok(result)
    }

    /// Exact caller-output length for a snapshot success payload.
    pub fn encoded_snapshot_len(&self, value: SessionSnapshot<'_>) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_snapshot(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
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
        let mut writer = SliceSink::new(output, self.limits());
        write_snapshot(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a typed snapshot success payload.
    pub fn decode_snapshot<'a>(
        &self,
        payload: &'a [u8],
        count: u32,
    ) -> Result<SessionSnapshot<'a>, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::snapshot::SPEC)?;
        let result = SessionSnapshot {
            total_bytes: read_u64(one_spec!(message, schema::snapshot::TOTAL_BYTES)?)?,
            offset: read_u64(one_spec!(message, schema::snapshot::OFFSET)?)?,
            canonical_toml_chunk: one_spec!(message, schema::snapshot::CANONICAL_TOML_CHUNK)?,
            eof: read_bool(one_spec!(message, schema::snapshot::EOF)?)?,
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
        let mut sizing = CountSink::new(self.limits());
        write_transaction_applied(&mut sizing, value)?;
        let required = checked_sink_len(self, &mut sizing)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_transaction_applied(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode the one-field transaction success payload.
    pub fn decode_transaction_applied(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<TransactionApplied, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::transaction_applied::SPEC)?;
        Ok(TransactionApplied {
            applied_operations: read_u32(one_spec!(
                message,
                schema::transaction_applied::APPLIED_OPERATIONS
            )?)?,
        })
    }

    /// Encode/decode helpers for the four-field reliable session-committed event.
    pub fn encode_session_committed(
        &self,
        value: SessionCommitted,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let mut sizing = CountSink::new(self.limits());
        write_session_committed(&mut sizing, value)?;
        let required = checked_sink_len(self, &mut sizing)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_session_committed(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a reliable session-committed event payload.
    pub fn decode_session_committed(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<SessionCommitted, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::session_committed::SPEC)?;
        Ok(SessionCommitted {
            event_sequence: read_u64(one_spec!(
                message,
                schema::session_committed::EVENT_SEQUENCE
            )?)?,
            origin_request_id: crate::RequestId::new(read_u64(one_spec!(
                message,
                schema::session_committed::ORIGIN_REQUEST_ID
            )?)?)
            .ok_or(DecodeError::InvalidTlv)?,
            previous_revision: crate::SessionRevision(read_u64(one_spec!(
                message,
                schema::session_committed::PREVIOUS_REVISION
            )?)?),
            applied_operations: read_u32(one_spec!(
                message,
                schema::session_committed::APPLIED_OPERATIONS
            )?)?,
        })
    }

    /// Encode/decode the exact two-field metadata cursor request.
    pub fn encode_parameter_metadata_request(
        &self,
        value: ParameterMetadataRequest,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let mut sizing = CountSink::new(self.limits());
        write_metadata_request(&mut sizing, value)?;
        let required = checked_sink_len(self, &mut sizing)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_metadata_request(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }
    /// Strictly decode the exact two-field metadata cursor request.
    pub fn decode_parameter_metadata_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<ParameterMetadataRequest, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::metadata_request::SPEC)?;
        let value = ParameterMetadataRequest {
            after_handle: read_u32(one_spec!(message, schema::metadata_request::AFTER_HANDLE)?)?,
            limit: read_u16(one_spec!(message, schema::metadata_request::LIMIT)?)?,
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
        let mut sink = CountSink::new(self.limits());
        write_metadata_page(self, &mut sink, value)?;
        checked_sink_len(self, &mut sink)
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
        let mut writer = SliceSink::new(output, self.limits());
        write_metadata_page(self, &mut writer, value)?;
        checked_writer_len(&writer, required)
    }
    /// Strictly decode a typed metadata page into bounded typed descriptors.
    pub fn decode_parameter_metadata_page(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<ParameterMetadataPage, DecodeError> {
        decode_metadata_page(self, Message::top_level(payload, count, self.limits())?)
    }
    /// Encode a bounded sorted unique nonzero state-handle request without allocation.
    pub fn encoded_parameter_state_request_len(
        &self,
        value: &ParameterStateRequest,
    ) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_state_request(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
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
        let mut writer = SliceSink::new(output, self.limits());
        write_state_request(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }
    /// Strictly decode a bounded sorted unique nonzero state-handle request.
    pub fn decode_parameter_state_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<ParameterStateRequest, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::state_request::SPEC)?;
        let bytes = one_spec!(message, schema::state_request::HANDLES)?;
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
        let mut sink = CountSink::new(self.limits());
        write_state_page(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
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
        let mut writer = SliceSink::new(output, self.limits());
        write_state_page(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }
    /// Strictly decode fixed 16-byte state records.
    pub fn decode_parameter_state_page(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<ParameterStatePage, DecodeError> {
        decode_state_page(self, Message::top_level(payload, count, self.limits())?)
    }

    /// Return the exact direct caller-buffer length for one typed automation command payload.
    pub fn encoded_automation_enqueue_len(
        &self,
        value: AutomationEnqueue<'_>,
    ) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_automation_enqueue(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
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
        let mut writer = SliceSink::new(output, self.limits());
        write_automation_enqueue(&mut writer, value)?;
        checked_writer_len(&writer, required)
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
        let mut sizing = CountSink::new(self.limits());
        write_automation_enqueued(&mut sizing, value)?;
        let required = checked_sink_len(self, &mut sizing)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_automation_enqueued(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode the exact four-field successful automation admission payload.
    pub fn decode_automation_enqueued(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<AutomationEnqueued, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::automation_enqueued::SPEC)?;
        let result = AutomationEnqueued {
            accepted_records: read_u16(one_spec!(
                message,
                schema::automation_enqueued::ACCEPTED_RECORDS
            )?)?,
            occupancy: read_u64(one_spec!(message, schema::automation_enqueued::OCCUPANCY)?)?,
            capacity: read_u64(one_spec!(message, schema::automation_enqueued::CAPACITY)?)?,
            generation: read_u64(one_spec!(message, schema::automation_enqueued::GENERATION)?)?,
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
        Message::top_level(payload, count, self.limits())?
            .schema_spec(&schema::transport_get::SPEC)
            .map(|_| ())
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
        let mut writer = SliceSink::new(output, self.limits());
        write_transport_set(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a typed absolute transport-state set request without allocation.
    pub fn decode_transport_set_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<TransportSetRequest, DecodeError> {
        let fields = Message::top_level(payload, count, self.limits())?
            .schema_spec(&schema::transport_set::SPEC)?;
        Ok(TransportSetRequest {
            state: parse_transport_state(read_u8(one_spec!(
                fields,
                schema::transport_set::STATE
            )?)?)?,
            position: optional_spec!(fields, schema::transport_set::POSITION)?
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
        let mut sizing = CountSink::new(self.limits());
        write_transport_snapshot(&mut sizing, value)?;
        let required = checked_sink_len(self, &mut sizing)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_transport_snapshot(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a typed three-field transport snapshot without allocation.
    pub fn decode_transport_snapshot(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<TransportSnapshot, DecodeError> {
        let fields = Message::top_level(payload, count, self.limits())?
            .schema_spec(&schema::transport_snapshot::SPEC)?;
        Ok(TransportSnapshot {
            state: parse_transport_state(read_u8(one_spec!(
                fields,
                schema::transport_snapshot::STATE
            )?)?)?,
            position: crate::SampleTime(read_u64(one_spec!(
                fields,
                schema::transport_snapshot::POSITION
            )?)?),
            effective_sample: crate::SampleTime(read_u64(one_spec!(
                fields,
                schema::transport_snapshot::EFFECTIVE_SAMPLE
            )?)?),
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
        let mut writer = SliceSink::new(output, self.limits());
        write_transport_state_event(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a reliable typed transport-state event payload without allocation.
    pub fn decode_transport_state_event(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<TransportStateEvent, DecodeError> {
        let fields = Message::top_level(payload, count, self.limits())?
            .schema_spec(&schema::transport_state_event::SPEC)?;
        Ok(TransportStateEvent {
            event_sequence: read_u64(one_spec!(
                fields,
                schema::transport_state_event::EVENT_SEQUENCE
            )?)?,
            state: parse_transport_state(read_u8(one_spec!(
                fields,
                schema::transport_state_event::STATE
            )?)?)?,
            position: crate::SampleTime(read_u64(one_spec!(
                fields,
                schema::transport_state_event::POSITION
            )?)?),
            effective_sample: crate::SampleTime(read_u64(one_spec!(
                fields,
                schema::transport_state_event::EFFECTIVE_SAMPLE
            )?)?),
            origin_request_id: match optional_spec!(
                fields,
                schema::transport_state_event::ORIGIN_REQUEST_ID
            )?
            .map(read_u64)
            .transpose()?
            {
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
        let required = self.encoded_automation_canceled_len(value);
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_automation_canceled(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a reliable typed `AUTOMATION_CANCELED` payload.
    pub fn decode_automation_canceled(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<AutomationCanceled, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::automation_canceled::SPEC)?;
        let value = AutomationCanceled {
            event_sequence: read_u64(one_spec!(
                message,
                schema::automation_canceled::EVENT_SEQUENCE
            )?)?,
            origin_request_id: crate::RequestId::new(read_u64(one_spec!(
                message,
                schema::automation_canceled::ORIGIN_REQUEST_ID
            )?)?)
            .ok_or(DecodeError::InvalidTlv)?,
            canceled_records: read_u16(one_spec!(
                message,
                schema::automation_canceled::CANCELED_RECORDS
            )?)?,
            reason: parse_automation_cancellation_reason(read_u8(one_spec!(
                message,
                schema::automation_canceled::REASON
            )?)?)?,
            queue_generation: read_u64(one_spec!(
                message,
                schema::automation_canceled::QUEUE_GENERATION
            )?)?,
            effective_sample: optional_spec!(
                message,
                schema::automation_canceled::EFFECTIVE_SAMPLE
            )?
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
        let mut sink = CountSink::new(self.limits());
        write_meter_batch(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
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
        let mut writer = SliceSink::new(output, self.limits());
        write_meter_batch(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a bounded borrowed fixed-record lossy `METER_BATCH` payload.
    pub fn decode_meter_batch<'a>(
        &self,
        payload: &'a [u8],
        count: u32,
    ) -> Result<DecodedMeterBatch<'a>, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::meter_batch::SPEC)?;
        let result = DecodedMeterBatch {
            observed_sample: crate::SampleTime(read_u64(one_spec!(
                message,
                schema::meter_batch::OBSERVED_SAMPLE
            )?)?),
            count: read_u16(one_spec!(message, schema::meter_batch::COUNT)?)?,
            record_bytes: one_spec!(message, schema::meter_batch::RECORDS)?,
        };
        if result.count == 0
            || result.count > 256
            || read_u16(one_spec!(message, schema::meter_batch::RECORD_BYTES)?)? != 16
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
        let mut sink = CountSink::new(self.limits());
        write_diagnostic_event(self, &mut sink, diagnostic)?;
        checked_sink_len(self, &mut sink)
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
        let mut writer = SliceSink::new(output, self.limits());
        write_diagnostic_event(self, &mut writer, diagnostic)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode one reliable typed diagnostic event requiring provider sequence field 7.
    pub fn decode_diagnostic_event(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<DiagnosticEvent, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::diagnostic_event::SPEC)?;
        let diagnostic = decode_diagnostic(
            self,
            Message::nested_at_depth(
                one_spec!(message, schema::diagnostic_event::DIAGNOSTIC)?,
                self.limits(),
                1,
            )?,
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
        let mut sink = CountSink::new(self.limits());
        write_telemetry_configuration(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
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
        let mut writer = SliceSink::new(output, self.limits());
        write_telemetry_configuration(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a telemetry configuration command or canonical success echo.
    pub fn decode_telemetry_configuration(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<TelemetryConfiguration, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::telemetry_configuration::SPEC)?;
        let result = TelemetryConfiguration {
            meter_handles: decode_nonzero_u32s(
                one_spec!(message, schema::telemetry_configuration::METER_HANDLES)?,
                true,
            )?,
            meter_period_blocks: read_u32(one_spec!(
                message,
                schema::telemetry_configuration::METER_PERIOD_BLOCKS
            )?)?,
            counter_ids: decode_counter_ids(one_spec!(
                message,
                schema::telemetry_configuration::COUNTER_IDS
            )?)?,
            counter_period_blocks: read_u32(one_spec!(
                message,
                schema::telemetry_configuration::COUNTER_PERIOD_BLOCKS
            )?)?,
            diagnostics_enabled: read_bool(one_spec!(
                message,
                schema::telemetry_configuration::DIAGNOSTICS_ENABLED
            )?)?,
            minimum_diagnostic_severity: DiagnosticSeverity::decode(read_u8(one_spec!(
                message,
                schema::telemetry_configuration::MINIMUM_DIAGNOSTIC_SEVERITY
            )?)?)?,
        };
        validate_telemetry_configuration(&result).map_err(|_| DecodeError::InvalidTlv)?;
        Ok(result)
    }

    /// Return exact caller-buffer bytes for one typed counters selector.
    pub fn encoded_counters_request_len(
        &self,
        value: &CountersRequest,
    ) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_counters_request(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
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
        let mut writer = SliceSink::new(output, self.limits());
        write_counters_request(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode an all-or-explicit-ID counters selector.
    pub fn decode_counters_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<CountersRequest, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::counters_request::SPEC)?;
        let all = read_bool(one_spec!(message, schema::counters_request::ALL)?)?;
        let ids = optional_spec!(message, schema::counters_request::IDS)?
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
        let mut sink = CountSink::new(self.limits());
        write_counter_snapshot(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
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
        let mut writer = SliceSink::new(output, self.limits());
        write_counter_snapshot(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a typed ascending nondestructive counter snapshot.
    pub fn decode_counter_snapshot(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<CounterSnapshot, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::counter_snapshot::SPEC)?;
        let mut values =
            Vec::with_capacity(values_spec!(message, schema::counter_snapshot::VALUE)?.count());
        for raw in values_spec!(message, schema::counter_snapshot::VALUE)? {
            let counter = Message::nested_at_depth(raw, self.limits(), 1)?;
            let counter = counter.schema_spec(&schema::counter_value::SPEC)?;
            values.push(CounterValue {
                id: parse_counter_id(read_u32(one_spec!(counter, schema::counter_value::ID)?)?)?,
                value: read_u64(one_spec!(counter, schema::counter_value::VALUE)?)?,
            });
        }
        let result = CounterSnapshot {
            observed_sample: crate::SampleTime(read_u64(one_spec!(
                message,
                schema::counter_snapshot::OBSERVED_SAMPLE
            )?)?),
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
        let mut sizing = CountSink::new(self.limits());
        write_diagnostics_request(&mut sizing, value)?;
        let required = checked_sink_len(self, &mut sizing)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_diagnostics_request(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a typed diagnostics cursor request.
    pub fn decode_diagnostics_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<DiagnosticsRequest, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::diagnostics_request::SPEC)?;
        let value = DiagnosticsRequest {
            after_sequence: read_u64(one_spec!(
                message,
                schema::diagnostics_request::AFTER_SEQUENCE
            )?)?,
            limit: read_u16(one_spec!(message, schema::diagnostics_request::LIMIT)?)?,
            minimum_severity: DiagnosticSeverity::decode(read_u8(one_spec!(
                message,
                schema::diagnostics_request::MINIMUM_SEVERITY
            )?)?)?,
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
        let mut sink = CountSink::new(self.limits());
        write_diagnostics_page(self, &mut sink, value)?;
        checked_sink_len(self, &mut sink)
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
        let mut writer = SliceSink::new(output, self.limits());
        write_diagnostics_page(self, &mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a typed bounded diagnostics page.
    pub fn decode_diagnostics_page(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<DiagnosticsPage, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::diagnostics_page::SPEC)?;
        let diagnostics = values_spec!(message, schema::diagnostics_page::DIAGNOSTIC)?
            .map(|raw| decode_diagnostic(self, Message::nested_at_depth(raw, self.limits(), 1)?))
            .collect::<Result<Vec<_>, _>>()?;
        let value = DiagnosticsPage {
            last_sequence: read_u64(one_spec!(message, schema::diagnostics_page::LAST_SEQUENCE)?)?,
            eof: read_bool(one_spec!(message, schema::diagnostics_page::EOF)?)?,
            diagnostics,
        };
        validate_diagnostics_page(&value).map_err(|_| DecodeError::InvalidTlv)?;
        Ok(value)
    }
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
    sink: &mut dyn Sink,
    spec: schema::FieldSpec,
    values: &[CounterId],
) -> Result<(), EncodeError> {
    let length = values
        .len()
        .checked_mul(4)
        .ok_or(EncodeError::LimitExceeded)?;
    sink.stream_field_spec(spec, length, &mut |sink| {
        for value in values {
            sink.raw(&(*value as u32).to_le_bytes())?;
        }
        Ok(())
    })
}

fn write_u32s(
    sink: &mut dyn Sink,
    spec: schema::FieldSpec,
    values: &[u32],
) -> Result<(), EncodeError> {
    let length = values
        .len()
        .checked_mul(4)
        .ok_or(EncodeError::LimitExceeded)?;
    sink.stream_field_spec(spec, length, &mut |sink| {
        for value in values {
            sink.raw(&value.to_le_bytes())?;
        }
        Ok(())
    })
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
    sink: &mut dyn Sink,
    records: &[crate::AutomationRecord],
) -> Result<(), EncodeError> {
    let length = records
        .len()
        .checked_mul(crate::AUTOMATION_RECORD_BYTES)
        .ok_or(EncodeError::LimitExceeded)?;
    sink.stream_field_spec(schema::automation_enqueue::RECORDS, length, &mut |sink| {
        for record in records {
            let mut bytes = [0_u8; crate::AUTOMATION_RECORD_BYTES];
            record
                .encode_le(&mut bytes)
                .map_err(|_| EncodeError::LimitExceeded)?;
            sink.raw(&bytes)?;
        }
        Ok(())
    })
}

/// Automation uses the shared allocation-free reader while borrowing its fixed record array.
fn decode_automation_enqueue<'a>(
    codec: &ProtocolCodec,
    payload: &'a [u8],
    count: u32,
) -> Result<DecodedAutomationEnqueue<'a>, DecodeError> {
    if count > codec.limits().max_tlv_count || payload.len() > codec.limits().max_frame_bytes {
        return Err(DecodeError::LimitExceeded);
    }
    let fields = Message::top_level(payload, count, codec.limits())?
        .schema_spec(&schema::automation_enqueue::SPEC)?;
    let count = read_u16(one_spec!(fields, schema::automation_enqueue::COUNT)?)?;
    let stride = read_u16(one_spec!(fields, schema::automation_enqueue::RECORD_BYTES)?)?;
    let records = one_spec!(fields, schema::automation_enqueue::RECORDS)?;
    if count == 0
        || usize::from(count) > crate::AUTOMATION_BATCH_RECORDS
        || stride != crate::AUTOMATION_RECORD_BYTES as u16
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

pub(crate) fn write_snapshot_request(
    sink: &mut dyn Sink,
    value: SessionSnapshotRequest,
) -> Result<(), EncodeError> {
    if value.maximum_bytes == 0 {
        return Err(EncodeError::LimitExceeded);
    }
    sink.check_field_count(schema::snapshot_request::SPEC.field_count(&[])?)?;
    write_spec!(
        sink,
        schema::snapshot_request::OFFSET,
        &value.offset.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::snapshot_request::MAXIMUM_BYTES,
        &value.maximum_bytes.to_le_bytes()
    )
}

pub(crate) fn write_snapshot(
    sink: &mut dyn Sink,
    value: SessionSnapshot<'_>,
) -> Result<(), EncodeError> {
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
    sink.check_field_count(schema::snapshot::SPEC.field_count(&[])?)?;
    write_spec!(
        sink,
        schema::snapshot::TOTAL_BYTES,
        &value.total_bytes.to_le_bytes()
    )?;
    write_spec!(sink, schema::snapshot::OFFSET, &value.offset.to_le_bytes())?;
    write_spec!(
        sink,
        schema::snapshot::CANONICAL_TOML_CHUNK,
        value.canonical_toml_chunk
    )?;
    write_spec!(sink, schema::snapshot::EOF, &[u8::from(value.eof)])
}

pub(crate) fn write_transaction_applied(
    sink: &mut dyn Sink,
    value: TransactionApplied,
) -> Result<(), EncodeError> {
    sink.check_field_count(schema::transaction_applied::SPEC.field_count(&[])?)?;
    write_spec!(
        sink,
        schema::transaction_applied::APPLIED_OPERATIONS,
        &value.applied_operations.to_le_bytes()
    )
}

pub(crate) fn write_session_committed(
    sink: &mut dyn Sink,
    value: SessionCommitted,
) -> Result<(), EncodeError> {
    sink.check_field_count(schema::session_committed::SPEC.field_count(&[])?)?;
    write_spec!(
        sink,
        schema::session_committed::EVENT_SEQUENCE,
        &value.event_sequence.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::session_committed::ORIGIN_REQUEST_ID,
        &value.origin_request_id.get().to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::session_committed::PREVIOUS_REVISION,
        &value.previous_revision.0.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::session_committed::APPLIED_OPERATIONS,
        &value.applied_operations.to_le_bytes()
    )
}

pub(crate) fn write_metadata_request(
    sink: &mut dyn Sink,
    value: ParameterMetadataRequest,
) -> Result<(), EncodeError> {
    if value.limit == 0 || value.limit > 256 {
        return Err(EncodeError::LimitExceeded);
    }
    sink.check_field_count(schema::metadata_request::SPEC.field_count(&[])?)?;
    write_spec!(
        sink,
        schema::metadata_request::AFTER_HANDLE,
        &value.after_handle.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::metadata_request::LIMIT,
        &value.limit.to_le_bytes()
    )
}

pub(crate) fn write_state_request(
    sink: &mut dyn Sink,
    value: &ParameterStateRequest,
) -> Result<(), EncodeError> {
    check_handles(&value.handles).map_err(|_| EncodeError::LimitExceeded)?;
    sink.check_field_count(schema::state_request::SPEC.field_count(&[])?)?;
    write_u32s(sink, schema::state_request::HANDLES, &value.handles)
}

pub(crate) fn write_automation_enqueue(
    sink: &mut dyn Sink,
    value: AutomationEnqueue<'_>,
) -> Result<(), EncodeError> {
    validate_automation_records(value.records).map_err(|_| EncodeError::LimitExceeded)?;
    sink.check_field_count(schema::automation_enqueue::SPEC.field_count(&[])?)?;
    let count = u16::try_from(value.records.len()).map_err(|_| EncodeError::LimitExceeded)?;
    write_spec!(
        sink,
        schema::automation_enqueue::COUNT,
        &count.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::automation_enqueue::RECORD_BYTES,
        &(crate::AUTOMATION_RECORD_BYTES as u16).to_le_bytes()
    )?;
    write_automation_record_bytes(sink, value.records)
}

pub(crate) fn write_automation_enqueued(
    sink: &mut dyn Sink,
    value: AutomationEnqueued,
) -> Result<(), EncodeError> {
    if value.accepted_records == 0 || value.capacity == 0 || value.occupancy > value.capacity {
        return Err(EncodeError::LimitExceeded);
    }
    sink.check_field_count(schema::automation_enqueued::SPEC.field_count(&[])?)?;
    write_spec!(
        sink,
        schema::automation_enqueued::ACCEPTED_RECORDS,
        &value.accepted_records.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::automation_enqueued::OCCUPANCY,
        &value.occupancy.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::automation_enqueued::CAPACITY,
        &value.capacity.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::automation_enqueued::GENERATION,
        &value.generation.to_le_bytes()
    )
}

pub(crate) fn write_transport_set(
    sink: &mut dyn Sink,
    value: TransportSetRequest,
) -> Result<(), EncodeError> {
    sink.check_field_count(schema::transport_set::SPEC.field_count(&[(
        schema::transport_set::POSITION,
        usize::from(value.position.is_some()),
    )])?)?;
    write_spec!(sink, schema::transport_set::STATE, &[value.state as u8])?;
    if let Some(position) = value.position {
        write_spec!(
            sink,
            schema::transport_set::POSITION,
            &position.0.to_le_bytes()
        )?;
    }
    Ok(())
}

pub(crate) fn write_transport_snapshot(
    sink: &mut dyn Sink,
    value: TransportSnapshot,
) -> Result<(), EncodeError> {
    sink.check_field_count(schema::transport_snapshot::SPEC.field_count(&[])?)?;
    write_spec!(
        sink,
        schema::transport_snapshot::STATE,
        &[value.state as u8]
    )?;
    write_spec!(
        sink,
        schema::transport_snapshot::POSITION,
        &value.position.0.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::transport_snapshot::EFFECTIVE_SAMPLE,
        &value.effective_sample.0.to_le_bytes()
    )
}

pub(crate) fn write_transport_state_event(
    sink: &mut dyn Sink,
    value: TransportStateEvent,
) -> Result<(), EncodeError> {
    sink.check_field_count(schema::transport_state_event::SPEC.field_count(&[(
        schema::transport_state_event::ORIGIN_REQUEST_ID,
        usize::from(value.origin_request_id.is_some()),
    )])?)?;
    write_spec!(
        sink,
        schema::transport_state_event::EVENT_SEQUENCE,
        &value.event_sequence.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::transport_state_event::STATE,
        &[value.state as u8]
    )?;
    write_spec!(
        sink,
        schema::transport_state_event::POSITION,
        &value.position.0.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::transport_state_event::EFFECTIVE_SAMPLE,
        &value.effective_sample.0.to_le_bytes()
    )?;
    if let Some(origin) = value.origin_request_id {
        write_spec!(
            sink,
            schema::transport_state_event::ORIGIN_REQUEST_ID,
            &origin.get().to_le_bytes()
        )?;
    }
    Ok(())
}

pub(crate) fn write_automation_canceled(
    sink: &mut dyn Sink,
    value: AutomationCanceled,
) -> Result<(), EncodeError> {
    if value.canceled_records == 0 {
        return Err(EncodeError::LimitExceeded);
    }
    sink.check_field_count(schema::automation_canceled::SPEC.field_count(&[(
        schema::automation_canceled::EFFECTIVE_SAMPLE,
        usize::from(value.effective_sample.is_some()),
    )])?)?;
    write_spec!(
        sink,
        schema::automation_canceled::EVENT_SEQUENCE,
        &value.event_sequence.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::automation_canceled::ORIGIN_REQUEST_ID,
        &value.origin_request_id.get().to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::automation_canceled::CANCELED_RECORDS,
        &value.canceled_records.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::automation_canceled::REASON,
        &[value.reason as u8]
    )?;
    write_spec!(
        sink,
        schema::automation_canceled::QUEUE_GENERATION,
        &value.queue_generation.to_le_bytes()
    )?;
    if let Some(sample) = value.effective_sample {
        write_spec!(
            sink,
            schema::automation_canceled::EFFECTIVE_SAMPLE,
            &sample.0.to_le_bytes()
        )?;
    }
    Ok(())
}

pub(crate) fn write_meter_batch(
    sink: &mut dyn Sink,
    value: MeterBatch<'_>,
) -> Result<(), EncodeError> {
    validate_meter_records(value.records)?;
    sink.check_field_count(schema::meter_batch::SPEC.field_count(&[])?)?;
    let count = u16::try_from(value.records.len()).map_err(|_| EncodeError::LimitExceeded)?;
    let length = value
        .records
        .len()
        .checked_mul(16)
        .ok_or(EncodeError::LimitExceeded)?;
    write_spec!(
        sink,
        schema::meter_batch::OBSERVED_SAMPLE,
        &value.observed_sample.0.to_le_bytes()
    )?;
    write_spec!(sink, schema::meter_batch::COUNT, &count.to_le_bytes())?;
    write_spec!(
        sink,
        schema::meter_batch::RECORD_BYTES,
        &16_u16.to_le_bytes()
    )?;
    sink.stream_field_spec(schema::meter_batch::RECORDS, length, &mut |sink| {
        for record in value.records {
            let mut bytes = [0_u8; 16];
            encode_meter_record(record, &mut bytes);
            sink.raw(&bytes)?;
        }
        Ok(())
    })
}

pub(crate) fn write_telemetry_configuration(
    sink: &mut dyn Sink,
    value: &TelemetryConfiguration,
) -> Result<(), EncodeError> {
    validate_telemetry_configuration(value).map_err(|_| EncodeError::LimitExceeded)?;
    sink.check_field_count(schema::telemetry_configuration::SPEC.field_count(&[])?)?;
    write_u32s(
        sink,
        schema::telemetry_configuration::METER_HANDLES,
        &value.meter_handles,
    )?;
    write_spec!(
        sink,
        schema::telemetry_configuration::METER_PERIOD_BLOCKS,
        &value.meter_period_blocks.to_le_bytes()
    )?;
    write_counter_ids(
        sink,
        schema::telemetry_configuration::COUNTER_IDS,
        &value.counter_ids,
    )?;
    write_spec!(
        sink,
        schema::telemetry_configuration::COUNTER_PERIOD_BLOCKS,
        &value.counter_period_blocks.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::telemetry_configuration::DIAGNOSTICS_ENABLED,
        &[u8::from(value.diagnostics_enabled)]
    )?;
    write_spec!(
        sink,
        schema::telemetry_configuration::MINIMUM_DIAGNOSTIC_SEVERITY,
        &[value.minimum_diagnostic_severity as u8]
    )
}

pub(crate) fn write_counters_request(
    sink: &mut dyn Sink,
    value: &CountersRequest,
) -> Result<(), EncodeError> {
    validate_counters_request(value).map_err(|_| EncodeError::LimitExceeded)?;
    sink.check_field_count(
        schema::counters_request::SPEC
            .field_count(&[(schema::counters_request::IDS, usize::from(!value.all))])?,
    )?;
    write_spec!(sink, schema::counters_request::ALL, &[u8::from(value.all)])?;
    if !value.all {
        write_u32s(sink, schema::counters_request::IDS, &value.ids)?;
    }
    Ok(())
}

pub(crate) fn write_counter_snapshot(
    sink: &mut dyn Sink,
    value: CounterSnapshotRef<'_>,
) -> Result<(), EncodeError> {
    validate_counter_snapshot_ref(value).map_err(|_| EncodeError::LimitExceeded)?;
    sink.check_field_count(
        schema::counter_snapshot::SPEC
            .field_count(&[(schema::counter_snapshot::VALUE, value.values.len())])?,
    )?;
    write_spec!(
        sink,
        schema::counter_snapshot::OBSERVED_SAMPLE,
        &value.observed_sample.0.to_le_bytes()
    )?;
    for counter in value.values {
        sink.nested_spec(schema::counter_snapshot::VALUE, &mut |sink| {
            sink.message_header(schema::counter_value::SPEC.field_count(&[])?)?;
            write_spec!(
                sink,
                schema::counter_value::ID,
                &(counter.id as u32).to_le_bytes()
            )?;
            write_spec!(
                sink,
                schema::counter_value::VALUE,
                &counter.value.to_le_bytes()
            )
        })?;
    }
    Ok(())
}

pub(crate) fn write_diagnostics_request(
    sink: &mut dyn Sink,
    value: DiagnosticsRequest,
) -> Result<(), EncodeError> {
    if value.limit == 0 || value.limit > 256 {
        return Err(EncodeError::LimitExceeded);
    }
    sink.check_field_count(schema::diagnostics_request::SPEC.field_count(&[])?)?;
    write_spec!(
        sink,
        schema::diagnostics_request::AFTER_SEQUENCE,
        &value.after_sequence.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::diagnostics_request::LIMIT,
        &value.limit.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::diagnostics_request::MINIMUM_SEVERITY,
        &[value.minimum_severity as u8]
    )
}

pub(crate) fn write_non_ok(
    codec: &ProtocolCodec,
    sink: &mut dyn Sink,
    value: &NonOkResponse,
) -> Result<(), EncodeError> {
    check_non_ok_nesting(codec, value)?;
    let count = schema::non_ok::SPEC.field_count(&[
        (schema::non_ok::DIAGNOSTIC, value.diagnostics.len()),
        (
            schema::non_ok::BACKPRESSURE,
            usize::from(value.backpressure.is_some()),
        ),
    ])?;
    sink.check_field_count(count)?;
    for diagnostic in &value.diagnostics {
        write_diagnostic_field(codec, sink, schema::non_ok::DIAGNOSTIC, diagnostic)?;
    }
    write_spec!(
        sink,
        schema::non_ok::OMITTED_DIAGNOSTICS,
        &value.omitted_diagnostics.to_le_bytes()
    )?;
    if let Some(backpressure) = value.backpressure {
        write_backpressure_field(sink, schema::non_ok::BACKPRESSURE, backpressure)?;
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
pub(crate) fn write_metadata_page(
    codec: &ProtocolCodec,
    sink: &mut dyn Sink,
    value: &ParameterMetadataPage,
) -> Result<(), EncodeError> {
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
    let count = schema::metadata_page::SPEC
        .field_count(&[(schema::metadata_page::DESCRIPTOR, value.descriptors.len())])?;
    sink.check_field_count(count)?;
    write_spec!(
        sink,
        schema::metadata_page::LAST_HANDLE,
        &value.last_handle.to_le_bytes()
    )?;
    write_spec!(sink, schema::metadata_page::EOF, &[u8::from(value.eof)])?;
    for descriptor in &value.descriptors {
        write_descriptor(codec, sink, schema::metadata_page::DESCRIPTOR, descriptor)?;
    }
    Ok(())
}
fn write_descriptor(
    codec: &ProtocolCodec,
    sink: &mut dyn Sink,
    spec: schema::FieldSpec,
    value: &ParameterDescriptor,
) -> Result<(), EncodeError> {
    if !descriptor_is_valid(codec.limits(), value) {
        return Err(EncodeError::LimitExceeded);
    }
    let fields = descriptor::SPEC.field_count(&[
        (descriptor::MINIMUM, usize::from(value.minimum.is_some())),
        (descriptor::MAXIMUM, usize::from(value.maximum.is_some())),
        (
            descriptor::DISPLAY_NAME,
            usize::from(value.display_name.is_some()),
        ),
        (
            descriptor::DISPLAY_UNIT,
            usize::from(value.display_unit.is_some()),
        ),
        (descriptor::ENUM_CHOICE, value.enum_choices.len()),
    ])?;
    sink.nested_spec(spec, &mut |sink| {
        sink.message_header(fields)?;
        write_spec!(sink, descriptor::HANDLE, &value.handle.to_le_bytes())?;
        write_spec!(sink, descriptor::TRACK_ID, value.track_id.as_bytes())?;
        write_spec!(
            sink,
            descriptor::RACK,
            &[schema::parameter_rack_wire(value.rack)]
        )?;
        write_spec!(sink, descriptor::EFFECT_ID, value.effect_id.as_bytes())?;
        write_spec!(
            sink,
            descriptor::PARAMETER_ID,
            &value.parameter_id.to_le_bytes()
        )?;
        write_spec!(
            sink,
            descriptor::CHANNEL,
            &[schema::parameter_channel_wire(value.channel)]
        )?;
        write_spec!(sink, descriptor::VALUE_KIND, &[value.value_kind as u8])?;
        write_spec!(
            sink,
            descriptor::UNIT,
            &[schema::parameter_unit_wire(value.unit)]
        )?;
        write_spec!(sink, descriptor::DOMAIN, &[value.domain as u8])?;
        if let Some(v) = value.minimum {
            write_spec!(sink, descriptor::MINIMUM, &v.to_le_bytes())?;
        }
        if let Some(v) = value.maximum {
            write_spec!(sink, descriptor::MAXIMUM, &v.to_le_bytes())?;
        }
        write_spec!(sink, descriptor::DEFAULT, &value.default.to_le_bytes())?;
        write_spec!(sink, descriptor::MAPPING, &[value.mapping as u8])?;
        write_spec!(
            sink,
            descriptor::AUTOMATION_RATE,
            &[value.automation_rate as u8]
        )?;
        write_spec!(
            sink,
            descriptor::SMOOTHING_SAMPLES,
            &value.smoothing_samples.to_le_bytes()
        )?;
        write_spec!(sink, descriptor::FLAGS, &value.flags.to_le_bytes())?;
        if let Some(v) = &value.display_name {
            write_spec!(sink, descriptor::DISPLAY_NAME, v.as_bytes())?;
        }
        if let Some(v) = &value.display_unit {
            write_spec!(sink, descriptor::DISPLAY_UNIT, v.as_bytes())?;
        }
        for choice in &value.enum_choices {
            write_enum_choice(codec, sink, choice)?;
        }
        Ok(())
    })
}

fn write_enum_choice(
    codec: &ProtocolCodec,
    sink: &mut dyn Sink,
    choice: &EnumChoice,
) -> Result<(), EncodeError> {
    if !choice.value.is_finite() {
        return Err(EncodeError::LimitExceeded);
    }
    check_string(codec, &choice.label)?;
    let fields = enum_choice::SPEC.field_count(&[])?;
    sink.nested_spec(descriptor::ENUM_CHOICE, &mut |sink| {
        sink.message_header(fields)?;
        write_spec!(sink, enum_choice::VALUE, &choice.value.to_le_bytes())?;
        write_spec!(sink, enum_choice::LABEL, choice.label.as_bytes())
    })
}
fn decode_metadata_page(
    codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<ParameterMetadataPage, DecodeError> {
    let message = message.schema_spec(&schema::metadata_page::SPEC)?;
    let descriptors = values_spec!(message, schema::metadata_page::DESCRIPTOR)?
        .map(|v| decode_descriptor(codec, Message::nested_at_depth(v, codec.limits(), 1)?))
        .collect::<Result<Vec<_>, _>>()?;
    let page = ParameterMetadataPage {
        last_handle: read_u32(one_spec!(message, schema::metadata_page::LAST_HANDLE)?)?,
        eof: read_bool(one_spec!(message, schema::metadata_page::EOF)?)?,
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
    let message = message.schema_spec(&descriptor::SPEC)?;
    let choices = values_spec!(message, descriptor::ENUM_CHOICE)?
        .map(|v| decode_choice(codec, Message::nested_at_depth(v, codec.limits(), 2)?))
        .collect::<Result<Vec<_>, _>>()?;
    let value = ParameterDescriptor {
        handle: read_u32(one_spec!(message, descriptor::HANDLE)?)?,
        track_id: read_string(codec, one_spec!(message, descriptor::TRACK_ID)?)?.to_owned(),
        rack: schema::parameter_rack_from_wire(read_u8(one_spec!(message, descriptor::RACK)?)?)?,
        effect_id: read_string(codec, one_spec!(message, descriptor::EFFECT_ID)?)?.to_owned(),
        parameter_id: read_u32(one_spec!(message, descriptor::PARAMETER_ID)?)?,
        channel: schema::parameter_channel_from_wire(read_u8(one_spec!(
            message,
            descriptor::CHANNEL
        )?)?)?,
        value_kind: parse_value_kind(read_u8(one_spec!(message, descriptor::VALUE_KIND)?)?)?,
        unit: schema::parameter_unit_from_wire(read_u8(one_spec!(message, descriptor::UNIT)?)?)?,
        domain: parse_domain(read_u8(one_spec!(message, descriptor::DOMAIN)?)?)?,
        minimum: optional_spec!(message, descriptor::MINIMUM)?
            .map(read_f32)
            .transpose()?,
        maximum: optional_spec!(message, descriptor::MAXIMUM)?
            .map(read_f32)
            .transpose()?,
        default: read_f32(one_spec!(message, descriptor::DEFAULT)?)?,
        mapping: parse_mapping(read_u8(one_spec!(message, descriptor::MAPPING)?)?)?,
        automation_rate: parse_rate(read_u8(one_spec!(message, descriptor::AUTOMATION_RATE)?)?)?,
        smoothing_samples: read_u32(one_spec!(message, descriptor::SMOOTHING_SAMPLES)?)?,
        flags: read_u32(one_spec!(message, descriptor::FLAGS)?)?,
        display_name: optional_spec!(message, descriptor::DISPLAY_NAME)?
            .map(|v| read_string(codec, v).map(str::to_owned))
            .transpose()?,
        display_unit: optional_spec!(message, descriptor::DISPLAY_UNIT)?
            .map(|v| read_string(codec, v).map(str::to_owned))
            .transpose()?,
        enum_choices: choices,
    };
    if !descriptor_is_valid(codec.limits(), &value) {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(value)
}
fn decode_choice(codec: &ProtocolCodec, message: Message<'_>) -> Result<EnumChoice, DecodeError> {
    let message = message.schema_spec(&enum_choice::SPEC)?;
    let value = read_f32(one_spec!(message, enum_choice::VALUE)?)?;
    if !value.is_finite() {
        return Err(DecodeError::InvalidTlv);
    };
    Ok(EnumChoice {
        value,
        label: read_string(codec, one_spec!(message, enum_choice::LABEL)?)?.to_owned(),
    })
}
fn descriptor_is_valid(limits: crate::ProtocolLimits, value: &ParameterDescriptor) -> bool {
    if value.handle == 0 || value.flags & !7 != 0 || !value.default.is_finite() {
        return false;
    }
    if value.track_id.len() > limits.max_string_bytes
        || value.effect_id.len() > limits.max_string_bytes
        || value
            .display_name
            .as_ref()
            .is_some_and(|text| text.len() > limits.max_string_bytes)
        || value
            .display_unit
            .as_ref()
            .is_some_and(|text| text.len() > limits.max_string_bytes)
        || !valid_stable_id(&value.track_id)
        || !valid_stable_id(&value.effect_id)
    {
        return false;
    }
    match value.domain {
        ParameterDomain::Continuous => match (value.minimum, value.maximum) {
            (Some(min), Some(max))
                if min.is_finite()
                    && max.is_finite()
                    && min <= value.default
                    && value.default <= max
                    && value.enum_choices.is_empty() => {}
            _ => return false,
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
        _ => return false,
    }
    true
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
pub(crate) fn write_state_page(
    sink: &mut dyn Sink,
    value: &ParameterStatePage,
) -> Result<(), EncodeError> {
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
    sink.check_field_count(schema::state_page::SPEC.field_count(&[])?)?;
    write_spec!(
        sink,
        schema::state_page::OBSERVED_SAMPLE,
        &value.observed_sample.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::state_page::COUNT,
        &u16::try_from(value.records.len())
            .map_err(|_| EncodeError::LimitExceeded)?
            .to_le_bytes(),
    )?;
    write_spec!(
        sink,
        schema::state_page::RECORD_BYTES,
        &16_u16.to_le_bytes()
    )?;
    sink.stream_field_spec(schema::state_page::RECORDS, bytes, &mut |sink| {
        for record in &value.records {
            sink.raw(&record.handle.to_le_bytes())?;
            sink.raw(&record.flags.to_le_bytes())?;
            sink.raw(&record.value.to_le_bytes())?;
            sink.raw(&0_u32.to_le_bytes())?;
        }
        Ok(())
    })
}
fn decode_state_page(
    _codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<ParameterStatePage, DecodeError> {
    let message = message.schema_spec(&schema::state_page::SPEC)?;
    let count = read_u16(one_spec!(message, schema::state_page::COUNT)?)? as usize;
    if count > 256 || read_u16(one_spec!(message, schema::state_page::RECORD_BYTES)?)? != 16 {
        return Err(DecodeError::InvalidTlv);
    }
    let bytes = one_spec!(message, schema::state_page::RECORDS)?;
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
        observed_sample: read_u64(one_spec!(message, schema::state_page::OBSERVED_SAMPLE)?)?,
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

pub(crate) fn write_capabilities(
    sink: &mut dyn Sink,
    value: &Capabilities<'_>,
) -> Result<(), EncodeError> {
    check_capabilities(value)?;
    sink.check_field_count(schema::capabilities::SPEC.field_count(&[])?)?;
    write_spec!(
        sink,
        schema::capabilities::MINIMUM_VERSION_MAJOR,
        &value.minimum_version.major.to_le_bytes(),
    )?;
    write_spec!(
        sink,
        schema::capabilities::MINIMUM_VERSION_MINOR,
        &value.minimum_version.minor.to_le_bytes(),
    )?;
    write_spec!(
        sink,
        schema::capabilities::MAXIMUM_VERSION_MAJOR,
        &value.maximum_version.major.to_le_bytes(),
    )?;
    write_spec!(
        sink,
        schema::capabilities::MAXIMUM_VERSION_MINOR,
        &value.maximum_version.minor.to_le_bytes(),
    )?;
    write_spec!(
        sink,
        schema::capabilities::MAXIMUM_FRAME_BYTES,
        &value.maximum_frame_bytes.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::capabilities::MAXIMUM_TLVS,
        &value.maximum_tlvs.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::capabilities::MAXIMUM_STRING_BYTES,
        &value.maximum_string_bytes.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::capabilities::MAXIMUM_NESTING,
        &[value.maximum_nesting]
    )?;
    write_spec!(
        sink,
        schema::capabilities::MAXIMUM_AUTOMATION_RECORDS,
        &value.maximum_automation_records.to_le_bytes(),
    )?;
    for (spec, field) in [
        (
            schema::capabilities::CONTROL_COMMAND_SLOTS,
            value.control_command_slots,
        ),
        (
            schema::capabilities::CONTROL_COMMAND_BYTES,
            value.control_command_bytes,
        ),
        (
            schema::capabilities::AUTOMATION_BATCH_SLOTS,
            value.automation_batch_slots,
        ),
        (
            schema::capabilities::RELIABLE_RESPONSE_SLOTS,
            value.reliable_response_slots,
        ),
        (
            schema::capabilities::RELIABLE_EVENT_SLOTS,
            value.reliable_event_slots,
        ),
        (schema::capabilities::TELEMETRY_SLOTS, value.telemetry_slots),
        (schema::capabilities::REPLAY_ENTRIES, value.replay_entries),
        (schema::capabilities::REPLAY_BYTES, value.replay_bytes),
        (
            schema::capabilities::MAXIMUM_CACHED_RESPONSE_BYTES,
            value.maximum_cached_response_bytes,
        ),
        (
            schema::capabilities::PER_BLOCK_AUTOMATION_DENSITY,
            value.per_block_automation_density,
        ),
        (
            schema::capabilities::ADMISSION_QUANTUM_FRAMES,
            value.admission_quantum_frames,
        ),
    ] {
        write_spec!(sink, spec, &field.to_le_bytes())?;
    }
    write_spec!(
        sink,
        schema::capabilities::MAXIMUM_PARAMETER_PAGE_ITEMS,
        &value.maximum_parameter_page_items.to_le_bytes(),
    )?;
    write_spec!(
        sink,
        schema::capabilities::MAXIMUM_DIAGNOSTIC_PAGE_ITEMS,
        &value.maximum_diagnostic_page_items.to_le_bytes(),
    )?;
    write_spec!(
        sink,
        schema::capabilities::MAXIMUM_TELEMETRY_HANDLES,
        &value.maximum_telemetry_handles.to_le_bytes(),
    )?;
    write_spec!(
        sink,
        schema::capabilities::MAXIMUM_TRANSACTION_EDITS,
        &value.maximum_transaction_edits.to_le_bytes(),
    )?;
    write_packed_u16(
        sink,
        schema::capabilities::SUPPORTED_COMMANDS,
        value.supported_commands,
    )?;
    write_packed_u16(
        sink,
        schema::capabilities::SUPPORTED_EVENTS,
        value.supported_events,
    )?;
    write_spec!(
        sink,
        schema::capabilities::FLAGS,
        &value.flags.0.to_le_bytes()
    )?;
    Ok(())
}

fn write_packed_u16(
    sink: &mut dyn Sink,
    spec: schema::FieldSpec,
    values: &[u16],
) -> Result<(), EncodeError> {
    let value_len = values
        .len()
        .checked_mul(2)
        .ok_or(EncodeError::LimitExceeded)?;
    sink.stream_field_spec(spec, value_len, &mut |sink| {
        for value in values {
            sink.raw(&value.to_le_bytes())?;
        }
        Ok(())
    })
}

fn decode_capabilities<'a>(
    codec: &ProtocolCodec,
    message: Message<'a>,
) -> Result<DecodedCapabilities<'a>, DecodeError> {
    let message = message.schema_spec(&schema::capabilities::SPEC)?;
    let value = DecodedCapabilities {
        minimum_version: crate::ProtocolVersion {
            major: read_u16(one_spec!(
                message,
                schema::capabilities::MINIMUM_VERSION_MAJOR
            )?)?,
            minor: read_u16(one_spec!(
                message,
                schema::capabilities::MINIMUM_VERSION_MINOR
            )?)?,
        },
        maximum_version: crate::ProtocolVersion {
            major: read_u16(one_spec!(
                message,
                schema::capabilities::MAXIMUM_VERSION_MAJOR
            )?)?,
            minor: read_u16(one_spec!(
                message,
                schema::capabilities::MAXIMUM_VERSION_MINOR
            )?)?,
        },
        maximum_frame_bytes: read_u64(one_spec!(
            message,
            schema::capabilities::MAXIMUM_FRAME_BYTES
        )?)?,
        maximum_tlvs: read_u32(one_spec!(message, schema::capabilities::MAXIMUM_TLVS)?)?,
        maximum_string_bytes: read_u64(one_spec!(
            message,
            schema::capabilities::MAXIMUM_STRING_BYTES
        )?)?,
        maximum_nesting: read_u8(one_spec!(message, schema::capabilities::MAXIMUM_NESTING)?)?,
        maximum_automation_records: read_u16(one_spec!(
            message,
            schema::capabilities::MAXIMUM_AUTOMATION_RECORDS
        )?)?,
        control_command_slots: read_u64(one_spec!(
            message,
            schema::capabilities::CONTROL_COMMAND_SLOTS
        )?)?,
        control_command_bytes: read_u64(one_spec!(
            message,
            schema::capabilities::CONTROL_COMMAND_BYTES
        )?)?,
        automation_batch_slots: read_u64(one_spec!(
            message,
            schema::capabilities::AUTOMATION_BATCH_SLOTS
        )?)?,
        reliable_response_slots: read_u64(one_spec!(
            message,
            schema::capabilities::RELIABLE_RESPONSE_SLOTS
        )?)?,
        reliable_event_slots: read_u64(one_spec!(
            message,
            schema::capabilities::RELIABLE_EVENT_SLOTS
        )?)?,
        telemetry_slots: read_u64(one_spec!(message, schema::capabilities::TELEMETRY_SLOTS)?)?,
        replay_entries: read_u64(one_spec!(message, schema::capabilities::REPLAY_ENTRIES)?)?,
        replay_bytes: read_u64(one_spec!(message, schema::capabilities::REPLAY_BYTES)?)?,
        maximum_cached_response_bytes: read_u64(one_spec!(
            message,
            schema::capabilities::MAXIMUM_CACHED_RESPONSE_BYTES
        )?)?,
        per_block_automation_density: read_u64(one_spec!(
            message,
            schema::capabilities::PER_BLOCK_AUTOMATION_DENSITY
        )?)?,
        admission_quantum_frames: read_u64(one_spec!(
            message,
            schema::capabilities::ADMISSION_QUANTUM_FRAMES
        )?)?,
        maximum_parameter_page_items: read_u16(one_spec!(
            message,
            schema::capabilities::MAXIMUM_PARAMETER_PAGE_ITEMS
        )?)?,
        maximum_diagnostic_page_items: read_u16(one_spec!(
            message,
            schema::capabilities::MAXIMUM_DIAGNOSTIC_PAGE_ITEMS
        )?)?,
        maximum_telemetry_handles: read_u16(one_spec!(
            message,
            schema::capabilities::MAXIMUM_TELEMETRY_HANDLES
        )?)?,
        maximum_transaction_edits: read_u32(one_spec!(
            message,
            schema::capabilities::MAXIMUM_TRANSACTION_EDITS
        )?)?,
        supported_commands: one_spec!(message, schema::capabilities::SUPPORTED_COMMANDS)?,
        supported_events: one_spec!(message, schema::capabilities::SUPPORTED_EVENTS)?,
        flags: CapabilityFlags(read_u64(one_spec!(message, schema::capabilities::FLAGS)?)?),
    };
    if !value.supported_commands.len().is_multiple_of(2)
        || !value.supported_events.len().is_multiple_of(2)
    {
        return Err(DecodeError::InvalidValueLength);
    }
    check_capabilities_invariants(CapabilityInvariantView::from(&value))?;
    let _ = codec;
    Ok(value)
}

fn check_capabilities(value: &Capabilities<'_>) -> Result<(), EncodeError> {
    check_capabilities_invariants(CapabilityInvariantView::from(value))
        .map_err(|_| EncodeError::LimitExceeded)
}

#[derive(Clone, Copy)]
enum IdSource<'a> {
    Native(&'a [u16]),
    LittleEndian(&'a [u8]),
}

impl IdSource<'_> {
    fn len(self) -> Option<usize> {
        match self {
            Self::Native(ids) => Some(ids.len()),
            Self::LittleEndian(bytes) if bytes.len().is_multiple_of(2) => Some(bytes.len() / 2),
            Self::LittleEndian(_) => None,
        }
    }

    fn get(self, index: usize) -> Option<u16> {
        match self {
            Self::Native(ids) => ids.get(index).copied(),
            Self::LittleEndian(bytes) => {
                let offset = index.checked_mul(2)?;
                Some(u16::from_le_bytes([
                    *bytes.get(offset)?,
                    *bytes.get(offset + 1)?,
                ]))
            }
        }
    }

    fn contains(self, wanted: u16) -> bool {
        self.len()
            .is_some_and(|len| (0..len).any(|index| self.get(index) == Some(wanted)))
    }

    fn is_strict_allocated(self, events: bool) -> bool {
        let Some(len) = self.len() else {
            return false;
        };
        let mut prior = None;
        for index in 0..len {
            let Some(id) = self.get(index) else {
                return false;
            };
            if !allocated_id(id, events) || prior.is_some_and(|previous| id <= previous) {
                return false;
            }
            prior = Some(id);
        }
        true
    }
}

#[derive(Clone, Copy)]
struct CapabilityInvariantView<'a> {
    minimum_version: crate::ProtocolVersion,
    maximum_version: crate::ProtocolVersion,
    maximum_frame_bytes: u64,
    maximum_tlvs: u32,
    maximum_nesting: u8,
    maximum_automation_records: u16,
    maximum_parameter_page_items: u16,
    maximum_diagnostic_page_items: u16,
    maximum_transaction_edits: u32,
    supported_commands: IdSource<'a>,
    supported_events: IdSource<'a>,
    flags: CapabilityFlags,
}

impl<'a> From<&Capabilities<'a>> for CapabilityInvariantView<'a> {
    fn from(value: &Capabilities<'a>) -> Self {
        Self {
            minimum_version: value.minimum_version,
            maximum_version: value.maximum_version,
            maximum_frame_bytes: value.maximum_frame_bytes,
            maximum_tlvs: value.maximum_tlvs,
            maximum_nesting: value.maximum_nesting,
            maximum_automation_records: value.maximum_automation_records,
            maximum_parameter_page_items: value.maximum_parameter_page_items,
            maximum_diagnostic_page_items: value.maximum_diagnostic_page_items,
            maximum_transaction_edits: value.maximum_transaction_edits,
            supported_commands: IdSource::Native(value.supported_commands),
            supported_events: IdSource::Native(value.supported_events),
            flags: value.flags,
        }
    }
}

impl<'a> From<&DecodedCapabilities<'a>> for CapabilityInvariantView<'a> {
    fn from(value: &DecodedCapabilities<'a>) -> Self {
        Self {
            minimum_version: value.minimum_version,
            maximum_version: value.maximum_version,
            maximum_frame_bytes: value.maximum_frame_bytes,
            maximum_tlvs: value.maximum_tlvs,
            maximum_nesting: value.maximum_nesting,
            maximum_automation_records: value.maximum_automation_records,
            maximum_parameter_page_items: value.maximum_parameter_page_items,
            maximum_diagnostic_page_items: value.maximum_diagnostic_page_items,
            maximum_transaction_edits: value.maximum_transaction_edits,
            supported_commands: IdSource::LittleEndian(value.supported_commands),
            supported_events: IdSource::LittleEndian(value.supported_events),
            flags: value.flags,
        }
    }
}

fn check_capabilities_invariants(value: CapabilityInvariantView<'_>) -> Result<(), DecodeError> {
    let has_command = |wanted| value.supported_commands.contains(wanted);
    let has_event = |wanted| value.supported_events.contains(wanted);
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
        || !value.supported_commands.is_strict_allocated(false)
        || !value.supported_events.is_strict_allocated(true)
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
fn allocated_id(id: u16, event: bool) -> bool {
    if event {
        matches!(id, 0x8001 | 0x8002 | 0x8010 | 0x8020 | 0x8021 | 0x8030)
    } else {
        matches!(id, 1..=11)
    }
}

fn write_diagnostic_field(
    codec: &ProtocolCodec,
    sink: &mut dyn Sink,
    spec: schema::FieldSpec,
    value: &Diagnostic,
) -> Result<(), EncodeError> {
    sink.nested_spec(spec, &mut |sink| {
        write_diagnostic_message(codec, sink, value)
    })
}

fn write_diagnostic_message(
    codec: &ProtocolCodec,
    sink: &mut dyn Sink,
    value: &Diagnostic,
) -> Result<(), EncodeError> {
    check_diagnostic(codec, value)?;
    let count = schema::diagnostic::SPEC.field_count(&[
        (schema::diagnostic::PATH, value.path.len()),
        (
            schema::diagnostic::DETAIL,
            usize::from(value.detail.is_some()),
        ),
        (
            schema::diagnostic::OPERATION_INDEX,
            usize::from(value.operation_index.is_some()),
        ),
        (
            schema::diagnostic::SAMPLE_TIME,
            usize::from(value.sample_time.is_some()),
        ),
        (
            schema::diagnostic::PROVIDER_SEQUENCE,
            usize::from(value.provider_sequence.is_some()),
        ),
    ])?;
    sink.message_header(count)?;
    write_spec!(sink, schema::diagnostic::CODE, value.code.as_bytes())?;
    write_spec!(sink, schema::diagnostic::SEVERITY, &[value.severity as u8])?;
    for segment in &value.path {
        write_path_segment_field(codec, sink, schema::diagnostic::PATH, segment)?;
    }
    if let Some(detail) = &value.detail {
        write_spec!(sink, schema::diagnostic::DETAIL, detail.as_bytes())?;
    }
    if let Some(index) = value.operation_index {
        write_spec!(
            sink,
            schema::diagnostic::OPERATION_INDEX,
            &index.to_le_bytes()
        )?;
    }
    if let Some(sample) = value.sample_time {
        write_spec!(sink, schema::diagnostic::SAMPLE_TIME, &sample.to_le_bytes())?;
    }
    if let Some(sequence) = value.provider_sequence {
        write_spec!(
            sink,
            schema::diagnostic::PROVIDER_SEQUENCE,
            &sequence.to_le_bytes()
        )?;
    }
    Ok(())
}

fn write_path_segment_field(
    codec: &ProtocolCodec,
    sink: &mut dyn Sink,
    spec: schema::FieldSpec,
    value: &PathSegment,
) -> Result<(), EncodeError> {
    sink.nested_spec(spec, &mut |sink| {
        write_path_segment_message(codec, sink, value)
    })
}

fn write_path_segment_message(
    codec: &ProtocolCodec,
    sink: &mut dyn Sink,
    value: &PathSegment,
) -> Result<(), EncodeError> {
    check_path_segment(codec, value)?;
    let variant = match value {
        PathSegment::Field(_) => schema::path_segment::FIELD,
        PathSegment::Index(_) => schema::path_segment::INDEX,
        PathSegment::StableId(_) => schema::path_segment::STABLE_ID,
    };
    sink.message_header(schema::path_segment::SPEC.field_count(&[(variant, 1)])?)?;
    write_spec!(sink, schema::path_segment::TAG, &[path_segment_tag(value)])?;
    match value {
        PathSegment::Field(field) => {
            write_spec!(sink, schema::path_segment::FIELD, field.as_bytes())?
        }
        PathSegment::Index(index) => {
            write_spec!(sink, schema::path_segment::INDEX, &index.to_le_bytes())?
        }
        PathSegment::StableId(id) => {
            write_spec!(sink, schema::path_segment::STABLE_ID, id.as_bytes())?
        }
    }
    Ok(())
}

fn write_backpressure_field(
    sink: &mut dyn Sink,
    spec: schema::FieldSpec,
    value: Backpressure,
) -> Result<(), EncodeError> {
    sink.nested_spec(spec, &mut |sink| write_backpressure_message(sink, value))
}

fn write_backpressure_message(sink: &mut dyn Sink, value: Backpressure) -> Result<(), EncodeError> {
    check_backpressure(value)?;
    let count = schema::backpressure::SPEC.field_count(&[
        (
            schema::backpressure::GENERATION,
            usize::from(value.generation.is_some()),
        ),
        (
            schema::backpressure::RETRY_BOUNDARY,
            usize::from(value.retry_boundary.is_some()),
        ),
        (
            schema::backpressure::REQUESTED_BYTES,
            usize::from(value.requested_bytes.is_some()),
        ),
        (
            schema::backpressure::AVAILABLE_BYTES,
            usize::from(value.available_bytes.is_some()),
        ),
    ])?;
    sink.message_header(count)?;
    write_spec!(
        sink,
        schema::backpressure::QUEUE_KIND,
        &[value.queue_kind as u8]
    )?;
    write_spec!(
        sink,
        schema::backpressure::CAPACITY,
        &value.capacity.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::backpressure::OCCUPANCY,
        &value.occupancy.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::backpressure::REQUESTED_ITEMS,
        &value.requested_items.to_le_bytes()
    )?;
    if let Some(generation) = value.generation {
        write_spec!(
            sink,
            schema::backpressure::GENERATION,
            &generation.to_le_bytes()
        )?;
    }
    if let Some(boundary) = value.retry_boundary {
        write_spec!(
            sink,
            schema::backpressure::RETRY_BOUNDARY,
            &boundary.to_le_bytes()
        )?;
    }
    if let Some(bytes) = value.requested_bytes {
        write_spec!(
            sink,
            schema::backpressure::REQUESTED_BYTES,
            &bytes.to_le_bytes()
        )?;
    }
    if let Some(bytes) = value.available_bytes {
        write_spec!(
            sink,
            schema::backpressure::AVAILABLE_BYTES,
            &bytes.to_le_bytes()
        )?;
    }
    Ok(())
}

pub(crate) fn write_diagnostic_event(
    codec: &ProtocolCodec,
    sink: &mut dyn Sink,
    diagnostic: &Diagnostic,
) -> Result<(), EncodeError> {
    if diagnostic.provider_sequence.is_none() {
        return Err(EncodeError::LimitExceeded);
    }
    let required_nesting = if diagnostic.path.is_empty() { 1 } else { 2 };
    if codec.limits().max_nesting < required_nesting {
        return Err(EncodeError::LimitExceeded);
    }
    sink.check_field_count(schema::diagnostic_event::SPEC.field_count(&[])?)?;
    write_diagnostic_field(
        codec,
        sink,
        schema::diagnostic_event::DIAGNOSTIC,
        diagnostic,
    )
}

pub(crate) fn write_diagnostics_page(
    codec: &ProtocolCodec,
    sink: &mut dyn Sink,
    value: &DiagnosticsPage,
) -> Result<(), EncodeError> {
    validate_diagnostics_page(value)?;
    let required_nesting = if value
        .diagnostics
        .iter()
        .any(|diagnostic| !diagnostic.path.is_empty())
    {
        2
    } else {
        1
    };
    if codec.limits().max_nesting < required_nesting {
        return Err(EncodeError::LimitExceeded);
    }
    let count = schema::diagnostics_page::SPEC.field_count(&[(
        schema::diagnostics_page::DIAGNOSTIC,
        value.diagnostics.len(),
    )])?;
    sink.check_field_count(count)?;
    write_spec!(
        sink,
        schema::diagnostics_page::LAST_SEQUENCE,
        &value.last_sequence.to_le_bytes()
    )?;
    write_spec!(sink, schema::diagnostics_page::EOF, &[u8::from(value.eof)])?;
    for diagnostic in &value.diagnostics {
        write_diagnostic_field(
            codec,
            sink,
            schema::diagnostics_page::DIAGNOSTIC,
            diagnostic,
        )?;
    }
    Ok(())
}

fn check_path_segment(codec: &ProtocolCodec, value: &PathSegment) -> Result<(), EncodeError> {
    match value {
        PathSegment::Field(field) => check_string(codec, field),
        PathSegment::Index(_) => Ok(()),
        PathSegment::StableId(id) => {
            check_string(codec, id)?;
            if !valid_stable_id(id) {
                return Err(EncodeError::LimitExceeded);
            }
            Ok(())
        }
    }
}

fn check_diagnostic(codec: &ProtocolCodec, value: &Diagnostic) -> Result<(), EncodeError> {
    check_string(codec, &value.code)?;
    if !valid_dotted_code(&value.code) {
        return Err(EncodeError::LimitExceeded);
    }
    if let Some(detail) = &value.detail {
        check_string(codec, detail)?;
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
    let message = message.schema_spec(&schema::non_ok::SPEC)?;
    let diagnostics = values_spec!(message, schema::non_ok::DIAGNOSTIC)?
        .map(|value| decode_diagnostic(codec, Message::nested_at_depth(value, codec.limits(), 1)?))
        .collect::<Result<Vec<_>, _>>()?;
    let omitted_diagnostics = read_u32(one_spec!(message, schema::non_ok::OMITTED_DIAGNOSTICS)?)?;
    let backpressure = optional_spec!(message, schema::non_ok::BACKPRESSURE)?
        .map(|value| {
            decode_backpressure(codec, Message::nested_at_depth(value, codec.limits(), 1)?)
        })
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
    let message = message.schema_spec(&schema::diagnostic::SPEC)?;
    let code = read_string(codec, one_spec!(message, schema::diagnostic::CODE)?)?.to_owned();
    if !valid_dotted_code(&code) {
        return Err(DecodeError::InvalidTlv);
    }
    let severity =
        DiagnosticSeverity::decode(read_u8(one_spec!(message, schema::diagnostic::SEVERITY)?)?)?;
    let path = values_spec!(message, schema::diagnostic::PATH)?
        .map(|value| {
            decode_path_segment(codec, Message::nested_at_depth(value, codec.limits(), 2)?)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let detail = optional_spec!(message, schema::diagnostic::DETAIL)?
        .map(|value| read_string(codec, value).map(str::to_owned))
        .transpose()?;
    let operation_index = optional_spec!(message, schema::diagnostic::OPERATION_INDEX)?
        .map(read_u32)
        .transpose()?;
    let sample_time = optional_spec!(message, schema::diagnostic::SAMPLE_TIME)?
        .map(read_u64)
        .transpose()?;
    let provider_sequence = optional_spec!(message, schema::diagnostic::PROVIDER_SEQUENCE)?
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
    let message = message.schema_spec(&schema::path_segment::SPEC)?;
    let tag = read_u8(one_spec!(message, schema::path_segment::TAG)?)?;
    let field = optional_spec!(message, schema::path_segment::FIELD)?;
    let index = optional_spec!(message, schema::path_segment::INDEX)?;
    let stable_id = optional_spec!(message, schema::path_segment::STABLE_ID)?;
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
    let message = message.schema_spec(&schema::backpressure::SPEC)?;
    let queue_kind = BackpressureQueueKind::decode(read_u8(one_spec!(
        message,
        schema::backpressure::QUEUE_KIND
    )?)?)?;
    let value = Backpressure {
        queue_kind,
        capacity: read_u64(one_spec!(message, schema::backpressure::CAPACITY)?)?,
        occupancy: read_u64(one_spec!(message, schema::backpressure::OCCUPANCY)?)?,
        requested_items: read_u16(one_spec!(message, schema::backpressure::REQUESTED_ITEMS)?)?,
        generation: optional_spec!(message, schema::backpressure::GENERATION)?
            .map(read_u64)
            .transpose()?,
        retry_boundary: optional_spec!(message, schema::backpressure::RETRY_BOUNDARY)?
            .map(read_u64)
            .transpose()?,
        requested_bytes: optional_spec!(message, schema::backpressure::REQUESTED_BYTES)?
            .map(read_u64)
            .transpose()?,
        available_bytes: optional_spec!(message, schema::backpressure::AVAILABLE_BYTES)?
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

fn read_bool(value: &[u8]) -> Result<bool, DecodeError> {
    match read_u8(value)? {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(DecodeError::InvalidValueLength),
    }
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

fn checked_sink_len(codec: &ProtocolCodec, sink: &mut dyn Sink) -> Result<usize, EncodeError> {
    sink.finish_message()?;
    let written = sink.written();
    if written > codec.limits().max_frame_bytes {
        return Err(EncodeError::LimitExceeded);
    }
    Ok(written)
}

fn checked_writer_len(sink: &dyn Sink, required: usize) -> Result<usize, EncodeError> {
    sink.finish_message()?;
    if sink.written() != required {
        return Err(EncodeError::LimitExceeded);
    }
    Ok(required)
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
    fn capability_id_views_reject_order_allocation_and_session_family_identically() {
        fn le(ids: &[u16]) -> Vec<u8> {
            ids.iter().flat_map(|id| id.to_le_bytes()).collect()
        }
        fn view<'a>(commands: IdSource<'a>, events: IdSource<'a>) -> CapabilityInvariantView<'a> {
            CapabilityInvariantView {
                minimum_version: crate::ProtocolVersion::V1,
                maximum_version: crate::ProtocolVersion::V1,
                maximum_frame_bytes: 4096,
                maximum_tlvs: 64,
                maximum_nesting: 4,
                maximum_automation_records: 256,
                maximum_parameter_page_items: 256,
                maximum_diagnostic_page_items: 256,
                maximum_transaction_edits: 64,
                supported_commands: commands,
                supported_events: events,
                flags: CapabilityFlags(
                    CapabilityFlags::B1B_BASE.0 | CapabilityFlags::SESSION_EVENT_STREAM.0,
                ),
            }
        }
        fn assert_parity_rejects(commands: &[u16], events: &[u16]) {
            let command_bytes = le(commands);
            let event_bytes = le(events);
            assert_eq!(
                check_capabilities_invariants(view(
                    IdSource::Native(commands),
                    IdSource::Native(events),
                )),
                Err(DecodeError::InvalidTlv)
            );
            assert_eq!(
                check_capabilities_invariants(view(
                    IdSource::LittleEndian(&command_bytes),
                    IdSource::LittleEndian(&event_bytes),
                )),
                Err(DecodeError::InvalidTlv)
            );
        }

        let commands = [1, 2, 3];
        let events = [0x8001, 0x8002];
        let command_bytes = le(&commands);
        let event_bytes = le(&events);
        assert!(
            check_capabilities_invariants(view(
                IdSource::Native(&commands),
                IdSource::Native(&events),
            ))
            .is_ok()
        );
        assert!(
            check_capabilities_invariants(view(
                IdSource::LittleEndian(&command_bytes),
                IdSource::LittleEndian(&event_bytes),
            ))
            .is_ok()
        );

        assert_parity_rejects(&[1, 3, 2], &events);
        assert_parity_rejects(&[1, 2, 12], &events);
        assert_parity_rejects(&commands, &[0x8001]);
        assert!(!IdSource::LittleEndian(&[1]).is_strict_allocated(false));
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
    fn descriptor_invariant_rejects_encode_and_decode_identically() {
        let codec = ProtocolCodec::default();
        let descriptor = b2_descriptor(1, ParameterDomain::Continuous);
        assert!(descriptor_is_valid(codec.limits(), &descriptor));
        let page = ParameterMetadataPage {
            last_handle: 1,
            eof: true,
            descriptors: vec![descriptor.clone()],
        };
        let mut encoded = vec![
            0;
            codec
                .encoded_parameter_metadata_page_len(&page)
                .expect("valid descriptor length")
        ];
        codec
            .encode_parameter_metadata_page(&page, &mut encoded)
            .expect("valid descriptor encode");
        assert_eq!(
            codec
                .decode_parameter_metadata_page(&encoded, 3)
                .expect("valid descriptor decode"),
            page
        );

        let mut invalid = descriptor;
        invalid.default = 2.0;
        assert!(!descriptor_is_valid(codec.limits(), &invalid));
        assert_eq!(
            codec.encoded_parameter_metadata_page_len(&ParameterMetadataPage {
                last_handle: 1,
                eof: true,
                descriptors: vec![invalid],
            }),
            Err(EncodeError::LimitExceeded)
        );

        let default_prefix = [12, 0, 6, 1, 4, 0, 0, 0];
        let offset = encoded
            .windows(default_prefix.len())
            .position(|window| window == default_prefix)
            .expect("descriptor default field")
            + default_prefix.len();
        encoded[offset..offset + 4].copy_from_slice(&2.0_f32.to_le_bytes());
        assert_eq!(
            codec.decode_parameter_metadata_page(&encoded, 3),
            Err(DecodeError::InvalidTlv)
        );
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
