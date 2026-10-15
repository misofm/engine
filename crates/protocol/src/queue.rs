//! Prepared protocol queues.
//!
//! All queue storage is created off render and delegates to issue-003's public bounded SPSC API.
//! The consumer operations below only move fixed `Copy` slots; they do not decode, allocate,
//! compile, publish a plan, or structurally mutate a session.

use core::{
    alloc::Layout,
    num::NonZeroUsize,
    sync::atomic::{AtomicUsize, Ordering},
};
use std::sync::Arc;

use engine::realtime::{
    Consumer, Producer, QueueEmpty, QueueFull, QueueGeneration, SpscError, bounded_spsc,
    bounded_spsc_retained_payload,
};

use crate::{MessageId, RequestId, SampleTime, SessionRevision};

/// One fixed lightweight control-command reservation. The control queue is prepared and reported
/// with these slots, but nothing enqueues one: its never-called enqueue/dequeue methods and their
/// byte accounting were removed as unused (#1034).
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
        /// Effective engine sample.
        effective_sample: SampleTime,
        /// Originating transport-set command, when any.
        origin_request_id: Option<RequestId>,
    },
    /// A reference to a reliable `OBSERVATION_ENDED` event (#1584 D6a). The event's key is
    /// variable-length text, so it stays in prepared endpoint storage until its complete event
    /// frame has reached caller output, as a diagnostic does.
    ObservationEnded {
        /// Index into the endpoint's bounded observation-ended store.
        cell: u32,
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

    /// Construct a reliable `OBSERVATION_ENDED` event referencing prepared storage `cell`, with
    /// zero outer-header request ID. `revision` is the revision that removed the tap.
    #[must_use]
    pub const fn observation_ended(revision: SessionRevision, cell: u32) -> Self {
        Self {
            header: ReliableHeader::Event,
            revision,
            message_id: MessageId::ObservationEnded,
            payload: ReliablePayload::ObservationEnded { cell },
        }
    }

    /// Construct a typed reliable `TRANSPORT_STATE` event with zero outer-header request ID.
    #[must_use]
    pub const fn transport_state(
        revision: SessionRevision,
        event_sequence: u64,
        state: crate::TransportState,
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
                effective_sample,
                origin_request_id,
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

/// The finite prepared queue capacities and byte budgets.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProtocolQueueConfig {
    /// Fixed command-slot capacity.
    pub control_command_slots: NonZeroUsize,
    /// Fixed total copied command-byte budget.
    pub control_command_bytes: NonZeroUsize,
    /// Fixed reliable response capacity.
    pub reliable_response_slots: NonZeroUsize,
    /// Fixed reliable event capacity.
    pub reliable_event_slots: NonZeroUsize,
    /// Fixed lossy telemetry SPSC and staging capacity.
    pub telemetry_slots: NonZeroUsize,
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
    /// Sum of queue headers/backings, staging arrays, and reservation credit storage. Allocator metadata and inline [`ProtocolQueues`] bytes are excluded.
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

/// Prepared protocol queues. Endpoints are deliberately single-producer/single-consumer and
/// `!Sync` through the public issue-003 endpoint types.
pub struct ProtocolQueues {
    config: ProtocolQueueConfig,
    control: QueuePair<ControlCommandSlot>,
    responses: QueuePair<ReliableSlot>,
    events: QueuePair<ReliableSlot>,
    telemetry: QueuePair<TelemetryRecord>,
    telemetry_staging: Box<[Option<TelemetryRecord>]>,
    counter_telemetry: QueuePair<CounterTelemetryRecord>,
    counter_telemetry_staging: Box<[Option<CounterTelemetryRecord>]>,
    telemetry_counters: TelemetryCounters,
    reliable_event_reservations: Arc<AtomicUsize>,
}

impl ProtocolQueues {
    /// Project the exact heap payloads that [`Self::prepare`] requests for this configuration.
    pub fn resource_report_for_config(
        config: ProtocolQueueConfig,
    ) -> Result<ProtocolQueueResourceReport, ProtocolQueueError> {
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
        add(Layout::new::<SharedReservationAllocation>().size())?;
        Ok(ProtocolQueueResourceReport {
            retained_payload_bytes: total,
            largest_allocation_bytes: largest,
        })
    }

    /// Allocate all bounded queue and telemetry staging storage off render.
    pub fn prepare(config: ProtocolQueueConfig) -> Result<Self, ProtocolQueueError> {
        let _resources = Self::resource_report_for_config(config)?;
        Ok(Self {
            control: QueuePair::new(config.control_command_slots, QueueGeneration(1))?,
            responses: QueuePair::new(config.reliable_response_slots, QueueGeneration(3))?,
            events: QueuePair::new(config.reliable_event_slots, QueueGeneration(4))?,
            telemetry: QueuePair::new(config.telemetry_slots, QueueGeneration(5))?,
            telemetry_staging: vec![None; config.telemetry_slots.get()].into_boxed_slice(),
            counter_telemetry: QueuePair::new(config.telemetry_slots, QueueGeneration(6))?,
            counter_telemetry_staging: vec![None; config.telemetry_slots.get()].into_boxed_slice(),
            telemetry_counters: TelemetryCounters::default(),
            reliable_event_reservations: Arc::new(AtomicUsize::new(0)),
            config,
        })
    }

    /// Return the exact prepared configuration.
    #[must_use]
    pub const fn config(&self) -> ProtocolQueueConfig {
        self.config
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

    /// Reserve every reliable event an action needs before it commits. A failure leaves the
    /// queue and the action's caller-owned state unchanged.
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
            QueueKind::ReliableResponse => self.responses.report(QueueKind::ReliableResponse),
            QueueKind::ReliableEvent => self.events.report(QueueKind::ReliableEvent),
            QueueKind::Telemetry => self.telemetry.report(QueueKind::Telemetry),
        }
    }
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

    fn config() -> ProtocolQueueConfig {
        ProtocolQueueConfig {
            control_command_slots: NonZeroUsize::new(2).expect("two"),
            control_command_bytes: NonZeroUsize::new(16).expect("bytes"),
            reliable_response_slots: NonZeroUsize::new(1).expect("response"),
            reliable_event_slots: NonZeroUsize::new(1).expect("event"),
            telemetry_slots: NonZeroUsize::new(2).expect("telemetry"),
        }
    }

    #[test]
    fn queue_resource_projection_is_nonzero_and_shares_prepare_overflow_rules() {
        let accepted = config();
        let report = ProtocolQueues::resource_report_for_config(accepted).expect("projection");
        assert!(report.retained_payload_bytes > report.largest_allocation_bytes);
        assert!(report.largest_allocation_bytes > 0);
        ProtocolQueues::prepare(accepted).expect("matching preparation");

        let overflowing = ProtocolQueueConfig {
            reliable_event_slots: NonZeroUsize::new(usize::MAX).expect("maximum is nonzero"),
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

    #[test]
    fn full_empty_wrap_and_generation_preserve_original_items() {
        let mut queues = ProtocolQueues::prepare(config()).expect("prepare");
        let control = queues.report(QueueKind::ControlCommand);
        assert_eq!(
            (control.capacity, control.generation),
            (2, QueueGeneration(1))
        );

        let response = |id: u64| ReliableSlot {
            header: ReliableHeader::Response {
                request_id: request(id),
            },
            revision: SessionRevision(7),
            message_id: MessageId::CapabilitiesGet,
            payload: ReliablePayload::EmptyResponse,
        };
        let (first, second) = (response(1), response(2));
        queues.try_enqueue_response(first).expect("response");
        assert!(matches!(
            queues.try_enqueue_response(second),
            Err(ReliableEnqueueError { value, report })
                if value == second
                    && report.kind == QueueKind::ReliableResponse
                    && report.capacity == 1
                    && report.occupancy == 1
                    && report.generation == QueueGeneration(3)
        ));

        let event = |sequence: u64| {
            ReliableSlot::session_committed(
                SessionRevision(8),
                sequence,
                request(sequence),
                SessionRevision(7),
                1,
            )
        };
        let (first, second) = (event(1), event(2));
        queues.try_enqueue_event(first).expect("enqueue");
        assert!(matches!(
            queues.try_enqueue_event(second),
            Err(ReliableEnqueueError { value, report })
                if value == second && report.capacity == 1 && report.generation == QueueGeneration(4)
        ));
        assert_eq!(queues.try_dequeue_event().expect("pop"), first);
        queues.try_enqueue_event(second).expect("wrap");
        assert_eq!(queues.try_dequeue_event().expect("pop"), second);
        assert!(queues.try_dequeue_event().is_err());

        let record = |handle: u32| TelemetryRecord {
            key: TelemetryKey {
                revision: SessionRevision(7),
                handle,
                component: 0,
            },
            observed_sample: SampleTime(1),
            flags: 1,
            value: 1.0,
        };
        for round in 0..3 {
            queues.stage_telemetry(record(round));
            queues.flush_telemetry();
            assert_eq!(queues.try_dequeue_telemetry().expect("pop"), record(round));
            assert!(queues.try_dequeue_telemetry().is_err());
        }
        let telemetry = queues.report(QueueKind::Telemetry);
        assert_eq!(
            (telemetry.capacity, telemetry.generation),
            (2, QueueGeneration(5))
        );
    }

    #[test]
    fn reliable_never_coalesces_and_lossy_telemetry_is_explicit() {
        let mut queues = ProtocolQueues::prepare(config()).expect("prepare");
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
        let mut queues = ProtocolQueues::prepare(config()).expect("prepare");
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
        let mut queues = ProtocolQueues::prepare(config()).expect("prepare");
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
