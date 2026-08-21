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
for mutation in '.round = 3' '.timing_ns = -1' '.allocations = 1' '.descriptive_only = false'; do
  if jq "$mutation" "$fixture" | jq -e -L "$script_directory" 'include "builtins-benchmark-record-validator"; builtins_benchmark_record_valid' >/dev/null 2>&1; then
    printf 'builtins benchmark validator accepted mutation: %s\n' "$mutation" >&2; exit 1
  fi
done
jq -n --slurpfile base "$fixture" '[range(0; 20) as $index | $base[0] + {workload: ["input_identity_1t_128","input_filters_1t_128","fader_mute_1t_128","matrix_identity_1t_128","matrix_ramp_1t_128","meter_success_7taps_128","meter_full_7taps_128","combined_1t_128","combined_4t_128","prepare_65537t"][$index % 10], round: (($index / 10 | floor) + 1)}]' | jq -e -L "$script_directory" -f "$script_directory/builtins-benchmark-validator.jq" >/dev/null
printf 'builtins benchmark runner/validator readiness: PASS (workload launches: 0)\n'
