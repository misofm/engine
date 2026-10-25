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
use core::num::NonZeroUsize;
use std::sync::{Arc, Mutex};

use builtins_compiler::MeterConsumer;
use control_plane::{
    CompileFailure, ControlAdapter, ControlLimits, ObservationChange, ObservationKeyError,
    ObservationKeyRef, ObservationRefusal, PlanQueries, PlanResources, PlanState,
    SessionSeekFailure, SessionState, SourceFailure, SourceRefusalKind,
};
use effect_contract::ObservationDescriptor;
use engine::realtime::{
    HostAttachment, ObservationWindow, ObserverCarry, ObserverKind, PlanarBufferMut,
};
use host_core::{
    CompiledSession, EffectObservationHandle, GainReductionTaps, HostLiveControlRequest,
    HostLiveLanes, HostPrepareCaps, HostPrepareReport, HostShapePolicy, MAXIMUM_VCA_REACH_PAIRS,
    MAXIMUM_VCAS_BOUNDED, MeterCarry, MeterDelivery, MeterFrame, MeterTables, MeterValidity,
    PrepareDiagnostics, PrepareRejection, PreparedHost, SourceControlError, SourceSubmission,
    SuccessorBase, TapView, compile_host_model, compiled_session_shape, control_table_bytes,
    parse_host_session, source_id_arena_bytes,
};
use host_core::{
    PreparedSpectrumCaptures, ResponseSnapshotError, ResponseSnapshotSink, SpectrumAnalysis,
    SpectrumCadence, SpectrumChannels, SpectrumContinuousWindow, SpectrumHop, SpectrumService,
    SpectrumServiceError, SpectrumSmoothingConfig, SpectrumSnapshot, SpectrumStreamMetadata,
    SpectrumStreamStatus, SpectrumTarget, SpectrumTargetRef,
};
#[cfg(test)]
use host_core::{SpectrumCaptureCollectionRequest, SpectrumCaptureRequest};
use session::{CompileCaps, SOURCE_CHANNELS_LAUNCH_MAXIMUM, STABLE_ID_MAXIMUM_BYTES};

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
/// below 13.1). 20 left 14.3% headroom. Issue #1381a re-measured the exact 1 MiB, 192-track by
/// 4-effect document on the shipped module (`scripts/check-web-boot-budget.mjs`): the boot now
/// builds the control plane's session, whose catalog, producers and protocol state the browser did
/// not hold, and the wasm high-water growth rose from under 20 MiB to 25,427,968 bytes (24.25
/// bytes per input byte). Root ruled 25 for the interval (2026-10-14). Issue #1611 found the cause
/// of that growth in the control plane's live-lane choice (`controls::live_lanes`), which collected
/// every control row of the session (53,763 rows of 232 bytes for this document) into one vector
/// grown by doubling to 15.2 MB, only to read three booleans, and on wasm the freed large blocks of
/// that doubling are not reused; it now walks the rows one owner at a time. The same document grows
/// the wasm high-water by 20,447,232 bytes (19.5 bytes per input byte, 312 pages), under the 20 MiB
/// pin of 20,971,520 bytes plus one page, so the multiplier is 20 again. Boot checks
/// `document_bytes * PARSE_TRANSIENT_MULTIPLIER` against the effective budget before UTF-8 decode
/// or parser allocation, and the peak-transient test keeps every phase visible so frontend growth
/// cannot silently outrun this projection.
pub const PARSE_TRANSIENT_MULTIPLIER: u64 = 20;

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
        SpectrumServiceError::NotArmed
        | SpectrumServiceError::Dropped
        | SpectrumServiceError::StreamActive
        | SpectrumServiceError::StreamInactive => RESULT_WRONG_STATE,
        SpectrumServiceError::CaptureInvalid => RESULT_RENDER_REJECTED,
        SpectrumServiceError::Exhausted => RESULT_REFUSED_BUDGET,
        SpectrumServiceError::CadenceRate => RESULT_UNSUPPORTED,
        SpectrumServiceError::UnknownEntry
        | SpectrumServiceError::Channels
        | SpectrumServiceError::WindowInvalid
        | SpectrumServiceError::CadenceShape
        | SpectrumServiceError::Smoothing
        | SpectrumServiceError::MetadataInvalid => RESULT_INVALID_ARGUMENT,
    }
}

/// The result word of a stream read that returned its status.
const fn spectrum_stream_result(status: SpectrumStreamStatus) -> u32 {
    match status {
        SpectrumStreamStatus::Inactive | SpectrumStreamStatus::Stopped => RESULT_WRONG_STATE,
        SpectrumStreamStatus::Warming | SpectrumStreamStatus::Pending => RESULT_BACKPRESSURE,
        SpectrumStreamStatus::Gap | SpectrumStreamStatus::Ready => RESULT_OK,
        SpectrumStreamStatus::Failed => RESULT_RENDER_REJECTED,
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

/// Exact byte size of one staged `miso.command.v1` record (#1677): the record names its tap by
/// stable key, so it carries the strip's and the effect's IDs.
pub const COMMAND_RECORD_BYTES: u32 = 272;
/// Width in bytes of each ID field of a staged record. An ID is 1 to 127 bytes
/// (`session::STABLE_ID_MAXIMUM_BYTES`), zero padded to this width.
pub const RECORD_ID_FIELD_BYTES: u32 = 128;
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
// The browser's read is host-core's (#1586 D3), which refuses a longer batch.
const _: () =
    assert!(MAXIMUM_OBSERVATION_READS <= host_core::observation::MAXIMUM_OBSERVATION_SELECTIONS);

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

/// The lanes a selection asks for, and a row's availability: host-core's, since the read is
/// host-core's ([`host_core::observation`], #1586 D3).
pub use host_core::observation::{ObservationReadChannels, ObservationReadStatus};

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
/// The record names the tap by its stable key: the strip ID, the rack, the console slot's or
/// insert's ID and the effect-local `tap_id`, the same key `OBSERVATION_ENDED` uses (#1677).
/// `window_blocks` carries the window length in render blocks (`0` = the plan's default). A
/// subscription changes what is read, never what is rendered.
pub const COMMAND_OBSERVE_SUBSCRIBE: u32 = 7;
/// Disarm one declared observation tap of one effect instance (issue #143 D3).
pub const COMMAND_OBSERVE_UNSUBSCRIBE: u32 = 8;
/// The `rack` of an effect-addressed record: one of the strip's inserts (decision 12, #1096).
///
/// It keeps the code the retired `dynamic` rack had. The record's effect ID is the insert's ID.
pub const RACK_INSERTS: u8 = 1;
/// The `rack` of an effect-addressed record: a session console slot (decision 12, #1096).
///
/// Appended. The record's effect ID is the console slot's ID.
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
fn strip_index(tracks: &[Box<str>], submixes: &[Box<str>], id: &[u8]) -> Option<usize> {
    // `str` orders by its bytes, so a byte-wise search is the search the canonical order allows.
    if let Ok(track) = tracks.binary_search_by(|track| track.as_bytes().cmp(id)) {
        return Some(track);
    }
    let submix = submixes
        .binary_search_by(|submix| submix.as_bytes().cmp(id))
        .ok()?;
    tracks.len().checked_add(submix)
}

/// The submission was admitted whole.
pub const COMMAND_REASON_NONE: u32 = 0;
/// A record's fixed shape is wrong: an unknown kind, a nonzero reserved word, or a non-finite value;
/// or the submission names one observation tap more than once (#1345 D2).
pub const COMMAND_REASON_MALFORMED: u32 = 1;
/// The record's strip ID is not a track or submix of the plan the record is admitted against
/// (#1677). The code keeps its number and its name from the index-addressed record.
pub const COMMAND_REASON_UNKNOWN_TRACK: u32 = 2;
/// `rack` is not an effect rack: an effect-addressed kind names [`RACK_INSERTS`] or
/// [`RACK_CONSOLE`]. The retired `simd1` (`0`) and `simd2` (`2`) codes land here (issue #1096).
pub const COMMAND_REASON_UNKNOWN_RACK: u32 = 3;
/// The record's effect ID is not a console slot or insert (per `rack`) of the addressed strip on
/// the plan the record is admitted against (#1677).
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
/// A bounded control queue had no room for the submission; nothing was admitted. Retryable: the
/// queue drains at the next render. Never the answer to a full observation event, which is
/// [`COMMAND_REASON_OBSERVATION_LIMIT`].
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
// Reasons 12-14 (`notSoloable`, `unknownRoute`, `unknownVca`) belonged to the value kinds #1565A
// deleted. They are retired and never reallocated, so the vocabulary is dense from 0 except for
// them, and the next reason is 15.
/// The subscription would leave armed taps whose one `OBSERVATION_ENDED` event, every key with its
/// real IDs, does not fit one control frame (#1676, #1685): the frame is full, so the engine could
/// not tell the app honestly when a later edit ends them all. The record named is the arm that
/// made the event outgrow the frame, after every unsubscribe of the same submission is counted;
/// nothing was admitted.
///
/// Not retryable as sent, and deliberately **not** `BACKPRESSURE`: no render makes room. The
/// submission is admitted only after the app unsubscribes another tap. The admission is the control
/// plane's, by the event's exact encoded bytes against the browser's 16 KiB frame, so the result is
/// `RESULT_UNSUPPORTED`, as for [`COMMAND_REASON_OBSERVATION_UNBOUND`].
pub const COMMAND_REASON_OBSERVATION_LIMIT: u32 = 15;
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
    /// `0` while this header describes the plan that renders; `1` once a swap block replaced it
    /// (issue #1290d1). The plan's meter frame travels with its header, so the worklet's views
    /// over the frame are the plan's: a header that reads `1` is the page's signal to switch to
    /// the views it cut over [`Self::successor_pointer`]'s header and frame, in the block that
    /// adopted the plan. The retired header stays alive until the service step after that block.
    pub retired: u32,
    /// Linear-memory address of the `3(T + S) + 3` word `f32` frame this header describes, or `0`
    /// for a handle with no compiled session (issue #1290d1).
    pub frame_pointer: u32,
    /// Linear-memory address of the header that replaces this one: the newest committed
    /// replacement plan's, from its commit until its adoption block, and the adopted plan's once
    /// [`Self::retired`] is set. `0` when no replacement waits. A page cuts the views over that
    /// header and its frame in the message handler that commits the plan, so the render path
    /// switches views by reference and never cuts one (issue #1290d1).
    pub successor_pointer: u32,
    /// The structure epoch of the control list this plan's strip order belongs to (issue #1562b):
    /// the `structure_epoch` that `CONTROL_LIST_GET` pages carry, as the commit that prepared the
    /// plan left it, and as each later commit that rode the plan's live lanes left it. Strips
    /// (tracks, then submixes) are listed in the order the frame carries them, so a client that
    /// holds a list of another epoch reads the list again before it names a frame's strips. A
    /// plan that renders keeps the epoch of its own commit while a replacement waits: the
    /// replacement's header carries the newer one, and the page sees it in the adoption block.
    /// Appended in place, so every earlier offset is unmoved.
    pub structure_epoch: u64,
}

/// `WebMeterHeader::reserved[1]` validity bits. The high 32 bits carry a saturating loss count.
/// Each is the core frame's [`MeterValidity`] bit of the same name (#1582 D2).
pub const METER_VALID_COMPLETE: u64 = MeterValidity::COMPLETE.bits() as u64;
/// The master peak covers the exact published track interval ([`MeterValidity::OUTPUT`]).
pub const METER_VALID_MASTER: u64 = MeterValidity::OUTPUT.bits() as u64;
/// One or more queued windows were dropped or rejected before this publication.
pub const METER_VALID_LOSS: u64 = MeterValidity::LOSS.bits() as u64;
/// Gain-reduction values are present from the effect tap's own latest window; their timing is
/// intentionally independent and never supplies the track/master interval.
pub const METER_VALID_GAIN_REDUCTION: u64 = MeterValidity::GAIN_REDUCTION.bits() as u64;
const METER_LOSS_SHIFT: u32 = 32;
/// Depth of every strip meter queue, track or submix (issue #1448 D1): one window per strip per
/// post, plus headroom for a control-side stall of a few windows. The live-control request builds
/// each meter queue with it, and each poll caps a meter's count at entry by it. It is host-core's
/// one depth (#1581 D3), the output meter's too, so the master and the strips queue alike.
const METER_QUEUE_DEPTH: usize = host_core::METER_QUEUE_DEPTH;

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
        retired: 0,
        frame_pointer: 0,
        successor_pointer: 0,
        structure_epoch: 0,
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
    /// Meter window in render blocks, or `0` for host-core's default window with no delivery
    /// (issues #137 D2, #1581 D2).
    ///
    /// Every plan meters every strip (one post-matrix left and right sample peak per strip, tracks
    /// then submixes) and the output, with a `blocks * quantum_frames` window; `0` is the core
    /// default, about 32 ms. A nonzero value also lets the port lease deliver: the lease opens the
    /// output meter's gate and gates whether a finished window is posted. With `0` the lease is
    /// refused, the output meter's gate stays shut, and the strip meters still fold. `12` is ~31
    /// frames per second at 48 kHz with a 128-frame quantum.
    pub live_control_meter_blocks: u64,
    /// Maximum declared observation taps to bind per effect, or `0` for no observation capacity
    /// at all (issue #143 D3, level 1).
    ///
    /// A nonzero value allocates the command staging, which is what accepts the observation
    /// records (kinds 7 and 8): a subscription rides the effect's own observation queue. With zero,
    /// a command submission is `RESULT_UNSUPPORTED` with `COMMAND_REASON_UNSUPPORTED_KIND`.
    pub live_control_observation_taps: u64,
    /// Must be zero. This was the master boot word (the designated master strip index plus one)
    /// until #1689 retired it: the master is the strip's `master` row, written by `SET_CONTROL`
    /// after boot, as on every host. The offset keeps its place and the word is refused when
    /// nonzero (`web.options.reserved2`).
    pub reserved2: u64,
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
            reserved2: 0,
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
    /// The structural commands this instance's control half ran in place of its render half
    /// (#1290a D6, #1294b), saturating: [`AudioWorkletEngineHost::blocking_rebuilds`], refreshed
    /// as every exported call records its result; zero after a dispose. A page without cross-origin
    /// isolation runs each rebuild in the worklet's message handler, so each one counts here.
    pub blocking_rebuilds: u32,
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
    /// The applied-revision watermark's revision (#1349 D2): the highest committed revision in
    /// effect together with every revision before it. Copied from the render report of the block
    /// that advanced it; the initial revision from boot until then.
    pub watermark_revision: u64,
    /// The absolute render sample of the first block in which `watermark_revision` was in effect;
    /// `0` for the initial revision.
    pub watermark_first_sample: u64,
    /// The watermark's outcome flags, in the bits of the C ABI's `miso_engine_v1_watermark`:
    /// `EXACT = 1`, `TRANSITION_FALLBACK = 2`, `SUPERSEDED = 4`. The browser assigns no bit.
    pub watermark_outcome_flags: u64,
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

/// The active plan's output latency and tail (#1688 D2), in the order and sizes of the C ABI
/// report's `latency_samples`, `tail_kind` and `tail_samples` words.
///
/// The host keeps one at a stable address, written by
/// [`AudioWorkletEngineHost::read_plan_latency`] and found through
/// `miso_engine_web_v1_plan_latency_ptr`. The words are the control plane's row for the active
/// plan epoch, copied: the browser computes nothing.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WebPlanLatency {
    /// Exact structure byte size ([`PLAN_LATENCY_BYTES`]); zero until the first successful read.
    pub struct_size: u32,
    /// Required zero.
    pub reserved0: u32,
    /// The plan's output latency in samples.
    pub latency_samples: u64,
    /// `0` for a tail that ends after `tail_samples`, `1` for an unbounded one: the C ABI's
    /// `TAIL_FINITE` and `TAIL_INFINITE`.
    pub tail_kind: u64,
    /// The finite tail in samples; zero for an unbounded tail.
    pub tail_samples: u64,
    /// Required-zero expansion words.
    pub reserved: [u64; 4],
}

/// Exact byte size of [`WebPlanLatency`].
pub const PLAN_LATENCY_BYTES: u32 = size_of::<WebPlanLatency>() as u32;

