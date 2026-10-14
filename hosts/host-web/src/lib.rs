//! Safe browser-Wasm preparation and ownership shell.
//!
//! This module deliberately contains no raw pointer handling or JavaScript integration. It owns
//! the complete immutable session and render plan that the AudioWorklet boundary drives.
//!
//! # The control plane (issue #1381a)
//!
//! The one instance's session is `control_plane::SessionState`, built at boot through
//! [`WebRuntimePreparer`]: it owns the committed model, the source, strip and effect producers, the
//! plan's revision gate, the plan publisher and the retirer. The host renders through the plan
//! owner (`PlanState`), which adopts a published successor at a block boundary with its
//! [`RenderCompanions`], and the service step (`miso_engine_web_v1_service`) reclaims the displaced
//! plan and its attachment on the control side, in the worklet's message handler, never in
//! `process()`. Every value reaches the cells through `SET_CONTROL` and the core
//! (`control_command`, #1562a); the browser keeps no value admission of its own (#1565A). What
//! remains of the retained command record is the monitor operation that arms an observation tap
//! (kinds 7 and 8), which reaches the effect's observation queue through the control plane.
//!
//! # What is here and what is not
//!
//! The compile pipeline is **not** here. Parsing, compiling, source rings, effect and builtin
//! preparation, the graph compile, the identity bindings and the engine-owned resource report are
//! `host-core`, shared with the C ABI host (issue #103). This crate owns exactly what is
//! the browser's: the frozen issue-024 ABI structs and result codes, the fixed staging buffers the
//! JavaScript side reads and writes through raw addresses, the browser bridge's own resource rows,
//! and the render/failure state machine. Before #106 F1 it carried a second, already-diverged copy
//! of the shared pipeline -- 288 lines of `compile_ready` alone, only one of whose two copies
//! rejected source generation `0`.

use core::mem::{MaybeUninit, size_of};
use core::num::{NonZeroU32, NonZeroUsize};
use core::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use builtins::{MeterMetricSet, MeterSnapshot, MeterTap};
use builtins_compiler::MeterConsumer;
use control_plane::{
    CompileFailure, ControlAdapter, ControlLimits, PlanQueries, PlanResources, PlanState,
    SessionState, SourceFailure,
};
use effect_contract::{ObservationCost, ObservationDescriptor};
use engine::realtime::{HostAttachment, ObservationWindow, PlanarBufferMut};
use host_core::{
    CompiledSession, EffectObservationHandle, HostLiveControlRequest, HostLiveLanes,
    HostMeterRequest, HostPrepareCaps, HostPrepareReport, HostShapePolicy, MAXIMUM_VCA_REACH_PAIRS,
    MAXIMUM_VCAS_BOUNDED, PrepareDiagnostics, PrepareRejection, PreparedHost, SourceControlError,
    SourceSubmission, SuccessorBase, compile_host_model, compiled_session_shape,
    control_table_bytes, parse_host_session, prepare_host_runtime_with_live_controls_and_spectrum,
    source_id_arena_bytes,
};
use host_core::{
    PreparedSpectrumCaptures, ResponseSnapshotError, ResponseSnapshotSink, SpectrumCadence,
    SpectrumChannels, SpectrumContinuousCaptureError, SpectrumContinuousReadError,
    SpectrumContinuousWindow, SpectrumHop, SpectrumService, SpectrumServiceError, SpectrumSnapshot,
    SpectrumTarget, SpectrumTargetRef,
};
#[cfg(test)]
use host_core::{SpectrumCaptureCollectionRequest, SpectrumCaptureRequest};
use session::CompileCaps;

use host_core::EffectRack;
/// Live effect addressing in the session's own terms (decision 12, issue #1096).
pub use host_core::{LiveEffectAddress, LiveEffectRack};

pub use host_core::{SOURCE_STALL_TOLERANCE_MS, default_source_ring_frames};

/// Browser host ABI version 1.0.
pub const ABI_VERSION: u32 = 0x0001_0000;

/// Maximum exact staged document length. Dense automation belongs in future content-addressed
/// binary blobs rather than unbounded JSON.
pub const MAXIMUM_DOCUMENT_BYTES: u32 = 1 << 20;

/// Conservative transient parse projection in bytes per staged document byte.
///
/// Issue #338 re-measured the then-pinned `jstrict 0.14.0` JSON frontend plus typed model and compilation
/// over the minimal document, dense one-, 64-, and 192-track documents, and the exact 1 MiB
/// admitted ceiling. The largest observed ratio was 14.738 bytes per input byte; 17 left 15.3%
/// headroom. Issue #1093 re-measured it after decision 12's required root `console` grew the
/// minimal document from 447 to 511 bytes and its parse, model and compile peak from 6,780 to
/// 8,944 bytes: 17.503 bytes per input byte, now the largest ratio (the dense documents stay
/// below 13.1). 20 left 14.3% headroom. Issue #1381a re-measured the exact 1 MiB, 512-track by
/// 4-effect document on the shipped module (`scripts/check-web-boot-budget.mjs`): the boot now
/// builds the control plane's session, whose catalog, producers and protocol state the browser did
/// not hold, and the wasm high-water growth rose from under 20 MiB to 25,427,968 bytes (24.25
/// bytes per input byte). Root ruled 25 (2026-10-14): 25 MiB = 26,214,400 bytes is the tightest
/// whole multiplier above the measurement (3.1% headroom); the growth is decision 17 C6's
/// always-attached live lanes (per-effect lanes 4.0 MB, bank lanes 2.4 MB, control-plane rows
/// ~0.4 MB, input lanes 0.17 MB), and its successor ("Shrink the always-attached per-effect live
/// lanes") aims to bring the pin back to 20. Boot checks
/// `document_bytes * PARSE_TRANSIENT_MULTIPLIER` against the effective budget before UTF-8 decode
/// or parser allocation, and the peak-transient test keeps every phase visible so frontend growth
/// cannot silently outrun this projection.
pub const PARSE_TRANSIENT_MULTIPLIER: u64 = 25;

/// Default host memory ceiling used only when the embedding passes zero.
///
/// 512 MiB matches the native runner profile and the browser app's historical quota-reserve
/// scale. It leaves more than four times the retained headroom of every frozen corpus fixture and
/// more than seven times the measured ~70 MiB worst transient at the 1 MiB document cap.
pub const DEFAULT_MAXIMUM_MEMORY_BYTES: u64 = 512 << 20;

/// Fixed live diagnostic capacity after a successful boot.
pub const DIAGNOSTIC_BYTES: u32 = 1 << 14;

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
/// Boot refused the effective memory budget.
pub const RESULT_REFUSED_BUDGET: u32 = 5;
/// A bounded source queue is full.
pub const RESULT_BACKPRESSURE: u32 = 6;
/// Requested capability is unsupported.
pub const RESULT_UNSUPPORTED: u32 = 7;
/// Rendering failed.
pub const RESULT_RENDER_REJECTED: u32 = 8;
/// The browser configuration changed and requires a fresh host.
pub const RESULT_REPREPARE_REQUIRED: u32 = 9;
/// The control plane refused a structural command frame with a typed diagnostic (issue #1562a),
/// the browser's counterpart of the C ABI's compile-rejected result. The diagnostic is found at
/// the control response, in place of a response frame.
pub const RESULT_COMPILE_REJECTED: u32 = 10;
/// An internal invariant failed *and was detected by a checked path*.
///
/// A panic does not produce this code on `wasm32`: the target is `panic = abort`, so a panic traps
/// the instance and kills the processor. The worklet converts that trap into this code from
/// JavaScript (see the `.d.ts` header); nothing inside Rust catches it.
pub const RESULT_INTERNAL: u32 = 255;

/// Boot refused the staged document; diagnostics replaced its staged prefix.
pub const RESULT_REFUSED_DOCUMENT: u32 = RESULT_INVALID_ARGUMENT;
/// Boot refused the options structure.
pub const RESULT_REFUSED_OPTIONS: u32 = RESULT_ABI_MISMATCH;
/// Boot was attempted while a live handle already existed.
pub const RESULT_REFUSED_LIFECYCLE: u32 = RESULT_WRONG_STATE;

/// The reliable event lane of `miso_engine_web_v1_control_event` (issue #1570b D1): the C ABI's
/// `EVENT_LANE_RELIABLE`.
pub const EVENT_LANE_RELIABLE: u32 = 0;
/// The lossy telemetry lane of `miso_engine_web_v1_control_event` (issue #1570b D1): the C ABI's
/// `EVENT_LANE_LOSSY`.
pub const EVENT_LANE_LOSSY: u32 = 1;

/// A session and render plan are ready.
pub const STATE_READY: u32 = 2;
/// A sticky preparation or render failure occurred.
pub const STATE_FAILED: u32 = 3;
/// Ownership has been explicitly disposed.
pub const STATE_DISPOSED: u32 = 4;

/// Scalar backend: what a native (non-wasm) build reports; no wasm build is scalar (#1062).
pub const BACKEND_SCALAR: u32 = 0;
/// Base Wasm `simd128` backend.
pub const BACKEND_SIMD128: u32 = 1;

/// Source-ID staging buffer.
pub const BUFFER_SOURCE_ID: u32 = 2;
/// Planar source PCM staging buffer.
pub const BUFFER_SOURCE_PCM: u32 = 3;
/// Fixed diagnostic buffer.
pub const BUFFER_DIAGNOSTIC: u32 = 4;
/// Contiguous dual-mono output buffer.
pub const BUFFER_OUTPUT_PCM: u32 = 5;
/// Fixed live-control command staging buffer (issue #137 D1).
pub const BUFFER_COMMAND: u32 = 6;
/// Fixed decimated meter-frame buffer (issue #137 D2).
pub const BUFFER_METER_FRAME: u32 = 7;

/// Explicit stopped-response target kinds.
#[allow(missing_docs)]
pub const RESPONSE_TARGET_EFFECT: u32 = 1;
#[allow(missing_docs)]
pub const RESPONSE_TARGET_INPUT_FILTERS: u32 = 2;
/// Explicit frequency-axis kinds.
#[allow(missing_docs)]
pub const RESPONSE_GRID_LINEAR: u32 = 1;
#[allow(missing_docs)]
pub const RESPONSE_GRID_LOGARITHMIC: u32 = 2;
/// Response channel-selection bits.
#[allow(missing_docs)]
pub const RESPONSE_CHANNEL_LEFT: u32 = 1;
#[allow(missing_docs)]
pub const RESPONSE_CHANNEL_RIGHT: u32 = 2;
#[allow(missing_docs)]
pub const RESPONSE_CHANNEL_BOTH: u32 = RESPONSE_CHANNEL_LEFT | RESPONSE_CHANNEL_RIGHT;
/// Response field-selection bits.
#[allow(missing_docs)]
pub const RESPONSE_FIELD_TOTAL: u32 = 1;
#[allow(missing_docs)]
pub const RESPONSE_FIELD_SECTIONS: u32 = 2;
/// Maximum request-side variable payload retained by the analysis-only adapter.
#[allow(missing_docs)]
pub const RESPONSE_MAXIMUM_EFFECT_ID_BYTES: u32 = 127;
#[allow(missing_docs)]
pub const RESPONSE_MAXIMUM_PARAMETER_OVERRIDES: u32 = 256;
#[allow(missing_docs)]
pub const RESPONSE_MAXIMUM_RESULT_BYTES: u64 = 16 << 20;

/// Maximum number of owners a bounded live response query may return.
pub const LIVE_RESPONSE_MAXIMUM_OWNERS: usize = 256;
/// Maximum number of response sections retained for each owner channel.
pub const LIVE_RESPONSE_MAXIMUM_SECTIONS: usize =
    effect_contract::RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS;
/// Maximum bytes in one stable live-response identity.
pub const LIVE_RESPONSE_MAXIMUM_ID_BYTES: usize = 127;
/// Maximum points accepted by one live response query.
pub const LIVE_RESPONSE_MAXIMUM_POINTS: usize = 4096;
/// Fixed raw snapshot staging capacity, allocated before a browser caches Wasm views.
pub const LIVE_RESPONSE_CAPTURE_BYTES: usize = 1 << 20;
/// Live response result mode (`target`).
pub const LIVE_RESPONSE_MODE_TARGET: u32 = 1;
/// Live response result meaning (`eqFilterSubtotal`).
pub const LIVE_RESPONSE_MEANING_EQ_FILTER_SUBTOTAL: u32 = 1;

/// Spectrum capture target at the selected track's `post_input` tap, after its input section
/// (`trackPostInput`; the tap rename of decision 12, issue #1096, with the code unchanged).
pub const SPECTRUM_TARGET_TRACK_POST_INPUT: u32 = host_core::SPECTRUM_TARGET_TRACK_POST_INPUT;
/// Spectrum capture target at the selected track's `post_pan` tap, after its pan/matrix stage
/// (`trackPostPan`; the code is unchanged).
pub const SPECTRUM_TARGET_TRACK_POST_PAN: u32 = host_core::SPECTRUM_TARGET_TRACK_POST_PAN;
/// Spectrum capture target at the designated final output node.
pub const SPECTRUM_TARGET_OUTPUT: u32 = host_core::SPECTRUM_TARGET_OUTPUT;
/// Capture the left plane.
pub const SPECTRUM_CHANNEL_LEFT: u32 = host_core::SPECTRUM_CHANNEL_LEFT;
/// Capture the right plane.
pub const SPECTRUM_CHANNEL_RIGHT: u32 = host_core::SPECTRUM_CHANNEL_RIGHT;
/// Capture both planes.
pub const SPECTRUM_CHANNEL_BOTH: u32 = host_core::SPECTRUM_CHANNEL_BOTH;
/// Maximum stable target identity accepted by the browser bridge.
pub const SPECTRUM_MAXIMUM_ID_BYTES: usize = host_core::SPECTRUM_MAXIMUM_ID_BYTES;
/// Maximum one-shot raw spectrum payload retained by the browser bridge.
pub const SPECTRUM_CAPTURE_BYTES: usize = host_core::SPECTRUM_MAXIMUM_CAPTURE_BYTES as usize;
/// Compatibility metadata value for the prepared-target collection.
///
/// Zero means the collection is caller-sized and has no compiled target-count ceiling. The
/// bridge returns the checked capacity after the caller stages its entry count.
pub const SPECTRUM_MAXIMUM_PREPARED_TARGETS: u32 = 0;
/// Fixed one-shot spectrum output bin count.
pub const SPECTRUM_BIN_COUNT: u32 = host_core::SPECTRUM_BIN_COUNT as u32;
/// Fixed one-shot spectrum window length.
pub const SPECTRUM_WINDOW_FRAMES: u32 = host_core::SPECTRUM_WINDOW_FRAMES as u32;

/// A continuous spectrum stream has not been started on this prepared host.
pub const SPECTRUM_STREAM_STATUS_INACTIVE: u32 = 0;
/// A continuous stream is active but has not observed a complete first window.
pub const SPECTRUM_STREAM_STATUS_WARMING: u32 = 1;
/// A continuous stream is active and has no completed window available yet.
pub const SPECTRUM_STREAM_STATUS_PENDING: u32 = 2;
/// A bounded stream queue dropped one or more completed windows.
pub const SPECTRUM_STREAM_STATUS_GAP: u32 = 3;
/// A graph discontinuity or failed render reset the stream history.
pub const SPECTRUM_STREAM_STATUS_FAILED: u32 = 4;
/// A stream was stopped after a successful activation.
pub const SPECTRUM_STREAM_STATUS_STOPPED: u32 = 5;
/// A completed stream window and its metadata are available to the worker.
pub const SPECTRUM_STREAM_STATUS_READY: u32 = 6;

/// Preparation request staged before boot when the caller wants one spectrum boundary.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WebSpectrumRequest {
    pub struct_size: u32,
    pub abi_version: u32,
    pub target: u32,
    pub channels: u32,
    pub target_id_bytes: u32,
    pub reserved0: u32,
    pub maximum_capture_bytes: u64,
    pub reserved: [u32; 2],
}

/// Preparation header for several exact spectrum targets.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WebSpectrumCollectionRequest {
    pub struct_size: u32,
    pub abi_version: u32,
    pub entry_count: u32,
    pub reserved0: u32,
    pub maximum_capture_bytes: u64,
    pub reserved: [u32; 2],
}

/// One fixed-slot target entry in a collection request.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WebSpectrumCollectionEntry {
    pub target: u32,
    pub channels: u32,
    pub target_id_bytes: u32,
    pub reserved: [u32; 3],
}

/// Raw one-shot 2048-frame spectrum window header.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WebSpectrumWindow {
    pub struct_size: u32,
    pub abi_version: u32,
    pub target: u32,
    pub channels: u32,
    pub sample_rate_hz: u32,
    pub frames: u32,
    pub source_underrun: u32,
    pub reserved0: u32,
    pub captured_sample: u64,
    pub end_sample: u64,
    pub snapshot_token: u64,
    pub left_offset: u32,
    pub right_offset: u32,
}

/// Analyzed one-shot spectrum header followed by owned `f32` vectors.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WebSpectrumResult {
    pub struct_size: u32,
    pub abi_version: u32,
    pub result: u32,
    pub target: u32,
    pub channels: u32,
    pub sample_rate_hz: u32,
    pub window_frames: u32,
    pub bin_count: u32,
    pub source_underrun: u32,
    pub floor_db: f32,
    pub captured_sample: u64,
    pub end_sample: u64,
    pub snapshot_token: u64,
    pub result_bytes: u64,
    pub frequencies_offset: u32,
    pub left_offset: u32,
    pub right_offset: u32,
    pub reserved0: u32,
}

/// Self-describing metadata for the additive managed spectrum stream.
///
/// The one-shot [`WebSpectrumWindow`] and [`WebSpectrumResult`] records remain byte-for-byte
/// unchanged. This record carries the stream cadence, capture history and worker-side smoothing
/// history that cannot be inferred from a single raw window. The two reserved words make the
/// alignment before the first `u64` explicit rather than relying on C layout padding.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WebSpectrumStreamMetadata {
    pub struct_size: u32,
    pub abi_version: u32,
    pub result: u32,
    pub status: u32,
    pub target: u32,
    pub channels: u32,
    pub sample_rate_hz: u32,
    pub quantum_frames: u32,
    pub hop_frames: u32,
    pub source_underrun: u32,
    pub reserved0: u32,
    pub reserved1: u32,
    pub capture_epoch: u64,
    pub sequence: u64,
    pub dropped_captures: u64,
    pub windows: u64,
    pub captured_sample: u64,
    pub end_sample: u64,
    pub analysis_epoch: u64,
    pub history_start_sample: u64,
    pub smoothing_ms: f64,
}

/// Byte sizes of the fixed spectrum ABI records.
#[allow(missing_docs)]
pub const SPECTRUM_REQUEST_BYTES: u32 = size_of::<WebSpectrumRequest>() as u32;
#[allow(missing_docs)]
pub const SPECTRUM_WINDOW_HEADER_BYTES: u32 = size_of::<WebSpectrumWindow>() as u32;
#[allow(missing_docs)]
pub const SPECTRUM_RESULT_HEADER_BYTES: u32 = size_of::<WebSpectrumResult>() as u32;
/// Byte size of the additive continuous-spectrum metadata record.
#[allow(missing_docs)]
pub const SPECTRUM_STREAM_METADATA_BYTES: u32 = size_of::<WebSpectrumStreamMetadata>() as u32;
/// Byte sizes of the collection request records.
#[allow(missing_docs)]
pub const SPECTRUM_COLLECTION_REQUEST_BYTES: u32 = size_of::<WebSpectrumCollectionRequest>() as u32;
#[allow(missing_docs)]
pub const SPECTRUM_COLLECTION_ENTRY_BYTES: u32 = size_of::<WebSpectrumCollectionEntry>() as u32;
/// Zero denotes caller-sized collection entry staging.
#[allow(missing_docs)]
pub const SPECTRUM_COLLECTION_ENTRY_CAPACITY: u32 = SPECTRUM_MAXIMUM_PREPARED_TARGETS;
/// Zero denotes caller-sized packed target-ID staging.
#[allow(missing_docs)]
pub const SPECTRUM_COLLECTION_TARGET_IDS_BYTES: u32 = 0;

fn spectrum_channels_raw(channels: SpectrumChannels) -> u32 {
    channels as u32
}

/// The result word of a spectrum service refusal.
pub(crate) const fn spectrum_result(error: SpectrumServiceError) -> u32 {
    match error {
        SpectrumServiceError::Unprepared | SpectrumServiceError::SelectSingle => RESULT_UNSUPPORTED,
        SpectrumServiceError::Busy => RESULT_BACKPRESSURE,
        SpectrumServiceError::NotArmed => RESULT_WRONG_STATE,
        SpectrumServiceError::CaptureInvalid => RESULT_RENDER_REJECTED,
        SpectrumServiceError::Exhausted => RESULT_REFUSED_BUDGET,
        SpectrumServiceError::UnknownEntry
        | SpectrumServiceError::Channels
        | SpectrumServiceError::WindowInvalid => RESULT_INVALID_ARGUMENT,
    }
}

/// Fixed request header for a live selected-track response query.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WebLiveResponseRequest {
    pub struct_size: u32,
    pub abi_version: u32,
    pub track_id_bytes: u32,
    pub grid: u32,
    pub channels: u32,
    pub points: u32,
    pub minimum_hz: f32,
    pub maximum_hz: f32,
    pub maximum_result_bytes: u32,
    pub reserved: [u32; 3],
}

/// Fixed live response owner record in the returned bounded payload.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WebLiveResponseOwner {
    pub track_id_offset: u32,
    pub track_id_bytes: u32,
    pub native_id_offset: u32,
    pub native_id_bytes: u32,
    pub stable_id_offset: u32,
    pub stable_id_bytes: u32,
    /// [`LIVE_RESPONSE_RACK_INPUT_FILTERS`], [`LIVE_RESPONSE_RACK_INSERTS`] or
    /// [`LIVE_RESPONSE_RACK_CONSOLE`]; the retired `1` and `3` are refused (issue #1096).
    pub rack: u32,
    pub slot: u32,
    pub kind: u32,
    pub bypassed: u32,
    pub availability: u32,
    pub left_offset: u32,
    pub left_count: u32,
    pub right_offset: u32,
    pub right_count: u32,
    pub reserved: [u32; 1],
}

/// Fixed copied section record in a raw live response snapshot payload.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WebLiveResponseSection {
    pub id: u32,
    pub kind: u32,
    pub enabled: u32,
    pub word_count: u32,
    pub words: [u32; 7],
}

/// Fixed result header followed by owner records, identity bytes and selected `f32` vectors.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WebLiveResponseResult {
    pub struct_size: u32,
    pub abi_version: u32,
    pub result: u32,
    pub mode: u32,
    pub meaning: u32,
    pub channels: u32,
    pub points: u32,
    pub owner_count: u32,
    pub excluded_count: u32,
    pub sample_rate_hz: u32,
    pub reserved0: u32,
    pub reserved1: u32,
    pub captured_sample: u64,
    pub snapshot_token: u64,
    pub result_bytes: u64,
    pub frequencies_offset: u32,
    pub left_offset: u32,
    pub right_offset: u32,
    pub owners_offset: u32,
    pub owner_record_bytes: u32,
    pub section_record_bytes: u32,
    pub reserved: [u32; 2],
}

/// Byte sizes of the live response fixed records.
#[allow(missing_docs)]
pub const LIVE_RESPONSE_REQUEST_BYTES: u32 = size_of::<WebLiveResponseRequest>() as u32;
#[allow(missing_docs)]
pub const LIVE_RESPONSE_OWNER_BYTES: u32 = size_of::<WebLiveResponseOwner>() as u32;
#[allow(missing_docs)]
pub const LIVE_RESPONSE_SECTION_BYTES: u32 = size_of::<WebLiveResponseSection>() as u32;
#[allow(missing_docs)]
pub const LIVE_RESPONSE_RESULT_BYTES: u32 = size_of::<WebLiveResponseResult>() as u32;

/// Fixed request header for the analysis-only response adapter.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WebResponseRequest {
    pub struct_size: u32,
    pub abi_version: u32,
    pub target: u32,
    pub grid: u32,
    pub channels: u32,
    pub fields: u32,
    pub quality: u32,
    pub link_mode: u32,
    pub bypass: u32,
    pub effect_id_bytes: u32,
    pub parameter_count: u32,
    pub points: u32,
    pub sample_rate_hz: u32,
    pub quantum_frames: u32,
    pub minimum_hz: f32,
    pub maximum_hz: f32,
    pub left_hpf_hz: f32,
    pub left_lpf_hz: f32,
    pub right_hpf_hz: f32,
    pub right_lpf_hz: f32,
    pub configuration_id: u64,
    pub maximum_prepared_bytes: u64,
    pub maximum_total_state_bytes: u64,
    pub maximum_scratch_bytes: u64,
    pub maximum_automation_spans_per_block: u32,
    pub maximum_result_bytes: u32,
    pub reserved: [u32; 2],
}

/// One fixed-width numeric parameter override following the request header.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WebResponseParameter {
    pub parameter_id: u32,
    pub channel: u32,
    pub value: f32,
    pub reserved: u32,
}

/// Fixed result header followed by contiguous little-endian `f32` vectors.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WebResponseResult {
    pub struct_size: u32,
    pub abi_version: u32,
    pub result: u32,
    pub target: u32,
    pub channels: u32,
    pub fields: u32,
    pub points: u32,
    pub section_count: u32,
    pub sample_rate_hz: u32,
    pub reserved0: u32,
    pub configuration_id: u64,
    pub floor_db: f32,
    pub bypass: u32,
    pub enabled_left: u32,
    pub enabled_right: u32,
    pub retained_bytes: u64,
    pub result_bytes: u64,
    pub frequencies_offset: u32,
    pub total_left_offset: u32,
    pub total_right_offset: u32,
    pub sections_left_offset: u32,
    pub sections_right_offset: u32,
    pub reserved: [u32; 3],
}

#[allow(missing_docs)]
pub const RESPONSE_REQUEST_BYTES: u32 = size_of::<WebResponseRequest>() as u32;
#[allow(missing_docs)]
pub const RESPONSE_PARAMETER_BYTES: u32 = size_of::<WebResponseParameter>() as u32;
#[allow(missing_docs)]
pub const RESPONSE_RESULT_BYTES: u32 = size_of::<WebResponseResult>() as u32;

/// Exact byte size of one staged `miso.command.v1` record.
pub const COMMAND_RECORD_BYTES: u32 = 48;
/// Largest number of records one `miso.command.v1` submission may stage.
///
/// The staging buffer is allocated at preparation for exactly this many records, so a submission
/// can never grow the module's memory and the render path never sees an allocation.
pub const MAXIMUM_COMMAND_RECORDS: u32 = 256;
/// Largest `live_control_observation_taps` the browser configuration accepts.
///
/// The frame carries one gain-reduction slot per track, so a session cannot usefully bind more
/// taps per effect than a consumer can read; the cap keeps a mistyped configuration from asking
/// preparation for an unbounded menu. Every launch effect declares one tap.
pub const MAXIMUM_OBSERVATION_TAPS: u32 = 16;

