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

use core::mem::{offset_of, size_of};

use host_web::{
    ABI_VERSION, BACKEND_SCALAR, BACKEND_SIMD128, BOOT_OPTIONS_BYTES, BUFFER_COMMAND,
    BUFFER_DIAGNOSTIC, BUFFER_METER_FRAME, BUFFER_OUTPUT_PCM, BUFFER_SOURCE_ID, BUFFER_SOURCE_PCM,
    COMMAND_EFFECT_BYPASS, COMMAND_EFFECT_PARAM, COMMAND_FADER_DB, COMMAND_INPUT_FILTERS,
    COMMAND_MATRIX, COMMAND_MUTE, COMMAND_OBSERVE_SUBSCRIBE, COMMAND_OBSERVE_UNSUBSCRIBE,
    COMMAND_PAN, COMMAND_POLARITY_INVERT, COMMAND_REASON_BACKPRESSURE, COMMAND_REASON_DOMAIN,
    COMMAND_REASON_MALFORMED, COMMAND_REASON_NONE, COMMAND_REASON_OBSERVATION_UNBOUND,
    COMMAND_REASON_UNKNOWN_EFFECT, COMMAND_REASON_UNKNOWN_PARAMETER, COMMAND_REASON_UNKNOWN_RACK,
    COMMAND_REASON_UNKNOWN_TAP, COMMAND_REASON_UNKNOWN_TRACK, COMMAND_REASON_UNSUPPORTED_KIND,
    COMMAND_REASON_WRONG_STATE, COMMAND_RECORD_BYTES, COMMAND_REPORT_BYTES, COMMAND_SOLO,
    COMMAND_TRIM_DB, DEFAULT_COMMAND_QUEUE_RECORDS, DEFAULT_MAXIMUM_MEMORY_BYTES,
    DEFAULT_METER_BLOCKS, DIAGNOSTIC_BYTES, LIVE_RESPONSE_CAPTURE_BYTES,
    LIVE_RESPONSE_MAXIMUM_ID_BYTES, LIVE_RESPONSE_MAXIMUM_OWNERS, LIVE_RESPONSE_MAXIMUM_POINTS,
    LIVE_RESPONSE_MAXIMUM_SECTIONS, LIVE_RESPONSE_MEANING_EQ_FILTER_SUBTOTAL,
    LIVE_RESPONSE_MODE_TARGET, LIVE_RESPONSE_OWNER_BYTES, LIVE_RESPONSE_REQUEST_BYTES,
    LIVE_RESPONSE_RESULT_BYTES, LIVE_RESPONSE_SECTION_BYTES, MAXIMUM_COMMAND_RECORDS,
    MAXIMUM_DOCUMENT_BYTES, MAXIMUM_OBSERVATION_TAPS, METER_HEADER_BYTES,
    OBSERVATION_ADMISSION_MAXIMUM, OBSERVATION_ADMISSION_PENDING_BOUNDARY,
    OBSERVATION_ADMISSION_RECEIPT, OBSERVATION_ADMISSION_REQUESTED,
    OBSERVATION_CAPTURE_FLAG_GRAPH_GENERATION, OBSERVATION_CAPTURE_KIND_RESPONSE,
    OBSERVATION_CAPTURE_KIND_SPECTRUM, OBSERVATION_CHANNEL_BOTH, OBSERVATION_CHANNEL_LEFT,
    OBSERVATION_CHANNEL_RIGHT, OBSERVATION_OPERATION_CAPTURE_RESPONSE,
    OBSERVATION_OPERATION_COLLECTION_SELECTION, OBSERVATION_OPERATION_METER_LEASE,
    OBSERVATION_OPERATION_METER_READ, OBSERVATION_OPERATION_ONE_SHOT,
    OBSERVATION_OPERATION_RAW_OBSERVATION_BATCH, OBSERVATION_OPERATION_READ_SPECTRUM,
    OBSERVATION_OPERATION_REMOVE_METERS_TO, OBSERVATION_OPERATION_REPLACE_METERS,
    OBSERVATION_OPERATION_RESIDENT_READ, OBSERVATION_OPERATION_RESTART_SPECTRUM,
    OBSERVATION_OPERATION_START_SPECTRUM, OBSERVATION_OPERATION_STOP_GRAPH,
    OBSERVATION_OPERATION_STOP_SPECTRUM, OBSERVATION_PROFILE_EQ_SPECTRUM,
    OBSERVATION_PROFILE_LEGACY_UNPROTECTED, OBSERVATION_RECEIPT_DOMAIN_GRAPH,
    OBSERVATION_RECEIPT_DOMAIN_RESIDENT, OBSERVATION_RECEIPT_STATE_APPLIED,
    OBSERVATION_RECEIPT_STATE_CLOSED, OBSERVATION_RECEIPT_STATE_FAILED,
    OBSERVATION_RECEIPT_STATE_PENDING, OBSERVATION_REFUSAL_REASON_ARITHMETIC_OVERFLOW,
    OBSERVATION_REFUSAL_REASON_BACKPRESSURE, OBSERVATION_REFUSAL_REASON_CAPACITY,
    OBSERVATION_REFUSAL_REASON_CLOSED, OBSERVATION_REFUSAL_REASON_CONFLICT,
    OBSERVATION_REFUSAL_REASON_INVALID_REQUEST, OBSERVATION_REFUSAL_REASON_NONE,
    OBSERVATION_REFUSAL_REASON_NOT_PREPARED, OBSERVATION_REFUSAL_REASON_REVISION_EXHAUSTED,
    OBSERVATION_REFUSAL_REASON_WORK_BUDGET, OBSERVATION_REFUSAL_REASON_WRONG_OWNER,
    OBSERVATION_RESULT_BYTES, OBSERVATION_SELECTION_BYTES,
    OBSERVATION_STATUS_FLAG_ORDINARY_AVAILABLE, OBSERVATION_STATUS_FLAG_REMOVAL_AVAILABLE,
    OBSERVATION_STATUS_FLAG_RENDER_FAILED, OBSERVATION_STATUS_FLAG_TERMINAL,
    OBSERVATION_STATUS_PENDING, OBSERVATION_STATUS_READY, OBSERVATION_STATUS_UNARMED,
    RESOURCE_REPORT_BYTES, RESPONSE_CHANNEL_BOTH, RESPONSE_CHANNEL_LEFT, RESPONSE_CHANNEL_RIGHT,
    RESPONSE_FIELD_SECTIONS, RESPONSE_FIELD_TOTAL, RESPONSE_GRID_LINEAR, RESPONSE_GRID_LOGARITHMIC,
    RESPONSE_MAXIMUM_EFFECT_ID_BYTES, RESPONSE_MAXIMUM_PARAMETER_OVERRIDES,
    RESPONSE_MAXIMUM_RESULT_BYTES, RESPONSE_PARAMETER_BYTES, RESPONSE_REQUEST_BYTES,
    RESPONSE_RESULT_BYTES, RESPONSE_TARGET_EFFECT, RESPONSE_TARGET_INPUT_FILTERS,
    RESULT_ABI_MISMATCH, RESULT_BACKPRESSURE, RESULT_BUFFER_TOO_SMALL, RESULT_INTERNAL,
    RESULT_INVALID_ARGUMENT, RESULT_OK, RESULT_REFUSED_BUDGET, RESULT_REFUSED_DOCUMENT,
    RESULT_REFUSED_LIFECYCLE, RESULT_REFUSED_OPTIONS, RESULT_RENDER_REJECTED,
    RESULT_REPREPARE_REQUIRED, RESULT_UNSUPPORTED, RESULT_WRONG_STATE, SOURCE_STALL_TOLERANCE_MS,
    SPECTRUM_BIN_COUNT, SPECTRUM_CAPTURE_BYTES, SPECTRUM_CHANNEL_BOTH, SPECTRUM_CHANNEL_LEFT,
    SPECTRUM_CHANNEL_RIGHT, SPECTRUM_COLLECTION_ENTRY_BYTES, SPECTRUM_COLLECTION_ENTRY_CAPACITY,
    SPECTRUM_COLLECTION_REQUEST_BYTES, SPECTRUM_COLLECTION_TARGET_IDS_BYTES,
    SPECTRUM_MAXIMUM_ID_BYTES, SPECTRUM_MAXIMUM_PREPARED_TARGETS, SPECTRUM_REQUEST_BYTES,
    SPECTRUM_RESULT_HEADER_BYTES, SPECTRUM_STREAM_METADATA_BYTES, SPECTRUM_STREAM_STATUS_FAILED,
    SPECTRUM_STREAM_STATUS_GAP, SPECTRUM_STREAM_STATUS_INACTIVE, SPECTRUM_STREAM_STATUS_PENDING,
    SPECTRUM_STREAM_STATUS_READY, SPECTRUM_STREAM_STATUS_STOPPED, SPECTRUM_STREAM_STATUS_WARMING,
    SPECTRUM_TARGET_OUTPUT, SPECTRUM_TARGET_TRACK_POST_INPUT_BUILTINS,
    SPECTRUM_TARGET_TRACK_POST_MATRIX, SPECTRUM_WINDOW_FRAMES, SPECTRUM_WINDOW_HEADER_BYTES,
    STATE_DISPOSED, STATE_FAILED, STATE_READY, STATUS_BYTES, WebBootOptions, WebBuiltinInputConfig,
    WebCommandReport, WebEqTargetConfig, WebEqTargetEdit, WebEqTargetRequest, WebEqTargetResult,
    WebInputFilterEdit, WebLiveResponseOwner, WebLiveResponseRequest, WebLiveResponseResult,
    WebLiveResponseSection, WebMeterHeader, WebObservationAdmission, WebObservationCaptureIdentity,
    WebObservationDemand, WebObservationIngressLimits, WebObservationPreparationRecord,
    WebObservationReceipt, WebObservationResult, WebObservationSelection, WebObservationStatus,
    WebObservationWorkLimits, WebPreparedEffectCompanionHeader, WebPreparedEffectCompanionRecord,
    WebPreparedEffectTarget, WebResourceReport, WebResponseParameter, WebResponseRequest,
    WebResponseResult, WebSpectrumCollectionEntry, WebSpectrumCollectionRequest,
    WebSpectrumRequest, WebSpectrumResult, WebSpectrumStreamMetadata, WebSpectrumWindow, WebStatus,
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
pub const EXPORTS: [&str; 131] = [
    "miso_engine_web_v1_abi_version",
    "miso_engine_web_v1_boot",
    "miso_engine_web_v1_boot_diagnostic_bytes",
    "miso_engine_web_v1_boot_options_ptr",
    "miso_engine_web_v1_boot_result",
    "miso_engine_web_v1_boot_with_observation_demand",
    "miso_engine_web_v1_buffer_capacity",
    "miso_engine_web_v1_buffer_ptr",
    "miso_engine_web_v1_command_report_ptr",
    "miso_engine_web_v1_command_submit",
    "miso_engine_web_v1_console_track_count",
    "miso_engine_web_v1_console_track_id",
    "miso_engine_web_v1_dispose",
    "miso_engine_web_v1_document_ptr",
    "miso_engine_web_v1_eq_target_close",
    "miso_engine_web_v1_eq_target_config_copy",
    "miso_engine_web_v1_eq_target_config_ptr",
    "miso_engine_web_v1_eq_target_open",
    "miso_engine_web_v1_eq_target_prepare",
    "miso_engine_web_v1_eq_target_rejected_edit_index",
    "miso_engine_web_v1_eq_target_rejected_reason",
    "miso_engine_web_v1_eq_target_request_capacity",
    "miso_engine_web_v1_eq_target_request_ptr",
    "miso_engine_web_v1_eq_target_result_bytes",
    "miso_engine_web_v1_eq_target_result_capacity",
    "miso_engine_web_v1_eq_target_result_ptr",
    "miso_engine_web_v1_input_filters_config_copy",
    "miso_engine_web_v1_input_filters_prepare",
    "miso_engine_web_v1_meter_header_ptr",
    "miso_engine_web_v1_meter_lease",
    "miso_engine_web_v1_meter_poll",
    "miso_engine_web_v1_observation_admission_bytes",
    "miso_engine_web_v1_observation_admission_ptr",
    "miso_engine_web_v1_observation_application_bytes",
    "miso_engine_web_v1_observation_application_capacity",
    "miso_engine_web_v1_observation_application_ptr",
    "miso_engine_web_v1_observation_application_take",
    "miso_engine_web_v1_observation_capture_identity_bytes",
    "miso_engine_web_v1_observation_capture_identity_ptr",
    "miso_engine_web_v1_observation_count",
    "miso_engine_web_v1_observation_demand_apply",
    "miso_engine_web_v1_observation_demand_bytes",
    "miso_engine_web_v1_observation_demand_capacity",
    "miso_engine_web_v1_observation_demand_ptr",
    "miso_engine_web_v1_observation_effect_index",
    "miso_engine_web_v1_observation_effect_slot_id",
    "miso_engine_web_v1_observation_id_capacity",
    "miso_engine_web_v1_observation_id_ptr",
    "miso_engine_web_v1_observation_native_effect_id",
    "miso_engine_web_v1_observation_preparation_bytes",
    "miso_engine_web_v1_observation_preparation_ptr",
    "miso_engine_web_v1_observation_rack",
    "miso_engine_web_v1_observation_read",
    "miso_engine_web_v1_observation_result_bytes",
    "miso_engine_web_v1_observation_result_ptr",
    "miso_engine_web_v1_observation_selection_bytes",
    "miso_engine_web_v1_observation_selection_capacity",
    "miso_engine_web_v1_observation_selection_ptr",
    "miso_engine_web_v1_observation_status_bytes",
    "miso_engine_web_v1_observation_status_ptr",
    "miso_engine_web_v1_observation_tap_count",
    "miso_engine_web_v1_observation_tap_id",
    "miso_engine_web_v1_observation_track_index",
    "miso_engine_web_v1_prepared_command_submit",
    "miso_engine_web_v1_prepared_companion_capacity",
    "miso_engine_web_v1_prepared_companion_ptr",
    "miso_engine_web_v1_render",
    "miso_engine_web_v1_resource_ptr",
    "miso_engine_web_v1_response_close",
    "miso_engine_web_v1_response_effect_id_capacity",
    "miso_engine_web_v1_response_effect_id_ptr",
    "miso_engine_web_v1_response_parameter_bytes",
    "miso_engine_web_v1_response_parameter_capacity",
    "miso_engine_web_v1_response_parameter_ptr",
    "miso_engine_web_v1_response_query",
    "miso_engine_web_v1_response_request_bytes",
    "miso_engine_web_v1_response_request_ptr",
    "miso_engine_web_v1_response_result_bytes",
    "miso_engine_web_v1_response_result_ptr",
    "miso_engine_web_v1_source_channels",
    "miso_engine_web_v1_source_count",
    "miso_engine_web_v1_source_frames",
    "miso_engine_web_v1_source_id",
    "miso_engine_web_v1_source_seek",
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
        (
            "consoleCommandQueueRecords",
            offset_of!(WebBootOptions, console_command_queue_records),
            "u64",
        ),
        (
            "consoleMeterBlocks",
            offset_of!(WebBootOptions, console_meter_blocks),
            "u64",
        ),
        (
            "consoleObservationTaps",
            offset_of!(WebBootOptions, console_observation_taps),
            "u64",
        ),
        (
            "consoleMasterTrackPlusOne",
            offset_of!(WebBootOptions, console_master_track_plus_one),
            "u64",
        ),
    ]
}

