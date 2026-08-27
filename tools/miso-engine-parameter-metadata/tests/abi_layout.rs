//! Issue #207 E0b: the emitted ABI layout is derived from the real public C structures.

use core::mem::{offset_of, size_of};

use miso_engine_host_web::{
    WebCommandReportV1, WebMeterHeaderV1, WebPrepareConfigV1, WebResourceReportV1, WebStatusV1,
};

type Field = (&'static str, usize, &'static str);

macro_rules! fields {
    ($structure:ty; $(($name:literal, $field:ident, $ty:literal)),+ $(,)?) => {
        &[$(($name, offset_of!($structure, $field), $ty)),+]
    };
}

fn structure_section<'a>(document: &'a str, name: &str, bytes: usize) -> &'a str {
    let marker = format!("\"{name}\": {{\n      \"bytes\": {bytes},\n      \"fields\": [\n");
    let tail = document
        .split_once(&marker)
        .unwrap_or_else(|| panic!("missing structure header: {name}"))
        .1;
    tail.split_once("\n    }")
        .unwrap_or_else(|| panic!("unterminated structure: {name}"))
        .0
}

fn command_record_section(document: &str) -> &str {
    let marker = "\"commandRecord\": {\n    \"bytes\": 48,\n    \"endianness\": \"little\",\n    \"fields\": [\n";
    let tail = document
        .split_once(marker)
        .expect("command record header")
        .1;
    tail.split_once("\n  },")
        .expect("command record terminator")
        .0
}

fn require_fields(section: &str, owner: &str, expected: &[Field]) {
    assert_eq!(
        section.matches("\"name\":").count(),
        expected.len(),
        "{owner} field count"
    );
    for (name, offset, ty) in expected {
        let field = format!("\"name\": \"{name}\", \"offset\": {offset}, \"type\": \"{ty}\"");
        assert!(section.contains(&field), "{owner} missing field: {field}");
    }
}

