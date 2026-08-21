def sha256: type == "string" and test("^[0-9a-f]{64}$");
def hash64: type == "string" and test("^[0-9a-f]{16}$");
def nonnegative_integer: type == "number" and floor == . and . >= 0;
def exact_keys: ["binary_sha256","candidate_sha256","coordinator_forbidden_total","cpu_model","descriptive_only","governor_or_power_mode","issue","kernel","llvm_version","max","min","mode","observations","os","output_hash","p50","p95","p99","partition_count","percentile_method","quantum_frames","render_errors","retained_bytes","round","rust_version","sample_rate_hz","schema_version","selected_lanes","unit_count","units","wave_count","worker_count","worker_forbidden_total"];
def mode_shape:
  if .mode == "sequential" then .selected_lanes == 1 and .worker_count == 0
  elif .mode == "two_lane" then .selected_lanes == 2 and .worker_count == 1
  elif .mode == "four_lane" then .selected_lanes == 4 and .worker_count == 3
  else false end;
def scheduler_benchmark_record_valid:
  (keys | sort) == exact_keys and
  .schema_version == 1 and .issue == 9 and (.round == 1 or .round == 2) and
  .sample_rate_hz == 48000 and .quantum_frames == 128 and .observations == 1000 and
  .percentile_method == "nearest_rank" and .units == "ns_per_frame" and .descriptive_only == true and
  mode_shape and
  ([.min,.p50,.p95,.p99,.max,.selected_lanes,.worker_count,.wave_count,.unit_count,.partition_count,.retained_bytes,.render_errors,.coordinator_forbidden_total,.worker_forbidden_total] | all(nonnegative_integer)) and
  .min <= .p50 and .p50 <= .p95 and .p95 <= .p99 and .p99 <= .max and
  .wave_count > 1 and .unit_count > 1 and .partition_count > 1 and .retained_bytes > 0 and
  .render_errors == 0 and .coordinator_forbidden_total == 0 and .worker_forbidden_total == 0 and
  (.output_hash | hash64) and (.candidate_sha256 | sha256) and (.binary_sha256 | sha256) and
  ([.cpu_model,.os,.kernel,.rust_version,.llvm_version,.governor_or_power_mode] | all(type == "string" and length > 0));
