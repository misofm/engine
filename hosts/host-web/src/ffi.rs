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
//! cover the render, command, meter, source, live-control, observation and track-response
//! exports, with the pointer, capacity and byte-count accessors of their stagings that
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
    ObservationReadValues, RESULT_BUFFER_TOO_SMALL, RESULT_INTERNAL, RESULT_INVALID_ARGUMENT,
    RESULT_OK, RESULT_REFUSED_BUDGET, RESULT_REFUSED_DOCUMENT, RESULT_REFUSED_LIFECYCLE,
    RESULT_UNSUPPORTED, RESULT_WRONG_STATE, STATE_READY, WebBootOptions, WebLiveResponseOwner,
    WebLiveResponseRequest, WebLiveResponseResult, WebLiveResponseSection, WebObservationResult,
    WebObservationSelection,
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
    ResponseChannels, ResponsePreviewGrid, ResponseSnapshot, ResponseSnapshotAvailability,
    ResponseSnapshotOwner, ResponseSnapshotQueryError, compose_track_response,
};

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
    reserved2: 0,
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
    static OBSERVATION_STAGING: StagingSlot<ObservationStaging> = const { Cell::new(None) };
    static RESPONSE_FRAME_STAGING: StagingSlot<ResponseFrameStaging> = const { Cell::new(None) };
}

const _: () = assert!(!needs_drop::<LiveHostSlot>());
const _: () = assert!(!needs_drop::<RefCell<BootStaging>>());
const _: () = assert!(!needs_drop::<StagingSlot<ResponseStaging>>());
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
    let _ = observation_staging();
}