/// Largest number of selected observation rows one bounded read may resolve.
///
/// The bound deliberately follows the existing command staging ceiling. A selected read is a
/// control-plane snapshot, not a subscription or a history, so the host never grows storage from
/// the number of renders or from the number of reads a caller makes.
pub const MAXIMUM_OBSERVATION_READS: usize = MAXIMUM_COMMAND_RECORDS as usize;

/// Requested left lane in a selected observation record.
pub const OBSERVATION_CHANNEL_LEFT: u32 = 1;
/// Requested right lane in a selected observation record.
pub const OBSERVATION_CHANNEL_RIGHT: u32 = 2;
/// Requested left and right lanes in a selected observation record.
pub const OBSERVATION_CHANNEL_BOTH: u32 = OBSERVATION_CHANNEL_LEFT | OBSERVATION_CHANNEL_RIGHT;
/// The selected tap has not published a complete window yet.
pub const OBSERVATION_STATUS_PENDING: u32 = 1;
/// The selected tap is currently disarmed.
pub const OBSERVATION_STATUS_UNARMED: u32 = 2;
/// The selected tap returned a complete resident window.
pub const OBSERVATION_STATUS_READY: u32 = 3;

/// Channels requested from one resident observation tap.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservationReadChannels {
    /// Return the owner-published left value.
    Left,
    /// Return the owner-published right value.
    Right,
    /// Return both owner-published lane values independently.
    Both,
}

/// Availability of one selected observation at the time of a read.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ObservationReadStatus {
    /// The tap is armed, but no complete window has been published yet.
    #[default]
    Pending,
    /// The tap exists, but its current admitted subscription is disarmed.
    Unarmed,
    /// A complete resident window was read without consuming it.
    Ready,
}

/// A typed refusal for one complete selected-observation batch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservationReadError {
    /// The host is not in the ready state.
    WrongState,
    /// A stable tuple, tap or batch shape is invalid for this host.
    InvalidSelection,
    /// The tuple names a declared target that was not prepared as a resident observation.
    Unsupported,
    /// The bounded selected-read ceiling was exceeded.
    BufferTooSmall,
}

/// One stable selection for [`AudioWorkletEngineHost::read_observations`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationSelection<'a> {
    /// Canonical normalized track identity.
    pub track_id: &'a str,
    /// The session rack containing the effect: a console slot or an insert (issue #1096).
    pub rack: LiveEffectRack,
    /// Stable local effect identity, never an array position: the slot ID of a console slot, the
    /// insert's ID of an insert.
    pub effect_slot_id: &'a str,
    /// Effect-local declared observation tap id.
    pub tap_id: u32,
    /// Independent owner-published lane values to return.
    pub channels: ObservationReadChannels,
}

/// Numeric owner address used by the Wasm bridge after a stable map has resolved a selection.
///
/// The address is deliberately not public identity: it is valid only for the current prepared
/// host and is re-resolved from the stable map by each SDK boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationAddress {
    /// Strip index: a track's canonical index, or `T + j` for submix `j` (issue #1213 D1).
    pub track_index: u32,
    /// The session rack (issue #1096).
    pub rack: LiveEffectRack,
    /// A console slot's index in the session's slot order, or an insert's index.
    pub effect_index: u32,
    /// Effect-local observation tap id.
    pub tap_id: u32,
    /// Requested owner-published lanes.
    pub channels: ObservationReadChannels,
}

/// One bound resident observation effect in the current prepared owner.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ObservationBinding<'a> {
    /// Numeric address fields used by the bridge.
    pub address: ObservationAddress,
    /// Stable local effect-slot identity.
    pub effect_slot_id: &'a str,
    /// Native effect contract identity.
    pub native_effect_id: &'static str,
    /// Existing owner descriptors, in declared tap order.
    pub descriptors: &'static [ObservationDescriptor],
}

/// Fixed selected-read input record shared with the Wasm boundary.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WebObservationSelection {
    /// Exact structure byte size.
    pub struct_size: u32,
    /// Browser ABI version.
    pub abi_version: u32,
    /// Strip index: canonical normalized tracks, then submixes (issue #1213, spelling P17).
    pub track_index: u32,
    /// [`RACK_INSERTS`] or [`RACK_CONSOLE`]; the retired `0` and `2` are refused.
    pub rack: u32,
    /// A console slot's index in the session's slot order, or an insert's index.
    pub effect_index: u32,
    /// Effect-local observation tap id.
    pub tap_id: u32,
    /// One of [`OBSERVATION_CHANNEL_LEFT`], [`OBSERVATION_CHANNEL_RIGHT`] or both.
    pub channels: u32,
    /// Required zero.
    pub reserved: u32,
}

/// Fixed selected-read result record shared with the Wasm boundary.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WebObservationResult {
    /// Exact structure byte size.
    pub struct_size: u32,
    /// Browser ABI version.
    pub abi_version: u32,
    /// One of the `OBSERVATION_STATUS_*` values.
    pub status: u32,
    /// Strip index: canonical normalized tracks, then submixes (issue #1213, spelling P17).
    pub track_index: u32,
    /// [`RACK_INSERTS`] or [`RACK_CONSOLE`]; the retired `0` and `2` are refused.
    pub rack: u32,
    /// A console slot's index in the session's slot order, or an insert's index.
    pub effect_index: u32,
    /// Effect-local observation tap id.
    pub tap_id: u32,
    /// Requested channel mask.
    pub channels: u32,
    /// Exact prepared sample rate.
    pub sample_rate_hz: u32,
    /// Required zero.
    pub reserved0: u32,
    /// Complete window's inclusive first sample, or zero without a window.
    pub first_sample: u64,
    /// Complete window's exclusive end sample, or zero without a window.
    pub end_sample: u64,
    /// Exact owner publication sequence, or zero without a window.
    pub sequence: u64,
    /// Render blocks folded into the complete window, or zero without a window.
    pub blocks: u32,
    /// Whether `left` is present for this requested channel mask.
    pub left_present: u32,
    /// Whether `right` is present for this requested channel mask.
    pub right_present: u32,
    /// Owner-published native-unit left value when present.
    pub left: f32,
    /// Owner-published native-unit right value when present.
    pub right: f32,
    /// Required zero expansion words.
    pub reserved: [u32; 3],
}

/// Byte size of [`WebObservationSelection`].
pub const OBSERVATION_SELECTION_BYTES: u32 = size_of::<WebObservationSelection>() as u32;
/// Byte size of [`WebObservationResult`].
pub const OBSERVATION_RESULT_BYTES: u32 = size_of::<WebObservationResult>() as u32;

/// One owned result row from a selected observation read.
#[derive(Clone, Debug, PartialEq)]
pub struct ObservationReadResult {
    /// The actual stable tuple resolved by the current prepared host.
    pub track_id: Box<str>,
    /// The resolved session rack.
    pub rack: LiveEffectRack,
    /// The actual stable local effect-slot identity.
    pub effect_slot_id: Box<str>,
    /// Native effect contract identity from the owner descriptor.
    pub native_effect_id: &'static str,
    /// The complete owner descriptor, including native and display units.
    pub descriptor: ObservationDescriptor,
    /// The requested channel selection.
    pub channels: ObservationReadChannels,
    /// The host session's exact prepared rate.
    pub sample_rate_hz: u32,
    /// Availability at the read boundary.
    pub status: ObservationReadStatus,
    /// The untouched latest complete window, when one is available.
    pub window: Option<engine::realtime::ObservationWindow>,
    /// Selected left value in the owner's declared native unit.
    pub left: Option<f32>,
    /// Selected right value in the owner's declared native unit.
    pub right: Option<f32>,
}

/// Allocation-free selected-read values for the Wasm staging bridge.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ObservationReadValues {
    /// Availability at the read boundary.
    pub status: ObservationReadStatus,
    /// The host session's exact prepared rate.
    pub sample_rate_hz: u32,
    /// The untouched latest complete window, when one is available.
    pub window: Option<ObservationWindow>,
    /// Selected left value in the owner's declared native unit.
    pub left: Option<f32>,
    /// Selected right value in the owner's declared native unit.
    pub right: Option<f32>,
}

/// Arm one declared observation tap of one effect instance (issue #143 D3).
///
/// `parameter_id` carries the effect-local `tap_id`, `smoothing_samples` carries the window length
/// in render blocks (`0` = the plan's default), and every value word must be `0.0`: a subscription
/// changes what is read, never what is rendered, so a nonzero value would be a caller mistake
/// rather than a meaningful field.
pub const COMMAND_OBSERVE_SUBSCRIBE: u32 = 7;
/// Disarm one declared observation tap of one effect instance (issue #143 D3).
pub const COMMAND_OBSERVE_UNSUBSCRIBE: u32 = 8;
/// The `rack` of an effect-addressed record: one of the track's inserts (decision 12, #1096).
///
/// It keeps the code the retired `dynamic` rack had. `effect_index` is the insert's position in
/// the track's `inserts`.
pub const RACK_INSERTS: u8 = 1;
/// The `rack` of an effect-addressed record: a session console slot (decision 12, #1096).
///
/// Appended. `effect_index` is the slot's position in the session's slot order, `pre_insert` then
/// `post_insert`, which is its index in the track's `console` entries.
///
/// The retired `simd1` and `simd2` codes, `0` and `2`, are refused with
/// [`COMMAND_REASON_UNKNOWN_RACK`] (or the observation and configuration paths' invalid-argument
/// result) and are never reallocated. `ABI_VERSION` did not move.
pub const RACK_CONSOLE: u8 = 3;
/// The `rack` of a builtin-addressed record, which names no effect rack.
pub const RACK_NOT_APPLICABLE: u8 = 255;

/// A live-response owner record's `rack`: the builtin input filters (issue #1096).
pub const LIVE_RESPONSE_RACK_INPUT_FILTERS: u32 = 0;
/// A live-response owner record's `rack`: one of the track's inserts. It keeps the internal code
/// the retired `dynamic` rack had.
pub const LIVE_RESPONSE_RACK_INSERTS: u32 = 2;
/// A live-response owner record's `rack`: a session console slot, appended. The retired `simd1`
/// and `simd2` codes, `1` and `3`, are refused and never reallocated.
pub const LIVE_RESPONSE_RACK_CONSOLE: u32 = 4;

/// Decode a record's `rack` and `effect_index` into a live effect address (issue #1096).
///
/// `None` for every other rack, the retired `0` and `2` among them.
pub(crate) fn live_effect_address(rack: u32, effect_index: u32) -> Option<LiveEffectAddress> {
    match u8::try_from(rack) {
        Ok(RACK_INSERTS) => Some(LiveEffectAddress::insert(effect_index)),
        Ok(RACK_CONSOLE) => Some(LiveEffectAddress::console(effect_index)),
        _ => None,
    }
}

/// The record `rack` of a live effect rack (issue #1096).
pub(crate) const fn live_effect_rack_raw(rack: LiveEffectRack) -> u8 {
    match rack {
        LiveEffectRack::Inserts => RACK_INSERTS,
        LiveEffectRack::Console => RACK_CONSOLE,
    }
}

/// The dense effect position of one live address, given its track's first position and its
/// lowered rack counts in chain order (issue #1096).
///
/// The one place a session address becomes a position in the effect band: preparation places
/// every producer with it and admission finds it with it, so the two cannot disagree. `None` when
/// the track has no such console slot or insert.
fn dense_effect_slot(base: u32, counts: [u32; 3], address: LiveEffectAddress) -> Option<usize> {
    let (rack, index) = address.lower(counts)?;
    let earlier = match rack {
        EffectRack::Simd1 => 0,
        EffectRack::Dynamic => counts[0],
        EffectRack::Simd2 => counts[0].checked_add(counts[1])?,
    };
    usize::try_from(base.checked_add(earlier)?.checked_add(index)?).ok()
}

/// The strip index of `id`: its position among the tracks, or `tracks.len() + j` for submix `j`
/// (issue #1207 D2).
///
/// host-core's strip list is the tracks in canonical ID order followed by the submixes in
/// canonical ID order. Each segment is sorted but their concatenation is not -- a submix ID may
/// sort before a track ID -- so the two segments are searched apart; a single binary search over
/// the concatenation would silently miss such a submix.
fn strip_index(tracks: &[Box<str>], submixes: &[Box<str>], id: &str) -> Option<usize> {
    if let Ok(track) = tracks.binary_search_by(|track| track.as_ref().cmp(id)) {
        return Some(track);
    }
    let submix = submixes
        .binary_search_by(|submix| submix.as_ref().cmp(id))
        .ok()?;
    tracks.len().checked_add(submix)
}

/// The submission was admitted whole.
pub const COMMAND_REASON_NONE: u32 = 0;
/// A record's fixed shape is wrong: an unknown kind, a nonzero reserved word, or a non-finite value;
/// or the submission names one observation tap more than once (#1345 D2).
pub const COMMAND_REASON_MALFORMED: u32 = 1;
/// `track_index` is not a track of the compiled session.
pub const COMMAND_REASON_UNKNOWN_TRACK: u32 = 2;
/// `rack` is not an effect rack: an effect-addressed kind names [`RACK_INSERTS`] or
/// [`RACK_CONSOLE`]. The retired `simd1` (`0`) and `simd2` (`2`) codes land here (issue #1096).
pub const COMMAND_REASON_UNKNOWN_RACK: u32 = 3;
/// `effect_index` is not an effect of the addressed rack.
pub const COMMAND_REASON_UNKNOWN_EFFECT: u32 = 4;
/// `parameter_id` is not a parameter of the addressed effect. No record raises it since #1565A
/// deleted the value kinds; the number stays declared and is never reallocated.
pub const COMMAND_REASON_UNKNOWN_PARAMETER: u32 = 5;
/// A value is outside the addressed parameter's declared domain. No record raises it since
/// #1565A deleted the value kinds (the core's `control.domain` is the one value check now); the
/// number stays declared and is never reallocated.
pub const COMMAND_REASON_DOMAIN: u32 = 6;
/// The submission cannot be applied by this host: a record on a host with no observation taps
/// attached (`live_control_observation_taps == 0`, so no command staging), or an observation tap
/// whose cost class is not resident.
///
/// This is not "malformed" and it is not "unknown target": the record is well formed and the tap
/// exists, and this session has no post-preparation write path for it.
pub const COMMAND_REASON_UNSUPPORTED_KIND: u32 = 7;
/// A bounded control queue had no room for the submission; nothing was admitted.
pub const COMMAND_REASON_BACKPRESSURE: u32 = 8;
/// The host is not `STATE_READY`.
pub const COMMAND_REASON_WRONG_STATE: u32 = 9;
/// `parameter_id` is not a declared observation tap of the addressed effect (issue #143).
///
/// Deliberately **not** `UNKNOWN_PARAMETER`: a parameter and a tap are different namespaces on the
/// same effect, and a caller that confuses them learns which one it got wrong.
pub const COMMAND_REASON_UNKNOWN_TAP: u32 = 10;
/// The tap exists and the address is right, but this effect instance holds no observation lane.
///
/// The honest form of "you asked for a subscription this preparation cannot deliver": the effect
/// is there and the tap is declared, but the plan bound no lane to arm for it. A session that
/// bound no observation capacity at all (`live_control_observation_taps == 0`) has no command
/// staging, and refuses every submission `COMMAND_REASON_UNSUPPORTED_KIND` before it reads a
/// record (#1565C); this reason is only reachable with capacity bound.
pub const COMMAND_REASON_OBSERVATION_UNBOUND: u32 = 11;
/// Default meter window in render blocks: ~31 frames per second at 48 kHz with a 128-frame quantum.
pub const DEFAULT_METER_BLOCKS: u32 = 12;

/// Exact versioned live-control command report shared with JavaScript (issue #137 D1).
///
/// One submission is one transaction: either every staged record was admitted, or none was and
/// `rejected_index`/`reason` name the first record that broke a rule. `applied_at_sample` is the
/// absolute sample the admitted records take effect at -- the first sample of the next rendered
/// block, because the matrix stage drains its queue at the top of the block before it touches a
/// single sample of audio.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WebCommandReport {
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

/// Byte size of [`WebCommandReport`].
pub const COMMAND_REPORT_BYTES: u32 = size_of::<WebCommandReport>() as u32;

/// The sample window and shape of the meter frame the `f32` buffer cannot carry (issue #143 D5).
///
/// # Why a second structure rather than more `f32`s
///
/// The meter frame is a `Float32Array` and the window it describes is a pair of absolute sample
/// counts. A `u64` does not survive an `f32`, and splitting one across two lanes would put a
/// decoding rule in the app that nothing could check. So the window rides a fixed structure the
/// JavaScript side reads through `miso_engine_web_v1_meter_header_ptr`, exactly as the status and
/// the resource report already do, and the `f32` buffer stays what it is: numbers a meter draws.
///
/// `first_sample`/`end_sample` are half-open and describe only the track and master peak window.
/// Gain-reduction slots are latest independently aged effect folds and do not inherit this span.
/// A consumer correlates the peak span against a command's `applied_at_sample` rather than against
/// a wall clock.
///
/// The gain-reduction section is recomputed from the taps armed when each frame is polled. An
/// unsubscribe disarms its tap when it is admitted, so every meter frame received after its
/// `result === 0` acknowledgement leaves that tap out. A subscribe, also one that re-arms an armed
/// tap, restarts its tap's window at the acknowledgement's `appliedAtSample`, and the frame leaves
/// that tap out until a window that opens at or after that sample closes. A strip's slot
/// (`trackGrDb`, `submixGrDb`) is the maximum over its armed taps' latest windows that opened at or
/// after their last subscribe, and `0` when no armed tap has such a window. `masterGrDb` is `null`
/// when the designated strip has none.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WebMeterHeader {
    /// Exact structure byte size.
    pub struct_size: u32,
    /// ABI version.
    pub abi_version: u32,
    /// Tracks the frame carries. With [`Self::submix_count`] it makes the `f32` view's
    /// `3(T + S) + 3` shape checkable.
    pub track_count: u32,
    /// Complete windows folded by the most recent poll.
    pub windows: u32,
    /// Absolute sample the reported window opened at, inclusive.
    pub first_sample: u64,
    /// Absolute sample the reported window closed at, exclusive.
    pub end_sample: u64,
    /// Monotonic frame sequence, incremented once per posted frame.
    pub sequence: u64,
    /// Designated master strip index (tracks first, then submixes) plus one, or `0` when none was
    /// designated (issue #143 D6; a submix can be the master since issue #1213 D5).
    pub master_track_plus_one: u32,
    /// `1` when `master_gr_db` is meaningful, `0` when no master tap is bound.
    pub master_gr_present: u32,
    /// Publication generation. It increments on each real lease transition and on a detected
    /// meter-producer reset, so retained frames cannot cross either delivery epoch.
    pub reserved: [u64; 2],
    /// Submix strips the frame carries after the tracks (issue #1209 D2), in canonical submix
    /// order. Appended in place, so every earlier offset is unmoved.
    pub submix_count: u32,
    /// Always zero; pads the structure to its eight-byte alignment.
    pub reserved_pad: u32,
}

/// `WebMeterHeader::reserved[1]` validity bits. The high 32 bits carry a saturating loss count.
pub const METER_VALID_COMPLETE: u64 = 1;
/// The master peak covers the exact published track interval.
pub const METER_VALID_MASTER: u64 = 1 << 1;
/// One or more queued windows were dropped or rejected before this publication.
pub const METER_VALID_LOSS: u64 = 1 << 2;
/// Gain-reduction values are present from the effect tap's own latest window; their timing is
/// intentionally independent and never supplies the track/master interval.
pub const METER_VALID_GAIN_REDUCTION: u64 = 1 << 3;
const METER_LOSS_SHIFT: u32 = 32;
/// Depth of every strip meter queue, track or submix, named once (issue #1448 D1): one window per
/// strip per post, plus headroom for a control-side stall of a few windows. The live-control
/// request builds each meter queue with it, and each poll caps a meter's count at entry by it.
const METER_QUEUE_DEPTH: usize = 8;

/// Byte size of [`WebMeterHeader`].
pub const METER_HEADER_BYTES: u32 = size_of::<WebMeterHeader>() as u32;

/// The header a handle with no compiled session reports: the zeroed shape, never a stale one.
static EMPTY_METER_HEADER: WebMeterHeader = empty_meter_header();

const fn empty_meter_header() -> WebMeterHeader {
    WebMeterHeader {
        struct_size: METER_HEADER_BYTES,
        abi_version: ABI_VERSION,
        track_count: 0,
        windows: 0,
        first_sample: 0,
        end_sample: 0,
        sequence: 0,
        master_track_plus_one: 0,
        master_gr_present: 0,
        reserved: [0; 2],
        submix_count: 0,
        reserved_pad: 0,
    }
}

/// Byte size of [`WebBootOptions`].
pub const BOOT_OPTIONS_BYTES: u32 = size_of::<WebBootOptions>() as u32;
/// Byte size of [`WebStatus`].
pub const STATUS_BYTES: u32 = size_of::<WebStatus>() as u32;
/// Byte size of [`WebResourceReport`].
pub const RESOURCE_REPORT_BYTES: u32 = size_of::<WebResourceReport>() as u32;

/// Exact versioned boot policy shared with JavaScript.
///
/// An all-zero value selects engine defaults. If either handshake word is nonzero, both must name
/// this exact layout and ABI version.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WebBootOptions {
    /// Zero for defaults, otherwise [`BOOT_OPTIONS_BYTES`].
    pub struct_size: u32,
    /// Zero for defaults, otherwise [`ABI_VERSION`].
    pub abi_version: u32,
    /// Required physical output sample rate, or zero to accept the document's rate.
    pub require_sample_rate_hz: u32,
    /// Required physical output quantum, or zero to accept the document's quantum.
    pub require_quantum_frames: u32,
    /// Per-source ring override, or zero for the engine's 100 ms derivation.
    pub source_ring_frames: u32,
    /// Must be zero.
    pub reserved0: u32,
    /// Total boot memory budget, or zero for [`DEFAULT_MAXIMUM_MEMORY_BYTES`].
    pub maximum_memory_bytes: u64,
    /// Must be zero. This was the live-command queue word until #1565 deleted the browser's own
    /// value commands; the offset keeps its place and the word is refused when nonzero
    /// (`web.options.reserved1`).
    pub reserved1: u64,
    /// Meter window in render blocks, or `0` to attach no meters at all (issue #137 D2).
    ///
    /// Zero is the honest form of "metering off costs nothing": no observer is bound, so the
    /// render path does not fold a single sample. A nonzero value binds one post-matrix meter per
    /// track with a `blocks * quantum_frames` window; the port lease then gates whether a finished
    /// window is posted. `12` is ~31 frames per second at 48 kHz with a 128-frame quantum.
    pub live_control_meter_blocks: u64,
    /// Maximum declared observation taps to bind per effect, or `0` for no observation capacity
    /// at all (issue #143 D3, level 1).
    ///
    /// A nonzero value allocates the command staging, which is what accepts the observation
    /// records (kinds 7 and 8): a subscription rides the effect's own observation queue. With zero,
    /// a command submission is `RESULT_UNSUPPORTED` with `COMMAND_REASON_UNSUPPORTED_KIND`.
    pub live_control_observation_taps: u64,
    /// The designated master strip index (tracks first, then submixes) **plus one**, or `0` for
    /// none (issue #143 D6). `T + j + 1` designates submix `j` (issue #1213 D5), so a mix bus
    /// with a limiter can report `masterGrDb`; a value past `T + S` refuses at boot.
    ///
    /// Boot v1 has no structural master bus, so `masterGrDb` is a designation rather than a discovery.
    /// Plus one because zero has to keep meaning "unset" in a word every V1 writer already zeroes.
    pub live_control_master_track_plus_one: u64,
}

impl WebBootOptions {
    /// Explicit handshake with every policy word left at its engine default.
    #[must_use]
    pub const fn explicit_defaults() -> Self {
        Self {
            struct_size: BOOT_OPTIONS_BYTES,
            abi_version: ABI_VERSION,
            require_sample_rate_hz: 0,
            require_quantum_frames: 0,
            source_ring_frames: 0,
            reserved0: 0,
            maximum_memory_bytes: 0,
            reserved1: 0,
            live_control_meter_blocks: 0,
            live_control_observation_taps: 0,
            live_control_master_track_plus_one: 0,
        }
    }

    /// Explicit defaults with the default meter window attached.
    #[must_use]
    pub const fn live_control_defaults() -> Self {
        Self {
            live_control_meter_blocks: DEFAULT_METER_BLOCKS as u64,
            ..Self::explicit_defaults()
        }
    }
}

/// Fixed browser-visible status snapshot.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WebStatus {
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
    /// The session's `live_values_superseded` count (decision 15, D15-2): the writes to live
    /// values that a later committed value replaced before the block that would have applied them,
    /// summed over every cell the session holds (a `Both` record counts in each channel cell).
    /// Loaded after every rendered block, so it is never behind the last one. It counts in the
    /// C ABI's unit (`COUNTERS_GET`): each read adds the writes it replaced. For one script of
    /// strip fader, mute and pan edits that stages at most one write per cell in a submission, it
    /// equals the C ABI's count. It does not for a script the two hosts stage differently: the
    /// browser also counts the extra writes one submission makes to one cell (send edits in wire
    /// order; a VCA ride on a member that also has a fader command), and the C ABI rebuilds,
    /// writing no cell, for input, route, VCA and submix edits.
    pub live_values_superseded: u64,
    /// Required-zero expansion words.
    pub reserved: [u64; 3],
}

/// The applied-revision watermark of the plan (#1381a D5), in the order and sizes of the C ABI's
/// `miso_engine_v1_watermark` (#1314 D6). It has no pre-roll counter (D15-8, round-5 amendment).
///
/// The host keeps one at a stable address, written by [`AudioWorkletEngineHost::read_watermark`]
/// and found through `miso_engine_web_v1_watermark_ptr`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WebPlanWatermark {
    /// Exact structure byte size ([`WATERMARK_BYTES`]).
    pub struct_size: u32,
    /// Required zero.
    pub reserved0: u32,
    /// The highest committed revision in effect together with every revision before it.
    pub revision: u64,
    /// The absolute render sample of the first block in which `revision` was in effect.
    pub first_sample: u64,
    /// The OR of the outcome flags over the revisions the last advance covered.
    pub outcome_flags: u64,
    /// Saturating count of revisions completed as exact.
    pub exact_count: u64,
    /// Saturating count of revisions completed as a transition fallback.
    pub transition_fallback_count: u64,
    /// Saturating count of revisions completed as superseded.
    pub superseded_count: u64,
    /// Required-zero expansion words.
    pub reserved: [u64; 5],
}

/// Exact byte size of [`WebPlanWatermark`], equal to the C ABI watermark's.
pub const WATERMARK_BYTES: u32 = size_of::<WebPlanWatermark>() as u32;

