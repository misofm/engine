#!/usr/bin/env bash
# E1 for issue #136: compare every pre-collapse tool subject with its consolidated entry point.
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)

fail() {
    printf 'tools golden comparison failure: %s\n' "$1" >&2
    exit 1
}

normalize() {
    local input=$1 output=$2
    local filter='def norm:
      if type == "object" then with_entries(
        .value = if (.key | test("(_ns$)|(_address$)|(_rss_bytes$)|(^|_)(duration|elapsed|timestamp|ns_per|samples_per_second|frames_per_second)|(^|_)(binary_sha256|tool_.*sha256|git_commit|git_tree|workspace_dirty|worker_cpu_fraction|peak_resident_bytes)$|^(min|max|p50|p95|p99|p99_9)$"))
                 then "<volatile>" else (.value | norm) end)
      elif type == "array" then map(norm)
      else . end;
      map(norm)';
    if jq -S -s "$filter" "$input" >"$output" 2>/dev/null; then
        return
    fi
    jq -S -n --rawfile output "$input" '{non_json_output:$output}' >"$output"
}

normalize_stderr() {
    local input=$1 output=$2 old_root=$3 new_root=$4
    sed -E -e "s|$old_root|<source-root>|g" -e "s|$new_root|<source-root>|g" \
        -e 's/\([0-9]+\)/(pid)/g' \
        -e 's#tools/miso-engine-[^: ]+/src/[A-Za-z0-9_./-]+[.]rs#<tool-source>#g' \
        "$input" >"$output"
}

self_test() {
    local scratch left right
    scratch=$(mktemp -d)
    trap 'rm -rf -- "$scratch"' RETURN
    printf '{"kind":"golden","duration_ns":1,"stable":7}\n' >"$scratch/old"
    printf '{"kind":"golden","duration_ns":2,"stable":7}\n' >"$scratch/new"
    normalize "$scratch/old" "$scratch/old.normalized"
    normalize "$scratch/new" "$scratch/new.normalized"
    diff -u "$scratch/old.normalized" "$scratch/new.normalized" >/dev/null ||
        fail 'timing-only self-test mutation was not normalized'
    printf '{"kind":"golden","duration_ns":2,"renamed":7}\n' >"$scratch/new"
    normalize "$scratch/new" "$scratch/new.normalized"
    if diff -u "$scratch/old.normalized" "$scratch/new.normalized" >/dev/null; then
        fail 'stable-key red mutation escaped'
    fi
    printf 'tools golden comparison self-test passed\n'
}

