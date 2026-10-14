//! Safe browser-Wasm preparation and ownership shell.
//!
//! This module deliberately contains no raw pointer handling or JavaScript integration. It owns
//! the complete immutable session and render plan that the AudioWorklet boundary drives.
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
use core::sync::atomic::{AtomicU64, Ordering};

use builtins::{
    BuiltinLaneSelector, Matrix2x2, MeterMetricSet, MeterSnapshot, MeterTap,
    PreparedInputFilterTarget, pan_matrix, validate_prepared_input_filter_target,
};
use builtins_compiler::{
    MeterConsumer, TrackControlProducer, TrackControlRecord, TrackFaderRecord, TrackInputRecord,
};
use controls::{ControlDescriptor, ControlError, Lane, OwnerKind, SectionKind};
use effect_contract::{
    EffectControlRecord, ObservationCost, ObservationDescriptor, ParameterChannel,
    ParameterChannelPolicy, parameter_value_valid,
};
use engine::realtime::{HostAttachment, ObservationWindow, PlanarBufferMut, RenderIo, RenderTime};
use host_core::{
    CompiledSession, EffectControlProducer, EffectObservationHandle, HostLiveControlRequest,
    HostLiveLanes, HostMeterRequest, HostPrepareCaps, HostShapePolicy, InputFilterEdit,
    InputFilterEditErrorKind, LiveControlSoloState, LiveRampRow, LiveRamps, LiveRouteMuteFollow,
    LiveRouteState, LiveStripValues, LiveVcaState, PrepareDiagnostics, PrepareRejection,
    PreparedHost, RouteControlError, RouteControlProducer, RouteControlRecord, SourceControlError,
    SourceSubmission, StripMuteSeed, SuccessorBase, apply_input_filter_edit, changed_lane_records,
    compile_host_model, compiled_session_shape, control_table_bytes, covers_lane, fader_gain_bits,
    parse_host_session, prepare_host_runtime_with_live_controls_and_spectrum,
    source_id_arena_bytes, trim_gain_bits,
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

use control_targets::{PREPARED_EFFECT_COMPANION_CAPACITY, PreparedControlWorkspace};

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
/// below 13.1). 20 leaves 14.3% headroom. Boot checks
/// `document_bytes * PARSE_TRANSIENT_MULTIPLIER` against the effective budget before UTF-8 decode
/// or parser allocation, and the peak-transient test keeps every phase visible so frontend growth
/// cannot silently outrun this projection.
pub const PARSE_TRANSIENT_MULTIPLIER: u64 = 20;

/// The most VCAs a browser session may declare (issue #1245 amendment A1).
///
/// A planner decision, subject to owner review. A VCA command runs on the AudioWorklet thread, and
/// its cost grows with the (strip, reaching VCA) pairs it recomposes, so the browser bounds both
/// the VCA count and the pair count ([`MAXIMUM_BROWSER_VCA_REACH_PAIRS`]) at boot, before any VCA
/// table is built. 256 is far more VCAs than a mix uses, and it is the width of the boot-time reach
/// count's ancestor bitsets.
pub const MAXIMUM_BROWSER_VCAS: usize = 256;

/// The most (strip, reaching VCA) pairs a browser session may declare (issue #1245 amendment A1).
///
/// A planner decision, subject to owner review. A batch's VCA work is bounded by about three
/// passes over the pairs (the VCA fader pass over both lanes, and one VCA-mute refresh), plus 256
/// member moves of at most [`MAXIMUM_BROWSER_VCAS`] reaching VCAs each; at this bound that is a
/// small fraction of one 128-frame quantum (the measurement is in the #1245 record). 16,384 pairs
/// hold, for example, 1,024 strips in 16 VCAs each. A session past either bound is refused at boot
/// with `web.vca.maximum_vcas` or `web.vca.reach_pairs` (`RESULT_REFUSED_BUDGET`).
pub const MAXIMUM_BROWSER_VCA_REACH_PAIRS: u64 = 16_384;

/// Words of one boot-time VCA ancestor bitset.
const VCA_REACH_WORDS: usize = MAXIMUM_BROWSER_VCAS / 64;

/// A conservative bound on the transient bytes `session::SessionModel::vca_reach` holds per
/// membership entry: its `BTreeMap<&str, Vec<usize>>` parent index at minimum node occupancy
/// (about 90 bytes per key and value with the node's edge share) plus the value `Vec`'s doubled
/// capacity, rounded up (issue #1245 amendment A1).
const VCA_PARENT_ENTRY_BYTES: u64 = 128;

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
/// The `smoothing_samples` word that means "no length given" (issue #1364 D1; decision 15 D15-1).
///
/// A ramped record whose word is this value is admitted with the session's own length for its
/// row: the `control_smoothing` keys, or the researched default table, rounded at the session's
/// sample rate (`host_core::LiveRamps`). Every other value, `0` (a step) included, is the length
/// itself. The sentinel is not a length: it never reaches the render plane, and a kind that
/// carries no ramp (input filters, bypass) refuses it as `MALFORMED`, as it refuses any non-zero
/// word.
pub const SESSION_DEFAULT_RAMP: u32 = u32::MAX;
/// Largest number of records one `miso.command.v1` submission may stage.
///
/// The staging buffer is allocated at preparation for exactly this many records, so a submission
/// can never grow the module's memory and the render path never sees an allocation.
pub const MAXIMUM_COMMAND_RECORDS: u32 = 256;
/// Per-track control-queue depth used when the configuration asks for the default.
pub const DEFAULT_COMMAND_QUEUE_RECORDS: u32 = 64;
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

/// Retarget the track's pan pair (`left`, `right`) over an explicit ramp window.
pub const COMMAND_PAN: u32 = 1;
/// Retarget the track's full 2x2 matrix over an explicit ramp window.
pub const COMMAND_MATRIX: u32 = 2;
/// Retarget a lane fader in decibels over an explicit ramp window (live since issue #140 B).
pub const COMMAND_FADER_DB: u32 = 3;
/// Set a lane mute, as a fader endpoint over the same ramp window (live since issue #140 B).
///
/// `values[0]` is exactly `0.0` or `1.0`: the core's boolean rule (`controls::check`), which
/// refuses `-0.0` as the session grammar and the C ABI do, for this kind and for polarity, send
/// mute, VCA mute and solo alike (decision 17, slice S7c D3).
pub const COMMAND_MUTE: u32 = 4;
/// Set an effect parameter (live since issue #140 A).
pub const COMMAND_EFFECT_PARAM: u32 = 5;
/// Set an effect bypass (live since issue #140 A).
///
/// A live bypass engages or lifts the latency-preserving shunt of the addressed console slot or
/// insert, and it can lift a session `bypass` too (issue #1087), with two exceptions. The delay
/// and the multiband compressor keep their session bypass as a *prepared* bypass
/// (`effect_compiler::lowers_session_bypass`; issues #1087 and #1100): their live-control lane is
/// seeded bypassed, and lifting it is admitted but renders nothing different, because the
/// processor itself was prepared bypassed. Neither can be a console slot, so both are inserts.
pub const COMMAND_EFFECT_BYPASS: u32 = 6;
/// Arm one declared observation tap of one effect instance (issue #143 D3).
///
/// `parameter_id` carries the effect-local `tap_id`, `smoothing_samples` carries the window length
/// in render blocks (`0` = the plan's default), and every value word must be `0.0`: a subscription
/// changes what is read, never what is rendered, so a nonzero value would be a caller mistake
/// rather than a meaningful field.
pub const COMMAND_OBSERVE_SUBSCRIBE: u32 = 7;
/// Disarm one declared observation tap of one effect instance (issue #143 D3).
pub const COMMAND_OBSERVE_UNSUBSCRIBE: u32 = 8;
/// Engage or clear one track's solo-in-place bit (issue #210 phase 1). Only a track index is
/// soloable: a submix's strip index refuses with [`COMMAND_REASON_NOT_SOLOABLE`] (issue #1213 D4).
///
/// The shape mirrors [`COMMAND_MUTE`] because solo *is* a mute composition: `rack = 255`,
/// `channel = 255` (solo is a strip gesture, not a lane one), `values[0]` exactly `0.0` or `1.0`,
/// and `smoothing_samples` the engage/disengage fade -- the same declick window a mute takes.
///
/// It moves no state of its own on the render thread. Admission composes
/// `effective_mute = user_mute || vca_mute || (any_solo && !solo_safe && !my_solo)` over the live
/// controls' [`host_core::LiveControlSoloState`] and emits the *existing*
/// `TrackFaderRecord::Mute` records into the *existing* per-strip fader queues, so this kind is
/// on the `render` plane (it moves what the render thread reads) while adding nothing below
/// `admit_commands`. Refusals reuse the existing vocabulary: `malformed` for a wrong-shaped
/// record, `domain` for a `values[0]` outside `{0.0, 1.0}` (exactly as `mute` does),
/// `unknownTrack`, `backpressure` and `wrongState`, plus `notSoloable` for a submix.
pub const COMMAND_SOLO: u32 = 9;
/// Retarget a lane's input trim in decibels over an explicit ramp window (#210 phase 3).
///
/// `channel` is `0` left, `1` right or `2` both; `values[0]` is the new `trim_db` and must lie in
/// `trim_db`'s own declared domain, `[-144, 24]`; `values[1..]` must be `0.0`; `rack` is `255`.
/// `smoothing_samples` is the ramp window, in sample updates, exactly as it is for `faderDb`.
///
/// A `channel = 2` command is **one** record carrying `BuiltinLaneSelector::Both`, not two
/// per-lane records: the input stage is upstream of the fader/matrix seam, and a both-channel
/// retarget must be admitted as one symmetry-preserving event or it would retire the track's mono
/// collapse. `builtins_compiler::TrackInputRecord` carries the argument in full.
///
/// The lane's polarity is preserved: a trim ride does not clear a flip.
pub const COMMAND_TRIM_DB: u32 = 10;
/// Set or clear a lane's input polarity inversion (#210 phase 3).
///
/// `channel` is `0` left, `1` right or `2` both; `values[0]` must be exactly `0.0` or `1.0`;
/// `values[1..]` must be `0.0`; `rack` is `255`. `smoothing_samples` is the ramp window.
///
/// A flip is a retarget of the **same** coefficient the trim rides to its own negation, so it
/// declicks through the trim ramp -- the linear ramp carries the coefficient through zero -- and
/// costs no DSP of its own. The lane's trim magnitude is preserved.
pub const COMMAND_POLARITY_INVERT: u32 = 11;
/// Retarget a builtin input HPF/LPF pair through prepared filter targets.
pub const COMMAND_INPUT_FILTERS: u32 = 12;
/// Retarget one live send's gain in decibels over an explicit ramp window (issue #1222 D1).
///
/// The index word is a **live-route index**: the send's position among the routes into submixes,
/// in canonical route-ID order -- not a strip index. An index past the last live route refuses
/// with [`COMMAND_REASON_UNKNOWN_ROUTE`]. `values[0]` is the gain; `values[1..]` must be `0.0`,
/// `rack` and `channel` must be `255`, and `effect_index` and `parameter_id` must be `0`, or the
/// record is `MALFORMED`. `smoothing_samples` is the ramp length.
///
/// Admission keeps a mirror of every live send's gain, matrix and mute
/// ([`host_core::LiveRouteState`]): a send record carries all of them folded into four
/// coefficients, so the record is built from the mirror with this one field replaced, through
/// [`host_core::RouteControlProducer::record`] -- the prepared route's own coefficient function.
/// A value that function refuses, or a ramp past its bound, is `DOMAIN`.
pub const COMMAND_ROUTE_GAIN_DB: u32 = 13;
/// Switch one live send on or off over an explicit ramp window (issue #1222 D1).
///
/// Addressed and shaped as [`COMMAND_ROUTE_GAIN_DB`]; `values[0]` must be exactly `0.0` (on) or
/// `1.0` (muted), anything else is `DOMAIN`, exactly as `mute` requires.
pub const COMMAND_ROUTE_MUTE: u32 = 14;
/// Retarget one live send's 2x2 matrix over an explicit ramp window (issue #1222 D1).
///
/// Addressed and shaped as [`COMMAND_ROUTE_GAIN_DB`]; `values[0..4]` are `ll, lr, rl, rr`.
pub const COMMAND_ROUTE_MATRIX: u32 = 15;
/// Ride one VCA's per-lane dB offset over an explicit ramp window (issue #1245 D1).
///
/// The index word is a **VCA index**: the VCA's position in the session's `vcas`, in canonical
/// VCA-ID order -- not a strip index. An index at or past the VCA count (every index, in a
/// session without VCAs) refuses with [`COMMAND_REASON_UNKNOWN_VCA`]. The shape is
/// [`COMMAND_FADER_DB`]'s -- `rack` `255`, `channel` `0`/`1`/`2`, `values[0]` the offset in
/// `[-144, 24]` dB (else `DOMAIN`), `values[1..]` `0.0`, `smoothing_samples` the ramp -- plus
/// `effect_index` and `parameter_id` `0`, as the route kinds rule; anything else is `MALFORMED`.
///
/// A VCA has no audio path. Admission composes the move through [`host_core::LiveVcaState`] --
/// `clamp(own + sum of the reaching VCAs' offsets)`, the one composition preparation bakes -- and
/// stages each reached member's changed fader lanes as the *existing* `TrackFaderRecord::FaderDb`
/// records on the member's fader queue, with this record's ramp. A member lane whose effective
/// value does not change gets no record.
pub const COMMAND_VCA_FADER_DB: u32 = 16;
/// Set or clear one VCA's per-lane mute over an explicit ramp window (issue #1245 D1).
///
/// Addressed and shaped as [`COMMAND_VCA_FADER_DB`]; `values[0]` must be exactly `0.0` or `1.0`,
/// anything else is `DOMAIN`, exactly as `mute` requires. The VCA mute term joins every reached
/// member's effective mute (`user_mute || vca_mute || solo term`, so mute wins over solo and a
/// solo-safe submix is muted too), and the *existing* strip mute records and, for `follows_mute`
/// sends, follow records carry the change.
pub const COMMAND_VCA_MUTE: u32 = 17;

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
/// `parameter_id` is not a parameter of the addressed effect.
pub const COMMAND_REASON_UNKNOWN_PARAMETER: u32 = 5;
/// A value is outside the addressed parameter's declared domain.
pub const COMMAND_REASON_DOMAIN: u32 = 6;
/// The record is well formed and correctly addressed, but this session cannot apply its kind.
///
/// This is not "malformed" and it is not "unknown target": the parameter exists and the value is
/// legal, and this session has no post-preparation write path for it. The parameter-metadata JSON
/// (deliverable 4) marks every such parameter, so a caller never has to discover this at runtime.
///
/// Issue #140 emptied this of everything the ABI *declares*: pan, matrix, fader, mute, effect
/// parameter and effect bypass are all live, and `liveUpdatable` is `true` for each. It remains
/// reachable for the states that are genuinely not addressable -- a host with no live controls
/// attached at all, and an effect whose parameter declares `AutomationRate::None`, which no launch
/// effect does but which a future one may.
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
/// The tap exists and the address is right, but this session bound no observation capacity.
///
/// The honest form of "you asked for a subscription this preparation cannot deliver": the effect
/// is there, the tap is declared, and the plan holds no lane to arm because the host asked for
/// none. A caller fixes it by preparing with `live_control_observation_taps` set, not by retrying.
pub const COMMAND_REASON_OBSERVATION_UNBOUND: u32 = 11;
/// The addressed strip is solo-safe: a submix is never soloed (issue #1212).
///
/// Its own reason, not `MALFORMED` or `UNKNOWN_TRACK`: the record is well formed and the strip
/// exists, and a caller that solos a bus learns that it addressed the wrong kind of strip.
pub const COMMAND_REASON_NOT_SOLOABLE: u32 = 12;
/// A send kind's index word is not a live route of this plan (issue #1222 D2).
///
/// Its own reason, not `UNKNOWN_TRACK`: a send is addressed by its live-route index, not a strip
/// index, and a caller that names a send that does not exist learns which address was wrong.
pub const COMMAND_REASON_UNKNOWN_ROUTE: u32 = 13;
/// A VCA kind's index word is not a VCA of this session (issue #1245 D2).
///
/// Its own reason, not `UNKNOWN_TRACK` or `UNKNOWN_ROUTE`: a VCA is addressed by its VCA index, in
/// canonical VCA-ID order, and a caller that names a VCA that does not exist learns which address
/// was wrong.
pub const COMMAND_REASON_UNKNOWN_VCA: u32 = 14;

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
    /// Whether the plan accepts live commands: nonzero allocates the command staging and accepts
    /// them, `0` accepts none and allocates no command staging (issue #137 D1). The number sizes
    /// nothing in the plan: every fader, mute, matrix, input, send and effect value is a
    /// latest-target cell (issues #1312, #1345, #1346, #1347), and an effect's observation queue
    /// holds one subscription per declared tap (#1345 D2). Only the JS port's in-flight bound
    /// reads it as a count. The word keeps its frozen name and offset in the boot options.
    ///
    /// Zero still leaves every strip's cells on the plan: the engine ramps a strip's mute for its
    /// own transitions, and no command can reach them (issues #1326, #1312).
    pub live_control_command_queue_records: u64,
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
    /// Requires `live_control_command_queue_records != 0`: a subscription rides the effect's own
    /// observation queue, so observation without live controls has no delivery path.
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
            live_control_command_queue_records: 0,
            live_control_meter_blocks: 0,
            live_control_observation_taps: 0,
            live_control_master_track_plus_one: 0,
        }
    }

    /// Explicit defaults with the live web controls attached.
    #[must_use]
    pub const fn live_control_defaults() -> Self {
        Self {
            live_control_command_queue_records: DEFAULT_COMMAND_QUEUE_RECORDS as u64,
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
    /// Fixed `MAXIMUM_COMMAND_RECORDS * COMMAND_RECORD_BYTES` live-control staging (issue #137 D1).
    command: Box<[u8]>,
    plane_references: Box<[MaybeUninit<&'static [f32]>]>,
    /// One fixed workspace for the trusted prepared-control companion and addressed EQ config.
    prepared_control: Option<Box<PreparedControlWorkspace>>,
}

type FfiSourceStaging<'a> = (
    &'a mut [f32],
    &'a mut [MaybeUninit<&'static [f32]>],
    &'a mut [u8],
);

/// Spectrum observer preparation requested by the browser staging boundary: the core's
/// [`host_core::SpectrumPreparation`], under its earlier name.
pub use host_core::SpectrumPreparation as SpectrumPreparationRequest;

/// The ramp lengths an absent per-edit length resolves to (issue #1364 D2).
///
/// Computed once, on the control thread, from the session this host prepared: the
/// `control_smoothing` table ([`LiveRamps::for_session`]) and, for each strip, the window a pan or
/// matrix record takes (#1054 D5). Admission reads it to replace [`SESSION_DEFAULT_RAMP`]; the
/// render path never sees it.
struct SessionRamps {
    live: LiveRamps,
    /// One entry per strip, the tracks then the submixes: the strip's own `smoothing_samples` when
    /// the prepared session gives its pan or matrix a non-zero window, else `live.pan_samples`.
    strip_pan: Box<[u32]>,
}

impl SessionRamps {
    /// `None` when the table cannot be reserved.
    fn try_new(model: &session::SessionModel) -> Option<Self> {
        let live = LiveRamps::for_session(model);
        let mut strip_pan = Vec::new();
        strip_pan
            .try_reserve_exact(model.tracks.len() + model.submixes.len())
            .ok()?;
        for matrix_or_pan in model
            .tracks
            .iter()
            .map(|track| &track.matrix_or_pan)
            .chain(model.submixes.iter().map(|submix| &submix.matrix_or_pan))
        {
            let window = match matrix_or_pan {
                session::MatrixOrPan::Pan {
                    smoothing_samples, ..
                }
                | session::MatrixOrPan::Matrix {
                    smoothing_samples, ..
                } => *smoothing_samples,
            };
            strip_pan.push(if window != 0 {
                window
            } else {
                live.pan_samples
            });
        }
        Some(Self {
            live,
            strip_pan: strip_pan.into_boxed_slice(),
        })
    }

    /// Bytes this table retains beyond the struct itself.
    fn retained_bytes(&self) -> u64 {
        (self.strip_pan.len() * size_of::<u32>()) as u64
    }

    /// The length a record of `row` carries for `word`: the session's own for the sentinel, else
    /// the word. The core's resolver does it (decision 17, slice S7c D4).
    const fn resolve(&self, row: LiveRampRow, word: u32) -> u32 {
        self.live.resolve(
            row,
            if word == SESSION_DEFAULT_RAMP {
                None
            } else {
                Some(word)
            },
        )
    }

    /// [`Self::resolve`] for a strip's pan or matrix record (#1054 D5): the sentinel becomes the
    /// strip's own window when its prepared pan or matrix has one, else the `pan_ms` length. A
    /// strip index past the table is the `pan_ms` length; admission refuses such a record next.
    fn resolve_strip_pan(&self, strip: usize, word: u32) -> u32 {
        // The strip's own window comes first; `strip_pan` already holds the `pan_ms` length for a
        // strip whose pan or matrix has none.
        let edit = if word == SESSION_DEFAULT_RAMP {
            self.strip_pan.get(strip).copied()
        } else {
            Some(word)
        };
        self.live.resolve(LiveRampRow::Pan, edit)
    }
}

/// The core's descriptors of the rows the browser's live kinds write (decision 17, slice S7c D1).
///
/// Resolved once at boot, where the host already allocates, from `controls::row_descriptor`; every
/// value check of an admitted record is `controls::check` against one of these, so the browser
/// states no range or boolean rule of its own. `ReadyOwnership` holds the table; #S10 replaces it
/// with the core's own list.
#[derive(Clone, Copy)]
struct WebControlRows {
    /// A strip's builtin `fader_db`, `mute`, `trim_db`, `polarity_invert` and `pan`.
    fader_db: ControlDescriptor,
    mute: ControlDescriptor,
    trim_db: ControlDescriptor,
    polarity_invert: ControlDescriptor,
    pan: ControlDescriptor,
    /// A strip's builtin `matrix_ll`, `matrix_lr`, `matrix_rl`, `matrix_rr`.
    matrix: [ControlDescriptor; 4],
    /// A strip's own `solo`.
    solo: ControlDescriptor,
    /// A route's own `gain_db` and `mute`, on a route into a submix.
    route_gain_db: ControlDescriptor,
    route_mute: ControlDescriptor,
    /// A route's own `matrix_ll`, `matrix_lr`, `matrix_rl`, `matrix_rr`.
    route_matrix: [ControlDescriptor; 4],
    /// A VCA's own `offset_db` and `mute`.
    vca_offset_db: ControlDescriptor,
    vca_mute: ControlDescriptor,
}

