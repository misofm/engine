//! Bounded request replay and a control-plane-only protocol dispatcher.
//!
//! The dispatcher consumes typed internal commands in this tranche. Later BTLV payload schemas
//! will construct the same commands after the bounded wire decoder has finished; no decoder calls
//! a renderer, and this module has no plan-publication capability.

use core::{alloc::Layout, fmt, num::NonZeroUsize, ops::Range};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

#[cfg(test)]
std::thread_local! {
    static PREPARED_IMMEDIATE_CALLS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
    static PROSPECTIVE_REPLAY_CLONES: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
    static RESPONSE_STAGING_VECS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
    static TYPED_COMMAND_DECODES: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}

use crate::{
    AutomationBatchError, AutomationBatchSlot, AutomationCanceled, AutomationCancellationReason,
    AutomationEnqueueError, AutomationEnqueued, Backpressure, BackpressureQueueKind, Capabilities,
    CapabilityFlags, CommandHeader, CounterSnapshot, CounterSnapshotRef, CounterTelemetryRecord,
    CounterValue, CountersRequest, DecodeError, DecodeScratch, DecodedCommandPayload, Diagnostic,
    DiagnosticEvent, DiagnosticsPage, DiagnosticsRequest, EncodeError, EventPayload,
    ExpectedRevision, MessageId, MeterBatch, MeterRecord, NonOkResponse, ParameterAutomationRate,
    ParameterDescriptor, ParameterDomain, ParameterHandle, ParameterMetadataPage,
    ParameterMetadataRequest, ParameterStatePage, ParameterStateRequest,
    PreparedSessionTransaction, ProtocolCodec, ProtocolQueues, QueueReport, ReliablePayload,
    ReliableSlot, RequestId, SampleTime, SessionCommitted, SessionEdit, SessionRevision,
    SessionSnapshot, SessionStore, SessionStoreError, StatusCode, SuccessResponsePayload,
    TelemetryConfiguration, TelemetryKey, TelemetryRecord, TransactionApplied, TransportSetRequest,
    TransportSnapshot, TransportState, TransportStateEvent, TypedEventFrame,
    TypedNonOkResponseFrame, TypedSuccessResponseFrame,
};

/// Bounded replay storage configuration for one logical endpoint lifetime.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplayCacheConfig {
    /// Exact maximum number of completed request/response entries.
    pub entries: NonZeroUsize,
    /// Exact maximum combined canonical-request and response bytes retained.
    pub bytes: NonZeroUsize,
    /// Maximum response bytes reserved before any new command executes.
    pub max_response_bytes: usize,
}

/// Exact configured heap-payload budget for one endpoint replay cache.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplayCacheResourceReport {
    /// Entry-ring payload plus the configured aggregate canonical request/response byte budget.
    pub retained_payload_bytes: u64,
    /// Largest single permitted replay allocation payload.
    pub largest_allocation_bytes: u64,
}

/// Endpoint-owned controller bounds that are advertised and enforced before edit allocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProtocolControllerConfig {
    /// Maximum accepted session edits in one transaction; zero disables transaction application.
    pub maximum_transaction_edits: u32,
    /// Maximum leading validation diagnostics included in one non-OK endpoint response.
    ///
    /// The response encoder may retain fewer when the endpoint's frame limit cannot encode the
    /// next diagnostic; `omitted_diagnostics` always reports the exact remaining suffix.
    pub maximum_response_diagnostics: u16,
    /// Typed provider/event enablement used for capability construction and dispatch.
    pub provider_features: ProviderFeatures,
}

/// Eager retained collection capacities for controller/provider configuration state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ControllerRetainedCapacity {
    /// Meter handles retained independently by the controller and provider.
    pub meter_handles: usize,
    /// Counter identifiers retained independently by the controller and provider.
    pub counter_ids: usize,
}

/// Eager controller/provider allocation could not be completed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ControllerResourceAllocationError;

fn retained_telemetry_configuration(
    capacity: ControllerRetainedCapacity,
) -> Result<TelemetryConfiguration, ControllerResourceAllocationError> {
    let mut meter_handles = Vec::new();
    meter_handles
        .try_reserve_exact(capacity.meter_handles)
        .map_err(|_| ControllerResourceAllocationError)?;
    let mut counter_ids = Vec::new();
    counter_ids
        .try_reserve_exact(capacity.counter_ids)
        .map_err(|_| ControllerResourceAllocationError)?;
    Ok(TelemetryConfiguration {
        meter_handles,
        meter_period_blocks: 0,
        counter_ids,
        counter_period_blocks: 0,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: crate::DiagnosticSeverity::Info,
    })
}

/// Typed provider and event enablement; no raw capability bytes are accepted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)] // Frozen one-to-one capability feature switches.
pub struct ProviderFeatures {
    pub parameters: bool,
    pub transport: bool,
    pub meters: bool,
    pub counters: bool,
    pub diagnostics: bool,
    pub session_events: bool,
    pub transport_events: bool,
}

impl ProviderFeatures {
    /// Enable every typed provider and event family.
    pub const ALL: Self = Self {
        parameters: true,
        transport: true,
        meters: true,
        counters: true,
        diagnostics: true,
        session_events: true,
        transport_events: true,
    };
    /// Disable every optional provider and event family.
    pub const NONE: Self = Self {
        parameters: false,
        transport: false,
        meters: false,
        counters: false,
        diagnostics: false,
        session_events: false,
        transport_events: false,
    };
}

impl Default for ProtocolControllerConfig {
    fn default() -> Self {
        Self {
            maximum_transaction_edits: 1024,
            maximum_response_diagnostics: 256,
            provider_features: ProviderFeatures::ALL,
        }
    }
}

/// Result of deciding one canonical request against an endpoint replay cache.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReplayDecision {
    /// The request is new and has reserved enough configured replay capacity to execute.
    Execute,
    /// An exact same-ID/same-byte request returns this cached response with no execution.
    Cached(ReplayHit),
    /// Same request ID had different canonical bytes.
    RequestIdReuse,
    /// Request ID was retired, evicted, or was not strictly increasing as a new request.
    ReplayExpired,
    /// The request is unaccepted because it cannot reserve replay capacity before execution.
    /// Its request ID remains reusable until a later successful preflight accepts it.
    Backpressure,
}

/// Opaque stable identity of one exact cached response.
///
/// A hit remains valid while its request entry survives in its originating cache, including
/// across arena compaction. Cross-cache and evicted hits are rejected by [`ReplayCache::cached`]
/// and [`ReplayCache::try_cached`] without indexing stale arena offsets.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplayHit {
    cache_id: u64,
    request_id: RequestId,
}

/// One cached exact request/response byte pair.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct ReplayEntry {
    request_id: u64,
    request_offset: usize,
    request_bytes: usize,
    response_offset: usize,
    response_bytes: usize,
    // Preserve the established per-slot resource authority even though response metadata now
    // comes from the canonical cached frame rather than duplicate typed fields.
    _resource_layout_reservation: [u8; 16],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ReplayReservation {
    request_id: RequestId,
    request_offset: usize,
    request_bytes: usize,
}

#[derive(Clone, Copy)]
struct ReplayPreflightPlan {
    evicted: usize,
    removed_bytes: usize,
}

static NEXT_REPLAY_CACHE_ID: AtomicU64 = AtomicU64::new(1);

/// Bounded exact-byte replay cache. It intentionally covers one endpoint lifetime only.
pub struct ReplayCache {
    cache_id: u64,
    config: ReplayCacheConfig,
    entries: Box<[ReplayEntry]>,
    storage: Box<[u8]>,
    reservation: Option<ReplayReservation>,
}

impl ReplayCache {
    /// Project the bounded retained payload budget used by one cache configuration.
    pub fn resource_report_for_config(
        config: ReplayCacheConfig,
    ) -> Result<ReplayCacheResourceReport, ReplayCacheError> {
        let entries = Layout::array::<ReplayEntry>(config.entries.get())
            .map_err(|_| ReplayCacheError::ResourceOverflow)?
            .size();
        let retained = entries
            .checked_add(config.bytes.get())
            .ok_or(ReplayCacheError::ResourceOverflow)?;
        Ok(ReplayCacheResourceReport {
            retained_payload_bytes: u64::try_from(retained)
                .map_err(|_| ReplayCacheError::ResourceOverflow)?,
            largest_allocation_bytes: u64::try_from(entries.max(config.bytes.get()))
                .map_err(|_| ReplayCacheError::ResourceOverflow)?,
        })
    }

    /// Prepare a bounded replay cache off the render plane.
    #[must_use]
    pub fn new(config: ReplayCacheConfig) -> Self {
        Self::try_new(config)
            .expect("valid replay configuration allocates its eager retained arena")
    }

    /// Fallibly allocate the complete replay metadata and byte arena before endpoint publication.
    pub fn try_new(config: ReplayCacheConfig) -> Result<Self, ReplayCacheError> {
        Self::resource_report_for_config(config)?;
        let cache_id = NEXT_REPLAY_CACHE_ID
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |id| id.checked_add(1))
            .map_err(|_| ReplayCacheError::ResourceOverflow)?;
        let mut entries = Vec::new();
        entries
            .try_reserve_exact(config.entries.get())
            .map_err(|_| ReplayCacheError::ResourceAllocation)?;
        entries.resize(config.entries.get(), ReplayEntry::default());
        let mut storage = Vec::new();
        storage
            .try_reserve_exact(config.bytes.get())
            .map_err(|_| ReplayCacheError::ResourceAllocation)?;
        storage.resize(config.bytes.get(), 0);
        Ok(Self {
            cache_id,
            config,
            entries: entries.into_boxed_slice(),
            storage: storage.into_boxed_slice(),
            reservation: None,
        })
    }

    fn try_clone_eager(&self) -> Result<Self, ReplayCacheError> {
        #[cfg(test)]
        PROSPECTIVE_REPLAY_CLONES.with(|clones| clones.set(clones.get().saturating_add(1)));
        let mut cloned = Self::try_new(self.config)?;
        cloned.entries.copy_from_slice(&self.entries);
        cloned.storage.copy_from_slice(&self.storage);
        cloned.reservation = self.reservation;
        Ok(cloned)
    }

    fn completed_len(&self) -> usize {
        self.entries.partition_point(|entry| entry.request_id != 0)
    }

    fn completed_retained_bytes(&self) -> usize {
        self.entries[..self.completed_len()]
            .last()
            .map_or(0, |entry| {
                entry.response_offset.saturating_add(entry.response_bytes)
            })
    }

    fn highest_new_id(&self) -> Option<RequestId> {
        self.reservation
            .map(|reservation| reservation.request_id)
            .or_else(|| {
                self.entries[..self.completed_len()]
                    .last()
                    .and_then(|entry| RequestId::new(entry.request_id))
            })
    }

    fn entry_index(&self, request_id: RequestId) -> Result<usize, usize> {
        let request_id = request_id.get();
        let entries = &self.entries[..self.completed_len()];
        let index = entries.partition_point(|entry| entry.request_id < request_id);
        if index < entries.len() && entries[index].request_id == request_id {
            Ok(index)
        } else {
            Err(index)
        }
    }

    fn evict_prefix(&mut self, count: usize, removed_bytes: usize) {
        if count == 0 {
            return;
        }
        let len = self.completed_len();
        let retained_bytes = self.completed_retained_bytes();
        self.storage.copy_within(removed_bytes..retained_bytes, 0);
        self.entries.copy_within(count..len, 0);
        let remaining = len - count;
        for entry in &mut self.entries[..remaining] {
            entry.request_offset -= removed_bytes;
            entry.response_offset -= removed_bytes;
        }
        self.entries[remaining..len].fill(ReplayEntry::default());
    }

    fn plan_preflight(
        &self,
        request_id: RequestId,
        request: &[u8],
    ) -> Result<ReplayPreflightPlan, ReplayDecision> {
        if let Ok(index) = self.entry_index(request_id) {
            let entry = self.entries[index];
            return Err(
                if self.storage[entry.request_offset..entry.request_offset + entry.request_bytes]
                    == *request
                {
                    ReplayDecision::Cached(ReplayHit {
                        cache_id: self.cache_id,
                        request_id,
                    })
                } else {
                    ReplayDecision::RequestIdReuse
                },
            );
        }
        if self.reservation.is_some() {
            return Err(ReplayDecision::Backpressure);
        }
        if self
            .highest_new_id()
            .is_some_and(|highest| request_id <= highest)
        {
            return Err(ReplayDecision::ReplayExpired);
        }
        let reservation = request
            .len()
            .checked_add(self.config.max_response_bytes)
            .ok_or(ReplayDecision::Backpressure)?;
        if reservation > self.config.bytes.get() {
            return Err(ReplayDecision::Backpressure);
        }
        let len = self.completed_len();
        let retained_bytes = self.completed_retained_bytes();
        let mut evicted = 0;
        let mut removed_bytes = 0;
        while len.saturating_sub(evicted) >= self.entries.len()
            || retained_bytes
                .saturating_sub(removed_bytes)
                .saturating_add(reservation)
                > self.storage.len()
        {
            let Some(entry) = self.entries.get(evicted).filter(|_| evicted < len) else {
                return Err(ReplayDecision::Backpressure);
            };
            removed_bytes = removed_bytes.saturating_add(entry.byte_len());
            evicted += 1;
        }
        Ok(ReplayPreflightPlan {
            evicted,
            removed_bytes,
        })
    }

    /// Inspect/capacity-reserve a request before execution. An `Execute` result makes enough
    /// room for `max_response_bytes`, so [`Self::complete`] cannot need an unbounded retry.
    ///
    /// A `Backpressure` result is deliberately **unaccepted**: it neither advances the strictly
    /// increasing new-request frontier nor creates a replay entry. This is the endpoint's one
    /// replay-preflight policy, so the same ID remains reusable if capacity later permits it.
    /// While an accepted request is pending completion, other new preflights return
    /// `Backpressure` without replacing its exact-byte reservation.
    pub fn preflight(&mut self, request_id: RequestId, request: &[u8]) -> ReplayDecision {
        let plan = match self.plan_preflight(request_id, request) {
            Ok(plan) => plan,
            Err(decision) => return decision,
        };
        self.apply_preflight_plan(request_id, request, plan);
        ReplayDecision::Execute
    }

    fn apply_preflight_plan(
        &mut self,
        request_id: RequestId,
        request: &[u8],
        plan: ReplayPreflightPlan,
    ) {
        self.evict_prefix(plan.evicted, plan.removed_bytes);
        let request_offset = self.completed_retained_bytes();
        self.storage[request_offset..request_offset + request.len()].copy_from_slice(request);
        self.reservation = Some(ReplayReservation {
            request_id,
            request_offset,
            request_bytes: request.len(),
        });
    }

    /// Whether this ID would reach new-request admission rather than an already-known replay
    /// result. This is intentionally read-only so caller-output reservation can happen before
    /// preflight advances the endpoint's new-request frontier.
    fn is_new_request(&self, request_id: RequestId) -> bool {
        self.entry_index(request_id).is_err()
            && self
                .highest_new_id()
                .is_none_or(|highest| request_id > highest)
    }

    /// Cache the exact bytes from a request that previously received [`ReplayDecision::Execute`].
    /// The response must fit the fixed reservation made by [`Self::preflight`]. An ID, length, or
    /// byte mismatch leaves the original reservation intact so its exact completion can retry.
    pub fn complete(
        &mut self,
        request_id: RequestId,
        request: &[u8],
        response: &[u8],
    ) -> Result<(), ReplayCacheError> {
        if response.len() > self.config.max_response_bytes {
            return Err(ReplayCacheError::ResponseTooLarge);
        }
        let byte_len = request
            .len()
            .checked_add(response.len())
            .ok_or(ReplayCacheError::ResponseTooLarge)?;
        let Some(reservation) = self.reservation else {
            return Err(ReplayCacheError::ReservationMissing);
        };
        if reservation.request_id != request_id
            || reservation.request_bytes != request.len()
            || self
                .storage
                .get(reservation.request_offset..reservation.request_offset + request.len())
                != Some(request)
        {
            return Err(ReplayCacheError::ReservationMissing);
        }
        let len = self.completed_len();
        let retained_bytes = self.completed_retained_bytes();
        if len >= self.entries.len() || retained_bytes.saturating_add(byte_len) > self.storage.len()
        {
            return Err(ReplayCacheError::ReservationMissing);
        }
        self.reservation = None;
        let request_offset = reservation.request_offset;
        let response_offset = request_offset + request.len();
        self.storage[response_offset..response_offset + response.len()].copy_from_slice(response);
        self.entries[len] = ReplayEntry {
            request_id: request_id.get(),
            request_offset,
            request_bytes: request.len(),
            response_offset,
            response_bytes: response.len(),
            _resource_layout_reservation: [0; 16],
        };
        Ok(())
    }

    /// Borrow the exact cached response bytes represented by a hit from [`Self::preflight`].
    ///
    /// Returns an empty slice for a stale or foreign hit. Use [`Self::try_cached`] when an empty
    /// cached response must be distinguished from invalid handle use.
    #[must_use]
    pub fn cached(&self, hit: ReplayHit) -> &[u8] {
        self.try_cached(hit).unwrap_or_default()
    }

    /// Fallibly borrow a cached response, rejecting evicted and cross-cache hits.
    #[must_use]
    pub fn try_cached(&self, hit: ReplayHit) -> Option<&[u8]> {
        if hit.cache_id != self.cache_id {
            return None;
        }
        let entry = self.entries.get(self.entry_index(hit.request_id).ok()?)?;
        self.storage
            .get(entry.response_offset..entry.response_offset.checked_add(entry.response_bytes)?)
    }

    /// Number of complete exact-byte entries currently retained.
    #[must_use]
    pub fn len(&self) -> usize {
        self.completed_len()
    }

    /// Whether no replay entries are retained.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.completed_len() == 0
    }

    /// Current retained exact-byte count.
    #[must_use]
    pub fn retained_bytes(&self) -> usize {
        self.completed_retained_bytes()
    }

    /// Return the immutable effective replay capacity configuration.
    #[must_use]
    pub const fn config(&self) -> ReplayCacheConfig {
        self.config
    }

    /// Inspect eager metadata and byte-arena capacities without exposing replay contents.
    #[must_use]
    pub fn retained_storage_capacities(&self) -> (usize, usize) {
        (self.entries.len(), self.storage.len())
    }
}

impl ReplayEntry {
    fn byte_len(&self) -> usize {
        self.request_bytes.saturating_add(self.response_bytes)
    }
}

/// A replay-cache invariant failure. Production controller use maps this to bounded internal
/// failure because preflight reservation should make it unreachable for conforming providers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayCacheError {
    /// A provider exceeded the configured response-byte bound.
    ResponseTooLarge,
    /// Completion was called without a sufficient preflight reservation.
    ReservationMissing,
    /// A configured retained payload layout or checked sum is not representable.
    ResourceOverflow,
    /// Eager replay metadata or byte-arena allocation failed before publication.
    ResourceAllocation,
}

impl Clone for ReplayCache {
    fn clone(&self) -> Self {
        self.try_clone_eager()
            .expect("cloning an accepted replay cache eagerly allocates its declared capacity")
    }
}

impl fmt::Display for ReplayCacheError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for ReplayCacheError {}

/// Mockable non-render provider surface for deferred parameter/transport/diagnostic ownership.
/// All setter operations use complete absolute values and are therefore idempotent.
pub trait ControlProvider {
    /// Return the endpoint-owned current absolute sample used for automation past-time admission.
    fn current_sample(&mut self) -> SampleTime;
    /// Return a bounded typed metadata page for this exact cursor and limit.
    fn parameter_metadata(
        &mut self,
        request: ParameterMetadataRequest,
    ) -> Result<ParameterMetadataPage, ParameterProviderError>;
    /// Return a bounded typed state page preserving requested handle order.
    fn parameter_state(
        &mut self,
        request: &ParameterStateRequest,
    ) -> Result<ParameterStatePage, ParameterProviderError>;
    /// Borrow one typed descriptor for transient automation domain admission.
    fn parameter_descriptor(
        &mut self,
        handle: ParameterHandle,
    ) -> Result<&ParameterDescriptor, ParameterProviderError>;
    /// Return a typed nondestructive counter snapshot for the requested registered IDs.
    fn counters(
        &mut self,
        request: &CountersRequest,
    ) -> Result<CounterSnapshot, ParameterProviderError>;
    /// Record explicitly canceled accepted automation. The counter is saturating and never reset.
    fn record_canceled_automation(&mut self, _records: u64) {}
    /// Return one bounded typed nondestructive diagnostics page.
    fn diagnostics(
        &mut self,
        request: DiagnosticsRequest,
    ) -> Result<DiagnosticsPage, ParameterProviderError>;
    /// Observe one fully typed transport snapshot at the provider's endpoint sample.
    fn transport_get(&mut self) -> TransportSnapshot;
    /// Apply an idempotent absolute typed transport set and return its effective snapshot.
    fn transport_set(&mut self, request: TransportSetRequest) -> TransportSnapshot;
    /// Persist and echo one complete typed endpoint-local telemetry configuration idempotently.
    fn telemetry_configure(
        &mut self,
        configuration: TelemetryConfiguration,
    ) -> TelemetryConfiguration;
}

/// Typed bounded provider fixture failure; providers never return raw BTLV payload bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParameterProviderError {
    /// The requested revision-scoped parameter handle is absent.
    NotFound,
    /// The typed provider is temporarily unavailable.
    Unavailable,
    /// A bounded diagnostic cursor predates retained provider history.
    ReplayExpired,
    /// The provider's bounded catalog or page limit was exceeded.
    LimitExceeded,
}

/// Bounded typed fixture input accepted by [`MockProvider::new`].
#[derive(Clone, Debug)]
pub struct MockProviderConfig {
    /// Endpoint current sample.
    pub current_sample: SampleTime,
    /// Typed bounded parameter descriptor catalog.
    pub parameter_metadata: Vec<crate::ParameterDescriptor>,
    /// Typed bounded parameter-state snapshot.
    pub parameter_state: ParameterStatePage,
    /// Typed sorted counter snapshot.
    pub counter_snapshot: CounterSnapshot,
    /// Typed ascending retained diagnostics.
    pub diagnostics: Vec<crate::Diagnostic>,
    /// Typed absolute transport state and position.
    pub transport_state: TransportState,
    /// Absolute transport position paired with `transport_state`.
    pub transport_position: SampleTime,
}

/// Small deterministic provider suitable for protocol conformance fixtures.
#[derive(Clone, Debug)]
pub struct MockProvider {
    /// Endpoint-owned current absolute sample for bounded automation admission fixtures.
    current_sample: SampleTime,
    /// Bounded typed metadata fixture catalog.
    parameter_metadata: Vec<crate::ParameterDescriptor>,
    /// Eager endpoint-owned descriptor used for accepted automation without changing the
    /// enumerable metadata catalog.
    automation_parameter: Option<crate::ParameterDescriptor>,
    /// Bounded typed state fixture records.
    parameter_state: ParameterStatePage,
    /// Typed nondestructive registered counter values.
    counter_snapshot: CounterSnapshot,
    /// Bounded retained typed diagnostic history.
    diagnostics: Vec<crate::Diagnostic>,
    /// Current absolute state.
    transport_state: TransportState,
    /// Current absolute position.
    transport_position: SampleTime,
    /// Last endpoint-local complete typed telemetry configuration.
    telemetry_configuration: TelemetryConfiguration,
}

impl Default for MockProvider {
    fn default() -> Self {
        Self {
            current_sample: SampleTime(0),
            parameter_metadata: Vec::new(),
            automation_parameter: None,
            parameter_state: ParameterStatePage {
                observed_sample: 0,
                records: Vec::new(),
            },
            counter_snapshot: CounterSnapshot {
                observed_sample: SampleTime(0),
                values: Vec::new(),
            },
            diagnostics: Vec::new(),
            transport_state: TransportState::Stopped,
            transport_position: SampleTime(0),
            telemetry_configuration: TelemetryConfiguration {
                meter_handles: Vec::new(),
                meter_period_blocks: 0,
                counter_ids: Vec::new(),
                counter_period_blocks: 0,
                diagnostics_enabled: false,
                minimum_diagnostic_severity: crate::DiagnosticSeverity::Info,
            },
        }
    }
}

impl MockProvider {
    /// Construct an empty provider with eager retained mutable collection capacities.
    pub fn try_with_retained_capacity(
        capacity: ControllerRetainedCapacity,
    ) -> Result<Self, ControllerResourceAllocationError> {
        let mut provider = Self {
            telemetry_configuration: retained_telemetry_configuration(capacity)?,
            ..Self::default()
        };
        provider
            .counter_snapshot
            .values
            .try_reserve_exact(capacity.counter_ids)
            .map_err(|_| ControllerResourceAllocationError)?;
        Ok(provider)
    }

    /// Inspect eager mutable retained capacities without exposing provider state mutation.
    #[must_use]
    pub fn retained_capacities(&self) -> (ControllerRetainedCapacity, usize) {
        (
            ControllerRetainedCapacity {
                meter_handles: self.telemetry_configuration.meter_handles.capacity(),
                counter_ids: self.telemetry_configuration.counter_ids.capacity(),
            },
            self.counter_snapshot.values.capacity(),
        )
    }

    /// Construct an empty enumerable provider with one eager automation-only descriptor.
    pub fn try_with_retained_capacity_and_automation(
        capacity: ControllerRetainedCapacity,
        descriptor: crate::ParameterDescriptor,
    ) -> Result<Self, ControllerResourceAllocationError> {
        let codec = ProtocolCodec::default();
        codec
            .encoded_parameter_metadata_page_len(&ParameterMetadataPage {
                last_handle: descriptor.handle,
                eof: true,
                descriptors: vec![descriptor.clone()],
            })
            .map_err(|_| ControllerResourceAllocationError)?;
        let mut provider = Self::try_with_retained_capacity(capacity)?;
        provider.automation_parameter = Some(descriptor);
        Ok(provider)
    }

    /// Construct deterministic typed fixtures only after every configured collection is bounded.
    pub fn new(config: MockProviderConfig) -> Result<Self, ParameterProviderError> {
        let validation_codec = ProtocolCodec::default();
        if config.parameter_metadata.len() > 256
            || config.parameter_state.records.len() > 256
            || config.counter_snapshot.values.len() > crate::CounterId::ValidationFailures as usize
            || config.diagnostics.len() > 256
            || config
                .parameter_metadata
                .windows(2)
                .any(|pair| pair[0].handle >= pair[1].handle)
            || config
                .diagnostics
                .windows(2)
                .any(|pair| pair[0].provider_sequence >= pair[1].provider_sequence)
            || config
                .diagnostics
                .iter()
                .any(|item| item.provider_sequence.is_none())
        {
            return Err(ParameterProviderError::LimitExceeded);
        }
        // Metadata and diagnostics are paginated. Validate each independently encodable item
        // instead of requiring an otherwise valid 256-item catalog/history to fit one frame.
        if config.parameter_metadata.iter().any(|descriptor| {
            validation_codec
                .encoded_parameter_metadata_page_len(&ParameterMetadataPage {
                    last_handle: descriptor.handle,
                    eof: true,
                    descriptors: vec![descriptor.clone()],
                })
                .is_err()
        }) || validation_codec
            .encoded_parameter_state_page_len(&config.parameter_state)
            .is_err()
            || validation_codec
                .encoded_counter_snapshot_len(&config.counter_snapshot)
                .is_err()
            || config.diagnostics.iter().any(|diagnostic| {
                validation_codec
                    .encoded_diagnostics_page_len(&DiagnosticsPage {
                        last_sequence: diagnostic
                            .provider_sequence
                            .expect("presence was checked above"),
                        eof: true,
                        diagnostics: vec![diagnostic.clone()],
                    })
                    .is_err()
            })
        {
            return Err(ParameterProviderError::LimitExceeded);
        }
        Ok(Self {
            current_sample: config.current_sample,
            parameter_metadata: config.parameter_metadata,
            automation_parameter: None,
            parameter_state: config.parameter_state,
            counter_snapshot: config.counter_snapshot,
            diagnostics: config.diagnostics,
            transport_state: config.transport_state,
            transport_position: config.transport_position,
            telemetry_configuration: TelemetryConfiguration {
                meter_handles: Vec::new(),
                meter_period_blocks: 0,
                counter_ids: Vec::new(),
                counter_period_blocks: 0,
                diagnostics_enabled: false,
                minimum_diagnostic_severity: crate::DiagnosticSeverity::Info,
            },
        })
    }
}

impl ControlProvider for MockProvider {
    fn current_sample(&mut self) -> SampleTime {
        self.current_sample
    }

