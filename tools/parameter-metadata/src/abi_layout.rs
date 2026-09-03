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

use host_web::{
    ABI_VERSION, BACKEND_SCALAR, BACKEND_SIMD128, BOOT_OPTIONS_BYTES, BUFFER_COMMAND,
    BUFFER_DIAGNOSTIC, BUFFER_METER_FRAME, BUFFER_OUTPUT_PCM, BUFFER_SOURCE_ID, BUFFER_SOURCE_PCM,
    COMMAND_EFFECT_BYPASS, COMMAND_EFFECT_PARAM, COMMAND_FADER_DB, COMMAND_MATRIX, COMMAND_MUTE,
    COMMAND_OBSERVE_SUBSCRIBE, COMMAND_OBSERVE_UNSUBSCRIBE, COMMAND_PAN, COMMAND_POLARITY_INVERT,
    COMMAND_REASON_BACKPRESSURE, COMMAND_REASON_DOMAIN, COMMAND_REASON_MALFORMED,
    COMMAND_REASON_NONE, COMMAND_REASON_OBSERVATION_UNBOUND, COMMAND_REASON_UNKNOWN_EFFECT,
    COMMAND_REASON_UNKNOWN_PARAMETER, COMMAND_REASON_UNKNOWN_RACK, COMMAND_REASON_UNKNOWN_TAP,
    COMMAND_REASON_UNKNOWN_TRACK, COMMAND_REASON_UNSUPPORTED_KIND, COMMAND_REASON_WRONG_STATE,
    COMMAND_RECORD_BYTES, COMMAND_REPORT_BYTES, COMMAND_SOLO, COMMAND_TRIM_DB,
    DEFAULT_COMMAND_QUEUE_RECORDS, DEFAULT_MAXIMUM_MEMORY_BYTES, DEFAULT_METER_BLOCKS,
    DIAGNOSTIC_BYTES, MAXIMUM_COMMAND_RECORDS, MAXIMUM_DOCUMENT_BYTES, MAXIMUM_OBSERVATION_TAPS,
    METER_HEADER_BYTES, RESOURCE_REPORT_BYTES, RESULT_ABI_MISMATCH, RESULT_BACKPRESSURE,
    RESULT_BUFFER_TOO_SMALL, RESULT_INTERNAL, RESULT_INVALID_ARGUMENT, RESULT_OK,
    RESULT_REFUSED_BUDGET, RESULT_REFUSED_DOCUMENT, RESULT_REFUSED_LIFECYCLE,
    RESULT_REFUSED_OPTIONS, RESULT_RENDER_REJECTED, RESULT_REPREPARE_REQUIRED, RESULT_UNSUPPORTED,
    RESULT_WRONG_STATE, SOURCE_STALL_TOLERANCE_MS, STATE_DISPOSED, STATE_FAILED, STATE_READY,
    STATUS_BYTES, WebBootOptions, WebCommandReport, WebMeterHeader, WebResourceReport, WebStatus,
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
pub const EXPORTS: [&str; 25] = [
    "miso_engine_web_v1_abi_version",
    "miso_engine_web_v1_boot",
    "miso_engine_web_v1_boot_diagnostic_bytes",
    "miso_engine_web_v1_boot_options_ptr",
    "miso_engine_web_v1_boot_result",
    "miso_engine_web_v1_buffer_capacity",
    "miso_engine_web_v1_buffer_ptr",
    "miso_engine_web_v1_command_report_ptr",
    "miso_engine_web_v1_command_submit",
    "miso_engine_web_v1_console_track_count",
    "miso_engine_web_v1_console_track_id",
    "miso_engine_web_v1_dispose",
    "miso_engine_web_v1_document_ptr",
    "miso_engine_web_v1_meter_header_ptr",
    "miso_engine_web_v1_meter_lease",
    "miso_engine_web_v1_meter_poll",
    "miso_engine_web_v1_render",
    "miso_engine_web_v1_resource_ptr",
    "miso_engine_web_v1_source_channels",
    "miso_engine_web_v1_source_count",
    "miso_engine_web_v1_source_frames",
    "miso_engine_web_v1_source_id",
    "miso_engine_web_v1_source_seek",
    "miso_engine_web_v1_source_submit",
    "miso_engine_web_v1_status_ptr",
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
            "sourceIdBytes",
            offset_of!(WebResourceReport, source_id_bytes),
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