/// Exact retained-resource projection for the browser bridge and production artifacts.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WebResourceReport {
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
    /// Boot-options allocation bytes.
    pub options_bytes: u64,
    /// Status allocation bytes.
    pub status_bytes: u64,
    /// Session JSON staging bytes.
    pub session_document_bytes: u64,
    /// Diagnostic bytes.
    pub diagnostic_bytes: u64,
    /// Source/track-ID staging bytes.
    pub id_staging_bytes: u64,
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
    /// Engine-owned bytes the plan's observation lanes and conflating cells retain (issue #143).
    ///
    /// Carved out of the report's first reserved word. Exactly zero for a session prepared with
    /// `live_control_observation_taps == 0`, and that zero is *walked* over the built runtime
    /// rather than computed from the configuration. The 224-byte layout is unchanged.
    pub observation_retained_bytes: u64,
    /// Required-zero expansion words.
    pub reserved: [u64; 3],
}

struct PreparedBuffers {
    diagnostic: Box<[u8]>,
    source_id: Box<[u8]>,
    source_pcm: Box<[f32]>,
    output_pcm: Box<[f32]>,
    /// Fixed `MAXIMUM_COMMAND_RECORDS * COMMAND_RECORD_BYTES` observation-command staging (issue
    /// #137 D1); empty unless the boot asked for observation taps.
    command: Box<[u8]>,
    /// One control frame (issue #1562a D1): [`CONTROL_FRAME_BYTES`], the boot's
    /// `maximum_control_frame_bytes`. Staged by the page, read by the control plane in place.
    control_request: Box<[u8]>,
    plane_references: Box<[MaybeUninit<&'static [f32]>]>,
}

type FfiSourceStaging<'a> = (
    &'a mut [f32],
    &'a mut [MaybeUninit<&'static [f32]>],
    &'a mut [u8],
);

/// Spectrum observer preparation requested by the browser staging boundary: the core's
/// [`host_core::SpectrumPreparation`], under its earlier name.
pub use host_core::SpectrumPreparation as SpectrumPreparationRequest;

/// What the browser knows of one effect instance of the plan, in the dense `effect_slot` order
/// (#1381a): where its producer is in the control plane's list, what the instance declares, and,
/// when it declares a tap, where its reader set is in the attachment's [`RenderCompanions`].
///
/// The queries that name an effect read this table, and only the window reads reach the
/// attachment; an observe record reaches the instance's observation queue through `producer`.
struct EffectInstance {
    /// The position of this effect's producer in [`SessionState::effect_producers`].
    producer: u32,
    /// The position of this effect's [`EffectObservationHandle`] in `RenderCompanions`'s list, or
    /// `None` for an effect that declares no tap.
    reader: Option<u32>,
    /// Normalized identity of the strip this instance belongs to.
    track_id: Box<str>,
    /// The session-declared effect identifier.
    effect_id: Box<str>,
    /// The instance's live address within its strip.
    address: LiveEffectAddress,
    /// The effect's declared parameter table and observation menu.
    descriptor: &'static effect_contract::EffectDescriptor,
}

/// Everything one compiled session owns on the browser side.
///
/// Field order is the drop order and is load-bearing: the control plane's session drops first (its
/// producers, then the plan exchange's control ends and the retirement queue), and the compiled
/// session model outlives it. The render half (the plan owner, which owns the source consumers and
/// the active attachment) is the host's own `plan` field, declared after this one. Nothing here is
/// ever dropped from `render_next`; see [`AudioWorkletEngineHost::fail`].
struct ReadyOwnership {
    /// The control plane (#1381a D1): the committed model at its boot revision, the source, strip
    /// and effect producers, the plan's revision gate, the plan publisher and the retirer. It is
    /// the one owner of the producers; the browser holds none of its own.
    control: WebSession,
    /// The preparer's rebuild switch (#1562a D9), for the test-support republish.
    #[cfg(feature = "test-support")]
    allow_rebuild: Arc<AtomicBool>,
    /// The limits the control half was built with (#1570b D6), for the cross-host proof.
    #[cfg(feature = "test-support")]
    limits: ControlLimits,
    /// The any-thread projection of the plan: its resource row and the applied-revision watermark
    /// (#1381a D5).
    queries: PlanQueries<WebAdapter>,
    /// Whether the boot asked for observation taps (`live_control_observation_taps != 0`): the
    /// command staging exists and the observation records (kinds 7 and 8) are accepted only then.
    commands_enabled: bool,
    /// A handle on the session's `live_values_superseded` count (#1312 D2): the one count the
    /// control plane made, which every cell reader of every plan the session prepares adds to,
    /// and which the status reports (#1399).
    live_values_superseded: host_core::LiveValuesSuperseded,
    /// Every effect instance of the plan in the dense `effect_slot` order, whose length is the
    /// number of instances. It is how an observe record (kinds 7 and 8, ruling C9) reaches its
    /// effect's observation queue, and how the reads reach its readers.
    effect_instances: Box<[Option<EffectInstance>]>,
    /// Per-strip prefix sum of effect instances, so `effect_slot` is arithmetic, not a lookup:
    /// the tracks first, then the submixes (issue #1207 D3), so a track's entry is the same with
    /// or without submixes.
    effect_base: Box<[u32]>,
    /// Room needed per effect's observation queue by the submission being validated: the
    /// subscriptions it queues on each effect. Allocated at compilation, one entry per effect
    /// instance.
    command_wanted: Box<[u32]>,
    /// The observation taps the submission being validated has named, one bit per tap, one word per
    /// effect instance (a session binds at most [`MAXIMUM_OBSERVATION_TAPS`] taps per effect).
    /// An `Observe` record overwrites its tap's whole state at the block that drains it and one
    /// submission drains in one block, so a submission names each tap once (#1345 D2). Cleared with
    /// `command_wanted` at the top of every submission.
    observe_named: Box<[u32]>,
    /// Decoded submission staging: one entry per staged record, [`MAXIMUM_COMMAND_RECORDS`] long.
    command_decoded: Box<[StagedObserve]>,
    /// Observation subscriptions admitted per effect queue since the last successful render.
    ///
    /// The browser's control plane and render plane are the same thread and every live-control
    /// stage drains its whole observation queue at the top of every block, so a successful render
    /// empties every control queue. That is what makes this an exact free-slot count rather than
    /// an estimate, and it is what lets a submission be refused *before* anything is queued.
    in_flight: Box<[u32]>,
    /// Whether any subscription has been queued since the last successful render.
    ///
    /// This keeps command-free blocks from walking every destination counter. It is set at each
    /// successful push, so a partial internal failure cannot accidentally credit queue capacity.
    has_in_flight_commands: bool,
    /// Canonical normalized track order: the leading segment of the strip order that
    /// `track_index` addresses (issue #1213 D1).
    tracks: Vec<Box<str>>,
    /// Canonical normalized submix order, the strip order's trailing segment: submix `j` is strip
    /// `tracks.len() + j` (issue #1207 D2), which is the index word a command addresses it by
    /// (issue #1213 D1).
    submixes: Vec<Box<str>>,
    /// Effects per strip (tracks, then submixes) in each lowered rack, in chain order
    /// `[pre_insert, inserts, post_insert]`, so an effect-addressed command is answered with
    /// `UNKNOWN_EFFECT` before anything else ([`LiveEffectAddress::lower`] reads them).
    rack_effects: Box<[[u32; 3]]>,
    /// The optional one-shot spectrum service over its observer or atomically selectable
    /// collection. It is declared before the prepared host so consumers are released before the
    /// plan's producers.
    spectrum: Option<SpectrumService>,
    /// Issue #143: the meter consumers (issue #137 D2, one per selected meter, empty when
    /// `live_control_meter_blocks` was zero) and the observation reader sets are the active
    /// plan's [`RenderCompanions`]; `effect_instances` says where each effect's reader set is.
    /// Strip index of each observed effect slot (a submix's is `tracks.len() + j`), or `u32::MAX`
    /// for a slot with no taps. Built once at compilation, so the poll's fold is arithmetic rather
    /// than a search.
    observation_tracks: Box<[u32]>,
    /// Whether each strip contributed a gain-reduction reading to the most recent poll. Allocated
    /// at compilation, one entry per strip; it is what makes `masterGrDb` absent rather than zero.
    observation_present: Box<[bool]>,
    /// Armed-tap bitmask per effect slot, maintained at **admission**.
    ///
    /// A conflating cell holds its last window forever -- that is what makes it wait-free -- so
    /// "has this tap been unsubscribed" is not a question the transport can answer, and it must
    /// not be: a reader that treated an unconsumed window as absent would make an armed tap's
    /// reading flicker to zero on every poll between windows, exactly as the peak section would if
    /// it forgot its last value. The subscription state is control-plane state, so it lives on the
    /// control plane, updated by the one function that admits the record.
    observation_armed: Box<[u32]>,
    /// Admission sample at which each tap's current arm was acknowledged. A resident cell keeps
    /// its previous window forever, so a read is fresh for a re-arm only when the whole returned
    /// window starts at or after this sample. The nested index is the dense effect slot followed
    /// by that descriptor's tap index; zero means no command has changed the tap yet.
    observation_arm_samples: Box<[Box<[u64]>]>,
    /// The designated master strip index (tracks first, then submixes), or `None` (issue #143
    /// D6, issue #1213 D5).
    master_track: Option<u32>,
    /// `[track0 L, track0 R, .., trackN L, trackN R, bus0 L, bus0 R, .., busM L, busM R, master L,
    /// master R, track0 GR, .., trackN GR, bus0 GR, .., busM GR, master GR]` -- `3(T + S) + 3`
    /// words, one peak pair and one gain-reduction word per strip (issue #1209 D1). With `S = 0`
    /// every word is where it always was.
    meter_frame: Box<[f32]>,
    /// The window and shape the `f32` frame cannot carry (issue #143 D5).
    meter_header: WebMeterHeader,
    /// Master peaks folded over the rendered block while the lease is on.
    master_peak: [f32; 2],
    /// Half-open span covered by `master_peak`; retained across empty polls until a track span is
    /// complete, so a master impulse cannot disappear between track-window boundaries.
    master_start_sample: Option<u64>,
    master_end_sample: u64,
    /// Bounded master intervals, split at the same fixed period as track snapshots. This keeps a
    /// delayed poll from needing to union several master envelopes or relabel one as another.
    master_windows: Box<[Option<MasterMeasurement>]>,
    master_read: usize,
    master_write: usize,
    master_count: usize,
    /// One bounded pending snapshot per track. This is fixed at preparation and never grows from
    /// the callback; the queue itself remains the only producer-side buffer.
    meter_pending: Box<[Option<MeterSnapshot>]>,
    /// One count per strip meter (track or submix), read from its queue at each poll's entry and
    /// decremented before each pop (issue #1448 D2). Scratch for one call: every poll sets every
    /// entry before it pops, so delivery resets do not touch it. Allocated at preparation, sized
    /// by the meter count.
    meter_remaining: Box<[usize]>,
    /// Host publication generation, advanced on lease transitions and detected producer resets.
    meter_generation: u64,
    /// Losses and rejected stale/mismatched windows waiting to be reported in the next complete
    /// frame. Keeping this out of the public header until publication preserves empty-poll
    /// immutability.
    meter_loss_count: u64,
    meter_snapshot_drops_seen: u64,
    meter_snapshot_generation: Option<u64>,
    /// Windows folded into `meter_frame` since the host was compiled.
    meter_windows: u64,
    /// Test-only proof that incomplete master intervals never reach effect-reader scanning.
    #[cfg(test)]
    meter_effect_scan_count: u64,
}

#[derive(Clone, Copy)]
struct MasterMeasurement {
    start_sample: u64,
    end_sample: u64,
    peak: [f32; 2],
}

impl ReadyOwnership {
    /// The compiled session model, which the control plane retains (#1381a): read by the
    /// source-introspection queries (issue #207) and the addressing tables.
    ///
    /// The queries read it directly rather than copying a parallel table out of it, so the order
    /// they report *is* the normalized model's order by construction: `compile_session` sorts
    /// `sources` by stable ID, and there is no second list that could disagree with it.
    fn session(&self) -> &CompiledSession {
        self.control.committed_session()
    }

    /// Clear host-owned pending delivery state. Producer-side accumulators continue rendering;
    /// `meter_activation_sample` makes the already bounded poll reject queued pre-activation
    /// windows before it accepts a clean interval.
    fn reset_meter_delivery(&mut self, clear_publication: bool) {
        for pending in &mut self.meter_pending {
            *pending = None;
        }
        self.master_peak = [0.0, 0.0];
        self.master_start_sample = None;
        self.master_end_sample = 0;
        for window in &mut self.master_windows {
            *window = None;
        }
        self.master_read = 0;
        self.master_write = 0;
        self.master_count = 0;
        self.meter_snapshot_generation = None;
        self.meter_snapshot_drops_seen = 0;
        self.meter_loss_count = 0;
        if clear_publication {
            // Keep the callback-reachable reset free of slice-fill's bounds-panic owner in the
            // shipped Wasm call graph. The iterator walks the already prepared frame exactly once.
            for slot in &mut self.meter_frame {
                *slot = 0.0;
            }
            self.meter_header.sequence = 0;
            self.meter_header.windows = 0;
            self.meter_header.first_sample = 0;
            self.meter_header.end_sample = 0;
            self.meter_header.master_gr_present = 0;
            self.meter_header.reserved[1] = 0;
            self.meter_windows = 0;
        }
    }

    fn push_master_window(&mut self, measurement: MasterMeasurement) {
        if self.master_count == self.master_windows.len() {
            // Drop-newest mirrors the engine meter queue contract. The loss is visible in the
            // next successful browser snapshot through `reserved[1]`.
            self.meter_loss_count = self.meter_loss_count.saturating_add(1);
            return;
        }
        if let Some(slot) = self.master_windows.get_mut(self.master_write) {
            *slot = Some(measurement);
            self.master_write = (self.master_write + 1) % self.master_windows.len();
            self.master_count += 1;
        }
    }

    fn pop_master_window(&mut self) -> Option<MasterMeasurement> {
        if self.master_count == 0 {
            return None;
        }
        let value = self.master_windows.get_mut(self.master_read)?.take();
        self.master_read = (self.master_read + 1) % self.master_windows.len();
        self.master_count -= 1;
        value
    }

    /// Strips these controls address: the tracks, then the submixes (issue #1213 D1). An
    /// observation record's `track_index` is a strip index, so a bus is addressed as a track is.
    fn strip_count(&self) -> usize {
        self.tracks.len() + self.submixes.len()
    }

    /// The dense index of one addressed effect instance in `effect_instances`.
    ///
    /// An effect is addressed in the session's terms, a console slot by its slot index or an
    /// insert by its index (issue #1096); [`LiveEffectAddress::lower`] is the one translation to
    /// the lowered rack that orders the band. `None` means the address names no instance this
    /// session prepared.
    fn effect_slot(&self, track: usize, address: LiveEffectAddress) -> Option<usize> {
        let slot = dense_effect_slot(
            *self.effect_base.get(track)?,
            *self.rack_effects.get(track)?,
            address,
        )?;
        (slot < self.effect_instances.len()).then_some(slot)
    }

    /// The position, in the control plane's producer list, of the effect at dense slot `effect`
    /// (#1381a): the one way the observation records reach an effect's observation queue (ruling
    /// C9).
    fn effect_producer_position(&self, effect: usize) -> Option<usize> {
        let position = self.effect_instances.get(effect)?.as_ref()?.producer;
        Some(position as usize)
    }

    /// The free room of one effect's observation queue (one slot per declared tap, #1345 D2).
    fn queue_available(&self, effect: usize) -> Option<u32> {
        let position = self.effect_producer_position(effect)?;
        let producer = self.control.effect_producers().get(position)?;
        u32::try_from(producer.observe_available_capacity()).ok()
    }

    /// Queue one admitted observation subscription on its effect. `Err` only on a full queue,
    /// which the free-room pass has already ruled out.
    fn push_observe(&mut self, staged: StagedObserve, applied_at_sample: u64) -> Result<(), ()> {
        let effect = staged.effect as usize;
        // Issue #143: the armed set is control-plane state and is updated here, where the record
        // is admitted, so it can never disagree with what the render side was told.
        let position = self.effect_producer_position(effect).ok_or(())?;
        self.control
            .push_effect_observe(
                position,
                staged.tap_index,
                staged.armed,
                staged.window_blocks,
            )
            .ok_or(())?
            .map_err(|_| ())?;
        if let Some(mask) = self.observation_armed.get_mut(effect)
            && staged.tap_index < u32::BITS
        {
            let bit = 1_u32 << staged.tap_index;
            *mask = if staged.armed {
                *mask | bit
            } else {
                *mask & !bit
            };
            if let Some(samples) = self.observation_arm_samples.get_mut(effect)
                && let Some(sample) = samples.get_mut(staged.tap_index as usize)
            {
                *sample = applied_at_sample;
            }
        }
        Ok(())
    }
}

/// One decoded and validated observation record (kinds 7 and 8), in the order of its submission.
#[derive(Clone, Copy)]
struct StagedObserve {
    /// The dense effect index the record addresses.
    effect: u32,
    /// The wire index of the record, which names it in a refusal.
    original_wire_index: u32,
    /// The effect-local tap index the render side arms.
    tap_index: u32,
    /// Whether the record arms (kind 7) or disarms (kind 8) the tap.
    armed: bool,
    /// The window length in render blocks (`0` = the plan's default).
    window_blocks: u32,
}

/// One compiled source's declared shape, in canonical normalized source order (issue #207).
///
/// The strict JSON declares one channel count and full-source frame count; sample rate is solely a
/// session-root fact and every prepared source begins at frame zero.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionSourceShape {
    /// Declared source channels; nonzero for every source a compiled session holds.
    pub channel_count: u32,
    /// Exact source length in sample frames; nonzero for every compiled source.
    pub frames: u64,
}

/// Safe ownership object backing one future AudioWorklet Wasm handle.
pub struct AudioWorkletEngineHost {
    options: WebBootOptions,
    status: WebStatus,
    resources: WebResourceReport,
    command_report: WebCommandReport,
    /// Issue #137 D2: the meter lease. `false` skips the master fold and every drain.
    meter_lease: bool,
    /// The watermark `read_watermark` last copied (#1381a D5), at a stable address in the
    /// control half: all zero until the first successful read.
    watermark: WebPlanWatermark,
    /// First sample eligible for the current lease. A snapshot that opens before this boundary
    /// is an old/partial interval and is dropped rather than presented as a clean measurement.
    meter_activation_sample: u64,
    buffers: Option<PreparedBuffers>,
    ready: Option<ReadyOwnership>,
    /// The render half of the plan exchange (#1381a D4): the plan owner that `render_next` renders
    /// through, and whose active plan's attachment holds the meter consumers and the observation
    /// readers. It is the render side of the single instance, beside the control half in `ready`,
    /// so the two borrow apart. Dropped, with `ready`, only by [`Self::dispose`].
    plan: Option<WebPlan>,
    /// Optional prepared cadence for the ordinary spectrum capture.
    spectrum_hop: Option<SpectrumHop>,
    diagnostic_len: usize,
    /// What the last control command left to read (issue #1562a D1).
    control_response: ControlResponse,
}

/// What [`AudioWorkletEngineHost::control_response`] reads (issue #1562a D1).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ControlResponse {
    /// Nothing: no control command has run, or the last one was refused before it ran.
    None,
    /// The control plane's response frame of this length, in its response scratch.
    Frame(usize),
    /// The refusal's diagnostic, `diagnostic_len` bytes of the diagnostic buffer.
    Diagnostic,
}

impl AudioWorkletEngineHost {
    /// Parse, validate, self-configure, project, prepare and publish one running host.
    ///
    /// The document is interpreted once. Every refusal is typed and nothing is published until
    /// both the shared runtime and all bridge staging buffers exist.
    pub fn boot(document: &[u8], options: WebBootOptions) -> Result<Self, BootFailure> {
        Self::boot_with_spectrum(document, options, None)
    }

    /// Prepare one optional graph spectrum observer alongside the existing live-control/meter plan.
    ///
    /// The request is supplied before boot because observer bindings are part of the immutable
    /// prepared graph. A replacement/reboot creates a fresh request and a fresh capture owner.
    pub(crate) fn boot_with_spectrum(
        document: &[u8],
        options: WebBootOptions,
        spectrum_request: Option<SpectrumPreparationRequest>,
    ) -> Result<Self, BootFailure> {
        Self::boot_with_spectrum_config(document, options, spectrum_request, None)
    }

