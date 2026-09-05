#!/usr/bin/env bash
# Exhaustive bounded schema mutations and scratch-only runner lifecycle tests. No audio launches.
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
record=scripts/fixtures/rack-benchmark-validator-record.json
single_validator=scripts/rack-benchmark-record-validator.jq
aggregate_validator=scripts/rack-benchmark-validator.jq
jq -e -L scripts -f "$single_validator" "$record" >/dev/null
scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT

fixture_checker=scripts/check-rack-benchmark-fixture.sh
fixture_copy="$scratch/fixture"
cp -a fixtures/rack/issue038-v1 "$fixture_copy"
bash "$fixture_checker" "$fixture_copy"
cp "$fixture_copy/workloads.toml" "$scratch/workloads.original"
printf 'mutation\n' >>"$fixture_copy/workloads.toml"
if bash "$fixture_checker" "$fixture_copy" >/dev/null 2>&1; then
    printf 'Issue-038 fixture checker accepted content corruption\n' >&2
    exit 1
fi
cp "$scratch/workloads.original" "$fixture_copy/workloads.toml"
cp "$fixture_copy/MANIFEST.tsv" "$scratch/manifest.original"
printf 'mutation\n' >>"$fixture_copy/MANIFEST.tsv"
if bash "$fixture_checker" "$fixture_copy" >/dev/null 2>&1; then
    printf 'Issue-038 fixture checker accepted manifest corruption\n' >&2
    exit 1
fi
cp "$scratch/manifest.original" "$fixture_copy/MANIFEST.tsv"
printf 'unlisted\n' >"$fixture_copy/unlisted"
if bash "$fixture_checker" "$fixture_copy" >/dev/null 2>&1; then
    printf 'Issue-038 fixture checker accepted an unlisted file\n' >&2
    exit 1
fi
rm "$fixture_copy/unlisted"
rm "$fixture_copy/workloads.toml"
if bash "$fixture_checker" "$fixture_copy" >/dev/null 2>&1; then
    printf 'Issue-038 fixture checker accepted a missing payload\n' >&2
    exit 1
fi
cp "$scratch/workloads.original" "$fixture_copy/workloads.toml"

# Exercise the two selected producer status controls against complete real output plus an
# injected command failure.  The delegates are fixed system commands; the test mode is part of
# the registered MISO_ENGINE_TEST_BENCH_MODE shim vocabulary.
scan_bin="$scratch/scan-bin"
mkdir -p "$scan_bin"
cat >"$scan_bin/find" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
/usr/bin/find "$@"
if [[ "${MISO_ENGINE_TEST_BENCH_MODE:-}" == discovery_fail ]]; then
    printf 'FIND_SENTINEL delegate_status=0 rows=2 mode=discovery_fail\n' >&2
    exit 73
fi
EOF
cat >"$scan_bin/wc" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
/usr/bin/wc "$@"
if [[ "${MISO_ENGINE_TEST_BENCH_MODE:-}" == payload_wc_fail && "${1:-}" == -c ]]; then
    printf 'WC_SENTINEL delegate_status=0 value=456 mode=payload_wc_fail\n' >&2
    exit 74
fi
EOF
chmod 755 "$scan_bin/find" "$scan_bin/wc"
assert_checker_rejects_producer_failure() {
    local mode=$1 diagnostic=$2
    local output status
    if output=$(MISO_ENGINE_TEST_BENCH_MODE="$mode" PATH="$scan_bin:$PATH" bash "$fixture_checker" "$fixture_copy" 2>&1); then
        printf 'checker swallowed %s producer failure\n%s\n' "$mode" "$output" >&2
        return 97
    else
        status=$?
    fi
    [[ "$status" != 0 ]] || return 96
    [[ "$output" == *"$diagnostic"* ]] || { printf 'wrong %s producer diagnostic\n%s\n' "$mode" "$output" >&2; return 96; }
    [[ "$output" == *SENTINEL* ]] || { printf 'missing %s producer sentinel\n%s\n' "$mode" "$output" >&2; return 96; }
}
assert_checker_rejects_producer_failure discovery_fail 'Issue-038 fixture discovery failed (status 73)'
assert_checker_rejects_producer_failure payload_wc_fail 'Issue-038 workload length wc failed (status 74)'

