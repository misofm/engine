//! Build-time transcription of the browser-Wasm boot ABI (issues #240, #243).
//!
//! # Why this document exists
//!
//! `miso-engine-v1-parameter-metadata.json` describes *effects*: names, units, domains, lattices.
//! It deliberately says nothing about bytes. But a JavaScript consumer of the browser host has a
//! second, entirely different thing to know — where `require_quantum_frames` sits inside the boot
//! options block, which numeric result code means "boot refused your document", what the staging
//! sequence is — and before this file that knowledge was hand-written on the JavaScript side and
//! drifted. Issue #207's N-13(d) counted **five** independent hand-written copies of the boot
//! configuration table; the fifth wrote a 192-byte struct's offsets into a 64-byte buffer and was
//! silently producing garbage.
//!
//! So this module emits the byte-level and state-machine vocabulary the same way the parameter
//! metadata emits the effect vocabulary: from the Rust definitions themselves, through
//! `offset_of!` and the frozen constants, never from a table anybody types twice. `--check`
//! regenerates and compares byte for byte, which is what makes a hand edit fail before a consumer
//! can observe the drift.
//!
//! # `bootResultAliases`: one name per value is not enough
//!
//! The boot verb's refusals are deliberate **aliases** of the general result codes rather than
//! fresh numbers: `RESULT_REFUSED_DOCUMENT == RESULT_INVALID_ARGUMENT` (1),
//! `RESULT_REFUSED_OPTIONS == RESULT_ABI_MISMATCH` (2), `RESULT_REFUSED_LIFECYCLE ==
//! RESULT_WRONG_STATE` (3). A flat one-name-per-value table therefore cannot express both
//! vocabularies: naming value 2 `refusedOptions` would misname every non-boot acknowledgement that
//! returns it, and naming it `abiMismatch` would leave the boot vocabulary unrepresentable.
//!
//! `resultCodes` therefore keeps the frozen base names, and `bootResultAliases` carries the three
//! alias spellings **scoped to `miso_engine_web_v1_boot`'s return value**, transcribed from the
//! Rust alias constants rather than typed out. A consumer surfaces the context-appropriate name:
//! boot's return reads through the alias table, an asset-hash mismatch reads `abiMismatch`, and
//! everything else reads the base names. No number moves; adopted ruling 5462139867 finding 2.
//!
//! # `stagingSequence`: four calls, named
//!
//! Boot v1 was described in prose as "the 3-call boot", which miscounts: the options block must be
//! addressed before it can be written, so the sequence is `abi_version` → `boot_options_ptr` →
//! `document_ptr` → `boot`. The sequence is emitted here, in order, by export name, so no consumer
//! reconstructs it from a sentence.
//!
//! # `sourceRing`: the default ring is a rule, not a number
//!
//! `default_source_ring_frames` covers `SOURCE_STALL_TOLERANCE_MS` of audio, rounded up to a whole
//! number of quanta, plus one quantum held by the consumer and one in the recycle path. A consumer
//! that wants the ring the engine will actually pick — for sizing its own producer — needs the two
//! inputs to that rule, not a rate-specific answer. Both are transcribed, so the SDK derives the
//! ring from the shape the boot itself reported instead of holding a private copy of `100`.

use core::mem::offset_of;

use serde::Serialize;

use crate::json::{self, Entries};
use host_core::RESPONSE_MAXIMUM_POINTS;
use host_web::{
    ABI_VERSION, BACKEND_SCALAR, BACKEND_SIMD128, BOOT_OPTIONS_BYTES, BUFFER_COMMAND,
    BUFFER_DIAGNOSTIC, BUFFER_METER_FRAME, BUFFER_OUTPUT_PCM, BUFFER_SOURCE_ID, BUFFER_SOURCE_PCM,
    COMMAND_OBSERVE_SUBSCRIBE, COMMAND_OBSERVE_UNSUBSCRIBE, COMMAND_REASON_BACKPRESSURE,
    COMMAND_REASON_DOMAIN, COMMAND_REASON_MALFORMED, COMMAND_REASON_NONE,
    COMMAND_REASON_OBSERVATION_LIMIT, COMMAND_REASON_OBSERVATION_UNBOUND,
    COMMAND_REASON_UNKNOWN_EFFECT, COMMAND_REASON_UNKNOWN_PARAMETER, COMMAND_REASON_UNKNOWN_RACK,
    COMMAND_REASON_UNKNOWN_TAP, COMMAND_REASON_UNKNOWN_TRACK, COMMAND_REASON_UNSUPPORTED_KIND,
    COMMAND_REASON_WRONG_STATE, COMMAND_RECORD_BYTES, COMMAND_REPORT_BYTES,
    DEFAULT_MAXIMUM_MEMORY_BYTES, DEFAULT_METER_BLOCKS, DIAGNOSTIC_BYTES,
    LIVE_RESPONSE_CAPTURE_BYTES, LIVE_RESPONSE_MAXIMUM_ID_BYTES, LIVE_RESPONSE_MAXIMUM_OWNERS,
    LIVE_RESPONSE_MAXIMUM_POINTS, LIVE_RESPONSE_MAXIMUM_SECTIONS,
    LIVE_RESPONSE_MEANING_EQ_FILTER_SUBTOTAL, LIVE_RESPONSE_MODE_TARGET, LIVE_RESPONSE_OWNER_BYTES,
    LIVE_RESPONSE_RACK_CONSOLE, LIVE_RESPONSE_RACK_INPUT_FILTERS, LIVE_RESPONSE_RACK_INSERTS,
    LIVE_RESPONSE_REQUEST_BYTES, LIVE_RESPONSE_RESULT_BYTES, LIVE_RESPONSE_SECTION_BYTES,
    MAXIMUM_COMMAND_RECORDS, MAXIMUM_DOCUMENT_BYTES, MAXIMUM_OBSERVATION_TAPS, METER_HEADER_BYTES,
    OBSERVATION_CHANNEL_BOTH, OBSERVATION_CHANNEL_LEFT, OBSERVATION_CHANNEL_RIGHT,
    OBSERVATION_RESULT_BYTES, OBSERVATION_SELECTION_BYTES, OBSERVATION_STATUS_PENDING,
    OBSERVATION_STATUS_READY, OBSERVATION_STATUS_UNARMED, PLAN_LATENCY_BYTES, RACK_CONSOLE,
    RACK_INSERTS, RACK_NOT_APPLICABLE, RESOURCE_REPORT_BYTES, RESPONSE_CHANNEL_BOTH,
    RESPONSE_CHANNEL_LEFT, RESPONSE_CHANNEL_RIGHT, RESPONSE_GRID_LINEAR, RESPONSE_GRID_LOGARITHMIC,
    RESULT_ABI_MISMATCH, RESULT_BACKPRESSURE, RESULT_BUFFER_TOO_SMALL, RESULT_COMPILE_REJECTED,
    RESULT_INTERNAL, RESULT_INVALID_ARGUMENT, RESULT_OK, RESULT_REFUSED_BUDGET,
    RESULT_REFUSED_DOCUMENT, RESULT_REFUSED_LIFECYCLE, RESULT_REFUSED_OPTIONS,
    RESULT_RENDER_REJECTED, RESULT_REPREPARE_REQUIRED, RESULT_UNSUPPORTED, RESULT_WRONG_STATE,
    SEEK_REPORT_BYTES, SOURCE_STALL_TOLERANCE_MS, SPECTRUM_BIN_COUNT, SPECTRUM_CAPTURE_BYTES,
    SPECTRUM_CHANNEL_BOTH, SPECTRUM_CHANNEL_LEFT, SPECTRUM_CHANNEL_RIGHT,
    SPECTRUM_COLLECTION_ENTRY_BYTES, SPECTRUM_COLLECTION_ENTRY_CAPACITY,
    SPECTRUM_COLLECTION_REQUEST_BYTES, SPECTRUM_COLLECTION_TARGET_IDS_BYTES,
    SPECTRUM_MAXIMUM_ID_BYTES, SPECTRUM_MAXIMUM_PREPARED_TARGETS, SPECTRUM_REQUEST_BYTES,
    SPECTRUM_RESULT_HEADER_BYTES, SPECTRUM_STREAM_METADATA_BYTES, SPECTRUM_STREAM_STATUS_FAILED,
    SPECTRUM_STREAM_STATUS_GAP, SPECTRUM_STREAM_STATUS_INACTIVE, SPECTRUM_STREAM_STATUS_PENDING,
    SPECTRUM_STREAM_STATUS_READY, SPECTRUM_STREAM_STATUS_STOPPED, SPECTRUM_STREAM_STATUS_WARMING,
    SPECTRUM_TARGET_OUTPUT, SPECTRUM_TARGET_TRACK_POST_INPUT, SPECTRUM_TARGET_TRACK_POST_PAN,
    SPECTRUM_WINDOW_FRAMES, SPECTRUM_WINDOW_HEADER_BYTES, STATE_DISPOSED, STATE_FAILED,
    STATE_READY, STATUS_BYTES, WATERMARK_BYTES, WebBootOptions, WebCommandReport,
    WebLiveResponseOwner, WebLiveResponseRequest, WebLiveResponseResult, WebLiveResponseSection,
    WebMeterHeader, WebObservationResult, WebObservationSelection, WebPlanLatency,
    WebPlanWatermark, WebResourceReport, WebSeekReport, WebSpectrumCollectionEntry,
    WebSpectrumCollectionRequest, WebSpectrumRequest, WebSpectrumResult, WebSpectrumStreamMetadata,
    WebSpectrumWindow, WebStatus,
};