    pub(crate) fn boot_with_spectrum_config(
        document: &[u8],
        options: WebBootOptions,
        spectrum_request: Option<SpectrumPreparationRequest>,
        spectrum_hop: Option<SpectrumHop>,
    ) -> Result<Self, BootFailure> {
        let document_bytes = u32::try_from(document.len()).map_err(|_| {
            BootFailure::fixed(RESULT_REFUSED_DOCUMENT, "web.document.maximum_bytes")
        })?;
        if document_bytes > MAXIMUM_DOCUMENT_BYTES {
            return Err(BootFailure::fixed(
                RESULT_REFUSED_DOCUMENT,
                "web.document.maximum_bytes",
            ));
        }
        let options = validate_options(options)?;
        let memory_budget = if options.maximum_memory_bytes == 0 {
            DEFAULT_MAXIMUM_MEMORY_BYTES
        } else {
            options.maximum_memory_bytes
        };
        let parse_projection = u64::from(document_bytes)
            .checked_mul(PARSE_TRANSIENT_MULTIPLIER)
            .ok_or_else(|| BootFailure::fixed(RESULT_REFUSED_BUDGET, "host.budget.arithmetic"))?;
        if parse_projection > memory_budget {
            return Err(BootFailure::projected_budget(
                "host.budget.parse_projection",
                parse_projection,
                memory_budget,
            ));
        }
        let document = core::str::from_utf8(document)
            .map_err(|_| BootFailure::fixed(RESULT_REFUSED_DOCUMENT, "web.document.utf8"))?;
        let model = parse_host_session(document)
            .map_err(|failure| BootFailure::document(failure.into_bytes()))?;
        let compile_caps = CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        };
        let session = compile_host_model(&model, compile_caps)
            .map_err(|failure| BootFailure::document(failure.into_bytes()))?;
        let shape = compiled_session_shape(&session)
            .map_err(|failure| BootFailure::document(failure.into_bytes()))?;
        if (options.require_sample_rate_hz != 0
            && options.require_sample_rate_hz != shape.sample_rate_hz)
            || (options.require_quantum_frames != 0
                && options.require_quantum_frames != shape.quantum_frames)
        {
            return Err(BootFailure::fixed(
                RESULT_REPREPARE_REQUIRED,
                "host.session.shape",
            ));
        }
        let source_ring_frames = if options.source_ring_frames == 0 {
            default_source_ring_frames(shape.sample_rate_hz, shape.quantum_frames)
        } else {
            options.source_ring_frames
        };
        if source_ring_frames < shape.quantum_frames
            || !source_ring_frames.is_multiple_of(shape.quantum_frames)
        {
            return Err(BootFailure::fixed(
                RESULT_REFUSED_OPTIONS,
                "web.options.source_ring_frames",
            ));
        }
        let (spectrum_collection_entry_bytes, spectrum_collection_target_id_bytes) =
            spectrum_collection_staging_bytes(spectrum_request.as_ref())?;
        let projection = project_buffers(
            document_bytes,
            shape.sample_rate_hz,
            shape.quantum_frames,
            shape.maximum_source_channels,
            shape
                .longest_source_id_bytes
                .max(shape.longest_track_id_bytes)
                .max(shape.longest_submix_id_bytes),
            options,
            (
                spectrum_request.is_some(),
                (
                    spectrum_collection_entry_bytes,
                    spectrum_collection_target_id_bytes,
                ),
            ),
        )?;
        let retained_projection = projected_retained_bytes(
            &session,
            source_ring_frames,
            projection.report.bridge_retained_bytes,
        )?;
        if retained_projection > memory_budget {
            return Err(BootFailure::projected_budget(
                "host.budget.retained_projection",
                retained_projection,
                memory_budget,
            ));
        }
        let caps = prepare_caps(&session, options, source_ring_frames, memory_budget);
        // The control plane compiles the session again and retains the one model the host reads
        // (#1381a). What this function holds of its own is freed first, so the boot's transient
        // peak is the one compile's, as it was before.
        drop(session);
        drop(model);
        let (ready, plan, resources) = compile_ready(
            document,
            &caps,
            options,
            memory_budget,
            projection.report,
            spectrum_request.as_ref(),
        )?;
        let exact_retained = exact_retained_bytes(&resources)?;
        if exact_retained > memory_budget {
            return Err(BootFailure::exact_budget(
                "host.budget.retained_exact",
                exact_retained,
                memory_budget,
            ));
        }
        let buffers = allocate_buffers(projection)?;
        let backend = selected_backend();
        Ok(Self {
            options,
            status: WebStatus {
                struct_size: STATUS_BYTES,
                abi_version: ABI_VERSION,
                state: STATE_READY,
                last_result: RESULT_OK,
                backend,
                sample_rate_hz: shape.sample_rate_hz,
                quantum_frames: shape.quantum_frames,
                reserved0: 0,
                next_absolute_sample: 0,
                rendered_quanta: 0,
                live_values_superseded: 0,
                reserved: [0; 3],
            },
            resources,
            command_report: empty_command_report(),
            watermark: WebPlanWatermark {
                struct_size: WATERMARK_BYTES,
                ..WebPlanWatermark::default()
            },
            meter_lease: false,
            meter_activation_sample: 0,
            buffers: Some(buffers),
            ready: Some(ready),
            plan: Some(plan),
            spectrum_hop: spectrum_request.as_ref().and(spectrum_hop),
            diagnostic_len: 0,
            control_response: ControlResponse::None,
        })
    }

    /// Read the immutable boot options.
    #[must_use]
    pub const fn options(&self) -> &WebBootOptions {
        &self.options
    }

    /// Read fixed status without allocation.
    #[must_use]
    pub const fn status(&self) -> &WebStatus {
        &self.status
    }

    /// Read the current exact resource projection.
    #[must_use]
    pub const fn resources(&self) -> &WebResourceReport {
        &self.resources
    }

    /// The active plan's companions, for the tests that read the consumers and readers a plan
    /// carries (#1381a).
    #[cfg(test)]
    pub(crate) fn test_companions(&mut self) -> &mut RenderCompanions {
        self.plan
            .as_mut()
            .and_then(|plan| plan.active_attachment_mut::<RenderCompanions>())
            .expect("a ready host's plan carries its companions")
    }

    /// The observation handle of the effect at dense slot `effect`, from the active plan's
    /// companions (#1381a).
    #[cfg(test)]
    pub(crate) fn test_observation_handle(
        &mut self,
        effect: usize,
    ) -> Option<&EffectObservationHandle> {
        let reader = self
            .ready
            .as_ref()?
            .effect_instances
            .get(effect)?
            .as_ref()?
            .reader?;
        self.test_companions()
            .effect_observations
            .get(reader as usize)
    }

    /// Run one structural command frame through the control plane (#1381a gates 2 and 4): it
    /// prepares a successor of the committed model through the browser's preparer and publishes
    /// it, for the render of the next block to adopt. No export reaches it, and the production
    /// host has no structural edit of its own until #1290.
    ///
    /// Returns the control plane's response frame.
    ///
    /// # Errors
    ///
    /// The control plane's refusal of the frame.
    #[cfg(feature = "test-support")]
    pub fn structural_command_for_test(
        &mut self,
        request: &[u8],
    ) -> Result<Vec<u8>, control_plane::CommandError> {
        let ready = self
            .ready
            .as_mut()
            .ok_or(control_plane::CommandError::Internal)?;
        ready.allow_rebuild.store(true, Ordering::Release);
        let length = ready.control.command(request, CONTROL_FRAME_BYTES);
        ready.allow_rebuild.store(false, Ordering::Release);
        Ok(ready.control.command_response(length?).to_vec())
    }

    /// The [`ControlLimits`] the control half was built with (#1570b D6), read-only: the cross-host
    /// proof builds the C ABI's compile limits from it field for field, so both hosts advertise the
    /// same capabilities.
    #[cfg(feature = "test-support")]
    #[must_use]
    pub fn control_limits_for_test(&self) -> Option<ControlLimits> {
        self.ready.as_ref().map(|ready| ready.limits)
    }

    /// The epoch of the plan that renders (#1381a gates 2 and 4): 0 for the boot plan, and one
    /// more for each published plan the owner adopted.
    #[cfg(feature = "test-support")]
    #[must_use]
    pub fn active_plan_epoch_for_test(&self) -> Option<u64> {
        self.plan.as_ref().map(|plan| plan.owner().active_epoch().0)
    }

    /// The staging buffer for one control command frame of `len` bytes (issue #1562a D1), or
    /// `None` when `len` is zero or exceeds the boot's `maximum_control_frame_bytes`
    /// (16 KiB) or the host is not ready. The page writes the frame here and
    /// calls [`Self::control_command`].
    pub fn control_request_mut(&mut self, len: u32) -> Option<&mut [u8]> {
        if self.status.state != STATE_READY || len == 0 || u64::from(len) > CONTROL_FRAME_BYTES {
            return None;
        }
        self.buffers
            .as_mut()
            .map(|buffers| &mut *buffers.control_request)
    }

    /// Run the staged control command frame of `len` bytes through the control plane (issue
    /// #1562a D1): the same [`control_plane::SessionState::command`] the C ABI's
    /// `miso_engine_v1_submit_command` runs, so any frame it accepts is accepted here: a
    /// transaction (live, model-only or monitor), the control list and state queries, the
    /// snapshot, counters and capabilities. The browser decodes nothing in the frame.
    ///
    /// This is control work: the worklet's message handler in `single` mode, never `process()`.
    /// Its allocations are the instance's counted control allocations. It does not touch the
    /// status's sticky state: a refusal leaves the host ready and the model, revision, overlay and
    /// cells as they were (D4, the acked-batch rule is the core's).
    ///
    /// Results: [`RESULT_OK`], with the response frame at [`Self::control_response`] (a refused
    /// transaction is still [`RESULT_OK`] with a non-ok response frame, as on the C ABI);
    /// [`RESULT_WRONG_STATE`] for a host that is not ready; [`RESULT_INVALID_ARGUMENT`] for a
    /// length that is zero or over the frame ceiling, or a frame the control plane cannot
    /// correlate; [`RESULT_BUFFER_TOO_SMALL`] for a response over the control frame;
    /// [`RESULT_BACKPRESSURE`] for a queue or plan with no room now; [`RESULT_COMPILE_REJECTED`]
    /// for a typed refusal before commit, `web.apply.rebuild_pending` for a transaction that needs
    /// a plan rebuild (D9); [`RESULT_INTERNAL`] for a bookkeeping failure. For each failure the
    /// response is the refusal's diagnostic (`code\tpath\n` lines).
    pub fn control_command(&mut self, len: u32) -> u32 {
        self.control_response = ControlResponse::None;
        if self.status.state != STATE_READY {
            return self.record(RESULT_WRONG_STATE);
        }
        if len == 0 || u64::from(len) > CONTROL_FRAME_BYTES {
            return self.record(RESULT_INVALID_ARGUMENT);
        }
        let Some((buffers, ready)) = self.buffers.as_mut().zip(self.ready.as_mut()) else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        let request = &buffers.control_request[..len as usize];
        match ready.control.command(request, CONTROL_FRAME_BYTES) {
            Ok(length) => {
                self.control_response = ControlResponse::Frame(length);
                self.record(RESULT_OK)
            }
            Err(error) => {
                let diagnostic = error.diagnostic();
                let kept = diagnostic.len().min(buffers.diagnostic.len());
                buffers.diagnostic[..kept].copy_from_slice(&diagnostic[..kept]);
                self.diagnostic_len = kept;
                self.control_response = ControlResponse::Diagnostic;
                self.record(match error {
                    control_plane::CommandError::Invalid => RESULT_INVALID_ARGUMENT,
                    control_plane::CommandError::BufferTooSmall { .. } => RESULT_BUFFER_TOO_SMALL,
                    control_plane::CommandError::Backpressure => RESULT_BACKPRESSURE,
                    control_plane::CommandError::CompileRejected(_) => RESULT_COMPILE_REJECTED,
                    control_plane::CommandError::Internal => RESULT_INTERNAL,
                })
            }
        }
    }

    /// Dequeue the next event of `lane` (issue #1570b D1): `0` is the reliable lane, `1` the lossy
    /// lane, the C ABI's codes. This is the browser form of `miso_engine_v1_dequeue_event`: the
    /// same [`control_plane::SessionState::dequeue_event`], with the whole response scratch as
    /// capacity. The frame is then at [`Self::control_response`], which shares the staging of
    /// [`Self::control_command`] and is valid until the next control call. An empty lane leaves it
    /// empty (zero bytes).
    ///
    /// Control work, like the command entry: the worklet's message handler in `single` mode, never
    /// `process()`. Like [`Self::service`] it leaves the status's last result alone, so a drain
    /// never overwrites a refusal the host has yet to read.
    ///
    /// Results: [`RESULT_OK`]; [`RESULT_INVALID_ARGUMENT`] for a lane other than `0` or `1`, which
    /// dequeues nothing; [`RESULT_WRONG_STATE`] for a host that is not ready (and so
    /// [`RESULT_REFUSED_LIFECYCLE`], the same code, for an instance with no control half);
    /// [`RESULT_BACKPRESSURE`], [`RESULT_BUFFER_TOO_SMALL`] and [`RESULT_INTERNAL`] for the
    /// matching [`control_plane::EventError`]. A refusal leaves the response empty and writes the
    /// core's text for it ([`control_plane::EventError::diagnostic`]) into the session diagnostic.
    pub fn control_event(&mut self, lane: u32) -> u32 {
        self.control_response = ControlResponse::None;
        let lane = match lane {
            EVENT_LANE_RELIABLE => control_plane::EventLane::Reliable,
            EVENT_LANE_LOSSY => control_plane::EventLane::Lossy,
            _ => return RESULT_INVALID_ARGUMENT,
        };
        if self.status.state != STATE_READY {
            return RESULT_REFUSED_LIFECYCLE;
        }
        let Some((buffers, ready)) = self.buffers.as_mut().zip(self.ready.as_mut()) else {
            return RESULT_REFUSED_LIFECYCLE;
        };
        match ready.control.dequeue_event(lane, CONTROL_FRAME_BYTES) {
            Ok(Some(length)) => {
                self.control_response = ControlResponse::Frame(length);
                RESULT_OK
            }
            Ok(None) => RESULT_OK,
            Err(error) => {
                let diagnostic = error.diagnostic();
                let kept = diagnostic.len().min(buffers.diagnostic.len());
                buffers.diagnostic[..kept].copy_from_slice(&diagnostic[..kept]);
                self.diagnostic_len = kept;
                match error {
                    control_plane::EventError::BufferTooSmall { .. } => RESULT_BUFFER_TOO_SMALL,
                    control_plane::EventError::Backpressure => RESULT_BACKPRESSURE,
                    control_plane::EventError::Internal => RESULT_INTERNAL,
                }
            }
        }
    }

    /// What the last [`Self::control_command`] left to read (issue #1562a D1): its response frame
    /// after a [`RESULT_OK`], the refusal's diagnostic after another result, empty before any
    /// command. Valid until the next control call.
    #[must_use]
    pub fn control_response(&self) -> &[u8] {
        match (
            self.control_response,
            self.ready.as_ref(),
            self.buffers.as_ref(),
        ) {
            (ControlResponse::Frame(length), Some(ready), _) => {
                ready.control.command_response(length)
            }
            (ControlResponse::Diagnostic, _, Some(buffers)) => {
                &buffers.diagnostic[..self.diagnostic_len]
            }
            _ => &[],
        }
    }

    /// The committed session revision (issue #1562a, carry-forward 2): what the SDK's transaction
    /// queue starts its expected revision from after a successful boot. It advances by one for
    /// each transaction the control plane commits, whatever its path, and is `0` for a session
    /// that was never edited and for a host that is not ready.
    #[must_use]
    pub fn committed_revision(&self) -> u64 {
        self.ready.as_ref().map_or(0, |ready| {
            ready
                .control
                .committed_session()
                .normalized_model()
                .revision
        })
    }

    /// Run the control plane's service step once (#1381a D5, #1348 D1): bring the provider epochs
    /// up to render and reclaim every retired plan and its attachment, here, never on the render
    /// thread. It commits and acknowledges nothing, and it leaves the status's last result alone,
    /// so a periodic tick never overwrites a refusal the host has yet to read.
    ///
    /// An epoch failure is sticky: the host fails with the session's diagnostic.
    pub fn service(&mut self) -> u32 {
        if self.status.state != STATE_READY {
            return RESULT_WRONG_STATE;
        }
        let Some(ready) = self.ready.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        match ready.control.service() {
            Ok(()) => RESULT_OK,
            Err(error) => {
                let diagnostic = error.diagnostic().into_owned();
                self.fail(RESULT_INTERNAL, &diagnostic)
            }
        }
    }

    /// Copy the plan's applied-revision watermark into the host's record (#1381a D5).
    ///
    /// A read that gives up after the watermark's bounded attempts returns
    /// [`RESULT_BACKPRESSURE`] and leaves the record untouched: a read to retry, never a refused
    /// edit. Like [`Self::service`] it leaves the status's last result alone.
    pub fn read_watermark(&mut self) -> u32 {
        if self.status.state != STATE_READY {
            return RESULT_WRONG_STATE;
        }
        let Some(ready) = self.ready.as_ref() else {
            return RESULT_WRONG_STATE;
        };
        let Ok(watermark) = ready.queries.watermark() else {
            return RESULT_BACKPRESSURE;
        };
        self.watermark = WebPlanWatermark {
            struct_size: WATERMARK_BYTES,
            reserved0: 0,
            revision: watermark.revision,
            first_sample: watermark.first_sample,
            outcome_flags: watermark.outcome_flags,
            exact_count: watermark.exact,
            transition_fallback_count: watermark.transition_fallback,
            superseded_count: watermark.superseded,
            reserved: [0; 5],
        };
        RESULT_OK
    }

    /// The record [`Self::read_watermark`] fills.
    #[must_use]
    pub const fn watermark(&self) -> &WebPlanWatermark {
        &self.watermark
    }

    /// Read the last live-control submission report (issue #137 D1).
    #[must_use]
    pub const fn command_report(&self) -> &WebCommandReport {
        &self.command_report
    }

    /// Canonical normalized track order, the addressing authority for `track_index`.
    #[must_use]
    pub fn live_control_tracks(&self) -> &[Box<str>] {
        self.ready.as_ref().map_or(&[], |ready| &ready.tracks)
    }

    /// Canonical normalized submix order (issue #1210 D1): the trailing segment of the strip order,
    /// submix `j` being strip `live_control_tracks().len() + j`, in the order of the meter frame's
    /// submix sections.
    #[must_use]
    pub fn live_control_submixes(&self) -> &[Box<str>] {
        self.ready.as_ref().map_or(&[], |ready| &ready.submixes)
    }

    /// Number of sources the compiled session declares; zero before compilation (issue #207).
    ///
    /// This is the bounds authority for every other source query, exactly as
    /// [`AudioWorkletEngineHost::live_control_tracks`] is for `track_index`: the shape queries
    /// report a sentinel for an out-of-range index.
    #[must_use]
    pub fn session_source_count(&self) -> usize {
        self.ready
            .as_ref()
            .map_or(0, |ready| ready.session().normalized_model().sources.len())
    }

    /// One canonical source ID, or `None` for an out-of-range index (issue #207).
    ///
    /// The order is the normalized model's: `compile_session` sorts `sources` by stable ID, and
    /// this reads that list rather than a copy of it, so "canonical source order" has exactly one
    /// definition in this crate.
    #[must_use]
    pub fn session_source_id(&self, index: u32) -> Option<&str> {
        let ready = self.ready.as_ref()?;
        let source = ready
            .session()
            .normalized_model()
            .sources
            .get(index as usize)?;
        Some(source.id.as_str())
    }

    /// One source's declared shape, or `None` for an out-of-range index (issue #207).
    #[must_use]
    pub fn session_source_shape(&self, index: u32) -> Option<SessionSourceShape> {
        let ready = self.ready.as_ref()?;
        let source = ready
            .session()
            .normalized_model()
            .sources
            .get(index as usize)?;
        Some(SessionSourceShape {
            channel_count: u32::from(source.channels),
            frames: source.frames,
        })
    }

    /// The decimated meter frame (issue #137 D2, extended by #143 D5).
    ///
    /// `3(T + S) + 3` words (issue #1209 D1): two peaks per strip (the tracks, then the submixes in
    /// canonical order), master left and right, then one **non-negative** gain-reduction magnitude
    /// in decibels per strip in the same order and the master's. With no submixes every word is
    /// where it always was. Gain reduction is the latest fold from each independently aged effect
    /// observation and does not share the peak interval in [`WebMeterHeader`].
    ///
    /// The gain-reduction section is recomputed from the taps armed when each frame is polled. An
    /// unsubscribe disarms its tap when it is admitted, so every meter frame received after its
    /// `result === 0` acknowledgement leaves that tap out. A subscribe, also one that re-arms an
    /// armed tap, restarts its tap's window at the acknowledgement's `appliedAtSample`, and the
    /// frame leaves that tap out until a window that opens at or after that sample closes. A
    /// strip's slot (`trackGrDb`, `submixGrDb`) is the maximum over its armed taps' latest windows
    /// that opened at or after their last subscribe, and `0` when no armed tap has such a window.
    /// `masterGrDb` is `null` when the designated strip has none.
    #[must_use]
    pub fn meter_frame(&self) -> &[f32] {
        self.ready.as_ref().map_or(&[], |ready| &ready.meter_frame)
    }

    /// The sample window and shape the `f32` frame cannot carry (issue #143 D5).
    #[must_use]
    pub fn meter_header(&self) -> &WebMeterHeader {
        self.ready
            .as_ref()
            .map_or(&EMPTY_METER_HEADER, |ready| &ready.meter_header)
    }

    /// Armed taps, in the dense effect-slot order. Off-ABI introspection for the tests.
    #[must_use]
    pub fn observation_armed_taps(&self) -> u32 {
        self.ready.as_ref().map_or(0, |ready| {
            ready
                .observation_armed
                .iter()
                .map(|mask| mask.count_ones())
                .sum()
        })
    }

    /// Whether preparation bound any observation taps at all (issue #143 D3, level 1).
    #[must_use]
    pub fn observation_attached(&self) -> bool {
        self.ready.as_ref().is_some_and(|ready| {
            ready
                .effect_instances
                .iter()
                .flatten()
                .any(|instance| instance.reader.is_some())
        })
    }

    /// Copy one selected track's response owners at the current completed render boundary.
    ///
    /// The plan belongs to the plan owner (#1381a), which copies them from the active plan between
    /// blocks. The caller supplies bounded storage and may finish/evaluate the immutable copy
    /// after this method returns.
    pub fn copy_response_snapshot(
        &mut self,
        track_id: &str,
        sink: &mut dyn ResponseSnapshotSink,
    ) -> Result<engine::realtime::ResponseSnapshotCapture, ResponseSnapshotError> {
        if self.status.state != STATE_READY {
            return Err(ResponseSnapshotError::Owner);
        }
        let Some(plan) = self.plan.as_mut() else {
            return Err(ResponseSnapshotError::Owner);
        };
        plan.copy_response_snapshot(engine::realtime::ResponseSnapshotRequest { track_id, sink })
    }

    /// The spectrum service of a ready host, or the result word that refuses the call.
    fn spectrum_service(&self) -> Result<&SpectrumService, u32> {
        if self.status.state != STATE_READY {
            return Err(RESULT_WRONG_STATE);
        }
        self.ready
            .as_ref()
            .ok_or(RESULT_WRONG_STATE)?
            .spectrum
            .as_ref()
            .ok_or_else(|| spectrum_result(SpectrumServiceError::Unprepared))
    }

    /// [`Self::spectrum_service`] for the calls that change the service.
    fn spectrum_service_mut(&mut self) -> Result<&mut SpectrumService, u32> {
        if self.status.state != STATE_READY {
            return Err(RESULT_WRONG_STATE);
        }
        self.ready
            .as_mut()
            .ok_or(RESULT_WRONG_STATE)?
            .spectrum
            .as_mut()
            .ok_or_else(|| spectrum_result(SpectrumServiceError::Unprepared))
    }

    /// Arm the prepared one-shot spectrum observer for its next complete window.
    pub fn arm_spectrum(&mut self) -> u32 {
        match self.spectrum_service_mut().map(SpectrumService::arm) {
            Ok(Ok(())) => RESULT_OK,
            Ok(Err(error)) => spectrum_result(error),
            Err(result) => result,
        }
    }

    /// Read the completed spectrum window as `requested` channels, if the observer has finished
    /// its fixed capture.
    ///
    /// # Errors
    ///
    /// The result word of the service's refusal.
    pub fn read_spectrum(
        &mut self,
        requested: SpectrumChannels,
    ) -> Result<Option<SpectrumSnapshot>, u32> {
        self.spectrum_service_mut()?
            .read(requested)
            .map_err(spectrum_result)
    }

    /// Start the managed spectrum stream for this prepared target.
    ///
    /// The sample rate and render quantum come from the prepared host, so callers cannot create
    /// a stream whose cadence differs from the graph that will produce its samples.
    pub fn start_spectrum_stream(&mut self) -> Result<SpectrumCadence, u32> {
        if self.status.state != STATE_READY {
            return Err(RESULT_WRONG_STATE);
        }
        let Some(ready) = self.ready.as_mut() else {
            return Err(RESULT_WRONG_STATE);
        };
        let Some(service) = ready.spectrum.as_mut() else {
            return Err(RESULT_UNSUPPORTED);
        };
        let capture = service.captures_mut();
        let started = match self.spectrum_hop {
            Some(hop) => capture.start_continuous_with_hop(
                self.status.sample_rate_hz,
                self.status.quantum_frames,
                hop,
            ),
            None => {
                capture.start_continuous(self.status.sample_rate_hz, self.status.quantum_frames)
            }
        };
        if started.is_ok() {
            service.end_caller_arm();
        }
        started.map_err(|error| match error {
            SpectrumContinuousCaptureError::Busy => RESULT_BACKPRESSURE,
            SpectrumContinuousCaptureError::Cadence(error) => match error {
                host_core::SpectrumCadenceError::UnsupportedRate => RESULT_UNSUPPORTED,
                host_core::SpectrumCadenceError::ZeroQuantum => RESULT_INVALID_ARGUMENT,
                host_core::SpectrumCadenceError::UnsupportedHop => RESULT_INVALID_ARGUMENT,
                host_core::SpectrumCadenceError::HopOverflow => RESULT_REFUSED_BUDGET,
            },
            SpectrumContinuousCaptureError::EpochOverflow => RESULT_REFUSED_BUDGET,
        })
    }

    /// Restart the active managed stream at a fresh capture epoch without a fallible stop/start
    /// pair. The caller has already prevalidated the replacement analysis configuration.
    pub fn restart_spectrum_stream(&mut self) -> u32 {
        if self.status.state != STATE_READY {
            return RESULT_WRONG_STATE;
        }
        let Some(ready) = self.ready.as_mut() else {
            return RESULT_WRONG_STATE;
        };
        let Some(service) = ready.spectrum.as_mut() else {
            return RESULT_UNSUPPORTED;
        };
        match service.captures_mut().restart_continuous() {
            Ok(()) => RESULT_OK,
            Err(SpectrumContinuousCaptureError::Busy) => RESULT_BACKPRESSURE,
            Err(SpectrumContinuousCaptureError::Cadence(_)) => RESULT_WRONG_STATE,
            Err(SpectrumContinuousCaptureError::EpochOverflow) => RESULT_REFUSED_BUDGET,
        }
    }

    /// Read one independently scheduled managed stream window or its explicit availability state.
    pub fn read_spectrum_stream(
        &mut self,
    ) -> Result<SpectrumContinuousWindow, SpectrumContinuousReadError> {
        if self.status.state != STATE_READY {
            return Err(SpectrumContinuousReadError::NotActive);
        }
        let Some(ready) = self.ready.as_mut() else {
            return Err(SpectrumContinuousReadError::NotActive);
        };
        let Some(service) = ready.spectrum.as_mut() else {
            return Err(SpectrumContinuousReadError::NotActive);
        };
        service.captures_mut().try_read_continuous()
    }

    /// Stop the managed stream and discard its queued native window.
    pub fn stop_spectrum_stream(&mut self) -> u32 {
        if self.status.state != STATE_READY {
            return RESULT_WRONG_STATE;
        }
        let Some(ready) = self.ready.as_mut() else {
            return RESULT_WRONG_STATE;
        };
        let Some(service) = ready.spectrum.as_mut() else {
            return RESULT_UNSUPPORTED;
        };
        service.captures_mut().stop_continuous();
        RESULT_OK
    }

    /// Return the active managed stream cadence, if it is running.
    #[must_use]
    pub fn spectrum_stream_cadence(&self) -> Option<SpectrumCadence> {
        self.ready
            .as_ref()
            .and_then(|ready| ready.spectrum.as_ref())
            .map(SpectrumService::captures)
            .and_then(PreparedSpectrumCaptures::cadence)
    }

    /// Return the native capture epoch for the active managed stream.
    #[must_use]
    pub fn spectrum_stream_epoch(&self) -> Option<u64> {
        self.ready
            .as_ref()
            .and_then(|ready| ready.spectrum.as_ref())
            .map(SpectrumService::captures)
            .and_then(PreparedSpectrumCaptures::stream_epoch)
    }

    /// Cancel a pending spectrum capture and discard any completed window.
    pub fn cancel_spectrum(&mut self) -> u32 {
        match self.spectrum_service_mut() {
            Ok(service) => {
                service.cancel();
                RESULT_OK
            }
            Err(result) => result,
        }
    }

    /// The target kind configured for this prepared host, or zero when no spectrum observer exists.
    #[must_use]
    pub fn spectrum_target(&self) -> u32 {
        self.ready
            .as_ref()
            .and_then(|ready| ready.spectrum.as_ref())
            .map_or(0, SpectrumService::target_kind)
    }

    /// The channel mask configured for this prepared host, or zero when no observer exists.
    #[must_use]
    pub fn spectrum_channels(&self) -> u32 {
        self.ready
            .as_ref()
            .and_then(|ready| ready.spectrum.as_ref())
            .and_then(SpectrumService::channels)
            .map_or(0, spectrum_channels_raw)
    }

    /// Atomically select one exact prepared spectrum target and channel mask.
    ///
    /// The collection performs all fallible admission before retiring its current capture. A
    /// refusal therefore leaves the old stream, queued window and effective configuration intact.
    /// A select that would replace the entry while the caller holds an armed window is refused as
    /// backpressure (decision 17 D7).
    ///
    /// The worklet calls this on the browser's audio thread, so the target is borrowed and the
    /// selection allocates nothing.
    pub fn select_spectrum(
        &mut self,
        target: SpectrumTargetRef<'_>,
        channels: SpectrumChannels,
    ) -> u32 {
        match self
            .spectrum_service_mut()
            .map(|service| service.select(target, channels))
        {
            Ok(Ok(_)) => RESULT_OK,
            Ok(Err(error)) => spectrum_result(error),
            Err(result) => result,
        }
    }

    /// Validate a collection selection and report whether it would replace the current entry.
    pub fn spectrum_selection_would_change(
        &self,
        target: SpectrumTargetRef<'_>,
        channels: SpectrumChannels,
    ) -> Result<bool, u32> {
        self.spectrum_service()?
            .selection_would_change(target, channels)
            .map_err(spectrum_result)
    }

    /// Return the collection selection identity, or zero for the singular compatibility path.
    #[must_use]
    pub fn spectrum_selection_epoch(&self) -> u64 {
        self.ready
            .as_ref()
            .and_then(|ready| ready.spectrum.as_ref())
            .map_or(0, SpectrumService::selection_epoch)
    }

    /// Read one bounded batch of selected resident observation taps.
    ///
    /// The complete selection is resolved before a result vector is allocated, so a malformed,
    /// unsupported or over-sized batch leaves the host untouched. Each successful row reads its
    /// existing control-side cell without acknowledging it; legacy meter polling therefore keeps
    /// its own consumer semantics and repeated selected reads remain non-consuming.
    pub fn read_observations(
        &mut self,
        selections: &[ObservationSelection<'_>],
    ) -> Result<Vec<ObservationReadResult>, ObservationReadError> {
        Self::read_selected(
            self.status.state,
            self.status.sample_rate_hz,
            self.ready.as_ref(),
            self.plan.as_mut(),
            selections,
        )
    }

    /// The body of [`Self::read_observations`], over split borrows: the selections may borrow the
    /// ready table's IDs while the readers are read through the render half.
    fn read_selected(
        state: u32,
        sample_rate_hz: u32,
        ready: Option<&ReadyOwnership>,
        plan: Option<&mut WebPlan>,
        selections: &[ObservationSelection<'_>],
    ) -> Result<Vec<ObservationReadResult>, ObservationReadError> {
        if state != STATE_READY {
            return Err(ObservationReadError::WrongState);
        }
        if selections.len() > MAXIMUM_OBSERVATION_READS {
            return Err(ObservationReadError::BufferTooSmall);
        }
        let (Some(ready), Some(plan)) = (ready, plan) else {
            return Err(ObservationReadError::WrongState);
        };

        // Validate the whole batch before allocating results or touching a reader. Duplicate
        // tuples are rejected explicitly so callers cannot mistake two identical rows for two
        // independent consumer cursors.
        for (index, selection) in selections.iter().enumerate() {
            resolve_observation(ready, selection)?;
            if selections[..index].iter().any(|prior| {
                prior.track_id == selection.track_id
                    && prior.rack == selection.rack
                    && prior.effect_slot_id == selection.effect_slot_id
                    && prior.tap_id == selection.tap_id
            }) {
                return Err(ObservationReadError::InvalidSelection);
            }
        }

        let mut results = Vec::with_capacity(selections.len());
        for selection in selections {
            let values = read_observation_values(ready, plan, sample_rate_hz, selection)?;
            let (effect, tap_index) = resolve_observation(ready, selection)?;
            let handle = ready
                .effect_instances
                .get(effect)
                .and_then(Option::as_ref)
                .ok_or(ObservationReadError::Unsupported)?;
            let tap = handle
                .descriptor
                .observations
                .get(tap_index)
                .ok_or(ObservationReadError::InvalidSelection)?;
            results.push(ObservationReadResult {
                track_id: handle.track_id.clone(),
                rack: selection.rack,
                effect_slot_id: handle.effect_id.clone(),
                native_effect_id: handle.descriptor.id.as_str(),
                descriptor: *tap,
                channels: selection.channels,
                sample_rate_hz: values.sample_rate_hz,
                status: values.status,
                window: values.window,
                left: values.left,
                right: values.right,
            });
        }
        Ok(results)
    }

    /// Number of currently bound resident observation effects in dense owner order.
    #[must_use]
    pub fn observation_binding_count(&self) -> usize {
        self.ready.as_ref().map_or(0, |ready| {
            ready
                .effect_instances
                .iter()
                .flatten()
                .filter(|instance| instance.reader.is_some())
                .count()
        })
    }

    /// One current-owner observation binding for the read-only session map.
    #[must_use]
    pub fn observation_binding(&self, index: u32) -> Option<ObservationBinding<'_>> {
        let ready = self.ready.as_ref()?;
        let mut observed_index = 0_u32;
        for (effect, handle) in ready.effect_instances.iter().enumerate() {
            let Some(handle) = handle.as_ref().filter(|handle| handle.reader.is_some()) else {
                continue;
            };
            if observed_index == index {
                let track_index = *ready.observation_tracks.get(effect)?;
                if track_index == u32::MAX {
                    return None;
                }
                return Some(ObservationBinding {
                    address: ObservationAddress {
                        track_index,
                        rack: handle.address.rack,
                        effect_index: handle.address.index,
                        tap_id: 0,
                        channels: ObservationReadChannels::Both,
                    },
                    effect_slot_id: handle.effect_id.as_ref(),
                    native_effect_id: handle.descriptor.id.as_str(),
                    descriptors: handle.descriptor.observations,
                });
            }
            observed_index = observed_index.saturating_add(1);
        }
        None
    }

    /// Read a bounded numeric batch after the current stable map has resolved it.
    pub fn read_observation_addresses(
        &mut self,
        addresses: &[ObservationAddress],
    ) -> Result<Vec<ObservationReadResult>, ObservationReadError> {
        if addresses.len() > MAXIMUM_OBSERVATION_READS {
            return Err(ObservationReadError::BufferTooSmall);
        }
        let Some(ready) = self.ready.as_ref() else {
            return Err(ObservationReadError::WrongState);
        };
        let mut selections = Vec::with_capacity(addresses.len());
        for address in addresses {
            selections.push(observation_selection_for_address(ready, *address)?);
        }
        Self::read_selected(
            self.status.state,
            self.status.sample_rate_hz,
            self.ready.as_ref(),
            self.plan.as_mut(),
            &selections,
        )
    }

    /// Fill a caller-owned numeric result slice without allocating on the Wasm boundary.
    pub fn read_observation_addresses_into(
        &mut self,
        addresses: &[ObservationAddress],
        output: &mut [ObservationReadValues],
    ) -> Result<(), ObservationReadError> {
        if self.status.state != STATE_READY {
            return Err(ObservationReadError::WrongState);
        }
        if addresses.len() > MAXIMUM_OBSERVATION_READS || output.len() < addresses.len() {
            return Err(ObservationReadError::BufferTooSmall);
        }
        let (Some(ready), Some(plan)) = (self.ready.as_ref(), self.plan.as_mut()) else {
            return Err(ObservationReadError::WrongState);
        };
        for (index, address) in addresses.iter().enumerate() {
            let selection = observation_selection_for_address(ready, *address)?;
            resolve_observation(ready, &selection)?;
            if addresses[..index].contains(address) {
                return Err(ObservationReadError::InvalidSelection);
            }
        }
        for (index, address) in addresses.iter().enumerate() {
            let selection = observation_selection_for_address(ready, *address)?;
            output[index] =
                read_observation_values(ready, plan, self.status.sample_rate_hz, &selection)?;
        }
        Ok(())
    }

    /// Number of complete meter windows folded since compilation.
    #[must_use]
    pub fn meter_windows(&self) -> u64 {
        self.ready.as_ref().map_or(0, |ready| ready.meter_windows)
    }

    #[cfg(test)]
    pub(crate) fn meter_poll_work(&mut self) -> (u64, u64) {
        let scans = self
            .ready
            .as_ref()
            .map_or(0, |ready| ready.meter_effect_scan_count);
        let meters = self
            .plan
            .as_mut()
            .and_then(|plan| plan.active_attachment_mut::<RenderCompanions>())
            .map_or(0, |companions| {
                companions
                    .meters
                    .iter()
                    .map(|meter| meter.consumer.empty_count() + meter.consumer.success_count())
                    .sum()
            });
        (meters, scans)
    }

    /// Whether meter observers were attached at preparation (issue #137 D2).
    #[must_use]
    pub fn meters_attached(&self) -> bool {
        self.ready
            .as_ref()
            .is_some_and(|ready| !ready.meter_pending.is_empty())
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
        // Repeated acquisition/release is idempotent. A real transition starts a new publication
        // generation, clears host-owned pending state, and marks producer-queued intervals from
        // the previous lease for bounded rejection by the next poll.
        if enabled == self.meter_lease {
            return self.record(RESULT_OK);
        }
        self.meter_lease = enabled;
        if let Some(ready) = self.ready.as_mut() {
            ready.meter_generation = ready.meter_generation.saturating_add(1);
            ready.reset_meter_delivery(true);
            ready.meter_header.reserved[0] = ready.meter_generation;
        }
        self.meter_activation_sample = self.next_meter_boundary(self.status.next_absolute_sample);
        self.record(RESULT_OK)
    }

    fn next_meter_boundary(&self, sample: u64) -> u64 {
        let Some(period) = self
            .options
            .live_control_meter_blocks
            .checked_mul(u64::from(self.status.quantum_frames))
            .filter(|period| *period != 0)
        else {
            return sample;
        };
        let remainder = sample % period;
        sample.saturating_add(if remainder == 0 {
            0
        } else {
            period - remainder
        })
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

    /// Mutable live-control command staging, or `None` when no live controls were attached.
    ///
    /// This is the buffer the JavaScript side writes records into through
    /// `miso_engine_web_v1_buffer_ptr(handle, BUFFER_COMMAND)`; it is public so an embedding that
    /// drives the safe host directly -- the native parity twin, the metadata round-trip gate --
    /// stages exactly the bytes the browser stages.
    pub fn command_staging_mut(&mut self) -> Option<&mut [u8]> {
        self.buffers
            .as_mut()
            .map(|value| &mut *value.command)
            .filter(|value| !value.is_empty())
    }

    /// Exact byte size of the live-control command staging buffer; zero when none was attached.
    #[must_use]
    pub fn command_staging_bytes(&self) -> u64 {
        self.buffers
            .as_ref()
            .map_or(0, |value| value.command.len() as u64)
    }

    /// Whether the boot asked for observation taps, so the command staging exists and the
    /// observation records (kinds 7 and 8) are accepted (issue #137 D1; #1565).
    #[must_use]
    pub fn live_controls_attached(&self) -> bool {
        self.ready
            .as_ref()
            .is_some_and(|ready| ready.commands_enabled)
    }

    /// Copy one canonical live-control track ID into ID staging; returns its byte length.
    pub(crate) fn copy_live_control_track_id(&mut self, index: u32) -> u32 {
        let Some(ready) = self.ready.as_ref() else {
            return 0;
        };
        let Some(id) = ready.tracks.get(index as usize) else {
            return 0;
        };
        Self::copy_id_into_staging(self.buffers.as_mut(), id.as_bytes())
    }

    /// Copy one canonical submix ID into ID staging; returns its byte length (issue #1210 D1).
    pub(crate) fn copy_live_control_submix_id(&mut self, index: u32) -> u32 {
        let Some(ready) = self.ready.as_ref() else {
            return 0;
        };
        let Some(id) = ready.submixes.get(index as usize) else {
            return 0;
        };
        Self::copy_id_into_staging(self.buffers.as_mut(), id.as_bytes())
    }

    /// Copy one canonical source ID into ID staging; returns its byte length (issue #207).
    pub(crate) fn copy_session_source_id(&mut self, index: u32) -> u32 {
        let Some(ready) = self.ready.as_ref() else {
            return 0;
        };
        let Some(source) = ready
            .session()
            .normalized_model()
            .sources
            .get(index as usize)
        else {
            return 0;
        };
        Self::copy_id_into_staging(self.buffers.as_mut(), source.id.as_str().as_bytes())
    }

    fn copy_id_into_staging(buffers: Option<&mut PreparedBuffers>, bytes: &[u8]) -> u32 {
        let length = bytes.len();
        let buffers = buffers.expect("a ready host owns its staging buffers");
        debug_assert!(
            length <= buffers.source_id.len(),
            "compiled ID exceeds its projected staging capacity"
        );
        buffers.source_id[..length].copy_from_slice(bytes);
        u32::try_from(length).expect("ID staging capacity is representable as u32")
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
        if frames > self.status.quantum_frames {
            return self.record(RESULT_INVALID_ARGUMENT);
        }
        let Some(ready) = self.ready.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        // #1381a D2: through the control plane's source routing, which feeds the sources of the
        // newest committed session. It runs no service step: this is called from `process()` in the
        // PCM-feed worklet, and the service step locks and allocates (batch 7 review B1).
        let result = ready.control.submit_unserviced(
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
            Err(failure) => source_failure_result(failure),
        };
        self.record(code)
    }

    /// Apply one strictly increasing generation-tagged source seek between render blocks.
    ///
    /// The seek goes through the control plane's source routing (#1381a D2). This host also owns
    /// the render plan on its one thread, so a successful admission prepares the plan's consumer
    /// too, and new PCM can enter even when old queues were full. A seek changes source-content
    /// position while the absolute render clock stays continuous; it therefore neither resets
    /// meter accumulators nor advances the meter publication epoch.
    pub fn seek_source(&mut self, source_id: &[u8], generation: u64, source_frame: u64) -> u32 {
        if self.status.state != STATE_READY {
            return self.record(RESULT_WRONG_STATE);
        }
        let (Some(ready), Some(plan)) = (self.ready.as_mut(), self.plan.as_mut()) else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        let Ok(source_index) = ready
            .session()
            .normalized_model()
            .sources
            .binary_search_by(|source| source.id.as_str().as_bytes().cmp(source_id))
        else {
            return self.record(RESULT_INVALID_ARGUMENT);
        };
        if !plan.can_prepare_source_seek(source_index) {
            return self.record(RESULT_WRONG_STATE);
        }
        let code = match ready
            .control
            .seek_unserviced(source_id, generation, source_frame)
        {
            Ok(()) => {
                if plan.prepare_source_seek(source_index, generation, source_frame) {
                    RESULT_OK
                } else {
                    RESULT_INTERNAL
                }
            }
            Err(failure) => source_failure_result(failure),
        };
        self.record(code)
    }

    // REALTIME_POLICY_BEGIN
    /// Render exactly one prepared quantum at the next absolute sample time.
    pub fn render_next(&mut self) -> u32 {
        if self.status.state != STATE_READY {
            return self.record(RESULT_WRONG_STATE);
        }
        // Lossless on every supported target (wasm32 and every 64-bit host); `try_from` would
        // leave an `expect_failed` trap owner in the render export's call graph.
        let quantum = self.status.quantum_frames as usize;
        let meter_period = self
            .options
            .live_control_meter_blocks
            .checked_mul(u64::from(self.status.quantum_frames));
        let (Some(ready), Some(plan)) = (self.ready.as_mut(), self.plan.as_mut()) else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        let Some(buffers) = self.buffers.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.buffers\t$\n");
        };
        let output = match PlanarBufferMut::try_new(&mut buffers.output_pcm, 2, quantum, quantum) {
            Ok(value) => value,
            Err(_) => return self.fail(RESULT_INTERNAL, b"web.internal.output\t$\n"),
        };
        let render_first_sample = self.status.next_absolute_sample;
        // #1381a D4: through the plan owner, which adopts a published successor at this block's
        // boundary and continues the clock.
        let result = plan.render(self.status.next_absolute_sample, output);
        match result {
            Ok(()) => {
                let next_absolute_sample = plan.owner().next_absolute_sample();
                self.status.next_absolute_sample = next_absolute_sample;
                self.status.rendered_quanta = self.status.rendered_quanta.saturating_add(1);
                // #1570b D3: the block's activity, published by the control plane's one rule
                // (the peak scan runs only for an armed observer, `NaN` otherwise), so the
                // service step stages the telemetry events and `TRANSPORT_GET` reports the
                // effective sample exactly as on the C ABI. At most `2 * quantum` loads, no
                // allocation, indexed with `get` so no `slice_index_fail` joins the call graph.
                if let (Some(left), Some(right)) = (
                    buffers.output_pcm.get(..quantum),
                    buffers
                        .output_pcm
                        .get(quantum..)
                        .and_then(|tail| tail.get(..quantum)),
                ) {
                    plan.observe_rendered_block([left, right]);
                }
                // #1399: the session's count, loaded after the block that added to it, so the
                // status is never behind the last rendered block.
                self.status.live_values_superseded = ready.live_values_superseded.get();
                // Issue #137 D1: every effect stage drained its observation queue at the top of
                // this block, so the exact free-slot count is the whole capacity again. Command-free
                // blocks have no counters to clear and skip the dense write pass entirely.
                if ready.has_in_flight_commands {
                    #[cfg(test)]
                    record_admission_counter_clear(ready.in_flight.len());
                    ready.in_flight.fill(0);
                    ready.has_in_flight_commands = false;
                }
                // Issue #137 D2: the master bus is the host's own output plane, so there is
                // nothing to observe and nothing to expose -- one branch and one pass over a
                // buffer already in cache, and only while the lease is held.
                if self.meter_lease && render_first_sample >= self.meter_activation_sample {
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
                            let magnitude = meter_sample_magnitude(*sample);
                            if magnitude > *peak {
                                *peak = magnitude;
                            }
                        }
                    }
                    ready.master_peak = peaks;
                    if ready.master_start_sample.is_none() {
                        ready.master_start_sample = Some(render_first_sample);
                    }
                    ready.master_end_sample = next_absolute_sample;
                    if let Some(period) = meter_period.filter(|period| *period != 0)
                        && ready.master_start_sample.is_some()
                        && ready.master_end_sample.saturating_sub(
                            ready.master_start_sample.unwrap_or(render_first_sample),
                        ) >= period
                    {
                        let measurement = MasterMeasurement {
                            start_sample: ready.master_start_sample.unwrap_or(render_first_sample),
                            end_sample: ready.master_end_sample,
                            peak: ready.master_peak,
                        };
                        ready.push_master_window(measurement);
                        ready.master_peak = [0.0, 0.0];
                        ready.master_start_sample = None;
                        ready.master_end_sample = 0;
                    }
                }
                self.record(RESULT_OK)
            }
            Err(_) => self.fail(RESULT_RENDER_REJECTED, b"web.render.rejected\t$\n"),
        }
    }
    // REALTIME_POLICY_END

    /// Admit one staged `miso.command.v1` submission of observation records (kinds 7 and 8) as a
    /// single transaction (issue #137 D1; #1565A).
    ///
    /// The browser's value kinds are gone: every value goes through the core
    /// ([`Self::control_command`]). What stays is the monitor operation that arms or disarms an
    /// effect's observation tap. A record of any other kind is `MALFORMED` at its index, as an
    /// unknown kind always was.
    ///
    /// # The two-pass shape is the contract, not an optimisation
    ///
    /// Pass one validates every record -- shape, addressing, tap, and free queue room -- and
    /// pushes nothing. Pass two pushes. A submission is therefore all-or-nothing: eval E4's "engine
    /// state untouched" is a property of the code, not of a lucky ordering, and eval E3's flood of
    /// queued records is refused before a single one reaches a queue.
    ///
    /// # Why this is not on the render thread's critical path
    ///
    /// This runs from `port.onmessage`, which the user agent dispatches between render quanta on
    /// the same thread as `process()`. It performs no allocation, no compilation and no plan
    /// replacement: the staging arrays were allocated at compilation, and the call moves at most
    /// [`MAXIMUM_COMMAND_RECORDS`] `Copy` records into the bounded observation queues. The block
    /// that follows drains the queues.
    pub fn submit_commands(&mut self, count: u32) -> u32 {
        self.command_report = empty_command_report();
        if self.status.state != STATE_READY {
            return self.finish_commands(RESULT_WRONG_STATE, COMMAND_REASON_WRONG_STATE, 0, 0);
        }
        if count > MAXIMUM_COMMAND_RECORDS {
            return self.finish_commands(RESULT_INVALID_ARGUMENT, COMMAND_REASON_MALFORMED, 0, 0);
        }
        if !self.live_controls_attached() {
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
        // Disjoint borrows: the staged bytes live in `buffers`, the live controls live in `ready`.
        let Some((buffers, ready)) = self.buffers.as_ref().zip(self.ready.as_mut()) else {
            return self.fail(RESULT_INTERNAL, b"web.internal.live_controls\t$\n");
        };
        let Some(bytes) = buffers.command.get(..staged) else {
            return self.finish_commands(RESULT_INVALID_ARGUMENT, COMMAND_REASON_MALFORMED, 0, 0);
        };
        match admit_commands(ready, bytes, count as usize, applied_at_sample) {
            Ok(()) => {
                self.command_report.applied_at_sample = applied_at_sample;
                self.finish_commands(RESULT_OK, COMMAND_REASON_NONE, 0, count)
            }
            Err(CommandRejection {
                result,
                reason,
                index,
            }) if result == RESULT_INTERNAL => {
                self.finish_commands(RESULT_INTERNAL, reason, index, 0);
                self.fail(RESULT_INTERNAL, b"web.internal.command\t$\n")
            }
            Err(CommandRejection {
                result,
                reason,
                index,
            }) => self.finish_commands(result, reason, index, 0),
        }
    }

    // REALTIME_POLICY_BEGIN
    /// Drain every finished meter window into the frame buffer (issue #137 D2).
    ///
    /// Returns the number of complete windows folded by this call. Zero work happens without the
    /// lease, and a host prepared with `live_control_meter_blocks == 0` has no observer to drain.
    ///
    /// Allocation-free by construction: it moves `Copy` snapshots out of bounded queues into a
    /// buffer allocated at compilation.
    pub fn poll_meters(&mut self) -> u32 {
        if !self.meter_lease || self.status.state != STATE_READY {
            return 0;
        }
        let reset_activation_sample = self.next_meter_boundary(self.status.next_absolute_sample);
        let (Some(ready), Some(plan)) = (self.ready.as_mut(), self.plan.as_mut()) else {
            return 0;
        };
        // #1381a D3: the meter consumers and the observation readers are the active plan's
        // companions. The downcast allocates and drops nothing.
        let Some(companions) = plan.active_attachment_mut::<RenderCompanions>() else {
            return 0;
        };
        // This function has no runtime-checked `[]` index (only `get`/`get_mut` and constant
        // indexes into a fixed array), so it adds no bounds check of its own. The one
        // `panic_bounds_check` in this export's call graph is the slot index of the inlined SPSC
        // `try_pop` (`slots[self.local]` in `crates/engine/src/realtime/spsc.rs`), which the ring
        // keeps in range. This export is called from `process()`, so the shipped artifact's
        // call-graph gate covers it as it covers the render export.
        let meter_count = companions.meters.len();
        if meter_count == 0 {
            return 0;
        }
        // The render path is the sole producer of this fixed-capacity ring and increments the
        // count only when the configured master interval closes. An empty ring therefore proves
        // that no coherent track/master publication can exist. Preserve pending/public bytes and
        // avoid walking every track consumer on incomplete quanta.
        if ready.master_count == 0 {
            return 0;
        }
        // Issue #1448 D2: each meter's queue pops at most its own count read here, at entry, capped
        // at the configured depth, whatever a producer on another thread publishes meanwhile. A
        // pass that does not end the loop has cleared at least one pending slot, and the next pass
        // either refills it (spending one count) or ends the loop at the empty-slot check, so
        // `1 + sum(remaining)` passes always suffice: the bound never cuts a recovery short.
        for (meter, remaining) in companions.meters.iter().zip(&mut ready.meter_remaining) {
            *remaining = meter.consumer.available_at_entry().min(METER_QUEUE_DEPTH);
        }
        let mut passes = 1_usize;
        for remaining in ready.meter_remaining.iter() {
            passes = passes.saturating_add(*remaining);
        }
        let mut windows = 0_u32;
        let mut folded_start = 0_u64;
        let mut folded_end = 0_u64;
        let mut folded = false;
        for _ in 0..passes {
            for ((meter, pending), remaining) in companions
                .meters
                .iter_mut()
                .zip(&mut ready.meter_pending)
                .zip(&mut ready.meter_remaining)
            {
                if pending.is_some() || *remaining == 0 {
                    continue;
                }
                // On a single-consumer queue a count read at entry is a lower bound on what this
                // consumer can pop, so a pop under a count above zero does not fail.
                *remaining -= 1;
                if let Ok(snapshot) = meter.consumer.try_pop() {
                    *pending = Some(snapshot);
                }
            }
            if ready.meter_pending.iter().any(Option::is_none) {
                break;
            }
            let Some(first) = ready
                .meter_pending
                .first()
                .and_then(Option::as_ref)
                .copied()
            else {
                break;
            };
            let all_equal = ready
                .meter_pending
                .iter()
                .enumerate()
                .all(|(index, entry)| {
                    let Some(snapshot) = entry else { return false };
                    companions
                        .meters
                        .get(index)
                        .is_some_and(|meter| snapshot.handle == meter.handle)
                        && snapshot.reset_generation == first.reset_generation
                        && snapshot.window_sequence == first.window_sequence
                        && snapshot.start_sample == first.start_sample
                        && snapshot.end_sample == first.end_sample
                        && snapshot.frames == first.frames
                });
            if !all_equal {
                // A sequence gap can be recovered by dropping only the older stream head. Equal
                // sequences with different spans/generations are a discontinuity: drop the whole
                // candidate set so no mixed interval reaches the public frame.
                let same_generation = ready.meter_pending.iter().all(|entry| {
                    entry
                        .is_some_and(|snapshot| snapshot.reset_generation == first.reset_generation)
                });
                let same_sequence = ready.meter_pending.iter().all(|entry| {
                    entry.is_some_and(|snapshot| snapshot.window_sequence == first.window_sequence)
                });
                if same_generation && !same_sequence {
                    let minimum = ready
                        .meter_pending
                        .iter()
                        .filter_map(|entry| entry.as_ref().map(|snapshot| snapshot.window_sequence))
                        .min()
                        .unwrap_or(first.window_sequence);
                    for pending in &mut ready.meter_pending {
                        if pending
                            .as_ref()
                            .is_some_and(|snapshot| snapshot.window_sequence == minimum)
                        {
                            *pending = None;
                            ready.meter_loss_count = ready.meter_loss_count.saturating_add(1);
                        }
                    }
                } else {
                    for pending in &mut ready.meter_pending {
                        *pending = None;
                        ready.meter_loss_count = ready.meter_loss_count.saturating_add(1);
                    }
                }
                continue;
            }
            if first.start_sample < self.meter_activation_sample
                || first.end_sample <= first.start_sample
                || u64::from(first.frames) != first.end_sample - first.start_sample
            {
                for pending in &mut ready.meter_pending {
                    *pending = None;
                }
                ready.meter_loss_count = ready.meter_loss_count.saturating_add(1);
                continue;
            }
            if ready
                .meter_snapshot_generation
                .is_some_and(|generation| generation != first.reset_generation)
            {
                // A producer reset is a new delivery epoch. Reject this group under the old
                // public identity and clear all bounded delivery state so the next render opens a
                // clean master/track interval. The prior public frame and header stay byte-stable
                // because a zero return never publishes an epoch transition.
                ready.meter_generation = ready.meter_generation.saturating_add(1);
                ready.reset_meter_delivery(false);
                ready.meter_loss_count = 1;
                self.meter_activation_sample = reset_activation_sample;
                return 0;
            }
            if ready.meter_header.sequence > 0
                && ready.meter_header.reserved[0] == ready.meter_generation
                && first.start_sample != ready.meter_header.end_sample
            {
                ready.meter_loss_count = ready.meter_loss_count.saturating_add(1);
            }
            folded_start = first.start_sample;
            folded = true;
            folded_end = first.end_sample;
            windows = windows.saturating_add(1);
            // The frozen adapter policy is bounded per-window delivery. Further complete windows
            // remain queued for the next poll, preserving one exact span per public frame.
            break;
        }
        if !folded {
            return 0;
        }
        // Issue #1209 D1: one peak pair per strip, tracks then submixes, so the master follows the
        // last submix rather than the last track.
        let strips = ready.tracks.len() + ready.submixes.len();
        let master = strips * 2;
        let mut stale_master_budget = ready.master_count;
        while stale_master_budget > 0 {
            let stale = ready
                .master_windows
                .get(ready.master_read)
                .and_then(Option::as_ref)
                .is_some_and(|measurement| measurement.end_sample <= folded_start);
            if !stale {
                break;
            }
            let _ = ready.pop_master_window();
            ready.meter_loss_count = ready.meter_loss_count.saturating_add(1);
            stale_master_budget -= 1;
        }
        let master_measurement = ready
            .master_windows
            .get(ready.master_read)
            .and_then(Option::as_ref)
            .copied();
        let master_valid = master_measurement.is_some_and(|measurement| {
            measurement.start_sample == folded_start && measurement.end_sample == folded_end
        });
        if !master_valid {
            // The host cannot recover an interval peak from one envelope once its boundary is
            // lost. Consume the track group, count the discontinuity, and wait for the next clean
            // master/track span rather than publishing a mixed or guessed master value.
            for pending in &mut ready.meter_pending {
                if let Some(snapshot) = pending.take()
                    && snapshot.cumulative_dropped_snapshots > ready.meter_snapshot_drops_seen
                {
                    ready.meter_loss_count = ready.meter_loss_count.saturating_add(
                        snapshot
                            .cumulative_dropped_snapshots
                            .saturating_sub(ready.meter_snapshot_drops_seen),
                    );
                    ready.meter_snapshot_drops_seen = snapshot.cumulative_dropped_snapshots;
                }
            }
            ready.meter_loss_count = ready.meter_loss_count.saturating_add(1);
            return 0;
        }
        let master_peak = ready
            .pop_master_window()
            .map_or([0.0, 0.0], |measurement| measurement.peak);
        // All candidate validation is complete. Only this commit section mutates the public frame;
        // every earlier zero-return path leaves both public buffers byte-for-byte unchanged.
        for (index, pending) in ready.meter_pending.iter_mut().enumerate() {
            let Some(snapshot) = pending.take() else {
                continue;
            };
            if snapshot.cumulative_dropped_snapshots > ready.meter_snapshot_drops_seen {
                ready.meter_loss_count = ready.meter_loss_count.saturating_add(
                    snapshot
                        .cumulative_dropped_snapshots
                        .saturating_sub(ready.meter_snapshot_drops_seen),
                );
                ready.meter_snapshot_drops_seen = snapshot.cumulative_dropped_snapshots;
            }
            if let Some(slot) = ready.meter_frame.get_mut(index * 2) {
                *slot = snapshot.left.sample_peak;
            }
            if let Some(slot) = ready.meter_frame.get_mut(index * 2 + 1) {
                *slot = snapshot.right.sample_peak;
            }
            ready.meter_snapshot_generation = Some(snapshot.reset_generation);
        }
        for (plane, peak) in master_peak.iter().enumerate() {
            if let Some(slot) = ready.meter_frame.get_mut(master + plane) {
                *slot = *peak;
            }
        }

        // Issue #143 D5: the gain-reduction section. This is the control plane -- `poll_meters` is
        // called from `process()` but after the render export, and it performs no allocation, no
        // lock and no unbounded loop -- so the one unit conversion the whole design permits lives
        // here: a tap that publishes a **linear** magnitude (the true-peak limiter's recursive
        // reduction word) becomes decibels once per closed window, never per sample and never on a
        // lane kernel.
        let gain_base = strips * 2 + 2;
        for slot in ready.meter_frame.iter_mut().skip(gain_base) {
            *slot = 0.0;
        }
        for slot in ready.observation_present.iter_mut() {
            *slot = false;
        }
        let mut observed_any = false;
        for (effect, entry) in ready.effect_instances.iter().enumerate() {
            #[cfg(test)]
            {
                ready.meter_effect_scan_count = ready.meter_effect_scan_count.saturating_add(1);
            }
            let Some(observed) = entry.as_ref() else {
                continue;
            };
            let Some(handle) = observed
                .reader
                .and_then(|reader| companions.effect_observations.get(reader as usize))
            else {
                continue;
            };
            let Some(track) = ready.observation_tracks.get(effect).copied() else {
                continue;
            };
            // Issue #1209 D5: the frame carries one gain-reduction word per strip, so a submix's
            // effect folds into its own word at `gain_base + T + j`.
            if track == u32::MAX {
                continue;
            }
            let armed = ready.observation_armed.get(effect).copied().unwrap_or(0);
            if armed == 0 {
                continue;
            }
            for (tap_index, reader) in handle.readers.iter().enumerate() {
                if u32::try_from(tap_index)
                    .ok()
                    .is_none_or(|index| index >= u32::BITS || armed & (1_u32 << index) == 0)
                {
                    continue;
                }
                // Issue #1534 D1-D4: a window from before the tap's last subscribe is no window.
                // It writes no slot, marks nothing present and is not acknowledged; the first
                // fresh window's acknowledgement covers its sequence.
                let Some(window) = reader.read().and_then(|window| {
                    window_since_arm(&ready.observation_arm_samples, effect, tap_index, window)
                }) else {
                    continue;
                };
                reader.acknowledge(window.sequence);
                let Some(tap) = observed.descriptor.observations.get(tap_index) else {
                    continue;
                };
                let magnitude = if window.left > window.right {
                    window.left
                } else {
                    window.right
                };
                let value = observed_decibels(*tap, magnitude);
                // Several armed taps on one track fold max-magnitude into the one positional slot
                // the frame carries for that track, on the control plane.
                if let Some(slot) = ready.meter_frame.get_mut(gain_base + track as usize)
                    && value > *slot
                {
                    *slot = value;
                }
                if let Some(present) = ready.observation_present.get_mut(track as usize) {
                    *present = true;
                }
                observed_any = true;
            }
        }
        // `masterGrDb` is a designation, not a discovery (D6). It is absent -- not zero -- when no
        // track was designated or the designated track published nothing, because zero would be
        // indistinguishable from "the master is not reducing".
        let master_present = match ready.master_track {
            Some(track) => ready
                .observation_present
                .get(track as usize)
                .copied()
                .unwrap_or(false),
            None => false,
        };
        if master_present
            && let Some(track) = ready.master_track
            && let Some(value) = ready.meter_frame.get(gain_base + track as usize).copied()
            && let Some(slot) = ready.meter_frame.get_mut(gain_base + strips)
        {
            *slot = value;
        }
        ready.meter_header.windows = windows;
        ready.meter_header.sequence = ready.meter_header.sequence.saturating_add(1);
        ready.meter_header.master_gr_present = u32::from(master_present);
        ready.meter_header.first_sample = folded_start;
        ready.meter_header.end_sample = folded_end;
        let mut validity = METER_VALID_COMPLETE;
        if master_valid {
            validity |= METER_VALID_MASTER;
        }
        if ready.meter_loss_count != 0 {
            validity |= METER_VALID_LOSS;
        }
        if observed_any {
            validity |= METER_VALID_GAIN_REDUCTION;
        }
        ready.meter_header.reserved[0] = ready.meter_generation;
        ready.meter_header.reserved[1] =
            validity | (ready.meter_loss_count.min(u64::from(u32::MAX)) << METER_LOSS_SHIFT);
        ready.meter_windows = ready.meter_windows.saturating_add(u64::from(windows));
        windows
    }
    // REALTIME_POLICY_END
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
        if actual_frames == self.status.quantum_frames {
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
        self.plan = None;
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
    /// `self.ready` and `self.plan` **are** this host's one-slot retirement queue (AGENTS.md: "the
    /// displaced plan goes to a bounded retirement queue and is reclaimed off the render thread").
    /// `fail` is reachable from `render_next`, which the AudioWorklet calls on the rendering
    /// thread, so it must not move, overwrite or drop the plan, the compiled session or the source
    /// rings: dropping them would free `Arc<spsc::Ring<_>>` payloads and run `dlmalloc::free`
    /// inside `process()`. The ownership stays exactly where it is and only [`Self::dispose`] — a
    /// control path call, delivered on `port.onmessage` — reclaims it.
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
/// | 0 | u8 | `kind`: [`COMMAND_OBSERVE_SUBSCRIBE`] or [`COMMAND_OBSERVE_UNSUBSCRIBE`] |
/// | 1 | u8 | `rack`: `1` inserts, `3` console; `0` and `2` retired |
/// | 2 | u8 | `channel`: `255` (not applicable) |
/// | 3 | u8 | required zero |
/// | 4 | u32 | `track_index` into the canonical normalized strip order |
/// | 8 | u32 | `effect_index`: a console slot's index in slot order, or an insert's index |
/// | 12 | u32 | `parameter_id`: the effect-local `tap_id` |
/// | 16 | u32 | `smoothing_samples`: the window length in render blocks (`0` = the plan's default) |
/// | 20 | u32 | required zero |
/// | 24..40 | 4 x f32 | `value0..value3`, all `0.0` |
/// | 40 | u64 | required zero |
///
/// The record keeps the 48-byte layout the value kinds shared (kinds 1-6 and 9-17 are deleted,
/// #1565A, and their numbers are never reallocated), so the staging, the report and the ABI
/// version did not move. There is no string on this path. The identity mapping a caller needs --
/// strip index to ID, effect index to effect ID, tap ID to descriptor -- is the session map the
/// ready message carries and the build-time metadata, never a per-command lookup.
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
            COMMAND_OBSERVE_SUBSCRIBE | COMMAND_OBSERVE_UNSUBSCRIBE
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

    /// Lower one observation subscribe/unsubscribe against the addressed effect's declared menu.
    ///
    /// `parameter_id` carries the effect-local `tap_id` and is translated to the `tap_index` the
    /// render side arms — an admission-time lookup, off the render thread. A tap the descriptor
    /// does not declare is [`COMMAND_REASON_UNKNOWN_TAP`] and never
    /// [`COMMAND_REASON_UNKNOWN_PARAMETER`]: they are different namespaces on one effect, and a
    /// caller that confuses them learns which one it got wrong.
    fn into_observe_record(
        self,
        descriptor: &'static effect_contract::EffectDescriptor,
        observed: bool,
    ) -> Result<(u32, bool, u32), u32> {
        if self.channel != 255 || self.values.iter().any(|value| *value != 0.0) {
            return Err(COMMAND_REASON_MALFORMED);
        }
        if self.parameter_id == 0 {
            return Err(COMMAND_REASON_UNKNOWN_TAP);
        }
        let Some((tap_index, tap)) = descriptor
            .observations
            .iter()
            .enumerate()
            .find(|(_, tap)| tap.id.0 == self.parameter_id)
        else {
            return Err(COMMAND_REASON_UNKNOWN_TAP);
        };
        // A computed tap is declared, validated and refused: V1 ships no implementation for one,
        // and saying so is better than binding a lane that would never publish.
        if !matches!(tap.cost, effect_contract::ObservationCost::Resident) {
            return Err(COMMAND_REASON_UNSUPPORTED_KIND);
        }
        // The tap exists and the address is right; this *session* bound no lane to arm.
        if !observed {
            return Err(COMMAND_REASON_OBSERVATION_UNBOUND);
        }
        let tap_index = u32::try_from(tap_index).map_err(|_| COMMAND_REASON_MALFORMED)?;
        Ok((
            tap_index,
            self.kind == COMMAND_OBSERVE_SUBSCRIBE,
            self.smoothing_samples,
        ))
    }
}

/// Validate a whole staged submission, then admit it. Nothing is pushed unless everything passes.
///
/// # The two passes are the contract, not an optimisation (#137 D1, #1565A)
///
/// Pass one decodes and lowers every record and counts the room each effect's observation queue
/// needs, then checks that room against every queue the submission touches. Pass two pushes. A
/// submission is therefore all-or-nothing: the refusal names the first record that broke a rule,
/// and a refused submission leaves the armed set, the arm samples and every queue as they were.
fn admit_commands(
    ready: &mut ReadyOwnership,
    bytes: &[u8],
    count: usize,
    applied_at_sample: u64,
) -> Result<(), CommandRejection> {
    let record_bytes = COMMAND_RECORD_BYTES as usize;
    // The index word is a strip index, tracks first, then submixes (issue #1213 D1).
    let strip_count = ready.strip_count();
    ready.command_wanted.fill(0);
    ready.observe_named.fill(0);
    let refuse = |reason: u32, index: usize| CommandRejection {
        result: match reason {
            // `ObservationUnbound` is "this preparation cannot deliver it", which is exactly what
            // `RESULT_UNSUPPORTED` means; `UnknownTap` is a bad address, like every other unknown.
            COMMAND_REASON_UNSUPPORTED_KIND | COMMAND_REASON_OBSERVATION_UNBOUND => {
                RESULT_UNSUPPORTED
            }
            COMMAND_REASON_BACKPRESSURE => RESULT_BACKPRESSURE,
            _ => RESULT_INVALID_ARGUMENT,
        },
        reason,
        index: index as u32,
    };
    let mut staged_count = 0_usize;
    for index in 0..count {
        let record = &bytes[index * record_bytes..(index + 1) * record_bytes];
        let command = CommandRecord::decode(record).map_err(|reason| refuse(reason, index))?;
        let track = command.track_index as usize;
        if track >= strip_count {
            return Err(refuse(COMMAND_REASON_UNKNOWN_TRACK, index));
        }
        // `inserts` (1) and `console` (3); the retired `simd1` (0) and `simd2` (2) are refused here
        // and never reinterpreted (decision 12, issue #1096).
        let Some(address) = live_effect_address(u32::from(command.rack), command.effect_index)
        else {
            return Err(refuse(COMMAND_REASON_UNKNOWN_RACK, index));
        };
        if ready.rack_effects.get(track).is_none() {
            return Err(refuse(COMMAND_REASON_UNKNOWN_TRACK, index));
        }
        let Some(effect) = ready.effect_slot(track, address) else {
            return Err(refuse(COMMAND_REASON_UNKNOWN_EFFECT, index));
        };
        let Some((descriptor, observed)) = ready
            .effect_instances
            .get(effect)
            .and_then(Option::as_ref)
            .map(|instance| (instance.descriptor, instance.reader.is_some()))
        else {
            return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, index));
        };
        let (tap_index, armed, window_blocks) = command
            .into_observe_record(descriptor, observed)
            .map_err(|reason| refuse(reason, index))?;
        // One submission names each tap once: a record overwrites its tap's whole state and the
        // submission drains in one block, so the second record would be a supersession the engine
        // must not make (D15-2: Observe records are never superseded) and would not fit the tap's
        // queue slot. Refused here, at this record's wire index, before the room pass or any
        // write.
        let bit = 1_u32
            .checked_shl(tap_index)
            .ok_or_else(|| refuse(COMMAND_REASON_MALFORMED, index))?;
        let named = ready
            .observe_named
            .get_mut(effect)
            .ok_or_else(|| refuse(COMMAND_REASON_MALFORMED, index))?;
        if *named & bit != 0 {
            return Err(refuse(COMMAND_REASON_MALFORMED, index));
        }
        *named |= bit;
        let Some(wanted) = ready.command_wanted.get_mut(effect) else {
            return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, index));
        };
        *wanted = wanted.saturating_add(1);
        let Some(entry) = ready.command_decoded.get_mut(staged_count) else {
            return Err(refuse(COMMAND_REASON_MALFORMED, index));
        };
        *entry = StagedObserve {
            effect: effect as u32,
            original_wire_index: index as u32,
            tap_index,
            armed,
            window_blocks,
        };
        staged_count += 1;
    }
    // The room pass: an effect's observation queue holds one subscription per declared tap, and
    // the subscriptions this submission queues must fit it.
    for entry in 0..staged_count {
        let staged = ready.command_decoded[entry];
        let effect = staged.effect as usize;
        let needed = ready.command_wanted[effect];
        let Some(available) = ready.queue_available(effect) else {
            return Err(refuse(
                COMMAND_REASON_UNSUPPORTED_KIND,
                staged.original_wire_index as usize,
            ));
        };
        if needed > available || ready.in_flight[effect].checked_add(needed).is_none() {
            return Err(refuse(
                COMMAND_REASON_BACKPRESSURE,
                staged.original_wire_index as usize,
            ));
        }
    }
    for entry in 0..staged_count {
        let staged = ready.command_decoded[entry];
        ready
            .push_observe(staged, applied_at_sample)
            .expect("the free-room pass checked this record's room");
        ready.in_flight[staged.effect as usize] += 1;
        ready.has_in_flight_commands = true;
    }
    Ok(())
}

