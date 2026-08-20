//! Bounded request replay and a control-plane-only protocol dispatcher.
//!
//! The dispatcher consumes typed internal commands in this tranche. Later BTLV payload schemas
//! will construct the same commands after the bounded wire decoder has finished; no decoder calls
//! a renderer, and this module has no plan-publication capability.

use core::{fmt, num::NonZeroUsize};
use std::collections::VecDeque;

use crate::{
    AutomationBatchError, AutomationBatchSlot, AutomationCanceled, AutomationCancellationReason,
    AutomationEnqueueError, AutomationEnqueued, Backpressure, BackpressureQueueKind, Capabilities,
    CapabilityFlags, CommandHeader, CounterSnapshot, CounterSnapshotRef, CounterTelemetryRecord,
    CounterValue, CountersRequest, DecodeError, DecodeScratch, DecodedCommandPayload, Diagnostic,
    DiagnosticEvent, DiagnosticsPage, DiagnosticsRequest, EncodeError, EventPayload,
    ExpectedRevision, MessageId, MeterBatch, MeterRecord, NonOkResponse, ParameterAutomationRate,
    ParameterDescriptor, ParameterDomain, ParameterHandle, ParameterMetadataPage,
    ParameterMetadataRequest, ParameterStatePage, ParameterStateRequest, ProtocolCodec,
    ProtocolQueues, QueueReport, ReliablePayload, ReliableSlot, RequestId, SampleTime,
    SessionCommitted, SessionEditV1, SessionRevision, SessionSnapshot, SessionStore,
    SessionStoreError, StatusCode, SuccessResponsePayload, TelemetryConfiguration, TelemetryKey,
    TelemetryRecord, TransactionApplied, TransportSetRequest, TransportSnapshot, TransportState,
    TransportStateEvent, TypedEventFrame, TypedNonOkResponseFrame, TypedSuccessResponseFrame,
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
    Cached(ControllerResponse),
    /// Same request ID had different canonical bytes.
    RequestIdReuse,
    /// Request ID was retired, evicted, or was not strictly increasing as a new request.
    ReplayExpired,
    /// The request is unaccepted because it cannot reserve replay capacity before execution.
    /// Its request ID remains reusable until a later successful preflight accepts it.
    Backpressure,
}

/// One cached exact request/response byte pair.
#[derive(Clone, Debug, Eq, PartialEq)]
struct ReplayEntry {
    request_id: RequestId,
    request: Vec<u8>,
    response: ControllerResponse,
}

/// Bounded exact-byte replay cache. It intentionally covers one endpoint lifetime only.
pub struct ReplayCache {
    config: ReplayCacheConfig,
    entries: VecDeque<ReplayEntry>,
    retained_bytes: usize,
    highest_new_id: Option<RequestId>,
}

impl ReplayCache {
    /// Prepare a bounded replay cache off the render plane.
    #[must_use]
    pub fn new(config: ReplayCacheConfig) -> Self {
        Self {
            config,
            entries: VecDeque::with_capacity(config.entries.get()),
            retained_bytes: 0,
            highest_new_id: None,
        }
    }

    /// Inspect/capacity-reserve a request before execution. An `Execute` result makes enough
    /// room for `max_response_bytes`, so [`Self::complete`] cannot need an unbounded retry.
    ///
    /// A `Backpressure` result is deliberately **unaccepted**: it neither advances the strictly
    /// increasing new-request frontier nor creates a replay entry. This is the endpoint's one
    /// replay-preflight policy, so the same ID remains reusable if capacity later permits it.
    pub fn preflight(&mut self, request_id: RequestId, request: &[u8]) -> ReplayDecision {
        if let Some(entry) = self
            .entries
            .iter()
            .find(|entry| entry.request_id == request_id)
        {
            return if entry.request == request {
                ReplayDecision::Cached(entry.response.clone())
            } else {
                ReplayDecision::RequestIdReuse
            };
        }
        if self
            .highest_new_id
            .is_some_and(|highest| request_id <= highest)
        {
            return ReplayDecision::ReplayExpired;
        }
        let reservation = match request.len().checked_add(self.config.max_response_bytes) {
            Some(value) => value,
            None => return ReplayDecision::Backpressure,
        };
        if reservation > self.config.bytes.get() {
            return ReplayDecision::Backpressure;
        }
        while self.entries.len() >= self.config.entries.get()
            || self.retained_bytes.saturating_add(reservation) > self.config.bytes.get()
        {
            let Some(entry) = self.entries.pop_front() else {
                return ReplayDecision::Backpressure;
            };
            self.retained_bytes = self.retained_bytes.saturating_sub(entry.byte_len());
        }
        self.highest_new_id = Some(request_id);
        ReplayDecision::Execute
    }

    /// Whether this ID would reach new-request admission rather than an already-known replay
    /// result. This is intentionally read-only so caller-output reservation can happen before
    /// preflight advances the endpoint's new-request frontier.
    fn is_new_request(&self, request_id: RequestId) -> bool {
        !self
            .entries
            .iter()
            .any(|entry| entry.request_id == request_id)
            && self
                .highest_new_id
                .is_none_or(|highest| request_id > highest)
    }

    /// Cache the exact bytes from a request that previously received [`ReplayDecision::Execute`].
    /// The response must fit the fixed reservation made by [`Self::preflight`].
    pub fn complete(
        &mut self,
        request_id: RequestId,
        request: &[u8],
        response: ControllerResponse,
    ) -> Result<(), ReplayCacheError> {
        if response.replay_byte_len() > self.config.max_response_bytes {
            return Err(ReplayCacheError::ResponseTooLarge);
        }
        let byte_len = request
            .len()
            .checked_add(response.replay_byte_len())
            .ok_or(ReplayCacheError::ResponseTooLarge)?;
        if self.entries.len() >= self.config.entries.get()
            || self.retained_bytes.saturating_add(byte_len) > self.config.bytes.get()
        {
            return Err(ReplayCacheError::ReservationMissing);
        }
        self.retained_bytes = self.retained_bytes.saturating_add(byte_len);
        self.entries.push_back(ReplayEntry {
            request_id,
            request: request.to_vec(),
            response,
        });
        Ok(())
    }

    /// Number of complete exact-byte entries currently retained.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether no replay entries are retained.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Current retained exact-byte count.
    #[must_use]
    pub const fn retained_bytes(&self) -> usize {
        self.retained_bytes
    }

    /// Return the immutable effective replay capacity configuration.
    #[must_use]
    pub const fn config(&self) -> ReplayCacheConfig {
        self.config
    }
}

