.schema_version as $schema |
type == "object" and
($schema == 1 or $schema == 2) and
.issue == 880 and
.task == "MQ-2" and
.kind == "compressor_ramp_benchmark" and
.status == "measured" and
(.candidate_commit | type == "string" and test("^[0-9a-f]{40}$")) and
(.candidate_tree | type == "string" and test("^[0-9a-f]{40}$")) and
(.benchmark_source_sha256 | type == "string" and test("^[0-9a-f]{64}$")) and
(.runner_sha256 | type == "string" and test("^[0-9a-f]{64}$")) and
.sample_rate_hz == 48000 and
.quantum_frames == 128 and
.bank_width == 8 and
.bank_count == 8 and
.track_count == 64 and
.warmup_blocks_per_arm == 32 and
.measured_blocks_per_round == 32 and
.measured_rounds_per_arm == 2 and
.workload_invocations == 1 and
.ramp_frames == 64 and
.coefficient_channels == 2 and
((if $schema == 1 then .rate_coefficient_calls_per_bank_per_parameter == 1024
  else .rate_coefficient_calls_per_bank_per_parameter == 16 end)) and
.programme_seeds.left == "0x88000001" and
.programme_seeds.right == "0x88000002" and
(.cpu_model | type == "string" and length > 0) and
(.architecture | type == "string" and length > 0) and
(.os | type == "string" and length > 0) and
(.kernel | type == "string" and length > 0) and
(.rust_version | type == "string" and length > 0) and
(.llvm_version | type == "string" and length > 0) and
(.target_triple | type == "string" and length > 0) and
(.arms | type == "array" and length == 3) and
.arms[0].name == "release_only" and
.arms[1].name == "attack_and_release" and
.arms[2].name == "no_automation" and
all(.arms[]; . as $arm | $arm.ramping_parameters as $parameters |
    ((if $schema == 1 then $arm.rate_coefficient_calls_per_bank_per_parameter == 1024
      else $arm.rate_coefficient_calls_per_bank_per_parameter == 16 end)) and
    $arm.rate_coefficient_calls_per_block ==
      (8 * 8 * 2 * (if $schema == 1 then 64 else 1 end) * $parameters) and
    (.round_block_ns | type == "array" and length == 2 and
      all(.[]; type == "array" and length == 32 and all(.[]; type == "number" and . > 0))) and
    (.round_mean_us | type == "array" and length == 2 and all(.[]; type == "number" and . > 0)) and
    (.round_max_us | type == "array" and length == 2 and all(.[]; type == "number" and . > 0)) and
    all(range(0; 2) as $round |
      (([$arm.round_block_ns[$round][]] | add / length / 1000) - $arm.round_mean_us[$round] | fabs) < 0.0006 and
      (([$arm.round_block_ns[$round][]] | max / 1000) - $arm.round_max_us[$round] | fabs) < 0.0006)) and
.arms[0].ramping_parameters == 1 and
.arms[0].rate_coefficient_calls_per_block == (8 * 8 * 2 * (if $schema == 1 then 64 else 1 end)) and
.arms[1].ramping_parameters == 2 and
.arms[1].rate_coefficient_calls_per_block == (8 * 8 * 2 * (if $schema == 1 then 64 else 1 end) * 2) and
.arms[2].ramping_parameters == 0 and
.arms[2].rate_coefficient_calls_per_block == 0