/// The emitted file name, shipped beside the Wasm artifact and the parameter metadata.
pub const OUTPUT_NAME: &str = "miso-engine-v1-abi-layout.json";
/// The schema tag every consumer must check before reading a field.
pub const SCHEMA: &str = "miso.web.abi-layout.v1";

/// The quanta the default source ring reserves beyond the stall tolerance.
///
/// One quantum is held by the consumer while it renders and one is in the recycle path, so a
/// producer that keeps the tolerance filled never finds the ring closed. Transcribed here because
/// a consumer deriving the ring from a reported shape needs the whole rule, and `+ 2` inside
/// `default_source_ring_frames` is otherwise a number the SDK would have to know privately.
pub const SOURCE_RING_RESERVE_QUANTA: u32 = 2;

/// The lifecycle stages a typed SDK error can be raised at, in the order a session passes through
/// them.
///
/// These name *where* a refusal happened, which is the one thing a numeric result code cannot say:
/// result `1` means one thing when the module hash did not match the provenance record and another
/// when the document failed to parse. The vocabulary replaces the dead two-phase `"compile"`
/// spelling of the pre-boot-v1 ABI (issue #243 S2(b)) and is anchored to the export surface:
/// `asset` covers module verification and instantiation, `boot` the four-call staging sequence,
/// `source` the source submit/seek exports, `render` the render export, `output` reading the
/// output PCM buffer, and `lifecycle` dispose and the boot-while-live refusal.
pub const ERROR_PHASES: [&str; 6] = ["asset", "boot", "source", "render", "output", "lifecycle"];

/// Every function the module exports, sorted, exactly as `scripts/check-web-audioworklet.sh`
/// freezes the set.
///
/// Publishing the whole surface -- not just the four boot calls -- is what lets a JavaScript
/// consumer name an export without typing a string. `memory` is deliberately absent: it is the
/// module's linear memory, not a call, and a consumer reaches it as `instance.exports.memory`.
pub const EXPORTS: [&str; 110] = [
    "miso_engine_web_v1_abi_version",
    "miso_engine_web_v1_boot",
    "miso_engine_web_v1_boot_diagnostic_bytes",
    "miso_engine_web_v1_boot_options_ptr",
    "miso_engine_web_v1_boot_result",
    "miso_engine_web_v1_boot_with_spectrum_hop",
    "miso_engine_web_v1_buffer_capacity",
    "miso_engine_web_v1_buffer_ptr",
    "miso_engine_web_v1_command_report_ptr",
    "miso_engine_web_v1_command_submit",
    "miso_engine_web_v1_control_allocation_count",
    "miso_engine_web_v1_control_command",
    "miso_engine_web_v1_control_event",
    "miso_engine_web_v1_control_request_ptr",
    "miso_engine_web_v1_control_response_bytes",
    "miso_engine_web_v1_control_response_ptr",
    "miso_engine_web_v1_control_revision",
    "miso_engine_web_v1_declare_discontinuity",
    "miso_engine_web_v1_dispose",
    "miso_engine_web_v1_document_ptr",
    "miso_engine_web_v1_meter_header_ptr",
    "miso_engine_web_v1_meter_lease",
    "miso_engine_web_v1_meter_poll",
    "miso_engine_web_v1_observation_count",
    "miso_engine_web_v1_observation_effect_index",
    "miso_engine_web_v1_observation_effect_slot_id",
    "miso_engine_web_v1_observation_id_capacity",
    "miso_engine_web_v1_observation_id_ptr",
    "miso_engine_web_v1_observation_native_effect_id",
    "miso_engine_web_v1_observation_rack",
    "miso_engine_web_v1_observation_read",
    "miso_engine_web_v1_observation_result_bytes",
    "miso_engine_web_v1_observation_result_ptr",
    "miso_engine_web_v1_observation_selection_bytes",
    "miso_engine_web_v1_observation_selection_capacity",
    "miso_engine_web_v1_observation_selection_ptr",
    "miso_engine_web_v1_observation_tap_count",
    "miso_engine_web_v1_observation_tap_id",
    "miso_engine_web_v1_observation_track_index",
    "miso_engine_web_v1_plan_latency_ptr",
    "miso_engine_web_v1_plan_latency_read",
    "miso_engine_web_v1_render",
    "miso_engine_web_v1_render_allocation_count",
    "miso_engine_web_v1_resource_ptr",
    "miso_engine_web_v1_response_frame_close",
    "miso_engine_web_v1_response_frame_ptr",
    "miso_engine_web_v1_response_frame_reply_bytes",
    "miso_engine_web_v1_response_frame_reply_ptr",
    "miso_engine_web_v1_response_frame_submit",
    "miso_engine_web_v1_service",
    "miso_engine_web_v1_session_seek",
    "miso_engine_web_v1_session_seek_at",
    "miso_engine_web_v1_source_channels",
    "miso_engine_web_v1_source_count",
    "miso_engine_web_v1_source_frames",
    "miso_engine_web_v1_source_id",
    "miso_engine_web_v1_source_seek",
    "miso_engine_web_v1_source_seek_at",
    "miso_engine_web_v1_source_seek_report",
    "miso_engine_web_v1_source_seek_report_ptr",
    "miso_engine_web_v1_source_submit",
    "miso_engine_web_v1_spectrum_analysis",
    "miso_engine_web_v1_spectrum_arm",
    "miso_engine_web_v1_spectrum_cancel",
    "miso_engine_web_v1_spectrum_capture_bytes",
    "miso_engine_web_v1_spectrum_capture_capacity",
    "miso_engine_web_v1_spectrum_capture_ptr",
    "miso_engine_web_v1_spectrum_capture_set_bytes",
    "miso_engine_web_v1_spectrum_close",
    "miso_engine_web_v1_spectrum_collection_entry_bytes",
    "miso_engine_web_v1_spectrum_collection_entry_capacity",
    "miso_engine_web_v1_spectrum_collection_entry_ptr",
    "miso_engine_web_v1_spectrum_collection_request_bytes",
    "miso_engine_web_v1_spectrum_collection_request_ptr",
    "miso_engine_web_v1_spectrum_collection_target_ids_capacity",
    "miso_engine_web_v1_spectrum_collection_target_ids_ptr",
    "miso_engine_web_v1_spectrum_hop_capability",
    "miso_engine_web_v1_spectrum_read",
    "miso_engine_web_v1_spectrum_request_bytes",
    "miso_engine_web_v1_spectrum_request_ptr",
    "miso_engine_web_v1_spectrum_result_bytes",
    "miso_engine_web_v1_spectrum_result_ptr",
    "miso_engine_web_v1_spectrum_select",
    "miso_engine_web_v1_spectrum_selection_epoch",
    "miso_engine_web_v1_spectrum_stream_analysis",
    "miso_engine_web_v1_spectrum_stream_analysis_configure",
    "miso_engine_web_v1_spectrum_stream_metadata_bytes",
    "miso_engine_web_v1_spectrum_stream_metadata_ptr",
    "miso_engine_web_v1_spectrum_stream_read",
    "miso_engine_web_v1_spectrum_stream_reset",
    "miso_engine_web_v1_spectrum_stream_select",
    "miso_engine_web_v1_spectrum_stream_start",
    "miso_engine_web_v1_spectrum_stream_stop",
    "miso_engine_web_v1_spectrum_target_id_capacity",
    "miso_engine_web_v1_spectrum_target_id_ptr",
    "miso_engine_web_v1_status_ptr",
    "miso_engine_web_v1_track_response_analysis",
    "miso_engine_web_v1_track_response_capture",
    "miso_engine_web_v1_track_response_close",
    "miso_engine_web_v1_track_response_request_bytes",
    "miso_engine_web_v1_track_response_request_ptr",
    "miso_engine_web_v1_track_response_result_bytes",
    "miso_engine_web_v1_track_response_result_ptr",
    "miso_engine_web_v1_track_response_snapshot_capacity",
    "miso_engine_web_v1_track_response_snapshot_ptr",
    "miso_engine_web_v1_track_response_snapshot_set_bytes",
    "miso_engine_web_v1_track_response_track_id_capacity",
    "miso_engine_web_v1_track_response_track_id_ptr",
    "miso_engine_web_v1_watermark_ptr",
    "miso_engine_web_v1_watermark_read",
];