counter_checker_mutant() {
    local label=$1 mutation=$2 mode=$3
    local mutant="$scratch/checker-$label.sh" output status
    sed "$mutation" "$fixture_checker" >"$mutant"
    chmod 755 "$mutant"
    if output=$(MISO_ENGINE_TEST_BENCH_MODE="$mode" PATH="$scan_bin:$PATH" bash "$mutant" "$fixture_copy" 2>&1); then
        printf 'ASSERT %s unexpected success\n' "$label" >&2
        return 97
    else
        status=$?
    fi
    printf 'ASSERT %s wrong rejection status=%s\n%s\n' "$label" "$status" "$output" >&2
    return 96
}
discovery_line=$(rg -n -F '    discovery_status=$?' "$fixture_checker" | cut -d: -f1)
if counter_checker_mutant discovery-status "${discovery_line}c\\    :" discovery_fail >/dev/null 2>&1; then
    exit 96
else
    mutant_status=$?
fi
[[ "$mutant_status" == 97 ]] || { printf 'discovery producer mutant assertion status %s, expected 97\n' "$mutant_status" >&2; exit 96; }
payload_wc_line=$(rg -n -F '    payload_wc_status=$?' "$fixture_checker" | cut -d: -f1)
if counter_checker_mutant payload-wc-status "${payload_wc_line}c\\    :" payload_wc_fail >/dev/null 2>&1; then
    exit 96
else
    mutant_status=$?
fi
[[ "$mutant_status" == 97 ]] || { printf 'payload wc producer mutant assertion status %s, expected 97\n' "$mutant_status" >&2; exit 96; }

reject_single() {
    local mutation=$1
    jq "$mutation" "$record" >"$scratch/mutated.json"
    if jq -e -L scripts -f "$single_validator" "$scratch/mutated.json" >/dev/null; then
        printf 'rack benchmark validator accepted mutation: %s\n' "$mutation" >&2
        exit 1
    fi
}

# Every schema key is individually required and rejects an object in place of its declared type.
while IFS= read -r key; do
    reject_single "del(.[\"$key\"])"
    reject_single ".[\"$key\"]={\"wrong_type\":true}"
done < <(jq -r 'keys[]' "$record")

# Each semantic family has a bounded valid-type mutation, including matched nonzero audit totals.
while IFS= read -r mutation; do
    [[ -n "$mutation" ]] && reject_single "$mutation"
done <<'MUTATIONS'
.schema_version=0
.issue=8
.workload_kind="bad"
.workload_id="issue038.bad.48000hz.q128"
.round=3
.sample_rate_hz=96000
.quantum_frames=64
.tracks=7
.bank_backend="Simd8"
.bank_width=8
.bank_count=1
.scalar_tail_count=7
.scalar_fallback_count=1
.identity_lane_count=1
.observations=999
.percentile_method="linear"
.units="ns"
.min_ns_per_frame=-1
.p50_ns_per_frame=1.5
.p95_ns_per_frame=1
.p99_ns_per_frame=2
.p99_9_ns_per_frame=3
.max_ns_per_frame=4
.descriptive_only=false
.candidate_commit_sha256="bad"
.binary_sha256="bad"
.fixture_id="wrong"
.fixture_sha256="bad"
.input_sha256="bad"
.output_sha256="bad"
.render_errors=1
.render_allocations=1|.forbidden_operation_total=1
.render_deallocations=1|.forbidden_operation_total=1
.render_locks=1|.forbidden_operation_total=1
.render_feature_detection_calls=1|.forbidden_operation_total=1
.render_logs=1|.forbidden_operation_total=1
.render_file_io=1|.forbidden_operation_total=1
.render_network_io=1|.forbidden_operation_total=1
.render_syscalls=1|.forbidden_operation_total=1
.render_panic_unwinds=1|.forbidden_operation_total=1
.forbidden_operation_total=1
.cpu_model=""
.cpu_model="unknown"
.cpu_model="default"
.missing_metadata|=reverse
.missing_metadata += ["architecture"]
.architecture="x86_64"
.extra=true
MUTATIONS

