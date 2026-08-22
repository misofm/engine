#!/usr/bin/env bash
# Static Issue-109 metadata repair, authority, and inherited-evidence checker.
set -euo pipefail
[[ $# -le 1 ]] || { printf 'usage: %s [root]\n' "$0" >&2; exit 2; }
root="$(cd "${1:-.}" && pwd)"
cd "$root"
fail() { printf 'Issue-109 policy failure: %s\n' "$1" >&2; exit 1; }
hash_file() { sha256sum "$1" | awk '{print $1}'; }
require_hash() {
    [[ -f "$1" && ! -L "$1" ]] || fail "missing regular authority: $1"
    [[ "$(hash_file "$1")" == "$2" ]] || fail "authority hash: $1"
}

require_hash Cargo.lock 4213efd775d1d1207fea805ccdc01392acb015ae36d1bf2eba783f938f19916a
require_hash tools/miso-engine-builtins-bench/Cargo.toml f361c26b6a59c984a9fc60484748b5a2fd0bd0c35079e83ee72d3932f118cf97
require_hash tools/miso-engine-builtins-bench/src/main.rs b520e3d14bd4fa2985d18f273e515261a53b4ea69ac1a2a38aba9bc77bf6e7fe
require_hash scripts/preflight-builtins-benchmark.sh 216cdd879a02b350279619066a28be7f3ef5fa9f05ec26641dd6d3bac634cfe8
require_hash scripts/run-builtins-benchmark.sh 17968dfbdc502ecf8f708e4d99db199848a153d08e6dbc25ef46a4bf9a02669f
require_hash scripts/test-builtins-benchmark.sh 19ecf0ed6c0b6dacbbd2ebf1417fff0bd1207d2cfd567d3a731f735805704b0c
require_hash scripts/builtins-benchmark-record-validator.jq c3db1d9574360bdab0d9ac335615787446e5537439d6accdded4fdd0a4479467
require_hash scripts/builtins-benchmark-validator.jq 6085e740f15d7902fca4443d761cfb8e29df7168ba12f632c7946db56a3e1b63
require_hash fixtures/builtins/v1/MANIFEST.tsv bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff
require_hash fixtures/builtins/v1/pcm/graph-taps.f32le 508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19
require_hash fixtures/builtins/v1/meters/graph-taps.jsonl 958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f
rg -Fq 0c71b71d864fbdd01aa918c6825abea78c38f0486535bc914af92142a5080d19 \
    .github/ISSUE_SPECS/068-builtin-native-aarch64-and-wasm-runtime-selection-and-instruction-qualification.md ||
    fail 'accepted Issue-068 source identity'

while read -r name bytes digest; do
    path="target/issue72/$name"
    [[ -f "$path" && ! -L "$path" && "$(stat -c %h "$path")" == 1 ]] ||
        fail "Issue-072 regular one-link artifact: $name"
    [[ "$(wc -c <"$path")" == "$bytes" && "$(hash_file "$path")" == "$digest" ]] ||
        fail "Issue-072 artifact identity: $name"
done <<'EOF'
nonbenchmark.seal.json 2109 7c38b068ae16055df3cfe6b817943f5fbb1a639d85597560e223d631bc37885d
miso_engine_builtins_bench 3200296 a7bafc459b69fb8bdfd7d9195e4ff8d1febf8602a57540498cb579d04a486912
builtins-benchmark.preflight.json 1525 f4e624b88eddbea5eb09928b544d13093d9a68be278f8afb6b70076fc8dce6bf
builtins-benchmark.raw.jsonl 40136 c44433bc5391bafa8463b0cfabcb78cfc80882015ff808c591d40ae5a508819a
builtins-benchmark.jsonl 40136 c44433bc5391bafa8463b0cfabcb78cfc80882015ff808c591d40ae5a508819a
builtins-benchmark.validator.stderr 211 7935bf62063c0e9d2bfaac91d02db6f448dbf0636fbf16d3c49660738f55b396
builtins-benchmark.disposition.json 1252 b650449d6a8944f4b00fcd833e5f775c9601a9aeb580864624a4b2c978a0698e
EOF
[[ ! -e target/issue72/builtins-benchmark.prelaunch.disposition.json &&
   ! -L target/issue72/builtins-benchmark.prelaunch.disposition.json ]] ||
    fail 'Issue-072 prelaunch disposition appeared'
[[ "$(stat -c %i target/issue72/builtins-benchmark.raw.jsonl)" != \
   "$(stat -c %i target/issue72/builtins-benchmark.jsonl)" ]] || fail 'Issue-072 raw/accepted inode alias'

successor=(
    scripts/run-builtins-benchmark-109.sh
    scripts/preflight-builtins-benchmark-109.sh
    scripts/test-builtins-benchmark-109.sh
    scripts/check-builtins-benchmark-109.sh
    scripts/test-builtins-benchmark-109-policy.sh
)
for path in "${successor[@]}"; do [[ -f "$path" && ! -L "$path" ]] || fail "missing successor: $path"; done

runner=scripts/run-builtins-benchmark-109.sh
for mapping in \
    CPU_MODEL CPU_ARCHITECTURE LOGICAL_CORE_COUNT PHYSICAL_CORE_COUNT OS KERNEL \
    GOVERNOR_OR_POWER_MODE RUST_VERSION LLVM_VERSION TARGET_TRIPLE TARGET_FEATURES PROFILE \
    OPT_LEVEL LTO CODEGEN_UNITS BACKGROUND_LOAD_NOTE; do
    count="$(rg -o "MISO_ENGINE_BENCH_${mapping}" "$runner" | wc -l | tr -d ' ')"
    [[ "$count" -ge 1 ]] || fail "metadata mapping: $mapping"
done
for token in \
    '/proc/cpuinfo' '/proc/loadavg' '/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor' \
    'uname -m' 'uname -s' 'uname -r' 'getconf _NPROCESSORS_ONLN' 'lscpu -p=CORE,SOCKET' \
    'rustc -V' 'rustc -vV' 'rustc --print cfg --target' \
    'profile=release opt_level=3 lto=false codegen_units=16' \
    'target/issue109' 'metadata_projection_sha256' 'issue":109' \
    'preflight_invocations":1' 'runner_invocations":1'; do
    rg -Fq "$token" "$runner" || fail "runner invariant: $token"
done
[[ "$(rg -n '^"\$binary" >&"\$raw_fd" 2>&"\$stderr_fd"$' "$runner" | wc -l | tr -d ' ')" == 1 ]] ||
    fail 'exactly one launch site'
rg -Fq 'preflight_invocations==0 and .runner_invocations==0 and .workload_invocations==0' \
    scripts/preflight-builtins-benchmark-109.sh || fail 'initial repair counters'
for path in scripts/run-builtins-benchmark-109.sh scripts/preflight-builtins-benchmark-109.sh; do
    if rg -n 'target/issue72/.*(>|mv|cp|ln|rm|truncate)|artifact_directory=.*target/issue72' "$path"; then
        fail "successor writes inherited namespace: $path"
    fi
done
if [[ -d target/issue109 ]]; then
    while IFS= read -r path; do
        [[ "$path" == target/issue109/metadata-repair.seal.json && -f "$path" && ! -L "$path" &&
           "$(stat -c %h "$path")" == 1 ]] || fail 'unauthorized Issue-109 artifact appeared'
    done < <(find target/issue109 -mindepth 1 -maxdepth 1 -print)
fi
printf 'Issue-109 policy: PASS counters=0/0/0/0\n'
