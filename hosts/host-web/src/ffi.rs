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
    RESPONSE_MAXIMUM_EFFECT_ID_BYTES, RESPONSE_MAXIMUM_PARAMETER_OVERRIDES,
    RESPONSE_MAXIMUM_RESULT_BYTES, RESPONSE_PARAMETER_BYTES, RESPONSE_REQUEST_BYTES,
    RESULT_INTERNAL, RESULT_INVALID_ARGUMENT,
    RESULT_OK, RESULT_REFUSED_BUDGET, RESULT_REFUSED_DOCUMENT, RESULT_REFUSED_LIFECYCLE,
    RESULT_UNSUPPORTED, STATE_READY, WebBootOptions, WebResponseParameter, WebResponseRequest,
    WebResponseResult, RESPONSE_RESULT_BYTES,
};
use core::{
    cell::{Cell, RefCell},
    mem::{MaybeUninit, size_of},
    ptr, slice,
};
use effect_contract::{EffectQuality, LinkMode, ParameterChannel};
use host_core::{
    ResponseParameterOverride, ResponsePreviewError, ResponsePreviewGrid, ResponsePreviewLimits,
    ResponsePreviewOutput, ResponsePreviewRequest, ResponsePreviewTarget,
    prepare_response_preview,
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
            parameters: vec![WebResponseParameter::default(); RESPONSE_MAXIMUM_PARAMETER_OVERRIDES as usize]
                .into_boxed_slice(),
            result: Vec::new(),
            result_header: WebResponseResult {
                struct_size: RESPONSE_RESULT_BYTES,
                abi_version: ABI_VERSION,
                result: RESULT_OK,
                ..WebResponseResult::default()
            },
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
    let end = result.len().checked_add(bytes).ok_or(RESULT_REFUSED_BUDGET)?;
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
    let section_points = match section_count.checked_mul(points) {
        Some(value) => value,
        None => return response_failure(staging, RESULT_REFUSED_BUDGET),
    };
    let want_left = request.channels & crate::RESPONSE_CHANNEL_LEFT != 0;
    let want_right = request.channels & crate::RESPONSE_CHANNEL_RIGHT != 0;
    let want_sections = request.fields & crate::RESPONSE_FIELD_SECTIONS != 0;
    let mut frequencies = vec![0.0; points];
    let mut total_left = vec![0.0; points];
    let mut total_right = vec![0.0; points];
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
    let maximum = request.maximum_result_bytes as usize;
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
        match append_f32_values(&mut payload, sections_left.as_deref().unwrap_or(&[]), maximum) {
            Ok(value) => value,
            Err(result) => return response_failure(staging, result),
        }
    } else {
        0
    };
    let sections_right_offset = if want_right && want_sections {
        match append_f32_values(&mut payload, sections_right.as_deref().unwrap_or(&[]), maximum) {
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
