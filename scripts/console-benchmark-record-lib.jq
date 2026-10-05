# Shared definitions for the console qualification benchmark records.
#
# Eight record shapes share one stream. `console_session` is a workload rendered through a real
# prepared plan; `console_hoist` is the paired-alternation comparison of the stationary-smoother
# arms; `console_meters` and `console_observation` are the #163 item 0d paired arms of the console
# observation facilities; `console_placement` is the #175 chain-shape row-pair; and
# `console_automation` is the automation-active row -- one Point span per block on one track,
# which is the only place in this stream a compressor's ramping body is executed at all; and
# `console_mono` is the mono-collapse row-pair, whose two arms are one session in this tree and
# whose digest equality is the gate the collapse will be constrained by when it lands; and
# `console_mixing_automation` (#1003) is the mono console riding eight controls on eight tracks in
# the shapes a real host pushes them.
# `console_benchmark_record_valid_lib` dispatches on `.record`, so a record that claims one shape
# and carries another's keys fails rather than being validated against the wrong table.
def sha256: type == "string" and test("^[0-9a-f]{64}$");
def nonnegative_integer: type == "number" and floor == . and . >= 0;
def positive_integer: type == "number" and floor == . and . > 0;

# The eleven runner-supplied metadata names, as they appear in a record.
def metadata_names: ["background_load_note","candidate_commit","cpu_affinity","cpu_model","governor_or_power_mode","llvm_version","measurement_control","profile","rust_version","target_features","target_triple"];

def session_keys: ["backend","background_load_note","candidate_commit","cpu_affinity","cpu_model","descriptive_only","fixture_id","governor_or_power_mode","input_signal","issue","llvm_version","max_ns_per_block","max_us_per_block","measurement_control","min_ns_per_block","min_us_per_block","missing_metadata","observations","os","output_sha256","p50_ns_per_block","p50_us_per_block","p50_us_per_block_per_track","p95_ns_per_block","p95_us_per_block","p99_ns_per_block","p99_us_per_block","percentile_method","profile","quantum_frames","record","render_errors","render_total_forbidden_operations","round","rust_version","sample_rate_hz","schema_version","source_feed","statistical_method","strip_content","strip_layout","synthetic_fixture","target_features","target_triple","tracks","units","workload_kind"];

# #881: the metered console row's meter group, carried by that row alone. Every other session
# row validates on `session_keys` exactly as before, so a standing row that grew the group, or the
# metered row without it, matches neither shape.
def metered_session_keys: ["bank_route_folds","bank_scatter_redirects","meter_dropped_snapshots","meter_metrics","meter_snapshots","meter_streams","meter_tap","meter_window_blocks"];

# #1085: the app shape's bypass group, carried by that row alone, on the same terms as the metered
# group: a standing row that grew it, or the app shape without it, matches no key set.
def bypass_session_keys: ["bypass_pattern","bypassed_tracks"];

def hoist_keys: ["arms","backend","background_load_note","bank_boundary","bit_identity","candidate_commit","cpu_affinity","cpu_model","descriptive_only","governor_or_power_mode","issue","llvm_version","measurement_control","missing_metadata","moving_output_sha256","moving_p50_ns","moving_p95_ns","moving_p99_ns","observations","os","paired_delta_median_ns","pairing","percentile_method","profile","quiet_output_sha256","quiet_p50_ns","quiet_p99_ns","record","restated_output_sha256","restated_p50_ns","restated_p95_ns","restated_p99_ns","round","rust_version","schema_version","statistical_method","target_features","target_triple","tracks","units","workload_kind"];

def meters_keys: ["arms","backend","background_load_note","bit_identity","candidate_commit","cpu_affinity","cpu_model","descriptive_only","governor_or_power_mode","issue","llvm_version","measurement_control","meter_frames_drained","meter_streams","meter_tap","meter_window_blocks","meters_off_bank_route_folds","meters_off_bank_scatter_redirects","meters_off_output_sha256","meters_off_p50_ns","meters_off_p95_ns","meters_off_p99_ns","meters_on_bank_route_folds","meters_on_bank_scatter_redirects","meters_on_output_sha256","meters_on_p50_ns","meters_on_p95_ns","meters_on_p99_ns","missing_metadata","observations","os","paired_delta_median_ns","pairing","percentile_method","profile","record","render_errors","render_total_forbidden_operations","round","rust_version","schema_version","statistical_method","target_features","target_triple","tracks","units","workload_kind"];

