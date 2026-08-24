//! Safe browser-Wasm preparation and ownership shell.
//!
//! This module deliberately contains no raw pointer handling or JavaScript integration. It owns
//! the complete immutable session and render plan that the AudioWorklet boundary drives.
//!
//! # What is here and what is not
//!
//! The compile pipeline is **not** here. Parsing, compiling, source rings, effect and builtin
//! preparation, the graph compile, the identity bindings and the engine-owned resource report are
//! `miso-engine-host-core`, shared with the C ABI host (issue #103). This crate owns exactly what is
//! the browser's: the frozen issue-024 ABI structs and result codes, the fixed staging buffers the
//! JavaScript side reads and writes through raw addresses, the browser bridge's own resource rows,
//! and the render/failure state machine. Before #106 F1 it carried a second, already-diverged copy
//! of the shared pipeline -- 288 lines of `compile_ready` alone, only one of whose two copies
//! rejected source generation `0`.

use core::mem::{MaybeUninit, size_of};

use miso_engine_core::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use miso_engine_host_core::{
    CompiledSession, HostPrepareCaps, HostShapePolicy, PreparedHost, SourceControlError,
    SourceSubmission, control_table_bytes, prepare_host_session, source_id_arena_bytes,
};

/// Frozen browser host ABI version 1.0.
pub const ABI_VERSION: u32 = 0x0001_0000;

/// Successful operation.
pub const RESULT_OK: u32 = 0;
/// Invalid caller input.
pub const RESULT_INVALID_ARGUMENT: u32 = 1;
/// ABI layout/version mismatch.
pub const RESULT_ABI_MISMATCH: u32 = 2;
/// Operation is invalid in the current state.
pub const RESULT_WRONG_STATE: u32 = 3;
/// A supplied buffer is too small.
pub const RESULT_BUFFER_TOO_SMALL: u32 = 4;
/// Preparation or compilation was rejected.
pub const RESULT_PREPARE_REJECTED: u32 = 5;
/// A bounded source queue is full.
pub const RESULT_BACKPRESSURE: u32 = 6;
/// Requested capability is unsupported.
pub const RESULT_UNSUPPORTED: u32 = 7;
/// Rendering failed.
pub const RESULT_RENDER_REJECTED: u32 = 8;
/// The browser configuration changed and requires a fresh host.
pub const RESULT_REPREPARE_REQUIRED: u32 = 9;
/// An internal invariant failed *and was detected by a checked path*.
///
/// A panic does not produce this code on `wasm32`: the target is `panic = abort`, so a panic traps
/// the instance and kills the processor. The worklet converts that trap into this code from
/// JavaScript (see the `.d.ts` header); nothing inside Rust catches it.
pub const RESULT_INTERNAL: u32 = 255;

/// Newly allocated configuration state.
pub const STATE_CONFIG: u32 = 0;
/// Fixed buffers have been prepared.
pub const STATE_PREPARED: u32 = 1;
/// A session and render plan are ready.
pub const STATE_READY: u32 = 2;
/// A sticky preparation or render failure occurred.
pub const STATE_FAILED: u32 = 3;
/// Ownership has been explicitly disposed.
pub const STATE_DISPOSED: u32 = 4;

/// Scalar Wasm backend.
pub const BACKEND_SCALAR: u32 = 0;
/// Base Wasm `simd128` backend.
pub const BACKEND_SIMD128: u32 = 1;

/// Session TOML staging buffer.
pub const BUFFER_SESSION_TOML: u32 = 1;
/// Source-ID staging buffer.
pub const BUFFER_SOURCE_ID: u32 = 2;
/// Planar source PCM staging buffer.
pub const BUFFER_SOURCE_PCM: u32 = 3;
/// Fixed diagnostic buffer.
pub const BUFFER_DIAGNOSTIC: u32 = 4;
/// Contiguous dual-mono output buffer.
pub const BUFFER_OUTPUT_PCM: u32 = 5;

/// Byte size of [`WebPrepareConfigV1`].
pub const PREPARE_CONFIG_BYTES: u32 = size_of::<WebPrepareConfigV1>() as u32;
/// Byte size of [`WebStatusV1`].
pub const STATUS_BYTES: u32 = size_of::<WebStatusV1>() as u32;
/// Byte size of [`WebResourceReportV1`].
pub const RESOURCE_REPORT_BYTES: u32 = size_of::<WebResourceReportV1>() as u32;

