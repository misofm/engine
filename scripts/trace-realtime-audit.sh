#!/usr/bin/env bash
# Prove the reference render thread makes no syscall between explicit outer-loop markers.
set -euo pipefail

binary="${1:-target/release/miso_engine_audit}"
blocks="${2:-1000000}"
trace_root="target/realtime-strace"

[[ -x "$binary" ]] || {
    printf 'missing realtime audit binary: %s\n' "$binary" >&2
    exit 1
}
command -v strace >/dev/null 2>&1 || {
    printf 'strace is required for the native realtime syscall gate\n' >&2
    exit 1
}

mkdir -p "$trace_root"
trace_prefix="$trace_root/trace"
find "$trace_root" -maxdepth 1 -type f -name 'trace.*' -delete

strace -ff -qq -s 200 -o "$trace_prefix" "$binary" \
    realtime --blocks "$blocks" --audit --trace-markers \
    >"$trace_root/audit.json"

marker_file=""
while IFS= read -r candidate; do
    if rg -q 'MISO_ENGINE_RT_BEGIN' "$candidate" && rg -q 'MISO_ENGINE_RT_END' "$candidate"; then
        [[ -z "$marker_file" ]] || {
            printf 'multiple trace threads contain both realtime markers\n' >&2
            exit 1
        }
        marker_file="$candidate"
    fi
done < <(find "$trace_root" -maxdepth 1 -type f -name 'trace.*' | sort)

[[ -n "$marker_file" ]] || {
    printf 'no trace thread contains both realtime markers\n' >&2
    exit 1
}

unexpected="$({
    awk '
        /MISO_ENGINE_RT_BEGIN/ { inside = 1; next }
        /MISO_ENGINE_RT_END/ { inside = 0; found_end = 1; next }
        inside { print }
        END { if (!found_end) exit 2 }
    ' "$marker_file"
} || true)"

[[ -z "$unexpected" ]] || {
    printf 'unexpected render-thread syscall(s):\n%s\n' "$unexpected" >&2
    exit 1
}

command -v jq >/dev/null 2>&1 || {
    printf 'jq is required to validate realtime audit JSON\n' >&2
    exit 1
}
jq -e --argjson blocks "$blocks" '
    .kind == "realtime_audit" and
    .blocks == $blocks and
    .swaps_accepted > 0 and
    .swaps_deferred > 0 and
    .total_violations == 0
' "$trace_root/audit.json" >/dev/null

printf 'realtime syscall trace: ok (%s blocks, %s)\n' "$blocks" "$marker_file"