def placement_keys: ["arms","backend","background_load_note","bit_identity","candidate_commit","cpu_affinity","cpu_model","descriptive_only","governor_or_power_mode","issue","llvm_version","measurement_control","merged_chain_layout","merged_chain_output_sha256","merged_chain_p50_ns","merged_chain_p95_ns","merged_chain_p99_ns","merged_chain_transposes_per_block","missing_metadata","observations","os","paired_delta_median_ns","paired_delta_median_ns_per_track","pairing","percentile_method","profile","record","render_errors","render_total_forbidden_operations","round","rust_version","schema_version","split_chains_layout","split_chains_output_sha256","split_chains_p50_ns","split_chains_p95_ns","split_chains_p99_ns","split_chains_transposes_per_block","statistical_method","target_features","target_triple","tracks","units","workload_kind"];

# The mono row-pair (the mono-collapse gate). Two arms of one session today; see
# `mono_record_valid`.
def mono_keys: ["arm_difference","arms","backend","background_load_note","bit_identity","candidate_commit","collapse_eligible_output_sha256","collapse_eligible_p50_ns","collapse_eligible_p95_ns","collapse_eligible_p99_ns","collapse_eligible_transposes_per_block","collapse_forced_off_output_sha256","collapse_forced_off_p50_ns","collapse_forced_off_p95_ns","collapse_forced_off_p99_ns","collapse_forced_off_transposes_per_block","cpu_affinity","cpu_model","descriptive_only","fixture_id","governor_or_power_mode","issue","lanes","llvm_version","measurement_control","missing_metadata","mono_source_tracks","observations","os","paired_delta_median_ns","paired_delta_median_ns_per_track","pairing","percentile_method","profile","record","render_errors","render_total_forbidden_operations","round","rust_version","schema_version","statistical_method","symmetric_lanes","target_features","target_triple","tracks","units","workload_kind"];

def observation_keys: ["absent_output_sha256","absent_p50_ns","absent_p95_ns","absent_p99_ns","armed_output_sha256","armed_p50_ns","armed_p95_ns","armed_p99_ns","armed_windows_published","arms","backend","background_load_note","bit_identity","candidate_commit","cpu_affinity","cpu_model","descriptive_only","governor_or_power_mode","issue","llvm_version","measurement_control","missing_metadata","observation_lanes","observation_taps","observation_window_blocks","observations","os","paired_arm_delta_median_ns","paired_capacity_delta_median_ns","pairing","percentile_method","profile","record","render_errors","render_total_forbidden_operations","round","rust_version","schema_version","statistical_method","target_features","target_triple","tracks","unarmed_output_sha256","unarmed_p50_ns","unarmed_p95_ns","unarmed_p99_ns","unarmed_windows_published","units","workload_kind"];

def automation_keys: ["arms","automated_channel","automated_effect","automated_effect_id","automated_output_sha256","automated_p50_ns","automated_p95_ns","automated_p99_ns","automated_parameter","automated_parameter_index","automated_pushes_accepted","automated_track_id","automation_spans_per_block","backend","background_load_note","bit_identity","candidate_commit","cpu_affinity","cpu_model","descriptive_only","fixture_id","governor_or_power_mode","input_signal","issue","llvm_version","measurement_control","missing_metadata","observations","os","paired_control_delta_median_ns","paired_ramp_delta_median_ns","paired_ramp_delta_median_ns_per_track","pairing","percentile_method","profile","quantum_frames","quiet_output_sha256","quiet_p50_ns","quiet_p95_ns","quiet_p99_ns","record","render_errors","render_total_forbidden_operations","restated_output_sha256","restated_p50_ns","restated_p95_ns","restated_p99_ns","restated_pushes_accepted","round","rust_version","sample_rate_hz","schema_version","smoothing_samples","statistical_method","strip_content","strip_layout","synthetic_fixture","target_features","target_triple","tracks","units","workload_kind"];