/// The boot staging sequence, by export name, in call order.
pub const STAGING_SEQUENCE: [&str; 4] = [
    "miso_engine_web_v1_abi_version",
    "miso_engine_web_v1_boot_options_ptr",
    "miso_engine_web_v1_document_ptr",
    "miso_engine_web_v1_boot",
];

type Field = (&'static str, usize, &'static str);

fn boot_options_fields() -> [Field; 11] {
    [
        ("structSize", offset_of!(WebBootOptions, struct_size), "u32"),
        ("abiVersion", offset_of!(WebBootOptions, abi_version), "u32"),
        (
            "requireSampleRateHz",
            offset_of!(WebBootOptions, require_sample_rate_hz),
            "u32",
        ),
        (
            "requireQuantumFrames",
            offset_of!(WebBootOptions, require_quantum_frames),
            "u32",
        ),
        (
            "sourceRingFrames",
            offset_of!(WebBootOptions, source_ring_frames),
            "u32",
        ),
        ("reserved0", offset_of!(WebBootOptions, reserved0), "u32"),
        (
            "maximumMemoryBytes",
            offset_of!(WebBootOptions, maximum_memory_bytes),
            "u64",
        ),
        ("reserved1", offset_of!(WebBootOptions, reserved1), "u64"),
        (
            "liveControlMeterBlocks",
            offset_of!(WebBootOptions, live_control_meter_blocks),
            "u64",
        ),
        (
            "liveControlObservationTaps",
            offset_of!(WebBootOptions, live_control_observation_taps),
            "u64",
        ),
        ("reserved2", offset_of!(WebBootOptions, reserved2), "u64"),
    ]
}

fn status_fields() -> [Field; 14] {
    [
        ("structSize", offset_of!(WebStatus, struct_size), "u32"),
        ("abiVersion", offset_of!(WebStatus, abi_version), "u32"),
        ("state", offset_of!(WebStatus, state), "u32"),
        ("lastResult", offset_of!(WebStatus, last_result), "u32"),
        ("backend", offset_of!(WebStatus, backend), "u32"),
        ("sampleRateHz", offset_of!(WebStatus, sample_rate_hz), "u32"),
        (
            "quantumFrames",
            offset_of!(WebStatus, quantum_frames),
            "u32",
        ),
        (
            "blockingRebuilds",
            offset_of!(WebStatus, blocking_rebuilds),
            "u32",
        ),
        (
            "nextAbsoluteSample",
            offset_of!(WebStatus, next_absolute_sample),
            "u64",
        ),
        (
            "renderedQuanta",
            offset_of!(WebStatus, rendered_quanta),
            "u64",
        ),
        (
            "liveValuesSuperseded",
            offset_of!(WebStatus, live_values_superseded),
            "u64",
        ),
        (
            "watermarkRevision",
            offset_of!(WebStatus, watermark_revision),
            "u64",
        ),
        (
            "watermarkFirstSample",
            offset_of!(WebStatus, watermark_first_sample),
            "u64",
        ),
        (
            "watermarkOutcomeFlags",
            offset_of!(WebStatus, watermark_outcome_flags),
            "u64",
        ),
    ]
}

fn resource_report_fields() -> [Field; 28] {
    [
        (
            "structSize",
            offset_of!(WebResourceReport, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebResourceReport, abi_version),
            "u32",
        ),
        (
            "sampleRateHz",
            offset_of!(WebResourceReport, sample_rate_hz),
            "u32",
        ),
        (
            "quantumFrames",
            offset_of!(WebResourceReport, quantum_frames),
            "u32",
        ),
        ("backend", offset_of!(WebResourceReport, backend), "u32"),
        (
            "reserved0",
            offset_of!(WebResourceReport, reserved0),
            "u32[3]",
        ),
        (
            "optionsBytes",
            offset_of!(WebResourceReport, options_bytes),
            "u64",
        ),
        (
            "statusBytes",
            offset_of!(WebResourceReport, status_bytes),
            "u64",
        ),
        (
            "sessionDocumentBytes",
            offset_of!(WebResourceReport, session_document_bytes),
            "u64",
        ),
        (
            "diagnosticBytes",
            offset_of!(WebResourceReport, diagnostic_bytes),
            "u64",
        ),
        (
            "idStagingBytes",
            offset_of!(WebResourceReport, id_staging_bytes),
            "u64",
        ),
        (
            "sourcePcmStagingBytes",
            offset_of!(WebResourceReport, source_pcm_staging_bytes),
            "u64",
        ),
        (
            "outputPcmBytes",
            offset_of!(WebResourceReport, output_pcm_bytes),
            "u64",
        ),
        (
            "bridgeMetadataBytes",
            offset_of!(WebResourceReport, bridge_metadata_bytes),
            "u64",
        ),
        (
            "bridgeRetainedBytes",
            offset_of!(WebResourceReport, bridge_retained_bytes),
            "u64",
        ),
        (
            "largestBridgeAllocationBytes",
            offset_of!(WebResourceReport, largest_bridge_allocation_bytes),
            "u64",
        ),
        (
            "sourceTotalBytes",
            offset_of!(WebResourceReport, source_total_bytes),
            "u64",
        ),
        (
            "sourceOverheadBytes",
            offset_of!(WebResourceReport, source_overhead_bytes),
            "u64",
        ),
        (
            "effectScalarStateBytes",
            offset_of!(WebResourceReport, effect_scalar_state_bytes),
            "u64",
        ),
        (
            "effectScalarScratchBytes",
            offset_of!(WebResourceReport, effect_scalar_scratch_bytes),
            "u64",
        ),
        (
            "builtinRetainedBytes",
            offset_of!(WebResourceReport, builtin_retained_bytes),
            "u64",
        ),
        (
            "graphSessionPlusPlanBytes",
            offset_of!(WebResourceReport, graph_session_plus_plan_bytes),
            "u64",
        ),
        (
            "graphIncrementalPlanBytes",
            offset_of!(WebResourceReport, graph_incremental_plan_bytes),
            "u64",
        ),
        (
            "graphMetadataBytes",
            offset_of!(WebResourceReport, graph_metadata_bytes),
            "u64",
        ),
        (
            "graphDelayBytes",
            offset_of!(WebResourceReport, graph_delay_bytes),
            "u64",
        ),
        (
            "largestNamedAllocationBytes",
            offset_of!(WebResourceReport, largest_named_allocation_bytes),
            "u64",
        ),
        (
            "observationRetainedBytes",
            offset_of!(WebResourceReport, observation_retained_bytes),
            "u64",
        ),
        (
            "reserved",
            offset_of!(WebResourceReport, reserved),
            "u64[3]",
        ),
    ]
}

