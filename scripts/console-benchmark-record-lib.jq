# Shared definitions for the console qualification benchmark records.
#
# Seven record shapes share one stream. `console_session` is a workload rendered through a real
# prepared plan; `console_hoist` is the paired-alternation comparison of the stationary-smoother
# arms; `console_meters` and `console_observation` are the #163 item 0d paired arms of the console
# observation facilities; `console_placement` is the #175 chain-shape row-pair; and
# `console_automation` is the automation-active row -- one Point span per block on one track,
# which is the only place in this stream a compressor's ramping body is executed at all; and
# `console_mono` is the mono-collapse row-pair, whose two arms are one session in this tree and
# whose digest equality is the gate the collapse will be constrained by when it lands.
# `console_benchmark_record_valid_lib` dispatches on `.record`, so a record that claims one shape
# and carries another's keys fails rather than being validated against the wrong table.
def sha256: type == "string" and test("^[0-9a-f]{64}$");
def nonnegative_integer: type == "number" and floor == . and . >= 0;
def positive_integer: type == "number" and floor == . and . > 0;

# The eleven runner-supplied metadata names, as they appear in a record.
def metadata_names: ["background_load_note","candidate_commit","cpu_affinity","cpu_model","governor_or_power_mode","llvm_version","measurement_control","profile","rust_version","target_features","target_triple"];

def session_keys: ["backend","background_load_note","candidate_commit","cpu_affinity","cpu_model","descriptive_only","fixture_id","governor_or_power_mode","input_signal","issue","llvm_version","max_ns_per_block","max_us_per_block","measurement_control","min_ns_per_block","min_us_per_block","missing_metadata","observations","os","output_sha256","p50_ns_per_block","p50_us_per_block","p50_us_per_block_per_track","p95_ns_per_block","p95_us_per_block","p99_ns_per_block","p99_us_per_block","percentile_method","profile","quantum_frames","record","render_errors","render_total_forbidden_operations","round","rust_version","sample_rate_hz","schema_version","statistical_method","strip_content","strip_layout","synthetic_fixture","target_features","target_triple","tracks","units","workload_kind"];

def hoist_keys: ["arms","backend","background_load_note","bank_boundary","bit_identity","candidate_commit","cpu_affinity","cpu_model","descriptive_only","governor_or_power_mode","issue","llvm_version","measurement_control","missing_metadata","moving_output_sha256","moving_p50_ns","moving_p95_ns","moving_p99_ns","observations","os","paired_delta_median_ns","pairing","percentile_method","profile","quiet_output_sha256","quiet_p50_ns","quiet_p99_ns","record","restated_output_sha256","restated_p50_ns","restated_p95_ns","restated_p99_ns","round","rust_version","schema_version","statistical_method","target_features","target_triple","tracks","units","workload_kind"];

def meters_keys: ["arms","backend","background_load_note","bit_identity","candidate_commit","cpu_affinity","cpu_model","descriptive_only","governor_or_power_mode","issue","llvm_version","measurement_control","meter_frames_drained","meter_streams","meter_tap","meter_window_blocks","meters_off_output_sha256","meters_off_p50_ns","meters_off_p95_ns","meters_off_p99_ns","meters_on_output_sha256","meters_on_p50_ns","meters_on_p95_ns","meters_on_p99_ns","missing_metadata","observations","os","paired_delta_median_ns","pairing","percentile_method","profile","record","render_errors","render_total_forbidden_operations","round","rust_version","schema_version","statistical_method","target_features","target_triple","tracks","units","workload_kind"];

def placement_keys: ["arms","backend","background_load_note","bit_identity","candidate_commit","cpu_affinity","cpu_model","descriptive_only","governor_or_power_mode","issue","llvm_version","measurement_control","merged_chain_layout","merged_chain_output_sha256","merged_chain_p50_ns","merged_chain_p95_ns","merged_chain_p99_ns","merged_chain_transposes_per_block","missing_metadata","observations","os","paired_delta_median_ns","paired_delta_median_ns_per_track","pairing","percentile_method","profile","record","render_errors","render_total_forbidden_operations","round","rust_version","schema_version","split_chains_layout","split_chains_output_sha256","split_chains_p50_ns","split_chains_p95_ns","split_chains_p99_ns","split_chains_transposes_per_block","statistical_method","target_features","target_triple","tracks","units","workload_kind"];

