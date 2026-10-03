#!/usr/bin/env bash
# Console validator mutation suite. Hermetic: no workload, no timing, no binary.
#
# A validator that has never been shown to reject anything is decoration. Every rule below is
# mutated in turn and asserted red, so the aggregate's guarantees -- sixty-two records, both rounds,
# one host, one admissibility state, the decomposition rows' pinned strip contents, every session
# row's pinned source feed, the class-A statements that neither the stationary smoother nor a
# meter nor an armed observation tap nor a restated parameter nor the mono collapse nor the
# driver-fed source feed nor the metered row's web meters and fused fader and matrix changes a
# rendered bit, the meters pair's equal fold and redirect counters, the metered row's pinned
# meter group, the mixing-automation row's pinned controls, host lowerings, collapse counters
# and per-effect bit movement, and the console-strip rows' pinned layouts, bypass group and sparse
# input (#1085) -- are properties the suite can actually lose. The same holds for the
# mixing-automation row's browser arm (`web-mixing-automation-validator.jq`, #1011): its input
# feed, its provenance, and its two measured rounds' agreement -- and for its runner: a refused run
# keeps its rounds, no artifact is overwritten, and the provenance is checked after the rounds.
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
  fixture_id: "fixtures/session/v1/console-sixty-four-track-intended.json",
  round: 1, backend: "Simd8", sample_rate_hz: 48000, quantum_frames: 128, observations: 1000,
  units: "us_per_block", percentile_method: "nearest_rank",
  min_us_per_block: 281.9, p50_us_per_block: 283.4, p95_us_per_block: 285.4,
  p99_us_per_block: 295.7, max_us_per_block: 297.2, p50_us_per_block_per_track: 4.429,
  min_ns_per_block: 281915, p50_ns_per_block: 283459, p95_ns_per_block: 285482,
  p99_ns_per_block: 295702, max_ns_per_block: 297245,
  output_sha256: $a, render_errors: 0, render_total_forbidden_operations: 0,
  descriptive_only: true, strip_content: "eq+compressor+limiter",
  strip_layout: "pre_insert:eq+compressor,post_insert:limiter", input_signal: "tone", source_feed: "bound",
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
  meters_off_bank_route_folds: 64, meters_on_bank_route_folds: 64,
  meters_off_bank_scatter_redirects: 0, meters_on_bank_scatter_redirects: 0,
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
  split_chains_layout: "pre_insert:eq,inserts:compressor",
  merged_chain_layout: "pre_insert:eq+compressor",
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
  synthetic_fixture: true, strip_content: "compressor", strip_layout: "pre_insert:compressor",
  input_signal: "tone", fixture_id: "fixtures/session/v1/console-sixty-four-track-intended.json",
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
  fixture_id: "fixtures/session/v1/console-sixty-four-track-mono.json",
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
  arm_difference: "collapse_eligible takes the mono collapse on every cohort; collapse_forced_off renders the same fixture dual",
  render_errors: 0, render_total_forbidden_operations: 0,
  descriptive_only: true,
  statistical_method: "two arms alternated per observation; nearest-rank percentiles over per-block nanoseconds; paired delta is collapse_forced_off minus collapse_eligible per observation; descriptive only; no threshold"
}')

# The mixing-automation row (#1003). The mono console riding eight controls, pushed in the
# shapes a real host pushes them; its digests, its preflight and its collapse counters are the
# claims, and the numbers are the diagnosis's order of magnitude.
mixing_controls='[
  {track_id: "ch00", slot_id: "eq", effect: "miso.parametric-eq", parameter: "band-1-gain", parameter_index: 3, lowering: "owner_both", base: -7.5, step: 0.25, even_value: -7.25, odd_value: -7.75},
  {track_id: "ch08", slot_id: "comp", effect: "miso.compressor", parameter: "threshold", parameter_index: 0, lowering: "left_then_right", base: -18, step: 0.5, even_value: -17.5, odd_value: -18.5},
  {track_id: "ch16", slot_id: "limiter", effect: "miso.true-peak-limiter", parameter: "ceiling", parameter_index: 0, lowering: "left_then_right", base: -1, step: 8, even_value: 0, odd_value: -9},
  {track_id: "ch24", slot_id: "eq", effect: "miso.parametric-eq", parameter: "band-1-gain", parameter_index: 3, lowering: "owner_both", base: 1.5, step: 0.25, even_value: 1.75, odd_value: 1.25},
  {track_id: "ch32", slot_id: "comp", effect: "miso.compressor", parameter: "threshold", parameter_index: 0, lowering: "left_then_right", base: -27, step: 0.5, even_value: -26.5, odd_value: -27.5},
  {track_id: "ch40", slot_id: "limiter", effect: "miso.true-peak-limiter", parameter: "ceiling", parameter_index: 0, lowering: "left_then_right", base: -1.75, step: 8, even_value: 0, odd_value: -9.75},
  {track_id: "ch48", slot_id: "eq", effect: "miso.parametric-eq", parameter: "band-1-gain", parameter_index: 3, lowering: "owner_both", base: -4.5, step: 0.25, even_value: -4.25, odd_value: -4.75},
  {track_id: "ch56", slot_id: "comp", effect: "miso.compressor", parameter: "threshold", parameter_index: 0, lowering: "left_then_right", base: -9, step: 0.5, even_value: -8.5, odd_value: -9.5}
]'
mixing=$(jq -cn --arg a "$digest_a" --arg b "$digest_b" --arg c "$digest_c" --argjson m "$metadata" \
    "$mixing_controls"' as $controls | $m + {
  schema_version: 1, issue: 149, record: "console_mixing_automation",
  workload_kind: "sixty_four_track_console_mono_mixing_automation", tracks: 64,
  synthetic_fixture: false, strip_content: "eq+compressor+limiter",
  strip_layout: "pre_insert:eq+compressor,post_insert:limiter", input_signal: "tone",
  fixture_id: "fixtures/session/v1/console-sixty-four-track-mono.json",
  round: 1, backend: "Simd8", sample_rate_hz: 48000, quantum_frames: 128,
  observations: 1000, preroll_blocks: 64, pairing: "alternating_per_observation",
  arms: ["quiet","restated","automated"],
  automated_controls: $controls,
  owner_edits_per_block: 3, parameter_records_per_block: 10, smoothing_samples: 64,
  restated_pushes_accepted: 13000, automated_pushes_accepted: 13000,
  units: "ns_per_block", percentile_method: "nearest_rank",
  quiet_p50_ns: 69800, quiet_p95_ns: 71000, quiet_p99_ns: 73000,
  restated_p50_ns: 108300, restated_p95_ns: 110000, restated_p99_ns: 112000,
  automated_p50_ns: 148800, automated_p95_ns: 151000, automated_p99_ns: 154000,
  paired_ramp_delta_median_ns: 40400, paired_collapse_delta_median_ns: 38500,
  quiet_bank_collapse_counters: [8512, 8], restated_bank_collapse_counters: [3192, 8],
  automated_bank_collapse_counters: [3192, 8],
  quiet_output_sha256: $a, restated_output_sha256: $a, automated_output_sha256: $b,
  bit_identity: "quiet == restated, asserted in-run",
  preflight_blocks: 128,
  preflight_output_sha256: {quiet: $c, restated: $c, automated: $b, automated_eq_only: ($a[0:63] + "1"),
    automated_compressor_only: ($a[0:63] + "2"), automated_limiter_only: ($a[0:63] + "3"),
    restated_eq_only: $c},
  preflight_bank_collapse_counters: {quiet: [1024, 8], restated: [384, 8], automated: [384, 8],
    automated_eq_only: [1024, 8], automated_compressor_only: [640, 8],
    automated_limiter_only: [768, 8], restated_eq_only: [1024, 8]},
  render_errors: 0, render_total_forbidden_operations: 0,
  descriptive_only: true,
  statistical_method: "three arms alternated per observation; nearest-rank percentiles over per-block nanoseconds; ramp delta is automated minus restated and collapse delta is restated minus quiet, per observation; descriptive only; no threshold"
}')

# ---------------------------------------------------------------------------------------------
# The #184 floor group. A session record either carries all eleven columns or none of them.
# ---------------------------------------------------------------------------------------------
#
# The derived columns are computed here the way the subject computes them, through the validator
# library's own inventories rather than a third copy of the numbers -- the suite's job is to mutate
# a correct record, not to restate the derivation. `tools/bench/src/floor.rs` is the
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
      | .synthetic_fixture = false | .strip_content = "eq" | .strip_layout = "pre_insert:eq"
      | .fixture_id = "fixtures/session/v1/parametric-eq-nine-track.json"
      | with_floor(5480000000; $s)')