/// Exact versioned preparation configuration shared with JavaScript.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WebPrepareConfigV1 {
    /// Exact structure byte size.
    pub struct_size: u32,
    /// Must equal [`ABI_VERSION`].
    pub abi_version: u32,
    /// Explicit browser context sample rate.
    pub sample_rate_hz: u32,
    /// Explicit caller-supplied render quantum.
    pub quantum_frames: u32,
    /// Session TOML staging capacity.
    pub session_toml_bytes: u32,
    /// Diagnostic staging capacity.
    pub diagnostic_bytes: u32,
    /// Source-ID staging capacity.
    pub source_id_bytes: u32,
    /// Maximum staged source plane count.
    pub maximum_source_channels: u32,
    /// Per-source bounded ring capacity in frames.
    pub source_ring_frames: u32,
    /// Maximum effect automation spans per render block.
    pub maximum_automation_spans_per_block: u32,
    /// Maximum tracks.
    pub maximum_tracks: u64,
    /// Maximum sources.
    pub maximum_sources: u64,
    /// Maximum routes.
    pub maximum_routes: u64,
    /// Maximum effects.
    pub maximum_effects: u64,
    /// Maximum graph session-plus-plan bytes.
    pub maximum_graph_session_plus_plan_bytes: u64,
    /// Maximum source-owned total bytes.
    pub maximum_source_total_bytes: u64,
    /// Maximum source overhead bytes.
    pub maximum_source_overhead_bytes: u64,
    /// Maximum scalar effect state bytes.
    pub maximum_effect_state_bytes: u64,
    /// Maximum scalar effect scratch bytes.
    pub maximum_effect_scratch_bytes: u64,
    /// Maximum retained builtin bytes.
    pub maximum_builtin_retained_bytes: u64,
    /// Maximum browser bridge retained bytes.
    pub maximum_host_retained_bytes: u64,
    /// Maximum single named allocation.
    pub maximum_named_allocation_bytes: u64,
    /// Maximum meter streams.
    pub maximum_meter_streams: u64,
    /// Maximum meter items.
    pub maximum_meter_items: u64,
    /// Maximum meter bytes.
    pub maximum_meter_bytes: u64,
    /// Required-zero expansion words.
    pub reserved: [u64; 4],
}

impl WebPrepareConfigV1 {
    /// A bounded launch configuration suitable for small embedding tests and examples.
    #[must_use]
    pub const fn launch_defaults(sample_rate_hz: u32, quantum_frames: u32) -> Self {
        Self {
            struct_size: PREPARE_CONFIG_BYTES,
            abi_version: ABI_VERSION,
            sample_rate_hz,
            quantum_frames,
            session_toml_bytes: 1 << 20,
            diagnostic_bytes: 1 << 14,
            source_id_bytes: 1 << 10,
            maximum_source_channels: 8,
            source_ring_frames: quantum_frames.saturating_mul(4),
            maximum_automation_spans_per_block: 256,
            maximum_tracks: 1_024,
            maximum_sources: 1_024,
            maximum_routes: 4_096,
            maximum_effects: 8_192,
            maximum_graph_session_plus_plan_bytes: 64 << 20,
            maximum_source_total_bytes: 64 << 20,
            maximum_source_overhead_bytes: 16 << 20,
            maximum_effect_state_bytes: 16 << 20,
            maximum_effect_scratch_bytes: 16 << 20,
            maximum_builtin_retained_bytes: 64 << 20,
            maximum_host_retained_bytes: 16 << 20,
            maximum_named_allocation_bytes: 64 << 20,
            maximum_meter_streams: 1_024,
            maximum_meter_items: 1 << 20,
            maximum_meter_bytes: 16 << 20,
            reserved: [0; 4],
        }
    }
}

/// Fixed browser-visible status snapshot.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WebStatusV1 {
    /// Exact structure byte size.
    pub struct_size: u32,
    /// ABI version.
    pub abi_version: u32,
    /// One of the frozen `STATE_*` values.
    pub state: u32,
    /// Most recent frozen result code.
    pub last_result: u32,
    /// Selected backend.
    pub backend: u32,
    /// Prepared sample rate.
    pub sample_rate_hz: u32,
    /// Prepared quantum.
    pub quantum_frames: u32,
    /// Required zero.
    pub reserved0: u32,
    /// Exact next render time.
    pub next_absolute_sample: u64,
    /// Number of successful rendered quanta.
    pub rendered_quanta: u64,
    /// Required-zero expansion words.
    pub reserved: [u64; 4],
}