#[cfg(test)]
thread_local! {
    static LIVE_RESPONSE_CAPTURE_FAULT: Cell<Option<ResponseSnapshotError>> = const { Cell::new(None) };
    static LIVE_RESPONSE_CAPTURE_OWNER_CALLS: Cell<u32> = const { Cell::new(0) };
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
/// resident-observation and meter-lease bodies write shared staging records, and would record a
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

/// The engine rack of a live-response owner record's `rack`, the inverse of
/// [`live_response_rack_raw`]: the console's `4` reads back as `1`, since the composer names a
/// console slot by its slot ID and not by its section. The retired `1` and `3` are refused.
fn live_response_rack(raw: u32) -> Result<u8, u32> {
    match raw {
        crate::LIVE_RESPONSE_RACK_INPUT_FILTERS => Ok(0),
        crate::LIVE_RESPONSE_RACK_INSERTS => Ok(2),
        crate::LIVE_RESPONSE_RACK_CONSOLE => Ok(1),
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
            // Both console sections share one record code by design, so a console owner reads
            // back as rack `1`; the composer names it by its slot ID. A retired or unknown code
            // refuses the whole snapshot.
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
    let Some(channels) = ResponseChannels::from_raw(request.channels) else {
        return live_response_failure(staging, RESULT_INVALID_ARGUMENT);
    };
    let (snapshot, snapshot_token) = match parse_live_snapshot(staging) {
        Ok(value) => value,
        Err(result) => return live_response_failure(staging, result),
    };
    let response = {
        let Some(track_id) = staging
            .live_track_id
            .get(..request.track_id_bytes as usize)
            .filter(|bytes| !bytes.is_empty())
            .and_then(|bytes| core::str::from_utf8(bytes).ok())
        else {
            return live_response_failure(staging, RESULT_INVALID_ARGUMENT);
        };
        compose_track_response(track_id, &snapshot, grid, channels)
    };
    let response = match response {
        Ok(value) => value,
        Err(error) => return live_response_failure(staging, live_response_error_code(error)),
    };
    let maximum = request.maximum_result_bytes as usize;
    let mut cursor = size_of::<WebLiveResponseResult>();
    let frequencies_offset = match append_live_f32(
        &mut staging.live_result,
        &mut cursor,
        &response.frequencies_hz,
        maximum,
    ) {
        Ok(value) => value,
        Err(result) => return live_response_failure(staging, result),
    };
    let left_offset = match &response.left_db {
        Some(left) => match append_live_f32(&mut staging.live_result, &mut cursor, left, maximum) {
            Ok(value) => value,
            Err(result) => return live_response_failure(staging, result),
        },
        None => 0,
    };
    let right_offset = match &response.right_db {
        Some(right) => {
            match append_live_f32(&mut staging.live_result, &mut cursor, right, maximum) {
                Ok(value) => value,
                Err(result) => return live_response_failure(staging, result),
            }
        }
        None => 0,
    };
    let Ok(owner_count) = u32::try_from(response.members.len()) else {
        return live_response_failure(staging, RESULT_REFUSED_BUDGET);
    };
    staging.live_result_header = WebLiveResponseResult {
        struct_size: LIVE_RESPONSE_RESULT_BYTES,
        abi_version: ABI_VERSION,
        result: RESULT_OK,
        mode: LIVE_RESPONSE_MODE_TARGET,
        meaning: LIVE_RESPONSE_MEANING_EQ_FILTER_SUBTOTAL,
        channels: request.channels,
        points: u32::try_from(points).unwrap_or(0),
        owner_count,
        excluded_count: response.excluded_count,
        sample_rate_hz: response.sample_rate_hz,
        reserved0: 0,
        reserved1: 0,
        captured_sample: response.captured_sample,
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

/// Stage an exact-length replacement document for the live host's next control command (issue
/// #1697, #1386 D7): the live twin of [`miso_engine_web_v1_document_ptr`], on the same staging
/// buffer. The page writes the document there, then stages the `SESSION_REPLACE` frame that names
/// it and calls [`miso_engine_web_v1_control_command`], which passes the document beside the frame
/// to the control plane and frees it after the command, whatever the command was.
///
/// Returns the buffer's address, or zero, with nothing staged, for a wrong handle, a host that is
/// not ready, a `len` that is zero or over [`MAXIMUM_DOCUMENT_BYTES`], or a failed reservation.
/// Every call on the live host first frees what an earlier one staged (the boot's copy too), so a
/// refused call leaves no document for the next command. It never touches the boot result word.
/// Control side only: the control Worker, or in `single` mode the worklet's message handler, never
/// `process()`; the bytes are a counted control allocation.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_replace_document_ptr(handle: u32, len: u32) -> u32 {
    pointer_u32(stage_replace_document(handle, len))
}

/// [`miso_engine_web_v1_replace_document_ptr`]'s staging, with the address as a pointer (null for
/// a refusal), so a native test can tell a staged document from a refusal.
fn stage_replace_document(handle: u32, len: u32) -> *mut u8 {
    with_host(handle, ptr::null_mut(), |host| {
        BOOT_STAGING.with(|staging| {
            let Ok(mut staging) = staging.try_borrow_mut() else {
                return ptr::null_mut();
            };
            // Assignment frees the earlier bytes; the `ManuallyDrop` registers no destructor.
            *staging.document = Vec::new();
            staging.document_valid = false;
            if host.status().state != STATE_READY || len == 0 || len > MAXIMUM_DOCUMENT_BYTES {
                return ptr::null_mut();
            }
            let count = len as usize;
            let mut document = Vec::new();
            if document.try_reserve_exact(count).is_err() {
                return ptr::null_mut();
            }
            document.resize(count, 0);
            *staging.document = document;
            staging.document_valid = true;
            staging.document.as_mut_ptr()
        })
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

fn boot_staged(len: u32) -> u32 {
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

        let result = AudioWorkletEngineHost::boot(&staging.document, staging.options);

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
        return 0;
    };
    let mut host = Some(host);
    // A publication refusal is a real boot failure: the prepared host is disposed off the render
    // path, exactly as for a refused preparation.
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
        return 0;
    };
    reserve_stagings();
    handle
}

/// Boot the exact staged document and atomically publish the sole running handle.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_boot(len: u32) -> u32 {
    boot_staged(len)
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
            if channel_count == 0 || channel_count > plane_slots.len() || frame_count > quantum {
                return host.record_boundary_result(RESULT_INVALID_ARGUMENT);
            }
            if id_count > ids.len() {
                let code = host.refuse_source_id();
                return host.record_boundary_result(code);
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
                let code = host.refuse_source_id();
                return host.record_boundary_result(code);
            };
            let source_id_pointer = source_id.as_ptr();
            // SAFETY: The checked ID prefix lies in stable staging and is only read during the
            // synchronous safe-host lookup; the seek operation does not mutate staging.
            let source_id = unsafe { slice::from_raw_parts(source_id_pointer, id_count) };
            host.seek_source(source_id, generation, source_frame)
        })
    })
}

