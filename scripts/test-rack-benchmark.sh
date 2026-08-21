#!/usr/bin/env bash
# Preflight mutation checks only; this script never invokes the timed workload.
set -euo pipefail
script_directory=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
runner="$script_directory/run-rack-benchmark.sh"
for arguments in '--retry' '--rounds 2' extra; do
  if "$runner" $arguments >/dev/null 2>&1; then
    printf 'rack benchmark runner accepted invalid arguments: %s\n' "$arguments" >&2
    exit 1
  fi
done
[[ "$(rg -c 'cargo run --locked --release --quiet -p miso-engine-rack-bench' "$runner")" == 1 ]] || { printf 'runner must have exactly one workload launch\n' >&2; exit 1; }
for required in 'workload_launches=0' 'refusing to overwrite' 'warmup' 'round'; do
  rg -q "$required" "$runner" || { printf 'runner omitted required preflight/record token: %s\n' "$required" >&2; exit 1; }
done
printf 'rack benchmark runner readiness: PASS (workload launches: 0)\n'