impl WebControlRows {
    /// The descriptors at `sample_rate`; `None` if a table row is not the row named here (a drift
    /// the boot refuses rather than admits).
    fn new(sample_rate: u32) -> Option<Self> {
        let row = |owner, section, control: u32, name: &str| {
            controls::row_descriptor(owner, section, control, sample_rate)
                .filter(|descriptor| descriptor.name == name)
        };
        let builtin = |control, name| row(OwnerKind::Strip, SectionKind::Builtins, control, name);
        let route = |control, name| row(OwnerKind::Route, SectionKind::Own, control, name);
        Some(Self {
            polarity_invert: builtin(1, "polarity_invert")?,
            trim_db: builtin(2, "trim_db")?,
            fader_db: builtin(5, "fader_db")?,
            mute: builtin(6, "mute")?,
            matrix: [
                builtin(7, "matrix_ll")?,
                builtin(8, "matrix_lr")?,
                builtin(9, "matrix_rl")?,
                builtin(10, "matrix_rr")?,
            ],
            pan: builtin(12, "pan")?,
            solo: row(OwnerKind::Strip, SectionKind::Own, 2, "solo")?,
            route_gain_db: route(1, "gain_db")?,
            route_mute: route(2, "mute")?,
            route_matrix: [
                route(3, "matrix_ll")?,
                route(4, "matrix_lr")?,
                route(5, "matrix_rl")?,
                route(6, "matrix_rr")?,
            ],
            vca_offset_db: row(OwnerKind::Vca, SectionKind::Own, 1, "offset_db")?,
            vca_mute: row(OwnerKind::Vca, SectionKind::Own, 2, "mute")?,
        })
    }
}

/// The browser's reason for a refusal of the core's validator: `control.domain` is `DOMAIN`, and
/// `control.lane` and `control.ramp.fixed` are the `MALFORMED` a wrong channel or a stray word
/// always was (decision 17, slice S7c D2).
const fn control_reason(error: ControlError) -> u32 {
    match error {
        ControlError::Domain => COMMAND_REASON_DOMAIN,
        _ => COMMAND_REASON_MALFORMED,
    }
}

/// `controls::check` of one wire value against one row of the table, on `lane`, with no explicit
/// ramp (the ramp word is resolved and bounded apart from the value).
fn check_value(descriptor: &ControlDescriptor, lane: Lane, value: f32) -> Result<(), u32> {
    controls::check(descriptor, lane, value, None).map_err(control_reason)
}

/// The core's lane for a builtin lane selector.
const fn control_lane(lanes: BuiltinLaneSelector) -> Lane {
    match lanes {
        BuiltinLaneSelector::Left => Lane::Left,
        BuiltinLaneSelector::Right => Lane::Right,
        BuiltinLaneSelector::Both => Lane::Both,
    }
}

/// The live mirrors as the core's effective-value inputs (decision 17, slice S7c D5): the
/// browser's one temporary view, deleted by #S10 with the mirrors.
///
/// Reads only; every read is a checked access and nothing allocates, because `command_submit`
/// runs render-locked. Strips are in the strip order the mirrors use.
///
/// A live send reports its follow term alone (`mute: false`): the route mirror keeps a send's own
/// mute apart from the source lanes it follows, because a record carries both, so
/// `controls::effective_gate` over this view is exactly the follow-zeroed lanes (the record's
/// silenced flag adds the send's own mute).
#[derive(Clone, Copy)]
struct WebEffectiveView<'a> {
    vcas: &'a LiveVcaState,
    solo: &'a LiveControlSoloState,
    routes: &'a LiveRouteState,
}

impl<'a> WebEffectiveView<'a> {
    const fn new(
        vcas: &'a LiveVcaState,
        solo: &'a LiveControlSoloState,
        routes: &'a LiveRouteState,
    ) -> Self {
        Self { vcas, solo, routes }
    }

    /// Whether some soloable strip is soloed, computed once per admitted batch: `cache` holds the
    /// answer until a solo record moves a solo bit (it is then cleared). Allocation-free.
    fn any_solo(&self, cache: &mut Option<bool>) -> bool {
        *cache.get_or_insert_with(|| controls::any_solo(self))
    }

    /// One strip's effective values.
    fn strip(&self, strip: usize, any_solo: bool) -> controls::EffectiveStrip {
        controls::effective_strip(self, strip, any_solo)
    }

    /// One live send's follow gate, per source lane.
    fn follow_gate(&self, route: usize, any_solo: bool) -> [bool; 2] {
        let Some(entry) = self.routes.get(route) else {
            return [false; 2];
        };
        controls::effective_gate(self, route, &self.strip(entry.source_strip, any_solo))
    }
}

impl controls::EffectiveInputs for WebEffectiveView<'_> {
    fn strip_count(&self) -> usize {
        self.solo.strip_count()
    }

    fn own_fader(&self, strip: usize) -> ([f32; 2], [bool; 2]) {
        (
            self.vcas.own_db(strip).unwrap_or([0.0; 2]),
            [0, 1].map(|lane| self.solo.user_mute(strip, lane)),
        )
    }

    fn soloed(&self, strip: usize) -> bool {
        self.solo.solo(strip)
    }

    fn solo_safe(&self, strip: usize) -> bool {
        self.solo.solo_safe(strip)
    }

    fn reach(&self, strip: usize) -> &[usize] {
        self.vcas.reach_of(strip)
    }

    fn vca(&self, vca: usize) -> ([f32; 2], [bool; 2]) {
        self.vcas.vca_values(vca).unwrap_or(([0.0; 2], [false; 2]))
    }

    fn route_count(&self) -> usize {
        self.routes.len()
    }

    fn route(&self, route: usize) -> controls::EffectiveRouteInput {
        let entry = self.routes.get(route);
        controls::EffectiveRouteInput {
            mute: false,
            follows_mute: entry.is_some_and(|entry| entry.follows_mute),
            source_strip: entry.map_or(usize::MAX, |entry| entry.source_strip),
        }
    }
}

/// Everything one compiled session owns on the browser side.
///
/// Field order is the drop order and is load-bearing: [`PreparedHost`] drops its plan (which owns
/// the source consumers) before its control-side producers, and the compiled session model outlives
/// both. Nothing here is ever dropped from `render_next`; see [`AudioWorkletEngineHost::fail`].
struct ReadyOwnership {
    /// Whether the app opted in to live commands (`live_control_command_queue_records != 0`,
    /// issue #1326 D4). The command gate reads this, never the producers: every plan carries each
    /// strip's fader/mute and matrix/pan lanes for the engine's own transitions, and only an app
    /// that opted in may drive them.
    commands_enabled: bool,
    /// Issue #137 D1 / #140 B: per-strip control-side producers -- matrix/pan and fader/mute --
    /// declared first so they are released before the plan that owns their consumer endpoints.
    /// Parallel to the strips (issue #1211 D3); a command's index word is a strip index (#1213).
    controls: Vec<TrackControlProducer>,
    /// The session's `live_values_superseded` count (#1312 D2): one count, made once by
    /// [`compile_ready`], which every cell reader of every plan the session prepares adds to, and
    /// which the status reports (#1399). A successor plan the session prepares must be given this
    /// same count.
    live_values_superseded: host_core::LiveValuesSuperseded,
    /// The plan's gate handle (#1502 D1, #1312 D7). The worklet writes the strips' cells, so it
    /// stamps those writes from this handle and publishes their revision on it, as the control
    /// plane does through its provider epoch; the plan's own render takes its snapshot from the
    /// same gate. Until Worker admission (#1382) this is the one handle on the one plan. It is
    /// never moved while a stamp is open: every revision stamped here is published in the same
    /// admission.
    plan_gate: engine::realtime::PlanRevisionGate,
    /// Issue #140 A: one control-side producer per prepared effect instance, in the dense slot
    /// order [`ReadyOwnership::effect_slot`] computes (a staged command's `queue_slot`), so an
    /// addressed command reaches its cells and its observation queue with one index and no search. The producer's
    /// parameter, bypass and EQ-target values are latest-target cells (#1345); only an
    /// observation subscription is a queue record.
    effect_controls: Box<[Option<EffectControlProducer>]>,
    /// Issue #1222: one live send producer per route into a submix, in canonical route-ID order
    /// (`HostLiveControlHandles::route_controls`), so a send command's live-route index reaches
    /// its queue with one index and no search. Declared before the plan that owns their lanes.
    route_controls: Vec<RouteControlProducer>,
    /// Issue #1222 D3: every live send's gain, matrix and mute, parallel to `route_controls`. A
    /// send record folds all of them, so a command that moves one rebuilds the record from this
    /// mirror. It carries its own transaction shadow, committed or rolled back with `solo`.
    routes: LiveRouteState,
    /// Per-strip prefix sum of effect instances, so `effect_slot` is arithmetic, not a lookup:
    /// the tracks first, then the submixes (issue #1207 D3), so a track's entry is the same with
    /// or without submixes.
    effect_base: Box<[u32]>,
    /// Room needed per destination queue by the submission being validated: the observation
    /// subscriptions it queues on each effect (every other record is a cell write and wants none).
    /// Allocated at compilation, one entry per queue (see [`ReadyOwnership::effect_slot`]).
    command_wanted: Box<[u32]>,
    /// The observation taps the submission being validated has named, one bit per tap, one word per
    /// effect instance (a session binds at most [`MAXIMUM_OBSERVATION_TAPS`] taps per effect).
    /// An `Observe` record overwrites its tap's whole state at the block that drains it and one
    /// submission drains in one block, so a submission names each tap once (#1345 D2). Cleared with
    /// `command_wanted` at the top of every submission.
    observe_named: Box<[u32]>,
    /// Decoded submission staging. Two entries per staged record, because one wire record
    /// addressed to `channel = both` on a per-lane effect parameter lowers to one span per lane,
    /// plus `2 * strip_count` for the coalesced solo emission (issue #210 phase 1, per strip
    /// since issue #1213), plus one per live send for the follow pass (issue #1224 D4).
    command_decoded: Box<[StagedCommand]>,
    /// Issue #210 phase 1: the live controls' solo bits and their mirrors of user mute and of what
    /// the render plane was last told. Solo composes into the *existing* mute records at admission
    /// and adds nothing below it, so this is the whole of solo-in-place on the host side.
    solo: LiveControlSoloState,
    /// Issue #1245 D3: every VCA's offset and mute, every reached member's own fader value and the
    /// effective fader value the render plane was last told. A VCA move composes into the
    /// *existing* member fader records here, and its mute term into `solo`. It carries its own
    /// transaction shadow, committed or rolled back with `solo` and `routes`. Empty -- and
    /// allocated nothing -- for a session without VCAs or without live controls.
    vcas: LiveVcaState,
    /// Issue #1560: what the render plane was last told about each strip's own fader (for a strip
    /// no VCA reaches), matrix, trim and polarity, so a command that repeats the committed value
    /// stages no record, as the core's classifier writes none for it. It carries its own
    /// transaction shadow, committed or rolled back with `solo`.
    strip_values: LiveStripValues,
    /// Issue #1364 D2: what an absent ramp length resolves to, computed once at preparation.
    ramps: SessionRamps,
    /// The core's descriptors of the rows the live kinds write, resolved once at preparation
    /// (decision 17, slice S7c D1).
    rows: WebControlRows,
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
    /// Per-strip semantic input-filter shadows (tracks, then submixes; issue #1213 D1), retained
    /// only when commands are enabled. They stay the control-side authority for the designed
    /// pair (#1346 D6); the strip's input cells hold the prepared targets.
    input_filter_shadows: Box<[BuiltinInputShadow]>,
    /// The compiled session rate used by input-filter semantic validation.
    sample_rate_hz: u32,
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
    host: PreparedHost,
    /// Issue #137 D2: meter consumers, declared after the plan that owns their producers. Empty
    /// when `live_control_meter_blocks` was zero, in which case no observer exists at all.
    meters: Vec<MeterConsumer>,
    /// Issue #143: one reader set per observed effect instance, in the same dense `effect_slot`
    /// order the command producers use, so an addressed subscription reaches its lane with one
    /// index and no search. Empty when the configuration named no observation capacity.
    effect_observations: Box<[Option<EffectObservationHandle>]>,
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
    /// The compiled session model, retained so the browser bridge keeps charging itself for what
    /// it holds -- and, since issue #207, read by the source-introspection queries.
    ///
    /// The queries read it directly rather than copying a parallel table out of it, so the order
    /// they report *is* the normalized model's order by construction: `compile_session` sorts
    /// `sources` by stable ID, and there is no second list that could disagree with it.
    session: CompiledSession,
}

#[derive(Clone, Copy)]
struct MasterMeasurement {
    start_sample: u64,
    end_sample: u64,
    peak: [f32; 2],
}

impl ReadyOwnership {
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

    /// Strips these live controls address: the tracks, then the submixes (issue #1213 D1). Every
    /// queue band and per-strip table is sized by it, so a bus is addressed exactly as a track is.
    fn strip_count(&self) -> usize {
        self.tracks.len() + self.submixes.len()
    }

    /// The dense destination-queue index of one addressed live-control channel.
    ///
    /// The layout is frozen here and nowhere else, with `strips = T + S` (issue #1213 D1: the
    /// bands are per strip, tracks first, so with `S = 0` every index is where it always was):
    ///
    /// | range | queue |
    /// |---|---|
    /// | `0 .. strips` | strip `s`'s matrix/pan cell (#1312: no queue, no room) |
    /// | `strips .. 2 * strips` | strip `s`'s fader/mute cells (#1312: no queue, no room) |
    /// | `2 * strips .. 3 * strips` | strip `s`'s input trim/polarity/filter cells (#1346: no queue, no room) |
    /// | `3 * strips .. 3 * strips + E` | effect instances: `(strip, pre_insert/inserts/post_insert, position)`; their values are cells (#1345), their subscriptions the only queue |
    /// | `3 * strips + E ..` | live send `r`'s cell, by live-route index (issue #1222 D4; #1347: no queue, no room) |
    ///
    /// One index therefore serves both the free-room pre-check and the push, and neither pass has
    /// to search. `None` means the address names no channel this session prepared.
    ///
    /// The input band was **appended** to the per-track prefix rather than inserted, so the two
    /// existing bands keep their indices and the only arithmetic that moved is the effect base's
    /// `2 * tracks` -> `3 * tracks`. Every site that spells that constant is below and in
    /// [`Self::queue_available`], [`Self::push`] and the `queue_count` allocation; there is no
    /// fourth spelling.
    ///
    /// An effect is addressed in the session's terms, a console slot by its slot index or an
    /// insert by its index (issue #1096); [`LiveEffectAddress::lower`] is the one translation to
    /// the lowered rack that orders the band.
    fn effect_slot(&self, track: usize, address: LiveEffectAddress) -> Option<usize> {
        let slot = dense_effect_slot(
            *self.effect_base.get(track)?,
            *self.rack_effects.get(track)?,
            address,
        )?;
        (slot < self.effect_controls.len()).then_some(slot)
    }

    /// The free room of one destination queue. The matrix, fader, input and send bands are
    /// latest-target cells (#1312, #1346, #1347), which have no room to want and are never asked:
    /// they answer `None`, as an unaddressable queue does. An effect's values are cells too
    /// (#1345); its queue is its observation queue, one slot per declared tap.
    fn queue_available(&self, slot: usize) -> Option<u32> {
        let tracks = self.strip_count();
        if slot < tracks * 3 {
            return None;
        }
        let effect = slot - tracks * 3;
        let producer = self.effect_controls.get(effect)?.as_ref()?;
        u32::try_from(producer.observe_available_capacity()).ok()
    }

    /// The dense destination-queue index of live send `route` (issue #1222 D4): the band after
    /// the effects. `None` for an index past the last live route.
    fn route_slot(&self, route: usize) -> Option<usize> {
        (route < self.route_controls.len())
            .then(|| self.strip_count() * 3 + self.effect_controls.len() + route)
    }

    /// Queue one admitted observation subscription on its effect. `Err` only on a full queue,
    /// which the free-room pass has already ruled out, or on a cell record (a strip's, a send's or
    /// an effect's parameter or bypass), which is never queued.
    fn push(
        &mut self,
        slot: usize,
        record: AdmittedCommand,
        applied_at_sample: u64,
    ) -> Result<(), ()> {
        let tracks = self.strip_count();
        match record {
            // The strips' fader, mute, matrix and input values and the sends' targets are cells,
            // written under the batch's stamp by the admission itself (`write_strip_cells`),
            // never pushed.
            AdmittedCommand::Matrix(_)
            | AdmittedCommand::Fader(_)
            | AdmittedCommand::Input(_)
            | AdmittedCommand::Route(_) => Err(()),
            AdmittedCommand::Effect(
                EffectControlRecord::Parameter { .. }
                | EffectControlRecord::PreparedTarget(_)
                | EffectControlRecord::Bypass(_),
            ) => Err(()),
            AdmittedCommand::Effect(EffectControlRecord::Observe {
                tap_index,
                armed,
                window_blocks,
            }) => {
                let effect = slot - tracks * 3;
                // Issue #143: the armed set is control-plane state and is updated here, where the
                // record is admitted, so it can never disagree with what the render side was told.
                let producer = self
                    .effect_controls
                    .get_mut(effect)
                    .ok_or(())?
                    .as_mut()
                    .ok_or(())?;
                producer
                    .try_push_observe(tap_index, armed, window_blocks)
                    .map_err(|_| ())?;
                if let Some(mask) = self.observation_armed.get_mut(effect)
                    && tap_index < u32::BITS
                {
                    let bit = 1_u32 << tap_index;
                    *mask = if armed { *mask | bit } else { *mask & !bit };
                    if let Some(samples) = self.observation_arm_samples.get_mut(effect)
                        && let Some(sample) = samples.get_mut(tap_index as usize)
                    {
                        *sample = applied_at_sample;
                    }
                }
                Ok(())
            }
        }
    }

    /// Preflight one effect record before any cell write or observation mutation.
    ///
    /// The host admission pass owns the original wire index, so this check belongs beside
    /// lowering rather than in [`Self::push`].  A prepared-capable owner must reject an
    /// unlowered semantic parameter before a mixed batch can mutate another destination.  The
    /// checked producer still runs the same check at the write as an invariant guard; this pass
    /// is what preserves the original wire index and whole-batch refusal contract. An
    /// observation subscription is not a cell and is room-checked, not preflighted.
    fn preflight_effect(&self, slot: usize, record: EffectControlRecord) -> Result<(), ()> {
        if matches!(record, EffectControlRecord::Observe { .. }) {
            return Ok(());
        }
        let effect_base = self.strip_count().checked_mul(3).ok_or(())?;
        let effect = slot.checked_sub(effect_base).ok_or(())?;
        let producer = self
            .effect_controls
            .get(effect)
            .and_then(Option::as_ref)
            .ok_or(())?;
        producer.preflight(record).map_err(|_| ())
    }
}

/// The one field a send command moves (issue #1222 D1).
#[derive(Clone, Copy)]
enum RouteEdit {
    GainDb(f32),
    Mute(bool),
    /// `[ll, lr, rl, rr]`.
    Matrix([f32; 4]),
}

/// The one VCA field a VCA command moves, on the lanes it covers (issue #1245 D1).
#[derive(Clone, Copy)]
enum VcaEdit {
    FaderDb(BuiltinLaneSelector, f32),
    Mute(BuiltinLaneSelector, bool),
}

/// One decoded record and the payload its destination queue takes (issue #140 C).
#[derive(Clone, Copy)]
enum AdmittedCommand {
    /// The matrix/pan channel #137 D1 shipped.
    Matrix(TrackControlRecord),
    /// The fader/mute channel (#140 B).
    Fader(TrackFaderRecord),
    /// The input trim/polarity channel (#210 phase 3).
    Input(TrackInputRecord),
    /// One effect instance's channel (#140 A).
    Effect(EffectControlRecord),
    /// One live send's channel (issue #1222 D4).
    Route(RouteControlRecord),
}

#[derive(Clone, Copy)]
struct BuiltinInputShadow {
    committed: [f32; 4],
    candidate: [f32; 4],
    dirty: [bool; 4],
    revision: u64,
}

impl BuiltinInputShadow {
    fn begin(&mut self) {
        self.candidate = self.committed;
        self.dirty = [false; 4];
    }

    fn apply(&mut self, sample_rate_hz: u32, edit: InputFilterEdit) -> Result<(), u32> {
        apply_input_filter_edit(sample_rate_hz, &mut self.candidate, &mut self.dirty, edit).map_err(
            |error| match error {
                InputFilterEditErrorKind::Parameter => COMMAND_REASON_MALFORMED,
                InputFilterEditErrorKind::Domain => COMMAND_REASON_DOMAIN,
                InputFilterEditErrorKind::Shape => COMMAND_REASON_MALFORMED,
            },
        )
    }

    fn rollback(&mut self) {
        self.candidate = self.committed;
        self.dirty = [false; 4];
    }

