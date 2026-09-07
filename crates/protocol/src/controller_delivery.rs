//! Fixed-revision typed controller ownership for transient automation delivery.

use core::mem::size_of;

use crate::delivery::{AutomationDeliveryState, DeliveryContext, PreparedAutomationDelivery};
use crate::{
    AutomationCancellationReason, AutomationDeliveryRender, CancelComplete, CancelToken,
    ControllerRequest, ControllerResourceAllocationError, ControllerResponse,
    ControllerRetainedCapacity, DeliveryError, DeliveryResourceReport, DeliveryTicket,
    EventEgressError, HandoffResult, ProtocolCodec, ProtocolController, ProtocolControllerConfig,
    ProtocolQueueConfig, ProtocolQueueError, ProtocolQueues, QueueKind, QueueReport, ReplayCache,
    ReplayCacheConfig, SessionStore, TerminalAutomation,
};
use crate::{ControlProvider, PreparedDeliveryCapabilities};

/// Preparation failed before the facade became visible to its caller.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ControllerAutomationPrepareError {
    /// The configured queue/storage layout is not representable.
    ProtocolQueue(ProtocolQueueError),
    /// The bounded replay cache could not be prepared.
    ReplayCache(crate::ReplayCacheError),
    /// Eager controller-owned retained storage could not be allocated.
    ControllerResource(ControllerResourceAllocationError),
}

impl From<ProtocolQueueError> for ControllerAutomationPrepareError {
    fn from(error: ProtocolQueueError) -> Self {
        Self::ProtocolQueue(error)
    }
}

impl From<crate::ReplayCacheError> for ControllerAutomationPrepareError {
    fn from(error: crate::ReplayCacheError) -> Self {
        Self::ReplayCache(error)
    }
}

impl From<ControllerResourceAllocationError> for ControllerAutomationPrepareError {
    fn from(error: ControllerResourceAllocationError) -> Self {
        Self::ControllerResource(error)
    }
}

/// Retained resource projection for one prepared controller-owned automation facade.
///
/// `queue_and_delivery` includes exactly one [`ProtocolQueues`] allocation set and the existing
/// #460 delivery ownership allocations. The inline fields report the actual Rust inline sizes
/// separately. Session/provider storage, replay backing, and ordinary controller diagnostic,
/// telemetry, and structural-owner allocations remain under their existing authorities and are
/// excluded from this projection; this is not a whole-controller or whole-process peak claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ControllerAutomationResources {
    /// One queue set plus the existing automation delivery ownership allocations.
    pub queue_and_delivery: DeliveryResourceReport,
    /// Inline control facade size, excluding heap payloads.
    pub control_inline_bytes: usize,
    /// Inline render-half size, excluding heap payloads.
    pub render_inline_bytes: usize,
}

/// Controller-owned typed automation admission and cancellation facade.
///
/// The facade fixes the prepared session revision and provider capability set for its lifetime.
/// It exposes bounded typed processing and the actual #460 render half while retaining one
/// ordinary controller as the sole queue, replay, and reliable-event sequence authority.
pub struct ControllerAutomationDelivery<P: ControlProvider> {
    controller: ProtocolController<P>,
    delivery: AutomationDeliveryState,
    capabilities: PreparedDeliveryCapabilities,
}

impl<P: ControlProvider> ControllerAutomationDelivery<P> {
    /// Prepare fresh queues, replay storage, ordinary controller state, and delivery ownership.
    #[allow(clippy::too_many_arguments)] // Frozen #530 constructor shape.
    pub fn prepare(
        session: SessionStore,
        queues: ProtocolQueueConfig,
        provider: P,
        replay: ReplayCacheConfig,
        codec: ProtocolCodec,
        config: ProtocolControllerConfig,
        retained: ControllerRetainedCapacity,
        capabilities: PreparedDeliveryCapabilities,
    ) -> Result<
        (
            Self,
            AutomationDeliveryRender,
            ControllerAutomationResources,
        ),
        ControllerAutomationPrepareError,
    > {
        let queue_and_delivery = PreparedAutomationDelivery::resource_report_for_config(queues)?;
        let prepared_queues = ProtocolQueues::prepare(queues)?;
        let replay = ReplayCache::try_new(replay)?;
        let controller = ProtocolController::try_with_config_and_retained_capacity(
            session,
            prepared_queues,
            provider,
            replay,
            codec,
            config,
            retained,
        )?;
        let (delivery, render) = PreparedAutomationDelivery::prepare_state_and_render(queues)?;
        let resources = ControllerAutomationResources {
            queue_and_delivery,
            control_inline_bytes: size_of::<Self>(),
            render_inline_bytes: size_of::<AutomationDeliveryRender>(),
        };
        Ok((
            Self {
                controller,
                delivery,
                capabilities,
            },
            render,
            resources,
        ))
    }