    fn parameter_metadata(
        &mut self,
        request: ParameterMetadataRequest,
    ) -> Result<ParameterMetadataPage, ParameterProviderError> {
        if self.parameter_metadata.len() > 256 {
            return Err(ParameterProviderError::LimitExceeded);
        }
        let descriptors = self
            .parameter_metadata
            .iter()
            .filter(|descriptor| descriptor.handle > request.after_handle)
            .take(request.limit as usize)
            .cloned()
            .collect::<Vec<_>>();
        let last_handle = descriptors
            .last()
            .map_or(request.after_handle, |descriptor| descriptor.handle);
        let eof = self
            .parameter_metadata
            .iter()
            .all(|descriptor| descriptor.handle <= last_handle);
        Ok(ParameterMetadataPage {
            last_handle,
            eof,
            descriptors,
        })
    }
    fn parameter_state(
        &mut self,
        request: &ParameterStateRequest,
    ) -> Result<ParameterStatePage, ParameterProviderError> {
        if self.parameter_state.records.len() > 256 {
            return Err(ParameterProviderError::LimitExceeded);
        }
        let mut records = Vec::with_capacity(request.handles.len());
        for handle in &request.handles {
            let Some(record) = self
                .parameter_state
                .records
                .iter()
                .find(|record| record.handle == *handle)
            else {
                return Err(ParameterProviderError::NotFound);
            };
            records.push(*record);
        }
        Ok(ParameterStatePage {
            observed_sample: self.parameter_state.observed_sample,
            records,
        })
    }
    fn parameter_descriptor(
        &mut self,
        handle: ParameterHandle,
    ) -> Result<&ParameterDescriptor, ParameterProviderError> {
        self.parameter_metadata
            .iter()
            .find(|descriptor| descriptor.handle == handle.0)
            .or_else(|| {
                self.automation_parameter
                    .as_ref()
                    .filter(|descriptor| descriptor.handle == handle.0)
            })
            .ok_or(ParameterProviderError::NotFound)
    }
    fn counters(
        &mut self,
        request: &CountersRequest,
    ) -> Result<CounterSnapshot, ParameterProviderError> {
        if request.all {
            return Ok(self.counter_snapshot.clone());
        }
        let mut values = Vec::with_capacity(request.ids.len());
        for id in &request.ids {
            let Some(value) = self
                .counter_snapshot
                .values
                .iter()
                .find(|value| value.id as u32 == *id)
            else {
                return Err(ParameterProviderError::NotFound);
            };
            values.push(*value);
        }
        Ok(CounterSnapshot {
            observed_sample: self.counter_snapshot.observed_sample,
            values,
        })
    }
    fn record_canceled_automation(&mut self, records: u64) {
        let id = crate::CounterId::CanceledAutomation;
        if let Some(value) = self
            .counter_snapshot
            .values
            .iter_mut()
            .find(|value| value.id == id)
        {
            value.value = value.value.saturating_add(records);
        } else {
            let insert = self
                .counter_snapshot
                .values
                .iter()
                .position(|value| value.id > id)
                .unwrap_or(self.counter_snapshot.values.len());
            self.counter_snapshot
                .values
                .insert(insert, crate::CounterValue { id, value: records });
        }
    }
    fn diagnostics(
        &mut self,
        request: DiagnosticsRequest,
    ) -> Result<DiagnosticsPage, ParameterProviderError> {
        if self.diagnostics.len() > 256
            || self
                .diagnostics
                .windows(2)
                .any(|pair| pair[0].provider_sequence >= pair[1].provider_sequence)
            || self
                .diagnostics
                .iter()
                .any(|item| item.provider_sequence.is_none())
        {
            return Err(ParameterProviderError::LimitExceeded);
        }
        if let Some(first) = self
            .diagnostics
            .first()
            .and_then(|item| item.provider_sequence)
            && request.after_sequence != 0
            // A cursor immediately before the retained first entry is still replayable.  Only
            // an actual gap in the bounded history is expired; zero retains its specified
            // "before first" meaning even when the first provider sequence is greater than one.
            && request.after_sequence.saturating_add(1) < first
        {
            return Err(ParameterProviderError::ReplayExpired);
        }
        let diagnostics = self
            .diagnostics
            .iter()
            .filter(|item| {
                item.provider_sequence
                    .is_some_and(|sequence| sequence > request.after_sequence)
                    && (item.severity as u8) >= request.minimum_severity as u8
            })
            .take(request.limit as usize)
            .cloned()
            .collect::<Vec<_>>();
        let last_sequence = diagnostics
            .last()
            .and_then(|item| item.provider_sequence)
            .unwrap_or(request.after_sequence);
        let eof = self
            .diagnostics
            .iter()
            .find(|item| {
                item.provider_sequence
                    .is_some_and(|sequence| sequence > last_sequence)
                    && (item.severity as u8) >= request.minimum_severity as u8
            })
            .is_none();
        Ok(DiagnosticsPage {
            last_sequence,
            eof,
            diagnostics,
        })
    }
    fn transport_get(&mut self) -> TransportSnapshot {
        TransportSnapshot {
            state: self.transport_state,
            position: self.transport_position,
            effective_sample: self.current_sample,
        }
    }
    fn transport_set(&mut self, request: TransportSetRequest) -> TransportSnapshot {
        self.transport_state = request.state;
        if let Some(position) = request.position {
            self.transport_position = position;
        }
        self.transport_get()
    }
    fn telemetry_configure(
        &mut self,
        configuration: TelemetryConfiguration,
    ) -> TelemetryConfiguration {
        self.telemetry_configuration.meter_handles.clear();
        self.telemetry_configuration
            .meter_handles
            .extend_from_slice(&configuration.meter_handles);
        self.telemetry_configuration.counter_ids.clear();
        self.telemetry_configuration
            .counter_ids
            .extend_from_slice(&configuration.counter_ids);
        self.telemetry_configuration.meter_period_blocks = configuration.meter_period_blocks;
        self.telemetry_configuration.counter_period_blocks = configuration.counter_period_blocks;
        self.telemetry_configuration.diagnostics_enabled = configuration.diagnostics_enabled;
        self.telemetry_configuration.minimum_diagnostic_severity =
            configuration.minimum_diagnostic_severity;
        configuration
    }
}

/// A typed control command after BTLV header validation. It deliberately has no PCM/media case.
#[allow(missing_docs)] // Variant names and fields are the frozen command schema registry.
#[allow(clippy::large_enum_variant)] // A fixed Copy automation slot must not be heap-indirected.
pub enum ControlCommand<'a> {
    CapabilitiesGet,
    SessionSnapshotGet {
        offset: u64,
        max_bytes: u32,
    },
    SessionTransactionApply {
        edits: &'a [SessionEdit],
    },
    ParameterMetadataGet {
        request: ParameterMetadataRequest,
    },
    ParameterStateGet {
        request: ParameterStateRequest,
    },
    AutomationEnqueue {
        batch: AutomationBatchSlot,
    },
    TransportGet,
    TransportSet {
        request: TransportSetRequest,
    },
    TelemetryConfigure {
        configuration: TelemetryConfiguration,
    },
    CountersGet {
        request: CountersRequest,
    },
    DiagnosticsGet {
        request: DiagnosticsRequest,
    },
}

impl ControlCommand<'_> {
    fn requires_exact_revision(&self) -> bool {
        matches!(
            self,
            Self::SessionTransactionApply { .. }
                | Self::AutomationEnqueue { .. }
                | Self::TransportSet { .. }
                | Self::TelemetryConfigure { .. }
        )
    }

    const fn message_id(&self) -> MessageId {
        match self {
            Self::CapabilitiesGet => MessageId::CapabilitiesGet,
            Self::SessionSnapshotGet { .. } => MessageId::SessionSnapshotGet,
            Self::SessionTransactionApply { .. } => MessageId::SessionTransactionApply,
            Self::ParameterMetadataGet { .. } => MessageId::ParameterMetadataGet,
            Self::ParameterStateGet { .. } => MessageId::ParameterStateGet,
            Self::AutomationEnqueue { .. } => MessageId::AutomationEnqueue,
            Self::TransportGet => MessageId::TransportGet,
            Self::TransportSet { .. } => MessageId::TransportSet,
            Self::TelemetryConfigure { .. } => MessageId::TelemetryConfigure,
            Self::CountersGet { .. } => MessageId::CountersGet,
            Self::DiagnosticsGet { .. } => MessageId::DiagnosticsGet,
        }
    }
}

/// One internally decoded request. `canonical_bytes` are retained exactly for replay comparison.
pub struct ControllerRequest<'a> {
    /// Endpoint request correlation identity.
    pub request_id: RequestId,
    /// Header revision mode, checked before command execution.
    pub expected_revision: ExpectedRevision,
    /// Exact canonical BTLV request frame bytes, borrowed for this call only.
    pub canonical_bytes: &'a [u8],
    /// Typed command derived from that same canonical input.
    pub command: ControlCommand<'a>,
}

/// An owned response cached exactly for replay and later encoded by the BTLV response schema.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControllerResponse {
    /// Correlation identity echoed from the processed request.
    pub request_id: RequestId,
    /// Exact typed status.
    pub status: StatusCode,
    /// Observed or newly committed authoritative revision.
    pub revision: SessionRevision,
    /// One complete canonical response frame retained for exact replay and payload projection.
    /// This is the response's sole byte backing allocation.
    pub frame: Vec<u8>,
}

#[derive(Clone, Copy)]
struct CapabilitySet {
    commands: [u16; 11],
    command_len: u8,
    events: [u16; 6],
    event_len: u8,
    flags: CapabilityFlags,
}

enum Body {
    Capabilities(CapabilitySet),
    Snapshot {
        total: u64,
        offset: u64,
        range: Range<usize>,
        eof: bool,
    },
    TransactionApplied(TransactionApplied),
    ParameterMetadata(ParameterMetadataPage),
    ParameterState(ParameterStatePage),
    AutomationEnqueued(AutomationEnqueued),
    Transport(TransportSnapshot),
    Telemetry(TelemetryConfiguration),
    Counters(CounterSnapshot),
    Diagnostics(DiagnosticsPage),
    NonOk(NonOkResponse),
}

struct Outcome {
    status: StatusCode,
    revision: SessionRevision,
    body: Body,
}

impl ControllerResponse {
    fn from_complete_frame(
        request_id: RequestId,
        status: StatusCode,
        revision: SessionRevision,
        frame: Vec<u8>,
    ) -> Self {
        Self {
            request_id,
            status,
            revision,
            frame,
        }
    }

    fn payload(&self) -> &[u8] {
        self.frame
            .get(crate::OUTER_HEADER_BYTES..)
            .unwrap_or_default()
    }

    fn complete_frame(&self) -> &[u8] {
        &self.frame
    }

    /// Return the exact caller-buffer byte count for the canonical typed response payload.
    #[must_use]
    pub const fn payload_len(&self) -> usize {
        self.frame.len().saturating_sub(crate::OUTER_HEADER_BYTES)
    }

    /// Copy the already canonical typed response into caller-owned output without exposing a raw
    /// provider payload API.
    pub fn encode_payload(&self, output: &mut [u8]) -> Result<usize, crate::EncodeError> {
        let payload = self.payload();
        let required = payload.len();
        if output.len() < required {
            return Err(crate::EncodeError::OutputTooSmall { required });
        }
        output[..required].copy_from_slice(payload);
        Ok(required)
    }
}

/// A full-frame command could not be processed without a safe correlation response.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommandFrameProcessError {
    /// The outer header did not establish a valid command request ID, so no response was made.
    Uncorrelatable(DecodeError),
    /// The caller output cannot hold the exact canonical response; it was not modified.
    Encode(EncodeError),
    /// A new command's caller output cannot hold the endpoint's advertised response reservation.
    /// No replay admission, dispatch, or mutation occurred.
    OutputReservationTooSmall {
        /// Advertised endpoint response reservation required before dispatch.
        required: usize,
    },
    /// A framed replay entry unexpectedly lacked its exact full response bytes.
    Internal,
    /// One affine structural preparation still owns this controller generation.
    PreparedCommandOutstanding,
}

/// One closed command-preparation outcome. Immediate outcomes have already completed the accepted
/// one-call behavior; structural outcomes own all prospective state and remain invisible.
pub enum PreparedCommandFrame {
    /// No external plan is required; these exact canonical bytes are ready for caller output.
    Immediate(PreparedImmediateCommandFrame),
    /// A valid structural replacement awaits external plan preparation and reservation.
    Structural(Box<PreparedStructuralCommand>),
}

/// Exact canonical response bytes for a token-free immediate decision.
pub struct PreparedImmediateCommandFrame {
    bytes: Vec<u8>,
}

impl PreparedImmediateCommandFrame {
    /// Exact response bytes already accepted by the controller.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Whether the exact response is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Atomically copy the complete canonical response to caller output.
    pub fn write_into(&self, output: &mut [u8]) -> Result<usize, EncodeError> {
        copy_complete_frame(&self.bytes, output)
    }
}

struct StructuralCommandPlan {
    request_id: RequestId,
    previous_revision: SessionRevision,
    prospective_session: PreparedSessionTransaction,
    replay_plan: ReplayPreflightPlan,
    event_sequence: u64,
    cancellation_batches: usize,
    required_events: usize,
    applied_operations: u32,
    response_measure: crate::btlv::MessageMeasure,
}

// Keeping the structural plan inline avoids introducing a heap allocation into the direct
// one-call transaction path merely to reduce this short-lived dispatch value's stack size.
#[allow(clippy::large_enum_variant)]
enum StructuralCommandDisposition {
    Legacy,
    Immediate {
        replay_plan: ReplayPreflightPlan,
        outcome: Outcome,
    },
    Structural(StructuralCommandPlan),
}

/// Opaque affine structural preparation bound to one exact controller identity and generation.
/// It is deliberately neither `Clone` nor publicly constructible.
pub struct PreparedStructuralCommand {
    owner: Arc<AtomicU64>,
    generation: u64,
    request_id: RequestId,
    previous_revision: SessionRevision,
    prospective_session: Option<PreparedSessionTransaction>,
    prospective_replay: Option<ReplayCache>,
    response: Option<ControllerResponse>,
    event_reservations: Option<crate::ReliableEventReservations>,
    event_sequence: u64,
    cancellation_batches: usize,
    applied_operations: u32,
    armed: bool,
    // The CAPI resource contract counts this public opaque token's established retained layout.
    // ReplayCache itself is compact in the live controller; keep the prepared-token authority
    // stable until that external resource contract can be revised in its own issue.
    _resource_layout_reservation: [u8; 24],
}

impl PreparedStructuralCommand {
    /// Borrow the complete prospective session compilation for downstream source/plan binding.
    #[must_use]
    pub fn prospective_session(&self) -> &PreparedSessionTransaction {
        self.prospective_session
            .as_ref()
            .expect("live affine token owns prospective session")
    }

    /// Exact canonical response byte count retained for post-publication caller output.
    #[must_use]
    pub fn response_len(&self) -> usize {
        self.response
            .as_ref()
            .map_or(0, |response| response.frame.len())
    }
}

impl Drop for PreparedStructuralCommand {
    fn drop(&mut self) {
        if self.armed {
            let _ = self.owner.compare_exchange(
                self.generation,
                self.generation.saturating_add(1),
                Ordering::AcqRel,
                Ordering::Acquire,
            );
            self.armed = false;
        }
    }
}

/// A committed structural response kept private until the caller has published its matched plan.
pub struct CommittedCommandFrame {
    bytes: Vec<u8>,
}

impl CommittedCommandFrame {
    /// Exact committed response length.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Whether the response is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Atomically copy the complete committed response to caller output.
    pub fn write_into(&self, output: &mut [u8]) -> Result<usize, EncodeError> {
        copy_complete_frame(&self.bytes, output)
    }
}

/// Affinity/use rejection before the non-fallible prepared state application begins.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreparedCommandCommitError {
    /// The token belongs to a different exact controller allocation.
    WrongController,
    /// Controller state changed or this generation was canceled/consumed.
    StaleGeneration,
}

/// Bounded event-egress outcome. The caller always owns output; short output leaves the exact
/// pending event intact for a later retry and no partial frame is written.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventEgressError {
    /// The event family is not enabled by endpoint capabilities/configuration.
    Disabled,
    /// Prepared reliable diagnostic storage has no free slot.
    DiagnosticStorageFull,
    /// A reliable queue rejected the original event without dropping or coalescing it.
    ReliableQueueFull(QueueReport),
    /// The typed caller-buffer encoder rejected the requested event/output.
    Encode(EncodeError),
}

impl fmt::Display for EventEgressError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for EventEgressError {}

const EMPTY_METER_RECORD: MeterRecord = MeterRecord {
    handle: 1,
    component: crate::MeterComponent::Left,
    flags: 0,
    value: 0.0,
};

const EMPTY_COUNTER_VALUE: CounterValue = CounterValue {
    id: crate::CounterId::ControlCommandBackpressure,
    value: 0,
};

/// One already-drained telemetry batch retained across a short caller output buffer.
#[derive(Clone, Copy)]
#[allow(clippy::large_enum_variant)] // Preallocated controller state preserves no-allocation retry.
enum PendingTelemetryEvent {
    None,
    Meter {
        revision: SessionRevision,
        observed_sample: SampleTime,
        len: u16,
        records: [MeterRecord; crate::AUTOMATION_BATCH_RECORDS],
    },
    Counter {
        revision: SessionRevision,
        observed_sample: SampleTime,
        len: u16,
        values: [CounterValue; crate::AUTOMATION_BATCH_RECORDS],
    },
}

/// One eagerly allocated reliable-diagnostic storage cell.
enum RetainedDiagnosticSlot {
    Empty,
    Owned(Diagnostic),
}

impl RetainedDiagnosticSlot {
    const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

impl fmt::Display for CommandFrameProcessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for CommandFrameProcessError {}

/// Control-plane dispatcher that owns one logical endpoint lifetime's session, queues, and replay.
pub struct ProtocolController<P: ControlProvider> {
    session: SessionStore,
    queues: ProtocolQueues,
    provider: P,
    replay: ReplayCache,
    codec: ProtocolCodec,
    config: ProtocolControllerConfig,
    next_reliable_event_sequence: u64,
    telemetry_configuration: TelemetryConfiguration,
    diagnostic_event_slots: Box<[RetainedDiagnosticSlot]>,
    pending_reliable_event: Option<ReliableSlot>,
    pending_meter_record: Option<TelemetryRecord>,
    pending_counter_record: Option<CounterTelemetryRecord>,
    pending_telemetry_event: PendingTelemetryEvent,
    structural_generation: Arc<AtomicU64>,
}

impl<P: ControlProvider> ProtocolController<P> {
    /// Construct an endpoint after eagerly allocating every mutable retained controller vector.
    pub fn try_with_config_and_retained_capacity(
        session: SessionStore,
        queues: ProtocolQueues,
        provider: P,
        replay: ReplayCache,
        codec: ProtocolCodec,
        config: ProtocolControllerConfig,
        capacity: ControllerRetainedCapacity,
    ) -> Result<Self, ControllerResourceAllocationError> {
        let diagnostic_slots = queues.config().reliable_event_slots.get();
        let mut diagnostics = Vec::new();
        diagnostics
            .try_reserve_exact(diagnostic_slots)
            .map_err(|_| ControllerResourceAllocationError)?;
        diagnostics.resize_with(diagnostic_slots, || RetainedDiagnosticSlot::Empty);
        Ok(Self {
            session,
            queues,
            provider,
            replay,
            codec,
            config,
            next_reliable_event_sequence: 1,
            telemetry_configuration: retained_telemetry_configuration(capacity)?,
            diagnostic_event_slots: diagnostics.into_boxed_slice(),
            pending_reliable_event: None,
            pending_meter_record: None,
            pending_counter_record: None,
            pending_telemetry_event: PendingTelemetryEvent::None,
            structural_generation: Arc::new(AtomicU64::new(0)),
        })
    }

    /// Inspect eager controller telemetry capacities without exposing mutable configuration.
    #[must_use]
    pub fn retained_configuration_capacity(&self) -> ControllerRetainedCapacity {
        ControllerRetainedCapacity {
            meter_handles: self.telemetry_configuration.meter_handles.capacity(),
            counter_ids: self.telemetry_configuration.counter_ids.capacity(),
        }
    }

    /// Bind a precompiled session, already prepared protocol queues, provider, and bounded replay
    /// cache into one control endpoint. This does not create a render plan.
    #[must_use]
    pub fn new(
        session: SessionStore,
        queues: ProtocolQueues,
        provider: P,
        replay: ReplayCache,
    ) -> Self {
        Self::with_codec(session, queues, provider, replay, ProtocolCodec::default())
    }

    /// Construct an endpoint whose typed BTLV payloads use these effective protocol limits.
    #[must_use]
    pub fn with_codec(
        session: SessionStore,
        queues: ProtocolQueues,
        provider: P,
        replay: ReplayCache,
        codec: ProtocolCodec,
    ) -> Self {
        Self::with_config(
            session,
            queues,
            provider,
            replay,
            codec,
            ProtocolControllerConfig::default(),
        )
    }

    /// Construct an endpoint with explicit typed controller bounds.
    #[must_use]
    pub fn with_config(
        session: SessionStore,
        queues: ProtocolQueues,
        provider: P,
        replay: ReplayCache,
        codec: ProtocolCodec,
        config: ProtocolControllerConfig,
    ) -> Self {
        Self::try_with_config_and_retained_capacity(
            session,
            queues,
            provider,
            replay,
            codec,
            config,
            ControllerRetainedCapacity {
                meter_handles: 0,
                counter_ids: 0,
            },
        )
        .expect("zero-capacity controller construction does not allocate telemetry vectors")
    }

    /// Process one logical request with exact-byte replay and no renderer call.
    pub fn process(&mut self, request: ControllerRequest<'_>) -> ControllerResponse {
        let message_id = request.command.message_id();
        if self.structural_generation.load(Ordering::Acquire) & 1 != 0 {
            return self.compatibility_response(
                message_id,
                request.request_id,
                self.non_ok(StatusCode::Backpressure, None),
            );
        }
        match self
            .replay
            .preflight(request.request_id, request.canonical_bytes)
        {
            ReplayDecision::Cached(hit) => {
                return self.compatibility_cached_response(message_id, request.request_id, hit);
            }
            ReplayDecision::RequestIdReuse => {
                return self.compatibility_response(
                    message_id,
                    request.request_id,
                    self.non_ok(StatusCode::RequestIdReuse, None),
                );
            }
            ReplayDecision::ReplayExpired => {
                return self.compatibility_response(
                    message_id,
                    request.request_id,
                    self.non_ok(StatusCode::ReplayExpired, None),
                );
            }
            ReplayDecision::Backpressure => {
                return self.compatibility_response(
                    message_id,
                    request.request_id,
                    self.replay_backpressure_outcome(request.canonical_bytes.len()),
                );
            }
            ReplayDecision::Execute => {}
        }
        let outcome = self.execute(&request);
        let response = self.compatibility_response(message_id, request.request_id, outcome);
        match self.replay.complete(
            request.request_id,
            request.canonical_bytes,
            response.complete_frame(),
        ) {
            Ok(()) => response,
            Err(_) => self.compatibility_response(
                message_id,
                request.request_id,
                self.non_ok(StatusCode::Internal, None),
            ),
        }
    }

    /// Decode one supported BTLV session transaction and immediately submit its typed edits to the
    /// same revisioned dispatcher. The decoded frame is borrowed only for this call.
    pub fn process_session_transaction_btlv(
        &mut self,
        codec: &ProtocolCodec,
        input: &[u8],
        scratch: &mut DecodeScratch<'_>,
    ) -> Result<ControllerResponse, DecodeError> {
        let decoded = codec.decode_session_transaction_limited(
            input,
            scratch,
            self.config.maximum_transaction_edits,
        )?;
        let header = decoded
            .frame
            .header
            .command()
            .ok_or(DecodeError::MessageKindMismatch)?;
        Ok(self.process(ControllerRequest {
            request_id: header.request_id,
            expected_revision: header.expected_revision,
            canonical_bytes: input,
            command: ControlCommand::SessionTransactionApply {
                edits: &decoded.edits,
            },
        }))
    }

    /// Decode and process the B1b typed command subset without exposing raw provider payloads.
    pub fn process_b1b_btlv(
        &mut self,
        input: &[u8],
        scratch: &mut DecodeScratch<'_>,
    ) -> Result<ControllerResponse, DecodeError> {
        let codec = self.codec;
        let decoded = codec.decode_typed_command_limited(
            input,
            scratch,
            self.config.maximum_transaction_edits,
        )?;
        let header = decoded.header;
        let command = match decoded.payload {
            DecodedCommandPayload::CapabilitiesGet => ControlCommand::CapabilitiesGet,
            DecodedCommandPayload::SessionSnapshotGet(request) => {
                ControlCommand::SessionSnapshotGet {
                    offset: request.offset,
                    max_bytes: request.maximum_bytes,
                }
            }
            DecodedCommandPayload::SessionTransactionApply(edits) => {
                return Ok(self.process(ControllerRequest {
                    request_id: header.request_id,
                    expected_revision: header.expected_revision,
                    canonical_bytes: input,
                    command: ControlCommand::SessionTransactionApply { edits: &edits },
                }));
            }
            DecodedCommandPayload::ParameterMetadataGet(request) => {
                ControlCommand::ParameterMetadataGet { request }
            }
            DecodedCommandPayload::ParameterStateGet(request) => {
                ControlCommand::ParameterStateGet { request }
            }
            DecodedCommandPayload::AutomationEnqueue(decoded) => {
                let revision = match header.expected_revision {
                    ExpectedRevision::Exact(revision) => revision,
                    ExpectedRevision::Any => SessionRevision(0),
                };
                ControlCommand::AutomationEnqueue {
                    batch: decoded.into_batch(revision, header.request_id)?,
                }
            }
            DecodedCommandPayload::TransportGet => ControlCommand::TransportGet,
            DecodedCommandPayload::TransportSet(request) => {
                ControlCommand::TransportSet { request }
            }
            DecodedCommandPayload::TelemetryConfigure(configuration) => {
                ControlCommand::TelemetryConfigure { configuration }
            }
            DecodedCommandPayload::CountersGet(request) => ControlCommand::CountersGet { request },
            DecodedCommandPayload::DiagnosticsGet(request) => {
                ControlCommand::DiagnosticsGet { request }
            }
        };
        Ok(self.process(ControllerRequest {
            request_id: header.request_id,
            expected_revision: header.expected_revision,
            canonical_bytes: input,
            command,
        }))
    }

    /// Process one complete schema-closed command and copy its canonical full response into the
    /// caller buffer.
    ///
    /// A malformed payload after a valid complete command header receives a correlatable common
    /// non-OK frame.  A malformed outer header has no safe request identity and instead returns
    /// [`CommandFrameProcessError::Uncorrelatable`].  Exact replay returns the original complete
    /// encoded response bytes without dispatching a second time. New commands require caller
    /// output at least as large as the advertised `maximum_cached_response_bytes` reservation;
    /// that pre-dispatch requirement is distinct from an exact codec frame length.
    pub fn process_command_frame_into(
        &mut self,
        input: &[u8],
        scratch: &mut DecodeScratch<'_>,
        output: &mut [u8],
    ) -> Result<usize, CommandFrameProcessError> {
        if self.structural_generation.load(Ordering::Acquire) & 1 != 0 {
            return Err(CommandFrameProcessError::PreparedCommandOutstanding);
        }
        let header = self
            .codec
            .decode_command_header(input)
            .map_err(CommandFrameProcessError::Uncorrelatable)?;
        match self.plan_structural_command(input, scratch, output.len(), header)? {
            StructuralCommandDisposition::Legacy => {
                self.process_command_frame_into_legacy(input, scratch, output)
            }
            StructuralCommandDisposition::Immediate {
                replay_plan,
                outcome,
            } => self.process_planned_immediate_into(input, header, replay_plan, outcome, output),
            StructuralCommandDisposition::Structural(plan) => {
                self.process_structural_plan_into(input, output, header, plan)
            }
        }
    }

    /// Prepare one complete command. Valid structural commands return an invisible affine token;
    /// every other accepted decision completes through the byte-identical one-call machinery.
    pub fn prepare_command_frame(
        &mut self,
        input: &[u8],
        scratch: &mut DecodeScratch<'_>,
        output_capacity: usize,
    ) -> Result<PreparedCommandFrame, CommandFrameProcessError> {
        if self.structural_generation.load(Ordering::Acquire) & 1 != 0 {
            return Err(CommandFrameProcessError::PreparedCommandOutstanding);
        }
        let header = self
            .codec
            .decode_command_header(input)
            .map_err(CommandFrameProcessError::Uncorrelatable)?;
        match self.plan_structural_command(input, scratch, output_capacity, header)? {
            StructuralCommandDisposition::Legacy => {
                self.process_immediate_command(input, scratch, output_capacity)
            }
            StructuralCommandDisposition::Immediate {
                replay_plan,
                outcome,
            } => {
                self.prepare_planned_immediate(input, header, replay_plan, outcome, output_capacity)
            }
            StructuralCommandDisposition::Structural(plan) => Ok(PreparedCommandFrame::Structural(
                Box::new(self.prepare_structural_plan(input, header, plan)?),
            )),
        }
    }

    fn prepare_structural_plan(
        &mut self,
        input: &[u8],
        header: CommandHeader,
        plan: StructuralCommandPlan,
    ) -> Result<PreparedStructuralCommand, CommandFrameProcessError> {
        let revision = plan.prospective_session.revision();
        let response_frame = TypedSuccessResponseFrame {
            request_id: header.request_id,
            revision,
            payload: SuccessResponsePayload::SessionTransactionApplied(TransactionApplied {
                applied_operations: plan.applied_operations,
            }),
        };
        #[cfg(test)]
        RESPONSE_STAGING_VECS.with(|allocations| {
            allocations.set(allocations.get().saturating_add(1));
        });
        let mut frame = vec![0_u8; self.replay.config.max_response_bytes];
        let written = self
            .codec
            .encode_success_response_frame_measured_into(
                &response_frame,
                plan.response_measure,
                &mut frame,
            )
            .map_err(CommandFrameProcessError::Encode)?;
        frame.truncate(written);
        let response = ControllerResponse::from_complete_frame(
            header.request_id,
            StatusCode::Ok,
            revision,
            frame,
        );
        let mut prospective_replay = self
            .replay
            .try_clone_eager()
            .map_err(|_| CommandFrameProcessError::Internal)?;
        if prospective_replay.preflight(header.request_id, input) != ReplayDecision::Execute {
            return Err(CommandFrameProcessError::Internal);
        }
        prospective_replay
            .complete(header.request_id, input, response.complete_frame())
            .map_err(|_| CommandFrameProcessError::Internal)?;
        let event_reservations = match self.queues.reserve_reliable_events(plan.required_events) {
            Ok(reservations) => reservations,
            Err(_) => return Err(CommandFrameProcessError::Internal),
        };

        let current = self.structural_generation.load(Ordering::Acquire);
        let generation = current
            .checked_add(1)
            .filter(|generation| generation & 1 == 1)
            .ok_or(CommandFrameProcessError::Internal)?;
        self.structural_generation
            .compare_exchange(current, generation, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| CommandFrameProcessError::PreparedCommandOutstanding)?;
        Ok(PreparedStructuralCommand {
            owner: Arc::clone(&self.structural_generation),
            generation,
            request_id: plan.request_id,
            previous_revision: plan.previous_revision,
            prospective_session: Some(plan.prospective_session),
            prospective_replay: Some(prospective_replay),
            response: Some(response),
            event_reservations: Some(event_reservations),
            event_sequence: plan.event_sequence,
            cancellation_batches: plan.cancellation_batches,
            applied_operations: plan.applied_operations,
            armed: true,
            _resource_layout_reservation: [0; 24],
        })
    }