jq -n --slurpfile scalar "$record" '
  $scalar[0] as $r |
  [$r,
   ($r|.round=2),
   ($r|.workload_kind="host_selected_eight_track_bank"|.workload_id="issue038.host_selected_eight_track_bank.48000hz.q128"|.bank_backend="Simd8"|.bank_width=8|.bank_count=1|.scalar_tail_count=0|.output_sha256="5555555555555555555555555555555555555555555555555555555555555555"),
   ($r|.round=2|.workload_kind="host_selected_eight_track_bank"|.workload_id="issue038.host_selected_eight_track_bank.48000hz.q128"|.bank_backend="Simd8"|.bank_width=8|.bank_count=1|.scalar_tail_count=0|.output_sha256="5555555555555555555555555555555555555555555555555555555555555555"),
   ($r|.workload_kind="mixed_twelve_track_graph"|.workload_id="issue038.mixed_twelve_track_graph.48000hz.q128"|.tracks=12|.bank_backend="Simd8"|.bank_width=8|.bank_count=1|.scalar_tail_count=2|.scalar_fallback_count=2|.identity_lane_count=2|.input_sha256="6666666666666666666666666666666666666666666666666666666666666666"|.output_sha256="7777777777777777777777777777777777777777777777777777777777777777"),
   ($r|.round=2|.workload_kind="mixed_twelve_track_graph"|.workload_id="issue038.mixed_twelve_track_graph.48000hz.q128"|.tracks=12|.bank_backend="Simd8"|.bank_width=8|.bank_count=1|.scalar_tail_count=2|.scalar_fallback_count=2|.identity_lane_count=2|.input_sha256="6666666666666666666666666666666666666666666666666666666666666666"|.output_sha256="7777777777777777777777777777777777777777777777777777777777777777")]
' >"$scratch/aggregate.json"
jq -e -L scripts -f "$aggregate_validator" "$scratch/aggregate.json" >/dev/null

reject_aggregate() {
    local mutation=$1
    jq "$mutation" "$scratch/aggregate.json" >"$scratch/aggregate-mutated.json"
    if jq -e -L scripts -f "$aggregate_validator" "$scratch/aggregate-mutated.json" >/dev/null; then
        printf 'rack benchmark aggregate accepted mutation: %s\n' "$mutation" >&2
        exit 1
    fi
}
while IFS= read -r mutation; do
    [[ -n "$mutation" ]] && reject_aggregate "$mutation"
done <<'AGGREGATE_MUTATIONS'
.[0].round=2
del(.[5])
. + [.[0]]
.[0].candidate_commit_sha256="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
.[0].binary_sha256="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
.[0].fixture_sha256="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
.[0].input_sha256="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
.[0].output_sha256="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
.[2].bank_backend="Simd4"
.[0].architecture="x86_64"|.[0].missing_metadata -= ["architecture"]
AGGREGATE_MUTATIONS

runner=scripts/run-rack-benchmark.sh
for argument in --retry '--rounds 2' extra; do
    if bash "$runner" "$argument" >/dev/null 2>&1; then
        printf 'runner accepted argument: %s\n' "$argument" >&2
        exit 1
    fi
done

# Build a hermetic scratch repository with fake cargo/git/rustc and a synthetic record emitter.
# These process launches never enter engine DSP and therefore keep audio workload_launches at zero.
template="$scratch/runner-template"
mkdir -p "$template/scripts/fixtures" "$template/bin" "$template/fixtures/rack/issue038-v1"
cp "$runner" "$template/scripts/run-rack-benchmark.sh"
cp scripts/rack-benchmark-record-lib.jq scripts/rack-benchmark-record-validator.jq \
    scripts/rack-benchmark-validator.jq "$template/scripts/"