# The mono row-pair (the mono-collapse gate). Two arms of one session today; see
# `mono_record_valid`.
def mono_keys: ["arm_difference","arms","backend","background_load_note","bit_identity","candidate_commit","collapse_eligible_output_sha256","collapse_eligible_p50_ns","collapse_eligible_p95_ns","collapse_eligible_p99_ns","collapse_eligible_transposes_per_block","collapse_forced_off_output_sha256","collapse_forced_off_p50_ns","collapse_forced_off_p95_ns","collapse_forced_off_p99_ns","collapse_forced_off_transposes_per_block","cpu_affinity","cpu_model","descriptive_only","fixture_id","governor_or_power_mode","issue","lanes","llvm_version","measurement_control","missing_metadata","mono_source_tracks","observations","os","paired_delta_median_ns","paired_delta_median_ns_per_track","pairing","percentile_method","profile","record","render_errors","render_total_forbidden_operations","round","rust_version","schema_version","statistical_method","symmetric_lanes","target_features","target_triple","tracks","units","workload_kind"];

def observation_keys: ["absent_output_sha256","absent_p50_ns","absent_p95_ns","absent_p99_ns","armed_output_sha256","armed_p50_ns","armed_p95_ns","armed_p99_ns","armed_windows_published","arms","backend","background_load_note","bit_identity","candidate_commit","cpu_affinity","cpu_model","descriptive_only","governor_or_power_mode","issue","llvm_version","measurement_control","missing_metadata","observation_lanes","observation_taps","observation_window_blocks","observations","os","paired_arm_delta_median_ns","paired_capacity_delta_median_ns","pairing","percentile_method","profile","record","render_errors","render_total_forbidden_operations","round","rust_version","schema_version","statistical_method","target_features","target_triple","tracks","unarmed_output_sha256","unarmed_p50_ns","unarmed_p95_ns","unarmed_p99_ns","unarmed_windows_published","units","workload_kind"];

def automation_keys: ["arms","automated_channel","automated_effect","automated_effect_id","automated_output_sha256","automated_p50_ns","automated_p95_ns","automated_p99_ns","automated_parameter","automated_parameter_index","automated_pushes_accepted","automated_track_id","automation_spans_per_block","backend","background_load_note","bit_identity","candidate_commit","cpu_affinity","cpu_model","descriptive_only","fixture_id","governor_or_power_mode","input_signal","issue","llvm_version","measurement_control","missing_metadata","observations","os","paired_control_delta_median_ns","paired_ramp_delta_median_ns","paired_ramp_delta_median_ns_per_track","pairing","percentile_method","profile","quantum_frames","quiet_output_sha256","quiet_p50_ns","quiet_p95_ns","quiet_p99_ns","record","render_errors","render_total_forbidden_operations","restated_output_sha256","restated_p50_ns","restated_p95_ns","restated_p99_ns","restated_pushes_accepted","round","rust_version","sample_rate_hz","schema_version","smoothing_samples","statistical_method","strip_content","strip_layout","synthetic_fixture","target_features","target_triple","tracks","units","workload_kind"];

# ---------------------------------------------------------------------------------------------
# Issue #184: floor accounting. Additive, and additive means additive.
# ---------------------------------------------------------------------------------------------
#
# A session record either carries the whole floor group or none of it. Every record sealed under
# `artifacts/` predates the group and validates on `session_keys` exactly as before; a record from
# a runner that measured the pinned core's clock validates on `session_floor_keys`, which is the
# same set plus these eleven. There is no third shape: dropping one column out of the group leaves
# a record that matches neither list, which is what makes the structural mutation sweep bite on
# every one of them individually.
def floor_keys: ["core_clock_hz","core_clock_source","cycles_per_block_p50","cycles_per_lane_sample","floor_basis","floor_control_row","floor_cycles_per_lane_sample","isolated_cycles_per_lane_sample","isolated_percent_of_floor","lane_samples_per_block","percent_of_floor"];
def session_floor_keys: (session_keys + floor_keys) | sort;