def mixing_automation_keys: ["arms","automated_bank_collapse_counters","automated_controls","automated_output_sha256","automated_p50_ns","automated_p95_ns","automated_p99_ns","automated_pushes_accepted","backend","background_load_note","bit_identity","candidate_commit","cpu_affinity","cpu_model","descriptive_only","fixture_id","governor_or_power_mode","input_signal","issue","llvm_version","measurement_control","missing_metadata","observations","os","owner_edits_per_block","paired_collapse_delta_median_ns","paired_ramp_delta_median_ns","pairing","parameter_records_per_block","percentile_method","preflight_bank_collapse_counters","preflight_blocks","preflight_output_sha256","preroll_blocks","profile","quantum_frames","quiet_bank_collapse_counters","quiet_output_sha256","quiet_p50_ns","quiet_p95_ns","quiet_p99_ns","record","render_errors","render_total_forbidden_operations","restated_bank_collapse_counters","restated_output_sha256","restated_p50_ns","restated_p95_ns","restated_p99_ns","restated_pushes_accepted","round","rust_version","sample_rate_hz","schema_version","smoothing_samples","statistical_method","strip_content","strip_layout","synthetic_fixture","target_features","target_triple","tracks","units","workload_kind"];

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
# Both SVF sections are 29 lane-ops since #1328's joint flush (24 before), so the chain is 79 (69).
def builtins_lane_ops: 79;
# The rack-free rows do not share a floor. A builtin section prepared as the exact identity is
# elided, not executed, so the identity row's arithmetic is the 79 with both 29-op SVF sections
# replaced by the single `add(+0.0)` a run of identity sections composes to:
# 7 sanitise + 1 identity add + 4 boundary + 2 fader + 4 pan + 3 route + 1 reduction.
# It is the floor of the whole table (#956). Its last two lines, the route's `mix2x2` (3) and the
# output node's reduction amortised per track (1), are the routing component every row pays to
# reach the master; no row is costed at them alone, because the builtins-less plumbing row that was
# measured a plan no host compiles and was retired.
def builtins_identity_lane_ops: 22;
# The EQ at the standing fixture's one live section: a select-free depth-one pass (29 since #1328's
# joint SVF flush, 24 before) and the 4.4 boundary scan (3). #976 dropped the identity padding
# section that used to run beside it.
def eq_lane_ops: 32;
# Current-lowering recount (#368): max/min are one lane-op on x86 and wasm; the shared stereo
# link contributes a fractional half-op per channel sample. exp2_int_in_range is two operations
# after #367. These are inventories, not runtime measurements.
def compressor_lane_ops: 81.5;
def limiter_lane_ops: 129.5;
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
    # neither kernel has an identity arm, and that claim is what the shared inventory states. It
    # names no control.
    "sixty_four_track_gain_pan_only":
      [$bi, 1, "none", floor_document + "builtins, identity"],
    # Its driver-fed twin (#928, re-based onto the gain/pan session by #956) and the native
    # pure-path target: the same session with its track inputs claimed by a prepared source set.
    # Moving a frozen block into the graph is a copy, not a lane-op, whichever feed does it, so the
    # inventory is the gain/pan row's -- the floor of the table, not below it -- and like its twin
    # it names no control.
    "sixty_four_track_gain_pan_ring":
      [$bi, 1, "none", floor_document + "builtins, identity"],
    # The three mono rows carry the whole intended strip, so they carry its inventory. Their
    # fixture differs from the standing one in per-channel values only -- one source channel
    # instead of two, and the left channel's designed words on both sides -- and a floor is an
    # inventory of operations, not of operands.
    "sixty_four_track_console_mono":
      [$becl, 1, "none", floor_document + "builtins+eq+compressor+limiter"],
    "sixty_four_track_console_mono_dual":
      [$becl, 1, "none", floor_document + "builtins+eq+compressor+limiter"],
    "sixty_four_track_console_half_mono":
      [$becl, 1, "none", floor_document + "builtins+eq+compressor+limiter"],
    # The metered console row (#881). Its strip is the standing console's, but its meters are
    # arithmetic no ruling has inventoried, so it states no floor rather than the unmetered strip's,
    # and names no control rather than isolating the meters against a floor nobody derived.
    "sixty_four_track_console_metered":
      [null, 1, "none", "not_derived"],
    # The bus-and-send row (#1227): no inventory exists for a bus or a send.
    "sixty_four_track_console_sends":
      [null, 1, "none", "not_derived"],
    # The five console-strip rows (#1085). A remainder's width factor depends on whether it renders
    # per node or as a padded bank, which is what the console strip changes (the nine-track factor
    # holds for a remainder of one only, where the two cost the same); the app shape's bypassed
    # lanes and the sparse row's silent ones are arithmetic no inventory counts; and the sixteen-
    # track row is read with its set. H5's figures stay arithmetic until S4 reports, so none of
    # the five states a floor or names a control.
    "ten_track_ragged_strip":
      [null, 1, "none", "not_derived"],
    "thirteen_track_ragged_strip":
      [null, 1, "none", "not_derived"],
    "sixteen_track_strip":
      [null, 1, "none", "not_derived"],
    "sixty_four_track_app_shape":
      [null, 1, "none", "not_derived"],
    "sixty_four_track_console_sparse":
      [null, 1, "none", "not_derived"]
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


# The twenty-three session workloads, sorted: `WORKLOADS`'s fifteen (append-only, in emission
# order), the driver-fed gain/pan row the bench emits after them (#928 and #956,
# `DRIVER_FED_WORKLOADS`), the metered console row (#881, `METERED_WORKLOADS`), the bus-and-send
# row (#1227, `BUS_SEND_WORKLOADS`), and the five console-strip rows it emits last (#1085,
# `CONSOLE_STRIP_WORKLOADS`).
def session_kinds: ["nine_track_baseline","nine_track_ragged_strip","one_twenty_eight_track_stretch","sixteen_track_strip","sixty_four_track_app_shape","sixty_four_track_builtins_only","sixty_four_track_compressor_only","sixty_four_track_console","sixty_four_track_console_half_mono","sixty_four_track_console_legacy","sixty_four_track_console_metered","sixty_four_track_console_mono","sixty_four_track_console_mono_dual","sixty_four_track_console_sends","sixty_four_track_console_sparse","sixty_four_track_dispatch_only","sixty_four_track_eq_comp_simd1","sixty_four_track_eq_only","sixty_four_track_gain_pan_only","sixty_four_track_gain_pan_ring","sixty_four_track_idle","ten_track_ragged_strip","thirteen_track_ragged_strip"];

