#!/usr/bin/env bash
# Exhaustive schema mutations and scratch-only runner lifecycle tests. No engine audio launches.
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
record=scripts/fixtures/scheduler-benchmark-validator-record.json
single=scripts/scheduler-benchmark-record-validator.jq
aggregate=scripts/scheduler-benchmark-validator.jq
jq -e -L scripts -f "$single" "$record" >/dev/null
scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT

reject_single() {
    local mutation=$1
    jq "$mutation" "$record" >"$scratch/mutated.json"
    if jq -e -L scripts -f "$single" "$scratch/mutated.json" >/dev/null; then
        printf 'scheduler benchmark validator accepted mutation: %s\n' "$mutation" >&2
        exit 1
    fi
}
while IFS= read -r key; do
    reject_single "del(.[\"$key\"])"
    reject_single ".[\"$key\"]={\"wrong_type\":true}"
done < <(jq -r 'keys[]' "$record")
while IFS= read -r mutation; do
    [[ -n "$mutation" ]] && reject_single "$mutation"
done <<'MUTATIONS'
.schema_version=2
.issue=39
.mode="bad"
.round=3
.sample_rate_hz=96000
.quantum_frames=64
.observations=999
.percentile_method="linear"
.units="ns"
.min=-1
.p50=51
.p95=19
.p99=29
.max=39
.selected_lanes=4
.worker_count=1
.wave_count=1
.unit_count=1
.partition_count=1
.retained_bytes=0
.output_hash="bad"
.render_errors=1
.coordinator_forbidden_total=1
.worker_forbidden_total=1
.descriptive_only=false
.candidate_sha256="bad"
.binary_sha256="bad"
.cpu_model=""
.os=""
.kernel=""
.rust_version=""
.llvm_version=""
.governor_or_power_mode=""
.extra=true
MUTATIONS

jq -n --slurpfile r "$record" '
  $r[0] as $x |
  [$x,($x|.round=2),
   ($x|.mode="two_lane"|.selected_lanes=2|.worker_count=1),
   ($x|.mode="two_lane"|.selected_lanes=2|.worker_count=1|.round=2),
   ($x|.mode="four_lane"|.selected_lanes=4|.worker_count=3),
   ($x|.mode="four_lane"|.selected_lanes=4|.worker_count=3|.round=2)]
' >"$scratch/aggregate.json"
jq -e -L scripts -f "$aggregate" "$scratch/aggregate.json" >/dev/null
reject_aggregate() {
    local mutation=$1
    jq "$mutation" "$scratch/aggregate.json" >"$scratch/aggregate-mutated.json"
    if jq -e -L scripts -f "$aggregate" "$scratch/aggregate-mutated.json" >/dev/null; then
        printf 'scheduler benchmark aggregate accepted mutation: %s\n' "$mutation" >&2
        exit 1
    fi
}
while IFS= read -r mutation; do
    [[ -n "$mutation" ]] && reject_aggregate "$mutation"
done <<'AGGREGATE_MUTATIONS'
del(.[5])
. + [.[0]]
.[0].round=2
.[0].candidate_sha256="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
.[0].binary_sha256="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
.[0].output_hash="ffffffffffffffff"
.[0].wave_count=9
.[0].unit_count=65
.[0].cpu_model="other"
.[0].os="other"
.[0].kernel="other"
AGGREGATE_MUTATIONS

runner=scripts/run-scheduler-benchmark.sh
bash -n "$runner"
for argument in --retry '--rounds 2' extra; do
    if bash "$runner" "$argument" >/dev/null 2>&1; then
        printf 'runner accepted argument: %s\n' "$argument" >&2
        exit 1
    fi
done

# The copied runner executes fake cargo/git/rustc and a synthetic JSON emitter. These processes do
# not enter engine DSP, so the scheduler benchmark/audio workload launch count remains exactly zero.
template="$scratch/runner-template"
mkdir -p "$template/scripts/fixtures" "$template/bin" "$template/artifacts/issue009"
cp "$runner" "$template/scripts/run-scheduler-benchmark.sh"
cp scripts/scheduler-benchmark-record-lib.jq scripts/scheduler-benchmark-record-validator.jq \
    scripts/scheduler-benchmark-validator.jq "$template/scripts/"