/// Run `operation` on the first `id_count` bytes of the source ID staging and return its result,
/// or refuse a count past the staging with the core's malformed-ID refusal (`source.id.invalid`,
/// `Err(`[`RESULT_INVALID_ARGUMENT`]`)`; `Err(`[`RESULT_INTERNAL`]`)` with no staging). A refusal
/// is not recorded: the caller records it exactly when its operation would record its own
/// results. The prefix is only read during the synchronous call.
fn with_staged_source_id(
    host: &mut AudioWorkletEngineHost,
    id_count: usize,
    operation: impl FnOnce(&mut AudioWorkletEngineHost, &[u8]) -> u32,
) -> Result<u32, u32> {
    let Some(ids) = host.source_id_mut() else {
        return Err(RESULT_INTERNAL);
    };
    let Some(source_id) = ids.get(..id_count) else {
        return Err(host.refuse_source_id());
    };
    let source_id_pointer = source_id.as_ptr();
    // SAFETY: The checked ID prefix lies in stable staging and is only read during the synchronous
    // safe-host call; neither anchored-seek operation mutates staging.
    let source_id = unsafe { slice::from_raw_parts(source_id_pointer, id_count) };
    Ok(operation(host, source_id))
}

/// Queue one generation-tagged anchored source seek for the staged source ID (issue #1293 D5):
/// `source_frame` enters the graph in the block that starts at absolute render sample
/// `anchor_sample`. It runs the control plane's `SessionState::seek_at`, the call the C ABI's
/// `miso_engine_v1_source_seek_at` runs, so the control Worker (or in `single` mode the worklet's
/// message handler) calls it, never `process()`: it runs the service step. It never prepares the
/// plan's consumer, which the anchor block applies.
///
/// Results: [`RESULT_OK`]; [`RESULT_INVALID_ARGUMENT`] for a wrong handle, an ID count past the
/// staging, an unknown source, an anchor that is not a quantum multiple or any other malformed
/// seek; [`RESULT_BACKPRESSURE`] for a full control queue; [`RESULT_REFUSED_LIFECYCLE`] for a host
/// that is not ready (nothing changes); [`RESULT_INTERNAL`].
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_source_seek_at(
    handle: u32,
    source_id_bytes: u32,
    generation: u64,
    source_frame: u64,
    anchor_sample: u64,
) -> u32 {
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        if host.status().state != STATE_READY {
            return host.seek_source_at(&[], 0, 0, 0);
        }
        // `seek_source_at` records its result, so its staged-ID refusal is recorded too.
        match with_staged_source_id(host, source_id_bytes as usize, |host, source_id| {
            host.seek_source_at(source_id, generation, source_frame, anchor_sample)
        }) {
            Ok(code) => code,
            Err(code) => host.record_boundary_result(code),
        }
    })
}