fn meter_header_fields() -> [Field; 15] {
    [
        ("structSize", offset_of!(WebMeterHeader, struct_size), "u32"),
        ("abiVersion", offset_of!(WebMeterHeader, abi_version), "u32"),
        ("trackCount", offset_of!(WebMeterHeader, track_count), "u32"),
        ("windows", offset_of!(WebMeterHeader, windows), "u32"),
        (
            "firstSample",
            offset_of!(WebMeterHeader, first_sample),
            "u64",
        ),
        ("endSample", offset_of!(WebMeterHeader, end_sample), "u64"),
        ("sequence", offset_of!(WebMeterHeader, sequence), "u64"),
        (
            "masterTrackPlusOne",
            offset_of!(WebMeterHeader, master_track_plus_one),
            "u32",
        ),
        (
            "masterGrPresent",
            offset_of!(WebMeterHeader, master_gr_present),
            "u32",
        ),
        ("reserved", offset_of!(WebMeterHeader, reserved), "u64[2]"),
        (
            "submixCount",
            offset_of!(WebMeterHeader, submix_count),
            "u32",
        ),
        ("retired", offset_of!(WebMeterHeader, retired), "u32"),
        (
            "framePointer",
            offset_of!(WebMeterHeader, frame_pointer),
            "u32",
        ),
        (
            "successorPointer",
            offset_of!(WebMeterHeader, successor_pointer),
            "u32",
        ),
        (
            "structureEpoch",
            offset_of!(WebMeterHeader, structure_epoch),
            "u64",
        ),
    ]
}

/// The watermark record (issue #1381a D5), in the order of the C ABI's `miso_engine_v1_watermark`.
fn plan_watermark_fields() -> [Field; 9] {
    [
        (
            "structSize",
            offset_of!(WebPlanWatermark, struct_size),
            "u32",
        ),
        ("reserved0", offset_of!(WebPlanWatermark, reserved0), "u32"),
        ("revision", offset_of!(WebPlanWatermark, revision), "u64"),
        (
            "firstSample",
            offset_of!(WebPlanWatermark, first_sample),
            "u64",
        ),
        (
            "outcomeFlags",
            offset_of!(WebPlanWatermark, outcome_flags),
            "u64",
        ),
        (
            "exactCount",
            offset_of!(WebPlanWatermark, exact_count),
            "u64",
        ),
        (
            "transitionFallbackCount",
            offset_of!(WebPlanWatermark, transition_fallback_count),
            "u64",
        ),
        (
            "supersededCount",
            offset_of!(WebPlanWatermark, superseded_count),
            "u64",
        ),
        ("reserved", offset_of!(WebPlanWatermark, reserved), "u64[5]"),
    ]
}

/// The plan latency record (issue #1688 D2): the C ABI report's latency and tail words.
fn plan_latency_fields() -> [Field; 6] {
    [
        ("structSize", offset_of!(WebPlanLatency, struct_size), "u32"),
        ("reserved0", offset_of!(WebPlanLatency, reserved0), "u32"),
        (
            "latencySamples",
            offset_of!(WebPlanLatency, latency_samples),
            "u64",
        ),
        ("tailKind", offset_of!(WebPlanLatency, tail_kind), "u64"),
        (
            "tailSamples",
            offset_of!(WebPlanLatency, tail_samples),
            "u64",
        ),
        ("reserved", offset_of!(WebPlanLatency, reserved), "u64[4]"),
    ]
}

/// The source seek report record (issue #1293 D6): the C ABI seek report's layout.
fn seek_report_fields() -> [Field; 8] {
    [
        ("structSize", offset_of!(WebSeekReport, struct_size), "u32"),
        ("reserved0", offset_of!(WebSeekReport, reserved0), "u32"),
        ("generation", offset_of!(WebSeekReport, generation), "u64"),
        (
            "firstSample",
            offset_of!(WebSeekReport, first_sample),
            "u64",
        ),
        (
            "sourceFrame",
            offset_of!(WebSeekReport, source_frame),
            "u64",
        ),
        (
            "cumulativeUnderrunFrames",
            offset_of!(WebSeekReport, cumulative_underrun_frames),
            "u64",
        ),
        (
            "cumulativeHeldFrames",
            offset_of!(WebSeekReport, cumulative_held_frames),
            "u64",
        ),
        ("reserved", offset_of!(WebSeekReport, reserved), "u64[2]"),
    ]
}

fn command_report_fields() -> [Field; 8] {
    [
        (
            "structSize",
            offset_of!(WebCommandReport, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebCommandReport, abi_version),
            "u32",
        ),
        ("result", offset_of!(WebCommandReport, result), "u32"),
        ("reason", offset_of!(WebCommandReport, reason), "u32"),
        (
            "rejectedIndex",
            offset_of!(WebCommandReport, rejected_index),
            "u32",
        ),
        ("admitted", offset_of!(WebCommandReport, admitted), "u32"),
        (
            "appliedAtSample",
            offset_of!(WebCommandReport, applied_at_sample),
            "u64",
        ),
        ("reserved", offset_of!(WebCommandReport, reserved), "u64[2]"),
    ]
}

fn observation_selection_fields() -> [Field; 8] {
    [
        (
            "structSize",
            offset_of!(WebObservationSelection, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebObservationSelection, abi_version),
            "u32",
        ),
        (
            "trackIndex",
            offset_of!(WebObservationSelection, track_index),
            "u32",
        ),
        ("rack", offset_of!(WebObservationSelection, rack), "u32"),
        (
            "effectIndex",
            offset_of!(WebObservationSelection, effect_index),
            "u32",
        ),
        ("tapId", offset_of!(WebObservationSelection, tap_id), "u32"),
        (
            "channels",
            offset_of!(WebObservationSelection, channels),
            "u32",
        ),
        (
            "reserved",
            offset_of!(WebObservationSelection, reserved),
            "u32",
        ),
    ]
}

fn observation_result_fields() -> [Field; 19] {
    [
        (
            "structSize",
            offset_of!(WebObservationResult, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebObservationResult, abi_version),
            "u32",
        ),
        ("status", offset_of!(WebObservationResult, status), "u32"),
        (
            "trackIndex",
            offset_of!(WebObservationResult, track_index),
            "u32",
        ),
        ("rack", offset_of!(WebObservationResult, rack), "u32"),
        (
            "effectIndex",
            offset_of!(WebObservationResult, effect_index),
            "u32",
        ),
        ("tapId", offset_of!(WebObservationResult, tap_id), "u32"),
        (
            "channels",
            offset_of!(WebObservationResult, channels),
            "u32",
        ),
        (
            "sampleRateHz",
            offset_of!(WebObservationResult, sample_rate_hz),
            "u32",
        ),
        (
            "reserved0",
            offset_of!(WebObservationResult, reserved0),
            "u32",
        ),
        (
            "firstSample",
            offset_of!(WebObservationResult, first_sample),
            "u64",
        ),
        (
            "endSample",
            offset_of!(WebObservationResult, end_sample),
            "u64",
        ),
        (
            "sequence",
            offset_of!(WebObservationResult, sequence),
            "u64",
        ),
        ("blocks", offset_of!(WebObservationResult, blocks), "u32"),
        (
            "leftPresent",
            offset_of!(WebObservationResult, left_present),
            "u32",
        ),
        (
            "rightPresent",
            offset_of!(WebObservationResult, right_present),
            "u32",
        ),
        ("left", offset_of!(WebObservationResult, left), "f32"),
        ("right", offset_of!(WebObservationResult, right), "f32"),
        (
            "reserved",
            offset_of!(WebObservationResult, reserved),
            "u32[3]",
        ),
    ]
}

