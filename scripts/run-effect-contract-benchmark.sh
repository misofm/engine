#!/usr/bin/env bash
set -euo pipefail
[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
command -v jq >/dev/null 2>&1 || { printf 'jq is required\n' >&2; exit 1; }
mkdir -p target/issue11
artifact="target/issue11/effect-contract-benchmark.jsonl"
cargo run --locked --release -q -p bench -- effect-contract --benchmark-two-rounds >"$artifact"
[[ "$(wc -l <"$artifact" | tr -d ' ')" == 12 ]] || { printf 'benchmark record count mismatch\n' >&2; exit 1; }
jq -e -s '
  length == 12 and
  (group_by(.workload) | length == 6 and all(length == 2 and (map(.round) | sort == [1,2]))) and
  all(.schema_version == 1 and .observations > 0 and .units == "ns" and
      .min <= .p50 and .p50 <= .p95 and .p95 <= .p99 and .p99 <= .p99_9 and .p99_9 <= .max and
      (if (.workload == "scalar_noop" or .workload == "bank4_noop" or .workload == "bank8_noop")
       then (.allocations == 0 and .deallocations == 0) else true end))
' "$artifact" >/dev/null
printf '%s\n' "$artifact"
