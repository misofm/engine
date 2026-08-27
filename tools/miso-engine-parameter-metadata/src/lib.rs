//! Build-time parameter-metadata codegen for the browser console (issue #137 D4).
//!
//! # Why this exists
//!
//! The app drives the engine through numeric IDs: a track index, a rack, an effect index and a
//! parameter ID. Numbers are what the command path takes, and deliberately so -- there is no
//! string on the hot path. But numbers are useless to a user interface without names, units,
//! ranges, defaults and enumerations, and the one place that knowledge lives is the effect
//! contract's descriptors. This tool emits it, once, at build time, next to the Wasm artifact, so
//! the app never introspects the module for metadata and never has to keep a hand-written copy in
//! step with the engine.
//!
//! # Completeness is structural
//!
//! The effect list comes from `launch_native_effect_registry_v1()` through
//! `NativeEffectRegistry::descriptors`, so "an effect in the registry is missing from the output"
//! is not a rule anybody has to remember: there is no second list to fall out of step with. The
//! `--check` mode regenerates and compares byte for byte, which is what makes the emitted file a
//! build artifact rather than a document.
//!
//! # `liveUpdatable`
//!
//! Every parameter carries whether the live-console command path can actually move it. Issue #140
//! made that true of every builtin block target and every automatable effect parameter: a builtin
//! row is live exactly when its declared update rate is `blockTarget`, and an effect row is live
//! exactly when its descriptor declares it automatable. A caller that reads this file never has to
//! discover either at runtime. See the browser ABI's `.d.ts` header for the full statement.
//!
//! # Issue #127 (named nudge sizes)
//!
//! Each parameter carries `"nudge": null`. When #127 lands its ladder on
//! `ParameterDescriptorV1`, that slot becomes an object and nothing else in this schema moves --
//! which is the whole reason it is a declared null rather than an absent key.

use core::mem::offset_of;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use miso_engine_bench_support::json::escape;
use miso_engine_builtins::{
    BUILTIN_PARAMETER_DESCRIPTORS_V1, BuiltinParameterDescriptorV1, BuiltinParameterDomain,
    BuiltinParameterMapping, BuiltinParameterReset, BuiltinParameterScope,
    BuiltinParameterUpdateRate, BuiltinSmoothingPolicy, builtin_filter_cutoff_maximum_hz_v1,
};
use miso_engine_effect_compiler::launch_native_effect_registry_v1;
use miso_engine_effect_contract::{
    AutomationRate, EffectDescriptorV1, ObservationCadenceV1, ObservationChannelsV1,
    ObservationCostV1, ObservationDescriptorV1, ObservationFoldV1, ObservationKindV1,
    ParameterChannelPolicy, ParameterDescriptorV1, ParameterDomain, ParameterMapping,
    ParameterUnit, SmoothingRule,
};
use miso_engine_host_web::{
    ABI_VERSION, BACKEND_SCALAR, BACKEND_SIMD128, BUFFER_COMMAND, BUFFER_DIAGNOSTIC,
    BUFFER_METER_FRAME, BUFFER_OUTPUT_PCM, BUFFER_SESSION_TOML, BUFFER_SOURCE_ID,
    BUFFER_SOURCE_PCM, COMMAND_EFFECT_BYPASS, COMMAND_EFFECT_PARAM, COMMAND_FADER_DB,
    COMMAND_MATRIX, COMMAND_MUTE, COMMAND_OBSERVE_SUBSCRIBE, COMMAND_OBSERVE_UNSUBSCRIBE,
    COMMAND_PAN, COMMAND_REASON_BACKPRESSURE, COMMAND_REASON_DOMAIN, COMMAND_REASON_MALFORMED,
    COMMAND_REASON_NONE, COMMAND_REASON_OBSERVATION_UNBOUND, COMMAND_REASON_UNKNOWN_EFFECT,
    COMMAND_REASON_UNKNOWN_PARAMETER, COMMAND_REASON_UNKNOWN_RACK, COMMAND_REASON_UNKNOWN_TAP,
    COMMAND_REASON_UNKNOWN_TRACK, COMMAND_REASON_UNSUPPORTED_KIND, COMMAND_REASON_WRONG_STATE,
    COMMAND_RECORD_BYTES, COMMAND_REPORT_BYTES, MAXIMUM_COMMAND_RECORDS, METER_HEADER_BYTES,
    PREPARE_CONFIG_BYTES, RESOURCE_REPORT_BYTES, RESULT_ABI_MISMATCH, RESULT_BACKPRESSURE,
    RESULT_BUFFER_TOO_SMALL, RESULT_INTERNAL, RESULT_INVALID_ARGUMENT, RESULT_OK,
    RESULT_PREPARE_REJECTED, RESULT_RENDER_REJECTED, RESULT_REPREPARE_REQUIRED, RESULT_UNSUPPORTED,
    RESULT_WRONG_STATE, STATE_CONFIG, STATE_DISPOSED, STATE_FAILED, STATE_PREPARED, STATE_READY,
    STATUS_BYTES, WebCommandReportV1, WebMeterHeaderV1, WebPrepareConfigV1, WebResourceReportV1,
    WebStatusV1,
};

