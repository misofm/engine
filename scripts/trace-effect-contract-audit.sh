#!/usr/bin/env bash
set -euo pipefail
binary="${1:-target/release/miso_engine_bench}"
blocks="${2:-1000000}"
trace_root="target/issue11/strace"
[[ -x "$binary" ]] || { printf 'missing effect audit binary: %s\n' "$binary" >&2; exit 1; }
command -v strace >/dev/null 2>&1 || { printf 'strace is required\n' >&2; exit 1; }
command -v jq >/dev/null 2>&1 || { printf 'jq is required\n' >&2; exit 1; }
mkdir -p "$trace_root"
trace_prefix="$trace_root/trace"
find "$trace_root" -maxdepth 1 -type f -name 'trace.*' -delete
strace -ff -qq -s 200 -o "$trace_prefix" "$binary" effect-contract --audit "$blocks" --trace-markers >"$trace_root/audit.json"
marker_file=""
while IFS= read -r candidate; do
    if rg -q 'MISO_ENGINE_EFFECT_RT_BEGIN' "$candidate" && rg -q 'MISO_ENGINE_EFFECT_RT_END' "$candidate"; then
        [[ -z "$marker_file" ]] || { printf 'multiple marked threads\n' >&2; exit 1; }
        marker_file="$candidate"
    fi
done < <(find "$trace_root" -maxdepth 1 -type f -name 'trace.*' | sort)
[[ -n "$marker_file" ]] || { printf 'missing effect trace markers\n' >&2; exit 1; }
unexpected="$(awk '/MISO_ENGINE_EFFECT_RT_BEGIN/ { inside=1; next } /MISO_ENGINE_EFFECT_RT_END/ { inside=0; ended=1; next } inside { print } END { if (!ended) exit 2 }' "$marker_file" || true)"
[[ -z "$unexpected" ]] || { printf 'unexpected effect-process syscall(s):\n%s\n' "$unexpected" >&2; exit 1; }
jq -e --argjson blocks "$blocks" '.kind == "effect_realtime_audit" and .blocks == $blocks and .frames_per_block == 128 and .total_violations == 0 and .allocations == 0 and .deallocations == 0' "$trace_root/audit.json" >/dev/null
printf 'effect realtime syscall/allocation audit: ok (%s blocks)\n' "$blocks"