# The op inventories, restated. `tools/bench/src/floor.rs` is the authority and
# `docs/rulings/effect-floor-accounting.md` is the derivation; this is the independent copy that
# makes a subject which quietly re-tuned a floor fail here rather than publish. The same discipline
# `session_kind_shape` applies to a workload's track count is applied to its floor.
def lane_ops_per_cycle: 8 * 3.7;
def builtins_lane_ops: 69;
# The rack-free rows do not share a floor. A builtin section prepared as the exact identity is
# elided, not executed, so the identity row's arithmetic is the 69 with both 24-op SVF sections
# replaced by the single `add(+0.0)` a run of identity sections composes to:
# 7 sanitise + 1 identity add + 4 boundary + 2 fader + 4 pan + 3 route + 1 reduction.
def builtins_identity_lane_ops: 22;
# The overhead floor: what a lane-sample pays with *no builtins prepared at all*. The route's
# `mix2x2` is one `mul` and one deliberately unfused `fma` per channel (3), and the output node's
# 64-input reduction amortises to 1 per track. Both are already lines of the two builtins
# inventories above, which is what makes `gain_pan_only - plumbing_only` an exact 18 rather than an
# estimate. Job 3's route fold moved neither: folding relocates the same `mix2x2_block` and the
# same 63 adds into the cohort chain's epilogue, in the order `route_fold` proves at bind.
def plumbing_lane_ops: 3 + 1;
def eq_lane_ops: 51;
def compressor_lane_ops: 94;
def limiter_lane_ops: 138;
# Nine tracks is one full eight-lane bank plus a one-track tail, and the tail costs a whole vector
# operation per lane-sample: `(8 + 8) / 9` of the full-bank floor.
def ragged_nine_track_width_factor: 16 / 9;
def floor_document: "docs/rulings/effect-floor-accounting.md: ";

# Per workload kind: required arithmetic per lane-sample, width factor, the row subtracted to
# isolate this row's subject, and the basis string. A null inventory is a row whose fixture was
# never inventoried, and it must say `not_derived` rather than guess.
def floor_pins:
  (builtins_lane_ops) as $b |
  (builtins_identity_lane_ops) as $bi |
  (builtins_lane_ops + eq_lane_ops) as $be |
  (builtins_lane_ops + compressor_lane_ops) as $bc |
  (builtins_lane_ops + eq_lane_ops + compressor_lane_ops) as $bec |
  (builtins_lane_ops + eq_lane_ops + compressor_lane_ops + limiter_lane_ops) as $becl |
  {
    "nine_track_baseline":
      [null, 1, "none", "not_derived"],
    "nine_track_ragged_strip":
      [$becl, ragged_nine_track_width_factor, "none",
       floor_document + "builtins+eq+compressor+limiter, ragged"],
    "sixty_four_track_console":
      [$becl, 1, "sixty_four_track_eq_comp_simd1",
       floor_document + "builtins+eq+compressor+limiter"],
    "one_twenty_eight_track_stretch":
      [$becl, 1, "none", floor_document + "builtins+eq+compressor+limiter"],
    "sixty_four_track_eq_only":
      [$be, 1, "sixty_four_track_builtins_only", floor_document + "builtins+eq"],
    "sixty_four_track_compressor_only":
      [$bc, 1, "sixty_four_track_builtins_only", floor_document + "builtins+compressor"],
    "sixty_four_track_console_legacy":
      [$bec, 1, "sixty_four_track_builtins_only", floor_document + "builtins+eq+compressor"],
    "sixty_four_track_eq_comp_simd1":
      [$bec, 1, "sixty_four_track_builtins_only", floor_document + "builtins+eq+compressor"],
    "sixty_four_track_idle":
      [$b, 1, "none", floor_document + "builtins, silent"],
    "sixty_four_track_builtins_only":
      [$b, 1, "none", floor_document + "builtins"],
    "sixty_four_track_dispatch_only":
      [$bi, 1, "none", floor_document + "builtins, identity"],
    # The other row that composes the identity inventory. One basis string for both identity rows
    # is deliberate -- a real fader and pan cost what an identity fader and pan cost, because
    # neither kernel has an identity arm, and that claim is what the shared inventory states.
    #
    # It names **no control**, and the plumbing row below is deliberately not one. The inventories
    # subtract (22 - 4 = 18, the scaffolding) but the rows do not: this row binds eight bank chains
    # so job 3's route fold fires, the plumbing row binds none so it pays 64 dispatched route ops
    # and an unfolded reduction, and the difference between them is the fold's saving as well as
    # the plumbing's arithmetic. The ruling records the measured evidence.
    "sixty_four_track_gain_pan_only":
      [$bi, 1, "none", floor_document + "builtins, identity"],
    # The floor of the whole table. Nothing in this stream can be costed below it: a row that
    # renders sixty-four tracks into one master pays a route matrix and its share of the master
    # reduction whatever else it does or does not prepare. Its own `percent_of_floor` is the
    # interesting number -- unfolded plumbing against four lane-ops -- and it is nobody's control.
    "sixty_four_track_plumbing_only":
      [plumbing_lane_ops, 1, "none", floor_document + "plumbing"],
    # The three mono rows carry the whole intended strip, so they carry its inventory. Their
    # fixture differs from the standing one in per-channel values only -- one source channel
    # instead of two, and the left channel's designed words on both sides -- and a floor is an
    # inventory of operations, not of operands.
    "sixty_four_track_console_mono":
      [$becl, 1, "none", floor_document + "builtins+eq+compressor+limiter"],
    "sixty_four_track_console_mono_dual":
      [$becl, 1, "none", floor_document + "builtins+eq+compressor+limiter"],
    "sixty_four_track_console_half_mono":
      [$becl, 1, "none", floor_document + "builtins+eq+compressor+limiter"]
  };