/// The emitted file name, shipped beside the Wasm artifact.
pub const OUTPUT_NAME: &str = "miso-engine-v2-parameter-metadata.json";
/// The schema tag every consumer must check before reading a field.
pub const SCHEMA: &str = "miso.web.parameter-metadata.v1";
/// The emitted ABI-layout file name, shipped beside the Wasm artifact.
pub const ABI_LAYOUT_OUTPUT_NAME: &str = "miso-engine-v2-abi-layout.json";
/// The schema tag for the browser ABI layout consumed by SDK code generation.
pub const ABI_LAYOUT_SCHEMA: &str = "miso.web.abi-layout.v1";
/// The launch sample rates a rate-keyed builtin domain is reported for.
pub const LAUNCH_RATES_HZ: [u32; 4] = [44_100, 48_000, 88_200, 96_000];

fn usage() -> ! {
    eprintln!(
        "usage: miso_engine_parameter_metadata --write DIRECTORY | --check DIRECTORY | \\
         --print | --print-abi-layout"
    );
    std::process::exit(2)
}

fn output_path(directory: &Path, name: &str) -> PathBuf {
    if !directory.is_dir() {
        eprintln!("{} is not a directory", directory.display());
        std::process::exit(2);
    }
    directory.join(name)
}

/// Render the whole document. Deterministic: registry order is `EffectId` order.
#[must_use]
pub fn render() -> String {
    let registry = launch_native_effect_registry_v1().expect("launch effect registry");
    let mut out = String::with_capacity(1 << 16);
    out.push_str("{\n");
    out.push_str(&format!("  \"schema\": \"{SCHEMA}\",\n"));
    out.push_str(&format!("  \"abiVersion\": {ABI_VERSION},\n"));
    out.push_str(&format!(
        "  \"commandRecordBytes\": {COMMAND_RECORD_BYTES},\n"
    ));
    out.push_str(&format!(
        "  \"maximumCommandRecords\": {MAXIMUM_COMMAND_RECORDS},\n"
    ));
    out.push_str("  \"commandKinds\": [\n");
    let kinds = [
        (COMMAND_PAN, "pan", true),
        (COMMAND_MATRIX, "matrix", true),
        (COMMAND_FADER_DB, "faderDb", true),
        (COMMAND_MUTE, "mute", true),
        (COMMAND_EFFECT_PARAM, "effectParam", true),
        (COMMAND_EFFECT_BYPASS, "effectBypass", true),
    ];
    for (index, (value, name, applied)) in kinds.iter().enumerate() {
        out.push_str(&format!(
            "    {{ \"value\": {value}, \"name\": \"{name}\", \"applied\": {applied} }}{}\n",
            comma(index, kinds.len())
        ));
    }
    out.push_str("  ],\n");
    // Issue #207 P2: observation subscription changes a host-side `miso.observe.v1` binding; it
    // is not a DSP `miso.command.v1` write. Keeping this separate preserves #140's invariant that
    // every declared `commandKinds` row is applied, without hiding wire kinds 7 and 8 from SDKs.
    out.push_str("  \"observationTransactionKinds\": [\n");
    let observation_transactions = [
        (COMMAND_OBSERVE_SUBSCRIBE, "observeSubscribe"),
        (COMMAND_OBSERVE_UNSUBSCRIBE, "observeUnsubscribe"),
    ];
    for (index, (value, name)) in observation_transactions.iter().enumerate() {
        out.push_str(&format!(
            "    {{ \"value\": {value}, \"name\": \"{name}\", \"protocol\": \"miso.observe.v1\" }}{}\n",
            comma(index, observation_transactions.len())
        ));
    }
    out.push_str("  ],\n");
    out.push_str("  \"commandReasons\": [\n");
    let reasons = [
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
        // Issue #143 added these two and #151 found the drift they caused: a vocabulary that stops
        // at `wrongState` tells every consumer that reasons 10 and 11 do not exist, and the only
        // reasons the observation path ever returns are exactly those two.
        // `scripts/check-command-reason-vocabulary.py` now holds this table, the Rust constants,
        // the host JS bound, the `.d.ts` enum and the schema gate's list to one another.
        (COMMAND_REASON_UNKNOWN_TAP, "unknownTap"),
        (COMMAND_REASON_OBSERVATION_UNBOUND, "observationUnbound"),
    ];
    for (index, (value, name)) in reasons.iter().enumerate() {
        out.push_str(&format!(
            "    {{ \"value\": {value}, \"name\": \"{name}\" }}{}\n",
            comma(index, reasons.len())
        ));
    }
    out.push_str("  ],\n");
    // Issue #143 D1: the observation vocabularies, so a consumer resolves a tap's raw `u32`s the
    // same way it resolves a parameter's -- from this document, never from a hand-written table.
    out.push_str("  \"observationVocabularies\": {\n");
    let vocabularies: [(&str, &[(u32, &str)]); 5] = [
        (
            "kinds",
            &[(ObservationKindV1::GainReductionDb as u32, "gainReductionDb")],
        ),
        (
            "costs",
            &[
                (ObservationCostV1::Resident as u32, "resident"),
                (ObservationCostV1::Computed as u32, "computed"),
            ],
        ),
        (
            "cadences",
            &[
                (ObservationCadenceV1::PerBlock as u32, "perBlock"),
                (ObservationCadenceV1::PerWindow as u32, "perWindow"),
            ],
        ),
        (
            "folds",
            &[
                (ObservationFoldV1::Latest as u32, "latest"),
                (ObservationFoldV1::PeakMagnitude as u32, "peakMagnitude"),
            ],
        ),
        (
            "channels",
            &[
                (ObservationChannelsV1::Shared as u32, "shared"),
                (ObservationChannelsV1::PerLane as u32, "perLane"),
            ],
        ),
    ];
    for (index, (name, rows)) in vocabularies.iter().enumerate() {
        out.push_str(&format!("    \"{name}\": ["));
        for (row, (value, label)) in rows.iter().enumerate() {
            out.push_str(&format!(
                "{{ \"value\": {value}, \"name\": \"{label}\" }}{}",
                if row + 1 == rows.len() { "" } else { ", " }
            ));
        }
        out.push_str(&format!("]{}\n", comma(index, vocabularies.len())));
    }
    out.push_str("  },\n");
    out.push_str("  \"builtins\": {\n    \"parameters\": [\n");
    let builtins = BUILTIN_PARAMETER_DESCRIPTORS_V1;
    for (index, parameter) in builtins.iter().enumerate() {
        out.push_str(&builtin_parameter(parameter));
        out.push_str(&format!("{}\n", comma(index, builtins.len())));
    }
    out.push_str("    ]\n  },\n");
    out.push_str("  \"effects\": [\n");
    let descriptors: Vec<&'static EffectDescriptorV1> = registry.descriptors().collect();
    assert_eq!(
        descriptors.len(),
        registry.len(),
        "every registered effect is emitted"
    );
    for (index, descriptor) in descriptors.iter().enumerate() {
        out.push_str(&effect(descriptor));
        out.push_str(&format!("{}\n", comma(index, descriptors.len())));
    }
    out.push_str("  ]\n}\n");
    out
}