expect_accept "$session" 'the base session record'
expect_accept "$hoist" 'the base hoist record'
expect_accept "$meters" 'the base meters record'
expect_accept "$observation" 'the base observation record'
expect_accept "$placement" 'the base placement record'
expect_accept "$automation" 'the base automation record'
expect_accept "$mono" 'the base mono row-pair record'
expect_accept "$mixing" 'the base mixing-automation record'
# The identity row's own inventory. Since the prepared-identity elision the two rack-free rows do
# not share a floor -- `dispatch_only` elides both SVF sections rather than executing them -- so
# this row is the one that proves the split is enforced rather than merely written down.
session_floor_dispatch=$(printf '%s' "$session" | jq -c -L "$scripts_dir" --arg s "$core_clock_source" \
    "$add_floor"' .workload_kind = "sixty_four_track_dispatch_only" | .synthetic_fixture = true
      | .strip_content = "identity" | .strip_layout = "builtins"
      | with_floor(5480000000; $s)')

# The gain-and-pan row: the identity inventory again, the floor of the table since #956 retired
# the builtins-less plumbing row. Built through the library's own pins rather than by writing the
# numbers a second time.
session_floor_gain_pan=$(printf '%s' "$session" | jq -c -L "$scripts_dir" --arg s "$core_clock_source" \
    "$add_floor"' .workload_kind = "sixty_four_track_gain_pan_only" | .synthetic_fixture = true
      | .strip_content = "gain+pan" | .strip_layout = "builtins"
      | with_floor(5480000000; $s)')
# #928, re-based by #956: the driver-fed gain-and-pan row, the native pure-path target. The
# gain-and-pan row's six facts and its inventory, and the one fact that separates the two: its
# track inputs are claimed by a prepared source set whose driver lends its played planes, instead
# of being bound to processors.
session_ring=$(printf '%s' "$session" | jq -c '.workload_kind = "sixty_four_track_gain_pan_ring"
      | .synthetic_fixture = true | .strip_content = "gain+pan" | .strip_layout = "builtins"
      | .source_feed = "played_planes"')
session_floor_ring=$(printf '%s' "$session_ring" | jq -c -L "$scripts_dir" --arg s "$core_clock_source" \
    "$add_floor"' with_floor(5480000000; $s)')

expect_accept "$session_floor" 'the base session record carrying the floor columns'
expect_accept "$session_floor_gain_pan" 'the gain-and-pan row, which names no control'
expect_accept "$session_floor_not_derived" 'a row whose fixture was never inventoried'
expect_accept "$session_floor_dispatch" 'the identity row carrying the identity inventory'
expect_accept "$session_ring" 'the driver-fed gain-and-pan row'
expect_accept "$session_floor_ring" 'the driver-fed gain-and-pan row carrying the identity inventory'

# #881: the metered console row. The standing console row's six facts, plus the meter group that
# row alone carries: the default web boot's meter set (one sample-peak meter at the post-matrix tap
# per track, a twelve-block window), every snapshot it published over the thousand timed blocks
# (64 x floor(1000 / 12)), none dropped, and the plan's #914 counters. No inventory exists for a
# metered strip, so its floor group states none and names no control.
session_metered=$(printf '%s' "$session" | jq -c '.workload_kind = "sixty_four_track_console_metered"
      | . + {meter_streams: 64, meter_tap: "post_matrix", meter_metrics: "sample_peak",
             meter_window_blocks: 12, meter_snapshots: 5312, meter_dropped_snapshots: 0,
             bank_route_folds: 64, bank_scatter_redirects: 0}')
session_floor_metered=$(printf '%s' "$session_metered" | jq -c -L "$scripts_dir" --arg s "$core_clock_source" \
    "$add_floor"' with_floor(5480000000; $s)')

expect_accept "$session_metered" 'the metered console row'
expect_accept "$session_floor_metered" 'the metered console row carrying an underived floor'

# #1085: the app-shape row, the standing console's EQ and compressor on every track's `dynamic`
# rack with a third of the tracks bypassed, carries the bypass group its compiled session states;
# the sparse-activity row is the standing console with every odd track fed silence. Neither has an
# inventory, so each floor group states none.
app_fixture="fixtures/session/v1/console-sixty-four-track-app.json"
session_app=$(printf '%s' "$session" | jq -c --arg f "$app_fixture" \
    '.workload_kind = "sixty_four_track_app_shape" | .strip_content = "eq+compressor"
      | .strip_layout = "inserts:eq+compressor" | .fixture_id = $f
      | . + {bypass_pattern: "index_mod_3_is_2", bypassed_tracks: 21}')
session_floor_app=$(printf '%s' "$session_app" | jq -c -L "$scripts_dir" --arg s "$core_clock_source" \
    "$add_floor"' with_floor(5480000000; $s)')
session_sparse=$(printf '%s' "$session" | jq -c \
    '.workload_kind = "sixty_four_track_console_sparse" | .synthetic_fixture = true
      | .input_signal = "odd_tracks_silent"')
expect_accept "$session_app" 'the app-shape row'
expect_accept "$session_floor_app" 'the app-shape row carrying an underived floor'
expect_accept "$session_sparse" 'the sparse-activity row'

# #1227: the bus-and-send row, the standing console's tracks as written feeding buses and sends
# from its own committed fixture. It states the standing row's strip facts under its own kind and
# fixture.
sends_fixture="fixtures/session/v1/console-sixty-four-track-sends.json"
session_sends=$(printf '%s' "$session" | jq -c --arg f "$sends_fixture" \
    '.workload_kind = "sixty_four_track_console_sends" | .fixture_id = $f')
expect_accept "$session_sends" 'the bus-and-send row'
expect_reject "$(printf '%s' "$session_sends" | jq -c '.fixture_id = "fixtures/session/v1/console-sixty-four-track-intended.json"')" \
    'a bus-and-send row naming the standing fixture'
expect_reject "$(printf '%s' "$session_sends" | jq -c '.synthetic_fixture = true')" \
    'a bus-and-send row reported as derived in code'
expect_reject "$(printf '%s' "$session" | jq -c --arg f "$sends_fixture" '.fixture_id = $f')" \
    'a standing console row naming the bus-and-send fixture'

# ---------------------------------------------------------------------------------------------
# Per-key structural mutations: every key is load-bearing in both directions.
# ---------------------------------------------------------------------------------------------
for base in "$session" "$session_floor" "$session_ring" "$session_metered" "$session_app" \
    "$session_sparse" "$session_sends" "$hoist" "$meters" "$observation" "$placement" "$automation" "$mono" "$mixing"; do
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
# The metered row's floor group is underived too, so it takes the same half-sweep.
while read -r field; do
    expect_reject "$(printf '%s' "$session_floor_metered" | jq -c "del(.\"$field\")")" \
        "the floor-accounted metered row without $field"
    if [[ "$(printf '%s' "$session_floor_metered" | jq -c ".\"$field\"")" != null ]]; then
        expect_reject "$(printf '%s' "$session_floor_metered" | jq -c ".\"$field\" = null")" \
            "the floor-accounted metered row with $field nulled"
    fi
done < <(printf '%s' "$session_floor_metered" | jq -r 'keys[]')
# And so is the app shape's (#1085).
while read -r field; do
    expect_reject "$(printf '%s' "$session_floor_app" | jq -c "del(.\"$field\")")" \
        "the floor-accounted app-shape row without $field"
    if [[ "$(printf '%s' "$session_floor_app" | jq -c ".\"$field\"")" != null ]]; then
        expect_reject "$(printf '%s' "$session_floor_app" | jq -c ".\"$field\" = null")" \
            "the floor-accounted app-shape row with $field nulled"
    fi
done < <(printf '%s' "$session_floor_app" | jq -r 'keys[]')

# ---------------------------------------------------------------------------------------------
# Semantic mutations: each names the property it destroys.
# ---------------------------------------------------------------------------------------------
session_mutation() { expect_reject "$(printf '%s' "$session" | jq -c "$1")" "$2"; }
hoist_mutation() { expect_reject "$(printf '%s' "$hoist" | jq -c "$1")" "$2"; }

session_mutation '.issue = 38' 'a session record from another issue'
session_mutation '.schema_version = 0' 'an invalid schema version'
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
session_mutation '.fixture_id = "fixtures/session/v1/canonical.json"' 'a record naming another fixture'
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
# #368: the pre-recount whole-strip floor (352 lane-ops) must not survive as a self-consistent
# floor/percentage pair after compressor and limiter repricing.
session_floor_mutation '.floor_cycles_per_lane_sample = (352 / (8 * 3.7))
  | .percent_of_floor = (100 * .floor_cycles_per_lane_sample / .cycles_per_lane_sample)' \
  'the stale pre-recount whole-strip inventory'
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
# The gain-and-pan row is costed at the identity inventory, the floor of the whole table (#956).
# Costed at the retired four-lane-op routing inventory, or citing it, is the same defect as the
# identity row costed at 69 -- self-consistent, floor and percentage together, and wrong about which
# arithmetic it executes.
expect_reject "$(printf '%s' "$session_floor_gain_pan" | jq -c \
    '.floor_cycles_per_lane_sample = (4 / (8 * 3.7))
     | .percent_of_floor = (100 * (4 / (8 * 3.7)) / .cycles_per_lane_sample)')" \
    'a gain-and-pan row costed at the retired routing-only inventory'
expect_reject "$(printf '%s' "$session_floor_gain_pan" | jq -c \
    '.floor_basis = "docs/rulings/effect-floor-accounting.md: plumbing"')" \
    'a gain-and-pan row citing the retired plumbing inventory'
expect_reject "$(printf '%s' "$session_floor_gain_pan" | jq -c \
    '.floor_basis = "docs/rulings/effect-floor-accounting.md: builtins"')" \
    'a gain-and-pan row citing the executed-filter inventory'
# It claims no isolate. `builtins_only` against it would subtract cleanly (69 - 22 = 47) but is not
# declared, so a record claiming it is claiming a control the table does not name.
expect_reject "$(printf '%s' "$session_floor_gain_pan" | jq -c \
    '.floor_control_row = "sixty_four_track_builtins_only" | .isolated_cycles_per_lane_sample = 0.5 | .isolated_percent_of_floor = 20.0')" \
    'a gain-and-pan row isolated against a row it is not a subset of'
expect_reject "$(printf '%s' "$session_floor_gain_pan" | jq -c \
    '.floor_control_row = "sixty_four_track_dispatch_only" | .isolated_cycles_per_lane_sample = 0.1 | .isolated_percent_of_floor = 1.0')" \
    'a gain-and-pan row isolated against the row that shares its inventory'
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
# #956 retired the builtins-less rows: a plan no host compiles is not a row, however honestly it
# states its facts, and neither is the `plumbing` word its content and layout were stated in.
session_mutation '.workload_kind = "sixty_four_track_plumbing_only" | .strip_content = "plumbing" | .strip_layout = "plumbing" | .synthetic_fixture = true' \
    'the retired builtins-less plumbing row'
session_mutation '.workload_kind = "sixty_four_track_plumbing_ring" | .strip_content = "plumbing" | .strip_layout = "plumbing" | .synthetic_fixture = true | .source_feed = "played_planes"' \
    'the retired builtins-less driver-fed plumbing row'
session_mutation '.workload_kind = "sixty_four_track_builtins_only" | .strip_content = "plumbing" | .strip_layout = "plumbing" | .synthetic_fixture = true' \
    'a builtins row claiming it prepared no builtin'
expect_accept "$(printf '%s' "$session" | jq -c '.workload_kind = "sixty_four_track_gain_pan_only" | .strip_content = "gain+pan" | .strip_layout = "builtins" | .synthetic_fixture = true')" \
    'an honest gain-and-pan row'

# #928: the source feed, required on every session record and pinned per kind. Exactly one row is
# driver-fed; a record naming the wrong feed would charge the feed's cost to the wrong row.
expect_reject "$(printf '%s' "$session" | jq -c 'del(.source_feed)')" \
    'a session record from before source_feed existed'
expect_reject "$(printf '%s' "$session_floor" | jq -c 'del(.source_feed)')" \
    'a floor-accounted session record missing source_feed'
session_mutation '.source_feed = "played_planes"' 'the standing console row claiming the driver-fed feed'
session_mutation '.source_feed = "ring"' 'a feed no workload declares'
session_mutation '.workload_kind = "sixty_four_track_gain_pan_only" | .strip_content = "gain+pan" | .strip_layout = "builtins" | .synthetic_fixture = true | .source_feed = "played_planes"' \
    'the bound gain-and-pan row claiming the driver-fed feed'
ring_mutation() { expect_reject "$(printf '%s' "$session_ring" | jq -c "$1")" "$2"; }
ring_mutation '.source_feed = "bound"' 'the driver-fed row claiming the bound feed'
ring_mutation 'del(.source_feed)' 'the driver-fed row missing source_feed'
ring_mutation '.strip_layout = "plumbing"' 'a driver-fed row claiming the retired plumbing layout'
ring_mutation '.strip_content = "identity"' 'a driver-fed row claiming the identity fader and pan'
ring_mutation '.synthetic_fixture = false' 'a derived driver-fed row reported as a checked-in fixture'
ring_mutation '.tracks = 9' 'a driver-fed row that is not eight full banks'
ring_mutation '.fixture_id = "fixtures/session/v1/console-sixty-four-track-mono.json"' \
    'a driver-fed row rendered from another fixture'
ring_mutation '.input_signal = "silence"' 'a driver-fed row claiming silence'
expect_reject "$(printf '%s' "$session_floor_ring" | jq -c '.source_feed = "bound"')" \
    'a floor-accounted driver-fed row claiming the bound feed'
# Its floor is the gain-and-pan row's, and like that row it isolates nothing. A feed change is not
# an arithmetic change, so a subtraction between the two feeds would publish a copy's cost as an
# isolate against a floor with no copy in it.
expect_reject "$(printf '%s' "$session_floor_ring" | jq -c \
    '.floor_cycles_per_lane_sample = (4 / (8 * 3.7))
     | .percent_of_floor = (100 * (4 / (8 * 3.7)) / .cycles_per_lane_sample)')" \
    'a driver-fed gain-and-pan row costed at the retired routing-only inventory'
expect_reject "$(printf '%s' "$session_floor_ring" | jq -c \
    '.floor_basis = "docs/rulings/effect-floor-accounting.md: plumbing"')" \
    'a driver-fed gain-and-pan row citing the retired plumbing inventory'
expect_reject "$(printf '%s' "$session_floor_ring" | jq -c \
    '.floor_control_row = "sixty_four_track_gain_pan_only" | .isolated_cycles_per_lane_sample = 0.1 | .isolated_percent_of_floor = 1.0')" \
    'the driver-fed row isolating its feed against the bound row'
expect_reject "$(printf '%s' "$session_floor_gain_pan" | jq -c \
    '.floor_control_row = "sixty_four_track_gain_pan_ring" | .isolated_cycles_per_lane_sample = 0.1 | .isolated_percent_of_floor = 1.0')" \
    'the bound gain-and-pan row isolated against the driver-fed row'

# #881: the metered console row. The meter group is carried by that row alone and is pinned field
# by field: a row that metered another tap, another metric set or another window would publish a
# cost the browser does not pay, and a row that consumed fewer snapshots than its windows closed,
# or dropped one, did not exercise the observer path it claims to time.
metered_mutation() { expect_reject "$(printf '%s' "$session_metered" | jq -c "$1")" "$2"; }
metered_group='. + {meter_streams: 64, meter_tap: "post_matrix", meter_metrics: "sample_peak",
  meter_window_blocks: 12, meter_snapshots: 5312, meter_dropped_snapshots: 0,
  bank_route_folds: 64, bank_scatter_redirects: 0}'
session_mutation "$metered_group" 'the standing console row carrying the meter group'
session_mutation '.workload_kind = "sixty_four_track_console_metered"' \
    'a metered row without its meter group'
expect_reject "$(printf '%s' "$session_floor" | jq -c "$metered_group")" \
    'a floor-accounted standing row carrying the meter group'
metered_mutation '.workload_kind = "sixty_four_track_console"' 'the meter group on the standing row'
metered_mutation '.workload_kind = "sixty_four_track_gain_pan_only" | .strip_content = "gain+pan" | .strip_layout = "builtins" | .synthetic_fixture = true' \
    'the meter group on another row'
metered_mutation '.meter_streams = 32' 'a metered row that metered half its tracks'
metered_mutation '.meter_tap = "post_fader"' 'a metered row at a tap the browser does not meter'
metered_mutation '.meter_metrics = "all"' 'a metered row computing every statistic'
# The window mutation carries the snapshot count its window implies, so the window pin alone
# refuses it; the count rule refuses the inconsistent pairs below.
metered_mutation '.meter_window_blocks = 4 | .meter_snapshots = 16000' 'a metered row at the facility arm window'
metered_mutation '.meter_window_blocks = 0' 'a meter window of no blocks'
metered_mutation '.meter_metrics = "other"' 'a metered row whose snapshots disagreed about their metric set'
metered_mutation '.meter_tap = "other"' 'a metered row whose streams observe more than one tap'
metered_mutation '.meter_snapshots = 0' 'a metered row that consumed no snapshot'
metered_mutation '.meter_snapshots = 5311' 'a metered row that lost a snapshot'
metered_mutation '.meter_snapshots = 5313' 'a metered row that consumed a snapshot no window closed'
metered_mutation '.meter_snapshots = 16000' 'a metered row counting four-block windows'
metered_mutation '.meter_dropped_snapshots = 1' 'a metered row whose stream dropped a snapshot'
metered_mutation '.meter_dropped_snapshots = -1' 'a negative drop count'
metered_mutation '.bank_route_folds = 0' 'a metered plan whose route fold declined'
metered_mutation '.bank_route_folds = 65' 'a metered plan folding more routes than it has tracks'
metered_mutation '.bank_scatter_redirects = -1' 'a negative redirect count'
metered_mutation '.bank_scatter_redirects = 0.5' 'a fractional redirect count'
metered_mutation '.bank_scatter_redirects = "0"' 'a redirect count carried as a string'
metered_mutation '.synthetic_fixture = true' 'a metered row reported as a derived session'
metered_mutation '.strip_content = "eq+compressor+limiter+meter"' 'a metered row counting its meters as strip content'
metered_mutation '.strip_layout = "builtins"' 'a metered row claiming the builtins layout'
metered_mutation '.input_signal = "silence"' 'a metered row claiming silence'
metered_mutation '.fixture_id = "fixtures/session/v1/console-sixty-four-track-mono.json"' \
    'a metered row rendered from the mono fixture'
metered_mutation '.source_feed = "played_planes"' 'a metered row claiming the driver-fed feed'
metered_mutation '.tracks = 9' 'a metered row that is not eight full banks'
metered_mutation '.observations = 100' 'a shortened metered run'
# The redirect count is the metered plan's own, whatever it is: any non-negative integer.
expect_accept "$(printf '%s' "$session_metered" | jq -c '.bank_scatter_redirects = 8')" \
    'a metered row redirecting lanes'
# Its floor group states none: no ruling has inventoried a metered strip, and the standing strip's
# inventory would publish the meters' cost as a gap in the strip's.
expect_reject "$(printf '%s' "$session_floor_metered" | jq -c \
    '.floor_cycles_per_lane_sample = (307 / (8 * 3.7))
     | .percent_of_floor = (100 * (307 / (8 * 3.7)) / .cycles_per_lane_sample)')" \
    'a metered row costed at the unmetered strip inventory'
expect_reject "$(printf '%s' "$session_floor_metered" | jq -c \
    '.floor_basis = "docs/rulings/effect-floor-accounting.md: builtins+eq+compressor+limiter"')" \
    'a metered row citing the unmetered strip inventory'
expect_reject "$(printf '%s' "$session_floor_metered" | jq -c \
    '.floor_control_row = "sixty_four_track_console" | .isolated_cycles_per_lane_sample = 0.1 | .isolated_percent_of_floor = 1.0')" \
    'a metered row isolating its meters against the standing row'

# The mono session rows. Both arms render the mono fixture as written, so both are checked-in
# rather than derived; the half-mono row is the one that is derived, and it is derived from the
# same file. A mono row pointed at the standing fixture would be the standing console row wearing
# the name the collapse gate is going to be read off.
mono_fixture="fixtures/session/v1/console-sixty-four-track-mono.json"
session_mutation ".workload_kind = \"sixty_four_track_console_mono\" | .fixture_id = \"fixtures/session/v1/console-sixty-four-track-intended.json\"" \
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
session_mutation '.strip_layout = "pre_insert:eq+compressor"' \
    'a standing console row claiming the limiter-free layout'
session_mutation '.strip_layout = "pre_insert:eq,inserts:compressor"' \
    'a standing console row claiming the retired layout'
session_mutation '.strip_layout = "post_insert:eq+compressor,pre_insert:limiter"' \
    'a layout naming racks in an order the strip does not run'
session_mutation '.strip_layout = "inserts:eq+compressor+limiter"' \
    'a layout no workload declares'
# The transition row and the chain-shape row: identical `strip_content`, different everything else
# that matters. Each must reject the other's identity.
session_mutation '.workload_kind = "sixty_four_track_console_legacy" | .strip_content = "eq+compressor" | .strip_layout = "pre_insert:eq+compressor"' \
    'a legacy row claiming the merged chain shape'
session_mutation '.workload_kind = "sixty_four_track_console_legacy" | .strip_content = "eq+compressor" | .strip_layout = "pre_insert:eq,inserts:compressor"' \
    'a legacy row rendered from the standing fixture'
session_mutation '.workload_kind = "sixty_four_track_eq_comp_simd1" | .strip_content = "eq+compressor" | .strip_layout = "pre_insert:eq,inserts:compressor" | .synthetic_fixture = true' \
    'a chain-shape row claiming the retired layout'
session_mutation '.workload_kind = "sixty_four_track_eq_comp_simd1" | .strip_content = "eq+compressor" | .strip_layout = "pre_insert:eq+compressor" | .synthetic_fixture = false' \
    'a derived chain-shape row reported as a checked-in fixture'
expect_accept "$(printf '%s' "$session" | jq -c --arg f "fixtures/session/v1/console-sixty-four-track.json" '.workload_kind = "sixty_four_track_console_legacy" | .strip_content = "eq+compressor" | .strip_layout = "pre_insert:eq,inserts:compressor" | .fixture_id = $f')" \
    'an honest transition row'
expect_accept "$(printf '%s' "$session" | jq -c '.workload_kind = "sixty_four_track_eq_comp_simd1" | .strip_content = "eq+compressor" | .strip_layout = "pre_insert:eq+compressor" | .synthetic_fixture = true')" \
    'an honest chain-shape row'
session_mutation '.input_signal = "noise"' 'an input signal no workload declares'
# Accept the honest forms, so the pins above are shown to be pins and not a blanket refusal.
expect_accept "$(printf '%s' "$session" | jq -c '.workload_kind = "sixty_four_track_eq_only" | .strip_content = "eq" | .strip_layout = "pre_insert:eq" | .synthetic_fixture = true')" \
    'an honest eq-only row'
expect_accept "$(printf '%s' "$session" | jq -c '.workload_kind = "sixty_four_track_idle" | .synthetic_fixture = true | .input_signal = "silence"')" \
    'an honest idle row'

# #1085: `strip_layout` is named in decision 12's console vocabulary (`pre_insert`, `inserts`,
# `post_insert`), so one spelling pins a row before and after the console strip. The rack-token
# spellings every row carried before are refused, so a subject that kept printing them, or a
# validator that kept accepting them beside the new ones, is caught.
session_mutation '.strip_layout = "simd1:eq+compressor,simd2:limiter"' \
    'a standing console row in the retired rack-token spelling'
session_mutation '.workload_kind = "sixty_four_track_console_legacy" | .strip_content = "eq+compressor" | .strip_layout = "simd1:eq,dynamic:compressor" | .fixture_id = "fixtures/session/v1/console-sixty-four-track.json"' \
    'a transition row in the retired rack-token spelling'
session_mutation '.workload_kind = "sixty_four_track_eq_only" | .strip_content = "eq" | .strip_layout = "simd1:eq" | .synthetic_fixture = true' \
    'an eq-only row in the retired rack-token spelling'

# #1085: the console-strip rows. The strip at ten, thirteen and sixteen tracks, the app shape and
# sparse activity, each pinned on the facts it states, on the same terms as every other row.
# The one row whose layout the console migration moves: its EQ and compressor read `inserts` on
# today's racks and `pre_insert` once the app puts them in the session console. Both are that row's
# own, and neither is any other row's.
expect_accept "$(printf '%s' "$session_app" | jq -c '.strip_layout = "pre_insert:eq+compressor"')" \
    'the app-shape row after the console migration'
for count in 10 13 16; do
    kind=$(jq -rn --argjson n "$count" '{"10": "ten_track_ragged_strip", "13": "thirteen_track_ragged_strip", "16": "sixteen_track_strip"}[$n | tostring]')
    strip_n=$(printf '%s' "$session" | jq -c --arg k "$kind" --argjson n "$count" \
        '.workload_kind = $k | .tracks = $n | .synthetic_fixture = true')
    expect_accept "$strip_n" "an honest $kind row"
    expect_accept "$(printf '%s' "$strip_n" | jq -c -L "$scripts_dir" --arg s "$core_clock_source" "$add_floor"' with_floor(5480000000; $s)')" \
        "an honest $kind row carrying an underived floor"
    strip_mutation() { expect_reject "$(printf '%s' "$strip_n" | jq -c "$1")" "$kind: $2"; }
    strip_mutation '.tracks = 9' 'a remainder row at another track count'
    strip_mutation '.tracks = 64' 'a remainder row claiming the full console'
    strip_mutation '.synthetic_fixture = false' 'a truncated fixture reported as checked in'
    strip_mutation '.strip_layout = "pre_insert:eq+compressor"' 'a strip-at-N row without its limiter'
    strip_mutation '.strip_content = "eq+compressor"' 'a strip-at-N row claiming no limiter'
    strip_mutation '.input_signal = "odd_tracks_silent"' 'a strip-at-N row claiming sparse input'
    strip_mutation '.fixture_id = "fixtures/session/v1/console-sixty-four-track-mono.json"' \
        'a strip-at-N row cut from the mono fixture'
    strip_mutation '.source_feed = "played_planes"' 'a strip-at-N row claiming the driver-fed feed'
    strip_mutation '. + {bypass_pattern: "none", bypassed_tracks: 0}' 'a strip-at-N row carrying the bypass group'
    expect_reject "$(printf '%s' "$strip_n" | jq -c -L "$scripts_dir" --arg s "$core_clock_source" "$add_floor"' with_floor(5480000000; $s)
        | .floor_cycles_per_lane_sample = (307 / (8 * 3.7))
        | .percent_of_floor = (100 * (307 / (8 * 3.7)) / .cycles_per_lane_sample)')" \
        "$kind costed at the full-bank strip inventory"
    expect_reject "$(printf '%s' "$strip_n" | jq -c -L "$scripts_dir" --arg s "$core_clock_source" "$add_floor"' with_floor(5480000000; $s)
        | .floor_basis = "docs/rulings/effect-floor-accounting.md: builtins+eq+compressor+limiter, ragged"')" \
        "$kind citing the nine-track ragged inventory"
done
session_mutation '.workload_kind = "sixteen_track_strip" | .synthetic_fixture = true' \
    'a sixteen-track row claiming sixty-four tracks'
app_mutation() { expect_reject "$(printf '%s' "$session_app" | jq -c "$1")" "$2"; }
app_mutation 'del(.bypass_pattern, .bypassed_tracks)' 'an app-shape row without its bypass group'
app_mutation 'del(.bypassed_tracks)' 'an app-shape row missing its bypassed-track count'
app_mutation '.bypass_pattern = "other"' 'an app-shape row whose bypass is not the app pattern'
app_mutation '.bypass_pattern = "none"' 'an app-shape row that bypassed nothing'
app_mutation '.bypass_pattern = "index_mod_2_is_1"' 'an app-shape row naming another pattern'
app_mutation '.bypassed_tracks = 20' 'an app-shape row that bypassed one track too few'
app_mutation '.bypassed_tracks = 22' 'an app-shape row that bypassed one track too many'
app_mutation '.bypassed_tracks = 0' 'an app-shape row counting no bypassed track'
app_mutation '.bypassed_tracks = "21"' 'a bypassed-track count carried as a string'
app_mutation '.strip_layout = "pre_insert:eq+compressor,post_insert:limiter"' 'an app-shape row claiming the limiter'
app_mutation '.strip_layout = "pre_insert:eq,inserts:compressor"' 'an app-shape row claiming the retired split layout'
app_mutation '.strip_layout = "dynamic:eq+compressor"' 'an app-shape row in the retired rack-token spelling'
app_mutation '.strip_content = "eq+compressor+limiter"' 'an app-shape row claiming the limiter content'
app_mutation '.fixture_id = "fixtures/session/v1/console-sixty-four-track-intended.json"' \
    'an app-shape row rendered from the standing fixture'
app_mutation '.synthetic_fixture = true' 'the committed app fixture reported as derived'
app_mutation '.input_signal = "odd_tracks_silent"' 'an app-shape row claiming sparse input'
app_mutation '.tracks = 16' 'an app-shape row that is not the sixty-four-track console'
expect_reject "$(printf '%s' "$session_floor_app" | jq -c \
    '.floor_cycles_per_lane_sample = ((69 + 27 + 81.5) / (8 * 3.7))
     | .percent_of_floor = (100 * ((69 + 27 + 81.5) / (8 * 3.7)) / .cycles_per_lane_sample)')" \
    'an app-shape row costed as though no lane were bypassed'
# The group is the app shape's alone, and the app layout is too.
session_mutation '. + {bypass_pattern: "index_mod_3_is_2", bypassed_tracks: 21}' \
    'the standing console row carrying the bypass group'
session_mutation '.strip_layout = "inserts:eq+compressor"' 'the standing console row claiming the app layout'
expect_reject "$(printf '%s' "$session_metered" | jq -c '. + {bypass_pattern: "index_mod_3_is_2", bypassed_tracks: 21}')" \
    'the metered row carrying the bypass group'
expect_reject "$(printf '%s' "$session" | jq -c '.workload_kind = "sixty_four_track_eq_comp_simd1" | .strip_content = "eq+compressor" | .strip_layout = "inserts:eq+compressor" | .synthetic_fixture = true')" \
    'the chain-shape row claiming the app layout'
sparse_mutation() { expect_reject "$(printf '%s' "$session_sparse" | jq -c "$1")" "$2"; }
sparse_mutation '.input_signal = "tone"' 'a sparse row claiming every track active'
sparse_mutation '.input_signal = "silence"' 'a sparse row claiming every track silent'
sparse_mutation '.synthetic_fixture = false' 'a sparse row reported as the fixture rendered as written'
sparse_mutation '.strip_layout = "inserts:eq+compressor"' 'a sparse row claiming the app layout'
sparse_mutation '.fixture_id = "fixtures/session/v1/console-sixty-four-track-app.json"' \
    'a sparse row rendered from the app fixture'
sparse_mutation '.tracks = 9' 'a sparse row that is not the sixty-four-track console'
session_mutation '.input_signal = "odd_tracks_silent"' 'the standing console row claiming sparse input'
expect_reject "$(printf '%s' "$session" | jq -c '.workload_kind = "sixty_four_track_idle" | .synthetic_fixture = true | .input_signal = "odd_tracks_silent"')" \
    'the idle row claiming sparse input'

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
# Issue #914: each arm's fold and redirect counters. Both optimisations render the bits of the path
# they replace, so only these counts can say the metered arm still folds -- and a metered arm that
# stopped folding would still pass the digest check above and publish the fold's cost as the meters'.
meters_mutation 'del(.meters_off_bank_route_folds, .meters_on_bank_route_folds, .meters_off_bank_scatter_redirects, .meters_on_bank_scatter_redirects)' \
    'a meters record without its fold and redirect counters (the shape before #914)'
meters_mutation 'del(.meters_on_bank_route_folds)' 'a meters record missing the metered arm fold count'
meters_mutation '.meters_on_bank_route_folds = 0' 'a metered arm whose route fold declined'
meters_mutation '.meters_on_bank_route_folds = 65' 'a metered arm folding more routes than the unmetered arm'
meters_mutation '.meters_on_bank_scatter_redirects = 8' 'a metered arm whose scatter redirects differ from the unmetered arm'
meters_mutation '.meters_off_bank_route_folds = 32 | .meters_on_bank_route_folds = 32' \
    'two arms that agree but do not fold every route of the console'
# The redirect pair is pinned equal but not to a value, so these three are the type rule's alone:
# both arms agree, and only "a count is a non-negative integer" is left to refuse them.
meters_mutation '.meters_off_bank_scatter_redirects = -1 | .meters_on_bank_scatter_redirects = -1' \
    'a negative redirect count'
meters_mutation '.meters_off_bank_scatter_redirects = 0.5 | .meters_on_bank_scatter_redirects = 0.5' \
    'a fractional redirect count'
meters_mutation '.meters_off_bank_scatter_redirects = "0" | .meters_on_bank_scatter_redirects = "0"' \
    'a redirect count written as a string'
# The redirect count is pinned equal across the arms, not to a value: a plan that both folds every
# route and redirects the same lanes in both arms is a different plan shape, not a dishonest record.
expect_accept "$(printf '%s' "$meters" | jq -c '.meters_off_bank_scatter_redirects = 8 | .meters_on_bank_scatter_redirects = 8')" \
    'a meters pair whose arms redirect the same lanes'

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
placement_mutation '.split_chains_layout = "pre_insert:eq+compressor"' 'a pair whose two arms name one layout'
placement_mutation '.merged_chain_layout = "pre_insert:eq,inserts:compressor"' 'a merged arm claiming the split layout'
placement_mutation '.split_chains_layout = "pre_insert:eq+compressor,post_insert:limiter"' 'a split arm carrying the limiter'
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
automation_mutation '.strip_layout = "pre_insert:eq+compressor,post_insert:limiter"' 'an automation row claiming the intended layout'
automation_mutation '.synthetic_fixture = false' 'a derived automation row claiming a checked-in fixture'
automation_mutation '.input_signal = "silence"' 'an automation row claiming silence while rendering a tone'
automation_mutation '.tracks = 9' 'an automation row that is not eight full banks'
automation_mutation '.fixture_id = "fixtures/session/v1/console-sixty-four-track.json"' \
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
mono_mutation '.fixture_id = "fixtures/session/v1/console-sixty-four-track-intended.json"' \
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
# Issue #911: `symmetric_lanes` counts the bank chains' track lanes, not the whole census's eligible
# half. The census also carries the `main-out` identity's vacuous witness, which names no track, so
# a record that reported the census half (65 on this fixture since #221) is counting a non-track
# unit as a track lane.
mono_mutation '.symmetric_lanes = 65' 'a mono pair counting a non-track witness as a track lane'
mono_mutation '.lanes = 64' 'a lane census that counts only the lanes it calls symmetric'
# The honesty field. A near-zero delta with this sentence removed reads as a measured saving.
mono_mutation '.arm_difference = "none: both arms are the mono fixture as written; no collapse exists in this tree"' \
    'a mono record claiming this tree has no collapse'
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

# ---------------------------------------------------------------------------------------------
# The mixing-automation row (#1003). Every claim it makes is mutated: the controls and the host
# lowering each one is pushed in, the push counts, the collapse counters, and the digests of the
# run and of the preflight.
# ---------------------------------------------------------------------------------------------
mixing_mutation() { expect_reject "$(printf '%s' "$mixing" | jq -c "$1")" "$2"; }

mixing_mutation '.arms = ["quiet","automated","restated"]' 'mixing arms in the wrong order'
mixing_mutation '.arms = ["quiet","restated"]' 'a mixing row missing its automated arm'
mixing_mutation '.pairing = "sequential"' 'a mixing row that was not alternated'
mixing_mutation '.record = "console_automation"' 'a mixing record claiming the one-track automation shape'
mixing_mutation '.workload_kind = "sixty_four_track_console_mono"' 'a mixing row claiming the quiet session kind'
mixing_mutation '.fixture_id = "fixtures/session/v1/console-sixty-four-track-intended.json"' \
    'a mixing row measured on the stereo console, which cannot collapse'
mixing_mutation '.synthetic_fixture = true' 'a mixing row claiming a derived fixture'
mixing_mutation '.strip_content = "compressor"' 'a mixing row claiming a decomposition strip'
mixing_mutation '.tracks = 9' 'a mixing row that is not eight full banks'
mixing_mutation '.units = "us_per_block"' 'the wrong mixing unit'
mixing_mutation '.statistical_method = "three arms alternated per observation"' \
    'a mixing method sentence that drifted'
mixing_mutation '.preroll_blocks = 0' 'a mixing row with no pre-roll'
# The controls. VERIFY-AUTOMATION F2: the EQ pushed as two one-channel edits retires a collapse the
# product keeps, and F3: the drafted row automated values nobody held.
mixing_mutation '.automated_controls[0].lowering = "left_then_right"' \
    'an EQ pushed as two one-channel edits, a shape the SDK never sends'
# The same record with every count made consistent with the drifted lowering, so only the pin on
# the lowering itself can refuse it.
mixing_mutation '.automated_controls[0].lowering = "left_then_right" | .owner_edits_per_block = 2 | .parameter_records_per_block = 12 | .restated_pushes_accepted = 14000 | .automated_pushes_accepted = 14000' \
    'an EQ pushed as two one-channel edits, its counts made consistent'
mixing_mutation '.automated_controls[1].lowering = "owner_both"' \
    'a compressor pushed as a Both record the web host does not send'
mixing_mutation '.automated_controls[2].step = 0.25' 'a limiter step that never engages (F6)'
mixing_mutation '.automated_controls[0].track_id = "ch01"' 'two automated tracks in one bank'
mixing_mutation '.automated_controls[3].parameter_index = 2' 'an EQ control naming another parameter'
mixing_mutation '.automated_controls[4].effect = "miso.true-peak-limiter"' 'a control on the wrong effect'
mixing_mutation '.automated_controls |= .[0:7]' 'seven automated controls'
mixing_mutation '.automated_controls |= reverse' 'the controls out of track order'
mixing_mutation '.automated_controls[5].even_value = -1.75' 'a ride whose even value is the base: no window'
mixing_mutation '.automated_controls[6].odd_value = -4.25' 'a ride that does not straddle its base'
mixing_mutation '.automated_controls[7].base = "held"' 'a base that is not a number'
mixing_mutation '.owner_edits_per_block = 8' 'owner edits that are not the EQ controls'
mixing_mutation '.parameter_records_per_block = 5' 'one record per compressor and limiter control'
mixing_mutation '.smoothing_samples = 32' 'a window that is not the descriptors'"'"''
mixing_mutation '.restated_pushes_accepted = 12999 | .automated_pushes_accepted = 12999' \
    'both pushing arms short of one accepted push'
mixing_mutation '.restated_pushes_accepted = 12999' 'a restated arm whose queue refused a push'
mixing_mutation '.automated_pushes_accepted = 8000' 'an automated arm counting pushes as sixteen spans'
# The collapse counters: read, not pinned, for the restated and automated arms -- but the quiet arm
# must collapse every cohort on every block, and no arm more than quiet.
mixing_mutation '.quiet_bank_collapse_counters = [8504, 8]' 'a quiet arm that lost a cohort-block'
mixing_mutation '.restated_bank_collapse_counters = [9000, 8]' 'a restated arm collapsing more than quiet'
mixing_mutation '.automated_bank_collapse_counters = [3192, 16]' 'an arm counting another cohort set'
mixing_mutation '.quiet_bank_collapse_counters = [0, 0]' 'a mono console with no cohort'
mixing_mutation 'del(.restated_bank_collapse_counters)' 'a record missing its collapse counters'
# The digests, in-run and in the preflight.
mixing_mutation '.restated_output_sha256 = "'"$digest_c"'"' 'restating the held values moved a bit'
mixing_mutation '.automated_output_sha256 = "'"$digest_a"'"' 'an automated arm that rendered the restated bits'
mixing_mutation '.bit_identity = "quiet != restated, asserted in-run"' 'a mixing row that inverted its class-A sentence'
mixing_mutation '.preflight_output_sha256.automated_limiter_only = "'"$digest_c"'"' \
    'a limiter ride that moved no bit (F6)'
mixing_mutation '.preflight_output_sha256.automated_eq_only = "'"$digest_c"'"' 'an EQ ride that moved no bit'
mixing_mutation '.preflight_output_sha256.automated_compressor_only = "'"$digest_c"'"' \
    'a compressor ride that moved no bit'
mixing_mutation '.preflight_output_sha256.quiet = "'"$digest_a"'"' 'a preflight restatement that moved a bit'
mixing_mutation '.preflight_output_sha256.restated_eq_only = "'"$digest_a"'"' \
    'a preflight EQ restatement that moved a bit'
mixing_mutation 'del(.preflight_output_sha256.restated_eq_only)' 'a preflight missing an arm'
mixing_mutation '.preflight_bank_collapse_counters.restated_eq_only = [640, 8]' \
    'an EQ restatement that retired a collapse: the harness drifted off the SDK shape (A6)'
mixing_mutation '.preflight_bank_collapse_counters.quiet = [1016, 8]' 'a preflight quiet arm that lost a cohort-block'
mixing_mutation '.preflight_blocks = 64' 'a preflight that compared no block after its pre-roll'
mixing_mutation '.quiet_p99_ns = 1' 'mixing percentiles out of order'
mixing_mutation '.render_errors = 1' 'a mixing row that produced render errors'
mixing_mutation '.render_total_forbidden_operations = 1' 'a mixing row that allocated on the render path'
# #1011: one mutation per rule that no mutation above isolates. Each changes the field its rule
# guards and keeps every other rule satisfied, so each one is refused by that rule alone and the
# rule cannot be deleted with the suite staying green. The mutations above that touch these
# fields are refused first by a neighbouring rule: a preflight quiet arm that lost a cohort-block
# by A6's equality, a preflight of 64 blocks by the quiet-counter rule.
mixing_mutation '.preflight_blocks = 64 | .preflight_bank_collapse_counters.quiet = [512, 8] | .preflight_bank_collapse_counters.restated_eq_only = [512, 8]' \
    'a preflight that compared no block after its pre-roll, its counters made consistent'
mixing_mutation '.preflight_bank_collapse_counters.quiet = [1016, 8] | .preflight_bank_collapse_counters.restated_eq_only = [1016, 8]' \
    'a preflight quiet arm that lost a cohort-block, the EQ-only arm losing it too'
mixing_mutation '.preroll_blocks = 32 | .quiet_bank_collapse_counters = [8256, 8] | .preflight_blocks = 96 | .preflight_bank_collapse_counters.quiet = [768, 8] | .preflight_bank_collapse_counters.restated_eq_only = [768, 8]' \
    'a shorter pre-roll than the row freezes, every count made consistent'
mixing_mutation '.owner_edits_per_block = 4 | .restated_pushes_accepted = 14000 | .automated_pushes_accepted = 14000' \
    'an owner edit per block the controls do not make, the push counts made consistent'
mixing_mutation '.parameter_records_per_block = 11 | .restated_pushes_accepted = 14000 | .automated_pushes_accepted = 14000' \
    'a parameter record per block the controls do not make, the push counts made consistent'
mixing_mutation '.automated_controls |= (to_entries | map({key: (.key | tostring), value}) | from_entries)' \
    'the controls as an object in control order'
mixing_mutation '.automated_controls[0] |= (.even_value = "b" | .base = "a")' \
    'a base and a ride value that are not numbers, in jq'"'"'s order'
mixing_mutation '.quiet_p50_ns = 0' 'a zero-cost quiet arm'
mixing_mutation '.restated_p99_ns = 1' 'restated percentiles out of order'
mixing_mutation '.automated_p99_ns = 1' 'automated percentiles out of order'
mixing_mutation '.restated_bank_collapse_counters = [3192, 8, 0]' 'a collapse counter that is not a pair'
mixing_mutation '.automated_bank_collapse_counters = [9000, 8]' 'an automated arm collapsing more than quiet'
mixing_mutation '.preflight_output_sha256.extra = "'"$digest_a"'"' 'a preflight arm the row does not run'
mixing_mutation '.preflight_output_sha256.automated_eq_only = "not-a-digest"' 'a preflight digest that is not a digest'
mixing_mutation '.preflight_output_sha256.automated = .preflight_output_sha256.restated' \
    'a preflight automated arm that rendered the restated bits'
mixing_mutation '.preflight_bank_collapse_counters.extra = [1024, 8]' 'preflight counters for an arm the row does not run'
mixing_mutation '.preflight_bank_collapse_counters.automated_eq_only = [1024, 8, 0]' \
    'a preflight collapse counter that is not a pair'
# The future the row exists to measure: with the collapse kept through restating records, the
# restated arm collapses what quiet does and the collapse delta is noise. Still a valid record.
expect_accept "$(printf '%s' "$mixing" | jq -c '.restated_bank_collapse_counters = [8512, 8] | .automated_bank_collapse_counters = [8512, 8] | .paired_collapse_delta_median_ns = -40 | .restated_p50_ns = 69700')" \
    'a mixing row whose restatements keep the collapse'

records=$(jq -cn --argjson session "$session" --argjson hoist "$hoist" \
    --argjson meters "$meters" --argjson observation "$observation" \
    --argjson placement "$placement" --argjson automation "$automation" \
    --argjson mono "$mono" --argjson mixing "$mixing" \
    --arg a "$digest_a" --arg b "$digest_b" '
  def console_fixture: "fixtures/session/v1/console-sixty-four-track-intended.json";
  def legacy_fixture: "fixtures/session/v1/console-sixty-four-track.json";
  def mono_fixture: "fixtures/session/v1/console-sixty-four-track-mono.json";
  def app_fixture: "fixtures/session/v1/console-sixty-four-track-app.json";
  def sends_fixture: "fixtures/session/v1/console-sixty-four-track-sends.json";
  def intended_layout: "pre_insert:eq+compressor,post_insert:limiter";
  def sessions: [
    {kind: "nine_track_baseline", tracks: 9, synthetic: false, strip: "eq", layout: "pre_insert:eq",
     signal: "tone", fixture: "fixtures/session/v1/parametric-eq-nine-track.json", digest: "1"},
    {kind: "nine_track_ragged_strip", tracks: 9, synthetic: true, strip: "eq+compressor+limiter",
     layout: intended_layout, signal: "tone", fixture: console_fixture, digest: "2"},
    {kind: "sixty_four_track_console", tracks: 64, synthetic: false, strip: "eq+compressor+limiter",
     layout: intended_layout, signal: "tone", fixture: console_fixture, digest: "3"},
    {kind: "one_twenty_eight_track_stretch", tracks: 128, synthetic: true,
     strip: "eq+compressor+limiter", layout: intended_layout, signal: "tone",
     fixture: console_fixture, digest: "4"},
    {kind: "sixty_four_track_eq_only", tracks: 64, synthetic: true, strip: "eq",
     layout: "pre_insert:eq", signal: "tone", fixture: console_fixture, digest: "5"},
    {kind: "sixty_four_track_compressor_only", tracks: 64, synthetic: true, strip: "compressor",
     layout: "pre_insert:compressor", signal: "tone", fixture: console_fixture, digest: "6"},
    {kind: "sixty_four_track_builtins_only", tracks: 64, synthetic: true, strip: "builtins",
     layout: "builtins", signal: "tone", fixture: console_fixture, digest: "7"},
    {kind: "sixty_four_track_dispatch_only", tracks: 64, synthetic: true, strip: "identity",
     layout: "builtins", signal: "tone", fixture: console_fixture, digest: "8"},
    {kind: "sixty_four_track_idle", tracks: 64, synthetic: true, strip: "eq+compressor+limiter",
     layout: intended_layout, signal: "silence", fixture: console_fixture, digest: "9"},
    {kind: "sixty_four_track_console_legacy", tracks: 64, synthetic: false, strip: "eq+compressor",
     layout: "pre_insert:eq,inserts:compressor", signal: "tone", fixture: legacy_fixture, digest: "e"},
    {kind: "sixty_four_track_eq_comp_simd1", tracks: 64, synthetic: true, strip: "eq+compressor",
     layout: "pre_insert:eq+compressor", signal: "tone", fixture: console_fixture, digest: "f"},
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
     fixture: mono_fixture, digest: "d"},
    # #928 and #956: emitted after the fifteen, and rendering the bits of the bound gain-and-pan
    # row.
    {kind: "sixty_four_track_gain_pan_ring", tracks: 64, synthetic: true, strip: "gain+pan",
     layout: "builtins", signal: "tone", fixture: console_fixture, digest: "a",
     feed: "played_planes"},
    # #881: emitted last, rendering the bits of the standing console row, and carrying the meter
    # group and the counters of its own plan.
    {kind: "sixty_four_track_console_metered", tracks: 64, synthetic: false,
     strip: "eq+compressor+limiter", layout: intended_layout, signal: "tone",
     fixture: console_fixture, digest: "3",
     extra: {meter_streams: 64, meter_tap: "post_matrix", meter_metrics: "sample_peak",
             meter_window_blocks: 12, meter_snapshots: 5312, meter_dropped_snapshots: 0,
             bank_route_folds: 64, bank_scatter_redirects: 0}},
    # #1227: the bus-and-send row, emitted after the metered row and before the console-strip rows,
    # rendering bits of its own, never the standing row bits.
    {kind: "sixty_four_track_console_sends", tracks: 64, synthetic: false,
     strip: "eq+compressor+limiter", layout: intended_layout, signal: "tone",
     fixture: sends_fixture, digest: "6"},
    # #1085: the five console-strip rows, emitted after the bus-and-send row. The sparse row renders
    # bits of its own, neither the standing nor the idle row bits.
    {kind: "ten_track_ragged_strip", tracks: 10, synthetic: true, strip: "eq+compressor+limiter",
     layout: intended_layout, signal: "tone", fixture: console_fixture, digest: "0"},
    {kind: "thirteen_track_ragged_strip", tracks: 13, synthetic: true,
     strip: "eq+compressor+limiter", layout: intended_layout, signal: "tone",
     fixture: console_fixture, digest: "1"},
    {kind: "sixteen_track_strip", tracks: 16, synthetic: true, strip: "eq+compressor+limiter",
     layout: intended_layout, signal: "tone", fixture: console_fixture, digest: "2"},
    {kind: "sixty_four_track_app_shape", tracks: 64, synthetic: false, strip: "eq+compressor",
     layout: "inserts:eq+compressor", signal: "tone", fixture: app_fixture, digest: "4",
     extra: {bypass_pattern: "index_mod_3_is_2", bypassed_tracks: 21}},
    {kind: "sixty_four_track_console_sparse", tracks: 64, synthetic: true,
     strip: "eq+compressor+limiter", layout: intended_layout, signal: "odd_tracks_silent",
     fixture: console_fixture, digest: "5"}
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
        | .source_feed = ($s.feed // "bound")
        | .fixture_id = $s.fixture | .round = $round
        | .output_sha256 = ($a[0:63] + $s.digest)
        | . + ($s.extra // {})),
      (hoists[] | . as $h | $hoist
        | .workload_kind = $h.kind | .tracks = $h.tracks | .round = $round
        | .quiet_output_sha256 = ($a[0:63] + $h.digest)
        | .restated_output_sha256 = ($a[0:63] + $h.digest)
        | .moving_output_sha256 = $b),
      ($meters | .round = $round),
      ($observation | .round = $round),
      ($placement | .round = $round),
      ($automation | .round = $round),
      ($mono | .round = $round),
      ($mixing | .round = $round)
  ) ]')

expect_aggregate_accept "$(printf '%s' "$records" | jq -c '.[]')" 'the sixty-two-record set'

# Index map of the frozen emission order: 0-22 are round one's twenty-three session rows (15 is
# the driver-fed gain-and-pan row, 16 the metered console row, 17 the bus-and-send row of #1227 and
# 18-22 the five console-strip rows of #1085), 23-24 its two hoist rows, 25 its meters row, 26 its
# observation row, 27 its placement row-pair, 28 its automation-active row, 29 its mono row-pair
# and 30 its mixing-automation row; 31-61 repeat for round two. #956 took two records out of the
# fifty before it (the plumbing row, both rounds), #1003 added two back at the end of each round,
# #1085 added ten after the metered row, and #1227 two between the metered row and the
# console-strip rows.
expect_aggregate_reject "$(printf '%s' "$records" | jq -c 'del(.[0]) | .[]')" 'sixty-one records'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '. as $r | ($r + [$r[27]]) | .[]')" 'sixty-three records'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '. as $r | ($r + [$r[0]]) | .[]')" 'a duplicated record'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[31].round = 1 | .[]')" 'a workload measured twice in one round'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[0].cpu_model = "Another CPU" | .[]')" 'records from two hosts'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[0].candidate_commit = "ffffffffffffffffffffffffffffffffffffffff" | .[]')" 'records from two commits'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[0].backend = "Scalar" | .[]')" 'records from two backends'
# Round one and round two must render the same bytes: they are two measurements of one workload.
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '.[31].output_sha256 = $c | .[]')" 'a workload whose rounds rendered different output'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.record == "console_session")] | .[]')" 'a set with no hoist rows'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.record != "console_meters")] | .[]')" 'a set with no meters arm'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.record != "console_observation")] | .[]')" 'a set with no observation arm'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '.[56].meters_off_output_sha256 = $c | .[56].meters_on_output_sha256 = $c | .[]')" 'a meters arm whose rounds rendered different output'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '.[57].absent_output_sha256 = $c | .[57].unarmed_output_sha256 = $c | .[57].armed_output_sha256 = $c | .[]')" 'an observation arm whose rounds rendered different output'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '.[58].split_chains_output_sha256 = $c | .[58].merged_chain_output_sha256 = $c | .[]')" 'a placement pair whose rounds rendered different output'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.record != "console_placement")] | .[]')" 'a set with no placement row-pair'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.record != "console_automation")] | .[]')" 'a set with no automation-active row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '.[59].quiet_output_sha256 = $c | .[59].restated_output_sha256 = $c | .[]')" 'an automation row whose rounds rendered different output'
# #144 item 13: two admissibility states in one accepted run is the comparison the control field
# exists to prevent, and a run that never stated one at all is not an accepted run.
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[0].measurement_control = "uncontrolled" | .[0].cpu_affinity = "uncontrolled" | .[0].background_load_note = "uncontrolled; MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1; waived affinity_unavailable" | .[]')" 'a run mixing controlled and uncontrolled records'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | .measurement_control = null | .cpu_affinity = null | .missing_metadata = ["cpu_affinity","measurement_control"]] | .[]')" 'a run that never stated its admissibility'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[0].workload_kind = "sixty_four_track_console" | .[0].tracks = 64 | .[0].synthetic_fixture = false | .[0].fixture_id = "fixtures/session/v1/console-sixty-four-track-intended.json" | .[0].strip_content = "eq+compressor+limiter" | .[0].strip_layout = "pre_insert:eq+compressor,post_insert:limiter" | .[]')" 'a set missing a declared workload'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_eq_only")] | .[]')" 'a set missing the eq-only decomposition row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_idle")] | .[]')" 'a set missing the idle row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_console_legacy")] | .[]')" 'a set missing the transition row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_eq_comp_simd1")] | .[]')" 'a set missing the chain-shape row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_compressor_automation")] | .[]')" 'a set missing the automation-active row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.record != "console_mono")] | .[]')" 'a set with no mono row-pair'
# #1003: the mixing-automation row is a row of the set, and both of its rounds are one frozen
# workload: the quiet and restated digests, the moving arm, the preflight and the collapse
# counters all agree across them.
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.record != "console_mixing_automation")] | .[]')" 'a set with no mixing-automation row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '.[61].quiet_output_sha256 = $c | .[61].restated_output_sha256 = $c | .[]')" 'a mixing row whose rounds rendered different output'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '.[61].automated_output_sha256 = $c | .[]')" 'a mixing row whose rounds rode different traffic'
# #1011: a fresh digest. `$digest_c` is the base record's preflight `restated` digest, so that
# edit is refused by the per-record A3 rule before the rounds are compared.
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg a "$digest_a" '.[61].preflight_output_sha256.automated_limiter_only = ($a[0:63] + "4") | .[]')" 'a mixing preflight whose rounds disagree'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[61].preflight_bank_collapse_counters.automated_compressor_only = [512, 8] | .[]')" 'a mixing preflight whose rounds collapsed differently'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[61].restated_bank_collapse_counters = [2128, 8] | .[]')" 'a mixing row whose rounds collapsed differently'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[61].automated_bank_collapse_counters = [2128, 8] | .[]')" 'a mixing automated arm whose rounds collapsed differently'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[61].quiet_bank_collapse_counters = [17024, 16] | .[61].restated_bank_collapse_counters = [6384, 16] | .[61].automated_bank_collapse_counters = [6384, 16] | .[]')" 'a mixing row whose rounds formed different cohorts'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '.[60].collapse_eligible_output_sha256 = $c | .[60].collapse_forced_off_output_sha256 = $c | .[]')" 'a mono pair whose rounds rendered different output'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '(.[] | select(.workload_kind == "sixty_four_track_gain_pan_ring")) |= (.workload_kind = "sixty_four_track_plumbing_ring" | .strip_content = "plumbing" | .strip_layout = "plumbing") | .[]')" 'a set carrying the retired plumbing ring row in place of the gain-and-pan ring row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_gain_pan_only")] | .[]')" 'a set missing the gain-and-pan row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_console_mono")] | .[]')" 'a set missing the mono session row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_console_mono_dual")] | .[]')" 'a set missing the mono control row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_console_half_mono")] | .[]')" 'a set missing the mixed-cohort row'
# #928 and #956: the driver-fed row is a row of the set, and its digest is the bound gain-and-pan
# row's. Both
# rounds are moved together so the rounds rule is satisfied and only the pair pin can refuse it.
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_gain_pan_ring")] | .[]')" 'a set missing the driver-fed gain-and-pan row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '(.[] | select(.workload_kind == "sixty_four_track_gain_pan_ring")).output_sha256 = $c | .[]')" 'a driver-fed row that rendered other bits than the bound row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '(.[] | select(.workload_kind == "sixty_four_track_gain_pan_only")).output_sha256 = $c | .[]')" 'a bound gain-and-pan row that rendered other bits than the driver-fed row'
expect_aggregate_accept "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '(.[] | select(.workload_kind == "sixty_four_track_gain_pan_only" or .workload_kind == "sixty_four_track_gain_pan_ring")).output_sha256 = $c | .[]')" 'the gain-and-pan pair moving together'
# #881: the metered console row is a row of the set and its digest is the standing console row's.
# Both rounds are moved together so the rounds rule is satisfied and only the pair pin can refuse
# it. Its counters are compared with no other record: every other plan in the run is a
# `Concurrent` one, so a redirect count that differs from theirs is a truthful record.
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_console_metered")] | .[]')" 'a set missing the metered console row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '(.[] | select(.workload_kind == "sixty_four_track_console_metered")).output_sha256 = $c | .[]')" 'a metered row that rendered other bits than the standing row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '(.[] | select(.record == "console_session" and .workload_kind == "sixty_four_track_console")).output_sha256 = $c | .[]')" 'a standing row that rendered other bits than the metered row'
expect_aggregate_accept "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '(.[] | select(.record == "console_session" and (.workload_kind == "sixty_four_track_console" or .workload_kind == "sixty_four_track_console_metered"))).output_sha256 = $c | .[]')" 'the console pair moving together'
expect_aggregate_accept "$(printf '%s' "$records" | jq -c '(.[] | select(.workload_kind == "sixty_four_track_console_metered")).bank_scatter_redirects = 8 | .[]')" 'a metered row redirecting lanes the Concurrent meters arm does not'
expect_aggregate_accept "$(printf '%s' "$records" | jq -c '(.[] | select(.record == "console_meters")) |= (.meters_off_bank_scatter_redirects = 8 | .meters_on_bank_scatter_redirects = 8) | .[]')" 'a Concurrent meters arm redirecting lanes the metered row does not'
# #1085: every console-strip row is required, and the sparse row renders neither the all-active
# row's bits nor the idle row's. Each digest edit below moves both rounds of one row together, so
# the rounds still agree and only the sparse rule can refuse it.
for kind in ten_track_ragged_strip thirteen_track_ragged_strip sixteen_track_strip \
    sixty_four_track_app_shape sixty_four_track_console_sparse; do
    expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg k "$kind" '[.[] | select(.workload_kind != $k)] | .[]')" \
        "a set missing the $kind row"