/// Exact retained-resource projection for the browser bridge and production artifacts.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WebResourceReportV1 {
    /// Exact structure byte size.
    pub struct_size: u32,
    /// ABI version.
    pub abi_version: u32,
    /// Prepared sample rate.
    pub sample_rate_hz: u32,
    /// Prepared render quantum.
    pub quantum_frames: u32,
    /// Selected backend.
    pub backend: u32,
    /// Required zero words.
    pub reserved0: [u32; 3],
    /// Config allocation bytes.
    pub config_bytes: u64,
    /// Status allocation bytes.
    pub status_bytes: u64,
    /// Session TOML staging bytes.
    pub session_toml_bytes: u64,
    /// Diagnostic bytes.
    pub diagnostic_bytes: u64,
    /// Source-ID staging bytes.
    pub source_id_bytes: u64,
    /// Source PCM staging bytes.
    pub source_pcm_staging_bytes: u64,
    /// Output PCM bytes.
    pub output_pcm_bytes: u64,
    /// Browser bridge metadata bytes.
    pub bridge_metadata_bytes: u64,
    /// Total retained browser bridge bytes.
    pub bridge_retained_bytes: u64,
    /// Largest browser bridge allocation.
    pub largest_bridge_allocation_bytes: u64,
    /// Production source-owned total bytes.
    pub source_total_bytes: u64,
    /// Production source overhead bytes.
    pub source_overhead_bytes: u64,
    /// Scalar effect state bytes.
    pub effect_scalar_state_bytes: u64,
    /// Scalar effect scratch bytes.
    pub effect_scalar_scratch_bytes: u64,
    /// Retained builtin bytes.
    pub builtin_retained_bytes: u64,
    /// Graph session-plus-plan bytes.
    pub graph_session_plus_plan_bytes: u64,
    /// Incremental plan bytes.
    pub graph_incremental_plan_bytes: u64,
    /// Graph metadata bytes.
    pub graph_metadata_bytes: u64,
    /// Graph delay bytes.
    pub graph_delay_bytes: u64,
    /// Largest named production or bridge allocation.
    pub largest_named_allocation_bytes: u64,
    /// Required-zero expansion words.
    pub reserved: [u64; 4],
}

struct PreparedBuffers {
    session_toml: Box<[u8]>,
    diagnostic: Box<[u8]>,
    source_id: Box<[u8]>,
    source_pcm: Box<[f32]>,
    output_pcm: Box<[f32]>,
    plane_references: Box<[MaybeUninit<&'static [f32]>]>,
}

type FfiSourceStaging<'a> = (
    &'a mut [f32],
    &'a mut [MaybeUninit<&'static [f32]>],
    &'a mut [u8],
);

/// Everything one compiled session owns on the browser side.
///
/// Field order is the drop order and is load-bearing: [`PreparedHost`] drops its plan (which owns
/// the source consumers) before its control-side producers, and the compiled session model outlives
/// both. Nothing here is ever dropped from `render_next`; see [`AudioWorkletEngineHost::fail`].
struct ReadyOwnership {
    host: PreparedHost,
    /// Retained so the browser bridge keeps charging itself for the compiled model it holds; the
    /// V1 browser ABI has no session query, so nothing reads it.
    _session: CompiledSession,
}

/// Safe ownership object backing one future AudioWorklet Wasm handle.
pub struct AudioWorkletEngineHost {
    config: WebPrepareConfigV1,
    status: WebStatusV1,
    resources: WebResourceReportV1,
    buffers: Option<PreparedBuffers>,
    ready: Option<ReadyOwnership>,
    diagnostic_len: usize,
}

impl AudioWorkletEngineHost {
    /// Create a configuration-only host. No session or staging storage is allocated.
    #[must_use]
    pub const fn new(config: WebPrepareConfigV1) -> Self {
        let backend = selected_backend();
        Self {
            config,
            status: WebStatusV1 {
                struct_size: STATUS_BYTES,
                abi_version: ABI_VERSION,
                state: STATE_CONFIG,
                last_result: RESULT_OK,
                backend,
                sample_rate_hz: 0,
                quantum_frames: 0,
                reserved0: 0,
                next_absolute_sample: 0,
                rendered_quanta: 0,
                reserved: [0; 4],
            },
            resources: empty_resource_report(backend),
            buffers: None,
            ready: None,
            diagnostic_len: 0,
        }
    }

    /// Read the immutable preparation configuration.
    #[must_use]
    pub const fn config(&self) -> &WebPrepareConfigV1 {
        &self.config
    }

    /// Mutably borrow configuration storage before preparation.
    pub fn config_mut(&mut self) -> Option<&mut WebPrepareConfigV1> {
        (self.status.state == STATE_CONFIG).then_some(&mut self.config)
    }

    /// Read fixed status without allocation.
    #[must_use]
    pub const fn status(&self) -> &WebStatusV1 {
        &self.status
    }

    /// Read the current exact resource projection.
    #[must_use]
    pub const fn resources(&self) -> &WebResourceReportV1 {
        &self.resources
    }