# #928: how a session row's track inputs reach the graph. `bound` is a `FrozenGraphSource`
# processor per track input, dispatched once per track per block; `played_planes` is a prepared
# source set whose driver copies each claim's block on request and lends its played planes in
# place -- the production feed. Exactly one row is driver-fed, and the feed is pinned per kind for
# the reason every other row fact is: a record claiming the production feed over bound processors,
# or the reverse, would attribute the feed's cost to the wrong row. The field is required on every
# session record, so a record from before it existed is refused; the driver-fed row's digest
# partner is pinned by the aggregate.
def driver_fed_kinds: ["sixty_four_track_gain_pan_ring"];
def session_source_feed:
  .workload_kind as $kind |
  .source_feed == (if any(driver_fed_kinds[]; . == $kind) then "played_planes" else "bound" end);

# #881: the rows prepared with the default web boot's meter set -- one `SAMPLE_PEAK` meter at
# `PostMatrix` per track, a twelve-block window, bound as permanent observers -- and the key sets a
# session record of each kind must carry exactly.
def metered_kinds: ["sixty_four_track_console_metered"];
def session_metered: .workload_kind as $kind | any(metered_kinds[]; . == $kind);
# #1085: the rows whose record states the bypass their compiled session carries.
def bypass_kinds: ["sixty_four_track_app_shape"];
def session_bypass: .workload_kind as $kind | any(bypass_kinds[]; . == $kind);
def session_row_keys:
  if session_metered then (session_keys + metered_session_keys) | sort
  elif session_bypass then (session_keys + bypass_session_keys) | sort
  else session_keys end;
def session_row_floor_keys: (session_row_keys + floor_keys) | sort;

# The metered row's meter group. The meter set is pinned field by field, because a row that metered
# another tap, another metric set or another window would publish a cost the browser does not pay
# under the name of the one it does. The count is exact: every stream is drained after every
# block, so each of the `observations` timed blocks that closes a window yields one snapshot per
# stream, and a stream that dropped one or published off its cadence changes the count. The fold
# and redirect counters are the #914 pair, which the digest cannot see: every route of the console
# folds whether or not a post-matrix meter reads it (#885). The redirect count is the metered plan's
# own; no record in the run shares its delivery, so none is its baseline (see the aggregate).
def metered_session_shape:
  .meter_streams == .tracks and .meter_tap == "post_matrix" and
  .meter_metrics == "sample_peak" and .meter_window_blocks == 12 and
  (.meter_snapshots | positive_integer) and
  .meter_snapshots == .meter_streams * ((.observations / .meter_window_blocks) | floor) and
  .meter_dropped_snapshots == 0 and
  ([.bank_route_folds,.bank_scatter_redirects] | all(nonnegative_integer)) and
  .bank_route_folds == .tracks;

# #1085: the app shape's bypass. The subject reads it off the compiled session and names the
# pattern only when every rack effect of exactly the tracks whose index is 2 mod 3 is bypassed and
# no other effect is; anything else is `other` and refused, and so is a count that is not that
# pattern's. A row that bypassed another set of tracks, or bypassed the EQ alone, measured another
# app than the one it names.
def bypass_session_shape:
  .bypass_pattern == "index_mod_3_is_2" and
  (.bypassed_tracks | nonnegative_integer) and
  .bypassed_tracks == ([range(.tracks) | select(. % 3 == 2)] | length);

