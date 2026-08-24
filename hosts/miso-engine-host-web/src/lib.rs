//! Safe browser-Wasm preparation and ownership shell.
//!
//! This module deliberately contains no raw pointer handling or JavaScript integration. It owns
//! the complete immutable session and render plan that the later AudioWorklet boundary will drive.

use core::{
    alloc::Layout,
    mem::{MaybeUninit, size_of},
};
use miso_engine_core::target_capabilities;
use miso_engine_graph_compiler::KernelDispatch;

use miso_engine_builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use miso_engine_core::{
    SampleRateHz,
    realtime::{PlanarBufferMut, PreparedRenderPlan, RenderIo, RenderTime},
};
use miso_engine_effect_compiler::{
    EffectCompileCaps, launch_native_effect_registry_v1, prepare_native_session_effects,
};
use miso_engine_graph::{
    GraphBindingBlock, GraphCompileCaps, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings,
    GraphRuntimeProcessor, StableGraphId, TrackStage,
};
use miso_engine_graph_compiler::{GraphBuiltinsCompileRequest, GraphCompiler};
use miso_engine_session::{
    CompileCaps, CompiledSession, DiagnosticSet, compile_session, parse_session_toml,
};
use miso_engine_source::{
    HostChunkError, HostChunkProvider, HostPlanarChunk, PcmSourceRing, PcmSourceRingConfig,
    SourceCommand, SourceFrame, SourceGeneration, SourceGraphSource, SourceGraphTrackMapping,
    SourceSeekError, prepare_graph_source_set,
};
use miso_engine_target_smoke::TargetSmoke;

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

struct ControlSource {
    id_offset: usize,
    id_bytes: usize,
    sample_rate_hz: u32,
    channel_count: u32,
    region_start: u64,
    region_end: u64,
    provider: HostChunkProvider,
}

// Field order is intentional: the plan and its source consumers drop before control producers.
struct ReadyOwnership {
    plan: PreparedRenderPlan,
    _session: CompiledSession,
    source_ids: Box<[u8]>,
    sources: Box<[ControlSource]>,
}

struct IdentityProcessor;

impl GraphRuntimeProcessor for IdentityProcessor {
    fn process(
        &mut self,
        _block: GraphBindingBlock<'_>,
    ) -> Result<(), miso_engine_core::realtime::RenderError> {
        Ok(())
    }
}

#[derive(Debug)]
struct PrepareFailure {
    code: u32,
    diagnostic: Vec<u8>,
}

