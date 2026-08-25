#!/usr/bin/env bash
# Issue-149 console validator mutation suite. Hermetic: no workload, no timing, no binary.
#
# A validator that has never been shown to reject anything is decoration. Every rule below is
# mutated in turn and asserted red, so the aggregate's guarantees -- twelve records, both rounds,
# one host, and the class-A statement that the stationary arm renders exactly what the untouched
# arm renders -- are properties the suite can actually lose.
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

session=$(jq -cn --arg a "$digest_a" '{
  schema_version: 1, issue: 149, record: "console_session",
  workload_kind: "sixty_four_track_console", tracks: 64, synthetic_fixture: false,
  fixture_id: "fixtures/session/v1/console-sixty-four-track.toml",
  round: 1, backend: "Simd8", sample_rate_hz: 48000, quantum_frames: 128, observations: 1000,
  units: "us_per_block", percentile_method: "nearest_rank",
  min_us_per_block: 281.9, p50_us_per_block: 283.4, p95_us_per_block: 285.4,
  p99_us_per_block: 295.7, max_us_per_block: 297.2, p50_us_per_block_per_track: 4.429,
  min_ns_per_block: 281915, p50_ns_per_block: 283459, p95_ns_per_block: 285482,
  p99_ns_per_block: 295702, max_ns_per_block: 297245,
  output_sha256: $a, render_errors: 0, render_total_forbidden_operations: 0,
  cpu_model: "Test CPU", os: "linux", governor_or_power_mode: "performance",
  rust_version: "rustc 1.97.1", llvm_version: "21.1.4", target_triple: "x86_64-unknown-linux-gnu",
  target_features: "runtime-avx2,fma;baseline", profile: "release",
  background_load_note: "not-controlled; pre-run loadavg 0.1,0.1,0.1",
  candidate_commit: "0123456789abcdef0123456789abcdef01234567",
  missing_metadata: [], descriptive_only: true,
  statistical_method: "nearest-rank percentiles over per-block nanoseconds; one warmup pass and two measured rounds; descriptive only; no threshold"
}')

hoist=$(jq -cn --arg a "$digest_a" --arg b "$digest_b" '{
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
  cpu_model: "Test CPU", os: "linux", governor_or_power_mode: "performance",
  rust_version: "rustc 1.97.1", llvm_version: "21.1.4", target_triple: "x86_64-unknown-linux-gnu",
  target_features: "runtime-avx2,fma;baseline", profile: "release",
  background_load_note: "not-controlled; pre-run loadavg 0.1,0.1,0.1",
  candidate_commit: "0123456789abcdef0123456789abcdef01234567",
  missing_metadata: [], descriptive_only: true,
  statistical_method: "three arms alternated per observation; nearest-rank percentiles over per-block nanoseconds; paired delta is moving minus restated per observation; descriptive only; no threshold"
}')

expect_accept "$session" 'the base session record'
expect_accept "$hoist" 'the base hoist record'

# ---------------------------------------------------------------------------------------------
# Per-key structural mutations: every key is load-bearing in both directions.
# ---------------------------------------------------------------------------------------------
for base in "$session" "$hoist"; do
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
# Aggregate mutations.
# ---------------------------------------------------------------------------------------------
records=$(jq -cn --argjson session "$session" --argjson hoist "$hoist" \
    --arg a "$digest_a" --arg b "$digest_b" '
  def sessions: [
    {kind: "nine_track_baseline", tracks: 9, synthetic: true,
     fixture: "fixtures/session/v1/parametric-eq-nine-track.toml", digest: "1"},
    {kind: "nine_track_ragged_strip", tracks: 9, synthetic: true,
     fixture: "fixtures/session/v1/console-sixty-four-track.toml", digest: "2"},
    {kind: "sixty_four_track_console", tracks: 64, synthetic: false,
     fixture: "fixtures/session/v1/console-sixty-four-track.toml", digest: "3"},
    {kind: "one_twenty_eight_track_stretch", tracks: 128, synthetic: true,
     fixture: "fixtures/session/v1/console-sixty-four-track.toml", digest: "4"}
  ];
  def hoists: [
    {kind: "nine_track_ragged_strip", tracks: 9, digest: "5"},
    {kind: "sixty_four_track_console", tracks: 64, digest: "6"}
  ];
  [ (1, 2) as $round | (
      (sessions[] | . as $s | $session
        | .workload_kind = $s.kind | .tracks = $s.tracks
        | .synthetic_fixture = (if $s.kind == "nine_track_baseline" then false else $s.synthetic end)
        | .fixture_id = $s.fixture | .round = $round
        | .output_sha256 = ($a[0:63] + $s.digest)),
      (hoists[] | . as $h | $hoist
        | .workload_kind = $h.kind | .tracks = $h.tracks | .round = $round
        | .quiet_output_sha256 = ($a[0:63] + $h.digest)
        | .restated_output_sha256 = ($a[0:63] + $h.digest)
        | .moving_output_sha256 = $b)
  ) ]')

expect_aggregate_accept "$(printf '%s' "$records" | jq -c '.[]')" 'the twelve-record set'

expect_aggregate_reject "$(printf '%s' "$records" | jq -c 'del(.[0]) | .[]')" 'eleven records'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '. as $r | ($r + [$r[0]]) | .[]')" 'a duplicated record'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[6].round = 1 | .[]')" 'a workload measured twice in one round'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[0].cpu_model = "Another CPU" | .[]')" 'records from two hosts'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[0].candidate_commit = "ffffffffffffffffffffffffffffffffffffffff" | .[]')" 'records from two commits'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[0].backend = "Scalar" | .[]')" 'records from two backends'
# Round one and round two must render the same bytes: they are two measurements of one workload.
expect_aggregate_reject "$(printf '%s' "$records" | jq -c --arg c "$digest_c" '.[6].output_sha256 = $c | .[]')" 'a workload whose rounds rendered different output'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '[.[] | select(.record == "console_session")] | .[]')" 'a set with no hoist rows'
expect_aggregate_reject "$(printf '%s' "$records" | jq -c '.[0].workload_kind = "sixty_four_track_console" | .[0].tracks = 64 | .[0].synthetic_fixture = false | .[0].fixture_id = "fixtures/session/v1/console-sixty-four-track.toml" | .[]')" 'a set missing a declared workload'

if [[ "$failures" != 0 ]]; then
    printf 'console benchmark validator suite: %s FAILED case(s)\n' "$failures" >&2
    exit 1
fi
printf 'console benchmark validators: PASS (real runner/workload/timing invocations: 0/0/0)\n'
