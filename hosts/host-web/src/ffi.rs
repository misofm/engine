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
//!
//! # Render-locked exports
//!
//! "Boot" is the engine worklet's whole construction path through its `initialize`
//! (`web/miso-engine-v1-audio-worklet.js`) up to the `miso.ready.v1` post, not only the `boot`
//! export. Issue #1333 D2 and issue #1488: every export that either AudioWorklet calls on its
//! render thread after boot wraps its body in `render_locked`, and
//! `miso_engine_web_v1_render_allocation_count` reports the allocator calls made inside those
//! windows. "Either AudioWorklet" is the engine worklet (`process()` and its port handlers) and the
//! SDK's PCM-feed worklet, whose `process()` drains its shared rings before it renders. The one
//! list of that set is the table `tools/realtime-policy/render-thread-exports.json`. Each entry
//! names an export (without the `miso_engine_web_v1_` prefix), the worklet handlers that call it
//! after boot, and whether its body is one `render_locked` call (`wrapped`). The
//! `tools/realtime-policy` tool proves that every `wrapped` entry's body is that call, and both
//! worklet harnesses (`scripts/test-web-audioworklet.mjs` and `sdk/test/browser-pcm-evals.mjs`)
//! fail when a worklet calls an export that the table does not list. This header does not repeat
//! the names: the table is the list, and a name set here would drift from it. The wrapped entries
//! cover the render, command, meter, source, live-control, observation, track-response and
//! spectrum exports, with the pointer, capacity and byte-count accessors of their stagings that
//! the worklets call after boot (issues #1488 and #1492). The two entries with `wrapped: false`
//! are the named exceptions below, each with its reason in the table. Boot is outside the rule,
//! as the next paragraph says.
//!
//! Four post-boot calls are named exceptions: `dispose`, teardown, which frees the host by design
//! and is also the boot-failure path; `render_allocation_count`, the reader of the count; and,
//! since issue #1381a, `service` and `control_allocation_count`. The service step is the single
//! instance's control work: the worklet's message handler runs it after every control message,
//! never in `process()`, and it frees every plan and attachment render displaced, so its calls are
//! the *control* allocations that `control_allocation_count` reads. Since issue #1562a the generic
//! command-frame entry (`control_request_ptr`, `control_command`, `control_response_ptr`,
//! `control_response_bytes`, `control_revision`) is control work of the same kind: it runs
//! `SessionState::command` in the message handler and is unwrapped, its allocations counted as
//! control allocations. Since issue #1570b `control_event`, the event entry, is the same kind of
//! control work, and the worklet's message handler drains both event lanes through it. The
//! render-locked count stays zero. Boot is outside the rule. The `render_lock` module header names the exports it calls,
//! wrapped or not, among them the 22 unwrapped ones it calls after the `boot` export returns a
//! handle.

#![allow(unsafe_code)]

use crate::render_lock::{control_allocation_count, render_allocation_count, render_locked};
use crate::{
    ABI_VERSION, AudioWorkletEngineHost, BUFFER_COMMAND, BUFFER_DIAGNOSTIC, BUFFER_METER_FRAME,
    BUFFER_OUTPUT_PCM, BUFFER_SOURCE_ID, BUFFER_SOURCE_PCM, BootFailure,
    LIVE_RESPONSE_CAPTURE_BYTES, LIVE_RESPONSE_MAXIMUM_ID_BYTES, LIVE_RESPONSE_MAXIMUM_OWNERS,
    LIVE_RESPONSE_MAXIMUM_POINTS, LIVE_RESPONSE_MAXIMUM_SECTIONS,
    LIVE_RESPONSE_MEANING_EQ_FILTER_SUBTOTAL, LIVE_RESPONSE_MODE_TARGET, LIVE_RESPONSE_OWNER_BYTES,
    LIVE_RESPONSE_REQUEST_BYTES, LIVE_RESPONSE_RESULT_BYTES, LIVE_RESPONSE_SECTION_BYTES,
    MAXIMUM_DOCUMENT_BYTES, MAXIMUM_OBSERVATION_READS, OBSERVATION_CHANNEL_BOTH,
    OBSERVATION_CHANNEL_LEFT, OBSERVATION_CHANNEL_RIGHT, OBSERVATION_RESULT_BYTES,
    OBSERVATION_SELECTION_BYTES, OBSERVATION_STATUS_PENDING, OBSERVATION_STATUS_READY,
    OBSERVATION_STATUS_UNARMED, ObservationAddress, ObservationReadChannels, ObservationReadError,
    ObservationReadValues, RESULT_BACKPRESSURE, RESULT_BUFFER_TOO_SMALL, RESULT_INTERNAL,
    RESULT_INVALID_ARGUMENT, RESULT_OK, RESULT_REFUSED_BUDGET, RESULT_REFUSED_DOCUMENT,
    RESULT_REFUSED_LIFECYCLE, RESULT_UNSUPPORTED, RESULT_WRONG_STATE, SPECTRUM_CAPTURE_BYTES,
    SPECTRUM_CHANNEL_BOTH, SPECTRUM_CHANNEL_LEFT, SPECTRUM_CHANNEL_RIGHT,
    SPECTRUM_COLLECTION_ENTRY_BYTES, SPECTRUM_COLLECTION_REQUEST_BYTES, SPECTRUM_MAXIMUM_ID_BYTES,
    SPECTRUM_REQUEST_BYTES, SPECTRUM_RESULT_HEADER_BYTES, SPECTRUM_STREAM_METADATA_BYTES,
    SPECTRUM_STREAM_STATUS_INACTIVE, SPECTRUM_WINDOW_FRAMES, SPECTRUM_WINDOW_HEADER_BYTES,
    STATE_READY, WebBootOptions, WebLiveResponseOwner, WebLiveResponseRequest,
    WebLiveResponseResult, WebLiveResponseSection, WebObservationResult, WebObservationSelection,
    WebSpectrumCollectionEntry, WebSpectrumCollectionRequest, WebSpectrumRequest,
    WebSpectrumResult, WebSpectrumStreamMetadata, WebSpectrumWindow, spectrum_result,
};
use core::{
    cell::{Cell, RefCell},
    mem::{ManuallyDrop, MaybeUninit, needs_drop, size_of},
    ptr, slice,
};
use engine::realtime::{
    ResponseSnapshotError, ResponseSnapshotOwnerInfo, ResponseSnapshotSection, ResponseSnapshotSink,
};
use host_core::{
    ResponsePreviewGrid, ResponseSnapshot, ResponseSnapshotAvailability, ResponseSnapshotOutput,
    ResponseSnapshotOwner, ResponseSnapshotQueryError, SpectrumAnalysis, SpectrumAnalysisHistory,
    SpectrumAnalysisResult, SpectrumChannels, SpectrumContinuousWindow, SpectrumHop,
    SpectrumPreparation, SpectrumRequestError, SpectrumSmoothingConfig, SpectrumSnapshot,
    SpectrumStreamMetadata, SpectrumStreamStatus, SpectrumTargetRef, SpectrumWindow,
    query_response_snapshot_into, spectrum_channels_from_raw, spectrum_smoothing,
};

#[cfg(test)]
use crate::{
    SPECTRUM_STREAM_STATUS_WARMING, SPECTRUM_TARGET_OUTPUT, SPECTRUM_TARGET_TRACK_POST_PAN,
};
#[cfg(test)]
use host_core::SpectrumCadence;

struct LiveHost {
    handle: u32,
    host: Box<AudioWorkletEngineHost>,
}

/// Boot staging is a const-initialised, destructor-free thread local (issue #1333 D5): `options`
/// is inline, and `document` is freed only by assignment, at a staged replacement or after
/// dispose, never by a thread-local destructor.
struct BootStaging {
    options: WebBootOptions,
    document: ManuallyDrop<Vec<u8>>,
    result: u32,
    diagnostic_bytes: u32,
    document_valid: bool,
}

struct ResponseStaging {
    live_request: Box<WebLiveResponseRequest>,
    live_track_id: Box<[u8]>,
    live_result: Vec<u8>,
    live_result_len: usize,
    live_result_header: WebLiveResponseResult,
    live_token: u64,
}

/// The session-less response preview's request and reply frames (#1574). It is allocated on first
/// touch and never reserved by a boot: the preview instance never boots, and no worklet calls it.
#[derive(Default)]
struct ResponseFrameStaging {
    request: Vec<u8>,
    reply: Vec<u8>,
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
    /// Worker-owned analysis storage: the analyzer and one result, made together, once.
    analysis: Option<SpectrumAnalysis>,
    /// The result's three 1,025-bin planes (frequencies, left and right dBFS) and its header words:
    /// 3 x 1,025 x 4 = 12,300 bytes plus about 50, so roughly 12 KiB, boxed so the staging moves
    /// by pointer. Like the analyzer it is made on first use by a worker, retained after, and not
    /// in the host's retained resource report (see `spectrum_staging_retained_bytes`).
    analysis_result: Option<Box<SpectrumAnalysisResult>>,
    /// Additive stream metadata is inline so exposing its pointer never allocates after boot. The
    /// stream state machine is the core's (`SpectrumService` in the host, `SpectrumAnalysis`
    /// here); this is the record that the browser reads and, in an analysis worker, writes.
    stream_metadata: WebSpectrumStreamMetadata,
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
            analysis: None,
            analysis_result: None,
            stream_metadata: WebSpectrumStreamMetadata {
                struct_size: SPECTRUM_STREAM_METADATA_BYTES,
                abi_version: ABI_VERSION,
                status: SPECTRUM_STREAM_STATUS_INACTIVE,
                ..WebSpectrumStreamMetadata::default()
            },
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
        self.analysis = None;
        self.analysis_result = None;
        self.capture_len = 0;
        self.result_len = 0;
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
        if self.analysis.is_none() {
            self.analysis = Some(SpectrumAnalysis::new());
        }
        if self.analysis_result.is_none() {
            self.analysis_result = Some(Box::new(SpectrumAnalysisResult::new()));
        }
        Ok(())
    }
}

/// What a test reads of a staging analysis's stream state: running, cadence, smoothing, window at
/// hand and whether the history exists.
#[cfg(test)]
type AnalysisState = (
    bool,
    Option<SpectrumCadence>,
    Option<SpectrumSmoothingConfig>,
    bool,
    bool,
);

#[cfg(test)]
fn analysis_stream_state(staging: &SpectrumStaging) -> AnalysisState {
    staging
        .analysis
        .as_ref()
        .map_or((false, None, None, false, false), |analysis| {
            (
                analysis.stream_active(),
                analysis.stream_cadence(),
                analysis.stream_smoothing(),
                analysis.has_stream_window(),
                analysis.has_stream_history(),
            )
        })
}

#[cfg(test)]
fn analysis_has_history() -> bool {
    with_spectrum_staging(|slot| analysis_stream_state(&slot.borrow()).4)
}

#[cfg(test)]
fn analysis_cadence() -> Option<SpectrumCadence> {
    with_spectrum_staging(|slot| analysis_stream_state(&slot.borrow()).1)
}

fn validate_spectrum_capture_capacity(capacity: usize) -> Result<(), u32> {
    if capacity > SPECTRUM_CAPTURE_BYTES {
        Err(RESULT_REFUSED_BUDGET)
    } else {
        Ok(())
    }
}

/// The longest stable observation ID the observation staging holds.
const OBSERVATION_MAXIMUM_ID_BYTES: u32 = 127;

struct ObservationStaging {
    selections: Box<[WebObservationSelection]>,
    addresses: Vec<ObservationAddress>,
    rows: Box<[ObservationReadValues]>,
    results: Vec<WebObservationResult>,
    id: Box<[u8]>,
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
            id: vec![0; OBSERVATION_MAXIMUM_ID_BYTES as usize].into_boxed_slice(),
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
    match bytes.checked_add(OBSERVATION_MAXIMUM_ID_BYTES as u64) {
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
    let ids = OBSERVATION_MAXIMUM_ID_BYTES as u64;
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
    size_of::<WebLiveResponseRequest>() as u64
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
/// buffers are allocated only for a configured capture; the analyzer and the boxed analysis result
/// (about 12 KiB) are worker-owned and are not part of the host's retained resource report.
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
            // The host's boxed stream window (`SpectrumService`) is charged with the staging it
            // is copied into.
            SPECTRUM_CAPTURE_PAYLOAD_BYTES
                + SPECTRUM_STREAM_METADATA_BYTES as u64
                + size_of::<SpectrumContinuousWindow>() as u64
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
        let history = max_u64(
            spectrum_analysis_history_retained_bytes(true),
            size_of::<SpectrumContinuousWindow>() as u64,
        );
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
}

/// The all-zero boot options, which select engine defaults; `WebBootOptions::default()` in a
/// `const` context.
const ZERO_BOOT_OPTIONS: WebBootOptions = WebBootOptions {
    struct_size: 0,
    abi_version: 0,
    require_sample_rate_hz: 0,
    require_quantum_frames: 0,
    source_ring_frames: 0,
    reserved0: 0,
    maximum_memory_bytes: 0,
    reserved1: 0,
    live_control_meter_blocks: 0,
    live_control_observation_taps: 0,
    live_control_master_track_plus_one: 0,
};

impl BootStaging {
    const fn new() -> Self {
        Self {
            options: ZERO_BOOT_OPTIONS,
            document: ManuallyDrop::new(Vec::new()),
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
        self.options = WebBootOptions::default();
        // Assignment through `DerefMut` frees the staged bytes; the `ManuallyDrop` only keeps a
        // thread-local destructor from ever being registered for them.
        *self.document = Vec::new();
        self.result = RESULT_OK;
        self.diagnostic_bytes = 0;
        self.document_valid = false;
    }
}

// Issue #1333 D5: every thread local here is const-initialised and needs no drop. With `+atomics`
// std registers a destructor, lazily and by allocating, for a thread local that needs drop, and a
// lazily initialised one allocates on first touch; either would put an allocation inside the
// worklet's render-locked window. `LIVE_HOST` is dropped explicitly by dispose's `take()`, and
// the stagings live for their instance's whole life, as lazily initialised statics always did.
type LiveHostSlot = RefCell<ManuallyDrop<Option<LiveHost>>>;
type StagingSlot<T> = Cell<Option<&'static RefCell<T>>>;

thread_local! {
    static LIVE_HOST: LiveHostSlot = const { RefCell::new(ManuallyDrop::new(None)) };
    // Zero is the never-issued sentinel. `next_handle` preserves the established nonzero
    // handle contract when the first owner is published, while cold invalid lookups can avoid
    // constructing the allocating observation staging entirely.
    static NEXT_HANDLE: Cell<u32> = const { Cell::new(0) };
    static BOOT_STAGING: RefCell<BootStaging> = const { RefCell::new(BootStaging::new()) };
    static RESPONSE_STAGING: StagingSlot<ResponseStaging> = const { Cell::new(None) };
    static SPECTRUM_STAGING: StagingSlot<SpectrumStaging> = const { Cell::new(None) };
    static OBSERVATION_STAGING: StagingSlot<ObservationStaging> = const { Cell::new(None) };
    static RESPONSE_FRAME_STAGING: StagingSlot<ResponseFrameStaging> = const { Cell::new(None) };
}

const _: () = assert!(!needs_drop::<LiveHostSlot>());
const _: () = assert!(!needs_drop::<RefCell<BootStaging>>());
const _: () = assert!(!needs_drop::<StagingSlot<ResponseStaging>>());
const _: () = assert!(!needs_drop::<StagingSlot<SpectrumStaging>>());
const _: () = assert!(!needs_drop::<StagingSlot<ObservationStaging>>());
const _: () = assert!(!needs_drop::<StagingSlot<ResponseFrameStaging>>());

/// The staging in `slot`, allocated once on first touch and never freed.
fn leaked_staging<T: 'static>(
    slot: &'static std::thread::LocalKey<StagingSlot<T>>,
    new: fn() -> T,
) -> &'static RefCell<T> {
    slot.with(|slot| {
        if let Some(staging) = slot.get() {
            return staging;
        }
        let staging: &'static RefCell<T> = Box::leak(Box::new(RefCell::new(new())));
        slot.set(Some(staging));
        staging
    })
}

fn response_staging() -> &'static RefCell<ResponseStaging> {
    leaked_staging(&RESPONSE_STAGING, ResponseStaging::new)
}

fn spectrum_staging() -> &'static RefCell<SpectrumStaging> {
    leaked_staging(&SPECTRUM_STAGING, SpectrumStaging::new)
}

fn observation_staging() -> &'static RefCell<ObservationStaging> {
    leaked_staging(&OBSERVATION_STAGING, ObservationStaging::new)
}

fn with_response_frame_staging<R>(
    operation: impl FnOnce(&RefCell<ResponseFrameStaging>) -> R,
) -> R {
    operation(leaked_staging(
        &RESPONSE_FRAME_STAGING,
        ResponseFrameStaging::default,
    ))
}

fn with_response_staging<R>(operation: impl FnOnce(&RefCell<ResponseStaging>) -> R) -> R {
    operation(response_staging())
}

fn with_spectrum_staging<R>(operation: impl FnOnce(&RefCell<SpectrumStaging>) -> R) -> R {
    operation(spectrum_staging())
}

fn with_observation_staging<R>(operation: impl FnOnce(&RefCell<ObservationStaging>) -> R) -> R {
    operation(observation_staging())
}

