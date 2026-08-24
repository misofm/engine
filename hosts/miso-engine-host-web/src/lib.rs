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
use core::num::{NonZeroU32, NonZeroUsize};

use miso_engine_builtins::{Matrix2x2, MeterSnapshot, MeterTap, pan_matrix};
use miso_engine_builtins_compiler::{MeterConsumer, TrackControlProducer, TrackControlRecordV1};
use miso_engine_core::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use miso_engine_host_core::{
    CompiledSession, HostConsoleRequestV1, HostPrepareCaps, HostShapePolicy, PreparedHost,
    SourceControlError, SourceSubmission, control_table_bytes, prepare_host_session_with_console,
    source_id_arena_bytes,
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
/// Fixed live-console command staging buffer (issue #137 D1).
pub const BUFFER_COMMAND: u32 = 6;
/// Fixed decimated meter-frame buffer (issue #137 D2).
pub const BUFFER_METER_FRAME: u32 = 7;

/// Exact byte size of one staged `miso.command.v1` record.
pub const COMMAND_RECORD_BYTES: u32 = 48;
/// Largest number of records one `miso.command.v1` submission may stage.
///
/// The staging buffer is allocated at preparation for exactly this many records, so a submission
/// can never grow the module's memory and the render path never sees an allocation.
pub const MAXIMUM_COMMAND_RECORDS: u32 = 256;
/// Per-track control-queue depth used when the configuration asks for the default.
pub const DEFAULT_COMMAND_QUEUE_RECORDS: u32 = 64;

/// Retarget the track's pan pair (`left`, `right`) over an explicit ramp window.
pub const COMMAND_PAN: u32 = 1;
/// Retarget the track's full 2x2 matrix over an explicit ramp window.
pub const COMMAND_MATRIX: u32 = 2;
/// Set a lane fader in decibels. Declared, validated, and refused; see [`COMMAND_REASON_UNSUPPORTED_KIND`].
pub const COMMAND_FADER_DB: u32 = 3;
/// Set a lane mute. Declared, validated, and refused; see [`COMMAND_REASON_UNSUPPORTED_KIND`].
pub const COMMAND_MUTE: u32 = 4;
/// Set an effect parameter. Declared, validated, and refused; see [`COMMAND_REASON_UNSUPPORTED_KIND`].
pub const COMMAND_EFFECT_PARAM: u32 = 5;
/// Set an effect bypass. Declared, validated, and refused; see [`COMMAND_REASON_UNSUPPORTED_KIND`].
pub const COMMAND_EFFECT_BYPASS: u32 = 6;

/// The submission was admitted whole.
pub const COMMAND_REASON_NONE: u32 = 0;
/// A record's fixed shape is wrong: an unknown kind, a nonzero reserved word, or a non-finite value.
pub const COMMAND_REASON_MALFORMED: u32 = 1;
/// `track_index` is not a track of the compiled session.
pub const COMMAND_REASON_UNKNOWN_TRACK: u32 = 2;
/// `rack` is not one of the three declared racks.
pub const COMMAND_REASON_UNKNOWN_RACK: u32 = 3;
/// `effect_index` is not an effect of the addressed rack.
pub const COMMAND_REASON_UNKNOWN_EFFECT: u32 = 4;
/// `parameter_id` is not a parameter of the addressed effect.
pub const COMMAND_REASON_UNKNOWN_PARAMETER: u32 = 5;
/// A value is outside the addressed parameter's declared domain.
pub const COMMAND_REASON_DOMAIN: u32 = 6;
/// The record is well formed and correctly addressed, but this ABI version cannot apply its kind.
///
/// This is not "malformed" and it is not "unknown target": the parameter exists and the value is
/// legal, and the engine has no post-preparation write path for it. The parameter-metadata JSON
/// (deliverable 4) marks every such parameter, so a caller never has to discover this at runtime.
pub const COMMAND_REASON_UNSUPPORTED_KIND: u32 = 7;
/// A bounded control queue had no room for the submission; nothing was admitted.
pub const COMMAND_REASON_BACKPRESSURE: u32 = 8;
/// The host is not `STATE_READY`.
pub const COMMAND_REASON_WRONG_STATE: u32 = 9;

/// The longest main-thread stall the default source ring hides without an underrun.
///
/// Chrome reports any main-thread task over 50 ms as a long task; a major garbage collection or a
/// layout burst in a real page runs 10-100 ms. The AudioWorklet render thread keeps pulling
/// quanta throughout, so a ring shorter than the stall underruns every source in the session. This
/// is a JIT-streaming budget, not a latency budget: the ring is prefilled ahead of the render
/// position and adds no output latency.
pub const SOURCE_STALL_TOLERANCE_MS: u32 = 100;

/// The default per-source ring capacity in frames for one rate and quantum.
///
/// `ceil(tolerance * fs / quantum)` quanta to cover the stall, plus two: one quantum is held by the
/// render consumer while it is being read, and one is in the recycle path between the consumer and
/// the producer. The result is always a whole number of quanta, which is what `PcmSourceRing`
/// requires of a capacity.
///
/// At 48 kHz / 128 that is 5 120 frames, 40 KiB for a stereo `f32` source -- 1 024 such sources fit
/// inside the 64 MiB default `maximum_source_total_bytes`.
///
/// `quantum_frames` must be nonzero. Every caller inside this crate runs after `validate_config`,
/// which rejects a zero quantum before any ring is built.
#[must_use]
pub const fn default_source_ring_frames(sample_rate_hz: u32, quantum_frames: u32) -> u32 {
    let stall_frames = (sample_rate_hz as u64 * SOURCE_STALL_TOLERANCE_MS as u64) / 1000;
    let quanta = stall_frames.div_ceil(quantum_frames as u64) + 2;
    (quanta * quantum_frames as u64) as u32
}

/// Default meter window in render blocks: ~31 frames per second at 48 kHz with a 128-frame quantum.
pub const DEFAULT_METER_BLOCKS: u32 = 12;

/// Exact versioned live-console command report shared with JavaScript (issue #137 D1).
///
/// One submission is one transaction: either every staged record was admitted, or none was and
/// `rejected_index`/`reason` name the first record that broke a rule. `applied_at_sample` is the
/// absolute sample the admitted records take effect at -- the first sample of the next rendered
/// block, because the matrix stage drains its queue at the top of the block before it touches a
/// single sample of audio.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WebCommandReportV1 {
    /// Exact structure byte size.
    pub struct_size: u32,
    /// ABI version.
    pub abi_version: u32,
    /// One of the frozen `RESULT_*` values.
    pub result: u32,
    /// One of the frozen `COMMAND_REASON_*` values.
    pub reason: u32,
    /// Zero-based index of the first refused record, or `0` when the submission was admitted.
    pub rejected_index: u32,
    /// Number of records admitted by this submission: the whole batch, or zero.
    pub admitted: u32,
    /// Absolute sample at which the admitted records take effect.
    pub applied_at_sample: u64,
    /// Required-zero expansion words.
    pub reserved: [u64; 2],
}