impl ReplayEntry {
    fn byte_len(&self) -> usize {
        self.request
            .len()
            .saturating_add(self.response.replay_byte_len())
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
        self.telemetry_configuration = configuration;
        self.telemetry_configuration.clone()
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
        edits: &'a [SessionEditV1],
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
    /// Canonical response bytes remain private replay storage; callers use `encode_payload`.
    bytes: Vec<u8>,
    /// Complete canonical response bytes for the schema-closed full-frame ingress path.
    ///
    /// Older payload-only controller callers intentionally leave this absent.  Keeping this
    /// internal preserves their compatibility without introducing a public raw-frame escape.
    frame_bytes: Option<Vec<u8>>,
}

impl ControllerResponse {
    /// Return the exact caller-buffer byte count for the canonical typed response payload.
    #[must_use]
    pub const fn payload_len(&self) -> usize {
        self.bytes.len()
    }

    /// Copy the already canonical typed response into caller-owned output without exposing a raw
    /// provider payload API.
    pub fn encode_payload(&self, output: &mut [u8]) -> Result<usize, crate::EncodeError> {
        let required = self.bytes.len();
        if output.len() < required {
            return Err(crate::EncodeError::OutputTooSmall { required });
        }
        output[..required].copy_from_slice(&self.bytes);
        Ok(required)
    }

    fn replay_byte_len(&self) -> usize {
        self.frame_bytes.as_ref().map_or(self.bytes.len(), Vec::len)
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
    diagnostic_event_slots: Box<[Option<Diagnostic>]>,
    pending_reliable_event: Option<ReliableSlot>,
    pending_meter_record: Option<TelemetryRecord>,
    pending_counter_record: Option<CounterTelemetryRecord>,
    pending_telemetry_event: PendingTelemetryEvent,
}

impl<P: ControlProvider> ProtocolController<P> {
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
        let diagnostic_slots = queues.config().reliable_event_slots.get();
        Self {
            session,
            queues,
            provider,
            replay,
            codec,
            config,
            next_reliable_event_sequence: 1,
            telemetry_configuration: TelemetryConfiguration {
                meter_handles: Vec::new(),
                meter_period_blocks: 0,
                counter_ids: Vec::new(),
                counter_period_blocks: 0,
                diagnostics_enabled: false,
                minimum_diagnostic_severity: crate::DiagnosticSeverity::Info,
            },
            diagnostic_event_slots: vec![None; diagnostic_slots].into_boxed_slice(),
            pending_reliable_event: None,
            pending_meter_record: None,
            pending_counter_record: None,
            pending_telemetry_event: PendingTelemetryEvent::None,
        }
    }