    fn plan_structural_command(
        &self,
        input: &[u8],
        scratch: &mut DecodeScratch<'_>,
        output_capacity: usize,
        header: CommandHeader,
    ) -> Result<StructuralCommandDisposition, CommandFrameProcessError> {
        if header.message_id != MessageId::SessionTransactionApply {
            return Ok(StructuralCommandDisposition::Legacy);
        }
        let is_new_request = self.replay.is_new_request(header.request_id);
        if is_new_request && output_capacity < self.replay.config().max_response_bytes {
            return Err(CommandFrameProcessError::OutputReservationTooSmall {
                required: self.replay.config().max_response_bytes,
            });
        }
        if !is_new_request || self.replay.reservation.is_some() {
            return Ok(StructuralCommandDisposition::Legacy);
        }
        if !self.config.provider_features.session_events
            || self.config.maximum_transaction_edits == 0
            || !matches!(header.expected_revision, ExpectedRevision::Exact(_))
            || matches!(
                header.expected_revision,
                ExpectedRevision::Exact(revision) if revision != self.session.revision()
            )
        {
            return Ok(StructuralCommandDisposition::Legacy);
        }
        let replay_plan = match self.replay.plan_preflight(header.request_id, input) {
            Ok(plan) => plan,
            Err(_) => return Ok(StructuralCommandDisposition::Legacy),
        };

        #[cfg(test)]
        TYPED_COMMAND_DECODES.with(|decodes| decodes.set(decodes.get().saturating_add(1)));
        let decoded = match self.codec.decode_typed_command(input, scratch) {
            Ok(decoded) => decoded,
            Err(error) => {
                return Ok(StructuralCommandDisposition::Immediate {
                    replay_plan,
                    outcome: self.non_ok(error.status(), None),
                });
            }
        };
        let DecodedCommandPayload::SessionTransactionApply(edits) = decoded.payload else {
            return Ok(StructuralCommandDisposition::Immediate {
                replay_plan,
                outcome: self.non_ok(StatusCode::Internal, None),
            });
        };
        if edits.is_empty() {
            return Ok(StructuralCommandDisposition::Immediate {
                replay_plan,
                outcome: self.non_ok(StatusCode::InvalidField, None),
            });
        }
        if edits.len() > self.config.maximum_transaction_edits as usize {
            return Ok(StructuralCommandDisposition::Immediate {
                replay_plan,
                outcome: self.non_ok(StatusCode::LimitExceeded, None),
            });
        }
        let cancellation_batches =
            usize::try_from(self.queues.report(crate::QueueKind::Automation).occupancy)
                .unwrap_or(usize::MAX);
        let required_events = cancellation_batches.saturating_add(1);
        let event_report = self.queues.report(crate::QueueKind::ReliableEvent);
        if required_events
            > event_report
                .capacity
                .saturating_sub(usize::try_from(event_report.occupancy).unwrap_or(usize::MAX))
        {
            return Ok(StructuralCommandDisposition::Immediate {
                replay_plan,
                outcome: self.queue_backpressure_outcome(event_report),
            });
        }
        let event_sequence = self.next_reliable_event_sequence;
        if event_sequence
            .checked_add(u64::try_from(required_events).unwrap_or(u64::MAX))
            .is_none()
        {
            return Ok(StructuralCommandDisposition::Immediate {
                replay_plan,
                outcome: self.non_ok(StatusCode::Internal, None),
            });
        }
        let applied_operations = match u32::try_from(edits.len()) {
            Ok(value) => value,
            Err(_) => {
                return Ok(StructuralCommandDisposition::Immediate {
                    replay_plan,
                    outcome: self.non_ok(StatusCode::LimitExceeded, None),
                });
            }
        };
        let prospective_session = match self
            .session
            .prepare_transaction(header.expected_revision, &edits)
        {
            Ok(value) => value,
            Err(error) => {
                return Ok(StructuralCommandDisposition::Immediate {
                    replay_plan,
                    outcome: self.transaction_error_outcome(&error),
                });
            }
        };
        let revision = prospective_session.revision();
        let response_frame = TypedSuccessResponseFrame {
            request_id: header.request_id,
            revision,
            payload: SuccessResponsePayload::SessionTransactionApplied(TransactionApplied {
                applied_operations,
            }),
        };
        let response_measure = match self.codec.measure_success_response_frame(&response_frame) {
            Ok(measure) => measure,
            Err(_) => {
                return Ok(StructuralCommandDisposition::Immediate {
                    replay_plan,
                    outcome: self.non_ok(StatusCode::Internal, None),
                });
            }
        };
        let response_len = crate::OUTER_HEADER_BYTES
            .checked_add(response_measure.length)
            .ok_or(CommandFrameProcessError::Internal)?;
        if response_len > self.replay.config().max_response_bytes {
            return Ok(StructuralCommandDisposition::Immediate {
                replay_plan,
                outcome: self.non_ok(StatusCode::Internal, None),
            });
        }
        if response_len > output_capacity {
            return Err(CommandFrameProcessError::Encode(
                EncodeError::OutputTooSmall {
                    required: response_len,
                },
            ));
        }
        Ok(StructuralCommandDisposition::Structural(
            StructuralCommandPlan {
                request_id: header.request_id,
                previous_revision: self.session.revision(),
                prospective_session,
                replay_plan,
                event_sequence,
                cancellation_batches,
                required_events,
                applied_operations,
                response_measure,
            },
        ))
    }

    fn process_structural_plan_into(
        &mut self,
        input: &[u8],
        output: &mut [u8],
        header: CommandHeader,
        plan: StructuralCommandPlan,
    ) -> Result<usize, CommandFrameProcessError> {
        let revision = plan.prospective_session.revision();
        let response_frame = TypedSuccessResponseFrame {
            request_id: header.request_id,
            revision,
            payload: SuccessResponsePayload::SessionTransactionApplied(TransactionApplied {
                applied_operations: plan.applied_operations,
            }),
        };
        let written = self
            .codec
            .encode_success_response_frame_measured_into(
                &response_frame,
                plan.response_measure,
                output,
            )
            .map_err(CommandFrameProcessError::Encode)?;

        // Every fallible semantic, output, replay-capacity, and event-capacity decision completed
        // above. Exclusive `&mut self` ownership makes these exact reservations invariant from
        // this point through the session/event/replay commit.
        let mut event_reservations = self
            .queues
            .reserve_reliable_events(plan.required_events)
            .expect("read-only structural plan reserved exact reliable-event capacity");
        let effective_sample = self.provider.current_sample();
        self.replay
            .apply_preflight_plan(plan.request_id, input, plan.replay_plan);
        self.replay
            .complete(plan.request_id, input, &output[..written])
            .expect("planned replay reservation accepts the measured direct response");

        let commit = self.session.commit_prepared(plan.prospective_session);
        debug_assert_eq!(commit.applied_operations, plan.applied_operations as usize);
        self.queues.commit_reserved_reliable_event(
            &mut event_reservations,
            ReliableSlot::session_committed(
                commit.revision,
                plan.event_sequence,
                plan.request_id,
                plan.previous_revision,
                plan.applied_operations,
            ),
        );
        self.next_reliable_event_sequence = plan.event_sequence.saturating_add(1);
        self.cancel_queued_automation_reserved(
            &mut event_reservations,
            AutomationCancellationReason::RevisionChanged,
            Some(effective_sample),
        )
        .expect("planned cancellation events retain their exact reservations");
        Ok(written)
    }

    fn process_planned_immediate_into(
        &mut self,
        input: &[u8],
        header: CommandHeader,
        replay_plan: ReplayPreflightPlan,
        outcome: Outcome,
        output: &mut [u8],
    ) -> Result<usize, CommandFrameProcessError> {
        let (written, _, _) = self
            .encode_outcome_or_internal_into(header.message_id, header.request_id, outcome, output)
            .map_err(CommandFrameProcessError::Encode)?;
        self.replay
            .apply_preflight_plan(header.request_id, input, replay_plan);
        self.replay
            .complete(header.request_id, input, &output[..written])
            .expect("planned immediate replay reservation accepts measured response");
        Ok(written)
    }

    fn prepare_planned_immediate(
        &mut self,
        input: &[u8],
        header: CommandHeader,
        replay_plan: ReplayPreflightPlan,
        outcome: Outcome,
        output_capacity: usize,
    ) -> Result<PreparedCommandFrame, CommandFrameProcessError> {
        #[cfg(test)]
        PREPARED_IMMEDIATE_CALLS.with(|calls| calls.set(calls.get().saturating_add(1)));
        let capacity = output_capacity.min(self.replay.config().max_response_bytes);
        let mut bytes = vec![0_u8; capacity];
        let written =
            self.process_planned_immediate_into(input, header, replay_plan, outcome, &mut bytes)?;
        bytes.truncate(written);
        Ok(PreparedCommandFrame::Immediate(
            PreparedImmediateCommandFrame { bytes },
        ))
    }

    fn process_immediate_command(
        &mut self,
        input: &[u8],
        scratch: &mut DecodeScratch<'_>,
        output_capacity: usize,
    ) -> Result<PreparedCommandFrame, CommandFrameProcessError> {
        #[cfg(test)]
        PREPARED_IMMEDIATE_CALLS.with(|calls| calls.set(calls.get().saturating_add(1)));
        let capacity = output_capacity.min(self.replay.config().max_response_bytes);
        let mut bytes = vec![0_u8; capacity];
        let written = self.process_command_frame_into_legacy(input, scratch, &mut bytes)?;
        bytes.truncate(written);
        Ok(PreparedCommandFrame::Immediate(
            PreparedImmediateCommandFrame { bytes },
        ))
    }

    /// Validate token affinity/currentness, then apply every prepared controller mutation without
    /// another fallible semantic or capacity decision.
    pub fn commit_prepared_structural(
        &mut self,
        mut prepared: PreparedStructuralCommand,
    ) -> Result<CommittedCommandFrame, PreparedCommandCommitError> {
        if !Arc::ptr_eq(&self.structural_generation, &prepared.owner) {
            return Err(PreparedCommandCommitError::WrongController);
        }
        if self.structural_generation.load(Ordering::Acquire) != prepared.generation
            || self.session.revision() != prepared.previous_revision
            || usize::try_from(self.queues.report(crate::QueueKind::Automation).occupancy)
                .unwrap_or(usize::MAX)
                != prepared.cancellation_batches
        {
            return Err(PreparedCommandCommitError::StaleGeneration);
        }
        let mut event_reservations = prepared
            .event_reservations
            .take()
            .expect("affine token owns exact event reservations");
        let effective_sample = self.provider.current_sample();
        let commit = self.session.commit_prepared(
            prepared
                .prospective_session
                .take()
                .expect("affine token owns prospective session"),
        );
        debug_assert_eq!(
            commit.applied_operations,
            prepared.applied_operations as usize
        );
        self.queues.commit_reserved_reliable_event(
            &mut event_reservations,
            ReliableSlot::session_committed(
                commit.revision,
                prepared.event_sequence,
                prepared.request_id,
                prepared.previous_revision,
                prepared.applied_operations,
            ),
        );
        self.next_reliable_event_sequence = prepared.event_sequence.saturating_add(1);
        let _ = self.cancel_queued_automation_reserved(
            &mut event_reservations,
            AutomationCancellationReason::RevisionChanged,
            Some(effective_sample),
        );
        self.replay = prepared
            .prospective_replay
            .take()
            .expect("affine token owns prospective replay");
        let response = prepared
            .response
            .take()
            .expect("affine token owns committed response");
        let bytes = response.frame;
        self.structural_generation
            .store(prepared.generation.saturating_add(1), Ordering::Release);
        prepared.armed = false;
        Ok(CommittedCommandFrame { bytes })
    }

    fn process_command_frame_into_legacy(
        &mut self,
        input: &[u8],
        scratch: &mut DecodeScratch<'_>,
        output: &mut [u8],
    ) -> Result<usize, CommandFrameProcessError> {
        let header = self
            .codec
            .decode_command_header(input)
            .map_err(CommandFrameProcessError::Uncorrelatable)?;

        // A new command must prove that its caller owns enough room for the endpoint's whole
        // bounded response reservation before replay admission or any typed dispatch. This keeps
        // an output-size failure from accepting a request, advancing replay state, or committing
        // a session/queue/provider mutation. Cached and replay-status responses are handled by
        // their exact already-known sizes below.
        if self.replay.is_new_request(header.request_id) {
            let required = self.replay.config().max_response_bytes;
            if output.len() < required {
                return Err(CommandFrameProcessError::OutputReservationTooSmall { required });
            }
        }
        match self.replay.preflight(header.request_id, input) {
            ReplayDecision::Cached(hit) => {
                return copy_complete_frame(self.replay.cached(hit), output)
                    .map_err(CommandFrameProcessError::Encode);
            }
            ReplayDecision::RequestIdReuse => {
                return self.write_uncached_status_frame(
                    header,
                    StatusCode::RequestIdReuse,
                    output,
                );
            }
            ReplayDecision::ReplayExpired => {
                return self.write_uncached_status_frame(header, StatusCode::ReplayExpired, output);
            }
            ReplayDecision::Backpressure => {
                return self.write_uncached_replay_backpressure_frame(header, input.len(), output);
            }
            ReplayDecision::Execute => {}
        }

        #[cfg(test)]
        TYPED_COMMAND_DECODES.with(|decodes| decodes.set(decodes.get().saturating_add(1)));
        let outcome = match self.codec.decode_typed_command(input, scratch) {
            Ok(decoded) => self.execute_decoded_command(header, decoded.payload),
            Err(error) => self.non_ok(error.status(), None),
        };
        let (written, _, _) = self
            .encode_outcome_or_internal_into(header.message_id, header.request_id, outcome, output)
            .map_err(CommandFrameProcessError::Encode)?;
        self.replay
            .complete(header.request_id, input, &output[..written])
            .map_err(|_| CommandFrameProcessError::Internal)?;
        Ok(written)
    }

    fn write_uncached_status_frame(
        &self,
        header: CommandHeader,
        status: StatusCode,
        output: &mut [u8],
    ) -> Result<usize, CommandFrameProcessError> {
        self.encode_outcome_or_internal_into(
            header.message_id,
            header.request_id,
            self.non_ok(status, None),
            output,
        )
        .map(|(written, _, _)| written)
        .map_err(CommandFrameProcessError::Encode)
    }

    fn write_uncached_replay_backpressure_frame(
        &self,
        header: CommandHeader,
        request_bytes: usize,
        output: &mut [u8],
    ) -> Result<usize, CommandFrameProcessError> {
        self.encode_outcome_or_internal_into(
            header.message_id,
            header.request_id,
            self.replay_backpressure_outcome(request_bytes),
            output,
        )
        .map(|(written, _, _)| written)
        .map_err(CommandFrameProcessError::Encode)
    }

    fn execute_decoded_command(
        &mut self,
        header: CommandHeader,
        payload: DecodedCommandPayload<'_>,
    ) -> Outcome {
        let command = match payload {
            DecodedCommandPayload::CapabilitiesGet => ControlCommand::CapabilitiesGet,
            DecodedCommandPayload::SessionSnapshotGet(request) => {
                ControlCommand::SessionSnapshotGet {
                    offset: request.offset,
                    max_bytes: request.maximum_bytes,
                }
            }
            DecodedCommandPayload::SessionTransactionApply(edits) => {
                return self.execute(&ControllerRequest {
                    request_id: header.request_id,
                    expected_revision: header.expected_revision,
                    canonical_bytes: &[],
                    command: ControlCommand::SessionTransactionApply { edits: &edits },
                });
            }
            DecodedCommandPayload::ParameterMetadataGet(request) => {
                ControlCommand::ParameterMetadataGet { request }
            }
            DecodedCommandPayload::ParameterStateGet(request) => {
                ControlCommand::ParameterStateGet { request }
            }
            DecodedCommandPayload::AutomationEnqueue(value) => {
                let ExpectedRevision::Exact(revision) = header.expected_revision else {
                    return self.non_ok(StatusCode::InvalidField, None);
                };
                let batch = match value.into_batch(revision, header.request_id) {
                    Ok(batch) => batch,
                    Err(error) => {
                        return self.non_ok(error.status(), None);
                    }
                };
                ControlCommand::AutomationEnqueue { batch }
            }
            DecodedCommandPayload::TransportGet => ControlCommand::TransportGet,
            DecodedCommandPayload::TransportSet(request) => {
                ControlCommand::TransportSet { request }
            }
            DecodedCommandPayload::TelemetryConfigure(configuration) => {
                ControlCommand::TelemetryConfigure { configuration }
            }
            DecodedCommandPayload::CountersGet(request) => ControlCommand::CountersGet { request },
            DecodedCommandPayload::DiagnosticsGet(request) => {
                ControlCommand::DiagnosticsGet { request }
            }
        };
        self.execute(&ControllerRequest {
            request_id: header.request_id,
            expected_revision: header.expected_revision,
            canonical_bytes: &[],
            command,
        })
    }

    fn encode_outcome_into(
        &self,
        message_id: MessageId,
        request_id: RequestId,
        outcome: Outcome,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let Outcome {
            status,
            revision,
            body,
        } = outcome;
        if status != StatusCode::Ok {
            let Body::NonOk(payload) = body else {
                return Err(EncodeError::MessageKindMismatch);
            };
            return self.codec.encode_non_ok_response_frame_into(
                &TypedNonOkResponseFrame {
                    request_id,
                    revision,
                    message_id,
                    status,
                    payload: &payload,
                },
                output,
            );
        }
        macro_rules! encode_success {
            ($payload:expr) => {
                self.codec.encode_success_response_frame_into(
                    &TypedSuccessResponseFrame {
                        request_id,
                        revision,
                        payload: $payload,
                    },
                    output,
                )
            };
        }
        match (message_id, body) {
            (MessageId::CapabilitiesGet, Body::Capabilities(set)) => {
                encode_success!(SuccessResponsePayload::Capabilities(
                    self.capabilities(&set)
                ))
            }
            (
                MessageId::SessionSnapshotGet,
                Body::Snapshot {
                    total,
                    offset,
                    range,
                    eof,
                },
            ) => {
                let bytes = self.session.canonical_snapshot().as_bytes();
                let chunk = bytes.get(range).ok_or(EncodeError::LimitExceeded)?;
                encode_success!(SuccessResponsePayload::SessionSnapshot(SessionSnapshot {
                    total_bytes: total,
                    offset,
                    canonical_json_chunk: chunk,
                    eof,
                }))
            }
            (MessageId::SessionTransactionApply, Body::TransactionApplied(value)) => {
                encode_success!(SuccessResponsePayload::SessionTransactionApplied(value))
            }
            (MessageId::ParameterMetadataGet, Body::ParameterMetadata(value)) => {
                encode_success!(SuccessResponsePayload::ParameterMetadata(value))
            }
            (MessageId::ParameterStateGet, Body::ParameterState(value)) => {
                encode_success!(SuccessResponsePayload::ParameterState(value))
            }
            (MessageId::AutomationEnqueue, Body::AutomationEnqueued(value)) => {
                encode_success!(SuccessResponsePayload::AutomationEnqueued(value))
            }
            (MessageId::TransportGet, Body::Transport(value)) => {
                encode_success!(SuccessResponsePayload::TransportGetSnapshot(value))
            }
            (MessageId::TransportSet, Body::Transport(value)) => {
                encode_success!(SuccessResponsePayload::TransportSetSnapshot(value))
            }
            (MessageId::TelemetryConfigure, Body::Telemetry(value)) => {
                encode_success!(SuccessResponsePayload::TelemetryConfiguration(value))
            }
            (MessageId::CountersGet, Body::Counters(value)) => {
                encode_success!(SuccessResponsePayload::CounterSnapshot(value))
            }
            (MessageId::DiagnosticsGet, Body::Diagnostics(value)) => {
                encode_success!(SuccessResponsePayload::DiagnosticsPage(value))
            }
            _ => Err(EncodeError::MessageKindMismatch),
        }
    }

    fn encode_outcome_frame(
        &self,
        message_id: MessageId,
        request_id: RequestId,
        outcome: Outcome,
    ) -> Result<Vec<u8>, EncodeError> {
        #[cfg(test)]
        RESPONSE_STAGING_VECS.with(|allocations| {
            allocations.set(allocations.get().saturating_add(1));
        });
        let mut output = vec![0_u8; self.replay.config.max_response_bytes];
        let written = self.encode_outcome_into(message_id, request_id, outcome, &mut output)?;
        output.truncate(written);
        Ok(output)
    }

    fn encode_outcome_or_internal_into(
        &self,
        message_id: MessageId,
        request_id: RequestId,
        outcome: Outcome,
        output: &mut [u8],
    ) -> Result<(usize, StatusCode, SessionRevision), EncodeError> {
        let status = outcome.status;
        let revision = outcome.revision;
        match self.encode_outcome_into(message_id, request_id, outcome, output) {
            Ok(written) => Ok((written, status, revision)),
            Err(EncodeError::OutputTooSmall { required }) => {
                Err(EncodeError::OutputTooSmall { required })
            }
            Err(_) => {
                let fallback = self.encode_failure_outcome();
                let status = fallback.status;
                let revision = fallback.revision;
                let written = self.encode_outcome_into(message_id, request_id, fallback, output)?;
                Ok((written, status, revision))
            }
        }
    }

    fn encode_failure_outcome(&self) -> Outcome {
        self.non_ok_value(
            StatusCode::Internal,
            NonOkResponse {
                diagnostics: vec![Diagnostic {
                    code: "protocol.encode".to_owned(),
                    severity: crate::DiagnosticSeverity::Error,
                    path: Vec::new(),
                    detail: None,
                    operation_index: None,
                    sample_time: None,
                    provider_sequence: None,
                }],
                omitted_diagnostics: 0,
                backpressure: None,
            },
        )
    }

    fn compatibility_response(
        &self,
        message_id: MessageId,
        request_id: RequestId,
        outcome: Outcome,
    ) -> ControllerResponse {
        let status = outcome.status;
        let revision = outcome.revision;
        let (status, revision, frame) =
            match self.encode_outcome_frame(message_id, request_id, outcome) {
                Ok(frame) => (status, revision, frame),
                Err(_) => {
                    let fallback = self.encode_failure_outcome();
                    let status = fallback.status;
                    let revision = fallback.revision;
                    let frame = self
                        .encode_outcome_frame(message_id, request_id, fallback)
                        .unwrap_or_default();
                    (status, revision, frame)
                }
            };
        ControllerResponse::from_complete_frame(request_id, status, revision, frame)
    }

    fn compatibility_cached_response(
        &self,
        message_id: MessageId,
        request_id: RequestId,
        hit: ReplayHit,
    ) -> ControllerResponse {
        let frame = self.replay.cached(hit);
        let Ok(decoded) = self.codec.decode_header(frame) else {
            return self.compatibility_response(
                message_id,
                request_id,
                self.non_ok(StatusCode::Internal, None),
            );
        };
        let Some(header) = decoded.header.response() else {
            return self.compatibility_response(
                message_id,
                request_id,
                self.non_ok(StatusCode::Internal, None),
            );
        };
        ControllerResponse::from_complete_frame(
            header.request_id,
            header.status,
            header.revision,
            frame.to_vec(),
        )
    }
    /// Borrow authoritative control-plane session state.
    #[must_use]
    pub const fn session(&self) -> &SessionStore {
        &self.session
    }

    /// Immutably inspect prepared queue reports without disturbing an outstanding reservation.
    #[must_use]
    pub const fn queues(&self) -> &ProtocolQueues {
        &self.queues
    }

    /// Mutably borrow preallocated protocol queues for fixed render-side consumption fixtures.
    pub fn queues_mut(&mut self) -> &mut ProtocolQueues {
        assert!(
            !self.structural_outstanding(),
            "prepared structural command owns exact queue reservations"
        );
        &mut self.queues
    }

    /// Borrow the provider for conformance/mock inspection.
    #[must_use]
    pub const fn provider(&self) -> &P {
        &self.provider
    }

    /// Borrow the bounded replay cache for occupancy fixture checks.
    #[must_use]
    pub const fn replay(&self) -> &ReplayCache {
        &self.replay
    }

    /// Replace only typed provider/event enablement for endpoint configuration tests/hosts.
    pub fn set_provider_features(&mut self, features: ProviderFeatures) {
        assert!(
            !self.structural_outstanding(),
            "prepared structural command freezes provider features"
        );
        self.config.provider_features = features;
    }

    /// Borrow the accepted endpoint telemetry configuration without permitting mutation.
    #[must_use]
    pub const fn telemetry_configuration(&self) -> &TelemetryConfiguration {
        &self.telemetry_configuration
    }

    /// Stage one mock/control-only meter batch for explicitly configured lossy event egress.
    /// This does not create production meters or touch a render plan.
    pub fn stage_meter_batch_event(
        &mut self,
        revision: SessionRevision,
        observed_sample: SampleTime,
        records: &[MeterRecord],
    ) -> Result<(), EventEgressError> {
        if self.structural_outstanding() {
            return Err(EventEgressError::ReliableQueueFull(
                self.queues.report(crate::QueueKind::ReliableEvent),
            ));
        }
        if !self.config.provider_features.meters
            || self.telemetry_configuration.meter_handles.is_empty()
            || self.telemetry_configuration.meter_period_blocks == 0
        {
            return Err(EventEgressError::Disabled);
        }
        self.codec
            .encoded_meter_batch_len(MeterBatch {
                observed_sample,
                records,
            })
            .map_err(EventEgressError::Encode)?;
        if records.iter().any(|record| {
            !self
                .telemetry_configuration
                .meter_handles
                .contains(&record.handle)
        }) {
            return Err(EventEgressError::Encode(EncodeError::LimitExceeded));
        }
        for record in records {
            self.queues.stage_telemetry(TelemetryRecord {
                key: TelemetryKey {
                    revision,
                    handle: record.handle,
                    component: record.component as u16,
                },
                observed_sample,
                flags: record.flags,
                value: record.value,
            });
        }
        self.queues.flush_telemetry();
        Ok(())
    }

    /// Stage one mock/control-only counter snapshot for explicitly configured lossy event egress.
    pub fn stage_counter_snapshot_event(
        &mut self,
        revision: SessionRevision,
        snapshot: &CounterSnapshot,
    ) -> Result<(), EventEgressError> {
        if self.structural_outstanding() {
            return Err(EventEgressError::ReliableQueueFull(
                self.queues.report(crate::QueueKind::ReliableEvent),
            ));
        }
        if !self.config.provider_features.counters
            || self.telemetry_configuration.counter_ids.is_empty()
            || self.telemetry_configuration.counter_period_blocks == 0
        {
            return Err(EventEgressError::Disabled);
        }
        self.codec
            .encoded_counter_snapshot_len(snapshot)
            .map_err(EventEgressError::Encode)?;
        if snapshot
            .values
            .iter()
            .any(|value| !self.telemetry_configuration.counter_ids.contains(&value.id))
        {
            return Err(EventEgressError::Encode(EncodeError::LimitExceeded));
        }
        for value in &snapshot.values {
            self.queues.stage_counter_telemetry(CounterTelemetryRecord {
                key: TelemetryKey {
                    revision,
                    handle: value.id as u32,
                    component: 0,
                },
                id: value.id,
                observed_sample: snapshot.observed_sample,
                value: value.value,
            });
        }
        self.queues.flush_counter_telemetry();
        Ok(())
    }

    /// Reliably queue one bounded typed diagnostic by reference, without an arbitrary byte
    /// payload. The owned diagnostic moves into prepared endpoint storage until successful egress.
    pub fn enqueue_diagnostic_event(
        &mut self,
        revision: SessionRevision,
        event: DiagnosticEvent,
    ) -> Result<(), EventEgressError> {
        if self.structural_outstanding() {
            return Err(EventEgressError::ReliableQueueFull(
                self.queues.report(crate::QueueKind::ReliableEvent),
            ));
        }
        if !self.config.provider_features.diagnostics
            || !self.telemetry_configuration.diagnostics_enabled
            || (event.diagnostic.severity as u8)
                < (self.telemetry_configuration.minimum_diagnostic_severity as u8)
        {
            return Err(EventEgressError::Disabled);
        }
        self.codec
            .encoded_diagnostic_event_len(&event)
            .map_err(EventEgressError::Encode)?;
        let Some(index) = self
            .diagnostic_event_slots
            .iter()
            .position(RetainedDiagnosticSlot::is_empty)
        else {
            return Err(EventEgressError::DiagnosticStorageFull);
        };
        let slot = u32::try_from(index).map_err(|_| EventEgressError::DiagnosticStorageFull)?;
        self.diagnostic_event_slots[index] = RetainedDiagnosticSlot::Owned(event.diagnostic);
        let queued = ReliableSlot {
            header: crate::ReliableHeader::Event,
            revision,
            message_id: MessageId::Diagnostic,
            payload: ReliablePayload::Diagnostic {
                diagnostic_slot: slot,
            },
        };
        if let Err(error) = self.queues.try_enqueue_event(queued) {
            self.diagnostic_event_slots[index] = RetainedDiagnosticSlot::Empty;
            return Err(EventEgressError::ReliableQueueFull(error.report));
        }
        Ok(())
    }

