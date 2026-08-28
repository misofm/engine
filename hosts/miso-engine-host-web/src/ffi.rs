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
    BUFFER_OUTPUT_PCM, BUFFER_SOURCE_ID, BUFFER_SOURCE_PCM, BootFailure, MAXIMUM_DOCUMENT_BYTES,
    RESULT_INTERNAL, RESULT_INVALID_ARGUMENT, RESULT_OK, RESULT_REFUSED_BUDGET,
    RESULT_REFUSED_DOCUMENT, RESULT_REFUSED_LIFECYCLE, STATE_READY, WebBootOptions,
};
use core::{
    cell::{Cell, RefCell},
    mem::MaybeUninit,
    ptr, slice,
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
        BUFFER_SOURCE_ID => resources.source_id_bytes,
        BUFFER_SOURCE_PCM => resources.source_pcm_staging_bytes,
        BUFFER_DIAGNOSTIC => resources.diagnostic_bytes,
        BUFFER_OUTPUT_PCM => resources.output_pcm_bytes,
        BUFFER_COMMAND => host.command_staging_bytes(),
        BUFFER_METER_FRAME => (host.meter_frame().len() * 4) as u64,
        _ => return 0,
    };
    u32::try_from(bytes).unwrap_or(0)
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
        match AudioWorkletEngineHost::boot(&staging.document, *staging.options) {
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

/// Copy one canonical track ID into the source-ID staging buffer; returns its byte length.
///
/// Zero means "no such track" or "the ID does not fit the staged buffer": the caller reads the
/// bytes out of [`BUFFER_SOURCE_ID`], which preparation already sized for the longest ID in the
/// session, and a session whose IDs did not fit was refused at compilation.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_console_track_id(handle: u32, index: u32) -> u32 {
    with_host_mut(handle, 0, |host| host.copy_console_track_id(index))
}

/// Return the number of sources the compiled session declares, or zero before compilation.
///
/// # Issue #207: source introspection
///
/// The browser ABI has exposed track discovery since #137 and nothing at all about sources, so a
/// headless driver compiling raw session TOML could not learn which sources exist, how many
/// channels they carry, or how many frames to feed them -- it could not drive the render loop it
/// had just compiled. These six queries close that, additively, in the shape the track queries
/// already established: a count, an ID copied through the staging buffer, and scalar shape reads.
///
/// **Canonical source order** is the normalized model's `sources` order -- `compile_session` sorts
/// by stable ID -- and the queries read that list itself, so no second table exists to drift from
/// it. **State gating** is the track queries' gating exactly: the answers come from the compiled
/// session, so every query reports zero/absent until `boot` succeeds, and keeps answering
/// afterwards for as long as the handle holds a compiled session, sticky failure included.
///
/// **This export is the bounds authority.** `source_channels`, `source_frames` and
/// `source_sample_rate` return zero for an out-of-range index because zero is impossible for a
/// compiled source, but `source_start_frame` has no spare value -- zero is an ordinary region
/// start -- so a caller establishes the range here and then indexes inside it.
///
/// These queries survived issue #240's ABI-v2 boot recut unchanged. What pins the complete surface
/// is the frozen export set in `scripts/check-web-audioworklet.sh`, which is exact rather than a
/// lower bound: an export that appears or disappears fails that gate.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_source_count(handle: u32) -> u32 {
    with_host(handle, 0, |host| {
        u32::try_from(host.session_source_count()).unwrap_or(0)
    })
}

/// Copy one canonical source ID into the source-ID staging buffer; returns its byte length.
///
/// Zero means "no such source". Unlike [`miso_engine_web_v1_console_track_id`] it cannot also mean
/// "the ID does not fit": compilation refuses a session whose source IDs exceed
/// [`BUFFER_SOURCE_ID`], which is what that buffer is sized for.
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

/// Return one source's declared region length in source sample frames, or zero out of range.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_source_frames(handle: u32, index: u32) -> u64 {
    with_host(handle, 0, |host| {
        host.session_source_shape(index)
            .map_or(0, |shape| shape.region_frames)
    })
}

/// Return one source's declared region start in source sample frames.
///
/// Zero is an ordinary answer -- most sessions start their regions there -- so this export carries
/// no out-of-range sentinel; [`miso_engine_web_v1_source_count`] is the bounds authority. It is
/// load-bearing rather than decorative: preparation builds the source ring *at* this frame, so a
/// driver that submitted from zero into a session with a nonzero region start would be feeding the
/// ring frames it is not waiting for.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_source_start_frame(handle: u32, index: u32) -> u64 {
    with_host(handle, 0, |host| {
        host.session_source_shape(index)
            .map_or(0, |shape| shape.region_start_frame)
    })
}

/// Return one source's declared native sample rate in hertz, or zero for an out-of-range index.
///
/// A compiled session's per-source rate necessarily equals the session rate -- preparation refuses
/// `host.source.rate.mismatch` because V1 has no sample-rate conversion -- so this reports the
/// declaration rather than new information. It is exposed because the session model carries the
/// field per source, and a consumer should read what the session says instead of re-deriving it
/// from an invariant it cannot see.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_source_sample_rate(handle: u32, index: u32) -> u32 {
    with_host(handle, 0, |host| {
        host.session_source_shape(index)
            .map_or(0, |shape| shape.sample_rate_hz)
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
