//! Optional ownership boundary for admitted automation.
//!
//! This module is deliberately separate from [`ProtocolController`].  It is a small, prepared
//! handoff service: the control owner retains admission rows and the render owner only sees
//! copies published through bounded SPSC endpoints.

#![allow(missing_docs)]

use core::num::NonZeroUsize;

use engine::realtime::{
    Consumer, Producer, QueueEmpty, QueueGeneration, bounded_spsc, bounded_spsc_retained_payload,
};

use crate::{
    AutomationBatchSlot, AutomationCancellationReason, AutomationEnqueueError, ParameterHandle,
    ProtocolQueueConfig, ProtocolQueues, QueueKind, QueueReport, ReliableEventReservations,
    ReliableSlot, RequestId, SampleTime, SessionRevision,
};

const MAX_RECORDS: usize = crate::AUTOMATION_BATCH_RECORDS;

pub type DeliveryTicket = CoreTicket;

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
enum BoundaryMessage {
    Cancel { token: CancelToken, frontier: u64 },
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
    pub effective_sample: SampleTime,
    pub canceled_events: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeliveryResourceReport {
    pub retained_payload_bytes: u64,
    pub largest_allocation_bytes: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoreTicket {
    pub generation: u64,
    /// Exact prepared ledger index; valid capacities are never narrowed.
    pub slot: usize,
    pub serial: u64,
}

#[derive(Clone, Copy, Debug)]
struct CoreMessage<P: Copy + Send + 'static> {
    ticket: CoreTicket,
    payload: P,
}

#[derive(Clone, Copy, Debug)]
struct CoreTerminal {
    ticket: CoreTicket,
    applied_prefix: u16,
    record_count: u16,
}

pub struct PreparedDelivery<P: Copy + Send + 'static> {
    _marker: core::marker::PhantomData<P>,
}

pub struct DeliveryCoreControl<P: Copy + Send + 'static> {
    producer: Producer<CoreMessage<P>>,
    terminal_consumer: Consumer<CoreTerminal>,
    entries: Box<[Option<(CoreTicket, P, bool)>]>,
    serial: u64,
    generation: u64,
    terminal_head: Option<CoreTerminal>,
}

pub struct DeliveryCoreRender<P: Copy + Send + 'static> {
    consumer: Consumer<CoreMessage<P>>,
    terminal_producer: Producer<CoreTerminal>,
    pending: Option<CoreMessage<P>>,
}

impl<P: Copy + Send + 'static> PreparedDelivery<P> {
    pub fn prepare(
        capacity: NonZeroUsize,
    ) -> Result<(DeliveryCoreControl<P>, DeliveryCoreRender<P>), crate::ProtocolQueueError> {
        let (producer, consumer) = bounded_spsc(capacity, QueueGeneration(1))?;
        let (terminal_producer, terminal_consumer) = bounded_spsc(capacity, QueueGeneration(2))?;
        Ok((
            DeliveryCoreControl {
                producer,
                terminal_consumer,
                entries: vec![None; capacity.get()].into_boxed_slice(),
                serial: 1,
                generation: 1,
                terminal_head: None,
            },
            DeliveryCoreRender {
                consumer,
                terminal_producer,
                pending: None,
            },
        ))
    }
}

