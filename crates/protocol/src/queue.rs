//! Fixed transient automation records and prepared protocol queues.
//!
//! All queue storage is created off render and delegates to issue-003's public bounded SPSC API.
//! The consumer operations below only move fixed `Copy` slots; they do not decode, allocate,
//! compile, publish a plan, or structurally mutate a session.

use core::{
    alloc::Layout,
    fmt,
    num::NonZeroUsize,
    sync::atomic::{AtomicUsize, Ordering},
};
use std::sync::Arc;

use engine::realtime::{
    Consumer, Producer, QueueEmpty, QueueFull, QueueGeneration, SpscError, bounded_spsc,
    bounded_spsc_retained_payload,
};

use crate::{
    AUTOMATION_BATCH_RECORDS, AUTOMATION_RECORD_BYTES, MessageId, RequestId, SampleTime,
    SessionRevision,
};

/// A revision-scoped runtime parameter handle. It is deliberately not a realtime parameter slot.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParameterHandle(pub u32);

/// Transient automation interpolation kinds encoded in a 32-byte record.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum AutomationKind {
    /// Apply a point at one exact sample; start/end and values must agree.
    Point = 1,
    /// Hold the start value over `[start, end)`.
    Step = 2,
    /// Interpolate linearly over `[start, end)`.
    Linear = 3,
    /// Interpolate exponentially over `[start, end)` using strictly positive endpoints.
    Exponential = 4,
}

impl AutomationKind {
    fn parse(value: u8) -> Result<Self, AutomationBatchError> {
        match value {
            1 => Ok(Self::Point),
            2 => Ok(Self::Step),
            3 => Ok(Self::Linear),
            4 => Ok(Self::Exponential),
            _ => Err(AutomationBatchError::InvalidKind),
        }
    }
}

/// A fixed transient automation record. It has an exact manual 32-byte LE encoding.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AutomationRecord {
    /// Point, step, linear, or exponential behavior.
    pub kind: AutomationKind,
    /// Revision-scoped public parameter handle.
    pub handle: ParameterHandle,
    /// Inclusive absolute start sample.
    pub start: SampleTime,
    /// Exclusive absolute end sample, or equal to `start` for a point.
    pub end: SampleTime,
    /// Value at `start`.
    pub start_value: f32,
    /// Value at `end`.
    pub end_value: f32,
}

impl AutomationRecord {
    /// A deterministic valid zero-time point used only to fill unused prepared batch slots.
    pub const EMPTY: Self = Self {
        kind: AutomationKind::Point,
        handle: ParameterHandle(0),
        start: SampleTime(0),
        end: SampleTime(0),
        start_value: 0.0,
        end_value: 0.0,
    };

    /// Decode one exact 32-byte LE record without allocation.
    pub fn decode_le(bytes: &[u8; AUTOMATION_RECORD_BYTES]) -> Result<Self, AutomationBatchError> {
        let kind = AutomationKind::parse(bytes[0])?;
        if bytes[1] != 0 || bytes[2] != 0 || bytes[3] != 0 {
            return Err(AutomationBatchError::NonzeroReserved);
        }
        let record = Self {
            kind,
            handle: ParameterHandle(u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]])),
            start: SampleTime(u64::from_le_bytes([
                bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14],
                bytes[15],
            ])),
            end: SampleTime(u64::from_le_bytes([
                bytes[16], bytes[17], bytes[18], bytes[19], bytes[20], bytes[21], bytes[22],
                bytes[23],
            ])),
            start_value: f32::from_le_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]),
            end_value: f32::from_le_bytes([bytes[28], bytes[29], bytes[30], bytes[31]]),
        };
        record.validate()?;
        Ok(record)
    }

    /// Encode the exact 32-byte LE record after validating its semantics.
    pub fn encode_le(
        &self,
        output: &mut [u8; AUTOMATION_RECORD_BYTES],
    ) -> Result<(), AutomationBatchError> {
        self.validate()?;
        output[0] = self.kind as u8;
        output[1..4].fill(0);
        output[4..8].copy_from_slice(&self.handle.0.to_le_bytes());
        output[8..16].copy_from_slice(&self.start.0.to_le_bytes());
        output[16..24].copy_from_slice(&self.end.0.to_le_bytes());
        output[24..28].copy_from_slice(&self.start_value.to_le_bytes());
        output[28..32].copy_from_slice(&self.end_value.to_le_bytes());
        Ok(())
    }

    /// Verify finite values and frozen point/segment constraints.
    pub fn validate(&self) -> Result<(), AutomationBatchError> {
        if self.handle.0 == 0 {
            return Err(AutomationBatchError::InvalidHandle);
        }
        if !self.start_value.is_finite() || !self.end_value.is_finite() {
            return Err(AutomationBatchError::NonFiniteValue);
        }
        match self.kind {
            AutomationKind::Point
                if self.start != self.end || self.start_value != self.end_value =>
            {
                Err(AutomationBatchError::InvalidPoint)
            }
            AutomationKind::Point => Ok(()),
            AutomationKind::Step | AutomationKind::Linear if self.end <= self.start => {
                Err(AutomationBatchError::InvalidRange)
            }
            AutomationKind::Step | AutomationKind::Linear => Ok(()),
            AutomationKind::Exponential
                if self.end <= self.start
                    || self.start_value == 0.0
                    || self.end_value == 0.0
                    || self.start_value.is_sign_positive() != self.end_value.is_sign_positive() =>
            {
                Err(AutomationBatchError::InvalidExponential)
            }
            AutomationKind::Exponential => Ok(()),
        }
    }
}

/// Why a transient record or batch was rejected before entering a queue.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AutomationBatchError {
    /// A v1 automation enqueue must contain at least one fixed record.
    EmptyBatch,
    /// A transient record referred to the forbidden zero parameter handle.
    InvalidHandle,
    /// Record kind was not one of the four frozen values.
    InvalidKind,
    /// Record flags/reserved bytes were nonzero.
    NonzeroReserved,
    /// A value was NaN or infinity.
    NonFiniteValue,
    /// A point did not use equal sample times and values.
    InvalidPoint,
    /// A step/linear range was not strictly increasing.
    InvalidRange,
    /// An exponential range or strictly-positive endpoint was invalid.
    InvalidExponential,
    /// The declared slot length exceeds 256.
    TooManyRecords,
    /// A record precedes the preceding record's `(start, handle)` key.
    OutOfOrder,
    /// Two records overlap or duplicate a point for one parameter handle.
    Overlap,
    /// A newly submitted batch moved global start time backwards.
    GlobalTimeBackwards,
    /// A record starts before the submission sample time.
    TimeInPast,
    /// A block contains more record starts than the prepared fixed capacity allows.
    DensityExceeded,
}

impl fmt::Display for AutomationBatchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for AutomationBatchError {}

/// A one-slot atomic automation message containing at most 256 prepared records.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AutomationBatchSlot {
    /// Revision under which the public handles are valid.
    pub revision: SessionRevision,
    /// Request/batch identity for cancellation and reliable acknowledgement.
    pub request_id: RequestId,
    /// Number of initialized logical records in `records`.
    pub len: u16,
    /// Fixed storage; only `0..len` is semantically present.
    pub records: [AutomationRecord; AUTOMATION_BATCH_RECORDS],
}