/// Resolve one stable selected-observation tuple against the current prepared owner.
///
/// The effect slot id is compared with the prepared control owner, and the observation handle is
/// compared through that same dense slot. This prevents a reordered or replaced same-position
/// effect from accidentally inheriting an old caller tuple.
///
/// `address.track_index` is a strip index (issue #1213 D1): a track, or submix `j` at `T + j`, the
/// same index [`strip_index`] resolves an ID to.
fn observation_selection_for_address<'a>(
    ready: &'a ReadyOwnership,
    address: ObservationAddress,
) -> Result<ObservationSelection<'a>, ObservationReadError> {
    let strip = address.track_index as usize;
    let track_id = ready
        .tracks
        .get(strip)
        .or_else(|| ready.submixes.get(strip.checked_sub(ready.tracks.len())?))
        .map(Box::as_ref)
        .ok_or(ObservationReadError::InvalidSelection)?;
    let effect = ready
        .effect_slot(
            address.track_index as usize,
            LiveEffectAddress {
                rack: address.rack,
                index: address.effect_index,
            },
        )
        .ok_or(ObservationReadError::InvalidSelection)?;
    let instance = ready
        .effect_instances
        .get(effect)
        .and_then(Option::as_ref)
        .ok_or(ObservationReadError::InvalidSelection)?;
    if instance.reader.is_none() {
        return Err(ObservationReadError::Unsupported);
    }
    let effect_slot_id = instance.effect_id.as_ref();
    Ok(ObservationSelection {
        track_id,
        rack: address.rack,
        effect_slot_id,
        tap_id: address.tap_id,
        channels: address.channels,
    })
}

