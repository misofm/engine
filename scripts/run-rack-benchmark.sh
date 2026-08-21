#!/usr/bin/env bash
# Exactly one authorized descriptive Issue-008 invocation. Never retry or tune.
set -euo pipefail
[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
script_directory=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
repository_root=$(cd "$script_directory/.." && pwd)
output="$repository_root/target/issue8/rack-benchmark.jsonl"
[[ ! -e "$output" ]] || { printf 'refusing to overwrite an existing issue-008 benchmark artifact\n' >&2; exit 1; }
mkdir -p "$repository_root/target/issue8"
run_workload() {
  (cd "$repository_root" && cargo run --locked --release --quiet -p miso-engine-rack-bench)
}
# The runner owns one untimed warmup and exactly two measured rounds.
run_workload >/dev/null # warmup
for round in 1 2; do
  run_workload | sed "s/}/,\"round\":$round}/"
done > "$output"
[[ "$(wc -l < "$output")" == 6 ]] || { printf 'unexpected record count\n' >&2; exit 1; }
printf '%s\n' "$output"
# workload_launches=0 applies to preflight only.
