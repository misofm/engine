//! Raw C-pointer ownership boundary.

#![allow(unsafe_code)]

use crate::{
    ABI_VERSION, BYTES_OUT_SIZE, BytesOut, CAPABILITIES_SIZE, Capabilities, CompileLimits,
    EXACT_LAUNCH_RATE_MASK, Engine, EngineConfig, FEATURE_MASK, HandleHeader, Plan,
    PlanResourceReport, PlanarOutput, RESULT_ABI_MISMATCH, RESULT_BACKPRESSURE,
    RESULT_BUFFER_TOO_SMALL, RESULT_COMPILE_REJECTED, RESULT_INTERNAL, RESULT_INVALID_ARGUMENT,
    RESULT_OK, RESULT_RENDER_REJECTED, RESULT_UNSUPPORTED, RESULT_WRONG_HANDLE, Session,
    SourceChunk, SubmitReport,
};
use core::ffi::c_void;
use core::ptr;
use core::sync::atomic::{AtomicU32, Ordering};
use miso_engine_lane::fpenv::CanonicalFpEnv;
use std::panic::{AssertUnwindSafe, catch_unwind};

use crate::runtime::{
    CommandError, EventError, EventLane, PlanQueries, PlanState, compile_children,
    limits_are_valid, plan_error,
};

fn catch_result(operation: impl FnOnce() -> u32) -> u32 {
    catch_unwind(AssertUnwindSafe(operation)).unwrap_or(RESULT_INTERNAL)
}

fn catch_destroy(operation: impl FnOnce()) {
    let _contained = catch_unwind(AssertUnwindSafe(operation));
}

fn set_engine_error(engine: &Engine, value: &[u8]) {
    let value = core::str::from_utf8(value).unwrap_or("capi.internal.utf8");
    let mut bytes = engine.last_error.borrow_mut();
    let mut len = value.len().min(bytes.len());
    while !value.is_char_boundary(len) {
        len -= 1;
    }
    bytes[..len].copy_from_slice(&value.as_bytes()[..len]);
    engine.last_error_len.set(len);
}

fn clear_engine_error(engine: &Engine) {
    engine.last_error_len.set(0);
}

unsafe fn validate_bytes_out<'a>(out: *mut BytesOut) -> Result<&'a mut BytesOut, u32> {
    if out.is_null() {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    // SAFETY: The caller promises readable and writable storage for one ABI bytes-out value.
    let out = unsafe { &mut *out };
    if out.struct_size != BYTES_OUT_SIZE
        || out.reserved0 != 0
        || (out.data.is_null() && out.capacity_bytes != 0)
    {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    Ok(out)
}

unsafe fn write_bytes(out: *mut BytesOut, value: &[u8]) -> u32 {
    // SAFETY: This helper inherits the entrypoint's bytes-output pointer contract.
    let out = match unsafe { validate_bytes_out(out) } {
        Ok(value) => value,
        Err(code) => return code,
    };
    let required = match u64::try_from(value.len()) {
        Ok(value) => value,
        Err(_) => return RESULT_INTERNAL,
    };
    out.required_bytes = required;
    if out.capacity_bytes < required {
        return RESULT_BUFFER_TOO_SMALL;
    }
    if required == 0 {
        return RESULT_OK;
    }
    if out.data.is_null() {
        return RESULT_BUFFER_TOO_SMALL;
    }
    // SAFETY: Capacity was checked against the complete byte count; the caller promises writable
    // nonoverlapping output storage and no pointer is retained.
    unsafe { ptr::copy_nonoverlapping(value.as_ptr(), out.data, value.len()) };
    RESULT_OK
}

/// Frozen upper bound on borrowed source-ID bytes: `StableId` is `[a-z][a-z0-9._-]{0,126}`
/// (`miso-engine-session/src/id.rs`), so no valid ID exceeds 127 bytes.
const MAX_SOURCE_ID_BYTES: u64 = 127;

/// Frozen maximum extent of any single borrowed region, in bytes.
///
/// `core::slice::from_raw_parts` requires the total size to fit in `isize`; a larger caller-declared
/// length is undefined behaviour before a single byte is read, so it is rejected here.
const MAX_BORROWED_BYTES: u64 = isize::MAX as u64;

/// Borrows caller bytes for the duration of one call.
///
/// Rejects a null pointer, a length above the call's own `limit`, and any length the platform
/// cannot address as one slice, all before the region is turned into a slice.
///
/// # Safety
///
/// A nonnull `data` must be readable for `bytes` bytes for the duration of the call.
unsafe fn borrowed_bytes<'a>(data: *const u8, bytes: u64, limit: u64) -> Result<&'a [u8], u32> {
    if data.is_null() || bytes > limit || bytes > MAX_BORROWED_BYTES {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    let bytes = usize::try_from(bytes).map_err(|_| RESULT_INVALID_ARGUMENT)?;
    // SAFETY: `data` is nonnull, the caller promises it is readable for the declared byte count for
    // this call, and the length was proved to be at most `isize::MAX` above.
    Ok(unsafe { core::slice::from_raw_parts(data, bytes) })
}

unsafe fn engine_kind(engine: *const Engine) -> u32 {
    if engine.is_null() {
        return RESULT_INVALID_ARGUMENT;
    }
    // SAFETY: The ABI contract requires a live library handle. Every opaque representation is
    // `repr(C)` with `HandleHeader` as its first field, so this borrows only the shared prefix and
    // never creates a reference to the wrong concrete handle representation.
    if unsafe { &*engine.cast::<HandleHeader>() }.is_engine() {
        RESULT_OK
    } else {
        RESULT_WRONG_HANDLE
    }
}

unsafe fn session_kind(session: *const Session) -> u32 {
    if session.is_null() {
        return RESULT_INVALID_ARGUMENT;
    }
    // SAFETY: See `engine_kind`; only the common `HandleHeader` prefix is borrowed.
    if unsafe { &*session.cast::<HandleHeader>() }.is_session() {
        RESULT_OK
    } else {
        RESULT_WRONG_HANDLE
    }
}

unsafe fn plan_kind(plan: *const Plan) -> u32 {
    if plan.is_null() {
        return RESULT_INVALID_ARGUMENT;
    }
    // SAFETY: See `engine_kind`; only the common `HandleHeader` prefix is borrowed.
    if unsafe { &*plan.cast::<HandleHeader>() }.is_plan() {
        RESULT_OK
    } else {
        RESULT_WRONG_HANDLE
    }
}

/// Projects the render-thread-exclusive plan state without borrowing the whole handle.
///
/// # Safety
///
/// `plan` must have passed [`plan_kind`]. The returned pointer aliases only the `state` field, so
/// the exclusive render borrow it produces stays disjoint from `queries` and `last_error`.
unsafe fn plan_state(plan: *mut Plan) -> *mut PlanState {
    // SAFETY: The live-kind check established the concrete representation; a raw field projection
    // creates no reference to `Plan` itself.
    unsafe { &raw mut (*plan).state }
}

/// Projects the any-thread render diagnostic slot without borrowing the whole handle.
///
/// # Safety
///
/// `plan` must have passed [`plan_kind`].
unsafe fn plan_error_slot(plan: *const Plan) -> *const AtomicU32 {
    // SAFETY: See `plan_state`; the projection is disjoint from `state`.
    unsafe { &raw const (*plan).last_error }
}

/// Projects the any-thread resource query view without borrowing the whole handle.
///
/// # Safety
///
/// `plan` must have passed [`plan_kind`].
unsafe fn plan_queries(plan: *const Plan) -> *const PlanQueries {
    // SAFETY: See `plan_state`; the projection is disjoint from `state`.
    unsafe { &raw const (*plan).queries }
}

/// Returns the frozen Engine V2 C ABI version.
///
/// Thread: any.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_v2_abi_version() -> u32 {
    catch_result(|| ABI_VERSION)
}

/// Writes the frozen ABI V1 launch-rate and feature capability masks.
///
/// # Safety
///
/// `out` must satisfy the writable ABI V1 capability-struct contract for this call.
///
/// Thread: any.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn miso_engine_v2_query_capabilities(out: *mut Capabilities) -> u32 {
    catch_result(|| {
        if out.is_null() {
            return RESULT_INVALID_ARGUMENT;
        }
        // SAFETY: `out` is nonnull and the caller promises writable storage whose first field is
        // readable. The exact size check precedes the complete fixed-size write.
        // SAFETY: `out` is nonnull and the caller promises readable capability storage for these
        // two input-validation fields before the complete fixed-size write.
        let (struct_size, reserved) = unsafe { ((*out).struct_size, (*out).reserved) };
        if struct_size != CAPABILITIES_SIZE || reserved != [0; 4] {
            return RESULT_INVALID_ARGUMENT;
        }
        // SAFETY: The exact struct-size check above establishes writable ABI V1 storage according
        // to the C contract; one complete value is written and no caller pointer is retained.
        unsafe {
            out.write(Capabilities {
                struct_size: CAPABILITIES_SIZE,
                abi_version: ABI_VERSION,
                exact_launch_rate_mask: EXACT_LAUNCH_RATE_MASK,
                feature_mask: FEATURE_MASK,
                reserved: [0; 4],
            });
        }
        RESULT_OK
    })
}

/// Creates an engine factory handle without compiling runtime state.
///
/// This is the C ABI's boot entry, so it is where the CPU is attested (master plan #83 D4). The
/// engine is compiled for a pinned instruction set and dispatches nothing at runtime; a host whose
/// CPU cannot execute it is told so, once, with [`RESULT_UNSUPPORTED`], rather than being allowed
/// to reach an illegal instruction inside a render callback. On a CPU that satisfies the pin —
/// every supported host — nothing about this call changes.
///
/// # Safety
///
/// `config` and `out_engine` must satisfy their readable/writable ABI V1 pointer contracts.
///
/// Thread: control.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn miso_engine_v2_engine_create(
    config: *const EngineConfig,
    out_engine: *mut *mut Engine,
) -> u32 {
    catch_result(|| {
        if out_engine.is_null() {
            return RESULT_INVALID_ARGUMENT;
        }
        // SAFETY: The caller promises writable storage for one output pointer. Clearing it before
        // all other validation preserves transactional publication on every rejection.
        unsafe { out_engine.write(ptr::null_mut()) };
        if config.is_null() {
            return RESULT_INVALID_ARGUMENT;
        }
        // SAFETY: The caller promises `config` identifies a readable EngineConfig for this call.
        let config = unsafe { &*config };
        if config.struct_size != crate::ENGINE_CONFIG_SIZE || config.reserved != [0; 4] {
            return RESULT_INVALID_ARGUMENT;
        }
        if config.abi_version != ABI_VERSION {
            return RESULT_ABI_MISMATCH;
        }
        // D4 boot attestation: control-plane work, once per engine, never from render.
        if miso_engine_lane::attest_host().is_err() {
            return RESULT_UNSUPPORTED;
        }

        let engine = Box::new(Engine::new());
        // SAFETY: `out_engine` was validated as writable above. Box::into_raw transfers the unique
        // allocation to the matching destroy entrypoint and no Rust owner remains.
        unsafe { out_engine.write(Box::into_raw(engine)) };
        RESULT_OK
    })
}