fn require_layout(document: &str) {
    require_fields(
        structure_section(document, "prepareConfig", size_of::<WebPrepareConfigV1>()),
        "prepareConfig",
        fields!(WebPrepareConfigV1;
            ("structSize", struct_size, "u32"), ("abiVersion", abi_version, "u32"),
            ("sampleRateHz", sample_rate_hz, "u32"), ("quantumFrames", quantum_frames, "u32"),
            ("sessionTomlBytes", session_toml_bytes, "u32"), ("diagnosticBytes", diagnostic_bytes, "u32"),
            ("sourceIdBytes", source_id_bytes, "u32"), ("maximumSourceChannels", maximum_source_channels, "u32"),
            ("sourceRingFrames", source_ring_frames, "u32"), ("maximumAutomationSpansPerBlock", maximum_automation_spans_per_block, "u32"),
            ("maximumTracks", maximum_tracks, "u64"), ("maximumSources", maximum_sources, "u64"),
            ("maximumRoutes", maximum_routes, "u64"), ("maximumEffects", maximum_effects, "u64"),
            ("maximumGraphSessionPlusPlanBytes", maximum_graph_session_plus_plan_bytes, "u64"),
            ("maximumSourceTotalBytes", maximum_source_total_bytes, "u64"),
            ("maximumSourceOverheadBytes", maximum_source_overhead_bytes, "u64"),
            ("maximumEffectStateBytes", maximum_effect_state_bytes, "u64"),
            ("maximumEffectScratchBytes", maximum_effect_scratch_bytes, "u64"),
            ("maximumBuiltinRetainedBytes", maximum_builtin_retained_bytes, "u64"),
            ("maximumHostRetainedBytes", maximum_host_retained_bytes, "u64"),
            ("maximumNamedAllocationBytes", maximum_named_allocation_bytes, "u64"),
            ("maximumMeterStreams", maximum_meter_streams, "u64"), ("maximumMeterItems", maximum_meter_items, "u64"),
            ("maximumMeterBytes", maximum_meter_bytes, "u64"),
            ("consoleCommandQueueRecords", console_command_queue_records, "u64"),
            ("consoleMeterBlocks", console_meter_blocks, "u64"),
            ("consoleObservationTaps", console_observation_taps, "u64"),
            ("consoleMasterTrackPlusOne", console_master_track_plus_one, "u64"),
        ),
    );
    require_fields(
        structure_section(document, "status", size_of::<WebStatusV1>()),
        "status",
        fields!(WebStatusV1;
            ("structSize", struct_size, "u32"), ("abiVersion", abi_version, "u32"),
            ("state", state, "u32"), ("lastResult", last_result, "u32"), ("backend", backend, "u32"),
            ("sampleRateHz", sample_rate_hz, "u32"), ("quantumFrames", quantum_frames, "u32"),
            ("reserved0", reserved0, "u32"), ("nextAbsoluteSample", next_absolute_sample, "u64"),
            ("renderedQuanta", rendered_quanta, "u64"), ("reserved", reserved, "u64[4]"),
        ),
    );
    require_fields(
        structure_section(document, "resourceReport", size_of::<WebResourceReportV1>()),
        "resourceReport",
        fields!(WebResourceReportV1;
            ("structSize", struct_size, "u32"), ("abiVersion", abi_version, "u32"),
            ("sampleRateHz", sample_rate_hz, "u32"), ("quantumFrames", quantum_frames, "u32"),
            ("backend", backend, "u32"), ("reserved0", reserved0, "u32[3]"),
            ("configBytes", config_bytes, "u64"), ("statusBytes", status_bytes, "u64"),
            ("sessionTomlBytes", session_toml_bytes, "u64"), ("diagnosticBytes", diagnostic_bytes, "u64"),
            ("sourceIdBytes", source_id_bytes, "u64"), ("sourcePcmStagingBytes", source_pcm_staging_bytes, "u64"),
            ("outputPcmBytes", output_pcm_bytes, "u64"), ("bridgeMetadataBytes", bridge_metadata_bytes, "u64"),
            ("bridgeRetainedBytes", bridge_retained_bytes, "u64"), ("largestBridgeAllocationBytes", largest_bridge_allocation_bytes, "u64"),
            ("sourceTotalBytes", source_total_bytes, "u64"), ("sourceOverheadBytes", source_overhead_bytes, "u64"),
            ("effectScalarStateBytes", effect_scalar_state_bytes, "u64"), ("effectScalarScratchBytes", effect_scalar_scratch_bytes, "u64"),
            ("builtinRetainedBytes", builtin_retained_bytes, "u64"), ("graphSessionPlusPlanBytes", graph_session_plus_plan_bytes, "u64"),
            ("graphIncrementalPlanBytes", graph_incremental_plan_bytes, "u64"), ("graphMetadataBytes", graph_metadata_bytes, "u64"),
            ("graphDelayBytes", graph_delay_bytes, "u64"), ("largestNamedAllocationBytes", largest_named_allocation_bytes, "u64"),
            ("observationRetainedBytes", observation_retained_bytes, "u64"), ("reserved", reserved, "u64[3]"),
        ),
    );
    require_fields(
        structure_section(document, "meterHeader", size_of::<WebMeterHeaderV1>()),
        "meterHeader",
        fields!(WebMeterHeaderV1;
            ("structSize", struct_size, "u32"), ("abiVersion", abi_version, "u32"),
            ("trackCount", track_count, "u32"), ("windows", windows, "u32"),
            ("firstSample", first_sample, "u64"), ("endSample", end_sample, "u64"),
            ("sequence", sequence, "u64"), ("masterTrackPlusOne", master_track_plus_one, "u32"),
            ("masterGrPresent", master_gr_present, "u32"), ("reserved", reserved, "u64[2]"),
        ),
    );
    require_fields(
        structure_section(document, "commandReport", size_of::<WebCommandReportV1>()),
        "commandReport",
        fields!(WebCommandReportV1;
            ("structSize", struct_size, "u32"), ("abiVersion", abi_version, "u32"),
            ("result", result, "u32"), ("reason", reason, "u32"),
            ("rejectedIndex", rejected_index, "u32"), ("admitted", admitted, "u32"),
            ("appliedAtSample", applied_at_sample, "u64"), ("reserved", reserved, "u64[2]"),
        ),
    );
    require_fields(
        command_record_section(document),
        "commandRecord",
        &[
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
        ],
    );
}

#[test]
fn emitted_layout_matches_real_repr_c_structures() {
    require_layout(&miso_engine_parameter_metadata::render_abi_layout());
}

/// A wrong `prepareConfig` offset must not be satisfied by an identically named field elsewhere.
#[test]
fn scoped_layout_checker_rejects_one_structure_specific_offset_mutation() {
    let document = miso_engine_parameter_metadata::render_abi_layout();
    let mutated = document.replacen(
        "\"name\": \"maximumTracks\", \"offset\": 40, \"type\": \"u64\"",
        "\"name\": \"maximumTracks\", \"offset\": 41, \"type\": \"u64\"",
        1,
    );
    assert_ne!(
        document, mutated,
        "red mutation must match the generated layout"
    );
    assert!(std::panic::catch_unwind(|| require_layout(&mutated)).is_err());
}
