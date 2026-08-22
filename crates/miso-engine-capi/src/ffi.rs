//! Raw C-pointer ownership boundary.

#![allow(unsafe_code)]

use crate::{
    ABI_VERSION, BYTES_OUT_SIZE, BytesOut, CAPABILITIES_SIZE, Capabilities, CompileLimits,
    EXACT_LAUNCH_RATE_MASK, Engine, EngineConfig, FEATURE_MASK, HandleHeader, Plan,
    PlanResourceReport, PlanarOutput, RESULT_ABI_MISMATCH, RESULT_BACKPRESSURE,
    RESULT_BUFFER_TOO_SMALL, RESULT_COMPILE_REJECTED, RESULT_INTERNAL, RESULT_INVALID_ARGUMENT,
    RESULT_OK, RESULT_RENDER_REJECTED, RESULT_WRONG_HANDLE, Session, SourceChunk, SubmitReport,
};
use core::ffi::c_void;
use core::ptr;
use std::panic::{AssertUnwindSafe, catch_unwind};

use crate::runtime::{CommandError, compile_children, limits_are_valid};

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

unsafe fn borrowed_bytes<'a>(data: *const u8, bytes: u64) -> Result<&'a [u8], u32> {
    if data.is_null() {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    let bytes = usize::try_from(bytes).map_err(|_| RESULT_INVALID_ARGUMENT)?;
    // SAFETY: The caller promises `data` is readable for the declared byte count for this call.
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

/// Returns the frozen Engine V2 C ABI version.
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_v2_abi_version() -> u32 {
    catch_result(|| ABI_VERSION)
}

/// Writes the frozen ABI V1 launch-rate and feature capability masks.
///
/// # Safety
///
/// `out` must satisfy the writable ABI V1 capability-struct contract for this call.
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
/// # Safety
///
/// `config` and `out_engine` must satisfy their readable/writable ABI V1 pointer contracts.
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
        let toml = match unsafe { borrowed_bytes(toml, toml_bytes) } {
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
        let plan = Box::new(Plan::new(children.plan, children.plan_error));
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
        {
            return RESULT_INVALID_ARGUMENT;
        }
        // SAFETY: The caller promises the source-ID storage is readable for this call.
        let source_id = match unsafe { borrowed_bytes(source_id, source_id_bytes) } {
            Ok(value) if !value.is_empty() && core::str::from_utf8(value).is_ok() => value,
            _ => return RESULT_INVALID_ARGUMENT,
        };
        let plane_count = chunk.plane_count as usize;
        // SAFETY: The caller promises an array of `plane_count` readable plane pointers.
        let plane_pointers = unsafe { core::slice::from_raw_parts(chunk.planes, plane_count) };
        let frames = chunk.frames as usize;
        let mut planes: [&[f32]; 255] = [&[]; 255];
        for (index, plane) in plane_pointers.iter().enumerate() {
            if plane.is_null() {
                return RESULT_INVALID_ARGUMENT;
            }
            // SAFETY: Each nonnull caller plane is readable for exactly `frames` samples and is
            // borrowed only until the underlying source submission copies it.
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
            Err(code) => {
                session
                    .last_error
                    .borrow_mut()
                    .set(if code == RESULT_BACKPRESSURE {
                        b"source.backpressure"
                    } else {
                        b"source.submit.rejected"
                    });
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
        let source_id = match unsafe { borrowed_bytes(source_id, source_id_bytes) } {
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
            Err(code) => {
                session
                    .last_error
                    .borrow_mut()
                    .set(if code == RESULT_BACKPRESSURE {
                        b"source.seek.backpressure"
                    } else {
                        b"source.seek.rejected"
                    });
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
        let request = match unsafe { borrowed_bytes(request, request_bytes) } {
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
            Err(CommandError::Internal) => {
                session.last_error.borrow_mut().set(b"control.internal");
                RESULT_INTERNAL
            }
        }
    })
}

/// Render one exact-time quantum directly into caller-owned contiguous planar storage.
///
/// # Safety
///
/// `plan` must be live and exclusive; `output` must satisfy the caller-owned output contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn miso_engine_v2_render_f32_planar(
    plan: *mut Plan,
    absolute_sample: u64,
    output: *const PlanarOutput,
) -> u32 {
    catch_result(|| {
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
        if output.struct_size != crate::PLANAR_OUTPUT_SIZE
            || output.reserved != [0; 2]
            || output.samples.is_null()
        {
            return RESULT_INVALID_ARGUMENT;
        }
        // SAFETY: The live-kind check establishes the concrete render-plan representation and the
        // ABI requires exclusive ownership for this call.
        let plan = unsafe { &mut *plan };
        let frames = plan.state.quantum_frames();
        let required_samples =
            match u64::from(output.plane_stride_samples).checked_add(u64::from(output.frames)) {
                Some(value) => value,
                None => {
                    plan.last_error.borrow_mut().set(b"render.output.overflow");
                    return RESULT_RENDER_REJECTED;
                }
            };
        if output.channels != 2
            || output.frames != frames
            || output.plane_stride_samples < frames
            || required_samples > output.sample_capacity
            || absolute_sample != plan.state.next_absolute_sample()
        {
            plan.last_error
                .borrow_mut()
                .set(b"render.contract.rejected");
            return RESULT_RENDER_REJECTED;
        }
        let required_samples = match usize::try_from(required_samples) {
            Ok(value)
                if value
                    .checked_mul(core::mem::size_of::<f32>())
                    .is_some_and(|bytes| bytes <= isize::MAX as usize) =>
            {
                value
            }
            _ => {
                plan.last_error.borrow_mut().set(b"render.output.platform");
                return RESULT_RENDER_REJECTED;
            }
        };
        // SAFETY: Scalar validation proved the exact contiguous region required by two planes is
        // within caller capacity and Rust's maximum slice extent. The caller promises this many
        // writable aligned `f32` elements exclusively for the duration of the call.
        let samples = unsafe { core::slice::from_raw_parts_mut(output.samples, required_samples) };
        let output = match miso_engine_core::realtime::PlanarBufferMut::try_new(
            samples,
            2,
            frames as usize,
            output.plane_stride_samples as usize,
        ) {
            Ok(value) => value,
            Err(_) => {
                plan.last_error.borrow_mut().set(b"render.output.rejected");
                return RESULT_RENDER_REJECTED;
            }
        };
        match plan.state.render(absolute_sample, output) {
            Ok(()) => {
                plan.last_error.borrow_mut().clear();
                RESULT_OK
            }
            Err(()) => {
                plan.last_error.borrow_mut().set(b"render.plan.rejected");
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
        // SAFETY: `plan` passed the live-kind check and is borrowed immutably for the copy.
        let plan = unsafe { &*plan };
        // SAFETY: Exact struct size establishes writable ABI V1 report storage.
        unsafe { out.write(plan.state.resources) };
        plan.last_error.borrow_mut().clear();
        RESULT_OK
    })
}

/// Copies the handle-local diagnostic. Checkpoint 1 engine handles have an empty diagnostic.
///
/// # Safety
///
/// `live_handle` must identify a live ABI handle and `out` must satisfy the bytes-output contract.
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
                // SAFETY: The recognized header identifies the remaining live plan handle.
                let plan = unsafe { &*live_handle.cast::<Plan>() };
                let bytes = plan.last_error.borrow();
                // SAFETY: `write_bytes` completes before this bounded RefCell borrow is dropped.
                unsafe { write_bytes(out, bytes.as_slice()) }
            }
        }
    })
}

/// Destroys a session handle off render; null is a no-op.
///
/// # Safety
///
/// A nonnull `session` must be the unique live session handle returned by this library.
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
    fn engine_creation_is_transactional_and_validates_v1() {
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
        assert_eq!(&source_error_storage, b"source.seek.rejected");
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
}