    /// Process one typed command with replay classification preceding facade policy.
    pub fn process(&mut self, request: ControllerRequest<'_>) -> ControllerResponse {
        let mut context = DeliveryContext {
            state: &mut self.delivery,
        };
        self.controller
            .process_with_delivery_context(request, Some(&mut context))
    }

    /// Hand off the next admitted batch to the render half when its fixed capability set supports
    /// every record in the batch.
    pub fn try_handoff_next(&mut self) -> Result<HandoffResult, DeliveryError> {
        self.controller
            .delivery_try_handoff_next(&mut self.delivery, &self.capabilities)
    }

    /// Collect a render terminal only after all records in its ticket were applied.
    pub fn collect_terminal(
        &mut self,
        ticket: DeliveryTicket,
    ) -> Result<TerminalAutomation, DeliveryError> {
        self.controller
            .delivery_collect_terminal(&mut self.delivery, ticket)
    }

    /// Begin cancellation at the next render boundary, reserving all reliable cancellation events
    /// before publishing the barrier.
    pub fn begin_cancel(
        &mut self,
        reason: AutomationCancellationReason,
    ) -> Result<CancelToken, DeliveryError> {
        self.controller
            .delivery_begin_cancel(&mut self.delivery, reason)
    }

    /// Poll cancellation until the render half has acknowledged the actual boundary.
    pub fn poll_cancel_boundary(
        &mut self,
        token: CancelToken,
    ) -> Result<Option<CancelComplete>, DeliveryError> {
        self.controller
            .delivery_poll_cancel_boundary(&mut self.delivery, token)
    }

    /// Dequeue and encode one retained reliable event using the ordinary controller retry path.
    pub fn dequeue_reliable_event_frame_into(
        &mut self,
        output: &mut [u8],
    ) -> Result<Option<usize>, EventEgressError> {
        self.controller.dequeue_reliable_event_frame_into(output)
    }

    /// Return the number of queued or render-owned automation batches.
    #[must_use]
    pub fn outstanding(&self) -> usize {
        self.controller.delivery_outstanding(&self.delivery)
    }

    /// Return resident queue automation occupancy, excluding handed-off ownership.
    #[must_use]
    pub fn resident_automation(&self) -> u64 {
        self.controller.delivery_resident_automation(&self.delivery)
    }

    /// Return the current automation queue report.
    #[must_use]
    pub fn automation_status(&self) -> QueueReport {
        self.controller.queues().report(QueueKind::Automation)
    }

