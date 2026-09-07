//! Fixed-revision typed controller ownership for transient automation delivery.

use core::mem::size_of;

use crate::delivery::{
    AutomationDeliveryState, DeliveryContext, PreparedAutomationDelivery,
};
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
        (Self, AutomationDeliveryRender, ControllerAutomationResources),
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