fn status_fields() -> [Field; 11] {
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
        ("reserved0", offset_of!(WebStatus, reserved0), "u32"),
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
        ("reserved", offset_of!(WebStatus, reserved), "u64[4]"),
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

fn meter_header_fields() -> [Field; 10] {
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

fn observation_status_fields() -> [Field; 11] {
    [
        (
            "structSize",
            offset_of!(WebObservationStatus, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebObservationStatus, abi_version),
            "u32",
        ),
        ("profile", offset_of!(WebObservationStatus, profile), "u32"),
        ("flags", offset_of!(WebObservationStatus, flags), "u32"),
        (
            "pendingCount",
            offset_of!(WebObservationStatus, pending_count),
            "u32",
        ),
        (
            "reserved",
            offset_of!(WebObservationStatus, reserved),
            "u32",
        ),
        ("owner", offset_of!(WebObservationStatus, owner), "u64"),
        (
            "ingressEpoch",
            offset_of!(WebObservationStatus, ingress_epoch),
            "u64",
        ),
        (
            "acceptedGeneration",
            offset_of!(WebObservationStatus, accepted_generation),
            "u64",
        ),
        (
            "appliedGeneration",
            offset_of!(WebObservationStatus, applied_generation),
            "u64",
        ),
        (
            "selectionEpoch",
            offset_of!(WebObservationStatus, selection_epoch),
            "u64",
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

fn observation_work_limits_fields() -> [Field; 13] {
    [
        (
            "structSize",
            offset_of!(WebObservationWorkLimits, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebObservationWorkLimits, abi_version),
            "u32",
        ),
        (
            "maximumActiveMeterChannels",
            offset_of!(WebObservationWorkLimits, maximum_active_meter_channels),
            "u64",
        ),
        (
            "maximumMeterSamplesPerBlock",
            offset_of!(WebObservationWorkLimits, maximum_meter_samples_per_block),
            "u64",
        ),
        (
            "maximumMeterPublicationsPerBlock",
            offset_of!(
                WebObservationWorkLimits,
                maximum_meter_publications_per_block
            ),
            "u64",
        ),
        (
            "maximumMeterPublicationBytesPerBlock",
            offset_of!(
                WebObservationWorkLimits,
                maximum_meter_publication_bytes_per_block
            ),
            "u64",
        ),
        (
            "maximumActiveSpectrumCaptures",
            offset_of!(WebObservationWorkLimits, maximum_active_spectrum_captures),
            "u64",
        ),
        (
            "maximumCaptureInputSamplesPerBlock",
            offset_of!(
                WebObservationWorkLimits,
                maximum_capture_input_samples_per_block
            ),
            "u64",
        ),
        (
            "maximumCaptureCopySamplesPerBlock",
            offset_of!(
                WebObservationWorkLimits,
                maximum_capture_copy_samples_per_block
            ),
            "u64",
        ),
        (
            "maximumCapturePublicationsPerBlock",
            offset_of!(
                WebObservationWorkLimits,
                maximum_capture_publications_per_block
            ),
            "u64",
        ),
        (
            "maximumCaptureBytesPerSecond",
            offset_of!(WebObservationWorkLimits, maximum_capture_bytes_per_second),
            "u64",
        ),
        (
            "maximumTransitionEntryVisitsPerBlock",
            offset_of!(
                WebObservationWorkLimits,
                maximum_transition_entry_visits_per_block
            ),
            "u64",
        ),
        (
            "maximumRetainedBytes",
            offset_of!(WebObservationWorkLimits, maximum_retained_bytes),
            "u64",
        ),
    ]
}

fn observation_ingress_limits_fields() -> [Field; 15] {
    let removal_end =
        offset_of!(WebObservationIngressLimits, removal_operations_per_boundary) + size_of::<u32>();
    let admission_start = offset_of!(WebObservationIngressLimits, maximum_admission_entry_visits);
    let alignment_padding_bytes = admission_start - removal_end;
    assert_eq!(
        alignment_padding_bytes, 4,
        "the ingress record's alignment gap must be four bytes"
    );
    [
        (
            "structSize",
            offset_of!(WebObservationIngressLimits, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebObservationIngressLimits, abi_version),
            "u32",
        ),
        (
            "maximumControlBytes",
            offset_of!(WebObservationIngressLimits, maximum_control_bytes),
            "u32",
        ),
        (
            "maximumObservationRows",
            offset_of!(WebObservationIngressLimits, maximum_observation_rows),
            "u32",
        ),
        (
            "maximumResultBytes",
            offset_of!(WebObservationIngressLimits, maximum_result_bytes),
            "u32",
        ),
        (
            "ordinaryOperationsPerBoundary",
            offset_of!(
                WebObservationIngressLimits,
                ordinary_operations_per_boundary
            ),
            "u32",
        ),
        (
            "removalOperationsPerBoundary",
            offset_of!(WebObservationIngressLimits, removal_operations_per_boundary),
            "u32",
        ),
        ("alignmentPadding", removal_end, "u8[4]"),
        ("maximumAdmissionEntryVisits", admission_start, "u64"),
        (
            "maximumResponseBindingVisits",
            offset_of!(WebObservationIngressLimits, maximum_response_binding_visits),
            "u64",
        ),
        (
            "maximumResponseSectionVisits",
            offset_of!(WebObservationIngressLimits, maximum_response_section_visits),
            "u64",
        ),
        (
            "maximumResponseCopyBytes",
            offset_of!(WebObservationIngressLimits, maximum_response_copy_bytes),
            "u64",
        ),
        (
            "maximumHandlerCopyBytesPerBoundary",
            offset_of!(
                WebObservationIngressLimits,
                maximum_handler_copy_bytes_per_boundary
            ),
            "u64",
        ),
        (
            "maximumCleanupEntryVisitsPerBoundary",
            offset_of!(
                WebObservationIngressLimits,
                maximum_cleanup_entry_visits_per_boundary
            ),
            "u64",
        ),
        (
            "maximumRetainedBytes",
            offset_of!(WebObservationIngressLimits, maximum_retained_bytes),
            "u64",
        ),
    ]
}

const OBSERVATION_PREPARATION_WORK_LIMIT_NAMES: [&str; 13] = [
    "workLimits.structSize",
    "workLimits.abiVersion",
    "workLimits.maximumActiveMeterChannels",
    "workLimits.maximumMeterSamplesPerBlock",
    "workLimits.maximumMeterPublicationsPerBlock",
    "workLimits.maximumMeterPublicationBytesPerBlock",
    "workLimits.maximumActiveSpectrumCaptures",
    "workLimits.maximumCaptureInputSamplesPerBlock",
    "workLimits.maximumCaptureCopySamplesPerBlock",
    "workLimits.maximumCapturePublicationsPerBlock",
    "workLimits.maximumCaptureBytesPerSecond",
    "workLimits.maximumTransitionEntryVisitsPerBlock",
    "workLimits.maximumRetainedBytes",
];

const OBSERVATION_PREPARATION_INGRESS_LIMIT_NAMES: [&str; 15] = [
    "ingressLimits.structSize",
    "ingressLimits.abiVersion",
    "ingressLimits.maximumControlBytes",
    "ingressLimits.maximumObservationRows",
    "ingressLimits.maximumResultBytes",
    "ingressLimits.ordinaryOperationsPerBoundary",
    "ingressLimits.removalOperationsPerBoundary",
    "ingressLimits.alignmentPadding",
    "ingressLimits.maximumAdmissionEntryVisits",
    "ingressLimits.maximumResponseBindingVisits",
    "ingressLimits.maximumResponseSectionVisits",
    "ingressLimits.maximumResponseCopyBytes",
    "ingressLimits.maximumHandlerCopyBytesPerBoundary",
    "ingressLimits.maximumCleanupEntryVisitsPerBoundary",
    "ingressLimits.maximumRetainedBytes",
];

const OBSERVATION_PREPARATION_SPECTRUM_REQUEST_NAMES: [&str; 8] = [
    "spectrumRequest.structSize",
    "spectrumRequest.abiVersion",
    "spectrumRequest.target",
    "spectrumRequest.channels",
    "spectrumRequest.targetIdBytes",
    "spectrumRequest.reserved0",
    "spectrumRequest.maximumCaptureBytes",
    "spectrumRequest.reserved",
];

fn observation_preparation_fields() -> [Field; 46] {
    let work_limits = observation_work_limits_fields();
    let ingress_limits = observation_ingress_limits_fields();
    let spectrum_request = spectrum_request_fields();

    let work_limits_offset = offset_of!(WebObservationPreparationRecord, work_limits);
    let ingress_limits_offset = offset_of!(WebObservationPreparationRecord, ingress_limits);
    let spectrum_request_offset = offset_of!(WebObservationPreparationRecord, spectrum_request);
    let target_id_offset = offset_of!(WebObservationPreparationRecord, target_id);

    assert_eq!(size_of::<WebObservationPreparationRecord>(), 392);
    assert_eq!(
        work_limits.len(),
        OBSERVATION_PREPARATION_WORK_LIMIT_NAMES.len()
    );
    assert_eq!(
        ingress_limits.len(),
        OBSERVATION_PREPARATION_INGRESS_LIMIT_NAMES.len()
    );
    assert_eq!(
        spectrum_request.len(),
        OBSERVATION_PREPARATION_SPECTRUM_REQUEST_NAMES.len()
    );
    assert_eq!(
        offset_of!(
            WebObservationPreparationRecord,
            activation_maximum_retained_bytes
        ) + size_of::<u64>(),
        work_limits_offset
    );
    assert_eq!(
        work_limits_offset + size_of::<WebObservationWorkLimits>(),
        ingress_limits_offset
    );
    assert_eq!(
        ingress_limits_offset + size_of::<WebObservationIngressLimits>(),
        spectrum_request_offset
    );
    assert_eq!(
        spectrum_request_offset + size_of::<WebSpectrumRequest>(),
        target_id_offset
    );
    assert_eq!(
        target_id_offset + 128,
        size_of::<WebObservationPreparationRecord>()
    );
    assert_eq!(work_limits[0].1, 0);
    assert_eq!(
        work_limits[12].1 + size_of::<u64>(),
        size_of::<WebObservationWorkLimits>()
    );
    assert_eq!(ingress_limits[0].1, 0);
    assert_eq!(
        ingress_limits[14].1 + size_of::<u64>(),
        size_of::<WebObservationIngressLimits>()
    );
    assert_eq!(spectrum_request[0].1, 0);
    assert_eq!(
        spectrum_request[7].1 + size_of::<[u32; 2]>(),
        size_of::<WebSpectrumRequest>()
    );

    [
        (
            "structSize",
            offset_of!(WebObservationPreparationRecord, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebObservationPreparationRecord, abi_version),
            "u32",
        ),
        (
            "profile",
            offset_of!(WebObservationPreparationRecord, profile),
            "u32",
        ),
        (
            "meterCount",
            offset_of!(WebObservationPreparationRecord, meter_count),
            "u32",
        ),
        (
            "residentTaps",
            offset_of!(WebObservationPreparationRecord, resident_taps),
            "u32",
        ),
        (
            "spectrumCount",
            offset_of!(WebObservationPreparationRecord, spectrum_count),
            "u32",
        ),
        (
            "maximumActiveObservers",
            offset_of!(WebObservationPreparationRecord, maximum_active_observers),
            "u32",
        ),
        (
            "reserved0",
            offset_of!(WebObservationPreparationRecord, reserved0),
            "u32",
        ),
        (
            "activationMaximumRetainedBytes",
            offset_of!(
                WebObservationPreparationRecord,
                activation_maximum_retained_bytes
            ),
            "u64",
        ),
        (
            OBSERVATION_PREPARATION_WORK_LIMIT_NAMES[0],
            work_limits_offset + work_limits[0].1,
            work_limits[0].2,
        ),
        (
            OBSERVATION_PREPARATION_WORK_LIMIT_NAMES[1],
            work_limits_offset + work_limits[1].1,
            work_limits[1].2,
        ),
        (
            OBSERVATION_PREPARATION_WORK_LIMIT_NAMES[2],
            work_limits_offset + work_limits[2].1,
            work_limits[2].2,
        ),
        (
            OBSERVATION_PREPARATION_WORK_LIMIT_NAMES[3],
            work_limits_offset + work_limits[3].1,
            work_limits[3].2,
        ),
        (
            OBSERVATION_PREPARATION_WORK_LIMIT_NAMES[4],
            work_limits_offset + work_limits[4].1,
            work_limits[4].2,
        ),
        (
            OBSERVATION_PREPARATION_WORK_LIMIT_NAMES[5],
            work_limits_offset + work_limits[5].1,
            work_limits[5].2,
        ),
        (
            OBSERVATION_PREPARATION_WORK_LIMIT_NAMES[6],
            work_limits_offset + work_limits[6].1,
            work_limits[6].2,
        ),
        (
            OBSERVATION_PREPARATION_WORK_LIMIT_NAMES[7],
            work_limits_offset + work_limits[7].1,
            work_limits[7].2,
        ),
        (
            OBSERVATION_PREPARATION_WORK_LIMIT_NAMES[8],
            work_limits_offset + work_limits[8].1,
            work_limits[8].2,
        ),
        (
            OBSERVATION_PREPARATION_WORK_LIMIT_NAMES[9],
            work_limits_offset + work_limits[9].1,
            work_limits[9].2,
        ),
        (
            OBSERVATION_PREPARATION_WORK_LIMIT_NAMES[10],
            work_limits_offset + work_limits[10].1,
            work_limits[10].2,
        ),
        (
            OBSERVATION_PREPARATION_WORK_LIMIT_NAMES[11],
            work_limits_offset + work_limits[11].1,
            work_limits[11].2,
        ),
        (
            OBSERVATION_PREPARATION_WORK_LIMIT_NAMES[12],
            work_limits_offset + work_limits[12].1,
            work_limits[12].2,
        ),
        (
            OBSERVATION_PREPARATION_INGRESS_LIMIT_NAMES[0],
            ingress_limits_offset + ingress_limits[0].1,
            ingress_limits[0].2,
        ),
        (
            OBSERVATION_PREPARATION_INGRESS_LIMIT_NAMES[1],
            ingress_limits_offset + ingress_limits[1].1,
            ingress_limits[1].2,
        ),
        (
            OBSERVATION_PREPARATION_INGRESS_LIMIT_NAMES[2],
            ingress_limits_offset + ingress_limits[2].1,
            ingress_limits[2].2,
        ),
        (
            OBSERVATION_PREPARATION_INGRESS_LIMIT_NAMES[3],
            ingress_limits_offset + ingress_limits[3].1,
            ingress_limits[3].2,
        ),
        (
            OBSERVATION_PREPARATION_INGRESS_LIMIT_NAMES[4],
            ingress_limits_offset + ingress_limits[4].1,
            ingress_limits[4].2,
        ),
        (
            OBSERVATION_PREPARATION_INGRESS_LIMIT_NAMES[5],
            ingress_limits_offset + ingress_limits[5].1,
            ingress_limits[5].2,
        ),
        (
            OBSERVATION_PREPARATION_INGRESS_LIMIT_NAMES[6],
            ingress_limits_offset + ingress_limits[6].1,
            ingress_limits[6].2,
        ),
        (
            OBSERVATION_PREPARATION_INGRESS_LIMIT_NAMES[7],
            ingress_limits_offset + ingress_limits[7].1,
            ingress_limits[7].2,
        ),
        (
            OBSERVATION_PREPARATION_INGRESS_LIMIT_NAMES[8],
            ingress_limits_offset + ingress_limits[8].1,
            ingress_limits[8].2,
        ),
        (
            OBSERVATION_PREPARATION_INGRESS_LIMIT_NAMES[9],
            ingress_limits_offset + ingress_limits[9].1,
            ingress_limits[9].2,
        ),
        (
            OBSERVATION_PREPARATION_INGRESS_LIMIT_NAMES[10],
            ingress_limits_offset + ingress_limits[10].1,
            ingress_limits[10].2,
        ),
        (
            OBSERVATION_PREPARATION_INGRESS_LIMIT_NAMES[11],
            ingress_limits_offset + ingress_limits[11].1,
            ingress_limits[11].2,
        ),
        (
            OBSERVATION_PREPARATION_INGRESS_LIMIT_NAMES[12],
            ingress_limits_offset + ingress_limits[12].1,
            ingress_limits[12].2,
        ),
        (
            OBSERVATION_PREPARATION_INGRESS_LIMIT_NAMES[13],
            ingress_limits_offset + ingress_limits[13].1,
            ingress_limits[13].2,
        ),
        (
            OBSERVATION_PREPARATION_INGRESS_LIMIT_NAMES[14],
            ingress_limits_offset + ingress_limits[14].1,
            ingress_limits[14].2,
        ),
        (
            OBSERVATION_PREPARATION_SPECTRUM_REQUEST_NAMES[0],
            spectrum_request_offset + spectrum_request[0].1,
            spectrum_request[0].2,
        ),
        (
            OBSERVATION_PREPARATION_SPECTRUM_REQUEST_NAMES[1],
            spectrum_request_offset + spectrum_request[1].1,
            spectrum_request[1].2,
        ),
        (
            OBSERVATION_PREPARATION_SPECTRUM_REQUEST_NAMES[2],
            spectrum_request_offset + spectrum_request[2].1,
            spectrum_request[2].2,
        ),
        (
            OBSERVATION_PREPARATION_SPECTRUM_REQUEST_NAMES[3],
            spectrum_request_offset + spectrum_request[3].1,
            spectrum_request[3].2,
        ),
        (
            OBSERVATION_PREPARATION_SPECTRUM_REQUEST_NAMES[4],
            spectrum_request_offset + spectrum_request[4].1,
            spectrum_request[4].2,
        ),
        (
            OBSERVATION_PREPARATION_SPECTRUM_REQUEST_NAMES[5],
            spectrum_request_offset + spectrum_request[5].1,
            spectrum_request[5].2,
        ),
        (
            OBSERVATION_PREPARATION_SPECTRUM_REQUEST_NAMES[6],
            spectrum_request_offset + spectrum_request[6].1,
            spectrum_request[6].2,
        ),
        (
            OBSERVATION_PREPARATION_SPECTRUM_REQUEST_NAMES[7],
            spectrum_request_offset + spectrum_request[7].1,
            spectrum_request[7].2,
        ),
        ("targetId", target_id_offset, "u8[128]"),
    ]
}

fn observation_demand_fields() -> [Field; 6] {
    [
        (
            "structSize",
            offset_of!(WebObservationDemand, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebObservationDemand, abi_version),
            "u32",
        ),
        (
            "operation",
            offset_of!(WebObservationDemand, operation),
            "u32",
        ),
        ("count", offset_of!(WebObservationDemand, count), "u32"),
        ("owner", offset_of!(WebObservationDemand, owner), "u64"),
        (
            "reserved",
            offset_of!(WebObservationDemand, reserved),
            "u32[2]",
        ),
    ]
}

fn observation_receipt_fields() -> [Field; 9] {
    [
        (
            "structSize",
            offset_of!(WebObservationReceipt, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebObservationReceipt, abi_version),
            "u32",
        ),
        ("domain", offset_of!(WebObservationReceipt, domain), "u32"),
        ("state", offset_of!(WebObservationReceipt, state), "u32"),
        ("owner", offset_of!(WebObservationReceipt, owner), "u64"),
        (
            "sequence",
            offset_of!(WebObservationReceipt, sequence),
            "u64",
        ),
        (
            "applicationSample",
            offset_of!(WebObservationReceipt, application_sample),
            "u64",
        ),
        ("result", offset_of!(WebObservationReceipt, result), "u32"),
        (
            "reserved",
            offset_of!(WebObservationReceipt, reserved),
            "u32",
        ),
    ]
}

const OBSERVATION_ADMISSION_RECEIPT_NAMES: [&str; 9] = [
    "receipt.structSize",
    "receipt.abiVersion",
    "receipt.domain",
    "receipt.state",
    "receipt.owner",
    "receipt.sequence",
    "receipt.applicationSample",
    "receipt.result",
    "receipt.reserved",
];

fn observation_admission_fields() -> [Field; 21] {
    let receipt = observation_receipt_fields();
    let limit_offset = offset_of!(WebObservationAdmission, limit);
    let receipt_offset = offset_of!(WebObservationAdmission, receipt);

    assert_eq!(size_of::<WebObservationAdmission>(), 232);
    assert_eq!(receipt.len(), OBSERVATION_ADMISSION_RECEIPT_NAMES.len());
    assert_eq!(receipt[0].1, 0);
    assert_eq!(
        receipt[receipt.len() - 1].1 + size_of::<u32>(),
        size_of::<WebObservationReceipt>()
    );
    assert_eq!(
        limit_offset + size_of::<[u8; 128]>(),
        receipt_offset,
        "the admission limit ends at the nested receipt"
    );
    assert_eq!(
        receipt_offset + size_of::<WebObservationReceipt>(),
        size_of::<WebObservationAdmission>()
    );

    [
        (
            "structSize",
            offset_of!(WebObservationAdmission, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebObservationAdmission, abi_version),
            "u32",
        ),
        ("result", offset_of!(WebObservationAdmission, result), "u32"),
        (
            "operation",
            offset_of!(WebObservationAdmission, operation),
            "u32",
        ),
        ("flags", offset_of!(WebObservationAdmission, flags), "u32"),
        ("reason", offset_of!(WebObservationAdmission, reason), "u32"),
        (
            "limitBytes",
            offset_of!(WebObservationAdmission, limit_bytes),
            "u32",
        ),
        (
            "reserved",
            offset_of!(WebObservationAdmission, reserved),
            "u32",
        ),
        (
            "ingressEpoch",
            offset_of!(WebObservationAdmission, ingress_epoch),
            "u64",
        ),
        (
            "requested",
            offset_of!(WebObservationAdmission, requested),
            "u64",
        ),
        (
            "maximum",
            offset_of!(WebObservationAdmission, maximum),
            "u64",
        ),
        ("limit", limit_offset, "u8[128]"),
        (
            OBSERVATION_ADMISSION_RECEIPT_NAMES[0],
            receipt_offset + receipt[0].1,
            receipt[0].2,
        ),
        (
            OBSERVATION_ADMISSION_RECEIPT_NAMES[1],
            receipt_offset + receipt[1].1,
            receipt[1].2,
        ),
        (
            OBSERVATION_ADMISSION_RECEIPT_NAMES[2],
            receipt_offset + receipt[2].1,
            receipt[2].2,
        ),
        (
            OBSERVATION_ADMISSION_RECEIPT_NAMES[3],
            receipt_offset + receipt[3].1,
            receipt[3].2,
        ),
        (
            OBSERVATION_ADMISSION_RECEIPT_NAMES[4],
            receipt_offset + receipt[4].1,
            receipt[4].2,
        ),
        (
            OBSERVATION_ADMISSION_RECEIPT_NAMES[5],
            receipt_offset + receipt[5].1,
            receipt[5].2,
        ),
        (
            OBSERVATION_ADMISSION_RECEIPT_NAMES[6],
            receipt_offset + receipt[6].1,
            receipt[6].2,
        ),
        (
            OBSERVATION_ADMISSION_RECEIPT_NAMES[7],
            receipt_offset + receipt[7].1,
            receipt[7].2,
        ),
        (
            OBSERVATION_ADMISSION_RECEIPT_NAMES[8],
            receipt_offset + receipt[8].1,
            receipt[8].2,
        ),
    ]
}

fn observation_capture_identity_fields() -> [Field; 8] {
    [
        (
            "structSize",
            offset_of!(WebObservationCaptureIdentity, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebObservationCaptureIdentity, abi_version),
            "u32",
        ),
        (
            "kind",
            offset_of!(WebObservationCaptureIdentity, kind),
            "u32",
        ),
        (
            "flags",
            offset_of!(WebObservationCaptureIdentity, flags),
            "u32",
        ),
        (
            "owner",
            offset_of!(WebObservationCaptureIdentity, owner),
            "u64",
        ),
        (
            "observationGeneration",
            offset_of!(WebObservationCaptureIdentity, observation_generation),
            "u64",
        ),
        (
            "selectionEpoch",
            offset_of!(WebObservationCaptureIdentity, selection_epoch),
            "u64",
        ),
        (
            "snapshotToken",
            offset_of!(WebObservationCaptureIdentity, snapshot_token),
            "u64",
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
fn command_record_fields() -> [Field; 11] {
    [
        ("kind", 0, "u8"),
        ("rack", 1, "u8"),
        ("channel", 2, "u8"),
        ("reserved0", 3, "u8"),
        ("trackIndex", 4, "u32"),
        ("effectIndex", 8, "u32"),
        ("parameterId", 12, "u32"),
        ("smoothingSamples", 16, "u32"),
        ("reserved1", 20, "u32"),
        ("values", 24, "f32[4]"),
        ("reserved2", 40, "u8[8]"),
    ]
}

fn response_request_fields() -> [Field; 27] {
    [
        (
            "structSize",
            offset_of!(WebResponseRequest, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebResponseRequest, abi_version),
            "u32",
        ),
        ("target", offset_of!(WebResponseRequest, target), "u32"),
        ("grid", offset_of!(WebResponseRequest, grid), "u32"),
        ("channels", offset_of!(WebResponseRequest, channels), "u32"),
        ("fields", offset_of!(WebResponseRequest, fields), "u32"),
        ("quality", offset_of!(WebResponseRequest, quality), "u32"),
        ("linkMode", offset_of!(WebResponseRequest, link_mode), "u32"),
        ("bypass", offset_of!(WebResponseRequest, bypass), "u32"),
        (
            "effectIdBytes",
            offset_of!(WebResponseRequest, effect_id_bytes),
            "u32",
        ),
        (
            "parameterCount",
            offset_of!(WebResponseRequest, parameter_count),
            "u32",
        ),
        ("points", offset_of!(WebResponseRequest, points), "u32"),
        (
            "sampleRateHz",
            offset_of!(WebResponseRequest, sample_rate_hz),
            "u32",
        ),
        (
            "quantumFrames",
            offset_of!(WebResponseRequest, quantum_frames),
            "u32",
        ),
        (
            "minimumHz",
            offset_of!(WebResponseRequest, minimum_hz),
            "f32",
        ),
        (
            "maximumHz",
            offset_of!(WebResponseRequest, maximum_hz),
            "f32",
        ),
        (
            "leftHpfHz",
            offset_of!(WebResponseRequest, left_hpf_hz),
            "f32",
        ),
        (
            "leftLpfHz",
            offset_of!(WebResponseRequest, left_lpf_hz),
            "f32",
        ),
        (
            "rightHpfHz",
            offset_of!(WebResponseRequest, right_hpf_hz),
            "f32",
        ),
        (
            "rightLpfHz",
            offset_of!(WebResponseRequest, right_lpf_hz),
            "f32",
        ),
        (
            "configurationId",
            offset_of!(WebResponseRequest, configuration_id),
            "u64",
        ),
        (
            "maximumPreparedBytes",
            offset_of!(WebResponseRequest, maximum_prepared_bytes),
            "u64",
        ),
        (
            "maximumTotalStateBytes",
            offset_of!(WebResponseRequest, maximum_total_state_bytes),
            "u64",
        ),
        (
            "maximumScratchBytes",
            offset_of!(WebResponseRequest, maximum_scratch_bytes),
            "u64",
        ),
        (
            "maximumAutomationSpansPerBlock",
            offset_of!(WebResponseRequest, maximum_automation_spans_per_block),
            "u32",
        ),
        (
            "maximumResultBytes",
            offset_of!(WebResponseRequest, maximum_result_bytes),
            "u32",
        ),
        (
            "reserved",
            offset_of!(WebResponseRequest, reserved),
            "u32[2]",
        ),
    ]
}

fn response_parameter_fields() -> [Field; 4] {
    [
        (
            "parameterId",
            offset_of!(WebResponseParameter, parameter_id),
            "u32",
        ),
        ("channel", offset_of!(WebResponseParameter, channel), "u32"),
        ("value", offset_of!(WebResponseParameter, value), "f32"),
        (
            "reserved",
            offset_of!(WebResponseParameter, reserved),
            "u32",
        ),
    ]
}

fn response_result_fields() -> [Field; 23] {
    [
        (
            "structSize",
            offset_of!(WebResponseResult, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebResponseResult, abi_version),
            "u32",
        ),
        ("result", offset_of!(WebResponseResult, result), "u32"),
        ("target", offset_of!(WebResponseResult, target), "u32"),
        ("channels", offset_of!(WebResponseResult, channels), "u32"),
        ("fields", offset_of!(WebResponseResult, fields), "u32"),
        ("points", offset_of!(WebResponseResult, points), "u32"),
        (
            "sectionCount",
            offset_of!(WebResponseResult, section_count),
            "u32",
        ),
        (
            "sampleRateHz",
            offset_of!(WebResponseResult, sample_rate_hz),
            "u32",
        ),
        ("reserved0", offset_of!(WebResponseResult, reserved0), "u32"),
        (
            "configurationId",
            offset_of!(WebResponseResult, configuration_id),
            "u64",
        ),
        ("floorDb", offset_of!(WebResponseResult, floor_db), "f32"),
        ("bypass", offset_of!(WebResponseResult, bypass), "u32"),
        (
            "enabledLeft",
            offset_of!(WebResponseResult, enabled_left),
            "u32",
        ),
        (
            "enabledRight",
            offset_of!(WebResponseResult, enabled_right),
            "u32",
        ),
        (
            "retainedBytes",
            offset_of!(WebResponseResult, retained_bytes),
            "u64",
        ),
        (
            "resultBytes",
            offset_of!(WebResponseResult, result_bytes),
            "u64",
        ),
        (
            "frequenciesOffset",
            offset_of!(WebResponseResult, frequencies_offset),
            "u32",
        ),
        (
            "totalLeftOffset",
            offset_of!(WebResponseResult, total_left_offset),
            "u32",
        ),
        (
            "totalRightOffset",
            offset_of!(WebResponseResult, total_right_offset),
            "u32",
        ),
        (
            "sectionsLeftOffset",
            offset_of!(WebResponseResult, sections_left_offset),
            "u32",
        ),
        (
            "sectionsRightOffset",
            offset_of!(WebResponseResult, sections_right_offset),
            "u32",
        ),
        (
            "reserved",
            offset_of!(WebResponseResult, reserved),
            "u32[3]",
        ),
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

fn eq_target_request_fields() -> [Field; 6] {
    [
        (
            "structSize",
            offset_of!(WebEqTargetRequest, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebEqTargetRequest, abi_version),
            "u32",
        ),
        (
            "sampleRateHz",
            offset_of!(WebEqTargetRequest, sample_rate_hz),
            "u32",
        ),
        (
            "seedCount",
            offset_of!(WebEqTargetRequest, seed_count),
            "u32",
        ),
        (
            "editCount",
            offset_of!(WebEqTargetRequest, edit_count),
            "u32",
        ),
        (
            "reserved",
            offset_of!(WebEqTargetRequest, reserved),
            "u32[3]",
        ),
    ]
}
fn eq_target_edit_fields() -> [Field; 3] {
    [
        (
            "parameterId",
            offset_of!(WebEqTargetEdit, parameter_id),
            "u32",
        ),
        ("channel", offset_of!(WebEqTargetEdit, channel), "u32"),
        ("value", offset_of!(WebEqTargetEdit, value), "f32"),
    ]
}
fn input_filter_edit_fields() -> [Field; 4] {
    [
        (
            "parameterId",
            offset_of!(WebInputFilterEdit, parameter_id),
            "u32",
        ),
        ("channel", offset_of!(WebInputFilterEdit, channel), "u32"),
        ("value0", offset_of!(WebInputFilterEdit, value0), "f32"),
        ("value1", offset_of!(WebInputFilterEdit, value1), "f32"),
    ]
}
fn prepared_target_fields() -> [Field; 3] {
    [
        ("slot", offset_of!(WebPreparedEffectTarget, slot), "u32"),
        (
            "channel",
            offset_of!(WebPreparedEffectTarget, channel),
            "u32",
        ),
        (
            "words",
            offset_of!(WebPreparedEffectTarget, words),
            "u32[12]",
        ),
    ]
}
fn eq_target_result_fields() -> [Field; 6] {
    [
        (
            "structSize",
            offset_of!(WebEqTargetResult, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebEqTargetResult, abi_version),
            "u32",
        ),
        (
            "valueCount",
            offset_of!(WebEqTargetResult, value_count),
            "u32",
        ),
        (
            "targetCount",
            offset_of!(WebEqTargetResult, target_count),
            "u32",
        ),
        (
            "workspaceRetainedBytes",
            offset_of!(WebEqTargetResult, workspace_retained_bytes),
            "u64",
        ),
        (
            "workspaceLargestAllocationBytes",
            offset_of!(WebEqTargetResult, workspace_largest_allocation_bytes),
            "u64",
        ),
    ]
}
fn companion_header_fields() -> [Field; 5] {
    [
        (
            "structSize",
            offset_of!(WebPreparedEffectCompanionHeader, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebPreparedEffectCompanionHeader, abi_version),
            "u32",
        ),
        (
            "hostGeneration",
            offset_of!(WebPreparedEffectCompanionHeader, host_generation),
            "u64",
        ),
        (
            "targetCount",
            offset_of!(WebPreparedEffectCompanionHeader, target_count),
            "u32",
        ),
        (
            "reserved",
            offset_of!(WebPreparedEffectCompanionHeader, reserved),
            "u32",
        ),
    ]
}
fn companion_record_fields() -> [Field; 8] {
    [
        (
            "trackIndex",
            offset_of!(WebPreparedEffectCompanionRecord, track_index),
            "u32",
        ),
        (
            "rack",
            offset_of!(WebPreparedEffectCompanionRecord, rack),
            "u32",
        ),
        (
            "effectIndex",
            offset_of!(WebPreparedEffectCompanionRecord, effect_index),
            "u32",
        ),
        (
            "reserved",
            offset_of!(WebPreparedEffectCompanionRecord, reserved),
            "u32",
        ),
        (
            "baseRevision",
            offset_of!(WebPreparedEffectCompanionRecord, base_revision),
            "u64",
        ),
        (
            "slot",
            offset_of!(WebPreparedEffectCompanionRecord, slot),
            "u32",
        ),
        (
            "channel",
            offset_of!(WebPreparedEffectCompanionRecord, channel),
            "u32",
        ),
        (
            "words",
            offset_of!(WebPreparedEffectCompanionRecord, words),
            "u32[12]",
        ),
    ]
}
fn eq_target_config_fields() -> [Field; 7] {
    [
        (
            "structSize",
            offset_of!(WebEqTargetConfig, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebEqTargetConfig, abi_version),
            "u32",
        ),
        (
            "sampleRateHz",
            offset_of!(WebEqTargetConfig, sample_rate_hz),
            "u32",
        ),
        (
            "valueCount",
            offset_of!(WebEqTargetConfig, value_count),
            "u32",
        ),
        (
            "hostGeneration",
            offset_of!(WebEqTargetConfig, host_generation),
            "u64",
        ),
        (
            "ownerRevision",
            offset_of!(WebEqTargetConfig, owner_revision),
            "u64",
        ),
        ("values", offset_of!(WebEqTargetConfig, values), "f32[60]"),
    ]
}
fn builtin_input_config_fields() -> [Field; 7] {
    [
        (
            "structSize",
            offset_of!(WebBuiltinInputConfig, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebBuiltinInputConfig, abi_version),
            "u32",
        ),
        (
            "sampleRateHz",
            offset_of!(WebBuiltinInputConfig, sample_rate_hz),
            "u32",
        ),
        (
            "valueCount",
            offset_of!(WebBuiltinInputConfig, value_count),
            "u32",
        ),
        (
            "hostGeneration",
            offset_of!(WebBuiltinInputConfig, host_generation),
            "u64",
        ),
        (
            "ownerRevision",
            offset_of!(WebBuiltinInputConfig, owner_revision),
            "u64",
        ),
        (
            "values",
            offset_of!(WebBuiltinInputConfig, values),
            "f32[4]",
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
        (COMMAND_PAN, "pan"),
        (COMMAND_MATRIX, "matrix"),
        (COMMAND_FADER_DB, "faderDb"),
        (COMMAND_MUTE, "mute"),
        (COMMAND_EFFECT_PARAM, "effectParam"),
        (COMMAND_EFFECT_BYPASS, "effectBypass"),
        (COMMAND_OBSERVE_SUBSCRIBE, "observeSubscribe"),
        (COMMAND_OBSERVE_UNSUBSCRIBE, "observeUnsubscribe"),
        (COMMAND_SOLO, "solo"),
        (COMMAND_TRIM_DB, "trimDb"),
        (COMMAND_POLARITY_INVERT, "polarityInvert"),
        (COMMAND_INPUT_FILTERS, "inputFilters"),
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
    ];

    let mut out = String::with_capacity(1 << 14);
    out.push_str("{\n");
    out.push_str(&format!("  \"schema\": \"{SCHEMA}\",\n"));
    out.push_str(&format!("  \"abiVersion\": {ABI_VERSION},\n"));
    out.push_str("  \"stagingSequence\": [");
    for (index, export) in STAGING_SEQUENCE.iter().enumerate() {
        out.push_str(&format!(
            "\"{export}\"{}",
            if index + 1 == STAGING_SEQUENCE.len() {
                ""
            } else {
                ", "
            }
        ));
    }
    out.push_str("],\n");
    out.push_str("  \"errorPhases\": [");
    for (index, phase) in ERROR_PHASES.iter().enumerate() {
        out.push_str(&format!(
            "\"{phase}\"{}",
            if index + 1 == ERROR_PHASES.len() {
                ""
            } else {
                ", "
            }
        ));
    }
    out.push_str("],\n");
    out.push_str("  \"exports\": [\n");
    for (index, export) in EXPORTS.iter().enumerate() {
        out.push_str(&format!(
            "    \"{export}\"{}\n",
            comma(index, EXPORTS.len())
        ));
    }
    out.push_str("  ],\n");
    out.push_str("  \"structures\": {\n");
    render_structure(
        &mut out,
        "bootOptions",
        BOOT_OPTIONS_BYTES,
        &boot_options_fields(),
        true,
    );
    render_structure(&mut out, "status", STATUS_BYTES, &status_fields(), true);
    render_structure(
        &mut out,
        "resourceReport",
        RESOURCE_REPORT_BYTES,
        &resource_report_fields(),
        true,
    );
    render_structure(
        &mut out,
        "meterHeader",
        METER_HEADER_BYTES,
        &meter_header_fields(),
        true,
    );
    render_structure(
        &mut out,
        "commandReport",
        COMMAND_REPORT_BYTES,
        &command_report_fields(),
        true,
    );
    render_structure(
        &mut out,
        "observationSelection",
        OBSERVATION_SELECTION_BYTES,
        &observation_selection_fields(),
        true,
    );
    render_structure(
        &mut out,
        "observationStatus",
        size_of::<WebObservationStatus>() as u32,
        &observation_status_fields(),
        true,
    );
    render_structure(
        &mut out,
        "observationWorkLimits",
        size_of::<WebObservationWorkLimits>() as u32,
        &observation_work_limits_fields(),
        true,
    );
    render_structure(
        &mut out,
        "observationIngressLimits",
        size_of::<WebObservationIngressLimits>() as u32,
        &observation_ingress_limits_fields(),
        true,
    );
    render_structure(
        &mut out,
        "observationPreparation",
        size_of::<WebObservationPreparationRecord>() as u32,
        &observation_preparation_fields(),
        true,
    );
    render_structure(
        &mut out,
        "observationDemand",
        size_of::<WebObservationDemand>() as u32,
        &observation_demand_fields(),
        true,
    );
    render_structure(
        &mut out,
        "observationReceipt",
        size_of::<WebObservationReceipt>() as u32,
        &observation_receipt_fields(),
        true,
    );
    render_structure(
        &mut out,
        "observationAdmission",
        size_of::<WebObservationAdmission>() as u32,
        &observation_admission_fields(),
        true,
    );
    render_structure(
        &mut out,
        "observationCaptureIdentity",
        size_of::<WebObservationCaptureIdentity>() as u32,
        &observation_capture_identity_fields(),
        true,
    );
    render_structure(
        &mut out,
        "observationResult",
        OBSERVATION_RESULT_BYTES,
        &observation_result_fields(),
        true,
    );
    render_structure(
        &mut out,
        "responseRequest",
        RESPONSE_REQUEST_BYTES,
        &response_request_fields(),
        true,
    );
    render_structure(
        &mut out,
        "responseParameter",
        RESPONSE_PARAMETER_BYTES,
        &response_parameter_fields(),
        true,
    );
    render_structure(
        &mut out,
        "responseResult",
        RESPONSE_RESULT_BYTES,
        &response_result_fields(),
        true,
    );
    render_structure(
        &mut out,
        "liveResponseRequest",
        LIVE_RESPONSE_REQUEST_BYTES,
        &live_response_request_fields(),
        true,
    );
    render_structure(
        &mut out,
        "spectrumRequest",
        SPECTRUM_REQUEST_BYTES,
        &spectrum_request_fields(),
        true,
    );
    render_structure(
        &mut out,
        "spectrumCollectionRequest",
        SPECTRUM_COLLECTION_REQUEST_BYTES,
        &spectrum_collection_request_fields(),
        true,
    );
    render_structure(
        &mut out,
        "spectrumCollectionEntry",
        SPECTRUM_COLLECTION_ENTRY_BYTES,
        &spectrum_collection_entry_fields(),
        true,
    );
    render_structure(
        &mut out,
        "spectrumWindow",
        SPECTRUM_WINDOW_HEADER_BYTES,
        &spectrum_window_fields(),
        true,
    );
    render_structure(
        &mut out,
        "spectrumResult",
        SPECTRUM_RESULT_HEADER_BYTES,
        &spectrum_result_fields(),
        true,
    );
    render_structure(
        &mut out,
        "spectrumStreamMetadata",
        SPECTRUM_STREAM_METADATA_BYTES,
        &spectrum_stream_metadata_fields(),
        true,
    );
    render_structure(
        &mut out,
        "liveResponseOwner",
        LIVE_RESPONSE_OWNER_BYTES,
        &live_response_owner_fields(),
        true,
    );
    render_structure(
        &mut out,
        "liveResponseSection",
        LIVE_RESPONSE_SECTION_BYTES,
        &live_response_section_fields(),
        true,
    );
    render_structure(
        &mut out,
        "liveResponseResult",
        LIVE_RESPONSE_RESULT_BYTES,
        &live_response_result_fields(),
        true,
    );
    render_structure(
        &mut out,
        "eqTargetRequest",
        32,
        &eq_target_request_fields(),
        true,
    );
    render_structure(&mut out, "eqTargetEdit", 12, &eq_target_edit_fields(), true);
    render_structure(
        &mut out,
        "inputFilterEdit",
        16,
        &input_filter_edit_fields(),
        true,
    );
    render_structure(
        &mut out,
        "preparedEffectTarget",
        56,
        &prepared_target_fields(),
        true,
    );
    render_structure(
        &mut out,
        "eqTargetResult",
        32,
        &eq_target_result_fields(),
        true,
    );
    render_structure(
        &mut out,
        "preparedEffectCompanionHeader",
        24,
        &companion_header_fields(),
        true,
    );
    render_structure(
        &mut out,
        "preparedEffectCompanionRecord",
        80,
        &companion_record_fields(),
        true,
    );
    render_structure(
        &mut out,
        "eqTargetConfig",
        272,
        &eq_target_config_fields(),
        true,
    );
    render_structure(
        &mut out,
        "builtinInputConfig",
        48,
        &builtin_input_config_fields(),
        false,
    );
    out.push_str("  },\n");
    out.push_str("  \"commandRecord\": {\n");
    out.push_str(&format!("    \"bytes\": {COMMAND_RECORD_BYTES},\n"));
    out.push_str("    \"endianness\": \"little\",\n");
    render_fields(&mut out, &command_record_fields(), "    ");
    out.push_str("  },\n");
    out.push_str("  \"constants\": {\n");
    render_named_constants(&mut out, "resultCodes", &results);
    render_named_constants(&mut out, "bootResultAliases", &boot_aliases);
    render_named_constants(&mut out, "states", &states);
    render_named_constants(&mut out, "backends", &backends);
    render_named_constants(&mut out, "bufferKinds", &buffers);
    render_named_constants(&mut out, "wireCommandKinds", &wire_command_kinds);
    render_named_constants(&mut out, "commandReasons", &command_reasons);
    render_named_constants(
        &mut out,
        "observationProfiles",
        &[
            (OBSERVATION_PROFILE_LEGACY_UNPROTECTED, "legacyUnprotected"),
            (OBSERVATION_PROFILE_EQ_SPECTRUM, "eqSpectrum"),
        ],
    );
    render_named_constants(
        &mut out,
        "observationOperations",
        &[
            (OBSERVATION_OPERATION_REPLACE_METERS, "replaceMeters"),
            (OBSERVATION_OPERATION_REMOVE_METERS_TO, "removeMetersTo"),
            (OBSERVATION_OPERATION_STOP_GRAPH, "stopGraph"),
            (OBSERVATION_OPERATION_START_SPECTRUM, "startSpectrum"),
            (OBSERVATION_OPERATION_RESTART_SPECTRUM, "restartSpectrum"),
            (OBSERVATION_OPERATION_READ_SPECTRUM, "readSpectrum"),
            (OBSERVATION_OPERATION_STOP_SPECTRUM, "stopSpectrum"),
            (OBSERVATION_OPERATION_CAPTURE_RESPONSE, "captureResponse"),
            (
                OBSERVATION_OPERATION_RAW_OBSERVATION_BATCH,
                "rawObservationBatch",
            ),
            (OBSERVATION_OPERATION_ONE_SHOT, "oneShot"),
            (
                OBSERVATION_OPERATION_COLLECTION_SELECTION,
                "collectionSelection",
            ),
            (OBSERVATION_OPERATION_METER_LEASE, "meterLease"),
            (OBSERVATION_OPERATION_METER_READ, "meterRead"),
            (OBSERVATION_OPERATION_RESIDENT_READ, "residentRead"),
        ],
    );
    render_named_constants(
        &mut out,
        "observationReceiptDomains",
        &[
            (OBSERVATION_RECEIPT_DOMAIN_GRAPH, "graph"),
            (OBSERVATION_RECEIPT_DOMAIN_RESIDENT, "resident"),
        ],
    );
    render_named_constants(
        &mut out,
        "observationReceiptStates",
        &[
            (OBSERVATION_RECEIPT_STATE_PENDING, "pending"),
            (OBSERVATION_RECEIPT_STATE_APPLIED, "applied"),
            (OBSERVATION_RECEIPT_STATE_CLOSED, "closed"),
            (OBSERVATION_RECEIPT_STATE_FAILED, "failed"),
        ],
    );
    render_named_constants(
        &mut out,
        "observationAdmissionFlags",
        &[
            (OBSERVATION_ADMISSION_RECEIPT, "receiptPresent"),
            (OBSERVATION_ADMISSION_REQUESTED, "requestedPresent"),
            (OBSERVATION_ADMISSION_MAXIMUM, "maximumPresent"),
            (OBSERVATION_ADMISSION_PENDING_BOUNDARY, "pendingBoundary"),
        ],
    );
    render_named_constants(
        &mut out,
        "observationStatusFlags",
        &[
            (
                OBSERVATION_STATUS_FLAG_ORDINARY_AVAILABLE,
                "ordinaryAvailable",
            ),
            (
                OBSERVATION_STATUS_FLAG_REMOVAL_AVAILABLE,
                "removalAvailable",
            ),
            (OBSERVATION_STATUS_FLAG_TERMINAL, "terminal"),
            (OBSERVATION_STATUS_FLAG_RENDER_FAILED, "renderFailed"),
        ],
    );
    render_named_constants(
        &mut out,
        "observationCaptureKinds",
        &[
            (OBSERVATION_CAPTURE_KIND_RESPONSE, "response"),
            (OBSERVATION_CAPTURE_KIND_SPECTRUM, "spectrum"),
        ],
    );
    render_named_constants(
        &mut out,
        "observationCaptureFlags",
        &[(OBSERVATION_CAPTURE_FLAG_GRAPH_GENERATION, "graphGeneration")],
    );
    render_named_constants(
        &mut out,
        "observationRefusalReasons",
        &[
            (OBSERVATION_REFUSAL_REASON_NONE, "none"),
            (OBSERVATION_REFUSAL_REASON_NOT_PREPARED, "notPrepared"),
            (OBSERVATION_REFUSAL_REASON_WRONG_OWNER, "wrongOwner"),
            (OBSERVATION_REFUSAL_REASON_CAPACITY, "capacity"),
            (OBSERVATION_REFUSAL_REASON_WORK_BUDGET, "workBudget"),
            (OBSERVATION_REFUSAL_REASON_BACKPRESSURE, "backpressure"),
            (OBSERVATION_REFUSAL_REASON_CONFLICT, "conflict"),
            (OBSERVATION_REFUSAL_REASON_CLOSED, "closed"),
            (OBSERVATION_REFUSAL_REASON_INVALID_REQUEST, "invalidRequest"),
            (
                OBSERVATION_REFUSAL_REASON_ARITHMETIC_OVERFLOW,
                "arithmeticOverflow",
            ),
            (
                OBSERVATION_REFUSAL_REASON_REVISION_EXHAUSTED,
                "revisionExhausted",
            ),
        ],
    );
    render_named_constants(
        &mut out,
        "observationChannels",
        &[
            (OBSERVATION_CHANNEL_LEFT, "left"),
            (OBSERVATION_CHANNEL_RIGHT, "right"),
            (OBSERVATION_CHANNEL_BOTH, "both"),
        ],
    );
    render_named_constants(
        &mut out,
        "observationStatuses",
        &[
            (OBSERVATION_STATUS_PENDING, "pending"),
            (OBSERVATION_STATUS_UNARMED, "unarmed"),
            (OBSERVATION_STATUS_READY, "ready"),
        ],
    );
    render_named_constants(
        &mut out,
        "responseTargets",
        &[
            (RESPONSE_TARGET_EFFECT, "effect"),
            (RESPONSE_TARGET_INPUT_FILTERS, "inputFilters"),
        ],
    );
    render_named_constants(
        &mut out,
        "responseGrids",
        &[
            (RESPONSE_GRID_LINEAR, "linear"),
            (RESPONSE_GRID_LOGARITHMIC, "logarithmic"),
        ],
    );
    render_named_constants(
        &mut out,
        "responseChannels",
        &[
            (RESPONSE_CHANNEL_LEFT, "left"),
            (RESPONSE_CHANNEL_RIGHT, "right"),
            (RESPONSE_CHANNEL_BOTH, "both"),
        ],
    );
    render_named_constants(
        &mut out,
        "responseFields",
        &[
            (RESPONSE_FIELD_TOTAL, "total"),
            (RESPONSE_FIELD_SECTIONS, "sections"),
        ],
    );
    render_named_constants(
        &mut out,
        "liveResponseModes",
        &[(LIVE_RESPONSE_MODE_TARGET, "target")],
    );
    render_named_constants(
        &mut out,
        "liveResponseMeanings",
        &[(LIVE_RESPONSE_MEANING_EQ_FILTER_SUBTOTAL, "eqFilterSubtotal")],
    );
    render_named_constants(
        &mut out,
        "spectrumTargets",
        &[
            (
                SPECTRUM_TARGET_TRACK_POST_INPUT_BUILTINS,
                "trackPostInputBuiltins",
            ),
            (SPECTRUM_TARGET_TRACK_POST_MATRIX, "trackPostMatrix"),
            (SPECTRUM_TARGET_OUTPUT, "output"),
        ],
    );
    render_named_constants(
        &mut out,
        "spectrumChannels",
        &[
            (SPECTRUM_CHANNEL_LEFT, "left"),
            (SPECTRUM_CHANNEL_RIGHT, "right"),
            (SPECTRUM_CHANNEL_BOTH, "both"),
        ],
    );
    render_named_constants(
        &mut out,
        "spectrumStreamStatuses",
        &[
            (SPECTRUM_STREAM_STATUS_INACTIVE, "inactive"),
            (SPECTRUM_STREAM_STATUS_WARMING, "warming"),
            (SPECTRUM_STREAM_STATUS_PENDING, "pending"),
            (SPECTRUM_STREAM_STATUS_GAP, "gap"),
            (SPECTRUM_STREAM_STATUS_FAILED, "failed"),
            (SPECTRUM_STREAM_STATUS_STOPPED, "stopped"),
            (SPECTRUM_STREAM_STATUS_READY, "ready"),
        ],
    );
    out.push_str(&format!(
        "    \"maximumCommandRecords\": {MAXIMUM_COMMAND_RECORDS},\n"
    ));
    out.push_str(&format!(
        "    \"maximumDocumentBytes\": {MAXIMUM_DOCUMENT_BYTES},\n"
    ));
    out.push_str(&format!("    \"diagnosticBytes\": {DIAGNOSTIC_BYTES},\n"));
    out.push_str(&format!(
        "    \"defaultCommandQueueRecords\": {DEFAULT_COMMAND_QUEUE_RECORDS},\n"
    ));
    out.push_str(&format!(
        "    \"defaultMeterBlocks\": {DEFAULT_METER_BLOCKS},\n"
    ));
    out.push_str(&format!(
        "    \"maximumObservationTaps\": {MAXIMUM_OBSERVATION_TAPS},\n"
    ));
    out.push_str(&format!(
        "    \"maximumResponseEffectIdBytes\": {RESPONSE_MAXIMUM_EFFECT_ID_BYTES},\n"
    ));
    out.push_str(&format!(
        "    \"maximumResponseParameterOverrides\": {RESPONSE_MAXIMUM_PARAMETER_OVERRIDES},\n"
    ));
    out.push_str(&format!(
        "    \"maximumResponseResultBytes\": {RESPONSE_MAXIMUM_RESULT_BYTES},\n"
    ));
    out.push_str(&format!(
        "    \"maximumLiveResponseOwners\": {LIVE_RESPONSE_MAXIMUM_OWNERS},\n"
    ));
    out.push_str(&format!(
        "    \"maximumLiveResponseSections\": {LIVE_RESPONSE_MAXIMUM_SECTIONS},\n"
    ));
    out.push_str(&format!(
        "    \"maximumLiveResponseIdBytes\": {LIVE_RESPONSE_MAXIMUM_ID_BYTES},\n"
    ));
    out.push_str(&format!(
        "    \"maximumLiveResponsePoints\": {LIVE_RESPONSE_MAXIMUM_POINTS},\n"
    ));
    out.push_str(&format!(
        "    \"liveResponseCaptureBytes\": {LIVE_RESPONSE_CAPTURE_BYTES},\n"
    ));
    out.push_str(&format!(
        "    \"spectrumCaptureBytes\": {SPECTRUM_CAPTURE_BYTES},\n"
    ));
    out.push_str(&format!(
        "    \"maximumPreparedSpectrumTargets\": {SPECTRUM_MAXIMUM_PREPARED_TARGETS},\n"
    ));
    out.push_str(&format!(
        "    \"spectrumCollectionRequestBytes\": {SPECTRUM_COLLECTION_REQUEST_BYTES},\n"
    ));
    out.push_str(&format!(
        "    \"spectrumCollectionEntryBytes\": {SPECTRUM_COLLECTION_ENTRY_BYTES},\n"
    ));
    out.push_str(&format!(
        "    \"spectrumCollectionEntryCapacity\": {SPECTRUM_COLLECTION_ENTRY_CAPACITY},\n"
    ));
    out.push_str(&format!(
        "    \"spectrumCollectionTargetIdsBytes\": {SPECTRUM_COLLECTION_TARGET_IDS_BYTES},\n"
    ));
    out.push_str(&format!(
        "    \"spectrumRequestBytes\": {SPECTRUM_REQUEST_BYTES},\n"
    ));
    out.push_str(&format!(
        "    \"spectrumWindowHeaderBytes\": {SPECTRUM_WINDOW_HEADER_BYTES},\n"
    ));
    out.push_str(&format!(
        "    \"spectrumResultHeaderBytes\": {SPECTRUM_RESULT_HEADER_BYTES},\n"
    ));
    out.push_str(&format!(
        "    \"spectrumStreamMetadataBytes\": {SPECTRUM_STREAM_METADATA_BYTES},\n"
    ));
    out.push_str(&format!(
        "    \"spectrumWindowFrames\": {SPECTRUM_WINDOW_FRAMES},\n"
    ));
    out.push_str(&format!(
        "    \"spectrumBinCount\": {SPECTRUM_BIN_COUNT},\n"
    ));
    out.push_str(&format!(
        "    \"maximumSpectrumIdBytes\": {SPECTRUM_MAXIMUM_ID_BYTES},\n"
    ));
    out.push_str(&format!(
        "    \"defaultMaximumMemoryBytes\": {DEFAULT_MAXIMUM_MEMORY_BYTES},\n"
    ));
    out.push_str("    \"sourceRing\": { ");
    out.push_str(&format!(
        "\"stallToleranceMs\": {SOURCE_STALL_TOLERANCE_MS}, \
         \"reserveQuanta\": {SOURCE_RING_RESERVE_QUANTA} }}\n"
    ));
    out.push_str("  }\n}\n");
    out
}

fn render_structure(
    out: &mut String,
    name: &str,
    bytes: u32,
    fields: &[Field],
    trailing_comma: bool,
) {
    out.push_str(&format!("    \"{name}\": {{\n      \"bytes\": {bytes},\n"));
    render_fields(out, fields, "      ");
    out.push_str(if trailing_comma {
        "    },\n"
    } else {
        "    }\n"
    });
}

fn render_fields(out: &mut String, fields: &[Field], indent: &str) {
    out.push_str(&format!("{indent}\"fields\": [\n"));
    for (index, (name, offset, kind)) in fields.iter().enumerate() {
        out.push_str(&format!(
            "{indent}  {{ \"name\": \"{name}\", \"offset\": {offset}, \"type\": \"{kind}\" }}{}\n",
            comma(index, fields.len())
        ));
    }
    out.push_str(&format!("{indent}]\n"));
}

fn render_named_constants(out: &mut String, name: &str, values: &[(u32, &str)]) {
    out.push_str(&format!("    \"{name}\": ["));
    for (index, (value, label)) in values.iter().enumerate() {
        out.push_str(&format!(
            "{{ \"value\": {value}, \"name\": \"{label}\" }}{}",
            if index + 1 == values.len() { "" } else { ", " }
        ));
    }
    out.push_str("],\n");
}

fn comma(index: usize, total: usize) -> &'static str {
    if index + 1 == total { "" } else { "," }
}
