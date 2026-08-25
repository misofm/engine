# Shared definitions for the console qualification benchmark records.
#
# Four record shapes share one stream. `console_session` is a workload rendered through a real
# prepared plan; `console_hoist` is the paired-alternation comparison of the stationary-smoother
# arms; `console_meters` and `console_observation` are the #163 item 0d paired arms of the console
# observation facilities. `console_benchmark_record_valid_lib` dispatches on `.record`, so a record
# that claims one shape and carries another's keys fails rather than being validated against the
# wrong table.
def sha256: type == "string" and test("^[0-9a-f]{64}$");
def nonnegative_integer: type == "number" and floor == . and . >= 0;
def positive_integer: type == "number" and floor == . and . > 0;

# The eleven runner-supplied metadata names, as they appear in a record.
def metadata_names: ["background_load_note","candidate_commit","cpu_affinity","cpu_model","governor_or_power_mode","llvm_version","measurement_control","profile","rust_version","target_features","target_triple"];

def session_keys: ["backend","background_load_note","candidate_commit","cpu_affinity","cpu_model","descriptive_only","fixture_id","governor_or_power_mode","input_signal","issue","llvm_version","max_ns_per_block","max_us_per_block","measurement_control","min_ns_per_block","min_us_per_block","missing_metadata","observations","os","output_sha256","p50_ns_per_block","p50_us_per_block","p50_us_per_block_per_track","p95_ns_per_block","p95_us_per_block","p99_ns_per_block","p99_us_per_block","percentile_method","profile","quantum_frames","record","render_errors","render_total_forbidden_operations","round","rust_version","sample_rate_hz","schema_version","statistical_method","strip_content","synthetic_fixture","target_features","target_triple","tracks","units","workload_kind"];

def hoist_keys: ["arms","backend","background_load_note","bank_boundary","bit_identity","candidate_commit","cpu_affinity","cpu_model","descriptive_only","governor_or_power_mode","issue","llvm_version","measurement_control","missing_metadata","moving_output_sha256","moving_p50_ns","moving_p95_ns","moving_p99_ns","observations","os","paired_delta_median_ns","pairing","percentile_method","profile","quiet_output_sha256","quiet_p50_ns","quiet_p99_ns","record","restated_output_sha256","restated_p50_ns","restated_p95_ns","restated_p99_ns","round","rust_version","schema_version","statistical_method","target_features","target_triple","tracks","units","workload_kind"];

def meters_keys: ["arms","backend","background_load_note","bit_identity","candidate_commit","cpu_affinity","cpu_model","descriptive_only","governor_or_power_mode","issue","llvm_version","measurement_control","meter_frames_drained","meter_streams","meter_tap","meter_window_blocks","meters_off_output_sha256","meters_off_p50_ns","meters_off_p95_ns","meters_off_p99_ns","meters_on_output_sha256","meters_on_p50_ns","meters_on_p95_ns","meters_on_p99_ns","missing_metadata","observations","os","paired_delta_median_ns","pairing","percentile_method","profile","record","render_errors","render_total_forbidden_operations","round","rust_version","schema_version","statistical_method","target_features","target_triple","tracks","units","workload_kind"];

def observation_keys: ["absent_output_sha256","absent_p50_ns","absent_p95_ns","absent_p99_ns","armed_output_sha256","armed_p50_ns","armed_p95_ns","armed_p99_ns","armed_windows_published","arms","backend","background_load_note","bit_identity","candidate_commit","cpu_affinity","cpu_model","descriptive_only","governor_or_power_mode","issue","llvm_version","measurement_control","missing_metadata","observation_lanes","observation_taps","observation_window_blocks","observations","os","paired_arm_delta_median_ns","paired_capacity_delta_median_ns","pairing","percentile_method","profile","record","render_errors","render_total_forbidden_operations","round","rust_version","schema_version","statistical_method","target_features","target_triple","tracks","unarmed_output_sha256","unarmed_p50_ns","unarmed_p95_ns","unarmed_p99_ns","unarmed_windows_published","units","workload_kind"];