/// Where a source landed after its newest seek (#1293 D6), in the order and sizes of the C ABI's
/// `miso_engine_v1_seek_report`.
///
/// The host keeps one at a stable address, written by
/// [`AudioWorkletEngineHost::read_seek_report`] and found through
/// `miso_engine_web_v1_source_seek_report_ptr`. The words are the control plane's
/// `SessionState::seek_report`, copied: the browser computes nothing (root ruling R7: 64 bytes,
/// the C struct's layout, reserved words zero).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WebSeekReport {
    /// Exact structure byte size ([`SEEK_REPORT_BYTES`]); zero until the first successful read.
    pub struct_size: u32,
    /// Required zero.
    pub reserved0: u32,
    /// The generation in effect; `0` until the source's ring has rendered a block.
    pub generation: u64,
    /// The source-read sample of the first block `generation` played in: the clock
    /// `miso_engine_web_v1_source_seek_at`'s anchor is on.
    pub first_sample: u64,
    /// The source frame that block started at; a late anchored seek's includes its lateness.
    pub source_frame: u64,
    /// Saturating cumulative missing in-region frames (true underruns).
    pub cumulative_underrun_frames: u64,
    /// Saturating cumulative in-region frames held while an anchored seek waited.
    pub cumulative_held_frames: u64,
    /// Required-zero expansion words.
    pub reserved: [u64; 2],
}

/// Exact byte size of [`WebSeekReport`], equal to the C ABI seek report's.
pub const SEEK_REPORT_BYTES: u32 = size_of::<WebSeekReport>() as u32;

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

/// What the browser derives from one plan's shape (issue #1290a): the strip order, the dense effect
/// table that addresses the plan's effects and observation readers, and the per-meter and
/// per-effect scratch that the control and delivery paths use. Every table is a function of the
/// plan's session model, producers and companions, so the plan's own preparation builds it
/// ([`build_plan_shape`]) and a replacement plan carries its own in its
/// [`RenderCompanions`]. The host swaps the pair at the block that adopts the plan, a pointer
/// exchange that allocates and frees nothing.
///
/// The published meter frame and its header are here too (#1290d1): the frame is `3(T + S) + 3`
/// words for this plan's strips, so a plan with another strip count publishes a frame of its own
/// size. They travel with the tables, in the same pointer exchange, and the page cuts its views
/// over the successor's frame in the message handler that commits it ([`WebMeterHeader::retired`]
/// and [`WebMeterHeader::successor_pointer`] tell the render path which views to use).
struct PlanShape {
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
    /// Per-strip prefix sum of effect instances, so `effect_slot` is arithmetic, not a lookup:
    /// the tracks first, then the submixes (issue #1207 D3), so a track's entry is the same with
    /// or without submixes.
    effect_base: Box<[u32]>,
    /// Every effect instance of the plan in the dense `effect_slot` order, whose length is the
    /// number of instances. It is how an observe record (kinds 7 and 8, ruling C9) reaches its
    /// effect's observation queue, and how the reads reach its readers.
    effect_instances: Box<[Option<EffectInstance>]>,
    /// Issue #143: the meter consumers (issue #137 D2, one per selected meter, empty when
    /// `live_control_meter_blocks` was zero) and the observation reader sets are the plan's
    /// [`RenderCompanions`]; `effect_instances` says where each effect's reader set is.
    /// Strip index of each observed effect slot (a submix's is `tracks.len() + j`), or `u32::MAX`
    /// for a slot with no taps. Built once at compilation, so the poll's fold is arithmetic rather
    /// than a search.
    observation_tracks: Box<[u32]>,
    /// The plan's effect instances by stable key, each with its reader set's position in the
    /// plan's [`RenderCompanions`] (#1586 D3): what a selected read resolves through. A carried
    /// swap exchanges reader sets at fixed positions, so the index stays right across it.
    observation_index: host_core::observation::ObservationIndex,
    /// The dense effect slot of each producer of the plan, in the control plane's producer order:
    /// entry `p` is the slot whose [`EffectInstance::producer`] is `p`. The inverse of that field,
    /// built in the same preparation loop, so the key the control plane resolved to a producer
    /// position reaches the per-slot tap state by an index, not a search (#1677).
    producer_slots: Box<[u32]>,
    /// The designated master strip index (tracks first, then submixes), or `None` (issue #143
    /// D6, issue #1213 D5).
    master_track: Option<u32>,
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
    /// Observation subscriptions admitted per effect queue since the last successful render.
    ///
    /// The browser's control plane and render plane are the same thread and every live-control
    /// stage drains its whole observation queue at the top of every block, so a successful render
    /// empties every control queue. That is what makes this an exact free-slot count rather than
    /// an estimate, and it is what lets a submission be refused *before* anything is queued.
    in_flight: Box<[u32]>,
    /// The plan's meter delivery tables (#1582 R1): built with the plan, off the render thread,
    /// and handed to the delivery by pointer exchange at the block that adopts the plan. `None`
    /// while the plan renders (the delivery holds them); a retired plan's are back here, for the
    /// control side to drop.
    meter_tables: Option<MeterTables>,
    /// The published frame, `3(T + S) + 3` words (issue #1209 D1): `[track0 L, track0 R, .., trackN
    /// L, trackN R, bus0 L, bus0 R, .., busM L, busM R, master L, master R, track0 GR, .., trackN
    /// GR, bus0 GR, .., busM GR, master GR]`, one peak pair and one gain-reduction word per strip.
    /// With `S = 0` every word is where it always was.
    meter_frame: Box<[f32]>,
    /// The header of [`Self::meter_frame`]: the plan's strip counts and designated master, and the
    /// window the poll published. The swap block copies the stream's fields from the retiring
    /// plan's header into the adopted plan's.
    meter_header: WebMeterHeader,
    /// The full-width address of the header [`WebMeterHeader::successor_pointer`] names, or `0`.
    /// The header word is a `u32` linear-memory address, which reads `0` for an address above
    /// 4 GiB (a native test host), so a gate that compares the word with the same conversion
    /// would pass vacuously. This is the pointer itself, set beside the word by
    /// [`Self::name_successor`], and a gate compares it with `core::ptr::eq`.
    #[cfg(any(test, feature = "test-support"))]
    successor_address: usize,
}

/// How many replacement plans' tables the host holds at once. After every commit the core holds
/// at most one unadopted candidate (a structural edit supersedes a waiting one, #1310), and the
/// host drops a superseded candidate's tables as soon as the core no longer holds its epoch
/// ([`ReadyOwnership::drop_superseded_shapes`]); the tables of plans adopted since the last
/// service step wait for it. So a command meets at most one entry after the reclaim and adds one:
/// four leaves room to spare, and the room check in [`ReadyOwnership::run_control`] never refuses
/// a structural edit, paused or not.
const PENDING_SHAPES: usize = 4;

/// One replacement plan's tables, committed with the plan (#1290a).
struct PendingShape {
    /// The plan's epoch.
    epoch: u64,
    /// The plan's tables until the block that adopts the plan swaps them with the host's, and the
    /// tables that block retired after it.
    shape: Box<PlanShape>,
    /// Whether the adoption block has passed: the entry then holds tables nothing reads, which
    /// the service step drops on the control side.
    adopted: bool,
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
    /// The limits the control half was built with (#1570b D6), for the cross-host proof.
    #[cfg(feature = "test-support")]
    limits: ControlLimits,
    /// The any-thread projection of the plan: its resource row and the applied-revision watermark
    /// (#1381a D5).
    queries: PlanQueries<WebAdapter>,
    /// The tables of the plan that renders (#1290a): the boot plan's until the block that adopts a
    /// replacement, which swaps in the replacement's own from its companions.
    shape: Box<PlanShape>,
    /// The epoch of the plan `shape` describes.
    shape_epoch: u64,
    /// Where the preparer leaves a replacement plan's tables (#1290a).
    record: Arc<Mutex<BootRecord>>,
    /// The tables of replacement plans between their commit and the service step after their
    /// adoption block (#1290a). Reserved at boot, so neither the commit (it checks the room first)
    /// nor the adoption (it swaps in place) ever grows it from a render call.
    pending_shapes: Vec<PendingShape>,
    /// The structural commands this instance's control half ran in place of its render half
    /// (#1290a D6), saturating: a rebuild's preparation and publication run on the thread that
    /// also renders, so each one is a blocking rebuild, counted and reported.
    blocking_rebuilds: u32,
    /// Whether the boot asked for observation taps (`live_control_observation_taps != 0`): the
    /// command staging exists and the observation records (kinds 7 and 8) are accepted only then.
    commands_enabled: bool,
    /// A handle on the session's `live_values_superseded` count (#1312 D2): the one count the
    /// control plane made, which every cell reader of every plan the session prepares adds to,
    /// and which the status reports (#1399).
    live_values_superseded: host_core::LiveValuesSuperseded,
    /// Decoded submission staging: one entry per staged record, [`MAXIMUM_COMMAND_RECORDS`] long.
    command_decoded: Box<[StagedObserve]>,
    /// Whether any subscription has been queued since the last successful render.
    ///
    /// This keeps command-free blocks from walking every destination counter. It is set at each
    /// successful push, so a partial internal failure cannot accidentally credit queue capacity.
    has_in_flight_commands: bool,
    /// The session's one meter delivery (#1582 D1): the output meter's reader and gate (#1581 D5),
    /// the level the lease drives, the window grid, the generation, the loss count and the
    /// rendering plan's meter tables. The poll calls it and copies the frame it publishes into
    /// the plan's [`PlanShape::meter_frame`] and [`PlanShape::meter_header`].
    meters: MeterDelivery,
    /// Test-only proof that incomplete master intervals never reach effect-reader scanning.
    #[cfg(test)]
    meter_effect_scan_count: u64,
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

    /// Run one control command frame through the control plane (#1290a). When it publishes a
    /// replacement plan (the control plane's newest plan epoch advances exactly when a rebuild was
    /// prepared, committed and published), the plan's tables are kept for its adoption block and
    /// the rebuild is counted as a blocking rebuild (D6); a refused or live command counts and
    /// keeps nothing.
    ///
    /// The room for the tables is checked before the command runs, so a command is never
    /// acknowledged and then unable to keep what the host needs to read its plan.
    fn run_command(
        &mut self,
        request: &[u8],
        document: &[u8],
        output_capacity: u64,
        analysis: Option<&mut SpectrumAnalysis>,
    ) -> Result<usize, control_plane::CommandError> {
        self.run_control(
            analysis,
            control_plane::CommandError::Backpressure,
            |control| control.command_with_document(request, document, output_capacity),
        )
    }

    /// Declare a discontinuity through the control plane (#1323), the function the C ABI's
    /// `miso_engine_v1_declare_discontinuity` runs. A plan it publishes is kept and counted as
    /// [`Self::run_command`] keeps a rebuild's.
    fn declare_discontinuity(
        &mut self,
        analysis: Option<&mut SpectrumAnalysis>,
    ) -> Result<(), control_plane::CommandError> {
        self.run_control(
            analysis,
            control_plane::CommandError::Backpressure,
            WebSession::declare_discontinuity,
        )
    }

    /// Seek the session through the control plane (#1643 D1): `SessionState::session_seek`, the
    /// call the C ABI's `miso_engine_v1_session_seek` and `_session_seek_at` run. The plain form
    /// declares the discontinuity in the same call (root ruling B16-R9), so a plan it publishes
    /// is kept and counted as [`Self::declare_discontinuity`] keeps one; the anchored form
    /// declares nothing and so publishes nothing.
    fn session_seek(
        &mut self,
        analysis: Option<&mut SpectrumAnalysis>,
        generation: u64,
        timeline_frame: u64,
        anchor_sample: Option<u64>,
    ) -> Result<(), SessionSeekFailure> {
        if anchor_sample.is_some() {
            return self
                .control
                .session_seek(generation, timeline_frame, anchor_sample);
        }
        self.run_control(
            analysis,
            SessionSeekFailure::Declaration(control_plane::CommandError::Backpressure),
            |control| control.session_seek(generation, timeline_frame, None),
        )
    }

    /// Run one control-plane call that may publish a replacement plan (#1290a, #1323): reclaim
    /// first, check the room for the plan's tables (refusing with `full` when there is none, with
    /// nothing run), run `call`, and keep the tables of a plan it published for its adoption block.
    ///
    /// `analysis` is the smoothing history of an instance that owns both halves of the spectrum
    /// stream: a promotion `call` runs in the core (a structural edit promotes the candidate render
    /// took before it rebuilds) ends it when the promotion ends the stream (#1618).
    fn run_control<T, E>(
        &mut self,
        analysis: Option<&mut SpectrumAnalysis>,
        full: E,
        call: impl FnOnce(&mut WebSession) -> Result<T, E>,
    ) -> Result<T, E> {
        self.reclaim_shapes();
        if self.pending_shapes.len() == self.pending_shapes.capacity() {
            return Err(full);
        }
        let stream = self.spectrum_stream();
        let before = self.control.newest_plan_epoch();
        let result = call(&mut self.control);
        self.end_ended_stream(stream, analysis);
        // Whatever the preparer left is taken: a command refused after its preparation leaves
        // tables nothing will read.
        let prepared = self
            .record
            .lock()
            .map_or(None, |mut record| record.shape.take());
        let epoch = self.control.newest_plan_epoch();
        if result.is_ok() && epoch != before {
            // A rebuild commits only after the preparer returned `Ok`, and the preparer returns
            // `Ok` only once it has left the plan's tables in the record (it refuses the
            // preparation, before anything is committed, when it cannot). So a committed rebuild
            // always has its tables, and no refusal can follow a commit.
            let shape = prepared.expect("a committed rebuild's preparer left the plan's tables");
            self.pending_shapes.push(PendingShape {
                epoch,
                shape: Box::new(shape),
                adopted: false,
            });
            // The replacement's header carries its master word before the rendering plan's header
            // names it (#1689), so the page never reads the successor without its master.
            self.sync_master();
            // The rendering plan's header names the newest replacement's, so the page cuts its
            // views over the successor's frame in the handler that committed it (#1290d1).
            if let Some(entry) = self.pending_shapes.last() {
                self.shape
                    .name_successor(core::ptr::from_ref(&entry.shape.meter_header));
            }
            self.blocking_rebuilds = self.blocking_rebuilds.saturating_add(1);
            // #1310: the commit may have superseded the candidate that waited; its tables go now,
            // on the control side, as the core says.
            self.drop_superseded_shapes();
        }
        if result.is_ok() {
            self.stamp_structure_epoch();
            self.sync_master();
        }
        result
    }

    /// Point the newest plan's tables at the master strip the core designates now (#1683): each
    /// replacement's that waits for its adoption, else the rendering plan's, each by
    /// the strip's ID in that plan's own strip order, so a strip added or removed before the
    /// master never moves the reading to another strip. A plan that does not hold the strip has no
    /// master. The designation moves no render value; the next poll folds the master reading from
    /// it. Control side only, as [`Self::stamp_structure_epoch`].
    fn sync_master(&mut self) {
        let master = self.control.master_strip();
        // While a replacement waits for its adoption, the designation rides it: the rendering
        // plan keeps its own until the block that adopts the replacement swaps the tables (#1683
        // review fold), so a rebuild that removes the master never clears the reading of the
        // plan still rendering.
        let waiting = self.pending_shapes.iter().any(|entry| !entry.adopted);
        let rendering = (!waiting).then_some(&mut *self.shape);
        let shapes = rendering.into_iter().chain(
            self.pending_shapes
                .iter_mut()
                .filter(|entry| !entry.adopted)
                .map(|entry| &mut *entry.shape),
        );
        for shape in shapes {
            let index = master.and_then(|id| {
                strip_index(&shape.tracks, &shape.submixes, id.as_str().as_bytes())
                    .and_then(|index| u32::try_from(index).ok())
            });
            shape.master_track = index;
            shape.meter_header.master_track_plus_one =
                index.map_or(0, |track| track.saturating_add(1));
        }
    }

    /// Stamp the structure epoch the control list holds now on the newest committed plan's meter
    /// header (#1562b): the replacement's while one waits, else the rendering plan's. A commit that
    /// rode the newest plan's live lanes may have changed a descriptor or an address (a VCA added,
    /// a `matrix_mode` switch), and a rebuild's commit changes the strip order. Control side only.
    fn stamp_structure_epoch(&mut self) {
        let epoch = self.control.structure_epoch();
        let header = match self
            .pending_shapes
            .iter_mut()
            .rev()
            .find(|entry| !entry.adopted)
        {
            Some(entry) => &mut entry.shape.meter_header,
            None => &mut self.shape.meter_header,
        };
        header.structure_epoch = epoch;
    }

    /// Drop the tables of plans render has adopted past (#1290a): the host's own, retired by the
    /// adoption block, and those of a candidate that a later one superseded. Control side only.
    fn reclaim_shapes(&mut self) {
        self.pending_shapes.retain(|entry| !entry.adopted);
        self.drop_superseded_shapes();
    }