/// Render the complete fixed browser ABI layout consumed by SDK code generation.
///
/// The values are derived directly from the public C-facing structures and frozen constants rather
/// than copied from JavaScript. This is deliberately separate from parameter metadata: its
/// vocabulary describes bytes and state-machine values, not effect descriptors.
#[must_use]
pub fn render_abi_layout() -> String {
    let prepare = [
        (
            "structSize",
            offset_of!(WebPrepareConfigV1, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebPrepareConfigV1, abi_version),
            "u32",
        ),
        (
            "sampleRateHz",
            offset_of!(WebPrepareConfigV1, sample_rate_hz),
            "u32",
        ),
        (
            "quantumFrames",
            offset_of!(WebPrepareConfigV1, quantum_frames),
            "u32",
        ),
        (
            "sessionTomlBytes",
            offset_of!(WebPrepareConfigV1, session_toml_bytes),
            "u32",
        ),
        (
            "diagnosticBytes",
            offset_of!(WebPrepareConfigV1, diagnostic_bytes),
            "u32",
        ),
        (
            "sourceIdBytes",
            offset_of!(WebPrepareConfigV1, source_id_bytes),
            "u32",
        ),
        (
            "maximumSourceChannels",
            offset_of!(WebPrepareConfigV1, maximum_source_channels),
            "u32",
        ),
        (
            "sourceRingFrames",
            offset_of!(WebPrepareConfigV1, source_ring_frames),
            "u32",
        ),
        (
            "maximumAutomationSpansPerBlock",
            offset_of!(WebPrepareConfigV1, maximum_automation_spans_per_block),
            "u32",
        ),
        (
            "maximumTracks",
            offset_of!(WebPrepareConfigV1, maximum_tracks),
            "u64",
        ),
        (
            "maximumSources",
            offset_of!(WebPrepareConfigV1, maximum_sources),
            "u64",
        ),
        (
            "maximumRoutes",
            offset_of!(WebPrepareConfigV1, maximum_routes),
            "u64",
        ),
        (
            "maximumEffects",
            offset_of!(WebPrepareConfigV1, maximum_effects),
            "u64",
        ),
        (
            "maximumGraphSessionPlusPlanBytes",
            offset_of!(WebPrepareConfigV1, maximum_graph_session_plus_plan_bytes),
            "u64",
        ),
        (
            "maximumSourceTotalBytes",
            offset_of!(WebPrepareConfigV1, maximum_source_total_bytes),
            "u64",
        ),
        (
            "maximumSourceOverheadBytes",
            offset_of!(WebPrepareConfigV1, maximum_source_overhead_bytes),
            "u64",
        ),
        (
            "maximumEffectStateBytes",
            offset_of!(WebPrepareConfigV1, maximum_effect_state_bytes),
            "u64",
        ),
        (
            "maximumEffectScratchBytes",
            offset_of!(WebPrepareConfigV1, maximum_effect_scratch_bytes),
            "u64",
        ),
        (
            "maximumBuiltinRetainedBytes",
            offset_of!(WebPrepareConfigV1, maximum_builtin_retained_bytes),
            "u64",
        ),
        (
            "maximumHostRetainedBytes",
            offset_of!(WebPrepareConfigV1, maximum_host_retained_bytes),
            "u64",
        ),
        (
            "maximumNamedAllocationBytes",
            offset_of!(WebPrepareConfigV1, maximum_named_allocation_bytes),
            "u64",
        ),
        (
            "maximumMeterStreams",
            offset_of!(WebPrepareConfigV1, maximum_meter_streams),
            "u64",
        ),
        (
            "maximumMeterItems",
            offset_of!(WebPrepareConfigV1, maximum_meter_items),
            "u64",
        ),
        (
            "maximumMeterBytes",
            offset_of!(WebPrepareConfigV1, maximum_meter_bytes),
            "u64",
        ),
        (
            "consoleCommandQueueRecords",
            offset_of!(WebPrepareConfigV1, console_command_queue_records),
            "u64",
        ),
        (
            "consoleMeterBlocks",
            offset_of!(WebPrepareConfigV1, console_meter_blocks),
            "u64",
        ),
        (
            "consoleObservationTaps",
            offset_of!(WebPrepareConfigV1, console_observation_taps),
            "u64",
        ),
        (
            "consoleMasterTrackPlusOne",
            offset_of!(WebPrepareConfigV1, console_master_track_plus_one),
            "u64",
        ),
    ];
    let status = [
        ("structSize", offset_of!(WebStatusV1, struct_size), "u32"),
        ("abiVersion", offset_of!(WebStatusV1, abi_version), "u32"),
        ("state", offset_of!(WebStatusV1, state), "u32"),
        ("lastResult", offset_of!(WebStatusV1, last_result), "u32"),
        ("backend", offset_of!(WebStatusV1, backend), "u32"),
        (
            "sampleRateHz",
            offset_of!(WebStatusV1, sample_rate_hz),
            "u32",
        ),
        (
            "quantumFrames",
            offset_of!(WebStatusV1, quantum_frames),
            "u32",
        ),
        ("reserved0", offset_of!(WebStatusV1, reserved0), "u32"),
        (
            "nextAbsoluteSample",
            offset_of!(WebStatusV1, next_absolute_sample),
            "u64",
        ),
        (
            "renderedQuanta",
            offset_of!(WebStatusV1, rendered_quanta),
            "u64",
        ),
        ("reserved", offset_of!(WebStatusV1, reserved), "u64[4]"),
    ];
    let resource = [
        (
            "structSize",
            offset_of!(WebResourceReportV1, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebResourceReportV1, abi_version),
            "u32",
        ),
        (
            "sampleRateHz",
            offset_of!(WebResourceReportV1, sample_rate_hz),
            "u32",
        ),
        (
            "quantumFrames",
            offset_of!(WebResourceReportV1, quantum_frames),
            "u32",
        ),
        ("backend", offset_of!(WebResourceReportV1, backend), "u32"),
        (
            "reserved0",
            offset_of!(WebResourceReportV1, reserved0),
            "u32[3]",
        ),
        (
            "configBytes",
            offset_of!(WebResourceReportV1, config_bytes),
            "u64",
        ),
        (
            "statusBytes",
            offset_of!(WebResourceReportV1, status_bytes),
            "u64",
        ),
        (
            "sessionTomlBytes",
            offset_of!(WebResourceReportV1, session_toml_bytes),
            "u64",
        ),
        (
            "diagnosticBytes",
            offset_of!(WebResourceReportV1, diagnostic_bytes),
            "u64",
        ),
        (
            "sourceIdBytes",
            offset_of!(WebResourceReportV1, source_id_bytes),
            "u64",
        ),
        (
            "sourcePcmStagingBytes",
            offset_of!(WebResourceReportV1, source_pcm_staging_bytes),
            "u64",
        ),
        (
            "outputPcmBytes",
            offset_of!(WebResourceReportV1, output_pcm_bytes),
            "u64",
        ),
        (
            "bridgeMetadataBytes",
            offset_of!(WebResourceReportV1, bridge_metadata_bytes),
            "u64",
        ),
        (
            "bridgeRetainedBytes",
            offset_of!(WebResourceReportV1, bridge_retained_bytes),
            "u64",
        ),
        (
            "largestBridgeAllocationBytes",
            offset_of!(WebResourceReportV1, largest_bridge_allocation_bytes),
            "u64",
        ),
        (
            "sourceTotalBytes",
            offset_of!(WebResourceReportV1, source_total_bytes),
            "u64",
        ),
        (
            "sourceOverheadBytes",
            offset_of!(WebResourceReportV1, source_overhead_bytes),
            "u64",
        ),
        (
            "effectScalarStateBytes",
            offset_of!(WebResourceReportV1, effect_scalar_state_bytes),
            "u64",
        ),
        (
            "effectScalarScratchBytes",
            offset_of!(WebResourceReportV1, effect_scalar_scratch_bytes),
            "u64",
        ),
        (
            "builtinRetainedBytes",
            offset_of!(WebResourceReportV1, builtin_retained_bytes),
            "u64",
        ),
        (
            "graphSessionPlusPlanBytes",
            offset_of!(WebResourceReportV1, graph_session_plus_plan_bytes),
            "u64",
        ),
        (
            "graphIncrementalPlanBytes",
            offset_of!(WebResourceReportV1, graph_incremental_plan_bytes),
            "u64",
        ),
        (
            "graphMetadataBytes",
            offset_of!(WebResourceReportV1, graph_metadata_bytes),
            "u64",
        ),
        (
            "graphDelayBytes",
            offset_of!(WebResourceReportV1, graph_delay_bytes),
            "u64",
        ),
        (
            "largestNamedAllocationBytes",
            offset_of!(WebResourceReportV1, largest_named_allocation_bytes),
            "u64",
        ),
        (
            "observationRetainedBytes",
            offset_of!(WebResourceReportV1, observation_retained_bytes),
            "u64",
        ),
        (
            "reserved",
            offset_of!(WebResourceReportV1, reserved),
            "u64[3]",
        ),
    ];
    let meter = [
        (
            "structSize",
            offset_of!(WebMeterHeaderV1, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebMeterHeaderV1, abi_version),
            "u32",
        ),
        (
            "trackCount",
            offset_of!(WebMeterHeaderV1, track_count),
            "u32",
        ),
        ("windows", offset_of!(WebMeterHeaderV1, windows), "u32"),
        (
            "firstSample",
            offset_of!(WebMeterHeaderV1, first_sample),
            "u64",
        ),
        ("endSample", offset_of!(WebMeterHeaderV1, end_sample), "u64"),
        ("sequence", offset_of!(WebMeterHeaderV1, sequence), "u64"),
        (
            "masterTrackPlusOne",
            offset_of!(WebMeterHeaderV1, master_track_plus_one),
            "u32",
        ),
        (
            "masterGrPresent",
            offset_of!(WebMeterHeaderV1, master_gr_present),
            "u32",
        ),
        ("reserved", offset_of!(WebMeterHeaderV1, reserved), "u64[2]"),
    ];
    let command_report = [
        (
            "structSize",
            offset_of!(WebCommandReportV1, struct_size),
            "u32",
        ),
        (
            "abiVersion",
            offset_of!(WebCommandReportV1, abi_version),
            "u32",
        ),
        ("result", offset_of!(WebCommandReportV1, result), "u32"),
        ("reason", offset_of!(WebCommandReportV1, reason), "u32"),
        (
            "rejectedIndex",
            offset_of!(WebCommandReportV1, rejected_index),
            "u32",
        ),
        ("admitted", offset_of!(WebCommandReportV1, admitted), "u32"),
        (
            "appliedAtSample",
            offset_of!(WebCommandReportV1, applied_at_sample),
            "u64",
        ),
        (
            "reserved",
            offset_of!(WebCommandReportV1, reserved),
            "u64[2]",
        ),
    ];
    let command_record = [
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
    ];
    let results = [
        (RESULT_OK, "ok"),
        (RESULT_INVALID_ARGUMENT, "invalidArgument"),
        (RESULT_ABI_MISMATCH, "abiMismatch"),
        (RESULT_WRONG_STATE, "wrongState"),
        (RESULT_BUFFER_TOO_SMALL, "bufferTooSmall"),
        (RESULT_PREPARE_REJECTED, "prepareRejected"),
        (RESULT_BACKPRESSURE, "backpressure"),
        (RESULT_UNSUPPORTED, "unsupported"),
        (RESULT_RENDER_REJECTED, "renderRejected"),
        (RESULT_REPREPARE_REQUIRED, "reprepareRequired"),
        (RESULT_INTERNAL, "internal"),
    ];
    let states = [
        (STATE_CONFIG, "config"),
        (STATE_PREPARED, "prepared"),
        (STATE_READY, "ready"),
        (STATE_FAILED, "failed"),
        (STATE_DISPOSED, "disposed"),
    ];
    let backends = [(BACKEND_SCALAR, "scalar"), (BACKEND_SIMD128, "simd128")];
    let buffers = [
        (BUFFER_SESSION_TOML, "sessionToml"),
        (BUFFER_SOURCE_ID, "sourceId"),
        (BUFFER_SOURCE_PCM, "sourcePcm"),
        (BUFFER_DIAGNOSTIC, "diagnostic"),
        (BUFFER_OUTPUT_PCM, "outputPcm"),
        (BUFFER_COMMAND, "command"),
        (BUFFER_METER_FRAME, "meterFrame"),
    ];
    // These are the complete wire values. They deliberately do not carry P2's metadata
    // `applied` claim: observation subscribe/unsubscribe are host transactions, not DSP writes.
    let wire_command_kinds = [
        (COMMAND_PAN, "pan"),
        (COMMAND_MATRIX, "matrix"),
        (COMMAND_FADER_DB, "faderDb"),
        (COMMAND_MUTE, "mute"),
        (COMMAND_EFFECT_PARAM, "effectParam"),
        (COMMAND_EFFECT_BYPASS, "effectBypass"),
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
    ];

    let mut out = String::with_capacity(1 << 14);
    out.push_str("{\n");
    out.push_str(&format!("  \"schema\": \"{ABI_LAYOUT_SCHEMA}\",\n"));
    out.push_str(&format!("  \"abiVersion\": {ABI_VERSION},\n"));
    out.push_str("  \"structures\": {\n");
    render_structure(
        &mut out,
        "prepareConfig",
        PREPARE_CONFIG_BYTES,
        &prepare,
        true,
    );
    render_structure(&mut out, "status", STATUS_BYTES, &status, true);
    render_structure(
        &mut out,
        "resourceReport",
        RESOURCE_REPORT_BYTES,
        &resource,
        true,
    );
    render_structure(&mut out, "meterHeader", METER_HEADER_BYTES, &meter, true);
    render_structure(
        &mut out,
        "commandReport",
        COMMAND_REPORT_BYTES,
        &command_report,
        false,
    );
    out.push_str("  },\n");
    out.push_str("  \"commandRecord\": {\n");
    out.push_str(&format!("    \"bytes\": {COMMAND_RECORD_BYTES},\n"));
    out.push_str("    \"endianness\": \"little\",\n");
    render_fields(&mut out, &command_record, "    ");
    out.push_str("  },\n");
    out.push_str("  \"constants\": {\n");
    render_named_constants(&mut out, "resultCodes", &results, true);
    render_named_constants(&mut out, "states", &states, true);
    render_named_constants(&mut out, "backends", &backends, true);
    render_named_constants(&mut out, "bufferKinds", &buffers, true);
    render_named_constants(&mut out, "wireCommandKinds", &wire_command_kinds, true);
    render_named_constants(&mut out, "commandReasons", &command_reasons, true);
    out.push_str(&format!(
        "    \"maximumCommandRecords\": {MAXIMUM_COMMAND_RECORDS}\n"
    ));
    out.push_str("  }\n}\n");
    out
}

