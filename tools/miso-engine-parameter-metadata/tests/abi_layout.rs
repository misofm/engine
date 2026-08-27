//! Issue #207 E0b: the emitted ABI layout is derived from the real public C structures.

use core::mem::{offset_of, size_of};

use miso_engine_host_web::{
    WebCommandReportV1, WebMeterHeaderV1, WebPrepareConfigV1, WebResourceReportV1, WebStatusV1,
};

fn require_field(document: &str, name: &str, offset: usize, ty: &str) {
    let expected = format!("\"name\": \"{name}\", \"offset\": {offset}, \"type\": \"{ty}\"");
    assert!(
        document.contains(&expected),
        "missing ABI field: {expected}"
    );
}

/// A field-order change must update the emitted ABI layout, and the artifact `--check` then refuses
/// the stale checked-in/generated copy. This test is the emitter-side half of that contract.
#[test]
fn emitted_layout_matches_real_repr_c_structures() {
    let document = miso_engine_parameter_metadata::render_abi_layout();
    for (name, bytes) in [
        ("prepareConfig", size_of::<WebPrepareConfigV1>()),
        ("status", size_of::<WebStatusV1>()),
        ("resourceReport", size_of::<WebResourceReportV1>()),
        ("meterHeader", size_of::<WebMeterHeaderV1>()),
        ("commandReport", size_of::<WebCommandReportV1>()),
    ] {
        assert!(
            document.contains(&format!("\"{name}\": {{\n      \"bytes\": {bytes}")),
            "missing ABI structure: {name}"
        );
    }

    for (name, offset, ty) in [
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
    ] {
        require_field(&document, name, offset, ty);
    }
    for (name, offset, ty) in [
        ("state", offset_of!(WebStatusV1, state), "u32"),
        ("lastResult", offset_of!(WebStatusV1, last_result), "u32"),
        ("backend", offset_of!(WebStatusV1, backend), "u32"),
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
    ] {
        require_field(&document, name, offset, ty);
    }
}