    /// Dequeue and encode one complete reliable event frame. A short caller output buffer leaves
    /// the exact queued event pending, so retrying with the required size neither loses nor
    /// reorders reliable events.
    pub fn dequeue_reliable_event_frame_into(
        &mut self,
        output: &mut [u8],
    ) -> Result<Option<usize>, EventEgressError> {
        if self.pending_reliable_event.is_none() {
            self.pending_reliable_event = self.queues.try_dequeue_event().ok();
        }
        let Some(slot) = self.pending_reliable_event else {
            return Ok(None);
        };
        let result = self.encode_reliable_event_slot(slot, output);
        if result.is_ok() {
            if let ReliablePayload::Diagnostic { diagnostic_slot } = slot.payload
                && let Ok(index) = usize::try_from(diagnostic_slot)
                && let Some(storage) = self.diagnostic_event_slots.get_mut(index)
            {
                *storage = RetainedDiagnosticSlot::Empty;
            }
            self.pending_reliable_event = None;
        }
        result.map(Some)
    }

    /// Drain and encode one complete explicitly lossy meter or counter event, with no more than
    /// 256 fixed records. A short caller output buffer retains the assembled batch unchanged.
    pub fn dequeue_lossy_event_frame_into(
        &mut self,
        output: &mut [u8],
    ) -> Result<Option<usize>, EventEgressError> {
        if matches!(self.pending_telemetry_event, PendingTelemetryEvent::None) {
            self.prepare_next_lossy_event();
        }
        let result = match &self.pending_telemetry_event {
            PendingTelemetryEvent::None => return Ok(None),
            PendingTelemetryEvent::Meter {
                revision,
                observed_sample,
                len,
                records,
            } => self.codec.encode_event_frame_into(
                &TypedEventFrame {
                    revision: *revision,
                    payload: EventPayload::MeterBatch(MeterBatch {
                        observed_sample: *observed_sample,
                        records: &records[..usize::from(*len)],
                    }),
                },
                output,
            ),
            PendingTelemetryEvent::Counter {
                revision,
                observed_sample,
                len,
                values,
            } => self.codec.encode_event_frame_into(
                &TypedEventFrame {
                    revision: *revision,
                    payload: EventPayload::CounterSnapshot(CounterSnapshotRef {
                        observed_sample: *observed_sample,
                        values: &values[..usize::from(*len)],
                    }),
                },
                output,
            ),
        };
        if result.is_ok() {
            self.pending_telemetry_event = PendingTelemetryEvent::None;
        }
        result.map(Some).map_err(EventEgressError::Encode)
    }

    fn encode_reliable_event_slot(
        &self,
        slot: ReliableSlot,
        output: &mut [u8],
    ) -> Result<usize, EventEgressError> {
        let result = match slot.payload {
            ReliablePayload::SessionCommitted {
                event_sequence,
                origin_request_id,
                previous_revision,
                applied_operations,
            } => self.codec.encode_event_frame_into(
                &TypedEventFrame {
                    revision: slot.revision,
                    payload: EventPayload::SessionCommitted(SessionCommitted {
                        event_sequence,
                        origin_request_id,
                        previous_revision,
                        applied_operations,
                    }),
                },
                output,
            ),
            ReliablePayload::AutomationCanceled {
                event_sequence,
                origin_request_id,
                canceled_records,
                reason,
                queue_generation,
                effective_sample,
            } => self.codec.encode_event_frame_into(
                &TypedEventFrame {
                    revision: slot.revision,
                    payload: EventPayload::AutomationCanceled(AutomationCanceled {
                        event_sequence,
                        origin_request_id,
                        canceled_records,
                        reason,
                        queue_generation,
                        effective_sample,
                    }),
                },
                output,
            ),
            ReliablePayload::TransportState {
                event_sequence,
                state,
                position,
                effective_sample,
                origin_request_id,
            } => self.codec.encode_event_frame_into(
                &TypedEventFrame {
                    revision: slot.revision,
                    payload: EventPayload::TransportState(TransportStateEvent {
                        event_sequence,
                        state,
                        position,
                        effective_sample,
                        origin_request_id,
                    }),
                },
                output,
            ),
            ReliablePayload::Diagnostic { diagnostic_slot } => {
                let index = usize::try_from(diagnostic_slot)
                    .map_err(|_| EventEgressError::DiagnosticStorageFull)?;
                let diagnostic = match self.diagnostic_event_slots.get(index) {
                    Some(RetainedDiagnosticSlot::Owned(diagnostic)) => diagnostic.clone(),
                    Some(RetainedDiagnosticSlot::Empty) | None => {
                        return Err(EventEgressError::DiagnosticStorageFull);
                    }
                };
                self.codec.encode_event_frame_into(
                    &TypedEventFrame {
                        revision: slot.revision,
                        payload: EventPayload::Diagnostic(&diagnostic),
                    },
                    output,
                )
            }
            ReliablePayload::EmptyResponse => {
                return Err(EventEgressError::Encode(EncodeError::MessageKindMismatch));
            }
        };
        result.map_err(EventEgressError::Encode)
    }

    fn prepare_next_lossy_event(&mut self) {
        self.queues.flush_telemetry();
        self.queues.flush_counter_telemetry();
        if let Some(first) = self
            .pending_meter_record
            .take()
            .or_else(|| self.queues.try_dequeue_telemetry().ok())
        {
            let revision = first.key.revision;
            let observed_sample = first.observed_sample;
            let mut records = [EMPTY_METER_RECORD; crate::AUTOMATION_BATCH_RECORDS];
            records[0] = MeterRecord {
                handle: first.key.handle,
                component: match first.key.component {
                    1 => crate::MeterComponent::Left,
                    2 => crate::MeterComponent::Right,
                    _ => crate::MeterComponent::Aggregate,
                },
                flags: first.flags,
                value: first.value,
            };
            let mut len = 1_usize;
            while len < records.len() {
                let Some(next) = self.queues.try_dequeue_telemetry().ok() else {
                    break;
                };
                if next.key.revision != revision || next.observed_sample != observed_sample {
                    self.pending_meter_record = Some(next);
                    break;
                }
                records[len] = MeterRecord {
                    handle: next.key.handle,
                    component: match next.key.component {
                        1 => crate::MeterComponent::Left,
                        2 => crate::MeterComponent::Right,
                        _ => crate::MeterComponent::Aggregate,
                    },
                    flags: next.flags,
                    value: next.value,
                };
                len += 1;
            }
            self.pending_telemetry_event = PendingTelemetryEvent::Meter {
                revision,
                observed_sample,
                len: u16::try_from(len).expect("fixed telemetry batch length"),
                records,
            };
            return;
        }
        if let Some(first) = self
            .pending_counter_record
            .take()
            .or_else(|| self.queues.try_dequeue_counter_telemetry().ok())
        {
            let revision = first.key.revision;
            let observed_sample = first.observed_sample;
            let mut values = [EMPTY_COUNTER_VALUE; crate::AUTOMATION_BATCH_RECORDS];
            values[0] = CounterValue {
                id: first.id,
                value: first.value,
            };
            let mut len = 1_usize;
            while len < values.len() {
                let Some(next) = self.queues.try_dequeue_counter_telemetry().ok() else {
                    break;
                };
                if next.key.revision != revision
                    || next.observed_sample != observed_sample
                    || next.id <= values[len - 1].id
                {
                    self.pending_counter_record = Some(next);
                    break;
                }
                values[len] = CounterValue {
                    id: next.id,
                    value: next.value,
                };
                len += 1;
            }
            self.pending_telemetry_event = PendingTelemetryEvent::Counter {
                revision,
                observed_sample,
                len: u16::try_from(len).expect("fixed telemetry batch length"),
                values,
            };
        }
    }

    fn execute(&mut self, request: &ControllerRequest<'_>) -> Outcome {
        let features = self.config.provider_features;
        let enabled = match request.command {
            ControlCommand::SessionTransactionApply { .. } => {
                features.session_events && self.config.maximum_transaction_edits != 0
            }
            ControlCommand::ParameterMetadataGet { .. }
            | ControlCommand::ParameterStateGet { .. } => features.parameters,
            ControlCommand::TransportGet => features.transport,
            ControlCommand::TransportSet { .. } => features.transport && features.transport_events,
            ControlCommand::CountersGet { .. } => features.counters,
            ControlCommand::DiagnosticsGet { .. } => features.diagnostics,
            _ => true,
        };
        if !enabled {
            return self.non_ok(StatusCode::Unavailable, None);
        }
        if request.command.requires_exact_revision()
            && !matches!(request.expected_revision, ExpectedRevision::Exact(_))
        {
            return self.non_ok(StatusCode::InvalidField, None);
        }
        if let ExpectedRevision::Exact(expected) = request.expected_revision
            && expected != self.session.revision()
        {
            return self.non_ok(StatusCode::RevisionConflict, None);
        }
        match &request.command {
            ControlCommand::CapabilitiesGet => {
                self.ok(Body::Capabilities(self.capability_registry()))
            }
            ControlCommand::SessionSnapshotGet { offset, max_bytes } => {
                let snapshot = self.session.canonical_snapshot().as_bytes();
                let offset = match usize::try_from(*offset) {
                    Ok(value) if value <= snapshot.len() => value,
                    _ => {
                        return self.non_ok(StatusCode::InvalidField, None);
                    }
                };
                if *max_bytes == 0 {
                    return self.non_ok(StatusCode::InvalidField, None);
                }
                let end = offset
                    .saturating_add(*max_bytes as usize)
                    .min(snapshot.len());
                self.ok(Body::Snapshot {
                    total: snapshot.len() as u64,
                    offset: offset as u64,
                    range: offset..end,
                    eof: end == snapshot.len(),
                })
            }
            ControlCommand::SessionTransactionApply { edits } => {
                if edits.is_empty() {
                    return self.non_ok(StatusCode::InvalidField, None);
                }
                if edits.len() > self.config.maximum_transaction_edits as usize {
                    return self.non_ok(StatusCode::LimitExceeded, None);
                }
                let event_sequence = self.next_reliable_event_sequence;
                let cancellation_batches =
                    usize::try_from(self.queues.report(crate::QueueKind::Automation).occupancy)
                        .unwrap_or(usize::MAX);
                let Some(_) = event_sequence.checked_add(
                    u64::try_from(cancellation_batches)
                        .unwrap_or(u64::MAX)
                        .saturating_add(1),
                ) else {
                    return self.non_ok(StatusCode::Internal, None);
                };
                let mut cancellation_reservations =
                    match self.queues.reserve_reliable_events(cancellation_batches) {
                        Ok(reservations) => reservations,
                        Err(report) => {
                            return self.queue_backpressure_outcome(report);
                        }
                    };
                let applied_operations = match u32::try_from(edits.len()) {
                    Ok(value) => value,
                    Err(_) => {
                        self.queues
                            .release_reliable_events(cancellation_reservations);
                        return self.non_ok(StatusCode::LimitExceeded, None);
                    }
                };
                let previous_revision = self.session.revision();
                let reservation = match self.queues.reserve_reliable_event() {
                    Ok(reservation) => reservation,
                    Err(report) => {
                        self.queues
                            .release_reliable_events(cancellation_reservations);
                        return self.queue_backpressure_outcome(report);
                    }
                };
                match self
                    .session
                    .apply_transaction(request.expected_revision, edits)
                {
                    Ok(commit) => {
                        self.queues.commit_reliable_event(
                            reservation,
                            ReliableSlot::session_committed(
                                commit.revision,
                                event_sequence,
                                request.request_id,
                                previous_revision,
                                applied_operations,
                            ),
                        );
                        self.next_reliable_event_sequence = event_sequence.saturating_add(1);
                        let effective_sample = self.provider.current_sample();
                        let _ = self.cancel_queued_automation_reserved(
                            &mut cancellation_reservations,
                            AutomationCancellationReason::RevisionChanged,
                            Some(effective_sample),
                        );
                        self.ok_at_revision(
                            commit.revision,
                            Body::TransactionApplied(TransactionApplied { applied_operations }),
                        )
                    }
                    Err(error) => {
                        self.queues.release_reliable_event(reservation);
                        self.queues
                            .release_reliable_events(cancellation_reservations);
                        self.transaction_error_outcome(&error)
                    }
                }
            }
            ControlCommand::ParameterMetadataGet {
                request: parameter_request,
            } => match self.provider.parameter_metadata(*parameter_request) {
                Ok(page) => self.ok(Body::ParameterMetadata(page)),
                Err(error) => self.non_ok(status_for_parameter(error), None),
            },
            ControlCommand::ParameterStateGet {
                request: parameter_request,
            } => match self.provider.parameter_state(parameter_request) {
                Ok(page) => self.ok(Body::ParameterState(page)),
                Err(error) => self.non_ok(status_for_parameter(error), None),
            },
            ControlCommand::AutomationEnqueue { batch } => {
                if batch.revision != self.session.revision() {
                    return self.non_ok(StatusCode::RevisionConflict, None);
                }
                if let Err(error) = self.validate_automation_domains(batch) {
                    return self.non_ok(error, None);
                }
                let current_sample = self.provider.current_sample();
                match self.queues.try_enqueue_automation(current_sample, *batch) {
                    Ok(()) => {
                        let report = self.queues.report(crate::QueueKind::Automation);
                        self.ok(Body::AutomationEnqueued(AutomationEnqueued {
                            accepted_records: batch.len,
                            occupancy: report.occupancy,
                            capacity: report.capacity as u64,
                            generation: report.generation.0,
                        }))
                    }
                    Err(AutomationEnqueueError::Full { report, .. }) => self.non_ok_value(
                        StatusCode::Backpressure,
                        NonOkResponse {
                            diagnostics: Vec::new(),
                            omitted_diagnostics: 0,
                            backpressure: Some(Backpressure {
                                queue_kind: BackpressureQueueKind::Automation,
                                capacity: report.capacity as u64,
                                occupancy: report.occupancy,
                                requested_items: 1,
                                generation: Some(report.generation.0),
                                retry_boundary: None,
                                requested_bytes: None,
                                available_bytes: None,
                            }),
                        },
                    ),
                    Err(AutomationEnqueueError::Invalid { error, .. }) => {
                        self.non_ok(status_for_automation(error), None)
                    }
                }
            }
            ControlCommand::TransportGet => {
                let snapshot = self.provider.transport_get();
                self.ok(Body::Transport(snapshot))
            }
            ControlCommand::TransportSet {
                request: transport_request,
            } => {
                let event_sequence = self.next_reliable_event_sequence;
                let cancellation_batches = if transport_request.position.is_some() {
                    usize::try_from(self.queues.report(crate::QueueKind::Automation).occupancy)
                        .unwrap_or(usize::MAX)
                } else {
                    0
                };
                let Some(_) = event_sequence.checked_add(
                    u64::try_from(cancellation_batches)
                        .unwrap_or(u64::MAX)
                        .saturating_add(1),
                ) else {
                    return self.non_ok(StatusCode::Internal, None);
                };
                let mut cancellation_reservations =
                    match self.queues.reserve_reliable_events(cancellation_batches) {
                        Ok(reservations) => reservations,
                        Err(report) => {
                            return self.queue_backpressure_outcome(report);
                        }
                    };
                let reservation = match self.queues.reserve_reliable_event() {
                    Ok(reservation) => reservation,
                    Err(report) => {
                        self.queues
                            .release_reliable_events(cancellation_reservations);
                        return self.queue_backpressure_outcome(report);
                    }
                };
                let snapshot = self.provider.transport_set(*transport_request);
                self.queues.commit_reliable_event(
                    reservation,
                    ReliableSlot::transport_state(
                        self.session.revision(),
                        event_sequence,
                        snapshot.state,
                        snapshot.position,
                        snapshot.effective_sample,
                        Some(request.request_id),
                    ),
                );
                self.next_reliable_event_sequence = event_sequence.saturating_add(1);
                if transport_request.position.is_some() {
                    let _ = self.cancel_queued_automation_reserved(
                        &mut cancellation_reservations,
                        AutomationCancellationReason::TransportLocate,
                        Some(snapshot.effective_sample),
                    );
                }
                self.ok(Body::Transport(snapshot))
            }
            ControlCommand::TelemetryConfigure { configuration } => {
                let configured = self.provider.telemetry_configure(configuration.clone());
                self.telemetry_configuration.meter_handles.clear();
                self.telemetry_configuration
                    .meter_handles
                    .extend_from_slice(&configured.meter_handles);
                self.telemetry_configuration.counter_ids.clear();
                self.telemetry_configuration
                    .counter_ids
                    .extend_from_slice(&configured.counter_ids);
                self.telemetry_configuration.meter_period_blocks = configured.meter_period_blocks;
                self.telemetry_configuration.counter_period_blocks =
                    configured.counter_period_blocks;
                self.telemetry_configuration.diagnostics_enabled = configured.diagnostics_enabled;
                self.telemetry_configuration.minimum_diagnostic_severity =
                    configured.minimum_diagnostic_severity;
                self.ok(Body::Telemetry(configured))
            }
            ControlCommand::CountersGet {
                request: counters_request,
            } => match self.provider.counters(counters_request) {
                Ok(snapshot) => self.ok(Body::Counters(snapshot)),
                Err(error) => self.non_ok(status_for_parameter(error), None),
            },
            ControlCommand::DiagnosticsGet {
                request: diagnostics_request,
            } => match self.provider.diagnostics(*diagnostics_request) {
                Ok(page) => self.ok(Body::Diagnostics(page)),
                Err(ParameterProviderError::ReplayExpired) => {
                    let expired = NonOkResponse {
                        diagnostics: vec![Diagnostic {
                            code: "diagnostics.cursor_expired".to_owned(),
                            severity: crate::DiagnosticSeverity::Error,
                            path: Vec::new(),
                            detail: None,
                            operation_index: None,
                            sample_time: None,
                            provider_sequence: None,
                        }],
                        omitted_diagnostics: 0,
                        backpressure: None,
                    };
                    self.non_ok_value(StatusCode::ReplayExpired, expired)
                }
                Err(error) => self.non_ok(status_for_parameter(error), None),
            },
        }
    }

    fn ok(&self, body: Body) -> Outcome {
        self.ok_at_revision(self.session.revision(), body)
    }

    fn ok_at_revision(&self, revision: SessionRevision, body: Body) -> Outcome {
        Outcome {
            status: StatusCode::Ok,
            revision,
            body,
        }
    }

    fn non_ok(&self, status: StatusCode, backpressure: Option<Backpressure>) -> Outcome {
        let backpressure = if status == StatusCode::Backpressure && backpressure.is_none() {
            Some(Backpressure {
                queue_kind: BackpressureQueueKind::ReplayCache,
                capacity: self.replay.config.entries.get() as u64,
                occupancy: self.replay.entries.len() as u64,
                requested_items: 1,
                generation: None,
                retry_boundary: None,
                requested_bytes: None,
                available_bytes: None,
            })
        } else {
            backpressure
        };
        let code = if backpressure.is_some() {
            "protocol.backpressure"
        } else {
            "protocol.failure"
        };
        self.non_ok_value(
            status,
            NonOkResponse {
                diagnostics: vec![Diagnostic {
                    code: code.to_owned(),
                    severity: crate::DiagnosticSeverity::Error,
                    path: Vec::new(),
                    detail: None,
                    operation_index: None,
                    sample_time: None,
                    provider_sequence: None,
                }],
                omitted_diagnostics: 0,
                backpressure,
            },
        )
    }

    fn non_ok_value(&self, status: StatusCode, payload: NonOkResponse) -> Outcome {
        Outcome {
            status,
            revision: self.session.revision(),
            body: Body::NonOk(payload),
        }
    }

    fn queue_backpressure_outcome(&self, report: QueueReport) -> Outcome {
        let queue_kind = match report.kind {
            crate::QueueKind::ControlCommand => BackpressureQueueKind::ControlCommand,
            crate::QueueKind::Automation => BackpressureQueueKind::Automation,
            crate::QueueKind::ReliableResponse => BackpressureQueueKind::ReliableResponse,
            crate::QueueKind::ReliableEvent => BackpressureQueueKind::ReliableEvent,
            crate::QueueKind::Telemetry => BackpressureQueueKind::Telemetry,
        };
        self.non_ok(
            StatusCode::Backpressure,
            Some(Backpressure {
                queue_kind,
                capacity: report.capacity as u64,
                occupancy: report.occupancy,
                requested_items: report.requested_slots,
                generation: Some(report.generation.0),
                retry_boundary: None,
                requested_bytes: None,
                available_bytes: None,
            }),
        )
    }

    fn replay_backpressure_outcome(&self, request_bytes: usize) -> Outcome {
        let config = self.replay.config();
        let requested_bytes = request_bytes.saturating_add(config.max_response_bytes);
        let available_bytes = config
            .bytes
            .get()
            .saturating_sub(self.replay.retained_bytes());
        self.non_ok(
            StatusCode::Backpressure,
            Some(Backpressure {
                queue_kind: BackpressureQueueKind::ReplayCache,
                capacity: config.entries.get() as u64,
                occupancy: self.replay.entries.len() as u64,
                requested_items: 1,
                generation: None,
                retry_boundary: None,
                requested_bytes: Some(u64::try_from(requested_bytes).unwrap_or(u64::MAX)),
                available_bytes: Some(u64::try_from(available_bytes).unwrap_or(u64::MAX)),
            }),
        )
    }

    fn transaction_error_outcome(&self, error: &SessionStoreError) -> Outcome {
        let status = status_for_transaction(error);
        let diagnostics = transaction_error_diagnostics(error);
        if diagnostics.is_empty() {
            return self.non_ok(status, None);
        }
        self.non_ok_value(status, self.bounded_non_ok_diagnostics(&diagnostics))
    }
    fn capability_registry(&self) -> CapabilitySet {
        let features = self.config.provider_features;
        let session_transactions =
            features.session_events && self.config.maximum_transaction_edits != 0;
        let mut commands = [0_u16; 11];
        let mut command_len = 0_usize;
        for id in 1_u16..=11 {
            let enabled = match id {
                3 => session_transactions,
                4 | 5 => features.parameters,
                7 => features.transport,
                8 => features.transport && features.transport_events,
                10 => features.counters,
                11 => features.diagnostics,
                _ => true,
            };
            if enabled {
                commands[command_len] = id;
                command_len += 1;
            }
        }
        let candidates = [0x8001_u16, 0x8002, 0x8010, 0x8020, 0x8021, 0x8030];
        let mut events = [0_u16; 6];
        let mut event_len = 0_usize;
        for id in candidates {
            let enabled = match id {
                0x8001 | 0x8002 => session_transactions,
                0x8010 => features.transport && features.transport_events,
                0x8020 => features.meters,
                0x8021 => features.counters,
                0x8030 => features.diagnostics,
                _ => false,
            };
            if enabled {
                events[event_len] = id;
                event_len += 1;
            }
        }
        let mut flags = (1 << 7) - 1;
        if !session_transactions {
            flags &= !(1 << 3);
        }
        if features.parameters {
            flags |= 1 << 7;
        }
        if features.transport {
            flags |= 1 << 8;
        }
        if features.meters {
            flags |= 1 << 9;
        }
        if features.counters {
            flags |= 1 << 10;
        }
        if features.diagnostics {
            flags |= 1 << 11;
        }
        if session_transactions {
            flags |= 1 << 12;
        }
        if features.transport && features.transport_events {
            flags |= 1 << 13;
        }
        CapabilitySet {
            commands,
            command_len: command_len as u8,
            events,
            event_len: event_len as u8,
            flags: CapabilityFlags(flags),
        }
    }

    fn effective_maximum_transaction_edits(&self) -> u32 {
        if self.config.provider_features.session_events {
            self.config.maximum_transaction_edits
        } else {
            0
        }
    }

    fn capabilities<'a>(&self, set: &'a CapabilitySet) -> Capabilities<'a> {
        let queue = self.queues.config();
        let replay = self.replay.config();
        Capabilities {
            minimum_version: crate::ProtocolVersion::V1,
            maximum_version: crate::ProtocolVersion::V1,
            maximum_frame_bytes: self.codec.limits().max_frame_bytes as u64,
            maximum_tlvs: self.codec.limits().max_tlv_count,
            maximum_string_bytes: self.codec.limits().max_string_bytes as u64,
            maximum_nesting: self.codec.limits().max_nesting,
            maximum_automation_records: crate::AUTOMATION_BATCH_RECORDS as u16,
            control_command_slots: queue.control_command_slots.get() as u64,
            control_command_bytes: queue.control_command_bytes.get() as u64,
            automation_batch_slots: queue.automation_batch_slots.get() as u64,
            reliable_response_slots: queue.reliable_response_slots.get() as u64,
            reliable_event_slots: queue.reliable_event_slots.get() as u64,
            telemetry_slots: queue.telemetry_slots.get() as u64,
            replay_entries: replay.entries.get() as u64,
            replay_bytes: replay.bytes.get() as u64,
            maximum_cached_response_bytes: replay.max_response_bytes as u64,
            per_block_automation_density: queue.per_block_automation_density.get() as u64,
            admission_quantum_frames: queue.quantum_frames.get() as u64,
            maximum_parameter_page_items: 256,
            maximum_diagnostic_page_items: 256,
            maximum_telemetry_handles: 256,
            maximum_transaction_edits: self.effective_maximum_transaction_edits(),
            supported_commands: &set.commands[..usize::from(set.command_len)],
            supported_events: &set.events[..usize::from(set.event_len)],
            flags: set.flags,
        }
    }

    fn bounded_non_ok_diagnostics(&self, diagnostics: &[Diagnostic]) -> NonOkResponse {
        let maximum = usize::from(self.config.maximum_response_diagnostics);
        let mut retained = Vec::with_capacity(diagnostics.len().min(maximum));
        for diagnostic in diagnostics.iter().take(maximum) {
            let mut candidate = retained.clone();
            candidate.push(diagnostic.clone());
            let value = NonOkResponse {
                diagnostics: candidate,
                omitted_diagnostics: 0,
                backpressure: None,
            };
            let fits = self
                .codec
                .encoded_non_ok_payload_len(&value)
                .is_ok_and(|length| {
                    length.saturating_add(crate::OUTER_HEADER_BYTES)
                        <= self.replay.config.max_response_bytes
                });
            if !fits {
                break;
            }
            retained.push(diagnostic.clone());
        }
        NonOkResponse {
            omitted_diagnostics: u32::try_from(diagnostics.len().saturating_sub(retained.len()))
                .unwrap_or(u32::MAX),
            diagnostics: retained,
            backpressure: None,
        }
    }

    fn validate_automation_domains(
        &mut self,
        batch: &AutomationBatchSlot,
    ) -> Result<(), StatusCode> {
        for record in batch.as_slice() {
            let descriptor = self
                .provider
                .parameter_descriptor(record.handle)
                .map_err(status_for_parameter)?;
            if descriptor.flags & 2 == 0
                || descriptor.automation_rate == ParameterAutomationRate::None
            {
                return Err(StatusCode::InvalidField);
            }
            for value in [record.start_value, record.end_value] {
                let valid = match descriptor.domain {
                    ParameterDomain::Continuous => descriptor
                        .minimum
                        .zip(descriptor.maximum)
                        .is_some_and(|(minimum, maximum)| minimum <= value && value <= maximum),
                    ParameterDomain::Boolean => value == 0.0 || value == 1.0,
                    ParameterDomain::Enumeration => descriptor
                        .enum_choices
                        .iter()
                        .any(|choice| choice.value == value),
                };
                if !valid {
                    return Err(StatusCode::InvalidField);
                }
            }
        }
        Ok(())
    }

    /// Encode a queued typed `SESSION_COMMITTED` event payload without an arbitrary byte variant.
    pub fn encode_session_committed_event(
        &self,
        slot: ReliableSlot,
        output: &mut [u8],
    ) -> Result<usize, crate::EncodeError> {
        let ReliablePayload::SessionCommitted {
            event_sequence,
            origin_request_id,
            previous_revision,
            applied_operations,
        } = slot.payload
        else {
            return Err(crate::EncodeError::MessageKindMismatch);
        };
        if slot.message_id != MessageId::SessionCommitted {
            return Err(crate::EncodeError::MessageKindMismatch);
        }
        self.codec.encode_session_committed(
            SessionCommitted {
                event_sequence,
                origin_request_id,
                previous_revision,
                applied_operations,
            },
            output,
        )
    }

    /// Encode a queued typed `TRANSPORT_STATE` event payload without arbitrary byte payloads.
    pub fn encode_transport_state_event(
        &self,
        slot: ReliableSlot,
        output: &mut [u8],
    ) -> Result<usize, crate::EncodeError> {
        let ReliablePayload::TransportState {
            event_sequence,
            state,
            position,
            effective_sample,
            origin_request_id,
        } = slot.payload
        else {
            return Err(crate::EncodeError::MessageKindMismatch);
        };
        if slot.message_id != MessageId::TransportState {
            return Err(crate::EncodeError::MessageKindMismatch);
        }
        self.codec.encode_transport_state_event(
            TransportStateEvent {
                event_sequence,
                state,
                position,
                effective_sample,
                origin_request_id,
            },
            output,
        )
    }

    /// Encode a queued typed `AUTOMATION_CANCELED` payload without arbitrary byte payloads.
    pub fn encode_automation_canceled_event(
        &self,
        slot: ReliableSlot,
        output: &mut [u8],
    ) -> Result<usize, crate::EncodeError> {
        let ReliablePayload::AutomationCanceled {
            event_sequence,
            origin_request_id,
            canceled_records,
            reason,
            queue_generation,
            effective_sample,
        } = slot.payload
        else {
            return Err(crate::EncodeError::MessageKindMismatch);
        };
        if slot.message_id != MessageId::AutomationCanceled {
            return Err(crate::EncodeError::MessageKindMismatch);
        }
        self.codec.encode_automation_canceled(
            AutomationCanceled {
                event_sequence,
                origin_request_id,
                canceled_records,
                reason,
                queue_generation,
                effective_sample,
            },
            output,
        )
    }

    /// Explicitly cancel all still-queued accepted automation for a non-command lifecycle path.
    /// A full reliable queue leaves automation untouched.
    pub fn cancel_pending_automation(
        &mut self,
        reason: AutomationCancellationReason,
        effective_sample: Option<SampleTime>,
    ) -> Result<u64, QueueReport> {
        if self.structural_outstanding() {
            return Err(self.queues.report(crate::QueueKind::ReliableEvent));
        }
        let batches = usize::try_from(self.queues.report(crate::QueueKind::Automation).occupancy)
            .unwrap_or(usize::MAX);
        let mut reservations = self.queues.reserve_reliable_events(batches)?;
        self.cancel_queued_automation_reserved(&mut reservations, reason, effective_sample)
    }

    fn structural_outstanding(&self) -> bool {
        self.structural_generation.load(Ordering::Acquire) & 1 != 0
    }

    fn cancel_queued_automation_reserved(
        &mut self,
        reservations: &mut crate::ReliableEventReservations,
        reason: AutomationCancellationReason,
        effective_sample: Option<SampleTime>,
    ) -> Result<u64, QueueReport> {
        let batches = usize::try_from(self.queues.report(crate::QueueKind::Automation).occupancy)
            .unwrap_or(usize::MAX);
        let next = self
            .next_reliable_event_sequence
            .checked_add(u64::try_from(batches).unwrap_or(u64::MAX));
        let Some(next_event_sequence) = next else {
            return Err(self.queues.report(crate::QueueKind::ReliableEvent));
        };
        let generation = self
            .queues
            .report(crate::QueueKind::Automation)
            .generation
            .0;
        let mut canceled = 0_u64;
        for _ in 0..batches {
            let batch = match self.queues.try_dequeue_automation() {
                Ok(batch) => batch,
                Err(_) => unreachable!("reservation and control-side cancellation are exclusive"),
            };
            self.queues.commit_reserved_reliable_event(
                reservations,
                ReliableSlot::automation_canceled(
                    self.session.revision(),
                    self.next_reliable_event_sequence,
                    batch.request_id,
                    batch.len,
                    reason,
                    generation,
                    effective_sample,
                ),
            );
            self.next_reliable_event_sequence = self.next_reliable_event_sequence.saturating_add(1);
            canceled = canceled.saturating_add(u64::from(batch.len));
        }
        self.queues.reset_automation_ordering_after_cancellation();
        self.next_reliable_event_sequence = next_event_sequence;
        if canceled != 0 {
            self.provider.record_canceled_automation(canceled);
        }
        Ok(canceled)
    }
}

