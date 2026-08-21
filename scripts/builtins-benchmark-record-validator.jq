def builtins_benchmark_record_valid:
  type == "object" and
  .schema_version == 1 and
  (.workload | type == "string") and
  (.round | type == "number" and (. == 1 or . == 2)) and
  (.observations | type == "number" and . > 0) and
  (.timing_ns | type == "number" and . >= 0) and
  .sample_rate_hz == 48000 and
  .quantum_frames == 128 and
  .allocations == 0 and
  .deallocations == 0 and
  .descriptive_only == true;