impl AutomationBatchSlot {
    /// Copy a validated input batch into an exact fixed 256-record queue slot.
    pub fn new(
        revision: SessionRevision,
        request_id: RequestId,
        records: &[AutomationRecord],
    ) -> Result<Self, AutomationBatchError> {
        let length =
            u16::try_from(records.len()).map_err(|_| AutomationBatchError::TooManyRecords)?;
        if records.is_empty() {
            return Err(AutomationBatchError::EmptyBatch);
        }
        if records.len() > AUTOMATION_BATCH_RECORDS {
            return Err(AutomationBatchError::TooManyRecords);
        }
        let mut result = Self {
            revision,
            request_id,
            len: length,
            records: [AutomationRecord::EMPTY; AUTOMATION_BATCH_RECORDS],
        };
        result.records[..records.len()].copy_from_slice(records);
        result.validate_records()?;
        Ok(result)
    }

    /// Borrow only records declared by this slot's fixed length.
    #[must_use]
    pub fn as_slice(&self) -> &[AutomationRecord] {
        &self.records[..usize::from(self.len)]
    }

    /// Revalidate semantic/order/overlap invariants without allocation.
    pub fn validate_records(&self) -> Result<(), AutomationBatchError> {
        if usize::from(self.len) > AUTOMATION_BATCH_RECORDS {
            return Err(AutomationBatchError::TooManyRecords);
        }
        let records = self.as_slice();
        if records.is_empty() {
            return Err(AutomationBatchError::EmptyBatch);
        }
        for (index, record) in records.iter().enumerate() {
            record.validate()?;
            if index != 0 {
                let previous = records[index - 1];
                if (record.start, record.handle) < (previous.start, previous.handle) {
                    return Err(AutomationBatchError::OutOfOrder);
                }
            }
        }
        for (index, left) in records.iter().enumerate() {
            for right in &records[index + 1..] {
                if left.handle != right.handle {
                    continue;
                }
                let left_end = if left.kind == AutomationKind::Point {
                    left.start
                } else {
                    left.end
                };
                let right_end = if right.kind == AutomationKind::Point {
                    right.start
                } else {
                    right.end
                };
                if left.start == right.start || (left.start < right_end && right.start < left_end) {
                    return Err(AutomationBatchError::Overlap);
                }
            }
        }
        Ok(())
    }
}

/// One fixed lightweight control-command reservation; byte payload copying belongs to the later
/// decoded-command schema but its bounded byte accounting is prepared here.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ControlCommandSlot {
    /// Request whose fully copied command bytes occupy this reservation.
    pub request_id: RequestId,
    /// Exact copied command byte count counted against the configured budget.
    pub byte_len: u32,
}

/// Kind-specific outer-header correlation stored by a reliable queue item.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReliableHeader {
    /// A response always echoes a nonzero command request ID.
    Response {
        /// Correlation identity of the originating command.
        request_id: RequestId,
    },
    /// An event has an exact zero outer-header request ID and carries any origin in its payload.
    Event,
}

/// Typed fixed reliable payloads implemented by this control tranche.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReliablePayload {
    /// A response with no queued typed payload in this tranche.
    EmptyResponse,
    /// The frozen reliable `SESSION_COMMITTED` event payload.
    SessionCommitted {
        /// Endpoint-monotonic reliable event sequence.
        event_sequence: u64,
        /// Nonzero transaction request that caused this committed revision.
        origin_request_id: RequestId,
        /// Revision before the atomic replacement.
        previous_revision: SessionRevision,
        /// Number of successfully applied session edits.
        applied_operations: u32,
    },
    /// The frozen reliable `TRANSPORT_STATE` event payload.
    TransportState {
        /// Endpoint-monotonic reliable event sequence.
        event_sequence: u64,
        /// Effective absolute stopped/playing state code.
        state: crate::TransportState,
        /// Effective absolute transport position.
        position: SampleTime,
        /// Effective engine sample.
        effective_sample: SampleTime,
        /// Originating transport-set command, when any.
        origin_request_id: Option<RequestId>,
    },
    /// The frozen reliable `AUTOMATION_CANCELED` event payload.
    AutomationCanceled {
        /// Endpoint-monotonic reliable event sequence.
        event_sequence: u64,
        /// Original accepted automation command identity.
        origin_request_id: RequestId,
        /// Exact record count from the canceled fixed batch.
        canceled_records: u16,
        /// Explicit reason; no accepted batch disappears silently.
        reason: crate::AutomationCancellationReason,
        /// Immutable automation queue generation.
        queue_generation: u64,
        /// Effective endpoint sample if known.
        effective_sample: Option<SampleTime>,
    },
    /// A bounded controller-owned typed diagnostic reference. The diagnostic itself remains in
    /// prepared endpoint storage until its complete event frame has reached caller output.
    Diagnostic {
        /// Index into the endpoint's bounded typed diagnostic store.
        diagnostic_slot: u32,
    },
}

/// One fixed reliable response or event record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReliableSlot {
    /// Kind-specific outer-header correlation. Events are always `Event` and therefore wire zero.
    pub header: ReliableHeader,
    /// Session revision observed by this record.
    pub revision: SessionRevision,
    /// Frozen response/event message ID.
    pub message_id: MessageId,
    /// Typed fixed payload; arbitrary byte payloads are intentionally not representable.
    pub payload: ReliablePayload,
}

impl ReliableSlot {
    /// Construct a typed reliable `SESSION_COMMITTED` event with zero outer-header request ID.
    #[must_use]
    pub const fn session_committed(
        revision: SessionRevision,
        event_sequence: u64,
        origin_request_id: RequestId,
        previous_revision: SessionRevision,
        applied_operations: u32,
    ) -> Self {
        Self {
            header: ReliableHeader::Event,
            revision,
            message_id: MessageId::SessionCommitted,
            payload: ReliablePayload::SessionCommitted {
                event_sequence,
                origin_request_id,
                previous_revision,
                applied_operations,
            },
        }
    }

    /// Construct a typed reliable `TRANSPORT_STATE` event with zero outer-header request ID.
    #[must_use]
    pub const fn transport_state(
        revision: SessionRevision,
        event_sequence: u64,
        state: crate::TransportState,
        position: SampleTime,
        effective_sample: SampleTime,
        origin_request_id: Option<RequestId>,
    ) -> Self {
        Self {
            header: ReliableHeader::Event,
            revision,
            message_id: MessageId::TransportState,
            payload: ReliablePayload::TransportState {
                event_sequence,
                state,
                position,
                effective_sample,
                origin_request_id,
            },
        }
    }

    /// Construct a typed reliable `AUTOMATION_CANCELED` event with zero outer request ID.
    #[must_use]
    pub const fn automation_canceled(
        revision: SessionRevision,
        event_sequence: u64,
        origin_request_id: RequestId,
        canceled_records: u16,
        reason: crate::AutomationCancellationReason,
        queue_generation: u64,
        effective_sample: Option<SampleTime>,
    ) -> Self {
        Self {
            header: ReliableHeader::Event,
            revision,
            message_id: MessageId::AutomationCanceled,
            payload: ReliablePayload::AutomationCanceled {
                event_sequence,
                origin_request_id,
                canceled_records,
                reason,
                queue_generation,
                effective_sample,
            },
        }
    }
}

