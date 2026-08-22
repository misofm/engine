//! Raw Wasm32 handle, pointer, and slice boundary.

#![allow(unsafe_code)]

use core::{
    cell::{Cell, RefCell},
    mem::MaybeUninit,
    ptr, slice,
};
use std::panic::{AssertUnwindSafe, catch_unwind};

use crate::{
    ABI_VERSION, AudioWorkletEngineHost, BUFFER_DIAGNOSTIC, BUFFER_OUTPUT_PCM, BUFFER_SESSION_TOML,
    BUFFER_SOURCE_ID, BUFFER_SOURCE_PCM, PREPARE_CONFIG_BYTES, RESULT_INTERNAL,
    RESULT_INVALID_ARGUMENT, RESULT_OK, STATE_READY, WebPrepareConfigV1,
};

struct LiveHost {
    handle: u32,
    host: Box<AudioWorkletEngineHost>,
}

thread_local! {
    static LIVE_HOST: RefCell<Option<LiveHost>> = const { RefCell::new(None) };
    static NEXT_HANDLE: Cell<u32> = const { Cell::new(1) };
}

fn catch_result(operation: impl FnOnce() -> u32) -> u32 {
    catch_unwind(AssertUnwindSafe(operation)).unwrap_or(RESULT_INTERNAL)
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
        BUFFER_SESSION_TOML => host
            .session_toml_mut()
            .map_or(ptr::null_mut(), <[u8]>::as_mut_ptr),
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
        _ => ptr::null_mut(),
    }
}

fn buffer_capacity(host: &AudioWorkletEngineHost, kind: u32) -> u32 {
    let resources = host.resources();
    let bytes = match kind {
        BUFFER_SESSION_TOML => resources.session_toml_bytes,
        BUFFER_SOURCE_ID => resources.source_id_bytes,
        BUFFER_SOURCE_PCM => resources.source_pcm_staging_bytes,
        BUFFER_DIAGNOSTIC => resources.diagnostic_bytes,
        BUFFER_OUTPUT_PCM => resources.output_pcm_bytes,
        _ => return 0,
    };
    u32::try_from(bytes).unwrap_or(0)
}

/// Return the frozen browser-Wasm ABI version.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_abi_version() -> u32 {
    ABI_VERSION
}

/// Allocate the sole configuration handle, or return zero while another handle is live.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_config_new() -> u32 {
    catch_unwind(AssertUnwindSafe(|| {
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
                host: Box::new(AudioWorkletEngineHost::new(
                    WebPrepareConfigV1::launch_defaults(48_000, 128),
                )),
            });
            handle
        })
    }))
    .unwrap_or(0)
}

/// Return the mutable configuration address while the handle is in config state.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_config_ptr(handle: u32) -> u32 {
    catch_result(|| {
        with_host_mut(handle, 0, |host| {
            host.config_mut()
                .map_or(0, |config| pointer_u32(ptr::from_mut(config)))
        })
    })
}

/// Return the exact frozen configuration byte size.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_config_bytes() -> u32 {
    PREPARE_CONFIG_BYTES
}

/// Validate configuration and allocate all fixed staging storage.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_prepare(handle: u32) -> u32 {
    catch_result(|| {
        with_host_mut(
            handle,
            RESULT_INVALID_ARGUMENT,
            AudioWorkletEngineHost::prepare,
        )
    })
}

/// Return one prepared stable staging-buffer address or zero.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_buffer_ptr(handle: u32, kind: u32) -> u32 {
    catch_result(|| with_host_mut(handle, 0, |host| pointer_u32(buffer_pointer(host, kind))))
}

/// Return one prepared staging-buffer capacity in bytes or zero.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_buffer_capacity(handle: u32, kind: u32) -> u32 {
    catch_result(|| with_host(handle, 0, |host| buffer_capacity(host, kind)))
}

/// Compile the staged strict TOML prefix and atomically publish session plus plan ownership.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_compile(handle: u32, toml_bytes: u32) -> u32 {
    catch_result(|| {
        with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
            host.compile(toml_bytes as usize)
        })
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
    catch_result(|| {
        with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
            if end_of_region > 1 || host.status().state != STATE_READY {
                return if end_of_region > 1 {
                    host.record_boundary_result(RESULT_INVALID_ARGUMENT)
                } else {
                    host.submit_source(&[], 0, 0, 0, &[], 0, false)
                };
            }
            let quantum = host.config().quantum_frames as usize;
            let channel_count = channels as usize;
            let frame_count = frames as usize;
            let id_count = source_id_bytes as usize;
            let sample_rate = host.config().sample_rate_hz;
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
    catch_result(|| {
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
    catch_result(|| {
        with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
            if host.status().state == STATE_READY && host.config().quantum_frames != actual_frames {
                return host.reject_output_quantum(actual_frames);
            }
            host.render_next()
        })
    })
}

/// Return the stable prepared resource-report address or zero for an invalid handle.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_resource_ptr(handle: u32) -> u32 {
    catch_result(|| {
        with_host(handle, 0, |host| {
            pointer_u32(ptr::from_ref(host.resources()))
        })
    })
}

/// Return the stable status address or zero for an invalid handle.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_status_ptr(handle: u32) -> u32 {
    catch_result(|| with_host(handle, 0, |host| pointer_u32(ptr::from_ref(host.status()))))
}

/// Quiescently dispose the live handle; zero is an explicit no-op.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_dispose(handle: u32) -> u32 {
    catch_result(|| {
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
            result
        })
    })
}

#[cfg(test)]
pub(crate) fn test_configure(handle: u32, config: WebPrepareConfigV1) -> u32 {
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        let Some(target) = host.config_mut() else {
            return RESULT_INVALID_ARGUMENT;
        };
        *target = config;
        RESULT_OK
    })
}

#[cfg(test)]
pub(crate) fn test_copy_staging(handle: u32, kind: u32, bytes: &[u8]) -> u32 {
    with_host_mut(handle, RESULT_INVALID_ARGUMENT, |host| {
        let target = match kind {
            BUFFER_SESSION_TOML => host.session_toml_mut(),
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
pub(crate) fn test_status(handle: u32) -> Option<crate::WebStatusV1> {
    with_host(handle, None, |host| Some(*host.status()))
}

#[cfg(test)]
pub(crate) fn test_resources(handle: u32) -> Option<crate::WebResourceReportV1> {
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