done
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '(.[] | select(.record == "console_session" and .workload_kind == "sixty_four_track_console_sparse")).output_sha256 = (first(.[] | select(.record == "console_session" and .workload_kind == "sixty_four_track_console")).output_sha256) | .[]')" \
    'a sparse row that rendered the all-active row bits'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '(.[] | select(.record == "console_session" and .workload_kind == "sixty_four_track_console_sparse")).output_sha256 = (first(.[] | select(.record == "console_session" and .workload_kind == "sixty_four_track_idle")).output_sha256) | .[]')" \
    'a sparse row that rendered the idle row bits'
expect_aggregate_accept "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '(.[] | select(.record == "console_session" and .workload_kind == "sixty_four_track_console_sparse")).output_sha256 = $c | .[]')" \
    'a sparse row rendering bits of its own'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '(.[] | select(.record == "console_session" and .workload_kind == "sixty_four_track_console_sparse" and .round == 2)).output_sha256 = $c | .[]')" \
    'a sparse row whose rounds rendered different output'
expect_aggregate_accept "$(printf '%s' "$records" | jq -c '(.[] | select(.workload_kind == "sixty_four_track_app_shape")).strip_layout = "pre_insert:eq+compressor" | .[]')" \
    'a run whose app shape reads the console spelling'
