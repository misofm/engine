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
//! cover the render, meter, source and live-control exports, with the pointer,
//! capacity and byte-count accessors of their stagings that the worklets call after boot (issues
//! #1488 and #1492). The two entries with `wrapped: false`
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

use crate::handoff;
use crate::render_lock::{control_allocation_count, render_allocation_count, render_locked};
// Named by the exports' result docs only.
use crate::{
    ABI_VERSION, AudioWorkletEngineHost, BUFFER_DIAGNOSTIC, BUFFER_METER_FRAME, BUFFER_OUTPUT_PCM,
    BUFFER_SOURCE_ID, BUFFER_SOURCE_PCM, BootFailure, CONTROL_FRAME_BYTES, MAXIMUM_DOCUMENT_BYTES,
    RESULT_INTERNAL, RESULT_INVALID_ARGUMENT, RESULT_OK, RESULT_REFUSED_BUDGET,
    RESULT_REFUSED_DOCUMENT, RESULT_REFUSED_LIFECYCLE, RESULT_WRONG_STATE, RenderHost, STATE_READY,
    WebBootOptions, WebStatus,
};
#[cfg(doc)]
use crate::{RESULT_BACKPRESSURE, RESULT_BUFFER_TOO_SMALL};
use core::{
    cell::{Cell, RefCell},
    mem::{ManuallyDrop, MaybeUninit, needs_drop},
    ptr, slice,
};

/// The instance's one live session (#1332 S6). A booted instance holds the host, with its render
/// half here (`single` mode) or released to another instance (the Worker, `worker` mode); a
/// render-only instance (the worklet in `worker` mode) holds only the render half it adopted, under
/// the handle of the host it belongs to.
struct LiveHost {
    handle: u32,
    /// The booted host; `None` on a render-only instance.
    host: Option<Box<AudioWorkletEngineHost>>,
    /// The adopted render half; `Some` only on a render-only instance.
    render: Option<RenderHost>,
}

/// Where a render export finds the render half (#1332 S6): in the booted host (whose released
/// shell refuses), or the render-only instance's adopted half.
enum RenderTarget<'a> {
    Host(&'a mut AudioWorkletEngineHost),
    Half(&'a mut RenderHost),
}

/// Boot staging is a const-initialised, destructor-free thread local (issue #1333 D5): `options`
/// is inline, and `document` is freed only by assignment, at a staged replacement or after
/// dispose, never by a thread-local destructor.
struct BootStaging {
    options: WebBootOptions,
    document: ManuallyDrop<Vec<u8>>,
    /// The boot command frame `miso_engine_web_v1_boot_command_ptr` staged (#2166), empty when none
    /// is: the next boot takes it, whatever its outcome.
    command: ManuallyDrop<Vec<u8>>,
    result: u32,
    diagnostic_bytes: u32,
    document_valid: bool,
}

/// The session-less response preview's request and reply frames (#1574). It is allocated on first
/// touch and never reserved by a boot: the preview instance never boots, and no worklet calls it.
#[derive(Default)]
struct ResponseFrameStaging {
    request: Vec<u8>,
    reply: Vec<u8>,
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
    reserved3: 0,
    reserved2: 0,
};

impl BootStaging {
    const fn new() -> Self {
        Self {
            options: ZERO_BOOT_OPTIONS,
            document: ManuallyDrop::new(Vec::new()),
            command: ManuallyDrop::new(Vec::new()),
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
        *self.command = Vec::new();
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
    // handle contract when the first owner is published.
    static NEXT_HANDLE: Cell<u32> = const { Cell::new(0) };
    static BOOT_STAGING: RefCell<BootStaging> = const { RefCell::new(BootStaging::new()) };
    static RESPONSE_FRAME_STAGING: StagingSlot<ResponseFrameStaging> = const { Cell::new(None) };
}

const _: () = assert!(!needs_drop::<LiveHostSlot>());
const _: () = assert!(!needs_drop::<RefCell<BootStaging>>());
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

fn with_response_frame_staging<R>(
    operation: impl FnOnce(&RefCell<ResponseFrameStaging>) -> R,
) -> R {
    operation(leaked_staging(
        &RESPONSE_FRAME_STAGING,
        ResponseFrameStaging::default,
    ))
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
        let Some(host) = slot
            .as_ref()
            .filter(|live| live.handle == handle)
            .and_then(|live| live.host.as_deref())
        else {
            return invalid;
        };
        operation(host)
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
        let Some(host) = slot
            .as_mut()
            .filter(|live| live.handle == handle)
            .and_then(|live| live.host.as_deref_mut())
        else {
            return invalid;
        };
        operation(host)
    })
}

/// The render half `handle` names, wherever it is on this instance (#1332 S6).
fn with_render_mut<R>(handle: u32, invalid: R, operation: impl FnOnce(RenderTarget<'_>) -> R) -> R {
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
        match (live.host.as_deref_mut(), live.render.as_mut()) {
            (Some(host), _) => operation(RenderTarget::Host(host)),
            (None, Some(render)) => operation(RenderTarget::Half(render)),
            (None, None) => invalid,
        }
    })
}