    /// Borrow the authoritative fixed-revision session snapshot source.
    #[must_use]
    pub fn session(&self) -> &SessionStore {
        self.controller.session()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use core::{
        num::NonZeroUsize,
        sync::atomic::{AtomicU64, Ordering},
    };
    use session::{CompileCaps, parse_session_json};
    use std::sync::Arc;

    const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.json");

    #[derive(Clone)]
    struct FixtureProvider {
        sample: SampleTime,
        transport: TransportSnapshot,
        canceled: Arc<AtomicU64>,
        descriptors: [ParameterDescriptor; 2],
    }

    impl FixtureProvider {
        fn new(sample: SampleTime) -> (Self, Arc<AtomicU64>) {
            let canceled = Arc::new(AtomicU64::new(0));
            let descriptor = |handle| ParameterDescriptor {
                handle,
                track_id: "fixture".to_owned(),
                rack: ParameterRack::Dynamic,
                effect_id: "effect".to_owned(),
                parameter_id: handle,
                channel: ParameterChannel::Left,
                value_kind: ParameterValueKind::F32,
                unit: ParameterUnit::Linear,
                domain: ParameterDomain::Continuous,
                minimum: Some(-1.0),
                maximum: Some(1.0),
                default: 0.0,
                mapping: ParameterMapping::Linear,
                automation_rate: ParameterAutomationRate::Sample,
                smoothing_samples: 0,
                flags: 3,
                display_name: None,
                display_unit: None,
                enum_choices: Vec::new(),
            };
            (
                Self {
                    sample,
                    transport: TransportSnapshot {
                        state: TransportState::Stopped,
                        position: SampleTime(0),
                        effective_sample: sample,
                    },
                    canceled: Arc::clone(&canceled),
                    descriptors: [descriptor(7), descriptor(8)],
                },
                canceled,
            )
        }
    }

    impl ControlProvider for FixtureProvider {
        fn current_sample(&mut self) -> SampleTime {
            self.sample
        }

        fn parameter_metadata(
            &mut self,
            request: ParameterMetadataRequest,
        ) -> Result<ParameterMetadataPage, ParameterProviderError> {
            let descriptors = self
                .descriptors
                .iter()
                .filter(|descriptor| descriptor.handle > request.after_handle)
                .take(usize::from(request.limit))
                .cloned()
                .collect::<Vec<_>>();
            let last_handle = descriptors
                .last()
                .map_or(request.after_handle, |descriptor| descriptor.handle);
            Ok(ParameterMetadataPage {
                last_handle,
                eof: descriptors.len() < usize::from(request.limit),
                descriptors,
            })
        }

        fn parameter_state(
            &mut self,
            request: &ParameterStateRequest,
        ) -> Result<ParameterStatePage, ParameterProviderError> {
            Ok(ParameterStatePage {
                observed_sample: self.sample.0,
                records: request
                    .handles
                    .iter()
                    .map(|handle| ParameterStateRecord {
                        handle: *handle,
                        flags: 1,
                        value: 0.0,
                    })
                    .collect(),
            })
        }

        fn parameter_descriptor(
            &mut self,
            handle: ParameterHandle,
        ) -> Result<&ParameterDescriptor, ParameterProviderError> {
            self.descriptors
                .iter()
                .find(|descriptor| descriptor.handle == handle.0)
                .ok_or(ParameterProviderError::NotFound)
        }

        fn counters(
            &mut self,
            _request: &CountersRequest,
        ) -> Result<CounterSnapshot, ParameterProviderError> {
            Ok(CounterSnapshot {
                observed_sample: self.sample,
                values: Vec::new(),
            })
        }

        fn record_canceled_automation(&mut self, records: u64) {
            self.canceled.fetch_add(records, Ordering::Relaxed);
        }

        fn diagnostics(
            &mut self,
            request: DiagnosticsRequest,
        ) -> Result<DiagnosticsPage, ParameterProviderError> {
            Ok(DiagnosticsPage {
                last_sequence: request.after_sequence,
                eof: true,
                diagnostics: Vec::new(),
            })
        }

        fn transport_get(&mut self) -> TransportSnapshot {
            self.transport
        }

        fn transport_set(&mut self, request: TransportSetRequest) -> TransportSnapshot {
            self.transport.state = request.state;
            if let Some(position) = request.position {
                self.transport.position = position;
            }
            self.transport.effective_sample = self.sample;
            self.transport
        }

        fn telemetry_configure(
            &mut self,
            configuration: TelemetryConfiguration,
        ) -> TelemetryConfiguration {
            configuration
        }
    }

    fn queue_config(reliable_events: usize) -> ProtocolQueueConfig {
        ProtocolQueueConfig {
            control_command_slots: NonZeroUsize::new(2).unwrap(),
            control_command_bytes: NonZeroUsize::new(512).unwrap(),
            automation_batch_slots: NonZeroUsize::new(2).unwrap(),
            reliable_response_slots: NonZeroUsize::new(2).unwrap(),
            reliable_event_slots: NonZeroUsize::new(reliable_events).unwrap(),
            telemetry_slots: NonZeroUsize::new(2).unwrap(),
            per_block_automation_density: NonZeroUsize::new(256).unwrap(),
            quantum_frames: NonZeroUsize::new(64).unwrap(),
        }
    }

    fn session() -> SessionStore {
        SessionStore::new(
            parse_session_json(EXAMPLE).unwrap(),
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .unwrap()
    }

    fn replay() -> ReplayCacheConfig {
        ReplayCacheConfig {
            entries: NonZeroUsize::new(32).unwrap(),
            bytes: NonZeroUsize::new(16 * 1024).unwrap(),
            max_response_bytes: 2048,
        }
    }

    fn facade(
        reliable_events: usize,
        capabilities: &[(ParameterHandle, AutomationKind)],
    ) -> (
        ControllerAutomationDelivery<FixtureProvider>,
        AutomationDeliveryRender,
        Arc<AtomicU64>,
    ) {
        facade_with(reliable_events, capabilities, SampleTime(0), 256)
    }

    fn facade_with(
        reliable_events: usize,
        capabilities: &[(ParameterHandle, AutomationKind)],
        sample: SampleTime,
        density: usize,
    ) -> (
        ControllerAutomationDelivery<FixtureProvider>,
        AutomationDeliveryRender,
        Arc<AtomicU64>,
    ) {
        let (provider, canceled) = FixtureProvider::new(sample);
        let capabilities = PreparedDeliveryCapabilities::new_exact(capabilities).unwrap();
        let mut queues = queue_config(reliable_events);
        queues.per_block_automation_density = NonZeroUsize::new(density).unwrap();
        let (facade, render, _) = ControllerAutomationDelivery::prepare(
            session(),
            queues,
            provider,
            replay(),
            ProtocolCodec::default(),
            ProtocolControllerConfig::default(),
            ControllerRetainedCapacity {
                meter_handles: 0,
                counter_ids: 0,
            },
            capabilities,
        )
        .unwrap();
        (facade, render, canceled)
    }

    fn batch(
        revision: SessionRevision,
        request_id: u64,
        handle: u32,
        start: u64,
    ) -> AutomationBatchSlot {
        AutomationBatchSlot::new(
            revision,
            RequestId::new(request_id).unwrap(),
            &[AutomationRecord {
                kind: AutomationKind::Point,
                handle: ParameterHandle(handle),
                start: SampleTime(start),
                end: SampleTime(start),
                start_value: 0.5,
                end_value: 0.5,
            }],
        )
        .unwrap()
    }

    fn mixed_batch(revision: SessionRevision, request_id: u64) -> AutomationBatchSlot {
        let first = AutomationRecord {
            kind: AutomationKind::Point,
            handle: ParameterHandle(7),
            start: SampleTime(1),
            end: SampleTime(1),
            start_value: 0.5,
            end_value: 0.5,
        };
        AutomationBatchSlot::new(
            revision,
            RequestId::new(request_id).unwrap(),
            &[
                first,
                AutomationRecord {
                    handle: ParameterHandle(8),
                    start: SampleTime(2),
                    end: SampleTime(2),
                    ..first
                },
            ],
        )
        .unwrap()
    }

    fn enqueue_request<'a>(
        request_id: u64,
        bytes: &'a [u8],
        batch: AutomationBatchSlot,
    ) -> ControllerRequest<'a> {
        ControllerRequest {
            request_id: RequestId::new(request_id).unwrap(),
            expected_revision: ExpectedRevision::Exact(batch.revision),
            canonical_bytes: bytes,
            command: ControlCommand::AutomationEnqueue { batch },
        }
    }