/// Byte size of [`WebCommandReportV1`].
pub const COMMAND_REPORT_BYTES: u32 = size_of::<WebCommandReportV1>() as u32;

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
    /// Per-track live-console control-queue depth in records, or `0` to attach no control channel
    /// and no command staging at all (issue #137 D1).
    ///
    /// Carved out of the frozen configuration's first reserved word, which every V1 writer already
    /// sets to zero. Zero is the honest form of "no live console": no queue is allocated, no
    /// staging buffer is allocated, the matrix processors keep the exact storage they had before
    /// this ABI existed, and a submission is refused with [`RESULT_UNSUPPORTED`]. The 192-byte
    /// layout is unchanged either way.
    pub console_command_queue_records: u64,
    /// Meter window in render blocks, or `0` to attach no meters at all (issue #137 D2).
    ///
    /// Zero is the honest form of "metering off costs nothing": no observer is bound, so the
    /// render path does not fold a single sample. A nonzero value binds one post-matrix meter per
    /// track with a `blocks * quantum_frames` window; the port lease then gates whether a finished
    /// window is posted. `12` is ~31 frames per second at 48 kHz with a 128-frame quantum.
    pub console_meter_blocks: u64,
    /// Required-zero expansion words.
    pub reserved: [u64; 2],
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
            source_ring_frames: default_source_ring_frames(sample_rate_hz, quantum_frames),
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
            console_command_queue_records: 0,
            console_meter_blocks: 0,
            reserved: [0; 2],
        }
    }

    /// The launch defaults with the live web console attached (issue #137 D1/D2).
    #[must_use]
    pub const fn console_defaults(sample_rate_hz: u32, quantum_frames: u32) -> Self {
        Self {
            console_command_queue_records: DEFAULT_COMMAND_QUEUE_RECORDS as u64,
            console_meter_blocks: DEFAULT_METER_BLOCKS as u64,
            ..Self::launch_defaults(sample_rate_hz, quantum_frames)
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
    /// Fixed `MAXIMUM_COMMAND_RECORDS * COMMAND_RECORD_BYTES` live-console staging (issue #137 D1).
    command: Box<[u8]>,
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
    /// Issue #137 D1: control-side producers, declared first so they are released before the plan
    /// that owns their consumer endpoints.
    controls: Vec<TrackControlProducer>,
    /// Per-track room needed by the submission being validated. Allocated at compilation.
    command_wanted: Box<[u32]>,
    /// Decoded submission staging, `MAXIMUM_COMMAND_RECORDS` long. Allocated at compilation.
    command_decoded: Box<[(u32, TrackControlRecordV1)]>,
    /// Per-track records admitted since the last successful render.
    ///
    /// The browser's control plane and render plane are the same thread and the matrix stage
    /// drains its whole queue at the top of every block, so a successful render empties every
    /// control queue. That is what makes this an exact free-slot count rather than an estimate,
    /// and it is what lets a submission be refused *before* anything is pushed.
    in_flight: Box<[u32]>,
    /// Canonical normalized track order: the addressing authority for `track_index`.
    tracks: Vec<Box<str>>,
    /// Effects declared per track per rack, `[simd1, dynamic, simd2]`, so an effect-addressed
    /// command is answered with `UNKNOWN_RACK` / `UNKNOWN_EFFECT` before anything else.
    rack_effects: Box<[[u32; 3]]>,
    host: PreparedHost,
    /// Issue #137 D2: meter consumers, declared after the plan that owns their producers. Empty
    /// when `console_meter_blocks` was zero, in which case no observer exists at all.
    meters: Vec<MeterConsumer>,
    /// `[track0 L, track0 R, .., trackN L, trackN R, master L, master R]` peak magnitudes.
    meter_frame: Box<[f32]>,
    /// Master peaks folded over the rendered block while the lease is on.
    master_peak: [f32; 2],
    /// Windows folded into `meter_frame` since the host was compiled.
    meter_windows: u64,
    /// Retained so the browser bridge keeps charging itself for the compiled model it holds; the
    /// V1 browser ABI has no session query, so nothing reads it.
    _session: CompiledSession,
}

/// Safe ownership object backing one future AudioWorklet Wasm handle.
pub struct AudioWorkletEngineHost {
    config: WebPrepareConfigV1,
    status: WebStatusV1,
    resources: WebResourceReportV1,
    command_report: WebCommandReportV1,
    /// Issue #137 D2: the meter lease. `false` skips the master fold and every drain.
    meter_lease: bool,
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
            command_report: empty_command_report(),
            meter_lease: false,
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

    /// Read the last live-console submission report (issue #137 D1).
    #[must_use]
    pub const fn command_report(&self) -> &WebCommandReportV1 {
        &self.command_report
    }

    /// Canonical normalized track order, the addressing authority for `track_index`.
    #[must_use]
    pub fn console_tracks(&self) -> &[Box<str>] {
        self.ready.as_ref().map_or(&[], |ready| &ready.tracks)
    }

    /// The decimated meter frame: two peaks per track, then master left and right (issue #137 D2).
    #[must_use]
    pub fn meter_frame(&self) -> &[f32] {
        self.ready.as_ref().map_or(&[], |ready| &ready.meter_frame)
    }

    /// Number of complete meter windows folded since compilation.
    #[must_use]
    pub fn meter_windows(&self) -> u64 {
        self.ready.as_ref().map_or(0, |ready| ready.meter_windows)
    }

    /// Whether meter observers were attached at preparation (issue #137 D2).
    #[must_use]
    pub fn meters_attached(&self) -> bool {
        self.ready
            .as_ref()
            .is_some_and(|ready| !ready.meters.is_empty())
    }

    /// Take or release the meter lease. Returns the frozen result code.
    ///
    /// A lease over a host that attached no observers is refused with [`RESULT_UNSUPPORTED`]: the
    /// caller asked for numbers this preparation cannot produce, and silently reporting zeros
    /// would be worse than saying so.
    pub fn set_meter_lease(&mut self, enabled: bool) -> u32 {
        if self.status.state != STATE_READY {
            return self.record(RESULT_WRONG_STATE);
        }
        if enabled && !self.meters_attached() {
            return self.record(RESULT_UNSUPPORTED);
        }
        self.meter_lease = enabled;
        if let Some(ready) = self.ready.as_mut() {
            ready.master_peak = [0.0, 0.0];
            ready.meter_frame.fill(0.0);
        }
        self.record(RESULT_OK)
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

    pub(crate) fn command_staging_mut(&mut self) -> Option<&mut [u8]> {
        self.buffers
            .as_mut()
            .map(|value| &mut *value.command)
            .filter(|value| !value.is_empty())
    }

    /// Exact byte size of the live-console command staging buffer; zero when none was attached.
    #[must_use]
    pub fn command_staging_bytes(&self) -> u64 {
        self.buffers
            .as_ref()
            .map_or(0, |value| value.command.len() as u64)
    }

    /// Whether preparation attached a live-console control channel (issue #137 D1).
    #[must_use]
    pub fn console_attached(&self) -> bool {
        self.ready
            .as_ref()
            .is_some_and(|ready| !ready.controls.is_empty())
    }

    /// Copy one canonical console track ID into source-ID staging; returns its byte length.
    pub(crate) fn copy_console_track_id(&mut self, index: u32) -> u32 {
        let Some(ready) = self.ready.as_ref() else {
            return 0;
        };
        let Some(id) = ready.tracks.get(index as usize) else {
            return 0;
        };
        let bytes = id.as_bytes();
        let length = bytes.len();
        let Some(buffers) = self.buffers.as_mut() else {
            return 0;
        };
        let Some(target) = buffers.source_id.get_mut(..length) else {
            return 0;
        };
        target.copy_from_slice(bytes);
        u32::try_from(length).unwrap_or(0)
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
                // Issue #137 D1: the matrix stage drained every control queue at the top of this
                // block, so the exact free-slot count is the whole capacity again.
                ready.in_flight.fill(0);
                // Issue #137 D2: the master bus is the host's own output plane, so there is
                // nothing to observe and nothing to expose -- one branch and one pass over a
                // buffer already in cache, and only while the lease is held.
                if self.meter_lease {
                    // Indexed with `get`, never with a range: a slice index would put
                    // `slice_index_fail` in the render export's call graph, and the shipped
                    // artifact's gate would -- correctly -- refuse the build.
                    let mut peaks = ready.master_peak;
                    for (plane, peak) in peaks.iter_mut().enumerate() {
                        let start = plane * quantum;
                        let Some(samples) = buffers
                            .output_pcm
                            .get(start..)
                            .and_then(|tail| tail.get(..quantum))
                        else {
                            break;
                        };
                        for sample in samples {
                            let magnitude = sample.abs();
                            if magnitude > *peak {
                                *peak = magnitude;
                            }
                        }
                    }
                    ready.master_peak = peaks;
                }
                self.record(RESULT_OK)
            }
            Err(_) => self.fail(RESULT_RENDER_REJECTED, b"web.render.rejected\t$\n"),
        }
    }

    /// Admit one staged `miso.command.v1` submission as a single transaction (issue #137 D1).
    ///
    /// # The two-pass shape is the contract, not an optimisation
    ///
    /// Pass one validates every record -- shape, addressing, domain, and free queue room -- and
    /// pushes nothing. Pass two pushes. A submission is therefore all-or-nothing: eval E4's
    /// "engine state untouched" is a property of the code, not of a lucky ordering, and eval E3's
    /// flood is refused before a single record reaches a queue.
    ///
    /// # Why this is not on the render thread's critical path
    ///
    /// This runs from `port.onmessage`, which the user agent dispatches between render quanta on
    /// the same thread as `process()`. It performs no allocation, no compilation and no plan
    /// replacement: both staging arrays were allocated at compilation, and the call moves at most
    /// [`MAXIMUM_COMMAND_RECORDS`] `Copy` records into bounded per-track queues. The block that
    /// follows drains them.
    pub fn submit_commands(&mut self, count: u32) -> u32 {
        self.command_report = empty_command_report();
        if self.status.state != STATE_READY {
            return self.finish_commands(RESULT_WRONG_STATE, COMMAND_REASON_WRONG_STATE, 0, 0);
        }
        if count > MAXIMUM_COMMAND_RECORDS {
            return self.finish_commands(RESULT_INVALID_ARGUMENT, COMMAND_REASON_MALFORMED, 0, 0);
        }
        if !self.console_attached() {
            return self.finish_commands(RESULT_UNSUPPORTED, COMMAND_REASON_UNSUPPORTED_KIND, 0, 0);
        }
        let applied_at_sample = self.status.next_absolute_sample;
        let staged = count as usize * COMMAND_RECORD_BYTES as usize;
        if self.buffers.is_none() {
            return self.fail(RESULT_INTERNAL, b"web.internal.buffers\t$\n");
        }
        if self.ready.is_none() {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        }
        // Disjoint borrows: the staged bytes live in `buffers`, the console lives in `ready`.
        let Some((buffers, ready)) = self.buffers.as_ref().zip(self.ready.as_mut()) else {
            return self.fail(RESULT_INTERNAL, b"web.internal.console\t$\n");
        };
        let Some(bytes) = buffers.command.get(..staged) else {
            return self.finish_commands(RESULT_INVALID_ARGUMENT, COMMAND_REASON_MALFORMED, 0, 0);
        };
        match admit_commands(ready, bytes, count as usize) {
            Ok(()) => {
                self.command_report.applied_at_sample = applied_at_sample;
                self.finish_commands(RESULT_OK, COMMAND_REASON_NONE, 0, count)
            }
            Err(CommandRejection {
                result,
                reason,
                index,
            }) => self.finish_commands(result, reason, index, 0),
        }
    }

    /// Drain every finished meter window into the frame buffer (issue #137 D2).
    ///
    /// Returns the number of complete windows folded by this call. Zero work happens without the
    /// lease, and a host prepared with `console_meter_blocks == 0` has no observer to drain.
    ///
    /// Allocation-free by construction: it moves `Copy` snapshots out of bounded queues into a
    /// buffer allocated at compilation.
    pub fn poll_meters(&mut self) -> u32 {
        if !self.meter_lease || self.status.state != STATE_READY {
            return 0;
        }
        let Some(ready) = self.ready.as_mut() else {
            return 0;
        };
        // Every index below is a `get_mut`, never a `[]`: a bounds check would put
        // `panic_bounds_check` in this export's call graph, and this export is called from
        // `process()`, so the shipped artifact's gate covers it exactly as it covers the render
        // export.
        let mut windows = 0_u32;
        for (index, meter) in ready.meters.iter_mut().enumerate() {
            let mut popped = 0_u32;
            while let Ok(snapshot) = meter.consumer.try_pop() {
                let MeterSnapshot { left, right, .. } = snapshot;
                if let Some(slot) = ready.meter_frame.get_mut(index * 2) {
                    *slot = left.sample_peak;
                }
                if let Some(slot) = ready.meter_frame.get_mut(index * 2 + 1) {
                    *slot = right.sample_peak;
                }
                popped = popped.saturating_add(1);
            }
            // Every track's meter shares one window length and one start sample, so they finish
            // together; the minimum is the number of windows the whole frame actually covers.
            windows = if index == 0 {
                popped
            } else {
                windows.min(popped)
            };
        }
        let master = ready.meter_frame.len().saturating_sub(2);
        for (plane, peak) in ready.master_peak.iter().enumerate() {
            if let Some(slot) = ready.meter_frame.get_mut(master + plane) {
                *slot = *peak;
            }
        }
        ready.master_peak = [0.0, 0.0];
        ready.meter_windows = ready.meter_windows.saturating_add(u64::from(windows));
        windows
    }

    fn finish_commands(&mut self, result: u32, reason: u32, index: u32, admitted: u32) -> u32 {
        self.command_report.result = result;
        self.command_report.reason = reason;
        self.command_report.rejected_index = index;
        self.command_report.admitted = admitted;
        if admitted == 0 {
            self.command_report.applied_at_sample = self.status.next_absolute_sample;
        }
        self.record(result)
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
        self.meter_lease = false;
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

/// One refused submission: the frozen result, the typed reason, and the offending record.
struct CommandRejection {
    result: u32,
    reason: u32,
    index: u32,
}

/// One decoded, still-unapplied `miso.command.v1` record (issue #137 D1).
///
/// # The frozen 48-byte little-endian layout
///
/// | offset | width | field |
/// |---|---|---|
/// | 0 | u8 | `kind`, one of the `COMMAND_*` values |
/// | 1 | u8 | `rack`: `0` simd1, `1` dynamic, `2` simd2, `255` not applicable |
/// | 2 | u8 | `channel`: `0` left, `1` right, `2` both, `255` not applicable |
/// | 3 | u8 | required zero |
/// | 4 | u32 | `track_index` into the canonical normalized track order |
/// | 8 | u32 | `effect_index` within the addressed rack |
/// | 12 | u32 | `parameter_id` from the effect contract |
/// | 16 | u32 | `smoothing_samples`, the ramp window for a retarget |
/// | 20 | u32 | required zero |
/// | 24..40 | 4 x f32 | `value0..value3` |
/// | 40 | u64 | required zero |
///
/// There is no string on this path. The identity mapping a caller needs -- track index to track
/// ID, effect index to effect ID, parameter ID to name and domain -- is deliverable 4's build-time
/// metadata plus the session map the ready message carries, never a per-command lookup.
#[derive(Clone, Copy)]
struct CommandRecord {
    kind: u32,
    rack: u8,
    channel: u8,
    track_index: u32,
    effect_index: u32,
    parameter_id: u32,
    smoothing_samples: u32,
    values: [f32; 4],
}

impl CommandRecord {
    fn decode(bytes: &[u8]) -> Result<Self, u32> {
        if bytes.len() != COMMAND_RECORD_BYTES as usize {
            return Err(COMMAND_REASON_MALFORMED);
        }
        let word = |offset: usize| {
            u32::from_le_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
            ])
        };
        if bytes[3] != 0 || word(20) != 0 || bytes[40..48].iter().any(|value| *value != 0) {
            return Err(COMMAND_REASON_MALFORMED);
        }
        let kind = u32::from(bytes[0]);
        if !matches!(
            kind,
            COMMAND_PAN
                | COMMAND_MATRIX
                | COMMAND_FADER_DB
                | COMMAND_MUTE
                | COMMAND_EFFECT_PARAM
                | COMMAND_EFFECT_BYPASS
        ) {
            return Err(COMMAND_REASON_MALFORMED);
        }
        let mut values = [0.0_f32; 4];
        for (index, value) in values.iter_mut().enumerate() {
            *value = f32::from_le_bytes([
                bytes[24 + index * 4],
                bytes[25 + index * 4],
                bytes[26 + index * 4],
                bytes[27 + index * 4],
            ]);
            if !value.is_finite() {
                return Err(COMMAND_REASON_MALFORMED);
            }
        }
        Ok(Self {
            kind,
            rack: bytes[1],
            channel: bytes[2],
            track_index: word(4),
            effect_index: word(8),
            parameter_id: word(12),
            smoothing_samples: word(16),
            values,
        })
    }

    /// Lower one addressed record onto the single live builtin surface, or say why it cannot be.
    ///
    /// `matrix_ll/lr/rl/rr` are the only builtin parameters whose declared update rate is
    /// `BuiltinParameterUpdateRate::BlockTarget`; `fader_db`, `mute` and every effect parameter
    /// declare `PreparedOnly` or have no post-preparation write path at all, so they are refused
    /// with [`COMMAND_REASON_UNSUPPORTED_KIND`] *after* their addressing and domain have been
    /// checked. A caller can therefore tell "you addressed nothing" from "you addressed something
    /// this engine cannot move yet", and deliverable 4's metadata tells it which is which before
    /// it ever sends one.
    fn into_matrix(self, rack_effects: [u32; 3]) -> Result<TrackControlRecordV1, u32> {
        match self.kind {
            COMMAND_PAN => {
                if self.rack != 255
                    || self.channel != 255
                    || self.values[2] != 0.0
                    || self.values[3] != 0.0
                {
                    return Err(COMMAND_REASON_MALFORMED);
                }
                let matrix = pan_matrix(self.values[0], self.values[1])
                    .map_err(|_| COMMAND_REASON_DOMAIN)?;
                Ok(TrackControlRecordV1 {
                    matrix,
                    smoothing_samples: self.smoothing_samples,
                })
            }
            COMMAND_MATRIX => {
                if self.rack != 255 || self.channel != 255 {
                    return Err(COMMAND_REASON_MALFORMED);
                }
                let matrix = Matrix2x2 {
                    ll: self.values[0],
                    lr: self.values[1],
                    rl: self.values[2],
                    rr: self.values[3],
                }
                .checked()
                .map_err(|_| COMMAND_REASON_DOMAIN)?;
                Ok(TrackControlRecordV1 {
                    matrix,
                    smoothing_samples: self.smoothing_samples,
                })
            }
            COMMAND_FADER_DB => {
                if self.rack != 255 || self.channel > 2 {
                    return Err(COMMAND_REASON_MALFORMED);
                }
                if !(-144.0..=24.0).contains(&self.values[0]) {
                    return Err(COMMAND_REASON_DOMAIN);
                }
                Err(COMMAND_REASON_UNSUPPORTED_KIND)
            }
            COMMAND_MUTE => {
                if self.rack != 255 || self.channel > 2 {
                    return Err(COMMAND_REASON_MALFORMED);
                }
                if self.values[0] != 0.0 && self.values[0] != 1.0 {
                    return Err(COMMAND_REASON_DOMAIN);
                }
                Err(COMMAND_REASON_UNSUPPORTED_KIND)
            }
            COMMAND_EFFECT_PARAM | COMMAND_EFFECT_BYPASS => {
                if self.rack > 2 {
                    return Err(COMMAND_REASON_UNKNOWN_RACK);
                }
                if self.effect_index >= rack_effects[self.rack as usize] {
                    return Err(COMMAND_REASON_UNKNOWN_EFFECT);
                }
                if self.kind == COMMAND_EFFECT_PARAM {
                    if self.channel > 2 {
                        return Err(COMMAND_REASON_MALFORMED);
                    }
                    if self.parameter_id == 0 {
                        return Err(COMMAND_REASON_UNKNOWN_PARAMETER);
                    }
                } else if self.channel != 255 || (self.values[0] != 0.0 && self.values[0] != 1.0) {
                    return Err(COMMAND_REASON_MALFORMED);
                }
                Err(COMMAND_REASON_UNSUPPORTED_KIND)
            }
            _ => Err(COMMAND_REASON_MALFORMED),
        }
    }
}