# Absolute agreement to the precision the subject prints (three decimals), with a little slack for
# the order the two sides multiply in.
def near($a; $b; $tolerance):
  ($a | type == "number") and ($b | type == "number") and
  ((($a - $b) | if . < 0 then - . else . end) <= $tolerance);

# Every derived column recomputed from the columns it was derived from. A miscomputed cycle count,
# a floor that does not match its inventory, or a percentage that does not match its own floor and
# its own measurement all fail here; a column that is merely *present* proves nothing.
def floor_shape:
  (floor_pins[.workload_kind]) as $pin |
  ($pin != null) and
  (.lane_samples_per_block == (.tracks * .quantum_frames * 2)) and
  (.core_clock_hz | type == "number" and . > 100000000 and . < 100000000000) and
  (.core_clock_source | type == "string" and length > 0) and
  near(.cycles_per_block_p50; .p50_ns_per_block * .core_clock_hz / 1000000000; 0.002) and
  near(.cycles_per_lane_sample; .cycles_per_block_p50 / .lane_samples_per_block; 0.002) and
  (.floor_basis == $pin[3]) and
  (.floor_control_row == $pin[2]) and
  (if $pin[0] == null then
     .floor_cycles_per_lane_sample == null and .percent_of_floor == null
   else
     near(.floor_cycles_per_lane_sample; $pin[0] * $pin[1] / lane_ops_per_cycle; 0.002) and
     near(.percent_of_floor;
          100 * .floor_cycles_per_lane_sample / .cycles_per_lane_sample; 0.02)
   end) and
  # The isolate is a subtraction between two rows, so only the aggregate can recompute it. What a
  # single record can say is whether it claims one at all, and that has to agree with the control
  # row it names.
  (if .floor_control_row == "none" then
     .isolated_cycles_per_lane_sample == null and .isolated_percent_of_floor == null
   else
     (.isolated_cycles_per_lane_sample | type == "number" and . > 0) and
     (.isolated_percent_of_floor | type == "number" and . > 0)
   end);


# The sixteen session workloads, sorted (`WORKLOADS` itself is append-only and in emission order).
def session_kinds: ["nine_track_baseline","nine_track_ragged_strip","one_twenty_eight_track_stretch","sixty_four_track_builtins_only","sixty_four_track_compressor_only","sixty_four_track_console","sixty_four_track_console_half_mono","sixty_four_track_console_legacy","sixty_four_track_console_mono","sixty_four_track_console_mono_dual","sixty_four_track_dispatch_only","sixty_four_track_eq_comp_simd1","sixty_four_track_eq_only","sixty_four_track_gain_pan_only","sixty_four_track_idle","sixty_four_track_plumbing_only"];

