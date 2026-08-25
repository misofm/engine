# Shared definitions for the Issue-149 console qualification benchmark records.
#
# Two record shapes share one stream. `console_session` is a workload rendered through a real
# prepared plan; `console_hoist` is the paired-alternation comparison of the stationary-smoother
# arms. `console_benchmark_record_valid_lib` dispatches on `.record`, so a record that claims one
# shape and carries the other's keys fails rather than being validated against the wrong table.
def sha256: type == "string" and test("^[0-9a-f]{64}$");
def nonnegative_integer: type == "number" and floor == . and . >= 0;
def positive_integer: type == "number" and floor == . and . > 0;

# The nine runner-supplied metadata names, as they appear in a record.
def metadata_names: ["background_load_note","candidate_commit","cpu_model","governor_or_power_mode","llvm_version","profile","rust_version","target_features","target_triple"];

def session_keys: ["backend","background_load_note","candidate_commit","cpu_model","descriptive_only","fixture_id","governor_or_power_mode","issue","llvm_version","max_ns_per_block","max_us_per_block","min_ns_per_block","min_us_per_block","missing_metadata","observations","os","output_sha256","p50_ns_per_block","p50_us_per_block","p50_us_per_block_per_track","p95_ns_per_block","p95_us_per_block","p99_ns_per_block","p99_us_per_block","percentile_method","profile","quantum_frames","record","render_errors","render_total_forbidden_operations","round","rust_version","sample_rate_hz","schema_version","statistical_method","synthetic_fixture","target_features","target_triple","tracks","units","workload_kind"];

def hoist_keys: ["arms","backend","background_load_note","bank_boundary","bit_identity","candidate_commit","cpu_model","descriptive_only","governor_or_power_mode","issue","llvm_version","missing_metadata","moving_output_sha256","moving_p50_ns","moving_p95_ns","moving_p99_ns","observations","os","paired_delta_median_ns","pairing","percentile_method","profile","quiet_output_sha256","quiet_p50_ns","quiet_p99_ns","record","restated_output_sha256","restated_p50_ns","restated_p95_ns","restated_p99_ns","round","rust_version","schema_version","statistical_method","target_features","target_triple","tracks","units","workload_kind"];

# Every workload names its track count and whether its fixture is written or synthesised. A
# synthetic row that claimed to be a checked-in fixture would be exactly the "measuring a fiction"
# failure the bench discipline exists to catch, so the pairing is pinned per kind.
def session_kind_shape:
  if .workload_kind == "nine_track_baseline" then
    .tracks == 9 and .synthetic_fixture == false and
    .fixture_id == "fixtures/session/v1/parametric-eq-nine-track.toml"
  elif .workload_kind == "nine_track_ragged_strip" then
    .tracks == 9 and .synthetic_fixture == true and
    .fixture_id == "fixtures/session/v1/console-sixty-four-track.toml"
  elif .workload_kind == "sixty_four_track_console" then
    .tracks == 64 and .synthetic_fixture == false and
    .fixture_id == "fixtures/session/v1/console-sixty-four-track.toml"
  elif .workload_kind == "one_twenty_eight_track_stretch" then
    .tracks == 128 and .synthetic_fixture == true and
    .fixture_id == "fixtures/session/v1/console-sixty-four-track.toml"
  else false end;

def hoist_kind_shape:
  if .workload_kind == "nine_track_ragged_strip" then .tracks == 9
  elif .workload_kind == "sixty_four_track_console" then .tracks == 64
  else false end;

# #104 F2: a runner that forgets to export a name must not produce an all-null record that still
# passes. `missing_metadata` has to be exactly the sorted set of names that came back null, and a
# name that did resolve must carry real text rather than a placeholder.
def honest_metadata:
  . as $record |
  (metadata_names | map(select(. as $key | $record[$key] == null)) | sort) as $absent |
  (.missing_metadata == $absent) and
  (.missing_metadata | . == (. | sort) and . == (. | unique)) and
  (metadata_names | all(. as $key |
    ($record[$key] == null or
     ($record[$key] | type == "string" and length > 0 and . != "unknown" and . != "default"))));

