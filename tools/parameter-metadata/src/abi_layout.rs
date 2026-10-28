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
//! `default_source_ring_frames` (#1406 D2) is the stall ring -- `SOURCE_STALL_TOLERANCE_MS` of
//! audio rounded up to a whole number of quanta, plus one quantum held by the consumer and one in
//! the recycle path -- plus the largest warm-successor prime, `primeGrowths` times the largest
//! launch latency at the rate rounded up to whole quanta, plus one more quantum. A consumer that
//! wants the ring the engine will actually pick -- for sizing its own producer -- needs the
//! inputs to that rule, not a rate-specific answer. All of them are transcribed, so the SDK
//! derives the ring from the shape the boot itself reported instead of holding a private copy of
//! `100`, of `+ 2` or of `+ 1`.

use core::mem::offset_of;

use serde::Serialize;

use crate::json::{self, Entries};
use host_core::{LAUNCH_SAMPLE_RATES, RESPONSE_MAXIMUM_POINTS};
use host_web::{
    ABI_VERSION, BACKEND_SCALAR, BACKEND_SIMD128, BOOT_OPTIONS_BYTES, BUFFER_DIAGNOSTIC,
    BUFFER_METER_FRAME, BUFFER_OUTPUT_PCM, BUFFER_SOURCE_ID, BUFFER_SOURCE_PCM,
    DEFAULT_MAXIMUM_MEMORY_BYTES, DEFAULT_METER_BLOCKS, DIAGNOSTIC_BYTES,
    INSTANCE_RESERVATION_BYTES, METER_HEADER_BYTES, PARSE_TRANSIENT_MULTIPLIER, PLAN_LATENCY_BYTES,
    RACK_CONSOLE, RACK_INSERTS, RACK_NOT_APPLICABLE, RESOURCE_REPORT_BYTES, RESULT_ABI_MISMATCH,
    RESULT_BACKPRESSURE, RESULT_BUFFER_TOO_SMALL, RESULT_COMPILE_REJECTED, RESULT_INTERNAL,
    RESULT_INVALID_ARGUMENT, RESULT_OK, RESULT_REFUSED_BUDGET, RESULT_REFUSED_DOCUMENT,
    RESULT_REFUSED_LIFECYCLE, RESULT_REFUSED_OPTIONS, RESULT_RENDER_REJECTED,
    RESULT_REPREPARE_REQUIRED, RESULT_UNSUPPORTED, RESULT_WRONG_STATE, SEEK_REPORT_BYTES,
    SOURCE_STALL_TOLERANCE_MS, STATE_DISPOSED, STATE_FAILED, STATE_READY, STATUS_BYTES,
    WASM_INITIAL_MEMORY_PAGES, WASM_MAXIMUM_MEMORY_PAGES, WATERMARK_BYTES, WebBootOptions,
    WebInstanceReservation, WebMeterHeader, WebPlanLatency, WebPlanWatermark, WebResourceReport,
    WebSeekReport, WebStatus,
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

/// The growths the default source ring holds a prime for (#1406 D3): `p_max_samples` is this many
/// times the largest launch latency, rounded up to whole quanta (#1286 D3 item 1).
pub const SOURCE_RING_PRIME_GROWTHS: u32 = 4;

/// The quantum the default source ring holds beyond the largest prime (#1406 D2): readiness asks
/// for `k + 1` blocks queued past the consumer for a lead of `k` quanta. Published, so no consumer
/// holds a private `+ 1`.
pub const SOURCE_RING_PRIME_RESERVE_QUANTA: u32 = 1;

/// The largest latency a launch effect declares at `sample_rate_hz`, in samples: the true-peak
/// limiter's `N + 6` with `N = fs / 100` (`crates/true-peak-limiter`), the `L_max(fs)` of
/// `p_max_samples` (#1286 D3 item 1).
#[must_use]
pub const fn source_ring_prime_latency_samples(sample_rate_hz: u32) -> u32 {
    sample_rate_hz / 100 + 6
}

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
pub const EXPORTS: [&str; 58] = [
    "miso_engine_web_v1_abi_version",
    "miso_engine_web_v1_boot",
    "miso_engine_web_v1_boot_command_ptr",
    "miso_engine_web_v1_boot_diagnostic_bytes",
    "miso_engine_web_v1_boot_diagnostic_ptr",
    "miso_engine_web_v1_boot_options_ptr",
    "miso_engine_web_v1_boot_result",
    "miso_engine_web_v1_buffer_capacity",
    "miso_engine_web_v1_buffer_ptr",
    "miso_engine_web_v1_control_allocation_count",
    "miso_engine_web_v1_control_command",
    "miso_engine_web_v1_control_event",
    "miso_engine_web_v1_control_frame_bytes",
    "miso_engine_web_v1_control_request_ptr",
    "miso_engine_web_v1_control_response_bytes",
    "miso_engine_web_v1_control_response_ptr",
    "miso_engine_web_v1_control_revision",
    "miso_engine_web_v1_declare_discontinuity",
    "miso_engine_web_v1_dispose",
    "miso_engine_web_v1_document_ptr",
    "miso_engine_web_v1_host_adopt",
    "miso_engine_web_v1_host_release",
    "miso_engine_web_v1_instance_bind",
    "miso_engine_web_v1_instance_reserve",
    "miso_engine_web_v1_meter_header_ptr",
    "miso_engine_web_v1_meter_lease",
    "miso_engine_web_v1_meter_poll",
    "miso_engine_web_v1_plan_latency_ptr",
    "miso_engine_web_v1_plan_latency_read",
    "miso_engine_web_v1_render",
    "miso_engine_web_v1_render_allocation_count",
    "miso_engine_web_v1_render_diagnostic_bytes",
    "miso_engine_web_v1_render_diagnostic_ptr",
    "miso_engine_web_v1_replace_document_ptr",
    "miso_engine_web_v1_resolved_source_ring_frames",
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
    "miso_engine_web_v1_source_fill",
    "miso_engine_web_v1_source_frames",
    "miso_engine_web_v1_source_id",
    "miso_engine_web_v1_source_seek",
    "miso_engine_web_v1_source_seek_at",
    "miso_engine_web_v1_source_seek_report",
    "miso_engine_web_v1_source_seek_report_ptr",
    "miso_engine_web_v1_source_submit",
    "miso_engine_web_v1_status_ptr",
    "miso_engine_web_v1_successor_header_ptr",
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
        (
            "maximumControlFrameBytes",
            offset_of!(WebBootOptions, maximum_control_frame_bytes),
            "u64",
        ),
        (
            "liveControlMeterBlocks",
            offset_of!(WebBootOptions, live_control_meter_blocks),
            "u64",
        ),
        (
            "maximumDocumentBytes",
            offset_of!(WebBootOptions, maximum_document_bytes),
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

/// The instance reservation record (issue #1332 S6): the stack and TLS block
/// `miso_engine_web_v1_instance_reserve` made for a second instance on the shared memory.
fn instance_reservation_fields() -> [Field; 4] {
    [
        (
            "structSize",
            offset_of!(WebInstanceReservation, struct_size),
            "u32",
        ),
        (
            "tlsBase",
            offset_of!(WebInstanceReservation, tls_base),
            "u32",
        ),
        (
            "stackTop",
            offset_of!(WebInstanceReservation, stack_top),
            "u32",
        ),
        (
            "stackBottom",
            offset_of!(WebInstanceReservation, stack_bottom),
            "u32",
        ),
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
        (BUFFER_METER_FRAME, "meterFrame"),
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
                    "instanceReservation",
                    structure(INSTANCE_RESERVATION_BYTES, &instance_reservation_fields()),
                ),
            ]
            .into_iter()
            .map(|(name, structure)| (name.to_owned(), structure))
            .collect(),
        ),
        constants: Constants {
            result_codes: named(&results),
            boot_result_aliases: named(&boot_aliases),
            states: named(&states),
            backends: named(&backends),
            buffer_kinds: named(&buffers),
            // Decision 12 (#1096): the `rack` of the companion and configuration records. The retired `simd1` (0) and `simd2` (2) codes are absent,
            // never reallocated.
            racks: named(&[
                (u32::from(RACK_INSERTS), "inserts"),
                (u32::from(RACK_CONSOLE), "console"),
                (u32::from(RACK_NOT_APPLICABLE), "notApplicable"),
            ]),
            // Issue #2191: the document's default limit is `budget / parseTransientMultiplier`.
            parse_transient_multiplier: PARSE_TRANSIENT_MULTIPLIER,
            maximum_source_channels: session::SOURCE_CHANNELS_LAUNCH_MAXIMUM,
            diagnostic_bytes: DIAGNOSTIC_BYTES,
            default_meter_blocks: DEFAULT_METER_BLOCKS,
            maximum_response_points: RESPONSE_MAXIMUM_POINTS,
            default_maximum_memory_bytes: DEFAULT_MAXIMUM_MEMORY_BYTES,
            source_ring: SourceRing {
                stall_tolerance_ms: SOURCE_STALL_TOLERANCE_MS,
                reserve_quanta: SOURCE_RING_RESERVE_QUANTA,
                prime_growths: SOURCE_RING_PRIME_GROWTHS,
                prime_reserve_quanta: SOURCE_RING_PRIME_RESERVE_QUANTA,
                prime_latency_samples: LAUNCH_SAMPLE_RATES
                    .iter()
                    .map(|rate| PrimeLatency {
                        sample_rate_hz: rate.0,
                        samples: source_ring_prime_latency_samples(rate.0),
                    })
                    .collect(),
            },
        },
        memory: Memory {
            initial_pages: WASM_INITIAL_MEMORY_PAGES,
            maximum_pages: WASM_MAXIMUM_MEMORY_PAGES,
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
    constants: Constants<'a>,
    memory: Memory,
}

/// The `memory` object (issue #1380 D1): the page counts of the shared `WebAssembly.Memory` every
/// instantiation site creates and passes as `env.memory`. The module's own import declares the
/// same limits, which `check-web-audioworklet.sh` holds equal to these.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Memory {
    initial_pages: u32,
    maximum_pages: u32,
}

#[derive(Serialize)]
struct Structure<'a> {
    bytes: u32,
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
    prime_growths: u32,
    prime_reserve_quanta: u32,
    /// One row per launch rate, in [`LAUNCH_SAMPLE_RATES`] order.
    prime_latency_samples: Vec<PrimeLatency>,
}

/// One `sourceRing.primeLatencySamples` row: `L_max(fs)` at one launch rate.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PrimeLatency {
    sample_rate_hz: u32,
    samples: u32,
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
    racks: Vec<NamedValue<'a>>,
    parse_transient_multiplier: u64,
    maximum_source_channels: u32,
    diagnostic_bytes: u32,
    default_meter_blocks: u32,
    maximum_response_points: usize,
    default_maximum_memory_bytes: u64,
    source_ring: SourceRing,
}

fn field_rows(fields: &[Field]) -> Vec<FieldRow<'static>> {
    fields
        .iter()
        .map(|&(name, offset, kind)| FieldRow { name, offset, kind })
        .collect()
}
