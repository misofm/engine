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

use crate::{
    ABI_VERSION, AudioWorkletEngineHost, BUFFER_COMMAND, BUFFER_DIAGNOSTIC, BUFFER_METER_FRAME,
    BUFFER_OUTPUT_PCM, BUFFER_SOURCE_ID, BUFFER_SOURCE_PCM, BootFailure,
    LIVE_RESPONSE_CAPTURE_BYTES, LIVE_RESPONSE_MAXIMUM_ID_BYTES, LIVE_RESPONSE_MAXIMUM_OWNERS,
    LIVE_RESPONSE_MAXIMUM_POINTS, LIVE_RESPONSE_MEANING_EQ_FILTER_SUBTOTAL,
    LIVE_RESPONSE_MODE_TARGET, LIVE_RESPONSE_OWNER_BYTES, LIVE_RESPONSE_REQUEST_BYTES,
    LIVE_RESPONSE_RESULT_BYTES, LIVE_RESPONSE_SECTION_BYTES, MAXIMUM_DOCUMENT_BYTES,
    MAXIMUM_OBSERVATION_READS, OBSERVATION_CHANNEL_BOTH, OBSERVATION_CHANNEL_LEFT,
    OBSERVATION_CHANNEL_RIGHT, OBSERVATION_RESULT_BYTES, OBSERVATION_SELECTION_BYTES,
    OBSERVATION_STATUS_PENDING, OBSERVATION_STATUS_READY, OBSERVATION_STATUS_UNARMED,
    ObservationAddress, ObservationReadChannels, ObservationReadError, ObservationReadValues,
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
    WebLiveResponseSection, WebObservationResult, WebObservationSelection, WebResponseParameter,
    WebResponseRequest, WebResponseResult, WebSpectrumCollectionEntry,
    WebSpectrumCollectionRequest, WebSpectrumRequest, WebSpectrumResult, WebSpectrumStreamMetadata,
    WebSpectrumWindow,
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
    ResponseParameterOverride, ResponsePreviewError, ResponsePreviewGrid, ResponsePreviewLimits,
    ResponsePreviewOutput, ResponsePreviewRequest, ResponsePreviewTarget, ResponseSnapshot,
    ResponseSnapshotAvailability, ResponseSnapshotOutput, ResponseSnapshotOwner,
    ResponseSnapshotQueryError, SpectrumAnalysisHistory, SpectrumAnalyzer, SpectrumCadence,
    SpectrumCaptureCollectionEntry, SpectrumCaptureCollectionRequest, SpectrumCaptureRequest,
    SpectrumChannels, SpectrumContinuousReadError, SpectrumContinuousWindow,
    SpectrumSmoothingConfig, SpectrumTarget, SpectrumWindow, prepare_response_preview,
    query_response_snapshot_into,
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
        }
    }

    fn configure_capture(&mut self) -> Result<(), u32> {
        if self.capture.is_some() && self.result.is_some() {
            return Ok(());
        }
        let mut capture = Vec::new();
        capture
            .try_reserve_exact(SPECTRUM_CAPTURE_BYTES)
            .map_err(|_| RESULT_REFUSED_BUDGET)?;
        capture.resize(SPECTRUM_CAPTURE_BYTES, 0);
        let mut result = Vec::new();
        result
            .try_reserve_exact(SPECTRUM_CAPTURE_BYTES)
            .map_err(|_| RESULT_REFUSED_BUDGET)?;
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
}

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
            id: vec![0; RESPONSE_MAXIMUM_EFFECT_ID_BYTES as usize].into_boxed_slice(),
        }
    }

    fn reset_results(&mut self) {
        self.addresses.clear();
        self.results.clear();
    }
}

/// Heap payload retained by the fixed selected-observation staging area.
pub(crate) const fn observation_staging_retained_bytes() -> u64 {
    let count = MAXIMUM_OBSERVATION_READS as u64;
    count * size_of::<WebObservationSelection>() as u64
        + count * size_of::<ObservationAddress>() as u64
        + count * size_of::<ObservationReadValues>() as u64
        + count * size_of::<WebObservationResult>() as u64
        + RESPONSE_MAXIMUM_EFFECT_ID_BYTES as u64
}