/// Allocate all three stagings once a boot has published its handle. `boot_staged` calls this
/// on its one success path, which both boot exports share, so a booted instance never allocates
/// a staging later, inside or outside a render-locked window (issue #1333 D5). A refused boot
/// allocates none of them, so a typed refusal stays within its memory bound, and an instance that
/// never boots allocates each on first touch, outside any window.
fn reserve_stagings() {
    let _ = response_staging();
    let _ = spectrum_staging();
    let _ = observation_staging();
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

/// Verify that `handle` names the live host before an alias touches caller-owned staging.
///
/// A failed live-host borrow and every nonmatching handle are terminal at the ABI boundary: the
/// spectrum and resident-observation bodies write shared staging records, and would record a
/// refusal there, so they run only after the matching host has been inspected. The check takes
/// the same exclusive borrow those bodies later need, so an immutable conflict cannot pass it.
fn verify_live_host(handle: u32) -> Result<(), u32> {
    if handle == 0 {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    LIVE_HOST.with(|slot| {
        let slot = slot.try_borrow_mut().map_err(|_| RESULT_INTERNAL)?;
        slot.as_ref()
            .filter(|live| live.handle == handle)
            .map(|_| ())
            .ok_or(RESULT_INVALID_ARGUMENT)
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

/// An observation record's `rack`: `inserts` (1) or `console` (3). The retired `simd1` (0) and
/// `simd2` (2) codes are refused and never reinterpreted (decision 12, issue #1096).
fn observation_rack(raw: u32) -> Result<crate::LiveEffectRack, u32> {
    crate::live_effect_address(raw, 0)
        .map(|address| address.rack)
        .ok_or(RESULT_INVALID_ARGUMENT)
}

/// A live-response owner record's `rack` for the engine's native owner rack (issue #1096).
///
/// The engine reports `0` for the input filters and the internal rack code otherwise (`1` for a
/// `console.pre_insert` slot, `2` for an insert and `3` for a `console.post_insert` slot). The
/// record keeps `0` and the inserts' `2`, retires `1` and `3`, and names both console sections
/// `4`, since a live address does not name the section.
const fn live_response_rack_raw(native: u8) -> Option<u32> {
    match native {
        0 => Some(crate::LIVE_RESPONSE_RACK_INPUT_FILTERS),
        2 => Some(crate::LIVE_RESPONSE_RACK_INSERTS),
        1 | 3 => Some(crate::LIVE_RESPONSE_RACK_CONSOLE),
        _ => None,
    }
}

/// Validate a live-response owner record's `rack`. The retired `1` and `3` are refused.
fn live_response_rack(raw: u32) -> Result<u8, u32> {
    match raw {
        crate::LIVE_RESPONSE_RACK_INPUT_FILTERS
        | crate::LIVE_RESPONSE_RACK_INSERTS
        | crate::LIVE_RESPONSE_RACK_CONSOLE => {
            u8::try_from(raw).map_err(|_| RESULT_INVALID_ARGUMENT)
        }
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

fn observation_rack_raw(rack: crate::LiveEffectRack) -> u32 {
    u32::from(crate::live_effect_rack_raw(rack))
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

/// The result word of a refused spectrum request word: a budget is a budget refusal, every other
/// word an invalid argument.
const fn spectrum_request_result(error: SpectrumRequestError) -> u32 {
    match error {
        SpectrumRequestError::Budget => RESULT_REFUSED_BUDGET,
        SpectrumRequestError::TargetKind
        | SpectrumRequestError::TargetId
        | SpectrumRequestError::Channels => RESULT_INVALID_ARGUMENT,
    }
}

fn spectrum_channels(raw: u32) -> Result<SpectrumChannels, u32> {
    spectrum_channels_from_raw(raw).map_err(spectrum_request_result)
}

/// Validate a raw target kind and identity and borrow them as a target. Allocates nothing, so the
/// select path calls it on the browser's audio thread.
fn spectrum_target_ref(raw: u32, id: &str) -> Result<SpectrumTargetRef<'_>, u32> {
    host_core::spectrum_target_ref(raw, id).map_err(spectrum_request_result)
}

fn spectrum_configured(staging: &SpectrumStaging) -> bool {
    staging.request.target != 0 || staging.collection_request.entry_count != 0
}

fn staged_spectrum_request(staging: &SpectrumStaging) -> Result<Option<SpectrumPreparation>, u32> {
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
        // The staging area bounds the slice (transport); the identity's own rule, 1 to 127 bytes,
        // is the core's.
        let id_bytes =
            usize::try_from(request.target_id_bytes).map_err(|_| RESULT_INVALID_ARGUMENT)?;
        if id_bytes > staging.target_id.len() {
            return Err(RESULT_INVALID_ARGUMENT);
        }
        let id = core::str::from_utf8(&staging.target_id[..id_bytes])
            .map_err(|_| RESULT_INVALID_ARGUMENT)?;
        return SpectrumPreparation::single(
            request.target,
            id,
            request.channels,
            request.maximum_capture_bytes,
        )
        .map(Some)
        .map_err(spectrum_request_result);
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
        // The core owns the empty-collection rule: budget 0 prepares nothing, any other budget is
        // its budget refusal.
        return SpectrumPreparation::collection(&[], collection.maximum_capture_bytes)
            .map_err(spectrum_request_result);
    }
    // A collection's limit covers all of its prepared entries. The core applies the aggregate
    // rule (non-zero, no one-window cap); the host preparation path applies the checked
    // aggregate/resource limits.
    let mut entries = Vec::new();
    entries
        .try_reserve_exact(count)
        .map_err(|_| RESULT_REFUSED_BUDGET)?;
    let mut id_start = 0_usize;
    for staged in &staging.collection_entries[..count] {
        if staged.reserved != [0; 3] {
            return Err(RESULT_INVALID_ARGUMENT);
        }
        let id_bytes =
            usize::try_from(staged.target_id_bytes).map_err(|_| RESULT_INVALID_ARGUMENT)?;
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
        entries.push((staged.target, id, staged.channels));
        id_start = id_end;
    }
    SpectrumPreparation::collection(&entries, collection.maximum_capture_bytes)
        .map_err(spectrum_request_result)
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

/// The core's stream metadata of a record. A status code the core does not know reads as inactive.
fn stream_metadata_of(record: &WebSpectrumStreamMetadata) -> SpectrumStreamMetadata {
    SpectrumStreamMetadata {
        status: SpectrumStreamStatus::from_code(record.status).unwrap_or_default(),
        target_kind: record.target,
        channels: record.channels,
        sample_rate_hz: record.sample_rate_hz,
        quantum_frames: record.quantum_frames,
        hop_frames: record.hop_frames,
        source_underrun: record.source_underrun != 0,
        capture_epoch: record.capture_epoch,
        sequence: record.sequence,
        dropped_captures: record.dropped_captures,
        windows: record.windows,
        captured_sample: record.captured_sample,
        end_sample: record.end_sample,
        analysis_epoch: record.analysis_epoch,
        history_start_sample: record.history_start_sample,
        smoothing_ms: record.smoothing_ms,
    }
}

/// Write the core's stream metadata into a record, except its record words (size, version, result
/// and the two reserved words).
fn write_stream_metadata(
    record: &mut WebSpectrumStreamMetadata,
    metadata: &SpectrumStreamMetadata,
) {
    record.status = metadata.status.code();
    record.target = metadata.target_kind;
    record.channels = metadata.channels;
    record.sample_rate_hz = metadata.sample_rate_hz;
    record.quantum_frames = metadata.quantum_frames;
    record.hop_frames = metadata.hop_frames;
    record.source_underrun = u32::from(metadata.source_underrun);
    record.capture_epoch = metadata.capture_epoch;
    record.sequence = metadata.sequence;
    record.dropped_captures = metadata.dropped_captures;
    record.windows = metadata.windows;
    record.captured_sample = metadata.captured_sample;
    record.end_sample = metadata.end_sample;
    record.analysis_epoch = metadata.analysis_epoch;
    record.history_start_sample = metadata.history_start_sample;
    record.smoothing_ms = metadata.smoothing_ms;
}

/// The record checks of an imported ready window: the record words of the metadata and the raw
/// window header it describes. The semantic checks are the core's
/// (`SpectrumAnalysis::configure_imported`).
fn imported_stream_metadata(staging: &SpectrumStaging) -> Result<SpectrumStreamMetadata, u32> {
    let record = staging.stream_metadata;
    if record.struct_size != SPECTRUM_STREAM_METADATA_BYTES
        || record.abi_version != ABI_VERSION
        || record.result != RESULT_OK
        || record.source_underrun > 1
        || record.reserved0 != 0
        || record.reserved1 != 0
    {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    let bytes = staging
        .capture
        .as_ref()
        .and_then(|capture| capture.get(..staging.capture_len))
        .ok_or(RESULT_INVALID_ARGUMENT)?;
    let header: WebSpectrumWindow =
        read_live_record(bytes, 0).map_err(|_| RESULT_INVALID_ARGUMENT)?;
    if header.struct_size != SPECTRUM_WINDOW_HEADER_BYTES
        || header.abi_version != ABI_VERSION
        || header.target != record.target
        || header.channels != record.channels
        || header.sample_rate_hz != record.sample_rate_hz
        || header.frames != SPECTRUM_WINDOW_FRAMES
        || header.source_underrun != record.source_underrun
        || header.captured_sample != record.captured_sample
        || header.end_sample != record.end_sample
        || header.snapshot_token == 0
        || staging.capture_len != spectrum_capture_bytes(header.channels)
    {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    Ok(stream_metadata_of(&record))
}

/// Adopt the stream that the metadata record describes in this instance's analysis.
fn adopt_imported_stream(staging: &mut SpectrumStaging) -> Result<(), u32> {
    let metadata = imported_stream_metadata(staging)?;
    staging.ensure_analysis_storage()?;
    staging
        .analysis
        .as_mut()
        .ok_or(RESULT_INTERNAL)?
        .configure_imported(&metadata)
        .map_err(spectrum_result)
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

fn write_spectrum_window(staging: &mut SpectrumStaging, snapshot: &SpectrumSnapshot) -> u32 {
    // The core's one snapshot rule: a valid target kind, a representable end sample, a non-zero
    // token and a launch sample rate.
    if let Err(error) = snapshot.validate() {
        return spectrum_failure(staging, spectrum_result(error));
    }
    let SpectrumSnapshot {
        window,
        target_kind: target,
        sample_rate_hz,
        token,
    } = snapshot;
    let (target, sample_rate_hz, token) = (*target, *sample_rate_hz, *token);
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
    if staging.ensure_analysis_storage().is_err() {
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
    // The record checks. The semantic checks (kind, token, end sample, rate) are the snapshot's.
    if header.struct_size != SPECTRUM_WINDOW_HEADER_BYTES
        || header.abi_version != ABI_VERSION
        || !(SPECTRUM_CHANNEL_LEFT..=SPECTRUM_CHANNEL_BOTH).contains(&header.channels)
        || header.frames != SPECTRUM_WINDOW_FRAMES
        || header.source_underrun > 1
        || header.reserved0 != 0
        || header.end_sample
            != header
                .captured_sample
                .saturating_add(u64::from(SPECTRUM_WINDOW_FRAMES))
        || staging.capture_len != spectrum_capture_bytes(header.channels)
    {
        return spectrum_failure(staging, RESULT_INVALID_ARGUMENT);
    }
    let mut snapshot = SpectrumSnapshot {
        window: SpectrumWindow {
            left: [0.0; host_core::SPECTRUM_WINDOW_FRAMES],
            right: [0.0; host_core::SPECTRUM_WINDOW_FRAMES],
            first_sample: header.captured_sample,
            channels: match spectrum_channels(header.channels) {
                Ok(value) => value,
                Err(result) => return spectrum_failure(staging, result),
            },
            source_underrun: header.source_underrun != 0,
        },
        target_kind: header.target,
        sample_rate_hz: header.sample_rate_hz,
        token: header.snapshot_token,
    };
    if let Err(error) = snapshot.validate() {
        return spectrum_failure(staging, spectrum_result(error));
    }
    let left = (header.channels & SPECTRUM_CHANNEL_LEFT) != 0;
    let right = (header.channels & SPECTRUM_CHANNEL_RIGHT) != 0;
    if left
        && spectrum_f32_plane(
            bytes,
            header.left_offset,
            header.frames,
            &mut snapshot.window.left,
        )
        .is_err()
    {
        return spectrum_failure(staging, RESULT_INVALID_ARGUMENT);
    }
    if right
        && spectrum_f32_plane(
            bytes,
            header.right_offset,
            header.frames,
            &mut snapshot.window.right,
        )
        .is_err()
    {
        return spectrum_failure(staging, RESULT_INVALID_ARGUMENT);
    }
    let window = &snapshot.window;
    let analysis = staging
        .analysis
        .as_mut()
        .expect("analysis storage initialized");
    let result = staging
        .analysis_result
        .as_mut()
        .expect("analysis storage initialized");
    let continuous_metadata = if continuous {
        let mut metadata = stream_metadata_of(&staging.stream_metadata);
        let continuous_window = SpectrumContinuousWindow {
            left: window.left,
            right: window.right,
            first_sample: window.first_sample,
            channels: window.channels,
            source_underrun: window.source_underrun,
            stream_epoch: metadata.capture_epoch,
            sequence: metadata.sequence,
            dropped_captures: metadata.dropped_captures,
        };
        match analysis.analyze_stream_into(&continuous_window, result, &mut metadata) {
            Ok(()) => Some(metadata),
            Err(error) => return spectrum_failure(staging, spectrum_result(error)),
        }
    } else {
        if let Err(error) = analysis.analyze_into(&snapshot, result) {
            return spectrum_failure(staging, spectrum_result(error));
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
        &result.frequencies_hz,
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
            &result.left_dbfs,
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
            &result.right_dbfs,
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
        write_stream_metadata(&mut staging.stream_metadata, &metadata);
        staging.stream_metadata.result = RESULT_OK;
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
            rack: live_response_rack_raw(owner.rack).ok_or(ResponseSnapshotError::InvalidShape)?,
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
            // The worker-side copy keeps the record's own code: both console sections share one
            // code by design, so the engine's native rack is not recoverable here, and the query
            // does not read it. A retired or unknown code refuses the whole snapshot.
            rack: live_response_rack(raw.rack)?,
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

/// The most request frame bytes the session-less response entry accepts.
const RESPONSE_FRAME_MAXIMUM_REQUEST_BYTES: u32 = 1 << 20;

/// Stage a request frame of `bytes` bytes for the session-less response entry and return its
/// writable address, or `0` when `bytes` is outside `1..=1 << 20` or the allocation is refused.
/// Control work of a preview instance: no render path calls it, and no worklet.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_response_frame_ptr(bytes: u32) -> u32 {
    if bytes == 0 || bytes > RESPONSE_FRAME_MAXIMUM_REQUEST_BYTES {
        return 0;
    }
    with_response_frame_staging(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return 0;
        };
        staging.reply.clear();
        staging.request.clear();
        if staging.request.try_reserve_exact(bytes as usize).is_err() {
            return 0;
        }
        staging.request.resize(bytes as usize, 0);
        pointer_u32(staging.request.as_ptr())
    })
}

/// Serve the staged request frame of `bytes` bytes through the core's session-less response entry.
/// `RESULT_OK` when a reply frame exists (OK or non-OK), `RESULT_INVALID_ARGUMENT` when the frame
/// has no decodable command header, which leaves no reply, and `RESULT_INTERNAL` when not even a
/// non-OK reply frame could be encoded, which leaves no reply either.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_response_frame_submit(bytes: u32) -> u32 {
    with_response_frame_staging(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        let staging = &mut *staging;
        let Some(request) = staging.request.get(..bytes as usize) else {
            return RESULT_INVALID_ARGUMENT;
        };
        match host_core::serve_response_frame(request, &mut staging.reply) {
            Ok(()) => RESULT_OK,
            Err(host_core::ResponseFrameError::Malformed) => RESULT_INVALID_ARGUMENT,
            Err(host_core::ResponseFrameError::Internal) => RESULT_INTERNAL,
        }
    })
}

/// Return the address of the reply frame, or `0` when there is none.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_response_frame_reply_ptr() -> u32 {
    with_response_frame_staging(|slot| {
        let Ok(staging) = slot.try_borrow() else {
            return 0;
        };
        if staging.reply.is_empty() {
            return 0;
        }
        pointer_u32(staging.reply.as_ptr())
    })
}

/// Return the reply frame's byte length.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_response_frame_reply_bytes() -> u32 {
    with_response_frame_staging(|slot| {
        slot.try_borrow()
            .ok()
            .and_then(|staging| u32::try_from(staging.reply.len()).ok())
            .unwrap_or(0)
    })
}

/// Free the request and reply buffers.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_response_frame_close() -> u32 {
    with_response_frame_staging(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        *staging = ResponseFrameStaging::default();
        RESULT_OK
    })
}

/// Return the writable pre-boot spectrum request header.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_request_ptr() -> u32 {
    with_spectrum_staging(|slot| {
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
    with_spectrum_staging(|slot| {
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
    with_spectrum_staging(|slot| {
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
    with_spectrum_staging(|slot| {
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
    with_spectrum_staging(|slot| {
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
    with_spectrum_staging(|slot| {
        let Ok(staging) = slot.try_borrow() else {
            return 0;
        };
        u32::try_from(staging.collection_target_ids.len()).unwrap_or(0)
    })
}

/// Return writable staging for the selected spectrum target identity.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_target_id_ptr() -> u32 {
    render_locked(|| {
        with_spectrum_staging(|slot| {
            let Ok(mut staging) = slot.try_borrow_mut() else {
                return 0;
            };
            pointer_u32(staging.target_id.as_mut_ptr())
        })
    })
}

/// Return the maximum spectrum target identity byte length.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_target_id_capacity() -> u32 {
    render_locked(|| SPECTRUM_MAXIMUM_ID_BYTES as u32)
}

/// Return the fixed raw spectrum-window staging address.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_capture_ptr() -> u32 {
    render_locked(|| {
        with_spectrum_staging(|slot| {
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
    })
}

/// Return the maximum raw spectrum-window staging capacity.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_capture_capacity() -> u32 {
    render_locked(|| {
        with_spectrum_staging(|slot| {
            slot.try_borrow().ok().map_or(0, |staging| {
                staging
                    .capture
                    .as_ref()
                    .and_then(|capture| u32::try_from(capture.len()).ok())
                    .unwrap_or(0)
            })
        })
    })
}

/// Return the byte length of the most recent raw spectrum window.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_capture_bytes() -> u32 {
    render_locked(|| {
        with_spectrum_staging(|slot| {
            slot.try_borrow()
                .ok()
                .and_then(|staging| u32::try_from(staging.capture_len).ok())
                .unwrap_or(0)
        })
    })
}

/// Set the byte length of raw spectrum capture bytes copied into the fixed staging area.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_capture_set_bytes(bytes: u32) -> u32 {
    with_spectrum_staging(|slot| {
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
    with_spectrum_staging(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        run_spectrum_analysis(&mut staging, false)
    })
}

/// Release the one-shot spectrum staging payload lengths.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_close() -> u32 {
    with_spectrum_staging(|slot| {
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
    with_spectrum_staging(|slot| {
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
    with_spectrum_staging(|slot| {
        slot.try_borrow()
            .ok()
            .and_then(|staging| u32::try_from(staging.result_len).ok())
            .unwrap_or(0)
    })
}

/// Arm the prepared spectrum observer for its next complete window.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_arm(handle: u32) -> u32 {
    render_locked(|| {
        with_host_mut(
            handle,
            RESULT_INVALID_ARGUMENT,
            AudioWorkletEngineHost::arm_spectrum,
        )
    })
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
    if let Err(result) = verify_live_host(handle) {
        return result;
    }
    with_spectrum_staging(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        let staging = &mut *staging;
        let id_bytes = match usize::try_from(target_id_bytes) {
            Ok(value) if value > 0 && value <= staging.target_id.len() => value,
            _ => return RESULT_INVALID_ARGUMENT,
        };
        let id = match core::str::from_utf8(&staging.target_id[..id_bytes]) {
            Ok(value) => value,
            Err(_) => return RESULT_INVALID_ARGUMENT,
        };
        let target = match spectrum_target_ref(target, id) {
            Ok(value) => value,
            Err(result) => return result,
        };
        let channels = match spectrum_channels(channels) {
            Ok(value) => value,
            Err(result) => return result,
        };
        // The core decides between the entry selection and the stream's restarting selection,
        // and commits the selection, the smoothing, the history reset and the metadata as one
        // operation.
        let restarted = with_host_mut(handle, Err(RESULT_INVALID_ARGUMENT), |host| {
            host.select_spectrum_target(target, channels, smoothing, staging.analysis.as_mut())
        });
        match restarted {
            Err(result) => result,
            Ok(false) => RESULT_OK,
            Ok(true) => {
                // A restart also drops the window staged for the old stream.
                let metadata = with_host(handle, None, |host| {
                    host.spectrum_stream_metadata().copied()
                });
                if let Some(metadata) = metadata {
                    write_stream_metadata(&mut staging.stream_metadata, &metadata);
                }
                staging.stream_metadata.result = RESULT_OK;
                staging.capture_len = 0;
                staging.result_len = 0;
                RESULT_OK
            }
        }
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
    render_locked(|| select_spectrum_internal(handle, target, channels, target_id_bytes))
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
    render_locked(|| {
        if let Err(result) = verify_live_host(handle) {
            return result;
        }
        let smoothing = match SpectrumSmoothingConfig::new(smoothing_ms) {
            Ok(value) => value,
            Err(_) => return RESULT_INVALID_ARGUMENT,
        };
        select_spectrum_with_smoothing(handle, target, channels, target_id_bytes, Some(smoothing))
    })
}

/// Return the monotonic identity of the currently committed collection selection.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_selection_epoch(handle: u32) -> u64 {
    with_host(handle, 0, AudioWorkletEngineHost::spectrum_selection_epoch)
}

/// Read a completed spectrum window into fixed staging; returns backpressure while pending.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_read(handle: u32, channels: u32) -> u32 {
    render_locked(|| {
        if let Err(result) = verify_live_host(handle) {
            return result;
        }
        with_spectrum_staging(|slot| {
            let Ok(mut staging) = slot.try_borrow_mut() else {
                return RESULT_INTERNAL;
            };
            staging.capture_len = 0;
            let requested = match spectrum_channels(channels) {
                Ok(value) => value,
                Err(result) => return result,
            };
            let read = with_host_mut(handle, Err(RESULT_INVALID_ARGUMENT), |host| {
                host.read_spectrum(requested)
            });
            let snapshot = match read {
                Ok(Some(value)) => value,
                Ok(None) => return RESULT_BACKPRESSURE,
                Err(result) => return result,
            };
            write_spectrum_window(&mut staging, &snapshot)
        })
    })
}

/// Start the managed spectrum stream for the prepared target.
///
/// The host supplies the sample rate, quantum and hop in the returned metadata. The smoothing
/// duration is validated before the prepared capture changes state.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_stream_start(handle: u32, smoothing_ms: f64) -> u32 {
    render_locked(|| {
        if let Err(result) = verify_live_host(handle) {
            return result;
        }
        let smoothing = match spectrum_smoothing(smoothing_ms) {
            Ok(value) => value,
            Err(error) => return spectrum_result(error),
        };
        with_spectrum_staging(|slot| {
            let Ok(mut staging) = slot.try_borrow_mut() else {
                return RESULT_INTERNAL;
            };
            let staging = &mut *staging;
            // An existing history is reset at this control boundary. The analysis owner creates a
            // missing history when it first evaluates a window, so stream start itself never
            // allocates a second host-side analyzer.
            let started = with_host_mut(handle, Err(RESULT_INVALID_ARGUMENT), |host| {
                host.start_spectrum_stream(smoothing, staging.analysis.as_mut())
            });
            if let Err(result) = started {
                staging.stream_metadata.result = result;
                return result;
            }
            let metadata = with_host(handle, None, |host| {
                host.spectrum_stream_metadata().copied()
            });
            staging.capture_len = 0;
            staging.result_len = 0;
            staging.stream_metadata = WebSpectrumStreamMetadata {
                struct_size: SPECTRUM_STREAM_METADATA_BYTES,
                abi_version: ABI_VERSION,
                result: RESULT_OK,
                ..WebSpectrumStreamMetadata::default()
            };
            if let Some(metadata) = metadata {
                write_stream_metadata(&mut staging.stream_metadata, &metadata);
            }
            RESULT_OK
        })
    })
}

/// Read one independently scheduled stream window into the existing fixed capture staging.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_stream_read(handle: u32) -> u32 {
    render_locked(|| {
        if let Err(result) = verify_live_host(handle) {
            return result;
        }
        with_spectrum_staging(|slot| {
            let Ok(mut staging) = slot.try_borrow_mut() else {
                return RESULT_INTERNAL;
            };
            let staging = &mut *staging;
            staging.capture_len = 0;
            staging.result_len = 0;
            let result = with_host_mut(handle, RESULT_WRONG_STATE, |host| {
                host.read_spectrum_stream(staging.analysis.as_mut())
            });
            let metadata = with_host(handle, None, |host| {
                host.spectrum_stream_metadata().copied()
            });
            let Some(metadata) = metadata else {
                // A host without a spectrum observer reads as an inactive stream.
                staging.stream_metadata.status = SPECTRUM_STREAM_STATUS_INACTIVE;
                staging.stream_metadata.source_underrun = 0;
                staging.stream_metadata.result = result;
                return result;
            };
            write_stream_metadata(&mut staging.stream_metadata, &metadata);
            staging.stream_metadata.result = result;
            if metadata.status != SpectrumStreamStatus::Ready || result != RESULT_OK {
                return result;
            }
            // The ready window is the core's; this copies it out into the fixed capture staging.
            let written = with_host(handle, None, |host| {
                host.spectrum_stream_window().map(|window| {
                    write_spectrum_window(
                        staging,
                        &SpectrumSnapshot {
                            window: window.as_window(),
                            target_kind: metadata.target_kind,
                            sample_rate_hz: metadata.sample_rate_hz,
                            token: metadata.windows,
                        },
                    )
                })
            });
            written.unwrap_or(RESULT_WRONG_STATE)
        })
    })
}

/// Analyze the most recently read managed stream window using the worker-side smoothing history.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_stream_analysis() -> u32 {
    with_spectrum_staging(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        // An instance that owns both halves started the stream before its analysis existed (the
        // audio thread keeps none). Its metadata record is the stream: the core says whether to
        // adopt it, and the adoption has the same checks as an import.
        let metadata = stream_metadata_of(&staging.stream_metadata);
        match SpectrumAnalysis::adoption_needed(staging.analysis.as_ref(), &metadata) {
            Ok(false) => {}
            Ok(true) => {
                if let Err(result) = adopt_imported_stream(&mut staging) {
                    return result;
                }
            }
            Err(error) => return spectrum_result(error),
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
    with_spectrum_staging(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        adopt_imported_stream(&mut staging).map_or_else(|result| result, |()| RESULT_OK)
    })
}

/// Reset managed spectrum analysis history without stopping the native capture stream.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_stream_reset() -> u32 {
    with_spectrum_staging(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        let staging = &mut *staging;
        let Some(analysis) = staging.analysis.as_mut() else {
            return RESULT_WRONG_STATE;
        };
        if let Err(error) = analysis.reset_stream() {
            return spectrum_result(error);
        }
        staging.result_len = 0;
        staging.stream_metadata.analysis_epoch = analysis.analysis_epoch();
        staging.stream_metadata.history_start_sample = analysis.history_start_sample();
        RESULT_OK
    })
}

/// Stop the managed native stream and retain its final normalized profile in metadata.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_stream_stop(handle: u32) -> u32 {
    render_locked(|| {
        if let Err(result) = verify_live_host(handle) {
            return result;
        }
        with_spectrum_staging(|slot| {
            let Ok(mut staging) = slot.try_borrow_mut() else {
                return RESULT_INTERNAL;
            };
            let staging = &mut *staging;
            let result = with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
                host.stop_spectrum_stream(staging.analysis.as_mut())
            });
            if result == RESULT_OK {
                staging.capture_len = 0;
                staging.result_len = 0;
                let metadata = with_host(handle, None, |host| {
                    host.spectrum_stream_metadata().copied()
                });
                if let Some(metadata) = metadata {
                    write_stream_metadata(&mut staging.stream_metadata, &metadata);
                }
                staging.stream_metadata.result = RESULT_OK;
            } else {
                staging.stream_metadata.result = result;
            }
            result
        })
    })
}

/// Return the fixed managed-stream metadata record address.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_stream_metadata_ptr() -> u32 {
    render_locked(|| {
        with_spectrum_staging(|slot| {
            let Ok(mut staging) = slot.try_borrow_mut() else {
                return 0;
            };
            staging.stream_metadata.struct_size = SPECTRUM_STREAM_METADATA_BYTES;
            staging.stream_metadata.abi_version = ABI_VERSION;
            pointer_u32(ptr::from_mut(&mut staging.stream_metadata))
        })
    })
}

/// Return the fixed managed-stream metadata record byte size.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_stream_metadata_bytes() -> u32 {
    render_locked(|| SPECTRUM_STREAM_METADATA_BYTES)
}

/// Cancel the prepared spectrum observer and discard any completed window.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_cancel(handle: u32) -> u32 {
    render_locked(|| {
        if let Err(result) = verify_live_host(handle) {
            return result;
        }
        with_spectrum_staging(|slot| {
            let Ok(mut staging) = slot.try_borrow_mut() else {
                return RESULT_INTERNAL;
            };
            let result = with_host_mut(
                handle,
                RESULT_INVALID_ARGUMENT,
                AudioWorkletEngineHost::cancel_spectrum,
            );
            if result == RESULT_WRONG_STATE {
                // A running stream refuses the cancel and keeps its staged window.
                staging.stream_metadata.result = RESULT_WRONG_STATE;
            } else {
                staging.capture_len = 0;
            }
            result
        })
    })
}

/// Return a writable request header for one live selected-track response query.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_request_ptr() -> u32 {
    render_locked(|| {
        with_response_staging(|slot| {
            let Ok(mut staging) = slot.try_borrow_mut() else {
                return 0;
            };
            staging.live_result_len = 0;
            staging.live_request.struct_size = LIVE_RESPONSE_REQUEST_BYTES;
            staging.live_request.abi_version = ABI_VERSION;
            pointer_u32(ptr::from_mut(&mut *staging.live_request))
        })
    })
}

/// Return the fixed live response request-header byte size.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_request_bytes() -> u32 {
    render_locked(|| LIVE_RESPONSE_REQUEST_BYTES)
}

