#!/usr/bin/env bash
# Schema, bounded mutation and runner-source negative checks only: zero workload launches.
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
record=scripts/fixtures/rack-benchmark-validator-record.json
single_validator=scripts/rack-benchmark-record-validator.jq
aggregate_validator=scripts/rack-benchmark-validator.jq
jq -e -L scripts -f "$single_validator" "$record" >/dev/null
scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT
for mutation in '.schema_version=1' '.workload_kind="bad"' '.round=3' '.p95_ns_per_frame=1' '.fixture_sha256="bad"' '.render_allocations=1' '.forbidden_operation_total=1' '.cpu_model="unknown"' 'del(.output_sha256)' '.extra=true'; do
    jq "$mutation" "$record" >"$scratch/mutated.json"
    if jq -e -L scripts -f "$single_validator" "$scratch/mutated.json" >/dev/null; then
        printf 'rack benchmark validator accepted mutation: %s\n' "$mutation" >&2
        exit 1
    fi
done
# Every schema key is individually required and has a bounded wrong-type rejection probe.
while IFS= read -r key; do
    jq --arg key "$key" 'del(.[$key])' "$record" >"$scratch/mutated.json"
    if jq -e -L scripts -f "$single_validator" "$scratch/mutated.json" >/dev/null; then
        printf 'rack benchmark validator accepted missing key: %s\n' "$key" >&2
        exit 1
    fi
    jq --arg key "$key" '.[$key] = []' "$record" >"$scratch/mutated.json"
    if jq -e -L scripts -f "$single_validator" "$scratch/mutated.json" >/dev/null; then
        printf 'rack benchmark validator accepted wrong key type: %s\n' "$key" >&2
        exit 1
    fi
done < <(jq -r 'keys[]' "$record")
jq -n --slurpfile scalar "$record" '$scalar[0] as $r | [$r,($r|.round=2),($r|.workload_kind="host_selected_eight_track_bank"|.workload_id="issue038.host_selected_eight_track_bank.48000hz.q128"|.bank_backend="X86Avx2"|.bank_width=8|.bank_count=1|.scalar_tail_count=0),($r|.round=2|.workload_kind="host_selected_eight_track_bank"|.workload_id="issue038.host_selected_eight_track_bank.48000hz.q128"|.bank_backend="X86Avx2"|.bank_width=8|.bank_count=1|.scalar_tail_count=0),($r|.workload_kind="mixed_twelve_track_graph"|.workload_id="issue038.mixed_twelve_track_graph.48000hz.q128"|.tracks=12|.bank_backend="X86Avx2"|.bank_width=8|.bank_count=1|.scalar_tail_count=2|.scalar_fallback_count=2|.identity_lane_count=2),($r|.round=2|.workload_kind="mixed_twelve_track_graph"|.workload_id="issue038.mixed_twelve_track_graph.48000hz.q128"|.tracks=12|.bank_backend="X86Avx2"|.bank_width=8|.bank_count=1|.scalar_tail_count=2|.scalar_fallback_count=2|.identity_lane_count=2)]' >"$scratch/aggregate.json"
jq -e -L scripts -f "$aggregate_validator" "$scratch/aggregate.json" >/dev/null
for mutation in '.[0].round=2' 'del(.[5])' '.[0].candidate_commit_sha256="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"' '.[0].output_sha256="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"'; do
    jq "$mutation" "$scratch/aggregate.json" >"$scratch/mutated.json"
    if jq -e -L scripts -f "$aggregate_validator" "$scratch/mutated.json" >/dev/null; then
        printf 'rack benchmark aggregate accepted mutation: %s\n' "$mutation" >&2
        exit 1
    fi
done
runner=scripts/run-rack-benchmark.sh
for argument in --retry '--rounds 2' extra; do
    if bash "$runner" "$argument" >/dev/null 2>&1; then
        printf 'runner accepted argument: %s\n' "$argument" >&2
        exit 1
    fi
done
rg -q 'set -euo pipefail' "$runner"
rg -q 'refusing to overwrite issue-038 artifact' "$runner"
rg -q 'run_round warmup' "$runner"
[[ "$(rg -c 'run_round [12]' "$runner")" == 2 ]]
[[ "$(rg -c 'MISO_ENGINE_RACK_BENCH_ROUND' "$runner")" == 1 ]]
printf 'rack benchmark preflight mutations: PASS (workload launches: 0)\n'