# #1227: the bus-and-send row is required, and it never renders the standing console row's bits.
# Each digest edit moves both rounds of the row together, so the rounds still agree and only the
# bus-and-send rule can refuse it.
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_console_sends")] | .[]')" \
    'a set missing the bus-and-send row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '(.[] | select(.record == "console_session" and .workload_kind == "sixty_four_track_console_sends")).output_sha256 = (first(.[] | select(.record == "console_session" and .workload_kind == "sixty_four_track_console")).output_sha256) | .[]')" \
    'a bus-and-send row that rendered the standing console row bits'
expect_aggregate_accept "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '(.[] | select(.record == "console_session" and .workload_kind == "sixty_four_track_console_sends")).output_sha256 = $c | .[]')" \
    'a bus-and-send row rendering bits of its own'

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
    "sixty_four_track_gain_pan_only": 1.01, "sixty_four_track_gain_pan_ring": 0.98,
    "sixty_four_track_console_metered": 4.80, "sixty_four_track_console_sends": 5.20,
    "sixty_four_track_console_mono": 4.38, "sixty_four_track_console_mono_dual": 4.38,
    "sixty_four_track_console_half_mono": 4.39,
    "ten_track_ragged_strip": 0.75, "thirteen_track_ragged_strip": 0.95,
    "sixteen_track_strip": 1.10, "sixty_four_track_app_shape": 2.90,
    "sixty_four_track_console_sparse": 4.10
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