impl PrepareFailure {
    fn fixed(code: u32, diagnostic: &str) -> Self {
        Self {
            code,
            diagnostic: diagnostic.as_bytes().to_vec(),
        }
    }
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
            Err(failure) => self.fail(failure.code, &failure.diagnostic),
        }
    }

    /// Submit one generation-tagged, exact-rate borrowed planar source chunk.
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
        let Some(source) = find_source_mut(ready, source_id) else {
            return self.record(RESULT_INVALID_ARGUMENT);
        };
        if sample_rate_hz != source.sample_rate_hz
            || u32::try_from(planes.len()).ok() != Some(source.channel_count)
            || start_frame < source.region_start
            || start_frame > source.region_end
            || start_frame.saturating_add(u64::from(frames)) > source.region_end
            || end_of_region != (start_frame + u64::from(frames) == source.region_end)
        {
            return self.record(RESULT_INVALID_ARGUMENT);
        }
        let result = source.provider.submit(HostPlanarChunk {
            sample_rate_hz: SampleRateHz(sample_rate_hz),
            generation: SourceGeneration(generation),
            start_frame: SourceFrame(start_frame),
            planes,
            frames,
            end_of_region,
        });
        let code = match result {
            Ok(_) => RESULT_OK,
            Err(HostChunkError::Full { .. }) => RESULT_BACKPRESSURE,
            Err(HostChunkError::InternalInvariant) => RESULT_INTERNAL,
            Err(_) => RESULT_INVALID_ARGUMENT,
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
        let Some(source) = find_source_mut(ready, source_id) else {
            return self.record(RESULT_INVALID_ARGUMENT);
        };
        if source_frame < source.region_start || source_frame > source.region_end {
            return self.record(RESULT_INVALID_ARGUMENT);
        }
        let code = match source.provider.try_seek(SourceCommand::Seek {
            generation: SourceGeneration(generation),
            frame: SourceFrame(source_frame),
        }) {
            Ok(()) => RESULT_OK,
            Err(SourceSeekError::Backpressure { .. }) => RESULT_BACKPRESSURE,
            Err(_) => RESULT_INVALID_ARGUMENT,
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
        let result = ready.plan.render(
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

fn compile_ready(
    toml: &str,
    config: WebPrepareConfigV1,
    mut report: WebResourceReportV1,
) -> Result<(ReadyOwnership, WebResourceReportV1), PrepareFailure> {
    let model = parse_session_toml(toml).map_err(|value| session_diagnostics(&value))?;
    let counts = [
        u64::try_from(model.tracks.len()).map_err(|_| fixed("web.resource.count"))?,
        u64::try_from(model.sources.len()).map_err(|_| fixed("web.resource.count"))?,
        u64::try_from(model.routes.len()).map_err(|_| fixed("web.resource.count"))?,
        count_effects(&model)?,
    ];
    if counts[0] > config.maximum_tracks
        || counts[1] > config.maximum_sources
        || counts[2] > config.maximum_routes
        || counts[3] > config.maximum_effects
    {
        return Err(fixed("web.resource.count"));
    }
    let aggregate_ring_frames = counts[1]
        .checked_mul(u64::from(config.source_ring_frames))
        .ok_or_else(|| fixed("web.resource.arithmetic"))?;
    let compiled = compile_session(
        &model,
        CompileCaps {
            max_compiled_model_bytes: config.maximum_graph_session_plus_plan_bytes,
            max_requested_runtime_bytes: config.maximum_graph_session_plus_plan_bytes,
            max_single_allocation_bytes: config.maximum_named_allocation_bytes,
            max_queue_items: u64::MAX,
            max_source_ring_frames: aggregate_ring_frames,
            max_source_ring_bytes: config.maximum_source_total_bytes,
        },
    )
    .map_err(|value| session_diagnostics(&value))?;
    if compiled.sample_rate().0 != config.sample_rate_hz
        || compiled.quantum().0 != config.quantum_frames
    {
        return Err(fixed("web.session.shape"));
    }

    let source_id_bytes =
        compiled
            .normalized_model()
            .sources
            .iter()
            .try_fold(0_usize, |total, source| {
                total
                    .checked_add(source.id.as_str().len())
                    .ok_or_else(|| fixed("web.resource.arithmetic"))
            })?;
    if compiled
        .normalized_model()
        .sources
        .iter()
        .any(|source| source.id.as_str().len() > config.source_id_bytes as usize)
    {
        return Err(fixed("web.source.id.capacity"));
    }
    let mut ids = Vec::new();
    ids.try_reserve_exact(source_id_bytes)
        .map_err(|_| fixed("web.resource.allocation"))?;
    let mut controls = Vec::new();
    controls
        .try_reserve_exact(compiled.source_count())
        .map_err(|_| fixed("web.resource.allocation"))?;
    let mut graph_sources = Vec::new();
    graph_sources
        .try_reserve_exact(compiled.source_count())
        .map_err(|_| fixed("web.resource.allocation"))?;
    for source in &compiled.normalized_model().sources {
        if source.sample_rate_hz != compiled.sample_rate().0
            || u32::from(source.mapping.channel_count) > config.maximum_source_channels
        {
            return Err(fixed("web.source.shape"));
        }
        let region_end = source
            .mapping
            .region
            .start_sample
            .checked_add(source.mapping.region.length_samples)
            .ok_or_else(|| fixed("web.source.region"))?;
        let (producer, consumer, resources) = PcmSourceRing::prepare_host_region(
            PcmSourceRingConfig {
                channel_count: u32::from(source.mapping.channel_count),
                quantum_frames: compiled.quantum(),
                frame_capacity: u64::from(config.source_ring_frames),
                initial_generation: SourceGeneration(1),
            },
            SourceFrame(source.mapping.region.start_sample),
        )
        .map_err(|_| fixed("web.source.prepare"))?;
        let id_offset = ids.len();
        ids.extend_from_slice(source.id.as_str().as_bytes());
        controls.push(ControlSource {
            id_offset,
            id_bytes: source.id.as_str().len(),
            sample_rate_hz: source.sample_rate_hz,
            channel_count: u32::from(source.mapping.channel_count),
            region_start: source.mapping.region.start_sample,
            region_end,
            provider: producer.into_host_chunk_provider(SampleRateHz(source.sample_rate_hz)),
        });
        graph_sources.push(SourceGraphSource::new(consumer, resources, 0, 0));
    }
    controls.sort_unstable_by(|left, right| {
        ids[left.id_offset..left.id_offset + left.id_bytes]
            .cmp(&ids[right.id_offset..right.id_offset + right.id_bytes])
    });
    let mappings = compiled
        .normalized_model()
        .tracks
        .iter()
        .map(|track| {
            let source_index = compiled
                .source_index(&track.source_id)
                .and_then(|value| usize::try_from(value).ok())
                .ok_or_else(|| fixed("web.source.mapping"))?;
            Ok(SourceGraphTrackMapping {
                node: GraphNodeId::TrackStage {
                    track_id: StableGraphId::parse(track.id.as_str())
                        .ok_or_else(|| fixed("web.source.mapping"))?,
                    stage: TrackStage::Input,
                },
                source_index,
                left_channel: u32::from(track.left_source_channel),
                right_channel: u32::from(track.right_source_channel),
            })
        })
        .collect::<Result<Vec<_>, PrepareFailure>>()?;

    let registry = launch_native_effect_registry_v1().map_err(|_| fixed("web.effect.registry"))?;
    let effects = prepare_native_session_effects(
        &compiled,
        &registry,
        EffectCompileCaps {
            maximum_total_state_bytes: config.maximum_effect_state_bytes,
            maximum_scratch_bytes: config.maximum_effect_scratch_bytes,
            maximum_automation_spans_per_block: config.maximum_automation_spans_per_block,
        },
    )
    .map_err(|value| effect_diagnostics(&value.0))?;
    let (effect_state, effect_scratch) =
        effects
            .entries
            .iter()
            .try_fold((0_u64, 0_u64), |(state, scratch), entry| {
                Ok::<_, PrepareFailure>((
                    state
                        .checked_add(
                            entry
                                .metadata
                                .state_sizes
                                .total()
                                .ok_or_else(|| fixed("web.effect.resource"))?,
                        )
                        .ok_or_else(|| fixed("web.effect.resource"))?,
                    scratch
                        .checked_add(entry.metadata.scratch_bytes)
                        .ok_or_else(|| fixed("web.effect.resource"))?,
                ))
            })?;
    let builtins = prepare_session_builtins(
        &compiled,
        &[],
        BuiltinCompileCaps {
            maximum_total_state_bytes: config.maximum_builtin_retained_bytes,
            maximum_total_retained_payload_bytes: config.maximum_builtin_retained_bytes,
            maximum_total_meter_items: config.maximum_meter_items,
            maximum_total_meter_bytes: config.maximum_meter_bytes,
            maximum_single_allocation_bytes: config.maximum_named_allocation_bytes,
            maximum_meter_streams: config.maximum_meter_streams,
            maximum_period_frames: u32::MAX,
            maximum_peak_hold_frames: u32::MAX,
            maximum_smoothing_samples: u32::MAX,
        },
    )
    .map_err(|value| builtin_diagnostics(&value.0))?;
    let builtin_resources = builtins.resource_report();
    let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        dispatch: KernelDispatch::select(target_capabilities()),
        plan_id: 1,
        effects,
        builtins,
        caps: GraphCompileCaps {
            maximum_nodes: u64::MAX,
            maximum_edges: u64::MAX,
            maximum_schedule_items: u64::MAX,
            maximum_dependency_levels: u64::MAX,
            maximum_audio_buffer_samples: u64::MAX,
            maximum_delay_samples_per_edge: u64::MAX,
            maximum_total_delay_samples: u64::MAX,
            maximum_graph_bytes: config.maximum_graph_session_plus_plan_bytes,
            maximum_plan_bytes: config.maximum_graph_session_plus_plan_bytes,
            maximum_single_allocation_bytes: config.maximum_named_allocation_bytes,
            maximum_finite_tail_samples: u64::MAX,
        },
    })
    .map_err(|value| graph_diagnostics(value.diagnostics.diagnostics()))?;
    let graph_resources = artifact.graph_resource_estimate().clone();
    let source_set = prepare_graph_source_set(artifact.envelope(), graph_sources, mappings)
        .map_err(|_| fixed("web.source.graph"))?;
    let source_resources = source_set.resource_report();
    if source_resources.total_engine_owned_bytes > config.maximum_source_total_bytes
        || source_resources.overhead_bytes > config.maximum_source_overhead_bytes
    {
        return Err(fixed("web.source.resource"));
    }
    let external_nodes = artifact
        .external_binding_nodes()
        .filter(|node| {
            !matches!(
                node,
                GraphNodeId::TrackStage {
                    stage: TrackStage::Input,
                    ..
                }
            )
        })
        .cloned()
        .collect::<Vec<_>>();
    let bindings = GraphRuntimeBindings {
        #[cfg(not(target_arch = "wasm32"))]
        worker_lease: None,
        envelope: artifact.envelope(),
        nodes: external_nodes
            .into_iter()
            .map(|node| GraphNodeBinding::new(node, Box::new(IdentityProcessor)))
            .collect(),
        observers: Vec::new(),
    };
    let bound = artifact
        .into_bound_with_source_set(bindings, source_set)
        .map_err(|value| fixed(value.code))?;
    if !bound.meter_consumers.is_empty() {
        return Err(fixed("web.meter.unexpected"));
    }

    let session_resources = compiled.resource_estimate();
    let ready_metadata = checked_sum_prepare([
        layout_bytes::<ControlSource>(controls.len())?,
        u64::try_from(source_id_bytes).map_err(|_| fixed("web.resource.platform"))?,
        session_resources.compiled_model_bytes,
    ])?;
    let bridge_metadata = report
        .bridge_metadata_bytes
        .checked_add(ready_metadata)
        .ok_or_else(|| fixed("web.resource.arithmetic"))?;
    let bridge_retained = report
        .bridge_retained_bytes
        .checked_add(ready_metadata)
        .ok_or_else(|| fixed("web.resource.arithmetic"))?;
    let ready_largest = [
        layout_bytes::<ControlSource>(controls.len())?,
        u64::try_from(source_id_bytes).map_err(|_| fixed("web.resource.platform"))?,
        session_resources.single_allocation_bytes,
    ]
    .into_iter()
    .max()
    .unwrap_or(0);
    let bridge_largest = report.largest_bridge_allocation_bytes.max(ready_largest);
    let largest_named = bridge_largest
        .max(graph_resources.largest_allocation_bytes)
        .max(source_resources.largest_allocation_bytes)
        .max(builtin_resources.maximum_single_allocation_bytes);
    if bridge_retained > config.maximum_host_retained_bytes
        || largest_named > config.maximum_named_allocation_bytes
        || builtin_resources.engine_owned_retained_payload_bytes
            > config.maximum_builtin_retained_bytes
    {
        return Err(fixed("web.resource.limit"));
    }
    report.bridge_metadata_bytes = bridge_metadata;
    report.bridge_retained_bytes = bridge_retained;
    report.largest_bridge_allocation_bytes = bridge_largest;
    report.source_total_bytes = source_resources.total_engine_owned_bytes;
    report.source_overhead_bytes = source_resources.overhead_bytes;
    report.effect_scalar_state_bytes = effect_state;
    report.effect_scalar_scratch_bytes = effect_scratch;
    report.builtin_retained_bytes = builtin_resources.engine_owned_retained_payload_bytes;
    report.graph_session_plus_plan_bytes = graph_resources.session_plus_plan_bytes;
    report.graph_incremental_plan_bytes = graph_resources.incremental_plan_bytes;
    report.graph_metadata_bytes = graph_resources.graph_metadata_bytes;
    report.graph_delay_bytes = graph_resources.delay_bytes;
    report.largest_named_allocation_bytes = largest_named;
    Ok((
        ReadyOwnership {
            plan: bound.plan,
            _session: compiled,
            source_ids: ids.into_boxed_slice(),
            sources: controls.into_boxed_slice(),
        },
        report,
    ))
}

fn find_source_mut<'a>(
    ready: &'a mut ReadyOwnership,
    source_id: &[u8],
) -> Option<&'a mut ControlSource> {
    let ids = &ready.source_ids;
    ready.sources.iter_mut().find(|source| {
        ids.get(source.id_offset..source.id_offset + source.id_bytes) == Some(source_id)
    })
}

