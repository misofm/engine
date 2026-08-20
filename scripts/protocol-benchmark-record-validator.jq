def protocol_benchmark_record_valid:
    . as $record |
    $record.schema_version == 1 and
    $record.benchmark_id == "issue005_btlv_flatbuffers" and
    $record.rounds == 2 and
    ($record.round == 1 or $record.round == 2) and
    ($record.format == "btlv" or $record.format == "flatbuffers") and
    ($record.order_index == 0 or $record.order_index == 1) and
    $record.seed == "0x4953535545303035" and
    ($record.corpus_checksum | test("^[0-9a-f]{16}$")) and
    $record.counts == {
        "structural_operations": 64,
        "parameter_descriptors": 256,
        "parameter_state_records": 256,
        "automation_records": 10000,
        "automation_batches": 40,
        "meters": 256
    } and
    ($record.frame_label | type) == "string" and
    ($record.automation_records_in_frame | type) == "number" and
    (($record.encoded_bytes | type) == "number" and $record.encoded_bytes > 0) and
    ($record.encode_wall_ns_per_frame | type) == "number" and
    ($record.decode_wall_ns_per_frame | type) == "number" and
    (
        ($record.automation_records_in_frame == 0 and
            $record.encode_wall_ns_per_automation_record == null and
            $record.decode_wall_ns_per_automation_record == null) or
        ($record.automation_records_in_frame > 0 and
            ($record.encode_wall_ns_per_automation_record | type) == "number" and
            ($record.decode_wall_ns_per_automation_record | type) == "number")
    ) and
    ($record.allocation_count_after_preparation | type) == "number" and
    ($record.allocation_bytes_after_preparation | type) == "number" and
    ($record.peak_scratch_bytes | type) == "number" and
    (($record.prepared_linear_memory_bytes | type) == "number" and
        $record.prepared_linear_memory_bytes > 0) and
    ($record.malformed_rejection_wall_ns | type) == "number" and
    (
        ["toolchain", "target_cpu", "target_features", "cpu", "governor", "wasm_host", "wasm_host_version"] |
        all(. as $key | ($record[$key] | type) == "string")
    ) and
    $record.timing_scope == "native-host-harness" and
    $record.wasm_timing_scope == "not-measured-corpus-parity-only" and
    ($record.wasm_scalar_bytes | type) == "number" and
    ($record.wasm_simd128_bytes | type) == "number" and
    ($record.wasm_simd128_delta_bytes | type) == "number" and
    $record.descriptive_only == true and
    $record.threshold == null;