impl<P: Copy + Send + 'static> DeliveryCoreControl<P> {
    pub fn try_publish(&mut self, payload: P) -> Result<CoreTicket, DeliveryError> {
        let ticket = self.reserve_payload(payload)?;
        if let Err(error) = self.publish_reserved(ticket) {
            self.entries[ticket.slot] = None;
            return Err(error);
        }
        Ok(ticket)
    }

    fn reserve_payload(&mut self, payload: P) -> Result<CoreTicket, DeliveryError> {
        let slot = self
            .entries
            .iter()
            .position(Option::is_none)
            .ok_or(DeliveryError::Full)?;
        let next_serial = self
            .serial
            .checked_add(1)
            .ok_or(DeliveryError::SequenceOverflow)?;
        let ticket = CoreTicket {
            generation: self.generation,
            slot,
            serial: self.serial,
        };
        self.entries[slot] = Some((ticket, payload, false));
        self.serial = next_serial;
        Ok(ticket)
    }

    fn publish_reserved(&mut self, ticket: CoreTicket) -> Result<(), DeliveryError> {
        let (owned, payload, published) = self
            .entries
            .get(ticket.slot)
            .and_then(|entry| *entry)
            .filter(|entry| entry.0 == ticket)
            .ok_or(DeliveryError::StaleTicket)?;
        if published {
            return Err(DeliveryError::StaleTicket);
        }
        self.producer
            .try_push(CoreMessage { ticket, payload })
            .map_err(|_| DeliveryError::Full)?;
        self.entries[ticket.slot] = Some((owned, payload, true));
        Ok(())
    }

    pub fn collect(&mut self, ticket: CoreTicket) -> Result<P, DeliveryError> {
        let entry = self
            .entries
            .get(ticket.slot)
            .and_then(|entry| *entry)
            .filter(|entry| entry.0 == ticket)
            .ok_or(DeliveryError::StaleTicket)?;
        let terminal = if let Some(terminal) = self.terminal_head {
            terminal
        } else {
            let terminal = self
                .terminal_consumer
                .try_pop()
                .map_err(|_| DeliveryError::Empty)?;
            self.terminal_head = Some(terminal);
            terminal
        };
        if terminal.ticket != ticket {
            return Err(DeliveryError::StaleTicket);
        }
        self.terminal_head = None;
        self.entries[ticket.slot] = None;
        Ok(entry.1)
    }

    fn poll_terminal(&mut self) -> Result<CoreTerminal, DeliveryError> {
        if let Some(terminal) = self.terminal_head.take() {
            return Ok(terminal);
        }
        self.terminal_consumer
            .try_pop()
            .map_err(|_| DeliveryError::Empty)
    }

    fn payload(&self, ticket: CoreTicket) -> Result<P, DeliveryError> {
        self.entries
            .get(ticket.slot)
            .and_then(|entry| *entry)
            .filter(|entry| entry.0 == ticket)
            .map(|entry| entry.1)
            .ok_or(DeliveryError::StaleTicket)
    }

    fn release(&mut self, ticket: CoreTicket) -> Result<P, DeliveryError> {
        let payload = self.payload(ticket)?;
        self.entries[ticket.slot] = None;
        Ok(payload)
    }

    fn outstanding(&self) -> usize {
        self.entries.iter().filter(|entry| entry.is_some()).count()
    }
}

impl<P: Copy + Send + 'static> DeliveryCoreRender<P> {
    pub fn begin(&mut self) -> Result<(CoreTicket, P), DeliveryError> {
        if self.pending.is_some() {
            return Err(DeliveryError::AlreadyPending);
        }
        let message = self.consumer.try_pop().map_err(|_| DeliveryError::Empty)?;
        let result = (message.ticket, message.payload);
        self.pending = Some(message);
        Ok(result)
    }

    pub fn finish(&mut self, ticket: CoreTicket) -> Result<(), DeliveryError> {
        let message = self.pending.as_ref().ok_or(DeliveryError::Empty)?;
        if message.ticket != ticket {
            return Err(DeliveryError::StaleTicket);
        }
        self.terminal_producer
            .try_push(CoreTerminal {
                ticket,
                applied_prefix: 0,
                record_count: 0,
            })
            .map_err(|_| DeliveryError::Full)?;
        self.pending = None;
        Ok(())
    }
}