# The nine session workloads, in the emission order of `WORKLOADS`.
def session_kinds: ["nine_track_baseline","nine_track_ragged_strip","one_twenty_eight_track_stretch","sixty_four_track_builtins_only","sixty_four_track_compressor_only","sixty_four_track_console","sixty_four_track_dispatch_only","sixty_four_track_eq_only","sixty_four_track_idle"];

def console_fixture: "fixtures/session/v1/console-sixty-four-track.toml";

# Every workload names its track count, whether its model was derived in code, what its strip
# carries and what its sources feed it. A synthetic row that claimed to be a checked-in fixture,
# or a decomposition row that claimed to carry a rack it had emptied, or an idle row that claimed
# silence while rendering a tone, would each be exactly the "measuring a fiction" failure the bench
# discipline exists to catch. All four facts are therefore pinned together, per kind.
#
# The decomposition rows (#163 item 0c) are what make the differences between rows subtractions:
# every one of them is the console fixture with part of the strip removed, so
# `sixty_four_track_console - sixty_four_track_eq_only` is the compressor's share of the block. A
# row whose `strip_content` drifted from what the subject actually built would silently turn those
# subtractions into comparisons of two different sessions.
def session_kind_shape:
  if .workload_kind == "nine_track_baseline" then
    .tracks == 9 and .synthetic_fixture == false and
    .strip_content == "eq" and .input_signal == "tone" and
    .fixture_id == "fixtures/session/v1/parametric-eq-nine-track.toml"
  elif .workload_kind == "nine_track_ragged_strip" then
    .tracks == 9 and .synthetic_fixture == true and
    .strip_content == "eq+compressor" and .input_signal == "tone" and
    .fixture_id == console_fixture
  elif .workload_kind == "sixty_four_track_console" then
    .tracks == 64 and .synthetic_fixture == false and
    .strip_content == "eq+compressor" and .input_signal == "tone" and
    .fixture_id == console_fixture
  elif .workload_kind == "one_twenty_eight_track_stretch" then
    .tracks == 128 and .synthetic_fixture == true and
    .strip_content == "eq+compressor" and .input_signal == "tone" and
    .fixture_id == console_fixture
  elif .workload_kind == "sixty_four_track_eq_only" then
    .tracks == 64 and .synthetic_fixture == true and
    .strip_content == "eq" and .input_signal == "tone" and
    .fixture_id == console_fixture
  elif .workload_kind == "sixty_four_track_compressor_only" then
    .tracks == 64 and .synthetic_fixture == true and
    .strip_content == "compressor" and .input_signal == "tone" and
    .fixture_id == console_fixture
  elif .workload_kind == "sixty_four_track_builtins_only" then
    .tracks == 64 and .synthetic_fixture == true and
    .strip_content == "builtins" and .input_signal == "tone" and
    .fixture_id == console_fixture
  elif .workload_kind == "sixty_four_track_dispatch_only" then
    .tracks == 64 and .synthetic_fixture == true and
    .strip_content == "identity" and .input_signal == "tone" and
    .fixture_id == console_fixture
  elif .workload_kind == "sixty_four_track_idle" then
    # The one row whose whole meaning is its input. `eq+compressor` because the strip is the
    # unmodified console strip: the idle row measures a fully armed console rendering silence, not
    # a stripped console rendering anything.
    .tracks == 64 and .synthetic_fixture == true and
    .strip_content == "eq+compressor" and .input_signal == "silence" and
    .fixture_id == console_fixture
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

