//! Bounded request replay and a control-plane-only protocol dispatcher.
//!
//! The dispatcher consumes typed commands directly or after bounded BTLV decoding. No decoder
//! calls a renderer, and this module has no plan-publication capability.

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
    Backpressure, BackpressureQueueKind, Capabilities, CapabilityFlags, CommandHeader,
    ControlListPage, ControlListRequest, ControlStatePage, CounterSnapshot, CounterSnapshotRef,
    CounterTelemetryRecord, CounterValue, CountersRequest, DecodeError, DecodeScratch,
    DecodedCommandPayload, Diagnostic, DiagnosticEvent, DiagnosticsPage, DiagnosticsRequest,
    EditPath, EncodeError, EventPayload, ExpectedRevision, MessageId, MeterBatch, MeterRecord,
    NonOkResponse, ObservationEnded, ObservationTapKey, PreparedSessionTransaction, ProtocolCodec,
    ProtocolQueues, QueueReport, ReliablePayload, ReliableSlot, RequestId, ResponseGetRequest,
    ResponseResult, SampleTime, SessionCommitted, SessionEdit, SessionRevision, SessionSnapshot,
    SessionStore, SessionStoreError, StatusCode, SuccessResponsePayload, TelemetryConfiguration,
    TelemetryKey, TelemetryRecord, TransactionApplied, TransportSetRequest, TransportSnapshot,
    TransportStateEvent, TypedEventFrame, TypedNonOkResponseFrame, TypedSuccessResponseFrame,
};
#[cfg(any(test, feature = "test-support"))]
use crate::{
    ControlBody, ControlEntry, ParameterChannel, ResponseCurves, ResponseTarget, TransportState,
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
    /// `CONTROL_LIST_GET` and `CONTROL_STATE_GET` (capability flag bit 7).
    pub controls: bool,
    /// `RESPONSE_GET` (capability flag bit 14).
    pub response: bool,
    pub transport: bool,
    pub meters: bool,
    pub counters: bool,
    pub diagnostics: bool,
    pub session_events: bool,
    pub transport_events: bool,
    /// `OBSERVATION_ENDED` (#1584): a host that keeps observation subscriptions tells its
    /// subscribers when one ends. It rides the session family, so it needs `session_events`.
    pub observation_events: bool,
}

impl ProviderFeatures {
    /// Enable every typed provider and event family.
    pub const ALL: Self = Self {
        controls: true,
        response: true,
        transport: true,
        meters: true,
        counters: true,
        diagnostics: true,
        session_events: true,
        transport_events: true,
        observation_events: true,
    };
    /// Disable every optional provider and event family.
    pub const NONE: Self = Self {
        controls: false,
        response: false,
        transport: false,
        meters: false,
        counters: false,
        diagnostics: false,
        session_events: false,
        transport_events: false,
        observation_events: false,
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
    /// Return the endpoint-owned current absolute sample.
    fn current_sample(&mut self) -> SampleTime;
    /// Return one bounded page of the control list after this cursor (decision 17, C4). A
    /// request whose `structure_epoch` is not `0` and not the provider's current structure is
    /// [`ParameterProviderError::Stale`].
    fn control_list(
        &mut self,
        request: &ControlListRequest,
    ) -> Result<ControlListPage, ParameterProviderError>;
    /// Return the values of the control rows of the same page, without the header rows, with the
    /// same staleness rule.
    fn control_state(
        &mut self,
        request: &ControlListRequest,
    ) -> Result<ControlStatePage, ParameterProviderError>;
    /// Compute the frequency response a `RESPONSE_GET` asks for, against the committed model. A
    /// read: it changes no session, revision, catalog or cell. A refusal carries its status and
    /// its diagnostic code ([`ResponseProviderError`]).
    fn response_get(
        &mut self,
        model: &session::SessionModel,
        request: &ResponseGetRequest,
    ) -> Result<ResponseResult, ResponseProviderError>;
    /// Return a typed nondestructive counter snapshot for the requested registered IDs.
    fn counters(
        &mut self,
        request: &CountersRequest,
    ) -> Result<CounterSnapshot, ParameterProviderError>;
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
    ///
    /// A provider refuses a configuration it can never serve with a typed error and changes
    /// nothing (#1351 D2): a counter ID it does not report is [`ParameterProviderError::NotFound`].
    fn telemetry_configure(
        &mut self,
        configuration: TelemetryConfiguration,
    ) -> Result<TelemetryConfiguration, ParameterProviderError>;
}

/// A provider's refusal of a `RESPONSE_GET`: the status and the diagnostic code the response
/// carries. The refusal changes nothing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResponseProviderError {
    /// The response status.
    pub status: StatusCode,
    /// The dotted diagnostic code, such as `response.points`.
    pub code: &'static str,
}

/// Typed bounded provider fixture failure; providers never return raw BTLV payload bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParameterProviderError {
    /// The requested revision-scoped control handle is absent.
    NotFound,
    /// The request names a structure that is no longer current (`control.list.stale`).
    Stale,
    /// The typed provider is temporarily unavailable.
    Unavailable,
    /// A bounded diagnostic cursor predates retained provider history.
    ReplayExpired,
    /// The provider's bounded catalog or page limit was exceeded.
    LimitExceeded,
}

/// Small deterministic provider suitable for protocol conformance fixtures.
#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Debug)]
pub struct MockProvider {
    /// Endpoint-owned current absolute sample.
    current_sample: SampleTime,
    /// The small fixed control list the fixture serves, in strictly ascending handle order.
    controls: Vec<ControlEntry>,
    /// The structure the fixture's handles belong to. A request for another is stale.
    structure_epoch: u64,
    /// The session revision the fixture's pages report.
    control_revision: u64,
    /// Typed nondestructive registered counter values.
    counter_snapshot: CounterSnapshot,
    /// Bounded retained typed diagnostic history.
    diagnostics: Vec<crate::Diagnostic>,
    /// Current absolute state.
    transport_state: TransportState,
    /// Last endpoint-local complete typed telemetry configuration.
    telemetry_configuration: TelemetryConfiguration,
    /// How many times `response_get` ran: a replayed read never runs again.
    response_calls: usize,
}