fn count_effects(model: &miso_engine_session::SessionTomlV1) -> Result<u64, PrepareFailure> {
    model.tracks.iter().try_fold(0_u64, |total, track| {
        let count = track
            .simd1
            .effects
            .len()
            .checked_add(track.dynamic.effects.len())
            .and_then(|value| value.checked_add(track.simd2.effects.len()))
            .ok_or_else(|| fixed("web.resource.arithmetic"))?;
        total
            .checked_add(u64::try_from(count).map_err(|_| fixed("web.resource.platform"))?)
            .ok_or_else(|| fixed("web.resource.arithmetic"))
    })
}

fn fixed(code: &str) -> PrepareFailure {
    PrepareFailure::fixed(RESULT_PREPARE_REJECTED, &format!("{code}\t$\n"))
}

fn session_diagnostics(value: &DiagnosticSet) -> PrepareFailure {
    let mut bytes = Vec::new();
    for diagnostic in value.diagnostics() {
        bytes.extend_from_slice(diagnostic.code.as_str().as_bytes());
        bytes.push(b'\t');
        bytes.extend_from_slice(diagnostic.path.to_string().as_bytes());
        bytes.push(b'\n');
    }
    PrepareFailure {
        code: RESULT_PREPARE_REJECTED,
        diagnostic: bytes,
    }
}

