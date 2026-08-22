#!/usr/bin/env bash
# Hermetic static mutations for the Issue-109 metadata-repair boundary.
set -euo pipefail
script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$script_directory/.." && pwd)"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
template="$scratch/template"
mkdir -p "$template/scripts" "$template/tools/miso-engine-builtins-bench/src" \
    "$template/fixtures/builtins/v1/pcm" "$template/fixtures/builtins/v1/meters" \
    "$template/target/issue72" "$template/.github/ISSUE_SPECS"
cp "$root/Cargo.lock" "$template/"
cp "$root/tools/miso-engine-builtins-bench/Cargo.toml" "$template/tools/miso-engine-builtins-bench/"
cp "$root/tools/miso-engine-builtins-bench/src/main.rs" "$template/tools/miso-engine-builtins-bench/src/"
cp "$root/fixtures/builtins/v1/MANIFEST.tsv" "$template/fixtures/builtins/v1/"
cp "$root/fixtures/builtins/v1/pcm/graph-taps.f32le" "$template/fixtures/builtins/v1/pcm/"
cp "$root/fixtures/builtins/v1/meters/graph-taps.jsonl" "$template/fixtures/builtins/v1/meters/"
cp "$root/.github/ISSUE_SPECS/068-builtin-native-aarch64-and-wasm-runtime-selection-and-instruction-qualification.md" \
    "$template/.github/ISSUE_SPECS/"
cp "$root/scripts/preflight-builtins-benchmark.sh" "$root/scripts/run-builtins-benchmark.sh" \
    "$root/scripts/test-builtins-benchmark.sh" "$root/scripts/builtins-benchmark-record-validator.jq" \
    "$root/scripts/builtins-benchmark-validator.jq" "$root/scripts/run-builtins-benchmark-109.sh" \
    "$root/scripts/preflight-builtins-benchmark-109.sh" "$root/scripts/test-builtins-benchmark-109.sh" \
    "$root/scripts/check-builtins-benchmark-109.sh" "$root/scripts/test-builtins-benchmark-109-policy.sh" \
    "$template/scripts/"
cp -a "$root/target/issue72/." "$template/target/issue72/"

bash "$template/scripts/check-builtins-benchmark-109.sh" "$template" >/dev/null
mutation=0
reject() {
    mutation=$((mutation + 1))
    case_root="$scratch/mutation-$mutation"
    cp -a "$template" "$case_root"
    operation=$1
    shift
    "$operation" "$case_root" "$@"
    if bash "$case_root/scripts/check-builtins-benchmark-109.sh" "$case_root" >/dev/null 2>&1; then
        printf 'Issue-109 policy mutation survived: %s\n' "$mutation" >&2
        exit 1
    fi
}
append_file() { printf '\nmutation\n' >>"$1/$2"; }
remove_file() { rm "$1/$2"; }
replace_text() { sed -i "s|$2|$3|g" "$1/$4"; }
add_launch() { printf '\n"$binary" >&"$raw_fd" 2>&"$stderr_fd"\n' >>"$1/scripts/run-builtins-benchmark-109.sh"; }
add_unauthorized_artifact() { mkdir -p "$1/target/issue109"; printf bad >"$1/target/issue109/bad"; }

reject append_file Cargo.lock
reject append_file tools/miso-engine-builtins-bench/src/main.rs
reject append_file target/issue72/builtins-benchmark.raw.jsonl
reject remove_file scripts/preflight-builtins-benchmark-109.sh
for mapping in CPU_MODEL CPU_ARCHITECTURE LOGICAL_CORE_COUNT PHYSICAL_CORE_COUNT OS KERNEL \
    GOVERNOR_OR_POWER_MODE RUST_VERSION LLVM_VERSION TARGET_TRIPLE TARGET_FEATURES PROFILE \
    OPT_LEVEL LTO CODEGEN_UNITS BACKGROUND_LOAD_NOTE; do
    reject replace_text "MISO_ENGINE_BENCH_$mapping" "MISO_ENGINE_BENCH_REMOVED" \
        scripts/run-builtins-benchmark-109.sh
done
for token in '/proc/cpuinfo' '/proc/loadavg' '/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor' \
    'uname -m' 'getconf _NPROCESSORS_ONLN' 'lscpu -p=CORE,SOCKET' 'rustc --print cfg --target' \
    'profile=release opt_level=3 lto=false codegen_units=16' 'target/issue109' \
    'metadata_projection_sha256'; do
    reject replace_text "$token" 'REMOVED_INVARIANT' scripts/run-builtins-benchmark-109.sh
done
reject add_launch
reject add_unauthorized_artifact
reject replace_text 'preflight_invocations==0 and .runner_invocations==0 and .workload_invocations==0' \
    'preflight_invocations==9' scripts/preflight-builtins-benchmark-109.sh
printf 'Issue-109 policy mutations: PASS (%s rejected)\n' "$mutation"
