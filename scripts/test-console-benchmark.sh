#!/usr/bin/env bash
# Console validator mutation suite. Hermetic: no workload, no timing, no binary.
#
# A validator that has never been shown to reject anything is decoration. Every rule below is
# mutated in turn and asserted red, so the aggregate's guarantees -- forty-six records, both
# rounds, one host, one admissibility state, the decomposition rows' pinned strip contents, and
# the class-A statements that neither the stationary smoother nor a meter nor an armed observation
# tap nor a restated parameter nor (once it exists) a mono collapse changes a rendered bit -- are
# properties the suite can actually lose.
set -euo pipefail
[[ "$#" -le 1 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
scripts_dir="$root/scripts"
export LC_ALL=C

# The validators are invoked with the candidate on stdin, but the *assertion* helpers take their
# JSON as an argument. That is deliberate: a helper on the right of a pipe runs in a subshell, so
# every `failures=$((failures + 1))` inside one is discarded and the suite reports PASS no matter
# how many cases fail. This suite exists to be able to fail.
record_valid() { printf '%s' "$1" | jq -e -L "$scripts_dir" -f "$scripts_dir/console-benchmark-record-validator.jq" >/dev/null 2>&1; }
aggregate_valid() { printf '%s\n' "$1" | jq -s -e -L "$scripts_dir" -f "$scripts_dir/console-benchmark-validator.jq" >/dev/null 2>&1; }

failures=0
expect_accept() {
    if ! record_valid "$1"; then printf 'expected accept: %s\n' "$2" >&2; failures=$((failures + 1)); fi
}
expect_reject() {
    if record_valid "$1"; then printf 'expected reject: %s\n' "$2" >&2; failures=$((failures + 1)); fi
}
expect_aggregate_accept() {
    if ! aggregate_valid "$1"; then printf 'expected aggregate accept: %s\n' "$2" >&2; failures=$((failures + 1)); fi
}
expect_aggregate_reject() {
    if aggregate_valid "$1"; then printf 'expected aggregate reject: %s\n' "$2" >&2; failures=$((failures + 1)); fi
}

# ---------------------------------------------------------------------------------------------
# Base records. Digests are placeholders; the shapes are the real ones.
# ---------------------------------------------------------------------------------------------
digest_a=$(printf 'a%.0s' {1..64})
digest_b=$(printf 'b%.0s' {1..64})
digest_c=$(printf 'c%.0s' {1..64})

# The eleven runner-supplied metadata names plus `os`, shared by every record shape.
metadata=$(jq -cn '{
  cpu_model: "Test CPU", os: "linux", governor_or_power_mode: "performance",
  rust_version: "rustc 1.97.1", llvm_version: "21.1.4", target_triple: "x86_64-unknown-linux-gnu",
  target_features: "runtime-avx2,fma;baseline", profile: "release",
  background_load_note: "controlled; loadavg 0.01 0.02 0.00 1/1 1; ceiling 0.50; affinity cpu 15; smt siblings 7 cpu7=0.00%; cooldown 60s waited 0s",
  measurement_control: "controlled", cpu_affinity: "15",
  candidate_commit: "0123456789abcdef0123456789abcdef01234567",
  missing_metadata: []
}')

session=$(jq -cn --arg a "$digest_a" --argjson m "$metadata" '$m + {
  schema_version: 1, issue: 149, record: "console_session",
  workload_kind: "sixty_four_track_console", tracks: 64, synthetic_fixture: false,
  fixture_id: "fixtures/session/v1/console-sixty-four-track-intended.toml",
  round: 1, backend: "Simd8", sample_rate_hz: 48000, quantum_frames: 128, observations: 1000,
  units: "us_per_block", percentile_method: "nearest_rank",
  min_us_per_block: 281.9, p50_us_per_block: 283.4, p95_us_per_block: 285.4,
  p99_us_per_block: 295.7, max_us_per_block: 297.2, p50_us_per_block_per_track: 4.429,
  min_ns_per_block: 281915, p50_ns_per_block: 283459, p95_ns_per_block: 285482,
  p99_ns_per_block: 295702, max_ns_per_block: 297245,
  output_sha256: $a, render_errors: 0, render_total_forbidden_operations: 0,
  descriptive_only: true, strip_content: "eq+compressor+limiter",
  strip_layout: "simd1:eq+compressor,simd2:limiter", input_signal: "tone",
  statistical_method: "nearest-rank percentiles over per-block nanoseconds; one warmup pass and two measured rounds; descriptive only; no threshold"
}')

hoist=$(jq -cn --arg a "$digest_a" --arg b "$digest_b" --argjson m "$metadata" '$m + {
  schema_version: 1, issue: 149, record: "console_hoist",
  workload_kind: "sixty_four_track_console", tracks: 64, round: 1, backend: "Simd8",
  bank_boundary: "effect_bank", observations: 1000, pairing: "alternating_per_observation",
  arms: ["quiet","restated","moving"],
  units: "ns_per_block", percentile_method: "nearest_rank",
  quiet_p50_ns: 31069, quiet_p99_ns: 32000,
  restated_p50_ns: 38313, restated_p95_ns: 39000, restated_p99_ns: 39946,
  moving_p50_ns: 39405, moving_p95_ns: 40100, moving_p99_ns: 41148,
  paired_delta_median_ns: 1083,
  quiet_output_sha256: $a, restated_output_sha256: $a, moving_output_sha256: $b,
  bit_identity: "quiet == restated, asserted in-run",
  descriptive_only: true,
  statistical_method: "three arms alternated per observation; nearest-rank percentiles over per-block nanoseconds; paired delta is moving minus restated per observation; descriptive only; no threshold"
}')

# The two #163 item 0d facility records. Both are paired-alternation comparisons of one workload
# under two or three preparations, and both carry a class-A statement: observing a console must not
# change what the console renders.
meters=$(jq -cn --arg a "$digest_a" --argjson m "$metadata" '$m + {
  schema_version: 1, issue: 149, record: "console_meters",
  workload_kind: "sixty_four_track_console", tracks: 64, round: 1, backend: "Simd8",
  observations: 1000, pairing: "alternating_per_observation",
  arms: ["meters_off","meters_on"],
  meter_streams: 64, meter_tap: "post_matrix", meter_window_blocks: 4,
  meter_frames_drained: 16000,
  units: "ns_per_block", percentile_method: "nearest_rank",
  meters_off_p50_ns: 245738, meters_off_p95_ns: 250000, meters_off_p99_ns: 252000,
  meters_on_p50_ns: 262840, meters_on_p95_ns: 266000, meters_on_p99_ns: 268000,
  paired_delta_median_ns: 17063,
  meters_off_output_sha256: $a, meters_on_output_sha256: $a,
  bit_identity: "meters_off == meters_on, asserted in-run",
  render_errors: 0, render_total_forbidden_operations: 0,
  descriptive_only: true,
  statistical_method: "two arms alternated per observation; nearest-rank percentiles over per-block nanoseconds; paired delta is meters_on minus meters_off per observation; descriptive only; no threshold"
}')

observation=$(jq -cn --arg a "$digest_a" --argjson m "$metadata" '$m + {
  schema_version: 1, issue: 149, record: "console_observation",
  workload_kind: "sixty_four_track_console", tracks: 64, round: 1, backend: "Simd8",
  observations: 1000, pairing: "alternating_per_observation",
  arms: ["absent","unarmed","armed"],
  observation_lanes: 64, observation_taps: 64, observation_window_blocks: 4,
  unarmed_windows_published: 0, armed_windows_published: 17024,
  units: "ns_per_block", percentile_method: "nearest_rank",
  absent_p50_ns: 249193, absent_p95_ns: 253000, absent_p99_ns: 255000,
  unarmed_p50_ns: 249915, unarmed_p95_ns: 254000, unarmed_p99_ns: 256000,
  armed_p50_ns: 251535, armed_p95_ns: 256000, armed_p99_ns: 258000,
  paired_capacity_delta_median_ns: 661, paired_arm_delta_median_ns: 1620,
  absent_output_sha256: $a, unarmed_output_sha256: $a, armed_output_sha256: $a,
  bit_identity: "absent == unarmed == armed, asserted in-run",
  render_errors: 0, render_total_forbidden_operations: 0,
  descriptive_only: true,
  statistical_method: "three arms alternated per observation; nearest-rank percentiles over per-block nanoseconds; capacity delta is unarmed minus absent and arm delta is armed minus unarmed, per observation; descriptive only; no threshold"
}')

# The #175 chain-shape row-pair. Two arms carrying identical arithmetic in two rack layouts.
placement=$(jq -cn --arg a "$digest_a" --argjson m "$metadata" '$m + {
  schema_version: 1, issue: 149, record: "console_placement",
  workload_kind: "sixty_four_track_placement", tracks: 64, round: 1, backend: "Simd8",
  observations: 1000, pairing: "alternating_per_observation",
  arms: ["split_chains","merged_chain"],
  split_chains_layout: "simd1:eq,dynamic:compressor",
  merged_chain_layout: "simd1:eq+compressor",
  units: "ns_per_block", percentile_method: "nearest_rank",
  split_chains_p50_ns: 94510, split_chains_p95_ns: 96000, split_chains_p99_ns: 97000,
  merged_chain_p50_ns: 95522, merged_chain_p95_ns: 97000, merged_chain_p99_ns: 98000,
  paired_delta_median_ns: 1032, paired_delta_median_ns_per_track: 16.125,
  split_chains_transposes_per_block: 24, merged_chain_transposes_per_block: 24,
  split_chains_output_sha256: $a, merged_chain_output_sha256: $a,
  bit_identity: "split_chains == merged_chain, asserted in-run",
  render_errors: 0, render_total_forbidden_operations: 0,
  descriptive_only: true,
  statistical_method: "two arms alternated per observation; nearest-rank percentiles over per-block nanoseconds; paired delta is merged_chain minus split_chains per observation; descriptive only; no threshold"
}')

# The automation-active row. Three arms on one control channel; only one of them opens a window.
automation=$(jq -cn --arg a "$digest_a" --arg b "$digest_b" --argjson m "$metadata" '$m + {
  schema_version: 1, issue: 149, record: "console_automation",
  workload_kind: "sixty_four_track_compressor_automation", tracks: 64,
  synthetic_fixture: true, strip_content: "compressor", strip_layout: "simd1:compressor",
  input_signal: "tone", fixture_id: "fixtures/session/v1/console-sixty-four-track-intended.toml",
  round: 1, backend: "Simd8", sample_rate_hz: 48000, quantum_frames: 128,
  observations: 1000, pairing: "alternating_per_observation",
  arms: ["quiet","restated","automated"],
  automated_track_id: "ch00", automated_effect_id: "comp",
  automated_effect: "miso.compressor", automated_parameter: "threshold",
  automated_parameter_index: 0, automated_channel: "left",
  automation_spans_per_block: 1, smoothing_samples: 64,
  restated_pushes_accepted: 1000, automated_pushes_accepted: 1000,
  units: "ns_per_block", percentile_method: "nearest_rank",
  quiet_p50_ns: 73329, quiet_p95_ns: 75000, quiet_p99_ns: 76000,
  restated_p50_ns: 74101, restated_p95_ns: 76000, restated_p99_ns: 77000,
  automated_p50_ns: 76685, automated_p95_ns: 78000, automated_p99_ns: 79000,
  paired_ramp_delta_median_ns: 2585,
  paired_ramp_delta_median_ns_per_track: 40.390625,
  paired_control_delta_median_ns: 811,
  quiet_output_sha256: $a, restated_output_sha256: $a, automated_output_sha256: $b,
  bit_identity: "quiet == restated, asserted in-run",
  render_errors: 0, render_total_forbidden_operations: 0,
  descriptive_only: true,
  statistical_method: "three arms alternated per observation; nearest-rank percentiles over per-block nanoseconds; ramp delta is automated minus restated and control delta is restated minus quiet, per observation; descriptive only; no threshold"
}')

# The mono row-pair. Two arms of one session in this tree, which is exactly what makes its three
# claims worth mutating: the digest equality that will gate the collapse, the census that says the
# fixture is collapse-eligible at all, and the `arm_difference` sentence that stops today's zero
# delta from reading as a measured saving.
mono=$(jq -cn --arg a "$digest_a" --argjson m "$metadata" '$m + {
  schema_version: 1, issue: 149, record: "console_mono",
  workload_kind: "sixty_four_track_mono_pair", tracks: 64, round: 1, backend: "Simd8",
  observations: 1000, pairing: "alternating_per_observation",
  arms: ["collapse_eligible","collapse_forced_off"],
  fixture_id: "fixtures/session/v1/console-sixty-four-track-mono.toml",
  units: "ns_per_block", percentile_method: "nearest_rank",
  collapse_eligible_p50_ns: 121904, collapse_eligible_p95_ns: 124000,
  collapse_eligible_p99_ns: 126000,
  collapse_forced_off_p50_ns: 121970, collapse_forced_off_p95_ns: 124100,
  collapse_forced_off_p99_ns: 126200,
  paired_delta_median_ns: 61, paired_delta_median_ns_per_track: 0.953125,
  collapse_eligible_transposes_per_block: 8, collapse_forced_off_transposes_per_block: 8,
  mono_source_tracks: 64, symmetric_lanes: 64, lanes: 129,
  collapse_eligible_output_sha256: $a, collapse_forced_off_output_sha256: $a,
  bit_identity: "collapse_eligible == collapse_forced_off, asserted in-run",
  arm_difference: "none: both arms are the mono fixture as written; no collapse exists in this tree",
  render_errors: 0, render_total_forbidden_operations: 0,
  descriptive_only: true,
  statistical_method: "two arms alternated per observation; nearest-rank percentiles over per-block nanoseconds; paired delta is collapse_forced_off minus collapse_eligible per observation; descriptive only; no threshold"
}')

# ---------------------------------------------------------------------------------------------
# The #184 floor group. A session record either carries all eleven columns or none of them.
# ---------------------------------------------------------------------------------------------
#
# The derived columns are computed here the way the subject computes them, through the validator
# library's own inventories rather than a third copy of the numbers -- the suite's job is to mutate
# a correct record, not to restate the derivation. `tools/miso-engine-bench/src/floor.rs` is the
# authority; the end-to-end agreement between it and the library is what a real run proves.
core_clock_source='perf stat cycles/task-clock over the warmup launch, cpu 15'
add_floor='
  include "console-benchmark-record-lib";
  def with_floor($clock; $source):
    (floor_pins[.workload_kind]) as $pin |
    (.tracks * .quantum_frames * 2) as $lane_samples |
    (.p50_ns_per_block * $clock / 1000000000) as $cycles |
    ($cycles / $lane_samples) as $per_lane |
    (if $pin[0] == null then null else $pin[0] * $pin[1] / lane_ops_per_cycle end) as $floor |
    . + {
      lane_samples_per_block: $lane_samples,
      core_clock_hz: $clock,
      core_clock_source: $source,
      cycles_per_block_p50: $cycles,
      cycles_per_lane_sample: $per_lane,
      floor_cycles_per_lane_sample: $floor,
      percent_of_floor: (if $floor == null then null else 100 * $floor / $per_lane end),
      floor_basis: $pin[3],
      floor_control_row: $pin[2],
      isolated_cycles_per_lane_sample: null,
      isolated_percent_of_floor: null
    };
'
# The console row isolates the limiter against the chain-shape row, so it must claim an isolate.
# Its value is a subtraction between two records and only the aggregate can recompute it; what a
# single record can be held to is that it claims one at all.
session_floor=$(printf '%s' "$session" | jq -c -L "$scripts_dir" --arg s "$core_clock_source" \
    "$add_floor"' with_floor(5480000000; $s)
      | .isolated_cycles_per_lane_sample = 20.5
      | .isolated_percent_of_floor = 22.75')
# The one row whose fixture was never inventoried. Null floors, and a basis that says so.
session_floor_not_derived=$(printf '%s' "$session" | jq -c -L "$scripts_dir" --arg s "$core_clock_source" \
    "$add_floor"' .workload_kind = "nine_track_baseline" | .tracks = 9
      | .synthetic_fixture = false | .strip_content = "eq" | .strip_layout = "simd1:eq"
      | .fixture_id = "fixtures/session/v1/parametric-eq-nine-track.toml"
      | with_floor(5480000000; $s)')

expect_accept "$session" 'the base session record'
expect_accept "$hoist" 'the base hoist record'
expect_accept "$meters" 'the base meters record'
expect_accept "$observation" 'the base observation record'
expect_accept "$placement" 'the base placement record'
expect_accept "$automation" 'the base automation record'
expect_accept "$mono" 'the base mono row-pair record'
# The identity row's own inventory. Since the prepared-identity elision the two rack-free rows do
# not share a floor -- `dispatch_only` elides both SVF sections rather than executing them -- so
# this row is the one that proves the split is enforced rather than merely written down.
session_floor_dispatch=$(printf '%s' "$session" | jq -c -L "$scripts_dir" --arg s "$core_clock_source" \
    "$add_floor"' .workload_kind = "sixty_four_track_dispatch_only" | .synthetic_fixture = true
      | .strip_content = "identity" | .strip_layout = "builtins"
      | with_floor(5480000000; $s)')

# The two overhead rows. `plumbing_only` is the floor of the table -- the route and the master
# reduction and nothing else -- and `gain_pan_only` is the identity inventory again, isolated
# against it. Both are built through the library's own pins rather than by writing the numbers a
# second time.
session_floor_plumbing=$(printf '%s' "$session" | jq -c -L "$scripts_dir" --arg s "$core_clock_source" \
    "$add_floor"' .workload_kind = "sixty_four_track_plumbing_only" | .synthetic_fixture = true
      | .strip_content = "plumbing" | .strip_layout = "plumbing"
      | with_floor(5480000000; $s)')
session_floor_gain_pan=$(printf '%s' "$session" | jq -c -L "$scripts_dir" --arg s "$core_clock_source" \
    "$add_floor"' .workload_kind = "sixty_four_track_gain_pan_only" | .synthetic_fixture = true
      | .strip_content = "gain+pan" | .strip_layout = "builtins"
      | with_floor(5480000000; $s)')

expect_accept "$session_floor" 'the base session record carrying the floor columns'
expect_accept "$session_floor_plumbing" 'the overhead floor row'
expect_accept "$session_floor_gain_pan" 'the gain-and-pan row, which names no control'
expect_accept "$session_floor_not_derived" 'a row whose fixture was never inventoried'
expect_accept "$session_floor_dispatch" 'the identity row carrying the identity inventory'

# ---------------------------------------------------------------------------------------------
# Per-key structural mutations: every key is load-bearing in both directions.
# ---------------------------------------------------------------------------------------------
for base in "$session" "$session_floor" "$hoist" "$meters" \
    "$observation" "$placement" "$automation" "$mono"; do
    kind=$(printf '%s' "$base" | jq -r '.record')
    while read -r field; do
        expect_reject "$(printf '%s' "$base" | jq -c "del(.\"$field\")")" "$kind without $field"
        # `null` is the one mutation that is a type change for every field in both records --
        # number, boolean, string and array alike -- and a null metadata field that is not named
        # in `missing_metadata` is exactly the #104 F2 dishonesty the validator has to refuse.
        expect_reject "$(printf '%s' "$base" | jq -c ".\"$field\" = null")" "$kind with $field nulled"
    done < <(printf '%s' "$base" | jq -r 'keys[]')
    expect_reject "$(printf '%s' "$base" | jq -c '.unexpected_key = 1')" "$kind with an extra key"
done

# The uninventoried row takes the deletion half of the sweep only. Four of its floor columns are
# legitimately `null` -- that is what "no floor was derived for this fixture" looks like in a
# record -- so nulling them is the identity rather than a mutation, and asserting it red would be
# asserting that an honest record is dishonest.
while read -r field; do
    expect_reject "$(printf '%s' "$session_floor_not_derived" | jq -c "del(.\"$field\")")" \
        "the uninventoried row without $field"
    if [[ "$(printf '%s' "$session_floor_not_derived" | jq -c ".\"$field\"")" != null ]]; then
        expect_reject "$(printf '%s' "$session_floor_not_derived" | jq -c ".\"$field\" = null")" \
            "the uninventoried row with $field nulled"
    fi
done < <(printf '%s' "$session_floor_not_derived" | jq -r 'keys[]')
expect_reject "$(printf '%s' "$session_floor_not_derived" | jq -c '.unexpected_key = 1')" \
    'the uninventoried row with an extra key'

# ---------------------------------------------------------------------------------------------
# Semantic mutations: each names the property it destroys.
# ---------------------------------------------------------------------------------------------
session_mutation() { expect_reject "$(printf '%s' "$session" | jq -c "$1")" "$2"; }
hoist_mutation() { expect_reject "$(printf '%s' "$hoist" | jq -c "$1")" "$2"; }

session_mutation '.issue = 38' 'a session record from another issue'
session_mutation '.schema_version = 2' 'an unfrozen schema version'
session_mutation '.round = 3' 'a third round'
session_mutation '.sample_rate_hz = 44100' 'a rate other than the launch rate'
session_mutation '.quantum_frames = 64' 'a quantum other than the launch quantum'
session_mutation '.observations = 100' 'a shortened observation count'
session_mutation '.percentile_method = "linear"' 'an interpolating percentile'
session_mutation '.units = "ns_per_frame"' 'the wrong unit'
session_mutation '.descriptive_only = false' 'a record claiming to be a gate'
session_mutation '.statistical_method = "descriptive"' 'a record whose method sentence drifted'
session_mutation '.os = ""' 'an empty operating-system field'
# The track count and the fixture are pinned together per workload: a sixty-four-track claim over a
# nine-track fixture is exactly the fiction the bench discipline exists to refuse.
session_mutation '.tracks = 63' 'a console record that is not eight full banks'
session_mutation '.workload_kind = "nine_track_baseline"' 'a kind that contradicts its track count'
session_mutation '.synthetic_fixture = true' 'a written fixture reported as synthetic'
session_mutation '.fixture_id = "fixtures/session/v1/canonical.toml"' 'a record naming another fixture'
session_mutation '.p50_ns_per_block = 999999999' 'percentiles out of order'
session_mutation '.min_ns_per_block = -1' 'a negative duration'
session_mutation '.render_errors = 1' 'a run that produced render errors'
session_mutation '.render_total_forbidden_operations = 1' 'a run that allocated on the render path'
session_mutation '.output_sha256 = "short"' 'a malformed digest'
# #104 F2: the honesty half. A null field must be named in missing_metadata and a placeholder must
# not pass as a value.
session_mutation '.cpu_model = null' 'a null metadata field not named in missing_metadata'
session_mutation '.cpu_model = "unknown"' 'a placeholder metadata value'
session_mutation '.cpu_model = ""' 'an empty metadata value'
session_mutation '.missing_metadata = ["cpu_model"]' 'a gap claimed for a field that resolved'

# #184: the floor group. Every derived column is recomputed by the validator from the columns it
# was derived from, so a wrong number is caught rather than merely a missing one. A column that is
# present but arbitrary is the failure mode this block exists to make red.
session_floor_mutation() { expect_reject "$(printf '%s' "$session_floor" | jq -c "$1")" "$2"; }

session_floor_mutation '.cycles_per_lane_sample = 1.0' 'a cycle count that does not follow from the clock'
session_floor_mutation '.cycles_per_block_p50 = 1.0' 'a block cycle count that does not follow from the wall time'
session_floor_mutation '.core_clock_hz = 3000000000' 'a clock the cycle columns were not derived under'
session_floor_mutation '.core_clock_hz = 1000' 'a clock outside any plausible core frequency'
session_floor_mutation '.core_clock_source = ""' 'a measured clock with no provenance'
session_floor_mutation '.lane_samples_per_block = 8192' 'a lane-sample count that is not tracks x frames x channels'
session_floor_mutation '.tracks = 9 | .workload_kind = "nine_track_ragged_strip"' 'a lane-sample count left behind by a changed track count'
session_floor_mutation '.floor_cycles_per_lane_sample = 1.0' 'a floor that does not follow from the published inventory'
session_floor_mutation '.floor_cycles_per_lane_sample = null' 'a derived row that dropped its floor'
session_floor_mutation '.percent_of_floor = 99.0' 'a percentage that flatters its own measurement'
session_floor_mutation '.percent_of_floor = null' 'a floor stated without the percentage it implies'
session_floor_mutation '.floor_basis = "docs/rulings/effect-floor-accounting.md: builtins"' 'a row citing another row inventory'
session_floor_mutation '.floor_basis = "not_derived"' 'a derived row claiming it was never inventoried'
session_floor_mutation '.floor_control_row = "sixty_four_track_builtins_only"' 'a row subtracting a control it does not isolate against'
session_floor_mutation '.floor_control_row = "none"' 'a row that claims an isolate and names no control'
session_floor_mutation '.isolated_cycles_per_lane_sample = -1' 'an isolate that costs less than nothing'
session_floor_mutation '.isolated_percent_of_floor = -1' 'an isolate percentage below zero'
# The rack-free split. A `dispatch_only` record that restates the 69-op builtins inventory --
# self-consistently, floor and percentage together, so the only thing wrong with it is the
# inventory itself -- is a row that claims to execute sections the render path elides.
expect_reject "$(printf '%s' "$session_floor_dispatch" | jq -c \
    '.floor_cycles_per_lane_sample = (69 / (8 * 3.7))
     | .percent_of_floor = (100 * (69 / (8 * 3.7)) / .cycles_per_lane_sample)')" \
    'an identity row costed as if it executed its filters'
expect_reject "$(printf '%s' "$session_floor_dispatch" | jq -c \
    '.floor_basis = "docs/rulings/effect-floor-accounting.md: builtins"')" \
    'an identity row citing the executed-filter inventory'
# And the other direction: the row that does execute them must not borrow the identity inventory.
expect_reject "$(printf '%s' "$session_floor" | jq -c \
    '.floor_basis = "docs/rulings/effect-floor-accounting.md: builtins, identity"')" \
    'a row citing the identity inventory it does not qualify for'
# The overhead pair. `plumbing_only` is the floor of the whole table, so a row costed at the
# identity inventory it is the floor *of* is the same defect as the identity row costed at 69 --
# self-consistent, floor and percentage together, and wrong about which arithmetic it executes.
expect_reject "$(printf '%s' "$session_floor_plumbing" | jq -c \
    '.floor_cycles_per_lane_sample = (22 / (8 * 3.7))
     | .percent_of_floor = (100 * (22 / (8 * 3.7)) / .cycles_per_lane_sample)')" \
    'a plumbing row costed as if it prepared a strip'
expect_reject "$(printf '%s' "$session_floor_plumbing" | jq -c \
    '.floor_basis = "docs/rulings/effect-floor-accounting.md: builtins, identity"')" \
    'a plumbing row citing the identity inventory'
expect_reject "$(printf '%s' "$session_floor_gain_pan" | jq -c \
    '.floor_basis = "docs/rulings/effect-floor-accounting.md: plumbing"')" \
    'a gain-and-pan row citing the plumbing inventory'
# Neither overhead row may claim an isolate. The inventories subtract -- 22 - 4 is the scaffolding
# -- but the rows do not, because a banked row folds its route and reduction into its chain's
# epilogue and an unbanked one dispatches them per track. A record that named the plumbing row as
# its control would be publishing a subtraction that removes the fold's saving along with the
# plumbing's arithmetic, and it comes in below the floor it claims to be measured against.
expect_reject "$(printf '%s' "$session_floor_gain_pan" | jq -c \
    '.floor_control_row = "sixty_four_track_plumbing_only" | .isolated_cycles_per_lane_sample = 0.5 | .isolated_percent_of_floor = 121.9')" \
    'a gain-and-pan row isolated against the unbanked plumbing row'
expect_reject "$(printf '%s' "$session_floor_gain_pan" | jq -c \
    '.floor_control_row = "sixty_four_track_builtins_only" | .isolated_cycles_per_lane_sample = 0.5 | .isolated_percent_of_floor = 20.0')" \
    'a gain-and-pan row isolated against a row it is not a subset of'
expect_reject "$(printf '%s' "$session_floor_plumbing" | jq -c \
    '.floor_control_row = "sixty_four_track_gain_pan_only" | .isolated_cycles_per_lane_sample = 1.0 | .isolated_percent_of_floor = 1.0')" \
    'the floor row subtracting a row above it'
# The not-derived row is the other half of the same rule: it must not invent a floor either.
expect_reject "$(printf '%s' "$session_floor_not_derived" | jq -c '.floor_cycles_per_lane_sample = 11.892 | .percent_of_floor = 12.5')" 'an uninventoried fixture given a floor anyway'
expect_reject "$(printf '%s' "$session_floor_not_derived" | jq -c '.floor_basis = "docs/rulings/effect-floor-accounting.md: builtins+eq"')" 'an uninventoried fixture citing an inventory'
# Additive means additive: the sealed shape stays legal, and half the group is not a shape at all.
expect_accept "$session" 'a record from before the floor columns existed'
expect_reject "$(printf '%s' "$session_floor" | jq -c 'del(.percent_of_floor, .isolated_percent_of_floor)')" 'a record carrying half the floor group'


# #163 item 0c: the decomposition rows. Every one of them is the console fixture with part of the
# strip removed, and the subtraction between two rows only means something if each row's declared
# content is what the subject actually built. A row claiming a rack it emptied is the fiction.
session_mutation '.workload_kind = "sixty_four_track_eq_only"' \
    'an eq-only row still claiming the compressor'
session_mutation '.workload_kind = "sixty_four_track_eq_only" | .strip_content = "eq" | .synthetic_fixture = true | .input_signal = "silence"' \
    'an eq-only row claiming silent input'
session_mutation '.workload_kind = "sixty_four_track_compressor_only" | .strip_content = "eq" | .synthetic_fixture = true' \
    'a compressor-only row claiming the eq'
session_mutation '.workload_kind = "sixty_four_track_builtins_only" | .strip_content = "identity" | .synthetic_fixture = true' \
    'a builtins-only row claiming identity builtins'
session_mutation '.workload_kind = "sixty_four_track_dispatch_only" | .strip_content = "builtins" | .synthetic_fixture = true' \
    'a dispatch-only row claiming live builtins'
session_mutation '.workload_kind = "sixty_four_track_eq_only" | .strip_content = "eq" | .synthetic_fixture = false' \
    'a derived row reported as a checked-in fixture'
session_mutation '.workload_kind = "sixty_four_track_idle" | .synthetic_fixture = true' \
    'an idle row rendering a tone'
session_mutation '.workload_kind = "sixty_four_track_idle" | .synthetic_fixture = true | .input_signal = "silence" | .strip_content = "identity"' \
    'an idle row that emptied the strip it claims to idle'
session_mutation '.strip_content = "eq+compressor+saturator"' 'a strip content no workload declares'

# The overhead rows. `gain_pan_only` and `dispatch_only` share a strip edit but for one field and
# share a floor inventory, so the only thing that separates them in a record is what they say they
# carried -- which makes each row claiming the other's content exactly the fiction to refuse.
session_mutation '.workload_kind = "sixty_four_track_gain_pan_only" | .strip_content = "identity" | .strip_layout = "builtins" | .synthetic_fixture = true' \
    'a gain-and-pan row claiming the identity fader and pan'
session_mutation '.workload_kind = "sixty_four_track_dispatch_only" | .strip_content = "gain+pan" | .strip_layout = "builtins" | .synthetic_fixture = true' \
    'an identity row claiming the fixture fader and pan'
# `plumbing` is a layout word of its own. A row that prepares no builtin binding at all reported as
# a `builtins` row would put the overhead floor and the thing it is the floor *of* under one name.
session_mutation '.workload_kind = "sixty_four_track_plumbing_only" | .strip_content = "plumbing" | .strip_layout = "builtins" | .synthetic_fixture = true' \
    'a plumbing row claiming the builtins layout'
session_mutation '.workload_kind = "sixty_four_track_builtins_only" | .strip_content = "plumbing" | .strip_layout = "plumbing" | .synthetic_fixture = true' \
    'a builtins row claiming it prepared no builtin'
session_mutation '.workload_kind = "sixty_four_track_plumbing_only" | .strip_content = "plumbing" | .strip_layout = "plumbing" | .synthetic_fixture = false' \
    'a derived plumbing row reported as a checked-in fixture'
expect_accept "$(printf '%s' "$session" | jq -c '.workload_kind = "sixty_four_track_plumbing_only" | .strip_content = "plumbing" | .strip_layout = "plumbing" | .synthetic_fixture = true')" \
    'an honest plumbing row'
expect_accept "$(printf '%s' "$session" | jq -c '.workload_kind = "sixty_four_track_gain_pan_only" | .strip_content = "gain+pan" | .strip_layout = "builtins" | .synthetic_fixture = true')" \
    'an honest gain-and-pan row'

# The mono session rows. Both arms render the mono fixture as written, so both are checked-in
# rather than derived; the half-mono row is the one that is derived, and it is derived from the
# same file. A mono row pointed at the standing fixture would be the standing console row wearing
# the name the collapse gate is going to be read off.
mono_fixture="fixtures/session/v1/console-sixty-four-track-mono.toml"
session_mutation ".workload_kind = \"sixty_four_track_console_mono\" | .fixture_id = \"fixtures/session/v1/console-sixty-four-track-intended.toml\"" \
    'a mono row rendered from the standing stereo fixture'
session_mutation ".workload_kind = \"sixty_four_track_console_mono\" | .fixture_id = \"$mono_fixture\" | .synthetic_fixture = true" \
    'a mono row reported as derived in code'
session_mutation ".workload_kind = \"sixty_four_track_console_half_mono\" | .fixture_id = \"$mono_fixture\" | .synthetic_fixture = false" \
    'the half-mono row reported as a checked-in fixture'
session_mutation ".workload_kind = \"sixty_four_track_console_mono\" | .fixture_id = \"$mono_fixture\" | .strip_content = \"eq+compressor\"" \
    'a mono row claiming it carries no limiter'
session_mutation ".fixture_id = \"$mono_fixture\"" \
    'the standing console row rendered from the mono fixture'
for kind in sixty_four_track_console_mono sixty_four_track_console_mono_dual; do
    expect_accept "$(printf '%s' "$session" | jq -c --arg f "$mono_fixture" --arg k "$kind" '.workload_kind = $k | .fixture_id = $f')" \
        "an honest $kind row"
done
expect_accept "$(printf '%s' "$session" | jq -c --arg f "$mono_fixture" '.workload_kind = "sixty_four_track_console_half_mono" | .fixture_id = $f | .synthetic_fixture = true')" \
    'an honest half-mono row'

# #175: `strip_layout` is pinned per kind for the same reason every other row fact is. Two rows in
# this stream now carry the same `strip_content` and differ only in where those effects sit, so a
# layout that drifted from what the subject built would silently turn the chain-shape row-pair
# into a comparison of one layout with itself.
session_mutation '.strip_layout = "simd1:eq+compressor"' \
    'a standing console row claiming the limiter-free layout'
session_mutation '.strip_layout = "simd1:eq,dynamic:compressor"' \
    'a standing console row claiming the retired layout'
session_mutation '.strip_layout = "simd2:eq+compressor,simd1:limiter"' \
    'a layout naming racks in an order the strip does not run'
session_mutation '.strip_layout = "dynamic:eq+compressor+limiter"' \
    'a layout no workload declares'
# The transition row and the chain-shape row: identical `strip_content`, different everything else
# that matters. Each must reject the other's identity.
session_mutation '.workload_kind = "sixty_four_track_console_legacy" | .strip_content = "eq+compressor" | .strip_layout = "simd1:eq+compressor"' \
    'a legacy row claiming the merged chain shape'
session_mutation '.workload_kind = "sixty_four_track_console_legacy" | .strip_content = "eq+compressor" | .strip_layout = "simd1:eq,dynamic:compressor"' \
    'a legacy row rendered from the standing fixture'
session_mutation '.workload_kind = "sixty_four_track_eq_comp_simd1" | .strip_content = "eq+compressor" | .strip_layout = "simd1:eq,dynamic:compressor" | .synthetic_fixture = true' \
    'a chain-shape row claiming the retired layout'
session_mutation '.workload_kind = "sixty_four_track_eq_comp_simd1" | .strip_content = "eq+compressor" | .strip_layout = "simd1:eq+compressor" | .synthetic_fixture = false' \
    'a derived chain-shape row reported as a checked-in fixture'
expect_accept "$(printf '%s' "$session" | jq -c --arg f "fixtures/session/v1/console-sixty-four-track.toml" '.workload_kind = "sixty_four_track_console_legacy" | .strip_content = "eq+compressor" | .strip_layout = "simd1:eq,dynamic:compressor" | .fixture_id = $f')" \
    'an honest transition row'
expect_accept "$(printf '%s' "$session" | jq -c '.workload_kind = "sixty_four_track_eq_comp_simd1" | .strip_content = "eq+compressor" | .strip_layout = "simd1:eq+compressor" | .synthetic_fixture = true')" \
    'an honest chain-shape row'
session_mutation '.input_signal = "noise"' 'an input signal no workload declares'
# Accept the honest forms, so the pins above are shown to be pins and not a blanket refusal.
expect_accept "$(printf '%s' "$session" | jq -c '.workload_kind = "sixty_four_track_eq_only" | .strip_content = "eq" | .strip_layout = "simd1:eq" | .synthetic_fixture = true')" \
    'an honest eq-only row'
expect_accept "$(printf '%s' "$session" | jq -c '.workload_kind = "sixty_four_track_idle" | .synthetic_fixture = true | .input_signal = "silence"')" \
    'an honest idle row'

# #144 item 13 / #163 phase 0a: admissibility. The record has to say whether its measurement was
# controlled, and the claim has to agree with the rest of the record.
session_mutation '.measurement_control = "mostly"' 'a third admissibility state'
session_mutation '.cpu_affinity = "uncontrolled"' 'a controlled run that pinned no core'
session_mutation '.cpu_affinity = "cpu15"' 'a controlled run whose affinity is not a core number'
session_mutation '.background_load_note = "uncontrolled; MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1; waived affinity_unavailable"' \
    'a controlled claim over an uncontrolled note'
session_mutation '.measurement_control = "uncontrolled"' \
    'an uncontrolled claim over a controlled note'
session_mutation '.measurement_control = "uncontrolled" | .background_load_note = "uncontrolled; loadavg 9.9"' \
    'an uncontrolled record that does not name the escape hatch it used'
expect_accept "$(printf '%s' "$session" | jq -c '.measurement_control = "uncontrolled" | .cpu_affinity = "uncontrolled" | .background_load_note = "uncontrolled; MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1; waived affinity_unavailable loadavg_above_ceiling; loadavg 9.9"')" \
    'an honest uncontrolled record'

hoist_mutation '.pairing = "sequential"' 'arms that were not alternated'
hoist_mutation '.arms = ["restated","moving"]' 'a comparison missing its untouched arm'
hoist_mutation '.observations = 10' 'a shortened observation count'
hoist_mutation '.restated_p50_ns = 999999' 'percentiles out of order'
hoist_mutation '.quiet_p50_ns = 0' 'a zero-cost arm'
hoist_mutation '.tracks = 12' 'a track count no workload declares'
hoist_mutation '.bank_boundary = "session"' 'a record misstating its measurement boundary'
# The class-A statement itself. These two are the reason the record carries digests at all.
expect_reject "$(printf '%s' "$hoist" | jq -c --arg c "$digest_c" '.restated_output_sha256 = $c')" \
    'a hoist that changed rendered output'
expect_reject "$(printf '%s' "$hoist" | jq -c '.moving_output_sha256 = .restated_output_sha256')" \
    'a control arm that did not move'
hoist_mutation '.bit_identity = "not checked"' 'a record that dropped its identity statement'
hoist_mutation '.statistical_method = "paired"' 'a hoist record whose method sentence drifted'
hoist_mutation '.backend = ""' 'an empty backend field'

# ---------------------------------------------------------------------------------------------
# #163 item 0d: the console-facility arms.
# ---------------------------------------------------------------------------------------------
meters_mutation() { expect_reject "$(printf '%s' "$meters" | jq -c "$1")" "$2"; }
observation_mutation() { expect_reject "$(printf '%s' "$observation" | jq -c "$1")" "$2"; }

meters_mutation '.pairing = "sequential"' 'meter arms that were not alternated'
meters_mutation '.arms = ["meters_on"]' 'a comparison with only one arm'
meters_mutation '.arms = ["meters_off","meters_armed"]' 'an arm name the subject does not run'
meters_mutation '.meter_streams = 32' 'a meters arm that metered half its tracks'
meters_mutation '.meter_tap = "post_fader"' 'a meters arm at a tap no console defaults to'
meters_mutation '.meter_window_blocks = 0' 'a meter window of no blocks'
# The arm has to have metered. This is the mutation that makes "meters cost nothing" impossible to
# report by accident.
meters_mutation '.meter_frames_drained = 0' 'a meters-on arm that published no meter frame'
meters_mutation '.meters_on_p50_ns = 999999999' 'meter percentiles out of order'
meters_mutation '.meters_off_p50_ns = 0' 'a zero-cost arm'
meters_mutation '.workload_kind = "sixty_four_track_eq_only"' 'a meters arm on another workload'
meters_mutation '.tracks = 32' 'a meters arm whose track count is not the console workload'
meters_mutation '.render_total_forbidden_operations = 1' 'a meters arm that allocated on the render path'
meters_mutation '.statistical_method = "paired"' 'a meters record whose method sentence drifted'
# The class-A statement. Metering is observation: it may not change a rendered bit.
expect_reject "$(printf '%s' "$meters" | jq -c --arg c "$digest_c" '.meters_on_output_sha256 = $c')" \
    'a meter attachment that changed rendered output'
meters_mutation '.bit_identity = "not checked"' 'a meters record that dropped its identity statement'

observation_mutation '.pairing = "sequential"' 'observation arms that were not alternated'
observation_mutation '.arms = ["unarmed","armed"]' 'an observation comparison missing its level-1 zero'
observation_mutation '.arms = ["absent","unarmed","subscribed"]' 'an arm name the subject does not run'
observation_mutation '.observation_lanes = 0' 'an observation record with no lane prepared'
observation_mutation '.observation_taps = 0' 'an observation record with no tap prepared'
observation_mutation '.observation_window_blocks = 0' 'an observation window of no blocks'
# The two halves of the honesty gate, which are the whole reason the record carries window counts.
observation_mutation '.unarmed_windows_published = 1' 'an unarmed arm that published a window'
observation_mutation '.armed_windows_published = 0' 'an armed arm that published nothing'
observation_mutation '.armed_p50_ns = 999999999' 'observation percentiles out of order'
observation_mutation '.absent_p50_ns = 0' 'a zero-cost arm'
observation_mutation '.workload_kind = "sixty_four_track_idle"' 'an observation arm on another workload'
observation_mutation '.render_errors = 1' 'an observation arm that produced render errors'
observation_mutation '.statistical_method = "paired"' 'an observation record whose method sentence drifted'
expect_reject "$(printf '%s' "$observation" | jq -c --arg c "$digest_c" '.armed_output_sha256 = $c')" \
    'arming a tap changed rendered output'
expect_reject "$(printf '%s' "$observation" | jq -c --arg c "$digest_c" '.unarmed_output_sha256 = $c')" \
    'attaching observation capacity changed rendered output'
observation_mutation '.bit_identity = "not checked"' 'an observation record that dropped its identity statement'

# A record that claims one shape and carries another'"'"'s keys must fail rather than be validated
# against the wrong table.
expect_reject "$(printf '%s' "$meters" | jq -c '.record = "console_observation"')" \
    'a meters record claiming to be an observation record'
expect_reject "$(printf '%s' "$observation" | jq -c '.record = "console_meters"')" \
    'an observation record claiming to be a meters record'
expect_reject "$(printf '%s' "$session" | jq -c '.record = "console_meters"')" \
    'a session record claiming to be a meters record'

# ---------------------------------------------------------------------------------------------
# Aggregate mutations.
# ---------------------------------------------------------------------------------------------
placement_mutation() { expect_reject "$(printf '%s' "$placement" | jq -c "$1")" "$2"; }

# The #175 row-pair's own claims. Two of them exist nowhere else in this stream.
placement_mutation '.arms = ["merged_chain","split_chains"]' 'placement arms in the wrong order'
placement_mutation '.arms = ["split_chains","merged_chain","limiter"]' 'a third placement arm'
placement_mutation '.pairing = "sequential"' 'a placement pair that was not alternated'
placement_mutation '.workload_kind = "sixty_four_track_console"' 'a placement record claiming a session kind'
placement_mutation '.tracks = 9' 'a placement pair that is not eight full banks'
placement_mutation '.units = "us_per_block"' 'the wrong placement unit'
placement_mutation '.statistical_method = "two arms alternated per observation"' \
    'a placement record whose method sentence drifted'
# The layouts *are* the comparison. A pair that names one layout twice is comparing nothing.
placement_mutation '.split_chains_layout = "simd1:eq+compressor"' 'a pair whose two arms name one layout'
placement_mutation '.merged_chain_layout = "simd1:eq,dynamic:compressor"' 'a merged arm claiming the split layout'
placement_mutation '.split_chains_layout = "simd1:eq+compressor,simd2:limiter"' 'a split arm carrying the limiter'
# The class-A claim. A placement change that moved a rendered bit is not a chain-shape measurement,
# and this is the one record in the stream that would notice.
placement_mutation '.merged_chain_output_sha256 = "'"$digest_b"'"' \
    'a placement pair whose two layouts rendered different output'
placement_mutation '.bit_identity = "asserted"' 'a placement record whose bit-identity sentence drifted'
placement_mutation '.bit_identity = "split_chains != merged_chain, asserted in-run"' \
    'a placement record claiming its layouts differ'
# The transpose counts are what make the delta explicable rather than merely reported.
placement_mutation '.split_chains_transposes_per_block = 0' 'an arm that transposed nothing'
placement_mutation '.merged_chain_transposes_per_block = 0' 'a merged arm that transposed nothing'
placement_mutation '.split_chains_p50_ns = 999999' 'placement percentiles out of order'
placement_mutation '.merged_chain_p99_ns = 1' 'merged placement percentiles out of order'
placement_mutation '.render_errors = 1' 'a placement pair that produced render errors'
placement_mutation '.render_total_forbidden_operations = 1' 'a placement pair that allocated on the render path'

# The automation-active row. Its two digest rules are the whole claim, so both are mutated in both
# directions: a row whose restated arm moved a bit is not measuring a window, and a row whose
# automated arm moved none is measuring nothing.
automation_mutation() { expect_reject "$(printf '%s' "$automation" | jq -c "$1")" "$2"; }

automation_mutation '.arms = ["quiet","automated","restated"]' 'automation arms in the wrong order'
automation_mutation '.arms = ["quiet","restated"]' 'an automation row missing its restated control'
automation_mutation '.pairing = "sequential"' 'an automation row that was not alternated'
automation_mutation '.record = "console_session"' 'an automation record claiming the session shape'
automation_mutation '.workload_kind = "sixty_four_track_compressor_only"' \
    'an automation row claiming the quiet decomposition kind'
automation_mutation '.units = "us_per_block"' 'the wrong automation unit'
automation_mutation '.statistical_method = "three arms alternated per observation"' \
    'an automation method sentence that drifted'
# The subject row's six pinned facts. A row repointed at another workload reports a ramping
# surcharge for a session nobody named.
automation_mutation '.strip_content = "eq+compressor+limiter"' 'an automation row claiming the full strip'
automation_mutation '.strip_layout = "simd1:eq+compressor,simd2:limiter"' 'an automation row claiming the intended layout'
automation_mutation '.synthetic_fixture = false' 'a derived automation row claiming a checked-in fixture'
automation_mutation '.input_signal = "silence"' 'an automation row claiming silence while rendering a tone'
automation_mutation '.tracks = 9' 'an automation row that is not eight full banks'
automation_mutation '.fixture_id = "fixtures/session/v1/console-sixty-four-track.toml"' \
    'an automation row claiming the retired fixture'
# What rides the control channel.
automation_mutation '.automation_spans_per_block = 64' 'a row claiming one span per block while sending sixty-four'
automation_mutation '.automated_parameter = "makeup"' 'a row that named the wrong automated parameter'
automation_mutation '.automated_parameter_index = 5' 'a parameter index that does not match its name'
automation_mutation '.automated_channel = "both"' 'a channel the compressor does not accept'
automation_mutation '.automated_effect = "miso.parametric-eq"' 'a row that automated the wrong effect'
automation_mutation '.smoothing_samples = 32' 'a window length that is not the descriptor'"'"'s'
automation_mutation '.restated_pushes_accepted = 999' 'a restated arm whose queue refused a push'
automation_mutation '.automated_pushes_accepted = 0' 'an automated arm that pushed nothing'
# The class-A statement and its honesty half.
automation_mutation '.restated_output_sha256 = "'"$digest_c"'"' \
    'a restated arm that moved a rendered bit'
automation_mutation '.automated_output_sha256 = "'"$digest_a"'"' \
    'an automated arm that rendered the restated arm'"'"'s bits'
automation_mutation '.bit_identity = "quiet != restated, asserted in-run"' \
    'an automation row that inverted its own class-A sentence'
automation_mutation '.bit_identity = "asserted"' 'an automation bit-identity sentence that drifted'
automation_mutation '.quiet_p99_ns = 1' 'automation percentiles out of order'
automation_mutation '.automated_p50_ns = 999999' 'automated percentiles out of order'
automation_mutation '.render_errors = 1' 'an automation row that produced render errors'
automation_mutation '.render_total_forbidden_operations = 1' 'an automation row that allocated on the render path'
# And the honest form is accepted, so every pin above is shown to be a pin rather than a blanket
# refusal of the shape.
expect_accept "$automation" 'the honest automation row'
# A record that claims one shape and carries another's keys.
expect_reject "$(printf '%s' "$placement" | jq -c '.record = "console_meters"')" \
    'a placement record claiming to be a meters record'
expect_reject "$(printf '%s' "$meters" | jq -c '.record = "console_placement"')" \
    'a meters record claiming to be a placement record'
# The saving the hypothesis predicted would show up here first: an arm pair whose transpose counts
# differ is a real finding, not a malformed record, so it must still validate.
expect_accept "$(printf '%s' "$placement" | jq -c '.split_chains_transposes_per_block = 32 | .paired_delta_median_ns = -4000 | .paired_delta_median_ns_per_track = -62.5')" \
    'a placement pair that did save a round-trip'

# ---------------------------------------------------------------------------------------------
# The mono row-pair: the gate the mono collapse will be measured and constrained by.
# ---------------------------------------------------------------------------------------------
mono_mutation() { expect_reject "$(printf '%s' "$mono" | jq -c "$1")" "$2"; }

mono_mutation '.arms = ["collapse_forced_off","collapse_eligible"]' 'mono arms in the wrong order'
mono_mutation '.arms = ["collapse_eligible"]' 'a mono pair with one arm'
mono_mutation '.arms = ["collapse_eligible","collapse_forced_off","collapse_partial"]' 'a third mono arm'
mono_mutation '.pairing = "sequential"' 'a mono pair that was not alternated'
mono_mutation '.record = "console_placement"' 'a mono record claiming the placement shape'
mono_mutation '.workload_kind = "sixty_four_track_console_mono"' 'a mono pair claiming a session kind'
mono_mutation '.tracks = 9' 'a mono pair that is not eight full banks'
mono_mutation '.units = "us_per_block"' 'the wrong mono unit'
mono_mutation '.statistical_method = "two arms alternated per observation"' \
    'a mono record whose method sentence drifted'
mono_mutation '.fixture_id = "fixtures/session/v1/console-sixty-four-track-intended.toml"' \
    'a mono pair measured on the standing stereo fixture'
# The class-A statement, which is the whole reason the pair exists. Trivially true today and the
# gate on the collapse tomorrow, so both the digests and the sentence that reports them are pinned.
mono_mutation '.collapse_forced_off_output_sha256 = "'"$digest_b"'"' \
    'a collapse-eligible session that rendered something other than the same session uncollapsed'
mono_mutation '.bit_identity = "asserted"' 'a mono bit-identity sentence that drifted'
mono_mutation '.bit_identity = "collapse_eligible != collapse_forced_off, asserted in-run"' \
    'a mono record that inverted its own class-A sentence'
# The premise. Without these three, a pair measured on an ordinary stereo session would pass the
# digest equality perfectly -- by being one session rendered twice under a name it has not earned.
mono_mutation '.mono_source_tracks = 0' 'a mono pair whose fixture has no mono-source track'
mono_mutation '.mono_source_tracks = 32' 'a mono pair measured on a half-mono session'
mono_mutation '.symmetric_lanes = 0' 'a mono pair whose prepared lanes are not symmetric'
mono_mutation '.lanes = 64' 'a lane census that counts only the lanes it calls symmetric'
# The honesty field. A near-zero delta with this sentence removed reads as a measured saving.
mono_mutation '.arm_difference = "the collapse is taken on the eligible arm"' \
    'a mono record claiming a collapse this tree does not have'
mono_mutation '.arm_difference = ""' 'a mono record that dropped its arm-difference statement'
# Two arms of one session are one plan.
mono_mutation '.collapse_forced_off_transposes_per_block = 16' \
    'a mono pair whose two arms realised different bank shapes'
mono_mutation '.collapse_eligible_transposes_per_block = 0' 'a mono arm that transposed nothing'
mono_mutation '.collapse_eligible_p50_ns = 999999' 'mono percentiles out of order'
mono_mutation '.collapse_forced_off_p50_ns = 0' 'a zero-cost mono arm'
mono_mutation '.render_errors = 1' 'a mono pair that produced render errors'
mono_mutation '.render_total_forbidden_operations = 1' 'a mono pair that allocated on the render path'
# And a pair that did measure a difference is a real finding, not a malformed record: when the
# collapse lands, the delta becomes nonzero and the record must still validate.
expect_accept "$(printf '%s' "$mono" | jq -c '.collapse_forced_off_p50_ns = 160000 | .collapse_forced_off_p95_ns = 162000 | .collapse_forced_off_p99_ns = 164000 | .paired_delta_median_ns = 38000 | .paired_delta_median_ns_per_track = 593.75')" \
    'a mono pair that did measure a saving'

records=$(jq -cn --argjson session "$session" --argjson hoist "$hoist" \
    --argjson meters "$meters" --argjson observation "$observation" \
    --argjson placement "$placement" --argjson automation "$automation" \
    --argjson mono "$mono" \
    --arg a "$digest_a" --arg b "$digest_b" '
  def console_fixture: "fixtures/session/v1/console-sixty-four-track-intended.toml";
  def legacy_fixture: "fixtures/session/v1/console-sixty-four-track.toml";
  def mono_fixture: "fixtures/session/v1/console-sixty-four-track-mono.toml";
  def intended_layout: "simd1:eq+compressor,simd2:limiter";
  def sessions: [
    {kind: "nine_track_baseline", tracks: 9, synthetic: false, strip: "eq", layout: "simd1:eq",
     signal: "tone", fixture: "fixtures/session/v1/parametric-eq-nine-track.toml", digest: "1"},
    {kind: "nine_track_ragged_strip", tracks: 9, synthetic: true, strip: "eq+compressor+limiter",
     layout: intended_layout, signal: "tone", fixture: console_fixture, digest: "2"},
    {kind: "sixty_four_track_console", tracks: 64, synthetic: false, strip: "eq+compressor+limiter",
     layout: intended_layout, signal: "tone", fixture: console_fixture, digest: "3"},
    {kind: "one_twenty_eight_track_stretch", tracks: 128, synthetic: true,
     strip: "eq+compressor+limiter", layout: intended_layout, signal: "tone",
     fixture: console_fixture, digest: "4"},
    {kind: "sixty_four_track_eq_only", tracks: 64, synthetic: true, strip: "eq",
     layout: "simd1:eq", signal: "tone", fixture: console_fixture, digest: "5"},
    {kind: "sixty_four_track_compressor_only", tracks: 64, synthetic: true, strip: "compressor",
     layout: "simd1:compressor", signal: "tone", fixture: console_fixture, digest: "6"},
    {kind: "sixty_four_track_builtins_only", tracks: 64, synthetic: true, strip: "builtins",
     layout: "builtins", signal: "tone", fixture: console_fixture, digest: "7"},
    {kind: "sixty_four_track_dispatch_only", tracks: 64, synthetic: true, strip: "identity",
     layout: "builtins", signal: "tone", fixture: console_fixture, digest: "8"},
    {kind: "sixty_four_track_idle", tracks: 64, synthetic: true, strip: "eq+compressor+limiter",
     layout: intended_layout, signal: "silence", fixture: console_fixture, digest: "9"},
    {kind: "sixty_four_track_console_legacy", tracks: 64, synthetic: false, strip: "eq+compressor",
     layout: "simd1:eq,dynamic:compressor", signal: "tone", fixture: legacy_fixture, digest: "e"},
    {kind: "sixty_four_track_eq_comp_simd1", tracks: 64, synthetic: true, strip: "eq+compressor",
     layout: "simd1:eq+compressor", signal: "tone", fixture: console_fixture, digest: "f"},
    {kind: "sixty_four_track_plumbing_only", tracks: 64, synthetic: true, strip: "plumbing",
     layout: "plumbing", signal: "tone", fixture: console_fixture, digest: "0"},
    {kind: "sixty_four_track_gain_pan_only", tracks: 64, synthetic: true, strip: "gain+pan",
     layout: "builtins", signal: "tone", fixture: console_fixture, digest: "a"},
    {kind: "sixty_four_track_console_mono", tracks: 64, synthetic: false,
     strip: "eq+compressor+limiter", layout: intended_layout, signal: "tone",
     fixture: mono_fixture, digest: "b"},
    {kind: "sixty_four_track_console_mono_dual", tracks: 64, synthetic: false,
     strip: "eq+compressor+limiter", layout: intended_layout, signal: "tone",
     fixture: mono_fixture, digest: "c"},
    {kind: "sixty_four_track_console_half_mono", tracks: 64, synthetic: true,
     strip: "eq+compressor+limiter", layout: intended_layout, signal: "tone",
     fixture: mono_fixture, digest: "d"}
  ];
  def hoists: [
    {kind: "nine_track_ragged_strip", tracks: 9, digest: "a"},
    {kind: "sixty_four_track_console", tracks: 64, digest: "b"}
  ];
  [ (1, 2) as $round | (
      (sessions[] | . as $s | $session
        | .workload_kind = $s.kind | .tracks = $s.tracks
        | .synthetic_fixture = $s.synthetic
        | .strip_content = $s.strip | .strip_layout = $s.layout | .input_signal = $s.signal
        | .fixture_id = $s.fixture | .round = $round
        | .output_sha256 = ($a[0:63] + $s.digest)),
      (hoists[] | . as $h | $hoist
        | .workload_kind = $h.kind | .tracks = $h.tracks | .round = $round
        | .quiet_output_sha256 = ($a[0:63] + $h.digest)
        | .restated_output_sha256 = ($a[0:63] + $h.digest)
        | .moving_output_sha256 = $b),
      ($meters | .round = $round),
      ($observation | .round = $round),
      ($placement | .round = $round),
      ($automation | .round = $round),
      ($mono | .round = $round)
  ) ]')

expect_aggregate_accept "$(printf '%s' "$records" | jq -c '.[]')" 'the forty-six-record set'

# Index map of the frozen emission order: 0-15 are round one's sixteen session rows, 16-17 its two
# hoist rows, 18 its meters row, 19 its observation row, 20 its placement row-pair, 21 its
# automation-active row and 22 its mono row-pair; 23-45 repeat for round two.
expect_aggregate_reject "$(printf '%s' "$records" | jq -c 'del(.[0]) | .[]')" 'forty-five records'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '. as $r | ($r + [$r[21]]) | .[]')" 'forty-seven records'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '. as $r | ($r + [$r[0]]) | .[]')" 'a duplicated record'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[23].round = 1 | .[]')" 'a workload measured twice in one round'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[0].cpu_model = "Another CPU" | .[]')" 'records from two hosts'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[0].candidate_commit = "ffffffffffffffffffffffffffffffffffffffff" | .[]')" 'records from two commits'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[0].backend = "Scalar" | .[]')" 'records from two backends'
# Round one and round two must render the same bytes: they are two measurements of one workload.
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '.[23].output_sha256 = $c | .[]')" 'a workload whose rounds rendered different output'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.record == "console_session")] | .[]')" 'a set with no hoist rows'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.record != "console_meters")] | .[]')" 'a set with no meters arm'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.record != "console_observation")] | .[]')" 'a set with no observation arm'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '.[41].meters_off_output_sha256 = $c | .[41].meters_on_output_sha256 = $c | .[]')" 'a meters arm whose rounds rendered different output'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '.[42].absent_output_sha256 = $c | .[42].unarmed_output_sha256 = $c | .[42].armed_output_sha256 = $c | .[]')" 'an observation arm whose rounds rendered different output'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '.[43].split_chains_output_sha256 = $c | .[43].merged_chain_output_sha256 = $c | .[]')" 'a placement pair whose rounds rendered different output'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.record != "console_placement")] | .[]')" 'a set with no placement row-pair'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.record != "console_automation")] | .[]')" 'a set with no automation-active row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '.[44].quiet_output_sha256 = $c | .[44].restated_output_sha256 = $c | .[]')" 'an automation row whose rounds rendered different output'
# #144 item 13: two admissibility states in one accepted run is the comparison the control field
# exists to prevent, and a run that never stated one at all is not an accepted run.
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[0].measurement_control = "uncontrolled" | .[0].cpu_affinity = "uncontrolled" | .[0].background_load_note = "uncontrolled; MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1; waived affinity_unavailable" | .[]')" 'a run mixing controlled and uncontrolled records'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | .measurement_control = null | .cpu_affinity = null | .missing_metadata = ["cpu_affinity","measurement_control"]] | .[]')" 'a run that never stated its admissibility'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[0].workload_kind = "sixty_four_track_console" | .[0].tracks = 64 | .[0].synthetic_fixture = false | .[0].fixture_id = "fixtures/session/v1/console-sixty-four-track-intended.toml" | .[0].strip_content = "eq+compressor+limiter" | .[0].strip_layout = "simd1:eq+compressor,simd2:limiter" | .[]')" 'a set missing a declared workload'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_eq_only")] | .[]')" 'a set missing the eq-only decomposition row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_idle")] | .[]')" 'a set missing the idle row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_console_legacy")] | .[]')" 'a set missing the transition row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_eq_comp_simd1")] | .[]')" 'a set missing the chain-shape row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_compressor_automation")] | .[]')" 'a set missing the automation-active row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.record != "console_mono")] | .[]')" 'a set with no mono row-pair'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '.[45].collapse_eligible_output_sha256 = $c | .[45].collapse_forced_off_output_sha256 = $c | .[]')" 'a mono pair whose rounds rendered different output'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_plumbing_only")] | .[]')" 'a set missing the overhead floor row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_gain_pan_only")] | .[]')" 'a set missing the gain-and-pan row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_console_mono")] | .[]')" 'a set missing the mono session row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_console_mono_dual")] | .[]')" 'a set missing the mono control row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_console_half_mono")] | .[]')" 'a set missing the mixed-cohort row'

