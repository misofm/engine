#!/usr/bin/env bash
# Scratch-only checks for Issue-880 MQ-1 argument, schema, and persistence preflight.
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
# shellcheck source=scripts/issue880-mq1-benchmark-lib.sh
source scripts/issue880-mq1-benchmark-lib.sh

scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT
runner=scripts/run-issue880-mq1-benchmark.sh
fixture=scripts/fixtures/issue880-mq1-record.json
validator=scripts/issue880-mq1-record-validator.jq

if output=$(bash "$runner" --bad --output "$scratch/bad.json" 2>&1); then
    printf 'MQ-1 runner accepted an unknown option\n' >&2
    exit 1
else
    status=$?
fi
[[ "$status" == 2 && "$output" == *usage:* ]]

jq -e -L scripts -f "$validator" "$fixture" >/dev/null
if jq '.measured_rounds_per_arm = 3' "$fixture" | jq -e -L scripts -f "$validator" >/dev/null; then
    printf 'MQ-1 record validator accepted a wrong round count\n' >&2
    exit 1
fi

printf 'persist probe\n' >"$scratch/source"
persist_no_clobber "$scratch/source" "$scratch/persisted"
cmp -s "$scratch/source" "$scratch/persisted"
printf 'keep existing bytes\n' >"$scratch/occupied"
if persist_no_clobber "$scratch/source" "$scratch/occupied" 2>/dev/null; then
    printf 'MQ-1 persistence helper overwrote an existing record\n' >&2
    exit 1
fi
[[ "$(<"$scratch/occupied")" == 'keep existing bytes' ]]

if run_and_capture "$scratch/exit-status" bash -c 'printf captured; exit 37'; then
    printf 'MQ-1 capture helper swallowed a command failure\n' >&2
    exit 1
else
    status=$?
fi
[[ "$status" == 37 && "$(<"$scratch/exit-status")" == captured ]]

bash "$runner" --preflight --output "$scratch/record.json" >"$scratch/preflight.out"
rg -q 'workload launches 0' "$scratch/preflight.out"
[[ ! -e "$scratch/record.json" && ! -e "$scratch/record.json.raw.log" ]]

for suffix in '' '.raw.log' '.failure.json'; do
    target="$scratch/no-clobber.json$suffix"
    printf 'existing\n' >"$target"
    if bash "$runner" --preflight --output "$scratch/no-clobber.json" >/dev/null 2>&1; then
        printf 'MQ-1 runner accepted existing output path %s\n' "$target" >&2
        exit 1
    else
        status=$?
    fi
    [[ "$status" == 1 && "$(<"$target")" == existing ]]
    rm -- "$target"
done

printf 'Issue-880 MQ-1 preflight self-test: PASS (timed workload launches 0)\n'