/// The staged `miso.command.v1` record, whose offsets are a decode rule rather than a `#[repr(C)]`
/// structure.
///
/// `CommandRecord::decode` reads these byte positions out of the staging buffer directly — there
/// is no Rust type to take `offset_of!` of — so the rows are transcribed from that function and
/// pinned against it by `tests/abi_layout.rs`, which encodes a record at these offsets and
/// requires the engine to decode exactly the values written.
fn command_record_fields() -> [Field; 9] {
    [
        ("kind", 0, "u8"),
        ("rack", 1, "u8"),
        ("stripIdBytes", 2, "u8"),
        ("effectIdBytes", 3, "u8"),
        ("tapId", 4, "u32"),
        ("windowBlocks", 8, "u32"),
        ("reserved0", 12, "u32"),
        ("stripId", 16, "u8[128]"),
        ("effectId", 144, "u8[128]"),
    ]
}

fn live_response_request_fields() -> [Field; 10] {
    [
        (
            "structSize",
            offset_of!(WebLiveResponseRequest, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebLiveResponseRequest, abi_version),
            "u32",
        ),
        (
            "trackIdBytes",
            offset_of!(WebLiveResponseRequest, track_id_bytes),
            "u32",
        ),
        ("grid", offset_of!(WebLiveResponseRequest, grid), "u32"),
        (
            "channels",
            offset_of!(WebLiveResponseRequest, channels),
            "u32",
        ),
        ("points", offset_of!(WebLiveResponseRequest, points), "u32"),
        (
            "minimumHz",
            offset_of!(WebLiveResponseRequest, minimum_hz),
            "f32",
        ),
        (
            "maximumHz",
            offset_of!(WebLiveResponseRequest, maximum_hz),
            "f32",
        ),
        (
            "maximumResultBytes",
            offset_of!(WebLiveResponseRequest, maximum_result_bytes),
            "u32",
        ),
        (
            "reserved",
            offset_of!(WebLiveResponseRequest, reserved),
            "u32[3]",
        ),
    ]
}

fn spectrum_request_fields() -> [Field; 8] {
    [
        (
            "structSize",
            offset_of!(WebSpectrumRequest, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebSpectrumRequest, abi_version),
            "u32",
        ),
        ("target", offset_of!(WebSpectrumRequest, target), "u32"),
        ("channels", offset_of!(WebSpectrumRequest, channels), "u32"),
        (
            "targetIdBytes",
            offset_of!(WebSpectrumRequest, target_id_bytes),
            "u32",
        ),
        (
            "reserved0",
            offset_of!(WebSpectrumRequest, reserved0),
            "u32",
        ),
        (
            "maximumCaptureBytes",
            offset_of!(WebSpectrumRequest, maximum_capture_bytes),
            "u64",
        ),
        (
            "reserved",
            offset_of!(WebSpectrumRequest, reserved),
            "u32[2]",
        ),
    ]
}

fn spectrum_collection_request_fields() -> [Field; 6] {
    [
        (
            "structSize",
            offset_of!(WebSpectrumCollectionRequest, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebSpectrumCollectionRequest, abi_version),
            "u32",
        ),
        (
            "entryCount",
            offset_of!(WebSpectrumCollectionRequest, entry_count),
            "u32",
        ),
        (
            "reserved0",
            offset_of!(WebSpectrumCollectionRequest, reserved0),
            "u32",
        ),
        (
            "maximumCaptureBytes",
            offset_of!(WebSpectrumCollectionRequest, maximum_capture_bytes),
            "u64",
        ),
        (
            "reserved",
            offset_of!(WebSpectrumCollectionRequest, reserved),
            "u32[2]",
        ),
    ]
}

fn spectrum_collection_entry_fields() -> [Field; 4] {
    [
        (
            "target",
            offset_of!(WebSpectrumCollectionEntry, target),
            "u32",
        ),
        (
            "channels",
            offset_of!(WebSpectrumCollectionEntry, channels),
            "u32",
        ),
        (
            "targetIdBytes",
            offset_of!(WebSpectrumCollectionEntry, target_id_bytes),
            "u32",
        ),
        (
            "reserved",
            offset_of!(WebSpectrumCollectionEntry, reserved),
            "u32[3]",
        ),
    ]
}

fn spectrum_window_fields() -> [Field; 13] {
    [
        (
            "structSize",
            offset_of!(WebSpectrumWindow, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebSpectrumWindow, abi_version),
            "u32",
        ),
        ("target", offset_of!(WebSpectrumWindow, target), "u32"),
        ("channels", offset_of!(WebSpectrumWindow, channels), "u32"),
        (
            "sampleRateHz",
            offset_of!(WebSpectrumWindow, sample_rate_hz),
            "u32",
        ),
        ("frames", offset_of!(WebSpectrumWindow, frames), "u32"),
        (
            "sourceUnderrun",
            offset_of!(WebSpectrumWindow, source_underrun),
            "u32",
        ),
        ("reserved0", offset_of!(WebSpectrumWindow, reserved0), "u32"),
        (
            "capturedSample",
            offset_of!(WebSpectrumWindow, captured_sample),
            "u64",
        ),
        (
            "endSample",
            offset_of!(WebSpectrumWindow, end_sample),
            "u64",
        ),
        (
            "snapshotToken",
            offset_of!(WebSpectrumWindow, snapshot_token),
            "u64",
        ),
        (
            "leftOffset",
            offset_of!(WebSpectrumWindow, left_offset),
            "u32",
        ),
        (
            "rightOffset",
            offset_of!(WebSpectrumWindow, right_offset),
            "u32",
        ),
    ]
}

fn spectrum_result_fields() -> [Field; 18] {
    [
        (
            "structSize",
            offset_of!(WebSpectrumResult, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebSpectrumResult, abi_version),
            "u32",
        ),
        ("result", offset_of!(WebSpectrumResult, result), "u32"),
        ("target", offset_of!(WebSpectrumResult, target), "u32"),
        ("channels", offset_of!(WebSpectrumResult, channels), "u32"),
        (
            "sampleRateHz",
            offset_of!(WebSpectrumResult, sample_rate_hz),
            "u32",
        ),
        (
            "windowFrames",
            offset_of!(WebSpectrumResult, window_frames),
            "u32",
        ),
        ("binCount", offset_of!(WebSpectrumResult, bin_count), "u32"),
        (
            "sourceUnderrun",
            offset_of!(WebSpectrumResult, source_underrun),
            "u32",
        ),
        ("floorDb", offset_of!(WebSpectrumResult, floor_db), "f32"),
        (
            "capturedSample",
            offset_of!(WebSpectrumResult, captured_sample),
            "u64",
        ),
        (
            "endSample",
            offset_of!(WebSpectrumResult, end_sample),
            "u64",
        ),
        (
            "snapshotToken",
            offset_of!(WebSpectrumResult, snapshot_token),
            "u64",
        ),
        (
            "resultBytes",
            offset_of!(WebSpectrumResult, result_bytes),
            "u64",
        ),
        (
            "frequenciesOffset",
            offset_of!(WebSpectrumResult, frequencies_offset),
            "u32",
        ),
        (
            "leftOffset",
            offset_of!(WebSpectrumResult, left_offset),
            "u32",
        ),
        (
            "rightOffset",
            offset_of!(WebSpectrumResult, right_offset),
            "u32",
        ),
        ("reserved0", offset_of!(WebSpectrumResult, reserved0), "u32"),
    ]
}

