//! Optional ownership boundary for admitted automation.
//!
//! This module is deliberately separate from [`ProtocolController`].  It is a small, prepared
//! handoff service: the control owner retains admission rows and the render owner only sees
//! copies published through bounded SPSC endpoints.

#![allow(missing_docs)]

use core::num::NonZeroUsize;

use engine::realtime::{
    Consumer, Producer, QueueGeneration, bounded_spsc, bounded_spsc_retained_payload,
};

use crate::{
    AutomationBatchSlot, AutomationCancellationReason, AutomationEnqueueError, ParameterHandle,
    ProtocolQueueConfig, ProtocolQueues, QueueReport, ReliableEventReservations, ReliableSlot,
    RequestId, SampleTime, SessionRevision,
};

const MAX_RECORDS: usize = crate::AUTOMATION_BATCH_RECORDS;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeliveryTicket {
    pub generation: u64,
    pub slot: u16,
    pub serial: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeliveryError {
    Full,
    Empty,
    StaleTicket,
    Unsupported,
    InvalidPrefix,
    AlreadyPending,
    CancellationPending,
    SequenceOverflow,
    ReliableFull(QueueReport),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HandoffResult {
    HandedOff(DeliveryTicket),
    PendingUnsupported,
    Empty,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PreparedDeliveryCapabilities {
    handles: [ParameterHandle; MAX_RECORDS],
    kinds: [crate::AutomationKind; MAX_RECORDS],
    len: u16,
}

impl PreparedDeliveryCapabilities {
    pub fn new(handles: &[ParameterHandle]) -> Option<Self> {
        if handles.is_empty() || handles.len() > MAX_RECORDS {
            return None;
        }
        let mut value = Self {
            handles: [ParameterHandle(0); MAX_RECORDS],
            kinds: [crate::AutomationKind::Point; MAX_RECORDS],
            len: handles.len() as u16,
        };
        value.handles[..handles.len()].copy_from_slice(handles);
        Some(value)
    }

    pub fn new_exact(pairs: &[(ParameterHandle, crate::AutomationKind)]) -> Option<Self> {
        if pairs.is_empty() || pairs.len() > MAX_RECORDS {
            return None;
        }
        let mut value = Self {
            handles: [ParameterHandle(0); MAX_RECORDS],
            kinds: [crate::AutomationKind::Point; MAX_RECORDS],
            len: pairs.len() as u16,
        };
        for (index, (handle, kind)) in pairs.iter().copied().enumerate() {
            value.handles[index] = handle;
            value.kinds[index] = kind;
        }
        Some(value)
    }

    fn supports(&self, batch: &AutomationBatchSlot) -> bool {
        batch.as_slice().iter().all(|record| {
            (0..usize::from(self.len)).any(|index| {
                self.handles[index] == record.handle && self.kinds[index] == record.kind
            })
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct DeliveryMessage {
    ticket: DeliveryTicket,
    batch: AutomationBatchSlot,
}

#[derive(Clone, Copy, Debug)]
enum BoundaryMessage {
    Cancel { generation: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PendingAutomation<'a> {
    pub ticket: DeliveryTicket,
    pub records: &'a [crate::AutomationRecord],
    pub applied_prefix: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerminalAutomation {
    pub ticket: DeliveryTicket,
    pub request_id: RequestId,
    pub revision: SessionRevision,
    pub applied_prefix: u16,
    pub record_count: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CancelComplete {
    pub generation: u64,
    pub canceled_events: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeliveryResourceReport {
    pub retained_payload_bytes: u64,
    pub largest_allocation_bytes: u64,
}

#[derive(Clone, Copy, Debug)]
struct LedgerEntry {
    ticket: DeliveryTicket,
    batch: AutomationBatchSlot,
    applied_prefix: u16,
    handed_off: bool,
}

#[derive(Debug)]
struct CancelState {
    generation: u64,
    reason: AutomationCancellationReason,
    revision: SessionRevision,
    effective_sample: Option<SampleTime>,
    reservations: Option<ReliableEventReservations>,
    barrier_sent: bool,
    barrier_seen: bool,
}

pub struct PreparedAutomationDelivery;

impl PreparedAutomationDelivery {
    pub fn resource_report_for_config(
        config: ProtocolQueueConfig,
    ) -> Result<DeliveryResourceReport, crate::ProtocolQueueError> {
        let base = ProtocolQueues::resource_report_for_config(config)?;
        let mut total = base.retained_payload_bytes;
        let mut largest = base.largest_allocation_bytes;
        let mut add = |bytes: usize| -> Result<(), crate::ProtocolQueueError> {
            let bytes =
                u64::try_from(bytes).map_err(|_| crate::ProtocolQueueError::CapacityOverflow)?;
            total = total
                .checked_add(bytes)
                .ok_or(crate::ProtocolQueueError::CapacityOverflow)?;
            largest = largest.max(bytes);
            Ok(())
        };
        for payload in [
            bounded_spsc_retained_payload::<DeliveryMessage>(config.automation_batch_slots)?,
            bounded_spsc_retained_payload::<BoundaryMessage>(NonZeroUsize::new(1).unwrap())?,
            bounded_spsc_retained_payload::<u8>(NonZeroUsize::new(1).unwrap())?,
            bounded_spsc_retained_payload::<TerminalAutomation>(config.automation_batch_slots)?,
        ] {
            add(payload.ring_header_bytes)?;
            add(payload.slot_payload_bytes)?;
        }
        add(core::alloc::Layout::array::<Option<LedgerEntry>>(
            config.automation_batch_slots.get(),
        )
        .map_err(|_| crate::ProtocolQueueError::CapacityOverflow)?
        .size())?;
        Ok(DeliveryResourceReport {
            retained_payload_bytes: total,
            largest_allocation_bytes: largest,
        })
    }
    pub fn prepare(
        config: ProtocolQueueConfig,
        initial_reliable_event_sequence: u64,
    ) -> Result<(AutomationDeliveryControl, AutomationDeliveryRender), crate::ProtocolQueueError>
    {
        if initial_reliable_event_sequence == 0 {
            return Err(crate::ProtocolQueueError::CapacityOverflow);
        }
        let _report = Self::resource_report_for_config(config)?;
        let queues = ProtocolQueues::prepare(config)?;
        let capacity = config.automation_batch_slots;
        let (producer, consumer) =
            bounded_spsc(capacity, QueueGeneration(initial_reliable_event_sequence))?;
        let (barrier_producer, barrier_consumer) = bounded_spsc(
            NonZeroUsize::new(1).unwrap(),
            QueueGeneration(initial_reliable_event_sequence),
        )?;
        let (ack_producer, ack_consumer) = bounded_spsc(
            NonZeroUsize::new(1).unwrap(),
            QueueGeneration(initial_reliable_event_sequence),
        )?;
        let (terminal_producer, terminal_consumer) =
            bounded_spsc(capacity, QueueGeneration(initial_reliable_event_sequence))?;
        let slots = capacity.get();
        Ok((
            AutomationDeliveryControl {
                queues,
                producer,
                barrier_producer,
                ack_consumer,
                terminal_consumer,
                ledger: vec![None; slots].into_boxed_slice(),
                next_slot: 0,
                next_serial: 1,
                next_generation: initial_reliable_event_sequence,
                outstanding: 0,
                sequence: initial_reliable_event_sequence,
                cancel: None,
                staged: None,
            },
            AutomationDeliveryRender {
                consumer,
                barrier_consumer,
                ack_producer,
                terminal_producer,
                pending: None,
                pending_prefix: 0,
                boundary_limit: config.automation_batch_slots.get(),
            },
        ))
    }
}

pub struct AutomationDeliveryControl {
    queues: ProtocolQueues,
    producer: Producer<DeliveryMessage>,
    barrier_producer: Producer<BoundaryMessage>,
    ack_consumer: Consumer<u8>,
    terminal_consumer: Consumer<TerminalAutomation>,
    ledger: Box<[Option<LedgerEntry>]>,
    next_slot: usize,
    next_serial: u64,
    next_generation: u64,
    outstanding: usize,
    sequence: u64,
    cancel: Option<CancelState>,
    staged: Option<(DeliveryTicket, AutomationBatchSlot)>,
}

impl AutomationDeliveryControl {
    pub fn try_admit(
        &mut self,
        current_sample: SampleTime,
        batch: AutomationBatchSlot,
    ) -> Result<(), AutomationEnqueueError> {
        if self.cancel.is_some() || self.outstanding == self.ledger.len() {
            return Err(AutomationEnqueueError::Full {
                batch,
                report: self.queues.report(crate::QueueKind::Automation),
            });
        }
        self.queues.try_enqueue_automation(current_sample, batch)?;
        self.outstanding += 1;
        Ok(())
    }

    pub fn try_handoff_next(
        &mut self,
        capabilities: &PreparedDeliveryCapabilities,
    ) -> Result<HandoffResult, DeliveryError> {
        if self.cancel.is_some() {
            return Err(DeliveryError::CancellationPending);
        }
        let slot = self.next_slot;
        let staged = self.staged.take();
        let (ticket, batch) = match staged.or_else(|| {
            self.queues
                .try_dequeue_automation_retaining_admission()
                .ok()
                .and_then(|batch| {
                    self.put_ledger(batch, false)
                        .ok()
                        .map(|ticket| (ticket, batch))
                })
        }) {
            Some(value) => value,
            None => return Ok(HandoffResult::Empty),
        };
        if !capabilities.supports(&batch) {
            // FIFO policy: retain the batch in the first free ledger entry as a staged head.
            self.staged = Some((ticket, batch));
            return Ok(HandoffResult::PendingUnsupported);
        }
        if let Some(entry) = self.ledger[usize::from(ticket.slot)].as_mut() {
            entry.handed_off = true;
        }
        let message = DeliveryMessage { ticket, batch };
        if self.producer.try_push(message).is_err() {
            self.ledger[usize::from(ticket.slot)] = None;
            self.outstanding = self.outstanding.saturating_sub(1);
            return Err(DeliveryError::Full);
        }
        self.next_slot = (slot + 1) % self.ledger.len();
        Ok(HandoffResult::HandedOff(ticket))
    }

    fn put_ledger(
        &mut self,
        batch: AutomationBatchSlot,
        handed_off: bool,
    ) -> Result<DeliveryTicket, DeliveryError> {
        let slot = self
            .ledger
            .iter()
            .position(Option::is_none)
            .ok_or(DeliveryError::Full)?;
        let serial = self.next_serial;
        self.next_serial = self
            .next_serial
            .checked_add(1)
            .ok_or(DeliveryError::SequenceOverflow)?;
        let ticket = DeliveryTicket {
            generation: self.next_generation,
            slot: slot as u16,
            serial,
        };
        self.ledger[slot] = Some(LedgerEntry {
            ticket,
            batch,
            applied_prefix: 0,
            handed_off,
        });
        Ok(ticket)
    }

    pub fn collect_terminal(&mut self) -> Result<TerminalAutomation, DeliveryError> {
        let terminal = self
            .terminal_consumer
            .try_pop()
            .map_err(|_| DeliveryError::Empty)?;
        let index = usize::from(terminal.ticket.slot);
        let entry = self
            .ledger
            .get(index)
            .and_then(Option::as_ref)
            .ok_or(DeliveryError::StaleTicket)?;
        if entry.ticket != terminal.ticket || terminal.applied_prefix > terminal.record_count {
            return Err(DeliveryError::StaleTicket);
        }
        self.queues.release_automation_admission(&entry.batch);
        self.ledger[index] = None;
        self.outstanding = self.outstanding.saturating_sub(1);
        Ok(terminal)
    }

    pub fn begin_cancel(
        &mut self,
        reason: AutomationCancellationReason,
        event_revision: SessionRevision,
        effective_sample: Option<SampleTime>,
    ) -> Result<(), DeliveryError> {
        if self.cancel.is_some() {
            return Err(DeliveryError::CancellationPending);
        }
        while let Ok(batch) = self.queues.try_dequeue_automation_retaining_admission() {
            self.put_ledger(batch, false)?;
        }
        let count = self.ledger.iter().filter(|entry| entry.is_some()).count();
        self.sequence
            .checked_add(count as u64)
            .ok_or(DeliveryError::SequenceOverflow)?;
        let reservations = self
            .queues
            .reserve_reliable_events(count)
            .map_err(DeliveryError::ReliableFull)?;
        self.barrier_producer
            .try_push(BoundaryMessage::Cancel {
                generation: self.next_generation,
            })
            .map_err(|_| DeliveryError::Full)?;
        self.cancel = Some(CancelState {
            generation: self.next_generation,
            reason,
            revision: event_revision,
            effective_sample,
            reservations: Some(reservations),
            barrier_sent: true,
            barrier_seen: false,
        });
        self.cancel.as_mut().unwrap().barrier_sent = true;
        Ok(())
    }

    pub fn poll_cancel_boundary(&mut self) -> Result<Option<CancelComplete>, DeliveryError> {
        let Some(cancel) = self.cancel.as_mut() else {
            return Ok(None);
        };
        if !cancel.barrier_seen {
            if self.ack_consumer.try_pop().is_err() {
                return Ok(None);
            }
            cancel.barrier_seen = true;
        }
        let mut published = 0u16;
        if let Some(reservations) = cancel.reservations.as_mut() {
            for entry in self.ledger.iter().flatten() {
                let remaining = entry.batch.len.saturating_sub(entry.applied_prefix);
                if remaining != 0 {
                    let event = ReliableSlot::automation_canceled(
                        cancel.revision,
                        self.sequence,
                        entry.batch.request_id,
                        remaining,
                        cancel.reason,
                        self.queues
                            .report(crate::QueueKind::Automation)
                            .generation
                            .0,
                        cancel.effective_sample,
                    );
                    self.queues
                        .commit_reserved_reliable_event(reservations, event);
                    self.sequence = self.sequence.saturating_add(1);
                    published = published.saturating_add(1);
                }
            }
        }
        let generation = cancel.generation;
        for slot in &mut self.ledger {
            if let Some(entry) = slot.take() {
                self.queues.release_automation_admission(&entry.batch);
            }
        }
        self.staged = None;
        self.outstanding = 0;
        self.cancel = None;
        self.next_generation = self
            .next_generation
            .checked_add(1)
            .ok_or(DeliveryError::SequenceOverflow)?;
        Ok(Some(CancelComplete {
            generation,
            canceled_events: published,
        }))
    }

    pub fn queues(&self) -> &ProtocolQueues {
        &self.queues
    }
    pub fn queues_mut(&mut self) -> &mut ProtocolQueues {
        &mut self.queues
    }
}

pub struct AutomationDeliveryRender {
    consumer: Consumer<DeliveryMessage>,
    barrier_consumer: Consumer<BoundaryMessage>,
    ack_producer: Producer<u8>,
    terminal_producer: Producer<TerminalAutomation>,
    pending: Option<DeliveryMessage>,
    pending_prefix: u16,
    boundary_limit: usize,
}

impl AutomationDeliveryRender {
    pub fn begin_boundary(&mut self, _first_sample: SampleTime) -> Option<PendingAutomation<'_>> {
        for _ in 0..self.boundary_limit {
            if let Ok(command) = self.barrier_consumer.try_pop() {
                match command {
                    BoundaryMessage::Cancel { .. } => {
                        if let Some(message) = self.pending.take() {
                            let _ = self.terminal_producer.try_push(TerminalAutomation {
                                ticket: message.ticket,
                                request_id: message.batch.request_id,
                                revision: message.batch.revision,
                                applied_prefix: self.pending_prefix,
                                record_count: message.batch.len,
                            });
                        }
                        let _ = self.ack_producer.try_push(1);
                    }
                }
                // Ack is represented by clearing the barrier channel; the control observes it
                // through this endpoint's explicit acknowledgement method below.
            }
            if self.pending.is_none() {
                self.pending = self.consumer.try_pop().ok();
                self.pending_prefix = 0;
            }
            if self.pending.is_some() {
                break;
            }
        }
        self.pending.as_ref().map(|message| PendingAutomation {
            ticket: message.ticket,
            records: message.batch.as_slice(),
            applied_prefix: self.pending_prefix,
        })
    }

    pub fn pending(&self, ticket: DeliveryTicket) -> Result<PendingAutomation<'_>, DeliveryError> {
        let message = self.pending.as_ref().ok_or(DeliveryError::Empty)?;
        if message.ticket != ticket {
            return Err(DeliveryError::StaleTicket);
        }
        Ok(PendingAutomation {
            ticket,
            records: message.batch.as_slice(),
            applied_prefix: self.pending_prefix,
        })
    }

    pub fn mark_applied(
        &mut self,
        ticket: DeliveryTicket,
        new_prefix: u16,
    ) -> Result<(), DeliveryError> {
        let message = self.pending.as_ref().ok_or(DeliveryError::Empty)?;
        if message.ticket != ticket
            || new_prefix < self.pending_prefix
            || usize::from(new_prefix) > message.batch.as_slice().len()
        {
            return Err(DeliveryError::InvalidPrefix);
        }
        self.pending_prefix = new_prefix;
        Ok(())
    }

    pub fn finish_applied(
        &mut self,
        ticket: DeliveryTicket,
        applied_prefix: u16,
    ) -> Result<(), DeliveryError> {
        let message = self.pending.take().ok_or(DeliveryError::Empty)?;
        if message.ticket != ticket
            || applied_prefix != self.pending_prefix
            || usize::from(applied_prefix) != message.batch.as_slice().len()
        {
            return Err(DeliveryError::InvalidPrefix);
        }
        self.terminal_producer
            .try_push(TerminalAutomation {
                ticket,
                request_id: message.batch.request_id,
                revision: message.batch.revision,
                applied_prefix,
                record_count: message.batch.len,
            })
            .map_err(|_| DeliveryError::Full)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::num::NonZeroUsize;

    fn config(slots: usize) -> ProtocolQueueConfig {
        ProtocolQueueConfig {
            control_command_slots: NonZeroUsize::new(2).unwrap(),
            control_command_bytes: NonZeroUsize::new(256).unwrap(),
            automation_batch_slots: NonZeroUsize::new(slots).unwrap(),
            reliable_response_slots: NonZeroUsize::new(2).unwrap(),
            reliable_event_slots: NonZeroUsize::new(4).unwrap(),
            telemetry_slots: NonZeroUsize::new(2).unwrap(),
            per_block_automation_density: NonZeroUsize::new(256).unwrap(),
            quantum_frames: NonZeroUsize::new(64).unwrap(),
        }
    }

    fn batch(id: u64, handle: u32) -> AutomationBatchSlot {
        AutomationBatchSlot::new(
            SessionRevision(1),
            RequestId::new(id).unwrap(),
            &[crate::AutomationRecord {
                kind: crate::AutomationKind::Point,
                handle: ParameterHandle(handle),
                start: SampleTime(id),
                end: SampleTime(id),
                start_value: 1.0,
                end_value: 1.0,
            }],
        )
        .unwrap()
    }

    #[test]
    fn retained_admission_survives_handoff_until_terminal_consumption() {
        let (mut control, mut render) = PreparedAutomationDelivery::prepare(config(1), 1).unwrap();
        let capabilities = PreparedDeliveryCapabilities::new_exact(&[(
            ParameterHandle(7),
            crate::AutomationKind::Point,
        )])
        .unwrap();
        control.try_admit(SampleTime(0), batch(1, 7)).unwrap();
        let ticket = match control.try_handoff_next(&capabilities).unwrap() {
            HandoffResult::HandedOff(ticket) => ticket,
            other => panic!("{other:?}"),
        };
        assert!(control.try_admit(SampleTime(2), batch(2, 7)).is_err());
        render.begin_boundary(SampleTime(0));
        render.mark_applied(ticket, 1).unwrap();
        render.finish_applied(ticket, 1).unwrap();
        control.collect_terminal().unwrap();
        control.try_admit(SampleTime(2), batch(2, 7)).unwrap();
    }

    #[test]
    fn unsupported_head_blocks_fifo_and_cancel_ack_publishes_event() {
        let (mut control, mut render) = PreparedAutomationDelivery::prepare(config(2), 1).unwrap();
        let unsupported = PreparedDeliveryCapabilities::new_exact(&[(
            ParameterHandle(8),
            crate::AutomationKind::Point,
        )])
        .unwrap();
        control.try_admit(SampleTime(0), batch(1, 7)).unwrap();
        control.try_admit(SampleTime(2), batch(2, 8)).unwrap();
        assert_eq!(
            control.try_handoff_next(&unsupported).unwrap(),
            HandoffResult::PendingUnsupported
        );
        assert_eq!(
            control.try_handoff_next(&unsupported).unwrap(),
            HandoffResult::PendingUnsupported
        );
        control
            .begin_cancel(
                AutomationCancellationReason::EndpointShutdown,
                SessionRevision(2),
                None,
            )
            .unwrap();
        render.begin_boundary(SampleTime(0));
        assert_eq!(
            control
                .poll_cancel_boundary()
                .unwrap()
                .unwrap()
                .canceled_events,
            2
        );
        assert!(control.queues_mut().try_dequeue_event().is_ok());
    }
}