/// Destroys an engine handle off render; null is a no-op.
///
/// # Safety
///
/// A nonnull `engine` must be the unique live engine handle returned by this library.
///
/// Thread: control, quiescent.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn miso_engine_v2_engine_destroy(engine: *mut Engine) {
    catch_destroy(|| {
        if engine.is_null() {
            return;
        }
        // SAFETY: The ABI contract requires `engine` to be a live unique engine handle returned by
        // this library and destroy to be quiescent. A wrong live kind is observed but not freed.
        if unsafe { engine_kind(engine) } != RESULT_OK {
            return;
        }
        // SAFETY: The successful kind check and ABI ownership contract establish that this is the
        // unique pointer produced by Box::into_raw and has not previously been destroyed.
        drop(unsafe { Box::from_raw(engine) });
    });
}

/// Transactionally compile one strict session and publish independent child handles together.
///
/// # Safety
///
/// Every nonnull pointer must satisfy its ABI V1 readable, writable, or live-handle contract.
///
/// Thread: control.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn miso_engine_v2_compile_session(
    engine: *mut Engine,
    toml: *const u8,
    toml_bytes: u64,
    limits: *const CompileLimits,
    diagnostics: *mut BytesOut,
    out_session: *mut *mut Session,
    out_plan: *mut *mut Plan,
) -> u32 {
    catch_result(|| {
        if out_session.is_null() || out_plan.is_null() {
            return RESULT_INVALID_ARGUMENT;
        }
        // SAFETY: Both output locations are nonnull and promised writable by the caller. Clearing
        // both before validation preserves the frozen atomic-publication rule.
        unsafe {
            out_session.write(ptr::null_mut());
            out_plan.write(ptr::null_mut());
        }
        // SAFETY: Nonnull live handle pointers are caller-provided under the handle contract.
        let kind = unsafe { engine_kind(engine) };
        if kind != RESULT_OK {
            return kind;
        }
        if toml.is_null() || limits.is_null() || diagnostics.is_null() {
            return RESULT_INVALID_ARGUMENT;
        }
        // SAFETY: The caller promises `limits` identifies a readable fixed ABI value.
        let limits = unsafe { *limits };
        if !limits_are_valid(limits) {
            return RESULT_INVALID_ARGUMENT;
        }
        // SAFETY: `diagnostics` is nonnull and promised readable/writable for this call.
        if let Err(code) = unsafe { validate_bytes_out(diagnostics) } {
            return code;
        }
        if toml_bytes > limits.maximum_toml_bytes {
            let value = b"capi.toml.limit\t$\n";
            // SAFETY: `engine` passed the live-kind check and can be borrowed for this call.
            set_engine_error(unsafe { &*engine }, value);
            // SAFETY: The bytes-output contract was validated above.
            let written = unsafe { write_bytes(diagnostics, value) };
            return if written == RESULT_OK {
                RESULT_COMPILE_REJECTED
            } else {
                written
            };
        }
        // SAFETY: The caller promises the TOML region is readable for `toml_bytes`.
        let toml = match unsafe { borrowed_bytes(toml, toml_bytes, limits.maximum_toml_bytes) } {
            Ok(value) => value,
            Err(code) => return code,
        };
        let toml = match core::str::from_utf8(toml) {
            Ok(value) => value,
            Err(_) => {
                let value = b"capi.toml.utf8\t$\n";
                // SAFETY: `engine` passed the live-kind check and can be borrowed for this call.
                set_engine_error(unsafe { &*engine }, value);
                // SAFETY: The bytes-output contract was validated above.
                let written = unsafe { write_bytes(diagnostics, value) };
                return if written == RESULT_OK {
                    RESULT_COMPILE_REJECTED
                } else {
                    written
                };
            }
        };
        let children = match compile_children(toml, limits) {
            Ok(value) => value,
            Err(mut failure) => {
                let maximum =
                    usize::try_from(limits.maximum_diagnostic_bytes).unwrap_or(usize::MAX);
                if failure.diagnostics.len() > maximum {
                    failure.diagnostics.clear();
                }
                // SAFETY: `engine` passed the live-kind check and can be borrowed for this call.
                set_engine_error(unsafe { &*engine }, &failure.diagnostics);
                // SAFETY: The bytes-output contract was validated above.
                let written = unsafe { write_bytes(diagnostics, &failure.diagnostics) };
                return if written == RESULT_OK {
                    RESULT_COMPILE_REJECTED
                } else {
                    written
                };
            }
        };
        // SAFETY: The bytes-output contract was validated above; success has no diagnostics.
        let written = unsafe { write_bytes(diagnostics, &[]) };
        if written != RESULT_OK {
            return written;
        }
        let session = Box::new(Session::new(children.session, children.session_error));
        let plan = Box::new(Plan::new(children.plan));
        // SAFETY: Both output locations were validated and cleared before compilation. Ownership
        // transfers only after both independent boxes exist, so publication is atomic.
        unsafe {
            out_session.write(Box::into_raw(session));
            out_plan.write(Box::into_raw(plan));
            clear_engine_error(&*engine);
        }
        RESULT_OK
    })
}

/// Submit one borrowed planar source chunk atomically into its prepared host ring.
///
/// # Safety
///
/// Every pointer must satisfy its ABI V1 borrowed-data, output, or live-handle contract.
///
/// Thread: control.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn miso_engine_v2_source_submit_planar_f32(
    session: *mut Session,
    source_id: *const u8,
    source_id_bytes: u64,
    chunk: *const SourceChunk,
    out_report: *mut SubmitReport,
) -> u32 {
    catch_result(|| {
        // SAFETY: Nonnull live handle pointers are caller-provided under the handle contract.
        let kind = unsafe { session_kind(session) };
        if kind != RESULT_OK {
            return kind;
        }
        if source_id.is_null() || chunk.is_null() || out_report.is_null() {
            return RESULT_INVALID_ARGUMENT;
        }
        // SAFETY: The caller promises readable input/output ABI structs for this call.
        let chunk = unsafe { &*chunk };
        // SAFETY: The caller promises readable input/output ABI structs for this call.
        let report = unsafe { &mut *out_report };
        if chunk.struct_size != crate::SOURCE_CHUNK_SIZE
            || chunk.reserved0 != 0
            || chunk.end_of_region > 1
            || report.struct_size != crate::SUBMIT_REPORT_SIZE
            || report.reserved0 != 0
            || chunk.plane_count == 0
            || chunk.plane_count > 255
            || chunk.planes.is_null()
            || !chunk.planes.is_aligned()
            || u64::from(chunk.frames).saturating_mul(core::mem::size_of::<f32>() as u64)
                > MAX_BORROWED_BYTES
        {
            return RESULT_INVALID_ARGUMENT;
        }
        // SAFETY: The caller promises the source-ID storage is readable for this call.
        let source_id =
            match unsafe { borrowed_bytes(source_id, source_id_bytes, MAX_SOURCE_ID_BYTES) } {
                Ok(value) if !value.is_empty() && core::str::from_utf8(value).is_ok() => value,
                _ => return RESULT_INVALID_ARGUMENT,
            };
        let plane_count = chunk.plane_count as usize;
        // SAFETY: `chunk.planes` was proved nonnull and pointer-aligned above, `plane_count` is at
        // most 255 so the array is far inside `isize::MAX` bytes, and the caller promises that many
        // readable plane pointers for this call.
        let plane_pointers = unsafe { core::slice::from_raw_parts(chunk.planes, plane_count) };
        let frames = chunk.frames as usize;
        let mut planes: [&[f32]; 255] = [&[]; 255];
        for (index, plane) in plane_pointers.iter().enumerate() {
            if plane.is_null() || !plane.is_aligned() {
                return RESULT_INVALID_ARGUMENT;
            }
            // SAFETY: Each caller plane is nonnull, `f32`-aligned, and at most `isize::MAX` bytes
            // long — all checked above — is readable for exactly `frames` samples, and is borrowed
            // only until the underlying source submission copies it.
            planes[index] = unsafe { core::slice::from_raw_parts(*plane, frames) };
        }
        // SAFETY: `session` passed the live-kind check and this operation requires its exclusive
        // serial producer owner under the ABI contract.
        let session = unsafe { &mut *session };
        match session.state.submit(
            source_id,
            crate::runtime::SourceSubmission {
                generation: chunk.generation,
                start_frame: chunk.start_frame,
                sample_rate_hz: chunk.sample_rate_hz,
                planes: &planes[..plane_count],
                frames: chunk.frames,
                end_of_region: chunk.end_of_region == 1,
            },
        ) {
            Ok(value) => {
                report.accepted_frames = u64::from(value.accepted_frames);
                report.cumulative_written_frames = value.cumulative_written_frames;
                report.active_generation = value.active_generation.0;
                session.last_error.borrow_mut().clear();
                RESULT_OK
            }
            Err(error) => {
                // F6: the facade's typed rejection reaches the caller as its own diagnostic
                // string, instead of one of two strings for seventeen distinct failures.
                let (code, diagnostic) = error.report();
                session.last_error.borrow_mut().set(diagnostic);
                code
            }
        }
    })
}

/// Queue one generation-tagged source seek for the next render block.
///
/// # Safety
///
/// `session` must be live and `source_id` must reference the declared borrowed byte count.
///
/// Thread: control.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn miso_engine_v2_source_seek(
    session: *mut Session,
    source_id: *const u8,
    source_id_bytes: u64,
    generation: u64,
    source_frame: u64,
) -> u32 {
    catch_result(|| {
        // SAFETY: Nonnull live handle pointers are caller-provided under the handle contract.
        let kind = unsafe { session_kind(session) };
        if kind != RESULT_OK {
            return kind;
        }
        if source_id.is_null() {
            return RESULT_INVALID_ARGUMENT;
        }
        // SAFETY: The caller promises the source-ID storage is readable for this call.
        let source_id =
            match unsafe { borrowed_bytes(source_id, source_id_bytes, MAX_SOURCE_ID_BYTES) } {
                Ok(value) if !value.is_empty() && core::str::from_utf8(value).is_ok() => value,
                _ => return RESULT_INVALID_ARGUMENT,
            };
        // SAFETY: `session` passed the live-kind check and the ABI requires exclusive serial
        // source-control ownership for seek.
        let session = unsafe { &mut *session };
        match session.state.seek(source_id, generation, source_frame) {
            Ok(()) => {
                session.last_error.borrow_mut().clear();
                RESULT_OK
            }
            Err(error) => {
                let (code, diagnostic) = error.report();
                session.last_error.borrow_mut().set(diagnostic);
                code
            }
        }
    })
}