/// A one-item reliable-event capacity hold that must be committed or released before the next
/// structural transaction. It exists only on the control side; no render operation observes it.
#[derive(Debug)]
pub struct ReliableEventReservation {
    generation: QueueGeneration,
    reservations: Arc<AtomicUsize>,
    armed: bool,
}

/// Multiple atomic reliable-event capacity holds for one structural invalidation.
#[derive(Debug)]
pub struct ReliableEventReservations {
    generation: QueueGeneration,
    remaining: usize,
    reservations: Arc<AtomicUsize>,
}

impl Drop for ReliableEventReservation {
    fn drop(&mut self) {
        if self.armed {
            self.reservations.fetch_sub(1, Ordering::AcqRel);
            self.armed = false;
        }
    }
}

impl Drop for ReliableEventReservations {
    fn drop(&mut self) {
        if self.remaining != 0 {
            self.reservations
                .fetch_sub(self.remaining, Ordering::AcqRel);
            self.remaining = 0;
        }
    }
}

/// Coalescing key for permitted lossy meter/counter telemetry only.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TelemetryKey {
    /// Session revision at observation.
    pub revision: SessionRevision,
    /// Meter/counter handle.
    pub handle: u32,
    /// Meter component or counter subfield.
    pub component: u16,
}

/// One fixed lossy telemetry value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TelemetryRecord {
    /// Replacement/coalescing identity.
    pub key: TelemetryKey,
    /// Observed absolute engine sample.
    pub observed_sample: SampleTime,
    /// Frozen VALID/CLIPPED/HELD meter flags.
    pub flags: u16,
    /// Meter/counter scalar value.
    pub value: f32,
}

/// One fixed lossy counter observation. Counter snapshots are drained into one bounded event
/// batch; they deliberately retain their `u64` values instead of reusing meter `f32` storage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CounterTelemetryRecord {
    /// Replacement/coalescing identity; `component` is zero for counter values.
    pub key: TelemetryKey,
    /// Registered counter identity.
    pub id: crate::CounterId,
    /// Observed absolute engine sample.
    pub observed_sample: SampleTime,
    /// Saturating non-resetting counter value.
    pub value: u64,
}

/// The finite prepared queue capacities and byte/density budgets.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProtocolQueueConfig {
    /// Fixed command-slot capacity.
    pub control_command_slots: NonZeroUsize,
    /// Fixed total copied command-byte budget.
    pub control_command_bytes: NonZeroUsize,
    /// Fixed atomic automation-batch capacity.
    pub automation_batch_slots: NonZeroUsize,
    /// Fixed reliable response capacity.
    pub reliable_response_slots: NonZeroUsize,
    /// Fixed reliable event capacity.
    pub reliable_event_slots: NonZeroUsize,
    /// Fixed lossy telemetry SPSC and staging capacity.
    pub telemetry_slots: NonZeroUsize,
    /// Prepared per-block maximum automation record-start density.
    pub per_block_automation_density: NonZeroUsize,
    /// Explicit render quantum used only for control-side density admission.
    pub quantum_frames: NonZeroUsize,
}

/// Queue creation failure before any caller can enqueue an item.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProtocolQueueError {
    /// An issue-003 `capacity + 1` allocation could not be represented.
    CapacityOverflow,
}

/// Exact engine-owned heap payload budget for one prepared protocol-queue set.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProtocolQueueResourceReport {
    /// Sum of queue headers/backings, staging arrays, automation admission rows, and reservation
    /// credit storage. Allocator metadata and inline [`ProtocolQueues`] bytes are excluded.
    pub retained_payload_bytes: u64,
    /// Largest single requested heap payload allocation among those rows.
    pub largest_allocation_bytes: u64,
}

#[repr(C)]
struct SharedReservationAllocation {
    strong: AtomicUsize,
    weak: AtomicUsize,
    value: AtomicUsize,
}

impl From<SpscError> for ProtocolQueueError {
    fn from(_: SpscError) -> Self {
        Self::CapacityOverflow
    }
}

/// The fixed protocol queue classes used in reports and typed backpressure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueKind {
    /// Reliable decoded command queue.
    ControlCommand,
    /// Reliable fixed automation-batch queue.
    Automation,
    /// Reliable response queue.
    ReliableResponse,
    /// Reliable event queue.
    ReliableEvent,
    /// Lossy meter/counter telemetry queue.
    Telemetry,
}

/// Bounded public-API queue state at an admission attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueReport {
    /// Queue class.
    pub kind: QueueKind,
    /// Exact logical capacity.
    pub capacity: usize,
    /// Current producer-success minus consumer-success occupancy, prior to counter saturation.
    pub occupancy: u64,
    /// Requested atomic slots, always one in BTLV v1.
    pub requested_slots: u16,
    /// Immutable issue-003 queue generation.
    pub generation: QueueGeneration,
}

/// Counters owned by telemetry admission, all saturating at `u64::MAX`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TelemetryCounters {
    /// Number of same-key latest values that replaced a staged record.
    pub telemetry_coalesced: u64,
    /// Number of distinct telemetry keys rejected because staging was full.
    pub telemetry_dropped: u64,
}

/// A reliable-queue full outcome preserving caller ownership.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReliableEnqueueError {
    /// Original unqueued item.
    pub value: ReliableSlot,
    /// Typed saturation report.
    pub report: QueueReport,
}

/// Automation rejection preserving the full unqueued fixed slot.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AutomationEnqueueError {
    /// Admission validation failed without modifying the queue/frontier.
    Invalid {
        /// Original batch remains caller-owned.
        batch: AutomationBatchSlot,
        /// Deterministic admission reason.
        error: AutomationBatchError,
    },
    /// Issue-003 queue was full; original batch remains caller-owned.
    Full {
        /// Original batch remains caller-owned.
        batch: AutomationBatchSlot,
        /// Typed saturation report.
        report: QueueReport,
    },
}

/// Prepared protocol queues. Endpoints are deliberately single-producer/single-consumer and
/// `!Sync` through the public issue-003 endpoint types.
pub struct ProtocolQueues {
    config: ProtocolQueueConfig,
    control: QueuePair<ControlCommandSlot>,
    automation: QueuePair<AutomationBatchSlot>,
    responses: QueuePair<ReliableSlot>,
    events: QueuePair<ReliableSlot>,
    telemetry: QueuePair<TelemetryRecord>,
    telemetry_staging: Box<[Option<TelemetryRecord>]>,
    counter_telemetry: QueuePair<CounterTelemetryRecord>,
    counter_telemetry_staging: Box<[Option<CounterTelemetryRecord>]>,
    telemetry_counters: TelemetryCounters,
    control_used_bytes: usize,
    automation_frontier: Option<SampleTime>,
    automation_density: Box<[AutomationDensityEntry]>,
    automation_intervals: Box<[AutomationIntervalEntry]>,
    reliable_event_reservations: Arc<AtomicUsize>,
}