cp "$record" "$template/scripts/fixtures/scheduler-benchmark-validator-record.json"
cat >"$template/bin/git" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "$1" in
    status) exit 0 ;;
    rev-parse) printf '%s\n' 0123456789abcdef0123456789abcdef01234567 ;;
    *) exit 90 ;;
esac
EOF
cat >"$template/bin/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/release
cp "$MISO_ENGINE_TEST_FAKE_BENCH" target/release/miso_engine_bench
chmod 755 target/release/miso_engine_bench
EOF
cat >"$template/bin/rustc" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${MISO_ENGINE_TEST_RUSTC_PIPE_FAIL:-0}" == 1 && "${1:-}" == -vV ]]; then
    printf 'host: x86_64-unknown-linux-gnu\nLLVM version: 22.1.6\n'
    exit 73
fi
exec "$MISO_ENGINE_TEST_REAL_RUSTC" "$@"
EOF
cat >"$template/scripts/fixtures/fake-bench.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$MISO_ENGINE_BENCH_ROUND" >>"$MISO_ENGINE_TEST_LAUNCH_LOG"
mode=${MISO_ENGINE_TEST_BENCH_MODE:-success}
round=$MISO_ENGINE_BENCH_ROUND
if [[ "$mode" == interrupt && "$round" == warmup ]]; then kill -TERM "$PPID"; exit 143; fi
if [[ "$mode" == warmup_fail && "$round" == warmup ]]; then printf 'synthetic warmup failure\n' >&2; exit 71; fi
if [[ "$mode" == round1_fail && "$round" == 1 ]]; then printf '{"partial":"round1"}\n'; exit 72; fi
if [[ "$mode" == round2_fail && "$round" == 2 ]]; then printf '{"partial":"round2"}\n'; exit 74; fi
if [[ "$round" == warmup ]]; then round=1; fi
if [[ "$mode" == invalid ]]; then printf '{}\n{}\n{}\n'; exit 0; fi
root=$(cd "$(dirname "$0")/../.." && pwd)
base="$root/scripts/fixtures/scheduler-benchmark-validator-record.json"
common=(--argjson round "$round" --arg candidate "$MISO_ENGINE_BENCH_CANDIDATE_SHA256" --arg binary "$MISO_ENGINE_BENCH_BINARY_SHA256")
jq -c "${common[@]}" '.round=$round|.candidate_sha256=$candidate|.binary_sha256=$binary' "$base"
jq -c "${common[@]}" '.round=$round|.candidate_sha256=$candidate|.binary_sha256=$binary|.mode="two_lane"|.selected_lanes=2|.worker_count=1' "$base"
jq -c "${common[@]}" '.round=$round|.candidate_sha256=$candidate|.binary_sha256=$binary|.mode="four_lane"|.selected_lanes=4|.worker_count=3' "$base"
EOF
chmod 755 "$template/bin/git" "$template/bin/cargo" "$template/bin/rustc" \
    "$template/scripts/fixtures/fake-bench.sh"

case_number=0
new_case() {
    local name=$1
    case_number=$((case_number + 1))
    case_root="$scratch/case-$case_number-$name"
    cp -a "$template" "$case_root"
    launch_log="$case_root/synthetic-launches.log"
}
run_case() {
    local mode=$1
    MISO_ENGINE_TEST_BENCH_MODE="$mode" MISO_ENGINE_TEST_LAUNCH_LOG="$launch_log" \
    MISO_ENGINE_TEST_FAKE_BENCH="$case_root/scripts/fixtures/fake-bench.sh" \
    MISO_ENGINE_TEST_REAL_RUSTC="$(command -v rustc)" PATH="$case_root/bin:$PATH" \
    bash "$case_root/scripts/run-scheduler-benchmark.sh"
}
expect_failure_reason() {
    local reason=$1
    jq -e --arg reason "$reason" '.status == "FAIL" and .reason == $reason' \
        "$case_root/artifacts/issue009/scheduler-benchmark.disposition.json" >/dev/null
}

new_case overwrite
printf 'protected raw bytes\n' >"$case_root/artifacts/issue009/scheduler-benchmark.raw.jsonl"
if run_case success >/dev/null 2>&1; then printf 'runner overwrote a pre-existing raw artifact\n' >&2; exit 1; fi
[[ ! -e "$launch_log" ]]
[[ "$(<"$case_root/artifacts/issue009/scheduler-benchmark.raw.jsonl")" == 'protected raw bytes' ]]