    /// Allocate and publish every fixed staging buffer transactionally.
    pub fn prepare(&mut self) -> u32 {
        if self.status.state != STATE_CONFIG {
            return self.record(RESULT_WRONG_STATE);
        }
        let result = prepare_buffers(self.config);
        let (buffers, report) = match result {
            Ok(value) => value,
            Err(code) => return self.record(code),
        };
        self.buffers = Some(buffers);
        self.resources = report;
        self.status.state = STATE_PREPARED;
        self.status.sample_rate_hz = self.config.sample_rate_hz;
        self.status.quantum_frames = self.config.quantum_frames;
        self.record(RESULT_OK)
    }

    /// Mutable session TOML staging storage, available only after preparation.
    pub fn session_toml_mut(&mut self) -> Option<&mut [u8]> {
        self.buffers.as_mut().map(|value| &mut *value.session_toml)
    }

    /// Mutable source-ID staging storage, available only after preparation.
    pub fn source_id_mut(&mut self) -> Option<&mut [u8]> {
        self.buffers.as_mut().map(|value| &mut *value.source_id)
    }

    /// Mutable contiguous planar source PCM staging storage.
    pub fn source_pcm_mut(&mut self) -> Option<&mut [f32]> {
        self.buffers.as_mut().map(|value| &mut *value.source_pcm)
    }

    /// The valid fixed diagnostic prefix.
    #[must_use]
    pub fn diagnostic(&self) -> &[u8] {
        self.buffers
            .as_ref()
            .map_or(&[], |value| &value.diagnostic[..self.diagnostic_len])
    }

    /// Contiguous dual-mono render output, left plane then right plane.
    #[must_use]
    pub fn output_pcm(&self) -> Option<&[f32]> {
        self.buffers.as_ref().map(|value| &*value.output_pcm)
    }

    pub(crate) fn diagnostic_buffer_mut(&mut self) -> Option<&mut [u8]> {
        self.buffers.as_mut().map(|value| &mut *value.diagnostic)
    }