fn resolve_observation(
    ready: &ReadyOwnership,
    selection: &ObservationSelection<'_>,
) -> Result<(usize, usize), ObservationReadError> {
    let track = strip_index(&ready.tracks, &ready.submixes, selection.track_id)
        .ok_or(ObservationReadError::InvalidSelection)?;
    let [pre_insert, inserts, post_insert] = ready
        .rack_effects
        .get(track)
        .copied()
        .ok_or(ObservationReadError::InvalidSelection)?;
    // A console slot is searched over both sections, in slot order; an insert over the inserts.
    let effect_count = match selection.rack {
        LiveEffectRack::Console => pre_insert.saturating_add(post_insert),
        LiveEffectRack::Inserts => inserts,
    };
    let mut addressed = None;
    for index in 0..effect_count {
        let address = LiveEffectAddress {
            rack: selection.rack,
            index,
        };
        let Some(effect) = ready.effect_slot(track, address) else {
            return Err(ObservationReadError::InvalidSelection);
        };
        let Some(observed) = ready.effect_instances.get(effect).and_then(Option::as_ref) else {
            continue;
        };
        if observed.effect_id.as_ref() == selection.effect_slot_id {
            addressed = Some(effect);
            break;
        }
    }
    let effect = addressed.ok_or(ObservationReadError::InvalidSelection)?;
    let handle = ready
        .effect_instances
        .get(effect)
        .and_then(Option::as_ref)
        .filter(|instance| instance.reader.is_some())
        .ok_or(ObservationReadError::Unsupported)?;
    if handle.track_id.as_ref() != selection.track_id
        || handle.effect_id.as_ref() != selection.effect_slot_id
    {
        return Err(ObservationReadError::InvalidSelection);
    }
    let (tap_index, tap) = handle
        .descriptor
        .observations
        .iter()
        .enumerate()
        .find(|(_, tap)| tap.id.0 == selection.tap_id)
        .ok_or(ObservationReadError::InvalidSelection)?;
    if tap.cost != ObservationCost::Resident || tap_index >= u32::BITS as usize {
        return Err(ObservationReadError::Unsupported);
    }
    Ok((effect, tap_index))
}

/// The one rule for whether an armed tap's latest window counts (issue #1534 D5): it counts only
/// if it opened at or after the tap's last subscribe, the `applied_at_sample` that
/// `arm_samples[effect][tap_index]` records. An observation cell conflates and keeps its last
/// window for ever, so a window from before a re-subscribe or a re-arm is a stale reading of the
/// previous arm and is the same as no window. An absent arm sample is `0`. The selected read
/// ([`read_observation_values`]) and the meter frame (`poll_meters`) both call it, so they cannot
/// drift apart; it uses `get` only, because `poll_meters` runs inside `process()`.
fn window_since_arm(
    arm_samples: &[Box<[u64]>],
    effect: usize,
    tap_index: usize,
    window: ObservationWindow,
) -> Option<ObservationWindow> {
    let arm_sample = arm_samples
        .get(effect)
        .and_then(|samples| samples.get(tap_index))
        .copied()
        .unwrap_or(0);
    (window.first_sample >= arm_sample).then_some(window)
}

fn read_observation_values(
    ready: &ReadyOwnership,
    plan: &mut WebPlan,
    sample_rate_hz: u32,
    selection: &ObservationSelection<'_>,
) -> Result<ObservationReadValues, ObservationReadError> {
    let (effect, tap_index) = resolve_observation(ready, selection)?;
    let observed = ready
        .effect_instances
        .get(effect)
        .and_then(Option::as_ref)
        .ok_or(ObservationReadError::Unsupported)?;
    // #1381a D3: the readers are the active plan's companions.
    let handle = plan
        .active_attachment_mut::<RenderCompanions>()
        .zip(observed.reader)
        .and_then(|(companions, reader)| companions.effect_observations.get(reader as usize))
        .ok_or(ObservationReadError::Unsupported)?;
    let armed = ready.observation_armed.get(effect).copied().unwrap_or(0)
        & (1_u32 << u32::try_from(tap_index).unwrap_or(u32::MAX));
    let window = (armed != 0)
        .then(|| {
            handle
                .readers
                .get(tap_index)
                .and_then(|reader| reader.read())
                .and_then(|window| {
                    window_since_arm(&ready.observation_arm_samples, effect, tap_index, window)
                })
        })
        .flatten();
    let status = if armed == 0 {
        ObservationReadStatus::Unarmed
    } else if window.is_some() {
        ObservationReadStatus::Ready
    } else {
        ObservationReadStatus::Pending
    };
    let (left, right) = match (status, window, selection.channels) {
        (ObservationReadStatus::Ready, Some(window), ObservationReadChannels::Left) => {
            (Some(window.left), None)
        }
        (ObservationReadStatus::Ready, Some(window), ObservationReadChannels::Right) => {
            (None, Some(window.right))
        }
        (ObservationReadStatus::Ready, Some(window), ObservationReadChannels::Both) => {
            (Some(window.left), Some(window.right))
        }
        _ => (None, None),
    };
    Ok(ObservationReadValues {
        status,
        sample_rate_hz,
        window,
        left,
        right,
    })
}

/// One published observation magnitude, as the **decibels of reduction** the frame carries.
///
/// The tap declares what crosses the transport (`unit`) and what a consumer reads (`display_unit`,
/// `minimum`, `maximum`); this is the one place the two are reconciled, and it runs once per closed
/// window on the control plane. That placement is the whole of R4: the true-peak limiter's `d` is a
/// linear reduction word and turning it into decibels needs a logarithm, which a render thread may
/// not take.
///
/// The result is clamped into the tap's own declared range, so a consumer never has to guess what
/// a number outside it would have meant.
fn observed_decibels(tap: effect_contract::ObservationDescriptor, magnitude: f32) -> f32 {
    use effect_contract::ParameterUnit;
    let decibels = match tap.unit {
        // Already a decibel magnitude: the declared `PeakMagnitude` fold made it non-negative.
        ParameterUnit::Db => magnitude,
        // `gain = 1 - d`, so the reduction in decibels is `-20 log10(1 - d)`. `d == 1` is total
        // reduction and has no finite decibel value; the declared maximum is what a meter draws.
        ParameterUnit::Linear => {
            if magnitude <= 0.0 {
                0.0
            } else if magnitude >= 1.0 {
                tap.maximum
            } else {
                -20.0 * math::log10f(1.0 - magnitude)
            }
        }
        // No launch tap declares another unit, and a menu that did would be describing something
        // this frame slot has no meaning for.
        _ => 0.0,
    };
    if !decibels.is_finite() || decibels < tap.minimum {
        tap.minimum
    } else if decibels > tap.maximum {
        tap.maximum
    } else {
        decibels
    }
}

/// Master peak sanitization mirrors the builtin meter's finite/normal sample contract. The
/// output PCM remains untouched; only the observer's magnitude is normalized.
fn meter_sample_magnitude(sample: f32) -> f32 {
    if !sample.is_finite() || sample.is_subnormal() {
        0.0
    } else {
        sample.abs()
    }
}

const fn empty_command_report() -> WebCommandReport {
    WebCommandReport {
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

const fn empty_resource_report(backend: u32) -> WebResourceReport {
    WebResourceReport {
        struct_size: RESOURCE_REPORT_BYTES,
        abi_version: ABI_VERSION,
        sample_rate_hz: 0,
        quantum_frames: 0,
        backend,
        reserved0: [0; 3],
        options_bytes: size_of::<WebBootOptions>() as u64,
        status_bytes: size_of::<WebStatus>() as u64,
        session_document_bytes: 0,
        diagnostic_bytes: 0,
        id_staging_bytes: 0,
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
        observation_retained_bytes: 0,
        reserved: [0; 3],
    }
}

/// One typed boot refusal. The FFI copies its bounded diagnostic over the staged document prefix.
#[derive(Debug, Eq, PartialEq)]
pub struct BootFailure {
    result: u32,
    diagnostic: Vec<u8>,
}

impl BootFailure {
    /// Frozen boot result code.
    #[must_use]
    pub const fn result(&self) -> u32 {
        self.result
    }

    /// `code\tpath\n` diagnostic bytes.
    #[must_use]
    pub fn diagnostic(&self) -> &[u8] {
        &self.diagnostic
    }

    fn fixed(result: u32, code: &str) -> Self {
        Self {
            result,
            diagnostic: fixed_diagnostic(code),
        }
    }

    fn document(diagnostic: Vec<u8>) -> Self {
        Self {
            result: RESULT_REFUSED_DOCUMENT,
            diagnostic,
        }
    }

    fn projected_budget(code: &str, projected: u64, budget: u64) -> Self {
        Self {
            result: RESULT_REFUSED_BUDGET,
            diagnostic: format!(
                "{code}\t$.maximum_memory_bytes[projected_bytes={projected},budget_bytes={budget}]\n"
            )
            .into_bytes(),
        }
    }

    fn exact_budget(code: &str, exact: u64, budget: u64) -> Self {
        Self {
            result: RESULT_REFUSED_BUDGET,
            diagnostic: format!(
                "{code}\t$.maximum_memory_bytes[exact_bytes={exact},budget_bytes={budget}]\n"
            )
            .into_bytes(),
        }
    }
}

impl From<Vec<u8>> for BootFailure {
    fn from(diagnostic: Vec<u8>) -> Self {
        Self::document(diagnostic)
    }
}

#[derive(Clone, Copy)]
struct PreparedBufferProjection {
    source_samples: u64,
    command_records: u32,
    report: WebResourceReport,
}

fn spectrum_collection_staging_bytes(
    spectrum_request: Option<&SpectrumPreparationRequest>,
) -> Result<(u64, u64), BootFailure> {
    let Some(SpectrumPreparationRequest::Collection(request)) = spectrum_request else {
        return Ok((0, 0));
    };
    let entry_count = u64::try_from(request.entries.len())
        .map_err(|_| BootFailure::fixed(RESULT_REFUSED_BUDGET, "host.budget.arithmetic"))?;
    let entry_bytes = entry_count
        .checked_mul(size_of::<WebSpectrumCollectionEntry>() as u64)
        .ok_or_else(|| BootFailure::fixed(RESULT_REFUSED_BUDGET, "host.budget.arithmetic"))?;
    let target_id_bytes = request.entries.iter().try_fold(0_u64, |total, entry| {
        let id_bytes = match &entry.target {
            SpectrumTarget::TrackPostInputBuiltins(id)
            | SpectrumTarget::TrackPostMatrix(id)
            | SpectrumTarget::Output(id) => u64::try_from(id.len()).ok(),
        }?;
        total.checked_add(id_bytes)
    });
    let Some(target_id_bytes) = target_id_bytes else {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_BUDGET,
            "host.budget.arithmetic",
        ));
    };
    Ok((entry_bytes, target_id_bytes))
}