/// Return the fixed raw snapshot staging address used by capture and analysis.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_snapshot_ptr() -> u32 {
    render_locked(|| {
        with_response_staging(|slot| {
            slot.try_borrow()
                .ok()
                .map_or(0, |staging| pointer_u32(staging.live_result.as_ptr()))
        })
    })
}

/// Return the maximum byte length of the fixed raw snapshot staging area.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_snapshot_capacity() -> u32 {
    render_locked(|| u32::try_from(LIVE_RESPONSE_CAPTURE_BYTES).unwrap_or(0))
}

/// Set the byte length of a raw snapshot copied into the fixed staging area.
///
/// The analysis Worker copies the Worklet's capture reply into this same staging area before
/// calling [`miso_engine_web_v1_track_response_analysis`].  The length is explicit so malformed
/// or truncated replies are rejected by the Rust decoder rather than inferred from stale bytes.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_snapshot_set_bytes(bytes: u32) -> u32 {
    with_response_staging(|slot| {
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
    render_locked(|| {
        with_response_staging(|slot| {
            let Ok(mut staging) = slot.try_borrow_mut() else {
                return 0;
            };
            pointer_u32(staging.live_track_id.as_mut_ptr())
        })
    })
}

/// Return the maximum selected-track identity byte length.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_track_id_capacity() -> u32 {
    render_locked(|| LIVE_RESPONSE_MAXIMUM_ID_BYTES as u32)
}

/// Capture one selected track's live response at the current boundary into raw staging.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_capture(handle: u32) -> u32 {
    render_locked(|| {
        LIVE_HOST.with(|host_slot| {
            let Ok(mut live) = host_slot.try_borrow_mut() else {
                return RESULT_INTERNAL;
            };
            let Some(live) = live.as_mut().filter(|value| value.handle == handle) else {
                return RESULT_WRONG_STATE;
            };
            with_response_staging(|staging_slot| {
                let Ok(mut staging) = staging_slot.try_borrow_mut() else {
                    return RESULT_INTERNAL;
                };
                run_live_response_capture(&mut live.host, &mut staging)
            })
        })
    })
}

/// Analyze one staged raw live-response snapshot using the native Rust response composer.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_analysis() -> u32 {
    with_response_staging(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        run_live_response_analysis(&mut staging)
    })
}

/// Return the live response result payload address.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_result_ptr() -> u32 {
    render_locked(|| {
        with_response_staging(|slot| {
            slot.try_borrow()
                .ok()
                .map_or(0, |staging| pointer_u32(staging.live_result.as_ptr()))
        })
    })
}

/// Return the live response result payload byte length.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_result_bytes() -> u32 {
    render_locked(|| {
        with_response_staging(|slot| {
            slot.try_borrow()
                .ok()
                .and_then(|staging| u32::try_from(staging.live_result_len).ok())
                .unwrap_or(0)
        })
    })
}

/// Release the live response staging payload.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_track_response_close() -> u32 {
    with_response_staging(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        staging.live_result_len = 0;
        RESULT_OK
    })
}

/// Return the frozen browser-Wasm ABI version.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_abi_version() -> u32 {
    ABI_VERSION
}

/// Return how many allocator calls this instance made inside render-locked windows (issue #1333
/// D3). Every export the AudioWorklet calls on its render thread after boot runs in such a
/// window, so browser qualification asserts this is exactly zero.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_render_allocation_count() -> u32 {
    render_allocation_count()
}

/// Return how many allocator calls this instance made outside render-locked windows (issue #1381a,
/// #1332 D1): its boot, its control messages and its service steps. In `single` mode, a page
/// without cross-origin isolation, the worklet's message handler runs the control work, so this is
/// the count of the one place the engine allocates on an audio thread; the host reports it as
/// `singleModeControlAllocations`. Control side only, never `process()`.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_control_allocation_count() -> u32 {
    control_allocation_count()
}