# The standing qualification fixture (#175): the intended production layout, EQ and compressor as
# one two-slot chain on `simd1` and a true-peak limiter on `simd2`.
def console_fixture: "fixtures/session/v1/console-sixty-four-track-intended.toml";
# The retired fixture, rendered by exactly one row for exactly one transition record.
def legacy_console_fixture: "fixtures/session/v1/console-sixty-four-track.toml";
# The mono qualification fixture: the standing strip with its source mapping and every upstream
# per-channel parameter symmetrised, so every track satisfies the channel-symmetry witness'
# structural terms. Its fader and pan asymmetry and its limiters' `maximum` link are kept, because
# they are what document the seam.
def mono_console_fixture: "fixtures/session/v1/console-sixty-four-track-mono.toml";

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
    .strip_content == "eq" and .strip_layout == "simd1:eq" and .input_signal == "tone" and
    .fixture_id == "fixtures/session/v1/parametric-eq-nine-track.toml"
  elif .workload_kind == "nine_track_ragged_strip" then
    .tracks == 9 and .synthetic_fixture == true and
    .strip_content == "eq+compressor+limiter" and
    .strip_layout == "simd1:eq+compressor,simd2:limiter" and .input_signal == "tone" and
    .fixture_id == console_fixture
  elif .workload_kind == "sixty_four_track_console" then
    .tracks == 64 and .synthetic_fixture == false and
    .strip_content == "eq+compressor+limiter" and
    .strip_layout == "simd1:eq+compressor,simd2:limiter" and .input_signal == "tone" and
    .fixture_id == console_fixture
  elif .workload_kind == "one_twenty_eight_track_stretch" then
    .tracks == 128 and .synthetic_fixture == true and
    .strip_content == "eq+compressor+limiter" and
    .strip_layout == "simd1:eq+compressor,simd2:limiter" and .input_signal == "tone" and
    .fixture_id == console_fixture
  # The transition row (#175). The one row still rendered from the retired fixture, and the only
  # row in the stream whose `dynamic` rack carries anything. It exists so the standing authority's
  # first record and the retired authority's last one are taken on one host in one run; pinning
  # its fixture separately is what stops it quietly becoming a second copy of the standing row.
  elif .workload_kind == "sixty_four_track_console_legacy" then
    .tracks == 64 and .synthetic_fixture == false and
    .strip_content == "eq+compressor" and
    .strip_layout == "simd1:eq,dynamic:compressor" and .input_signal == "tone" and
    .fixture_id == legacy_console_fixture
  # The chain-shape row: the standing fixture's two-slot chain carrying the retired fixture's
  # arithmetic. Identical `strip_content` to the row above and a different `strip_layout`, which is
  # the entire reason `strip_layout` is a field: these two rows are otherwise indistinguishable in
  # a record, and the number that separates them is attributed to chain shape alone.
  elif .workload_kind == "sixty_four_track_eq_comp_simd1" then
    .tracks == 64 and .synthetic_fixture == true and
    .strip_content == "eq+compressor" and
    .strip_layout == "simd1:eq+compressor" and .input_signal == "tone" and
    .fixture_id == console_fixture
  elif .workload_kind == "sixty_four_track_eq_only" then
    .tracks == 64 and .synthetic_fixture == true and
    .strip_content == "eq" and .strip_layout == "simd1:eq" and .input_signal == "tone" and
    .fixture_id == console_fixture
  elif .workload_kind == "sixty_four_track_compressor_only" then
    .tracks == 64 and .synthetic_fixture == true and
    .strip_content == "compressor" and .strip_layout == "simd1:compressor" and
    .input_signal == "tone" and .fixture_id == console_fixture
  elif .workload_kind == "sixty_four_track_builtins_only" then
    .tracks == 64 and .synthetic_fixture == true and
    .strip_content == "builtins" and .strip_layout == "builtins" and .input_signal == "tone" and
    .fixture_id == console_fixture
  elif .workload_kind == "sixty_four_track_dispatch_only" then
    .tracks == 64 and .synthetic_fixture == true and
    .strip_content == "identity" and .strip_layout == "builtins" and .input_signal == "tone" and
    .fixture_id == console_fixture
  # The identity row's controlled partner: identical strip edit but for one field, the fixture's
  # own fader and pan values kept. `strip_content` is what separates the two records, and the pair
  # is only a measurement of "an identity fader costs what a real one costs" while both rows say
  # honestly which they carried.
  elif .workload_kind == "sixty_four_track_gain_pan_only" then
    .tracks == 64 and .synthetic_fixture == true and
    .strip_content == "gain+pan" and .strip_layout == "builtins" and .input_signal == "tone" and
    .fixture_id == console_fixture
  # The overhead floor row. `plumbing` is a third layout word, not an empty `builtins` one: the
  # difference between this row and the `builtins` rows -- that no builtin binding exists at all --
  # is exactly what the row measures, so a record that called it `builtins` would be naming the
  # thing it is defined by not having.
  elif .workload_kind == "sixty_four_track_plumbing_only" then
    .tracks == 64 and .synthetic_fixture == true and
    .strip_content == "plumbing" and .strip_layout == "plumbing" and .input_signal == "tone" and
    .fixture_id == console_fixture
  # The mono row-pair, as session rows. Both render the mono fixture exactly as it is checked in,
  # so both are `synthetic_fixture == false`: they are two rows of one session, which is the
  # property their digest equality will rest on once the collapse exists.
  elif .workload_kind == "sixty_four_track_console_mono"
       or .workload_kind == "sixty_four_track_console_mono_dual" then
    .tracks == 64 and .synthetic_fixture == false and
    .strip_content == "eq+compressor+limiter" and
    .strip_layout == "simd1:eq+compressor,simd2:limiter" and .input_signal == "tone" and
    .fixture_id == mono_console_fixture
  # The mixed-cohort row, derived in code from the mono fixture by putting the standing fixture's
  # stereo source mapping back on the odd tracks.
  elif .workload_kind == "sixty_four_track_console_half_mono" then
    .tracks == 64 and .synthetic_fixture == true and
    .strip_content == "eq+compressor+limiter" and
    .strip_layout == "simd1:eq+compressor,simd2:limiter" and .input_signal == "tone" and
    .fixture_id == mono_console_fixture
  elif .workload_kind == "sixty_four_track_idle" then
    # The one row whose whole meaning is its input. The strip is the unmodified standing console
    # strip: the idle row measures a fully armed console rendering silence, not a stripped console
    # rendering anything.
    .tracks == 64 and .synthetic_fixture == true and
    .strip_content == "eq+compressor+limiter" and
    .strip_layout == "simd1:eq+compressor,simd2:limiter" and .input_signal == "silence" and
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
def placement_statistical_method:
  "two arms alternated per observation; nearest-rank percentiles over per-block nanoseconds; paired delta is merged_chain minus split_chains per observation; descriptive only; no threshold";