    pub(crate) fn ffi_source_staging_mut(&mut self) -> Option<FfiSourceStaging<'_>> {
        self.buffers.as_mut().map(|value| {
            (
                &mut *value.source_pcm,
                &mut *value.plane_references,
                &mut *value.source_id,
            )
        })
    }

    /// Parse, compile and atomically publish one immutable session and plan.
    pub fn compile(&mut self, toml_bytes: usize) -> u32 {
        if self.status.state != STATE_PREPARED {
            return self.record(RESULT_WRONG_STATE);
        }
        let Some(buffers) = self.buffers.as_ref() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.buffers\t$\n");
        };
        let Some(bytes) = buffers.session_toml.get(..toml_bytes) else {
            return self.record(RESULT_BUFFER_TOO_SMALL);
        };
        let toml = match core::str::from_utf8(bytes) {
            Ok(value) => value,
            Err(_) => return self.fail(RESULT_INVALID_ARGUMENT, b"web.toml.utf8\t$\n"),
        };
        match compile_ready(toml, self.config, self.resources) {
            Ok((ready, resources)) => {
                self.ready = Some(ready);
                self.resources = resources;
                self.diagnostic_len = 0;
                self.status.state = STATE_READY;
                self.record(RESULT_OK)
            }
            Err(failure) => self.fail(RESULT_PREPARE_REJECTED, &failure),
        }
    }

    /// Submit one generation-tagged, exact-rate borrowed planar source chunk.
    ///
    /// The host owns two rules: the `STATE_READY` gate, and the staging-shaped bound
    /// `frames <= quantum_frames` (the JavaScript side copies into a one-quantum staging plane, so
    /// a longer chunk could not have been staged). Everything else -- rate, channel count, region
    /// bounds, end-of-region symmetry, generation `0`, ring backpressure -- is the facade's, in the
    /// facade's fixed order, so the browser host and the C ABI host can no longer disagree about
    /// what a rejection is. Generation `0` is now rejected here too; before #106 F1 only the C ABI
    /// copy caught it.
    #[allow(clippy::too_many_arguments)]
    pub fn submit_source(
        &mut self,
        source_id: &[u8],
        generation: u64,
        start_frame: u64,
        sample_rate_hz: u32,
        planes: &[&[f32]],
        frames: u32,
        end_of_region: bool,
    ) -> u32 {
        if self.status.state != STATE_READY {
            return self.record(RESULT_WRONG_STATE);
        }
        if frames > self.config.quantum_frames {
            return self.record(RESULT_INVALID_ARGUMENT);
        }
        let Some(ready) = self.ready.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        let result = ready.host.sources.submit(
            source_id,
            SourceSubmission {
                generation,
                start_frame,
                sample_rate_hz,
                planes,
                frames,
                end_of_region,
            },
        );
        let code = match result {
            Ok(_) => RESULT_OK,
            Err(error) => source_result(error),
        };
        self.record(code)
    }

    /// Queue one strictly increasing generation-tagged absolute source seek.
    pub fn seek_source(&mut self, source_id: &[u8], generation: u64, source_frame: u64) -> u32 {
        if self.status.state != STATE_READY {
            return self.record(RESULT_WRONG_STATE);
        }
        let Some(ready) = self.ready.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        let code = match ready.host.sources.seek(source_id, generation, source_frame) {
            Ok(()) => RESULT_OK,
            Err(error) => source_result(error),
        };
        self.record(code)
    }

    /// Render exactly one prepared quantum at the next absolute sample time.
    pub fn render_next(&mut self) -> u32 {
        if self.status.state != STATE_READY {
            return self.record(RESULT_WRONG_STATE);
        }
        // Lossless on every supported target (wasm32 and every 64-bit host); `try_from` would
        // leave an `expect_failed` trap owner in the render export's call graph.
        let quantum = self.config.quantum_frames as usize;
        let Some(ready) = self.ready.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        let Some(buffers) = self.buffers.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.buffers\t$\n");
        };
        let output = match PlanarBufferMut::try_new(&mut buffers.output_pcm, 2, quantum, quantum) {
            Ok(value) => value,
            Err(_) => return self.fail(RESULT_INTERNAL, b"web.internal.output\t$\n"),
        };
        let result = ready.host.plan.render(
            RenderIo {
                input: None,
                output,
            },
            RenderTime {
                absolute_sample: self.status.next_absolute_sample,
            },
        );
        match result {
            Ok(report) => {
                self.status.next_absolute_sample = report.next_absolute_sample;
                self.status.rendered_quanta = self.status.rendered_quanta.saturating_add(1);
                self.record(RESULT_OK)
            }
            Err(_) => self.fail(RESULT_RENDER_REJECTED, b"web.render.rejected\t$\n"),
        }
    }

    pub(crate) fn record_boundary_result(&mut self, code: u32) -> u32 {
        self.record(code)
    }

    /// Mark an observed browser output-shape mismatch as sticky reprepare-required.
    pub fn reject_output_quantum(&mut self, actual_frames: u32) -> u32 {
        if self.status.state != STATE_READY {
            return self.record(RESULT_WRONG_STATE);
        }
        if actual_frames == self.config.quantum_frames {
            return self.record(RESULT_OK);
        }
        self.status.state = STATE_FAILED;
        self.record(RESULT_REPREPARE_REQUIRED)
    }

    /// Release all prepared ownership. Repeated disposal is harmless.
    ///
    /// This runs in the worklet's `port.onmessage` handler, never inside `process()`; it is the
    /// single point where the plan, the compiled session and the source rings are freed.
    pub fn dispose(&mut self) -> u32 {
        self.ready = None;
        self.buffers = None;
        self.diagnostic_len = 0;
        self.status.state = STATE_DISPOSED;
        self.record(RESULT_OK)
    }

    fn record(&mut self, code: u32) -> u32 {
        self.status.last_result = code;
        code
    }

    /// Record a sticky failure without ever releasing ownership.
    ///
    /// `self.ready` **is** this host's one-slot retirement queue (AGENTS.md: "the displaced plan
    /// goes to a bounded retirement queue and is reclaimed off the render thread"). `fail` is
    /// reachable from `render_next`, which the AudioWorklet calls on the rendering thread, so it
    /// must not move, overwrite or drop the plan, the compiled session or the source rings:
    /// dropping them would free `Arc<spsc::Ring<_>>` payloads and run `dlmalloc::free` inside
    /// `process()`. The ownership stays exactly where it is and only [`Self::dispose`] — a control
    /// path call, delivered on `port.onmessage` — reclaims it.
    ///
    /// A separate `retired` field would not help: `self.retired = self.ready.take()` compiles to a
    /// conditional `drop_glue::<Option<ReadyOwnership>>` call on the overwritten value, which the
    /// call-graph gate sees even though it never executes.
    ///
    /// The output block is filled with positive zero so a failed render emits silence rather than
    /// the previous quantum.
    fn fail(&mut self, code: u32, diagnostic: &[u8]) -> u32 {
        self.status.state = STATE_FAILED;
        if let Some(buffers) = self.buffers.as_mut() {
            buffers.output_pcm.fill(0.0);
            self.diagnostic_len = diagnostic.len().min(buffers.diagnostic.len());
            buffers.diagnostic[..self.diagnostic_len]
                .copy_from_slice(&diagnostic[..self.diagnostic_len]);
        }
        self.record(code)
    }
}

const fn selected_backend() -> u32 {
    if cfg!(all(target_arch = "wasm32", target_feature = "simd128")) {
        BACKEND_SIMD128
    } else {
        BACKEND_SCALAR
    }
}