fn render_structure(
    out: &mut String,
    name: &str,
    bytes: u32,
    fields: &[(&str, usize, &str)],
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

fn render_fields(out: &mut String, fields: &[(&str, usize, &str)], indent: &str) {
    out.push_str(&format!("{indent}\"fields\": [\n"));
    for (index, (name, offset, ty)) in fields.iter().enumerate() {
        out.push_str(&format!(
            "{indent}  {{ \"name\": \"{name}\", \"offset\": {offset}, \"type\": \"{ty}\" }}{}\n",
            comma(index, fields.len())
        ));
    }
    out.push_str(&format!("{indent}]\n"));
}

fn render_named_constants(
    out: &mut String,
    name: &str,
    values: &[(u32, &str)],
    trailing_comma: bool,
) {
    out.push_str(&format!("    \"{name}\": ["));
    for (index, (value, label)) in values.iter().enumerate() {
        out.push_str(&format!(
            "{{ \"value\": {value}, \"name\": \"{label}\" }}{}",
            if index + 1 == values.len() { "" } else { ", " }
        ));
    }
    out.push_str(if trailing_comma { "],\n" } else { "]\n" });
}

fn comma(index: usize, total: usize) -> &'static str {
    if index + 1 == total { "" } else { "," }
}

/// Finite `f32` as JSON. Non-finite values cannot occur: every descriptor field is validated
/// finite by `validate_descriptor_v1` before a factory may enter the registry.
fn number(value: f32) -> String {
    assert!(value.is_finite(), "descriptor values are finite");
    let text = format!("{value:?}");
    if text.contains('.') || text.contains('e') {
        text
    } else {
        format!("{text}.0")
    }
}

