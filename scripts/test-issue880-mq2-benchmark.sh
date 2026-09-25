#!/usr/bin/env bash
# Scratch-only checks for MQ-2 argument, schema, marker extraction and persistence preflight.
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"

scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT
runner=scripts/run-issue880-mq2-benchmark.sh
fixture=scripts/fixtures/issue880-mq2-record.json
validator=scripts/issue880-mq2-record-validator.jq

if output=$(bash "$runner" --bad --output "$scratch/bad.json" 2>&1); then
    printf 'MQ-2 runner accepted an unknown option\n' >&2
    exit 1
else
    status=$?
fi
[[ "$status" == 2 && "$output" == *usage:* ]]

jq -e -f "$validator" "$fixture" >/dev/null
if jq '.arms[1].rate_coefficient_calls_per_block = 1' "$fixture" | jq -e -f "$validator" >/dev/null; then
    printf 'MQ-2 record validator accepted a wrong derived call count\n' >&2
    exit 1
fi

bash "$runner" --preflight --output "$scratch/record.json" >"$scratch/preflight.out"
rg -q 'timed workload launches 0' "$scratch/preflight.out"
[[ ! -e "$scratch/record.json" && ! -e "$scratch/record.json.raw.log" ]]

for suffix in '' '.raw.log' '.failure.json'; do
    target="$scratch/no-clobber.json$suffix"
    printf 'existing\n' >"$target"
    if bash "$runner" --preflight --output "$scratch/no-clobber.json" >/dev/null 2>&1; then
        printf 'MQ-2 runner accepted existing output path %s\n' "$target" >&2
        exit 1
    else
        status=$?
    fi
    [[ "$status" == 1 && "$(<"$target")" == existing ]]
    rm -- "$target"
done

printf 'Issue-880 MQ-2 preflight self-test: PASS (timed workload launches 0)\n'