/// Return the module-owned, zero-default boot-options address.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_boot_options_ptr() -> u32 {
    BOOT_STAGING.with(|staging| {
        let Ok(mut staging) = staging.try_borrow_mut() else {
            return 0;
        };
        pointer_u32(ptr::from_mut(&mut staging.options))
    })
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
        *staging.document = document;
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

fn boot_staged(len: u32, spectrum_hop: Option<SpectrumHop>) -> u32 {
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

        let spectrum_request = with_spectrum_staging(|spectrum| {
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
        let result = spectrum_request.and_then(|request| {
            AudioWorkletEngineHost::boot_with_spectrum_config(
                &staging.document,
                staging.options,
                request,
                spectrum_hop,
            )
        });

        match result {
            Ok(host) => {
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
        with_spectrum_staging(|slot| {
            if let Ok(mut staging) = slot.try_borrow_mut() {
                staging.release_capture();
            }
        });
        return 0;
    };
    let mut host = Some(host);
    // A publication refusal is a real boot failure: the prepared host is disposed off the render
    // path and the staged capture is released, exactly as for a refused preparation.
    let handle = LIVE_HOST.with(|live_slot| {
        let Ok(mut live_slot) = live_slot.try_borrow_mut() else {
            return None;
        };
        if live_slot.is_some() {
            return None;
        }
        let handle = next_handle();
        **live_slot = Some(LiveHost {
            handle,
            host: Box::new(
                host.take()
                    .expect("boot host is present before publication"),
            ),
        });
        Some(handle)
    });
    let Some(handle) = handle else {
        if let Some(mut host) = host.take() {
            let _ = host.dispose();
        }
        BOOT_STAGING.with(|staging| {
            if let Ok(mut staging) = staging.try_borrow_mut() {
                staging.record_failure(BootFailure::fixed(RESULT_INTERNAL, "web.host.publication"));
            }
        });
        with_spectrum_staging(|slot| {
            if let Ok(mut staging) = slot.try_borrow_mut() {
                staging.release_capture();
            }
        });
        return 0;
    };
    reserve_stagings();
    handle
}

fn reject_explicit_spectrum_hop() -> u32 {
    BOOT_STAGING.with(|staging| {
        if let Ok(mut staging) = staging.try_borrow_mut() {
            staging.record_failure(BootFailure::fixed(
                RESULT_INVALID_ARGUMENT,
                "web.spectrum.hop",
            ));
        }
    });
    0
}

fn boot_staged_with_spectrum_hop(len: u32, hop_frames: u32) -> u32 {
    let Ok(spectrum_hop) = SpectrumHop::new(hop_frames) else {
        return reject_explicit_spectrum_hop();
    };
    boot_staged(len, Some(spectrum_hop))
}

/// Return the capability value for explicit prepared spectrum-hop boot.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_hop_capability() -> u32 {
    1
}

/// Boot the exact staged document and atomically publish the sole running handle.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_boot(len: u32) -> u32 {
    boot_staged(len, None)
}

/// Boot the exact staged document with an explicit ordinary spectrum capture hop in frames.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_boot_with_spectrum_hop(len: u32, hop_frames: u32) -> u32 {
    boot_staged_with_spectrum_hop(len, hop_frames)
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
    render_locked(|| {
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
                // SAFETY: Preparation allocated `SOURCE_CHANNELS_LAUNCH_MAXIMUM * quantum` stable PCM
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
    render_locked(|| {
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
    })
}

/// Render one exact prepared quantum after validating the browser's actual frame count.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_render(handle: u32, actual_frames: u32) -> u32 {
    render_locked(|| {
        with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
            if host.status().state == STATE_READY && host.status().quantum_frames != actual_frames {
                return host.reject_output_quantum(actual_frames);
            }
            host.render_next()
        })
    })
}

/// Run a host operation that can promote an adopted plan's spectrum captures (`service`, and the
/// reclaim every `control_command` starts with), with the staging's analysis for the promotion to
/// end (#1618). A promotion that does not keep the stream ends the analysis with it; the staging's
/// stream record and staged window then describe a stream that no longer exists, so the record is
/// written from the service and the window is dropped, as `spectrum_stream_stop` does. Without
/// that, `spectrum_stream_analysis` would adopt the stale record again and re-report the window.
/// A staging that is borrowed has no analysis running: the operation gets `None`.
fn with_promotion_analysis<R>(
    host: &mut AudioWorkletEngineHost,
    operation: impl FnOnce(&mut AudioWorkletEngineHost, Option<&mut SpectrumAnalysis>) -> R,
) -> R {
    with_spectrum_staging(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return operation(host, None);
        };
        let staging = &mut *staging;
        let was_running = staging
            .analysis
            .as_ref()
            .is_some_and(SpectrumAnalysis::stream_active);
        let result = operation(host, staging.analysis.as_mut());
        let running = staging
            .analysis
            .as_ref()
            .is_some_and(SpectrumAnalysis::stream_active);
        if was_running && !running {
            staging.capture_len = 0;
            staging.result_len = 0;
            if let Some(metadata) = host.spectrum_stream_metadata().copied() {
                write_stream_metadata(&mut staging.stream_metadata, &metadata);
            }
            staging.stream_metadata.result = RESULT_OK;
        }
        result
    })
}

/// Run the control plane's service step once (issue #1381a D5, #1348 D1): reclaim every plan and
/// attachment render has displaced, in the caller's thread and never in render. Control side
/// only: the control Worker, or in `single` mode the worklet's message handler, never `process()`.
///
/// It commits and acknowledges nothing and leaves the status's last result alone. Results:
/// [`RESULT_OK`]; [`RESULT_INVALID_ARGUMENT`] for a wrong handle; [`RESULT_WRONG_STATE`] for a host
/// that is not ready; an epoch failure as [`RESULT_INTERNAL`] with the session's diagnostic.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_service(handle: u32) -> u32 {
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        with_promotion_analysis(host, AudioWorkletEngineHost::service)
    })
}

/// Return the control request staging address for a frame of `len` bytes (issue #1562a D1), or
/// zero for an invalid handle, a host that is not ready, or a `len` that is zero or over the boot's
/// `maximum_control_frame_bytes`. The page writes one command frame there, then calls
/// [`miso_engine_web_v1_control_command`]. Control side only.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_control_request_ptr(handle: u32, len: u32) -> u32 {
    with_host_mut(handle, 0, |host| {
        host.control_request_mut(len)
            .map_or(0, |staging| pointer_u32(staging.as_ptr()))
    })
}

/// Run the staged control command frame of `len` bytes through the control plane's
/// `SessionState::command`, the entry the C ABI's `miso_engine_v1_submit_command` runs (issue
/// #1562a D1). In `single` mode it runs in the worklet's message handler, never in `process()`; it
/// is not render-locked, so its allocations are counted as the instance's control allocations.
///
/// Results: [`RESULT_OK`] (the response frame is at [`miso_engine_web_v1_control_response_ptr`],
/// and a refused transaction is still an ok call with a non-ok frame); [`RESULT_INVALID_ARGUMENT`]
/// for a wrong handle, a bad length or a frame the core cannot correlate;
/// [`RESULT_WRONG_STATE`]; [`RESULT_BUFFER_TOO_SMALL`]; [`RESULT_BACKPRESSURE`];
/// [`crate::RESULT_COMPILE_REJECTED`] for a typed refusal before commit (`web.apply.shape_unavailable`);
/// [`RESULT_INTERNAL`]. After any failure the response is the refusal's diagnostic.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_control_command(handle: u32, len: u32) -> u32 {
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        with_promotion_analysis(host, |host, analysis| host.control_command(len, analysis))
    })
}

/// Dequeue the next event of `lane` through the control plane's `SessionState::dequeue_event`, the
/// entry the C ABI's `miso_engine_v1_dequeue_event` runs (issue #1570b D1). `lane` is `0` for the
/// reliable lane and `1` for the lossy lane. On [`RESULT_OK`] the event frame is at
/// [`miso_engine_web_v1_control_response_ptr`] with [`miso_engine_web_v1_control_response_bytes`]
/// bytes, and zero bytes means the lane is empty. The staging is shared with
/// [`miso_engine_web_v1_control_command`]: the frame is valid until the next `control_command` or
/// `control_event` call. A Wasm caller cannot hand the module an output buffer, so the pointer
/// exports are the transport and the dequeue itself is the core's.
///
/// Control side only: in `single` mode the worklet's message handler, never `process()`. It is not
/// render-locked, so its allocations are control allocations, and it leaves the status's last
/// result alone.
///
/// Results: [`RESULT_OK`]; [`RESULT_INVALID_ARGUMENT`] for a wrong handle or a lane other than `0`
/// or `1` (nothing is dequeued); [`RESULT_REFUSED_LIFECYCLE`] for a host with no control half;
/// [`RESULT_BACKPRESSURE`], [`RESULT_BUFFER_TOO_SMALL`] or [`RESULT_INTERNAL`] for the core's
/// matching event refusal, whose text is then the session diagnostic.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_control_event(handle: u32, lane: u32) -> u32 {
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        host.control_event(lane)
    })
}

/// Return the address of the last control command's response (issue #1562a D1), valid until the
/// next control call, or zero for an invalid handle or a host with nothing to read.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_control_response_ptr(handle: u32) -> u32 {
    with_host(handle, 0, |host| {
        let response = host.control_response();
        if response.is_empty() {
            0
        } else {
            pointer_u32(response.as_ptr())
        }
    })
}

/// Return the byte length of the last control command's response, zero for an invalid handle.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_control_response_bytes(handle: u32) -> u32 {
    with_host(handle, 0, |host| {
        u32::try_from(host.control_response().len()).unwrap_or(0)
    })
}

/// Return the committed session revision (issue #1562a, carry-forward 2), zero for an invalid
/// handle or a session never edited. The SDK's transaction queue seeds its expected revision from
/// it after a successful boot.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_control_revision(handle: u32) -> u64 {
    with_host(handle, 0, AudioWorkletEngineHost::committed_revision)
}

/// Copy the plan's applied-revision watermark into its fixed record (issue #1381a D5), found at
/// [`miso_engine_web_v1_watermark_ptr`]. Control side only.
///
/// Results: [`RESULT_OK`]; [`RESULT_INVALID_ARGUMENT`] for a wrong handle, which writes nothing;
/// [`RESULT_WRONG_STATE`] for a host that is not ready; [`RESULT_BACKPRESSURE`] for a read that
/// gave up after the watermark's bounded attempts, which leaves the record untouched: a read to
/// retry, never a refused edit.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_watermark_read(handle: u32) -> u32 {
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        host.read_watermark()
    })
}

/// Return the stable address of the 96-byte watermark record or zero for an invalid handle.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_watermark_ptr(handle: u32) -> u32 {
    with_host(handle, 0, |host| {
        pointer_u32(ptr::from_ref(host.watermark()))
    })
}

/// Return the stable prepared resource-report address or zero for an invalid handle.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_resource_ptr(handle: u32) -> u32 {
    with_host(handle, 0, |host| {
        pointer_u32(ptr::from_ref(host.resources()))
    })
}

/// Admit one staged observation command submission, kinds 7 and 8 (issue #137 D1; #1565A).
///
/// `count` records were written into [`BUFFER_COMMAND`], which exists when the boot asked for
/// observation taps. Every value goes through the control exports instead. The submission is one
/// transaction: the return value is the frozen result code, and
/// [`miso_engine_web_v1_command_report_ptr`] names the first refused record and why.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_command_submit(handle: u32, count: u32) -> u32 {
    render_locked(|| {
        with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
            host.submit_commands(count)
        })
    })
}

/// Return the stable live-control command-report address or zero for an invalid handle.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_command_report_ptr(handle: u32) -> u32 {
    with_host(handle, 0, |host| {
        pointer_u32(ptr::from_ref(host.command_report()))
    })
}

/// Take (`1`) or release (`0`) the decimated meter lease (issue #137 D2).
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_meter_lease(handle: u32, enabled: u32) -> u32 {
    render_locked(|| {
        if let Err(result) = verify_live_host(handle) {
            return result;
        }
        with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
            if enabled > 1 {
                return host.record_boundary_result(RESULT_INVALID_ARGUMENT);
            }
            host.set_meter_lease(enabled == 1)
        })
    })
}

/// Drain finished meter windows into the frame buffer; returns the number of complete windows.
///
/// Called from `process()` after the render export, so it is allocation-free and bounded: it moves
/// `Copy` snapshots out of queues sized at compilation into a buffer sized at compilation.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_meter_poll(handle: u32) -> u32 {
    render_locked(|| with_host_mut(handle, 0, AudioWorkletEngineHost::poll_meters))
}

/// Return the address of the header that describes the plan now rendering, or zero for an invalid
/// handle (issue #143 D5). The address changes at a plan swap (issue #1290d1: each plan carries its
/// own header), so a page reads it again after a swap or follows `successor_pointer`.
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

/// Return the number of prepared resident observation effects in the current owner map.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_count(handle: u32) -> u32 {
    render_locked(|| {
        with_host(handle, 0, |host| {
            u32::try_from(host.observation_binding_count()).unwrap_or(0)
        })
    })
}

/// Return the stable observation-ID staging address used by binding introspection.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_id_ptr() -> u32 {
    render_locked(|| {
        with_observation_staging(|slot| {
            let Ok(mut staging) = slot.try_borrow_mut() else {
                return 0;
            };
            pointer_u32(staging.id.as_mut_ptr())
        })
    })
}

/// Return the maximum stable observation-ID staging length.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_id_capacity() -> u32 {
    render_locked(|| OBSERVATION_MAXIMUM_ID_BYTES)
}

fn copy_observation_id(handle: u32, index: u32, native: bool) -> u32 {
    with_observation_staging(|slot| {
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
    render_locked(|| copy_observation_id(handle, index, false))
}

/// Copy one bound observation effect's native contract ID into the observation-ID staging area.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_native_effect_id(handle: u32, index: u32) -> u32 {
    render_locked(|| copy_observation_id(handle, index, true))
}

/// One bound observation effect's strip index (tracks, then submixes), or zero when out of range.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_track_index(handle: u32, index: u32) -> u32 {
    render_locked(|| {
        with_host(handle, 0, |host| {
            host.observation_binding(index)
                .map_or(0, |binding| binding.address.track_index)
        })
    })
}

/// Return one bound observation effect's rack (`1` inserts or `3` console), or zero when out of
/// range. Zero is the retired `simd1` code, so it names no rack (issue #1096).
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_rack(handle: u32, index: u32) -> u32 {
    render_locked(|| {
        with_host(handle, 0, |host| {
            host.observation_binding(index)
                .map_or(0, |binding| observation_rack_raw(binding.address.rack))
        })
    })
}

/// Return one bound observation effect's index within its rack, or zero when out of range: a
/// console slot's index in the session's slot order, or an insert's index.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_effect_index(handle: u32, index: u32) -> u32 {
    render_locked(|| {
        with_host(handle, 0, |host| {
            host.observation_binding(index)
                .map_or(0, |binding| binding.address.effect_index)
        })
    })
}

/// Return one bound observation effect's declared tap count, or zero when out of range.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_tap_count(handle: u32, index: u32) -> u32 {
    render_locked(|| {
        with_host(handle, 0, |host| {
            host.observation_binding(index)
                .and_then(|binding| u32::try_from(binding.descriptors.len()).ok())
                .unwrap_or(0)
        })
    })
}

/// Return one bound observation effect's declared tap ID, or zero when out of range.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_tap_id(
    handle: u32,
    index: u32,
    tap_index: u32,
) -> u32 {
    render_locked(|| {
        with_host(handle, 0, |host| {
            host.observation_binding(index)
                .and_then(|binding| binding.descriptors.get(tap_index as usize))
                .map_or(0, |tap| tap.id.0)
        })
    })
}

/// Return a writable bounded selected-observation input staging address.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_selection_ptr() -> u32 {
    render_locked(|| {
        with_observation_staging(|slot| {
            let Ok(mut staging) = slot.try_borrow_mut() else {
                return 0;
            };
            pointer_u32(staging.selections.as_mut_ptr())
        })
    })
}

/// Return the fixed selected-observation input record size.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_selection_bytes() -> u32 {
    render_locked(|| OBSERVATION_SELECTION_BYTES)
}

/// Return the maximum selected-observation input record count.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_selection_capacity() -> u32 {
    render_locked(|| MAXIMUM_OBSERVATION_READS as u32)
}