/// Seek the timeline and every source to timeline frame `frame` under one new `generation`, all or
/// nothing (issue #1643 D1). It runs the control plane's `SessionState::session_seek`, the call the
/// C ABI's `miso_engine_v1_session_seek` runs, so the control Worker (or in `single` mode the
/// worklet's message handler) calls it, never `process()`: it runs the service step. It never
/// prepares the plan's consumer; every consumer observes the seek at the next block. It is a
/// declared discontinuity (root ruling B16-R9): after every check and before any push it declares
/// it as [`miso_engine_web_v1_declare_discontinuity`] does, and a plan that publishes is counted as
/// a blocking rebuild; a refused seek declares nothing, so the page never declares before it.
///
/// Results: [`RESULT_OK`]; [`RESULT_INVALID_ARGUMENT`] for a wrong handle, a generation that is
/// zero or not above every generation the session has published (`source.generation.stale`) or any
/// other malformed seek; [`RESULT_BACKPRESSURE`] when one consumer's slot is full (nothing
/// changes); [`RESULT_REFUSED_LIFECYCLE`] for a host that is not ready (nothing changes);
/// [`RESULT_INTERNAL`]; and the refusals of [`miso_engine_web_v1_declare_discontinuity`] for its
/// declaration, with the core's diagnostic at [`miso_engine_web_v1_control_response_ptr`] and
/// nothing pushed.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_session_seek(handle: u32, generation: u64, frame: u64) -> u32 {
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        host.session_seek(generation, frame, None)
    })
}

/// The anchored form of [`miso_engine_web_v1_session_seek`] (issue #1643 D1): every consumer
/// applies the seek in the block that starts at absolute render sample `anchor`, which must be a
/// quantum multiple. Same function, same results and same thread rule as the C ABI's
/// `miso_engine_v1_session_seek_at`. It declares no discontinuity: the loop and exact-start form
/// keeps continuity (root ruling B16-R9).
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_session_seek_at(
    handle: u32,
    generation: u64,
    frame: u64,
    anchor: u64,
) -> u32 {
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        host.session_seek(generation, frame, Some(anchor))
    })
}

/// Copy where the staged source landed after its newest seek into the fixed 64-byte record at
/// [`miso_engine_web_v1_source_seek_report_ptr`] (issue #1293 D6): the generation in effect, the
/// source-read sample and source frame of its first block, and the cumulative underrun and held
/// frames. It runs the control plane's `SessionState::seek_report`, the call the C ABI's
/// `miso_engine_v1_source_seek_report` runs, so it is control side only.
///
/// The reserved source ID `$timeline` (9 bytes) reads the session timeline's report instead
/// (#1704): its source frame is the timeline sample that block started at, and its underrun and
/// held counts are 0. Only this call takes it; submit, seek and seek-at refuse it as an unknown
/// source.
///
/// Results: [`RESULT_OK`]; [`RESULT_INVALID_ARGUMENT`] for a wrong handle, an ID count past the
/// staging or an unknown source; [`RESULT_BACKPRESSURE`] for a read that landed inside render's
/// publication (read again); [`RESULT_REFUSED_LIFECYCLE`] for a host that is not ready;
/// [`RESULT_INTERNAL`]. A refusal leaves the record alone, and no result changes the status's last
/// result.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_source_seek_report(handle: u32, source_id_bytes: u32) -> u32 {
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        if host.status().state != STATE_READY {
            return host.read_seek_report(&[]);
        }
        // `read_seek_report` leaves the status's last result alone, so its staged-ID refusal
        // does too.
        let (Ok(code) | Err(code)) =
            with_staged_source_id(host, source_id_bytes as usize, |host, source_id| {
                host.read_seek_report(source_id)
            });
        code
    })
}

/// Return the stable address of the 64-byte seek report record, or zero for an invalid handle or a
/// host that is not ready (issue #1293 D6).
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_source_seek_report_ptr(handle: u32) -> u32 {
    with_host(handle, 0, |host| pointer_u32(seek_report_pointer(host)))
}