fn spectrum_stream_metadata_fields() -> [Field; 21] {
    [
        (
            "structSize",
            offset_of!(WebSpectrumStreamMetadata, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebSpectrumStreamMetadata, abi_version),
            "u32",
        ),
        (
            "result",
            offset_of!(WebSpectrumStreamMetadata, result),
            "u32",
        ),
        (
            "status",
            offset_of!(WebSpectrumStreamMetadata, status),
            "u32",
        ),
        (
            "target",
            offset_of!(WebSpectrumStreamMetadata, target),
            "u32",
        ),
        (
            "channels",
            offset_of!(WebSpectrumStreamMetadata, channels),
            "u32",
        ),
        (
            "sampleRateHz",
            offset_of!(WebSpectrumStreamMetadata, sample_rate_hz),
            "u32",
        ),
        (
            "quantumFrames",
            offset_of!(WebSpectrumStreamMetadata, quantum_frames),
            "u32",
        ),
        (
            "hopFrames",
            offset_of!(WebSpectrumStreamMetadata, hop_frames),
            "u32",
        ),
        (
            "sourceUnderrun",
            offset_of!(WebSpectrumStreamMetadata, source_underrun),
            "u32",
        ),
        (
            "reserved0",
            offset_of!(WebSpectrumStreamMetadata, reserved0),
            "u32",
        ),
        (
            "reserved1",
            offset_of!(WebSpectrumStreamMetadata, reserved1),
            "u32",
        ),
        (
            "captureEpoch",
            offset_of!(WebSpectrumStreamMetadata, capture_epoch),
            "u64",
        ),
        (
            "sequence",
            offset_of!(WebSpectrumStreamMetadata, sequence),
            "u64",
        ),
        (
            "droppedCaptures",
            offset_of!(WebSpectrumStreamMetadata, dropped_captures),
            "u64",
        ),
        (
            "windows",
            offset_of!(WebSpectrumStreamMetadata, windows),
            "u64",
        ),
        (
            "capturedSample",
            offset_of!(WebSpectrumStreamMetadata, captured_sample),
            "u64",
        ),
        (
            "endSample",
            offset_of!(WebSpectrumStreamMetadata, end_sample),
            "u64",
        ),
        (
            "analysisEpoch",
            offset_of!(WebSpectrumStreamMetadata, analysis_epoch),
            "u64",
        ),
        (
            "historyStartSample",
            offset_of!(WebSpectrumStreamMetadata, history_start_sample),
            "u64",
        ),
        (
            "smoothingMs",
            offset_of!(WebSpectrumStreamMetadata, smoothing_ms),
            "f64",
        ),
    ]
}

fn live_response_owner_fields() -> [Field; 16] {
    [
        (
            "trackIdOffset",
            offset_of!(WebLiveResponseOwner, track_id_offset),
            "u32",
        ),
        (
            "trackIdBytes",
            offset_of!(WebLiveResponseOwner, track_id_bytes),
            "u32",
        ),
        (
            "nativeIdOffset",
            offset_of!(WebLiveResponseOwner, native_id_offset),
            "u32",
        ),
        (
            "nativeIdBytes",
            offset_of!(WebLiveResponseOwner, native_id_bytes),
            "u32",
        ),
        (
            "stableIdOffset",
            offset_of!(WebLiveResponseOwner, stable_id_offset),
            "u32",
        ),
        (
            "stableIdBytes",
            offset_of!(WebLiveResponseOwner, stable_id_bytes),
            "u32",
        ),
        ("rack", offset_of!(WebLiveResponseOwner, rack), "u32"),
        ("slot", offset_of!(WebLiveResponseOwner, slot), "u32"),
        ("kind", offset_of!(WebLiveResponseOwner, kind), "u32"),
        (
            "bypassed",
            offset_of!(WebLiveResponseOwner, bypassed),
            "u32",
        ),
        (
            "availability",
            offset_of!(WebLiveResponseOwner, availability),
            "u32",
        ),
        (
            "leftOffset",
            offset_of!(WebLiveResponseOwner, left_offset),
            "u32",
        ),
        (
            "leftCount",
            offset_of!(WebLiveResponseOwner, left_count),
            "u32",
        ),
        (
            "rightOffset",
            offset_of!(WebLiveResponseOwner, right_offset),
            "u32",
        ),
        (
            "rightCount",
            offset_of!(WebLiveResponseOwner, right_count),
            "u32",
        ),
        (
            "reserved",
            offset_of!(WebLiveResponseOwner, reserved),
            "u32[1]",
        ),
    ]
}

fn live_response_section_fields() -> [Field; 5] {
    [
        ("id", offset_of!(WebLiveResponseSection, id), "u32"),
        ("kind", offset_of!(WebLiveResponseSection, kind), "u32"),
        (
            "enabled",
            offset_of!(WebLiveResponseSection, enabled),
            "u32",
        ),
        (
            "wordCount",
            offset_of!(WebLiveResponseSection, word_count),
            "u32",
        ),
        ("words", offset_of!(WebLiveResponseSection, words), "u32[7]"),
    ]
}

fn live_response_result_fields() -> [Field; 22] {
    [
        (
            "structSize",
            offset_of!(WebLiveResponseResult, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebLiveResponseResult, abi_version),
            "u32",
        ),
        ("result", offset_of!(WebLiveResponseResult, result), "u32"),
        ("mode", offset_of!(WebLiveResponseResult, mode), "u32"),
        ("meaning", offset_of!(WebLiveResponseResult, meaning), "u32"),
        (
            "channels",
            offset_of!(WebLiveResponseResult, channels),
            "u32",
        ),
        ("points", offset_of!(WebLiveResponseResult, points), "u32"),
        (
            "ownerCount",
            offset_of!(WebLiveResponseResult, owner_count),
            "u32",
        ),
        (
            "excludedCount",
            offset_of!(WebLiveResponseResult, excluded_count),
            "u32",
        ),
        (
            "sampleRateHz",
            offset_of!(WebLiveResponseResult, sample_rate_hz),
            "u32",
        ),
        (
            "reserved0",
            offset_of!(WebLiveResponseResult, reserved0),
            "u32",
        ),
        (
            "reserved1",
            offset_of!(WebLiveResponseResult, reserved1),
            "u32",
        ),
        (
            "capturedSample",
            offset_of!(WebLiveResponseResult, captured_sample),
            "u64",
        ),
        (
            "snapshotToken",
            offset_of!(WebLiveResponseResult, snapshot_token),
            "u64",
        ),
        (
            "resultBytes",
            offset_of!(WebLiveResponseResult, result_bytes),
            "u64",
        ),
        (
            "frequenciesOffset",
            offset_of!(WebLiveResponseResult, frequencies_offset),
            "u32",
        ),
        (
            "leftOffset",
            offset_of!(WebLiveResponseResult, left_offset),
            "u32",
        ),
        (
            "rightOffset",
            offset_of!(WebLiveResponseResult, right_offset),
            "u32",
        ),
        (
            "ownersOffset",
            offset_of!(WebLiveResponseResult, owners_offset),
            "u32",
        ),
        (
            "ownerRecordBytes",
            offset_of!(WebLiveResponseResult, owner_record_bytes),
            "u32",
        ),
        (
            "sectionRecordBytes",
            offset_of!(WebLiveResponseResult, section_record_bytes),
            "u32",
        ),
        (
            "reserved",
            offset_of!(WebLiveResponseResult, reserved),
            "u32[2]",
        ),
    ]
}