# #144 item 13 / #163 phase 0a: a record states whether its measurement was *controlled*, and the
# statement has to agree with the rest of the record.
#
# A controlled run pinned the workload to one named core and passed a load-average ceiling, an SMT
# sibling quiet check and a binary-mtime cooldown; its note begins `controlled;` and its
# `cpu_affinity` is a CPU number. An uncontrolled run is one where an operator set
# `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1` on a machine that cannot offer those controls, and its
# note has to say so *and* name the escape hatch, so nobody can read an uncontrolled number as a
# controlled one. There is no third value: the field is the distinction, and a record that invents
# a new word for it fails here rather than being quietly grouped with the controlled ones.
def admissibility:
  if .measurement_control == null then
    # Only reachable when the runner did not export the name at all, which `honest_metadata` has
    # already forced the record to declare in `missing_metadata`. The aggregate refuses it outright.
    true
  elif .measurement_control == "controlled" then
    (.cpu_affinity | type == "string" and test("^[0-9]+$")) and
    (.background_load_note | type == "string" and startswith("controlled;"))
  elif .measurement_control == "uncontrolled" then
    (.background_load_note | type == "string" and startswith("uncontrolled;") and
     test("MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1"))
  else false end;

def ordered_percentiles($fields):
  ($fields | all(nonnegative_integer)) and
  ($fields | . == (. | sort));

def session_statistical_method:
  "nearest-rank percentiles over per-block nanoseconds; one warmup pass and two measured rounds; descriptive only; no threshold";
def hoist_statistical_method:
  "three arms alternated per observation; nearest-rank percentiles over per-block nanoseconds; paired delta is moving minus restated per observation; descriptive only; no threshold";
def meters_statistical_method:
  "two arms alternated per observation; nearest-rank percentiles over per-block nanoseconds; paired delta is meters_on minus meters_off per observation; descriptive only; no threshold";
def observation_statistical_method:
  "three arms alternated per observation; nearest-rank percentiles over per-block nanoseconds; capacity delta is unarmed minus absent and arm delta is armed minus unarmed, per observation; descriptive only; no threshold";

# Facts every console record states the same way, whatever its shape.
def common_shape:
  .schema_version == 1 and .issue == 149 and
  (.os | type == "string" and length > 0) and
  (.backend | type == "string" and length > 0) and
  (.missing_metadata | type == "array" and all(.[]; type == "string")) and
  (.round == 1 or .round == 2) and
  .observations == 1000 and .percentile_method == "nearest_rank" and
  .descriptive_only == true and
  honest_metadata and admissibility;

def session_record_valid:
  (keys | sort) == session_keys and
  .record == "console_session" and common_shape and
  # The method is pinned verbatim. A record that changed how it was measured but kept the old
  # sentence would be the most expensive kind of quiet drift, so the sentence is part of the shape.
  .statistical_method == session_statistical_method and
  .sample_rate_hz == 48000 and .quantum_frames == 128 and
  .units == "us_per_block" and
  session_kind_shape and
  ordered_percentiles([.min_ns_per_block,.p50_ns_per_block,.p95_ns_per_block,.p99_ns_per_block,.max_ns_per_block]) and
  ([.min_us_per_block,.p50_us_per_block,.p95_us_per_block,.p99_us_per_block,.max_us_per_block,.p50_us_per_block_per_track] | all(type == "number" and . > 0)) and
  (.output_sha256 | sha256) and
  .render_errors == 0 and .render_total_forbidden_operations == 0;

def hoist_record_valid:
  (keys | sort) == hoist_keys and
  .record == "console_hoist" and common_shape and
  .statistical_method == hoist_statistical_method and
  .units == "ns_per_block" and
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
  .bit_identity == "quiet == restated, asserted in-run";