expect_aggregate_accept "$(printf '%s' "$floor_records" | jq -c '.[]')" 'the sixty-two-record set with floor accounting'
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

# ---------------------------------------------------------------------------------------------
# #1011: the browser arm of the mixing-automation row (`scripts/web-mixing-automation-validator.jq`).
# Two measured rounds of one prepared module, slurped. The base pair states what the runner writes:
# the row's controls in the host's lowerings, the arm's streamed input beside the native row's
# frozen blocks, the provenance, and digests that agree across the rounds.
# ---------------------------------------------------------------------------------------------
web_valid() { printf '%s\n' "$1" | jq -s -e -L "$scripts_dir" -f "$scripts_dir/web-mixing-automation-validator.jq" >/dev/null 2>&1; }
expect_web_accept() {
    if ! web_valid "$1"; then printf 'expected web accept: %s\n' "$2" >&2; failures=$((failures + 1)); fi
}
expect_web_reject() {
    if web_valid "$1"; then printf 'expected web reject: %s\n' "$2" >&2; failures=$((failures + 1)); fi
}
commit_a=$(printf '1%.0s' {1..40})
commit_b=$(printf '2%.0s' {1..40})
web_round=$(jq -cn --arg a "$digest_a" --arg b "$digest_b" --arg c "$digest_c" --arg commit "$commit_a" \
    "$mixing_controls"' as $controls | {
  schema_version: 1, issue: 1003, record: "web_mixing_automation", round: 1,
  workload_kind: "sixty_four_track_console_mono_mixing_automation",
  fixture_id: "fixtures/session/v1/console-sixty-four-track-mono.json", source_frames: "48000000",
  tracks: 64, input_signal: "tone",
  input_feed: {waveform: "sine", radians_per_frame: 0.017, amplitude: 0.6, track_phase_radians: 0,
    delivery: "streamed_source", block_frames: 128, continuous_across_blocks: true},
  native_input_feed: {waveform: "sine", radians_per_frame: 0.017, amplitude: 0.6,
    track_phase_radians: 0.31, delivery: "frozen_block_per_track", block_frames: 128,
    continuous_across_blocks: false},
  sample_rate_hz: 48000, quantum_frames: 128,
  module_sha256: $b, module_matches_pin: false, pinned_sha256: $c,
  node_version: "v22.23.2", v8_version: "12.4.254.21-node.56", node_flags: ["--no-liftoff"],
  console_command_queue_records: 64, source_ring_frames: 5120,
  observations: 1000, preroll_blocks: 64, pairing: "alternating_per_observation",
  arms: ["quiet","restated","automated"],
  controls: [$controls[] | . + {parameter_id: (if .effect == "miso.parametric-eq" then 4 else 1 end)}],
  command_records_per_block: 8, records_admitted: {quiet: 0, restated: 8000, automated: 8000},
  units: "ns_per_block", percentile_method: "nearest_rank",
  quiet_p50_ns: 154304, quiet_p95_ns: 225630, quiet_p99_ns: 233174,
  restated_p50_ns: 189220, restated_p95_ns: 277649, restated_p99_ns: 285344,
  automated_p50_ns: 216622, automated_p95_ns: 318577, automated_p99_ns: 325940,
  paired_ramp_delta_median_ns: 27603, paired_collapse_delta_median_ns: 34897,
  quiet_output_sha256: $a, restated_output_sha256: $a, automated_output_sha256: $b,
  preflight_output_sha256: {quiet: $c, restated: $c, automated: $b, automated_eq_only: ($a[0:63] + "1"),
    automated_compressor_only: ($a[0:63] + "2"), automated_limiter_only: ($a[0:63] + "3"),
    restated_eq_only: $c},
  documents: [
    {workload_kind: "sixty_four_track_console",
     fixture_id: "fixtures/session/v1/console-sixty-four-track-intended.json", tracks: 64,
     strip_content: "eq+compressor+limiter",
     strip_layout: "pre_insert:eq+compressor,post_insert:limiter", input_signal: "tone",
     bypass_pattern: "none", bypassed_tracks: 0,
     p50_ns: 301000, p95_ns: 322000, p99_ns: 340000, output_sha256: ($a[0:63] + "6")},
    {workload_kind: "sixty_four_track_app_shape",
     fixture_id: "fixtures/session/v1/console-sixty-four-track-app.json", tracks: 64,
     strip_content: "eq+compressor", strip_layout: "inserts:eq+compressor", input_signal: "tone",
     bypass_pattern: "index_mod_3_is_2", bypassed_tracks: 21,
     p50_ns: 190000, p95_ns: 205000, p99_ns: 219000, output_sha256: ($a[0:63] + "7")}],
  bit_identity: "quiet == restated, asserted in-run", bank_collapse_counters_exported: false,
  loadavg_start: "9.52 7.22 7.64 8/1930 3836898", loadavg_end: "9.48 7.25 7.65 6/1921 3837039",
  descriptive_only: true,
  statistical_method: "three arms alternated per observation; one warmup launch and two measured launches; descriptive only; no threshold",
  candidate_commit: $commit, prepared_commit: $commit,
  measurement_control: "uncontrolled; MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1; waived loadavg_above_ceiling; loadavg 9.52 7.22 7.64 21/1947 3836866; affinity cpu 31",
  cpu_affinity: "31"
}')
web_pair=$(printf '%s' "$web_round" | jq -c '., (.round = 2 | .quiet_p50_ns = 150000 | .paired_ramp_delta_median_ns = 27011
    | .documents[1].p50_ns = 188000)')
