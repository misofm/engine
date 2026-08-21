#!/usr/bin/env bash
# Validator/runner readiness only. This script never invokes the benchmark workload.
set -euo pipefail

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
runner="$script_directory/run-builtins-benchmark.sh"
fixture="$script_directory/fixtures/builtins-benchmark-validator-record.json"
command -v jq >/dev/null || { printf 'jq is required for benchmark validator tests\n' >&2; exit 1; }
for arguments in '--retry' '--rounds 2' extra; do
  if "$runner" $arguments >/dev/null 2>&1; then
    printf 'builtins benchmark runner accepted invalid arguments: %s\n' "$arguments" >&2; exit 1
  fi
done
[[ "$(rg -c 'cargo run --locked --release --quiet -p miso-engine-builtins-bench' "$runner")" == 1 ]] || {
  printf 'builtins runner must contain one workload launch\n' >&2; exit 1;
}
jq -e -L "$script_directory" 'include "builtins-benchmark-record-validator"; builtins_benchmark_record_valid' "$fixture" >/dev/null
for mutation in \
  '.round = 3' '.sample_rate_hz = 44100' '.p99_ns = 0' \
  '.render_allocations = 1' '.fixture_manifest_sha256 = "not-a-hash"' \
  '.workload_id = "unstable"' '.missing_metadata = ["z", "a"]'; do
  if jq "$mutation" "$fixture" | jq -e -L "$script_directory" 'include "builtins-benchmark-record-validator"; builtins_benchmark_record_valid' >/dev/null 2>&1; then
    printf 'builtins benchmark validator accepted mutation: %s\n' "$mutation" >&2; exit 1
  fi
done
jq -n --slurpfile base "$fixture" '
  ["full_chain_filters", "identity_chain", "matrix_ramp", "meter_success_full", "prepare_256_tracks"] as $kinds |
  [48000, 96000] as $rates |
  [1, 2] as $rounds |
  [$kinds[] as $kind | $rates[] as $rate | $rounds[] as $round |
    $base[0] + {
      workload_kind: $kind,
      workload_id: ("issue007." + $kind + "." + ($rate | tostring) + "hz.q128"),
      sample_rate_hz: $rate,
      round: $round,
      warmup_batches: (if $kind == "prepare_256_tracks" then 16 else 64 end),
      measured_batches: (if $kind == "prepare_256_tracks" then 128 else 512 end),
      operations_per_batch: (if $kind == "prepare_256_tracks" then 1 else 8 end),
      tracks: (if $kind == "prepare_256_tracks" then 256 else 1 end),
      meter_observers: (if $kind == "prepare_256_tracks" then 56 else 0 end),
      meter_queue_capacity: (if $kind == "prepare_256_tracks" then 4 else 0 end),
      render_scope: (if $kind == "prepare_256_tracks" then "not_applicable_preparation" else "render" end),
      render_allocations: (if $kind == "prepare_256_tracks" then "not_applicable" else 0 end),
      render_deallocations: (if $kind == "prepare_256_tracks" then "not_applicable" else 0 end),
      render_locks: (if $kind == "prepare_256_tracks" then "not_applicable" else 0 end),
      render_logs: (if $kind == "prepare_256_tracks" then "not_applicable" else 0 end),
      render_file_io: (if $kind == "prepare_256_tracks" then "not_applicable" else 0 end),
      render_network_io: (if $kind == "prepare_256_tracks" then "not_applicable" else 0 end),
      render_syscalls: (if $kind == "prepare_256_tracks" then "not_applicable" else 0 end)
    }
  ]' | jq -e -L "$script_directory" -f "$script_directory/builtins-benchmark-validator.jq" >/dev/null
printf 'builtins benchmark runner/validator readiness: PASS (workload launches: 0)\n'