/// Largest one allocation in the fixed selected-observation staging area.
pub(crate) const fn observation_staging_largest_allocation_bytes() -> u64 {
    let count = MAXIMUM_OBSERVATION_READS as u64;
    let selections = count * size_of::<WebObservationSelection>() as u64;
    let addresses = count * size_of::<ObservationAddress>() as u64;
    let rows = count * size_of::<ObservationReadValues>() as u64;
    let results = count * size_of::<WebObservationResult>() as u64;
    let ids = RESPONSE_MAXIMUM_EFFECT_ID_BYTES as u64;
    let mut largest = selections;
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
            SPECTRUM_CAPTURE_BYTES as u64 * 2 + SPECTRUM_STREAM_METADATA_BYTES as u64
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
    static NEXT_HANDLE: Cell<u32> = const { Cell::new(1) };
    static BOOT_STAGING: RefCell<BootStaging> = RefCell::new(BootStaging::new());
    static RESPONSE_STAGING: RefCell<ResponseStaging> = RefCell::new(ResponseStaging::new());
    static SPECTRUM_STAGING: RefCell<SpectrumStaging> = RefCell::new(SpectrumStaging::new());
    static OBSERVATION_STAGING: RefCell<ObservationStaging> = RefCell::new(ObservationStaging::new());
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
        if self.owner_count >= LIVE_RESPONSE_MAXIMUM_OWNERS
            || left.len() > 4
            || right.len() > 4
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
    if count > 4 || record_bytes != LIVE_RESPONSE_SECTION_BYTES {
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
            || raw.left_count > 4
            || raw.right_count > 4
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
    SPECTRUM_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
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
        let selection_epoch_before =
            with_host(handle, 0, AudioWorkletEngineHost::spectrum_selection_epoch);
        let result = with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
            host.select_spectrum(&target, channels)
        });
        let selection_epoch_after =
            with_host(handle, 0, AudioWorkletEngineHost::spectrum_selection_epoch);
        if result == RESULT_OK
            && staging.stream_active
            && selection_epoch_after != selection_epoch_before
        {
            // Selection commits on the host side, so refresh the copied stream profile in the
            // same control operation. The next read may be warming, but it must never publish
            // the old target after a successful switch.
            let selected_target = with_host(handle, 0, AudioWorkletEngineHost::spectrum_target);
            let selected_channels = with_host(handle, 0, AudioWorkletEngineHost::spectrum_channels);
            staging.stream_metadata.target = selected_target;
            staging.stream_metadata.channels = selected_channels;
            staging.stream_metadata.status = SPECTRUM_STREAM_STATUS_WARMING;
            staging.stream_metadata.result = RESULT_OK;
            staging.stream_metadata.capture_epoch =
                with_host(handle, 0, |host| host.spectrum_stream_epoch().unwrap_or(0));
            staging.stream_metadata.sequence = 0;
            staging.stream_metadata.dropped_captures = 0;
            staging.stream_metadata.windows = 0;
            staging.stream_metadata.captured_sample = 0;
            staging.stream_metadata.end_sample = 0;
            staging.stream_metadata.source_underrun = 0;
            staging.capture_len = 0;
            staging.result_len = 0;
            staging.stream_window = None;
        }
        result
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
    let smoothing = match SpectrumSmoothingConfig::new(smoothing_ms) {
        Ok(value) => value,
        Err(_) => return RESULT_INVALID_ARGUMENT,
    };
    SPECTRUM_STAGING.with(|slot| {
        let Ok(mut staging) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
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
            smoothing_ms: smoothing.smoothing_ms(),
            ..staging.stream_metadata
        };
        RESULT_OK
    })
}

/// Read one independently scheduled stream window into the existing fixed capture staging.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_spectrum_stream_read(handle: u32) -> u32 {
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
        RESPONSE_STAGING.with(|staging_slot| {
            let Ok(mut staging) = staging_slot.try_borrow_mut() else {
                return RESULT_INTERNAL;
            };
            run_live_response_capture(&mut live.host, &mut staging)
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

/// Boot the exact staged document and atomically publish the sole running handle.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_boot(len: u32) -> u32 {
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
        match spectrum_request.and_then(|request| {
            AudioWorkletEngineHost::boot_with_spectrum(&staging.document, *staging.options, request)
        }) {
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
        SPECTRUM_STAGING.with(|slot| {
            if let Ok(mut staging) = slot.try_borrow_mut() {
                staging.release_capture();
            }
        });
        return 0;
    };
    LIVE_HOST.with(|slot| {
        let Ok(mut slot) = slot.try_borrow_mut() else {
            return 0;
        };
        if slot.is_some() {
            return 0;
        }
        let handle = next_handle();
        *slot = Some(LiveHost {
            handle,
            host: Box::new(host),
        });
        handle
    })
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
    LIVE_HOST.with(|slot| {
        let Ok(mut slot) = slot.try_borrow_mut() else {
            return RESULT_INVALID_ARGUMENT;
        };
        let Some(live) = slot.as_ref().filter(|live| live.handle == handle) else {
            return RESULT_INVALID_ARGUMENT;
        };
        let _ = live;
        let Some(mut live) = slot.take() else {
            return RESULT_INTERNAL;
        };
        let result = live.host.dispose();
        drop(live);
        SPECTRUM_STAGING.with(|staging| {
            if let Ok(mut staging) = staging.try_borrow_mut() {
                staging.release_capture();
            }
        });
        BOOT_STAGING.with(|staging| {
            if let Ok(mut staging) = staging.try_borrow_mut() {
                staging.reset_after_dispose();
            }
        });
        result
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
mod live_response_ffi_tests {
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

    fn measured<T>(operation: impl FnOnce() -> T) -> (T, u64, u64) {
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
    fn manual_spectrum_cancel_refuses_an_active_managed_stream() {
        SPECTRUM_STAGING.with(|slot| {
            let mut staging = slot.borrow_mut();
            staging.release_capture();
            staging.stream_active = true;
            staging.capture_len = 32;
        });
        assert_eq!(miso_engine_web_v1_spectrum_cancel(0), RESULT_WRONG_STATE);
        SPECTRUM_STAGING.with(|slot| {
            let staging = slot.borrow();
            assert!(staging.stream_active);
            assert_eq!(staging.capture_len, 32);
            assert_eq!(staging.stream_metadata.result, RESULT_WRONG_STATE);
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
        let selected = SPECTRUM_STAGING.with(|slot| slot.borrow().stream_metadata);
        assert_eq!(selected.status, SPECTRUM_STREAM_STATUS_WARMING);
        assert_eq!(selected.target, SPECTRUM_TARGET_OUTPUT);
        assert_eq!(selected.channels, SPECTRUM_CHANNEL_LEFT);
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