cp "$record" "$template/scripts/fixtures/rack-benchmark-validator-record.json"
cp fixtures/rack/issue038-v1/MANIFEST.tsv "$template/fixtures/rack/issue038-v1/MANIFEST.tsv"

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
cp "$MISO_ENGINE_TEST_FAKE_BENCH" target/release/bench
chmod 755 target/release/bench
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
if [[ "$mode" == interrupt && "$round" == warmup ]]; then
    kill -TERM "$PPID"
    exit 143
fi
if [[ "$mode" == warmup_fail && "$round" == warmup ]]; then
    printf 'synthetic warmup failure\n' >&2
    exit 71
fi
if [[ "$mode" == round1_fail && "$round" == 1 ]]; then
    printf '{"partial":"round1"}\n'
    exit 72
fi
if [[ "$mode" == round2_fail && "$round" == 2 ]]; then
    printf '{"partial":"round2"}\n'
    exit 74
fi
if [[ "$round" == warmup ]]; then
    round=1
fi
if [[ "$mode" == invalid ]]; then
    printf '{}\n{}\n{}\n'
    exit 0
fi
root=$(cd "$(dirname "$0")/../.." && pwd)
base="$root/scripts/fixtures/rack-benchmark-validator-record.json"
common=(--argjson round "$round" --arg candidate "$MISO_ENGINE_BENCH_CANDIDATE_SHA256" --arg binary "$MISO_ENGINE_BENCH_BINARY_SHA256")
jq -c "${common[@]}" '.round=$round|.candidate_commit_sha256=$candidate|.binary_sha256=$binary' "$base"
jq -c "${common[@]}" '.round=$round|.candidate_commit_sha256=$candidate|.binary_sha256=$binary|.workload_kind="host_selected_eight_track_bank"|.workload_id="issue038.host_selected_eight_track_bank.48000hz.q128"|.bank_backend="Simd8"|.bank_width=8|.bank_count=1|.scalar_tail_count=0|.output_sha256="5555555555555555555555555555555555555555555555555555555555555555"' "$base"
jq -c "${common[@]}" '.round=$round|.candidate_commit_sha256=$candidate|.binary_sha256=$binary|.workload_kind="mixed_twelve_track_graph"|.workload_id="issue038.mixed_twelve_track_graph.48000hz.q128"|.tracks=12|.bank_backend="Simd8"|.bank_width=8|.bank_count=1|.scalar_tail_count=2|.scalar_fallback_count=2|.identity_lane_count=2|.input_sha256="6666666666666666666666666666666666666666666666666666666666666666"|.output_sha256="7777777777777777777777777777777777777777777777777777777777777777"' "$base"
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
    MISO_ENGINE_TEST_BENCH_MODE="$mode" \
    MISO_ENGINE_TEST_LAUNCH_LOG="$launch_log" \
    MISO_ENGINE_TEST_FAKE_BENCH="$case_root/scripts/fixtures/fake-bench.sh" \
    MISO_ENGINE_TEST_REAL_RUSTC="$(command -v rustc)" \
    PATH="$case_root/bin:$PATH" \
    bash "$case_root/scripts/run-rack-benchmark.sh"
}
expect_failure_reason() {
    local reason=$1
    jq -e --arg reason "$reason" '.status == "FAIL" and .reason == $reason' \
        "$case_root/artifacts/issue038/rack-benchmark.disposition.json" >/dev/null
}

new_case overwrite
mkdir -p "$case_root/artifacts/issue038"
printf 'protected raw bytes\n' >"$case_root/artifacts/issue038/rack-benchmark.raw.jsonl"
if run_case success >/dev/null 2>&1; then
    printf 'runner overwrote a pre-existing raw artifact\n' >&2
    exit 1
fi
[[ ! -e "$launch_log" ]]
[[ "$(<"$case_root/artifacts/issue038/rack-benchmark.raw.jsonl")" == 'protected raw bytes' ]]

new_case warmup-failure
if run_case warmup_fail >/dev/null 2>&1; then
    printf 'runner swallowed warmup failure\n' >&2
    exit 1