const fn empty_resource_report(backend: u32) -> WebResourceReportV1 {
    WebResourceReportV1 {
        struct_size: RESOURCE_REPORT_BYTES,
        abi_version: ABI_VERSION,
        sample_rate_hz: 0,
        quantum_frames: 0,
        backend,
        reserved0: [0; 3],
        config_bytes: size_of::<WebPrepareConfigV1>() as u64,
        status_bytes: size_of::<WebStatusV1>() as u64,
        session_toml_bytes: 0,
        diagnostic_bytes: 0,
        source_id_bytes: 0,
        source_pcm_staging_bytes: 0,
        output_pcm_bytes: 0,
        bridge_metadata_bytes: 0,
        bridge_retained_bytes: 0,
        largest_bridge_allocation_bytes: 0,
        source_total_bytes: 0,
        source_overhead_bytes: 0,
        effect_scalar_state_bytes: 0,
        effect_scalar_scratch_bytes: 0,
        builtin_retained_bytes: 0,
        graph_session_plus_plan_bytes: 0,
        graph_incremental_plan_bytes: 0,
        graph_metadata_bytes: 0,
        graph_delay_bytes: 0,
        largest_named_allocation_bytes: 0,
        reserved: [0; 4],
    }
}

fn prepare_buffers(
    config: WebPrepareConfigV1,
) -> Result<(PreparedBuffers, WebResourceReportV1), u32> {
    validate_config(config)?;
    let source_samples = checked_product([
        u64::from(config.maximum_source_channels),
        u64::from(config.quantum_frames),
    ])?;
    let source_pcm_bytes = source_samples
        .checked_mul(4)
        .ok_or(RESULT_PREPARE_REJECTED)?;
    let output_pcm_bytes = u64::from(config.quantum_frames)
        .checked_mul(8)
        .ok_or(RESULT_PREPARE_REJECTED)?;
    let plane_reference_bytes = u64::from(config.maximum_source_channels)
        .checked_mul(size_of::<&[f32]>() as u64)
        .ok_or(RESULT_PREPARE_REJECTED)?;
    let host_shell_bytes =
        u64::try_from(size_of::<AudioWorkletEngineHost>()).map_err(|_| RESULT_PREPARE_REJECTED)?;
    let fixed_metadata = host_shell_bytes
        .checked_sub(u64::from(PREPARE_CONFIG_BYTES) + u64::from(STATUS_BYTES))
        .ok_or(RESULT_INTERNAL)?;
    let bridge_metadata = fixed_metadata
        .checked_add(plane_reference_bytes)
        .ok_or(RESULT_PREPARE_REJECTED)?;
    let rows = [
        u64::from(PREPARE_CONFIG_BYTES),
        u64::from(STATUS_BYTES),
        u64::from(config.session_toml_bytes),
        u64::from(config.diagnostic_bytes),
        u64::from(config.source_id_bytes),
        source_pcm_bytes,
        output_pcm_bytes,
        bridge_metadata,
    ];
    let retained = checked_sum(rows)?;
    let largest = rows
        .into_iter()
        .max()
        .unwrap_or(0)
        .max(host_shell_bytes)
        .max(plane_reference_bytes);
    if retained > config.maximum_host_retained_bytes
        || largest > config.maximum_named_allocation_bytes
    {
        return Err(RESULT_PREPARE_REJECTED);
    }
    let buffers = PreparedBuffers {
        session_toml: boxed_zero_u8(config.session_toml_bytes)?,
        diagnostic: boxed_zero_u8(config.diagnostic_bytes)?,
        source_id: boxed_zero_u8(config.source_id_bytes)?,
        source_pcm: boxed_zero_f32(source_samples)?,
        output_pcm: boxed_zero_f32(u64::from(config.quantum_frames) * 2)?,
        plane_references: boxed_uninit_planes(config.maximum_source_channels)?,
    };
    let mut report = empty_resource_report(selected_backend());
    report.sample_rate_hz = config.sample_rate_hz;
    report.quantum_frames = config.quantum_frames;
    report.session_toml_bytes = u64::from(config.session_toml_bytes);
    report.diagnostic_bytes = u64::from(config.diagnostic_bytes);
    report.source_id_bytes = u64::from(config.source_id_bytes);
    report.source_pcm_staging_bytes = source_pcm_bytes;
    report.output_pcm_bytes = output_pcm_bytes;
    report.bridge_metadata_bytes = bridge_metadata;
    report.bridge_retained_bytes = retained;
    report.largest_bridge_allocation_bytes = largest;
    report.largest_named_allocation_bytes = largest;
    Ok((buffers, report))
}

