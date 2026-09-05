#!/usr/bin/env bash
set -euo pipefail
[[ $# -eq 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
artifact="$root/target/issue431-prepared"
binary="$artifact/bench"
seal="$artifact/builtins-benchmark.preflight.json"
[[ -x "$binary" && ! -L "$binary" && -f "$seal" ]] || { printf 'missing current preflight\n' >&2; exit 1; }
[[ ! -e "$artifact/builtins-benchmark.jsonl" && ! -L "$artifact/builtins-benchmark.jsonl" ]] || { printf 'refusing overwrite\n' >&2; exit 1; }
tmp="$artifact/builtins-benchmark.raw.jsonl.tmp"
trap 'rm -f -- "$tmp"' EXIT
"$binary" builtins >"$tmp" 2>"$artifact/builtins-benchmark.stderr"
mv -n -- "$tmp" "$artifact/builtins-benchmark.raw.jsonl"
jq -e -L "$root/scripts" -f "$root/scripts/builtins-current-benchmark-validator.jq" \
  "$artifact/builtins-benchmark.raw.jsonl" > "$artifact/builtins-benchmark.jsonl"