if [[ ${1:-} == --self-test ]]; then
    [[ $# -eq 1 ]] || fail 'usage: compare-tools-golden.sh --self-test | BASE_COMMIT'
    self_test
    exit 0
fi
[[ $# -eq 1 ]] || fail 'usage: compare-tools-golden.sh --self-test | BASE_COMMIT'
base=$1
git -C "$root" rev-parse --verify "$base^{commit}" >/dev/null || fail 'unknown base commit'
git -C "$root" merge-base --is-ancestor "$base" HEAD || fail 'base is not an ancestor of HEAD'
command -v cargo >/dev/null || fail 'cargo is required'
command -v jq >/dev/null || fail 'jq is required'

scratch=$(mktemp -d)
base_tree="$root/target/issue136-tools-golden-base"
cleanup() {
    rm -rf -- "$scratch"
}
trap cleanup EXIT
if [[ -e "$base_tree/.git" ]]; then
    [[ $(git -C "$base_tree" rev-parse HEAD) == $(git -C "$root" rev-parse "$base") ]] ||
        fail "cached base worktree does not match $base"
else
    git -C "$root" worktree add --detach "$base_tree" "$base" >/dev/null
fi

old_target="$root/target/issue136-tools-golden-build/old"
new_target="$root/target/issue136-tools-golden-build/new"
old_packages=(
    miso-engine-bootstrap-bench miso-engine-conformance-bench miso-engine-realtime-audit
    miso-engine-protocol-audit miso-engine-capi-audit miso-engine-protocol-bench
    miso-engine-session-bench miso-engine-effect-contract-bench
    miso-engine-effect-interchange-bench miso-engine-graph-audit miso-engine-scheduler-audit
    miso-engine-scheduler-bench miso-engine-graph-bench miso-engine-builtins-audit
    miso-engine-builtins-bench miso-engine-builtins-fixture miso-engine-rack-bench
    miso-engine-source-fixture miso-engine-source-audit
)
old_build_args=()
for package in "${old_packages[@]}"; do
    old_build_args+=(-p "$package")
done
# Build every baseline package in one invocation so Cargo resolves the same union of dependency
# features that the two consolidated packages necessarily share.
CARGO_TARGET_DIR="$old_target" cargo build --quiet --locked --release --all-features \
    --manifest-path "$base_tree/Cargo.toml" "${old_build_args[@]}"
# This is deliberately the union build: feature leakage introduced by package consolidation is
# part of the comparison, not an exemption from it.
CARGO_TARGET_DIR="$new_target" cargo build --quiet --locked --release \
    --manifest-path "$root/Cargo.toml" --all-features -p miso-engine-bench -p miso-engine-audit

export MISO_ENGINE_BENCH_CANDIDATE_COMMIT=1111111111111111111111111111111111111111
export MISO_ENGINE_BENCH_CANDIDATE_TREE=2222222222222222222222222222222222222222
export MISO_ENGINE_BENCH_CANDIDATE_SHA256=3333333333333333333333333333333333333333333333333333333333333333
export MISO_ENGINE_BENCH_BINARY_SHA256=4444444444444444444444444444444444444444444444444444444444444444
export MISO_ENGINE_BENCH_TOOL_MANIFEST_SHA256=5555555555555555555555555555555555555555555555555555555555555555
export MISO_ENGINE_BENCH_TOOL_SOURCE_SHA256=6666666666666666666666666666666666666666666666666666666666666666
export MISO_ENGINE_BENCH_FIXTURE_MANIFEST_SHA256=7777777777777777777777777777777777777777777777777777777777777777
export MISO_ENGINE_BENCH_ROUND=1
export MISO_ENGINE_BENCH_CPU_MODEL=golden-cpu
export MISO_ENGINE_BENCH_CPU_ARCHITECTURE=golden-architecture
export MISO_ENGINE_BENCH_LOGICAL_CORE_COUNT=8
export MISO_ENGINE_BENCH_PHYSICAL_CORE_COUNT=4
export MISO_ENGINE_BENCH_OS=golden-os
export MISO_ENGINE_BENCH_KERNEL=golden-kernel
export MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE=golden-governor
export MISO_ENGINE_BENCH_POWER_SOURCE=golden-power
export MISO_ENGINE_BENCH_RUST_VERSION=golden-rustc
export MISO_ENGINE_BENCH_LLVM_VERSION=golden-llvm
export MISO_ENGINE_BENCH_TARGET_TRIPLE=golden-target
export MISO_ENGINE_BENCH_TARGET_FEATURES=golden-features
export MISO_ENGINE_BENCH_PROFILE=release
export MISO_ENGINE_BENCH_OPT_LEVEL=3
export MISO_ENGINE_BENCH_LTO=fat
export MISO_ENGINE_BENCH_CODEGEN_UNITS=1
export MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE=none
export MISO_ENGINE_BENCH_RUNTIME_OR_BROWSER=native
export MISO_ENGINE_BENCH_WASM_HOST=golden-wasm-host
export MISO_ENGINE_BENCH_WASM_HOST_VERSION=1
export MISO_ENGINE_BENCH_WASM_SCALAR_BYTES=1
export MISO_ENGINE_BENCH_WASM_SIMD_BYTES=1

case_command() {
    local side=$1 case_name=$2 source_root=$3
    local binary_root
    binary_root=$([[ $side == old ]] && printf '%s/release' "$old_target" || printf '%s/release' "$new_target")
    case "$case_name" in
        bootstrap) old_bin=miso_engine_bootstrap_bench; old_args=(--rounds 1); new_tool=bench; new_args=(bootstrap --rounds 1) ;;
        conformance) old_bin=miso_engine_conformance_bench; old_args=(--rounds 1); new_tool=bench; new_args=(conformance --rounds 1) ;;
        realtime-audit) old_bin=miso_engine_realtime_audit; old_args=(--audit --blocks 3); new_tool=audit; new_args=(realtime --audit --blocks 3) ;;
        realtime-bench) old_bin=miso_engine_realtime_audit; old_args=(--blocks 1 --benchmark-rounds 1); new_tool=audit; new_args=(realtime --blocks 1 --benchmark-rounds 1) ;;
        protocol-audit) old_bin=miso_engine_protocol_audit; old_args=(); new_tool=audit; new_args=(protocol) ;;
        capi-audit) old_bin=miso_engine_capi_audit; old_args=(); new_tool=audit; new_args=(capi) ;;
        protocol-bench) old_bin=miso_engine_protocol_bench; old_args=(--rounds 2); new_tool=bench; new_args=(protocol --rounds 2) ;;
        session) old_bin=miso_engine_session_bench; old_args=(); new_tool=bench; new_args=(session) ;;
        effect-contract-conformance) old_bin=miso_engine_effect_contract_bench; old_args=(--conformance); new_tool=bench; new_args=(effect-contract --conformance) ;;
        effect-contract-audit) old_bin=miso_engine_effect_contract_bench; old_args=(--audit 1); new_tool=bench; new_args=(effect-contract --audit 1) ;;
        effect-contract-bench) old_bin=miso_engine_effect_contract_bench; old_args=(--benchmark-two-rounds); new_tool=bench; new_args=(effect-contract --benchmark-two-rounds) ;;
        effect-interchange) old_bin=miso_engine_effect_interchange_bench; old_args=(); new_tool=bench; new_args=(effect-interchange) ;;
        graph-audit) old_bin=miso_engine_graph_audit; old_args=(--blocks 3); new_tool=audit; new_args=(graph --blocks 3) ;;
        parametric-eq-audit) old_bin=miso_engine_graph_audit_parametric_eq; old_args=(--blocks 100000); new_tool=audit; new_args=(parametric-eq --blocks 100000) ;;
        delay-audit) old_bin=miso_engine_graph_audit_delay; old_args=(--blocks 100000); new_tool=audit; new_args=(delay --blocks 100000) ;;
        gate-expander-audit) old_bin=miso_engine_graph_audit_gate_expander; old_args=(--blocks 100000); new_tool=audit; new_args=(gate-expander --blocks 100000) ;;
        compressor-audit) old_bin=miso_engine_graph_audit_compressor; old_args=(--blocks 100000); new_tool=audit; new_args=(compressor --blocks 100000) ;;
        scheduler-audit) old_bin=miso_engine_scheduler_audit; old_args=(); new_tool=audit; new_args=(scheduler) ;;
        scheduler-bench) old_bin=miso_engine_scheduler_bench; old_args=(); new_tool=bench; new_args=(scheduler) ;;
        graph-bench) old_bin=miso_engine_graph_bench; old_args=(); new_tool=bench; new_args=(graph) ;;
        builtins-audit) old_bin=miso_engine_builtins_audit; old_args=(); new_tool=audit; new_args=(builtins) ;;
        builtins-graph-audit) old_bin=miso_engine_builtins_audit_graph; old_args=(); new_tool=audit; new_args=(builtins-graph) ;;
        builtins-audit-fixture)
            old_bin=miso_engine_builtins_audit_fixture
            old_args=(--check "$source_root/tools/miso-engine-builtins-audit/fixtures/v1")
            new_tool=audit; new_args=(builtins-fixture --check "$source_root/tools/miso-engine-audit/fixtures/builtins-audit-v1") ;;
        builtins-bench) old_bin=miso_engine_builtins_bench; old_args=(); new_tool=bench; new_args=(builtins) ;;
        builtins-fixture)
            old_bin=miso_engine_builtins_fixture; old_args=(--check "$source_root/fixtures/builtins/v1")
            new_tool=audit; new_args=(fixture-builtins --check "$source_root/fixtures/builtins/v1") ;;
        builtins-listening-cli)
            old_bin=miso_engine_builtins_fixture_listening; old_args=(--golden-invalid)
            new_tool=audit; new_args=(fixture-builtins-listening --golden-invalid) ;;
        rack-bench) old_bin=miso_engine_rack_bench; old_args=(); new_tool=bench; new_args=(rack) ;;
        source-fixture) old_bin=miso_engine_source_fixture; old_args=(); new_tool=audit; new_args=(fixture-source) ;;
        source-audit) old_bin=miso_engine_source_audit; old_args=(); new_tool=audit; new_args=(source) ;;
        source-duration) old_bin=miso_engine_source_audit_duration; old_args=(); new_tool=audit; new_args=(source-duration) ;;
        *) fail "unknown case $case_name" ;;
    esac
    if [[ $side == old ]]; then
        GOLDEN_COMMAND=("$binary_root/$old_bin" "${old_args[@]}")
    else
        GOLDEN_COMMAND=("$binary_root/miso_engine_$new_tool" "${new_args[@]}")
    fi
}