/// One prepared aggregate count of queued automation record starts for an absolute render block.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct AutomationDensityEntry {
    block: u64,
    starts: usize,
    occupied: bool,
}

/// One preallocated record identity used to reject overlaps across queued batches.
#[derive(Clone, Copy, Debug, PartialEq)]
struct AutomationIntervalEntry {
    record: AutomationRecord,
    occupied: bool,
}

impl Default for AutomationIntervalEntry {
    fn default() -> Self {
        Self {
            record: AutomationRecord::EMPTY,
            occupied: false,
        }
    }
}

impl ProtocolQueues {
    /// Project the exact heap payloads that [`Self::prepare`] requests for this configuration.
    pub fn resource_report_for_config(
        config: ProtocolQueueConfig,
    ) -> Result<ProtocolQueueResourceReport, ProtocolQueueError> {
        let density_entries = config
            .automation_batch_slots
            .get()
            .checked_mul(AUTOMATION_BATCH_RECORDS)
            .ok_or(ProtocolQueueError::CapacityOverflow)?;
        let mut total = 0_u64;
        let mut largest = 0_u64;
        let mut add = |bytes: usize| -> Result<(), ProtocolQueueError> {
            let bytes = u64::try_from(bytes).map_err(|_| ProtocolQueueError::CapacityOverflow)?;
            total = total
                .checked_add(bytes)
                .ok_or(ProtocolQueueError::CapacityOverflow)?;
            largest = largest.max(bytes);
            Ok(())
        };
        macro_rules! queue {
            ($item:ty, $capacity:expr) => {{
                let payload = bounded_spsc_retained_payload::<$item>($capacity)?;
                add(payload.ring_header_bytes)?;
                add(payload.slot_payload_bytes)?;
            }};
        }
        queue!(ControlCommandSlot, config.control_command_slots);
        queue!(AutomationBatchSlot, config.automation_batch_slots);
        queue!(ReliableSlot, config.reliable_response_slots);
        queue!(ReliableSlot, config.reliable_event_slots);
        queue!(TelemetryRecord, config.telemetry_slots);
        queue!(CounterTelemetryRecord, config.telemetry_slots);
        add(
            Layout::array::<Option<TelemetryRecord>>(config.telemetry_slots.get())
                .map_err(|_| ProtocolQueueError::CapacityOverflow)?
                .size(),
        )?;
        add(
            Layout::array::<Option<CounterTelemetryRecord>>(config.telemetry_slots.get())
                .map_err(|_| ProtocolQueueError::CapacityOverflow)?
                .size(),
        )?;
        add(Layout::array::<AutomationDensityEntry>(density_entries)
            .map_err(|_| ProtocolQueueError::CapacityOverflow)?
            .size())?;
        add(Layout::array::<AutomationIntervalEntry>(density_entries)
            .map_err(|_| ProtocolQueueError::CapacityOverflow)?
            .size())?;
        add(Layout::new::<SharedReservationAllocation>().size())?;
        Ok(ProtocolQueueResourceReport {
            retained_payload_bytes: total,
            largest_allocation_bytes: largest,
        })
    }

    /// Allocate all bounded queue and telemetry staging storage off render.
    pub fn prepare(config: ProtocolQueueConfig) -> Result<Self, ProtocolQueueError> {
        let _resources = Self::resource_report_for_config(config)?;
        let density_entries = config
            .automation_batch_slots
            .get()
            .checked_mul(AUTOMATION_BATCH_RECORDS)
            .ok_or(ProtocolQueueError::CapacityOverflow)?;
        Ok(Self {
            control: QueuePair::new(config.control_command_slots, QueueGeneration(1))?,
            automation: QueuePair::new(config.automation_batch_slots, QueueGeneration(2))?,
            responses: QueuePair::new(config.reliable_response_slots, QueueGeneration(3))?,
            events: QueuePair::new(config.reliable_event_slots, QueueGeneration(4))?,
            telemetry: QueuePair::new(config.telemetry_slots, QueueGeneration(5))?,
            telemetry_staging: vec![None; config.telemetry_slots.get()].into_boxed_slice(),
            counter_telemetry: QueuePair::new(config.telemetry_slots, QueueGeneration(6))?,
            counter_telemetry_staging: vec![None; config.telemetry_slots.get()].into_boxed_slice(),
            telemetry_counters: TelemetryCounters::default(),
            control_used_bytes: 0,
            automation_frontier: None,
            automation_density: vec![AutomationDensityEntry::default(); density_entries]
                .into_boxed_slice(),
            automation_intervals: vec![AutomationIntervalEntry::default(); density_entries]
                .into_boxed_slice(),
            reliable_event_reservations: Arc::new(AtomicUsize::new(0)),
            config,
        })
    }

    /// Return the exact prepared configuration.
    #[must_use]
    pub const fn config(&self) -> ProtocolQueueConfig {
        self.config
    }

    /// Try to reserve copied command bytes and one reliable command queue slot atomically.
    pub fn try_enqueue_control(
        &mut self,
        value: ControlCommandSlot,
    ) -> Result<(), ControlCommandSlot> {
        let bytes = usize::try_from(value.byte_len).unwrap_or(usize::MAX);
        if bytes
            > self
                .config
                .control_command_bytes
                .get()
                .saturating_sub(self.control_used_bytes)
        {
            return Err(value);
        }
        match self.control.producer.try_push(value) {
            Ok(()) => {
                self.control_used_bytes = self.control_used_bytes.saturating_add(bytes);
                Ok(())
            }
            Err(full) => Err(full.value),
        }
    }

    /// Pop one fixed command reservation and return its bytes to the configured budget.
    pub fn try_dequeue_control(&mut self) -> Result<ControlCommandSlot, QueueEmpty> {
        let value = self.control.consumer.try_pop()?;
        self.control_used_bytes = self
            .control_used_bytes
            .saturating_sub(usize::try_from(value.byte_len).unwrap_or(usize::MAX));
        Ok(value)
    }

    /// Validate and enqueue one entire fixed automation batch; no partial batch is ever queued.
    #[allow(clippy::result_large_err)] // Required: full caller-owned fixed slot returns on failure.
    pub fn try_enqueue_automation(
        &mut self,
        endpoint_current_sample: SampleTime,
        batch: AutomationBatchSlot,
    ) -> Result<(), AutomationEnqueueError> {
        self.validate_automation_admission(endpoint_current_sample, &batch)
            .map_err(|error| AutomationEnqueueError::Invalid { batch, error })?;
        let frontier = batch.as_slice().last().map(|record| record.start);
        match self.automation.producer.try_push(batch) {
            Ok(()) => {
                self.add_automation_density(&batch);
                self.add_automation_intervals(&batch);
                if let Some(frontier) = frontier {
                    self.automation_frontier = Some(frontier);
                }
                Ok(())
            }
            Err(full) => Err(AutomationEnqueueError::Full {
                batch: full.value,
                report: self.report(QueueKind::Automation),
            }),
        }
    }

    /// Pop one fixed batch without decoding or allocation.
    pub fn try_dequeue_automation(&mut self) -> Result<AutomationBatchSlot, QueueEmpty> {
        let batch = self.automation.consumer.try_pop()?;
        self.remove_automation_density(&batch);
        self.remove_automation_intervals(&batch);
        Ok(batch)
    }