fn optional_number(value: Option<f32>) -> String {
    value.map_or_else(|| "null".to_owned(), number)
}

fn effect(descriptor: &EffectDescriptorV1) -> String {
    let mut out = String::new();
    out.push_str("    {\n");
    out.push_str(&format!(
        "      \"id\": \"{}\",\n",
        escape(descriptor.id.as_str())
    ));
    out.push_str(&format!(
        "      \"displayName\": \"{}\",\n",
        escape(descriptor.display_name)
    ));
    out.push_str(&format!(
        "      \"contractMajor\": {}, \"contractMinor\": {}, \"stateLayoutVersion\": {},\n",
        descriptor.contract_major, descriptor.contract_minor, descriptor.state_layout_version
    ));
    out.push_str("      \"parameters\": [\n");
    for (index, parameter) in descriptor.parameters.iter().enumerate() {
        out.push_str(&effect_parameter(parameter));
        out.push_str(&format!("{}\n", comma(index, descriptor.parameters.len())));
    }
    out.push_str("      ],\n");
    // Issue #143: never absent. An effect that declares no tap emits `[]`, so a consumer reads one
    // shape for every effect and "this build has no menu for that effect" is impossible to
    // confuse with "this document predates observation".
    out.push_str("      \"observations\": [");
    for (index, observation) in descriptor.observations.iter().enumerate() {
        out.push('\n');
        out.push_str(&effect_observation(observation));
        out.push_str(comma(index, descriptor.observations.len()));
    }
    if descriptor.observations.is_empty() {
        out.push_str("]\n    }");
    } else {
        out.push_str("\n      ]\n    }");
    }
    out
}