    fn decode_event<'a>(codec: &ProtocolCodec, frame: &'a [u8]) -> DecodedTypedEventFrame<'a> {
        codec
            .decode_typed_event(frame, &mut DecodeScratch::new(&mut [0_u16; 32]))
            .unwrap()
    }

    fn event_sequence(codec: &ProtocolCodec, frame: &[u8]) -> u64 {
        match decode_event(codec, frame).payload {
            DecodedEventPayload::TransportState(value) => value.event_sequence,
            DecodedEventPayload::AutomationCanceled(value) => value.event_sequence,
            _ => 0,
        }
    }

    #[test]
    fn real_admission_replay_shape_and_domain_guards_are_owned_by_facade() {
        let (mut facade, mut render, _) = facade(8, &[(ParameterHandle(7), AutomationKind::Point)]);
        let revision = facade.session().revision();
        let admitted_batch = batch(revision, 1, 7, 1);
        let request = enqueue_request(1, b"enqueue-one", admitted_batch);
        let response = facade.process(request);
        assert_eq!(response.status, StatusCode::Ok);
        let decoded = ProtocolCodec::default()
            .decode_typed_response(&response.frame, &mut DecodeScratch::new(&mut [0_u16; 32]))
            .unwrap();
        let DecodedTypedResponseFrame::Success { payload, .. } = decoded else {
            panic!("automation admission must return success payload")
        };
        let DecodedSuccessResponsePayload::AutomationEnqueued(accepted) = payload else {
            panic!("wrong success payload")
        };
        assert_eq!(
            (
                accepted.accepted_records,
                accepted.occupancy,
                accepted.capacity
            ),
            (1, 1, 2)
        );
        let ticket = match facade.try_handoff_next().unwrap() {
            HandoffResult::HandedOff(ticket) => ticket,
            other => panic!("{other:?}"),
        };
        let pending = render.begin_boundary(SampleTime(0)).unwrap();
        assert_eq!(pending.records, admitted_batch.as_slice());
        let count = pending.records.len() as u16;
        render.mark_applied(ticket, count).unwrap();
        render.finish_applied(ticket, count).unwrap();
        facade.collect_terminal(ticket).unwrap();
        assert_eq!(facade.outstanding(), 0);
        assert_eq!(
            facade.process(enqueue_request(1, b"enqueue-one", batch(revision, 1, 7, 1))),
            response
        );
        assert_eq!(
            facade
                .process(enqueue_request(1, b"changed", batch(revision, 1, 7, 1)))
                .status,
            StatusCode::RequestIdReuse
        );

        let mut malformed = batch(revision, 2, 7, 3);
        malformed.len = 2;
        malformed.records[1] = AutomationRecord {
            kind: AutomationKind::Point,
            handle: ParameterHandle(0),
            start: SampleTime(4),
            end: SampleTime(4),
            start_value: 0.5,
            end_value: 0.5,
        };
        let rejected = facade.process(enqueue_request(2, b"malformed", malformed));
        assert_eq!(rejected.status, StatusCode::InvalidField);
        assert_eq!(facade.outstanding(), 0);

        let mut too_long = batch(revision, 3, 7, 3);
        too_long.len = 257;
        assert_eq!(
            facade
                .process(enqueue_request(3, b"too-long", too_long))
                .status,
            StatusCode::LimitExceeded
        );
        assert_eq!(facade.outstanding(), 0);

        let mut outside_domain = batch(revision, 4, 7, 4);
        outside_domain.records[0].start_value = 2.0;
        outside_domain.records[0].end_value = 2.0;
        assert_eq!(
            facade
                .process(enqueue_request(4, b"outside-domain", outside_domain))
                .status,
            StatusCode::InvalidField
        );
        assert_eq!(facade.outstanding(), 0);

        let (mut past_facade, _, _) = facade_with(
            8,
            &[(ParameterHandle(7), AutomationKind::Point)],
            SampleTime(10),
            256,
        );
        let past_revision = past_facade.session().revision();
        assert_eq!(
            past_facade
                .process(enqueue_request(1, b"past", batch(past_revision, 1, 7, 9)))
                .status,
            StatusCode::TimeInPast
        );
        assert_eq!(past_facade.outstanding(), 0);
    }

    #[test]
    fn reservations_survive_render_completion_until_terminal_collection() {
        let (mut facade, mut render, _) = facade(8, &[(ParameterHandle(7), AutomationKind::Point)]);
        let revision = facade.session().revision();
        let first = facade.process(enqueue_request(1, b"first", batch(revision, 1, 7, 1)));
        assert_eq!(first.status, StatusCode::Ok);
        let ticket = match facade.try_handoff_next().unwrap() {
            HandoffResult::HandedOff(ticket) => ticket,
            other => panic!("{other:?}"),
        };
        let count = render.begin_boundary(SampleTime(0)).unwrap().records.len() as u16;
        render.mark_applied(ticket, count).unwrap();
        render.finish_applied(ticket, count).unwrap();
        assert_eq!(
            facade
                .process(enqueue_request(2, b"overlap", batch(revision, 2, 7, 1)))
                .status,
            StatusCode::AutomationOrder
        );
        assert_eq!(
            facade
                .process(enqueue_request(3, b"second", batch(revision, 3, 7, 2)))
                .status,
            StatusCode::Ok
        );
        assert_eq!(
            facade
                .process(enqueue_request(4, b"third", batch(revision, 4, 7, 3)))
                .status,
            StatusCode::Backpressure
        );
        facade.collect_terminal(ticket).unwrap();
        assert_eq!(
            facade.collect_terminal(ticket),
            Err(DeliveryError::StaleTicket)
        );
        assert_eq!(
            facade
                .process(enqueue_request(
                    5,
                    b"after-collect",
                    batch(revision, 5, 7, 3)
                ))
                .status,
            StatusCode::Ok
        );

        let (mut dense, mut dense_render, _) = facade_with(
            8,
            &[
                (ParameterHandle(7), AutomationKind::Point),
                (ParameterHandle(8), AutomationKind::Point),
            ],
            SampleTime(0),
            1,
        );
        let dense_revision = dense.session().revision();
        assert_eq!(
            dense
                .process(enqueue_request(
                    10,
                    b"dense-first",
                    batch(dense_revision, 10, 7, 1)
                ))
                .status,
            StatusCode::Ok
        );
        assert_eq!(
            dense
                .process(enqueue_request(
                    11,
                    b"dense-second",
                    batch(dense_revision, 11, 8, 1)
                ))
                .status,
            StatusCode::LimitExceeded
        );
        let dense_ticket = match dense.try_handoff_next().unwrap() {
            HandoffResult::HandedOff(ticket) => ticket,
            other => panic!("{other:?}"),
        };
        assert_eq!(
            dense
                .process(enqueue_request(
                    12,
                    b"dense-after-handoff",
                    batch(dense_revision, 12, 8, 1)
                ))
                .status,
            StatusCode::LimitExceeded
        );
        let dense_count = dense_render
            .begin_boundary(SampleTime(0))
            .unwrap()
            .records
            .len() as u16;
        dense_render
            .mark_applied(dense_ticket, dense_count)
            .unwrap();
        dense_render
            .finish_applied(dense_ticket, dense_count)
            .unwrap();
        dense.collect_terminal(dense_ticket).unwrap();
        assert_eq!(
            dense
                .process(enqueue_request(
                    13,
                    b"dense-after-collect",
                    batch(dense_revision, 13, 8, 1)
                ))
                .status,
            StatusCode::Ok
        );
    }

    #[test]
    fn unsupported_head_blocks_fifo_and_real_cancel_releases_all_owners() {
        let (mut endpoint, mut render, canceled) =
            facade(8, &[(ParameterHandle(7), AutomationKind::Point)]);
        let revision = endpoint.session().revision();
        assert_eq!(
            endpoint
                .process(enqueue_request(1, b"unsupported", mixed_batch(revision, 1)))
                .status,
            StatusCode::Ok
        );
        assert_eq!(
            endpoint
                .process(enqueue_request(
                    2,
                    b"supported-follower",
                    batch(revision, 2, 7, 2)
                ))
                .status,
            StatusCode::Ok
        );
        assert_eq!(
            endpoint.try_handoff_next().unwrap(),
            HandoffResult::PendingUnsupported
        );
        let token = endpoint
            .begin_cancel(AutomationCancellationReason::EndpointShutdown)
            .unwrap();
        assert!(endpoint.poll_cancel_boundary(token).unwrap().is_none());
        assert!(render.begin_boundary(SampleTime(77)).is_none());
        let complete = endpoint.poll_cancel_boundary(token).unwrap().unwrap();
        assert_eq!(
            (complete.canceled_events, complete.canceled_records),
            (2, 3)
        );
        assert_eq!(canceled.load(Ordering::Relaxed), 3);
        assert_eq!(endpoint.outstanding(), 0);
    }

    #[test]
    fn one_controller_queue_and_sequence_cover_transport_cancel_and_short_retry() {
        let (mut endpoint, mut render, canceled) =
            facade(8, &[(ParameterHandle(7), AutomationKind::Point)]);
        let revision = endpoint.session().revision();
        let transport = |id, bytes| ControllerRequest {
            request_id: RequestId::new(id).unwrap(),
            expected_revision: ExpectedRevision::Exact(revision),
            canonical_bytes: bytes,
            command: ControlCommand::TransportSet {
                request: TransportSetRequest {
                    state: TransportState::Playing,
                    position: None,
                },
            },
        };
        let first_transport = endpoint.process(transport(1, b"transport-one"));
        assert_eq!(first_transport.status, StatusCode::Ok);
        let ticket = {
            assert_eq!(
                endpoint
                    .process(enqueue_request(2, b"partial", batch(revision, 2, 7, 1)))
                    .status,
                StatusCode::Ok
            );
            match endpoint.try_handoff_next().unwrap() {
                HandoffResult::HandedOff(ticket) => ticket,
                other => panic!("{other:?}"),
            }
        };
        render.begin_boundary(SampleTime(0)).unwrap();
        render.mark_applied(ticket, 0).unwrap();
        let token = endpoint
            .begin_cancel(AutomationCancellationReason::EndpointShutdown)
            .unwrap();
        assert!(endpoint.poll_cancel_boundary(token).unwrap().is_none());
        let blocked = endpoint.process(transport(3, b"blocked-transport"));
        assert_eq!(blocked.status, StatusCode::Unavailable);
        assert_eq!(
            endpoint.process(transport(3, b"blocked-transport")),
            blocked
        );
        assert_eq!(
            endpoint.process(transport(1, b"transport-one")),
            first_transport
        );
        assert!(render.begin_boundary(SampleTime(99)).is_none());
        let complete = endpoint.poll_cancel_boundary(token).unwrap().unwrap();
        assert_eq!(
            (
                complete.effective_sample,
                complete.canceled_events,
                complete.canceled_records,
                complete.applied_records
            ),
            (SampleTime(99), 1, 1, 0)
        );
        assert_eq!(canceled.load(Ordering::Relaxed), 1);
        assert_eq!(
            endpoint.poll_cancel_boundary(token),
            Err(DeliveryError::StaleTicket)
        );
        let later = endpoint.process(transport(4, b"transport-later"));
        assert_eq!(later.status, StatusCode::Ok);

        let mut short = [0_u8; 1];
        assert!(matches!(
            endpoint.dequeue_reliable_event_frame_into(&mut short),
            Err(EventEgressError::Encode(EncodeError::OutputTooSmall { .. }))
        ));
        let mut frame = [0_u8; 1024];
        let first = endpoint
            .dequeue_reliable_event_frame_into(&mut frame)
            .unwrap()
            .unwrap();
        match decode_event(&ProtocolCodec::default(), &frame[..first]).payload {
            DecodedEventPayload::TransportState(value) => {
                assert_eq!(
                    (
                        value.event_sequence,
                        value.state,
                        value.position,
                        value.effective_sample
                    ),
                    (1, TransportState::Playing, SampleTime(0), SampleTime(0))
                );
            }
            _ => panic!("unexpected first event"),
        }
        let second = endpoint
            .dequeue_reliable_event_frame_into(&mut frame)
            .unwrap()
            .unwrap();
        match decode_event(&ProtocolCodec::default(), &frame[..second]).payload {
            DecodedEventPayload::AutomationCanceled(value) => {
                assert_eq!(
                    (
                        value.event_sequence,
                        value.origin_request_id,
                        value.canceled_records,
                        value.effective_sample
                    ),
                    (2, RequestId::new(2).unwrap(), 1, Some(SampleTime(99)))
                );
            }
            _ => panic!("unexpected cancellation event"),
        }
        let third = endpoint
            .dequeue_reliable_event_frame_into(&mut frame)
            .unwrap()
            .unwrap();
        assert_eq!(
            event_sequence(&ProtocolCodec::default(), &frame[..third]),
            3
        );

        let (mut full, _, _) = facade(1, &[(ParameterHandle(7), AutomationKind::Point)]);
        let full_revision = full.session().revision();
        assert_eq!(
            full.process(transport_request(full_revision, 1, b"full-transport"))
                .status,
            StatusCode::Ok
        );
        assert_eq!(
            full.process(enqueue_request(
                2,
                b"full-batch",
                batch(full_revision, 2, 7, 1)
            ))
            .status,
            StatusCode::Ok
        );
        let before = (full.outstanding(), full.automation_status());
        assert!(matches!(
            full.begin_cancel(AutomationCancellationReason::EndpointShutdown),
            Err(DeliveryError::ReliableFull(_))
        ));
        assert_eq!((full.outstanding(), full.automation_status()), before);
        let mut full_frame = [0_u8; 1024];
        let first_full = full
            .dequeue_reliable_event_frame_into(&mut full_frame)
            .unwrap()
            .unwrap();
        assert_eq!(
            event_sequence(&ProtocolCodec::default(), &full_frame[..first_full]),
            1
        );
        assert_eq!(
            full.process(transport_request(full_revision, 3, b"full-later"))
                .status,
            StatusCode::Ok
        );
        let second_full = full
            .dequeue_reliable_event_frame_into(&mut full_frame)
            .unwrap()
            .unwrap();
        assert_eq!(
            event_sequence(&ProtocolCodec::default(), &full_frame[..second_full]),
            2
        );
    }

    fn transport_request<'a>(
        revision: SessionRevision,
        id: u64,
        bytes: &'a [u8],
    ) -> ControllerRequest<'a> {
        ControllerRequest {
            request_id: RequestId::new(id).unwrap(),
            expected_revision: ExpectedRevision::Exact(revision),
            canonical_bytes: bytes,
            command: ControlCommand::TransportSet {
                request: TransportSetRequest {
                    state: TransportState::Playing,
                    position: None,
                },
            },
        }
    }

    #[test]
    fn fixed_revision_refuses_structural_locate_and_state_without_model_change() {
        let (mut facade, _, _) = facade(8, &[(ParameterHandle(7), AutomationKind::Point)]);
        let revision = facade.session().revision();
        let snapshot = facade.session().canonical_snapshot().to_owned();
        let edits: [SessionEdit; 0] = [];
        assert_eq!(
            facade
                .process(ControllerRequest {
                    request_id: RequestId::new(1).unwrap(),
                    expected_revision: ExpectedRevision::Exact(revision),
                    canonical_bytes: b"transaction",
                    command: ControlCommand::SessionTransactionApply { edits: &edits },
                })
                .status,
            StatusCode::Unavailable
        );
        assert_eq!(
            facade
                .process(ControllerRequest {
                    request_id: RequestId::new(2).unwrap(),
                    expected_revision: ExpectedRevision::Exact(revision),
                    canonical_bytes: b"locate",
                    command: ControlCommand::TransportSet {
                        request: TransportSetRequest {
                            state: TransportState::Playing,
                            position: Some(SampleTime(9))
                        }
                    },
                })
                .status,
            StatusCode::Unavailable
        );
        assert_eq!(
            facade
                .process(ControllerRequest {
                    request_id: RequestId::new(3).unwrap(),
                    expected_revision: ExpectedRevision::Exact(revision),
                    canonical_bytes: b"state",
                    command: ControlCommand::ParameterStateGet {
                        request: ParameterStateRequest { handles: vec![7] }
                    },
                })
                .status,
            StatusCode::Unavailable
        );
        assert_eq!(facade.session().canonical_snapshot(), snapshot);
        assert_eq!(facade.session().revision(), revision);
        assert_eq!(
            facade
                .process(ControllerRequest {
                    request_id: RequestId::new(4).unwrap(),
                    expected_revision: ExpectedRevision::Any,
                    canonical_bytes: b"metadata",
                    command: ControlCommand::ParameterMetadataGet {
                        request: ParameterMetadataRequest {
                            after_handle: 0,
                            limit: 8
                        }
                    },
                })
                .status,
            StatusCode::Ok
        );
        assert_eq!(
            facade
                .process(ControllerRequest {
                    request_id: RequestId::new(5).unwrap(),
                    expected_revision: ExpectedRevision::Any,
                    canonical_bytes: b"snapshot",
                    command: ControlCommand::SessionSnapshotGet {
                        offset: 0,
                        max_bytes: 1024
                    },
                })
                .status,
            StatusCode::Ok
        );
        let transport_before = facade.process(ControllerRequest {
            request_id: RequestId::new(6).unwrap(),
            expected_revision: ExpectedRevision::Any,
            canonical_bytes: b"transport-before",
            command: ControlCommand::TransportGet,
        });
        let transport_after = facade.process(ControllerRequest {
            request_id: RequestId::new(7).unwrap(),
            expected_revision: ExpectedRevision::Any,
            canonical_bytes: b"transport-after",
            command: ControlCommand::TransportGet,
        });
        let decode_transport = |response: &ControllerResponse| {
            let DecodedTypedResponseFrame::Success { payload, .. } = ProtocolCodec::default()
                .decode_typed_response(&response.frame, &mut DecodeScratch::new(&mut [0_u16; 16]))
                .unwrap()
            else {
                panic!("transport get must succeed")
            };
            let DecodedSuccessResponsePayload::TransportGetSnapshot(value) = payload else {
                panic!("wrong transport payload")
            };
            value
        };
        assert_eq!(
            decode_transport(&transport_before),
            decode_transport(&transport_after)
        );
        assert_eq!(facade.outstanding(), 0);
    }
}