/// Validate a whole staged submission, then admit it. Nothing is pushed unless everything passes.
fn admit_commands(
    ready: &mut ReadyOwnership,
    bytes: &[u8],
    count: usize,
) -> Result<(), CommandRejection> {
    let record_bytes = COMMAND_RECORD_BYTES as usize;
    let track_count = ready.tracks.len();
    ready.command_wanted.fill(0);
    for index in 0..count {
        let record = &bytes[index * record_bytes..(index + 1) * record_bytes];
        let command = CommandRecord::decode(record).map_err(|reason| CommandRejection {
            result: RESULT_INVALID_ARGUMENT,
            reason,
            index: index as u32,
        })?;
        let track = command.track_index as usize;
        if track >= track_count {
            return Err(CommandRejection {
                result: RESULT_INVALID_ARGUMENT,
                reason: COMMAND_REASON_UNKNOWN_TRACK,
                index: index as u32,
            });
        }
        let applied = command
            .into_matrix(ready.rack_effects[track])
            .map_err(|reason| CommandRejection {
                result: if reason == COMMAND_REASON_UNSUPPORTED_KIND {
                    RESULT_UNSUPPORTED
                } else {
                    RESULT_INVALID_ARGUMENT
                },
                reason,
                index: index as u32,
            })?;
        ready.command_wanted[track] += 1;
        ready.command_decoded[index] = (command.track_index, applied);
    }
    for index in 0..count {
        let track = ready.command_decoded[index].0 as usize;
        let needed = ready.command_wanted[track];
        let Some(producer) = ready.controls.get(track) else {
            return Err(CommandRejection {
                result: RESULT_UNSUPPORTED,
                reason: COMMAND_REASON_UNSUPPORTED_KIND,
                index: index as u32,
            });
        };
        let capacity = u32::try_from(producer.producer.capacity()).unwrap_or(0);
        if ready.in_flight[track].saturating_add(needed) > capacity {
            return Err(CommandRejection {
                result: RESULT_BACKPRESSURE,
                reason: COMMAND_REASON_BACKPRESSURE,
                index: index as u32,
            });
        }
    }
    for index in 0..count {
        let (track, record) = ready.command_decoded[index];
        let track = track as usize;
        let Some(producer) = ready.controls.get_mut(track) else {
            return Err(CommandRejection {
                result: RESULT_INTERNAL,
                reason: COMMAND_REASON_UNSUPPORTED_KIND,
                index: index as u32,
            });
        };
        if producer.producer.try_push(record).is_err() {
            return Err(CommandRejection {
                result: RESULT_INTERNAL,
                reason: COMMAND_REASON_BACKPRESSURE,
                index: index as u32,
            });
        }
        ready.in_flight[track] += 1;
    }
    Ok(())
}