new_case warmup-failure
if run_case warmup_fail >/dev/null 2>&1; then printf 'runner swallowed warmup failure\n' >&2; exit 1; fi
expect_failure_reason warmup_failed
[[ "$(wc -l <"$launch_log")" == 1 ]]
[[ ! -e "$case_root/artifacts/issue009/scheduler-benchmark.raw.jsonl" ]]

new_case round1-failure
if run_case round1_fail >/dev/null 2>&1; then printf 'runner swallowed measured round-1 failure\n' >&2; exit 1; fi
expect_failure_reason round_1_failed
[[ "$(wc -l <"$launch_log")" == 2 ]]
grep -Fqx '{"partial":"round1"}' "$case_root/artifacts/issue009/scheduler-benchmark.raw.jsonl"
[[ ! -e "$case_root/artifacts/issue009/scheduler-benchmark.accepted.jsonl" ]]

new_case round2-failure
if run_case round2_fail >/dev/null 2>&1; then printf 'runner swallowed measured round-2 failure\n' >&2; exit 1; fi
expect_failure_reason round_2_failed
[[ "$(wc -l <"$launch_log")" == 3 ]]
[[ "$(tail -n 1 "$case_root/artifacts/issue009/scheduler-benchmark.raw.jsonl")" == '{"partial":"round2"}' ]]

new_case validation-failure
if run_case invalid >/dev/null 2>&1; then printf 'runner promoted invalid synthetic output\n' >&2; exit 1; fi
expect_failure_reason validation_failed
[[ "$(wc -l <"$launch_log")" == 3 ]]
[[ "$(wc -l <"$case_root/artifacts/issue009/scheduler-benchmark.raw.jsonl")" == 6 ]]
[[ ! -e "$case_root/artifacts/issue009/scheduler-benchmark.accepted.jsonl" ]]

new_case pipe-failure
if MISO_ENGINE_TEST_RUSTC_PIPE_FAIL=1 run_case success >/dev/null 2>&1; then printf 'runner swallowed metadata pipeline failure\n' >&2; exit 1; fi
expect_failure_reason metadata_failed
[[ ! -e "$launch_log" ]]

new_case interruption
if run_case interrupt >/dev/null 2>&1; then printf 'runner swallowed interruption\n' >&2; exit 1; fi
expect_failure_reason interrupted
[[ "$(wc -l <"$launch_log")" == 1 ]]
[[ ! -e "$case_root/artifacts/issue009/scheduler-benchmark.accepted.jsonl" ]]

new_case success
accepted_path=$(run_case success)
raw_path="$case_root/artifacts/issue009/scheduler-benchmark.raw.jsonl"
[[ "$accepted_path" == "$case_root/artifacts/issue009/scheduler-benchmark.accepted.jsonl" ]]
cmp -s "$raw_path" "$accepted_path"
[[ "$(wc -l <"$launch_log")" == 3 ]]
raw_sha=$(sha256sum "$raw_path" | awk '{print $1}')
accepted_sha=$(sha256sum "$accepted_path" | awk '{print $1}')
jq -e --arg raw "$raw_sha" --arg accepted "$accepted_sha" '
    .status == "PASS" and .reason == "complete" and
    .runner_invocations == 1 and .workload_process_launches == 3 and
    .warmup_launches == 1 and .measured_rounds_completed == 2 and
    .raw_sha256 == $raw and .accepted_sha256 == $accepted and .raw_sha256 == .accepted_sha256
' "$case_root/artifacts/issue009/scheduler-benchmark.disposition.json" >/dev/null
if run_case success >/dev/null 2>&1; then printf 'runner resumed or overwrote a completed lifecycle\n' >&2; exit 1; fi
[[ "$(wc -l <"$launch_log")" == 3 ]]

rg -q 'set -euo pipefail' "$runner"
rg -q 'set -o noclobber' "$runner"
[[ "$(rg -c '^run_round (warmup|1|2) ' "$runner")" == 3 ]]
[[ "$(rg -c '^run_round [12] ' "$runner")" == 2 ]]
! rg -n 'retry|resume|eval|source ' "$runner" >/dev/null
printf 'scheduler benchmark validators/lifecycle: PASS (audio workload launches: 0)\n'