/// Process one bounded Issue-005 capability command with exact-byte replay.
///
/// # Safety
///
/// Every pointer must satisfy its ABI V1 borrowed-frame, output, or live-handle contract.
///
/// Thread: control.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn miso_engine_v2_submit_command(
    session: *mut Session,
    request: *const u8,
    request_bytes: u64,
    response: *mut BytesOut,
) -> u32 {
    catch_result(|| {
        // SAFETY: Nonnull live handle pointers are caller-provided under the handle contract.
        let kind = unsafe { session_kind(session) };
        if kind != RESULT_OK {
            return kind;
        }
        if request.is_null() || response.is_null() {
            return RESULT_INVALID_ARGUMENT;
        }
        // SAFETY: The response descriptor is nonnull and promised readable/writable for this call.
        let output_capacity = match unsafe { validate_bytes_out(response) } {
            Ok(value) => value.capacity_bytes,
            Err(code) => return code,
        };
        // SAFETY: The caller promises the complete request frame is readable for this call.
        // The control codec bounds the frame itself; only the platform slice cap applies here.
        let request = match unsafe { borrowed_bytes(request, request_bytes, MAX_BORROWED_BYTES) } {
            Ok(value) if !value.is_empty() => value,
            _ => return RESULT_INVALID_ARGUMENT,
        };
        // SAFETY: The live-kind check establishes the concrete session representation; generic
        // command calls are serialized with source control by the ABI contract.
        let session = unsafe { &mut *session };
        match session.state.command(request, output_capacity) {
            Ok(bytes) => {
                let encoded = session.state.command_response(bytes);
                // SAFETY: The descriptor was validated and command preflight proved capacity.
                let code = unsafe { write_bytes(response, encoded) };
                if code == RESULT_OK {
                    session.last_error.borrow_mut().clear();
                }
                code
            }
            Err(CommandError::BufferTooSmall { required }) => {
                // SAFETY: The response descriptor was validated above; only required length is
                // updated and no caller payload byte is written.
                unsafe { (*response).required_bytes = required };
                session
                    .last_error
                    .borrow_mut()
                    .set(b"control.output.too_small");
                RESULT_BUFFER_TOO_SMALL
            }
            Err(CommandError::Invalid) => {
                session
                    .last_error
                    .borrow_mut()
                    .set(b"control.frame.invalid");
                RESULT_INVALID_ARGUMENT
            }
            Err(CommandError::Backpressure) => {
                session
                    .last_error
                    .borrow_mut()
                    .set(b"control.plan.backpressure");
                RESULT_BACKPRESSURE
            }
            Err(CommandError::CompileRejected(failure)) => {
                session.last_error.borrow_mut().set(&failure.diagnostics);
                RESULT_COMPILE_REJECTED
            }
            Err(CommandError::Internal) => {
                session.last_error.borrow_mut().set(b"control.internal");
                RESULT_INTERNAL
            }
        }
    })
}

/// Dequeue one complete reliable or lossy event frame into caller-owned bytes.
///
/// # Safety
///
/// `session` must be live and `event` must satisfy the ABI V1 bytes-output contract.
///
/// Thread: control.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn miso_engine_v2_dequeue_event(
    session: *mut Session,
    lane: u32,
    event: *mut BytesOut,
) -> u32 {
    catch_result(|| {
        // SAFETY: Nonnull live handle pointers are caller-provided under the handle contract.
        let kind = unsafe { session_kind(session) };
        if kind != RESULT_OK {
            return kind;
        }
        let lane = match lane {
            crate::EVENT_LANE_RELIABLE => EventLane::Reliable,
            crate::EVENT_LANE_LOSSY => EventLane::Lossy,
            _ => return RESULT_INVALID_ARGUMENT,
        };
        // SAFETY: The caller promises a readable/writable bytes-output descriptor for this call.
        let capacity = match unsafe { validate_bytes_out(event) } {
            Ok(value) => value.capacity_bytes,
            Err(code) => return code,
        };
        // SAFETY: The live-kind check establishes the concrete serialized session owner.
        let session = unsafe { &mut *session };
        match session.state.dequeue_event(lane, capacity) {
            Ok(Some(bytes)) => {
                let encoded = session.state.event_response(bytes);
                // SAFETY: Event preparation admitted the exact caller capacity.
                let code = unsafe { write_bytes(event, encoded) };
                if code == RESULT_OK {
                    session.last_error.borrow_mut().clear();
                }
                code
            }
            Ok(None) => {
                // SAFETY: Empty egress is successful and writes only the required length.
                let code = unsafe { write_bytes(event, &[]) };
                if code == RESULT_OK {
                    session.last_error.borrow_mut().clear();
                }
                code
            }
            Err(EventError::BufferTooSmall { required }) => {
                // SAFETY: Validation established writable descriptor metadata.
                unsafe { (*event).required_bytes = required };
                session
                    .last_error
                    .borrow_mut()
                    .set(b"control.event.output.too_small");
                RESULT_BUFFER_TOO_SMALL
            }
            Err(EventError::Backpressure) => {
                session
                    .last_error
                    .borrow_mut()
                    .set(b"control.event.backpressure");
                RESULT_BACKPRESSURE
            }
            Err(EventError::Internal) => {
                session
                    .last_error
                    .borrow_mut()
                    .set(b"control.event.internal");
                RESULT_INTERNAL
            }
        }
    })
}

/// Render one exact-time quantum directly into caller-owned contiguous planar storage.
///
/// The caller's floating-point environment is borrowed, never adopted (issue #146): every path out
/// of this function -- success, any rejection code, or an unwind -- restores the caller's exact
/// control word, because [`CanonicalFpEnv`] is an ordinary stack value whose `Drop` is the restore.
/// A DAW audio callback that arrives with FTZ and DAZ set therefore gets the same bits as one that
/// does not, and gets its own environment back untouched.
///
/// # Safety
///
/// `plan` must be live and exclusive; `output` must satisfy the caller-owned output contract.
///
/// Thread: render only, never concurrently with itself.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn miso_engine_v2_render_f32_planar(
    plan: *mut Plan,
    absolute_sample: u64,
    output: *const PlanarOutput,
) -> u32 {
    catch_result(|| {
        // Issue #146, first statement of the entry and last thing undone: the whole call runs in
        // the canonical floating-point environment, validation included, so a rejected call also
        // hands the caller's word back unchanged. One control-word read, two writes and two empty
        // assembly barriers per block; measured in `artifacts/issue146/`.
        let _fp_env = CanonicalFpEnv::enter();
        // SAFETY: Nonnull live handle pointers are caller-provided under the handle contract.
        let kind = unsafe { plan_kind(plan) };
        if kind != RESULT_OK {
            return kind;
        }
        if output.is_null() {
            return RESULT_INVALID_ARGUMENT;
        }
        // SAFETY: The caller promises one readable fixed ABI output descriptor for this call.
        let output = unsafe { &*output };
        // `PlanarOutput` is stereo by contract, so a channel count other than two is an ABI
        // mismatch, not a render rejection.
        if output.struct_size != crate::PLANAR_OUTPUT_SIZE
            || output.reserved != [0; 2]
            || output.samples.is_null()
            || output.channels != 2
        {
            return RESULT_INVALID_ARGUMENT;
        }
        // SAFETY: The live-kind check establishes the concrete render-plan representation and the
        // ABI requires exclusive ownership of the render-thread state for this call. Both borrows
        // are disjoint field projections, so a concurrent `plan_resources` or `last_error` query on
        // another thread never aliases the exclusive `PlanState` borrow.
        let state = unsafe { &mut *plan_state(plan) };
        // SAFETY: See above; the diagnostic slot is disjoint from `state`.
        let error = unsafe { &*plan_error_slot(plan) };
        // Issue #146 session-start re-attestation, on the thread that will render: the first block
        // this plan renders proves the canonical word actually took here, and refuses the render
        // rather than silently producing off-pin audio if it did not. Later blocks skip it.
        if !state.fp_env_attested.get() {
            if !miso_engine_lane::fpenv::in_canonical_fp_environment() {
                error.store(plan_error::FP_ENVIRONMENT, Ordering::Relaxed);
                return RESULT_RENDER_REJECTED;
            }
            state.fp_env_attested.set(true);
        }
        if !output.samples.is_aligned() {
            error.store(plan_error::OUTPUT_UNALIGNED, Ordering::Relaxed);
            return RESULT_INVALID_ARGUMENT;
        }
        // One validation pass, each rule checked by whoever owns it (audit #103 F4). The boundary
        // owns only what is ABI: the descriptor is well formed and the declared capacity is one
        // addressable slice. `PlanarBufferMut` owns the plane layout; core owns the quantum and
        // the clock. Nothing here recomputes a byte extent the layout check will compute again.
        let capacity = match usize::try_from(output.sample_capacity) {
            Ok(value) if value <= isize::MAX as usize / core::mem::size_of::<f32>() => value,
            _ => {
                error.store(plan_error::OUTPUT_PLATFORM, Ordering::Relaxed);
                return RESULT_RENDER_REJECTED;
            }
        };
        let samples_pointer = output.samples;
        let stride = output.plane_stride_samples as usize;
        let frames = output.frames as usize;
        // SAFETY: `output.samples` was proved nonnull and `f32`-aligned above, and
        // `capacity * size_of::<f32>() <= isize::MAX`. The ABI's output contract is that the
        // caller owns exactly `sample_capacity` writable `f32` elements there, exclusively for
        // the duration of this call.
        let samples = unsafe { core::slice::from_raw_parts_mut(samples_pointer, capacity) };
        let render_output = match miso_engine_core::realtime::PlanarBufferMut::try_new(
            samples, 2, frames, stride,
        ) {
            Ok(value) => value,
            Err(_) => {
                error.store(plan_error::OUTPUT_LAYOUT, Ordering::Relaxed);
                return RESULT_RENDER_REJECTED;
            }
        };
        match state.render(absolute_sample, render_output) {
            Ok(()) => {
                // Issue #163 phase 4 item 2: the scan runs only for an armed observer.
                //
                // This is `2 * frames` scalar loads plus an `abs` and a `max` on every successful
                // render -- 256 of them at the launch quantum -- and its sole consumer is
                // `collect_render_activity`, which discards it unless the endpoint has configured
                // meter handles and a nonzero meter period. `render_peak_observed` is that
                // condition, refreshed by the control thread. When it is false the block
                // publishes `NaN`, which the consumer reads as "not measured" and drops, so an
                // unarmed block never fabricates a peak of `0.0`.
                let peak = if state.render_peak_observed() {
                    let mut peak = 0.0_f32;
                    for channel in 0..2_usize {
                        let plane = channel * stride;
                        for frame in 0..frames {
                            // SAFETY: The validated two-plane layout proves this exact sample
                            // index is initialized and readable after the exclusive render borrow
                            // ended.
                            let sample = unsafe { *samples_pointer.add(plane + frame) };
                            peak = peak.max(sample.abs());
                        }
                    }
                    peak
                } else {
                    f32::NAN
                };
                state.publish_render_observation(peak);
                error.store(plan_error::NONE, Ordering::Relaxed);
                RESULT_OK
            }
            Err(code) => {
                error.store(code, Ordering::Relaxed);
                RESULT_RENDER_REJECTED
            }
        }
    })
}

/// Copy the frozen address-free resource projection for a prepared plan.
///
/// # Safety
///
/// `plan` must be live and `out` must satisfy the writable ABI V1 report contract.
///
/// Thread: any, concurrent with render.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn miso_engine_v2_plan_resources(
    plan: *const Plan,
    out: *mut PlanResourceReport,
) -> u32 {
    catch_result(|| {
        // SAFETY: Nonnull live handle pointers are caller-provided under the handle contract.
        let kind = unsafe { plan_kind(plan) };
        if kind != RESULT_OK {
            return kind;
        }
        if out.is_null() {
            return RESULT_INVALID_ARGUMENT;
        }
        // SAFETY: The caller promises readable/writable storage for one report. The size check
        // precedes the complete fixed-size write.
        // SAFETY: `out` is nonnull and the caller promises readable report storage for these two
        // input-validation fields before the complete fixed-size write.
        let (struct_size, reserved) = unsafe { ((*out).struct_size, (*out).reserved) };
        if struct_size != crate::PLAN_RESOURCE_REPORT_SIZE || reserved != [0; 4] {
            return RESULT_INVALID_ARGUMENT;
        }
        // SAFETY: `plan` passed the live-kind check. Only the any-thread `queries` field is
        // projected, so this query never aliases a concurrent render's exclusive `PlanState`
        // borrow. The call is pure: it writes nothing back through the plan handle.
        let queries = unsafe { &*plan_queries(plan) };
        let report = queries.resources();
        // SAFETY: Exact struct size establishes writable ABI V1 report storage.
        unsafe { out.write(report) };
        RESULT_OK
    })
}