fn validate_config(config: WebPrepareConfigV1) -> Result<(), u32> {
    if config.struct_size != PREPARE_CONFIG_BYTES || config.abi_version != ABI_VERSION {
        return Err(RESULT_ABI_MISMATCH);
    }
    if config.reserved != [0; 4]
        || !matches!(config.sample_rate_hz, 44_100 | 48_000 | 88_200 | 96_000)
        || config.quantum_frames == 0
        || config.session_toml_bytes == 0
        || config.diagnostic_bytes == 0
        || config.source_id_bytes == 0
        || config.maximum_source_channels == 0
        || config.source_ring_frames < config.quantum_frames
        || !config
            .source_ring_frames
            .is_multiple_of(config.quantum_frames)
        || config.maximum_automation_spans_per_block == 0
    {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    let limits = [
        config.maximum_tracks,
        config.maximum_sources,
        config.maximum_routes,
        config.maximum_effects,
        config.maximum_graph_session_plus_plan_bytes,
        config.maximum_source_total_bytes,
        config.maximum_source_overhead_bytes,
        config.maximum_effect_state_bytes,
        config.maximum_effect_scratch_bytes,
        config.maximum_builtin_retained_bytes,
        config.maximum_host_retained_bytes,
        config.maximum_named_allocation_bytes,
        config.maximum_meter_streams,
        config.maximum_meter_items,
        config.maximum_meter_bytes,
    ];
    if limits
        .into_iter()
        .any(|value| value == 0 || value > u64::from(u32::MAX))
    {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    let source_bytes = checked_product([
        u64::from(config.maximum_source_channels),
        u64::from(config.quantum_frames),
        4,
    ])?;
    let output_bytes = u64::from(config.quantum_frames)
        .checked_mul(8)
        .ok_or(RESULT_PREPARE_REJECTED)?;
    if source_bytes > i32::MAX as u64 || output_bytes > i32::MAX as u64 {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    Ok(())
}

/// Map the facade's one typed source rejection onto the frozen browser result codes.
///
/// The facade owns the vocabulary and the diagnostic strings; the browser ABI owns the numbers.
/// Backpressure is bounded and retryable, an internal invariant failure is not the caller's fault,
/// and everything else is a malformed submission.
fn source_result(error: SourceControlError) -> u32 {
    if error.is_backpressure() {
        RESULT_BACKPRESSURE
    } else if error.is_internal() {
        RESULT_INTERNAL
    } else {
        RESULT_INVALID_ARGUMENT
    }
}

/// Translate the frozen browser configuration into the facade's caps, field for field.
///
/// This is the only place the mapping is spelled. `HostShapePolicy::Exact`: unlike the C ABI host,
/// the browser host is handed its `AudioContext` rate and the caller's quantum and must refuse any
/// session that declares anything else -- there is no resampler and no requantiser.
const fn prepare_caps(config: WebPrepareConfigV1) -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::Exact {
            sample_rate_hz: config.sample_rate_hz,
            quantum_frames: config.quantum_frames,
        },
        source_ring_frames: config.source_ring_frames,
        maximum_source_channels: Some(config.maximum_source_channels),
        maximum_automation_spans_per_block: config.maximum_automation_spans_per_block,
        maximum_tracks: config.maximum_tracks,
        maximum_sources: config.maximum_sources,
        maximum_routes: config.maximum_routes,
        maximum_effects: config.maximum_effects,
        maximum_graph_session_plus_plan_bytes: config.maximum_graph_session_plus_plan_bytes,
        maximum_source_total_bytes: config.maximum_source_total_bytes,
        maximum_source_overhead_bytes: config.maximum_source_overhead_bytes,
        maximum_effect_state_bytes: config.maximum_effect_state_bytes,
        maximum_effect_scratch_bytes: config.maximum_effect_scratch_bytes,
        maximum_builtin_retained_bytes: config.maximum_builtin_retained_bytes,
        maximum_named_allocation_bytes: config.maximum_named_allocation_bytes,
        maximum_meter_streams: config.maximum_meter_streams,
        maximum_meter_items: config.maximum_meter_items,
        maximum_meter_bytes: config.maximum_meter_bytes,
    }
}

/// Run the shared pipeline, then fold its engine-owned report into the frozen browser report.
///
/// The split is the one the facade documents: engine-owned rows come from
/// `miso_engine_host_core::HostPrepareReport`, the browser bridge's own rows (staging buffers, the
/// host shell, the plane-reference table) were already computed by `prepare_buffers`, and the three
/// browser caps are applied here because they are the *browser's* caps on the shared report.
fn compile_ready(
    toml: &str,
    config: WebPrepareConfigV1,
    mut report: WebResourceReportV1,
) -> Result<(ReadyOwnership, WebResourceReportV1), Vec<u8>> {
    let caps = prepare_caps(config);
    let (session, host) = prepare_host_session(toml, &caps).map_err(|value| value.into_bytes())?;
    let engine = host.report;

    // Browser-only: every source ID must fit the fixed staging buffer JavaScript writes it into.
    // The facade has no such buffer, so this rule stays the host's.
    if host.sources.longest_id_bytes() > config.source_id_bytes as usize {
        return Err(fixed_diagnostic("web.source.id.capacity"));
    }

    let control_table = control_table_bytes(engine.source_count as usize)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let id_arena = source_id_arena_bytes(engine.source_id_bytes as usize)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    // Charged because the browser bridge retains them: the facade's control table and ID arena
    // (`control_retained_bytes` is exactly those two) and the compiled session model.
    let ready_metadata =
        checked_sum_prepare([engine.control_retained_bytes, engine.session_model_bytes])?;
    let bridge_metadata = report
        .bridge_metadata_bytes
        .checked_add(ready_metadata)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let bridge_retained = report
        .bridge_retained_bytes
        .checked_add(ready_metadata)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let bridge_largest = report
        .largest_bridge_allocation_bytes
        .max(control_table)
        .max(id_arena)
        .max(engine.session_largest_allocation_bytes);
    let largest_named = bridge_largest.max(engine.largest_engine_allocation_bytes);
    if bridge_retained > config.maximum_host_retained_bytes
        || largest_named > config.maximum_named_allocation_bytes
    {
        return Err(fixed_diagnostic("web.resource.limit"));
    }

    report.bridge_metadata_bytes = bridge_metadata;
    report.bridge_retained_bytes = bridge_retained;
    report.largest_bridge_allocation_bytes = bridge_largest;
    report.source_total_bytes = engine.source_total_bytes;
    report.source_overhead_bytes = engine.source_overhead_bytes;
    report.effect_scalar_state_bytes = engine.effect_scalar_state_bytes;
    report.effect_scalar_scratch_bytes = engine.effect_scalar_scratch_bytes;
    report.builtin_retained_bytes = engine.builtin_retained_payload_bytes;
    report.graph_session_plus_plan_bytes = engine.graph_session_plus_plan_bytes;
    report.graph_incremental_plan_bytes = engine.graph_incremental_plan_bytes;
    report.graph_metadata_bytes = engine.graph_metadata_bytes;
    report.graph_delay_bytes = engine.graph_delay_bytes;
    report.largest_named_allocation_bytes = largest_named;
    Ok((
        ReadyOwnership {
            host,
            _session: session,
        },
        report,
    ))
}

/// One `code\t$\n` diagnostic line for a rule that names no session path.
fn fixed_diagnostic(code: &str) -> Vec<u8> {
    miso_engine_host_core::fixed_diagnostic_line(code)
}

fn checked_sum(values: impl IntoIterator<Item = u64>) -> Result<u64, u32> {
    values.into_iter().try_fold(0_u64, |total, value| {
        total.checked_add(value).ok_or(RESULT_PREPARE_REJECTED)
    })
}

fn checked_sum_prepare(values: impl IntoIterator<Item = u64>) -> Result<u64, Vec<u8>> {
    values.into_iter().try_fold(0_u64, |total, value| {
        total
            .checked_add(value)
            .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))
    })
}