    /// Drop the tables of every candidate the core no longer holds (#1310, batch 15a review fold
    /// B1): a structural edit that superseded a waiting candidate dropped its plan with the commit,
    /// and no block will adopt it. The core decides
    /// ([`control_plane::SessionState::holds_plan_epoch`]); an adopted entry waits for
    /// [`Self::reclaim_shapes`]. Control side only: it frees.
    fn drop_superseded_shapes(&mut self) {
        let control = &self.control;
        self.pending_shapes
            .retain(|entry| entry.adopted || control.holds_plan_epoch(entry.epoch));
    }

    /// The capture epoch of the session's running spectrum stream, `Some(None)` for a running
    /// stream with no epoch, or `None` when no stream runs (#1596).
    fn spectrum_stream(&self) -> Option<Option<u64>> {
        self.control
            .spectrum_monitor()
            .service()
            .filter(|service| service.stream_active())
            .map(SpectrumService::stream_epoch)
    }

    /// End `analysis`'s stream when the session's stream that ran as `before`
    /// ([`Self::spectrum_stream`]) no longer runs (#1618, #1596): the core promotes the plans
    /// render adopted, and a promotion that does not keep the stream ends it with the session's
    /// own analysis, never with the instance's. Without this the instance's analysis would keep
    /// analyzing a stream that no longer exists. Allocation-free.
    fn end_ended_stream(
        &self,
        before: Option<Option<u64>>,
        analysis: Option<&mut SpectrumAnalysis>,
    ) {
        if before.is_some()
            && self.spectrum_stream() != before
            && let Some(analysis) = analysis
        {
            analysis.end_stream();
        }
    }

    // REALTIME_POLICY_BEGIN
    /// Take the tables of the plan whose epoch is `epoch` (#1290a), at the block that adopted it.
    /// The tables it retires stay in the entry for [`Self::reclaim_shapes`]; an entry of an
    /// earlier epoch was superseded and is marked the same way. A pointer exchange: it allocates
    /// and frees nothing, and indexes nothing, so it adds no trap to the render export.
    ///
    /// `carried` is the pairing the swap block's attachment hook made (#1381b): the readers the
    /// executors carried, which this hand-over follows with the meter delivery's per-meter state
    /// (a carried tap's arm state is in its slot, #1585 D6).
    /// A swap that carried a meter keeps the meter delivery running, because the carried meters
    /// keep their window grid and the host's master windows are continuous; one that carried none
    /// restarts it, because its meters open their first window at the adoption block. A swap that
    /// adds or removes a strip in front shifts the meters' handles (a handle is a position): each
    /// carried meter remembers the handle its predecessor published under ([`MeterDelivery::adopt`],
    /// which takes the adopted plan's [`PlanShape::meter_tables`]), so a window closed before the
    /// swap, pending or still queued, is accepted by the strip it was carried to, not dropped.
    ///
    /// The meter frame travels with the tables (#1290d1): the adopted plan's frame and header are
    /// the published ones from this block on. The retiring header is marked
    /// ([`WebMeterHeader::retired`]) and names the adopted one, so the page's render path switches
    /// to the views it cut over them in advance. Pointer exchanges and scalar writes only.
    fn adopt_shape(&mut self, epoch: u64, carried: &[CarriedReader], first_sample: u64) {
        self.shape_epoch = epoch;
        let meters_carried = carried
            .iter()
            .any(|reader| reader.kind == ObserverKind::Meter);
        let mut taken = false;
        for entry in self.pending_shapes.iter_mut() {
            if entry.adopted {
                continue;
            }
            if entry.epoch == epoch {
                core::mem::swap(&mut self.shape, &mut entry.shape);
                entry.adopted = true;
                taken = true;
                // `self.shape` is the adopted plan's and `entry.shape` the retired one's. The
                // stream the page reads continues in the adopted plan's header (#1290d1): its
                // window, sequence, generation and loss are the stream's, its strip counts and
                // frame are the plan's. The retired header says so and names the adopted one, so
                // the page's render path switches to the views it cut in advance.
                let stream = entry.shape.meter_header;
                entry.shape.meter_header.retired = 1;
                entry
                    .shape
                    .name_successor(core::ptr::from_ref(&self.shape.meter_header));
                self.shape.meter_header.windows = stream.windows;
                self.shape.meter_header.first_sample = stream.first_sample;
                self.shape.meter_header.end_sample = stream.end_sample;
                self.shape.meter_header.sequence = stream.sequence;
                self.shape.meter_header.master_gr_present = stream.master_gr_present;
                self.shape.meter_header.reserved = stream.reserved;
                // The delivery adopts the plan's meter tables (#1582 R1): the pending strip
                // snapshots follow their meters, and a swap that carried no meter restarts the
                // delivery at this block, on a new grid (#1581 D5). The retired tables come back
                // in the adopted shape's slot and go to the retired one, by pointer exchange.
                // Every pending shape is built with its tables (`build_plan_shape`), and only
                // this exchange takes them.
                debug_assert!(
                    self.shape.meter_tables.is_some(),
                    "an adopted plan shape has no meter tables"
                );
                if let Some(tables) = self.shape.meter_tables.as_mut() {
                    self.meters.adopt(
                        tables,
                        carried
                            .iter()
                            .filter(|reader| reader.kind == ObserverKind::Meter)
                            .map(|reader| MeterCarry {
                                successor: reader.successor,
                                predecessor: reader.predecessor,
                                successor_handle: reader.successor_handle,
                                predecessor_handle: reader.predecessor_handle,
                            }),
                        meters_carried,
                        first_sample,
                    );
                }
                core::mem::swap(&mut self.shape.meter_tables, &mut entry.shape.meter_tables);
                // A carried tap's arm state needs no hand-over: it is in the tap's slot, which
                // moves with the tap (#1585 D6).
            } else if entry.epoch < epoch {
                entry.adopted = true;
            }
        }
        if taken {
            // A replacement committed after this one waits behind it (#1290d1).
            let waiting = self
                .pending_shapes
                .iter()
                .rev()
                .find(|entry| !entry.adopted)
                .map_or(core::ptr::null(), |entry| {
                    core::ptr::from_ref(&entry.shape.meter_header)
                });
            self.shape.name_successor(waiting);
            // The tables hold the admissions made against the plan before it rendered; the block
            // that adopted it drained its observation queues at its top, so the pass that follows
            // the swap clears the counts (a block that did not adopt cleared only the tables of
            // the plan it rendered).
            self.has_in_flight_commands = true;
        }
    }

    // REALTIME_POLICY_END

    /// Run `admit` with the tables of the newest committed plan in place of the rendering plan's
    /// (#1290a, batch 10 review): the rendering plan's while no replacement is waiting for its
    /// block, else the waiting replacement's, the plan whose producers the control plane lists
    /// (`effect_producers`). An observation record names its tap by stable key; the tables resolve
    /// the key to an effect (#1677) and the producers reach its queue, so both must be one plan's:
    /// the newest, as a live value is, so a submission between a rebuild's commit and its adoption
    /// is admitted against the successor and takes effect on it, and the browser answers nothing
    /// the C ABI would not.
    ///
    /// The tables are exchanged by pointer and exchanged back before this returns, so nothing
    /// allocates and the rendering plan's tables are the host's again for the next render call.
    /// `Err` only when the control plane's newest epoch has no waiting tables, which a commit
    /// never leaves (the command keeps the tables of every plan it publishes).
    fn admit_on_newest_plan<T>(&mut self, admit: impl FnOnce(&mut Self) -> T) -> Result<T, ()> {
        let newest = self.control.newest_plan_epoch();
        if newest == self.shape_epoch {
            return Ok(admit(self));
        }
        let index = self
            .pending_shapes
            .iter()
            .position(|entry| !entry.adopted && entry.epoch == newest)
            .ok_or(())?;
        core::mem::swap(&mut self.shape, &mut self.pending_shapes[index].shape);
        let admitted = admit(self);
        core::mem::swap(&mut self.shape, &mut self.pending_shapes[index].shape);
        Ok(admitted)
    }

    /// The dense index of one addressed effect instance in `effect_instances`.
    ///
    /// An effect is addressed in the session's terms, a console slot by its slot index or an
    /// insert by its index (issue #1096); [`LiveEffectAddress::lower`] is the one translation to
    /// the lowered rack that orders the band. `None` means the address names no instance this
    /// session prepared.
    fn effect_slot(&self, track: usize, address: LiveEffectAddress) -> Option<usize> {
        let slot = dense_effect_slot(
            *self.shape.effect_base.get(track)?,
            *self.shape.rack_effects.get(track)?,
            address,
        )?;
        (slot < self.shape.effect_instances.len()).then_some(slot)
    }