# One edit applied to both rounds, so the rounds still agree and only a per-record rule can refuse it.
web_mutation() { expect_web_reject "$(printf '%s\n' "$web_pair" | jq -c -s "map($1) | .[]")" "$2"; }
# One edit applied to round two only, which only the cross-round rule can refuse.
web_round_mutation() { expect_web_reject "$(printf '%s\n' "$web_pair" | jq -c -s ".[1] |= ($1) | .[]")" "$2"; }

expect_web_accept "$web_pair" 'the two measured browser rounds'
while read -r field; do
    web_mutation "del(.\"$field\")" "a browser round without $field"
    web_mutation ".\"$field\" = null" "a browser round with $field nulled"
done < <(printf '%s' "$web_round" | jq -r 'keys[]')
web_mutation '.unexpected_key = 1' 'a browser round with an extra key'
web_mutation '.round = 0' 'the warmup launch published as a measured round'
web_mutation '.quantum_frames = 256 | .input_feed.block_frames = 256 | .native_input_feed.block_frames = 256' \
    'a browser round in another quantum, its feeds made consistent'
web_mutation '.observations = 500 | .records_admitted.restated = 4000 | .records_admitted.automated = 4000' \
    'a shortened browser round, its admitted records made consistent'
web_mutation '.command_records_per_block = 13 | .records_admitted.restated = 13000 | .records_admitted.automated = 13000' \
    'the native push count on the web wire, its admitted records made consistent'