    /// Begin a new accepted-automation ordering epoch after explicit cancellation drained the
    /// queue. This is control-side bookkeeping only and does not allocate.
    pub(crate) fn reset_automation_ordering_after_cancellation(&mut self) {
        debug_assert_eq!(self.automation.report(QueueKind::Automation).occupancy, 0);
        self.automation_frontier = None;
    }

    /// Enqueue a reliable response or preserve it exactly on full.
    pub fn try_enqueue_response(
        &mut self,
        value: ReliableSlot,
    ) -> Result<(), ReliableEnqueueError> {
        match self.responses.producer.try_push(value) {
            Ok(()) => Ok(()),
            Err(full) => Err(ReliableEnqueueError {
                value: full.value,
                report: self.report(QueueKind::ReliableResponse),
            }),
        }
    }

    /// Pop one reliable response.
    pub fn try_dequeue_response(&mut self) -> Result<ReliableSlot, QueueEmpty> {
        self.responses.consumer.try_pop()
    }

    /// Enqueue a reliable event or preserve it exactly on full.
    pub fn try_enqueue_event(&mut self, value: ReliableSlot) -> Result<(), ReliableEnqueueError> {
        let reserved = self.reliable_event_reservations.load(Ordering::Acquire);
        if reserved != 0
            && self
                .events
                .report(QueueKind::ReliableEvent)
                .occupancy
                .saturating_add(u64::try_from(reserved).unwrap_or(u64::MAX))
                >= u64::try_from(self.events.producer.capacity()).unwrap_or(u64::MAX)
        {
            return Err(ReliableEnqueueError {
                value,
                report: self.report(QueueKind::ReliableEvent),
            });
        }
        match self.events.producer.try_push(value) {
            Ok(()) => Ok(()),
            Err(full) => Err(ReliableEnqueueError {
                value: full.value,
                report: self.report(QueueKind::ReliableEvent),
            }),
        }
    }

    /// Pop one reliable event.
    pub fn try_dequeue_event(&mut self) -> Result<ReliableSlot, QueueEmpty> {
        self.events.consumer.try_pop()
    }

    /// Reserve one reliable event slot before an atomic session replacement. A full queue leaves
    /// the caller with a typed report and does not modify either queue state or a session store.
    pub fn reserve_reliable_event(&mut self) -> Result<ReliableEventReservation, QueueReport> {
        let report = self.report(QueueKind::ReliableEvent);
        if report.occupancy.saturating_add(
            u64::try_from(self.reliable_event_reservations.load(Ordering::Acquire))
                .unwrap_or(u64::MAX),
        ) >= u64::try_from(report.capacity).unwrap_or(u64::MAX)
        {
            return Err(report);
        }
        self.reliable_event_reservations
            .fetch_add(1, Ordering::AcqRel);
        Ok(ReliableEventReservation {
            generation: report.generation,
            reservations: Arc::clone(&self.reliable_event_reservations),
            armed: true,
        })
    }

    /// Reserve every reliable event needed before an action can invalidate accepted automation.
    /// A failure leaves both the automation queue and the action's caller-owned state unchanged.
    pub fn reserve_reliable_events(
        &mut self,
        count: usize,
    ) -> Result<ReliableEventReservations, QueueReport> {
        let report = self.report(QueueKind::ReliableEvent);
        let reserved = self.reliable_event_reservations.load(Ordering::Acquire);
        let capacity = report.capacity;
        if count
            > capacity
                .saturating_sub(usize::try_from(report.occupancy).unwrap_or(usize::MAX))
                .saturating_sub(reserved)
        {
            return Err(report);
        }
        self.reliable_event_reservations
            .fetch_add(count, Ordering::AcqRel);
        Ok(ReliableEventReservations {
            generation: report.generation,
            remaining: count,
            reservations: Arc::clone(&self.reliable_event_reservations),
        })
    }

    /// Commit one event from a prior multi-event reservation.
    pub fn commit_reserved_reliable_event(
        &mut self,
        reservations: &mut ReliableEventReservations,
        value: ReliableSlot,
    ) {
        debug_assert_eq!(reservations.generation, self.events.producer.generation());
        debug_assert_ne!(reservations.remaining, 0);
        debug_assert_ne!(self.reliable_event_reservations.load(Ordering::Acquire), 0);
        reservations.remaining = reservations.remaining.saturating_sub(1);
        self.reliable_event_reservations
            .fetch_sub(1, Ordering::AcqRel);
        match self.events.producer.try_push(value) {
            Ok(()) => {}
            Err(_) => unreachable!("a reserved reliable-event slot must remain available"),
        }
    }

    /// Release all uncommitted multi-event reservation capacity on a failed transaction.
    pub fn release_reliable_events(&mut self, reservations: ReliableEventReservations) {
        debug_assert_eq!(reservations.generation, self.events.producer.generation());
        drop(reservations);
    }

    /// Publish the exact event covered by a prior reservation. The reservation makes a full
    /// outcome unreachable because `ProtocolQueues` has one exclusive control-side producer.
    pub fn commit_reliable_event(
        &mut self,
        mut reservation: ReliableEventReservation,
        value: ReliableSlot,
    ) {
        debug_assert_eq!(reservation.generation, self.events.producer.generation());
        debug_assert_ne!(self.reliable_event_reservations.load(Ordering::Acquire), 0);
        self.reliable_event_reservations
            .fetch_sub(1, Ordering::AcqRel);
        reservation.armed = false;
        match self.events.producer.try_push(value) {
            Ok(()) => {}
            Err(_) => unreachable!("a reserved reliable-event slot must remain available"),
        }
    }

    /// Release a pre-commit reservation when session validation rejects its transaction.
    pub fn release_reliable_event(&mut self, reservation: ReliableEventReservation) {
        debug_assert_eq!(reservation.generation, self.events.producer.generation());
        drop(reservation);
    }

    /// Stage the latest permitted lossy telemetry. Only a same key coalesces; a distinct key at
    /// prepared staging capacity drops and increments the required saturating counter.
    pub fn stage_telemetry(&mut self, value: TelemetryRecord) {
        if let Some(slot) = self.telemetry_staging.iter_mut().find(|slot| {
            slot.as_ref()
                .is_some_and(|existing| existing.key == value.key)
        }) {
            *slot = Some(value);
            self.telemetry_counters.telemetry_coalesced = self
                .telemetry_counters
                .telemetry_coalesced
                .saturating_add(1);
        } else if let Some(slot) = self
            .telemetry_staging
            .iter_mut()
            .find(|slot| slot.is_none())
        {
            *slot = Some(value);
        } else {
            self.telemetry_counters.telemetry_dropped =
                self.telemetry_counters.telemetry_dropped.saturating_add(1);
        }
    }

    /// Move staged telemetry into the fixed SPSC queue until it is full. Staging entries that do
    /// not fit remain latest values for a later control-side flush rather than being discarded.
    pub fn flush_telemetry(&mut self) {
        for slot in &mut self.telemetry_staging {
            let Some(value) = *slot else {
                continue;
            };
            match self.telemetry.producer.try_push(value) {
                Ok(()) => *slot = None,
                Err(QueueFull { .. }) => break,
            }
        }
    }