/// The render half `handle` names, read-only (#1332 S6).
fn with_render<R>(
    handle: u32,
    invalid: R,
    operation: impl FnOnce(&WebStatus, &crate::WebMeterHeader, &[f32], Option<&[f32]>) -> R,
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
        match (live.host.as_deref(), live.render.as_ref()) {
            (Some(host), _) => operation(
                host.status(),
                host.meter_header(),
                host.meter_frame(),
                host.output_pcm(),
            ),
            (None, Some(render)) => operation(
                render.status(),
                render.meter_header(),
                render.meter_frame(),
                render.output_pcm(),
            ),
            (None, None) => invalid,
        }
    })
}

/// Verify that `handle` names the live host before an alias touches caller-owned staging.
///
/// A failed live-host borrow and every nonmatching handle are terminal at the ABI boundary: the
/// meter-lease body records its refusal on the host, so it runs only after the matching host has
/// been inspected. The check takes
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

/// [`buffer_pointer`] on a render-only instance (#1332 S6): the render half's two buffers.
fn render_buffer_pointer(render: &RenderHost, kind: u32) -> *mut u8 {
    match kind {
        BUFFER_OUTPUT_PCM => render
            .output_pcm()
            .map_or(ptr::null_mut(), |value| value.as_ptr().cast_mut().cast()),
        BUFFER_METER_FRAME => {
            let frame = render.meter_frame();
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
        BUFFER_METER_FRAME => (host.meter_frame().len() * 4) as u64,
        _ => return 0,
    };
    u32::try_from(bytes).unwrap_or(0)
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
/// `singleModeControlAllocations`. Since #1332 S6 the count is the calling instance's own, as the
/// render-locked count is: on one shared memory the Worker's and the worklet's are separate.
/// Control side only, never `process()`.
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
        // #2166: a staged boot command belongs to its document; staging a document frees any
        // command staged before it, so the page stages the command after the document.
        *staging.command = Vec::new();
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

/// Stage an exact-length boot command frame for the next boot (#2166): one encoded
/// `SPECTRUM_CONFIGURE` command frame, which the control plane applies before it prepares the first
/// plan ([`control_plane::compile_children_with_boot_command`]), so the boot's spectrum taps are
/// that plan's and the boot counts no blocking rebuild. Optional: a boot with none staged has no
/// taps. Call it after `document_ptr` (which frees a command staged before it) and before `boot`;
/// the boot takes the frame whatever its outcome, and dispose frees one no boot took.
///
/// Returns the buffer's address, or zero, with nothing staged, while a host is live, for a `len`
/// that is zero or over the control frame ceiling, or for a failed reservation. Every call first
/// frees what an earlier one staged. It never touches the boot result word. Control side only (the
/// boot's own staging), never `process()`.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_boot_command_ptr(len: u32) -> u32 {
    pointer_u32(stage_boot_command(len))
}

/// [`miso_engine_web_v1_boot_command_ptr`]'s staging, with the address as a pointer (null for a
/// refusal).
fn stage_boot_command(len: u32) -> *mut u8 {
    let live = LIVE_HOST.with(|slot| slot.try_borrow().map_or(true, |slot| slot.is_some()));
    BOOT_STAGING.with(|staging| {
        let Ok(mut staging) = staging.try_borrow_mut() else {
            return ptr::null_mut();
        };
        // Assignment frees the earlier bytes; the `ManuallyDrop` registers no destructor.
        *staging.command = Vec::new();
        if live || len == 0 || u64::from(len) > CONTROL_FRAME_BYTES {
            return ptr::null_mut();
        }
        let count = len as usize;
        let mut command = Vec::new();
        if command.try_reserve_exact(count).is_err() {
            return ptr::null_mut();
        }
        command.resize(count, 0);
        *staging.command = command;
        staging.command.as_mut_ptr()
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
///
/// On a render-only instance (#1332 S6) only the render half's buffers answer:
/// [`BUFFER_OUTPUT_PCM`] and [`BUFFER_METER_FRAME`].
pub extern "C" fn miso_engine_web_v1_buffer_ptr(handle: u32, kind: u32) -> u32 {
    with_render_mut(handle, 0, |target| match target {
        RenderTarget::Host(host) => pointer_u32(buffer_pointer(host, kind)),
        RenderTarget::Half(render) => pointer_u32(render_buffer_pointer(render, kind)),
    })
}

/// Return one prepared staging-buffer capacity in bytes or zero.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_buffer_capacity(handle: u32, kind: u32) -> u32 {
    with_render_mut(handle, 0, |target| match target {
        RenderTarget::Host(host) => buffer_capacity(host, kind),
        RenderTarget::Half(render) => {
            let words = match kind {
                BUFFER_OUTPUT_PCM => render.output_pcm().map_or(0, <[f32]>::len),
                BUFFER_METER_FRAME => render.meter_frame().len(),
                _ => 0,
            };
            u32::try_from(words * 4).unwrap_or(0)
        }
    })
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
        // #2166: the boot takes the staged command, whatever its outcome.
        let command = core::mem::take(&mut *staging.command);
        if !staging.document_valid || staging.document.len() != len as usize {
            staging.result = RESULT_REFUSED_DOCUMENT;
            staging.diagnostic_bytes = 0;
            staging.document_valid = false;
            return None;
        }

        let result = AudioWorkletEngineHost::boot_with_command(
            &staging.document,
            staging.options,
            (!command.is_empty()).then_some(command.as_slice()),
        );
        drop(command);

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
            host: Some(Box::new(
                host.take()
                    .expect("boot host is present before publication"),
            )),
            render: None,
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
        with_render_mut(handle, RESULT_INVALID_ARGUMENT, |target| match target {
            RenderTarget::Host(host) => {
                if host.status().state == STATE_READY
                    && host.status().quantum_frames != actual_frames
                {
                    return host.reject_output_quantum(actual_frames);
                }
                host.render_next()
            }
            RenderTarget::Half(render) => {
                if render.status().state == STATE_READY {
                    // #1332 S6: a render-only instance runs on a reserved stack with no guard
                    // page; its canary is checked before every block, and the first bad one fails
                    // the session.
                    if !handoff::bound_canary_intact() {
                        return render.fail(RESULT_INTERNAL, STACK_CANARY_DIAGNOSTIC);
                    }
                    if render.status().quantum_frames != actual_frames {
                        return render.reject_output_quantum(actual_frames);
                    }
                }
                render.render_next()
            }
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

/// Take (`1`) or release (`0`) the decimated meter lease (issue #137 D2).
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_meter_lease(handle: u32, enabled: u32) -> u32 {
    render_locked(|| {
        if let Err(result) = verify_live_host(handle) {
            return result;
        }
        with_render_mut(handle, RESULT_INVALID_ARGUMENT, |target| match target {
            RenderTarget::Host(host) => {
                if enabled > 1 {
                    return host.record_boundary_result(RESULT_INVALID_ARGUMENT);
                }
                host.set_meter_lease(enabled == 1)
            }
            RenderTarget::Half(_) if enabled > 1 => RESULT_INVALID_ARGUMENT,
            RenderTarget::Half(render) => render.set_meter_lease(enabled == 1),
        })
    })
}

/// Drain finished meter windows into the frame buffer; returns the number of complete windows.
///
/// Called from `process()` after the render export, so it is allocation-free and bounded: it moves
/// `Copy` snapshots out of queues sized at compilation into a buffer sized at compilation.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_meter_poll(handle: u32) -> u32 {
    render_locked(|| {
        with_render_mut(handle, 0, |target| match target {
            RenderTarget::Host(host) => host.poll_meters(),
            RenderTarget::Half(render) => render.poll_meters(),
        })
    })
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
///
/// #1332 S7 (root ruling R-S7-meters): in `worker` mode the worklet's `process()` calls it after
/// every render under the meter lease, to learn which header render reports as current, so it is
/// render-locked and held to the call-graph gate (no allocator, free or drop glue).
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_meter_header_ptr(handle: u32) -> u32 {
    render_locked(|| {
        with_render(handle, 0, |_, header, _, _| {
            pointer_u32(ptr::from_ref(header))
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
///
/// #1332 S6: on a render-only instance it is the adopted half's status (its clock, its counters
/// and its last result). On an instance whose render half is released it is the status shell the
/// release left: from the release on, its `state` is the session's lifecycle word as of the last
/// control call, and its `last_result` and `blocking_rebuilds` are the control half's, while the
/// render clock and counters stay as they were at the release.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_status_ptr(handle: u32) -> u32 {
    with_render(handle, 0, |status, _, _, _| {
        pointer_u32(ptr::from_ref(status))
    })
}

/// Quiescently dispose the live handle; zero is an explicit no-op.
///
/// #1332 S8: on an instance that bound no reservation, a render-only session (a half adopted
/// after its host was disposed) is freed; a bound (render-only) instance refuses with
/// [`RESULT_WRONG_STATE`] and frees nothing.
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
            // #1332 S6: a render-only instance holds no host; its half goes back by a release.
            // #1332 S8: an orphaned half -- the Worker disposed its host while the half was
            // away, then adopted the half when it came back -- is freed here: only an instance
            // that bound no reservation (`instance_bind` marks the render-only one) frees it,
            // never the worklet, whose render thread must not free.
            if live.host.is_none() {
                if handoff::instance_bound() {
                    return RESULT_WRONG_STATE;
                }
                drop(live_slot.take());
                boot.reset_after_dispose();
                return RESULT_OK;
            }
            let Some(mut live) = live_slot.take() else {
                return RESULT_INTERNAL;
            };
            let result = live
                .host
                .as_mut()
                .map_or(RESULT_INTERNAL, |host| host.dispose());
            drop(live);
            boot.reset_after_dispose();
            result
        })
    })
}

/// Release the render half of the host `handle` names and return its token, or `0` (#1332 S6).
///
/// On the instance that booted the host (the Worker), the render half leaves the host, which keeps
/// its control half and a status shell (see [`miso_engine_web_v1_status_ptr`]); every render
/// export on that host then refuses with [`RESULT_WRONG_STATE`] (the poll answers `0`). On a
/// render-only instance (the worklet), the half it adopted leaves the instance, which holds no
/// session afterwards; first the instance's reserved stack canary is checked
/// ([`miso_engine_web_v1_instance_bind`]), and an overwritten canary fails the session
/// (`web.instance.stack_canary`, sticky [`RESULT_INTERNAL`]) before the half goes, so the Worker
/// that adopts it back sees the failure. Nothing is freed: the half is deposited in the handoff
/// registry, and the token is the only way back to it. `0` for a wrong handle, a host whose half
/// is already away or that has no session, or a full registry (the half stays where it was).
///
/// Render-locked: it allocates and frees nothing, wherever it runs.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_host_release(handle: u32) -> u64 {
    render_locked(|| release_render_half(handle))
}

/// The diagnostic of a render-only session whose reserved stack's canary was overwritten.
const STACK_CANARY_DIAGNOSTIC: &[u8] = b"web.instance.stack_canary\t$\n";

fn release_render_half(handle: u32) -> u64 {
    if handle == 0 {
        return 0;
    }
    LIVE_HOST.with(|slot| {
        let Ok(mut slot) = slot.try_borrow_mut() else {
            return 0;
        };
        let Some(live) = slot.as_mut().filter(|live| live.handle == handle) else {
            return 0;
        };
        if let Some(host) = live.host.as_deref_mut() {
            let Some(render) = host.release_render() else {
                return 0;
            };
            return match handoff::deposit(handle, render) {
                Ok(token) => token,
                Err(render) => {
                    // A full registry: the half goes back where it was. It is this host's, so the
                    // adoption cannot refuse it; were it to, the half is forgotten, never dropped
                    // here.
                    if let Err(render) = host.adopt_render(render) {
                        core::mem::forget(render);
                    }
                    0
                }
            };
        }
        let Some(mut render) = live.render.take() else {
            return 0;
        };
        if !handoff::bound_canary_intact() {
            let _ = render.fail(RESULT_INTERNAL, STACK_CANARY_DIAGNOSTIC);
        }
        match handoff::deposit(handle, render) {
            Ok(token) => {
                // The instance holds no session now. What leaves the slot holds neither a host
                // nor a half, so it is forgotten: no drop glue joins the call.
                let mut empty = None;
                core::mem::swap(&mut **slot, &mut empty);
                core::mem::forget(empty);
                token
            }
            Err(render) => {
                let mut back = Some(render);
                core::mem::swap(&mut live.render, &mut back);
                core::mem::forget(back);
                0
            }
        }
    })
}

/// Adopt the render half `token` names into this instance (#1332 S6).
///
/// It stores the half only into an empty slot: on an instance with no session (the worklet), the
/// half becomes the instance's render-only session, under the handle of the host it belongs to
/// (`token >> 32`), and the render exports serve it; on the instance whose host released it (the
/// Worker), the host takes it back, and its render exports serve it again. Results:
/// [`RESULT_OK`], the token consumed; [`RESULT_WRONG_STATE`] for an instance whose slot is taken (a
/// render-only session, or a host whose render half is here), the token unconsumed;
/// [`RESULT_INVALID_ARGUMENT`] for a token no release issued or one already taken, and for a half
/// another host released (unconsumed); [`RESULT_INTERNAL`] for a refused borrow.
///
/// Render-locked: it allocates and frees nothing, wherever it runs.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_host_adopt(token: u64) -> u32 {
    render_locked(|| adopt_render_half(token))
}

fn adopt_render_half(token: u64) -> u32 {
    let handle = handoff::token_handle(token);
    if handle == 0 {
        return RESULT_INVALID_ARGUMENT;
    }
    LIVE_HOST.with(|slot| {
        let Ok(mut slot) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        if let Some(live) = slot.as_mut() {
            let Some(host) = live.host.as_deref_mut() else {
                return RESULT_WRONG_STATE;
            };
            if host.render_here() {
                return RESULT_WRONG_STATE;
            }
            return match handoff::take(token, |render| host.adopt_render(render)) {
                Some(Ok(())) => RESULT_OK,
                Some(Err(())) | None => RESULT_INVALID_ARGUMENT,
            };
        }
        match handoff::take(token, Ok) {
            Some(Ok(render)) => {
                let mut fresh = Some(LiveHost {
                    handle,
                    host: None,
                    render: Some(render),
                });
                core::mem::swap(&mut **slot, &mut fresh);
                // The slot was empty (checked above): nothing to drop.
                core::mem::forget(fresh);
                RESULT_OK
            }
            Some(Err(())) | None => RESULT_INVALID_ARGUMENT,
        }
    })
}

/// Allocate one stack and TLS block for a second instance on this memory and return the address
/// of its 16-byte record (`WebInstanceReservation`: `struct_size`, `tls_base`, `stack_top`,
/// `stack_bottom`, little-endian `u32` words), or `0` (#1332 S6, #1331 recipe rule 2).
///
/// `tls_size` and `tls_align` are the module's exported `__tls_size` and `__tls_align` globals
/// (a Wasm global is not addressable from Rust, so the page passes them). The stack is 2 MiB
/// (owner decision O6) with a 64-byte canary below it, and the block is leaked for the instance's
/// life. The page then starts the second instance with `__wasm_init_tls(tls_base)` and
/// `__stack_pointer = stack_top`, before any other export, and binds it with
/// [`miso_engine_web_v1_instance_bind`]. Control work: the first instance calls it, before the
/// second starts; it allocates.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_instance_reserve(tls_size: u32, tls_align: u32) -> u32 {
    handoff::reserve(tls_size, tls_align).map_or(0, |reservation| {
        u32::try_from(reservation.record).unwrap_or(0)
    })
}

/// Bind this instance to the reservation whose record is at `record` (#1332 S6), so that
/// [`miso_engine_web_v1_host_release`] checks its stack canary. [`RESULT_OK`] when this module
/// made the reservation and its canary is intact; [`RESULT_INVALID_ARGUMENT`] otherwise, which
/// binds nothing. The second instance calls it once, after its stack and TLS are set and before
/// [`miso_engine_web_v1_host_adopt`]. Render-locked: loads and compares only.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_instance_bind(record: u32) -> u32 {
    render_locked(|| {
        if handoff::bind(record as usize) {
            RESULT_OK
        } else {
            RESULT_INVALID_ARGUMENT
        }
    })
}

/// The address of the fixed diagnostic of the render-only session's sticky failure, or `0` when
/// it has none or this instance holds no render-only session (#1332 S6). The worklet reads it
/// after a render failure; on the instance that holds the host, the session diagnostic
/// ([`BUFFER_DIAGNOSTIC`]) carries a render failure instead. Render-locked.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_render_diagnostic_ptr(handle: u32) -> u32 {
    render_locked(|| {
        with_render_mut(handle, 0, |target| match target {
            RenderTarget::Half(render) if !render.diagnostic().is_empty() => {
                pointer_u32(render.diagnostic().as_ptr())
            }
            _ => 0,
        })
    })
}

