#!/usr/bin/env bash
# Exactly one authorized issue-007 descriptive invocation. Do not retry or tune this workload.
set -euo pipefail

[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "$script_directory/.." && pwd)"
raw_output="$repository_root/target/issue7/builtins-benchmark.raw.jsonl"
accepted_output="$repository_root/target/issue7/builtins-benchmark.jsonl"
[[ ! -e "$raw_output" && ! -e "$accepted_output" ]] || {
  printf 'refusing to overwrite an existing issue-007 benchmark artifact\n' >&2; exit 1;
}
command -v jq >/dev/null || { printf 'jq is required for benchmark validation\n' >&2; exit 1; }
mkdir -p "$repository_root/target/issue7"
export MISO_ENGINE_BENCH_CPU_MODEL="$(awk -F: '/model name|Hardware/ {gsub(/^ +/, "", $2); print $2; exit}' /proc/cpuinfo 2>/dev/null || true)"
export MISO_ENGINE_BENCH_LOGICAL_CORES="$(getconf _NPROCESSORS_ONLN 2>/dev/null || true)"
export MISO_ENGINE_BENCH_OS="$(uname -s 2>/dev/null || true)"
export MISO_ENGINE_BENCH_KERNEL="$(uname -r 2>/dev/null || true)"
export MISO_ENGINE_BENCH_GOVERNOR="$(cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor 2>/dev/null || printf unknown)"
export MISO_ENGINE_BENCH_RUST_VERSION="$(rustc --version 2>/dev/null || true)"
export MISO_ENGINE_BENCH_LLVM_VERSION="$(rustc -vV 2>/dev/null | awk -F': ' '/LLVM version/ {print $2; exit}' || true)"
export MISO_ENGINE_BENCH_TARGET="$(rustc -vV 2>/dev/null | awk -F': ' '/host/ {print $2; exit}' || true)"
export MISO_ENGINE_BENCH_TARGET_FEATURES="${RUSTFLAGS:-default}"
export MISO_ENGINE_BENCH_PROFILE="release"
export MISO_ENGINE_BENCH_OPT_LEVEL="3"
export MISO_ENGINE_BENCH_LTO="${CARGO_PROFILE_RELEASE_LTO:-unknown}"
export MISO_ENGINE_BENCH_CODEGEN_UNITS="${CARGO_PROFILE_RELEASE_CODEGEN_UNITS:-unknown}"
export MISO_ENGINE_BENCH_BACKGROUND_LOAD="runner metadata only; no timed retries"
if ! (cd "$repository_root" && cargo run --locked --release --quiet -p miso-engine-builtins-bench) >"$raw_output"; then
  printf 'builtins benchmark workload failed; raw output preserved at %s\n' "$raw_output" >&2; exit 1
fi
if ! jq -s -e -L "$script_directory" -f "$script_directory/builtins-benchmark-validator.jq" "$raw_output" >/dev/null; then
  printf 'builtins benchmark validation failed; rejected output preserved at %s\n' "$raw_output" >&2; exit 1
fi
mv "$raw_output" "$accepted_output"
cat "$accepted_output"