    /// Stage the latest permitted lossy counter observation. Counter records share the explicit
    /// telemetry coalesce/drop counters but have their own prepared fixed-value queue because
    /// counter values are `u64`, not meter `f32` samples.
    pub fn stage_counter_telemetry(&mut self, value: CounterTelemetryRecord) {
        if let Some(slot) = self.counter_telemetry_staging.iter_mut().find(|slot| {
            slot.as_ref()
                .is_some_and(|existing| existing.key == value.key)
        }) {
            *slot = Some(value);
            self.telemetry_counters.telemetry_coalesced = self
                .telemetry_counters
                .telemetry_coalesced
                .saturating_add(1);
        } else if let Some(slot) = self
            .counter_telemetry_staging
            .iter_mut()
            .find(|slot| slot.is_none())
        {
            *slot = Some(value);
        } else {
            self.telemetry_counters.telemetry_dropped =
                self.telemetry_counters.telemetry_dropped.saturating_add(1);
        }
    }

    /// Move staged counter observations into their fixed lossy queue without dropping values
    /// merely because the consumer has not drained the current batch yet.
    pub fn flush_counter_telemetry(&mut self) {
        for slot in &mut self.counter_telemetry_staging {
            let Some(value) = *slot else {
                continue;
            };
            match self.counter_telemetry.producer.try_push(value) {
                Ok(()) => *slot = None,
                Err(QueueFull { .. }) => break,
            }
        }
    }

    /// Pop one lossy meter/counter telemetry record.
    pub fn try_dequeue_telemetry(&mut self) -> Result<TelemetryRecord, QueueEmpty> {
        self.telemetry.consumer.try_pop()
    }

    /// Pop one fixed lossy counter observation for bounded event batching.
    pub fn try_dequeue_counter_telemetry(&mut self) -> Result<CounterTelemetryRecord, QueueEmpty> {
        self.counter_telemetry.consumer.try_pop()
    }

    /// Read required lossy-telemetry counters without resetting them.
    #[must_use]
    pub const fn telemetry_counters(&self) -> TelemetryCounters {
        self.telemetry_counters
    }

    #[cfg(test)]
    fn set_telemetry_counters_for_test(&mut self, counters: TelemetryCounters) {
        self.telemetry_counters = counters;
    }

    /// Return a typed report for one queue using public endpoint success counters and generation.
    #[must_use]
    pub fn report(&self, kind: QueueKind) -> QueueReport {
        match kind {
            QueueKind::ControlCommand => self.control.report(QueueKind::ControlCommand),
            QueueKind::Automation => self.automation.report(QueueKind::Automation),
            QueueKind::ReliableResponse => self.responses.report(QueueKind::ReliableResponse),
            QueueKind::ReliableEvent => self.events.report(QueueKind::ReliableEvent),
            QueueKind::Telemetry => self.telemetry.report(QueueKind::Telemetry),
        }
    }

    fn validate_automation_admission(
        &self,
        endpoint_current_sample: SampleTime,
        batch: &AutomationBatchSlot,
    ) -> Result<(), AutomationBatchError> {
        batch.validate_records()?;
        let records = batch.as_slice();
        if let (Some(frontier), Some(first)) = (self.automation_frontier, records.first())
            && first.start < frontier
        {
            return Err(AutomationBatchError::GlobalTimeBackwards);
        }
        if records
            .iter()
            .any(|record| record.start < endpoint_current_sample)
        {
            return Err(AutomationBatchError::TimeInPast);
        }
        for record in records {
            if self.automation_intervals.iter().any(|entry| {
                entry.occupied
                    && entry.record.handle == record.handle
                    && automation_records_overlap(entry.record, *record)
            }) {
                return Err(AutomationBatchError::Overlap);
            }
            let block = record.start.0
                / u64::try_from(self.config.quantum_frames.get()).unwrap_or(u64::MAX);
            let density = self.density_for_block(block).saturating_add(
                records
                    .iter()
                    .filter(|candidate| {
                        candidate.start.0
                            / u64::try_from(self.config.quantum_frames.get()).unwrap_or(u64::MAX)
                            == block
                    })
                    .count(),
            );
            if density > self.config.per_block_automation_density.get() {
                return Err(AutomationBatchError::DensityExceeded);
            }
        }
        Ok(())
    }

    fn density_for_block(&self, block: u64) -> usize {
        self.automation_density
            .iter()
            .find(|entry| entry.occupied && entry.block == block)
            .map_or(0, |entry| entry.starts)
    }

    fn add_automation_density(&mut self, batch: &AutomationBatchSlot) {
        for record in batch.as_slice() {
            let block = record.start.0
                / u64::try_from(self.config.quantum_frames.get()).unwrap_or(u64::MAX);
            if let Some(entry) = self
                .automation_density
                .iter_mut()
                .find(|entry| entry.occupied && entry.block == block)
            {
                entry.starts = entry.starts.saturating_add(1);
            } else if let Some(entry) = self
                .automation_density
                .iter_mut()
                .find(|entry| !entry.occupied)
            {
                *entry = AutomationDensityEntry {
                    block,
                    starts: 1,
                    occupied: true,
                };
            } else {
                unreachable!("prepared density entries cover every queued fixed record");
            }
        }
    }

    fn remove_automation_density(&mut self, batch: &AutomationBatchSlot) {
        for record in batch.as_slice() {
            let block = record.start.0
                / u64::try_from(self.config.quantum_frames.get()).unwrap_or(u64::MAX);
            let entry = self
                .automation_density
                .iter_mut()
                .find(|entry| entry.occupied && entry.block == block)
                .expect("queued automation record must have prepared density state");
            entry.starts = entry.starts.saturating_sub(1);
            if entry.starts == 0 {
                *entry = AutomationDensityEntry::default();
            }
        }
    }

    fn add_automation_intervals(&mut self, batch: &AutomationBatchSlot) {
        for record in batch.as_slice() {
            let entry = self
                .automation_intervals
                .iter_mut()
                .find(|entry| !entry.occupied)
                .expect("prepared interval entries cover every queued fixed record");
            *entry = AutomationIntervalEntry {
                record: *record,
                occupied: true,
            };
        }
    }

    fn remove_automation_intervals(&mut self, batch: &AutomationBatchSlot) {
        for record in batch.as_slice() {
            let entry = self
                .automation_intervals
                .iter_mut()
                .find(|entry| entry.occupied && entry.record == *record)
                .expect("queued automation record must have prepared interval state");
            *entry = AutomationIntervalEntry::default();
        }
    }
}

fn automation_records_overlap(left: AutomationRecord, right: AutomationRecord) -> bool {
    let left_end = if left.kind == AutomationKind::Point {
        left.start
    } else {
        left.end
    };
    let right_end = if right.kind == AutomationKind::Point {
        right.start
    } else {
        right.end
    };
    left.start == right.start || (left.start < right_end && right.start < left_end)
}

struct QueuePair<T: Copy + Send + 'static> {
    producer: Producer<T>,
    consumer: Consumer<T>,
}