def automation_statistical_method:
  "three arms alternated per observation; nearest-rank percentiles over per-block nanoseconds; ramp delta is automated minus restated and control delta is restated minus quiet, per observation; descriptive only; no threshold";
def mono_statistical_method:
  "two arms alternated per observation; nearest-rank percentiles over per-block nanoseconds; paired delta is collapse_forced_off minus collapse_eligible per observation; descriptive only; no threshold";
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
  ((keys | sort) == session_keys or (keys | sort) == session_floor_keys) and
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
  .render_errors == 0 and .render_total_forbidden_operations == 0 and
  (if (keys | sort) == session_floor_keys then floor_shape else true end);

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

# The #175 chain-shape row-pair. Two arms that carry *identical arithmetic* and differ only in
# whether the EQ and the compressor are one two-slot chain on `simd1` or two one-slot chains
# across `simd1` and `dynamic`.
#
# Two claims are pinned here that no other record in this stream makes.
#
# The first is bit identity across a *placement* change, which is AGENTS.md's rule and #166's
# result: "Banking regroups lanes; it never changes per-lane arithmetic, so a placement change must
# not move a rendered bit." If these two digests ever differ the delta is not a chain-shape
# measurement at all, so the record is refused rather than published with a caveat.
#
# The second is the transpose count of each arm, which is what makes the measured delta
# *explicable* rather than merely reported. The G5 shape gate says one planar/AoSoA round-trip per
# bank chain per block, and #175 opened on the hypothesis that the merged layout would therefore
# pay one round-trip per cohort where the split layout paid two. It does not: `graph`'s
# `runtime::bank_chain` materialises every prepared bank as a *single-slot* chain, so the cohort
# planner's grouping never reaches the counter and both arms transpose the same number of times.
# The record carries both counts so that finding is a datum in the stream and not a claim in a
# README -- and so that the day the graph layer takes the saving, this validator's equality goes
# red and says so.
def placement_record_valid:
  (keys | sort) == placement_keys and
  .record == "console_placement" and common_shape and
  .statistical_method == placement_statistical_method and
  .units == "ns_per_block" and
  .pairing == "alternating_per_observation" and
  .arms == ["split_chains","merged_chain"] and
  .workload_kind == "sixty_four_track_placement" and .tracks == 64 and
  # The two arms are named by their layouts, and the layouts are the point of the comparison.
  .split_chains_layout == "simd1:eq,dynamic:compressor" and
  .merged_chain_layout == "simd1:eq+compressor" and
  ([.split_chains_p50_ns,.split_chains_p95_ns,.split_chains_p99_ns,.merged_chain_p50_ns,.merged_chain_p95_ns,.merged_chain_p99_ns] | all(positive_integer)) and
  (.split_chains_p50_ns <= .split_chains_p95_ns and .split_chains_p95_ns <= .split_chains_p99_ns) and
  (.merged_chain_p50_ns <= .merged_chain_p95_ns and .merged_chain_p95_ns <= .merged_chain_p99_ns) and
  (.paired_delta_median_ns | type == "number" and floor == .) and
  (.paired_delta_median_ns_per_track | type == "number") and
  ([.split_chains_transposes_per_block,.merged_chain_transposes_per_block] | all(positive_integer)) and
  ([.split_chains_output_sha256,.merged_chain_output_sha256] | all(sha256)) and
  .split_chains_output_sha256 == .merged_chain_output_sha256 and
  .bit_identity == "split_chains == merged_chain, asserted in-run" and
  .render_errors == 0 and .render_total_forbidden_operations == 0;