fn effect_diagnostics(values: &[miso_engine_effect_compiler::EffectDiagnostic]) -> PrepareFailure {
    let mut bytes = Vec::new();
    for value in values {
        bytes.extend_from_slice(value.code.as_bytes());
        bytes.push(b'\t');
        bytes.extend_from_slice(value.path.as_bytes());
        bytes.push(b'\n');
    }
    PrepareFailure {
        code: RESULT_PREPARE_REJECTED,
        diagnostic: bytes,
    }
}

fn builtin_diagnostics(
    values: &[miso_engine_builtins_compiler::BuiltinDiagnostic],
) -> PrepareFailure {
    let mut bytes = Vec::new();
    for value in values {
        bytes.extend_from_slice(value.code.as_bytes());
        bytes.push(b'\t');
        bytes.extend_from_slice(value.path.as_bytes());
        bytes.push(b'\n');
    }
    PrepareFailure {
        code: RESULT_PREPARE_REJECTED,
        diagnostic: bytes,
    }
}

fn graph_diagnostics<'a>(
    values: impl IntoIterator<Item = &'a miso_engine_graph::GraphDiagnostic>,
) -> PrepareFailure {
    let mut bytes = Vec::new();
    for value in values {
        bytes.extend_from_slice(value.code.as_bytes());
        bytes.push(b'\t');
        bytes.extend_from_slice(value.path.as_bytes());
        bytes.push(b'\n');
    }
    PrepareFailure {
        code: RESULT_PREPARE_REJECTED,
        diagnostic: bytes,
    }
}