/// Copies the handle-local diagnostic. Checkpoint 1 engine handles have an empty diagnostic.
///
/// # Safety
///
/// `live_handle` must identify a live ABI handle and `out` must satisfy the bytes-output contract.
///
/// Thread: any, concurrent with render, for a plan; control for a session or engine.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn miso_engine_v2_last_error(
    live_handle: *const c_void,
    out: *mut BytesOut,
) -> u32 {
    catch_result(|| {
        if live_handle.is_null() || out.is_null() {
            return RESULT_INVALID_ARGUMENT;
        }
        // SAFETY: Every live handle starts with the same header. Reading it through each opaque
        // representation is valid for the live-handle kinds defined by this library.
        let recognized = unsafe {
            engine_kind(live_handle.cast::<Engine>()) == RESULT_OK
                || session_kind(live_handle.cast::<Session>()) == RESULT_OK
                || plan_kind(live_handle.cast::<Plan>()) == RESULT_OK
        };
        if !recognized {
            return RESULT_WRONG_HANDLE;
        }
        // SAFETY: The pointer is a recognized live handle with the common header representation.
        let is_engine = unsafe { engine_kind(live_handle.cast::<Engine>()) } == RESULT_OK;
        if is_engine {
            // SAFETY: The recognized header identifies a live engine handle.
            let engine = unsafe { &*live_handle.cast::<Engine>() };
            let bytes = engine.last_error.borrow();
            let len = engine.last_error_len.get().min(bytes.len());
            // SAFETY: `write_bytes` completes before this bounded RefCell borrow is dropped.
            unsafe { write_bytes(out, &bytes[..len]) }
        } else {
            // SAFETY: The pointer is a recognized live handle with the common header representation.
            let is_session = unsafe { session_kind(live_handle.cast::<Session>()) } == RESULT_OK;
            if is_session {
                // SAFETY: The recognized header identifies a live session handle.
                let session = unsafe { &*live_handle.cast::<Session>() };
                let bytes = session.last_error.borrow();
                // SAFETY: `write_bytes` completes before this bounded RefCell borrow is dropped.
                unsafe { write_bytes(out, bytes.as_slice()) }
            } else {
                // SAFETY: The recognized header identifies the remaining live plan handle. Only
                // the atomic diagnostic slot is projected, so this query is safe concurrently with
                // a render call on another thread.
                let code = unsafe {
                    (*plan_error_slot(live_handle.cast::<Plan>())).load(Ordering::Relaxed)
                };
                // SAFETY: The bytes-output contract was validated by `write_bytes` itself.
                unsafe { write_bytes(out, plan_error::text(code)) }
            }
        }
    })
}

/// Destroys a session handle off render; null is a no-op.
///
/// # Safety
///
/// A nonnull `session` must be the unique live session handle returned by this library.
///
/// Thread: control, quiescent.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn miso_engine_v2_session_destroy(session: *mut Session) {
    catch_destroy(|| {
        if session.is_null() {
            return;
        }
        // SAFETY: Only the matching live kind may be reconstructed and destroyed.
        if unsafe { session_kind(session) } == RESULT_OK {
            // SAFETY: The ABI contract guarantees unique live ownership and quiescent destroy.
            drop(unsafe { Box::from_raw(session) });
        }
    });
}

/// Destroys a render-plan handle off render; null is a no-op.
///
/// # Safety
///
/// A nonnull `plan` must be the unique live plan handle returned by this library.
///
/// Thread: control, quiescent.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn miso_engine_v2_plan_destroy(plan: *mut Plan) {
    catch_destroy(|| {
        if plan.is_null() {
            return;
        }
        // SAFETY: Only the matching live kind may be reconstructed and destroyed.
        if unsafe { plan_kind(plan) } == RESULT_OK {
            // SAFETY: The ABI contract guarantees unique live ownership and quiescent destroy.
            drop(unsafe { Box::from_raw(plan) });
        }
    });
}

#[cfg(test)]
pub(crate) fn test_source_submit(
    session: *mut Session,
    source_id: &[u8],
    chunk: &SourceChunk,
    report: &mut SubmitReport,
) -> u32 {
    // SAFETY: Test callers retain the live session and all borrowed ABI storage for this call.
    unsafe {
        miso_engine_v2_source_submit_planar_f32(
            session,
            source_id.as_ptr(),
            source_id.len() as u64,
            chunk,
            report,
        )
    }
}

#[cfg(test)]
pub(crate) fn test_submit_command(
    session: *mut Session,
    request: &[u8],
    output: &mut BytesOut,
) -> u32 {
    // SAFETY: Test callers retain the live session, request, and ABI output for this call.
    unsafe {
        miso_engine_v2_submit_command(session, request.as_ptr(), request.len() as u64, output)
    }
}

#[cfg(test)]
pub(crate) fn test_dequeue_event(session: *mut Session, lane: u32, output: &mut BytesOut) -> u32 {
    // SAFETY: Test callers retain the live session and ABI output for this call.
    unsafe { miso_engine_v2_dequeue_event(session, lane, output) }
}

#[cfg(test)]
pub(crate) fn test_session_state_summary(session: *mut Session) -> (u64, usize, u64, usize) {
    // SAFETY: Test callers retain the exclusively owned live session for this inspection.
    let state = unsafe { &(*session).state };
    state.test_state_summary()
}

#[cfg(test)]
pub(crate) fn test_transaction_snapshot(
    session: *mut Session,
) -> crate::runtime::TestTransactionSnapshot {
    // SAFETY: Test callers retain the exclusively owned live session for this inspection.
    unsafe { &(*session).state }.test_transaction_snapshot()
}

#[cfg(test)]
pub(crate) fn test_plan_snapshot(plan: *mut Plan) -> (u64, PlanResourceReport) {
    // SAFETY: Test callers retain the exclusively owned live plan for this inspection.
    let plan = unsafe { &(*plan).state };
    (plan.owner.next_absolute_sample(), plan.resources())
}

#[cfg(test)]
pub(crate) fn test_telemetry_counters(
    session: *mut Session,
) -> miso_engine_protocol::TelemetryCounters {
    // SAFETY: Test callers retain the exclusively owned live session for this inspection.
    unsafe { &(*session).state }.test_telemetry_counters()
}

#[cfg(test)]
pub(crate) fn test_set_structural_faults(
    session: *mut Session,
    faults: [Option<crate::runtime::TestStructuralFaultPhase>; 2],
) {
    // SAFETY: Test callers retain the exclusively owned live session for deterministic faults.
    unsafe { &mut (*session).state }.test_set_structural_faults(faults);
}

#[cfg(test)]
pub(crate) fn test_reset_lifecycle_observer() {
    crate::runtime::test_reset_lifecycle_observer();
}

#[cfg(test)]
pub(crate) fn test_owner_counters(session: *mut Session) -> crate::runtime::TestOwnerCounters {
    // SAFETY: Test callers retain the exclusively owned live session for this inspection.
    unsafe { &(*session).state }.test_owner_counters()
}

#[cfg(test)]
pub(crate) fn test_retained_capacities(session: *mut Session) -> [usize; 7] {
    // SAFETY: Test callers retain the exclusively owned live session for this inspection.
    unsafe { &(*session).state }.test_retained_capacities()
}

#[cfg(test)]
pub(crate) fn test_source_seek(
    session: *mut Session,
    source_id: &[u8],
    generation: u64,
    source_frame: u64,
) -> u32 {
    // SAFETY: Test callers retain the live session and borrowed source ID for this call.
    unsafe {
        miso_engine_v2_source_seek(
            session,
            source_id.as_ptr(),
            source_id.len() as u64,
            generation,
            source_frame,
        )
    }
}

#[cfg(test)]
pub(crate) fn test_render(plan: *mut Plan, absolute_sample: u64, output: &PlanarOutput) -> u32 {
    // SAFETY: Test callers retain the exclusive live plan and writable output for this call.
    unsafe { miso_engine_v2_render_f32_planar(plan, absolute_sample, output) }
}

#[cfg(test)]
pub(crate) fn test_session_destroy(session: *mut Session) {
    // SAFETY: Test callers transfer one unique quiescent live session exactly once.
    unsafe { miso_engine_v2_session_destroy(session) }
}

#[cfg(test)]
pub(crate) fn test_plan_destroy(plan: *mut Plan) {
    // SAFETY: Test callers transfer one unique quiescent live plan exactly once.
    unsafe { miso_engine_v2_plan_destroy(plan) }
}