fn effect_observation(observation: &ObservationDescriptorV1) -> String {
    // `subscribable` is derived from the cost class exactly as `liveUpdatable` is derived from
    // `automatable`: a `Resident` tap is a copy out of state the block already wrote and the
    // subscribe path binds it; a `Computed` tap has no implementation in V1 and the subscribe path
    // answers `unsupportedKind`. The two statements are the same statement, which is why this is
    // derived rather than written down -- and why the schema gate refuses a computed tap that
    // claims to be subscribable.
    let subscribable = matches!(observation.cost, ObservationCostV1::Resident);
    format!(
        "        {{ \"id\": {}, \"name\": \"{}\", \"displayUnit\": \"{}\", \
\"kind\": {}, \"kindName\": \"{}\", \"unit\": {}, \"unitName\": \"{}\", \
\"cost\": {}, \"costName\": \"{}\", \"cadence\": {}, \"cadenceName\": \"{}\", \
\"fold\": {}, \"foldName\": \"{}\", \"channels\": {}, \"channelsName\": \"{}\", \
\"minimum\": {}, \"maximum\": {}, \"subscribable\": {} }}",
        observation.id.0,
        escape(observation.display_name),
        escape(observation.display_unit),
        observation.kind as u32,
        observation_kind_name(observation.kind),
        observation.unit as u32,
        unit_name(observation.unit),
        observation.cost as u32,
        observation_cost_name(observation.cost),
        observation.cadence as u32,
        observation_cadence_name(observation.cadence),
        observation.fold as u32,
        observation_fold_name(observation.fold),
        observation.channels as u32,
        observation_channels_name(observation.channels),
        number(observation.minimum),
        number(observation.maximum),
        subscribable,
    )
}