# The meters arm (#163 item 0d). Two plans of one workload differing in one thing: whether the
# session was prepared with a meter stream per track at the post-matrix tap, the shape a host
# prepares for a real console.
def meters_record_valid:
  (keys | sort) == meters_keys and
  .record == "console_meters" and common_shape and
  .statistical_method == meters_statistical_method and
  .units == "ns_per_block" and
  .pairing == "alternating_per_observation" and
  .arms == ["meters_off","meters_on"] and
  .workload_kind == "sixty_four_track_console" and .tracks == 64 and
  # One stream per track, at the tap a console meters by default. A record that metered fewer
  # streams than it had tracks measured a shape no host prepares.
  .meter_streams == .tracks and .meter_tap == "post_matrix" and
  (.meter_window_blocks | positive_integer) and
  # The arm has to have actually metered. A meters-on arm that published nothing would report a
  # delta of nothing and read as a wonderful result.
  (.meter_frames_drained | positive_integer) and
  ([.meters_off_p50_ns,.meters_off_p95_ns,.meters_off_p99_ns,.meters_on_p50_ns,.meters_on_p95_ns,.meters_on_p99_ns] | all(positive_integer)) and
  (.meters_off_p50_ns <= .meters_off_p95_ns and .meters_off_p95_ns <= .meters_off_p99_ns) and
  (.meters_on_p50_ns <= .meters_on_p95_ns and .meters_on_p95_ns <= .meters_on_p99_ns) and
  (.paired_delta_median_ns | type == "number" and floor == .) and
  ([.meters_off_output_sha256,.meters_on_output_sha256] | all(sha256)) and
  # The class-A statement: metering is observation. Attaching a meter stream may not change a
  # rendered bit, and this is where that is stated rather than assumed.
  .meters_off_output_sha256 == .meters_on_output_sha256 and
  .bit_identity == "meters_off == meters_on, asserted in-run" and
  .render_errors == 0 and .render_total_forbidden_operations == 0;

# The observation arm (#163 item 0d), which is the issue #143 two-level zero measured rather than
# argued: `absent` has no lane at all, `unarmed` has a lane with nothing armed, `armed` has every
# declared tap armed. All three carry the live-console control channel, so the deltas are the
# observation lane and the arming, never the control queue.
def observation_record_valid:
  (keys | sort) == observation_keys and
  .record == "console_observation" and common_shape and
  .statistical_method == observation_statistical_method and
  .units == "ns_per_block" and
  .pairing == "alternating_per_observation" and
  .arms == ["absent","unarmed","armed"] and
  .workload_kind == "sixty_four_track_console" and .tracks == 64 and
  (.observation_lanes | positive_integer) and
  (.observation_taps | positive_integer) and
  (.observation_window_blocks | positive_integer) and
  # The two halves of the honesty gate, carried in the record. An unarmed lane that published a
  # window was not unarmed; an armed lane that published none was measuring the unarmed cost twice
  # and reporting the difference as noise.
  .unarmed_windows_published == 0 and
  (.armed_windows_published | positive_integer) and
  ([.absent_p50_ns,.absent_p95_ns,.absent_p99_ns,.unarmed_p50_ns,.unarmed_p95_ns,.unarmed_p99_ns,.armed_p50_ns,.armed_p95_ns,.armed_p99_ns] | all(positive_integer)) and
  (.absent_p50_ns <= .absent_p95_ns and .absent_p95_ns <= .absent_p99_ns) and
  (.unarmed_p50_ns <= .unarmed_p95_ns and .unarmed_p95_ns <= .unarmed_p99_ns) and
  (.armed_p50_ns <= .armed_p95_ns and .armed_p95_ns <= .armed_p99_ns) and
  ([.paired_capacity_delta_median_ns,.paired_arm_delta_median_ns] | all(type == "number" and floor == .)) and
  ([.absent_output_sha256,.unarmed_output_sha256,.armed_output_sha256] | all(sha256)) and
  # Observation is observation: neither attaching a lane nor arming a tap may change a rendered
  # bit.
  .absent_output_sha256 == .unarmed_output_sha256 and
  .unarmed_output_sha256 == .armed_output_sha256 and
  .bit_identity == "absent == unarmed == armed, asserted in-run" and
  .render_errors == 0 and .render_total_forbidden_operations == 0;

def console_benchmark_record_valid_lib:
  type == "object" and (.record | type == "string") and
  (if .record == "console_session" then session_record_valid
   elif .record == "console_hoist" then hoist_record_valid
   elif .record == "console_meters" then meters_record_valid
   elif .record == "console_observation" then observation_record_valid
   else false end);