cases=(
    bootstrap conformance realtime-audit realtime-bench protocol-audit capi-audit protocol-bench
    session effect-contract-conformance effect-contract-audit effect-contract-bench
    effect-interchange graph-audit parametric-eq-audit delay-audit gate-expander-audit
    compressor-audit scheduler-audit scheduler-bench graph-bench builtins-audit
    builtins-graph-audit builtins-audit-fixture builtins-bench builtins-fixture
    builtins-listening-cli rack-bench source-fixture source-audit source-duration
)
report="$root/target/issue136-tools-golden"
mkdir -p "$report"
[[ -z $(find "$report" -mindepth 1 -maxdepth 1 -print -quit) ]] || fail "report exists: $report"
for case_name in "${cases[@]}"; do
    for side in old new; do
        source_root=$([[ $side == old ]] && printf '%s' "$base_tree" || printf '%s' "$root")
        case_command "$side" "$case_name" "$source_root"
        status=0
        (cd "$source_root" && "${GOLDEN_COMMAND[@]}") >"$report/$case_name.$side.stdout" \
            2>"$report/$case_name.$side.stderr" || status=$?
        printf '%s\n' "$status" >"$report/$case_name.$side.status"
        normalize "$report/$case_name.$side.stdout" "$report/$case_name.$side.normalized.json"
        normalize_stderr "$report/$case_name.$side.stderr" \
            "$report/$case_name.$side.normalized.stderr" "$base_tree" "$root"
        jq -S -n --argjson status "$status" \
            --rawfile stderr "$report/$case_name.$side.normalized.stderr" \
            --slurpfile records "$report/$case_name.$side.normalized.json" \
            '{status:$status,records:$records[0],stderr:$stderr}' \
            >"$report/$case_name.$side.comparison.json"
    done
    diff -u "$report/$case_name.old.comparison.json" \
        "$report/$case_name.new.comparison.json" >"$report/$case_name.diff" ||
        fail "$case_name differs; see $report/$case_name.diff"
done
printf 'tools golden comparison passed: %s subjects, empty normalized diff\n' "${#cases[@]}"