    /// Process one logical request with exact-byte replay and no renderer call.
    pub fn process(&mut self, request: ControllerRequest<'_>) -> ControllerResponse {
        match self
            .replay
            .preflight(request.request_id, request.canonical_bytes)
        {
            ReplayDecision::Cached(response) => return response,
            ReplayDecision::RequestIdReuse => {
                return self.response(request.request_id, StatusCode::RequestIdReuse, Vec::new());
            }
            ReplayDecision::ReplayExpired => {
                return self.response(request.request_id, StatusCode::ReplayExpired, Vec::new());
            }
            ReplayDecision::Backpressure => {
                return self.replay_backpressure_response(
                    request.request_id,
                    request.canonical_bytes.len(),
                );
            }
            ReplayDecision::Execute => {}
        }
        let response = self.execute(&request);
        match self.replay.complete(
            request.request_id,
            request.canonical_bytes,
            response.clone(),
        ) {
            Ok(()) => response,
            Err(_) => self.response(request.request_id, StatusCode::Internal, Vec::new()),
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
        let frame = codec.decode(input, scratch)?;
        let header = frame
            .header
            .command()
            .ok_or(DecodeError::MessageKindMismatch)?;
        let command = match header.message_id {
            MessageId::CapabilitiesGet => {
                if header.expected_revision != ExpectedRevision::Any {
                    return Err(DecodeError::InvalidTlv);
                }
                codec.decode_capabilities_request(frame.payload, header.tlv_count)?;
                ControlCommand::CapabilitiesGet
            }
            MessageId::SessionSnapshotGet => ControlCommand::SessionSnapshotGet {
                offset: codec
                    .decode_snapshot_request(frame.payload, header.tlv_count)?
                    .offset,
                max_bytes: codec
                    .decode_snapshot_request(frame.payload, header.tlv_count)?
                    .maximum_bytes,
            },
            MessageId::SessionTransactionApply => {
                return self.process_session_transaction_btlv(&codec, input, scratch);
            }
            MessageId::ParameterMetadataGet => ControlCommand::ParameterMetadataGet {
                request: codec
                    .decode_parameter_metadata_request(frame.payload, header.tlv_count)?,
            },
            MessageId::ParameterStateGet => ControlCommand::ParameterStateGet {
                request: codec.decode_parameter_state_request(frame.payload, header.tlv_count)?,
            },
            MessageId::AutomationEnqueue => {
                let decoded = codec.decode_automation_enqueue(frame.payload, header.tlv_count)?;
                let revision = match header.expected_revision {
                    ExpectedRevision::Exact(revision) => revision,
                    ExpectedRevision::Any => SessionRevision(0),
                };
                ControlCommand::AutomationEnqueue {
                    batch: decoded.into_batch(revision, header.request_id)?,
                }
            }
            MessageId::TransportGet => {
                codec.decode_transport_get_request(frame.payload, header.tlv_count)?;
                ControlCommand::TransportGet
            }
            MessageId::TransportSet => ControlCommand::TransportSet {
                request: codec.decode_transport_set_request(frame.payload, header.tlv_count)?,
            },
            MessageId::TelemetryConfigure => ControlCommand::TelemetryConfigure {
                configuration: codec
                    .decode_telemetry_configuration(frame.payload, header.tlv_count)?,
            },
            MessageId::CountersGet => ControlCommand::CountersGet {
                request: codec.decode_counters_request(frame.payload, header.tlv_count)?,
            },
            MessageId::DiagnosticsGet => ControlCommand::DiagnosticsGet {
                request: codec.decode_diagnostics_request(frame.payload, header.tlv_count)?,
            },
            _ => return Err(DecodeError::UnsupportedMessage),
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
        let header = self
            .codec
            .decode_correlatable_command_header(input)
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
            ReplayDecision::Cached(response) => {
                let bytes = response
                    .frame_bytes
                    .as_deref()
                    .ok_or(CommandFrameProcessError::Internal)?;
                return copy_complete_frame(bytes, output)
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

        let mut response = match self.codec.decode_typed_command(input, scratch) {
            Ok(decoded) => self.execute_decoded_command(header, decoded.payload),
            Err(error) => self.response(header.request_id, error.status(), Vec::new()),
        };
        let bytes = self
            .encode_controller_response_frame(header.message_id, &response)
            .map_err(CommandFrameProcessError::Encode)?;
        response.frame_bytes = Some(bytes);
        let result = copy_complete_frame(
            response
                .frame_bytes
                .as_deref()
                .expect("just assigned complete response"),
            output,
        )
        .map_err(CommandFrameProcessError::Encode);
        self.replay
            .complete(header.request_id, input, response)
            .map_err(|_| CommandFrameProcessError::Internal)?;
        result
    }

    fn write_uncached_status_frame(
        &self,
        header: CommandHeader,
        status: StatusCode,
        output: &mut [u8],
    ) -> Result<usize, CommandFrameProcessError> {
        let response = self.response(header.request_id, status, Vec::new());
        let bytes = self
            .encode_controller_response_frame(header.message_id, &response)
            .map_err(CommandFrameProcessError::Encode)?;
        copy_complete_frame(&bytes, output).map_err(CommandFrameProcessError::Encode)
    }

    fn write_uncached_replay_backpressure_frame(
        &self,
        header: CommandHeader,
        request_bytes: usize,
        output: &mut [u8],
    ) -> Result<usize, CommandFrameProcessError> {
        let response = self.replay_backpressure_response(header.request_id, request_bytes);
        let bytes = self
            .encode_controller_response_frame(header.message_id, &response)
            .map_err(CommandFrameProcessError::Encode)?;
        copy_complete_frame(&bytes, output).map_err(CommandFrameProcessError::Encode)
    }

    fn execute_decoded_command(
        &mut self,
        header: CommandHeader,
        payload: DecodedCommandPayload<'_>,
    ) -> ControllerResponse {
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
                    return self.response(header.request_id, StatusCode::InvalidField, Vec::new());
                };
                let batch = match value.into_batch(revision, header.request_id) {
                    Ok(batch) => batch,
                    Err(error) => {
                        return self.response(header.request_id, error.status(), Vec::new());
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

    fn encode_controller_response_frame(
        &self,
        message_id: MessageId,
        response: &ControllerResponse,
    ) -> Result<Vec<u8>, EncodeError> {
        let tlv_count = canonical_tlv_count(&response.bytes)?;
        let required = if response.status == StatusCode::Ok {
            self.encoded_success_response_frame_len(message_id, response, tlv_count)?
        } else {
            let payload = self
                .codec
                .decode_non_ok_payload(&response.bytes, tlv_count)
                .map_err(|_| EncodeError::LimitExceeded)?;
            encoded_non_ok_response_frame_len(&self.codec, &payload)?
        };
        let mut output = vec![0_u8; required];
        if response.status == StatusCode::Ok {
            self.encode_success_response_frame(message_id, response, tlv_count, &mut output)?;
        } else {
            let payload = self
                .codec
                .decode_non_ok_payload(&response.bytes, tlv_count)
                .map_err(|_| EncodeError::LimitExceeded)?;
            self.codec.encode_non_ok_response_frame_into(
                &TypedNonOkResponseFrame {
                    request_id: response.request_id,
                    revision: response.revision,
                    message_id,
                    status: response.status,
                    payload: &payload,
                },
                &mut output,
            )?;
        }
        Ok(output)
    }

    fn encoded_success_response_frame_len(
        &self,
        message_id: MessageId,
        response: &ControllerResponse,
        tlv_count: u32,
    ) -> Result<usize, EncodeError> {
        let mut empty = [];
        match self.encode_success_response_frame(message_id, response, tlv_count, &mut empty) {
            Err(EncodeError::OutputTooSmall { required }) => Ok(required),
            Err(error) => Err(error),
            Ok(_) => Err(EncodeError::LimitExceeded),
        }
    }

    fn encode_success_response_frame(
        &self,
        message_id: MessageId,
        response: &ControllerResponse,
        tlv_count: u32,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        macro_rules! encode_success {
            ($payload:expr) => {
                self.codec.encode_success_response_frame_into(
                    &TypedSuccessResponseFrame {
                        request_id: response.request_id,
                        revision: response.revision,
                        payload: $payload,
                    },
                    output,
                )
            };
        }
        match message_id {
            // The response body was just constructed from the current controller limits and
            // features. Reconstructing that typed value keeps full-frame encoding on the closed
            // typed API rather than exposing an opaque response-byte route.
            MessageId::CapabilitiesGet => {
                let queue = self.queues.config();
                let replay = self.replay.config();
                let (commands, events, flags) = self.capability_registry();
                encode_success!(SuccessResponsePayload::Capabilities(Capabilities {
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
                    supported_commands: &commands,
                    supported_events: &events,
                    flags,
                }))
            }
            MessageId::SessionSnapshotGet => {
                encode_success!(SuccessResponsePayload::SessionSnapshot(
                    self.codec
                        .decode_snapshot(&response.bytes, tlv_count)
                        .map_err(|_| EncodeError::LimitExceeded)?,
                ))
            }
            MessageId::SessionTransactionApply => {
                encode_success!(SuccessResponsePayload::SessionTransactionApplied(
                    self.codec
                        .decode_transaction_applied(&response.bytes, tlv_count)
                        .map_err(|_| EncodeError::LimitExceeded)?,
                ))
            }
            MessageId::ParameterMetadataGet => {
                encode_success!(SuccessResponsePayload::ParameterMetadata(
                    self.codec
                        .decode_parameter_metadata_page(&response.bytes, tlv_count)
                        .map_err(|_| EncodeError::LimitExceeded)?,
                ))
            }
            MessageId::ParameterStateGet => {
                encode_success!(SuccessResponsePayload::ParameterState(
                    self.codec
                        .decode_parameter_state_page(&response.bytes, tlv_count)
                        .map_err(|_| EncodeError::LimitExceeded)?,
                ))
            }
            MessageId::AutomationEnqueue => {
                encode_success!(SuccessResponsePayload::AutomationEnqueued(
                    self.codec
                        .decode_automation_enqueued(&response.bytes, tlv_count)
                        .map_err(|_| EncodeError::LimitExceeded)?,
                ))
            }
            MessageId::TransportGet => {
                encode_success!(SuccessResponsePayload::TransportGetSnapshot(
                    self.codec
                        .decode_transport_snapshot(&response.bytes, tlv_count)
                        .map_err(|_| EncodeError::LimitExceeded)?,
                ))
            }
            MessageId::TransportSet => {
                encode_success!(SuccessResponsePayload::TransportSetSnapshot(
                    self.codec
                        .decode_transport_snapshot(&response.bytes, tlv_count)
                        .map_err(|_| EncodeError::LimitExceeded)?,
                ))
            }
            MessageId::TelemetryConfigure => {
                encode_success!(SuccessResponsePayload::TelemetryConfiguration(
                    self.codec
                        .decode_telemetry_configuration(&response.bytes, tlv_count)
                        .map_err(|_| EncodeError::LimitExceeded)?,
                ))
            }
            MessageId::CountersGet => encode_success!(SuccessResponsePayload::CounterSnapshot(
                self.codec
                    .decode_counter_snapshot(&response.bytes, tlv_count)
                    .map_err(|_| EncodeError::LimitExceeded)?,
            )),
            MessageId::DiagnosticsGet => encode_success!(SuccessResponsePayload::DiagnosticsPage(
                self.codec
                    .decode_diagnostics_page(&response.bytes, tlv_count)
                    .map_err(|_| EncodeError::LimitExceeded)?,
            )),
            _ => Err(EncodeError::MessageKindMismatch),
        }
    }

    /// Borrow authoritative control-plane session state.
    #[must_use]
    pub const fn session(&self) -> &SessionStore {
        &self.session
    }

    /// Mutably borrow preallocated protocol queues for fixed render-side consumption fixtures.
    pub fn queues_mut(&mut self) -> &mut ProtocolQueues {
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
        self.config.provider_features = features;
    }

    /// Stage one mock/control-only meter batch for explicitly configured lossy event egress.
    /// This does not create production meters or touch a render plan.
    pub fn stage_meter_batch_event(
        &mut self,
        revision: SessionRevision,
        observed_sample: SampleTime,
        records: &[MeterRecord],
    ) -> Result<(), EventEgressError> {
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
        let Some(index) = self.diagnostic_event_slots.iter().position(Option::is_none) else {
            return Err(EventEgressError::DiagnosticStorageFull);
        };
        let slot = u32::try_from(index).map_err(|_| EventEgressError::DiagnosticStorageFull)?;
        self.diagnostic_event_slots[index] = Some(event.diagnostic);
        let queued = ReliableSlot {
            header: crate::ReliableHeader::Event,
            revision,
            message_id: MessageId::Diagnostic,
            payload: ReliablePayload::Diagnostic {
                diagnostic_slot: slot,
            },
        };
        if let Err(error) = self.queues.try_enqueue_event(queued) {
            self.diagnostic_event_slots[index] = None;
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
                let diagnostic = self
                    .diagnostic_event_slots
                    .get(index)
                    .and_then(Option::as_ref)
                    .ok_or(EventEgressError::DiagnosticStorageFull)?;
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

    fn execute(&mut self, request: &ControllerRequest<'_>) -> ControllerResponse {
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
            return self.response(request.request_id, StatusCode::Unavailable, Vec::new());
        }
        if request.command.requires_exact_revision()
            && !matches!(request.expected_revision, ExpectedRevision::Exact(_))
        {
            return self.response(request.request_id, StatusCode::InvalidField, Vec::new());
        }
        if let ExpectedRevision::Exact(expected) = request.expected_revision
            && expected != self.session.revision()
        {
            return self.response(request.request_id, StatusCode::RevisionConflict, Vec::new());
        }
        match &request.command {
            ControlCommand::CapabilitiesGet => match self.encode_capabilities_payload() {
                Ok(bytes) => self.response(request.request_id, StatusCode::Ok, bytes),
                Err(_) => self.response(request.request_id, StatusCode::Internal, Vec::new()),
            },
            ControlCommand::SessionSnapshotGet { offset, max_bytes } => {
                let snapshot = self.session.canonical_snapshot().as_bytes();
                let offset = match usize::try_from(*offset) {
                    Ok(value) if value <= snapshot.len() => value,
                    _ => {
                        return self.response(
                            request.request_id,
                            StatusCode::InvalidField,
                            Vec::new(),
                        );
                    }
                };
                if *max_bytes == 0 {
                    return self.response(request.request_id, StatusCode::InvalidField, Vec::new());
                }
                let end = offset
                    .saturating_add(*max_bytes as usize)
                    .min(snapshot.len());
                let value = SessionSnapshot {
                    total_bytes: snapshot.len() as u64,
                    offset: offset as u64,
                    canonical_toml_chunk: &snapshot[offset..end],
                    eof: end == snapshot.len(),
                };
                match self.encode_snapshot_payload(value) {
                    Ok(bytes) => self.response(request.request_id, StatusCode::Ok, bytes),
                    Err(_) => {
                        self.response(request.request_id, StatusCode::LimitExceeded, Vec::new())
                    }
                }
            }
            ControlCommand::SessionTransactionApply { edits } => {
                if edits.is_empty() {
                    return self.response(request.request_id, StatusCode::InvalidField, Vec::new());
                }
                if edits.len() > self.config.maximum_transaction_edits as usize {
                    return self.response(
                        request.request_id,
                        StatusCode::LimitExceeded,
                        Vec::new(),
                    );
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
                    return self.response(request.request_id, StatusCode::Internal, Vec::new());
                };
                let mut cancellation_reservations =
                    match self.queues.reserve_reliable_events(cancellation_batches) {
                        Ok(reservations) => reservations,
                        Err(report) => {
                            return self.queue_backpressure_response(request.request_id, report);
                        }
                    };
                let applied_operations = match u32::try_from(edits.len()) {
                    Ok(value) => value,
                    Err(_) => {
                        self.queues
                            .release_reliable_events(cancellation_reservations);
                        return self.response(
                            request.request_id,
                            StatusCode::LimitExceeded,
                            Vec::new(),
                        );
                    }
                };
                let previous_revision = self.session.revision();
                let reservation = match self.queues.reserve_reliable_event() {
                    Ok(reservation) => reservation,
                    Err(report) => {
                        self.queues
                            .release_reliable_events(cancellation_reservations);
                        return self.queue_backpressure_response(request.request_id, report);
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
                        match self.encode_transaction_applied_payload(TransactionApplied {
                            applied_operations,
                        }) {
                            Ok(bytes) => self.response(request.request_id, StatusCode::Ok, bytes),
                            Err(_) => {
                                self.response(request.request_id, StatusCode::Internal, Vec::new())
                            }
                        }
                    }
                    Err(error) => {
                        self.queues.release_reliable_event(reservation);
                        self.queues
                            .release_reliable_events(cancellation_reservations);
                        self.transaction_error_response(request.request_id, &error)
                    }
                }
            }
            ControlCommand::ParameterMetadataGet {
                request: parameter_request,
            } => match self.provider.parameter_metadata(*parameter_request) {
                Ok(page) => match self.encode_parameter_metadata_payload(&page) {
                    Ok(bytes) => self.response(request.request_id, StatusCode::Ok, bytes),
                    Err(_) => {
                        self.response(request.request_id, StatusCode::LimitExceeded, Vec::new())
                    }
                },
                Err(error) => {
                    self.response(request.request_id, status_for_parameter(error), Vec::new())
                }
            },
            ControlCommand::ParameterStateGet {
                request: parameter_request,
            } => match self.provider.parameter_state(parameter_request) {
                Ok(page) => match self.encode_parameter_state_payload(&page) {
                    Ok(bytes) => self.response(request.request_id, StatusCode::Ok, bytes),
                    Err(_) => {
                        self.response(request.request_id, StatusCode::LimitExceeded, Vec::new())
                    }
                },
                Err(error) => {
                    self.response(request.request_id, status_for_parameter(error), Vec::new())
                }
            },
            ControlCommand::AutomationEnqueue { batch } => {
                if batch.revision != self.session.revision() {
                    return self.response(
                        request.request_id,
                        StatusCode::RevisionConflict,
                        Vec::new(),
                    );
                }
                if let Err(error) = self.validate_automation_domains(batch) {
                    return self.response(request.request_id, error, Vec::new());
                }
                let current_sample = self.provider.current_sample();
                match self.queues.try_enqueue_automation(current_sample, *batch) {
                    Ok(()) => match self.encode_automation_enqueued_payload(batch.len) {
                        Ok(bytes) => self.response(request.request_id, StatusCode::Ok, bytes),
                        Err(_) => {
                            self.response(request.request_id, StatusCode::Internal, Vec::new())
                        }
                    },
                    Err(AutomationEnqueueError::Full { report, .. }) => {
                        match self.encode_automation_backpressure(
                            report.capacity,
                            report.occupancy,
                            report.generation.0,
                        ) {
                            Ok(bytes) => {
                                self.response(request.request_id, StatusCode::Backpressure, bytes)
                            }
                            Err(_) => {
                                self.response(request.request_id, StatusCode::Internal, Vec::new())
                            }
                        }
                    }
                    Err(AutomationEnqueueError::Invalid { error, .. }) => {
                        self.response(request.request_id, status_for_automation(error), Vec::new())
                    }
                }
            }
            ControlCommand::TransportGet => {
                let snapshot = self.provider.transport_get();
                match self.encode_transport_snapshot_payload(snapshot) {
                    Ok(bytes) => self.response(request.request_id, StatusCode::Ok, bytes),
                    Err(_) => self.response(request.request_id, StatusCode::Internal, Vec::new()),
                }
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
                    return self.response(request.request_id, StatusCode::Internal, Vec::new());
                };
                let mut cancellation_reservations =
                    match self.queues.reserve_reliable_events(cancellation_batches) {
                        Ok(reservations) => reservations,
                        Err(report) => {
                            return self.queue_backpressure_response(request.request_id, report);
                        }
                    };
                let reservation = match self.queues.reserve_reliable_event() {
                    Ok(reservation) => reservation,
                    Err(report) => {
                        self.queues
                            .release_reliable_events(cancellation_reservations);
                        return self.queue_backpressure_response(request.request_id, report);
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
                match self.encode_transport_snapshot_payload(snapshot) {
                    Ok(bytes) => self.response(request.request_id, StatusCode::Ok, bytes),
                    Err(_) => self.response(request.request_id, StatusCode::Internal, Vec::new()),
                }
            }
            ControlCommand::TelemetryConfigure { configuration } => {
                let configured = self.provider.telemetry_configure(configuration.clone());
                match self.encode_telemetry_configuration_payload(&configured) {
                    Ok(bytes) => {
                        // The codec has just validated the complete normalized six-field value.
                        // Only then may it enable/mock-configure endpoint event egress.
                        self.telemetry_configuration = configured;
                        self.response(request.request_id, StatusCode::Ok, bytes)
                    }
                    Err(_) => self.response(request.request_id, StatusCode::Internal, Vec::new()),
                }
            }
            ControlCommand::CountersGet {
                request: counters_request,
            } => match self.provider.counters(counters_request) {
                Ok(snapshot) => match self.encode_counter_snapshot_payload(&snapshot) {
                    Ok(bytes) => self.response(request.request_id, StatusCode::Ok, bytes),
                    Err(_) => self.response(request.request_id, StatusCode::Internal, Vec::new()),
                },
                Err(error) => {
                    self.response(request.request_id, status_for_parameter(error), Vec::new())
                }
            },
            ControlCommand::DiagnosticsGet {
                request: diagnostics_request,
            } => match self.provider.diagnostics(*diagnostics_request) {
                Ok(page) => match self.encode_diagnostics_page_payload(&page) {
                    Ok(bytes) => self.response(request.request_id, StatusCode::Ok, bytes),
                    Err(_) => self.response(request.request_id, StatusCode::Internal, Vec::new()),
                },
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
                    match self.encode_non_ok_payload(&expired) {
                        Ok(bytes) => {
                            self.response(request.request_id, StatusCode::ReplayExpired, bytes)
                        }
                        Err(_) => {
                            self.response(request.request_id, StatusCode::Internal, Vec::new())
                        }
                    }
                }
                Err(error) => {
                    self.response(request.request_id, status_for_parameter(error), Vec::new())
                }
            },
        }
    }

    fn response(
        &self,
        request_id: RequestId,
        status: StatusCode,
        mut bytes: Vec<u8>,
    ) -> ControllerResponse {
        if status != StatusCode::Ok && bytes.is_empty() {
            let backpressure = (status == StatusCode::Backpressure).then_some(Backpressure {
                queue_kind: BackpressureQueueKind::ReplayCache,
                capacity: self.replay.config.entries.get() as u64,
                occupancy: self.replay.entries.len() as u64,
                requested_items: 1,
                generation: None,
                retry_boundary: None,
                requested_bytes: None,
                available_bytes: None,
            });
            let value = NonOkResponse {
                diagnostics: vec![Diagnostic {
                    code: if backpressure.is_some() {
                        "protocol.backpressure".to_owned()
                    } else {
                        "protocol.failure".to_owned()
                    },
                    severity: crate::DiagnosticSeverity::Error,
                    path: Vec::new(),
                    detail: None,
                    operation_index: None,
                    sample_time: None,
                    provider_sequence: None,
                }],
                omitted_diagnostics: 0,
                backpressure,
            };
            bytes = self.encode_non_ok_payload(&value).unwrap_or_default();
        }
        ControllerResponse {
            request_id,
            status,
            revision: self.session.revision(),
            bytes,
            frame_bytes: None,
        }
    }

    fn queue_backpressure_response(
        &self,
        request_id: RequestId,
        report: QueueReport,
    ) -> ControllerResponse {
        let queue_kind = match report.kind {
            crate::QueueKind::ControlCommand => BackpressureQueueKind::ControlCommand,
            crate::QueueKind::Automation => BackpressureQueueKind::Automation,
            crate::QueueKind::ReliableResponse => BackpressureQueueKind::ReliableResponse,
            crate::QueueKind::ReliableEvent => BackpressureQueueKind::ReliableEvent,
            crate::QueueKind::Telemetry => BackpressureQueueKind::Telemetry,
        };
        let payload = NonOkResponse {
            diagnostics: vec![Diagnostic {
                code: "protocol.backpressure".to_owned(),
                severity: crate::DiagnosticSeverity::Error,
                path: Vec::new(),
                detail: None,
                operation_index: None,
                sample_time: None,
                provider_sequence: None,
            }],
            omitted_diagnostics: 0,
            backpressure: Some(Backpressure {
                queue_kind,
                capacity: report.capacity as u64,
                occupancy: report.occupancy,
                requested_items: report.requested_slots,
                generation: Some(report.generation.0),
                retry_boundary: None,
                requested_bytes: None,
                available_bytes: None,
            }),
        };
        match self.encode_non_ok_payload(&payload) {
            Ok(bytes) => self.response(request_id, StatusCode::Backpressure, bytes),
            Err(_) => self.response(request_id, StatusCode::Internal, Vec::new()),
        }
    }

    fn replay_backpressure_response(
        &self,
        request_id: RequestId,
        request_bytes: usize,
    ) -> ControllerResponse {
        let config = self.replay.config();
        let requested_bytes = request_bytes.saturating_add(config.max_response_bytes);
        let available_bytes = config
            .bytes
            .get()
            .saturating_sub(self.replay.retained_bytes());
        let payload = NonOkResponse {
            diagnostics: vec![Diagnostic {
                code: "protocol.backpressure".to_owned(),
                severity: crate::DiagnosticSeverity::Error,
                path: Vec::new(),
                detail: None,
                operation_index: None,
                sample_time: None,
                provider_sequence: None,
            }],
            omitted_diagnostics: 0,
            backpressure: Some(Backpressure {
                queue_kind: BackpressureQueueKind::ReplayCache,
                capacity: config.entries.get() as u64,
                occupancy: self.replay.entries.len() as u64,
                requested_items: 1,
                generation: None,
                retry_boundary: None,
                requested_bytes: Some(u64::try_from(requested_bytes).unwrap_or(u64::MAX)),
                available_bytes: Some(u64::try_from(available_bytes).unwrap_or(u64::MAX)),
            }),
        };
        match self.encode_non_ok_payload(&payload) {
            Ok(bytes) => self.response(request_id, StatusCode::Backpressure, bytes),
            Err(_) => self.response(request_id, StatusCode::Internal, Vec::new()),
        }
    }

    fn transaction_error_response(
        &self,
        request_id: RequestId,
        error: &SessionStoreError,
    ) -> ControllerResponse {
        let status = status_for_transaction(error);
        let diagnostics = transaction_error_diagnostics(error);
        if diagnostics.is_empty() {
            return self.response(request_id, status, Vec::new());
        }
        match self.encode_bounded_non_ok_diagnostics(&diagnostics) {
            Ok(bytes) => self.response(request_id, status, bytes),
            Err(_) => {
                // A validation rejection must not fall back to the generic `protocol.failure`
                // identity. The empty common error payload remains canonical and explicitly
                // reports that every original diagnostic was omitted by endpoint limits.
                let value = NonOkResponse {
                    diagnostics: Vec::new(),
                    omitted_diagnostics: u32::try_from(diagnostics.len()).unwrap_or(u32::MAX),
                    backpressure: None,
                };
                match self.encode_non_ok_payload(&value) {
                    Ok(bytes) => self.response(request_id, status, bytes),
                    Err(_) => ControllerResponse {
                        request_id,
                        status,
                        revision: self.session.revision(),
                        bytes: Vec::new(),
                        frame_bytes: None,
                    },
                }
            }
        }
    }

    fn encode_capabilities_payload(&self) -> Result<Vec<u8>, crate::EncodeError> {
        let queue = self.queues.config();
        let replay = self.replay.config();
        let (commands, events, flags) = self.capability_registry();
        let value = Capabilities {
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
            supported_commands: &commands,
            supported_events: &events,
            flags,
        };
        let mut bytes = vec![0; self.codec.encoded_capabilities_len(&value)?];
        self.codec.encode_capabilities(&value, &mut bytes)?;
        Ok(bytes)
    }

    fn capability_registry(&self) -> (Vec<u16>, Vec<u16>, CapabilityFlags) {
        let features = self.config.provider_features;
        let session_transactions =
            features.session_events && self.config.maximum_transaction_edits != 0;
        let mut commands = vec![1_u16, 2, 6, 9];
        if session_transactions {
            commands.push(3);
        }
        if features.parameters {
            commands.extend([4, 5]);
        }
        if features.transport {
            commands.push(7);
        }
        if features.transport && features.transport_events {
            commands.push(8);
        }
        if features.counters {
            commands.push(10);
        }
        if features.diagnostics {
            commands.push(11);
        }
        commands.sort_unstable();
        let mut events = Vec::new();
        if session_transactions {
            events.extend([0x8001_u16, 0x8002]);
        }
        if features.transport && features.transport_events {
            events.push(0x8010);
        }
        if features.meters {
            events.push(0x8020);
        }
        if features.counters {
            events.push(0x8021);
        }
        if features.diagnostics {
            events.push(0x8030);
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
        (commands, events, CapabilityFlags(flags))
    }

    fn effective_maximum_transaction_edits(&self) -> u32 {
        if self.config.provider_features.session_events {
            self.config.maximum_transaction_edits
        } else {
            0
        }
    }
    fn encode_snapshot_payload(
        &self,
        value: SessionSnapshot<'_>,
    ) -> Result<Vec<u8>, crate::EncodeError> {
        let mut bytes = vec![0; self.codec.encoded_snapshot_len(value)?];
        self.codec.encode_snapshot(value, &mut bytes)?;
        Ok(bytes)
    }
    fn encode_transaction_applied_payload(
        &self,
        value: TransactionApplied,
    ) -> Result<Vec<u8>, crate::EncodeError> {
        let mut bytes = vec![0; 16];
        self.codec.encode_transaction_applied(value, &mut bytes)?;
        Ok(bytes)
    }
    fn encode_parameter_metadata_payload(
        &self,
        page: &ParameterMetadataPage,
    ) -> Result<Vec<u8>, crate::EncodeError> {
        let mut bytes = vec![0; self.codec.encoded_parameter_metadata_page_len(page)?];
        self.codec
            .encode_parameter_metadata_page(page, &mut bytes)?;
        Ok(bytes)
    }
    fn encode_parameter_state_payload(
        &self,
        page: &ParameterStatePage,
    ) -> Result<Vec<u8>, crate::EncodeError> {
        let mut bytes = vec![0; self.codec.encoded_parameter_state_page_len(page)?];
        self.codec.encode_parameter_state_page(page, &mut bytes)?;
        Ok(bytes)
    }
    fn encode_automation_enqueued_payload(
        &self,
        accepted_records: u16,
    ) -> Result<Vec<u8>, crate::EncodeError> {
        let report = self.queues.report(crate::QueueKind::Automation);
        let value = AutomationEnqueued {
            accepted_records,
            occupancy: report.occupancy,
            capacity: report.capacity as u64,
            generation: report.generation.0,
        };
        let mut bytes = vec![0; 64];
        self.codec.encode_automation_enqueued(value, &mut bytes)?;
        Ok(bytes)
    }
    fn encode_automation_backpressure(
        &self,
        capacity: usize,
        occupancy: u64,
        generation: u64,
    ) -> Result<Vec<u8>, crate::EncodeError> {
        let value = NonOkResponse {
            diagnostics: Vec::new(),
            omitted_diagnostics: 0,
            backpressure: Some(Backpressure {
                queue_kind: BackpressureQueueKind::Automation,
                capacity: capacity as u64,
                occupancy,
                requested_items: 1,
                generation: Some(generation),
                retry_boundary: None,
                requested_bytes: None,
                available_bytes: None,
            }),
        };
        let mut bytes = vec![0; self.codec.encoded_non_ok_payload_len(&value)?];
        self.codec.encode_non_ok_payload(&value, &mut bytes)?;
        Ok(bytes)
    }
    fn encode_transport_snapshot_payload(
        &self,
        value: TransportSnapshot,
    ) -> Result<Vec<u8>, crate::EncodeError> {
        let mut bytes = vec![0; 48];
        self.codec.encode_transport_snapshot(value, &mut bytes)?;
        Ok(bytes)
    }
    fn encode_telemetry_configuration_payload(
        &self,
        value: &TelemetryConfiguration,
    ) -> Result<Vec<u8>, crate::EncodeError> {
        let mut bytes = vec![0; self.codec.encoded_telemetry_configuration_len(value)?];
        self.codec
            .encode_telemetry_configuration(value, &mut bytes)?;
        Ok(bytes)
    }
    fn encode_counter_snapshot_payload(
        &self,
        value: &CounterSnapshot,
    ) -> Result<Vec<u8>, crate::EncodeError> {
        let mut bytes = vec![0; self.codec.encoded_counter_snapshot_len(value)?];
        self.codec.encode_counter_snapshot(value, &mut bytes)?;
        Ok(bytes)
    }
    fn encode_diagnostics_page_payload(
        &self,
        value: &DiagnosticsPage,
    ) -> Result<Vec<u8>, crate::EncodeError> {
        let mut bytes = vec![0; self.codec.encoded_diagnostics_page_len(value)?];
        self.codec.encode_diagnostics_page(value, &mut bytes)?;
        Ok(bytes)
    }
    fn encode_non_ok_payload(&self, value: &NonOkResponse) -> Result<Vec<u8>, crate::EncodeError> {
        let mut bytes = vec![0; self.codec.encoded_non_ok_payload_len(value)?];
        self.codec.encode_non_ok_payload(value, &mut bytes)?;
        Ok(bytes)
    }

    fn encode_bounded_non_ok_diagnostics(
        &self,
        diagnostics: &[Diagnostic],
    ) -> Result<Vec<u8>, crate::EncodeError> {
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
            let encoded_len = match self.codec.encoded_non_ok_payload_len(&value) {
                Ok(length) => length,
                Err(_) => {
                    break;
                }
            };
            if encoded_len > self.replay.config.max_response_bytes {
                break;
            }
            retained.push(diagnostic.clone());
        }
        let value = NonOkResponse {
            omitted_diagnostics: u32::try_from(diagnostics.len().saturating_sub(retained.len()))
                .unwrap_or(u32::MAX),
            diagnostics: retained,
            backpressure: None,
        };
        self.encode_non_ok_payload(&value)
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
        let batches = usize::try_from(self.queues.report(crate::QueueKind::Automation).occupancy)
            .unwrap_or(usize::MAX);
        let mut reservations = self.queues.reserve_reliable_events(batches)?;
        self.cancel_queued_automation_reserved(&mut reservations, reason, effective_sample)
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

fn canonical_tlv_count(payload: &[u8]) -> Result<u32, EncodeError> {
    let mut offset = 0_usize;
    let mut count = 0_u32;
    while offset != payload.len() {
        let prefix_end = offset
            .checked_add(crate::TLV_PREFIX_BYTES)
            .ok_or(EncodeError::LimitExceeded)?;
        let prefix = payload
            .get(offset..prefix_end)
            .ok_or(EncodeError::LimitExceeded)?;
        let value_len = u32::from_le_bytes(
            prefix[4..8]
                .try_into()
                .map_err(|_| EncodeError::LimitExceeded)?,
        );
        let value_len = usize::try_from(value_len).map_err(|_| EncodeError::LimitExceeded)?;
        let field_end = prefix_end
            .checked_add(value_len)
            .ok_or(EncodeError::LimitExceeded)?;
        let padded_end = field_end
            .checked_add((8 - field_end % 8) % 8)
            .ok_or(EncodeError::LimitExceeded)?;
        if payload.get(offset..padded_end).is_none() {
            return Err(EncodeError::LimitExceeded);
        }
        offset = padded_end;
        count = count.checked_add(1).ok_or(EncodeError::LimitExceeded)?;
    }
    Ok(count)
}

fn encoded_non_ok_response_frame_len(
    codec: &ProtocolCodec,
    payload: &NonOkResponse,
) -> Result<usize, EncodeError> {
    let payload_len = codec.encoded_non_ok_payload_len(payload)?;
    let required = crate::OUTER_HEADER_BYTES
        .checked_add(payload_len)
        .ok_or(EncodeError::LimitExceeded)?;
    if required > codec.limits().max_frame_bytes {
        return Err(EncodeError::LimitExceeded);
    }
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
    diagnostic: &miso_engine_session::Diagnostic,
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

fn session_path_segment_to_protocol(
    segment: &miso_engine_session::PathSegment,
) -> crate::PathSegment {
    match segment {
        miso_engine_session::PathSegment::Field(field) => crate::PathSegment::Field(field.clone()),
        miso_engine_session::PathSegment::Index(index) => crate::PathSegment::Index(
            u64::try_from(*index)
                .expect("usize path indices fit the u64 protocol carrier on supported targets"),
        ),
        miso_engine_session::PathSegment::Id(id) => crate::PathSegment::StableId(id.clone()),
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
    use miso_engine_session::{CompileCaps, parse_session_toml};
    use std::num::NonZeroUsize;

    const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.toml");

    fn id(value: u64) -> RequestId {
        RequestId::new(value).expect("nonzero")
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
            parse_session_toml(EXAMPLE).expect("fixture"),
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

    fn egress_controller(
        reliable_event_slots: usize,
        telemetry_slots: usize,
    ) -> ProtocolController<MockProvider> {
        let session = SessionStore::new(
            parse_session_toml(EXAMPLE).expect("fixture"),
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
        let transaction = [SessionEditV1::SetSessionId {
            session_id: miso_engine_session::StableId::parse("framed-session").expect("ID"),
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
    fn full_frame_ingress_preserves_output_ownership_and_correlates_payload_errors() {
        let transaction = [SessionEditV1::SetSessionId {
            session_id: miso_engine_session::StableId::parse("not-committed").expect("ID"),
        }];
        let input = full_command(
            1,
            ExpectedRevision::Exact(SessionRevision(7)),
            crate::CommandPayload::SessionTransactionApply(&transaction),
        );
        let mut endpoint = controller(8, 2);
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

        let mut malformed = full_command(
            2,
            ExpectedRevision::Any,
            crate::CommandPayload::CapabilitiesGet,
        );
        malformed[20..24].copy_from_slice(&8_u32.to_le_bytes());
        malformed[40..44].copy_from_slice(&1_u32.to_le_bytes());
        malformed.extend_from_slice(&[1, 0, 1, 0, 1, 0, 0, 0]);
        let response = process_full_command(&mut endpoint, &malformed);
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
                edits: &[SessionEditV1::RemoveSource {
                    source_id: miso_engine_session::StableId::parse("voice").expect("id"),
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
            session_path_segment_to_protocol(&miso_engine_session::PathSegment::Field(
                "tracks".to_owned(),
            )),
            crate::PathSegment::Field("tracks".to_owned())
        );
        assert_eq!(
            session_path_segment_to_protocol(&miso_engine_session::PathSegment::Index(3)),
            crate::PathSegment::Index(3)
        );
        assert_eq!(
            session_path_segment_to_protocol(&miso_engine_session::PathSegment::Id(
                "vocal".to_owned(),
            )),
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
                edits: &[SessionEditV1::SetTrackFader {
                    track_id: miso_engine_session::StableId::parse("vocal").expect("stable ID"),
                    fader,
                }],
            },
        };

        let response = controller.process(request);
        assert_eq!(response.status, StatusCode::ValidationFailed);
        let codec = ProtocolCodec::default();
        let decoded = codec
            .decode_non_ok_payload(&response.bytes, 2)
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
        assert_eq!(encoded, response.bytes, "non-OK payload remains canonical");
    }

    #[test]
    fn edit_rejections_use_typed_operation_diagnostics_not_protocol_failure() {
        let mut controller = controller(4, 1);
        let request = ControllerRequest {
            request_id: id(1),
            expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
            canonical_bytes: b"missing-source-edit",
            command: ControlCommand::SessionTransactionApply {
                edits: &[SessionEditV1::RemoveSource {
                    source_id: miso_engine_session::StableId::parse("missing").expect("stable ID"),
                }],
            },
        };
        let response = controller.process(request);
        assert_eq!(response.status, StatusCode::ValidationFailed);
        let decoded = ProtocolCodec::default()
            .decode_non_ok_payload(&response.bytes, 2)
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
            let response = controller.response(
                id(u64::try_from(index).expect("case index fits") + 1),
                case.status,
                Vec::new(),
            );
            assert_eq!(response.status, case.status, "{} status", case.name);
            let decoded = codec
                .decode_non_ok_payload(&response.bytes, case.top_level_tlvs)
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
            assert_eq!(canonical, response.bytes, "{} canonical bytes", case.name);
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
        assert_eq!(cache.highest_new_id, None);
        assert_eq!(cache.preflight(id(1), b"x"), ReplayDecision::Backpressure);
        assert_eq!(cache.highest_new_id, None);
    }

    #[test]
    fn decoded_btlv_transaction_reaches_same_atomic_session_store() {
        let mut controller = controller(4, 1);
        let edits = [SessionEditV1::SetSessionId {
            session_id: miso_engine_session::StableId::parse("renamed").expect("ID"),
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
    fn decoded_track_edit_reaches_same_atomic_session_store() {
        let mut controller = controller(4, 1);
        let mut fader = controller.session().compiled().normalized_model().tracks[0]
            .fader
            .clone();
        fader.left_db = -2.0;
        let edits = [SessionEditV1::SetTrackFader {
            track_id: miso_engine_session::StableId::parse("vocal").expect("ID"),
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
        let edits = [SessionEditV1::SetRouteGainDb {
            route_id: miso_engine_session::StableId::parse("to-main").expect("ID"),
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
        let edits = [SessionEditV1::SetAutomationSegments {
            automation_id: miso_engine_session::StableId::parse("eq-gain").expect("ID"),
            segments: vec![miso_engine_session::AutomationSegment {
                shape: miso_engine_session::AutomationShape::Exponential,
                start_sample: 12,
                end_sample: 12,
                start_value: 0.0,
                end_value: 0.0,
                unit: miso_engine_session::ParameterUnit::Db,
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
                .decode_automation_enqueued(&response.bytes, 4)
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
                .decode_non_ok_payload(&response.bytes, 2)
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
                .decode_transport_snapshot(&get_response.bytes, 3)
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
                .decode_transport_snapshot(&set_response.bytes, 3)
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
            codec.decode_transport_snapshot(&retained.bytes, 3),
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
            codec.decode_telemetry_configuration(&first.bytes, 6),
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
        assert_eq!(second.bytes, first.bytes);

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
        assert_eq!(first.bytes, second.bytes);
        assert_eq!(
            codec
                .decode_counter_snapshot(&first.bytes, 3)
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
            codec.decode_diagnostics_page(&first.bytes, 3),
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
            repeat.bytes, first.bytes,
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
            codec.decode_diagnostics_page(&final_page.bytes, 3),
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
            codec.decode_diagnostics_page(&filtered.bytes, 3),
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
                .decode_non_ok_payload(&expired.bytes, 2)
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
            codec.decode_diagnostics_page(&empty.bytes, 2),
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
                .decode_capabilities(&response.bytes, 27)
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
            .decode_capabilities(&caps.bytes, 27)
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
        let edits = [SessionEditV1::SetSessionId {
            session_id: miso_engine_session::StableId::parse("disabled-events").expect("id"),
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
            .decode_capabilities(&caps.bytes, 27)
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
        let edits = [SessionEditV1::SetSessionId {
            session_id: miso_engine_session::StableId::parse("blocked-commit").expect("ID"),
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
        let edits = [SessionEditV1::SetSessionId {
            session_id: miso_engine_session::StableId::parse("evented-commit").expect("ID"),
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
            .decode_snapshot(&first_response.bytes, 4)
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
            .decode_snapshot(&final_response.bytes, 4)
            .expect("final payload");
        assert!(final_snapshot.eof);
        assert!(final_snapshot.canonical_toml_chunk.is_empty());

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