const fn observation_kind_name(kind: ObservationKindV1) -> &'static str {
    match kind {
        ObservationKindV1::GainReductionDb => "gainReductionDb",
    }
}

const fn observation_cost_name(cost: ObservationCostV1) -> &'static str {
    match cost {
        ObservationCostV1::Resident => "resident",
        ObservationCostV1::Computed => "computed",
    }
}

const fn observation_cadence_name(cadence: ObservationCadenceV1) -> &'static str {
    match cadence {
        ObservationCadenceV1::PerBlock => "perBlock",
        ObservationCadenceV1::PerWindow => "perWindow",
    }
}

const fn observation_fold_name(fold: ObservationFoldV1) -> &'static str {
    match fold {
        ObservationFoldV1::Latest => "latest",
        ObservationFoldV1::PeakMagnitude => "peakMagnitude",
    }
}

const fn observation_channels_name(channels: ObservationChannelsV1) -> &'static str {
    match channels {
        ObservationChannelsV1::Shared => "shared",
        ObservationChannelsV1::PerLane => "perLane",
    }
}

fn effect_parameter(parameter: &ParameterDescriptorV1) -> String {
    let mut out = String::new();
    out.push_str("        {\n");
    out.push_str(&format!("          \"id\": {},\n", parameter.id.0));
    out.push_str(&format!(
        "          \"name\": \"{}\",\n",
        escape(parameter.display_name)
    ));
    out.push_str(&format!(
        "          \"displayUnit\": \"{}\",\n",
        escape(parameter.display_unit)
    ));
    out.push_str(&format!(
        "          \"unit\": {}, \"unitName\": \"{}\",\n",
        parameter.unit as u32,
        unit_name(parameter.unit)
    ));
    out.push_str(&format!(
        "          \"domain\": {}, \"domainName\": \"{}\",\n",
        parameter.domain as u32,
        domain_name(parameter.domain)
    ));
    out.push_str(&format!(
        "          \"minimum\": {}, \"maximum\": {}, \"default\": {},\n",
        optional_number(parameter.minimum),
        optional_number(parameter.maximum),
        number(parameter.default_value)
    ));
    out.push_str(&format!(
        "          \"mapping\": {}, \"mappingName\": \"{}\",\n",
        parameter.mapping as u32,
        mapping_name(parameter.mapping)
    ));
    out.push_str(&format!(
        "          \"automationRate\": {}, \"automationRateName\": \"{}\",\n",
        parameter.automation_rate as u32,
        automation_rate_name(parameter.automation_rate)
    ));
    out.push_str(&format!(
        "          \"channelPolicy\": {}, \"channelPolicyName\": \"{}\",\n",
        parameter.channel_policy as u32,
        channel_policy_name(parameter.channel_policy)
    ));
    out.push_str(&format!(
        "          \"smoothing\": {}, \"smoothingName\": \"{}\", \"smoothingSamples\": {},\n",
        parameter.smoothing as u32,
        smoothing_name(parameter.smoothing),
        parameter.smoothing_samples
    ));
    out.push_str(&format!(
        "          \"readable\": {}, \"automatable\": {},\n",
        parameter.readable, parameter.automatable
    ));
    // Issue #140 A: the live-console command path now feeds an admitted parameter into the
    // running plan as a `PreparedAutomationSpan`, so a parameter is live exactly when its own
    // descriptor says it can be automated. The two statements are the same statement, which is
    // why this is derived from `automatable` rather than written down. A parameter that declares
    // `AutomationRate::None` has no span the effect would accept and stays `false`.
    out.push_str(&format!(
        "          \"liveUpdatable\": {},\n",
        parameter.automatable
    ));
    out.push_str("          \"enumChoices\": [");
    for (index, choice) in parameter.enum_choices.iter().enumerate() {
        out.push_str(&format!(
            "{{ \"value\": {}, \"label\": \"{}\" }}{}",
            number(choice.value),
            escape(choice.label),
            if index + 1 == parameter.enum_choices.len() {
                ""
            } else {
                ", "
            }
        ));
    }
    out.push_str("],\n");
    // Issue #127 slot. A declared null, not an absent key, so adding the ladder is additive.
    out.push_str("          \"nudge\": null\n        }");
    out
}

