//! Raw Wasm32 handle, pointer, and slice boundary.
//!
//! # No `catch_unwind` on this boundary
//!
//! `wasm32-unknown-unknown` is `panic = abort` (`rustc --print cfg` carries `panic="abort"`), and
//! issue 083 D12 makes the whole release profile `panic = "abort"` besides. A panic here traps the
//! Wasm instance; the user agent kills the processor and fires `processorerror`. There is nothing
//! to catch, so the exports are their own bodies: the wrappers this module used to carry produced
//! dead landing pads and pulled the std abort runtime -- which formats and frees on its way to
//! `abort` -- into the render export's call graph. `process()` in the worklet converts a throw from
//! the render export into sticky `RESULT_INTERNAL` and positive-zero output; that is where trap
//! containment lives, in JavaScript, where it is actually observable.

#![allow(unsafe_code)]

#[cfg(test)]
use crate::OBSERVATION_OPERATION_STOP;
use crate::{
    ABI_VERSION, AudioWorkletEngineHost, BUFFER_COMMAND, BUFFER_DIAGNOSTIC, BUFFER_METER_FRAME,
    BUFFER_OUTPUT_PCM, BUFFER_SOURCE_ID, BUFFER_SOURCE_PCM, BootFailure,
    LIVE_RESPONSE_CAPTURE_BYTES, LIVE_RESPONSE_MAXIMUM_ID_BYTES, LIVE_RESPONSE_MAXIMUM_OWNERS,
    LIVE_RESPONSE_MAXIMUM_POINTS, LIVE_RESPONSE_MAXIMUM_SECTIONS,
    LIVE_RESPONSE_MEANING_EQ_FILTER_SUBTOTAL, LIVE_RESPONSE_MODE_TARGET, LIVE_RESPONSE_OWNER_BYTES,
    LIVE_RESPONSE_REQUEST_BYTES, LIVE_RESPONSE_RESULT_BYTES, LIVE_RESPONSE_SECTION_BYTES,
    MAXIMUM_DOCUMENT_BYTES, MAXIMUM_OBSERVATION_READS, OBSERVATION_CAPTURE_KIND_RESPONSE,
    OBSERVATION_CHANNEL_BOTH, OBSERVATION_CHANNEL_LEFT, OBSERVATION_CHANNEL_RIGHT,
    OBSERVATION_OPERATION_CAPTURE_RESPONSE, OBSERVATION_OPERATION_COLLECTION_SELECTION,
    OBSERVATION_OPERATION_METER_LEASE, OBSERVATION_OPERATION_ONE_SHOT,
    OBSERVATION_OPERATION_READ_SPECTRUM, OBSERVATION_OPERATION_REMOVE_METERS_TO,
    OBSERVATION_OPERATION_RESIDENT_READ, OBSERVATION_OPERATION_START_SPECTRUM,
    OBSERVATION_OPERATION_STOP_GRAPH, OBSERVATION_OPERATION_STOP_SPECTRUM,
    OBSERVATION_RESULT_BYTES, OBSERVATION_SELECTION_BYTES, OBSERVATION_STATUS_PENDING,
    OBSERVATION_STATUS_READY, OBSERVATION_STATUS_UNARMED, ObservationAddress, ObservationClass,
    ObservationIngressLimits, ObservationLengths, ObservationReadChannels, ObservationReadError,
    ObservationReadValues, ProtectedResponseCaptureError, ProtectedSpectrumReadError,
    RESPONSE_MAXIMUM_EFFECT_ID_BYTES, RESPONSE_MAXIMUM_PARAMETER_OVERRIDES,
    RESPONSE_MAXIMUM_RESULT_BYTES, RESPONSE_PARAMETER_BYTES, RESPONSE_REQUEST_BYTES,
    RESPONSE_RESULT_BYTES, RESULT_BACKPRESSURE, RESULT_BUFFER_TOO_SMALL, RESULT_INTERNAL,
    RESULT_INVALID_ARGUMENT, RESULT_OK, RESULT_REFUSED_BUDGET, RESULT_REFUSED_DOCUMENT,
    RESULT_REFUSED_LIFECYCLE, RESULT_RENDER_REJECTED, RESULT_UNSUPPORTED, RESULT_WRONG_STATE,
    SPECTRUM_CAPTURE_BYTES, SPECTRUM_CHANNEL_BOTH, SPECTRUM_CHANNEL_LEFT, SPECTRUM_CHANNEL_RIGHT,
    SPECTRUM_COLLECTION_ENTRY_BYTES, SPECTRUM_COLLECTION_REQUEST_BYTES, SPECTRUM_MAXIMUM_ID_BYTES,
    SPECTRUM_REQUEST_BYTES, SPECTRUM_RESULT_HEADER_BYTES, SPECTRUM_STREAM_METADATA_BYTES,
    SPECTRUM_STREAM_STATUS_FAILED, SPECTRUM_STREAM_STATUS_GAP, SPECTRUM_STREAM_STATUS_INACTIVE,
    SPECTRUM_STREAM_STATUS_PENDING, SPECTRUM_STREAM_STATUS_READY, SPECTRUM_STREAM_STATUS_STOPPED,
    SPECTRUM_STREAM_STATUS_WARMING, SPECTRUM_TARGET_OUTPUT,
    SPECTRUM_TARGET_TRACK_POST_INPUT_BUILTINS, SPECTRUM_TARGET_TRACK_POST_MATRIX,
    SPECTRUM_WINDOW_FRAMES, SPECTRUM_WINDOW_HEADER_BYTES, STATE_READY, SpectrumPreparationRequest,
    WebBootOptions, WebLiveResponseOwner, WebLiveResponseRequest, WebLiveResponseResult,
    WebLiveResponseSection, WebObservationAdmission, WebObservationCaptureIdentity,
    WebObservationDemand, WebObservationPreparation, WebObservationPreparationRecord,
    WebObservationProfile, WebObservationReceipt, WebObservationResult, WebObservationSelection,
    WebObservationStatus, WebResponseParameter, WebResponseRequest, WebResponseResult,
    WebSpectrumCollectionEntry, WebSpectrumCollectionRequest, WebSpectrumRequest,
    WebSpectrumResult, WebSpectrumStreamMetadata, WebSpectrumWindow, legacy_response_refusal,
};
use core::{
    cell::{Cell, RefCell},
    mem::{MaybeUninit, size_of},
    ptr, slice,
};
use effect_contract::{EffectQuality, LinkMode, ParameterChannel};
use engine::realtime::{
    ResponseSnapshotError, ResponseSnapshotOwnerInfo, ResponseSnapshotSection, ResponseSnapshotSink,
};
use host_core::{
    GraphObservationActivationConfig, ObservationRefusal, ObservationRefusalReason,
    ObservationWorkLimits, ObservedContinuousSpectrumWindow, ResponseParameterOverride,
    ResponsePreviewError, ResponsePreviewGrid, ResponsePreviewLimits, ResponsePreviewOutput,
    ResponsePreviewRequest, ResponsePreviewTarget, ResponseSnapshot, ResponseSnapshotAvailability,
    ResponseSnapshotOutput, ResponseSnapshotOwner, ResponseSnapshotQueryError,
    SpectrumAnalysisHistory, SpectrumAnalyzer, SpectrumCadence, SpectrumCaptureCollectionEntry,
    SpectrumCaptureCollectionRequest, SpectrumCaptureRequest, SpectrumChannels,
    SpectrumContinuousReadError, SpectrumContinuousWindow, SpectrumSmoothingConfig, SpectrumTarget,
    SpectrumWindow, prepare_response_preview, query_response_snapshot_into,
};

struct LiveHost {
    handle: u32,
    host: Box<AudioWorkletEngineHost>,
}

struct BootStaging {
    options: Box<WebBootOptions>,
    document: Vec<u8>,
    result: u32,
    diagnostic_bytes: u32,
    document_valid: bool,
}

struct ResponseStaging {
    request: Box<WebResponseRequest>,
    effect_id: Box<[u8]>,
    parameters: Box<[WebResponseParameter]>,
    result: Vec<u8>,
    result_header: WebResponseResult,
    live_request: Box<WebLiveResponseRequest>,
    live_track_id: Box<[u8]>,
    live_result: Vec<u8>,
    live_result_len: usize,
    live_result_header: WebLiveResponseResult,
    live_token: u64,
}

struct SpectrumStaging {
    request: Box<WebSpectrumRequest>,
    target_id: Box<[u8]>,
    collection_request: Box<WebSpectrumCollectionRequest>,
    /// Collection slots are allocated only after the caller writes its entry count. Keeping these
    /// vectors empty at thread-local construction is essential: a session that does not request a
    /// collection must not pay for a compiled target-count ceiling or a 32 KiB ID arena.
    collection_entries: Vec<WebSpectrumCollectionEntry>,
    collection_target_ids: Vec<u8>,
    capture: Option<Vec<u8>>,
    capture_len: usize,
    result: Option<Vec<u8>>,
    result_len: usize,
    analyzer: Option<Box<SpectrumAnalyzer>>,
    /// Additive stream metadata is inline so exposing its pointer never allocates after boot.
    stream_metadata: WebSpectrumStreamMetadata,
    /// Stream analysis configuration is control/worker state; render only writes the raw capture.
    stream_active: bool,
    stream_cadence: Option<SpectrumCadence>,
    stream_smoothing: Option<SpectrumSmoothingConfig>,
    stream_history: Option<Box<SpectrumAnalysisHistory>>,
    stream_window: Option<SpectrumStreamWindowFacts>,
    #[cfg(test)]
    stream_history_exhausted_for_test: bool,
}

#[derive(Clone, Copy, Debug, Default)]
struct SpectrumStreamWindowFacts {
    stream_epoch: u64,
    sequence: u64,
    dropped_captures: u64,
}

impl SpectrumStaging {
    fn new() -> Self {
        Self {
            request: Box::new(WebSpectrumRequest {
                struct_size: SPECTRUM_REQUEST_BYTES,
                abi_version: ABI_VERSION,
                ..WebSpectrumRequest::default()
            }),
            target_id: vec![0; SPECTRUM_MAXIMUM_ID_BYTES].into_boxed_slice(),
            collection_request: Box::new(WebSpectrumCollectionRequest {
                struct_size: SPECTRUM_COLLECTION_REQUEST_BYTES,
                abi_version: ABI_VERSION,
                ..WebSpectrumCollectionRequest::default()
            }),
            collection_entries: Vec::new(),
            collection_target_ids: Vec::new(),
            capture: None,
            capture_len: 0,
            result: None,
            result_len: 0,
            analyzer: None,
            stream_metadata: WebSpectrumStreamMetadata {
                struct_size: SPECTRUM_STREAM_METADATA_BYTES,
                abi_version: ABI_VERSION,
                status: SPECTRUM_STREAM_STATUS_INACTIVE,
                ..WebSpectrumStreamMetadata::default()
            },
            stream_active: false,
            stream_cadence: None,
            stream_smoothing: None,
            stream_history: None,
            stream_window: None,
            #[cfg(test)]
            stream_history_exhausted_for_test: false,
        }
    }

    fn configure_capture(&mut self) -> Result<(), u32> {
        if self.capture.is_some() && self.result.is_some() {
            return Ok(());
        }
        #[cfg(test)]
        SPECTRUM_CAPTURE_CONFIGURE_ENTRIES.with(|entries| {
            entries.set(entries.get().saturating_add(1));
        });
        let mut capture = Vec::new();
        capture
            .try_reserve_exact(SPECTRUM_CAPTURE_BYTES)
            .map_err(|_| RESULT_REFUSED_BUDGET)?;
        validate_spectrum_capture_capacity(capture.capacity())?;
        capture.resize(SPECTRUM_CAPTURE_BYTES, 0);
        let mut result = Vec::new();
        result
            .try_reserve_exact(SPECTRUM_CAPTURE_BYTES)
            .map_err(|_| RESULT_REFUSED_BUDGET)?;
        validate_spectrum_capture_capacity(result.capacity())?;
        result.resize(SPECTRUM_CAPTURE_BYTES, 0);
        self.capture = Some(capture);
        self.result = Some(result);
        self.capture_len = 0;
        self.result_len = 0;
        Ok(())
    }

    fn release_capture(&mut self) {
        self.capture = None;
        self.result = None;
        self.analyzer = None;
        self.stream_history = None;
        self.capture_len = 0;
        self.result_len = 0;
        self.stream_active = false;
        self.stream_cadence = None;
        self.stream_smoothing = None;
        self.stream_window = None;
        #[cfg(test)]
        {
            self.stream_history_exhausted_for_test = false;
        }
        self.stream_metadata = WebSpectrumStreamMetadata {
            struct_size: SPECTRUM_STREAM_METADATA_BYTES,
            abi_version: ABI_VERSION,
            status: SPECTRUM_STREAM_STATUS_INACTIVE,
            ..WebSpectrumStreamMetadata::default()
        };
    }

    fn release_collection_staging(&mut self) {
        self.collection_entries = Vec::new();
        self.collection_target_ids = Vec::new();
    }

    fn ensure_analysis_storage(&mut self) -> Result<(), u32> {
        self.configure_capture()?;
        if self.analyzer.is_none() {
            self.analyzer = Some(Box::new(SpectrumAnalyzer::new()));
        }
        Ok(())
    }

    fn ensure_stream_analysis_storage(&mut self) -> Result<(), u32> {
        self.ensure_analysis_storage()?;
        if self.stream_history.is_none() {
            self.stream_history = Some(Box::new(SpectrumAnalysisHistory::new()));
        }
        Ok(())
    }

    fn reset_stream_analysis(&mut self) -> Result<(), u32> {
        if let Some(history) = self.stream_history.as_mut() {
            history.reset().map_err(|_| RESULT_REFUSED_BUDGET)?;
        }
        self.stream_window = None;
        self.result_len = 0;
        self.stream_metadata.analysis_epoch = self
            .stream_history
            .as_ref()
            .map_or(0, |history| history.analysis_epoch());
        self.stream_metadata.history_start_sample = 0;
        Ok(())
    }

    #[cfg(test)]
    fn stream_history_exhausted(&self) -> bool {
        self.stream_history_exhausted_for_test
            || self
                .stream_history
                .as_ref()
                .is_some_and(|history| history.analysis_epoch() == u64::MAX)
    }

    #[cfg(not(test))]
    fn stream_history_exhausted(&self) -> bool {
        self.stream_history
            .as_ref()
            .is_some_and(|history| history.analysis_epoch() == u64::MAX)
    }
}

fn validate_spectrum_capture_capacity(capacity: usize) -> Result<(), u32> {
    if capacity > SPECTRUM_CAPTURE_BYTES {
        Err(RESULT_REFUSED_BUDGET)
    } else {
        Ok(())
    }
}

struct ObservationStaging {
    selections: Box<[WebObservationSelection]>,
    addresses: Vec<ObservationAddress>,
    rows: Box<[ObservationReadValues]>,
    results: Vec<WebObservationResult>,
    id: Box<[u8]>,
    endpoint: ObservationEndpointStaging,
}

/// Fixed protected-observation input/output staging carried by the existing TLS owner.
///
/// This is deliberately inline: it is the endpoint's fixed wire workspace, not a second receipt
/// ledger. The outer host's four receipt rows remain the sole native application authority.
struct ObservationEndpointStaging {
    preparation: WebObservationPreparationRecord,
    demand: WebObservationDemand,
    admission: WebObservationAdmission,
    status: WebObservationStatus,
    capture_identity: WebObservationCaptureIdentity,
    applications: [WebObservationReceipt; 4],
    application_count: u32,
    handle: u32,
    terminal_application_pending: bool,
}

impl Default for ObservationEndpointStaging {
    fn default() -> Self {
        // Keep WebObservationDemand's established zero-default/layout unchanged. Endpoint
        // staging is the owned ABI workspace, so its headers are initialized explicitly here.
        let demand = WebObservationDemand {
            struct_size: size_of::<WebObservationDemand>() as u32,
            abi_version: ABI_VERSION,
            ..WebObservationDemand::default()
        };
        Self {
            preparation: WebObservationPreparationRecord::default(),
            demand,
            admission: WebObservationAdmission::default(),
            status: WebObservationStatus::default(),
            capture_identity: WebObservationCaptureIdentity::default(),
            applications: [WebObservationReceipt::default(); 4],
            application_count: 0,
            handle: 0,
            terminal_application_pending: false,
        }
    }
}

impl ObservationStaging {
    fn new() -> Self {
        Self {
            selections: vec![WebObservationSelection::default(); MAXIMUM_OBSERVATION_READS]
                .into_boxed_slice(),
            addresses: Vec::with_capacity(MAXIMUM_OBSERVATION_READS),
            rows: vec![ObservationReadValues::default(); MAXIMUM_OBSERVATION_READS]
                .into_boxed_slice(),
            results: Vec::with_capacity(MAXIMUM_OBSERVATION_READS),
            id: vec![0; RESPONSE_MAXIMUM_EFFECT_ID_BYTES as usize].into_boxed_slice(),
            endpoint: ObservationEndpointStaging::default(),
        }
    }

    fn reset_results(&mut self) {
        self.addresses.clear();
        self.results.clear();
    }
}

/// Heap payload retained by the fixed selected-observation staging area.
const fn checked_mul_or_zero(left: u64, right: u64) -> u64 {
    match left.checked_mul(right) {
        Some(value) => value,
        None => 0,
    }
}

pub(crate) const fn observation_staging_retained_bytes() -> u64 {
    let count = MAXIMUM_OBSERVATION_READS as u64;
    let containing = size_of::<RefCell<ObservationStaging>>() as u64;
    let mut bytes = containing;
    bytes = match bytes.checked_add(checked_mul_or_zero(
        count,
        size_of::<WebObservationSelection>() as u64,
    )) {
        Some(bytes) => bytes,
        None => 0,
    };
    bytes = match bytes.checked_add(checked_mul_or_zero(
        count,
        size_of::<ObservationAddress>() as u64,
    )) {
        Some(bytes) => bytes,
        None => 0,
    };
    bytes = match bytes.checked_add(checked_mul_or_zero(
        count,
        size_of::<ObservationReadValues>() as u64,
    )) {
        Some(bytes) => bytes,
        None => 0,
    };
    bytes = match bytes.checked_add(checked_mul_or_zero(
        count,
        size_of::<WebObservationResult>() as u64,
    )) {
        Some(bytes) => bytes,
        None => 0,
    };
    match bytes.checked_add(RESPONSE_MAXIMUM_EFFECT_ID_BYTES as u64) {
        Some(bytes) => bytes,
        None => 0,
    }
}

/// Largest one allocation in the fixed selected-observation staging area.
pub(crate) const fn observation_staging_largest_allocation_bytes() -> u64 {
    let count = MAXIMUM_OBSERVATION_READS as u64;
    let containing = size_of::<RefCell<ObservationStaging>>() as u64;
    let selections = checked_mul_or_zero(count, size_of::<WebObservationSelection>() as u64);
    let addresses = checked_mul_or_zero(count, size_of::<ObservationAddress>() as u64);
    let rows = checked_mul_or_zero(count, size_of::<ObservationReadValues>() as u64);
    let results = checked_mul_or_zero(count, size_of::<WebObservationResult>() as u64);
    let ids = RESPONSE_MAXIMUM_EFFECT_ID_BYTES as u64;
    let mut largest = containing;
    if selections > largest {
        largest = selections;
    }
    if addresses > largest {
        largest = addresses;
    }
    if rows > largest {
        largest = rows;
    }
    if results > largest {
        largest = results;
    }
    if ids > largest {
        largest = ids;
    }
    largest
}

/// Heap payload retained by the prewarmed live-response capture staging area.
pub(crate) const fn live_response_staging_retained_bytes() -> u64 {
    // Prewarming this shared workspace also initializes its existing preview request buffers.
    size_of::<WebResponseRequest>() as u64
        + RESPONSE_MAXIMUM_EFFECT_ID_BYTES as u64
        + RESPONSE_MAXIMUM_PARAMETER_OVERRIDES as u64 * size_of::<WebResponseParameter>() as u64
        + size_of::<WebLiveResponseRequest>() as u64
        + LIVE_RESPONSE_MAXIMUM_ID_BYTES as u64
        + LIVE_RESPONSE_CAPTURE_BYTES as u64
}

/// Largest one allocation in the fixed live-response capture staging area.
pub(crate) const fn live_response_staging_largest_allocation_bytes() -> u64 {
    LIVE_RESPONSE_CAPTURE_BYTES as u64
}

const fn max_u64(left: u64, right: u64) -> u64 {
    if left > right { left } else { right }
}

const fn spectrum_capture_payload_bytes() -> u64 {
    (SPECTRUM_CAPTURE_BYTES as u64)
        .checked_mul(2)
        .expect("spectrum capture payload size overflow")
}

pub(crate) const SPECTRUM_CAPTURE_PAYLOAD_BYTES: u64 = spectrum_capture_payload_bytes();

/// Heap payload retained by the one-shot spectrum staging area.
///
/// Request and target-ID staging are always available for the pre-boot write. The two PCM byte
/// buffers are allocated only for a configured capture; the analyzer is worker-owned and is not
/// part of the host's retained resource report.
pub(crate) const fn spectrum_staging_retained_bytes(
    configured: bool,
    collection_entry_bytes: u64,
    collection_target_id_bytes: u64,
) -> u64 {
    size_of::<WebSpectrumRequest>() as u64
        + SPECTRUM_MAXIMUM_ID_BYTES as u64
        + size_of::<WebSpectrumCollectionRequest>() as u64
        + collection_entry_bytes
        + collection_target_id_bytes
        + if configured {
            SPECTRUM_CAPTURE_PAYLOAD_BYTES + SPECTRUM_STREAM_METADATA_BYTES as u64
        } else {
            0
        }
}

/// Separately reported worker-side history retained for managed spectrum analysis.
pub(crate) const fn spectrum_analysis_history_retained_bytes(configured: bool) -> u64 {
    if configured {
        size_of::<SpectrumAnalysisHistory>() as u64
    } else {
        0
    }
}

/// Largest one allocation in the one-shot spectrum staging area.
pub(crate) const fn spectrum_staging_largest_allocation_bytes(
    configured: bool,
    collection_entry_bytes: u64,
    collection_target_id_bytes: u64,
) -> u64 {
    if configured {
        let history = spectrum_analysis_history_retained_bytes(true);
        let capture = max_u64(
            max_u64(SPECTRUM_CAPTURE_BYTES as u64, collection_entry_bytes),
            collection_target_id_bytes,
        );
        if history > capture { history } else { capture }
    } else {
        max_u64(
            max_u64(SPECTRUM_MAXIMUM_ID_BYTES as u64, collection_entry_bytes),
            collection_target_id_bytes,
        )
    }
}

impl ResponseStaging {
    fn new() -> Self {
        Self {
            request: Box::new(WebResponseRequest {
                struct_size: RESPONSE_REQUEST_BYTES,
                abi_version: ABI_VERSION,
                ..WebResponseRequest::default()
            }),
            effect_id: vec![0; RESPONSE_MAXIMUM_EFFECT_ID_BYTES as usize].into_boxed_slice(),
            parameters: vec![
                WebResponseParameter::default();
                RESPONSE_MAXIMUM_PARAMETER_OVERRIDES as usize
            ]
            .into_boxed_slice(),
            result: Vec::new(),
            result_header: WebResponseResult {
                struct_size: RESPONSE_RESULT_BYTES,
                abi_version: ABI_VERSION,
                result: RESULT_OK,
                ..WebResponseResult::default()
            },
            live_request: Box::new(WebLiveResponseRequest {
                struct_size: LIVE_RESPONSE_REQUEST_BYTES,
                abi_version: ABI_VERSION,
                ..WebLiveResponseRequest::default()
            }),
            live_track_id: vec![0; LIVE_RESPONSE_MAXIMUM_ID_BYTES].into_boxed_slice(),
            live_result: vec![0; crate::LIVE_RESPONSE_CAPTURE_BYTES],
            live_result_len: 0,
            live_result_header: WebLiveResponseResult {
                struct_size: LIVE_RESPONSE_RESULT_BYTES,
                abi_version: ABI_VERSION,
                mode: LIVE_RESPONSE_MODE_TARGET,
                meaning: LIVE_RESPONSE_MEANING_EQ_FILTER_SUBTOTAL,
                owner_record_bytes: LIVE_RESPONSE_OWNER_BYTES,
                ..WebLiveResponseResult::default()
            },
            live_token: 0,
        }
    }

    fn reset(&mut self) {
        *self = Self::new();
    }
}

impl BootStaging {
    fn new() -> Self {
        Self {
            options: Box::new(WebBootOptions::default()),
            document: Vec::new(),
            result: RESULT_OK,
            diagnostic_bytes: 0,
            document_valid: false,
        }
    }

    fn record_failure(&mut self, failure: BootFailure) {
        self.result = failure.result();
        let length = failure.diagnostic().len().min(self.document.len());
        self.document[..length].copy_from_slice(&failure.diagnostic()[..length]);
        self.diagnostic_bytes = u32::try_from(length).unwrap_or(0);
        self.document_valid = false;
    }

    fn reset_after_dispose(&mut self) {
        *self.options = WebBootOptions::default();
        self.document = Vec::new();
        self.result = RESULT_OK;
        self.diagnostic_bytes = 0;
        self.document_valid = false;
    }
}

thread_local! {
    static LIVE_HOST: RefCell<Option<LiveHost>> = const { RefCell::new(None) };
    // Zero is the never-issued sentinel. `next_handle` preserves the established nonzero
    // handle contract when the first owner is published, while cold invalid lookups can avoid
    // constructing the allocating observation staging TLS entirely.
    static NEXT_HANDLE: Cell<u32> = const { Cell::new(0) };
    static BOOT_STAGING: RefCell<BootStaging> = RefCell::new(BootStaging::new());
    static RESPONSE_STAGING: RefCell<ResponseStaging> = RefCell::new(ResponseStaging::new());
    static SPECTRUM_STAGING: RefCell<SpectrumStaging> = RefCell::new(SpectrumStaging::new());
    static OBSERVATION_STAGING: RefCell<ObservationStaging> = RefCell::new(ObservationStaging::new());
}

#[cfg(test)]
thread_local! {
    static LIVE_RESPONSE_CAPTURE_FAULT: Cell<Option<ResponseSnapshotError>> = const { Cell::new(None) };
    static LIVE_RESPONSE_CAPTURE_OWNER_CALLS: Cell<u32> = const { Cell::new(0) };
    static SPECTRUM_CAPTURE_CONFIGURE_ENTRIES: Cell<u32> = const { Cell::new(0) };
}

fn next_handle() -> u32 {
    NEXT_HANDLE.with(|next| {
        let result = next.get().max(1);
        next.set(result.wrapping_add(1).max(1));
        result
    })
}

fn with_host<R>(
    handle: u32,
    invalid: R,
    operation: impl FnOnce(&AudioWorkletEngineHost) -> R,
) -> R {
    if handle == 0 {
        return invalid;
    }
    LIVE_HOST.with(|slot| {
        let Ok(slot) = slot.try_borrow() else {
            return invalid;
        };
        let Some(live) = slot.as_ref().filter(|live| live.handle == handle) else {
            return invalid;
        };
        operation(&live.host)
    })
}

fn with_host_mut<R>(
    handle: u32,
    invalid: R,
    operation: impl FnOnce(&mut AudioWorkletEngineHost) -> R,
) -> R {
    if handle == 0 {
        return invalid;
    }
    LIVE_HOST.with(|slot| {
        let Ok(mut slot) = slot.try_borrow_mut() else {
            return invalid;
        };
        let Some(live) = slot.as_mut().filter(|live| live.handle == handle) else {
            return invalid;
        };
        operation(&mut live.host)
    })
}

/// Refuse an unsupported protected alias before it touches caller-owned staging or input.
///
/// The native owner records the unsupported attempt and diagnostic. `Ok(None)` means that a
/// successfully inspected matching handle is legacy, so the existing alias path remains
/// responsible for its legacy behavior. A failed live-host borrow or invalid handle is terminal
/// and must return before any alias-owned staging is touched.
fn refuse_protected_observation_alias(handle: u32, operation: u32) -> Result<Option<u32>, u32> {
    if handle == 0 {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    LIVE_HOST.with(|slot| {
        let mut slot = slot.try_borrow_mut().map_err(|_| RESULT_INTERNAL)?;
        let live = slot
            .as_mut()
            .filter(|live| live.handle == handle)
            .ok_or(RESULT_INVALID_ARGUMENT)?;
        if live.host.protected_observation_prepared() {
            Ok(Some(live.host.refuse_unsupported_observation(operation)))
        } else {
            Ok(None)
        }
    })
}

/// Classify a stream endpoint only after inspecting the matching live host.
///
/// A failed host borrow and every nonmatching handle are terminal at the ABI boundary. They must
/// not be mistaken for a legacy host, because the legacy fallback writes the shared stream
/// staging record on refusal. Only an inspected matching host may select the legacy or protected
/// implementation.
fn stream_host_dispatch(handle: u32) -> Result<bool, u32> {
    if handle == 0 {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    LIVE_HOST.with(|slot| {
        // These aliases may enter the legacy path, which later needs an exclusive host borrow.
        // Acquire that same kind of borrow for classification so an immutable conflict cannot be
        // mistaken for a verified legacy host and mutate stream staging on the fallback path.
        let mut slot = slot.try_borrow_mut().map_err(|_| RESULT_INTERNAL)?;
        let Some(live) = slot.as_mut().filter(|live| live.handle == handle) else {
            return Err(RESULT_INVALID_ARGUMENT);
        };
        Ok(live.host.protected_observation_prepared())
    })
}

fn pointer_u32<T>(pointer: *const T) -> u32 {
    u32::try_from(pointer.addr()).unwrap_or(0)
}

fn buffer_pointer(host: &mut AudioWorkletEngineHost, kind: u32) -> *mut u8 {
    match kind {
        BUFFER_SOURCE_ID => host
            .source_id_mut()
            .map_or(ptr::null_mut(), <[u8]>::as_mut_ptr),
        BUFFER_SOURCE_PCM => host
            .source_pcm_mut()
            .map_or(ptr::null_mut(), |value| value.as_mut_ptr().cast()),
        BUFFER_DIAGNOSTIC => host
            .diagnostic_buffer_mut()
            .map_or(ptr::null_mut(), <[u8]>::as_mut_ptr),
        BUFFER_OUTPUT_PCM => host
            .output_pcm()
            .map_or(ptr::null_mut(), |value| value.as_ptr().cast_mut().cast()),
        BUFFER_COMMAND => host
            .command_staging_mut()
            .map_or(ptr::null_mut(), <[u8]>::as_mut_ptr),
        BUFFER_METER_FRAME => {
            let frame = host.meter_frame();
            if frame.is_empty() {
                ptr::null_mut()
            } else {
                frame.as_ptr().cast_mut().cast()
            }
        }
        _ => ptr::null_mut(),
    }
}

fn buffer_capacity(host: &AudioWorkletEngineHost, kind: u32) -> u32 {
    let resources = host.resources();
    let bytes = match kind {
        BUFFER_SOURCE_ID => resources.id_staging_bytes,
        BUFFER_SOURCE_PCM => resources.source_pcm_staging_bytes,
        BUFFER_DIAGNOSTIC => resources.diagnostic_bytes,
        BUFFER_OUTPUT_PCM => resources.output_pcm_bytes,
        BUFFER_COMMAND => host.command_staging_bytes(),
        BUFFER_METER_FRAME => (host.meter_frame().len() * 4) as u64,
        _ => return 0,
    };
    u32::try_from(bytes).unwrap_or(0)
}

fn observation_rack(raw: u32) -> Result<host_core::EffectRack, u32> {
    match raw {
        0 => Ok(host_core::EffectRack::Simd1),
        1 => Ok(host_core::EffectRack::Dynamic),
        2 => Ok(host_core::EffectRack::Simd2),
        _ => Err(RESULT_INVALID_ARGUMENT),
    }
}

fn observation_channels(raw: u32) -> Result<ObservationReadChannels, u32> {
    match raw {
        OBSERVATION_CHANNEL_LEFT => Ok(ObservationReadChannels::Left),
        OBSERVATION_CHANNEL_RIGHT => Ok(ObservationReadChannels::Right),
        OBSERVATION_CHANNEL_BOTH => Ok(ObservationReadChannels::Both),
        _ => Err(RESULT_INVALID_ARGUMENT),
    }
}

fn observation_rack_raw(rack: host_core::EffectRack) -> u32 {
    match rack {
        host_core::EffectRack::Simd1 => 0,
        host_core::EffectRack::Dynamic => 1,
        host_core::EffectRack::Simd2 => 2,
    }
}

fn observation_status_raw(status: crate::ObservationReadStatus) -> u32 {
    match status {
        crate::ObservationReadStatus::Pending => OBSERVATION_STATUS_PENDING,
        crate::ObservationReadStatus::Unarmed => OBSERVATION_STATUS_UNARMED,
        crate::ObservationReadStatus::Ready => OBSERVATION_STATUS_READY,
    }
}

fn observation_error_code(error: ObservationReadError) -> u32 {
    match error {
        ObservationReadError::WrongState => RESULT_WRONG_STATE,
        ObservationReadError::InvalidSelection => RESULT_INVALID_ARGUMENT,
        ObservationReadError::Unsupported => RESULT_UNSUPPORTED,
        ObservationReadError::BufferTooSmall => RESULT_BUFFER_TOO_SMALL,
    }
}

fn response_error_code(error: ResponsePreviewError) -> u32 {
    match error {
        ResponsePreviewError::UnsupportedEffect => RESULT_UNSUPPORTED,
        ResponsePreviewError::UnknownEffect
        | ResponsePreviewError::InvalidShape
        | ResponsePreviewError::InvalidParameter
        | ResponsePreviewError::DuplicateParameter
        | ResponsePreviewError::ConflictingChannel
        | ResponsePreviewError::InvalidGrid
        | ResponsePreviewError::OutputShape
        | ResponsePreviewError::Owner(_) => RESULT_INVALID_ARGUMENT,
    }
}

fn response_grid(request: WebResponseRequest) -> Result<ResponsePreviewGrid, u32> {
    let points = usize::try_from(request.points).map_err(|_| RESULT_INVALID_ARGUMENT)?;
    match request.grid {
        crate::RESPONSE_GRID_LINEAR => Ok(ResponsePreviewGrid::Linear {
            points,
            minimum_hz: request.minimum_hz,
            maximum_hz: request.maximum_hz,
        }),
        crate::RESPONSE_GRID_LOGARITHMIC => Ok(ResponsePreviewGrid::Logarithmic {
            points,
            minimum_hz: request.minimum_hz,
            maximum_hz: request.maximum_hz,
        }),
        _ => Err(RESULT_INVALID_ARGUMENT),
    }
}

fn response_quality(raw: u32) -> Result<EffectQuality, u32> {
    EffectQuality::from_raw(raw).ok_or(RESULT_INVALID_ARGUMENT)
}

fn response_link_mode(raw: u32) -> Result<LinkMode, u32> {
    LinkMode::from_raw(raw).ok_or(RESULT_INVALID_ARGUMENT)
}

fn response_channel(raw: u32) -> Result<ParameterChannel, u32> {
    ParameterChannel::from_raw(raw).ok_or(RESULT_INVALID_ARGUMENT)
}

fn append_f32_values(result: &mut Vec<u8>, values: &[f32], maximum: usize) -> Result<u32, u32> {
    let offset = u32::try_from(result.len()).map_err(|_| RESULT_REFUSED_BUDGET)?;
    let bytes = values
        .len()
        .checked_mul(size_of::<f32>())
        .ok_or(RESULT_REFUSED_BUDGET)?;
    let end = result
        .len()
        .checked_add(bytes)
        .ok_or(RESULT_REFUSED_BUDGET)?;
    if end > maximum {
        return Err(RESULT_REFUSED_BUDGET);
    }
    result.reserve(bytes);
    for value in values {
        result.extend_from_slice(&value.to_le_bytes());
    }
    Ok(offset)
}

fn response_header_bytes(header: WebResponseResult) -> Vec<u8> {
    let pointer = ptr::from_ref(&header).cast::<u8>();
    // SAFETY: `WebResponseResult` is repr(C), and the exact object is alive for this copy.
    unsafe { slice::from_raw_parts(pointer, size_of::<WebResponseResult>()) }.to_vec()
}

/// Copy one fully initialized ABI record without allocating a temporary byte vector.
///
/// Capture runs through the prepared-plan owner and must remain allocation-free.  The caller
/// supplies fixed staging, so a checked `copy_nonoverlapping` is the only serialization step.
fn copy_live_record<T: Copy>(bytes: &mut [u8], offset: usize, value: &T) -> bool {
    let Some(end) = offset.checked_add(size_of::<T>()) else {
        return false;
    };
    let Some(destination) = bytes.get_mut(offset..end) else {
        return false;
    };
    // SAFETY: `destination` is checked to be exactly the record size and `value` points to a
    // live, fully initialized `repr(C)` record for the duration of this copy.
    unsafe {
        ptr::copy_nonoverlapping(
            ptr::from_ref(value).cast::<u8>(),
            destination.as_mut_ptr(),
            size_of::<T>(),
        );
    }
    true
}

fn live_response_error_code(error: host_core::ResponseSnapshotQueryError) -> u32 {
    match error {
        ResponseSnapshotQueryError::InvalidGrid
        | ResponseSnapshotQueryError::OutputShape
        | ResponseSnapshotQueryError::MalformedSnapshot => RESULT_INVALID_ARGUMENT,
        ResponseSnapshotQueryError::Capacity => RESULT_REFUSED_BUDGET,
        ResponseSnapshotQueryError::UnsupportedProvider => RESULT_UNSUPPORTED,
        ResponseSnapshotQueryError::Numerical => RESULT_INTERNAL,
    }
}

fn live_response_capture_error_code(error: ResponseSnapshotError) -> u32 {
    match error {
        ResponseSnapshotError::Unsupported => RESULT_UNSUPPORTED,
        ResponseSnapshotError::MissingTrack | ResponseSnapshotError::InvalidShape => {
            RESULT_INVALID_ARGUMENT
        }
        ResponseSnapshotError::Capacity => RESULT_REFUSED_BUDGET,
        ResponseSnapshotError::Owner => RESULT_INTERNAL,
    }
}

fn spectrum_channels(raw: u32) -> Result<SpectrumChannels, u32> {
    match raw {
        SPECTRUM_CHANNEL_LEFT => Ok(SpectrumChannels::Left),
        SPECTRUM_CHANNEL_RIGHT => Ok(SpectrumChannels::Right),
        SPECTRUM_CHANNEL_BOTH => Ok(SpectrumChannels::Stereo),
        _ => Err(RESULT_INVALID_ARGUMENT),
    }
}

fn spectrum_target(raw: u32, id: &str) -> Result<SpectrumTarget, u32> {
    if id.is_empty() || id.len() > SPECTRUM_MAXIMUM_ID_BYTES {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    let id: Box<str> = id.into();
    match raw {
        SPECTRUM_TARGET_TRACK_POST_INPUT_BUILTINS => Ok(SpectrumTarget::TrackPostInputBuiltins(id)),
        SPECTRUM_TARGET_TRACK_POST_MATRIX => Ok(SpectrumTarget::TrackPostMatrix(id)),
        SPECTRUM_TARGET_OUTPUT => Ok(SpectrumTarget::Output(id)),
        _ => Err(RESULT_INVALID_ARGUMENT),
    }
}

fn spectrum_configured(staging: &SpectrumStaging) -> bool {
    staging.request.target != 0 || staging.collection_request.entry_count != 0
}

fn staged_spectrum_request(
    staging: &SpectrumStaging,
) -> Result<Option<SpectrumPreparationRequest>, u32> {
    let request = *staging.request;
    if request.target != 0 && staging.collection_request.entry_count != 0 {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    if request.struct_size != SPECTRUM_REQUEST_BYTES
        || request.abi_version != ABI_VERSION
        || request.reserved0 != 0
        || request.reserved != [0; 2]
    {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    if request.target == 0 {
        if request.channels != 0
            || request.target_id_bytes != 0
            || request.maximum_capture_bytes != 0
        {
            return Err(RESULT_INVALID_ARGUMENT);
        }
    } else {
        let id_bytes =
            usize::try_from(request.target_id_bytes).map_err(|_| RESULT_INVALID_ARGUMENT)?;
        if id_bytes == 0 || id_bytes > staging.target_id.len() {
            return Err(RESULT_INVALID_ARGUMENT);
        }
        let id = core::str::from_utf8(&staging.target_id[..id_bytes])
            .map_err(|_| RESULT_INVALID_ARGUMENT)?;
        let target = spectrum_target(request.target, id)?;
        let channels = spectrum_channels(request.channels)?;
        if request.maximum_capture_bytes == 0
            || request.maximum_capture_bytes > SPECTRUM_CAPTURE_BYTES as u64
        {
            return Err(RESULT_REFUSED_BUDGET);
        }
        return Ok(Some(SpectrumPreparationRequest::Single(
            SpectrumCaptureRequest {
                target,
                channels,
                maximum_capture_bytes: request.maximum_capture_bytes,
            },
        )));
    }

    let collection = *staging.collection_request;
    if collection.struct_size != SPECTRUM_COLLECTION_REQUEST_BYTES
        || collection.abi_version != ABI_VERSION
        || collection.reserved0 != 0
        || collection.reserved != [0; 2]
    {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    let count = usize::try_from(collection.entry_count).map_err(|_| RESULT_INVALID_ARGUMENT)?;
    if count > staging.collection_entries.len() {
        return Err(RESULT_REFUSED_BUDGET);
    }
    if count == 0 {
        if collection.maximum_capture_bytes != 0 {
            return Err(RESULT_INVALID_ARGUMENT);
        }
        return Ok(None);
    }
    // A collection's limit covers all of its prepared entries. The one-shot staging cap is a
    // per-entry raw-window bound and must not reject an aggregate collection budget that is larger
    // than one window. The host preparation path applies the checked aggregate/resource limits.
    if collection.maximum_capture_bytes == 0 {
        return Err(RESULT_REFUSED_BUDGET);
    }
    let mut entries = Vec::new();
    entries
        .try_reserve_exact(count)
        .map_err(|_| RESULT_REFUSED_BUDGET)?;
    for (index, staged) in staging.collection_entries[..count].iter().enumerate() {
        if staged.reserved != [0; 3] {
            return Err(RESULT_INVALID_ARGUMENT);
        }
        let id_bytes =
            usize::try_from(staged.target_id_bytes).map_err(|_| RESULT_INVALID_ARGUMENT)?;
        if id_bytes == 0 || id_bytes > SPECTRUM_MAXIMUM_ID_BYTES {
            return Err(RESULT_INVALID_ARGUMENT);
        }
        let id_start = staging.collection_entries[..index]
            .iter()
            .try_fold(0_usize, |offset, previous| {
                offset.checked_add(usize::try_from(previous.target_id_bytes).ok()?)
            })
            .ok_or(RESULT_INVALID_ARGUMENT)?;
        let id_end = id_start
            .checked_add(id_bytes)
            .ok_or(RESULT_INVALID_ARGUMENT)?;
        let id = core::str::from_utf8(
            staging
                .collection_target_ids
                .get(id_start..id_end)
                .ok_or(RESULT_INVALID_ARGUMENT)?,
        )
        .map_err(|_| RESULT_INVALID_ARGUMENT)?;
        entries.push(SpectrumCaptureCollectionEntry {
            target: spectrum_target(staged.target, id)?,
            channels: spectrum_channels(staged.channels)?,
        });
    }
    Ok(Some(SpectrumPreparationRequest::Collection(
        SpectrumCaptureCollectionRequest {
            entries,
            maximum_capture_bytes: collection.maximum_capture_bytes,
        },
    )))
}

fn validate_observation_preparation_record(
    record: &WebObservationPreparationRecord,
) -> Result<(), u32> {
    if record.struct_size != crate::OBSERVATION_PREPARATION_BYTES
        || record.abi_version != ABI_VERSION
        || record.profile != crate::OBSERVATION_PROFILE_EQ_SPECTRUM
        || record.meter_count != 0
        || record.resident_taps != 0
        || record.spectrum_count != 1
        || record.maximum_active_observers != 1
        || record.reserved0 != 0
    {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    let maximum_active_observers =
        usize::try_from(record.maximum_active_observers).map_err(|_| RESULT_INVALID_ARGUMENT)?;
    if maximum_active_observers != 1 {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    let work = record.work_limits;
    if work.struct_size != crate::OBSERVATION_WORK_LIMITS_BYTES || work.abi_version != ABI_VERSION {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    let ingress = record.ingress_limits;
    if ingress.struct_size != crate::OBSERVATION_INGRESS_LIMITS_BYTES
        || ingress.abi_version != ABI_VERSION
    {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    let request = record.spectrum_request;
    if request.struct_size != SPECTRUM_REQUEST_BYTES
        || request.abi_version != ABI_VERSION
        || request.target != SPECTRUM_TARGET_TRACK_POST_MATRIX
        || request.channels != SPECTRUM_CHANNEL_BOTH
        || request.reserved0 != 0
        || request.reserved != [0; 2]
        || request.maximum_capture_bytes == 0
    {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    let target_bytes =
        usize::try_from(request.target_id_bytes).map_err(|_| RESULT_INVALID_ARGUMENT)?;
    if !(1..=SPECTRUM_MAXIMUM_ID_BYTES).contains(&target_bytes)
        || record.target_id[target_bytes..]
            .iter()
            .any(|byte| *byte != 0)
    {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    core::str::from_utf8(&record.target_id[..target_bytes]).map_err(|_| RESULT_INVALID_ARGUMENT)?;
    Ok(())
}

fn spectrum_capture_bytes(channels: u32) -> usize {
    (SPECTRUM_WINDOW_HEADER_BYTES as usize)
        + if channels & SPECTRUM_CHANNEL_LEFT != 0 {
            SPECTRUM_WINDOW_FRAMES as usize * size_of::<f32>()
        } else {
            0
        }
        + if channels & SPECTRUM_CHANNEL_RIGHT != 0 {
            SPECTRUM_WINDOW_FRAMES as usize * size_of::<f32>()
        } else {
            0
        }
}

fn spectrum_failure(staging: &mut SpectrumStaging, result: u32) -> u32 {
    let Some(bytes) = staging.result.as_mut() else {
        staging.result_len = 0;
        return result;
    };
    let header = WebSpectrumResult {
        struct_size: SPECTRUM_RESULT_HEADER_BYTES,
        abi_version: ABI_VERSION,
        result,
        floor_db: host_core::SPECTRUM_FLOOR_DB,
        result_bytes: u64::from(SPECTRUM_RESULT_HEADER_BYTES),
        ..WebSpectrumResult::default()
    };
    let copied = copy_live_record(bytes, 0, &header);
    debug_assert!(copied);
    staging.result_len = SPECTRUM_RESULT_HEADER_BYTES as usize;
    result
}

fn stream_metadata_profile(
    staging: &mut SpectrumStaging,
    target: u32,
    channels: u32,
    cadence: SpectrumCadence,
    status: u32,
    result: u32,
) {
    let metadata = &mut staging.stream_metadata;
    metadata.struct_size = SPECTRUM_STREAM_METADATA_BYTES;
    metadata.abi_version = ABI_VERSION;
    metadata.result = result;
    metadata.status = status;
    metadata.target = target;
    metadata.channels = channels;
    metadata.sample_rate_hz = cadence.sample_rate_hz();
    metadata.quantum_frames = cadence.quantum_frames();
    metadata.hop_frames = cadence.hop_frames();
    metadata.source_underrun = 0;
    metadata.reserved0 = 0;
    metadata.reserved1 = 0;
}

fn stream_metadata_error(
    staging: &mut SpectrumStaging,
    status: u32,
    result: u32,
    capture_epoch: Option<u64>,
    dropped_captures: Option<u64>,
) {
    staging.stream_metadata.result = result;
    staging.stream_metadata.status = status;
    if let Some(epoch) = capture_epoch {
        staging.stream_metadata.capture_epoch = epoch;
    }
    if let Some(drops) = dropped_captures {
        staging.stream_metadata.dropped_captures = drops;
    } else if status == SPECTRUM_STREAM_STATUS_FAILED && capture_epoch.is_some() {
        // Failed windows start a fresh native capture epoch; ordinary pending/warming reads keep
        // the current epoch's accumulated drops visible.
        staging.stream_metadata.dropped_captures = 0;
    }
    staging.stream_metadata.source_underrun = 0;
    staging.capture_len = 0;
    staging.result_len = 0;
    staging.stream_window = None;
}

fn protected_spectrum_read_refusal(
    host: &mut AudioWorkletEngineHost,
    reason: ObservationRefusalReason,
    limit: Option<&'static str>,
    requested: Option<u64>,
    maximum: Option<u64>,
) -> u32 {
    host.record_observation_refusal(
        OBSERVATION_OPERATION_READ_SPECTRUM,
        ObservationRefusal {
            reason,
            limit,
            requested,
            maximum,
        },
    )
}

/// Pack one checked protected stereo window into the existing raw staging bytes.
///
/// Every range used here is preflighted by `protected_spectrum_stream_read` before native
/// consumption. The length is published only after the complete header and both planes have been
/// copied, so a caller never observes a partially committed window.
fn commit_protected_spectrum_window(
    staging: &mut SpectrumStaging,
    window: &ObservedContinuousSpectrumWindow,
    target: u32,
    cadence: SpectrumCadence,
    snapshot_token: u64,
) {
    let header_bytes = SPECTRUM_WINDOW_HEADER_BYTES as usize;
    let plane_bytes = (SPECTRUM_WINDOW_FRAMES as usize) * size_of::<f32>();
    let right_offset = header_bytes + plane_bytes;
    let total_bytes = right_offset + plane_bytes;
    let left_offset = u32::try_from(header_bytes).expect("fixed spectrum header fits u32");
    let right_offset_u32 = u32::try_from(right_offset).expect("fixed spectrum plane fits u32");
    let end_sample = window
        .window
        .end_sample()
        .expect("protected spectrum end was preflighted");
    let header = WebSpectrumWindow {
        struct_size: SPECTRUM_WINDOW_HEADER_BYTES,
        abi_version: ABI_VERSION,
        target,
        channels: SPECTRUM_CHANNEL_BOTH,
        sample_rate_hz: cadence.sample_rate_hz(),
        frames: SPECTRUM_WINDOW_FRAMES,
        source_underrun: u32::from(window.window.source_underrun),
        reserved0: 0,
        captured_sample: window.window.first_sample,
        end_sample,
        snapshot_token,
        left_offset,
        right_offset: right_offset_u32,
    };

    let bytes = staging
        .capture
        .as_mut()
        .expect("protected spectrum capacity was preflighted");
    debug_assert!(bytes.len() >= total_bytes);
    for (index, value) in window.window.left.iter().copied().enumerate() {
        let at = header_bytes + index * size_of::<f32>();
        bytes[at..at + size_of::<f32>()].copy_from_slice(&value.to_le_bytes());
    }
    for (index, value) in window.window.right.iter().copied().enumerate() {
        let at = right_offset + index * size_of::<f32>();
        bytes[at..at + size_of::<f32>()].copy_from_slice(&value.to_le_bytes());
    }
    assert!(
        copy_live_record(bytes, 0, &header),
        "protected spectrum header capacity was preflighted"
    );
    staging.capture_len = total_bytes;
    staging.result_len = 0;
}

fn imported_stream_configuration(
    staging: &SpectrumStaging,
) -> Result<(SpectrumCadence, SpectrumSmoothingConfig), u32> {
    let metadata = staging.stream_metadata;
    let windows = metadata
        .sequence
        .checked_add(1)
        .ok_or(RESULT_INVALID_ARGUMENT)?;
    if metadata.struct_size != SPECTRUM_STREAM_METADATA_BYTES
        || metadata.abi_version != ABI_VERSION
        || metadata.result != RESULT_OK
        || metadata.status != SPECTRUM_STREAM_STATUS_READY
        || !(SPECTRUM_TARGET_TRACK_POST_INPUT_BUILTINS..=SPECTRUM_TARGET_OUTPUT)
            .contains(&metadata.target)
        || !(SPECTRUM_CHANNEL_LEFT..=SPECTRUM_CHANNEL_BOTH).contains(&metadata.channels)
        || metadata.source_underrun > 1
        || metadata.reserved0 != 0
        || metadata.reserved1 != 0
        || metadata.capture_epoch == 0
        || metadata.windows != windows
        || metadata.end_sample
            != metadata
                .captured_sample
                .checked_add(u64::from(SPECTRUM_WINDOW_FRAMES))
                .ok_or(RESULT_INVALID_ARGUMENT)?
    {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    spectrum_channels(metadata.channels)?;
    let cadence = SpectrumCadence::new(metadata.sample_rate_hz, metadata.quantum_frames)
        .map_err(|_| RESULT_INVALID_ARGUMENT)?;
    if cadence.hop_frames() != metadata.hop_frames {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    let smoothing =
        SpectrumSmoothingConfig::new(metadata.smoothing_ms).map_err(|_| RESULT_INVALID_ARGUMENT)?;
    let bytes = staging
        .capture
        .as_ref()
        .and_then(|capture| capture.get(..staging.capture_len))
        .ok_or(RESULT_INVALID_ARGUMENT)?;
    let header: WebSpectrumWindow =
        read_live_record(bytes, 0).map_err(|_| RESULT_INVALID_ARGUMENT)?;
    if header.struct_size != SPECTRUM_WINDOW_HEADER_BYTES
        || header.abi_version != ABI_VERSION
        || header.target != metadata.target
        || header.channels != metadata.channels
        || header.sample_rate_hz != metadata.sample_rate_hz
        || header.frames != SPECTRUM_WINDOW_FRAMES
        || header.source_underrun != metadata.source_underrun
        || header.captured_sample != metadata.captured_sample
        || header.end_sample != metadata.end_sample
        || header.snapshot_token == 0
        || staging.capture_len != spectrum_capture_bytes(header.channels)
    {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    Ok((cadence, smoothing))
}

fn append_spectrum_f32(bytes: &mut [u8], cursor: &mut usize, values: &[f32]) -> Result<u32, u32> {
    let byte_count = values
        .len()
        .checked_mul(size_of::<f32>())
        .ok_or(RESULT_REFUSED_BUDGET)?;
    let end = cursor
        .checked_add(byte_count)
        .ok_or(RESULT_REFUSED_BUDGET)?;
    if end > bytes.len() {
        return Err(RESULT_REFUSED_BUDGET);
    }
    let offset = u32::try_from(*cursor).map_err(|_| RESULT_REFUSED_BUDGET)?;
    for (index, value) in values.iter().copied().enumerate() {
        let at = *cursor + index * size_of::<f32>();
        bytes[at..at + size_of::<f32>()].copy_from_slice(&value.to_le_bytes());
    }
    *cursor = end;
    Ok(offset)
}

fn write_spectrum_window(
    staging: &mut SpectrumStaging,
    window: &SpectrumWindow,
    target: u32,
    sample_rate_hz: u32,
    token: u64,
) -> u32 {
    if !(SPECTRUM_TARGET_TRACK_POST_INPUT_BUILTINS..=SPECTRUM_TARGET_OUTPUT).contains(&target)
        || window.left.len() != SPECTRUM_WINDOW_FRAMES as usize
        || window.right.len() != SPECTRUM_WINDOW_FRAMES as usize
        || sample_rate_hz == 0
        || window.end_sample().is_none()
        || token == 0
    {
        return spectrum_failure(staging, RESULT_INVALID_ARGUMENT);
    }
    let Some(bytes) = staging.capture.as_mut() else {
        return RESULT_REFUSED_BUDGET;
    };
    let maximum = bytes.len();
    let mut cursor = SPECTRUM_WINDOW_HEADER_BYTES as usize;
    let left_offset = if (window.channels as u32) & SPECTRUM_CHANNEL_LEFT != 0 {
        match append_spectrum_f32(bytes, &mut cursor, &window.left) {
            Ok(offset) => offset,
            Err(result) => return spectrum_failure(staging, result),
        }
    } else {
        0
    };
    let right_offset = if (window.channels as u32) & SPECTRUM_CHANNEL_RIGHT != 0 {
        match append_spectrum_f32(bytes, &mut cursor, &window.right) {
            Ok(offset) => offset,
            Err(result) => return spectrum_failure(staging, result),
        }
    } else {
        0
    };
    if cursor > maximum {
        return spectrum_failure(staging, RESULT_REFUSED_BUDGET);
    }
    let header = WebSpectrumWindow {
        struct_size: SPECTRUM_WINDOW_HEADER_BYTES,
        abi_version: ABI_VERSION,
        target,
        channels: window.channels as u32,
        sample_rate_hz,
        frames: SPECTRUM_WINDOW_FRAMES,
        source_underrun: u32::from(window.source_underrun),
        reserved0: 0,
        captured_sample: window.first_sample,
        end_sample: window.end_sample().unwrap_or(0),
        snapshot_token: token,
        left_offset,
        right_offset,
    };
    if !copy_live_record(bytes, 0, &header) {
        return spectrum_failure(staging, RESULT_REFUSED_BUDGET);
    }
    staging.capture_len = cursor;
    RESULT_OK
}

fn spectrum_f32_plane(
    bytes: &[u8],
    offset: u32,
    count: u32,
    destination: &mut [f32],
) -> Result<(), u32> {
    let count = usize::try_from(count).map_err(|_| RESULT_INVALID_ARGUMENT)?;
    if count != destination.len() {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    let start = usize::try_from(offset).map_err(|_| RESULT_INVALID_ARGUMENT)?;
    let byte_count = count
        .checked_mul(size_of::<f32>())
        .ok_or(RESULT_INVALID_ARGUMENT)?;
    let end = start
        .checked_add(byte_count)
        .ok_or(RESULT_INVALID_ARGUMENT)?;
    let source = bytes.get(start..end).ok_or(RESULT_INVALID_ARGUMENT)?;
    for (index, value) in destination.iter_mut().enumerate() {
        let at = index * size_of::<f32>();
        let raw = [source[at], source[at + 1], source[at + 2], source[at + 3]];
        *value = f32::from_le_bytes(raw);
    }
    Ok(())
}

fn run_spectrum_analysis(staging: &mut SpectrumStaging, continuous: bool) -> u32 {
    if (if continuous {
        staging.ensure_stream_analysis_storage()
    } else {
        staging.ensure_analysis_storage()
    })
    .is_err()
    {
        return spectrum_failure(staging, RESULT_REFUSED_BUDGET);
    }
    let bytes = match staging
        .capture
        .as_ref()
        .and_then(|capture| capture.get(..staging.capture_len))
    {
        Some(bytes) => bytes,
        None => return spectrum_failure(staging, RESULT_INVALID_ARGUMENT),
    };
    let header: WebSpectrumWindow = match read_live_record(bytes, 0) {
        Ok(value) => value,
        Err(result) => return spectrum_failure(staging, result),
    };
    if header.struct_size != SPECTRUM_WINDOW_HEADER_BYTES
        || header.abi_version != ABI_VERSION
        || !(SPECTRUM_TARGET_TRACK_POST_INPUT_BUILTINS..=SPECTRUM_TARGET_OUTPUT)
            .contains(&header.target)
        || !(SPECTRUM_CHANNEL_LEFT..=SPECTRUM_CHANNEL_BOTH).contains(&header.channels)
        || header.frames != SPECTRUM_WINDOW_FRAMES
        || header.source_underrun > 1
        || header.reserved0 != 0
        || header.snapshot_token == 0
        || header.end_sample
            != header
                .captured_sample
                .saturating_add(u64::from(SPECTRUM_WINDOW_FRAMES))
        || staging.capture_len != spectrum_capture_bytes(header.channels)
    {
        return spectrum_failure(staging, RESULT_INVALID_ARGUMENT);
    }
    let mut window = SpectrumWindow {
        left: [0.0; host_core::SPECTRUM_WINDOW_FRAMES],
        right: [0.0; host_core::SPECTRUM_WINDOW_FRAMES],
        first_sample: header.captured_sample,
        channels: match spectrum_channels(header.channels) {
            Ok(value) => value,
            Err(result) => return spectrum_failure(staging, result),
        },
        source_underrun: header.source_underrun != 0,
    };
    let left = (header.channels & SPECTRUM_CHANNEL_LEFT) != 0;
    let right = (header.channels & SPECTRUM_CHANNEL_RIGHT) != 0;
    if left
        && spectrum_f32_plane(bytes, header.left_offset, header.frames, &mut window.left).is_err()
    {
        return spectrum_failure(staging, RESULT_INVALID_ARGUMENT);
    }
    if right
        && spectrum_f32_plane(bytes, header.right_offset, header.frames, &mut window.right).is_err()
    {
        return spectrum_failure(staging, RESULT_INVALID_ARGUMENT);
    }
    let mut frequencies = [0.0; host_core::SPECTRUM_BIN_COUNT];
    let mut left_dbfs = [0.0; host_core::SPECTRUM_BIN_COUNT];
    let mut right_dbfs = [0.0; host_core::SPECTRUM_BIN_COUNT];
    let mut output = host_core::SpectrumOutput {
        frequencies_hz: &mut frequencies,
        left_dbfs: left.then_some(&mut left_dbfs),
        right_dbfs: right.then_some(&mut right_dbfs),
    };
    let continuous_metadata = if continuous {
        let Some(cadence) = staging.stream_cadence else {
            return spectrum_failure(staging, RESULT_WRONG_STATE);
        };
        let Some(smoothing) = staging.stream_smoothing else {
            return spectrum_failure(staging, RESULT_WRONG_STATE);
        };
        let Some(facts) = staging.stream_window else {
            return spectrum_failure(staging, RESULT_INVALID_ARGUMENT);
        };
        let continuous_window = SpectrumContinuousWindow {
            left: window.left,
            right: window.right,
            first_sample: window.first_sample,
            channels: window.channels,
            source_underrun: window.source_underrun,
            stream_epoch: facts.stream_epoch,
            sequence: facts.sequence,
            dropped_captures: facts.dropped_captures,
        };
        let Some(history) = staging.stream_history.as_mut() else {
            return spectrum_failure(staging, RESULT_WRONG_STATE);
        };
        match staging
            .analyzer
            .as_ref()
            .expect("analysis storage initialized")
            .analyze_continuous(&continuous_window, cadence, smoothing, history, &mut output)
        {
            Ok(metadata) => Some(metadata),
            Err(_) => return spectrum_failure(staging, RESULT_INVALID_ARGUMENT),
        }
    } else {
        if staging
            .analyzer
            .as_ref()
            .expect("analysis storage initialized")
            .analyze(&window, header.sample_rate_hz, &mut output)
            .is_err()
        {
            return spectrum_failure(staging, RESULT_INVALID_ARGUMENT);
        }
        None
    };
    let mut cursor = SPECTRUM_RESULT_HEADER_BYTES as usize;
    let frequencies_offset = match append_spectrum_f32(
        staging
            .result
            .as_mut()
            .expect("analysis storage initialized"),
        &mut cursor,
        &frequencies,
    ) {
        Ok(offset) => offset,
        Err(result) => return spectrum_failure(staging, result),
    };
    let left_offset = if left {
        match append_spectrum_f32(
            staging
                .result
                .as_mut()
                .expect("analysis storage initialized"),
            &mut cursor,
            &left_dbfs,
        ) {
            Ok(offset) => offset,
            Err(result) => return spectrum_failure(staging, result),
        }
    } else {
        0
    };
    let right_offset = if right {
        match append_spectrum_f32(
            staging
                .result
                .as_mut()
                .expect("analysis storage initialized"),
            &mut cursor,
            &right_dbfs,
        ) {
            Ok(offset) => offset,
            Err(result) => return spectrum_failure(staging, result),
        }
    } else {
        0
    };
    let result_header = WebSpectrumResult {
        struct_size: SPECTRUM_RESULT_HEADER_BYTES,
        abi_version: ABI_VERSION,
        result: RESULT_OK,
        target: header.target,
        channels: header.channels,
        sample_rate_hz: header.sample_rate_hz,
        window_frames: header.frames,
        bin_count: host_core::SPECTRUM_BIN_COUNT as u32,
        source_underrun: header.source_underrun,
        floor_db: host_core::SPECTRUM_FLOOR_DB,
        captured_sample: header.captured_sample,
        end_sample: header.end_sample,
        snapshot_token: header.snapshot_token,
        result_bytes: u64::try_from(cursor).unwrap_or(u64::MAX),
        frequencies_offset,
        left_offset,
        right_offset,
        reserved0: 0,
    };
    if !copy_live_record(
        staging
            .result
            .as_mut()
            .expect("analysis storage initialized"),
        0,
        &result_header,
    ) {
        return spectrum_failure(staging, RESULT_REFUSED_BUDGET);
    }
    staging.result_len = cursor;
    if let Some(metadata) = continuous_metadata {
        staging.stream_metadata.result = RESULT_OK;
        staging.stream_metadata.status = SPECTRUM_STREAM_STATUS_READY;
        staging.stream_metadata.source_underrun = u32::from(metadata.source_underrun);
        staging.stream_metadata.capture_epoch = metadata.stream_epoch;
        staging.stream_metadata.sequence = metadata.sequence;
        staging.stream_metadata.captured_sample = metadata.fft_first_sample;
        staging.stream_metadata.end_sample = metadata.fft_end_sample;
        staging.stream_metadata.analysis_epoch = metadata.analysis_epoch;
        staging.stream_metadata.history_start_sample = metadata.history_start_sample;
        staging.stream_metadata.smoothing_ms = metadata.smoothing_ms;
    }
    RESULT_OK
}

fn live_response_grid(request: WebLiveResponseRequest) -> Result<ResponsePreviewGrid, u32> {
    let points = usize::try_from(request.points).map_err(|_| RESULT_INVALID_ARGUMENT)?;
    if !(2..=LIVE_RESPONSE_MAXIMUM_POINTS).contains(&points)
        || request.channels == 0
        || request.channels > crate::RESPONSE_CHANNEL_BOTH
        || !request.minimum_hz.is_finite()
        || !request.maximum_hz.is_finite()
    {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    match request.grid {
        crate::RESPONSE_GRID_LINEAR => Ok(ResponsePreviewGrid::Linear {
            points,
            minimum_hz: request.minimum_hz,
            maximum_hz: request.maximum_hz,
        }),
        crate::RESPONSE_GRID_LOGARITHMIC => Ok(ResponsePreviewGrid::Logarithmic {
            points,
            minimum_hz: request.minimum_hz,
            maximum_hz: request.maximum_hz,
        }),
        _ => Err(RESULT_INVALID_ARGUMENT),
    }
}

fn protected_live_response_grid(
    request: WebLiveResponseRequest,
    sample_rate_hz: u32,
) -> Result<ResponsePreviewGrid, u32> {
    let grid = live_response_grid(request)?;
    let nyquist_hz = sample_rate_hz as f32 * 0.5;
    if request.minimum_hz < 0.0
        || request.minimum_hz >= request.maximum_hz
        || request.maximum_hz > nyquist_hz
        || (request.grid == crate::RESPONSE_GRID_LOGARITHMIC && request.minimum_hz <= 0.0)
    {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    Ok(grid)
}

fn live_response_failure(staging: &mut ResponseStaging, result: u32) -> u32 {
    staging.live_result_header = WebLiveResponseResult {
        struct_size: LIVE_RESPONSE_RESULT_BYTES,
        abi_version: ABI_VERSION,
        result,
        mode: LIVE_RESPONSE_MODE_TARGET,
        meaning: LIVE_RESPONSE_MEANING_EQ_FILTER_SUBTOTAL,
        owner_record_bytes: LIVE_RESPONSE_OWNER_BYTES,
        section_record_bytes: LIVE_RESPONSE_SECTION_BYTES,
        result_bytes: LIVE_RESPONSE_RESULT_BYTES as u64,
        ..WebLiveResponseResult::default()
    };
    let copied = copy_live_record(&mut staging.live_result, 0, &staging.live_result_header);
    debug_assert!(copied);
    staging.live_result_len = LIVE_RESPONSE_RESULT_BYTES as usize;
    result
}

struct LiveResponseCaptureSink<'a> {
    bytes: &'a mut [u8],
    next: usize,
    owner_count: usize,
    excluded_count: usize,
}

impl<'a> LiveResponseCaptureSink<'a> {
    fn new(bytes: &'a mut [u8]) -> Result<Self, ResponseSnapshotError> {
        let owner_bytes = LIVE_RESPONSE_MAXIMUM_OWNERS
            .checked_mul(size_of::<WebLiveResponseOwner>())
            .ok_or(ResponseSnapshotError::Capacity)?;
        let next = size_of::<WebLiveResponseResult>()
            .checked_add(owner_bytes)
            .ok_or(ResponseSnapshotError::Capacity)?;
        if next > bytes.len() {
            return Err(ResponseSnapshotError::Capacity);
        }
        Ok(Self {
            bytes,
            next,
            owner_count: 0,
            excluded_count: 0,
        })
    }

    fn append(&mut self, source: &[u8]) -> Result<u32, ResponseSnapshotError> {
        let end = self
            .next
            .checked_add(source.len())
            .ok_or(ResponseSnapshotError::Capacity)?;
        if end > self.bytes.len() {
            return Err(ResponseSnapshotError::Capacity);
        }
        let offset = u32::try_from(self.next).map_err(|_| ResponseSnapshotError::Capacity)?;
        self.bytes[self.next..end].copy_from_slice(source);
        self.next = end;
        Ok(offset)
    }

    fn append_section(
        &mut self,
        section: ResponseSnapshotSection,
    ) -> Result<u32, ResponseSnapshotError> {
        let record = WebLiveResponseSection {
            id: section.id,
            kind: section.kind,
            enabled: u32::from(section.enabled),
            word_count: u32::from(section.word_count),
            words: section.words,
        };
        let offset = self.next;
        if !copy_live_record(self.bytes, offset, &record) {
            return Err(ResponseSnapshotError::Capacity);
        }
        self.next = offset
            .checked_add(size_of::<WebLiveResponseSection>())
            .ok_or(ResponseSnapshotError::Capacity)?;
        u32::try_from(offset).map_err(|_| ResponseSnapshotError::Capacity)
    }

    fn write_owner(
        &mut self,
        index: usize,
        owner: WebLiveResponseOwner,
    ) -> Result<(), ResponseSnapshotError> {
        let offset = size_of::<WebLiveResponseResult>()
            .checked_add(
                index
                    .checked_mul(size_of::<WebLiveResponseOwner>())
                    .ok_or(ResponseSnapshotError::Capacity)?,
            )
            .ok_or(ResponseSnapshotError::Capacity)?;
        if !copy_live_record(self.bytes, offset, &owner) {
            return Err(ResponseSnapshotError::Capacity);
        }
        Ok(())
    }
}

impl ResponseSnapshotSink for LiveResponseCaptureSink<'_> {
    fn copy_owner(
        &mut self,
        owner: ResponseSnapshotOwnerInfo<'_>,
        left: &[ResponseSnapshotSection],
        right: &[ResponseSnapshotSection],
    ) -> Result<(), ResponseSnapshotError> {
        #[cfg(test)]
        {
            LIVE_RESPONSE_CAPTURE_OWNER_CALLS.with(|calls| {
                calls.set(calls.get().saturating_add(1));
            });
            if let Some(error) = LIVE_RESPONSE_CAPTURE_FAULT.with(Cell::take) {
                return Err(error);
            }
        }
        if self.owner_count >= LIVE_RESPONSE_MAXIMUM_OWNERS
            || left.len() > LIVE_RESPONSE_MAXIMUM_SECTIONS
            || right.len() > LIVE_RESPONSE_MAXIMUM_SECTIONS
            || owner.track_id.len() > LIVE_RESPONSE_MAXIMUM_ID_BYTES
            || owner.native_id.len() > LIVE_RESPONSE_MAXIMUM_ID_BYTES
            || owner.stable_id.len() > LIVE_RESPONSE_MAXIMUM_ID_BYTES
        {
            return Err(ResponseSnapshotError::Capacity);
        }
        let track_id_offset = self.append(owner.track_id.as_bytes())?;
        let native_id_offset = self.append(owner.native_id.as_bytes())?;
        let stable_id_offset = self.append(owner.stable_id.as_bytes())?;
        let left_offset = self.next;
        for section in left {
            self.append_section(*section)?;
        }
        let right_offset = self.next;
        for section in right {
            self.append_section(*section)?;
        }
        let record = WebLiveResponseOwner {
            track_id_offset,
            track_id_bytes: u32::try_from(owner.track_id.len())
                .map_err(|_| ResponseSnapshotError::Capacity)?,
            native_id_offset,
            native_id_bytes: u32::try_from(owner.native_id.len())
                .map_err(|_| ResponseSnapshotError::Capacity)?,
            stable_id_offset,
            stable_id_bytes: u32::try_from(owner.stable_id.len())
                .map_err(|_| ResponseSnapshotError::Capacity)?,
            rack: u32::from(owner.rack),
            slot: owner.slot,
            kind: owner.kind,
            bypassed: u32::from(owner.bypassed),
            availability: u32::from(owner.availability == ResponseSnapshotAvailability::Provided),
            left_offset: u32::try_from(left_offset).map_err(|_| ResponseSnapshotError::Capacity)?,
            left_count: u32::try_from(left.len()).map_err(|_| ResponseSnapshotError::Capacity)?,
            right_offset: u32::try_from(right_offset)
                .map_err(|_| ResponseSnapshotError::Capacity)?,
            right_count: u32::try_from(right.len()).map_err(|_| ResponseSnapshotError::Capacity)?,
            reserved: [0; 1],
        };
        self.write_owner(self.owner_count, record)?;
        self.owner_count += 1;
        if owner.availability == ResponseSnapshotAvailability::DeclaredUnavailable {
            self.excluded_count += 1;
        }
        Ok(())
    }
}

/// Capture one protected selected track through the shared ordinary ingress permit.
///
/// The protected path is deliberately separate from the legacy compatibility path below. Every
/// refusal before the admitted native call leaves the previously committed response payload and
/// identity untouched; only an admitted provider/sink failure uses the legacy failure header.
fn run_protected_live_response_capture(
    host: &mut AudioWorkletEngineHost,
    staging: &mut ResponseStaging,
) -> u32 {
    const OPERATION: u32 = OBSERVATION_OPERATION_CAPTURE_RESPONSE;
    // Copy fixed scalars before any request interpretation. The ordinary attempt is classified
    // from these declared lengths, so an oversized request spends the attempt before refusal.
    let request = *staging.live_request;
    let live_token = staging.live_token;
    let control_bytes = match u64::try_from(size_of::<WebLiveResponseRequest>())
        .ok()
        .and_then(|header| header.checked_add(u64::from(request.track_id_bytes)))
    {
        Some(value) => value,
        None => {
            let refusal = ObservationRefusal {
                reason: ObservationRefusalReason::ArithmeticOverflow,
                limit: None,
                requested: None,
                maximum: None,
            };
            return host.record_observation_refusal(OPERATION, refusal);
        }
    };
    let permit = match host.begin_observation(
        ObservationClass::Ordinary,
        ObservationLengths {
            control_bytes,
            rows: 1,
            result_bytes: u64::from(request.maximum_result_bytes),
        },
    ) {
        Ok(permit) => permit,
        Err(refusal) => return host.record_observation_refusal(OPERATION, refusal),
    };

    // Token exhaustion and every shape/target/capacity check below are pre-provider refusals. The
    // permit has already spent this boundary's Ordinary attempt, but no response staging is
    // touched and no capture identity is replaced.
    let sequence = match live_token.checked_add(1) {
        Some(value) => value,
        None => {
            host.record_observation_admission(OPERATION, RESULT_REFUSED_BUDGET, None, None, false);
            return RESULT_REFUSED_BUDGET;
        }
    };
    let track_id_bytes = match usize::try_from(request.track_id_bytes) {
        Ok(value) => value,
        Err(_) => {
            host.record_observation_admission(
                OPERATION,
                RESULT_INVALID_ARGUMENT,
                None,
                None,
                false,
            );
            return RESULT_INVALID_ARGUMENT;
        }
    };
    let maximum_result_bytes = match usize::try_from(request.maximum_result_bytes) {
        Ok(value) => value,
        Err(_) => {
            host.record_observation_admission(
                OPERATION,
                RESULT_INVALID_ARGUMENT,
                None,
                None,
                false,
            );
            return RESULT_INVALID_ARGUMENT;
        }
    };
    if request.struct_size != LIVE_RESPONSE_REQUEST_BYTES
        || request.abi_version != ABI_VERSION
        || request.reserved != [0; 3]
        || track_id_bytes == 0
        || track_id_bytes > staging.live_track_id.len()
        || request.maximum_result_bytes == 0
        || maximum_result_bytes > staging.live_result.len()
    {
        host.record_observation_admission(OPERATION, RESULT_INVALID_ARGUMENT, None, None, false);
        return RESULT_INVALID_ARGUMENT;
    }
    let sample_rate_hz = host.status().sample_rate_hz;
    if let Err(result) = protected_live_response_grid(request, sample_rate_hz) {
        host.record_observation_admission(OPERATION, result, None, None, false);
        return result;
    }
    let track_id = match core::str::from_utf8(&staging.live_track_id[..track_id_bytes]) {
        Ok(value) if !value.is_empty() && value.len() <= LIVE_RESPONSE_MAXIMUM_ID_BYTES => value,
        _ => {
            host.record_observation_admission(
                OPERATION,
                RESULT_INVALID_ARGUMENT,
                None,
                None,
                false,
            );
            return RESULT_INVALID_ARGUMENT;
        }
    };

    let packed_bound = match host.response_capture_preflight(track_id) {
        Ok(value) => value,
        Err(error) => {
            let result = live_response_capture_error_code(error);
            host.record_observation_admission(OPERATION, result, None, None, false);
            return result;
        }
    };
    let actual_result_bytes = match u64::try_from(staging.live_result.len()) {
        Ok(value) => value,
        Err(_) => {
            host.record_observation_admission(OPERATION, RESULT_REFUSED_BUDGET, None, None, false);
            return RESULT_REFUSED_BUDGET;
        }
    };
    if u64::from(request.maximum_result_bytes) < packed_bound
        || actual_result_bytes < packed_bound
        || u64::from(request.maximum_result_bytes) > actual_result_bytes
    {
        host.record_observation_admission(OPERATION, RESULT_REFUSED_BUDGET, None, None, false);
        return RESULT_REFUSED_BUDGET;
    }
    let owner = match host.protected_observation_identity() {
        Ok((owner, _epoch)) => owner,
        Err(refusal) => return host.record_observation_refusal(OPERATION, refusal),
    };

    // All known fixed capacity and target/state checks are complete before constructing the sink.
    // The affine permit is moved exactly once into the typed admitted seam.
    let captured = {
        let mut sink =
            match LiveResponseCaptureSink::new(&mut staging.live_result[..maximum_result_bytes]) {
                Ok(value) => value,
                Err(error) => {
                    let result = live_response_capture_error_code(error);
                    host.record_observation_admission(OPERATION, result, None, None, false);
                    return result;
                }
            };
        let result = host.copy_response_snapshot_admitted(permit, track_id, &mut sink);
        match result {
            Ok(capture) => Ok((capture, sink.owner_count, sink.excluded_count, sink.next)),
            Err(error) => Err(error),
        }
    };
    let (capture, owner_count, excluded_count, result_bytes) = match captured {
        Ok(value) => value,
        Err(ProtectedResponseCaptureError::Refused(refusal)) => {
            return live_response_capture_error_code(legacy_response_refusal(refusal));
        }
        Err(ProtectedResponseCaptureError::Capture(error)) => {
            let result = live_response_capture_error_code(error);
            return live_response_failure(staging, result);
        }
    };
    if result_bytes > maximum_result_bytes {
        return live_response_failure(staging, RESULT_REFUSED_BUDGET);
    }

    let owners_offset = size_of::<WebLiveResponseResult>() as u32;
    staging.live_result_header = WebLiveResponseResult {
        struct_size: LIVE_RESPONSE_RESULT_BYTES,
        abi_version: ABI_VERSION,
        result: RESULT_OK,
        mode: LIVE_RESPONSE_MODE_TARGET,
        meaning: LIVE_RESPONSE_MEANING_EQ_FILTER_SUBTOTAL,
        channels: request.channels,
        points: 0,
        owner_count: u32::try_from(owner_count).unwrap_or(0),
        excluded_count: u32::try_from(excluded_count).unwrap_or(0),
        sample_rate_hz: host.status().sample_rate_hz,
        reserved0: 0,
        reserved1: 0,
        captured_sample: capture.captured_sample,
        snapshot_token: sequence,
        result_bytes: u64::try_from(result_bytes).unwrap_or(u64::MAX),
        frequencies_offset: 0,
        left_offset: 0,
        right_offset: 0,
        owners_offset,
        owner_record_bytes: LIVE_RESPONSE_OWNER_BYTES,
        section_record_bytes: LIVE_RESPONSE_SECTION_BYTES,
        reserved: [0; 2],
    };
    if !copy_live_record(&mut staging.live_result, 0, &staging.live_result_header) {
        return live_response_failure(staging, RESULT_REFUSED_BUDGET);
    }

    // The raw payload is complete before the three committed publication scalars become visible.
    staging.live_result_len = result_bytes;
    staging.live_token = sequence;
    host.commit_observation_capture_identity(WebObservationCaptureIdentity {
        struct_size: size_of::<WebObservationCaptureIdentity>() as u32,
        abi_version: ABI_VERSION,
        kind: OBSERVATION_CAPTURE_KIND_RESPONSE,
        flags: 0,
        owner: owner.get(),
        observation_generation: 0,
        selection_epoch: 0,
        snapshot_token: sequence,
    });
    RESULT_OK
}

fn run_live_response_capture(
    host: &mut AudioWorkletEngineHost,
    staging: &mut ResponseStaging,
) -> u32 {
    let request = *staging.live_request;
    if request.struct_size != LIVE_RESPONSE_REQUEST_BYTES
        || request.abi_version != ABI_VERSION
        || request.reserved != [0; 3]
        || request.track_id_bytes == 0
        || request.track_id_bytes as usize > staging.live_track_id.len()
        || request.maximum_result_bytes == 0
        || usize::try_from(request.maximum_result_bytes)
            .map_or(true, |value| value > LIVE_RESPONSE_CAPTURE_BYTES)
    {
        return live_response_failure(staging, RESULT_INVALID_ARGUMENT);
    }
    if let Err(result) = live_response_grid(request) {
        return live_response_failure(staging, result);
    }
    let track_id =
        match core::str::from_utf8(&staging.live_track_id[..request.track_id_bytes as usize]) {
            Ok(value) => value,
            Err(_) => return live_response_failure(staging, RESULT_INVALID_ARGUMENT),
        };
    let maximum_result_bytes = usize::try_from(request.maximum_result_bytes)
        .expect("validated live response result bound");
    let captured = {
        let mut sink =
            match LiveResponseCaptureSink::new(&mut staging.live_result[..maximum_result_bytes]) {
                Ok(value) => value,
                Err(error) => {
                    return live_response_failure(staging, live_response_capture_error_code(error));
                }
            };
        match host.copy_response_snapshot(track_id, &mut sink) {
            Ok(value) => Ok((value, sink.owner_count, sink.excluded_count, sink.next)),
            Err(error) => Err(live_response_capture_error_code(error)),
        }
    };
    let (capture, owner_count, excluded_count, result_bytes) = match captured {
        Ok(value) => value,
        Err(result) => return live_response_failure(staging, result),
    };
    let Some(sequence) = staging.live_token.checked_add(1) else {
        return live_response_failure(staging, RESULT_REFUSED_BUDGET);
    };
    if result_bytes > maximum_result_bytes {
        return live_response_failure(staging, RESULT_REFUSED_BUDGET);
    }
    staging.live_token = sequence;
    let owners_offset = size_of::<WebLiveResponseResult>() as u32;
    staging.live_result_header = WebLiveResponseResult {
        struct_size: LIVE_RESPONSE_RESULT_BYTES,
        abi_version: ABI_VERSION,
        result: RESULT_OK,
        mode: LIVE_RESPONSE_MODE_TARGET,
        meaning: LIVE_RESPONSE_MEANING_EQ_FILTER_SUBTOTAL,
        channels: request.channels,
        points: 0,
        owner_count: u32::try_from(owner_count).unwrap_or(0),
        excluded_count: u32::try_from(excluded_count).unwrap_or(0),
        sample_rate_hz: host.status().sample_rate_hz,
        reserved0: 0,
        reserved1: 0,
        captured_sample: capture.captured_sample,
        snapshot_token: sequence,
        result_bytes: u64::try_from(result_bytes).unwrap_or(u64::MAX),
        frequencies_offset: 0,
        left_offset: 0,
        right_offset: 0,
        owners_offset,
        owner_record_bytes: LIVE_RESPONSE_OWNER_BYTES,
        section_record_bytes: LIVE_RESPONSE_SECTION_BYTES,
        reserved: [0; 2],
    };
    let copied = copy_live_record(&mut staging.live_result, 0, &staging.live_result_header);
    debug_assert!(copied);
    staging.live_result_len = result_bytes;
    RESULT_OK
}

fn read_live_record<T: Copy>(bytes: &[u8], offset: u32) -> Result<T, u32> {
    let start = usize::try_from(offset).map_err(|_| RESULT_INVALID_ARGUMENT)?;
    let end = start
        .checked_add(size_of::<T>())
        .ok_or(RESULT_INVALID_ARGUMENT)?;
    let source = bytes.get(start..end).ok_or(RESULT_INVALID_ARGUMENT)?;
    // SAFETY: The source range is checked against the payload and `read_unaligned` accepts the
    // byte alignment of a Wasm result payload.
    Ok(unsafe { ptr::read_unaligned(source.as_ptr().cast::<T>()) })
}

fn live_payload_bytes(bytes: &[u8], offset: u32, count: u32) -> Result<&[u8], u32> {
    let start = usize::try_from(offset).map_err(|_| RESULT_INVALID_ARGUMENT)?;
    let length = usize::try_from(count).map_err(|_| RESULT_INVALID_ARGUMENT)?;
    let end = start.checked_add(length).ok_or(RESULT_INVALID_ARGUMENT)?;
    bytes.get(start..end).ok_or(RESULT_INVALID_ARGUMENT)
}

fn parse_live_string(bytes: &[u8], offset: u32, count: u32) -> Result<Box<str>, u32> {
    let value = core::str::from_utf8(live_payload_bytes(bytes, offset, count)?)
        .map_err(|_| RESULT_INVALID_ARGUMENT)?;
    if value.is_empty() || value.len() > LIVE_RESPONSE_MAXIMUM_ID_BYTES {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    Ok(value.into())
}

fn parse_live_sections(
    bytes: &[u8],
    offset: u32,
    count: u32,
    record_bytes: u32,
) -> Result<Box<[ResponseSnapshotSection]>, u32> {
    if count > LIVE_RESPONSE_MAXIMUM_SECTIONS as u32 || record_bytes != LIVE_RESPONSE_SECTION_BYTES
    {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    let count_usize = usize::try_from(count).map_err(|_| RESULT_INVALID_ARGUMENT)?;
    let stride = usize::try_from(record_bytes).map_err(|_| RESULT_INVALID_ARGUMENT)?;
    let start = usize::try_from(offset).map_err(|_| RESULT_INVALID_ARGUMENT)?;
    let total = count_usize
        .checked_mul(stride)
        .ok_or(RESULT_INVALID_ARGUMENT)?;
    let end = start.checked_add(total).ok_or(RESULT_INVALID_ARGUMENT)?;
    if end > bytes.len() {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    let mut sections = Vec::with_capacity(count_usize);
    for index in 0..count_usize {
        let section_offset = u32::try_from(
            start
                .checked_add(index.checked_mul(stride).ok_or(RESULT_INVALID_ARGUMENT)?)
                .ok_or(RESULT_INVALID_ARGUMENT)?,
        )
        .map_err(|_| RESULT_INVALID_ARGUMENT)?;
        let raw: WebLiveResponseSection = read_live_record(bytes, section_offset)?;
        if raw.enabled > 1
            || raw.word_count > 7
            || raw.words[usize::try_from(raw.word_count).unwrap_or(8)..]
                .iter()
                .any(|word| *word != 0)
        {
            return Err(RESULT_INVALID_ARGUMENT);
        }
        sections.push(ResponseSnapshotSection {
            id: raw.id,
            kind: raw.kind,
            enabled: raw.enabled != 0,
            word_count: u8::try_from(raw.word_count).map_err(|_| RESULT_INVALID_ARGUMENT)?,
            words: raw.words,
        });
    }
    Ok(sections.into_boxed_slice())
}

fn parse_live_snapshot(staging: &ResponseStaging) -> Result<(ResponseSnapshot, u64), u32> {
    let bytes = staging
        .live_result
        .get(..staging.live_result_len)
        .ok_or(RESULT_INVALID_ARGUMENT)?;
    let header: WebLiveResponseResult = read_live_record(bytes, 0)?;
    if header.struct_size != LIVE_RESPONSE_RESULT_BYTES
        || header.abi_version != ABI_VERSION
        || header.result != RESULT_OK
        || header.mode != LIVE_RESPONSE_MODE_TARGET
        || header.meaning != LIVE_RESPONSE_MEANING_EQ_FILTER_SUBTOTAL
        || header.reserved0 != 0
        || header.reserved1 != 0
        || header.reserved != [0; 2]
        || header.snapshot_token == 0
        || header.points != 0
        || header.owner_record_bytes != LIVE_RESPONSE_OWNER_BYTES
        || header.section_record_bytes != LIVE_RESPONSE_SECTION_BYTES
        || header.result_bytes != staging.live_result_len as u64
        || header.owner_count as usize > LIVE_RESPONSE_MAXIMUM_OWNERS
    {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    let owner_count = usize::try_from(header.owner_count).map_err(|_| RESULT_INVALID_ARGUMENT)?;
    let owners_start =
        usize::try_from(header.owners_offset).map_err(|_| RESULT_INVALID_ARGUMENT)?;
    let owner_stride = size_of::<WebLiveResponseOwner>();
    let owner_end = owners_start
        .checked_add(
            owner_count
                .checked_mul(owner_stride)
                .ok_or(RESULT_INVALID_ARGUMENT)?,
        )
        .ok_or(RESULT_INVALID_ARGUMENT)?;
    if owner_end > bytes.len() || owners_start < size_of::<WebLiveResponseResult>() {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    let mut owners = Vec::with_capacity(owner_count);
    let mut excluded = 0_u32;
    for index in 0..owner_count {
        let offset = u32::try_from(
            owners_start
                .checked_add(
                    index
                        .checked_mul(owner_stride)
                        .ok_or(RESULT_INVALID_ARGUMENT)?,
                )
                .ok_or(RESULT_INVALID_ARGUMENT)?,
        )
        .map_err(|_| RESULT_INVALID_ARGUMENT)?;
        let raw: WebLiveResponseOwner = read_live_record(bytes, offset)?;
        if raw.bypassed > 1
            || raw.availability > 1
            || raw.reserved != [0; 1]
            || raw.left_count > LIVE_RESPONSE_MAXIMUM_SECTIONS as u32
            || raw.right_count > LIVE_RESPONSE_MAXIMUM_SECTIONS as u32
        {
            return Err(RESULT_INVALID_ARGUMENT);
        }
        let track_id = parse_live_string(bytes, raw.track_id_offset, raw.track_id_bytes)?;
        let native_id = parse_live_string(bytes, raw.native_id_offset, raw.native_id_bytes)?;
        let stable_id = parse_live_string(bytes, raw.stable_id_offset, raw.stable_id_bytes)?;
        let left = parse_live_sections(
            bytes,
            raw.left_offset,
            raw.left_count,
            header.section_record_bytes,
        )?;
        let right = parse_live_sections(
            bytes,
            raw.right_offset,
            raw.right_count,
            header.section_record_bytes,
        )?;
        if raw.availability == 1 && (left.is_empty() || right.is_empty()) {
            return Err(RESULT_INVALID_ARGUMENT);
        }
        if raw.availability == 0 {
            excluded = excluded.checked_add(1).ok_or(RESULT_INVALID_ARGUMENT)?;
        }
        owners.push(ResponseSnapshotOwner {
            track_id,
            native_id,
            stable_id,
            rack: u8::try_from(raw.rack).map_err(|_| RESULT_INVALID_ARGUMENT)?,
            slot: raw.slot,
            kind: raw.kind,
            bypassed: raw.bypassed != 0,
            availability: if raw.availability == 0 {
                ResponseSnapshotAvailability::DeclaredUnavailable
            } else {
                ResponseSnapshotAvailability::Provided
            },
            left,
            right,
        });
    }
    if excluded != header.excluded_count {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    Ok((
        ResponseSnapshot {
            sample_rate_hz: header.sample_rate_hz,
            captured_sample: header.captured_sample,
            owners: owners.into_boxed_slice(),
        },
        header.snapshot_token,
    ))
}

fn append_live_f32(
    bytes: &mut [u8],
    cursor: &mut usize,
    values: &[f32],
    maximum: usize,
) -> Result<u32, u32> {
    let byte_count = values
        .len()
        .checked_mul(size_of::<f32>())
        .ok_or(RESULT_REFUSED_BUDGET)?;
    let end = cursor
        .checked_add(byte_count)
        .ok_or(RESULT_REFUSED_BUDGET)?;
    if end > maximum || end > bytes.len() {
        return Err(RESULT_REFUSED_BUDGET);
    }
    let offset = u32::try_from(*cursor).map_err(|_| RESULT_REFUSED_BUDGET)?;
    for (index, value) in values.iter().copied().enumerate() {
        let at = *cursor + index * size_of::<f32>();
        bytes[at..at + size_of::<f32>()].copy_from_slice(&value.to_le_bytes());
    }
    *cursor = end;
    Ok(offset)
}

fn run_live_response_analysis(staging: &mut ResponseStaging) -> u32 {
    let request = *staging.live_request;
    if request.struct_size != LIVE_RESPONSE_REQUEST_BYTES
        || request.abi_version != ABI_VERSION
        || request.reserved != [0; 3]
        || request.maximum_result_bytes == 0
        || usize::try_from(request.maximum_result_bytes)
            .map_or(true, |value| value > LIVE_RESPONSE_CAPTURE_BYTES)
    {
        return live_response_failure(staging, RESULT_INVALID_ARGUMENT);
    }
    let grid = match live_response_grid(request) {
        Ok(value) => value,
        Err(result) => return live_response_failure(staging, result),
    };
    let points = grid.points();
    let selected_channels = usize::from(request.channels & crate::RESPONSE_CHANNEL_LEFT != 0)
        + usize::from(request.channels & crate::RESPONSE_CHANNEL_RIGHT != 0);
    let Some(required_result_bytes) = points
        .checked_mul(size_of::<f32>())
        .and_then(|vector_bytes| vector_bytes.checked_mul(selected_channels + 1))
        .and_then(|vectors| size_of::<WebLiveResponseResult>().checked_add(vectors))
    else {
        return live_response_failure(staging, RESULT_REFUSED_BUDGET);
    };
    let maximum_result_bytes = usize::try_from(request.maximum_result_bytes)
        .expect("validated live response result bound");
    if required_result_bytes > maximum_result_bytes {
        return live_response_failure(staging, RESULT_REFUSED_BUDGET);
    }
    let (snapshot, snapshot_token) = match parse_live_snapshot(staging) {
        Ok(value) => value,
        Err(result) => return live_response_failure(staging, result),
    };
    let mut frequencies = vec![0.0_f32; points];
    let mut left = vec![0.0_f32; points];
    let mut right = vec![0.0_f32; points];
    let summary = match query_response_snapshot_into(
        &snapshot,
        grid,
        ResponseSnapshotOutput {
            frequencies_hz: &mut frequencies,
            total_left_db: &mut left,
            total_right_db: &mut right,
        },
    ) {
        Ok(value) => value,
        Err(error) => return live_response_failure(staging, live_response_error_code(error)),
    };
    let maximum = request.maximum_result_bytes as usize;
    let mut cursor = size_of::<WebLiveResponseResult>();
    let frequencies_offset =
        match append_live_f32(&mut staging.live_result, &mut cursor, &frequencies, maximum) {
            Ok(value) => value,
            Err(result) => return live_response_failure(staging, result),
        };
    let left_offset = if request.channels & crate::RESPONSE_CHANNEL_LEFT != 0 {
        match append_live_f32(&mut staging.live_result, &mut cursor, &left, maximum) {
            Ok(value) => value,
            Err(result) => return live_response_failure(staging, result),
        }
    } else {
        0
    };
    let right_offset = if request.channels & crate::RESPONSE_CHANNEL_RIGHT != 0 {
        match append_live_f32(&mut staging.live_result, &mut cursor, &right, maximum) {
            Ok(value) => value,
            Err(result) => return live_response_failure(staging, result),
        }
    } else {
        0
    };
    staging.live_result_header = WebLiveResponseResult {
        struct_size: LIVE_RESPONSE_RESULT_BYTES,
        abi_version: ABI_VERSION,
        result: RESULT_OK,
        mode: LIVE_RESPONSE_MODE_TARGET,
        meaning: LIVE_RESPONSE_MEANING_EQ_FILTER_SUBTOTAL,
        channels: request.channels,
        points: u32::try_from(points).unwrap_or(0),
        owner_count: summary.owners,
        excluded_count: summary.excluded_owners,
        sample_rate_hz: summary.sample_rate_hz,
        reserved0: 0,
        reserved1: 0,
        captured_sample: summary.captured_sample,
        snapshot_token,
        result_bytes: u64::try_from(cursor).unwrap_or(u64::MAX),
        frequencies_offset,
        left_offset,
        right_offset,
        owners_offset: 0,
        owner_record_bytes: LIVE_RESPONSE_OWNER_BYTES,
        section_record_bytes: LIVE_RESPONSE_SECTION_BYTES,
        reserved: [0; 2],
    };
    let copied = copy_live_record(&mut staging.live_result, 0, &staging.live_result_header);
    debug_assert!(copied);
    staging.live_result_len = cursor;
    RESULT_OK
}

fn response_buffer_budget(
    points: usize,
    section_count: usize,
    channels: u32,
    fields: u32,
    maximum: usize,
) -> Result<(), u32> {
    let frequency_bytes = points
        .checked_mul(size_of::<f32>())
        .ok_or(RESULT_REFUSED_BUDGET)?;
    let section_points = section_count
        .checked_mul(points)
        .ok_or(RESULT_REFUSED_BUDGET)?;
    let section_bytes = section_points
        .checked_mul(size_of::<f32>())
        .ok_or(RESULT_REFUSED_BUDGET)?;
    let working_bytes = frequency_bytes
        .checked_add(
            frequency_bytes
                .checked_mul(2)
                .ok_or(RESULT_REFUSED_BUDGET)?,
        )
        .and_then(|bytes| {
            if fields & crate::RESPONSE_FIELD_SECTIONS != 0 {
                bytes.checked_add(section_bytes.checked_mul(2)?)
            } else {
                Some(bytes)
            }
        })
        .ok_or(RESULT_REFUSED_BUDGET)?;
    let mut result_bytes = size_of::<WebResponseResult>()
        .checked_add(frequency_bytes)
        .ok_or(RESULT_REFUSED_BUDGET)?;
    if channels & crate::RESPONSE_CHANNEL_LEFT != 0 {
        result_bytes = result_bytes
            .checked_add(frequency_bytes)
            .ok_or(RESULT_REFUSED_BUDGET)?;
    }
    if channels & crate::RESPONSE_CHANNEL_RIGHT != 0 {
        result_bytes = result_bytes
            .checked_add(frequency_bytes)
            .ok_or(RESULT_REFUSED_BUDGET)?;
    }
    if fields & crate::RESPONSE_FIELD_SECTIONS != 0 {
        if channels & crate::RESPONSE_CHANNEL_LEFT != 0 {
            result_bytes = result_bytes
                .checked_add(section_bytes)
                .ok_or(RESULT_REFUSED_BUDGET)?;
        }
        if channels & crate::RESPONSE_CHANNEL_RIGHT != 0 {
            result_bytes = result_bytes
                .checked_add(section_bytes)
                .ok_or(RESULT_REFUSED_BUDGET)?;
        }
    }
    if result_bytes > maximum || working_bytes > maximum {
        return Err(RESULT_REFUSED_BUDGET);
    }
    Ok(())
}

fn response_failure(staging: &mut ResponseStaging, result: u32) -> u32 {
    staging.result_header = WebResponseResult {
        struct_size: RESPONSE_RESULT_BYTES,
        abi_version: ABI_VERSION,
        result,
        ..WebResponseResult::default()
    };
    staging.result = response_header_bytes(staging.result_header);
    result
}

fn run_response_query(staging: &mut ResponseStaging) -> u32 {
    let request = *staging.request;
    if request.struct_size != RESPONSE_REQUEST_BYTES
        || request.abi_version != ABI_VERSION
        || request.reserved != [0; 2]
        || request.channels == 0
        || request.channels & !crate::RESPONSE_CHANNEL_BOTH != 0
        || request.fields == 0
        || request.fields & !(crate::RESPONSE_FIELD_TOTAL | crate::RESPONSE_FIELD_SECTIONS) != 0
        || request.parameter_count > RESPONSE_MAXIMUM_PARAMETER_OVERRIDES
        || request.effect_id_bytes > RESPONSE_MAXIMUM_EFFECT_ID_BYTES
        || request.maximum_result_bytes == 0
        || u64::from(request.maximum_result_bytes) > RESPONSE_MAXIMUM_RESULT_BYTES
    {
        return response_failure(staging, RESULT_INVALID_ARGUMENT);
    }
    let grid = match response_grid(request) {
        Ok(value) => value,
        Err(result) => return response_failure(staging, result),
    };
    let parameter_count = request.parameter_count as usize;
    let mut overrides = Vec::with_capacity(parameter_count);
    for row in &staging.parameters[..parameter_count] {
        let channel = match response_channel(row.channel) {
            Ok(value) => value,
            Err(result) => return response_failure(staging, result),
        };
        overrides.push(ResponseParameterOverride {
            parameter_id: row.parameter_id,
            channel,
            value: row.value,
        });
    }
    let target = match request.target {
        crate::RESPONSE_TARGET_EFFECT => {
            let id_bytes = &staging.effect_id[..request.effect_id_bytes as usize];
            let Ok(effect_id) = core::str::from_utf8(id_bytes) else {
                return response_failure(staging, RESULT_INVALID_ARGUMENT);
            };
            let quality = match response_quality(request.quality) {
                Ok(value) => value,
                Err(result) => return response_failure(staging, result),
            };
            let link_mode = match response_link_mode(request.link_mode) {
                Ok(value) => value,
                Err(result) => return response_failure(staging, result),
            };
            ResponsePreviewTarget::Effect {
                effect_id,
                overrides: &overrides,
                quality,
                bypass: request.bypass != 0,
                link_mode,
            }
        }
        crate::RESPONSE_TARGET_INPUT_FILTERS => {
            if request.parameter_count != 0 || request.effect_id_bytes != 0 {
                return response_failure(staging, RESULT_INVALID_ARGUMENT);
            }
            ResponsePreviewTarget::InputFilters {
                left_hpf_hz: request.left_hpf_hz,
                left_lpf_hz: request.left_lpf_hz,
                right_hpf_hz: request.right_hpf_hz,
                right_lpf_hz: request.right_lpf_hz,
            }
        }
        _ => return response_failure(staging, RESULT_INVALID_ARGUMENT),
    };
    let limits = ResponsePreviewLimits {
        maximum_prepared_bytes: match usize::try_from(request.maximum_prepared_bytes) {
            Ok(value) if value != 0 => value,
            _ => return response_failure(staging, RESULT_REFUSED_BUDGET),
        },
        maximum_total_state_bytes: request.maximum_total_state_bytes,
        maximum_scratch_bytes: request.maximum_scratch_bytes,
        maximum_automation_spans_per_block: request.maximum_automation_spans_per_block,
    };
    let prepared = match prepare_response_preview(ResponsePreviewRequest {
        configuration_id: request.configuration_id,
        sample_rate_hz: request.sample_rate_hz,
        quantum_frames: request.quantum_frames,
        target,
        limits,
    }) {
        Ok(value) => value,
        Err(error) => return response_failure(staging, response_error_code(error)),
    };
    let points = grid.points();
    let section_count = prepared.descriptor().sections.len();
    let want_left = request.channels & crate::RESPONSE_CHANNEL_LEFT != 0;
    let want_right = request.channels & crate::RESPONSE_CHANNEL_RIGHT != 0;
    let want_sections = request.fields & crate::RESPONSE_FIELD_SECTIONS != 0;
    let maximum = request.maximum_result_bytes as usize;
    if let Err(result) = response_buffer_budget(
        points,
        section_count,
        request.channels,
        request.fields,
        maximum,
    ) {
        return response_failure(staging, result);
    }
    let mut frequencies = vec![0.0; points];
    let mut total_left = vec![0.0; points];
    let mut total_right = vec![0.0; points];
    let section_points = match section_count.checked_mul(points) {
        Some(value) => value,
        None => return response_failure(staging, RESULT_REFUSED_BUDGET),
    };
    let mut sections_left = want_sections.then(|| vec![0.0; section_points]);
    let mut sections_right = want_sections.then(|| vec![0.0; section_points]);
    if !want_left {
        total_left.fill(0.0);
    }
    if !want_right {
        total_right.fill(0.0);
    }
    let summary = match prepared.query_into(
        grid,
        ResponsePreviewOutput {
            frequencies_hz: &mut frequencies,
            total_left_db: &mut total_left,
            total_right_db: &mut total_right,
            sections_left_db: sections_left.as_deref_mut(),
            sections_right_db: sections_right.as_deref_mut(),
        },
    ) {
        Ok(value) => value,
        Err(error) => return response_failure(staging, response_error_code(error)),
    };
    let mut payload = Vec::new();
    let header_size = size_of::<WebResponseResult>();
    if header_size > maximum {
        return response_failure(staging, RESULT_REFUSED_BUDGET);
    }
    payload.resize(header_size, 0);
    let frequencies_offset = match append_f32_values(&mut payload, &frequencies, maximum) {
        Ok(value) => value,
        Err(result) => return response_failure(staging, result),
    };
    let total_left_offset = if want_left {
        match append_f32_values(&mut payload, &total_left, maximum) {
            Ok(value) => value,
            Err(result) => return response_failure(staging, result),
        }
    } else {
        0
    };
    let total_right_offset = if want_right {
        match append_f32_values(&mut payload, &total_right, maximum) {
            Ok(value) => value,
            Err(result) => return response_failure(staging, result),
        }
    } else {
        0
    };
    let sections_left_offset = if want_left && want_sections {
        match append_f32_values(
            &mut payload,
            sections_left.as_deref().unwrap_or(&[]),
            maximum,
        ) {
            Ok(value) => value,
            Err(result) => return response_failure(staging, result),
        }
    } else {
        0
    };
    let sections_right_offset = if want_right && want_sections {
        match append_f32_values(
            &mut payload,
            sections_right.as_deref().unwrap_or(&[]),
            maximum,
        ) {
            Ok(value) => value,
            Err(result) => return response_failure(staging, result),
        }
    } else {
        0
    };
    let configuration = prepared.configuration();
    let mask = |values: &[bool]| {
        values
            .iter()
            .enumerate()
            .fold(0_u32, |mask, (index, enabled)| {
                mask | (u32::from(*enabled) << index.min(31))
            })
    };
    let header = WebResponseResult {
        struct_size: RESPONSE_RESULT_BYTES,
        abi_version: ABI_VERSION,
        result: RESULT_OK,
        target: request.target,
        channels: request.channels,
        fields: request.fields,
        points: request.points,
        section_count: u32::try_from(section_count).unwrap_or(0),
        sample_rate_hz: summary.sample_rate_hz,
        reserved0: 0,
        configuration_id: summary.configuration_id,
        floor_db: summary.floor_db,
        bypass: u32::from(configuration.bypass.unwrap_or(false)),
        enabled_left: mask(configuration.enabled_left),
        enabled_right: mask(configuration.enabled_right),
        retained_bytes: u64::try_from(prepared.retained_bytes()).unwrap_or(u64::MAX),
        result_bytes: u64::try_from(payload.len()).unwrap_or(u64::MAX),
        frequencies_offset,
        total_left_offset,
        total_right_offset,
        sections_left_offset,
        sections_right_offset,
        reserved: [0; 3],
    };
    let header_bytes = response_header_bytes(header);
    payload[..header_size].copy_from_slice(&header_bytes);
    staging.result_header = header;
    staging.result = payload;
    RESULT_OK
}

/// Return a writable request header for one analysis-only response query.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_response_request_ptr() -> u32 {
    RESPONSE_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return 0;
        };
        staging.result.clear();
        staging.request.struct_size = RESPONSE_REQUEST_BYTES;
        staging.request.abi_version = ABI_VERSION;
        pointer_u32(ptr::from_mut(&mut *staging.request))
    })
}

/// Return the fixed request-header byte size.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_response_request_bytes() -> u32 {
    RESPONSE_REQUEST_BYTES
}

/// Return a writable effect-ID staging address.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_response_effect_id_ptr() -> u32 {
    RESPONSE_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return 0;
        };
        pointer_u32(staging.effect_id.as_mut_ptr())
    })
}

/// Return the maximum effect-ID staging length.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_response_effect_id_capacity() -> u32 {
    RESPONSE_MAXIMUM_EFFECT_ID_BYTES
}

/// Return a writable numeric-override staging address.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_response_parameter_ptr() -> u32 {
    RESPONSE_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return 0;
        };
        pointer_u32(staging.parameters.as_mut_ptr())
    })
}

/// Return the numeric-override record size.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_response_parameter_bytes() -> u32 {
    RESPONSE_PARAMETER_BYTES
}

/// Return the maximum number of numeric-override records.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_response_parameter_capacity() -> u32 {
    RESPONSE_MAXIMUM_PARAMETER_OVERRIDES
}

/// Prepare and query one explicit response without booting the audio host.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_response_query() -> u32 {
    RESPONSE_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        run_response_query(&mut staging)
    })
}

/// Return the result-header-plus-vectors address.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_response_result_ptr() -> u32 {
    RESPONSE_STAGING.with(|slot| {
        let Ok(staging) = slot.try_borrow() else {
            return 0;
        };
        pointer_u32(staging.result.as_ptr())
    })
}

/// Return the current result-header-plus-vectors byte length.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_response_result_bytes() -> u32 {
    RESPONSE_STAGING.with(|slot| {
        slot.try_borrow()
            .ok()
            .and_then(|staging| u32::try_from(staging.result.len()).ok())
            .unwrap_or(0)
    })
}

/// Close and release the one analysis-only response workspace.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_response_close() -> u32 {
    RESPONSE_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        staging.reset();
        RESULT_OK
    })
}

/// Return the writable pre-boot spectrum request header.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_request_ptr() -> u32 {
    SPECTRUM_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return 0;
        };
        staging.capture_len = 0;
        staging.result_len = 0;
        staging.release_collection_staging();
        staging.collection_request.entry_count = 0;
        staging.collection_request.maximum_capture_bytes = 0;
        staging.request.struct_size = SPECTRUM_REQUEST_BYTES;
        staging.request.abi_version = ABI_VERSION;
        pointer_u32(ptr::from_mut(&mut *staging.request))
    })
}

/// Return the fixed pre-boot spectrum request-header byte size.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_request_bytes() -> u32 {
    SPECTRUM_REQUEST_BYTES
}

/// Return the writable pre-boot spectrum collection request header.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_collection_request_ptr() -> u32 {
    SPECTRUM_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return 0;
        };
        staging.request.target = 0;
        staging.request.channels = 0;
        staging.request.target_id_bytes = 0;
        staging.request.maximum_capture_bytes = 0;
        staging.capture_len = 0;
        staging.result_len = 0;
        staging.release_collection_staging();
        staging.collection_request.struct_size = SPECTRUM_COLLECTION_REQUEST_BYTES;
        staging.collection_request.abi_version = ABI_VERSION;
        pointer_u32(ptr::from_mut(&mut *staging.collection_request))
    })
}

/// Return the fixed spectrum collection request-header byte size.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_collection_request_bytes() -> u32 {
    SPECTRUM_COLLECTION_REQUEST_BYTES
}

/// Return writable staging for the caller-sized spectrum collection entries.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_collection_entry_ptr() -> u32 {
    SPECTRUM_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return 0;
        };
        let count = usize::try_from(staging.collection_request.entry_count).ok();
        let Some(count) = count else {
            return 0;
        };
        let Some(bytes) = count.checked_mul(size_of::<WebSpectrumCollectionEntry>()) else {
            return 0;
        };
        // The caller chooses the collection size, while this checked bridge-side ceiling keeps a
        // malformed u32 count from turning the pre-boot staging call into an unbounded request.
        // It is a byte budget, not a target-count limit; larger configured collections remain
        // admissible whenever their actual bytes fit the host's later resource projection.
        let Ok(bytes_u64) = u64::try_from(bytes) else {
            return 0;
        };
        if bytes_u64 > crate::DEFAULT_MAXIMUM_MEMORY_BYTES {
            return 0;
        }
        staging.collection_entries = Vec::new();
        staging.collection_target_ids = Vec::new();
        if staging.collection_entries.try_reserve_exact(count).is_err() {
            return 0;
        }
        staging
            .collection_entries
            .resize(count, WebSpectrumCollectionEntry::default());
        debug_assert_eq!(
            bytes,
            staging.collection_entries.len() * size_of::<WebSpectrumCollectionEntry>()
        );
        pointer_u32(staging.collection_entries.as_mut_ptr())
    })
}

/// Return the number of collection entries staged by the caller.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_collection_entry_capacity() -> u32 {
    SPECTRUM_STAGING.with(|slot| {
        let Ok(staging) = slot.try_borrow() else {
            return 0;
        };
        u32::try_from(staging.collection_entries.len()).unwrap_or(0)
    })
}

/// Return the byte size of one fixed-slot spectrum collection entry.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_collection_entry_bytes() -> u32 {
    SPECTRUM_COLLECTION_ENTRY_BYTES
}

/// Return writable staging for the collection's packed target identities.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_collection_target_ids_ptr() -> u32 {
    SPECTRUM_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return 0;
        };
        let Some(bytes) = staging
            .collection_entries
            .iter()
            .try_fold(0_usize, |total, entry| {
                total.checked_add(usize::try_from(entry.target_id_bytes).ok()?)
            })
        else {
            return 0;
        };
        let Ok(bytes_u64) = u64::try_from(bytes) else {
            return 0;
        };
        let entry_bytes = staging
            .collection_entries
            .len()
            .checked_mul(size_of::<WebSpectrumCollectionEntry>())
            .and_then(|bytes| u64::try_from(bytes).ok());
        let Some(entry_bytes) = entry_bytes else {
            return 0;
        };
        let Some(total_bytes) = entry_bytes.checked_add(bytes_u64) else {
            return 0;
        };
        if total_bytes > crate::DEFAULT_MAXIMUM_MEMORY_BYTES {
            return 0;
        }
        staging.collection_target_ids = Vec::new();
        if staging
            .collection_target_ids
            .try_reserve_exact(bytes)
            .is_err()
        {
            return 0;
        }
        staging.collection_target_ids.resize(bytes, 0);
        pointer_u32(staging.collection_target_ids.as_mut_ptr())
    })
}

/// Return the packed collection target-identity staging capacity in bytes.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_collection_target_ids_capacity() -> u32 {
    SPECTRUM_STAGING.with(|slot| {
        let Ok(staging) = slot.try_borrow() else {
            return 0;
        };
        u32::try_from(staging.collection_target_ids.len()).unwrap_or(0)
    })
}

/// Return writable staging for the selected spectrum target identity.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_target_id_ptr() -> u32 {
    SPECTRUM_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return 0;
        };
        pointer_u32(staging.target_id.as_mut_ptr())
    })
}

/// Return the maximum spectrum target identity byte length.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_target_id_capacity() -> u32 {
    SPECTRUM_MAXIMUM_ID_BYTES as u32
}

/// Return the fixed raw spectrum-window staging address.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_capture_ptr() -> u32 {
    SPECTRUM_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return 0;
        };
        if staging.capture.is_none()
            && spectrum_configured(&staging)
            && staging.configure_capture().is_err()
        {
            return 0;
        }
        staging
            .capture
            .as_ref()
            .map_or(0, |capture| pointer_u32(capture.as_ptr()))
    })
}

/// Return the maximum raw spectrum-window staging capacity.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_capture_capacity() -> u32 {
    SPECTRUM_STAGING.with(|slot| {
        slot.try_borrow().ok().map_or(0, |staging| {
            staging
                .capture
                .as_ref()
                .and_then(|capture| u32::try_from(capture.len()).ok())
                .unwrap_or(0)
        })
    })
}

/// Return the byte length of the most recent raw spectrum window.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_capture_bytes() -> u32 {
    SPECTRUM_STAGING.with(|slot| {
        slot.try_borrow()
            .ok()
            .and_then(|staging| u32::try_from(staging.capture_len).ok())
            .unwrap_or(0)
    })
}

/// Set the byte length of raw spectrum capture bytes copied into the fixed staging area.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_capture_set_bytes(bytes: u32) -> u32 {
    SPECTRUM_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        if staging.capture.is_none()
            && spectrum_configured(&staging)
            && staging.configure_capture().is_err()
        {
            return RESULT_REFUSED_BUDGET;
        }
        let Ok(length) = usize::try_from(bytes) else {
            return RESULT_INVALID_ARGUMENT;
        };
        let Some(capacity) = staging.capture.as_ref().map(Vec::len) else {
            return RESULT_INVALID_ARGUMENT;
        };
        if !(SPECTRUM_WINDOW_HEADER_BYTES as usize..=capacity).contains(&length) {
            return RESULT_INVALID_ARGUMENT;
        }
        staging.capture_len = length;
        RESULT_OK
    })
}

/// Analyze one staged raw spectrum window using the native off-render analyzer.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_analysis() -> u32 {
    SPECTRUM_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        run_spectrum_analysis(&mut staging, false)
    })
}

/// Release the one-shot spectrum staging payload lengths.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_close() -> u32 {
    SPECTRUM_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        staging.capture_len = 0;
        staging.result_len = 0;
        RESULT_OK
    })
}

/// Return the fixed analyzed spectrum-result staging address.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_result_ptr() -> u32 {
    SPECTRUM_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return 0;
        };
        if staging.result.is_none()
            && spectrum_configured(&staging)
            && staging.configure_capture().is_err()
        {
            return 0;
        }
        staging
            .result
            .as_ref()
            .map_or(0, |result| pointer_u32(result.as_ptr()))
    })
}

/// Return the byte length of the most recent analyzed spectrum result.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_result_bytes() -> u32 {
    SPECTRUM_STAGING.with(|slot| {
        slot.try_borrow()
            .ok()
            .and_then(|staging| u32::try_from(staging.result_len).ok())
            .unwrap_or(0)
    })
}

/// Arm the prepared spectrum observer for its next complete window.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_arm(handle: u32) -> u32 {
    with_host_mut(
        handle,
        RESULT_INVALID_ARGUMENT,
        AudioWorkletEngineHost::arm_spectrum,
    )
}

fn select_spectrum_internal(handle: u32, target: u32, channels: u32, target_id_bytes: u32) -> u32 {
    select_spectrum_with_smoothing(handle, target, channels, target_id_bytes, None)
}

fn select_spectrum_with_smoothing(
    handle: u32,
    target: u32,
    channels: u32,
    target_id_bytes: u32,
    smoothing: Option<SpectrumSmoothingConfig>,
) -> u32 {
    match refuse_protected_observation_alias(handle, OBSERVATION_OPERATION_COLLECTION_SELECTION) {
        Err(result) | Ok(Some(result)) => return result,
        Ok(None) => {}
    }
    SPECTRUM_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        if smoothing.is_some() && !staging.stream_active {
            return RESULT_WRONG_STATE;
        }
        let id_bytes = match usize::try_from(target_id_bytes) {
            Ok(value) if value > 0 && value <= staging.target_id.len() => value,
            _ => return RESULT_INVALID_ARGUMENT,
        };
        let id = match core::str::from_utf8(&staging.target_id[..id_bytes]) {
            Ok(value) => value,
            Err(_) => return RESULT_INVALID_ARGUMENT,
        };
        let target = match spectrum_target(target, id) {
            Ok(value) => value,
            Err(result) => return result,
        };
        let channels = match spectrum_channels(channels) {
            Ok(value) => value,
            Err(result) => return result,
        };
        let smoothing_changed = smoothing.is_some() && staging.stream_smoothing != smoothing;
        let selection_would_change = match with_host(handle, Err(RESULT_INVALID_ARGUMENT), |host| {
            host.spectrum_selection_would_change(&target, channels)
        }) {
            Ok(value) => value,
            Err(result) => return result,
        };
        if staging.stream_active
            && (selection_would_change || smoothing_changed)
            && staging
                .stream_history
                .as_ref()
                .is_some_and(|history| history.analysis_epoch() == u64::MAX)
        {
            return RESULT_REFUSED_BUDGET;
        }
        let selection_epoch_before =
            with_host(handle, 0, AudioWorkletEngineHost::spectrum_selection_epoch);
        let result = with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
            host.select_spectrum(&target, channels)
        });
        let selection_epoch_after =
            with_host(handle, 0, AudioWorkletEngineHost::spectrum_selection_epoch);
        if result != RESULT_OK {
            return result;
        }
        let selection_changed = selection_epoch_after != selection_epoch_before;
        debug_assert_eq!(selection_changed, selection_would_change);
        if staging.stream_active && smoothing_changed && !selection_changed {
            let restart = with_host_mut(
                handle,
                RESULT_INVALID_ARGUMENT,
                AudioWorkletEngineHost::restart_spectrum_stream,
            );
            if restart != RESULT_OK {
                return restart;
            }
        }
        if staging.stream_active && (selection_changed || smoothing_changed) {
            if selection_changed || smoothing_changed {
                // The epoch check above makes this reset infallible at the commit point. The
                // native capture selection and analysis configuration therefore change as one
                // bounded control operation, without a fallible stop/start pair.
                let reset = staging.reset_stream_analysis();
                debug_assert!(reset.is_ok(), "smoothing reset was preflighted");
                if reset.is_err() {
                    return RESULT_INTERNAL;
                }
            }
            // Selection commits on the host side, so refresh the copied stream profile in the
            // same control operation. The next read may be warming, but it must never publish
            // the old target after a successful switch.
            if selection_changed {
                let selected_target = with_host(handle, 0, AudioWorkletEngineHost::spectrum_target);
                let selected_channels =
                    with_host(handle, 0, AudioWorkletEngineHost::spectrum_channels);
                staging.stream_metadata.target = selected_target;
                staging.stream_metadata.channels = selected_channels;
            }
            if selection_changed || smoothing_changed {
                staging.stream_metadata.capture_epoch =
                    with_host(handle, 0, |host| host.spectrum_stream_epoch().unwrap_or(0));
                staging.stream_metadata.sequence = 0;
                staging.stream_metadata.dropped_captures = 0;
                staging.stream_metadata.windows = 0;
                staging.stream_metadata.captured_sample = 0;
                staging.stream_metadata.end_sample = 0;
                staging.stream_metadata.source_underrun = 0;
                staging.stream_metadata.analysis_epoch = staging
                    .stream_history
                    .as_ref()
                    .map_or(0, |history| history.analysis_epoch());
                staging.stream_metadata.history_start_sample = 0;
            }
            staging.stream_metadata.status = SPECTRUM_STREAM_STATUS_WARMING;
            staging.stream_metadata.result = RESULT_OK;
            if let Some(smoothing) = smoothing {
                staging.stream_smoothing = Some(smoothing);
                staging.stream_metadata.smoothing_ms = smoothing.smoothing_ms();
            }
            staging.capture_len = 0;
            staging.result_len = 0;
            staging.stream_window = None;
        }
        result
    })
}

/// Atomically select one exact prepared collection entry.
///
/// The target identity is read from the fixed selection-ID staging buffer. The host validates the
/// entry and its epoch before retiring the current capture, so every refusal preserves it.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_select(
    handle: u32,
    target: u32,
    channels: u32,
    target_id_bytes: u32,
) -> u32 {
    select_spectrum_internal(handle, target, channels, target_id_bytes)
}

/// Atomically select one prepared stream entry and commit its smoothing configuration.
///
/// Smoothing and target admission happen before the host selection is committed. A changed
/// configuration resets the preallocated worker history at the same control boundary; no
/// fallible stream stop/start pair can leave a partially switched owner.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_stream_select(
    handle: u32,
    target: u32,
    channels: u32,
    target_id_bytes: u32,
    smoothing_ms: f64,
) -> u32 {
    match refuse_protected_observation_alias(handle, OBSERVATION_OPERATION_COLLECTION_SELECTION) {
        Err(result) | Ok(Some(result)) => return result,
        Ok(None) => {}
    }
    let smoothing = match SpectrumSmoothingConfig::new(smoothing_ms) {
        Ok(value) => value,
        Err(_) => return RESULT_INVALID_ARGUMENT,
    };
    select_spectrum_with_smoothing(handle, target, channels, target_id_bytes, Some(smoothing))
}

/// Return the monotonic identity of the currently committed collection selection.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_selection_epoch(handle: u32) -> u64 {
    with_host(handle, 0, AudioWorkletEngineHost::spectrum_selection_epoch)
}

/// Read a completed spectrum window into fixed staging; returns backpressure while pending.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_read(handle: u32, channels: u32) -> u32 {
    match refuse_protected_observation_alias(handle, OBSERVATION_OPERATION_ONE_SHOT) {
        Err(result) | Ok(Some(result)) => return result,
        Ok(None) => {}
    }
    SPECTRUM_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        staging.capture_len = 0;
        let requested = match spectrum_channels(channels) {
            Ok(value) => value,
            Err(result) => return result,
        };
        let target = with_host(handle, 0, AudioWorkletEngineHost::spectrum_target);
        let sample_rate_hz = with_host(handle, 0, |host| host.status().sample_rate_hz);
        if sample_rate_hz == 0 {
            return RESULT_INVALID_ARGUMENT;
        }
        let available = with_host(handle, 0, AudioWorkletEngineHost::spectrum_channels);
        if available == 0 || (channels & !available) != 0 {
            return RESULT_INVALID_ARGUMENT;
        }
        let read = with_host_mut(handle, Err(RESULT_INVALID_ARGUMENT), |host| {
            host.read_spectrum()
        });
        let (mut window, token) = match read {
            Ok(Some(value)) => value,
            Ok(None) => return RESULT_BACKPRESSURE,
            Err(result) => return result,
        };
        if (requested as u32) & !(window.channels as u32) != 0 {
            return RESULT_INVALID_ARGUMENT;
        }
        window.channels = requested;
        write_spectrum_window(&mut staging, &window, target, sample_rate_hz, token)
    })
}

/// Start the managed spectrum stream for the prepared target.
///
/// The host supplies the sample rate, quantum and hop in the returned metadata. The smoothing
/// duration is validated before the prepared capture changes state.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_stream_start(handle: u32, smoothing_ms: f64) -> u32 {
    match stream_host_dispatch(handle) {
        Ok(true) => return protected_spectrum_stream_start(handle, smoothing_ms),
        Ok(false) => {}
        Err(result) => return result,
    }
    let smoothing = match SpectrumSmoothingConfig::new(smoothing_ms) {
        Ok(value) => value,
        Err(_) => return RESULT_INVALID_ARGUMENT,
    };
    SPECTRUM_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        // An existing history is reset at this control boundary. The analysis owner initializes a
        // missing history when it first evaluates a window, so stream start itself never creates a
        // second host-side analyzer allocation.
        if staging
            .stream_history
            .as_ref()
            .is_some_and(|history| history.analysis_epoch() == u64::MAX)
        {
            staging.stream_metadata.result = RESULT_REFUSED_BUDGET;
            return RESULT_REFUSED_BUDGET;
        }
        let cadence = with_host_mut(handle, Err(RESULT_INVALID_ARGUMENT), |host| {
            host.start_spectrum_stream()
        });
        let cadence = match cadence {
            Ok(value) => value,
            Err(result) => {
                staging.stream_metadata.result = result;
                return result;
            }
        };
        if staging.reset_stream_analysis().is_err() {
            let _ = with_host_mut(
                handle,
                RESULT_INTERNAL,
                AudioWorkletEngineHost::stop_spectrum_stream,
            );
            staging.stream_metadata.result = RESULT_REFUSED_BUDGET;
            return RESULT_REFUSED_BUDGET;
        }
        let target = with_host(handle, 0, AudioWorkletEngineHost::spectrum_target);
        let channels = with_host(handle, 0, AudioWorkletEngineHost::spectrum_channels);
        let epoch = with_host(handle, 0, |host| host.spectrum_stream_epoch().unwrap_or(0));
        staging.stream_active = true;
        staging.stream_cadence = Some(cadence);
        staging.stream_smoothing = Some(smoothing);
        staging.capture_len = 0;
        staging.result_len = 0;
        staging.stream_window = None;
        let analysis_epoch = staging
            .stream_history
            .as_ref()
            .map_or(0, |history| history.analysis_epoch());
        staging.stream_metadata = WebSpectrumStreamMetadata::default();
        stream_metadata_profile(
            &mut staging,
            target,
            channels,
            cadence,
            SPECTRUM_STREAM_STATUS_WARMING,
            RESULT_OK,
        );
        staging.stream_metadata = WebSpectrumStreamMetadata {
            struct_size: SPECTRUM_STREAM_METADATA_BYTES,
            abi_version: ABI_VERSION,
            result: RESULT_OK,
            capture_epoch: epoch,
            analysis_epoch,
            smoothing_ms: smoothing.smoothing_ms(),
            ..staging.stream_metadata
        };
        RESULT_OK
    })
}

/// Start one protected managed stream through the admitted native seam.
///
/// The operation credit is spent before decoding the smoothing value or borrowing output
/// staging. The prepared target length is the only target sizing input; the legacy staged request
/// and target-ID buffers are deliberately not consulted. Native acceptance is the transaction
/// boundary for every FFI-visible stream field, including analysis history reset.
fn protected_spectrum_stream_start(handle: u32, smoothing_ms: f64) -> u32 {
    const OPERATION: u32 = OBSERVATION_OPERATION_START_SPECTRUM;
    LIVE_HOST.with(|slot| {
        let Ok(mut slot) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        let Some(live) = slot.as_mut().filter(|live| live.handle == handle) else {
            return RESULT_INVALID_ARGUMENT;
        };
        let host = &mut live.host;
        let target_bytes = host.protected_spectrum_target_bytes().unwrap_or_default();
        let permit = match host.begin_observation(
            ObservationClass::Ordinary,
            AudioWorkletEngineHost::protected_spectrum_control_lengths(false, target_bytes),
        ) {
            Ok(permit) => permit,
            Err(refusal) => return host.record_observation_refusal(OPERATION, refusal),
        };

        let smoothing = match SpectrumSmoothingConfig::new(smoothing_ms) {
            Ok(value) => value,
            Err(_) => {
                return host.record_observation_refusal(
                    OPERATION,
                    ObservationRefusal {
                        reason: ObservationRefusalReason::InvalidRequest,
                        limit: None,
                        requested: None,
                        maximum: None,
                    },
                );
            }
        };

        SPECTRUM_STAGING.with(|staging_slot| {
            let Ok(mut staging) = staging_slot.try_borrow_mut() else {
                host.record_observation_admission(OPERATION, RESULT_INTERNAL, None, None, false);
                return RESULT_INTERNAL;
            };

            // Reset can fail only at the epoch ceiling. Check that boundary after admission and
            // before native publication so a refusal cannot publish a graph or alter staging.
            if staging.stream_history_exhausted() {
                return host.record_observation_refusal(
                    OPERATION,
                    ObservationRefusal {
                        reason: ObservationRefusalReason::ArithmeticOverflow,
                        limit: None,
                        requested: None,
                        maximum: None,
                    },
                );
            }

            let cadence = match host.start_spectrum_stream_admitted(permit) {
                Ok(value) => value,
                Err(result) => return result,
            };

            // The epoch check above makes this reset infallible at the commit point. There is no
            // late stop fallback: native acceptance remains authoritative if a defensive release
            // build ever observes an unexpected reset error.
            let reset = staging.reset_stream_analysis();
            debug_assert!(reset.is_ok(), "stream-history reset was preflighted");

            let target = host.spectrum_target();
            let channels = host.spectrum_channels();
            let epoch = host.spectrum_stream_epoch().unwrap_or(0);
            staging.stream_active = true;
            staging.stream_cadence = Some(cadence);
            staging.stream_smoothing = Some(smoothing);
            staging.capture_len = 0;
            staging.result_len = 0;
            staging.stream_window = None;
            let analysis_epoch = staging
                .stream_history
                .as_ref()
                .map_or(0, |history| history.analysis_epoch());
            staging.stream_metadata = WebSpectrumStreamMetadata::default();
            stream_metadata_profile(
                &mut staging,
                target,
                channels,
                cadence,
                SPECTRUM_STREAM_STATUS_WARMING,
                RESULT_OK,
            );
            staging.stream_metadata = WebSpectrumStreamMetadata {
                struct_size: SPECTRUM_STREAM_METADATA_BYTES,
                abi_version: ABI_VERSION,
                result: RESULT_OK,
                capture_epoch: epoch,
                analysis_epoch,
                smoothing_ms: smoothing.smoothing_ms(),
                ..staging.stream_metadata
            };
            RESULT_OK
        })
    })
}

/// Read one protected managed stream window through the typed native seam.
///
/// The live-host borrow and the affine Ordinary permit are acquired before this function touches
/// spectrum staging. Known staging capacity and cadence are checked before native consumption;
/// after an admitted native window, every shape and identity check completes before the first raw
/// byte is changed.
fn protected_spectrum_stream_read(handle: u32) -> u32 {
    const OPERATION: u32 = OBSERVATION_OPERATION_READ_SPECTRUM;
    LIVE_HOST.with(|slot| {
        let Ok(mut slot) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        let Some(live) = slot.as_mut().filter(|live| live.handle == handle) else {
            return RESULT_INVALID_ARGUMENT;
        };
        let host = &mut live.host;
        let read_lengths = host.protected_spectrum_read_lengths();
        let (owner, _) = match host.protected_observation_identity() {
            Ok(identity) => identity,
            Err(refusal) => {
                return host.record_observation_refusal(OPERATION, refusal);
            }
        };
        let permit = match host.begin_observation(ObservationClass::Ordinary, read_lengths) {
            Ok(permit) => permit,
            Err(refusal) => return host.record_observation_refusal(OPERATION, refusal),
        };

        SPECTRUM_STAGING.with(|staging_slot| {
            let Ok(mut staging) = staging_slot.try_borrow_mut() else {
                host.record_observation_admission(OPERATION, RESULT_INTERNAL, None, None, false);
                return RESULT_INTERNAL;
            };

            let target = host.spectrum_target();
            let channels = host.spectrum_channels();
            if target != SPECTRUM_TARGET_TRACK_POST_MATRIX || channels != SPECTRUM_CHANNEL_BOTH {
                return protected_spectrum_read_refusal(
                    host,
                    ObservationRefusalReason::InvalidRequest,
                    None,
                    None,
                    None,
                );
            }
            let cadence = match host.protected_spectrum_cadence() {
                Ok(cadence) => cadence,
                Err(refusal) => return host.record_observation_refusal(OPERATION, refusal),
            };
            let header_bytes = usize::try_from(SPECTRUM_WINDOW_HEADER_BYTES)
                .expect("fixed spectrum header fits usize");
            let plane_bytes = (SPECTRUM_WINDOW_FRAMES as usize) * size_of::<f32>();
            let required_bytes = header_bytes
                .checked_add(plane_bytes)
                .and_then(|bytes| bytes.checked_add(plane_bytes));
            let Some(required_bytes) = required_bytes else {
                return protected_spectrum_read_refusal(
                    host,
                    ObservationRefusalReason::ArithmeticOverflow,
                    None,
                    None,
                    None,
                );
            };
            let Some(cached_bytes) = usize::try_from(read_lengths.result_bytes).ok() else {
                return protected_spectrum_read_refusal(
                    host,
                    ObservationRefusalReason::ArithmeticOverflow,
                    Some("maximum_result_bytes"),
                    Some(read_lengths.result_bytes),
                    None,
                );
            };
            if cached_bytes < required_bytes {
                return protected_spectrum_read_refusal(
                    host,
                    ObservationRefusalReason::Capacity,
                    Some("maximum_result_bytes"),
                    Some(required_bytes as u64),
                    Some(read_lengths.result_bytes),
                );
            }

            if staging.capture.is_none()
                && staging.result.is_none()
                && staging.configure_capture().is_err()
            {
                return protected_spectrum_read_refusal(
                    host,
                    ObservationRefusalReason::Capacity,
                    Some("spectrum_capture_capacity"),
                    Some(required_bytes as u64),
                    Some(0),
                );
            }
            let capacity = staging.capture.as_ref().map_or(0, Vec::len);
            if capacity < header_bytes {
                return protected_spectrum_read_refusal(
                    host,
                    ObservationRefusalReason::Capacity,
                    Some("spectrum_capture_capacity"),
                    Some(header_bytes as u64),
                    Some(capacity as u64),
                );
            }
            if capacity < required_bytes {
                return protected_spectrum_read_refusal(
                    host,
                    ObservationRefusalReason::Capacity,
                    Some("spectrum_capture_capacity"),
                    Some(required_bytes as u64),
                    Some(capacity as u64),
                );
            }

            let native = host.read_spectrum_stream_admitted(permit);
            let window = match native {
                Ok(window) => window,
                Err(ProtectedSpectrumReadError::Refused(_)) => {
                    // The admitted seam has already recorded the exact refusal. Do not map it
                    // through the lossy public compatibility facade or touch committed staging.
                    return host.observation_admission().result;
                }
                Err(ProtectedSpectrumReadError::Native(error)) => {
                    let (status, result, epoch, drops, reset) = match error {
                        host_core::HostSpectrumReadError::Inactive
                        | host_core::HostSpectrumReadError::Closed => (
                            SPECTRUM_STREAM_STATUS_INACTIVE,
                            RESULT_WRONG_STATE,
                            None,
                            None,
                            false,
                        ),
                        host_core::HostSpectrumReadError::PendingApplication => (
                            SPECTRUM_STREAM_STATUS_PENDING,
                            RESULT_BACKPRESSURE,
                            None,
                            None,
                            false,
                        ),
                        host_core::HostSpectrumReadError::Warming => (
                            SPECTRUM_STREAM_STATUS_WARMING,
                            RESULT_BACKPRESSURE,
                            None,
                            None,
                            false,
                        ),
                        host_core::HostSpectrumReadError::Pending => (
                            SPECTRUM_STREAM_STATUS_PENDING,
                            RESULT_BACKPRESSURE,
                            None,
                            None,
                            false,
                        ),
                        host_core::HostSpectrumReadError::Gap {
                            stream_epoch,
                            dropped_captures,
                            ..
                        } => (
                            SPECTRUM_STREAM_STATUS_GAP,
                            RESULT_OK,
                            Some(stream_epoch),
                            Some(dropped_captures),
                            true,
                        ),
                        host_core::HostSpectrumReadError::Failed { stream_epoch, .. } => (
                            SPECTRUM_STREAM_STATUS_FAILED,
                            RESULT_RENDER_REJECTED,
                            Some(stream_epoch),
                            None,
                            true,
                        ),
                    };
                    if reset && staging.reset_stream_analysis().is_err() {
                        let refusal = ObservationRefusal {
                            reason: ObservationRefusalReason::ArithmeticOverflow,
                            limit: None,
                            requested: None,
                            maximum: None,
                        };
                        host.record_observation_admission(
                            OPERATION,
                            RESULT_REFUSED_BUDGET,
                            Some(refusal),
                            None,
                            false,
                        );
                        stream_metadata_error(
                            &mut staging,
                            SPECTRUM_STREAM_STATUS_FAILED,
                            RESULT_REFUSED_BUDGET,
                            epoch,
                            drops,
                        );
                        return RESULT_REFUSED_BUDGET;
                    }
                    stream_metadata_error(&mut staging, status, result, epoch, drops);
                    return host.observation_admission().result;
                }
            };

            post_read_protected_spectrum_window(
                host,
                &mut staging,
                &window,
                owner.get(),
                target,
                channels,
                cadence,
            )
        })
    })
}

/// Validate an already consumed native window before changing any committed output.
/// A failed post-read check records its arithmetic or identity outcome without implying that
/// the native queue can restore the window.
fn post_read_protected_spectrum_window(
    host: &mut AudioWorkletEngineHost,
    staging: &mut SpectrumStaging,
    window: &ObservedContinuousSpectrumWindow,
    owner: u64,
    target: u32,
    channels: u32,
    cadence: SpectrumCadence,
) -> u32 {
    const OPERATION: u32 = OBSERVATION_OPERATION_READ_SPECTRUM;
    let expected_end = match window.window.end_sample() {
        Some(end_sample) => end_sample,
        None => {
            let refusal = ObservationRefusal {
                reason: ObservationRefusalReason::ArithmeticOverflow,
                limit: None,
                requested: None,
                maximum: None,
            };
            host.record_observation_admission(
                OPERATION,
                RESULT_REFUSED_BUDGET,
                Some(refusal),
                None,
                false,
            );
            return RESULT_REFUSED_BUDGET;
        }
    };
    if window.window.channels as u32 != SPECTRUM_CHANNEL_BOTH
        || window.window.left.len() != SPECTRUM_WINDOW_FRAMES as usize
        || window.window.right.len() != SPECTRUM_WINDOW_FRAMES as usize
    {
        let refusal = ObservationRefusal {
            reason: ObservationRefusalReason::InvalidRequest,
            limit: None,
            requested: None,
            maximum: None,
        };
        host.record_observation_admission(
            OPERATION,
            RESULT_INVALID_ARGUMENT,
            Some(refusal),
            None,
            false,
        );
        return RESULT_INVALID_ARGUMENT;
    }
    let snapshot_token = match window.window.sequence.checked_add(1) {
        Some(token) => token,
        None => {
            let refusal = ObservationRefusal {
                reason: ObservationRefusalReason::ArithmeticOverflow,
                limit: None,
                requested: None,
                maximum: None,
            };
            host.record_observation_admission(
                OPERATION,
                RESULT_REFUSED_BUDGET,
                Some(refusal),
                None,
                false,
            );
            return RESULT_REFUSED_BUDGET;
        }
    };
    if expected_end
        != window
            .window
            .first_sample
            .checked_add(u64::from(SPECTRUM_WINDOW_FRAMES))
            .expect("end sample was checked")
    {
        let refusal = ObservationRefusal {
            reason: ObservationRefusalReason::InvalidRequest,
            limit: None,
            requested: None,
            maximum: None,
        };
        host.record_observation_admission(
            OPERATION,
            RESULT_INVALID_ARGUMENT,
            Some(refusal),
            None,
            false,
        );
        return RESULT_INVALID_ARGUMENT;
    }

    let identity = match AudioWorkletEngineHost::spectrum_capture_identity(window) {
        Ok(identity) => identity,
        Err(result) => {
            let reason = if result == RESULT_REFUSED_BUDGET {
                ObservationRefusalReason::ArithmeticOverflow
            } else {
                ObservationRefusalReason::InvalidRequest
            };
            let refusal = ObservationRefusal {
                reason,
                limit: None,
                requested: None,
                maximum: None,
            };
            host.record_observation_admission(OPERATION, result, Some(refusal), None, false);
            return result;
        }
    };
    let status = host.observation_status();
    if identity.owner != owner
        || identity.observation_generation == 0
        || identity.selection_epoch == 0
        || identity.observation_generation != status.applied_generation
        || identity.selection_epoch != status.selection_epoch
        || identity.snapshot_token != snapshot_token
    {
        let refusal = ObservationRefusal {
            reason: ObservationRefusalReason::InvalidRequest,
            limit: None,
            requested: None,
            maximum: None,
        };
        host.record_observation_admission(
            OPERATION,
            RESULT_INVALID_ARGUMENT,
            Some(refusal),
            None,
            false,
        );
        return RESULT_INVALID_ARGUMENT;
    }

    commit_protected_spectrum_window(staging, window, target, cadence, snapshot_token);
    staging.stream_window = Some(SpectrumStreamWindowFacts {
        stream_epoch: window.window.stream_epoch,
        sequence: window.window.sequence,
        dropped_captures: window.window.dropped_captures,
    });
    staging.stream_metadata.result = RESULT_OK;
    staging.stream_metadata.status = SPECTRUM_STREAM_STATUS_READY;
    staging.stream_metadata.target = target;
    staging.stream_metadata.channels = channels;
    staging.stream_metadata.sample_rate_hz = cadence.sample_rate_hz();
    staging.stream_metadata.quantum_frames = cadence.quantum_frames();
    staging.stream_metadata.hop_frames = cadence.hop_frames();
    staging.stream_metadata.source_underrun = u32::from(window.window.source_underrun);
    staging.stream_metadata.capture_epoch = window.window.stream_epoch;
    staging.stream_metadata.sequence = window.window.sequence;
    staging.stream_metadata.dropped_captures = window.window.dropped_captures;
    staging.stream_metadata.windows = snapshot_token;
    staging.stream_metadata.captured_sample = window.window.first_sample;
    staging.stream_metadata.end_sample = expected_end;
    host.commit_observation_capture_identity(identity);
    RESULT_OK
}

/// Read one independently scheduled stream window into the existing fixed capture staging.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_stream_read(handle: u32) -> u32 {
    match stream_host_dispatch(handle) {
        Ok(true) => return protected_spectrum_stream_read(handle),
        Ok(false) => {}
        Err(result) => return result,
    }
    SPECTRUM_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        staging.capture_len = 0;
        staging.result_len = 0;
        // A collection selection can occur between reads. Read the host's committed entry on
        // every path, including warming/pending, so metadata cannot retain the previous target.
        staging.stream_metadata.target =
            with_host(handle, 0, AudioWorkletEngineHost::spectrum_target);
        staging.stream_metadata.channels =
            with_host(handle, 0, AudioWorkletEngineHost::spectrum_channels);
        let read = with_host_mut(
            handle,
            Err(SpectrumContinuousReadError::NotActive),
            |host| host.read_spectrum_stream(),
        );
        let window = match read {
            Ok(window) => window,
            Err(error) => {
                let (status, result, epoch, drops) = match error {
                    SpectrumContinuousReadError::NotActive => (
                        SPECTRUM_STREAM_STATUS_INACTIVE,
                        RESULT_WRONG_STATE,
                        None,
                        None,
                    ),
                    SpectrumContinuousReadError::Warming => (
                        SPECTRUM_STREAM_STATUS_WARMING,
                        RESULT_BACKPRESSURE,
                        None,
                        None,
                    ),
                    SpectrumContinuousReadError::Pending => (
                        SPECTRUM_STREAM_STATUS_PENDING,
                        RESULT_BACKPRESSURE,
                        None,
                        None,
                    ),
                    SpectrumContinuousReadError::Gap {
                        stream_epoch,
                        dropped_captures,
                    } => (
                        SPECTRUM_STREAM_STATUS_GAP,
                        RESULT_OK,
                        Some(stream_epoch),
                        Some(dropped_captures),
                    ),
                    SpectrumContinuousReadError::Failed { stream_epoch } => (
                        SPECTRUM_STREAM_STATUS_FAILED,
                        RESULT_RENDER_REJECTED,
                        Some(stream_epoch),
                        None,
                    ),
                };
                if matches!(
                    error,
                    SpectrumContinuousReadError::Gap { .. }
                        | SpectrumContinuousReadError::Failed { .. }
                ) && staging.reset_stream_analysis().is_err()
                {
                    stream_metadata_error(
                        &mut staging,
                        SPECTRUM_STREAM_STATUS_FAILED,
                        RESULT_REFUSED_BUDGET,
                        epoch,
                        drops,
                    );
                    return RESULT_REFUSED_BUDGET;
                }
                stream_metadata_error(&mut staging, status, result, epoch, drops);
                return result;
            }
        };
        let Some(cadence) = staging.stream_cadence else {
            stream_metadata_error(
                &mut staging,
                SPECTRUM_STREAM_STATUS_FAILED,
                RESULT_WRONG_STATE,
                Some(window.stream_epoch),
                Some(window.dropped_captures),
            );
            return RESULT_WRONG_STATE;
        };
        let windows = match window.sequence.checked_add(1) {
            Some(value) => value,
            None => {
                stream_metadata_error(
                    &mut staging,
                    SPECTRUM_STREAM_STATUS_FAILED,
                    RESULT_REFUSED_BUDGET,
                    Some(window.stream_epoch),
                    Some(window.dropped_captures),
                );
                return RESULT_REFUSED_BUDGET;
            }
        };
        let Some(end_sample) = window.end_sample() else {
            stream_metadata_error(
                &mut staging,
                SPECTRUM_STREAM_STATUS_FAILED,
                RESULT_INVALID_ARGUMENT,
                Some(window.stream_epoch),
                Some(window.dropped_captures),
            );
            return RESULT_INVALID_ARGUMENT;
        };
        staging.stream_window = Some(SpectrumStreamWindowFacts {
            stream_epoch: window.stream_epoch,
            sequence: window.sequence,
            dropped_captures: window.dropped_captures,
        });
        staging.stream_metadata.result = RESULT_OK;
        staging.stream_metadata.status = SPECTRUM_STREAM_STATUS_READY;
        staging.stream_metadata.capture_epoch = window.stream_epoch;
        staging.stream_metadata.sequence = window.sequence;
        staging.stream_metadata.dropped_captures = window.dropped_captures;
        staging.stream_metadata.windows = windows;
        staging.stream_metadata.captured_sample = window.first_sample;
        staging.stream_metadata.end_sample = end_sample;
        staging.stream_metadata.source_underrun = u32::from(window.source_underrun);
        let target = with_host(handle, 0, AudioWorkletEngineHost::spectrum_target);
        let token = match window.sequence.checked_add(1) {
            Some(value) => value,
            None => {
                stream_metadata_error(
                    &mut staging,
                    SPECTRUM_STREAM_STATUS_FAILED,
                    RESULT_REFUSED_BUDGET,
                    Some(window.stream_epoch),
                    Some(window.dropped_captures),
                );
                return RESULT_REFUSED_BUDGET;
            }
        };
        write_spectrum_window(
            &mut staging,
            &window.as_window(),
            target,
            cadence.sample_rate_hz(),
            token,
        )
    })
}

/// Analyze the most recently read managed stream window using the worker-side smoothing history.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_stream_analysis() -> u32 {
    SPECTRUM_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        if !staging.stream_active {
            return RESULT_WRONG_STATE;
        }
        run_spectrum_analysis(&mut staging, true)
    })
}

/// Import one ready stream window's metadata into a worker-side staging instance.
///
/// A browser analysis worker has no render-host handle, so it cannot call the host-backed stream
/// start/read exports. It writes the capture metadata into the existing fixed record and calls
/// this export before [`miso_engine_web_v1_spectrum_stream_analysis`].
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_stream_analysis_configure() -> u32 {
    SPECTRUM_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        let (cadence, smoothing) = match imported_stream_configuration(&staging) {
            Ok(value) => value,
            Err(result) => return result,
        };
        if staging.ensure_stream_analysis_storage().is_err() {
            return RESULT_REFUSED_BUDGET;
        }
        staging.stream_active = true;
        staging.stream_cadence = Some(cadence);
        staging.stream_smoothing = Some(smoothing);
        staging.stream_window = Some(SpectrumStreamWindowFacts {
            stream_epoch: staging.stream_metadata.capture_epoch,
            sequence: staging.stream_metadata.sequence,
            dropped_captures: staging.stream_metadata.dropped_captures,
        });
        RESULT_OK
    })
}

/// Reset managed spectrum analysis history without stopping the native capture stream.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_stream_reset() -> u32 {
    SPECTRUM_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        if !staging.stream_active {
            return RESULT_WRONG_STATE;
        }
        staging
            .reset_stream_analysis()
            .map_or(RESULT_REFUSED_BUDGET, |_| RESULT_OK)
    })
}

/// Stop the managed native stream and retain its final normalized profile in metadata.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_stream_stop(handle: u32) -> u32 {
    match stream_host_dispatch(handle) {
        Ok(true) => return protected_spectrum_stream_stop(handle),
        Ok(false) => {}
        Err(result) => return result,
    }
    SPECTRUM_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        let result = with_host_mut(
            handle,
            RESULT_INVALID_ARGUMENT,
            AudioWorkletEngineHost::stop_spectrum_stream,
        );
        if result == RESULT_OK {
            staging.stream_active = false;
            staging.stream_cadence = None;
            staging.stream_smoothing = None;
            staging.capture_len = 0;
            staging.result_len = 0;
            staging.stream_window = None;
            staging.stream_metadata.result = RESULT_OK;
            staging.stream_metadata.status = SPECTRUM_STREAM_STATUS_STOPPED;
        } else {
            staging.stream_metadata.result = result;
        }
        result
    })
}

/// Stop one protected managed stream through the admitted native seam.
///
/// A pending stop is a scalar identity shortcut and therefore runs before removal admission. All
/// other protected stops spend exactly one removal credit and call native stop exactly once. Raw
/// lengths are invalidated only after native acceptance; the backing bytes and host capture
/// identity remain untouched for C2b's read/packing path.
fn protected_spectrum_stream_stop(handle: u32) -> u32 {
    const OPERATION: u32 = OBSERVATION_OPERATION_STOP_SPECTRUM;
    LIVE_HOST.with(|slot| {
        let Ok(mut slot) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        let Some(live) = slot.as_mut().filter(|live| live.handle == handle) else {
            return RESULT_INVALID_ARGUMENT;
        };
        let host = &mut live.host;

        if let Some(receipt) = host.pending_stop_receipt() {
            return SPECTRUM_STAGING.with(|staging_slot| {
                let Ok(mut staging) = staging_slot.try_borrow_mut() else {
                    return RESULT_INTERNAL;
                };
                host.record_observation_admission(OPERATION, RESULT_OK, None, Some(receipt), true);
                staging.stream_active = false;
                staging.stream_cadence = None;
                staging.stream_smoothing = None;
                staging.capture_len = 0;
                staging.result_len = 0;
                staging.stream_window = None;
                staging.stream_metadata.result = RESULT_OK;
                staging.stream_metadata.status = SPECTRUM_STREAM_STATUS_STOPPED;
                RESULT_OK
            });
        }

        SPECTRUM_STAGING.with(|staging_slot| {
            let Ok(mut staging) = staging_slot.try_borrow_mut() else {
                return RESULT_INTERNAL;
            };
            let permit = match host.begin_observation(
                ObservationClass::Removal,
                AudioWorkletEngineHost::protected_spectrum_control_lengths(true, 0),
            ) {
                Ok(permit) => permit,
                Err(refusal) => return host.record_observation_refusal(OPERATION, refusal),
            };
            let result = host.stop_spectrum_stream_admitted(permit);
            if result != RESULT_OK {
                return result;
            }
            staging.stream_active = false;
            staging.stream_cadence = None;
            staging.stream_smoothing = None;
            staging.capture_len = 0;
            staging.result_len = 0;
            staging.stream_window = None;
            staging.stream_metadata.result = RESULT_OK;
            staging.stream_metadata.status = SPECTRUM_STREAM_STATUS_STOPPED;
            RESULT_OK
        })
    })
}

/// Return the fixed managed-stream metadata record address.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_stream_metadata_ptr() -> u32 {
    SPECTRUM_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return 0;
        };
        staging.stream_metadata.struct_size = SPECTRUM_STREAM_METADATA_BYTES;
        staging.stream_metadata.abi_version = ABI_VERSION;
        pointer_u32(ptr::from_mut(&mut staging.stream_metadata))
    })
}

/// Return the fixed managed-stream metadata record byte size.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_stream_metadata_bytes() -> u32 {
    SPECTRUM_STREAM_METADATA_BYTES
}

/// Cancel the prepared spectrum observer and discard any completed window.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_cancel(handle: u32) -> u32 {
    match refuse_protected_observation_alias(handle, OBSERVATION_OPERATION_ONE_SHOT) {
        Err(result) | Ok(Some(result)) => return result,
        Ok(None) => {}
    }
    SPECTRUM_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        if staging.stream_active {
            staging.stream_metadata.result = RESULT_WRONG_STATE;
            return RESULT_WRONG_STATE;
        }
        staging.capture_len = 0;
        with_host_mut(
            handle,
            RESULT_INVALID_ARGUMENT,
            AudioWorkletEngineHost::cancel_spectrum,
        )
    })
}

/// Return a writable request header for one live selected-track response query.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_request_ptr() -> u32 {
    RESPONSE_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return 0;
        };
        staging.live_result_len = 0;
        staging.live_request.struct_size = LIVE_RESPONSE_REQUEST_BYTES;
        staging.live_request.abi_version = ABI_VERSION;
        pointer_u32(ptr::from_mut(&mut *staging.live_request))
    })
}

/// Return the fixed live response request-header byte size.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_request_bytes() -> u32 {
    LIVE_RESPONSE_REQUEST_BYTES
}

/// Return the fixed raw snapshot staging address used by capture and analysis.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_snapshot_ptr() -> u32 {
    RESPONSE_STAGING.with(|slot| {
        slot.try_borrow()
            .ok()
            .map_or(0, |staging| pointer_u32(staging.live_result.as_ptr()))
    })
}

/// Return the maximum byte length of the fixed raw snapshot staging area.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_snapshot_capacity() -> u32 {
    u32::try_from(LIVE_RESPONSE_CAPTURE_BYTES).unwrap_or(0)
}

/// Set the byte length of a raw snapshot copied into the fixed staging area.
///
/// The analysis Worker copies the Worklet's capture reply into this same staging area before
/// calling [`miso_engine_web_v1_track_response_analysis`].  The length is explicit so malformed
/// or truncated replies are rejected by the Rust decoder rather than inferred from stale bytes.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_snapshot_set_bytes(bytes: u32) -> u32 {
    RESPONSE_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        let Ok(length) = usize::try_from(bytes) else {
            return RESULT_INVALID_ARGUMENT;
        };
        if !(LIVE_RESPONSE_RESULT_BYTES as usize..=LIVE_RESPONSE_CAPTURE_BYTES).contains(&length) {
            return RESULT_INVALID_ARGUMENT;
        }
        staging.live_result_len = length;
        RESULT_OK
    })
}

/// Return writable staging for the selected track's UTF-8 identity.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_track_id_ptr() -> u32 {
    RESPONSE_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return 0;
        };
        pointer_u32(staging.live_track_id.as_mut_ptr())
    })
}

/// Return the maximum selected-track identity byte length.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_track_id_capacity() -> u32 {
    LIVE_RESPONSE_MAXIMUM_ID_BYTES as u32
}

/// Capture one selected track's live response at the current boundary into raw staging.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_capture(handle: u32) -> u32 {
    LIVE_HOST.with(|host_slot| {
        let Ok(mut live) = host_slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        let Some(live) = live.as_mut().filter(|value| value.handle == handle) else {
            return RESULT_WRONG_STATE;
        };
        let protected = live.host.protected_observation_prepared();
        RESPONSE_STAGING.with(|staging_slot| {
            let Ok(mut staging) = staging_slot.try_borrow_mut() else {
                return RESULT_INTERNAL;
            };
            if protected {
                run_protected_live_response_capture(&mut live.host, &mut staging)
            } else {
                run_live_response_capture(&mut live.host, &mut staging)
            }
        })
    })
}

/// Analyze one staged raw live-response snapshot using the native Rust response composer.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_analysis() -> u32 {
    RESPONSE_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        run_live_response_analysis(&mut staging)
    })
}

/// Return the live response result payload address.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_result_ptr() -> u32 {
    RESPONSE_STAGING.with(|slot| {
        slot.try_borrow()
            .ok()
            .map_or(0, |staging| pointer_u32(staging.live_result.as_ptr()))
    })
}

/// Return the live response result payload byte length.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_result_bytes() -> u32 {
    RESPONSE_STAGING.with(|slot| {
        slot.try_borrow()
            .ok()
            .and_then(|staging| u32::try_from(staging.live_result_len).ok())
            .unwrap_or(0)
    })
}

/// Release the live response staging payload.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_close() -> u32 {
    RESPONSE_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        staging.live_result_len = 0;
        RESULT_OK
    })
}

/// Open the bounded EQ target-preparation workspace.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_eq_target_open() -> u32 {
    crate::control_targets::open()
}

/// Return the writable EQ target request buffer.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_eq_target_request_ptr() -> u32 {
    crate::control_targets::request_ptr()
}

/// Return the EQ target request capacity.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_eq_target_request_capacity() -> u32 {
    crate::control_targets::request_capacity()
}

/// Prepare the staged EQ target request.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_eq_target_prepare(request_bytes: u32) -> u32 {
    crate::control_targets::prepare(request_bytes)
}

/// Return the EQ target result buffer.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_eq_target_result_ptr() -> u32 {
    crate::control_targets::result_ptr()
}

/// Return the current EQ target result length.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_eq_target_result_bytes() -> u32 {
    crate::control_targets::result_bytes()
}

/// Return the EQ target result capacity.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_eq_target_result_capacity() -> u32 {
    crate::control_targets::result_capacity()
}

/// Return the original edit index from the most recent refusal.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_eq_target_rejected_edit_index() -> u32 {
    crate::control_targets::rejected_edit_index()
}

/// Return the existing command reason from the most recent EQ preparation refusal.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_eq_target_rejected_reason() -> u32 {
    crate::control_targets::rejected_reason()
}

/// Close the EQ target-preparation workspace.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_eq_target_close() -> u32 {
    crate::control_targets::close()
}

/// Prepare the staged builtin input-filter target request.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_input_filters_prepare(request_bytes: u32) -> u32 {
    crate::control_targets::input_filter_prepare(request_bytes)
}

/// Return the frozen browser-Wasm ABI version.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_abi_version() -> u32 {
    ABI_VERSION
}

/// Return the module-owned, zero-default boot-options address.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_boot_options_ptr() -> u32 {
    BOOT_STAGING.with(|staging| {
        let Ok(mut staging) = staging.try_borrow_mut() else {
            return 0;
        };
        pointer_u32(ptr::from_mut(&mut *staging.options))
    })
}

/// Return the fixed protected-observation preparation record address.
///
/// The protected boot transaction remains private in this checkpoint. This pointer is therefore
/// only a control-side staging address; publishing it does not publish a protected boot entry
/// point or create an owner.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_preparation_ptr() -> u32 {
    OBSERVATION_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return 0;
        };
        pointer_u32(ptr::from_mut(&mut staging.endpoint.preparation))
    })
}

/// Return the actual protected-observation preparation record size.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_preparation_bytes() -> u32 {
    u32::try_from(size_of::<WebObservationPreparationRecord>()).unwrap_or(0)
}

/// Return the fixed protected-observation demand record address.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_demand_ptr(handle: u32) -> u32 {
    pointer_u32(observation_demand_ptr_for_handle(handle))
}

fn observation_demand_ptr_for_handle(handle: u32) -> *mut WebObservationDemand {
    if handle == 0 {
        return ptr::null_mut();
    }
    // The demand address is live-owner input staging. It must never be exposed for a disposed,
    // unrelated, or otherwise invalid handle, even when the endpoint retains a terminal snapshot.
    LIVE_HOST.with(|live_slot| {
        let Ok(live_slot) = live_slot.try_borrow() else {
            return ptr::null_mut();
        };
        let Some(live) = live_slot.as_ref().filter(|live| live.handle == handle) else {
            return ptr::null_mut();
        };
        let _ = live;
        OBSERVATION_STAGING.with(|slot| {
            let Ok(mut staging) = slot.try_borrow_mut() else {
                return ptr::null_mut();
            };
            ptr::from_mut(&mut staging.endpoint.demand)
        })
    })
}

/// Return the actual protected-observation demand record size.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_demand_bytes() -> u32 {
    u32::try_from(size_of::<WebObservationDemand>()).unwrap_or(0)
}

/// Meter-row capacity for the scalar protected-observation demand endpoint.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_demand_capacity() -> u32 {
    0
}

fn observation_demand_class(operation: u32) -> ObservationClass {
    match operation {
        OBSERVATION_OPERATION_REMOVE_METERS_TO | OBSERVATION_OPERATION_STOP_GRAPH => {
            ObservationClass::Removal
        }
        _ => ObservationClass::Ordinary,
    }
}

fn observation_demand_refusal(
    host: &mut AudioWorkletEngineHost,
    operation: u32,
    reason: ObservationRefusalReason,
) -> u32 {
    host.record_observation_refusal(
        operation,
        ObservationRefusal {
            reason,
            limit: None,
            requested: None,
            maximum: None,
        },
    )
}

fn apply_observation_demand(
    host: &mut AudioWorkletEngineHost,
    demand: WebObservationDemand,
) -> u32 {
    let operation = demand.operation;
    let class = observation_demand_class(operation);
    let lengths = ObservationLengths {
        control_bytes: size_of::<WebObservationDemand>() as u64,
        rows: 0,
        result_bytes: 0,
    };
    let (owner, _) = match host.protected_observation_identity() {
        Ok(identity) => identity,
        Err(refusal) => return host.record_observation_refusal(operation, refusal),
    };
    let header_valid = demand.struct_size == size_of::<WebObservationDemand>() as u32
        && demand.abi_version == ABI_VERSION
        && demand.reserved == [0; 2];

    // These meter operations are known but unsupported in V1. Validate their fixed header and
    // owner first so their diagnostics retain the established precedence, then refuse without
    // acquiring either protected credit.
    if matches!(
        operation,
        crate::OBSERVATION_OPERATION_REPLACE_METERS | OBSERVATION_OPERATION_REMOVE_METERS_TO
    ) {
        if !header_valid {
            return observation_demand_refusal(
                host,
                operation,
                ObservationRefusalReason::InvalidRequest,
            );
        }
        if demand.owner != owner.get() {
            return observation_demand_refusal(
                host,
                operation,
                ObservationRefusalReason::WrongOwner,
            );
        }
        return host.refuse_unsupported_observation(operation);
    }

    let valid_stop = header_valid
        && operation == OBSERVATION_OPERATION_STOP_GRAPH
        && demand.count == 0
        && demand.owner == owner.get();

    // A repeated pending stop is an identity query on the already-admitted native operation. It
    // is reachable only after every fixed field and the owner have matched, so malformed and
    // wrong-owner records still spend their classified attempt below.
    if valid_stop && let Some(receipt) = host.pending_stop_receipt() {
        // The pending receipt is authoritative, but the preceding admission may have been
        // replaced by a refusal. Rebuild the successful pending projection and retag only
        // its wire operation; never copy that arbitrary refusal's result or reason.
        host.record_observation_admission(
            OBSERVATION_OPERATION_STOP_GRAPH,
            RESULT_OK,
            None,
            Some(receipt),
            true,
        );
        return RESULT_OK;
    }

    let permit = match host.begin_observation(class, lengths) {
        Ok(permit) => permit,
        Err(refusal) => return host.record_observation_refusal(operation, refusal),
    };
    if !header_valid {
        return observation_demand_refusal(
            host,
            operation,
            ObservationRefusalReason::InvalidRequest,
        );
    }
    if demand.owner != owner.get() {
        return observation_demand_refusal(host, operation, ObservationRefusalReason::WrongOwner);
    }
    if operation != OBSERVATION_OPERATION_STOP_GRAPH {
        host.record_observation_admission(
            operation,
            RESULT_UNSUPPORTED,
            Some(ObservationRefusal {
                reason: ObservationRefusalReason::InvalidRequest,
                limit: None,
                requested: None,
                maximum: None,
            }),
            None,
            false,
        );
        return RESULT_UNSUPPORTED;
    }
    if demand.count != 0 {
        return observation_demand_refusal(
            host,
            operation,
            ObservationRefusalReason::InvalidRequest,
        );
    }

    // The native helper owns receipt reservation and the single stop publication. Its admission
    // record remains authoritative; only the wire operation tag is retagged after that call.
    let result = host.stop_spectrum_stream_admitted(permit);
    if host.observation_admission().operation == OBSERVATION_OPERATION_STOP_SPECTRUM {
        host.side_records.admission.operation = OBSERVATION_OPERATION_STOP_GRAPH;
    }
    result
}

/// Apply one fixed protected-observation demand record to a live matching protected owner.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_demand_apply(handle: u32) -> u32 {
    if handle == 0 {
        return RESULT_INVALID_ARGUMENT;
    }
    OBSERVATION_STAGING.with(|slot| {
        let Ok(staging) = slot.try_borrow() else {
            return RESULT_INTERNAL;
        };
        let demand = staging.endpoint.demand;
        with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
            apply_observation_demand(host, demand)
        })
    })
}

fn observation_admission_ptr_for_handle(handle: u32) -> u32 {
    pointer_u32(observation_admission_ptr_for_handle_raw(handle))
}

fn observation_admission_ptr_for_handle_raw(handle: u32) -> *mut WebObservationAdmission {
    if handle == 0 {
        return ptr::null_mut();
    }
    OBSERVATION_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return ptr::null_mut();
        };
        // A failed LIVE_HOST borrow is distinct from an empty slot. In either case the query
        // refuses while a live slot exists or cannot be inspected; only a proven empty slot may
        // fall through to the retained terminal mirror.
        LIVE_HOST.with(|live_slot| {
            let Ok(live_slot) = live_slot.try_borrow() else {
                return ptr::null_mut();
            };
            match live_slot.as_ref() {
                Some(live) if live.handle == handle => {
                    staging.endpoint.admission = *live.host.observation_admission();
                    ptr::from_mut(&mut staging.endpoint.admission)
                }
                Some(_) => ptr::null_mut(),
                None if staging.endpoint.handle == handle => {
                    ptr::from_mut(&mut staging.endpoint.admission)
                }
                None => ptr::null_mut(),
            }
        })
    })
}

fn observation_status_ptr_for_handle(handle: u32) -> u32 {
    pointer_u32(observation_status_ptr_for_handle_raw(handle))
}

fn observation_status_ptr_for_handle_raw(handle: u32) -> *mut WebObservationStatus {
    if handle == 0 {
        return ptr::null_mut();
    }
    OBSERVATION_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return ptr::null_mut();
        };
        LIVE_HOST.with(|live_slot| {
            let Ok(live_slot) = live_slot.try_borrow() else {
                return ptr::null_mut();
            };
            match live_slot.as_ref() {
                Some(live) if live.handle == handle => {
                    staging.endpoint.status = live.host.observation_status();
                    ptr::from_mut(&mut staging.endpoint.status)
                }
                Some(_) => ptr::null_mut(),
                None if staging.endpoint.handle == handle => {
                    ptr::from_mut(&mut staging.endpoint.status)
                }
                None => ptr::null_mut(),
            }
        })
    })
}

fn observation_capture_identity_ptr_for_handle(handle: u32) -> u32 {
    pointer_u32(observation_capture_identity_ptr_for_handle_raw(handle))
}

fn observation_capture_identity_ptr_for_handle_raw(
    handle: u32,
) -> *mut WebObservationCaptureIdentity {
    if handle == 0 {
        return ptr::null_mut();
    }
    OBSERVATION_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return ptr::null_mut();
        };
        LIVE_HOST.with(|live_slot| {
            let Ok(live_slot) = live_slot.try_borrow() else {
                return ptr::null_mut();
            };
            match live_slot.as_ref() {
                Some(live) if live.handle == handle => {
                    staging.endpoint.capture_identity = *live.host.observation_capture_identity();
                    ptr::from_mut(&mut staging.endpoint.capture_identity)
                }
                Some(_) => ptr::null_mut(),
                None if staging.endpoint.handle == handle => {
                    ptr::from_mut(&mut staging.endpoint.capture_identity)
                }
                None => ptr::null_mut(),
            }
        })
    })
}

/// Return the current or matching retained terminal admission record address.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_admission_ptr(handle: u32) -> u32 {
    observation_admission_ptr_for_handle(handle)
}

/// Return the actual protected-observation admission record size.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_admission_bytes() -> u32 {
    u32::try_from(size_of::<WebObservationAdmission>()).unwrap_or(0)
}

/// Return the current or matching retained terminal status record address.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_status_ptr(handle: u32) -> u32 {
    observation_status_ptr_for_handle(handle)
}

/// Return the actual protected-observation status record size.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_status_bytes() -> u32 {
    u32::try_from(size_of::<WebObservationStatus>()).unwrap_or(0)
}

/// Return the current or matching retained terminal capture-identity record address.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_capture_identity_ptr(handle: u32) -> u32 {
    observation_capture_identity_ptr_for_handle(handle)
}

/// Return the actual protected-observation capture-identity record size.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_capture_identity_bytes() -> u32 {
    u32::try_from(size_of::<WebObservationCaptureIdentity>()).unwrap_or(0)
}

fn observation_application_take_live(handle: u32, staging: &mut ObservationStaging) -> u32 {
    LIVE_HOST.with(|live_slot| {
        let Ok(mut live_slot) = live_slot.try_borrow_mut() else {
            return 0;
        };
        let Some(live) = live_slot.as_mut().filter(|live| live.handle == handle) else {
            return 0;
        };
        // The native helper performs the bounded reconciliation exactly once. Its returned slice
        // is stable until this host borrow ends, so copy the rows before releasing LIVE_HOST.
        let applications = live.host.take_observation_applications();
        let count = applications.len();
        debug_assert!(count <= staging.endpoint.applications.len());
        staging.endpoint.applications[..count].copy_from_slice(applications);
        staging.endpoint.application_count = u32::try_from(count).unwrap_or(0);
        staging.endpoint.terminal_application_pending = false;
        u32::try_from(count).unwrap_or(0)
    })
}

/// Transfer completed protected-observation receipts into the fixed ABI staging array.
///
/// A matching disposed handle receives the one retained terminal handoff. Repeated takes return
/// zero without clearing the previous rows, which keeps the fixed address stable for a caller that
/// has already observed the terminal batch. A cold invalid handle returns before touching the
/// allocating observation staging TLS.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_application_take(handle: u32) -> u32 {
    if handle == 0 {
        return 0;
    }

    // Distinguish a live match, a live mismatch, an empty slot, and a borrow conflict before
    // touching OBSERVATION_STAGING. The zero NEXT_HANDLE value is the never-issued sentinel and
    // makes a cold invalid take allocation-free.
    let live_state = LIVE_HOST.with(|live_slot| {
        let Ok(live_slot) = live_slot.try_borrow_mut() else {
            return 0_u8;
        };
        match live_slot.as_ref() {
            Some(live) if live.handle == handle => 1,
            Some(_) => 2,
            None => 3,
        }
    });
    match live_state {
        0 | 2 => return 0,
        1 => {
            return OBSERVATION_STAGING.with(|slot| {
                let Ok(mut staging) = slot.try_borrow_mut() else {
                    return 0;
                };
                observation_application_take_live(handle, &mut staging)
            });
        }
        3 => {}
        _ => unreachable!(),
    }

    if NEXT_HANDLE.with(Cell::get) == 0 {
        return 0;
    }
    OBSERVATION_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return 0;
        };
        if staging.endpoint.handle != handle {
            return 0;
        }
        if !staging.endpoint.terminal_application_pending {
            // The terminal rows remain in the fixed bytes for stable caller inspection, while a
            // repeated take reports an empty batch through both its return value and count.
            staging.endpoint.application_count = 0;
            return 0;
        }
        staging.endpoint.terminal_application_pending = false;
        staging.endpoint.application_count
    })
}

/// Return the fixed protected-observation application row staging address.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_application_ptr() -> u32 {
    OBSERVATION_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return 0;
        };
        pointer_u32(staging.endpoint.applications.as_mut_ptr())
    })
}

/// Return the actual protected-observation application row size.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_application_bytes() -> u32 {
    u32::try_from(size_of::<WebObservationReceipt>()).unwrap_or(0)
}

/// Return the fixed protected-observation application row capacity.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_application_capacity() -> u32 {
    4
}

/// Stage an exact-length document before boot. Refuses lengths above the engine bound.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_document_ptr(len: u32) -> u32 {
    let live = LIVE_HOST.with(|slot| slot.try_borrow().map_or(true, |slot| slot.is_some()));
    BOOT_STAGING.with(|staging| {
        let Ok(mut staging) = staging.try_borrow_mut() else {
            return 0;
        };
        if live {
            staging.result = RESULT_REFUSED_LIFECYCLE;
            staging.diagnostic_bytes = 0;
            staging.document_valid = false;
            return 0;
        }
        if len > MAXIMUM_DOCUMENT_BYTES {
            staging.result = RESULT_REFUSED_DOCUMENT;
            staging.diagnostic_bytes = 0;
            staging.document_valid = false;
            return 0;
        }
        let count = len as usize;
        let mut document = Vec::new();
        if document.try_reserve_exact(count).is_err() {
            staging.result = RESULT_REFUSED_BUDGET;
            staging.diagnostic_bytes = 0;
            staging.document_valid = false;
            return 0;
        }
        document.resize(count, 0);
        staging.document = document;
        staging.result = RESULT_OK;
        staging.diagnostic_bytes = 0;
        staging.document_valid = true;
        pointer_u32(staging.document.as_mut_ptr())
    })
}

/// Return one prepared stable staging-buffer address or zero.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_buffer_ptr(handle: u32, kind: u32) -> u32 {
    with_host_mut(handle, 0, |host| pointer_u32(buffer_pointer(host, kind)))
}

/// Return one prepared staging-buffer capacity in bytes or zero.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_buffer_capacity(handle: u32, kind: u32) -> u32 {
    with_host(handle, 0, |host| buffer_capacity(host, kind))
}

#[derive(Clone, Copy)]
enum StagedBootMode {
    Legacy,
    ProtectedObservation,
}

impl ObservationEndpointStaging {
    fn reset_outputs(&mut self) {
        self.demand = WebObservationDemand {
            struct_size: size_of::<WebObservationDemand>() as u32,
            abi_version: ABI_VERSION,
            ..WebObservationDemand::default()
        };
        self.admission = WebObservationAdmission::default();
        self.status = WebObservationStatus::default();
        self.capture_identity = WebObservationCaptureIdentity::default();
        self.applications = [WebObservationReceipt::default(); 4];
        self.application_count = 0;
        self.handle = 0;
        self.terminal_application_pending = false;
    }
}

fn prewarm_observation_staging() -> Result<(), BootFailure> {
    OBSERVATION_STAGING.with(|slot| {
        slot.try_borrow_mut()
            .map(|_| ())
            .map_err(|_| BootFailure::fixed(RESULT_INTERNAL, "web.observation.staging"))
    })?;
    RESPONSE_STAGING.with(|slot| {
        slot.try_borrow_mut()
            .map(|_| ())
            .map_err(|_| BootFailure::fixed(RESULT_INTERNAL, "web.response.staging"))
    })?;
    SPECTRUM_STAGING.with(|slot| {
        let mut staging = slot
            .try_borrow_mut()
            .map_err(|_| BootFailure::fixed(RESULT_INTERNAL, "web.spectrum.staging"))?;
        staging
            .configure_capture()
            .map_err(|result| BootFailure::fixed(result, "web.spectrum.staging"))
    })
}

fn boot_staged(len: u32, mode: StagedBootMode) -> u32 {
    let already_live = LIVE_HOST.with(|slot| slot.try_borrow().map_or(true, |slot| slot.is_some()));
    if already_live {
        BOOT_STAGING.with(|staging| {
            if let Ok(mut staging) = staging.try_borrow_mut() {
                staging.result = RESULT_REFUSED_LIFECYCLE;
                staging.diagnostic_bytes = 0;
                staging.document_valid = false;
            }
        });
        return 0;
    }
    let booted = BOOT_STAGING.with(|staging| {
        let Ok(mut staging) = staging.try_borrow_mut() else {
            return None;
        };
        if !staging.document_valid || staging.document.len() != len as usize {
            staging.result = RESULT_REFUSED_DOCUMENT;
            staging.diagnostic_bytes = 0;
            staging.document_valid = false;
            return None;
        }

        let result = match mode {
            StagedBootMode::Legacy => {
                let spectrum_request = SPECTRUM_STAGING.with(|spectrum| {
                    let Ok(mut spectrum) = spectrum.try_borrow_mut() else {
                        return Err(BootFailure::fixed(RESULT_INTERNAL, "web.spectrum.staging"));
                    };
                    let request = staged_spectrum_request(&spectrum)
                        .map_err(|result| BootFailure::fixed(result, "web.spectrum.request"))?;
                    if request.is_some() {
                        spectrum
                            .configure_capture()
                            .map_err(|result| BootFailure::fixed(result, "web.spectrum.staging"))?;
                    } else {
                        spectrum.release_capture();
                    }
                    Ok(request)
                });
                spectrum_request.and_then(|request| {
                    AudioWorkletEngineHost::boot_with_spectrum(
                        &staging.document,
                        *staging.options,
                        request,
                    )
                })
            }
            StagedBootMode::ProtectedObservation => OBSERVATION_STAGING.with(|slot| {
                let Ok(staged) = slot.try_borrow() else {
                    return Err(BootFailure::fixed(
                        RESULT_INTERNAL,
                        "web.observation.staging",
                    ));
                };
                let record = &staged.endpoint.preparation;
                validate_observation_preparation_record(record)
                    .map_err(|result| BootFailure::fixed(result, "web.observation.preparation"))?;
                let request = record.spectrum_request;
                let maximum_active_observers = usize::try_from(record.maximum_active_observers)
                    .map_err(|_| {
                        BootFailure::fixed(
                            RESULT_INVALID_ARGUMENT,
                            "web.observation.maximum_active_observers",
                        )
                    })?;
                let target_bytes = usize::try_from(request.target_id_bytes).map_err(|_| {
                    BootFailure::fixed(RESULT_INVALID_ARGUMENT, "web.observation.target")
                })?;
                let target =
                    core::str::from_utf8(&record.target_id[..target_bytes]).map_err(|_| {
                        BootFailure::fixed(RESULT_INVALID_ARGUMENT, "web.observation.target")
                    })?;
                let mut entries = Vec::new();
                entries.try_reserve_exact(1).map_err(|_| {
                    BootFailure::fixed(RESULT_REFUSED_BUDGET, "web.observation.spectrum")
                })?;
                entries.push(SpectrumCaptureCollectionEntry {
                    target: SpectrumTarget::TrackPostMatrix(target.into()),
                    channels: SpectrumChannels::Stereo,
                });
                let spectrum = SpectrumCaptureCollectionRequest {
                    entries,
                    maximum_capture_bytes: request.maximum_capture_bytes,
                };
                let work = record.work_limits;
                let ingress = record.ingress_limits;
                let preparation = WebObservationPreparation {
                    profile: WebObservationProfile::EqSpectrum,
                    demand: host_core::HostObservationPreparation {
                        meters: &[],
                        spectrum: Some(&spectrum),
                        work_limits: ObservationWorkLimits {
                            maximum_active_meter_channels: work.maximum_active_meter_channels,
                            maximum_meter_samples_per_block: work.maximum_meter_samples_per_block,
                            maximum_meter_publications_per_block: work
                                .maximum_meter_publications_per_block,
                            maximum_meter_publication_bytes_per_block: work
                                .maximum_meter_publication_bytes_per_block,
                            maximum_active_spectrum_captures: work.maximum_active_spectrum_captures,
                            maximum_capture_input_samples_per_block: work
                                .maximum_capture_input_samples_per_block,
                            maximum_capture_copy_samples_per_block: work
                                .maximum_capture_copy_samples_per_block,
                            maximum_capture_publications_per_block: work
                                .maximum_capture_publications_per_block,
                            maximum_capture_bytes_per_second: work.maximum_capture_bytes_per_second,
                            maximum_transition_entry_visits_per_block: work
                                .maximum_transition_entry_visits_per_block,
                            maximum_retained_bytes: work.maximum_retained_bytes,
                        },
                        activation: GraphObservationActivationConfig {
                            maximum_active_observers,
                            maximum_retained_bytes: record.activation_maximum_retained_bytes,
                        },
                    },
                    ingress: ObservationIngressLimits {
                        maximum_control_bytes: ingress.maximum_control_bytes,
                        maximum_observation_rows: ingress.maximum_observation_rows,
                        maximum_result_bytes: ingress.maximum_result_bytes,
                        ordinary_operations_per_boundary: ingress.ordinary_operations_per_boundary,
                        removal_operations_per_boundary: ingress.removal_operations_per_boundary,
                        maximum_admission_entry_visits: ingress.maximum_admission_entry_visits,
                        maximum_response_binding_visits: ingress.maximum_response_binding_visits,
                        maximum_response_section_visits: ingress.maximum_response_section_visits,
                        maximum_response_copy_bytes: ingress.maximum_response_copy_bytes,
                        maximum_handler_copy_bytes_per_boundary: ingress
                            .maximum_handler_copy_bytes_per_boundary,
                        maximum_cleanup_entry_visits_per_boundary: ingress
                            .maximum_cleanup_entry_visits_per_boundary,
                        maximum_retained_bytes: ingress.maximum_retained_bytes,
                    },
                };
                AudioWorkletEngineHost::boot_with_observation_demand(
                    &staging.document,
                    *staging.options,
                    &preparation,
                )
            }),
        };

        match result {
            Ok(host) => {
                if matches!(mode, StagedBootMode::ProtectedObservation)
                    && let Err(failure) = prewarm_observation_staging()
                {
                    staging.record_failure(failure);
                    return None;
                }
                staging.result = RESULT_OK;
                staging.diagnostic_bytes = 0;
                staging.document_valid = false;
                Some(host)
            }
            Err(failure) => {
                staging.record_failure(failure);
                None
            }
        }
    });
    let Some(host) = booted else {
        SPECTRUM_STAGING.with(|slot| {
            if let Ok(mut staging) = slot.try_borrow_mut() {
                staging.release_capture();
            }
        });
        return 0;
    };
    let initial_status = host.observation_status();
    let mut host = Some(host);
    // Acquire endpoint staging before publishing LIVE_HOST. A conflicting borrow is a real boot
    // failure, and the prior terminal handoff remains intact because reset_outputs runs only on a
    // successful publication.
    let handle = OBSERVATION_STAGING.with(|observation_slot| {
        let Ok(mut observation) = observation_slot.try_borrow_mut() else {
            return None;
        };
        LIVE_HOST.with(|live_slot| {
            let Ok(mut live_slot) = live_slot.try_borrow_mut() else {
                return None;
            };
            if live_slot.is_some() {
                return None;
            }
            let handle = next_handle();
            observation.endpoint.reset_outputs();
            observation.endpoint.status = initial_status;
            observation.endpoint.handle = handle;
            *live_slot = Some(LiveHost {
                handle,
                host: Box::new(
                    host.take()
                        .expect("boot host is present before publication"),
                ),
            });
            Some(handle)
        })
    });
    let Some(handle) = handle else {
        if let Some(mut host) = host.take() {
            let _ = host.dispose();
        }
        BOOT_STAGING.with(|staging| {
            if let Ok(mut staging) = staging.try_borrow_mut() {
                staging.record_failure(BootFailure::fixed(
                    RESULT_INTERNAL,
                    "web.observation.staging",
                ));
            }
        });
        SPECTRUM_STAGING.with(|slot| {
            if let Ok(mut staging) = slot.try_borrow_mut() {
                staging.release_capture();
            }
        });
        return 0;
    };
    handle
}

/// Private protected staged boot used by the checkpoint-A fixtures.
#[allow(dead_code)]
pub(crate) fn boot_staged_observation_demand(len: u32) -> u32 {
    boot_staged(len, StagedBootMode::ProtectedObservation)
}

/// Boot the exact staged document with the protected observation preparation.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_boot_with_observation_demand(len: u32) -> u32 {
    boot_staged_observation_demand(len)
}

/// Boot the exact staged document and atomically publish the sole running handle.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_boot(len: u32) -> u32 {
    boot_staged(len, StagedBootMode::Legacy)
}

/// Return the frozen result code of the last boot attempt.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_boot_result() -> u32 {
    BOOT_STAGING.with(|staging| {
        staging
            .try_borrow()
            .map_or(RESULT_INTERNAL, |staging| staging.result)
    })
}

/// Return the valid diagnostic prefix that replaced the refused staged document.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_boot_diagnostic_bytes() -> u32 {
    BOOT_STAGING.with(|staging| {
        staging
            .try_borrow()
            .map_or(0, |staging| staging.diagnostic_bytes)
    })
}

/// Submit one staged planar source chunk to the named prepared source.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_source_submit(
    handle: u32,
    source_id_bytes: u32,
    generation: u64,
    start_frame: u64,
    channels: u32,
    frames: u32,
    end_of_region: u32,
) -> u32 {
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        if end_of_region > 1 || host.status().state != STATE_READY {
            return if end_of_region > 1 {
                host.record_boundary_result(RESULT_INVALID_ARGUMENT)
            } else {
                host.submit_source(&[], 0, 0, 0, &[], 0, false)
            };
        }
        let quantum = host.status().quantum_frames as usize;
        let channel_count = channels as usize;
        let frame_count = frames as usize;
        let id_count = source_id_bytes as usize;
        let sample_rate = host.status().sample_rate_hz;
        let Some((pcm, plane_slots, ids)) = host.ffi_source_staging_mut() else {
            return host.record_boundary_result(RESULT_INTERNAL);
        };
        if channel_count == 0
            || channel_count > plane_slots.len()
            || frame_count > quantum
            || id_count > ids.len()
        {
            return host.record_boundary_result(RESULT_INVALID_ARGUMENT);
        }
        let Some(required_samples) = channel_count.checked_mul(quantum) else {
            return host.record_boundary_result(RESULT_INVALID_ARGUMENT);
        };
        if required_samples > pcm.len() {
            return host.record_boundary_result(RESULT_INVALID_ARGUMENT);
        }
        let pcm_pointer = pcm.as_ptr();
        let id_pointer = ids.as_ptr();
        for (channel, slot) in plane_slots[..channel_count].iter_mut().enumerate() {
            let offset = channel * quantum;
            // SAFETY: Preparation allocated `maximum_source_channels * quantum` stable PCM
            // samples. The checked channel bound/product and `frames <= quantum` prove this
            // exact plane prefix is readable for the synchronous source submission below.
            let plane = unsafe { slice::from_raw_parts(pcm_pointer.add(offset), frame_count) };
            slot.write(plane);
        }
        let plane_pointer = plane_slots.as_ptr().cast::<&[f32]>();
        // SAFETY: Every element in this exact prefix was initialized above with a valid slice
        // into stable PCM staging. `MaybeUninit<T>` has the same layout as `T`; the borrow ends
        // before this function returns and the host does not mutate staging during submission.
        let planes = unsafe { slice::from_raw_parts(plane_pointer, channel_count) };
        // SAFETY: The checked ID prefix lies in stable source-ID staging. It is read only for
        // the synchronous lookup and is not retained by the safe host.
        let source_id = unsafe { slice::from_raw_parts(id_pointer, id_count) };
        let result = host.submit_source(
            source_id,
            generation,
            start_frame,
            sample_rate,
            planes,
            frames,
            end_of_region == 1,
        );
        if let Some((_, slots, _)) = host.ffi_source_staging_mut() {
            for slot in &mut slots[..channel_count] {
                *slot = MaybeUninit::uninit();
            }
        }
        result
    })
}

/// Queue one generation-tagged absolute seek for the staged source ID.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_source_seek(
    handle: u32,
    source_id_bytes: u32,
    generation: u64,
    source_frame: u64,
) -> u32 {
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        if host.status().state != STATE_READY {
            return host.seek_source(&[], 0, 0);
        }
        let id_count = source_id_bytes as usize;
        let Some(ids) = host.source_id_mut() else {
            return host.record_boundary_result(RESULT_INTERNAL);
        };
        let Some(source_id) = ids.get(..id_count) else {
            return host.record_boundary_result(RESULT_INVALID_ARGUMENT);
        };
        let source_id_pointer = source_id.as_ptr();
        // SAFETY: The checked ID prefix lies in stable staging and is only read during the
        // synchronous safe-host lookup; the seek operation does not mutate staging.
        let source_id = unsafe { slice::from_raw_parts(source_id_pointer, id_count) };
        host.seek_source(source_id, generation, source_frame)
    })
}

/// Render one exact prepared quantum after validating the browser's actual frame count.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_render(handle: u32, actual_frames: u32) -> u32 {
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        if host.status().state == STATE_READY && host.status().quantum_frames != actual_frames {
            return host.reject_output_quantum(actual_frames);
        }
        host.render_next()
    })
}

/// Return the stable prepared resource-report address or zero for an invalid handle.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_resource_ptr(handle: u32) -> u32 {
    with_host(handle, 0, |host| {
        pointer_u32(ptr::from_ref(host.resources()))
    })
}

/// Admit one staged live-console command submission (issue #137 D1).
///
/// `count` records were written into [`BUFFER_COMMAND`]. The submission is one transaction: the
/// return value is the frozen result code, and
/// [`miso_engine_web_v1_command_report_ptr`] names the first refused record and why.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_command_submit(handle: u32, count: u32) -> u32 {
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        host.submit_commands(count)
    })
}

/// Admit one semantic command batch with its opaque prepared-target companion.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_prepared_command_submit(
    handle: u32,
    count: u32,
    companion_bytes: u32,
) -> u32 {
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        host.submit_prepared_commands(count, companion_bytes)
    })
}

/// Return the fixed prepared companion staging address.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_prepared_companion_ptr(handle: u32) -> u32 {
    with_host_mut(handle, 0, |host| {
        host.prepared_companion_mut()
            .map_or(0, |bytes| pointer_u32(bytes.as_mut_ptr()))
    })
}

/// Return the fixed prepared companion staging capacity.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_prepared_companion_capacity(handle: u32) -> u32 {
    with_host(
        handle,
        0,
        AudioWorkletEngineHost::prepared_companion_capacity,
    )
}

/// Copy one accepted EQ owner's canonical configuration into the fixed config workspace.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_eq_target_config_copy(
    handle: u32,
    track_index: u32,
    rack: u32,
    effect_index: u32,
) -> u32 {
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        host.copy_eq_target_config(track_index, rack, effect_index)
    })
}

/// Return the fixed addressed EQ configuration workspace.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_eq_target_config_ptr(handle: u32) -> u32 {
    with_host(handle, 0, |host| {
        host.eq_target_config()
            .map_or(0, |bytes| pointer_u32(bytes.as_ptr()))
    })
}

/// Copy one accepted builtin input-filter configuration into the shared config workspace.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_input_filters_config_copy(
    handle: u32,
    track_index: u32,
) -> u32 {
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        host.copy_input_filter_config(track_index)
    })
}

/// Return the stable live-console command-report address or zero for an invalid handle.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_command_report_ptr(handle: u32) -> u32 {
    with_host(handle, 0, |host| {
        pointer_u32(ptr::from_ref(host.command_report()))
    })
}

/// Take (`1`) or release (`0`) the decimated meter lease (issue #137 D2).
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_meter_lease(handle: u32, enabled: u32) -> u32 {
    match refuse_protected_observation_alias(handle, OBSERVATION_OPERATION_METER_LEASE) {
        Err(result) | Ok(Some(result)) => return result,
        Ok(None) => {}
    }
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        if enabled > 1 {
            return host.record_boundary_result(RESULT_INVALID_ARGUMENT);
        }
        host.set_meter_lease(enabled == 1)
    })
}

/// Drain finished meter windows into the frame buffer; returns the number of complete windows.
///
/// Called from `process()` after the render export, so it is allocation-free and bounded: it moves
/// `Copy` snapshots out of queues sized at compilation into a buffer sized at compilation.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_meter_poll(handle: u32) -> u32 {
    with_host_mut(handle, 0, AudioWorkletEngineHost::poll_meters)
}

/// Return the stable meter-header address, or zero for an invalid handle (issue #143 D5).
///
/// The `f32` meter frame carries numbers a meter draws; the window those numbers describe is a
/// pair of absolute sample counts, which an `f32` cannot hold. They ride this fixed structure, read
/// exactly as the status and the resource report are.
/// `reserved[0]` is the publication generation, advanced by lease transitions and detected meter
/// producer resets. In `reserved[1]`, bits 0..3 are validity
/// (`complete`, `master aligned`, `loss observed`, `gain reduction present`) and bits 32..63 carry
/// the saturating loss count. A successful poll publishes one exact track/master window; an empty
/// or invalid poll leaves every byte of the prior header and frame untouched. Source seeks keep
/// absolute render time continuous and therefore do not advance this meter generation.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_meter_header_ptr(handle: u32) -> u32 {
    with_host(handle, 0, |host| {
        pointer_u32(ptr::from_ref(host.meter_header()))
    })
}

/// Return the number of tracks the live console addresses, or zero before compilation.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_console_track_count(handle: u32) -> u32 {
    with_host(handle, 0, |host| {
        u32::try_from(host.console_tracks().len()).unwrap_or(0)
    })
}

/// Copy one canonical track ID into the ID staging buffer; returns its byte length.
///
/// Zero means "no such track". The caller reads the bytes out of [`BUFFER_SOURCE_ID`], which
/// preparation already sized for the longest source or track ID in the session.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_console_track_id(handle: u32, index: u32) -> u32 {
    with_host_mut(handle, 0, |host| host.copy_console_track_id(index))
}

/// Return the number of prepared resident observation effects in the current owner map.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_count(handle: u32) -> u32 {
    with_host(handle, 0, |host| {
        u32::try_from(host.observation_binding_count()).unwrap_or(0)
    })
}

/// Return the stable observation-ID staging address used by binding introspection.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_id_ptr() -> u32 {
    OBSERVATION_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return 0;
        };
        pointer_u32(staging.id.as_mut_ptr())
    })
}

/// Return the maximum stable observation-ID staging length.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_id_capacity() -> u32 {
    RESPONSE_MAXIMUM_EFFECT_ID_BYTES
}

fn copy_observation_id(handle: u32, index: u32, native: bool) -> u32 {
    OBSERVATION_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return 0;
        };
        with_host(handle, 0, |host| {
            let Some(binding) = host.observation_binding(index) else {
                return 0;
            };
            let bytes = if native {
                binding.native_effect_id.as_bytes()
            } else {
                binding.effect_slot_id.as_bytes()
            };
            if bytes.len() > staging.id.len() {
                return 0;
            }
            staging.id[..bytes.len()].copy_from_slice(bytes);
            u32::try_from(bytes.len()).unwrap_or(0)
        })
    })
}

/// Copy one bound observation effect's stable local slot ID into the observation-ID staging area.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_effect_slot_id(handle: u32, index: u32) -> u32 {
    copy_observation_id(handle, index, false)
}

/// Copy one bound observation effect's native contract ID into the observation-ID staging area.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_native_effect_id(handle: u32, index: u32) -> u32 {
    copy_observation_id(handle, index, true)
}

/// Return one bound observation effect's track index, or zero when out of range.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_track_index(handle: u32, index: u32) -> u32 {
    with_host(handle, 0, |host| {
        host.observation_binding(index)
            .map_or(0, |binding| binding.address.track_index)
    })
}

/// Return one bound observation effect's rack (`0`, `1` or `2`), or zero when out of range.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_rack(handle: u32, index: u32) -> u32 {
    with_host(handle, 0, |host| {
        host.observation_binding(index)
            .map_or(0, |binding| observation_rack_raw(binding.address.rack))
    })
}

/// Return one bound observation effect's position within its rack, or zero when out of range.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_effect_index(handle: u32, index: u32) -> u32 {
    with_host(handle, 0, |host| {
        host.observation_binding(index)
            .map_or(0, |binding| binding.address.effect_index)
    })
}

/// Return one bound observation effect's declared tap count, or zero when out of range.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_tap_count(handle: u32, index: u32) -> u32 {
    with_host(handle, 0, |host| {
        host.observation_binding(index)
            .and_then(|binding| u32::try_from(binding.descriptors.len()).ok())
            .unwrap_or(0)
    })
}

/// Return one bound observation effect's declared tap ID, or zero when out of range.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_tap_id(
    handle: u32,
    index: u32,
    tap_index: u32,
) -> u32 {
    with_host(handle, 0, |host| {
        host.observation_binding(index)
            .and_then(|binding| binding.descriptors.get(tap_index as usize))
            .map_or(0, |tap| tap.id.0)
    })
}

/// Return a writable bounded selected-observation input staging address.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_selection_ptr() -> u32 {
    OBSERVATION_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return 0;
        };
        pointer_u32(staging.selections.as_mut_ptr())
    })
}

/// Return the fixed selected-observation input record size.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_selection_bytes() -> u32 {
    OBSERVATION_SELECTION_BYTES
}

/// Return the maximum selected-observation input record count.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_selection_capacity() -> u32 {
    MAXIMUM_OBSERVATION_READS as u32
}

/// Read one complete bounded batch of selected resident observations.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_read(handle: u32, count: u32) -> u32 {
    match refuse_protected_observation_alias(handle, OBSERVATION_OPERATION_RESIDENT_READ) {
        Err(result) | Ok(Some(result)) => return result,
        Ok(None) => {}
    }
    OBSERVATION_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        staging.reset_results();
        let count = match usize::try_from(count) {
            Ok(count) if count <= MAXIMUM_OBSERVATION_READS => count,
            _ => return RESULT_BUFFER_TOO_SMALL,
        };
        for index in 0..count {
            let selection = staging.selections[index];
            if selection.struct_size != OBSERVATION_SELECTION_BYTES
                || selection.abi_version != ABI_VERSION
                || selection.reserved != 0
            {
                return RESULT_INVALID_ARGUMENT;
            }
            let rack = match observation_rack(selection.rack) {
                Ok(value) => value,
                Err(result) => return result,
            };
            let channels = match observation_channels(selection.channels) {
                Ok(value) => value,
                Err(result) => return result,
            };
            staging.addresses.push(ObservationAddress {
                track_index: selection.track_index,
                rack,
                effect_index: selection.effect_index,
                tap_id: selection.tap_id,
                channels,
            });
        }
        let read = {
            let ObservationStaging {
                addresses, rows, ..
            } = &mut *staging;
            with_host(
                handle,
                Err(ObservationReadError::InvalidSelection),
                |host| host.read_observation_addresses_into(addresses, &mut rows[..count]),
            )
        };
        if let Err(error) = read {
            return observation_error_code(error);
        }
        let ObservationStaging {
            addresses,
            rows,
            results,
            ..
        } = &mut *staging;
        for (address, row) in addresses.iter().zip(rows.iter()).take(count) {
            let Some(window) = row.window else {
                results.push(WebObservationResult {
                    struct_size: OBSERVATION_RESULT_BYTES,
                    abi_version: ABI_VERSION,
                    status: observation_status_raw(row.status),
                    track_index: address.track_index,
                    rack: observation_rack_raw(address.rack),
                    effect_index: address.effect_index,
                    tap_id: address.tap_id,
                    channels: selection_channels_raw(address.channels),
                    sample_rate_hz: row.sample_rate_hz,
                    ..WebObservationResult::default()
                });
                continue;
            };
            results.push(WebObservationResult {
                struct_size: OBSERVATION_RESULT_BYTES,
                abi_version: ABI_VERSION,
                status: observation_status_raw(row.status),
                track_index: address.track_index,
                rack: observation_rack_raw(address.rack),
                effect_index: address.effect_index,
                tap_id: address.tap_id,
                channels: selection_channels_raw(address.channels),
                sample_rate_hz: row.sample_rate_hz,
                first_sample: window.first_sample,
                end_sample: window.end_sample,
                sequence: window.sequence,
                blocks: window.blocks,
                left_present: u32::from(row.left.is_some()),
                right_present: u32::from(row.right.is_some()),
                left: row.left.unwrap_or(0.0),
                right: row.right.unwrap_or(0.0),
                ..WebObservationResult::default()
            });
        }
        RESULT_OK
    })
}

fn selection_channels_raw(channels: ObservationReadChannels) -> u32 {
    match channels {
        ObservationReadChannels::Left => OBSERVATION_CHANNEL_LEFT,
        ObservationReadChannels::Right => OBSERVATION_CHANNEL_RIGHT,
        ObservationReadChannels::Both => OBSERVATION_CHANNEL_BOTH,
    }
}

/// Return the selected-observation result-record address, or zero before a successful read.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_result_ptr() -> u32 {
    OBSERVATION_STAGING.with(|slot| {
        let Ok(staging) = slot.try_borrow() else {
            return 0;
        };
        pointer_u32(staging.results.as_ptr())
    })
}

/// Return the selected-observation result byte length.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_result_bytes() -> u32 {
    OBSERVATION_STAGING.with(|slot| {
        slot.try_borrow()
            .ok()
            .and_then(|staging| {
                staging
                    .results
                    .len()
                    .checked_mul(OBSERVATION_RESULT_BYTES as usize)
                    .and_then(|bytes| u32::try_from(bytes).ok())
            })
            .unwrap_or(0)
    })
}

/// Return the number of sources the compiled session declares, or zero before compilation.
///
/// # Issue #207: source introspection
///
/// The browser ABI has exposed track discovery since #137 and nothing at all about sources, so a
/// headless driver compiling raw session JSON could not learn which sources exist, how many
/// channels they carry, or how many frames to feed them -- it could not drive the render loop it
/// had just compiled. These four queries close that, additively, in the shape the track queries
/// already established: a count, an ID copied through the staging buffer, and scalar shape reads.
///
/// **Canonical source order** is the normalized model's `sources` order -- `compile_session` sorts
/// by stable ID -- and the queries read that list itself, so no second table exists to drift from
/// it. **State gating** is the track queries' gating exactly: the answers come from the compiled
/// session, so every query reports zero/absent until `boot` succeeds, and keeps answering
/// afterwards for as long as the handle holds a compiled session, sticky failure included.
///
/// **This export is the bounds authority.** `source_channels` and `source_frames` return zero for
/// an out-of-range index because zero is impossible for a compiled source.
///
/// These queries survived issue #240's boot ABI recut unchanged. What pins the complete surface
/// is the frozen export set in `scripts/check-web-audioworklet.sh`, which is exact rather than a
/// lower bound: an export that appears or disappears fails that gate.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_source_count(handle: u32) -> u32 {
    with_host(handle, 0, |host| {
        u32::try_from(host.session_source_count()).unwrap_or(0)
    })
}

/// Copy one canonical source ID into the ID staging buffer; returns its byte length.
///
/// Zero means "no such source". [`BUFFER_SOURCE_ID`] is sized for the longest source or track ID
/// in the compiled session.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_source_id(handle: u32, index: u32) -> u32 {
    with_host_mut(handle, 0, |host| host.copy_session_source_id(index))
}

/// Return one source's declared channel count, or zero for an out-of-range index.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_source_channels(handle: u32, index: u32) -> u32 {
    with_host(handle, 0, |host| {
        host.session_source_shape(index)
            .map_or(0, |shape| shape.channel_count)
    })
}

/// Return one source's exact length in source sample frames, or zero out of range.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_source_frames(handle: u32, index: u32) -> u64 {
    with_host(handle, 0, |host| {
        host.session_source_shape(index)
            .map_or(0, |shape| shape.frames)
    })
}

/// Return the stable status address or zero for an invalid handle.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_status_ptr(handle: u32) -> u32 {
    with_host(handle, 0, |host| pointer_u32(ptr::from_ref(host.status())))
}

/// Quiescently dispose the live handle; zero is an explicit no-op.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_dispose(handle: u32) -> u32 {
    if handle == 0 {
        return RESULT_OK;
    }

    // Acquire every staging owner first. A borrow refusal therefore leaves LIVE_HOST untouched,
    // including its native ownership and pending receipt rows.
    OBSERVATION_STAGING.with(|observation_slot| {
        let Ok(mut observation) = observation_slot.try_borrow_mut() else {
            return RESULT_INVALID_ARGUMENT;
        };
        SPECTRUM_STAGING.with(|spectrum_slot| {
            let Ok(mut spectrum) = spectrum_slot.try_borrow_mut() else {
                return RESULT_INVALID_ARGUMENT;
            };
            BOOT_STAGING.with(|boot_slot| {
                let Ok(mut boot) = boot_slot.try_borrow_mut() else {
                    return RESULT_INVALID_ARGUMENT;
                };
                LIVE_HOST.with(|live_slot| {
                    let Ok(mut live_slot) = live_slot.try_borrow_mut() else {
                        return RESULT_INVALID_ARGUMENT;
                    };
                    let Some(live) = live_slot.as_ref().filter(|live| live.handle == handle) else {
                        return RESULT_INVALID_ARGUMENT;
                    };
                    let _ = live;
                    let Some(mut live) = live_slot.take() else {
                        return RESULT_INTERNAL;
                    };

                    // Native disposal performs its one reconciliation and terminalizes remaining
                    // Pending rows. Snapshot all scalar mirrors only after that transition, then
                    // take the resulting rows once from the now-terminal native owner.
                    let result = live.host.dispose();
                    observation.endpoint.status = live.host.observation_status();
                    observation.endpoint.admission = *live.host.observation_admission();
                    observation.endpoint.capture_identity =
                        *live.host.observation_capture_identity();
                    let applications = live.host.take_observation_applications();
                    let count = applications.len();
                    debug_assert!(count <= observation.endpoint.applications.len());
                    observation.endpoint.applications[..count].copy_from_slice(applications);
                    observation.endpoint.application_count = u32::try_from(count).unwrap_or(0);
                    observation.endpoint.handle = handle;
                    observation.endpoint.terminal_application_pending = count != 0;

                    drop(live);
                    spectrum.release_capture();
                    boot.reset_after_dispose();
                    result
                })
            })
        })
    })
}

#[cfg(test)]
pub(crate) fn test_stage_document(bytes: &[u8]) {
    BOOT_STAGING.with(|staging| {
        let mut staging = staging.borrow_mut();
        staging.document.clear();
        staging.document.extend_from_slice(bytes);
        staging.document_valid = true;
        staging.result = RESULT_OK;
        staging.diagnostic_bytes = 0;
    })
}

#[cfg(test)]
pub(crate) fn test_boot(bytes: &[u8], options: WebBootOptions) -> u32 {
    BOOT_STAGING.with(|staging| *staging.borrow_mut().options = options);
    test_stage_document(bytes);
    miso_engine_web_v1_boot(bytes.len() as u32)
}

#[cfg(test)]
pub(crate) fn test_staged_document() -> Vec<u8> {
    BOOT_STAGING.with(|staging| staging.borrow().document.clone())
}

#[cfg(test)]
pub(crate) fn test_copy_staging(handle: u32, kind: u32, bytes: &[u8]) -> u32 {
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        let target = match kind {
            BUFFER_SOURCE_ID => host.source_id_mut(),
            _ => None,
        };
        let Some(target) = target else {
            return RESULT_INVALID_ARGUMENT;
        };
        let Some(target) = target.get_mut(..bytes.len()) else {
            return RESULT_INVALID_ARGUMENT;
        };
        target.copy_from_slice(bytes);
        RESULT_OK
    })
}

#[cfg(test)]
pub(crate) fn test_read_source_id(handle: u32, length: u32) -> Option<Vec<u8>> {
    with_host_mut(handle, None, |host| {
        host.source_id_mut()
            .and_then(|bytes| bytes.get(..length as usize))
            .map(<[u8]>::to_vec)
    })
}

#[cfg(test)]
pub(crate) fn test_fill_source_pcm(handle: u32, value: f32) -> u32 {
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        let Some(target) = host.source_pcm_mut() else {
            return RESULT_INVALID_ARGUMENT;
        };
        target.fill(value);
        RESULT_OK
    })
}

#[cfg(test)]
pub(crate) fn test_buffer_address(handle: u32, kind: u32) -> usize {
    with_host_mut(handle, 0, |host| buffer_pointer(host, kind).addr())
}

#[cfg(test)]
pub(crate) fn test_status(handle: u32) -> Option<crate::WebStatus> {
    with_host(handle, None, |host| Some(*host.status()))
}

#[cfg(test)]
pub(crate) fn test_resources(handle: u32) -> Option<crate::WebResourceReport> {
    with_host(handle, None, |host| Some(*host.resources()))
}

#[cfg(test)]
pub(crate) fn test_status_address(handle: u32) -> usize {
    with_host(handle, 0, |host| ptr::from_ref(host.status()).addr())
}

#[cfg(test)]
pub(crate) fn test_resource_address(handle: u32) -> usize {
    with_host(handle, 0, |host| ptr::from_ref(host.resources()).addr())
}

#[cfg(test)]
mod response_budget_tests {
    use super::*;

    #[test]
    fn response_budget_rejects_point_storage_before_allocation() {
        assert_eq!(
            response_buffer_budget(
                250_000,
                2,
                crate::RESPONSE_CHANNEL_BOTH,
                crate::RESPONSE_FIELD_TOTAL,
                16,
            ),
            Err(RESULT_REFUSED_BUDGET),
        );
        let result_bytes = size_of::<WebResponseResult>() + 8 + 16 + 32;
        assert_eq!(
            response_buffer_budget(
                2,
                2,
                crate::RESPONSE_CHANNEL_BOTH,
                crate::RESPONSE_FIELD_SECTIONS,
                result_bytes,
            ),
            Ok(()),
        );
    }
}

#[cfg(test)]
pub(crate) mod live_response_ffi_tests {
    use super::*;
    use crate::ffi::observation_checkpoint_a_tests::{
        no_live_host, protected_document, protected_preparation_record, stage_protected_boot,
    };
    use core::alloc::Layout;
    use core::cell::Cell;
    use std::alloc::{GlobalAlloc, System};

    thread_local! {
        static ARMED: Cell<bool> = const { Cell::new(false) };
        static ALLOCATIONS: Cell<u64> = const { Cell::new(0) };
        static DEALLOCATIONS: Cell<u64> = const { Cell::new(0) };
    }

    struct CountingAllocator;

    #[global_allocator]
    static ALLOCATOR: CountingAllocator = CountingAllocator;

    fn count_allocation() {
        if ARMED.try_with(Cell::get).unwrap_or(false) {
            ALLOCATIONS.with(|count| count.set(count.get().saturating_add(1)));
        }
    }

    fn count_deallocation() {
        if ARMED.try_with(Cell::get).unwrap_or(false) {
            DEALLOCATIONS.with(|count| count.set(count.get().saturating_add(1)));
        }
    }

    // SAFETY: every allocator operation is forwarded unchanged to the system allocator; the
    // thread-local counters observe successful operations but never alter ownership or layout.
    unsafe impl GlobalAlloc for CountingAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            // SAFETY: the caller supplied this layout for one system allocation.
            let pointer = unsafe { System.alloc(layout) };
            if !pointer.is_null() {
                count_allocation();
            }
            pointer
        }

        unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
            // SAFETY: the caller supplied this layout for one zeroed system allocation.
            let pointer = unsafe { System.alloc_zeroed(layout) };
            if !pointer.is_null() {
                count_allocation();
            }
            pointer
        }

        unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
            count_deallocation();
            // SAFETY: the pointer and layout are the matching allocation supplied by the caller.
            unsafe { System.dealloc(pointer, layout) };
        }

        unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
            // SAFETY: the caller supplied the matching allocation and replacement size.
            let replacement = unsafe { System.realloc(pointer, layout, new_size) };
            if !replacement.is_null() {
                count_allocation();
                count_deallocation();
            }
            replacement
        }
    }

    pub(crate) fn measured<T>(operation: impl FnOnce() -> T) -> (T, u64, u64) {
        ARMED.with(|armed| armed.set(false));
        ALLOCATIONS.with(|count| count.set(0));
        DEALLOCATIONS.with(|count| count.set(0));
        ARMED.with(|armed| armed.set(true));
        let result = operation();
        ARMED.with(|armed| armed.set(false));
        (
            result,
            ALLOCATIONS.with(Cell::get),
            DEALLOCATIONS.with(Cell::get),
        )
    }

    fn stage_request(track_id: &[u8]) {
        stage_request_with_limit(track_id, LIVE_RESPONSE_CAPTURE_BYTES as u32);
    }

    fn stage_request_with_limit(track_id: &[u8], maximum_result_bytes: u32) {
        RESPONSE_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            assert!(track_id.len() <= staging.live_track_id.len());
            staging.live_track_id.fill(0);
            staging.live_track_id[..track_id.len()].copy_from_slice(track_id);
            *staging.live_request = WebLiveResponseRequest {
                struct_size: LIVE_RESPONSE_REQUEST_BYTES,
                abi_version: ABI_VERSION,
                track_id_bytes: u32::try_from(track_id.len()).expect("short test track ID"),
                grid: crate::RESPONSE_GRID_LINEAR,
                channels: crate::RESPONSE_CHANNEL_BOTH,
                points: 5,
                minimum_hz: 20.0,
                maximum_hz: 20_000.0,
                maximum_result_bytes,
                reserved: [0; 3],
            };
            staging.live_result_len = 0;
        });
    }

    fn captured_header() -> WebLiveResponseResult {
        RESPONSE_STAGING.with(|slot| slot.borrow().live_result_header)
    }

    #[derive(Clone, Debug, PartialEq)]
    struct ResponseMarkers {
        raw_result: Vec<u8>,
        live_result_len: usize,
        live_token: u64,
        live_result_header: WebLiveResponseResult,
        capture_identity: WebObservationCaptureIdentity,
    }

    fn response_markers() -> ResponseMarkers {
        let (raw_result, live_result_len, live_token, live_result_header) =
            RESPONSE_STAGING.with(|slot| {
                let staging = slot.borrow();
                (
                    staging.live_result.clone(),
                    staging.live_result_len,
                    staging.live_token,
                    staging.live_result_header,
                )
            });
        let capture_identity = LIVE_HOST.with(|slot| {
            *slot
                .borrow()
                .as_ref()
                .expect("protected live host")
                .host
                .observation_capture_identity()
        });
        ResponseMarkers {
            raw_result,
            live_result_len,
            live_token,
            live_result_header,
            capture_identity,
        }
    }

    fn seed_response_markers() -> ResponseMarkers {
        let snapshot_token = 41;
        let result_bytes = LIVE_RESPONSE_CAPTURE_BYTES;
        let seeded_header = WebLiveResponseResult {
            struct_size: LIVE_RESPONSE_RESULT_BYTES,
            abi_version: ABI_VERSION,
            result: RESULT_OK,
            mode: LIVE_RESPONSE_MODE_TARGET,
            meaning: LIVE_RESPONSE_MEANING_EQ_FILTER_SUBTOTAL,
            channels: crate::RESPONSE_CHANNEL_BOTH,
            points: 5,
            owner_count: 1,
            excluded_count: 1,
            sample_rate_hz: 48_000,
            captured_sample: 12_288,
            snapshot_token,
            result_bytes: result_bytes as u64,
            owners_offset: LIVE_RESPONSE_RESULT_BYTES,
            owner_record_bytes: LIVE_RESPONSE_OWNER_BYTES,
            section_record_bytes: LIVE_RESPONSE_SECTION_BYTES,
            ..WebLiveResponseResult::default()
        };
        RESPONSE_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            staging.live_result.fill(0xa5);
            assert!(copy_live_record(
                &mut staging.live_result,
                0,
                &seeded_header
            ));
            staging.live_result_header = seeded_header;
            staging.live_result_len = result_bytes;
            staging.live_token = snapshot_token;
        });
        let owner = LIVE_HOST.with(|slot| {
            slot.borrow()
                .as_ref()
                .expect("protected live host")
                .host
                .observation_status()
                .owner
        });
        LIVE_HOST.with(|slot| {
            slot.borrow_mut()
                .as_mut()
                .expect("protected live host")
                .host
                .commit_observation_capture_identity(WebObservationCaptureIdentity {
                    struct_size: size_of::<WebObservationCaptureIdentity>() as u32,
                    abi_version: ABI_VERSION,
                    kind: 1,
                    flags: 1,
                    owner,
                    observation_generation: 42,
                    selection_epoch: 43,
                    snapshot_token,
                });
        });
        response_markers()
    }

    fn assert_response_markers(markers: &ResponseMarkers) {
        RESPONSE_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert_eq!(staging.live_result, markers.raw_result);
            assert_eq!(staging.live_result_len, markers.live_result_len);
            assert_eq!(staging.live_token, markers.live_token);
            assert_eq!(staging.live_result_header, markers.live_result_header);
        });
        LIVE_HOST.with(|slot| {
            assert_eq!(
                *slot
                    .borrow()
                    .as_ref()
                    .expect("protected live host")
                    .host
                    .observation_capture_identity(),
                markers.capture_identity
            );
        });
    }

    fn assert_response_staging_markers(markers: &ResponseMarkers) {
        RESPONSE_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert_eq!(staging.live_result, markers.raw_result);
            assert_eq!(staging.live_result_len, markers.live_result_len);
            assert_eq!(staging.live_token, markers.live_token);
            assert_eq!(staging.live_result_header, markers.live_result_header);
        });
    }

    fn boot_private_protected() -> u32 {
        no_live_host();
        RESPONSE_STAGING.with(|slot| slot.borrow_mut().reset());
        let document = protected_document();
        stage_protected_boot(document, protected_preparation_record());
        let handle = boot_staged_observation_demand(document.len() as u32);
        assert_ne!(handle, 0, "private protected fixture must boot");
        handle
    }

    #[test]
    fn ffi_protected_response_capture_publishes_raw_and_identity() {
        let handle = boot_private_protected();
        stage_request_with_limit(b"eq0", 65_536);

        assert_eq!(miso_engine_web_v1_track_response_capture(handle), RESULT_OK);
        let header = captured_header();
        assert_eq!(header.result, RESULT_OK);
        assert_eq!(header.snapshot_token, 1);
        assert!(header.result_bytes > u64::from(LIVE_RESPONSE_RESULT_BYTES));
        assert_eq!(
            RESPONSE_STAGING.with(|slot| slot.borrow().live_result_len),
            header.result_bytes as usize
        );
        let identity = LIVE_HOST.with(|slot| {
            *slot
                .borrow()
                .as_ref()
                .expect("protected live host")
                .host
                .observation_capture_identity()
        });
        let owner = LIVE_HOST.with(|slot| {
            slot.borrow()
                .as_ref()
                .expect("protected live host")
                .host
                .observation_status()
                .owner
        });
        assert_eq!(identity.kind, 1);
        assert_eq!(identity.flags, 0);
        assert_eq!(identity.owner, owner);
        assert_eq!(identity.observation_generation, 0);
        assert_eq!(identity.selection_epoch, 0);
        assert_eq!(identity.snapshot_token, 1);
        assert_eq!(
            miso_engine_web_v1_dispose(handle),
            RESULT_OK,
            "protected response fixture dispose"
        );
    }

    #[test]
    fn ffi_protected_response_preflight_refusal_preserves_committed_markers() {
        let handle = boot_private_protected();
        stage_request_with_limit(b"missing", 65_536);
        let markers = seed_response_markers();

        assert_eq!(
            miso_engine_web_v1_track_response_capture(handle),
            RESULT_INVALID_ARGUMENT
        );
        assert_response_markers(&markers);
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    }

    #[test]
    fn ffi_protected_response_dispatch_borrow_conflicts_preserve_committed_markers() {
        let handle = boot_private_protected();
        stage_request(b"eq0");
        let markers = seed_response_markers();

        LIVE_HOST.with(|slot| {
            let _borrow = slot.borrow();
            assert_eq!(
                miso_engine_web_v1_track_response_capture(handle),
                RESULT_INTERNAL
            );
        });
        assert_response_markers(&markers);

        LIVE_HOST.with(|slot| {
            let _borrow = slot.borrow_mut();
            assert_eq!(
                miso_engine_web_v1_track_response_capture(handle),
                RESULT_INTERNAL
            );
        });
        assert_response_markers(&markers);

        RESPONSE_STAGING.with(|slot| {
            let _borrow = slot.borrow_mut();
            assert_eq!(
                miso_engine_web_v1_track_response_capture(handle),
                RESULT_INTERNAL
            );
        });
        assert_response_markers(&markers);
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    }

    #[test]
    fn ffi_protected_response_invalid_handles_preserve_committed_markers() {
        let handle = boot_private_protected();
        stage_request(b"eq0");
        let markers = seed_response_markers();

        assert_eq!(
            miso_engine_web_v1_track_response_capture(0),
            RESULT_WRONG_STATE
        );
        assert_response_markers(&markers);

        let unrelated = handle.wrapping_add(1).max(1);
        assert_eq!(
            miso_engine_web_v1_track_response_capture(unrelated),
            RESULT_WRONG_STATE
        );
        assert_response_markers(&markers);

        let committed_identity = markers.capture_identity;
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
        assert_eq!(
            miso_engine_web_v1_track_response_capture(handle),
            RESULT_WRONG_STATE
        );
        assert_response_staging_markers(&markers);
        let terminal_identity =
            OBSERVATION_STAGING.with(|slot| slot.borrow().endpoint.capture_identity);
        assert_eq!(terminal_identity, committed_identity);
    }

    #[test]
    fn ffi_protected_response_token_exhaustion_preserves_markers_and_admission() {
        let handle = boot_private_protected();
        stage_request_with_limit(b"eq0", 65_536);
        let _ = seed_response_markers();
        RESPONSE_STAGING.with(|slot| slot.borrow_mut().live_token = u64::MAX);
        let markers = response_markers();
        let callback_before = live_response_owner_callback_calls();

        assert_eq!(
            miso_engine_web_v1_track_response_capture(handle),
            RESULT_REFUSED_BUDGET
        );
        assert_response_markers(&markers);
        assert_eq!(
            live_response_owner_callback_calls(),
            callback_before,
            "token exhaustion must not invoke the response sink owner callback"
        );
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            assert_eq!(
                host.observation_status().flags & crate::OBSERVATION_STATUS_FLAG_ORDINARY_AVAILABLE,
                0,
                "token exhaustion spends the Ordinary attempt"
            );
            assert_eq!(
                *host.observation_admission(),
                WebObservationAdmission {
                    result: RESULT_REFUSED_BUDGET,
                    operation: 8,
                    ingress_epoch: 1,
                    ..WebObservationAdmission::default()
                }
            );
        });
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    }

    fn render_one_protected_boundary(handle: u32) {
        assert_eq!(
            test_copy_staging(handle, BUFFER_SOURCE_ID, b"fixture-source"),
            RESULT_OK
        );
        assert_eq!(test_fill_source_pcm(handle, 0.25), RESULT_OK);
        assert_eq!(
            miso_engine_web_v1_source_submit(handle, 14, 1, 0, 2, 128, 0),
            RESULT_OK
        );
        assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
    }

    fn real_response_markers_after_boundary(handle: u32) -> ResponseMarkers {
        stage_request_with_limit(b"eq0", 65_536);
        assert_eq!(
            miso_engine_web_v1_track_response_capture(handle),
            RESULT_OK,
            "private protected response fixture capture"
        );
        render_one_protected_boundary(handle);
        let markers = response_markers();
        assert!(
            markers.raw_result.iter().any(|byte| *byte != 0),
            "real response payload must be nonzero"
        );
        assert_eq!(markers.live_result_header.result, RESULT_OK);
        assert!(markers.live_result_len > LIVE_RESPONSE_RESULT_BYTES as usize);
        assert!(markers.live_token > 0);
        assert_eq!(markers.capture_identity.kind, 1);
        assert_ne!(markers.capture_identity.owner, 0);
        assert_eq!(markers.capture_identity.snapshot_token, markers.live_token);
        markers
    }

    fn assert_real_response_refusal(
        case: &str,
        handle: u32,
        markers: &ResponseMarkers,
        expected: u32,
    ) {
        let callback_before = live_response_owner_callback_calls();
        let result = miso_engine_web_v1_track_response_capture(handle);
        assert_eq!(result, expected, "{case} result");
        assert_response_markers(markers);
        assert_eq!(
            live_response_owner_callback_calls(),
            callback_before,
            "{case} must not invoke the response sink owner callback"
        );
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            let status = host.observation_status();
            assert_eq!(
                status.flags & crate::OBSERVATION_STATUS_FLAG_ORDINARY_AVAILABLE,
                0,
                "{case} spends the Ordinary attempt"
            );
            assert_eq!(
                *host.observation_admission(),
                WebObservationAdmission {
                    result: expected,
                    operation: 8,
                    ingress_epoch: status.ingress_epoch,
                    ..WebObservationAdmission::default()
                },
                "{case} admission"
            );
        });
    }

    fn configure_live_request(request: impl FnOnce(&mut WebLiveResponseRequest)) {
        RESPONSE_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            request(&mut staging.live_request);
        });
    }

    fn configure_live_track_id(bytes: &[u8], declared_bytes: u32) {
        RESPONSE_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            assert!(bytes.len() <= staging.live_track_id.len());
            staging.live_track_id.fill(0);
            staging.live_track_id[..bytes.len()].copy_from_slice(bytes);
            staging.live_request.track_id_bytes = declared_bytes;
        });
    }

    fn protected_response_packed_bound() -> usize {
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            usize::try_from(
                host.response_capture_preflight("eq0")
                    .expect("prepared response target bound"),
            )
            .expect("packed response bound fits usize")
        })
    }

    fn configure_live_result_capacity(capacity: usize) {
        configure_live_request(|request| {
            request.maximum_result_bytes =
                u32::try_from(capacity).expect("test response capacity fits u32");
        });
    }

    fn arm_live_response_owner_fault(error: ResponseSnapshotError) {
        LIVE_RESPONSE_CAPTURE_OWNER_CALLS.with(|calls| calls.set(0));
        LIVE_RESPONSE_CAPTURE_FAULT.with(|fault| fault.set(Some(error)));
    }

    fn live_response_owner_callback_calls() -> u32 {
        LIVE_RESPONSE_CAPTURE_OWNER_CALLS.with(Cell::get)
    }

    fn truncate_live_result_for_test(
        before: &ResponseMarkers,
        actual_capacity: usize,
    ) -> ResponseMarkers {
        assert!(actual_capacity <= before.raw_result.len());
        let surviving_prefix = before.raw_result[..actual_capacity].to_vec();
        // This deliberately truncates the control-side fixture Vec to model a short actual
        // staging region. Production response staging remains fixed; compare every surviving
        // byte before invoking the protected FFI refusal.
        RESPONSE_STAGING.with(|slot| {
            slot.borrow_mut().live_result.truncate(actual_capacity);
        });
        let truncated = response_markers();
        assert_eq!(truncated.raw_result, surviving_prefix);
        assert_eq!(truncated.live_result_len, before.live_result_len);
        assert_eq!(truncated.live_token, before.live_token);
        assert_eq!(truncated.live_result_header, before.live_result_header);
        assert_eq!(truncated.capture_identity, before.capture_identity);
        truncated
    }

    fn run_real_response_refusal(case: &str, configure: impl FnOnce(), expected: u32) {
        let handle = boot_private_protected();
        let markers = real_response_markers_after_boundary(handle);
        configure();
        assert_real_response_refusal(case, handle, &markers, expected);
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    }

    #[test]
    fn ffi_protected_response_malformed_fixed_request_preserves_real_markers() {
        run_real_response_refusal(
            "malformed struct size",
            || configure_live_request(|request| request.struct_size -= 1),
            RESULT_INVALID_ARGUMENT,
        );
        run_real_response_refusal(
            "malformed ABI version",
            || configure_live_request(|request| request.abi_version += 1),
            RESULT_INVALID_ARGUMENT,
        );
        run_real_response_refusal(
            "nonzero reserved request field",
            || configure_live_request(|request| request.reserved[0] = 1),
            RESULT_INVALID_ARGUMENT,
        );
    }

    #[test]
    fn ffi_protected_response_invalid_grid_preserves_real_markers() {
        run_real_response_refusal(
            "invalid response grid",
            || configure_live_request(|request| request.grid = u32::MAX),
            RESULT_INVALID_ARGUMENT,
        );
    }

    #[test]
    fn ffi_protected_response_semantic_grid_refusals_preserve_real_markers_before_sink() {
        let cases = [
            (
                "finite reversed endpoints",
                crate::RESPONSE_GRID_LINEAR,
                20_000.0,
                Some(20.0),
            ),
            (
                "equal endpoints",
                crate::RESPONSE_GRID_LINEAR,
                20.0,
                Some(20.0),
            ),
            (
                "negative minimum",
                crate::RESPONSE_GRID_LINEAR,
                -1.0,
                Some(20.0),
            ),
            (
                "logarithmic zero minimum",
                crate::RESPONSE_GRID_LOGARITHMIC,
                0.0,
                Some(20_000.0),
            ),
            (
                "maximum above Nyquist",
                crate::RESPONSE_GRID_LINEAR,
                20.0,
                None,
            ),
        ];
        for (case, grid, minimum_hz, maximum_hz) in cases {
            let handle = boot_private_protected();
            let markers = real_response_markers_after_boundary(handle);
            let callback_before = live_response_owner_callback_calls();
            let maximum_hz = maximum_hz.unwrap_or_else(|| {
                LIVE_HOST.with(|slot| {
                    let live = slot.borrow();
                    let sample_rate_hz = live
                        .as_ref()
                        .expect("protected live host")
                        .host
                        .status()
                        .sample_rate_hz;
                    sample_rate_hz as f32 * 0.5 + 1.0
                })
            });
            configure_live_request(|request| {
                request.grid = grid;
                request.minimum_hz = minimum_hz;
                request.maximum_hz = maximum_hz;
            });

            assert_real_response_refusal(case, handle, &markers, RESULT_INVALID_ARGUMENT);
            assert_eq!(
                live_response_owner_callback_calls(),
                callback_before,
                "{case} must not invoke the response sink owner callback"
            );
            assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
        }
    }

    #[test]
    fn ffi_protected_response_invalid_id_preserves_real_markers() {
        run_real_response_refusal(
            "invalid UTF-8 track ID",
            || configure_live_track_id(&[0xff], 1),
            RESULT_INVALID_ARGUMENT,
        );
        run_real_response_refusal(
            "overlong track ID",
            || {
                configure_live_track_id(
                    b"eq0",
                    u32::try_from(LIVE_RESPONSE_MAXIMUM_ID_BYTES + 1)
                        .expect("overlong ID declaration"),
                )
            },
            RESULT_INVALID_ARGUMENT,
        );
    }

    #[test]
    fn ffi_protected_response_wrong_exact_target_preserves_real_markers() {
        run_real_response_refusal(
            "wrong prepared response target",
            || configure_live_track_id(b"eq1", 3),
            RESULT_INVALID_ARGUMENT,
        );
    }

    #[test]
    fn ffi_protected_response_declared_capacity_one_byte_short_preserves_real_markers() {
        let handle = boot_private_protected();
        let markers = real_response_markers_after_boundary(handle);
        let packed_bound = protected_response_packed_bound();
        assert!(packed_bound > 0);
        configure_live_result_capacity(packed_bound - 1);

        assert_real_response_refusal(
            "declared response capacity one byte short",
            handle,
            &markers,
            RESULT_REFUSED_BUDGET,
        );
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    }

    #[test]
    fn ffi_protected_response_actual_capacity_one_byte_short_preserves_surviving_markers() {
        let handle = boot_private_protected();
        let complete = real_response_markers_after_boundary(handle);
        let packed_bound = protected_response_packed_bound();
        assert!(packed_bound > 0);
        let markers = truncate_live_result_for_test(&complete, packed_bound - 1);
        configure_live_result_capacity(packed_bound);

        assert_real_response_refusal(
            "actual response staging capacity one byte short",
            handle,
            &markers,
            RESULT_INVALID_ARGUMENT,
        );
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    }

    #[test]
    fn ffi_protected_response_declared_capacity_over_actual_staging_preserves_markers() {
        let handle = boot_private_protected();
        let complete = real_response_markers_after_boundary(handle);
        let packed_bound = protected_response_packed_bound();
        let actual_capacity = packed_bound;
        let declared_capacity = packed_bound + 1;
        assert!(declared_capacity <= complete.raw_result.len());
        let markers = truncate_live_result_for_test(&complete, actual_capacity);
        configure_live_result_capacity(declared_capacity);

        assert_real_response_refusal(
            "declared response capacity exceeds actual staging",
            handle,
            &markers,
            RESULT_INVALID_ARGUMENT,
        );
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    }

    #[test]
    fn ffi_protected_response_exact_packed_capacity_advances_and_packs_complete_result() {
        let handle = boot_private_protected();
        let previous = real_response_markers_after_boundary(handle);
        let packed_bound = protected_response_packed_bound();
        configure_live_result_capacity(packed_bound);

        assert_eq!(miso_engine_web_v1_track_response_capture(handle), RESULT_OK);
        let current = response_markers();
        assert_eq!(current.live_token, previous.live_token + 1);
        assert_eq!(current.live_result_header.result, RESULT_OK);
        assert_eq!(
            current.live_result_len,
            current.live_result_header.result_bytes as usize
        );
        assert!(current.live_result_len <= packed_bound);
        assert_eq!(current.capture_identity.kind, 1);
        assert_eq!(current.capture_identity.flags, 0);
        assert_eq!(
            current.capture_identity.owner,
            previous.capture_identity.owner
        );
        assert_eq!(current.capture_identity.observation_generation, 0);
        assert_eq!(current.capture_identity.selection_epoch, 0);
        assert_eq!(current.capture_identity.snapshot_token, current.live_token);
        RESPONSE_STAGING.with(|slot| {
            let staging = slot.borrow();
            let (snapshot, token) =
                parse_live_snapshot(&staging).expect("exact-fit response is complete");
            assert_eq!(token, current.live_token);
            assert!(!snapshot.owners.is_empty());
        });
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    }

    #[test]
    fn ffi_protected_response_admitted_owner_failure_preserves_success_identity() {
        let handle = boot_private_protected();
        let previous = real_response_markers_after_boundary(handle);
        let packed_bound = protected_response_packed_bound();
        configure_live_result_capacity(packed_bound);
        arm_live_response_owner_fault(ResponseSnapshotError::Owner);

        assert_eq!(
            miso_engine_web_v1_track_response_capture(handle),
            RESULT_INTERNAL
        );
        assert_eq!(live_response_owner_callback_calls(), 1);
        assert!(
            LIVE_RESPONSE_CAPTURE_FAULT.with(Cell::get).is_none(),
            "the one-shot fault must be consumed by the admitted owner callback"
        );

        let expected_header = WebLiveResponseResult {
            struct_size: LIVE_RESPONSE_RESULT_BYTES,
            abi_version: ABI_VERSION,
            result: RESULT_INTERNAL,
            mode: LIVE_RESPONSE_MODE_TARGET,
            meaning: LIVE_RESPONSE_MEANING_EQ_FILTER_SUBTOTAL,
            owner_record_bytes: LIVE_RESPONSE_OWNER_BYTES,
            section_record_bytes: LIVE_RESPONSE_SECTION_BYTES,
            result_bytes: u64::from(LIVE_RESPONSE_RESULT_BYTES),
            ..WebLiveResponseResult::default()
        };
        let current = response_markers();
        assert_eq!(current.live_result_header, expected_header);
        assert_eq!(current.live_result_len, LIVE_RESPONSE_RESULT_BYTES as usize);
        assert_eq!(current.live_token, previous.live_token);
        assert_eq!(current.capture_identity, previous.capture_identity);
        RESPONSE_STAGING.with(|slot| {
            let staging = slot.borrow();
            let published: WebLiveResponseResult =
                read_live_record(&staging.live_result, 0).expect("failure header");
            assert_eq!(published, expected_header);
        });

        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            let status = host.observation_status();
            let admission = host.observation_admission();
            assert_eq!(admission.result, RESULT_INTERNAL);
            assert_eq!(admission.operation, 8);
            assert_eq!(admission.flags, 0);
            assert_eq!(admission.receipt, WebObservationReceipt::default());
            assert_eq!(
                status.flags & crate::OBSERVATION_STATUS_FLAG_ORDINARY_AVAILABLE,
                0,
                "an admitted owner failure spends the Ordinary attempt"
            );
            assert_eq!(host.side_records.application_len, 0);
            assert_eq!(host.side_records.pending_count, 0);
            assert_eq!(host.side_records.completed_count, 0);
            assert_eq!(host.side_records.reserved_mask, 0);
        });
        assert_eq!(miso_engine_web_v1_observation_application_take(handle), 0);
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    }

    #[test]
    fn ffi_protected_response_shares_ordinary_credit_with_spectrum_and_replenishes_at_render() {
        let handle = boot_private_protected();
        stage_request_with_limit(b"eq0", 65_536);
        let markers = seed_response_markers();

        assert_eq!(
            miso_engine_web_v1_spectrum_select(handle, 0, 0, 0),
            RESULT_UNSUPPORTED
        );
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            assert_eq!(
                *host.observation_admission(),
                WebObservationAdmission {
                    result: RESULT_UNSUPPORTED,
                    operation: 11,
                    reason: 8,
                    ingress_epoch: 1,
                    ..WebObservationAdmission::default()
                }
            );
        });

        // Unsupported selection is not a protected admission. Spend the Ordinary credit through
        // a genuine malformed stream start before exercising response backpressure.
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, f64::NAN),
            RESULT_INVALID_ARGUMENT
        );
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            let status = host.observation_status();
            let admission = host.observation_admission();
            assert_eq!(admission.operation, OBSERVATION_OPERATION_START_SPECTRUM);
            assert_eq!(admission.result, RESULT_INVALID_ARGUMENT);
            assert_eq!(admission.reason, 8);
            assert_eq!(status.ingress_epoch, 1);
            assert_eq!(
                status.flags & crate::OBSERVATION_STATUS_FLAG_ORDINARY_AVAILABLE,
                0,
                "the genuine malformed stream start spends Ordinary credit"
            );
        });

        let callback_before = live_response_owner_callback_calls();
        assert_eq!(
            miso_engine_web_v1_track_response_capture(handle),
            RESULT_BACKPRESSURE
        );
        assert_response_markers(&markers);
        assert_eq!(
            live_response_owner_callback_calls(),
            callback_before,
            "ordinary backpressure must not invoke the response sink owner callback"
        );
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            let admission = host.observation_admission();
            assert_eq!(admission.result, RESULT_BACKPRESSURE);
            assert_eq!(admission.operation, 8);
            assert_eq!(admission.reason, 5);
            assert_eq!(admission.requested, 2);
            assert_eq!(admission.maximum, 1);
            assert_eq!(
                host.observation_status().flags & crate::OBSERVATION_STATUS_FLAG_ORDINARY_AVAILABLE,
                0
            );
        });

        render_one_protected_boundary(handle);
        assert_ne!(
            LIVE_HOST.with(|slot| {
                slot.borrow()
                    .as_ref()
                    .expect("protected live host")
                    .host
                    .observation_status()
                    .flags
            }) & crate::OBSERVATION_STATUS_FLAG_ORDINARY_AVAILABLE,
            0,
            "a successful render replenishes the Ordinary attempt"
        );

        stage_request_with_limit(b"eq0", 65_536);
        assert_eq!(miso_engine_web_v1_track_response_capture(handle), RESULT_OK);
        assert_eq!(captured_header().snapshot_token, markers.live_token + 1);
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    }

    #[test]
    fn ffi_protected_response_success_creates_no_observation_application_receipt() {
        let handle = boot_private_protected();
        stage_request_with_limit(b"eq0", 65_536);
        assert_eq!(miso_engine_web_v1_observation_application_take(handle), 0);

        assert_eq!(miso_engine_web_v1_track_response_capture(handle), RESULT_OK);
        assert_eq!(miso_engine_web_v1_observation_application_take(handle), 0);
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            assert_eq!(host.side_records.application_len, 0);
            assert_eq!(host.side_records.pending_count, 0);
            assert_eq!(host.side_records.completed_count, 0);
            assert_eq!(host.side_records.reserved_mask, 0);
            let admission = host.observation_admission();
            assert_eq!(admission.result, RESULT_OK);
            assert_eq!(admission.operation, 8);
            assert_eq!(admission.flags & crate::OBSERVATION_ADMISSION_RECEIPT, 0);
            assert_eq!(admission.receipt, WebObservationReceipt::default());
        });
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    }

    #[test]
    fn ffi_live_capture_analysis_is_prewarmed_and_token_exhaustion_is_typed() {
        let document = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
        let options = WebBootOptions {
            require_sample_rate_hz: 48_000,
            require_quantum_frames: 128,
            ..WebBootOptions::explicit_defaults()
        };
        let handle = test_boot(document.as_bytes(), options);
        assert_ne!(handle, 0, "the mixed input-filter/EQ fixture must boot");

        // Initialize every fixed response staging allocation before measuring the owner callback.
        assert_eq!(
            miso_engine_web_v1_track_response_request_bytes(),
            LIVE_RESPONSE_REQUEST_BYTES
        );
        let _ = miso_engine_web_v1_track_response_request_ptr();
        let _ = miso_engine_web_v1_track_response_track_id_ptr();
        let _ = miso_engine_web_v1_track_response_snapshot_ptr();
        assert_eq!(
            miso_engine_web_v1_track_response_snapshot_capacity(),
            LIVE_RESPONSE_CAPTURE_BYTES as u32
        );
        stage_request_with_limit(b"eq0", LIVE_RESPONSE_RESULT_BYTES);
        let (refused_capture, allocations, deallocations) =
            measured(|| miso_engine_web_v1_track_response_capture(handle));
        assert_eq!(refused_capture, RESULT_REFUSED_BUDGET);
        assert_eq!(allocations, 0, "an undersized capture refusal allocated");
        assert_eq!(deallocations, 0, "an undersized capture refusal freed");
        assert_eq!(captured_header().result, RESULT_REFUSED_BUDGET);
        assert_eq!(
            RESPONSE_STAGING.with(|slot| slot.borrow().live_token),
            0,
            "an undersized capture refusal advanced the snapshot token"
        );
        stage_request(b"eq0");

        let maximum_raw_payload = size_of::<WebLiveResponseResult>()
            + LIVE_RESPONSE_MAXIMUM_OWNERS
                * (size_of::<WebLiveResponseOwner>()
                    + 3 * LIVE_RESPONSE_MAXIMUM_ID_BYTES
                    + 2 * LIVE_RESPONSE_MAXIMUM_SECTIONS * size_of::<WebLiveResponseSection>());
        assert_eq!(maximum_raw_payload, 249_192);
        assert!(maximum_raw_payload <= LIVE_RESPONSE_CAPTURE_BYTES);

        let (capture_result, allocations, deallocations) =
            measured(|| miso_engine_web_v1_track_response_capture(handle));
        assert_eq!(capture_result, RESULT_OK);
        assert_eq!(allocations, 0, "prewarmed FFI capture allocated");
        assert_eq!(deallocations, 0, "prewarmed FFI capture freed");
        let capture_header = captured_header();
        assert_eq!(capture_header.result, RESULT_OK);
        assert_eq!(capture_header.captured_sample, 0);
        assert_eq!(capture_header.snapshot_token, 1);
        assert!(
            capture_header.owner_count >= 2,
            "input filters and EQ must both be captured"
        );
        assert!(capture_header.result_bytes > u64::from(LIVE_RESPONSE_RESULT_BYTES));

        let expected_result_bytes = RESPONSE_STAGING.with(|slot| {
            let staging = slot.borrow();
            let bytes = &staging.live_result[..staging.live_result_len];
            let header: WebLiveResponseResult = read_live_record(bytes, 0).expect("capture header");
            let owner_count = usize::try_from(header.owner_count).expect("owner count");
            let owners_start = usize::try_from(header.owners_offset).expect("owners offset");
            let mut expected =
                owners_start + LIVE_RESPONSE_MAXIMUM_OWNERS * size_of::<WebLiveResponseOwner>();
            for index in 0..owner_count {
                let owner_offset = owners_start + index * size_of::<WebLiveResponseOwner>();
                let raw: WebLiveResponseOwner =
                    read_live_record(bytes, u32::try_from(owner_offset).expect("owner offset"))
                        .expect("owner record");
                expected += usize::try_from(raw.track_id_bytes).expect("track ID bytes")
                    + usize::try_from(raw.native_id_bytes).expect("native ID bytes")
                    + usize::try_from(raw.stable_id_bytes).expect("stable ID bytes")
                    + (usize::try_from(raw.left_count).expect("left count")
                        + usize::try_from(raw.right_count).expect("right count"))
                        * size_of::<WebLiveResponseSection>();
                let native_id =
                    live_payload_bytes(bytes, raw.native_id_offset, raw.native_id_bytes)
                        .expect("native ID payload");
                if native_id == b"miso.parametric-eq" {
                    assert_eq!(raw.left_count as usize, LIVE_RESPONSE_MAXIMUM_SECTIONS);
                    assert_eq!(raw.right_count as usize, LIVE_RESPONSE_MAXIMUM_SECTIONS);
                }
                if native_id == b"miso.builtin.input-filters" {
                    assert_eq!(raw.left_count, 2);
                    assert_eq!(raw.right_count, 2);
                }
            }
            expected
        });
        assert_eq!(expected_result_bytes, capture_header.result_bytes as usize);

        let (snapshot, token) = RESPONSE_STAGING
            .with(|slot| parse_live_snapshot(&slot.borrow()).expect("captured mixed snapshot"));
        assert_eq!(token, 1);
        assert!(
            snapshot
                .owners
                .iter()
                .any(|owner| owner.native_id.as_ref() == "miso.builtin.input-filters")
        );
        assert!(
            snapshot
                .owners
                .iter()
                .any(|owner| owner.native_id.as_ref() == "miso.parametric-eq")
        );
        let eq_owner = snapshot
            .owners
            .iter()
            .find(|owner| owner.native_id.as_ref() == "miso.parametric-eq")
            .expect("EQ owner");
        assert_eq!(eq_owner.left.len(), LIVE_RESPONSE_MAXIMUM_SECTIONS);
        assert_eq!(eq_owner.right.len(), LIVE_RESPONSE_MAXIMUM_SECTIONS);
        let input_filter_owner = snapshot
            .owners
            .iter()
            .find(|owner| owner.native_id.as_ref() == "miso.builtin.input-filters")
            .expect("input-filter owner");
        assert_eq!(input_filter_owner.left.len(), 2);
        assert_eq!(input_filter_owner.right.len(), 2);

        RESPONSE_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            let result_len = staging.live_result_len;
            let eq_left_count = {
                let bytes = &staging.live_result[..result_len];
                let header: WebLiveResponseResult =
                    read_live_record(bytes, 0).expect("capture header");
                let owners_start = usize::try_from(header.owners_offset).expect("owners offset");
                let mut eq_left_count = None;
                for index in 0..usize::try_from(header.owner_count).expect("owner count") {
                    let owner_offset = owners_start + index * size_of::<WebLiveResponseOwner>();
                    let raw: WebLiveResponseOwner =
                        read_live_record(bytes, u32::try_from(owner_offset).expect("owner offset"))
                            .expect("owner record");
                    let native_id =
                        live_payload_bytes(bytes, raw.native_id_offset, raw.native_id_bytes)
                            .expect("native ID payload");
                    if native_id == b"miso.parametric-eq" {
                        eq_left_count = Some(
                            owner_offset + core::mem::offset_of!(WebLiveResponseOwner, left_count),
                        );
                    }
                }
                eq_left_count.expect("EQ owner count")
            };
            let original_count = {
                let bytes = &staging.live_result[..result_len];
                u32::from_le_bytes(
                    bytes[eq_left_count..eq_left_count + 4]
                        .try_into()
                        .expect("count"),
                )
            };
            {
                let bytes = &mut staging.live_result[..result_len];
                bytes[eq_left_count..eq_left_count + 4].copy_from_slice(&7_u32.to_le_bytes());
            }
            assert!(
                parse_live_snapshot(&staging).is_err(),
                "seven sections must be rejected"
            );
            {
                let bytes = &mut staging.live_result[..result_len];
                bytes[eq_left_count..eq_left_count + 4]
                    .copy_from_slice(&original_count.to_le_bytes());
            }
        });
        RESPONSE_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            let original_len = staging.live_result_len;
            let original_result_bytes = {
                let bytes = &staging.live_result[..original_len];
                let at = core::mem::offset_of!(WebLiveResponseResult, result_bytes);
                u64::from_le_bytes(bytes[at..at + 8].try_into().expect("result bytes"))
            };
            {
                let bytes = &mut staging.live_result[..original_len];
                let at = core::mem::offset_of!(WebLiveResponseResult, result_bytes);
                bytes[at..at + 8].copy_from_slice(
                    &u64::try_from(original_len - 1)
                        .expect("truncated length")
                        .to_le_bytes(),
                );
            }
            staging.live_result_len = original_len - 1;

            assert!(
                parse_live_snapshot(&staging).is_err(),
                "truncated sections must be rejected"
            );
            staging.live_result_len = original_len;
            let bytes = &mut staging.live_result[..original_len];
            let at = core::mem::offset_of!(WebLiveResponseResult, result_bytes);
            bytes[at..at + 8].copy_from_slice(&original_result_bytes.to_le_bytes());
        });

        assert_eq!(miso_engine_web_v1_track_response_analysis(), RESULT_OK);
        let analysis_header = captured_header();
        assert_eq!(analysis_header.result, RESULT_OK);
        assert_eq!(analysis_header.points, 5);
        assert_eq!(analysis_header.captured_sample, 0);
        assert_eq!(analysis_header.snapshot_token, 1);
        assert!(analysis_header.result_bytes > u64::from(LIVE_RESPONSE_RESULT_BYTES));
        let result_len = miso_engine_web_v1_track_response_result_bytes();
        assert_eq!(analysis_header.result_bytes, u64::from(result_len));
        RESPONSE_STAGING.with(|slot| {
            let staging = slot.borrow();
            let bytes = &staging.live_result[..staging.live_result_len];
            let values = bytes
                .chunks_exact(size_of::<f32>())
                .skip(size_of::<WebLiveResponseResult>() / size_of::<f32>())
                .take(15)
                .map(|chunk| f32::from_le_bytes(chunk.try_into().expect("f32 word")));
            assert!(values.into_iter().all(f32::is_finite));
        });

        let undersized_analysis_bytes =
            u32::try_from(size_of::<WebLiveResponseResult>() + 5 * size_of::<f32>() * 3 - 1)
                .expect("small live response bound");
        RESPONSE_STAGING.with(|slot| {
            slot.borrow_mut().live_request.maximum_result_bytes = undersized_analysis_bytes;
        });
        let (refused_analysis, allocations, deallocations) =
            measured(|| miso_engine_web_v1_track_response_analysis());
        assert_eq!(refused_analysis, RESULT_REFUSED_BUDGET);
        assert_eq!(allocations, 0, "an undersized analysis refusal allocated");
        assert_eq!(deallocations, 0, "an undersized analysis refusal freed");
        assert_eq!(captured_header().result, RESULT_REFUSED_BUDGET);

        // Closing clears payload length but must not reset the host-scoped sequence identity.
        assert_eq!(miso_engine_web_v1_track_response_close(), RESULT_OK);
        stage_request(b"eq0");
        let (second_result, allocations, deallocations) =
            measured(|| miso_engine_web_v1_track_response_capture(handle));
        assert_eq!(second_result, RESULT_OK);
        assert_eq!(allocations, 0);
        assert_eq!(deallocations, 0);
        assert_eq!(captured_header().snapshot_token, 2);

        RESPONSE_STAGING.with(|slot| slot.borrow_mut().live_token = u64::MAX);
        stage_request(b"eq0");
        let (exhausted_result, allocations, deallocations) =
            measured(|| miso_engine_web_v1_track_response_capture(handle));
        assert_eq!(exhausted_result, RESULT_REFUSED_BUDGET);
        assert_eq!(allocations, 0, "token exhaustion refusal allocated");
        assert_eq!(deallocations, 0, "token exhaustion refusal freed");
        assert_eq!(captured_header().result, RESULT_REFUSED_BUDGET);
        assert_eq!(
            RESPONSE_STAGING.with(|slot| slot.borrow().live_token),
            u64::MAX,
            "exhaustion must not wrap or reuse a snapshot token"
        );
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    }
}

#[cfg(test)]
mod spectrum_ffi_tests {
    use super::*;

    #[test]
    fn cold_spectrum_capture_configuration_is_idempotent() {
        SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
        SPECTRUM_CAPTURE_CONFIGURE_ENTRIES.with(|entries| entries.set(0));

        let first_result = SPECTRUM_STAGING.with(|slot| slot.borrow_mut().configure_capture());
        let first_entry_count = SPECTRUM_CAPTURE_CONFIGURE_ENTRIES.with(|entries| entries.get());
        let capacities = SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            staging.capture.as_ref().map(|capture| {
                (
                    capture.len(),
                    capture.capacity(),
                    staging.result.as_ref().map(Vec::len),
                    staging.result.as_ref().map(Vec::capacity),
                )
            })
        });
        let second_result = if first_result.is_ok() {
            Some(SPECTRUM_STAGING.with(|slot| slot.borrow_mut().configure_capture()))
        } else {
            None
        };
        let second_entry_count = SPECTRUM_CAPTURE_CONFIGURE_ENTRIES.with(|entries| entries.get());

        SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
        SPECTRUM_CAPTURE_CONFIGURE_ENTRIES.with(|entries| entries.set(0));

        match first_result {
            Ok(()) => {
                assert_eq!(
                    capacities,
                    Some((
                        SPECTRUM_CAPTURE_BYTES,
                        SPECTRUM_CAPTURE_BYTES,
                        Some(SPECTRUM_CAPTURE_BYTES),
                        Some(SPECTRUM_CAPTURE_BYTES),
                    ))
                );
                assert_eq!(second_result, Some(Ok(())));
                assert_eq!(first_entry_count, 1);
                assert_eq!(second_entry_count, 1);
            }
            Err(result) => {
                assert_eq!(result, RESULT_REFUSED_BUDGET);
                assert_eq!(first_entry_count, 1);
                assert_eq!(capacities, None);
                assert_eq!(second_result, None);
                eprintln!(
                    "unable to assert successful cold spectrum capture configuration: allocator refused the fixed capacity"
                );
            }
        }
    }

    #[test]
    fn spectrum_capture_capacity_validator_rejects_explicit_surplus() {
        let oversized: Vec<u8> = Vec::with_capacity(SPECTRUM_CAPTURE_BYTES + 1);
        assert!(
            oversized.capacity() > SPECTRUM_CAPTURE_BYTES,
            "fixture must supply an explicitly oversized Vec"
        );
        assert_eq!(
            validate_spectrum_capture_capacity(oversized.capacity()),
            Err(RESULT_REFUSED_BUDGET)
        );

        let mut normal: Vec<u8> = Vec::new();
        if normal.try_reserve_exact(SPECTRUM_CAPTURE_BYTES).is_ok() {
            let expected = if normal.capacity() > SPECTRUM_CAPTURE_BYTES {
                Err(RESULT_REFUSED_BUDGET)
            } else {
                Ok(())
            };
            assert_eq!(
                validate_spectrum_capture_capacity(normal.capacity()),
                expected,
                "validator must follow the actual Vec capacity"
            );
        }
    }

    fn stage_left_output_request() {
        SPECTRUM_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            staging.release_capture();
            staging.release_collection_staging();
            staging.collection_request.entry_count = 0;
            staging.collection_request.maximum_capture_bytes = 0;
            let target_id = b"main-out";
            staging.target_id.fill(0);
            staging.target_id[..target_id.len()].copy_from_slice(target_id);
            *staging.request = WebSpectrumRequest {
                struct_size: SPECTRUM_REQUEST_BYTES,
                abi_version: ABI_VERSION,
                target: SPECTRUM_TARGET_OUTPUT,
                channels: SPECTRUM_CHANNEL_LEFT,
                target_id_bytes: target_id.len() as u32,
                maximum_capture_bytes: SPECTRUM_CAPTURE_BYTES as u64,
                ..WebSpectrumRequest::default()
            };
        });
    }

    fn stage_collection_request() {
        SPECTRUM_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            staging.release_capture();
            staging.release_collection_staging();
            *staging.request = WebSpectrumRequest {
                struct_size: SPECTRUM_REQUEST_BYTES,
                abi_version: ABI_VERSION,
                ..WebSpectrumRequest::default()
            };
            *staging.collection_request = WebSpectrumCollectionRequest {
                struct_size: SPECTRUM_COLLECTION_REQUEST_BYTES,
                abi_version: ABI_VERSION,
                entry_count: 2,
                maximum_capture_bytes: (SPECTRUM_CAPTURE_BYTES * 2) as u64,
                ..WebSpectrumCollectionRequest::default()
            };
            let entries = [
                (
                    SPECTRUM_TARGET_OUTPUT,
                    SPECTRUM_CHANNEL_LEFT,
                    b"main-out".as_slice(),
                ),
                (
                    SPECTRUM_TARGET_TRACK_POST_MATRIX,
                    SPECTRUM_CHANNEL_BOTH,
                    b"eq0".as_slice(),
                ),
            ];
            staging
                .collection_entries
                .resize(entries.len(), WebSpectrumCollectionEntry::default());
            let target_id_bytes = entries.iter().map(|(_, _, id)| id.len()).sum::<usize>();
            staging.collection_target_ids.resize(target_id_bytes, 0);
            let mut id_start = 0;
            for (index, (target, channels, id)) in entries.into_iter().enumerate() {
                staging.collection_entries[index] = WebSpectrumCollectionEntry {
                    target,
                    channels,
                    target_id_bytes: id.len() as u32,
                    ..WebSpectrumCollectionEntry::default()
                };
                let id_end = id_start + id.len();
                staging.collection_target_ids[id_start..id_end].copy_from_slice(id);
                id_start = id_end;
            }
        });
    }

    #[test]
    fn collection_staging_uses_caller_count_and_packed_identity_bytes() {
        SPECTRUM_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            staging.release_capture();
            staging.release_collection_staging();
            staging.collection_request.entry_count = 257;
            staging.collection_request.maximum_capture_bytes = (SPECTRUM_CAPTURE_BYTES * 2) as u64;
        });
        let _ = miso_engine_web_v1_spectrum_collection_entry_ptr();
        assert_eq!(miso_engine_web_v1_spectrum_collection_entry_capacity(), 257);
        SPECTRUM_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            for entry in &mut staging.collection_entries {
                entry.target_id_bytes = 1;
            }
        });
        let _ = miso_engine_web_v1_spectrum_collection_target_ids_ptr();
        assert_eq!(
            miso_engine_web_v1_spectrum_collection_target_ids_capacity(),
            257
        );
        SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_collection_staging());
    }

    #[test]
    fn failed_stream_metadata_starts_new_epoch_drop_count_at_zero() {
        SPECTRUM_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            staging.stream_metadata = WebSpectrumStreamMetadata {
                capture_epoch: 7,
                dropped_captures: 12,
                ..WebSpectrumStreamMetadata::default()
            };
            stream_metadata_error(
                &mut staging,
                SPECTRUM_STREAM_STATUS_PENDING,
                RESULT_BACKPRESSURE,
                None,
                None,
            );
            assert_eq!(staging.stream_metadata.capture_epoch, 7);
            assert_eq!(staging.stream_metadata.dropped_captures, 12);
            stream_metadata_error(
                &mut staging,
                SPECTRUM_STREAM_STATUS_FAILED,
                RESULT_RENDER_REJECTED,
                Some(8),
                None,
            );
            assert_eq!(staging.stream_metadata.capture_epoch, 8);
            assert_eq!(staging.stream_metadata.dropped_captures, 0);
        });
    }

    #[test]
    fn invalid_spectrum_cancel_refuses_before_touching_managed_stream() {
        SPECTRUM_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            staging.release_capture();
            staging.stream_active = true;
            staging.capture_len = 32;
            staging.stream_metadata.result = RESULT_OK;
        });
        assert_eq!(
            miso_engine_web_v1_spectrum_cancel(0),
            RESULT_INVALID_ARGUMENT
        );
        SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert!(staging.stream_active);
            assert_eq!(staging.capture_len, 32);
            assert_eq!(staging.stream_metadata.result, RESULT_OK);
        });
        SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
    }

    #[test]
    fn invalid_spectrum_channel_selection_preserves_window_for_valid_retry() {
        stage_left_output_request();
        let document = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
        let handle = test_boot(
            document.as_bytes(),
            WebBootOptions {
                require_sample_rate_hz: 48_000,
                require_quantum_frames: 128,
                ..WebBootOptions::explicit_defaults()
            },
        );
        assert_ne!(handle, 0, "spectrum fixture must boot");
        assert_eq!(
            test_copy_staging(handle, BUFFER_SOURCE_ID, b"fixture-source"),
            RESULT_OK
        );
        assert_eq!(test_fill_source_pcm(handle, 0.25), RESULT_OK);
        assert_eq!(miso_engine_web_v1_spectrum_arm(handle), RESULT_OK);
        for block in 0..16_u64 {
            assert_eq!(
                miso_engine_web_v1_source_submit(handle, 14, 1, block * 128, 2, 128, 0),
                RESULT_OK,
                "source block {block}"
            );
            assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        }

        // Stereo is a valid request shape but exceeds the left-only prepared capture. The refusal
        // must happen before `read_spectrum` consumes the completed one-shot window.
        assert_eq!(
            miso_engine_web_v1_spectrum_read(handle, SPECTRUM_CHANNEL_BOTH),
            RESULT_INVALID_ARGUMENT
        );
        assert_eq!(miso_engine_web_v1_spectrum_capture_bytes(), 0);
        assert_eq!(
            miso_engine_web_v1_spectrum_read(handle, SPECTRUM_CHANNEL_LEFT),
            RESULT_OK,
            "a valid retry must still read the completed window"
        );
        assert_eq!(
            miso_engine_web_v1_spectrum_capture_bytes(),
            SPECTRUM_WINDOW_HEADER_BYTES + SPECTRUM_WINDOW_FRAMES * size_of::<f32>() as u32
        );
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    }

    #[test]
    fn collection_selection_is_atomic_and_keeps_one_active_capture() {
        stage_collection_request();
        let document = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
        let handle = test_boot(
            document.as_bytes(),
            WebBootOptions {
                require_sample_rate_hz: 48_000,
                require_quantum_frames: 128,
                ..WebBootOptions::explicit_defaults()
            },
        );
        assert_ne!(handle, 0, "collection spectrum fixture must boot");
        assert_eq!(miso_engine_web_v1_spectrum_selection_epoch(handle), 0);

        assert_eq!(
            test_copy_staging(handle, BUFFER_SOURCE_ID, b"fixture-source"),
            RESULT_OK
        );
        assert_eq!(test_fill_source_pcm(handle, 0.25), RESULT_OK);
        SPECTRUM_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            let id = b"main-out";
            staging.target_id.fill(0);
            staging.target_id[..id.len()].copy_from_slice(id);
        });
        assert_eq!(
            miso_engine_web_v1_spectrum_select(
                handle,
                SPECTRUM_TARGET_OUTPUT,
                SPECTRUM_CHANNEL_LEFT,
                8,
            ),
            RESULT_OK
        );
        assert_eq!(miso_engine_web_v1_spectrum_selection_epoch(handle), 1);
        assert_eq!(
            miso_engine_web_v1_spectrum_target_id_capacity(),
            SPECTRUM_MAXIMUM_ID_BYTES as u32
        );

        for block in 0..8_u64 {
            assert_eq!(
                miso_engine_web_v1_source_submit(handle, 14, 1, block * 128, 2, 128, 0),
                RESULT_OK,
                "source block {block}"
            );
            assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        }

        // An unprepared identity is rejected before the old partial capture is touched.
        SPECTRUM_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            let id = b"missing";
            staging.target_id.fill(0);
            staging.target_id[..id.len()].copy_from_slice(id);
        });
        assert_eq!(
            miso_engine_web_v1_spectrum_select(
                handle,
                SPECTRUM_TARGET_OUTPUT,
                SPECTRUM_CHANNEL_LEFT,
                7,
            ),
            RESULT_INVALID_ARGUMENT
        );
        assert_eq!(miso_engine_web_v1_spectrum_selection_epoch(handle), 1);
        assert_eq!(
            with_host(handle, 0, AudioWorkletEngineHost::spectrum_target),
            SPECTRUM_TARGET_OUTPUT
        );
        assert_eq!(
            with_host(handle, 0, AudioWorkletEngineHost::spectrum_channels),
            SPECTRUM_CHANNEL_LEFT
        );

        for block in 8..16_u64 {
            assert_eq!(
                miso_engine_web_v1_source_submit(handle, 14, 1, block * 128, 2, 128, 0),
                RESULT_OK,
                "source block {block}"
            );
            assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        }
        assert_eq!(
            miso_engine_web_v1_spectrum_read(handle, SPECTRUM_CHANNEL_LEFT),
            RESULT_OK
        );

        SPECTRUM_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            let id = b"eq0";
            staging.target_id.fill(0);
            staging.target_id[..id.len()].copy_from_slice(id);
        });
        assert_eq!(
            miso_engine_web_v1_spectrum_select(
                handle,
                SPECTRUM_TARGET_TRACK_POST_MATRIX,
                SPECTRUM_CHANNEL_BOTH,
                3,
            ),
            RESULT_OK
        );
        assert_eq!(miso_engine_web_v1_spectrum_selection_epoch(handle), 2);
        assert_eq!(
            with_host(handle, 0, AudioWorkletEngineHost::spectrum_target),
            SPECTRUM_TARGET_TRACK_POST_MATRIX
        );
        assert_eq!(
            with_host(handle, 0, AudioWorkletEngineHost::spectrum_channels),
            SPECTRUM_CHANNEL_BOTH
        );

        // A managed stream must refresh its copied profile on a live collection switch. The
        // stale-start target would make the following ready window look like the old entry.
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        assert_eq!(
            SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata.target),
            SPECTRUM_TARGET_TRACK_POST_MATRIX
        );
        assert!(SPECTRUM_STAGING.with(|slot| slot.borrow().stream_history.is_none()));
        for block in 16..32_u64 {
            assert_eq!(
                miso_engine_web_v1_source_submit(handle, 14, 1, block * 128, 2, 128, 0),
                RESULT_OK,
                "stream source block {block}"
            );
            assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        }
        assert_eq!(miso_engine_web_v1_spectrum_stream_read(handle), RESULT_OK);
        assert_eq!(
            SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata.target),
            SPECTRUM_TARGET_TRACK_POST_MATRIX
        );

        // Repeating the exact prepared selection is an idempotent no-op. It must preserve the
        // ready window and its sequence instead of spuriously resetting the live stream.
        SPECTRUM_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            let id = b"eq0";
            staging.target_id.fill(0);
            staging.target_id[..id.len()].copy_from_slice(id);
        });
        let before_noop = SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata);
        assert_eq!(
            miso_engine_web_v1_spectrum_select(
                handle,
                SPECTRUM_TARGET_TRACK_POST_MATRIX,
                SPECTRUM_CHANNEL_BOTH,
                3,
            ),
            RESULT_OK
        );
        let after_noop = SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata);
        assert_eq!(miso_engine_web_v1_spectrum_selection_epoch(handle), 2);
        assert_eq!(after_noop.status, SPECTRUM_STREAM_STATUS_READY);
        assert_eq!(after_noop.target, before_noop.target);
        assert_eq!(after_noop.channels, before_noop.channels);
        assert_eq!(after_noop.sequence, before_noop.sequence);
        assert_eq!(after_noop.windows, before_noop.windows);
        assert!(SPECTRUM_STAGING.with(|slot| slot.borrow().stream_history.is_none()));

        // Smoothing is part of the same transaction. An invalid configuration must leave both
        // the selected entry and the ready stream untouched.
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_select(
                handle,
                SPECTRUM_TARGET_OUTPUT,
                SPECTRUM_CHANNEL_LEFT,
                8,
                f64::NAN,
            ),
            RESULT_INVALID_ARGUMENT
        );
        let after_refusal = SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata);
        assert_eq!(after_refusal.status, SPECTRUM_STREAM_STATUS_READY);
        assert_eq!(after_refusal.target, SPECTRUM_TARGET_TRACK_POST_MATRIX);
        assert_eq!(after_refusal.channels, SPECTRUM_CHANNEL_BOTH);
        assert_eq!(after_refusal.smoothing_ms, 0.0);
        assert_eq!(
            with_host(handle, 0, AudioWorkletEngineHost::spectrum_target),
            SPECTRUM_TARGET_TRACK_POST_MATRIX
        );

        SPECTRUM_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            let id = b"main-out";
            staging.target_id.fill(0);
            staging.target_id[..id.len()].copy_from_slice(id);
        });
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_select(
                handle,
                SPECTRUM_TARGET_OUTPUT,
                SPECTRUM_CHANNEL_LEFT,
                8,
                250.5,
            ),
            RESULT_OK
        );
        let selected = SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata);
        assert_eq!(selected.status, SPECTRUM_STREAM_STATUS_WARMING);
        assert_eq!(selected.target, SPECTRUM_TARGET_OUTPUT);
        assert_eq!(selected.channels, SPECTRUM_CHANNEL_LEFT);
        assert_eq!(selected.smoothing_ms, 250.5);
        assert!(SPECTRUM_STAGING.with(|slot| slot.borrow().stream_history.is_none()));
        for block in 32..48_u64 {
            assert_eq!(
                miso_engine_web_v1_source_submit(handle, 14, 1, block * 128, 2, 128, 0),
                RESULT_OK,
                "switched stream source block {block}"
            );
            assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        }
        assert_eq!(miso_engine_web_v1_spectrum_stream_read(handle), RESULT_OK);
        let switched = SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata);
        assert_eq!(switched.status, SPECTRUM_STREAM_STATUS_READY);
        assert_eq!(switched.target, SPECTRUM_TARGET_OUTPUT);
        assert_eq!(switched.channels, SPECTRUM_CHANNEL_LEFT);

        // The first analysis creates the sole worker-side history. A same-entry smoothing update
        // must restart the native window and clear a queued old result while retaining the new
        // history epoch; it must not allocate another analyzer on the host side.
        assert_eq!(miso_engine_web_v1_spectrum_stream_analysis(), RESULT_OK);
        let analyzed = SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata);
        assert_eq!(analyzed.analysis_epoch, 0);
        assert!(SPECTRUM_STAGING.with(|slot| slot.borrow().stream_history.is_some()));
        for block in 48..64_u64 {
            assert_eq!(
                miso_engine_web_v1_source_submit(handle, 14, 1, block * 128, 2, 128, 0),
                RESULT_OK,
                "queued old stream source block {block}"
            );
            assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        }
        let before_smoothing_restart = SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata);
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_select(
                handle,
                SPECTRUM_TARGET_OUTPUT,
                SPECTRUM_CHANNEL_LEFT,
                8,
                500.25,
            ),
            RESULT_OK
        );
        let after_smoothing_restart = SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata);
        assert_eq!(
            after_smoothing_restart.status,
            SPECTRUM_STREAM_STATUS_WARMING
        );
        assert_eq!(after_smoothing_restart.target, SPECTRUM_TARGET_OUTPUT);
        assert_eq!(after_smoothing_restart.channels, SPECTRUM_CHANNEL_LEFT);
        assert_eq!(after_smoothing_restart.smoothing_ms, 500.25);
        assert_eq!(after_smoothing_restart.sequence, 0);
        assert_eq!(after_smoothing_restart.windows, 0);
        assert_eq!(
            after_smoothing_restart.capture_epoch,
            before_smoothing_restart.capture_epoch + 1
        );
        assert_eq!(after_smoothing_restart.analysis_epoch, 1);
        assert_eq!(miso_engine_web_v1_spectrum_capture_bytes(), 0);
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_read(handle),
            RESULT_BACKPRESSURE
        );
        assert_eq!(
            SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata.status),
            SPECTRUM_STREAM_STATUS_WARMING
        );
        SPECTRUM_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            let id = b"eq0";
            staging.target_id.fill(0);
            staging.target_id[..id.len()].copy_from_slice(id);
        });
        assert_eq!(
            miso_engine_web_v1_spectrum_select(
                handle,
                SPECTRUM_TARGET_TRACK_POST_MATRIX,
                SPECTRUM_CHANNEL_BOTH,
                3,
            ),
            RESULT_OK
        );
        let after_target_restart = SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata);
        assert_eq!(after_target_restart.status, SPECTRUM_STREAM_STATUS_WARMING);
        assert_eq!(miso_engine_web_v1_spectrum_selection_epoch(handle), 4);
        assert_eq!(after_target_restart.analysis_epoch, 2);
        assert_eq!(
            after_target_restart.target,
            SPECTRUM_TARGET_TRACK_POST_MATRIX
        );
        assert_eq!(after_target_restart.channels, SPECTRUM_CHANNEL_BOTH);
        assert!(SPECTRUM_STAGING.with(|slot| slot.borrow().stream_history.is_some()));
        assert_eq!(miso_engine_web_v1_spectrum_stream_stop(handle), RESULT_OK);
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    }

    #[test]
    fn continuous_spectrum_ffi_reports_profile_and_analyzes_one_window() {
        stage_left_output_request();
        let document = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
        let handle = test_boot(
            document.as_bytes(),
            WebBootOptions {
                require_sample_rate_hz: 48_000,
                require_quantum_frames: 128,
                ..WebBootOptions::explicit_defaults()
            },
        );
        assert_ne!(handle, 0, "spectrum fixture must boot");
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        let started = SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata);
        assert_eq!(started.status, SPECTRUM_STREAM_STATUS_WARMING);
        assert_eq!(started.sample_rate_hz, 48_000);
        assert_eq!(started.quantum_frames, 128);
        assert_eq!(started.hop_frames, 2_048);
        assert_eq!(started.channels, SPECTRUM_CHANNEL_LEFT);
        assert_eq!(started.smoothing_ms, 0.0);

        assert_eq!(
            test_copy_staging(handle, BUFFER_SOURCE_ID, b"fixture-source"),
            RESULT_OK
        );
        assert_eq!(test_fill_source_pcm(handle, 0.25), RESULT_OK);
        for block in 0..16_u64 {
            assert_eq!(
                miso_engine_web_v1_source_submit(handle, 14, 1, block * 128, 2, 128, 0),
                RESULT_OK,
                "source block {block}"
            );
            assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        }
        assert_eq!(miso_engine_web_v1_spectrum_stream_read(handle), RESULT_OK);
        let captured = SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata);
        assert_eq!(captured.status, SPECTRUM_STREAM_STATUS_READY);
        assert_eq!(captured.capture_epoch, 1);
        assert_eq!(captured.sequence, 0);
        assert_eq!(captured.windows, 1);
        assert_eq!(captured.captured_sample, 0);
        assert_eq!(captured.end_sample, 2_048);
        assert_eq!(miso_engine_web_v1_spectrum_stream_analysis(), RESULT_OK);
        let analyzed = SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata);
        assert_eq!(analyzed.status, SPECTRUM_STREAM_STATUS_READY);
        assert_eq!(analyzed.analysis_epoch, 0);
        assert_eq!(analyzed.history_start_sample, 0);
        assert_eq!(analyzed.smoothing_ms, 0.0);
        assert!(SPECTRUM_STAGING.with(|slot| slot.borrow().result_len > 0));
        assert_eq!(miso_engine_web_v1_spectrum_stream_stop(handle), RESULT_OK);
        assert_eq!(
            SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata.status),
            SPECTRUM_STREAM_STATUS_STOPPED
        );
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    }

    #[test]
    fn continuous_spectrum_ffi_imports_metadata_without_a_host_handle() {
        stage_left_output_request();
        SPECTRUM_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            staging.configure_capture().expect("fixed spectrum staging");
            let mut cursor = SPECTRUM_WINDOW_HEADER_BYTES as usize;
            let samples = [0.0_f32; host_core::SPECTRUM_WINDOW_FRAMES];
            let capture = staging.capture.as_mut().expect("capture staging");
            append_spectrum_f32(capture, &mut cursor, &samples).expect("left plane");
            let header = WebSpectrumWindow {
                struct_size: SPECTRUM_WINDOW_HEADER_BYTES,
                abi_version: ABI_VERSION,
                target: SPECTRUM_TARGET_OUTPUT,
                channels: SPECTRUM_CHANNEL_LEFT,
                sample_rate_hz: 48_000,
                frames: SPECTRUM_WINDOW_FRAMES,
                captured_sample: 0,
                end_sample: u64::from(SPECTRUM_WINDOW_FRAMES),
                snapshot_token: 1,
                left_offset: SPECTRUM_WINDOW_HEADER_BYTES,
                ..WebSpectrumWindow::default()
            };
            assert!(copy_live_record(capture, 0, &header));
            staging.capture_len = cursor;
            staging.stream_metadata = WebSpectrumStreamMetadata {
                struct_size: SPECTRUM_STREAM_METADATA_BYTES,
                abi_version: ABI_VERSION,
                result: RESULT_OK,
                status: SPECTRUM_STREAM_STATUS_READY,
                target: SPECTRUM_TARGET_OUTPUT,
                channels: SPECTRUM_CHANNEL_LEFT,
                sample_rate_hz: 48_000,
                quantum_frames: 128,
                hop_frames: 2_048,
                capture_epoch: 1,
                sequence: 0,
                windows: 1,
                captured_sample: 0,
                end_sample: u64::from(SPECTRUM_WINDOW_FRAMES),
                smoothing_ms: 0.0,
                ..WebSpectrumStreamMetadata::default()
            };
        });

        assert_eq!(
            miso_engine_web_v1_spectrum_stream_analysis_configure(),
            RESULT_OK
        );
        assert_eq!(miso_engine_web_v1_spectrum_stream_analysis(), RESULT_OK);
        let metadata = SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata);
        assert_eq!(metadata.status, SPECTRUM_STREAM_STATUS_READY);
        assert_eq!(metadata.capture_epoch, 1);
        assert_eq!(metadata.sequence, 0);
        assert_eq!(metadata.analysis_epoch, 0);
        assert!(SPECTRUM_STAGING.with(|slot| slot.borrow().result_len > 0));

        SPECTRUM_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            let mut header: WebSpectrumWindow =
                read_live_record(staging.capture.as_ref().expect("capture staging"), 0)
                    .expect("window header");
            header.target = SPECTRUM_TARGET_TRACK_POST_MATRIX;
            assert!(copy_live_record(
                staging.capture.as_mut().expect("capture staging"),
                0,
                &header
            ));
        });
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_analysis_configure(),
            RESULT_INVALID_ARGUMENT,
            "metadata/raw target disagreement must be rejected before analysis"
        );
    }

    fn stage_imported_left_window(first_sample: u64, sample_rate_hz: u32, token: u64, value: f32) {
        SPECTRUM_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            staging.configure_capture().expect("fixed spectrum staging");
            let window = SpectrumWindow {
                left: [value; host_core::SPECTRUM_WINDOW_FRAMES],
                right: [0.0; host_core::SPECTRUM_WINDOW_FRAMES],
                first_sample,
                channels: SpectrumChannels::Left,
                source_underrun: false,
            };
            assert_eq!(
                write_spectrum_window(
                    &mut staging,
                    &window,
                    SPECTRUM_TARGET_OUTPUT,
                    sample_rate_hz,
                    token,
                ),
                RESULT_OK
            );
        });
    }

    fn stage_imported_stream_metadata(
        first_sample: u64,
        sequence: u64,
        sample_rate_hz: u32,
        smoothing_ms: f64,
    ) {
        SPECTRUM_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            let end_sample = first_sample
                .checked_add(u64::from(SPECTRUM_WINDOW_FRAMES))
                .expect("test sample range");
            staging.stream_metadata = WebSpectrumStreamMetadata {
                struct_size: SPECTRUM_STREAM_METADATA_BYTES,
                abi_version: ABI_VERSION,
                result: RESULT_OK,
                status: SPECTRUM_STREAM_STATUS_READY,
                target: SPECTRUM_TARGET_OUTPUT,
                channels: SPECTRUM_CHANNEL_LEFT,
                sample_rate_hz,
                quantum_frames: 128,
                hop_frames: 2_048,
                capture_epoch: 1,
                sequence,
                windows: sequence.checked_add(1).expect("test window count"),
                captured_sample: first_sample,
                end_sample,
                smoothing_ms,
                ..WebSpectrumStreamMetadata::default()
            };
        });
    }

    fn spectrum_result_header() -> WebSpectrumResult {
        SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            read_live_record(staging.result.as_ref().expect("spectrum result staging"), 0)
                .expect("spectrum result header")
        })
    }

    #[test]
    fn imported_stream_analysis_does_not_poison_one_shot_or_stream_history() {
        stage_left_output_request();

        // Import and analyze the first continuous window. This leaves the worker history armed
        // while the raw staging area is reused for the one-shot query below.
        stage_imported_left_window(0, 48_000, 1, 0.25);
        stage_imported_stream_metadata(0, 0, 48_000, 100.0);
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_analysis_configure(),
            RESULT_OK
        );
        assert_eq!(miso_engine_web_v1_spectrum_stream_analysis(), RESULT_OK);
        let first_stream = SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata);
        assert_eq!(first_stream.status, SPECTRUM_STREAM_STATUS_READY);
        assert_eq!(first_stream.analysis_epoch, 0);
        assert_eq!(first_stream.history_start_sample, 0);
        assert_eq!(first_stream.sequence, 0);

        // A one-shot window may use a different launch rate and sample boundary. Its result must
        // come from the one-shot analyzer and must not reset or advance stream history merely
        // because imported stream state remains active in this worker.
        stage_imported_left_window(8_192, 44_100, 9, 0.5);
        assert_eq!(miso_engine_web_v1_spectrum_analysis(), RESULT_OK);
        let one_shot = spectrum_result_header();
        assert_eq!(one_shot.sample_rate_hz, 44_100);
        assert_eq!(one_shot.captured_sample, 8_192);
        assert_eq!(one_shot.snapshot_token, 9);
        assert_eq!(
            SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata),
            first_stream,
            "one-shot analysis must not consume continuous history"
        );

        // The next imported stream window must continue the original history rather than
        // starting a new analysis epoch after the one-shot query.
        stage_imported_left_window(2_048, 48_000, 10, 0.75);
        stage_imported_stream_metadata(2_048, 1, 48_000, 100.0);
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_analysis_configure(),
            RESULT_OK
        );
        assert_eq!(miso_engine_web_v1_spectrum_stream_analysis(), RESULT_OK);
        let second_stream = SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata);
        assert_eq!(second_stream.status, SPECTRUM_STREAM_STATUS_READY);
        assert_eq!(second_stream.sequence, 1);
        assert_eq!(second_stream.captured_sample, 2_048);
        assert_eq!(second_stream.analysis_epoch, first_stream.analysis_epoch);
        assert_eq!(
            second_stream.history_start_sample,
            first_stream.history_start_sample
        );
        assert_eq!(spectrum_result_header().captured_sample, 2_048);

        stage_left_output_request();
    }
}

#[cfg(test)]
mod observation_checkpoint_a_tests {
    use super::*;
    use crate::{WebObservationIngressLimits, WebObservationWorkLimits};

    pub(super) fn protected_preparation_record() -> WebObservationPreparationRecord {
        let mut record = WebObservationPreparationRecord {
            struct_size: crate::OBSERVATION_PREPARATION_BYTES,
            abi_version: ABI_VERSION,
            profile: crate::OBSERVATION_PROFILE_EQ_SPECTRUM,
            meter_count: 0,
            resident_taps: 0,
            spectrum_count: 1,
            maximum_active_observers: 1,
            reserved0: 0,
            activation_maximum_retained_bytes: u64::MAX,
            work_limits: WebObservationWorkLimits {
                struct_size: crate::OBSERVATION_WORK_LIMITS_BYTES,
                abi_version: ABI_VERSION,
                maximum_active_meter_channels: u64::MAX - 1,
                maximum_meter_samples_per_block: u64::MAX - 2,
                maximum_meter_publications_per_block: u64::MAX - 3,
                maximum_meter_publication_bytes_per_block: u64::MAX - 4,
                maximum_active_spectrum_captures: u64::MAX - 5,
                maximum_capture_input_samples_per_block: u64::MAX - 6,
                maximum_capture_copy_samples_per_block: u64::MAX - 7,
                maximum_capture_publications_per_block: u64::MAX - 8,
                maximum_capture_bytes_per_second: u64::MAX - 9,
                maximum_transition_entry_visits_per_block: u64::MAX - 10,
                maximum_retained_bytes: u64::MAX - 11,
            },
            ingress_limits: WebObservationIngressLimits {
                struct_size: crate::OBSERVATION_INGRESS_LIMITS_BYTES,
                abi_version: ABI_VERSION,
                maximum_control_bytes: 8_192,
                maximum_observation_rows: 32,
                maximum_result_bytes: 65_536,
                ordinary_operations_per_boundary: 1,
                removal_operations_per_boundary: 1,
                maximum_admission_entry_visits: u64::MAX - 12,
                maximum_response_binding_visits: u64::MAX - 13,
                maximum_response_section_visits: u64::MAX - 14,
                maximum_response_copy_bytes: u64::MAX - 15,
                maximum_handler_copy_bytes_per_boundary: u64::MAX - 16,
                maximum_cleanup_entry_visits_per_boundary: u64::MAX - 17,
                maximum_retained_bytes: u64::MAX,
            },
            spectrum_request: WebSpectrumRequest {
                struct_size: SPECTRUM_REQUEST_BYTES,
                abi_version: ABI_VERSION,
                target: SPECTRUM_TARGET_TRACK_POST_MATRIX,
                channels: SPECTRUM_CHANNEL_BOTH,
                target_id_bytes: 3,
                maximum_capture_bytes: SPECTRUM_CAPTURE_BYTES as u64,
                ..WebSpectrumRequest::default()
            },
            target_id: [0; 128],
        };
        record.target_id[..3].copy_from_slice(b"eq0");
        record
    }

    pub(super) fn stage_protected_boot(document: &[u8], record: WebObservationPreparationRecord) {
        BOOT_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            *staging.options = WebBootOptions {
                require_sample_rate_hz: 48_000,
                require_quantum_frames: 128,
                ..WebBootOptions::explicit_defaults()
            };
        });
        OBSERVATION_STAGING.with(|slot| {
            slot.borrow_mut().endpoint.preparation = record;
        });
        test_stage_document(document);
    }

    pub(super) fn protected_document() -> &'static [u8] {
        include_bytes!("../../../fixtures/session/v1/parametric-eq-nine-track.json")
    }

    fn one_track_protected_document() -> String {
        let mut model = session::parse_session_json(
            core::str::from_utf8(protected_document()).expect("protected fixture is UTF-8"),
        )
        .expect("protected fixture parses");
        assert_eq!(model.sample_rate_hz, 48_000);
        model.quantum_frames = 128;
        model.tracks.truncate(1);
        model.routes.truncate(1);
        session::canonical_session_json(&model).expect("canonical one-track protected session")
    }

    pub(super) fn no_live_host() {
        assert!(LIVE_HOST.with(|slot| slot.borrow().is_none()));
    }

    #[test]
    fn private_protected_boot_is_dormant_and_forwards_explicit_limits() {
        no_live_host();
        let record = protected_preparation_record();
        stage_protected_boot(protected_document(), record);

        let handle = boot_staged_observation_demand(protected_document().len() as u32);
        assert_ne!(handle, 0, "valid protected preparation must boot");
        assert_eq!(miso_engine_web_v1_boot_result(), RESULT_OK);

        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let live = live.as_ref().expect("protected handle published");
            assert_eq!(live.handle, handle);
            let ready = live.host.ready.as_ref().expect("ready owner");
            let crate::PreparedObservationStorage::Protected(storage) = &ready.observation else {
                panic!("private protected boot fell back to legacy storage");
            };
            assert!(ready.observation.legacy().is_none());
            assert_eq!(storage.controller.spectrum_state().accepted_generation, 0);
            assert_eq!(storage.controller.spectrum_state().applied_generation, 0);
            assert_eq!(storage.ingress.epoch, 1);
            assert_eq!(storage.ingress.limits.maximum_control_bytes, 8_192);
            assert_eq!(
                (
                    storage.ingress.limits.maximum_observation_rows,
                    storage.ingress.limits.maximum_result_bytes,
                    storage.ingress.limits.ordinary_operations_per_boundary,
                    storage.ingress.limits.removal_operations_per_boundary,
                ),
                (
                    record.ingress_limits.maximum_observation_rows,
                    record.ingress_limits.maximum_result_bytes,
                    record.ingress_limits.ordinary_operations_per_boundary,
                    record.ingress_limits.removal_operations_per_boundary,
                )
            );
            assert_eq!(
                (
                    storage.ingress.limits.maximum_admission_entry_visits,
                    storage.ingress.limits.maximum_response_binding_visits,
                    storage.ingress.limits.maximum_response_section_visits,
                    storage.ingress.limits.maximum_response_copy_bytes,
                    storage
                        .ingress
                        .limits
                        .maximum_handler_copy_bytes_per_boundary,
                    storage
                        .ingress
                        .limits
                        .maximum_cleanup_entry_visits_per_boundary,
                    storage.ingress.limits.maximum_retained_bytes,
                ),
                (
                    record.ingress_limits.maximum_admission_entry_visits,
                    record.ingress_limits.maximum_response_binding_visits,
                    record.ingress_limits.maximum_response_section_visits,
                    record.ingress_limits.maximum_response_copy_bytes,
                    record
                        .ingress_limits
                        .maximum_handler_copy_bytes_per_boundary,
                    record
                        .ingress_limits
                        .maximum_cleanup_entry_visits_per_boundary,
                    record.ingress_limits.maximum_retained_bytes,
                )
            );
        });

        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
        no_live_host();

        let mut too_low = record;
        too_low
            .work_limits
            .maximum_transition_entry_visits_per_block = 3;
        stage_protected_boot(protected_document(), too_low);
        assert_eq!(
            boot_staged_observation_demand(protected_document().len() as u32),
            0,
            "native preparation must receive the supplied transition limit"
        );
        assert_eq!(miso_engine_web_v1_boot_result(), RESULT_REFUSED_BUDGET);
        no_live_host();
    }

    #[test]
    fn public_protected_boot_forwards_owner_and_refuses_under_budget() {
        no_live_host();
        let document = protected_document();
        let record = protected_preparation_record();
        stage_protected_boot(document, record);

        let handle = miso_engine_web_v1_boot_with_observation_demand(document.len() as u32);
        assert_ne!(handle, 0, "public protected boot must publish a handle");
        assert_eq!(miso_engine_web_v1_boot_result(), RESULT_OK);

        let retained = LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let live = live.as_ref().expect("public protected handle");
            assert_eq!(live.host.status().state, STATE_READY);
            let status = live.host.observation_status();
            assert_eq!(status.profile, crate::OBSERVATION_PROFILE_EQ_SPECTRUM);
            assert_ne!(status.owner, 0);
            let ready = live.host.ready.as_ref().expect("ready owner");
            let crate::PreparedObservationStorage::Protected(storage) = &ready.observation else {
                panic!("public protected boot fell back to legacy storage");
            };
            assert_eq!(status.owner, storage.controller.owner().get());
            storage.ingress.bounds.retained_bytes
        });
        assert!(retained > 0);

        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
        no_live_host();

        let mut too_low = record;
        too_low.ingress_limits.maximum_retained_bytes = retained - 1;
        stage_protected_boot(document, too_low);
        assert_eq!(
            miso_engine_web_v1_boot_with_observation_demand(document.len() as u32),
            0,
            "public protected boot must refuse one byte below retained budget"
        );
        assert_eq!(miso_engine_web_v1_boot_result(), RESULT_REFUSED_BUDGET);
        let diagnostic_bytes = miso_engine_web_v1_boot_diagnostic_bytes();
        assert!(
            diagnostic_bytes > 0,
            "budget refusal must publish a boot diagnostic"
        );
        no_live_host();
    }

    #[test]
    fn public_protected_boot_refuses_fixed_spectrum_payload_budget_floor() {
        const PROTECTED_PAYLOAD_FLOOR_BYTES: u64 = 2_097_152;
        const GENEROUS_BUDGET_BYTES: u64 = 67_108_864;

        struct BudgetCase {
            name: &'static str,
            host_maximum_memory_bytes: u64,
            ingress_maximum_retained_bytes: u64,
            diagnostic_prefix: &'static [u8],
        }

        let cases = [
            BudgetCase {
                name: "host-only",
                host_maximum_memory_bytes: PROTECTED_PAYLOAD_FLOOR_BYTES,
                ingress_maximum_retained_bytes: GENEROUS_BUDGET_BYTES,
                diagnostic_prefix: b"host.budget.retained_projection\t",
            },
            BudgetCase {
                name: "ingress-only",
                host_maximum_memory_bytes: GENEROUS_BUDGET_BYTES,
                ingress_maximum_retained_bytes: PROTECTED_PAYLOAD_FLOOR_BYTES,
                diagnostic_prefix: b"web.observation.ingress.maximum_retained_bytes\t",
            },
            BudgetCase {
                name: "both-low",
                host_maximum_memory_bytes: PROTECTED_PAYLOAD_FLOOR_BYTES,
                ingress_maximum_retained_bytes: PROTECTED_PAYLOAD_FLOOR_BYTES,
                diagnostic_prefix: b"host.budget.retained_projection\t",
            },
        ];
        let document = one_track_protected_document();

        no_live_host();
        for case in cases {
            SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
            SPECTRUM_CAPTURE_CONFIGURE_ENTRIES.with(|entries| entries.set(0));

            let mut record = protected_preparation_record();
            record.ingress_limits.maximum_retained_bytes = case.ingress_maximum_retained_bytes;
            stage_protected_boot(document.as_bytes(), record);
            BOOT_STAGING.with(|slot| {
                slot.borrow_mut().options.maximum_memory_bytes = case.host_maximum_memory_bytes;
            });
            test_stage_document(document.as_bytes());

            let handle = miso_engine_web_v1_boot_with_observation_demand(document.len() as u32);
            assert_eq!(
                handle, 0,
                "protected boot must refuse the independent payload floor for {}",
                case.name
            );
            assert_eq!(
                miso_engine_web_v1_boot_result(),
                RESULT_REFUSED_BUDGET,
                "protected boot result for {}",
                case.name
            );
            let diagnostic = BOOT_STAGING.with(|slot| {
                let staging = slot.borrow();
                let length = staging.diagnostic_bytes as usize;
                assert!(length <= staging.document.len(), "bounded boot diagnostic");
                staging.document[..length].to_vec()
            });
            assert!(
                diagnostic.starts_with(case.diagnostic_prefix),
                "unexpected {} diagnostic: {:?}",
                case.name,
                String::from_utf8_lossy(&diagnostic)
            );
            assert_eq!(
                SPECTRUM_CAPTURE_CONFIGURE_ENTRIES.with(|entries| entries.get()),
                0,
                "under-budget {} boot entered spectrum configuration",
                case.name
            );
            no_live_host();
        }
    }

    #[test]
    fn public_protected_boot_preserves_endpoint_on_budget_refusal_and_recovers() {
        const PROTECTED_PAYLOAD_FLOOR_BYTES: u64 = 2_097_152;
        const GENEROUS_BUDGET_BYTES: u64 = 67_108_864;

        no_live_host();
        SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
        SPECTRUM_CAPTURE_CONFIGURE_ENTRIES.with(|entries| entries.set(0));

        let document = one_track_protected_document();
        let mut refusal_record = protected_preparation_record();
        refusal_record.ingress_limits.maximum_retained_bytes = GENEROUS_BUDGET_BYTES;
        stage_protected_boot(document.as_bytes(), refusal_record);
        BOOT_STAGING.with(|slot| {
            slot.borrow_mut().options.maximum_memory_bytes = PROTECTED_PAYLOAD_FLOOR_BYTES;
        });

        let expected_outputs = OBSERVATION_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            staging.endpoint.status.owner = 0x1111_2222_3333_4444;
            staging.endpoint.admission.operation = 0x5555_6666;
            staging.endpoint.capture_identity.snapshot_token = 0x7777_8888_9999_AAAA;
            staging.endpoint.applications[0].sequence = 0xBBBB_CCCC_DDDD_EEEE;
            staging.endpoint.application_count = 1;
            staging.endpoint.handle = 0xF00D_BAAD;
            (
                staging.endpoint.status.owner,
                staging.endpoint.admission.operation,
                staging.endpoint.capture_identity.snapshot_token,
                staging.endpoint.applications[0].sequence,
                staging.endpoint.application_count,
                staging.endpoint.handle,
            )
        });

        assert_eq!(
            miso_engine_web_v1_boot_with_observation_demand(document.len() as u32),
            0,
            "under-budget protected boot must not publish a handle"
        );
        assert_eq!(miso_engine_web_v1_boot_result(), RESULT_REFUSED_BUDGET);
        assert_eq!(
            SPECTRUM_CAPTURE_CONFIGURE_ENTRIES.with(|entries| entries.get()),
            0,
            "under-budget protected boot must not enter spectrum configuration"
        );
        no_live_host();
        let actual_outputs = OBSERVATION_STAGING.with(|slot| {
            let staging = slot.borrow();
            (
                staging.endpoint.status.owner,
                staging.endpoint.admission.operation,
                staging.endpoint.capture_identity.snapshot_token,
                staging.endpoint.applications[0].sequence,
                staging.endpoint.application_count,
                staging.endpoint.handle,
            )
        });
        assert_eq!(actual_outputs, expected_outputs);

        let mut recovery_record = protected_preparation_record();
        recovery_record.ingress_limits.maximum_retained_bytes = GENEROUS_BUDGET_BYTES;
        stage_protected_boot(document.as_bytes(), recovery_record);
        BOOT_STAGING.with(|slot| {
            slot.borrow_mut().options.maximum_memory_bytes = GENEROUS_BUDGET_BYTES;
        });

        let handle = miso_engine_web_v1_boot_with_observation_demand(document.len() as u32);
        assert_ne!(handle, 0, "generous protected boot must recover");
        assert_eq!(miso_engine_web_v1_boot_result(), RESULT_OK);
        assert_eq!(
            SPECTRUM_CAPTURE_CONFIGURE_ENTRIES.with(|entries| entries.get()),
            1,
            "successful protected recovery must configure spectrum once"
        );
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let live = live.as_ref().expect("recovered protected handle");
            let status = live.host.observation_status();
            assert_eq!(status.profile, crate::OBSERVATION_PROFILE_EQ_SPECTRUM);
            assert!(status.owner > 0, "recovered protected owner must be real");
            let ready = live.host.ready.as_ref().expect("recovered ready owner");
            assert!(matches!(
                ready.observation,
                crate::PreparedObservationStorage::Protected(_)
            ));
        });

        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
        no_live_host();
        SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
        SPECTRUM_CAPTURE_CONFIGURE_ENTRIES.with(|entries| entries.set(0));
    }

    #[test]
    fn public_protected_boot_accepts_valid_spectrum_request_capture_budgets() {
        const GENEROUS_BUDGET_BYTES: u64 = 67_108_864;
        let document = one_track_protected_document();

        no_live_host();
        for maximum_capture_bytes in [1_048_576_u64, 2_097_152] {
            SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
            SPECTRUM_CAPTURE_CONFIGURE_ENTRIES.with(|entries| entries.set(0));

            let mut record = protected_preparation_record();
            record.spectrum_request.maximum_capture_bytes = maximum_capture_bytes;
            record.ingress_limits.maximum_retained_bytes = GENEROUS_BUDGET_BYTES;
            stage_protected_boot(document.as_bytes(), record);
            BOOT_STAGING.with(|slot| {
                slot.borrow_mut().options.maximum_memory_bytes = GENEROUS_BUDGET_BYTES;
            });
            test_stage_document(document.as_bytes());

            let handle = miso_engine_web_v1_boot_with_observation_demand(document.len() as u32);
            assert_ne!(handle, 0, "valid protected boot must publish a handle");
            assert_eq!(miso_engine_web_v1_boot_result(), RESULT_OK);
            assert_eq!(
                SPECTRUM_CAPTURE_CONFIGURE_ENTRIES.with(|entries| entries.get()),
                1
            );
            LIVE_HOST.with(|slot| {
                let live = slot.borrow();
                let live = live.as_ref().expect("protected handle");
                let status = live.host.observation_status();
                assert!(status.owner > 0, "protected owner must be real");
                let ready = live.host.ready.as_ref().expect("ready owner");
                assert!(matches!(
                    ready.observation,
                    crate::PreparedObservationStorage::Protected(_)
                ));
            });

            let (capture_len, capture_capacity, result_len, result_capacity) = SPECTRUM_STAGING
                .with(|slot| {
                    let staging = slot.borrow();
                    let capture = staging.capture.as_ref().expect("capture staging");
                    let result = staging.result.as_ref().expect("result staging");
                    (
                        capture.len(),
                        capture.capacity(),
                        result.len(),
                        result.capacity(),
                    )
                });
            assert_eq!(
                (capture_len, capture_capacity, result_len, result_capacity),
                (1_048_576, 1_048_576, 1_048_576, 1_048_576)
            );

            assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
            no_live_host();
            SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
        }
    }

    struct ProtectedBootPreparationCase {
        name: &'static str,
        mutate: fn(&mut WebObservationPreparationRecord),
    }

    #[test]
    fn public_protected_boot_rejects_nested_headers_reserved_and_padded_ids() {
        no_live_host();
        let cases: [ProtectedBootPreparationCase; 6] = [
            ProtectedBootPreparationCase {
                name: "work header",
                mutate: |record: &mut WebObservationPreparationRecord| {
                    record.work_limits.struct_size = 0;
                },
            },
            ProtectedBootPreparationCase {
                name: "ingress header",
                mutate: |record: &mut WebObservationPreparationRecord| {
                    record.ingress_limits.abi_version = ABI_VERSION + 1;
                },
            },
            ProtectedBootPreparationCase {
                name: "request reserved0",
                mutate: |record: &mut WebObservationPreparationRecord| {
                    record.spectrum_request.reserved0 = 1;
                },
            },
            ProtectedBootPreparationCase {
                name: "request reserved",
                mutate: |record: &mut WebObservationPreparationRecord| {
                    record.spectrum_request.reserved[0] = 1;
                },
            },
            ProtectedBootPreparationCase {
                name: "padded target id",
                mutate: |record: &mut WebObservationPreparationRecord| {
                    record.target_id[3] = 1;
                },
            },
            ProtectedBootPreparationCase {
                name: "target id over maximum",
                mutate: |record: &mut WebObservationPreparationRecord| {
                    record.spectrum_request.target_id_bytes = 128;
                },
            },
        ];

        for case in cases {
            let mut record = protected_preparation_record();
            (case.mutate)(&mut record);
            stage_protected_boot(protected_document(), record);
            assert_eq!(
                miso_engine_web_v1_boot_with_observation_demand(protected_document().len() as u32),
                0,
                "{} must refuse without publishing a host",
                case.name
            );
            assert_eq!(
                miso_engine_web_v1_boot_result(),
                RESULT_INVALID_ARGUMENT,
                "{}",
                case.name
            );
            no_live_host();
        }
    }

    #[test]
    fn public_protected_boot_retained_budget_is_inclusive() {
        no_live_host();
        let mut record = protected_preparation_record();
        stage_protected_boot(protected_document(), record);
        let first =
            miso_engine_web_v1_boot_with_observation_demand(protected_document().len() as u32);
        assert_ne!(first, 0, "baseline protected boot");
        let retained = LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let live = live.as_ref().expect("baseline handle");
            let ready = live.host.ready.as_ref().expect("ready owner");
            let crate::PreparedObservationStorage::Protected(storage) = &ready.observation else {
                panic!("protected owner");
            };
            storage.ingress.bounds.retained_bytes
        });
        assert_eq!(miso_engine_web_v1_dispose(first), RESULT_OK);

        record.ingress_limits.maximum_retained_bytes = retained;
        stage_protected_boot(protected_document(), record);
        let exact =
            miso_engine_web_v1_boot_with_observation_demand(protected_document().len() as u32);
        assert_ne!(exact, 0, "exact retained budget must be accepted");
        assert_eq!(miso_engine_web_v1_dispose(exact), RESULT_OK);

        record.ingress_limits.maximum_retained_bytes = retained - 1;
        stage_protected_boot(protected_document(), record);
        assert_eq!(
            miso_engine_web_v1_boot_with_observation_demand(protected_document().len() as u32),
            0,
            "one byte below retained budget must refuse"
        );
        assert_eq!(miso_engine_web_v1_boot_result(), RESULT_REFUSED_BUDGET);
        no_live_host();
    }

    #[test]
    fn observation_staging_retention_adds_actual_refcell_once() {
        let count = MAXIMUM_OBSERVATION_READS as u64;
        let unchanged_heap_sum = count
            * (size_of::<WebObservationSelection>() as u64
                + size_of::<ObservationAddress>() as u64
                + size_of::<ObservationReadValues>() as u64)
            + count * size_of::<WebObservationResult>() as u64
            + RESPONSE_MAXIMUM_EFFECT_ID_BYTES as u64;
        let containing = size_of::<RefCell<ObservationStaging>>() as u64;
        assert_eq!(
            observation_staging_retained_bytes(),
            unchanged_heap_sum + containing,
            "the actual staging container is charged exactly once"
        );
        assert!(observation_staging_largest_allocation_bytes() >= containing);
    }
}

#[cfg(test)]
mod observation_checkpoint_b1_tests {
    use super::*;
    use crate::OBSERVATION_RECEIPT_STATE_PENDING;
    use crate::ffi::observation_checkpoint_a_tests::{
        no_live_host, protected_document, protected_preparation_record, stage_protected_boot,
    };

    fn boot_protected() -> u32 {
        no_live_host();
        let document = protected_document();
        stage_protected_boot(document, protected_preparation_record());
        let handle = boot_staged_observation_demand(document.len() as u32);
        assert_ne!(handle, 0, "protected fixture must boot");
        handle
    }

    fn stage_demand(handle: u32, operation: u32, count: u32, owner: u64) {
        assert_ne!(handle, 0);
        OBSERVATION_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            staging.endpoint.demand = WebObservationDemand {
                struct_size: size_of::<WebObservationDemand>() as u32,
                abi_version: ABI_VERSION,
                operation,
                count,
                owner,
                reserved: [0; 2],
            };
        });
    }

    fn owner(handle: u32) -> u64 {
        LIVE_HOST.with(|slot| {
            slot.borrow()
                .as_ref()
                .filter(|live| live.handle == handle)
                .expect("protected live host")
                .host
                .observation_status()
                .owner
        })
    }

    fn status(handle: u32) -> WebObservationStatus {
        LIVE_HOST.with(|slot| {
            slot.borrow()
                .as_ref()
                .filter(|live| live.handle == handle)
                .expect("protected live host")
                .host
                .observation_status()
        })
    }

    #[test]
    fn additive_demand_stops_native_stream_and_repeats_pending_identity() {
        let handle = boot_protected();
        let expected_owner = owner(handle);

        assert_eq!(
            miso_engine_web_v1_observation_preparation_bytes() as usize,
            size_of::<WebObservationPreparationRecord>()
        );
        assert_eq!(
            miso_engine_web_v1_observation_demand_bytes() as usize,
            size_of::<WebObservationDemand>()
        );
        assert_eq!(miso_engine_web_v1_observation_demand_capacity(), 0);
        assert_eq!(
            miso_engine_web_v1_observation_admission_bytes() as usize,
            size_of::<WebObservationAdmission>()
        );
        assert_eq!(
            miso_engine_web_v1_observation_status_bytes() as usize,
            size_of::<WebObservationStatus>()
        );
        assert_eq!(
            miso_engine_web_v1_observation_capture_identity_bytes() as usize,
            size_of::<WebObservationCaptureIdentity>()
        );

        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        stage_demand(handle, OBSERVATION_OPERATION_STOP_GRAPH, 0, expected_owner);
        assert_eq!(
            miso_engine_web_v1_observation_demand_apply(handle),
            RESULT_OK
        );
        let _ = miso_engine_web_v1_observation_admission_ptr(handle);
        let first = OBSERVATION_STAGING.with(|slot| slot.borrow().endpoint.admission);
        assert_eq!(first.operation, OBSERVATION_OPERATION_STOP_GRAPH);
        assert_eq!(first.receipt.state, OBSERVATION_RECEIPT_STATE_PENDING);
        assert_ne!(first.receipt.sequence, 0);

        // An intervening malformed removal is refused after the removal attempt was spent. The
        // following valid StopGraph must still use the original native pending receipt.
        stage_demand(handle, OBSERVATION_OPERATION_STOP_GRAPH, 0, expected_owner);
        OBSERVATION_STAGING.with(|slot| {
            slot.borrow_mut().endpoint.demand.struct_size = 0;
        });
        assert_eq!(
            miso_engine_web_v1_observation_demand_apply(handle),
            RESULT_BACKPRESSURE
        );

        // The second valid StopGraph is a scalar identity shortcut. It must preserve the native
        // pending receipt and must not copy the intervening refusal or spend another permit/native
        // stop publication.
        stage_demand(handle, OBSERVATION_OPERATION_STOP_GRAPH, 0, expected_owner);
        assert_eq!(
            miso_engine_web_v1_observation_demand_apply(handle),
            RESULT_OK
        );
        let _ = miso_engine_web_v1_observation_admission_ptr(handle);
        let repeated = OBSERVATION_STAGING.with(|slot| slot.borrow().endpoint.admission);
        assert_eq!(repeated.operation, OBSERVATION_OPERATION_STOP_GRAPH);
        assert_eq!(repeated.result, RESULT_OK);
        assert_eq!(repeated.reason, 0);
        assert_eq!(repeated.receipt, first.receipt);
        assert_eq!(
            LIVE_HOST.with(|slot| slot
                .borrow()
                .as_ref()
                .unwrap()
                .host
                .side_records
                .pending_count),
            2
        );

        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
        no_live_host();
    }

    #[test]
    fn known_meter_demands_do_not_spend_credit_but_stop_still_does() {
        let handle = boot_protected();
        let expected_owner = owner(handle);
        let available = crate::OBSERVATION_STATUS_FLAG_ORDINARY_AVAILABLE
            | crate::OBSERVATION_STATUS_FLAG_REMOVAL_AVAILABLE;
        let before = status(handle);

        // Both known meter tags retain header-before-owner precedence, then refuse as unsupported
        // without consuming either protected attempt.
        for operation in [
            crate::OBSERVATION_OPERATION_REPLACE_METERS,
            OBSERVATION_OPERATION_REMOVE_METERS_TO,
        ] {
            stage_demand(
                handle,
                operation,
                0,
                expected_owner.wrapping_add(1),
            );
            OBSERVATION_STAGING.with(|slot| {
                slot.borrow_mut().endpoint.demand.struct_size = 0;
            });
            assert_eq!(
                miso_engine_web_v1_observation_demand_apply(handle),
                RESULT_INVALID_ARGUMENT
            );
            let _ = miso_engine_web_v1_observation_admission_ptr(handle);
            let malformed = OBSERVATION_STAGING.with(|slot| slot.borrow().endpoint.admission);
            assert_eq!(malformed.operation, operation);
            assert_eq!(malformed.result, RESULT_INVALID_ARGUMENT);
            assert_eq!(malformed.reason, 8);
            assert_eq!(malformed.receipt, WebObservationReceipt::default());
            assert_eq!(status(handle), before, "malformed meter {operation}");

            stage_demand(handle, operation, 0, expected_owner.wrapping_add(1));
            assert_eq!(
                miso_engine_web_v1_observation_demand_apply(handle),
                RESULT_INVALID_ARGUMENT
            );
            let _ = miso_engine_web_v1_observation_admission_ptr(handle);
            let wrong_owner = OBSERVATION_STAGING.with(|slot| slot.borrow().endpoint.admission);
            assert_eq!(wrong_owner.operation, operation);
            assert_eq!(wrong_owner.result, RESULT_INVALID_ARGUMENT);
            assert_eq!(wrong_owner.reason, 2);
            assert_eq!(wrong_owner.receipt, WebObservationReceipt::default());
            assert_eq!(status(handle), before, "wrong-owner meter {operation}");

            stage_demand(handle, operation, 0, expected_owner);
            assert_eq!(
                miso_engine_web_v1_observation_demand_apply(handle),
                RESULT_UNSUPPORTED
            );
            let _ = miso_engine_web_v1_observation_admission_ptr(handle);
            let unsupported = OBSERVATION_STAGING.with(|slot| slot.borrow().endpoint.admission);
            assert_eq!(unsupported.operation, operation);
            assert_eq!(unsupported.result, RESULT_UNSUPPORTED);
            assert_eq!(unsupported.reason, 8);
            assert_eq!(unsupported.receipt, WebObservationReceipt::default());
            assert_eq!(status(handle), before, "unsupported meter {operation}");
        }

        // An unknown tag remains on the historical admission path: it is classified as an
        // unsupported demand only after the Ordinary permit is spent.
        stage_demand(handle, u32::MAX, 0, expected_owner);
        assert_eq!(
            miso_engine_web_v1_observation_demand_apply(handle),
            RESULT_UNSUPPORTED
        );
        let _ = miso_engine_web_v1_observation_admission_ptr(handle);
        let unknown = OBSERVATION_STAGING.with(|slot| slot.borrow().endpoint.admission);
        assert_eq!(unknown.operation, u32::MAX);
        assert_eq!(unknown.result, RESULT_UNSUPPORTED);
        assert_eq!(unknown.reason, 8);
        let after_unknown = status(handle);
        assert_eq!(
            after_unknown.flags & crate::OBSERVATION_STATUS_FLAG_ORDINARY_AVAILABLE,
            0,
            "unknown demand tags retain ordinary credit spending"
        );
        assert_eq!(after_unknown.flags & available, before.flags & available & !crate::OBSERVATION_STATUS_FLAG_ORDINARY_AVAILABLE);
        assert_eq!(after_unknown.ingress_epoch, before.ingress_epoch);

        // A malformed genuine StopGraph still spends its removal attempt, preserving protected
        // admission semantics for an actual protected operation.
        stage_demand(handle, OBSERVATION_OPERATION_STOP_GRAPH, 0, expected_owner);
        OBSERVATION_STAGING.with(|slot| {
            slot.borrow_mut().endpoint.demand.struct_size = 0;
        });
        assert_eq!(
            miso_engine_web_v1_observation_demand_apply(handle),
            RESULT_INVALID_ARGUMENT
        );
        let _ = miso_engine_web_v1_observation_admission_ptr(handle);
        let malformed_stop = OBSERVATION_STAGING.with(|slot| slot.borrow().endpoint.admission);
        assert_eq!(malformed_stop.reason, 8);
        assert_eq!(
            status(handle).flags & crate::OBSERVATION_STATUS_FLAG_REMOVAL_AVAILABLE,
            0
        );

        stage_demand(handle, OBSERVATION_OPERATION_STOP_GRAPH, 0, expected_owner);
        assert_eq!(
            miso_engine_web_v1_observation_demand_apply(handle),
            RESULT_BACKPRESSURE
        );

        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
        no_live_host();
    }

    #[test]
    fn wrong_owner_removal_spends_credit_and_read_only_queries_do_not_poll() {
        let handle = boot_protected();
        let expected_owner = owner(handle);

        stage_demand(
            handle,
            OBSERVATION_OPERATION_STOP_GRAPH,
            0,
            expected_owner.wrapping_add(1),
        );
        assert_eq!(
            miso_engine_web_v1_observation_demand_apply(handle),
            RESULT_INVALID_ARGUMENT
        );
        let _ = miso_engine_web_v1_observation_admission_ptr(handle);
        let wrong_owner = OBSERVATION_STAGING.with(|slot| slot.borrow().endpoint.admission);
        assert_eq!(wrong_owner.reason, 2);

        stage_demand(handle, OBSERVATION_OPERATION_STOP_GRAPH, 0, expected_owner);
        assert_eq!(
            miso_engine_web_v1_observation_demand_apply(handle),
            RESULT_BACKPRESSURE
        );
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);

        // Start creates one real native pending receipt. Scalar getters must leave that pending
        // row untouched: they only project fixed scalar records and never reconcile the queue.
        let second = boot_protected();
        let second_owner = owner(second);
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(second, 0.0),
            RESULT_OK
        );
        let pending_before = LIVE_HOST.with(|slot| {
            slot.borrow()
                .as_ref()
                .unwrap()
                .host
                .side_records
                .pending_count
        });
        let _ = miso_engine_web_v1_observation_admission_ptr(second);
        let _ = miso_engine_web_v1_observation_status_ptr(second);
        let _ = miso_engine_web_v1_observation_capture_identity_ptr(second);
        let _ = miso_engine_web_v1_observation_preparation_ptr();
        let _ = miso_engine_web_v1_observation_demand_ptr(second);
        let (pending_after, status) = LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().unwrap().host;
            (host.side_records.pending_count, host.observation_status())
        });
        assert_eq!(pending_after, pending_before);
        assert_eq!(status.pending_count, u32::from(pending_before));
        assert_eq!(status.owner, second_owner);

        assert_eq!(miso_engine_web_v1_dispose(second), RESULT_OK);
        no_live_host();
    }

    #[test]
    fn demand_pointer_requires_live_matching_handle() {
        let handle = boot_protected();
        assert!(!observation_demand_ptr_for_handle(handle).is_null());
        assert!(observation_demand_ptr_for_handle(0).is_null());
        assert!(observation_demand_ptr_for_handle(handle.wrapping_add(1)).is_null());
        assert_eq!(
            miso_engine_web_v1_observation_demand_ptr(handle.wrapping_add(1)),
            0
        );

        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
        no_live_host();
        assert!(observation_demand_ptr_for_handle(handle).is_null());
        assert!(observation_demand_ptr_for_handle(handle.wrapping_add(1)).is_null());
    }

    #[test]
    fn scalar_queries_reject_live_host_borrow_conflicts_before_terminal_fallback() {
        let handle = boot_protected();
        assert!(!observation_admission_ptr_for_handle_raw(handle).is_null());
        assert!(!observation_status_ptr_for_handle_raw(handle).is_null());
        assert!(!observation_capture_identity_ptr_for_handle_raw(handle).is_null());
        assert!(!observation_demand_ptr_for_handle(handle).is_null());
        LIVE_HOST.with(|slot| {
            let _borrow = slot.borrow_mut();
            assert!(observation_admission_ptr_for_handle_raw(handle).is_null());
            assert!(observation_status_ptr_for_handle_raw(handle).is_null());
            assert!(observation_capture_identity_ptr_for_handle_raw(handle).is_null());
            assert!(observation_demand_ptr_for_handle(handle).is_null());
            assert_eq!(observation_admission_ptr_for_handle(handle), 0);
            assert_eq!(observation_status_ptr_for_handle(handle), 0);
            assert_eq!(observation_capture_identity_ptr_for_handle(handle), 0);
            assert_eq!(miso_engine_web_v1_observation_demand_ptr(handle), 0);
        });

        // Once the live slot is actually empty, the scalar getters may serve the matching
        // retained endpoint mirror. An unrelated handle remains invalid.
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
        no_live_host();
        assert!(!observation_admission_ptr_for_handle_raw(handle).is_null());
        assert!(!observation_status_ptr_for_handle_raw(handle).is_null());
        assert!(!observation_capture_identity_ptr_for_handle_raw(handle).is_null());
        assert!(observation_demand_ptr_for_handle(handle).is_null());
        assert!(observation_admission_ptr_for_handle_raw(handle.wrapping_add(1)).is_null());
        assert!(observation_status_ptr_for_handle_raw(handle.wrapping_add(1)).is_null());
        assert!(observation_capture_identity_ptr_for_handle_raw(handle.wrapping_add(1)).is_null());
        assert!(observation_demand_ptr_for_handle(handle.wrapping_add(1)).is_null());
        assert_eq!(
            observation_admission_ptr_for_handle(handle.wrapping_add(1)),
            0
        );
        assert_eq!(observation_status_ptr_for_handle(handle.wrapping_add(1)), 0);
        assert_eq!(
            observation_capture_identity_ptr_for_handle(handle.wrapping_add(1)),
            0
        );
    }
}

#[cfg(test)]
mod observation_checkpoint_c1_tests {
    use super::*;
    use crate::ffi::observation_checkpoint_a_tests::{
        no_live_host, protected_document, protected_preparation_record, stage_protected_boot,
    };
    use crate::{
        COMMAND_RECORD_BYTES, MAXIMUM_COMMAND_RECORDS, PreparedObservationStorage,
        WebCommandReport,
    };

    fn boot_protected() -> u32 {
        no_live_host();
        let document = protected_document();
        stage_protected_boot(document, protected_preparation_record());
        let handle = boot_staged_observation_demand(document.len() as u32);
        assert_ne!(handle, 0, "protected fixture must boot");
        handle
    }

    fn boot_protected_with_console() -> u32 {
        no_live_host();
        let document = protected_document();
        stage_protected_boot(document, protected_preparation_record());
        BOOT_STAGING.with(|slot| {
            slot.borrow_mut().options.console_command_queue_records = 4;
        });
        let handle = boot_staged_observation_demand(document.len() as u32);
        assert_ne!(handle, 0, "protected console fixture must boot");
        handle
    }

    fn dispose(handle: u32) {
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
        no_live_host();
    }

    fn live_status(handle: u32) -> WebObservationStatus {
        LIVE_HOST.with(|slot| {
            slot.borrow()
                .as_ref()
                .filter(|live| live.handle == handle)
                .expect("protected live host")
                .host
                .observation_status()
        })
    }

    fn set_ingress_state(handle: u32, epoch: u64, ordinary_used: bool, removal_used: bool, exhausted: bool) {
        LIVE_HOST.with(|slot| {
            let mut live = slot.borrow_mut();
            let host = &mut live
                .as_mut()
                .filter(|live| live.handle == handle)
                .expect("protected live host")
                .host;
            let ready = host.ready.as_mut().expect("ready ownership");
            let PreparedObservationStorage::Protected(storage) = &mut ready.observation else {
                panic!("protected owner");
            };
            storage.ingress.epoch = epoch;
            storage.ingress.ordinary_used = ordinary_used;
            storage.ingress.removal_used = removal_used;
            storage.ingress.exhausted = exhausted;
        });
    }

    fn stage_spectrum_markers() -> (usize, usize, WebSpectrumStreamMetadata, bool, Option<f64>) {
        SPECTRUM_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            staging.capture_len = 17;
            staging.result_len = 19;
            staging.stream_active = true;
            staging.stream_smoothing = Some(SpectrumSmoothingConfig::new(5.0).unwrap());
            staging.stream_metadata.status = SPECTRUM_STREAM_STATUS_READY;
            staging.stream_metadata.result = RESULT_OK;
            staging.stream_metadata.sequence = 41;
            staging.stream_metadata.dropped_captures = 3;
            (
                staging.capture_len,
                staging.result_len,
                staging.stream_metadata,
                staging.stream_active,
                staging
                    .stream_smoothing
                    .map(SpectrumSmoothingConfig::smoothing_ms),
            )
        })
    }

    fn assert_spectrum_markers(
        markers: (usize, usize, WebSpectrumStreamMetadata, bool, Option<f64>),
    ) {
        SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert_eq!(staging.capture_len, markers.0);
            assert_eq!(staging.result_len, markers.1);
            assert_eq!(staging.stream_metadata, markers.2);
            assert_eq!(staging.stream_active, markers.3);
            assert_eq!(
                staging
                    .stream_smoothing
                    .map(SpectrumSmoothingConfig::smoothing_ms),
                markers.4
            );
        });
    }

    fn stage_resident_markers() -> Vec<WebObservationResult> {
        OBSERVATION_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            staging.addresses.clear();
            staging.results.clear();
            staging.results.extend([
                WebObservationResult {
                    status: OBSERVATION_STATUS_READY,
                    track_index: 96,
                    sequence: 17,
                    ..WebObservationResult::default()
                },
                WebObservationResult {
                    status: OBSERVATION_STATUS_PENDING,
                    track_index: 97,
                    sequence: 19,
                    ..WebObservationResult::default()
                },
            ]);
            staging.results.clone()
        })
    }

    fn assert_resident_markers(markers: &[WebObservationResult]) {
        OBSERVATION_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert_eq!(staging.results.as_slice(), markers);
        });
    }

    #[test]
    fn protected_alias_host_borrow_refusal_is_terminal_before_any_staging_work() {
        let handle = boot_protected();
        let spectrum_markers = stage_spectrum_markers();
        let resident_markers = stage_resident_markers();

        LIVE_HOST.with(|slot| {
            let _borrow = slot.borrow();
            assert_eq!(
                miso_engine_web_v1_spectrum_read(handle, u32::MAX),
                RESULT_INTERNAL
            );
            assert_eq!(miso_engine_web_v1_spectrum_cancel(handle), RESULT_INTERNAL);
            assert_eq!(
                miso_engine_web_v1_spectrum_select(handle, u32::MAX, u32::MAX, u32::MAX),
                RESULT_INTERNAL
            );
            assert_eq!(
                miso_engine_web_v1_spectrum_stream_select(
                    handle,
                    u32::MAX,
                    u32::MAX,
                    u32::MAX,
                    f64::NAN,
                ),
                RESULT_INTERNAL
            );
            assert_eq!(
                miso_engine_web_v1_observation_read(handle, u32::MAX),
                RESULT_INTERNAL
            );
            assert_eq!(miso_engine_web_v1_meter_lease(handle, 2), RESULT_INTERNAL);
        });

        assert_spectrum_markers(spectrum_markers);
        assert_resident_markers(&resident_markers);
        dispose(handle);
    }

    #[test]
    fn invalid_alias_handle_is_terminal_before_any_staging_work() {
        let handle = boot_protected();
        let spectrum_markers = stage_spectrum_markers();
        let resident_markers = stage_resident_markers();
        assert_eq!(
            miso_engine_web_v1_spectrum_read(0, u32::MAX),
            RESULT_INVALID_ARGUMENT
        );
        assert_eq!(
            miso_engine_web_v1_spectrum_cancel(0),
            RESULT_INVALID_ARGUMENT
        );
        assert_eq!(
            miso_engine_web_v1_observation_read(0, u32::MAX),
            RESULT_INVALID_ARGUMENT
        );
        assert_spectrum_markers(spectrum_markers);
        assert_resident_markers(&resident_markers);
        dispose(handle);
    }

    #[test]
    fn protected_unsupported_aliases_guard_before_staging_and_input_validation() {
        let handle = boot_protected();
        let markers = stage_spectrum_markers();
        SPECTRUM_STAGING.with(|slot| {
            let _borrow = slot.borrow_mut();
            assert_eq!(
                miso_engine_web_v1_spectrum_read(handle, u32::MAX),
                RESULT_UNSUPPORTED
            );
        });
        assert_spectrum_markers(markers);
        dispose(handle);

        let handle = boot_protected();
        let markers = stage_spectrum_markers();
        SPECTRUM_STAGING.with(|slot| {
            let _borrow = slot.borrow_mut();
            assert_eq!(
                miso_engine_web_v1_spectrum_cancel(handle),
                RESULT_UNSUPPORTED
            );
        });
        assert_spectrum_markers(markers);
        dispose(handle);

        let handle = boot_protected();
        let markers = stage_spectrum_markers();
        SPECTRUM_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            staging.target_id[0] = 0xff;
            let _borrow = staging;
            assert_eq!(
                miso_engine_web_v1_spectrum_select(handle, u32::MAX, u32::MAX, 1),
                RESULT_UNSUPPORTED
            );
        });
        assert_spectrum_markers(markers);
        dispose(handle);

        let handle = boot_protected();
        let markers = stage_spectrum_markers();
        SPECTRUM_STAGING.with(|slot| {
            let _borrow = slot.borrow_mut();
            assert_eq!(
                miso_engine_web_v1_spectrum_stream_select(
                    handle,
                    u32::MAX,
                    u32::MAX,
                    u32::MAX,
                    f64::NAN,
                ),
                RESULT_UNSUPPORTED
            );
        });
        assert_spectrum_markers(markers);
        dispose(handle);

        let handle = boot_protected();
        OBSERVATION_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            staging.addresses.push(ObservationAddress {
                track_index: 99,
                rack: host_core::EffectRack::Dynamic,
                effect_index: 98,
                tap_id: 97,
                channels: ObservationReadChannels::Both,
            });
            staging.results.push(WebObservationResult {
                status: OBSERVATION_STATUS_READY,
                track_index: 96,
                ..WebObservationResult::default()
            });
            let _borrow = staging;
            assert_eq!(
                miso_engine_web_v1_observation_read(handle, u32::MAX),
                RESULT_UNSUPPORTED
            );
        });
        OBSERVATION_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert_eq!(staging.addresses.len(), 1);
            assert_eq!(staging.results.len(), 1);
            assert_eq!(staging.results[0].track_index, 96);
        });
        dispose(handle);

        let handle = boot_protected();
        let before = LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            (host.meter_lease, *host.status())
        });
        assert_eq!(
            miso_engine_web_v1_meter_lease(handle, 2),
            RESULT_UNSUPPORTED,
            "enabled validation must follow the protected guard"
        );
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            assert_eq!((host.meter_lease, *host.status()), before);
        });
        dispose(handle);
    }

    #[test]
    fn each_unsupported_alias_does_not_spend_its_classified_attempt() {
        let ordinary_calls: [fn(u32) -> u32; 5] = [
            |handle| miso_engine_web_v1_spectrum_read(handle, u32::MAX),
            |handle| miso_engine_web_v1_spectrum_select(handle, u32::MAX, u32::MAX, u32::MAX),
            |handle| {
                miso_engine_web_v1_spectrum_stream_select(
                    handle,
                    u32::MAX,
                    u32::MAX,
                    u32::MAX,
                    f64::NAN,
                )
            },
            |handle| miso_engine_web_v1_observation_read(handle, u32::MAX),
            |handle| miso_engine_web_v1_meter_lease(handle, 2),
        ];
        for (label, (epoch, ordinary_used, removal_used, exhausted)) in [
            ("free", (1, false, false, false)),
            ("spent", (1, true, true, false)),
            ("exhausted", (u64::MAX, true, true, true)),
        ] {
            let handle = boot_protected();
            set_ingress_state(handle, epoch, ordinary_used, removal_used, exhausted);
            let before = protected_ffi_state(handle);
            for &call in &ordinary_calls {
                assert_eq!(call(handle), RESULT_UNSUPPORTED, "{label} ordinary alias");
                assert_eq!(
                    protected_ffi_state(handle),
                    before,
                    "{label} ordinary unsupported alias changed protected state"
                );
                assert_eq!(call(handle), RESULT_UNSUPPORTED, "{label} repeated ordinary alias");
                assert_eq!(
                    protected_ffi_state(handle),
                    before,
                    "{label} repeated ordinary unsupported alias changed protected state"
                );
            }
            assert_eq!(
                miso_engine_web_v1_spectrum_cancel(handle),
                RESULT_UNSUPPORTED,
                "{label} removal alias"
            );
            assert_eq!(
                protected_ffi_state(handle),
                before,
                "{label} removal unsupported alias changed protected state"
            );
            assert_eq!(
                miso_engine_web_v1_meter_lease(handle, 0),
                RESULT_UNSUPPORTED,
                "{label} meter release alias"
            );
            assert_eq!(
                protected_ffi_state(handle),
                before,
                "{label} repeated removal-class alias changed protected state"
            );
            dispose(handle);
        }
    }

    struct WireCommandInput {
        kind: u32,
        rack: u8,
        channel: u8,
        track_index: u32,
        effect_index: u32,
        parameter_id: u32,
        smoothing_samples: u32,
        values: [f32; 4],
    }

    fn wire_command(input: WireCommandInput) -> [u8; COMMAND_RECORD_BYTES as usize] {
        let mut bytes = [0; COMMAND_RECORD_BYTES as usize];
        bytes[0] = input.kind as u8;
        bytes[1] = input.rack;
        bytes[2] = input.channel;
        for (offset, value) in [
            (4, input.track_index),
            (8, input.effect_index),
            (12, input.parameter_id),
            (16, input.smoothing_samples),
        ] {
            bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        }
        for (index, value) in input.values.into_iter().enumerate() {
            let offset = 24 + index * 4;
            bytes[offset..offset + 4].copy_from_slice(&value.to_bits().to_le_bytes());
        }
        bytes
    }

    fn stage_commands(handle: u32, commands: &[[u8; COMMAND_RECORD_BYTES as usize]]) {
        LIVE_HOST.with(|slot| {
            let mut live = slot.borrow_mut();
            let host = &mut live
                .as_mut()
                .filter(|live| live.handle == handle)
                .expect("protected console host")
                .host;
            let bytes = host.command_staging_mut().expect("command staging");
            for (index, command) in commands.iter().enumerate() {
                let start = index * COMMAND_RECORD_BYTES as usize;
                bytes[start..start + command.len()].copy_from_slice(command);
            }
        });
    }

    fn stage_empty_companion(handle: u32) -> u32 {
        const HEADER_BYTES: usize = 24;
        LIVE_HOST.with(|slot| {
            let mut live = slot.borrow_mut();
            let host = &mut live
                .as_mut()
                .filter(|live| live.handle == handle)
                .expect("protected console host")
                .host;
            let generation = host.host_generation;
            let bytes = host.prepared_companion_mut().expect("prepared staging");
            bytes[..HEADER_BYTES].fill(0);
            bytes[0..4].copy_from_slice(&(HEADER_BYTES as u32).to_le_bytes());
            bytes[4..8].copy_from_slice(&ABI_VERSION.to_le_bytes());
            bytes[8..16].copy_from_slice(&generation.to_le_bytes());
            HEADER_BYTES as u32
        })
    }

    #[derive(Debug, PartialEq)]
    struct ControlStateSnapshot {
        producer_success_count: u64,
        producer_available_capacity: usize,
        fader_success_count: u64,
        fader_available_capacity: usize,
        input_success_count: u64,
        input_available_capacity: usize,
    }

    #[derive(Debug, PartialEq)]
    struct InputFilterStateSnapshot {
        committed: [f32; 4],
        candidate: [f32; 4],
        dirty: [bool; 4],
        revision: u64,
    }

    #[derive(Debug, PartialEq)]
    struct CommandStateSnapshot {
        command_wanted: Vec<u32>,
        in_flight: Vec<u32>,
        controls: Vec<ControlStateSnapshot>,
        input_filter_shadows: Vec<InputFilterStateSnapshot>,
        has_in_flight_commands: bool,
    }

    #[derive(Debug, PartialEq)]
    struct HostStagingState {
        command: Option<(usize, u64)>,
        companion: Option<(usize, u64)>,
    }

    #[derive(Debug, PartialEq)]
    struct ProtectedSideState {
        application_len: u64,
        pending_count: u64,
        completed_count: u64,
        reserved_mask: u64,
        receipt_count: u64,
        receipt_fingerprint: u64,
    }

    #[derive(Debug, PartialEq)]
    struct SpectrumStagingState {
        capture: Option<(usize, u64)>,
        capture_len: usize,
        result: Option<(usize, u64)>,
        result_len: usize,
        stream_metadata: WebSpectrumStreamMetadata,
        stream_active: bool,
        stream_smoothing_bits: Option<u64>,
        stream_window: Option<(u64, u64, u64)>,
    }

    #[derive(Debug, PartialEq)]
    struct ObservationStagingState {
        address_count: usize,
        address_fingerprint: u64,
        results: Vec<WebObservationResult>,
    }

    #[derive(Debug, PartialEq)]
    struct ProtectedFfiState {
        status: WebObservationStatus,
        ingress: (u64, bool, bool, bool),
        side: ProtectedSideState,
        capture_identity: WebObservationCaptureIdentity,
        command: CommandStateSnapshot,
        host_staging: HostStagingState,
        spectrum_staging: SpectrumStagingState,
        observation_staging: ObservationStagingState,
    }

    fn fingerprint_mix(hash: u64, value: u64) -> u64 {
        hash ^ value
            .wrapping_add(0x9e37_79b9_7f4a_7c15)
            .wrapping_add(hash << 6)
            .wrapping_add(hash >> 2)
    }

    fn bytes_fingerprint(bytes: &[u8]) -> u64 {
        bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
            fingerprint_mix(hash, u64::from(*byte))
        })
    }

    fn side_state(handle: u32) -> ProtectedSideState {
        LIVE_HOST.with(|slot| {
            let live_slot = slot.borrow();
            let live = live_slot
                .as_ref()
                .filter(|live| live.handle == handle)
                .expect("protected live host");
            let side = &live.host.side_records;
            let receipt_fingerprint = side.receipts.iter().fold(
                0xcbf2_9ce4_8422_2325,
                |hash, receipt| {
                    let hash = fingerprint_mix(hash, receipt.state as u64);
                    let hash = fingerprint_mix(hash, receipt.owner as u64);
                    let hash = fingerprint_mix(hash, receipt.sequence as u64);
                    fingerprint_mix(hash, receipt.application_sample as u64)
                },
            );
            ProtectedSideState {
                application_len: side.application_len as u64,
                pending_count: side.pending_count as u64,
                completed_count: side.completed_count as u64,
                reserved_mask: side.reserved_mask as u64,
                receipt_count: side.receipts.len() as u64,
                receipt_fingerprint,
            }
        })
    }

    fn host_staging_state(handle: u32) -> HostStagingState {
        LIVE_HOST.with(|slot| {
            let mut live = slot.borrow_mut();
            let host = &mut live
                .as_mut()
                .filter(|live| live.handle == handle)
                .expect("protected live host")
                .host;
            let command = host
                .command_staging_mut()
                .map(|bytes| (bytes.len(), bytes_fingerprint(bytes)));
            let companion = host
                .prepared_companion_mut()
                .map(|bytes| (bytes.len(), bytes_fingerprint(bytes)));
            HostStagingState { command, companion }
        })
    }

    fn spectrum_staging_state() -> SpectrumStagingState {
        SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            SpectrumStagingState {
                capture: staging
                    .capture
                    .as_ref()
                    .map(|bytes| (bytes.len(), bytes_fingerprint(bytes))),
                capture_len: staging.capture_len,
                result: staging
                    .result
                    .as_ref()
                    .map(|bytes| (bytes.len(), bytes_fingerprint(bytes))),
                result_len: staging.result_len,
                stream_metadata: staging.stream_metadata,
                stream_active: staging.stream_active,
                stream_smoothing_bits: staging
                    .stream_smoothing
                    .map(|smoothing| smoothing.smoothing_ms().to_bits()),
                stream_window: staging.stream_window.map(|window| {
                    (
                        window.stream_epoch,
                        window.sequence,
                        window.dropped_captures,
                    )
                }),
            }
        })
    }

    fn observation_staging_state() -> ObservationStagingState {
        OBSERVATION_STAGING.with(|slot| {
            let staging = slot.borrow();
            let address_fingerprint = staging.addresses.iter().fold(
                0xcbf2_9ce4_8422_2325,
                |hash, address| {
                    let hash = fingerprint_mix(hash, address.track_index as u64);
                    let hash = fingerprint_mix(hash, address.rack as u64);
                    let hash = fingerprint_mix(hash, address.effect_index as u64);
                    let hash = fingerprint_mix(hash, address.tap_id as u64);
                    fingerprint_mix(hash, address.channels as u64)
                },
            );
            ObservationStagingState {
                address_count: staging.addresses.len(),
                address_fingerprint,
                results: staging.results.clone(),
            }
        })
    }

    fn protected_ffi_state(handle: u32) -> ProtectedFfiState {
        let ingress = LIVE_HOST.with(|slot| {
            let live_slot = slot.borrow();
            let live = live_slot
                .as_ref()
                .filter(|live| live.handle == handle)
                .expect("protected live host");
            let ready = live.host.ready.as_ref().expect("ready ownership");
            let PreparedObservationStorage::Protected(storage) = &ready.observation else {
                panic!("protected owner");
            };
            (
                storage.ingress.epoch,
                storage.ingress.ordinary_used,
                storage.ingress.removal_used,
                storage.ingress.exhausted,
            )
        });
        let capture_identity = LIVE_HOST.with(|slot| {
            *slot
                .borrow()
                .as_ref()
                .filter(|live| live.handle == handle)
                .expect("protected live host")
                .host
                .observation_capture_identity()
        });
        ProtectedFfiState {
            status: live_status(handle),
            ingress,
            side: side_state(handle),
            capture_identity,
            command: command_state(handle),
            host_staging: host_staging_state(handle),
            spectrum_staging: spectrum_staging_state(),
            observation_staging: observation_staging_state(),
        }
    }

    fn command_state(handle: u32) -> CommandStateSnapshot {
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live
                .as_ref()
                .filter(|live| live.handle == handle)
                .expect("protected console host")
                .host;
            let ready = host.ready.as_ref().expect("ready ownership");
            CommandStateSnapshot {
                command_wanted: ready.command_wanted.to_vec(),
                in_flight: ready.in_flight.to_vec(),
                controls: ready
                    .controls
                    .iter()
                    .map(|control| ControlStateSnapshot {
                        producer_success_count: control.producer.success_count(),
                        producer_available_capacity: control.producer.available_capacity(),
                        fader_success_count: control.fader.success_count(),
                        fader_available_capacity: control.fader.available_capacity(),
                        input_success_count: control.input.success_count(),
                        input_available_capacity: control.input.available_capacity(),
                    })
                    .collect(),
                input_filter_shadows: ready
                    .input_filter_shadows
                    .iter()
                    .map(|shadow| InputFilterStateSnapshot {
                        committed: shadow.committed,
                        candidate: shadow.candidate,
                        dirty: shadow.dirty,
                        revision: shadow.revision,
                    })
                    .collect(),
                has_in_flight_commands: ready.has_in_flight_commands,
            }
        })
    }

    fn assert_mixed_batch_refused_before_companion(
        handle: u32,
        prepared: bool,
    ) -> WebCommandReport {
        let audio = wire_command(WireCommandInput {
            kind: crate::COMMAND_PAN,
            rack: u8::MAX,
            channel: u8::MAX,
            track_index: 0,
            effect_index: 0,
            parameter_id: 0,
            smoothing_samples: 0,
            values: [0.0; 4],
        });
        let observation = wire_command(WireCommandInput {
            kind: crate::COMMAND_OBSERVE_SUBSCRIBE,
            rack: 0,
            channel: u8::MAX,
            track_index: 0,
            effect_index: 0,
            parameter_id: 0,
            smoothing_samples: 0,
            values: [0.0; 4],
        });
        stage_commands(handle, &[audio, observation]);
        let before = protected_ffi_state(handle);
        let result = if prepared {
            // The companion span is deliberately malformed; raw observation classification must
            // refuse before it is even inspected.
            miso_engine_web_v1_prepared_command_submit(handle, 2, 1)
        } else {
            miso_engine_web_v1_command_submit(handle, 2)
        };
        assert_eq!(result, RESULT_UNSUPPORTED);
        let after = protected_ffi_state(handle);
        assert_eq!(
            after, before,
            "unsupported raw observation changed protected state"
        );
        let report = LIVE_HOST.with(|slot| {
            *slot
                .borrow()
                .as_ref()
                .filter(|live| live.handle == handle)
                .expect("protected live host")
                .host
                .command_report()
        });
        assert_eq!(report.result, RESULT_UNSUPPORTED);
        assert_eq!(report.reason, crate::COMMAND_REASON_UNSUPPORTED_KIND);
        assert_eq!(report.rejected_index, 1);
        assert_eq!(report.admitted, 0);
        report
    }

    #[test]
    fn both_command_submit_exports_refuse_mixed_observation_batches_before_mutation() {
        let handle = boot_protected_with_console();
        let _plain = assert_mixed_batch_refused_before_companion(handle, false);
        let audio = wire_command(WireCommandInput {
            kind: crate::COMMAND_PAN,
            rack: u8::MAX,
            channel: u8::MAX,
            track_index: 0,
            effect_index: 0,
            parameter_id: 0,
            smoothing_samples: 0,
            values: [0.0; 4],
        });
        stage_commands(handle, &[audio]);
        assert_eq!(
            miso_engine_web_v1_command_submit(handle, 1),
            RESULT_OK,
            "audio-only command remains admissible after unsupported observation refusal"
        );
        dispose(handle);

        let handle = boot_protected_with_console();
        let _prepared = assert_mixed_batch_refused_before_companion(handle, true);
        stage_commands(
            handle,
            &[wire_command(WireCommandInput {
                kind: crate::COMMAND_PAN,
                rack: u8::MAX,
                channel: u8::MAX,
                track_index: 0,
                effect_index: 0,
                parameter_id: 0,
                smoothing_samples: 0,
                values: [0.0; 4],
            })],
        );
        let companion_bytes = stage_empty_companion(handle);
        assert_eq!(
            miso_engine_web_v1_prepared_command_submit(handle, 1, companion_bytes),
            RESULT_OK,
            "prepared audio-only command remains admissible after unsupported observation refusal"
        );
        dispose(handle);
    }

    #[test]
    fn raw_observation_refusal_preserves_compact_state_across_credit_states() {
        for (prepared, route) in [(false, "plain"), (true, "prepared")] {
            for (label, (epoch, ordinary_used, removal_used, exhausted)) in [
                ("free", (1, false, false, false)),
                ("spent", (1, true, true, false)),
                ("exhausted", (u64::MAX, true, true, true)),
            ] {
                let handle = boot_protected_with_console();
                set_ingress_state(handle, epoch, ordinary_used, removal_used, exhausted);
                let _ = assert_mixed_batch_refused_before_companion(handle, prepared);
                let report = LIVE_HOST.with(|slot| {
                    *slot
                        .borrow()
                        .as_ref()
                        .filter(|live| live.handle == handle)
                        .expect("protected live host")
                        .host
                        .command_report()
                });
                assert_eq!(report.rejected_index, 1, "{route} {label}");
                dispose(handle);
            }
        }
    }

    #[test]
    fn raw_observation_refusal_scans_one_last_observe_record_within_bound() {
        for (prepared, route) in [(false, "plain"), (true, "prepared")] {
            let handle = boot_protected_with_console();
            let audio = wire_command(WireCommandInput {
                kind: crate::COMMAND_PAN,
                rack: u8::MAX,
                channel: u8::MAX,
                track_index: 0,
                effect_index: 0,
                parameter_id: 0,
                smoothing_samples: 0,
                values: [0.0; 4],
            });
            let observation = wire_command(WireCommandInput {
                kind: crate::COMMAND_OBSERVE_SUBSCRIBE,
                rack: 0,
                channel: u8::MAX,
                track_index: 0,
                effect_index: 0,
                parameter_id: 0,
                smoothing_samples: 0,
                values: [0.0; 4],
            });
            let mut commands = vec![audio; MAXIMUM_COMMAND_RECORDS as usize];
            let last = MAXIMUM_COMMAND_RECORDS as usize - 1;
            commands[last] = observation;
            stage_commands(handle, &commands);
            let before = protected_ffi_state(handle);
            let result = if prepared {
                // The malformed companion must remain uninspected after the bounded scan finds
                // the last Observe record.
                miso_engine_web_v1_prepared_command_submit(
                    handle,
                    MAXIMUM_COMMAND_RECORDS,
                    1,
                )
            } else {
                miso_engine_web_v1_command_submit(handle, MAXIMUM_COMMAND_RECORDS)
            };
            assert_eq!(result, RESULT_UNSUPPORTED, "{route} last Observe");
            assert_eq!(
                protected_ffi_state(handle),
                before,
                "{route} last-record refusal changed protected state"
            );
            let report = LIVE_HOST.with(|slot| {
                *slot
                    .borrow()
                    .as_ref()
                    .filter(|live| live.handle == handle)
                    .expect("protected live host")
                    .host
                    .command_report()
            });
            assert_eq!(report.rejected_index, MAXIMUM_COMMAND_RECORDS - 1);
            assert_eq!(report.admitted, 0);
            dispose(handle);
        }
    }
}

#[cfg(test)]
mod observation_checkpoint_c2a_tests {
    use super::*;
    use crate::OBSERVATION_RECEIPT_STATE_PENDING;
    use crate::ffi::observation_checkpoint_a_tests::{
        no_live_host, protected_document, protected_preparation_record, stage_protected_boot,
    };

    fn boot_protected() -> u32 {
        no_live_host();
        let document = protected_document();
        stage_protected_boot(document, protected_preparation_record());
        let handle = boot_staged_observation_demand(document.len() as u32);
        assert_ne!(handle, 0, "protected fixture must boot");
        handle
    }

    fn stage_stop_demand(handle: u32) {
        let owner = LIVE_HOST.with(|slot| {
            slot.borrow()
                .as_ref()
                .filter(|live| live.handle == handle)
                .expect("protected live host")
                .host
                .observation_status()
                .owner
        });
        OBSERVATION_STAGING.with(|slot| {
            slot.borrow_mut().endpoint.demand = WebObservationDemand {
                struct_size: size_of::<WebObservationDemand>() as u32,
                abi_version: ABI_VERSION,
                operation: OBSERVATION_OPERATION_STOP_GRAPH,
                count: 0,
                owner,
                reserved: [0; 2],
            };
        });
    }

    fn boot_legacy() -> u32 {
        no_live_host();
        SPECTRUM_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            staging.release_capture();
            let target_id = b"main-out";
            *staging.request = WebSpectrumRequest {
                struct_size: SPECTRUM_REQUEST_BYTES,
                abi_version: ABI_VERSION,
                target: SPECTRUM_TARGET_OUTPUT,
                channels: SPECTRUM_CHANNEL_LEFT,
                target_id_bytes: target_id.len() as u32,
                maximum_capture_bytes: SPECTRUM_CAPTURE_BYTES as u64,
                ..WebSpectrumRequest::default()
            };
            staging.target_id.fill(0);
            staging.target_id[..target_id.len()].copy_from_slice(target_id);
        });
        let document = protected_document();
        let handle = test_boot(document, WebBootOptions::explicit_defaults());
        assert_ne!(handle, 0, "legacy fixture must boot");
        handle
    }

    fn dispose(handle: u32) {
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
        no_live_host();
    }

    #[derive(Clone, Debug, PartialEq)]
    struct CommittedMarkers {
        capture: Vec<u8>,
        capture_len: usize,
        capture_header: WebSpectrumWindow,
        result: Vec<u8>,
        result_len: usize,
        result_header: WebSpectrumResult,
        stream_metadata: WebSpectrumStreamMetadata,
        stream_active: bool,
        stream_cadence: Option<SpectrumCadence>,
        stream_smoothing: Option<SpectrumSmoothingConfig>,
        stream_window: Option<(u64, u64, u64)>,
        capture_identity: WebObservationCaptureIdentity,
    }

    fn stage_markers() -> CommittedMarkers {
        let cadence = SpectrumCadence::new(48_000, 128).expect("fixture cadence");
        let smoothing = SpectrumSmoothingConfig::new(5.0).expect("fixture smoothing");
        let capture_len = usize::try_from(
            SPECTRUM_WINDOW_HEADER_BYTES + 2 * SPECTRUM_WINDOW_FRAMES * size_of::<f32>() as u32,
        )
        .expect("fixture capture length");
        let result_len = usize::try_from(SPECTRUM_RESULT_HEADER_BYTES).expect("result header");
        let capture_header = WebSpectrumWindow {
            struct_size: SPECTRUM_WINDOW_HEADER_BYTES,
            abi_version: ABI_VERSION,
            target: SPECTRUM_TARGET_TRACK_POST_MATRIX,
            channels: SPECTRUM_CHANNEL_BOTH,
            sample_rate_hz: cadence.sample_rate_hz(),
            frames: SPECTRUM_WINDOW_FRAMES,
            source_underrun: 1,
            reserved0: 0,
            captured_sample: 10_240,
            end_sample: 12_288,
            snapshot_token: 44,
            left_offset: SPECTRUM_WINDOW_HEADER_BYTES,
            right_offset: SPECTRUM_WINDOW_HEADER_BYTES
                + SPECTRUM_WINDOW_FRAMES * size_of::<f32>() as u32,
        };
        let result_header = WebSpectrumResult {
            struct_size: SPECTRUM_RESULT_HEADER_BYTES,
            abi_version: ABI_VERSION,
            result: RESULT_OK,
            target: SPECTRUM_TARGET_TRACK_POST_MATRIX,
            channels: SPECTRUM_CHANNEL_BOTH,
            sample_rate_hz: cadence.sample_rate_hz(),
            window_frames: SPECTRUM_WINDOW_FRAMES,
            bin_count: host_core::SPECTRUM_BIN_COUNT as u32,
            source_underrun: 1,
            floor_db: host_core::SPECTRUM_FLOOR_DB,
            captured_sample: capture_header.captured_sample,
            end_sample: capture_header.end_sample,
            snapshot_token: capture_header.snapshot_token,
            result_bytes: SPECTRUM_RESULT_HEADER_BYTES as u64,
            frequencies_offset: 0,
            left_offset: 0,
            right_offset: 0,
            reserved0: 0,
        };

        SPECTRUM_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            staging.configure_capture().expect("fixed spectrum staging");
            let capture = staging.capture.as_mut().expect("capture bytes");
            capture[..capture_len].fill(0xa5);
            let left_offset = capture_header.left_offset as usize;
            let right_offset = capture_header.right_offset as usize;
            for (index, byte) in capture[left_offset..right_offset].iter_mut().enumerate() {
                *byte = (index as u8).wrapping_add(0x11);
            }
            for (index, byte) in capture[right_offset..capture_len].iter_mut().enumerate() {
                *byte = (index as u8).wrapping_add(0x77);
            }
            assert!(copy_live_record(capture, 0, &capture_header));
            let result = staging.result.as_mut().expect("result bytes");
            result[..result_len].fill(0x5a);
            assert!(copy_live_record(result, 0, &result_header));
            staging.capture_len = capture_len;
            staging.result_len = result_len;
            staging.stream_active = true;
            staging.stream_cadence = Some(cadence);
            staging.stream_smoothing = Some(smoothing);
            staging.stream_window = Some(SpectrumStreamWindowFacts {
                stream_epoch: 17,
                sequence: 41,
                dropped_captures: 3,
            });
            staging.stream_metadata = WebSpectrumStreamMetadata {
                struct_size: SPECTRUM_STREAM_METADATA_BYTES,
                abi_version: ABI_VERSION,
                result: RESULT_OK,
                status: SPECTRUM_STREAM_STATUS_READY,
                target: SPECTRUM_TARGET_TRACK_POST_MATRIX,
                channels: SPECTRUM_CHANNEL_BOTH,
                sample_rate_hz: cadence.sample_rate_hz(),
                quantum_frames: cadence.quantum_frames(),
                hop_frames: cadence.hop_frames(),
                source_underrun: 1,
                reserved0: 0,
                reserved1: 0,
                capture_epoch: 17,
                sequence: 41,
                dropped_captures: 3,
                windows: 42,
                captured_sample: capture_header.captured_sample,
                end_sample: capture_header.end_sample,
                analysis_epoch: 9,
                history_start_sample: 8_192,
                smoothing_ms: smoothing.smoothing_ms(),
            };
        });
        let capture_identity = stage_capture_identity();
        SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            CommittedMarkers {
                capture: staging.capture.as_ref().expect("capture bytes")[..staging.capture_len]
                    .to_vec(),
                capture_len: staging.capture_len,
                capture_header,
                result: staging.result.as_ref().expect("result bytes")[..staging.result_len]
                    .to_vec(),
                result_len: staging.result_len,
                result_header,
                stream_metadata: staging.stream_metadata,
                stream_active: staging.stream_active,
                stream_cadence: staging.stream_cadence,
                stream_smoothing: staging.stream_smoothing,
                stream_window: staging.stream_window.map(|window| {
                    (
                        window.stream_epoch,
                        window.sequence,
                        window.dropped_captures,
                    )
                }),
                capture_identity,
            }
        })
    }

    fn assert_staged_markers(markers: &CommittedMarkers) {
        SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert_eq!(staging.capture_len, markers.capture_len);
            assert_eq!(staging.result_len, markers.result_len);
            assert_eq!(staging.stream_metadata, markers.stream_metadata);
            assert_eq!(staging.stream_active, markers.stream_active);
            assert_eq!(staging.stream_cadence, markers.stream_cadence);
            assert_eq!(staging.stream_smoothing, markers.stream_smoothing);
            assert_eq!(
                staging.stream_window.map(|window| {
                    (
                        window.stream_epoch,
                        window.sequence,
                        window.dropped_captures,
                    )
                }),
                markers.stream_window
            );
            let capture = staging.capture.as_ref().expect("capture bytes");
            let capture_prefix = capture.len().min(markers.capture.len());
            assert_eq!(
                &capture[..capture_prefix],
                &markers.capture[..capture_prefix]
            );
            if capture.len() >= size_of::<WebSpectrumWindow>() {
                assert_eq!(
                    read_live_record::<WebSpectrumWindow>(capture, 0)
                        .expect("committed capture header"),
                    markers.capture_header
                );
            }
            let result = staging.result.as_ref().expect("result bytes");
            assert_eq!(&result[..markers.result.len()], markers.result.as_slice());
            assert_eq!(
                read_live_record::<WebSpectrumResult>(result, 0).expect("committed result header"),
                markers.result_header
            );
        });
    }

    fn assert_markers(markers: &CommittedMarkers) {
        assert_staged_markers(markers);
        assert_eq!(capture_identity(), markers.capture_identity);
    }

    fn assert_stopped_output(markers: &CommittedMarkers) {
        SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert_eq!(staging.capture_len, 0);
            assert_eq!(staging.result_len, 0);
            let capture = staging.capture.as_ref().expect("capture bytes");
            let result = staging.result.as_ref().expect("result bytes");
            assert_eq!(&capture[..markers.capture.len()], markers.capture.as_slice());
            assert_eq!(&result[..markers.result.len()], markers.result.as_slice());
        });
        assert_eq!(capture_identity(), markers.capture_identity);
    }

    fn stage_capture_identity() -> WebObservationCaptureIdentity {
        let identity = WebObservationCaptureIdentity {
            struct_size: size_of::<WebObservationCaptureIdentity>() as u32,
            abi_version: ABI_VERSION,
            kind: 2,
            flags: 1,
            owner: 41,
            observation_generation: 42,
            selection_epoch: 43,
            snapshot_token: 44,
        };
        LIVE_HOST.with(|slot| {
            slot.borrow_mut()
                .as_mut()
                .expect("protected live host")
                .host
                .side_records
                .capture_identity = identity;
        });
        identity
    }

    fn capture_identity() -> WebObservationCaptureIdentity {
        LIVE_HOST.with(|slot| {
            *slot
                .borrow()
                .as_ref()
                .expect("protected live host")
                .host
                .observation_capture_identity()
        })
    }

    fn render_one_block(handle: u32) {
        assert_eq!(
            test_copy_staging(handle, BUFFER_SOURCE_ID, b"fixture-source"),
            RESULT_OK
        );
        assert_eq!(test_fill_source_pcm(handle, 0.25), RESULT_OK);
        assert_eq!(
            miso_engine_web_v1_source_submit(handle, 14, 1, 0, 2, 128, 0),
            RESULT_OK
        );
        assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
    }

    fn render_protected_window(handle: u32) {
        assert_eq!(
            test_copy_staging(handle, BUFFER_SOURCE_ID, b"fixture-source"),
            RESULT_OK
        );
        assert_eq!(test_fill_source_pcm(handle, 0.25), RESULT_OK);
        assert_eq!(
            miso_engine_web_v1_source_submit(handle, 14, 1, 0, 2, 128, 0),
            RESULT_OK,
            "protected source block 0"
        );
        assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        assert_eq!(
            miso_engine_web_v1_observation_application_take(handle),
            1,
            "the protected start receipt must apply before capture"
        );
        for block in 1..=16_u64 {
            assert_eq!(
                miso_engine_web_v1_source_submit(handle, 14, 1, block * 128, 2, 128, 0),
                RESULT_OK,
                "protected source block {block}"
            );
            assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        }
    }

    #[test]
    fn consumed_protected_window_arithmetic_faults_preserve_committed_output() {
        SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
        let handle = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        render_protected_window(handle);

        // Obtain one actual admitted native window. The typed read consumes the queue record;
        // only copies of this owned result are faulted below, so no rollback is implied.
        let observed = LIVE_HOST.with(|slot| {
            let mut live = slot.borrow_mut();
            let host = &mut live.as_mut().expect("protected host").host;
            let permit = host
                .begin_observation(
                    ObservationClass::Ordinary,
                    host.protected_spectrum_read_lengths(),
                )
                .expect("ordinary read permit");
            host.read_spectrum_stream_admitted(permit)
                .expect("real admitted native window")
        });
        assert_eq!(observed.window.sequence, 0);
        assert!(observed.window.left.iter().any(|value| *value != 0.0));
        assert!(observed.window.right.iter().any(|value| *value != 0.0));

        for (fault, first_sample, sequence) in [
            ("sequence", observed.window.first_sample, u64::MAX),
            (
                "end sample",
                u64::MAX - u64::from(SPECTRUM_WINDOW_FRAMES) + 1,
                observed.window.sequence,
            ),
        ] {
            let markers = stage_markers();
            let mut copied = observed;
            copied.window.first_sample = first_sample;
            copied.window.sequence = sequence;
            let (result, admission) = LIVE_HOST.with(|slot| {
                let mut live = slot.borrow_mut();
                let host = &mut live.as_mut().expect("protected host").host;
                let owner = host
                    .protected_observation_identity()
                    .expect("same live protected owner")
                    .0
                    .get();
                let target = host.spectrum_target();
                let channels = host.spectrum_channels();
                let cadence = host.protected_spectrum_cadence().expect("prepared cadence");
                let result = SPECTRUM_STAGING.with(|slot| {
                    post_read_protected_spectrum_window(
                        host,
                        &mut slot.borrow_mut(),
                        &copied,
                        owner,
                        target,
                        channels,
                        cadence,
                    )
                });
                (result, *host.observation_admission())
            });
            assert_eq!(result, RESULT_REFUSED_BUDGET, "{fault}");
            assert_eq!(admission.operation, OBSERVATION_OPERATION_READ_SPECTRUM);
            assert_eq!(admission.result, RESULT_REFUSED_BUDGET, "{fault}");
            assert_eq!(
                admission.reason, 9,
                "{fault} must record arithmetic overflow"
            );
            assert_markers(&markers);
        }
        dispose(handle);
    }

    #[test]
    fn protected_stream_read_commits_nonzero_stereo_and_matching_identity() {
        SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
        let handle = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        render_protected_window(handle);

        assert_eq!(miso_engine_web_v1_spectrum_stream_read(handle), RESULT_OK);
        let (header, left, right) = SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            let bytes = staging.capture.as_ref().expect("protected capture");
            let header: WebSpectrumWindow = read_live_record(bytes, 0).expect("raw header");
            let mut left = [0.0_f32; host_core::SPECTRUM_WINDOW_FRAMES];
            let mut right = [0.0_f32; host_core::SPECTRUM_WINDOW_FRAMES];
            spectrum_f32_plane(bytes, header.left_offset, header.frames, &mut left)
                .expect("left plane");
            spectrum_f32_plane(bytes, header.right_offset, header.frames, &mut right)
                .expect("right plane");
            (header, left, right)
        });
        assert_eq!(header.target, SPECTRUM_TARGET_TRACK_POST_MATRIX);
        assert_eq!(header.channels, SPECTRUM_CHANNEL_BOTH);
        assert_eq!(header.frames, SPECTRUM_WINDOW_FRAMES);
        assert_eq!(header.snapshot_token, 1);
        assert_eq!(header.end_sample, 2_048);
        assert_eq!(header.left_offset, SPECTRUM_WINDOW_HEADER_BYTES);
        assert_eq!(
            header.right_offset,
            SPECTRUM_WINDOW_HEADER_BYTES + SPECTRUM_WINDOW_FRAMES * size_of::<f32>() as u32
        );
        assert!(left.iter().any(|value| *value != 0.0));
        assert!(right.iter().any(|value| *value != 0.0));

        let identity = capture_identity();
        assert_eq!(identity.kind, 2);
        assert_eq!(identity.flags, 1);
        assert_eq!(
            identity.owner,
            LIVE_HOST.with(|slot| {
                slot.borrow()
                    .as_ref()
                    .expect("protected host")
                    .host
                    .observation_status()
                    .owner
            })
        );
        assert_ne!(identity.observation_generation, 0);
        assert_ne!(identity.selection_epoch, 0);
        assert_eq!(identity.snapshot_token, header.snapshot_token);
        assert_eq!(
            SPECTRUM_STAGING.with(|slot| slot.borrow().capture_len),
            usize::try_from(header.right_offset).unwrap()
                + host_core::SPECTRUM_WINDOW_FRAMES * size_of::<f32>()
        );
        dispose(handle);
    }

    #[test]
    fn protected_stream_read_reports_generation_seek_failed_and_retains_last_ready_capture() {
        SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
        let handle = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        render_protected_window(handle);
        assert_eq!(miso_engine_web_v1_spectrum_stream_read(handle), RESULT_OK);

        let (ready_capture, ready_header, ready_metadata) = SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            let bytes = staging.capture.as_ref().expect("protected capture");
            let header: WebSpectrumWindow = read_live_record(bytes, 0).expect("ready header");
            let mut left = [0.0_f32; host_core::SPECTRUM_WINDOW_FRAMES];
            let mut right = [0.0_f32; host_core::SPECTRUM_WINDOW_FRAMES];
            spectrum_f32_plane(bytes, header.left_offset, header.frames, &mut left)
                .expect("ready left plane");
            spectrum_f32_plane(bytes, header.right_offset, header.frames, &mut right)
                .expect("ready right plane");
            assert!(left.iter().any(|value| *value != 0.0));
            assert!(right.iter().any(|value| *value != 0.0));
            assert_eq!(staging.stream_metadata.status, SPECTRUM_STREAM_STATUS_READY);
            (
                bytes[..staging.capture_len].to_vec(),
                header,
                staging.stream_metadata,
            )
        });
        let ready_identity = capture_identity();
        assert_eq!(ready_metadata.capture_epoch, 1);
        assert_eq!(ready_metadata.sequence, 0);
        assert_eq!(ready_metadata.windows, 1);
        assert_eq!(ready_header.snapshot_token, 1);
        assert_ne!(ready_identity.owner, 0);
        assert_eq!(ready_identity.snapshot_token, ready_header.snapshot_token);

        assert_eq!(miso_engine_web_v1_spectrum_stream_analysis(), RESULT_OK);
        let analyzed_metadata = SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata);
        SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert_eq!(
                staging
                    .stream_history
                    .as_ref()
                    .expect("analysis history")
                    .history_start_sample(),
                Some(ready_metadata.captured_sample),
                "analysis must populate worker history before the seek"
            );
        });
        assert_eq!(
            analyzed_metadata.history_start_sample,
            ready_metadata.captured_sample
        );

        // Match the native seek fixture: leave a partial window in flight before changing the
        // source generation. The next render crosses that boundary and invalidates the partial
        // window, while the render owner remains usable.
        for block in 17..24_u64 {
            assert_eq!(
                miso_engine_web_v1_source_submit(handle, 14, 1, block * 128, 2, 128, 0),
                RESULT_OK,
                "protected partial source block {block}"
            );
            assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        }
        assert_eq!(
            miso_engine_web_v1_source_seek(handle, 14, 2, 2_048),
            RESULT_OK,
            "generation-tagged seek must be admitted"
        );
        assert_eq!(test_fill_source_pcm(handle, 0.75), RESULT_OK);
        assert_eq!(
            miso_engine_web_v1_source_submit(handle, 14, 2, 2_048, 2, 128, 0),
            RESULT_OK,
            "fresh generation source block"
        );
        assert_eq!(
            miso_engine_web_v1_render(handle, 128),
            RESULT_OK,
            "the seek boundary is reported by the protected stream read"
        );

        assert_eq!(
            miso_engine_web_v1_spectrum_stream_read(handle),
            RESULT_RENDER_REJECTED
        );
        let failed_metadata = SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert_eq!(
                staging.stream_metadata.status,
                SPECTRUM_STREAM_STATUS_FAILED
            );
            assert_eq!(staging.stream_metadata.result, RESULT_RENDER_REJECTED);
            assert_eq!(staging.stream_metadata.capture_epoch, 2);
            assert_eq!(
                staging.stream_metadata.capture_epoch,
                ready_metadata.capture_epoch + 1,
                "the failed read must publish the native stream epoch"
            );
            assert_eq!(staging.stream_metadata.sequence, ready_metadata.sequence);
            assert_eq!(staging.stream_metadata.windows, ready_metadata.windows);
            assert_eq!(
                staging.stream_metadata.captured_sample,
                ready_metadata.captured_sample
            );
            assert_eq!(
                staging.stream_metadata.end_sample,
                ready_metadata.end_sample
            );
            assert_eq!(
                staging.stream_metadata.analysis_epoch,
                analyzed_metadata.analysis_epoch + 1,
                "Failed must advance the populated analysis history epoch"
            );
            assert_eq!(staging.stream_metadata.history_start_sample, 0);
            assert_eq!(
                staging
                    .stream_history
                    .as_ref()
                    .expect("reset analysis history")
                    .history_start_sample(),
                None,
                "Failed must clear a populated worker history"
            );
            assert!(staging.stream_window.is_none());
            assert_eq!(staging.capture_len, 0);
            assert_eq!(staging.result_len, 0);

            // The failed outcome invalidates the published lengths, but must not overwrite the
            // last committed raw bytes. Check the backing bytes directly, including their header
            // and snapshot token, rather than relying on the now-invalid length.
            let bytes = staging.capture.as_ref().expect("protected capture");
            assert_eq!(&bytes[..ready_capture.len()], ready_capture.as_slice());
            assert_eq!(
                read_live_record::<WebSpectrumWindow>(bytes, 0).expect("retained ready header"),
                ready_header
            );
            staging.stream_metadata
        });
        assert_eq!(failed_metadata.status, SPECTRUM_STREAM_STATUS_FAILED);
        assert_eq!(failed_metadata.result, RESULT_RENDER_REJECTED);
        assert_eq!(capture_identity(), ready_identity);
        dispose(handle);
    }

    #[test]
    fn protected_stream_read_reports_real_queue_gap_and_recovers_queued_window() {
        SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
        let handle = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        render_protected_window(handle);
        assert_eq!(miso_engine_web_v1_spectrum_stream_read(handle), RESULT_OK);
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_analysis(),
            RESULT_OK,
            "seed the worker history and committed result before the overrun"
        );

        let (ready_capture, ready_result, ready_header, ready_result_header, ready_metadata) =
            SPECTRUM_STAGING.with(|slot| {
                let staging = slot.borrow();
                let capture = staging.capture.as_ref().expect("protected capture");
                let result = staging.result.as_ref().expect("protected result");
                let capture_header: WebSpectrumWindow =
                    read_live_record(capture, 0).expect("ready capture header");
                let result_header: WebSpectrumResult =
                    read_live_record(result, 0).expect("ready result header");
                assert!(staging.capture_len > SPECTRUM_WINDOW_HEADER_BYTES as usize);
                assert!(staging.result_len > SPECTRUM_RESULT_HEADER_BYTES as usize);
                assert_eq!(staging.stream_metadata.status, SPECTRUM_STREAM_STATUS_READY);
                assert_eq!(staging.stream_metadata.capture_epoch, 1);
                assert_eq!(staging.stream_metadata.sequence, 0);
                assert_eq!(staging.stream_metadata.windows, 1);
                assert_eq!(staging.stream_metadata.analysis_epoch, 0);
                assert_eq!(staging.stream_metadata.history_start_sample, 0);
                assert!(capture_header.snapshot_token != 0);
                assert_eq!(capture_header.snapshot_token, result_header.snapshot_token);
                assert!(
                    capture
                        [capture_header.left_offset as usize..capture_header.right_offset as usize]
                        .iter()
                        .any(|byte| *byte != 0)
                );
                assert!(
                    capture[capture_header.right_offset as usize..staging.capture_len]
                        .iter()
                        .any(|byte| *byte != 0)
                );
                (
                    capture[..staging.capture_len].to_vec(),
                    result[..staging.result_len].to_vec(),
                    capture_header,
                    result_header,
                    staging.stream_metadata,
                )
            });
        let ready_identity = capture_identity();
        assert_ne!(ready_identity.owner, 0);
        assert_ne!(ready_identity.observation_generation, 0);
        assert_ne!(ready_identity.selection_epoch, 0);
        assert_eq!(ready_identity.snapshot_token, ready_header.snapshot_token);

        // The first fixture leaves block 16 as the first partial post-read window. Blocks 17..48
        // complete two windows without a read, overflowing the native one-record queue and
        // recording one dropped capture.
        for block in 17..=48_u64 {
            assert_eq!(
                miso_engine_web_v1_source_submit(handle, 14, 1, block * 128, 2, 128, 0),
                RESULT_OK,
                "protected overrun source block {block}"
            );
            assert_eq!(
                miso_engine_web_v1_render(handle, 128),
                RESULT_OK,
                "protected overrun render block {block}"
            );
        }

        // The admitted native Gap is itself a successful FFI operation. It resets worker history
        // and invalidates published lengths, while committed backing bytes and identity remain
        // authoritative until a later Ready window is actually packed.
        assert_eq!(miso_engine_web_v1_spectrum_stream_read(handle), RESULT_OK);
        let gap_metadata = SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert_eq!(staging.stream_metadata.status, SPECTRUM_STREAM_STATUS_GAP);
            assert_eq!(staging.stream_metadata.result, RESULT_OK);
            assert_eq!(staging.stream_metadata.capture_epoch, 1);
            assert_eq!(staging.stream_metadata.dropped_captures, 1);
            assert_eq!(staging.stream_metadata.sequence, ready_metadata.sequence);
            assert_eq!(staging.stream_metadata.windows, ready_metadata.windows);
            assert_eq!(
                staging.stream_metadata.analysis_epoch,
                ready_metadata.analysis_epoch + 1
            );
            assert_eq!(staging.stream_metadata.history_start_sample, 0);
            assert!(
                staging.stream_history.is_some(),
                "history remains allocated"
            );
            assert_eq!(staging.capture_len, 0);
            assert_eq!(staging.result_len, 0);
            let capture = staging.capture.as_ref().expect("retained capture bytes");
            let result = staging.result.as_ref().expect("retained result bytes");
            assert_eq!(&capture[..ready_capture.len()], ready_capture.as_slice());
            assert_eq!(&result[..ready_result.len()], ready_result.as_slice());
            assert_eq!(
                read_live_record::<WebSpectrumWindow>(capture, 0).expect("retained capture header"),
                ready_header
            );
            assert_eq!(
                read_live_record::<WebSpectrumResult>(result, 0).expect("retained result header"),
                ready_result_header
            );
            staging.stream_metadata
        });
        assert_eq!(gap_metadata.status, SPECTRUM_STREAM_STATUS_GAP);
        assert_eq!(gap_metadata.result, RESULT_OK);
        assert_eq!(gap_metadata.capture_epoch, 1);
        assert_eq!(gap_metadata.dropped_captures, 1);
        assert_eq!(capture_identity(), ready_identity);
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            assert_eq!(
                host.observation_admission().operation,
                OBSERVATION_OPERATION_READ_SPECTRUM
            );
            assert_eq!(host.observation_admission().result, RESULT_OK);
        });

        // Gap consumed the current Ordinary credit. One contiguous render boundary replenishes
        // it while leaving the queued post-gap window available for the next admitted read.
        assert_eq!(
            miso_engine_web_v1_source_submit(handle, 14, 1, 49 * 128, 2, 128, 0),
            RESULT_OK
        );
        assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        assert_eq!(miso_engine_web_v1_spectrum_stream_read(handle), RESULT_OK);
        let (recovered_header, recovered_metadata, recovered_identity) =
            SPECTRUM_STAGING.with(|slot| {
                let staging = slot.borrow();
                let capture = staging.capture.as_ref().expect("recovered capture");
                let header: WebSpectrumWindow =
                    read_live_record(capture, 0).expect("recovered capture header");
                assert_eq!(staging.stream_metadata.status, SPECTRUM_STREAM_STATUS_READY);
                assert_eq!(staging.stream_metadata.result, RESULT_OK);
                assert_eq!(staging.stream_metadata.capture_epoch, 1);
                assert_eq!(staging.stream_metadata.sequence, 1);
                // The queued sequence-1 record was published before sequence 2 overran the
                // one-record queue, so its own captured drop counter remains zero. The admitted
                // Gap above reported the later native cumulative drop count of one.
                assert_eq!(staging.stream_metadata.dropped_captures, 0);
                assert_eq!(staging.stream_metadata.windows, 2);
                assert_eq!(staging.stream_metadata.captured_sample, 2_048);
                assert_eq!(staging.stream_metadata.end_sample, 4_096);
                assert_eq!(header.snapshot_token, 2);
                assert_eq!(header.captured_sample, 2_048);
                assert_eq!(header.end_sample, 4_096);
                assert_eq!(header.snapshot_token, staging.stream_metadata.windows);
                assert_eq!(header.channels, SPECTRUM_CHANNEL_BOTH);
                assert!(staging.capture_len > SPECTRUM_WINDOW_HEADER_BYTES as usize);
                assert!(
                    capture[header.left_offset as usize..header.right_offset as usize]
                        .iter()
                        .any(|byte| *byte != 0)
                );
                assert!(
                    capture[header.right_offset as usize..staging.capture_len]
                        .iter()
                        .any(|byte| *byte != 0)
                );
                (header, staging.stream_metadata, capture_identity())
            });
        assert_eq!(recovered_header.snapshot_token, 2);
        assert_eq!(recovered_metadata.status, SPECTRUM_STREAM_STATUS_READY);
        assert_eq!(recovered_metadata.dropped_captures, 0);
        assert_eq!(recovered_identity.owner, ready_identity.owner);
        assert_eq!(
            recovered_identity.observation_generation,
            ready_identity.observation_generation
        );
        assert_eq!(
            recovered_identity.selection_epoch,
            ready_identity.selection_epoch
        );
        assert_eq!(recovered_identity.snapshot_token, 2);
        dispose(handle);
    }

    #[test]
    fn protected_stream_read_reports_native_pending_states_and_retains_last_ready_capture() {
        SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
        let handle = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        assert_eq!(
            test_copy_staging(handle, BUFFER_SOURCE_ID, b"fixture-source"),
            RESULT_OK
        );
        assert_eq!(test_fill_source_pcm(handle, 0.25), RESULT_OK);

        // The render boundary replenishes Ordinary admission while leaving the accepted graph
        // unapplied. The typed native read must therefore report PendingApplication.
        assert_eq!(
            miso_engine_web_v1_source_submit(handle, 14, 1, 0, 2, 128, 0),
            RESULT_OK
        );
        assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_read(handle),
            RESULT_BACKPRESSURE
        );
        SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert_eq!(
                staging.stream_metadata.status,
                SPECTRUM_STREAM_STATUS_PENDING
            );
            assert_eq!(staging.stream_metadata.result, RESULT_BACKPRESSURE);
        });
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected host").host;
            let admission = host.observation_admission();
            assert_eq!(admission.operation, OBSERVATION_OPERATION_READ_SPECTRUM);
            assert_eq!(admission.result, RESULT_BACKPRESSURE);
            assert_ne!(
                admission.flags & crate::OBSERVATION_ADMISSION_PENDING_BOUNDARY,
                0,
                "PendingApplication must carry the pending-boundary flag"
            );
            let status = host.observation_status();
            assert_ne!(status.accepted_generation, 0);
            assert_eq!(status.applied_generation, 0);
            assert_eq!(status.pending_count, 1);
        });

        assert_eq!(miso_engine_web_v1_observation_application_take(handle), 1);

        // One applied block is still short of a complete window. A second admitted read must
        // expose the native Pending state after the application boundary.
        assert_eq!(
            miso_engine_web_v1_source_submit(handle, 14, 1, 128, 2, 128, 0),
            RESULT_OK
        );
        assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_read(handle),
            RESULT_BACKPRESSURE
        );
        SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert_eq!(
                staging.stream_metadata.status,
                SPECTRUM_STREAM_STATUS_PENDING
            );
            assert_eq!(staging.stream_metadata.result, RESULT_BACKPRESSURE);
        });
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected host").host;
            let admission = host.observation_admission();
            assert_eq!(admission.operation, OBSERVATION_OPERATION_READ_SPECTRUM);
            assert_eq!(admission.result, RESULT_BACKPRESSURE);
            assert_eq!(
                admission.flags & crate::OBSERVATION_ADMISSION_PENDING_BOUNDARY,
                0,
                "an applied Pending read is no longer boundary-pending"
            );
            let status = host.observation_status();
            assert_eq!(status.accepted_generation, status.applied_generation);
            assert_eq!(status.pending_count, 0);
        });

        for block in 2..=16_u64 {
            assert_eq!(
                miso_engine_web_v1_source_submit(handle, 14, 1, block * 128, 2, 128, 0),
                RESULT_OK,
                "protected source block {block}"
            );
            assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        }
        assert_eq!(miso_engine_web_v1_spectrum_stream_read(handle), RESULT_OK);
        let (ready_capture, ready_header, ready_identity) = SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            let bytes = staging.capture.as_ref().expect("protected capture");
            let header: WebSpectrumWindow = read_live_record(bytes, 0).expect("ready header");
            assert_eq!(
                staging.capture_len,
                usize::try_from(header.right_offset).unwrap()
                    + host_core::SPECTRUM_WINDOW_FRAMES * size_of::<f32>()
            );
            (
                bytes[..staging.capture_len].to_vec(),
                header,
                capture_identity(),
            )
        });
        assert_eq!(ready_header.snapshot_token, 1);
        assert_ne!(ready_identity.owner, 0);
        assert_eq!(ready_identity.snapshot_token, ready_header.snapshot_token);

        // Replenish admission without completing another window. The admitted native Pending
        // result changes status only; the previous raw capture/header and identity stay intact.
        assert_eq!(
            miso_engine_web_v1_source_submit(handle, 14, 1, 17 * 128, 2, 128, 0),
            RESULT_OK
        );
        assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_read(handle),
            RESULT_BACKPRESSURE
        );
        SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert_eq!(
                staging.stream_metadata.status,
                SPECTRUM_STREAM_STATUS_PENDING
            );
            assert_eq!(staging.stream_metadata.result, RESULT_BACKPRESSURE);
            let bytes = staging.capture.as_ref().expect("protected capture");
            assert_eq!(&bytes[..ready_capture.len()], ready_capture.as_slice());
            assert_eq!(
                read_live_record::<WebSpectrumWindow>(bytes, 0).expect("retained ready header"),
                ready_header
            );
        });
        assert_eq!(capture_identity(), ready_identity);
        dispose(handle);
    }

    #[test]
    fn protected_stream_read_writes_raw_header_bytes_in_release_safe_path() {
        SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
        let handle = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        render_protected_window(handle);

        assert_eq!(miso_engine_web_v1_spectrum_stream_read(handle), RESULT_OK);
        SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            let bytes = staging.capture.as_ref().expect("protected capture");
            assert!(
                bytes[..SPECTRUM_WINDOW_HEADER_BYTES as usize]
                    .iter()
                    .any(|byte| *byte != 0),
                "the committed header must contain raw bytes in release builds"
            );
            let header: WebSpectrumWindow =
                read_live_record(bytes, 0).expect("committed protected header");
            assert_eq!(header.struct_size, SPECTRUM_WINDOW_HEADER_BYTES);
            assert_eq!(header.abi_version, ABI_VERSION);
            assert_eq!(header.target, SPECTRUM_TARGET_TRACK_POST_MATRIX);
            assert_eq!(header.channels, SPECTRUM_CHANNEL_BOTH);
            assert_eq!(header.frames, SPECTRUM_WINDOW_FRAMES);
            assert_eq!(header.snapshot_token, 1);
        });
        dispose(handle);
    }

    #[test]
    fn protected_stream_read_reports_native_inactive_before_and_after_stop() {
        SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
        let booted = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_read(booted),
            RESULT_WRONG_STATE
        );
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected host").host;
            assert_eq!(host.observation_admission().result, RESULT_WRONG_STATE);
            assert_eq!(host.observation_admission().reason, 0);
        });
        SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert_eq!(
                staging.stream_metadata.status,
                SPECTRUM_STREAM_STATUS_INACTIVE
            );
            assert_eq!(staging.stream_metadata.result, RESULT_WRONG_STATE);
        });
        dispose(booted);

        SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
        let stopped = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(stopped, 0.0),
            RESULT_OK
        );
        render_one_block(stopped);
        assert_eq!(miso_engine_web_v1_observation_application_take(stopped), 1);
        assert_eq!(miso_engine_web_v1_spectrum_stream_stop(stopped), RESULT_OK);
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_read(stopped),
            RESULT_WRONG_STATE
        );
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected host").host;
            assert_eq!(host.observation_admission().result, RESULT_WRONG_STATE);
            assert_eq!(host.observation_admission().reason, 0);
        });
        SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert_eq!(
                staging.stream_metadata.status,
                SPECTRUM_STREAM_STATUS_INACTIVE
            );
            assert_eq!(staging.stream_metadata.result, RESULT_WRONG_STATE);
        });
        dispose(stopped);
    }

    #[test]
    fn protected_stream_read_capacity_refusal_keeps_queue_and_committed_markers() {
        SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
        let handle = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        render_protected_window(handle);
        let markers = stage_markers();
        let identity = stage_capture_identity();
        let required = usize::try_from(
            SPECTRUM_WINDOW_HEADER_BYTES + 2 * SPECTRUM_WINDOW_FRAMES * size_of::<f32>() as u32,
        )
        .unwrap();
        SPECTRUM_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            staging
                .capture
                .as_mut()
                .expect("capture staging")
                .truncate(required - 1);
        });

        assert_eq!(
            miso_engine_web_v1_spectrum_stream_read(handle),
            RESULT_REFUSED_BUDGET
        );
        assert_markers(&markers);
        assert_eq!(capture_identity(), identity);
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected host").host;
            assert_eq!(
                host.observation_admission().operation,
                OBSERVATION_OPERATION_READ_SPECTRUM
            );
            assert_eq!(host.observation_admission().result, RESULT_REFUSED_BUDGET);
            assert_eq!(host.observation_admission().reason, 3);
        });

        // The capacity refusal spends only the Ordinary ingress attempt. A successful render
        // replenishes that credit; the exact-fit retry must still consume the queued sequence 0.
        assert_eq!(
            test_copy_staging(handle, BUFFER_SOURCE_ID, b"fixture-source"),
            RESULT_OK
        );
        assert_eq!(test_fill_source_pcm(handle, 0.25), RESULT_OK);
        assert_eq!(
            miso_engine_web_v1_source_submit(handle, 14, 1, 17 * 128, 2, 128, 0),
            RESULT_OK
        );
        assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        SPECTRUM_STAGING.with(|slot| {
            slot.borrow_mut()
                .capture
                .as_mut()
                .expect("capture staging")
                .resize(required, 0);
        });
        assert_eq!(miso_engine_web_v1_spectrum_stream_read(handle), RESULT_OK);
        let header: WebSpectrumWindow = SPECTRUM_STAGING
            .with(|slot| read_live_record(slot.borrow().capture.as_ref().unwrap(), 0).unwrap());
        assert_eq!(header.snapshot_token, 1);
        assert_eq!(header.captured_sample, 0);
        dispose(handle);
    }

    #[test]
    fn protected_stream_read_borrow_conflicts_preserve_complete_committed_markers() {
        SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
        let handle = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        render_protected_window(handle);
        let markers = stage_markers();

        LIVE_HOST.with(|slot| {
            let _borrow = slot.borrow();
            assert_eq!(
                miso_engine_web_v1_spectrum_stream_read(handle),
                RESULT_INTERNAL
            );
        });
        assert_staged_markers(&markers);
        assert_eq!(capture_identity(), markers.capture_identity);

        LIVE_HOST.with(|slot| {
            let _borrow = slot.borrow_mut();
            assert_eq!(
                miso_engine_web_v1_spectrum_stream_read(handle),
                RESULT_INTERNAL
            );
        });
        assert_staged_markers(&markers);
        assert_eq!(capture_identity(), markers.capture_identity);
        dispose(handle);
    }

    #[test]
    fn protected_stream_read_zero_unrelated_and_disposed_handles_preserve_complete_markers() {
        SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
        let handle = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        render_protected_window(handle);
        let markers = stage_markers();

        assert_eq!(
            miso_engine_web_v1_spectrum_stream_read(0),
            RESULT_INVALID_ARGUMENT
        );
        assert_markers(&markers);
        let unrelated = handle.wrapping_add(1).max(1);
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_read(unrelated),
            RESULT_INVALID_ARGUMENT
        );
        assert_markers(&markers);

        // Disposal clears the shared stream staging, so recreate a live protected owner and use
        // the old handle as the disposed-handle refusal target.
        dispose(handle);
        let replacement = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(replacement, 0.0),
            RESULT_OK
        );
        render_protected_window(replacement);
        let replacement_markers = stage_markers();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_read(handle),
            RESULT_INVALID_ARGUMENT
        );
        assert_markers(&replacement_markers);
        dispose(replacement);
    }

    #[test]
    fn protected_stream_read_header_short_capacity_refusal_preserves_complete_markers() {
        SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
        let handle = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        render_protected_window(handle);
        let markers = stage_markers();
        let header_bytes = usize::try_from(SPECTRUM_WINDOW_HEADER_BYTES).unwrap();
        SPECTRUM_STAGING.with(|slot| {
            slot.borrow_mut()
                .capture
                .as_mut()
                .expect("capture staging")
                .truncate(header_bytes - 1);
        });

        assert_eq!(
            miso_engine_web_v1_spectrum_stream_read(handle),
            RESULT_REFUSED_BUDGET
        );
        assert_markers(&markers);
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected host").host;
            assert_eq!(host.observation_admission().result, RESULT_REFUSED_BUDGET);
            assert_eq!(host.observation_admission().reason, 3);
        });
        dispose(handle);
    }

    #[test]
    fn protected_stream_read_left_plane_only_capacity_refusal_preserves_complete_markers() {
        SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
        let handle = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        render_protected_window(handle);
        let markers = stage_markers();
        let left_only = usize::try_from(
            SPECTRUM_WINDOW_HEADER_BYTES + SPECTRUM_WINDOW_FRAMES * size_of::<f32>() as u32,
        )
        .unwrap();
        SPECTRUM_STAGING.with(|slot| {
            slot.borrow_mut()
                .capture
                .as_mut()
                .expect("capture staging")
                .truncate(left_only);
        });

        assert_eq!(
            miso_engine_web_v1_spectrum_stream_read(handle),
            RESULT_REFUSED_BUDGET
        );
        assert_markers(&markers);
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected host").host;
            assert_eq!(host.observation_admission().result, RESULT_REFUSED_BUDGET);
            assert_eq!(host.observation_admission().reason, 3);
        });
        dispose(handle);
    }

    #[test]
    fn protected_stream_read_borrow_and_spent_credit_refusals_keep_queued_window() {
        SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
        let handle = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        render_protected_window(handle);
        let markers = stage_markers();
        let identity = stage_capture_identity();

        SPECTRUM_STAGING.with(|slot| {
            let _borrow = slot.borrow_mut();
            assert_eq!(
                miso_engine_web_v1_spectrum_stream_read(handle),
                RESULT_INTERNAL
            );
        });
        assert_markers(&markers);
        assert_eq!(capture_identity(), identity);

        // The staging-borrow refusal already spends Ordinary credit. Unsupported selection must
        // leave that spent state alone; the queued native window remains available for the next
        // admitted read after a successful render boundary.
        let spent_after_borrow = LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            let status = host.observation_status();
            assert_eq!(
                status.flags & crate::OBSERVATION_STATUS_FLAG_ORDINARY_AVAILABLE,
                0,
                "a staging-borrow refusal spends the Ordinary attempt"
            );
            (status.ingress_epoch, status.flags)
        });
        assert_eq!(
            miso_engine_web_v1_spectrum_select(handle, 0, 0, 0),
            RESULT_UNSUPPORTED
        );
        let after_selection = LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            let status = host.observation_status();
            (status.ingress_epoch, status.flags)
        });
        assert_eq!(after_selection, spent_after_borrow);
        assert_markers(&markers);
        assert_eq!(capture_identity(), identity);
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_read(handle),
            RESULT_BACKPRESSURE
        );
        assert_markers(&markers);
        assert_eq!(capture_identity(), identity);

        assert_eq!(
            test_copy_staging(handle, BUFFER_SOURCE_ID, b"fixture-source"),
            RESULT_OK
        );
        assert_eq!(test_fill_source_pcm(handle, 0.25), RESULT_OK);
        assert_eq!(
            miso_engine_web_v1_source_submit(handle, 14, 1, 17 * 128, 2, 128, 0),
            RESULT_OK
        );
        assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        assert_eq!(miso_engine_web_v1_spectrum_stream_read(handle), RESULT_OK);
        let header: WebSpectrumWindow = SPECTRUM_STAGING
            .with(|slot| read_live_record(slot.borrow().capture.as_ref().unwrap(), 0).unwrap());
        assert_eq!(header.snapshot_token, 1);
        dispose(handle);
    }

    #[test]
    fn protected_start_refusals_spend_ordinary_credit_without_publication_or_staging_mutation() {
        let handle = boot_protected();
        let markers = stage_markers();
        let identity = stage_capture_identity();

        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, f64::NAN),
            RESULT_INVALID_ARGUMENT
        );
        assert_markers(&markers);
        assert_eq!(capture_identity(), identity);
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            let status = host.observation_status();
            assert_eq!(status.accepted_generation, 0);
            assert_eq!(status.pending_count, 0);
            assert_eq!(
                status.flags & crate::OBSERVATION_STATUS_FLAG_ORDINARY_AVAILABLE,
                0,
                "invalid smoothing spends the ordinary attempt"
            );
            assert_eq!(host.observation_admission().operation, 4);
            assert_eq!(host.observation_admission().result, RESULT_INVALID_ARGUMENT);
        });
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_BACKPRESSURE,
            "a valid retry cannot publish after the invalid smoothing attempt"
        );
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            assert_eq!(host.observation_status().accepted_generation, 0);
            assert_eq!(host.observation_status().pending_count, 0);
        });
        assert_markers(&markers);
        assert_eq!(capture_identity(), identity);
        dispose(handle);
    }

    #[test]
    fn protected_start_history_exhaustion_refuses_before_native_publication() {
        let handle = boot_protected();
        let markers = stage_markers();
        let identity = stage_capture_identity();
        SPECTRUM_STAGING.with(|slot| {
            slot.borrow_mut().stream_history_exhausted_for_test = true;
        });

        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_REFUSED_BUDGET
        );
        assert_markers(&markers);
        assert_eq!(capture_identity(), identity);
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            let status = host.observation_status();
            assert_eq!(status.accepted_generation, 0);
            assert_eq!(status.pending_count, 0);
            assert_eq!(
                status.flags & crate::OBSERVATION_STATUS_FLAG_ORDINARY_AVAILABLE,
                0,
                "history exhaustion spends the ordinary attempt"
            );
            assert_eq!(host.observation_admission().operation, 4);
            assert_eq!(host.observation_admission().result, RESULT_REFUSED_BUDGET);
        });
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_BACKPRESSURE,
            "a valid retry cannot publish after history exhaustion"
        );
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            assert_eq!(host.observation_status().accepted_generation, 0);
            assert_eq!(host.observation_status().pending_count, 0);
        });
        assert_markers(&markers);
        assert_eq!(capture_identity(), identity);
        dispose(handle);
    }

    #[test]
    fn protected_stream_dispatch_refusals_preserve_markers_for_host_borrow_and_invalid_handles() {
        let handle = boot_protected();
        let markers = stage_markers();
        let identity = stage_capture_identity();

        LIVE_HOST.with(|slot| {
            let _borrow = slot.borrow();
            assert_eq!(
                miso_engine_web_v1_spectrum_stream_start(handle, 12.5),
                RESULT_INTERNAL
            );
            assert_eq!(
                miso_engine_web_v1_spectrum_stream_stop(handle),
                RESULT_INTERNAL
            );
        });
        assert_markers(&markers);
        assert_eq!(capture_identity(), identity);

        LIVE_HOST.with(|slot| {
            let _borrow = slot.borrow_mut();
            assert_eq!(
                miso_engine_web_v1_spectrum_stream_start(handle, 12.5),
                RESULT_INTERNAL
            );
            assert_eq!(
                miso_engine_web_v1_spectrum_stream_stop(handle),
                RESULT_INTERNAL
            );
        });
        assert_markers(&markers);
        assert_eq!(capture_identity(), identity);

        let invalid_handle = handle.wrapping_add(1).max(1);
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(invalid_handle, 12.5),
            RESULT_INVALID_ARGUMENT
        );
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_stop(invalid_handle),
            RESULT_INVALID_ARGUMENT
        );
        assert_markers(&markers);
        assert_eq!(capture_identity(), identity);

        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(0, 12.5),
            RESULT_INVALID_ARGUMENT
        );
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_stop(0),
            RESULT_INVALID_ARGUMENT
        );
        assert_markers(&markers);
        assert_eq!(capture_identity(), identity);
        dispose(handle);
    }

    #[test]
    fn legacy_stream_dispatch_refusals_preserve_markers_for_both_host_borrow_kinds() {
        let handle = boot_legacy();
        let markers = stage_markers();
        let identity = stage_capture_identity();

        LIVE_HOST.with(|slot| {
            let _borrow = slot.borrow();
            assert_eq!(
                miso_engine_web_v1_spectrum_stream_start(handle, 12.5),
                RESULT_INTERNAL
            );
            assert_eq!(
                miso_engine_web_v1_spectrum_stream_stop(handle),
                RESULT_INTERNAL
            );
        });
        assert_markers(&markers);
        assert_eq!(capture_identity(), identity);

        LIVE_HOST.with(|slot| {
            let _borrow = slot.borrow_mut();
            assert_eq!(
                miso_engine_web_v1_spectrum_stream_start(handle, 12.5),
                RESULT_INTERNAL
            );
            assert_eq!(
                miso_engine_web_v1_spectrum_stream_stop(handle),
                RESULT_INTERNAL
            );
        });
        assert_markers(&markers);
        assert_eq!(capture_identity(), identity);

        let invalid_handle = handle.wrapping_add(1).max(1);
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(invalid_handle, 12.5),
            RESULT_INVALID_ARGUMENT
        );
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_stop(invalid_handle),
            RESULT_INVALID_ARGUMENT
        );
        assert_markers(&markers);
        assert_eq!(capture_identity(), identity);
        dispose(handle);
    }

    #[test]
    fn protected_start_publishes_one_pending_receipt_and_commits_configuration() {
        let handle = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 12.5),
            RESULT_OK
        );
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            let status = host.observation_status();
            let admission = host.observation_admission();
            assert_ne!(status.accepted_generation, 0);
            assert_eq!(status.pending_count, 1);
            assert_eq!(admission.operation, 4);
            assert_eq!(admission.result, RESULT_OK);
            assert_ne!(admission.flags & crate::OBSERVATION_ADMISSION_RECEIPT, 0);
            assert_ne!(
                admission.flags & crate::OBSERVATION_ADMISSION_PENDING_BOUNDARY,
                0
            );
            assert_eq!(admission.receipt.state, OBSERVATION_RECEIPT_STATE_PENDING);
        });
        SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert!(staging.stream_active);
            assert_eq!(staging.stream_smoothing.unwrap().smoothing_ms(), 12.5);
            assert_eq!(
                staging.stream_metadata.status,
                SPECTRUM_STREAM_STATUS_WARMING
            );
            assert_eq!(staging.capture_len, 0);
            assert_eq!(staging.result_len, 0);
        });
        dispose(handle);
    }

    #[test]
    fn protected_stop_preserves_refused_output_then_reuses_authoritative_pending_receipt() {
        let handle = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        render_one_block(handle);
        assert_eq!(miso_engine_web_v1_observation_application_take(handle), 1);
        let markers = stage_markers();
        let identity = stage_capture_identity();

        // A new ordinary attempt is refused, but removal credit remains independent.
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, f64::NAN),
            RESULT_INVALID_ARGUMENT
        );
        assert_markers(&markers);
        assert_eq!(capture_identity(), identity);

        assert_eq!(miso_engine_web_v1_spectrum_stream_stop(handle), RESULT_OK);
        let first_receipt = LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            assert_eq!(host.side_records.pending_count, 1);
            assert_eq!(host.observation_admission().result, RESULT_OK);
            assert_eq!(
                host.observation_admission().receipt.state,
                OBSERVATION_RECEIPT_STATE_PENDING
            );
            host.observation_admission().receipt
        });
        assert_eq!(SPECTRUM_STAGING.with(|slot| slot.borrow().capture_len), 0);
        assert_eq!(SPECTRUM_STAGING.with(|slot| slot.borrow().result_len), 0);
        assert_eq!(capture_identity(), identity);
        SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert_eq!(
                &staging.capture.as_ref().expect("capture bytes")[..markers.capture.len()],
                markers.capture.as_slice()
            );
            assert_eq!(
                &staging.result.as_ref().expect("result bytes")[..markers.result.len()],
                markers.result.as_slice()
            );
        });

        // Unsupported cancel must not spend Removal or disturb the Pending row.
        assert_eq!(
            miso_engine_web_v1_spectrum_cancel(handle),
            RESULT_UNSUPPORTED
        );
        assert_stopped_output(&markers);

        // A malformed genuine StopGraph is refused against the already-spent Removal credit. The
        // valid repeated stop must return the original native Pending identity.
        stage_stop_demand(handle);
        OBSERVATION_STAGING.with(|slot| {
            slot.borrow_mut().endpoint.demand.struct_size = 0;
        });
        assert_eq!(
            miso_engine_web_v1_observation_demand_apply(handle),
            RESULT_BACKPRESSURE
        );
        let _ = miso_engine_web_v1_observation_admission_ptr(handle);
        let malformed = OBSERVATION_STAGING.with(|slot| slot.borrow().endpoint.admission);
        assert_eq!(malformed.operation, OBSERVATION_OPERATION_STOP_GRAPH);
        assert_eq!(malformed.result, RESULT_BACKPRESSURE);
        assert_eq!(malformed.reason, 5);
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            assert_eq!(host.side_records.pending_count, 1);
            assert!(
                host.side_records
                    .receipts
                    .iter()
                    .any(|receipt| *receipt == first_receipt),
                "the authoritative Pending receipt survives the intervening refusal"
            );
        });
        assert_stopped_output(&markers);

        assert_eq!(miso_engine_web_v1_spectrum_stream_stop(handle), RESULT_OK);
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            assert_eq!(host.observation_admission().result, RESULT_OK);
            assert_eq!(host.observation_admission().receipt, first_receipt);
            assert_eq!(host.observation_admission().receipt.owner, first_receipt.owner);
            assert_eq!(
                host.observation_admission().receipt.sequence,
                first_receipt.sequence
            );
            assert_eq!(host.side_records.pending_count, 1);
        });
        assert_eq!(capture_identity(), identity);
        dispose(handle);
    }

    #[test]
    fn protected_removal_stop_remains_admissible_after_spent_ordinary_read_credit() {
        let handle = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        render_protected_window(handle);
        let markers = stage_markers();
        let required = usize::try_from(
            SPECTRUM_WINDOW_HEADER_BYTES + 2 * SPECTRUM_WINDOW_FRAMES * size_of::<f32>() as u32,
        )
        .unwrap();
        SPECTRUM_STAGING.with(|slot| {
            slot.borrow_mut()
                .capture
                .as_mut()
                .expect("capture staging")
                .truncate(required - 1);
        });

        assert_eq!(
            miso_engine_web_v1_spectrum_stream_read(handle),
            RESULT_REFUSED_BUDGET
        );
        assert_markers(&markers);
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_stop(handle),
            RESULT_OK,
            "Removal must remain available after an Ordinary read refusal"
        );
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected host").host;
            assert_eq!(
                host.observation_admission().operation,
                OBSERVATION_OPERATION_STOP
            );
            assert_eq!(host.observation_admission().result, RESULT_OK);
            assert_eq!(
                host.observation_admission().receipt.state,
                OBSERVATION_RECEIPT_STATE_PENDING
            );
            assert_eq!(host.side_records.pending_count, 1);
        });
        SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert!(!staging.stream_active);
            assert_eq!(
                staging.stream_metadata.status,
                SPECTRUM_STREAM_STATUS_STOPPED
            );
            assert_eq!(staging.capture_len, 0);
            assert_eq!(staging.result_len, 0);
        });
        assert_eq!(capture_identity(), markers.capture_identity);
        dispose(handle);
    }

    #[test]
    fn protected_stop_refusal_preserves_all_committed_output_markers() {
        let handle = boot_protected();
        let markers = stage_markers();
        let identity = stage_capture_identity();
        let before = LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            (
                host.side_records.application_len,
                host.side_records.pending_count,
                host.side_records.completed_count,
                host.side_records.reserved_mask,
                host.side_records.receipts,
                *host.observation_capture_identity(),
            )
        });
        assert_eq!(
            miso_engine_web_v1_spectrum_cancel(handle),
            RESULT_UNSUPPORTED
        );

        // A matching-owner malformed genuine StopGraph spends Removal but cannot publish a row.
        stage_stop_demand(handle);
        OBSERVATION_STAGING.with(|slot| {
            slot.borrow_mut().endpoint.demand.struct_size = 0;
        });
        assert_eq!(
            miso_engine_web_v1_observation_demand_apply(handle),
            RESULT_INVALID_ARGUMENT
        );
        let _ = miso_engine_web_v1_observation_admission_ptr(handle);
        let malformed = OBSERVATION_STAGING.with(|slot| slot.borrow().endpoint.admission);
        assert_eq!(malformed.operation, OBSERVATION_OPERATION_STOP_GRAPH);
        assert_eq!(malformed.result, RESULT_INVALID_ARGUMENT);
        assert_eq!(malformed.reason, 8);
        let after_malformed = LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            (
                host.side_records.application_len,
                host.side_records.pending_count,
                host.side_records.completed_count,
                host.side_records.reserved_mask,
                host.side_records.receipts,
                *host.observation_capture_identity(),
            )
        });
        assert_eq!(after_malformed, before);
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            assert_eq!(host.observation_status().accepted_generation, 0);
            assert_eq!(host.observation_status().applied_generation, 0);
            assert_eq!(host.observation_status().pending_count, 0);
            assert_eq!(
                host.observation_status().flags
                    & crate::OBSERVATION_STATUS_FLAG_REMOVAL_AVAILABLE,
                0
            );
        });
        assert_markers(&markers);
        assert_eq!(capture_identity(), identity);

        assert_eq!(
            miso_engine_web_v1_spectrum_stream_stop(handle),
            RESULT_BACKPRESSURE
        );
        assert_markers(&markers);
        assert_eq!(capture_identity(), identity);
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            assert_eq!(host.observation_status().accepted_generation, 0);
            assert_eq!(host.side_records.pending_count, 0);
        });
        let after_stop = LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            (
                host.side_records.application_len,
                host.side_records.pending_count,
                host.side_records.completed_count,
                host.side_records.reserved_mask,
                host.side_records.receipts,
                *host.observation_capture_identity(),
            )
        });
        assert_eq!(after_stop, before);
        dispose(handle);
    }
}

#[cfg(test)]
mod observation_checkpoint_b2_tests {
    use super::*;
    use crate::ffi::observation_checkpoint_a_tests::{
        no_live_host, protected_document, protected_preparation_record, stage_protected_boot,
    };
    use crate::{
        OBSERVATION_RECEIPT_STATE_APPLIED, OBSERVATION_RECEIPT_STATE_CLOSED, STATE_FAILED,
    };
    use builtins::Matrix2x2;
    use builtins_compiler::TrackControlRecord;

    fn boot_protected() -> u32 {
        no_live_host();
        let document = protected_document();
        stage_protected_boot(document, protected_preparation_record());
        BOOT_STAGING.with(|slot| {
            slot.borrow_mut().options.console_command_queue_records = 4;
        });
        let handle = miso_engine_web_v1_boot_with_observation_demand(document.len() as u32);
        assert_ne!(handle, 0, "protected fixture must boot");
        handle
    }

    fn owner(handle: u32) -> u64 {
        LIVE_HOST.with(|slot| {
            slot.borrow()
                .as_ref()
                .filter(|live| live.handle == handle)
                .expect("protected live host")
                .host
                .observation_status()
                .owner
        })
    }

    fn stage_stop(handle: u32) {
        let owner = owner(handle);
        OBSERVATION_STAGING.with(|slot| {
            slot.borrow_mut().endpoint.demand = WebObservationDemand {
                struct_size: size_of::<WebObservationDemand>() as u32,
                abi_version: ABI_VERSION,
                operation: OBSERVATION_OPERATION_STOP_GRAPH,
                count: 0,
                owner,
                reserved: [0; 2],
            };
        });
    }

    fn inject_nan_matrix(handle: u32) {
        LIVE_HOST.with(|slot| {
            let mut slot = slot.borrow_mut();
            let live = slot
                .as_mut()
                .filter(|live| live.handle == handle)
                .expect("protected live host");
            live.host.ready.as_mut().expect("ready ownership").controls[0]
                .producer
                .try_push(TrackControlRecord {
                    matrix: Matrix2x2 {
                        ll: f32::NAN,
                        ..Matrix2x2::IDENTITY
                    },
                    smoothing_samples: 0,
                })
                .expect("documented private malformed matrix fixture");
        });
    }

    fn render_protected_spectrum_window(handle: u32) {
        assert_eq!(
            test_copy_staging(handle, BUFFER_SOURCE_ID, b"fixture-source"),
            RESULT_OK
        );
        assert_eq!(test_fill_source_pcm(handle, 0.25), RESULT_OK);
        assert_eq!(
            miso_engine_web_v1_source_submit(handle, 14, 1, 0, 2, 128, 0),
            RESULT_OK,
            "protected source block 0"
        );
        assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        assert_eq!(
            miso_engine_web_v1_observation_application_take(handle),
            1,
            "the first real render must apply the protected start receipt"
        );
        for block in 1..=16_u64 {
            assert_eq!(
                miso_engine_web_v1_source_submit(handle, 14, 1, block * 128, 2, 128, 0),
                RESULT_OK,
                "protected source block {block}"
            );
            assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        }
        let read_result = miso_engine_web_v1_spectrum_stream_read(handle);
        if read_result != RESULT_OK {
            let (status, admission) = LIVE_HOST.with(|slot| {
                let live = slot.borrow();
                let host = &live.as_ref().expect("protected live host").host;
                (host.observation_status(), *host.observation_admission())
            });
            panic!(
                "the protected read fixture must publish one real window: result={read_result}, status={status:?}, admission={admission:?}"
            );
        }
        let metadata = SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata);
        assert_eq!(metadata.status, SPECTRUM_STREAM_STATUS_READY);
        assert_eq!(metadata.capture_epoch, 1);
        assert_eq!(metadata.sequence, 0);
        assert_eq!(
            metadata.end_sample - metadata.captured_sample,
            2_048,
            "protected read fixture must contain one complete window"
        );
    }

    #[test]
    fn application_take_transfers_real_same_boundary_rows_once() {
        let handle = boot_protected();
        assert_eq!(
            miso_engine_web_v1_observation_application_bytes() as usize,
            size_of::<WebObservationReceipt>()
        );
        assert_eq!(miso_engine_web_v1_observation_application_capacity(), 4);
        assert_eq!(
            miso_engine_web_v1_observation_application_take(handle.wrapping_add(1)),
            0
        );

        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        stage_stop(handle);
        assert_eq!(
            miso_engine_web_v1_observation_demand_apply(handle),
            RESULT_OK
        );
        inject_nan_matrix(handle);
        assert_eq!(
            miso_engine_web_v1_render(handle, 128),
            RESULT_RENDER_REJECTED
        );
        LIVE_HOST.with(|slot| {
            let mut live = slot.borrow_mut();
            live.as_mut()
                .expect("live host")
                .host
                .reconcile_observation_applications();
        });
        let expected_rows = LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("live host").host;
            assert_eq!(host.side_records.pending_count, 0);
            let rows: Vec<_> = host
                .side_records
                .receipts
                .iter()
                .copied()
                .filter(|row| row.state == OBSERVATION_RECEIPT_STATE_APPLIED)
                .collect();
            assert_eq!(rows.len(), 2);
            rows
        });

        // Once the first real render has completed, all rows are Applied and their identities
        // belong to the native side-records. Invalid handles and both staging-owner borrow
        // refusals must leave those completed rows untouched for a later valid take.
        assert_eq!(
            miso_engine_web_v1_observation_application_take(handle.wrapping_add(1)),
            0
        );
        OBSERVATION_STAGING.with(|slot| {
            let _borrow = slot.borrow_mut();
            assert_eq!(miso_engine_web_v1_observation_application_take(handle), 0);
        });
        LIVE_HOST.with(|slot| {
            let _borrow = slot.borrow_mut();
            assert_eq!(miso_engine_web_v1_observation_application_take(handle), 0);
        });
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("live host").host;
            assert_eq!(host.side_records.pending_count, 0);
            assert_eq!(
                host.side_records
                    .receipts
                    .iter()
                    .copied()
                    .filter(|row| row.state == OBSERVATION_RECEIPT_STATE_APPLIED)
                    .collect::<Vec<_>>(),
                expected_rows,
                "refused takes must preserve completed identities"
            );
        });

        assert_eq!(
            miso_engine_web_v1_observation_application_take(handle),
            2,
            "start and stop must be applied before the same failing render returns"
        );
        let rows = OBSERVATION_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert_eq!(staging.endpoint.application_count, 2);
            [
                staging.endpoint.applications[0],
                staging.endpoint.applications[1],
            ]
        });
        assert_eq!(rows.as_slice(), expected_rows.as_slice());
        assert!(rows.iter().all(|row| {
            row.state == OBSERVATION_RECEIPT_STATE_APPLIED && row.application_sample == 0
        }));
        assert_eq!(
            LIVE_HOST.with(|slot| slot.borrow().as_ref().unwrap().host.status().state),
            STATE_FAILED
        );
        assert_eq!(miso_engine_web_v1_observation_application_take(handle), 0);
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
        no_live_host();
    }

    #[test]
    fn never_rendered_pending_row_is_closed_and_terminal_take_is_one_shot() {
        let handle = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
        no_live_host();

        assert_eq!(miso_engine_web_v1_observation_application_take(handle), 1);
        let row = OBSERVATION_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert_eq!(staging.endpoint.application_count, 1);
            staging.endpoint.applications[0]
        });
        assert_eq!(row.state, OBSERVATION_RECEIPT_STATE_CLOSED);
        assert_eq!(miso_engine_web_v1_observation_application_take(handle), 0);
        assert_eq!(
            OBSERVATION_STAGING.with(|slot| slot.borrow().endpoint.application_count),
            0
        );
        let retained = OBSERVATION_STAGING.with(|slot| slot.borrow().endpoint.applications[0]);
        assert_eq!(
            retained, row,
            "a repeated terminal take must not clear rows"
        );
    }

    #[test]
    fn application_take_borrow_and_invalid_handle_refusals_do_not_consume_pending_rows() {
        let handle = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        assert_eq!(
            miso_engine_web_v1_observation_application_take(handle.wrapping_add(1)),
            0
        );
        OBSERVATION_STAGING.with(|slot| {
            let _borrow = slot.borrow_mut();
            assert_eq!(miso_engine_web_v1_observation_application_take(handle), 0);
        });
        LIVE_HOST.with(|slot| {
            let _borrow = slot.borrow_mut();
            assert_eq!(miso_engine_web_v1_observation_application_take(handle), 0);
        });
        let pending = LIVE_HOST.with(|slot| {
            slot.borrow()
                .as_ref()
                .expect("live host retained")
                .host
                .side_records
                .pending_count
        });
        assert_eq!(pending, 1);
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
        no_live_host();
    }

    #[test]
    fn public_protected_lifecycle_publishes_spectrum_then_response_identity() {
        SPECTRUM_STAGING.with(|slot| slot.borrow_mut().release_capture());
        let handle = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );

        let (pending_status, pending_receipt) = LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            let status = host.observation_status();
            let admission = *host.observation_admission();
            assert_eq!(admission.operation, 4);
            assert_eq!(admission.result, RESULT_OK);
            assert_eq!(
                admission.receipt.state,
                crate::OBSERVATION_RECEIPT_STATE_PENDING
            );
            assert_eq!(status.pending_count, 1);
            assert_ne!(status.owner, 0);
            assert_eq!(admission.receipt.owner, status.owner);
            assert_ne!(admission.receipt.sequence, 0);
            assert_eq!(admission.receipt.sequence, status.accepted_generation);
            assert_eq!(admission.receipt.application_sample, 0);
            (status, admission.receipt)
        });

        render_protected_spectrum_window(handle);

        let applied = OBSERVATION_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert_eq!(staging.endpoint.application_count, 1);
            staging.endpoint.applications[0]
        });
        assert_eq!(applied.state, OBSERVATION_RECEIPT_STATE_APPLIED);
        assert_eq!(applied.result, RESULT_OK);
        assert_eq!(applied.owner, pending_receipt.owner);
        assert_eq!(applied.sequence, pending_receipt.sequence);
        assert_eq!(applied.application_sample, 0);

        let applied_status = LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            let status = host.observation_status();
            assert_eq!(status.owner, pending_status.owner);
            assert_eq!(status.pending_count, 0);
            assert_eq!(status.accepted_generation, pending_receipt.sequence);
            assert_eq!(status.applied_generation, pending_receipt.sequence);
            assert_ne!(status.selection_epoch, 0);
            let output = host.output_pcm().expect("protected output");
            assert_eq!(output.len() % 2, 0);
            let midpoint = output.len() / 2;
            assert!(output[..midpoint].iter().any(|value| *value != 0.0));
            assert!(output[midpoint..].iter().any(|value| *value != 0.0));
            status
        });

        let (spectrum_header, left, right) = SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            let bytes = staging
                .capture
                .as_ref()
                .expect("protected spectrum capture");
            let header: WebSpectrumWindow = read_live_record(bytes, 0).expect("spectrum header");
            let mut left = [0.0_f32; host_core::SPECTRUM_WINDOW_FRAMES];
            let mut right = [0.0_f32; host_core::SPECTRUM_WINDOW_FRAMES];
            spectrum_f32_plane(bytes, header.left_offset, header.frames, &mut left)
                .expect("spectrum left plane");
            spectrum_f32_plane(bytes, header.right_offset, header.frames, &mut right)
                .expect("spectrum right plane");
            (header, left, right)
        });
        assert_eq!(spectrum_header.target, SPECTRUM_TARGET_TRACK_POST_MATRIX);
        assert_eq!(spectrum_header.channels, SPECTRUM_CHANNEL_BOTH);
        assert_eq!(spectrum_header.snapshot_token, 1);
        assert!(left.iter().any(|value| *value != 0.0));
        assert!(right.iter().any(|value| *value != 0.0));

        let spectrum_identity = LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            let identity = *host.observation_capture_identity();
            assert_eq!(identity.kind, 2);
            assert_eq!(identity.flags, 1);
            assert_eq!(identity.owner, applied_status.owner);
            assert_eq!(
                identity.observation_generation,
                applied_status.applied_generation
            );
            assert_eq!(identity.selection_epoch, applied_status.selection_epoch);
            assert_eq!(identity.snapshot_token, spectrum_header.snapshot_token);
            identity
        });

        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            assert_eq!(
                host.observation_status().flags & crate::OBSERVATION_STATUS_FLAG_ORDINARY_AVAILABLE,
                0,
                "the admitted spectrum read spends this boundary's Ordinary credit"
            );
        });
        assert_eq!(
            test_copy_staging(handle, BUFFER_SOURCE_ID, b"fixture-source"),
            RESULT_OK
        );
        assert_eq!(test_fill_source_pcm(handle, 0.25), RESULT_OK);
        assert_eq!(
            miso_engine_web_v1_source_submit(handle, 14, 1, 17 * 128, 2, 128, 0),
            RESULT_OK
        );
        assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            assert_ne!(
                host.observation_status().flags & crate::OBSERVATION_STATUS_FLAG_ORDINARY_AVAILABLE,
                0,
                "a render boundary must replenish Ordinary credit"
            );
        });

        RESPONSE_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            staging.live_track_id.fill(0);
            staging.live_track_id[..3].copy_from_slice(b"eq0");
            staging.live_token = 0;
            staging.live_result_len = 0;
            *staging.live_request = WebLiveResponseRequest {
                struct_size: LIVE_RESPONSE_REQUEST_BYTES,
                abi_version: ABI_VERSION,
                track_id_bytes: 3,
                grid: crate::RESPONSE_GRID_LINEAR,
                channels: crate::RESPONSE_CHANNEL_BOTH,
                points: 5,
                minimum_hz: 20.0,
                maximum_hz: 20_000.0,
                maximum_result_bytes: 65_536,
                reserved: [0; 3],
            };
        });
        assert_eq!(miso_engine_web_v1_track_response_capture(handle), RESULT_OK);

        let response_header = RESPONSE_STAGING.with(|slot| {
            let staging = slot.borrow();
            let bytes = &staging.live_result[..staging.live_result_len];
            let header: WebLiveResponseResult =
                read_live_record(bytes, 0).expect("response header");
            assert_eq!(header.result, RESULT_OK);
            assert_eq!(header.snapshot_token, 1);
            assert!(header.owner_count > 0);
            assert!(header.result_bytes > u64::from(LIVE_RESPONSE_RESULT_BYTES));
            assert_eq!(staging.live_result_len, header.result_bytes as usize);
            assert!(bytes.iter().any(|byte| *byte != 0));
            let owner_offset = header.owners_offset;
            let owner: WebLiveResponseOwner =
                read_live_record(bytes, owner_offset).expect("response owner");
            let track_id = live_payload_bytes(bytes, owner.track_id_offset, owner.track_id_bytes)
                .expect("response owner track ID");
            assert_eq!(track_id, b"eq0");
            header
        });
        let response_identity = LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            let identity = *host.observation_capture_identity();
            assert_eq!(identity.kind, 1);
            assert_eq!(identity.flags, 0);
            assert_eq!(identity.owner, spectrum_identity.owner);
            assert_eq!(identity.observation_generation, 0);
            assert_eq!(identity.selection_epoch, 0);
            assert_eq!(identity.snapshot_token, response_header.snapshot_token);
            assert_eq!(
                host.observation_status().flags & crate::OBSERVATION_STATUS_FLAG_ORDINARY_AVAILABLE,
                0,
                "the response capture spends the replenished Ordinary credit"
            );
            assert_ne!(
                host.observation_status().flags & crate::OBSERVATION_STATUS_FLAG_REMOVAL_AVAILABLE,
                0,
                "the reserved removal credit remains available for stop"
            );
            identity
        });
        assert_eq!(response_identity.owner, applied_status.owner);
        assert_eq!(response_identity.snapshot_token, 1);

        let retired_handle = handle;
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_stop(retired_handle),
            RESULT_OK
        );
        let (stop_status, stop_admission, stop_receipt) = LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            let status = host.observation_status();
            let admission = *host.observation_admission();
            assert_eq!(admission.operation, OBSERVATION_OPERATION_STOP);
            assert_eq!(admission.result, RESULT_OK);
            assert_ne!(admission.flags & crate::OBSERVATION_ADMISSION_RECEIPT, 0);
            assert_ne!(
                admission.flags & crate::OBSERVATION_ADMISSION_PENDING_BOUNDARY,
                0
            );
            assert_eq!(
                admission.receipt.state,
                crate::OBSERVATION_RECEIPT_STATE_PENDING
            );
            assert_eq!(admission.receipt.owner, response_identity.owner);
            assert_eq!(status.owner, response_identity.owner);
            assert_eq!(status.pending_count, 1);
            assert_eq!(host.side_records.pending_count, 1);
            (status, admission, admission.receipt)
        });

        // A repeated stop is the pending identity shortcut: it must not publish another native
        // removal or replace the authoritative Pending receipt.
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_stop(retired_handle),
            RESULT_OK
        );
        LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            assert_eq!(host.observation_status(), stop_status);
            assert_eq!(*host.observation_admission(), stop_admission);
            assert_eq!(host.side_records.pending_count, 1);
        });

        assert_eq!(
            test_copy_staging(retired_handle, BUFFER_SOURCE_ID, b"fixture-source"),
            RESULT_OK
        );
        assert_eq!(test_fill_source_pcm(retired_handle, 0.25), RESULT_OK);
        assert_eq!(
            miso_engine_web_v1_source_submit(retired_handle, 14, 1, 18 * 128, 2, 128, 0),
            RESULT_OK
        );
        assert_eq!(miso_engine_web_v1_render(retired_handle, 128), RESULT_OK);

        let (native_terminal_status, native_terminal_admission, native_response_identity) =
            LIVE_HOST.with(|slot| {
                let live = slot.borrow();
                let host = &live.as_ref().expect("protected live host").host;
                let status = host.observation_status();
                assert_eq!(status.owner, response_identity.owner);
                assert_eq!(status.accepted_generation, 0);
                assert_eq!(status.applied_generation, applied_status.applied_generation);
                assert_eq!(status.selection_epoch, applied_status.selection_epoch);
                assert_eq!(status.pending_count, 1);
                assert_eq!(host.side_records.pending_count, 1);
                (
                    status,
                    *host.observation_admission(),
                    *host.observation_capture_identity(),
                )
            });
        assert_eq!(native_terminal_admission, stop_admission);
        assert_eq!(native_response_identity, response_identity);
        assert_eq!(miso_engine_web_v1_dispose(retired_handle), RESULT_OK);
        no_live_host();

        OBSERVATION_STAGING.with(|slot| {
            let staging = slot.borrow();
            let status = staging.endpoint.status;
            assert_eq!(status.profile, native_terminal_status.profile);
            assert_eq!(status.owner, native_terminal_status.owner);
            assert_eq!(status.ingress_epoch, native_terminal_status.ingress_epoch);
            assert_eq!(status.accepted_generation, 0);
            assert_eq!(status.applied_generation, 0);
            assert_eq!(status.selection_epoch, stop_status.selection_epoch);
            assert_ne!(status.selection_epoch, 0);
            assert_eq!(status.pending_count, 0);
            assert_ne!(status.flags & crate::OBSERVATION_STATUS_FLAG_TERMINAL, 0);
            assert_eq!(
                status.flags
                    & (crate::OBSERVATION_STATUS_FLAG_ORDINARY_AVAILABLE
                        | crate::OBSERVATION_STATUS_FLAG_REMOVAL_AVAILABLE),
                0
            );
            assert_eq!(staging.endpoint.admission, native_terminal_admission);
            assert_eq!(staging.endpoint.capture_identity, native_response_identity);
        });

        assert_eq!(
            miso_engine_web_v1_observation_application_take(retired_handle),
            1
        );
        let stop_application = OBSERVATION_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert_eq!(staging.endpoint.application_count, 1);
            staging.endpoint.applications[0]
        });
        assert_eq!(stop_application.state, OBSERVATION_RECEIPT_STATE_APPLIED);
        assert_eq!(stop_application.result, RESULT_OK);
        assert_eq!(stop_application.domain, stop_receipt.domain);
        assert_eq!(stop_application.owner, stop_receipt.owner);
        assert_eq!(stop_application.sequence, stop_receipt.sequence);
        assert_eq!(stop_application.application_sample, 18 * 128);
        assert_eq!(
            miso_engine_web_v1_observation_application_take(retired_handle),
            0
        );
        assert_eq!(
            OBSERVATION_STAGING.with(|slot| slot.borrow().endpoint.application_count),
            0
        );
    }

    #[test]
    fn dispose_borrow_refusal_preserves_live_owner_and_failed_replacement_preserves_handoff() {
        let handle = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        OBSERVATION_STAGING.with(|slot| {
            let _borrow = slot.borrow_mut();
            assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_INVALID_ARGUMENT);
        });
        assert!(LIVE_HOST.with(|slot| slot.borrow().is_some()));
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
        let terminal_row = OBSERVATION_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert_eq!(staging.endpoint.handle, handle);
            assert_eq!(staging.endpoint.application_count, 1);
            assert!(staging.endpoint.terminal_application_pending);
            staging.endpoint.applications[0]
        });

        test_stage_document(b"not-json");
        assert_eq!(miso_engine_web_v1_boot(8), 0);
        // A failed replacement boot cannot reset the sole retained terminal handoff. The
        // original row remains unconsumed, with its complete owner/sequence/result identity.
        OBSERVATION_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert_eq!(staging.endpoint.handle, handle);
            assert_eq!(staging.endpoint.application_count, 1);
            assert!(staging.endpoint.terminal_application_pending);
            assert_eq!(staging.endpoint.applications[0], terminal_row);
        });
        assert_eq!(miso_engine_web_v1_observation_application_take(handle), 1);
        assert_eq!(
            OBSERVATION_STAGING.with(|slot| slot.borrow().endpoint.applications[0]),
            terminal_row
        );
        assert_eq!(miso_engine_web_v1_observation_application_take(handle), 0);
        assert_eq!(
            OBSERVATION_STAGING.with(|slot| slot.borrow().endpoint.application_count),
            0
        );
        assert_eq!(
            OBSERVATION_STAGING.with(|slot| slot.borrow().endpoint.applications[0]),
            terminal_row,
            "a repeated terminal take must retain the original row bytes"
        );

        let replacement = boot_protected();
        assert_ne!(replacement, 0);
        assert_eq!(miso_engine_web_v1_observation_application_take(handle), 0);
        assert_eq!(
            OBSERVATION_STAGING.with(|slot| slot.borrow().endpoint.application_count),
            0
        );
        assert_eq!(miso_engine_web_v1_dispose(replacement), RESULT_OK);
    }

    #[test]
    fn terminal_drop_retains_actual_status_admission_and_spectrum_identity() {
        let handle = boot_protected();
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        render_protected_spectrum_window(handle);

        let live_capture = LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            let identity = *host.observation_capture_identity();
            assert_eq!(
                identity.kind, 2,
                "protected read must publish spectrum identity"
            );
            assert_eq!(identity.flags, 1);
            assert_ne!(identity.owner, 0);
            assert_ne!(identity.observation_generation, 0);
            assert_ne!(identity.selection_epoch, 0);
            assert_ne!(identity.snapshot_token, 0);
            identity
        });

        stage_stop(handle);
        assert_eq!(
            miso_engine_web_v1_observation_demand_apply(handle),
            RESULT_OK
        );
        let live_admission = LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            let admission = *host.observation_admission();
            assert_eq!(admission.operation, OBSERVATION_OPERATION_STOP_GRAPH);
            assert_eq!(admission.result, RESULT_OK);
            assert_ne!(admission.receipt.owner, 0);
            assert_ne!(admission.receipt.sequence, 0);
            admission
        });

        // Apply the reserved removal at a real render boundary and consume both completed rows so
        // terminal disposal has no hidden application work. The final status still comes from the
        // actual native generations and selection epoch.
        assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        assert_eq!(miso_engine_web_v1_observation_application_take(handle), 1);

        let live_status = LIVE_HOST.with(|slot| {
            let live = slot.borrow();
            let host = &live.as_ref().expect("protected live host").host;
            let status = host.observation_status();
            assert_eq!(status.profile, crate::OBSERVATION_PROFILE_EQ_SPECTRUM);
            assert_ne!(status.owner, 0);
            // The completed StopGraph removes the accepted spectrum. The zero generations here
            // are the native terminal selection, while the successful capture identity above
            // retains the nonzero generation that produced the window.
            assert_eq!(status.accepted_generation, 0);
            assert_eq!(status.applied_generation, 0);
            assert_ne!(status.selection_epoch, 0);
            assert_eq!(status.pending_count, 0);
            status
        });

        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
        no_live_host();

        // Read the actual retained TLS handoff directly. No native pointer is converted through
        // the truncated u32 ABI address in this assertion.
        OBSERVATION_STAGING.with(|slot| {
            let staging = slot.borrow();
            let status = staging.endpoint.status;
            assert_eq!(status.profile, live_status.profile);
            assert_eq!(status.owner, live_status.owner);
            assert_eq!(status.ingress_epoch, live_status.ingress_epoch);
            assert_eq!(status.accepted_generation, live_status.accepted_generation);
            assert_eq!(status.applied_generation, live_status.applied_generation);
            assert_eq!(status.selection_epoch, live_status.selection_epoch);
            assert_eq!(status.pending_count, 0);
            assert_ne!(status.flags & crate::OBSERVATION_STATUS_FLAG_TERMINAL, 0);
            assert_eq!(
                status.flags
                    & (crate::OBSERVATION_STATUS_FLAG_ORDINARY_AVAILABLE
                        | crate::OBSERVATION_STATUS_FLAG_REMOVAL_AVAILABLE),
                0
            );
            assert_eq!(
                status.flags & crate::OBSERVATION_STATUS_FLAG_RENDER_FAILED,
                0
            );
            assert_eq!(staging.endpoint.admission, live_admission);
            assert_eq!(staging.endpoint.capture_identity, live_capture);
            assert_eq!(staging.endpoint.capture_identity.kind, 2);
            assert_eq!(staging.endpoint.capture_identity.owner, status.owner);
            assert_ne!(staging.endpoint.capture_identity.observation_generation, 0);
            assert_ne!(staging.endpoint.capture_identity.selection_epoch, 0);
            assert_ne!(staging.endpoint.capture_identity.snapshot_token, 0);
        });
    }

    #[test]
    fn legacy_boot_fails_before_publication_when_endpoint_staging_is_borrowed() {
        no_live_host();
        let document = protected_document();
        test_stage_document(document);
        OBSERVATION_STAGING.with(|slot| {
            let _borrow = slot.borrow_mut();
            assert_eq!(miso_engine_web_v1_boot(document.len() as u32), 0);
        });
        assert_eq!(miso_engine_web_v1_boot_result(), RESULT_INTERNAL);
        no_live_host();
    }
}