fn checked_sum(values: impl IntoIterator<Item = u64>) -> Result<u64, u32> {
    values.into_iter().try_fold(0_u64, |total, value| {
        total.checked_add(value).ok_or(RESULT_PREPARE_REJECTED)
    })
}

fn checked_sum_prepare(values: impl IntoIterator<Item = u64>) -> Result<u64, PrepareFailure> {
    values.into_iter().try_fold(0_u64, |total, value| {
        total
            .checked_add(value)
            .ok_or_else(|| fixed("web.resource.arithmetic"))
    })
}

fn checked_product<const N: usize>(values: [u64; N]) -> Result<u64, u32> {
    values.into_iter().try_fold(1_u64, |total, value| {
        total.checked_mul(value).ok_or(RESULT_PREPARE_REJECTED)
    })
}

fn layout_bytes<T>(count: usize) -> Result<u64, PrepareFailure> {
    let layout = Layout::array::<T>(count).map_err(|_| fixed("web.resource.arithmetic"))?;
    u64::try_from(layout.size()).map_err(|_| fixed("web.resource.platform"))
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

/// Return portable bootstrap values for a Web embedding host.
#[must_use]
pub fn web_target_smoke() -> TargetSmoke {
    miso_engine_target_smoke::target_smoke()
}

mod ffi;

pub use ffi::*;

#[cfg(test)]
mod tests;