fn copy_complete_frame(input: &[u8], output: &mut [u8]) -> Result<usize, EncodeError> {
    let required = input.len();
    if output.len() < required {
        return Err(EncodeError::OutputTooSmall { required });
    }
    output[..required].copy_from_slice(input);
    Ok(required)
}

fn status_for_transaction(error: &SessionStoreError) -> StatusCode {
    match error {
        SessionStoreError::ExactRevisionRequired | SessionStoreError::EmptyTransaction => {
            StatusCode::InvalidField
        }
        SessionStoreError::RevisionConflict { .. } => StatusCode::RevisionConflict,
        SessionStoreError::RevisionExhausted => StatusCode::RevisionExhausted,
        SessionStoreError::Edit { .. } | SessionStoreError::Validation { .. } => {
            StatusCode::ValidationFailed
        }
    }
}

fn transaction_error_diagnostics(error: &SessionStoreError) -> Vec<Diagnostic> {
    match error {
        SessionStoreError::Validation {
            operation_index,
            diagnostics,
        } => diagnostics
            .diagnostics()
            .iter()
            .map(|diagnostic| session_diagnostic_to_protocol(diagnostic, *operation_index))
            .collect(),
        SessionStoreError::Edit {
            operation_index,
            error,
        } => vec![Diagnostic {
            code: match error {
                crate::SessionEditError::NotFound => "session.edit.not_found",
                crate::SessionEditError::InvalidFinalPosition => {
                    "session.edit.invalid_final_position"
                }
                crate::SessionEditError::InvalidEffectOrder => "session.edit.invalid_effect_order",
                crate::SessionEditError::EmptyAutomationSegments => {
                    "session.edit.empty_automation_segments"
                }
            }
            .to_owned(),
            severity: crate::DiagnosticSeverity::Error,
            path: vec![
                crate::PathSegment::Field("edits".to_owned()),
                crate::PathSegment::Index(u64::try_from(*operation_index).expect(
                    "usize operation indices fit the u64 protocol carrier on supported targets",
                )),
            ],
            detail: Some(error.to_string()),
            operation_index: Some(
                u32::try_from(*operation_index)
                    .expect("controller bounds every session operation index to u32"),
            ),
            sample_time: None,
            provider_sequence: None,
        }],
        SessionStoreError::ExactRevisionRequired
        | SessionStoreError::EmptyTransaction
        | SessionStoreError::RevisionConflict { .. }
        | SessionStoreError::RevisionExhausted => Vec::new(),
    }
}

fn session_diagnostic_to_protocol(
    diagnostic: &session::Diagnostic,
    operation_index: usize,
) -> Diagnostic {
    Diagnostic {
        code: diagnostic.code.as_str().to_owned(),
        severity: crate::DiagnosticSeverity::Error,
        path: diagnostic
            .path
            .segments()
            .iter()
            .map(session_path_segment_to_protocol)
            .collect(),
        detail: Some(diagnostic.message.clone()),
        operation_index: Some(
            u32::try_from(operation_index)
                .expect("controller bounds every session operation index to u32"),
        ),
        sample_time: None,
        provider_sequence: None,
    }
}

fn session_path_segment_to_protocol(segment: &session::PathSegment) -> crate::PathSegment {
    match segment {
        session::PathSegment::Field(field) => crate::PathSegment::Field(field.clone()),
        session::PathSegment::Index(index) => crate::PathSegment::Index(
            u64::try_from(*index)
                .expect("usize path indices fit the u64 protocol carrier on supported targets"),
        ),
        session::PathSegment::Id(id) => crate::PathSegment::StableId(id.clone()),
    }
}

fn status_for_automation(error: AutomationBatchError) -> StatusCode {
    match error {
        AutomationBatchError::TimeInPast => StatusCode::TimeInPast,
        AutomationBatchError::OutOfOrder
        | AutomationBatchError::Overlap
        | AutomationBatchError::GlobalTimeBackwards => StatusCode::AutomationOrder,
        AutomationBatchError::DensityExceeded | AutomationBatchError::TooManyRecords => {
            StatusCode::LimitExceeded
        }
        _ => StatusCode::InvalidField,
    }
}

