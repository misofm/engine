#!/usr/bin/env bash
# Schema, aggregate and runner lifecycle checks only; audio workload launches remain zero.
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
record=scripts/fixtures/scheduler-benchmark-validator-record.json
single=scripts/scheduler-benchmark-record-validator.jq
aggregate=scripts/scheduler-benchmark-validator.jq
jq -e -L scripts -f "$single" "$record" >/dev/null
scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT
reject() {
    local mutation=$1
    jq "$mutation" "$record" >"$scratch/mutated.json"
    if jq -e -L scripts -f "$single" "$scratch/mutated.json" >/dev/null; then
        printf 'scheduler benchmark validator accepted mutation: %s\n' "$mutation" >&2
        exit 1
    fi
}
for mutation in '.mode="bad"' '.round=3' '.selected_lanes=4' '.render_errors=1' '.worker_forbidden_total=1' '.output_hash="bad"' 'del(.binary_sha256)' '.extra=true'; do reject "$mutation"; done
jq -n --slurpfile r "$record" '$r[0] as $x | [$x,($x|.round=2),($x|.mode="two_lane"|.selected_lanes=2|.worker_count=1),($x|.mode="two_lane"|.selected_lanes=2|.worker_count=1|.round=2),($x|.mode="four_lane"|.selected_lanes=4|.worker_count=3),($x|.mode="four_lane"|.selected_lanes=4|.worker_count=3|.round=2)]' >"$scratch/aggregate.json"
jq -e -L scripts -f "$aggregate" "$scratch/aggregate.json" >/dev/null
for mutation in 'del(.[5])' '.[0].round=2' '.[4].output_hash="ffffffffffffffff"'; do
    jq "$mutation" "$scratch/aggregate.json" >"$scratch/aggregate-mutated.json"
    if jq -e -L scripts -f "$aggregate" "$scratch/aggregate-mutated.json" >/dev/null; then
        printf 'scheduler benchmark aggregate accepted mutation: %s\n' "$mutation" >&2
        exit 1
    fi
done
runner=scripts/run-scheduler-benchmark.sh
bash -n "$runner"
for argument in --retry '--rounds 2' extra; do
    if bash "$runner" "$argument" >/dev/null 2>&1; then printf 'runner accepted argument: %s\n' "$argument" >&2; exit 1; fi
done
template="$scratch/template"
mkdir -p "$template/scripts" "$template/artifacts/issue009"
cp "$runner" "$template/scripts/run-scheduler-benchmark.sh"
touch "$template/artifacts/issue009/scheduler-benchmark.raw.jsonl"
if bash "$template/scripts/run-scheduler-benchmark.sh" >/dev/null 2>&1; then
    printf 'runner overwrote an existing raw artifact\n' >&2
    exit 1
fi
[[ $(rg -c '^run_round warmup ' "$runner") == 1 ]]
[[ $(rg -c '^run_round 1 ' "$runner") == 1 ]]
[[ $(rg -c '^run_round 2 ' "$runner") == 1 ]]
! rg -n 'retry|for[[:space:]].*round|while[[:space:]].*round' "$runner" >/dev/null
printf 'scheduler benchmark preflight mutations: PASS (workload_launches=0)\n'
