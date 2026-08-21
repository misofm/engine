#!/usr/bin/env bash
# Sole exactly-once Issue-038 timing entrypoint. Do not invoke its binary directly.
set -euo pipefail
[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
artifact_dir="$root/artifacts/issue038"
raw="$artifact_dir/rack-benchmark.raw.jsonl"
accepted="$artifact_dir/rack-benchmark.accepted.jsonl"
stderr_log="$artifact_dir/rack-benchmark.stderr.log"
disposition="$artifact_dir/rack-benchmark.disposition.json"
for path in "$raw" "$accepted" "$stderr_log" "$disposition"; do
    [[ ! -e "$path" ]] || { printf 'refusing to overwrite issue-038 artifact: %s\n' "$path" >&2; exit 1; }
done
mkdir -p "$artifact_dir"
umask 077
candidate=$(git rev-parse HEAD)
cargo build --locked --release --quiet -p miso-engine-rack-bench
binary="$root/target/release/miso_engine_rack_bench"
[[ -x "$binary" ]] || { printf 'missing rack benchmark binary\n' >&2; exit 1; }
binary_sha256=$(sha256sum "$binary" | awk '{print $1}')
write_disposition() {
    local status=$1 reason=$2 raw_sha=null raw_bytes=0
    [[ -e "$raw" ]] && raw_sha="\"$(sha256sum "$raw" | awk '{print $1}')\"" && raw_bytes=$(wc -c <"$raw")
    printf '{"schema_version":2,"issue":38,"status":"%s","reason":"%s","raw_sha256":%s,"raw_bytes":%s}\n' "$status" "$reason" "$raw_sha" "$raw_bytes" >"$disposition"
}
failed=1
trap 'if [[ "$failed" == 1 && ! -e "$disposition" ]]; then write_disposition FAIL interrupted; fi' EXIT INT TERM
run_round() {
    local round=$1
    MISO_ENGINE_RACK_BENCH_ROUND="$round" MISO_ENGINE_RACK_BENCH_CANDIDATE_SHA256="$candidate" MISO_ENGINE_RACK_BENCH_BINARY_SHA256="$binary_sha256" \
    MISO_ENGINE_BENCH_CPU_MODEL="$(awk -F: '/model name/ {gsub(/^ +/, "", $2); print $2; exit}' /proc/cpuinfo)" MISO_ENGINE_BENCH_ARCHITECTURE="$(uname -m)" MISO_ENGINE_BENCH_LOGICAL_CORES="$(getconf _NPROCESSORS_ONLN)" MISO_ENGINE_BENCH_OS="$(uname -s)" MISO_ENGINE_BENCH_KERNEL="$(uname -r)" MISO_ENGINE_BENCH_RUST_VERSION="$(rustc -V)" MISO_ENGINE_BENCH_LLVM_VERSION="$(rustc -vV | awk -F: '/LLVM version/ {gsub(/^ +/, "", $2); print $2}')" MISO_ENGINE_BENCH_TARGET="$(rustc -vV | awk -F: '/host/ {gsub(/^ +/, "", $2); print $2}')" MISO_ENGINE_BENCH_TARGET_FEATURES="avx2$(grep -qm1 ' fma ' /proc/cpuinfo && printf ',fma' || true)" MISO_ENGINE_BENCH_PROFILE=release MISO_ENGINE_BENCH_OPT_LEVEL=3 MISO_ENGINE_BENCH_LTO=unknown MISO_ENGINE_BENCH_CODEGEN_UNITS=unknown "$binary"
}
# One untimed warmup, then exactly the two frozen measured rounds. Raw stdout is never rewritten.
run_round warmup >/dev/null 2>"$stderr_log" || { write_disposition FAIL warmup_failed; exit 1; }
run_round 1 >"$raw" 2>>"$stderr_log" || { write_disposition FAIL round_1_failed; exit 1; }
run_round 2 >>"$raw" 2>>"$stderr_log" || { write_disposition FAIL round_2_failed; exit 1; }
[[ "$(wc -l <"$raw")" == 6 ]] || { write_disposition FAIL record_count; exit 1; }
jq -s -e -L scripts -f scripts/rack-benchmark-validator.jq "$raw" >/dev/null || { write_disposition FAIL validation_failed; exit 1; }
cp -- "$raw" "$accepted"
cmp -s -- "$raw" "$accepted" || { write_disposition FAIL accepted_not_byte_identical; exit 1; }
write_disposition PASS complete
failed=0
trap - EXIT INT TERM
printf '%s\n' "$accepted"