/// The seek report record's address for a ready host, else null (#1293 D6).
fn seek_report_pointer(host: &AudioWorkletEngineHost) -> *const crate::WebSeekReport {
    if host.status().state == STATE_READY {
        ptr::from_ref(host.seek_report())
    } else {
        ptr::null()
    }
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

/// Run the control plane's service step once (issue #1381a D5, #1348 D1): reclaim every plan and
/// attachment render has displaced, in the caller's thread and never in render. Control side
/// only: the control Worker, or in `single` mode the worklet's message handler, never `process()`.
///
/// It commits and acknowledges nothing and leaves the status's last result alone. Results:
/// [`RESULT_OK`]; [`RESULT_INVALID_ARGUMENT`] for a wrong handle; [`RESULT_WRONG_STATE`] for a host
/// that is not ready; an epoch failure as [`RESULT_INTERNAL`] with the session's diagnostic.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_service(handle: u32) -> u32 {
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| host.service())
}

/// Declare a discontinuity (issue #1323): the next block need not continue the previous output,
/// because the page has stopped or is about to seek every source. The page calls it at a stop and
/// before a seek of every source made with per-source seeks; a plain
/// [`miso_engine_web_v1_session_seek`] declares it in the same call (root ruling B16-R9). It runs the control plane's `SessionState::declare_discontinuity`,
/// the function the C ABI's `miso_engine_v1_declare_discontinuity` runs, so both hosts reset the
/// latency floors alike. Control side only: the control Worker, or in `single` mode the worklet's
/// message handler, never `process()`; a plan it publishes is counted as a blocking rebuild.
///
/// Results: [`RESULT_OK`] (also when nothing needed a reset); [`RESULT_INVALID_ARGUMENT`] for a
/// wrong handle; [`RESULT_WRONG_STATE`] for a host that is not ready; and the refusals of
/// [`miso_engine_web_v1_control_command`] for a rebuild, with the core's diagnostic at
/// [`miso_engine_web_v1_control_response_ptr`]. A refusal changes nothing.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_declare_discontinuity(handle: u32) -> u32 {
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        host.declare_discontinuity()
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
/// `SessionState::command_with_document`, the entry the C ABI's `miso_engine_v1_submit_command`
/// and `miso_engine_v1_submit_document_command` run (issue #1562a D1, #1697). The document
/// [`miso_engine_web_v1_replace_document_ptr`] staged, if any, goes beside the frame (a
/// `SESSION_REPLACE` reads it, every other command ignores it); after the command, whatever its
/// frame or result (a wrong handle's refusal too, while a host is live), the staging is freed, so
/// a document serves exactly one command and the first command after boot frees the boot's copy. In `single` mode it runs in the worklet's message
/// handler, never in `process()`; it is not render-locked, so its allocations are counted as the
/// instance's control allocations.
///
/// Results: [`RESULT_OK`] (the response frame is at [`miso_engine_web_v1_control_response_ptr`],
/// and a refused transaction is still an ok call with a non-ok frame); [`RESULT_INVALID_ARGUMENT`]
/// for a wrong handle, a bad length or a frame the core cannot correlate;
/// [`RESULT_WRONG_STATE`]; [`RESULT_BUFFER_TOO_SMALL`]; [`RESULT_BACKPRESSURE`];
/// [`crate::RESULT_COMPILE_REJECTED`] for a typed refusal before commit (`web.apply.shape_unavailable`);
/// [`RESULT_INTERNAL`]. After any failure the response is the refusal's diagnostic.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_control_command(handle: u32, len: u32) -> u32 {
    let mut ran = false;
    let result = with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        ran = true;
        BOOT_STAGING.with(|staging| {
            let Ok(mut staging) = staging.try_borrow_mut() else {
                return RESULT_INTERNAL;
            };
            let staging = &mut *staging;
            let document: &[u8] = if staging.document_valid {
                &staging.document
            } else {
                &[]
            };
            let result = host.control_command_with_document(len, document);
            // #1386 D7: no host keeps the document after the call.
            *staging.document = Vec::new();
            staging.document_valid = false;
            result
        })
    });
    // A wrong handle runs no command, yet a document `replace_document_ptr` staged must not
    // survive it either. Only while a host is live: before boot the staging is the boot's document
    // or a refused boot's diagnostic, which no control call may free.
    let live = LIVE_HOST.with(|slot| slot.try_borrow().is_ok_and(|slot| slot.is_some()));
    if !ran && live {
        BOOT_STAGING.with(|staging| {
            if let Ok(mut staging) = staging.try_borrow_mut() {
                *staging.document = Vec::new();
                staging.document_valid = false;
            }
        });
    }
    result
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

