def exact_keys: [
  "schema_version", "issue", "workload_kind", "workload_id", "sample_rate_hz",
  "quantum_frames", "round", "render_scope", "warmup_batches", "measured_batches",
  "operations_per_batch", "total_operations", "frames_per_operation", "tracks",
  "meter_observers", "meter_queue_capacity", "retained_payload_bytes",
  "percentile_method", "units", "min_ns", "p50_ns", "p95_ns", "p99_ns",
  "p99_9_ns", "max_ns", "descriptive_only", "candidate_commit", "binary_sha256",
  "fixture_manifest_id", "fixture_manifest_sha256", "input_fixture_id",
  "input_fixture_sha256", "output_sha256", "render_errors", "render_allocations",
  "render_deallocations", "render_locks", "render_logs", "render_file_io",
  "render_network_io", "render_syscalls", "render_feature_detection",
  "render_panic_unwind", "render_total_forbidden_operations", "cpu_model",
  "cpu_architecture", "logical_core_count", "physical_core_count", "os", "kernel",
  "governor_or_power_mode", "rust_version", "llvm_version", "target_triple",
  "target_features", "profile", "opt_level", "lto", "codegen_units",
  "background_load_note", "missing_metadata"
];

def metadata_fields: [
  "cpu_model", "cpu_architecture", "logical_core_count", "physical_core_count", "os",
  "kernel", "governor_or_power_mode", "rust_version", "llvm_version", "target_triple",
  "target_features", "profile", "opt_level", "lto", "codegen_units",
  "background_load_note"
];

def hash: type == "string" and test("^[0-9a-f]{64}$");
def commit: type == "string" and test("^[0-9a-f]{40}$");
def whole: type == "number" and floor == .;
def nonnegative: whole and . >= 0;
def positive: whole and . > 0;
def usable_text: type == "string" and length > 0 and . != "unknown" and . != "default";
def render_workload:
  . == "full_chain_filters" or . == "identity_chain" or . == "matrix_ramp" or . == "meter_success_full";
def frozen_manifest_sha256:
  "bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff";
def frozen_input_sha256:
  if .workload_kind == "full_chain_filters" and .sample_rate_hz == 48000 then
    "4e5e2c9fc8e2c2400b816715273879f3635f2374133e5775ade18dabee1f6ad9"
  elif .workload_kind == "full_chain_filters" and .sample_rate_hz == 96000 then
    "cc4f23f6579cc255a1282797de2b78c93951f947c7b0ab72fa2ca713780f8a1e"
  elif .workload_kind == "identity_chain" and .sample_rate_hz == 48000 then
    "65232ba5a59f54a22762a6ebc82620be6332f9d583c0e61fe4c5d82ede23e7ac"
  elif .workload_kind == "identity_chain" and .sample_rate_hz == 96000 then
    "9bc765fb84d94dd31f83137e2aa091fd09a28a8dab8fbe1d18a0b4a9a60c85a7"
  elif .workload_kind == "matrix_ramp" and .sample_rate_hz == 48000 then
    "f0d94928bed16804a26befde5eaabd3a8c233afa194a5cdcb259141af78c831b"
  elif .workload_kind == "matrix_ramp" and .sample_rate_hz == 96000 then
    "ef5bf8c4e954c1e497eea997bffeb85fabad69ac6966f2798bd34ce2fa5ced6f"
  elif .workload_kind == "meter_success_full" and .sample_rate_hz == 48000 then
    "ded3579ee8ffbf79d920648a33a7e2f35fa9c9b386e98ef469d583830ef992de"
  elif .workload_kind == "meter_success_full" and .sample_rate_hz == 96000 then
    "aa1c4d8835753ce290d7abcf1cbf3ffdb98b79a58f0ec6cd0cce6614f5befef9"
  elif .workload_kind == "prepare_256_tracks" and .sample_rate_hz == 48000 then
    "0c2130e5f3563e011cc7251a4a42d27b2a84f5871a81facae49be0a5c1cf21ff"
  elif .workload_kind == "prepare_256_tracks" and .sample_rate_hz == 96000 then
    "5ca5e3b6e0080b66c53f0a12753e3681ea1caf6571ff3747e2303ac8cf0779a6"
  else null
  end;
