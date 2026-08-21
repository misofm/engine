#!/usr/bin/env bash
set -euo pipefail

workspace_dir=$(cd "$(dirname "$0")/.." && pwd)
binary="$workspace_dir/target/release/miso_engine_builtins_graph_audit"
trace_root="$workspace_dir/target/issue7/graph-strace"
cargo build --quiet --locked --release --manifest-path "$workspace_dir/Cargo.toml" \
  -p miso-engine-builtins-audit --bin miso_engine_builtins_graph_audit
command -v strace >/dev/null 2>&1 || {
  printf 'strace is required for the issue-007 graph lifecycle syscall gate\n' >&2
  exit 1
}
mkdir -p "$trace_root"
trace_prefix="$trace_root/trace"
find "$trace_root" -maxdepth 1 -type f -name 'trace.*' -delete
strace -ff -qq -o "$trace_prefix" "$binary" >"$trace_root/audit.json"

marker_file=""
while IFS= read -r candidate; do
  if rg -q 'MISO_ISSUE007_GRAPH_RT_BEGIN' "$candidate" && rg -q 'MISO_ISSUE007_GRAPH_RT_END' "$candidate"; then
    [[ -z "$marker_file" ]] || { printf 'multiple graph render trace threads\n' >&2; exit 1; }
    marker_file="$candidate"
  fi
done < <(find "$trace_root" -maxdepth 1 -type f -name 'trace.*' | sort)
[[ -n "$marker_file" ]] || { printf 'missing issue-007 graph render markers\n' >&2; exit 1; }
[[ "$(rg -c 'MISO_ISSUE007_GRAPH_RT_BEGIN' "$marker_file")" == 7 ]] || {
  printf 'unexpected issue-007 graph render marker begin count\n' >&2; exit 1;
}
[[ "$(rg -c 'MISO_ISSUE007_GRAPH_RT_END' "$marker_file")" == 7 ]] || {
  printf 'unexpected issue-007 graph render marker end count\n' >&2; exit 1;
}
unexpected=$(awk '
  /MISO_ISSUE007_GRAPH_RT_BEGIN/ { inside = 1; next }
  /MISO_ISSUE007_GRAPH_RT_END/ { inside = 0; next }
  inside { print }
' "$marker_file")
[[ -z "$unexpected" ]] || { printf 'unexpected issue-007 graph render syscall(s):\n%s\n' "$unexpected" >&2; exit 1; }
jq -e '
  .kind == "issue007_graph_realtime_lifecycle_audit" and
  .renders == 1000000 and .quantum_frames == 128 and .observers == 7 and
  .render_count_by_epoch == {"1":4,"2":999996} and
  .swaps_applied == 2 and .swaps_deferred == 1 and .prior_plan_renders_on_deferred == 1 and
  .drained_blocks == 6 and .observer_windows_per_drained_block == 7 and
  .queue_success_windows == 42 and .queue_full_windows == 6999958 and
  .retired_destroyed_off_render == 2 and
  .allocations == 0 and .deallocations == 0 and .locks == 0 and .logs == 0 and
  .file_io == 0 and .network_io == 0 and .syscalls == 0 and .total_violations == 0
' "$trace_root/audit.json" >/dev/null
printf 'issue-007 graph lifecycle syscall trace: PASS (1000000 renders)\n'