const fn empty_command_report() -> WebCommandReportV1 {
    WebCommandReportV1 {
        struct_size: COMMAND_REPORT_BYTES,
        abi_version: ABI_VERSION,
        result: RESULT_OK,
        reason: COMMAND_REASON_NONE,
        rejected_index: 0,
        admitted: 0,
        applied_at_sample: 0,
        reserved: [0; 2],
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
    let command_records = if config.console_command_queue_records == 0 {
        0
    } else {
        MAXIMUM_COMMAND_RECORDS
    };
    let command_bytes = u64::from(command_records)
        .checked_mul(u64::from(COMMAND_RECORD_BYTES))
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
        command_bytes,
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
        command: boxed_zero_u8(command_records * COMMAND_RECORD_BYTES)?,
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
    if config.reserved != [0; 2]
        || config.console_command_queue_records > u64::from(MAXIMUM_COMMAND_RECORDS)
        || config.console_meter_blocks > u64::from(u32::MAX)
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
    let console = console_request(config).ok_or_else(|| fixed_diagnostic("web.console.config"))?;
    let (session, host, handles) = prepare_host_session_with_console(toml, &caps, &console)
        .map_err(|value| value.into_bytes())?;
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
    let track_count = handles.tracks.len();
    let mut rack_effects = Vec::new();
    rack_effects
        .try_reserve_exact(track_count)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    for track in &session.normalized_model().tracks {
        let count = |effects: usize| -> Result<u32, Vec<u8>> {
            u32::try_from(effects).map_err(|_| fixed_diagnostic("web.console.effects"))
        };
        rack_effects.push([
            count(track.simd1.effects.len())?,
            count(track.dynamic.effects.len())?,
            count(track.simd2.effects.len())?,
        ]);
    }
    let ready = ReadyOwnership {
        controls: handles.track_controls,
        command_wanted: boxed_zero_u32(track_count)?,
        command_decoded: boxed_command_staging()?,
        in_flight: boxed_zero_u32(track_count)?,
        tracks: handles.tracks,
        rack_effects: rack_effects.into_boxed_slice(),
        host,
        meters: handles.meters,
        meter_frame: boxed_zero_meter_frame(track_count)?,
        master_peak: [0.0, 0.0],
        meter_windows: 0,
        _session: session,
    };
    Ok((ready, report))
}

/// Translate the browser configuration's two console words into the facade's console request.
///
/// `console_meter_blocks == 0` is the honest form of "metering off": no observer is bound, so the
/// render path folds nothing at all. The port lease is a second, finer switch over posting.
fn console_request(config: WebPrepareConfigV1) -> Option<HostConsoleRequestV1> {
    let control_queue_depth = match config.console_command_queue_records {
        0 => None,
        records => Some(NonZeroUsize::new(u32::try_from(records).ok()? as usize)?),
    };
    let meter_period_frames = if config.console_meter_blocks == 0 {
        None
    } else {
        let blocks = u32::try_from(config.console_meter_blocks).ok()?;
        Some(NonZeroU32::new(blocks.checked_mul(config.quantum_frames)?)?)
    };
    Some(HostConsoleRequestV1 {
        control_queue_depth,
        meter_period_frames,
        // One window per track per post, plus headroom for a control-side stall of a few windows.
        meter_queue_depth: NonZeroUsize::new(8)?,
        meter_tap: MeterTap::PostMatrix,
    })
}

fn boxed_zero_u32(count: usize) -> Result<Box<[u32]>, Vec<u8>> {
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    value.resize(count, 0);
    Ok(value.into_boxed_slice())
}

fn boxed_zero_meter_frame(track_count: usize) -> Result<Box<[f32]>, Vec<u8>> {
    let count = track_count
        .checked_mul(2)
        .and_then(|value| value.checked_add(2))
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    value.resize(count, 0.0);
    Ok(value.into_boxed_slice())
}

fn boxed_command_staging() -> Result<Box<[(u32, TrackControlRecordV1)]>, Vec<u8>> {
    let count = MAXIMUM_COMMAND_RECORDS as usize;
    let empty = (
        0_u32,
        TrackControlRecordV1 {
            matrix: Matrix2x2::IDENTITY,
            smoothing_samples: 0,
        },
    );
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    value.resize(count, empty);
    Ok(value.into_boxed_slice())
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