# The standing qualification fixture (#175): the intended production layout, EQ and compressor as
# one two-slot chain on `simd1` and a true-peak limiter on `simd2`.
def console_fixture: "fixtures/session/v1/console-sixty-four-track-intended.json";
# #1085: the app shape, derived from the standing fixture by `scripts/derive-app-console-fixture.py`.
def app_console_fixture: "fixtures/session/v1/console-sixty-four-track-app.json";
# #1227: the bus-and-send session, derived from the standing fixture by
# `scripts/derive-sends-console-fixture.py`.
def sends_console_fixture: "fixtures/session/v1/console-sixty-four-track-sends.json";
# #1085: `strip_layout` names the chain in decision 12's console vocabulary, through its lowering
# (`simd1` -> `pre_insert`, `dynamic` -> `inserts`, `simd2` -> `post_insert`), so one spelling pins
# a row on today's per-track racks and on the console model. This is the intended strip's.
def intended_layout: "pre_insert:eq+compressor,post_insert:limiter";
# The retired fixture, rendered by exactly one row for exactly one transition record.
def legacy_console_fixture: "fixtures/session/v1/console-sixty-four-track.json";
# The mono qualification fixture: the standing strip with its source mapping and every upstream
# per-channel parameter symmetrised, so every track satisfies the channel-symmetry witness'
# structural terms. Its fader and pan asymmetry and its limiters' `maximum` link are kept, because
# they are what document the seam.
def mono_console_fixture: "fixtures/session/v1/console-sixty-four-track-mono.json";

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
    .strip_content == "eq" and .strip_layout == "pre_insert:eq" and .input_signal == "tone" and
    .fixture_id == "fixtures/session/v1/parametric-eq-nine-track.json"
  elif .workload_kind == "nine_track_ragged_strip" then
    .tracks == 9 and .synthetic_fixture == true and
    .strip_content == "eq+compressor+limiter" and
    .strip_layout == "pre_insert:eq+compressor,post_insert:limiter" and .input_signal == "tone" and
    .fixture_id == console_fixture
  elif .workload_kind == "sixty_four_track_console" then
    .tracks == 64 and .synthetic_fixture == false and
    .strip_content == "eq+compressor+limiter" and
    .strip_layout == "pre_insert:eq+compressor,post_insert:limiter" and .input_signal == "tone" and
    .fixture_id == console_fixture
  # The metered console row (#881) is that session as written, prepared as the default web boot
  # prepares it: meters, and the between-render-calls delivery that fuses each cohort's fader and
  # matrix into one stage. Neither is strip content -- a meter observes, and the fused stage computes
  # the split pair's arithmetic -- so it states the standing row's six facts, and its meter group
  # says what else it prepared.
  elif .workload_kind == "sixty_four_track_console_metered" then
    .tracks == 64 and .synthetic_fixture == false and
    .strip_content == "eq+compressor+limiter" and
    .strip_layout == "pre_insert:eq+compressor,post_insert:limiter" and .input_signal == "tone" and
    .fixture_id == console_fixture
  # The bus-and-send row (#1227): the standing console's tracks as written, feeding eight
  # processed buses and two effect returns from its committed fixture. Its tracks' strip is the
  # standing one, so it states the standing row's strip facts; the buses, the returns and the
  # sends are what its fixture adds, and naming that fixture is what tells it apart from the
  # standing row (the aggregate holds its digest apart from that row's too).
  elif .workload_kind == "sixty_four_track_console_sends" then
    .tracks == 64 and .synthetic_fixture == false and
    .strip_content == "eq+compressor+limiter" and
    .strip_layout == intended_layout and .input_signal == "tone" and
    .fixture_id == sends_console_fixture
  elif .workload_kind == "one_twenty_eight_track_stretch" then
    .tracks == 128 and .synthetic_fixture == true and
    .strip_content == "eq+compressor+limiter" and
    .strip_layout == "pre_insert:eq+compressor,post_insert:limiter" and .input_signal == "tone" and
    .fixture_id == console_fixture
  # The transition row (#175). The one row still rendered from the retired fixture, and the only
  # row in the stream whose `dynamic` rack carries anything. It exists so the standing authority's
  # first record and the retired authority's last one are taken on one host in one run; pinning
  # its fixture separately is what stops it quietly becoming a second copy of the standing row.
  elif .workload_kind == "sixty_four_track_console_legacy" then
    .tracks == 64 and .synthetic_fixture == false and
    .strip_content == "eq+compressor" and
    .strip_layout == "pre_insert:eq,inserts:compressor" and .input_signal == "tone" and
    .fixture_id == legacy_console_fixture
  # The chain-shape row: the standing fixture's two-slot chain carrying the retired fixture's
  # arithmetic. Identical `strip_content` to the row above and a different `strip_layout`, which is
  # the entire reason `strip_layout` is a field: these two rows are otherwise indistinguishable in
  # a record, and the number that separates them is attributed to chain shape alone.
  elif .workload_kind == "sixty_four_track_eq_comp_simd1" then
    .tracks == 64 and .synthetic_fixture == true and
    .strip_content == "eq+compressor" and
    .strip_layout == "pre_insert:eq+compressor" and .input_signal == "tone" and
    .fixture_id == console_fixture
  elif .workload_kind == "sixty_four_track_eq_only" then
    .tracks == 64 and .synthetic_fixture == true and
    .strip_content == "eq" and .strip_layout == "pre_insert:eq" and .input_signal == "tone" and
    .fixture_id == console_fixture
  elif .workload_kind == "sixty_four_track_compressor_only" then
    .tracks == 64 and .synthetic_fixture == true and
    .strip_content == "compressor" and .strip_layout == "pre_insert:compressor" and
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
  # Its driver-fed twin (#928, re-based by #956) states the same six facts: one session, fed two
  # ways. What separates the two records is `source_feed`, pinned by `session_source_feed`, and
  # nothing else.
  elif .workload_kind == "sixty_four_track_gain_pan_ring" then
    .tracks == 64 and .synthetic_fixture == true and
    .strip_content == "gain+pan" and .strip_layout == "builtins" and .input_signal == "tone" and
    .fixture_id == console_fixture
  # The mono row-pair, as session rows. Both render the mono fixture exactly as it is checked in,
  # so both are `synthetic_fixture == false`: they are two rows of one session, which is the
  # property their digest equality will rest on once the collapse exists.
  elif .workload_kind == "sixty_four_track_console_mono"
       or .workload_kind == "sixty_four_track_console_mono_dual" then
    .tracks == 64 and .synthetic_fixture == false and
    .strip_content == "eq+compressor+limiter" and
    .strip_layout == "pre_insert:eq+compressor,post_insert:limiter" and .input_signal == "tone" and
    .fixture_id == mono_console_fixture
  # The mixed-cohort row, derived in code from the mono fixture by putting the standing fixture's
  # stereo source mapping back on the odd tracks.
  elif .workload_kind == "sixty_four_track_console_half_mono" then
    .tracks == 64 and .synthetic_fixture == true and
    .strip_content == "eq+compressor+limiter" and
    .strip_layout == "pre_insert:eq+compressor,post_insert:limiter" and .input_signal == "tone" and
    .fixture_id == mono_console_fixture
  elif .workload_kind == "sixty_four_track_idle" then
    # The one row whose whole meaning is its input. The strip is the unmodified standing console
    # strip: the idle row measures a fully armed console rendering silence, not a stripped console
    # rendering anything.
    .tracks == 64 and .synthetic_fixture == true and
    .strip_content == "eq+compressor+limiter" and
    .strip_layout == "pre_insert:eq+compressor,post_insert:limiter" and .input_signal == "silence" and
    .fixture_id == console_fixture
  # #1085: the strip at N. The standing fixture's first N tracks, derived in code as the ragged
  # nine-track row is, so every one is synthetic and states the standing strip. Together with the
  # nine- and sixty-four-track rows they are the remainders the console strip's padding moves: two,
  # five, and none (two full banks).
  elif .workload_kind == "ten_track_ragged_strip" or
       .workload_kind == "thirteen_track_ragged_strip" or
       .workload_kind == "sixteen_track_strip" then
    .tracks == ({"ten_track_ragged_strip": 10, "thirteen_track_ragged_strip": 13,
                 "sixteen_track_strip": 16}[.workload_kind]) and
    .synthetic_fixture == true and
    .strip_content == "eq+compressor+limiter" and
    .strip_layout == intended_layout and .input_signal == "tone" and
    .fixture_id == console_fixture
  # The app shape: EQ -> compressor on every track, no limiter, a third of the tracks bypassed
  # (pinned by `bypass_session_shape`). Rendered from its committed fixture as written. It is the
  # one row whose layout the console migration moves: today its effects sit in `dynamic`, which
  # reads `inserts`, and once the app puts them in the session console they read `pre_insert`.
  # Both spellings are accepted for this row alone, so the row is pinned before and after.
  elif .workload_kind == "sixty_four_track_app_shape" then
    .tracks == 64 and .synthetic_fixture == false and
    .strip_content == "eq+compressor" and
    (.strip_layout == "inserts:eq+compressor" or .strip_layout == "pre_insert:eq+compressor") and
    .input_signal == "tone" and .fixture_id == app_console_fixture
  # Sparse activity: the standing strip as written, with every odd track fed exact zeros, so no
  # bank is wholly silent. Reported as derived for the reason the idle row is: its session is the
  # standing one, and what it renders is fed in code. Its digest is held apart from the all-active
  # and idle rows' by the aggregate.
  elif .workload_kind == "sixty_four_track_console_sparse" then
    .tracks == 64 and .synthetic_fixture == true and
    .strip_content == "eq+compressor+limiter" and
    .strip_layout == intended_layout and .input_signal == "odd_tracks_silent" and
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
def mixing_automation_statistical_method:
  "three arms alternated per observation; nearest-rank percentiles over per-block nanoseconds; ramp delta is automated minus restated and collapse delta is restated minus quiet, per observation; descriptive only; no threshold";
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
  ((keys | sort) == session_row_keys or (keys | sort) == session_row_floor_keys) and
  .record == "console_session" and common_shape and
  # The method is pinned verbatim. A record that changed how it was measured but kept the old
  # sentence would be the most expensive kind of quiet drift, so the sentence is part of the shape.
  .statistical_method == session_statistical_method and
  .sample_rate_hz == 48000 and .quantum_frames == 128 and
  .units == "us_per_block" and
  session_kind_shape and
  session_source_feed and
  ordered_percentiles([.min_ns_per_block,.p50_ns_per_block,.p95_ns_per_block,.p99_ns_per_block,.max_ns_per_block]) and
  ([.min_us_per_block,.p50_us_per_block,.p95_us_per_block,.p99_us_per_block,.max_us_per_block,.p50_us_per_block_per_track] | all(type == "number" and . > 0)) and
  (.output_sha256 | sha256) and
  .render_errors == 0 and .render_total_forbidden_operations == 0 and
  (if session_metered then metered_session_shape else true end) and
  (if session_bypass then bypass_session_shape else true end) and
  (if (keys | sort) == session_row_floor_keys then floor_shape else true end);

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
  # Issue #914: each arm's plan-shape counters. The route fold and the scatter redirect both render
  # the bits of the path they replace, so the digest pair above cannot say whether either fired and
  # neither can a timing; these counts are the only statement there is. Every route of the console
  # folds (`every_standing_workload_folds_one_route_per_track` pins it in code), and a post-matrix
  # meter folds exactly what the unmetered plan folds and moves no redirect (the #885 and #886
  # contracts). A meters-on arm that stopped folding would still render meters_off's bits and read
  # as the price of metering, so the record is refused rather than accepted.
  ([.meters_off_bank_route_folds,.meters_on_bank_route_folds,.meters_off_bank_scatter_redirects,.meters_on_bank_scatter_redirects] | all(nonnegative_integer)) and
  .meters_off_bank_route_folds == .tracks and
  .meters_on_bank_route_folds == .meters_off_bank_route_folds and
  .meters_on_bank_scatter_redirects == .meters_off_bank_scatter_redirects and
  .render_errors == 0 and .render_total_forbidden_operations == 0;

