def graph_benchmark_record_valid:
    . as $record |
    $record.schema_version == 1 and
    ($record.benchmark_id == "graph_compile_256t_1024r_32s" or
        $record.benchmark_id == "graph_validate_65537_tracks" or
        $record.benchmark_id == "graph_debug_sha_dot_256t_1024r_32s") and
    $record.rounds == 2 and ($record.round == 1 or $record.round == 2) and
    ($record.fixture_label | type) == "string" and
    ($record.fixture_sha256 | test("^[0-9a-f]{64}$")) and
    (($record.fixture_bytes | type) == "number" and $record.fixture_bytes > 0) and
    (
        if $record.benchmark_id == "graph_validate_65537_tracks" then
            $record.fixture_counts == {
                "tracks": 65537, "routes": 1, "submixes": 0, "effects": 0, "sidechains": 0
            } and $record.warmup_iterations == 0 and $record.measured_iterations == 1
        else
            $record.fixture_counts == {
                "tracks": 256, "routes": 1024, "submixes": 32, "effects": 64, "sidechains": 32
            } and $record.warmup_iterations == 1 and $record.measured_iterations == 5
        end
    ) and
    $record.sample_rate_hz == 48000 and $record.quantum_frames == 128 and
    $record.percentile_method == "nearest-rank" and
    ($record.timing_ns | [
        .min, .p50, .p95, .p99, .p99_9, .max,
        .effect_prepare_p50, .graph_compile_p50
    ] | all(type == "number" and . >= 0)) and
    ($record.timing_ns.min <= $record.timing_ns.p50 and
        $record.timing_ns.p50 <= $record.timing_ns.p95 and
        $record.timing_ns.p95 <= $record.timing_ns.p99 and
        $record.timing_ns.p99 <= $record.timing_ns.p99_9 and
        $record.timing_ns.p99_9 <= $record.timing_ns.max) and
    ($record.output_graph_sha256 | test("^[0-9a-f]{64}$")) and
    ($record.output_counts | [
        .logical_nodes, .materialized_nodes, .edges, .schedule_items,
        .dependency_levels, .routes, .canonical_debug_bytes, .dot_bytes
    ] | all(type == "number" and . > 0)) and
    (($record.output_counts.effects | type) == "number" and
        $record.output_counts.effects == $record.fixture_counts.effects) and
    ($record.memory | [
        .peak_resident_bytes, .estimated_plan_bytes,
        .estimated_session_plus_plan_bytes, .largest_allocation_bytes
    ] | all(type == "number" and . >= 0)) and
    (($record.timestamp_epoch_seconds | type) == "number" and
        $record.timestamp_epoch_seconds > 0) and
    ([
        "cpu", "os", "governor_or_power_mode", "power_source", "rustc", "llvm",
        "target_triple", "target_features", "opt_level", "lto", "codegen_units",
        "background_load"
    ] | all(. as $key | ($record[$key] | type) == "string" and ($record[$key] | length) > 0)) and
    (($record.missing_metadata | type) == "array" and
        ($record.missing_metadata | all(type == "string"))) and
    $record.metadata_incomplete == (($record.missing_metadata | length) > 0) and
    $record.errors == 0 and $record.descriptive_only == true and $record.threshold == null;