    fn commit(&mut self, base_revision: u64) -> Result<(), ()> {
        let next_revision = self.revision.checked_add(1).ok_or(())?;
        if self.revision != base_revision {
            return Err(());
        }
        self.committed = self.candidate;
        self.dirty = [false; 4];
        self.revision = next_revision;
        Ok(())
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum PreparedOwnerKind {
    Eq,
    BuiltinInput,
}

#[derive(Clone, Copy)]
struct PreparedOwner {
    kind: PreparedOwnerKind,
    queue_slot: u32,
    base_revision: u64,
    first_wire_index: u32,
    target_count: u32,
}

#[derive(Clone, Copy)]
enum StagedCommandKind {
    Command(AdmittedCommand),
    PreparedOwner(PreparedOwner),
}

#[derive(Clone, Copy)]
struct StagedCommand {
    queue_slot: u32,
    original_wire_index: u32,
    kind: StagedCommandKind,
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
    /// First sample eligible for the current lease. A snapshot that opens before this boundary
    /// is an old/partial interval and is dropped rather than presented as a clean measurement.
    meter_activation_sample: u64,
    buffers: Option<PreparedBuffers>,
    ready: Option<ReadyOwnership>,
    /// Checked host-scoped generation for prepared companion ownership. This token is distinct
    /// from the wrapping Wasm handle and changes on every successfully published host.
    host_generation: u64,
    /// Optional prepared cadence for the ordinary spectrum capture.
    spectrum_hop: Option<SpectrumHop>,
    diagnostic_len: usize,
}

static NEXT_HOST_GENERATION: AtomicU64 = AtomicU64::new(1);

fn next_host_generation() -> Option<u64> {
    NEXT_HOST_GENERATION
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
            current.checked_add(1)
        })
        .ok()
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
                .max(shape.longest_submix_id_bytes)
                .max(shape.longest_route_id_bytes)
                .max(shape.longest_vca_id_bytes),
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
        // Issue #1245 amendment A1: the live VCA state's retained bytes and the transient its
        // construction holds, on top of everything retained before it, fit the budget; the bounds
        // above keep both small. Checked before preparation builds a single reach table.
        let vca_shape = browser_vca_shape(session.normalized_model())?;
        let vca_peak = vca_shape
            .retained_bytes()
            .zip(vca_shape.transient_bytes())
            .and_then(|(retained, transient)| retained.checked_add(transient))
            .and_then(|vca| vca.checked_add(retained_projection))
            .ok_or_else(|| BootFailure::fixed(RESULT_REFUSED_BUDGET, "host.budget.arithmetic"))?;
        if vca_peak > memory_budget {
            return Err(BootFailure::projected_budget(
                "host.budget.vca_projection",
                vca_peak,
                memory_budget,
            ));
        }
        let caps = prepare_caps(&session, options, source_ring_frames, memory_budget);
        let (ready, resources) = compile_ready(
            session,
            &caps,
            options,
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
        let host_generation = next_host_generation()
            .ok_or_else(|| BootFailure::fixed(RESULT_REFUSED_BUDGET, "web.host.generation"))?;
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
            meter_lease: false,
            meter_activation_sample: 0,
            buffers: Some(buffers),
            ready: Some(ready),
            host_generation,
            spectrum_hop: spectrum_request.as_ref().and(spectrum_hop),
            diagnostic_len: 0,
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

    /// Number of live routes (issue #1223 D1): exactly the plan's live send producers, the routes
    /// into submixes in canonical route-ID order, which is the order a send kind's index word
    /// reads. Zero before compilation and without live controls.
    #[must_use]
    pub fn live_control_route_count(&self) -> usize {
        self.ready
            .as_ref()
            .map_or(0, |ready| ready.route_controls.len())
    }

    /// Number of live VCAs (issue #1246 D1): the session's VCAs in canonical VCA-ID order, which
    /// is the order a VCA kind's (`16`, `17`) index word reads. It is the live VCA state's own
    /// count, the bound admission refuses an index against, so it is zero before compilation,
    /// without VCAs and without live controls.
    #[must_use]
    pub fn live_control_vca_count(&self) -> usize {
        self.ready
            .as_ref()
            .map_or(0, |ready| ready.vcas.vca_count())
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
            .map_or(0, |ready| ready.session.normalized_model().sources.len())
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
            .session
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
            .session
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
                .effect_observations
                .iter()
                .any(std::option::Option::is_some)
        })
    }

    /// Copy one selected track's response owners at the current completed render boundary.
    ///
    /// The prepared plan remains exclusively owned by this host. The caller supplies bounded
    /// storage and may finish/evaluate the immutable copy after this method returns.
    pub fn copy_response_snapshot(
        &mut self,
        track_id: &str,
        sink: &mut dyn ResponseSnapshotSink,
    ) -> Result<engine::realtime::ResponseSnapshotCapture, ResponseSnapshotError> {
        if self.status.state != STATE_READY {
            return Err(ResponseSnapshotError::Owner);
        }
        let Some(ready) = self.ready.as_mut() else {
            return Err(ResponseSnapshotError::Owner);
        };
        ready.host.copy_response_snapshot(track_id, sink)
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
        &self,
        selections: &[ObservationSelection<'_>],
    ) -> Result<Vec<ObservationReadResult>, ObservationReadError> {
        if self.status.state != STATE_READY {
            return Err(ObservationReadError::WrongState);
        }
        if selections.len() > MAXIMUM_OBSERVATION_READS {
            return Err(ObservationReadError::BufferTooSmall);
        }
        let Some(ready) = self.ready.as_ref() else {
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
            let values = read_observation_values(ready, self.status.sample_rate_hz, selection)?;
            let (effect, tap_index) = resolve_observation(ready, selection)?;
            let handle = ready
                .effect_observations
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
                .effect_observations
                .iter()
                .filter(|entry| entry.is_some())
                .count()
        })
    }

    /// One current-owner observation binding for the read-only session map.
    #[must_use]
    pub fn observation_binding(&self, index: u32) -> Option<ObservationBinding<'_>> {
        let ready = self.ready.as_ref()?;
        let mut observed_index = 0_u32;
        for (effect, handle) in ready.effect_observations.iter().enumerate() {
            let Some(handle) = handle.as_ref() else {
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
        &self,
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
        self.read_observations(&selections)
    }

    /// Fill a caller-owned numeric result slice without allocating on the Wasm boundary.
    pub fn read_observation_addresses_into(
        &self,
        addresses: &[ObservationAddress],
        output: &mut [ObservationReadValues],
    ) -> Result<(), ObservationReadError> {
        if self.status.state != STATE_READY {
            return Err(ObservationReadError::WrongState);
        }
        if addresses.len() > MAXIMUM_OBSERVATION_READS || output.len() < addresses.len() {
            return Err(ObservationReadError::BufferTooSmall);
        }
        let Some(ready) = self.ready.as_ref() else {
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
            output[index] = read_observation_values(ready, self.status.sample_rate_hz, &selection)?;
        }
        Ok(())
    }

    /// Number of complete meter windows folded since compilation.
    #[must_use]
    pub fn meter_windows(&self) -> u64 {
        self.ready.as_ref().map_or(0, |ready| ready.meter_windows)
    }

    #[cfg(test)]
    pub(crate) fn meter_poll_work(&self) -> (u64, u64) {
        self.ready.as_ref().map_or((0, 0), |ready| {
            (
                ready
                    .meters
                    .iter()
                    .map(|meter| meter.consumer.empty_count() + meter.consumer.success_count())
                    .sum(),
                ready.meter_effect_scan_count,
            )
        })
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

    /// Mutable opaque prepared-control companion staging. The caller must provide the exact
    /// little-endian frame before [`Self::submit_prepared_commands`].
    pub fn prepared_companion_mut(&mut self) -> Option<&mut [u8]> {
        self.buffers
            .as_mut()
            .and_then(|buffers| buffers.prepared_control.as_mut())
            .map(|workspace| &mut workspace.companion[..])
    }

    /// Exact fixed companion capacity in bytes.
    #[must_use]
    pub fn prepared_companion_capacity(&self) -> u32 {
        self.buffers
            .as_ref()
            .and_then(|buffers| buffers.prepared_control.as_ref())
            .map_or(0, |_| PREPARED_EFFECT_COMPANION_CAPACITY as u32)
    }

    /// Read-only addressed EQ configuration copy, or `None` when this host has no prepared owner
    /// for the requested effect. The output is retained in the one fixed workspace until the
    /// next successful copy.
    pub fn copy_eq_target_config(&mut self, track_index: u32, rack: u32, effect_index: u32) -> u32 {
        if self.status.state != STATE_READY {
            return self.record(RESULT_WRONG_STATE);
        }
        // The retired `simd1`/`simd2` codes decode to nothing (issue #1096).
        let Some(address) = live_effect_address(rack, effect_index) else {
            return self.record(RESULT_INVALID_ARGUMENT);
        };
        let Some(ready) = self.ready.as_ref() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        let Some(effect) = ready.effect_slot(track_index as usize, address) else {
            return self.record(RESULT_INVALID_ARGUMENT);
        };
        let Some(producer) = ready.effect_controls.get(effect).and_then(Option::as_ref) else {
            return self.record(RESULT_UNSUPPORTED);
        };
        if producer.descriptor.id.as_str() != "miso.parametric-eq" {
            return self.record(RESULT_UNSUPPORTED);
        }
        let Some(owner) = producer.owner() else {
            return self.record(RESULT_UNSUPPORTED);
        };
        if owner.committed().len() != host_core::EQ_VALUE_COUNT {
            return self.record(RESULT_UNSUPPORTED);
        }
        let sample_rate = owner.sample_rate();
        let owner_revision = owner.committed_revision();
        let values = owner.committed();
        let Some(buffers) = self.buffers.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.buffers\t$\n");
        };
        let Some(workspace) = buffers.prepared_control.as_mut() else {
            return self.record(RESULT_UNSUPPORTED);
        };
        let output = &mut workspace.config;
        output.fill(0);
        output[0..4].copy_from_slice(&(size_of::<WebEqTargetConfig>() as u32).to_le_bytes());
        output[4..8].copy_from_slice(&ABI_VERSION.to_le_bytes());
        output[8..12].copy_from_slice(&sample_rate.to_le_bytes());
        output[12..16].copy_from_slice(&(host_core::EQ_VALUE_COUNT as u32).to_le_bytes());
        output[16..24].copy_from_slice(&self.host_generation.to_le_bytes());
        output[24..32].copy_from_slice(&owner_revision.to_le_bytes());
        for (index, value) in values.iter().enumerate() {
            let offset = 32 + index * 4;
            output[offset..offset + 4].copy_from_slice(&value.value.to_bits().to_le_bytes());
        }
        self.record(RESULT_OK)
    }

    /// Fixed address of the most recent addressed EQ configuration copy.
    #[must_use]
    pub fn eq_target_config(&self) -> Option<&[u8]> {
        self.buffers
            .as_ref()
            .and_then(|buffers| buffers.prepared_control.as_ref())
            .map(|workspace| workspace.config.as_slice())
    }

    /// Copy one accepted builtin input-filter configuration into the shared config workspace.
    pub fn copy_input_filter_config(&mut self, track_index: u32) -> u32 {
        if self.status.state != STATE_READY {
            return self.record(RESULT_WRONG_STATE);
        }
        let Some(ready) = self.ready.as_ref() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        let Some(shadow) = ready.input_filter_shadows.get(track_index as usize) else {
            return self.record(RESULT_UNSUPPORTED);
        };
        let values = shadow.committed;
        let revision = shadow.revision;
        let sample_rate = ready.sample_rate_hz;
        let Some(buffers) = self.buffers.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.buffers\t$\n");
        };
        let Some(workspace) = buffers.prepared_control.as_mut() else {
            return self.record(RESULT_UNSUPPORTED);
        };
        let output = &mut workspace.config;
        output.fill(0);
        output[0..4].copy_from_slice(&(size_of::<WebBuiltinInputConfig>() as u32).to_le_bytes());
        output[4..8].copy_from_slice(&ABI_VERSION.to_le_bytes());
        output[8..12].copy_from_slice(&sample_rate.to_le_bytes());
        output[12..16].copy_from_slice(&4_u32.to_le_bytes());
        output[16..24].copy_from_slice(&self.host_generation.to_le_bytes());
        output[24..32].copy_from_slice(&revision.to_le_bytes());
        for (index, value) in values.into_iter().enumerate() {
            let offset = 32 + index * 4;
            output[offset..offset + 4].copy_from_slice(&value.to_bits().to_le_bytes());
        }
        self.record(RESULT_OK)
    }

    /// Submit a prepared companion alongside the existing semantic command staging. This private
    /// trusted-host route shares the ordinary lowering and report, but requires every touched
    /// prepared owner to publish its supplied target prefix before any shadow or solo commit.
    pub fn submit_prepared_commands(&mut self, count: u32, companion_bytes: u32) -> u32 {
        self.submit_commands_inner(count, Some(companion_bytes))
    }

    /// Whether the app opted in to live commands (issue #137 D1). Every plan carries each strip's
    /// fader/mute and matrix/pan lanes whatever this says (issue #1326), so it keys on the boot
    /// option, not on the lanes.
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

    /// Copy one live route ID into ID staging; returns its byte length (issue #1223 D1). The
    /// index is the live-route index, read from the producer the send kinds address.
    pub(crate) fn copy_live_control_route_id(&mut self, index: u32) -> u32 {
        let Some(ready) = self.ready.as_ref() else {
            return 0;
        };
        let Some(producer) = ready.route_controls.get(index as usize) else {
            return 0;
        };
        Self::copy_id_into_staging(self.buffers.as_mut(), producer.route_id().as_bytes())
    }

    /// Copy one VCA ID into ID staging; returns its byte length (issue #1246 D1). VCA `index` is
    /// the normalized model's `vcas[index]`, the model the live VCA state was built from, and an
    /// index at or past [`Self::live_control_vca_count`] -- every index without live controls --
    /// copies nothing.
    pub(crate) fn copy_live_control_vca_id(&mut self, index: u32) -> u32 {
        let Some(ready) = self.ready.as_ref() else {
            return 0;
        };
        let index = index as usize;
        if index >= ready.vcas.vca_count() {
            return 0;
        }
        let Some(vca) = ready.session.normalized_model().vcas.get(index) else {
            return 0;
        };
        Self::copy_id_into_staging(self.buffers.as_mut(), vca.id.as_str().as_bytes())
    }

    /// Copy one canonical source ID into ID staging; returns its byte length (issue #207).
    pub(crate) fn copy_session_source_id(&mut self, index: u32) -> u32 {
        let Some(ready) = self.ready.as_ref() else {
            return 0;
        };
        let Some(source) = ready.session.normalized_model().sources.get(index as usize) else {
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

    /// Apply one strictly increasing generation-tagged source seek between render blocks.
    /// This web host owns both producer and render plan on one exclusive thread. Successful
    /// admission also prepares its consumer so new PCM can enter even when old queues were full.
    /// A seek changes source-content position while the absolute render clock stays continuous;
    /// it therefore neither resets meter accumulators nor advances the meter publication epoch.
    pub fn seek_source(&mut self, source_id: &[u8], generation: u64, source_frame: u64) -> u32 {
        if self.status.state != STATE_READY {
            return self.record(RESULT_WRONG_STATE);
        }
        let Some(ready) = self.ready.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        let Ok(source_index) = ready
            .session
            .normalized_model()
            .sources
            .binary_search_by(|source| source.id.as_str().as_bytes().cmp(source_id))
        else {
            return self.record(RESULT_INVALID_ARGUMENT);
        };
        if !ready.host.plan.can_prepare_source_seek(source_index) {
            return self.record(RESULT_WRONG_STATE);
        }
        let code = match ready.host.sources.seek(source_id, generation, source_frame) {
            Ok(()) => {
                if ready
                    .host
                    .plan
                    .prepare_source_seek(source_index, generation, source_frame)
                {
                    RESULT_OK
                } else {
                    RESULT_INTERNAL
                }
            }
            Err(error) => source_result(error),
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
        let render_first_sample = self.status.next_absolute_sample;
        let result = ready.host.plan.render(
            RenderIo { output },
            RenderTime {
                absolute_sample: self.status.next_absolute_sample,
            },
        );
        match result {
            Ok(report) => {
                self.status.next_absolute_sample = report.next_absolute_sample;
                self.status.rendered_quanta = self.status.rendered_quanta.saturating_add(1);
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
                    ready.master_end_sample = report.next_absolute_sample;
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

    /// Admit one staged `miso.command.v1` submission as a single transaction (issue #137 D1).
    ///
    /// # The two-pass shape is the contract, not an optimisation
    ///
    /// Pass one validates every record -- shape, addressing, domain, and free queue room -- and
    /// pushes nothing. Pass two pushes (and writes the strip cells, which have no room to check,
    /// #1312). A submission is therefore all-or-nothing: eval E4's "engine state untouched" is a
    /// property of the code, not of a lucky ordering, and eval E3's flood of queued records is
    /// refused before a single one reaches a queue.
    ///
    /// # Why this is not on the render thread's critical path
    ///
    /// This runs from `port.onmessage`, which the user agent dispatches between render quanta on
    /// the same thread as `process()`. It performs no allocation, no compilation and no plan
    /// replacement: both staging arrays were allocated at compilation, and the call moves at most
    /// [`MAXIMUM_COMMAND_RECORDS`] `Copy` records into bounded per-track queues and strip cells.
    /// The block that follows drains the queues and reads the cells.
    pub fn submit_commands(&mut self, count: u32) -> u32 {
        self.submit_commands_inner(count, None)
    }

    fn submit_commands_inner(&mut self, count: u32, companion_bytes: Option<u32>) -> u32 {
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
        let (prepared, companion) = match companion_bytes {
            None => (false, None),
            Some(bytes_len) => {
                let Ok(bytes_len) = usize::try_from(bytes_len) else {
                    return self.finish_commands(
                        RESULT_INVALID_ARGUMENT,
                        COMMAND_REASON_MALFORMED,
                        0,
                        0,
                    );
                };
                let Some(companion) = buffers
                    .prepared_control
                    .as_ref()
                    .and_then(|workspace| workspace.companion.get(..bytes_len))
                else {
                    return self.finish_commands(
                        RESULT_INVALID_ARGUMENT,
                        COMMAND_REASON_MALFORMED,
                        0,
                        0,
                    );
                };
                (true, Some(companion))
            }
        };
        match admit_commands(
            ready,
            bytes,
            count as usize,
            applied_at_sample,
            self.host_generation,
            prepared,
            companion,
        ) {
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

    /// Entries in the decode staging array, for the issue #210 phase 1 sizing pin.
    #[cfg(test)]
    pub(crate) fn command_staging_entries(&self) -> Option<usize> {
        self.ready.as_ref().map(|ready| ready.command_decoded.len())
    }

    /// The live controls' solo state, for the issue #210 phase 1 evals.
    ///
    /// The ABI has no readback of it on purpose -- solo is control-plane state and the app is the
    /// one that issued every gesture that moved it -- so this exists only for the tests that have
    /// to prove the mirror agrees with the session it was prepared from.
    #[cfg(test)]
    pub(crate) fn live_control_solo(&self) -> Option<&LiveControlSoloState> {
        self.ready.as_ref().map(|ready| &ready.solo)
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
        let Some(ready) = self.ready.as_mut() else {
            return 0;
        };
        // This function has no runtime-checked `[]` index (only `get`/`get_mut` and constant
        // indexes into a fixed array), so it adds no bounds check of its own. The one
        // `panic_bounds_check` in this export's call graph is the slot index of the inlined SPSC
        // `try_pop` (`slots[self.local]` in `crates/engine/src/realtime/spsc.rs`), which the ring
        // keeps in range. This export is called from `process()`, so the shipped artifact's
        // call-graph gate covers it as it covers the render export.
        let meter_count = ready.meters.len();
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
        for (meter, remaining) in ready.meters.iter().zip(&mut ready.meter_remaining) {
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
            for ((meter, pending), remaining) in ready
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
                    ready
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
        for (effect, entry) in ready.effect_observations.iter().enumerate() {
            #[cfg(test)]
            {
                ready.meter_effect_scan_count = ready.meter_effect_scan_count.saturating_add(1);
            }
            let Some(handle) = entry.as_ref() else {
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
                let Some(tap) = handle.descriptor.observations.get(tap_index) else {
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

fn owner_command_reason(error: host_core::EffectControlOwnerError) -> u32 {
    use host_core::EffectControlOwnerError;
    match error {
        EffectControlOwnerError::Domain => COMMAND_REASON_DOMAIN,
        EffectControlOwnerError::Parameter => COMMAND_REASON_UNKNOWN_PARAMETER,
        EffectControlOwnerError::Unsupported | EffectControlOwnerError::NotAutomatable => {
            COMMAND_REASON_UNSUPPORTED_KIND
        }
        EffectControlOwnerError::Revision | EffectControlOwnerError::Invariant => {
            COMMAND_REASON_WRONG_STATE
        }
        EffectControlOwnerError::Rate
        | EffectControlOwnerError::Shape
        | EffectControlOwnerError::Target(_) => COMMAND_REASON_MALFORMED,
    }
}

struct PreparedCompanion<'a> {
    bytes: &'a [u8],
    target_count: usize,
}

#[derive(Clone, Copy)]
struct CompanionTarget {
    track_index: u32,
    rack: u8,
    effect_index: u32,
    base_revision: u64,
    target: effect_contract::PreparedEffectTarget,
}

fn builtin_input_slot(strip_count: usize, strip: usize) -> Option<usize> {
    strip_count
        .checked_mul(2)
        .and_then(|base| base.checked_add(strip))
}

fn prepared_queue_address(
    ready: &ReadyOwnership,
    track: usize,
    rack: u8,
    effect: u32,
) -> Option<(usize, PreparedOwnerKind)> {
    if rack == RACK_NOT_APPLICABLE {
        if effect != 0 || ready.input_filter_shadows.get(track).is_none() {
            return None;
        }
        return Some((
            builtin_input_slot(ready.strip_count(), track)?,
            PreparedOwnerKind::BuiltinInput,
        ));
    }
    let effect_slot = ready.effect_slot(track, live_effect_address(u32::from(rack), effect)?)?;
    Some((
        ready
            .strip_count()
            .checked_mul(3)?
            .checked_add(effect_slot)?,
        PreparedOwnerKind::Eq,
    ))
}

fn read_u32_le(bytes: &[u8], offset: usize) -> Option<u32> {
    Some(u32::from_le_bytes(
        bytes.get(offset..offset + 4)?.try_into().ok()?,
    ))
}

fn read_u64_le(bytes: &[u8], offset: usize) -> Option<u64> {
    Some(u64::from_le_bytes(
        bytes.get(offset..offset + 8)?.try_into().ok()?,
    ))
}

fn parse_prepared_companion<'a>(
    bytes: &'a [u8],
    host_generation: u64,
) -> Result<PreparedCompanion<'a>, ()> {
    let header = size_of::<WebPreparedEffectCompanionHeader>();
    if bytes.len() < header {
        return Err(());
    }
    let struct_size = read_u32_le(bytes, 0).ok_or(())?;
    let abi_version = read_u32_le(bytes, 4).ok_or(())?;
    let generation = read_u64_le(bytes, 8).ok_or(())?;
    let target_count = usize::try_from(read_u32_le(bytes, 16).ok_or(())?).map_err(|_| ())?;
    let reserved = read_u32_le(bytes, 20).ok_or(())?;
    if struct_size as usize != header
        || abi_version != ABI_VERSION
        || generation != host_generation
        || reserved != 0
        || target_count > 2 * MAXIMUM_COMMAND_RECORDS as usize
    {
        return Err(());
    }
    let expected = header
        .checked_add(
            target_count
                .checked_mul(size_of::<WebPreparedEffectCompanionRecord>())
                .ok_or(())?,
        )
        .ok_or(())?;
    if bytes.len() != expected {
        return Err(());
    }
    Ok(PreparedCompanion {
        bytes,
        target_count,
    })
}

fn companion_wire_index(
    ready: &ReadyOwnership,
    lowered: usize,
    companion: &PreparedCompanion<'_>,
    index: usize,
) -> usize {
    let start = size_of::<WebPreparedEffectCompanionHeader>()
        + index * size_of::<WebPreparedEffectCompanionRecord>();
    let address = (|| {
        let track = read_u32_le(companion.bytes, start)? as usize;
        let rack = u8::try_from(read_u32_le(companion.bytes, start + 4)?).ok()?;
        let effect = read_u32_le(companion.bytes, start + 8)?;
        prepared_queue_address(ready, track, rack, effect)
    })();
    let Some((queue_slot, kind)) = address else {
        return 0;
    };
    ready.command_decoded[..lowered]
        .iter()
        .find_map(|entry| match entry.kind {
            StagedCommandKind::PreparedOwner(owner)
                if owner.queue_slot as usize == queue_slot && owner.kind == kind =>
            {
                Some(owner.first_wire_index as usize)
            }
            _ => None,
        })
        .unwrap_or(0)
}

fn companion_target(
    companion: &PreparedCompanion<'_>,
    index: usize,
) -> Result<CompanionTarget, ()> {
    let start = size_of::<WebPreparedEffectCompanionHeader>()
        .checked_add(
            index
                .checked_mul(size_of::<WebPreparedEffectCompanionRecord>())
                .ok_or(())?,
        )
        .ok_or(())?;
    let bytes = companion
        .bytes
        .get(start..start + size_of::<WebPreparedEffectCompanionRecord>())
        .ok_or(())?;
    let track_index = read_u32_le(bytes, 0).ok_or(())?;
    let rack = u8::try_from(read_u32_le(bytes, 4).ok_or(())?).map_err(|_| ())?;
    let effect_index = read_u32_le(bytes, 8).ok_or(())?;
    if read_u32_le(bytes, 12).ok_or(())? != 0 {
        return Err(());
    }
    let base_revision = read_u64_le(bytes, 16).ok_or(())?;
    let slot = read_u32_le(bytes, 24).ok_or(())?;
    let channel_raw = read_u32_le(bytes, 28).ok_or(())?;
    let channel = match channel_raw {
        0 => effect_contract::ParameterChannel::Left,
        1 => effect_contract::ParameterChannel::Right,
        2 => effect_contract::ParameterChannel::Both,
        _ => return Err(()),
    };
    let mut words = [0_u32; effect_contract::PREPARED_EFFECT_TARGET_WORDS];
    for (word, value) in words.iter_mut().enumerate() {
        *value = read_u32_le(bytes, 32 + word * 4).ok_or(())?;
    }
    Ok(CompanionTarget {
        track_index,
        rack,
        effect_index,
        base_revision,
        target: effect_contract::PreparedEffectTarget {
            slot,
            channel,
            words,
        },
    })
}

fn companion_builtin_target(target: CompanionTarget) -> Result<PreparedInputFilterTarget, ()> {
    if target.target.slot > 1 || target.target.words[2..6].iter().any(|word| *word != 0) {
        return Err(());
    }
    let lanes = match target.target.channel {
        ParameterChannel::Left => BuiltinLaneSelector::Left,
        ParameterChannel::Right => BuiltinLaneSelector::Right,
        ParameterChannel::Both => BuiltinLaneSelector::Both,
    };
    let prepared = PreparedInputFilterTarget {
        lanes,
        section: target.target.slot,
        pair: [
            f32::from_bits(target.target.words[0]),
            f32::from_bits(target.target.words[1]),
        ],
        coefficients: core::array::from_fn(|index| f32::from_bits(target.target.words[6 + index])),
    };
    validate_prepared_input_filter_target(&prepared).map_err(|_| ())?;
    Ok(prepared)
}

fn validate_builtin_target_set(
    shadow: &BuiltinInputShadow,
    targets: &[PreparedInputFilterTarget],
) -> Result<(), ()> {
    if targets.len() > 4 {
        return Err(());
    }
    let mut seen = [[false; 2]; 2];
    for target in targets {
        let section = usize::try_from(target.section).map_err(|_| ())?;
        if section > 1 {
            return Err(());
        }
        let pairs = match target.lanes {
            BuiltinLaneSelector::Left => {
                if target.pair[0].to_bits() != shadow.candidate[0].to_bits()
                    || target.pair[1].to_bits() != shadow.candidate[1].to_bits()
                {
                    return Err(());
                }
                [true, false]
            }
            BuiltinLaneSelector::Right => {
                if target.pair[0].to_bits() != shadow.candidate[2].to_bits()
                    || target.pair[1].to_bits() != shadow.candidate[3].to_bits()
                {
                    return Err(());
                }
                [false, true]
            }
            BuiltinLaneSelector::Both => {
                if shadow.candidate[0].to_bits() != shadow.candidate[2].to_bits()
                    || shadow.candidate[1].to_bits() != shadow.candidate[3].to_bits()
                    || target.pair[0].to_bits() != shadow.candidate[0].to_bits()
                    || target.pair[1].to_bits() != shadow.candidate[1].to_bits()
                {
                    return Err(());
                }
                [true, true]
            }
        };
        for (lane, selected) in pairs.into_iter().enumerate() {
            if selected {
                if !shadow.dirty[lane * 2 + section] || seen[section][lane] {
                    return Err(());
                }
                seen[section][lane] = true;
            }
        }
    }
    for (section, section_seen) in seen.iter().enumerate() {
        for (lane, present) in section_seen.iter().enumerate() {
            if shadow.dirty[lane * 2 + section] != *present {
                return Err(());
            }
        }
    }
    Ok(())
}

/// One decoded, still-unapplied `miso.command.v1` record (issue #137 D1).
///
/// # The frozen 48-byte little-endian layout
///
/// | offset | width | field |
/// |---|---|---|
/// | 0 | u8 | `kind`, one of the `COMMAND_*` values |
/// | 1 | u8 | `rack`: `1` inserts, `3` console, `255` not applicable; `0` and `2` retired |
/// | 2 | u8 | `channel`: `0` left, `1` right, `2` both, `255` not applicable |
/// | 3 | u8 | required zero |
/// | 4 | u32 | `track_index` into the canonical normalized track order |
/// | 8 | u32 | `effect_index`: a console slot's index in slot order, or an insert's index |
/// | 12 | u32 | `parameter_id` from the effect contract |
/// | 16 | u32 | `smoothing_samples`, the ramp window for a retarget, or [`SESSION_DEFAULT_RAMP`] (`0xFFFF_FFFF`) for the session's own length |
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
                | COMMAND_OBSERVE_SUBSCRIBE
                | COMMAND_OBSERVE_UNSUBSCRIBE
                | COMMAND_SOLO
                | COMMAND_TRIM_DB
                | COMMAND_POLARITY_INVERT
                | COMMAND_INPUT_FILTERS
                | COMMAND_ROUTE_GAIN_DB
                | COMMAND_ROUTE_MUTE
                | COMMAND_ROUTE_MATRIX
                | COMMAND_VCA_FADER_DB
                | COMMAND_VCA_MUTE
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

    /// Replace an absent ramp length with the session's own (issue #1364 D2), before any check of
    /// the record. Only the twelve ramped kinds read the word as a length; every other kind keeps
    /// it, so a sentinel on input filters or bypass is the non-zero word they refuse as
    /// `MALFORMED`, exactly as before. A finite length, `0` included, is kept.
    ///
    /// The row each kind reads is #1054 D3's: fader, trim, send gain and VCA offset the fader
    /// key; mute, solo, send mute and VCA mute the mute key; polarity twice the mute key; pan,
    /// matrix and send matrix the pan key, with a strip's own pan or matrix window first. A
    /// `follows_mute` send record takes the ramp of the strip mute record that moved it, which
    /// this resolves, so it follows the mute key too.
    fn with_session_ramp(mut self, ramps: &SessionRamps) -> Self {
        let row = match self.kind {
            COMMAND_PAN | COMMAND_MATRIX => {
                self.smoothing_samples =
                    ramps.resolve_strip_pan(self.track_index as usize, self.smoothing_samples);
                return self;
            }
            COMMAND_FADER_DB => LiveRampRow::Fader,
            COMMAND_TRIM_DB => LiveRampRow::InputTrim,
            COMMAND_MUTE => LiveRampRow::Mute,
            COMMAND_SOLO => LiveRampRow::Solo,
            COMMAND_POLARITY_INVERT => LiveRampRow::PolarityInvert,
            COMMAND_ROUTE_GAIN_DB => LiveRampRow::SendGain,
            COMMAND_ROUTE_MUTE => LiveRampRow::SendMute,
            COMMAND_ROUTE_MATRIX => LiveRampRow::SendMatrix,
            COMMAND_VCA_FADER_DB => LiveRampRow::VcaOffset,
            COMMAND_VCA_MUTE => LiveRampRow::VcaMute,
            _ => return self,
        };
        self.smoothing_samples = ramps.resolve(row, self.smoothing_samples);
        self
    }

    fn into_input_filter_edit(self) -> Result<InputFilterEdit, u32> {
        if self.rack != RACK_NOT_APPLICABLE
            || self.effect_index != 0
            || self.smoothing_samples != 0
            || self.values[2..].iter().any(|value| value.to_bits() != 0)
        {
            return Err(COMMAND_REASON_MALFORMED);
        }
        if !matches!(self.parameter_id, 0 | 3 | 4) {
            return Err(COMMAND_REASON_MALFORMED);
        }
        if self.parameter_id != 0 && self.values[1].to_bits() != 0 {
            return Err(COMMAND_REASON_MALFORMED);
        }
        let channel = lane_selector(self.channel).ok_or(COMMAND_REASON_MALFORMED)?;
        let channel = match channel {
            BuiltinLaneSelector::Left => ParameterChannel::Left,
            BuiltinLaneSelector::Right => ParameterChannel::Right,
            BuiltinLaneSelector::Both => ParameterChannel::Both,
        };
        Ok(InputFilterEdit {
            parameter_id: self.parameter_id,
            channel,
            value0: self.values[0],
            value1: self.values[1],
        })
    }

    /// Lower one addressed track-builtin record, or say why it cannot be.
    ///
    /// Issue #140 C: `fader_db` and `mute` are no longer refused here. They lower onto the live
    /// ramped fader section (`FaderMuteRampBuiltins`), which a live-control-attached track binds in
    /// place of the prepared `FaderMuteBuiltins`. A track with no live controls still refuses them
    /// -- with [`COMMAND_REASON_UNSUPPORTED_KIND`], because the target exists and the value is
    /// legal and this *session* has no write path -- which is exactly what the reason means.
    fn into_track_record(self, rows: &WebControlRows) -> Result<AdmittedCommand, u32> {
        match self.kind {
            COMMAND_PAN => {
                if self.rack != RACK_NOT_APPLICABLE
                    || self.channel != 255
                    || self.values[2] != 0.0
                    || self.values[3] != 0.0
                {
                    return Err(COMMAND_REASON_MALFORMED);
                }
                check_value(&rows.pan, Lane::Left, self.values[0])?;
                check_value(&rows.pan, Lane::Right, self.values[1])?;
                let matrix = pan_matrix(self.values[0], self.values[1])
                    .map_err(|_| COMMAND_REASON_DOMAIN)?;
                Ok(AdmittedCommand::Matrix(TrackControlRecord {
                    matrix,
                    smoothing_samples: self.smoothing_samples,
                }))
            }
            COMMAND_MATRIX => {
                if self.rack != RACK_NOT_APPLICABLE || self.channel != 255 {
                    return Err(COMMAND_REASON_MALFORMED);
                }
                for (row, value) in rows.matrix.iter().zip(self.values) {
                    check_value(row, Lane::Both, value)?;
                }
                let matrix = Matrix2x2 {
                    ll: self.values[0],
                    lr: self.values[1],
                    rl: self.values[2],
                    rr: self.values[3],
                }
                .checked()
                .map_err(|_| COMMAND_REASON_DOMAIN)?;
                Ok(AdmittedCommand::Matrix(TrackControlRecord {
                    matrix,
                    smoothing_samples: self.smoothing_samples,
                }))
            }
            COMMAND_FADER_DB => {
                if self.rack != RACK_NOT_APPLICABLE
                    || self.values[1..].iter().any(|value| *value != 0.0)
                {
                    return Err(COMMAND_REASON_MALFORMED);
                }
                let lanes = lane_selector(self.channel).ok_or(COMMAND_REASON_MALFORMED)?;
                check_value(&rows.fader_db, control_lane(lanes), self.values[0])?;
                Ok(AdmittedCommand::Fader(TrackFaderRecord::FaderDb {
                    lanes,
                    db: self.values[0],
                    smoothing_samples: self.smoothing_samples,
                }))
            }
            COMMAND_MUTE => {
                if self.rack != RACK_NOT_APPLICABLE
                    || self.values[1..].iter().any(|value| *value != 0.0)
                {
                    return Err(COMMAND_REASON_MALFORMED);
                }
                let lanes = lane_selector(self.channel).ok_or(COMMAND_REASON_MALFORMED)?;
                check_value(&rows.mute, control_lane(lanes), self.values[0])?;
                Ok(AdmittedCommand::Fader(TrackFaderRecord::Mute {
                    lanes,
                    muted: self.values[0] == 1.0,
                    smoothing_samples: self.smoothing_samples,
                }))
            }
            // #210 phase 3. The shape rules are `faderDb`'s, because the two parameters share a
            // domain and a smoothing law; the destination is the strip's input cells (#1346)
            // rather than the fader ones, which `admit_commands_staged` decides from the lowered
            // variant exactly as it already decides between the matrix and fader bands.
            COMMAND_TRIM_DB => {
                if self.rack != RACK_NOT_APPLICABLE
                    || self.values[1..].iter().any(|value| *value != 0.0)
                {
                    return Err(COMMAND_REASON_MALFORMED);
                }
                let lanes = lane_selector(self.channel).ok_or(COMMAND_REASON_MALFORMED)?;
                check_value(&rows.trim_db, control_lane(lanes), self.values[0])?;
                Ok(AdmittedCommand::Input(TrackInputRecord::TrimDb {
                    lanes,
                    db: self.values[0],
                    smoothing_samples: self.smoothing_samples,
                }))
            }
            // The shape rules are `mute`'s: a boolean-exact domain on `values[0]`.
            COMMAND_POLARITY_INVERT => {
                if self.rack != RACK_NOT_APPLICABLE
                    || self.values[1..].iter().any(|value| *value != 0.0)
                {
                    return Err(COMMAND_REASON_MALFORMED);
                }
                let lanes = lane_selector(self.channel).ok_or(COMMAND_REASON_MALFORMED)?;
                check_value(&rows.polarity_invert, control_lane(lanes), self.values[0])?;
                Ok(AdmittedCommand::Input(TrackInputRecord::PolarityInvert {
                    lanes,
                    inverted: self.values[0] == 1.0,
                    smoothing_samples: self.smoothing_samples,
                }))
            }
            _ => Err(COMMAND_REASON_MALFORMED),
        }
    }

    /// Check one send record's fixed shape and read its value (issue #1222 D1).
    ///
    /// `rack` and `channel` are `255`, `effect_index` and `parameter_id` are `0`, and every value
    /// word past the kind's own is `0.0`; anything else is `MALFORMED`, as the fader kinds rule.
    /// The value is checked against the send's own rows by the core (`DOMAIN`); the prepared
    /// route's coefficient function checks it again when the record is built.
    fn into_route_edit(self, rows: &WebControlRows) -> Result<RouteEdit, u32> {
        let own = match self.kind {
            COMMAND_ROUTE_GAIN_DB | COMMAND_ROUTE_MUTE => 1,
            COMMAND_ROUTE_MATRIX => 4,
            _ => return Err(COMMAND_REASON_MALFORMED),
        };
        if self.rack != RACK_NOT_APPLICABLE
            || self.channel != 255
            || self.effect_index != 0
            || self.parameter_id != 0
            || self.values[own..].iter().any(|value| *value != 0.0)
        {
            return Err(COMMAND_REASON_MALFORMED);
        }
        match self.kind {
            COMMAND_ROUTE_GAIN_DB => {
                check_value(&rows.route_gain_db, Lane::Both, self.values[0])?;
                Ok(RouteEdit::GainDb(self.values[0]))
            }
            COMMAND_ROUTE_MUTE => {
                check_value(&rows.route_mute, Lane::Both, self.values[0])?;
                Ok(RouteEdit::Mute(self.values[0] == 1.0))
            }
            _ => {
                for (row, value) in rows.route_matrix.iter().zip(self.values) {
                    check_value(row, Lane::Both, value)?;
                }
                Ok(RouteEdit::Matrix(self.values))
            }
        }
    }

    /// Check one VCA record's fixed shape and read its edit (issue #1245 D1).
    ///
    /// Kind 3's and kind 4's rules -- `rack` `255`, `channel` a lane selector, `values[1..]` `0.0`
    /// (else `MALFORMED`), an offset in `[-144, 24]` dB or a mute of exactly `0.0` or `1.0` (else
    /// `DOMAIN`) -- plus the route kinds' `effect_index` and `parameter_id` of `0` (else
    /// `MALFORMED`), which kinds 3 and 4 do not check.
    fn into_vca_edit(self, rows: &WebControlRows) -> Result<VcaEdit, u32> {
        if !matches!(self.kind, COMMAND_VCA_FADER_DB | COMMAND_VCA_MUTE)
            || self.rack != RACK_NOT_APPLICABLE
            || self.effect_index != 0
            || self.parameter_id != 0
            || self.values[1..].iter().any(|value| *value != 0.0)
        {
            return Err(COMMAND_REASON_MALFORMED);
        }
        let lanes = lane_selector(self.channel).ok_or(COMMAND_REASON_MALFORMED)?;
        let lane = control_lane(lanes);
        if self.kind == COMMAND_VCA_FADER_DB {
            check_value(&rows.vca_offset_db, lane, self.values[0])?;
            return Ok(VcaEdit::FaderDb(lanes, self.values[0]));
        }
        check_value(&rows.vca_mute, lane, self.values[0])?;
        Ok(VcaEdit::Mute(lanes, self.values[0] == 1.0))
    }

    /// Read one `solo` record's requested bit, or say why it cannot be read (issue #210 phase 1).
    ///
    /// Deliberately *not* an arm of [`Self::into_track_record`]: a solo record lowers to no record
    /// of its own. It moves live-control state, and the mute records that state composes to are
    /// staged once, coalesced, at the end of the submission's first pass. The shape rules are
    /// `mute`'s, with `channel = 255` because solo addresses a strip and not a lane, and the same
    /// `DOMAIN`-for-a-non-boolean rule `mute` uses for `values[0]`.
    fn into_solo_request(self, rows: &WebControlRows) -> Result<bool, u32> {
        if self.rack != RACK_NOT_APPLICABLE || self.channel != 255 {
            return Err(COMMAND_REASON_MALFORMED);
        }
        if self.values[1] != 0.0 || self.values[2] != 0.0 || self.values[3] != 0.0 {
            return Err(COMMAND_REASON_MALFORMED);
        }
        check_value(&rows.solo, Lane::Both, self.values[0])?;
        Ok(self.values[0] == 1.0)
    }

    /// Lower one effect-addressed record against the addressed effect's own descriptor.
    ///
    /// Writes one or two records into `out` and returns how many: a `channel = both` command on a
    /// parameter whose policy is `PerLane` lowers to **one record per lane**, because every launch
    /// effect counts a policy-violating span as `invalid_spans` rather than applying it. Doing the
    /// expansion here -- on the control plane, once per submission -- is what keeps the render
    /// side's drain a pure one-record-one-span map.
    /// Lower one observation subscribe/unsubscribe against the addressed effect's declared menu.
    ///
    /// `parameter_id` carries the effect-local `tap_id` and is translated to the `tap_index` the
    /// render side arms — an admission-time lookup, off the render thread, exactly as a parameter
    /// id is. A tap the descriptor does not declare is [`COMMAND_REASON_UNKNOWN_TAP`] and never
    /// [`COMMAND_REASON_UNKNOWN_PARAMETER`]: they are different namespaces on one effect, and a
    /// caller that confuses them learns which one it got wrong.
    fn into_observe_record(
        self,
        descriptor: &'static effect_contract::EffectDescriptor,
        observed: bool,
    ) -> Result<AdmittedCommand, u32> {
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
        Ok(AdmittedCommand::Effect(EffectControlRecord::Observe {
            tap_index,
            armed: self.kind == COMMAND_OBSERVE_SUBSCRIBE,
            window_blocks: self.smoothing_samples,
        }))
    }

    fn into_effect_records(
        self,
        descriptor: &'static effect_contract::EffectDescriptor,
        out: &mut [AdmittedCommand; 2],
    ) -> Result<usize, u32> {
        if self.kind == COMMAND_EFFECT_BYPASS {
            if self.channel != 255
                || self.parameter_id != 0
                || self.smoothing_samples != 0
                || self.values[1..].iter().any(|value| *value != 0.0)
            {
                return Err(COMMAND_REASON_MALFORMED);
            }
            if self.values[0] != 0.0 && self.values[0] != 1.0 {
                return Err(COMMAND_REASON_DOMAIN);
            }
            out[0] = AdmittedCommand::Effect(EffectControlRecord::Bypass(self.values[0] == 1.0));
            return Ok(1);
        }
        if self.channel > 2 || self.values[1..].iter().any(|value| *value != 0.0) {
            return Err(COMMAND_REASON_MALFORMED);
        }
        if self.parameter_id == 0 {
            return Err(COMMAND_REASON_UNKNOWN_PARAMETER);
        }
        let Some((parameter_index, parameter)) = descriptor
            .parameters
            .iter()
            .enumerate()
            .find(|(_, parameter)| parameter.id.0 == self.parameter_id)
        else {
            return Err(COMMAND_REASON_UNKNOWN_PARAMETER);
        };
        // A parameter the descriptor says cannot be automated at the block rate has no live cell
        // (#1345 D1: one cell per `Block`-rate parameter), so it has no write path at all. No
        // launch effect declares one, which is why this is `UNSUPPORTED_KIND` and not `DOMAIN`.
        if !parameter.automatable
            || parameter.automation_rate != effect_contract::AutomationRate::Block
        {
            return Err(COMMAND_REASON_UNSUPPORTED_KIND);
        }
        if !parameter_value_valid(parameter, self.values[0]) {
            return Err(COMMAND_REASON_DOMAIN);
        }
        let parameter_index =
            u32::try_from(parameter_index).map_err(|_| COMMAND_REASON_MALFORMED)?;
        let record = |channel: ParameterChannel| {
            AdmittedCommand::Effect(EffectControlRecord::Parameter {
                parameter_index,
                channel,
                value: self.values[0],
            })
        };
        match (parameter.channel_policy, self.channel) {
            // A shared parameter is addressed as "both" and by nothing else.
            (ParameterChannelPolicy::Shared, 2) => {
                out[0] = record(ParameterChannel::Both);
                Ok(1)
            }
            (ParameterChannelPolicy::Shared, _) => Err(COMMAND_REASON_MALFORMED),
            (ParameterChannelPolicy::PerLane, 0) => {
                out[0] = record(ParameterChannel::Left);
                Ok(1)
            }
            (ParameterChannelPolicy::PerLane, 1) => {
                out[0] = record(ParameterChannel::Right);
                Ok(1)
            }
            (ParameterChannelPolicy::PerLane, _) => {
                out[0] = record(ParameterChannel::Left);
                out[1] = record(ParameterChannel::Right);
                Ok(2)
            }
        }
    }
}

/// The wire `channel` byte as a builtin lane selector; `None` for a byte the ABI does not define.
const fn lane_selector(channel: u8) -> Option<BuiltinLaneSelector> {
    match channel {
        0 => Some(BuiltinLaneSelector::Left),
        1 => Some(BuiltinLaneSelector::Right),
        2 => Some(BuiltinLaneSelector::Both),
        _ => None,
    }
}

/// Validate a whole staged submission, then admit it. Nothing is pushed unless everything passes.
///
/// # The three passes are the contract, not an optimisation (#137 D1, extended by #140 C)
///
/// Pass one decodes and lowers every record and counts the room each *destination queue* needs.
/// Pass two checks that room against every one of those queues. Pass three pushes. A submission is
/// therefore all-or-nothing across every kind in it: a batch that moves a matrix, a fader and two
/// effect parameters is admitted whole or refused whole, and the refusal names the first record
/// that broke a rule.
///
/// One wire record can lower to two admitted records (`channel = both` on a per-lane effect
/// parameter), and one submission that touches solo owes up to two *more* per strip (issue #210
/// phase 1; per strip since issue #1213), and one that changes a strip's effective mute owes one
/// more per live send that follows it (issue #1224), and one that rides a VCA owes up to two more
/// per strip a VCA reaches (issue #1245), which is why `command_decoded` is
/// `2 * MAXIMUM_COMMAND_RECORDS + 2 * strip_count + route_count + 2 * vca_reached_strips` long and
/// why the room counted is per lowered record rather than per wire record.
///
/// # The solo transaction (issue #210 phase 1)
///
/// Pass one now also mutates live-control solo state -- the solo bits, the user-mute mirror, the
/// emitted-mute mirror -- while it is still deciding whether the submission is admissible at all.
/// So the state carries its own shadow and this wrapper is where it is closed: `commit` once pass
/// three has actually pushed, `rollback` on every refusal. A refused submission leaves host state
/// exactly as it was, which is the same all-or-nothing contract the queues keep. The live-send
/// mirror (issue #1222) and the live VCA state (issue #1245) are closed the same way, together.
fn admit_commands(
    ready: &mut ReadyOwnership,
    bytes: &[u8],
    count: usize,
    applied_at_sample: u64,
    host_generation: u64,
    prepared_submission: bool,
    companion: Option<&[u8]>,
) -> Result<(), CommandRejection> {
    for shadow in &mut ready.input_filter_shadows {
        shadow.begin();
    }
    match admit_commands_staged(
        ready,
        bytes,
        count,
        applied_at_sample,
        host_generation,
        prepared_submission,
        companion,
    ) {
        Ok(()) => {
            ready.solo.commit();
            ready.routes.commit();
            ready.vcas.commit();
            ready.strip_values.commit();
            Ok(())
        }
        Err(rejection) => {
            ready.solo.rollback();
            ready.routes.rollback();
            ready.vcas.rollback();
            ready.strip_values.rollback();
            for shadow in &mut ready.input_filter_shadows {
                shadow.rollback();
            }
            if rejection.result != RESULT_INTERNAL {
                for producer in ready.effect_controls.iter_mut().flatten() {
                    let _ = producer.discard_owner();
                }
            }
            Err(rejection)
        }
    }
}

/// Whether `staged` writes a cell: a fader, mute, matrix/pan, input trim or polarity record, a
/// send's target, an effect's parameter or bypass value, a prepared builtin input-filter marker
/// with at least one target, or an EQ owner's marker (its targets are cells).
fn writes_strip_cells(staged: &StagedCommand) -> bool {
    match staged.kind {
        StagedCommandKind::Command(
            AdmittedCommand::Matrix(_)
            | AdmittedCommand::Fader(_)
            | AdmittedCommand::Input(_)
            | AdmittedCommand::Route(_),
        ) => true,
        StagedCommandKind::Command(AdmittedCommand::Effect(record)) => {
            !matches!(record, EffectControlRecord::Observe { .. })
        }
        StagedCommandKind::PreparedOwner(marker) => {
            marker.kind == PreparedOwnerKind::Eq || marker.target_count != 0
        }
    }
}

/// Write the admitted fader, mute, matrix/pan, input trim, polarity, prepared-filter, send and
/// effect values of one batch into the strips', sends' and effects' cells as one transaction
/// (#1312 D7, #1345 D5, #1346 D6, #1347 D4), then publish its revision on the plan's gate.
///
/// A batch that stages none takes no stamp and no revision. The stamp is the batch's last
/// fallible step: its refusal (a revision past the gate's ceiling) writes nothing and returns the
/// internal refusal. After the stamp nothing can fail: each step is an `expect` on an invariant the
/// staging pass checked (root's D7 ruling), so a refused batch's values can never be published
/// and an open stamp never outlives the batch.
fn write_strip_cells(
    ready: &mut ReadyOwnership,
    lowered: usize,
    strip_count: usize,
    companion: Option<&PreparedCompanion<'_>>,
) -> Result<(), CommandRejection> {
    let Some(first_wire_index) = ready.command_decoded[..lowered]
        .iter()
        .find(|staged| writes_strip_cells(staged))
        .map(|staged| staged.original_wire_index)
    else {
        return Ok(());
    };
    let revision = ready.plan_gate.last_published().saturating_add(1);
    let Ok(stamp) = ready.plan_gate.stamp(revision) else {
        return Err(CommandRejection {
            result: RESULT_INTERNAL,
            reason: COMMAND_REASON_BACKPRESSURE,
            index: first_wire_index,
        });
    };
    // Every step from here to the publication cannot fail (root's D7 ruling): the staging pass
    // gave each cell record a strip index below `strip_count`, and `controls` holds one producer
    // set per strip. A violation is a bug, not a refusal: it must not leave a stamp open or
    // publish a half-written batch under a refusal.
    for entry in 0..lowered {
        let staged = ready.command_decoded[entry];
        let slot = staged.queue_slot as usize;
        match staged.kind {
            StagedCommandKind::Command(AdmittedCommand::Matrix(record)) => {
                ready
                    .controls
                    .get_mut(slot)
                    .expect("a staged matrix record addresses a strip with a cell producer")
                    .producer
                    .write(stamp, record);
            }
            StagedCommandKind::Command(AdmittedCommand::Fader(record)) => {
                ready
                    .controls
                    .get_mut(
                        slot.checked_sub(strip_count)
                            .expect("a staged fader record sits in the fader band"),
                    )
                    .expect("a staged fader record addresses a strip with a cell producer")
                    .fader
                    .write(stamp, record);
            }
            StagedCommandKind::Command(AdmittedCommand::Route(record)) => {
                // The send band follows the effects; the staging pass resolved the route index.
                ready
                    .route_controls
                    .get_mut(
                        slot.checked_sub(strip_count * 3 + ready.effect_controls.len())
                            .expect("a staged send record sits in the send band"),
                    )
                    .expect("a staged send record addresses a live route's cell producer")
                    .write(stamp, record);
            }
            StagedCommandKind::Command(AdmittedCommand::Input(record)) => {
                ready
                    .controls
                    .get_mut(
                        slot.checked_sub(strip_count * 2)
                            .expect("a staged input record sits in the input band"),
                    )
                    .and_then(|control| control.input.as_mut())
                    .expect("the room pass checked this strip's input cells")
                    .write(stamp, record);
            }
            StagedCommandKind::Command(AdmittedCommand::Effect(
                record @ (EffectControlRecord::Parameter { .. } | EffectControlRecord::Bypass(_)),
            )) => {
                ready
                    .effect_controls
                    .get_mut(
                        slot.checked_sub(strip_count * 3)
                            .expect("a staged effect record sits in the effect band"),
                    )
                    .and_then(Option::as_mut)
                    .expect("a staged effect record addresses a prepared effect producer")
                    .write(stamp, record)
                    .expect("the staging pass preflighted this effect record");
            }
            StagedCommandKind::PreparedOwner(marker) if marker.kind == PreparedOwnerKind::Eq => {
                let effect = slot
                    .checked_sub(strip_count * 3)
                    .expect("an Eq owner marker sits in the effect band");
                let mut targets = [effect_contract::PreparedEffectTarget {
                    slot: 0,
                    channel: effect_contract::ParameterChannel::Both,
                    words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
                }; host_core::EQ_TARGET_CAPACITY];
                let mut target_count = 0_usize;
                if let Some(companion) = companion {
                    for index in 0..companion.target_count {
                        let target = companion_target(companion, index)
                            .expect("the companion's targets were decoded while staging");
                        // `prepared_queue_address` for an EQ target: the effect band slot, which
                        // is this marker's exactly when the address resolves to it. Spelled out
                        // because the open stamp borrows the plan's gate, and that function
                        // borrows all of `ready`.
                        let addressed = target.rack != RACK_NOT_APPLICABLE
                            && live_effect_address(u32::from(target.rack), target.effect_index)
                                .and_then(|address| {
                                    let track = target.track_index as usize;
                                    let addressed = dense_effect_slot(
                                        *ready.effect_base.get(track)?,
                                        *ready.rack_effects.get(track)?,
                                        address,
                                    )?;
                                    (addressed < ready.effect_controls.len()).then_some(addressed)
                                })
                                .is_some_and(|addressed| strip_count * 3 + addressed == slot);
                        if addressed {
                            targets[target_count] = target.target;
                            target_count += 1;
                        }
                    }
                }
                assert_eq!(
                    target_count, marker.target_count as usize,
                    "the targets written are the targets staged"
                );
                ready
                    .effect_controls
                    .get_mut(effect)
                    .and_then(Option::as_mut)
                    .expect("an Eq owner marker addresses a prepared effect producer")
                    .publish_candidate_targets(
                        stamp,
                        marker.base_revision,
                        &targets[..target_count],
                    )
                    .expect("the owner's targets were preflighted");
            }
            StagedCommandKind::PreparedOwner(marker)
                if marker.kind == PreparedOwnerKind::BuiltinInput =>
            {
                let track = slot
                    .checked_sub(strip_count * 2)
                    .expect("a builtin input owner marker sits in the input band");
                let mut written = 0_usize;
                if let Some(companion) = companion {
                    for index in 0..companion.target_count {
                        let target = companion_target(companion, index)
                            .expect("the companion's targets were decoded while staging");
                        // `prepared_queue_address` for a builtin input target: the strip's input
                        // band slot, which is this marker's exactly when the strip index is. It is
                        // spelled out because the open stamp borrows the plan's gate, and that
                        // function borrows all of `ready`.
                        let addressed = target.rack == RACK_NOT_APPLICABLE
                            && target.effect_index == 0
                            && target.track_index as usize == track;
                        if addressed {
                            let target = companion_builtin_target(target)
                                .expect("the builtin target was validated while staging");
                            ready
                                .controls
                                .get_mut(track)
                                .and_then(|control| control.input.as_mut())
                                .expect("the room pass checked this strip's input cells")
                                .write(stamp, TrackInputRecord::PreparedFilter { target });
                            written += 1;
                        }
                    }
                }
                assert_eq!(
                    written, marker.target_count as usize,
                    "the targets written are the targets staged"
                );
            }
            _ => {}
        }
    }
    ready.plan_gate.publish(revision);
    Ok(())
}

/// The lanes of `slot`'s cell that an earlier command of this batch already staged a record for
/// (#1560). `select` names the lanes one staged record covers, `None` for a record of another
/// kind.
///
/// A command that repeats the committed value stages nothing, but a repeat that follows an earlier
/// command of the same batch on the same cell still stages its record: the cell keeps the last
/// record, so the repeat's ramp length replaces the earlier one's, as it does for the core, whose
/// transaction takes the last edit's length. Only the first write of a cell in a batch can be a
/// no-op.
fn staged_lanes(
    ready: &ReadyOwnership,
    lowered: usize,
    slot: usize,
    select: impl Fn(&AdmittedCommand) -> Option<BuiltinLaneSelector>,
) -> [bool; 2] {
    let mut lanes = [false; 2];
    for staged in &ready.command_decoded[..lowered] {
        if staged.queue_slot as usize != slot {
            continue;
        }
        if let StagedCommandKind::Command(record) = &staged.kind
            && let Some(selector) = select(record)
        {
            for (lane, seen) in lanes.iter_mut().enumerate() {
                *seen |= covers_lane(selector, lane);
            }
        }
    }
    lanes
}

/// The three passes themselves. Every exit from here is a transaction boundary for its caller.
fn admit_commands_staged(
    ready: &mut ReadyOwnership,
    bytes: &[u8],
    count: usize,
    applied_at_sample: u64,
    host_generation: u64,
    prepared_submission: bool,
    companion: Option<&[u8]>,
) -> Result<(), CommandRejection> {
    let record_bytes = COMMAND_RECORD_BYTES as usize;
    // Issue #1213 D1: the index word is a strip index, tracks first, then submixes, and every band
    // below is per strip. With `S = 0` this is the track count, so every slot is unmoved.
    let strip_count = ready.strip_count();
    let companion = match companion {
        Some(bytes) => match parse_prepared_companion(bytes, host_generation) {
            Ok(companion) => Some(companion),
            Err(()) => {
                return Err(CommandRejection {
                    result: RESULT_INVALID_ARGUMENT,
                    reason: COMMAND_REASON_MALFORMED,
                    index: 0,
                });
            }
        },
        None => None,
    };
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
    let mut lowered = 0_usize;
    // Issue #210 phase 1: solo state changes are applied as they are read, but the mute records
    // they compose to are staged once, after the whole batch, by the coalescing pass below.
    let mut solo_seen = false;
    // Issue #1245 D3: a VCA mute (kind 17) moves the same composition, so it runs the same pass.
    let mut vca_mute_seen = false;
    // The coalesced mute records answer to the batch's first kind 9 or 17 record and take the
    // last one's ramp.
    let mut coalesce_first_wire_index = 0_u32;
    let mut coalesce_smoothing = 0_u32;
    // Issue #1224 D3: whether any strip mute record was staged, so the follow pass below runs.
    let mut mute_seen = false;
    // Decision 17 C7 (batch 1 review MINOR 1): `controls::any_solo` walks every strip, so it runs
    // once per batch, on first use, and again only after a kind 9 record moved a solo bit.
    let mut any_solo_cache: Option<bool> = None;
    // Issue #1245 D3: a VCA ride (kind 16) stages nothing itself; the VCA fader pass below stages
    // every reached member's changed lanes once, at the first kind 16 record's wire index, with
    // the last one's ramp.
    let mut vca_fader_seen = false;
    let mut vca_fader_first_wire_index = 0_u32;
    let mut vca_fader_smoothing = 0_u32;
    for index in 0..count {
        let record = &bytes[index * record_bytes..(index + 1) * record_bytes];
        let command = CommandRecord::decode(record)
            .map_err(|reason| refuse(reason, index))?
            .with_session_ramp(&ready.ramps);
        let track = command.track_index as usize;
        // Issue #1222 D2: the index word is read by kind. A send kind's is a live-route index,
        // checked against the live routes in its own arm below, and a VCA kind's a VCA index
        // (issue #1245 D1), checked against the VCAs in its own arm; every other kind's is a
        // strip index, checked here against the strip count. One generic check ahead of the
        // dispatch would refuse a valid send or VCA at or past the strip count, and pass an
        // invalid one below it.
        let strip_addressed = !matches!(
            command.kind,
            COMMAND_ROUTE_GAIN_DB
                | COMMAND_ROUTE_MUTE
                | COMMAND_ROUTE_MATRIX
                | COMMAND_VCA_FADER_DB
                | COMMAND_VCA_MUTE
        );
        if strip_addressed && track >= strip_count {
            return Err(refuse(COMMAND_REASON_UNKNOWN_TRACK, index));
        }
        let mut staged = [AdmittedCommand::Effect(EffectControlRecord::Bypass(false)); 2];
        let (slot, produced) = match command.kind {
            // The three kinds that lower to one record on one of the three per-track bands. The
            // band is read off the lowered variant rather than off the kind, so a kind added to
            // this arm cannot land in the wrong queue by forgetting a second table.
            COMMAND_PAN
            | COMMAND_MATRIX
            | COMMAND_FADER_DB
            | COMMAND_TRIM_DB
            | COMMAND_POLARITY_INVERT => {
                staged[0] = command
                    .into_track_record(&ready.rows)
                    .map_err(|reason| refuse(reason, index))?;
                let slot = match staged[0] {
                    AdmittedCommand::Fader(_) => strip_count + track,
                    AdmittedCommand::Input(_) => strip_count * 2 + track,
                    _ => track,
                };
                if ready.controls.get(track).is_none() {
                    return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, index));
                }
                // Issue #1560: a command that repeats the value the render plane holds stages no
                // record, as the core's classifier writes none for an equal value: the record
                // would restart the cell's ramp over its own length, so a ramp still in flight
                // would render other bits. The comparison is the core's, by the bits it compares
                // (`LiveStripValues`), and the mirror moves with each record staged.
                match staged[0] {
                    AdmittedCommand::Matrix(record) => {
                        let matrix = record.matrix;
                        let bits = [matrix.ll, matrix.lr, matrix.rl, matrix.rr].map(f32::to_bits);
                        let held = ready.strip_values.get(track).map(|held| held.matrix);
                        let staged_before = staged_lanes(ready, lowered, slot, |record| {
                            matches!(record, AdmittedCommand::Matrix(_))
                                .then_some(BuiltinLaneSelector::Both)
                        });
                        if held == Some(bits) && !staged_before[0] {
                            (slot, 0)
                        } else {
                            ready.strip_values.set_matrix(track, bits);
                            (slot, 1)
                        }
                    }
                    // Issue #1245 D3: a fader move on a strip some VCA reaches moves the member's
                    // own value, and what reaches the queue is its effective value -- the own
                    // value plus every reaching VCA's offset, clamped -- for the covered lanes. Two
                    // different lane values split a `Both` command into one `Left` and one `Right`
                    // record. A strip no VCA reaches (`set_member_db` refuses it) lowers its own
                    // value as is.
                    //
                    // Decision 17 C7, slice S7c D5: the effective value is
                    // `controls::effective_strip`'s. The fader does not read the solo term, so the
                    // `any_solo` it is given is moot.
                    AdmittedCommand::Fader(TrackFaderRecord::FaderDb {
                        lanes,
                        db,
                        smoothing_samples,
                    }) => {
                        let member = ready.vcas.set_member_db(track, lanes, db);
                        let target = if member {
                            WebEffectiveView::new(&ready.vcas, &ready.solo, &ready.routes)
                                .strip(track, false)
                                .fader_db
                        } else {
                            [db, db]
                        };
                        // What the render plane holds: a reached strip's effective value is the
                        // VCA state's, any other strip's is the own value kept in the mirror.
                        let held = match ready.vcas.emitted_db(track) {
                            Some(emitted) => Some(emitted.map(fader_gain_bits)),
                            None => ready.strip_values.get(track).map(|held| held.fader_gain),
                        };
                        let staged_before =
                            staged_lanes(ready, lowered, slot, |record| match record {
                                AdmittedCommand::Fader(TrackFaderRecord::FaderDb {
                                    lanes, ..
                                }) => Some(*lanes),
                                _ => None,
                            });
                        let changed = [0, 1].map(|lane| {
                            covers_lane(lanes, lane)
                                && (staged_before[lane]
                                    || held.is_none_or(|held| {
                                        fader_gain_bits(target[lane]) != held[lane]
                                    }))
                        });
                        let mut produced = 0;
                        for (lane_selector, lane) in changed_lane_records(
                            changed,
                            target[0].to_bits() == target[1].to_bits(),
                        ) {
                            let db = target[lane];
                            if member {
                                ready.vcas.record_emitted_db(track, lane_selector, db);
                            } else {
                                ready.strip_values.set_fader_gain(
                                    track,
                                    lane_selector,
                                    fader_gain_bits(db),
                                );
                            }
                            staged[produced] = AdmittedCommand::Fader(TrackFaderRecord::FaderDb {
                                lanes: lane_selector,
                                db,
                                smoothing_samples,
                            });
                            produced += 1;
                        }
                        (slot, produced)
                    }
                    // The input cells are the strip's, whatever the value: a strip without them
                    // refuses a trim or a polarity whether or not it would change anything.
                    AdmittedCommand::Input(record) => {
                        if ready
                            .controls
                            .get(track)
                            .and_then(|control| control.input.as_ref())
                            .is_none()
                        {
                            return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, index));
                        }
                        let held = ready.strip_values.get(track).copied();
                        let mut produced = 0;
                        match record {
                            TrackInputRecord::TrimDb {
                                lanes,
                                db,
                                smoothing_samples,
                            } => {
                                let bits = trim_gain_bits(db);
                                let staged_before =
                                    staged_lanes(ready, lowered, slot, |record| match record {
                                        AdmittedCommand::Input(TrackInputRecord::TrimDb {
                                            lanes,
                                            ..
                                        }) => Some(*lanes),
                                        _ => None,
                                    });
                                let changed = [0, 1].map(|lane| {
                                    covers_lane(lanes, lane)
                                        && (staged_before[lane]
                                            || held.is_none_or(|held| held.trim_gain[lane] != bits))
                                });
                                for (lane_selector, _) in changed_lane_records(changed, true) {
                                    ready.strip_values.set_trim_gain(track, lane_selector, bits);
                                    staged[produced] =
                                        AdmittedCommand::Input(TrackInputRecord::TrimDb {
                                            lanes: lane_selector,
                                            db,
                                            smoothing_samples,
                                        });
                                    produced += 1;
                                }
                            }
                            TrackInputRecord::PolarityInvert {
                                lanes,
                                inverted,
                                smoothing_samples,
                            } => {
                                let staged_before =
                                    staged_lanes(ready, lowered, slot, |record| match record {
                                        AdmittedCommand::Input(
                                            TrackInputRecord::PolarityInvert { lanes, .. },
                                        ) => Some(*lanes),
                                        _ => None,
                                    });
                                let changed = [0, 1].map(|lane| {
                                    covers_lane(lanes, lane)
                                        && (staged_before[lane]
                                            || held
                                                .is_none_or(|held| held.polarity[lane] != inverted))
                                });
                                for (lane_selector, _) in changed_lane_records(changed, true) {
                                    ready
                                        .strip_values
                                        .set_polarity(track, lane_selector, inverted);
                                    staged[produced] =
                                        AdmittedCommand::Input(TrackInputRecord::PolarityInvert {
                                            lanes: lane_selector,
                                            inverted,
                                            smoothing_samples,
                                        });
                                    produced += 1;
                                }
                            }
                            TrackInputRecord::PreparedFilter { .. } => {
                                return Err(refuse(COMMAND_REASON_MALFORMED, index));
                            }
                        }
                        (slot, produced)
                    }
                    _ => (slot, 1),
                }
            }
            // A mute command carries the user's *intent*; what reaches the queue is the composed
            // effective mute. With no solo engaged the two are the same value and this stages
            // byte-for-byte what it staged before solo existed. Every lane a selector covers
            // shares one strip-scoped solo term; only a one-lane VCA mute splits a `Both` command
            // into two per-lane records (below).
            //
            // Issue #1213 D3 (VERIFY-2 M4): the effective value is read from the core's one
            // composition (`controls::effective_strip`), which knows a submix is solo-safe; it is
            // never spelled again here. An
            // inline `muted || (any_solo && !solo)` would stage `muted: true` for a bus *unmute*
            // under any track solo.
            COMMAND_MUTE => {
                let lowered_record = command
                    .into_track_record(&ready.rows)
                    .map_err(|reason| refuse(reason, index))?;
                if ready.controls.get(track).is_none() {
                    return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, index));
                }
                let AdmittedCommand::Fader(TrackFaderRecord::Mute {
                    lanes,
                    muted,
                    smoothing_samples,
                }) = lowered_record
                else {
                    return Err(refuse(COMMAND_REASON_MALFORMED, index));
                };
                if !ready.solo.set_user_mute(track, lanes, muted) {
                    return Err(refuse(COMMAND_REASON_UNKNOWN_TRACK, index));
                }
                // After `set_user_mute` every covered lane holds `muted` and the solo term is per
                // strip, but a VCA mute (issue #1242 D3) is per lane: a `Both` command on a strip
                // whose VCA mutes one lane composes to two different lane values. Then it stages
                // one `Left` and one `Right` record, each with its own lane's effective mute, and
                // mirrors each lane separately. When the covered lanes agree -- always, for a
                // session without VCAs -- it stages the one record it staged before.
                mute_seen = true;
                //
                // Decision 17 C7, slice S7c D5: the effective mute is `controls::effective_strip`'s,
                // read over the mirrors as they stand now, so an earlier kind 17 or kind 9 in this
                // batch is already in it.
                let [left, right] = {
                    let view = WebEffectiveView::new(&ready.vcas, &ready.solo, &ready.routes);
                    view.strip(track, view.any_solo(&mut any_solo_cache)).mute
                };
                // Issue #1560: only a covered lane whose effective mute differs from what the render
                // plane was last told stages a record, as the core's classifier does: re-muting a
                // settled lane restarts its ramp. A `Both` command whose lanes both change to one
                // value stages the one record it always did; one lane changing stages that lane's.
                let effective = [left, right];
                let staged_before =
                    staged_lanes(ready, lowered, strip_count + track, |record| match record {
                        AdmittedCommand::Fader(TrackFaderRecord::Mute { lanes, .. }) => {
                            Some(*lanes)
                        }
                        _ => None,
                    });
                let changed = [0, 1].map(|lane| {
                    covers_lane(lanes, lane)
                        && (staged_before[lane]
                            || effective[lane] != ready.solo.emitted_mute(track, lane))
                });
                let mut produced = 0;
                for (lane_selector, lane) in changed_lane_records(changed, left == right) {
                    ready
                        .solo
                        .record_emitted(track, lane_selector, effective[lane]);
                    staged[produced] = AdmittedCommand::Fader(TrackFaderRecord::Mute {
                        lanes: lane_selector,
                        muted: effective[lane],
                        smoothing_samples,
                    });
                    produced += 1;
                }
                (strip_count + track, produced)
            }
            // Solo lowers to nothing here. It moves one live-control bit; the records that bit
            // composes to are the coalescing pass's business, because a batch of alternating
            // toggles would otherwise fan out up to `2 * strip_count` records *per transition*.
            COMMAND_SOLO => {
                let engaged = command
                    .into_solo_request(&ready.rows)
                    .map_err(|reason| refuse(reason, index))?;
                if ready.controls.get(track).is_none() {
                    return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, index));
                }
                // Issue #1213 D4: solo addresses tracks only. A submix is solo-safe, so a solo at
                // its strip index is refused, at this record's wire index, before any state moves.
                if ready.solo.solo_safe(track) {
                    return Err(refuse(COMMAND_REASON_NOT_SOLOABLE, index));
                }
                if !ready.solo.set_solo(track, engaged) {
                    return Err(refuse(COMMAND_REASON_UNKNOWN_TRACK, index));
                }
                any_solo_cache = None;
                if !solo_seen && !vca_mute_seen {
                    coalesce_first_wire_index = index as u32;
                }
                solo_seen = true;
                coalesce_smoothing = command.smoothing_samples;
                (strip_count + track, 0)
            }
            // Issue #1245 D3: a VCA kind moves the VCA state and stages nothing itself. A ride's
            // member fader records are the VCA fader pass's business, and a mute's member mute
            // records the coalescing pass's, so a batch of several moves stages each member's net
            // change once. The mute term reaches the strip-mute owner at once, so a later kind 4
            // in the same batch composes with it.
            COMMAND_VCA_FADER_DB | COMMAND_VCA_MUTE => {
                // No live commands: the app did not opt in, so the session has no write path for
                // any member.
                if !ready.commands_enabled {
                    return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, index));
                }
                if track >= ready.vcas.vca_count() {
                    return Err(refuse(COMMAND_REASON_UNKNOWN_VCA, index));
                }
                match command
                    .into_vca_edit(&ready.rows)
                    .map_err(|reason| refuse(reason, index))?
                {
                    VcaEdit::FaderDb(lanes, db) => {
                        if !ready.vcas.set_vca_db(track, lanes, db) {
                            return Err(refuse(COMMAND_REASON_UNKNOWN_VCA, index));
                        }
                        if !vca_fader_seen {
                            vca_fader_first_wire_index = index as u32;
                        }
                        vca_fader_seen = true;
                        vca_fader_smoothing = command.smoothing_samples;
                    }
                    VcaEdit::Mute(lanes, muted) => {
                        if !ready.vcas.set_vca_mute(track, lanes, muted) {
                            return Err(refuse(COMMAND_REASON_UNKNOWN_VCA, index));
                        }
                        // The strip-mute owner's VCA term is refreshed lazily (amendment A1): for
                        // the strip a later kind 4 addresses, before it composes, and for every
                        // reached strip once after the loop. Refreshing every reached member here
                        // would cost one pass over the reach pairs per record.
                        if !solo_seen && !vca_mute_seen {
                            coalesce_first_wire_index = index as u32;
                        }
                        vca_mute_seen = true;
                        coalesce_smoothing = command.smoothing_samples;
                    }
                }
                (0, 0)
            }
            // Issue #1222 D4: a send kind moves one field of its mirror and stages one record
            // built from the whole mirror, through the prepared route's own coefficient function.
            // The mirror is touched only once the record is built, and every later record in the
            // batch reads it, so several edits of one send stage one record each, in wire order;
            // the send's cell holds the last, and render applies only that one and counts the
            // others as superseded.
            COMMAND_ROUTE_GAIN_DB | COMMAND_ROUTE_MUTE | COMMAND_ROUTE_MATRIX => {
                // No live commands: the app did not opt in, so the session has no write path for
                // any send.
                if !ready.commands_enabled {
                    return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, index));
                }
                let (Some(slot), Some(producer), Some(&current)) = (
                    ready.route_slot(track),
                    ready.route_controls.get(track),
                    ready.routes.get(track),
                ) else {
                    return Err(refuse(COMMAND_REASON_UNKNOWN_ROUTE, index));
                };
                let edit = command
                    .into_route_edit(&ready.rows)
                    .map_err(|reason| refuse(reason, index))?;
                let mut next = current;
                match edit {
                    RouteEdit::GainDb(gain_db) => next.gain_db = gain_db,
                    RouteEdit::Mute(mute) => next.mute = mute,
                    RouteEdit::Matrix(matrix) => next.matrix = matrix,
                }
                let record = producer
                    .record(
                        next.gain_db,
                        next.matrix,
                        next.mute,
                        next.source_lane_muted,
                        command.smoothing_samples,
                    )
                    .map_err(|error| match error {
                        RouteControlError::Domain | RouteControlError::Length => {
                            refuse(COMMAND_REASON_DOMAIN, index)
                        }
                    })?;
                // Issue #1560: the mirror holds the committed values, so it moves whether or not a
                // record follows. A record is staged only when the target the render plane holds
                // changes -- the folded coefficients or the silenced flag, the core's comparison
                // (`route_records`): a repeated value, or a gain or matrix edit on a send that stays
                // silenced, stages nothing, because the record would restart the send's ramp.
                let held = producer
                    .record(
                        current.gain_db,
                        current.matrix,
                        current.mute,
                        current.source_lane_muted,
                        command.smoothing_samples,
                    )
                    .map_err(|_| refuse(COMMAND_REASON_DOMAIN, index))?;
                let moved = match edit {
                    RouteEdit::GainDb(gain_db) => ready.routes.set_gain_db(track, gain_db),
                    RouteEdit::Mute(mute) => ready.routes.set_mute(track, mute),
                    RouteEdit::Matrix(matrix) => ready.routes.set_matrix(track, matrix),
                };
                if !moved {
                    return Err(refuse(COMMAND_REASON_UNKNOWN_ROUTE, index));
                }
                let bits = |record: &RouteControlRecord| record.target().map(f32::to_bits);
                let staged_before = staged_lanes(ready, lowered, slot, |record| {
                    matches!(record, AdmittedCommand::Route(_)).then_some(BuiltinLaneSelector::Both)
                })[0];
                if !staged_before && held.mute() == record.mute() && bits(&held) == bits(&record) {
                    (slot, 0)
                } else {
                    staged[0] = AdmittedCommand::Route(record);
                    (slot, 1)
                }
            }
            COMMAND_INPUT_FILTERS => {
                if !prepared_submission {
                    return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, index));
                }
                let edit = command
                    .into_input_filter_edit()
                    .map_err(|reason| refuse(reason, index))?;
                let Some(shadow) = ready.input_filter_shadows.get_mut(track) else {
                    return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, index));
                };
                shadow
                    .apply(ready.sample_rate_hz, edit)
                    .map_err(|reason| refuse(reason, index))?;
                let slot = builtin_input_slot(strip_count, track)
                    .ok_or_else(|| refuse(COMMAND_REASON_UNSUPPORTED_KIND, index))?;
                let owner_seen = ready.command_decoded[..lowered].iter().any(|entry| {
                    matches!(
                        entry.kind,
                        StagedCommandKind::PreparedOwner(owner)
                            if owner.kind == PreparedOwnerKind::BuiltinInput
                                && owner.queue_slot as usize == slot
                    )
                });
                if !owner_seen {
                    let base_revision = shadow.revision;
                    let marker = PreparedOwner {
                        kind: PreparedOwnerKind::BuiltinInput,
                        queue_slot: slot as u32,
                        base_revision,
                        first_wire_index: index as u32,
                        target_count: 0,
                    };
                    let Some(entry) = ready.command_decoded.get_mut(lowered) else {
                        return Err(refuse(COMMAND_REASON_MALFORMED, index));
                    };
                    *entry = StagedCommand {
                        queue_slot: marker.queue_slot,
                        original_wire_index: index as u32,
                        kind: StagedCommandKind::PreparedOwner(marker),
                    };
                    lowered += 1;
                }
                (slot, 0)
            }
            COMMAND_EFFECT_PARAM
            | COMMAND_EFFECT_BYPASS
            | COMMAND_OBSERVE_SUBSCRIBE
            | COMMAND_OBSERVE_UNSUBSCRIBE => {
                // `inserts` (1) and `console` (3); the retired `simd1` (0) and `simd2` (2) are
                // refused here and never reinterpreted (decision 12, issue #1096).
                let Some(address) =
                    live_effect_address(u32::from(command.rack), command.effect_index)
                else {
                    return Err(refuse(COMMAND_REASON_UNKNOWN_RACK, index));
                };
                if ready.rack_effects.get(track).is_none() {
                    return Err(refuse(COMMAND_REASON_UNKNOWN_TRACK, index));
                }
                let Some(effect) = ready.effect_slot(track, address) else {
                    return Err(refuse(COMMAND_REASON_UNKNOWN_EFFECT, index));
                };
                let (descriptor, is_eq, has_owner) = {
                    let Some(producer) = ready.effect_controls.get(effect).and_then(Option::as_ref)
                    else {
                        return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, index));
                    };
                    (
                        producer.descriptor,
                        producer.descriptor.id.as_str() == "miso.parametric-eq",
                        producer.has_owner(),
                    )
                };
                let produced = if matches!(
                    command.kind,
                    COMMAND_OBSERVE_SUBSCRIBE | COMMAND_OBSERVE_UNSUBSCRIBE
                ) {
                    let observed = ready
                        .effect_observations
                        .get(effect)
                        .is_some_and(Option::is_some);
                    staged[0] = command
                        .into_observe_record(descriptor, observed)
                        .map_err(|reason| refuse(reason, index))?;
                    // One submission names each tap once: a record overwrites its tap's whole
                    // state and the submission drains in one block, so the second record would
                    // be a supersession the engine must not make (D15-2: Observe records are
                    // never superseded) and would not fit the tap's queue slot. Refused here,
                    // at this record's wire index, before the room pass, any write or the stamp.
                    let AdmittedCommand::Effect(EffectControlRecord::Observe { tap_index, .. }) =
                        staged[0]
                    else {
                        return Err(refuse(COMMAND_REASON_MALFORMED, index));
                    };
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
                    1
                } else {
                    command
                        .into_effect_records(descriptor, &mut staged)
                        .map_err(|reason| refuse(reason, index))?
                };
                let mut produced = produced;
                if is_eq
                    && has_owner
                    && matches!(command.kind, COMMAND_EFFECT_PARAM)
                    && !prepared_submission
                {
                    return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, index));
                }
                if prepared_submission && matches!(command.kind, COMMAND_EFFECT_PARAM) && is_eq {
                    if !has_owner {
                        return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, index));
                    }
                    let owner_seen = ready.command_decoded[..lowered].iter().any(|entry| {
                        matches!(
                            entry.kind,
                            StagedCommandKind::PreparedOwner(owner)
                                if owner.queue_slot as usize == strip_count * 3 + effect
                        )
                    });
                    if !owner_seen {
                        let base_revision = ready
                            .effect_controls
                            .get(effect)
                            .and_then(Option::as_ref)
                            .and_then(|producer| {
                                producer.owner().map(|owner| owner.committed_revision())
                            })
                            .ok_or_else(|| refuse(COMMAND_REASON_UNSUPPORTED_KIND, index))?;
                        ready
                            .effect_controls
                            .get_mut(effect)
                            .and_then(Option::as_mut)
                            .ok_or_else(|| refuse(COMMAND_REASON_UNSUPPORTED_KIND, index))?
                            .begin_owner(base_revision)
                            .map_err(|error| refuse(owner_command_reason(error), index))?;
                        let marker = PreparedOwner {
                            kind: PreparedOwnerKind::Eq,
                            queue_slot: (strip_count * 3 + effect) as u32,
                            base_revision,
                            first_wire_index: index as u32,
                            target_count: 0,
                        };
                        let Some(entry) = ready.command_decoded.get_mut(lowered) else {
                            return Err(refuse(COMMAND_REASON_MALFORMED, index));
                        };
                        *entry = StagedCommand {
                            queue_slot: marker.queue_slot,
                            original_wire_index: index as u32,
                            kind: StagedCommandKind::PreparedOwner(marker),
                        };
                        lowered += 1;
                    }
                    for record in staged.iter().copied().take(produced) {
                        let AdmittedCommand::Effect(EffectControlRecord::Parameter {
                            parameter_index,
                            channel,
                            value,
                        }) = record
                        else {
                            return Err(refuse(COMMAND_REASON_MALFORMED, index));
                        };
                        ready
                            .effect_controls
                            .get_mut(effect)
                            .and_then(Option::as_mut)
                            .ok_or_else(|| refuse(COMMAND_REASON_UNSUPPORTED_KIND, index))?
                            .edit_owner(parameter_index, channel, value)
                            .map_err(|error| refuse(owner_command_reason(error), index))?;
                    }
                    produced = 0;
                }
                // Every original effect command is checked before the publication pass can
                // mutate a queue or the observation mirror.  Keep this tied to `index`: a
                // per-lane lowering may produce two records, but a refusal still names the
                // original wire command that caused it.
                let slot = strip_count * 3 + effect;
                for record in staged.iter().copied().take(produced) {
                    let AdmittedCommand::Effect(record) = record else {
                        return Err(refuse(COMMAND_REASON_MALFORMED, index));
                    };
                    if ready.preflight_effect(slot, record).is_err() {
                        return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, index));
                    }
                }
                (slot, produced)
            }
            _ => return Err(refuse(COMMAND_REASON_MALFORMED, index)),
        };
        let Some(wanted) = ready.command_wanted.get_mut(slot) else {
            return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, index));
        };
        // Only an observation subscription is a queue record; every other record is a cell
        // write and wants no room (#1345 D2).
        let queued = staged
            .iter()
            .take(produced)
            .filter(|record| {
                matches!(
                    record,
                    AdmittedCommand::Effect(EffectControlRecord::Observe { .. })
                )
            })
            .count();
        *wanted = wanted.saturating_add(queued as u32);
        for record in staged.into_iter().take(produced) {
            let Some(entry) = ready.command_decoded.get_mut(lowered) else {
                return Err(refuse(COMMAND_REASON_MALFORMED, index));
            };
            *entry = StagedCommand {
                queue_slot: slot as u32,
                original_wire_index: index as u32,
                kind: StagedCommandKind::Command(record),
            };
            lowered += 1;
        }
    }
    // Issue #1245 D3: the VCA fader pass. Every kind 16 and kind 3 in the batch has moved the VCA
    // state; what each reached member still owes the render plane is the difference between its
    // effective fader value and what the render plane was last told -- at most two records per
    // reached strip, and never a redundant one (a settled lane's retarget moves bits). It stages
    // them on each member's existing fader queue with the last kind 16 record's ramp, at the first
    // one's wire index, ahead of the mute records below.
    if vca_fader_seen {
        for strip in 0..strip_count {
            let slot = strip_count + strip;
            if !ready.vcas.reaches(strip) {
                continue;
            }
            let fader_db = WebEffectiveView::new(&ready.vcas, &ready.solo, &ready.routes)
                .strip(strip, false)
                .fader_db;
            for (lanes, db) in ready
                .vcas
                .fader_delta(strip, fader_db)
                .into_iter()
                .flatten()
            {
                let Some(entry) = ready.command_decoded.get_mut(lowered) else {
                    return Err(refuse(COMMAND_REASON_MALFORMED, count.saturating_sub(1)));
                };
                *entry = StagedCommand {
                    queue_slot: slot as u32,
                    original_wire_index: vca_fader_first_wire_index,
                    kind: StagedCommandKind::Command(AdmittedCommand::Fader(
                        TrackFaderRecord::FaderDb {
                            lanes,
                            db,
                            smoothing_samples: vca_fader_smoothing,
                        },
                    )),
                };
                lowered += 1;
                ready.vcas.record_emitted_db(strip, lanes, db);
            }
        }
    }
    // The coalesced net emission (issue #210 phase 1, correction 1). Every solo and mute change in
    // the batch has been applied; what the live controls owe the render plane is now the difference
    // between the composed effective mute and what the render plane was last told -- at most two
    // records per strip, and **never** a redundant one. That last clause is load-bearing for bit
    // identity, not an optimisation: re-muting an already-settled-muted lane with a nonzero
    // smoothing window re-enters the ramp kernel and turns an exact `+0.0` into `-0.0` for a
    // negative input. It runs only for a batch that actually moved a solo bit; without one, every
    // mute command already staged its own record and the difference is empty by construction.
    //
    // The fade is the last solo record's `smoothing_samples`: a batch is one gesture, and the
    // gesture that moved the solo state is the one whose declick window the live controls asked
    // for. A VCA mute (kind 17, issue #1245 D3) moves the same composition and runs the same pass:
    // the ramp is the last kind 9 or 17 record's, and the records answer to the first one's wire
    // index.
    if solo_seen || vca_mute_seen {
        // Decision 17 C7, slice S7c D5: every strip's effective mute is the core's, read over the
        // mirrors as the batch left them (a kind 17 already moved the VCA state), with the solo
        // term computed once for the pass. The pass writes only the emitted mirror, which neither
        // the solo term nor the VCA mute reads.
        let any_solo = WebEffectiveView::new(&ready.vcas, &ready.solo, &ready.routes)
            .any_solo(&mut any_solo_cache);
        for strip in 0..strip_count {
            let slot = strip_count + strip;
            let mute = WebEffectiveView::new(&ready.vcas, &ready.solo, &ready.routes)
                .strip(strip, any_solo)
                .mute;
            for (lanes, muted) in ready.solo.strip_delta(strip, mute).into_iter().flatten() {
                let Some(entry) = ready.command_decoded.get_mut(lowered) else {
                    return Err(refuse(COMMAND_REASON_MALFORMED, count.saturating_sub(1)));
                };
                *entry = StagedCommand {
                    queue_slot: slot as u32,
                    original_wire_index: coalesce_first_wire_index,
                    kind: StagedCommandKind::Command(AdmittedCommand::Fader(
                        TrackFaderRecord::Mute {
                            lanes,
                            muted,
                            smoothing_samples: coalesce_smoothing,
                        },
                    )),
                };
                lowered += 1;
                ready.solo.record_emitted(strip, lanes, muted);
            }
        }
    }
    // Issue #1224 D3: the follow pass. It runs after every kind 4 record and the coalescing pass
    // above, so it reads the batch's final effective mutes: a send with `follows_mute` whose
    // source strip's effective lane mutes moved gets one record, built from its mirror through
    // the prepared route's coefficient function, with the new source lanes. `delta` yields only a
    // change, so a settled send is never retargeted.
    //
    // The ramp is the last strip-mute record staged for the source strip in this batch that
    // covers a lane whose effective mute changed, in staging order (kind 4 records in wire order,
    // then the coalescing pass's), and the follow record answers to that record's wire index.
    // Every effective-mute change stages a strip mute record covering its lane, so one always
    // exists; a follow without one is refused rather than guessed.
    //
    // Nothing is written here: the records join the batch's one cell write below with every
    // other staged entry, and the mirror's lanes move under its shadow, after every record is
    // built.
    if (solo_seen || mute_seen || vca_mute_seen) && !ready.routes.is_empty() {
        let follow_start = lowered;
        let route_base = strip_count * 3 + ready.effect_controls.len();
        // Decision 17 C7, slice S7c D5: each send's follow gate is `controls::effective_gate`'s.
        let view = WebEffectiveView::new(&ready.vcas, &ready.solo, &ready.routes);
        let any_solo = view.any_solo(&mut any_solo_cache);
        let follow_gate = |route: usize| view.follow_gate(route, any_solo);
        for (route, source_lane_muted) in LiveRouteMuteFollow::delta(&ready.routes, &follow_gate) {
            let (Some(entry), Some(producer)) =
                (ready.routes.get(route), ready.route_controls.get(route))
            else {
                return Err(refuse(COMMAND_REASON_MALFORMED, count.saturating_sub(1)));
            };
            let fader_slot = (strip_count + entry.source_strip) as u32;
            // Only a record covering a lane whose effective mute changed sets the ramp: a no-op
            // record on the other lane must not (#1224 verdict MINOR-1). The scan stops at the
            // follow records, which are never strip mutes (MINOR-2).
            let changed = [
                source_lane_muted[0] != entry.source_lane_muted[0],
                source_lane_muted[1] != entry.source_lane_muted[1],
            ];
            let Some((smoothing_samples, wire_index)) = ready.command_decoded[..follow_start]
                .iter()
                .rev()
                .find_map(|staged| match staged.kind {
                    StagedCommandKind::Command(AdmittedCommand::Fader(
                        TrackFaderRecord::Mute {
                            lanes,
                            smoothing_samples,
                            ..
                        },
                    )) if staged.queue_slot == fader_slot
                        && match lanes {
                            BuiltinLaneSelector::Left => changed[0],
                            BuiltinLaneSelector::Right => changed[1],
                            BuiltinLaneSelector::Both => changed[0] || changed[1],
                        } =>
                    {
                        Some((smoothing_samples, staged.original_wire_index))
                    }
                    _ => None,
                })
            else {
                return Err(refuse(COMMAND_REASON_MALFORMED, count.saturating_sub(1)));
            };
            // A ramp past `ROUTE_RAMP_LENGTH_MAXIMUM` cannot be followed at the strip's length, so
            // the strip mute that asked for it is refused whole, never clamped.
            let record = producer
                .record(
                    entry.gain_db,
                    entry.matrix,
                    entry.mute,
                    source_lane_muted,
                    smoothing_samples,
                )
                .map_err(|_| refuse(COMMAND_REASON_DOMAIN, wire_index as usize))?;
            let slot = route_base + route;
            let Some(staged) = ready.command_decoded.get_mut(lowered) else {
                return Err(refuse(COMMAND_REASON_MALFORMED, count.saturating_sub(1)));
            };
            *staged = StagedCommand {
                queue_slot: slot as u32,
                original_wire_index: wire_index,
                kind: StagedCommandKind::Command(AdmittedCommand::Route(record)),
            };
            lowered += 1;
        }
        for staged in follow_start..lowered {
            let route = ready.command_decoded[staged].queue_slot as usize - route_base;
            let lanes = WebEffectiveView::new(&ready.vcas, &ready.solo, &ready.routes)
                .follow_gate(route, any_solo);
            if !ready.routes.follow(route, lanes) {
                return Err(refuse(COMMAND_REASON_MALFORMED, count.saturating_sub(1)));
            }
        }
    }
    // Decode and validate every companion record before any target publication. An address with
    // no first semantic marker is an orphan, while a prepared-capable owner with no companion
    // records is rejected by its exact target validator below.
    if let Some(companion) = companion.as_ref() {
        for index in 0..companion.target_count {
            let target = companion_target(companion, index).map_err(|_| {
                refuse(
                    COMMAND_REASON_MALFORMED,
                    companion_wire_index(ready, lowered, companion, index),
                )
            })?;
            let Some((queue_slot, kind)) = prepared_queue_address(
                ready,
                target.track_index as usize,
                target.rack,
                target.effect_index,
            ) else {
                return Err(refuse(COMMAND_REASON_MALFORMED, 0));
            };
            if kind == PreparedOwnerKind::Eq {
                let effect = queue_slot
                    .checked_sub(strip_count * 3)
                    .ok_or_else(|| refuse(COMMAND_REASON_MALFORMED, 0))?;
                let Some(producer) = ready.effect_controls.get(effect).and_then(Option::as_ref)
                else {
                    return Err(refuse(COMMAND_REASON_MALFORMED, 0));
                };
                if !producer.has_owner() {
                    return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, 0));
                }
            }
        }
    }
    // Gather each owner's <=12 supplied targets on the stack and run its exact candidate
    // validator. This performs no coefficient design and retains no target cache.
    for entry in 0..lowered {
        let StagedCommandKind::PreparedOwner(mut marker) = ready.command_decoded[entry].kind else {
            continue;
        };
        let mut target_count = 0_usize;
        if marker.kind == PreparedOwnerKind::Eq {
            let mut targets = [effect_contract::PreparedEffectTarget {
                slot: 0,
                channel: effect_contract::ParameterChannel::Both,
                words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
            }; host_core::EQ_TARGET_CAPACITY];
            if let Some(companion) = companion.as_ref() {
                for index in 0..companion.target_count {
                    let target = companion_target(companion, index).map_err(|_| {
                        refuse(COMMAND_REASON_MALFORMED, marker.first_wire_index as usize)
                    })?;
                    let Some((queue_slot, kind)) = prepared_queue_address(
                        ready,
                        target.track_index as usize,
                        target.rack,
                        target.effect_index,
                    ) else {
                        return Err(refuse(COMMAND_REASON_MALFORMED, 0));
                    };
                    if kind != PreparedOwnerKind::Eq || queue_slot != marker.queue_slot as usize {
                        continue;
                    }
                    if target.base_revision != marker.base_revision || target_count == targets.len()
                    {
                        return Err(refuse(
                            COMMAND_REASON_MALFORMED,
                            marker.first_wire_index as usize,
                        ));
                    }
                    targets[target_count] = target.target;
                    target_count += 1;
                }
            }
            let effect = marker
                .queue_slot
                .checked_sub((strip_count * 3) as u32)
                .ok_or_else(|| refuse(COMMAND_REASON_MALFORMED, marker.first_wire_index as usize))?
                as usize;
            let producer = ready
                .effect_controls
                .get_mut(effect)
                .and_then(Option::as_mut)
                .ok_or_else(|| {
                    refuse(
                        COMMAND_REASON_UNSUPPORTED_KIND,
                        marker.first_wire_index as usize,
                    )
                })?;
            if let Err(error) =
                producer.preflight_candidate_targets(marker.base_revision, &targets[..target_count])
            {
                return Err(refuse(
                    owner_command_reason(error),
                    marker.first_wire_index as usize,
                ));
            }
        } else {
            let track = marker
                .queue_slot
                .checked_sub((strip_count * 2) as u32)
                .ok_or_else(|| refuse(COMMAND_REASON_MALFORMED, marker.first_wire_index as usize))?
                as usize;
            let mut targets = [PreparedInputFilterTarget {
                lanes: BuiltinLaneSelector::Both,
                section: 0,
                pair: [0.0; 2],
                coefficients: [0.0; 6],
            }; 4];
            if let Some(companion) = companion.as_ref() {
                for index in 0..companion.target_count {
                    let target = companion_target(companion, index).map_err(|_| {
                        refuse(COMMAND_REASON_MALFORMED, marker.first_wire_index as usize)
                    })?;
                    let Some((queue_slot, kind)) = prepared_queue_address(
                        ready,
                        target.track_index as usize,
                        target.rack,
                        target.effect_index,
                    ) else {
                        return Err(refuse(COMMAND_REASON_MALFORMED, 0));
                    };
                    if kind != PreparedOwnerKind::BuiltinInput
                        || queue_slot != marker.queue_slot as usize
                    {
                        continue;
                    }
                    if target.base_revision != marker.base_revision || target_count == targets.len()
                    {
                        return Err(refuse(
                            COMMAND_REASON_MALFORMED,
                            marker.first_wire_index as usize,
                        ));
                    }
                    targets[target_count] = companion_builtin_target(target).map_err(|_| {
                        refuse(COMMAND_REASON_MALFORMED, marker.first_wire_index as usize)
                    })?;
                    target_count += 1;
                }
            }
            let shadow = ready.input_filter_shadows.get(track).ok_or_else(|| {
                refuse(
                    COMMAND_REASON_UNSUPPORTED_KIND,
                    marker.first_wire_index as usize,
                )
            })?;
            if shadow.revision.checked_add(1).is_none() {
                return Err(refuse(
                    COMMAND_REASON_MALFORMED,
                    marker.first_wire_index as usize,
                ));
            }
            validate_builtin_target_set(shadow, &targets[..target_count])
                .map_err(|_| refuse(COMMAND_REASON_MALFORMED, marker.first_wire_index as usize))?;
        }
        // An owner's targets are cells (#1345, #1346): they want no room.
        marker.target_count = target_count as u32;
        if let Some(staged) = ready.command_decoded.get_mut(entry) {
            staged.kind = StagedCommandKind::PreparedOwner(marker);
        }
    }
    if let Some(companion) = companion.as_ref() {
        for index in 0..companion.target_count {
            let target = companion_target(companion, index).map_err(|_| {
                refuse(
                    COMMAND_REASON_MALFORMED,
                    companion_wire_index(ready, lowered, companion, index),
                )
            })?;
            let Some((queue_slot, kind)) = prepared_queue_address(
                ready,
                target.track_index as usize,
                target.rack,
                target.effect_index,
            ) else {
                return Err(refuse(COMMAND_REASON_MALFORMED, 0));
            };
            let matched = ready.command_decoded[..lowered].iter().any(|entry| {
                matches!(entry.kind, StagedCommandKind::PreparedOwner(marker)
                    if marker.queue_slot as usize == queue_slot
                        && marker.kind == kind
                        && marker.base_revision == target.base_revision)
            });
            if !matched {
                return Err(refuse(COMMAND_REASON_MALFORMED, 0));
            }
        }
    }
    for entry in 0..lowered {
        let staged = ready.command_decoded[entry];
        let slot = staged.queue_slot as usize;
        // The matrix and fader bands are cells (#1312): a write needs no room.
        if slot < strip_count * 2 {
            continue;
        }
        // So is the input band (#1346): no room to want, but the strip must hold the cells. A
        // strip prepared without its input lane is unaddressable here, like an absent effect lane
        // (#1254 D4); a plan that takes commands always has it and an option-0 plan takes none
        // (it refuses every command first), so this never fires.
        if slot < strip_count * 3 {
            if ready
                .controls
                .get(slot - strip_count * 2)
                .and_then(|control| control.input.as_ref())
                .is_none()
            {
                return Err(refuse(
                    COMMAND_REASON_UNSUPPORTED_KIND,
                    staged.original_wire_index as usize,
                ));
            }
            continue;
        }
        // The send band follows the effects (#1347): a send's target is a cell, which has no room
        // to want. Its route index was resolved while staging.
        if slot >= strip_count * 3 + ready.effect_controls.len() {
            continue;
        }
        // An effect's queue is its observation queue (#1345 D2): the subscriptions this
        // submission queues must fit it. Its parameter, bypass and target values are cells and
        // want none.
        let needed = ready.command_wanted[slot];
        if needed == 0 {
            continue;
        }
        let Some(available) = ready.queue_available(slot) else {
            return Err(refuse(
                COMMAND_REASON_UNSUPPORTED_KIND,
                staged.original_wire_index as usize,
            ));
        };
        if needed > available || ready.in_flight[slot].checked_add(needed).is_none() {
            return Err(refuse(
                COMMAND_REASON_BACKPRESSURE,
                staged.original_wire_index as usize,
            ));
        }
    }
    // The strips' cells, under one stamp (#1312 D7). Every check of the batch has passed, so the
    // stamp is the last fallible step: a refused batch above stamped and wrote nothing, and used
    // no revision. The revision is host-local, the next above the gate's last published one, and
    // is published as soon as the batch's cells are written, whatever the later passes do: the
    // writes are made, so the revision that reveals them must follow, or a later batch would
    // reveal them under its own.
    write_strip_cells(ready, lowered, strip_count, companion.as_ref())?;
    for entry in 0..lowered {
        let staged = ready.command_decoded[entry];
        let slot = staged.queue_slot as usize;
        match staged.kind {
            // Cells were written above: the strips', the sends' and the effects' values, and the
            // owners' targets.
            StagedCommandKind::Command(
                AdmittedCommand::Matrix(_)
                | AdmittedCommand::Fader(_)
                | AdmittedCommand::Input(_)
                | AdmittedCommand::Route(_)
                | AdmittedCommand::Effect(
                    EffectControlRecord::Parameter { .. } | EffectControlRecord::Bypass(_),
                ),
            )
            | StagedCommandKind::PreparedOwner(_) => {}
            // A prepared target reaches a lane only through its owner's transaction, and
            // `preflight_effect` refuses a raw one while staging.
            StagedCommandKind::Command(AdmittedCommand::Effect(
                EffectControlRecord::PreparedTarget(_),
            )) => unreachable!("a raw prepared target was refused while staging"),
            // The only queue record: an observation subscription, in order (#1345 D2).
            StagedCommandKind::Command(record) => {
                ready
                    .push(slot, record, applied_at_sample)
                    .expect("the free-room pass checked this record's room");
                ready.in_flight[slot] += 1;
                ready.has_in_flight_commands = true;
            }
        }
    }
    for entry in 0..lowered {
        let staged = ready.command_decoded[entry];
        let StagedCommandKind::PreparedOwner(marker) = staged.kind else {
            continue;
        };
        if marker.kind == PreparedOwnerKind::Eq {
            let effect = marker.queue_slot as usize - strip_count * 3;
            ready
                .effect_controls
                .get_mut(effect)
                .and_then(Option::as_mut)
                .expect("an Eq owner marker addresses a prepared effect producer")
                .commit_owner()
                .expect("the owner's targets were published");
        } else {
            let track = marker.queue_slot as usize - strip_count * 2;
            ready
                .input_filter_shadows
                .get_mut(track)
                .expect("a builtin input owner marker addresses a track's shadow")
                .commit(marker.base_revision)
                .expect("the shadow's base revision was checked while staging");
        }
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
    let effect_slot_id = ready
        .effect_controls
        .get(effect)
        .and_then(Option::as_ref)
        .map(|producer| producer.effect_id.as_ref())
        .ok_or(ObservationReadError::InvalidSelection)?;
    if ready
        .effect_observations
        .get(effect)
        .and_then(Option::as_ref)
        .is_none()
    {
        return Err(ObservationReadError::Unsupported);
    }
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
        let Some(producer) = ready.effect_controls.get(effect).and_then(Option::as_ref) else {
            continue;
        };
        if producer.effect_id.as_ref() == selection.effect_slot_id {
            addressed = Some(effect);
            break;
        }
    }
    let effect = addressed.ok_or(ObservationReadError::InvalidSelection)?;
    let handle = ready
        .effect_observations
        .get(effect)
        .and_then(Option::as_ref)
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
    sample_rate_hz: u32,
    selection: &ObservationSelection<'_>,
) -> Result<ObservationReadValues, ObservationReadError> {
    let (effect, tap_index) = resolve_observation(ready, selection)?;
    let handle = ready
        .effect_observations
        .get(effect)
        .and_then(Option::as_ref)
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

    fn preparation(failure: PrepareDiagnostics) -> Self {
        let result = match failure.kind() {
            PrepareRejection::Resource | PrepareRejection::Platform => RESULT_REFUSED_BUDGET,
            PrepareRejection::Session
            | PrepareRejection::Shape
            | PrepareRejection::Effect
            | PrepareRejection::Builtin
            | PrepareRejection::Graph => RESULT_REFUSED_DOCUMENT,
        };
        Self {
            result,
            diagnostic: failure.into_bytes(),
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
    let command_records = if options.live_control_command_queue_records == 0 {
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
    let prepared_control_bytes = if command_records == 0 {
        0
    } else {
        u64::try_from(size_of::<PreparedControlWorkspace>()).map_err(|_| arithmetic())?
    };
    let host_shell_bytes =
        u64::try_from(size_of::<AudioWorkletEngineHost>()).map_err(|_| arithmetic())?;
    let fixed_metadata = host_shell_bytes
        .checked_sub(u64::from(BOOT_OPTIONS_BYTES) + u64::from(STATUS_BYTES))
        .ok_or_else(arithmetic)?;
    let bridge_metadata = fixed_metadata
        .checked_add(prepared_control_bytes)
        .and_then(|bytes| bytes.checked_add(plane_reference_bytes))
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
        .max(prepared_control_bytes)
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
        plane_references: boxed_uninit_planes(
            u32::try_from(projection.source_samples / u64::from(projection.report.quantum_frames))
                .map_err(|_| BootFailure::fixed(RESULT_REFUSED_BUDGET, "web.resource.platform"))?,
        )
        .map_err(allocation)?,
        prepared_control: (projection.command_records != 0).then(PreparedControlWorkspace::zeroed),
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
    // Issue #143: observation capacity requires a control channel to carry the subscription, and a
    // master designation must be a plausible track index. The session-level range check is the
    // facade's; this one only refuses what cannot be a track index at all.
    if options.reserved0 != 0 {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_OPTIONS,
            "web.options.reserved0",
        ));
    }
    if options.live_control_observation_taps > u64::from(MAXIMUM_OBSERVATION_TAPS)
        || (options.live_control_observation_taps != 0
            && options.live_control_command_queue_records == 0)
        || options.live_control_master_track_plus_one > u64::from(u32::MAX)
        || (options.live_control_master_track_plus_one != 0
            && options.live_control_observation_taps == 0)
        || options.live_control_command_queue_records > u64::from(MAXIMUM_COMMAND_RECORDS)
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
/// The VCA shape the browser bounds at boot (issue #1245 amendment A1).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct BrowserVcaShape {
    strips: u64,
    vcas: u64,
    /// (strip, reaching VCA) pairs: the entries of `vca_reach()`.
    pairs: u64,
    /// Membership entries over every VCA.
    members: u64,
}

impl BrowserVcaShape {
    /// Exactly the bytes [`LiveVcaState::try_new`] keeps for this shape: per VCA its offset and
    /// mute, per strip its own and emitted dB, the two flattened reach tables with their starts,
    /// and the shadow of the four mutable arrays. Zero without a VCA.
    fn retained_bytes(self) -> Option<u64> {
        if self.vcas == 0 {
            return Some(0);
        }
        let word = size_of::<usize>() as u64;
        let vca = (size_of::<[f32; 2]>() + size_of::<[bool; 2]>()) as u64;
        let strip = 2 * size_of::<[f32; 2]>() as u64;
        let mirrors = self
            .vcas
            .checked_mul(vca)?
            .checked_add(self.strips.checked_mul(strip)?)?;
        let tables = (self.strips + 1)
            .checked_add(self.vcas + 1)?
            .checked_add(self.pairs.checked_mul(2)?)?
            .checked_mul(word)?;
        mirrors.checked_mul(2)?.checked_add(tables)
    }

    /// A conservative bound on the transient bytes [`LiveVcaState::try_new`] holds beyond what it
    /// keeps. `try_new` holds one `vca_reach()` result (a per-strip list with up to doubled
    /// capacity, plus its parent index) and the inverse table's cursor, and drops the reach lists
    /// before it seeds the emitted mirror from the flattened tables. This charges two reach
    /// results and the effective faders, so it over-counts. At the browser's bounds (256 VCAs,
    /// 16,384 pairs) it is never the binding refusal: the exact retained-budget check, or
    /// preparation's own resource limit, refuses a smaller budget first (the #1245 verdict's
    /// NIT-2); it stays as a defensive pre-check before any per-pair table is built.
    fn transient_bytes(self) -> Option<u64> {
        if self.vcas == 0 {
            return Some(0);
        }
        let word = size_of::<usize>() as u64;
        let reach = self
            .strips
            .checked_mul(size_of::<Vec<usize>>() as u64)?
            .checked_add(self.pairs.checked_mul(2 * word)?)?
            .checked_add(self.members.checked_mul(VCA_PARENT_ENTRY_BYTES)?)?
            .checked_add(self.vcas.checked_mul(word + 1)?)?;
        reach
            .checked_mul(2)?
            .checked_add(
                self.strips
                    .checked_mul(size_of::<controls::EffectiveStrip>() as u64)?,
            )?
            .checked_add(self.vcas.checked_mul(word)?)
    }
}

/// Count a normalized model's VCAs and (strip, reaching VCA) pairs, refusing a session past the
/// browser's bounds before any per-pair table exists (issue #1245 amendment A1).
///
/// Each VCA's ancestors -- the VCAs that reach it -- are one fixed-width bitset, filled in an order
/// where every VCA follows the VCAs that list it; a strip's reach is the union, over the VCAs that
/// list it, of each one's ancestors and itself. The work is linear in the membership entries, and
/// it allocates nothing proportional to the pair count it is bounding.
fn browser_vca_shape(model: &session::SessionModel) -> Result<BrowserVcaShape, BootFailure> {
    let refused = |code: &str| BootFailure::fixed(RESULT_REFUSED_BUDGET, code);
    let allocation = || refused("web.resource.allocation");
    let strips = (model.tracks.len() + model.submixes.len()) as u64;
    let vcas = &model.vcas;
    if vcas.is_empty() {
        return Ok(BrowserVcaShape {
            strips,
            ..BrowserVcaShape::default()
        });
    }
    if vcas.len() > MAXIMUM_BROWSER_VCAS {
        return Err(refused("web.vca.maximum_vcas"));
    }
    // VCA IDs, sorted, for member lookup.
    let mut ids: Vec<(&str, usize)> = Vec::new();
    ids.try_reserve_exact(vcas.len())
        .map_err(|_| allocation())?;
    ids.extend(
        vcas.iter()
            .enumerate()
            .map(|(index, vca)| (vca.id.as_str(), index)),
    );
    ids.sort_unstable();
    let vca_of = |id: &str| {
        ids.binary_search_by(|(candidate, _)| (*candidate).cmp(id))
            .ok()
            .map(|position| ids[position].1)
    };
    let members: usize = vcas.iter().map(|vca| vca.members.len()).sum();
    // Kahn's order over the VCA-to-VCA membership edges.
    let mut listed_by = vec_zeroed::<u32>(vcas.len()).ok_or_else(allocation)?;
    for vca in vcas {
        for member in &vca.members {
            if let Some(child) = vca_of(member.as_str()) {
                listed_by[child] += 1;
            }
        }
    }
    let mut ancestors = vec_zeroed::<[u64; VCA_REACH_WORDS]>(vcas.len()).ok_or_else(allocation)?;
    let mut ready: Vec<usize> = Vec::new();
    ready
        .try_reserve_exact(vcas.len())
        .map_err(|_| allocation())?;
    ready.extend((0..vcas.len()).filter(|index| listed_by[*index] == 0));
    // Every membership entry naming a strip, with the reach it contributes.
    let mut strip_entries: Vec<(&str, [u64; VCA_REACH_WORDS])> = Vec::new();
    strip_entries
        .try_reserve_exact(members)
        .map_err(|_| allocation())?;
    let mut ordered = 0_usize;
    while let Some(index) = ready.pop() {
        ordered += 1;
        let mut reach = ancestors[index];
        reach[index / 64] |= 1 << (index % 64);
        for member in &vcas[index].members {
            match vca_of(member.as_str()) {
                Some(child) => {
                    for (word, bits) in ancestors[child].iter_mut().zip(reach) {
                        *word |= bits;
                    }
                    listed_by[child] -= 1;
                    if listed_by[child] == 0 {
                        ready.push(child);
                    }
                }
                None => strip_entries.push((member.as_str(), reach)),
            }
        }
    }
    // Validation refuses a membership cycle; an order that misses a VCA cannot be counted.
    if ordered != vcas.len() {
        return Err(refused("web.vca.reach_pairs"));
    }
    strip_entries.sort_unstable_by(|left, right| left.0.cmp(right.0));
    let mut pairs = 0_u64;
    for run in strip_entries.chunk_by(|left, right| left.0 == right.0) {
        let mut reach = [0_u64; VCA_REACH_WORDS];
        for (_, bits) in run {
            for (word, bit) in reach.iter_mut().zip(bits) {
                *word |= bit;
            }
        }
        pairs += reach
            .iter()
            .map(|word| u64::from(word.count_ones()))
            .sum::<u64>();
    }
    if pairs > MAXIMUM_BROWSER_VCA_REACH_PAIRS {
        return Err(refused("web.vca.reach_pairs"));
    }
    Ok(BrowserVcaShape {
        strips,
        vcas: vcas.len() as u64,
        pairs,
        members: members as u64,
    })
}

/// `count` zeroed values in an exactly reserved `Vec`, or `None` when it cannot be reserved.
fn vec_zeroed<T: Copy + Default>(count: usize) -> Option<Vec<T>> {
    let mut values = Vec::new();
    values.try_reserve_exact(count).ok()?;
    values.resize(count, T::default());
    Some(values)
}

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
        maximum_vcas: u64::MAX,
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

/// The render-side halves a browser plan's preparation returns beside the plan (issue #1400 D5):
/// the meter consumers, the effect observation handles and the spectrum capture.
///
/// They travel inside the plan as its [`HostAttachment`], so a plan the control plane prepares
/// carries everything the render side reads, and they drop with the plan on the retirement path.
/// The browser renders its plan directly until it renders through the plan owner (#1381), which
/// reads them there; #1381 adds its own bookkeeping to this struct.
///
/// Field order is the drop order: the capture, then the consumers, as in `ReadyOwnership`.
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
        let shell = u64::try_from(size_of::<Self>()).ok()?;
        let meters = u64::try_from(self.meters.capacity())
            .ok()?
            .checked_mul(u64::try_from(size_of::<MeterConsumer>()).ok()?)?;
        let observations = u64::try_from(self.effect_observations.capacity())
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
}