#[cfg(any(test, feature = "test-support"))]
impl Default for MockProvider {
    fn default() -> Self {
        Self {
            current_sample: SampleTime(0),
            controls: Vec::new(),
            structure_epoch: 1,
            control_revision: 0,
            counter_snapshot: CounterSnapshot {
                observed_sample: SampleTime(0),
                values: Vec::new(),
            },
            diagnostics: Vec::new(),
            transport_state: TransportState::Stopped,
            telemetry_configuration: TelemetryConfiguration {
                meter_handles: Vec::new(),
                meter_period_blocks: 0,
                counter_ids: Vec::new(),
                counter_period_blocks: 0,
                diagnostics_enabled: false,
                minimum_diagnostic_severity: crate::DiagnosticSeverity::Info,
            },
            response_calls: 0,
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl MockProvider {
    /// How many `RESPONSE_GET` requests reached the provider.
    #[must_use]
    pub const fn response_calls(&self) -> usize {
        self.response_calls
    }

    /// The rows after the request's cursor, at most its limit, of its owner, and the page's last
    /// handle and end flag.
    fn control_window(
        &self,
        request: &ControlListRequest,
    ) -> Result<(Vec<ControlEntry>, u32, bool), ParameterProviderError> {
        if request.structure_epoch != 0 && request.structure_epoch != self.structure_epoch {
            return Err(ParameterProviderError::Stale);
        }
        let mut rows = self.controls.iter().filter(|entry| {
            entry.handle > request.after_handle
                && request.owner.as_ref().is_none_or(|owner| {
                    entry.address.owner_kind == owner.kind && entry.address.owner_id == owner.id
                })
        });
        let entries = rows
            .by_ref()
            .take(usize::from(request.limit))
            .cloned()
            .collect::<Vec<_>>();
        let last_handle = entries
            .last()
            .map_or(request.after_handle, |entry| entry.handle);
        Ok((entries, last_handle, rows.next().is_none()))
    }
}

#[cfg(any(test, feature = "test-support"))]
impl ControlProvider for MockProvider {
    fn current_sample(&mut self) -> SampleTime {
        self.current_sample
    }

    fn control_list(
        &mut self,
        request: &ControlListRequest,
    ) -> Result<ControlListPage, ParameterProviderError> {
        let (entries, last_handle, eof) = self.control_window(request)?;
        Ok(ControlListPage {
            revision: self.control_revision,
            structure_epoch: self.structure_epoch,
            last_handle,
            eof,
            entries,
        })
    }
    fn control_state(
        &mut self,
        request: &ControlListRequest,
    ) -> Result<ControlStatePage, ParameterProviderError> {
        let (entries, last_handle, eof) = self.control_window(request)?;
        Ok(ControlStatePage {
            revision: self.control_revision,
            structure_epoch: self.structure_epoch,
            last_handle,
            eof,
            records: entries
                .iter()
                .filter_map(|entry| match &entry.body {
                    ControlBody::Control(row) => Some(crate::ControlStateRecord {
                        handle: entry.handle,
                        value: row.value,
                    }),
                    ControlBody::OwnerHeader(_) | ControlBody::SectionHeader(_) => None,
                })
                .collect(),
        })
    }
    fn response_get(
        &mut self,
        _model: &session::SessionModel,
        request: &ResponseGetRequest,
    ) -> Result<ResponseResult, ResponseProviderError> {
        self.response_calls += 1;
        let refuse = |status, code| Err(ResponseProviderError { status, code });
        match &request.target {
            ResponseTarget::Strip(_) | ResponseTarget::Address(_) => {
                return refuse(StatusCode::Unavailable, "response.target.unavailable");
            }
            ResponseTarget::Configuration(_) => {}
        }
        if !(2..=4096).contains(&request.points) {
            return refuse(StatusCode::InvalidField, "response.points");
        }
        let points = request.points as usize;
        let sides = |wanted: bool| {
            wanted.then(|| (0..points).map(|index| -(index as f32)).collect::<Vec<_>>())
        };
        let left = matches!(
            request.channels,
            ParameterChannel::Left | ParameterChannel::Both
        );
        let right = matches!(
            request.channels,
            ParameterChannel::Right | ParameterChannel::Both
        );
        let sections =
            |wanted: bool| (wanted && request.sections).then(|| vec![-1.0_f32; 2 * points]);
        Ok(ResponseResult::Curves(ResponseCurves {
            sample_rate_hz: 48_000,
            configuration_id: request.configuration_id.unwrap_or(0),
            floor_db: -120.0,
            bypass: false,
            section_count: 2,
            enabled_left: 0b11,
            enabled_right: 0b01,
            retained_bytes: 64,
            frequencies_hz: (0..points).map(|index| 20.0 + index as f32).collect(),
            left_db: sides(left),
            right_db: sides(right),
            sections_left_db: sections(left),
            sections_right_db: sections(right),
        }))
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
            effective_sample: self.current_sample,
        }
    }
    fn transport_set(&mut self, request: TransportSetRequest) -> TransportSnapshot {
        self.transport_state = request.state;
        self.transport_get()
    }
    fn telemetry_configure(
        &mut self,
        configuration: TelemetryConfiguration,
    ) -> Result<TelemetryConfiguration, ParameterProviderError> {
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
        Ok(configuration)
    }
}

/// A typed control command after BTLV header validation. It deliberately has no PCM/media case.
#[allow(missing_docs)] // Variant names and fields are the frozen command schema registry.
pub enum ControlCommand<'a> {
    CapabilitiesGet,
    SessionSnapshotGet {
        offset: u64,
        max_bytes: u32,
    },
    SessionTransactionApply {
        edits: &'a [SessionEdit],
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
    ControlListGet {
        request: ControlListRequest,
    },
    ControlStateGet {
        request: ControlListRequest,
    },
    ResponseGet {
        request: ResponseGetRequest,
    },
}

impl ControlCommand<'_> {
    fn requires_exact_revision(&self) -> bool {
        matches!(
            self,
            Self::SessionTransactionApply { .. }
                | Self::TransportSet { .. }
                | Self::TelemetryConfigure { .. }
        )
    }

    const fn message_id(&self) -> MessageId {
        match self {
            Self::CapabilitiesGet => MessageId::CapabilitiesGet,
            Self::SessionSnapshotGet { .. } => MessageId::SessionSnapshotGet,
            Self::SessionTransactionApply { .. } => MessageId::SessionTransactionApply,
            Self::TransportGet => MessageId::TransportGet,
            Self::TransportSet { .. } => MessageId::TransportSet,
            Self::TelemetryConfigure { .. } => MessageId::TelemetryConfigure,
            Self::CountersGet { .. } => MessageId::CountersGet,
            Self::DiagnosticsGet { .. } => MessageId::DiagnosticsGet,
            Self::ControlListGet { .. } => MessageId::ControlListGet,
            Self::ControlStateGet { .. } => MessageId::ControlStateGet,
            Self::ResponseGet { .. } => MessageId::ResponseGet,
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
    commands: [u16; 12],
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
    ControlList(ControlListPage),
    ControlState(ControlStatePage),
    Response(ResponseResult),
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
    /// Atomically copy the complete canonical response to caller output.
    pub fn write_into(&self, output: &mut [u8]) -> Result<usize, EncodeError> {
        copy_complete_frame(&self.bytes, output)
    }
}

/// How a command's transaction is prepared (#1613).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Preparation {
    /// The one-call path: no host monitor overlay, a compilation always.
    Whole,
    /// A host with a monitor overlay: a compilation always.
    Monitor,
    /// A host that commits a transaction of values without a compilation.
    InPlace,
}

struct StructuralCommandPlan {
    request_id: RequestId,
    previous_revision: SessionRevision,
    prospective_session: PreparedSessionTransaction,
    replay_plan: ReplayPreflightPlan,
    event_sequence: u64,
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
    // The replay entry this transaction will take, planned against the committed cache (#1615): the
    // eviction the commit applies, and the request bytes it stores. The committed cache is not
    // cloned; nothing is written to it before the commit, so a dropped token has nothing to undo.
    replay_plan: ReplayPreflightPlan,
    request: Box<[u8]>,
    response: Option<ControllerResponse>,
    event_reservations: Option<crate::ReliableEventReservations>,
    event_sequence: u64,
    applied_operations: u32,
    /// The one `OBSERVATION_ENDED` event this commit emits after its `SESSION_COMMITTED`, carrying
    /// every ended key, with its sequence (#1584 D6a fold). `None` unless the host attached keys.
    observation_ended: Option<ObservationEnded>,
    /// The reliable-event room held for `observation_ended`, checked and reserved before the
    /// host commits anything.
    ended_reservations: Option<crate::ReliableEventReservations>,
    armed: bool,
}

impl PreparedStructuralCommand {
    /// Borrow the complete prospective session compilation for downstream source/plan binding.
    #[must_use]
    pub fn prospective_session(&self) -> &PreparedSessionTransaction {
        self.prospective_session
            .as_ref()
            .expect("live affine token owns prospective session")
    }

    /// The writes to monitor rows (solo) the transaction holds, in transaction order (#1559 D3).
    /// The host applies them to its monitor overlay when it commits the token.
    #[must_use]
    pub fn monitor_edits(&self) -> &[controls::MonitorWrite] {
        self.prospective_session().monitor_edits()
    }

    /// Whether every edit of the transaction writes a monitor row: the commit then changes no
    /// model and no revision, and its response has [`EditPath::Monitor`].
    #[must_use]
    pub fn is_monitor_only(&self) -> bool {
        self.prospective_session().is_monitor_only()
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
    /// The compiled session the commit displaced (#1310 D3), until the caller takes it.
    displaced: Option<session::CompiledSession>,
}

impl CommittedCommandFrame {
    /// Atomically copy the complete committed response to caller output.
    pub fn write_into(&self, output: &mut [u8]) -> Result<usize, EncodeError> {
        copy_complete_frame(&self.bytes, output)
    }

    /// The compiled session this commit displaced (#1310 D3): `Some` once for a transaction
    /// installed as a whole compilation (every rebuild), so a host keeps the predecessor's model
    /// while a successor prepared from it is pending; `None` for a transaction committed in place
    /// or of monitor writes only, which displace none. A frame dropped untaken drops it.
    pub fn take_displaced(&mut self) -> Option<session::CompiledSession> {
        self.displaced.take()
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

/// Why [`ProtocolController::attach_observation_ended`] attached nothing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AttachObservationEndedError {
    /// The token is not this controller's current one.
    Token(PreparedCommandCommitError),
    /// The reliable event queue has no room for the token's `SESSION_COMMITTED` and the event
    /// together: it holds unread events. Nothing changed; a drain makes room.
    NoRoom(QueueReport),
    /// The event cannot be encoded: its frame is over the endpoint's frame limit, a key is not a
    /// stable ID, or the keys are not in strictly ascending order. No drain helps, so the host
    /// refuses the transaction for good and never as backpressure.
    Unencodable,
    /// A token of monitor writes only commits no revision and so emits no event.
    MonitorOnly,
}

impl fmt::Display for AttachObservationEndedError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for AttachObservationEndedError {}

/// Bounded event-egress outcome. The caller always owns output; short output leaves the exact
/// pending event intact for a later retry and no partial frame is written.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventEgressError {
    /// The event family is not enabled by endpoint capabilities/configuration.
    Disabled,
    /// Prepared reliable diagnostic storage has no free slot.
    DiagnosticStorageFull,
    /// A reliable `OBSERVATION_ENDED` slot names a cell that holds no event. An internal
    /// invariant: the commit fills the cell before it queues the slot, and the cell stays until
    /// the slot's frame is delivered. Reported, never expected.
    ObservationEndedCellMissing,
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
        records: [MeterRecord; crate::MAXIMUM_TELEMETRY_PAGE_RECORDS],
    },
    Counter {
        revision: SessionRevision,
        observed_sample: SampleTime,
        len: u16,
        values: [CounterValue; crate::MAXIMUM_TELEMETRY_PAGE_RECORDS],
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
    /// Prepared storage for the keys of queued `OBSERVATION_ENDED` events (#1584 D6a): one cell
    /// more than the reliable queue has slots, because one popped event stays pending in
    /// `pending_reliable_event` until its frame reaches the caller. A cell is taken by a commit
    /// whose room the queue reserved, so a commit never finds none.
    observation_ended_cells: Box<[Option<ObservationEnded>]>,
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
        let ended_cells = diagnostic_slots
            .checked_add(1)
            .ok_or(ControllerResourceAllocationError)?;
        let mut observation_ended_cells = Vec::new();
        observation_ended_cells
            .try_reserve_exact(ended_cells)
            .map_err(|_| ControllerResourceAllocationError)?;
        observation_ended_cells.resize_with(ended_cells, || None);
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
            observation_ended_cells: observation_ended_cells.into_boxed_slice(),
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
            DecodedCommandPayload::ControlListGet(request) => {
                ControlCommand::ControlListGet { request }
            }
            DecodedCommandPayload::ControlStateGet(request) => {
                ControlCommand::ControlStateGet { request }
            }
            DecodedCommandPayload::ResponseGet(request) => ControlCommand::ResponseGet { request },
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
        // The one-call path has no host to apply a monitor write to: it refuses solo, as it always
        // did, rather than acknowledge a write nobody applies.
        match self.plan_structural_command(
            input,
            scratch,
            output.len(),
            header,
            Preparation::Whole,
        )? {
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
        // The prepared path hands the token to a host that owns a monitor overlay (#1559).
        self.prepare_command_frame_with(
            input,
            scratch,
            output_capacity,
            header,
            Preparation::Monitor,
        )
    }

    /// [`Self::prepare_command_frame`] for a host that commits a transaction of values without a
    /// compilation (#1613): a `SESSION_TRANSACTION_APPLY` whose edits are all `SetControl`s, on
    /// existing values or adding parameter entries (#1620), is prepared with [`SessionStore::prepare_transaction_in_place`], so its token
    /// holds the edited owners and no compilation ([`PreparedSessionTransaction::is_in_place`]).
    /// Every other command, and every transaction the in-place preparation cannot prove equal to
    /// the full one, is prepared exactly as [`Self::prepare_command_frame`] prepares it, and its
    /// refusals are its refusals.
    ///
    /// A host that cannot commit an in-place token calls [`Self::materialize_prepared`] first.
    pub fn prepare_command_frame_in_place(
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
        self.prepare_command_frame_with(
            input,
            scratch,
            output_capacity,
            header,
            Preparation::InPlace,
        )
    }

    /// Replace an in-place token's prospective session by the full preparation of its edits
    /// ([`SessionStore::materialize`]), so the host can go on with the compiled transaction as
    /// [`Self::prepare_command_frame`] gives it. A token that holds a compilation is left as it is.
    ///
    /// # Errors
    ///
    /// The store's refusal, which cannot happen for a token the store prepared in place: an
    /// in-place preparation accepts only what the full one accepts.
    pub fn materialize_prepared(
        &self,
        prepared: &mut PreparedStructuralCommand,
    ) -> Result<(), SessionStoreError> {
        let Some(session) = prepared.prospective_session.take() else {
            return Ok(());
        };
        match self.session.materialize(session) {
            Ok(session) => {
                prepared.prospective_session = Some(session);
                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    fn prepare_command_frame_with(
        &mut self,
        input: &[u8],
        scratch: &mut DecodeScratch<'_>,
        output_capacity: usize,
        header: CommandHeader,
        preparation: Preparation,
    ) -> Result<PreparedCommandFrame, CommandFrameProcessError> {
        match self.plan_structural_command(input, scratch, output_capacity, header, preparation)? {
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
                edit_path: EditPath::ModelOnly,
            }),
        };
        // The frame staging holds exactly the measured response: the final frame (the same fields
        // with the edit path the host reports) has the same length, so nothing larger is staged.
        let frame_len = crate::OUTER_HEADER_BYTES
            .checked_add(plan.response_measure.length)
            .ok_or(CommandFrameProcessError::Internal)?;
        if frame_len > self.replay.config.max_response_bytes {
            return Err(CommandFrameProcessError::Encode(
                EncodeError::OutputTooSmall {
                    required: frame_len,
                },
            ));
        }
        #[cfg(test)]
        RESPONSE_STAGING_VECS.with(|allocations| {
            allocations.set(allocations.get().saturating_add(1));
        });
        let mut frame = Vec::new();
        frame
            .try_reserve_exact(frame_len)
            .map_err(|_| CommandFrameProcessError::Internal)?;
        frame.resize(frame_len, 0);
        let written = self
            .codec
            .encode_success_response_frame_measured_into(
                &response_frame,
                plan.response_measure,
                &mut frame,
            )
            .map_err(CommandFrameProcessError::Encode)?;
        if written != frame_len {
            return Err(CommandFrameProcessError::Internal);
        }
        let response = ControllerResponse::from_complete_frame(
            header.request_id,
            StatusCode::Ok,
            revision,
            frame,
        );
        // What the commit will write into the committed cache: the request bytes, kept off the
        // cache until then. The plan came from the committed cache in `plan_structural_command`,
        // and nothing writes the cache while this token is outstanding (every entry point refuses
        // on the odd generation set below), so the commit's eviction and completion cannot fail.
        let mut request = Vec::new();
        request
            .try_reserve_exact(input.len())
            .map_err(|_| CommandFrameProcessError::Internal)?;
        request.extend_from_slice(input);
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
            replay_plan: plan.replay_plan,
            request: request.into_boxed_slice(),
            response: Some(response),
            event_reservations: Some(event_reservations),
            event_sequence: plan.event_sequence,
            applied_operations: plan.applied_operations,
            observation_ended: None,
            ended_reservations: None,
            armed: true,
        })
    }

    fn plan_structural_command(
        &self,
        input: &[u8],
        scratch: &mut DecodeScratch<'_>,
        output_capacity: usize,
        header: CommandHeader,
        preparation: Preparation,
    ) -> Result<StructuralCommandDisposition, CommandFrameProcessError> {
        let monitor = preparation != Preparation::Whole;
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
        // A transaction of monitor writes changes no revision and emits no event, so it needs no
        // event room (#1559 D4). Any other commit reserves its `SESSION_COMMITTED` and nothing
        // else: a revision change cancels nothing (#1628).
        let monitor_only = monitor && crate::model::is_monitor_transaction(&edits);
        let required_events = usize::from(!monitor_only);
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
        let prepared_transaction = match preparation {
            Preparation::Whole => self
                .session
                .prepare_transaction(header.expected_revision, &edits),
            Preparation::Monitor => self
                .session
                .prepare_transaction_with_monitor(header.expected_revision, &edits),
            Preparation::InPlace => self
                .session
                .prepare_transaction_in_place(header.expected_revision, &edits),
        };
        let prospective_session = match prepared_transaction {
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
                edit_path: EditPath::ModelOnly,
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
                edit_path: EditPath::ModelOnly,
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

    /// The predicate [`Self::commit_prepared_structural`] decides with, without consuming the
    /// token or changing anything (#1257 D3).
    ///
    /// A host that must act on the token's outcome before committing it -- the C ABI pushes live
    /// records to a running plan and only then commits (#1053 D6) -- calls this first: while it
    /// holds the controller by `&mut`, an `Ok` here means the commit cannot fail.
    ///
    /// # Errors
    ///
    /// [`PreparedCommandCommitError::WrongController`] for a token another controller prepared;
    /// [`PreparedCommandCommitError::StaleGeneration`] when the generation or the revision changed
    /// since the token was prepared.
    pub fn check_prepared_structural(
        &self,
        prepared: &PreparedStructuralCommand,
    ) -> Result<(), PreparedCommandCommitError> {
        if !Arc::ptr_eq(&self.structural_generation, &prepared.owner) {
            return Err(PreparedCommandCommitError::WrongController);
        }
        if self.structural_generation.load(Ordering::Acquire) != prepared.generation
            || self.session.revision() != prepared.previous_revision
        {
            return Err(PreparedCommandCommitError::StaleGeneration);
        }
        Ok(())
    }

    /// Reserve room for the one `OBSERVATION_ENDED` event that `prepared`'s commit emits after its
    /// `SESSION_COMMITTED`, carrying every key in `ended` (#1584 D6a fold, the acked-batch
    /// question), and return the event frame's encoded length.
    ///
    /// The host knows which subscriptions the transaction ends, the controller does not, so the
    /// host calls this after it prepares the token and before it commits anything. `ended` is in
    /// strictly ascending key order (the wire's: strip, console before inserts, effect ID, tap
    /// ID). The event is encoded for size now, off the commit: a frame over the endpoint's limit is
    /// [`AttachObservationEndedError::Unencodable`]. The room is checked together with the
    /// `SESSION_COMMITTED` the token already holds, one slot for the event whatever the number of
    /// keys: a queue holding unread events that leave less room refuses here, with nothing
    /// changed, and the host refuses the transaction as backpressure. After an `Ok` the commit
    /// cannot fail for room and allocates nothing. A second call replaces the first. An empty list
    /// attaches nothing and returns 0.
    ///
    /// # Errors
    ///
    /// [`AttachObservationEndedError`]: the token is another controller's or stale, it is a token of
    /// monitor writes (which emits no event), the queue has no room, or the event cannot be
    /// encoded.
    pub fn attach_observation_ended(
        &mut self,
        prepared: &mut PreparedStructuralCommand,
        ended: Vec<ObservationTapKey>,
    ) -> Result<usize, AttachObservationEndedError> {
        self.check_prepared_structural(prepared)
            .map_err(AttachObservationEndedError::Token)?;
        // Drop an earlier attachment first, so its room counts as free for this one.
        prepared.observation_ended = None;
        prepared.ended_reservations = None;
        if ended.is_empty() {
            return Ok(0);
        }
        if prepared.prospective_session().is_monitor_only() {
            return Err(AttachObservationEndedError::MonitorOnly);
        }
        let event = ObservationEnded {
            // `event_sequence` is the `SESSION_COMMITTED` sequence; this event follows it.
            event_sequence: prepared.event_sequence.saturating_add(1),
            keys: ended,
        };
        // The size of the frame, which an empty output reports without writing one.
        let revision = prepared.prospective_session().revision();
        let required = match self.codec.encode_event_frame_into(
            &TypedEventFrame {
                revision,
                payload: EventPayload::ObservationEnded(&event),
            },
            &mut [],
        ) {
            Err(EncodeError::OutputTooSmall { required }) => required,
            Err(_) | Ok(_) => return Err(AttachObservationEndedError::Unencodable),
        };
        let reservations = self
            .queues
            .reserve_reliable_events(1)
            .map_err(AttachObservationEndedError::NoRoom)?;
        prepared.observation_ended = Some(event);
        prepared.ended_reservations = Some(reservations);
        Ok(required)
    }

    /// Validate token affinity/currentness, then apply every prepared controller mutation without
    /// another fallible semantic or capacity decision.
    ///
    /// `path` is how the host applied the transaction (#1313 D3). Preparation encodes the response
    /// with [`EditPath::ModelOnly`] because the host classifies after preparing; the frame never
    /// leaves the token before this commit. The commit re-encodes the same frame with `path` into
    /// the token's own buffer, then applies the token's replay plan to the committed cache and
    /// completes the entry with that frame (#1615). The field is fixed width, so the length is
    /// unchanged; the plan was made against the committed cache, which no entry point writes while
    /// the token is outstanding, so neither write can fail. A caller whose token is refused returns
    /// before either write, and a token dropped instead of committed leaves the cache as it was.
    /// The reply exists only after the replay entry is written, in this one call. A token of monitor writes only commits no model and no revision
    /// (#1559 D4): its response takes [`EditPath::Monitor`] whatever `path` says.
    pub fn commit_prepared_structural(
        &mut self,
        mut prepared: PreparedStructuralCommand,
        path: EditPath,
    ) -> Result<CommittedCommandFrame, PreparedCommandCommitError> {
        self.check_prepared_structural(&prepared)?;
        let mut event_reservations = prepared
            .event_reservations
            .take()
            .expect("affine token owns exact event reservations");
        let session = prepared
            .prospective_session
            .take()
            .expect("affine token owns prospective session");
        let response_revision = session.revision();
        // A monitor-only transaction is not a session mutation (#1559 D4): the model and the
        // revision stay and no `SESSION_COMMITTED` follows.
        // Its token reserved no event. The response and the replay entry are recorded as for any
        // other transaction, so an exact replay returns these bytes and applies nothing.
        let monitor_only = session.is_monitor_only();
        let path = if monitor_only {
            EditPath::Monitor
        } else {
            path
        };
        debug_assert!(
            monitor_only || path != EditPath::Monitor,
            "only a transaction of monitor writes has the monitor path"
        );
        let mut displaced = None;
        if !monitor_only {
            let (commit, previous) = self.session.commit_prepared_displacing(session);
            displaced = previous;
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
            // #1584 D6a fold: the one event of the subscriptions this commit ended follows it. Its
            // room was reserved before the host committed, and the event was built and sized
            // then, so this moves it and allocates nothing.
            if let Some(event) = prepared.observation_ended.take() {
                let mut reservations = prepared
                    .ended_reservations
                    .take()
                    .expect("an attached observation-ended event holds its reserved room");
                let cell = self
                    .observation_ended_cells
                    .iter()
                    .position(Option::is_none)
                    .expect("a reserved reliable slot has a free observation-ended cell");
                self.next_reliable_event_sequence = event.event_sequence.saturating_add(1);
                self.observation_ended_cells[cell] = Some(event);
                self.queues.commit_reserved_reliable_event(
                    &mut reservations,
                    ReliableSlot::observation_ended(
                        commit.revision,
                        u32::try_from(cell).expect("cell count is a queue length"),
                    ),
                );
            }
        }
        let response = prepared
            .response
            .take()
            .expect("affine token owns committed response");
        let mut bytes = response.frame;
        let final_frame = TypedSuccessResponseFrame {
            request_id: prepared.request_id,
            revision: response_revision,
            payload: SuccessResponsePayload::SessionTransactionApplied(TransactionApplied {
                applied_operations: prepared.applied_operations,
                edit_path: path,
            }),
        };
        let written = self
            .codec
            .encode_success_response_frame_into(&final_frame, &mut bytes)
            .expect(
                "the edit path is fixed width: the prepared frame buffer holds the final frame",
            );
        assert_eq!(
            written,
            bytes.len(),
            "the final frame has the prepared frame's length"
        );
        // The replay entry is written here, in the same call that returns the reply: the reply
        // exists only after the entry does (the acked-batch rule). The cache is the one the plan was
        // made against: no entry point wrote it while the token was outstanding.
        self.replay.apply_preflight_plan(
            prepared.request_id,
            &prepared.request,
            prepared.replay_plan,
        );
        self.replay
            .complete(prepared.request_id, &prepared.request, &bytes)
            .expect(
                "the plan reserved room for the response, whose length the prepared frame fixed",
            );
        self.structural_generation
            .store(prepared.generation.saturating_add(1), Ordering::Release);
        prepared.armed = false;
        Ok(CommittedCommandFrame { bytes, displaced })
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
        payload: DecodedCommandPayload,
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
            DecodedCommandPayload::ControlListGet(request) => {
                ControlCommand::ControlListGet { request }
            }
            DecodedCommandPayload::ControlStateGet(request) => {
                ControlCommand::ControlStateGet { request }
            }
            DecodedCommandPayload::ResponseGet(request) => ControlCommand::ResponseGet { request },
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
            (MessageId::ControlListGet, Body::ControlList(value)) => {
                encode_success!(SuccessResponsePayload::ControlListPage(value))
            }
            (MessageId::ControlStateGet, Body::ControlState(value)) => {
                encode_success!(SuccessResponsePayload::ControlStatePage(value))
            }
            (MessageId::ResponseGet, Body::Response(value)) => {
                encode_success!(SuccessResponsePayload::ResponseResult(value))
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

    /// Mutably borrow the provider on its endpoint-owning control thread.
    pub fn provider_mut(&mut self) -> &mut P {
        &mut self.provider
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
            if let ReliablePayload::ObservationEnded { cell } = slot.payload
                && let Ok(index) = usize::try_from(cell)
                && let Some(storage) = self.observation_ended_cells.get_mut(index)
            {
                *storage = None;
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
            ReliablePayload::TransportState {
                event_sequence,
                state,
                effective_sample,
                origin_request_id,
            } => self.codec.encode_event_frame_into(
                &TypedEventFrame {
                    revision: slot.revision,
                    payload: EventPayload::TransportState(TransportStateEvent {
                        event_sequence,
                        state,
                        effective_sample,
                        origin_request_id,
                    }),
                },
                output,
            ),
            ReliablePayload::ObservationEnded { cell } => {
                let event = usize::try_from(cell)
                    .ok()
                    .and_then(|index| self.observation_ended_cells.get(index))
                    .and_then(Option::as_ref)
                    .ok_or(EventEgressError::ObservationEndedCellMissing)?;
                self.codec.encode_event_frame_into(
                    &TypedEventFrame {
                        revision: slot.revision,
                        payload: EventPayload::ObservationEnded(event),
                    },
                    output,
                )
            }
            ReliablePayload::Diagnostic { diagnostic_slot } => {
                let index = usize::try_from(diagnostic_slot)
                    .map_err(|_| EventEgressError::DiagnosticStorageFull)?;
                let diagnostic = match self.diagnostic_event_slots.get(index) {
                    Some(RetainedDiagnosticSlot::Owned(diagnostic)) => diagnostic,
                    Some(RetainedDiagnosticSlot::Empty) | None => {
                        return Err(EventEgressError::DiagnosticStorageFull);
                    }
                };
                self.codec.encode_event_frame_into(
                    &TypedEventFrame {
                        revision: slot.revision,
                        payload: EventPayload::Diagnostic(diagnostic),
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
            let mut records = [EMPTY_METER_RECORD; crate::MAXIMUM_TELEMETRY_PAGE_RECORDS];
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
            let mut values = [EMPTY_COUNTER_VALUE; crate::MAXIMUM_TELEMETRY_PAGE_RECORDS];
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
            ControlCommand::ControlListGet { .. } | ControlCommand::ControlStateGet { .. } => {
                features.controls
            }
            ControlCommand::ResponseGet { .. } => features.response,
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
                if event_sequence.checked_add(1).is_none() {
                    return self.non_ok(StatusCode::Internal, None);
                }
                let Ok(applied_operations) = u32::try_from(edits.len()) else {
                    return self.non_ok(StatusCode::LimitExceeded, None);
                };
                let previous_revision = self.session.revision();
                let reservation = match self.queues.reserve_reliable_event() {
                    Ok(reservation) => reservation,
                    Err(report) => {
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
                        self.ok_at_revision(
                            commit.revision,
                            Body::TransactionApplied(TransactionApplied {
                                applied_operations,
                                edit_path: EditPath::ModelOnly,
                            }),
                        )
                    }
                    Err(error) => {
                        self.queues.release_reliable_event(reservation);
                        self.transaction_error_outcome(&error)
                    }
                }
            }
            ControlCommand::ControlListGet { request } => {
                match self.provider.control_list(request) {
                    Ok(page) => match self.fit_control_list(page) {
                        Ok(page) => self.ok(Body::ControlList(page)),
                        Err(status) => self.non_ok(status, None),
                    },
                    Err(error) => self.provider_error_outcome(error),
                }
            }
            ControlCommand::ControlStateGet { request } => {
                match self.provider.control_state(request) {
                    Ok(page) => match self.fit_control_state(page) {
                        Ok(page) => self.ok(Body::ControlState(page)),
                        Err(status) => self.non_ok(status, None),
                    },
                    Err(error) => self.provider_error_outcome(error),
                }
            }
            ControlCommand::ResponseGet { request } => {
                // A read of the committed model: nothing it touches survives the call.
                let model = self.session.compiled().normalized_model();
                match self.provider.response_get(model, request) {
                    Ok(reply) => match self.fit_response(&reply) {
                        Ok(()) => self.ok(Body::Response(reply)),
                        Err(error) => self.response_error_outcome(error),
                    },
                    Err(error) => self.response_error_outcome(error),
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
                if event_sequence.checked_add(1).is_none() {
                    return self.non_ok(StatusCode::Internal, None);
                }
                let reservation = match self.queues.reserve_reliable_event() {
                    Ok(reservation) => reservation,
                    Err(report) => {
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
                        snapshot.effective_sample,
                        Some(request.request_id),
                    ),
                );
                self.next_reliable_event_sequence = event_sequence.saturating_add(1);
                self.ok(Body::Transport(snapshot))
            }
            ControlCommand::TelemetryConfigure { configuration } => {
                let configured = match self.provider.telemetry_configure(configuration.clone()) {
                    Ok(configured) => configured,
                    // A refused configuration changes nothing: the stored one stays (#1351 D2).
                    Err(error) => return self.non_ok(status_for_parameter(error), None),
                };
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

    /// The most payload bytes a success response may carry.
    fn response_payload_budget(&self) -> usize {
        self.replay
            .config
            .max_response_bytes
            .saturating_sub(crate::OUTER_HEADER_BYTES)
    }

    /// Trim a control list page to the leading rows whose response fits the endpoint's response
    /// bound: `limit` is the most rows a page holds, not the number it must, so a caller with a
    /// small frame still pages through the list. A page that is trimmed is not the end of the
    /// list. A single row that does not fit is `LIMIT_EXCEEDED`.
    fn fit_control_list(&self, mut page: ControlListPage) -> Result<ControlListPage, StatusCode> {
        let budget = self.response_payload_budget();
        let cursor = page.last_handle;
        loop {
            let frame = TypedSuccessResponseFrame {
                request_id: RequestId::new(1).ok_or(StatusCode::Internal)?,
                revision: self.session.revision(),
                payload: SuccessResponsePayload::ControlListPage(page),
            };
            let measured = self.codec.measure_success_response_frame(&frame);
            let SuccessResponsePayload::ControlListPage(back) = frame.payload else {
                return Err(StatusCode::Internal);
            };
            page = back;
            let count = page.entries.len();
            let Some(keep) = rows_that_fit(measured.map(|measure| measure.length), budget, count)?
            else {
                return Ok(page);
            };
            page.entries.truncate(keep);
            page.last_handle = page.entries.last().map_or(cursor, |entry| entry.handle);
            page.eof = false;
        }
    }

    /// [`Self::fit_control_list`] for a control state page.
    fn fit_control_state(
        &self,
        mut page: ControlStatePage,
    ) -> Result<ControlStatePage, StatusCode> {
        let budget = self.response_payload_budget();
        loop {
            let frame = TypedSuccessResponseFrame {
                request_id: RequestId::new(1).ok_or(StatusCode::Internal)?,
                revision: self.session.revision(),
                payload: SuccessResponsePayload::ControlStatePage(page),
            };
            let measured = self.codec.measure_success_response_frame(&frame);
            let SuccessResponsePayload::ControlStatePage(back) = frame.payload else {
                return Err(StatusCode::Internal);
            };
            page = back;
            let count = page.records.len();
            let Some(keep) = rows_that_fit(measured.map(|measure| measure.length), budget, count)?
            else {
                return Ok(page);
            };
            page.records.truncate(keep);
            page.last_handle = page
                .records
                .last()
                .map_or(page.last_handle, |record| record.handle);
            page.eof = false;
        }
    }

    /// Whether a response reply fits the endpoint's response bound. A reply that does not is
    /// `LIMIT_EXCEEDED` (`response.reply_too_large`); a provider that checked the size first
    /// never reaches this.
    fn fit_response(&self, reply: &ResponseResult) -> Result<(), ResponseProviderError> {
        let too_large = ResponseProviderError {
            status: StatusCode::LimitExceeded,
            code: "response.reply_too_large",
        };
        match self.codec.encoded_response_result_len(reply) {
            Ok(length) if length <= self.response_payload_budget() => Ok(()),
            Ok(_) | Err(EncodeError::LimitExceeded) => Err(too_large),
            Err(_) => Err(ResponseProviderError {
                status: StatusCode::Internal,
                code: "response.internal",
            }),
        }
    }

    /// The outcome of a `RESPONSE_GET` refusal: its status and one error diagnostic that carries
    /// its code.
    fn response_error_outcome(&self, error: ResponseProviderError) -> Outcome {
        self.non_ok_value(
            error.status,
            NonOkResponse {
                diagnostics: vec![Diagnostic {
                    code: error.code.to_owned(),
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

    /// The outcome of a provider refusal. A stale structure is a revision conflict that names
    /// itself, so a caller refetches the list from the top (`control.list.stale`).
    fn provider_error_outcome(&self, error: ParameterProviderError) -> Outcome {
        let code = match error {
            ParameterProviderError::Stale => "control.list.stale",
            _ => return self.non_ok(status_for_parameter(error), None),
        };
        self.non_ok_value(
            status_for_parameter(error),
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
                backpressure: None,
            },
        )
    }

    fn queue_backpressure_outcome(&self, report: QueueReport) -> Outcome {
        let queue_kind = match report.kind {
            crate::QueueKind::ControlCommand => BackpressureQueueKind::ControlCommand,
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
        let mut commands = [0_u16; 12];
        let mut command_len = 0_usize;
        for id in 1_u16..=14 {
            let enabled = match id {
                // The retired IDs are never advertised.
                id if crate::RETIRED_MESSAGE_IDS.contains(&id) => false,
                3 => session_transactions,
                7 => features.transport,
                8 => features.transport && features.transport_events,
                10 => features.counters,
                11 => features.diagnostics,
                12 | 13 => features.controls,
                14 => features.response,
                _ => true,
            };
            if enabled {
                commands[command_len] = id;
                command_len += 1;
            }
        }
        let candidates = [0x8001_u16, 0x8010, 0x8011, 0x8020, 0x8021, 0x8030];
        let mut events = [0_u16; 6];
        let mut event_len = 0_usize;
        for id in candidates {
            let enabled = match id {
                0x8001 => session_transactions,
                0x8010 => features.transport && features.transport_events,
                0x8011 => session_transactions && features.observation_events,
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
        // Bit 5 was `AUTOMATION_ENQUEUE`: retired with the command, never set (#1627).
        let mut flags = ((1 << 7) - 1) & !(1 << 5);
        if !session_transactions {
            flags &= !(1 << 3);
        }
        if features.controls {
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
        if features.response {
            flags |= 1 << 14;
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
            control_command_slots: queue.control_command_slots.get() as u64,
            control_command_bytes: queue.control_command_bytes.get() as u64,
            reliable_response_slots: queue.reliable_response_slots.get() as u64,
            reliable_event_slots: queue.reliable_event_slots.get() as u64,
            telemetry_slots: queue.telemetry_slots.get() as u64,
            replay_entries: replay.entries.get() as u64,
            replay_bytes: replay.bytes.get() as u64,
            maximum_cached_response_bytes: replay.max_response_bytes as u64,
            maximum_control_page_items: 256,
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
        let mut value = NonOkResponse {
            diagnostics: Vec::with_capacity(diagnostics.len().min(maximum)),
            omitted_diagnostics: 0,
            backpressure: None,
        };
        for diagnostic in diagnostics.iter().take(maximum) {
            value.diagnostics.push(diagnostic.clone());
            let fits = self
                .codec
                .encoded_non_ok_payload_len(&value)
                .is_ok_and(|length| {
                    length.saturating_add(crate::OUTER_HEADER_BYTES)
                        <= self.replay.config.max_response_bytes
                });
            if !fits {
                value.diagnostics.pop();
                break;
            }
        }
        value.omitted_diagnostics =
            u32::try_from(diagnostics.len().saturating_sub(value.diagnostics.len()))
                .unwrap_or(u32::MAX);
        value
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
                effective_sample,
                origin_request_id,
            },
            output,
        )
    }

    fn structural_outstanding(&self) -> bool {
        self.structural_generation.load(Ordering::Acquire) & 1 != 0
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
                crate::SessionEditError::ConsoleSlotFixed => "session.edit.console_slot_fixed",
                crate::SessionEditError::RampOutOfRange => "session.edit.ramp_out_of_range",
                crate::SessionEditError::ValueOpRetired => "session.edit.value_op_retired",
                // The control core's own code (`control.domain`, ...), unprefixed.
                crate::SessionEditError::Control(error) => error.code(),
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

/// How many of `count` page rows to keep when the page measured `measured` payload bytes against
/// `budget`: `None` when it fits, else fewer rows than `count`, at least one. A page whose one row
/// does not fit is `LIMIT_EXCEEDED`; a codec limit (frame bytes, field count) is the same excess
/// as the response bound, so it halves the page. A page whose content breaks the message's rules
/// (`InvalidField`) is the provider's defect at any size: `INTERNAL`, never trimmed.
fn rows_that_fit(
    measured: Result<usize, EncodeError>,
    budget: usize,
    count: usize,
) -> Result<Option<usize>, StatusCode> {
    let keep = match measured {
        Ok(length) if length <= budget => return Ok(None),
        Ok(length) => {
            let kept = (count as u64) * (budget as u64) / (length.max(1) as u64);
            usize::try_from(kept).unwrap_or(count)
        }
        Err(EncodeError::LimitExceeded) => count / 2,
        Err(_) => return Err(StatusCode::Internal),
    };
    if count <= 1 {
        return Err(StatusCode::LimitExceeded);
    }
    Ok(Some(keep.clamp(1, count - 1)))
}

fn status_for_parameter(error: ParameterProviderError) -> StatusCode {
    match error {
        ParameterProviderError::NotFound => StatusCode::NotFound,
        ParameterProviderError::Stale => StatusCode::RevisionConflict,
        ParameterProviderError::Unavailable => StatusCode::Unavailable,
        ParameterProviderError::ReplayExpired => StatusCode::ReplayExpired,
        ParameterProviderError::LimitExceeded => StatusCode::LimitExceeded,
    }
}

#[cfg(test)]
mod tests;
