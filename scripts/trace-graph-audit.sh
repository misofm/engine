#!/usr/bin/env bash
set -euo pipefail

workspace_dir=$(cd "$(dirname "$0")/.." && pwd)
binary="$workspace_dir/target/release/miso_engine_graph_audit"
blocks="${1:-1000000}"
trace_root="$workspace_dir/target/issue6/strace"

cargo build --quiet --locked --release --manifest-path "$workspace_dir/Cargo.toml" \
  -p miso-engine-graph-audit
command -v strace >/dev/null 2>&1 || {
  printf 'strace is required for the graph realtime syscall gate\n' >&2
  exit 1
}
mkdir -p "$trace_root"
trace_prefix="$trace_root/trace"
find "$trace_root" -maxdepth 1 -type f -name 'trace.*' -delete

strace -ff -qq -o "$trace_prefix" "$binary" --blocks "$blocks" \
  > "$trace_root/audit.json"

marker_file=""
while IFS= read -r candidate; do
  if rg -q 'MISO_ENGINE_GRAPH_RT_BEGIN' "$candidate" && rg -q 'MISO_ENGINE_GRAPH_RT_END' "$candidate"; then
    [[ -z "$marker_file" ]] || {
      printf 'multiple trace threads contain both graph markers\n' >&2
      exit 1
    }
    marker_file="$candidate"
  fi
done < <(find "$trace_root" -maxdepth 1 -type f -name 'trace.*' | sort)
[[ -n "$marker_file" ]] || {
  printf 'no trace thread contains both graph markers\n' >&2
  exit 1
}

unexpected=$({
  awk '
    /MISO_ENGINE_GRAPH_RT_BEGIN/ { inside = 1; next }
    /MISO_ENGINE_GRAPH_RT_END/ { inside = 0; found_end = 1; next }
    inside { print }
    END { if (!found_end) exit 2 }
  ' "$marker_file"
} || true)
[[ -z "$unexpected" ]] || {
  printf 'unexpected graph render syscall(s):\n%s\n' "$unexpected" >&2
  exit 1
}

jq -e --argjson blocks "$blocks" '
  .kind == "graph_realtime_audit" and
  .blocks == $blocks and
  .quantum_frames == 1 and
  .swaps_accepted == 2 and
  .swaps_deferred == 1 and
  .displaced_plans_destroyed_off_render == 2 and
  .total_violations == 0
' "$trace_root/audit.json" >/dev/null
printf 'graph realtime syscall trace: PASS (%s blocks)\n' "$blocks"