fn status_for_parameter(error: ParameterProviderError) -> StatusCode {
    match error {
        ParameterProviderError::NotFound => StatusCode::NotFound,
        ParameterProviderError::Unavailable => StatusCode::Unavailable,
        ParameterProviderError::ReplayExpired => StatusCode::ReplayExpired,
        ParameterProviderError::LimitExceeded => StatusCode::LimitExceeded,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use session::{CompileCaps, canonical_session_json, parse_session_json};
    use std::num::NonZeroUsize;

    const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.json");

    fn id(value: u64) -> RequestId {
        RequestId::new(value).expect("nonzero")
    }

    #[test]
    fn replay_resource_projection_is_bounded_and_overflow_checked() {
        let config = ReplayCacheConfig {
            entries: NonZeroUsize::new(4).expect("entries"),
            bytes: NonZeroUsize::new(1_024).expect("bytes"),
            max_response_bytes: 256,
        };
        let report = ReplayCache::resource_report_for_config(config).expect("projection");
        #[cfg(target_pointer_width = "64")]
        assert_eq!(report.retained_payload_bytes, 1_248);
        assert_eq!(report.largest_allocation_bytes, 1_024);
        let overflow = ReplayCacheConfig {
            entries: NonZeroUsize::new(usize::MAX).expect("maximum is nonzero"),
            ..config
        };
        assert_eq!(
            ReplayCache::resource_report_for_config(overflow),
            Err(ReplayCacheError::ResourceOverflow)
        );
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn replay_layout_stays_within_the_capi_resource_oracle() {
        assert_eq!(core::mem::size_of::<ReplayEntry>(), 56);
        assert_eq!(core::mem::size_of::<ReplayCache>(), 88);
        // #241 re-pin (-24): deleting three source/limit edit variants narrows the embedded
        // prepared-command enum by 24 bytes; all twelve queue endpoints are otherwise unchanged.
        assert_eq!(
            core::mem::size_of::<ProtocolController<MockProvider>>(),
            6_064
        );
        assert_eq!(core::mem::size_of::<PreparedStructuralCommand>(), 752);
    }

    #[test]
    fn replay_prefix_derives_length_retained_bytes_and_highest_pending_id() {
        let mut cache = ReplayCache::new(ReplayCacheConfig {
            entries: NonZeroUsize::new(2).expect("two entries"),
            bytes: NonZeroUsize::new(8).expect("eight bytes"),
            max_response_bytes: 3,
        });
        assert_eq!((cache.len(), cache.retained_bytes()), (0, 0));
        assert_eq!(cache.highest_new_id(), None);

        assert_eq!(cache.preflight(id(1), b"aa"), ReplayDecision::Execute);
        assert_eq!((cache.len(), cache.retained_bytes()), (0, 0));
        assert_eq!(cache.highest_new_id(), Some(id(1)));
        cache.complete(id(1), b"aa", b"x").expect("first");
        assert_eq!((cache.len(), cache.retained_bytes()), (1, 3));
        assert_eq!(cache.highest_new_id(), Some(id(1)));

        assert_eq!(cache.preflight(id(2), b"c"), ReplayDecision::Execute);
        assert_eq!((cache.len(), cache.retained_bytes()), (1, 3));
        assert_eq!(cache.highest_new_id(), Some(id(2)));
        cache.complete(id(2), b"c", b"yy").expect("second");
        let ReplayDecision::Cached(second_hit) = cache.preflight(id(2), b"c") else {
            panic!("second request must hit");
        };
        assert_eq!((cache.len(), cache.retained_bytes()), (2, 6));

        assert_eq!(cache.preflight(id(3), b"d"), ReplayDecision::Execute);
        assert_eq!(
            (cache.len(), cache.retained_bytes(), cache.highest_new_id()),
            (1, 3, Some(id(3))),
            "prefix eviction and pending bytes are derived without retained counters"
        );
        assert_eq!(cache.cached(second_hit), b"yy");
        cache.complete(id(3), b"d", b"zzz").expect("third");
        assert_eq!(
            (cache.len(), cache.retained_bytes(), cache.highest_new_id()),
            (2, 7, Some(id(3)))
        );
        assert_eq!(cache.preflight(id(1), b"aa"), ReplayDecision::ReplayExpired);
    }

    #[test]
    fn fixed_replay_arena_returns_exact_hits_and_compacts_one_evicted_prefix() {
        let mut cache = ReplayCache::new(ReplayCacheConfig {
            entries: NonZeroUsize::new(2).expect("two entries"),
            bytes: NonZeroUsize::new(18).expect("arena bytes"),
            max_response_bytes: 4,
        });
        assert_eq!(cache.retained_storage_capacities(), (2, 18));

        assert_eq!(cache.preflight(id(1), b"one"), ReplayDecision::Execute);
        cache.complete(id(1), b"one", b"1111").expect("first");
        let ReplayDecision::Cached(first_hit) = cache.preflight(id(1), b"one") else {
            panic!("first request must hit");
        };
        assert_eq!(cache.preflight(id(2), b"two"), ReplayDecision::Execute);
        cache.complete(id(2), b"two", b"22").expect("second");

        let ReplayDecision::Cached(second_hit) = cache.preflight(id(2), b"two") else {
            panic!("second request must hit");
        };
        assert_eq!(cache.cached(second_hit), b"22");
        assert_eq!(
            cache.preflight(id(2), b"changed"),
            ReplayDecision::RequestIdReuse
        );

        assert_eq!(cache.preflight(id(3), b"three"), ReplayDecision::Execute);
        cache.complete(id(3), b"three", b"333").expect("third");
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.try_cached(first_hit), None, "evicted hit is stale");
        assert_eq!(cache.cached(first_hit), b"");
        assert_eq!(
            cache.cached(second_hit),
            b"22",
            "surviving hit resolves after compaction"
        );
        assert_eq!(
            cache.preflight(id(1), b"one"),
            ReplayDecision::ReplayExpired
        );
        let ReplayDecision::Cached(compacted_second_hit) = cache.preflight(id(2), b"two") else {
            panic!("second request must survive prefix compaction");
        };
        assert_eq!(cache.cached(compacted_second_hit), b"22");
        let ReplayDecision::Cached(third_hit) = cache.preflight(id(3), b"three") else {
            panic!("third request must hit");
        };
        assert_eq!(cache.cached(third_hit), b"333");
        assert_eq!(cache.retained_storage_capacities(), (2, 18));

        let mut other = ReplayCache::new(ReplayCacheConfig {
            entries: NonZeroUsize::new(2).expect("two entries"),
            bytes: NonZeroUsize::new(18).expect("arena bytes"),
            max_response_bytes: 4,
        });
        assert_eq!(other.preflight(id(2), b"two"), ReplayDecision::Execute);
        other
            .complete(id(2), b"two", b"xx")
            .expect("foreign same-ID entry");
        assert_eq!(
            other.try_cached(second_hit),
            None,
            "foreign hit is rejected"
        );
        assert_eq!(other.cached(second_hit), b"");
    }

    #[test]
    fn replay_reservation_binds_exact_request_bytes() {
        let mut cache = ReplayCache::new(ReplayCacheConfig {
            entries: NonZeroUsize::new(1).expect("one entry"),
            bytes: NonZeroUsize::new(8).expect("arena bytes"),
            max_response_bytes: 4,
        });
        assert_eq!(cache.preflight(id(1), b"aa"), ReplayDecision::Execute);
        assert_eq!(
            cache.preflight(id(2), b"cc"),
            ReplayDecision::Backpressure,
            "pending reservation cannot be overwritten"
        );
        assert_eq!(
            cache.complete(id(1), b"bb", b"ok"),
            Err(ReplayCacheError::ReservationMissing)
        );
        cache
            .complete(id(1), b"aa", b"ok")
            .expect("exact pending request remains retryable");
        let ReplayDecision::Cached(hit) = cache.preflight(id(1), b"aa") else {
            panic!("exact completed request hits");
        };
        assert_eq!(cache.cached(hit), b"ok");
    }

    #[test]
    fn fixed_replay_arena_reports_response_and_reservation_bounds() {
        let config = ReplayCacheConfig {
            entries: NonZeroUsize::new(1).expect("one entry"),
            bytes: NonZeroUsize::new(8).expect("arena bytes"),
            max_response_bytes: 4,
        };
        let mut cache = ReplayCache::new(config);
        assert_eq!(
            cache.complete(id(1), b"x", b"12345"),
            Err(ReplayCacheError::ResponseTooLarge)
        );
        assert_eq!(
            cache.complete(id(1), b"123456789", b""),
            Err(ReplayCacheError::ReservationMissing)
        );
        assert_eq!(
            cache.preflight(id(1), b"12345"),
            ReplayDecision::Backpressure
        );
        assert!(cache.is_empty());
        assert_eq!(cache.retained_bytes(), 0);
    }

    fn controller(
        replay_entries: usize,
        automation_slots: usize,
    ) -> ProtocolController<MockProvider> {
        controller_at_sample(replay_entries, automation_slots, SampleTime(0))
    }

    fn controller_at_sample(
        replay_entries: usize,
        automation_slots: usize,
        current_sample: SampleTime,
    ) -> ProtocolController<MockProvider> {
        let session = SessionStore::new(
            parse_session_json(EXAMPLE).expect("fixture"),
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("session");
        let queues = ProtocolQueues::prepare(crate::ProtocolQueueConfig {
            control_command_slots: NonZeroUsize::new(1).expect("one"),
            control_command_bytes: NonZeroUsize::new(64).expect("bytes"),
            automation_batch_slots: NonZeroUsize::new(automation_slots).expect("automation"),
            reliable_response_slots: NonZeroUsize::new(1).expect("response"),
            reliable_event_slots: NonZeroUsize::new(1).expect("event"),
            telemetry_slots: NonZeroUsize::new(1).expect("telemetry"),
            per_block_automation_density: NonZeroUsize::new(256).expect("density"),
            quantum_frames: NonZeroUsize::new(1).expect("quantum"),
        })
        .expect("queues");
        ProtocolController::new(
            session,
            queues,
            MockProvider {
                current_sample,
                parameter_metadata: vec![automation_descriptor()],
                parameter_state: ParameterStatePage {
                    observed_sample: current_sample.0,
                    records: vec![crate::ParameterStateRecord {
                        handle: 1,
                        flags: 1,
                        value: 0.0,
                    }],
                },
                counter_snapshot: CounterSnapshot {
                    observed_sample: current_sample,
                    values: vec![
                        crate::CounterValue {
                            id: crate::CounterId::ControlCommandBackpressure,
                            value: 9,
                        },
                        crate::CounterValue {
                            id: crate::CounterId::TelemetryCoalesced,
                            value: 3,
                        },
                    ],
                },
                diagnostics: vec![
                    retained_diagnostic(3, crate::DiagnosticSeverity::Warning),
                    retained_diagnostic(4, crate::DiagnosticSeverity::Error),
                ],
                ..MockProvider::default()
            },
            ReplayCache::new(ReplayCacheConfig {
                entries: NonZeroUsize::new(replay_entries).expect("entries"),
                bytes: NonZeroUsize::new(4096).expect("bytes"),
                max_response_bytes: 1024,
            }),
        )
    }

    #[test]
    fn frozen_deep_transaction_reaches_public_b1b_process_path() {
        let corpus = crate::complete_schema_corpus();
        let transaction = corpus
            .iter()
            .find(|frame| frame.name == "command.session_transaction_apply")
            .expect("frozen transaction frame");
        controller(8, 1)
            .process_b1b_btlv(
                &transaction.bytes,
                &mut DecodeScratch::new(&mut [0_u16; 1024]),
            )
            .expect("public B1b process path accepts the frozen deep transaction");
    }

    #[test]
    fn public_b1b_uses_exactly_the_typed_reader_passes_and_replays_identical_bytes() {
        fn assert_single_typed_dispatch(frame_name: &str) {
            let corpus = crate::complete_schema_corpus();
            let frame = corpus
                .iter()
                .find(|frame| frame.name == frame_name)
                .expect("frozen command frame");
            let codec = ProtocolCodec::default();

            crate::btlv::reset_reader_passes();
            codec
                .decode_typed_command(&frame.bytes, &mut DecodeScratch::new(&mut [0_u16; 1024]))
                .expect("typed baseline decode");
            let typed_reader_passes = crate::btlv::reader_passes();
            assert!(typed_reader_passes > 0);

            let mut controller = controller(8, 1);
            crate::btlv::reset_reader_passes();
            let first = controller
                .process_b1b_btlv(&frame.bytes, &mut DecodeScratch::new(&mut [0_u16; 1024]))
                .expect("single typed dispatch");
            assert_eq!(
                crate::btlv::reader_passes(),
                typed_reader_passes,
                "controller must add no generic structural walk for {frame_name}"
            );

            let replay = controller
                .process_b1b_btlv(&frame.bytes, &mut DecodeScratch::new(&mut [0_u16; 1024]))
                .expect("exact canonical replay");
            assert_eq!(
                replay, first,
                "status and exact bytes replay for {frame_name}"
            );
        }

        assert_single_typed_dispatch("command.transport_get");
        assert_single_typed_dispatch("command.session_transaction_apply");

        let corpus = crate::complete_schema_corpus();
        let transaction = corpus
            .iter()
            .find(|frame| frame.name == "command.session_transaction_apply")
            .expect("frozen transaction frame");
        let mut limited = controller(8, 1);
        // One below the fixture's own edit count: #241 took the corpus from 42 edits to 39 by
        // deleting opcodes 0x0006/0x0102/0x0104, so the boundary this row exists to probe moved
        // 41 -> 38. A smaller number still refuses, but it stops being a boundary.
        assert_eq!(crate::complete_all_opcode_fixture().len(), 39);
        limited.config.maximum_transaction_edits = 38;
        assert_eq!(
            limited.process_b1b_btlv(
                &transaction.bytes,
                &mut DecodeScratch::new(&mut [0_u16; 1024]),
            ),
            Err(DecodeError::LimitExceeded)
        );
        assert_eq!(limited.session().revision(), SessionRevision(7));
        assert!(limited.replay.is_empty());
    }

    fn egress_controller(
        reliable_event_slots: usize,
        telemetry_slots: usize,
    ) -> ProtocolController<MockProvider> {
        let session = SessionStore::new(
            parse_session_json(EXAMPLE).expect("fixture"),
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("session");
        let queues = ProtocolQueues::prepare(crate::ProtocolQueueConfig {
            control_command_slots: NonZeroUsize::new(1).expect("control"),
            control_command_bytes: NonZeroUsize::new(64).expect("bytes"),
            automation_batch_slots: NonZeroUsize::new(1).expect("automation"),
            reliable_response_slots: NonZeroUsize::new(1).expect("response"),
            reliable_event_slots: NonZeroUsize::new(reliable_event_slots).expect("events"),
            telemetry_slots: NonZeroUsize::new(telemetry_slots).expect("telemetry"),
            per_block_automation_density: NonZeroUsize::new(256).expect("density"),
            quantum_frames: NonZeroUsize::new(1).expect("quantum"),
        })
        .expect("queues");
        ProtocolController::new(
            session,
            queues,
            MockProvider::default(),
            ReplayCache::new(ReplayCacheConfig {
                entries: NonZeroUsize::new(8).expect("replay"),
                bytes: NonZeroUsize::new(32 * 1024).expect("bytes"),
                max_response_bytes: 4096,
            }),
        )
    }

    fn configure_event_egress(
        controller: &mut ProtocolController<MockProvider>,
        request_id: u64,
        meter_handles: Vec<u32>,
        counter_ids: Vec<crate::CounterId>,
        diagnostics_enabled: bool,
    ) -> TelemetryConfiguration {
        let configuration = TelemetryConfiguration {
            meter_period_blocks: if meter_handles.is_empty() { 0 } else { 1 },
            counter_period_blocks: if counter_ids.is_empty() { 0 } else { 1 },
            meter_handles,
            counter_ids,
            diagnostics_enabled,
            minimum_diagnostic_severity: crate::DiagnosticSeverity::Info,
        };
        let response = controller.process(ControllerRequest {
            request_id: id(request_id),
            expected_revision: ExpectedRevision::Exact(controller.session().revision()),
            canonical_bytes: b"typed-event-egress-config",
            command: ControlCommand::TelemetryConfigure {
                configuration: configuration.clone(),
            },
        });
        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(controller.telemetry_configuration, configuration);
        configuration
    }

    fn automation_descriptor() -> crate::ParameterDescriptor {
        crate::ParameterDescriptor {
            handle: 1,
            track_id: "vocal".to_owned(),
            rack: crate::ParameterRack::Dynamic,
            effect_id: "comp".to_owned(),
            parameter_id: 1,
            channel: crate::ParameterChannel::Left,
            value_kind: crate::ParameterValueKind::F32,
            unit: crate::ParameterUnit::Db,
            domain: crate::ParameterDomain::Continuous,
            minimum: Some(-1.0),
            maximum: Some(1.0),
            default: 0.0,
            mapping: crate::ParameterMapping::Linear,
            automation_rate: crate::ParameterAutomationRate::Sample,
            smoothing_samples: 0,
            flags: 3,
            display_name: None,
            display_unit: None,
            enum_choices: Vec::new(),
        }
    }

    fn retained_diagnostic(sequence: u64, severity: crate::DiagnosticSeverity) -> Diagnostic {
        Diagnostic {
            code: "provider.retained".to_owned(),
            severity,
            path: Vec::new(),
            detail: None,
            operation_index: None,
            sample_time: None,
            provider_sequence: Some(sequence),
        }
    }

    fn capability<'a>(request_id: u64, bytes: &'a [u8]) -> ControllerRequest<'a> {
        ControllerRequest {
            request_id: id(request_id),
            expected_revision: ExpectedRevision::Any,
            canonical_bytes: bytes,
            command: ControlCommand::CapabilitiesGet,
        }
    }

    fn batch(request_id: u64, sample: u64) -> AutomationBatchSlot {
        AutomationBatchSlot::new(
            SessionRevision(7),
            id(request_id),
            &[crate::AutomationRecord {
                kind: crate::AutomationKind::Point,
                handle: crate::ParameterHandle(1),
                start: SampleTime(sample),
                end: SampleTime(sample),
                start_value: 1.0,
                end_value: 1.0,
            }],
        )
        .expect("batch")
    }

    fn full_command(
        request_id: u64,
        expected_revision: ExpectedRevision,
        payload: crate::CommandPayload<'_>,
    ) -> Vec<u8> {
        let codec = ProtocolCodec::default();
        let mut bytes = vec![0_u8; 16 * 1024];
        let length = codec
            .encode_command_frame_into(
                &crate::TypedCommandFrame {
                    request_id: id(request_id),
                    expected_revision,
                    payload,
                },
                &mut bytes,
            )
            .expect("typed command frame");
        bytes.truncate(length);
        bytes
    }

    fn process_full_command(
        controller: &mut ProtocolController<MockProvider>,
        input: &[u8],
    ) -> Vec<u8> {
        let mut output = [0_u8; 2048];
        let mut fields = [0_u16; 1024];
        let length = controller
            .process_command_frame_into(input, &mut DecodeScratch::new(&mut fields), &mut output)
            .expect("full-frame processing");
        output[..length].to_vec()
    }

    #[test]
    fn full_frame_ingress_dispatches_every_registered_command_through_typed_frames() {
        let revision = ExpectedRevision::Exact(SessionRevision(7));
        let transaction = [SessionEdit::SetSessionId {
            session_id: session::StableId::parse("framed-session").expect("ID"),
        }];
        let parameter_state = ParameterStateRequest { handles: vec![1] };
        let automation_records = [crate::AutomationRecord {
            kind: crate::AutomationKind::Point,
            handle: ParameterHandle(1),
            start: SampleTime(0),
            end: SampleTime(0),
            start_value: 0.5,
            end_value: 0.5,
        }];
        let telemetry = TelemetryConfiguration {
            meter_handles: vec![1],
            meter_period_blocks: 1,
            counter_ids: vec![crate::CounterId::ControlCommandBackpressure],
            counter_period_blocks: 1,
            diagnostics_enabled: false,
            minimum_diagnostic_severity: crate::DiagnosticSeverity::Info,
        };
        let counters = CountersRequest {
            all: true,
            ids: Vec::new(),
        };

        let cases = [
            (
                MessageId::CapabilitiesGet,
                full_command(
                    1,
                    ExpectedRevision::Any,
                    crate::CommandPayload::CapabilitiesGet,
                ),
            ),
            (
                MessageId::SessionSnapshotGet,
                full_command(
                    1,
                    ExpectedRevision::Any,
                    crate::CommandPayload::SessionSnapshotGet(crate::SessionSnapshotRequest {
                        offset: 0,
                        maximum_bytes: 32,
                    }),
                ),
            ),
            (
                MessageId::SessionTransactionApply,
                full_command(
                    1,
                    revision,
                    crate::CommandPayload::SessionTransactionApply(&transaction),
                ),
            ),
            (
                MessageId::ParameterMetadataGet,
                full_command(
                    1,
                    ExpectedRevision::Any,
                    crate::CommandPayload::ParameterMetadataGet(ParameterMetadataRequest {
                        after_handle: 0,
                        limit: 1,
                    }),
                ),
            ),
            (
                MessageId::ParameterStateGet,
                full_command(
                    1,
                    ExpectedRevision::Any,
                    crate::CommandPayload::ParameterStateGet(&parameter_state),
                ),
            ),
            (
                MessageId::AutomationEnqueue,
                full_command(
                    1,
                    revision,
                    crate::CommandPayload::AutomationEnqueue(crate::AutomationEnqueue {
                        records: &automation_records,
                    }),
                ),
            ),
            (
                MessageId::TransportGet,
                full_command(
                    1,
                    ExpectedRevision::Any,
                    crate::CommandPayload::TransportGet,
                ),
            ),
            (
                MessageId::TransportSet,
                full_command(
                    1,
                    revision,
                    crate::CommandPayload::TransportSet(TransportSetRequest {
                        state: TransportState::Playing,
                        position: Some(SampleTime(48)),
                    }),
                ),
            ),
            (
                MessageId::TelemetryConfigure,
                full_command(
                    1,
                    revision,
                    crate::CommandPayload::TelemetryConfigure(&telemetry),
                ),
            ),
            (
                MessageId::CountersGet,
                full_command(
                    1,
                    ExpectedRevision::Any,
                    crate::CommandPayload::CountersGet(&counters),
                ),
            ),
            (
                MessageId::DiagnosticsGet,
                full_command(
                    1,
                    ExpectedRevision::Any,
                    crate::CommandPayload::DiagnosticsGet(DiagnosticsRequest {
                        after_sequence: 0,
                        limit: 1,
                        minimum_severity: crate::DiagnosticSeverity::Info,
                    }),
                ),
            ),
        ];

        let codec = ProtocolCodec::default();
        for (message_id, input) in cases {
            let response = process_full_command(&mut controller(8, 2), &input);
            let mut fields = [0_u16; 1024];
            match codec
                .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
                .expect("typed full response")
            {
                crate::DecodedTypedResponseFrame::Success { header, .. } => {
                    assert_eq!(header.message_id, message_id);
                    assert_eq!(header.request_id, id(1));
                }
                crate::DecodedTypedResponseFrame::NonOk { header, .. } => {
                    panic!("{message_id:?} unexpectedly returned {:#?}", header.status);
                }
            }
        }
    }

    #[test]
    fn full_frame_ingress_replays_exact_bytes_and_rejects_changed_request_reuse() {
        let records = [crate::AutomationRecord {
            kind: crate::AutomationKind::Point,
            handle: ParameterHandle(1),
            start: SampleTime(0),
            end: SampleTime(0),
            start_value: 0.25,
            end_value: 0.25,
        }];
        let input = full_command(
            1,
            ExpectedRevision::Exact(SessionRevision(7)),
            crate::CommandPayload::AutomationEnqueue(crate::AutomationEnqueue {
                records: &records,
            }),
        );
        let mut endpoint = controller(8, 2);
        let first = process_full_command(&mut endpoint, &input);
        assert_eq!(
            endpoint
                .queues_mut()
                .report(crate::QueueKind::Automation)
                .occupancy,
            1
        );
        let replay = process_full_command(&mut endpoint, &input);
        assert_eq!(replay, first);
        assert_eq!(
            endpoint
                .queues_mut()
                .report(crate::QueueKind::Automation)
                .occupancy,
            1,
            "replay must not enqueue a second batch"
        );

        let changed = full_command(
            1,
            ExpectedRevision::Any,
            crate::CommandPayload::TransportGet,
        );
        let response = process_full_command(&mut endpoint, &changed);
        let mut fields = [0_u16; 32];
        match ProtocolCodec::default()
            .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
            .expect("request-reuse response")
        {
            crate::DecodedTypedResponseFrame::NonOk { header, .. } => {
                assert_eq!(header.status, StatusCode::RequestIdReuse);
                assert_eq!(header.message_id, MessageId::TransportGet);
            }
            crate::DecodedTypedResponseFrame::Success { .. } => panic!("changed ID was executed"),
        }
    }

    #[test]
    fn public_immediate_frame_path_never_builds_a_prepared_vec() {
        PREPARED_IMMEDIATE_CALLS.with(|calls| calls.set(0));
        let mut endpoint = controller(8, 2);
        let input = full_command(
            1,
            ExpectedRevision::Any,
            crate::CommandPayload::CapabilitiesGet,
        );
        let first = process_full_command(&mut endpoint, &input);
        assert_eq!(
            process_full_command(&mut endpoint, &input),
            first,
            "replay bytes"
        );

        let changed = full_command(
            1,
            ExpectedRevision::Any,
            crate::CommandPayload::TransportGet,
        );
        let _reuse = process_full_command(&mut endpoint, &changed);

        let mut malformed = full_command(
            2,
            ExpectedRevision::Any,
            crate::CommandPayload::CapabilitiesGet,
        );
        malformed[20..24].copy_from_slice(&8_u32.to_le_bytes());
        malformed[40..44].copy_from_slice(&1_u32.to_le_bytes());
        malformed.extend_from_slice(&[1, 0, 1, 0, 1, 0, 0, 0]);
        let _non_ok = process_full_command(&mut endpoint, &malformed);

        PREPARED_IMMEDIATE_CALLS.with(|calls| {
            assert_eq!(
                calls.get(),
                0,
                "public immediate, replay, reuse, and correlatable-error paths write directly"
            );
        });
    }

    #[test]
    fn one_call_transaction_success_and_fallbacks_avoid_owned_preparation() {
        fn status(bytes: &[u8]) -> StatusCode {
            match ProtocolCodec::default()
                .decode_typed_response(bytes, &mut DecodeScratch::new(&mut [0_u16; 64]))
                .expect("typed response")
            {
                crate::DecodedTypedResponseFrame::Success { header, .. }
                | crate::DecodedTypedResponseFrame::NonOk { header, .. } => header.status,
            }
        }

        PREPARED_IMMEDIATE_CALLS.with(|calls| calls.set(0));
        PROSPECTIVE_REPLAY_CLONES.with(|clones| clones.set(0));
        RESPONSE_STAGING_VECS.with(|allocations| allocations.set(0));
        let first_edits = [SessionEdit::SetSessionId {
            session_id: session::StableId::parse("direct-transaction").expect("ID"),
        }];
        let first = full_command(
            1,
            ExpectedRevision::Exact(SessionRevision(7)),
            crate::CommandPayload::SessionTransactionApply(&first_edits),
        );
        let mut endpoint = controller(1, 2);
        TYPED_COMMAND_DECODES.with(|decodes| decodes.set(0));
        crate::typed_frame::reset_frame_writer_passes();
        let committed = process_full_command(&mut endpoint, &first);
        assert_eq!(status(&committed), StatusCode::Ok);
        TYPED_COMMAND_DECODES.with(|decodes| assert_eq!(decodes.get(), 1));
        assert_eq!(crate::typed_frame::frame_writer_passes(), (1, 1));
        PROSPECTIVE_REPLAY_CLONES.with(|clones| assert_eq!(clones.get(), 0));
        RESPONSE_STAGING_VECS.with(|allocations| assert_eq!(allocations.get(), 0));
        let committed_snapshot = endpoint.session().canonical_snapshot().to_owned();
        let committed_events = endpoint.queues().report(crate::QueueKind::ReliableEvent);

        let replay = process_full_command(&mut endpoint, &first);
        assert_eq!(replay, committed, "transaction replay bytes");
        assert_eq!(endpoint.session().canonical_snapshot(), committed_snapshot);
        assert_eq!(
            endpoint.queues().report(crate::QueueKind::ReliableEvent),
            committed_events
        );
        let event = endpoint
            .queues_mut()
            .try_dequeue_event()
            .expect("one committed event");
        assert_eq!(event.message_id, MessageId::SessionCommitted);
        assert!(
            endpoint.queues_mut().try_dequeue_event().is_err(),
            "one-call commit and exact replay emit one event total"
        );

        let changed_edits = [SessionEdit::SetSessionId {
            session_id: session::StableId::parse("changed-reuse").expect("ID"),
        }];
        let changed = full_command(
            1,
            ExpectedRevision::Exact(SessionRevision(8)),
            crate::CommandPayload::SessionTransactionApply(&changed_edits),
        );
        assert_eq!(
            status(&process_full_command(&mut endpoint, &changed)),
            StatusCode::RequestIdReuse
        );
        assert_eq!(endpoint.session().canonical_snapshot(), committed_snapshot);
        PROSPECTIVE_REPLAY_CLONES.with(|clones| {
            assert_eq!(
                clones.get(),
                0,
                "cached and reuse IDs bypass prospective replay allocation"
            );
        });

        let evict = full_command(
            2,
            ExpectedRevision::Any,
            crate::CommandPayload::TransportGet,
        );
        assert_eq!(
            status(&process_full_command(&mut endpoint, &evict)),
            StatusCode::Ok
        );
        assert_eq!(
            status(&process_full_command(&mut endpoint, &first)),
            StatusCode::ReplayExpired
        );
        assert_eq!(endpoint.session().canonical_snapshot(), committed_snapshot);
        PROSPECTIVE_REPLAY_CLONES.with(|clones| {
            assert_eq!(
                clones.get(),
                0,
                "expired IDs bypass prospective replay allocation"
            );
        });

        let conflict_edits = [SessionEdit::SetSessionId {
            session_id: session::StableId::parse("must-not-commit").expect("ID"),
        }];
        let conflict = full_command(
            3,
            ExpectedRevision::Exact(SessionRevision(6)),
            crate::CommandPayload::SessionTransactionApply(&conflict_edits),
        );
        let mut conflicted = controller(4, 2);
        let before_conflict = conflicted.session().canonical_snapshot().to_owned();
        let clones_before_conflict = PROSPECTIVE_REPLAY_CLONES.with(core::cell::Cell::get);
        let direct_conflict = process_full_command(&mut conflicted, &conflict);
        assert_eq!(status(&direct_conflict), StatusCode::RevisionConflict);
        assert_eq!(conflicted.session().canonical_snapshot(), before_conflict);
        PROSPECTIVE_REPLAY_CLONES.with(|clones| {
            assert_eq!(
                clones.get(),
                clones_before_conflict,
                "header-known semantic fallback bypasses prospective replay allocation"
            );
        });

        let invalid_edits = [SessionEdit::SetSampleRateHz { sample_rate_hz: 0 }];
        let invalid = full_command(
            4,
            ExpectedRevision::Exact(SessionRevision(7)),
            crate::CommandPayload::SessionTransactionApply(&invalid_edits),
        );
        let mut invalid_endpoint = controller(4, 2);
        let invalid_snapshot = invalid_endpoint.session().canonical_snapshot().to_owned();
        TYPED_COMMAND_DECODES.with(|decodes| decodes.set(0));
        assert_eq!(
            status(&process_full_command(&mut invalid_endpoint, &invalid)),
            StatusCode::ValidationFailed
        );
        TYPED_COMMAND_DECODES.with(|decodes| assert_eq!(decodes.get(), 1));
        assert_eq!(
            invalid_endpoint.session().canonical_snapshot(),
            invalid_snapshot
        );
        PROSPECTIVE_REPLAY_CLONES.with(|clones| {
            assert_eq!(
                clones.get(),
                clones_before_conflict,
                "session-prepare fallback bypasses prospective replay allocation"
            );
        });

        let mut prepared_compat = controller(4, 2);
        let prepared = prepared_compat
            .prepare_command_frame(&conflict, &mut DecodeScratch::new(&mut [0_u16; 1024]), 2048)
            .expect("public owned fallback");
        let PreparedCommandFrame::Immediate(prepared) = prepared else {
            panic!("revision conflict must not produce a structural token");
        };
        let mut prepared_bytes = [0_u8; 2048];
        let prepared_len = prepared
            .write_into(&mut prepared_bytes)
            .expect("prepared compatibility bytes");
        assert_eq!(&prepared_bytes[..prepared_len], direct_conflict);

        let mut pressured = controller(4, 2);
        pressured.replay = ReplayCache::new(ReplayCacheConfig {
            entries: NonZeroUsize::new(1).expect("entry"),
            bytes: NonZeroUsize::new(64).expect("arena"),
            max_response_bytes: 32,
        });
        let before_pressure = pressured.session().canonical_snapshot().to_owned();
        assert_eq!(
            status(&process_full_command(&mut pressured, &first)),
            StatusCode::Backpressure
        );
        assert_eq!(pressured.session().canonical_snapshot(), before_pressure);
        assert!(pressured.replay.is_empty());
        PROSPECTIVE_REPLAY_CLONES.with(|clones| {
            assert_eq!(
                clones.get(),
                clones_before_conflict,
                "replay-capacity backpressure bypasses prospective replay allocation"
            );
        });

        PREPARED_IMMEDIATE_CALLS.with(|calls| {
            assert_eq!(
                calls.get(),
                1,
                "only the explicit public prepare fallback may build owned immediate bytes"
            );
        });
    }

    #[test]
    fn full_frame_ingress_preserves_output_ownership_and_correlates_payload_errors() {
        let transaction = [SessionEdit::SetSessionId {
            session_id: session::StableId::parse("not-committed").expect("ID"),
        }];
        let input = full_command(
            1,
            ExpectedRevision::Exact(SessionRevision(7)),
            crate::CommandPayload::SessionTransactionApply(&transaction),
        );
        let mut endpoint = controller(8, 2);
        PROSPECTIVE_REPLAY_CLONES.with(|clones| clones.set(0));
        RESPONSE_STAGING_VECS.with(|allocations| allocations.set(0));
        let mut short = [0xa5_u8; 32];
        let mut fields = [0_u16; 16];
        assert_eq!(
            endpoint.process_command_frame_into(
                &input,
                &mut DecodeScratch::new(&mut fields),
                &mut short,
            ),
            Err(CommandFrameProcessError::OutputReservationTooSmall { required: 1024 })
        );
        assert_eq!(short, [0xa5; 32]);
        assert_eq!(endpoint.session().revision(), SessionRevision(7));
        assert!(endpoint.replay().is_empty());
        assert_eq!(
            endpoint
                .queues()
                .report(crate::QueueKind::ReliableEvent)
                .occupancy,
            0
        );
        PROSPECTIVE_REPLAY_CLONES.with(|clones| assert_eq!(clones.get(), 0));
        RESPONSE_STAGING_VECS.with(|allocations| assert_eq!(allocations.get(), 0));

        let before_malformed_transaction = endpoint.session().canonical_snapshot().to_owned();
        let mut malformed_transaction = input.clone();
        malformed_transaction[crate::OUTER_HEADER_BYTES + 2] = 1;
        TYPED_COMMAND_DECODES.with(|decodes| decodes.set(0));
        let transaction_response = process_full_command(&mut endpoint, &malformed_transaction);
        TYPED_COMMAND_DECODES.with(|decodes| {
            assert_eq!(
                decodes.get(),
                1,
                "a malformed transaction carries its typed-decode error into the direct outcome path"
            );
        });
        let mut transaction_response_fields = [0_u16; 32];
        match ProtocolCodec::default()
            .decode_typed_response(
                &transaction_response,
                &mut DecodeScratch::new(&mut transaction_response_fields),
            )
            .expect("correlatable malformed transaction response")
        {
            crate::DecodedTypedResponseFrame::NonOk { header, .. } => {
                assert_eq!(header.request_id, id(1));
                assert_eq!(header.message_id, MessageId::SessionTransactionApply);
                assert_eq!(header.status, StatusCode::MalformedFrame);
            }
            crate::DecodedTypedResponseFrame::Success { .. } => {
                panic!("malformed transaction payload succeeded")
            }
        }
        assert_eq!(
            endpoint.session().canonical_snapshot(),
            before_malformed_transaction
        );
        assert_eq!(
            endpoint
                .queues()
                .report(crate::QueueKind::ReliableEvent)
                .occupancy,
            0
        );

        let mut malformed = full_command(
            2,
            ExpectedRevision::Any,
            crate::CommandPayload::CapabilitiesGet,
        );
        malformed[20..24].copy_from_slice(&8_u32.to_le_bytes());
        malformed[40..44].copy_from_slice(&1_u32.to_le_bytes());
        malformed.extend_from_slice(&[1, 0, 1, 0, 1, 0, 0, 0]);
        TYPED_COMMAND_DECODES.with(|decodes| decodes.set(0));
        let response = process_full_command(&mut endpoint, &malformed);
        TYPED_COMMAND_DECODES.with(|decodes| assert_eq!(decodes.get(), 1));
        let mut response_fields = [0_u16; 32];
        match ProtocolCodec::default()
            .decode_typed_response(&response, &mut DecodeScratch::new(&mut response_fields))
            .expect("correlatable error response")
        {
            crate::DecodedTypedResponseFrame::NonOk { header, .. } => {
                assert_eq!(header.request_id, id(2));
                assert_eq!(header.message_id, MessageId::CapabilitiesGet);
                assert_eq!(header.status, StatusCode::MalformedFrame);
            }
            crate::DecodedTypedResponseFrame::Success { .. } => {
                panic!("malformed payload succeeded")
            }
        }

        let mut uncorrelatable = malformed;
        uncorrelatable[0] ^= 1;
        let mut output = [0x5a_u8; 2048];
        assert_eq!(
            endpoint.process_command_frame_into(
                &uncorrelatable,
                &mut DecodeScratch::new(&mut [0_u16; 32]),
                &mut output,
            ),
            Err(CommandFrameProcessError::Uncorrelatable(
                DecodeError::BadMagic
            ))
        );
        assert_eq!(output, [0x5a; 2048]);

        let mut owned = [0x3c_u8; 2048];
        let mut owned_fields = [0_u16; 32];
        let length = endpoint
            .process_command_frame_into(
                &full_command(
                    3,
                    ExpectedRevision::Any,
                    crate::CommandPayload::TransportGet,
                ),
                &mut DecodeScratch::new(&mut owned_fields),
                &mut owned,
            )
            .expect("owned response");
        assert!(owned[..length].iter().any(|byte| *byte != 0x3c));
        assert!(owned[length..].iter().all(|byte| *byte == 0x3c));
    }

    #[test]
    fn structural_prepare_is_invisible_until_commit_and_byte_identical_to_one_call() {
        let transaction = [SessionEdit::SetSessionId {
            session_id: session::StableId::parse("prepared-session").expect("ID"),
        }];
        let input = full_command(
            1,
            ExpectedRevision::Exact(SessionRevision(7)),
            crate::CommandPayload::SessionTransactionApply(&transaction),
        );

        let mut accepted = controller(8, 2);
        let mut accepted_output = [0_u8; 2048];
        let accepted_len = accepted
            .process_command_frame_into(
                &input,
                &mut DecodeScratch::new(&mut [0_u16; 1024]),
                &mut accepted_output,
            )
            .expect("accepted one-call path");
        let accepted_event = accepted.queues_mut().try_dequeue_event().expect("event");

        let mut endpoint = controller(8, 2);
        let before_snapshot = endpoint.session().canonical_snapshot().to_owned();
        let before_event = endpoint
            .queues_mut()
            .report(crate::QueueKind::ReliableEvent);
        PROSPECTIVE_REPLAY_CLONES.with(|clones| clones.set(0));
        RESPONSE_STAGING_VECS.with(|allocations| allocations.set(0));
        let prepared = match endpoint
            .prepare_command_frame(&input, &mut DecodeScratch::new(&mut [0_u16; 1024]), 2048)
            .expect("prepare")
        {
            PreparedCommandFrame::Structural(prepared) => *prepared,
            PreparedCommandFrame::Immediate(_) => panic!("valid transaction was immediate"),
        };
        PROSPECTIVE_REPLAY_CLONES.with(|clones| assert_eq!(clones.get(), 1));
        RESPONSE_STAGING_VECS.with(|allocations| assert_eq!(allocations.get(), 1));
        assert_eq!(endpoint.session().revision(), SessionRevision(7));
        assert_eq!(endpoint.session().canonical_snapshot(), before_snapshot);
        assert!(endpoint.replay().is_empty());
        assert_eq!(
            endpoint.queues().report(crate::QueueKind::ReliableEvent),
            before_event
        );
        assert_eq!(
            prepared.prospective_session().revision(),
            SessionRevision(8)
        );
        assert!(
            prepared
                .prospective_session()
                .compiled()
                .canonical_json()
                .contains("prepared-session")
        );

        let committed = endpoint
            .commit_prepared_structural(prepared)
            .expect("affine commit");
        let mut output = [0xa5_u8; 2048];
        let committed_len = committed.write_into(&mut output).expect("committed bytes");
        assert_eq!(committed_len, accepted_len);
        assert_eq!(&output[..committed_len], &accepted_output[..accepted_len]);
        assert!(output[committed_len..].iter().all(|byte| *byte == 0xa5));
        assert_eq!(endpoint.session().revision(), SessionRevision(8));
        assert_eq!(
            endpoint.queues_mut().try_dequeue_event().expect("event"),
            accepted_event
        );
        assert_eq!(endpoint.replay().len(), 1);
    }

    #[test]
    fn structural_token_cancel_owner_generation_and_serial_rules_are_affine() {
        fn structural(
            endpoint: &mut ProtocolController<MockProvider>,
            input: &[u8],
        ) -> PreparedStructuralCommand {
            match endpoint
                .prepare_command_frame(input, &mut DecodeScratch::new(&mut [0_u16; 1024]), 2048)
                .expect("prepare")
            {
                PreparedCommandFrame::Structural(prepared) => *prepared,
                PreparedCommandFrame::Immediate(_) => panic!("expected structural token"),
            }
        }

        let edit = |name: &'static str| {
            [SessionEdit::SetSessionId {
                session_id: session::StableId::parse(name).expect("ID"),
            }]
        };
        let first_edits = edit("token-first");
        let first = full_command(
            1,
            ExpectedRevision::Exact(SessionRevision(7)),
            crate::CommandPayload::SessionTransactionApply(&first_edits),
        );
        let mut owner = controller(8, 2);
        let mut other = controller(8, 2);

        let canceled = structural(&mut owner, &first);
        assert!(matches!(
            owner.prepare_command_frame(&first, &mut DecodeScratch::new(&mut [0_u16; 1024]), 2048,),
            Err(CommandFrameProcessError::PreparedCommandOutstanding)
        ));
        let blocked = owner.process(capability(99, b"must-not-enter-replay"));
        assert_eq!(blocked.status, StatusCode::Backpressure);
        assert!(owner.replay().is_empty());
        assert_eq!(owner.session().revision(), SessionRevision(7));
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let _ = owner.queues_mut();
            }))
            .is_err()
        );
        assert!(matches!(
            owner.enqueue_diagnostic_event(
                SessionRevision(7),
                DiagnosticEvent {
                    diagnostic: retained_diagnostic(1, crate::DiagnosticSeverity::Error),
                },
            ),
            Err(EventEgressError::ReliableQueueFull(_))
        ));
        assert!(
            owner
                .cancel_pending_automation(
                    AutomationCancellationReason::ExplicitReconfiguration,
                    Some(SampleTime(0)),
                )
                .is_err()
        );
        drop(canceled);
        assert_eq!(owner.session().revision(), SessionRevision(7));
        assert!(owner.replay().is_empty());
        owner
            .queues_mut()
            .try_enqueue_event(ReliableSlot::session_committed(
                SessionRevision(7),
                77,
                id(77),
                SessionRevision(6),
                1,
            ))
            .expect("drop released exact reservation");
        let _ = owner
            .queues_mut()
            .try_dequeue_event()
            .expect("clear reservation probe");

        let wrong_owner = structural(&mut owner, &first);
        assert!(matches!(
            other.commit_prepared_structural(wrong_owner),
            Err(PreparedCommandCommitError::WrongController)
        ));
        assert_eq!(owner.session().revision(), SessionRevision(7));
        assert_eq!(other.session().revision(), SessionRevision(7));

        let stale = structural(&mut owner, &first);
        owner.structural_generation.fetch_add(2, Ordering::AcqRel);
        assert!(matches!(
            owner.commit_prepared_structural(stale),
            Err(PreparedCommandCommitError::StaleGeneration)
        ));
        owner.structural_generation.fetch_add(1, Ordering::AcqRel);

        let first_token = structural(&mut owner, &first);
        owner
            .commit_prepared_structural(first_token)
            .expect("first serial commit");
        let _ = owner
            .queues_mut()
            .try_dequeue_event()
            .expect("serial event consumer");
        let second_edits = edit("token-second");
        let second = full_command(
            2,
            ExpectedRevision::Exact(SessionRevision(8)),
            crate::CommandPayload::SessionTransactionApply(&second_edits),
        );
        let second_token = structural(&mut owner, &second);
        owner
            .commit_prepared_structural(second_token)
            .expect("second serial commit");
        assert_eq!(owner.session().revision(), SessionRevision(9));
        assert_eq!(owner.replay().len(), 2);
    }

    #[test]
    fn identical_replay_changed_reuse_eviction_and_expiry_are_deterministic() {
        let mut controller = controller(1, 1);
        let first = controller.process(capability(1, b"first"));
        assert_eq!(first.status, StatusCode::Ok);
        assert_eq!(controller.process(capability(1, b"first")), first);
        assert_eq!(
            controller.process(capability(1, b"changed")).status,
            StatusCode::RequestIdReuse
        );
        assert_eq!(
            controller.process(capability(2, b"second")).status,
            StatusCode::Ok
        );
        assert_eq!(
            controller.process(capability(1, b"first")).status,
            StatusCode::ReplayExpired
        );
        assert_eq!(
            controller.process(capability(1, b"other")).status,
            StatusCode::ReplayExpired
        );
    }

    #[test]
    fn compatibility_response_uses_one_frame_backing_and_cached_header_only_decode() {
        fn assert_shared_payload_backing(response: &ControllerResponse) {
            assert_eq!(
                response.payload().as_ptr(),
                response
                    .frame
                    .as_ptr()
                    .wrapping_add(crate::OUTER_HEADER_BYTES),
                "payload must be a range into the sole complete-frame backing"
            );
            assert_eq!(
                response.frame.len(),
                crate::OUTER_HEADER_BYTES + response.payload_len()
            );
        }

        let mut controller = controller(2, 1);
        let first = controller.process(capability(1, b"single-frame-backing"));
        assert_shared_payload_backing(&first);
        let mut payload = vec![0xa5; first.payload_len()];
        assert_eq!(first.encode_payload(&mut payload), Ok(first.payload_len()));
        assert_eq!(payload, first.payload());

        crate::btlv::reset_reader_passes();
        let cached = controller.process(capability(1, b"single-frame-backing"));
        assert_eq!(
            crate::btlv::reader_passes(),
            0,
            "cached metadata reconstruction must not invoke the recursive TLV Reader"
        );
        assert_eq!(
            cached, first,
            "cache hit preserves exact metadata and bytes"
        );
        assert_shared_payload_backing(&cached);

        let cloned = cached.clone();
        assert_eq!(cloned, cached, "Clone/Eq preserve complete frame bytes");
        assert_shared_payload_backing(&cloned);

        let malformed = ControllerResponse::from_complete_frame(
            id(99),
            StatusCode::Internal,
            SessionRevision(7),
            vec![0xa5; crate::OUTER_HEADER_BYTES - 1],
        );
        assert_eq!(malformed.payload_len(), 0);
        assert_eq!(malformed.payload(), &[]);
        assert_eq!(malformed.encode_payload(&mut []), Ok(0));
    }

    #[test]
    fn mutations_require_exact_revision_and_roll_back_on_validation() {
        let mut controller = controller(4, 1);
        let any = ControllerRequest {
            request_id: id(1),
            expected_revision: ExpectedRevision::Any,
            canonical_bytes: b"any",
            command: ControlCommand::SessionTransactionApply { edits: &[] },
        };
        assert_eq!(controller.process(any).status, StatusCode::InvalidField);
        let invalid = ControllerRequest {
            request_id: id(2),
            expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
            canonical_bytes: b"invalid",
            command: ControlCommand::SessionTransactionApply {
                edits: &[SessionEdit::RemoveSource {
                    source_id: session::StableId::parse("voice").expect("id"),
                }],
            },
        };
        assert_eq!(
            controller.process(invalid).status,
            StatusCode::ValidationFailed
        );
        assert_eq!(controller.session().revision(), SessionRevision(7));
    }

    #[test]
    fn session_diagnostic_path_segments_preserve_field_index_and_stable_id_variants() {
        assert_eq!(
            session_path_segment_to_protocol(&session::PathSegment::Field("tracks".to_owned(),)),
            crate::PathSegment::Field("tracks".to_owned())
        );
        assert_eq!(
            session_path_segment_to_protocol(&session::PathSegment::Index(3)),
            crate::PathSegment::Index(3)
        );
        assert_eq!(
            session_path_segment_to_protocol(&session::PathSegment::Id("vocal".to_owned(),)),
            crate::PathSegment::StableId("vocal".to_owned())
        );
    }

    #[test]
    fn validation_diagnostics_preserve_operation_paths_and_exact_omission_count() {
        let mut controller = controller(4, 1);
        controller.config.maximum_response_diagnostics = 1;
        let mut fader = controller.session().compiled().normalized_model().tracks[0]
            .fader
            .clone();
        fader.left_db = f32::NAN;
        fader.right_db = f32::INFINITY;
        let request = ControllerRequest {
            request_id: id(1),
            expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
            canonical_bytes: b"two-validation-diagnostics",
            command: ControlCommand::SessionTransactionApply {
                edits: &[SessionEdit::SetTrackFader {
                    track_id: session::StableId::parse("vocal").expect("stable ID"),
                    fader,
                }],
            },
        };

        let response = controller.process(request);
        assert_eq!(response.status, StatusCode::ValidationFailed);
        let codec = ProtocolCodec::default();
        let decoded = codec
            .decode_non_ok_payload(response.payload(), 2)
            .expect("canonical validation error");
        assert_eq!(decoded.omitted_diagnostics, 1);
        assert_eq!(decoded.diagnostics.len(), 1);
        assert_eq!(
            decoded.diagnostics[0],
            Diagnostic {
                code: "numeric.non_finite".to_owned(),
                severity: crate::DiagnosticSeverity::Error,
                path: vec![
                    crate::PathSegment::Field("tracks".to_owned()),
                    crate::PathSegment::Index(0),
                    crate::PathSegment::Field("fader".to_owned()),
                    crate::PathSegment::Field("left_db".to_owned()),
                ],
                detail: Some("value must be finite".to_owned()),
                operation_index: Some(1),
                sample_time: None,
                provider_sequence: None,
            }
        );

        let mut encoded = vec![
            0_u8;
            codec
                .encoded_non_ok_payload_len(&decoded)
                .expect("round-trip length")
        ];
        codec
            .encode_non_ok_payload(&decoded, &mut encoded)
            .expect("round-trip encode");
        assert_eq!(
            encoded,
            response.payload(),
            "non-OK payload remains canonical"
        );
    }

    #[test]
    fn edit_rejections_use_typed_operation_diagnostics_not_protocol_failure() {
        let mut controller = controller(4, 1);
        let request = ControllerRequest {
            request_id: id(1),
            expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
            canonical_bytes: b"missing-source-edit",
            command: ControlCommand::SessionTransactionApply {
                edits: &[SessionEdit::RemoveSource {
                    source_id: session::StableId::parse("missing").expect("stable ID"),
                }],
            },
        };
        let response = controller.process(request);
        assert_eq!(response.status, StatusCode::ValidationFailed);
        let decoded = ProtocolCodec::default()
            .decode_non_ok_payload(response.payload(), 2)
            .expect("typed edit error");
        assert_eq!(decoded.omitted_diagnostics, 0);
        assert_eq!(decoded.diagnostics.len(), 1);
        assert_eq!(decoded.diagnostics[0].code, "session.edit.not_found");
        assert_eq!(decoded.diagnostics[0].operation_index, Some(0));
    }

    #[test]
    fn every_non_ok_status_has_a_canonical_common_response_payload() {
        struct Case {
            name: &'static str,
            status: StatusCode,
            top_level_tlvs: u32,
            decoder_error: Option<DecodeError>,
        }

        let cases = [
            Case {
                name: "malformed frame",
                status: StatusCode::MalformedFrame,
                top_level_tlvs: 2,
                decoder_error: Some(DecodeError::BadMagic),
            },
            Case {
                name: "unsupported version",
                status: StatusCode::UnsupportedVersion,
                top_level_tlvs: 2,
                decoder_error: Some(DecodeError::UnsupportedVersion),
            },
            Case {
                name: "unsupported message",
                status: StatusCode::UnsupportedMessage,
                top_level_tlvs: 2,
                decoder_error: Some(DecodeError::UnsupportedMessage),
            },
            Case {
                name: "unknown required field",
                status: StatusCode::UnknownRequiredField,
                top_level_tlvs: 2,
                decoder_error: Some(DecodeError::UnknownRequiredField),
            },
            Case {
                name: "invalid field",
                status: StatusCode::InvalidField,
                top_level_tlvs: 2,
                decoder_error: None,
            },
            Case {
                name: "limit exceeded",
                status: StatusCode::LimitExceeded,
                top_level_tlvs: 2,
                decoder_error: Some(DecodeError::LimitExceeded),
            },
            Case {
                name: "revision conflict",
                status: StatusCode::RevisionConflict,
                top_level_tlvs: 2,
                decoder_error: None,
            },
            Case {
                name: "revision exhausted",
                status: StatusCode::RevisionExhausted,
                top_level_tlvs: 2,
                decoder_error: None,
            },
            Case {
                name: "request ID reuse",
                status: StatusCode::RequestIdReuse,
                top_level_tlvs: 2,
                decoder_error: None,
            },
            Case {
                name: "replay expired",
                status: StatusCode::ReplayExpired,
                top_level_tlvs: 2,
                decoder_error: None,
            },
            Case {
                name: "backpressure",
                status: StatusCode::Backpressure,
                top_level_tlvs: 3,
                decoder_error: None,
            },
            Case {
                name: "validation failed",
                status: StatusCode::ValidationFailed,
                top_level_tlvs: 2,
                decoder_error: None,
            },
            Case {
                name: "not found",
                status: StatusCode::NotFound,
                top_level_tlvs: 2,
                decoder_error: None,
            },
            Case {
                name: "unavailable",
                status: StatusCode::Unavailable,
                top_level_tlvs: 2,
                decoder_error: None,
            },
            Case {
                name: "time in past",
                status: StatusCode::TimeInPast,
                top_level_tlvs: 2,
                decoder_error: None,
            },
            Case {
                name: "automation order",
                status: StatusCode::AutomationOrder,
                top_level_tlvs: 2,
                decoder_error: None,
            },
            Case {
                name: "PCM forbidden",
                status: StatusCode::PcmForbidden,
                top_level_tlvs: 2,
                decoder_error: Some(DecodeError::PcmForbidden),
            },
            Case {
                name: "internal",
                status: StatusCode::Internal,
                top_level_tlvs: 2,
                decoder_error: None,
            },
        ];

        let codec = ProtocolCodec::default();
        for (index, case) in cases.iter().enumerate() {
            if let Some(error) = case.decoder_error {
                assert_eq!(error.status(), case.status, "{} decoder mapping", case.name);
            }
            let controller = controller(4, 1);
            let request_id = id(u64::try_from(index).expect("case index fits") + 1);
            let response = controller.compatibility_response(
                MessageId::CapabilitiesGet,
                request_id,
                controller.non_ok(case.status, None),
            );
            assert_eq!(response.status, case.status, "{} status", case.name);
            let decoded = codec
                .decode_non_ok_payload(response.payload(), case.top_level_tlvs)
                .unwrap_or_else(|error| panic!("{} common non-OK payload: {error:?}", case.name));
            assert_eq!(
                decoded.omitted_diagnostics, 0,
                "{} omitted count",
                case.name
            );
            assert_eq!(
                decoded.backpressure.is_some(),
                case.status == StatusCode::Backpressure,
                "{} typed backpressure presence",
                case.name
            );
            let mut canonical = vec![
                0_u8;
                codec
                    .encoded_non_ok_payload_len(&decoded)
                    .expect("canonical non-OK length")
            ];
            codec
                .encode_non_ok_payload(&decoded, &mut canonical)
                .expect("canonical non-OK encode");
            assert_eq!(
                canonical,
                response.payload(),
                "{} canonical bytes",
                case.name
            );
        }
    }

    #[test]
    fn backpressure_is_cached_without_second_enqueue() {
        let mut controller = controller(4, 1);
        let first = ControllerRequest {
            request_id: id(1),
            expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
            canonical_bytes: b"first-automation",
            command: ControlCommand::AutomationEnqueue { batch: batch(1, 0) },
        };
        assert_eq!(controller.process(first).status, StatusCode::Ok);
        let second = ControllerRequest {
            request_id: id(2),
            expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
            canonical_bytes: b"second-automation",
            command: ControlCommand::AutomationEnqueue { batch: batch(2, 1) },
        };
        let rejected = controller.process(second);
        assert_eq!(rejected.status, StatusCode::Backpressure);
        let _ = controller
            .queues_mut()
            .try_dequeue_automation()
            .expect("first remains");
        let retry = ControllerRequest {
            request_id: id(2),
            expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
            canonical_bytes: b"second-automation",
            command: ControlCommand::AutomationEnqueue { batch: batch(2, 1) },
        };
        assert_eq!(controller.process(retry), rejected);
        assert!(controller.queues_mut().try_dequeue_automation().is_err());
    }

    #[test]
    fn replay_admission_rejects_before_provider_execution() {
        let mut cache = ReplayCache::new(ReplayCacheConfig {
            entries: NonZeroUsize::new(1).expect("one"),
            bytes: NonZeroUsize::new(8).expect("eight"),
            max_response_bytes: 8,
        });
        assert_eq!(cache.preflight(id(1), b"x"), ReplayDecision::Backpressure);
        assert!(cache.is_empty());
        assert_eq!(cache.highest_new_id(), None);
        assert_eq!(cache.preflight(id(1), b"x"), ReplayDecision::Backpressure);
        assert_eq!(cache.highest_new_id(), None);
    }

    #[test]
    fn decoded_btlv_transaction_reaches_same_atomic_session_store() {
        let mut controller = controller(4, 1);
        let edits = [SessionEdit::SetSessionId {
            session_id: session::StableId::parse("renamed").expect("ID"),
        }];
        let frame = crate::SessionTransactionFrame {
            request_id: id(1),
            expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
            edits: &edits,
        };
        let codec = ProtocolCodec::default();
        let mut bytes = vec![
            0;
            codec
                .encoded_session_transaction_len(&frame)
                .expect("length")
        ];
        codec
            .encode_session_transaction(&frame, &mut bytes)
            .expect("encode");
        let response = controller
            .process_session_transaction_btlv(
                &codec,
                &bytes,
                &mut DecodeScratch::new(&mut [0_u16; 1]),
            )
            .expect("decode/process");
        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(response.revision, SessionRevision(8));
        assert_eq!(
            controller
                .session()
                .compiled()
                .normalized_model()
                .session_id
                .as_str(),
            "renamed"
        );
    }

    #[test]
    fn rejected_engine_rates_have_identical_typed_and_btlv_rollback_diagnostics() {
        const DETAIL: &str =
            "launch sample_rate_hz must be one of 44100, 48000, 88200, or 96000 Hz";

        fn assert_rejection(
            controller: &mut ProtocolController<MockProvider>,
            response: &ControllerResponse,
            before_revision: SessionRevision,
            before_snapshot: &str,
            before_model: &session::SessionModel,
            before_events: QueueReport,
        ) {
            assert_eq!(response.status, StatusCode::ValidationFailed);
            assert_eq!(response.revision, before_revision);
            let decoded = ProtocolCodec::default()
                .decode_non_ok_payload(response.payload(), 2)
                .expect("typed launch-rate diagnostic");
            assert_eq!(decoded.omitted_diagnostics, 0);
            assert_eq!(decoded.diagnostics.len(), 1);
            let diagnostic = &decoded.diagnostics[0];
            assert_eq!(diagnostic.code, "sample_rate.unsupported_at_launch");
            assert_eq!(
                diagnostic.path,
                [crate::PathSegment::Field("sample_rate_hz".to_owned())]
            );
            assert_eq!(diagnostic.detail.as_deref(), Some(DETAIL));
            assert_eq!(diagnostic.operation_index, Some(1));
            assert_eq!(controller.session().revision(), before_revision);
            assert_eq!(controller.session().canonical_snapshot(), before_snapshot);
            assert_eq!(
                controller.session().compiled().normalized_model(),
                before_model
            );
            assert_eq!(
                controller
                    .queues_mut()
                    .report(crate::QueueKind::ReliableEvent),
                before_events
            );
        }

        for rate in [176_400, 192_000, 352_800, 384_000, 0, 32_000, 192_001] {
            let edits = [SessionEdit::SetSampleRateHz {
                sample_rate_hz: rate,
            }];

            let mut typed = controller(4, 1);
            let before_revision = typed.session().revision();
            let before_snapshot = typed.session().canonical_snapshot().to_owned();
            let before_model = typed.session().compiled().normalized_model().clone();
            let before_events = typed.queues_mut().report(crate::QueueKind::ReliableEvent);
            let response = typed.process(ControllerRequest {
                request_id: id(1),
                expected_revision: ExpectedRevision::Exact(before_revision),
                canonical_bytes: b"typed-rejected-engine-rate",
                command: ControlCommand::SessionTransactionApply { edits: &edits },
            });
            assert_rejection(
                &mut typed,
                &response,
                before_revision,
                &before_snapshot,
                &before_model,
                before_events,
            );

            let mut btlv = controller(4, 1);
            let before_revision = btlv.session().revision();
            let before_snapshot = btlv.session().canonical_snapshot().to_owned();
            let before_model = btlv.session().compiled().normalized_model().clone();
            let before_events = btlv.queues_mut().report(crate::QueueKind::ReliableEvent);
            let frame = crate::SessionTransactionFrame {
                request_id: id(1),
                expected_revision: ExpectedRevision::Exact(before_revision),
                edits: &edits,
            };
            let codec = ProtocolCodec::default();
            let mut bytes = vec![
                0;
                codec
                    .encoded_session_transaction_len(&frame)
                    .expect("transaction length")
            ];
            codec
                .encode_session_transaction(&frame, &mut bytes)
                .expect("transaction encode");
            let response = btlv
                .process_b1b_btlv(&bytes, &mut DecodeScratch::new(&mut [0_u16; 1]))
                .expect("BTLV transaction response");
            assert_rejection(
                &mut btlv,
                &response,
                before_revision,
                &before_snapshot,
                &before_model,
                before_events,
            );
        }
    }

    #[test]
    fn decoded_track_edit_reaches_same_atomic_session_store() {
        let mut controller = controller(4, 1);
        let mut fader = controller.session().compiled().normalized_model().tracks[0]
            .fader
            .clone();
        fader.left_db = -2.0;
        let edits = [SessionEdit::SetTrackFader {
            track_id: session::StableId::parse("vocal").expect("ID"),
            fader,
        }];
        let frame = crate::SessionTransactionFrame {
            request_id: id(1),
            expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
            edits: &edits,
        };
        let codec = ProtocolCodec::default();
        let mut bytes = vec![
            0;
            codec
                .encoded_session_transaction_len(&frame)
                .expect("length")
        ];
        codec
            .encode_session_transaction(&frame, &mut bytes)
            .expect("encode");
        let response = controller
            .process_session_transaction_btlv(
                &codec,
                &bytes,
                &mut DecodeScratch::new(&mut [0_u16; 1]),
            )
            .expect("decode/process");
        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(
            controller.session().compiled().normalized_model().tracks[0]
                .fader
                .left_db,
            -2.0
        );
    }

    #[test]
    fn decoded_route_edit_reaches_same_atomic_session_store() {
        let mut controller = controller(4, 1);
        let edits = [SessionEdit::SetRouteGainDb {
            route_id: session::StableId::parse("to-main").expect("ID"),
            gain_db: -1.0,
        }];
        let frame = crate::SessionTransactionFrame {
            request_id: id(1),
            expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
            edits: &edits,
        };
        let codec = ProtocolCodec::default();
        let mut bytes = vec![
            0;
            codec
                .encoded_session_transaction_len(&frame)
                .expect("length")
        ];
        codec
            .encode_session_transaction(&frame, &mut bytes)
            .expect("encode");
        let response = controller
            .process_session_transaction_btlv(
                &codec,
                &bytes,
                &mut DecodeScratch::new(&mut [0_u16; 1]),
            )
            .expect("decode/process");
        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(
            controller.session().compiled().normalized_model().routes[0].gain_db,
            -1.0
        );
    }

    #[test]
    fn decoded_invalid_automation_rolls_back_after_final_compilation() {
        let mut controller = controller(4, 1);
        let before = controller.session().canonical_snapshot().to_owned();
        let edits = [SessionEdit::SetAutomationSegments {
            automation_id: session::StableId::parse("eq-gain").expect("ID"),
            segments: vec![session::AutomationSegment {
                shape: session::AutomationShape::Exponential,
                start_sample: 12,
                end_sample: 12,
                start_value: 0.0,
                end_value: 0.0,
                unit: session::ParameterUnit::Db,
            }],
        }];
        let frame = crate::SessionTransactionFrame {
            request_id: id(1),
            expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
            edits: &edits,
        };
        let codec = ProtocolCodec::default();
        let mut bytes = vec![
            0;
            codec
                .encoded_session_transaction_len(&frame)
                .expect("length")
        ];
        codec
            .encode_session_transaction(&frame, &mut bytes)
            .expect("encode");
        let response = controller
            .process_session_transaction_btlv(
                &codec,
                &bytes,
                &mut DecodeScratch::new(&mut [0_u16; 1]),
            )
            .expect("decode/process");
        assert_eq!(response.status, StatusCode::ValidationFailed);
        assert_eq!(controller.session().revision(), SessionRevision(7));
        assert_eq!(controller.session().canonical_snapshot(), before);
    }

    #[test]
    fn endpoint_owned_current_sample_rejects_past_automation_without_client_time() {
        let mut controller = controller_at_sample(4, 1, SampleTime(5));
        let request = ControllerRequest {
            request_id: id(1),
            expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
            canonical_bytes: b"endpoint-current-sample",
            command: ControlCommand::AutomationEnqueue { batch: batch(1, 4) },
        };
        assert_eq!(controller.process(request).status, StatusCode::TimeInPast);
        assert!(controller.queues_mut().try_dequeue_automation().is_err());
    }

    #[test]
    fn b2b_btlv_automation_uses_header_identity_typed_domains_and_backpressure() {
        fn frame(request_id: u64, sample: u64, value: f32) -> Vec<u8> {
            let codec = ProtocolCodec::default();
            let records = [crate::AutomationRecord {
                kind: crate::AutomationKind::Point,
                handle: crate::ParameterHandle(1),
                start: SampleTime(sample),
                end: SampleTime(sample),
                start_value: value,
                end_value: value,
            }];
            let payload_len = codec
                .encoded_automation_enqueue_len(crate::AutomationEnqueue { records: &records })
                .expect("payload length");
            let mut result = vec![0_u8; crate::OUTER_HEADER_BYTES + payload_len];
            codec
                .write_outer_header(
                    &mut result,
                    crate::FrameKind::Command,
                    MessageId::AutomationEnqueue,
                    StatusCode::Ok,
                    request_id,
                    7,
                    0,
                    payload_len as u32,
                    3,
                )
                .expect("header");
            codec
                .encode_automation_enqueue(
                    crate::AutomationEnqueue { records: &records },
                    &mut result[crate::OUTER_HEADER_BYTES..],
                )
                .expect("payload");
            result
        }

        let mut controller = controller(8, 1);
        let first = frame(1, 0, 1.0);
        let response = controller
            .process_b1b_btlv(&first, &mut DecodeScratch::new(&mut [0_u16; 3]))
            .expect("BTLV admission");
        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(
            ProtocolCodec::default()
                .decode_automation_enqueued(response.payload(), 4)
                .expect("success payload"),
            AutomationEnqueued {
                accepted_records: 1,
                occupancy: 1,
                capacity: 1,
                generation: 2,
            }
        );
        let full = frame(2, 1, 1.0);
        let response = controller
            .process_b1b_btlv(&full, &mut DecodeScratch::new(&mut [0_u16; 3]))
            .expect("BTLV backpressure");
        assert_eq!(response.status, StatusCode::Backpressure);
        assert_eq!(
            ProtocolCodec::default()
                .decode_non_ok_payload(response.payload(), 2)
                .expect("typed backpressure")
                .backpressure
                .expect("backpressure"),
            crate::Backpressure {
                queue_kind: crate::BackpressureQueueKind::Automation,
                capacity: 1,
                occupancy: 1,
                requested_items: 1,
                generation: Some(2),
                retry_boundary: None,
                requested_bytes: None,
                available_bytes: None,
            }
        );
        let _ = controller
            .queues_mut()
            .try_dequeue_automation()
            .expect("first batch");
        let invalid_domain = frame(3, 2, 2.0);
        assert_eq!(
            controller
                .process_b1b_btlv(&invalid_domain, &mut DecodeScratch::new(&mut [0_u16; 3]))
                .expect("domain response")
                .status,
            StatusCode::InvalidField
        );
        let mut past = controller_at_sample(8, 1, SampleTime(5));
        assert_eq!(
            past.process_b1b_btlv(&frame(1, 4, 1.0), &mut DecodeScratch::new(&mut [0_u16; 3]))
                .expect("past response")
                .status,
            StatusCode::TimeInPast
        );
    }

    #[test]
    fn b3a_transport_btlv_is_typed_idempotent_and_emits_reliable_state() {
        fn set_frame(
            request_id: u64,
            revision: ExpectedRevision,
            request: TransportSetRequest,
        ) -> Vec<u8> {
            let codec = ProtocolCodec::default();
            let payload_len = codec.encoded_transport_set_request_len(request);
            let mut frame = vec![0_u8; crate::OUTER_HEADER_BYTES + payload_len];
            let (wire_revision, flags) = match revision {
                ExpectedRevision::Exact(value) => (value.0, 0),
                ExpectedRevision::Any => (0, 1),
            };
            codec
                .write_outer_header(
                    &mut frame,
                    crate::FrameKind::Command,
                    MessageId::TransportSet,
                    StatusCode::Ok,
                    request_id,
                    wire_revision,
                    flags,
                    payload_len as u32,
                    if request.position.is_some() { 2 } else { 1 },
                )
                .expect("header");
            codec
                .encode_transport_set_request(request, &mut frame[crate::OUTER_HEADER_BYTES..])
                .expect("payload");
            frame
        }
        fn hex(bytes: &[u8]) -> String {
            let mut result = String::with_capacity(bytes.len() * 2);
            for byte in bytes {
                use core::fmt::Write as _;
                write!(result, "{byte:02x}").expect("hex");
            }
            result
        }

        let codec = ProtocolCodec::default();
        let get = crate::Frame::Command(crate::CommandFrame {
            request_id: id(1),
            expected_revision: ExpectedRevision::Any,
            message_id: MessageId::TransportGet,
        });
        let mut get_frame = [0_u8; crate::OUTER_HEADER_BYTES];
        codec.encode(&get, &mut get_frame).expect("get frame");
        assert_eq!(
            hex(&get_frame),
            "4d49534f43544c0001000000300001010700000000000000010000000000000000000000000000000000000000000000"
        );
        let set = TransportSetRequest {
            state: TransportState::Playing,
            position: Some(SampleTime(9)),
        };
        let set_bytes = set_frame(2, ExpectedRevision::Exact(SessionRevision(7)), set);
        let mut controller = controller(8, 1);
        let get_response = controller
            .process_b1b_btlv(&get_frame, &mut DecodeScratch::new(&mut [0_u16; 0]))
            .expect("typed get");
        assert_eq!(get_response.status, StatusCode::Ok);
        assert_eq!(
            codec
                .decode_transport_snapshot(get_response.payload(), 3)
                .expect("get snapshot"),
            TransportSnapshot {
                state: TransportState::Stopped,
                position: SampleTime(0),
                effective_sample: SampleTime(0),
            }
        );
        let set_response = controller
            .process_b1b_btlv(&set_bytes, &mut DecodeScratch::new(&mut [0_u16; 2]))
            .expect("typed set");
        assert_eq!(set_response.status, StatusCode::Ok);
        assert_eq!(
            codec
                .decode_transport_snapshot(set_response.payload(), 3)
                .expect("set snapshot"),
            TransportSnapshot {
                state: TransportState::Playing,
                position: SampleTime(9),
                effective_sample: SampleTime(0),
            }
        );
        let event = controller
            .queues_mut()
            .try_dequeue_event()
            .expect("transport event");
        let mut event_payload = [0_u8; 80];
        assert_eq!(
            controller.encode_transport_state_event(event, &mut event_payload),
            Ok(80)
        );
        assert_eq!(
            codec.decode_transport_state_event(&event_payload, 5),
            Ok(TransportStateEvent {
                event_sequence: 1,
                state: TransportState::Playing,
                position: SampleTime(9),
                effective_sample: SampleTime(0),
                origin_request_id: Some(id(2)),
            })
        );
        let mut event_frame = [0_u8; crate::OUTER_HEADER_BYTES + 80];
        codec
            .write_outer_header(
                &mut event_frame,
                crate::FrameKind::Event,
                MessageId::TransportState,
                StatusCode::Ok,
                0,
                7,
                0,
                80,
                5,
            )
            .expect("event header");
        event_frame[crate::OUTER_HEADER_BYTES..].copy_from_slice(&event_payload);
        assert_eq!(
            hex(&event_frame),
            concat!(
                "4d49534f43544c0001000000300003001080000050000000",
                "000000000000000007000000000000000500000000000000",
                "01000401080000000100000000000000",
                "02000101010000000200000000000000",
                "03000401080000000900000000000000",
                "04000401080000000000000000000000",
                "05000400080000000200000000000000"
            )
        );
        let retain = TransportSetRequest {
            state: TransportState::Stopped,
            position: None,
        };
        let retained = controller
            .process_b1b_btlv(
                &set_frame(3, ExpectedRevision::Exact(SessionRevision(7)), retain),
                &mut DecodeScratch::new(&mut [0_u16; 1]),
            )
            .expect("retain position");
        assert_eq!(
            codec.decode_transport_snapshot(retained.payload(), 3),
            Ok(TransportSnapshot {
                state: TransportState::Stopped,
                position: SampleTime(9),
                effective_sample: SampleTime(0),
            })
        );
        let blocked = controller
            .process_b1b_btlv(
                &set_frame(4, ExpectedRevision::Exact(SessionRevision(7)), set),
                &mut DecodeScratch::new(&mut [0_u16; 2]),
            )
            .expect("event capacity response");
        assert_eq!(blocked.status, StatusCode::Backpressure);
        assert_eq!(
            controller.provider().transport_state,
            TransportState::Stopped,
            "a full reliable-event queue must leave the typed provider unchanged"
        );
        assert_eq!(controller.provider().transport_position, SampleTime(9));
        let invalid_revision = set_frame(5, ExpectedRevision::Any, set);
        assert_eq!(
            controller
                .process_b1b_btlv(&invalid_revision, &mut DecodeScratch::new(&mut [0_u16; 2]))
                .expect("revision response")
                .status,
            StatusCode::InvalidField
        );
    }

    #[test]
    fn transport_state_only_preserves_automation_and_locate_starts_new_ordering_epoch() {
        let mut controller = egress_controller(3, 1);
        controller
            .queues_mut()
            .try_enqueue_automation(SampleTime(0), batch(9, 100))
            .expect("queued automation");
        let state_only = controller.process(ControllerRequest {
            request_id: id(1),
            expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
            canonical_bytes: b"state-only-preserves-automation",
            command: ControlCommand::TransportSet {
                request: TransportSetRequest {
                    state: TransportState::Playing,
                    position: None,
                },
            },
        });
        assert_eq!(state_only.status, StatusCode::Ok);
        assert_eq!(
            controller
                .queues_mut()
                .report(crate::QueueKind::Automation)
                .occupancy,
            1
        );
        let _ = controller
            .queues_mut()
            .try_dequeue_event()
            .expect("state event");

        let locate = controller.process(ControllerRequest {
            request_id: id(2),
            expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
            canonical_bytes: b"locate-cancels-automation",
            command: ControlCommand::TransportSet {
                request: TransportSetRequest {
                    state: TransportState::Playing,
                    position: Some(SampleTime(10)),
                },
            },
        });
        assert_eq!(locate.status, StatusCode::Ok);
        assert!(controller.queues_mut().try_dequeue_automation().is_err());
        controller
            .queues_mut()
            .try_enqueue_automation(SampleTime(0), batch(10, 50))
            .expect("locate reset permits an earlier absolute scheduling epoch");
    }

    #[test]
    fn b3b1_btlv_telemetry_echo_and_counters_are_typed_and_nondestructive() {
        fn frame(
            message_id: MessageId,
            request_id: u64,
            revision: ExpectedRevision,
            payload: &[u8],
            count: u32,
        ) -> Vec<u8> {
            let codec = ProtocolCodec::default();
            let mut bytes = vec![0_u8; crate::OUTER_HEADER_BYTES + payload.len()];
            let (wire_revision, flags) = match revision {
                ExpectedRevision::Any => (0, 1),
                ExpectedRevision::Exact(revision) => (revision.0, 0),
            };
            codec
                .write_outer_header(
                    &mut bytes,
                    crate::FrameKind::Command,
                    message_id,
                    StatusCode::Ok,
                    request_id,
                    wire_revision,
                    flags,
                    payload.len() as u32,
                    count,
                )
                .expect("header");
            bytes[crate::OUTER_HEADER_BYTES..].copy_from_slice(payload);
            bytes
        }
        let codec = ProtocolCodec::default();
        let configuration = TelemetryConfiguration {
            meter_handles: vec![1],
            meter_period_blocks: 4,
            counter_ids: vec![crate::CounterId::ControlCommandBackpressure],
            counter_period_blocks: 8,
            diagnostics_enabled: false,
            minimum_diagnostic_severity: crate::DiagnosticSeverity::Warning,
        };
        let mut config_payload = vec![
            0;
            codec
                .encoded_telemetry_configuration_len(&configuration)
                .expect("config length")
        ];
        codec
            .encode_telemetry_configuration(&configuration, &mut config_payload)
            .expect("config");
        let mut controller = controller(8, 1);
        let first = controller
            .process_b1b_btlv(
                &frame(
                    MessageId::TelemetryConfigure,
                    1,
                    ExpectedRevision::Exact(SessionRevision(7)),
                    &config_payload,
                    6,
                ),
                &mut DecodeScratch::new(&mut [0_u16; 6]),
            )
            .expect("configure");
        assert_eq!(first.status, StatusCode::Ok);
        assert_eq!(
            codec.decode_telemetry_configuration(first.payload(), 6),
            Ok(configuration.clone())
        );
        assert_eq!(controller.provider().telemetry_configuration, configuration);
        let second = controller
            .process_b1b_btlv(
                &frame(
                    MessageId::TelemetryConfigure,
                    2,
                    ExpectedRevision::Exact(SessionRevision(7)),
                    &config_payload,
                    6,
                ),
                &mut DecodeScratch::new(&mut [0_u16; 6]),
            )
            .expect("idempotent configure");
        assert_eq!(second.payload(), first.payload());

        let request = CountersRequest {
            all: false,
            ids: vec![1, 5],
        };
        let mut request_payload = [0_u8; 32];
        codec
            .encode_counters_request(&request, &mut request_payload)
            .expect("counter request");
        let first = controller
            .process_b1b_btlv(
                &frame(
                    MessageId::CountersGet,
                    3,
                    ExpectedRevision::Any,
                    &request_payload,
                    2,
                ),
                &mut DecodeScratch::new(&mut [0_u16; 2]),
            )
            .expect("counter read");
        let second = controller
            .process_b1b_btlv(
                &frame(
                    MessageId::CountersGet,
                    4,
                    ExpectedRevision::Any,
                    &request_payload,
                    2,
                ),
                &mut DecodeScratch::new(&mut [0_u16; 2]),
            )
            .expect("nonreset read");
        assert_eq!(first.payload(), second.payload());
        assert_eq!(
            codec
                .decode_counter_snapshot(first.payload(), 3)
                .expect("snapshot")
                .values
                .len(),
            2
        );
        let unknown = CountersRequest {
            all: false,
            ids: vec![99],
        };
        let mut unknown_payload = [0_u8; 32];
        codec
            .encode_counters_request(&unknown, &mut unknown_payload)
            .expect("unknown selector");
        assert_eq!(
            controller
                .process_b1b_btlv(
                    &frame(
                        MessageId::CountersGet,
                        5,
                        ExpectedRevision::Any,
                        &unknown_payload,
                        2
                    ),
                    &mut DecodeScratch::new(&mut [0_u16; 2]),
                )
                .expect("unknown response")
                .status,
            StatusCode::NotFound
        );
    }

    #[test]
    fn b3b2_diagnostics_btlv_pages_are_typed_nondestructive_and_expire_old_cursors() {
        fn frame(request_id: u64, request: DiagnosticsRequest) -> Vec<u8> {
            let codec = ProtocolCodec::default();
            let mut bytes = vec![0_u8; crate::OUTER_HEADER_BYTES + 48];
            codec
                .write_outer_header(
                    &mut bytes,
                    crate::FrameKind::Command,
                    MessageId::DiagnosticsGet,
                    StatusCode::Ok,
                    request_id,
                    0,
                    1,
                    48,
                    3,
                )
                .expect("header");
            codec
                .encode_diagnostics_request(request, &mut bytes[crate::OUTER_HEADER_BYTES..])
                .expect("payload");
            bytes
        }

        let codec = ProtocolCodec::default();
        let mut controller = controller(8, 1);
        let first_request = DiagnosticsRequest {
            after_sequence: 0,
            limit: 1,
            minimum_severity: crate::DiagnosticSeverity::Info,
        };
        let first = controller
            .process_b1b_btlv(
                &frame(1, first_request),
                &mut DecodeScratch::new(&mut [0_u16; 3]),
            )
            .expect("first page");
        assert_eq!(first.status, StatusCode::Ok);
        assert_eq!(
            codec.decode_diagnostics_page(first.payload(), 3),
            Ok(DiagnosticsPage {
                last_sequence: 3,
                eof: false,
                diagnostics: vec![retained_diagnostic(3, crate::DiagnosticSeverity::Warning)],
            })
        );
        let repeat = controller
            .process_b1b_btlv(
                &frame(2, first_request),
                &mut DecodeScratch::new(&mut [0_u16; 3]),
            )
            .expect("nondestructive first page");
        assert_eq!(repeat.status, StatusCode::Ok);
        assert_eq!(
            repeat.payload(),
            first.payload(),
            "reads must not drain retained history"
        );

        let final_page = controller
            .process_b1b_btlv(
                &frame(
                    3,
                    DiagnosticsRequest {
                        after_sequence: 3,
                        limit: 1,
                        minimum_severity: crate::DiagnosticSeverity::Info,
                    },
                ),
                &mut DecodeScratch::new(&mut [0_u16; 3]),
            )
            .expect("final page");
        assert_eq!(
            codec.decode_diagnostics_page(final_page.payload(), 3),
            Ok(DiagnosticsPage {
                last_sequence: 4,
                eof: true,
                diagnostics: vec![retained_diagnostic(4, crate::DiagnosticSeverity::Error)],
            })
        );
        let filtered = controller
            .process_b1b_btlv(
                &frame(
                    4,
                    DiagnosticsRequest {
                        after_sequence: 0,
                        limit: 1,
                        minimum_severity: crate::DiagnosticSeverity::Error,
                    },
                ),
                &mut DecodeScratch::new(&mut [0_u16; 3]),
            )
            .expect("severity filtered page");
        assert_eq!(filtered.status, StatusCode::Ok);
        assert_eq!(
            codec.decode_diagnostics_page(filtered.payload(), 3),
            Ok(DiagnosticsPage {
                last_sequence: 4,
                eof: true,
                diagnostics: vec![retained_diagnostic(4, crate::DiagnosticSeverity::Error)],
            })
        );

        let expired = controller
            .process_b1b_btlv(
                &frame(
                    5,
                    DiagnosticsRequest {
                        after_sequence: 1,
                        limit: 1,
                        minimum_severity: crate::DiagnosticSeverity::Info,
                    },
                ),
                &mut DecodeScratch::new(&mut [0_u16; 3]),
            )
            .expect("expired cursor response");
        assert_eq!(expired.status, StatusCode::ReplayExpired);
        assert_eq!(
            codec
                .decode_non_ok_payload(expired.payload(), 2)
                .expect("typed expiration diagnostic")
                .diagnostics,
            vec![Diagnostic {
                code: "diagnostics.cursor_expired".to_owned(),
                severity: crate::DiagnosticSeverity::Error,
                path: Vec::new(),
                detail: None,
                operation_index: None,
                sample_time: None,
                provider_sequence: None,
            }]
        );

        let empty = controller
            .process_b1b_btlv(
                &frame(
                    6,
                    DiagnosticsRequest {
                        after_sequence: 4,
                        limit: 1,
                        minimum_severity: crate::DiagnosticSeverity::Info,
                    },
                ),
                &mut DecodeScratch::new(&mut [0_u16; 3]),
            )
            .expect("empty page");
        assert_eq!(
            codec.decode_diagnostics_page(empty.payload(), 2),
            Ok(DiagnosticsPage {
                last_sequence: 4,
                eof: true,
                diagnostics: Vec::new(),
            }),
            "an empty page preserves the input cursor"
        );
    }

    #[test]
    fn provider_feature_matrix_derives_capabilities_and_refuses_disabled_dispatch() {
        let codec = ProtocolCodec::default();
        for (features, required_command, required_event, required_bit) in [
            (
                ProviderFeatures {
                    parameters: true,
                    ..ProviderFeatures::NONE
                },
                4,
                0,
                7,
            ),
            (
                ProviderFeatures {
                    transport: true,
                    transport_events: true,
                    ..ProviderFeatures::NONE
                },
                7,
                0x8010,
                8,
            ),
            (
                ProviderFeatures {
                    meters: true,
                    ..ProviderFeatures::NONE
                },
                0,
                0x8020,
                9,
            ),
            (
                ProviderFeatures {
                    counters: true,
                    ..ProviderFeatures::NONE
                },
                10,
                0x8021,
                10,
            ),
            (
                ProviderFeatures {
                    diagnostics: true,
                    ..ProviderFeatures::NONE
                },
                11,
                0x8030,
                11,
            ),
            (
                ProviderFeatures {
                    session_events: true,
                    ..ProviderFeatures::NONE
                },
                0,
                0x8001,
                12,
            ),
        ] {
            let mut endpoint = controller(8, 1);
            endpoint.set_provider_features(features);
            let response = endpoint.process(capability(1, b"features"));
            let decoded = codec
                .decode_capabilities(response.payload(), 27)
                .expect("capabilities");
            if required_command != 0 {
                assert!(
                    decoded
                        .supported_commands
                        .chunks_exact(2)
                        .any(|id| u16::from_le_bytes([id[0], id[1]]) == required_command)
                );
            }
            if required_event != 0 {
                assert!(
                    decoded
                        .supported_events
                        .chunks_exact(2)
                        .any(|id| u16::from_le_bytes([id[0], id[1]]) == required_event)
                );
            }
            assert_ne!(decoded.flags.0 & (1 << required_bit), 0);
        }
        let mut endpoint = controller(8, 1);
        endpoint.set_provider_features(ProviderFeatures::NONE);
        let caps = endpoint.process(capability(1, b"none"));
        let decoded = codec
            .decode_capabilities(caps.payload(), 27)
            .expect("none caps");
        assert_eq!(decoded.flags.0 & !((1 << 7) - 1), 0);
        assert_eq!(
            endpoint
                .process(ControllerRequest {
                    request_id: id(2),
                    expected_revision: ExpectedRevision::Any,
                    canonical_bytes: b"disabled-transport",
                    command: ControlCommand::TransportGet
                })
                .status,
            StatusCode::Unavailable
        );
        let edits = [SessionEdit::SetSessionId {
            session_id: session::StableId::parse("disabled-events").expect("id"),
        }];
        assert_eq!(
            endpoint
                .process(ControllerRequest {
                    request_id: id(3),
                    expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
                    canonical_bytes: b"disabled-session-event",
                    command: ControlCommand::SessionTransactionApply { edits: &edits }
                })
                .status,
            StatusCode::Unavailable
        );
        assert!(endpoint.queues_mut().try_dequeue_event().is_err());

        let mut endpoint = controller(8, 1);
        endpoint.config.maximum_transaction_edits = 0;
        let caps = endpoint.process(capability(1, b"zero-edit-limit"));
        let decoded = codec
            .decode_capabilities(caps.payload(), 27)
            .expect("zero-edit capabilities");
        assert_eq!(decoded.maximum_transaction_edits, 0);
        assert!(
            !decoded
                .supported_commands
                .chunks_exact(2)
                .any(|value| u16::from_le_bytes([value[0], value[1]]) == 3)
        );
        assert!(
            !decoded.supported_events.chunks_exact(2).any(|value| {
                matches!(u16::from_le_bytes([value[0], value[1]]), 0x8001 | 0x8002)
            })
        );
        assert_eq!(decoded.flags.0 & ((1 << 3) | (1 << 12)), 0);
        assert_eq!(
            endpoint
                .process(ControllerRequest {
                    request_id: id(2),
                    expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
                    canonical_bytes: b"zero-edit-disabled",
                    command: ControlCommand::SessionTransactionApply { edits: &edits },
                })
                .status,
            StatusCode::Unavailable
        );
    }

    #[test]
    fn session_commit_reserves_reliable_event_before_replacing_store() {
        let mut controller = controller(4, 1);
        let before_snapshot = controller.session().canonical_snapshot().to_owned();
        controller
            .queues_mut()
            .try_enqueue_event(ReliableSlot::session_committed(
                SessionRevision(7),
                99,
                id(99),
                SessionRevision(6),
                1,
            ))
            .expect("fill prepared event queue");
        let edits = [SessionEdit::SetSessionId {
            session_id: session::StableId::parse("blocked-commit").expect("ID"),
        }];
        let response = controller.process(ControllerRequest {
            request_id: id(1),
            expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
            canonical_bytes: b"event-capacity-before-commit",
            command: ControlCommand::SessionTransactionApply { edits: &edits },
        });
        assert_eq!(response.status, StatusCode::Backpressure);
        assert_eq!(controller.session().revision(), SessionRevision(7));
        assert_eq!(controller.session().canonical_snapshot(), before_snapshot);
    }

    #[test]
    fn committed_transaction_emits_typed_zero_header_id_event() {
        let mut controller = controller(4, 1);
        let edits = [SessionEdit::SetSessionId {
            session_id: session::StableId::parse("evented-commit").expect("ID"),
        }];
        let response = controller.process(ControllerRequest {
            request_id: id(1),
            expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
            canonical_bytes: b"emit-session-committed",
            command: ControlCommand::SessionTransactionApply { edits: &edits },
        });
        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(response.revision, SessionRevision(8));
        assert_eq!(
            controller
                .queues_mut()
                .try_dequeue_event()
                .expect("reliable session event"),
            ReliableSlot::session_committed(SessionRevision(8), 1, id(1), SessionRevision(7), 1,)
        );
    }

    #[test]
    fn snapshot_pagination_has_exact_revision_conflict_and_eof_boundaries() {
        let mut controller = controller(4, 1);
        let codec = ProtocolCodec::default();
        let request = |request_id, revision, offset, maximum_bytes| {
            let snapshot = crate::SessionSnapshotRequest {
                offset,
                maximum_bytes,
            };
            let mut bytes = vec![0_u8; crate::OUTER_HEADER_BYTES + 32];
            codec
                .encode(
                    &crate::Frame::Command(crate::CommandFrame {
                        request_id: id(request_id),
                        expected_revision: revision,
                        message_id: MessageId::SessionSnapshotGet,
                    }),
                    &mut bytes[..crate::OUTER_HEADER_BYTES],
                )
                .expect("header");
            bytes[20..24].copy_from_slice(&32_u32.to_le_bytes());
            bytes[40..44].copy_from_slice(&2_u32.to_le_bytes());
            codec
                .encode_snapshot_request(snapshot, &mut bytes[crate::OUTER_HEADER_BYTES..])
                .expect("payload");
            bytes
        };
        let first = request(1, ExpectedRevision::Any, 0, 1);
        let first_response = controller
            .process_b1b_btlv(&first, &mut DecodeScratch::new(&mut [0_u16; 2]))
            .expect("first page");
        assert_eq!(first_response.status, StatusCode::Ok);
        let first_page = codec
            .decode_snapshot(first_response.payload(), 4)
            .expect("page");
        assert_eq!(first_page.offset, 0);
        assert!(!first_page.eof);
        let revision = first_response.revision;

        let final_page = request(
            2,
            ExpectedRevision::Exact(revision),
            controller.session().canonical_snapshot().len() as u64,
            1,
        );
        let final_response = controller
            .process_b1b_btlv(&final_page, &mut DecodeScratch::new(&mut [0_u16; 2]))
            .expect("final page");
        let final_snapshot = codec
            .decode_snapshot(final_response.payload(), 4)
            .expect("final payload");
        assert!(final_snapshot.eof);
        assert!(final_snapshot.canonical_json_chunk.is_empty());

        let conflict = request(
            3,
            ExpectedRevision::Exact(SessionRevision(revision.0 - 1)),
            1,
            1,
        );
        assert_eq!(
            controller
                .process_b1b_btlv(&conflict, &mut DecodeScratch::new(&mut [0_u16; 2]))
                .expect("conflict response")
                .status,
            StatusCode::RevisionConflict
        );
    }

    #[test]
    fn canonical_json_snapshots_reparse_before_and_after_commit_across_utf8_split_pages() {
        fn request(
            codec: &ProtocolCodec,
            request_id: u64,
            revision: ExpectedRevision,
            offset: u64,
            maximum_bytes: u32,
        ) -> Vec<u8> {
            let mut bytes = vec![0_u8; crate::OUTER_HEADER_BYTES + 32];
            codec
                .encode(
                    &crate::Frame::Command(crate::CommandFrame {
                        request_id: id(request_id),
                        expected_revision: revision,
                        message_id: MessageId::SessionSnapshotGet,
                    }),
                    &mut bytes[..crate::OUTER_HEADER_BYTES],
                )
                .expect("header");
            bytes[20..24].copy_from_slice(&32_u32.to_le_bytes());
            bytes[40..44].copy_from_slice(&2_u32.to_le_bytes());
            codec
                .encode_snapshot_request(
                    crate::SessionSnapshotRequest {
                        offset,
                        maximum_bytes,
                    },
                    &mut bytes[crate::OUTER_HEADER_BYTES..],
                )
                .expect("snapshot request");
            bytes
        }

        let mut controller = controller(4, 1);
        let codec = ProtocolCodec::default();
        let mut initial_bytes = Vec::new();
        let mut initial_offset = 0_u64;
        let mut request_id = 1_u64;
        loop {
            let encoded = request(
                &codec,
                request_id,
                ExpectedRevision::Exact(SessionRevision(7)),
                initial_offset,
                128,
            );
            let response = controller
                .process_b1b_btlv(&encoded, &mut DecodeScratch::new(&mut [0_u16; 2]))
                .expect("initial snapshot page");
            assert_eq!(response.status, StatusCode::Ok);
            let page = codec
                .decode_snapshot(response.payload(), 4)
                .expect("initial snapshot payload");
            initial_bytes.extend_from_slice(page.canonical_json_chunk);
            initial_offset += page.canonical_json_chunk.len() as u64;
            request_id += 1;
            if page.eof {
                break;
            }
        }
        let initial_text = core::str::from_utf8(&initial_bytes).expect("initial UTF-8");
        let initial_model = parse_session_json(initial_text).expect("initial snapshot reparses");
        assert_eq!(
            canonical_session_json(&initial_model).expect("initial canonical"),
            initial_text
        );

        let edits = [SessionEdit::SetEffectIdentity {
            track_id: session::StableId::parse("vocal").expect("track ID"),
            rack_name: session::RackName::Dynamic,
            effect_id: session::StableId::parse("eq").expect("effect ID"),
            identity: session::EffectIdentity::ThirdPartyCid {
                cid: "bafy-é-🙂".to_owned(),
            },
        }];
        let applied = controller.process(ControllerRequest {
            request_id: id(request_id),
            expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
            canonical_bytes: b"unicode-commit",
            command: ControlCommand::SessionTransactionApply { edits: &edits },
        });
        assert_eq!(applied.status, StatusCode::Ok);
        assert_eq!(applied.revision, SessionRevision(8));
        request_id += 1;

        let mut snapshot = Vec::new();
        let mut offset = 0_u64;
        let mut split_utf8 = false;
        loop {
            let encoded = request(
                &codec,
                request_id,
                ExpectedRevision::Exact(SessionRevision(8)),
                offset,
                1,
            );
            let response = controller
                .process_b1b_btlv(&encoded, &mut DecodeScratch::new(&mut [0_u16; 2]))
                .expect("one-byte snapshot page");
            assert_eq!(response.status, StatusCode::Ok);
            assert_eq!(response.revision, SessionRevision(8));
            let page = codec
                .decode_snapshot(response.payload(), 4)
                .expect("snapshot page");
            split_utf8 |= !page.canonical_json_chunk.is_empty()
                && core::str::from_utf8(page.canonical_json_chunk).is_err();
            snapshot.extend_from_slice(page.canonical_json_chunk);
            offset += page.canonical_json_chunk.len() as u64;
            request_id += 1;
            if page.eof {
                break;
            }
        }
        assert!(split_utf8, "one-byte paging must split the multibyte CID");
        let snapshot = String::from_utf8(snapshot).expect("reassembled snapshot UTF-8");
        assert_eq!(snapshot, controller.session().canonical_snapshot());
        let model = parse_session_json(&snapshot).expect("committed snapshot reparses");
        assert_eq!(
            canonical_session_json(&model).expect("committed canonical"),
            snapshot
        );
        assert_eq!(model.revision, 8);
    }

    #[test]
    fn c2b2_event_egress_emits_all_six_schema_closed_full_frames() {
        let codec = ProtocolCodec::default();
        let mut controller = egress_controller(4, 256);
        let revision = controller.session().revision();
        let meter_handles = (1_u32..=256).collect::<Vec<_>>();
        let counter_ids = vec![
            crate::CounterId::ControlCommandBackpressure,
            crate::CounterId::TelemetryCoalesced,
        ];
        configure_event_egress(&mut controller, 1, meter_handles, counter_ids, true);

        let committed = ReliableSlot::session_committed(revision, 1, id(9), SessionRevision(6), 2);
        let canceled = ReliableSlot::automation_canceled(
            revision,
            2,
            id(10),
            3,
            AutomationCancellationReason::ExplicitReconfiguration,
            2,
            Some(SampleTime(12)),
        );
        let transport = ReliableSlot::transport_state(
            revision,
            3,
            TransportState::Playing,
            SampleTime(8),
            SampleTime(12),
            Some(id(11)),
        );
        controller
            .queues_mut()
            .try_enqueue_event(committed)
            .expect("committed");
        controller
            .queues_mut()
            .try_enqueue_event(canceled)
            .expect("canceled");
        controller
            .queues_mut()
            .try_enqueue_event(transport)
            .expect("transport");
        let diagnostic = Diagnostic {
            code: "mock.egress".to_owned(),
            severity: crate::DiagnosticSeverity::Warning,
            path: Vec::new(),
            detail: Some("bounded storage".to_owned()),
            operation_index: None,
            sample_time: Some(12),
            provider_sequence: Some(1),
        };
        controller
            .enqueue_diagnostic_event(
                revision,
                DiagnosticEvent {
                    diagnostic: diagnostic.clone(),
                },
            )
            .expect("diagnostic");

        let meter_records = (1_u32..=256)
            .map(|handle| MeterRecord {
                handle,
                component: crate::MeterComponent::Left,
                flags: u16::try_from((handle % 7) + 1).expect("registered meter flags"),
                value: handle as f32,
            })
            .collect::<Vec<_>>();
        controller
            .stage_meter_batch_event(revision, SampleTime(24), &meter_records)
            .expect("meters");
        let counters = CounterSnapshot {
            observed_sample: SampleTime(24),
            values: vec![
                CounterValue {
                    id: crate::CounterId::ControlCommandBackpressure,
                    value: 7,
                },
                CounterValue {
                    id: crate::CounterId::TelemetryCoalesced,
                    value: 8,
                },
            ],
        };
        controller
            .stage_counter_snapshot_event(revision, &counters)
            .expect("counters");

        let expected = [
            TypedEventFrame {
                revision,
                payload: EventPayload::SessionCommitted(SessionCommitted {
                    event_sequence: 1,
                    origin_request_id: id(9),
                    previous_revision: SessionRevision(6),
                    applied_operations: 2,
                }),
            },
            TypedEventFrame {
                revision,
                payload: EventPayload::AutomationCanceled(AutomationCanceled {
                    event_sequence: 2,
                    origin_request_id: id(10),
                    canceled_records: 3,
                    reason: AutomationCancellationReason::ExplicitReconfiguration,
                    queue_generation: 2,
                    effective_sample: Some(SampleTime(12)),
                }),
            },
            TypedEventFrame {
                revision,
                payload: EventPayload::TransportState(TransportStateEvent {
                    event_sequence: 3,
                    state: TransportState::Playing,
                    position: SampleTime(8),
                    effective_sample: SampleTime(12),
                    origin_request_id: Some(id(11)),
                }),
            },
            TypedEventFrame {
                revision,
                payload: EventPayload::Diagnostic(&diagnostic),
            },
        ];
        for frame in &expected {
            let mut expected_bytes = vec![0_u8; 1024];
            let expected_len = codec
                .encode_event_frame_into(frame, &mut expected_bytes)
                .expect("expected reliable frame");
            let mut actual = vec![0_u8; expected_len];
            assert_eq!(
                controller.dequeue_reliable_event_frame_into(&mut actual),
                Ok(Some(expected_len))
            );
            assert_eq!(actual, expected_bytes[..expected_len]);
            assert!(actual[24..32].iter().all(|byte| *byte == 0));
            assert_eq!(
                codec
                    .decode_typed_event(&actual, &mut DecodeScratch::new(&mut [0_u16; 32]))
                    .expect("typed reliable event")
                    .header
                    .revision,
                revision
            );
        }

        let meter_frame = TypedEventFrame {
            revision,
            payload: EventPayload::MeterBatch(MeterBatch {
                observed_sample: SampleTime(24),
                records: &meter_records,
            }),
        };
        let counter_frame = TypedEventFrame {
            revision,
            payload: EventPayload::CounterSnapshot(CounterSnapshotRef {
                observed_sample: SampleTime(24),
                values: &counters.values,
            }),
        };
        for frame in [&meter_frame, &counter_frame] {
            let mut expected_bytes = vec![0_u8; 8192];
            let expected_len = codec
                .encode_event_frame_into(frame, &mut expected_bytes)
                .expect("expected lossy frame");
            let mut actual = vec![0_u8; expected_len];
            assert_eq!(
                controller.dequeue_lossy_event_frame_into(&mut actual),
                Ok(Some(expected_len))
            );
            assert_eq!(actual, expected_bytes[..expected_len]);
            assert!(actual[24..32].iter().all(|byte| *byte == 0));
        }
        assert_eq!(controller.dequeue_lossy_event_frame_into(&mut []), Ok(None));
    }

    #[test]
    fn counter_egress_splits_nonascending_staged_snapshots_without_stalling() {
        let codec = ProtocolCodec::default();
        let mut controller = egress_controller(1, 4);
        let revision = controller.session().revision();
        let high = crate::CounterId::TelemetryCoalesced;
        let low = crate::CounterId::ControlCommandBackpressure;
        configure_event_egress(&mut controller, 1, Vec::new(), vec![low, high], false);
        for (id, value) in [(high, 5_u64), (low, 1_u64)] {
            controller
                .stage_counter_snapshot_event(
                    revision,
                    &CounterSnapshot {
                        observed_sample: SampleTime(24),
                        values: vec![CounterValue { id, value }],
                    },
                )
                .expect("stage one valid counter snapshot");
        }
        for (id, value) in [(high, 5_u64), (low, 1_u64)] {
            let expected = TypedEventFrame {
                revision,
                payload: EventPayload::CounterSnapshot(CounterSnapshotRef {
                    observed_sample: SampleTime(24),
                    values: &[CounterValue { id, value }],
                }),
            };
            let mut expected_bytes = vec![0_u8; 256];
            let expected_len = codec
                .encode_event_frame_into(&expected, &mut expected_bytes)
                .expect("expected counter event");
            let mut actual = vec![0_u8; expected_len];
            assert_eq!(
                controller.dequeue_lossy_event_frame_into(&mut actual),
                Ok(Some(expected_len))
            );
            assert_eq!(actual, expected_bytes[..expected_len]);
        }
        assert_eq!(controller.dequeue_lossy_event_frame_into(&mut []), Ok(None));
    }

    #[test]
    fn c2b2_short_buffers_retain_pending_reliable_order_and_diagnostic_storage() {
        let mut controller = egress_controller(1, 1);
        let revision = controller.session().revision();
        let first = ReliableSlot::session_committed(revision, 1, id(1), SessionRevision(6), 1);
        controller
            .queues_mut()
            .try_enqueue_event(first)
            .expect("first reliable event");
        let mut short = [0xa5_u8; 1];
        assert!(matches!(
            controller.dequeue_reliable_event_frame_into(&mut short),
            Err(EventEgressError::Encode(EncodeError::OutputTooSmall { required })) if required > short.len()
        ));
        assert_eq!(short, [0xa5]);
        let second = ReliableSlot::transport_state(
            revision,
            2,
            TransportState::Stopped,
            SampleTime(0),
            SampleTime(0),
            None,
        );
        controller
            .queues_mut()
            .try_enqueue_event(second)
            .expect("queued after pending first");
        let mut output = [0_u8; 256];
        let first_len = controller
            .dequeue_reliable_event_frame_into(&mut output)
            .expect("first retry")
            .expect("first length");
        assert_eq!(
            ProtocolCodec::default()
                .decode_typed_event(
                    &output[..first_len],
                    &mut DecodeScratch::new(&mut [0_u16; 8])
                )
                .expect("first event")
                .header
                .message_id,
            MessageId::SessionCommitted
        );
        let second_len = controller
            .dequeue_reliable_event_frame_into(&mut output)
            .expect("second event")
            .expect("second length");
        assert_eq!(
            ProtocolCodec::default()
                .decode_typed_event(
                    &output[..second_len],
                    &mut DecodeScratch::new(&mut [0_u16; 8])
                )
                .expect("second event")
                .header
                .message_id,
            MessageId::TransportState
        );

        configure_event_egress(&mut controller, 2, Vec::new(), Vec::new(), true);
        let diagnostic = DiagnosticEvent {
            diagnostic: retained_diagnostic(99, crate::DiagnosticSeverity::Info),
        };
        controller
            .enqueue_diagnostic_event(revision, diagnostic.clone())
            .expect("diagnostic stored");
        assert!(matches!(
            controller.dequeue_reliable_event_frame_into(&mut short),
            Err(EventEgressError::Encode(EncodeError::OutputTooSmall { .. }))
        ));
        assert_eq!(
            controller.enqueue_diagnostic_event(revision, diagnostic.clone()),
            Err(EventEgressError::DiagnosticStorageFull)
        );
        let _ = controller
            .dequeue_reliable_event_frame_into(&mut output)
            .expect("diagnostic retry");
        controller
            .enqueue_diagnostic_event(revision, diagnostic)
            .expect("storage released only after full frame");

        let mut lossy = egress_controller(1, 1);
        let lossy_revision = lossy.session().revision();
        configure_event_egress(&mut lossy, 1, vec![1], Vec::new(), false);
        lossy
            .stage_meter_batch_event(
                lossy_revision,
                SampleTime(4),
                &[MeterRecord {
                    handle: 1,
                    component: crate::MeterComponent::Left,
                    flags: 3,
                    value: 0.5,
                }],
            )
            .expect("meter staging");
        assert!(matches!(
            lossy.dequeue_lossy_event_frame_into(&mut short),
            Err(EventEgressError::Encode(EncodeError::OutputTooSmall { .. }))
        ));
        assert_eq!(short, [0xa5]);
        let lossy_len = lossy
            .dequeue_lossy_event_frame_into(&mut output)
            .expect("lossy retry")
            .expect("lossy length");
        assert_eq!(
            ProtocolCodec::default()
                .decode_typed_event(
                    &output[..lossy_len],
                    &mut DecodeScratch::new(&mut [0_u16; 8])
                )
                .expect("meter event")
                .header
                .message_id,
            MessageId::MeterBatch
        );
    }

    #[test]
    fn c2b2_disabled_configuration_emits_nothing_and_reliable_full_returns_original() {
        let mut controller = egress_controller(1, 1);
        let revision = controller.session().revision();
        assert_eq!(
            controller.stage_meter_batch_event(
                revision,
                SampleTime(0),
                &[MeterRecord {
                    handle: 1,
                    component: crate::MeterComponent::Left,
                    flags: 1,
                    value: 0.0,
                }],
            ),
            Err(EventEgressError::Disabled)
        );
        assert_eq!(
            controller.enqueue_diagnostic_event(
                revision,
                DiagnosticEvent {
                    diagnostic: retained_diagnostic(1, crate::DiagnosticSeverity::Info),
                },
            ),
            Err(EventEgressError::Disabled)
        );
        assert_eq!(controller.dequeue_lossy_event_frame_into(&mut []), Ok(None));
        let event = ReliableSlot::session_committed(revision, 1, id(1), SessionRevision(6), 1);
        controller
            .queues_mut()
            .try_enqueue_event(event)
            .expect("first reliable");
        assert!(matches!(
            controller.queues_mut().try_enqueue_event(event),
            Err(crate::ReliableEnqueueError { value, .. }) if value == event
        ));
    }
}