    /// The position, in the control plane's producer list, of the effect at dense slot `effect`
    /// (#1381a): the one way the observation records reach an effect's observation queue (ruling
    /// C9). A free function of the shape, so the admission can read it while it lends the control
    /// plane mutably.
    fn effect_producer_position(shape: &PlanShape, effect: usize) -> Option<usize> {
        let position = shape.effect_instances.get(effect)?.as_ref()?.producer;
        Some(position as usize)
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
    /// The watermark `read_watermark` last copied (#1381a D5), at a stable address in the
    /// control half: all zero until the first successful read.
    watermark: WebPlanWatermark,
    /// The plan latency `read_plan_latency` last copied (#1688 D2), at a stable address in the
    /// control half: all zero until the first successful read.
    plan_latency: WebPlanLatency,
    /// The seek report `read_seek_report` last copied (#1293 D6), at a stable address in the
    /// control half: all zero until the first successful read.
    seek_report: WebSeekReport,
    buffers: Option<PreparedBuffers>,
    ready: Option<ReadyOwnership>,
    /// The render half of the plan exchange (#1381a D4): the plan owner that `render_next` renders
    /// through, and whose active plan's attachment holds the meter consumers and the observation
    /// readers. It is the render side of the single instance, beside the control half in `ready`,
    /// so the two borrow apart. Dropped, with `ready`, only by [`Self::dispose`].
    plan: Option<WebPlan>,
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
            spectrum_hop,
        )?;
        let exact_retained = exact_retained_bytes(&resources)?;
        if exact_retained > memory_budget {
            return Err(BootFailure::exact_budget(
                "host.budget.retained_exact",
                exact_retained,
                memory_budget,
            ));
        }
        // #1349 D2: the status opens with the plan's initial watermark, `(initial revision, 0,
        // EXACT)`. No render is in flight at boot, so the read cannot be busy.
        let initial_watermark = ready
            .queries
            .watermark()
            .map_err(|_| BootFailure::fixed(RESULT_INTERNAL, "web.internal.watermark"))?;
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
                blocking_rebuilds: 0,
                next_absolute_sample: 0,
                rendered_quanta: 0,
                live_values_superseded: 0,
                watermark_revision: initial_watermark.revision,
                watermark_first_sample: initial_watermark.first_sample,
                watermark_outcome_flags: initial_watermark.outcome_flags,
            },
            resources,
            command_report: empty_command_report(),
            watermark: WebPlanWatermark {
                struct_size: WATERMARK_BYTES,
                ..WebPlanWatermark::default()
            },
            plan_latency: WebPlanLatency::default(),
            seek_report: WebSeekReport::default(),
            buffers: Some(buffers),
            ready: Some(ready),
            plan: Some(plan),
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
            .shape
            .effect_instances
            .get(effect)?
            .as_ref()?
            .reader?;
        self.test_companions()
            .effect_observations
            .get(reader as usize)
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

    /// Whether no seek waits in a command slot of the newest committed session's timeline and
    /// sources (batch 16b review, MINOR f): a session seek one above the published generation
    /// passes its check, which a full slot refuses. `None` for a host with no control half. It
    /// shows a stray push on a host that cannot render the block that would empty the slot.
    #[cfg(feature = "test-support")]
    #[must_use]
    pub fn session_seek_slots_empty_for_test(&self) -> Option<bool> {
        let control = &self.ready.as_ref()?.control;
        let providers = control
            .test_pending_providers()
            .last()
            .unwrap_or_else(|| control.test_providers());
        let sources = providers.test_sources();
        let next = sources.published_generation().checked_add(1)?;
        Some(sources.check_session_seek(next, 0, None).is_ok())
    }

    /// The session timeline's sample at `source_read_sample` in the plan that renders (#1641's
    /// test reader, #1643 gates); `None` for a host with no plan, a plan with no timeline, or a
    /// sample the reader has no record for. Between render calls only.
    #[cfg(feature = "test-support")]
    pub fn timeline_at_for_test(&mut self, source_read_sample: u64) -> Option<i64> {
        let plan = self.plan.as_mut()?;
        host_core::test_only_timeline_at(
            plan.test_owner_mut().active_plan_mut(),
            source_read_sample,
        )
    }

    /// Render one block through the plan owner at `absolute_sample` rather than the host's clock,
    /// and nothing else (#1398 gate 1b); `false` when the owner refused it or the host is not
    /// ready. A mistimed block adopts a published successor before the owner checks its time, and
    /// is refused without publishing the epoch atomic: the window in which render has swapped
    /// plans and the control plane has not seen the epoch. The host's own records (clock, shape,
    /// status) do not move.
    #[cfg(feature = "test-support")]
    pub fn render_plan_at_for_test(&mut self, absolute_sample: u64) -> bool {
        let quantum = self.status.quantum_frames as usize;
        let (Some(plan), Some(buffers)) = (self.plan.as_mut(), self.buffers.as_mut()) else {
            return false;
        };
        PlanarBufferMut::try_new(&mut buffers.output_pcm, 2, quantum, quantum)
            .is_ok_and(|output| plan.render(absolute_sample, output).is_ok())
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
    /// for a typed refusal before commit (a transaction that needs a plan rebuild is prepared and
    /// published like any other, #1562b); [`RESULT_REPREPARE_REQUIRED`] for a transaction that changes the
    /// session's sample rate or quantum (#1290a: the session is re-prepared from its document);
    /// [`RESULT_INTERNAL`] for a bookkeeping failure. For each failure the
    /// response is the refusal's diagnostic (`code\tpath\n` lines).
    ///
    /// A frame refused after the core has seen it has consumed its request ID: the same bytes sent
    /// again are answered [`RESULT_OK`] with a non-ok response frame whose code is
    /// `protocol.failure` (the replay cache), and a retry must take a new request ID to meet the
    /// refusal again.
    ///
    /// `analysis` is the smoothing history of an instance that owns both halves of the spectrum
    /// stream (see [`Self::service`]); a plan this command's reclaim promotes ends it when the
    /// stream is not kept.
    ///
    /// It is [`Self::control_command_with_document`] with an empty document, so a
    /// `SESSION_REPLACE` frame sent here is refused `session.replace.document_length`.
    pub fn control_command(&mut self, len: u32, analysis: Option<&mut SpectrumAnalysis>) -> u32 {
        self.control_command_with_document(len, &[], analysis)
    }

    /// [`Self::control_command`] with the document a `SESSION_REPLACE` frame names (#1697, #1386
    /// D7), borrowed for this call only: the core checks its length and SHA-256, parses it and
    /// diffs it against the committed model, exactly as for the C ABI's
    /// `miso_engine_v1_submit_document_command`, and every other command ignores it. The browser
    /// decodes nothing in it either. `miso_engine_web_v1_control_command` passes the document
    /// `miso_engine_web_v1_replace_document_ptr` staged. Results as [`Self::control_command`].
    pub fn control_command_with_document(
        &mut self,
        len: u32,
        document: &[u8],
        analysis: Option<&mut SpectrumAnalysis>,
    ) -> u32 {
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
        match ready.run_command(request, document, CONTROL_FRAME_BYTES, analysis) {
            Ok(length) => {
                self.control_response = ControlResponse::Frame(length);
                self.record(RESULT_OK)
            }
            Err(error) => self.refuse_control(&error),
        }
    }

    /// Declare a discontinuity (#1323): the next block this host renders need not continue the
    /// previous output, because the page has stopped or is about to seek every source. The page
    /// calls it at a stop and before a seek of every source made with per-source seeks (a plain
    /// [`Self::session_seek`] declares it in the same call, root ruling B16-R9). It runs the
    /// control plane's
    /// `SessionState::declare_discontinuity`, the function the C ABI's
    /// `miso_engine_v1_declare_discontinuity` runs: if the newest plan runs behind its natural
    /// timing (a latency floor of a structural transaction raised it, #1285), the committed
    /// session is prepared again with no floors and only its sources carried, and published for
    /// the next block; otherwise nothing is published. The model and the revision do not change.
    /// Control work, as [`Self::control_command`]: never `process()`.
    ///
    /// Results: [`RESULT_OK`]; [`RESULT_WRONG_STATE`] for a host that is not ready; and the
    /// refusals of [`Self::control_command`] for a rebuild, with the core's diagnostic at
    /// [`Self::control_response`]. A refusal changes nothing.
    pub fn declare_discontinuity(&mut self, analysis: Option<&mut SpectrumAnalysis>) -> u32 {
        self.control_response = ControlResponse::None;
        if self.status.state != STATE_READY {
            return self.record(RESULT_WRONG_STATE);
        }
        let Some(ready) = self.ready.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        match ready.declare_discontinuity(analysis) {
            Ok(()) => self.record(RESULT_OK),
            Err(error) => self.refuse_control(&error),
        }
    }

    /// A refused control call (#1562a D1, #1323): the core's diagnostic into the session
    /// diagnostic and the response, and the result code for `error`.
    fn refuse_control(&mut self, error: &control_plane::CommandError) -> u32 {
        let Some(buffers) = self.buffers.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        let diagnostic = error.diagnostic();
        let kept = diagnostic.len().min(buffers.diagnostic.len());
        buffers.diagnostic[..kept].copy_from_slice(&diagnostic[..kept]);
        self.diagnostic_len = kept;
        self.control_response = ControlResponse::Diagnostic;
        self.record(match error {
            control_plane::CommandError::Invalid => RESULT_INVALID_ARGUMENT,
            control_plane::CommandError::BufferTooSmall { .. } => RESULT_BUFFER_TOO_SMALL,
            control_plane::CommandError::Backpressure => RESULT_BACKPRESSURE,
            // #1290a: a transaction that changes the rate or the quantum cannot be applied
            // to a running plan; the page re-prepares the session from its document.
            control_plane::CommandError::CompileRejected(_)
                if diagnostic.starts_with(control_plane::REPREPARE_REQUIRED.as_bytes()) =>
            {
                RESULT_REPREPARE_REQUIRED
            }
            control_plane::CommandError::CompileRejected(_) => RESULT_COMPILE_REJECTED,
            control_plane::CommandError::Internal => RESULT_INTERNAL,
        })
    }

    /// A refused source call (#1706 D3): the control plane's diagnostic for `failure` into the
    /// session diagnostic and the response, as a refused control call writes its own, and the
    /// result code of the failure's kind. It records nothing: each caller records exactly when
    /// its call records its results. The copy is into preallocated storage, so `submit_source`
    /// calls it from `process()`.
    pub(crate) fn refuse_source(&mut self, failure: SourceFailure) -> u32 {
        if let Some(buffers) = self.buffers.as_mut() {
            let diagnostic = failure.diagnostic().as_bytes();
            let kept = diagnostic.len().min(buffers.diagnostic.len());
            buffers.diagnostic[..kept].copy_from_slice(&diagnostic[..kept]);
            self.diagnostic_len = kept;
            self.control_response = ControlResponse::Diagnostic;
        }
        match failure.kind() {
            SourceRefusalKind::InvalidArgument => RESULT_INVALID_ARGUMENT,
            SourceRefusalKind::Backpressure => RESULT_BACKPRESSURE,
            SourceRefusalKind::Internal => RESULT_INTERNAL,
        }
    }

    /// A source ID the staging cannot hold (#1706 D4): the core's malformed-ID refusal, unrecorded
    /// as `refuse_source`.
    pub(crate) fn refuse_source_id(&mut self) -> u32 {
        self.refuse_source(SourceFailure::Control(SourceControlError::InvalidId))
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

    /// How many structural commands ran their rebuild on this instance's own thread (#1290a D6),
    /// saturating at `u32::MAX`: a page without cross-origin isolation runs the control half in
    /// the worklet's message handler, outside `process()`, so each published replacement plan is
    /// a blocking rebuild that decision 15 accepts only when it is counted and reported. A
    /// refused command, a live command and a command that fails after preparation count nothing.
    ///
    /// The status word [`WebStatus::blocking_rebuilds`] carries it to the page (#1294b), whose
    /// `miso.status.v1` reply names it `blockingRebuilds`; #1295's real-browser gate expects
    /// exactly one on the non-isolated leg and none on the isolated one.
    #[must_use]
    pub fn blocking_rebuilds(&self) -> u32 {
        self.ready
            .as_ref()
            .map_or(0, |ready| ready.blocking_rebuilds)
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
    ///
    /// `analysis` is the smoothing history of an instance that owns both halves of the spectrum
    /// stream, which a promotion this step performs ends when it does not keep the stream (#1618);
    /// the browser's audio thread passes `None`.
    pub fn service(&mut self, analysis: Option<&mut SpectrumAnalysis>) -> u32 {
        if self.status.state != STATE_READY {
            return RESULT_WRONG_STATE;
        }
        let Some(ready) = self.ready.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        let stream = ready.spectrum_stream();
        match ready.control.service() {
            Ok(()) => {
                ready.end_ended_stream(stream, analysis);
                ready.reclaim_shapes();
                RESULT_OK
            }
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

    /// Copy the active plan's output latency and tail into the host's record (#1688 D2).
    ///
    /// The words are the control plane's resource row for the active plan epoch, the same
    /// `PlanQueries::resources()` row the C ABI reports, with the tail in the control plane's
    /// two-word encoding: a successor's values appear from the first read after the block that
    /// adopts it. Like [`Self::service`] it leaves the status's last result alone.
    pub fn read_plan_latency(&mut self) -> u32 {
        if self.status.state != STATE_READY {
            return RESULT_WRONG_STATE;
        }
        let Some(ready) = self.ready.as_ref() else {
            return RESULT_WRONG_STATE;
        };
        let resources = ready.queries.resources();
        let (tail_kind, tail_samples) = control_plane::tail_words(resources.output_tail_every_peak);
        self.plan_latency = WebPlanLatency {
            struct_size: PLAN_LATENCY_BYTES,
            reserved0: 0,
            latency_samples: resources.latency_samples,
            tail_kind,
            tail_samples,
            reserved: [0; 4],
        };
        RESULT_OK
    }

    /// The record [`Self::read_plan_latency`] fills.
    #[must_use]
    pub const fn plan_latency(&self) -> &WebPlanLatency {
        &self.plan_latency
    }

    /// Copy where the staged source landed after its newest seek into the host's record (#1293
    /// D6): the control plane's `SessionState::seek_report`, the call the C ABI's
    /// `miso_engine_v1_source_seek_report` runs, after its service step. Control side only: the
    /// message handler, never `process()`.
    ///
    /// A host that is not ready refuses with [`RESULT_REFUSED_LIFECYCLE`]; a read that landed
    /// inside render's publication on every bounded attempt is [`RESULT_BACKPRESSURE`] (read
    /// again); a source the session does not declare is [`RESULT_INVALID_ARGUMENT`]. Every refusal
    /// leaves the record alone. Like [`Self::service`] it leaves the status's last result alone.
    pub fn read_seek_report(&mut self, source_id: &[u8]) -> u32 {
        self.control_response = ControlResponse::None;
        if self.status.state != STATE_READY {
            return RESULT_REFUSED_LIFECYCLE;
        }
        let Some(ready) = self.ready.as_mut() else {
            return RESULT_REFUSED_LIFECYCLE;
        };
        match ready.control.seek_report(source_id) {
            Ok(report) => {
                self.seek_report = WebSeekReport {
                    struct_size: SEEK_REPORT_BYTES,
                    reserved0: 0,
                    generation: report.generation,
                    first_sample: report.first_sample,
                    source_frame: report.source_frame,
                    cumulative_underrun_frames: report.cumulative_underrun_frames,
                    cumulative_held_frames: report.cumulative_held_frames,
                    reserved: [0; 2],
                };
                RESULT_OK
            }
            Err(failure) => self.refuse_source(failure),
        }
    }

    /// The record [`Self::read_seek_report`] fills.
    #[must_use]
    pub const fn seek_report(&self) -> &WebSeekReport {
        &self.seek_report
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
        self.ready.as_ref().map_or(&[], |ready| &ready.shape.tracks)
    }

    /// Canonical normalized submix order (issue #1210 D1): the trailing segment of the strip order,
    /// submix `j` being strip `live_control_tracks().len() + j`, in the order of the meter frame's
    /// submix sections.
    #[must_use]
    pub fn live_control_submixes(&self) -> &[Box<str>] {
        self.ready
            .as_ref()
            .map_or(&[], |ready| &ready.shape.submixes)
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
        self.ready
            .as_ref()
            .map_or(&[], |ready| &ready.shape.meter_frame)
    }

    /// The sample window and shape the `f32` frame cannot carry (issue #143 D5).
    #[must_use]
    pub fn meter_header(&self) -> &WebMeterHeader {
        self.ready
            .as_ref()
            .map_or(&EMPTY_METER_HEADER, |ready| &ready.shape.meter_header)
    }

    /// The full-width address of the header the rendering plan's header names as its successor,
    /// or `0` when none waits (a gate compares it with the candidate's or the adopted header's
    /// address; the header's own `u32` word reads `0` above 4 GiB on a native host).
    #[cfg(any(test, feature = "test-support"))]
    #[must_use]
    pub fn meter_successor_address(&self) -> usize {
        self.ready
            .as_ref()
            .map_or(0, |ready| ready.shape.successor_address)
    }

    /// Armed taps of the rendering plan, as their slots report them (#1585 D6): the taps render
    /// has armed. Off-ABI introspection for the tests.
    #[must_use]
    pub fn observation_armed_taps(&mut self) -> u32 {
        let Some(plan) = self.plan.as_mut() else {
            return 0;
        };
        plan.active_attachment_mut::<RenderCompanions>()
            .map_or(0, |companions| {
                companions
                    .effect_observations
                    .iter()
                    .flat_map(|handle| handle.readers.iter())
                    .filter(|reader| reader.read_state().is_some_and(|state| state.armed))
                    .count()
                    .try_into()
                    .unwrap_or(u32::MAX)
            })
    }

    /// Whether preparation bound any observation taps at all (issue #143 D3, level 1).
    #[must_use]
    pub fn observation_attached(&self) -> bool {
        self.ready.as_ref().is_some_and(|ready| {
            ready
                .shape
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

    /// The spectrum service of a ready host, or the result word that refuses the call: the
    /// session's one service (#1596 R1), which the spectrum frames read too, until #1597 deletes
    /// these exports.
    fn spectrum_service(&self) -> Result<&SpectrumService, u32> {
        if self.status.state != STATE_READY {
            return Err(RESULT_WRONG_STATE);
        }
        self.ready
            .as_ref()
            .ok_or(RESULT_WRONG_STATE)?
            .control
            .spectrum_monitor()
            .service()
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
            .control
            .spectrum_monitor_mut()
            .service_and_analysis_mut()
            .map(|(service, _)| service)
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
    /// a stream whose cadence differs from the graph that will produce its samples. `analysis` is
    /// the smoothing history of a caller that owns both halves; the browser's audio thread passes
    /// `None`.
    ///
    /// # Errors
    ///
    /// The result word of the service's refusal.
    pub fn start_spectrum_stream(
        &mut self,
        smoothing: SpectrumSmoothingConfig,
        analysis: Option<&mut SpectrumAnalysis>,
    ) -> Result<(), u32> {
        self.spectrum_service_mut()?
            .start_stream(smoothing, analysis)
            .map_err(spectrum_result)
    }

    /// Select one exact prepared entry, with the stream's restarting selection while a stream
    /// runs. Returns whether a running stream restarted (its metadata is then `Warming`).
    /// `smoothing` changes a running stream's smoothing; without a running stream it is a
    /// wrong state. `analysis` is the smoothing history of a caller that owns both halves.
    ///
    /// The collection performs all fallible admission before retiring its current capture, so a
    /// refusal leaves the old stream, queued window and effective configuration intact. A select
    /// that would replace the entry while the caller holds an armed window is refused as
    /// backpressure (decision 17 D7).
    ///
    /// The worklet calls this on the browser's audio thread, so the target is borrowed and the
    /// selection allocates nothing.
    ///
    /// # Errors
    ///
    /// The result word of the service's refusal.
    pub fn select_spectrum_target(
        &mut self,
        target: SpectrumTargetRef<'_>,
        channels: SpectrumChannels,
        smoothing: Option<SpectrumSmoothingConfig>,
        analysis: Option<&mut SpectrumAnalysis>,
    ) -> Result<bool, u32> {
        self.spectrum_service_mut()?
            .select_target(target, channels, smoothing, analysis)
            .map_err(spectrum_result)
    }

    /// Read the managed stream once and return the result word of its status. The stream metadata
    /// and the ready window are read with [`Self::spectrum_stream_metadata`] and
    /// [`Self::spectrum_stream_window`].
    pub fn read_spectrum_stream(&mut self, analysis: Option<&mut SpectrumAnalysis>) -> u32 {
        match self.spectrum_service_mut() {
            Ok(service) => match service.read_stream(analysis) {
                Ok(()) => spectrum_stream_result(service.stream_metadata().status),
                Err(error) => spectrum_result(error),
            },
            // A host without a stream reads as an inactive one.
            Err(_) => RESULT_WRONG_STATE,
        }
    }

    /// Stop the managed stream and discard its queued native window. With `analysis`, the
    /// analysis of the stream ends at the same call.
    pub fn stop_spectrum_stream(&mut self, analysis: Option<&mut SpectrumAnalysis>) -> u32 {
        match self.spectrum_service_mut() {
            Ok(service) => {
                service.stop_stream(analysis);
                RESULT_OK
            }
            Err(result) => result,
        }
    }

    /// The stream metadata, or `None` without a spectrum observer.
    #[must_use]
    pub fn spectrum_stream_metadata(&self) -> Option<&SpectrumStreamMetadata> {
        self.spectrum_service()
            .ok()
            .map(SpectrumService::stream_metadata)
    }

    /// The window of the last stream read that was ready.
    #[must_use]
    pub fn spectrum_stream_window(&self) -> Option<&SpectrumContinuousWindow> {
        self.spectrum_service()
            .ok()
            .and_then(SpectrumService::stream_window)
    }

    /// Return the active managed stream cadence, if it is running.
    #[must_use]
    pub fn spectrum_stream_cadence(&self) -> Option<SpectrumCadence> {
        self.spectrum_service()
            .ok()
            .and_then(SpectrumService::stream_cadence)
    }

    /// Return the native capture epoch for the active managed stream.
    #[must_use]
    pub fn spectrum_stream_epoch(&self) -> Option<u64> {
        self.spectrum_service()
            .ok()
            .and_then(SpectrumService::stream_epoch)
    }

    /// Cancel a pending spectrum capture and discard any completed window.
    pub fn cancel_spectrum(&mut self) -> u32 {
        match self.spectrum_service_mut() {
            Ok(service) => service
                .cancel()
                .map_or_else(spectrum_result, |()| RESULT_OK),
            Err(result) => result,
        }
    }

    /// The target kind configured for this prepared host, or zero when no spectrum observer exists.
    #[must_use]
    pub fn spectrum_target(&self) -> u32 {
        self.ready
            .as_ref()
            .and_then(|ready| ready.control.spectrum_monitor().service())
            .map_or(0, SpectrumService::target_kind)
    }

    /// The channel mask configured for this prepared host, or zero when no observer exists.
    #[must_use]
    pub fn spectrum_channels(&self) -> u32 {
        self.ready
            .as_ref()
            .and_then(|ready| ready.control.spectrum_monitor().service())
            .and_then(SpectrumService::channels)
            .map_or(0, spectrum_channels_raw)
    }

    /// Return the collection selection identity, or zero for the singular compatibility path.
    #[must_use]
    pub fn spectrum_selection_epoch(&self) -> u64 {
        self.ready
            .as_ref()
            .and_then(|ready| ready.control.spectrum_monitor().service())
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

        // #1586 D3: host-core checks the whole batch (every selection resolves, no tap twice)
        // before it writes a row, so a refused batch allocates no result vector.
        let handles = observation_handles(plan);
        let mut results: Vec<ObservationReadResult> = Vec::new();
        host_core::observation::read_with(
            &ready.shape.observation_index,
            handles,
            selections,
            |selection| Ok::<_, ObservationReadError>(core_selection(selection)),
            |key| observation_issued(ready, key),
            |position, entry, row| {
                if results.capacity() == 0 {
                    results.reserve_exact(selections.len());
                }
                let selection = &selections[position];
                let Some(tap) = row.tap else {
                    return;
                };
                results.push(ObservationReadResult {
                    track_id: entry.strip_id.clone(),
                    rack: selection.rack,
                    effect_slot_id: entry.effect_id.clone(),
                    native_effect_id: entry.effect.id.as_str(),
                    descriptor: *tap,
                    channels: selection.channels,
                    sample_rate_hz,
                    status: row.status,
                    window: row.window,
                    left: row.left,
                    right: row.right,
                });
            },
        )?;
        Ok(results)
    }

    /// Number of currently bound resident observation effects in dense owner order.
    #[must_use]
    pub fn observation_binding_count(&self) -> usize {
        self.ready.as_ref().map_or(0, |ready| {
            ready
                .shape
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
        for (effect, handle) in ready.shape.effect_instances.iter().enumerate() {
            let Some(handle) = handle.as_ref().filter(|handle| handle.reader.is_some()) else {
                continue;
            };
            if observed_index == index {
                let track_index = *ready.shape.observation_tracks.get(effect)?;
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
        // #1586 D3: each numeric address maps to its stable selection through the session map in
        // the same pass as host-core's check, so the first refusal in address order is returned
        // and nothing is written before the whole batch passed.
        let handles = observation_handles(plan);
        let sample_rate_hz = self.status.sample_rate_hz;
        host_core::observation::read_with(
            &ready.shape.observation_index,
            handles,
            addresses,
            |address| {
                observation_selection_for_address(ready, *address)
                    .map(|selection| core_selection(&selection))
            },
            |key| observation_issued(ready, key),
            |position, _, row| {
                output[position] = ObservationReadValues {
                    status: row.status,
                    sample_rate_hz,
                    window: row.window,
                    left: row.left,
                    right: row.right,
                };
            },
        )
    }

    /// Number of complete meter windows folded since compilation.
    #[must_use]
    pub fn meter_windows(&self) -> u64 {
        self.ready
            .as_ref()
            .map_or(0, |ready| ready.meters.folded_windows())
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

    /// Whether the host was prepared with a meter window (issue #137 D2). It is the
    /// configuration's word, not the plan's strip count: a session with no strip (#1682) is
    /// metered by the master alone, and takes the lease so that the first strip's meters are
    /// delivered from the block that adopts them.
    #[must_use]
    pub fn meters_attached(&self) -> bool {
        self.ready.is_some() && self.options.live_control_meter_blocks != 0
    }

    /// Take or release the meter lease. Returns the frozen result code.
    ///
    /// A lease over a host prepared with the boot word `0` is refused with [`RESULT_UNSUPPORTED`]:
    /// the caller asked for numbers this preparation does not deliver, and silently reporting
    /// zeros would be worse than saying so. The lease drives the output meter's gate (#1581 D5).
    pub fn set_meter_lease(&mut self, enabled: bool) -> u32 {
        if self.status.state != STATE_READY {
            return self.record(RESULT_WRONG_STATE);
        }
        if enabled && !self.meters_attached() {
            return self.record(RESULT_UNSUPPORTED);
        }
        // The lease is the core delivery's level (#1582 D4). Repeated acquisition/release is
        // idempotent. A real transition starts a new publication generation, clears the pending
        // state and the publication, marks producer-queued intervals from the previous lease for
        // bounded rejection by the next poll, and drives the output meter's gate (#1581 D5); the
        // cleared publication is copied into the page's frame and header.
        let from = self.status.next_absolute_sample;
        if let Some(ready) = self.ready.as_mut()
            && ready.meters.set_level(enabled, from)
        {
            publish_web_meter_frame(
                ready.meters.frame(),
                ready.shape.master_track,
                &mut ready.shape.meter_frame,
                &mut ready.shape.meter_header,
            );
        }
        self.record(RESULT_OK)
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

    /// Copy one canonical source ID into ID staging; returns its byte length (issue #207).
    ///
    /// Zero means "no such source", and also a staging that cannot hold the ID: staging is sized
    /// for the longest ID the grammar allows, so the second cannot happen for a compiled session.
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
            .unwrap_or(0)
    }

    /// Copy `bytes` into ID staging and return their length, or why they cannot be staged. Never
    /// panics: this runs inside a render-locked export, where a panic is a trap.
    fn copy_id_into_staging(
        buffers: Option<&mut PreparedBuffers>,
        bytes: &[u8],
    ) -> Result<u32, IdStagingError> {
        let buffers = buffers.ok_or(IdStagingError::NoStaging)?;
        let Some(target) = buffers.source_id.get_mut(..bytes.len()) else {
            return Err(IdStagingError::TooLong);
        };
        target.copy_from_slice(bytes);
        u32::try_from(bytes.len()).map_err(|_| IdStagingError::TooLong)
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
        self.control_response = ControlResponse::None;
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
            Err(failure) => self.refuse_source(failure),
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
        self.control_response = ControlResponse::None;
        if self.status.state != STATE_READY {
            return self.record(RESULT_WRONG_STATE);
        }
        let (Some(ready), Some(plan)) = (self.ready.as_mut(), self.plan.as_mut()) else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        // #1325: the consumer to prepare is the plan that renders, by its own source rows. Through
        // a strip removal's phase 1 that plan still renders a source the newest committed session
        // removed, and its source indices are not the newest session's. A source it does not
        // render (one a waiting successor adds, or one removed and adopted) has no consumer here
        // to prepare; the control plane's routing answers it.
        let rendering = ready
            .control
            .rendering_source_index(plan.owner().active_epoch().0, source_id);
        if rendering.is_some_and(|index| !plan.can_prepare_source_seek(index)) {
            return self.record(RESULT_WRONG_STATE);
        }
        let code = match ready
            .control
            .seek_unserviced(source_id, generation, source_frame)
        {
            Ok(()) => match rendering {
                Some(index) if !plan.prepare_source_seek(index, generation, source_frame) => {
                    RESULT_INTERNAL
                }
                _ => RESULT_OK,
            },
            Err(failure) => self.refuse_source(failure),
        };
        self.record(code)
    }

    /// Queue one anchored source seek (#1293 D5): `source_frame` enters the graph in the block that
    /// starts at absolute render sample `anchor_sample`.
    ///
    /// It runs the control plane's `SessionState::seek_at`, the call the C ABI's
    /// `miso_engine_v1_source_seek_at` runs, after its service step, so it is a control-side call
    /// (the message handler, never `process()`). It never prepares the plan's consumer: the
    /// anchored seek is held by the ring and the consumer applies it at its anchor block, where
    /// `prepare_source_seek` would refuse a held seek. A host that is not ready refuses with
    /// [`RESULT_REFUSED_LIFECYCLE`] and changes nothing; the core's refusals map through
    /// `refuse_source` (an unknown source or an anchor that is not a quantum multiple is
    /// [`RESULT_INVALID_ARGUMENT`]).
    pub fn seek_source_at(
        &mut self,
        source_id: &[u8],
        generation: u64,
        source_frame: u64,
        anchor_sample: u64,
    ) -> u32 {
        self.control_response = ControlResponse::None;
        if self.status.state != STATE_READY {
            return self.record(RESULT_REFUSED_LIFECYCLE);
        }
        let Some(ready) = self.ready.as_mut() else {
            return self.record(RESULT_REFUSED_LIFECYCLE);
        };
        let code = match ready
            .control
            .seek_at(source_id, generation, source_frame, anchor_sample)
        {
            Ok(()) => RESULT_OK,
            Err(failure) => self.refuse_source(failure),
        };
        self.record(code)
    }

    /// Seek the timeline and every source of the newest committed session to timeline sample
    /// `timeline_frame` under one new `generation`, all or nothing (#1643 D1).
    ///
    /// It runs the control plane's `SessionState::session_seek`, the call the C ABI's
    /// `miso_engine_v1_session_seek` and `_session_seek_at` run, after its service step, so it is
    /// a control-side call (the message handler, never `process()`). With `anchor_sample`, every
    /// consumer applies the seek in the block that starts there. This adapter holds no seek
    /// logic, and it never prepares the plan's consumer: every consumer observes the seek through
    /// its command queue at the next block, the timeline included. A host that is not ready
    /// refuses with [`RESULT_REFUSED_LIFECYCLE`] and changes nothing; the core's refusals map
    /// through `refuse_source` (a stale generation or an anchor that is not a quantum
    /// multiple is [`RESULT_INVALID_ARGUMENT`], one full slot [`RESULT_BACKPRESSURE`]). The plain
    /// form declares the discontinuity in the same call (root ruling B16-R9): a plan it publishes
    /// is kept and counted as [`Self::declare_discontinuity`]'s, and a refused declaration
    /// reports as that call's does, with nothing pushed. The anchored form declares nothing.
    pub fn session_seek(
        &mut self,
        analysis: Option<&mut SpectrumAnalysis>,
        generation: u64,
        timeline_frame: u64,
        anchor_sample: Option<u64>,
    ) -> u32 {
        self.control_response = ControlResponse::None;
        if self.status.state != STATE_READY {
            return self.record(RESULT_REFUSED_LIFECYCLE);
        }
        let Some(ready) = self.ready.as_mut() else {
            return self.record(RESULT_REFUSED_LIFECYCLE);
        };
        match ready.session_seek(analysis, generation, timeline_frame, anchor_sample) {
            Ok(()) => self.record(RESULT_OK),
            Err(SessionSeekFailure::Seek(failure)) => {
                let code = self.refuse_source(failure);
                self.record(code)
            }
            // B16-R9: the plain form's declaration refused, reported as
            // [`Self::declare_discontinuity`] reports its own.
            Err(SessionSeekFailure::Declaration(error)) => self.refuse_control(&error),
        }
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
            Ok(watermark) => {
                // #1349 D1/D2: copied from the report of the block that advanced the watermark;
                // a block that did not advance it leaves the three words as they were.
                if let Some(watermark) = watermark {
                    self.status.watermark_revision = watermark.revision;
                    self.status.watermark_first_sample = watermark.first_sample;
                    self.status.watermark_outcome_flags = watermark.outcome_flags;
                }
                let next_absolute_sample = plan.owner().next_absolute_sample();
                // #1290a: the block that adopted a replacement plan takes the replacement's own
                // tables, a pointer exchange that allocates and frees nothing; the control side
                // drops the retired ones at its next step.
                let epoch = plan.owner().active_epoch().0;
                if epoch != ready.shape_epoch {
                    // #882: the host's carried-meter fact (`adopt_shape`'s `meters_carried`) is the
                    // engine's, which chose this block's strip meter level.
                    debug_assert_eq!(
                        plan.active_attachment_mut::<RenderCompanions>()
                            .is_some_and(|companions| companions
                                .carried()
                                .iter()
                                .any(|reader| reader.kind == ObserverKind::Meter)),
                        plan.owner().active_strip_meters_carried(),
                        "the host and the engine disagree on whether the swap carried a meter"
                    );
                    // #1381b: what the swap block's hook paired travels with the tables.
                    let carried = plan
                        .active_attachment_mut::<RenderCompanions>()
                        .map_or(&[][..], |companions| companions.carried());
                    ready.adopt_shape(epoch, carried, render_first_sample);
                }
                self.status.next_absolute_sample = next_absolute_sample;
                self.status.rendered_quanta = self.status.rendered_quanta.saturating_add(1);
                // #1570b D3: the block's activity, published by the control plane's one rule
                // (the peak scan runs only for an armed observer, `NaN` otherwise), so the
                // service step stages the telemetry events and `TRANSPORT_GET` reports the
                // effective sample exactly as on the C ABI. The same call folds the session's
                // output meter (#1581 D5), whose gate the lease drives: the master peaks the poll
                // pairs with the strip windows. At most `4 * quantum` loads, no allocation,
                // indexed with `get` so no `slice_index_fail` joins the call graph.
                if let (Some(left), Some(right)) = (
                    buffers.output_pcm.get(..quantum),
                    buffers
                        .output_pcm
                        .get(quantum..)
                        .and_then(|tail| tail.get(..quantum)),
                ) {
                    plan.observe_rendered_block(
                        [left, right],
                        render_first_sample,
                        next_absolute_sample,
                    );
                }
                // #1399: the session's count, loaded after the block that added to it, so the
                // status is never behind the last rendered block.
                self.status.live_values_superseded = ready.live_values_superseded.get();
                // Issue #137 D1: every effect stage drained its observation queue at the top of
                // this block, so the exact free-slot count is the whole capacity again. Command-free
                // blocks have no counters to clear and skip the dense write pass entirely.
                if ready.has_in_flight_commands {
                    #[cfg(test)]
                    record_admission_counter_clear(ready.shape.in_flight.len());
                    ready.shape.in_flight.fill(0);
                    ready.has_in_flight_commands = false;
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
        // #1290a: against the newest committed plan, the successor between a rebuild's commit and
        // its adoption block (see `admit_on_newest_plan`).
        let admitted =
            ready.admit_on_newest_plan(|newest| admit_commands(newest, bytes, count as usize));
        let Ok(admitted) = admitted else {
            return self.fail(RESULT_INTERNAL, b"web.internal.pending_shape\t$\n");
        };
        match admitted {
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
    /// lease, which a host prepared with `live_control_meter_blocks == 0` never grants.
    ///
    /// Allocation-free by construction: it moves `Copy` snapshots out of bounded queues into a
    /// buffer allocated at compilation.
    pub fn poll_meters(&mut self) -> u32 {
        if self.status.state != STATE_READY {
            return 0;
        }
        let next_sample = self.status.next_absolute_sample;
        let (Some(ready), Some(plan)) = (self.ready.as_mut(), self.plan.as_mut()) else {
            return 0;
        };
        // #1381a D3: the meter consumers and the observation readers are the active plan's
        // companions. The downcast allocates and drops nothing.
        let Some(companions) = plan.active_attachment_mut::<RenderCompanions>() else {
            return 0;
        };
        // #1582 D4: the core delivery folds the windows; this export keeps only the copy. This
        // function has no runtime-checked `[]` index, so it adds no bounds check of its own. The
        // one `panic_bounds_check` in this export's call graph is the slot index of the SPSC
        // queues inlined into `MeterDelivery::poll`, which the ring keeps in range. This export
        // is called from `process()`, so the shipped artifact's call-graph gate covers it as it
        // covers the render export.
        let shape = &mut *ready.shape;
        let mut taps = WebGainReductionTaps {
            effect_instances: &shape.effect_instances,
            observation_tracks: &shape.observation_tracks,
            effect_observations: &companions.effect_observations,
            #[cfg(test)]
            scans: 0,
        };
        let published = ready
            .meters
            .poll(&mut companions.meters, &mut taps, next_sample);
        #[cfg(test)]
        {
            ready.meter_effect_scan_count =
                ready.meter_effect_scan_count.saturating_add(taps.scans);
        }
        let Some(frame) = published else {
            return 0;
        };
        // R4: one copy per published window.
        publish_web_meter_frame(
            frame,
            shape.master_track,
            &mut shape.meter_frame,
            &mut shape.meter_header,
        );
        frame.windows
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
        self.ready = None;
        self.plan = None;
        self.buffers = None;
        self.diagnostic_len = 0;
        self.status.state = STATE_DISPOSED;
        self.record(RESULT_OK)
    }

    /// Every control call returns its result through here, so the status's `blocking_rebuilds`
    /// word (#1294b) is refreshed in this one place: current after the control call that rebuilt,
    /// zero after a dispose (the ready half is gone), as [`Self::blocking_rebuilds`] reports.
    fn record(&mut self, code: u32) -> u32 {
        self.status.last_result = code;
        self.status.blocking_rebuilds = self.blocking_rebuilds();
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

/// One decoded, still-unapplied `miso.command.v1` record (issue #137 D1, #1677).
///
/// # The frozen 272-byte little-endian layout
///
/// | offset | width | field |
/// |---|---|---|
/// | 0 | u8 | `kind`: [`COMMAND_OBSERVE_SUBSCRIBE`] or [`COMMAND_OBSERVE_UNSUBSCRIBE`] |
/// | 1 | u8 | `rack`: `1` inserts, `3` console; `0` and `2` retired |
/// | 2 | u8 | `strip_id_bytes`: the length of the strip ID, `1..=127` |
/// | 3 | u8 | `effect_id_bytes`: the length of the console slot's or insert's ID, `1..=127` |
/// | 4 | u32 | `tap_id`: the effect-local tap ID |
/// | 8 | u32 | `window_blocks`: the window length in render blocks (`0` = the plan's default) |
/// | 12 | u32 | required zero |
/// | 16 | 128 bytes | `strip_id`: the strip's ID, zero padded |
/// | 144 | 128 bytes | `effect_id`: the console slot's or insert's ID, zero padded |
///
/// The record names its tap by the stable key `OBSERVATION_ENDED` uses (strip, section, slot or
/// insert ID, tap ID). No index of any plan is in it, so the core resolves the key on the plan it
/// admits the record against, and a rebuild between a client's last read and the record cannot make
/// it arm another effect's tap. Kinds 1-6 and 9-17 are deleted (#1565A) and their numbers are never
/// reallocated. There is no string parsing on this path: the IDs are compared as bytes with the
/// plan's own.
#[derive(Clone, Copy)]
struct CommandRecord<'a> {
    kind: u32,
    rack: u8,
    tap_id: u32,
    window_blocks: u32,
    strip_id: &'a [u8],
    effect_id: &'a [u8],
}

impl<'a> CommandRecord<'a> {
    fn decode(bytes: &'a [u8]) -> Result<Self, u32> {
        const ID: usize = RECORD_ID_FIELD_BYTES as usize;
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
        if word(12) != 0 {
            return Err(COMMAND_REASON_MALFORMED);
        }
        let kind = u32::from(bytes[0]);
        if !matches!(
            kind,
            COMMAND_OBSERVE_SUBSCRIBE | COMMAND_OBSERVE_UNSUBSCRIBE
        ) {
            return Err(COMMAND_REASON_MALFORMED);
        }
        // An ID field holds its bytes and then zeros: a length of zero, a length past the field,
        // or a byte after the length, is a record that does not say what it names. A stable ID is
        // 1..=127 bytes, so an empty one names nothing.
        let id = |length: u8, offset: usize| -> Result<&'a [u8], u32> {
            let length = usize::from(length);
            let field = &bytes[offset..offset + ID];
            if length == 0 || length >= ID || field[length..].iter().any(|byte| *byte != 0) {
                return Err(COMMAND_REASON_MALFORMED);
            }
            Ok(&field[..length])
        };
        Ok(Self {
            kind,
            rack: bytes[1],
            tap_id: word(4),
            window_blocks: word(8),
            strip_id: id(bytes[2], 16)?,
            effect_id: id(bytes[3], 16 + ID)?,
        })
    }

    /// The stable key this record names, for the control plane to resolve on the plan it admits
    /// against (#1677). `rack` is the one part of the key the record spells as a code: an unknown
    /// code, the retired `simd1` and `simd2` among them, is [`COMMAND_REASON_UNKNOWN_RACK`].
    fn key(&self) -> Result<ObservationKeyRef<'a>, u32> {
        let rack = match self.rack {
            RACK_INSERTS => LiveEffectRack::Inserts,
            RACK_CONSOLE => LiveEffectRack::Console,
            _ => return Err(COMMAND_REASON_UNKNOWN_RACK),
        };
        Ok(ObservationKeyRef {
            strip_id: self.strip_id,
            rack,
            effect_id: self.effect_id,
            tap_id: self.tap_id,
        })
    }
}

/// The reason a key the control plane could not resolve is refused with (#1677).
const fn key_refusal(error: ObservationKeyError) -> u32 {
    match error {
        ObservationKeyError::UnknownStrip => COMMAND_REASON_UNKNOWN_TRACK,
        ObservationKeyError::UnknownEffect => COMMAND_REASON_UNKNOWN_EFFECT,
        ObservationKeyError::UnknownTap => COMMAND_REASON_UNKNOWN_TAP,
        ObservationKeyError::ComputedTap => COMMAND_REASON_UNSUPPORTED_KIND,
    }
}

/// Validate a whole staged submission, then admit it. Nothing is pushed unless everything passes.
///
/// # The two passes are the contract, not an optimisation (#137 D1, #1565A)
///
/// Pass one decodes and lowers every record and counts the room each effect's observation queue
/// needs, then checks that room against every queue the submission touches. Pass two pushes. A
/// submission is therefore all-or-nothing: the refusal names the first record that broke a rule,
/// and a refused submission leaves every queue as it was.
fn admit_commands(
    ready: &mut ReadyOwnership,
    bytes: &[u8],
    count: usize,
) -> Result<(), CommandRejection> {
    let record_bytes = COMMAND_RECORD_BYTES as usize;
    ready.shape.command_wanted.fill(0);
    ready.shape.observe_named.fill(0);
    let refuse = |reason: u32, index: usize| CommandRejection {
        result: match reason {
            // `ObservationUnbound` is "this preparation cannot deliver it", which is exactly what
            // `RESULT_UNSUPPORTED` means; `UnknownTap` is a bad address, like every other unknown.
            // `ObservationLimit` is the same answer for the configuration: this session's control
            // frame cannot carry the end of one more tap, and only an unsubscribe changes that.
            COMMAND_REASON_UNSUPPORTED_KIND
            | COMMAND_REASON_OBSERVATION_UNBOUND
            | COMMAND_REASON_OBSERVATION_LIMIT => RESULT_UNSUPPORTED,
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
        // The record names its tap by stable key, and the control plane resolves it on the newest
        // committed plan, the one this submission is admitted against (#1677). Every refusal is
        // here, before the room pass and any write.
        let key = command.key().map_err(|reason| refuse(reason, index))?;
        let resolved = ready
            .control
            .resolve_observation_key(&key)
            .map_err(|error| refuse(key_refusal(error), index))?;
        let tap_index = resolved.tap_index;
        let armed = command.kind == COMMAND_OBSERVE_SUBSCRIBE;
        let window_blocks = command.window_blocks;
        // The tables of the same plan reach the effect's per-slot state from its producer.
        let Some((effect, observed)) = ready
            .shape
            .producer_slots
            .get(resolved.producer)
            .map(|slot| *slot as usize)
            .and_then(|slot| {
                let instance = ready.shape.effect_instances.get(slot)?.as_ref()?;
                Some((slot, instance.reader.is_some()))
            })
        else {
            return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, index));
        };
        // The tap exists and the key is right; this *session* bound no lane to arm.
        if !observed {
            return Err(refuse(COMMAND_REASON_OBSERVATION_UNBOUND, index));
        }
        // One submission names each tap once: a record overwrites its tap's whole state and the
        // submission drains in one block, so the second record would be a supersession the engine
        // must not make (D15-2: Observe records are never superseded) and would not fit the tap's
        // queue slot. Refused here, at this record's wire index, before the room pass or any
        // write.
        let bit = 1_u32
            .checked_shl(tap_index)
            .ok_or_else(|| refuse(COMMAND_REASON_MALFORMED, index))?;
        let named = ready
            .shape
            .observe_named
            .get_mut(effect)
            .ok_or_else(|| refuse(COMMAND_REASON_MALFORMED, index))?;
        if *named & bit != 0 {
            return Err(refuse(COMMAND_REASON_MALFORMED, index));
        }
        *named |= bit;
        let Some(wanted) = ready.shape.command_wanted.get_mut(effect) else {
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
    // The in-flight counter of an effect counts its records the render side has not drained. The
    // queue's room bounds it far below `u32::MAX`, so this refusal is a defence that never fires;
    // it runs before the core is lent the batch because nothing may be refused after a push.
    for entry in 0..staged_count {
        let staged = ready.command_decoded[entry];
        let effect = staged.effect as usize;
        if ready.shape.in_flight[effect]
            .checked_add(ready.shape.command_wanted[effect])
            .is_none()
        {
            return Err(refuse(
                COMMAND_REASON_BACKPRESSURE,
                staged.original_wire_index as usize,
            ));
        }
    }
    // The frame, the room and the push are one core call (#1676, #1685): the control plane judges
    // the whole submission by the encoded bytes of the one `OBSERVATION_ENDED` event the taps it
    // leaves armed would need, then by the room of every effect's observation queue, and only then
    // queues it. A subscription whose event would outgrow the control frame is refused there,
    // before any record is queued and so before any is acknowledged, and the adapter reports it as
    // `observationLimit` at the record that crossed it.
    // A change's index in the batch is its staged index, and the staged index names the record's
    // wire index.
    let staged = &ready.command_decoded[..staged_count];
    let shape = &ready.shape;
    let admitted = ready
        .control
        .admit_and_push_observation_batch(staged.iter().map(|staged| {
            ObservationChange {
                // Pass one resolved every record to a producer; a position that vanished names no
                // producer and is refused as such.
                position: ReadyOwnership::effect_producer_position(shape, staged.effect as usize)
                    .unwrap_or(usize::MAX),
                tap_index: staged.tap_index,
                armed: staged.armed,
                window_blocks: staged.window_blocks,
            }
        }));
    if let Err(refusal) = admitted {
        let (reason, index) = match refusal {
            ObservationRefusal::Limit(limit) => (COMMAND_REASON_OBSERVATION_LIMIT, limit.index),
            ObservationRefusal::Full { index } => (COMMAND_REASON_BACKPRESSURE, index),
            ObservationRefusal::NoProducer { index } => (COMMAND_REASON_UNSUPPORTED_KIND, index),
            // Pass one already refuses a second naming of a tap as malformed at that record; the
            // core's own check gives the same answer, so the browser's outcome is one.
            ObservationRefusal::Duplicate { index } => (COMMAND_REASON_MALFORMED, index),
            // Pass one resolved the tap from the effect's menu and refused an effect with no
            // reader as `observationUnbound`; the core's own slot check gives the same answer.
            ObservationRefusal::Unbound { index } => (COMMAND_REASON_OBSERVATION_UNBOUND, index),
        };
        let wire = staged
            .get(index)
            .map_or(0, |staged| staged.original_wire_index as usize);
        return Err(refuse(reason, wire));
    }
    debug_assert!(
        ready.control.observation_ended_bytes() <= ready.control.observation_frame_bytes(),
        "a browser admission left an ended event of {} bytes past the frame of {}",
        ready.control.observation_ended_bytes(),
        ready.control.observation_frame_bytes()
    );
    for entry in 0..staged_count {
        let staged = ready.command_decoded[entry];
        ready.shape.in_flight[staged.effect as usize] += 1;
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
        .shape
        .tracks
        .get(strip)
        .or_else(|| {
            ready
                .shape
                .submixes
                .get(strip.checked_sub(ready.shape.tracks.len())?)
        })
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
        .shape
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

/// One plan's observation taps as the core's gain-reduction fold sees them (#1582 D3): the
/// shape's dense effect table and strip table over the plan companions' reader sets. Each tap's
/// arm state is its slot's, which render writes (#1585 D6).
struct WebGainReductionTaps<'a> {
    effect_instances: &'a [Option<EffectInstance>],
    observation_tracks: &'a [u32],
    effect_observations: &'a [EffectObservationHandle],
    /// Test-only: the effect slots visited, so a test proves an incomplete master interval never
    /// reaches the scan.
    #[cfg(test)]
    scans: u64,
}

// REALTIME_POLICY_BEGIN: #1582 D3, the browser's taps for the meter frame's fold.
impl GainReductionTaps for WebGainReductionTaps<'_> {
    fn for_each_tap(&mut self, visit: &mut impl FnMut(TapView<'_>)) {
        for (effect, entry) in self.effect_instances.iter().enumerate() {
            #[cfg(test)]
            {
                self.scans = self.scans.saturating_add(1);
            }
            let Some(observed) = entry.as_ref() else {
                continue;
            };
            let Some(handle) = observed
                .reader
                .and_then(|reader| self.effect_observations.get(reader as usize))
            else {
                continue;
            };
            let Some(strip) = self.observation_tracks.get(effect).copied() else {
                continue;
            };
            // Issue #1209 D5: the frame carries one gain-reduction word per strip, so a submix's
            // effect folds into its own word at strip `T + j`.
            if strip == u32::MAX {
                continue;
            }
            for (reader, descriptor) in handle.readers.iter().zip(observed.descriptor.observations)
            {
                // A torn read is no arm this poll: the next poll reads the tap again.
                let Some(state) = reader.read_state() else {
                    continue;
                };
                visit(TapView {
                    strip,
                    descriptor,
                    reader,
                    armed: state.armed,
                    arm_sample: state.arm_sample,
                });
            }
        }
    }
}

/// Copy the core frame into the page's frame words and header (#1582 D4), the layout of
/// [`PlanShape::meter_frame`]: every strip's peak pair, tracks then submixes (issue #1209 D1, so
/// the master follows the last submix rather than the last track), the master's, every strip's
/// gain-reduction word, then the master's ([`MeterFrame::master_gain_reduction`] of `master`, the
/// plan's master strip; `0.0` when absent). Bounded by the strip count.
fn publish_web_meter_frame(
    frame: &MeterFrame,
    master: Option<u32>,
    words: &mut [f32],
    header: &mut WebMeterHeader,
) {
    let mut slots = words.iter_mut();
    // The source goes first in each zip, so its end leaves the next slot unconsumed.
    for (peak, slot) in frame.strip_peaks.iter().flatten().zip(&mut slots) {
        *slot = *peak;
    }
    let master_reading = frame.master_gain_reduction(master);
    let master_gain = master_reading.unwrap_or(0.0);
    let tail = frame
        .output_peak
        .iter()
        .chain(&frame.strip_gain_reduction_db)
        .chain(core::iter::once(&master_gain));
    for (value, slot) in tail.zip(slots) {
        *slot = *value;
    }
    header.windows = frame.windows;
    header.sequence = frame.sequence;
    header.master_gr_present = u32::from(master_reading.is_some());
    header.first_sample = frame.first_sample;
    header.end_sample = frame.end_sample;
    header.reserved[0] = frame.generation;
    header.reserved[1] = u64::from(frame.validity.bits())
        | (frame.loss_count.min(u64::from(u32::MAX)) << METER_LOSS_SHIFT);
}
// REALTIME_POLICY_END

/// The active plan's observation handles (#1381a D3: the readers are its companions), which the
/// read reads through: after a carried swap they hold the carried readers (#1586a). Empty for a
/// plan with no companions, whose instances then all read as unbound.
fn observation_handles(plan: &mut WebPlan) -> &[EffectObservationHandle] {
    plan.active_attachment_mut::<RenderCompanions>()
        .map_or(&[], |companions| &*companions.effect_observations)
}

/// A stable browser selection in host-core's terms.
const fn core_selection<'a>(
    selection: &ObservationSelection<'a>,
) -> host_core::observation::ObservationSelection<'a> {
    host_core::observation::ObservationSelection {
        strip_id: selection.track_id,
        section: selection.rack,
        effect_id: selection.effect_slot_id,
        tap_id: selection.tap_id,
        channels: selection.channels,
    }
}

/// The tag and kind of the last record the control plane issued for one tap (#1585 D4), looked
/// up by its stable key on the newest plan.
fn observation_issued(
    ready: &ReadyOwnership,
    key: &host_core::observation::ObservationKey<'_>,
) -> (u64, bool) {
    ready.control.observation_issued(&ObservationKeyRef {
        strip_id: key.strip_id.as_bytes(),
        rack: key.section,
        effect_id: key.effect_id.as_bytes(),
        tap_id: key.tap_id,
    })
}

impl From<host_core::observation::ObservationReadError> for ObservationReadError {
    fn from(error: host_core::observation::ObservationReadError) -> Self {
        use host_core::observation::ObservationRefusal as Refusal;
        match error.refusal {
            Refusal::TooManySelections | Refusal::RowsTooShort => Self::BufferTooSmall,
            Refusal::UnknownAddress | Refusal::UnknownTap | Refusal::Duplicate => {
                Self::InvalidSelection
            }
            Refusal::UnsupportedTap | Refusal::Unbound => Self::Unsupported,
        }
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

/// Why an ID could not be copied into ID staging.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum IdStagingError {
    /// The host owns no staging buffers.
    NoStaging,
    /// The ID is longer than the staging buffer.
    TooLong,
}

fn project_buffers(
    document_bytes: u32,
    sample_rate_hz: u32,
    quantum_frames: u32,
    options: WebBootOptions,
    spectrum_staging: (bool, (u64, u64)),
) -> Result<PreparedBufferProjection, BootFailure> {
    // The staging holds what a rebuild may add, not only what the boot session declares (#1562b):
    // a source, track or submix ID of the longest length the grammar allows, and a source of the
    // most channels a source can have (two). Sizing from the boot session would let an acked rebuild declare a
    // source the host could never stage or feed.
    let maximum_source_channels = SOURCE_CHANNELS_LAUNCH_MAXIMUM;
    let id_staging_bytes = STABLE_ID_MAXIMUM_BYTES as u64;
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
    // #1689: the word at offset 56 was the master boot word, reserved the same way.
    if options.reserved2 != 0 {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_OPTIONS,
            "web.options.reserved2",
        ));
    }
    if options.live_control_observation_taps > u64::from(MAXIMUM_OBSERVATION_TAPS)
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
        // The boot sets the real figure once its bridge rows are exact (`compile_ready`).
        maximum_replacement_peak_bytes: memory_budget,
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
    /// #1596: the browser reads spectrum through the session's three frames, as the C ABI does.
    const SPECTRUM: bool = true;
    /// #1697: `miso_engine_web_v1_replace_document_ptr` stages the document a `SESSION_REPLACE`
    /// names, and [`AudioWorkletEngineHost::control_command_with_document`] passes it beside the
    /// frame.
    const DOCUMENT_TRANSPORT: bool = true;
}

/// The control-protocol session the browser host owns (#1381a): the committed model at its boot
/// revision, the source, strip and effect producers, the plan publisher and the retirer.
type WebSession = SessionState<WebAdapter>;
/// The render half of the browser's plan exchange: the plan owner that renders and adopts.
type WebPlan = PlanState<WebAdapter>;

/// The render-side halves a browser plan's preparation returns beside the plan (issue #1400 D5):
/// the meter consumers and the effect observation handles.
///
/// They travel inside the plan as its [`HostAttachment`], so a plan the control plane prepares
/// carries everything the render side reads, and they drop with the plan on the retirement path.
/// The browser renders through the plan owner (#1381a D4) and reaches the active plan's companions
/// through `PlanState::active_attachment_mut`: the meter poll and the observation reads read the
/// readers there. A plan's spectrum captures are not among them: the preparer returns them to the
/// session, whose spectrum monitor installs them at the plan's promotion (#1596 D3).
///
/// Field order is the drop order: the meter consumers, then the effect observation handles, the
/// relative order the same two fields have in `ReadyOwnership`. Neither reads the other while it
/// drops; the order is kept so the two owners release alike.
pub struct RenderCompanions {
    /// One consumer per selected meter, in strip order; empty when metering is off.
    meters: Vec<MeterConsumer>,
    /// One reader set per observed effect instance, in `HostLiveControlHandles` order.
    effect_observations: Vec<EffectObservationHandle>,
    /// What the swap block that adopted this plan carried (#1381b): one move per paired meter or
    /// observation reader, written by [`HostAttachment::adopt_predecessor`] and read once by the
    /// block's table hand-over ([`ReadyOwnership::adopt_shape`]). Its length is fixed at
    /// preparation, one entry per reader, so the hook never grows it.
    carried: Box<[CarriedReader]>,
    /// How many entries of `carried` the last swap block wrote.
    carried_len: u32,
}

/// One reader the swap block paired with the predecessor's (#1381b): `successor` and `predecessor`
/// are positions in the two plans' own reader vectors (host-core's order), the indices of the
/// engine's carry record.
#[derive(Clone, Copy)]
struct CarriedReader {
    kind: ObserverKind,
    successor: u32,
    predecessor: u32,
    /// For a meter, the handles the two positions' meters publish under (`0` for any other kind):
    /// the poll accepts the predecessor's for windows it closed before the swap (#1290d1).
    successor_handle: u64,
    predecessor_handle: u64,
}

impl CarriedReader {
    /// The value of an unused entry.
    const UNUSED: Self = Self {
        kind: ObserverKind::Meter,
        successor: 0,
        predecessor: 0,
        successor_handle: 0,
        predecessor_handle: 0,
    };
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
            meters,
            effect_observations,
            carried,
            carried_len: _,
        } = self;
        let shell = u64::try_from(size_of::<Self>()).ok()?;
        let carried = u64::try_from(carried.len())
            .ok()?
            .checked_mul(u64::try_from(size_of::<CarriedReader>()).ok()?)?;
        let meters = u64::try_from(meters.capacity())
            .ok()?
            .checked_mul(u64::try_from(size_of::<MeterConsumer>()).ok()?)?;
        let observations = u64::try_from(effect_observations.capacity())
            .ok()?
            .checked_mul(u64::try_from(size_of::<EffectObservationHandle>()).ok()?)?;
        let total = shell
            .checked_add(meters)?
            .checked_add(observations)?
            .checked_add(carried)?;
        Some((total, shell.max(meters).max(observations).max(carried)))
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

/// What a preparation leaves for the host (#1381a D1): the facts of the first plan that
/// `PlanResources` and the attachment do not carry, and (#1290a) the tables of the plan the preparer
/// prepared last. The preparer writes it, boot and a committed command read it.
#[derive(Default)]
struct BootRecord {
    /// The engine-owned report of the boot plan, after the attachment's rows were charged.
    report: Option<HostPrepareReport>,
    /// The session's `live_values_superseded` count, shared with every cell reader of its plans.
    superseded: Option<host_core::LiveValuesSuperseded>,
    /// The class of the preparation refusal, so a refused boot keeps its result code.
    rejection: Option<PrepareRejection>,
    /// The tables of the plan the preparer prepared last (#1290a): one function builds the boot
    /// plan's and every replacement plan's. Boot takes the first; a command that publishes a
    /// replacement takes that plan's, and one that fails after preparing leaves a table to drop.
    shape: Option<PlanShape>,
}

impl RenderCompanions {
    /// What the swap block that adopted this plan paired (#1381b); empty when nothing was.
    fn carried(&self) -> &[CarriedReader] {
        self.carried.get(..self.carried_len as usize).unwrap_or(&[])
    }
}

impl HostAttachment for RenderCompanions {
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }

    // REALTIME_POLICY_BEGIN
    /// Pair the readers the executors carried (#1381 D3; #1327 D3): the successor's meter
    /// consumers and observation reader sets swap with the predecessor's at the records the
    /// engine's carry names, through host-core's [`host_core::pair_carried_meters`] and
    /// [`host_core::pair_carried_observations`]. The browser keeps both vectors in host-core's
    /// order, so the record's indices address them directly. The pairs are noted for the block's
    /// table hand-over, which moves the meter delivery's per-meter state with them.
    ///
    /// Render thread, in the swap block: swaps and scalar writes into storage sized at
    /// preparation. A predecessor of another type, or a record with no reader, pairs nothing.
    fn adopt_predecessor(
        &mut self,
        predecessor: &mut dyn HostAttachment,
        carries: &[ObserverCarry],
    ) {
        self.carried_len = 0;
        let Some(previous) = predecessor.as_any_mut().downcast_mut::<Self>() else {
            return;
        };
        host_core::pair_carried_meters(&mut self.meters, &mut previous.meters, carries);
        host_core::pair_carried_observations(
            &mut self.effect_observations,
            &mut previous.effect_observations,
            carries,
        );
        let mut written = 0;
        for carry in carries {
            let (successor_len, predecessor_len) = match carry.kind {
                ObserverKind::Meter => (self.meters.len(), previous.meters.len()),
                ObserverKind::Observation => (
                    self.effect_observations.len(),
                    previous.effect_observations.len(),
                ),
                ObserverKind::Spectrum => continue,
            };
            if carry.successor >= successor_len || carry.predecessor >= predecessor_len {
                continue;
            }
            let (Ok(successor), Ok(predecessor)) = (
                u32::try_from(carry.successor),
                u32::try_from(carry.predecessor),
            ) else {
                continue;
            };
            let Some(entry) = self.carried.get_mut(written) else {
                break;
            };
            let (successor_handle, predecessor_handle) = match carry.kind {
                ObserverKind::Meter => (
                    self.meters
                        .get(carry.successor)
                        .map_or(0, |meter| meter.handle.0.get()),
                    previous
                        .meters
                        .get(carry.predecessor)
                        .map_or(0, |meter| meter.handle.0.get()),
                ),
                _ => (0, 0),
            };
            *entry = CarriedReader {
                kind: carry.kind,
                successor,
                predecessor,
                successor_handle,
                predecessor_handle,
            };
            written += 1;
        }
        self.carried_len = u32::try_from(written).unwrap_or(0);
    }
    // REALTIME_POLICY_END
}

/// The diagnostic code of a preparation that could not leave the plan's tables for the host (a
/// record another thread poisoned): refused before the transaction is committed (#1290a).
const SHAPE_UNAVAILABLE: &str = "web.apply.shape_unavailable";

/// The browser's [`control_plane::RuntimePreparer`] (issue #1400 D5): host-core's one entry
/// ([`host_core::prepare_host_runtime_with_live_lanes_and_spectrum`]) for the session's boot
/// options and the spectrum configuration the session passes (#1593 D3), with the render-side
/// companions moved into the plan.
///
/// Every plan, the boot plan and every successor, delivers controls concurrently and meters what
/// host-core's selection meters (issue #1581 D1, [`host_core::meter_selection`]): one `PostMatrix`
/// sample-peak meter per strip, the tracks then the submixes, so `meters[i]` is strip `i`'s peak
/// pair. A successor prepares every source it leaves unchanged vacant, to carry the displaced
/// plan's ring.
///
/// It attaches the live lanes the control plane chose ([`controls::live_lanes`], passed in as the
/// `lanes` argument), so the boot word chooses no lane here.
pub struct WebRuntimePreparer {
    options: WebBootOptions,
    /// Where the boot preparation leaves its facts for the host (#1381a), or `None` for a preparer
    /// that is only called.
    record: Option<Arc<Mutex<BootRecord>>>,
}

impl WebRuntimePreparer {
    /// A preparer for a session booted with `options`. The spectrum request is the session's
    /// (#1593 D3): every call takes it as an argument.
    #[must_use]
    pub fn new(options: WebBootOptions) -> Self {
        Self {
            options,
            record: None,
        }
    }

    /// The same preparer, leaving its boot facts in `record`.
    fn recording(mut self, record: Arc<Mutex<BootRecord>>) -> Self {
        self.record = Some(record);
        self
    }
}

impl control_plane::RuntimePreparer for WebRuntimePreparer {
    fn prepare(
        &self,
        compiled: &CompiledSession,
        caps: &HostPrepareCaps,
        lanes: HostLiveLanes,
        // The session's configuration (#1593 D3): the boot request, then whatever of it the
        // session's edits left. The captures go to the session, whose spectrum monitor installs
        // them at the plan's promotion and serves every read (#1596 D3), as on the C ABI.
        spectrum: Option<&host_core::SpectrumPreparation>,
        successor: Option<SuccessorBase<'_>>,
        superseded: &host_core::LiveValuesSuperseded,
    ) -> Result<
        (
            PreparedHost,
            host_core::HostLiveControlHandles,
            Option<PreparedSpectrumCaptures>,
        ),
        PrepareDiagnostics,
    > {
        let prepared = self.prepare_plan(compiled, caps, lanes, spectrum, successor, superseded);
        if let (Err(failure), Some(record)) = (&prepared, &self.record)
            && let Ok(mut record) = record.lock()
        {
            record.rejection = Some(failure.kind());
        }
        prepared
    }

    /// The boot word (#1581 T2 obligation), so the session's output meter closes its windows on
    /// the strip meters' grid: `0` is host-core's default for both.
    fn meter_window_blocks(&self) -> u32 {
        u32::try_from(self.options.live_control_meter_blocks).unwrap_or(0)
    }
}

impl WebRuntimePreparer {
    /// The preparation itself. With no `successor` it prepares the session's first plan, whose
    /// facts the host's boot reads from the record.
    fn prepare_plan(
        &self,
        compiled: &CompiledSession,
        caps: &HostPrepareCaps,
        lanes: HostLiveLanes,
        spectrum: Option<&host_core::SpectrumPreparation>,
        successor: Option<SuccessorBase<'_>>,
        superseded: &host_core::LiveValuesSuperseded,
    ) -> Result<
        (
            PreparedHost,
            host_core::HostLiveControlHandles,
            Option<PreparedSpectrumCaptures>,
        ),
        PrepareDiagnostics,
    > {
        let boot = successor.is_none();
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
        let (mut host, mut handles, spectrum_captures) =
            host_core::prepare_host_runtime_with_live_lanes_and_spectrum(
                compiled,
                caps,
                &live_controls,
                lanes,
                spectrum,
                successor,
                superseded,
            )?;
        // #1290a: the plan's tables, built from the plan's own model, producers and companions.
        // They go to the host through the record: the boot's at boot, a replacement's when its
        // command commits. The boot report charges the boot's to the bridge rows; a replacement
        // plan charges its own to the plan's rows, so the swap budget counts them.
        let shape = build_plan_shape(
            compiled.normalized_model(),
            &handles.effect_controls,
            &handles.effect_observations,
            handles.meters.len(),
        )
        .map_err(|code| {
            let rejection = if code.starts_with("web.resource.") {
                PrepareRejection::Platform
            } else {
                PrepareRejection::Session
            };
            PrepareDiagnostics::fixed(rejection, code)
        })?;
        let meters = core::mem::take(&mut handles.meters);
        let effect_observations = core::mem::take(&mut handles.effect_observations);
        let mut carried: Vec<CarriedReader> = Vec::new();
        carried
            .try_reserve_exact(meters.len() + effect_observations.len())
            .map_err(|_| {
                PrepareDiagnostics::fixed(PrepareRejection::Platform, "web.resource.allocation")
            })?;
        carried.resize(
            meters.len() + effect_observations.len(),
            CarriedReader::UNUSED,
        );
        let companions = RenderCompanions {
            meters,
            effect_observations,
            carried: carried.into_boxed_slice(),
            carried_len: 0,
        };
        // The attachment is the plan's own allocation, so the plan's rows charge it: the control
        // plane builds its plan resource row from this report, and nothing else would count the
        // box or the two vectors the companions keep alive for the plan's lifetime.
        let arithmetic =
            || PrepareDiagnostics::fixed(PrepareRejection::Platform, "web.resource.arithmetic");
        let (mut retained, mut largest) = companions.retained_bytes().ok_or_else(arithmetic)?;
        if !boot {
            let (tables, tables_largest) = shape.retained_bytes().ok_or_else(arithmetic)?;
            retained = retained.checked_add(tables).ok_or_else(arithmetic)?;
            largest = largest.max(tables_largest);
        }
        charge_attachment(&mut host.report, caps, retained, largest)?;
        host.plan.set_host_attachment(Box::new(companions));
        if let Some(record) = &self.record {
            // The hand-off is checked here, before anything is committed: a preparation that
            // cannot leave the plan's tables for the host is refused, so a committed rebuild
            // always has its tables (`ReadyOwnership::run_command` relies on it).
            let mut record = record.lock().map_err(|_| {
                PrepareDiagnostics::fixed(PrepareRejection::Platform, SHAPE_UNAVAILABLE)
            })?;
            if boot {
                record.report = Some(host.report);
                record.superseded = Some(handles.live_values_superseded.clone());
            }
            record.shape = Some(shape);
        }
        Ok((host, handles, spectrum_captures))
    }
}

/// Build one plan's tables (issue #1290a): `model` is the plan's normalized session, `producers`
/// its effect producers in the control plane's list order, `observations` its companions'
/// observation handles, `meter_count` its companions' meters. The same function builds the boot
/// plan's and every replacement plan's, so the two cannot disagree on what an index means.
///
/// Issue #1207 D1: the handles' strips are the tracks, then the submixes. Since issue #1213 D1
/// every queue band, per-strip table and meter word covers both segments, tracks first, so a
/// track's entry is the same with or without submixes. #1381a: the strip order is the normalized
/// model's, the order host-core's addressing authority uses and the control plane's producers
/// follow.
///
/// Errors are the diagnostic code of the refusal.
fn build_plan_shape(
    model: &session::SessionModel,
    producers: &[host_core::EffectControlProducer],
    observations: &[EffectObservationHandle],
    meter_count: usize,
) -> Result<PlanShape, &'static str> {
    let track_count = model.tracks.len();
    let submix_count = model.submixes.len();
    let strip_count = track_count + submix_count;
    let mut tracks: Vec<Box<str>> = Vec::new();
    tracks
        .try_reserve_exact(track_count)
        .map_err(|_| "web.resource.allocation")?;
    tracks.extend(
        model
            .tracks
            .iter()
            .map(|track| Box::from(track.id.as_str())),
    );
    let mut submixes: Vec<Box<str>> = Vec::new();
    submixes
        .try_reserve_exact(submix_count)
        .map_err(|_| "web.resource.allocation")?;
    submixes.extend(
        model
            .submixes
            .iter()
            .map(|submix| Box::from(submix.id.as_str())),
    );
    let mut rack_effects = Vec::new();
    rack_effects
        .try_reserve_exact(strip_count)
        .map_err(|_| "web.resource.allocation")?;
    let count = |effects: usize| -> Result<u32, &'static str> {
        u32::try_from(effects).map_err(|_| "web.live_controls.effects")
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
        .map_err(|_| "web.resource.allocation")?;
    let mut total_effects = 0_u32;
    for counts in &rack_effects {
        effect_base.push(total_effects);
        for count in counts {
            total_effects = total_effects
                .checked_add(*count)
                .ok_or("web.live_controls.effects")?;
        }
    }
    // #1381a: the effect producers are the control plane's. The browser keeps, in the dense slot
    // order, where each instance's producer is, so an observe record (kinds 7 and 8) reaches its
    // effect's observation queue through the control plane's accessor.
    let mut effect_instances: Vec<Option<EffectInstance>> = Vec::new();
    effect_instances
        .try_reserve_exact(total_effects as usize)
        .map_err(|_| "web.resource.allocation")?;
    effect_instances.resize_with(total_effects as usize, || None);
    let mut producer_slots: Vec<u32> = Vec::new();
    producer_slots
        .try_reserve_exact(producers.len())
        .map_err(|_| "web.resource.allocation")?;
    producer_slots.resize(producers.len(), u32::MAX);
    for (position, producer) in producers.iter().enumerate() {
        let (track_id, address, effect_id, descriptor) = (
            &producer.track_id,
            &producer.address,
            &producer.effect_id,
            producer.descriptor,
        );
        let Some(strip) = strip_index(&tracks, &submixes, track_id.as_bytes()) else {
            return Err("web.live_controls.effects");
        };
        let Some(entry) = dense_effect_slot(effect_base[strip], rack_effects[strip], *address)
            .and_then(|slot| effect_instances.get_mut(slot))
        else {
            return Err("web.live_controls.effects");
        };
        if entry.is_some() {
            return Err("web.live_controls.effects");
        }
        // The slot the entry sits at: `dense_effect_slot` again, since `entry` borrows the table.
        let slot = dense_effect_slot(effect_base[strip], rack_effects[strip], *address)
            .and_then(|slot| u32::try_from(slot).ok())
            .ok_or("web.live_controls.effects")?;
        producer_slots[position] = slot;
        *entry = Some(EffectInstance {
            producer: u32::try_from(position).map_err(|_| "web.live_controls.effects")?,
            reader: None,
            track_id: track_id.clone(),
            effect_id: effect_id.clone(),
            address: *address,
            descriptor,
        });
    }
    // `observation_tracks[slot]` is the strip index of that effect slot's observed instance, or
    // `u32::MAX` for a slot with no taps. Built once, here, so the poll's fold is arithmetic.
    let mut observation_tracks = vec![u32::MAX; total_effects as usize];
    let meter_tables =
        MeterTables::new(meter_count, strip_count).map_err(|_| "web.resource.allocation")?;
    // The plan's published frame and the header that describes it (#1290d1). The frame's address
    // is the heap block's, so it stays put as the tables move.
    let meter_frame = zero_meter_frame(strip_count)?;
    let mut meter_header = empty_meter_header();
    meter_header.track_count =
        u32::try_from(track_count).map_err(|_| "web.live_controls.effects")?;
    meter_header.submix_count =
        u32::try_from(submix_count).map_err(|_| "web.live_controls.effects")?;
    meter_header.frame_pointer = address_word(meter_frame.as_ptr());
    for (reader, handle) in observations.iter().enumerate() {
        let Some(strip) = strip_index(&tracks, &submixes, handle.track_id.as_bytes()) else {
            return Err("web.live_controls.observation");
        };
        let Some(slot) = dense_effect_slot(effect_base[strip], rack_effects[strip], handle.address)
        else {
            return Err("web.live_controls.observation");
        };
        let Some(instance) = effect_instances
            .get_mut(slot)
            .and_then(Option::as_mut)
            .filter(|instance| instance.reader.is_none())
        else {
            return Err("web.live_controls.observation");
        };
        // The frame carries one gain-reduction slot per strip, so every observed effect of a strip
        // points at that strip and the poll folds them max-magnitude into the one slot; a
        // submix's effect points at its strip index `T + j` (issue #1209 D5).
        observation_tracks[slot] =
            u32::try_from(strip).map_err(|_| "web.live_controls.observation")?;
        instance.reader = Some(u32::try_from(reader).map_err(|_| "web.live_controls.observation")?);
    }
    // #1586 D3: every instance by stable key, observed or not (an unobserved one reads as
    // unbound), with its reader set's position.
    let mut index_entries = Vec::new();
    index_entries
        .try_reserve_exact(effect_instances.iter().flatten().count())
        .map_err(|_| "web.resource.allocation")?;
    index_entries.extend(effect_instances.iter().flatten().map(|instance| {
        host_core::observation::ObservationIndexEntry {
            strip_id: instance.track_id.clone(),
            section: instance.address.rack,
            effect_id: instance.effect_id.clone(),
            handle: instance.reader,
            effect: instance.descriptor,
        }
    }));
    let observation_index = host_core::observation::ObservationIndex::new(index_entries)
        .map_err(|_| "web.live_controls.effects")?;
    Ok(PlanShape {
        tracks,
        submixes,
        rack_effects: rack_effects.into_boxed_slice(),
        effect_base: effect_base.into_boxed_slice(),
        effect_instances: effect_instances.into_boxed_slice(),
        observation_tracks: observation_tracks.into_boxed_slice(),
        observation_index,
        producer_slots: producer_slots.into_boxed_slice(),
        // #1689: no plan is prepared with a master; `ReadyOwnership::sync_master` points the
        // shape at the strip the core's `master` row designates after every control command.
        master_track: None,
        command_wanted: zero_u32(total_effects as usize)?,
        observe_named: zero_u32(total_effects as usize)?,
        in_flight: zero_u32(total_effects as usize)?,
        meter_tables: Some(meter_tables),
        meter_frame,
        meter_header,
        #[cfg(any(test, feature = "test-support"))]
        successor_address: 0,
    })
}

impl PlanShape {
    /// Name `successor` as the header that replaces this plan's (null for none): the `u32` word
    /// the page reads, and beside it the full-width pointer a gate compares.
    #[inline]
    fn name_successor(&mut self, successor: *const WebMeterHeader) {
        self.meter_header.successor_pointer = address_word(successor);
        #[cfg(any(test, feature = "test-support"))]
        {
            self.successor_address = successor.addr();
        }
    }
}

/// `3(T + S) + 3` words: two peak lanes and one gain-reduction magnitude per strip, then the
/// master's (issue #1209 D1). With `S = 0` it is the `3T + 3` frame every reader already knows.
fn zero_meter_frame(strip_count: usize) -> Result<Box<[f32]>, &'static str> {
    let count = strip_count
        .checked_mul(3)
        .and_then(|value| value.checked_add(3))
        .ok_or("web.resource.arithmetic")?;
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| "web.resource.allocation")?;
    value.resize(count, 0.0);
    Ok(value.into_boxed_slice())
}

/// The linear-memory address a header names a structure by: the pointer's address as a `u32`, or
/// `0` where the address does not fit (a native test host; the shipped `wasm32` module's always
/// does). The same conversion the exports use.
fn address_word<T>(pointer: *const T) -> u32 {
    u32::try_from(pointer.addr()).unwrap_or(0)
}

/// `count` zero words, or the allocation refusal.
fn zero_u32(count: usize) -> Result<Box<[u32]>, &'static str> {
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| "web.resource.allocation")?;
    value.resize(count, 0);
    Ok(value.into_boxed_slice())
}

/// The bridge rows one plan's tables are charged (issue #1290a): the boot charges them to the
/// bridge, a replacement plan charges them to its attachment, so the swap budget counts them.
struct ShapeRows {
    /// The dense instance tables' own bytes, with the observation index (#1586 D3).
    instance_table_bytes: u64,
    /// The IDs the dense instance table copies out of the producers.
    instance_id_bytes: u64,
    /// The meter delivery tables (#1582 R1): per meter a pending snapshot, a remaining count and
    /// a carried-from handle, and the core meter frame's per-strip tables (#1582 D2).
    meter_tables_bytes: u64,
    /// The largest of those tables, a single allocation.
    meter_tables_largest: u64,
    /// The published meter frame (#1290d1).
    frame_bytes: u64,
}

impl PlanShape {
    /// The rows this shape is charged, or `None` on arithmetic overflow.
    fn bridge_rows(&self) -> Option<ShapeRows> {
        // The IDs the dense instance table copies out of the producers are retained beside it.
        let instance_id_bytes =
            self.effect_instances
                .iter()
                .flatten()
                .try_fold(0_u64, |total, instance| {
                    total.checked_add(
                        u64::try_from(instance.track_id.len() + instance.effect_id.len()).ok()?,
                    )
                })?;
        let instance_table_bytes = u64::try_from(self.effect_instances.len())
            .ok()?
            .checked_mul((size_of::<Option<EffectInstance>>() + size_of::<u32>()) as u64)?
            // #1677: the producer-to-instance table is retained beside it.
            .checked_add(
                u64::try_from(self.producer_slots.len())
                    .ok()?
                    .checked_mul(size_of::<u32>() as u64)?,
            )?
            // #1586 D3: the observation index, its table and the IDs it copies.
            .checked_add(self.observation_index.retained_bytes()?)?;
        // #1582 R1: the delivery tables, held here from the plan's preparation to its adoption
        // and from its retirement on; the boot plan's are charged before the delivery takes them.
        let (meter_tables_bytes, meter_tables_largest) =
            self.meter_tables.as_ref().map_or((0, 0), |tables| {
                (tables.heap_bytes(), tables.largest_allocation_bytes())
            });
        let meter_tables_bytes = u64::try_from(meter_tables_bytes).ok()?;
        let meter_tables_largest = u64::try_from(meter_tables_largest).ok()?;
        let frame_bytes = u64::try_from(self.meter_frame.len())
            .ok()?
            .checked_mul(size_of::<f32>() as u64)?;
        Some(ShapeRows {
            instance_table_bytes,
            instance_id_bytes,
            meter_tables_bytes,
            meter_tables_largest,
            frame_bytes,
        })
    }

    /// The heap a replacement plan's attachment holds for these tables, and the largest single
    /// allocation: the shell, then the rows. `None` on overflow.
    fn retained_bytes(&self) -> Option<(u64, u64)> {
        let rows = self.bridge_rows()?;
        let shell = u64::try_from(size_of::<Self>()).ok()?;
        let total = shell
            .checked_add(rows.instance_table_bytes)?
            .checked_add(rows.instance_id_bytes)?
            .checked_add(rows.meter_tables_bytes)?
            .checked_add(rows.frame_bytes)?;
        let largest = shell
            .max(rows.instance_table_bytes)
            .max(rows.meter_tables_largest)
            .max(rows.frame_bytes);
        Some((total, largest))
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
    spectrum_hop: Option<SpectrumHop>,
) -> Result<(ReadyOwnership, WebPlan, WebResourceReport), BootFailure> {
    // The command staging, and with it the observation records, exist exactly when the boot asked
    // for observation taps (#1565).
    let commands_enabled = options.live_control_observation_taps != 0;
    // #1312 D2, #1399: the session's `live_values_superseded` count is the control plane's, made
    // once before any plan is prepared; the preparer leaves a handle on it in the record, which the
    // status reads.
    let record = Arc::new(Mutex::new(BootRecord::default()));
    let preparer = WebRuntimePreparer::new(options).recording(Arc::clone(&record));
    let limits = control_limits(caps, memory_budget);
    // #1593 D2: the boot request and hop are the session's spectrum configuration, which the
    // session gives every plan it prepares, keeping each entry whose target an edit leaves.
    let spectrum = spectrum_request.map(|preparation| host_core::SpectrumConfiguration {
        preparation: preparation.clone(),
        hop: spectrum_hop,
    });
    let control_plane::CompiledChildren {
        session: mut control,
        mut plan,
    } = control_plane::compile_children::<WebAdapter>(
        document,
        limits,
        Box::new(preparer),
        spectrum,
    )
    .map_err(|failure| boot_refusal(&failure, &record))?;
    let (engine, live_values_superseded, mut shape) = {
        let mut record = record
            .lock()
            .map_err(|_| fixed_diagnostic("web.internal.boot_record"))?;
        match (
            record.report.take(),
            record.superseded.take(),
            record.shape.take(),
        ) {
            (Some(engine), Some(superseded), Some(shape)) => (engine, superseded, shape),
            _ => return Err(fixed_diagnostic("web.internal.boot_record").into()),
        }
    };
    let resources = plan.queries().resources();
    let session = control.committed_session();
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
    // #1290a: the tables were built by the boot plan's own preparation (`build_plan_shape`), the
    // same function every replacement plan's preparation runs. The boot plan's spectrum captures
    // went to the session's spectrum monitor (#1596 D3).
    plan.active_attachment_mut::<RenderCompanions>()
        .ok_or_else(|| fixed_diagnostic("web.internal.companions"))?;
    let rows = shape
        .bridge_rows()
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.bridge_metadata_bytes = report
        .bridge_metadata_bytes
        .checked_add(rows.instance_table_bytes)
        .and_then(|bytes| bytes.checked_add(rows.instance_id_bytes))
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.bridge_retained_bytes = report
        .bridge_retained_bytes
        .checked_add(rows.instance_table_bytes)
        .and_then(|bytes| bytes.checked_add(rows.instance_id_bytes))
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.largest_bridge_allocation_bytes = report
        .largest_bridge_allocation_bytes
        .max(rows.instance_table_bytes);
    report.largest_named_allocation_bytes = report
        .largest_named_allocation_bytes
        .max(report.largest_bridge_allocation_bytes);
    // The master windows are the control plane's output meter (#1581 D5): its queue and words are
    // in the control plane's retained row, charged above with it.
    let meter_delivery_bytes = rows
        .meter_tables_bytes
        .checked_add(rows.frame_bytes)
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
        .max(rows.meter_tables_largest)
        .max(rows.frame_bytes);
    report.largest_named_allocation_bytes = report
        .largest_named_allocation_bytes
        .max(report.largest_bridge_allocation_bytes);
    // The decoded observation staging is a separate retained allocation from the public wire
    // staging row; charge the full actual array before the final aggregate budget check.
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
    // #1290a D3: a replacement is admitted against the one budget. What it leaves for the plans is
    // the budget less every byte the bridge retains, the committed model aside: the replacement's
    // own check counts both compiled models.
    control.set_replacement_peak_limit(
        memory_budget
            .saturating_sub(report.bridge_retained_bytes)
            .saturating_add(engine.session_model_bytes),
    );
    // #1581 D5: the session's output meter is the control plane's; its reader and gate are the
    // host's, handed out once, here.
    let (output_reader, output_gate) = control
        .take_output_meter()
        .ok_or_else(|| fixed_diagnostic("web.internal.output_meter"))?;
    let structure_epoch = control.structure_epoch();
    // #1582 D1: the session's one meter delivery takes the boot plan's tables (charged above with
    // the plan's rows) and the output meter's halves; the boot plan's first block is the grid's
    // origin.
    let meter_tables = shape
        .meter_tables
        .take()
        .ok_or_else(|| fixed_diagnostic("web.internal.meter_tables"))?;
    let meter_window_frames = options
        .live_control_meter_blocks
        .checked_mul(u64::from(session_quantum_frames))
        .unwrap_or(0);
    let ready = ReadyOwnership {
        control,
        queries,
        #[cfg(feature = "test-support")]
        limits,
        commands_enabled,
        live_values_superseded,
        shape: {
            let mut shape = Box::new(shape);
            shape.meter_header.structure_epoch = structure_epoch;
            shape
        },
        shape_epoch: 0,
        record,
        pending_shapes: {
            let mut pending = Vec::new();
            pending
                .try_reserve_exact(PENDING_SHAPES)
                .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
            pending
        },
        blocking_rebuilds: 0,
        command_decoded: boxed_observe_staging(decoded_count)?,
        has_in_flight_commands: false,
        meters: MeterDelivery::new(
            output_reader,
            output_gate,
            meter_window_frames,
            meter_tables,
        ),
        #[cfg(test)]
        meter_effect_scan_count: 0,
    };
    Ok((ready, plan, report))
}

/// Translate the browser configuration's two live-control words into the facade's live-control
/// request.
///
/// The meter word passes unchanged (issue #1581 D2): `live_control_meter_blocks == 0` binds the
/// strip meters at host-core's default window, and the lease over such a host is still refused
/// (`set_meter_lease`). The port lease is a second, finer switch over posting.
fn live_control_request(
    options: WebBootOptions,
    quantum_frames: u32,
) -> Option<HostLiveControlRequest> {
    let meter_window_blocks = u32::try_from(options.live_control_meter_blocks).ok()?;
    // A window longer than `u32::MAX` frames is refused here, as host-core would.
    meter_window_blocks.checked_mul(quantum_frames)?;
    Some(HostLiveControlRequest {
        meter_window_blocks,
        meter_queue_depth: NonZeroUsize::new(METER_QUEUE_DEPTH)?,
        // Issue #143 D3: a carved browser configuration word, translated once, here.
        observation_taps: u32::try_from(options.live_control_observation_taps).ok()?,
    })
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
#[cfg(feature = "render-lock-probe")]
pub use render_lock::miso_engine_web_v1_render_lock_probe;

#[cfg(test)]
mod tests;