def input_id:
  .workload_id == ("issue035." + .workload_kind + "." + (.sample_rate_hz | tostring) + "hz.q128") and
  .input_fixture_id == ("fixtures/builtins/v1/benchmark/" + .workload_kind + "-" + (.sample_rate_hz | tostring) + ".toml") and
  .input_fixture_sha256 == frozen_input_sha256;
def audit_names: [
  "render_allocations", "render_deallocations", "render_locks", "render_logs",
  "render_file_io", "render_network_io", "render_syscalls", "render_feature_detection",
  "render_panic_unwind"
];
def audit_total: [.render_allocations, .render_deallocations, .render_locks, .render_logs,
                  .render_file_io, .render_network_io, .render_syscalls,
                  .render_feature_detection, .render_panic_unwind] | add;
def audit_values_zero:
  . as $record | all(audit_names[]; . as $name | $record[$name] == 0);
def audit_values_not_applicable:
  . as $record | all(audit_names[]; . as $name | $record[$name] == "not_applicable");
def missing_metadata_fields:
  . as $record |
  [metadata_fields[] as $field | select($record[$field] == null) | $field];
def metadata:
  . as $record |
  (all(["cpu_model", "cpu_architecture", "os", "kernel", "governor_or_power_mode",
        "rust_version", "llvm_version", "target_triple", "target_features", "profile",
        "opt_level", "lto", "codegen_units", "background_load_note"][];
       . as $field | $record[$field] == null or ($record[$field] | usable_text))) and
  ($record.logical_core_count == null or ($record.logical_core_count | positive)) and
  ($record.physical_core_count == null or ($record.physical_core_count | positive)) and
  ($record.missing_metadata | type == "array" and all(.[]; type == "string") and . == (sort | unique)) and
  ($record.missing_metadata == ($record | missing_metadata_fields | sort));
def percentiles:
  (.min_ns, .p50_ns, .p95_ns, .p99_ns, .p99_9_ns, .max_ns | nonnegative) and
  .min_ns <= .p50_ns and .p50_ns <= .p95_ns and .p95_ns <= .p99_ns and
  .p99_ns <= .p99_9_ns and .p99_9_ns <= .max_ns;
def render_audit:
  .render_errors == 0 and audit_values_zero and
  .render_total_forbidden_operations == audit_total;
def preparation_audit:
  .render_errors == "not_applicable" and
  audit_values_not_applicable and
  .render_total_forbidden_operations == "not_applicable";
def render_shape:
  .render_scope == "render" and .warmup_batches == 64 and .measured_batches == 512 and
  .operations_per_batch == 8 and .total_operations == 4096 and .frames_per_operation == 128 and
  .tracks == 1 and .retained_payload_bytes == 0 and render_audit and
  (if .workload_kind == "meter_success_full"
   then .meter_observers == 14 and .meter_queue_capacity == 1
   else .meter_observers == 0 and .meter_queue_capacity == null
   end);
def preparation_shape:
  .render_scope == "not_applicable_preparation" and .warmup_batches == 16 and
  .measured_batches == 128 and .operations_per_batch == 1 and .total_operations == 128 and
  .frames_per_operation == null and .tracks == 256 and .meter_observers == 56 and
  .meter_queue_capacity == 4 and (.retained_payload_bytes | nonnegative) and preparation_audit;
def builtins_benchmark_record_valid:
  type == "object" and (keys | sort) == (exact_keys | sort) and
  .schema_version == 2 and .issue == 35 and
  (.workload_kind | render_workload or . == "prepare_256_tracks") and
  (.sample_rate_hz == 48000 or .sample_rate_hz == 96000) and .quantum_frames == 128 and
  (.round == 1 or .round == 2) and input_id and .percentile_method == "nearest_rank" and
  .units == "ns_per_operation" and .descriptive_only == true and percentiles and
  (.candidate_commit | commit) and (.binary_sha256 | hash) and
  .fixture_manifest_id == "fixtures/builtins/v1/MANIFEST.tsv" and
  .fixture_manifest_sha256 == frozen_manifest_sha256 and
  (.output_sha256 | hash) and metadata and
  (if .workload_kind | render_workload then render_shape else preparation_shape end);