impl HostAttachment for RenderCompanions {
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}

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
}

impl WebRuntimePreparer {
    /// A preparer for a session booted with `options` and the optional `spectrum` request.
    #[must_use]
    pub fn new(options: WebBootOptions, spectrum: Option<SpectrumPreparationRequest>) -> Self {
        Self { options, spectrum }
    }
}

impl control_plane::RuntimePreparer for WebRuntimePreparer {
    fn prepare(
        &self,
        compiled: &CompiledSession,
        caps: &HostPrepareCaps,
        lanes: HostLiveLanes,
        successor: Option<SuccessorBase<'_>>,
        _bound_cache: &mut control_plane::InputBoundCache<'_>,
        superseded: &host_core::LiveValuesSuperseded,
    ) -> Result<(PreparedHost, host_core::HostLiveControlHandles), PrepareDiagnostics> {
        let live_controls =
            live_control_request(self.options, compiled.quantum().0).ok_or_else(|| {
                PrepareDiagnostics::fixed(PrepareRejection::Session, "web.live_controls.config")
            })?;
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
        let report = &mut host.report;
        let charged = |row: u64| {
            row.checked_add(retained).ok_or_else(|| {
                PrepareDiagnostics::fixed(PrepareRejection::Platform, "web.resource.arithmetic")
            })
        };
        report.graph_session_plus_plan_bytes = charged(report.graph_session_plus_plan_bytes)?;
        report.graph_incremental_plan_bytes = charged(report.graph_incremental_plan_bytes)?;
        report.largest_engine_allocation_bytes =
            report.largest_engine_allocation_bytes.max(largest);
        host.plan.set_host_attachment(Box::new(companions));
        Ok((host, handles))
    }
}