def ordered_percentiles($fields):
  ($fields | all(nonnegative_integer)) and
  ($fields | . == (. | sort));

def session_statistical_method:
  "nearest-rank percentiles over per-block nanoseconds; one warmup pass and two measured rounds; descriptive only; no threshold";
def hoist_statistical_method:
  "three arms alternated per observation; nearest-rank percentiles over per-block nanoseconds; paired delta is moving minus restated per observation; descriptive only; no threshold";

def session_record_valid:
  (keys | sort) == session_keys and
  .record == "console_session" and .schema_version == 1 and .issue == 149 and
  # The method is pinned verbatim. A record that changed how it was measured but kept the old
  # sentence would be the most expensive kind of quiet drift, so the sentence is part of the shape.
  .statistical_method == session_statistical_method and
  (.os | type == "string" and length > 0) and
  (.missing_metadata | type == "array" and all(.[]; type == "string")) and
  (.round == 1 or .round == 2) and
  .sample_rate_hz == 48000 and .quantum_frames == 128 and .observations == 1000 and
  .percentile_method == "nearest_rank" and .units == "us_per_block" and
  .descriptive_only == true and
  (.backend | type == "string" and length > 0) and
  session_kind_shape and
  ordered_percentiles([.min_ns_per_block,.p50_ns_per_block,.p95_ns_per_block,.p99_ns_per_block,.max_ns_per_block]) and
  ([.min_us_per_block,.p50_us_per_block,.p95_us_per_block,.p99_us_per_block,.max_us_per_block,.p50_us_per_block_per_track] | all(type == "number" and . > 0)) and
  (.output_sha256 | sha256) and
  .render_errors == 0 and .render_total_forbidden_operations == 0 and
  honest_metadata;

def hoist_record_valid:
  (keys | sort) == hoist_keys and
  .record == "console_hoist" and .schema_version == 1 and .issue == 149 and
  .statistical_method == hoist_statistical_method and
  (.os | type == "string" and length > 0) and
  (.backend | type == "string" and length > 0) and
  (.missing_metadata | type == "array" and all(.[]; type == "string")) and
  (.round == 1 or .round == 2) and
  .observations == 1000 and .percentile_method == "nearest_rank" and
  .units == "ns_per_block" and .descriptive_only == true and
  .pairing == "alternating_per_observation" and
  .arms == ["quiet","restated","moving"] and
  .bank_boundary == "effect_bank" and
  hoist_kind_shape and
  ([.quiet_p50_ns,.quiet_p99_ns,.restated_p50_ns,.restated_p95_ns,.restated_p99_ns,.moving_p50_ns,.moving_p95_ns,.moving_p99_ns] | all(positive_integer)) and
  (.restated_p50_ns <= .restated_p95_ns and .restated_p95_ns <= .restated_p99_ns) and
  (.moving_p50_ns <= .moving_p95_ns and .moving_p95_ns <= .moving_p99_ns) and
  (.paired_delta_median_ns | type == "number" and floor == .) and
  ([.quiet_output_sha256,.restated_output_sha256,.moving_output_sha256] | all(sha256)) and
  # The class-A statement, carried in the record rather than only in a commit message: the
  # stationary arm renders exactly what the untouched arm renders, and the control arm does not.
  .quiet_output_sha256 == .restated_output_sha256 and
  .restated_output_sha256 != .moving_output_sha256 and
  .bit_identity == "quiet == restated, asserted in-run" and
  honest_metadata;

def console_benchmark_record_valid_lib:
  type == "object" and (.record | type == "string") and
  (if .record == "console_session" then session_record_valid
   elif .record == "console_hoist" then hoist_record_valid
   else false end);
