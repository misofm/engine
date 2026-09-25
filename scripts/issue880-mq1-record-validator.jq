type == "object" and
.schema_version == 1 and
.issue == 880 and
.task == "MQ-1" and
.kind == "transient_shaper_benchmark" and
.status == "measured" and
(.candidate_commit | type == "string" and test("^[0-9a-f]{40}$")) and
(.engine_effect_commit | type == "string" and test("^[0-9a-f]{40}$")) and
.engine_effect_commit == "6f662fee7b47a5eb38b67e0ddc6d007edd438cfa" and
.engine_effect_revision == "E1 (MA-3)" and
(.benchmark_source_sha256 | type == "string" and test("^[0-9a-f]{64}$")) and
(.runner_sha256 | type == "string" and test("^[0-9a-f]{64}$")) and
.sample_rate_hz == 48000 and
.duration_seconds == 4 and
.frames == 192000 and
.block_frames == 128 and
.track_count == 8 and
.link_mode == "dual_mono" and
.attack == 0.75 and
.sustain == -0.5 and
.mix == 1.0 and
.programme_seeds.left == "0x88000001" and
.programme_seeds.right == "0x88000002" and
(.fixture_sha256 | type == "string" and test("^[0-9a-f]{64}$")) and
.workload_invocations == 1 and
.warmups_per_arm == 1 and
.measured_rounds_per_arm == 2 and
.bank_width == 8 and
(.bank_ns_per_lane_sample | type == "array" and length == 2 and all(.[]; type == "number" and . > 0)) and
.scalar_width == 1 and
(.scalar_ns_per_lane_sample | type == "array" and length == 2 and all(.[]; type == "number" and . > 0)) and
(.cpu_model | type == "string" and length > 0) and
(.architecture | type == "string" and length > 0) and
(.os | type == "string" and length > 0) and
(.kernel | type == "string" and length > 0) and
(.rust_version | type == "string" and length > 0) and
(.llvm_version | type == "string" and length > 0) and
(.target_triple | type == "string" and length > 0)