fn project_buffers(
    document_bytes: u32,
    sample_rate_hz: u32,
    quantum_frames: u32,
    maximum_source_channels: u32,
    id_staging_bytes: u64,
    options: WebBootOptions,
    spectrum_staging: (bool, (u64, u64)),
) -> Result<PreparedBufferProjection, BootFailure> {
    let (
        spectrum_configured,
        (spectrum_collection_entry_bytes, spectrum_collection_target_id_bytes),
    ) = spectrum_staging;
    let arithmetic = || BootFailure::fixed(RESULT_REFUSED_BUDGET, "host.budget.arithmetic");
    let source_samples = u64::from(maximum_source_channels)
        .checked_mul(u64::from(quantum_frames))
        .ok_or_else(arithmetic)?;
    let source_pcm_bytes = source_samples.checked_mul(4).ok_or_else(arithmetic)?;
    let output_pcm_bytes = u64::from(quantum_frames)
        .checked_mul(8)
        .ok_or_else(arithmetic)?;
    // The command staging exists exactly when the boot asked for observation taps (#1565): kinds 7
    // and 8 are the only records it holds.
    let command_records = if options.live_control_observation_taps == 0 {
        0
    } else {
        MAXIMUM_COMMAND_RECORDS
    };
    let command_bytes = u64::from(command_records)
        .checked_mul(u64::from(COMMAND_RECORD_BYTES))
        .ok_or_else(arithmetic)?;
    let plane_reference_bytes = u64::from(maximum_source_channels)
        .checked_mul(size_of::<&[f32]>() as u64)
        .ok_or_else(arithmetic)?;
    let host_shell_bytes =
        u64::try_from(size_of::<AudioWorkletEngineHost>()).map_err(|_| arithmetic())?;
    let fixed_metadata = host_shell_bytes
        .checked_sub(u64::from(BOOT_OPTIONS_BYTES) + u64::from(STATUS_BYTES))
        .ok_or_else(arithmetic)?;
    let bridge_metadata = fixed_metadata
        .checked_add(plane_reference_bytes)
        .and_then(|bytes| bytes.checked_add(crate::ffi::observation_staging_retained_bytes()))
        .and_then(|bytes| bytes.checked_add(crate::ffi::live_response_staging_retained_bytes()))
        .and_then(|bytes| {
            bytes.checked_add(crate::ffi::spectrum_staging_retained_bytes(
                spectrum_configured,
                spectrum_collection_entry_bytes,
                spectrum_collection_target_id_bytes,
            ))
        })
        .and_then(|bytes| {
            bytes.checked_add(crate::ffi::spectrum_analysis_history_retained_bytes(
                spectrum_configured,
            ))
        })
        .ok_or_else(arithmetic)?;
    let rows = [
        u64::from(BOOT_OPTIONS_BYTES),
        u64::from(STATUS_BYTES),
        u64::from(document_bytes),
        u64::from(DIAGNOSTIC_BYTES),
        id_staging_bytes,
        source_pcm_bytes,
        output_pcm_bytes,
        command_bytes,
        CONTROL_FRAME_BYTES,
        bridge_metadata,
    ];
    let retained = rows.into_iter().try_fold(0_u64, |total, row| {
        total.checked_add(row).ok_or_else(arithmetic)
    })?;
    // `bridge_metadata` is the sum of retained payloads, not one allocation. Keep the largest
    // row below to the individually allocated fixed buffers and add the staging helper's exact
    // largest payload separately.
    let largest = rows[..rows.len() - 1]
        .iter()
        .copied()
        .max()
        .unwrap_or(0)
        .max(host_shell_bytes)
        .max(plane_reference_bytes)
        .max(crate::ffi::observation_staging_largest_allocation_bytes())
        .max(crate::ffi::live_response_staging_largest_allocation_bytes())
        .max(crate::ffi::spectrum_staging_largest_allocation_bytes(
            spectrum_configured,
            spectrum_collection_entry_bytes,
            spectrum_collection_target_id_bytes,
        ))
        .max(crate::ffi::spectrum_analysis_history_retained_bytes(
            spectrum_configured,
        ));
    let mut report = empty_resource_report(selected_backend());
    report.sample_rate_hz = sample_rate_hz;
    report.quantum_frames = quantum_frames;
    report.session_document_bytes = u64::from(document_bytes);
    report.diagnostic_bytes = u64::from(DIAGNOSTIC_BYTES);
    report.id_staging_bytes = id_staging_bytes;
    report.source_pcm_staging_bytes = source_pcm_bytes;
    report.output_pcm_bytes = output_pcm_bytes;
    report.bridge_metadata_bytes = bridge_metadata;
    report.bridge_retained_bytes = retained;
    report.largest_bridge_allocation_bytes = largest;
    report.largest_named_allocation_bytes = largest;
    Ok(PreparedBufferProjection {
        source_samples,
        command_records,
        report,
    })
}

fn allocate_buffers(projection: PreparedBufferProjection) -> Result<PreparedBuffers, BootFailure> {
    let allocation = |_| BootFailure::fixed(RESULT_REFUSED_BUDGET, "web.resource.allocation");
    Ok(PreparedBuffers {
        diagnostic: boxed_zero_u8(DIAGNOSTIC_BYTES).map_err(allocation)?,
        source_id: boxed_zero_u8(
            u32::try_from(projection.report.id_staging_bytes)
                .map_err(|_| BootFailure::fixed(RESULT_REFUSED_BUDGET, "web.resource.platform"))?,
        )
        .map_err(allocation)?,
        source_pcm: boxed_zero_f32(projection.source_samples).map_err(allocation)?,
        output_pcm: boxed_zero_f32(u64::from(projection.report.quantum_frames) * 2)
            .map_err(allocation)?,
        command: boxed_zero_u8(projection.command_records * COMMAND_RECORD_BYTES)
            .map_err(allocation)?,
        control_request: boxed_zero_u8(CONTROL_FRAME_BYTES as u32).map_err(allocation)?,
        plane_references: boxed_uninit_planes(
            u32::try_from(projection.source_samples / u64::from(projection.report.quantum_frames))
                .map_err(|_| BootFailure::fixed(RESULT_REFUSED_BUDGET, "web.resource.platform"))?,
        )
        .map_err(allocation)?,
    })
}

fn validate_options(mut options: WebBootOptions) -> Result<WebBootOptions, BootFailure> {
    if options.struct_size == 0 && options.abi_version == 0 {
        options.struct_size = BOOT_OPTIONS_BYTES;
        options.abi_version = ABI_VERSION;
    } else if options.struct_size != BOOT_OPTIONS_BYTES {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_OPTIONS,
            "web.options.struct_size",
        ));
    } else if options.abi_version != ABI_VERSION {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_OPTIONS,
            "web.options.abi_version",
        ));
    }
    // Issue #143: a master designation must be a plausible track index. The session-level range
    // check is the facade's; this one only refuses what cannot be a track index at all.
    if options.reserved0 != 0 {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_OPTIONS,
            "web.options.reserved0",
        ));
    }
    // #1565: the word at offset 32 was the live-command queue word. It is reserved now, and a
    // writer that still sets it is refused rather than ignored.
    if options.reserved1 != 0 {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_OPTIONS,
            "web.options.reserved1",
        ));
    }
    if options.live_control_observation_taps > u64::from(MAXIMUM_OBSERVATION_TAPS)
        || options.live_control_master_track_plus_one > u64::from(u32::MAX)
        || (options.live_control_master_track_plus_one != 0
            && options.live_control_observation_taps == 0)
        || options.live_control_meter_blocks > u64::from(u32::MAX)
    {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_OPTIONS,
            "web.options.live_controls",
        ));
    }
    Ok(options)
}

/// Project all retained declaration/runtime bytes whose allocation shape is known after compile.
///
/// The compiled-model row is the session estimator's post-canonical exact row. The runtime row is
/// re-derived from the ring boot actually chose rather than the soon-to-be-removed document limits
/// word. Bridge storage is the exact layout projection above. Preparation's graph/effect/builtin
/// subcompilers receive this same total budget and retain their existing checked pre-allocation
/// estimators, so no structural count ceiling is introduced here.
fn projected_retained_bytes(
    compiled: &CompiledSession,
    source_ring_frames: u32,
    bridge_retained_bytes: u64,
) -> Result<u64, BootFailure> {
    let arithmetic = || BootFailure::fixed(RESULT_REFUSED_BUDGET, "host.budget.arithmetic");
    let source_ring_bytes = compiled
        .normalized_model()
        .sources
        .iter()
        .try_fold(0_u64, |total, source| {
            let bytes = u64::from(source_ring_frames)
                .checked_mul(u64::from(source.channels))?
                .checked_mul(size_of::<f32>() as u64)?;
            total.checked_add(bytes)
        })
        .ok_or_else(arithmetic)?;
    let estimate = compiled.resource_estimate();
    [
        bridge_retained_bytes,
        estimate.compiled_model_bytes,
        estimate.queue_bytes,
        source_ring_bytes,
    ]
    .into_iter()
    .try_fold(0_u64, |total, row| {
        total.checked_add(row).ok_or_else(arithmetic)
    })
}

/// Exact aggregate retained bytes reported by the fully prepared engine plus the browser bridge.
///
/// `source_total_bytes` owns the source-ring PCM charge plus source overhead. The graph row does
/// not repeat it, and `bridge_retained_bytes` already includes the compiled model, the facade's
/// control table/ID arena and the prepared plan's state inventory. These three disjoint rows are
/// therefore the complete retained set.
fn exact_retained_bytes(report: &WebResourceReport) -> Result<u64, BootFailure> {
    report
        .bridge_retained_bytes
        .checked_add(report.graph_session_plus_plan_bytes)
        .and_then(|total| total.checked_add(report.source_total_bytes))
        .ok_or_else(|| BootFailure::fixed(RESULT_REFUSED_BUDGET, "host.budget.arithmetic"))
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

/// The result word of a source submission or seek the control plane's source routing refused
/// (#1381a D2): the facade's typed rejection as [`source_result`] maps it, or the control plane's
/// own epoch failure, which is internal.
fn source_failure_result(failure: SourceFailure) -> u32 {
    match failure {
        SourceFailure::Control(error) => source_result(error),
        SourceFailure::Internal => RESULT_INTERNAL,
    }
}

/// Translate document-derived shape plus host policy into the shared preparation caps.
///
/// Count ceilings are intentionally unbounded: boot v1 has one physics gate, not structural caps.
/// Byte ceilings are the same effective memory budget, so every subsystem retains its own checked
/// pre-allocation arithmetic while the aggregate projection is checked before preparation starts.
fn prepare_caps(
    compiled: &CompiledSession,
    options: WebBootOptions,
    source_ring_frames: u32,
    memory_budget: u64,
) -> HostPrepareCaps {
    let sample_rate_hz = if options.require_sample_rate_hz == 0 {
        compiled.sample_rate().0
    } else {
        options.require_sample_rate_hz
    };
    let quantum_frames = if options.require_quantum_frames == 0 {
        compiled.quantum().0
    } else {
        options.require_quantum_frames
    };
    let automation_spans = compiled
        .normalized_model()
        .automation
        .iter()
        .try_fold(0_u32, |total, lane| {
            total.checked_add(u32::try_from(lane.segments.len()).ok()?)
        })
        .unwrap_or(u32::MAX)
        .max(1);
    HostPrepareCaps {
        shape: HostShapePolicy::Exact {
            sample_rate_hz,
            quantum_frames,
        },
        source_ring_frames,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: automation_spans,
        maximum_tracks: u64::MAX,
        maximum_submixes: u64::MAX,
        maximum_vcas: MAXIMUM_VCAS_BOUNDED,
        maximum_vca_reach_pairs: MAXIMUM_VCA_REACH_PAIRS,
        maximum_sources: u64::MAX,
        maximum_routes: u64::MAX,
        maximum_effects: u64::MAX,
        maximum_graph_session_plus_plan_bytes: memory_budget,
        maximum_source_total_bytes: memory_budget,
        maximum_source_overhead_bytes: memory_budget,
        maximum_effect_state_bytes: memory_budget,
        maximum_effect_scratch_bytes: memory_budget,
        maximum_builtin_retained_bytes: memory_budget,
        maximum_named_allocation_bytes: memory_budget,
        maximum_meter_streams: u64::MAX,
        maximum_meter_items: u64::MAX,
        maximum_meter_bytes: memory_budget,
    }
}

/// The control plane's limits for the browser's session (#1381a D1): the preparation caps the boot
/// computed, as plain values, and the control plane's own fixed allocations.
///
/// The byte ceilings are the memory budget, as the caps' are, and counts are unbounded: the
/// boot's one physics gate is the budget. The control frame, the replay cache and the diagnostic
/// are the control plane's fixed allocations; they are small and the same for every session, and
/// the budget's exact check ([`exact_retained_bytes`]) charges them in the bridge rows.
fn control_limits(caps: &HostPrepareCaps, memory_budget: u64) -> ControlLimits {
    ControlLimits {
        source_ring_frames: caps.source_ring_frames,
        maximum_automation_spans_per_block: caps.maximum_automation_spans_per_block,
        maximum_document_bytes: u64::from(MAXIMUM_DOCUMENT_BYTES),
        maximum_diagnostic_bytes: u64::from(DIAGNOSTIC_BYTES),
        maximum_tracks: caps.maximum_tracks,
        maximum_sources: caps.maximum_sources,
        maximum_routes: caps.maximum_routes,
        maximum_effects: caps.maximum_effects,
        maximum_graph_session_plus_plan_bytes: caps.maximum_graph_session_plus_plan_bytes,
        maximum_source_total_bytes: caps.maximum_source_total_bytes,
        maximum_source_overhead_bytes: caps.maximum_source_overhead_bytes,
        maximum_effect_state_bytes: caps.maximum_effect_state_bytes,
        maximum_effect_scratch_bytes: caps.maximum_effect_scratch_bytes,
        maximum_builtin_retained_bytes: caps.maximum_builtin_retained_bytes,
        maximum_control_retained_bytes: memory_budget,
        maximum_named_allocation_bytes: caps.maximum_named_allocation_bytes,
        maximum_meter_streams: caps.maximum_meter_streams,
        maximum_meter_items: caps.maximum_meter_items,
        maximum_meter_bytes: caps.maximum_meter_bytes,
        maximum_control_frame_bytes: CONTROL_FRAME_BYTES,
        maximum_replay_bytes: CONTROL_REPLAY_BYTES,
        maximum_replay_entries: CONTROL_REPLAY_ENTRIES,
        maximum_submixes: caps.maximum_submixes,
        maximum_vcas: caps.maximum_vcas,
        maximum_vca_reach_pairs: caps.maximum_vca_reach_pairs,
    }
}

/// Bytes of the control plane's one control frame (#1381a): the request and response scratch the
/// session keeps for the protocol the browser serves through the core.
const CONTROL_FRAME_BYTES: u64 = 16 << 10;
/// Bytes of the control plane's replay cache.
const CONTROL_REPLAY_BYTES: u64 = 64 << 10;
/// Entries of the control plane's replay cache.
const CONTROL_REPLAY_ENTRIES: u64 = 32;

/// A refused boot preparation as the browser reports it (#1381a): the control plane's diagnostic
/// bytes, and the result code the refusal's class chooses: a resource or platform refusal is a
/// budget refusal and the rest are document refusals, as boot chose them before the control plane
/// prepared the plan. The class is what the preparer saw; a refusal it
/// did not see is the control plane's own, and a resource fault is a budget refusal.
fn boot_refusal(failure: &CompileFailure, record: &Mutex<BootRecord>) -> BootFailure {
    let rejection = record.lock().ok().and_then(|record| record.rejection);
    let result = match (rejection, failure) {
        (Some(PrepareRejection::Resource | PrepareRejection::Platform), _)
        | (None, CompileFailure::Resource(_)) => RESULT_REFUSED_BUDGET,
        (Some(_), _) | (None, CompileFailure::Diagnostics(_)) => RESULT_REFUSED_DOCUMENT,
    };
    BootFailure {
        result,
        diagnostic: failure.diagnostic().into_owned(),
    }
}

/// The browser's preparation (issue #1400 D5): the live request's meters, the three branches
/// (no spectrum, one spectrum capture, a spectrum collection) and the host-core entry each one
/// calls, at boot (`successor` is `None`) and as a successor of a running plan.
///
/// Every branch delivers controls concurrently and none calls a `_between_render_calls` entry, so
/// the same function serves a control plane that prepares while another thread renders. A
/// successor prepares every source it leaves unchanged vacant, to carry the displaced plan's ring.
/// Issue #1209 D3: one `PostMatrix` sample-peak meter per strip, the tracks then the submixes, in
/// the strip order host-core's addressing authority uses, so `meters[i]` is strip `i`'s peak pair;
/// none when the request has no meter period.
fn prepare_web_host(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: &HostLiveControlRequest,
    spectrum: Option<&SpectrumPreparationRequest>,
    lanes: HostLiveLanes,
    superseded: &host_core::LiveValuesSuperseded,
    successor: Option<SuccessorBase<'_>>,
) -> Result<
    (
        PreparedHost,
        host_core::HostLiveControlHandles,
        Option<PreparedSpectrumCaptures>,
    ),
    PrepareDiagnostics,
> {
    match spectrum {
        Some(SpectrumPreparationRequest::Single(request)) => {
            let (host, handles, capture) = match successor {
                Some(base) => {
                    host_core::prepare_host_runtime_with_live_controls_and_spectrum_successor(
                        compiled,
                        caps,
                        live_controls,
                        request,
                        lanes,
                        superseded,
                        base,
                    )
                }
                None => prepare_host_runtime_with_live_controls_and_spectrum(
                    compiled,
                    caps,
                    live_controls,
                    request,
                    lanes,
                    superseded,
                ),
            }?;
            Ok((
                host,
                handles,
                Some(PreparedSpectrumCaptures::Single(capture)),
            ))
        }
        Some(SpectrumPreparationRequest::Collection(request)) => {
            let (host, handles, capture) = match successor {
                Some(base) => {
                    host_core::prepare_host_runtime_with_live_controls_and_spectrum_collection_successor(
                        compiled,
                        caps,
                        live_controls,
                        request,
                        lanes,
                        superseded,
                        base,
                    )
                }
                None => host_core::prepare_host_runtime_with_live_controls_and_spectrum_collection(
                    compiled,
                    caps,
                    live_controls,
                    request,
                    lanes,
                    superseded,
                ),
            }?;
            Ok((
                host,
                handles,
                Some(PreparedSpectrumCaptures::Collection(capture)),
            ))
        }
        None => {
            let meters: Vec<HostMeterRequest> = if live_controls.meter_period_frames.is_some() {
                let model = compiled.normalized_model();
                model
                    .tracks
                    .iter()
                    .map(|track| track.id.as_str())
                    .chain(model.submixes.iter().map(|submix| submix.id.as_str()))
                    .map(|strip| HostMeterRequest {
                        strip_id: strip.into(),
                        tap: MeterTap::PostMatrix,
                        metrics: MeterMetricSet::SAMPLE_PEAK,
                    })
                    .collect()
            } else {
                Vec::new()
            };
            let (host, handles) = match successor {
                Some(base) => host_core::prepare_host_runtime_with_selected_meters_successor(
                    compiled,
                    caps,
                    live_controls,
                    &meters,
                    lanes,
                    superseded,
                    base,
                ),
                None => host_core::prepare_host_runtime_with_selected_meters(
                    compiled,
                    caps,
                    live_controls,
                    &meters,
                    lanes,
                    superseded,
                ),
            }?;
            Ok((host, handles, None))
        }
    }
}

/// The browser as a control-plane adapter (#1381a): its plan resource table stores the control
/// plane's own row, and it keeps no per-session handle beside the two halves.
///
/// The row is the control plane's [`PlanResources`] unchanged, because the browser's resource
/// report projects from it directly (`compile_ready`). The adapter's own fixed allocations are
/// the browser bridge's, which [`WebResourceReport`] charges in its bridge rows.
pub struct WebAdapter;

impl ControlAdapter for WebAdapter {
    type Row = PlanResources;
    const ADAPTER_ALLOCATIONS: &'static [u64] = &[];
}

/// The control-protocol session the browser host owns (#1381a): the committed model at its boot
/// revision, the source, strip and effect producers, the plan publisher and the retirer.
type WebSession = SessionState<WebAdapter>;
/// The render half of the browser's plan exchange: the plan owner that renders and adopts.
type WebPlan = PlanState<WebAdapter>;

/// The render-side halves a browser plan's preparation returns beside the plan (issue #1400 D5):
/// the meter consumers, the effect observation handles and the spectrum capture.
///
/// They travel inside the plan as its [`HostAttachment`], so a plan the control plane prepares
/// carries everything the render side reads, and they drop with the plan on the retirement path.
/// The browser renders through the plan owner (#1381a D4) and reaches the active plan's companions
/// through `PlanState::active_attachment_mut`: the meter poll and the observation reads read the
/// readers there. The spectrum capture is the one companion the host does not read there:
/// [`SpectrumService`] owns its captures, so boot moves the capture out into it
/// (`take_spectrum_capture`) until the spectrum readers move to the control plane (#1596).
///
/// Field order is the drop order: the spectrum capture, then the meter consumers, then the effect
/// observation handles, the relative order the same three fields have in `ReadyOwnership`. No one
/// of them reads another while it drops; the order is kept so the two owners release alike.
pub struct RenderCompanions {
    spectrum_capture: Option<PreparedSpectrumCaptures>,
    /// One consumer per selected meter, in strip order; empty when metering is off.
    meters: Vec<MeterConsumer>,
    /// One reader set per observed effect instance, in `HostLiveControlHandles` order.
    effect_observations: Vec<EffectObservationHandle>,
}

impl RenderCompanions {
    /// The meter consumers: one per selected meter, in strip order.
    #[must_use]
    pub fn meters(&self) -> &[MeterConsumer] {
        &self.meters
    }

    /// The effect observation handles, one reader set per observed effect instance.
    #[must_use]
    pub fn effect_observations(&self) -> &[EffectObservationHandle] {
        &self.effect_observations
    }

    /// The heap the attachment itself holds for the plan's lifetime, and its largest single
    /// allocation: the box, and the buffers of the two vectors. What the consumers, the handles
    /// and the capture own is host-core's report's, which already counts it; `None` on overflow.
    fn retained_bytes(&self) -> Option<(u64, u64)> {
        // Every field is named, so a field added to the struct fails to compile here until its
        // heap is charged (or declared host-core's, as the capture is).
        let Self {
            spectrum_capture: _,
            meters,
            effect_observations,
        } = self;
        let shell = u64::try_from(size_of::<Self>()).ok()?;
        let meters = u64::try_from(meters.capacity())
            .ok()?
            .checked_mul(u64::try_from(size_of::<MeterConsumer>()).ok()?)?;
        let observations = u64::try_from(effect_observations.capacity())
            .ok()?
            .checked_mul(u64::try_from(size_of::<EffectObservationHandle>()).ok()?)?;
        let total = shell.checked_add(meters)?.checked_add(observations)?;
        Some((total, shell.max(meters).max(observations)))
    }

    /// Whether the plan was prepared with a spectrum capture (one, or a collection).
    #[must_use]
    pub fn has_spectrum_capture(&self) -> bool {
        self.spectrum_capture.is_some()
    }

    /// Move the spectrum capture out, once, to the host's spectrum service (#1381a). Control side
    /// only: boot calls it before the first render.
    fn take_spectrum_capture(&mut self) -> Option<PreparedSpectrumCaptures> {
        self.spectrum_capture.take()
    }
}

#[cfg(feature = "test-support")]
std::thread_local! {
    static COMPANION_DROPS: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
}

/// How many [`RenderCompanions`] dropped on this thread (#1381a gate 2: the retired plan's
/// attachment drops in the service step, on the control side, and never in a render call).
#[cfg(feature = "test-support")]
#[must_use]
pub fn companion_drops_on_this_thread() -> u64 {
    COMPANION_DROPS.with(core::cell::Cell::get)
}

#[cfg(feature = "test-support")]
impl Drop for RenderCompanions {
    fn drop(&mut self) {
        COMPANION_DROPS.with(|drops| drops.set(drops.get() + 1));
    }
}

/// What a boot preparation leaves for the host (#1381a D1): the facts of the first plan that
/// `PlanResources` and the attachment do not carry. The preparer writes it, boot reads it.
#[derive(Default)]
struct BootRecord {
    /// The engine-owned report of the boot plan, after the attachment's rows were charged.
    report: Option<HostPrepareReport>,
    /// The designated master strip, echoed back by preparation after validation.
    master_track: Option<u32>,
    /// The session's `live_values_superseded` count, shared with every cell reader of its plans.
    superseded: Option<host_core::LiveValuesSuperseded>,
    /// The class of the preparation refusal, so a refused boot keeps its result code.
    rejection: Option<PrepareRejection>,
}

impl HostAttachment for RenderCompanions {
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}

/// The diagnostic code of a command frame whose transaction needs a plan rebuild, which the
/// browser cannot apply in place yet (issue #1562a D9; #1562b removes the refusal).
pub const REBUILD_PENDING: &str = "web.apply.rebuild_pending";

/// The browser's [`control_plane::RuntimePreparer`] (issue #1400 D5): `prepare_web_host` for the
/// session's boot options and spectrum request, with the render-side companions moved into the
/// plan.
///
/// It attaches the live lanes the control plane chose ([`controls::live_lanes`], passed in as the
/// `lanes` argument), so the boot word chooses no lane here. It ignores `bound_cache`: no browser
/// entry reads the input-design bound cache (#1602).
pub struct WebRuntimePreparer {
    options: WebBootOptions,
    spectrum: Option<SpectrumPreparationRequest>,
    /// Where the boot preparation leaves its facts for the host (#1381a), or `None` for a preparer
    /// that is only called.
    record: Option<Arc<Mutex<BootRecord>>>,
    /// Whether the command path takes a structural transaction (issue #1562a D9). It is `false`:
    /// the browser has no in-place structural edit until #1562b, so a frame that classifies as a
    /// rebuild is refused `web.apply.rebuild_pending` before anything is committed. Only the
    /// test-support republish of #1381a's gates opens it, for one call.
    allow_rebuild: Arc<AtomicBool>,
}

impl WebRuntimePreparer {
    /// A preparer for a session booted with `options` and the optional `spectrum` request.
    #[must_use]
    pub fn new(options: WebBootOptions, spectrum: Option<SpectrumPreparationRequest>) -> Self {
        Self {
            options,
            spectrum,
            record: None,
            allow_rebuild: Arc::new(AtomicBool::new(false)),
        }
    }

    /// The same preparer, leaving its boot facts in `record`.
    fn recording(mut self, record: Arc<Mutex<BootRecord>>) -> Self {
        self.record = Some(record);
        self
    }
}

impl control_plane::RuntimePreparer for WebRuntimePreparer {
    fn refuse_rebuild(&self) -> Option<&'static str> {
        (!self.allow_rebuild.load(Ordering::Acquire)).then_some(REBUILD_PENDING)
    }

    fn prepare(
        &self,
        compiled: &CompiledSession,
        caps: &HostPrepareCaps,
        lanes: HostLiveLanes,
        successor: Option<SuccessorBase<'_>>,
        _bound_cache: &mut control_plane::InputBoundCache<'_>,
        superseded: &host_core::LiveValuesSuperseded,
    ) -> Result<(PreparedHost, host_core::HostLiveControlHandles), PrepareDiagnostics> {
        let boot = successor.is_none();
        let prepared = self.prepare_plan(compiled, caps, lanes, successor, superseded, boot);
        if let (Err(failure), Some(record)) = (&prepared, &self.record)
            && let Ok(mut record) = record.lock()
        {
            record.rejection = Some(failure.kind());
        }
        prepared
    }
}