/// Render the whole document. Deterministic: every table below is a fixed array.
#[must_use]
pub fn render() -> String {
    let results = [
        (RESULT_OK, "ok"),
        (RESULT_INVALID_ARGUMENT, "invalidArgument"),
        (RESULT_ABI_MISMATCH, "abiMismatch"),
        (RESULT_WRONG_STATE, "wrongState"),
        (RESULT_BUFFER_TOO_SMALL, "bufferTooSmall"),
        (RESULT_REFUSED_BUDGET, "refusedBudget"),
        (RESULT_BACKPRESSURE, "backpressure"),
        (RESULT_UNSUPPORTED, "unsupported"),
        (RESULT_RENDER_REJECTED, "renderRejected"),
        (RESULT_REPREPARE_REQUIRED, "reprepareRequired"),
        (RESULT_COMPILE_REJECTED, "compileRejected"),
        (RESULT_INTERNAL, "internal"),
    ];
    // Scoped to `miso_engine_web_v1_boot`'s return value. Transcribed from the alias constants, so
    // a renumbering of the base codes moves these with them.
    let boot_aliases = [
        (RESULT_REFUSED_DOCUMENT, "refusedDocument"),
        (RESULT_REFUSED_OPTIONS, "refusedOptions"),
        (RESULT_REFUSED_LIFECYCLE, "refusedLifecycle"),
    ];
    let states = [
        (STATE_READY, "ready"),
        (STATE_FAILED, "failed"),
        (STATE_DISPOSED, "disposed"),
    ];
    let backends = [(BACKEND_SCALAR, "scalar"), (BACKEND_SIMD128, "simd128")];
    let buffers = [
        (BUFFER_SOURCE_ID, "sourceId"),
        (BUFFER_SOURCE_PCM, "sourcePcm"),
        (BUFFER_DIAGNOSTIC, "diagnostic"),
        (BUFFER_OUTPUT_PCM, "outputPcm"),
        (BUFFER_COMMAND, "command"),
        (BUFFER_METER_FRAME, "meterFrame"),
    ];
    let wire_command_kinds = [
        (COMMAND_OBSERVE_SUBSCRIBE, "observeSubscribe"),
        (COMMAND_OBSERVE_UNSUBSCRIBE, "observeUnsubscribe"),
    ];
    let command_reasons = [
        (COMMAND_REASON_NONE, "none"),
        (COMMAND_REASON_MALFORMED, "malformed"),
        (COMMAND_REASON_UNKNOWN_TRACK, "unknownTrack"),
        (COMMAND_REASON_UNKNOWN_RACK, "unknownRack"),
        (COMMAND_REASON_UNKNOWN_EFFECT, "unknownEffect"),
        (COMMAND_REASON_UNKNOWN_PARAMETER, "unknownParameter"),
        (COMMAND_REASON_DOMAIN, "domain"),
        (COMMAND_REASON_UNSUPPORTED_KIND, "unsupportedKind"),
        (COMMAND_REASON_BACKPRESSURE, "backpressure"),
        (COMMAND_REASON_WRONG_STATE, "wrongState"),
        (COMMAND_REASON_UNKNOWN_TAP, "unknownTap"),
        (COMMAND_REASON_OBSERVATION_UNBOUND, "observationUnbound"),
        // 12-14 are retired with the value kinds (#1565A) and never reallocated; #1676 added 15.
        (COMMAND_REASON_OBSERVATION_LIMIT, "observationLimit"),
    ];

    let named = |values: &[(u32, &'static str)]| {
        values
            .iter()
            .map(|&(value, name)| NamedValue { value, name })
            .collect()
    };
    let structure = |bytes: u32, fields: &[Field]| Structure {
        bytes,
        fields: field_rows(fields),
    };
    let document = Document {
        schema: SCHEMA,
        abi_version: ABI_VERSION,
        staging_sequence: &STAGING_SEQUENCE,
        error_phases: &ERROR_PHASES,
        exports: &EXPORTS,
        structures: Entries(
            [
                (
                    "bootOptions",
                    structure(BOOT_OPTIONS_BYTES, &boot_options_fields()),
                ),
                ("status", structure(STATUS_BYTES, &status_fields())),
                (
                    "resourceReport",
                    structure(RESOURCE_REPORT_BYTES, &resource_report_fields()),
                ),
                (
                    "meterHeader",
                    structure(METER_HEADER_BYTES, &meter_header_fields()),
                ),
                (
                    "commandReport",
                    structure(COMMAND_REPORT_BYTES, &command_report_fields()),
                ),
                (
                    "planWatermark",
                    structure(WATERMARK_BYTES, &plan_watermark_fields()),
                ),
                (
                    "planLatency",
                    structure(PLAN_LATENCY_BYTES, &plan_latency_fields()),
                ),
                (
                    "seekReport",
                    structure(SEEK_REPORT_BYTES, &seek_report_fields()),
                ),
                (
                    "observationSelection",
                    structure(OBSERVATION_SELECTION_BYTES, &observation_selection_fields()),
                ),
                (
                    "observationResult",
                    structure(OBSERVATION_RESULT_BYTES, &observation_result_fields()),
                ),
                (
                    "liveResponseRequest",
                    structure(LIVE_RESPONSE_REQUEST_BYTES, &live_response_request_fields()),
                ),
                (
                    "spectrumRequest",
                    structure(SPECTRUM_REQUEST_BYTES, &spectrum_request_fields()),
                ),
                (
                    "spectrumCollectionRequest",
                    structure(
                        SPECTRUM_COLLECTION_REQUEST_BYTES,
                        &spectrum_collection_request_fields(),
                    ),
                ),
                (
                    "spectrumCollectionEntry",
                    structure(
                        SPECTRUM_COLLECTION_ENTRY_BYTES,
                        &spectrum_collection_entry_fields(),
                    ),
                ),
                (
                    "spectrumWindow",
                    structure(SPECTRUM_WINDOW_HEADER_BYTES, &spectrum_window_fields()),
                ),
                (
                    "spectrumResult",
                    structure(SPECTRUM_RESULT_HEADER_BYTES, &spectrum_result_fields()),
                ),
                (
                    "spectrumStreamMetadata",
                    structure(
                        SPECTRUM_STREAM_METADATA_BYTES,
                        &spectrum_stream_metadata_fields(),
                    ),
                ),
                (
                    "liveResponseOwner",
                    structure(LIVE_RESPONSE_OWNER_BYTES, &live_response_owner_fields()),
                ),
                (
                    "liveResponseSection",
                    structure(LIVE_RESPONSE_SECTION_BYTES, &live_response_section_fields()),
                ),
                (
                    "liveResponseResult",
                    structure(LIVE_RESPONSE_RESULT_BYTES, &live_response_result_fields()),
                ),
            ]
            .into_iter()
            .map(|(name, structure)| (name.to_owned(), structure))
            .collect(),
        ),
        command_record: CommandRecord {
            bytes: COMMAND_RECORD_BYTES,
            endianness: "little",
            fields: field_rows(&command_record_fields()),
        },
        constants: Constants {
            result_codes: named(&results),
            boot_result_aliases: named(&boot_aliases),
            states: named(&states),
            backends: named(&backends),
            buffer_kinds: named(&buffers),
            wire_command_kinds: named(&wire_command_kinds),
            command_reasons: named(&command_reasons),
            // Decision 12 (#1096): the `rack` of the command, observation, companion and
            // configuration records. The retired `simd1` (0) and `simd2` (2) codes are absent,
            // never reallocated.
            racks: named(&[
                (u32::from(RACK_INSERTS), "inserts"),
                (u32::from(RACK_CONSOLE), "console"),
                (u32::from(RACK_NOT_APPLICABLE), "notApplicable"),
            ]),
            // The live-response owner record's `rack`; the retired `1` and `3` are absent.
            live_response_racks: named(&[
                (LIVE_RESPONSE_RACK_INPUT_FILTERS, "inputFilters"),
                (LIVE_RESPONSE_RACK_INSERTS, "inserts"),
                (LIVE_RESPONSE_RACK_CONSOLE, "console"),
            ]),
            observation_channels: named(&[
                (OBSERVATION_CHANNEL_LEFT, "left"),
                (OBSERVATION_CHANNEL_RIGHT, "right"),
                (OBSERVATION_CHANNEL_BOTH, "both"),
            ]),
            observation_statuses: named(&[
                (OBSERVATION_STATUS_PENDING, "pending"),
                (OBSERVATION_STATUS_UNARMED, "unarmed"),
                (OBSERVATION_STATUS_READY, "ready"),
            ]),
            response_grids: named(&[
                (RESPONSE_GRID_LINEAR, "linear"),
                (RESPONSE_GRID_LOGARITHMIC, "logarithmic"),
            ]),
            response_channels: named(&[
                (RESPONSE_CHANNEL_LEFT, "left"),
                (RESPONSE_CHANNEL_RIGHT, "right"),
                (RESPONSE_CHANNEL_BOTH, "both"),
            ]),
            live_response_modes: named(&[(LIVE_RESPONSE_MODE_TARGET, "target")]),
            live_response_meanings: named(&[(
                LIVE_RESPONSE_MEANING_EQ_FILTER_SUBTOTAL,
                "eqFilterSubtotal",
            )]),
            spectrum_targets: named(&[
                (SPECTRUM_TARGET_TRACK_POST_INPUT, "trackPostInput"),
                (SPECTRUM_TARGET_TRACK_POST_PAN, "trackPostPan"),
                (SPECTRUM_TARGET_OUTPUT, "output"),
            ]),
            spectrum_channels: named(&[
                (SPECTRUM_CHANNEL_LEFT, "left"),
                (SPECTRUM_CHANNEL_RIGHT, "right"),
                (SPECTRUM_CHANNEL_BOTH, "both"),
            ]),
            spectrum_stream_statuses: named(&[
                (SPECTRUM_STREAM_STATUS_INACTIVE, "inactive"),
                (SPECTRUM_STREAM_STATUS_WARMING, "warming"),
                (SPECTRUM_STREAM_STATUS_PENDING, "pending"),
                (SPECTRUM_STREAM_STATUS_GAP, "gap"),
                (SPECTRUM_STREAM_STATUS_FAILED, "failed"),
                (SPECTRUM_STREAM_STATUS_STOPPED, "stopped"),
                (SPECTRUM_STREAM_STATUS_READY, "ready"),
            ]),
            maximum_command_records: MAXIMUM_COMMAND_RECORDS,
            maximum_document_bytes: MAXIMUM_DOCUMENT_BYTES,
            maximum_source_channels: session::SOURCE_CHANNELS_LAUNCH_MAXIMUM,
            diagnostic_bytes: DIAGNOSTIC_BYTES,
            default_meter_blocks: DEFAULT_METER_BLOCKS,
            maximum_observation_taps: MAXIMUM_OBSERVATION_TAPS,
            maximum_response_points: RESPONSE_MAXIMUM_POINTS,
            maximum_live_response_owners: LIVE_RESPONSE_MAXIMUM_OWNERS,
            maximum_live_response_sections: LIVE_RESPONSE_MAXIMUM_SECTIONS,
            maximum_live_response_id_bytes: LIVE_RESPONSE_MAXIMUM_ID_BYTES,
            maximum_live_response_points: LIVE_RESPONSE_MAXIMUM_POINTS,
            live_response_capture_bytes: LIVE_RESPONSE_CAPTURE_BYTES,
            spectrum_capture_bytes: SPECTRUM_CAPTURE_BYTES,
            maximum_prepared_spectrum_targets: SPECTRUM_MAXIMUM_PREPARED_TARGETS,
            spectrum_collection_request_bytes: SPECTRUM_COLLECTION_REQUEST_BYTES,
            spectrum_collection_entry_bytes: SPECTRUM_COLLECTION_ENTRY_BYTES,
            spectrum_collection_entry_capacity: SPECTRUM_COLLECTION_ENTRY_CAPACITY,
            spectrum_collection_target_ids_bytes: SPECTRUM_COLLECTION_TARGET_IDS_BYTES,
            spectrum_request_bytes: SPECTRUM_REQUEST_BYTES,
            spectrum_window_header_bytes: SPECTRUM_WINDOW_HEADER_BYTES,
            spectrum_result_header_bytes: SPECTRUM_RESULT_HEADER_BYTES,
            spectrum_stream_metadata_bytes: SPECTRUM_STREAM_METADATA_BYTES,
            spectrum_window_frames: SPECTRUM_WINDOW_FRAMES,
            spectrum_bin_count: SPECTRUM_BIN_COUNT,
            maximum_spectrum_id_bytes: SPECTRUM_MAXIMUM_ID_BYTES,
            default_maximum_memory_bytes: DEFAULT_MAXIMUM_MEMORY_BYTES,
            source_ring: SourceRing {
                stall_tolerance_ms: SOURCE_STALL_TOLERANCE_MS,
                reserve_quanta: SOURCE_RING_RESERVE_QUANTA,
            },
        },
    };
    json::document(&document)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Document<'a> {
    schema: &'a str,
    abi_version: u32,
    staging_sequence: &'a [&'a str],
    error_phases: &'a [&'a str],
    exports: &'a [&'a str],
    structures: Entries<Structure<'a>>,
    command_record: CommandRecord<'a>,
    constants: Constants<'a>,
}