fn builtin_parameter(parameter: &BuiltinParameterDescriptorV1) -> String {
    // A rate-keyed cutoff has no single maximum: `builtin_filter_cutoff_maximum_hz_v1` gives one
    // per launch rate, so the row carries the exact `f32` for each rather than a number that would
    // be wrong at three of the four.
    let mut maximum_by_rate = String::from("null");
    let (minimum, maximum, domain_name) = match parameter.domain {
        BuiltinParameterDomain::BooleanExact => (None, None, "booleanExact"),
        BuiltinParameterDomain::FiniteInclusive { minimum, maximum } => {
            (Some(minimum), Some(maximum), "finiteInclusive")
        }
        BuiltinParameterDomain::DisabledOrRateKeyedHertzV1 { minimum_hz, .. } => {
            maximum_by_rate = format!(
                "{{ {} }}",
                LAUNCH_RATES_HZ
                    .iter()
                    .map(|rate| format!(
                        "\"{rate}\": {}",
                        optional_number(builtin_filter_cutoff_maximum_hz_v1(*rate))
                    ))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            (Some(minimum_hz), None, "disabledOrRateKeyedHertz")
        }
    };
    // `matrix_ll/lr/rl/rr` are the only builtin parameters the ABI declares as `BlockTarget`, and
    // they are exactly the ones the live-console command path applies. The two statements are the
    // same statement, which is why this is derived from the descriptor rather than written down.
    let live = matches!(
        parameter.update_rate,
        BuiltinParameterUpdateRate::BlockTarget
    );
    format!(
        "      {{ \"id\": {}, \"name\": \"{}\", \"scope\": \"{}\", \"mapping\": \"{}\", \
\"domain\": \"{}\", \"minimum\": {}, \"maximum\": {}, \"maximumByRate\": {}, \"default\": {}, \
\"updateRate\": \"{}\", \"smoothing\": \"{}\", \"reset\": \"{}\", \"disabledValue\": {}, \
\"liveUpdatable\": {}, \"nudge\": null }}",
        parameter.id,
        escape(parameter.name),
        match parameter.scope {
            BuiltinParameterScope::PerLane => "perLane",
            BuiltinParameterScope::MatrixShared => "matrixShared",
        },
        match parameter.mapping {
            BuiltinParameterMapping::Boolean => "boolean",
            BuiltinParameterMapping::DecibelAmplitude => "decibelAmplitude",
            BuiltinParameterMapping::Hertz => "hertz",
            BuiltinParameterMapping::Linear => "linear",
        },
        domain_name,
        optional_number(minimum),
        optional_number(maximum),
        maximum_by_rate,
        number(parameter.default),
        match parameter.update_rate {
            BuiltinParameterUpdateRate::PreparedOnly => "preparedOnly",
            BuiltinParameterUpdateRate::BlockTarget => "blockTarget",
        },
        match parameter.smoothing {
            BuiltinSmoothingPolicy::None => "none",
            BuiltinSmoothingPolicy::LinearNUpdates => "linearNUpdates",
        },
        match parameter.reset {
            BuiltinParameterReset::RestorePreparedValue => "restorePreparedValue",
            BuiltinParameterReset::KeepTargetResetCurrent => "keepTargetResetCurrent",
        },
        optional_number(parameter.disabled_value),
        live,
    )
}

const fn unit_name(unit: ParameterUnit) -> &'static str {
    match unit {
        ParameterUnit::Db => "db",
        ParameterUnit::Hz => "hz",
        ParameterUnit::Milliseconds => "milliseconds",
        ParameterUnit::Samples => "samples",
        ParameterUnit::Linear => "linear",
        ParameterUnit::Ratio => "ratio",
    }
}

const fn domain_name(domain: ParameterDomain) -> &'static str {
    match domain {
        ParameterDomain::Continuous => "continuous",
        ParameterDomain::Boolean => "boolean",
        ParameterDomain::Enumeration => "enumeration",
    }
}

const fn mapping_name(mapping: ParameterMapping) -> &'static str {
    match mapping {
        ParameterMapping::Linear => "linear",
        ParameterMapping::Logarithmic => "logarithmic",
        ParameterMapping::Exponential => "exponential",
        ParameterMapping::Stepped => "stepped",
    }
}

const fn automation_rate_name(rate: AutomationRate) -> &'static str {
    match rate {
        AutomationRate::Sample => "sample",
        AutomationRate::Block => "block",
        AutomationRate::None => "none",
    }
}

const fn channel_policy_name(policy: ParameterChannelPolicy) -> &'static str {
    match policy {
        ParameterChannelPolicy::Shared => "shared",
        ParameterChannelPolicy::PerLane => "perLane",
    }
}

const fn smoothing_name(rule: SmoothingRule) -> &'static str {
    match rule {
        SmoothingRule::None => "none",
        SmoothingRule::Linear => "linear",
        SmoothingRule::OnePole99 => "onePole99",
    }
}

/// Command-line entry point: `--write DIR`, `--check DIR`, `--print` or `--print-abi-layout`.
pub fn run() {
    let mut arguments = std::env::args().skip(1);
    let mode = arguments.next().unwrap_or_else(|| usage());
    match mode.as_str() {
        "--print" => {
            if arguments.next().is_some() {
                usage();
            }
            print!("{}", render());
        }
        "--print-abi-layout" => {
            if arguments.next().is_some() {
                usage();
            }
            print!("{}", render_abi_layout());
        }
        "--write" | "--check" => {
            let directory = PathBuf::from(arguments.next().unwrap_or_else(|| usage()));
            if arguments.next().is_some() {
                usage();
            }
            let documents = [
                (OUTPUT_NAME, render()),
                (ABI_LAYOUT_OUTPUT_NAME, render_abi_layout()),
            ];
            if mode == "--write" {
                for (name, document) in &documents {
                    let path = output_path(&directory, name);
                    let mut file = std::fs::File::create(&path).unwrap_or_else(|error| {
                        eprintln!("cannot create {}: {error}", path.display());
                        std::process::exit(2)
                    });
                    file.write_all(document.as_bytes())
                        .expect("write artifact metadata");
                    println!("wrote {}", path.display());
                }
            } else {
                for (name, document) in &documents {
                    let path = output_path(&directory, name);
                    let existing = std::fs::read_to_string(&path).unwrap_or_else(|error| {
                        eprintln!("cannot read {}: {error}", path.display());
                        std::process::exit(1)
                    });
                    if existing != document.as_str() {
                        eprintln!("{} is stale; regenerate with --write", path.display());
                        std::process::exit(1);
                    }
                    println!("{} is current", path.display());
                }
            }
        }
        _ => usage(),
    }
}