impl<T: Copy + Send + 'static> QueuePair<T> {
    fn new(
        capacity: NonZeroUsize,
        generation: QueueGeneration,
    ) -> Result<Self, ProtocolQueueError> {
        let (producer, consumer) = bounded_spsc(capacity, generation)?;
        Ok(Self { producer, consumer })
    }

    fn report(&self, kind: QueueKind) -> QueueReport {
        QueueReport {
            kind,
            capacity: self.producer.capacity(),
            occupancy: self
                .producer
                .success_count()
                .saturating_sub(self.consumer.success_count()),
            requested_slots: 1,
            generation: self.producer.generation(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(value: u64) -> RequestId {
        RequestId::new(value).expect("nonzero")
    }

    fn config(automation_slots: usize, density: usize) -> ProtocolQueueConfig {
        ProtocolQueueConfig {
            control_command_slots: NonZeroUsize::new(2).expect("two"),
            control_command_bytes: NonZeroUsize::new(16).expect("bytes"),
            automation_batch_slots: NonZeroUsize::new(automation_slots).expect("automation"),
            reliable_response_slots: NonZeroUsize::new(1).expect("response"),
            reliable_event_slots: NonZeroUsize::new(1).expect("event"),
            telemetry_slots: NonZeroUsize::new(2).expect("telemetry"),
            per_block_automation_density: NonZeroUsize::new(density).expect("density"),
            quantum_frames: NonZeroUsize::new(1).expect("quantum"),
        }
    }

    fn point(sample: u64, handle: u32) -> AutomationRecord {
        AutomationRecord {
            kind: AutomationKind::Point,
            handle: ParameterHandle(handle),
            start: SampleTime(sample),
            end: SampleTime(sample),
            start_value: sample as f32,
            end_value: sample as f32,
        }
    }

    #[test]
    fn queue_resource_projection_is_nonzero_and_shares_prepare_overflow_rules() {
        let accepted = config(1, 256);
        let report = ProtocolQueues::resource_report_for_config(accepted).expect("projection");
        assert!(report.retained_payload_bytes > report.largest_allocation_bytes);
        assert!(report.largest_allocation_bytes > 0);
        ProtocolQueues::prepare(accepted).expect("matching preparation");

        let overflowing = ProtocolQueueConfig {
            automation_batch_slots: NonZeroUsize::new(usize::MAX).expect("maximum is nonzero"),
            ..accepted
        };
        assert_eq!(
            ProtocolQueues::resource_report_for_config(overflowing),
            Err(ProtocolQueueError::CapacityOverflow)
        );
        assert!(matches!(
            ProtocolQueues::prepare(overflowing),
            Err(ProtocolQueueError::CapacityOverflow)
        ));
    }

    fn batch(request_id: u64, start: u64, records: usize) -> AutomationBatchSlot {
        let values = (0..records)
            .map(|index| point(start + u64::try_from(index).expect("index"), 1))
            .collect::<Vec<_>>();
        AutomationBatchSlot::new(SessionRevision(7), request(request_id), &values).expect("batch")
    }

    #[test]
    fn transient_record_has_exact_manual_le_codec() {
        let record = AutomationRecord {
            kind: AutomationKind::Linear,
            handle: ParameterHandle(7),
            start: SampleTime(8),
            end: SampleTime(16),
            start_value: 0.25,
            end_value: 0.5,
        };
        let mut bytes = [0; AUTOMATION_RECORD_BYTES];
        record.encode_le(&mut bytes).expect("encode");
        assert_eq!(bytes[0], 3);
        assert_eq!(AutomationRecord::decode_le(&bytes), Ok(record));
        bytes[1] = 1;
        assert_eq!(
            AutomationRecord::decode_le(&bytes),
            Err(AutomationBatchError::NonzeroReserved)
        );
    }

    #[test]
    fn full_empty_wrap_and_generation_preserve_original_items() {
        let mut queues = ProtocolQueues::prepare(config(1, 256)).expect("prepare");
        let first = batch(1, 0, 1);
        queues
            .try_enqueue_automation(SampleTime(0), first)
            .expect("enqueue");
        let second = batch(2, 1, 1);
        let returned = queues.try_enqueue_automation(SampleTime(0), second);
        assert!(matches!(
            returned,
            Err(AutomationEnqueueError::Full { batch, report })
                if batch == second && report.capacity == 1 && report.generation == QueueGeneration(2)
        ));
        assert_eq!(queues.try_dequeue_automation().expect("pop"), first);
        queues
            .try_enqueue_automation(SampleTime(0), second)
            .expect("wrap");
        assert_eq!(queues.try_dequeue_automation().expect("pop"), second);
        assert!(queues.try_dequeue_automation().is_err());
    }

    #[test]
    fn ten_thousand_events_fit_as_exactly_forty_atomic_batches() {
        let mut queues = ProtocolQueues::prepare(config(40, 256)).expect("prepare");
        let mut starts = 0_u64;
        for batch_index in 0..40_u64 {
            let count = if batch_index < 39 { 256 } else { 16 };
            let value = batch(batch_index + 1, starts, count);
            queues
                .try_enqueue_automation(SampleTime(0), value)
                .expect("whole batch");
            starts += u64::try_from(count).expect("count");
        }
        assert_eq!(starts, 10_000);
        let mut consumed = 0usize;
        for _ in 0..40 {
            consumed += queues
                .try_dequeue_automation()
                .expect("fixed consumer")
                .as_slice()
                .len();
        }
        assert_eq!(consumed, 10_000);
    }

    #[test]
    fn past_order_overlap_and_density_reject_wholly() {
        let mut queues = ProtocolQueues::prepare(config(2, 1)).expect("prepare");
        let past = batch(1, 4, 1);
        assert!(matches!(
            queues.try_enqueue_automation(SampleTime(5), past),
            Err(AutomationEnqueueError::Invalid { batch, error: AutomationBatchError::TimeInPast }) if batch == past
        ));
        let dense =
            AutomationBatchSlot::new(SessionRevision(7), request(2), &[point(5, 1), point(5, 2)])
                .expect("batch");
        assert!(matches!(
            queues.try_enqueue_automation(SampleTime(5), dense),
            Err(AutomationEnqueueError::Invalid {
                error: AutomationBatchError::DensityExceeded,
                ..
            })
        ));
        let first = batch(3, 10, 1);
        queues
            .try_enqueue_automation(SampleTime(0), first)
            .expect("first");
        let behind = batch(4, 9, 1);
        assert!(matches!(
            queues.try_enqueue_automation(SampleTime(0), behind),
            Err(AutomationEnqueueError::Invalid {
                error: AutomationBatchError::GlobalTimeBackwards,
                ..
            })
        ));
        assert_eq!(queues.try_dequeue_automation().expect("only first"), first);
    }

    #[test]
    fn overlap_is_rejected_across_separately_queued_batches() {
        let mut queues = ProtocolQueues::prepare(config(2, 256)).expect("prepare");
        let segment = AutomationRecord {
            kind: AutomationKind::Linear,
            handle: ParameterHandle(7),
            start: SampleTime(10),
            end: SampleTime(20),
            start_value: 0.0,
            end_value: 1.0,
        };
        let first = AutomationBatchSlot::new(SessionRevision(7), request(1), &[segment])
            .expect("segment batch");
        queues
            .try_enqueue_automation(SampleTime(0), first)
            .expect("first queued batch");
        let overlapping = AutomationBatchSlot::new(SessionRevision(7), request(2), &[point(15, 7)])
            .expect("point batch");
        assert!(matches!(
            queues.try_enqueue_automation(SampleTime(0), overlapping),
            Err(AutomationEnqueueError::Invalid {
                batch,
                error: AutomationBatchError::Overlap
            }) if batch == overlapping
        ));
        assert_eq!(
            queues.try_dequeue_automation().expect("original batch"),
            first
        );
        assert!(queues.try_dequeue_automation().is_err());
    }

    #[test]
    fn invalid_and_out_of_order_slots_return_whole_unchanged_batches() {
        let mut queues = ProtocolQueues::prepare(config(2, 256)).expect("prepare");
        let mut invalid_records = [AutomationRecord::EMPTY; AUTOMATION_BATCH_RECORDS];
        invalid_records[0] = AutomationRecord {
            kind: AutomationKind::Linear,
            handle: ParameterHandle(1),
            start: SampleTime(2),
            end: SampleTime(2),
            start_value: 0.0,
            end_value: 1.0,
        };
        let invalid = AutomationBatchSlot {
            revision: SessionRevision(7),
            request_id: request(1),
            len: 1,
            records: invalid_records,
        };
        assert!(matches!(
            queues.try_enqueue_automation(SampleTime(0), invalid),
            Err(AutomationEnqueueError::Invalid { batch, error: AutomationBatchError::InvalidRange }) if batch == invalid
        ));
        let mut unordered_records = [AutomationRecord::EMPTY; AUTOMATION_BATCH_RECORDS];
        unordered_records[0] = point(3, 1);
        unordered_records[1] = point(2, 2);
        let unordered = AutomationBatchSlot {
            revision: SessionRevision(7),
            request_id: request(2),
            len: 2,
            records: unordered_records,
        };
        assert!(matches!(
            queues.try_enqueue_automation(SampleTime(0), unordered),
            Err(AutomationEnqueueError::Invalid { batch, error: AutomationBatchError::OutOfOrder }) if batch == unordered
        ));
        assert!(queues.try_dequeue_automation().is_err());
    }

    #[test]
    fn zero_record_and_aggregate_frontier_density_reject_wholly() {
        assert_eq!(
            AutomationBatchSlot::new(SessionRevision(7), request(1), &[]),
            Err(AutomationBatchError::EmptyBatch)
        );
        let mut queues = ProtocolQueues::prepare(config(2, 1)).expect("prepare");
        let first = AutomationBatchSlot::new(SessionRevision(7), request(2), &[point(8, 1)])
            .expect("first");
        let second = AutomationBatchSlot::new(SessionRevision(7), request(3), &[point(8, 2)])
            .expect("second");
        queues
            .try_enqueue_automation(SampleTime(0), first)
            .expect("first batch");
        assert!(matches!(
            queues.try_enqueue_automation(SampleTime(0), second),
            Err(AutomationEnqueueError::Invalid {
                batch,
                error: AutomationBatchError::DensityExceeded,
            }) if batch == second
        ));
        assert_eq!(
            queues.try_dequeue_automation().expect("first remains"),
            first
        );
        queues
            .try_enqueue_automation(SampleTime(0), second)
            .expect("density is released only by the consumed batch");
    }

    #[test]
    fn reliable_never_coalesces_and_lossy_telemetry_is_explicit() {
        let mut queues = ProtocolQueues::prepare(config(1, 256)).expect("prepare");
        let response = ReliableSlot {
            header: ReliableHeader::Response {
                request_id: request(1),
            },
            revision: SessionRevision(7),
            message_id: MessageId::CapabilitiesGet,
            payload: ReliablePayload::EmptyResponse,
        };
        queues.try_enqueue_response(response).expect("reliable");
        assert!(
            matches!(queues.try_enqueue_response(response), Err(ReliableEnqueueError { value, .. }) if value == response)
        );
        let first = TelemetryRecord {
            key: TelemetryKey {
                revision: SessionRevision(7),
                handle: 1,
                component: 0,
            },
            observed_sample: SampleTime(1),
            flags: 1,
            value: 1.0,
        };
        let replacement = TelemetryRecord {
            value: 2.0,
            ..first
        };
        queues.stage_telemetry(first);
        queues.stage_telemetry(replacement);
        queues.flush_telemetry();
        assert_eq!(queues.try_dequeue_telemetry().expect("latest"), replacement);
        assert_eq!(queues.telemetry_counters().telemetry_coalesced, 1);
        queues.stage_telemetry(TelemetryRecord {
            key: TelemetryKey {
                revision: SessionRevision(7),
                handle: 2,
                component: 0,
            },
            observed_sample: SampleTime(1),
            flags: 1,
            value: 1.0,
        });
        queues.stage_telemetry(TelemetryRecord {
            key: TelemetryKey {
                revision: SessionRevision(7),
                handle: 3,
                component: 0,
            },
            observed_sample: SampleTime(1),
            flags: 1,
            value: 1.0,
        });
        queues.stage_telemetry(TelemetryRecord {
            key: TelemetryKey {
                revision: SessionRevision(7),
                handle: 4,
                component: 0,
            },
            observed_sample: SampleTime(1),
            flags: 1,
            value: 1.0,
        });
        assert_eq!(queues.telemetry_counters().telemetry_dropped, 1);
    }

    #[test]
    fn reliable_event_reservation_excludes_commit_capacity() {
        let mut queues = ProtocolQueues::prepare(config(1, 256)).expect("prepare");
        let reservation = queues.reserve_reliable_event().expect("reserve");
        let event = ReliableSlot::session_committed(
            SessionRevision(8),
            1,
            request(1),
            SessionRevision(7),
            1,
        );
        assert!(matches!(
            queues.try_enqueue_event(event),
            Err(ReliableEnqueueError { value, report })
                if value == event && report.kind == QueueKind::ReliableEvent
        ));
        queues.commit_reliable_event(reservation, event);
        assert_eq!(queues.try_dequeue_event().expect("reserved event"), event);
    }

    #[test]
    fn lossy_coalesce_and_drop_counters_saturate_under_fault_injection() {
        let mut queues = ProtocolQueues::prepare(config(1, 256)).expect("prepare");
        queues.set_telemetry_counters_for_test(TelemetryCounters {
            telemetry_coalesced: u64::MAX,
            telemetry_dropped: u64::MAX,
        });
        let first = TelemetryRecord {
            key: TelemetryKey {
                revision: SessionRevision(7),
                handle: 1,
                component: 1,
            },
            observed_sample: SampleTime(1),
            flags: 1,
            value: 1.0,
        };
        queues.stage_telemetry(first);
        queues.stage_telemetry(TelemetryRecord {
            value: 2.0,
            ..first
        });
        queues.stage_telemetry(TelemetryRecord {
            key: TelemetryKey {
                handle: 2,
                ..first.key
            },
            ..first
        });
        assert_eq!(
            queues.telemetry_counters(),
            TelemetryCounters {
                telemetry_coalesced: u64::MAX,
                telemetry_dropped: u64::MAX,
            }
        );
    }
}