/// Run the shared pipeline, then fold its engine-owned report into the frozen browser report.
///
/// The split is the one the facade documents: engine-owned rows come from
/// `host_core::HostPrepareReport`, the browser bridge's own rows (staging buffers, the
/// host shell, the plane-reference table) were already computed by `prepare_buffers`, and the three
/// browser caps are applied here because they are the *browser's* caps on the shared report.
fn compile_ready(
    session: CompiledSession,
    caps: &HostPrepareCaps,
    options: WebBootOptions,
    mut report: WebResourceReport,
    spectrum_request: Option<&SpectrumPreparationRequest>,
) -> Result<(ReadyOwnership, WebResourceReport), BootFailure> {
    let live_controls = live_control_request(options, session.quantum().0)
        .ok_or_else(|| fixed_diagnostic("web.live_controls.config"))?;
    let lanes = live_lanes(options);
    let commands_enabled = options_enable_commands(options);
    // #1312 D2, #1399: the session's `live_values_superseded` count, made here once, before any
    // plan is prepared. Every plan the session prepares takes this count, so host-core charges
    // none for it and the report below charges it once, as a bridge allocation.
    let live_values_superseded = host_core::LiveValuesSuperseded::new();
    // #1400 D5: the three boot shapes are `prepare_web_host`'s; the companions stay in
    // `ReadyOwnership` until the browser renders through the plan owner (#1381).
    let (host, handles, spectrum_capture) = prepare_web_host(
        &session,
        caps,
        &live_controls,
        spectrum_request,
        lanes,
        &live_values_superseded,
        None,
    )
    .map_err(BootFailure::preparation)?;
    let engine = host.report;

    let control_table = control_table_bytes(engine.source_count as usize)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let id_arena = source_id_arena_bytes(engine.source_id_bytes as usize)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    // Charged because the browser bridge retains them: the facade's control table and ID arena
    // (`control_retained_bytes` is exactly those two), the compiled session model, and the
    // prepared plan's state inventory (issue #1272), which `PreparedHost` keeps for a successor,
    // the session's `live_values_superseded` count (#1399), the one allocation every plan of
    // the session shares and host-core's graph row therefore leaves out, and the control catalog
    // (decision 17, C4): the control-plane dependency turns host-core's `control-provider` on for
    // this artifact, so every preparation builds it, `PreparedHost` keeps it, and `ReadyOwnership`
    // holds it for the whole session (#1400 amendment). No host-core row counts it.
    let catalog = host.control_catalog.resources();
    let ready_metadata = checked_sum_prepare([
        engine.control_retained_bytes,
        engine.session_model_bytes,
        engine.inventory_retained_bytes,
        host_core::LiveValuesSuperseded::retained_bytes(),
        catalog.retained_bytes,
    ])?;
    let mut bridge_metadata = report
        .bridge_metadata_bytes
        .checked_add(ready_metadata)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let mut bridge_retained = report
        .bridge_retained_bytes
        .checked_add(ready_metadata)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let mut bridge_largest = report
        .largest_bridge_allocation_bytes
        .max(control_table)
        .max(id_arena)
        .max(catalog.largest_allocation_bytes)
        .max(engine.session_largest_allocation_bytes);
    // One input-filter shadow per strip, the tracks then the submixes (issue #1213 D1): the order
    // of `handles.strips`, so a bus's prepared input-filter owner is its strip index.
    let mut input_filter_shadows = Vec::new();
    if commands_enabled {
        let model = session.normalized_model();
        input_filter_shadows
            .try_reserve_exact(model.tracks.len() + model.submixes.len())
            .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
        let strip_builtins = model
            .tracks
            .iter()
            .map(|track| &track.builtins)
            .chain(model.submixes.iter().map(|submix| &submix.builtins));
        for builtins in strip_builtins {
            let candidate = [
                if builtins.left.hpf_hz == 0.0 {
                    0.0
                } else {
                    builtins.left.hpf_hz
                },
                if builtins.left.lpf_hz == 0.0 {
                    0.0
                } else {
                    builtins.left.lpf_hz
                },
                if builtins.right.hpf_hz == 0.0 {
                    0.0
                } else {
                    builtins.right.hpf_hz
                },
                if builtins.right.lpf_hz == 0.0 {
                    0.0
                } else {
                    builtins.right.lpf_hz
                },
            ];
            builtins::validate_input_filter_pair(
                session.sample_rate().0,
                candidate[0],
                candidate[1],
            )
            .map_err(|_| fixed_diagnostic("web.live_controls.input_filter"))?;
            builtins::validate_input_filter_pair(
                session.sample_rate().0,
                candidate[2],
                candidate[3],
            )
            .map_err(|_| fixed_diagnostic("web.live_controls.input_filter"))?;
            input_filter_shadows.push(BuiltinInputShadow {
                committed: candidate,
                candidate,
                dirty: [false; 4],
                revision: 0,
            });
        }
    }
    let input_shadow_bytes = u64::try_from(
        input_filter_shadows
            .len()
            .checked_mul(size_of::<BuiltinInputShadow>())
            .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?,
    )
    .map_err(|_| fixed_diagnostic("web.resource.arithmetic"))?;
    bridge_metadata = bridge_metadata
        .checked_add(input_shadow_bytes)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    bridge_retained = bridge_retained
        .checked_add(input_shadow_bytes)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    bridge_largest = bridge_largest.max(input_shadow_bytes);
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
    let track_count = handles.track_count;
    let mut tracks = handles.strips;
    if track_count > tracks.len() {
        return Err(fixed_diagnostic("web.live_controls.effects").into());
    }
    let strip_count = tracks.len();
    let mut submixes: Vec<Box<str>> = Vec::new();
    submixes
        .try_reserve_exact(strip_count - track_count)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    submixes.extend(tracks.drain(track_count..));
    let mut rack_effects = Vec::new();
    rack_effects
        .try_reserve_exact(strip_count)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    // Issue #210 phase 1: the solo state's user-mute mirror starts from the *session's* baked
    // fader mutes -- the same `left_mute`/`right_mute` words `track_parameters` compiles into the
    // prepared fader section -- read in the same normalized track order `handles.strips` leads with,
    // because that order is the addressing authority for every queue, meter and command index.
    // One seed per strip (issue #1213 D3): the tracks, soloable, then the submixes, solo-safe, each
    // seeded from its own session fader mutes. Issue #1242 D3, decision 17 C7: each also carries
    // its effective mute from the same composition the compilers baked (`controls::effective`, in
    // strip order), which is what the render plane has been told, before the solo state and the
    // live-send mirror are built from it.
    let mut prepared_mutes: Vec<StripMuteSeed> = Vec::new();
    prepared_mutes
        .try_reserve_exact(strip_count)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    let model = session.normalized_model();
    if model.tracks.len() != track_count || model.submixes.len() != submixes.len() {
        return Err(fixed_diagnostic("web.live_controls.effects").into());
    }
    let effective_faders = controls::effective(model, &controls::MonitorOverlay::default());
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
        prepared_mutes.push(StripMuteSeed {
            mutes: [track.fader.left_mute, track.fader.right_mute],
            solo_safe: false,
            effective_mute: effective_faders.strips[prepared_mutes.len()].mute,
        });
    }
    for submix in &model.submixes {
        rack_effects.push([
            count(model.console.pre_insert.len())?,
            count(submix.inserts.effects.len())?,
            count(model.console.post_insert.len())?,
        ]);
        prepared_mutes.push(StripMuteSeed {
            mutes: [submix.fader.left_mute, submix.fader.right_mute],
            solo_safe: true,
            effective_mute: effective_faders.strips[prepared_mutes.len()].mute,
        });
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
    let mut effect_controls: Vec<Option<EffectControlProducer>> = Vec::new();
    effect_controls
        .try_reserve_exact(total_effects as usize)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    effect_controls.resize_with(total_effects as usize, || None);
    for producer in handles.effect_controls {
        let Some(strip) = strip_index(&tracks, &submixes, &producer.track_id) else {
            return Err(fixed_diagnostic("web.live_controls.effects").into());
        };
        let Some(entry) =
            dense_effect_slot(effect_base[strip], rack_effects[strip], producer.address)
                .and_then(|slot| effect_controls.get_mut(slot))
        else {
            return Err(fixed_diagnostic("web.live_controls.effects").into());
        };
        if entry.is_some() {
            return Err(fixed_diagnostic("web.live_controls.effects").into());
        }
        *entry = Some(producer);
    }
    // Host-core checked its native Vec and owned payload against the preparation caps. The
    // browser retains this dense replacement table instead of that Vec. Add only the replacement
    // table and transferred payload to the bridge rows, each once.
    let effect_control_table_bytes = u64::try_from(effect_controls.len())
        .ok()
        .and_then(|count| count.checked_mul(size_of::<Option<EffectControlProducer>>() as u64))
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let effect_control_payload_bytes = engine.effect_control_resources.owned_payload_bytes;
    let effect_control_largest = effect_control_table_bytes.max(
        engine
            .effect_control_resources
            .largest_owned_allocation_bytes,
    );
    let effect_control_retained = effect_control_table_bytes
        .checked_add(effect_control_payload_bytes)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.bridge_metadata_bytes = report
        .bridge_metadata_bytes
        .checked_add(effect_control_retained)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.bridge_retained_bytes = report
        .bridge_retained_bytes
        .checked_add(effect_control_retained)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.largest_bridge_allocation_bytes = report
        .largest_bridge_allocation_bytes
        .max(effect_control_largest);
    report.largest_named_allocation_bytes = report
        .largest_named_allocation_bytes
        .max(effect_control_largest);
    // Issue #1222 D3: the live sends, in `handles.route_controls` order, which must be the
    // model's live-route order -- the order a command's live-route index and the mirror both
    // read. Checked by ID here, once, rather than assumed.
    let route_controls = handles.route_controls;
    let live_routes_in_order = route_controls.is_empty()
        || (LiveRouteState::live_routes(model).count() == route_controls.len()
            && LiveRouteState::live_routes(model)
                .zip(&route_controls)
                .all(|(route, producer)| route.id.as_str() == producer.route_id()));
    if !live_routes_in_order {
        return Err(fixed_diagnostic("web.live_controls.routes").into());
    }
    let solo = LiveControlSoloState::try_new(&prepared_mutes)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    // A following send's source lanes start at its source strip's effective mute; solo is never
    // persisted, so at preparation that is the session fader mute or VCA mute the compilers
    // folded in (issue #1242 D3).
    let routes = if route_controls.is_empty() {
        LiveRouteState::empty()
    } else {
        LiveRouteState::try_new(model, &|strip, lane| {
            effective_faders
                .strips
                .get(strip)
                .is_some_and(|effective| effective.mute[lane])
        })
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?
    };
    // Retained bridge state the browser owns for its live sends (issue #1222; #1221 verdict
    // MINOR-1): every byte the route lanes add -- cells, render-side owners, the producer table
    // this host keeps as host-core built it, the route IDs and the activity table -- which the
    // compile-time graph estimate deliberately leaves out, plus the mirror and its shadow.
    let route_mirror_bytes = u64::try_from(routes.len())
        .ok()
        .and_then(|count| count.checked_mul(size_of::<host_core::LiveRoute>() as u64))
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let route_retained = engine
        .route_control_resources
        .total_bytes
        .checked_add(route_mirror_bytes)
        .and_then(|bytes| bytes.checked_add(route_mirror_bytes))
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let route_largest = engine
        .route_control_resources
        .largest_allocation_bytes
        .max(route_mirror_bytes);
    report.bridge_metadata_bytes = report
        .bridge_metadata_bytes
        .checked_add(route_retained)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.bridge_retained_bytes = report
        .bridge_retained_bytes
        .checked_add(route_retained)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.largest_bridge_allocation_bytes =
        report.largest_bridge_allocation_bytes.max(route_largest);
    report.largest_named_allocation_bytes =
        report.largest_named_allocation_bytes.max(route_largest);
    // Issue #1245 D3/D4: the live VCA state, built after the solo state from the same normalized
    // model, and charged beside the route mirror -- every byte it reserved, its shadow included.
    // A session without VCAs, or a host without live controls (which refuses every VCA kind),
    // keeps the empty state and adds nothing to any row.
    let vcas = if !commands_enabled {
        LiveVcaState::empty()
    } else {
        LiveVcaState::try_new(model).map_err(|_| fixed_diagnostic("web.resource.allocation"))?
    };
    // Issue #1560: the strips' mirrored values, seeded from the same model and charged beside the
    // VCA state, its shadow included.
    let strip_values =
        LiveStripValues::try_new(model).map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    // Issue #1364 D2: the session's ramp table and the per-strip pan windows, computed here, once,
    // on the control thread, and charged like the other retained tables.
    let ramps =
        SessionRamps::try_new(model).ok_or_else(|| fixed_diagnostic("web.resource.allocation"))?;
    // Decision 17, slice S7c D1: the core's descriptor of every row the live kinds write.
    let rows = WebControlRows::new(session.sample_rate().0)
        .ok_or_else(|| fixed_diagnostic("web.live_controls.rows"))?;
    let live_state_retained = vcas
        .retained_bytes()
        .checked_add(ramps.retained_bytes())
        .and_then(|bytes| bytes.checked_add(strip_values.retained_bytes()))
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let live_state_largest = vcas
        .largest_allocation_bytes()
        .max(ramps.retained_bytes())
        .max(strip_values.largest_allocation_bytes());
    report.bridge_metadata_bytes = report
        .bridge_metadata_bytes
        .checked_add(live_state_retained)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.bridge_retained_bytes = report
        .bridge_retained_bytes
        .checked_add(live_state_retained)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.largest_bridge_allocation_bytes = report
        .largest_bridge_allocation_bytes
        .max(live_state_largest);
    report.largest_named_allocation_bytes = report
        .largest_named_allocation_bytes
        .max(live_state_largest);
    // Three per-strip bands since #210 phase 3: matrix/pan, fader/mute, input trim/polarity, each
    // `T + S` long since issue #1213 D1; the effect band follows and covers every strip, and the
    // live send band follows it (issue #1222 D4).
    let queue_count = strip_count
        .checked_mul(3)
        .and_then(|value| value.checked_add(total_effects as usize))
        .and_then(|value| value.checked_add(route_controls.len()))
        .ok_or_else(|| fixed_diagnostic("web.live_controls.effects"))?;
    // Issue #143: the observation handles are permuted into the same dense `effect_slot` order
    // the command producers use, so one index serves both the subscribe path and the poll.
    let mut effect_observations: Vec<Option<EffectObservationHandle>> = Vec::new();
    effect_observations
        .try_reserve_exact(total_effects as usize)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    effect_observations.resize_with(total_effects as usize, || None);
    let mut observation_arm_samples: Vec<Box<[u64]>> = Vec::new();
    observation_arm_samples
        .try_reserve_exact(total_effects as usize)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    observation_arm_samples.resize_with(total_effects as usize, || Box::new([]));
    // `observation_tracks[slot]` is the strip index of that effect slot's observed instance, or
    // `u32::MAX` for a slot with no taps. Built once, here, so the poll's fold is arithmetic.
    let mut observation_tracks = vec![u32::MAX; total_effects as usize];
    let observation_present = vec![false; strip_count];
    for handle in handles.effect_observations {
        let Some(strip) = strip_index(&tracks, &submixes, &handle.track_id) else {
            return Err(fixed_diagnostic("web.live_controls.observation").into());
        };
        let Some(slot) = dense_effect_slot(effect_base[strip], rack_effects[strip], handle.address)
        else {
            return Err(fixed_diagnostic("web.live_controls.observation").into());
        };
        let Some(entry) = effect_observations.get_mut(slot) else {
            return Err(fixed_diagnostic("web.live_controls.observation").into());
        };
        if entry.is_some() {
            return Err(fixed_diagnostic("web.live_controls.observation").into());
        }
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
        *entry = Some(handle);
    }
    let observation_arm_sample_bytes = observation_arm_samples
        .iter()
        .try_fold(0_u64, |total, samples| {
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
        .checked_mul(size_of::<Box<[u64]>>() as u64)
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
    meter_header.master_track_plus_one = handles
        .master_track
        .map_or(0, |track| track.saturating_add(1));
    let meter_count = handles.meters.len();
    let master_capacity = handles
        .meters
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
    // The decoded command array carries the enlarged internal EffectControlRecord enum. Its
    // typed backing is a separate retained allocation from the public 48-byte wire staging row;
    // charge the full actual array before the final aggregate budget check.
    let decoded_count =
        command_staging_count(strip_count, routes.len(), vcas.reached_strip_count())?;
    let decoded_bytes = u64::try_from(decoded_count)
        .ok()
        .and_then(|count| count.checked_mul(size_of::<StagedCommand>() as u64))
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
    let mut host = host;
    let plan_gate = host
        .plan
        .take_revision_gate()
        .ok_or_else(|| fixed_diagnostic("web.internal.plan_gate"))?;
    let ready = ReadyOwnership {
        commands_enabled,
        controls: handles.strip_controls,
        live_values_superseded,
        plan_gate,
        effect_controls: effect_controls.into_boxed_slice(),
        effect_observations: effect_observations.into_boxed_slice(),
        observation_tracks: observation_tracks.into_boxed_slice(),
        observation_present: observation_present.into_boxed_slice(),
        observation_armed: boxed_zero_u32(total_effects as usize)?,
        observation_arm_samples: observation_arm_samples.into_boxed_slice(),
        master_track: handles.master_track,
        meter_header,
        effect_base: effect_base.into_boxed_slice(),
        command_wanted: boxed_zero_u32(queue_count)?,
        observe_named: boxed_zero_u32(total_effects as usize)?,
        command_decoded: boxed_command_staging(
            strip_count,
            routes.len(),
            vcas.reached_strip_count(),
        )?,
        solo,
        vcas,
        strip_values,
        ramps,
        rows,
        route_controls,
        routes,
        in_flight: boxed_zero_u32(queue_count)?,
        has_in_flight_commands: false,
        input_filter_shadows: input_filter_shadows.into_boxed_slice(),
        sample_rate_hz: session.sample_rate().0,
        tracks,
        submixes,
        rack_effects: rack_effects.into_boxed_slice(),
        spectrum: spectrum_capture.map(|captures| {
            SpectrumService::new(captures, session.sample_rate().0, session.quantum().0)
        }),
        host,
        meters: handles.meters,
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
        session,
    };
    Ok((ready, report))
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

/// Whether the app opted in to live commands (issue #1326 D4): the one word the public command
/// gate, the input-filter shadows and the live VCA state read. The lanes do not say, because every
/// plan carries them.
fn options_enable_commands(options: WebBootOptions) -> bool {
    options.live_control_command_queue_records != 0
}

/// The live lanes a browser plan attaches (issue #1326 D3): every lane for an app that opted in to
/// live commands, and only each strip's fader/mute and matrix/pan lanes otherwise, so no strip's
/// builtin tail turns infinite and no route loses its fold.
fn live_lanes(options: WebBootOptions) -> HostLiveLanes {
    if options_enable_commands(options) {
        HostLiveLanes::ALL
    } else {
        HostLiveLanes::FADER_AND_MATRIX
    }
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

fn boxed_command_staging(
    strip_count: usize,
    route_count: usize,
    vca_reached_strips: usize,
) -> Result<Box<[StagedCommand]>, Vec<u8>> {
    // Two entries per staged wire record: one `channel = both` command on a per-lane effect
    // parameter lowers to one record per lane (#140 C).
    //
    // Plus `2 * strip_count` for issue #210 phase 1's coalesced solo emission (per strip since
    // issue #1213 D1, whose coalescing pass walks every strip). A submission that moves a solo
    // bit owes the live controls the difference between the composed effective mute and what the
    // render plane was last told; that is at most two records per strip, because
    // `TrackFaderRecord::Mute` carries one `muted` bool and a strip whose user mute is asymmetric
    // needs one record per lane to restore. The two terms add rather than max: a batch may carry
    // 256 effect-parameter records *and* a solo toggle.
    //
    // Plus `route_count` for issue #1224's follow pass: at most one record per live send per
    // batch, because `delta` yields each send once, after the batch's last strip mute.
    //
    // Plus `2 * vca_reached_strips` for issue #1245's VCA fader pass: at most one `FaderDb` per
    // lane of each strip some VCA reaches, per batch. A VCA mute adds nothing here: its member
    // records are the coalescing pass's, already bounded by `2 * strip_count`.
    let count = command_staging_count(strip_count, route_count, vca_reached_strips)?;
    let empty = StagedCommand {
        queue_slot: 0,
        original_wire_index: 0,
        kind: StagedCommandKind::Command(AdmittedCommand::Effect(EffectControlRecord::Bypass(
            false,
        ))),
    };
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    value.resize(count, empty);
    Ok(value.into_boxed_slice())
}

fn command_staging_count(
    strip_count: usize,
    route_count: usize,
    vca_reached_strips: usize,
) -> Result<usize, Vec<u8>> {
    (MAXIMUM_COMMAND_RECORDS as usize * 2)
        .checked_add(strip_count.checked_mul(2).ok_or_else(arithmetic)?)
        .and_then(|count| count.checked_add(route_count))
        .and_then(|count| count.checked_add(vca_reached_strips.checked_mul(2)?))
        .ok_or_else(arithmetic)
}

fn arithmetic() -> Vec<u8> {
    fixed_diagnostic("web.resource.arithmetic")
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

pub mod control_targets;
pub use control_targets::{
    WebBuiltinInputConfig, WebEqTargetConfig, WebEqTargetEdit, WebEqTargetRequest,
    WebEqTargetResult, WebInputFilterEdit, WebPreparedEffectCompanionHeader,
    WebPreparedEffectCompanionRecord, WebPreparedEffectTarget,
};
mod ffi;
mod render_lock;

pub use ffi::*;
pub use render_lock::RenderLockedAllocator;

#[cfg(test)]
mod tests;