# ---------------------------------------------------------------------------------------------
# #184 at the aggregate: the isolate is a subtraction between two rows, so only a whole run has
# the two rows. The set below carries the floor columns on every session row, with a distinct cost
# per workload so that every named subtraction is a positive number the aggregate can recompute.
# ---------------------------------------------------------------------------------------------
floor_records=$(printf '%s' "$records" | jq -c -L "$scripts_dir" --arg s "$core_clock_source" \
    "$add_floor"'
  def scale: {
    "nine_track_baseline": 0.30, "nine_track_ragged_strip": 0.70,
    "sixty_four_track_builtins_only": 1.00, "sixty_four_track_dispatch_only": 1.02,
    "sixty_four_track_idle": 1.05, "sixty_four_track_eq_only": 1.60,
    "sixty_four_track_compressor_only": 2.60, "sixty_four_track_eq_comp_simd1": 3.20,
    "sixty_four_track_console_legacy": 3.30, "sixty_four_track_console": 4.40,
    "one_twenty_eight_track_stretch": 8.60,
    "sixty_four_track_plumbing_only": 0.42, "sixty_four_track_gain_pan_only": 1.01,
    "sixty_four_track_console_mono": 4.38, "sixty_four_track_console_mono_dual": 4.38,
    "sixty_four_track_console_half_mono": 4.39
  }[.workload_kind];
  def rescale:
    scale as $k |
    reduce ("min", "p50", "p95", "p99", "max") as $p (.;
      .[$p + "_ns_per_block"] = ((.[$p + "_ns_per_block"] * $k) | floor)
      | .[$p + "_us_per_block"] = (.[$p + "_ns_per_block"] / 1000))
    | .p50_us_per_block_per_track = (.p50_us_per_block / .tracks);
  [ .[] | if .record == "console_session" then rescale | with_floor(5480000000; $s) else . end ]
  | . as $all
  | ([ $all[] | select(.record == "console_session")
       | {key: (.workload_kind + ":" + (.round | tostring)), value: .} ] | from_entries) as $by
  | [ $all[]
      | if .record == "console_session" and .floor_control_row != "none" then
          ($by[.floor_control_row + ":" + (.round | tostring)]) as $control
          | .isolated_cycles_per_lane_sample =
              (.cycles_per_lane_sample - $control.cycles_per_lane_sample)
          | .isolated_percent_of_floor =
              (100 * (.floor_cycles_per_lane_sample - $control.floor_cycles_per_lane_sample)
                 / .isolated_cycles_per_lane_sample)
        else . end ]')

expect_aggregate_accept "$(printf '%s' "$floor_records" | jq -c '.[]')" 'the forty-six-record set with floor accounting'
expect_aggregate_reject "$(printf '%s' "$floor_records" | jq -c '(.[] | select(.workload_kind == "sixty_four_track_compressor_only")).isolated_cycles_per_lane_sample = 3.0 | .[]')" 'an isolate that is not the subtraction it names'
expect_aggregate_reject "$(printf '%s' "$floor_records" | jq -c '(.[] | select(.workload_kind == "sixty_four_track_compressor_only")).isolated_percent_of_floor = 88.0 | .[]')" 'an isolate percentage that does not follow from the two rows floors'
# The control row moving is the same defect seen from the other side: the subtraction stops being
# the subtraction the subtracted row published.
expect_aggregate_reject "$(printf '%s' "$floor_records" | jq -c '(.[] | select(.workload_kind == "sixty_four_track_builtins_only" and .round == 1)).cycles_per_lane_sample = 1.0 | .[]')" 'a control row whose cost moved under the rows that subtract it'
# A run cannot lose its counter half way through: with the columns on some rows and not others the
# two halves are not comparable and the aggregate refuses the run rather than the record.
expect_aggregate_reject "$(printf '%s' "$floor_records" | jq -c -L "$scripts_dir" 'include "console-benchmark-record-lib"; [.[] | if .workload_kind == "sixty_four_track_idle" then delpaths([floor_keys[] | [.]]) else . end] | .[]')" 'a run carrying cycle columns on some session rows only'
# Two clocks in one run is two hosts in one run, restated in hertz. The row is recomputed under the
# second clock so that every per-record rule still passes and only the aggregate rule bites.
expect_aggregate_reject "$(printf '%s' "$floor_records" | jq -c -L "$scripts_dir" --arg s "$core_clock_source" "$add_floor"'[.[] | if .workload_kind == "sixty_four_track_idle" then with_floor(4100000000; $s) else . end] | .[]')" 'a run measured under two core clocks'
expect_aggregate_reject "$(printf '%s' "$floor_records" | jq -c '[.[] | if .record == "console_session" then .core_clock_source = (.core_clock_source + .workload_kind) else . end] | .[]')" 'a run whose rows disagree about where their clock came from'

if [[ "$failures" != 0 ]]; then
    printf 'console benchmark validator suite: %s FAILED case(s)\n' "$failures" >&2
    exit 1
fi
printf 'console benchmark validators: PASS (real runner/workload/timing invocations: 0/0/0)\n'