#[cfg(test)]
pub(crate) fn test_lifecycle_counters() -> crate::runtime::TestOwnerCounters {
    crate::runtime::test_lifecycle_counters()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn query(out: *mut Capabilities) -> u32 {
        // SAFETY: Tests pass null deliberately or storage for one complete Capabilities value.
        unsafe { miso_engine_v2_query_capabilities(out) }
    }

    fn create(config: *const EngineConfig, out: *mut *mut Engine) -> u32 {
        // SAFETY: Tests pass valid local pointer storage, with deliberate null handled by the ABI.
        unsafe { miso_engine_v2_engine_create(config, out) }
    }

    fn destroy(engine: *mut Engine) {
        // SAFETY: Tests pass null or the unique live engine returned by `create` exactly once.
        unsafe { miso_engine_v2_engine_destroy(engine) }
    }

    fn seek(
        session: *mut Session,
        source_id: *const u8,
        source_id_bytes: u64,
        generation: u64,
        source_frame: u64,
    ) -> u32 {
        // SAFETY: The wrong-kind test passes a live engine handle cast to the common opaque header;
        // the entrypoint rejects it before inspecting the deliberately null source ID.
        unsafe {
            miso_engine_v2_source_seek(
                session,
                source_id,
                source_id_bytes,
                generation,
                source_frame,
            )
        }
    }

    fn last_error(live_handle: *const c_void, out: *mut BytesOut) -> u32 {
        // SAFETY: Tests pass one live engine handle and writable BytesOut storage.
        unsafe { miso_engine_v2_last_error(live_handle, out) }
    }

    fn config() -> EngineConfig {
        EngineConfig {
            struct_size: crate::ENGINE_CONFIG_SIZE,
            abi_version: ABI_VERSION,
            reserved: [0; 4],
        }
    }

    fn limits() -> CompileLimits {
        CompileLimits {
            struct_size: crate::COMPILE_LIMITS_SIZE,
            source_ring_frames: 1_024,
            maximum_automation_spans_per_block: 128,
            reserved0: 0,
            maximum_toml_bytes: 1_000_000,
            maximum_diagnostic_bytes: 4_096,
            maximum_tracks: 100,
            maximum_sources: 100,
            maximum_routes: 100,
            maximum_effects: 100,
            maximum_graph_session_plus_plan_bytes: 100_000_000,
            maximum_source_total_bytes: 10_000_000,
            maximum_source_overhead_bytes: 10_000_000,
            maximum_effect_state_bytes: 100_000_000,
            maximum_effect_scratch_bytes: 100_000_000,
            maximum_builtin_retained_bytes: 100_000_000,
            maximum_capi_retained_bytes: 10_000_000,
            maximum_named_allocation_bytes: 100_000_000,
            maximum_meter_streams: 1,
            maximum_meter_items: 1,
            maximum_meter_bytes: 1,
            maximum_control_frame_bytes: 4_096,
            maximum_replay_bytes: 8_192,
            maximum_replay_entries: 16,
            reserved: [0; 4],
        }
    }

    /// Compiles the pinned nine-track fixture and returns its three live handles.
    fn compiled_fixture() -> (*mut Engine, *mut Session, *mut Plan) {
        const TOML: &[u8] =
            include_bytes!("../../../fixtures/session/v1/parametric-eq-nine-track.toml");
        let mut engine = ptr::null_mut();
        assert_eq!(create(&config(), &mut engine), RESULT_OK);
        let mut diagnostics = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: ptr::null_mut(),
            capacity_bytes: 0,
            required_bytes: u64::MAX,
        };
        let mut session = ptr::dangling_mut::<Session>();
        let mut plan = ptr::dangling_mut::<Plan>();
        // SAFETY: Every pointer names a complete local ABI value or the immutable fixture bytes.
        let result = unsafe {
            miso_engine_v2_compile_session(
                engine,
                TOML.as_ptr(),
                TOML.len() as u64,
                &limits(),
                &mut diagnostics,
                &mut session,
                &mut plan,
            )
        };
        assert_eq!(result, RESULT_OK);
        assert!(!session.is_null() && !plan.is_null());
        (engine, session, plan)
    }

    /// Destroys the three handles of [`compiled_fixture`] in the documented quiescent order.
    fn destroy_fixture(engine: *mut Engine, session: *mut Session, plan: *mut Plan) {
        // SAFETY: Each handle is the unique live handle of its kind and no call is in flight.
        unsafe {
            miso_engine_v2_plan_destroy(plan);
            miso_engine_v2_session_destroy(session);
        }
        destroy(engine);
    }

    /// Carries one live plan pointer into a scoped thread with its provenance intact.
    ///
    /// The ABI's own contract is that a plan may be rendered on one thread while another queries
    /// it, so the pointer itself must cross the thread boundary. An integer round trip would
    /// launder the provenance this test exists to check, so the pointer is moved as a pointer.
    #[derive(Clone, Copy)]
    struct SendPlanPtr(*mut Plan);

    // SAFETY: The ABI documents plan handles as usable from more than one thread under the split
    // ownership contract in the C header; this wrapper only moves the pointer, and every call made
    // through it in this test obeys that contract.
    unsafe impl Send for SendPlanPtr {}

    impl SendPlanPtr {
        fn get(self) -> *mut Plan {
            self.0
        }
    }

    /// A zeroed ABI V1 report with the exact frozen struct size.
    fn empty_report() -> PlanResourceReport {
        PlanResourceReport {
            struct_size: crate::PLAN_RESOURCE_REPORT_SIZE,
            abi_version: 0,
            sample_rate_hz: 0,
            quantum_frames: 0,
            source_count: 0,
            track_count: 0,
            latency_samples: 0,
            tail_kind: 0,
            tail_samples: 0,
            graph_session_plus_plan_bytes: 0,
            graph_incremental_plan_bytes: 0,
            graph_metadata_bytes: 0,
            graph_delay_bytes: 0,
            effect_bank_scratch_bytes: 0,
            effect_bank_runtime_buffer_bytes: 0,
            effect_bank_metadata_bytes: 0,
            builtin_bank_bytes: 0,
            builtin_bank_scratch_bytes: 0,
            source_pcm_payload_bytes: 0,
            source_overhead_bytes: 0,
            source_total_bytes: 0,
            effect_scalar_state_bytes: 0,
            effect_scalar_scratch_bytes: 0,
            builtin_processor_payload_bytes: 0,
            builtin_meter_payload_bytes: 0,
            builtin_retained_payload_bytes: 0,
            capi_retained_bytes: 0,
            largest_named_allocation_bytes: 0,
            reserved: [0; 4],
        }
    }

    /// Reads a handle diagnostic into an owned buffer sized by its own required-bytes query.
    fn read_last_error(live_handle: *const c_void) -> Vec<u8> {
        let mut query = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: ptr::null_mut(),
            capacity_bytes: 0,
            required_bytes: 0,
        };
        let probe = last_error(live_handle, &mut query);
        assert!(probe == RESULT_OK || probe == RESULT_BUFFER_TOO_SMALL);
        let mut storage = vec![0_u8; query.required_bytes as usize];
        query.data = storage.as_mut_ptr();
        query.capacity_bytes = storage.len() as u64;
        assert_eq!(last_error(live_handle, &mut query), RESULT_OK);
        storage
    }

    #[test]
    fn version_and_capabilities_are_exact() {
        assert_eq!(miso_engine_v2_abi_version(), ABI_VERSION);
        let mut capabilities = Capabilities {
            struct_size: CAPABILITIES_SIZE,
            abi_version: 0,
            exact_launch_rate_mask: 0,
            feature_mask: 0,
            reserved: [0; 4],
        };
        assert_eq!(query(&mut capabilities), RESULT_OK);
        assert_eq!(capabilities.abi_version, ABI_VERSION);
        assert_eq!(capabilities.exact_launch_rate_mask, 0x0f);
        assert_eq!(capabilities.feature_mask, 0x1f);
        assert_eq!(capabilities.reserved, [0; 4]);
    }

    #[test]
    fn query_rejects_null_and_wrong_size_without_a_write() {
        assert_eq!(query(ptr::null_mut()), RESULT_INVALID_ARGUMENT);
        let mut capabilities = Capabilities {
            struct_size: CAPABILITIES_SIZE - 1,
            abi_version: 77,
            exact_launch_rate_mask: 78,
            feature_mask: 79,
            reserved: [80; 4],
        };
        assert_eq!(query(&mut capabilities), RESULT_INVALID_ARGUMENT);
        assert_eq!(capabilities.abi_version, 77);
        assert_eq!(capabilities.reserved, [80; 4]);

        capabilities.struct_size = CAPABILITIES_SIZE;
        assert_eq!(query(&mut capabilities), RESULT_INVALID_ARGUMENT);
        assert_eq!(capabilities.abi_version, 77);
        assert_eq!(capabilities.reserved, [80; 4]);
    }

    #[test]
    fn engine_creation_is_transactional_and_validates() {
        let mut engine = ptr::dangling_mut::<Engine>();
        let mut wrong_version = config();
        wrong_version.abi_version += 1;
        assert_eq!(create(&wrong_version, &mut engine), RESULT_ABI_MISMATCH);
        assert!(engine.is_null());

        let mut nonzero_reserved = config();
        nonzero_reserved.reserved[2] = 1;
        assert_eq!(
            create(&nonzero_reserved, &mut engine),
            RESULT_INVALID_ARGUMENT
        );
        assert!(engine.is_null());

        assert_eq!(create(&config(), &mut engine), RESULT_OK);
        assert!(!engine.is_null());
        destroy(engine);
        destroy(ptr::null_mut());
    }

    #[test]
    fn wrong_live_handle_kind_is_rejected_by_stub() {
        let mut engine = ptr::null_mut();
        assert_eq!(create(&config(), &mut engine), RESULT_OK);
        let result = seek(engine.cast(), ptr::null(), 0, 0, 0);
        assert_eq!(result, RESULT_WRONG_HANDLE);
        destroy(engine);
    }

    #[test]
    fn engine_last_error_uses_empty_query_result() {
        let mut engine = ptr::null_mut();
        assert_eq!(create(&config(), &mut engine), RESULT_OK);
        let mut output = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: ptr::null_mut(),
            capacity_bytes: 0,
            required_bytes: u64::MAX,
        };
        assert_eq!(last_error(engine.cast(), &mut output), RESULT_OK);
        assert_eq!(output.required_bytes, 0);
        destroy(engine);
    }

    #[test]
    fn compile_publishes_both_children_and_source_control_is_region_checked() {
        const TOML: &[u8] =
            include_bytes!("../../../fixtures/session/v1/parametric-eq-nine-track.toml");
        let mut engine = ptr::null_mut();
        assert_eq!(create(&config(), &mut engine), RESULT_OK);
        let mut diagnostics = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: ptr::null_mut(),
            capacity_bytes: 0,
            required_bytes: u64::MAX,
        };
        let mut session = ptr::dangling_mut::<Session>();
        let mut plan = ptr::dangling_mut::<Plan>();
        // SAFETY: Every pointer names a complete local ABI value or the immutable fixture bytes.
        let result = unsafe {
            miso_engine_v2_compile_session(
                engine,
                TOML.as_ptr(),
                TOML.len() as u64,
                &limits(),
                &mut diagnostics,
                &mut session,
                &mut plan,
            )
        };
        assert_eq!(result, RESULT_OK);
        assert_eq!(diagnostics.required_bytes, 0);
        assert!(!session.is_null());
        assert!(!plan.is_null());

        let mut resources = PlanResourceReport {
            struct_size: crate::PLAN_RESOURCE_REPORT_SIZE,
            abi_version: 0,
            sample_rate_hz: 0,
            quantum_frames: 0,
            source_count: 0,
            track_count: 0,
            latency_samples: 0,
            tail_kind: 0,
            tail_samples: 0,
            graph_session_plus_plan_bytes: 0,
            graph_incremental_plan_bytes: 0,
            graph_metadata_bytes: 0,
            graph_delay_bytes: 0,
            effect_bank_scratch_bytes: 0,
            effect_bank_runtime_buffer_bytes: 0,
            effect_bank_metadata_bytes: 0,
            builtin_bank_bytes: 0,
            builtin_bank_scratch_bytes: 0,
            source_pcm_payload_bytes: 0,
            source_overhead_bytes: 0,
            source_total_bytes: 0,
            effect_scalar_state_bytes: 0,
            effect_scalar_scratch_bytes: 0,
            builtin_processor_payload_bytes: 0,
            builtin_meter_payload_bytes: 0,
            builtin_retained_payload_bytes: 0,
            capi_retained_bytes: 0,
            largest_named_allocation_bytes: 0,
            reserved: [u64::MAX; 4],
        };
        assert_eq!(
            // SAFETY: The plan is live and `resources` is writable storage of the exact size.
            unsafe { miso_engine_v2_plan_resources(plan, &mut resources) },
            RESULT_INVALID_ARGUMENT
        );
        assert_eq!(resources.abi_version, 0);
        assert_eq!(resources.reserved, [u64::MAX; 4]);
        resources.reserved = [0; 4];
        assert_eq!(
            // SAFETY: The plan is live and `resources` is writable storage of the exact size.
            unsafe { miso_engine_v2_plan_resources(plan, &mut resources) },
            RESULT_OK
        );
        assert_eq!(resources.sample_rate_hz, 48_000);
        assert_eq!(resources.quantum_frames, 128);
        assert_eq!(resources.source_count, 1);
        assert_eq!(resources.track_count, 9);
        assert_eq!(resources.reserved, [0; 4]);

        let left = [0.25_f32; 128];
        let right = [-0.5_f32; 128];
        let planes = [left.as_ptr(), right.as_ptr()];
        let chunk = SourceChunk {
            struct_size: crate::SOURCE_CHUNK_SIZE,
            sample_rate_hz: 48_000,
            generation: 1,
            start_frame: 0,
            planes: planes.as_ptr(),
            plane_count: 2,
            frames: 128,
            end_of_region: 0,
            reserved0: 0,
        };
        let mut submitted = SubmitReport {
            struct_size: crate::SUBMIT_REPORT_SIZE,
            reserved0: 0,
            accepted_frames: 0,
            cumulative_written_frames: 0,
            active_generation: 0,
        };
        assert_eq!(
            // SAFETY: All borrowed chunk planes and ABI structs remain live for the complete call.
            unsafe {
                miso_engine_v2_source_submit_planar_f32(
                    session,
                    b"fixture-source".as_ptr(),
                    14,
                    &chunk,
                    &mut submitted,
                )
            },
            RESULT_OK
        );
        assert_eq!(submitted.accepted_frames, 128);

        let mut pcm = vec![f32::NAN; 256];
        let mut output = PlanarOutput {
            struct_size: crate::PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: pcm.as_mut_ptr(),
            sample_capacity: pcm.len() as u64,
            frames: 128,
            plane_stride_samples: 128,
            reserved: [0; 2],
        };
        assert_eq!(
            // SAFETY: The plan is live and the complete contiguous planar region is writable.
            unsafe { miso_engine_v2_render_f32_planar(plan, 128, &output) },
            RESULT_RENDER_REJECTED
        );
        assert!(pcm.iter().all(|sample| sample.is_nan()));
        output.sample_capacity = 255;
        assert_eq!(
            // SAFETY: The intentionally short declared capacity is rejected before dereference.
            unsafe { miso_engine_v2_render_f32_planar(plan, 0, &output) },
            RESULT_RENDER_REJECTED
        );
        assert!(pcm.iter().all(|sample| sample.is_nan()));
        output.sample_capacity = 256;
        assert_eq!(
            // SAFETY: The output descriptor now satisfies the complete render contract.
            unsafe { miso_engine_v2_render_f32_planar(plan, 0, &output) },
            RESULT_OK
        );
        assert!(pcm.iter().all(|sample| sample.is_finite()));
        assert_eq!(
            // SAFETY: The stale time is rejected before entering the prepared plan.
            unsafe { miso_engine_v2_render_f32_planar(plan, 0, &output) },
            RESULT_RENDER_REJECTED
        );
        assert_eq!(
            // SAFETY: The exact next block time advances the same live exclusive plan.
            unsafe { miso_engine_v2_render_f32_planar(plan, 128, &output) },
            RESULT_OK
        );
        assert_eq!(
            seek(session, b"fixture-source".as_ptr(), 14, 2, 48_001),
            RESULT_INVALID_ARGUMENT
        );
        let mut source_error = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: ptr::null_mut(),
            capacity_bytes: 0,
            required_bytes: 0,
        };
        assert_eq!(
            last_error(session.cast(), &mut source_error),
            RESULT_BUFFER_TOO_SMALL
        );
        let mut source_error_storage = vec![0_u8; source_error.required_bytes as usize];
        source_error.data = source_error_storage.as_mut_ptr();
        source_error.capacity_bytes = source_error_storage.len() as u64;
        assert_eq!(last_error(session.cast(), &mut source_error), RESULT_OK);
        // F6: the out-of-region seek now names the rule it broke, not "rejected".
        assert_eq!(&source_error_storage, b"source.region.outside");
        assert_eq!(
            seek(session, b"fixture-source".as_ptr(), 14, 2, 48_000),
            RESULT_OK
        );
        source_error.data = ptr::null_mut();
        source_error.capacity_bytes = 0;
        assert_eq!(last_error(session.cast(), &mut source_error), RESULT_OK);
        assert_eq!(source_error.required_bytes, 0);

        let codec = miso_engine_protocol::ProtocolCodec::default();
        let mut request = vec![0_u8; 128];
        let request_len = codec
            .encode_command_frame_into(
                &miso_engine_protocol::TypedCommandFrame {
                    request_id: miso_engine_protocol::RequestId::new(1).expect("request"),
                    expected_revision: miso_engine_protocol::ExpectedRevision::Any,
                    payload: miso_engine_protocol::CommandPayload::CapabilitiesGet,
                },
                &mut request,
            )
            .expect("capability command");
        request.truncate(request_len);
        let mut response = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: ptr::null_mut(),
            capacity_bytes: 0,
            required_bytes: 0,
        };
        assert_eq!(
            // SAFETY: The complete request and output descriptor remain live for this call.
            unsafe {
                miso_engine_v2_submit_command(
                    session,
                    request.as_ptr(),
                    request.len() as u64,
                    &mut response,
                )
            },
            RESULT_BUFFER_TOO_SMALL
        );
        assert_eq!(response.required_bytes, 4_096);
        let mut response_bytes = vec![0xa5; response.required_bytes as usize];
        response.data = response_bytes.as_mut_ptr();
        response.capacity_bytes = response_bytes.len() as u64;
        assert_eq!(
            // SAFETY: Retry storage satisfies the advertised complete response reservation.
            unsafe {
                miso_engine_v2_submit_command(
                    session,
                    request.as_ptr(),
                    request.len() as u64,
                    &mut response,
                )
            },
            RESULT_OK
        );
        response_bytes.truncate(response.required_bytes as usize);
        let mut decode_fields = [0_u16; 64];
        assert!(matches!(
            codec
                .decode_typed_response(
                    &response_bytes,
                    &mut miso_engine_protocol::DecodeScratch::new(&mut decode_fields),
                )
                .expect("canonical capability response"),
            miso_engine_protocol::DecodedTypedResponseFrame::Success { header, .. }
                if header.request_id.get() == 1
        ));

        let mut invalid_event_storage = [0xa5_u8; 8];
        let mut event_out = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: invalid_event_storage.as_mut_ptr(),
            capacity_bytes: invalid_event_storage.len() as u64,
            required_bytes: 77,
        };
        assert_eq!(
            // SAFETY: The live session and complete output descriptor remain valid.
            unsafe { miso_engine_v2_dequeue_event(session, 2, &mut event_out) },
            RESULT_INVALID_ARGUMENT
        );
        assert_eq!(event_out.required_bytes, 77);
        assert_eq!(invalid_event_storage, [0xa5; 8]);
        assert_eq!(
            // SAFETY: The reliable lane is valid and currently empty.
            unsafe {
                miso_engine_v2_dequeue_event(session, crate::EVENT_LANE_RELIABLE, &mut event_out)
            },
            RESULT_OK
        );
        assert_eq!(event_out.required_bytes, 0);

        let edit = miso_engine_protocol::SessionEdit::SetSessionId {
            session_id: miso_engine_session::StableId::parse("capi-ffi-replaced")
                .expect("stable ID"),
        };
        let mut structural_request = vec![0_u8; 4_096];
        let structural_len = codec
            .encode_command_frame_into(
                &miso_engine_protocol::TypedCommandFrame {
                    request_id: miso_engine_protocol::RequestId::new(2).expect("request"),
                    expected_revision: miso_engine_protocol::ExpectedRevision::Exact(
                        miso_engine_protocol::SessionRevision(42),
                    ),
                    payload: miso_engine_protocol::CommandPayload::SessionTransactionApply(
                        core::slice::from_ref(&edit),
                    ),
                },
                &mut structural_request,
            )
            .expect("structural command");
        structural_request.truncate(structural_len);
        response.required_bytes = 0;
        response_bytes.resize(response.capacity_bytes as usize, 0xa5);
        response_bytes.fill(0xa5);
        assert_eq!(
            // SAFETY: The structural request and admitted response storage are complete.
            unsafe {
                miso_engine_v2_submit_command(
                    session,
                    structural_request.as_ptr(),
                    structural_request.len() as u64,
                    &mut response,
                )
            },
            RESULT_OK
        );
        let mut structural_fields = [0_u16; 64];
        assert!(matches!(
            codec
                .decode_typed_response(
                    &response_bytes[..response.required_bytes as usize],
                    &mut miso_engine_protocol::DecodeScratch::new(&mut structural_fields),
                )
                .expect("structural response"),
            miso_engine_protocol::DecodedTypedResponseFrame::Success { header, .. }
                if header.revision == miso_engine_protocol::SessionRevision(43)
        ));

        event_out.data = ptr::null_mut();
        event_out.capacity_bytes = 0;
        event_out.required_bytes = 0;
        assert_eq!(
            // SAFETY: A zero-capacity query is valid and consumes no reliable event.
            unsafe {
                miso_engine_v2_dequeue_event(session, crate::EVENT_LANE_RELIABLE, &mut event_out)
            },
            RESULT_BUFFER_TOO_SMALL
        );
        let event_bytes = event_out.required_bytes as usize;
        assert!(event_bytes > 1);
        let mut short_event = vec![0xa5_u8; event_bytes - 1];
        event_out.data = short_event.as_mut_ptr();
        event_out.capacity_bytes = short_event.len() as u64;
        assert_eq!(
            // SAFETY: The one-short buffer is valid for its declared capacity.
            unsafe {
                miso_engine_v2_dequeue_event(session, crate::EVENT_LANE_RELIABLE, &mut event_out)
            },
            RESULT_BUFFER_TOO_SMALL
        );
        assert_eq!(event_out.required_bytes as usize, event_bytes);
        assert!(short_event.iter().all(|byte| *byte == 0xa5));
        let mut reliable_event = vec![0xa5_u8; event_bytes];
        event_out.data = reliable_event.as_mut_ptr();
        event_out.capacity_bytes = reliable_event.len() as u64;
        assert_eq!(
            // SAFETY: The exact retry buffer receives the complete pending event.
            unsafe {
                miso_engine_v2_dequeue_event(session, crate::EVENT_LANE_RELIABLE, &mut event_out)
            },
            RESULT_OK
        );
        let mut event_fields = [0_u16; 64];
        assert!(matches!(
            codec
                .decode_typed_event(
                    &reliable_event,
                    &mut miso_engine_protocol::DecodeScratch::new(&mut event_fields),
                )
                .expect("reliable event"),
            miso_engine_protocol::DecodedTypedEventFrame {
                header,
                payload: miso_engine_protocol::DecodedEventPayload::SessionCommitted(_),
            } if header.revision == miso_engine_protocol::SessionRevision(43)
        ));

        assert_eq!(
            // SAFETY: The next exact boundary applies the matched replacement plan.
            unsafe { miso_engine_v2_render_f32_planar(plan, 256, &output) },
            RESULT_OK
        );

        // SAFETY: Each independently owned child and engine is destroyed exactly once, quiescent.
        unsafe {
            miso_engine_v2_session_destroy(session);
            miso_engine_v2_plan_destroy(plan);
        }
        destroy(engine);
    }

    #[test]
    fn compile_diagnostics_query_is_atomic_and_handle_local() {
        let mut engine = ptr::null_mut();
        assert_eq!(create(&config(), &mut engine), RESULT_OK);
        let invalid_utf8 = [0xff_u8];
        let mut diagnostics = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: ptr::null_mut(),
            capacity_bytes: 0,
            required_bytes: 0,
        };
        let mut session = ptr::dangling_mut::<Session>();
        let mut plan = ptr::dangling_mut::<Plan>();
        assert_eq!(
            // SAFETY: The input byte and all ABI values are valid for the duration of the call.
            unsafe {
                miso_engine_v2_compile_session(
                    engine,
                    invalid_utf8.as_ptr(),
                    1,
                    &limits(),
                    &mut diagnostics,
                    &mut session,
                    &mut plan,
                )
            },
            RESULT_BUFFER_TOO_SMALL
        );
        assert!(session.is_null());
        assert!(plan.is_null());
        assert!(diagnostics.required_bytes > 0);

        let mut undersized = vec![0xa5; diagnostics.required_bytes as usize - 1];
        diagnostics.data = undersized.as_mut_ptr();
        diagnostics.capacity_bytes = undersized.len() as u64;
        session = ptr::dangling_mut();
        plan = ptr::dangling_mut();
        assert_eq!(
            // SAFETY: The intentionally undersized output remains valid for its declared capacity.
            unsafe {
                miso_engine_v2_compile_session(
                    engine,
                    invalid_utf8.as_ptr(),
                    1,
                    &limits(),
                    &mut diagnostics,
                    &mut session,
                    &mut plan,
                )
            },
            RESULT_BUFFER_TOO_SMALL
        );
        assert!(session.is_null());
        assert!(plan.is_null());
        assert!(undersized.iter().all(|byte| *byte == 0xa5));

        let mut storage = vec![0xa5; diagnostics.required_bytes as usize];
        diagnostics.data = storage.as_mut_ptr();
        diagnostics.capacity_bytes = storage.len() as u64;
        session = ptr::dangling_mut();
        plan = ptr::dangling_mut();
        assert_eq!(
            // SAFETY: The retry output can hold the complete diagnostic and outputs are writable.
            unsafe {
                miso_engine_v2_compile_session(
                    engine,
                    invalid_utf8.as_ptr(),
                    1,
                    &limits(),
                    &mut diagnostics,
                    &mut session,
                    &mut plan,
                )
            },
            RESULT_COMPILE_REJECTED
        );
        assert!(session.is_null());
        assert!(plan.is_null());
        assert_eq!(&storage, b"capi.toml.utf8\t$\n");

        let mut error_query = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: ptr::null_mut(),
            capacity_bytes: 0,
            required_bytes: 0,
        };
        assert_eq!(
            last_error(engine.cast(), &mut error_query),
            RESULT_BUFFER_TOO_SMALL
        );
        assert_eq!(error_query.required_bytes, diagnostics.required_bytes);
        destroy(engine);
    }

    /// F6, end to end: a rejected source submission or seek reaches a C host as the diagnostic for
    /// the rule it broke, through the real entry point and the real `last_error` path. Every one of
    /// these used to be `RESULT_INVALID_ARGUMENT` with `source.submit.rejected` or
    /// `source.seek.rejected`, which told a host nothing about what to fix.
    #[test]
    fn source_rejections_reach_the_c_host_as_their_own_diagnostic() {
        let (engine, session, plan) = compiled_fixture();
        let left = [0.25_f32; 128];
        let right = [-0.5_f32; 128];
        let stereo = [left.as_ptr(), right.as_ptr()];
        let three = [left.as_ptr(), right.as_ptr(), left.as_ptr()];
        let base = || SourceChunk {
            struct_size: crate::SOURCE_CHUNK_SIZE,
            sample_rate_hz: 48_000,
            generation: 1,
            start_frame: 0,
            planes: stereo.as_ptr(),
            plane_count: 2,
            frames: 128,
            end_of_region: 0,
            reserved0: 0,
        };
        let mut report = SubmitReport {
            struct_size: crate::SUBMIT_REPORT_SIZE,
            reserved0: 0,
            accepted_frames: 0,
            cumulative_written_frames: 0,
            active_generation: 0,
        };

        /// `(why, source ID, the one rule this row breaks, the diagnostic a C host must read)`.
        type Row<'a> = (&'a str, &'a [u8], &'a dyn Fn(&mut SourceChunk), &'a [u8]);
        // Each row breaks exactly one rule, in the order the facade checks them.
        let rows: [Row<'_>; 7] = [
            (
                "no source carries this ID",
                b"absent-source",
                &|_| {},
                b"source.id.unknown",
            ),
            (
                "start + frames does not fit in u64",
                b"fixture-source",
                &|chunk| chunk.start_frame = u64::MAX,
                b"source.region.overflow",
            ),
            (
                "the chunk rate is not the source's",
                b"fixture-source",
                &|chunk| chunk.sample_rate_hz = 44_100,
                b"source.rate.mismatch",
            ),
            (
                "three planes for a stereo source",
                b"fixture-source",
                &|chunk| {
                    chunk.planes = three.as_ptr();
                    chunk.plane_count = 3;
                },
                b"source.channels.mismatch",
            ),
            (
                "the chunk runs past the mapped region",
                b"fixture-source",
                &|chunk| chunk.start_frame = 47_999,
                b"source.region.outside",
            ),
            (
                "a chunk ending at the region end must say so",
                b"fixture-source",
                &|chunk| chunk.start_frame = 47_872,
                b"source.region.end_mismatch",
            ),
            (
                "generation zero is reserved",
                b"fixture-source",
                &|chunk| chunk.generation = 0,
                b"source.generation.zero",
            ),
        ];
        for (why, id, mutate, diagnostic) in rows {
            let mut chunk = base();
            mutate(&mut chunk);
            assert_eq!(
                // SAFETY: The session is live and every borrowed plane and ABI struct outlives the
                // call; the submission is rejected before any plane is read.
                unsafe {
                    miso_engine_v2_source_submit_planar_f32(
                        session,
                        id.as_ptr(),
                        id.len() as u64,
                        &chunk,
                        &mut report,
                    )
                },
                RESULT_INVALID_ARGUMENT,
                "{why}"
            );
            assert_eq!(read_last_error(session.cast()), diagnostic, "{why}");
        }

        // The seek path reports through the same table.
        assert_eq!(
            seek(session, b"fixture-source".as_ptr(), 14, 1, 48_001),
            RESULT_INVALID_ARGUMENT
        );
        assert_eq!(read_last_error(session.cast()), b"source.region.outside");
        assert_eq!(
            seek(session, b"absent-source".as_ptr(), 13, 1, 0),
            RESULT_INVALID_ARGUMENT
        );
        assert_eq!(read_last_error(session.cast()), b"source.id.unknown");

        // An accepted submission clears the diagnostic, so a stale string can never be mistaken
        // for a fresh rejection.
        assert_eq!(
            // SAFETY: As above; this chunk satisfies every rule.
            unsafe {
                miso_engine_v2_source_submit_planar_f32(
                    session,
                    b"fixture-source".as_ptr(),
                    14,
                    &base(),
                    &mut report,
                )
            },
            RESULT_OK
        );
        assert_eq!(report.accepted_frames, 128);
        assert_eq!(read_last_error(session.cast()), b"");
        destroy_fixture(engine, session, plan);
    }

    /// F4: one validation pass, one diagnostic per rule. Each rejection names the single check it
    /// failed -- five of these used to share the string `render.contract.rejected` -- and none of
    /// them writes a sample, so the host buffer still holds its pre-call NaN fill.
    #[test]
    fn render_rejections_name_their_single_check() {
        let (engine, session, plan) = compiled_fixture();
        let mut pcm = vec![f32::NAN; 512];
        let base = PlanarOutput {
            struct_size: crate::PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: pcm.as_mut_ptr(),
            sample_capacity: 512,
            frames: 128,
            plane_stride_samples: 128,
            reserved: [0; 2],
        };

        let cases: [(&str, PlanarOutput, u64, &[u8]); 4] = [
            (
                "a short frame count is core's output shape",
                PlanarOutput { frames: 64, ..base },
                0,
                b"render.output.shape",
            ),
            (
                "a stride below the frame count is the layout check",
                PlanarOutput {
                    plane_stride_samples: 100,
                    ..base
                },
                0,
                b"render.output.layout",
            ),
            (
                "two planes past the declared capacity is the layout check",
                PlanarOutput {
                    sample_capacity: 255,
                    ..base
                },
                0,
                b"render.output.layout",
            ),
            (
                "a capacity no slice can address is the platform check",
                PlanarOutput {
                    sample_capacity: u64::MAX,
                    ..base
                },
                0,
                b"render.output.platform",
            ),
        ];
        for (why, output, time, diagnostic) in cases {
            assert_eq!(
                // SAFETY: The plan is live and the descriptor is rejected before any write.
                unsafe { miso_engine_v2_render_f32_planar(plan, time, &output) },
                RESULT_RENDER_REJECTED,
                "{why}"
            );
            assert_eq!(read_last_error(plan.cast()), diagnostic, "{why}");
            assert!(
                pcm.iter().all(|sample| sample.is_nan()),
                "{why}: a rejected render must not write"
            );
        }

        // The clock is the plan's, so the first block must start at zero.
        assert_eq!(
            // SAFETY: The plan is live and the descriptor is valid.
            unsafe { miso_engine_v2_render_f32_planar(plan, 128, &base) },
            RESULT_RENDER_REJECTED
        );
        assert_eq!(read_last_error(plan.cast()), b"render.time.discontinuity");
        assert!(pcm.iter().all(|sample| sample.is_nan()));

        // `PlanarOutput` is stereo by contract: a channel count of one is an ABI mismatch, not a
        // render rejection, and never reaches the plan.
        assert_eq!(
            // SAFETY: The plan is live and the descriptor is rejected by the ABI check.
            unsafe {
                miso_engine_v2_render_f32_planar(
                    plan,
                    0,
                    &PlanarOutput {
                        channels: 1,
                        ..base
                    },
                )
            },
            RESULT_INVALID_ARGUMENT
        );

        assert_eq!(
            // SAFETY: The plan is live and the descriptor is valid.
            unsafe { miso_engine_v2_render_f32_planar(plan, 0, &base) },
            RESULT_OK
        );
        assert!(
            pcm[..128].iter().all(|sample| !sample.is_nan()),
            "an accepted render writes the left plane"
        );
        assert!(
            pcm[128..256].iter().all(|sample| !sample.is_nan()),
            "an accepted render writes the right plane"
        );
        assert_eq!(read_last_error(plan.cast()), b"");
        destroy_fixture(engine, session, plan);
    }

    /// F2 (a): `miso_engine_v2_plan_resources` takes a `const` plan and must be pure. Before this
    /// fix it cleared the render diagnostic through a `RefCell` the render thread also writes.
    #[test]
    fn plan_resources_does_not_clear_the_render_diagnostic() {
        let (engine, session, plan) = compiled_fixture();
        let mut pcm = vec![0.0_f32; 256];
        let output = PlanarOutput {
            struct_size: crate::PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: pcm.as_mut_ptr(),
            sample_capacity: 255,
            frames: 128,
            plane_stride_samples: 128,
            reserved: [0; 2],
        };
        assert_eq!(
            // SAFETY: The plan is live; the short declared capacity is rejected before any write.
            unsafe { miso_engine_v2_render_f32_planar(plan, 0, &output) },
            RESULT_RENDER_REJECTED
        );
        assert_eq!(
            read_last_error(plan.cast()),
            plan_error::text(plan_error::OUTPUT_LAYOUT)
        );
        let mut resources = empty_report();
        assert_eq!(
            // SAFETY: The plan is live and `resources` is writable storage of the exact size.
            unsafe { miso_engine_v2_plan_resources(plan, &mut resources) },
            RESULT_OK
        );
        assert_eq!(resources.quantum_frames, 128);
        assert_eq!(
            read_last_error(plan.cast()),
            plan_error::text(plan_error::OUTPUT_LAYOUT),
            "a const-plan query must not clear the render diagnostic"
        );
        destroy_fixture(engine, session, plan);
    }

    /// F2 (a, b): `plan_resources` and `last_error` on a plan are documented as any-thread calls
    /// that may run concurrently with `render_f32_planar`. They previously reached the plan through
    /// `&*plan` while render held `&mut *plan`, so the contract was undefined behaviour rather than
    /// a property of the code. Both queries now project disjoint fields.
    #[test]
    fn plan_queries_are_pure_and_concurrent_with_render() {
        const BLOCKS: u64 = if cfg!(miri) { 16 } else { 2_000 };
        let (engine, session, plan) = compiled_fixture();
        let rendering = std::sync::atomic::AtomicBool::new(true);
        let render_handle = SendPlanPtr(plan);
        let query_handle = SendPlanPtr(plan);
        let rendering_ref = &rendering;
        std::thread::scope(|scope| {
            scope.spawn(move || {
                let plan = render_handle.get();
                let mut pcm = vec![0.0_f32; 256];
                let output = PlanarOutput {
                    struct_size: crate::PLANAR_OUTPUT_SIZE,
                    channels: 2,
                    samples: pcm.as_mut_ptr(),
                    sample_capacity: pcm.len() as u64,
                    frames: 128,
                    plane_stride_samples: 128,
                    reserved: [0; 2],
                };
                for block in 0..BLOCKS {
                    assert_eq!(
                        // SAFETY: This thread is the exclusive render owner of the live plan and
                        // owns the complete contiguous output region for the call.
                        unsafe { miso_engine_v2_render_f32_planar(plan, block * 128, &output) },
                        RESULT_OK
                    );
                }
                rendering_ref.store(false, std::sync::atomic::Ordering::Release);
            });
            scope.spawn(move || {
                let plan = query_handle.get().cast_const();
                let mut queries = 0_u64;
                let mut storage = [0_u8; 64];
                while rendering_ref.load(std::sync::atomic::Ordering::Acquire) || queries == 0 {
                    let mut resources = empty_report();
                    assert_eq!(
                        // SAFETY: The plan is live and this any-thread query only reads the
                        // immutable `queries` projection and the atomic diagnostic slot.
                        unsafe { miso_engine_v2_plan_resources(plan, &mut resources) },
                        RESULT_OK
                    );
                    assert_eq!(resources.quantum_frames, 128);
                    assert_eq!(resources.sample_rate_hz, 48_000);
                    assert_eq!(resources.reserved, [0; 4]);
                    let mut error = BytesOut {
                        struct_size: BYTES_OUT_SIZE,
                        reserved0: 0,
                        data: storage.as_mut_ptr(),
                        capacity_bytes: storage.len() as u64,
                        required_bytes: u64::MAX,
                    };
                    assert_eq!(
                        // SAFETY: See above; the diagnostic query loads one atomic word.
                        unsafe { miso_engine_v2_last_error(plan.cast(), &mut error) },
                        RESULT_OK
                    );
                    assert_eq!(error.required_bytes, 0);
                    queries += 1;
                }
                assert!(queries > 0);
            });
        });
        destroy_fixture(engine, session, plan);
    }

    /// F3: every caller-declared length is bounded by the call's own limit and by `isize::MAX`
    /// before the region becomes a slice. The pointer used here is dangling: if any check were
    /// missing, `from_raw_parts` would hit its debug precondition and abort the test process.
    #[test]
    fn oversized_borrowed_lengths_are_rejected_before_any_read() {
        let (engine, session, plan) = compiled_fixture();
        let dangling = ptr::NonNull::<u8>::dangling().as_ptr().cast_const();
        let left = [0.25_f32; 128];
        let right = [-0.5_f32; 128];
        let planes = [left.as_ptr(), right.as_ptr()];
        let chunk = SourceChunk {
            struct_size: crate::SOURCE_CHUNK_SIZE,
            sample_rate_hz: 48_000,
            generation: 1,
            start_frame: 0,
            planes: planes.as_ptr(),
            plane_count: 2,
            frames: 128,
            end_of_region: 0,
            reserved0: 0,
        };
        let mut submitted = SubmitReport {
            struct_size: crate::SUBMIT_REPORT_SIZE,
            reserved0: 0,
            accepted_frames: 0,
            cumulative_written_frames: 0,
            active_generation: 0,
        };
        for source_id_bytes in [u64::MAX, MAX_SOURCE_ID_BYTES + 1, MAX_BORROWED_BYTES + 1] {
            assert_eq!(
                // SAFETY: The oversized declared length is rejected before `dangling` is read.
                unsafe {
                    miso_engine_v2_source_submit_planar_f32(
                        session,
                        dangling,
                        source_id_bytes,
                        &chunk,
                        &mut submitted,
                    )
                },
                RESULT_INVALID_ARGUMENT
            );
            assert_eq!(submitted.accepted_frames, 0);
            assert_eq!(
                seek(session, dangling, source_id_bytes, 1, 0),
                RESULT_INVALID_ARGUMENT
            );
        }
        let mut response = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: ptr::null_mut(),
            capacity_bytes: 0,
            required_bytes: 0,
        };
        assert_eq!(
            // SAFETY: The oversized declared frame length is rejected before `dangling` is read.
            unsafe {
                miso_engine_v2_submit_command(
                    session,
                    dangling,
                    MAX_BORROWED_BYTES + 1,
                    &mut response,
                )
            },
            RESULT_INVALID_ARGUMENT
        );
        assert_eq!(response.required_bytes, 0);

        let mut unbounded = limits();
        unbounded.maximum_toml_bytes = u64::MAX;
        let mut diagnostics = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: ptr::null_mut(),
            capacity_bytes: 0,
            required_bytes: 0,
        };
        let mut oversized_session = ptr::null_mut();
        let mut oversized_plan = ptr::null_mut();
        assert_eq!(
            // SAFETY: The oversized declared TOML length is rejected before `dangling` is read.
            unsafe {
                miso_engine_v2_compile_session(
                    engine,
                    dangling,
                    MAX_BORROWED_BYTES + 1,
                    &unbounded,
                    &mut diagnostics,
                    &mut oversized_session,
                    &mut oversized_plan,
                )
            },
            RESULT_INVALID_ARGUMENT
        );
        assert!(oversized_session.is_null());
        assert!(oversized_plan.is_null());
        destroy_fixture(engine, session, plan);
    }

    /// F3: every float plane the ABI turns into a slice is null- and alignment-checked first. A
    /// misaligned pointer reaching `from_raw_parts` would abort the test process on its debug
    /// precondition instead of returning a typed code.
    #[test]
    fn misaligned_planes_and_output_are_rejected() {
        let (engine, session, plan) = compiled_fixture();
        let staging = vec![0.0_f32; 130];
        let base = staging.as_ptr().cast::<u8>();
        let misaligned_plane = base.wrapping_add(1).cast::<f32>();
        assert!(!misaligned_plane.is_aligned());
        let aligned_plane = staging.as_ptr();
        let planes = [misaligned_plane, aligned_plane];
        let mut chunk = SourceChunk {
            struct_size: crate::SOURCE_CHUNK_SIZE,
            sample_rate_hz: 48_000,
            generation: 1,
            start_frame: 0,
            planes: planes.as_ptr(),
            plane_count: 2,
            frames: 128,
            end_of_region: 0,
            reserved0: 0,
        };
        let mut submitted = SubmitReport {
            struct_size: crate::SUBMIT_REPORT_SIZE,
            reserved0: 0,
            accepted_frames: 0,
            cumulative_written_frames: 0,
            active_generation: 0,
        };
        assert_eq!(
            // SAFETY: The misaligned plane is rejected before it becomes a slice.
            unsafe {
                miso_engine_v2_source_submit_planar_f32(
                    session,
                    b"fixture-source".as_ptr(),
                    14,
                    &chunk,
                    &mut submitted,
                )
            },
            RESULT_INVALID_ARGUMENT
        );
        assert_eq!(submitted.accepted_frames, 0);

        let pointer_store = [0_u64; 4];
        let misaligned_planes = pointer_store
            .as_ptr()
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<*const f32>();
        assert!(!misaligned_planes.is_aligned());
        chunk.planes = misaligned_planes;
        assert_eq!(
            // SAFETY: The misaligned plane array is rejected before it becomes a slice.
            unsafe {
                miso_engine_v2_source_submit_planar_f32(
                    session,
                    b"fixture-source".as_ptr(),
                    14,
                    &chunk,
                    &mut submitted,
                )
            },
            RESULT_INVALID_ARGUMENT
        );
        assert_eq!(submitted.accepted_frames, 0);

        let mut pcm = vec![f32::NAN; 258];
        let mut output = PlanarOutput {
            struct_size: crate::PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: pcm.as_mut_ptr().cast::<u8>().wrapping_add(2).cast::<f32>(),
            sample_capacity: 256,
            frames: 128,
            plane_stride_samples: 128,
            reserved: [0; 2],
        };
        assert!(!output.samples.is_aligned());
        assert_eq!(
            // SAFETY: The misaligned output is rejected before it becomes a slice.
            unsafe { miso_engine_v2_render_f32_planar(plan, 0, &output) },
            RESULT_INVALID_ARGUMENT
        );
        assert!(pcm.iter().all(|sample| sample.is_nan()));
        assert_eq!(
            read_last_error(plan.cast()),
            plan_error::text(plan_error::OUTPUT_UNALIGNED)
        );

        output.samples = pcm.as_mut_ptr();
        assert_eq!(
            // SAFETY: The aligned output now satisfies the complete render contract.
            unsafe { miso_engine_v2_render_f32_planar(plan, 0, &output) },
            RESULT_OK
        );
        assert!(pcm[..256].iter().all(|sample| sample.is_finite()));
        assert_eq!(
            read_last_error(plan.cast()),
            plan_error::text(plan_error::NONE)
        );
        destroy_fixture(engine, session, plan);
    }

    /// F2 (b): the control/render split is a property of the code, not of a comment. Forming a
    /// reference to the whole `Plan` re-creates the whole-struct borrow that this job removed, so
    /// no such form may appear in this file.
    #[test]
    fn ffi_never_forms_a_whole_plan_reference() {
        const SOURCE: &str = include_str!("ffi.rs");
        let production = SOURCE
            .split("#[cfg(test)]\nmod tests {")
            .next()
            .expect("production region precedes the test module");
        assert!(production.contains("miso_engine_v2_render_f32_planar"));
        for form in ["&*plan", "&mut *plan", "&(*plan)", "&mut (*plan)"] {
            let mut hits = Vec::new();
            for (index, line) in production.lines().enumerate() {
                if line.trim_start().starts_with("//") {
                    continue;
                }
                let mut rest = line;
                while let Some(at) = rest.find(form) {
                    let tail = &rest[at + form.len()..];
                    // `&*plan.cast::<HandleHeader>()` borrows only the shared 16-byte header and
                    // `&*plan_error_slot(..)` names a different item; neither borrows the plan.
                    let projection = tail
                        .starts_with(|next: char| next.is_alphanumeric() || next == '_')
                        || tail.starts_with('.');
                    if !projection {
                        hits.push(format!("{}: {}", index + 1, line.trim()));
                    }
                    rest = tail;
                }
            }
            assert!(
                hits.is_empty(),
                "ffi.rs forms a whole-plan reference {form}: {hits:?}"
            );
        }
    }
}