fi
expect_failure_reason warmup_failed
[[ "$(wc -l <"$launch_log")" == 1 ]]
[[ ! -e "$case_root/artifacts/issue038/rack-benchmark.raw.jsonl" ]]

new_case round1-failure
if run_case round1_fail >/dev/null 2>&1; then
    printf 'runner swallowed measured round-1 failure\n' >&2
    exit 1
fi
expect_failure_reason round_1_failed
[[ "$(wc -l <"$launch_log")" == 2 ]]
grep -Fqx '{"partial":"round1"}' "$case_root/artifacts/issue038/rack-benchmark.raw.jsonl"
[[ ! -e "$case_root/artifacts/issue038/rack-benchmark.accepted.jsonl" ]]

new_case round2-failure
if run_case round2_fail >/dev/null 2>&1; then
    printf 'runner swallowed measured round-2 failure\n' >&2
    exit 1
fi
expect_failure_reason round_2_failed
[[ "$(wc -l <"$launch_log")" == 3 ]]
[[ "$(tail -n 1 "$case_root/artifacts/issue038/rack-benchmark.raw.jsonl")" == '{"partial":"round2"}' ]]

new_case validation-failure
if run_case invalid >/dev/null 2>&1; then
    printf 'runner promoted invalid synthetic output\n' >&2
    exit 1
fi
expect_failure_reason validation_failed
[[ "$(wc -l <"$launch_log")" == 3 ]]
[[ "$(wc -l <"$case_root/artifacts/issue038/rack-benchmark.raw.jsonl")" == 6 ]]
[[ ! -e "$case_root/artifacts/issue038/rack-benchmark.accepted.jsonl" ]]

new_case pipe-failure
if MISO_ENGINE_TEST_RUSTC_PIPE_FAIL=1 run_case success >/dev/null 2>&1; then
    printf 'runner swallowed metadata pipeline failure\n' >&2
    exit 1
fi
expect_failure_reason metadata_failed
[[ ! -e "$launch_log" ]]

new_case interruption
if run_case interrupt >/dev/null 2>&1; then
    printf 'runner swallowed interruption\n' >&2
    exit 1
fi
expect_failure_reason interrupted
[[ "$(wc -l <"$launch_log")" == 1 ]]
[[ ! -e "$case_root/artifacts/issue038/rack-benchmark.accepted.jsonl" ]]

new_case success
accepted_path=$(run_case success)
raw_path="$case_root/artifacts/issue038/rack-benchmark.raw.jsonl"
[[ "$accepted_path" == "$case_root/artifacts/issue038/rack-benchmark.accepted.jsonl" ]]
cmp -s "$raw_path" "$accepted_path"
[[ "$(wc -l <"$launch_log")" == 3 ]]
raw_sha=$(sha256sum "$raw_path" | awk '{print $1}')
accepted_sha=$(sha256sum "$accepted_path" | awk '{print $1}')
jq -e --arg raw "$raw_sha" --arg accepted "$accepted_sha" '
    .status == "PASS" and .reason == "complete" and
    .runner_invocations == 1 and .workload_process_launches == 3 and
    .warmup_launches == 1 and .measured_rounds_completed == 2 and
    .raw_sha256 == $raw and .accepted_sha256 == $accepted and .raw_sha256 == .accepted_sha256
' "$case_root/artifacts/issue038/rack-benchmark.disposition.json" >/dev/null
if run_case success >/dev/null 2>&1; then
    printf 'runner resumed or overwrote a completed lifecycle\n' >&2
    exit 1
fi
[[ "$(wc -l <"$launch_log")" == 3 ]]

rg -q 'set -euo pipefail' "$runner"
rg -q 'set -o noclobber' "$runner"
[[ "$(rg -c '^run_round (warmup|1|2) ' "$runner")" == 3 ]]
[[ "$(rg -c '^run_round [12] ' "$runner")" == 2 ]]
! rg -n 'retry|resume|eval|source ' "$runner" >/dev/null
printf 'rack benchmark validators/lifecycle: PASS (audio workload launches: 0)\n'