impl WebRuntimePreparer {
    /// The preparation itself. `boot` is whether it prepares the session's first plan, whose facts
    /// the host's boot reads from the record.
    fn prepare_plan(
        &self,
        compiled: &CompiledSession,
        caps: &HostPrepareCaps,
        lanes: HostLiveLanes,
        successor: Option<SuccessorBase<'_>>,
        superseded: &host_core::LiveValuesSuperseded,
        boot: bool,
    ) -> Result<(PreparedHost, host_core::HostLiveControlHandles), PrepareDiagnostics> {
        let live_controls =
            live_control_request(self.options, compiled.quantum().0).ok_or_else(|| {
                PrepareDiagnostics::fixed(PrepareRejection::Session, "web.live_controls.config")
            })?;
        // Observation taps ride the effect lanes (#1254 D2): an app that asked for taps gets the
        // effect lanes whatever the session's live rows chose, as before the control plane chose
        // the lanes, so a session with no effect to observe still boots.
        let lanes = HostLiveLanes {
            effects: lanes.effects || live_controls.observation_taps != 0,
            ..lanes
        };
        let (mut host, mut handles, spectrum_capture) = prepare_web_host(
            compiled,
            caps,
            &live_controls,
            self.spectrum.as_ref(),
            lanes,
            superseded,
            successor,
        )?;
        let companions = RenderCompanions {
            spectrum_capture,
            meters: core::mem::take(&mut handles.meters),
            effect_observations: core::mem::take(&mut handles.effect_observations),
        };
        // The attachment is the plan's own allocation, so the plan's rows charge it: the control
        // plane builds its plan resource row from this report, and nothing else would count the
        // box or the two vectors the companions keep alive for the plan's lifetime.
        let (retained, largest) = companions.retained_bytes().ok_or_else(|| {
            PrepareDiagnostics::fixed(PrepareRejection::Platform, "web.resource.arithmetic")
        })?;
        charge_attachment(&mut host.report, caps, retained, largest)?;
        host.plan.set_host_attachment(Box::new(companions));
        if boot
            && let Some(record) = &self.record
            && let Ok(mut record) = record.lock()
        {
            record.report = Some(host.report);
            record.master_track = handles.master_track;
            record.superseded = Some(handles.live_values_superseded.clone());
        }
        Ok((host, handles))
    }
}

/// Charge the render companions' own heap to the plan's rows, after checking the charged total
/// against the caps the plan was prepared under, so a plan that fits only without its companions is
/// refused before they are boxed.
///
/// The cap on `graph_session_plus_plan_bytes` governs the graph, the plan and the compiled model
/// together (`HostPrepareCaps`), and the report's graph row leaves the model out, so the model is
/// added for the comparison. Host-core's own admission also counts the effect and route controls,
/// which neither row holds, so it refuses first for a session it would refuse; this is the check
/// of what only the browser adds. A refusal leaves the report as it was.
fn charge_attachment(
    report: &mut host_core::HostPrepareReport,
    caps: &HostPrepareCaps,
    retained: u64,
    largest: u64,
) -> Result<(), PrepareDiagnostics> {
    let arithmetic =
        || PrepareDiagnostics::fixed(PrepareRejection::Platform, "web.resource.arithmetic");
    let graph_session_plus_plan = report
        .graph_session_plus_plan_bytes
        .checked_add(retained)
        .ok_or_else(arithmetic)?;
    let graph_incremental_plan = report
        .graph_incremental_plan_bytes
        .checked_add(retained)
        .ok_or_else(arithmetic)?;
    let governed = graph_session_plus_plan
        .checked_add(report.session_model_bytes)
        .ok_or_else(arithmetic)?;
    if governed > caps.maximum_graph_session_plus_plan_bytes {
        return Err(PrepareDiagnostics::fixed(
            PrepareRejection::Resource,
            "host.graph.resource.limit",
        ));
    }
    let largest_allocation = report.largest_engine_allocation_bytes.max(largest);
    if largest_allocation > caps.maximum_named_allocation_bytes {
        return Err(PrepareDiagnostics::fixed(
            PrepareRejection::Resource,
            "host.resource.limit",
        ));
    }
    report.graph_session_plus_plan_bytes = graph_session_plus_plan;
    report.graph_incremental_plan_bytes = graph_incremental_plan;
    report.largest_engine_allocation_bytes = largest_allocation;
    Ok(())
}

/// Prepare the session through the control plane, then fold its report into the frozen browser
/// report (#1381a D1).
///
/// The control plane's `compile_children` prepares the boot plan through [`WebRuntimePreparer`]
/// and owns the producers from then on: the sources, the strips' and effects' cells, the plan's
/// revision gate and the exchange's control ends. What this function builds is the browser's own:
/// the dense tables that address the readers in the plan's attachment, the live-control mirrors
/// of the retained command path, and the report. Engine-owned rows come from the plan's
/// [`PlanResources`] row and the preparer's [`BootRecord`]; the browser bridge's own rows
/// (staging buffers, the host shell, the plane-reference table) were already computed by
/// `prepare_buffers`, and the three browser caps are applied here because they are the *browser's*
/// caps on the shared report.
fn compile_ready(
    document: &str,
    caps: &HostPrepareCaps,
    options: WebBootOptions,
    memory_budget: u64,
    mut report: WebResourceReport,
    spectrum_request: Option<&SpectrumPreparationRequest>,
) -> Result<(ReadyOwnership, WebPlan, WebResourceReport), BootFailure> {
    // The command staging, and with it the observation records, exist exactly when the boot asked
    // for observation taps (#1565).
    let commands_enabled = options.live_control_observation_taps != 0;
    // #1312 D2, #1399: the session's `live_values_superseded` count is the control plane's, made
    // once before any plan is prepared; the preparer leaves a handle on it in the record, which the
    // status reads. #1400 D5: the three boot shapes are `prepare_web_host`'s.
    let record = Arc::new(Mutex::new(BootRecord::default()));
    let preparer =
        WebRuntimePreparer::new(options, spectrum_request.cloned()).recording(Arc::clone(&record));
    #[cfg(feature = "test-support")]
    let allow_rebuild = Arc::clone(&preparer.allow_rebuild);
    let limits = control_limits(caps, memory_budget);
    let control_plane::CompiledChildren {
        session: control,
        mut plan,
    } = control_plane::compile_children::<WebAdapter>(document, limits, Box::new(preparer), None)
        .map_err(|failure| boot_refusal(&failure, &record))?;
    let (engine, master_track, live_values_superseded) = {
        let mut record = record
            .lock()
            .map_err(|_| fixed_diagnostic("web.internal.boot_record"))?;
        match (record.report.take(), record.superseded.take()) {
            (Some(engine), Some(superseded)) => (engine, record.master_track, superseded),
            _ => return Err(fixed_diagnostic("web.internal.boot_record").into()),
        }
    };
    let resources = plan.queries().resources();
    // The effect producers' identities, taken before the control plane's model is borrowed for the
    // rest of this function: the control plane keeps the one compiled model, and the browser reads
    // it (#1381a), so the host holds no second copy of it.
    let producer_facts: Vec<(
        Box<str>,
        LiveEffectAddress,
        Box<str>,
        &'static effect_contract::EffectDescriptor,
    )> = control
        .effect_producers()
        .iter()
        .map(|producer| {
            (
                producer.track_id.clone(),
                producer.address,
                producer.effect_id.clone(),
                producer.descriptor,
            )
        })
        .collect();
    let session = control.committed_session();
    let session_sample_rate_hz = session.sample_rate().0;
    let session_quantum_frames = session.quantum().0;

    let control_table = control_table_bytes(engine.source_count as usize)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let id_arena = source_id_arena_bytes(engine.source_id_bytes as usize)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    // Charged because the browser bridge retains them: everything the control plane keeps for the
    // session (its `control_retained_bytes` row: the facade's control table and ID arena, the
    // plan's state inventory (issue #1272), the strip, effect and send producers with their
    // payload, the protocol queues, the replay cache and the session's `live_values_superseded`
    // count (#1399)), and the compiled session model, which the control plane's store keeps and
    // the bridge reads.
    let ready_metadata =
        checked_sum_prepare([resources.control_retained_bytes, engine.session_model_bytes])?;
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
        .max(control.catalog_resources().largest_allocation_bytes)
        .max(engine.session_largest_allocation_bytes);
    let largest_named = bridge_largest.max(engine.largest_engine_allocation_bytes);
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
    // Issue #143 R7: the engine's walked row, carried through unchanged. Zero for a session
    // prepared with `live_control_observation_taps == 0`.
    report.observation_retained_bytes = engine.observation_retained_bytes;
    // Issue #1207 D1: `handles.strips` is the tracks, then the submixes. Since issue #1213 D1
    // every queue band, per-strip table and meter word covers both segments, tracks first, so a
    // track's entry is the same with or without submixes.
    //
    // #1381a: the strip order is the normalized model's, tracks then submixes, the order host-core's
    // addressing authority uses and the control plane's producers follow.
    let model = session.normalized_model();
    let track_count = model.tracks.len();
    let strip_count = track_count + model.submixes.len();
    let mut tracks: Vec<Box<str>> = Vec::new();
    tracks
        .try_reserve_exact(track_count)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    tracks.extend(
        model
            .tracks
            .iter()
            .map(|track| Box::from(track.id.as_str())),
    );
    let mut submixes: Vec<Box<str>> = Vec::new();
    submixes
        .try_reserve_exact(strip_count - track_count)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    submixes.extend(
        model
            .submixes
            .iter()
            .map(|submix| Box::from(submix.id.as_str())),
    );
    let mut rack_effects = Vec::new();
    rack_effects
        .try_reserve_exact(strip_count)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    let count = |effects: usize| -> Result<u32, Vec<u8>> {
        u32::try_from(effects).map_err(|_| fixed_diagnostic("web.live_controls.effects"))
    };
    // The lowered racks (decision 12), in chain order: every strip carries every `pre_insert` and
    // `post_insert` slot, so those counts are the session's. A live address (rack byte `3`
    // console or `1` inserts, #1096) reaches them through `dense_effect_slot`. Tracks first, then
    // submixes (issue #1207 D3): the strip order, so `rack_effects[t]` of a track never moves.
    for track in &model.tracks {
        rack_effects.push([
            count(model.console.pre_insert.len())?,
            count(track.inserts.effects.len())?,
            count(model.console.post_insert.len())?,
        ]);
    }
    for submix in &model.submixes {
        rack_effects.push([
            count(model.console.pre_insert.len())?,
            count(submix.inserts.effects.len())?,
            count(model.console.post_insert.len())?,
        ]);
    }
    // Issue #140 A: the dense effect-queue index. `effect_base[s]` is the number of effect
    // instances declared by every earlier strip, so `effect_slot` is arithmetic rather than a
    // search, and the producers are permuted into exactly that order once, here. The tracks lead,
    // so every track effect keeps its dense index and the submixes' effects follow them.
    let mut effect_base = Vec::new();
    effect_base
        .try_reserve_exact(strip_count)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    let mut total_effects = 0_u32;
    for counts in &rack_effects {
        effect_base.push(total_effects);
        for count in counts {
            total_effects = total_effects
                .checked_add(*count)
                .ok_or_else(|| fixed_diagnostic("web.live_controls.effects"))?;
        }
    }
    // #1381a: the effect producers are the control plane's. The browser keeps, in the dense slot
    // order, where each instance's producer is, so an observe record (kinds 7 and 8) reaches its
    // effect's observation queue through the control plane's accessor.
    let mut effect_instances: Vec<Option<EffectInstance>> = Vec::new();
    effect_instances
        .try_reserve_exact(total_effects as usize)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    effect_instances.resize_with(total_effects as usize, || None);
    for (position, (track_id, address, effect_id, descriptor)) in producer_facts.iter().enumerate()
    {
        let Some(strip) = strip_index(&tracks, &submixes, track_id) else {
            return Err(fixed_diagnostic("web.live_controls.effects").into());
        };
        let Some(entry) = dense_effect_slot(effect_base[strip], rack_effects[strip], *address)
            .and_then(|slot| effect_instances.get_mut(slot))
        else {
            return Err(fixed_diagnostic("web.live_controls.effects").into());
        };
        if entry.is_some() {
            return Err(fixed_diagnostic("web.live_controls.effects").into());
        }
        *entry = Some(EffectInstance {
            producer: u32::try_from(position)
                .map_err(|_| fixed_diagnostic("web.live_controls.effects"))?,
            reader: None,
            track_id: track_id.clone(),
            effect_id: effect_id.clone(),
            address: *address,
            descriptor,
        });
    }
    // Issue #143: the observation handles stay in the plan's attachment, in host-core's order; the
    // browser keeps, in the same dense `effect_slot` order the command addressing uses, which
    // reader each slot's handle is and what the handle declares, so one index serves both the
    // subscribe path and the poll. The companions are reached through the plan's owner.
    let companions = plan
        .active_attachment_mut::<RenderCompanions>()
        .ok_or_else(|| fixed_diagnostic("web.internal.companions"))?;
    let spectrum_capture = companions.take_spectrum_capture();
    let mut observation_arm_samples: Vec<Box<[u64]>> = Vec::new();
    observation_arm_samples
        .try_reserve_exact(total_effects as usize)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    observation_arm_samples.resize_with(total_effects as usize, || Box::new([]));
    // `observation_tracks[slot]` is the strip index of that effect slot's observed instance, or
    // `u32::MAX` for a slot with no taps. Built once, here, so the poll's fold is arithmetic.
    let mut observation_tracks = vec![u32::MAX; total_effects as usize];
    let observation_present = vec![false; strip_count];
    for (reader, handle) in companions.effect_observations().iter().enumerate() {
        let Some(strip) = strip_index(&tracks, &submixes, &handle.track_id) else {
            return Err(fixed_diagnostic("web.live_controls.observation").into());
        };
        let Some(slot) = dense_effect_slot(effect_base[strip], rack_effects[strip], handle.address)
        else {
            return Err(fixed_diagnostic("web.live_controls.observation").into());
        };
        let Some(instance) = effect_instances
            .get_mut(slot)
            .and_then(Option::as_mut)
            .filter(|instance| instance.reader.is_none())
        else {
            return Err(fixed_diagnostic("web.live_controls.observation").into());
        };
        // The frame carries one gain-reduction slot per strip, so every observed effect of a strip
        // points at that strip and the poll folds them max-magnitude into the one slot; a
        // submix's effect points at its strip index `T + j` (issue #1209 D5).
        observation_tracks[slot] =
            u32::try_from(strip).map_err(|_| fixed_diagnostic("web.live_controls.observation"))?;
        let tap_count = handle.descriptor.observations.len();
        let mut arm_samples = Vec::new();
        arm_samples
            .try_reserve_exact(tap_count)
            .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
        arm_samples.resize(tap_count, 0);
        observation_arm_samples[slot] = arm_samples.into_boxed_slice();
        instance.reader = Some(
            u32::try_from(reader).map_err(|_| fixed_diagnostic("web.live_controls.observation"))?,
        );
    }
    // The IDs the dense instance table copies out of the producers are retained beside the arm
    // samples.
    let observed_string_bytes = effect_instances
        .iter()
        .flatten()
        .try_fold(0_u64, |total, instance| {
            total.checked_add(
                u64::try_from(instance.track_id.len() + instance.effect_id.len()).ok()?,
            )
        })
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let observation_arm_sample_bytes = observation_arm_samples
        .iter()
        .try_fold(observed_string_bytes, |total, samples| {
            total.checked_add(
                u64::try_from(samples.len())
                    .ok()?
                    .checked_mul(size_of::<u64>() as u64)?,
            )
        })
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let observation_arm_sample_largest = observation_arm_samples
        .iter()
        .try_fold(0_u64, |largest, samples| {
            let bytes = u64::try_from(samples.len())
                .ok()?
                .checked_mul(size_of::<u64>() as u64)?;
            Some(largest.max(bytes))
        })
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let observation_arm_table_bytes = u64::from(total_effects)
        .checked_mul((size_of::<Box<[u64]>>() + size_of::<Option<EffectInstance>>()) as u64)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let observation_arm_largest = observation_arm_sample_largest.max(observation_arm_table_bytes);
    report.bridge_metadata_bytes = report
        .bridge_metadata_bytes
        .checked_add(observation_arm_table_bytes)
        .and_then(|bytes| bytes.checked_add(observation_arm_sample_bytes))
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.bridge_retained_bytes = report
        .bridge_retained_bytes
        .checked_add(observation_arm_table_bytes)
        .and_then(|bytes| bytes.checked_add(observation_arm_sample_bytes))
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.largest_bridge_allocation_bytes = report
        .largest_bridge_allocation_bytes
        .max(observation_arm_largest);
    report.largest_named_allocation_bytes = report
        .largest_named_allocation_bytes
        .max(report.largest_bridge_allocation_bytes);
    let mut meter_header = empty_meter_header();
    meter_header.track_count =
        u32::try_from(track_count).map_err(|_| fixed_diagnostic("web.live_controls.effects"))?;
    meter_header.submix_count =
        u32::try_from(submixes.len()).map_err(|_| fixed_diagnostic("web.live_controls.effects"))?;
    meter_header.master_track_plus_one = master_track.map_or(0, |track| track.saturating_add(1));
    let meter_count = companions.meters().len();
    let master_capacity = companions
        .meters()
        .first()
        .map_or(0, |meter| meter.consumer.capacity());
    let pending_meter_bytes = u64::try_from(meter_count)
        .ok()
        .and_then(|count| count.checked_mul(size_of::<Option<MeterSnapshot>>() as u64))
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let remaining_meter_bytes = u64::try_from(meter_count)
        .ok()
        .and_then(|count| count.checked_mul(size_of::<usize>() as u64))
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let master_meter_bytes = u64::try_from(master_capacity)
        .ok()
        .and_then(|count| count.checked_mul(size_of::<Option<MasterMeasurement>>() as u64))
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let meter_delivery_bytes = pending_meter_bytes
        .checked_add(remaining_meter_bytes)
        .and_then(|bytes| bytes.checked_add(master_meter_bytes))
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.bridge_metadata_bytes = report
        .bridge_metadata_bytes
        .checked_add(meter_delivery_bytes)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.bridge_retained_bytes = report
        .bridge_retained_bytes
        .checked_add(meter_delivery_bytes)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.largest_bridge_allocation_bytes = report
        .largest_bridge_allocation_bytes
        .max(pending_meter_bytes)
        .max(remaining_meter_bytes)
        .max(master_meter_bytes);
    report.largest_named_allocation_bytes = report
        .largest_named_allocation_bytes
        .max(report.largest_bridge_allocation_bytes);
    // The decoded observation staging is a separate retained allocation from the public 48-byte
    // wire staging row; charge the full actual array before the final aggregate budget check.
    let decoded_count = if commands_enabled {
        MAXIMUM_COMMAND_RECORDS as usize
    } else {
        0
    };
    let decoded_bytes = u64::try_from(decoded_count)
        .ok()
        .and_then(|count| count.checked_mul(size_of::<StagedObserve>() as u64))
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.bridge_metadata_bytes = report
        .bridge_metadata_bytes
        .checked_add(decoded_bytes)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.bridge_retained_bytes = report
        .bridge_retained_bytes
        .checked_add(decoded_bytes)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.largest_bridge_allocation_bytes =
        report.largest_bridge_allocation_bytes.max(decoded_bytes);
    report.largest_named_allocation_bytes =
        report.largest_named_allocation_bytes.max(decoded_bytes);
    let queries = plan.queries();
    let ready = ReadyOwnership {
        control,
        queries,
        #[cfg(feature = "test-support")]
        allow_rebuild,
        #[cfg(feature = "test-support")]
        limits,
        commands_enabled,
        live_values_superseded,
        effect_instances: effect_instances.into_boxed_slice(),
        observation_tracks: observation_tracks.into_boxed_slice(),
        observation_present: observation_present.into_boxed_slice(),
        observation_armed: boxed_zero_u32(total_effects as usize)?,
        observation_arm_samples: observation_arm_samples.into_boxed_slice(),
        master_track,
        meter_header,
        effect_base: effect_base.into_boxed_slice(),
        command_wanted: boxed_zero_u32(total_effects as usize)?,
        observe_named: boxed_zero_u32(total_effects as usize)?,
        command_decoded: boxed_observe_staging(decoded_count)?,
        in_flight: boxed_zero_u32(total_effects as usize)?,
        has_in_flight_commands: false,
        tracks,
        submixes,
        rack_effects: rack_effects.into_boxed_slice(),
        spectrum: spectrum_capture.map(|captures| {
            SpectrumService::new(captures, session_sample_rate_hz, session_quantum_frames)
        }),
        meter_frame: boxed_zero_meter_frame(strip_count)?,
        master_peak: [0.0, 0.0],
        master_start_sample: None,
        master_end_sample: 0,
        master_windows: (0..master_capacity)
            .map(|_| None)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        master_read: 0,
        master_write: 0,
        master_count: 0,
        meter_pending: (0..meter_count)
            .map(|_| None)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        meter_remaining: (0..meter_count)
            .map(|_| 0)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        meter_generation: 0,
        meter_loss_count: 0,
        meter_snapshot_drops_seen: 0,
        meter_snapshot_generation: None,
        meter_windows: 0,
        #[cfg(test)]
        meter_effect_scan_count: 0,
    };
    Ok((ready, plan, report))
}

/// Translate the browser configuration's two live-control words into the facade's live-control
/// request.
///
/// `live_control_meter_blocks == 0` is the honest form of "metering off": no observer is bound, so
/// the render path folds nothing at all. The port lease is a second, finer switch over posting.
fn live_control_request(
    options: WebBootOptions,
    quantum_frames: u32,
) -> Option<HostLiveControlRequest> {
    let meter_period_frames = if options.live_control_meter_blocks == 0 {
        None
    } else {
        let blocks = u32::try_from(options.live_control_meter_blocks).ok()?;
        Some(NonZeroU32::new(blocks.checked_mul(quantum_frames)?)?)
    };
    Some(HostLiveControlRequest {
        meter_period_frames,
        meter_queue_depth: NonZeroUsize::new(METER_QUEUE_DEPTH)?,
        meter_tap: MeterTap::PostMatrix,
        // Issue #143 D3/D6: both are carved browser configuration words, translated once, here.
        observation_taps: u32::try_from(options.live_control_observation_taps).ok()?,
        master_track: match options.live_control_master_track_plus_one {
            0 => None,
            value => Some(u32::try_from(value.checked_sub(1)?).ok()?),
        },
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

/// `3(T + S) + 3` words: two peak lanes and one gain-reduction magnitude per strip, then the
/// master's (issue #1209 D1). With `S = 0` it is the `3T + 3` frame every reader already knows.
fn boxed_zero_meter_frame(strip_count: usize) -> Result<Box<[f32]>, Vec<u8>> {
    let count = strip_count
        .checked_mul(3)
        .and_then(|value| value.checked_add(3))
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    value.resize(count, 0.0);
    Ok(value.into_boxed_slice())
}

/// The decode staging of the observation records: `count` entries, none when the boot asked for no
/// observation taps.
fn boxed_observe_staging(count: usize) -> Result<Box<[StagedObserve]>, Vec<u8>> {
    let empty = StagedObserve {
        effect: 0,
        original_wire_index: 0,
        tap_index: 0,
        armed: false,
        window_blocks: 0,
    };
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    value.resize(count, empty);
    Ok(value.into_boxed_slice())
}

/// One `code\t$\n` diagnostic line for a rule that names no session path.
fn fixed_diagnostic(code: &str) -> Vec<u8> {
    host_core::fixed_diagnostic_line(code)
}

fn checked_sum_prepare(values: impl IntoIterator<Item = u64>) -> Result<u64, Vec<u8>> {
    values.into_iter().try_fold(0_u64, |total, value| {
        total
            .checked_add(value)
            .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))
    })
}

fn boxed_zero_u8(bytes: u32) -> Result<Box<[u8]>, u32> {
    let count = usize::try_from(bytes).map_err(|_| RESULT_REFUSED_BUDGET)?;
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| RESULT_REFUSED_BUDGET)?;
    value.resize(count, 0);
    Ok(value.into_boxed_slice())
}

fn boxed_zero_f32(samples: u64) -> Result<Box<[f32]>, u32> {
    let count = usize::try_from(samples).map_err(|_| RESULT_REFUSED_BUDGET)?;
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| RESULT_REFUSED_BUDGET)?;
    value.resize(count, 0.0);
    Ok(value.into_boxed_slice())
}

fn boxed_uninit_planes(channels: u32) -> Result<Box<[MaybeUninit<&'static [f32]>]>, u32> {
    let count = usize::try_from(channels).map_err(|_| RESULT_REFUSED_BUDGET)?;
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| RESULT_REFUSED_BUDGET)?;
    value.resize_with(count, MaybeUninit::uninit);
    Ok(value.into_boxed_slice())
}

#[cfg(test)]
std::thread_local! {
    static ADMISSION_COUNTER_CLEAR: std::cell::Cell<(u64, u64)> =
        const { std::cell::Cell::new((0, 0)) };
}

#[cfg(test)]
fn reset_admission_counter_clear() {
    ADMISSION_COUNTER_CLEAR.with(|stats| stats.set((0, 0)));
}

#[cfg(test)]
fn admission_counter_clear_stats() -> (u64, u64) {
    ADMISSION_COUNTER_CLEAR.with(std::cell::Cell::get)
}

#[cfg(test)]
fn record_admission_counter_clear(elements: usize) {
    ADMISSION_COUNTER_CLEAR.with(|stats| {
        let (calls, cleared) = stats.get();
        stats.set((
            calls.saturating_add(1),
            cleared.saturating_add(elements as u64),
        ));
    });
}

mod ffi;
mod render_lock;

pub use ffi::*;
pub use render_lock::RenderLockedAllocator;

#[cfg(test)]
mod tests;