impl<P: Copy + Send + 'static> DeliveryCoreRender<P> {
    fn finish_with_progress(
        &mut self,
        ticket: CoreTicket,
        applied_prefix: u16,
        record_count: u16,
    ) -> Result<(), DeliveryError> {
        let message = self.pending.as_ref().ok_or(DeliveryError::Empty)?;
        if message.ticket != ticket || applied_prefix > record_count {
            return Err(DeliveryError::InvalidPrefix);
        }
        self.terminal_producer
            .try_push(CoreTerminal {
                ticket,
                applied_prefix,
                record_count,
            })
            .map_err(|_| DeliveryError::Full)?;
        self.pending = None;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
struct AdmissionOwner {
    ticket: DeliveryTicket,
    order: u64,
    applied_prefix: u16,
}

#[derive(Debug)]
struct CancelState {
    generation: u64,
    reason: AutomationCancellationReason,
    revision: SessionRevision,
    effective_sample: Option<SampleTime>,
    reservations: Option<ReliableEventReservations>,
    barrier_seen: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CancelToken {
    generation: u64,
}

#[derive(Clone, Copy, Debug)]
struct CancelAck {
    token: CancelToken,
    effective_sample: SampleTime,
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
            bounded_spsc_retained_payload::<CoreMessage<AutomationBatchSlot>>(
                config.automation_batch_slots,
            )?,
            bounded_spsc_retained_payload::<CoreTerminal>(config.automation_batch_slots)?,
            bounded_spsc_retained_payload::<BoundaryMessage>(NonZeroUsize::new(1).unwrap())?,
            bounded_spsc_retained_payload::<CancelAck>(NonZeroUsize::new(1).unwrap())?,
        ] {
            add(payload.ring_header_bytes)?;
            add(payload.slot_payload_bytes)?;
        }
        add(
            core::alloc::Layout::array::<Option<(CoreTicket, AutomationBatchSlot, bool)>>(
                config.automation_batch_slots.get(),
            )
            .map_err(|_| crate::ProtocolQueueError::CapacityOverflow)?
            .size(),
        )?;
        add(core::alloc::Layout::array::<Option<AdmissionOwner>>(
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
        let _ = Self::resource_report_for_config(config)?;
        let queues = ProtocolQueues::prepare(config)?;
        let (mut core, render_core) =
            PreparedDelivery::<AutomationBatchSlot>::prepare(config.automation_batch_slots)?;
        core.generation = 1;
        let one = NonZeroUsize::new(1).unwrap();
        let (barrier_producer, barrier_consumer) = bounded_spsc(one, QueueGeneration(1))?;
        let (ack_producer, ack_consumer) = bounded_spsc(one, QueueGeneration(1))?;
        Ok((
            AutomationDeliveryControl {
                queues,
                core,
                owners: vec![None; config.automation_batch_slots.get()].into_boxed_slice(),
                next_order: 1,
                next_generation: 1,
                sequence: initial_reliable_event_sequence,
                barrier_producer,
                ack_consumer,
                cancel: None,
                staged: None,
            },
            AutomationDeliveryRender {
                core: render_core,
                barrier_consumer,
                ack_producer,
                deferred_cancel: None,
                pending_prefix: 0,
                boundary_limit: config.automation_batch_slots.get() + 1,
            },
        ))
    }
}

pub struct AutomationDeliveryControl {
    queues: ProtocolQueues,
    core: DeliveryCoreControl<AutomationBatchSlot>,
    owners: Box<[Option<AdmissionOwner>]>,
    next_order: u64,
    next_generation: u64,
    sequence: u64,
    barrier_producer: Producer<BoundaryMessage>,
    ack_consumer: Consumer<CancelAck>,
    cancel: Option<CancelState>,
    staged: Option<DeliveryTicket>,
}

impl AutomationDeliveryControl {
    #[allow(clippy::result_large_err)]
    pub fn try_admit(
        &mut self,
        current_sample: SampleTime,
        batch: AutomationBatchSlot,
    ) -> Result<(), AutomationEnqueueError> {
        if self.cancel.is_some() || self.core.outstanding() == self.owners.len() {
            return Err(AutomationEnqueueError::Full {
                batch,
                report: self.queues.report(QueueKind::Automation),
            });
        }
        self.queues.try_enqueue_automation(current_sample, batch)
    }

    fn own(&mut self, batch: AutomationBatchSlot) -> Result<DeliveryTicket, DeliveryError> {
        let order = self.next_order;
        let next_order = order
            .checked_add(1)
            .ok_or(DeliveryError::SequenceOverflow)?;
        let ticket = self.core.reserve_payload(batch)?;
        self.owners[ticket.slot] = Some(AdmissionOwner {
            ticket,
            order,
            applied_prefix: 0,
        });
        self.next_order = next_order;
        Ok(ticket)
    }

    pub fn try_handoff_next(
        &mut self,
        capabilities: &PreparedDeliveryCapabilities,
    ) -> Result<HandoffResult, DeliveryError> {
        if self.cancel.is_some() {
            return Err(DeliveryError::CancellationPending);
        }
        if let Some(ticket) = self.staged {
            let batch = self.core.payload(ticket)?;
            if !capabilities.supports(&batch) {
                return Ok(HandoffResult::PendingUnsupported);
            }
            self.core.publish_reserved(ticket)?;
            self.staged = None;
            return Ok(HandoffResult::HandedOff(ticket));
        }
        if self.core.outstanding() == self.owners.len() {
            return Err(DeliveryError::Full);
        }
        if self.core.serial == u64::MAX || self.next_order == u64::MAX {
            return Err(DeliveryError::SequenceOverflow);
        }
        let batch = match self.queues.try_dequeue_automation_retaining_admission() {
            Ok(v) => v,
            Err(_) => return Ok(HandoffResult::Empty),
        };
        let supported = capabilities.supports(&batch);
        let ticket = self.own(batch)?;
        if !supported {
            self.staged = Some(ticket);
            return Ok(HandoffResult::PendingUnsupported);
        }
        self.core.publish_reserved(ticket)?;
        Ok(HandoffResult::HandedOff(ticket))
    }

    fn reconcile(&mut self) -> Result<(), DeliveryError> {
        loop {
            match self.core.poll_terminal() {
                Ok(terminal) => {
                    let owner = self
                        .owners
                        .get_mut(terminal.ticket.slot)
                        .and_then(Option::as_mut)
                        .ok_or(DeliveryError::StaleTicket)?;
                    if owner.ticket != terminal.ticket
                        || terminal.applied_prefix < owner.applied_prefix
                        || terminal.applied_prefix > terminal.record_count
                    {
                        self.core.terminal_head = Some(terminal);
                        return Err(DeliveryError::InvalidPrefix);
                    }
                    owner.applied_prefix = terminal.applied_prefix;
                }
                Err(DeliveryError::Empty) => return Ok(()),
                Err(error) => return Err(error),
            }
        }
    }

    pub fn collect_terminal(
        &mut self,
        ticket: DeliveryTicket,
    ) -> Result<TerminalAutomation, DeliveryError> {
        if self.cancel.is_some() {
            return Err(DeliveryError::CancellationPending);
        }
        self.reconcile()?;
        let owner = self
            .owners
            .get(ticket.slot)
            .and_then(|v| *v)
            .filter(|o| o.ticket == ticket)
            .ok_or(DeliveryError::StaleTicket)?;
        let batch = self.core.payload(ticket)?;
        if owner.applied_prefix != batch.len {
            return Err(DeliveryError::Empty);
        }
        self.release(ticket, batch);
        Ok(TerminalAutomation {
            ticket,
            request_id: batch.request_id,
            revision: batch.revision,
            applied_prefix: owner.applied_prefix,
            record_count: batch.len,
        })
    }

    fn release(&mut self, ticket: DeliveryTicket, batch: AutomationBatchSlot) {
        let owner = self.owners[ticket.slot]
            .take()
            .expect("ticket owns admission");
        debug_assert_eq!(owner.ticket, ticket);
        self.core.release(ticket).expect("same core owns payload");
        self.queues.release_automation_admission(&batch);
    }

    pub fn begin_cancel(
        &mut self,
        reason: AutomationCancellationReason,
        revision: SessionRevision,
    ) -> Result<CancelToken, DeliveryError> {
        if self.cancel.is_some() {
            return Err(DeliveryError::CancellationPending);
        }
        self.reconcile()?;
        let queued = usize::try_from(self.queues.report(QueueKind::Automation).occupancy)
            .unwrap_or(usize::MAX);
        let total = self
            .core
            .outstanding()
            .checked_add(queued)
            .ok_or(DeliveryError::SequenceOverflow)?;
        let total_u64 = u64::try_from(total).map_err(|_| DeliveryError::SequenceOverflow)?;
        self.sequence
            .checked_add(total_u64)
            .ok_or(DeliveryError::SequenceOverflow)?;
        self.next_order
            .checked_add(u64::try_from(queued).map_err(|_| DeliveryError::SequenceOverflow)?)
            .ok_or(DeliveryError::SequenceOverflow)?;
        let next_generation = self
            .next_generation
            .checked_add(1)
            .ok_or(DeliveryError::SequenceOverflow)?;
        let reservations = self
            .queues
            .reserve_reliable_events(total)
            .map_err(DeliveryError::ReliableFull)?;
        let token = CancelToken {
            generation: self.next_generation,
        };
        let frontier = self
            .core
            .serial
            .checked_add(u64::try_from(queued).map_err(|_| DeliveryError::SequenceOverflow)?)
            .and_then(|next| next.checked_sub(1))
            .ok_or(DeliveryError::SequenceOverflow)?;
        if self
            .barrier_producer
            .try_push(BoundaryMessage::Cancel { token, frontier })
            .is_err()
        {
            self.queues.release_reliable_events(reservations);
            return Err(DeliveryError::Full);
        }
        while let Ok(batch) = self.queues.try_dequeue_automation_retaining_admission() {
            // Queue count, core credit and both identity counters were prevalidated.
            self.own(batch)
                .expect("prevalidated cancellation ownership");
        }
        self.cancel = Some(CancelState {
            generation: token.generation,
            reason,
            revision,
            effective_sample: None,
            reservations: Some(reservations),
            barrier_seen: false,
        });
        self.next_generation = next_generation;
        Ok(token)
    }

    pub fn poll_cancel_boundary(
        &mut self,
        token: CancelToken,
    ) -> Result<Option<CancelComplete>, DeliveryError> {
        let state = self.cancel.as_mut().ok_or(DeliveryError::StaleTicket)?;
        if state.generation != token.generation {
            return Err(DeliveryError::StaleTicket);
        }
        if !state.barrier_seen {
            let ack = match self.ack_consumer.try_pop() {
                Ok(v) => v,
                Err(_) => return Ok(None),
            };
            if ack.token != token {
                return Err(DeliveryError::StaleTicket);
            }
            state.effective_sample = Some(ack.effective_sample);
            state.barrier_seen = true;
        }
        self.reconcile()?;
        if self
            .owners
            .iter()
            .flatten()
            .any(|o| self.core.payload(o.ticket).is_err())
        {
            return Err(DeliveryError::StaleTicket);
        }
        let mut order: Vec<_> = self.owners.iter().flatten().copied().collect();
        order.sort_unstable_by_key(|owner| owner.order);
        let sample = self.cancel.as_ref().unwrap().effective_sample.unwrap();
        let mut published = 0u16;
        for owner in order {
            let batch = self.core.payload(owner.ticket)?;
            let remaining = batch.len.saturating_sub(owner.applied_prefix);
            if remaining != 0 {
                let state = self.cancel.as_mut().unwrap();
                let event = ReliableSlot::automation_canceled(
                    state.revision,
                    self.sequence,
                    batch.request_id,
                    remaining,
                    state.reason,
                    self.queues.report(QueueKind::Automation).generation.0,
                    Some(sample),
                );
                self.queues
                    .commit_reserved_reliable_event(state.reservations.as_mut().unwrap(), event);
                self.sequence += 1;
                published += 1;
            }
            self.release(owner.ticket, batch);
        }
        let mut state = self.cancel.take().unwrap();
        self.queues
            .release_reliable_events(state.reservations.take().unwrap());
        self.staged = None;
        self.queues.reset_automation_ordering_after_cancellation();
        Ok(Some(CancelComplete {
            generation: token.generation,
            effective_sample: sample,
            canceled_events: published,
        }))
    }

    pub fn try_dequeue_event(&mut self) -> Result<ReliableSlot, QueueEmpty> {
        self.queues.try_dequeue_event()
    }
    pub fn queues(&self) -> &ProtocolQueues {
        &self.queues
    }
    pub fn outstanding(&self) -> usize {
        self.core.outstanding()
    }
    pub fn resident_automation(&self) -> u64 {
        self.queues.report(QueueKind::Automation).occupancy
    }
}

pub struct AutomationDeliveryRender {
    core: DeliveryCoreRender<AutomationBatchSlot>,
    barrier_consumer: Consumer<BoundaryMessage>,
    ack_producer: Producer<CancelAck>,
    deferred_cancel: Option<(CancelToken, u64)>,
    pending_prefix: u16,
    boundary_limit: usize,
}

impl AutomationDeliveryRender {
    pub fn begin_boundary(&mut self, first_sample: SampleTime) -> Option<PendingAutomation<'_>> {
        if self.deferred_cancel.is_none() {
            if let Ok(BoundaryMessage::Cancel { token, frontier }) = self.barrier_consumer.try_pop()
            {
                self.deferred_cancel = Some((token, frontier));
            }
        }
        if let Some((token, frontier)) = self.deferred_cancel {
            for _ in 0..self.boundary_limit {
                if let Some(message) = self.core.pending {
                    if message.ticket.serial <= frontier {
                        let prefix = self.pending_prefix;
                        if self
                            .core
                            .finish_with_progress(message.ticket, prefix, message.payload.len)
                            .is_err()
                        {
                            return None;
                        }
                        self.pending_prefix = 0;
                        continue;
                    }
                }
                match self.core.begin() {
                    Ok((ticket, batch)) if ticket.serial <= frontier => {
                        if self
                            .core
                            .finish_with_progress(ticket, 0, batch.len)
                            .is_err()
                        {
                            return None;
                        }
                    }
                    Ok(_) => return None,
                    Err(DeliveryError::Empty) => {
                        if self
                            .ack_producer
                            .try_push(CancelAck {
                                token,
                                effective_sample: first_sample,
                            })
                            .is_ok()
                        {
                            self.deferred_cancel = None;
                        }
                        return None;
                    }
                    Err(_) => return None,
                }
            }
            return None;
        }
        if self.core.pending.is_none() {
            if self.core.begin().is_ok() {
                self.pending_prefix = 0;
            }
        }
        self.core.pending.as_ref().map(|message| PendingAutomation {
            ticket: message.ticket,
            records: message.payload.as_slice(),
            applied_prefix: self.pending_prefix,
        })
    }

    pub fn pending(&self, ticket: DeliveryTicket) -> Result<PendingAutomation<'_>, DeliveryError> {
        let message = self.core.pending.as_ref().ok_or(DeliveryError::Empty)?;
        if message.ticket != ticket {
            return Err(DeliveryError::StaleTicket);
        }
        Ok(PendingAutomation {
            ticket,
            records: message.payload.as_slice(),
            applied_prefix: self.pending_prefix,
        })
    }

    pub fn mark_applied(
        &mut self,
        ticket: DeliveryTicket,
        new_prefix: u16,
    ) -> Result<(), DeliveryError> {
        let message = self.core.pending.as_ref().ok_or(DeliveryError::Empty)?;
        if message.ticket != ticket {
            return Err(DeliveryError::StaleTicket);
        }
        if new_prefix < self.pending_prefix || new_prefix > message.payload.len {
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
        let message = self.core.pending.as_ref().ok_or(DeliveryError::Empty)?;
        if message.ticket != ticket {
            return Err(DeliveryError::StaleTicket);
        }
        if applied_prefix != self.pending_prefix || applied_prefix != message.payload.len {
            return Err(DeliveryError::InvalidPrefix);
        }
        self.core
            .finish_with_progress(ticket, applied_prefix, message.payload.len)?;
        self.pending_prefix = 0;
        Ok(())
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
        control.collect_terminal(ticket).unwrap();
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
        let token = control
            .begin_cancel(
                AutomationCancellationReason::EndpointShutdown,
                SessionRevision(2),
            )
            .unwrap();
        render.begin_boundary(SampleTime(0));
        assert_eq!(
            control
                .poll_cancel_boundary(token)
                .unwrap()
                .unwrap()
                .canceled_events,
            2
        );
        assert!(control.try_dequeue_event().is_ok());
    }

    #[test]
    fn ordered_cancel_reconciles_handed_off_and_partial_at_actual_boundary() {
        let (mut control, mut render) = PreparedAutomationDelivery::prepare(config(2), 41).unwrap();
        control.try_admit(SampleTime(0), batch(1, 7)).unwrap();
        control.try_admit(SampleTime(2), batch(2, 7)).unwrap();
        let first = match control
            .try_handoff_next(
                &PreparedDeliveryCapabilities::new_exact(&[(
                    ParameterHandle(7),
                    crate::AutomationKind::Point,
                )])
                .unwrap(),
            )
            .unwrap()
        {
            HandoffResult::HandedOff(ticket) => ticket,
            other => panic!("{other:?}"),
        };
        let pending = render.begin_boundary(SampleTime(8)).unwrap();
        assert_eq!(pending.ticket, first);
        // Zero is a valid monotonic partial prefix and leaves the whole batch cancelable.
        render.mark_applied(first, 0).unwrap();
        let token = control
            .begin_cancel(
                AutomationCancellationReason::EndpointShutdown,
                SessionRevision(3),
            )
            .unwrap();
        assert!(control.poll_cancel_boundary(token).unwrap().is_none());
        assert!(render.begin_boundary(SampleTime(1234)).is_none());
        assert_eq!(render.pending(first), Err(DeliveryError::Empty));
        let complete = control.poll_cancel_boundary(token).unwrap().unwrap();
        assert_eq!(complete.effective_sample, SampleTime(1234));
        assert_eq!(complete.canceled_events, 2);
        assert_eq!(control.outstanding(), 0);
        assert_eq!(
            control.poll_cancel_boundary(token),
            Err(DeliveryError::StaleTicket)
        );
    }

    #[test]
    fn generic_copy_core_transfers_layout_independent_payload() {
        let (mut control, mut render) =
            PreparedDelivery::<u32>::prepare(NonZeroUsize::new(1).unwrap()).unwrap();
        let ticket = control.try_publish(0xfeed_beef).unwrap();
        assert_eq!(render.begin().unwrap(), (ticket, 0xfeed_beef));
        render.finish(ticket).unwrap();
        assert_eq!(control.collect(ticket).unwrap(), 0xfeed_beef);
    }

    #[test]
    fn generic_core_rejected_identities_preserve_pending_and_terminal_owner() {
        let (mut control, mut render) =
            PreparedDelivery::<u32>::prepare(NonZeroUsize::new(2).unwrap()).unwrap();
        let first = control.try_publish(11).unwrap();
        let second = control.try_publish(22).unwrap();
        assert_eq!(render.begin().unwrap(), (first, 11));
        assert_eq!(render.finish(second), Err(DeliveryError::StaleTicket));
        render.finish(first).unwrap();
        assert_eq!(control.collect(second), Err(DeliveryError::StaleTicket));
        assert_eq!(control.collect(first), Ok(11));
        assert_eq!(render.begin().unwrap(), (second, 22));
        render.finish(second).unwrap();
        assert_eq!(control.collect(second), Ok(22));
    }

    #[test]
    fn generic_core_slot_identity_covers_capacity_above_u16() {
        let capacity = NonZeroUsize::new(usize::from(u16::MAX) + 2).unwrap();
        let (mut control, _render) = PreparedDelivery::<u8>::prepare(capacity).unwrap();
        let mut last = None;
        for _ in 0..capacity.get() {
            last = Some(control.try_publish(1).unwrap());
        }
        assert_eq!(last.unwrap().slot, usize::from(u16::MAX) + 1);
    }
}