#[derive(Serialize)]
struct Structure<'a> {
    bytes: u32,
    fields: Vec<FieldRow<'a>>,
}

#[derive(Serialize)]
struct CommandRecord<'a> {
    bytes: u32,
    endianness: &'a str,
    fields: Vec<FieldRow<'a>>,
}

#[derive(Serialize)]
struct FieldRow<'a> {
    name: &'a str,
    offset: usize,
    #[serde(rename = "type")]
    kind: &'a str,
}

#[derive(Serialize)]
struct NamedValue<'a> {
    value: u32,
    name: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SourceRing {
    stall_tolerance_ms: u32,
    reserve_quanta: u32,
}

/// The `constants` object: the named code tables, then the scalar bounds, then `sourceRing`.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Constants<'a> {
    result_codes: Vec<NamedValue<'a>>,
    boot_result_aliases: Vec<NamedValue<'a>>,
    states: Vec<NamedValue<'a>>,
    backends: Vec<NamedValue<'a>>,
    buffer_kinds: Vec<NamedValue<'a>>,
    wire_command_kinds: Vec<NamedValue<'a>>,
    command_reasons: Vec<NamedValue<'a>>,
    racks: Vec<NamedValue<'a>>,
    live_response_racks: Vec<NamedValue<'a>>,
    observation_channels: Vec<NamedValue<'a>>,
    observation_statuses: Vec<NamedValue<'a>>,
    response_grids: Vec<NamedValue<'a>>,
    response_channels: Vec<NamedValue<'a>>,
    live_response_modes: Vec<NamedValue<'a>>,
    live_response_meanings: Vec<NamedValue<'a>>,
    spectrum_targets: Vec<NamedValue<'a>>,
    spectrum_channels: Vec<NamedValue<'a>>,
    spectrum_stream_statuses: Vec<NamedValue<'a>>,
    maximum_command_records: u32,
    maximum_document_bytes: u32,
    maximum_source_channels: u32,
    diagnostic_bytes: u32,
    default_meter_blocks: u32,
    maximum_observation_taps: u32,
    maximum_response_points: usize,
    maximum_live_response_owners: usize,
    maximum_live_response_sections: usize,
    maximum_live_response_id_bytes: usize,
    maximum_live_response_points: usize,
    live_response_capture_bytes: usize,
    spectrum_capture_bytes: usize,
    maximum_prepared_spectrum_targets: u32,
    spectrum_collection_request_bytes: u32,
    spectrum_collection_entry_bytes: u32,
    spectrum_collection_entry_capacity: u32,
    spectrum_collection_target_ids_bytes: u32,
    spectrum_request_bytes: u32,
    spectrum_window_header_bytes: u32,
    spectrum_result_header_bytes: u32,
    spectrum_stream_metadata_bytes: u32,
    spectrum_window_frames: u32,
    spectrum_bin_count: u32,
    maximum_spectrum_id_bytes: usize,
    default_maximum_memory_bytes: u64,
    source_ring: SourceRing,
}

fn field_rows(fields: &[Field]) -> Vec<FieldRow<'static>> {
    fields
        .iter()
        .map(|&(name, offset, kind)| FieldRow { name, offset, kind })
        .collect()
}