# The automation-active row (one Point span per block, on one track).
#
# Its subject is the `sixty_four_track_compressor_only` decomposition row, so it is pinned against
# exactly the six facts that row is pinned against -- an arm repointed at another workload, or at
# the same workload with a different strip edit, fails here rather than reporting a ramping
# surcharge for a session nobody named. What the row adds on top of those facts is *what rides the
# control channel*, and every part of that is pinned too: which track, which slot, which effect,
# which parameter, which channel, how many spans per block, and how long the window the surcharge
# is the cost of.
#
# The two digest rules are the row's whole claim. `quiet == restated` is the class-A statement --
# restating a parameter at the value it already holds must move no rendered bit, which is what
# makes the ramp delta the cost of the *window* rather than of the queue drain. `restated !=
# automated` is the honesty half: an arm that renders the restated arm's bits opened no window and
# measured the cost of nothing, which is precisely the failure the EQ hoist arm recorded when it
# tried a one-ULP step.
def automation_record_valid:
  (keys | sort) == automation_keys and
  .record == "console_automation" and common_shape and
  .statistical_method == automation_statistical_method and
  .workload_kind == "sixty_four_track_compressor_automation" and
  # The subject row's six pinned facts, verbatim from `session_kind_shape`.
  .tracks == 64 and .synthetic_fixture == true and
  .strip_content == "compressor" and .strip_layout == "simd1:compressor" and
  .input_signal == "tone" and .fixture_id == console_fixture and
  .sample_rate_hz == 48000 and .quantum_frames == 128 and
  .pairing == "alternating_per_observation" and
  .arms == ["quiet","restated","automated"] and
  # What rides the control channel.
  .automated_track_id == "ch00" and .automated_effect_id == "comp" and
  .automated_effect == "miso.compressor" and
  .automated_parameter == "threshold" and .automated_parameter_index == 0 and
  .automated_channel == "left" and
  .automation_spans_per_block == 1 and .smoothing_samples == 64 and
  # Every block of both pushing arms was accepted by the bounded queue. A refused push would be
  # the cost of automation that never arrived, reported as though it had.
  .restated_pushes_accepted == .observations and
  .automated_pushes_accepted == .observations and
  .units == "ns_per_block" and
  ([.quiet_p50_ns,.quiet_p95_ns,.quiet_p99_ns,.restated_p50_ns,.restated_p95_ns,.restated_p99_ns,.automated_p50_ns,.automated_p95_ns,.automated_p99_ns] | all(positive_integer)) and
  ordered_percentiles([.quiet_p50_ns,.quiet_p95_ns,.quiet_p99_ns]) and
  ordered_percentiles([.restated_p50_ns,.restated_p95_ns,.restated_p99_ns]) and
  ordered_percentiles([.automated_p50_ns,.automated_p95_ns,.automated_p99_ns]) and
  (.paired_ramp_delta_median_ns | type == "number" and floor == .) and
  (.paired_control_delta_median_ns | type == "number" and floor == .) and
  (.paired_ramp_delta_median_ns_per_track | type == "number") and
  ([.quiet_output_sha256,.restated_output_sha256,.automated_output_sha256] | all(sha256)) and
  .quiet_output_sha256 == .restated_output_sha256 and
  .restated_output_sha256 != .automated_output_sha256 and
  .bit_identity == "quiet == restated, asserted in-run" and
  .render_errors == 0 and .render_total_forbidden_operations == 0;

