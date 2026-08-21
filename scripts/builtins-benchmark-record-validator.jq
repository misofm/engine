def hash: type == "string" and test("^[0-9a-f]{64}$");
def whole: type == "number" and floor == .;
def positive: whole and . > 0;
def render_workload: . == "full_chain_filters" or . == "identity_chain" or . == "matrix_ramp" or . == "meter_success_full";
def forbidden_zero:
  .render_allocations == 0 and .render_deallocations == 0 and .render_locks == 0 and
  .render_logs == 0 and .render_file_io == 0 and .render_network_io == 0 and .render_syscalls == 0;
def forbidden_na:
  .render_allocations == "not_applicable" and .render_deallocations == "not_applicable" and
  .render_locks == "not_applicable" and .render_logs == "not_applicable" and
  .render_file_io == "not_applicable" and .render_network_io == "not_applicable" and
  .render_syscalls == "not_applicable";
def metadata:
  (.cpu_model, .logical_cores, .os, .kernel, .governor_or_power_mode, .rust_version,
   .llvm_version, .target_triple, .target_features, .profile, .opt_level, .lto,
   .codegen_units, .background_load_note | type == "string") and
  (.missing_metadata | type == "array" and . == (sort | unique));
def percentiles:
  (.min_ns, .p50_ns, .p95_ns, .p99_ns, .p99_9_ns, .max_ns | whole and . >= 0) and
  .min_ns <= .p50_ns and .p50_ns <= .p95_ns and .p95_ns <= .p99_ns and
  .p99_ns <= .p99_9_ns and .p99_9_ns <= .max_ns;
def builtins_benchmark_record_valid:
  type == "object" and .schema_version == 2 and .issue == 7 and
  (.workload_kind | render_workload or . == "prepare_256_tracks") and
  (.workload_id | type == "string" and test("^issue007\\.(full_chain_filters|identity_chain|matrix_ramp|meter_success_full|prepare_256_tracks)\\.(48000|96000)hz\\.q128$")) and
  (.sample_rate_hz == 48000 or .sample_rate_hz == 96000) and .quantum_frames == 128 and
  (.round == 1 or .round == 2) and .frames_per_operation == 128 and
  (.tracks | positive) and (.meter_observers | whole and . >= 0) and
  (.meter_queue_capacity | whole and . >= 0) and (.retained_payload_bytes | whole and . >= 0) and
  .percentile_method == "nearest_rank" and percentiles and
  .fixture_manifest_id == "fixtures/builtins/v1/MANIFEST.tsv" and (.fixture_manifest_sha256 | hash) and
  .input_fixture_id == "fixtures/builtins/v1/MANIFEST.tsv" and (.input_fixture_sha256 | hash) and
  (.output_sha256 | hash) and metadata and
  (if .workload_kind | render_workload then
     .render_scope == "render" and .warmup_batches == 64 and .measured_batches == 512 and
     .operations_per_batch == 8 and forbidden_zero
   else
     .render_scope == "not_applicable_preparation" and .warmup_batches == 16 and
     .measured_batches == 128 and .operations_per_batch == 1 and .tracks == 256 and
     .meter_observers == 56 and .meter_queue_capacity == 4 and forbidden_na
   end);