web_mutation '.source_ring_frames = 0' 'a browser round with no source ring'
web_mutation '.input_feed.waveform = "square"' 'a streamed input that is not a sine'
web_mutation '.native_input_feed.waveform = "square"' 'a native input that is not a sine'
web_mutation '.native_input_feed.block_frames = 256' 'a native feed in another quantum'
web_mutation '.quiet_p99_ns = 1' 'browser quiet percentiles out of order'
web_mutation '.restated_p99_ns = 1' 'browser restated percentiles out of order'
web_mutation '.quiet_output_sha256 = "x" | .restated_output_sha256 = "x"' 'browser digests that are not digests'
web_mutation '.preflight_output_sha256.automated_eq_only = "not-a-digest"' 'a browser preflight digest that is not a digest'
web_mutation '.candidate_commit = "HEAD" | .prepared_commit = "HEAD"' 'a run and a prepare naming no commit'
web_mutation '.record = "console_mixing_automation"' 'a browser round claiming the native shape'
web_mutation '.fixture_id = "fixtures/session/v1/console-sixty-four-track-intended.json"' \
    'a browser round on the stereo console'
# The input: the claim #1011 added, and what separates the two feeds.
web_mutation '.input_feed.continuous_across_blocks = false' 'a streamed tone claiming frozen blocks'
web_mutation '.input_feed.delivery = "frozen_block_per_track"' 'the browser arm claiming the native feed'
web_mutation '.input_feed.track_phase_radians = 0.31' 'a browser tone phase-offset by track'
web_mutation '.native_input_feed.continuous_across_blocks = true' 'the native feed claiming a continuous tone'
web_mutation '.native_input_feed.track_phase_radians = 0' 'the native feed claiming tracks in phase'
web_mutation '.native_input_feed.delivery = "streamed_source"' 'the native feed claiming a streamed source'
web_mutation '.input_feed.radians_per_frame = 0.02' 'a browser tone at another rate than the native'"'"'s'
web_mutation '.input_feed.amplitude = 0.3' 'a browser tone at another level than the native'"'"'s'
web_mutation '.input_feed.block_frames = 256' 'a browser feed in another quantum'
web_mutation '.input_feed.extra = 1' 'a feed with a fact the validator does not know'
web_mutation '.input_signal = "silence"' 'a browser round claiming silence'
# The module and the protocol.
web_mutation '.module_matches_pin = true' 'a module claiming the pin it does not match'
web_mutation '.node_flags = []' 'a browser round with Liftoff enabled'
web_mutation '.console_command_queue_records = 8' 'a queue the SDK does not boot'
web_mutation '.source_ring_frames = 100' 'a source ring that is not whole quanta'
web_mutation '.prepared_commit = "'"$commit_b"'"' 'a module prepared at another commit than the run'"'"'s'
web_mutation '.candidate_commit = "HEAD"' 'a commit that is not a commit id'
web_mutation '.measurement_control = "uncontrolled"' 'an uncontrolled round that does not name its waiver'
web_mutation '.cpu_affinity = "any"' 'a round pinned to no CPU'
web_mutation '.bank_collapse_counters_exported = true' 'a browser round claiming collapse counters it cannot read'
# The row, as the native pin states it.
web_mutation '.controls[0].lowering = "left_then_right"' 'an EQ pushed as two one-channel edits'
web_mutation '.controls[2].step = 0.25' 'a limiter step that never engages'
web_mutation '.controls[5].even_value = -1.75' 'a ride whose even value is the base'
web_mutation '.controls[6].odd_value = -4.25' 'a ride that does not straddle its base'
web_mutation '.controls[1].parameter_id = 0' 'a control with no wire parameter id'
web_mutation '.command_records_per_block = 13' 'the native push count on the web wire'
web_mutation '.records_admitted.restated = 7999' 'a restated record the host did not admit'
web_mutation '.records_admitted.quiet = 8' 'a quiet arm that submitted'
web_mutation '.quiet_p50_ns = 0' 'a zero-cost browser arm'
web_mutation '.automated_p99_ns = 1' 'browser percentiles out of order'
web_mutation '.paired_ramp_delta_median_ns = 0.5' 'a fractional paired delta'
web_mutation '.restated_output_sha256 = "'"$digest_c"'"' 'restating the held values moved a bit in V8'
web_mutation '.automated_output_sha256 = .restated_output_sha256' 'an automated arm that rendered the restated bits'
web_mutation '.bit_identity = "asserted"' 'a browser bit-identity sentence that drifted'
web_mutation '.preflight_output_sha256.automated_limiter_only = .preflight_output_sha256.restated' \
    'a browser limiter ride that moved no bit'
web_mutation '.preflight_output_sha256.automated_eq_only = .preflight_output_sha256.restated' \
    'a browser EQ ride that moved no bit'
web_mutation '.preflight_output_sha256.automated_compressor_only = .preflight_output_sha256.restated' \
    'a browser compressor ride that moved no bit'
web_mutation '.preflight_output_sha256.automated = .preflight_output_sha256.restated' \
    'a browser preflight automated arm that rendered the restated bits'
web_mutation '.preflight_output_sha256.restated_eq_only = "'"$digest_a"'"' 'a browser EQ restatement that moved a bit'
web_mutation '.preflight_output_sha256.quiet = "'"$digest_a"'"' 'a browser preflight restatement that moved a bit'
web_mutation '.preflight_output_sha256.extra = "'"$digest_a"'"' 'a browser preflight arm the row does not run'
# #1085: the console-strip documents. Each is the native row it names, with that row's facts; the
# app shape reads either layout spelling, and neither document's digest may be the other's.
web_document_mutation() { web_mutation "$1" "the documents: $2"; }
web_document_mutation 'del(.documents)' 'a browser round without its documents'
web_document_mutation '.documents = []' 'a browser round that timed no document'
web_document_mutation '.documents |= .[0:1]' 'a browser round that timed the console alone'
web_document_mutation '.documents |= reverse' 'documents in another order'
web_document_mutation '.documents |= . + [.[0]]' 'a third document'
web_document_mutation '.documents[0].workload_kind = "sixty_four_track_console_mono"' 'the mono console in place of the standing one'
web_document_mutation '.documents[0].fixture_id = "fixtures/session/v1/console-sixty-four-track-mono.json"' 'the standing console booted from the mono fixture'
web_document_mutation '.documents[0].strip_layout = "simd1:eq+compressor,simd2:limiter"' 'the rack-token spelling'
web_document_mutation '.documents[0].strip_layout = "inserts:eq+compressor"' 'the standing console claiming the app layout'
web_document_mutation '.documents[0].bypass_pattern = "index_mod_3_is_2" | .documents[0].bypassed_tracks = 21' 'the standing console claiming the app bypass'
web_document_mutation '.documents[1].fixture_id = "fixtures/session/v1/console-sixty-four-track-intended.json"' 'the app shape booted from the standing fixture'
web_document_mutation '.documents[1].strip_content = "eq+compressor+limiter"' 'the app shape claiming a limiter'
web_document_mutation '.documents[1].strip_layout = "pre_insert:eq+compressor,post_insert:limiter"' 'the app shape claiming the standing layout'
web_document_mutation '.documents[1].bypass_pattern = "other"' 'an app shape with another bypass'
web_document_mutation '.documents[1].bypassed_tracks = 20' 'an app shape bypassing one track too few'
web_document_mutation '.documents[1].tracks = 16' 'an app shape that is not the sixty-four-track console'
web_document_mutation '.documents[0].input_signal = "odd_tracks_silent"' 'a document claiming sparse input'
web_document_mutation '.documents[1].output_sha256 = .documents[0].output_sha256' 'one session booted twice'
web_document_mutation '.documents[0].output_sha256 = "x"' 'a document digest that is not a digest'
web_document_mutation '.documents[0].p50_ns = 0' 'a zero-cost document'
web_document_mutation '.documents[1].p99_ns = 1' 'document percentiles out of order'
web_document_mutation '.documents[0].extra = 1' 'a document with a fact the validator does not know'
web_document_mutation 'del(.documents[1].bypassed_tracks)' 'a document missing a fact'
expect_web_accept "$(printf '%s\n' "$web_pair" | jq -c -s 'map(.documents[1].strip_layout = "pre_insert:eq+compressor") | .[]')" \
    'the app-shape document after the console migration'
