#!/usr/bin/env bash
# Console validator mutation suite. Hermetic: no workload, no timing, no binary.
#
# A validator that has never been shown to reject anything is decoration. Every rule below is
# mutated in turn and asserted red, so the aggregate's guarantees -- twenty-six records, both
# rounds, one host, one admissibility state, the decomposition rows' pinned strip contents, and
# the class-A statements that neither the stationary smoother nor a meter nor an armed observation
# tap changes a rendered bit -- are properties the suite can actually lose.
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

expect_accept "$session" 'the base session record'
expect_accept "$hoist" 'the base hoist record'
expect_accept "$meters" 'the base meters record'
expect_accept "$observation" 'the base observation record'
expect_accept "$placement" 'the base placement record'

# ---------------------------------------------------------------------------------------------
# Per-key structural mutations: every key is load-bearing in both directions.
# ---------------------------------------------------------------------------------------------
for base in "$session" "$hoist" "$meters" "$observation" "$placement"; do
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
# A record that claims one shape and carries another's keys.
expect_reject "$(printf '%s' "$placement" | jq -c '.record = "console_meters"')" \
    'a placement record claiming to be a meters record'
expect_reject "$(printf '%s' "$meters" | jq -c '.record = "console_placement"')" \
    'a meters record claiming to be a placement record'
# The saving the hypothesis predicted would show up here first: an arm pair whose transpose counts
# differ is a real finding, not a malformed record, so it must still validate.
expect_accept "$(printf '%s' "$placement" | jq -c '.split_chains_transposes_per_block = 32 | .paired_delta_median_ns = -4000 | .paired_delta_median_ns_per_track = -62.5')" \
    'a placement pair that did save a round-trip'

records=$(jq -cn --argjson session "$session" --argjson hoist "$hoist" \
    --argjson meters "$meters" --argjson observation "$observation" \
    --argjson placement "$placement" \
    --arg a "$digest_a" --arg b "$digest_b" '
  def console_fixture: "fixtures/session/v1/console-sixty-four-track-intended.toml";
  def legacy_fixture: "fixtures/session/v1/console-sixty-four-track.toml";
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
     layout: "simd1:eq+compressor", signal: "tone", fixture: console_fixture, digest: "f"}
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
      ($placement | .round = $round)
  ) ]')

expect_aggregate_accept "$(printf '%s' "$records" | jq -c '.[]')" 'the thirty-two-record set'

# Index map of the frozen emission order: 0-10 are round one's eleven session rows, 11-12 its two
# hoist rows, 13 its meters row, 14 its observation row and 15 its placement row-pair; 16-31
# repeat for round two.
expect_aggregate_reject "$(printf '%s' "$records" | jq -c 'del(.[0]) | .[]')" 'thirty-one records'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '. as $r | ($r + [$r[15]]) | .[]')" 'thirty-three records'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '. as $r | ($r + [$r[0]]) | .[]')" 'a duplicated record'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[16].round = 1 | .[]')" 'a workload measured twice in one round'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[0].cpu_model = "Another CPU" | .[]')" 'records from two hosts'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[0].candidate_commit = "ffffffffffffffffffffffffffffffffffffffff" | .[]')" 'records from two commits'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[0].backend = "Scalar" | .[]')" 'records from two backends'
# Round one and round two must render the same bytes: they are two measurements of one workload.
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '.[16].output_sha256 = $c | .[]')" 'a workload whose rounds rendered different output'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.record == "console_session")] | .[]')" 'a set with no hoist rows'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.record != "console_meters")] | .[]')" 'a set with no meters arm'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.record != "console_observation")] | .[]')" 'a set with no observation arm'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '.[29].meters_off_output_sha256 = $c | .[29].meters_on_output_sha256 = $c | .[]')" 'a meters arm whose rounds rendered different output'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '.[30].absent_output_sha256 = $c | .[30].unarmed_output_sha256 = $c | .[30].armed_output_sha256 = $c | .[]')" 'an observation arm whose rounds rendered different output'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '.[31].split_chains_output_sha256 = $c | .[31].merged_chain_output_sha256 = $c | .[]')" 'a placement pair whose rounds rendered different output'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.record != "console_placement")] | .[]')" 'a set with no placement row-pair'
# #144 item 13: two admissibility states in one accepted run is the comparison the control field
# exists to prevent, and a run that never stated one at all is not an accepted run.
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[0].measurement_control = "uncontrolled" | .[0].cpu_affinity = "uncontrolled" | .[0].background_load_note = "uncontrolled; MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1; waived affinity_unavailable" | .[]')" 'a run mixing controlled and uncontrolled records'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | .measurement_control = null | .cpu_affinity = null | .missing_metadata = ["cpu_affinity","measurement_control"]] | .[]')" 'a run that never stated its admissibility'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[0].workload_kind = "sixty_four_track_console" | .[0].tracks = 64 | .[0].synthetic_fixture = false | .[0].fixture_id = "fixtures/session/v1/console-sixty-four-track-intended.toml" | .[0].strip_content = "eq+compressor+limiter" | .[0].strip_layout = "simd1:eq+compressor,simd2:limiter" | .[]')" 'a set missing a declared workload'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_eq_only")] | .[]')" 'a set missing the eq-only decomposition row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_idle")] | .[]')" 'a set missing the idle row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_console_legacy")] | .[]')" 'a set missing the transition row'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.workload_kind != "sixty_four_track_eq_comp_simd1")] | .[]')" 'a set missing the chain-shape row'

if [[ "$failures" != 0 ]]; then
    printf 'console benchmark validator suite: %s FAILED case(s)\n' "$failures" >&2
    exit 1
fi
printf 'console benchmark validators: PASS (real runner/workload/timing invocations: 0/0/0)\n'