# The mono row-pair: the gate the mono collapse is measured and constrained by.
#
# Three claims are pinned here, and the first is now load-bearing rather than trivial.
#
# The first is the class-A statement: a collapse-eligible session and the same session with the
# collapse forced off must render the same bits. It was written one milestone before the mechanism
# it checks -- a gate authored by the same change it is supposed to check is not a gate -- and
# since mono-collapse M2 the eligible arm takes the collapse on all eight cohorts and the forced-off
# arm renders the same fixture dual, so the equality is a statement about the whole mechanism.
#
# The second is the premise that statement is *about*. `mono_source_tracks` is the count of tracks
# whose structural (`SOURCE`) witness holds, and `symmetric_lanes`/`lanes` is the prepared plan's
# own census of the remaining terms. Both must show every track eligible, because a pair measured
# on a session with no mono-source track would be the standing console measured twice under
# another name -- and it would pass the digest equality perfectly.
#
# The third is `arm_difference`, pinned verbatim, and it moved when the mechanism arrived: it used
# to say there was no collapse in the tree, because a reader who found a two-arm paired record with
# a near-zero delta and no such field would reasonably have read it as "the collapse saves
# nothing". It now names the one bind-time switch that separates the arms. Changing that claim
# means editing this pin, `MONO_ARM_DIFFERENCE` in the bench, and the mutation case that guards it.
def mono_record_valid:
  (keys | sort) == mono_keys and
  .record == "console_mono" and common_shape and
  .statistical_method == mono_statistical_method and
  .units == "ns_per_block" and
  .pairing == "alternating_per_observation" and
  .arms == ["collapse_eligible","collapse_forced_off"] and
  .workload_kind == "sixty_four_track_mono_pair" and .tracks == 64 and
  .fixture_id == mono_console_fixture and
  ([.collapse_eligible_p50_ns,.collapse_eligible_p95_ns,.collapse_eligible_p99_ns,.collapse_forced_off_p50_ns,.collapse_forced_off_p95_ns,.collapse_forced_off_p99_ns] | all(positive_integer)) and
  ordered_percentiles([.collapse_eligible_p50_ns,.collapse_eligible_p95_ns,.collapse_eligible_p99_ns]) and
  ordered_percentiles([.collapse_forced_off_p50_ns,.collapse_forced_off_p95_ns,.collapse_forced_off_p99_ns]) and
  (.paired_delta_median_ns | type == "number" and floor == .) and
  (.paired_delta_median_ns_per_track | type == "number") and
  ([.collapse_eligible_transposes_per_block,.collapse_forced_off_transposes_per_block] | all(positive_integer)) and
  # Two arms of one session pay one bank shape. A pair whose arms transposed differently would be
  # two plans, whatever their digests said.
  .collapse_eligible_transposes_per_block == .collapse_forced_off_transposes_per_block and
  # The premise: every track of the fixture is collapse-eligible, structurally and as prepared.
  .mono_source_tracks == .tracks and
  .symmetric_lanes == .tracks and
  (.lanes | positive_integer) and .lanes > .symmetric_lanes and
  ([.collapse_eligible_output_sha256,.collapse_forced_off_output_sha256] | all(sha256)) and
  .collapse_eligible_output_sha256 == .collapse_forced_off_output_sha256 and
  .bit_identity == "collapse_eligible == collapse_forced_off, asserted in-run" and
  .arm_difference == "collapse_eligible takes the mono collapse on every cohort; collapse_forced_off renders the same fixture dual" and
  .render_errors == 0 and .render_total_forbidden_operations == 0;

def console_benchmark_record_valid_lib:
  type == "object" and (.record | type == "string") and
  (if .record == "console_session" then session_record_valid
   elif .record == "console_hoist" then hoist_record_valid
   elif .record == "console_meters" then meters_record_valid
   elif .record == "console_observation" then observation_record_valid
   elif .record == "console_placement" then placement_record_valid
   elif .record == "console_automation" then automation_record_valid
   elif .record == "console_mono" then mono_record_valid
   else false end);