/// Copy the active plan's output latency and tail into their fixed record (issue #1688 D2),
/// found at [`miso_engine_web_v1_plan_latency_ptr`]. Control side only.
///
/// Results: [`RESULT_OK`]; [`RESULT_INVALID_ARGUMENT`] for a wrong handle, which writes nothing;
/// [`RESULT_WRONG_STATE`] for a host that is not ready, which leaves the record alone. The
/// record's words are the control plane's row for the plan that renders now; a rebuild's
/// successor shows from the first read after the block that adopts it.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_plan_latency_read(handle: u32) -> u32 {
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        host.read_plan_latency()
    })
}

/// Return the stable address of the 64-byte plan latency record or zero for an invalid handle.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_plan_latency_ptr(handle: u32) -> u32 {
    with_host(handle, 0, |host| {
        pointer_u32(ptr::from_ref(host.plan_latency()))
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

/// Return the per-source ring frames the boot resolved, or zero for an invalid handle (#1406b).
///
/// A zero `sourceRingFrames` boot option selects the engine's default ring; this is the one place
/// a host reads the ring it got, so no host holds a copy of the rule.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_resolved_source_ring_frames(handle: u32) -> u32 {
    render_locked(|| with_host(handle, 0, AudioWorkletEngineHost::source_ring_frames))
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
            boot.reset_after_dispose();
            result
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
    use super::{WebBootOptions, WebLiveResponseRequest, WebObservationSelection};

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

    /// Whether no seek waits in a session seek's command slots, for the host `handle` names
    /// (batch 16b review, MINOR f); `None` for an unknown handle or one with no control half.
    #[cfg(feature = "test-support")]
    pub fn session_seek_slots_empty(handle: u32) -> Option<bool> {
        super::with_host(handle, None, |host| {
            host.session_seek_slots_empty_for_test()
        })
    }

    /// The session timeline's sample at `source_read_sample` in the plan that renders, for the
    /// host `handle` names (#1643 gates); `None` for an unknown handle.
    #[cfg(feature = "test-support")]
    pub fn timeline_at(handle: u32, source_read_sample: u64) -> Option<i64> {
        super::with_host_mut(handle, None, |host| {
            host.timeline_at_for_test(source_read_sample)
        })
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

    /// The seek report record the host `handle` names last copied (#1293 D6), as the worklet reads
    /// it through `miso_engine_web_v1_source_seek_report_ptr`; `None` for an unknown handle.
    pub fn seek_report(handle: u32) -> Option<crate::WebSeekReport> {
        super::with_host(handle, None, |host| Some(*host.seek_report()))
    }

    /// The plan latency record the host `handle` names last copied (#1688), as the worklet reads
    /// it through `miso_engine_web_v1_plan_latency_ptr`; `None` for an unknown handle.
    pub fn plan_latency(handle: u32) -> Option<crate::WebPlanLatency> {
        super::with_host(handle, None, |host| Some(*host.plan_latency()))
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

/// Stage `bytes` through [`miso_engine_web_v1_replace_document_ptr`]'s staging and write them
/// there, as the worklet does (#1697); `false` when the staging refused the length.
#[cfg(test)]
pub(crate) fn test_replace_document(handle: u32, bytes: &[u8]) -> bool {
    let Ok(length) = u32::try_from(bytes.len()) else {
        return false;
    };
    if stage_replace_document(handle, length).is_null() {
        return false;
    }
    BOOT_STAGING.with(|staging| staging.borrow_mut().document.copy_from_slice(bytes));
    true
}

/// The document staging as `(document_valid, length, capacity)` (#1697 gate 2).
#[cfg(test)]
pub(crate) fn test_document_staging() -> (bool, usize, usize) {
    BOOT_STAGING.with(|staging| {
        let staging = staging.borrow();
        (
            staging.document_valid,
            staging.document.len(),
            staging.document.capacity(),
        )
    })
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

/// The address `miso_engine_web_v1_source_seek_report_ptr` returns before its `u32` cast (a
/// native address does not fit it), and the record's own address.
#[cfg(test)]
pub(crate) fn test_seek_report_addresses(handle: u32) -> (usize, usize) {
    with_host(handle, (0, 0), |host| {
        (
            seek_report_pointer(host).addr(),
            ptr::from_ref(host.seek_report()).addr(),
        )
    })
}

/// The bytes `miso_engine_web_v1_control_response_ptr/_bytes` name, read before the `u32` cast.
#[cfg(test)]
pub(crate) fn test_control_response(handle: u32) -> Vec<u8> {
    with_host(handle, Vec::new(), |host| host.control_response().to_vec())
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
                .as_chunks::<{ size_of::<f32>() }>()
                .0
                .iter()
                .skip(size_of::<WebLiveResponseResult>() / size_of::<f32>())
                .take(15)
                .map(|chunk| f32::from_le_bytes(*chunk));
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
mod observation_alias_tests {
    //! The ordinary observation aliases' live-host guard and the resident staging accounting.
    use super::*;

    pub(super) fn no_live_host() {
        assert!(LIVE_HOST.with(|slot| slot.borrow().is_none()));
    }

    fn boot_ordinary() -> u32 {
        no_live_host();
        let document = include_bytes!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
        let handle = test_boot(document, WebBootOptions::explicit_defaults());
        assert_ne!(handle, 0, "ordinary fixture must boot");
        handle
    }

    fn dispose(handle: u32) {
        assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
        no_live_host();
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
    /// console selection over `pre_insert` alone (the observation index then finds no `comp` and
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
                    serde_json::json!({ "slot": "eq", "bypass": false, "link_mode": "slot", "params": [] }),
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
        let resident_markers = stage_resident_markers();

        LIVE_HOST.with(|slot| {
            let _borrow = slot.borrow();
            assert_eq!(
                miso_engine_web_v1_observation_read(handle, u32::MAX),
                RESULT_INTERNAL
            );
            assert_eq!(miso_engine_web_v1_meter_lease(handle, 2), RESULT_INTERNAL);
        });

        assert_resident_markers(&resident_markers);
        dispose(handle);
    }

    /// Issue #1406b: the export answers the ring the boot resolved, the engine's default ring on a
    /// zero override and the override itself otherwise, and zero without a live handle.
    #[test]
    fn source_ring_frames_export_reports_the_resolved_ring() {
        no_live_host();
        assert_eq!(miso_engine_web_v1_resolved_source_ring_frames(0), 0);
        let document = include_bytes!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
        for (override_frames, expected) in [
            (0, crate::default_source_ring_frames(48_000, 128)),
            (1_280, 1_280),
        ] {
            let options = WebBootOptions {
                require_sample_rate_hz: 48_000,
                require_quantum_frames: 128,
                source_ring_frames: override_frames,
                ..WebBootOptions::explicit_defaults()
            };
            let handle = test_boot(document, options);
            assert_ne!(handle, 0, "boot with ring override {override_frames}");
            assert_eq!(
                miso_engine_web_v1_resolved_source_ring_frames(handle),
                expected
            );
            assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
            assert_eq!(miso_engine_web_v1_resolved_source_ring_frames(handle), 0);
        }
    }

    #[test]
    fn invalid_alias_handle_is_terminal_before_any_staging_work() {
        let handle = boot_ordinary();
        let resident_markers = stage_resident_markers();
        assert_eq!(
            miso_engine_web_v1_observation_read(0, u32::MAX),
            RESULT_INVALID_ARGUMENT
        );
        assert_resident_markers(&resident_markers);
        dispose(handle);
    }
}