# The observation arm (#163 item 0d), which is the issue #143 two-level zero measured rather than
# argued: `absent` has no lane at all, `unarmed` has a lane with nothing armed, `armed` has every
# declared tap armed. All three carry the live-control channel, so the deltas are the
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
  .split_chains_layout == "pre_insert:eq,inserts:compressor" and
  .merged_chain_layout == "pre_insert:eq+compressor" and
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
  .strip_content == "compressor" and .strip_layout == "pre_insert:compressor" and
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
# whose structural (`SOURCE`) witness holds; `symmetric_lanes` is the eligible count over the
# banked track lanes alone (issue #911: the harness's identity output node is a symmetric op unit
# the collapse can never arm, so the whole-census count reads one above the track count), and
# `lanes` remains the prepared plan's whole census. Both must show every track eligible, because a pair measured
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

# The mixing-automation row (#1003): the mono console riding eight controls on eight tracks.
#
# The row's controls are its workload, so they are pinned control by control: which track, which
# slot, which parameter, the step, and the lowering each one is pushed in. The lowering is the
# claim VERIFY-AUTOMATION F2 turned on -- the EQ goes as one owner edit on `Both`, which keeps a
# mono cohort's collapse, and the compressor and the limiter as a Left and a Right record -- so a
# record that pushed the EQ as two one-channel edits measured a loss the product does not have and
# is refused. The bases are not pinned here: they are the fixture's held values, read from the
# model. The digest equality below shows the row restated a held value only where restating another
# value would move bits -- the EQ gains and the compressor thresholds. Neither limiter engages near
# its held ceiling, so a ceiling restated off it renders the same bits and passes every rule here;
# `restated_pushes_exactly_the_held_bases` in `tools/console-workload/tests/automation.rs` is what
# pins the restated value of all eight controls to their bases (#1011). Each ride must move: its
# two values straddle the base.
def mixing_automation_controls:
  [["ch00","eq","miso.parametric-eq","band-1-gain",3,"owner_both",0.25],
   ["ch08","comp","miso.compressor","threshold",0,"left_then_right",0.5],
   ["ch16","limiter","miso.true-peak-limiter","ceiling",0,"left_then_right",8],
   ["ch24","eq","miso.parametric-eq","band-1-gain",3,"owner_both",0.25],
   ["ch32","comp","miso.compressor","threshold",0,"left_then_right",0.5],
   ["ch40","limiter","miso.true-peak-limiter","ceiling",0,"left_then_right",8],
   ["ch48","eq","miso.parametric-eq","band-1-gain",3,"owner_both",0.25],
   ["ch56","comp","miso.compressor","threshold",0,"left_then_right",0.5]];
