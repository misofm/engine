#!/usr/bin/env bash
set -euo pipefail
[[ $# -eq 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
artifact="$root/target/issue431-prepared"
mkdir -p "$artifact"
[[ ! -e "$artifact/bench" && ! -L "$artifact/bench" ]] || { printf 'refusing existing prepared binary\n' >&2; exit 1; }
[[ -z "$(git -C "$root" status --porcelain=v1 --untracked-files=normal)" ]] || { printf 'candidate is not clean\n' >&2; exit 1; }
target_dir="$artifact/cargo-target"
CARGO_TARGET_DIR="$target_dir" cargo build --locked --release -p bench
cp -- "$target_dir/release/bench" "$artifact/bench"
chmod 0755 "$artifact/bench"
sha256sum "$artifact/bench" > "$artifact/bench.sha256"
printf '{"schema_version":1,"issue":431,"kind":"builtins_current_preflight","status":"READY","candidate_commit":"%s","binary_sha256":"%s"}\n' \
  "$(git -C "$root" rev-parse HEAD)" "$(awk '{print $1}' "$artifact/bench.sha256")" > "$artifact/builtins-benchmark.preflight.json"