fn checked_product<const N: usize>(values: [u64; N]) -> Result<u64, u32> {
    values.into_iter().try_fold(1_u64, |total, value| {
        total.checked_mul(value).ok_or(RESULT_PREPARE_REJECTED)
    })
}

fn boxed_zero_u8(bytes: u32) -> Result<Box<[u8]>, u32> {
    let count = usize::try_from(bytes).map_err(|_| RESULT_PREPARE_REJECTED)?;
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| RESULT_PREPARE_REJECTED)?;
    value.resize(count, 0);
    Ok(value.into_boxed_slice())
}

fn boxed_zero_f32(samples: u64) -> Result<Box<[f32]>, u32> {
    let count = usize::try_from(samples).map_err(|_| RESULT_PREPARE_REJECTED)?;
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| RESULT_PREPARE_REJECTED)?;
    value.resize(count, 0.0);
    Ok(value.into_boxed_slice())
}

fn boxed_uninit_planes(channels: u32) -> Result<Box<[MaybeUninit<&'static [f32]>]>, u32> {
    let count = usize::try_from(channels).map_err(|_| RESULT_PREPARE_REJECTED)?;
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| RESULT_PREPARE_REJECTED)?;
    value.resize_with(count, MaybeUninit::uninit);
    Ok(value.into_boxed_slice())
}

mod ffi;

pub use ffi::*;

#[cfg(test)]
mod tests;