def mixing_automation_preflight_arms:
  ["quiet","restated","automated","automated_eq_only","automated_compressor_only","automated_limiter_only","restated_eq_only"];
def collapse_counters: type == "array" and length == 2 and all(nonnegative_integer) and .[1] > 0;

# Three claims beyond the controls, each one a statement the digests or the timings cannot make.
#
# The digests: `quiet == restated` is the collapse's own class-A statement -- a restating record
# may retire a cohort's collapse, and the dual bank renders the collapsed bank's bits -- and
# `restated != automated` is the honesty half. The preflight repeats both and adds that each effect
# moves bits on its own (VERIFY-AUTOMATION A3, F6): a whole-mix inequality could be carried by the
# EQ alone while the limiter's ramps moved nothing.
#
# The collapse counters: the quiet arm collapses every cohort on every block it rendered, or the
# row is not measuring a collapse at all; no arm collapses more than quiet; and the EQ-only
# restated preflight arm collapses exactly what quiet does (A6), which is what pins the harness to
# the SDK's `Both` shape. The restated and automated arms' counters are stated, not pinned: they
# are what the automation fixes move.
#
# The pushes: every block of both pushing arms, every record accepted.
def mixing_automation_record_valid:
  (keys | sort) == mixing_automation_keys and
  .record == "console_mixing_automation" and common_shape and
  .statistical_method == mixing_automation_statistical_method and
  .workload_kind == "sixty_four_track_console_mono_mixing_automation" and
  # The mono session row's six pinned facts, verbatim from `session_kind_shape`.
  .tracks == 64 and .synthetic_fixture == false and
  .strip_content == "eq+compressor+limiter" and
  .strip_layout == "pre_insert:eq+compressor,post_insert:limiter" and .input_signal == "tone" and
  .fixture_id == mono_console_fixture and
  .sample_rate_hz == 48000 and .quantum_frames == 128 and
  .pairing == "alternating_per_observation" and
  .arms == ["quiet","restated","automated"] and
  .preroll_blocks == 64 and
  (.automated_controls | type == "array" and
    map([.track_id,.slot_id,.effect,.parameter,.parameter_index,.lowering,.step])
      == mixing_automation_controls and
    all(.[]; ([.base,.even_value,.odd_value] | all(type == "number")) and
             .even_value > .base and .base > .odd_value)) and
  .owner_edits_per_block == ([.automated_controls[] | select(.lowering == "owner_both")] | length) and
  .parameter_records_per_block
    == 2 * ([.automated_controls[] | select(.lowering == "left_then_right")] | length) and
  .smoothing_samples == 64 and
  .restated_pushes_accepted == .observations * (.owner_edits_per_block + .parameter_records_per_block) and
  .automated_pushes_accepted == .restated_pushes_accepted and
  .units == "ns_per_block" and
  ([.quiet_p50_ns,.quiet_p95_ns,.quiet_p99_ns,.restated_p50_ns,.restated_p95_ns,.restated_p99_ns,.automated_p50_ns,.automated_p95_ns,.automated_p99_ns] | all(positive_integer)) and
  ordered_percentiles([.quiet_p50_ns,.quiet_p95_ns,.quiet_p99_ns]) and
  ordered_percentiles([.restated_p50_ns,.restated_p95_ns,.restated_p99_ns]) and
  ordered_percentiles([.automated_p50_ns,.automated_p95_ns,.automated_p99_ns]) and
  ([.paired_ramp_delta_median_ns,.paired_collapse_delta_median_ns] | all(type == "number" and floor == .)) and
  ([.quiet_bank_collapse_counters,.restated_bank_collapse_counters,.automated_bank_collapse_counters] | all(collapse_counters)) and
  ([.quiet_bank_collapse_counters,.restated_bank_collapse_counters,.automated_bank_collapse_counters] | map(.[1]) | unique | length == 1) and
  .quiet_bank_collapse_counters[0] == .quiet_bank_collapse_counters[1] * (.preroll_blocks + .observations) and
  .restated_bank_collapse_counters[0] <= .quiet_bank_collapse_counters[0] and
  .automated_bank_collapse_counters[0] <= .quiet_bank_collapse_counters[0] and
  ([.quiet_output_sha256,.restated_output_sha256,.automated_output_sha256] | all(sha256)) and
  .quiet_output_sha256 == .restated_output_sha256 and
  .restated_output_sha256 != .automated_output_sha256 and
  .bit_identity == "quiet == restated, asserted in-run" and
  .preflight_blocks == .preroll_blocks + 64 and
  (.preflight_output_sha256 | type == "object" and (keys_unsorted == mixing_automation_preflight_arms) and
    all(.[]; sha256) and
    .quiet == .restated and .restated_eq_only == .restated and
    .automated != .restated and .automated_eq_only != .restated and
    .automated_compressor_only != .restated and .automated_limiter_only != .restated) and
  (.preflight_bank_collapse_counters | type == "object" and
    (keys_unsorted == mixing_automation_preflight_arms) and all(.[]; collapse_counters)) and
  .preflight_bank_collapse_counters.quiet[0]
    == .preflight_bank_collapse_counters.quiet[1] * .preflight_blocks and
  .preflight_bank_collapse_counters.restated_eq_only == .preflight_bank_collapse_counters.quiet and
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
   elif .record == "console_mixing_automation" then mixing_automation_record_valid
   else false end);