/// Read one complete bounded batch of selected resident observations.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_read(handle: u32, count: u32) -> u32 {
    render_locked(|| {
        if let Err(result) = verify_live_host(handle) {
            return result;
        }
        with_observation_staging(|slot| {
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
                with_host_mut(
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
    render_locked(|| {
        with_observation_staging(|slot| {
            let Ok(staging) = slot.try_borrow() else {
                return 0;
            };
            pointer_u32(staging.results.as_ptr())
        })
    })
}

/// Return the selected-observation result byte length.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_observation_result_bytes() -> u32 {
    render_locked(|| {
        with_observation_staging(|slot| {
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
    render_locked(|| {
        with_host(handle, 0, |host| {
            u32::try_from(host.session_source_count()).unwrap_or(0)
        })
    })
}

/// Copy one canonical source ID into the ID staging buffer; returns its byte length.
///
/// Zero means "no such source". [`BUFFER_SOURCE_ID`] is sized for the longest ID the grammar
/// allows, so no session a rebuild commits outgrows it.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_source_id(handle: u32, index: u32) -> u32 {
    render_locked(|| with_host_mut(handle, 0, |host| host.copy_session_source_id(index)))
}

/// Return one source's declared channel count, or zero for an out-of-range index.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_source_channels(handle: u32, index: u32) -> u32 {
    render_locked(|| {
        with_host(handle, 0, |host| {
            host.session_source_shape(index)
                .map_or(0, |shape| shape.channel_count)
        })
    })
}

/// Return one source's exact length in source sample frames, or zero out of range.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_source_frames(handle: u32, index: u32) -> u64 {
    render_locked(|| {
        with_host(handle, 0, |host| {
            host.session_source_shape(index)
                .map_or(0, |shape| shape.frames)
        })
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
    // including its native ownership.
    with_spectrum_staging(|spectrum_slot| {
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
                let result = live.host.dispose();
                drop(live);
                spectrum.release_capture();
                boot.reset_after_dispose();
                result
            })
        })
    })
}

/// Native writers for the staging the `u32` pointer exports address on `wasm32`.
///
/// On a 64-bit host a staging address does not fit the exports' `u32`, so the pointer exports
/// return zero there. host-web's own integration binaries (issue #1333 gate 8) drive the boundary
/// through the exports and stage requests through these writers, which store exactly what the
/// worklet writes through the exports' addresses. None of them is render-locked: the worklet never
/// calls them. Not built for wasm.
#[cfg(not(target_family = "wasm"))]
#[doc(hidden)]
pub mod native_staging {
    use super::{
        WebBootOptions, WebLiveResponseRequest, WebObservationSelection,
        WebSpectrumCollectionEntry, WebSpectrumCollectionRequest, WebSpectrumRequest,
    };

    /// Stage the boot options and the exact document, as `miso_engine_web_v1_boot_options_ptr`
    /// and `miso_engine_web_v1_document_ptr` let the worklet do. Returns the staged length for the
    /// boot export, or `None` when the document was refused.
    pub fn boot(options: WebBootOptions, document: &[u8]) -> Option<u32> {
        let length = u32::try_from(document.len()).ok()?;
        super::BOOT_STAGING.with(|staging| staging.borrow_mut().options = options);
        let _ = super::miso_engine_web_v1_document_ptr(length);
        super::BOOT_STAGING.with(|staging| {
            let mut staging = staging.borrow_mut();
            if !staging.document_valid || staging.document.len() != document.len() {
                return None;
            }
            staging.document.copy_from_slice(document);
            Some(length)
        })
    }

    /// Stage the pre-boot spectrum request and its target identity.
    pub fn spectrum_request(request: WebSpectrumRequest, target_id: &[u8]) {
        super::with_spectrum_staging(|slot| {
            let mut staging = slot.borrow_mut();
            staging.target_id.fill(0);
            staging.target_id[..target_id.len()].copy_from_slice(target_id);
            *staging.request = request;
        });
    }

    /// Stage a pre-boot spectrum collection: its header, entries and packed target identities,
    /// in the order the worklet's `stageSpectrumCollectionRequest` writes them. Each pointer
    /// export sizes its staging as it does for the worklet (its returned address does not fit a
    /// native `u32`); this writer then stores what the worklet would write through that address.
    /// Returns `false` when an export did not size its staging to the request.
    pub fn spectrum_collection(
        request: WebSpectrumCollectionRequest,
        entries: &[WebSpectrumCollectionEntry],
        target_ids: &[u8],
    ) -> bool {
        let _ = super::miso_engine_web_v1_spectrum_collection_request_ptr();
        super::with_spectrum_staging(|slot| *slot.borrow_mut().collection_request = request);
        let _ = super::miso_engine_web_v1_spectrum_collection_entry_ptr();
        let staged = super::with_spectrum_staging(|slot| {
            let mut staging = slot.borrow_mut();
            if staging.collection_entries.len() != entries.len() {
                return false;
            }
            staging.collection_entries.copy_from_slice(entries);
            true
        });
        if !staged {
            return false;
        }
        let _ = super::miso_engine_web_v1_spectrum_collection_target_ids_ptr();
        super::with_spectrum_staging(|slot| {
            let mut staging = slot.borrow_mut();
            if staging.collection_target_ids.len() != target_ids.len() {
                return false;
            }
            staging.collection_target_ids.copy_from_slice(target_ids);
            true
        })
    }

    /// Stage the target identity a post-boot `spectrum_select` reads, as the worklet writes it
    /// through `miso_engine_web_v1_spectrum_target_id_ptr`.
    pub fn spectrum_target_id(target_id: &[u8]) {
        super::with_spectrum_staging(|slot| {
            let mut staging = slot.borrow_mut();
            staging.target_id.fill(0);
            staging.target_id[..target_id.len()].copy_from_slice(target_id);
        });
    }

    /// Stage the selected-observation records an `observation_read` will read.
    pub fn observation_selections(selections: &[WebObservationSelection]) {
        super::with_observation_staging(|slot| {
            slot.borrow_mut().selections[..selections.len()].copy_from_slice(selections);
        });
    }

    /// Stage the live track-response request and its track identity.
    pub fn track_response_request(request: WebLiveResponseRequest, track_id: &[u8]) {
        super::with_response_staging(|slot| {
            let mut staging = slot.borrow_mut();
            staging.live_track_id.fill(0);
            staging.live_track_id[..track_id.len()].copy_from_slice(track_id);
            *staging.live_request = request;
        });
    }

    /// Stage a source identity, as the worklet writes it through
    /// `miso_engine_web_v1_buffer_ptr(handle, BUFFER_SOURCE_ID)` before `source_submit` or
    /// `source_seek`. Returns `false` when the handle owns no staging that fits it.
    pub fn source_id(handle: u32, id: &[u8]) -> bool {
        super::with_host_mut(handle, false, |host| {
            host.source_id_mut()
                .and_then(|staging| staging.get_mut(..id.len()))
                .map(|staging| staging.copy_from_slice(id))
                .is_some()
        })
    }

    /// Read back the first `length` bytes of the source-ID staging, as the worklet reads them after
    /// `miso_engine_web_v1_source_id` returns that length. `None` when the handle owns no staging
    /// or `length` exceeds it.
    pub fn staged_source_id(handle: u32, length: u32) -> Option<Vec<u8>> {
        super::with_host_mut(handle, None, |host| {
            host.source_id_mut()
                .and_then(|staging| staging.get(..length as usize))
                .map(<[u8]>::to_vec)
        })
    }

    /// Fill the planar source-PCM staging with `value`, as the worklet copies a chunk's planes
    /// through `miso_engine_web_v1_buffer_ptr(handle, BUFFER_SOURCE_PCM)`.
    pub fn source_pcm(handle: u32, value: f32) -> bool {
        super::with_host_mut(handle, false, |host| {
            host.source_pcm_mut().map(|pcm| pcm.fill(value)).is_some()
        })
    }

    /// Stage one control frame as the worklet does through `miso_engine_web_v1_control_request_ptr`,
    /// run `miso_engine_web_v1_control_command` and read the response through the response
    /// exports' record (issue #1562a D1). Returns the export's result and the response bytes (the
    /// response frame after an ok result, else the refusal's diagnostic), or `None` for a handle
    /// or a frame length the staging refuses.
    pub fn control_command(handle: u32, frame: &[u8]) -> Option<(u32, Vec<u8>)> {
        let length = u32::try_from(frame.len()).ok()?;
        let staged = super::with_host_mut(handle, false, |host| {
            host.control_request_mut(length)
                .map(|staging| staging[..frame.len()].copy_from_slice(frame))
                .is_some()
        });
        if !staged {
            return None;
        }
        let result = super::miso_engine_web_v1_control_command(handle, length);
        let response =
            super::with_host(handle, Vec::new(), |host| host.control_response().to_vec());
        Some((result, response))
    }

    /// Run `miso_engine_web_v1_control_event` on `lane` and read the frame through the response
    /// exports' record (issue #1570b D1), as the worklet's drain does. Returns the export's result
    /// and the event frame (empty for an empty lane or a refusal).
    pub fn control_event(handle: u32, lane: u32) -> (u32, Vec<u8>) {
        let result = super::miso_engine_web_v1_control_event(handle, lane);
        let frame = if result == super::RESULT_OK {
            super::with_host(handle, Vec::new(), |host| host.control_response().to_vec())
        } else {
            Vec::new()
        };
        (result, frame)
    }

    /// How many blocking rebuilds the host `handle` names counted (#1290a D6); `None` for an
    /// unknown handle.
    #[cfg(feature = "test-support")]
    pub fn blocking_rebuilds(handle: u32) -> Option<u32> {
        super::with_host(handle, None, |host| Some(host.blocking_rebuilds()))
    }

    /// The epoch of the plan that renders, for the host `handle` names (#1381a gates 2 and 4).
    #[cfg(feature = "test-support")]
    pub fn active_plan_epoch(handle: u32) -> Option<u64> {
        super::with_host(handle, None, |host| host.active_plan_epoch_for_test())
    }

    /// The status record of the host `handle` names; `None` for an unknown handle.
    pub fn status(handle: u32) -> Option<crate::WebStatus> {
        super::with_host(handle, None, |host| Some(*host.status()))
    }

    /// The output planes (left, then right) the host `handle` names rendered last, as the worklet
    /// reads them through `miso_engine_web_v1_buffer_ptr(handle, BUFFER_OUTPUT_PCM)`.
    pub fn output_pcm(handle: u32) -> Option<Vec<f32>> {
        super::with_host(handle, None, |host| host.output_pcm().map(<[f32]>::to_vec))
    }

    /// The watermark record the host `handle` names last copied (#1381a gate 5), as the worklet
    /// reads it through `miso_engine_web_v1_watermark_ptr`; `None` for an unknown handle.
    pub fn watermark(handle: u32) -> Option<crate::WebPlanWatermark> {
        super::with_host(handle, None, |host| Some(*host.watermark()))
    }

    /// The meter header and frame the host `handle` names last published, as the worklet reads
    /// them through `miso_engine_web_v1_meter_header_ptr` and the header's frame pointer (#1290d1);
    /// `None` for an unknown handle.
    pub fn meter_frame(handle: u32) -> Option<(crate::WebMeterHeader, Vec<f32>)> {
        super::with_host(handle, None, |host| {
            Some((*host.meter_header(), host.meter_frame().to_vec()))
        })
    }

    /// Stage live-control command records, as the worklet writes them through
    /// `miso_engine_web_v1_buffer_ptr(handle, BUFFER_COMMAND)`.
    pub fn command_records(handle: u32, records: &[u8]) -> bool {
        super::with_host_mut(handle, false, |host| {
            host.command_staging_mut()
                .and_then(|staging| staging.get_mut(..records.len()))
                .map(|staging| staging.copy_from_slice(records))
                .is_some()
        })
    }
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
    BOOT_STAGING.with(|staging| staging.borrow_mut().options = options);
    test_stage_document(bytes);
    miso_engine_web_v1_boot(bytes.len() as u32)
}

#[cfg(test)]
pub(crate) fn test_staged_document() -> Vec<u8> {
    BOOT_STAGING.with(|staging| Vec::clone(&staging.borrow().document))
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
mod response_frame_tests {
    use super::*;
    use protocol::{
        CommandPayload, DecodeScratch, DecodedTypedResponseFrame, ExpectedRevision, ProtocolCodec,
        RequestId, StatusCode, TypedCommandFrame,
    };

    /// #1574: the session-less response entry stages one frame, answers one frame, and frees
    /// both. A frame with no command header is a typed refusal with no reply, and a size outside
    /// the staging bound is refused before any allocation.
    #[test]
    fn the_response_frame_exports_stage_answer_and_close() {
        assert_eq!(miso_engine_web_v1_response_frame_ptr(0), 0);
        assert_eq!(miso_engine_web_v1_response_frame_ptr((1 << 20) + 1), 0);

        // A staged request that is not a frame leaves no reply. (A native address does not fit
        // the `u32` the export returns, so the staging is read directly.)
        let _ = miso_engine_web_v1_response_frame_ptr(10);
        assert_eq!(
            with_response_frame_staging(|slot| slot.borrow().request.len()),
            10
        );
        assert_eq!(
            miso_engine_web_v1_response_frame_submit(10),
            RESULT_INVALID_ARGUMENT
        );
        assert_eq!(miso_engine_web_v1_response_frame_reply_bytes(), 0);

        // A `CAPABILITIES_GET` frame is a command this entry does not serve: a non-OK reply.
        let mut request = vec![0_u8; 256];
        let length = ProtocolCodec::default()
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(7).expect("request ID"),
                    expected_revision: ExpectedRevision::Any,
                    payload: CommandPayload::CapabilitiesGet,
                },
                &mut request,
            )
            .expect("command frame");
        let _ = miso_engine_web_v1_response_frame_ptr(u32::try_from(length).expect("length"));
        with_response_frame_staging(|slot| {
            slot.borrow_mut()
                .request
                .copy_from_slice(&request[..length]);
        });
        assert_eq!(
            miso_engine_web_v1_response_frame_submit(u32::try_from(length).expect("length")),
            RESULT_OK
        );
        let reply = with_response_frame_staging(|slot| slot.borrow().reply.clone());
        assert_eq!(
            miso_engine_web_v1_response_frame_reply_bytes() as usize,
            reply.len()
        );
        let mut fields = [0_u16; 64];
        let DecodedTypedResponseFrame::NonOk { header, .. } = ProtocolCodec::default()
            .decode_typed_response(&reply, &mut DecodeScratch::new(&mut fields))
            .expect("a typed response")
        else {
            panic!("a refusal");
        };
        assert_eq!(header.status, StatusCode::UnsupportedMessage);
        assert_eq!(header.request_id.get(), 7);

        assert_eq!(miso_engine_web_v1_response_frame_close(), RESULT_OK);
        assert_eq!(miso_engine_web_v1_response_frame_reply_bytes(), 0);
    }
}

#[cfg(test)]
pub(crate) mod live_response_ffi_tests {
    use super::*;
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
        with_response_staging(|slot| {
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
        with_response_staging(|slot| slot.borrow().live_result_header)
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
            with_response_staging(|slot| slot.borrow().live_token),
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

        let expected_result_bytes = with_response_staging(|slot| {
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

        let (snapshot, token) = with_response_staging(|slot| {
            parse_live_snapshot(&slot.borrow()).expect("captured mixed snapshot")
        });
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

        with_response_staging(|slot| {
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
        with_response_staging(|slot| {
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
        with_response_staging(|slot| {
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
        with_response_staging(|slot| {
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

        with_response_staging(|slot| slot.borrow_mut().live_token = u64::MAX);
        stage_request(b"eq0");
        let (exhausted_result, allocations, deallocations) =
            measured(|| miso_engine_web_v1_track_response_capture(handle));
        assert_eq!(exhausted_result, RESULT_REFUSED_BUDGET);
        assert_eq!(allocations, 0, "token exhaustion refusal allocated");
        assert_eq!(deallocations, 0, "token exhaustion refusal freed");
        assert_eq!(captured_header().result, RESULT_REFUSED_BUDGET);
        assert_eq!(
            with_response_staging(|slot| slot.borrow().live_token),
            u64::MAX,
            "exhaustion must not wrap or reuse a snapshot token"
        );
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    }

    /// Byte offset of each owner record's `rack` word in the captured payload, in record order,
    /// with the owner's stable identity.
    fn owner_racks() -> Vec<(String, usize, u32)> {
        with_response_staging(|slot| {
            let staging = slot.borrow();
            let bytes = &staging.live_result[..staging.live_result_len];
            let header: WebLiveResponseResult = read_live_record(bytes, 0).expect("header");
            let owners_start = usize::try_from(header.owners_offset).expect("owners offset");
            (0..usize::try_from(header.owner_count).expect("owner count"))
                .map(|index| {
                    let at = owners_start + index * size_of::<WebLiveResponseOwner>();
                    let raw: WebLiveResponseOwner =
                        read_live_record(bytes, u32::try_from(at).expect("offset"))
                            .expect("owner record");
                    let stable =
                        live_payload_bytes(bytes, raw.stable_id_offset, raw.stable_id_bytes)
                            .expect("stable ID");
                    (
                        String::from_utf8(stable.to_vec()).expect("UTF-8 stable ID"),
                        at + core::mem::offset_of!(WebLiveResponseOwner, rack),
                        raw.rack,
                    )
                })
                .collect()
        })
    }

    /// Decision 12 (issue #1096): the live-response owner record names the input filters `0`, an
    /// insert `2` (the inserts keep the code `dynamic` had) and a console slot `4` in either
    /// section, and the analysis refuses a snapshot whose owner carries the retired `simd1` (`1`)
    /// or `simd2` (`3`) code, or any unallocated one.
    ///
    /// Red mutations: pass the engine's native rack through unchanged (the console owners read `1`
    /// and `3`); or accept any `u8` in `live_response_rack` (a retired code is analysed).
    #[test]
    fn live_response_owners_name_console_slots_inserts_and_input_filters() {
        let mut model = session::parse_session_json(include_str!(
            "../../../fixtures/session/v1/parametric-eq-nine-track.json"
        ))
        .expect("nine-track fixture");
        let mut post = model.console.pre_insert[0].clone();
        post.slot = session::StableId::parse("post-eq").expect("slot id");
        model.console.post_insert = vec![post];
        for track in &mut model.tracks {
            let mut entry = track.console[0].clone();
            entry.slot = session::StableId::parse("post-eq").expect("slot id");
            track.console.push(entry);
        }
        let mut insert = model.lower_track(&model.tracks[0]).pre_insert[0].clone();
        insert.id = session::StableId::parse("ins-eq").expect("insert id");
        model.tracks[0].inserts.effects.push(insert);
        let document = session::canonical_session_json(&model).expect("canonical document");
        let options = WebBootOptions {
            require_sample_rate_hz: 48_000,
            require_quantum_frames: 128,
            ..WebBootOptions::explicit_defaults()
        };
        let handle = test_boot(document.as_bytes(), options);
        assert_ne!(handle, 0, "the console and insert fixture must boot");

        stage_request(b"eq0");
        assert_eq!(miso_engine_web_v1_track_response_capture(handle), RESULT_OK);
        let owners = owner_racks();
        assert_eq!(
            owners
                .iter()
                .map(|(stable, _, rack)| (stable.as_str(), *rack))
                .collect::<Vec<_>>(),
            [
                ("input-filters", crate::LIVE_RESPONSE_RACK_INPUT_FILTERS),
                ("eq", crate::LIVE_RESPONSE_RACK_CONSOLE),
                ("ins-eq", crate::LIVE_RESPONSE_RACK_INSERTS),
                ("post-eq", crate::LIVE_RESPONSE_RACK_CONSOLE),
            ],
            "owners in signal order, each named by its session rack"
        );
        assert_eq!(miso_engine_web_v1_track_response_analysis(), RESULT_OK);

        for refused in [1_u32, 3, 5, 255] {
            stage_request(b"eq0");
            assert_eq!(miso_engine_web_v1_track_response_capture(handle), RESULT_OK);
            let (_, at, _) = owner_racks()[1].clone();
            with_response_staging(|slot| {
                slot.borrow_mut().live_result[at..at + 4].copy_from_slice(&refused.to_le_bytes());
            });
            assert_eq!(
                miso_engine_web_v1_track_response_analysis(),
                RESULT_INVALID_ARGUMENT,
                "an owner record at rack {refused} is refused"
            );
        }
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    }
}

#[cfg(test)]
mod spectrum_ffi_tests {
    use super::*;
    use crate::ffi::observation_alias_tests::no_live_host;
    use crate::{SPECTRUM_STREAM_STATUS_READY, SPECTRUM_STREAM_STATUS_STOPPED};

    #[test]
    fn cold_spectrum_capture_configuration_is_idempotent() {
        with_spectrum_staging(|slot| slot.borrow_mut().release_capture());
        SPECTRUM_CAPTURE_CONFIGURE_ENTRIES.with(|entries| entries.set(0));

        let first_result = with_spectrum_staging(|slot| slot.borrow_mut().configure_capture());
        let first_entry_count = SPECTRUM_CAPTURE_CONFIGURE_ENTRIES.with(|entries| entries.get());
        let capacities = with_spectrum_staging(|slot| {
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
            Some(with_spectrum_staging(|slot| {
                slot.borrow_mut().configure_capture()
            }))
        } else {
            None
        };
        let second_entry_count = SPECTRUM_CAPTURE_CONFIGURE_ENTRIES.with(|entries| entries.get());

        with_spectrum_staging(|slot| slot.borrow_mut().release_capture());
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
    fn spectrum_hop_capability_is_the_pinned_value() {
        assert_eq!(miso_engine_web_v1_spectrum_hop_capability(), 1);
    }

    #[test]
    fn invalid_explicit_hop_borrow_conflict_returns_zero_without_publication() {
        no_live_host();
        let document = stage_ordinary_output_boot();
        BOOT_STAGING.with(|slot| {
            let _borrow = slot.borrow_mut();
            assert_eq!(
                miso_engine_web_v1_boot_with_spectrum_hop(document.len() as u32, 0),
                0,
                "an invalid hop must never become a fake handle on a staging borrow conflict",
            );
        });
        no_live_host();
    }

    fn stage_ordinary_output_boot() -> &'static [u8] {
        let document = include_bytes!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
        stage_left_output_request();
        BOOT_STAGING.with(|slot| {
            slot.borrow_mut().options = WebBootOptions {
                require_sample_rate_hz: 48_000,
                require_quantum_frames: 128,
                ..WebBootOptions::explicit_defaults()
            };
        });
        test_stage_document(document);
        document
    }

    fn stream_hop_metadata() -> WebSpectrumStreamMetadata {
        with_spectrum_staging(|slot| slot.borrow().stream_metadata)
    }

    #[test]
    fn explicit_ordinary_boot_publishes_h256_and_h1024_stream_metadata() {
        for hop in [256_u32, 1024] {
            no_live_host();
            let document = stage_ordinary_output_boot();
            let handle = miso_engine_web_v1_boot_with_spectrum_hop(document.len() as u32, hop);
            assert_ne!(handle, 0, "ordinary H{hop} boot must publish a handle");
            assert_eq!(miso_engine_web_v1_boot_result(), RESULT_OK);
            assert_eq!(
                miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
                RESULT_OK,
                "ordinary H{hop} stream start",
            );
            let metadata = stream_hop_metadata();
            assert_eq!(metadata.sample_rate_hz, 48_000);
            assert_eq!(metadata.quantum_frames, 128);
            assert_eq!(metadata.hop_frames, hop);
            assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
        }
    }

    #[test]
    fn invalid_explicit_hops_refuse_before_publication_and_valid_retries_work() {
        const INVALID_HOPS: [u32; 11] =
            [0, 1, 255, 257, 511, 513, 1023, 1025, 2047, 2049, u32::MAX];

        for hop in INVALID_HOPS {
            no_live_host();
            let document = stage_ordinary_output_boot();
            SPECTRUM_CAPTURE_CONFIGURE_ENTRIES.with(|entries| entries.set(0));
            assert_eq!(
                miso_engine_web_v1_boot_with_spectrum_hop(document.len() as u32, hop),
                0,
                "ordinary invalid H{hop} must refuse",
            );
            assert_eq!(miso_engine_web_v1_boot_result(), RESULT_INVALID_ARGUMENT);
            assert!(miso_engine_web_v1_boot_diagnostic_bytes() > 0);
            assert_eq!(
                SPECTRUM_CAPTURE_CONFIGURE_ENTRIES.with(|entries| entries.get()),
                0,
                "ordinary invalid H{hop} entered spectrum preparation",
            );
            no_live_host();

            let document = stage_ordinary_output_boot();
            let retry = miso_engine_web_v1_boot_with_spectrum_hop(document.len() as u32, 256);
            assert_ne!(retry, 0, "ordinary retry after invalid H{hop}");
            assert_eq!(miso_engine_web_v1_dispose(retry), RESULT_OK);
        }
    }

    #[test]
    fn legacy_boot_wrappers_keep_the_derived_default_hop() {
        no_live_host();
        let document = stage_ordinary_output_boot();
        let handle = miso_engine_web_v1_boot(document.len() as u32);
        assert_ne!(handle, 0);
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        let expected = SpectrumCadence::new(48_000, 128)
            .expect("launch default cadence")
            .hop_frames();
        assert_eq!(stream_hop_metadata().hop_frames, expected);
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
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
        with_spectrum_staging(|slot| {
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
        with_spectrum_staging(|slot| {
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
                    SPECTRUM_TARGET_TRACK_POST_PAN,
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
        with_spectrum_staging(|slot| {
            let mut staging = slot.borrow_mut();
            staging.release_capture();
            staging.release_collection_staging();
            staging.collection_request.entry_count = 257;
            staging.collection_request.maximum_capture_bytes = (SPECTRUM_CAPTURE_BYTES * 2) as u64;
        });
        let _ = miso_engine_web_v1_spectrum_collection_entry_ptr();
        assert_eq!(miso_engine_web_v1_spectrum_collection_entry_capacity(), 257);
        with_spectrum_staging(|slot| {
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
        with_spectrum_staging(|slot| slot.borrow_mut().release_collection_staging());
    }

    #[test]
    fn invalid_spectrum_cancel_refuses_before_touching_managed_stream() {
        with_spectrum_staging(|slot| {
            let mut staging = slot.borrow_mut();
            staging.release_capture();
            staging.capture_len = 32;
            staging.stream_metadata.result = RESULT_OK;
        });
        assert_eq!(
            miso_engine_web_v1_spectrum_cancel(0),
            RESULT_INVALID_ARGUMENT
        );
        with_spectrum_staging(|slot| {
            let staging = slot.borrow();
            assert_eq!(staging.capture_len, 32);
            assert_eq!(staging.stream_metadata.result, RESULT_OK);
        });
        with_spectrum_staging(|slot| slot.borrow_mut().release_capture());
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

    /// Test value: the browser answers the staged request words as the core's request rules do,
    /// not with a second copy: an empty collection takes budget 0 only (any other budget is the
    /// core's budget refusal, `RESULT_REFUSED_BUDGET`, not an invalid argument), and an identity
    /// of 0 or 128 bytes is the core's identity refusal, for a single and for a collection entry.
    #[test]
    fn staged_request_words_are_answered_as_the_core_answers_them() {
        let staged = || with_spectrum_staging(|slot| staged_spectrum_request(&slot.borrow()));

        stage_collection_request();
        assert!(matches!(
            staged(),
            Ok(Some(SpectrumPreparation::Collection(_)))
        ));

        with_spectrum_staging(|slot| {
            let mut staging = slot.borrow_mut();
            staging.collection_request.entry_count = 0;
            staging.collection_request.maximum_capture_bytes = 0;
        });
        assert!(
            matches!(staged(), Ok(None)),
            "an empty collection prepares nothing"
        );
        with_spectrum_staging(|slot| {
            slot.borrow_mut().collection_request.maximum_capture_bytes = 1
        });
        assert_eq!(staged().err(), Some(RESULT_REFUSED_BUDGET));

        for id_bytes in [0_u32, 128] {
            stage_collection_request();
            with_spectrum_staging(|slot| {
                let mut staging = slot.borrow_mut();
                staging.collection_entries[0].target_id_bytes = id_bytes;
                staging.collection_target_ids.resize(256, 0);
            });
            assert_eq!(
                staged().err(),
                Some(RESULT_INVALID_ARGUMENT),
                "a collection identity of {id_bytes} bytes"
            );
        }
        // A zero budget is the core's budget refusal, checked before any entry.
        stage_collection_request();
        with_spectrum_staging(|slot| {
            let mut staging = slot.borrow_mut();
            staging.collection_request.maximum_capture_bytes = 0;
            staging.collection_entries[0].target_id_bytes = 0;
        });
        assert_eq!(staged().err(), Some(RESULT_REFUSED_BUDGET));

        for (id_bytes, expected) in [
            (0_u32, RESULT_INVALID_ARGUMENT),
            (1, RESULT_OK),
            (127, RESULT_OK),
            (128, RESULT_INVALID_ARGUMENT),
        ] {
            with_spectrum_staging(|slot| {
                let mut staging = slot.borrow_mut();
                staging.release_capture();
                staging.release_collection_staging();
                *staging.collection_request = WebSpectrumCollectionRequest::default();
                *staging.request = WebSpectrumRequest {
                    struct_size: SPECTRUM_REQUEST_BYTES,
                    abi_version: ABI_VERSION,
                    target: SPECTRUM_TARGET_OUTPUT,
                    channels: SPECTRUM_CHANNEL_LEFT,
                    target_id_bytes: id_bytes,
                    maximum_capture_bytes: SPECTRUM_CAPTURE_BYTES as u64,
                    ..WebSpectrumRequest::default()
                };
                staging.target_id.fill(b'x');
            });
            let result = staged().err().unwrap_or(RESULT_OK);
            assert_eq!(result, expected, "a single identity of {id_bytes} bytes");
        }
        with_spectrum_staging(|slot| slot.borrow_mut().release_capture());
    }

    /// Test value: the read export answers the core's refusals, two of which differ from the
    /// export that held its own checks (#1591): a host prepared without a spectrum is
    /// `RESULT_UNSUPPORTED` (it was an invalid argument), and a collection with no selection is
    /// `RESULT_WRONG_STATE` (it was an invalid argument). The third change, a token that cannot
    /// advance refusing before the pop and keeping the window, is the service's read order and has
    /// no test (the token cannot be moved from outside the service); a mask outside the selected
    /// one leaving the window for a retry is
    /// `invalid_spectrum_channel_selection_preserves_window_for_valid_retry`.
    #[test]
    fn the_read_export_answers_the_core_refusals() {
        let document = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
        let options = WebBootOptions {
            require_sample_rate_hz: 48_000,
            require_quantum_frames: 128,
            ..WebBootOptions::explicit_defaults()
        };

        with_spectrum_staging(|slot| {
            let mut staging = slot.borrow_mut();
            staging.release_capture();
            staging.release_collection_staging();
        });
        let unprepared = test_boot(document.as_bytes(), options);
        assert_ne!(unprepared, 0, "a session without a spectrum boots");
        assert_eq!(
            miso_engine_web_v1_spectrum_read(unprepared, SPECTRUM_CHANNEL_LEFT),
            RESULT_UNSUPPORTED
        );
        assert_eq!(miso_engine_web_v1_dispose(unprepared), RESULT_OK);

        stage_collection_request();
        let unselected = test_boot(document.as_bytes(), options);
        assert_ne!(unselected, 0, "a collection session boots");
        assert_eq!(
            miso_engine_web_v1_spectrum_read(unselected, SPECTRUM_CHANNEL_LEFT),
            RESULT_WRONG_STATE,
            "no entry is selected"
        );
        assert_eq!(miso_engine_web_v1_spectrum_capture_bytes(), 0);
        assert_eq!(miso_engine_web_v1_dispose(unselected), RESULT_OK);
        with_spectrum_staging(|slot| slot.borrow_mut().release_collection_staging());
    }

    #[test]
    fn select_after_the_callers_arm_is_backpressure_until_the_window_is_read() {
        fn stage_id(id: &[u8]) {
            with_spectrum_staging(|slot| {
                let mut staging = slot.borrow_mut();
                staging.target_id.fill(0);
                staging.target_id[..id.len()].copy_from_slice(id);
            });
        }
        fn render_window(handle: u32, first_block: u64) {
            for block in first_block..first_block + 16 {
                assert_eq!(
                    miso_engine_web_v1_source_submit(handle, 14, 1, block * 128, 2, 128, 0),
                    RESULT_OK,
                    "source block {block}"
                );
                assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
            }
        }
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
        assert_eq!(
            test_copy_staging(handle, BUFFER_SOURCE_ID, b"fixture-source"),
            RESULT_OK
        );
        assert_eq!(test_fill_source_pcm(handle, 0.25), RESULT_OK);

        // The select-made arm belongs to no caller: a later select replaces it.
        stage_id(b"main-out");
        assert_eq!(
            miso_engine_web_v1_spectrum_select(
                handle,
                SPECTRUM_TARGET_OUTPUT,
                SPECTRUM_CHANNEL_LEFT,
                8
            ),
            RESULT_OK
        );
        stage_id(b"eq0");
        assert_eq!(
            miso_engine_web_v1_spectrum_select(
                handle,
                SPECTRUM_TARGET_TRACK_POST_PAN,
                SPECTRUM_CHANNEL_BOTH,
                3
            ),
            RESULT_OK
        );
        assert_eq!(miso_engine_web_v1_spectrum_selection_epoch(handle), 2);
        render_window(handle, 0);
        assert_eq!(
            miso_engine_web_v1_spectrum_read(handle, SPECTRUM_CHANNEL_BOTH),
            RESULT_OK
        );

        // The caller's own arm is not discarded by a select that would replace the entry.
        assert_eq!(miso_engine_web_v1_spectrum_arm(handle), RESULT_OK);
        stage_id(b"main-out");
        assert_eq!(
            miso_engine_web_v1_spectrum_select(
                handle,
                SPECTRUM_TARGET_OUTPUT,
                SPECTRUM_CHANNEL_LEFT,
                8
            ),
            RESULT_BACKPRESSURE
        );
        assert_eq!(miso_engine_web_v1_spectrum_selection_epoch(handle), 2);
        render_window(handle, 16);
        assert_eq!(
            miso_engine_web_v1_spectrum_read(handle, SPECTRUM_CHANNEL_BOTH),
            RESULT_OK,
            "the armed window arrives after the refused select"
        );
        assert_eq!(
            miso_engine_web_v1_spectrum_select(
                handle,
                SPECTRUM_TARGET_OUTPUT,
                SPECTRUM_CHANNEL_LEFT,
                8
            ),
            RESULT_OK
        );
        assert_eq!(miso_engine_web_v1_spectrum_selection_epoch(handle), 3);
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
        with_spectrum_staging(|slot| {
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
        with_spectrum_staging(|slot| {
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

        with_spectrum_staging(|slot| {
            let mut staging = slot.borrow_mut();
            let id = b"eq0";
            staging.target_id.fill(0);
            staging.target_id[..id.len()].copy_from_slice(id);
        });
        assert_eq!(
            miso_engine_web_v1_spectrum_select(
                handle,
                SPECTRUM_TARGET_TRACK_POST_PAN,
                SPECTRUM_CHANNEL_BOTH,
                3,
            ),
            RESULT_OK
        );
        assert_eq!(miso_engine_web_v1_spectrum_selection_epoch(handle), 2);
        assert_eq!(
            with_host(handle, 0, AudioWorkletEngineHost::spectrum_target),
            SPECTRUM_TARGET_TRACK_POST_PAN
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
            with_spectrum_staging(|slot| slot.borrow().stream_metadata.target),
            SPECTRUM_TARGET_TRACK_POST_PAN
        );
        assert!(!analysis_has_history());
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
            with_spectrum_staging(|slot| slot.borrow().stream_metadata.target),
            SPECTRUM_TARGET_TRACK_POST_PAN
        );

        // Repeating the exact prepared selection is an idempotent no-op. It must preserve the
        // ready window and its sequence instead of spuriously resetting the live stream.
        with_spectrum_staging(|slot| {
            let mut staging = slot.borrow_mut();
            let id = b"eq0";
            staging.target_id.fill(0);
            staging.target_id[..id.len()].copy_from_slice(id);
        });
        let before_noop = with_spectrum_staging(|slot| slot.borrow().stream_metadata);
        assert_eq!(
            miso_engine_web_v1_spectrum_select(
                handle,
                SPECTRUM_TARGET_TRACK_POST_PAN,
                SPECTRUM_CHANNEL_BOTH,
                3,
            ),
            RESULT_OK
        );
        let after_noop = with_spectrum_staging(|slot| slot.borrow().stream_metadata);
        assert_eq!(miso_engine_web_v1_spectrum_selection_epoch(handle), 2);
        assert_eq!(after_noop.status, SPECTRUM_STREAM_STATUS_READY);
        assert_eq!(after_noop.target, before_noop.target);
        assert_eq!(after_noop.channels, before_noop.channels);
        assert_eq!(after_noop.sequence, before_noop.sequence);
        assert_eq!(after_noop.windows, before_noop.windows);
        assert!(!analysis_has_history());

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
        let after_refusal = with_spectrum_staging(|slot| slot.borrow().stream_metadata);
        assert_eq!(after_refusal.status, SPECTRUM_STREAM_STATUS_READY);
        assert_eq!(after_refusal.target, SPECTRUM_TARGET_TRACK_POST_PAN);
        assert_eq!(after_refusal.channels, SPECTRUM_CHANNEL_BOTH);
        assert_eq!(after_refusal.smoothing_ms, 0.0);
        assert_eq!(
            with_host(handle, 0, AudioWorkletEngineHost::spectrum_target),
            SPECTRUM_TARGET_TRACK_POST_PAN
        );

        with_spectrum_staging(|slot| {
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
        let selected = with_spectrum_staging(|slot| slot.borrow().stream_metadata);
        assert_eq!(selected.status, SPECTRUM_STREAM_STATUS_WARMING);
        assert_eq!(selected.target, SPECTRUM_TARGET_OUTPUT);
        assert_eq!(selected.channels, SPECTRUM_CHANNEL_LEFT);
        assert_eq!(selected.smoothing_ms, 250.5);
        assert!(!analysis_has_history());
        for block in 32..48_u64 {
            assert_eq!(
                miso_engine_web_v1_source_submit(handle, 14, 1, block * 128, 2, 128, 0),
                RESULT_OK,
                "switched stream source block {block}"
            );
            assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        }
        assert_eq!(miso_engine_web_v1_spectrum_stream_read(handle), RESULT_OK);
        let switched = with_spectrum_staging(|slot| slot.borrow().stream_metadata);
        assert_eq!(switched.status, SPECTRUM_STREAM_STATUS_READY);
        assert_eq!(switched.target, SPECTRUM_TARGET_OUTPUT);
        assert_eq!(switched.channels, SPECTRUM_CHANNEL_LEFT);

        // The first analysis creates the sole worker-side history. A same-entry smoothing update
        // must restart the native window and clear a queued old result while retaining the new
        // history epoch; it must not allocate another analyzer on the host side.
        assert_eq!(miso_engine_web_v1_spectrum_stream_analysis(), RESULT_OK);
        let analyzed = with_spectrum_staging(|slot| slot.borrow().stream_metadata);
        assert_eq!(analyzed.analysis_epoch, 0);
        assert!(analysis_has_history());
        for block in 48..64_u64 {
            assert_eq!(
                miso_engine_web_v1_source_submit(handle, 14, 1, block * 128, 2, 128, 0),
                RESULT_OK,
                "queued old stream source block {block}"
            );
            assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        }
        let before_smoothing_restart = with_spectrum_staging(|slot| slot.borrow().stream_metadata);
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
        let after_smoothing_restart = with_spectrum_staging(|slot| slot.borrow().stream_metadata);
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
            with_spectrum_staging(|slot| slot.borrow().stream_metadata.status),
            SPECTRUM_STREAM_STATUS_WARMING
        );
        with_spectrum_staging(|slot| {
            let mut staging = slot.borrow_mut();
            let id = b"eq0";
            staging.target_id.fill(0);
            staging.target_id[..id.len()].copy_from_slice(id);
        });
        assert_eq!(
            miso_engine_web_v1_spectrum_select(
                handle,
                SPECTRUM_TARGET_TRACK_POST_PAN,
                SPECTRUM_CHANNEL_BOTH,
                3,
            ),
            RESULT_OK
        );
        let after_target_restart = with_spectrum_staging(|slot| slot.borrow().stream_metadata);
        assert_eq!(after_target_restart.status, SPECTRUM_STREAM_STATUS_WARMING);
        assert_eq!(miso_engine_web_v1_spectrum_selection_epoch(handle), 4);
        assert_eq!(after_target_restart.analysis_epoch, 2);
        assert_eq!(after_target_restart.target, SPECTRUM_TARGET_TRACK_POST_PAN);
        assert_eq!(after_target_restart.channels, SPECTRUM_CHANNEL_BOTH);
        assert!(analysis_has_history());
        assert_eq!(miso_engine_web_v1_spectrum_stream_stop(handle), RESULT_OK);
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    }

    /// Batch 8 review, m4. Turns red if the analysis export answers a running stream that has no
    /// ready window as a wrong state (the browser's bytes: invalid input), or a stopped or
    /// never-started stream as anything but a wrong state.
    #[test]
    fn stream_analysis_answers_by_the_stream_state() {
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
            miso_engine_web_v1_spectrum_stream_analysis(),
            RESULT_WRONG_STATE,
            "no stream was started"
        );
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK
        );
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_read(handle),
            RESULT_BACKPRESSURE
        );
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_analysis(),
            RESULT_INVALID_ARGUMENT,
            "the stream runs and no window is ready"
        );
        assert_eq!(miso_engine_web_v1_spectrum_stream_stop(handle), RESULT_OK);
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_analysis(),
            RESULT_WRONG_STATE,
            "the stream was stopped"
        );
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
        let started = with_spectrum_staging(|slot| slot.borrow().stream_metadata);
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
        let captured = with_spectrum_staging(|slot| slot.borrow().stream_metadata);
        assert_eq!(captured.status, SPECTRUM_STREAM_STATUS_READY);
        assert_eq!(captured.capture_epoch, 1);
        assert_eq!(captured.sequence, 0);
        assert_eq!(captured.windows, 1);
        assert_eq!(captured.captured_sample, 0);
        assert_eq!(captured.end_sample, 2_048);
        assert_eq!(miso_engine_web_v1_spectrum_stream_analysis(), RESULT_OK);
        let analyzed = with_spectrum_staging(|slot| slot.borrow().stream_metadata);
        assert_eq!(analyzed.status, SPECTRUM_STREAM_STATUS_READY);
        assert_eq!(analyzed.analysis_epoch, 0);
        assert_eq!(analyzed.history_start_sample, 0);
        assert_eq!(analyzed.smoothing_ms, 0.0);
        assert!(with_spectrum_staging(|slot| slot.borrow().result_len > 0));
        assert_eq!(miso_engine_web_v1_spectrum_stream_stop(handle), RESULT_OK);
        assert_eq!(
            with_spectrum_staging(|slot| slot.borrow().stream_metadata.status),
            SPECTRUM_STREAM_STATUS_STOPPED
        );
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    }

    /// The nine-track fixture with a muted copy of its first track (whose ID sorts first) and the
    /// copy's route to the output, as one rebuild transaction at `revision` (#1618, m7).
    fn adding_a_strip_frame(request: u64, revision: u64) -> Vec<u8> {
        use protocol::{
            CommandPayload, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit,
            SessionRevision, TypedCommandFrame,
        };
        let model = session::parse_session_json(include_str!(
            "../../../fixtures/session/v1/parametric-eq-nine-track.json"
        ))
        .expect("the fixture parses");
        let mut track = model.tracks[0].clone();
        track.id = session::StableId::parse("a-muted").expect("track ID");
        track.fader.left_mute = true;
        track.fader.right_mute = true;
        let mut route = model.routes[0].clone();
        route.id = session::StableId::parse("a-muted-main").expect("route ID");
        let session::RouteSource::Track { track_id, .. } = &mut route.source else {
            panic!("a track route")
        };
        *track_id = track.id.clone();
        let edits = [
            SessionEdit::UpsertTrack { track },
            SessionEdit::UpsertRoute { route },
        ];
        let codec = ProtocolCodec::default();
        let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
        let length = codec
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(request).expect("nonzero request"),
                    expected_revision: ExpectedRevision::Exact(SessionRevision(revision)),
                    payload: CommandPayload::SessionTransactionApply(&edits),
                },
                &mut bytes,
            )
            .expect("a transaction frame");
        bytes.truncate(length);
        bytes
    }

    /// A rebuild whose promotion does not keep the spectrum stream ends the instance's analysis
    /// with it (#1618, m1 and m7 of the batch 12b review), whichever control export runs the
    /// promotion: the service step, or the reclaim a control command starts with. The swap carries
    /// (the engine reports it so) but its pairing does not name the capture, so the successor's
    /// fresh capture is installed and the service's stream ends.
    ///
    /// Test value: red if `miso_engine_web_v1_service` or `miso_engine_web_v1_control_command`
    /// promotes without the staging's analysis: the stream ends but the analysis stays active, so
    /// `spectrum_stream_reset` answers ok instead of a wrong state and `spectrum_stream_analysis`
    /// re-analyzes the last window of a stream that no longer exists.
    #[test]
    fn a_promotion_that_drops_the_stream_ends_the_analysis() {
        for by_service in [true, false] {
            stage_left_output_request();
            let document =
                include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
            let handle = test_boot(
                document.as_bytes(),
                WebBootOptions {
                    require_sample_rate_hz: 48_000,
                    require_quantum_frames: 128,
                    live_control_meter_blocks: 2,
                    live_control_observation_taps: 4,
                    ..WebBootOptions::explicit_defaults()
                },
            );
            assert_ne!(handle, 0, "spectrum fixture must boot");
            assert_eq!(
                miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
                RESULT_OK
            );
            assert_eq!(
                test_copy_staging(handle, BUFFER_SOURCE_ID, b"fixture-source"),
                RESULT_OK
            );
            assert_eq!(test_fill_source_pcm(handle, 0.25), RESULT_OK);
            let mut block = 0_u64;
            let mut render = |count: u64| {
                for _ in 0..count {
                    assert_eq!(
                        miso_engine_web_v1_source_submit(handle, 14, 1, block * 128, 2, 128, 0),
                        RESULT_OK,
                        "source block {block}"
                    );
                    assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
                    block += 1;
                }
            };
            render(16);
            assert_eq!(miso_engine_web_v1_spectrum_stream_read(handle), RESULT_OK);
            assert_eq!(miso_engine_web_v1_spectrum_stream_analysis(), RESULT_OK);
            assert!(with_spectrum_staging(|slot| {
                slot.borrow()
                    .analysis
                    .as_ref()
                    .is_some_and(SpectrumAnalysis::stream_active)
            }));

            // A rebuild commits, and the block after it adopts the successor plan.
            let revision = with_host(handle, 0, AudioWorkletEngineHost::committed_revision);
            let (result, _) =
                native_staging::control_command(handle, &adding_a_strip_frame(1, revision))
                    .expect("the control staging");
            assert_eq!(result, RESULT_OK);
            render(2);
            // The pairing names nothing: the swap carried, but not this capture.
            with_host_mut(handle, (), |host| {
                let ready = host.ready.as_mut().expect("ready");
                let waiting = ready
                    .pending_shapes
                    .iter_mut()
                    .filter(|entry| entry.adoption == Some(true))
                    .filter_map(|entry| entry.spectrum.as_mut())
                    .map(|handover| handover.pairing = Box::default())
                    .count();
                assert_eq!(waiting, 1, "the adopted plan waits for its promotion");
            });
            if by_service {
                assert_eq!(miso_engine_web_v1_service(handle), RESULT_OK);
            } else {
                // Any control command reclaims first; a revision probe is the lightest.
                let probe = {
                    use protocol::{
                        CommandPayload, ExpectedRevision, ProtocolCodec, RequestId,
                        TypedCommandFrame,
                    };
                    let codec = ProtocolCodec::default();
                    let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
                    let length = codec
                        .encode_command_frame_into(
                            &TypedCommandFrame {
                                request_id: RequestId::new(2).expect("nonzero request"),
                                expected_revision: ExpectedRevision::Any,
                                payload: CommandPayload::CapabilitiesGet,
                            },
                            &mut bytes,
                        )
                        .expect("a capabilities frame");
                    bytes.truncate(length);
                    bytes
                };
                let (result, _) =
                    native_staging::control_command(handle, &probe).expect("the control staging");
                assert_eq!(result, RESULT_OK);
            }

            assert!(
                with_spectrum_staging(|slot| {
                    slot.borrow()
                        .analysis
                        .as_ref()
                        .is_some_and(|analysis| !analysis.stream_active())
                }),
                "the analysis ended with the stream (by_service = {by_service})"
            );
            assert_eq!(
                miso_engine_web_v1_spectrum_stream_reset(),
                RESULT_WRONG_STATE,
                "there is no stream to reset (by_service = {by_service})"
            );
            assert_ne!(
                miso_engine_web_v1_spectrum_stream_analysis(),
                RESULT_OK,
                "the analysis does not re-report the last window (by_service = {by_service})"
            );
            assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
        }
    }

    /// Issue #1592 D4. Turns red if a stream start on a collection discards a caller's arm (the
    /// browser's old `arm`, then `stream_start`), or no longer replaces the arm that a select made.
    #[test]
    fn stream_start_refuses_a_callers_arm_on_a_collection() {
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
        assert_eq!(
            test_copy_staging(handle, BUFFER_SOURCE_ID, b"fixture-source"),
            RESULT_OK
        );
        assert_eq!(test_fill_source_pcm(handle, 0.25), RESULT_OK);
        with_spectrum_staging(|slot| {
            let mut staging = slot.borrow_mut();
            staging.target_id.fill(0);
            staging.target_id[..8].copy_from_slice(b"main-out");
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
        assert_eq!(miso_engine_web_v1_spectrum_cancel(handle), RESULT_OK);
        assert_eq!(miso_engine_web_v1_spectrum_arm(handle), RESULT_OK);
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_BACKPRESSURE,
            "the caller's arm keeps its window"
        );
        assert_eq!(
            with_spectrum_staging(|slot| slot.borrow().stream_metadata.result),
            RESULT_BACKPRESSURE
        );
        for block in 0..16_u64 {
            assert_eq!(
                miso_engine_web_v1_source_submit(handle, 14, 1, block * 128, 2, 128, 0),
                RESULT_OK,
                "source block {block}"
            );
            assert_eq!(miso_engine_web_v1_render(handle, 128), RESULT_OK);
        }
        assert_eq!(
            miso_engine_web_v1_spectrum_read(handle, SPECTRUM_CHANNEL_LEFT),
            RESULT_OK,
            "the armed window still reads"
        );
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(handle, 0.0),
            RESULT_OK,
            "the read ended the caller's claim"
        );
        assert_eq!(miso_engine_web_v1_spectrum_stream_stop(handle), RESULT_OK);
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    }

    #[test]
    fn continuous_spectrum_ffi_imports_metadata_without_a_host_handle() {
        stage_left_output_request();
        with_spectrum_staging(|slot| {
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
        let metadata = with_spectrum_staging(|slot| slot.borrow().stream_metadata);
        assert_eq!(metadata.status, SPECTRUM_STREAM_STATUS_READY);
        assert_eq!(metadata.capture_epoch, 1);
        assert_eq!(metadata.sequence, 0);
        assert_eq!(metadata.analysis_epoch, 0);
        assert!(with_spectrum_staging(|slot| slot.borrow().result_len > 0));

        with_spectrum_staging(|slot| {
            let mut staging = slot.borrow_mut();
            let mut header: WebSpectrumWindow =
                read_live_record(staging.capture.as_ref().expect("capture staging"), 0)
                    .expect("window header");
            header.target = SPECTRUM_TARGET_TRACK_POST_PAN;
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
        with_spectrum_staging(|slot| {
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
                    &SpectrumSnapshot {
                        window,
                        target_kind: SPECTRUM_TARGET_OUTPUT,
                        sample_rate_hz,
                        token,
                    },
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
        stage_imported_stream_metadata_with_hop(
            first_sample,
            sequence,
            sample_rate_hz,
            smoothing_ms,
            2_048,
        );
    }

    fn stage_imported_stream_metadata_with_hop(
        first_sample: u64,
        sequence: u64,
        sample_rate_hz: u32,
        smoothing_ms: f64,
        hop_frames: u32,
    ) {
        with_spectrum_staging(|slot| {
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
                hop_frames,
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

    #[test]
    fn imported_stream_analysis_accepts_explicit_hops() {
        for hop_frames in [256_u32, 1_024] {
            stage_left_output_request();
            stage_imported_left_window(0, 48_000, 1, 0.25);
            stage_imported_stream_metadata_with_hop(0, 0, 48_000, 0.0, hop_frames);

            assert_eq!(
                miso_engine_web_v1_spectrum_stream_analysis_configure(),
                RESULT_OK,
                "H{hop_frames} import configuration",
            );
            assert_eq!(
                analysis_cadence()
                    .expect("configured stream cadence")
                    .hop_frames(),
                hop_frames,
            );
            assert_eq!(
                miso_engine_web_v1_spectrum_stream_analysis(),
                RESULT_OK,
                "H{hop_frames} imported window analysis",
            );
            let metadata = with_spectrum_staging(|slot| slot.borrow().stream_metadata);
            assert_eq!(metadata.status, SPECTRUM_STREAM_STATUS_READY);
            assert_eq!(metadata.hop_frames, hop_frames);
            assert!(with_spectrum_staging(|slot| slot.borrow().result_len > 0));
        }
        stage_left_output_request();
    }

    #[test]
    fn imported_stream_analysis_accepts_high_rate_derived_default_hop() {
        let cadence = SpectrumCadence::new(96_000, 128).expect("launch default cadence");
        assert_eq!(cadence.hop_frames(), 3_200);
        stage_left_output_request();
        stage_imported_left_window(0, 96_000, 1, 0.25);
        stage_imported_stream_metadata_with_hop(0, 0, 96_000, 0.0, cadence.hop_frames());

        assert_eq!(
            miso_engine_web_v1_spectrum_stream_analysis_configure(),
            RESULT_OK,
        );
        assert_eq!(
            analysis_cadence()
                .expect("configured stream cadence")
                .hop_frames(),
            3_200,
        );
        assert_eq!(miso_engine_web_v1_spectrum_stream_analysis(), RESULT_OK);
        let metadata = with_spectrum_staging(|slot| slot.borrow().stream_metadata);
        assert_eq!(metadata.status, SPECTRUM_STREAM_STATUS_READY);
        assert_eq!(metadata.hop_frames, 3_200);
        assert!(with_spectrum_staging(|slot| slot.borrow().result_len > 0));
        stage_left_output_request();
    }

    #[test]
    fn imported_stream_analysis_rejects_unsupported_hops_transactionally() {
        for (sample_rate_hz, hop_frames) in [(48_000_u32, 0_u32), (48_000, 300), (96_000, 3_072)] {
            stage_left_output_request();
            stage_imported_left_window(0, sample_rate_hz, 1, 0.25);
            stage_imported_stream_metadata_with_hop(0, 0, sample_rate_hz, 0.0, hop_frames);
            let before = with_spectrum_staging(|slot| {
                let staging = slot.borrow();
                (
                    staging.capture_len,
                    staging.result_len,
                    analysis_stream_state(&staging),
                )
            });

            assert_eq!(
                miso_engine_web_v1_spectrum_stream_analysis_configure(),
                RESULT_INVALID_ARGUMENT,
                "unsupported H{hop_frames} import",
            );
            let after = with_spectrum_staging(|slot| {
                let staging = slot.borrow();
                (
                    staging.capture_len,
                    staging.result_len,
                    analysis_stream_state(&staging),
                )
            });
            assert_eq!(
                after, before,
                "unsupported H{hop_frames} partially configured state"
            );
        }
        stage_left_output_request();
    }

    fn spectrum_result_header() -> WebSpectrumResult {
        with_spectrum_staging(|slot| {
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
        let first_stream = with_spectrum_staging(|slot| slot.borrow().stream_metadata);
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
            with_spectrum_staging(|slot| slot.borrow().stream_metadata),
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
        let second_stream = with_spectrum_staging(|slot| slot.borrow().stream_metadata);
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
mod observation_alias_tests {
    //! The ordinary observation aliases' live-host guard and the resident staging accounting.
    use super::*;
    use crate::SPECTRUM_STREAM_STATUS_READY;

    pub(super) fn no_live_host() {
        assert!(LIVE_HOST.with(|slot| slot.borrow().is_none()));
    }

    fn boot_ordinary() -> u32 {
        no_live_host();
        with_spectrum_staging(|slot| {
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
        let document = include_bytes!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
        let handle = test_boot(document, WebBootOptions::explicit_defaults());
        assert_ne!(handle, 0, "ordinary fixture must boot");
        handle
    }

    fn dispose(handle: u32) {
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
        no_live_host();
    }

    fn stage_spectrum_markers() -> (usize, usize, WebSpectrumStreamMetadata, AnalysisState) {
        with_spectrum_staging(|slot| {
            let mut staging = slot.borrow_mut();
            staging.capture_len = 17;
            staging.result_len = 19;
            staging.stream_metadata.status = SPECTRUM_STREAM_STATUS_READY;
            staging.stream_metadata.result = RESULT_OK;
            staging.stream_metadata.sequence = 41;
            staging.stream_metadata.dropped_captures = 3;
            staging.ensure_analysis_storage().expect("analysis storage");
            staging
                .analysis
                .as_mut()
                .expect("analysis storage")
                .configure_imported(&SpectrumStreamMetadata {
                    status: SpectrumStreamStatus::Ready,
                    target_kind: SPECTRUM_TARGET_OUTPUT,
                    channels: SPECTRUM_CHANNEL_LEFT,
                    sample_rate_hz: 48_000,
                    quantum_frames: 128,
                    hop_frames: 2_048,
                    capture_epoch: 1,
                    sequence: 41,
                    windows: 42,
                    end_sample: u64::from(SPECTRUM_WINDOW_FRAMES),
                    smoothing_ms: 5.0,
                    ..SpectrumStreamMetadata::default()
                })
                .expect("fixture metadata imports");
            (
                staging.capture_len,
                staging.result_len,
                staging.stream_metadata,
                analysis_stream_state(&staging),
            )
        })
    }

    fn assert_spectrum_markers(markers: (usize, usize, WebSpectrumStreamMetadata, AnalysisState)) {
        with_spectrum_staging(|slot| {
            let staging = slot.borrow();
            assert_eq!(staging.capture_len, markers.0);
            assert_eq!(staging.result_len, markers.1);
            assert_eq!(staging.stream_metadata, markers.2);
            assert_eq!(analysis_stream_state(&staging), markers.3);
        });
    }

    fn stage_resident_markers() -> Vec<WebObservationResult> {
        with_observation_staging(|slot| {
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
        with_observation_staging(|slot| {
            let staging = slot.borrow();
            assert_eq!(staging.results.as_slice(), markers);
        });
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
        analysis: AnalysisState,
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
            target: SPECTRUM_TARGET_TRACK_POST_PAN,
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
            target: SPECTRUM_TARGET_TRACK_POST_PAN,
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

        with_spectrum_staging(|slot| {
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
            staging.stream_metadata = WebSpectrumStreamMetadata {
                struct_size: SPECTRUM_STREAM_METADATA_BYTES,
                abi_version: ABI_VERSION,
                result: RESULT_OK,
                status: SPECTRUM_STREAM_STATUS_READY,
                target: SPECTRUM_TARGET_TRACK_POST_PAN,
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
            let metadata = stream_metadata_of(&staging.stream_metadata);
            staging.ensure_analysis_storage().expect("analysis storage");
            staging
                .analysis
                .as_mut()
                .expect("analysis storage")
                .configure_imported(&metadata)
                .expect("fixture metadata imports");
        });
        with_spectrum_staging(|slot| {
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
                analysis: analysis_stream_state(&staging),
            }
        })
    }

    fn assert_staged_markers(markers: &CommittedMarkers) {
        with_spectrum_staging(|slot| {
            let staging = slot.borrow();
            assert_eq!(staging.capture_len, markers.capture_len);
            assert_eq!(staging.result_len, markers.result_len);
            assert_eq!(staging.stream_metadata, markers.stream_metadata);
            assert_eq!(analysis_stream_state(&staging), markers.analysis);
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

    /// Decision 12 (issue #1096): the observation map and the selected-read records name a console
    /// slot `3` and an insert `1`, and a selection carrying the retired `simd1` (`0`) or `simd2`
    /// (`2`) code, an unallocated code, or the rack the session does not use there is refused.
    /// The two fixtures are the same compressors as a console slot and as inserts.
    ///
    /// Red mutations: encode the console as the retired `0` in `observation_rack_raw`, or decode
    /// `0`/`2` in `observation_rack` -> a retired selection is read.
    #[test]
    fn observation_records_address_console_slots_and_inserts() {
        let console: &[u8] =
            include_bytes!("../../../fixtures/session/v1/compressor-bank-observation.json");
        let inserts: &[u8] =
            include_bytes!("../../../fixtures/session/v1/compressor-dynamic-bank-observation.json");
        for (document, rack, unused) in [
            (console, crate::RACK_CONSOLE, crate::RACK_INSERTS),
            (inserts, crate::RACK_INSERTS, crate::RACK_CONSOLE),
        ] {
            no_live_host();
            let options = WebBootOptions {
                require_sample_rate_hz: 48_000,
                require_quantum_frames: 128,
                live_control_meter_blocks: 1,
                live_control_observation_taps: 4,
                ..WebBootOptions::explicit_defaults()
            };
            let handle = test_boot(document, options);
            assert_ne!(handle, 0, "the observation fixture must boot");
            let count = miso_engine_web_v1_observation_count(handle);
            assert_eq!(count, 8, "one observed compressor per track");
            for index in 0..count {
                assert_eq!(
                    miso_engine_web_v1_observation_rack(handle, index),
                    u32::from(rack)
                );
                assert_eq!(
                    miso_engine_web_v1_observation_effect_index(handle, index),
                    0
                );
            }
            let track_index = miso_engine_web_v1_observation_track_index(handle, 3);
            let tap_id = miso_engine_web_v1_observation_tap_id(handle, 3, 0);
            let read = |selection_rack: u32| {
                with_observation_staging(|slot| {
                    slot.borrow_mut().selections[0] = WebObservationSelection {
                        struct_size: OBSERVATION_SELECTION_BYTES,
                        abi_version: ABI_VERSION,
                        track_index,
                        rack: selection_rack,
                        effect_index: 0,
                        tap_id,
                        channels: OBSERVATION_CHANNEL_BOTH,
                        reserved: 0,
                    };
                });
                miso_engine_web_v1_observation_read(handle, 1)
            };
            assert_eq!(read(u32::from(rack)), RESULT_OK);
            let row = with_observation_staging(|slot| slot.borrow().results[0]);
            assert_eq!(
                (row.track_index, row.rack, row.effect_index),
                (track_index, u32::from(rack), 0)
            );
            for refused in [0, 2, 4, u32::from(unused)] {
                assert_eq!(
                    read(refused),
                    RESULT_INVALID_ARGUMENT,
                    "a selection at rack {refused} is refused"
                );
            }
            dispose(handle);
        }
    }

    /// #1096 verdict M1: an observed `post_insert` slot is read at its console slot index, which
    /// counts after `pre_insert`.
    ///
    /// The fixture puts an unobserved EQ in `pre_insert` and the observed compressor in
    /// `post_insert`, so the compressor is console slot `1`. Red if a numeric read resolves a
    /// console selection over `pre_insert` alone (`resolve_observation` then finds no `comp` and
    /// refuses the read), or if `post_insert` is numbered from `0` (the map reports slot `0`, and
    /// slot `0` is the EQ, which has no tap).
    #[test]
    fn a_post_insert_observation_is_read_at_its_console_slot_index() {
        let mut document: serde_json::Value = serde_json::from_slice(include_bytes!(
            "../../../fixtures/session/v1/compressor-bank-observation.json"
        ))
        .expect("the observation fixture is JSON");
        let compressor = document["console"]["pre_insert"][0].take();
        assert_eq!(compressor["slot"], "comp");
        document["console"]["pre_insert"] = serde_json::json!([{
            "slot": "eq",
            "identity": { "kind": "native", "effect_id": "miso.parametric-eq" },
            "quality": "normal",
            "link_mode": "dual_mono",
        }]);
        document["console"]["post_insert"] = serde_json::json!([compressor]);
        let tracks = document["tracks"]
            .as_array_mut()
            .expect("the fixture's tracks");
        for track in tracks.iter_mut() {
            track["console"]
                .as_array_mut()
                .expect("a track's console entries")
                .insert(
                    0,
                    serde_json::json!({ "slot": "eq", "bypass": false, "params": [] }),
                );
        }
        let bytes = serde_json::to_vec(&document).expect("the moved fixture serializes");

        no_live_host();
        let options = WebBootOptions {
            require_sample_rate_hz: 48_000,
            require_quantum_frames: 128,
            live_control_meter_blocks: 1,
            live_control_observation_taps: 4,
            ..WebBootOptions::explicit_defaults()
        };
        let handle = test_boot(&bytes, options);
        assert_ne!(handle, 0, "the post_insert observation fixture must boot");
        let count = miso_engine_web_v1_observation_count(handle);
        assert_eq!(
            count, 8,
            "one observed compressor per track; the EQ has no tap"
        );
        for index in 0..count {
            assert_eq!(
                miso_engine_web_v1_observation_rack(handle, index),
                u32::from(crate::RACK_CONSOLE)
            );
            assert_eq!(
                miso_engine_web_v1_observation_effect_index(handle, index),
                1,
                "the post_insert compressor is console slot 1, after the pre_insert EQ"
            );
        }
        let track_index = miso_engine_web_v1_observation_track_index(handle, 5);
        let tap_id = miso_engine_web_v1_observation_tap_id(handle, 5, 0);
        let read = |rack: u32, effect_index: u32| {
            with_observation_staging(|slot| {
                slot.borrow_mut().selections[0] = WebObservationSelection {
                    struct_size: OBSERVATION_SELECTION_BYTES,
                    abi_version: ABI_VERSION,
                    track_index,
                    rack,
                    effect_index,
                    tap_id,
                    channels: OBSERVATION_CHANNEL_BOTH,
                    reserved: 0,
                };
            });
            miso_engine_web_v1_observation_read(handle, 1)
        };
        let console = u32::from(crate::RACK_CONSOLE);
        assert_eq!(
            read(console, 1),
            RESULT_OK,
            "the numeric read of the post_insert slot"
        );
        let row = with_observation_staging(|slot| slot.borrow().results[0]);
        assert_eq!(
            (row.track_index, row.rack, row.effect_index, row.tap_id),
            (track_index, console, 1, tap_id)
        );
        assert_eq!(
            read(console, 0),
            RESULT_UNSUPPORTED,
            "console slot 0 is the untapped EQ"
        );
        assert_eq!(
            read(console, 2),
            RESULT_INVALID_ARGUMENT,
            "there is no console slot 2"
        );
        assert_eq!(
            read(u32::from(crate::RACK_INSERTS), 0),
            RESULT_INVALID_ARGUMENT,
            "the track has no inserts"
        );
        dispose(handle);
    }

    #[test]
    fn observation_staging_retention_adds_actual_refcell_once() {
        let count = MAXIMUM_OBSERVATION_READS as u64;
        let unchanged_heap_sum = count
            * (size_of::<WebObservationSelection>() as u64
                + size_of::<ObservationAddress>() as u64
                + size_of::<ObservationReadValues>() as u64)
            + count * size_of::<WebObservationResult>() as u64
            + OBSERVATION_MAXIMUM_ID_BYTES as u64;
        let containing = size_of::<RefCell<ObservationStaging>>() as u64;
        assert_eq!(
            observation_staging_retained_bytes(),
            unchanged_heap_sum + containing,
            "the actual staging container is charged exactly once"
        );
        assert!(observation_staging_largest_allocation_bytes() >= containing);
    }

    #[test]
    fn alias_host_borrow_refusal_is_terminal_before_any_staging_work() {
        let handle = boot_ordinary();
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
        let handle = boot_ordinary();
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
    fn stream_guard_refusals_preserve_markers_for_both_host_borrow_kinds() {
        let handle = boot_ordinary();
        let markers = stage_markers();

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
        assert_staged_markers(&markers);

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
        assert_staged_markers(&markers);

        let invalid_handle = handle.wrapping_add(1).max(1);
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_start(invalid_handle, 12.5),
            RESULT_INVALID_ARGUMENT
        );
        assert_eq!(
            miso_engine_web_v1_spectrum_stream_stop(invalid_handle),
            RESULT_INVALID_ARGUMENT
        );
        assert_staged_markers(&markers);
        dispose(handle);
    }
}