# The protocol across the rounds: two of them, measured, of one module at one commit, rendering the
# same bits. Each edit below leaves both records valid on their own.
expect_web_reject "$(printf '%s\n' "$web_pair" | jq -c -s '.[0] | .')" 'one measured browser round'
expect_web_reject "$(printf '%s\n' "$web_pair" | jq -c -s '.[0].round = 0 | .[]')" 'a warmup record beside a measured round'
expect_web_reject "$(printf '%s\n' "$web_pair" | jq -c -s '.[1].round = 1 | .[]')" 'two browser records of round one'
expect_web_reject "$(printf '%s\n' "$web_pair" | jq -c -s '. + [.[1]] | .[]')" 'three browser rounds'
web_round_mutation '.quiet_output_sha256 = "'"$digest_c"'" | .restated_output_sha256 = "'"$digest_c"'"' \
    'browser rounds that rendered different bits'
web_round_mutation '.automated_output_sha256 = "'"$digest_c"'"' 'browser rounds that rode different traffic'
web_round_mutation '.preflight_output_sha256.automated_limiter_only = "'"${digest_a:0:63}4"'"' \
    'browser rounds whose preflights disagree'
web_round_mutation '.module_sha256 = "'"$digest_a"'"' 'browser rounds of two modules'
web_round_mutation '.candidate_commit = "'"$commit_b"'" | .prepared_commit = "'"$commit_b"'"' \
    'browser rounds at two commits'
web_round_mutation '.controls[0].base = -7.25 | .controls[0].even_value = -7.0' 'browser rounds with two control tables'
web_round_mutation '.input_feed.amplitude = 0.5 | .native_input_feed.amplitude = 0.5' 'browser rounds on two tones'
web_round_mutation '.native_input_feed.track_phase_radians = 0.5' 'browser rounds stating two native feeds'
web_round_mutation '.node_version = "v24.0.0"' 'browser rounds under two Node versions'
web_round_mutation '.v8_version = "13.0"' 'browser rounds under two V8 versions'
web_round_mutation '.node_flags = ["--no-liftoff","--jitless"]' 'browser rounds under two flag sets'
web_round_mutation '.source_frames = "96000000"' 'browser rounds on two documents'
web_round_mutation '.source_ring_frames = 6144' 'browser rounds with two source rings'
web_round_mutation '.measurement_control = "controlled; loadavg 0.01; ceiling 0.50; affinity cpu 31"' \
    'browser rounds under two admissibility states'
web_round_mutation '.cpu_affinity = "30"' 'browser rounds on two CPUs'
web_round_mutation '.documents[0].output_sha256 = "'"${digest_a:0:63}8"'"' 'browser rounds whose standing-console document rendered different bits'
web_round_mutation '.documents[1].output_sha256 = "'"${digest_a:0:63}9"'"' 'browser rounds whose app-shape document rendered different bits'
web_round_mutation '.documents[1].strip_layout = "pre_insert:eq+compressor"' 'browser rounds stating two app layouts'

# ---------------------------------------------------------------------------------------------
# The browser arm's runner (`run-web-mixing-automation-benchmark.sh run`, #1011 follow-up): what it
# keeps, what it refuses to overwrite, and what it checks again after the rounds. Nothing is timed.
# Each case runs the real runner in a throwaway git repository whose harness is a stub: it prints
# the base round above for each launch, after doing to the run the one thing the case is about.
# ---------------------------------------------------------------------------------------------
runner_tmp=$(mktemp -d)
trap 'rm -rf -- "$runner_tmp"' EXIT
web_template=$(printf '%s' "$web_round" | jq -c 'del(.candidate_commit, .prepared_commit, .measurement_control, .cpu_affinity)')
runner_git() { git -C "$1" -c user.name=runner-test -c user.email=runner-test@invalid "${@:2}" >/dev/null 2>&1; }
# A repository carrying the runner and its validators, committed, and a prepared work directory
# whose provenance names that commit.
runner_fixture() {
    local case_dir="$runner_tmp/$1" repo work
    repo="$case_dir/repo"
    work="$case_dir/work"
    mkdir -p "$repo/scripts" "$work"
    cp -- "$scripts_dir/run-web-mixing-automation-benchmark.sh" "$scripts_dir/check-bench-preconditions.sh" \
        "$scripts_dir/web-mixing-automation-lib.jq" "$scripts_dir/web-mixing-automation-validator.jq" \
        "$scripts_dir/console-benchmark-record-lib.jq" "$repo/scripts/"
    printf '%s\n' "$web_template" >"$repo/round.json"
    printf 'tracked\n' >"$repo/tracked.txt"
    cat >"$repo/scripts/web-mixing-automation-benchmark.mjs" <<'STUB'
// Stub harness: prints the template round, after the one disturbance its case names.
import { appendFileSync, readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { execFileSync } from "node:child_process";
const repo = new URL("../", import.meta.url).pathname;
const [, , mode, module, controls, round] = process.argv;
const plan = JSON.parse(readFileSync(`${repo}case.json`, "utf8"));
appendFileSync(`${repo}launches`, `${round}\n`);
const record = JSON.parse(readFileSync(`${repo}round.json`, "utf8"));
record.round = round === "warmup" ? 0 : Number(round);
if (round === "2") {
  const git = (...args) => execFileSync("git", ["-C", repo, "-c", "user.name=stub", "-c", "user.email=stub@invalid", ...args]);
  if (plan.disturb === "disagree") record.automated_output_sha256 = "c".repeat(64);
  if (plan.disturb === "race") {
    mkdirSync(`${repo}artifacts/steps/${plan.step}`, { recursive: true });
    writeFileSync(`${repo}artifacts/steps/${plan.step}/web-mixing-automation.jsonl`, "concurrent\n");
  }
  if (plan.disturb === "head") git("commit", "--allow-empty", "-q", "-m", "moved");
  if (plan.disturb === "tracked") appendFileSync(`${repo}tracked.txt`, "edited\n");
  if (plan.disturb === "module") appendFileSync(module, "x");
  if (plan.disturb === "provenance") {
    const provenance = `${module.slice(0, module.lastIndexOf("/"))}/provenance.json`;
    writeFileSync(provenance, readFileSync(provenance, "utf8").replace(/"module_sha256": "[0-9a-f]{64}"/, `"module_sha256": "${"0".repeat(64)}"`));
  }
}
if (mode !== "run") process.exit(2);
process.stdout.write(`${JSON.stringify(record)}\n`);
STUB
    runner_git "$repo" init -q
    runner_git "$repo" add -A
    runner_git "$repo" commit -q -m fixture
    printf 'module bytes\n' >"$work/host_web.wasm"
    printf '{"controls":[]}\n' >"$work/controls.json"
    jq -n --arg commit "$(git -C "$repo" rev-parse HEAD)" \
        --arg module "$(sha256sum "$work/host_web.wasm" | awk '{print $1}')" \
        --arg controls "$(sha256sum "$work/controls.json" | awk '{print $1}')" \
        '{commit: $commit, module_sha256: $module, controls_sha256: $controls}' >"$work/provenance.json"
    jq -n --arg disturb "$2" --arg step "$1" '{disturb: $disturb, step: $step}' >"$repo/case.json"
    printf '%s' "$case_dir"
}
# Runs the fixture's runner; its combined output goes to `$case_dir/output`, its status is returned.
runner_run() {
    local case_dir=$1
    MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1 bash "$case_dir/repo/scripts/run-web-mixing-automation-benchmark.sh" \
        run "$case_dir/work" --step "$(basename "$case_dir")" >"$case_dir/output" 2>&1
}
runner_expect() {
    if ! eval "$2"; then printf 'expected runner behaviour: %s\n' "$1" >&2; failures=$((failures + 1)); fi
}
lines() { if [[ -f "$1" ]]; then wc -l <"$1"; else printf 0; fi; }

# The control: undisturbed, the runner writes the two measured records and nothing else.
case_dir=$(runner_fixture valid none)
artifacts="$case_dir/repo/artifacts/steps/valid"
runner_run "$case_dir" && status=0 || status=$?
runner_expect 'an undisturbed run succeeds' '[[ $status == 0 ]]'
runner_expect 'an undisturbed run launches warmup, 1 and 2' '[[ "$(tr "\n" " " <"$case_dir/repo/launches")" == "warmup 1 2 " ]]'
runner_expect 'an undisturbed run writes two records' '[[ $(lines "$artifacts/web-mixing-automation.jsonl") == 2 ]]'
runner_expect 'an undisturbed run keeps no refused records' '[[ ! -e "$artifacts/web-mixing-automation.refused.jsonl" ]]'

# Finding 1: a refusal after the rounds keeps what was measured, and says why.
case_dir=$(runner_fixture disagree disagree)
artifacts="$case_dir/repo/artifacts/steps/disagree"
runner_run "$case_dir" && status=0 || status=$?
runner_expect 'rounds that disagree are refused' '[[ $status != 0 ]]'
runner_expect 'refused rounds publish no record' '[[ ! -e "$artifacts/web-mixing-automation.jsonl" ]]'
runner_expect 'refused rounds are kept' '[[ $(lines "$artifacts/web-mixing-automation.refused.jsonl") == 2 ]]'
runner_expect 'a refusal names the disagreeing field' 'grep -q "the rounds disagree on automated_output_sha256" "$case_dir/output"'
runner_expect 'a refusal is logged' 'grep -q "the rounds disagree on automated_output_sha256" "$artifacts/web-mixing-automation.stderr.log"'

# Finding 2: no artifact is overwritten. An existing one is refused before anything is launched,
# and one that appears while the rounds run is refused at the write and left as it was.
for existing in web-mixing-automation.jsonl web-mixing-automation.refused.jsonl web-mixing-automation.stderr.log; do
    case_dir=$(runner_fixture "existing-${existing//./-}" none)
    artifacts="$case_dir/repo/artifacts/steps/existing-${existing//./-}"
    mkdir -p "$artifacts"
    printf 'earlier\n' >"$artifacts/$existing"
    runner_run "$case_dir" && status=0 || status=$?
    runner_expect "an existing $existing is refused" '[[ $status != 0 ]]'
    runner_expect "an existing $existing is refused before any launch" '[[ ! -e "$case_dir/repo/launches" ]]'
    runner_expect "an existing $existing is left as it was" '[[ "$(cat "$artifacts/$existing")" == earlier ]]'
done
case_dir=$(runner_fixture race race)
artifacts="$case_dir/repo/artifacts/steps/race"
runner_run "$case_dir" && status=0 || status=$?
runner_expect 'a record that appears during the rounds is refused' '[[ $status != 0 ]]'
runner_expect 'a record that appears during the rounds is not overwritten' '[[ "$(cat "$artifacts/web-mixing-automation.jsonl")" == concurrent ]]'
runner_expect 'the rounds of a raced run are kept' '[[ $(lines "$artifacts/web-mixing-automation.refused.jsonl") == 2 ]]'

# Finding 3: the provenance is checked again after the rounds, before the record is written.
for disturb in head tracked module provenance; do
    case_dir=$(runner_fixture "moved-$disturb" "$disturb")
    artifacts="$case_dir/repo/artifacts/steps/moved-$disturb"
    runner_run "$case_dir" && status=0 || status=$?
    runner_expect "a run whose $disturb changed during the rounds is refused" '[[ $status != 0 ]]'
    runner_expect "a run whose $disturb changed during the rounds publishes no record" '[[ ! -e "$artifacts/web-mixing-automation.jsonl" ]]'
    runner_expect "a run whose $disturb changed during the rounds keeps them" '[[ $(lines "$artifacts/web-mixing-automation.refused.jsonl") == 2 ]]'
done
rm -rf -- "$runner_tmp"
trap - EXIT

if [[ "$failures" != 0 ]]; then
    printf 'console benchmark validator suite: %s FAILED case(s)\n' "$failures" >&2
    exit 1
fi
printf 'console benchmark validators: PASS (real runner/workload/timing invocations: 0/0/0; browser runner on a stub harness, untimed)\n'
