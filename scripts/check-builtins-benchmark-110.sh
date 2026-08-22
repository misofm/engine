#!/usr/bin/env bash
# Static Issue-110 metadata completion, authority, and inherited-evidence checker.
set -euo pipefail
[[ $# -le 1 ]] || { printf 'usage: %s [root]\n' "$0" >&2; exit 2; }
root="$(cd "${1:-.}" && pwd)"
cd "$root"
fail() { printf 'Issue-110 policy failure: %s\n' "$1" >&2; exit 1; }
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
require_hash scripts/run-builtins-benchmark-109.sh 84858d352667919b5ecb23b0f75ea4e52890708b820de725696e6f17e1b10935
require_hash scripts/preflight-builtins-benchmark-109.sh 14e1b76739e03aa0cf9b003cd3a0856e0f34bfc517e1112981121a878aca0d23
require_hash scripts/test-builtins-benchmark-109.sh 14dd9ec48921fefdba8a57afa827f27222dc881311929f55a44d0edc89c97ef4
require_hash scripts/check-builtins-benchmark-109.sh bb911ce6f38707830f99340b90d0ed8d219cb808091fb0105b471b1945184889
require_hash scripts/test-builtins-benchmark-109-policy.sh ce88ea207caa77250d91fa773f73f921079c1b16b511780d4afd6cb822c3d5ed
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

require_hash target/issue109/metadata-repair.seal.json \
    1e8cec4904d8987ddca581e5b23870629d734127ad3f9e010f6a5c2d178b69c6
[[ "$(wc -c <target/issue109/metadata-repair.seal.json)" == 2538 &&
   "$(stat -c %h target/issue109/metadata-repair.seal.json)" == 1 ]] ||
    fail 'Issue-109 repair seal shape'
mapfile -d '' -t issue109_members < <(find target/issue109 -mindepth 1 -maxdepth 1 -print0)
[[ "${#issue109_members[@]}" == 1 &&
   "${issue109_members[0]}" == target/issue109/metadata-repair.seal.json ]] ||
    fail 'Issue-109 namespace membership'
for name in miso_engine_builtins_bench builtins-benchmark.preflight.json \
    builtins-benchmark.raw.jsonl builtins-benchmark.jsonl \
    builtins-benchmark.validator.stderr builtins-benchmark.prelaunch.disposition.json \
    builtins-benchmark.disposition.json; do
    [[ ! -e "target/issue109/$name" && ! -L "target/issue109/$name" ]] ||
        fail "Issue-109 forbidden artifact: $name"
done

successor=(
    scripts/run-builtins-benchmark-110.sh
    scripts/preflight-builtins-benchmark-110.sh
    scripts/test-builtins-benchmark-110.sh
    scripts/check-builtins-benchmark-110.sh
    scripts/test-builtins-benchmark-110-policy.sh
)
for path in "${successor[@]}"; do [[ -f "$path" && ! -L "$path" ]] || fail "missing successor: $path"; done

runner=scripts/run-builtins-benchmark-110.sh
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
    'target/issue110' 'metadata_projection_sha256' 'issue":110' \
    'preflight_invocations":1' 'runner_invocations":1'; do
    rg -Fq "$token" "$runner" || fail "runner invariant: $token"
done
[[ "$(rg -n '^"\$binary" >&"\$raw_fd" 2>&"\$stderr_fd"$' "$runner" | wc -l | tr -d ' ')" == 1 ]] ||
    fail 'exactly one launch site'
rg -Fq 'preflight_invocations==0 and .runner_invocations==0 and .workload_invocations==0' \
    scripts/preflight-builtins-benchmark-110.sh || fail 'initial completion counters'
for path in scripts/run-builtins-benchmark-110.sh scripts/preflight-builtins-benchmark-110.sh; do
    if rg -n 'target/issue(72|109)/.*(>|mv|cp|ln|rm|truncate)|artifact_directory=.*target/issue(72|109)' "$path"; then
        fail "successor writes inherited namespace: $path"
    fi
done
if [[ -d target/issue110 ]]; then
    while IFS= read -r path; do
        [[ "$path" == target/issue110/completion.seal.json && -f "$path" && ! -L "$path" &&
           "$(stat -c %h "$path")" == 1 ]] || fail 'unauthorized Issue-110 artifact appeared'
    done < <(find target/issue110 -mindepth 1 -maxdepth 1 -print)
fi
printf 'Issue-110 policy: PASS counters=0/0/0/0\n'