/// The byte length of [`miso_engine_web_v1_render_diagnostic_ptr`]'s text, or `0` (#1332 S6).
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_render_diagnostic_bytes(handle: u32) -> u32 {
    render_locked(|| {
        with_render_mut(handle, 0, |target| match target {
            RenderTarget::Half(render) => u32::try_from(render.diagnostic().len()).unwrap_or(0),
            RenderTarget::Host(_) => 0,
        })
    })
}

/// The address of the meter header of the newest replacement plan the host's control half
/// committed, or `0` before the first or for a wrong handle (#1332 S6, the meter-view hook S7's
/// Worker uses). The Worker reads it after each control call that may publish a plan and names it,
/// with the memory's byte length, to the worklet, whose message handler extends its views over
/// the shared memory so that they cover the header and its frame before render adopts the plan:
/// the worklet's `process()` then follows a retired header's `successor_pointer` without cutting
/// a view. Only an address: after a supersession it may name freed tables, so the worklet never
/// reads through it before render has adopted that plan. Control side; not render-locked.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_successor_header_ptr(handle: u32) -> u32 {
    with_host(handle, 0, |host| {
        u32::try_from(host.successor_header_address()).unwrap_or(0)
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
    use super::{AudioWorkletEngineHost, WebBootOptions};

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

    /// Reserve a second instance's stack and TLS block, as
    /// `miso_engine_web_v1_instance_reserve` does, with the full-width addresses (#1332 S6).
    pub fn instance_reserve(tls_size: u32, tls_align: u32) -> Option<crate::Reservation> {
        crate::handoff::reserve(tls_size, tls_align)
    }

    /// Overwrite one byte of `reservation`'s canary, as a stack overflow would (#1332 S6).
    #[cfg(feature = "test-support")]
    pub fn overwrite_canary(reservation: &crate::Reservation) {
        crate::handoff::overwrite_canary_for_test(reservation);
    }

    /// Bind this thread's instance to the reservation at `record`, as
    /// `miso_engine_web_v1_instance_bind` does with a `u32` address (#1332 S6).
    pub fn instance_bind(record: usize) -> u32 {
        super::render_locked(|| {
            if crate::handoff::bind(record) {
                super::RESULT_OK
            } else {
                super::RESULT_INVALID_ARGUMENT
            }
        })
    }

    /// The status `miso_engine_web_v1_status_ptr(handle)` names on this thread's instance: the
    /// host's, its shell's, or a render-only session's (#1332 S6).
    pub fn render_status(handle: u32) -> Option<crate::WebStatus> {
        super::with_render(handle, None, |status, _, _, _| Some(*status))
    }

    /// The output planes and the meter header and frame of the render half `handle` names on this
    /// thread's instance, wherever it is (#1332 S6).
    pub fn render_output(handle: u32) -> Option<(Vec<f32>, crate::WebMeterHeader, Vec<f32>)> {
        super::with_render(handle, None, |_, header, frame, output| {
            Some((output?.to_vec(), *header, frame.to_vec()))
        })
    }

    /// The render-only session's diagnostic, as `miso_engine_web_v1_render_diagnostic_ptr` and
    /// `_bytes` name it (#1332 S6).
    pub fn render_diagnostic(handle: u32) -> Vec<u8> {
        super::with_render_mut(handle, Vec::new(), |target| match target {
            super::RenderTarget::Half(render) => render.diagnostic().to_vec(),
            super::RenderTarget::Host(_) => Vec::new(),
        })
    }

    /// The session diagnostic of the host `handle` names, as the worklet reads it through
    /// `BUFFER_DIAGNOSTIC` (#1332 S6).
    pub fn diagnostic(handle: u32) -> Vec<u8> {
        super::with_host(handle, Vec::new(), |host| host.diagnostic().to_vec())
    }

    /// The full-width address [`super::miso_engine_web_v1_successor_header_ptr`] names (#1332 S6).
    pub fn successor_header(handle: u32) -> usize {
        super::with_host(handle, 0, AudioWorkletEngineHost::successor_header_address)
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

/// Stage `bytes` as the boot command, as the worklet does through
/// [`miso_engine_web_v1_boot_command_ptr`] (#2166); `false` when the staging refused them.
#[cfg(test)]
pub(crate) fn test_stage_boot_command(bytes: &[u8]) -> bool {
    let Ok(length) = u32::try_from(bytes.len()) else {
        return false;
    };
    if stage_boot_command(length).is_null() {
        return false;
    }
    BOOT_STAGING.with(|staging| staging.borrow_mut().command.copy_from_slice(bytes));
    true
}

/// The staged boot command's length (#2166).
#[cfg(test)]
pub(crate) fn test_staged_boot_command_bytes() -> usize {
    BOOT_STAGING.with(|staging| staging.borrow().command.len())
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

/// The lib tests' counting global allocator: [`measured`](allocation_counter::measured) counts
/// the current thread's allocations and frees inside one closure.
#[cfg(test)]
pub(crate) mod allocation_counter {
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
}

#[cfg(test)]
mod alias_tests {
    //! The ordinary aliases' live-host guard.
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

    #[test]
    fn alias_host_borrow_refusal_is_terminal() {
        let handle = boot_ordinary();
        LIVE_HOST.with(|slot| {
            let _borrow = slot.borrow();
            assert_eq!(miso_engine_web_v1_meter_lease(handle, 2), RESULT_INTERNAL);
        });
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
    fn invalid_alias_handle_is_terminal() {
        let handle = boot_ordinary();
        assert_eq!(
            miso_engine_web_v1_meter_lease(0, 1),
            RESULT_INVALID_ARGUMENT
        );
        dispose(handle);
    }
}
